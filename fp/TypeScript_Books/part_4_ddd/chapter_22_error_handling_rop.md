# Chapter 22 — Error Handling: Railway-Oriented Programming

> **Bạn sẽ học được**:
> - Tại sao `try/catch` không phải FP way
> - `Result<T, E>` deep dive — từ Ch13 đến production-grade
> - `neverthrow` library — typed errors trong TypeScript thực tế
> - `ResultAsync` — async operations với Result
> - Error accumulation — thu thập TẤT CẢ lỗi, không fail-fast
> - Giới thiệu Effect — next-generation error handling
>
> **Yêu cầu trước**: Chapter 13 (Result type), Chapter 21 (pipelines, andThen, ROP preview).
> **Thời gian đọc**: ~50 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Error handling production-grade — typed, composable, no try/catch.

---

Bạn đã đi tàu hỏa chưa?

Đường ray có **ghi** (switch/turnout) — nơi tàu rẽ sang track khác. Khi tàu gặp tín hiệu xanh: đi thẳng (happy track). Tín hiệu đỏ: rẽ sang track phụ (error track). Và đây là phần hay: **tàu trên error track vẫn di chuyển** — nó không "crash", nó chuyển sang hành trình khác (sửa chữa, quay lại bến). Tàu KHÔNG bao giờ "dừng giữa đường và cháy nổ" — đó mới là `throw new Error()`.

**Railway-Oriented Programming** (ROP, Scott Wlaschin) mô hình hóa error handling như đường ray: mỗi function là một đoạn ray có ghi. Input ok → đi happy track. Input lỗi hoặc function thất bại → rẽ error track. Các function tiếp theo trên happy track bị **skip** — tàu "trượt" trên error track cho đến ga cuối, nơi bạn xử lý lỗi. Kết quả: **no try/catch, no throw, no uncaught exceptions**. Type system đảm bảo mọi lỗi đều được handle.

---

## 22.1 — Vấn đề với `try/catch`

### Exception = goto hiện đại

`throw` giống `goto` — nhảy đến bất kỳ đâu, caller không biết function có throw hay không. TypeScript KHÔNG type-check exceptions: `function divide(a: number, b: number): number` — signature không nói "có thể throw DivisionByZero". Caller phải tự nhớ wrap `try/catch` — quên = runtime crash. Và `catch(e)` — `e` là `unknown`, không biết loại error gì.

```typescript
// filename: src/try_catch_problems.ts

// ❌ Vấn đề 1: Signature không nói có throw
const divide = (a: number, b: number): number => {
    if (b === 0) throw new Error("Division by zero");  // Ẩn trong implementation!
    return a / b;
};

// Caller KHÔNG BIẾT divide có thể throw
// const result = divide(10, 0);
// Runtime: "Error: Division by zero" → crash
// TypeScript: không warning nào!

// ❌ Vấn đề 2: catch(e) — e là unknown
// try {
//     const user = await fetchUser(id);
//     const order = await createOrder(user);
// } catch (e) {
//     // e là gì? NetworkError? ValidationError? DatabaseError?
//     // Không biết! Phải instanceof check hoặc cast
//     if (e instanceof Error) console.log(e.message);
// }

// ❌ Vấn đề 3: try/catch phá composition
// Không thể pipe() hay flow() functions có throw
// pipe(input, validate, save)  ← nếu validate throw, pipe() crash
```

### FP way: lỗi = GIẤU TRỊ, không phải ngoại lệ

```typescript
// filename: src/result_approach.ts
import assert from "node:assert/strict";

// ✅ FP: error là VALUE trong return type
type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// Signature NÓI RÕ có thể fail
const divideSafe = (a: number, b: number): Result<number, "division_by_zero"> =>
    b === 0 ? err("division_by_zero") : ok(a / b);

// Caller BẮT BUỘC phải handle cả ok và err
const result = divideSafe(10, 0);
switch (result.tag) {
    case "ok":
        console.log(`Result: ${result.value}`);
        break;
    case "err":
        console.log(`Error: ${result.error}`);  // typed! "division_by_zero"
        break;
}

// ✅ Composable — works with pipe()
const result2 = divideSafe(10, 2);
assert.strictEqual(result2.tag, "ok");
if (result2.tag === "ok") {
    assert.strictEqual(result2.value, 5);
}

console.log("Result approach OK ✅");
```

| | `try/catch` | `Result<T, E>` |
|--|------------|----------------|
| **Error visibility** | Hidden in implementation | In return type — caller SEES it |
| **Type safety** | `catch(e: unknown)` — no type | `error: E` — fully typed |
| **Composition** | Breaks pipe/flow | Works with andThen/map |
| **Forgettable?** | YES — forget catch = crash | NO — must handle Result |
| **Performance** | Exception = expensive (stack trace) | Result = cheap (object) |

> **💡 "Honesty in types"**: Return type `Result<User, "not_found" | "db_error">` TELLS the caller all possible outcomes. `throw` hides them. Type honesty = fewer bugs.

---

## ✅ Checkpoint 22.1

> Đến đây bạn phải hiểu:
> 1. **`throw`** hides errors from type system. Caller doesn't know
> 2. **`Result<T, E>`** = error as VALUE in return type. Compiler enforces handling
> 3. **Typed errors**: `"division_by_zero"` vs `catch(e: unknown)` — night and day
> 4. **Composable**: Result works with pipe(), andThen(). Exceptions don't
>
> **Test nhanh**: `function fetchUser(id: string): User` — có thể fail không?
> <details><summary>Đáp án</summary>**KHÔNG BIẾT!** TypeScript signature không nói. Có thể throw NetworkError, NotFoundError, hoặc bất kỳ gì. FP: `Result<User, FetchError>` — nói RÕ có thể fail và fail bằng gì.</details>

---

## 22.2 — `Result<T, E>` Deep Dive

### Result utilities — production grade

Ch13 và Ch21 đã giới thiệu `ok`, `err`, `andThen`, `map`. Giờ bổ sung thêm utilities cần cho production: `mapErr` (transform error), `unwrapOr` (default value), `match` (pattern match), `fromThrowable` (wrap function có throw).

```typescript
// filename: src/result_utils.ts
import assert from "node:assert/strict";

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// Core combinators
const map = <T, U, E>(r: Result<T, E>, fn: (v: T) => U): Result<U, E> =>
    r.tag === "ok" ? ok(fn(r.value)) : r;

const mapErr = <T, E, F>(r: Result<T, E>, fn: (e: E) => F): Result<T, F> =>
    r.tag === "err" ? err(fn(r.error)) : r;

const andThen = <T, U, E>(r: Result<T, E>, fn: (v: T) => Result<U, E>): Result<U, E> =>
    r.tag === "ok" ? fn(r.value) : r;

// Extractors
const unwrapOr = <T, E>(r: Result<T, E>, defaultValue: T): T =>
    r.tag === "ok" ? r.value : defaultValue;

const match = <T, E, U>(
    r: Result<T, E>,
    handlers: { readonly ok: (v: T) => U; readonly err: (e: E) => U }
): U =>
    r.tag === "ok" ? handlers.ok(r.value) : handlers.err(r.error);

// Bridge: wrap throwable function → Result
const fromThrowable = <T>(fn: () => T): Result<T, Error> => {
    try {
        return ok(fn());
    } catch (e) {
        return err(e instanceof Error ? e : new Error(String(e)));
    }
};

// --- Tests ---
const good: Result<number, string> = ok(42);
const bad: Result<number, string> = err("oops");

// map
assert.deepStrictEqual(map(good, x => x * 2), ok(84));
assert.deepStrictEqual(map(bad, x => x * 2), err("oops"));  // skip!

// mapErr
assert.deepStrictEqual(mapErr(bad, e => `Error: ${e}`), err("Error: oops"));
assert.deepStrictEqual(mapErr(good, e => `Error: ${e}`), ok(42));  // skip!

// unwrapOr
assert.strictEqual(unwrapOr(good, 0), 42);
assert.strictEqual(unwrapOr(bad, 0), 0);  // default

// match
const message = match(good, {
    ok: v => `Got ${v}`,
    err: e => `Failed: ${e}`,
});
assert.strictEqual(message, "Got 42");

// fromThrowable
const parsed = fromThrowable(() => JSON.parse('{"a": 1}'));
assert.strictEqual(parsed.tag, "ok");

const invalid = fromThrowable(() => JSON.parse("not json"));
assert.strictEqual(invalid.tag, "err");

console.log("Result utils OK ✅");
```

### Combining multiple Results

```typescript
// filename: src/combine_results.ts
import assert from "node:assert/strict";

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// combine: all ok → ok(tuple). Any err → first err
const combine = <T extends readonly Result<unknown, unknown>[]>(
    results: T
): Result<
    { [K in keyof T]: T[K] extends Result<infer V, unknown> ? V : never },
    T[number] extends Result<unknown, infer E> ? E : never
> => {
    const values: unknown[] = [];
    for (const r of results) {
        if (r.tag === "err") return r as any;
        values.push((r as any).value);
    }
    return ok(values as any);
};

// Test
const r1 = ok(1);
const r2 = ok("hello");
const r3 = ok(true);

const combined = combine([r1, r2, r3] as const);
assert.strictEqual(combined.tag, "ok");
if (combined.tag === "ok") {
    assert.deepStrictEqual(combined.value, [1, "hello", true]);
}

// One err → fail
const withErr = combine([ok(1), err("oops"), ok(3)] as const);
assert.strictEqual(withErr.tag, "err");

console.log("Combine results OK ✅");
```

> **💡 Result combinators summary**: `map` = transform ok. `mapErr` = transform err. `andThen` = chain (may fail). `unwrapOr` = extract with default. `match` = pattern match. `combine` = all-or-nothing. `fromThrowable` = bridge to throwable code.

---

## ✅ Checkpoint 22.2

> Đến đây bạn phải hiểu:
> 1. **`map`** = transform ok, skip err. **`mapErr`** = transform err, skip ok
> 2. **`andThen`** = chain fallible operations. **`unwrapOr`** = safe extract
> 3. **`match`** = exhaustive pattern matching on Result
> 4. **`fromThrowable`** = bridge throwable → Result. **`combine`** = all-or-nothing
>
> **Test nhanh**: `map(err("fail"), x => x * 2)` kết quả gì?
> <details><summary>Đáp án</summary>`err("fail")` — `map` SKIP fn khi result là err. Chỉ transform ok values.</details>

---

## 22.3 — `neverthrow` Library

### Production-grade Result cho TypeScript

Tự viết Result utilities tốt cho học tập — nhưng production cần library battle-tested. **`neverthrow`** là library nhẹ (~3KB) cho Result pattern trong TypeScript, với `ResultAsync` cho async operations.

```typescript
// filename: src/neverthrow_intro.ts
// import { ok, err, Result, ResultAsync } from "neverthrow";
// (Ví dụ mô phỏng neverthrow API)

import assert from "node:assert/strict";

// Simulating neverthrow API cho book examples
type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T; readonly isOk: () => true; readonly isErr: () => false }
    | { readonly tag: "err"; readonly error: E; readonly isOk: () => false; readonly isErr: () => true };

const ok = <T>(value: T): Result<T, never> => ({
    tag: "ok", value,
    isOk: () => true as const,
    isErr: () => false as const,
});

const err = <E>(error: E): Result<never, E> => ({
    tag: "err", error,
    isOk: () => false as const,
    isErr: () => true as const,
});

// neverthrow-style methods (standalone functions for our examples)
const andThen = <T, U, E>(r: Result<T, E>, fn: (v: T) => Result<U, E>): Result<U, E> =>
    r.tag === "ok" ? fn(r.value) : r;

const map = <T, U, E>(r: Result<T, E>, fn: (v: T) => U): Result<U, E> =>
    r.tag === "ok" ? ok(fn(r.value)) : r;

const mapErr = <T, E, F>(r: Result<T, E>, fn: (e: E) => F): Result<T, F> =>
    r.tag === "err" ? err(fn(r.error)) : r;

// --- Domain example: User registration ---

type RegistrationError =
    | { readonly type: "INVALID_EMAIL"; readonly email: string }
    | { readonly type: "WEAK_PASSWORD"; readonly reason: string }
    | { readonly type: "DUPLICATE_EMAIL"; readonly email: string };

type ValidEmail = string & { readonly __brand: "ValidEmail" };
type HashedPassword = string & { readonly __brand: "HashedPassword" };

const validateEmail = (email: string): Result<ValidEmail, RegistrationError> => {
    const trimmed = email.trim().toLowerCase();
    return trimmed.includes("@") && trimmed.includes(".")
        ? ok(trimmed as ValidEmail)
        : err({ type: "INVALID_EMAIL", email });
};

const validatePassword = (password: string): Result<string, RegistrationError> =>
    password.length >= 8
        ? ok(password)
        : err({ type: "WEAK_PASSWORD", reason: `Need 8+ chars, got ${password.length}` });

const hashPassword = (password: string): Result<HashedPassword, never> =>
    ok(`hashed_${password}` as HashedPassword);

// Pipeline
const registerUser = (
    email: string,
    password: string,
): Result<{ email: ValidEmail; passwordHash: HashedPassword }, RegistrationError> => {
    const validEmail = validateEmail(email);
    if (validEmail.tag === "err") return validEmail;

    const validPassword = validatePassword(password);
    if (validPassword.tag === "err") return validPassword;

    const hashed = hashPassword(validPassword.value);
    if (hashed.tag === "err") return hashed;

    return ok({ email: validEmail.value, passwordHash: hashed.value });
};

// Tests
const goodReg = registerUser("An@Mail.com", "strongPass123");
assert.strictEqual(goodReg.tag, "ok");
if (goodReg.tag === "ok") {
    assert.strictEqual(goodReg.value.email, "an@mail.com");
    assert.ok(goodReg.value.passwordHash.startsWith("hashed_"));
}

const badEmail = registerUser("not-email", "strongPass123");
assert.strictEqual(badEmail.tag, "err");
if (badEmail.tag === "err") {
    assert.strictEqual(badEmail.error.type, "INVALID_EMAIL");
}

const weakPw = registerUser("ok@mail.com", "short");
assert.strictEqual(weakPw.tag, "err");
if (weakPw.tag === "err") {
    assert.strictEqual(weakPw.error.type, "WEAK_PASSWORD");
}

console.log("neverthrow-style OK ✅");
```

> **💡 neverthrow in production**: `npm install neverthrow`. Methods: `.map()`, `.mapErr()`, `.andThen()`, `.match()`, `.isOk()`, `.isErr()`. ~3KB, zero dependencies, full TypeScript support.

---

## ✅ Checkpoint 22.3

> Đến đây bạn phải hiểu:
> 1. **neverthrow** = production-grade Result library (~3KB)
> 2. **Typed errors**: `RegistrationError` = DU of possible error types
> 3. **Pipeline**: validateEmail → validatePassword → hashPassword → register
> 4. **Each step**: returns `Result<Success, Error>` — all typed
>
> **Test nhanh**: `neverthrow` cần DI container không?
> <details><summary>Đáp án</summary>**KHÔNG!** `neverthrow` chỉ là Result type + utilities. Không có IoC, không DI container. Function parameters = DI (Ch19).</details>

---

## 22.4 — `ResultAsync` — Async Error Handling

### Tàu chạy qua đường hầm (async) vẫn trên track

Async operations (database, API, email) = đường hầm dài — tàu vào, chờ, rồi ra. `ResultAsync` wraps `Promise<Result<T, E>>` — async operation mà vẫn giữ ROP semantics. Chain `.andThen()` trên `ResultAsync` tự await + rẽ ray nếu err.

```typescript
// filename: src/result_async.ts
import assert from "node:assert/strict";

// Simulating ResultAsync (neverthrow-style)
type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// ResultAsync = Promise<Result<T, E>> with chainable methods
type ResultAsync<T, E> = Promise<Result<T, E>>;

const okAsync = <T>(value: T): ResultAsync<T, never> =>
    Promise.resolve(ok(value));

const errAsync = <E>(error: E): ResultAsync<never, E> =>
    Promise.resolve(err(error));

// andThenAsync: chain async operations
const andThenAsync = <T, U, E>(
    resultAsync: ResultAsync<T, E>,
    fn: (value: T) => ResultAsync<U, E>,
): ResultAsync<U, E> =>
    resultAsync.then(r => r.tag === "ok" ? fn(r.value) : Promise.resolve(r));

const mapAsync = <T, U, E>(
    resultAsync: ResultAsync<T, E>,
    fn: (value: T) => U,
): ResultAsync<U, E> =>
    resultAsync.then(r => r.tag === "ok" ? ok(fn(r.value)) : r);

// --- Domain: Order processing with async steps ---

type OrderError =
    | { readonly type: "NOT_FOUND"; readonly orderId: string }
    | { readonly type: "PAYMENT_FAILED"; readonly reason: string }
    | { readonly type: "EMAIL_FAILED"; readonly email: string };

type Order = {
    readonly id: string;
    readonly customerId: string;
    readonly total: number;
    readonly status: "draft" | "confirmed" | "paid";
};

// Async steps
const findOrder = (orderId: string): ResultAsync<Order, OrderError> => {
    // Simulate DB lookup
    const orders: Record<string, Order> = {
        "ORD-1": { id: "ORD-1", customerId: "C1", total: 500000, status: "confirmed" },
    };
    const order = orders[orderId];
    return order ? okAsync(order) : errAsync({ type: "NOT_FOUND", orderId });
};

const processPayment = (order: Order): ResultAsync<Order, OrderError> => {
    // Simulate payment gateway
    if (order.total > 10000000) {
        return errAsync({ type: "PAYMENT_FAILED", reason: "Amount exceeds limit" });
    }
    return okAsync({ ...order, status: "paid" as const });
};

const sendConfirmation = (order: Order): ResultAsync<Order, OrderError> => {
    // Simulate email
    return okAsync(order);  // assume success
};

// Async pipeline
const processOrder = (orderId: string): ResultAsync<Order, OrderError> =>
    andThenAsync(
        andThenAsync(findOrder(orderId), processPayment),
        sendConfirmation,
    );

// Test
const run = async () => {
    // Happy path
    const good = await processOrder("ORD-1");
    assert.strictEqual(good.tag, "ok");
    if (good.tag === "ok") {
        assert.strictEqual(good.value.status, "paid");
    }

    // Not found
    const notFound = await processOrder("ORD-999");
    assert.strictEqual(notFound.tag, "err");
    if (notFound.tag === "err") {
        assert.strictEqual(notFound.error.type, "NOT_FOUND");
    }

    console.log("ResultAsync OK ✅");
};

run();
```

> **💡 ResultAsync pattern**: `findOrder(id)` returns `ResultAsync<Order, Error>`. Chain with `andThenAsync`. Error at ANY step → remaining steps SKIPPED. No try/catch anywhere!

---

## ✅ Checkpoint 22.4

> Đến đây bạn phải hiểu:
> 1. **`ResultAsync<T, E>`** = `Promise<Result<T, E>>` with chain support
> 2. **`andThenAsync`** = async andThen. Error → skip remaining async steps
> 3. **No try/catch**: async errors become Result errors, not exceptions
> 4. **In `neverthrow`**: `ResultAsync` has `.andThen()`, `.map()`, `.mapErr()` as methods
>
> **Test nhanh**: `andThenAsync(errAsync("fail"), expensiveDbCall)` — DB call xảy ra không?
> <details><summary>Đáp án</summary>**KHÔNG!** `andThenAsync` check: input là err → return err immediately, SKIP fn. DB call never executed. Railway magic!</details>

---

## 22.5 — Error Accumulation

### Thu thập TẤT CẢ lỗi, không chỉ lỗi đầu tiên

`andThen` fail-fast: gặp error đầu tiên → dừng. Form validation? User nhập 5 field sai — bạn muốn hiển thị TẤT CẢ 5 lỗi cùng lúc, không "sửa 1 → submit → sửa 1 → submit" 5 lần. Error accumulation = validate tất cả fields, collect errors, trả về danh sách.

```typescript
// filename: src/error_accumulation.ts
import assert from "node:assert/strict";

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// Accumulate: validate ALL, collect ALL errors
type ValidationError = {
    readonly field: string;
    readonly message: string;
};

type UserForm = {
    readonly name: string;
    readonly email: string;
    readonly age: string;
    readonly password: string;
};

// Individual validators (each returns Result)
const validateName = (name: string): Result<string, ValidationError> =>
    name.trim().length >= 2
        ? ok(name.trim())
        : err({ field: "name", message: "Tên phải ≥ 2 ký tự" });

const validateEmail = (email: string): Result<string, ValidationError> =>
    email.includes("@") && email.includes(".")
        ? ok(email.toLowerCase().trim())
        : err({ field: "email", message: "Email không hợp lệ" });

const validateAge = (ageStr: string): Result<number, ValidationError> => {
    const age = Number(ageStr);
    return Number.isInteger(age) && age >= 18 && age <= 120
        ? ok(age)
        : err({ field: "age", message: "Tuổi phải 18-120, là số nguyên" });
};

const validatePassword = (password: string): Result<string, ValidationError> =>
    password.length >= 8
        ? ok(password)
        : err({ field: "password", message: "Mật khẩu phải ≥ 8 ký tự" });

// Accumulate ALL errors
type ValidUser = {
    readonly name: string;
    readonly email: string;
    readonly age: number;
    readonly password: string;
};

const validateUserForm = (form: UserForm): Result<ValidUser, readonly ValidationError[]> => {
    const results = [
        validateName(form.name),
        validateEmail(form.email),
        validateAge(form.age),
        validatePassword(form.password),
    ];

    const errors: ValidationError[] = [];
    const values: unknown[] = [];

    for (const r of results) {
        if (r.tag === "err") errors.push(r.error);
        else values.push(r.value);
    }

    if (errors.length > 0) return err(errors);

    return ok({
        name: values[0] as string,
        email: values[1] as string,
        age: values[2] as number,
        password: values[3] as string,
    });
};

// Test: all valid
const goodForm = validateUserForm({
    name: "An", email: "an@mail.com", age: "25", password: "strongPass123"
});
assert.strictEqual(goodForm.tag, "ok");

// Test: multiple errors at once!
const badForm = validateUserForm({
    name: "A", email: "not-email", age: "abc", password: "short"
});
assert.strictEqual(badForm.tag, "err");
if (badForm.tag === "err") {
    assert.strictEqual(badForm.error.length, 4);  // ALL 4 errors collected!
    assert.strictEqual(badForm.error[0].field, "name");
    assert.strictEqual(badForm.error[1].field, "email");
    assert.strictEqual(badForm.error[2].field, "age");
    assert.strictEqual(badForm.error[3].field, "password");
}

// Test: partial errors
const partialForm = validateUserForm({
    name: "An", email: "not-email", age: "25", password: "short"
});
assert.strictEqual(partialForm.tag, "err");
if (partialForm.tag === "err") {
    assert.strictEqual(partialForm.error.length, 2);  // email + password
}

console.log("Error accumulation OK ✅");
```

| Pattern | Behavior | Use case |
|---------|----------|----------|
| **Fail-fast** (`andThen`) | Stop at first error | Sequential dependencies |
| **Accumulate** | Collect ALL errors | Form validation, parallel checks |

> **💡 Fail-fast vs Accumulate**: andThen = fail-fast (step 2 depends on step 1). Accumulate = parallel validation (fields independent). Choose based on dependencies between validations.

---

## ✅ Checkpoint 22.5

> Đến đây bạn phải hiểu:
> 1. **Fail-fast** = `andThen` — stop at first error. For sequential deps
> 2. **Accumulate** = collect ALL errors. For independent validations
> 3. **Form validation** = accumulate. Show ALL errors at once
> 4. **Return type**: `Result<ValidUser, readonly ValidationError[]>` — array of errors
>
> **Test nhanh**: 3 fields sai → user thấy bao nhiêu lỗi với accumulation?
> <details><summary>Đáp án</summary>**3 lỗi!** Tất cả được thu thập. Fail-fast chỉ show 1 lỗi đầu tiên. UX tốt = show tất cả cùng lúc.</details>

---

## 22.6 — Giới thiệu Effect (Preview)

### Beyond neverthrow — Effect ecosystem

`Effect` (trước đây `@effect/io`) là framework FP cho TypeScript, mạnh hơn `neverthrow` nhiều: typed errors + dependency injection + resource management + concurrency + observability + retries. Đây là preview — Ch25-30 sẽ đào sâu.

```typescript
// filename: src/effect_preview.ts
// import { Effect, pipe } from "effect";
// (Ví dụ mô phỏng Effect API concepts)

import assert from "node:assert/strict";

// Effect<Success, Error, Requirements>
// - Success: what it produces when it succeeds
// - Error: what errors it can fail with
// - Requirements: what services it needs (DI!)

// Simulating Effect concepts with our Result
type Effect<S, E> = () => Promise<
    | { readonly tag: "ok"; readonly value: S }
    | { readonly tag: "err"; readonly error: E }
>;

const succeed = <S>(value: S): Effect<S, never> =>
    () => Promise.resolve({ tag: "ok", value });

const fail = <E>(error: E): Effect<never, E> =>
    () => Promise.resolve({ tag: "err", error });

const flatMap = <S, S2, E, E2>(
    effect: Effect<S, E>,
    fn: (value: S) => Effect<S2, E2>,
): Effect<S2, E | E2> =>
    async () => {
        const result = await effect();
        if (result.tag === "err") return result;
        return fn(result.value)();
    };

// Example: Effect-style program
type HttpError = { readonly _tag: "HttpError"; readonly status: number };
type ParseError = { readonly _tag: "ParseError"; readonly message: string };

const fetchUser = (id: string): Effect<{ name: string }, HttpError> =>
    id === "1"
        ? succeed({ name: "An" })
        : fail({ _tag: "HttpError", status: 404 });

const parseAge = (input: string): Effect<number, ParseError> => {
    const age = Number(input);
    return Number.isNaN(age)
        ? fail({ _tag: "ParseError", message: `"${input}" is not a number` })
        : succeed(age);
};

// Program = description of computation, NOT execution
const program = flatMap(fetchUser("1"), user =>
    flatMap(parseAge("25"), age =>
        succeed({ user: user.name, age })
    )
);

// Run (execute the program)
const run = async () => {
    const result = await program();
    assert.strictEqual(result.tag, "ok");
    if (result.tag === "ok") {
        assert.strictEqual(result.value.user, "An");
        assert.strictEqual(result.value.age, 25);
    }

    // Error case
    const badProgram = flatMap(fetchUser("999"), user =>
        succeed({ user: user.name })
    );
    const badResult = await badProgram();
    assert.strictEqual(badResult.tag, "err");
    if (badResult.tag === "err") {
        assert.strictEqual(badResult.error._tag, "HttpError");
    }

    console.log("Effect preview OK ✅");
};

run();
```

### neverthrow vs Effect — chọn cái nào?

| | `neverthrow` | `Effect` |
|--|-------------|----------|
| **Size** | ~3KB | ~50KB+ |
| **Learning curve** | Thấp | Cao |
| **Use case** | Simple Result-based error handling | Full FP framework |
| **Features** | Result, ResultAsync | + DI, resources, concurrency, retry, logging |
| **When** | Đủ cho hầu hết projects | Khi cần production-grade FP ecosystem |

> **💡 Rule of thumb**: Start with `neverthrow`. Grow into `Effect` when you need DI, resource management, or concurrency. Ch25-30 sẽ dạy Effect đầy đủ.

---

## ✅ Checkpoint 22.6

> Đến đây bạn phải hiểu:
> 1. **Effect** = full FP framework. `Effect<Success, Error, Requirements>`
> 2. **Effect = blueprint**: describe computation, run later. Lazy.
> 3. **Effect vs neverthrow**: Effect thêm DI, resources, concurrency, retry
> 4. **Start simple**: neverthrow → grow into Effect khi cần
>
> **Test nhanh**: `Effect<User, HttpError | ParseError>` nói gì về function?
> <details><summary>Đáp án</summary>Có thể **succeed** với `User`, có thể **fail** với `HttpError` HOẶC `ParseError`. Tất cả trong return type — không ẩn giấu. Type honesty!</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Result utilities

```typescript
// Cho: Result<number, string>
// 1. map: nhân đôi ok value
// 2. mapErr: prefix "Error: " vào error
// 3. unwrapOr: trả 0 nếu err
// 4. match: ok → "Got: {value}", err → "Failed: {error}"
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

const map = <T, U, E>(r: Result<T, E>, fn: (v: T) => U): Result<U, E> =>
    r.tag === "ok" ? ok(fn(r.value)) : r;

const mapErr = <T, E, F>(r: Result<T, E>, fn: (e: E) => F): Result<T, F> =>
    r.tag === "err" ? err(fn(r.error)) : r;

const unwrapOr = <T, E>(r: Result<T, E>, def: T): T =>
    r.tag === "ok" ? r.value : def;

const match = <T, E, U>(r: Result<T, E>, h: { ok: (v: T) => U; err: (e: E) => U }): U =>
    r.tag === "ok" ? h.ok(r.value) : h.err(r.error);

const good: Result<number, string> = ok(21);
const bad: Result<number, string> = err("oops");

assert.deepStrictEqual(map(good, x => x * 2), ok(42));
assert.deepStrictEqual(map(bad, x => x * 2), err("oops"));

assert.deepStrictEqual(mapErr(bad, e => `Error: ${e}`), err("Error: oops"));

assert.strictEqual(unwrapOr(good, 0), 21);
assert.strictEqual(unwrapOr(bad, 0), 0);

assert.strictEqual(match(good, { ok: v => `Got: ${v}`, err: e => `Failed: ${e}` }), "Got: 21");
assert.strictEqual(match(bad, { ok: v => `Got: ${v}`, err: e => `Failed: ${e}` }), "Failed: oops");
```

</details>

---

**Bài 2** (10 phút): Error accumulation form

```typescript
// Viết form validation cho "Product Creation":
// Fields: name (2-100 chars), price (> 0), stock (integer >= 0), sku (format: "ABC-1234")
// Accumulate ALL errors, return Result<ValidProduct, ValidationError[]>
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

type ValidationError = { readonly field: string; readonly message: string };

type ProductForm = {
    readonly name: string;
    readonly price: number;
    readonly stock: number;
    readonly sku: string;
};

type ValidProduct = {
    readonly name: string;
    readonly price: number;
    readonly stock: number;
    readonly sku: string;
};

const validateName = (name: string): Result<string, ValidationError> =>
    name.length >= 2 && name.length <= 100
        ? ok(name.trim())
        : err({ field: "name", message: "Name: 2-100 characters" });

const validatePrice = (price: number): Result<number, ValidationError> =>
    price > 0 ? ok(price) : err({ field: "price", message: "Price must be > 0" });

const validateStock = (stock: number): Result<number, ValidationError> =>
    Number.isInteger(stock) && stock >= 0
        ? ok(stock)
        : err({ field: "stock", message: "Stock: integer >= 0" });

const validateSku = (sku: string): Result<string, ValidationError> =>
    /^[A-Z]{3}-\d{4}$/.test(sku)
        ? ok(sku)
        : err({ field: "sku", message: "SKU format: ABC-1234" });

const validateProductForm = (form: ProductForm): Result<ValidProduct, readonly ValidationError[]> => {
    const results = [
        validateName(form.name),
        validatePrice(form.price),
        validateStock(form.stock),
        validateSku(form.sku),
    ];

    const errors = results
        .filter((r): r is Result<never, ValidationError> & { tag: "err" } => r.tag === "err")
        .map(r => r.error);

    if (errors.length > 0) return err(errors);

    return ok({ name: form.name.trim(), price: form.price, stock: form.stock, sku: form.sku });
};

// Tests
const good = validateProductForm({ name: "Laptop", price: 20000000, stock: 10, sku: "LAP-0001" });
assert.strictEqual(good.tag, "ok");

const allBad = validateProductForm({ name: "L", price: -1, stock: 1.5, sku: "nope" });
assert.strictEqual(allBad.tag, "err");
if (allBad.tag === "err") {
    assert.strictEqual(allBad.error.length, 4);
}
```

</details>

---

**Bài 3** (15 phút): Full async ROP workflow

```typescript
// Viết async order workflow dùng ResultAsync:
// 1. findCustomer(customerId) → ResultAsync<Customer, "not_found">
// 2. validateOrder(customer, items) → Result<ValidOrder, "empty_items" | "inactive_customer">
// 3. reserveStock(items) → ResultAsync<ReservedItems, "out_of_stock">
// 4. createOrder(customer, items) → ResultAsync<Order, "save_failed">
//
// Chain tất cả, test happy path + each error case
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

type OrderError = "not_found" | "empty_items" | "inactive_customer" | "out_of_stock" | "save_failed";

type Customer = { readonly id: string; readonly name: string; readonly active: boolean };
type Item = { readonly productId: string; readonly quantity: number };
type ValidOrder = { readonly customer: Customer; readonly items: readonly Item[] };
type Order = ValidOrder & { readonly orderId: string };

// Async steps
const findCustomer = async (id: string): Promise<Result<Customer, OrderError>> => {
    const customers: Record<string, Customer> = {
        "C1": { id: "C1", name: "An", active: true },
        "C2": { id: "C2", name: "Bình", active: false },
    };
    const c = customers[id];
    return c ? ok(c) : err("not_found");
};

// Sync step
const validateOrder = (customer: Customer, items: readonly Item[]): Result<ValidOrder, OrderError> =>
    !customer.active ? err("inactive_customer")
    : items.length === 0 ? err("empty_items")
    : ok({ customer, items });

const reserveStock = async (items: readonly Item[]): Promise<Result<readonly Item[], OrderError>> => {
    const outOfStock = items.find(i => i.quantity > 50);
    return outOfStock ? err("out_of_stock") : ok(items);
};

const createOrder = async (order: ValidOrder): Promise<Result<Order, OrderError>> =>
    ok({ ...order, orderId: `ORD-${Date.now()}` });

// Full workflow
const processOrder = async (customerId: string, items: readonly Item[]): Promise<Result<Order, OrderError>> => {
    const customerResult = await findCustomer(customerId);
    if (customerResult.tag === "err") return customerResult;

    const validated = validateOrder(customerResult.value, items);
    if (validated.tag === "err") return validated;

    const reserved = await reserveStock(validated.value.items);
    if (reserved.tag === "err") return reserved;

    return createOrder(validated.value);
};

// Tests
const run = async () => {
    const items: Item[] = [{ productId: "P1", quantity: 2 }];

    // Happy path
    const good = await processOrder("C1", items);
    assert.strictEqual(good.tag, "ok");

    // Not found
    const nf = await processOrder("C999", items);
    assert.strictEqual(nf.tag, "err");
    if (nf.tag === "err") assert.strictEqual(nf.error, "not_found");

    // Inactive
    const inactive = await processOrder("C2", items);
    assert.strictEqual(inactive.tag, "err");
    if (inactive.tag === "err") assert.strictEqual(inactive.error, "inactive_customer");

    // Empty items
    const empty = await processOrder("C1", []);
    assert.strictEqual(empty.tag, "err");
    if (empty.tag === "err") assert.strictEqual(empty.error, "empty_items");

    console.log("Full async ROP OK ✅");
};

run();
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "I still want to throw" | Habit | throw = hidden error. Result = visible. Discipline! |
| Error type `string` | Lazy error typing | Use DU: `{ type: "NOT_FOUND" } \| { type: "INVALID" }` |
| andThen deeply nested | Too many chained steps | Use local variables or neverthrow's `.andThen()` method syntax |
| "When to fail-fast vs accumulate?" | Context dependent | Sequential deps → fail-fast. Independent fields → accumulate |
| "neverthrow or Effect?" | Project size dependent | Small/medium → neverthrow. Large/complex → Effect |

---

## Tóm tắt

Chương này chuyển tàu error handling từ "crash and burn" (try/catch) sang "rẽ ray an toàn" (ROP). Error = value, không phải exception.

- ✅ **`try/catch` problems**: hidden errors, untyped, breaks composition.
- ✅ **`Result<T, E>`**: error as value in return type. Compiler enforces handling.
- ✅ **Utilities**: `map`, `mapErr`, `andThen`, `unwrapOr`, `match`, `combine`, `fromThrowable`.
- ✅ **`ResultAsync`**: async Result. Chain with `andThenAsync`. No try/catch.
- ✅ **Error accumulation**: collect ALL errors for form validation.
- ✅ **Effect preview**: next-gen FP framework. `Effect<Success, Error, Requirements>`.
- ✅ **neverthrow vs Effect**: start simple (neverthrow), grow when needed (Effect).

## Tiếp theo

→ Chapter 23: **Serialization & DTOs** — JSON + Zod schemas, Domain ↔ DTO mapping, boundary validation, API contracts.
