# Chapter 32 — Property-Based Testing

> **Bạn sẽ học được**:
> - Property-Based Testing (PBT) vs Example-Based Testing
> - `fast-check` library — generate random inputs, find edge cases
> - Properties: invariants, idempotency, round-trip, oracle
> - Shrinking — tự động tìm minimal failing case
> - PBT cho domain logic: branded types, state machines, serialization
>
> **Yêu cầu trước**: Chapter 31 (TDD basics).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Test 1000+ cases tự động — phát hiện bug mà bạn không nghĩ tới.

---

Bạn nhớ game "20 câu hỏi" không? Một người nghĩ một vật, người kia hỏi 20 câu yes/no để đoán. Example-based testing = bạn ĐOÁN vật cụ thể: "Có phải con chó?" "Có phải cái bàn?". Property-based testing = bạn hỏi TÍNH CHẤT: "Nó có 4 chân?" "Nó có thể di chuyển?" Tính chất thu hẹp search space NHANH hơn đoán cụ thể.

**PBT** = thay vì viết 5 examples, bạn viết 1 property (tính chất) và library tự generate 1000+ random inputs để kiểm tra.

Tại sao mạnh hơn example testing? Vì bạn KHÔNG THỂ nghĩ ra hết edge cases. `sort([3,1,2])` → `[1,2,3]` — bạn test 5 arrays và tin code đúng. Nhưng `sort([NaN, 0, -0])`? `sort([Infinity, -Infinity])`? `sort([])`? PBT generate HÀNG NGHÌN inputs ngẫu nhiên, tìm ra bugs bạn KHÔNG NGHĨ TỚI.

Và khi PBT tìm ra bug: nó không chỉ báo "đã fail" — nó **shrink** input về kích thước NHỎ NHẤT vẫn fail. Array 100 phần tử fail? Shrink xuống 3 phần tử vẫn fail? Đó là minimal reproducing case. Đừng cơ — cho không!

---

## Property-Based Testing — Let the machine find bugs

PBT tạo RANDOM inputs, kiểm tra PROPERTIES (điều luôn đúng). 100 test cases tự động vs 5 test cases bạn nghĩ ra. Khi fail: shrinking tìm input NHỎ NHẤT gây lỗi. `fast-check` là library. Patterns: round-trip, idempotency, invariant, oracle.


## 32.1 — Example-Based vs Property-Based

Sự khác biệt cốt lõi giữa hai cách tiếp cận: example test nói "với INPUT NÀY, kết quả phải là THẾ NÀY". Property test nói "với BẤT KỲ input, TÍNH CHẤT NÀY phải đúng". Một cái test cụ thể. Cái kia test tổng quát. Hãy so sánh:

```typescript
// filename: src/pbt_why.ts
import assert from "node:assert/strict";

// Function under test
const sort = (arr: number[]): number[] => [...arr].sort((a, b) => a - b);

// ❌ Example-based: test CỤ THỂ inputs
assert.deepStrictEqual(sort([3, 1, 2]), [1, 2, 3]);
assert.deepStrictEqual(sort([]), []);
assert.deepStrictEqual(sort([1]), [1]);
// Problem: bạn ĐOÁN edge cases. Quên: [NaN]? [-0, 0]? [Infinity]?

// ✅ Property-based: test TÍNH CHẤT
// Property 1: output có cùng length với input
// Property 2: output sorted (mỗi element ≤ element sau)
// Property 3: output chứa cùng elements
// Library generate 1000 random arrays → check properties

// Simulating PBT:
const isSorted = (arr: number[]): boolean =>
    arr.every((val, i) => i === 0 || arr[i - 1] <= val);

const hasSameElements = (a: number[], b: number[]): boolean => {
    if (a.length !== b.length) return false;
    const sortedA = [...a].sort();
    const sortedB = [...b].sort();
    return sortedA.every((v, i) => v === sortedB[i]);
};

// Test 100 random arrays
for (let i = 0; i < 100; i++) {
    const len = Math.floor(Math.random() * 20);
    const input = Array.from({ length: len }, () => Math.floor(Math.random() * 200) - 100);
    const output = sort(input);

    assert.strictEqual(output.length, input.length, "Same length");
    assert.ok(isSorted(output), `Sorted: [${output}]`);
    assert.ok(hasSameElements(input, output), "Same elements");
}

console.log("PBT concept (100 random tests) OK ✅");
```

> **💡 "Don't test examples, test properties"**: Example = "sort([3,1,2]) = [1,2,3]". Property = "for ANY array, sort output is sorted AND has same elements". Properties catch cases you didn't think of!

---

## ✅ Checkpoint 32.1

> Đến đây bạn phải hiểu:
> 1. **Example-based**: test specific inputs. Risk: miss edge cases
> 2. **Property-based**: test invariants. Library generates random inputs
> 3. **Properties**: "output sorted", "same elements", "same length"
> 4. **PBT finds bugs you didn't think of**: weird inputs, boundaries, edge cases
>
> **Test nhanh**: sort([5, 5, 5]) — example test có thể miss, property test catch?
> <details><summary>Đáp án</summary>Property "same elements" catches: nếu sort giữ duplicates sai → hasSameElements fails. Example test thường chỉ test distinct values.</details>

---

## 32.2 — fast-check: Automatic PBT

`fast-check` là thư viện PBT mạnh nhất cho TypeScript. Nó có hai thành phần chính: **Arbitraries** (generators tạo random data) và **Properties** (assertions phải đúng cho MỊI generated input). Bạn khai báo: "cho tôi random integer arrays, và với mỗi array, sort phải sorted." fast-check generate 1000 arrays, kiểm tra tất cả.

Nếu fail: fast-check tự **shrink** input. Array 50 phần tử fail? fast-check thử: 25 phần tử còn fail? 10 phần tử? 3? Tìm minimal failing case. Đây là killer feature.

```typescript
// filename: src/fast_check_concept.ts
import assert from "node:assert/strict";

// fast-check API (conceptual — real fast-check has this exact API):
//
// import * as fc from 'fast-check';
//
// fc.assert(
//     fc.property(
//         fc.array(fc.integer()),    // Arbitrary: generates random int arrays
//         (arr) => {                  // Property: must return true for ALL inputs
//             const sorted = sort(arr);
//             return sorted.length === arr.length && isSorted(sorted);
//         }
//     ),
//     { numRuns: 1000 }             // Run 1000 times
// );

// Simulating fast-check Arbitraries:
type Arbitrary<A> = {
    generate: () => A;
};

const integer = (min = -1000, max = 1000): Arbitrary<number> => ({
    generate: () => Math.floor(Math.random() * (max - min + 1)) + min,
});

const string = (maxLen = 20): Arbitrary<string> => ({
    generate: () => {
        const len = Math.floor(Math.random() * maxLen);
        return Array.from({ length: len }, () =>
            String.fromCharCode(Math.floor(Math.random() * 26) + 97)
        ).join("");
    },
});

const array = <A>(arb: Arbitrary<A>, maxLen = 10): Arbitrary<A[]> => ({
    generate: () => {
        const len = Math.floor(Math.random() * maxLen);
        return Array.from({ length: len }, () => arb.generate());
    },
});

// Property runner
const check = <A>(arb: Arbitrary<A>, property: (a: A) => boolean, runs = 100): void => {
    for (let i = 0; i < runs; i++) {
        const value = arb.generate();
        if (!property(value)) {
            throw new Error(`Property failed for input: ${JSON.stringify(value)}`);
        }
    }
};

// --- Test: reverse is involution ---
const reverse = <A>(arr: readonly A[]): readonly A[] => [...arr].reverse();

check(
    array(integer()),
    (arr) => {
        const doubled = reverse(reverse(arr));
        return JSON.stringify(doubled) === JSON.stringify(arr);
    },
    200,
);

// --- Test: string concat associativity ---
check(
    string(10),
    (s) => {
        return (s + "") === s && ("" + s) === s;  // identity
    },
    200,
);

console.log("fast-check concept OK ✅");
```

---

## 32.3 — Property Patterns

### 4 loại property phổ biến

Khi bắt đầu với PBT, câu hỏi lớn nhất là: "Viết property GÌ?" Bốn patterns sau cover ~90% các trường hợp. Học chúng, và bạn sẽ thấy properties KHẮP NƠI:

1. **Round-trip** (encode/decode): `decode(encode(x)) === x`. Test serialization, parsing, encryption.
2. **Idempotency**: `f(f(x)) === f(x)`. Test normalization, formatting, deduplication.
3. **Invariant**: property luôn đúng cho MỊI input. `abs(n) >= 0`. `length >= 0`. `sorted output is sorted`.
4. **Oracle**: so sánh với implementation BIẾT ĐÚNG. `mySqrt(n)` vs `Math.sqrt(n)`.

**Câu chuyện thực tế**: QuickCheck (PBT library gốc cho Haskell) đã tìm ra bugs trong Ericsson telecom systems mà 20 năm testing truyền thống không tìm được. Volvo dùng PBT để test xe tự lái. Dropbox dùng PBT để test file sync. Nó KHÔNG chỉ là “toy technique” — nó là industrial-strength testing.

```typescript
// filename: src/property_patterns.ts
import assert from "node:assert/strict";

// === 1. ROUND-TRIP (encode/decode) ===
// encode(decode(x)) === x AND decode(encode(x)) === x

const encode = (s: string): string => Buffer.from(s).toString("base64");
const decode = (s: string): string => Buffer.from(s, "base64").toString("utf-8");

// Property: round-trip
for (let i = 0; i < 50; i++) {
    const chars = "abcdefghijklmnopqrstuvwxyz0123456789 !@#";
    const len = Math.floor(Math.random() * 30);
    const input = Array.from({ length: len }, () =>
        chars[Math.floor(Math.random() * chars.length)]
    ).join("");

    assert.strictEqual(decode(encode(input)), input, `Round-trip failed for "${input}"`);
}

// === 2. IDEMPOTENCY ===
// f(f(x)) === f(x)

const normalize = (s: string): string => s.trim().toLowerCase().replace(/\s+/g, " ");

for (let i = 0; i < 50; i++) {
    const input = "  HELLO   World  ".repeat(Math.floor(Math.random() * 3));
    assert.strictEqual(normalize(normalize(input)), normalize(input));
}

// === 3. INVARIANT ===
// Some property holds for ALL inputs

const abs = (n: number): number => Math.abs(n);
for (let i = 0; i < 100; i++) {
    const n = (Math.random() - 0.5) * 2000;
    assert.ok(abs(n) >= 0, "abs always non-negative");
    assert.strictEqual(abs(n), abs(-n), "abs(n) === abs(-n)");
}

// === 4. ORACLE (compare with known-correct implementation) ===
const mySqrt = (n: number): number => {
    if (n < 0) return NaN;
    let x = n;
    for (let i = 0; i < 100; i++) x = (x + n / x) / 2;
    return Math.round(x * 1000) / 1000;
};

for (let i = 0; i < 50; i++) {
    const n = Math.random() * 1000;
    const mine = mySqrt(n);
    const oracle = Math.round(Math.sqrt(n) * 1000) / 1000;
    assert.strictEqual(mine, oracle, `sqrt(${n}): ${mine} vs ${oracle}`);
}

console.log("Property patterns OK ✅");
```

| Pattern | Mô tả | Ví dụ |
|---------|--------|-------|
| **Round-trip** | encode → decode = identity | JSON parse/stringify, serialization |
| **Idempotency** | f(f(x)) = f(x) | normalize, sort, deduplicate |
| **Invariant** | Property holds for ALL inputs | abs(n) ≥ 0, length ≥ 0 |
| **Oracle** | Compare with trusted implementation | mySqrt vs Math.sqrt |

---

## ✅ Checkpoint 32.2-32.3

> Đến đây bạn phải hiểu:
> 1. **fast-check**: auto-generate inputs + check properties. 1000+ tests
> 2. **Round-trip**: decode(encode(x)) === x. Tests serialization
> 3. **Idempotency**: f(f(x)) === f(x). Tests normalization
> 4. **Invariant**: property ALWAYS true. Tests domain rules
> 5. **Oracle**: compare with known-correct impl. Tests algorithms
>
> **Test nhanh**: `JSON.parse(JSON.stringify(obj))` — property pattern nào?
> <details><summary>Đáp án</summary>**Round-trip!** stringify → parse = identity (cho JSON-safe objects). Nếu object có Date, RegExp → round-trip fails. PBT sẽ tìm ra!</details>

---

## 🏋️ Bài tập

**Bài 1** (10 phút): PBT for reverse

```typescript
// Properties cho Array.reverse():
// 1. reverse(reverse(arr)) === arr (round-trip/involution)
// 2. reverse(arr).length === arr.length (invariant)
// 3. reverse([x]) === [x] (identity for single)
// Test 100 random arrays
```

<details><summary>✅ Lời giải</summary>

```typescript
import assert from "node:assert/strict";

const reverse = <A>(arr: readonly A[]): readonly A[] => [...arr].reverse();

for (let i = 0; i < 100; i++) {
    const len = Math.floor(Math.random() * 20);
    const input = Array.from({ length: len }, () => Math.floor(Math.random() * 100));

    // Round-trip
    assert.deepStrictEqual(reverse(reverse(input)), input);
    // Length invariant
    assert.strictEqual(reverse(input).length, input.length);
}
// Single element
assert.deepStrictEqual(reverse([42]), [42]);
assert.deepStrictEqual(reverse([]), []);
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "Don't know which property to test" | Need practice | Start with round-trip (easiest). Then invariants |
| "PBT is slow" | Too many runs | Default 100 is enough. 1000 for critical code |
| "Random test fails intermittently" | Genuine bug! | PBT found edge case. Reproduce with minimal input, fix bug |
| "Can't generate domain types" | Need custom Arbitrary | Compose: `fc.record({ name: fc.string(), age: fc.nat(150) })` |

---

## 💬 Đối thoại với bản thân: PBT Q&A

**Q: PBT nghe hay nhưng có thực tế không? Viết properties KHÓ hơn viết examples.**

A: Đúng — lần đầu KHÓ hơn. Nhưng 1 property test = 100-1000 example tests. Và PBT tìm bugs bạn KHÔNG NGHĨ RA. QuickCheck tìm bugs trong telecoms sau 20 năm testing truyền thống. Investment ban đầu cao, payoff cực lớn.

**Q: Khi nào dùng PBT vs example tests?**

A: Example tests: documentation + happy path + known edge cases. PBT: invariants + fuzzing + serialization round-trips. Kết hợp cả hai. Examples cho clarity, PBT cho completeness.

**Q: Shrinking là gì và tại sao quan trọng?**

A: Khi PBT tìm ra failing input (ví dụ array 50 elements), shrinking tự động THU NHỎ input đến NHỎ NHẤT vẫn fail. Array 50 → array 3 → array 1. Dễ debug hơn 50x. `fast-check` làm shrinking tự động.

---

## Tóm tắt

Property-Based Testing thay đổi cách bạn nghĩ về testing: từ "đoán vài câu" sang "chứng minh tính chất". Example tests vẫn cần (cho happy path và documentation). PBT bổ sung (để tìm bugs bạn không nghĩ tới).

Kết hợp với FP: pure functions là ứng cử viên HOÀN HẢO cho PBT. Không side effects, deterministic, dễ viết properties. Round-trip test cho Zod schemas. Idempotency cho normalizers. Invariant cho domain rules. Oracle cho algorithms.

- ✅ **PBT** = test properties, not examples. Generate 1000+ random inputs.
- ✅ **fast-check** = TypeScript PBT library. Arbitraries + properties.
- ✅ **4 patterns**: round-trip, idempotency, invariant, oracle.
- ✅ **Shrinking**: auto-find minimal failing input.
- ✅ **PBT + TDD**: example tests for happy path, PBT for edge cases.

## Tiếp theo

→ Chapter 33: **Architecture Patterns** — Hexagonal, Ports & Adapters, Functional Core / Imperative Shell.
