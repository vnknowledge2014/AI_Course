# Appendix A — fp-ts ↔ Effect Comparison

> Hai thư viện FP chính trong TypeScript: `fp-ts` (battle-tested, community standard) và `Effect` (next-gen, batteries-included). Bảng so sánh giúp bạn chọn đúng tool và migrate giữa chúng.

---

## Tổng quan

| Tiêu chí | fp-ts | Effect |
|----------|-------|--------|
| **Tác giả** | Giulio Canti | Effect Contributors (fork từ fp-ts) |
| **Triết lý** | Port Haskell typeclasses sang TS | "Practical FP" — batteries-included |
| **Learning curve** | Cao (cần hiểu HKTs, typeclasses) | Trung bình (API thân thiện hơn) |
| **Bundle size** | Nhỏ (tree-shakable) | Lớn hơn (all-in-one) |
| **Async** | `TaskEither` | `Effect` (built-in) |
| **DI** | `Reader` / `ReaderTaskEither` | `Layer` / `Context` (first-class) |
| **Error handling** | `Either<E, A>` | `Effect<A, E, R>` (3 type params) |
| **Concurrency** | Manual (`Task`) | Built-in (`Effect.fork`, `Fiber`) |
| **Tracing/Metrics** | Không có | Built-in (OpenTelemetry) |
| **Production readiness** | Mature, ít updates | Active development, breaking changes |

---

## Type Mapping

### Core Types

| Concept | fp-ts | Effect |
|---------|-------|--------|
| **Optional value** | `Option<A>` | `Option.Option<A>` |
| **Error or value** | `Either<E, A>` | `Either.Either<A, E>` (note: params swapped!) |
| **Sync computation** | `IO<A>` | `Effect.Effect<A>` |
| **Async computation** | `Task<A>` | `Effect.Effect<A>` |
| **Async + Error** | `TaskEither<E, A>` | `Effect.Effect<A, E>` |
| **With dependencies** | `ReaderTaskEither<R, E, A>` | `Effect.Effect<A, E, R>` |
| **Validation** | `These<E, A>` | `Effect` with `Effect.validate` |

### Constructors

```typescript
// === fp-ts ===
import * as E from "fp-ts/Either";
import * as TE from "fp-ts/TaskEither";
import { pipe } from "fp-ts/function";

const result = pipe(
    E.right(42),
    E.map(n => n * 2),
    E.chain(n => n > 0 ? E.right(n) : E.left("negative"))
);

// === Effect ===
import { Effect, pipe } from "effect";

const result = pipe(
    Effect.succeed(42),
    Effect.map(n => n * 2),
    Effect.flatMap(n => n > 0 ? Effect.succeed(n) : Effect.fail("negative"))
);
```

### Pipeline / Composition

```typescript
// === fp-ts ===
import { pipe, flow } from "fp-ts/function";
import * as TE from "fp-ts/TaskEither";

const program = pipe(
    TE.of(input),
    TE.chain(validate),
    TE.chain(process),
    TE.map(format)
);

// === Effect ===
import { Effect, pipe } from "effect";

const program = pipe(
    Effect.succeed(input),
    Effect.flatMap(validate),
    Effect.flatMap(process),
    Effect.map(format)
);

// Effect also supports generator syntax (do-notation):
const program2 = Effect.gen(function* () {
    const validated = yield* validate(input);
    const processed = yield* process(validated);
    return format(processed);
});
```

### Error Handling

```typescript
// === fp-ts: fold/match ===
import * as E from "fp-ts/Either";
import { pipe } from "fp-ts/function";

pipe(
    result,
    E.fold(
        error => console.error(error),
        value => console.log(value)
    )
);

// === Effect: match/catchAll ===
import { Effect, pipe } from "effect";

pipe(
    effect,
    Effect.match({
        onFailure: error => console.error(error),
        onSuccess: value => console.log(value)
    })
);
```

### Dependency Injection

```typescript
// === fp-ts: Reader ===
import * as RTE from "fp-ts/ReaderTaskEither";

type Env = { db: Database; logger: Logger };
const getUser = (id: string): RTE.ReaderTaskEither<Env, Error, User> =>
    (env) => env.db.findUser(id);

// === Effect: Layer + Context ===
import { Effect, Context, Layer } from "effect";

class Database extends Context.Tag("Database")<Database, { findUser: (id: string) => Effect.Effect<User, Error> }>() {}

const getUser = (id: string) =>
    Effect.flatMap(Database, db => db.findUser(id));

const DatabaseLive = Layer.succeed(Database, { findUser: /* impl */ });
```

---

## Khi nào dùng cái nào?

| Tình huống | Khuyên dùng | Lý do |
|------------|-------------|-------|
| **Project mới, team nhỏ** | Effect | API đơn giản hơn, DI built-in |
| **Project có sẵn fp-ts** | fp-ts | Không cần migrate |
| **Cần concurrency/fibers** | Effect | Built-in fiber system |
| **Cần bundle nhỏ** | fp-ts | Tree-shakable, nhẹ hơn |
| **Learning FP concepts** | fp-ts | Gần Haskell hơn, tài liệu nhiều |
| **Production microservices** | Effect | Observability, DI, retry built-in |

---

> **💡 Lời khuyên**: Đừng lo chọn sai. Cả hai đều dạy cùng mental model (Functor, Monad, pipe). Hiểu fp-ts → chuyển Effect dễ dàng. Hiểu Effect → đọc fp-ts code được. Concepts > Libraries.
