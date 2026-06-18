# Appendix B — From F#/PureScript to TypeScript

> Bảng dịch thuật cho developers đến từ F#, PureScript, Haskell. Tìm khái niệm quen thuộc → TypeScript equivalent.

---

## Type System

| Concept | F# | PureScript/Haskell | TypeScript |
|---------|----|--------------------|------------|
| **Type alias** | `type Age = int` | `type Age = Int` | `type Age = number` |
| **Record** | `type Person = { Name: string; Age: int }` | `type Person = { name :: String, age :: Int }` | `type Person = { readonly name: string; readonly age: number }` |
| **Discriminated Union** | `type Shape = Circle of float \| Rect of float * float` | `data Shape = Circle Number \| Rect Number Number` | `type Shape = { tag: "circle"; r: number } \| { tag: "rect"; w: number; h: number }` |
| **Generic type** | `'a list` | `List a` | `Array<T>` |
| **Constrained generic** | `'a when 'a :> IComparable` | `class Ord a => ...` | `<T extends Comparable>` |
| **Unit** | `unit` / `()` | `Unit` | `void` / `undefined` |
| **Option** | `Option<'a>` / `Some x` / `None` | `Maybe a` / `Just a` / `Nothing` | `Option<A>` (fp-ts) / `Option.Option<A>` (Effect) |
| **Result** | `Result<'a, 'e>` | `Either e a` | `Either<E, A>` (fp-ts) / `Result<A, E>` (neverthrow) |
| **Newtype** | `type Email = Email of string` | `newtype Email = Email String` | `type Email = string & { __brand: "Email" }` (branded type) |

---

## Functions & Composition

| Concept | F# | PureScript/Haskell | TypeScript |
|---------|----|--------------------|------------|
| **Function** | `let add x y = x + y` | `add x y = x + y` | `const add = (x: number, y: number) => x + y` |
| **Lambda** | `fun x -> x + 1` | `\x -> x + 1` | `(x: number) => x + 1` |
| **Pipe** | `x \|> f \|> g` | `x # f # g` (PureScript) | `pipe(x, f, g)` (fp-ts/Effect) |
| **Compose** | `f >> g` | `f >>> g` / `g <<< f` | `flow(f, g)` (fp-ts) |
| **Partial application** | `let add5 = add 5` (auto-curried) | `add5 = add 5` (auto-curried) | Manual: `const add5 = (y: number) => add(5, y)` |
| **Pattern match** | `match x with \| Some v -> ... \| None -> ...` | `case x of Just v -> ... Nothing -> ...` | `switch (x.tag) { case "some": ... case "none": ... }` |
| **Exhaustive check** | Compiler warns on missing cases | Compiler error | `assertNever(x)` in `default:` |

---

## FP Patterns

| Concept | F# | PureScript/Haskell | TypeScript |
|---------|----|--------------------|------------|
| **Functor** | `.map` on collections | `class Functor f where fmap :: (a -> b) -> f a -> f b` | `Option.map(f)` / `Either.map(f)` (per-type, no HKTs) |
| **Monad** | Computation expressions `let! x = ...` | `do x <- action; ...` | `Effect.gen(function* () { const x = yield* action; ... })` |
| **Applicative** | N/A (implicit) | `liftA2 f a b` | `Effect.all([a, b])` / `sequenceT(E.Apply)(a, b)` |
| **Monoid** | Not built-in | `class Monoid m where mempty :: m; mappend :: m -> m -> m` | `{ concat: (a, b) => ..., empty: ... }` (fp-ts Monoid) |
| **Railway (ROP)** | `Result.bind` / `>>=` | `Either` + `do` notation | `pipe(result, E.chain(f), E.chain(g))` / `Effect.flatMap` |
| **IO** | Side effects everywhere | `IO a` / `Effect a` | `Effect.Effect<A>` (Effect) / `IO<A>` (fp-ts) |

---

## Module System

| Concept | F# | PureScript | TypeScript |
|---------|----|-----------|-----------| 
| **Module** | `module MyModule` | `module MyModule where` | `// filename = module` |
| **Import** | `open MyModule` | `import MyModule (fn1, fn2)` | `import { fn1, fn2 } from "./myModule"` |
| **Export** | Public by default | Explicit exports | `export const fn1 = ...` |
| **Visibility** | `internal` / `private` | Not exported = private | Not exported = private |
| **Barrel file** | N/A | N/A | `export * from "./submodule"` in `index.ts` |

---

## DDD Patterns

| Pattern | F# | TypeScript |
|---------|----|-----------| 
| **Value Object** | `type Email = Email of string` | `type Email = string & { __brand: "Email" }` |
| **Smart Constructor** | `let create (s: string) = if valid s then Some (Email s) else None` | `const createEmail = (s: string): Result<Email, Error> => ...` |
| **Workflow** | `let placeOrder = validate >> price >> acknowledge` | `const placeOrder = flow(validate, flatMap(price), flatMap(acknowledge))` |
| **State Machine** | `type OrderState = Draft \| Confirmed \| Shipped` | `type OrderState = { tag: "draft" } \| { tag: "confirmed"; at: Date } \| { tag: "shipped"; tracking: string }` |
| **Anti-corruption Layer** | Manual mapping functions | `Zod` schema + `parse` |

---

## Testing

| Concept | F# | TypeScript |
|---------|----|-----------| 
| **Unit test** | `[<Fact>] let ``test name`` () = ...` (xUnit) | `test("name", () => { ... })` (vitest/jest) |
| **Assertion** | `Assert.Equal(expected, actual)` | `assert.strictEqual(actual, expected)` |
| **PBT** | FsCheck | fast-check |
| **Mocking** | Manual / Foq | Manual (FP DI) / vitest.mock |

---

> **💡 Key insight**: TypeScript thiếu auto-currying và HKTs (Higher-Kinded Types) so với F#/Haskell. Điều này có nghĩa Functor/Monad phải được implement per-type (không có abstract typeclass). Nhưng TypeScript BÙ LẠI bằng: structural typing (không cần declare interface trước), discriminated unions (pattern matching), và ecosystem lớn nhất thế giới.
