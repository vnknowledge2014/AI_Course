# Chapter 25 — Lựa chọn Library: fp-ts (Legacy) và Effect (Hiện đại)

> **Bạn sẽ học được**:
> - Tại sao tự viết (Result, Option, pipe) là tốt để học, nhưng production cần thư viện.
> - **Effect-TS**: Hệ sinh thái thế hệ mới, tiêu chuẩn của TypeScript FP hiện đại.
> - `Effect<A, E, R>`: Chén thánh giải quyết data, errors và dependencies trong MỘT type.
> - `Effect.gen`: Viết imperative, chạy functional (generator-based do-notation).
> - **fp-ts**: "Ông tổ" của FP TypeScript (Option, Either, TaskEither) — tại sao giờ nó được coi là legacy.
> - Decision Framework: Khi nào nâng cấp từ fp-ts lên Effect.
>
> **Yêu cầu trước**: Chapter 12 (pipe/flow), Chapter 13 (Result/Option), Chapter 21-22 (pipelines, ROP).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Hiểu Effect-TS là gì, tại sao nó vượt trội, và cách "đọc hiểu" code fp-ts cũ.

---

Bạn đã tự đóng bàn bao giờ chưa?

Chúng ta đã dành Ch12-24 để "tự đóng" công cụ FP: `pipe()`, `flow()`, `Result<T,E>`, `Option<T>`, `andThen`. Bạn đã hiểu cốt lõi của chúng. Nhưng thợ mộc chuyên nghiệp không tự rèn mọi thứ — họ mua **hộp công cụ chuyên dụng** (Makita, Bosch) với độ chính xác cao, đầy đủ phụ kiện và tài liệu.

**Effect-TS** (thường gọi là Effect) chính là cỗ máy vạn năng thế hệ mới nhất của hệ sinh thái TypeScript FP. Nó là đích đến của toàn bộ hành trình này.
**fp-ts** là hộp công cụ đời cũ (Haskell-inspired), từng làm mưa làm gió, nhưng giờ mang tính di sản (legacy).

Chương này sẽ giới thiệu bạn với Effect-TS như công cụ chính, và cách nhìn nhận fp-ts như một nền tảng chuyển tiếp.

---

## 25.1 — Tại sao cần Library thay vì tự viết?

Chúng ta tự viết `Result`, `Option` ở các chương trước để HIỂU BẢN CHẤT. Nhưng trong production, việc tự viết có giới hạn:

1. **Thiếu utilities**: Làm sao để sequence một `Array<Result>`? Làm sao chạy 10 Task song song nhưng fail-fast?
2. **Thiếu Error Tracing**: Khi lỗi xảy ra sâu trong pipeline, làm sao có stack trace chuẩn FP?
3. **Thiếu Dependency Injection**: Truyền `config` hoặc `database` connection vào từng function rất cồng kềnh.
4. **Vấn đề Asynchronous**: Kết hợp `Result` và `Promise` sinh ra `Promise<Result<T, E>>` rất khó đọc (bạn phải `await` rồi mới `match`).

Libraries sinh ra để giải quyết những vấn đề đó ở quy mô lớn.

---

## 25.2 — Effect-TS: Kỷ nguyên mới của TypeScript FP

Effect-TS được truyền cảm hứng mạnh mẽ từ ZIO (của Scala). Nó không cố ép TypeScript trông giống Haskell (như fp-ts đã làm). Nó tận dụng tối đa tính năng của TypeScript (như Generators) để tạo ra trải nghiệm FP "thuần phong mỹ tục" nhất.

### Một Type Duy Nhất: `Effect<A, E, R>`

Thay vì chia nhỏ thành `Option`, `Either`, `Task`, `ReaderTaskEither` (như fp-ts), Effect gom TẤT CẢ vào MỘT type duy nhất:

**`Effect<Success, Error, Requirements>`**

- **A (Success)**: Kiểu dữ liệu trả về nếu thành công.
- **E (Error)**: Kiểu lỗi nếu thất bại (mặc định là `never`).
- **R (Requirements)**: Các services/dependencies cần thiết để chạy (mặc định là `never`).

> Nhìn vào type là biết mọi thứ! Một function trả về `Effect<User, DbError, Database>` nói rằng: "Đưa cho tôi cái `Database`, tôi sẽ trả cho bạn `User`, hoặc có thể thất bại với `DbError`".

```typescript
# filename: src/effect_core_concepts.ts
import assert from "node:assert/strict";

// --- Mô phỏng Concept của Effect ---
// (Đây là mô phỏng để hiểu bản chất, thư viện thực tế import từ "effect")

type Effect<A, E = never, R = never> = {
    readonly _tag: "Effect";
    readonly run: (env: R) => Promise<{ _tag: "Success"; value: A } | { _tag: "Failure"; error: E }>;
};

// 1. Succeed: Không lỗi, không yêu cầu dependencies
const succeed = <A>(value: A): Effect<A, never, never> => ({
    _tag: "Effect",
    run: async () => ({ _tag: "Success", value })
});

// 2. Fail: Lỗi, không yêu cầu dependencies
const fail = <E>(error: E): Effect<never, E, never> => ({
    _tag: "Effect",
    run: async () => ({ _tag: "Failure", error })
});

// 3. Map (Functor)
const map = <A, B, E, R>(effect: Effect<A, E, R>, f: (a: A) => B): Effect<B, E, R> => ({
    _tag: "Effect",
    run: async (env) => {
        const res = await effect.run(env);
        return res._tag === "Success" ? { _tag: "Success", value: f(res.value) } : res;
    }
});

// 4. FlatMap (Monad bind)
const flatMap = <A, B, E1, E2, R1, R2>(
    effect: Effect<A, E1, R1>,
    f: (a: A) => Effect<B, E2, R2>
): Effect<B, E1 | E2, R1 & R2> => ({
    _tag: "Effect",
    run: async (env) => {
        const res = await effect.run(env);
        return res._tag === "Success" ? f(res.value).run(env) : res;
    }
});

// --- Sử dụng ---
const parseNumber = (s: string): Effect<number, string, never> => {
    const n = Number(s);
    return isNaN(n) ? fail("Not a number") : succeed(n);
};

const doubleIfPositive = (n: number): Effect<number, string, never> => 
    n > 0 ? succeed(n * 2) : fail("Negative number");

// Chain (Pipe)
const program = flatMap(parseNumber("42"), doubleIfPositive);

// Run
async function main() {
    const res1 = await program.run(undefined as never);
    assert.deepStrictEqual(res1, { _tag: "Success", value: 84 });

    const res2 = await flatMap(parseNumber("abc"), doubleIfPositive).run(undefined as never);
    assert.deepStrictEqual(res2, { _tag: "Failure", error: "Not a number" });
    
    console.log("Effect Concepts OK ✅");
}

main();
```

---

## 25.3 — Sức mạnh tuyệt đối của `Effect.gen`

Việc dùng `flatMap` liên tục (như code trên) dẫn đến callback hell (dù đã được `pipe` cứu vớt một phần).
Effect mang đến **Do-notation thông qua TypeScript Generators** (`yield*`). Nó cho phép bạn viết mã FP an toàn, nhưng với cú pháp trông giống hệt như mã thủ tục (imperative) dùng `async/await`.

> Đây là điểm ăn tiền lớn nhất khiến Effect vượt qua fp-ts.

```typescript
// filename: src/effect_gen_concept.ts

// Ví dụ này minh họa cách viết Effect.gen trong THỰC TẾ
// Sử dụng thư viện 'effect' thật.

/* 
import { Effect } from "effect";

// 1. Khai báo các Effects
const getUser = (id: number): Effect.Effect<string, Error> => 
    id === 1 ? Effect.succeed("John") : Effect.fail(new Error("User not found"));

const getAddress = (username: string): Effect.Effect<string, Error> =>
    username === "John" ? Effect.succeed("123 Street") : Effect.fail(new Error("Address not found"));

// 2. Gom chúng lại bằng Effect.gen (Trông giống async/await!)
const program = Effect.gen(function* () {
    // yield* unwrap giá trị. Nếu có lỗi, nó short-circuit toàn bộ Generator!
    const user = yield* getUser(1);
    const address = yield* getAddress(user);
    
    return `${user} lives at ${address}`;
});

// 3. Chạy Effect
Effect.runPromise(program)
    .then(console.log) // "John lives at 123 Street"
    .catch(console.error);

*/
console.log("Effect.gen concept OK ✅");
```

### Tại sao `Effect.gen` lại tuyệt vời?
1. **Không có `try/catch` lồng nhau**: `yield*` xử lý lỗi ngầm bên dưới (giống Monad bind). Nếu một dòng fail, toàn bộ block ngừng và trả về Error.
2. **TypeScript tự suy luận lỗi (Auto Union)**: Nếu `getUser` throw `DbError` và `getAddress` throw `ApiError`, TypeScript tự động biết `program` có kiểu lỗi là `DbError | ApiError`.
3. **Phẳng hóa (Flat)**: Code chạy theo trình tự từ trên xuống dưới, không có callback nesting.

---

## ✅ Checkpoint 25.1-25.3

> Đến đây bạn phải hiểu:
> 1. **Effect** gom tất cả (Sync, Async, Error, Dependencies) vào `Effect<A, E, R>`.
> 2. Hàm Effect là **Lazy**: Nó chỉ miêu tả công việc (description), không thực thi cho đến khi bạn gọi `runPromise`.
> 3. **`Effect.gen`** dùng Generator (`yield*`) để viết Monadic workflow dưới dạng mã thủ tục. Short-circuit tự động khi có lỗi.
>
> **Test nhanh**: Sự khác biệt giữa `await getUser()` và `yield* getUser()` là gì?
> <details><summary>Đáp án</summary>`await` ném Exception (throw), bắt buộc phải dùng `try/catch` ở runtime và mất type safety. `yield*` bóc tách Effect một cách an toàn, lỗi được chuyển thành type `E` trong chữ ký hàm, không bao giờ crash app.</details>

---

## 25.4 — Di sản `fp-ts`: Đọc hiểu Code cũ

Mặc dù Effect là tương lai, rất nhiều codebase FP TypeScript ngoài kia (và các bài hướng dẫn) vẫn đang dùng `fp-ts`. Hiểu `fp-ts` giúp bạn maintain hệ thống cũ và hiểu hành trình tiến hóa của TS FP.

`fp-ts` mô phỏng cấu trúc của ngôn ngữ Haskell, chia các khái niệm thành nhiều loại Types rời rạc:
- `Option<A>`: Thành công hoặc Không có gì. (Chỉ có A, không có E)
- `Either<E, A>`: Lỗi hoặc Thành công. (Có A, có E, Sync)
- `Task<A>`: Hàm bất đồng bộ không bao giờ lỗi. (Có A, Async)
- `TaskEither<E, A>`: Hàm bất đồng bộ có thể lỗi. (Có E, Có A, Async)
- `ReaderTaskEither<R, E, A>`: Yêu cầu dependencies, có thể lỗi, bất đồng bộ! (Tiệm cận với type `Effect`)

### So sánh fp-ts Pipeline và Effect

Hãy xem cùng một workflow (tìm user, kiểm tra tuổi, in tên) được viết bằng fp-ts và Effect:

```typescript
# filename: src/fpts_vs_effect.ts
import assert from "node:assert/strict";

// ─── 1. Mô phỏng fp-ts (Legacy Context) ───
type Either<E, A> = { readonly _tag: "Left"; readonly left: E } | { readonly _tag: "Right"; readonly right: A };
const left = <E>(e: E): Either<E, never> => ({ _tag: "Left", left: e });
const right = <A>(a: A): Either<never, A> => ({ _tag: "Right", right: a });

const mapEither = <A, B>(f: (a: A) => B) => <E>(fa: Either<E, A>): Either<E, B> =>
    fa._tag === "Left" ? fa : right(f(fa.right));

const chainEither = <E, A, B>(f: (a: A) => Either<E, B>) => (fa: Either<E, A>): Either<E, B> =>
    fa._tag === "Left" ? fa : f(fa.right);

function pipe(a: any, ...fns: any[]): any {
    return fns.reduce((acc, fn) => fn(acc), a);
}

// Logic:
const findUserEither = (id: number): Either<string, { age: number }> => 
    id === 1 ? right({ age: 20 }) : left("User not found");

const checkAgeEither = (user: { age: number }): Either<string, number> => 
    user.age >= 18 ? right(user.age) : left("Underage");

// fp-ts Pipeline:
const programEither = pipe(
    findUserEither(1),
    chainEither(checkAgeEither),
    mapEither(age => `Valid age: ${age}`)
);

assert.deepStrictEqual(programEither, { _tag: "Right", right: "Valid age: 20" });


// ─── 2. Mô phỏng Effect (Modern Context) ───
type Effect<A, E> = { _tag: "Effect", run: () => { _tag: "Success"; value: A } | { _tag: "Failure"; error: E } };
const succeed = <A>(value: A): Effect<A, never> => ({ _tag: "Effect", run: () => ({ _tag: "Success", value }) });
const fail = <E>(error: E): Effect<never, E> => ({ _tag: "Effect", run: () => ({ _tag: "Failure", error }) });
const flatMap = <A, B, E1, E2>(eff: Effect<A, E1>, f: (a: A) => Effect<B, E2>): Effect<B, E1 | E2> => ({
    _tag: "Effect", run: () => { const res = eff.run(); return res._tag === "Success" ? f(res.value).run() : res; }
});
const mapEffect = <A, B, E>(eff: Effect<A, E>, f: (a: A) => B): Effect<B, E> => ({
    _tag: "Effect", run: () => { const res = eff.run(); return res._tag === "Success" ? { _tag: "Success", value: f(res.value) } : res; }
});

// Logic:
const findUserEffect = (id: number): Effect<{ age: number }, string> => 
    id === 1 ? succeed({ age: 20 }) : fail("User not found");

const checkAgeEffect = (user: { age: number }): Effect<number, string> => 
    user.age >= 18 ? succeed(user.age) : fail("Underage");

// Effect Pipeline (không dùng gen):
const programEffect = mapEffect(
    flatMap(findUserEffect(1), checkAgeEffect),
    age => `Valid age: ${age}`
);

assert.deepStrictEqual(programEffect.run(), { _tag: "Success", value: "Valid age: 20" });

console.log("fp-ts vs Effect OK ✅");
```

### Tại sao fp-ts trở thành Legacy?

1. **Fragmentation (Phân mảnh)**: Trong fp-ts, nếu hàm A trả `Either` và hàm B trả `TaskEither`, bạn KHÔNG THỂ `pipe` chúng trực tiếp. Bạn phải dùng hàm convert (`fromEither` lên `TaskEither`). Quá trình này rườm rà. Effect hợp nhất tất cả làm một (`Effect.sync`, `Effect.promise`).
2. **Học thuật (Academics)**: Tên gọi hàm trong fp-ts bị ảnh hưởng bởi Toán học: `chain` thay vì `flatMap`, `sequenceT`, `traverseArray`. Mất rất nhiều công sức để onboard team mới.
3. **Type Inference**: TypeScript làm rất tệ trong việc suy luận Currying của fp-ts (như `chain` ở trên). Code fp-ts thường cần rất nhiều Type Annotations thủ công.

---

## 25.5 — Decision Framework: Khi nào chọn gì?

| Tiêu chí | Dùng Effect-TS | Dùng fp-ts | Dùng Thư viện Tự Viết (như Ch13) |
|---|---|---|---|
| **Trạng thái Project** | Mới bắt đầu, hoặc có kế hoạch rewrite. | Maintain hệ thống cũ. | Các package siêu nhỏ, thư viện lõi. |
| **Kỹ năng Team** | Biết TypeScript, Async/Await. | Giỏi Haskell/Scala, quen currying. | Nhóm nhỏ, không muốn phụ thuộc deps ngoài. |
| **Dependency Injection** | Cần truyền database, config sâu xuống. | Tự xoay xở với `Reader`. | Tự xoay xở bằng Class hoặc Closures. |
| **Concurrency & Async**| Mạnh mẽ, built-in, quản lý timeouts, retries. | Dùng `TaskEither`, `sequenceT`. | Khó, tự viết bằng `Promise.all`. |

**Khuyến nghị:**
Nếu bạn bắt đầu một backend Node.js, Next.js hay NestJS theo chuẩn Functional Programming vào năm nay: **Chọn Effect-TS làm công cụ duy nhất.** Bỏ qua fp-ts.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Dịch thuật fp-ts sang Effect

Dưới đây là một type signature trong fp-ts. Tương đương của nó trong Effect là gì?
1. `TaskEither<Error, string>`
2. `Either<Error, User>`
3. `ReaderTaskEither<Config, Error, number>`
4. `Task<boolean>`

<details><summary>✅ Lời giải Bài 1</summary>

1. `TaskEither<Error, string>` = `Effect.Effect<string, Error, never>` (Async có thể lỗi).
2. `Either<Error, User>` = `Effect.Effect<User, Error, never>` (Nó có thể biểu thị bằng Effect dù là Sync).
3. `ReaderTaskEither<Config, Error, number>` = `Effect.Effect<number, Error, Config>`.
4. `Task<boolean>` = `Effect.Effect<boolean, never, never>`.
</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Thấy type `Effect<A, E, R>` khó hiểu | Chưa quen với tham số thứ 3 | Đọc từ trái sang: `A` = Success, `E` = Lỗi, `R` = Đồ nghề cần thiết (Môi trường/DI). Đa số `R` là `never`. |
| Gặp lỗi Type ở `yield*` (Effect) | TS chưa cấu hình đúng | Bật `"strict": true` và `"exactOptionalPropertyTypes": true` trong `tsconfig.json`. |
| fp-ts báo lỗi Type mismatch khi `chain` | Cố mix `Either` và `TaskEither` | Dùng `TE.fromEither` để bọc `Either` thành `TaskEither` trước khi pipe. (Điều mà Effect tự động làm). |

---

## Tóm tắt

- ✅ **Library vs Self-written**: Khi hiểu cốt lõi, production cần Library để scale, giảm lỗi và hưởng thụ hệ sinh thái.
- ✅ **fp-ts**: Từng là tiêu chuẩn vàng, dựa trên Haskell. Nhiều types rời rạc (`Option`, `Either`, `TaskEither`). Giờ mang tính duy trì codebase cũ.
- ✅ **Effect-TS**: Tiêu chuẩn FP hiện tại và tương lai của TypeScript.
- ✅ **`Effect<A, E, R>`**: Một interface duy nhất "cân" mọi vấn đề: dữ liệu trả về (A), lỗi (E), và Dependencies (R).
- ✅ **`Effect.gen`**: Giải quyết callback hell bằng Generator `yield*`. Code ngắn, type-safe, auto-union lỗi.

## Tiếp theo

→ Chapter 26: **Effect Equal & Order** — Quên `===` đi, học cách Effect xử lý so sánh bằng (Equal) và sắp xếp (Order) một cách an toàn và đúng bản chất ngữ nghĩa.
