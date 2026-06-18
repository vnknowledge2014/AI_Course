# Chapter 13 — ADTs in TypeScript

> **Bạn sẽ học được**:
> - Algebraic Data Types (ADTs) — sum types + product types trong TypeScript
> - Discriminated Unions deep dive — multi-level DU, complex patterns
> - Option\<T\> — thay `null` bằng type-safe container
> - Result\<T, E\> — thay exceptions bằng values
> - Pattern matching — exhaustive handling, complex DU hierarchies
> - Branded types deep — domain modeling an toàn
>
> **Yêu cầu trước**: Chapter 12 (composition, pipe), Chapter 9 (branded types intro).
> **Thời gian đọc**: ~50 phút | **Level**: Intermediate → Advanced
> **Kết quả cuối cùng**: Model domain phức tạp bằng ADTs — compiler bắt bugs thay dev.

---

Bạn bước vào một tiệm kem. Trước mặt bạn có 3 vị: chocolate, vanilla, matcha. Bạn **chọn MỘT** — không thể lấy cả ba cùng lúc. Đó là **sum type**: tại mỗi thời điểm, giá trị chỉ là MỘT trong các variant.

Bây giờ hãy nhìn sang bộ Lego bạn vừa mua. Hộp ghi: "gồm 1 xe, 1 người, 1 cây". Bạn phải có **TẤT CẢ** mảnh ghép mới xây được mô hình hoàn chỉnh. Đó là **product type**: giá trị gồm TẤT CẢ các fields.

Hai ý tưởng này — "chọn 1 từ nhiều" (sum) và "cần tất cả" (product) — là nền tảng của **Algebraic Data Types**. Tên gọi "algebraic" không phải để hù dọa: nó chỉ đơn giản là phép "cộng" (sum = hoặc) và phép "nhân" (product = và) áp dụng cho types. Bạn đã gặp chúng ở Chapter 1 dưới góc nhìn toán học — giờ chúng ta biến chúng thành công cụ thực chiến để model domain, loại bỏ null, thay thế exceptions, và xây dựng state machines an toàn.

---

## 13.1 — ADTs: Sum Types + Product Types

### Từ tiệm kem đến type system

Hãy quay lại hai khái niệm cơ bản. **Product type** — bộ Lego — là object/struct mà bạn quen thuộc: `{ x: number; y: number }` nghĩa là "có x VÀ y". Số trạng thái khả dĩ bằng **tích** các fields: nếu `x` có 3 giá trị và `y` có 5 giá trị, product type có 3 × 5 = 15 trạng thái.

**Sum type** — tiệm kem — là discriminated union: `{ tag: "circle"; radius: number } | { tag: "rectangle"; width: number; height: number }`. Tại mỗi thời điểm, shape chỉ là circle HOẶC rectangle. Số trạng thái bằng **tổng** các variant: nếu circle có 10 trạng thái và rectangle có 20, sum type có 10 + 20 = 30.

Khi kết hợp sum + product, bạn có **ADT** — công cụ mạnh nhất để mô hình hóa domain sao cho compiler tự bắt bugs. Hãy xem:

```typescript
// filename: src/adt_basics.ts
import assert from "node:assert/strict";

// Product type — TẤT CẢ fields đều có
type Point = {
    readonly x: number;
    readonly y: number;
};
// Số states = number × number (vô hạn)

// Sum type — CHỈ MỘT variant tại mỗi thời điểm
type Shape =
    | { readonly tag: "circle"; readonly radius: number }
    | { readonly tag: "rectangle"; readonly width: number; readonly height: number }
    | { readonly tag: "triangle"; readonly base: number; readonly height: number };
// Số variants = 3 (circle HOẶC rectangle HOẶC triangle)

const area = (shape: Shape): number => {
    switch (shape.tag) {
        case "circle":
            return Math.PI * shape.radius ** 2;
        case "rectangle":
            return shape.width * shape.height;
        case "triangle":
            return (shape.base * shape.height) / 2;
    }
    // TypeScript biết: exhaustive! Không cần default
};

assert.strictEqual(area({ tag: "rectangle", width: 3, height: 4 }), 12);
assert.strictEqual(area({ tag: "triangle", base: 6, height: 4 }), 12);

console.log("ADT basics OK ✅");
```

Hãy chú ý switch statement ở trên: không có `default` case! TypeScript biết rằng `Shape` chỉ có 3 variant, và bạn đã xử lý hết cả 3. Nếu ngày mai bạn thêm variant `"polygon"` vào `Shape` nhưng quên cập nhật `area`, compiler sẽ báo lỗi ngay — không đợi đến runtime. Đây chính là "exhaustive checking", và nó là lý do ADTs mạnh hơn `if/else` hay enum.

### Tại sao "Algebraic"?

Vậy tại sao gọi là "algebraic"? Vì bạn có thể **tính** chính xác số trạng thái của type bằng phép cộng và nhân — giống đại số mà bạn học ở cấp 2. Điều này không chỉ là trò chơi toán học: khi bạn biết chính xác type có bao nhiêu trạng thái, bạn biết chính xác có bao nhiêu case cần xử lý. Không more, không less.

```typescript
// filename: src/why_algebraic.ts

// Product: states = tích các fields
type RGB = { readonly r: number; readonly g: number; readonly b: number };
// states = number × number × number

// Sum: states = tổng các variants
type Color =
    | { readonly tag: "rgb"; readonly r: number; readonly g: number; readonly b: number }
    | { readonly tag: "hex"; readonly value: string }
    | { readonly tag: "named"; readonly name: "red" | "green" | "blue" };
// states = (number³) + (string) + (3) = nhiều + nhiều + 3

// ADT = kết hợp sum + product
// Mỗi variant (sum) có fields riêng (product)
// → Mô hình hóa domain chính xác, KHÔNG có trạng thái vô nghĩa
```

> **💡 "Make illegal states unrepresentable"** (Ch1): ADTs = công cụ chính. Sum types giới hạn "cái gì có thể xảy ra". Product types mô tả "data của mỗi trường hợp".

---

## ✅ Checkpoint 13.1

> Đến đây bạn phải hiểu:
> 1. **Product type** = struct/object — "có A VÀ B". States = tích
> 2. **Sum type** = discriminated union — "là A HOẶC B". States = tổng
> 3. **ADT** = kết hợp sum + product. Mỗi variant có data riêng
> 4. TypeScript DU = sum type. Dùng `tag` field để phân biệt
>
> **Test nhanh**: `type T = { readonly tag: "a"; readonly x: boolean } | { readonly tag: "b"; readonly y: boolean }` — bao nhiêu states?
> <details><summary>Đáp án</summary>**4**: variant "a" có x=true|false (2) + variant "b" có y=true|false (2) = 2 + 2 = 4.</details>

---

## 13.2 — Option\<T\>: Thay `null` bằng Type Safety

### Vị kem "không có gì"

Hãy tưởng tượng bạn đến tiệm kem và hỏi: "Cho tôi vị dâu." Nhân viên trả lời: "Hết dâu rồi." Trong thế giới JavaScript, câu trả lời này được biểu diễn bằng `undefined` hoặc `null` — một giá trị ma quái mà Tony Hoare (người phát minh null) gọi là *"sai lầm tỷ đô"*.

Tại sao null nguy hiểm? Vì function `findIceCream("strawberry")` trả `IceCream | undefined`, nhưng compiler không **bắt buộc** bạn kiểm tra `undefined`. Bạn có thể vô tư viết `result.flavor` và chỉ phát hiện lỗi khi chương trình crash lúc 3 giờ sáng trên production.

`Option<T>` giải quyết vấn đề này bằng cách biến "có hoặc không" thành một ADT tường minh: `some(value)` = "đây, kem của bạn" hoặc `none` = "hết rồi". Và vì nó là ADT, compiler **bắt buộc** bạn xử lý cả hai trường hợp — bạn không thể "quên" check, vì switch/match phải exhaustive.

```typescript
// filename: src/null_problem.ts

// ❌ null/undefined = "Billion dollar mistake" (Tony Hoare)
const findUser = (id: string): { name: string } | undefined => {
    // ... database lookup
    return undefined;
};

const user = findUser("123");
// user.name;  // 💥 Runtime: Cannot read properties of undefined
// Phải nhớ check null EVERY TIME — dễ quên = bug!
```

### Option\<T\> — Explicit "có hoặc không"

```typescript
// filename: src/option.ts
import assert from "node:assert/strict";

// Option<T> = "có giá trị T" HOẶC "không có gì"
type Option<T> =
    | { readonly tag: "some"; readonly value: T }
    | { readonly tag: "none" };

// Smart constructors
const some = <T>(value: T): Option<T> => ({ tag: "some", value });
const none: Option<never> = { tag: "none" };
// none có type Option<never> — assignable cho Option<T> với mọi T

// Tại sao tốt hơn null/undefined?
// 1. COMPILER bắt buộc bạn xử lý BOTH cases
// 2. Không thể "quên" check — switch phải exhaustive
// 3. Chainable — dùng map/flatMap (sẽ thấy ở dưới)

// Ví dụ: tìm user
type User = { readonly id: string; readonly name: string; readonly age: number };

const users: readonly User[] = [
    { id: "1", name: "An", age: 25 },
    { id: "2", name: "Bình", age: 30 },
];

const findUser = (id: string): Option<User> => {
    const user = users.find(u => u.id === id);
    return user ? some(user) : none;
};

// Sử dụng — PHẢI xử lý cả hai cases!
const greet = (opt: Option<User>): string => {
    switch (opt.tag) {
        case "some": return `Hello ${opt.value.name}`;
        case "none": return "User not found";
    }
};

assert.strictEqual(greet(findUser("1")), "Hello An");
assert.strictEqual(greet(findUser("99")), "User not found");

console.log("Option OK ✅");
```

Nhìn hàm `greet`: nếu bạn xóa case `"none"`, TypeScript sẽ báo lỗi — function không cover hết variants. Đây là sự khác biệt cốt lõi so với `null`: compiler **ép** bạn suy nghĩ về trường hợp "không có gì", thay vì để bạn tự nhớ (và quên).

### Option utilities — map, flatMap, getOrElse

Nhưng `Option` thực sự tỏa sáng khi bạn cần **chain** nhiều thao tác có thể "không có gì". Hãy nghĩ: tìm user → lấy email → uppercase email. Mỗi bước đều có thể fail (user không tồn tại, email là null). Với `null`, bạn phải viết `if (user) { if (user.email) { ... } }` — nested horror. Với Option, bạn có `map` và `flatMap`.

`map` = "nếu có giá trị, transform nó; nếu none, giữ none". Giống `Array.map()`: `[1,2,3].map(double)` cho `[2,4,6]`, còn `[].map(double)` cho `[]` — operation trên container rỗng = container rỗng.

`flatMap` = "nếu có giá trị, gọi function trả về Option MỚI". Tránh `Option<Option<T>>` (giống `Array.flatMap` tránh `Array<Array<T>>`).

```typescript
// filename: src/option_utils.ts
import assert from "node:assert/strict";

type Option<T> =
    | { readonly tag: "some"; readonly value: T }
    | { readonly tag: "none" };

const some = <T>(value: T): Option<T> => ({ tag: "some", value });
const none: Option<never> = { tag: "none" };

// map — transform giá trị BÊN TRONG option (nếu có)
const mapOption = <T, U>(opt: Option<T>, f: (value: T) => U): Option<U> => {
    switch (opt.tag) {
        case "some": return some(f(opt.value));
        case "none": return none;
    }
};

// flatMap — transform trả về Option (tránh Option<Option<T>>)
const flatMapOption = <T, U>(opt: Option<T>, f: (value: T) => Option<U>): Option<U> => {
    switch (opt.tag) {
        case "some": return f(opt.value);
        case "none": return none;
    }
};

// getOrElse — lấy giá trị hoặc default
const getOrElse = <T>(opt: Option<T>, defaultValue: T): T => {
    switch (opt.tag) {
        case "some": return opt.value;
        case "none": return defaultValue;
    }
};

// Ví dụ: pipeline với Option
type User = { readonly id: string; readonly name: string; readonly email: string | null };

const users: readonly User[] = [
    { id: "1", name: "An", email: "an@mail.com" },
    { id: "2", name: "Bình", email: null },
];

const findUser = (id: string): Option<User> => {
    const user = users.find(u => u.id === id);
    return user ? some(user) : none;
};

const getEmail = (user: User): Option<string> =>
    user.email ? some(user.email) : none;

// Chain: findUser → getEmail → uppercase
const getUserEmail = (id: string): string =>
    getOrElse(
        mapOption(
            flatMapOption(findUser(id), getEmail),
            email => email.toUpperCase()
        ),
        "no-email"
    );

assert.strictEqual(getUserEmail("1"), "AN@MAIL.COM");
assert.strictEqual(getUserEmail("2"), "no-email");   // Bình has null email
assert.strictEqual(getUserEmail("99"), "no-email");  // user not found

console.log("Option utils OK ✅");
```

Hãy đọc `getUserEmail` từ trong ra ngoài: `findUser(id)` → nếu có user, `getEmail(user)` → nếu có email, `toUpperCase()` → nếu hết đường, trả `"no-email"`. Ba bước, ba cơ hội "không có gì", KHÔNG MỘT dòng `if (x != null)`. Option chain tự động "nhảy" qua `none` — giống dây chuyền Ch12 tự động dừng khi hết nguyên liệu.

> **💡 Option vs null**: Trong thực tế, TypeScript strict mode + `strictNullChecks` + `noUncheckedIndexedAccess` đã giúp nhiều. Option\<T\> giá trị nhất khi: (1) cần **chainable** operations, (2) domain model EXPLICIT "có/không", (3) dùng library FP như fp-ts.

---

## ✅ Checkpoint 13.2

> Đến đây bạn phải hiểu:
> 1. **Option\<T\>** = `some(value)` | `none` — explicit "có hoặc không"
> 2. **map** = transform bên trong (nếu some). **flatMap** = transform trả Option
> 3. **getOrElse** = lấy giá trị hoặc default
> 4. Option bắt buộc xử lý BOTH cases — compiler enforce
>
> **Test nhanh**: `mapOption(none, x => x + 1)` trả gì?
> <details><summary>Đáp án</summary>`none`! map trên none = none. Không gọi function f. Giống Array: `[].map(f)` = `[]`.</details>

---

## 13.3 — Result\<T, E\>: Errors as Values

### Khi tiệm kem cần giải thích TẠI SAO hết hàng

`Option` cho bạn biết "có hoặc không", nhưng khi "không" xảy ra, bạn không biết **tại sao**. Hết hàng? Vị đó không tồn tại? Cửa hàng đóng cửa? Với `none`, tất cả đều trông giống nhau — mất thông tin lỗi.

`Result<T, E>` mở rộng ý tưởng của Option bằng cách thay `none` vô danh thành `err(error)` — một variant mang theo **lý do thất bại**. Giống như nhân viên tiệm kem không chỉ lắc đầu, mà giải thích: "Hết chocolate từ sáng rồi anh ơi, ngày mai nhập lại."

Nhưng tại sao không dùng `try/catch`? Vì exceptions có một vấn đề nghiêm trọng: **chúng vô hình trong type system**. Function `JSON.parse(input)` trả `unknown` — type signature không hề gợi ý rằng nó sẽ throw. Caller có thể hoàn toàn quên try/catch, và compiler không nói gì — cho đến khi runtime crash.

```typescript
// filename: src/try_catch_problem.ts

// ❌ try/catch problem:
// 1. Compiler KHÔNG biết function có throw hay không
// 2. Caller có thể QUÊN try/catch — runtime crash
// 3. Catch nhận `unknown` — không biết error type
// 4. Side effect: throw làm gián đoạn control flow

const parseJSON = (input: string): unknown => {
    return JSON.parse(input);  // Throws nếu invalid — nhưng type signature KHÔNG nói!
};

// const data = parseJSON("invalid");  // 💥 Runtime crash! Signature nói trả unknown — lies!
```

### Result\<T, E\> — Errors trong type system

```typescript
// filename: src/result.ts
import assert from "node:assert/strict";

// Result<T, E> = "thành công với T" HOẶC "thất bại với E"
type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

// Smart constructors
const ok = <T, E = never>(value: T): Result<T, E> => ({ tag: "ok", value });
const err = <T = never, E = unknown>(error: E): Result<T, E> => ({ tag: "err", error });

// parseJSON — error TRONG type system!
const safeParseJSON = (input: string): Result<unknown, string> => {
    try {
        return ok(JSON.parse(input));
    } catch (e) {
        return err(`Invalid JSON: ${String(e)}`);
    }
};

// Caller PHẢI xử lý error — compiler enforce!
const result1 = safeParseJSON('{"name": "An"}');
const result2 = safeParseJSON("not json");

switch (result1.tag) {
    case "ok":
        assert.deepStrictEqual(result1.value, { name: "An" });
        break;
    case "err":
        throw new Error("Should not reach here");
}

switch (result2.tag) {
    case "ok":
        throw new Error("Should not reach here");
    case "err":
        assert.ok(result2.error.includes("Invalid JSON"));
        break;
}

console.log("Result OK ✅");
```

Bạn có nhận ra sự chuyển đổi không? `JSON.parse` — function nguy hiểm ẩn exception — bị **gói lại** bên trong `safeParseJSON`, và error trở thành value tường minh trong return type. Từ bên ngoài nhìn vào, `safeParseJSON` là một function thuần khiết: không throw, không side effects, kết quả nằm hoàn toàn trong type `Result<unknown, string>`.

### Result utilities — map, flatMap, bimap

Giống `Option`, `Result` cũng có `map` và `flatMap` để chain operations. Nhưng `Result` thêm một ý tưởng hay: `mapError` — transform error mà không đụng đến success. Hãy hình dung hai đường ray song song: `map` hoạt động trên đường ray "happy" (ok), `mapError` hoạt động trên đường ray "failure" (err), và `flatMap` cho phép "chuyển ray" từ happy sang failure khi có lỗi mới.

Đây chính là **Railway-Oriented Programming** — khái niệm mà Scott Wlaschin giới thiệu trong "F# for Fun and Profit". Happy path chạy thẳng, error tự động "nhảy" sang track lỗi và skip hết các bước còn lại.

```typescript
// filename: src/result_utils.ts
import assert from "node:assert/strict";

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T, E = never>(value: T): Result<T, E> => ({ tag: "ok", value });
const err = <T = never, E = unknown>(error: E): Result<T, E> => ({ tag: "err", error });

// map — transform success value (giữ nguyên error)
const mapResult = <T, U, E>(result: Result<T, E>, f: (value: T) => U): Result<U, E> => {
    switch (result.tag) {
        case "ok": return ok(f(result.value));
        case "err": return result;
    }
};

// mapError — transform error (giữ nguyên success)
const mapError = <T, E, F>(result: Result<T, E>, f: (error: E) => F): Result<T, F> => {
    switch (result.tag) {
        case "ok": return result;
        case "err": return err(f(result.error));
    }
};

// flatMap — chain Results (tránh Result<Result<T>>)
const flatMapResult = <T, U, E>(result: Result<T, E>, f: (value: T) => Result<U, E>): Result<U, E> => {
    switch (result.tag) {
        case "ok": return f(result.value);
        case "err": return result;
    }
};

// getOrThrow — cuối pipeline, convert Result → value (hoặc throw)
const getOrThrow = <T, E>(result: Result<T, E>): T => {
    switch (result.tag) {
        case "ok": return result.value;
        case "err": throw new Error(`Result error: ${String(result.error)}`);
    }
};

// --- Railway: chain nhiều validations ---
type User = {
    readonly name: string;
    readonly email: string;
    readonly age: number;
};

const validateName = (name: string): Result<string, string> =>
    name.length >= 2 ? ok(name) : err("Name must be >= 2 chars");

const validateEmail = (email: string): Result<string, string> =>
    email.includes("@") ? ok(email) : err("Invalid email");

const validateAge = (age: number): Result<number, string> =>
    age >= 0 && age <= 150 ? ok(age) : err("Age must be 0-150");

const createUser = (name: string, email: string, age: number): Result<User, string> => {
    const nameResult = validateName(name);
    if (nameResult.tag === "err") return nameResult;

    const emailResult = validateEmail(email);
    if (emailResult.tag === "err") return emailResult;

    const ageResult = validateAge(age);
    if (ageResult.tag === "err") return ageResult;

    return ok({ name: nameResult.value, email: emailResult.value, age: ageResult.value });
};

// Test
assert.deepStrictEqual(
    createUser("An", "an@mail.com", 25),
    { tag: "ok", value: { name: "An", email: "an@mail.com", age: 25 } }
);

assert.deepStrictEqual(
    createUser("", "an@mail.com", 25),
    { tag: "err", error: "Name must be >= 2 chars" }
);

assert.deepStrictEqual(
    createUser("An", "invalid", 25),
    { tag: "err", error: "Invalid email" }
);

console.log("Result utils OK ✅");
```

Nhìn `createUser`: mỗi validation là một "trạm kiểm tra" trên đường ray. Nếu tàu (data) vượt qua, nó tiếp tục chạy. Nếu fail bất kỳ trạm nào, nó "chuyển ray" sang error track và skip hết phần còn lại. First error wins — pipeline dừng ngay lập tức. So sánh với `try/catch` nơi bạn phải wrap toàn bộ block và catch `unknown`, Result cho bạn biết **chính xác** error type và **chính xác** chỗ nào fail.

> **💡 Result = "railway-oriented programming"**: Happy path (ok) chạy hết pipeline. Error (err) "nhảy" sang track lỗi — skip hết các bước tiếp theo. Giống tàu hỏa chuyển ray khi gặp lỗi.

---

## ✅ Checkpoint 13.3

> Đến đây bạn phải hiểu:
> 1. **Result\<T, E\>** = `ok(value)` | `err(error)` — error TRONG type system
> 2. **map** = transform ok (skip err). **flatMap** = chain Results
> 3. **Railway**: first error stops pipeline — skip remaining steps
> 4. **try/catch** = ẩn errors. **Result** = explicit, compiler enforce
>
> **Test nhanh**: `mapResult(err("oops"), x => x + 1)` trả gì?
> <details><summary>Đáp án</summary>`err("oops")`! map trên error = giữ nguyên error. Không gọi function f. Giống Option none.</details>

---

## 13.4 — Branded Types: Domain Safety

### Dán nhãn cho từng vị kem

Quay lại tiệm kem. Cả chocolate, vanilla, và matcha đều là "kem" — tức `string` trong code. Nhưng bạn KHÔNG muốn nhầm lẫn chúng: đổ sốt chocolate lên matcha sẽ là thảm họa. Trong TypeScript, cả `Email`, `Username`, và `Password` đều là `string` — và compiler vui vẻ cho bạn truyền `username` vào chỗ cần `email`. Không warning, không error.

**Branded types** giải quyết vấn đề này bằng cách "dán nhãn" lên type: `Email` vẫn là `string` bên trong, nhưng có một phantom brand `__brand: "Email"` khiến compiler coi nó KHÁC với `Username` (brand `"Username"`). Giống như dán sticker "Chocolate" và "Matcha" lên hai hộp kem — bên trong đều là kem, nhưng nhãn ngăn bạn nhầm.

Nhưng — và đây là điểm quan trọng — bạn không thể tự ý "dán nhãn". Nhãn chỉ được dán qua **smart constructor** — một function validate input trước khi brand. Giống như nhân viên QC kiểm tra kem trước khi dán nhãn xuất xưởng.

```typescript
// filename: src/branded_domain.ts
import assert from "node:assert/strict";

// --- Brand infrastructure ---
type Brand<T, B extends string> = T & { readonly __brand: B };

// --- Domain types ---
type Email = Brand<string, "Email">;
type Username = Brand<string, "Username">;
type Password = Brand<string, "Password">;
type UserId = Brand<string, "UserId">;

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// --- Smart constructors: VALIDATE trước khi brand ---
const Email = (input: string): Result<Email, string> => {
    const trimmed = input.trim().toLowerCase();
    if (!trimmed.includes("@")) return err("Email phải có @");
    if (trimmed.length > 254) return err("Email quá dài");
    return ok(trimmed as Email);
};

const Username = (input: string): Result<Username, string> => {
    if (input.length < 3) return err("Username >= 3 ký tự");
    if (input.length > 30) return err("Username <= 30 ký tự");
    if (!/^[a-zA-Z0-9_]+$/.test(input)) return err("Username chỉ cho phép a-z, 0-9, _");
    return ok(input as Username);
};

const Password = (input: string): Result<Password, string> => {
    if (input.length < 8) return err("Password >= 8 ký tự");
    if (!/[A-Z]/.test(input)) return err("Password cần ít nhất 1 chữ hoa");
    if (!/[0-9]/.test(input)) return err("Password cần ít nhất 1 số");
    return ok(input as Password);
};

// --- Type User dùng branded types ---
type User = {
    readonly id: UserId;
    readonly email: Email;
    readonly username: Username;
};

// ❌ Compiler ngăn nhầm lẫn!
// const sendEmail = (to: Email, subject: string) => { ... };
// sendEmail(user.username, "hello");  // ❌ Username ≠ Email!
// sendEmail(user.email, "hello");     // ✅

// Test
const emailResult = Email("An@Mail.Com");
assert.strictEqual(emailResult.tag, "ok");
if (emailResult.tag === "ok") {
    assert.strictEqual(emailResult.value, "an@mail.com"); // trimmed + lowercased
}

const badEmail = Email("invalid");
assert.strictEqual(badEmail.tag, "err");

const userResult = Username("an_nguyen");
assert.strictEqual(userResult.tag, "ok");

const badUser = Username("ab");  // too short
assert.strictEqual(badUser.tag, "err");

console.log("Branded domain OK ✅");
```

Chú ý smart constructor `Email`: nó không chỉ validate, mà còn **normalize** (trim + lowercase). Sau khi qua cổng này, bạn biết chắc `Email` luôn hợp lệ VÀ chuẩn hóa. Không cần kiểm tra lại bất kỳ đâu trong code — type system đảm bảo. Đây là nguyên tắc **"Parse, don't validate"**: thay vì validate rải rác khắp nơi, bạn parse MỘT LẦN ở biên giới hệ thống, rồi dùng branded type an toàn bên trong.

### Branded types cho numeric domains

Branded types không chỉ dành cho strings. Số cũng dễ nhầm: `PositiveNumber` vs bất kỳ `number`, `VND` vs `USD`, `Percentage` vs ratio. Compiler không phân biệt `applyDiscount(price, taxRate)` với `applyDiscount(taxRate, price)` — cả hai đều là `number`. Branded types sửa điều này.

```typescript
// filename: src/branded_numbers.ts
import assert from "node:assert/strict";

type Brand<T, B extends string> = T & { readonly __brand: B };

// Dương strictly > 0
type PositiveNumber = Brand<number, "PositiveNumber">;
const PositiveNumber = (n: number): PositiveNumber => {
    // Throw approach — dùng khi KHÔNG BAO GIỜ nên fail (internal invariant)
    if (n <= 0) throw new Error(`Expected positive, got ${n}`);
    return n as PositiveNumber;
};

// Percentage 0-100
type Percentage = Brand<number, "Percentage">;
const Percentage = (n: number): Percentage => {
    // Throw approach — dùng cho simple constraints
    // (Dùng Result nếu cần handle lỗi: xem Email/Username ở trên)
    if (n < 0 || n > 100) throw new Error(`Percentage must be 0-100, got ${n}`);
    return n as Percentage;
};

// Money (cents, tránh floating point)
type VND = Brand<number, "VND">;
type USD = Brand<number, "USD">;
const VND = (dong: number): VND => Math.round(dong) as VND;
const USD = (cents: number): USD => Math.round(cents) as USD;

// ❌ Compiler ngăn trộn currencies!
const applyDiscount = (price: VND, pct: Percentage): VND =>
    VND(price * (1 - pct / 100));

const price = VND(500000);
const discount = Percentage(20);
const final = applyDiscount(price, discount);

assert.strictEqual(final, 400000 as VND);

// applyDiscount(USD(1000), discount);  // ❌ USD ≠ VND!

console.log("Branded numbers OK ✅");
```

> **💡 Branded types = "Parse, don't validate"**: Khi bạn có `Email`, bạn BIẾT nó đã validated. Không cần check lại. Smart constructor = cổng duy nhất tạo branded value. Sau đó, type system đảm bảo safety.

---

## ✅ Checkpoint 13.4

> Đến đây bạn phải hiểu:
> 1. **Branded types** = `T & { __brand: B }` — compile-time tag
> 2. **Smart constructor** = validate + brand. Cổng duy nhất
> 3. **Ngăn nhầm**: Email ≠ Username, VND ≠ USD
> 4. **"Parse, don't validate"**: sau khi branded, không cần check lại
>
> **Test nhanh**: `const x: Email = "test@mail.com";` — compile OK?
> <details><summary>Đáp án</summary>**KHÔNG!** `string` không assignable cho `Email` (vì thiếu `__brand`). Phải qua smart constructor: `Email("test@mail.com")`.</details>

---

## 13.5 — Complex ADT Hierarchies

### Nhiều tiệm kem, nhiều tầng menu

Cho đến giờ, ADTs của chúng ta chỉ có MỘT tầng: `Shape = Circle | Rectangle | Triangle`. Nhưng domain thực tế phức tạp hơn: hệ thống thanh toán cần mô hình hóa **phương thức** (credit card HOẶC bank transfer HOẶC e-wallet) VÀ **kết quả** (success HOẶC declined HOẶC pending HOẶC error). Hai ADTs độc lập, mỗi cái với data fields riêng biệt.

Giống như tiệm kem có menu tầng 1 (vị: chocolate, vanilla, matcha) và menu tầng 2 (topping: sprinkles, syrup, cream). Bạn chọn MỘT vị VÀ MỘT topping — hai sum types kết hợp thành product. Multi-level ADT cho phép mô hình hóa domain phức tạp mà vẫn giữ exhaustive checking ở mỗi tầng.

```typescript
// filename: src/complex_adt.ts
import assert from "node:assert/strict";

// Payment system — 2 levels of DU

// Level 1: Payment method
type PaymentMethod =
    | { readonly tag: "credit_card"; readonly number: string; readonly cvv: string }
    | { readonly tag: "bank_transfer"; readonly bankCode: string; readonly accountNo: string }
    | { readonly tag: "e_wallet"; readonly provider: "momo" | "zalopay" | "vnpay"; readonly phone: string };

// Level 2: Payment result (generic over any method)
type PaymentResult =
    | { readonly tag: "success"; readonly transactionId: string; readonly amount: number }
    | { readonly tag: "declined"; readonly reason: string }
    | { readonly tag: "pending"; readonly estimatedTime: number }  // minutes
    | { readonly tag: "error"; readonly code: number; readonly message: string };

// Handle payment result
const describeResult = (result: PaymentResult): string => {
    switch (result.tag) {
        case "success":
            return `✅ Thành công! Mã GD: ${result.transactionId}`;
        case "declined":
            return `❌ Từ chối: ${result.reason}`;
        case "pending":
            return `⏳ Đang xử lý (~${result.estimatedTime} phút)`;
        case "error":
            return `💥 Lỗi ${result.code}: ${result.message}`;
    }
};

// Handle payment method
const describeMethod = (method: PaymentMethod): string => {
    switch (method.tag) {
        case "credit_card":
            return `💳 Credit card: ****${method.number.slice(-4)}`;
        case "bank_transfer":
            return `🏦 Bank: ${method.bankCode} - ${method.accountNo}`;
        case "e_wallet":
            return `📱 ${method.provider.toUpperCase()}: ${method.phone}`;
    }
};

// Test
const momoPayment: PaymentMethod = {
    tag: "e_wallet",
    provider: "momo",
    phone: "0901234567",
};

const success: PaymentResult = {
    tag: "success",
    transactionId: "TXN-001",
    amount: 500000,
};

assert.strictEqual(
    describeMethod(momoPayment),
    "📱 MOMO: 0901234567"
);
assert.strictEqual(
    describeResult(success),
    "✅ Thành công! Mã GD: TXN-001"
);

console.log("Complex ADT OK ✅");
```

### State machines bằng DU

Đợi đã — nếu ADTs có thể mô hình hóa "cái gì có thể xảy ra", liệu chúng có thể mô hình hóa "cái gì **KHÔNG được** xảy ra"?

Câu trả lời là có — thông qua **state machines**. Một đơn hàng bắt đầu ở trạng thái "draft", chuyển sang "submitted" khi khách gửi, sang "paid" khi thanh toán, sang "shipped" khi giao hàng. Nhưng bạn KHÔNG THỂ ship một đơn chưa thanh toán, và KHÔNG THỂ cancel một đơn đã giao. Bằng cách dùng DU cho states và **narrowed arguments** cho transitions, compiler sẽ từ chối mọi transition bất hợp lệ.

```typescript
// filename: src/state_machine.ts
import assert from "node:assert/strict";

// Order state machine — MỖI state có data KHÁC nhau
type OrderState =
    | { readonly tag: "draft"; readonly items: readonly string[] }
    | { readonly tag: "submitted"; readonly items: readonly string[]; readonly submittedAt: number }
    | { readonly tag: "paid"; readonly items: readonly string[]; readonly paidAt: number; readonly paymentId: string }
    | { readonly tag: "shipped"; readonly items: readonly string[]; readonly trackingCode: string }
    | { readonly tag: "delivered"; readonly deliveredAt: number }
    | { readonly tag: "cancelled"; readonly reason: string };

// Transitions — CHỈ cho phép transitions hợp lệ!
const submitOrder = (order: OrderState & { readonly tag: "draft" }): OrderState => ({
    tag: "submitted",
    items: order.items,
    submittedAt: Date.now(),
});

const payOrder = (
    order: OrderState & { readonly tag: "submitted" },
    paymentId: string
): OrderState => ({
    tag: "paid",
    items: order.items,
    paidAt: Date.now(),
    paymentId,
});

const cancelOrder = (
    order: OrderState & { readonly tag: "draft" | "submitted" },
    reason: string
): OrderState => ({
    tag: "cancelled",
    reason,
});

// ❌ cancelOrder({ tag: "shipped", ... }, "changed mind")
// Compile error! shipped không phải "draft" | "submitted"!

// Describe state
const describeOrder = (order: OrderState): string => {
    switch (order.tag) {
        case "draft":
            return `📝 Draft: ${order.items.length} items`;
        case "submitted":
            return `📤 Submitted at ${new Date(order.submittedAt).toISOString()}`;
        case "paid":
            return `💰 Paid (${order.paymentId})`;
        case "shipped":
            return `🚚 Shipped: ${order.trackingCode}`;
        case "delivered":
            return `✅ Delivered`;
        case "cancelled":
            return `❌ Cancelled: ${order.reason}`;
    }
};

const draft: OrderState = { tag: "draft", items: ["Laptop", "Mouse"] };
assert.strictEqual(describeOrder(draft), "📝 Draft: 2 items");

console.log("State machine OK ✅");
```

Nhìn kỹ tham số `cancelOrder`: `order: OrderState & { readonly tag: "draft" | "submitted" }`. Intersection type `&` thu hẹp `OrderState` xuống CHỈ variant "draft" hoặc "submitted". Nếu bạn truyền order có tag "shipped" hoặc "paid", compiler từ chối **ngay lúc compile** — bạn không cần chạy chương trình để phát hiện lỗi logic. business rules ĐÃ NẰM trong type system.

> **💡 State machines + DU = impossible invalid transitions**: Compiler ngăn bạn cancel order đã shipped! Mỗi transition function CHỈ chấp nhận states hợp lệ. Domain logic = type-safe.

---

## ✅ Checkpoint 13.5

> Đến đây bạn phải hiểu:
> 1. **Multi-level DU**: PaymentMethod × PaymentResult — independent ADTs
> 2. **State machines**: mỗi state có data riêng. Transitions type-checked
> 3. **Narrowing arguments**: `order: OrderState & { tag: "draft" }` = CHỈ chấp nhận draft
> 4. Compiler ngăn **invalid transitions** — không cần runtime checks
>
> **Test nhanh**: `cancelOrder({ tag: "paid", ... }, "sorry")` — lỗi gì?
> <details><summary>Đáp án</summary>**Compile error**! `cancelOrder` chỉ nhận `tag: "draft" | "submitted"`. `"paid"` không trong union → compiler từ chối.</details>

---

## 13.6 — Putting It All Together: Domain Modeling

### Lắp ráp toàn bộ: tiệm kem thành chuỗi franchise

Qua 5 section, bạn đã có đầy đủ công cụ: ADTs cho domain variants, Option cho "có hoặc không", Result cho errors tường minh, Branded types cho domain safety, State machines cho lifecycle. Giờ hãy kết hợp tất cả vào một ví dụ E-commerce thực tế — nơi mỗi công cụ đóng vai trò riêng trong hệ thống.

Hãy hình dung hệ thống này như một nhà máy hoàn chỉnh (callback từ Ch12): branded types là **nhãn QC** trên mỗi sản phẩm, ADTs là **menu đa dạng** sản phẩm, Result là **hệ thống kiểm soát lỗi**, và pure functions là **các trạm xử lý** trên dây chuyền.

```typescript
// filename: src/domain_modeling.ts
import assert from "node:assert/strict";

// --- Branded types ---
type Brand<T, B extends string> = T & { readonly __brand: B };
type ProductId = Brand<string, "ProductId">;
type OrderId = Brand<string, "OrderId">;
type Money = Brand<number, "Money">;  // đơn vị VND

const ProductId = (id: string): ProductId => id as ProductId;
const OrderId = (id: string): OrderId => id as OrderId;
const Money = (amount: number): Money => Math.round(amount) as Money;

// --- ADTs ---
type Product = {
    readonly id: ProductId;
    readonly name: string;
    readonly price: Money;
    readonly stock: number;
};

type OrderItem = {
    readonly productId: ProductId;
    readonly quantity: number;
    readonly unitPrice: Money;
};

type DiscountType =
    | { readonly tag: "percentage"; readonly percent: number }
    | { readonly tag: "fixed"; readonly amount: Money }
    | { readonly tag: "buy_x_get_y"; readonly buy: number; readonly free: number };

type OrderError =
    | { readonly tag: "out_of_stock"; readonly productId: ProductId }
    | { readonly tag: "invalid_quantity"; readonly message: string }
    | { readonly tag: "discount_expired" }
    | { readonly tag: "minimum_order"; readonly required: Money; readonly actual: Money };

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// --- Pure functions ---
const itemTotal = (item: OrderItem): Money =>
    Money(item.unitPrice * item.quantity);

const subtotal = (items: readonly OrderItem[]): Money =>
    Money(items.reduce((sum, item) => sum + itemTotal(item), 0));

const applyDiscount = (amount: Money, discount: DiscountType): Money => {
    switch (discount.tag) {
        case "percentage":
            return Money(amount * (1 - discount.percent / 100));
        case "fixed":
            return Money(Math.max(0, amount - discount.amount));
        case "buy_x_get_y":
            // Simplified: giảm giá theo tỉ lệ free/(buy+free)
            return Money(amount * (discount.buy / (discount.buy + discount.free)));
    }
};

const validateOrder = (
    items: readonly OrderItem[],
    products: readonly Product[],
    minimumOrder: Money
): Result<readonly OrderItem[], OrderError> => {
    // Check stock
    for (const item of items) {
        const product = products.find(p => p.id === item.productId);
        if (!product || product.stock < item.quantity) {
            return err({ tag: "out_of_stock", productId: item.productId });
        }
    }

    // Check quantities
    for (const item of items) {
        if (item.quantity <= 0) {
            return err({ tag: "invalid_quantity", message: `Quantity must be > 0` });
        }
    }

    // Check minimum
    const total = subtotal(items);
    if (total < minimumOrder) {
        return err({ tag: "minimum_order", required: minimumOrder, actual: total });
    }

    return ok(items);
};

// --- Test ---
const products: readonly Product[] = [
    { id: ProductId("P1"), name: "Laptop", price: Money(20000000), stock: 5 },
    { id: ProductId("P2"), name: "Mouse", price: Money(500000), stock: 10 },
];

const items: readonly OrderItem[] = [
    { productId: ProductId("P1"), quantity: 1, unitPrice: Money(20000000) },
    { productId: ProductId("P2"), quantity: 2, unitPrice: Money(500000) },
];

// Validate → OK
const validResult = validateOrder(items, products, Money(100000));
assert.strictEqual(validResult.tag, "ok");

// Subtotal
const sub = subtotal(items);
assert.strictEqual(sub, 21000000 as Money);  // 20M + 2×500K

// Apply 10% discount
const afterDiscount = applyDiscount(sub, { tag: "percentage", percent: 10 });
assert.strictEqual(afterDiscount, 18900000 as Money);  // 21M × 0.9

// Apply fixed discount
const afterFixed = applyDiscount(sub, { tag: "fixed", amount: Money(1000000) });
assert.strictEqual(afterFixed, 20000000 as Money);  // 21M - 1M

console.log("Domain modeling OK ✅");
```

Hãy đếm những gì type system bảo vệ trong ví dụ này: `ProductId` không thể nhầm với `OrderId` (branded types), `DiscountType` phải handle cả 3 variant (exhaustive switch), `OrderError` nói chính xác lỗi gì xảy ra (sum type), `validateOrder` trả `Result` buộc caller xử lý lỗi (no hidden exceptions), và `Money` đảm bảo không nhầm VND với con số thường. **Zero runtime checks** — tất cả đều compile-time.

---

## ✅ Checkpoint 13.6

> Đến đây bạn phải hiểu:
> 1. **Branded types** cho IDs, Money = compile-time safety
> 2. **ADTs** cho DiscountType, OrderError, OrderState = exhaustive handling
> 3. **Result** cho validation = errors in types, no exceptions
> 4. **Pure functions** cho business logic = testable, composable
>
> **Test nhanh**: `applyDiscount(Money(1000), { tag: "fixed", amount: Money(2000) })` = ?
> <details><summary>Đáp án</summary>`Money(0)` — `Math.max(0, 1000 - 2000)` = 0. Discount lớn hơn amount → floor ở 0, không âm!</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Option pipeline

```typescript
// Viết function:
// 1. parseNumber(input: string): Option<number> — parse string thành number (NaN → none)
// 2. safeDivide(a: number, b: number): Option<number> — chia a/b (b=0 → none)
// 3. Dùng flatMap: parseNumber("10") → safeDivide bởi parseNumber("2") → kết quả?
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

type Option<T> =
    | { readonly tag: "some"; readonly value: T }
    | { readonly tag: "none" };

const some = <T>(value: T): Option<T> => ({ tag: "some", value });
const none: Option<never> = { tag: "none" };

const flatMapOption = <T, U>(opt: Option<T>, f: (v: T) => Option<U>): Option<U> =>
    opt.tag === "some" ? f(opt.value) : none;

const getOrElse = <T>(opt: Option<T>, def: T): T =>
    opt.tag === "some" ? opt.value : def;

const parseNumber = (input: string): Option<number> => {
    if (input.trim() === "") return none;   // Number("") = 0, not NaN!
    const n = Number(input);
    return Number.isNaN(n) ? none : some(n);
};

const safeDivide = (a: number, b: number): Option<number> =>
    b === 0 ? none : some(a / b);

// Pipeline: "10" / "2" = 5
const result = flatMapOption(
    parseNumber("10"),
    a => flatMapOption(parseNumber("2"), b => safeDivide(a, b))
);

assert.deepStrictEqual(result, { tag: "some", value: 5 });

// "10" / "0" = none
const divByZero = flatMapOption(
    parseNumber("10"),
    a => flatMapOption(parseNumber("0"), b => safeDivide(a, b))
);

assert.deepStrictEqual(divByZero, { tag: "none" });

// "abc" / "2" = none
const badInput = flatMapOption(
    parseNumber("abc"),
    a => flatMapOption(parseNumber("2"), b => safeDivide(a, b))
);

assert.deepStrictEqual(badInput, { tag: "none" });
```

</details>

---

**Bài 2** (10 phút): Result chain

```typescript
// Viết validation pipeline cho form đăng ký:
// 1. validateUsername(s): Result<string, string> — 3-20 chars, alphanumeric
// 2. validatePassword(s): Result<string, string> — 8+ chars, 1 uppercase, 1 number
// 3. validateAge(n): Result<number, string> — 18-120
// 4. createRegistration(username, password, age): Result<Registration, string>
//    — chain cả 3 validations, first error stops
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

type Registration = {
    readonly username: string;
    readonly password: string;
    readonly age: number;
};

const validateUsername = (s: string): Result<string, string> => {
    if (s.length < 3 || s.length > 20) return err("Username: 3-20 chars");
    if (!/^[a-zA-Z0-9]+$/.test(s)) return err("Username: alphanumeric only");
    return ok(s);
};

const validatePassword = (s: string): Result<string, string> => {
    if (s.length < 8) return err("Password: >= 8 chars");
    if (!/[A-Z]/.test(s)) return err("Password: need 1 uppercase");
    if (!/[0-9]/.test(s)) return err("Password: need 1 number");
    return ok(s);
};

const validateAge = (n: number): Result<number, string> => {
    if (n < 18 || n > 120) return err("Age: 18-120");
    return ok(n);
};

const createRegistration = (
    username: string,
    password: string,
    age: number
): Result<Registration, string> => {
    const u = validateUsername(username);
    if (u.tag === "err") return u;

    const p = validatePassword(password);
    if (p.tag === "err") return p;

    const a = validateAge(age);
    if (a.tag === "err") return a;

    return ok({ username: u.value, password: p.value, age: a.value });
};

// Tests
assert.strictEqual(
    createRegistration("an", "Pass1234", 25).tag,
    "err"  // username too short
);

assert.strictEqual(
    createRegistration("annguyen", "weak", 25).tag,
    "err"  // password too short
);

assert.strictEqual(
    createRegistration("annguyen", "StrongPass1", 25).tag,
    "ok"
);
```

</details>

---

**Bài 3** (15 phút): State machine

```typescript
// Viết ticket state machine cho support system:
// States: "open" | "in_progress" | "resolved" | "closed"
// Transitions:
//   open → in_progress (assign to agent)
//   open → closed (auto-close)
//   in_progress → resolved (agent resolve)
//   in_progress → open (reopen)
//   resolved → closed (customer confirm)
//   resolved → open (customer reopen)
//
// Mỗi state có data khác nhau.
// Function transition CHỈ chấp nhận states hợp lệ (compile-time check).
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type TicketState =
    | { readonly tag: "open"; readonly title: string; readonly createdAt: number }
    | { readonly tag: "in_progress"; readonly title: string; readonly agentId: string; readonly startedAt: number }
    | { readonly tag: "resolved"; readonly title: string; readonly resolution: string; readonly resolvedAt: number }
    | { readonly tag: "closed"; readonly closedAt: number; readonly reason: "auto" | "confirmed" | "reopened_then_closed" };

// Valid transitions only!
const assignToAgent = (
    ticket: TicketState & { readonly tag: "open" },
    agentId: string
): TicketState => ({
    tag: "in_progress",
    title: ticket.title,
    agentId,
    startedAt: Date.now(),
});

const resolve = (
    ticket: TicketState & { readonly tag: "in_progress" },
    resolution: string
): TicketState => ({
    tag: "resolved",
    title: ticket.title,
    resolution,
    resolvedAt: Date.now(),
});

const reopen = (
    ticket: TicketState & { readonly tag: "in_progress" | "resolved" }
): TicketState => ({
    tag: "open",
    title: ticket.title,
    createdAt: Date.now(),
});

const close = (
    ticket: TicketState & { readonly tag: "open" | "resolved" },
    reason: "auto" | "confirmed"
): TicketState => ({
    tag: "closed",
    closedAt: Date.now(),
    reason,
});

// ❌ resolve({ tag: "open", ... })   — compile error!
// ❌ close({ tag: "in_progress", ... }) — compile error!

const describe = (ticket: TicketState): string => {
    switch (ticket.tag) {
        case "open": return `📋 Open: ${ticket.title}`;
        case "in_progress": return `🔧 In Progress (Agent: ${ticket.agentId})`;
        case "resolved": return `✅ Resolved: ${ticket.resolution}`;
        case "closed": return `🔒 Closed (${ticket.reason})`;
    }
};

const ticket: TicketState = { tag: "open", title: "Login bug", createdAt: Date.now() };
assert.strictEqual(describe(ticket), "📋 Open: Login bug");
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| `Option<never>` cho none | `never` = subtype của mọi T → `Option<never>` assignable cho `Option<T>` | Đây là design đúng — không cần sửa |
| Branded type assignable lẫn nhau | Quên `readonly` trên `__brand` | Luôn `{ readonly __brand: B }` |
| Result chain verbose | Nhiều `if (x.tag === "err") return x` | Dùng `flatMap` chain hoặc fp-ts `pipe` + `E.chain` |
| DU exhaustive check fail | Quên 1 variant trong switch | Thêm `default: assertNever(x)` hoặc bật `noImplicitReturns` |
| State machine cho phép invalid transition | Argument type quá rộng (`OrderState` thay vì `OrderState & {tag:"draft"}`) | Narrowing argument types — CHỈ chấp nhận valid source states |

---

## Tóm tắt

Chương này đã đưa bạn từ tiệm kem đơn giản (sum types) qua bộ Lego (product types) đến nhà máy sản xuất hoàn chỉnh (domain modeling). Mỗi công cụ giải quyết một vấn đề cụ thể:

- ✅ **ADTs** = Sum types (DU: `|`) + Product types (objects: `&`). Model domain chính xác.
- ✅ **Option\<T\>** = `some(value)` | `none`. Thay null/undefined. Chainable (map/flatMap).
- ✅ **Result\<T, E\>** = `ok(value)` | `err(error)`. Errors in types, not exceptions.
- ✅ **Branded types** + smart constructors = domain safety. Parse, don't validate.
- ✅ **State machines** = DU + narrowed transitions. Compiler ngăn invalid states.
- ✅ **Domain modeling** = ADTs + branded types + Result + pure functions = type-safe business logic.

Bạn giờ đã có khả năng mô hình hóa domain sao cho compiler bắt bugs thay bạn. Nhưng smart constructors ở Ch13 còn đơn giản — validate vài điều kiện rồi brand. Thực tế cần validation phức tạp hơn: schema validation, nested objects, arrays, async validation. Chương tiếp theo sẽ đưa bạn vào thế giới **Smart Constructors & Validation** chuyên sâu — nơi bạn gặp Zod, io-ts, và pattern "parse, don't validate" ở quy mô production.

## Tiếp theo

→ Chapter 14: **Smart Constructors & Validation** — Zod/io-ts, runtime validation, "parse, don't validate" pattern, form validation pipelines.
