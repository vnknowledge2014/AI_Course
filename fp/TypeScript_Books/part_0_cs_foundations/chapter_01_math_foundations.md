# Chapter 1 — Math Foundations for FP

> **Bạn sẽ học được**:
> - Lambda Calculus là gì — và tại sao nó đơn giản hơn bạn tưởng
> - Curry-Howard Correspondence — compiler chính là "thầy giáo" kiểm tra bài cho bạn
> - Dùng phép cộng và phép nhân để đếm số trạng thái chương trình (**Algebraic Type Sizes**)
> - Tại sao "make illegal states unrepresentable" giúp code hết bug
>
> **Yêu cầu trước**: Chapter 0 (TypeScript in 10 Minutes). Về toán, chỉ cần biết cộng và nhân là đủ.
> **Thời gian đọc**: ~45 phút | **Level**: CS Foundations
> **Kết quả cuối cùng**: Nhìn vào bất kỳ data model nào và tính được "có bao nhiêu trạng thái sai có thể xảy ra" — rồi sửa thiết kế để trạng thái sai = 0.

---

## Trước khi bắt đầu: Bạn không cần "giỏi toán"

Nếu bạn đọc tên chapter — "Math Foundations" — và thấy hơi lo, hoàn toàn bình thường. Nhưng chapter này **không** yêu cầu bạn giải phương trình hay nhớ công thức phức tạp.

Toàn bộ "toán" ở đây chỉ cần 3 thứ:
- Biết **cộng**: 2 + 3 = 5
- Biết **nhân**: 2 × 3 = 6
- Hiểu câu: "Nếu trời mưa **thì** mang ô"

Nếu bạn hiểu được 3 thứ trên — kể cả bạn mới học lớp 3 — bạn đã đủ nền tảng cho toàn bộ chapter này rồi. Mình sẽ đi từng bước nhỏ, với rất nhiều ví dụ từ đời thường.

---

## 1.1 — Lambda Calculus: Chiếc máy xay chỉ có một nút

### Câu chuyện

Hãy tưởng tượng bạn có một chiếc **máy xay sinh tố** thần kỳ. Bạn bỏ **trái cây** vào → máy xay → ra **sinh tố**. Máy này luôn hoạt động giống nhau: cùng loại trái cây vào, luôn ra cùng loại sinh tố. Không bao giờ bỏ cam vào mà ra nước dừa.

Đó chính là **function** — khái niệm quan trọng nhất trong lập trình.

Năm 1936, nhà toán học Alonzo Church đặt câu hỏi: *"Nếu ta chỉ dùng máy xay (functions) — không cần tủ lạnh (biến), không cần lò nướng (vòng lặp), không cần gì khác — liệu ta có thể nấu được MỌI món ăn (mọi phép tính) không?"*

Câu trả lời: **Có.** Và hệ thống đó gọi là **Lambda Calculus**.

### Lambda = "Cái máy xay" không tên

Trong toán, Lambda Calculus viết function thế này:

```
λx. x + 1
```

Đọc từng phần:
- `λ` (lambda) = "đây là một cái máy xay"
- `x` = "bỏ cái gì đó vào, gọi nó là x"
- `.` = "rồi làm phép tính sau đây"
- `x + 1` = "trả ra x cộng 1"

Ghép lại: **"Đây là cái máy, bỏ x vào, ra x + 1".**

Trong TypeScript, bạn viết gần như y hệt — chỉ đổi `λ` thành `=>`:

```typescript
// filename: src/lambda.ts
import assert from "node:assert/strict";

// Tạo "cái máy xay" — nhận x, trả ra x + 1
const addOne = (x: number): number => x + 1;

// Bỏ số 5 vào máy
const result = addOne(5);

console.log(`addOne(5) = ${result}`);
// Output: addOne(5) = 6

assert.strictEqual(result, 6);
```

Dấu `=>` trong TypeScript thay cho `λ` — vì bàn phím không có phím lambda! Đây gọi là **arrow function** — tên gọi chính thức của "chiếc máy xay không tên" trong TypeScript. Ở Chapter 0 bạn đã gặp nó rồi.

| Toán (Lambda Calculus) | TypeScript (Arrow Function) | Nghĩa bằng lời |
|---|---|---|
| `λx. x + 1` | `(x: number) => x + 1` | Máy cộng 1 |
| `λx. x * 2` | `(x: number) => x * 2` | Máy nhân đôi |
| `λx. λy. x + y` | `(x: number, y: number) => x + y` | Máy cộng hai số |

### β-reduction = "Bỏ trái cây vào máy xay"

Khi bạn **bỏ một giá trị cụ thể vào function**, toán học gọi đó là **β-reduction** (beta reduction). Nghe phức tạp, nhưng thực ra cực kỳ đơn giản — chỉ là **thay số vào rồi tính**:

```
Máy:      λx. x + 1
Bỏ 5 vào: (λx. x + 1) 5
Thay x=5:  5 + 1
Kết quả:   6
```

Giống như: Máy xay cam → bỏ cam vào → ra nước cam. Chỉ vậy thôi.

```typescript
// filename: src/beta.ts
import assert from "node:assert/strict";

const addOne = (x: number): number => x + 1;

// Mỗi dòng đều là β-reduction — thay x vào rồi tính:
assert.strictEqual(addOne(5),  6);    // (λx. x + 1) 5  →  5 + 1  →  6
assert.strictEqual(addOne(0),  1);    // (λx. x + 1) 0  →  0 + 1  →  1
assert.strictEqual(addOne(-3), -2);   // (λx. x + 1) -3 → -3 + 1  → -2

console.log("All β-reductions passed! ✅");
// Output: All β-reductions passed! ✅
```

> **💡 Tại sao cần biết tên?** Bạn không cần nhớ "β-reduction" để viết code. Nhưng khi đọc sách FP hay nghe đồng nghiệp nói, bạn sẽ biết: "À, họ chỉ nói chuyện bỏ trái cây vào máy xay thôi mà."

### Higher-Order Functions: Máy lớn chứa máy nhỏ

Đây là lúc mọi thứ trở nên thú vị: **function có thể nhận function khác làm tham số**, hoặc **trả về function mới**. Giống một cái máy xay lớn mà bạn có thể lắp các lưỡi dao khác nhau vào.

```typescript
// filename: src/higher_order.ts
import assert from "node:assert/strict";

// apply nhận: 1 function + 1 số → chạy function đó với số đó
const apply = (f: (x: number) => number, value: number): number => f(value);

const addOne = (x: number): number => x + 1;
const double = (x: number): number => x * 2;

// "Máy lớn" apply lắp "lưỡi dao" addOne
assert.strictEqual(apply(addOne, 5), 6);      // addOne(5) = 6

// Đổi lưỡi dao sang double
assert.strictEqual(apply(double, 5), 10);     // double(5) = 10

console.log("Higher-order functions work! ✅");
// Output: Higher-order functions work! ✅
```

```typescript
// filename: src/compose.ts
import assert from "node:assert/strict";

// compose: "Nối ống" hai máy lại — output máy 1 = input máy 2
const compose = <A, B, C>(
    f: (b: B) => C,
    g: (a: A) => B
): ((a: A) => C) => {
    return (a: A): C => f(g(a));
};

const addOne = (x: number): number => x + 1;
const double = (x: number): number => x * 2;

// compose đọc PHẢI sang TRÁI: compose(f, g) = "chạy g trước, rồi f"
// double(5) = 10 → addOne(10) = 11
const doubleThenAdd = compose(addOne, double);
assert.strictEqual(doubleThenAdd(5), 11);

// addOne(5) = 6 → double(6) = 12
const addThenDouble = compose(double, addOne);
assert.strictEqual(addThenDouble(5), 12);

// Thứ tự quan trọng! Giống dây chuyền sản xuất — đổi thứ tự máy = đổi kết quả
console.log(`doubleThenAdd(5) = ${doubleThenAdd(5)}`);
console.log(`addThenDouble(5) = ${addThenDouble(5)}`);
// Output:
// doubleThenAdd(5) = 11
// addThenDouble(5) = 12
```

> **💡 compose = nền tảng của FP**: Viết function nhỏ, nối lại thành hệ thống lớn. Giống lắp LEGO — mỗi viên đơn giản, ghép lại thành lâu đài. Chapter 12 sẽ dạy bạn dùng `pipe()` và `flow()` — phiên bản "dễ đọc" của compose.

### Church Encoding: Ảo thuật thuần túy

Đây là phần "bonus" — không cần nhớ, nhưng cực kỳ đẹp về mặt toán học.

Alonzo Church chứng minh rằng bạn có thể biểu diễn **mọi thứ** chỉ bằng functions — kể cả `true`, `false`, số, và phép `if`:

```typescript
// filename: src/church.ts

// TRUE = function chọn cái đầu tiên
const TRUE = <T>(a: T) => (_b: T): T => a;

// FALSE = function chọn cái thứ hai
const FALSE = <T>(_a: T) => (b: T): T => b;

// IF = function nhận điều_kiện, rồi trả function chọn
const IF = <T>(condition: (a: T) => (b: T) => T) =>
    (then: T) => (otherwise: T): T =>
        condition(then)(otherwise);

// Thử nghiệm:
console.log(IF(TRUE)("Đúng!")("Sai!"));
// Output: Đúng!
// TRUE chọn cái đầu tiên → "Đúng!"

console.log(IF(FALSE)("Đúng!")("Sai!"));
// Output: Sai!
// FALSE chọn cái thứ hai → "Sai!"
```

TRUE là "luôn chọn cái đầu tiên". FALSE là "luôn chọn cái thứ hai". IF chỉ gọi điều kiện rồi để nó tự chọn.

Đây là **trò ảo thuật**: không `if-else`, không `true/false`, chỉ có functions gọi functions. Church chứng minh rằng điều này đủ để tính **mọi thứ** máy tính có thể tính.

> **💡 Điều này có ý nghĩa gì cho bạn?** Mỗi lần bạn viết `(x) => x + 1` trong TypeScript, bạn đang dùng ý tưởng của Church từ 1936. Arrow function **chính là** Lambda Calculus — chỉ đổi `λ` thành `=>`.

---

## ✅ Checkpoint 1.1

> Ghi nhớ:
> 1. **Lambda Calculus** = hệ thống mà mọi thứ chỉ cần functions
> 2. **Arrow function** `(x) => x + 1` chính là lambda `λx. x + 1`
> 3. **β-reduction** = bỏ giá trị vào function rồi tính (thay biến)
> 4. **Higher-order function** = function nhận/trả function (máy lớn chứa máy nhỏ)
>
> **Test nhanh**: Cho `const f = (x: number) => x * 3`. `f(4)` = bao nhiêu?
> <details><summary>Đáp án</summary>12. β-reduction: thay x = 4 → 4 * 3 = 12. Máy nhân 3, bỏ 4 vào, ra 12.</details>

---

## 1.2 — Curry-Howard: Types = Lời hứa, Compiler = Thầy giáo

### Câu chuyện

Đi mua sữa ở siêu thị. Trên hộp sữa viết: **"Sữa tươi 100%, không đường, hạn sử dụng: 30/06/2026"**. Bạn TIN nhãn dán đó — không tự kiểm tra bằng kính hiển vi.

Nhãn dán trên hộp sữa = **type annotation** trên code.

Khi bạn viết:

```typescript
const price: number = 35000;
```

Bạn đang dán nhãn: "Giá trị `price` **chắc chắn** là số. Tôi hứa." Và TypeScript compiler sẽ **kiểm tra lời hứa đó** — nếu ai đó viết `price = "hello"`, compiler **từ chối** ngay.

### Curry-Howard Correspondence

Năm 1969, hai nhà toán học Haskell Curry và William Howard phát hiện mối liên hệ đáng kinh ngạc:

| Logic | Programming |
|-------|-------------|
| Mệnh đề (proposition) | Type |
| Bằng chứng (proof) | Giá trị (value) |
| Phép suy luận | Function |
| "Nếu A thì B" | `(a: A) => B` |

Nói đơn giản: **mỗi type là một lời hứa**, và **mỗi giá trị đúng type là bằng chứng** rằng lời hứa được giữ.

```typescript
// filename: src/curry_howard.ts
import assert from "node:assert/strict";

// LỜI HỨA: function này nhận string, TRẢ VỀ number
// → "Nếu bạn cho tôi string, tôi SẼ cho bạn number"
const stringLength = (s: string): number => s.length;

// BẰNG CHỨNG: gọi function → nhận được giá trị đúng type
const result: number = stringLength("TypeScript");

assert.strictEqual(result, 10);
console.log(`"TypeScript" has ${result} characters`);
// Output: "TypeScript" has 10 characters

// Compiler kiểm tra lời hứa:
// stringLength(42);
// ❌ Error: Argument of type 'number' is not assignable to parameter of type 'string'
// → Compiler: "Bạn hứa cho tôi string, nhưng đưa number? KHÔNG ĐƯỢC!"
```

### Compiler = Thầy giáo nghiêm khắc

Nhiều người thấy compiler báo lỗi và khó chịu. Nhưng hãy nghĩ thế này:

Compiler giống **thầy giáo kiểm bài trước khi nộp thi**. Thầy gạch đỏ chỗ sai ≠ thầy ác. Thầy gạch đỏ = thầy **giúp bạn sửa trước khi giám khảo (production) đánh trượt**.

```typescript
// filename: src/compiler_teacher.ts

// ❌ Bài kiểm tra 1: nhầm kiểu
// const age: number = "twenty five";
// Thầy giáo: "Em hứa 'number' nhưng viết 'string'. Sửa lại!"

// ❌ Bài kiểm tra 2: quên trường hợp
type Direction = "north" | "south" | "east" | "west";

const assertNever = (x: never): never => {
    throw new Error(`Unexpected: ${x}`);
};

const oppositeOf = (d: Direction): Direction => {
    switch (d) {
        case "north": return "south";
        case "south": return "north";
        case "east":  return "west";
        case "west":  return "east";
        default: return assertNever(d);
    }
};

// ✅ Thầy gật đầu: "Đã xử lý hết mọi hướng. Bài tốt."
console.log(`Opposite of north: ${oppositeOf("north")}`);
// Output: Opposite of north: south
```

> **💡 strict mode + no `any` làm thầy giáo NGHIÊM hơn**: Không strict = thầy hiền, cho qua nhiều lỗi. Strict = thầy gạch đỏ mọi chỗ nghi ngờ. Tốn thời gian sửa lúc đầu, nhưng lúc deploy = tự tin gấp 10 lần.

### Hợp đồng cà phê

Type annotation giống **hợp đồng** giữa bạn và máy (compiler):

```typescript
// filename: src/contract.ts
import assert from "node:assert/strict";

// HỢP ĐỒNG: Tôi (function) cam kết:
// - Nhận vào: tên cà phê (string) + giá (number)
// - Trả ra: mô tả đầy đủ (string)
// Nếu vi phạm → compiler từ chối!
const describeOrder = (coffee: string, price: number): string =>
    `${coffee}: ${price.toLocaleString()}đ`;

// ✅ Đúng hợp đồng — compiler cho qua
assert.strictEqual(describeOrder("Cà phê sữa", 35000), "Cà phê sữa: 35,000đ");

// ❌ Vi phạm hợp đồng:
// describeOrder(35000, "Cà phê sữa");
// → Error: Argument of type 'number' is not assignable to parameter of type 'string'
// Compiler: "Hợp đồng ghi string trước, number sau. Bạn đưa ngược!"

console.log(describeOrder("Bạc xỉu", 29000));
// Output: Bạc xỉu: 29,000đ
```

---

## ✅ Checkpoint 1.2

> Ghi nhớ:
> 1. **Type = lời hứa**. `const x: number` = "x chắc chắn là số"
> 2. **Value = bằng chứng**. `const x: number = 42` = "đây, 42 là bằng chứng"
> 3. **Compiler = thầy giáo** kiểm tra lời hứa. Báo lỗi = giúp bạn sửa sớm
> 4. **`(a: A) => B`** = "nếu bạn cho A, tôi hứa trả B"
>
> **Test nhanh**: Code này có compile không?
> ```typescript
> const add = (a: number, b: number): number => a + b;
> const result: string = add(1, 2);
> ```
> <details><summary>Đáp án</summary>Không! `add` hứa trả `number`, nhưng `result` đòi `string`. Compiler từ chối: "Type 'number' is not assignable to type 'string'". Lời hứa bị vi phạm.</details>

---

## 1.3 — Algebraic Types: Đếm bằng phép cộng và phép nhân

### Câu chuyện: Menu quán cà phê

Bạn mở quán cà phê. Menu có:
- **Đồ uống**: Cà phê, Trà, Sinh tố (3 loại)
- **Size**: Nhỏ, Vừa, Lớn (3 size)

Hỏi: Có bao nhiêu cách order?

Bạn đếm: Cà phê Nhỏ, Cà phê Vừa, Cà phê Lớn, Trà Nhỏ, Trà Vừa, Trà Lớn, Sinh tố Nhỏ, Sinh tố Vừa, Sinh tố Lớn = **9 cách**.

Hoặc tính nhanh: 3 đồ uống × 3 size = **9**.

Đó chính là **phép nhân kiểu** — Product Type.

### Product Type = Chọn TẤT CẢ = Phép nhân

Khi bạn tạo object type, bạn phải chọn **TẤT CẢ** fields — giống order phải chọn cả đồ uống VÀ size:

```typescript
// filename: src/product.ts
import assert from "node:assert/strict";

type Drink = "coffee" | "tea" | "smoothie";   // 3 lựa chọn
type Size = "S" | "M" | "L";                  // 3 lựa chọn

// Product type: phải chọn CẢ drink VÀ size
type Order = {
    readonly drink: Drink;
    readonly size: Size;
};

// Bao nhiêu orders khác nhau có thể tạo?
// 3 drinks × 3 sizes = 9 orders!
const order1: Order = { drink: "coffee", size: "M" };
const order2: Order = { drink: "smoothie", size: "L" };

console.log(`Order: ${order1.drink} size ${order1.size}`);
// Output: Order: coffee size M
```

Công thức: **Product type = nhân các trường với nhau**.

```
type Order = { drink: Drink, size: Size }
Số trạng thái = |Drink| × |Size| = 3 × 3 = 9
```

Thêm ví dụ:

```typescript
// filename: src/product_count.ts

// Point có bao nhiêu giá trị khả dĩ?
type SmallNumber = 0 | 1 | 2;                 // 3 giá trị
type Point = { readonly x: SmallNumber; readonly y: SmallNumber };
// |Point| = 3 × 3 = 9 điểm: (0,0), (0,1), (0,2), (1,0),...

// User có bao nhiêu trạng thái?
type User = {
    readonly isAdmin: boolean;     // 2 giá trị
    readonly isActive: boolean;    // 2 giá trị
    readonly isVerified: boolean;  // 2 giá trị
};
// |User| = 2 × 2 × 2 = 8 trạng thái

console.log("Product types = phép nhân:");
console.log("  Order:  3 × 3 = 9");
console.log("  Point:  3 × 3 = 9");
console.log("  User:   2 × 2 × 2 = 8");
// Output:
// Product types = phép nhân:
//   Order:  3 × 3 = 9
//   Point:  3 × 3 = 9
//   User:   2 × 2 × 2 = 8
```

> **💡 Tại sao gọi "Algebraic"?** Vì types tuân theo quy tắc toán học — nhân, cộng, giống đại số. Không phải ngẫu nhiên mà `type Order = { drink: Drink; size: Size }` giống `Order = Drink × Size` trong toán.

### Sum Type = Chọn MỘT = Phép cộng

Ở Chapter 0 bạn đã gặp discriminated unions. Bây giờ ta nhìn nó qua lăng kính "đếm trạng thái":

```typescript
// filename: src/sum.ts
import assert from "node:assert/strict";

// Thanh toán: chọn MỘT trong 3 cách
type Payment =
    | { readonly tag: "cash" }                              // 1
    | { readonly tag: "card"; readonly last4: string }      // 1 (gộp)
    | { readonly tag: "momo"; readonly phone: string };     // 1 (gộp)

// Nếu chỉ đếm theo tag: 3 cách thanh toán
// Nhưng nếu đếm cả data bên trong thì phức tạp hơn — phần sau

// Ẩn dụ: Tiệm kem — chỉ chọn MỘT vị
type IceCreamFlavor =
    | { readonly tag: "vanilla" }
    | { readonly tag: "chocolate" }
    | { readonly tag: "strawberry" };
// |IceCreamFlavor| = 1 + 1 + 1 = 3

// Kèo bóng đá: thắng, thua, hoặc hòa
type MatchResult =
    | { readonly tag: "win"; readonly score: string }
    | { readonly tag: "lose"; readonly score: string }
    | { readonly tag: "draw"; readonly score: string };
// Đếm theo tag: 3 kết quả

console.log("Sum types = phép cộng:");
console.log("  IceCreamFlavor: 1 + 1 + 1 = 3");
console.log("  MatchResult:    1 + 1 + 1 = 3 (theo tag)");
// Output:
// Sum types = phép cộng:
//   IceCreamFlavor: 1 + 1 + 1 = 3
//   MatchResult:    1 + 1 + 1 = 3 (theo tag)
```

### Kết hợp: Product + Sum

Sức mạnh thật sự nằm ở chỗ **kết hợp cả hai**:

```typescript
// filename: src/combined.ts

type Size = "S" | "M" | "L";  // 3

type Drink =
    | { readonly tag: "coffee"; readonly shots: 1 | 2 | 3 }    // 3 (1 tag × 3 shots)
    | { readonly tag: "tea" }                                    // 1
    | { readonly tag: "smoothie"; readonly fruit: "mango" | "banana" };  // 2 (1 tag × 2 fruits)
// |Drink| = 3 + 1 + 2 = 6

type FullOrder = {
    readonly drink: Drink;    // 6 lựa chọn
    readonly size: Size;      // 3 lựa chọn
};
// |FullOrder| = 6 × 3 = 18 đơn hàng khác nhau

console.log("FullOrder: (3 + 1 + 2) × 3 = 6 × 3 = 18");
// Output: FullOrder: (3 + 1 + 2) × 3 = 6 × 3 = 18
```

### Bảng quy tắc đếm

| Cấu trúc | TypeScript | Phép tính | Ví dụ |
|-----------|-----------|-----------|-------|
| Object type `{ a: A; b: B }` | Product | `\|A\| × \|B\|` | `{ drink: 3, size: 3 }` = 9 |
| Union `A \| B` | Sum | `\|A\| + \|B\|` | `"coffee" \| "tea"` = 2 |
| `boolean` | — | 2 | `true \| false` |
| `string` | — | ∞ | Vô hạn chuỗi |
| `number` | — | ∞ | Vô hạn số |
| Literal `"S" \| "M" \| "L"` | Sum | Đếm | 3 |

---

## ✅ Checkpoint 1.3

> Ghi nhớ:
> 1. **Product type** (object) = phép nhân. `{ a: A; b: B }` → `|A| × |B|`
> 2. **Sum type** (union) = phép cộng. `A | B` → `|A| + |B|`
> 3. **boolean** = 2. **Literal union** = đếm variants. **string/number** = ∞
>
> **Test nhanh**: Bao nhiêu giá trị cho type này?
> ```typescript
> type Card = {
>     readonly suit: "hearts" | "diamonds" | "clubs" | "spades";
>     readonly rank: "A" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "J" | "Q" | "K";
> };
> ```
> <details><summary>Đáp án</summary>4 suits × 13 ranks = 52. Đúng — một bộ bài tây chuẩn!</details>

---

## 1.4 — Make Illegal States Unrepresentable

### Câu chuyện: Đơn hàng bị bug

Bạn viết hệ thống cho quán cà phê. Đơn hàng có các trạng thái: **Đã xác nhận → Đang pha → Sẵn sàng → Đã lấy**. Dev cũ dùng boolean flags:

```typescript
// filename: src/bad_design.ts

// ❌ THIẾT KẾ XẤU — boolean flags
type BadOrder = {
    readonly isConfirmed: boolean;    // 2
    readonly isPreparing: boolean;    // 2
    readonly isReady: boolean;        // 2
    readonly isPickedUp: boolean;     // 2
};

// Bao nhiêu trạng thái? 2 × 2 × 2 × 2 = 16 trạng thái!
// Nhưng chỉ CẦN 4 trạng thái hợp lệ:
// 1. confirmed  = (T, F, F, F)
// 2. preparing  = (T, T, F, F)
// 3. ready      = (T, T, T, F)
// 4. pickedUp   = (T, T, T, T)

// Vậy có 16 - 4 = 12 trạng thái VÔ NGHĨA:
// - isReady = true, isConfirmed = false ???  Sẵn sàng mà chưa xác nhận?
// - isPreparing = false, isPickedUp = true ??? Chưa pha mà đã lấy?

console.log("BadOrder: 2⁴ = 16 states, chỉ 4 hợp lệ → 12 bugs tiềm ẩn!");
```

**12 trạng thái vô nghĩa = 12 bugs tiềm ẩn.** Mỗi bug là một cái hố mà ai đó — bạn, đồng nghiệp, hoặc user — sẽ rơi vào.

### Giải pháp: Discriminated Union

```typescript
// filename: src/good_design.ts
import assert from "node:assert/strict";

// ✅ THIẾT KẾ TỐT — discriminated union
type GoodOrder =
    | { readonly tag: "confirmed"; readonly orderId: string }
    | { readonly tag: "preparing"; readonly orderId: string; readonly barista: string }
    | { readonly tag: "ready";     readonly orderId: string; readonly barista: string }
    | { readonly tag: "pickedUp";  readonly orderId: string; readonly time: Date };

// Bao nhiêu trạng thái? 1 + 1 + 1 + 1 = 4 — ĐÚNG 4!
// Không thể tạo "sẵn sàng mà chưa xác nhận"
// Không thể tạo "đang pha và đã lấy cùng lúc"
// Compiler NGĂN bạn tạo trạng thái vô nghĩa!

// Mỗi trạng thái chỉ chứa data phù hợp:
const order1: GoodOrder = { tag: "confirmed", orderId: "A001" };
const order2: GoodOrder = { tag: "preparing", orderId: "A001", barista: "An" };
const order3: GoodOrder = { tag: "ready", orderId: "A001", barista: "An" };

const assertNever = (x: never): never => {
    throw new Error(`Unexpected: ${JSON.stringify(x)}`);
};

const describeOrder = (order: GoodOrder): string => {
    switch (order.tag) {
        case "confirmed":  return `Đơn ${order.orderId}: Đã xác nhận ✔`;
        case "preparing":  return `Đơn ${order.orderId}: ${order.barista} đang pha ☕`;
        case "ready":      return `Đơn ${order.orderId}: Sẵn sàng lấy! 🎉`;
        case "pickedUp":   return `Đơn ${order.orderId}: Đã lấy lúc ${order.time.toLocaleTimeString()}`;
        default: return assertNever(order);
    }
};

console.log(describeOrder(order1));
console.log(describeOrder(order2));
console.log(describeOrder(order3));
// Output:
// Đơn A001: Đã xác nhận ✔
// Đơn A001: An đang pha ☕
// Đơn A001: Sẵn sàng lấy! 🎉
```

### So sánh: Boolean flags vs Discriminated Union

| | Boolean flags | Discriminated Union |
|---|---|---|
| Tổng trạng thái | 2⁴ = **16** | **4** |
| Trạng thái hợp lệ | 4 | 4 |
| **Trạng thái vô nghĩa** | **12** | **0** |
| Compiler bảo vệ? | ❌ Không | ✅ `assertNever` |
| Mỗi variant có data riêng? | ❌ Tất cả fields luôn có | ✅ Chỉ data phù hợp |

> **💡 Quy tắc vàng**: Khi bạn thấy 2+ boolean flags mà chúng liên quan đến nhau → thay bằng discriminated union. Bạn vừa diệt trước hàng tá bugs mà không cần viết test nào.

### Ví dụ thực tế: API Response

Pattern này dùng ở mọi nơi trong production:

```typescript
// filename: src/api_response.ts

// ❌ Cách cũ — boolean flags
type BadResponse = {
    readonly isLoading: boolean;   // 2
    readonly isError: boolean;     // 2
    readonly data: string | null;  // string = ∞, nhưng null cũng là 1 giá trị
    readonly error: string | null;
};
// Vấn đề: isLoading = true VÀ isError = true cùng lúc? Vô nghĩa!
// data có giá trị nhưng isError = true? Bug!
// Chỉ 3 trạng thái hợp lệ: loading, error, success

// ✅ Cách tốt — discriminated union
type ApiResponse<T> =
    | { readonly tag: "loading" }
    | { readonly tag: "error"; readonly message: string }
    | { readonly tag: "success"; readonly data: T };
// Đúng 3 trạng thái — không hơn, không kém

const assertNever = (x: never): never => {
    throw new Error(`Unexpected: ${JSON.stringify(x)}`);
};

const render = (response: ApiResponse<string>): string => {
    switch (response.tag) {
        case "loading": return "⏳ Đang tải...";
        case "error":   return `❌ Lỗi: ${response.message}`;
        case "success": return `✅ ${response.data}`;
        default: return assertNever(response);
    }
};

console.log(render({ tag: "loading" }));
console.log(render({ tag: "error", message: "Network timeout" }));
console.log(render({ tag: "success", data: "Cà phê sữa đá" }));
// Output:
// ⏳ Đang tải...
// ❌ Lỗi: Network timeout
// ✅ Cà phê sữa đá
```

---

## ✅ Checkpoint 1.4

> Ghi nhớ:
> 1. **Boolean flags** → trạng thái tăng theo cấp số nhân (2ⁿ) → hầu hết vô nghĩa
> 2. **Discriminated union** → chỉ có đúng trạng thái hợp lệ (cộng thay nhân)
> 3. **"Make illegal states unrepresentable"** = thiết kế type sao cho compiler NGĂN bugs
> 4. **Quy tắc vàng**: 2+ boolean flags liên quan → thay bằng union
>
> **Test nhanh**: Type này có bao nhiêu trạng thái? Bao nhiêu hợp lệ?
> ```typescript
> type UserStatus = {
>     readonly isLoggedIn: boolean;
>     readonly isAdmin: boolean;
>     readonly isBanned: boolean;
> };
> ```
> <details><summary>Đáp án</summary>2³ = 8 trạng thái. Nhưng "đã bị banned VÀ đang logged in VÀ là admin" có hợp lý không? Hầu hết app sẽ nói không. Dùng union: `type UserStatus = "guest" | "user" | "admin" | "banned"` → chỉ 4 trạng thái.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Đếm trạng thái

Tính `|T|` (số trạng thái) cho mỗi type:

```typescript
// a)
type Light = "red" | "yellow" | "green";

// b)
type Coord = { readonly x: boolean; readonly y: boolean };

// c)
type Maybe = { readonly tag: "some"; readonly value: boolean } | { readonly tag: "none" };

// d)
type Combo = {
    readonly drink: "coffee" | "tea";
    readonly size: "S" | "M" | "L";
    readonly ice: boolean;
};
```

<details><summary>✅ Lời giải Bài 1</summary>

```
a) |Light| = 3               (3 literals)
b) |Coord| = 2 × 2 = 4       (product: boolean × boolean)
c) |Maybe| = 2 + 1 = 3       (sum: {some: boolean} = 2, {none} = 1)
d) |Combo| = 2 × 3 × 2 = 12  (product: drink × size × ice)
```

</details>

---

**Bài 2** (10 phút): Refactor boolean flags → discriminated union

Code hiện tại dùng boolean flags. Refactor thành discriminated union:

```typescript
// ❌ Code hiện tại
type UploadStatus = {
    readonly isUploading: boolean;
    readonly isComplete: boolean;
    readonly isFailed: boolean;
    readonly progress: number;      // 0-100
    readonly errorMessage: string;  // chỉ dùng khi failed
    readonly fileUrl: string;       // chỉ dùng khi complete
};
// Bao nhiêu trạng thái vô nghĩa? 2³ = 8, hợp lệ = 3, vô nghĩa = 5
```

<details><summary>💡 Gợi ý</summary>3 trạng thái hợp lệ: uploading (cần progress), complete (cần fileUrl), failed (cần errorMessage). Mỗi variant chỉ chứa data cần thiết.</details>

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
// ✅ Refactored — discriminated union
type UploadStatus =
    | { readonly tag: "uploading"; readonly progress: number }
    | { readonly tag: "complete"; readonly fileUrl: string }
    | { readonly tag: "failed"; readonly errorMessage: string };

// Chỉ 3 trạng thái — compiler ngăn mọi trạng thái vô nghĩa!
// Mỗi variant chỉ chứa data phù hợp:
// - uploading → progress (không cần errorMessage hay fileUrl)
// - complete → fileUrl (không cần progress)
// - failed → errorMessage (không cần progress hay fileUrl)
```

</details>

---

**Bài 3** (15 phút): Higher-order functions + compose

Viết function `pipeline` nhận một mảng functions `(x: number) => number` và chain chúng lại:

```typescript
// pipeline([addOne, double, addOne])(5)
// Chạy TRÁI sang PHẢI: addOne(5) → double(6) → addOne(12)
// = 5 → 6 → 12 → 13
```

<details><summary>💡 Gợi ý</summary>Dùng `.reduce()` — bắt đầu với giá trị ban đầu, apply từng function lần lượt.</details>

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

const pipeline = (fns: ReadonlyArray<(x: number) => number>) =>
    (value: number): number =>
        fns.reduce((acc, fn) => fn(acc), value);

const addOne = (x: number): number => x + 1;
const double = (x: number): number => x * 2;
const square = (x: number): number => x * x;

// Chain: 5 → addOne → 6 → double → 12 → addOne → 13
assert.strictEqual(pipeline([addOne, double, addOne])(5), 13);

// Chain: 3 → double → 6 → square → 36
assert.strictEqual(pipeline([double, square])(3), 36);

// Chain rỗng: trả giá trị gốc
assert.strictEqual(pipeline([])(42), 42);

console.log("Pipeline tests passed! ✅");
// Output: Pipeline tests passed! ✅
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `Type 'X' is not assignable to type 'Y'` | Vi phạm "hợp đồng" type | Kiểm tra: bạn đang trả đúng type mà function hứa chưa? |
| Switch thiếu case nhưng compiler không báo | Chưa có `assertNever` ở `default` | Thêm `default: return assertNever(x)` |
| `Property 'X' does not exist on type 'Y'` | Chưa narrow type bằng `switch` | Kiểm tra `tag` trước khi truy cập field riêng |
| Arrow function trả `void` thay vì giá trị | Dùng `{}` thay vì expression | `(x) => x + 1` ✅ vs `(x) => { x + 1 }` ❌ (thiếu `return`) |

---

## Tóm tắt

- ✅ **Lambda Calculus**: Mọi thứ chỉ cần functions. Arrow function `(x) => x + 1` chính là lambda `λx. x + 1` từ 1936.
- ✅ **β-reduction**: Bỏ giá trị vào function rồi tính. `addOne(5)` = thay `x = 5` → `6`.
- ✅ **Curry-Howard**: Type = lời hứa, value = bằng chứng, compiler = thầy giáo kiểm bài.
- ✅ **Product types** (objects): Phép nhân — `{ a: A; b: B }` → `|A| × |B|` trạng thái.
- ✅ **Sum types** (unions): Phép cộng — `A | B` → `|A| + |B|` trạng thái.
- ✅ **Make illegal states unrepresentable**: Boolean flags → discriminated unions. Từ 2ⁿ trạng thái xuống đúng k trạng thái hợp lệ.

## Tiếp theo

→ Chapter 2: **Algorithmic Thinking & Complexity** — bạn sẽ học Big-O (tại sao `.push()` nhanh mà `.unshift()` chậm?), recursion (function gọi chính nó), và `.reduce()` — "vũ khí tối thượng" thay thế mọi vòng lặp `for`.
