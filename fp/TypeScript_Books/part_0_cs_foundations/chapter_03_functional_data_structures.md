# Chapter 3 — Functional Data Structures

> **Bạn sẽ học được**:
> - Tại sao `{...obj}` spread là O(n) — và khi nào điều đó quan trọng
> - Structural sharing — cách Git, React, và Immer tránh sao chép thừa
> - `Map` và `Set` — khi Object và Array không đủ
> - Graph bằng `Map<string, Set<string>>` — mô hình hóa quan hệ phức tạp
>
> **Yêu cầu trước**: Chapter 0–2 (đặc biệt Big-O và `.reduce()`).
> **Thời gian đọc**: ~40 phút | **Level**: CS Foundations
> **Kết quả cuối cùng**: Chọn đúng cấu trúc dữ liệu cho mỗi bài toán — và hiểu chi phí ẩn của immutability.

---

## 3.1 — Chi phí ẩn của Immutability

### Câu chuyện: Sửa 1 chữ, photocopy cả cuốn sách

Bạn có tờ hợp đồng 100 trang. Phát hiện lỗi chính tả ở trang 42. Trong thế giới immutable (bất biến), bạn **không được** dùng bút sửa trực tiếp — phải **photocopy toàn bộ 100 trang**, sửa trang 42 trong bản mới, rồi vứt bản cũ.

Sửa 1 chữ → photocopy 100 trang. Nghe lãng phí? Đúng. Và đây chính xác là điều xảy ra khi bạn dùng spread `{...obj}` hay `[...arr]` trong TypeScript.

### Spread = Shallow copy = O(n)

```typescript
// filename: src/spread_cost.ts
import assert from "node:assert/strict";

// Object spread — sao chép TẤT CẢ fields
type User = {
    readonly name: string;
    readonly age: number;
    readonly city: string;
    readonly email: string;
};

const user: User = { name: "An", age: 25, city: "HCM", email: "an@mail.com" };

// Sửa age → phải copy TẤT CẢ fields khác
const updated = { ...user, age: 26 };
// Tạo object mới: { name: "An", age: 26, city: "HCM", email: "an@mail.com" }
// 4 fields → 4 phép copy = O(n) với n = số fields

assert.strictEqual(updated.age, 26);
assert.strictEqual(user.age, 25);  // bản gốc KHÔNG ĐỔI!

console.log(`Original: ${user.name}, ${user.age}`);
console.log(`Updated:  ${updated.name}, ${updated.age}`);
// Output:
// Original: An, 25
// Updated:  An, 26
```

```typescript
// filename: src/array_spread.ts
import assert from "node:assert/strict";

// Array spread — tệ hơn!
const numbers: readonly number[] = [1, 2, 3, 4, 5];

// Thêm 1 phần tử cuối → copy tất cả + thêm 1 = O(n)
const withSix = [...numbers, 6];
assert.deepStrictEqual(withSix, [1, 2, 3, 4, 5, 6]);

// Sửa phần tử thứ 2 → copy tất cả, đổi 1 = O(n)
const changed = numbers.map((x, i) => i === 2 ? 99 : x);
assert.deepStrictEqual(changed, [1, 2, 99, 4, 5]);

// "Xóa" phần tử → filter = O(n)
const without3 = numbers.filter(x => x !== 3);
assert.deepStrictEqual(without3, [1, 2, 4, 5]);

console.log("Immutable arrays: every update is O(n)");
// Output: Immutable arrays: every update is O(n)
```

### Bảng chi phí

| Thao tác | Mutable (mutation) | Immutable (spread/copy) |
|----------|-------------------|-------------------------|
| Sửa 1 field trong object | O(1) | **O(n)** — copy tất cả |
| Thêm cuối array | O(1) `.push()` | **O(n)** `[...arr, x]` |
| Sửa 1 phần tử array | O(1) `arr[i] = x` | **O(n)** `.map()` |
| Xóa 1 phần tử | O(n) `.splice()` | **O(n)** `.filter()` |

> **💡 Vậy tại sao vẫn dùng immutable?** Vì bugs từ mutation **đắt hơn nhiều** so với O(n) copy. Một cái race condition trong React vì mutation có thể gây hàng tuần debug. O(n) copy = giá nhỏ để trả cho **sự an toàn**. Và khi data thật sự lớn → structural sharing giải quyết.

### Lồng sâu = Đau đớn

Spread chỉ sao chép **1 tầng** (shallow copy). Object lồng sâu? Mỗi tầng phải spread riêng:

```typescript
// filename: src/nested_pain.ts

type Address = { readonly city: string; readonly zip: string };
type Company = { readonly name: string; readonly address: Address };
type Employee = { readonly name: string; readonly company: Company };

const emp: Employee = {
    name: "An",
    company: {
        name: "Startup X",
        address: { city: "HCM", zip: "700000" }
    }
};

// Sửa city → phải spread 3 tầng!
const moved: Employee = {
    ...emp,
    company: {
        ...emp.company,
        address: {
            ...emp.company.address,
            city: "Hà Nội"
        }
    }
};

console.log(`Before: ${emp.company.address.city}`);
console.log(`After:  ${moved.company.address.city}`);
// Output:
// Before: HCM
// After:  Hà Nội

// 3 tầng spread = 3× copy = dễ sai, dài dòng, khó đọc
```

Vấn đề này nghiêm trọng đến mức có thư viện riêng để giải quyết → Section 3.2.

---

## ✅ Checkpoint 3.1

> Đến đây bạn phải hiểu:
> 1. **Spread `{...obj}` và `[...arr]`** = shallow copy = **O(n)**
> 2. Spread chỉ copy **1 tầng** — nested objects cần spread nhiều lần
> 3. Immutability vẫn đáng dùng vì **bugs từ mutation đắt hơn** chi phí copy
> 4. Data lớn hoặc lồng sâu → cần giải pháp tốt hơn spread
>
> **Test nhanh**: Object có 1.000 fields. Bạn sửa 1 field bằng spread. Bao nhiêu fields bị copy?
> <details><summary>Đáp án</summary>1.000! Spread copy TẤT CẢ fields rồi ghi đè field bạn sửa. Đó là tại sao object lớn + spread = chậm.</details>

---

## 3.2 — Structural Sharing: Chia sẻ thay vì sao chép

### Ẩn dụ: Git — chỉ lưu thay đổi

Bạn dùng Git hàng ngày. Khi commit, Git **không** copy toàn bộ project. Nó chỉ lưu **files thay đổi** — các files không đổi được **chia sẻ** (shared) giữa các commits.

```
Commit 1: [file_A] [file_B] [file_C]
                ↓         ↓         ↓
Commit 2: [file_A] [file_B'] [file_C]   ← chỉ B thay đổi
               ↑                    ↑
               └── shared ──────────┘     ← A và C không copy!
```

Đây là **structural sharing** — ý tưởng cốt lõi đằng sau mọi persistent data structure.

### Immer — "Viết mutation, nhận immutable"

[Immer](https://immerjs.github.io/immer/) là thư viện phổ biến nhất cho immutable updates trong TypeScript. Ý tưởng thiên tài: bạn **viết code như đang mutate**, nhưng Immer tạo **bản mới** phía sau.

```typescript
// filename: src/immer_example.ts
// npm install immer
import { produce } from "immer";
import assert from "node:assert/strict";

// Chú ý: Immer cần types KHÔNG có readonly — vì draft phải "mutable"
// Đây là trade-off thiết kế: type mutable, nhưng kết quả thực tế immutable
type Address = { city: string; zip: string };
type Company = { name: string; address: Address };
type Employee = { name: string; company: Company };

const emp: Employee = {
    name: "An",
    company: {
        name: "Startup X",
        address: { city: "HCM", zip: "700000" }
    }
};

// ✅ Immer: viết mutation → nhận immutable bản mới
const moved = produce(emp, draft => {
    draft.company.address.city = "Hà Nội";
});

// Bản gốc KHÔNG ĐỔI
assert.strictEqual(emp.company.address.city, "HCM");
assert.strictEqual(moved.company.address.city, "Hà Nội");

// Structural sharing: company.name không đổi → SHARED!
assert.strictEqual(emp.name, moved.name);  // cùng reference

console.log(`Before: ${emp.company.address.city}`);
console.log(`After:  ${moved.company.address.city}`);
// Output:
// Before: HCM
// After:  Hà Nội
```

So sánh:

```typescript
// ❌ Spread thủ công — 3 tầng, dài dòng, dễ sai
const moved1: Employee = {
    ...emp,
    company: {
        ...emp.company,
        address: { ...emp.company.address, city: "Hà Nội" }
    }
};

// ✅ Immer — 1 dòng, đọc như mutation, kết quả immutable
const moved2 = produce(emp, draft => {
    draft.company.address.city = "Hà Nội";
});
```

> **💡 Immer dùng Proxy**: Khi bạn "mutate" `draft`, Immer theo dõi thay đổi qua JavaScript Proxy, rồi tạo bản mới với structural sharing. Parts không đổi được chia sẻ — không copy thừa.

### HAMT — Cấu trúc persistent chuyên dụng

Ngoài Immer, có thư viện dùng **Hash Array Mapped Trie (HAMT)** — cấu trúc dữ liệu chuyên cho immutable collections:

| Thư viện | Mô tả | Update |
|----------|-------|--------|
| Spread `{...obj}` | Copy toàn bộ | O(n) |
| **Immer** | Proxy + structural sharing | O(changed fields) |
| **immutable-js** | HAMT collections (List, Map, Set) | O(log₃₂ n) |

`immutable-js` cho Big-O tốt hơn spread cho collections lớn, nhưng có trade-off: API riêng (không dùng `.map()`, `.filter()` chuẩn), và phải convert qua lại với JS objects.

> **💡 Quy tắc chọn**:
> - Object nhỏ (< 20 fields): spread `{...obj}` là đủ
> - Object lồng sâu: **Immer** — API đẹp, zero learning curve
> - Collection rất lớn (10K+ items, update thường xuyên): xem xét **immutable-js**

---

## ✅ Checkpoint 3.2

> Đến đây bạn phải hiểu:
> 1. **Structural sharing** = chia sẻ phần không đổi, chỉ copy phần thay đổi (giống Git)
> 2. **Immer** = viết code mutation → nhận bản immutable mới. Dùng Proxy
> 3. Spread = O(n). Immer = O(changed). HAMT = O(log n)
> 4. Immer là lựa chọn tốt nhất cho **hầu hết** use cases trong TypeScript
>
> **Test nhanh**: Object 1.000 fields, bạn sửa 1 field. Spread copy bao nhiêu? Immer copy bao nhiêu?
> <details><summary>Đáp án</summary>Spread: 1.000 fields. Immer: chỉ path đến field thay đổi (structural sharing) — tốt hơn nhiều.</details>

---

## 3.3 — `Map` và `Set`: Khi Object không đủ

### Object ≠ Dictionary

JavaScript `Object` thường bị dùng làm dictionary (key-value store). Nhưng nó có **nhiều bẫy**:

```typescript
// filename: src/object_traps.ts

// ❌ Object làm dictionary — các bẫy
const scores: Record<string, number> = {};

// (mutation — chỉ dùng ở đây để demo bẫy, không phải pattern tốt)
scores["Alice"] = 95;
scores["Bob"] = 87;

// Bẫy 1: key luôn là string
// scores[42] → tự chuyển thành scores["42"]

// Bẫy 2: prototype pollution
// "toString", "constructor", "hasOwnProperty" là keys sẵn có

// Bẫy 3: No size property
// Object.keys(scores).length → O(n) mỗi lần hỏi size!

// Bẫy 4: Thứ tự key không đảm bảo cho mọi trường hợp
```

### `Map` — Dictionary đúng cách

```typescript
// filename: src/map_basics.ts
import assert from "node:assert/strict";

// ✅ Map — dictionary chính thống
const scores: ReadonlyMap<string, number> = new Map([
    ["An", 95],
    ["Bình", 87],
    ["Cường", 92],
]);

// Truy cập — O(1) trung bình
assert.strictEqual(scores.get("An"), 95);
assert.strictEqual(scores.get("Unknown"), undefined);  // key không tồn tại

// Size — O(1)!
assert.strictEqual(scores.size, 3);

// Kiểm tra key — O(1)
assert.strictEqual(scores.has("Bình"), true);

// Duyệt theo thứ tự chèn
for (const [name, score] of scores) {
    console.log(`${name}: ${score}`);
}
// Output:
// An: 95
// Bình: 87
// Cường: 92
```

### Immutable Map operations

`ReadonlyMap` ngăn mutation. Để "update", tạo Map mới:

```typescript
// filename: src/map_immutable.ts
import assert from "node:assert/strict";

const original = new Map([["An", 95], ["Bình", 87]]);

// "Thêm" key — tạo Map mới
const withNew = new Map(original);
withNew.set("Cường", 92);

// "Sửa" value — tạo Map mới
const updated = new Map(original);
updated.set("An", 100);

// Original KHÔNG ĐỔI
assert.strictEqual(original.get("An"), 95);
assert.strictEqual(original.size, 2);
assert.strictEqual(updated.get("An"), 100);
assert.strictEqual(withNew.size, 3);

console.log(`Original size: ${original.size}`);
console.log(`With new: ${withNew.size}`);
// Output:
// Original size: 2
// With new: 3
```

### `Set` — Tập hợp không trùng

```typescript
// filename: src/set_basics.ts
import assert from "node:assert/strict";

// Set = tập hợp không trùng lặp
const tags: ReadonlySet<string> = new Set(["typescript", "fp", "ddd"]);

// Kiểm tra — O(1)
assert.strictEqual(tags.has("fp"), true);
assert.strictEqual(tags.has("java"), false);

// Size — O(1)
assert.strictEqual(tags.size, 3);

// Loại trùng từ array
const dupes = [1, 2, 3, 2, 1, 4, 3, 5];
const unique = [...new Set(dupes)];
assert.deepStrictEqual(unique, [1, 2, 3, 4, 5]);

console.log(`Unique: ${JSON.stringify(unique)}`);
// Output: Unique: [1,2,3,4,5]
```

### Bảng so sánh: Object vs Map vs Set

| | Object | Map | Set |
|---|---|---|---|
| Key types | string, symbol | **bất kỳ** | — (values) |
| `.size` | O(n) | **O(1)** | **O(1)** |
| Lookup | O(1) | O(1) | O(1) |
| Thứ tự | không đảm bảo | **insertion order** | **insertion order** |
| Prototype pollution | ⚠️ có | ❌ không | ❌ không |
| Dùng khi | literal config | **dynamic key-value** | **unique items** |

> **💡 Quy tắc**: `Object` cho cấu hình tĩnh (type đã biết). `Map` cho dữ liệu động (key từ user, API). `Set` cho danh sách unique.

---

## ✅ Checkpoint 3.3

> Đến đây bạn phải hiểu:
> 1. **Map** = dictionary đúng cách. Key bất kỳ, `.size` O(1), insertion order
> 2. **Set** = tập hợp không trùng. `.has()` O(1), loại trùng bằng `new Set(arr)`
> 3. **ReadonlyMap** / **ReadonlySet** cho immutability — ngăn `.set()`, `.delete()`
> 4. **Object** chỉ cho cấu hình tĩnh — KHÔNG dùng làm dictionary động
>
> **Test nhanh**: Cách nhanh nhất loại trùng từ `[3, 1, 4, 1, 5, 9, 2, 6, 5]`?
> <details><summary>Đáp án</summary>`[...new Set([3, 1, 4, 1, 5, 9, 2, 6, 5])]` → `[3, 1, 4, 5, 9, 2, 6]`. One-liner!</details>

---

## 3.4 — Graph: Mô hình hóa quan hệ

### Câu chuyện: Mạng xã hội

An follow Bình. Bình follow Cường. Cường follow An. Đây là **graph** — nodes (người) nối nhau bằng edges (follow).

```
An → Bình → Cường
↑                ↓
└────────────────┘
```

Graph xuất hiện ở mọi nơi: mạng xã hội, dependency tree (`npm`), routing, recommendation systems.

### Graph bằng `Map` + `Set`

TypeScript không có kiểu Graph built-in. Cách tự nhiên nhất: `Map<string, Set<string>>` — mỗi node map tới tập các nodes nó kết nối:

```typescript
// filename: src/graph.ts
import assert from "node:assert/strict";

// Directed graph — "ai follow ai"
type Graph = ReadonlyMap<string, ReadonlySet<string>>;

// Xây graph từ danh sách edges
const buildGraph = (edges: readonly (readonly [string, string])[]): Graph => {
    const graph = new Map<string, Set<string>>();

    for (const [from, to] of edges) {
        if (!graph.has(from)) graph.set(from, new Set());
        graph.get(from)!.add(to);

        // Đảm bảo node "to" cũng có mặt (dù chưa follow ai)
        if (!graph.has(to)) graph.set(to, new Set());
    }

    return graph;
};

const socialGraph = buildGraph([
    ["An", "Bình"],
    ["An", "Cường"],
    ["Bình", "Cường"],
    ["Cường", "An"],
]);

// Queries — O(1) mỗi query!
const anFollows = socialGraph.get("An")!;
assert.strictEqual(anFollows.has("Bình"), true);
assert.strictEqual(anFollows.has("Cường"), true);
assert.strictEqual(anFollows.size, 2);

console.log("An follows:", [...anFollows].join(", "));
console.log("Bình follows:", [...socialGraph.get("Bình")!].join(", "));
// Output:
// An follows: Bình, Cường
// Bình follows: Cường
```

### Duyệt Graph: BFS

Tìm tất cả người mà An có thể "reach" (trực tiếp hoặc gián tiếp):

```typescript
// filename: src/graph_bfs.ts
import assert from "node:assert/strict";

type Graph = ReadonlyMap<string, ReadonlySet<string>>;

// BFS — duyệt theo chiều rộng
const reachable = (graph: Graph, start: string): ReadonlySet<string> => {
    const visited = new Set<string>();
    const queue: string[] = [start];

    while (queue.length > 0) {
        const current = queue.shift()!;           // O(n) — OK cho demo
        if (visited.has(current)) continue;
        visited.add(current);

        const neighbors = graph.get(current);
        if (neighbors) {
            for (const neighbor of neighbors) {
                if (!visited.has(neighbor)) {
                    queue.push(neighbor);
                }
            }
        }
    }

    visited.delete(start);  // bỏ chính mình
    return visited;
};

// Dùng lại socialGraph từ trên
const buildGraph = (edges: readonly (readonly [string, string])[]): Graph => {
    const graph = new Map<string, Set<string>>();
    for (const [from, to] of edges) {
        if (!graph.has(from)) graph.set(from, new Set());
        graph.get(from)!.add(to);
        if (!graph.has(to)) graph.set(to, new Set());
    }
    return graph;
};

const socialGraph = buildGraph([
    ["An", "Bình"],
    ["An", "Cường"],
    ["Bình", "Cường"],
    ["Cường", "An"],
    ["Dũng", "An"],
]);

const anReaches = reachable(socialGraph, "An");
console.log("An can reach:", [...anReaches].join(", "));
// Output: An can reach: Bình, Cường

const dungReaches = reachable(socialGraph, "Dũng");
console.log("Dũng can reach:", [...dungReaches].join(", "));
// Output: Dũng can reach: An, Bình, Cường
```

> **💡 Tại sao BFS dùng mutation?** `queue` và `visited` là mutable — vì BFS là thuật toán inherently stateful. Trong FP thuần túy, bạn dùng recursion + accumulator. Nhưng trong TypeScript thực tế, mutable local state bên trong function **hoàn toàn OK** — miễn function tổng thể là pure (cùng input → cùng output, không side effects bên ngoài).

### Bảng Big-O cho Graph operations

| Thao tác | `Map<string, Set<string>>` |
|----------|---------------------------|
| Thêm edge | O(1) |
| Kiểm tra edge tồn tại | O(1) |
| Lấy neighbors | O(1) |
| BFS/DFS toàn graph | O(V + E) |
| Tìm shortest path | O(V + E) BFS |

V = số nodes, E = số edges.

---

## ✅ Checkpoint 3.4

> Đến đây bạn phải hiểu:
> 1. **Graph** = nodes + edges. Biểu diễn bằng `Map<string, Set<string>>`
> 2. **Adjacency list** (Map+Set) cho O(1) lookup edges và neighbors
> 3. **BFS** = duyệt theo chiều rộng = O(V + E). Dùng queue
> 4. Mutable local state trong function pure = **OK** (BFS pattern)
>
> **Test nhanh**: Graph có 5 nodes, mỗi node follow 2 nodes khác. Bao nhiêu edges?
> <details><summary>Đáp án</summary>5 × 2 = 10 edges. Mỗi node đóng góp 2 edges (directed graph).</details>


## 🏋️ Bài tập

**Bài 1** (5 phút): Immutable update

Viết function `setPort` — update port trong nested config KHÔNG dùng Immer (chỉ spread):

```typescript
type Config = {
    readonly server: {
        readonly host: string;
        readonly port: number;
    };
    readonly debug: boolean;
};

const config: Config = {
    server: { host: "localhost", port: 3000 },
    debug: false
};

// Viết function: setPort(config, 8080) → config mới với port = 8080
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
const setPort = (config: Config, port: number): Config => ({
    ...config,
    server: {
        ...config.server,
        port
    }
});

const updated = setPort(config, 8080);
// updated.server.port === 8080
// config.server.port === 3000 (không đổi!)
```

</details>

---

**Bài 2** (10 phút): Frequency counter bằng `.reduce()` + `Map`

Viết function `frequency` đếm số lần xuất hiện mỗi phần tử:

```typescript
// frequency(["a", "b", "a", "c", "b", "a"])
// → Map { "a" => 3, "b" => 2, "c" => 1 }
```

<details><summary>💡 Gợi ý</summary>Dùng `.reduce()` với accumulator là `Map<string, number>`. Mỗi bước: lấy count hiện tại (mặc định 0), cộng 1.</details>

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

// Cách 1: Immutable thuần — tạo Map mới mỗi bước (O(n²) tổng, OK cho n nhỏ)
const frequencyPure = (items: readonly string[]): ReadonlyMap<string, number> =>
    items.reduce((acc, item) => {
        const map = new Map(acc);
        map.set(item, (map.get(item) ?? 0) + 1);
        return map;
    }, new Map<string, number>());

// Cách 2: Mutation nội bộ — O(n), thực tế hơn (giống BFS pattern)
const frequency = (items: readonly string[]): ReadonlyMap<string, number> => {
    const map = new Map<string, number>();
    for (const item of items) {
        map.set(item, (map.get(item) ?? 0) + 1);
    }
    return map;  // trả ReadonlyMap — bên ngoài không mutate được
};

const result = frequency(["a", "b", "a", "c", "b", "a"]);
assert.strictEqual(result.get("a"), 3);
assert.strictEqual(result.get("b"), 2);
assert.strictEqual(result.get("c"), 1);
```

</details>

---

**Bài 3** (15 phút): Graph — tìm mutual followers

Cho graph mạng xã hội, tìm tất cả cặp **(A, B)** mà A follow B **VÀ** B follow A:

```typescript
// Input: buildGraph([["An","Bình"], ["Bình","An"], ["An","Cường"], ["Cường","Dũng"]])
// Output: [["An", "Bình"]]  (An↔Bình follow lẫn nhau)
```

<details><summary>💡 Gợi ý</summary>Duyệt tất cả edges. Với mỗi edge (A→B), kiểm tra B→A có tồn tại không. Tránh duplicate bằng cách chỉ giữ cặp A < B (theo ABC).</details>

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type Graph = ReadonlyMap<string, ReadonlySet<string>>;

const mutualFollowers = (graph: Graph): readonly (readonly [string, string])[] => {
    const result: [string, string][] = [];

    for (const [person, follows] of graph) {
        for (const target of follows) {
            // Kiểm tra follow ngược
            const targetFollows = graph.get(target);
            if (targetFollows?.has(person) && person < target) {
                result.push([person, target]);
            }
        }
    }

    return result;
};

const buildGraph = (edges: readonly (readonly [string, string])[]): Graph => {
    const graph = new Map<string, Set<string>>();
    for (const [from, to] of edges) {
        if (!graph.has(from)) graph.set(from, new Set());
        graph.get(from)!.add(to);
        if (!graph.has(to)) graph.set(to, new Set());
    }
    return graph;
};

const graph = buildGraph([
    ["An", "Bình"], ["Bình", "An"],
    ["An", "Cường"], ["Cường", "Dũng"],
]);

const mutuals = mutualFollowers(graph);
assert.deepStrictEqual(mutuals, [["An", "Bình"]]);
console.log("Mutual followers:", mutuals);
// Output: Mutual followers: [ [ 'An', 'Bình' ] ]
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Spread sửa object nhưng nested object cũng đổi theo | Shallow copy — nested objects vẫn shared reference | Spread từng tầng, hoặc dùng **Immer** |
| `Map.get()` trả `undefined` | Key không tồn tại | Luôn kiểm tra `.has()` trước, hoặc dùng `??` fallback |
| `Set` không loại trùng objects | `Set` dùng reference equality cho objects | Dùng `Set` với primitives, hoặc serialize thành string key |
| Graph thiếu nodes | Chỉ thêm nodes có edges | Thêm cả node "đích" khi xây graph |
| TS compiler vẫn cho `.set()` trên Map | Biến chưa khai báo kiểu `ReadonlyMap` | Khai báo `const m: ReadonlyMap<K,V>` — compiler sẽ chặn `.set()` |

---

## Tóm tắt

- ✅ **Spread `{...obj}`** = shallow copy = O(n). Nested → spread nhiều tầng = đau đớn.
- ✅ **Structural sharing** = chia sẻ phần không đổi (Git model). **Immer** = giải pháp tốt nhất cho TS.
- ✅ **Map** thay Object cho dynamic data. Key bất kỳ, `.size` O(1), insertion order.
- ✅ **Set** cho danh sách unique. `.has()` O(1). Loại trùng: `[...new Set(arr)]`.
- ✅ **Graph** = `Map<string, Set<string>>`. BFS O(V+E). Graph ở mọi nơi: social, routing, dependencies.

---

## 🎉 Part 0 Complete!

Bạn đã hoàn thành **Part 0 — CS Foundations**. Bây giờ bạn biết:

| Chapter | Kỹ năng | Ứng dụng |
|---------|---------|----------|
| **Ch0** | TypeScript syntax | Đọc hiểu mọi code trong sách |
| **Ch1** | Lambda, Types, Algebraic sizing | Thiết kế data model không bug |
| **Ch2** | Big-O, Recursion, `.reduce()` | Viết code nhanh, chọn thuật toán đúng |
| **Ch3** | Immutability costs, Map/Set, Graph | Chọn cấu trúc dữ liệu đúng |

## Tiếp theo

→ Part I: **TypeScript Fundamentals** — bắt đầu từ Chapter 4: **Getting Started** — thiết lập project thật, Node.js + npm + `tsconfig.json` chi tiết, và viết chương trình TypeScript đầu tiên có cấu trúc.
