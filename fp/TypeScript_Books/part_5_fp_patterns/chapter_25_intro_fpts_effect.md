# Chapter 25 — Introduction to fp-ts / Effect

> **Bạn sẽ học được**:
> - Tại sao cần library FP khi đã tự viết Result, pipe, flow?
> - fp-ts ecosystem overview — `Option`, `Either`, `Task`, `TaskEither`
> - Effect ecosystem overview — `Effect<A, E, R>`, generators, services
> - `pipe()` và `flow()` trong fp-ts vs tự viết
> - Chọn fp-ts hay Effect — decision framework
>
> **Yêu cầu trước**: Chapter 12 (pipe/flow), Chapter 13 (Result/Option), Chapter 21-22 (pipelines, ROP).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Hiểu hai ecosystem FP lớn nhất của TypeScript — biết khi nào dùng cái nào.

---

Bạn đã tự đóng bàn bao giờ chưa?

Chúng ta đã dành Ch12-24 để "tự đóng" công cụ FP: `pipe()`, `flow()`, `Result<T,E>`, `Option<T>`, `andThen`, `map`, `mapErr`... Như thợ mộc tự mài đục, tự rèn cưa. Hiểu cách công cụ hoạt động = nền tảng vững. Nhưng thợ mộc chuyên nghiệp không tự rèn mọi thứ — họ mua **hộp công cụ chuyên dụng** (Makita, Bosch) với precision tốt hơn, đầy đủ phụ kiện, và warranty.

**fp-ts** và **Effect** là hai hộp công cụ FP cho TypeScript. `fp-ts` = hộp công cụ truyền thống (Haskell-inspired), đầy đủ type classes, battle-tested. `Effect` = hộp công cụ thế hệ mới (ZIO-inspired), tích hợp DI + resource management + concurrency. Chương này giới thiệu cả hai — để bạn chọn đúng hộp cho project.

---

## fp-ts & Effect — TypeScript FP Libraries

Đến đây bạn đã biết FP concepts: immutability, purity, composition, ADTs. Nhưng TypeScript stdlib không cung cấp `Option`, `Either`, `Task` — bạn cần libraries. Hai lựa chọn chính:

**fp-ts** (Giulio Canti): Haskell-inspired, `pipe()` + `flow()`, tight integration với functional concepts. Mature và battle-tested.

**Effect** (Effect-TS team): Modern alternative, better DX, built-in dependency injection, streaming, concurrency. Growing adoption.

Chapter này dạy bạn dùng cả hai — vì trong thực tế, bạn sẽ gặp cả hai trong codebases khác nhau.


## 25.1 — Tại sao cần Library FP?

### Tự viết vs dùng library

Ch12-24 tự viết tất cả — đó là cách tốt nhất để HIỂU. Nhưng production code cần:

```typescript
// filename: src/why_library.ts

// ❌ Vấn đề khi tự viết:

// 1. Thiếu edge cases — Result của bạn handle stack overflow? Race conditions?
// 2. Thiếu utilities — bạn có sequenceT, traverseArray, getMonoid?
// 3. Thiếu interop — third-party library expect fp-ts types
// 4. Thiếu documentation — team mới phải đọc source code bạn
// 5. Thiếu testing — library đã tested hàng nghìn test cases

// ✅ Library cho bạn:
// 1. Battle-tested types: Option, Either, Task, TaskEither, IO, Reader
// 2. 200+ utility functions, all type-safe
// 3. Community + ecosystem (plugins, integrations)
// 4. Documentation + learning resources
// 5. Performance optimized
```

### fp-ts vs Effect — tổng quan

```typescript
// filename: src/ecosystem_overview.ts

// fp-ts: Haskell cho TypeScript
// - Author: Giulio Canti (gcanti)
// - Style: pipe(value, transform1, transform2)
// - Learning: Haskell/PureScript background → familiar
// - Size: modular (import only what you need)
// - Maturity: 2017+, mature, large ecosystem

// Effect: ZIO cho TypeScript
// - Author: Effect team (grew from fp-ts community)
// - Style: Effect.gen(function*() { ... })  — generator-based
// - Learning: Imperative-looking FP (easier for beginners!)
// - Size: larger (~50KB+), batteries-included
// - Maturity: 2022+, rapidly growing, all-in-one framework
```

> **💡 "Learn the concepts first, choose the library second"**: Ch12-24 taught concepts. fp-ts và Effect = implementations of those concepts. Concepts transfer between libraries. Choice depends on team + project.

---

## ✅ Checkpoint 25.1

> Đến đây bạn phải hiểu:
> 1. **Self-written code** tốt cho học tập. Library tốt cho production
> 2. **fp-ts** = Haskell-style, pipe-based, modular, mature
> 3. **Effect** = ZIO-style, generator-based, batteries-included, modern
> 4. **Concepts from Ch12-24** apply to BOTH libraries
>
> **Test nhanh**: Bạn đã tự viết `map`, `andThen`, `pipe`, `flow` — fp-ts có gì khác?
> <details><summary>Đáp án</summary>Cùng concepts! fp-ts's `map`, `chain` (= andThen), `pipe`, `flow` = identical semantics. Khác: hàng trăm utilities thêm, tested, optimized, documented.</details>

---

## 25.2 — fp-ts: Option & Either

### Option = "có hoặc không" (thay thế null/undefined)

`Option<A>` = `None | Some<A>`. Thay vì `A | null | undefined` (3 trạng thái, dễ quên check), `Option` ép bạn handle cả hai case. fp-ts cung cấp `map`, `chain`, `getOrElse`, `fold` — tất cả piped.

```typescript
// filename: src/fpts_option.ts
import assert from "node:assert/strict";

// Simulating fp-ts Option API
type None = { readonly _tag: "None" };
type Some<A> = { readonly _tag: "Some"; readonly value: A };
type Option<A> = None | Some<A>;

const none: Option<never> = { _tag: "None" };
const some = <A>(value: A): Option<A> => ({ _tag: "Some", value });

// Constructors
const fromNullable = <A>(value: A | null | undefined): Option<A> =>
    value == null ? none : some(value);

// Combinators
const map = <A, B>(fn: (a: A) => B) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : some(fn(fa.value));

const chain = <A, B>(fn: (a: A) => Option<B>) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : fn(fa.value);

const getOrElse = <A>(onNone: () => A) => (fa: Option<A>): A =>
    fa._tag === "None" ? onNone() : fa.value;

const fold = <A, B>(onNone: () => B, onSome: (a: A) => B) => (fa: Option<A>): B =>
    fa._tag === "None" ? onNone() : onSome(fa.value);

// pipe (from fp-ts)
function pipe<A>(a: A): A;
function pipe<A, B>(a: A, ab: (a: A) => B): B;
function pipe<A, B, C>(a: A, ab: (a: A) => B, bc: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D): D;
function pipe(a: unknown, ...fns: ((x: unknown) => unknown)[]): unknown {
    return fns.reduce((acc, fn) => fn(acc), a);
}

// --- Usage ---

// Safe dictionary lookup
const users: Record<string, { name: string; age: number }> = {
    "U1": { name: "An", age: 25 },
    "U2": { name: "Bình", age: 30 },
};

const findUser = (id: string): Option<{ name: string; age: number }> =>
    fromNullable(users[id]);

const getUserName = (id: string): string =>
    pipe(
        findUser(id),
        map(user => user.name),
        getOrElse(() => "Unknown"),
    );

assert.strictEqual(getUserName("U1"), "An");
assert.strictEqual(getUserName("U999"), "Unknown");

// Chain: lookup → validate → transform
const getAdultName = (id: string): Option<string> =>
    pipe(
        findUser(id),
        chain(user => user.age >= 18 ? some(user) : none),
        map(user => user.name.toUpperCase()),
    );

assert.deepStrictEqual(getAdultName("U1"), some("AN"));
assert.deepStrictEqual(getAdultName("U999"), none);

console.log("fp-ts Option OK ✅");
```

### Either = Result (left = error, right = success)

`Either<E, A>` = fp-ts version of `Result<T, E>`. Convention: `Left` = error, `Right` = success ("right" = "correct").

```typescript
// filename: src/fpts_either.ts
import assert from "node:assert/strict";

// Simulating fp-ts Either API
type Left<E> = { readonly _tag: "Left"; readonly left: E };
type Right<A> = { readonly _tag: "Right"; readonly right: A };
type Either<E, A> = Left<E> | Right<A>;

const left = <E>(e: E): Either<E, never> => ({ _tag: "Left", left: e });
const right = <A>(a: A): Either<never, A> => ({ _tag: "Right", right: a });

const map = <A, B>(fn: (a: A) => B) => <E>(fa: Either<E, A>): Either<E, B> =>
    fa._tag === "Left" ? fa : right(fn(fa.right));

const mapLeft = <E, F>(fn: (e: E) => F) => <A>(fa: Either<E, A>): Either<F, A> =>
    fa._tag === "Left" ? left(fn(fa.left)) : fa;

const chain = <E, A, B>(fn: (a: A) => Either<E, B>) => (fa: Either<E, A>): Either<E, B> =>
    fa._tag === "Left" ? fa : fn(fa.right);

const fold = <E, A, B>(onLeft: (e: E) => B, onRight: (a: A) => B) =>
    (fa: Either<E, A>): B =>
        fa._tag === "Left" ? onLeft(fa.left) : onRight(fa.right);

function pipe<A>(a: A): A;
function pipe<A, B>(a: A, ab: (a: A) => B): B;
function pipe<A, B, C>(a: A, ab: (a: A) => B, bc: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D): D;
function pipe(a: unknown, ...fns: ((x: unknown) => unknown)[]): unknown {
    return fns.reduce((acc, fn) => fn(acc), a);
}

// --- Usage ---
type ParseError = "not_a_number" | "not_positive" | "too_large";

const parseNumber = (input: string): Either<ParseError, number> => {
    const n = Number(input);
    return isNaN(n) ? left("not_a_number") : right(n);
};

const ensurePositive = (n: number): Either<ParseError, number> =>
    n > 0 ? right(n) : left("not_positive");

const ensureReasonable = (n: number): Either<ParseError, number> =>
    n <= 1000000 ? right(n) : left("too_large");

// ROP pipeline with Either!
const parsePrice = (input: string): Either<ParseError, number> =>
    pipe(
        parseNumber(input),
        chain(ensurePositive),
        chain(ensureReasonable),
        map(n => Math.round(n * 100) / 100),  // round to 2 decimals
    );

assert.deepStrictEqual(parsePrice("42.567"), right(42.57));
assert.deepStrictEqual(parsePrice("abc"), left("not_a_number"));
assert.deepStrictEqual(parsePrice("-5"), left("not_positive"));
assert.deepStrictEqual(parsePrice("9999999"), left("too_large"));

// fold = pattern match
const message = pipe(
    parsePrice("42.567"),
    fold(
        err => `Error: ${err}`,
        price => `Price: ${price} VND`,
    ),
);
assert.strictEqual(message, "Price: 42.57 VND");

console.log("fp-ts Either OK ✅");
```

> **💡 Either vs Result**: Identical concept! `Left` = `err`, `Right` = `ok`. fp-ts dùng Haskell convention. `chain` = `andThen`. `fold` = `match`. Nếu bạn hiểu Result (Ch13, Ch22) → bạn đã hiểu Either.

---

## ✅ Checkpoint 25.2

> Đến đây bạn phải hiểu:
> 1. **`Option<A>`** = `None | Some<A>`. Replaces `null/undefined`
> 2. **`Either<E, A>`** = `Left<E> | Right<A>`. Replaces `Result<T, E>`
> 3. **fp-ts style**: `pipe(value, map(fn), chain(fn2), getOrElse(() => default))`
> 4. **Curried functions**: `map(fn)` returns a function. Designed for pipe
>
> **Test nhanh**: `pipe(none, map(x => x + 1))` kết quả gì?
> <details><summary>Đáp án</summary>`none`! `map` skip khi Option là None. Giống `map(err(...), fn)` — error track, function không chạy.</details>

---

## 25.3 — fp-ts: Task & TaskEither

### Task = lazy async computation

`Task<A>` = `() => Promise<A>` — một async computation chưa chạy. Giống `Effect` nhưng đơn giản hơn: không có error channel (always succeeds). `TaskEither<E, A>` = `() => Promise<Either<E, A>>` — async computation that can fail.

```typescript
// filename: src/fpts_task.ts
import assert from "node:assert/strict";

// Either types
type Either<E, A> = { readonly _tag: "Left"; readonly left: E } | { readonly _tag: "Right"; readonly right: A };
const left = <E>(e: E): Either<E, never> => ({ _tag: "Left", left: e });
const right = <A>(a: A): Either<never, A> => ({ _tag: "Right", right: a });

// Task = lazy async (always succeeds)
type Task<A> = () => Promise<A>;

const taskOf = <A>(a: A): Task<A> => () => Promise.resolve(a);

const taskMap = <A, B>(fn: (a: A) => B) => (fa: Task<A>): Task<B> =>
    () => fa().then(fn);

const taskChain = <A, B>(fn: (a: A) => Task<B>) => (fa: Task<A>): Task<B> =>
    () => fa().then(a => fn(a)());

// TaskEither = lazy async that can fail
type TaskEither<E, A> = () => Promise<Either<E, A>>;

const teRight = <A>(a: A): TaskEither<never, A> => () => Promise.resolve(right(a));
const teLeft = <E>(e: E): TaskEither<E, never> => () => Promise.resolve(left(e));

const teMap = <A, B>(fn: (a: A) => B) => <E>(fa: TaskEither<E, A>): TaskEither<E, B> =>
    () => fa().then(ea => ea._tag === "Left" ? ea : right(fn(ea.right)));

const teChain = <E, A, B>(fn: (a: A) => TaskEither<E, B>) => (fa: TaskEither<E, A>): TaskEither<E, B> =>
    () => fa().then(ea => ea._tag === "Left" ? Promise.resolve(ea) : fn(ea.right)());

const teFold = <E, A, B>(onLeft: (e: E) => Task<B>, onRight: (a: A) => Task<B>) =>
    (fa: TaskEither<E, A>): Task<B> =>
        () => fa().then(ea => ea._tag === "Left" ? onLeft(ea.left)() : onRight(ea.right)());

// Wrap throwable async → TaskEither
const tryCatch = <E, A>(
    f: () => Promise<A>,
    onError: (e: unknown) => E,
): TaskEither<E, A> =>
    () => f().then(a => right(a) as Either<E, A>).catch(e => left(onError(e)));

function pipe<A>(a: A): A;
function pipe<A, B>(a: A, ab: (a: A) => B): B;
function pipe<A, B, C>(a: A, ab: (a: A) => B, bc: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D): D;
function pipe(a: unknown, ...fns: ((x: unknown) => unknown)[]): unknown {
    return fns.reduce((acc, fn) => fn(acc), a);
}

// --- Usage: async workflow with TaskEither ---
type AppError =
    | { readonly tag: "not_found"; readonly id: string }
    | { readonly tag: "validation"; readonly message: string };

const findUser = (id: string): TaskEither<AppError, { name: string; email: string }> => {
    const users: Record<string, { name: string; email: string }> = {
        "U1": { name: "An", email: "an@mail.com" },
    };
    return users[id]
        ? teRight(users[id])
        : teLeft({ tag: "not_found", id });
};

const validateEmail = (user: { name: string; email: string }): TaskEither<AppError, { name: string; email: string }> =>
    user.email.includes("@")
        ? teRight(user)
        : teLeft({ tag: "validation", message: "Invalid email" });

const formatGreeting = (user: { name: string; email: string }): string =>
    `Hello, ${user.name}! (${user.email})`;

// Pipeline
const greetUser = (id: string): TaskEither<AppError, string> =>
    pipe(
        findUser(id),
        teChain(validateEmail),
        teMap(formatGreeting),
    );

// Run
const run = async () => {
    const good = await greetUser("U1")();
    assert.strictEqual(good._tag, "Right");
    if (good._tag === "Right") {
        assert.strictEqual(good.right, "Hello, An! (an@mail.com)");
    }

    const notFound = await greetUser("U999")();
    assert.strictEqual(notFound._tag, "Left");
    if (notFound._tag === "Left") {
        assert.strictEqual(notFound.left.tag, "not_found");
    }

    // tryCatch: wrap throwable
    const safeParse = tryCatch(
        () => Promise.resolve(JSON.parse('{"a": 1}')),
        (e) => ({ tag: "validation" as const, message: String(e) }),
    );
    const parsed = await safeParse();
    assert.strictEqual(parsed._tag, "Right");

    console.log("TaskEither OK ✅");
};

run();
```

> **💡 Task vs TaskEither**: `Task<A>` = async that always succeeds. `TaskEither<E, A>` = async that can fail. Choose: does this operation ALWAYS succeed? → Task. Can it fail? → TaskEither.

---

## ✅ Checkpoint 25.3

> Đến đây bạn phải hiểu:
> 1. **`Task<A>`** = `() => Promise<A>`. Lazy async, always succeeds
> 2. **`TaskEither<E, A>`** = `() => Promise<Either<E, A>>`. Lazy async, can fail
> 3. **`tryCatch`** = bridge: Promise that throws → TaskEither
> 4. **Pipeline**: `pipe(findUser(id), teChain(validate), teMap(format))` — async ROP
>
> **Test nhanh**: `const task = greetUser("U1")` — đã chạy chưa?
> <details><summary>Đáp án</summary>**CHƯA!** `Task` = lazy. `task` là function, chưa execute. Phải `await task()` mới chạy. Đây là điểm khác Promise (eager).</details>

---

## 25.4 — Effect: Next-Generation FP

### Hộp công cụ thế hệ mới — tất cả trong một

Effect = framework FP toàn diện. Thay vì kết hợp nhiều types (Option, Either, Task, Reader, IO...), Effect dùng **MỘT type**: `Effect<Success, Error, Requirements>`. Và syntax chính: **generators** — code TRÔNG imperative nhưng CHẠY functional.

```typescript
// filename: src/effect_intro.ts
import assert from "node:assert/strict";

// Simulating Effect core concepts

// Effect<A, E, R>
// A = Success type (what it produces)
// E = Error type (how it can fail)
// R = Requirements (what services it needs — DI built-in!)

// Simplified Effect simulation
type Effect<A, E = never> = {
    readonly _tag: "Effect";
    readonly run: () => Promise<{ _tag: "Success"; value: A } | { _tag: "Failure"; error: E }>;
};

const succeed = <A>(value: A): Effect<A, never> => ({
    _tag: "Effect",
    run: () => Promise.resolve({ _tag: "Success", value }),
});

const fail = <E>(error: E): Effect<never, E> => ({
    _tag: "Effect",
    run: () => Promise.resolve({ _tag: "Failure", error }),
});

const map = <A, B>(effect: Effect<A, any>, fn: (a: A) => B): Effect<B, any> => ({
    _tag: "Effect",
    run: async () => {
        const result = await effect.run();
        return result._tag === "Success"
            ? { _tag: "Success", value: fn(result.value) }
            : result;
    },
});

const flatMap = <A, B, E1, E2>(
    effect: Effect<A, E1>,
    fn: (a: A) => Effect<B, E2>,
): Effect<B, E1 | E2> => ({
    _tag: "Effect",
    run: async () => {
        const result = await effect.run();
        return result._tag === "Success"
            ? fn(result.value).run()
            : result;
    },
});

// --- Generator-based syntax (Effect.gen) ---
// This is how real Effect looks — imperative style!

// Real Effect.gen:
// const program = Effect.gen(function*() {
//     const user = yield* findUser("U1");      // looks like await!
//     const order = yield* createOrder(user);    // sequential, typed
//     return { userId: user.id, orderId: order.id };
// });

// Our simulation:
type AppError =
    | { readonly tag: "not_found"; readonly id: string }
    | { readonly tag: "db_error"; readonly message: string };

const findUser = (id: string): Effect<{ name: string; email: string }, AppError> =>
    id === "U1"
        ? succeed({ name: "An", email: "an@mail.com" })
        : fail({ tag: "not_found", id });

const createOrder = (
    user: { name: string; email: string },
): Effect<{ orderId: string; total: number }, AppError> =>
    succeed({ orderId: `ORD-${Date.now()}`, total: 500000 });

// Composed program
const program = flatMap(
    findUser("U1"),
    user => map(
        createOrder(user),
        order => ({ user: user.name, orderId: order.orderId }),
    ),
);

// Run
const run = async () => {
    const result = await program.run();
    assert.strictEqual(result._tag, "Success");
    if (result._tag === "Success") {
        assert.strictEqual(result.value.user, "An");
        assert.ok(result.value.orderId.startsWith("ORD-"));
    }

    // Error case
    const errorProgram = flatMap(
        findUser("U999"),
        user => succeed({ greeting: `Hello ${user.name}` }),
    );
    const errorResult = await errorProgram.run();
    assert.strictEqual(errorResult._tag, "Failure");
    if (errorResult._tag === "Failure") {
        assert.strictEqual(errorResult.error.tag, "not_found");
    }

    console.log("Effect intro OK ✅");
};

run();
```

### Effect.gen — imperative-looking FP

Đây là killer feature của Effect. Thay vì `flatMap(flatMap(flatMap(...)))`, bạn viết:

```typescript
// filename: src/effect_gen_concept.ts

// Effect.gen syntax (conceptual — real Effect uses this exact syntax):
//
// const registerUser = Effect.gen(function*() {
//     const email = yield* validateEmail(rawEmail);    // may fail with ValidationError
//     const existing = yield* findByEmail(email);       // may fail with DbError
//     if (existing) yield* Effect.fail(new DuplicateError());
//     const user = yield* saveUser({ email });          // may fail with DbError
//     yield* sendWelcome(user);                         // may fail with EmailError
//     return user;
// });
//
// Type inferred:
// Effect<User, ValidationError | DbError | DuplicateError | EmailError, UserRepo & EmailService>
//
// ☝️ ALL error types UNION automatically!
// ☝️ ALL service requirements tracked in R parameter!
// ☝️ Reads like imperative code, but:
//   - No try/catch
//   - All errors typed
//   - DI built-in (R parameter)
//   - Lazy (doesn't run until you call Effect.runPromise)

console.log("Effect.gen concept OK ✅");
```

> **💡 Effect.gen = "write imperative, get functional"**: `yield*` = `await` nhưng type-safe. Error types accumulate automatically. Service requirements tracked. Compiler catches missing error handling.

---

## ✅ Checkpoint 25.4

> Đến đây bạn phải hiểu:
> 1. **`Effect<A, E, R>`** = unified type. Success + Error + Requirements (DI!)
> 2. **`Effect.gen`** = generator syntax. `yield*` looks like `await`
> 3. **Error union**: `yield*` operations auto-union error types
> 4. **Lazy**: Effect = description. `Effect.runPromise(program)` = execution
>
> **Test nhanh**: `Effect<User, HttpError | DbError, UserRepo & Logger>` — cái này nói gì?
> <details><summary>Đáp án</summary>Computation that: produces `User` on success, can fail with `HttpError` hoặc `DbError`, REQUIRES `UserRepo` AND `Logger` services to run.</details>

---

## 25.5 — fp-ts vs Effect: Decision Framework

### Bảng so sánh chi tiết

```typescript
// filename: src/comparison.ts
import assert from "node:assert/strict";

// === fp-ts approach: multiple types, pipe-based ===

// Types needed for one workflow:
// - Option<A>     for nullable values
// - Either<E, A>  for sync errors
// - TaskEither<E, A>  for async errors
// - Reader<R, A>  for DI
// - ReaderTaskEither<R, E, A>  for async errors + DI
// ...combine as needed

// fp-ts workflow style:
// pipe(
//     findUser(id),                  // TaskEither<Error, User>
//     TE.chain(validate),            // chain = andThen
//     TE.map(format),               // map success
//     TE.mapLeft(toApiError),       // map error
//     TE.fold(handleError, handleSuccess),
// )

// === Effect approach: one type, generator-based ===

// One type for everything:
// Effect<A, E, R>

// Effect workflow style:
// Effect.gen(function*() {
//     const user = yield* findUser(id);   // auto-unwrap
//     const valid = yield* validate(user);
//     return format(valid);
// })

// Chỉ cần nhớ: Effect.gen + yield*. Xong!
console.log("Comparison OK ✅");
```

### Khi nào chọn fp-ts, khi nào chọn Effect?

| Tiêu chí | fp-ts | Effect |
|-----------|-------|--------|
| **Learning curve** | Cao (Haskell concepts) | Trung bình (generators = familiar) |
| **Bundle size** | Nhỏ (tree-shakable) | Lớn (~50KB+) |
| **Style** | `pipe(value, fn1, fn2)` | `Effect.gen(function*() { ... })` |
| **DI** | Reader monad (complex) | Built-in `R` parameter (easy!) |
| **Error handling** | Either/TaskEither | Built-in `E` parameter |
| **Concurrency** | Manual (Task.sequenceArray) | Built-in (Fiber, Race, Timeout) |
| **Resource management** | Bracket pattern | Scope + Finalizer |
| **Observability** | Separate library | Built-in (Span, Tracer) |
| **Community** | Large, mature | Growing fast |
| **Best for** | Existing FP teams. Modular needs | New projects. Full-stack FP |

> **💡 Rule of thumb**:
> - **"I just need Option/Either"** → Self-written (Ch13) hoặc `neverthrow` (Ch22)
> - **"I need typed errors + async pipelines"** → `fp-ts` 
> - **"I need DI + resources + concurrency + observability"** → `Effect`
> - **"I'm starting a new large project"** → Consider `Effect` — batteries included

---

## ✅ Checkpoint 25.5

> Đến đây bạn phải hiểu:
> 1. **fp-ts**: pipe-based, modular, Haskell-inspired. Good for existing teams
> 2. **Effect**: generator-based, batteries-included, ZIO-inspired. Good for new projects
> 3. **Both implement same concepts** from Ch12-24. Transfer knowledge!
> 4. **Decision**: based on project size, team experience, needs (DI? concurrency?)
>
> **Test nhanh**: Team biết Haskell, project nhỏ → fp-ts hay Effect?
> <details><summary>Đáp án</summary>**fp-ts!** Team đã familiar Haskell concepts (pipe, typeclass). Project nhỏ = không cần Effect's heavy features. fp-ts modular, import only what needed.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Option pipeline

```typescript
// Viết pipeline dùng Option:
// 1. Lookup user by ID from Map
// 2. Get user's email (Option — some users don't have email)
// 3. Extract domain from email (e.g., "an@mail.com" → "mail.com")
// 4. Default to "unknown" if any step returns None
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

type Option<A> = { _tag: "None" } | { _tag: "Some"; value: A };
const none: Option<never> = { _tag: "None" };
const some = <A>(v: A): Option<A> => ({ _tag: "Some", value: v });

const map = <A, B>(fn: (a: A) => B) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : some(fn(fa.value));

const chain = <A, B>(fn: (a: A) => Option<B>) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : fn(fa.value);

const getOrElse = <A>(def: () => A) => (fa: Option<A>): A =>
    fa._tag === "None" ? def() : fa.value;

const fromNullable = <A>(v: A | null | undefined): Option<A> =>
    v == null ? none : some(v);

function pipe<A>(a: A): A;
function pipe<A, B>(a: A, ab: (a: A) => B): B;
function pipe<A, B, C>(a: A, ab: (a: A) => B, bc: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D): D;
function pipe<A, B, C, D, E>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E): E;
function pipe(a: unknown, ...fns: ((x: unknown) => unknown)[]): unknown {
    return fns.reduce((acc, fn) => fn(acc), a);
}

type User = { name: string; email?: string };
const users = new Map<string, User>([
    ["U1", { name: "An", email: "an@mail.com" }],
    ["U2", { name: "Bình" }],  // no email
]);

const findUser = (id: string): Option<User> => fromNullable(users.get(id));

const getEmail = (user: User): Option<string> => fromNullable(user.email);

const getDomain = (email: string): string => email.split("@")[1] ?? "unknown";

const getUserDomain = (id: string): string =>
    pipe(
        findUser(id),
        chain(getEmail),
        map(getDomain),
        getOrElse(() => "unknown"),
    );

assert.strictEqual(getUserDomain("U1"), "mail.com");
assert.strictEqual(getUserDomain("U2"), "unknown");  // no email
assert.strictEqual(getUserDomain("U999"), "unknown"); // no user
```

</details>

---

**Bài 2** (10 phút): Either pipeline

```typescript
// Viết validation pipeline dùng Either:
// 1. parseAge(string) → Either<Error, number>
// 2. validateRange(min, max) → (n: number) → Either<Error, number>
// 3. categorize → "child" | "teen" | "adult" | "senior"
// Pipe tất cả lại
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type Either<E, A> = { _tag: "Left"; left: E } | { _tag: "Right"; right: A };
const left = <E>(e: E): Either<E, never> => ({ _tag: "Left", left: e });
const right = <A>(a: A): Either<never, A> => ({ _tag: "Right", right: a });

const map = <A, B>(fn: (a: A) => B) => <E>(fa: Either<E, A>): Either<E, B> =>
    fa._tag === "Left" ? fa : right(fn(fa.right));

const chain = <E, A, B>(fn: (a: A) => Either<E, B>) => (fa: Either<E, A>): Either<E, B> =>
    fa._tag === "Left" ? fa : fn(fa.right);

function pipe<A>(a: A): A;
function pipe<A, B>(a: A, ab: (a: A) => B): B;
function pipe<A, B, C>(a: A, ab: (a: A) => B, bc: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D): D;
function pipe(a: unknown, ...fns: ((x: unknown) => unknown)[]): unknown {
    return fns.reduce((acc, fn) => fn(acc), a);
}

type AgeError = "not_a_number" | "out_of_range";

const parseAge = (input: string): Either<AgeError, number> => {
    const n = Number(input);
    return Number.isNaN(n) || !Number.isInteger(n) ? left("not_a_number") : right(n);
};

const validateRange = (min: number, max: number) => (n: number): Either<AgeError, number> =>
    n >= min && n <= max ? right(n) : left("out_of_range");

type Category = "child" | "teen" | "adult" | "senior";
const categorize = (age: number): Category =>
    age < 13 ? "child" : age < 18 ? "teen" : age < 65 ? "adult" : "senior";

const classifyAge = (input: string): Either<AgeError, Category> =>
    pipe(
        parseAge(input),
        chain(validateRange(0, 150)),
        map(categorize),
    );

assert.deepStrictEqual(classifyAge("25"), right("adult"));
assert.deepStrictEqual(classifyAge("8"), right("child"));
assert.deepStrictEqual(classifyAge("70"), right("senior"));
assert.deepStrictEqual(classifyAge("abc"), left("not_a_number"));
assert.deepStrictEqual(classifyAge("200"), left("out_of_range"));
```

</details>

---

**Bài 3** (15 phút): TaskEither workflow

```typescript
// Viết async workflow dùng TaskEither:
// 1. fetchConfig() → TaskEither<"config_missing", Config>
// 2. connectDb(config) → TaskEither<"db_error", DbConnection>
// 3. runMigrations(db) → TaskEither<"migration_error", number>
//
// Chain tất cả. Test happy + each error
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type Either<E, A> = { _tag: "Left"; left: E } | { _tag: "Right"; right: A };
const left = <E>(e: E): Either<E, never> => ({ _tag: "Left", left: e });
const right = <A>(a: A): Either<never, A> => ({ _tag: "Right", right: a });

type TaskEither<E, A> = () => Promise<Either<E, A>>;
const teRight = <A>(a: A): TaskEither<never, A> => () => Promise.resolve(right(a));
const teLeft = <E>(e: E): TaskEither<E, never> => () => Promise.resolve(left(e));

const teChain = <E, A, B>(fn: (a: A) => TaskEither<E, B>) => (fa: TaskEither<E, A>): TaskEither<E, B> =>
    () => fa().then(ea => ea._tag === "Left" ? Promise.resolve(ea) : fn(ea.right)());

const teMap = <A, B>(fn: (a: A) => B) => <E>(fa: TaskEither<E, A>): TaskEither<E, B> =>
    () => fa().then(ea => ea._tag === "Left" ? ea : right(fn(ea.right)));

function pipe<A>(a: A): A;
function pipe<A, B>(a: A, ab: (a: A) => B): B;
function pipe<A, B, C>(a: A, ab: (a: A) => B, bc: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D): D;
function pipe(a: unknown, ...fns: ((x: unknown) => unknown)[]): unknown {
    return fns.reduce((acc, fn) => fn(acc), a);
}

type AppError = "config_missing" | "db_error" | "migration_error";
type Config = { host: string; port: number };
type DbConnection = { connected: true; host: string };

const fetchConfig = (exists: boolean): TaskEither<AppError, Config> =>
    exists ? teRight({ host: "localhost", port: 5432 }) : teLeft("config_missing");

const connectDb = (config: Config): TaskEither<AppError, DbConnection> =>
    config.host ? teRight({ connected: true, host: config.host }) : teLeft("db_error");

const runMigrations = (db: DbConnection): TaskEither<AppError, number> =>
    db.connected ? teRight(5) : teLeft("migration_error");

const setup = (configExists: boolean): TaskEither<AppError, string> =>
    pipe(
        fetchConfig(configExists),
        teChain(connectDb),
        teChain(runMigrations),
        teMap(count => `Applied ${count} migrations`),
    );

const run = async () => {
    const good = await setup(true)();
    assert.strictEqual(good._tag, "Right");
    if (good._tag === "Right") assert.strictEqual(good.right, "Applied 5 migrations");

    const noConfig = await setup(false)();
    assert.strictEqual(noConfig._tag, "Left");
    if (noConfig._tag === "Left") assert.strictEqual(noConfig.left, "config_missing");

    console.log("TaskEither workflow OK ✅");
};

run();
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "Too many types: Option, Either, Task, TaskEither..." | fp-ts complexity | Use Effect instead — one type for all. Or learn incrementally |
| pipe() chains too long | Readability issue | Break into named sub-pipelines. Or use Effect.gen |
| "map vs chain — when to use which?" | Core confusion | `map`: transform value, function ALWAYS succeeds. `chain`: transform value, function CAN FAIL (returns Option/Either) |
| "fold vs getOrElse vs match?" | Too many extractors | `getOrElse`: extract with default. `fold`/`match`: handle both cases |
| "Effect is too heavy for my project" | Over-engineering | Start with neverthrow (Ch22). Upgrade to Effect when you NEED concurrency/DI/resources |
| "fp-ts vs Effect vs hand-written?" | Decision paralysis | Hand-written for learning. neverthrow for small projects. fp-ts/Effect for serious FP |

---

## Tóm tắt

Chương này giới thiệu hai hộp công cụ FP cho TypeScript — fp-ts (truyền thống, modular) và Effect (hiện đại, batteries-included).

- ✅ **Self-written → Library**: Tự viết để hiểu, library cho production.
- ✅ **fp-ts Option**: `None | Some<A>`. Replaces null/undefined.
- ✅ **fp-ts Either**: `Left<E> | Right<A>`. Replaces Result/try-catch.
- ✅ **fp-ts Task/TaskEither**: Lazy async. `() => Promise<Either<E, A>>`.
- ✅ **Effect**: `Effect<A, E, R>` — unified type. Generator syntax (`yield*`).
- ✅ **Effect.gen**: Write imperative, get functional. Error types auto-union.
- ✅ **Decision**: neverthrow (simple) → fp-ts (modular FP) → Effect (full framework).

## Tiếp theo

→ Chapter 26: **Functors & Map** — what IS a Functor? Functor laws. `Option.map`, `Either.map`, `Array.map`, `Task.map` — they're ALL functors!
