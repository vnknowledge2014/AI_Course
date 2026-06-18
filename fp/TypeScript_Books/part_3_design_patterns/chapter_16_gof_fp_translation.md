# Chapter 16 — GoF → FP Translation

> **Bạn sẽ học được**:
> - Tại sao nhiều GoF patterns BIẾN MẤT trong FP
> - Strategy = Higher-Order Functions
> - Observer = EventEmitter / Reactive streams
> - Command = Discriminated Unions (ADTs)
> - Visitor = Exhaustive switch
> - Decorator = Function wrapper / composition
> - Middleware = Function composition pipeline
>
> **Yêu cầu trước**: Chapter 12 (composition), Chapter 13 (ADTs, DU), Chapter 15 (generics).
> **Thời gian đọc**: ~50 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Nhận diện GoF patterns trong FP code — viết giải pháp FP thay vì class-heavy OOP.

---

Hãy tưởng tượng bạn là thợ xây trong hai thời đại khác nhau.

Năm 1994, bạn chỉ có gạch, vữa, và bản vẽ giấy. Để xây cổng vòm, bạn cần **giàn giáo** phức tạp — hàng chục thanh gỗ chống đỡ tạm thời, sẽ tháo sau khi vòm đứng vững. Giàn giáo không phải mục đích — nó là workaround vì bạn chưa có bê tông đúc sẵn.

Năm 2025, bạn có bê tông đúc sẵn: đặt khuôn, đổ bê tông, xong. Không cần giàn giáo. Cổng vòm vẫn như cũ — nhưng **cách xây** đơn giản hơn gấp bội.

**GoF Design Patterns** là giàn giáo năm 1994. Chúng giải quyết vấn đề thật — nhưng bằng cách xây infrastructure phức tạp (interfaces, abstract classes, inheritance hierarchies) vì ngôn ngữ lúc đó THIẾU first-class functions, closures, và sum types. TypeScript có tất cả những thứ này. Chương này sẽ cho bạn thấy 6 patterns GoF nổi tiếng "tan biến" thành những giải pháp FP đơn giản đến bất ngờ.

---

## 16.1 — Tại sao Patterns "Biến mất" trong FP?

### Giàn giáo cho cổng vòm đã có bao giờ

Gang of Four (GoF) viết **Design Patterns** năm 1994, cho C++ và Smalltalk. Nhiều patterns tồn tại vì ngôn ngữ THIẾU những tính năng mà TypeScript có sẵn. Peter Norvig đã chỉ ra: "16 trong 23 patterns biến mất hoặc đơn giản hóa trong Lisp." Đây không phải vì patterns sai — mà vì chúng là **giàn giáo** cho những tòa nhà mà bê tông đúc sẵn xây nhanh hơn.

| Thiếu | OOP Pattern | FP có sẵn |
|-------|------------|-----------|
| First-class functions | Strategy, Command, Template Method | Functions ARE values — truyền, return, compose |
| Sum types | Visitor, State, Chain of Responsibility | Discriminated unions + exhaustive switch |
| Closures | Factory, Singleton (đôi khi) | Closure captures state |
| Composition | Decorator, Adapter, Proxy | Function composition `pipe(f, g, h)` |

> **💡 Peter Norvig (2003)**: "16 trong 23 patterns biến mất hoặc đơn giản hóa trong Lisp." FP + TypeScript giữ type safety VÀ loại bỏ boilerplate.

```typescript
// filename: src/patterns_vanish.ts

// ❌ OOP Strategy pattern:
// interface SortStrategy { sort(data: number[]): number[] }
// class BubbleSort implements SortStrategy { ... }
// class QuickSort implements SortStrategy { ... }
// class Sorter { constructor(private strategy: SortStrategy) {} }

// ✅ FP: function IS the strategy
type SortFn = (data: readonly number[]) => readonly number[];

const bubbleSort: SortFn = (data) => [...data].sort((a, b) => a - b);
const reverseSort: SortFn = (data) => [...data].sort((a, b) => b - a);

const process = (data: readonly number[], sort: SortFn): readonly number[] =>
    sort(data);

// Không cần interface, class, constructor injection — chỉ cần function!
```

Nhìn sự khác biệt: OOP cần `interface SortStrategy` → `class BubbleSort implements SortStrategy` → `class Sorter { constructor(private strategy: SortStrategy) }`. FP chỉ cần: `type SortFn = ...` và truyền function. Bốn files tụt thành bốn dòng. Giàn giáo đã dỡ.

---

## ✅ Checkpoint 16.1

> Đến đây bạn phải hiểu:
> 1. Nhiều GoF patterns = workarounds cho THIẾU FP features
> 2. TypeScript CÓ: first-class functions, closures, DU, composition
> 3. Pattern vẫn tồn tại, nhưng implementation ĐƠN GIẢN hơn nhiều
>
> **Test nhanh**: Strategy pattern trong OOP cần interface + class. Trong FP cần gì?
> <details><summary>Đáp án</summary>Chỉ cần **function type** + **truyền function**. `type Strategy = (x: T) => U`. Done.</details>

---

## 16.2 — Strategy = Higher-Order Functions

### Chiến lược giá cả: OOP dùng hierarchy, FP dùng function

Strategy pattern giải quyết bài toán: "thay đổi hành vi tại runtime". OOP truyền thống cần interface → multiple classes → dependency injection. Trong FP, "thay đổi hành vi" = "truyền function khác". Vì function IS a value trong TypeScript, bạn truyền nó như bất kỳ value nào.

Nhưng đợi đã — nếu strategy cần **config** (ví dụ: discount tiers)? OOP dùng constructor để inject config. FP dùng **closure**: factory function nhận config, return strategy function. Config bị "bắt" trong closure — không cần class, không cần `this`.

```typescript
// filename: src/strategy_hof.ts
import assert from "node:assert/strict";

// --- Pricing strategy ---

type PricingStrategy = (basePrice: number, quantity: number) => number;

// Strategies = just functions
const regularPricing: PricingStrategy = (price, qty) =>
    price * qty;

const bulkPricing: PricingStrategy = (price, qty) =>
    qty >= 10 ? price * qty * 0.9 : price * qty;  // 10% off for bulk

const premiumPricing: PricingStrategy = (price, qty) =>
    price * qty * 0.85;  // 15% off always

// Factory: tạo strategy với config (closure = state)
const tieredPricing = (tiers: readonly { readonly minQty: number; readonly discount: number }[]): PricingStrategy =>
    (price, qty) => {
        // Tìm tier cao nhất mà quantity đạt
        const sorted = [...tiers].sort((a, b) => b.minQty - a.minQty);
        const tier = sorted.find(t => qty >= t.minQty);
        const discount = tier?.discount ?? 0;
        return price * qty * (1 - discount);
    };

// Usage — HOF replaces entire Strategy interface + classes
const calculateTotal = (
    items: readonly { readonly price: number; readonly qty: number }[],
    strategy: PricingStrategy
): number =>
    items.reduce((total, item) => total + strategy(item.price, item.qty), 0);

const items = [
    { price: 100, qty: 5 },
    { price: 200, qty: 12 },
];

assert.strictEqual(calculateTotal(items, regularPricing), 2900);
// 100*5 + 200*12 = 500 + 2400 = 2900

assert.strictEqual(calculateTotal(items, bulkPricing), 2660);
// 100*5 (no bulk) + 200*12*0.9 (bulk) = 500 + 2160 = 2660

// Tiered: 1-9 = 0%, 10-19 = 10%, 20+ = 20%
const tiered = tieredPricing([
    { minQty: 10, discount: 0.1 },
    { minQty: 20, discount: 0.2 },
]);

assert.strictEqual(calculateTotal(items, tiered), 2660);
// 100*5*1.0 (qty=5, no tier) + 200*12*0.9 (qty=12, tier 10%) = 500 + 2160 = 2660

console.log("Strategy = HOF OK ✅");
```

### Khi nào vẫn cần interface?

Nếu strategy cần **nhiều hơn 1 function** (ví dụ: logger cần `info`, `error`, `debug`), dùng object literal — nhưng VẪN không cần class. Object literal = "struct có methods" mà không kéo theo inheritance, `this` context, prototype chain.

```typescript
// filename: src/strategy_interface.ts

// Nếu strategy CẦN NHIỀU HƠN 1 function → dùng object (nhưng vẫn không cần class!)
type Logger = {
    readonly info: (msg: string) => void;
    readonly error: (msg: string) => void;
    readonly debug: (msg: string) => void;
};

const consoleLogger: Logger = {
    info: (msg) => console.log(`[INFO] ${msg}`),
    error: (msg) => console.error(`[ERROR] ${msg}`),
    debug: (msg) => console.log(`[DEBUG] ${msg}`),
};

const silentLogger: Logger = {
    info: () => {},
    error: () => {},
    debug: () => {},
};

// Vẫn không cần class — object literal đủ!
const processData = (data: string, logger: Logger): string => {
    logger.info(`Processing: ${data}`);
    logger.debug(`Length: ${data.length}`);
    return data.toUpperCase();
};
```

---

## ✅ Checkpoint 16.2

> Đến đây bạn phải hiểu:
> 1. **Strategy** = function type + truyền function. Không cần interface/class
> 2. **Closure** = strategy factory. Config captured trong closure
> 3. **Multiple methods** → object literal. VẪN không cần class
>
> **Test nhanh**: `const f = (x: number) => x * 2;` — `f` đang implement pattern gì?
> <details><summary>Đáp án</summary>**Strategy**! `f` là một strategy (chiến lược xử lý data). Truyền `f` vào HOF để thay đổi behavior. Trong FP, mọi function đều có thể là strategy.</details>

---

## 16.3 — Observer = EventEmitter

### Theo dõi giá cổ phiếu: OOP dùng Subject/Observer hierarchy, FP dùng EventBus

Observer pattern truyền thống yêu cầu: `abstract class Subject` → `addObserver/removeObserver/notify` → `interface Observer` → `class ConcreteObserver extends Observer`. Trong FP TypeScript, tất cả quy về một **EventBus** với 2 methods: `on` (đăng ký handler) và `emit` (phát sự kiện). Handler là function — đăng ký = push function, hủy đăng ký = return unsubscribe function.

Điểm đáng chú ý: `on()` **trả về** unsubscribe function — FP style. Không cần nhớ reference rồi gọi `removeEventListener`. Closure giữ reference bên trong — clean, composable.

```typescript
// filename: src/observer_emitter.ts
import assert from "node:assert/strict";

// Type-safe EventEmitter (from Ch15)
type EventMap = {
    readonly priceChanged: { readonly symbol: string; readonly oldPrice: number; readonly newPrice: number };
    readonly orderFilled: { readonly orderId: string; readonly quantity: number };
    readonly alert: { readonly message: string; readonly severity: "low" | "high" };
};

type Handler<T> = (payload: T) => void;

type EventBus<E extends Record<string, unknown>> = {
    readonly on: <K extends keyof E>(event: K, handler: Handler<E[K]>) => () => void;
    readonly emit: <K extends keyof E>(event: K, payload: E[K]) => void;
};

const createEventBus = <E extends Record<string, unknown>>(): EventBus<E> => {
    const handlers = new Map<keyof E, Set<Handler<unknown>>>();

    return {
        on: (event, handler) => {
            if (!handlers.has(event)) handlers.set(event, new Set());
            const set = handlers.get(event)!;
            set.add(handler as Handler<unknown>);

            // Return unsubscribe function (FP style!)
            return () => { set.delete(handler as Handler<unknown>); };
        },
        emit: (event, payload) => {
            const set = handlers.get(event);
            if (set) set.forEach(h => h(payload));
        },
    };
};

// Usage
const bus = createEventBus<EventMap>();

const prices: number[] = [];
const unsub = bus.on("priceChanged", (e) => {
    // e: { symbol: string; oldPrice: number; newPrice: number } — auto-inferred!
    prices.push(e.newPrice);
});

bus.emit("priceChanged", { symbol: "BTC", oldPrice: 40000, newPrice: 42000 });
bus.emit("priceChanged", { symbol: "BTC", oldPrice: 42000, newPrice: 41000 });

assert.deepStrictEqual(prices, [42000, 41000]);

// Unsubscribe — functional cleanup
unsub();
bus.emit("priceChanged", { symbol: "BTC", oldPrice: 41000, newPrice: 43000 });
assert.deepStrictEqual(prices, [42000, 41000]);  // không nhận nữa!

console.log("Observer = EventEmitter OK ✅");
```

> **💡 FP Observer improvements**:
> - `on()` returns unsubscribe function — no `removeEventListener` needed
> - Type-safe: EventMap constrains both `on` và `emit`
> - No inheritance: `createEventBus()` = factory, not `extends Observable`

---

## ✅ Checkpoint 16.3

> Đến đây bạn phải hiểu:
> 1. **Observer** = EventBus + handlers. Functional: no class inheritance
> 2. **Unsubscribe** = return function from `on`. Clean, composable
> 3. **Type-safe**: `EventMap` enforces correct payloads
>
> **Test nhanh**: `bus.emit("priceChanged", { message: "hi" })` — lỗi gì?
> <details><summary>Đáp án</summary>**Type error**: `priceChanged` expects `{ symbol, oldPrice, newPrice }`, got `{ message }`. EventMap constraint catches wrong payload.</details>

---

## 16.4 — Command = Discriminated Unions

### Undo/Redo: OOP dùng Command classes, FP dùng DATA

Đây có lẽ là pattern thú vị nhất khi "dịch" sang FP. Trong OOP, Command pattern đóng gói **behavior** vào objects: mỗi command là class với methods `execute()` và `undo()`. Trong FP, command là **data** — discriminated union mang thông tin "làm gì" nhưng KHÔNG mang behavior. Behavior nằm ở `execute` function — pure function nhận `(state, command) → newState`.

Tại sao data tốt hơn behavior? Vì data có thể: **serialize** (gửi qua network), **log** (ghi vào database), **replay** (chạy lại từ đầu bằng `reduce`), và **reverse** (tạo undo command từ forward command). Tất cả miễn phí — bạn chỉ cần xử lý DU. Không cần interface, không cần class, không cần inheritance.

```typescript
// filename: src/command_du.ts
import assert from "node:assert/strict";

// --- Text editor commands ---

// OOP: interface Command { execute(): void; undo(): void; }
// class InsertCommand implements Command { ... }
// class DeleteCommand implements Command { ... }

// ✅ FP: Command = DU (data, not behavior)
type EditorCommand =
    | { readonly tag: "insert"; readonly position: number; readonly text: string }
    | { readonly tag: "delete"; readonly position: number; readonly length: number }
    | { readonly tag: "replace"; readonly position: number; readonly length: number; readonly text: string }
    | { readonly tag: "move_cursor"; readonly position: number };

// State = immutable
type EditorState = {
    readonly content: string;
    readonly cursor: number;
};

// Execute = pure function (state, command) → new state
const execute = (state: EditorState, command: EditorCommand): EditorState => {
    switch (command.tag) {
        case "insert":
            return {
                content: state.content.slice(0, command.position) +
                         command.text +
                         state.content.slice(command.position),
                cursor: command.position + command.text.length,
            };
        case "delete":
            return {
                content: state.content.slice(0, command.position) +
                         state.content.slice(command.position + command.length),
                cursor: command.position,
            };
        case "replace":
            return {
                content: state.content.slice(0, command.position) +
                         command.text +
                         state.content.slice(command.position + command.length),
                cursor: command.position + command.text.length,
            };
        case "move_cursor":
            return { ...state, cursor: command.position };
    }
};

// Undo = reverse command (data → data, not behavior → behavior)
const reverseCommand = (state: EditorState, command: EditorCommand): EditorCommand => {
    switch (command.tag) {
        case "insert":
            return { tag: "delete", position: command.position, length: command.text.length };
        case "delete":
            return {
                tag: "insert",
                position: command.position,
                text: state.content.slice(command.position, command.position + command.length),
            };
        case "replace":
            return {
                tag: "replace",
                position: command.position,
                length: command.text.length,
                text: state.content.slice(command.position, command.position + command.length),
            };
        case "move_cursor":
            return { tag: "move_cursor", position: state.cursor };
    }
};

// Test
const initial: EditorState = { content: "Hello World", cursor: 0 };

const cmd1: EditorCommand = { tag: "insert", position: 5, text: " Beautiful" };
const state1 = execute(initial, cmd1);
assert.strictEqual(state1.content, "Hello Beautiful World");

const cmd2: EditorCommand = { tag: "delete", position: 5, length: 10 };
const state2 = execute(state1, cmd2);
assert.strictEqual(state2.content, "Hello World");

// Undo cmd1 from state1
const undoCmd = reverseCommand(initial, cmd1);
const undone = execute(state1, undoCmd);
assert.strictEqual(undone.content, "Hello World");

// Command history = just an array of data!
const history: readonly EditorCommand[] = [cmd1, cmd2];
const replayed = history.reduce(execute, initial);
assert.strictEqual(replayed.content, "Hello World");

console.log("Command = DU OK ✅");
```

Nhìn dòng cuối: `history.reduce(execute, initial)`. Đây chính là **time-travel debugging** — replay toàn bộ history từ trạng thái ban đầu. Nếu bạn từng dùng Redux DevTools, bạn đã thấy pattern này hoạt động: actions = commands = data, reducer = execute, time travel = replay bằng reduce. Redux IS the Command pattern in FP.

> **💡 Command pattern FP advantages**:
> - Commands = **data** (serializable, loggable, replayable)
> - Undo = generate reverse command (data → data)
> - History = `readonly Command[]` — replay bằng `reduce`
> - No classes, no execute/undo methods — pure functions

---

## ✅ Checkpoint 16.4

> Đến đây bạn phải hiểu:
> 1. **Command = DU**: mỗi variant = 1 command type, carrying DATA
> 2. **Execute** = pure function `(state, command) → newState` (giống reducer!)
> 3. **Undo** = generate reverse command (data → data)
> 4. **History** = array of commands. Replay bằng `reduce(execute, initialState)`
>
> **Test nhanh**: `history.reduce(execute, initial)` giống cái gì trong React?
> <details><summary>Đáp án</summary>**Redux reducer**! `(state, action) → newState`. Actions = commands = DU. Reducer = execute. Redux IS command pattern.</details>

---

## 16.5 — Visitor = Exhaustive Switch

### Duyệt cây cú pháp: OOP cần Visitor hierarchy, FP chỉ cần switch

Visitor pattern trong OOP là một trong những patterns phức tạp nhất: bạn cần `interface Visitor` với method cho MỖI node type, `accept()` method trên MỖI node class, và "double dispatch" — node gọi `visitor.visitX(this)`. Tất cả chỉ để: "gọi function khác nhau tùy type của node".

Trong FP, bạn đã có công cụ này: **discriminated union + exhaustive switch**. DU = node types, switch = "visitor", mỗi case = "visitX". Compiler bắt quên case — giống như interface bắt thiếu `visitX` method. Đơn giản hơn gấp 10 lần.

```typescript
// filename: src/visitor_switch.ts
import assert from "node:assert/strict";

// --- AST (Abstract Syntax Tree) ---

// OOP Visitor:
// interface Visitor { visitLiteral(n: Literal): void; visitBinOp(n: BinOp): void; }
// class PrintVisitor implements Visitor { ... }
// class EvalVisitor implements Visitor { ... }

// ✅ FP: DU + exhaustive switch = Visitor
type Expr =
    | { readonly tag: "literal"; readonly value: number }
    | { readonly tag: "add"; readonly left: Expr; readonly right: Expr }
    | { readonly tag: "multiply"; readonly left: Expr; readonly right: Expr }
    | { readonly tag: "negate"; readonly operand: Expr };

// "Visitor" 1: evaluate
const evaluate = (expr: Expr): number => {
    switch (expr.tag) {
        case "literal":  return expr.value;
        case "add":      return evaluate(expr.left) + evaluate(expr.right);
        case "multiply": return evaluate(expr.left) * evaluate(expr.right);
        case "negate":   return -evaluate(expr.operand);
    }
};

// "Visitor" 2: print
const print = (expr: Expr): string => {
    switch (expr.tag) {
        case "literal":  return String(expr.value);
        case "add":      return `(${print(expr.left)} + ${print(expr.right)})`;
        case "multiply": return `(${print(expr.left)} × ${print(expr.right)})`;
        case "negate":   return `(-${print(expr.operand)})`;
    }
};

// "Visitor" 3: count nodes
const countNodes = (expr: Expr): number => {
    switch (expr.tag) {
        case "literal":  return 1;
        case "add":      return 1 + countNodes(expr.left) + countNodes(expr.right);
        case "multiply": return 1 + countNodes(expr.left) + countNodes(expr.right);
        case "negate":   return 1 + countNodes(expr.operand);
    }
};

// Test: (3 + 4) × (-(2))
const expr: Expr = {
    tag: "multiply",
    left: {
        tag: "add",
        left: { tag: "literal", value: 3 },
        right: { tag: "literal", value: 4 },
    },
    right: {
        tag: "negate",
        operand: { tag: "literal", value: 2 },
    },
};

assert.strictEqual(evaluate(expr), -14);       // (3+4) × (-(2)) = 7 × (-2) = -14
assert.strictEqual(print(expr), "((3 + 4) × (-2))");
assert.strictEqual(countNodes(expr), 6);        // multiply, add, 3, 4, negate, 2 = 6

console.log("Visitor = switch OK ✅");
```

Một điểm tinh tế: trong OOP, thêm **operation mới** (new visitor) dễ — chỉ tạo class mới. Nhưng thêm **node type mới** khó — phải sửa MỌI visitor. Trong FP, tương tự: thêm function mới dễ, thêm variant vào DU → compiler bắt MỌI switch thiếu case. Đây là **Expression Problem** — không có giải pháp hoàn hảo, nhưng FP switch ít overhead hơn OOP visitor hierarchy.

> **💡 Expression Problem**: OOP Visitor dễ thêm operations (new visitor), khó thêm variants (phải sửa mọi visitor). FP switch dễ thêm operations (new function), khó thêm variants (phải sửa mọi function). Compiler catches: quên variant = exhaustive check fail.

---

## ✅ Checkpoint 16.5

> Đến đây bạn phải hiểu:
> 1. **Visitor = exhaustive switch**: mỗi function = 1 "visitor"
> 2. **DU là data**: Expr tree = nested DU. Functions operate ON data
> 3. **Expression Problem**: FP dễ thêm operations, OOP dễ thêm types
> 4. **Exhaustive check** = compiler catches quên variant (giống visitor.visitX)
>
> **Test nhanh**: Thêm variant `{ tag: "abs"; operand: Expr }` — file nào compile error?
> <details><summary>Đáp án</summary>**Cả 3 functions** (`evaluate`, `print`, `countNodes`)! Exhaustive switch thiếu case "abs". Compiler bắt = giống Visitor interface bắt thiếu `visitAbs`.</details>

---

## 16.6 — Decorator = Function Wrapper

### Bọc lớp: OOP kế thừa, FP wrap function

Decorator pattern thêm behavior cho function MÀ KHÔNG sửa function gốc. OOP dùng class kế thừa hoặc proxy. FP dùng **wrapper function**: nhận function → return function mới bọc quanh nó. Giàn giáo biến thành một dòng code.

`withLogging(handler)` = handler + log. `withTiming(handler)` = handler + thời gian. `withAuth(handler, role)` = handler + kiểm tra quyền. Stack chúng lại: `withLogging(withTiming(withAuth(handler, "admin")))` — lớp ngoài chạy trước, lớp trong chạy sau. Giống matryoshka (búp bê Nga) — mở lớp ngoài → lớp giữa → lớp trong → handler gốc.

```typescript
// filename: src/decorator_wrapper.ts
import assert from "node:assert/strict";

// --- HTTP handler decoration ---

type Request = { readonly path: string; readonly method: string; readonly body?: unknown };
type Response = { readonly status: number; readonly body: unknown };
type Handler = (req: Request) => Response;

// OOP: class LoggingDecorator extends Handler { ... }
// ✅ FP: wrapper function = decorator

const withLogging = (handler: Handler): Handler =>
    (req) => {
        console.log(`→ ${req.method} ${req.path}`);
        const res = handler(req);
        console.log(`← ${res.status}`);
        return res;
    };

const withTiming = (handler: Handler): Handler =>
    (req) => {
        const start = performance.now();
        const res = handler(req);
        const ms = (performance.now() - start).toFixed(2);
        return { ...res, body: { ...res.body as object, _timing: `${ms}ms` } };
    };

const withAuth = (handler: Handler, requiredRole: string): Handler =>
    (req) => {
        // Simplified: would check req headers for role = requiredRole
        const authorized = requiredRole === "admin" || requiredRole === "user";
        if (!authorized) return { status: 403, body: { error: `Forbidden: ${requiredRole} required` } };
        return handler(req);
    };

const withCache = (handler: Handler): Handler => {
    const cache = new Map<string, Response>();
    return (req) => {
        if (req.method === "GET") {
            const cached = cache.get(req.path);
            if (cached) return cached;
        }
        const res = handler(req);
        if (req.method === "GET") cache.set(req.path, res);
        return res;
    };
};

// Base handler
const getUser: Handler = (req) => ({
    status: 200,
    body: { name: "An", path: req.path },
});

// Compose decorators — inside out!
const enhanced = withLogging(withTiming(withAuth(getUser, "admin")));

// Hoặc dùng pipe (từ Ch12) — cần curry withAuth trước:
// const authAdmin = (h: Handler) => withAuth(h, "admin");
// const enhanced = pipe(getUser, authAdmin, withTiming, withLogging);

const res = enhanced({ path: "/users/1", method: "GET" });
assert.strictEqual(res.status, 200);

console.log("Decorator = wrapper OK ✅");
```

---

## 16.7 — Middleware = Composition Pipeline

### Express middleware thực chất là gì?

Middleware giống decorator nhưng thêm khả năng **quyết định có gọi next hay không**. Decorator luôn gọi hàm gốc; middleware có thể short-circuit (trả response sớm) hoặc modify context trước khi truyền xuống. Pattern `(ctx, next) => result` — mỗi middleware wrap quanh `next()`, tạo thành chuỗi "hành tây" (onion model).

`compose(logger, timer, errorHandler)` = `logger(timer(errorHandler(handler)))`. Request đi vào từ ngoài (logger) → vào trong (timer) → trong cùng (errorHandler → handler). Response đi ngược ra. Đây là lý do middleware chạy "hai lần": trước `next()` (request) và sau `next()` (response).

```typescript
// filename: src/middleware.ts
import assert from "node:assert/strict";

// Middleware = function that wraps the NEXT handler
type Context = {
    readonly path: string;
    readonly method: string;
    readonly headers: ReadonlyMap<string, string>;
    readonly body?: unknown;
    readonly state: Record<string, unknown>;  // mutable per-request
};

type MiddlewareResult = {
    readonly status: number;
    readonly body: unknown;
};

type Middleware = (ctx: Context, next: () => MiddlewareResult) => MiddlewareResult;

// Compose middlewares (right to left, innermost first)
const compose = (...middlewares: readonly Middleware[]) =>
    (ctx: Context, finalHandler: () => MiddlewareResult): MiddlewareResult => {
        const dispatch = (index: number): MiddlewareResult => {
            if (index >= middlewares.length) return finalHandler();
            return middlewares[index](ctx, () => dispatch(index + 1));
        };
        return dispatch(0);
    };

// Middlewares
const logger: Middleware = (ctx, next) => {
    console.log(`→ ${ctx.method} ${ctx.path}`);
    const result = next();
    console.log(`← ${result.status}`);
    return result;
};

const timer: Middleware = (ctx, next) => {
    const start = Date.now();
    const result = next();
    const ms = Date.now() - start;
    console.log(`⏱ ${ms}ms`);
    return result;
};

const errorHandler: Middleware = (_ctx, next) => {
    try {
        return next();
    } catch (e) {
        return { status: 500, body: { error: String(e) } };
    }
};

// Application
const app = compose(logger, timer, errorHandler);

const result = app(
    {
        path: "/api/users",
        method: "GET",
        headers: new Map(),
        state: {},
    },
    () => ({ status: 200, body: { users: ["An", "Bình"] } })
);

assert.strictEqual(result.status, 200);

console.log("Middleware = composition OK ✅");
```

> **💡 Middleware IS function composition**: Mỗi middleware = wrapper quanh `next`. `compose(a, b, c)` = `a(b(c(handler)))`. Express middleware = Decorator pattern + Chain of Responsibility — tất cả qua function composition.

---

## ✅ Checkpoint 16.7

> Đến đây bạn phải hiểu:
> 1. **Middleware** = function wrapping next handler → composition chain
> 2. **compose** = right-to-left nesting. First middleware runs first
> 3. **Decorator vs Middleware**: decorator = wrap function. Middleware = wrap with `next` callback
> 4. Express middleware = Decorator + Chain of Responsibility — FP = just composition
>
> **Test nhanh**: `compose(logger, errorHandler)` — lỗi xảy ra ở handler. Middleware nào catch?
> <details><summary>Đáp án</summary>`errorHandler`! Nó wrap handler trực tiếp (innermost). `logger` chạy trước, gọi `next` → `errorHandler` gọi `next` → handler throws → `errorHandler` catches. Logger thấy status 500.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Strategy pattern

```typescript
// Viết sorting system:
// 1. type Comparator<T> = (a: T, b: T) => number
// 2. sortBy<T>(arr: readonly T[], cmp: Comparator<T>): readonly T[]
// 3. Tạo 3 comparators: byName, byAge, byNameDesc (ngược)
// Test: sortBy(users, byAge) → sorted by age ascending
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

type Comparator<T> = (a: T, b: T) => number;

const sortBy = <T>(arr: readonly T[], cmp: Comparator<T>): readonly T[] =>
    [...arr].sort(cmp);

type User = { readonly name: string; readonly age: number };

const byName: Comparator<User> = (a, b) => a.name.localeCompare(b.name);
const byAge: Comparator<User> = (a, b) => a.age - b.age;
const byNameDesc: Comparator<User> = (a, b) => b.name.localeCompare(a.name);

// Bonus: reverse any comparator
const reverse = <T>(cmp: Comparator<T>): Comparator<T> =>
    (a, b) => cmp(b, a);

const users: readonly User[] = [
    { name: "Cường", age: 30 },
    { name: "An", age: 25 },
    { name: "Bình", age: 28 },
];

const byAgeSorted = sortBy(users, byAge);
assert.strictEqual(byAgeSorted[0].name, "An");
assert.strictEqual(byAgeSorted[2].name, "Cường");

const byNameSorted = sortBy(users, byName);
assert.strictEqual(byNameSorted[0].name, "An");

const reversed = sortBy(users, reverse(byAge));
assert.strictEqual(reversed[0].name, "Cường");
```

</details>

---

**Bài 2** (10 phút): Command pattern + undo

```typescript
// Viết simple calculator với command pattern:
// Commands: "add" | "subtract" | "multiply" | "divide"
// State: { value: number }
//
// 1. type CalcCommand = DU với 4 operations + amount
// 2. execute(state, cmd) → new state
// 3. undo: reverseCommand(cmd) → CalcCommand
// 4. Test: 0 → add(10) → multiply(3) → subtract(5) = 25 → undo subtract → undo multiply = 10
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type CalcCommand =
    | { readonly tag: "add"; readonly amount: number }
    | { readonly tag: "subtract"; readonly amount: number }
    | { readonly tag: "multiply"; readonly factor: number }
    | { readonly tag: "divide"; readonly divisor: number };

type CalcState = { readonly value: number };

const execute = (state: CalcState, cmd: CalcCommand): CalcState => {
    switch (cmd.tag) {
        case "add":      return { value: state.value + cmd.amount };
        case "subtract": return { value: state.value - cmd.amount };
        case "multiply": return { value: state.value * cmd.factor };
        case "divide":   return { value: state.value / cmd.divisor };
    }
};

const reverseCommand = (cmd: CalcCommand): CalcCommand => {
    switch (cmd.tag) {
        case "add":      return { tag: "subtract", amount: cmd.amount };
        case "subtract": return { tag: "add", amount: cmd.amount };
        case "multiply": return { tag: "divide", divisor: cmd.factor };
        case "divide":   return { tag: "multiply", factor: cmd.divisor };
    }
};

// Test: 0 → add(10) → multiply(3) → subtract(5) = 25
const initial: CalcState = { value: 0 };

const commands: readonly CalcCommand[] = [
    { tag: "add", amount: 10 },
    { tag: "multiply", factor: 3 },
    { tag: "subtract", amount: 5 },
];

const final = commands.reduce(execute, initial);
assert.strictEqual(final.value, 25);  // (0+10)*3 - 5 = 25

// Undo last 2: subtract(5) then multiply(3)
const undo2 = execute(execute(final, reverseCommand(commands[2])), reverseCommand(commands[1]));
assert.strictEqual(undo2.value, 10);  // 25 + 5 = 30, 30 / 3 = 10
```

</details>

---

**Bài 3** (15 phút): Decorator composition

```typescript
// Viết decorator system cho functions:
// 1. withRetry(fn, maxAttempts): retry nếu throw
// 2. withTimeout(fn, ms): throw nếu quá lâu (simplified: random delay)
// 3. withFallback(fn, fallbackValue): return fallback nếu throw
// 4. Compose: withRetry(withFallback(handler, defaultValue), 3)
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

// Generic decorator type
type Fn<I, O> = (input: I) => O;

const withRetry = <I, O>(fn: Fn<I, O>, maxAttempts: number): Fn<I, O> =>
    (input) => {
        let lastError: unknown;
        for (let i = 0; i < maxAttempts; i++) {
            try {
                return fn(input);
            } catch (e) {
                lastError = e;
            }
        }
        throw lastError;
    };

const withFallback = <I, O>(fn: Fn<I, O>, fallback: O): Fn<I, O> =>
    (input) => {
        try {
            return fn(input);
        } catch {
            return fallback;
        }
    };

const withLogging = <I, O>(fn: Fn<I, O>, label: string): Fn<I, O> =>
    (input) => {
        console.log(`[${label}] input:`, input);
        const result = fn(input);
        console.log(`[${label}] output:`, result);
        return result;
    };

// Test
let callCount = 0;
const flaky: Fn<string, string> = (input) => {
    callCount++;
    if (callCount < 3) throw new Error("Flaky!");
    return input.toUpperCase();
};

const reliable = withRetry(flaky, 5);
assert.strictEqual(reliable("hello"), "HELLO");
assert.strictEqual(callCount, 3);  // failed 2, succeeded on 3rd

// Compose: retry then fallback
callCount = 0;
const alwaysFails: Fn<string, string> = () => { throw new Error("Always fails"); };

const safe = withFallback(withRetry(alwaysFails, 3), "DEFAULT");
assert.strictEqual(safe("anything"), "DEFAULT");
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Strategy quá nhiều params | Function cần config + input | Closure: `createStrategy(config)` → return `(input) => result` |
| Event handler leak | Quên unsubscribe | `on()` trả unsubscribe fn. Clean up khi component unmount |
| Command undo sai | Reverse command thiếu context | Lưu "before state" cùng command, hoặc dùng snapshot |
| Decorator order confusion | `a(b(c(x)))` — c chạy trước | Đọc inside-out. Hoặc dùng `pipe(c, b, a)` cho left-to-right |
| Middleware `next()` gọi 2 lần | Bug trong middleware | Convention: gọi `next()` đúng 1 lần. Hoặc enforce bằng type |

---

## Tóm tắt

Chương này đã dỡ giàn giáo GoF — cho bạn thấy 6 patterns nổi tiếng "tan biến" thành giải pháp FP thanh lịch:

- ✅ **Strategy = HOF**: function type + truyền function. Closure cho config.
- ✅ **Observer = EventBus**: type-safe `on`/`emit`. Return unsubscribe function.
- ✅ **Command = DU**: commands = DATA. Execute = pure `(state, cmd) → state`. Undo = reverse data.
- ✅ **Visitor = switch**: exhaustive switch = type-safe "visitor". Compiler catches quên.
- ✅ **Decorator = wrapper**: `(fn) → fn`. Compose decorators by nesting or pipe.
- ✅ **Middleware = composition**: `compose(a, b, c)` = chain of wrappers with `next`.

Bạn không cần "quên" GoF — chỉ cần nhận ra rằng trong TypeScript, giải pháp thường đơn giản hơn nhiều so với sách năm 1994 dạy. Functions, closures, DU, và composition là "bê tông đúc sẵn" thay thế cho "giàn giáo" OOP. Chương tiếp theo sẽ đưa bạn vào một pattern kiến trúc cao cấp hơn: **CQRS & Event Sourcing** — nơi Command pattern được đẩy lên tầm kiến trúc hệ thống.

## Tiếp theo

→ Chapter 17: **CQRS & Event Sourcing** — Tách read/write. Event Sourcing: immutable event log, `reduce` rebuild state, projections, temporal queries.
