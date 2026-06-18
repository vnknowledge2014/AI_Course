# Chapter 0 — TypeScript in 10 Minutes

> **Mục đích**: Giới thiệu đủ cú pháp TypeScript để bạn **đọc hiểu** code examples trong Part 0 (Chapters 1–3). Đây không phải hướng dẫn đầy đủ — Part I sẽ dạy lại mọi thứ chi tiết hơn.
>
> **Yêu cầu trước**: Không có. Biết JavaScript là lợi thế lớn, nhưng không bắt buộc.
> **Thời gian đọc**: ~10–15 phút | **Level**: Quick Primer
> **Kết quả cuối cùng**: Bạn có thể đọc hiểu mọi đoạn code trong Part 0 mà không bị "kẹt" ở syntax.

---

TypeScript = JavaScript + static types. Nghe đơn giản, nhưng chính sự kết hợp này tạo ra **ngôn ngữ FP phổ biến nhất thế giới**: arrow functions là lambda, discriminated unions thay algebraic data types, và ecosystem npm khổng lồ — hàng triệu packages sẵn sàng. Bật `strict: true`, bỏ `any` — bạn có một ngôn ngữ gần với Rust/Roc về độ an toàn mà không cần học ownership hay borrow checker.

## 0.1 — Cài đặt & Chạy thử

### Cài Node.js và TypeScript

TypeScript là JavaScript có thêm **static types** — compiler kiểm tra types trước khi chạy, giống thầy giáo kiểm bài trước khi nộp.

Mở terminal:

```bash
# Cài Node.js (nếu chưa có) — lên https://nodejs.org
node --version
# Output (ví dụ): v22.0.0

# Cài TypeScript
npm install -g typescript tsx

# Kiểm tra
tsc --version
# Output (ví dụ): Version 5.7.3
```

`tsc` là compiler — biến TypeScript thành JavaScript. `tsx` là runner — chạy TypeScript trực tiếp, không cần biên dịch tay.

### Tạo file đầu tiên

Tạo file `hello.ts`:

```typescript
// filename: hello.ts
console.log("Xin chào TypeScript! 🎉");
```

Chạy:

```bash
npx tsx hello.ts
# Output: Xin chào TypeScript! 🎉
```

### Bật strict mode

Tạo file `tsconfig.json` trong thư mục project:

```json
{
  "compilerOptions": {
    "strict": true,
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "bundler"
  }
}
```

`"strict": true` là quy tắc số 1 của cuốn sách này. Nó bật **tất cả** kiểm tra an toàn — giống đeo dây an toàn khi lái xe. Mọi code trong sách đều chạy với strict mode.

> **💡 Tại sao strict?** Không strict = TypeScript cho phép hàng tá lỗi "lặng lẽ" chạy qua. Strict mode buộc bạn viết rõ ràng — lúc đầu có vẻ phiền, nhưng khi deploy lên production thì bạn sẽ cảm ơn nó.

---

## 0.2 — Biến: `const` và `let`

```typescript
// filename: basics.ts

// const = không thay đổi được (immutable binding)
const name = "TypeScript";
const age = 28;
const pi = 3.14;
const active = true;

// TypeScript tự suy ra kiểu (type inference)
// name: string, age: number, pi: number, active: boolean

console.log(`${name} đã ${age} tuổi`);
// Output: TypeScript đã 28 tuổi

// let = có thể gán lại
let score = 0;
score = 95;  // ✅ OK

// const thì không:
// name = "JS";  // ❌ Error: Cannot assign to 'name' because it is a constant
```

Quy tắc đơn giản: **mặc định dùng `const`**. Chỉ dùng `let` khi thật sự cần gán lại. Không bao giờ dùng `var` — nó là di sản cũ, nhiều bẫy.

> **💡 Tại sao `const` mặc định?** Giống Rust (`let` immutable mặc định) và Roc (mọi thứ immutable). Khi biến không đổi, bạn không cần lo "ai đó sửa giá trị ở dòng 200". Tư duy Functional Programming bắt đầu từ đây.

### Kiểu dữ liệu cơ bản

| Kiểu | Ý nghĩa | Ví dụ |
|------|---------|-------|
| `number` | Mọi số (nguyên + thực) | `42`, `3.14`, `-7` |
| `string` | Chuỗi Unicode | `"xin chào"`, `` `template` `` |
| `boolean` | Đúng/Sai | `true`, `false` |
| `null` | Không có giá trị (chủ ý) | `null` |
| `undefined` | Chưa gán giá trị | `undefined` |

TypeScript gộp số nguyên và số thực vào chung `number` — khác Rust (có `i32`, `f64`, `u8`...) và Roc (`I64`, `F64`). Đơn giản hơn, nhưng ít chính xác hơn. Khi cần phân biệt (ví dụ: `UserId` phải là số nguyên dương), Chapter 13 sẽ dạy **branded types**.

---

## 0.3 — Functions: Arrow functions

Hãy tưởng tượng một chiếc máy xay: bỏ trái cây vào → ra sinh tố. Đó là function — Chapter 1 sẽ gọi nó là **lambda**. Trong TypeScript, chiếc máy đó viết bằng **arrow function** — dấu `=>` như mũi tên chỉ từ đầu vào sang đầu ra:

```typescript
// filename: functions.ts

// Arrow function: (tham_số: kiểu) => kết_quả
const addOne = (x: number): number => x + 1;

const add = (a: number, b: number): number => a + b;

// Gọi function
console.log(`addOne(5) = ${addOne(5)}`);   // → 6
console.log(`add(3, 5) = ${add(3, 5)}`);   // → 8

// Function không trả giá trị: void
const greet = (name: string): void => {
    console.log(`Hello, ${name}!`);
};

greet("TypeScript");
// Output: Hello, TypeScript!
```

So sánh nhanh:

| Toán (Lambda) | TypeScript | Rust | Roc |
|---|---|---|---|
| `λx. x + 1` | `(x: number) => x + 1` | `\|x\| x + 1` | `\x -> x + 1` |
| `λx. λy. x + y` | `(x: number, y: number) => x + y` | `\|x, y\| x + y` | `\x, y -> x + y` |

Bạn sẽ gặp lại bảng này ở Chapter 1 khi học Lambda Calculus.

> **💡 Tại sao arrow functions?** Vì `=>` không tạo `this` riêng — tránh được hàng tá bugs liên quan đến `this`. Trong FP, ta không dùng `this`. Arrow function = lựa chọn tự nhiên.

---

## 0.4 — Type aliases: Đặt tên cho kiểu

`type` cho phép bạn đặt tên cho bất kỳ kiểu nào — giống đặt biệt danh:

```typescript
// filename: types.ts

// Đặt tên cho kiểu cơ bản
type Age = number;
type Name = string;

// Đặt tên cho object type (giống struct trong Rust, record trong Roc)
type User = {
    readonly name: Name;
    readonly age: Age;
    readonly isActive: boolean;
};

const user: User = { name: "An", age: 25, isActive: true };
console.log(`${user.name}, ${user.age} tuổi`);
// Output: An, 25 tuổi

// readonly = không sửa được
// user.name = "Bình";  // ❌ Error: Cannot assign to 'name' because it is a read-only property
```

`readonly` = bất biến (immutable) cho từng field. Giống `let` trong Rust. Cuốn sách này dùng `readonly` ở mọi nơi có thể.

> **💡 `type` vs `interface`**: Cả hai đều tạo object types. `type` linh hoạt hơn — dùng được cho unions, intersections, và mọi kiểu phức tạp. `interface` chỉ dùng cho objects. Sách này dùng `type` là chính.

---

## 0.5 — Discriminated Unions: Một trong nhiều lựa chọn

Bạn vào quán và thanh toán bằng **tiền mặt** HOẶC **thẻ** HOẶC **ví MoMo**. Không bao giờ trả cùng lúc cả 3. Đây là **sum type** — chọn MỘT trong nhiều lựa chọn. TypeScript biểu diễn nó bằng **discriminated unions**, tương đương `enum` trong Rust và tag union trong Roc.

```typescript
// filename: unions.ts

// Đèn giao thông: chỉ có thể là Đỏ, Vàng, hoặc Xanh
type TrafficLight =
    | { readonly tag: "red" }
    | { readonly tag: "yellow" }
    | { readonly tag: "green" };

const light: TrafficLight = { tag: "red" };
console.log(`Light: ${light.tag}`);
// Output: Light: red
```

"Discriminated" vì có field `tag` giúp phân biệt. Field này có thể tên gì cũng được (`kind`, `type`, `_tag`...) — sách này dùng `tag` thống nhất.

Union variant có thể **chứa data** bên trong:

```typescript
// filename: shapes.ts

type Shape =
    | { readonly tag: "circle"; readonly radius: number }
    | { readonly tag: "rectangle"; readonly width: number; readonly height: number };

const circle: Shape = { tag: "circle", radius: 5.0 };
const rect: Shape = { tag: "rectangle", width: 3.0, height: 4.0 };

console.log(circle);
// Output: { tag: 'circle', radius: 5 }
console.log(rect);
// Output: { tag: 'rectangle', width: 3, height: 4 }
```

---

## 0.6 — `switch` + Exhaustive check

`switch` trên discriminated unions giống `match` trong Rust, `when...is` trong Roc. TypeScript **có thể bắt buộc** bạn xử lý mọi trường hợp — nếu bạn viết đúng cách:

```typescript
// filename: switch.ts

type Season =
    | { readonly tag: "spring" }
    | { readonly tag: "summer" }
    | { readonly tag: "fall" }
    | { readonly tag: "winter" };

// Helper: compiler sẽ báo lỗi nếu bạn quên 1 case
const assertNever = (x: never): never => {
    throw new Error(`Unexpected value: ${JSON.stringify(x)}`);
};

const describe = (season: Season): string => {
    switch (season.tag) {
        case "spring": return "Hoa nở 🌸";
        case "summer": return "Nắng nóng ☀️";
        case "fall":   return "Lá rụng 🍂";
        case "winter": return "Lạnh giá ❄️";
        default: return assertNever(season);
    }
    // Nếu bạn thêm variant mới vào Season mà quên thêm case
    // → compiler báo lỗi ở dòng assertNever!
};

const now: Season = { tag: "summer" };
console.log(`${now.tag}: ${describe(now)}`);
// Output: summer: Nắng nóng ☀️
```

`assertNever` là trick nhỏ nhưng cực kỳ mạnh: nếu `switch` chưa cover hết variants, TypeScript sẽ **từ chối biên dịch**. Giống `match` exhaustive check trong Rust.

> **💡 Ghi nhớ**: Luôn thêm `default: return assertNever(...)` vào switch trên discriminated unions. Đây là "bảo hiểm" quan trọng nhất.

---

## 0.7 — Destructuring + Pattern matching nhẹ

TypeScript cho phép "mở hộp" data ra biến — gọi là destructuring:

```typescript
// filename: destructure.ts

// Object destructuring
type Point = { readonly x: number; readonly y: number };
const origin: Point = { x: 0, y: 0 };
const { x, y } = origin;
console.log(`x=${x}, y=${y}`);
// Output: x=0, y=0

// Array destructuring
const [first, second, ...rest] = [10, 20, 30, 40, 50];
console.log(`first=${first}, second=${second}, rest=${JSON.stringify(rest)}`);
// Output: first=10, second=20, rest=[30,40,50]

// Dùng trong function parameters
const distance = ({ x, y }: Point): number => Math.sqrt(x * x + y * y);
console.log(`Distance from origin: ${distance({ x: 3, y: 4 })}`);
// Output: Distance from origin: 5
```

Kết hợp với discriminated unions — đây là "pattern matching" của TypeScript:

```typescript
// filename: match_shapes.ts

type Shape =
    | { readonly tag: "circle"; readonly radius: number }
    | { readonly tag: "rectangle"; readonly width: number; readonly height: number };

const assertNever = (x: never): never => {
    throw new Error(`Unexpected: ${JSON.stringify(x)}`);
};

const area = (shape: Shape): number => {
    switch (shape.tag) {
        case "circle":
            // TypeScript tự biết shape.radius tồn tại ở đây!
            return Math.PI * shape.radius * shape.radius;
        case "rectangle":
            // Và shape.width + shape.height tồn tại ở đây!
            return shape.width * shape.height;
        default:
            return assertNever(shape);
    }
};

console.log(`Circle area: ${area({ tag: "circle", radius: 5 }).toFixed(2)}`);
console.log(`Rect area: ${area({ tag: "rectangle", width: 3, height: 4 }).toFixed(2)}`);
// Output:
// Circle area: 78.54
// Rect area: 12.00
```

Khi bạn kiểm tra `shape.tag === "circle"`, TypeScript tự **thu hẹp kiểu** (type narrowing) — nó biết rằng bên trong nhánh đó, `shape` có field `radius`. Không cần cast, không cần `as`. Compiler tự hiểu.

---

## 0.8 — Arrays, `.map()`, `.filter()`, `.reduce()`

Arrays trong TypeScript có sẵn các methods FP quan trọng. Không cần import gì:

```typescript
// filename: arrays.ts

const numbers: readonly number[] = [1, 2, 3, 4, 5];

// .map() — biến đổi mỗi phần tử (giống List.map trong Roc)
const doubled = numbers.map(x => x * 2);
console.log(`Doubled: ${JSON.stringify(doubled)}`);
// Output: Doubled: [2,4,6,8,10]

// .filter() — lọc (giống List.keepIf trong Roc)
const evens = numbers.filter(x => x % 2 === 0);
console.log(`Evens: ${JSON.stringify(evens)}`);
// Output: Evens: [2,4]

// .reduce() — gom lại (giống List.walk trong Roc, .fold() trong Rust)
const total = numbers.reduce((sum, x) => sum + x, 0);
console.log(`Total: ${total}`);
// Output: Total: 15

// Kết hợp — pipeline bằng chaining
const result = numbers
    .map(x => x * x)          // bình phương
    .filter(x => x > 10)      // giữ > 10
    .reduce((s, x) => s + x, 0); // tổng
console.log(`Pipeline: ${result}`);
// Output: Pipeline: 371
// (16 + 25 + 36 + 49 + 64 + 81 + 100 = 371)
```

Ba methods này — `.map()`, `.filter()`, `.reduce()` — thay thế phần lớn vòng lặp `for` trong FP. Bạn sẽ dùng chúng xuyên suốt cuốn sách.

> **💡 `readonly number[]`**: Khai báo mảng bất biến — không cho phép `.push()`, `.pop()`, hay gán `[i] = value`. Giống cách FP nghĩ: tạo mảng mới thay vì sửa mảng cũ.

---

## 0.9 — Output và Assertions

```typescript
// filename: output.ts

const name = "TypeScript";
const version = 5.7;

// Template literal — chèn giá trị bằng ${}
console.log(`Hello, ${name}!`);
console.log(`${name} version ${version}`);

// In object/array
const nums = [1, 2, 3];
console.log("nums =", nums);
console.log("nums =", JSON.stringify(nums));

// Số thực — toFixed(n) cho n chữ số thập phân
console.log(`Pi ≈ ${Math.PI.toFixed(2)}`);

// Output:
// Hello, TypeScript!
// TypeScript version 5.7
// nums = [1, 2, 3]
// nums = [1,2,3]
// Pi ≈ 3.14
```

### Tự kiểm tra kết quả

`console.assert` in warning khi điều kiện sai, nhưng **không dừng chương trình**. Để kiểm tra nghiêm ngặt hơn, sách này dùng Node.js `assert` module — nó **throw ngay** khi sai, giống `assert_eq!` trong Rust:

```typescript
// filename: assert.ts
import assert from "node:assert/strict";

const result = 2 + 3;
assert.strictEqual(result, 5);  // ✅ Qua — không in gì

// assert.strictEqual(result, 99);
// ❌ AssertionError: Expected values to be strictly equal.
// 5 !== 99

console.log("All assertions passed!");
// Output: All assertions passed!
```

> **💡 Tại sao `node:assert/strict`?** Vì `console.assert` chỉ in warning rồi chạy tiếp — bugs vẫn lọt qua. `assert.strictEqual` **dừng ngay** khi sai — giống Rust `assert_eq!` và Roc `expect`. Trong production, dùng Vitest.

---

## 0.10 — Generics (đọc hiểu)

Generics cho phép function hoạt động với **nhiều kiểu** mà vẫn an toàn. Chưa cần hiểu sâu — chỉ cần **nhận ra** khi thấy:

```typescript
// filename: generics.ts

// <T> = "kiểu gì cũng được, tôi gọi nó là T"
const first = <T>(items: readonly T[]): T | undefined => items[0];

console.log(first([10, 20, 30]));     // 10 — T = number
console.log(first(["a", "b", "c"]));  // "a" — T = string
console.log(first([]));               // undefined
```

`<T>` đọc là "cho bất kỳ kiểu T nào". Function `first` hoạt động với mảng số, mảng chuỗi, mảng gì cũng được — TypeScript tự suy ra `T`.

---

## 0.11 — `null`, `undefined` và cách xử lý an toàn

Trong strict mode, TypeScript bắt lỗi `null`/`undefined` — bạn phải xử lý chúng rõ ràng:

```typescript
// filename: null_safety.ts

// Function có thể trả undefined
const findEven = (numbers: readonly number[]): number | undefined => {
    return numbers.find(n => n % 2 === 0);
};

const result = findEven([1, 3, 4, 7]);

// ❌ Không thể dùng result trực tiếp — nó có thể undefined!
// console.log(result * 2);  // Error: Object is possibly 'undefined'

// ✅ Phải kiểm tra trước
if (result !== undefined) {
    console.log(`Found even: ${result}`);    // TypeScript biết result là number ở đây
} else {
    console.log("No even number");
}
// Output: Found even: 4

// Optional chaining — truy cập an toàn
const users = [{ name: "An" }, { name: "Bình" }];
console.log(users[0]?.name);   // "An"
console.log(users[99]?.name);  // undefined (không crash!)
```

`T | undefined` tương đương `Option<T>` trong Rust — "có giá trị hoặc không". TypeScript không có wrapper type riêng mà dùng union trực tiếp. Chapter 22 sẽ giới thiệu `Result<T, E>` cho error handling.

---

## 0.12 — Cấm `any` — Quy tắc số 2

`any` là "tắt TypeScript đi" — nó vô hiệu hóa mọi kiểm tra type:

```typescript
// ❌ TUYỆT ĐỐI KHÔNG làm thế này
// const data: any = "hello";
// data.foo.bar.baz();  // Không lỗi compile... nhưng CRASH lúc chạy!

// ✅ Dùng unknown nếu chưa biết kiểu
const data: unknown = "hello";
// data.foo();  // ❌ Error: Object is of type 'unknown'

// Phải kiểm tra kiểu trước khi dùng
if (typeof data === "string") {
    console.log(data.toUpperCase());  // ✅ OK — TypeScript biết data là string
}
// Output: HELLO
```

| | `any` | `unknown` |
|---|---|---|
| Cho phép gọi method? | ✅ (nguy hiểm!) | ❌ (phải kiểm tra trước) |
| Compiler bảo vệ? | ❌ Không | ✅ Có |
| Dùng khi nào? | **Không bao giờ** * | Khi nhận data từ bên ngoài |

\* Ngoại trừ khi book demo "đây là cách SAI" — lúc đó sẽ ghi rõ `// ❌`

> **💡 Quy tắc**: `strict: true` + không `any` = hai trụ cột an toàn. Mọi code trong sách tuân thủ cả hai.

---

## 🔧 Tham chiếu nhanh

| Cú pháp | Ý nghĩa | Ví dụ |
|---------|---------|-------|
| `const x = 5` | Biến immutable | `const name = "TS"` |
| `let x = 5` | Biến có thể gán lại | `x = 10` được |
| `(x: number) => x + 1` | Arrow function | Closure / lambda |
| `type X = { a: number }` | Type alias (product) | Giống struct/record |
| `type X = \| A \| B` | Discriminated union (sum) | Giống enum/tags |
| `switch (x.tag) { ... }` | Pattern matching | Phải cover hết variants |
| `T \| undefined` | Có thể không có giá trị | Giống Option |
| `readonly` | Bất biến | `readonly name: string` |
| `readonly T[]` | Mảng bất biến | Không `.push()`, `.pop()` |
| `.map()`, `.filter()`, `.reduce()` | Array FP methods | Thay thế `for` |
| `` `${x}` `` | Template literal | Chèn giá trị vào chuỗi |
| `console.log(x)` | In ra console | |
| `assert.strictEqual(a, b)` | Kiểm tra bằng nhau | Throw nếu khác |
| `<T>` | Generic type parameter | Function cho nhiều kiểu |
| `unknown` | Chưa biết kiểu (an toàn) | Phải narrow trước khi dùng |

---

## Tóm tắt

Chapter này chỉ cung cấp **đủ syntax** để bạn đọc hiểu code trong Part 0. Bạn **không cần nhớ hết** — hãy dùng bảng tham chiếu ở trên khi cần.

Những thứ **chưa đề cập** (sẽ học kỹ ở Part I–II):
- **Module system** (Chapter 10) — `import`/`export`, project structure
- **Advanced Type System** (Chapter 9) — conditional, mapped, template literal types
- **Smart Constructors & Validation** (Chapter 14) — Zod, branded types
- **Error handling patterns** (Chapter 22) — Result type, Railway-Oriented Programming
- **`fp-ts` và `Effect`** (Chapter 25+) — thư viện FP chuyên dụng

Bạn sẽ gặp lại mọi concept ở đây nhưng ở mức **sâu hơn rất nhiều** trong Part I.

## Tiếp theo

→ Chapter 1: **Math Foundations for FP** — bạn sẽ học Lambda Calculus (arrow function chính là lambda!), Curry-Howard (types = logic), và cách dùng phép cộng/nhân để đếm trạng thái hệ thống — kỹ năng diệt bugs mạnh nhất mà 90% lập trình viên không biết.
