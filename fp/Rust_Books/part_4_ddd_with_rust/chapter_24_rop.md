# Chapter 24 — Railway-Oriented Programming in Rust ⭐

> **Bạn sẽ học được**:
> - **Two-track model (Đường ray đôi)**: Cách tiếp cận `Result<T, E>` bằng hình ảnh Đường ray (Success track + Failure track)
> - `map`, `and_then`, `map_err` — Các cầu dao chuyển hướng giữa các đường ray
> - **Gom Tất Cả Lỗi (Collect ALL errors)** — Giải quyết điểm yếu chết người của dấu `?` bằng `Validated<T>`
> - Xây dựng hệ thống Error có cấu trúc (Error type hierarchy)
> - Áp dụng vào thực tế: Pipeline Đăng Ký Tài Khoản thu thập 100% lỗi cùng lúc.
>
> **Yêu cầu trước**: Chapter 10 (Error Handling), Chapter 23 (Workflows).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Error handling (Xử lý lỗi) không còn là những dòng `if/else` chắp vá mệt mỏi nữa, mà trở thành một **Nghệ thuật thiết kế tinh tế** (First-class design tool).

---

## 24.1 — Two-Track Model: Mô hình Đường ray đôi

Chapter 23 đã dùng `and_then` để nối các khâu xử lý lại với nhau. Đây là cách giải thích trực quan nhất của nó: Mỗi function giống như một **Đoạn đường ray có 2 làn** — Làn trên (Ok) và Làn dưới (Err). 

Khi một function bị lỗi (Fail), dữ liệu sẽ bị "rớt" xuống Làn Err, và nó sẽ **Trượt thẳng** đi qua tuốt tuồn tuột mọi trạm kiểm soát phía sau mà không bị xử lý nữa. Đó chính là "Railway-Oriented Programming" (Lập trình hướng Đường ray) — Một cách gọi dân dã của việc xử lý lỗi bằng `Result`.

### `Result<T, E>` = Đường ray đôi

Hãy nhìn bản vẽ này:

```text
Success track (Ok):  ═══╦═══╦═══╦═══╦═══→ ✅ Ra Kết Quả
                        ║   ║   ║   ║
Failure track (Err): ───╨───╨───╨───╨───→ ❌ Báo Lỗi

Mỗi function là một trạm kiểm soát (Dấu ╦).
Nếu function thành công → Chạy tiếp ở làn trên (Success track).
Nếu function thất bại → Rớt xuống làn dưới (Failure track).
Khi đã rớt xuống làn dưới → BỎ QUA tất cả các trạm còn lại và trôi về đích.
```

### Các công tắc bẻ ghi (Switch functions)

Tùy vào việc hàm của bạn trả về cái gì, chúng ta sẽ dùng những "công tắc" khác nhau để nối chúng vào đường ray:

```rust
// filename: src/main.rs

// Loại 1: Hàm không bao giờ lỗi (Pure transform)
// Đi thẳng làn trên
fn double(x: i32) -> i32 { x * 2 }
// 👉 Dùng công tắc: .map()

// Loại 2: Hàm có thể Lỗi (Nguy hiểm)
// Đang ở làn trên, có thể rớt xuống làn dưới
fn validate_positive(x: i32) -> Result<i32, String> {
    if x > 0 { Ok(x) } else { Err(format!("{} is not positive", x)) }
}
// 👉 Dùng công tắc: .and_then()

// Loại 3: Hàm sửa Lỗi
// Đi thẳng làn dưới (Nhặt rác và đóng gói lại)
fn add_context(err: String) -> String {
    format!("[Validation] {}", err)
}
// 👉 Dùng công tắc: .map_err()
```

Hãy thử cho Tàu chạy:

```rust
fn main() {
    // Kịch bản Tàu Chạy Suôn Sẻ (Happy path)
    let ok = Ok(5)
        .map(double)                       // Thành 10
        .and_then(validate_positive)       // Chạy tốt! Thành Ok(10)
        .map(double)                       // Thành 20
        .map_err(add_context);             // Làn trên không chạy qua hàm này
    println!("Happy: {:?}", ok);  // Ok(20)

    // Kịch bản Tàu Trật Bánh (Sad path)
    let err = Ok(-3)
        .map(double)                       // Thành -6
        .and_then(validate_positive)       // LỖI! Rớt xuống đường ray Err!
        .map(double)                       // BỊ BỎ QUA (Skipped)
        .map_err(add_context);             // Chạm trúng hàm gom rác
    println!("Sad: {:?}", err);   // Err("[Validation] -6 is not positive")
}
```

### Bảng Cheat Sheet

| Công tắc | Nhận vào | Ép ra | Khi nào dùng |
|--------|-------|--------|-------------|
| `.map(f)` | `Ok(T)` → `f(T) → U` | `Ok(U)` | Chế biến Dữ liệu, không bao giờ Lỗi |
| `.and_then(f)` | `Ok(T)` → `f(T) → Result<U,E>` | `Ok(U)` hoặc `Err(E)` | Làm việc Nguy hiểm, có thể Lỗi |
| `.map_err(f)` | `Err(E)` → `f(E) → F` | `Err(F)` | Gói ghém, ghi thêm Log vào Error |
| `.or_else(f)` | `Err(E)` → `f(E) → Result<T,F>` | `Ok(T)` hoặc `Err(F)` | Dùng để Báo lỗi xong thì Cứu Vãn (Retry) |

---

## 24.2 — Điểm mù của toán tử `?` (Monadic Bind)

Ở bài trước ta đã biết `?` là một cách viết rút gọn của `and_then`. Nó làm code ngắn đi rất nhiều. Nhưng nó có một **Điểm mù chí mạng**.

Hãy xem ví dụ Đăng Ký Người Dùng dưới đây:

```rust
// filename: src/main.rs

#[derive(Debug)]
struct UserInput { name: String, email: String, age: String }
#[derive(Debug)]
struct ValidUser { name: String, email: String, age: u32 }

fn validate_name(name: &str) -> Result<String, String> {
    let trimmed = name.trim();
    if trimmed.len() >= 2 { Ok(trimmed.to_string()) }
    else { Err("Name must be at least 2 characters".into()) }
}

fn validate_email(email: &str) -> Result<String, String> {
    if email.contains('@') { Ok(email.to_lowercase()) }
    else { Err(format!("Invalid email: {}", email)) }
}

fn validate_age(age_str: &str) -> Result<u32, String> {
    let age: u32 = age_str.parse().map_err(|_| format!("Invalid age"))?;
    if (18..=150).contains(&age) { Ok(age) }
    else { Err(format!("Age {} not in range 18-150", age)) }
}
```

Và đây là hàm gom tụi nó lại bằng dấu `?`:

```rust
// Dùng ? — Pipeline tự động DỪNG ở Lỗi ĐẦU TIÊN
fn validate_user(input: &UserInput) -> Result<ValidUser, String> {
    let name = validate_name(&input.name)?;    // Hễ lỗi? → Phanh lết bánh!
    let email = validate_email(&input.email)?; // Hễ lỗi? → Phanh lết bánh!
    let age = validate_age(&input.age)?;       // Hễ lỗi? → Phanh lết bánh!
    Ok(ValidUser { name, email, age })
}

fn main() {
    let bad_input = UserInput { name: "M".into(), email: "bad".into(), age: "xyz".into() };
    
    println!("Kết quả: {:?}", validate_user(&bad_input));
    // Chỉ in ra LỖI TÊN QUÁ NGẮN! Các lỗi Email và Age KHÔNG HỀ BỊ PHÁT HIỆN.
}
```

> **⚠️ Thảm họa UX (Trải nghiệm người dùng)**: Khách hàng điền 3 ô bị sai cả 3. Bạn báo lỗi ô Tên. Khách sửa tên xong nhấn Submit, bạn lại báo lỗi ô Email. Khách sửa xong Submit, bạn lại báo ô Tuổi. Khách hàng sẽ đập nát cái máy tính của họ mất!

---

## 24.3 — Thu gom TẤT CẢ Lỗi: `Validated<T>` ⭐

Vấn đề của `?` (và `and_then`) là nó **Fail Fast (Dừng ngay khi lỗi)**.
Điều chúng ta cần là một cấu trúc mới: `Validated<T>`. Nó sẽ chạy song song các bước kiểm tra, và gom Bào Cáo Tất Cả Lỗi lại một thể.

### Vấn đề và Giải pháp

```text
Toán tử ?:         Tên? ──(Lỗi)──> STOP! (Ngưng, văng lỗi)
                   
Validated Pattern: Tên ──> Lỗi
                   Email ──> Lỗi
                   Tuổi ──> Lỗi
                   == GOM LẠI ==> Mảng["Lỗi Tên", "Lỗi Email", "Lỗi Tuổi"]
```

### Xây dựng `Validated<T>`

`Validated` rất giống `Result`, nhưng vế Lỗi của nó luôn là một Mảng (Vec) chứa các thông báo lỗi:

```rust
// filename: src/main.rs

/// Validated: Hoặc là Hợp lệ Valid(T), hoặc là chứa Một Rổ Lỗi Invalid(Vec<Lỗi>)
#[derive(Debug, Clone)]
enum Validated<T> {
    Valid(T),
    Invalid(Vec<String>), // Luôn là một mảng!
}
```

Tuyệt chiêu nằm ở hàm **Combine (Gộp)**. Nó sẽ nhận vào nhiều cái `Validated`, nếu tất cả đều ngon lành thì nó Gộp lại, còn nếu có bất cứ thằng nào `Invalid` thì nó Tịch thu Lỗi của thằng đó nhét vào Rổ.

```rust
// Gộp 3 cái Validated lại với nhau
fn combine3<A, B, C, D, F: FnOnce(A, B, C) -> D>(
    va: Validated<A>,
    vb: Validated<B>,
    vc: Validated<C>,
    f: F,
) -> Validated<D> {
    match (va, vb, vc) {
        // Trường hợp Tuyệt Mỹ: Cả 3 đều hợp lệ
        (Validated::Valid(a), Validated::Valid(b), Validated::Valid(c)) =>
            Validated::Valid(f(a, b, c)),
            
        // Trường hợp Có Lỗi: Đi soi từng thằng, thằng nào Lỗi thì hốt vào Mảng
        (a, b, c) => {
            let mut errors = vec![];
            if let Validated::Invalid(e) = a { errors.extend(e); }
            if let Validated::Invalid(e) = b { errors.extend(e); }
            if let Validated::Invalid(e) = c { errors.extend(e); }
            Validated::Invalid(errors)
        }
    }
}
```

Bây giờ ta viết lại các Hàm Validate để chúng trả về `Validated` thay vì `Result`:

```rust
fn v_name(name: &str) -> Validated<String> {
    if name.trim().len() >= 2 { Validated::Valid(name.trim().to_string()) }
    else { Validated::Invalid(vec!["Name must be at least 2 chars".into()]) }
}

fn v_email(email: &str) -> Validated<String> {
    let mut errors = vec![];
    if !email.contains('@') { errors.push("Email must contain @".into()); }
    if email.len() < 5 { errors.push("Email too short".into()); }
    
    if errors.is_empty() { Validated::Valid(email.to_lowercase()) }
    else { Validated::Invalid(errors) }
}

fn v_age(age_str: &str) -> Validated<u32> {
    match age_str.parse::<u32>() {
        Err(_) => Validated::Invalid(vec![format!("'{}' is not a number", age_str)]),
        Ok(age) if age < 18 => Validated::Invalid(vec![format!("Age {} under 18", age)]),
        Ok(age) => Validated::Valid(age),
    }
}
```

Và giờ là Phép Màu:

```rust
#[derive(Debug)]
struct ValidUser { name: String, email: String, age: u32 }

fn main() {
    // Khách hàng nhập Sai Bét mọi thứ
    let user = combine3(
        v_name("M"),           // ❌ Lỗi 1
        v_email("bad"),        // ❌ Lỗi 2, 3
        v_age("abc"),          // ❌ Lỗi 4
        |name, email, age| ValidUser { name, email, age },
    );
    
    println!("Kết quả: {:#?}", user);
    // In ra: Invalid([ "Name must be at least 2 chars", "Email must contain @", "Email too short", "'abc' is not a number" ])
}
```

Tuyệt vời! Người dùng giờ đây sẽ thấy Toàn Bộ Lỗi ngay trong 1 lần submit. 

> **💡 Quy tắc ngầm**: 
> - Dùng **`Validated`** cho Form Validation, lấy Input từ API (để báo nhiều lỗi một lần).
> - Dùng **Toán tử `?`** cho các Quy trình Workflow (Bởi vì nếu Bấm Thẻ không được thì Dừng luôn, khỏi gói Hàng làm chi cho mệt).

---

## 24.4 — Domain Errors có Cấu Trúc (Structured Errors)

Đến nay ta toàn dùng chữ `String` để báo lỗi. Rất tiện, nhưng Tồi Tệ khi Hệ thống lớn lên. 
Bạn không thể dùng lệnh `if err == "Lỗi thiếu tiền"` được (vì nhỡ ai đó đổi chữ L viết thường thành l viết hoa thì sao?).

Sử dụng `Enum` để báo lỗi giúp code vừa dễ đọc, lại vừa dễ ép máy tính xử lý logic rẽ nhánh.

### Hệ thống Cây Gia Phả Lỗi (Error Hierarchy)

```rust
// filename: src/main.rs
use std::fmt;

// 1. Các lỗi Cấp Thấp (Validation)
#[derive(Debug, Clone)]
enum ValidationError {
    FieldRequired(String),
    FieldTooShort { field: String, min: usize, actual: usize },
    OutOfRange { field: String, min: String, max: String, actual: String },
}

// 2. Các lỗi Cấp Cao (Domain / Business)
#[derive(Debug)]
enum DomainError {
    // Bao bọc các lỗi Cấp thấp vào bên trong
    Validation(Vec<ValidationError>),
    BusinessRule(String),
    NotFound { entity: String, id: String },
}
```

Chỉ cần Viết `Display` cho chúng để in ra text cực đẹp:

```rust
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::FieldRequired(field) => write!(f, "'{}' is required", field),
            ValidationError::FieldTooShort { field, min, actual } => 
                write!(f, "'{}' too short: {} chars (min {})", field, actual, min),
            ValidationError::OutOfRange { field, min, max, actual } => 
                write!(f, "'{}' out of range: {} (must be {}-{})", field, actual, min, max),
        }
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::Validation(errors) => {
                writeln!(f, "Validation failed:")?;
                for e in errors { writeln!(f, "  - {}", e)?; }
                Ok(())
            }
            DomainError::BusinessRule(msg) => write!(f, "Business rule: {}", msg),
            DomainError::NotFound { entity, id } => write!(f, "{} '{}' not found", entity, id),
        }
    }
}
```

Bây giờ các hàm validate sẽ trả về Enum thay vì String rác:

```rust
fn validate_price(price: i64) -> Result<u32, ValidationError> {
    if price <= 0 || price > 1_000_000_000 {
        Err(ValidationError::OutOfRange {
            field: "price".into(),
            min: "1".into(), max: "1,000,000,000".into(), actual: price.to_string(),
        })
    } else {
        Ok(price as u32)
    }
}
```

---

## 24.5 — Trạm trung chuyển lỗi: `map_err` (Error Enrichment)

Khi Lỗi chui từ Cấp thấp (Validation) nảy lên Cấp cao (API), nó cần được "Gắn thêm mác" để dễ nhận biết. Hàm `map_err` làm đúng việc đó: Gắn nhãn vào Lỗi trước khi ném nó cho tầng trên.

```rust
// filename: src/main.rs

#[derive(Debug)]
enum AppError { Validation(String), Database(String) }

fn parse_id(input: &str) -> Result<u64, AppError> {
    input.parse::<u64>()
        // Ép lỗi của hệ thống Parse thành AppError::Validation
        .map_err(|e| AppError::Validation(format!("Invalid ID '{}': {}", input, e)))
}

fn find_user(id: u64) -> Result<String, AppError> {
    if id == 42 { Ok("Minh".into()) }
    else { Err(AppError::Database(format!("User {} not found", id))) }
}

fn main() {
    // Pipeline rẽ Lỗi cực mượt
    let result = parse_id("abc").and_then(find_user);
    
    match result {
        Ok(name) => println!("Hello {}", name),
        Err(AppError::Validation(msg)) => println!("📝 Mời nhập lại cho đúng: {}", msg),
        Err(AppError::Database(msg)) => println!("💾 Báo IT kiểm tra DB: {}", msg),
    }
}
```

---

## 24.6 — Thực chiến: Hoàn thiện Form Đăng Ký Tài Khoản

Đây là một khuôn mẫu Hoàn Chỉnh để đưa lên Production. 
- **Giai đoạn 1**: Kiểm tra đầu vào (Dùng `Validated` để hốt MỌI LỖI gõ sai).
- **Giai đoạn 2**: Kiểm tra Nghiệp vụ hệ thống (Dùng Fail-fast). Nếu Giai đoạn 1 sai, từ chối chạy Giai đoạn 2 để tiết kiệm tài nguyên.

```rust
// filename: src/main.rs

// 1. Types
struct RegistrationForm { name: String, email: String }
struct RegisteredUser { id: u64, name: String, email: String }

// 2. Định nghĩa Validated gọn nhẹ
enum V<T> { Ok(T), Errs(Vec<String>) }

fn combine2<A,B,R>(a: V<A>, b: V<B>, f: impl FnOnce(A,B)->R) -> V<R> {
    let mut errs = vec![];
    let val_a = match a { V::Ok(v) => Some(v), V::Errs(e) => { errs.extend(e); None } };
    let val_b = match b { V::Ok(v) => Some(v), V::Errs(e) => { errs.extend(e); None } };
    
    if errs.is_empty() { V::Ok(f(val_a.unwrap(), val_b.unwrap())) }
    else { V::Errs(errs) }
}

// 3. Validators
fn v_name(name: &str) -> V<String> {
    if name.len() < 2 { V::Errs(vec!["Name: Tối thiểu 2 ký tự".into()]) } else { V::Ok(name.into()) }
}
fn v_email(email: &str) -> V<String> {
    if !email.contains('@') { V::Errs(vec!["Email: Phải có chữ @".into()]) } else { V::Ok(email.into()) }
}

// 4. Hàm Chốt (Đóng gói 2 Giai đoạn)
fn register(form: RegistrationForm) -> Result<RegisteredUser, Vec<String>> {
    
    // Giai đoạn 1: Gom mọi lỗi chính tả (Validate)
    let validated = combine2(v_name(&form.name), v_email(&form.email), |n, e| (n, e));

    match validated {
        V::Errs(errors) => Err(errors), // Trả 1 rổ lỗi cho Frontend
        V::Ok((name, email)) => {
            // Giai đoạn 2: Lỗi nghiệp vụ (Business Rules - Fail Fast)
            if email == "admin@co.com" { 
                return Err(vec!["Không được phép đăng ký email này".into()]); 
            }
            Ok(RegisteredUser { id: 99, name, email })
        }
    }
}
```

Thực thi:

```rust
fn main() {
    let form = RegistrationForm { name: "M".into(), email: "bad".into() };
    
    match register(form) {
        Ok(user) => println!("✅ {}", user.name),
        Err(errors) => {
            println!("❌ Đăng ký thất bại ({} lỗi):", errors.len());
            for e in errors { println!("  • {}", e); }
        }
    }
    // In ra: Đăng ký thất bại (2 lỗi): Tối thiểu 2 ký tự & Phải có chữ @
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Map và And_then chạy song song

```rust
Ok("42").and_then(|s| s.parse::<i32>().map_err(|e| e.to_string()))
        .map(|n| n * 2)
        .and_then(|n| if n > 100 { Err("too big".into()) } else { Ok(n) })
        .map_err(|e: String| format!("Error: {}", e))
```
Đáp án của kết quả trên là gì?

<details><summary>✅ Lời giải</summary>
`"42"` biến thành `Ok(42)` nhờ parse.
Map x2 = `Ok(84)`.
Kiểm tra 84 > 100? Sai, đi tiếp = `Ok(84)`.
Hàm map_err cuối không bắt được lỗi nên bỏ qua.
Kết quả cuối cùng: `Ok(84)`.
</details>

---

**Bài 2** (15 phút): Địa chỉ an toàn (Validated Address)

Áp dụng `combine3` để tạo một Trạm kiểm duyệt Địa chỉ:
- `street`: Không rỗng, max 200 ký tự.
- `city`: Từ 2-100 ký tự.
- `zip`: Chính xác 5 chữ số.
Viết 3 hàm kiểm duyệt và Gộp nó lại.

<details><summary>✅ Gợi ý Lời giải</summary>

Dùng `V::Errs()` để báo lỗi cho mỗi hàm. Viết hàm `v_zip(z: &str)` dùng lệnh `z.chars().all(|c| c.is_ascii_digit())` để kiểm tra độ dài và tính số. Cuối cùng nhét 3 thằng vào `combine3`.
</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| `?` chỉ bắt được 1 lỗi, ghét quá | Tính chất Fail-fast của nó là vậy | Tự viết `Validated<T>` và hàm `combine` như trên để hốt rác. |
| Error types (Chữ E) không khớp nhau trong đường ống pipeline | `Result<T, E1>` cắm nhầm `Result<T, E2>` | Dùng `map_err(|e| E2(e))` để Đóng gói Lỗi cũ vào trong Cái Hộp Lỗi Mới. |
| Viết `combine3`, `combine4` tốn dòng quá | Đúng vậy, nó khá phiền toái. | Các thư viện ngoài như `frunk` (Haskell-like) sẽ cấp sẵn macro cho bạn, hoặc bạn tự viết 1 Macro `validate!()`. |

---

---

## ✅ Checkpoint 24

1. `?` tương ứng với phép nào trong FP?
2. `?` tự chuyển kiểu lỗi nhờ trait nào?
3. Vì sao `Validated` cần tồn tại bên cạnh `Result`?

<details>
<summary>Đáp án</summary>

1. **Monadic bind** (`and_then`). `?` chỉ là đường cú pháp cho "nếu `Err` thì trả về sớm, nếu `Ok` thì lấy giá trị ra và đi tiếp".
2. `From`. `?` gọi ngầm `From::from` trên giá trị lỗi, nên kiểu lỗi của hàm gọi chỉ cần `impl From<LỗiCủaHàmCon>`. Đây là chỗ `thiserror` với `#[from]` tiết kiệm rất nhiều boilerplate.
3. Vì `Result` **fail-fast** — nó dừng ở lỗi đầu tiên, do bản chất của bind là bước sau phụ thuộc bước trước. Form nhiều trường cần thu hết lỗi, mà điều đó chỉ làm được khi các nhánh **độc lập** — tức là applicative, không phải monad.
</details>

## Tóm tắt

- ✅ **Mô hình Hai Đường Ray**: `Result<T,E>` cho phép Tàu chạy mượt mà ở làn Ok, hoặc Trật Bánh và trượt dài ở làn Err.
- ✅ **Công tắc bẻ ghi**: `.map()` chế biến đồ tốt. `.and_then()` nối đường ray rủi ro. `.map_err()` bọc lại rác.
- ✅ **Fail Fast vs Collect All**: Dấu `?` chỉ bắt 1 lỗi và phanh gấp (Dành cho Quy trình nối tiếp). `Validated<T>` thì chạy song song và hứng TẤT CẢ lỗi (Dành cho Form Validation).
- ✅ **Enum Error Cấu Trúc**: Từ bỏ thói quen báo lỗi bằng `String`. Dùng Enum để Máy tính có thể Đọc Hiểu và Xử lý.

## Tiếp theo

Bạn đã có Types đẹp, Pipeline mượt, và Bắt Lỗi cực nét. Nhưng để Data có thể giao tiếp với Front-end (React/Vue) hoặc App Mobile, nó phải biến thành dạng JSON.

→ Chapter 25: **Serialization & Anti-Corruption Layer** — Học cách dùng thần khí `serde`, cách phân tách Domain Types khỏi DTOs (Data Transfer Objects), và xây dựng Ranh Giới Chống Suy Đồi (Anti-corruption layer).
