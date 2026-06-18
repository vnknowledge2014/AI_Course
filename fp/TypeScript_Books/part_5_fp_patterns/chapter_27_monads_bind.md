# Chapter 27 — Monads & Bind

> **Bạn sẽ học được**:
> - Monad là gì — giải quyết vấn đề nested containers từ Ch26
> - `chain`/`flatMap` = map + flatten. The missing piece
> - Monad laws — 3 luật đảm bảo chain hoạt động đúng
> - Do-notation / Effect.gen — imperative-looking monadic code
> - Option, Either, Array, Task — ĐỀU là Monads
>
> **Yêu cầu trước**: Chapter 26 (Functors, map, nested containers problem).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Hiểu Monad — không còn sợ từ này. Và biết bạn đã DÙNG nó suốt Ch21-24.

---

Bạn nhớ vấn đề cuối Ch26 không? `Option<Option<string>>` — hộp trong hộp. Bạn muốn giá trị bên trong, nhưng bị KẸT hai lớp vỏ.

Nghĩ thế này: bạn gửi thư cho bạn bè qua bưu điện. Bạn bỏ thư vào phong bì (lớp 1). Bưu điện bỏ phong bì của bạn vào bao thư lớn (lớp 2). Bạn bè nhận bao thư, mở ra, thấy phong bì, mở tiếp — rồi mới đọc thư. Phiền! Lý tưởng: bưu điện **là phẳng** (flatten) — bạn bè nhận TRỰC TIẾP phong bì của bạn, không cần bao ngoài.

**Monad** = Functor + **flatten**. `chain` (hay `flatMap`) = `map` rồi `flatten`. Thay vì `Option<Option<A>>`, chain cho bạn `Option<A>` — gỡ bỏ lớp vỏ thừa. Đây là reason Monad TỒN TẠI — nó giải quyết vấn đề THỰC TẾ, không phải toán trừu tượng.

Và đây là bí mật: bạn đã DÙNG Monad suốt Ch21-24 mà không biết! `andThen` từ Ch21 chính là `chain`. `Array.flatMap()` chính là `chain`. `Promise.then()` cũng là `chain` (auto-flatten khi callback return Promise). Tên khác, pattern GIỐNG. Chương này sẽ chỉ cho bạn TÊN của thứ bạn đã biết.

---

## Monads & Bind — Chain dependent computations

`flatMap` / `chain` / `andThen` = Monad. Kết quả của step 1 quyết định step 2. Nếu step 1 fail → skip tất cả steps sau. Đây là "railway" của ROP (Ch22) ở level type class. `async/await` = do-notation của Promise Monad.


## 27.1 — The Envelope Problem

### map tạo ra hộp trong hộp

Hãy bắt đầu bằng ví dụ cụ thể. Bạn có 3 lookups: tìm user, lấy address ID từ user, tìm address từ ID. Mỗi lookup có thể THẤT BẠI (return `None`). Với chỉ `map` (Functor), mỗi lần bạn đưa function return container vào map, bạn thêm MỘT lớp vỏ. Ba lần = ba lớp: `Option<Option<Option<Address>>>`. Vô dụng!

```typescript
// filename: src/nesting_problem.ts
import assert from "node:assert/strict";

type Option<A> = { _tag: "None" } | { _tag: "Some"; value: A };
const none: Option<never> = { _tag: "None" };
const some = <A>(v: A): Option<A> => ({ _tag: "Some", value: v });

const map = <A, B>(fn: (a: A) => B) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : some(fn(fa.value));

const fromNullable = <A>(v: A | null | undefined): Option<A> =>
    v == null ? none : some(v);

// Domain
type User = { name: string; addressId?: string };
type Address = { street: string; city: string };

const users = new Map([["U1", { name: "An", addressId: "A1" }], ["U2", { name: "Bình" }]]);
const addresses = new Map([["A1", { street: "123 Lê Lợi", city: "HCMC" }]]);

const findUser = (id: string): Option<User> => fromNullable(users.get(id));
const findAddress = (id: string): Option<Address> => fromNullable(addresses.get(id));
const getAddressId = (user: User): Option<string> => fromNullable(user.addressId);

// ❌ With MAP only: nested!
const userAddress_map = (userId: string): Option<Option<Option<Address>>> =>
    map(
        (user: User) => map(
            (addrId: string) => findAddress(addrId)
        )(getAddressId(user))
    )(findUser(userId));

// Result: Option<Option<Option<Address>>> — THREE layers! 😱
// Unusable. We want Option<Address>.

console.log("Nesting problem shown ✅");
```

Ba lớp Option lồng nhau. Để lấy được `Address`, bạn phải unwrap BA lần. Rồi handle các case: outer None? middle None? inner None? Tổng cộng 2³ = 8 cases. Đây chính là vấn đề mà `chain` giải quyết.

### chain = map + flatten

So sánh: `map(fn)(Some(x))` = `Some(fn(x))` — LUÔN bọc thêm một lớp `Some`. Nhưng nếu `fn` đã return `Option` rồi? Kết quả: `Some(Option<B>)` = lồng! `chain(fn)(Some(x))` = `fn(x)` — dùng kết quả của `fn` TRỰC TIẾP. Không bọc thêm. Phẳng.

Khác biệt chỉ MỘT dòng code, nhưng hệ quả KHỔNG LỒ: map bọc `some(fn(x))`, chain dùng được `fn(x)`. Đó là tất cả thỡi!

```typescript
// filename: src/chain_solution.ts
import assert from "node:assert/strict";

type Option<A> = { _tag: "None" } | { _tag: "Some"; value: A };
const none: Option<never> = { _tag: "None" };
const some = <A>(v: A): Option<A> => ({ _tag: "Some", value: v });

const fromNullable = <A>(v: A | null | undefined): Option<A> =>
    v == null ? none : some(v);

// chain = flatMap = bind
// If Some → apply function (which returns Option). Result is FLAT Option.
// If None → return None.
const chain = <A, B>(fn: (a: A) => Option<B>) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : fn(fa.value);
//                               ^^^^^^^^^^^
// Key: fn(fa.value) already returns Option<B>
// map would wrap it: Some(Option<B>) = Option<Option<B>> ← nested!
// chain uses fn's result DIRECTLY = Option<B> ← flat!

const map = <A, B>(fn: (a: A) => B) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : some(fn(fa.value));
//                               ^^^^^^^^^^^^^^^^
// map wraps: some(fn(fa.value)) = Some(B) ← always wraps

function pipe<A>(a: A): A;
function pipe<A, B>(a: A, ab: (a: A) => B): B;
function pipe<A, B, C>(a: A, ab: (a: A) => B, bc: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D): D;
function pipe(a: unknown, ...fns: ((x: unknown) => unknown)[]): unknown {
    return fns.reduce((acc, fn) => fn(acc), a);
}

// Domain (same as before)
type User = { name: string; addressId?: string };
type Address = { street: string; city: string };

const users = new Map([["U1", { name: "An", addressId: "A1" }], ["U2", { name: "Bình" }]]);
const addresses = new Map([["A1", { street: "123 Lê Lợi", city: "HCMC" }]]);

const findUser = (id: string): Option<User> => fromNullable(users.get(id));
const findAddress = (id: string): Option<Address> => fromNullable(addresses.get(id));
const getAddressId = (user: User): Option<string> => fromNullable(user.addressId);

// ✅ With CHAIN: flat!
const userAddress = (userId: string): Option<Address> =>
    pipe(
        findUser(userId),                    // Option<User>
        chain(user => getAddressId(user)),   // Option<string>  ← FLAT!
        chain(addrId => findAddress(addrId)),// Option<Address> ← FLAT!
    );

// Tests
assert.deepStrictEqual(
    userAddress("U1"),
    some({ street: "123 Lê Lợi", city: "HCMC" }),
);

assert.deepStrictEqual(userAddress("U2"), none);    // user exists, no addressId
assert.deepStrictEqual(userAddress("U999"), none);  // user not found

// Mix map + chain:
const userCity = (userId: string): Option<string> =>
    pipe(
        findUser(userId),
        chain(getAddressId),
        chain(findAddress),
        map(addr => addr.city),  // map: Address → string (always succeeds)
    );

assert.deepStrictEqual(userCity("U1"), some("HCMC"));
assert.deepStrictEqual(userCity("U2"), none);

console.log("chain solution OK ✅");
```

Đọc lại: `pipe(findUser, chain(getAddressId), chain(findAddress))` — ba bước, data chảy xuống. Mỗi bước có thể fail (return None). Nếu bước 1 fail → bước 2, 3 được SKIP. Kết quả: `Option<Address>` — phẳng! Không lồng.

Đây chính xác là `andThen` từ Ch21-22. Bạn đã dùng pattern này suốt Part IV mà không biết nó gọi là Monad. Và quy tắc đơn giản: function return giá trị THUẦN → `map`. Function return CONTAINER → `chain`. Chỉ vậy thôi.

> **💡 map vs chain — the rule**: Function returns PLAIN value → use `map`. Function returns CONTAINER (Option, Either, etc.) → use `chain`. `map(fn)` wraps result. `chain(fn)` uses result directly (fn already wrapped it).

---

## ✅ Checkpoint 27.1

> Đến đây bạn phải hiểu:
> 1. **map** wraps: `map(fn)(Some(x)) = Some(fn(x))`. Creates nesting if fn returns container
> 2. **chain** doesn't wrap: `chain(fn)(Some(x)) = fn(x)`. Flat result
> 3. **When to use**: fn returns `A` → map. fn returns `Option<A>` (or Either, Task) → chain
> 4. **chain = andThen** (from Ch21-22). Same concept, Haskell name
>
> **Test nhanh**: `chain(x => some(x * 2))(some(5))` = ?
> <details><summary>Đáp án</summary>`some(10)`. chain applies fn to 5, fn returns `some(10)`. chain uses result directly = `some(10)`. If we used map: `some(some(10))` ← nested!</details>

---

## 27.2 — Monad = Functor + chain + of

### Định nghĩa chính thức

Monad xây dựng TRÊN Functor (Ch26). Nếu Functor = container + map, thì Monad = Functor + hai thứ nữa: `of` (đưa giá trị vào container) và `chain` (map + flatten). Ba thành phần, không hơn.

Điều bất ngờ: bạn sẽ thấy Array, Option, Either, và cả Promise — TẤT CẢ đều là Monads. `Array.flatMap` chính là `chain`. `Promise.then()` chính là `chain` (khi callback return Promise, `.then` auto-flattens — không bao giờ `Promise<Promise<A>>`). Bạn đã sống trong thế giới Monad suốt đời, chỉ chưa biết tên.

```typescript
// filename: src/monad_definition.ts
import assert from "node:assert/strict";

// Monad yêu cầu 3 thứ:
// 1. Functor: map :: (A → B) → F<A> → F<B>     ← from Ch26
// 2. of :: A → F<A>                              ← wrap value in container
// 3. chain :: (A → F<B>) → F<A> → F<B>          ← map + flatten

// Option Monad
type Option<A> = { _tag: "None" } | { _tag: "Some"; value: A };

const optionMonad = {
    of: <A>(a: A): Option<A> => ({ _tag: "Some", value: a }),

    map: <A, B>(fn: (a: A) => B) => (fa: Option<A>): Option<B> =>
        fa._tag === "None" ? { _tag: "None" } : { _tag: "Some", value: fn(fa.value) },

    chain: <A, B>(fn: (a: A) => Option<B>) => (fa: Option<A>): Option<B> =>
        fa._tag === "None" ? { _tag: "None" } : fn(fa.value),
};

// Either Monad
type Either<E, A> = { _tag: "Left"; left: E } | { _tag: "Right"; right: A };

const eitherMonad = {
    of: <A>(a: A): Either<never, A> => ({ _tag: "Right", right: a }),

    map: <A, B>(fn: (a: A) => B) => <E>(fa: Either<E, A>): Either<E, B> =>
        fa._tag === "Left" ? fa : { _tag: "Right", right: fn(fa.right) },

    chain: <E, A, B>(fn: (a: A) => Either<E, B>) => (fa: Either<E, A>): Either<E, B> =>
        fa._tag === "Left" ? fa : fn(fa.right),
};

// Array Monad (!)
const arrayMonad = {
    of: <A>(a: A): A[] => [a],

    map: <A, B>(fn: (a: A) => B) => (fa: A[]): B[] => fa.map(fn),

    chain: <A, B>(fn: (a: A) => B[]) => (fa: A[]): B[] => fa.flatMap(fn),
    // Array.flatMap = map + flatten! It IS chain!
};

// Array.flatMap = Monad chain
const result = arrayMonad.chain((x: number) => [x, x * 10])([1, 2, 3]);
assert.deepStrictEqual(result, [1, 10, 2, 20, 3, 30]);

// vs map (creates nested):
const nested = arrayMonad.map((x: number) => [x, x * 10])([1, 2, 3]);
assert.deepStrictEqual(nested, [[1, 10], [2, 20], [3, 30]]);  // Array<Array<number>>!

console.log("Monad definition OK ✅");
```

Chú ý dòng `Array.flatMap` — đó là `chain` của Array Monad! JavaScript đã có sẵn. Và hãy so sánh `map` vs `chain` trên Array: `map(fn)([1,2,3])` với `fn` return array → `[[1,10],[2,20],[3,30]]` (nested). `chain(fn)([1,2,3])` → `[1,10,2,20,3,30]` (flat). Chính xác pattern phong bì đầu chương: chain = map rồi gỡ bao ngoài.

> **💡 `Array.flatMap` = Monad chain!** Bạn đã dùng Monads mà không biết tên. `Promise.then()` cũng là chain (auto-flattens nested Promises). `andThen` từ Ch21 = chain. Everywhere!

---

## 27.3 — Monad Laws

### Ba luật đảm bảo chain hoạt động đúng

Giống Functor có 2 laws (Ch26), Monad có 3 laws. Laws không phải toán cho vui — chúng đảm bảo bạn có thể refactor code tự do mà không đổi behavior.

Law 1 (Left identity): bọc giá trị vào container rồi chain = giống apply function trực tiếp. Container “trong suốt”.
Law 2 (Right identity): chain với hàm bọc (`of`) = không thay đổi. Gửi thư rồi nhận lại nguyên vẹn.
Law 3 (Associativity): chain A rồi chain B = chain (A rồi B). Thứ tự group không ảnh hưởng kết quả.

```typescript
// filename: src/monad_laws.ts
import assert from "node:assert/strict";

type Option<A> = { _tag: "None" } | { _tag: "Some"; value: A };
const none: Option<never> = { _tag: "None" };
const some = <A>(v: A): Option<A> => ({ _tag: "Some", value: v });

const of = some;
const chain = <A, B>(fn: (a: A) => Option<B>) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : fn(fa.value);

const double = (x: number): Option<number> => some(x * 2);
const addOneOpt = (x: number): Option<number> => x >= 0 ? some(x + 1) : none;

// === LAW 1: Left identity ===
// chain(f)(of(a)) === f(a)
// "Wrap in container then chain = just apply function"
assert.deepStrictEqual(chain(double)(of(5)), double(5));  // some(10) = some(10)

// === LAW 2: Right identity ===
// chain(of)(fa) === fa
// "Chain with wrapper = no change"
assert.deepStrictEqual(chain(of)(some(5)), some(5));
assert.deepStrictEqual(chain(of)(none), none);

// === LAW 3: Associativity ===
// chain(g)(chain(f)(fa)) === chain(x => chain(g)(f(x)))(fa)
// "Order of chaining doesn't matter"
const fa = some(5);
const way1 = chain(addOneOpt)(chain(double)(fa));           // chain then chain
const way2 = chain((x: number) => chain(addOneOpt)(double(x)))(fa); // nested chain
assert.deepStrictEqual(way1, way2);  // some(11) — (5*2)+1

console.log("Monad laws OK ✅");
```

| Law | Nói bằng lời | Ý nghĩa |
|-----|-------------|---------|
| **Left identity** | of(a) then chain(f) = f(a) | Container wrapping is transparent |
| **Right identity** | chain(of)(fa) = fa | Wrapping in chain = no-op |
| **Associativity** | chain order doesn't matter | Can restructure chain, still same result |

---

## 27.4 — Do-notation / Effect.gen

### Giải thoát khỏi chain hell

Nếu bạn có 5 bước chain, code bắt đầu trông như “callback hell” của Promise thời xưa: `chain(a => chain(b => chain(c => ...)))`. Đã có thuốc: **do-notation**. Haskell có syntax `do`. Effect có `Effect.gen(function*() {...})`. Cả hai cho phép viết code TRÔNG imperative (từng dòng, sequential) mà THỰC CHẤT là monadic chains.

Tưởng tượng: `async/await` là do-notation cho Promise monad. `yield*` trong Effect.gen là do-notation cho Effect monad. Cùng idea: viết như sequential, chạy như monadic.

```typescript
// filename: src/do_notation.ts
import assert from "node:assert/strict";

type Option<A> = { _tag: "None" } | { _tag: "Some"; value: A };
const none: Option<never> = { _tag: "None" };
const some = <A>(v: A): Option<A> => ({ _tag: "Some", value: v });
const fromNullable = <A>(v: A | null | undefined): Option<A> => v == null ? none : some(v);
const chain = <A, B>(fn: (a: A) => Option<B>) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : fn(fa.value);
const map = <A, B>(fn: (a: A) => B) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : some(fn(fa.value));

function pipe<A>(a: A): A;
function pipe<A, B>(a: A, ab: (a: A) => B): B;
function pipe<A, B, C>(a: A, ab: (a: A) => B, bc: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D): D;
function pipe<A, B, C, D, E>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E): E;
function pipe(a: unknown, ...fns: ((x: unknown) => unknown)[]): unknown {
    return fns.reduce((acc, fn) => fn(acc), a);
}

// Domain
type User = { name: string; deptId?: string };
type Dept = { name: string; managerId?: string };
type Manager = { name: string; email: string };

const users = new Map([["U1", { name: "An", deptId: "D1" }]]);
const depts = new Map([["D1", { name: "Engineering", managerId: "M1" }]]);
const managers = new Map([["M1", { name: "Cường", email: "cuong@mail.com" }]]);

const findUser = (id: string): Option<User> => fromNullable(users.get(id));
const findDept = (id: string): Option<Dept> => fromNullable(depts.get(id));
const findManager = (id: string): Option<Manager> => fromNullable(managers.get(id));

// ❌ Deep chain nesting
const getManagerEmail_chain = (userId: string): Option<string> =>
    pipe(
        findUser(userId),
        chain(user => pipe(
            fromNullable(user.deptId),
            chain(deptId => pipe(
                findDept(deptId),
                chain(dept => pipe(
                    fromNullable(dept.managerId),
                    chain(managerId => pipe(
                        findManager(managerId),
                        map(m => m.email),
                    )),
                )),
            )),
        )),
    );

// ✅ "Do-notation" style — simulated with helper
const Do = <A>(gen: () => Generator<Option<unknown>, A, unknown>): Option<A> => {
    const iterator = gen();
    const step = (value?: unknown): Option<A> => {
        const result = iterator.next(value);
        if (result.done) return some(result.value);
        const option = result.value as Option<unknown>;
        if (option._tag === "None") return none;
        return step(option.value);
    };
    return step();
};

const getManagerEmail_do = (userId: string): Option<string> =>
    Do(function* () {
        const user = yield findUser(userId);
        const deptId = yield fromNullable((user as User).deptId);
        const dept = yield findDept(deptId as string);
        const managerId = yield fromNullable((dept as Dept).managerId);
        const manager = yield findManager(managerId as string);
        return (manager as Manager).email;
    });

// Both give same results!
assert.deepStrictEqual(getManagerEmail_chain("U1"), some("cuong@mail.com"));
assert.deepStrictEqual(getManagerEmail_do("U1"), some("cuong@mail.com"));
assert.deepStrictEqual(getManagerEmail_chain("U999"), none);
assert.deepStrictEqual(getManagerEmail_do("U999"), none);

// Effect.gen looks exactly like the Do version:
// const program = Effect.gen(function*() {
//     const user = yield* findUser(userId);
//     const deptId = yield* getDeptId(user);
//     const dept = yield* findDept(deptId);
//     return dept.name;
// });

console.log("Do-notation OK ✅");
```

> **💡 Do-notation = "write imperative, execute monadic"**: Each `yield` = implicit chain. If any step returns None/Left → short-circuit. Reads like sequential code, executes like pipeline.

---

## ✅ Checkpoint 27.2-27.4

> Đến đây bạn phải hiểu:
> 1. **Monad = Functor + of + chain**. Three operations
> 2. **3 laws**: left identity, right identity, associativity
> 3. **Array.flatMap = chain**. Promise.then = chain. andThen (Ch21) = chain
> 4. **Do-notation/Effect.gen**: generator-based, imperative-looking monadic code
>
> **Test nhanh**: `chain(x => [x, x*2])([1,2,3])` = ?
> <details><summary>Đáp án</summary>`[1, 2, 2, 4, 3, 6]`! = `Array.flatMap`. Map gives `[[1,2],[2,4],[3,6]]`. Chain = map + flatten = flat array.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): map vs chain

```typescript
// Cho: findOrder(id) → Option<Order>, getCustomer(order) → Option<Customer>
// Câu hỏi: dùng map hay chain cho mỗi step?
// Step 1: findOrder("ORD-1") → ?
// Step 2: from order, get customerId (always exists) → ?
// Step 3: getCustomer(customerId) → ?
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
// Step 1: findOrder("ORD-1") → initial value, no map/chain needed
// Step 2: order → order.customerId — always exists → MAP (fn returns plain value)
// Step 3: getCustomer(customerId) — returns Option → CHAIN (fn returns container)

// pipe(
//     findOrder("ORD-1"),           // Option<Order>
//     map(order => order.customerId),// Option<string> — map, plain return
//     chain(getCustomer),           // Option<Customer> — chain, Option return
// )
```

</details>

---

**Bài 2** (10 phút): Either chain workflow

```typescript
// Viết validation pipeline dùng Either chain:
// 1. parseJSON(input) → Either<ParseError, object>
// 2. extractField(obj, "email") → Either<FieldError, string>
// 3. validateEmail(email) → Either<ValidationError, ValidEmail>
// Union all errors vào single type
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type Either<E, A> = { _tag: "Left"; left: E } | { _tag: "Right"; right: A };
const left = <E>(e: E): Either<E, never> => ({ _tag: "Left", left: e });
const right = <A>(a: A): Either<never, A> => ({ _tag: "Right", right: a });
const chain = <E, A, B>(fn: (a: A) => Either<E, B>) => (fa: Either<E, A>): Either<E, B> =>
    fa._tag === "Left" ? fa : fn(fa.right);

function pipe<A>(a: A): A;
function pipe<A, B>(a: A, ab: (a: A) => B): B;
function pipe<A, B, C>(a: A, ab: (a: A) => B, bc: (b: B) => C): C;
function pipe(a: unknown, ...fns: ((x: unknown) => unknown)[]): unknown {
    return fns.reduce((acc, fn) => fn(acc), a);
}

type AppError =
    | { tag: "parse_error"; message: string }
    | { tag: "field_missing"; field: string }
    | { tag: "invalid_email"; value: string };

const parseJSON = (input: string): Either<AppError, Record<string, unknown>> => {
    try { return right(JSON.parse(input)); }
    catch (e) { return left({ tag: "parse_error", message: String(e) }); }
};

const extractField = (field: string) => (obj: Record<string, unknown>): Either<AppError, string> =>
    typeof obj[field] === "string"
        ? right(obj[field] as string)
        : left({ tag: "field_missing", field });

const validateEmail = (email: string): Either<AppError, string> =>
    email.includes("@") ? right(email.toLowerCase()) : left({ tag: "invalid_email", value: email });

const extractAndValidateEmail = (input: string): Either<AppError, string> =>
    pipe(parseJSON(input), chain(extractField("email")), chain(validateEmail));

assert.deepStrictEqual(extractAndValidateEmail('{"email":"An@Mail.COM"}'), right("an@mail.com"));
assert.strictEqual(extractAndValidateEmail("bad json")._tag, "Left");
assert.strictEqual(extractAndValidateEmail('{"name":"An"}')._tag, "Left");
assert.strictEqual(extractAndValidateEmail('{"email":"nope"}')._tag, "Left");
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Nested `Option<Option<A>>` | Used map instead of chain | If fn returns container → use chain |
| "Monad is too abstract" | Theory-first approach | Think: "chain = andThen from Ch21". You've used it! |
| Deep chain nesting | Multiple dependent steps | Use Do-notation or Effect.gen |
| "Promise is a Monad?" | `.then()` auto-flattens | Yes! `Promise.then(fn)` = chain if fn returns Promise |

---

## 💬 Đối thoại với bản thân: Monad Q&A

**Q: Monad và Functor khác nhau chỗ nào?**

A: Functor `map`: function KHÔNG trả container. `(A) => B` + `F<A>` = `F<B>`. Monad `flatMap`: function TRẢ container. `(A) => F<B>` + `F<A>` = `F<B>` (đã flatten). Nếu dùng `map` với function trả container: `F<F<B>>` — nested! `flatMap` = `map` + `flatten`.

**Q: Tại sao gọi là "Monad" mà không gọi là "FlatMappable"?**

A: Tên gốc từ category theory (toán học). Trong thực hành, bạn có thể nghĩ Monad = "chainable container". Tên không quan trọng bằng PATTERN: nhận kết quả bước trước, quyết định bước sau.

**Q: `async/await` là Monad thật sao?**

A: `await promise` = unwrap container (Promise). Kết quả dùng cho bước sau. Nếu promise reject → skip các await sau (fail-fast). Đây CHÍNH XÁC là Monad. JavaScript designers không gọi nó là Monad, nhưng bản chất giống nhau.

**Q: Khi nào dùng `map` vs `flatMap`?**

A: Nếu function của bạn trả giá trị THƯỜNG (string, number) → `map`. Nếu trả Result/Option/Promise → `flatMap`. Đơn giản thế.

---

## Tóm tắt

Chương này đã vén màn Monad — và bạn thấy: chẳng có gì huyền bí. Monad = Functor + `of` + `chain`. `chain` = map + flatten. Bạn đã dùng nó suốt:

- `Array.flatMap()` — Monad chain cho Array
- `Promise.then()` — Monad chain cho Promise
- `andThen()` từ Ch21-22 — Monad chain cho Result/Either

Sực mạnh của việc BIẾT tên: bây giờ bạn có thể NÓI về pattern chung giữa Array, Option, Either, Task. “Chain” không phải 4 thứ khác nhau — nó là MỘT thứ áp dụng cho 4 containers. Và laws đảm bảo nó hoạt động GIỐNG NHAU trong mọi tình huống.

- ✅ **Monad** = Functor + `of` + `chain`. Solves nested container problem.
- ✅ **chain/flatMap** = map + flatten. `Option<Option<A>>` → `Option<A>`.
- ✅ **When**: fn returns plain value → `map`. fn returns container → `chain`.
- ✅ **3 laws**: left/right identity + associativity. Enable safe refactoring.
- ✅ **You've used Monads**: `Array.flatMap`, `Promise.then`, `andThen` (Ch21).
- ✅ **Do-notation/Effect.gen**: imperative syntax for monadic chains.

## Tiếp theo

→ Chapter 28: **Applicative & Validation** — khi bạn cần validate NHIỀU fields song song và thu thập TẤT CẢ lỗi. Monad FAIL ở đây — chỉ bắt lỗi đầu tiên. Applicative giải quyết.
