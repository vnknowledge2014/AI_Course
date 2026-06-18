# Chapter 30 — Traverse & Sequence

> **Bạn sẽ học được**:
> - Vấn đề: `Array<Option<A>>` → muốn `Option<Array<A>>`
> - `sequence` = flip containers: outer ↔ inner
> - `traverse` = map + sequence in one step
> - Practical: validate array of items, fetch multiple resources, batch operations
>
> **Yêu cầu trước**: Chapter 26-28 (Functor, Monad, Applicative).
> **Thời gian đọc**: ~35 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Flip containers — biến danh sách "maybe-values" thành "maybe-danh sách".

---

Bạn có danh sách bạn bè mời đi ăn. Mỗi người trả lời: "Có" (Some) hoặc "Không" (None). Bạn có `Array<Option<"yes">>`. Nhưng bạn muốn biết: **TẤT CẢ có đi không?** Nếu AI CŨNG Some → `Some(Array<"yes">)`. Nếu BẤT KỲ ai None → `None`. Bạn cần flip: `Array<Option<A>>` → `Option<Array<A>>`.

Đây là vấn đề xuất hiện RẤT THƯỜNG: validate một danh sách items (mỗi item có thể fail), fetch nhiều resources (mỗi fetch có thể fail), parse một array strings thành numbers (mỗi parse có thể fail). Bạn có `Array<Maybe<T>>` nhưng muốn `Maybe<Array<T>>` — hoặc tất cả đều OK, hoặc fail hẳn.

Và bạn đã dùng nó suốt đời: `Promise.all([p1, p2, p3])` = `Array<Promise<A>>` → `Promise<Array<A>>`. Đó chính xác là `sequence` cho Promise!

---

## Traverse & Sequence — Đảo ngược cấu trúc

Bạn có `Array<Option<A>>` nhưng cần `Option<Array<A>>` — "nếu tất cả đều Some, gom thành một Option chứa Array. Nếu bất kỳ nào None, trả None". Đây là `sequence`.

`traverse` là tổng quát hơn: apply function trả về `Option`/`Either` cho mỗi element, rồi "đảo" cấu trúc. Pattern này xuất hiện ở mọi nơi: validate danh sách items, fetch nhiều resources, parse array of strings thành array of numbers.


## 30.1 — The Flip Problem

Hãy xem vấn đề cụ thể. Bạn có array của Option values — và muốn biết: tất cả có thành công không? Nếu có → gộp kết quả. Nếu không → fail.

```typescript
// filename: src/flip_problem.ts
import assert from "node:assert/strict";

type Option<A> = { _tag: "None" } | { _tag: "Some"; value: A };
const none: Option<never> = { _tag: "None" };
const some = <A>(v: A): Option<A> => ({ _tag: "Some", value: v });

// Có: Array<Option<number>>
const parsed: Option<number>[] = [some(1), some(2), some(3)];
const withNone: Option<number>[] = [some(1), none, some(3)];

// Muốn: Option<Array<number>>
// parsed → some([1, 2, 3])   ← all Some → wrap array in Some
// withNone → none             ← any None → whole thing is None

// ❌ Manual: verbose and error-prone
const sequenceManual = <A>(options: readonly Option<A>[]): Option<readonly A[]> => {
    const result: A[] = [];
    for (const opt of options) {
        if (opt._tag === "None") return none;
        result.push(opt.value);
    }
    return some(result);
};

assert.deepStrictEqual(sequenceManual(parsed), some([1, 2, 3]));
assert.deepStrictEqual(sequenceManual(withNone), none);

console.log("Flip problem OK ✅");
```

---

## 30.2 — `sequence`: Flip Containers

`sequence` là hàm flip: `Array<F<A>>` → `F<Array<A>>`. "Di chuyển" container từ BÊN TRONG ra BÊN NGOÀI. Với Option: nếu tất cả Some → Some(array). Nếu bất kỳ None → None. Với Either: nếu tất cả Right → Right(array). Nếu bất kỳ Left → Left (fail-fast). Với Promise: đó chính xác là `Promise.all`!

```typescript
// filename: src/sequence.ts
import assert from "node:assert/strict";

type Option<A> = { _tag: "None" } | { _tag: "Some"; value: A };
const none: Option<never> = { _tag: "None" };
const some = <A>(v: A): Option<A> => ({ _tag: "Some", value: v });

type Either<E, A> = { _tag: "Left"; left: E } | { _tag: "Right"; right: A };
const left = <E>(e: E): Either<E, never> => ({ _tag: "Left", left: e });
const right = <A>(a: A): Either<never, A> => ({ _tag: "Right", right: a });

// sequence for Option
const sequenceOption = <A>(options: readonly Option<A>[]): Option<readonly A[]> => {
    const result: A[] = [];
    for (const opt of options) {
        if (opt._tag === "None") return none;
        result.push(opt.value);
    }
    return some(result);
};

// sequence for Either (fail-fast)
const sequenceEither = <E, A>(eithers: readonly Either<E, A>[]): Either<E, readonly A[]> => {
    const result: A[] = [];
    for (const ea of eithers) {
        if (ea._tag === "Left") return ea;  // fail-fast!
        result.push(ea.right);
    }
    return right(result);
};

// sequence for Promise (= Promise.all!)
const sequencePromise = <A>(promises: readonly Promise<A>[]): Promise<readonly A[]> =>
    Promise.all(promises);

// --- Tests ---
// Option
assert.deepStrictEqual(sequenceOption([some(1), some(2), some(3)]), some([1, 2, 3]));
assert.deepStrictEqual(sequenceOption([some(1), none, some(3)]), none);
assert.deepStrictEqual(sequenceOption([]), some([]));  // empty = success!

// Either
assert.deepStrictEqual(sequenceEither([right(1), right(2)]), right([1, 2]));
assert.deepStrictEqual(sequenceEither([right(1), left("err"), right(3)]), left("err"));

console.log("sequence OK ✅");
```

> **💡 `Promise.all` IS sequence!** `Array<Promise<A>>` → `Promise<Array<A>>`. You've used it forever. sequence = generalized Promise.all for ANY container.

---

## ✅ Checkpoint 30.1-30.2

> Đến đây bạn phải hiểu:
> 1. **Flip problem**: `Array<F<A>>` → `F<Array<A>>`
> 2. **`sequence`**: flips outer/inner containers
> 3. **All-or-nothing**: one None/Left → whole result is None/Left
> 4. **`Promise.all` = sequence for Promises!**
>
> **Test nhanh**: `sequenceOption([])` = ?
> <details><summary>Đáp án</summary>`some([])`! Empty array — no Nones to fail. Vacuously true. Same as `Promise.all([])` → resolved with `[]`.</details>

---

## 30.3 — `traverse`: Map + Sequence in One Step

Thường bạn không có SẴN `Array<Option<A>>` — bạn có `Array<A>` và function `(A) => Option<B>`. Bạn cần `map` trước (tạo `Array<Option<B>>`), rồi `sequence` (flip thành `Option<Array<B>>`). `traverse` làm CẢ HAI trong MỘT pass — hiệu quả hơn (early exit khi gặp None/Left).

Pattern này XUẤT HIỆN Ở ĐÂU? Validate danh sách items. Parse danh sách strings. Fetch nhiều resources. Bất cứ khi nào bạn có danh sách và một function có-thể-fail.

```typescript
// filename: src/traverse.ts
import assert from "node:assert/strict";

type Option<A> = { _tag: "None" } | { _tag: "Some"; value: A };
const none: Option<never> = { _tag: "None" };
const some = <A>(v: A): Option<A> => ({ _tag: "Some", value: v });

type Either<E, A> = { _tag: "Left"; left: E } | { _tag: "Right"; right: A };
const left = <E>(e: E): Either<E, never> => ({ _tag: "Left", left: e });
const right = <A>(a: A): Either<never, A> => ({ _tag: "Right", right: a });

// traverse = map then sequence (but more efficient — one pass)
// Instead of: sequence(array.map(fn))
// Use: traverse(fn)(array)

const traverseOption = <A, B>(fn: (a: A) => Option<B>) => (as: readonly A[]): Option<readonly B[]> => {
    const result: B[] = [];
    for (const a of as) {
        const opt = fn(a);
        if (opt._tag === "None") return none;  // early exit!
        result.push(opt.value);
    }
    return some(result);
};

const traverseEither = <A, B, E>(fn: (a: A) => Either<E, B>) => (as: readonly A[]): Either<E, readonly B[]> => {
    const result: B[] = [];
    for (const a of as) {
        const ea = fn(a);
        if (ea._tag === "Left") return ea;
        result.push(ea.right);
    }
    return right(result);
};

// traversePromise = array.map + Promise.all (common pattern!)
const traversePromise = <A, B>(fn: (a: A) => Promise<B>) => (as: readonly A[]): Promise<readonly B[]> =>
    Promise.all(as.map(fn));

// --- Practical: parse array of strings to numbers ---
const parseNumber = (s: string): Option<number> => {
    const n = Number(s);
    return isNaN(n) ? none : some(n);
};

const allValid = traverseOption(parseNumber)(["1", "2", "3"]);
assert.deepStrictEqual(allValid, some([1, 2, 3]));

const hasInvalid = traverseOption(parseNumber)(["1", "abc", "3"]);
assert.deepStrictEqual(hasInvalid, none);

// --- Practical: validate array of order items ---
type ItemError = { index: number; message: string };
type RawItem = { productId: string; quantity: number };
type ValidItem = { productId: string; quantity: number; valid: true };

const validateItem = (item: RawItem, index: number): Either<ItemError, ValidItem> =>
    item.quantity <= 0
        ? left({ index, message: "Quantity must be > 0" })
        : item.productId.length === 0
            ? left({ index, message: "Product ID required" })
            : right({ ...item, valid: true as const });

const validateItems = (items: readonly RawItem[]): Either<ItemError, readonly ValidItem[]> =>
    traverseEither((item: RawItem, i?: number) => validateItem(item, items.indexOf(item)))(items);

const goodItems = validateItems([
    { productId: "P1", quantity: 2 },
    { productId: "P2", quantity: 1 },
]);
assert.strictEqual(goodItems._tag, "Right");

const badItems = validateItems([
    { productId: "P1", quantity: 2 },
    { productId: "", quantity: 1 },      // invalid!
]);
assert.strictEqual(badItems._tag, "Left");

console.log("traverse OK ✅");
```

> **💡 traverse = "for each item, apply fallible function. If ALL succeed → collect results. If ANY fails → fail."** It's the FP equivalent of a loop with early-return on error.

---

## 30.4 — Async Traverse: Batch Operations

```typescript
// filename: src/async_traverse.ts
import assert from "node:assert/strict";

type Either<E, A> = { _tag: "Left"; left: E } | { _tag: "Right"; right: A };
const left = <E>(e: E): Either<E, never> => ({ _tag: "Left", left: e });
const right = <A>(a: A): Either<never, A> => ({ _tag: "Right", right: a });

// Async traverse with Either = fetch multiple, fail on first error
const traverseAsyncEither = async <A, B, E>(
    fn: (a: A) => Promise<Either<E, B>>,
    items: readonly A[],
): Promise<Either<E, readonly B[]>> => {
    const results: B[] = [];
    for (const item of items) {
        const result = await fn(item);
        if (result._tag === "Left") return result;  // fail-fast
        results.push(result.right);
    }
    return right(results);
};

// --- Practical: fetch multiple users ---
type FetchError = { userId: string; reason: string };
type User = { id: string; name: string };

const fetchUser = async (id: string): Promise<Either<FetchError, User>> => {
    const users: Record<string, User> = {
        "U1": { id: "U1", name: "An" },
        "U2": { id: "U2", name: "Bình" },
        "U3": { id: "U3", name: "Cường" },
    };
    const user = users[id];
    return user ? right(user) : left({ userId: id, reason: "Not found" });
};

const run = async () => {
    // All found
    const allFound = await traverseAsyncEither(fetchUser, ["U1", "U2", "U3"]);
    assert.strictEqual(allFound._tag, "Right");
    if (allFound._tag === "Right") {
        assert.strictEqual(allFound.right.length, 3);
        assert.strictEqual(allFound.right[0].name, "An");
    }

    // One missing → fail
    const oneMissing = await traverseAsyncEither(fetchUser, ["U1", "U999", "U3"]);
    assert.strictEqual(oneMissing._tag, "Left");
    if (oneMissing._tag === "Left") {
        assert.strictEqual(oneMissing.left.userId, "U999");
    }

    console.log("Async traverse OK ✅");
};

run();
```

---

## ✅ Checkpoint 30.3-30.4

> Đến đây bạn phải hiểu:
> 1. **`traverse(fn)(array)`** = map + sequence in one pass. More efficient
> 2. **Sync**: `traverseOption`, `traverseEither`. **Async**: `traverseAsyncEither`
> 3. **Promise.all(items.map(fn))** = traverse for Promises
> 4. **Pattern**: validate/fetch each item, fail on first error, collect all successes
>
> **Test nhanh**: `traverseOption(parseNumber)(["1", "abc"])` = ?
> <details><summary>Đáp án</summary>`none`! "abc" → parseNumber → none → traverse stops, returns none. If all valid → `some([1, 2, ...])`.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): sequence & traverse basics.

```typescript
// 1. sequenceOption([some(1), some(2), none]) = ?
// 2. traverseOption(x => x > 0 ? some(x) : none)([1, 2, -3]) = ?
// 3. Promise.all([fetch("a"), fetch("b")]) = sequence or traverse?
```

<details><summary>✅ Lời giải</summary>

```typescript
// 1. none — any None in array → result is None
// 2. none — -3 fails, so entire traverse fails
// 3. sequence! Array<Promise> → Promise<Array>. If you had Promise.all(ids.map(fetch)), that's traverse.
```

</details>

**Bài 2** (10 phút): Validate order items with accumulation.

<details><summary>✅ Lời giải</summary>

```typescript
import assert from "node:assert/strict";

type Validation<E, A> = { _tag: "Failure"; errors: E[] } | { _tag: "Success"; value: A };
const fail = <E>(e: E): Validation<E, never> => ({ _tag: "Failure", errors: [e] });
const ok = <A>(a: A): Validation<never, A> => ({ _tag: "Success", value: a });

type ItemError = { index: number; message: string };
type RawItem = { name: string; qty: number };

const validateItem = (item: RawItem, index: number): Validation<ItemError, RawItem> => {
    const errors: ItemError[] = [];
    if (item.name.length === 0) errors.push({ index, message: "Name required" });
    if (item.qty <= 0) errors.push({ index, message: "Qty > 0" });
    return errors.length > 0 ? { _tag: "Failure", errors } : ok(item);
};

// Traverse with accumulation (collect ALL errors across ALL items)
const traverseValidation = <A, B, E>(
    fn: (a: A, i: number) => Validation<E, B>
) => (items: readonly A[]): Validation<E, readonly B[]> => {
    const allErrors: E[] = [];
    const allValues: B[] = [];
    items.forEach((item, i) => {
        const result = fn(item, i);
        if (result._tag === "Failure") allErrors.push(...result.errors);
        else allValues.push(result.value);
    });
    return allErrors.length > 0 ? { _tag: "Failure", errors: allErrors } : ok(allValues);
};

const items: RawItem[] = [
    { name: "Laptop", qty: 1 },
    { name: "", qty: -1 },    // 2 errors
    { name: "Mouse", qty: 0 }, // 1 error
];

const result = traverseValidation(validateItem)(items);
assert.strictEqual(result._tag, "Failure");
if (result._tag === "Failure") {
    assert.strictEqual(result.errors.length, 3);  // ALL errors from ALL items!
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "sequence vs traverse?" | Dễ nhầm | sequence = flip containers. traverse = map then flip. traverse(fn) = sequence ∘ map(fn) |
| "Want ALL errors, not just first" | Using fail-fast traverse | Use Validation-based traverse (accumulation) |
| "Promise.all fails on first rejection" | Promise.all = fail-fast sequence | Use Promise.allSettled for accumulation |

---

## Tóm tắt

- ✅ **Flip problem**: `Array<Option<A>>` → `Option<Array<A>>`.
- ✅ **`sequence`** = flip containers. All-or-nothing.
- ✅ **`traverse(fn)(array)`** = map + sequence. One pass.
- ✅ **`Promise.all` IS sequence!** You've used it all along.
- ✅ **Async traverse** = fetch/validate batch. Fail-fast or accumulate.

---

## 🎉 Part V Complete!

Bạn đã hoàn thành Part V: FP Patterns. Từ fp-ts/Effect overview (Ch25), qua Functors (Ch26), Monads (Ch27), Applicatives (Ch28), Monoids (Ch29), đến Traverse (Ch30). Đây là FP theory applied to TypeScript — practical, testable, production-ready.

Hãy nhìn lại hành trình: bạn bắt đầu với `.map()` trên arrays (ai cũng biết) và kết thúc với `traverse`, `applicative validation`, và `monoid aggregation` — những patterns mạnh mẽ mà ít developer TypeScript biết tên. Bạn không chỉ dùng được chúng — bạn HIỂU tại sao chúng hoạt động và laws nào đảm bảo chúng SAFE.

## Tiếp theo

→ Part VI: **Testing & Full-Stack** — Chapter 31: TDD with TypeScript. Đưa FP patterns vào thực chiến.
