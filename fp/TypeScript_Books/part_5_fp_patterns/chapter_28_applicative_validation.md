# Chapter 28 — Applicative & Validation

> **Bạn sẽ học được**:
> - Applicative Functor — giữa Functor và Monad
> - `ap` = apply function INSIDE container to value INSIDE container
> - Validation pattern — collect ALL errors (vs Monad fail-fast)
> - `sequenceT` / `Do` pattern — combine multiple independent computations
>
> **Yêu cầu trước**: Chapter 26 (Functors, map), Chapter 27 (Monads, chain).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Thu thập TẤT CẢ lỗi validation cùng lúc — UX tốt hơn fail-fast.

---

Bạn nhớ form đăng ký mà hiển thị TẤT CẢ lỗi cùng lúc? "Tên quá ngắn", "Email không hợp lệ", "Mật khẩu yếu" — ba lỗi hiện đồng thời. Monad (`chain`) KHÔNG THỂ làm điều này — chain fail-fast: gặp lỗi đầu → dừng, chưa kiểm tra fields sau.

Hãy nghĩ như hội đồng giám khảo. **Monad-judge** kiểm tra từng tiêu chí tuần tự: fail ở tiêu chí 1 → dừng, không chấm tiếp. **Applicative-jury** kiểm tra TẤT CẢ tiêu chí SONG SONG: mỗi giám khảo chấm 1 tiêu chí, gom kết quả — bạn nhận MỌI feedback cùng lúc.

Vì sao điều này quan trọng? Vì UX. Bác sĩ khám bệnh kiểm tra TẤT CẢ chỉ số cùng lúc: huyết áp, đường huyết, cholesterol. Không phải kiểm tra huyết áp, báo bạn về sửa, quay lại kiểm tra đường huyết. Đó là sự khác biệt giữa Monad (sequential/fail-fast) và Applicative (parallel/collect-all).

Chương này giải quyết một vấn đề THỰC TẾ nhất trong FP: khi các validations ĐỘC LẬP với nhau, Monad “quá nghiêm khắc” (dừng lại ở lỗi đầu). Applicative “chinh chiến” (kiểm tra hết, báo cáo hết).

---

## Applicative & Validation — Collect ALL Errors

`Result.flatMap()` / `Either.chain()` là fail-fast: gặp lỗi đầu tiên → dừng. Nhưng user form có 5 fields sai → bạn muốn hiện **tất cả** 5 lỗi, không chỉ lỗi đầu.

Applicative Functor giải quyết: `sequenceT` / `Do` notation chạy **tất cả** validations và collect errors. Đây là pattern production dùng everywhere — từ form validation đến API request validation.


## 28.1 — Vấn đề của Monad fail-fast

Hãy chứng minh vấn đề bằng code. Ba validations độc lập: name, email, age. Với `chain` (Monad), chúng chạy TUẦN TỰ — lỗi ở step 1 → step 2, 3 không chạy. User chỉ thấy MỘT lỗi, sửa, submit lại, thấy lỗi tiếp. Ba lần submit cho ba lỗi. Kinhthiệm người dùng tệ!

```typescript
// filename: src/fail_fast_problem.ts
import assert from "node:assert/strict";

type Either<E, A> = { _tag: "Left"; left: E } | { _tag: "Right"; right: A };
const left = <E>(e: E): Either<E, never> => ({ _tag: "Left", left: e });
const right = <A>(a: A): Either<never, A> => ({ _tag: "Right", right: a });
const chain = <E, A, B>(fn: (a: A) => Either<E, B>) => (fa: Either<E, A>): Either<E, B> =>
    fa._tag === "Left" ? fa : fn(fa.right);

function pipe<A>(a: A): A;
function pipe<A, B>(a: A, ab: (a: A) => B): B;
function pipe<A, B, C>(a: A, ab: (a: A) => B, bc: (b: B) => C): C;
function pipe(a: unknown, ...fns: ((x: unknown) => unknown)[]): unknown {
    return fns.reduce((acc, fn) => fn(acc), a);
}

type FormError = { field: string; message: string };

const validateName = (name: string): Either<FormError, string> =>
    name.length >= 2 ? right(name) : left({ field: "name", message: "Min 2 chars" });

const validateEmail = (email: string): Either<FormError, string> =>
    email.includes("@") ? right(email) : left({ field: "email", message: "Invalid email" });

const validateAge = (age: number): Either<FormError, number> =>
    age >= 18 ? right(age) : left({ field: "age", message: "Must be 18+" });

// ❌ Monad chain: fail-fast! User chỉ thấy LỖI ĐẦU TIÊN
const validateForm_monad = (name: string, email: string, age: number) =>
    pipe(
        validateName(name),
        chain(_ => validateEmail(email)),  // chỉ chạy nếu name OK
        chain(_ => validateAge(age)),      // chỉ chạy nếu email OK
    );

const result = validateForm_monad("A", "bad", 15);
assert.strictEqual(result._tag, "Left");
if (result._tag === "Left") {
    assert.strictEqual(result.left.field, "name");
    // User chỉ thấy "name" error. Sửa → submit → thấy "email" error. Sửa → submit → "age" error.
    // 3 lần submit cho 3 lỗi! BAD UX!
}

console.log("Fail-fast problem shown ✅");
```

---

## 28.2 — Applicative: Collect ALL errors

### Validation = Either nhưng accumulate errors

Đây là điểm Applicative tỏa sáng. Thay vì `Either<E, A>` (fail-fast), bạn dùng `Validation<E, A>` — khác biệt duy nhất: khi CẢ HAI fail, errors được TÍCH TỤ (concatenate). Không dừng lại ở lỗi đầu.

Key operation là `ap` (apply): lấy function BÊN TRONG container, áp dụng cho value BÊN TRONG container khác. Nếu cả hai Success → apply. Nếu một Failure → giữ failure. Nếu CẢ HAI Failure → GHÉP errors lại. Đó là magic: ghép errors thay vì dừng.

```typescript
// filename: src/applicative_validation.ts
import assert from "node:assert/strict";

// Validation<E, A> = Either nhưng E phải là array (accumulate)
type Validation<E, A> =
    | { readonly _tag: "Failure"; readonly errors: readonly E[] }
    | { readonly _tag: "Success"; readonly value: A };

const failure = <E>(e: E): Validation<E, never> => ({ _tag: "Failure", errors: [e] });
const success = <A>(a: A): Validation<never, A> => ({ _tag: "Success", value: a });

// map: transform success, leave failure
const map = <A, B>(fn: (a: A) => B) => <E>(fa: Validation<E, A>): Validation<E, B> =>
    fa._tag === "Failure" ? fa : success(fn(fa.value));

// ap: THE KEY — apply function inside Validation to value inside Validation
// If both success → apply function
// If one failure → keep that failure
// If BOTH failure → COMBINE errors!
const ap = <E, A, B>(
    ff: Validation<E, (a: A) => B>,
    fa: Validation<E, A>,
): Validation<E, B> => {
    if (ff._tag === "Failure" && fa._tag === "Failure") {
        return { _tag: "Failure", errors: [...ff.errors, ...fa.errors] }; // COMBINE!
    }
    if (ff._tag === "Failure") return ff;
    if (fa._tag === "Failure") return fa;
    return success(ff.value(fa.value));
};

// --- sequenceT-style: validate all, combine results ---
type FormError = { field: string; message: string };

const validateName = (name: string): Validation<FormError, string> =>
    name.length >= 2 ? success(name) : failure({ field: "name", message: "Min 2 chars" });

const validateEmail = (email: string): Validation<FormError, string> =>
    email.includes("@") ? success(email.toLowerCase()) : failure({ field: "email", message: "Invalid email" });

const validateAge = (age: number): Validation<FormError, number> =>
    age >= 18 ? success(age) : failure({ field: "age", message: "Must be 18+" });

// Combine 3 validations
type ValidForm = { name: string; email: string; age: number };

const validateForm = (name: string, email: string, age: number): Validation<FormError, ValidForm> => {
    const nameV = validateName(name);
    const emailV = validateEmail(email);
    const ageV = validateAge(age);

    // Use ap to combine all three
    const makeFn = success(
        (name: string) => (email: string) => (age: number): ValidForm => ({ name, email, age })
    );

    return ap(ap(ap(makeFn, nameV), emailV), ageV);
};

// ✅ Happy path
const good = validateForm("An", "an@mail.com", 25);
assert.strictEqual(good._tag, "Success");
if (good._tag === "Success") {
    assert.strictEqual(good.value.name, "An");
    assert.strictEqual(good.value.email, "an@mail.com");
    assert.strictEqual(good.value.age, 25);
}

// ✅ ALL 3 errors at once!
const allBad = validateForm("A", "bad", 15);
assert.strictEqual(allBad._tag, "Failure");
if (allBad._tag === "Failure") {
    assert.strictEqual(allBad.errors.length, 3);  // ALL THREE!
    assert.strictEqual(allBad.errors[0].field, "name");
    assert.strictEqual(allBad.errors[1].field, "email");
    assert.strictEqual(allBad.errors[2].field, "age");
}

// ✅ Partial errors
const partial = validateForm("An", "bad", 15);
assert.strictEqual(partial._tag, "Failure");
if (partial._tag === "Failure") {
    assert.strictEqual(partial.errors.length, 2);  // email + age
}

console.log("Applicative validation OK ✅");
```

> **💡 Monad = sequential (fail-fast). Applicative = parallel (collect all)**. Use Monad when step N DEPENDS on step N-1. Use Applicative when validations are INDEPENDENT.

---

## ✅ Checkpoint 28.1-28.2

> Đến đây bạn phải hiểu:
> 1. **Monad chain** = fail-fast. First error stops. Sequential deps only
> 2. **Applicative ap** = collect all errors. Independent validations
> 3. **`ap(ap(ap(makeFn, v1), v2), v3)`** = combine 3 validations
> 4. **UX**: User sees ALL errors at once. Fix all → 1 submit. Much better!
>
> **Test nhanh**: Name depends on email? (e.g., name must match email prefix) — Monad hay Applicative?
> <details><summary>Đáp án</summary>**Monad!** Step 2 (check name matches email) DEPENDS on step 1 (validate email). Sequential dependency → chain/andThen.</details>

---

## 28.3 — Practical Validation Helper

Cuối cùng, `ap(ap(ap(makeFn, v1), v2), v3)` hơi ugly. Trong thực tế, bạn dùng helper như fp-ts `sequenceT` hoặc `Do` pattern. Hoặc đơn giản: viết `validateAll` helper nhận object của validations, gom errors, return kết quả:

```typescript
// filename: src/practical_validation.ts
import assert from "node:assert/strict";

type Validation<E, A> =
    | { readonly _tag: "Failure"; readonly errors: readonly E[] }
    | { readonly _tag: "Success"; readonly value: A };

const failure = <E>(e: E): Validation<E, never> => ({ _tag: "Failure", errors: [e] });
const success = <A>(a: A): Validation<never, A> => ({ _tag: "Success", value: a });

// Helper: validate all fields, combine into object
const validateAll = <E, T extends Record<string, Validation<E, unknown>>>(
    validations: T,
): Validation<E, { [K in keyof T]: T[K] extends Validation<any, infer A> ? A : never }> => {
    const errors: E[] = [];
    const values: Record<string, unknown> = {};

    for (const [key, validation] of Object.entries(validations)) {
        if (validation._tag === "Failure") {
            errors.push(...validation.errors);
        } else {
            values[key] = validation.value;
        }
    }

    if (errors.length > 0) return { _tag: "Failure", errors };
    return { _tag: "Success", value: values } as any;
};

// --- Clean usage ---
type FieldError = { field: string; message: string };

const validateProduct = (input: {
    name: string; price: number; stock: number; category: string;
}) =>
    validateAll({
        name: input.name.length >= 2
            ? success(input.name.trim())
            : failure<FieldError>({ field: "name", message: "Min 2 chars" }),
        price: input.price > 0
            ? success(input.price)
            : failure<FieldError>({ field: "price", message: "Must be > 0" }),
        stock: Number.isInteger(input.stock) && input.stock >= 0
            ? success(input.stock)
            : failure<FieldError>({ field: "stock", message: "Non-negative integer" }),
        category: ["electronics", "books", "clothing"].includes(input.category)
            ? success(input.category as "electronics" | "books" | "clothing")
            : failure<FieldError>({ field: "category", message: "Invalid category" }),
    });

// Test
const good = validateProduct({ name: "Laptop", price: 20000000, stock: 5, category: "electronics" });
assert.strictEqual(good._tag, "Success");

const allBad = validateProduct({ name: "X", price: -1, stock: 1.5, category: "food" });
assert.strictEqual(allBad._tag, "Failure");
if (allBad._tag === "Failure") {
    assert.strictEqual(allBad.errors.length, 4);
}

console.log("Practical validation OK ✅");
```

---

## ✅ Checkpoint 28.3

> Đến đây bạn phải hiểu:
> 1. **`validateAll`** = practical helper for form validation
> 2. **Object of validations** → single result with all errors or all values
> 3. **Clean API**: `validateAll({ name: ..., price: ..., stock: ... })`
> 4. **This IS Applicative** — just with a nicer API than raw `ap`
>
> **Test nhanh**: 4 fields, 2 invalid → errors array length?
> <details><summary>Đáp án</summary>**2!** Only invalid fields contribute errors. Valid fields produce values (but combined result is Failure since errors > 0).</details>

---

## 🏋️ Bài tập

**Bài 1** (10 phút): Form validation

```typescript
// Viết validation cho "Create Account" form:
// username: 3-20 chars, alphanumeric only
// email: must contain @ and .
// password: 8+ chars, at least 1 number
// confirmPassword: must match password
// Note: confirmPassword DEPENDS on password → use Monad for that pair!
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

type Validation<E, A> =
    | { _tag: "Failure"; errors: readonly E[] }
    | { _tag: "Success"; value: A };
const failure = <E>(e: E): Validation<E, never> => ({ _tag: "Failure", errors: [e] });
const success = <A>(a: A): Validation<never, A> => ({ _tag: "Success", value: a });

type FieldError = { field: string; message: string };

const validateUsername = (u: string): Validation<FieldError, string> =>
    u.length >= 3 && u.length <= 20 && /^[a-zA-Z0-9]+$/.test(u)
        ? success(u) : failure({ field: "username", message: "3-20 alphanumeric chars" });

const validateEmail = (e: string): Validation<FieldError, string> =>
    e.includes("@") && e.includes(".")
        ? success(e.toLowerCase()) : failure({ field: "email", message: "Invalid email" });

// Password + confirm = sequential (Monad-ish within Applicative)
const validatePasswords = (pw: string, confirm: string): Validation<FieldError, string> => {
    if (pw.length < 8) return failure({ field: "password", message: "Min 8 chars" });
    if (!/\d/.test(pw)) return failure({ field: "password", message: "Need at least 1 number" });
    if (pw !== confirm) return failure({ field: "confirmPassword", message: "Passwords don't match" });
    return success(pw);
};

const validateAll = <E, T extends Record<string, Validation<E, unknown>>>(v: T): Validation<E, { [K in keyof T]: T[K] extends Validation<any, infer A> ? A : never }> => {
    const errors: E[] = []; const values: Record<string, unknown> = {};
    for (const [k, val] of Object.entries(v)) { if (val._tag === "Failure") errors.push(...val.errors); else values[k] = val.value; }
    return errors.length > 0 ? { _tag: "Failure", errors } : { _tag: "Success", value: values } as any;
};

const validateForm = (u: string, e: string, pw: string, confirm: string) =>
    validateAll({ username: validateUsername(u), email: validateEmail(e), password: validatePasswords(pw, confirm) });

const good = validateForm("an123", "an@mail.com", "secure1pass", "secure1pass");
assert.strictEqual(good._tag, "Success");

const allBad = validateForm("a!", "bad", "short", "nope");
assert.strictEqual(allBad._tag, "Failure");
if (allBad._tag === "Failure") assert.strictEqual(allBad.errors.length, 3);
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "When Monad, when Applicative?" | Dependency | Independent fields → Applicative. Dependent steps → Monad |
| `ap` syntax ugly | Curried application complex | Use `validateAll` helper or fp-ts `sequenceT` |
| "Validation vs Either?" | Accumulation behavior | Either = fail-fast. Validation = accumulate. Same shape, different ap! |

---

## Tóm tắt

Chương này giải quyết một vấn đề thực tế mà Monad KHÔNG làm được: thu thập TẤT CẢ lỗi cùng lúc. Monad = giám khảo nghiêm khắc (dừng ở lỗi đầu). Applicative = hội đồng (kiểm tra hết, báo cáo hết). Chỏn đúng công cụ cho đúng vấn đề.

- ✅ **Monad** = fail-fast (chain). **Applicative** = collect all (ap).
- ✅ **`ap`** = apply function-in-container to value-in-container. Combine errors.
- ✅ **`validateAll`** = practical helper. Object of validations → combined result.
- ✅ **UX**: Show ALL errors at once. User fixes all → 1 submit.
- ✅ **Rule**: Independent validations → Applicative. Sequential deps → Monad.

## Tiếp theo

→ Chapter 29: **Monoids & Abstract Algebra** — combining values with rules. `concat`, `empty`, merging configs, aggregating data. Pattern xuất hiện KHẮP NƠI.
