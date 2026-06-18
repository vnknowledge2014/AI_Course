# Chapter 35 — Frontend: React + FP

> **Bạn sẽ học được**:
> - React components as pure functions — `f(props) = JSX`
> - Immutability in React state — never mutate
> - FP patterns in hooks: `useReducer` as state machine
> - Option/Either in React — handle loading, error, success states
> - Separating logic from UI — custom hooks, pure helpers
>
> **Yêu cầu trước**: Chapter 13 (ADTs), Chapter 20 (DUs), Chapter 26-28 (Functor, Monad, Applicative).
> **Thời gian đọc**: ~45 phút | **Level**: Intermediate-Advanced
> **Kết quả cuối cùng**: React components với FP mindset — predictable, testable, composable.

---

Bạn biết nghệ sĩ origami không? Họ nhận tờ giấy (props), gấp theo CÙNG cách, ra CÙNG hình. Cho cùng tờ giấy đỏ → con hạc đỏ. Cho tờ giấy xanh → con hạc xanh. Hình dạng quyết định bởi CÁCH GẤP (component function), không phải tờ giấy (props).

**React component = pure function**: `f(props) = JSX`. Cùng props → cùng output. No side effects trong render. State hooks = controlled effects (managed by React).

Đây không phải trùng hợp. React được THIẾT KẾ theo FP philosophy. Dan Abramov (React core team): “React components should be pure functions of props.” `useReducer` = State machine từ Ch20. `useMemo` = memoization từ Ch12. `useEffect` = managed side effects. Tất cả FP patterns bạn học từ Ch12-30 ÁP DỤNG TRỰC TIẾP vào React.

Chapter này không dạy React API — chương này dạy cách ÁP DỤNG FP thinking vào React. Bạn đã biết patterns: DUs (Ch14/20), pure functions (Ch12/16), Functors (Ch26). Giờ áp dụng chúng vào UI.

---

## React + FP — Frontend as Pure Functions

React component là **pure function**: nhận props, trả JSX. Không side-effects trong render. Hooks (`useState`, `useEffect`) quản lý state và side-effects theo declarative pattern.

Chapter này áp dụng FP concepts vào frontend: discriminated unions cho UI states (`Loading | Error | Success`), `fp-ts`/`Effect` cho async data fetching, Zod cho form validation. Kết quả: React code type-safe, predictable, testable — không phải "component works sometimes, breaks sometimes".


## 35.1 — React Components as Pure Functions

```typescript
// filename: src/frontend/pure_components.tsx
// Note: This is conceptual — React JSX shown as type signatures

import assert from "node:assert/strict";

// === Component = Pure Function ===
// type FC<P> = (props: P) => JSX.Element

// ✅ Pure: same props → same output
// const UserCard = ({ name, email }: { name: string; email: string }) => (
//     <div className="card">
//         <h3>{name}</h3>
//         <p>{email}</p>
//     </div>
// );

// ❌ Impure: reads global state, different output same props
// const BadCard = ({ name }: { name: string }) => (
//     <div>{name} - {new Date().toISOString()}</div>  // time changes!
// );

// === Simulate: component as pure function ===
type UserCardProps = { name: string; email: string; isActive: boolean };
type VNode = { tag: string; children: (string | VNode)[] };

const UserCard = (props: UserCardProps): VNode => ({
    tag: "div",
    children: [
        { tag: "h3", children: [props.name] },
        { tag: "p", children: [props.email] },
        { tag: "span", children: [props.isActive ? "Active" : "Inactive"] },
    ],
});

// Pure! Same props → same output
const card1 = UserCard({ name: "An", email: "an@mail.com", isActive: true });
const card2 = UserCard({ name: "An", email: "an@mail.com", isActive: true });
assert.deepStrictEqual(card1, card2);  // identical!

// Different props → different output
const card3 = UserCard({ name: "Bình", email: "binh@mail.com", isActive: false });
assert.notDeepStrictEqual(card1, card3);

console.log("Pure components OK ✅");
```

---

## 35.2 — Immutable State & useReducer as State Machine

Tại sao `useReducer` thay vì `useState`? Với forms phức tạp (nhiều fields, nhiều states như idle/editing/submitting/error), `useState` tạo mới biến rời: `isLoading`, `error`, `formData` — 3 biến độc lập có thể CONFLICT (isLoading=true VÀ error=something?). `useReducer` gộp tất cả vào MỘT state machine. Transitions rõ ràng. Impossible states = không có.

Và reducer là **PURE FUNCTION**: `(state, action) => newState`. Test không cần React, không cần DOM, không cần browser. Đây chính xác là Functional Core (Ch33): pure reducer = core. React hooks = imperative shell.

```typescript
// filename: src/frontend/state_machine.ts
import assert from "node:assert/strict";

// === useReducer = State Machine ===
// const [state, dispatch] = useReducer(reducer, initialState);
// reducer = (state, action) => newState  ← PURE FUNCTION!

// Form state machine
type FormState =
    | { status: "idle" }
    | { status: "editing"; name: string; email: string }
    | { status: "submitting"; name: string; email: string }
    | { status: "success"; userId: string }
    | { status: "error"; message: string; name: string; email: string };

type FormAction =
    | { type: "start_editing" }
    | { type: "update_field"; field: "name" | "email"; value: string }
    | { type: "submit" }
    | { type: "submit_success"; userId: string }
    | { type: "submit_error"; message: string }
    | { type: "reset" };

// Reducer = pure function! Testable!
const formReducer = (state: FormState, action: FormAction): FormState => {
    switch (action.type) {
        case "start_editing":
            return { status: "editing", name: "", email: "" };

        case "update_field":
            if (state.status !== "editing" && state.status !== "error") return state;
            const editable = state.status === "editing" ? state : state;
            return {
                ...editable,
                status: "editing" as const,
                [action.field]: action.value,
            };

        case "submit":
            if (state.status !== "editing") return state;
            return { status: "submitting", name: state.name, email: state.email };

        case "submit_success":
            return { status: "success", userId: action.userId };

        case "submit_error":
            if (state.status !== "submitting") return state;
            return {
                status: "error",
                message: action.message,
                name: state.name,
                email: state.email,
            };

        case "reset":
            return { status: "idle" };
    }
};

// === Test reducer (pure function!) ===
let state: FormState = { status: "idle" };

// Start editing
state = formReducer(state, { type: "start_editing" });
assert.strictEqual(state.status, "editing");

// Update fields
state = formReducer(state, { type: "update_field", field: "name", value: "An" });
state = formReducer(state, { type: "update_field", field: "email", value: "an@mail.com" });
if (state.status === "editing") {
    assert.strictEqual(state.name, "An");
    assert.strictEqual(state.email, "an@mail.com");
}

// Submit
state = formReducer(state, { type: "submit" });
assert.strictEqual(state.status, "submitting");

// Success
state = formReducer(state, { type: "submit_success", userId: "U-1" });
assert.strictEqual(state.status, "success");

// Invalid: can't edit when success
state = formReducer(state, { type: "update_field", field: "name", value: "X" });
assert.strictEqual(state.status, "success");  // unchanged!

// Reset
state = formReducer(state, { type: "reset" });
assert.strictEqual(state.status, "idle");

console.log("State machine reducer OK ✅");
```

> **💡 useReducer = DU state machine from Ch20!** States = branded union tags. Actions = events. Reducer = transition function. All PURE. Test without React.

---

## ✅ Checkpoint 35.1-35.2

> Đến đây bạn phải hiểu:
> 1. **React component = pure function**: `f(props) = JSX`. Same in → same out
> 2. **Immutable state**: never mutate. Spread `{...state, field: newValue}`
> 3. **useReducer**: `(state, action) => newState`. Pure reducer = testable
> 4. **State machine**: DU type for states + actions. Invalid transitions return current state
>
> **Test nhanh**: `formReducer` cần DOM để test không?
> <details><summary>Đáp án</summary>**KHÔNG!** Pure function. Input (state + action) → output (new state). No React, no DOM, no browser. Test như bất kỳ pure function nào.</details>

---

## 35.3 — Option/Either in React: Loading States

```typescript
// filename: src/frontend/remote_data.ts
import assert from "node:assert/strict";

// === RemoteData = DU for async UI state ===
type RemoteData<E, A> =
    | { status: "idle" }
    | { status: "loading" }
    | { status: "success"; data: A }
    | { status: "error"; error: E };

// Constructor helpers
const idle = <E, A>(): RemoteData<E, A> => ({ status: "idle" });
const loading = <E, A>(): RemoteData<E, A> => ({ status: "loading" });
const success = <A>(data: A): RemoteData<never, A> => ({ status: "success", data });
const error = <E>(error: E): RemoteData<E, never> => ({ status: "error", error });

// map (Functor!)
const map = <A, B, E>(rd: RemoteData<E, A>, fn: (a: A) => B): RemoteData<E, B> =>
    rd.status === "success" ? success(fn(rd.data)) : rd as RemoteData<E, B>;

// fold (exhaustive pattern match)
const fold = <E, A, R>(
    rd: RemoteData<E, A>,
    cases: {
        idle: () => R;
        loading: () => R;
        success: (data: A) => R;
        error: (error: E) => R;
    },
): R => {
    switch (rd.status) {
        case "idle": return cases.idle();
        case "loading": return cases.loading();
        case "success": return cases.success(rd.data);
        case "error": return cases.error(rd.error);
    }
};

// === React pattern (conceptual) ===
// const UserProfile = ({ userId }: { userId: string }) => {
//     const [user, setUser] = useState<RemoteData<string, User>>(idle());
//
//     useEffect(() => {
//         setUser(loading());
//         fetchUser(userId)
//             .then(u => setUser(success(u)))
//             .catch(e => setUser(error(e.message)));
//     }, [userId]);
//
//     return fold(user, {
//         idle: () => <div>Click to load</div>,
//         loading: () => <Spinner />,
//         success: (u) => <UserCard name={u.name} email={u.email} />,
//         error: (msg) => <ErrorBanner message={msg} />,
//     });
// };

// Test fold (pure!)
const rd1: RemoteData<string, number> = success(42);
const rendered1 = fold(rd1, {
    idle: () => "idle",
    loading: () => "loading...",
    success: (n) => `Value: ${n}`,
    error: (e) => `Error: ${e}`,
});
assert.strictEqual(rendered1, "Value: 42");

const rd2: RemoteData<string, number> = error("Network failed");
const rendered2 = fold(rd2, {
    idle: () => "idle",
    loading: () => "loading...",
    success: (n) => `Value: ${n}`,
    error: (e) => `Error: ${e}`,
});
assert.strictEqual(rendered2, "Error: Network failed");

// map
const mapped = map(success(5), x => x * 2);
assert.deepStrictEqual(mapped, success(10));

const mappedErr = map(error("fail") as RemoteData<string, number>, x => x * 2);
assert.strictEqual(mappedErr.status, "error");

console.log("RemoteData OK ✅");
```

> **💡 RemoteData replaces `isLoading` + `error` + `data` booleans!** One type, four states, exhaustive matching. No impossible states (loading + error simultaneously).

---

## ✅ Checkpoint 35.3

> Đến đây bạn phải hiểu:
> 1. **RemoteData<E, A>** = DU for async UI: idle | loading | success | error
> 2. **fold** = exhaustive match. Handle ALL states. No forgotten edge cases
> 3. **map** = Functor. Transform success data, leave other states alone
> 4. **Replaces**: `{ isLoading: boolean; error: string | null; data: T | null }` mess

---

## 🏋️ Bài tập

**Bài 1** (10 phút): Todo reducer

```typescript
// Write a useReducer-style reducer for Todo app:
// States: list of { id, text, done }
// Actions: add, toggle, remove, clearCompleted
// All pure — test without React
```

<details><summary>✅ Lời giải</summary>

```typescript
import assert from "node:assert/strict";

type Todo = { id: number; text: string; done: boolean };
type TodoAction =
    | { type: "add"; text: string }
    | { type: "toggle"; id: number }
    | { type: "remove"; id: number }
    | { type: "clear_completed" };

let nextId = 1;
const todoReducer = (state: readonly Todo[], action: TodoAction): readonly Todo[] => {
    switch (action.type) {
        case "add": return [...state, { id: nextId++, text: action.text, done: false }];
        case "toggle": return state.map(t => t.id === action.id ? { ...t, done: !t.done } : t);
        case "remove": return state.filter(t => t.id !== action.id);
        case "clear_completed": return state.filter(t => !t.done);
    }
};

let todos: readonly Todo[] = [];
todos = todoReducer(todos, { type: "add", text: "Learn FP" });
todos = todoReducer(todos, { type: "add", text: "Write tests" });
assert.strictEqual(todos.length, 2);

todos = todoReducer(todos, { type: "toggle", id: 1 });
assert.strictEqual(todos[0].done, true);

todos = todoReducer(todos, { type: "clear_completed" });
assert.strictEqual(todos.length, 1);
assert.strictEqual(todos[0].text, "Write tests");
```

</details>

---

## Tóm tắt

React và FP là bạn tự nhiên. Components là pure functions. State là immutable. Reducers là state machines. RemoteData là ADTs. Mọi thứ từ Ch12-30 xuất hiện lại tại đây — dưới hình thức React.

- ✅ **React = pure functions**: `f(props) = JSX`. Same props → same output.
- ✅ **useReducer = state machine**: DU states + actions. Pure reducer, testable.
- ✅ **RemoteData<E, A>**: idle | loading | success | error. Exhaustive fold.
- ✅ **Separate logic from UI**: custom hooks for effects, pure functions for logic.
- ✅ **FP in React**: immutability, pattern matching, composition.

## Tiếp theo

→ Chapter 36: **Capstone Part 1 — Domain Model** — full DDD project applying ALL patterns.
