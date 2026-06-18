# Chapter 18 — GoF Patterns → FP Translation

> **Bạn sẽ học được**:
> - Tại sao nhiều GoF patterns **biến mất** trong FP — vì ngôn ngữ đã hỗ trợ sẵn
> - **Strategy** = Higher-Order Function
> - **Observer** = Channel / Callback list
> - **Command** = Enum variant (data as command)
> - **Visitor** = Pattern matching
> - **Factory** = Smart constructor
> - **Decorator** = Function composition
> - **Adapter** = `From`/`Into` trait
>
> **Yêu cầu trước**: Part II complete (đặc biệt Chapter 13: HOF, Chapter 16: Traits).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn nhìn design patterns dưới góc nhìn FP — đơn giản hơn, ít boilerplate hơn.

---

## Bước vào Part III: Design Patterns

GoF (Gang of Four) patterns được thiết kế cho OOP: Java, C#, C++. Nhiều patterns tồn tại vì **OOP thiếu features** mà FP có sẵn:

| OOP cần pattern vì... | FP/Rust có sẵn... |
|---|---|
| Không truyền function as param | Closures, HOFs |
| Class hierarchy cồng kềnh | Enums + pattern matching |
| Interface verbose | Traits |
| Inheritance phức tạp | Composition |
| Mutation everywhere | Immutability |

> **💡 Peter Norvig (1996)**: "16 trong 23 GoF patterns trở nên **invisible hoặc simpler** trong ngôn ngữ dynamic/functional."

---

## GoF Patterns dưới góc nhìn FP

Design Patterns của Gang of Four (GoF) ra đời trong bối cảnh OOP — Java, C++, Smalltalk. Nhiều patterns trong đó tồn tại để **bù đắp hạn chế** của OOP: thiếu first-class functions (→ Strategy, Command, Template Method), thiếu sum types (→ Visitor), thiếu pattern matching (→ State).

Trong ngôn ngữ có first-class functions, closures, enums, và pattern matching — như Rust — nhiều GoF patterns trở nên **trivial**. Strategy là higher-order function. Command là enum. Observer là callback. Visitor là match.

Điều này không có nghĩa GoF patterns vô dụng — nhiều pattern (như Repository, Factory) vẫn áp dụng. Nhưng bạn sẽ thấy implementation đơn giản hơn nhiều khi có FP tools.

---

## 18.1 — Strategy Pattern → Higher-Order Function

### OOP: Interface + classes

```
// OOP (Java-style pseudocode):
interface PricingStrategy { int calculate(int price); }
class RegularPricing implements PricingStrategy { ... }
class VipPricing implements PricingStrategy { ... }
class HolidayPricing implements PricingStrategy { ... }
// → 1 interface + N classes + constructor injection
```

### FP/Rust: Chỉ là một function parameter

```rust
// filename: src/main.rs

// Strategy = function. Không cần interface/class.
fn calculate_total(prices: &[u32], strategy: impl Fn(u32) -> u32) -> u32 {
    prices.iter().map(|&p| strategy(p)).sum()
}

// "Strategies" = closures hoặc functions
fn no_discount(price: u32) -> u32 { price }
fn vip_discount(price: u32) -> u32 { price * 85 / 100 }

fn make_coupon(percent: u32) -> impl Fn(u32) -> u32 {
    move |price| price * (100 - percent) / 100
}

fn main() {
    let prices = vec![100_000, 200_000, 50_000];

    println!("Regular: {}đ", calculate_total(&prices, no_discount));
    println!("VIP:     {}đ", calculate_total(&prices, vip_discount));
    println!("Coupon:  {}đ", calculate_total(&prices, make_coupon(15)));
    println!("Lambda:  {}đ", calculate_total(&prices, |p| if p > 100_000 { p * 90 / 100 } else { p }));

    // Output:
    // Regular: 350000đ
    // VIP:     297500đ
    // Coupon:  297500đ
    // Lambda:  330000đ
}
```

**So sánh**: OOP = 1 interface + N classes (mỗi class 1 file). FP = 1 function param + N closures (có thể inline).

---

## 18.2 — Command Pattern → Enum

### OOP: Interface + Command classes

```
// OOP: mỗi command là 1 class
interface Command { void execute(); void undo(); }
class AddItemCommand implements Command { ... }
class RemoveItemCommand implements Command { ... }
```

### FP/Rust: Enum = data chứa command

```rust
// filename: src/main.rs

// Command = enum variant chứa data
#[derive(Debug, Clone)]
enum EditorCommand {
    InsertText { position: usize, text: String },
    DeleteRange { start: usize, end: usize },
    Replace { from: String, to: String },
    Undo,
}

#[derive(Debug, Clone)]
struct Document {
    content: String,
    history: Vec<EditorCommand>,
}

impl Document {
    fn new(content: &str) -> Self {
        Document { content: content.to_string(), history: vec![] }
    }

    // Execute: pattern match trên command
    fn execute(&self, cmd: EditorCommand) -> Self {
        let new_content = match &cmd {
            EditorCommand::InsertText { position, text } => {
                let mut s = self.content.clone();
                let pos = (*position).min(s.len());
                s.insert_str(pos, text);
                s
            }
            EditorCommand::DeleteRange { start, end } => {
                let mut s = self.content.clone();
                let end = (*end).min(s.len());
                let start = (*start).min(end);
                s.drain(start..end);
                s
            }
            EditorCommand::Replace { from, to } => {
                self.content.replace(from.as_str(), to.as_str())
            }
            EditorCommand::Undo => {
                // Simplified: chỉ print thông báo
                println!("  (Undo not fully implemented in this demo)");
                return self.clone();
            }
        };

        let mut new_history = self.history.clone();
        new_history.push(cmd);

        Document { content: new_content, history: new_history }
    }
}

fn main() {
    let doc = Document::new("Hello World");
    println!("Start: '{}'", doc.content);

    let doc = doc.execute(EditorCommand::InsertText {
        position: 5,
        text: ", Rust".into(),
    });
    println!("Insert: '{}'", doc.content);

    let doc = doc.execute(EditorCommand::Replace {
        from: "World".into(),
        to: "FP".into(),
    });
    println!("Replace: '{}'", doc.content);

    let doc = doc.execute(EditorCommand::DeleteRange { start: 0, end: 7 });
    println!("Delete: '{}'", doc.content);

    println!("\nHistory ({} commands):", doc.history.len());
    for (i, cmd) in doc.history.iter().enumerate() {
        println!("  {}. {:?}", i + 1, cmd);
    }
}
```

**So sánh**: OOP = N command classes. FP = 1 enum + pattern match. Serializable, history tự nhiên.

---

## 18.3 — Observer Pattern → Callbacks / Channels

### FP/Rust: Vec of callbacks

```rust
// filename: src/main.rs

type Listener<T> = Box<dyn Fn(&T)>;

struct EventEmitter<T> {
    listeners: Vec<Listener<T>>,
}

impl<T> EventEmitter<T> {
    fn new() -> Self { EventEmitter { listeners: vec![] } }

    fn on(mut self, handler: impl Fn(&T) + 'static) -> Self {
        self.listeners.push(Box::new(handler));
        self
    }

    fn emit(&self, event: &T) {
        for listener in &self.listeners {
            listener(event);
        }
    }
}

#[derive(Debug)]
enum AppEvent {
    UserLogin(String),
    PageView(String),
    Purchase { item: String, amount: u32 },
}

fn main() {
    let emitter = EventEmitter::new()
        .on(|e: &AppEvent| println!("  [Logger] {:?}", e))
        .on(|e: &AppEvent| {
            if let AppEvent::Purchase { amount, .. } = e {
                if *amount > 100_000 {
                    println!("  [Alert] Large purchase: {}đ!", amount);
                }
            }
        })
        .on(|e: &AppEvent| {
            if let AppEvent::UserLogin(name) = e {
                println!("  [Welcome] Hello, {}!", name);
            }
        });

    println!("Event 1:");
    emitter.emit(&AppEvent::UserLogin("Minh".into()));

    println!("\nEvent 2:");
    emitter.emit(&AppEvent::Purchase { item: "Laptop".into(), amount: 25_000_000 });

    println!("\nEvent 3:");
    emitter.emit(&AppEvent::PageView("/products".into()));
}
```

---

## 18.4 — Visitor Pattern → Pattern Matching

### OOP: Double dispatch nightmare

```
// OOP Java:
interface ShapeVisitor {
    void visit(Circle c);
    void visit(Rectangle r);
    void visit(Triangle t);
}
// → Thêm shape = sửa interface + MỌI visitor classes
```

### FP/Rust: Chỉ là `match`

```rust
// filename: src/main.rs

#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { w: f64, h: f64 },
    Triangle { base: f64, height: f64 },
}

// "Visitor" = function với match. Thêm "visitor" = thêm function.
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { w, h } => w * h,
        Shape::Triangle { base, height } => 0.5 * base * height,
    }
}

fn svg(shape: &Shape) -> String {
    match shape {
        Shape::Circle { radius } =>
            format!("<circle r=\"{}\" />", radius),
        Shape::Rectangle { w, h } =>
            format!("<rect width=\"{}\" height=\"{}\" />", w, h),
        Shape::Triangle { base, height } =>
            format!("<polygon points=\"0,{} {},{} {},{}\" />", height, base / 2.0, 0, *base, *height),
    }
}

fn description(shape: &Shape) -> String {
    match shape {
        Shape::Circle { radius } => format!("Circle with radius {:.1}", radius),
        Shape::Rectangle { w, h } => format!("Rectangle {:.1}×{:.1}", w, h),
        Shape::Triangle { base, height } => format!("Triangle base={:.1} h={:.1}", base, height),
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { w: 4.0, h: 6.0 },
        Shape::Triangle { base: 3.0, height: 4.0 },
    ];

    for s in &shapes {
        println!("{} → area={:.2} → {}", description(s), area(s), svg(s));
    }
}
```

---

## 18.5 — Factory → Smart Constructor

OOP Factory tạo objects qua factory class. Trong FP/Rust: smart constructor = function `new()` trả `Result` — nếu data không hợp lệ, không tạo được. Không cần factory class riêng.

```rust
// filename: src/main.rs

// Smart constructor: validate + construct — đảm bảo type luôn valid
#[derive(Debug, Clone)]
struct Password(String);

impl Password {
    fn new(value: &str) -> Result<Self, Vec<String>> {
        let mut errors = vec![];

        if value.len() < 8 {
            errors.push("Must be at least 8 characters".into());
        }
        if !value.chars().any(|c| c.is_uppercase()) {
            errors.push("Must contain uppercase letter".into());
        }
        if !value.chars().any(|c| c.is_ascii_digit()) {
            errors.push("Must contain digit".into());
        }
        if !value.chars().any(|c| "!@#$%^&*".contains(c)) {
            errors.push("Must contain special character".into());
        }

        if errors.is_empty() {
            Ok(Password(value.to_string()))
        } else {
            Err(errors)
        }
    }

    fn value(&self) -> &str { &self.0 }
}

fn main() {
    let attempts = vec!["short", "NoDigitsHere!", "NoSpecial1A", "V@lid_Pass1"];

    for attempt in attempts {
        match Password::new(attempt) {
            Ok(pwd) => println!("✅ '{}' → valid password", pwd.value()),
            Err(errors) => {
                println!("❌ '{}':", attempt);
                for e in &errors { println!("   - {}", e); }
            }
        }
    }
}
```

---

## 18.6 — Decorator → Function Composition

OOP Decorator wrap object bằng wrapper class. FP: wrap function bằng function khác. Stack nhiều layers bằng composition — không cần inheritance chain.

```rust
// filename: src/main.rs

type Handler = Box<dyn Fn(&str) -> String>;

// "Decorators" = functions wrapping functions
fn with_logging(next: Handler) -> Handler {
    Box::new(move |input| {
        println!("  [LOG] Input: '{}'", input);
        let result = next(input);
        println!("  [LOG] Output: '{}'", result);
        result
    })
}

fn with_timing(next: Handler) -> Handler {
    Box::new(move |input| {
        let start = std::time::Instant::now();
        let result = next(input);
        println!("  [TIME] {}µs", start.elapsed().as_micros());
        result
    })
}

fn with_trimming(next: Handler) -> Handler {
    Box::new(move |input| {
        let trimmed = input.trim();
        next(trimmed)
    })
}

fn main() {
    // Base handler
    let handler: Handler = Box::new(|input: &str| input.to_uppercase());

    // Stack decorators: trim → log → time → uppercase
    let decorated = with_timing(with_logging(with_trimming(handler)));

    println!("Result: {}", decorated("  hello rust  "));
}
```

---

## 18.7 — Adapter → `From`/`Into`

```rust
// filename: src/main.rs

// External API trả format khác
#[derive(Debug)]
struct ExternalUser {
    full_name: String,
    email_address: String,
    is_active: bool,
}

// Domain model — format của chúng ta
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    status: UserStatus,
}

#[derive(Debug)]
enum UserStatus { Active, Inactive }

// Adapter = From implementation
impl From<ExternalUser> for User {
    fn from(ext: ExternalUser) -> Self {
        User {
            name: ext.full_name,
            email: ext.email_address.to_lowercase(),
            status: if ext.is_active { UserStatus::Active } else { UserStatus::Inactive },
        }
    }
}

fn process_user(user: impl Into<User>) {
    let user: User = user.into();
    println!("Processing: {:?}", user);
}

fn main() {
    let external = ExternalUser {
        full_name: "Nguyen Minh".into(),
        email_address: "Minh@Company.COM".into(),
        is_active: true,
    };

    // Tự động convert!
    process_user(external);
    // Processing: User { name: "Nguyen Minh", email: "minh@company.com", status: Active }
}
```

---

## 📊 Bảng tổng hợp: GoF → FP

| GoF Pattern | OOP Implementation | Rust/FP Translation | LoC giảm |
|---|---|---|---|
| **Strategy** | Interface + N classes | `Fn` closure / function param | ~80% |
| **Command** | Interface + N command classes | Enum variants + match | ~70% |
| **Observer** | Interface + subscriber list | Vec of closures / channels | ~60% |
| **Visitor** | Double dispatch + interfaces | Pattern matching | ~75% |
| **Factory** | Factory classes | Smart constructors | ~60% |
| **Decorator** | Wrapper classes | Function composition | ~70% |
| **Adapter** | Adapter classes | `From`/`Into` trait | ~80% |
| **Iterator** | Iterator interface | `Iterator` trait (built-in!) | 100% |
| **Singleton** | Static instance + lock | Module-level const/lazy_static | ~50% |
| **Builder** | Builder class | Builder struct + method chain | ~same |

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Xác định pattern

```rust
fn process(items: &[i32], transform: impl Fn(i32) -> i32) -> Vec<i32> {
    items.iter().map(|&x| transform(x)).collect()
}
```
Đây là GoF pattern nào?

<details><summary>✅ Lời giải</summary>
<b>Strategy pattern</b> — <code>transform</code> là strategy được inject vào function. Caller quyết định behavior.
</details>

---

**Bài 2** (10 phút): Calculator commands

Viết enum `CalcCommand { Add(f64), Sub(f64), Mul(f64), Div(f64), Reset }`. Implement `fn execute(state: f64, cmd: &CalcCommand) -> Result<f64, String>` (pure function). Process danh sách commands bằng `fold`.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
#[derive(Debug)]
enum CalcCommand {
    Add(f64),
    Sub(f64),
    Mul(f64),
    Div(f64),
    Reset,
}

fn execute(state: f64, cmd: &CalcCommand) -> Result<f64, String> {
    match cmd {
        CalcCommand::Add(n) => Ok(state + n),
        CalcCommand::Sub(n) => Ok(state - n),
        CalcCommand::Mul(n) => Ok(state * n),
        CalcCommand::Div(n) if *n == 0.0 => Err("Division by zero".into()),
        CalcCommand::Div(n) => Ok(state / n),
        CalcCommand::Reset => Ok(0.0),
    }
}

fn main() {
    let commands = vec![
        CalcCommand::Add(10.0),
        CalcCommand::Mul(3.0),
        CalcCommand::Sub(5.0),
        CalcCommand::Div(5.0),
    ];

    let result = commands.iter().try_fold(0.0_f64, |acc, cmd| {
        let next = execute(acc, cmd)?;
        println!("  {:?} → {:.1}", cmd, next);
        Ok(next)
    });

    println!("Final: {:?}", result);
    // Add(10) → 10.0, Mul(3) → 30.0, Sub(5) → 25.0, Div(5) → 5.0
    // Final: Ok(5.0)
}
```

</details>

---

**Bài 3** (15 phút): Middleware pipeline

Viết `MiddlewareStack` lưu `Vec<Box<dyn Fn(Request) -> Request>>`. Methods: `use_middleware(f)` (builder), `handle(req: Request) -> Request`. Tạo 3 middlewares: `auth_check`, `rate_limiter`, `logger`. Chạy request qua stack.

<details><summary>✅ Lời giải Bài 3</summary>

```rust
#[derive(Debug, Clone)]
struct Request {
    path: String,
    headers: Vec<(String, String)>,
    body: String,
}

struct MiddlewareStack {
    middlewares: Vec<Box<dyn Fn(Request) -> Request>>,
}

impl MiddlewareStack {
    fn new() -> Self { MiddlewareStack { middlewares: vec![] } }

    fn use_middleware<F: Fn(Request) -> Request + 'static>(mut self, f: F) -> Self {
        self.middlewares.push(Box::new(f));
        self
    }

    fn handle(&self, req: Request) -> Request {
        self.middlewares.iter().fold(req, |r, mw| mw(r))
    }
}

fn main() {
    let stack = MiddlewareStack::new()
        .use_middleware(|mut req| {
            println!("[Logger] {} {}", "→", req.path);
            req
        })
        .use_middleware(|mut req| {
            req.headers.push(("X-Request-Id".into(), "abc123".into()));
            req
        })
        .use_middleware(|mut req| {
            if !req.headers.iter().any(|(k, _)| k == "Authorization") {
                req.headers.push(("X-Auth".into(), "anonymous".into()));
            }
            req
        });

    let req = Request {
        path: "/api/users".into(),
        headers: vec![],
        body: "{}".into(),
    };

    let processed = stack.handle(req);
    println!("Headers: {:?}", processed.headers);
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Tôi quen OOP, khó bỏ patterns" | Thói quen tư duy | Hỏi: "Pattern này giải quyết gì? Rust có feature nào sẵn cho vấn đề đó?" |
| Quá nhiều Box\<dyn Fn\> | Over-abstracting | Dùng generics `impl Fn` khi static dispatch đủ |
| Enum quá lớn (20+ variants) | Monolith command enum | Tách thành sub-enums theo domain |
| Decorator stack quá sâu | Khó debug | Dùng `inspect` cho debug, hoặc refactor thành explicit pipeline |

---

## Tóm tắt

- ✅ **Strategy** = `impl Fn` param. Không cần interface + classes.
- ✅ **Command** = enum variant. Data + behavior tập trung. History tự nhiên.
- ✅ **Observer** = `Vec<Box<dyn Fn>>` callbacks.
- ✅ **Visitor** = pattern match. Thêm "visitor" = thêm function, không sửa data.
- ✅ **Factory** = smart constructor returning `Result`. Validation built-in.
- ✅ **Decorator** = function wrapping function. Composable.
- ✅ **Adapter** = `From`/`Into`. Type-safe, idiomatic.
- ✅ FP/Rust **đơn giản hơn OOP** cho hầu hết patterns vì closures, enums, traits **đã có sẵn**.

## Tiếp theo

→ Chapter 19: **CQRS & Event Sourcing** — bạn sẽ học tách read/write models, lưu events thay vì state, rebuild state bằng `fold`, và projections — patterns cực mạnh cho domain-driven systems.
