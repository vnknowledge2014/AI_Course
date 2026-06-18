# Chapter 14 — Smart Constructors & Validation

> **Bạn sẽ học được**:
> - "Parse, don't validate" — triết lý cốt lõi
> - Smart constructors — validate + brand tại cổng vào
> - Zod — runtime validation library, schema-first
> - Accumulating errors — thu thập TẤT CẢ lỗi, không chỉ lỗi đầu tiên
> - Validation pipelines — compose validators bằng pipe
> - Boundary validation — "validate ở biên, trust ở core"
>
> **Yêu cầu trước**: Chapter 13 (ADTs, Result, branded types, smart constructors).
> **Thời gian đọc**: ~45 phút | **Level**: Intermediate → Advanced
> **Kết quả cuối cùng**: Hệ thống validation type-safe, composable, production-ready.

---

Bạn đã bao giờ đi qua cổng hải quan chưa?

Khi bạn nhập cảnh vào một quốc gia, có một **biên giới rõ ràng**: bên ngoài là thế giới không đáng tin (hộ chiếu giả, hàng cấm, thông tin sai), bên trong là lãnh thổ an toàn (đã kiểm tra, đã xác minh). Tại cổng hải quan, nhân viên **kiểm tra kỹ**: hộ chiếu hợp lệ? Visa còn hạn? Hàng hóa đã khai báo? Chỉ khi mọi thứ pass, bạn mới được vào. Và một khi đã vào trong, bạn không bị kiểm tra lại mỗi lần rẽ phố — hộ chiếu đã "stamp" nghĩa là "đã xác minh".

Đây chính xác là cách **boundary validation** hoạt động trong phần mềm. Dữ liệu từ bên ngoài (API request, form HTML, file JSON) là "du khách chưa kiểm tra". Smart constructors đóng vai trò **cổng hải quan**: validate, normalize, rồi "stamp" (brand) — biến `string` thành `Email`, biến `number` thành `Money`. Sau khi đi qua cổng, core logic tin tưởng type system — không cần validate lại. Type = con dấu hộ chiếu.

Nhưng trong thực tế, hải quan không chỉ kiểm tra MỘT thứ. Họ kiểm tra hộ chiếu, visa, hành lý, tờ khai — **tất cả cùng lúc**, rồi trả về danh sách TẤT CẢ vấn đề, không phải từng thứ một. Chương này sẽ dạy bạn cả hai kỹ thuật: **railway** (dừng ở lỗi đầu tiên — cho dependent steps) và **accumulating** (thu thập mọi lỗi — cho UX tốt hơn).

---

## 14.1 — "Parse, Don't Validate"

### Sự khác biệt giữa kiểm tra và "stamp"

Hãy tưởng tượng hai nhân viên hải quan. Nhân viên A nhìn hộ chiếu, gật đầu nói "OK", rồi trả lại — **không stamp**. Nhân viên B nhìn hộ chiếu, kiểm tra, rồi **đóng dấu** vào trang visa. Cả hai đều "kiểm tra", nhưng chỉ B tạo ra bằng chứng vĩnh viễn rằng hộ chiếu đã hợp lệ.

Trong code, nhân viên A là `validate` — trả `boolean`, type không đổi. Nhân viên B là `parse` — trả `Result<BrandedType, Error>`, type **thay đổi** phản ánh kết quả. Sau khi parse, compiler **biết** dữ liệu đã hợp lệ — bạn không thể vô tình dùng dữ liệu chưa kiểm tra.

```typescript
// filename: src/parse_vs_validate.ts
import assert from "node:assert/strict";

// ❌ VALIDATE — type không đổi
const isValidEmail = (input: string): boolean =>
    input.includes("@") && input.length <= 254;

// Sau validate, vẫn là string — compiler KHÔNG biết nó valid!
const email = "an@mail.com";
if (isValidEmail(email)) {
    // email vẫn là string ở đây
    // Ai đó có thể truyền string chưa validate vào function cần email
}

// ✅ PARSE — type ĐỔI
type Brand<T, B extends string> = T & { readonly __brand: B };
type Email = Brand<string, "Email">;

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

const parseEmail = (input: string): Result<Email, string> => {
    const trimmed = input.trim().toLowerCase();
    if (!trimmed.includes("@")) return err("Email phải có @");
    if (trimmed.length > 254) return err("Email quá dài");
    return ok(trimmed as Email);
};

// Sau parse, type = Email — compiler ENFORCE!
const result = parseEmail("An@Mail.Com");
if (result.tag === "ok") {
    const email: Email = result.value;       // ✅ Type = Email, not string
    // sendNewsletter(email);                // ✅ Compiler biết đã parsed
    // sendNewsletter("random string");      // ❌ string ≠ Email
}

console.log("Parse don't validate OK ✅");
```

Nhìn sự khác biệt: `isValidEmail` trả `boolean` — sau khi gọi, `email` vẫn là `string`. Bất kỳ ai cũng có thể truyền string chưa validate vào function cần email. `parseEmail` trả `Result<Email, string>` — sau khi unwrap, bạn có `Email`, một type hoàn toàn khác. Compiler ngăn bạn truyền `string` thường vào nơi cần `Email`.

Đây là triết lý **"Parse, don't validate"** — mỗi phép kiểm tra nên thay đổi type, biến "qua cổng hải quan" thành một con dấu vĩnh viễn trên "hộ chiếu" dữ liệu.

> **💡 "Parse, don't validate"**: Mỗi check nên **thay đổi type**. Nếu bạn biết data valid, type phải REFLECT điều đó. Compiler = ally, không phải enemy.

---

## ✅ Checkpoint 14.1

> Đến đây bạn phải hiểu:
> 1. **Validate** = check + giữ type cũ → compiler không biết
> 2. **Parse** = check + type MỚI → compiler enforce
> 3. **Smart constructor** = parser: `string` → `Result<Email, Error>`
> 4. Sau khi parse, KHÔNG CẦN validate lại — type đảm bảo
>
> **Test nhanh**: `isPositive(n: number): boolean` vs `parsePositive(n: number): Result<PositiveNumber, string>` — cái nào "parse, don't validate"?
> <details><summary>Đáp án</summary>`parsePositive`! Trả `PositiveNumber` = type MỚI. `isPositive` trả `boolean` = type KHÔNG ĐỔI.</details>

---

## 14.2 — Smart Constructors: Chi tiết

### Nhiều quy tắc, một cổng duy nhất

Hải quan không chỉ kiểm tra MỘT thứ trên hộ chiếu — họ kiểm tra ảnh, ngày hết hạn, số hộ chiếu, tên, quốc tịch. Tương tự, smart constructor cho `Username` không chỉ check độ dài — nó kiểm tra tối thiểu, tối đa, format, ký tự cho phép, và có thể normalize (trim, lowercase).

Và giống như bạn chỉ đi qua hải quan MỘT LẦN (không bị kiểm tra lại ở mỗi quầy check-in khách sạn), smart constructor là **cổng duy nhất** tạo branded value. Sau đó, type system đảm bảo mọi giá trị `Username` trong chương trình đều đã qua kiểm tra.

Một điểm đáng chú ý: `Password` ở ví dụ dưới trả `Result<Password, readonly string[]>` — mảng lỗi thay vì 1 lỗi. Tại sao? Vì khi user nhập mật khẩu "weak", bạn muốn nói: "thiếu chữ hoa, thiếu số, quá ngắn" — **tất cả cùng lúc**, không phải "quá ngắn" lần 1, "thiếu chữ hoa" lần 2, "thiếu số" lần 3. Đây là accumulating errors ở level đơn field.

```typescript
// filename: src/smart_constructors.ts
import assert from "node:assert/strict";

type Brand<T, B extends string> = T & { readonly __brand: B };

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// --- Smart constructors với MULTIPLE validation rules ---

type Username = Brand<string, "Username">;
const Username = (input: string): Result<Username, string> => {
    const trimmed = input.trim();
    if (trimmed.length < 3) return err("Username: >= 3 ký tự");
    if (trimmed.length > 30) return err("Username: <= 30 ký tự");
    if (!/^[a-zA-Z][a-zA-Z0-9_]*$/.test(trimmed))
        return err("Username: bắt đầu bằng chữ, chỉ cho phép a-z, 0-9, _");
    return ok(trimmed as Username);
};

type Age = Brand<number, "Age">;
const Age = (input: number): Result<Age, string> => {
    if (!Number.isInteger(input)) return err("Age: phải là số nguyên");
    if (input < 0) return err("Age: >= 0");
    if (input > 150) return err("Age: <= 150");
    return ok(input as Age);
};

type Password = Brand<string, "Password">;
const Password = (input: string): Result<Password, readonly string[]> => {
    const errors: string[] = [];
    if (input.length < 8) errors.push("Tối thiểu 8 ký tự");
    if (!/[A-Z]/.test(input)) errors.push("Cần ít nhất 1 chữ hoa");
    if (!/[a-z]/.test(input)) errors.push("Cần ít nhất 1 chữ thường");
    if (!/[0-9]/.test(input)) errors.push("Cần ít nhất 1 số");

    return errors.length === 0
        ? ok(input as Password)
        : err(errors);
};

// Test
assert.strictEqual(Username("an_nguyen").tag, "ok");
assert.strictEqual(Username("ab").tag, "err");
assert.strictEqual(Username("123abc").tag, "err");

assert.strictEqual(Age(25).tag, "ok");
assert.strictEqual(Age(-1).tag, "err");
assert.strictEqual(Age(3.5).tag, "err");

const weakPass = Password("weak");
assert.strictEqual(weakPass.tag, "err");
if (weakPass.tag === "err") {
    assert.ok(weakPass.error.length >= 3);  // nhiều lỗi!
}

const strongPass = Password("StrongP4ss");
assert.strictEqual(strongPass.tag, "ok");

console.log("Smart constructors OK ✅");
```

Chú ý `Username` dùng early return (railway — dừng ở lỗi đầu) vì các rules **phụ thuộc** nhau: nếu quá ngắn thì check format cũng vô nghĩa. `Password` dùng accumulate vì các rules **độc lập**: thiếu chữ hoa không ảnh hưởng check số. Bạn sẽ thấy pattern này lặp lại ở quy mô lớn hơn trong section 14.4.

> **💡 Password example**: Smart constructor trả `Result<Password, readonly string[]>` — thu thập TẤT CẢ lỗi password cùng lúc, không chỉ lỗi đầu! User sửa 1 lần thay vì 4 lần.

---

## ✅ Checkpoint 14.2

> Đến đây bạn phải hiểu:
> 1. **Smart constructor** = validate + transform + brand. Cổng DUY NHẤT
> 2. **Trim/normalize** trước khi validate — consistent data
> 3. **Multiple errors**: trả `readonly string[]` khi mỗi field có nhiều rules
> 4. **Return type** luôn `Result<BrandedType, ErrorType>` — explicit
>
> **Test nhanh**: Smart constructor cho `type Port = Brand<number, "Port">` — rules: 1-65535, integer. Viết signature?
> <details><summary>Đáp án</summary>`const Port = (n: number): Result<Port, string>`. Check: `Number.isInteger(n) && n >= 1 && n <= 65535`.</details>

---

## 14.3 — Zod: Runtime Schema Validation

### Khi hải quan cần xử lý cả container hàng hóa

Smart constructors viết tay hoạt động tốt cho từng type đơn lẻ: `Email`, `Username`, `Password`. Nhưng hãy tưởng tượng bạn là hải quan tại cảng, và một container hàng đến với hàng chục loại sản phẩm lồng nhau — objects chứa arrays chứa objects. Viết smart constructor cho từng field sẽ tốn rất nhiều code.

**Zod** là thư viện giải quyết vấn đề này bằng cách cho bạn mô tả **schema** (hình dạng dữ liệu mong muốn), rồi tự động: (1) validate runtime data theo schema, (2) generate TypeScript type từ schema, và (3) thu thập TẤT CẢ lỗi. Một schema, hai vai trò: vừa là runtime validator, vừa là compile-time type. "Parse, don't validate" ở quy mô container.

```typescript
// filename: src/zod_intro.ts
// npm install zod

import { z } from "zod";
import assert from "node:assert/strict";

// --- Schema = "mô tả shape data mong muốn" ---
const UserSchema = z.object({
    name: z.string().min(2, "Tên >= 2 ký tự").max(50),
    email: z.string().email("Email không hợp lệ"),
    age: z.number().int().min(0).max(150),
});

// Zod tự generate TypeScript type từ schema!
type User = z.infer<typeof UserSchema>;
// = { name: string; email: string; age: number }

// --- Parse: validate + type narrow ---
const validInput = { name: "An", email: "an@mail.com", age: 25 };
const result = UserSchema.safeParse(validInput);
// safeParse trả { success: true, data: User } | { success: false, error: ZodError }

if (result.success) {
    const user: User = result.data;
    // user.name = "An" — typed correctly!
    assert.strictEqual(user.name, "An");
}

// Invalid input
const badInput = { name: "A", email: "not-email", age: -5 };
const badResult = UserSchema.safeParse(badInput);

if (!badResult.success) {
    // Zod thu thập TẤT CẢ errors!
    assert.ok(badResult.error.issues.length >= 3);
    // issues[0].message = "Tên >= 2 ký tự"
    // issues[1].message = "Email không hợp lệ"
    // issues[2].message = "Number must be greater than or equal to 0"
}

console.log("Zod OK ✅");
```

Bạn có nhận ra điều gì lạ không? `z.infer<typeof UserSchema>` sinh ra TypeScript type **từ runtime schema**. Nghĩa là bạn viết schema MỘT LẦN, và có BOTH: runtime validation + compile-time type. Không cần viết `type User = { ... }` rồi viết validator riêng — không bao giờ lệch nhau.

### Zod schemas phổ biến

```typescript
// filename: src/zod_schemas.ts
import { z } from "zod";

// Primitives
const name = z.string().min(1).max(100);
const age = z.number().int().nonnegative();
const isActive = z.boolean();

// Enum — thay string union
const Role = z.enum(["admin", "user", "guest"]);
type Role = z.infer<typeof Role>;  // "admin" | "user" | "guest"

// Arrays
const tags = z.array(z.string()).min(1).max(10);

// Optional & nullable
const bio = z.string().optional();       // string | undefined
const avatar = z.string().nullable();    // string | null

// Nested objects
const AddressSchema = z.object({
    street: z.string().min(1),
    city: z.string().min(1),
    zipCode: z.string().regex(/^\d{5,6}$/, "Mã bưu điện: 5-6 chữ số"),
});

// Object with nested
const ProfileSchema = z.object({
    user: z.object({
        name: z.string(),
        email: z.string().email(),
    }),
    address: AddressSchema,
    roles: z.array(Role).nonempty(),
});

type Profile = z.infer<typeof ProfileSchema>;

// Discriminated union!
const PaymentSchema = z.discriminatedUnion("tag", [
    z.object({ tag: z.literal("credit_card"), number: z.string(), cvv: z.string() }),
    z.object({ tag: z.literal("bank_transfer"), bankCode: z.string() }),
    z.object({ tag: z.literal("e_wallet"), provider: z.enum(["momo", "zalopay"]) }),
]);

type Payment = z.infer<typeof PaymentSchema>;
// = { tag: "credit_card"; ... } | { tag: "bank_transfer"; ... } | { tag: "e_wallet"; ... }
```

### Zod transform — Parse AND transform

Zod không chỉ validate — nó có thể **biến đổi** dữ liệu trong quá trình parsing. Giống nhân viên hải quan không chỉ kiểm tra hộ chiếu, mà còn ghi nhận số hộ chiếu, chụp ảnh, cấp thẻ nhập cảnh — output khác input.

`transform` kết hợp với `pipe` cho phép: trim trước, validate sau. Thứ tự rất quan trọng — nếu validate email trước khi trim, spaces sẽ làm fail validation sai.

```typescript
// filename: src/zod_transform.ts
import { z } from "zod";
import assert from "node:assert/strict";

// Transform: validate → transform → output type mới!
// ⚠️ Thứ tự quan trọng: trim TRƯỚC, validate SAU!
// (Nếu dùng .email().transform(trim) — spaces sẽ fail .email() trước khi trim chạy)
const EmailSchema = z
    .string()
    .transform(s => s.trim().toLowerCase())
    .pipe(z.string().email("Email không hợp lệ"));

// Input: "  An@Mail.Com  "
// Step 1: transform → "an@mail.com" (trimmed + lowercased)
// Step 2: pipe → validate email format → ✅
// Output: "an@mail.com"
const result = EmailSchema.safeParse("  An@Mail.Com  ");
if (result.success) {
    assert.strictEqual(result.data, "an@mail.com");
}

// refine: custom validation
const PasswordSchema = z
    .string()
    .min(8, "Tối thiểu 8 ký tự")
    .refine(s => /[A-Z]/.test(s), "Cần ít nhất 1 chữ hoa")
    .refine(s => /[0-9]/.test(s), "Cần ít nhất 1 số");

const weak = PasswordSchema.safeParse("weak");
assert.strictEqual(weak.success, false);

// coerce: tự convert type
const PageSchema = z.object({
    page: z.coerce.number().int().positive(),  // "3" → 3
    limit: z.coerce.number().int().positive().default(20),
});

const query = PageSchema.safeParse({ page: "3" });
if (query.success) {
    assert.strictEqual(query.data.page, 3);      // coerced string → number
    assert.strictEqual(query.data.limit, 20);     // default applied
}

console.log("Zod transform OK ✅");
```

> **💡 Zod = "parse, don't validate" for objects**: Schema là single source of truth. Type + validation = 1 chỗ. Không cần viết type rồi viết validator riêng.

---

## ✅ Checkpoint 14.3

> Đến đây bạn phải hiểu:
> 1. **Zod schema** = mô tả shape data. `z.infer<>` = extract TypeScript type
> 2. **`safeParse`** = trả `{ success, data }` hoặc `{ success, error }` — giống Result!
> 3. **`transform`** = parse + transform. **`refine`** = custom validation
> 4. **`coerce`** = tự convert type (string → number, etc.)
>
> **Test nhanh**: `z.string().email().safeParse(42)` — kết quả?
> <details><summary>Đáp án</summary>`{ success: false }` — 42 là number, không phải string. Zod check type TRƯỚC, rồi mới check email format.</details>

---

## 14.4 — Accumulating Errors: Thu thập TẤT CẢ lỗi

### Hải quan kiểm tra TẤT CẢ, không chỉ thứ đầu tiên

Nhớ railway-oriented programming ở Ch13? Pipeline dừng ở lỗi đầu tiên: `createUser("", "invalid", -5)` chỉ trả "Name must be >= 2 chars" — user phải sửa name, submit lại, nhận email error, submit lại, nhận age error. Ba lần submit cho ba lỗi!

Đây giống như nhân viên hải quan kiểm tra **từng thứ một**: nhìn hộ chiếu → sai → đuổi về. Bạn sửa hộ chiếu, quay lại. Họ nhìn visa → sai → đuổi về. Sửa visa, quay lại. Rồi hàng hóa...

Nhân viên hải quan thông minh hơn: họ kiểm tra **tất cả cùng lúc** — hộ chiếu, visa, hành lý — rồi đưa danh sách TẤT CẢ vấn đề. Bạn sửa hết, quay lại MỘT LẦN. Đây là **accumulating errors**.

```typescript
// filename: src/first_error_problem.ts

// Ch13 railway — stops at FIRST error:
// createUser("", "invalid", -5)
// → err("Name must be >= 2 chars")
// User phải sửa name → submit → nhận email error → sửa → submit → nhận age error
// 😤 3 lần submit cho 3 lỗi!
```

### Giải pháp: Accumulate errors

```typescript
// filename: src/accumulate_errors.ts
import assert from "node:assert/strict";

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });


// combineResults — thu thập TẤT CẢ lỗi
const combineResults = <T extends readonly Result<unknown, readonly string[]>[]>(
    ...results: T
): Result<
    { [K in keyof T]: T[K] extends Result<infer V, unknown> ? V : never },
    readonly string[]
> => {
    const errors: string[] = [];
    const values: unknown[] = [];

    for (const result of results) {
        if (result.tag === "err") {
            errors.push(...result.error);
        } else {
            values.push(result.value);
        }
    }

    return errors.length > 0
        ? err(errors) as any   // as any: mapped tuple type quá phức tạp — runtime-safe
        : ok(values as any);
};

// --- Validators ---
const validateName = (name: string): Result<string, readonly string[]> =>
    name.length >= 2 ? ok(name) : err(["Tên >= 2 ký tự"]);

const validateEmail = (email: string): Result<string, readonly string[]> =>
    email.includes("@") ? ok(email) : err(["Email không hợp lệ"]);

const validateAge = (age: number): Result<number, readonly string[]> => {
    const errors: string[] = [];
    if (!Number.isInteger(age)) errors.push("Tuổi phải là số nguyên");
    if (age < 0) errors.push("Tuổi >= 0");
    if (age > 150) errors.push("Tuổi <= 150");
    return errors.length === 0 ? ok(age) : err(errors);
};

// --- Thu thập TẤT CẢ lỗi cùng lúc ---
type User = {
    readonly name: string;
    readonly email: string;
    readonly age: number;
};

const createUser = (
    name: string,
    email: string,
    age: number
): Result<User, readonly string[]> => {
    const combined = combineResults(
        validateName(name),
        validateEmail(email),
        validateAge(age),
    );

    if (combined.tag === "err") return combined;

    const [validName, validEmail, validAge] = combined.value;
    return ok({ name: validName, email: validEmail, age: validAge });
};

// Test: TẤT CẢ 3 lỗi cùng lúc!
const result = createUser("", "invalid", -5);
assert.strictEqual(result.tag, "err");
if (result.tag === "err") {
    assert.strictEqual(result.error.length, 3);
    // ["Tên >= 2 ký tự", "Email không hợp lệ", "Tuổi >= 0"]
}

// Test: valid
const good = createUser("An", "an@mail.com", 25);
assert.strictEqual(good.tag, "ok");

console.log("Accumulate errors OK ✅");
```

`combineResults` là chìa khóa: nó chạy TẤT CẢ validators, gom errors lại, và chỉ trả `ok` khi KHÔNG có lỗi nào. So sánh với railway (Ch13) nơi error đầu tiên skip hết phần còn lại, accumulating chạy **song song** — không validator nào bị bỏ qua.

### Railway vs Accumulating — Khi nào dùng?

| | Railway (first error) | Accumulating (all errors) |
|---|---|---|
| Stops at | First error | Never — collects all |
| UX | Bad → sửa từng lỗi | Good → sửa hết 1 lần |
| Dùng khi | **Dependent** validations (step 2 cần step 1) | **Independent** validations (name, email, age riêng) |
| Ví dụ | Parse JSON → validate schema | Form validation |

Quy tắc đơn giản: nếu validator B **cần kết quả** của validator A (ví dụ: parse JSON trước, rồi mới validate schema), dùng railway. Nếu các validators **độc lập** (name, email, age không liên quan nhau), dùng accumulating. Trong thực tế, bạn thường **kết hợp cả hai**: railway cho dependent chain, accumulating cho independent fields bên trong mỗi bước.

> **💡 Rule of thumb**: Form validation = accumulate. Pipeline processing = railway. Thường combine: railway cho dependent steps, accumulate cho independent fields.

---

## ✅ Checkpoint 14.4

> Đến đây bạn phải hiểu:
> 1. **Railway** = first error stops. Tốt cho dependent steps
> 2. **Accumulating** = collect ALL errors. Tốt cho independent fields
> 3. **`combineResults`** = chạy tất cả validators, gom errors
> 4. **UX**: user sửa 1 lần thay vì nhiều lần submit
>
> **Test nhanh**: Validate "email" rồi "password" — railway hay accumulate?
> <details><summary>Đáp án</summary>**Accumulate**! Email và password INDEPENDENT — lỗi email không ảnh hưởng password. User nên thấy cả 2 lỗi cùng lúc.</details>

---

## 14.5 — Validation Pipelines

### Xây dựng dây chuyền kiểm tra

Bạn đã thấy smart constructors đơn lẻ (validate email, parse password) và accumulating errors (gom lỗi nhiều fields). Giờ hãy kết hợp cả hai: **validation pipeline** — chuỗi validators ghép lại bằng composition, giống dây chuyền sản xuất từ Ch12.

Mỗi validator là một function `(input: I) => Result<O, E>` — nhận input, trả thành công hoặc lỗi. `chain(a, b)` ghép hai validators: nếu `a` thành công, chạy `b` trên kết quả; nếu `a` thất bại, trả lỗi ngay (railway). Kết quả: bạn xây pipeline phức tạp từ building blocks đơn giản — normalize → nonEmpty → emailFormat → validDomain.

```typescript
// filename: src/validation_pipeline.ts
import assert from "node:assert/strict";

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// --- Composable validator type ---
type Validator<I, O, E> = (input: I) => Result<O, E>;

// chain: run validator B only if A succeeds (railway)
const chain = <I, M, O, E>(
    first: Validator<I, M, E>,
    second: Validator<M, O, E>
): Validator<I, O, E> =>
    (input) => {
        const result = first(input);
        return result.tag === "ok" ? second(result.value) : result;
    };

// --- Build pipeline from small validators ---

// Step 1: trim and lowercase
const normalize = (input: string): Result<string, string> =>
    ok(input.trim().toLowerCase());

// Step 2: check non-empty
const nonEmpty = (input: string): Result<string, string> =>
    input.length > 0 ? ok(input) : err("Không được để trống");

// Step 3: check email format
const emailFormat = (input: string): Result<string, string> =>
    input.includes("@") ? ok(input) : err("Email phải có @");

// Step 4: check domain
const validDomain = (input: string): Result<string, string> => {
    const domain = input.split("@")[1];
    return domain && domain.includes(".")
        ? ok(input)
        : err("Email phải có domain hợp lệ (vd: mail.com)");
};

// Compose: normalize → nonEmpty → emailFormat → validDomain
const validateEmail = (input: string): Result<string, string> => {
    const step1 = normalize(input);
    if (step1.tag === "err") return step1;

    const step2 = nonEmpty(step1.value);
    if (step2.tag === "err") return step2;

    const step3 = emailFormat(step2.value);
    if (step3.tag === "err") return step3;

    return validDomain(step3.value);
};

// Hoặc dùng chain helper:
const validateEmailChained = chain(
    chain(
        chain(normalize, nonEmpty),
        emailFormat
    ),
    validDomain
);

// Test
assert.strictEqual(validateEmail("  An@Mail.Com  ").tag, "ok");
assert.strictEqual(validateEmail("  ").tag, "err");
assert.strictEqual(validateEmail("noatsign").tag, "err");
assert.strictEqual(validateEmail("bad@").tag, "err");

assert.strictEqual(validateEmailChained("  An@Mail.Com  ").tag, "ok");

console.log("Validation pipeline OK ✅");
```

So sánh `validateEmail` (manual) và `validateEmailChained` (dùng `chain`): kết quả giống nhau, nhưng `chain` loại bỏ boilerplate `if (step.tag === "err") return step`. Trong production, bạn thường dùng thư viện như `fp-ts` hoặc `Effect` — chúng cung cấp `pipe` + `chain` sẵn, nên pipeline validation trông sạch như: `pipe(input, normalize, chain(nonEmpty), chain(emailFormat), chain(validDomain))`.

### Real-world: Form validation

Giờ hãy áp dụng tất cả vào bài toán thực tế nhất: **validate form HTML**. Thách thức: HTML forms gửi TẤT CẢ giá trị dưới dạng `string` — kể cả tuổi ("25"), số lượng ("3"), giá ("500000"). Bạn phải parse strings → proper types, accumulate errors per-field, VÀ xử lý dependent fields (confirmPassword cần match password).

```typescript
// filename: src/form_validation.ts
import assert from "node:assert/strict";

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// --- Field errors = Map of field → error messages ---
type FieldErrors = ReadonlyMap<string, readonly string[]>;

const emptyErrors: FieldErrors = new Map();

const addError = (
    errors: FieldErrors,
    field: string,
    message: string
): FieldErrors => {
    const existing = errors.get(field) ?? [];
    return new Map([...errors, [field, [...existing, message]]]);
};

// --- Validate form ---
type RegistrationForm = {
    readonly username: string;
    readonly email: string;
    readonly password: string;
    readonly confirmPassword: string;
    readonly age: string;  // from HTML form — always string!
};

type Registration = {
    readonly username: string;
    readonly email: string;
    readonly password: string;
    readonly age: number;
};

const validateRegistration = (form: RegistrationForm): Result<Registration, FieldErrors> => {
    let errors: FieldErrors = emptyErrors;

    // Username
    const username = form.username.trim();
    if (username.length < 3) errors = addError(errors, "username", "Tối thiểu 3 ký tự");
    if (username.length > 30) errors = addError(errors, "username", "Tối đa 30 ký tự");
    if (!/^[a-zA-Z][a-zA-Z0-9_]*$/.test(username) && username.length > 0)
        errors = addError(errors, "username", "Bắt đầu bằng chữ, chỉ a-z, 0-9, _");

    // Email
    const email = form.email.trim().toLowerCase();
    if (!email.includes("@")) errors = addError(errors, "email", "Email không hợp lệ");

    // Password
    if (form.password.length < 8) errors = addError(errors, "password", "Tối thiểu 8 ký tự");
    if (!/[A-Z]/.test(form.password)) errors = addError(errors, "password", "Cần ít nhất 1 chữ hoa");
    if (!/[0-9]/.test(form.password)) errors = addError(errors, "password", "Cần ít nhất 1 số");

    // Confirm password (DEPENDENT — needs password to be valid first)
    if (form.password !== form.confirmPassword)
        errors = addError(errors, "confirmPassword", "Mật khẩu không khớp");

    // Age (parse string → number)
    const age = Number(form.age);
    if (form.age.trim() === "" || Number.isNaN(age))
        errors = addError(errors, "age", "Tuổi phải là số");
    else if (!Number.isInteger(age) || age < 18 || age > 120)
        errors = addError(errors, "age", "Tuổi: 18-120");

    if (errors.size > 0) return err(errors);

    return ok({
        username,
        email,
        password: form.password,
        age,
    });
};

// Test: invalid form → được TẤT CẢ lỗi!
const badForm: RegistrationForm = {
    username: "ab",
    email: "invalid",
    password: "weak",
    confirmPassword: "different",
    age: "abc",
};

const result = validateRegistration(badForm);
assert.strictEqual(result.tag, "err");
if (result.tag === "err") {
    assert.ok(result.error.has("username"));
    assert.ok(result.error.has("email"));
    assert.ok(result.error.has("password"));
    assert.ok(result.error.has("confirmPassword"));
    assert.ok(result.error.has("age"));
}

// Test: valid form
const goodForm: RegistrationForm = {
    username: "an_nguyen",
    email: "An@Mail.Com",
    password: "StrongP4ss",
    confirmPassword: "StrongP4ss",
    age: "25",
};

const goodResult = validateRegistration(goodForm);
assert.strictEqual(goodResult.tag, "ok");
if (goodResult.tag === "ok") {
    assert.strictEqual(goodResult.value.email, "an@mail.com");
    assert.strictEqual(goodResult.value.age, 25);  // parsed from string!
}

console.log("Form validation OK ✅");
```

Chú ý `FieldErrors = Map<string, readonly string[]>` — mỗi field có thể có NHIỀU lỗi (password: "quá ngắn", "thiếu hoa", "thiếu số"). UI frontend có thể hiển thị chính xác mỗi lỗi dưới từng field tương ứng — trải nghiệm người dùng tốt hơn rất nhiều so với một đống text "Something went wrong".

---

## ✅ Checkpoint 14.5

> Đến đây bạn phải hiểu:
> 1. **Composable validators**: chain(a, b) = run b if a succeeds
> 2. **FieldErrors**: `Map<field, messages[]>` — per-field, multiple errors
> 3. **Form data = strings**: HTML forms gửi strings → phải parse (age: string → number)
> 4. **Mix railway + accumulate**: dependent fields (confirm=password) dùng railway, independent fields accumulate
>
> **Test nhanh**: `confirmPassword` nên validate trước hay sau `password`?
> <details><summary>Đáp án</summary>**Cùng lúc** (accumulate)! Dù password sai, user vẫn muốn biết confirm có khớp không. Nhưng nếu password TRỐNG, thì "không khớp" không hữu ích — lúc đó chỉ hiện lỗi password.</details>

---

## 14.6 — Boundary Validation: Validate Once, Trust Forever

### Bản đồ kiến trúc: biên giới và lãnh thổ

Đây là section quan trọng nhất của chương — nó tổng hợp mọi thứ thành một **kiến trúc**. Hãy nhìn toàn cảnh hệ thống: bên ngoài là thế giới không đáng tin (API requests, form submissions, file uploads), bên trong là core logic thuần khiết. Giữa chúng là **BOUNDARY** — lớp hải quan parse mọi dữ liệu đi vào.

Nguyên tắc đơn giản: **parse ở biên giới, tin tưởng ở lõi**. Boundary functions nhận raw types (`string`, `unknown`), parse thành branded types (`Email`, `Money`), rồi truyền vào core. Core functions **không bao giờ validate** — chúng chỉ nhận branded types. Type system đảm bảo rằng mọi `Email` đi vào core đã được kiểm tra tại biên.

```typescript
// filename: src/boundary_validation.ts
import assert from "node:assert/strict";

// --- Architecture ---
//
// UNTRUSTED WORLD (external)
//     │
//     ▼
// ┌─────────────────────┐
// │  BOUNDARY (parse)    │  ← Zod / Smart constructors
// │  string → Email      │  ← untrusted → branded
// │  unknown → User      │  ← API response → typed
// └─────────────────────┘
//     │
//     ▼ Branded types only!
// ┌─────────────────────┐
// │  CORE (pure logic)   │  ← Chỉ nhận branded types
// │  No validation!      │  ← Types = proof of validity
// │  applyDiscount(VND)  │  ← VND đã parsed → trust
// └─────────────────────┘
//     │
//     ▼
// UNTRUSTED WORLD (output)

type Brand<T, B extends string> = T & { readonly __brand: B };
type Email = Brand<string, "Email">;
type UserId = Brand<string, "UserId">;
type Money = Brand<number, "Money">;

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// BOUNDARY: parse incoming data
const parseEmail = (input: string): Result<Email, string> => {
    const trimmed = input.trim().toLowerCase();
    if (!trimmed.includes("@")) return err("Invalid email");
    return ok(trimmed as Email);
};

const parseMoney = (input: number): Result<Money, string> => {
    if (!Number.isFinite(input)) return err("Invalid amount");
    if (input < 0) return err("Amount must be >= 0");
    return ok(Math.round(input) as Money);
};

// CORE: pure functions — NO validation, only branded types
// Compiler GUARANTEES email is valid, amount is valid
const sendReceipt = (email: Email, amount: Money): string =>
    `Receipt sent to ${email}: ${amount.toLocaleString()}đ`;

const applyDiscount = (amount: Money, percent: number): Money =>
    Math.round(amount * (1 - percent / 100)) as Money;

// BOUNDARY: entry point — parse then call core
const processPayment = (
    rawEmail: string,
    rawAmount: number,
    discountPercent: number
): Result<string, string> => {
    const emailResult = parseEmail(rawEmail);
    if (emailResult.tag === "err") return emailResult;

    const amountResult = parseMoney(rawAmount);
    if (amountResult.tag === "err") return amountResult;

    const discounted = applyDiscount(amountResult.value, discountPercent);
    return ok(sendReceipt(emailResult.value, discounted));
};

// Test
const result = processPayment("An@Mail.Com", 500000, 10);
assert.strictEqual(result.tag, "ok");
if (result.tag === "ok") {
    assert.ok(result.value.includes("an@mail.com"));
    assert.ok(result.value.includes("450,000"));
}

const badEmail = processPayment("invalid", 500000, 10);
assert.strictEqual(badEmail.tag, "err");

console.log("Boundary validation OK ✅");
```

Hãy chú ý `sendReceipt` và `applyDiscount` — hai core functions. Chúng nhận `Email` và `Money`, không phải `string` và `number`. Không có `if` nào kiểm tra validity bên trong — vì TYPE ĐÃ LÀ BẰNG CHỨNG. Nếu ai đó cố truyền `string` vào `sendReceipt`, compiler từ chối. Đây là sức mạnh thực sự của "parse, don't validate": validation code **chỉ tồn tại ở biên**, core logic hoàn toàn sạch.

> **💡 "Validate once, trust forever"**: Parse ở boundary (API handler, CLI entry, form submit). Core functions nhận branded types — KHÔNG validate lại. Type = proof of validity. Đơn giản, an toàn, nhanh.

---

## ✅ Checkpoint 14.6

> Đến đây bạn phải hiểu:
> 1. **Boundary** = parse untrusted input → branded types
> 2. **Core** = pure functions, nhận branded types, KHÔNG validate
> 3. **Type = proof**: `Email` = đã validated. Không cần check lại
> 4. Architecture: **Untrusted → Boundary (parse) → Core (trust) → Output**
>
> **Test nhanh**: Core function nhận `amount: number` thay vì `amount: Money` — vấn đề gì?
> <details><summary>Đáp án</summary>Bất kỳ number nào (âm, NaN, Infinity) đều pass! Core mất safety. Dùng `Money` = compiler GUARANTEE amount đã parsed.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Smart constructor

Viết smart constructor cho `type Url = Brand<string, "Url">`:
- Phải bắt đầu bằng `http://` hoặc `https://`
- Phải có domain (vd: `google.com`)
- Trim whitespace
- Trả `Result<Url, string>`

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
type Brand<T, B extends string> = T & { readonly __brand: B };
type Url = Brand<string, "Url">;

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

const Url = (input: string): Result<Url, string> => {
    const trimmed = input.trim();
    if (!trimmed.startsWith("http://") && !trimmed.startsWith("https://"))
        return err("URL phải bắt đầu bằng http:// hoặc https://");

    const afterProtocol = trimmed.replace(/^https?:\/\//, "");
    if (!afterProtocol.includes("."))
        return err("URL phải có domain (vd: google.com)");

    return ok(trimmed as Url);
};

// Test
import assert from "node:assert/strict";
assert.strictEqual(Url("  https://google.com  ").tag, "ok");
assert.strictEqual(Url("ftp://wrong.com").tag, "err");
assert.strictEqual(Url("https://nodomain").tag, "err");
```

</details>

---

**Bài 2** (10 phút): Zod schema

Viết Zod schema cho API response:

```typescript
// POST /api/orders response:
// {
//   "orderId": "ORD-123",
//   "items": [{ "name": "Laptop", "price": 20000000, "quantity": 1 }],
//   "status": "pending" | "confirmed" | "shipped",
//   "createdAt": "2024-01-15T10:30:00Z" (ISO string → Date),
//   "coupon": null | { "code": string, "discount": number (0-100) }
// }
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import { z } from "zod";
import assert from "node:assert/strict";

const OrderItemSchema = z.object({
    name: z.string().min(1),
    price: z.number().nonnegative(),
    quantity: z.number().int().positive(),
});

const CouponSchema = z.object({
    code: z.string().min(1),
    discount: z.number().min(0).max(100),
});

const OrderResponseSchema = z.object({
    orderId: z.string().startsWith("ORD-"),
    items: z.array(OrderItemSchema).nonempty(),
    status: z.enum(["pending", "confirmed", "shipped"]),
    createdAt: z.string().datetime().transform(s => new Date(s)),
    coupon: CouponSchema.nullable(),
});

type OrderResponse = z.infer<typeof OrderResponseSchema>;

// Test
const apiResponse = {
    orderId: "ORD-123",
    items: [{ name: "Laptop", price: 20000000, quantity: 1 }],
    status: "pending",
    createdAt: "2024-01-15T10:30:00Z",
    coupon: null,
};

const result = OrderResponseSchema.safeParse(apiResponse);
assert.strictEqual(result.success, true);
if (result.success) {
    assert.ok(result.data.createdAt instanceof Date);  // transformed!
}
```

</details>

---

**Bài 3** (15 phút): Full validation pipeline

Viết validation cho "create product" form:

```typescript
// Input (từ HTML form — TẤT CẢ là strings):
type ProductForm = {
    readonly name: string;       // >= 3 chars, <= 100 chars
    readonly price: string;      // parse → number > 0
    readonly category: string;   // must be "electronics" | "clothing" | "food"
    readonly stock: string;      // parse → integer >= 0
    readonly description: string; // optional, <= 500 chars
};

// Output:
type Product = {
    readonly name: string;
    readonly price: number;
    readonly category: "electronics" | "clothing" | "food";
    readonly stock: number;
    readonly description: string | undefined;
};

// Requirements:
// 1. Accumulate ALL errors (FieldErrors = Map<field, messages[]>)
// 2. Parse strings → proper types
// 3. Return Result<Product, FieldErrors>
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

type FieldErrors = ReadonlyMap<string, readonly string[]>;

const addError = (
    errors: FieldErrors, field: string, message: string
): FieldErrors => {
    const existing = errors.get(field) ?? [];
    return new Map([...errors, [field, [...existing, message]]]);
};

type ProductForm = {
    readonly name: string;
    readonly price: string;
    readonly category: string;
    readonly stock: string;
    readonly description: string;
};

type Product = {
    readonly name: string;
    readonly price: number;
    readonly category: "electronics" | "clothing" | "food";
    readonly stock: number;
    readonly description: string | undefined;
};

const VALID_CATEGORIES = ["electronics", "clothing", "food"] as const;

const validateProduct = (form: ProductForm): Result<Product, FieldErrors> => {
    let errors: FieldErrors = new Map();

    // Name
    const name = form.name.trim();
    if (name.length < 3) errors = addError(errors, "name", "Tối thiểu 3 ký tự");
    if (name.length > 100) errors = addError(errors, "name", "Tối đa 100 ký tự");

    // Price (string → number)
    const price = Number(form.price);
    if (form.price.trim() === "" || Number.isNaN(price))
        errors = addError(errors, "price", "Giá phải là số");
    else if (price <= 0)
        errors = addError(errors, "price", "Giá phải > 0");

    // Category
    const category = form.category.trim().toLowerCase();
    if (!VALID_CATEGORIES.includes(category as typeof VALID_CATEGORIES[number]))
        errors = addError(errors, "category", "Danh mục: electronics, clothing, food");

    // Stock (string → integer)
    const stock = Number(form.stock);
    if (form.stock.trim() === "" || Number.isNaN(stock))
        errors = addError(errors, "stock", "Tồn kho phải là số");
    else if (!Number.isInteger(stock) || stock < 0)
        errors = addError(errors, "stock", "Tồn kho: số nguyên >= 0");

    // Description (optional)
    const description = form.description.trim() || undefined;
    if (description && description.length > 500)
        errors = addError(errors, "description", "Mô tả tối đa 500 ký tự");

    if (errors.size > 0) return err(errors);

    return ok({
        name,
        price,
        category: category as Product["category"],
        stock,
        description,
    });
};

// Test: all errors
const bad: ProductForm = {
    name: "ab",
    price: "abc",
    category: "unknown",
    stock: "-5",
    description: "",
};

const result = validateProduct(bad);
assert.strictEqual(result.tag, "err");
if (result.tag === "err") {
    assert.ok(result.error.has("name"));
    assert.ok(result.error.has("price"));
    assert.ok(result.error.has("category"));
    assert.ok(result.error.has("stock"));
}

// Test: valid
const good: ProductForm = {
    name: "Laptop Pro",
    price: "25000000",
    category: "electronics",
    stock: "10",
    description: "Great laptop",
};

const goodResult = validateProduct(good);
assert.strictEqual(goodResult.tag, "ok");
if (goodResult.tag === "ok") {
    assert.strictEqual(goodResult.value.price, 25000000);
    assert.strictEqual(goodResult.value.stock, 10);
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Smart constructor bypass bằng `as` | TypeScript cho phép `"invalid" as Email` | Convention: team đồng ý KHÔNG dùng `as` ngoài smart constructors |
| Zod `parse` throws | Dùng `parse` thay vì `safeParse` | Luôn dùng `safeParse` — trả Result-like, không exceptions |
| Form validation thiếu per-field errors | Dùng `string` error thay vì `Map<field, string[]>` | Dùng `FieldErrors = Map<string, string[]>` cho UI mapping |
| Validate ở core thay vì boundary | Core functions nhận raw types (`string`) | Refactor: boundary parse → branded types. Core chỉ nhận branded |
| Accumulate thất bại cho dependent validations | Validate dependent field khi dependency invalid | Check dependency first: `if (password valid) then check confirmPassword` |

---

## Tóm tắt

Chương này đã đưa bạn từ con dấu hộ chiếu đơn giản (smart constructor) qua hệ thống hải quan hoàn chỉnh (Zod, accumulating errors) đến kiến trúc biên giới - lõi (boundary validation). Mỗi tầng xây trên tầng trước:

- ✅ **"Parse, don't validate"** — check + change type. Smart constructors = parsers.
- ✅ **Smart constructors** = validate + normalize + brand. Return `Result<Branded, Error>`.
- ✅ **Zod** = schema-first validation. `z.infer<>` = TypeScript type. `safeParse` = Result-like.
- ✅ **Accumulating errors** = collect ALL errors. `FieldErrors = Map<field, messages[]>`. Better UX.
- ✅ **Validation pipelines** = compose validators bằng chain. Railway + accumulate hỗn hợp.
- ✅ **Boundary validation** = parse at edges, trust in core. Type = proof of validity.

Bạn giờ có hệ thống validation hoàn chỉnh: smart constructors cho từng type, Zod cho objects phức tạp, accumulating cho UX, boundary architecture cho kiến trúc sạch. Nhưng tất cả validators và pipelines ở trên đều working với **concrete types** — `string`, `number`, `Email`. Nếu bạn muốn viết MỘT `map` function hoạt động cho cả `Option`, `Result`, và `Array` — bạn cần **Generics**. Chương tiếp theo sẽ đi sâu vào thế giới type-level programming.

## Tiếp theo

→ Chapter 15: **Generics Deep Dive** — Constraints `extends`, conditional types advanced, `infer` patterns, parametric polymorphism, và real-world generic utilities.
