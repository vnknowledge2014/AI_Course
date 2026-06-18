# Chapter 31 — TDD with TypeScript

> **Bạn sẽ học được**:
> - TDD cycle: Red → Green → Refactor — BẢN CHẤT, không phải hình thức
> - Vitest setup và best practices cho TypeScript
> - Testing pure functions — dễ nhất, hiệu quả nhất
> - Testing domain logic — Value Objects, DUs, workflows
> - Testing với FP DI — no mocks needed (from Ch19, Ch24)
> - Test naming conventions & organization
>
> **Yêu cầu trước**: Chapter 12-24 (FP foundations, domain modeling).
> **Thời gian đọc**: ~50 phút | **Level**: Intermediate-Advanced
> **Kết quả cuối cùng**: Viết test TRƯỚC code — tin vào refactoring vì tests bảo vệ.

---

Bạn đã bao giờ xem phim hậu trường chưa?

Trước khi diễn viên đứng trước camera, đạo diễn viết **kịch bản**. Kịch bản nói: "Nhân vật A vào phòng, nói câu X, phản ứng Y xảy ra." Khi quay, đạo diễn kiểm tra: diễn viên có nói đúng câu X? Phản ứng Y có xảy ra? Nếu sai → quay lại. Nếu đúng → scene đạt.

**TDD** = viết kịch bản trước khi quay phim. "Test" = kịch bản: nó mô tả behavior mong muốn. "Code" = diễn xuất: implementor làm cho test pass. **Red** = kịch bản viết xong, diễn viên chưa diễn (test fails). **Green** = diễn viên diễn đúng (test passes). **Refactor** = đạo diễn chỉnh ánh sáng, góc quay (cải thiện code mà không đổi behavior).

---

## TDD với TypeScript — Red → Green → Refactor

Test-Driven Development không chỉ là "viết test trước code" — đó là **design methodology**. Khi bạn viết test trước, bạn thiết kế API từ perspective của consumer. Kết quả: API tự nhiên hơn, testable by design, và bạn luôn có safety net cho refactoring.

TypeScript + vitest (hoặc jest) cho bạn setup đơn giản nhất: `npm i -D vitest`, viết file `.test.ts`, chạy `vitest`. Type inference làm tests gọn gàng — không cần type annotations verbose. Chapter này dạy TDD cycle qua ví dụ thực tế, theo phong cách *Learn Go with Tests*.


*Learn Go with Tests* dạy Go hoàn toàn qua TDD. Mỗi concept bắt đầu bằng test. Chapter này áp dụng approach tương tự cho TypeScript — với lợi thế: type system đã bắt nhiều bugs mà Go cần tests, nên TypeScript tests tập trung vào **business logic**.

`vitest` là tool tốt nhất: TypeScript native, fast, jest-compatible API. Setup = `npm i -D vitest`, viết `.test.ts`, chạy `vitest`. Done. Cycle: Red (test fail) → Green (code pass) → Refactor (clean up). Mỗi cycle dưới 5 phút.

## 31.1 — TDD Cycle: Red → Green → Refactor

### Tại sao viết test TRƯỚC?

```typescript
// filename: src/tdd_why.ts

// ❌ Test-after:
// 1. Viết code → 2. Code "works" (tested manually) → 3. Viết tests (bored, skip edge cases)
// Problem: tests mirror implementation, not behavior. Tests fragile.

// ✅ Test-first (TDD):
// 1. Viết test mô tả BEHAVIOR → 2. Test FAILS (red) → 3. Viết code tối thiểu cho test pass (green)
// → 4. Refactor (clean code, tests still pass) → repeat
// Benefit: tests describe WHAT, not HOW. Code driven by requirements.

// TDD forces you to think about:
// - WHAT should this function do? (not HOW to implement)
// - What are the EDGE CASES?
// - What ERRORS can occur?
// - What is the INTERFACE? (how to call it)

console.log("TDD why OK ✅");
```

### Vitest setup cho TypeScript

```typescript
// filename: src/tdd_setup.ts

// Vitest = modern test runner for TypeScript
// Installation: npm install -D vitest
// Config: vitest.config.ts or package.json

// vitest.config.ts:
// import { defineConfig } from 'vitest/config';
// export default defineConfig({
//     test: {
//         include: ['src/**/*.test.ts'],
//         coverage: { provider: 'v8' },
//     },
// });

// Run: npx vitest        (watch mode)
// Run: npx vitest run    (single run)
// Run: npx vitest --ui   (browser UI)

console.log("Vitest setup OK ✅");
```

---

## TDD with TypeScript — Red-Green-Refactor

TDD = viết test TRƯỚC code. Red (test fail) → Green (viết code đủ để pass) → Refactor (cải tiến structure giữ tests green). Với FP: test pure functions (không cần mock), inject dependencies qua parameters (không cần DI framework). Vitest là test runner.


## 31.2 — TDD Pure Functions

### Bắt đầu dễ nhất: pure functions

Pure functions = vàng cho testing. No side effects, no mocks, deterministic. Input A → output B. Always.

```typescript
// filename: src/tdd_pure.test.ts
import assert from "node:assert/strict";

// === RED: Write test first ===
// We WANT: parsePrice("42,500.99") → 42500.99
// We WANT: parsePrice("abc") → null
// We WANT: parsePrice("-5") → null (negative)
// We WANT: parsePrice("0") → 0 (zero is valid)

// Simulate test syntax:
// describe("parsePrice", () => {
//     it("parses valid price string", () => { ... });
//     it("returns null for non-numeric", () => { ... });
// });

// === GREEN: Minimal implementation ===
const parsePrice = (input: string): number | null => {
    const cleaned = input.replace(/,/g, "");
    const n = Number(cleaned);
    if (isNaN(n) || n < 0) return null;
    return Math.round(n * 100) / 100;
};

// === Tests ===
// Happy path
assert.strictEqual(parsePrice("42500.99"), 42500.99);
assert.strictEqual(parsePrice("42,500.99"), 42500.99);  // with comma
assert.strictEqual(parsePrice("0"), 0);

// Edge cases
assert.strictEqual(parsePrice("abc"), null);
assert.strictEqual(parsePrice("-5"), null);
assert.strictEqual(parsePrice(""), null);
assert.strictEqual(parsePrice("12.345"), 12.35);  // rounded

// === REFACTOR: Improve (tests still pass) ===
// Could add: currency prefix handling, locale-aware parsing, etc.

console.log("TDD pure functions OK ✅");
```

### TDD workflow demo — Email validation

```typescript
// filename: src/tdd_email.test.ts
import assert from "node:assert/strict";

// === STEP 1: RED — Write tests for behavior we WANT ===

// Test plan:
// ✅ valid: "an@mail.com" → { tag: "ok", value: "an@mail.com" }
// ✅ trimmed + lowered: " An@Mail.COM " → { tag: "ok", value: "an@mail.com" }
// ❌ no @: "anmail.com" → { tag: "err", error: "missing_at" }
// ❌ no domain: "an@" → { tag: "err", error: "missing_domain" }
// ❌ empty: "" → { tag: "err", error: "empty" }

type Result<T, E> = { tag: "ok"; value: T } | { tag: "err"; error: E };
type EmailError = "empty" | "missing_at" | "missing_domain" | "invalid_format";

// === STEP 2: GREEN — Minimal implementation ===
const validateEmail = (input: string): Result<string, EmailError> => {
    const trimmed = input.trim().toLowerCase();
    if (trimmed.length === 0) return { tag: "err", error: "empty" };
    if (!trimmed.includes("@")) return { tag: "err", error: "missing_at" };
    const [local, domain] = trimmed.split("@");
    if (!domain || domain.length === 0) return { tag: "err", error: "missing_domain" };
    if (!domain.includes(".")) return { tag: "err", error: "invalid_format" };
    return { tag: "ok", value: trimmed };
};

// === STEP 3: Run tests ===
// Happy path
assert.deepStrictEqual(validateEmail("an@mail.com"), { tag: "ok", value: "an@mail.com" });
assert.deepStrictEqual(validateEmail(" An@Mail.COM "), { tag: "ok", value: "an@mail.com" });

// Error cases
assert.deepStrictEqual(validateEmail(""), { tag: "err", error: "empty" });
assert.deepStrictEqual(validateEmail("anmail.com"), { tag: "err", error: "missing_at" });
assert.deepStrictEqual(validateEmail("an@"), { tag: "err", error: "missing_domain" });
assert.deepStrictEqual(validateEmail("an@mail"), { tag: "err", error: "invalid_format" });

// === STEP 4: REFACTOR ===
// Could extract: isValidDomain, normalizeEmail, etc.
// Tests PROTECT us — refactor freely!

console.log("TDD email validation OK ✅");
```

> **💡 "Tests are executable documentation"**: Tests say WHAT the function does, not HOW. A new developer reads tests → understands behavior. Codebase changes → tests catch regressions.

---

## ✅ Checkpoint 31.1-31.2

> Đến đây bạn phải hiểu:
> 1. **Red → Green → Refactor**: test first → implement → improve
> 2. **Pure functions** = easiest to test. No mocks, no setup
> 3. **Test BEHAVIOR, not implementation**: test what it does, not how
> 4. **Edge cases**: empty, negative, boundary values — TDD forces you to think about them
>
> **Test nhanh**: Bạn refactor `parsePrice` để dùng regex thay vì `Number()` — tests phải sửa?
> <details><summary>Đáp án</summary>**KHÔNG!** Tests kiểm tra behavior (input → output). Implementation thay đổi, behavior giữ nguyên → tests vẫn pass. Đây là power của TDD.</details>

---

## 31.3 — Testing Domain Logic

### DUs, Value Objects, Workflows — all testable without mocks

```typescript
// filename: src/tdd_domain.test.ts
import assert from "node:assert/strict";

// === Domain Types ===
type OrderStatus = "draft" | "confirmed" | "shipped" | "delivered" | "cancelled";

type OrderEvent =
    | { type: "confirm" }
    | { type: "ship"; trackingId: string }
    | { type: "deliver" }
    | { type: "cancel"; reason: string };

type TransitionResult =
    | { tag: "ok"; newStatus: OrderStatus }
    | { tag: "err"; error: string };

// === State machine (pure function!) ===
const transition = (current: OrderStatus, event: OrderEvent): TransitionResult => {
    switch (current) {
        case "draft":
            if (event.type === "confirm") return { tag: "ok", newStatus: "confirmed" };
            if (event.type === "cancel") return { tag: "ok", newStatus: "cancelled" };
            return { tag: "err", error: `Cannot ${event.type} from draft` };

        case "confirmed":
            if (event.type === "ship") return { tag: "ok", newStatus: "shipped" };
            if (event.type === "cancel") return { tag: "ok", newStatus: "cancelled" };
            return { tag: "err", error: `Cannot ${event.type} from confirmed` };

        case "shipped":
            if (event.type === "deliver") return { tag: "ok", newStatus: "delivered" };
            return { tag: "err", error: `Cannot ${event.type} from shipped` };

        case "delivered":
        case "cancelled":
            return { tag: "err", error: `Order is ${current} — no transitions` };
    }
};

// === Tests: Valid transitions ===
assert.deepStrictEqual(transition("draft", { type: "confirm" }), { tag: "ok", newStatus: "confirmed" });
assert.deepStrictEqual(transition("confirmed", { type: "ship", trackingId: "TRACK-1" }), { tag: "ok", newStatus: "shipped" });
assert.deepStrictEqual(transition("shipped", { type: "deliver" }), { tag: "ok", newStatus: "delivered" });
assert.deepStrictEqual(transition("draft", { type: "cancel", reason: "Changed mind" }), { tag: "ok", newStatus: "cancelled" });

// === Tests: Invalid transitions ===
assert.strictEqual(transition("draft", { type: "ship", trackingId: "T1" }).tag, "err");
assert.strictEqual(transition("delivered", { type: "cancel", reason: "Oops" }).tag, "err");
assert.strictEqual(transition("cancelled", { type: "confirm" }).tag, "err");

// === Tests: Full path ===
const events: OrderEvent[] = [
    { type: "confirm" },
    { type: "ship", trackingId: "TRACK-001" },
    { type: "deliver" },
];

let status: OrderStatus = "draft";
for (const event of events) {
    const result = transition(status, event);
    assert.strictEqual(result.tag, "ok");
    if (result.tag === "ok") status = result.newStatus;
}
assert.strictEqual(status, "delivered");

console.log("TDD domain logic OK ✅");
```

---

## 31.4 — Testing with FP Dependency Injection

### No mocks — just plain objects

```typescript
// filename: src/tdd_fp_di.test.ts
import assert from "node:assert/strict";

// === Domain types ===
type UserId = string & { __brand: "UserId" };
type Email = string & { __brand: "Email" };
type User = { id: UserId; name: string; email: Email; isActive: boolean };

// === Dependencies (interfaces) ===
type UserRepo = {
    readonly findByEmail: (email: Email) => Promise<User | null>;
    readonly save: (user: User) => Promise<void>;
};

type EmailService = {
    readonly sendWelcome: (user: User) => Promise<void>;
};

type Deps = {
    readonly userRepo: UserRepo;
    readonly emailService: EmailService;
    readonly generateId: () => UserId;
    readonly now: () => Date;
};

// === Use case ===
type Result<T, E> = { tag: "ok"; value: T } | { tag: "err"; error: E };

type RegisterError = "invalid_email" | "duplicate_email";

const registerUser = async (
    deps: Deps,
    name: string,
    rawEmail: string,
): Promise<Result<User, RegisterError>> => {
    const email = rawEmail.trim().toLowerCase();
    if (!email.includes("@") || !email.includes(".")) {
        return { tag: "err", error: "invalid_email" };
    }

    const existing = await deps.userRepo.findByEmail(email as Email);
    if (existing) return { tag: "err", error: "duplicate_email" };

    const user: User = {
        id: deps.generateId(),
        name: name.trim(),
        email: email as Email,
        isActive: true,
    };

    await deps.userRepo.save(user);
    await deps.emailService.sendWelcome(user);

    return { tag: "ok", value: user };
};

// === Tests: PLAIN OBJECTS as fakes. No mock library! ===
const run = async () => {
    // Test helpers
    const savedUsers: User[] = [];
    const sentEmails: Email[] = [];
    let idCounter = 0;

    const makeDeps = (existingEmails: string[] = []): Deps => ({
        userRepo: {
            findByEmail: async (email) =>
                existingEmails.includes(email)
                    ? { id: "EXISTING" as UserId, name: "X", email, isActive: true }
                    : null,
            save: async (user) => { savedUsers.push(user); },
        },
        emailService: {
            sendWelcome: async (user) => { sentEmails.push(user.email); },
        },
        generateId: () => `U-${++idCounter}` as UserId,
        now: () => new Date("2024-06-15"),
    });

    // Test 1: Happy path
    const deps1 = makeDeps();
    const result = await registerUser(deps1, "An", " An@Mail.COM ");
    assert.strictEqual(result.tag, "ok");
    if (result.tag === "ok") {
        assert.strictEqual(result.value.email, "an@mail.com"); // trimmed + lowered
        assert.strictEqual(result.value.name, "An");
    }
    assert.strictEqual(savedUsers.length, 1);
    assert.strictEqual(sentEmails.length, 1);

    // Test 2: Invalid email
    const result2 = await registerUser(deps1, "Bình", "bad-email");
    assert.strictEqual(result2.tag, "err");
    if (result2.tag === "err") assert.strictEqual(result2.error, "invalid_email");

    // Test 3: Duplicate email
    const deps3 = makeDeps(["existing@mail.com"]);
    const result3 = await registerUser(deps3, "Cường", "existing@mail.com");
    assert.strictEqual(result3.tag, "err");
    if (result3.tag === "err") assert.strictEqual(result3.error, "duplicate_email");

    console.log("TDD with FP DI OK ✅");
};

run();
```

> **💡 FP DI = no mock library!** `makeDeps()` creates plain objects. `savedUsers` array records what was saved. `sentEmails` records notifications. Inspect arrays in assertions. No `jest.fn()`, no `sinon.stub()`. Simple, explicit, type-safe.

---

## ✅ Checkpoint 31.3-31.4

> Đến đây bạn phải hiểu:
> 1. **State machine** = pure function → test ALL transitions + invalid paths
> 2. **FP DI** = pass deps as parameter. Test = pass fake deps
> 3. **No mocks!** Plain objects + arrays as recorders
> 4. **`makeDeps()`** factory = configurable fakes per test
>
> **Test nhanh**: Thay `UserRepo` implementation (thêm caching) — tests phải sửa?
> <details><summary>Đáp án</summary>**KHÔNG!** Tests test `registerUser` behavior via interface. `UserRepo` implementation thay đổi = infrastructure concern. Domain tests unchanged.</details>

---

## 31.5 — Test Naming & Organization

```typescript
// filename: src/test_organization.ts

// === Naming Convention ===
// "should [expected behavior] when [condition]"

// describe("parsePrice", () => {
//     it("should return number when input is valid numeric string");
//     it("should return null when input is empty");
//     it("should remove commas and parse correctly");
//     it("should return null when input is negative");
// });

// === Arrange-Act-Assert (AAA) ===
// Arrange: setup data + deps
// Act: call function
// Assert: verify result

// describe("registerUser", () => {
//     it("should save user when email is valid and unique", async () => {
//         // Arrange
//         const deps = makeDeps();
//         // Act
//         const result = await registerUser(deps, "An", "an@mail.com");
//         // Assert
//         assert.strictEqual(result.tag, "ok");
//     });
// });

// === File Organization ===
// src/
//   domain/
//     order.ts           ← domain code
//     order.test.ts      ← co-located tests
//   application/
//     register-user.ts
//     register-user.test.ts
//   __tests__/           ← OR centralized
//     domain/
//     application/

console.log("Test organization OK ✅");
```

---

## 🏋️ Bài tập

**Bài 1** (10 phút): TDD a calculator

```typescript
// TDD a calculateTotal function:
// Input: array of { quantity: number, unitPrice: number, discount?: number }
// Output: total after discounts
// Rules: discount is percentage (0-100), default 0
// Write tests FIRST, then implement
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

type LineItem = { quantity: number; unitPrice: number; discount?: number };

// Tests FIRST:
// calculateTotal([]) → 0
// calculateTotal([{ quantity: 2, unitPrice: 100 }]) → 200
// calculateTotal([{ quantity: 1, unitPrice: 1000, discount: 10 }]) → 900
// calculateTotal([
//   { quantity: 2, unitPrice: 100 },
//   { quantity: 1, unitPrice: 500, discount: 20 }
// ]) → 200 + 400 = 600

// Implementation:
const calculateTotal = (items: readonly LineItem[]): number =>
    items.reduce((sum, item) => {
        const subtotal = item.quantity * item.unitPrice;
        const discountRate = (item.discount ?? 0) / 100;
        return sum + subtotal * (1 - discountRate);
    }, 0);

// Run tests:
assert.strictEqual(calculateTotal([]), 0);
assert.strictEqual(calculateTotal([{ quantity: 2, unitPrice: 100 }]), 200);
assert.strictEqual(calculateTotal([{ quantity: 1, unitPrice: 1000, discount: 10 }]), 900);
assert.strictEqual(calculateTotal([
    { quantity: 2, unitPrice: 100 },
    { quantity: 1, unitPrice: 500, discount: 20 },
]), 600);
```

</details>

**Bài 2** (15 phút): TDD with FP DI

```typescript
// TDD a "transfer money" use case:
// Deps: AccountRepo { findById, save }
// Input: fromId, toId, amount
// Errors: "from_not_found" | "to_not_found" | "insufficient_funds"
// Write tests with fake deps FIRST
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type AccountId = string;
type Account = { id: AccountId; balance: number };
type AccountRepo = {
    findById: (id: AccountId) => Promise<Account | null>;
    save: (account: Account) => Promise<void>;
};
type Result<T, E> = { tag: "ok"; value: T } | { tag: "err"; error: E };
type TransferError = "from_not_found" | "to_not_found" | "insufficient_funds";

const transfer = async (
    repo: AccountRepo, fromId: string, toId: string, amount: number,
): Promise<Result<{ from: Account; to: Account }, TransferError>> => {
    const from = await repo.findById(fromId);
    if (!from) return { tag: "err", error: "from_not_found" };
    const to = await repo.findById(toId);
    if (!to) return { tag: "err", error: "to_not_found" };
    if (from.balance < amount) return { tag: "err", error: "insufficient_funds" };

    const updatedFrom = { ...from, balance: from.balance - amount };
    const updatedTo = { ...to, balance: to.balance + amount };
    await repo.save(updatedFrom);
    await repo.save(updatedTo);
    return { tag: "ok", value: { from: updatedFrom, to: updatedTo } };
};

const run = async () => {
    const store = new Map<string, Account>([
        ["A1", { id: "A1", balance: 1000 }],
        ["A2", { id: "A2", balance: 500 }],
    ]);
    const repo: AccountRepo = {
        findById: async (id) => store.get(id) ?? null,
        save: async (account) => { store.set(account.id, account); },
    };

    const r1 = await transfer(repo, "A1", "A2", 300);
    assert.strictEqual(r1.tag, "ok");
    if (r1.tag === "ok") {
        assert.strictEqual(r1.value.from.balance, 700);
        assert.strictEqual(r1.value.to.balance, 800);
    }

    const r2 = await transfer(repo, "A1", "A2", 9999);
    assert.strictEqual(r2.tag, "err");

    const r3 = await transfer(repo, "X", "A2", 100);
    assert.deepStrictEqual(r3, { tag: "err", error: "from_not_found" });

    console.log("Transfer TDD OK ✅");
};
run();
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "Tests pass but code still breaks" | Not testing edge cases | TDD: think about edges FIRST. Empty, null, boundary, negative |
| "Tests break every time I refactor" | Testing implementation, not behavior | Test WHAT (input → output), not HOW (internal steps) |
| "Too many mocks" | Tight coupling | Use FP DI. Plain objects as fakes. No mock library |
| "Don't know what to test first" | Analysis paralysis | Start with happy path. Then ONE error case. Then edge cases |

---

## Tóm tắt

- ✅ **TDD = Red → Green → Refactor**. Kịch bản trước, diễn xuất sau.
- ✅ **Pure functions** = easiest targets. No mocks, deterministic.
- ✅ **Domain logic** = state machines, value objects. Pure → testable.
- ✅ **FP DI** = pass deps as params. Test with plain object fakes.
- ✅ **AAA**: Arrange → Act → Assert. Clear test structure.
- ✅ **"should [behavior] when [condition]"** naming convention.

→ Chapter 32: **Property-Based Testing** — test 1000 cases tự động với fast-check.
