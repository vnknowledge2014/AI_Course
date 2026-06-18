# Chapter 29 — Monoids & Abstract Algebra

> **Bạn sẽ học được**:
> - Semigroup = type with `concat` (combine two values)
> - Monoid = Semigroup + `empty` (identity element)
> - Monoids khắp nơi: number, string, array, object merge
> - `concatAll` — fold danh sách với Monoid
> - Practical: merge configs, aggregate reports, combine permissions
>
> **Yêu cầu trước**: Chapter 12 (reduce), Chapter 26-28 (FP patterns).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Thấy pattern "combine" KHẮP NƠI — và abstract nó thành Monoid.

---

Bạn biết cách ghép Lego không? Lấy 2 khối, ghép lại thành 1 khối lớn hơn. Lấy khối kết quả, ghép thêm khối nữa. Cứ thế. Có khối "rỗng" (khối 1x1 phẳng — không thêm gì). Ghép với bất kỳ khối nào → giữ nguyên. Đây chính xác là **Monoid**: một phép ghép (`concat`) + một phần tử rỗng (`empty`).

Đừng vội nghĩ "đây là toán". Monoid là thứ bạn dùng MỖI NGÀY:
- `1 + 2 + 3` — number addition, empty = `0`.
- `"hello" + " " + "world"` — string concatenation, empty = `""`.
- `[1,2].concat([3,4])` — array merge, empty = `[]`.
- `{...defaults, ...userConfig}` — object merge, empty = `{}`.

Monoid cho bạn tên để gọi pattern này, và `concatAll` để fold bất kỳ danh sách nào. `Array.reduce()` = Monoid ở dạng thô. Monoid = `reduce()` được abstract hóa thành interface reusable.

---

## Semigroups & Monoids — Combine anything

Semigroup = `concat(a, b)` (kết hợp hai thứ). Monoid = Semigroup + `empty` (phần tử không). Bạn đã dùng Monoid mỗi ngày: `+` (number), `.concat()` (array/string), `{...a, ...b}` (object merge). Chapter này cho bạn TÊN và INTERFACE để dùng có hệ thống.


## 29.1 — Semigroup: Combine Two Things

Semigroup là nửa đầu của Monoid. Chỉ có `concat` — cách GHÉP hai giá trị cùng type thành một. Yêu cầu duy nhất: **associativity** — `(a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)`. Thứ tự group không ảnh hưởng kết quả (nhưng thứ tự phần tử có thể ảnh hưởng — string concat không commutative: `"a"+"b" ≠ "b"+"a"`).

Tại sao associativity quan trọng? Vì nó cho phép **parallelize**. Nếu bạn có `a ⊕ b ⊕ c ⊕ d`, bạn có thể tính `(a ⊕ b)` và `(c ⊕ d)` SONG SONG, rồi ghép kết quả. Đây là nền tảng của MapReduce (Google), Spark, và mọi hệ thống distributed aggregation.

```typescript
// filename: src/semigroup.ts
import assert from "node:assert/strict";

// Semigroup<A> = { concat: (x: A, y: A) => A }
// Rule: concat must be ASSOCIATIVE
// concat(concat(a, b), c) === concat(a, concat(b, c))

type Semigroup<A> = {
    readonly concat: (x: A, y: A) => A;
};

// Examples of Semigroups:

// Number addition
const semigroupSum: Semigroup<number> = { concat: (x, y) => x + y };

// Number multiplication
const semigroupProduct: Semigroup<number> = { concat: (x, y) => x * y };

// String concatenation
const semigroupString: Semigroup<string> = { concat: (x, y) => x + y };

// Array concatenation
const semigroupArray = <A>(): Semigroup<readonly A[]> => ({
    concat: (x, y) => [...x, ...y],
});

// Max
const semigroupMax: Semigroup<number> = { concat: (x, y) => Math.max(x, y) };

// --- Tests ---
assert.strictEqual(semigroupSum.concat(3, 4), 7);
assert.strictEqual(semigroupProduct.concat(3, 4), 12);
assert.strictEqual(semigroupString.concat("hello", " world"), "hello world");
assert.deepStrictEqual(semigroupArray<number>().concat([1, 2], [3, 4]), [1, 2, 3, 4]);

// Associativity:
assert.strictEqual(
    semigroupSum.concat(semigroupSum.concat(1, 2), 3),
    semigroupSum.concat(1, semigroupSum.concat(2, 3)),
);

console.log("Semigroup OK ✅");
```

---

## 29.2 — Monoid: Semigroup + Empty

Semigroup chỉ biết GHÉP — nhưng không có "giá trị khởi đầu". Monoid thêm `empty` — phần tử TRUNG TÍNH. Ghép với bất kỳ thứ gì → không đổi. `0` cho phép cộng. `1` cho phép nhân. `""` cho phép string. `[]` cho phép array. `true` cho AND. `false` cho OR.

Tại sao cần `empty`? Vì `concatAll`/`reduce` cần giá trị khởi đầu. `[1,2,3].reduce((a,b) => a+b, ???)` — `???` chính là `empty`. Và quan trọng: `concatAll([])` phải return GÌ? Phải là `empty` — danh sách rỗng = không có gì để ghép = phần tử trung tính.

```typescript
// filename: src/monoid.ts
import assert from "node:assert/strict";

type Monoid<A> = {
    readonly concat: (x: A, y: A) => A;
    readonly empty: A;  // identity element
};

// Monoids (Semigroup + empty):
const monoidSum: Monoid<number> = { concat: (x, y) => x + y, empty: 0 };
const monoidProduct: Monoid<number> = { concat: (x, y) => x * y, empty: 1 };
const monoidString: Monoid<string> = { concat: (x, y) => x + y, empty: "" };
const monoidArray = <A>(): Monoid<readonly A[]> => ({ concat: (x, y) => [...x, ...y], empty: [] });
const monoidAll: Monoid<boolean> = { concat: (x, y) => x && y, empty: true };  // AND
const monoidAny: Monoid<boolean> = { concat: (x, y) => x || y, empty: false }; // OR

// Identity law: concat(empty, x) === concat(x, empty) === x
assert.strictEqual(monoidSum.concat(monoidSum.empty, 5), 5);
assert.strictEqual(monoidSum.concat(5, monoidSum.empty), 5);
assert.strictEqual(monoidString.concat(monoidString.empty, "hello"), "hello");

// concatAll = fold with monoid
const concatAll = <A>(M: Monoid<A>) => (values: readonly A[]): A =>
    values.reduce(M.concat, M.empty);

assert.strictEqual(concatAll(monoidSum)([1, 2, 3, 4, 5]), 15);
assert.strictEqual(concatAll(monoidProduct)([1, 2, 3, 4, 5]), 120);
assert.strictEqual(concatAll(monoidString)(["hello", " ", "world"]), "hello world");
assert.deepStrictEqual(concatAll(monoidArray<number>())([[1, 2], [3], [4, 5]]), [1, 2, 3, 4, 5]);
assert.strictEqual(concatAll(monoidAll)([true, true, true]), true);
assert.strictEqual(concatAll(monoidAll)([true, false, true]), false);

// Empty list = identity element
assert.strictEqual(concatAll(monoidSum)([]), 0);
assert.strictEqual(concatAll(monoidProduct)([]), 1);

console.log("Monoid + concatAll OK ✅");
```

> **💡 concatAll = Array.reduce with Monoid!** `[1,2,3].reduce((a,b) => a+b, 0)` = `concatAll(monoidSum)([1,2,3])`. Same thing, but Monoid makes the pattern EXPLICIT and reusable.

---

## ✅ Checkpoint 29.1-29.2

> Đến đây bạn phải hiểu:
> 1. **Semigroup** = type + `concat`. Associative
> 2. **Monoid** = Semigroup + `empty`. Identity element
> 3. **`concatAll`** = fold array with Monoid. reduce() abstracted
> 4. **Many monoids**: sum, product, string, array, boolean AND/OR
>
> **Test nhanh**: `concatAll(monoidProduct)([])` = ?
> <details><summary>Đáp án</summary>`1`! Empty list → return identity element. For product, identity = 1 (multiply by 1 = no change).</details>

---

## 29.3 — Practical Monoids

### Merge configs

Đây là nơi Monoid tỏa sáng trong thực tế. Bạn có configs từ nhiều nguồn: defaults, environment variables, CLI flags. Cần merge chúng theo thứ tự ưu tiên. Đó chính xác là `concatAll(monoidConfig)([defaults, envConfig, cliConfig])`. Mỗi nguồn là một "khối Lego", ghép lại thành config hoàn chỉnh.

```typescript
// filename: src/monoid_config.ts
import assert from "node:assert/strict";

type Monoid<A> = { readonly concat: (x: A, y: A) => A; readonly empty: A };
const concatAll = <A>(M: Monoid<A>) => (values: readonly A[]): A =>
    values.reduce(M.concat, M.empty);

// Config merging = Monoid!
type AppConfig = {
    readonly port: number;
    readonly host: string;
    readonly debug: boolean;
    readonly features: readonly string[];
};

// "Last wins" merge for primitives, array concat for lists
const monoidConfig: Monoid<AppConfig> = {
    empty: { port: 3000, host: "localhost", debug: false, features: [] },
    concat: (base, override) => ({
        port: override.port !== monoidConfig.empty.port ? override.port : base.port,
        host: override.host !== monoidConfig.empty.host ? override.host : base.host,
        debug: override.debug || base.debug,        // OR: any true → true
        features: [...base.features, ...override.features],  // combine arrays
    }),
};

// Layer configs: defaults → env → cli
const defaults = monoidConfig.empty;
const envConfig: AppConfig = { port: 8080, host: "localhost", debug: false, features: ["auth"] };
const cliConfig: AppConfig = { port: 3000, host: "0.0.0.0", debug: true, features: ["logging"] };

const finalConfig = concatAll(monoidConfig)([defaults, envConfig, cliConfig]);

assert.strictEqual(finalConfig.port, 8080);        // env overrode
assert.strictEqual(finalConfig.host, "0.0.0.0");   // cli overrode
assert.strictEqual(finalConfig.debug, true);        // any true → true
assert.deepStrictEqual(finalConfig.features, ["auth", "logging"]);  // combined

console.log("Config monoid OK ✅");
```

Đọc lại: `concatAll(monoidConfig)([defaults, envConfig, cliConfig])` — một dòng. Thêm nguồn config mới? Chỉ thêm vào array. Không sửa logic merge. Đó là sức mạnh của Monoid: chỉ định nghĩa `concat` và `empty` MỘT LẦN, dùng với BẤT KỲ số lượng items.

### Aggregate reports

Tương tự, báo cáo doanh số (SalesReport) cũng là Monoid. Báo cáo ngày 1 + ngày 2 + ngày 3 = báo cáo tuần. Báo cáo tuần 1-4 = báo cáo tháng. Pattern GIỐNG nhau — chỉ khác Monoid instance.

```typescript
// filename: src/monoid_reports.ts
import assert from "node:assert/strict";

type Monoid<A> = { readonly concat: (x: A, y: A) => A; readonly empty: A };
const concatAll = <A>(M: Monoid<A>) => (values: readonly A[]): A =>
    values.reduce(M.concat, M.empty);

type SalesReport = {
    readonly totalRevenue: number;
    readonly orderCount: number;
    readonly uniqueProducts: readonly string[];
    readonly topCategory: string;
};

const monoidReport: Monoid<SalesReport> = {
    empty: { totalRevenue: 0, orderCount: 0, uniqueProducts: [], topCategory: "" },
    concat: (a, b) => ({
        totalRevenue: a.totalRevenue + b.totalRevenue,
        orderCount: a.orderCount + b.orderCount,
        uniqueProducts: [...new Set([...a.uniqueProducts, ...b.uniqueProducts])],
        topCategory: a.totalRevenue >= b.totalRevenue ? a.topCategory : b.topCategory,
    }),
};

const daily: SalesReport[] = [
    { totalRevenue: 5000000, orderCount: 10, uniqueProducts: ["Laptop", "Mouse"], topCategory: "electronics" },
    { totalRevenue: 3000000, orderCount: 8, uniqueProducts: ["Mouse", "Keyboard"], topCategory: "peripherals" },
    { totalRevenue: 7000000, orderCount: 15, uniqueProducts: ["Monitor"], topCategory: "electronics" },
];

const weekly = concatAll(monoidReport)(daily);
assert.strictEqual(weekly.totalRevenue, 15000000);
assert.strictEqual(weekly.orderCount, 33);
assert.deepStrictEqual([...weekly.uniqueProducts].sort(), ["Keyboard", "Laptop", "Monitor", "Mouse"]);

console.log("Report monoid OK ✅");
```

> **💡 "Anywhere you combine things → Monoid"**: merge configs, aggregate reports, combine permissions, union sets, intersect filters. Pattern is UNIVERSAL.

---

## ✅ Checkpoint 29.3

> Đến đây bạn phải hiểu:
> 1. **Config merge** = Monoid (last-wins + array concat)
> 2. **Report aggregation** = Monoid (sum numbers, union sets)
> 3. **`concatAll(monoid)(items)`** = reduce with explicit algebra
> 4. **Monoid = reusable "how to combine"** strategy
>
> **Test nhanh**: Permission system: `Admin | Editor | Viewer` — highest wins. Monoid?
> <details><summary>Đáp án</summary>`concat = max(a, b)`, `empty = "viewer"`. `concatAll(monoidPermission)(userPermissions)` → highest permission. Yes, it's a Monoid!</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Implement `monoidMin` cho numbers.

<details><summary>✅ Lời giải</summary>

```typescript
import assert from "node:assert/strict";
type Monoid<A> = { concat: (x: A, y: A) => A; empty: A };
const concatAll = <A>(M: Monoid<A>) => (vs: readonly A[]): A => vs.reduce(M.concat, M.empty);

const monoidMin: Monoid<number> = { concat: Math.min, empty: Infinity };
assert.strictEqual(concatAll(monoidMin)([5, 3, 8, 1, 7]), 1);
assert.strictEqual(concatAll(monoidMin)([]), Infinity);
```

</details>

**Bài 2** (10 phút): Monoid cho `Permissions = { canRead: boolean; canWrite: boolean; canDelete: boolean }`.

<details><summary>✅ Lời giải</summary>

```typescript
import assert from "node:assert/strict";
type Monoid<A> = { concat: (x: A, y: A) => A; empty: A };
const concatAll = <A>(M: Monoid<A>) => (vs: readonly A[]): A => vs.reduce(M.concat, M.empty);

type Permissions = { canRead: boolean; canWrite: boolean; canDelete: boolean };
const monoidPermissions: Monoid<Permissions> = {
    empty: { canRead: false, canWrite: false, canDelete: false },
    concat: (a, b) => ({
        canRead: a.canRead || b.canRead,
        canWrite: a.canWrite || b.canWrite,
        canDelete: a.canDelete || b.canDelete,
    }),
};

const userPerms = concatAll(monoidPermissions)([
    { canRead: true, canWrite: false, canDelete: false },  // viewer role
    { canRead: false, canWrite: true, canDelete: false },  // editor role
]);
assert.deepStrictEqual(userPerms, { canRead: true, canWrite: true, canDelete: false });
```

</details>

---

## 29.4 — Khi nào dùng Monoid?

Hãy nghĩ về Monoid như một "bộ công cụ ghép". Bất cứ khi nào bạn thấy mình viết `reduce()`, hãy tự hỏi: "Đây có phải Monoid không?"

**Dấu hiệu nhận biết Monoid trong code:**
- `array.reduce((a, b) => merge(a, b), empty)` — đây là `concatAll`!
- `{...defaults, ...config1, ...config2}` — config Monoid!
- `sum += value` trong loop — number addition Monoid!
- `results.push(...partial)` — array concat Monoid!
- `errors.concat(moreErrors)` — error collection Monoid!

**Monoid + Parallelism**: Vì associativity, bạn có thể cắt danh sách thành chunks, fold mỗi chunk SONG SONG, rồi fold các kết quả. Đây là nền tảng MapReduce (Google): Map = transform mỗi item. Reduce = fold với Monoid. Entire big data ecosystem build trên Monoid.

**Monoid trong thực tế hàng ngày**:
- React: `className={[base, variant, size].filter(Boolean).join(' ')}` — string Monoid
- Redux: `combineReducers` — reducer Monoid  
- CSS: cascade = style Monoid (concat = override properties, empty = `{}`)
- Git: merge = commit Monoid (commit ⊕ commit = merged commit... roughly)

> **💡 Intuition**: Nếu bạn có thể "gộp" hai thứ thành một thứ cùng loại — và có "phần tử không" — bạn có Monoid. Đặt tên cho nó, viết `concat` và `empty`, dùng `concatAll` mọi nơi.

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| `concat` không associative | Implementation sai | Test: `concat(concat(a,b),c) === concat(a,concat(b,c))` |
| `empty` không trung tính | Sai giá trị empty | Test: `concat(a, empty) === a` và `concat(empty, a) === a` |
| `concatAll([])` trả error | Không xử lý empty list | Trả `empty` cho danh sách rỗng |
| Average sai khi merge | Không dùng weighted average | Lưu sum và count riêng, tính avg khi cần |
| Non-commutative surprise | `concat(a,b) ≠ concat(b,a)` | Monoid chỉ cần associative, KHÔNG cần commutative |

---

## Tóm tắt

Monoid là một trong những abstractions PHỔ BIẾN nhất trong lập trình — và bạn đã dùng nó mỗi ngày với `+`, `.concat()`, và `{...a, ...b}`. Giờ bạn có TÊN và INTERFACE để dùng nó có hệ thống.

- ✅ **Semigroup** = `concat` (associative). Combine two things.
- ✅ **Monoid** = Semigroup + `empty`. Identity element.
- ✅ **`concatAll`** = fold with Monoid. `reduce()` abstracted.
- ✅ **Practical**: merge configs, aggregate reports, combine permissions.
- ✅ **Universal pattern**: anywhere you combine things → Monoid.

## Tiếp theo

→ Chapter 30: **Traverse & Sequence** — flip containers. `Array<Option<A>>` → `Option<Array<A>>`. Và `Promise.all` chính là `sequence`!
