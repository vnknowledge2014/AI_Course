# Chapter 25b — Đại số trừu tượng: Equivalence & Order (Effect-TS)

> **Bạn sẽ học được**:
> - Vấn đề chí mạng của toán tử `===` trong JavaScript/TypeScript.
> - **Equivalence (Eq)**: Abstraction cho việc so sánh bằng (Equality) an toàn và có thể kết hợp.
> - **Order (Ord)**: Abstraction cho việc sắp xếp (Sorting), giải quyết bài toán custom sorting dễ sinh bug.
> - Khái niệm "Data.struct" trong Effect để tự động hóa Equality.
> - Combinators: `mapInput`, `reverse`, `combine` — sức mạnh thực sự của đại số.
>
> **Yêu cầu trước**: Chapter 25 (Effect-first).
> **Thời gian đọc**: ~30 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Code của bạn không còn những vòng lặp hay hàm `sort` dài dòng, thay vào đó là các abstraction thanh lịch và reusable.

---

Hãy xem dòng code TypeScript này:
`{ id: 1 } === { id: 1 }`
Kết quả là `false`. Chào mừng bạn đến với JavaScript!

Toán tử `===` so sánh bộ nhớ (Reference Equality) chứ không so sánh giá trị cấu trúc (Structural Equality). Điều này khiến việc so sánh 2 đối tượng `User` hay 2 mảng cực kỳ phiền toái. Bạn phải tự viết các hàm `isEqual(user1, user2)` khắp nơi.
Tiếp theo, thử sắp xếp mảng các `User` theo tên, nếu trùng tên thì sắp xếp theo tuổi giảm dần. Hàm `sort` của bạn sẽ có một đống lệnh `if/else`, trả về `1`, `-1`, `0` rất dễ nhầm lẫn.

Functional Programming giải quyết tận gốc rễ vấn đề này thông qua hai concept toán học: **Equivalence** (Sự tương đương) và **Order** (Sự sắp xếp). Effect-TS cung cấp cho bạn những công cụ sắc bén nhất để làm điều này.

---

## 25b.1 — Vấn đề của Reference Equality

### Khi hai thứ giống hệt nhau nhưng lại khác nhau

```typescript
// filename: src/equality_problem.ts
import assert from "node:assert/strict";

const u1 = { id: 1, name: "Alice" };
const u2 = { id: 1, name: "Alice" };

// ❌ Thất bại: Reference equality
// assert.ok(u1 === u2); // Fails!

// Thay vào đó, bạn phải so sánh thủ công:
const isSameUser = (a: typeof u1, b: typeof u2) => a.id === b.id && a.name === b.name;
assert.ok(isSameUser(u1, u2)); // True

// Hoặc sử dụng lodash.isEqual, mất type safety và chậm.
```

Effect-TS đưa ra một giải pháp cực kỳ thanh lịch nhờ mô-đun `Data`. Nó giúp tạo ra các Object và Array hỗ trợ **Structural Equality** (So sánh cấu trúc) ra khỏi hộp.

```typescript
// filename: src/effect_data_struct.ts
import { Data, Equal } from "effect";
import assert from "node:assert/strict";

// Thay vì tạo object JS thông thường, ta dùng Data.struct
const u1 = Data.struct({ id: 1, name: "Alice" });
const u2 = Data.struct({ id: 1, name: "Alice" });

// ❌ Toán tử === của JS vẫn trả về false do giới hạn của ngôn ngữ
// assert.ok(u1 === u2); // Fails!

// ✅ Nhưng Effect cung cấp hàm Equal.equals!
assert.ok(Equal.equals(u1, u2)); // TRUE!

// Hoạt động với cả Array (Tuple)
const arr1 = Data.tuple(1, 2, 3);
const arr2 = Data.tuple(1, 2, 3);
assert.ok(Equal.equals(arr1, arr2)); // TRUE!

console.log("Effect Data Struct OK ✅");
```

> **💡 Bản chất**: `Data.struct` và `Data.tuple` lén gắn thêm hàm `[Equal.symbol]` và `[Hash.symbol]` vào prototype của object. Khi bạn gọi `Equal.equals`, Effect sẽ kiểm tra nếu object có các Symbol này, nó sẽ dùng logic so sánh tùy chỉnh!

---

## 25b.2 — Equivalence Type Class

### Tự định nghĩa tiêu chuẩn của "Sự Bằng Nhau"

Tuy `Data.struct` rất tuyệt, nhưng đôi khi bạn muốn 2 đối tượng được coi là Bằng Nhau chỉ dựa trên một vài thuộc tính. Ví dụ: 2 `User` là một nếu chúng có cùng `id`, mặc kệ tên có bị đổi hay không.

Đó là lúc ta cần type class **`Equivalence<A>`**.
Một Equivalence cho kiểu `A` chỉ đơn giản là một hàm: `(x: A, y: A) => boolean`.

```typescript
// filename: src/equivalence_concept.ts
import { Equivalence, String, Number } from "effect";
import assert from "node:assert/strict";

type User = {
    readonly id: number;
    readonly name: string;
};

// ── 1. Tạo Equivalence tùy chỉnh ──
// Chúng ta tự viết một hàm kiểm tra
const userEquivalenceById: Equivalence.Equivalence<User> = (a, b) => a.id === b.id;

const u1: User = { id: 1, name: "Alice" };
const u2: User = { id: 1, name: "Alice in Wonderland" };

assert.ok(userEquivalenceById(u1, u2)); // True, vì id giống nhau

// ── 2. Tạo Equivalence bằng Combinator (Sức mạnh thực sự!) ──
// Thay vì tự viết hàm so sánh thủ công, ta BẢN ĐỒ (map) Equivalence của Number
// sang Equivalence của User.

// Giải thích:
// Number.Equivalence là (a: number, b: number) => boolean
// Chúng ta "mapInput" để biến đổi (User) thành (number) trước khi so sánh.
const userEq: Equivalence.Equivalence<User> = Equivalence.mapInput(
    Number.Equivalence,
    (user: User) => user.id
);

assert.ok(userEq(u1, u2)); // Vẫn True!

// ── 3. So sánh phức hợp (Struct) ──
// Bạn muốn 2 User bằng nhau khi CẢ id và name giống nhau?
const strictUserEq = Equivalence.struct({
    id: Number.Equivalence,
    name: String.Equivalence
});

assert.ok(strictUserEq({ id: 1, name: "Bob" }, { id: 1, name: "Bob" })); // True
assert.ok(!strictUserEq(u1, u2)); // False, vì name khác nhau

console.log("Equivalence OK ✅");
```

> **💡 mapInput**: (Còn gọi là Contravariant Functor). Bạn có công cụ so sánh Number. Bạn cần công cụ so sánh User. Giải pháp: Cung cấp hàm biến đổi `User -> Number`. Boom! Bạn có công cụ so sánh User.

---

## 25b.3 — Vấn đề của Sorting trong JavaScript

Nếu Equality giải quyết `===`, thì **Order** giải quyết vấn đề của `> <` và `Array.sort()`.

```typescript
// filename: src/sorting_problem.ts
import assert from "node:assert/strict";

type Employee = { id: number; name: string; salary: number };

const employees: Employee[] = [
    { id: 1, name: "Charlie", salary: 3000 },
    { id: 2, name: "Alice", salary: 5000 },
    { id: 3, name: "Bob", salary: 5000 }
];

// ❌ Sorting thuần của JS:
// Yêu cầu: Sort theo salary giảm dần. Nếu trùng salary, sort theo name tăng dần.
const sortedJS = [...employees].sort((a, b) => {
    if (a.salary > b.salary) return -1;
    if (a.salary < b.salary) return 1;
    // Trùng salary
    if (a.name > b.name) return 1;
    if (a.name < b.name) return -1;
    return 0;
});
// Rất dài, dễ nhầm dấu, khó bảo trì, khó test từng phần.
```

---

## 25b.4 — Order: Sắp xếp mang tính Đại Số

### Bóc tách logic so sánh

Một **`Order<A>`** là một hàm: `(x: A, y: A) => -1 | 0 | 1`.
Effect định nghĩa sẵn Order cho Primitives (String, Number, Date, v.v.).

```typescript
// filename: src/order_concept.ts
import { Order, Number, String, Array as A } from "effect";
import assert from "node:assert/strict";

type Employee = { id: number; name: string; salary: number };

const employees: Employee[] = [
    { id: 1, name: "Charlie", salary: 3000 },
    { id: 2, name: "Alice", salary: 5000 },
    { id: 3, name: "Bob", salary: 5000 }
];

// 1. Tạo Order cơ bản bằng mapInput
// Sắp xếp theo Salary (tăng dần)
const bySalary: Order.Order<Employee> = Order.mapInput(
    Number.Order,
    (emp) => emp.salary
);

// Sắp xếp theo Name (tăng dần)
const byName: Order.Order<Employee> = Order.mapInput(
    String.Order,
    (emp) => emp.name
);

// 2. Combinators: Đảo ngược (Reverse)
const bySalaryDesc = Order.reverse(bySalary);

// 3. Kết hợp (Combine)
// Yêu cầu: Salary giảm dần, NẾU trùng salary THÌ name tăng dần.
const bySalaryDescThenName = Order.combine(bySalaryDesc, byName);

// 4. Sử dụng với Array
const sorted = A.sort(employees, bySalaryDescThenName);

assert.deepStrictEqual(sorted, [
    { id: 2, name: "Alice", salary: 5000 },    // Salary max, A < B
    { id: 3, name: "Bob", salary: 5000 },      // Salary max, B > A
    { id: 1, name: "Charlie", salary: 3000 }   // Salary thấp
]);

console.log("Order OK ✅");
```

> **💡 Tại sao Order lại vượt trội?**: 
> 1. Tính **Composable**: Bạn tạo các mảnh Order nhỏ (`bySalary`, `byName`). Bạn kết hợp chúng bằng `Order.combine`. Bạn đảo ngược bằng `Order.reverse`. Không có một câu lệnh `if/else` nào!
> 2. Tính **Reusable**: `byName` có thể đem đi dùng ở khắp nơi.
> 3. Chuẩn bị cho Semigroup/Monoid (Ch26). `Order` chính là một dạng Semigroup!

---

## ✅ Checkpoint 25b.1-25b.4

> Đến đây bạn phải hiểu:
> 1. **`Data.struct`**: Giải pháp có sẵn của Effect cho cấu trúc dữ liệu Immutable hỗ trợ Structural Equality.
> 2. **`Equivalence<A>`**: Giao thức cho phép tự định nghĩa khái niệm "bằng nhau" của 2 object.
> 3. **`Order<A>`**: Giao thức sắp xếp.
> 4. **`mapInput`**: Combinator tối thượng để tạo Eq/Ord cho kiểu phức tạp dựa trên kiểu có sẵn (như String/Number).
> 5. **`Order.combine`**: Kết hợp nhiều tiêu chí sắp xếp một cách an toàn và gọn gàng.
>
> **Test nhanh**: Nếu tôi muốn sắp xếp một mảng các `Product` theo `price` tăng dần, tôi nên bắt đầu từ đâu?
> <details><summary>Đáp án</summary>Lấy `Number.Order`. Dùng `Order.mapInput(Number.Order, (p: Product) => p.price)`. Sau đó đưa vào `Array.sort`.</details>

---

## 🏋️ Bài tập

**Bài 1** (10 phút): Multi-criteria Order

```typescript
// Cho kiểu Ticket. 
// Hãy tạo một Order để sắp xếp vé theo mức độ ưu tiên:
// 1. status: "open" lên trước, "closed" xuống sau.
// 2. Nếu cùng status: priority: "high" lên trước, "low" xuống sau.
// 3. Nếu cùng priority: createdAt: cũ hơn lên trước.

import { Order, String, Number } from "effect";

type Ticket = {
    id: string;
    status: "open" | "closed";
    priority: "high" | "low";
    createdAt: number; // timestamp
};

// HINT: Viết 3 Order riêng biệt cho từng tiêu chí bằng mapInput.
// Gợi ý cho status/priority: bạn có thể so sánh chuỗi, nhưng "high" lại < "low" về bảng chữ cái, 
// "open" > "closed" về chữ cái. Có thể map nó ra số trước khi so sánh.
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import { Order, Number, Array as A } from "effect";

type Ticket = { id: string; status: "open" | "closed"; priority: "high" | "low"; createdAt: number; };

// Ánh xạ string ra số để có thứ tự logic thay vì alphabetical
const statusValue = (s: Ticket["status"]) => s === "open" ? 1 : 2;
const priorityValue = (p: Ticket["priority"]) => p === "high" ? 1 : 2;

// 1. Order theo status
const byStatus = Order.mapInput(Number.Order, (t: Ticket) => statusValue(t.status));

// 2. Order theo priority
const byPriority = Order.mapInput(Number.Order, (t: Ticket) => priorityValue(t.priority));

// 3. Order theo thời gian tạo
const byCreatedAt = Order.mapInput(Number.Order, (t: Ticket) => t.createdAt);

// 4. Combine!
const masterOrder = Order.combine(byStatus, Order.combine(byPriority, byCreatedAt));

// Test
const tickets: Ticket[] = [
    { id: "1", status: "closed", priority: "high", createdAt: 100 },
    { id: "2", status: "open", priority: "low", createdAt: 300 },
    { id: "3", status: "open", priority: "high", createdAt: 200 },
    { id: "4", status: "open", priority: "high", createdAt: 50 },
];

const sorted = A.sort(tickets, masterOrder);
// Kết quả mong đợi theo thứ tự ID: "4", "3", "2", "1"
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| `Data.struct` vẫn dùng `===` ra `false` | Bản chất JavaScript | Nhớ dùng `Equal.equals(a, b)` thay vì `===`. |
| `Array.sort` thay đổi mảng gốc | Đang dùng method có sẵn của JS Array | Dùng `Array.sort(arr, order)` của module `effect/Array` — nó trả về mảng copy (Immutable). |
| TypeScript báo lỗi tham số `mapInput` bị đảo ngược | Đặc tính của Contravariant (Tỉ lệ nghịch) | `mapInput` nhận Order(B) và hàm (A => B), và trả về Order(A). Nhớ thứ tự tham số. |

---

## Tóm tắt

- ✅ JavaScript so sánh Reference Equality. Effect cung cấp **`Data.struct`** và **`Equal.equals`** để hỗ trợ Structural Equality.
- ✅ **Equivalence**: Type class trừu tượng hóa khái niệm "Bằng nhau". Bạn có thể định nghĩa 2 đối tượng là bằng nhau chỉ bằng ID của chúng thông qua `mapInput`.
- ✅ **Order**: Trừu tượng hóa việc sắp xếp (`<`, `>`, `===`). Giúp code dễ đọc, dễ kết hợp (composable).
- ✅ **Combinators**: `Order.reverse` và `Order.combine` giúp giải quyết các bài toán multi-criteria sorting cực kỳ thanh lịch mà không đụng tới một dòng `if/else`.

## Tiếp theo

→ Chapter 26 (Mới): **Semigroups & Monoids** — Đã biết thế nào là bằng nhau (Eq) và sắp xếp (Ord), giờ là lúc học cách Gộp (Merge) chúng lại bằng Semigroups trong Effect.
