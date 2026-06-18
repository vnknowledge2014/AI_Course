# Chapter 15 — Pattern Matching Mastery

> **Bạn sẽ học được**:
> - Patterns nâng cao: nested destructuring, `@` bindings, `ref`/`ref mut`
> - Guards phức tạp: multiple conditions, combining với `|`
> - `if let` chains và `let-else` — modern Rust patterns
> - Exhaustive matching — compiler verify logic cho bạn
> - Thiết kế API dùng pattern matching: state machines, validation, parsing
>
> **Yêu cầu trước**: Chapter 6 (Control Flow), Chapter 14 (Algebraic Types).
> **Thời gian đọc**: ~40 phút | **Level**: Intermediate
> **Kết quả cuối cùng**: Pattern matching trở thành công cụ thiết kế chính — bạn "nói chuyện" với compiler để verify logic.

---

## Tại sao Pattern Matching thay đổi cách bạn viết code?

Nếu bạn đến từ Java, C#, hoặc Python, bạn quen điều khiển flow bằng `if/else` và `switch`. Code kiểu đó trông như thế này:

```
if order.status == "paid" {
    if order.total > 500_000 {
        // free shipping
    } else if order.items > 3 {
        // reduced shipping
    } else {
        // standard shipping
    }
} else if order.status == "pending" {
    // ...
} else {
    // ???
}
```

Vấn đề: bạn đang tự chịu trách nhiệm kiểm tra từng trường hợp. Quên một case? Compiler im lặng, bug chỉ xuất hiện lúc runtime — thường là 3 giờ sáng, khi production đang cháy.

Pattern matching trong Rust đảo ngược hoàn toàn mô hình này. Thay vì bạn hỏi "giá trị này bằng gì?", bạn khai báo "data này có thể có những hình dạng nào?" và compiler đảm bảo bạn xử lý **hết**. Quên một case? Code không compile. Không có chuyện "tôi quên xử lý trường hợp đó" — compiler là người gác cổng.

Sự khác biệt này nghe nhỏ nhưng hệ quả rất lớn. Trong OOP, chúng ta phải viết hàng trăm dòng defensive code — null checks, type guards, validation. Trong Rust, compiler làm tất cả những việc đó miễn phí, tại compile time.

Scott Wlaschin, tác giả *Domain Modeling Made Functional*, gọi đây là "making illegal states unrepresentable" — thiết kế types sao cho trạng thái sai **không thể tồn tại**. Pattern matching là công cụ chính để hiện thực hóa triết lý đó.

Trong chapter này, chúng ta sẽ đi từ patterns cơ bản đến nâng cao, và kết thúc bằng một bài toán thiết kế thực tế: state machine cho authentication flow. Đến cuối chapter, bạn sẽ thấy pattern matching không chỉ là syntax — nó là cách tư duy.

---

## 15.1 — Patterns ở mọi nơi

### Máy X-quang cho data

Hãy tưởng tượng bạn là nhân viên hải quan tại sân bay. Hàng trăm vali đi qua băng chuyền — từ bên ngoài, chúng trông giống nhau: hộp đen, kích thước tương tự, không nhãn mác. Nhưng bạn có máy X-quang. Máy nhìn xuyên qua vỏ ngoài, cho bạn thấy bên trong: hình dạng, số lượng, vị trí từng vật phẩm. Dựa vào hình ảnh đó, bạn phân loại ngay: cho qua, kiểm tra kỹ hơn, hay giữ lại.

Pattern matching trong Rust hoạt động y hệt. Một giá trị — dù là struct, enum, tuple, hay Option — trông giống "hộp đen" từ ngoài. Pattern matching mở hộp ra, nhìn vào cấu trúc bên trong, và quyết định: đó là hình dạng nào?

Điều đáng chú ý: Rust không chỉ dùng patterns trong `match`. Patterns xuất hiện ở **khắp nơi** trong ngôn ngữ — nhiều hơn bạn tưởng:

```rust
// filename: src/main.rs
fn main() {
    // 1. let = pattern
    let (x, y, z) = (1, 2, 3);
    let [first, _, last] = [10, 20, 30];
    println!("({}, {}, {}), [{}, {}]", x, y, z, first, last);

    // 2. function params = pattern
    fn distance((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }
    println!("Distance: {:.2}", distance((0.0, 0.0), (3.0, 4.0)));

    // 3. for = pattern
    let pairs = vec![(1, "one"), (2, "two"), (3, "three")];
    for (num, word) in &pairs {
        println!("{} = {}", num, word);
    }

    // 4. if let = pattern
    let config: Option<u16> = Some(8080);
    if let Some(port) = config {
        println!("Port: {}", port);
    }

    // 5. while let = pattern
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();
}
```

Hãy dừng lại và nhìn kỹ đoạn code trên. Bạn đã dùng patterns ở **năm vị trí khác nhau** mà có thể không nhận ra:

- **`let (x, y, z) = ...`** — destructuring assignment. Bạn không gán "một giá trị vào một biến" — bạn đang nói "giá trị này có hình dạng 3 phần tử, tách chúng ra".
- **`fn distance((x1, y1): ...)`** — function parameter cũng là pattern. Thay vì nhận tuple rồi truy cập `.0`, `.1`, bạn destructure ngay tại chữ ký.
- **`for (num, word) in ...`** — mỗi vòng lặp cũng dùng pattern để tách cặp giá trị.
- **`if let Some(port) = config`** — "nếu `config` có hình dạng `Some`, lấy giá trị bên trong ra". Gọn hơn `match` khi chỉ quan tâm một case.
- **`while let Some(top) = stack.pop()`** — lặp liên tục cho đến khi `pop()` trả về `None`.

Nhận ra pattern rồi, bạn sẽ thấy chúng ở mọi nơi trong Rust code. Đây không phải feature bổ sung — đây là **xương sống** của ngôn ngữ.

> **So sánh với ngôn ngữ khác**: JavaScript ES6 có destructuring (`const { name, age } = user`) nhưng không có exhaustive checking. TypeScript có discriminated unions nhưng kiểm tra chỉ ở mức type, không bắt buộc xử lý hết. Python 3.10 có structural pattern matching nhưng quá mới và chưa phổ biến. Rust's pattern matching là sự kết hợp hoàn chỉnh nhất: destructure + exhaust + compile-time guarantee.

---

## 15.2 — Nested Destructuring: Tách deep structures

Patterns cơ bản — tách tuple, destructure struct — bạn đã thấy ở chapters trước. Giờ hãy nâng cấp lên bài toán thực tế: data lồng nhiều tầng.

Trong thực tế, domain data hiếm khi phẳng. Một đơn hàng chứa thanh toán, thanh toán chứa loại tiền tệ, kết quả đơn hàng chứa đơn hàng... Cấu trúc lồng nhau là bình thường. Vấn đề: làm sao truy cập sâu vào cấu trúc mà không phải viết hàng loạt `if` và `unwrap`?

Câu trả lời: nested patterns. Bạn mô tả hình dạng mong muốn ở **mọi tầng**, và Rust sẽ match toàn bộ cùng lúc.

### Enum trong Enum

Hãy xem một bài toán e-commerce: kết quả đơn hàng (`OrderResult`) chứa phương thức thanh toán (`Payment`), và thanh toán chứa loại tiền (`Currency`). Ba tầng lồng nhau.

```rust
// filename: src/main.rs

#[derive(Debug)]
enum Currency { VND, USD, EUR }

#[derive(Debug)]
enum Payment {
    Cash(u64, Currency),
    Card { amount: u64, currency: Currency, last_four: String },
    Crypto { amount: f64, token: String },
}

#[derive(Debug)]
enum OrderResult {
    Success(Payment),
    Failed { reason: String, attempted: Payment },
    Cancelled,
}

fn describe(result: &OrderResult) -> String {
    match result {
        // Nested: tách OrderResult → Payment → Currency
        OrderResult::Success(Payment::Cash(amount, Currency::VND)) =>
            format!("✅ Cash {}đ", amount),

        OrderResult::Success(Payment::Cash(amount, currency)) =>
            format!("✅ Cash {:?} {}", currency, amount),

        OrderResult::Success(Payment::Card { amount, last_four, .. }) =>
            format!("✅ Card ****{}: {}đ", last_four, amount),

        OrderResult::Success(Payment::Crypto { token, amount }) =>
            format!("✅ {} {:.4}", token, amount),

        OrderResult::Failed { reason, attempted } =>
            format!("❌ Failed: {} (tried {:?})", reason, attempted),

        OrderResult::Cancelled =>
            "🚫 Cancelled".to_string(),
    }
}

fn main() {
    let results = vec![
        OrderResult::Success(Payment::Cash(500_000, Currency::VND)),
        OrderResult::Success(Payment::Card {
            amount: 1_200_000,
            currency: Currency::VND,
            last_four: "4444".into(),
        }),
        OrderResult::Success(Payment::Crypto { amount: 0.0025, token: "BTC".into() }),
        OrderResult::Failed {
            reason: "Insufficient funds".into(),
            attempted: Payment::Cash(999_999, Currency::USD),
        },
        OrderResult::Cancelled,
    ];

    for r in &results {
        println!("{}", describe(r));
    }
}
```

Dừng lại ở arm đầu tiên — `OrderResult::Success(Payment::Cash(amount, Currency::VND))`. Một dòng duy nhất nhưng làm ba việc cùng lúc: kiểm tra kết quả là `Success`, kiểm tra thanh toán là `Cash`, **và** kiểm tra loại tiền là `VND`. Trong imperative code, bạn cần ba `if` lồng nhau để làm điều tương tự.

Lưu ý thứ tự các arms cũng quan trọng. Arm `Cash(amount, Currency::VND)` đặt **trước** arm `Cash(amount, currency)` — vì VND là trường hợp cụ thể hơn. Nếu đảo thứ tự, arm tổng quát sẽ "nuốt" hết và arm VND không bao giờ được chạy (compiler sẽ cảnh báo `unreachable pattern`).

### Struct trong Vec trong Option

Nested patterns cũng hoạt động khi kết hợp structs và `Option`:

```rust
// filename: src/main.rs

#[derive(Debug)]
struct Address {
    city: String,
    zip: String,
}

#[derive(Debug)]
struct User {
    name: String,
    address: Option<Address>,
}

fn greeting(user: &User) -> String {
    match user {
        // Nested: User → Option → Address
        User { name, address: Some(Address { city, .. }) } =>
            format!("Hello {} from {}!", name, city),
        User { name, address: None } =>
            format!("Hello {}!", name),
    }
}

fn main() {
    let users = vec![
        User { name: "Minh".into(), address: Some(Address { city: "HCMC".into(), zip: "70000".into() }) },
        User { name: "Lan".into(), address: None },
    ];

    for u in &users {
        println!("{}", greeting(u));
    }
    // Hello Minh from HCMC!
    // Hello Lan!
}
```

Một điều thú vị ở đây: pattern `User { name, address: Some(Address { city, .. }) }` nhìn phức tạp, nhưng nó diễn đạt **chính xác** ý đồ: "nếu user có address, lấy city ra". Không cần `user.address.as_ref().map(|a| &a.city)` — pattern matching nói thẳng điều bạn muốn.

`..` trong `Address { city, .. }` có nghĩa "tôi không quan tâm các fields còn lại". Đây là cách Rust cho phép bạn chọn lọc — chỉ lấy những gì cần, bỏ qua phần thừa.

---

## ✅ Checkpoint 15.2

> Ghi nhớ:
> 1. Patterns lồng nhau: `Success(Card { amount, .. })` = tách nhiều tầng cùng lúc
> 2. `..` bỏ qua fields/variants không cần — code gọn, tập trung vào ý chính
> 3. Rust kiểm tra **tất cả** variants — quên 1 cái = compile error
> 4. Thứ tự arms: cụ thể trước, tổng quát sau
>
> **Test nhanh**: `Some(Some(42))` match pattern nào?
> <details><summary>Đáp án</summary><code>Some(Some(n))</code> với n = 42. Nested Option destructuring — hai lớp "hộp" cần mở.</details>

---

## 15.3 — Guards & Bindings nâng cao

Patterns cho phép bạn match theo **hình dạng** — enum variant nào, struct có fields gì. Nhưng đôi khi hình dạng đúng mà **giá trị** sai. Ví dụ: `Payment::Cash(amount, Currency::VND)` match mọi thanh toán tiền mặt VND, nhưng bạn chỉ muốn xử lý đặc biệt khi `amount > 500_000`.

Đây là lúc guards xuất hiện. Guard là điều kiện `if` đi kèm pattern — "match hình dạng này **và** thỏa điều kiện này":

### Complex guards

Hãy xem bài toán tính phí vận chuyển — logic phụ thuộc vào nhiều yếu tố cùng lúc:

```rust
// filename: src/main.rs

#[derive(Debug)]
struct Order {
    total: u32,
    items: u32,
    is_vip: bool,
}

fn shipping_fee(order: &Order) -> u32 {
    match order {
        // Free shipping: VIP hoặc order > 500k
        Order { is_vip: true, .. } => 0,
        Order { total, .. } if *total >= 500_000 => 0,

        // Reduced: 3+ items
        Order { items, total, .. } if *items >= 3 && *total >= 200_000 => 15_000,

        // Standard
        Order { total, .. } if *total >= 100_000 => 30_000,

        // Small orders
        _ => 50_000,
    }
}

fn main() {
    let orders = vec![
        Order { total: 600_000, items: 2, is_vip: false },
        Order { total: 300_000, items: 5, is_vip: false },
        Order { total: 150_000, items: 1, is_vip: false },
        Order { total: 50_000, items: 1, is_vip: false },
        Order { total: 50_000, items: 1, is_vip: true },
    ];

    for o in &orders {
        println!("{:?} → shipping: {}đ", o, shipping_fee(o));
    }
}
```

Đoạn code này thay thế một chuỗi `if/else if` dài ở ngôn ngữ khác, nhưng mỗi arm tự mô tả ý đồ rõ ràng. Đọc từ trên xuống như đọc bảng quy tắc kinh doanh:
- VIP → miễn phí
- Đơn lớn (≥500k) → miễn phí
- Nhiều sản phẩm + trung bình → giảm phí
- Đơn vừa → phí chuẩn
- Còn lại → phí cao nhất

Lưu ý `*total` — vì `total` là reference (chúng ta match trên `&Order`), cần dereference bằng `*` để so sánh giá trị. Đây là chi tiết dễ quên khi mới bắt đầu, nhưng compiler sẽ nhắc bạn.

### `@` bindings — Bắt giá trị VÀ kiểm tra pattern

Đôi khi bạn muốn cả hai: kiểm tra giá trị thuộc range nào **và** sử dụng giá trị đó. Toán tử `@` giải quyết nhu cầu này — nó "gắn tên" cho giá trị đã match:

```rust
// filename: src/main.rs

fn classify_response(status: u16) -> &'static str {
    match status {
        // @ binding: bắt giá trị vào biến VÀ kiểm tra range
        code @ 200..=299 => {
            println!("  (success code: {})", code);
            "Success"
        }
        code @ 300..=399 => {
            println!("  (redirect code: {})", code);
            "Redirect"
        }
        code @ 400..=499 => {
            println!("  (client error: {})", code);
            "Client Error"
        }
        code @ 500..=599 => {
            println!("  (server error: {})", code);
            "Server Error"
        }
        _ => "Unknown",
    }
}

fn main() {
    for code in [200, 301, 404, 500, 999] {
        println!("{}: {}", code, classify_response(code));
    }
}
```

Không có `@`, bạn phải viết `200..=299 => { ... }` nhưng không có cách nào truy cập giá trị cụ thể (200? 201? 299?). Guard `if status >= 200 && status <= 299` thì được, nhưng dài hơn **và** compiler không biết đây là range pattern → mất tối ưu hóa.

`@` là "cầu nối" giữa pattern matching (kiểm tra hình dạng) và variable binding (sử dụng giá trị). Bạn sẽ thấy nó hữu ích nhất khi xử lý HTTP status codes, version numbers, hoặc bất kỳ đâu cần "phân loại nhưng vẫn giữ giá trị gốc".

### `@` với enums — Bắt toàn bộ variant

`@` còn mạnh hơn khi kết hợp với enums — bạn có thể bắt **toàn bộ variant** cho debug logging trong khi vẫn destructure fields:

```rust
// filename: src/main.rs

#[derive(Debug)]
enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
}

fn execute(cmd: &Command) {
    match cmd {
        // @ bắt toàn bộ variant VÀ destructure
        c @ Command::Move { x, y } if *x == 0 && *y == 0 => {
            println!("Ignoring zero move: {:?}", c);
        }
        Command::Move { x, y } => {
            println!("Moving to ({}, {})", x, y);
        }
        Command::Write(text) if text.is_empty() => {
            println!("Ignoring empty write");
        }
        Command::Write(text) => {
            println!("Writing: {}", text);
        }
        Command::Color(r, g, b) => {
            println!("Color: #{:02X}{:02X}{:02X}", r, g, b);
        }
        Command::Quit => println!("Quitting"),
    }
}

fn main() {
    let commands = vec![
        Command::Move { x: 0, y: 0 },
        Command::Move { x: 10, y: 20 },
        Command::Write("".into()),
        Command::Write("hello".into()),
        Command::Color(255, 128, 0),
        Command::Quit,
    ];

    for cmd in &commands {
        execute(cmd);
    }
}
```

Arm đầu tiên — `c @ Command::Move { x, y } if *x == 0 && *y == 0` — kết hợp ba kỹ thuật cùng lúc: `@` binding (bắt toàn bộ vào `c`), destructuring (`x`, `y`), và guard (`if *x == 0`). Ba dòng `if` trong imperative code gói gọn thành một arm pattern match.

---

## 15.4 — `let-else` và `if let` chains (Modern Rust)

Trước Rust 1.65, khi bạn cần "lấy giá trị ra hoặc return sớm", bạn phải viết `match` đầy đủ:

```rust
let port = match config.get("port") {
    Some(p) => p,
    None => return Err("Missing port"),
};
```

Ba dòng cho một thao tác đơn giản. Khi có 5-6 lần kiểm tra liên tiếp, code trở thành kim tự tháp `match` lồng nhau — khó đọc, khó maintain.

`let-else` giải quyết vấn đề này bằng cú pháp gọn gàng hơn: "match pattern này, nếu không match thì diverge (return/break/continue)".

### `let-else` (Rust 1.65+): Early return khi pattern không match

```rust
// filename: src/main.rs

fn process_config(map: &std::collections::HashMap<String, String>) -> Result<String, String> {
    // let-else: match → tiếp tục, else → return/break/continue
    let Some(host) = map.get("host") else {
        return Err("Missing 'host'".into());
    };

    let Some(port_str) = map.get("port") else {
        return Err("Missing 'port'".into());
    };

    let Ok(port) = port_str.parse::<u16>() else {
        return Err(format!("Invalid port: {}", port_str));
    };

    // Tại đây: host và port chắc chắn valid
    Ok(format!("{}:{}", host, port))
}

fn main() {
    use std::collections::HashMap;

    let mut config = HashMap::new();
    config.insert("host".into(), "localhost".into());
    config.insert("port".into(), "8080".into());

    println!("{:?}", process_config(&config));
    // Ok("localhost:8080")

    config.remove("port");
    println!("{:?}", process_config(&config));
    // Err("Missing 'port'")
}
```

So sánh trước và sau: mỗi `let-else` thay thế 3-4 dòng `match`. Quan trọng hơn, code đọc từ trên xuống theo "happy path" — nếu bạn đến được dòng `Ok(format!(...))`, bạn **biết chắc** host và port đều valid. Không cần nhìn lại.

Đây chính là pattern "Parse, Don't Validate" mà Alexis King viết trong bài blog nổi tiếng cùng tên. Thay vì validate xong rồi truyền nguyên liệu thô (có thể quên check), bạn parse thành giá trị đã-verified và trả về sớm nếu thất bại.

### `if let` chains — Kiểm tra nhiều điều kiện

Khi bạn cần kiểm tra nhiều `Option`/`Result` nhưng chỉ quan tâm case "tất cả đều có", `if let` chain là lựa chọn:

```rust
// filename: src/main.rs

#[derive(Debug)]
struct Request {
    auth_token: Option<String>,
    body: Option<String>,
    method: String,
}

fn handle_request(req: &Request) -> String {
    // Chain if let: mỗi điều kiện phải pass
    if let Some(token) = &req.auth_token {
        if let Some(body) = &req.body {
            if req.method == "POST" {
                return format!("✅ POST with auth={} body_len={}", token, body.len());
            }
        }
    }

    // Fallback
    format!("❌ Unauthorized or invalid request")
}

// Cleaner alternative: extract + match tuple
fn handle_request_v2(req: &Request) -> String {
    match (&req.auth_token, &req.body, req.method.as_str()) {
        (Some(token), Some(body), "POST") =>
            format!("✅ POST auth={} body_len={}", token, body.len()),
        (Some(_), _, method) =>
            format!("⚠️ {} — no body or wrong method", method),
        (None, _, _) =>
            "❌ Unauthorized".to_string(),
    }
}

fn main() {
    let req = Request {
        auth_token: Some("abc123".into()),
        body: Some("{\"key\": \"value\"}".into()),
        method: "POST".into(),
    };
    println!("{}", handle_request_v2(&req));

    let no_auth = Request { auth_token: None, body: None, method: "GET".into() };
    println!("{}", handle_request_v2(&no_auth));
}
```

Hai phiên bản cùng logic, nhưng `v2` dùng match trên tuple — gọn hơn và exhaustive. Trick ở đây: gom nhiều giá trị cần kiểm tra vào một tuple, rồi match toàn bộ cùng lúc. Đây là một trong những patterns phổ biến nhất trong Rust thực tế.

> **Khi nào dùng gì?**
> - `match` — khi cần xử lý **nhiều** cases, hoặc cần exhaustive checking
> - `if let` — khi chỉ quan tâm **một** case, phần còn lại bỏ qua
> - `let-else` — khi cần **early return** — "lấy giá trị ra hoặc thoát ngay"

---

## 15.5 — Exhaustive Matching: Compiler là đồng đội

Đây là phần quan trọng nhất của chapter — và có lẽ là feature quan trọng nhất của Rust's type system.

Bạn có bao giờ thêm một giá trị mới vào enum (ví dụ thêm `Delete` vào `Permission`) và quên cập nhật **một** trong 17 chỗ sử dụng enum đó? Trong Java hoặc Python, có — và bug chỉ xuất hiện ở production. Trong Rust, không bao giờ. Compiler sẽ liệt kê **tất cả** các vị trí cần sửa.

Đây không phải lý thuyết suông. Hãy xem thực tế:

### Tại sao exhaustive matching là superpower?

```rust
// filename: src/main.rs

#[derive(Debug)]
enum Permission {
    Read,
    Write,
    Execute,
    Admin,
}

// Thêm variant mới? Compiler báo MỌI CHỖ cần sửa!
// Thử uncomment Delete:
// #[derive(Debug)]
// enum Permission {
//     Read, Write, Execute, Admin, Delete,  // ← MỚI
// }
// → Compiler error ở MỌI match block thiếu Delete!

fn can_modify(perm: &Permission) -> bool {
    match perm {
        Permission::Write | Permission::Admin => true,
        Permission::Read | Permission::Execute => false,
        // Nếu thêm Delete → compiler bắt buộc xử lý ở đây!
    }
}

fn permission_level(perm: &Permission) -> u8 {
    match perm {
        Permission::Read => 1,
        Permission::Write => 2,
        Permission::Execute => 3,
        Permission::Admin => 4,
        // Thêm variant → BAT BUOC thêm arm ở đây!
    }
}

fn main() {
    let perms = vec![Permission::Read, Permission::Write, Permission::Admin];
    for p in &perms {
        println!("{:?}: level={}, can_modify={}", p, permission_level(p), can_modify(p));
    }
}
```

Thử thí nghiệm: uncomment dòng `Delete` variant. Ngay lập tức, compiler sẽ báo lỗi ở **cả hai** functions — `can_modify` và `permission_level`. Không phải một, mà tất cả. Trong codebase 50 files với 200 chỗ match enum đó, compiler vẫn tìm được hết.

Đây là lý do dân Rust thường nói "if it compiles, it works" — không phải lúc nào cũng đúng, nhưng exhaustive matching loại bỏ một lớp bugs rất lớn: "quên xử lý trường hợp mới".

> **💡 Đây là lý do tránh `_ =>` khi có thể**: `_ =>` bắt tất cả → compiler **không báo** khi thêm variant mới. Toàn bộ power của exhaustive matching biến mất. Chỉ dùng `_ =>` khi bạn thực sự muốn "mọi thứ khác" — ví dụ HTTP status code có hàng trăm giá trị, bạn không cần list hết.

### Non-exhaustive enums từ external crates

Trong thực tế, không phải lúc nào bạn cũng sở hữu enum. Khi dùng enum từ crate khác, tác giả crate có thể đánh dấu `#[non_exhaustive]` — nghĩa là "tôi có thể thêm variants mới trong tương lai". Trong trường hợp này, Rust **bắt buộc** bạn phải có `_ =>`:

```rust
// filename: src/main.rs

// #[non_exhaustive] — crate khác PHẢI có _ => arm
#[non_exhaustive]
#[derive(Debug)]
enum ApiError {
    NotFound,
    Unauthorized,
    RateLimit,
}

fn handle(err: &ApiError) -> &str {
    match err {
        ApiError::NotFound => "Not found",
        ApiError::Unauthorized => "Unauthorized",
        ApiError::RateLimit => "Rate limited",
        _ => "Unknown error",  // BẮT BUỘC vì #[non_exhaustive]
    }
}

fn main() {
    println!("{}", handle(&ApiError::NotFound));
}
```

`#[non_exhaustive]` là hợp đồng giữa library author và users: "tôi giữ quyền mở rộng, bạn phải sẵn sàng cho trường hợp mới". Thiết kế cẩn thận này giúp libraries evolve mà không breaking downstream code.

---

## 15.6 — Pattern Matching trong thiết kế

Đến đây bạn đã nắm vững cú pháp. Giờ hãy xem pattern matching giải quyết bài toán thiết kế thực tế — loại bài toán mà trong OOP phải dùng State pattern với abstract classes và inheritance.

### State machine với exhaustive transitions

Authentication flow là ví dụ kinh điển: user bắt đầu Anonymous, nhập username, submit password, rồi thành Authenticated hoặc bị Locked. Mỗi trạng thái chỉ cho phép một số hành động nhất định.

Trong OOP, bạn cần: `LoginState` interface, `AnonymousState` class, `AuthenticatingState` class... 5 classes, mỗi class override methods khác nhau. Rất nhiều boilerplate.

Trong Rust, hai enums + một function:

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
enum LoginState {
    Anonymous,
    EnteringCredentials { username: String },
    Authenticating { username: String, password: String },
    Authenticated { user_id: u64, username: String },
    Locked { username: String, attempts: u32 },
}

#[derive(Debug)]
enum LoginAction {
    StartLogin(String),
    SubmitPassword(String),
    AuthSuccess(u64),
    AuthFailed,
    Reset,
}

fn transition(state: LoginState, action: LoginAction) -> LoginState {
    match (state, action) {
        // Anonymous → nhập username
        (LoginState::Anonymous, LoginAction::StartLogin(username)) =>
            LoginState::EnteringCredentials { username },

        // Nhập xong → submit password
        (LoginState::EnteringCredentials { username }, LoginAction::SubmitPassword(password)) =>
            LoginState::Authenticating { username, password },

        // Auth thành công
        (LoginState::Authenticating { username, .. }, LoginAction::AuthSuccess(user_id)) =>
            LoginState::Authenticated { user_id, username },

        // Auth thất bại → quay lại (hoặc lock)
        (LoginState::Authenticating { username, .. }, LoginAction::AuthFailed) =>
            LoginState::EnteringCredentials { username },

        // Reset từ bất kỳ state nào
        (_, LoginAction::Reset) => LoginState::Anonymous,

        // Mọi transition khác: giữ nguyên state (hoặc báo lỗi)
        (state, action) => {
            println!("  ⚠️ Invalid: {:?} in {:?}", action, state);
            state
        }
    }
}

fn main() {
    let mut state = LoginState::Anonymous;
    println!("State: {:?}", state);

    let actions = vec![
        LoginAction::StartLogin("minh".into()),
        LoginAction::SubmitPassword("pass123".into()),
        LoginAction::AuthSuccess(42),
    ];

    for action in actions {
        println!("Action: {:?}", action);
        state = transition(state, action);
        println!("State: {:?}\n", state);
    }
}
```

Function `transition` là **toàn bộ** logic state machine. Đọc từ trên xuống, bạn thấy ngay: từ `Anonymous` chỉ có thể `StartLogin`, từ `EnteringCredentials` chỉ có thể `SubmitPassword`, v.v. Compiler đảm bảo bạn không quên case nào.

So sánh với State pattern trong OOP: ở đó, logic phân tán qua 5 classes. Thêm một trạng thái mới? Tạo class mới, implement interface, cập nhật factory... Ở Rust: thêm variant vào enum → compiler chỉ chỗ cần thêm arm → xong. Tất cả ở một chỗ, compiler verify toàn bộ.

Đây chính là sức mạnh khi kết hợp algebraic types (Ch14) với pattern matching (Ch15). Types mô tả **có thể** xảy ra gì, pattern matching đảm bảo bạn **xử lý hết**. Cặp đôi này thay thế cả một họ design patterns trong OOP.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Viết pattern

Match giá trị `(Option<i32>, Option<i32>)`:
- Cả hai Some → tổng
- Chỉ một Some → giá trị đó
- Cả hai None → 0

<details><summary>✅ Lời giải Bài 1</summary>

```rust
fn sum_options(a: Option<i32>, b: Option<i32>) -> i32 {
    match (a, b) {
        (Some(x), Some(y)) => x + y,
        (Some(x), None) | (None, Some(x)) => x,
        (None, None) => 0,
    }
}
```

Lưu ý arm thứ hai: `(Some(x), None) | (None, Some(x))` — toán tử `|` kết hợp hai patterns dùng chung biến `x`. Gọn hơn viết hai arms riêng.

</details>

---

**Bài 2** (10 phút): Expression evaluator

Viết enum `Expr` với: `Num(f64)`, `Add(Box<Expr>, Box<Expr>)`, `Mul(Box<Expr>, Box<Expr>)`, `Neg(Box<Expr>)`. Implement `fn eval(expr: &Expr) -> f64` dùng pattern matching.

Bài này minh họa một ứng dụng kinh điển của pattern matching: **recursive data structures**. Mỗi `Expr` có thể chứa `Expr` khác bên trong (qua `Box`), và `eval` đệ quy xuống từng tầng.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
#[derive(Debug)]
enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Neg(e) => -eval(e),
    }
}

fn main() {
    // (3 + 4) * -(2)
    let expr = Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Num(3.0)),
            Box::new(Expr::Num(4.0)),
        )),
        Box::new(Expr::Neg(Box::new(Expr::Num(2.0)))),
    );
    println!("(3 + 4) * -(2) = {}", eval(&expr));  // -14
}
```

Pattern matching + recursive enums = cách xây dựng interpreters, compilers, và query engines. Nếu bạn từng tự hỏi "làm sao database xử lý SQL query?" — câu trả lời bắt đầu từ chính pattern này: parse SQL thành AST (enum lồng nhau), rồi eval đệ quy.

</details>

---

**Bài 3** (15 phút): HTTP Router

Viết router dùng pattern matching:
- `Request { method: "GET", path: "/" }` → "Home page"
- `Request { method: "GET", path: "/users" }` → "User list"
- `Request { method: "GET", path }` nếu path bắt đầu `/users/` → "User profile: {id}"
- `Request { method: "POST", path: "/users" }` → "Create user"
- Mọi thứ khác → "404 Not Found"

Bài này cho thấy pattern matching trong web server thực tế. Framework như Axum dùng chính cơ chế tương tự (nhưng phức tạp hơn với extractors) — hiểu bài tập này giúp bạn hiểu Axum routing.

<details><summary>✅ Lời giải Bài 3</summary>

```rust
struct Request {
    method: String,
    path: String,
}

fn route(req: &Request) -> String {
    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/") => "🏠 Home page".into(),
        ("GET", "/users") => "👥 User list".into(),
        ("GET", path) if path.starts_with("/users/") => {
            let id = &path[7..];
            format!("👤 User profile: {}", id)
        }
        ("POST", "/users") => "➕ Create user".into(),
        ("DELETE", path) if path.starts_with("/users/") => {
            let id = &path[7..];
            format!("🗑️ Delete user: {}", id)
        }
        (method, path) => format!("❓ 404 {} {} Not Found", method, path),
    }
}

fn main() {
    let requests = vec![
        Request { method: "GET".into(), path: "/".into() },
        Request { method: "GET".into(), path: "/users".into() },
        Request { method: "GET".into(), path: "/users/42".into() },
        Request { method: "POST".into(), path: "/users".into() },
        Request { method: "GET".into(), path: "/unknown".into() },
    ];

    for req in &requests {
        println!("{} {} → {}", req.method, req.path, route(req));
    }
}
```

Arm cuối `(method, path) =>` — đây là "catch-all" có ý nghĩa: bắt method và path để hiển thị trong 404 message. Tốt hơn `_ =>` vì bạn giữ được thông tin debug.

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `unreachable pattern` | Pattern trên đã cover pattern dưới | Đặt pattern cụ thể **trước** `_` hoặc catch-all |
| `non-exhaustive patterns` | Thiếu variant/case | Thêm arm cho variant thiếu |
| `cannot move out of borrowed content` | Destructure trong `&` reference | Dùng `ref` hoặc clone |
| `let-else requires diverging else` | else block phải `return`/`break`/`panic!` | Thêm `return Err(...)` hoặc `continue` |
| Guard condition quá phức tạp | Quá nhiều `if` trong guard | Tách thành helper function |

---

## Tóm tắt

Chapter này trang bị cho bạn **máy X-quang** để nhìn xuyên qua data và phân loại chúng chính xác:

- ✅ **Patterns ở mọi nơi**: `let`, function params, `for`, `if let`, `while let`, `match` — patterns là xương sống của Rust, không phải feature bổ sung.
- ✅ **Nested destructuring**: `Success(Card { amount, .. })` — tách nhiều tầng cùng lúc, thay thế chuỗi `if` lồng nhau.
- ✅ **Guards & `@` bindings**: guards thêm logic (`if condition`), `@` bắt giá trị + kiểm tra (`code @ 200..=299`).
- ✅ **Modern patterns**: `let-else` cho early return gọn gàng, match trên tuples cho routing — hai patterns sẽ gặp hàng ngày.
- ✅ **Exhaustive matching = compiler verify**: thêm enum variant → compiler bắt MỌI CHỖ cần update. Feature thay đổi cách bạn thiết kế — không sợ refactor.

Pattern matching kết hợp với algebraic types (Ch14) tạo thành cặp đôi mạnh nhất trong Rust. Types mô tả "thế giới có thể trông như thế nào", pattern matching đảm bảo "bạn đã xử lý hết mọi khả năng". Cùng nhau, họ thay thế nhiều design patterns phức tạp trong OOP bằng code đơn giản, an toàn, và dễ đọc.

## Tiếp theo

Bạn đã biết nhìn xuyên data — giờ hãy học cách **hứa hẹn** data. Traits là hợp đồng: "nếu type này có trait Display, nó **cam kết** biết tự in chính mình". Chứng chỉ nghề nghiệp cho types.

→ Chapter 16: **Traits — Rust's Typeclass System** — bạn sẽ học trait declaration, implementation, default methods, trait bounds, `impl Trait` vs `dyn Trait`, và derive macros.
