# Chapter 24 — Railway-Oriented Programming in Rust ⭐

> **Bạn sẽ học được**:
> - **Two-track model**: `Result<T, E>` = Success track + Failure track
> - `map`, `and_then`, `map_err` — switch functions giữa tracks
> - **Collect ALL errors** — `Validated<T>` applicative pattern
> - Compose validations → applicative style
> - Error type hierarchy — domain errors có cấu trúc
> - Real-world: complete validation pipeline
>
> **Yêu cầu trước**: Chapter 10 (Error Handling), Chapter 23 (Workflows).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Error handling trở thành **first-class design tool** — không chỉ catch errors, mà **compose** chúng.

---

## 24.1 — Two-Track Model

Chapter 23 dùng `and_then` để nối workflow steps. Đây là mô hình trực giác của nó: mỗi function giống một đoạn đường ray có **2 tracks** — track trên (Ok) và track dưới (Err). Khi một function fail, dữ liệu chuyển xuống track Err và **trượt thẳng** qua mọi step còn lại. Đây là "Railway-Oriented Programming" — tên gọi trực giác cho error handling bằng `Result`.

### `Result<T, E>` = Đường ray đôi

Hình dung mỗi function như một **đoạn đường ray**. Có 2 tracks:

```
Success track (Ok):  ═══╦═══╦═══╦═══╦═══→ ✅ Result
                        ║   ║   ║   ║
Failure track (Err): ───╨───╨───╨───╨───→ ❌ Error

Mỗi function = 1 đoạn rail.
Nếu function thành công → ở trên Success track.
Nếu function fail → chuyển xuống Failure track.
Khi đã ở Failure track → SKIP tất cả steps còn lại.
```

### Switch functions

```rust
// filename: src/main.rs

// 3 loại "switch functions":

// Type 1: One-track → One-track (pure transform)
fn double(x: i32) -> i32 { x * 2 }
// Dùng với: .map()

// Type 2: One-track → Two-track (có thể fail)
fn validate_positive(x: i32) -> Result<i32, String> {
    if x > 0 { Ok(x) } else { Err(format!("{} is not positive", x)) }
}
// Dùng với: .and_then()

// Type 3: Failure-track → Failure-track (transform error)
fn add_context(err: String) -> String {
    format!("[Validation] {}", err)
}
// Dùng với: .map_err()

fn main() {
    // Happy path: mọi step thành công
    let ok = Ok(5)
        .map(double)                       // 10
        .and_then(validate_positive)       // Ok(10)
        .map(double)                       // 20
        .map_err(add_context);
    println!("Happy: {:?}", ok);  // Ok(20)

    // Sad path: fail ở validate → skip phần sau
    let err = Ok(-3)
        .map(double)                       // -6
        .and_then(validate_positive)       // Err!
        .map(double)                       // SKIPPED
        .map_err(add_context);
    println!("Sad: {:?}", err);   // Err("[Validation] -6 is not positive")
}
```

### Bảng cheat sheet

| Method | Input | Output | Khi nào dùng |
|--------|-------|--------|-------------|
| `.map(f)` | `Ok(T)` → `f(T) → U` | `Ok(U)` | Transform giá trị, không fail |
| `.and_then(f)` | `Ok(T)` → `f(T) → Result<U,E>` | `Ok(U)` hoặc `Err(E)` | Step có thể fail |
| `.map_err(f)` | `Err(E)` → `f(E) → F` | `Err(F)` | Transform/enrich error |
| `.or_else(f)` | `Err(E)` → `f(E) → Result<T,F>` | Recover từ error | Fallback/retry |
| `?` | `Err(E)` → return sớm | | Ngắn gọn cho `and_then` |

---

## 24.2 — `?` Operator = Monadic Bind

Bạn có thể viết `and_then` chains dài, nhưng Rust có cách gọn hơn: `?`. Dấu `?` sau expression hoạt động như `and_then` + return sớm — nếu Ok thì lấy giá trị ra, nếu Err thì return ngay từ function.

Nhưng có một hạn chế quan trọng:

```rust
// filename: src/main.rs

#[derive(Debug)]
struct UserInput {
    name: String,
    email: String,
    age: String,
}

#[derive(Debug)]
struct ValidUser {
    name: String,
    email: String,
    age: u32,
}

// Dùng ? — pipeline tự động dừng ở lỗi đầu tiên
fn validate_user(input: &UserInput) -> Result<ValidUser, String> {
    let name = validate_name(&input.name)?;    // fail? → return Err
    let email = validate_email(&input.email)?;  // fail? → return Err
    let age = validate_age(&input.age)?;        // fail? → return Err
    Ok(ValidUser { name, email, age })
}

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
    let age: u32 = age_str.parse().map_err(|_| format!("Invalid age: {}", age_str))?;
    if (18..=150).contains(&age) { Ok(age) }
    else { Err(format!("Age {} not in range 18-150", age)) }
}

fn main() {
    let good = UserInput { name: "Minh".into(), email: "minh@co.com".into(), age: "25".into() };
    println!("Valid: {:?}", validate_user(&good));

    let bad_name = UserInput { name: "M".into(), email: "minh@co.com".into(), age: "25".into() };
    println!("Bad name: {:?}", validate_user(&bad_name));
    // Chỉ báo LỖI ĐẦU TIÊN! Email và age KHÔNG ĐƯỢC kiểm tra.
}
```

> **⚠️ Vấn đề**: `?` stops at first error. User phải sửa lỗi 1 → submit → thấy lỗi 2 → sửa → submit → lỗi 3... Rất khó chịu!

---

## 24.3 — Collect ALL Errors: `Validated<T>` ⭐

Vấn đề của `?`: người dùng điền form có 3 lỗi, nhưng chỉ thấy lỗi đầu tiên. Sửa, submit lại, thấy lỗi thứ hai. Lại sửa... Rất khó chịu. UX tốt hơn: show **tất cả** lỗi cùng lúc để người dùng sửa một lần.

Giải pháp: tạo kiểu `Validated<T>` — giống `Result` nhưng thay vì dừng ở lỗi đầu, nó **gom tất cả lỗi** rồi trả một lượt.

### Vấn đề và giải pháp

```
? operator:           validate_name? → validate_email? → validate_age?
                      Err("bad name") → STOP! ← chỉ 1 error

Validated pattern:    validate_name ──→ validate_email ──→ validate_age
                      Err("bad name")   Err("bad email")   Err("bad age")
                      ← COLLECT ALL → Vec["bad name", "bad email", "bad age"]
```

### Implement `Validated<T>`

```rust
// filename: src/main.rs

/// Validated: hoặc Valid(T), hoặc Invalid(tất cả errors)
#[derive(Debug, Clone)]
enum Validated<T> {
    Valid(T),
    Invalid(Vec<String>),
}

impl<T> Validated<T> {
    fn from_result(result: Result<T, String>) -> Self {
        match result {
            Ok(val) => Validated::Valid(val),
            Err(err) => Validated::Invalid(vec![err]),
        }
    }

    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Validated<U> {
        match self {
            Validated::Valid(val) => Validated::Valid(f(val)),
            Validated::Invalid(errs) => Validated::Invalid(errs),
        }
    }

    fn is_valid(&self) -> bool {
        matches!(self, Validated::Valid(_))
    }
}

/// Combine 2 Validated: nếu cả 2 valid → apply function, nếu bất kỳ invalid → collect ALL errors
fn combine2<A, B, C, F: FnOnce(A, B) -> C>(
    va: Validated<A>,
    vb: Validated<B>,
    f: F,
) -> Validated<C> {
    match (va, vb) {
        (Validated::Valid(a), Validated::Valid(b)) => Validated::Valid(f(a, b)),
        (Validated::Invalid(e1), Validated::Invalid(e2)) => {
            let mut all = e1;
            all.extend(e2);
            Validated::Invalid(all)
        }
        (Validated::Invalid(e), _) | (_, Validated::Invalid(e)) => Validated::Invalid(e),
    }
}

/// Combine 3 Validated
fn combine3<A, B, C, D, F: FnOnce(A, B, C) -> D>(
    va: Validated<A>,
    vb: Validated<B>,
    vc: Validated<C>,
    f: F,
) -> Validated<D> {
    match (va, vb, vc) {
        (Validated::Valid(a), Validated::Valid(b), Validated::Valid(c)) =>
            Validated::Valid(f(a, b, c)),
        (a, b, c) => {
            let mut errors = vec![];
            if let Validated::Invalid(e) = a { errors.extend(e); }
            if let Validated::Invalid(e) = b { errors.extend(e); }
            if let Validated::Invalid(e) = c { errors.extend(e); }
            Validated::Invalid(errors)
        }
    }
}

// ═══ Domain validations ═══
fn validate_name(name: &str) -> Validated<String> {
    if name.trim().len() >= 2 {
        Validated::Valid(name.trim().to_string())
    } else {
        Validated::Invalid(vec!["Name must be at least 2 characters".into()])
    }
}

fn validate_email(email: &str) -> Validated<String> {
    let mut errors = vec![];
    if !email.contains('@') { errors.push("Email must contain @".into()); }
    if email.len() < 5 { errors.push("Email too short".into()); }
    if errors.is_empty() { Validated::Valid(email.to_lowercase()) }
    else { Validated::Invalid(errors) }
}

fn validate_age(age_str: &str) -> Validated<u32> {
    match age_str.parse::<u32>() {
        Err(_) => Validated::Invalid(vec![format!("'{}' is not a number", age_str)]),
        Ok(age) if age < 18 => Validated::Invalid(vec![format!("Age {} under 18", age)]),
        Ok(age) if age > 150 => Validated::Invalid(vec![format!("Age {} over 150", age)]),
        Ok(age) => Validated::Valid(age),
    }
}

#[derive(Debug)]
struct ValidUser { name: String, email: String, age: u32 }

fn main() {
    // Case 1: All valid
    let user = combine3(
        validate_name("Minh"),
        validate_email("minh@co.com"),
        validate_age("25"),
        |name, email, age| ValidUser { name, email, age },
    );
    println!("All valid: {:?}\n", user);

    // Case 2: MULTIPLE errors → ALL collected! ⭐
    let user = combine3(
        validate_name("M"),           // ❌ too short
        validate_email("bad"),         // ❌ no @, too short
        validate_age("abc"),           // ❌ not a number
        |name, email, age| ValidUser { name, email, age },
    );
    println!("All invalid: {:?}\n", user);
    // Invalid(["Name must be at least 2 characters", "Email must contain @", "Email too short", "'abc' is not a number"])

    // Case 3: Some valid, some invalid
    let user = combine3(
        validate_name("Minh"),         // ✅
        validate_email("bad"),         // ❌
        validate_age("25"),            // ✅
        |name, email, age| ValidUser { name, email, age },
    );
    println!("Mixed: {:?}", user);
    // Invalid(["Email must contain @", "Email too short"])
}
```

---

## ✅ Checkpoint 24.3

> Ghi nhớ:
> 1. **`?` / `and_then`** = fail fast (1 error). **`Validated`** = collect all errors.
> 2. `combine3(va, vb, vc, f)` = nếu tất cả Valid → apply f, nếu bất kỳ Invalid → gom errors
> 3. Dùng **Validated cho form validation, API input** — user thấy MỌI lỗi cùng lúc
> 4. Dùng **`?` cho workflow steps** — step 2 phụ thuộc step 1, fail fast hợp lý

---

## 24.4 — Structured Domain Errors

Dùng `String` cho errors đơn giản nhưng không scale: bạn không thể match pattern trên string để xử lý từng loại lỗi khác nhau. Error enum cho phép code vừa đọc được (với `Display`), vừa xử lý được bằng máy (với `match`).

### Error type hierarchy

```rust
// filename: src/main.rs
use std::fmt;

// ═══ Error types có cấu trúc — không chỉ String! ═══

#[derive(Debug, Clone)]
enum ValidationError {
    FieldRequired(String),
    FieldTooShort { field: String, min: usize, actual: usize },
    FieldTooLong { field: String, max: usize, actual: usize },
    InvalidFormat { field: String, expected: String },
    OutOfRange { field: String, min: String, max: String, actual: String },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::FieldRequired(field) =>
                write!(f, "'{}' is required", field),
            ValidationError::FieldTooShort { field, min, actual } =>
                write!(f, "'{}' too short: {} chars (min {})", field, actual, min),
            ValidationError::FieldTooLong { field, max, actual } =>
                write!(f, "'{}' too long: {} chars (max {})", field, actual, max),
            ValidationError::InvalidFormat { field, expected } =>
                write!(f, "'{}' invalid format: expected {}", field, expected),
            ValidationError::OutOfRange { field, min, max, actual } =>
                write!(f, "'{}' out of range: {} (must be {}-{})", field, actual, min, max),
        }
    }
}

#[derive(Debug)]
enum DomainError {
    Validation(Vec<ValidationError>),
    BusinessRule(String),
    NotFound { entity: String, id: String },
    Conflict(String),
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
            DomainError::Conflict(msg) => write!(f, "Conflict: {}", msg),
        }
    }
}

// ═══ Validations returning structured errors ═══

fn validate_product_name(name: &str) -> Result<String, ValidationError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        Err(ValidationError::FieldRequired("product_name".into()))
    } else if trimmed.len() < 3 {
        Err(ValidationError::FieldTooShort { field: "product_name".into(), min: 3, actual: trimmed.len() })
    } else if trimmed.len() > 100 {
        Err(ValidationError::FieldTooLong { field: "product_name".into(), max: 100, actual: trimmed.len() })
    } else {
        Ok(trimmed.to_string())
    }
}

fn validate_price(price: i64) -> Result<u32, ValidationError> {
    if price <= 0 || price > 1_000_000_000 {
        Err(ValidationError::OutOfRange {
            field: "price".into(),
            min: "1".into(),
            max: "1,000,000,000".into(),
            actual: price.to_string(),
        })
    } else {
        Ok(price as u32)
    }
}

// Collect all validation errors
fn validate_product(name: &str, price: i64) -> Result<(String, u32), DomainError> {
    let mut errors = vec![];

    let name_result = validate_product_name(name);
    let price_result = validate_price(price);

    if let Err(e) = &name_result { errors.push(e.clone()); }
    if let Err(e) = &price_result { errors.push(e.clone()); }

    if errors.is_empty() {
        Ok((name_result.unwrap(), price_result.unwrap()))
    } else {
        Err(DomainError::Validation(errors))
    }
}



fn main() {
    // Happy path
    match validate_product("Coffee Premium", 185_000) {
        Ok((name, price)) => println!("✅ {} — {}đ", name, price),
        Err(e) => println!("❌ {}", e),
    }

    // Multiple errors
    match validate_product("AB", -500) {
        Ok(_) => println!("✅ ok"),
        Err(e) => println!("❌ {}", e),
    }
}
```

---

## 24.5 — Composing with `map_err`: Error Enrichment

Khi errors đi qua nhiều layers (validation → business logic → database), mỗi layer cần thêm context. `map_err` biến đổi error type khi nó đi lên trên — giống việc gắn nhãn vào gói hàng ở mỗi trạm kiểm soát.

```rust
// filename: src/main.rs

#[derive(Debug)]
enum AppError {
    Validation(String),
    Database(String),
    External(String),
}

fn parse_id(input: &str) -> Result<u64, AppError> {
    input.parse::<u64>()
        .map_err(|e| AppError::Validation(format!("Invalid ID '{}': {}", input, e)))
}

fn find_user(id: u64) -> Result<String, AppError> {
    if id == 42 { Ok("Minh".into()) }
    else { Err(AppError::Database(format!("User {} not found", id))) }
}

fn send_notification(user: &str) -> Result<String, AppError> {
    Ok(format!("Notified {}", user))
}

// Pipeline with error context
fn process(input: &str) -> Result<String, AppError> {
    parse_id(input)
        .and_then(find_user)
        .and_then(|user| send_notification(&user))
}

fn handle_error(err: &AppError) -> String {
    match err {
        AppError::Validation(msg) => format!("📝 Fix your input: {}", msg),
        AppError::Database(msg) => format!("💾 Database issue: {}", msg),
        AppError::External(msg) => format!("🌐 Service error: {}", msg),
    }
}

fn main() {
    for input in &["42", "99", "abc"] {
        match process(input) {
            Ok(result) => println!("✅ {}", result),
            Err(e) => println!("❌ {}", handle_error(&e)),
        }
    }
}
```

---

## 24.6 — Real-world: Complete Registration Pipeline

Đây là pattern hoàn chỉnh bạn sẽ dùng trong production: Phase 1 validate tất cả fields (collect all errors với `Validated`), Phase 2 check business rules (fail-fast với `?`). Nếu Phase 1 có lỗi, không cần chạy Phase 2.

```rust
// filename: src/main.rs

// ═══ Full ROP pipeline: registration form ═══

#[derive(Debug)]
struct RegistrationForm {
    name: String,
    email: String,
    password: String,
    age: String,
}

#[derive(Debug)]
struct RegisteredUser {
    id: u64,
    name: String,
    email: String,
    age: u32,
}

// Validated type
#[derive(Debug)]
enum V<T> { Ok(T), Errs(Vec<String>) }

impl<T> V<T> {
    fn from(r: Result<T, String>) -> Self {
        match r { Ok(v) => V::Ok(v), Err(e) => V::Errs(vec![e]) }
    }
}

fn combine4<A,B,C,D,R>(a: V<A>, b: V<B>, c: V<C>, d: V<D>, f: impl FnOnce(A,B,C,D)->R) -> V<R> {
    let mut errs = vec![];
    let a = match a { V::Ok(v) => Some(v), V::Errs(e) => { errs.extend(e); None } };
    let b = match b { V::Ok(v) => Some(v), V::Errs(e) => { errs.extend(e); None } };
    let c = match c { V::Ok(v) => Some(v), V::Errs(e) => { errs.extend(e); None } };
    let d = match d { V::Ok(v) => Some(v), V::Errs(e) => { errs.extend(e); None } };
    if errs.is_empty() { V::Ok(f(a.unwrap(), b.unwrap(), c.unwrap(), d.unwrap())) }
    else { V::Errs(errs) }
}

fn v_name(name: &str) -> V<String> {
    let t = name.trim();
    if t.len() < 2 { V::Errs(vec!["Name: at least 2 chars".into()]) }
    else { V::Ok(t.into()) }
}

fn v_email(email: &str) -> V<String> {
    let mut e = vec![];
    if !email.contains('@') { e.push("Email: must contain @".into()); }
    if email.len() < 5 { e.push("Email: too short".into()); }
    if e.is_empty() { V::Ok(email.to_lowercase()) } else { V::Errs(e) }
}

fn v_password(pw: &str) -> V<String> {
    let mut e = vec![];
    if pw.len() < 8 { e.push("Password: min 8 chars".into()); }
    if !pw.chars().any(|c| c.is_uppercase()) { e.push("Password: need uppercase".into()); }
    if !pw.chars().any(|c| c.is_ascii_digit()) { e.push("Password: need digit".into()); }
    if e.is_empty() { V::Ok(pw.into()) } else { V::Errs(e) }
}

fn v_age(age: &str) -> V<u32> {
    match age.parse::<u32>() {
        Err(_) => V::Errs(vec![format!("Age: '{}' is not a number", age)]),
        Ok(a) if a < 18 => V::Errs(vec![format!("Age: {} under 18", a)]),
        Ok(a) if a > 150 => V::Errs(vec![format!("Age: {} over 150", a)]),
        Ok(a) => V::Ok(a),
    }
}

fn register(form: RegistrationForm) -> Result<RegisteredUser, Vec<String>> {
    // Step 1: Validate ALL fields (collect errors)
    let validated = combine4(
        v_name(&form.name),
        v_email(&form.email),
        v_password(&form.password),
        v_age(&form.age),
        |name, email, _pw, age| (name, email, age),
    );

    match validated {
        V::Errs(errors) => Err(errors),
        V::Ok((name, email, age)) => {
            // Step 2: Business rules (fail-fast OK here)
            // e.g., check duplicate email (would hit DB)
            Ok(RegisteredUser { id: 1, name, email, age })
        }
    }
}

fn main() {
    // All invalid → see ALL errors at once!
    let form = RegistrationForm {
        name: "M".into(),
        email: "bad".into(),
        password: "weak".into(),
        age: "xyz".into(),
    };

    match register(form) {
        Ok(user) => println!("✅ {:?}", user),
        Err(errors) => {
            println!("❌ Registration failed ({} errors):", errors.len());
            for e in &errors { println!("  • {}", e); }
        }
    }

    println!();

    // All valid
    let form = RegistrationForm {
        name: "Minh Nguyen".into(),
        email: "minh@company.com".into(),
        password: "Str0ngP@ss".into(),
        age: "28".into(),
    };

    match register(form) {
        Ok(user) => println!("✅ Welcome, {}! (ID: {})", user.name, user.id),
        Err(errors) => {
            println!("❌ ({} errors):", errors.len());
            for e in &errors { println!("  • {}", e); }
        }
    }
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): map/and_then tracing

```rust
Ok("42").and_then(|s| s.parse::<i32>().map_err(|e| e.to_string()))
        .map(|n| n * 2)
        .and_then(|n| if n > 100 { Err("too big".into()) } else { Ok(n) })
        .map_err(|e: String| format!("Error: {}", e))
```
Result = ?

<details><summary>✅ Lời giải</summary>

```
"42" → parse → Ok(42) → map ×2 → Ok(84) → 84 > 100? No → Ok(84)
Result = Ok(84)
```

</details>

---

**Bài 2** (10 phút): Validated address

Dùng collect-all-errors pattern, validate `Address`:
- `street`: non-empty, max 200 chars
- `city`: non-empty, 2-100 chars
- `zip`: exactly 5 digits
- Compose bằng `combine3`

<details><summary>✅ Lời giải Bài 2</summary>

```rust
#[derive(Debug)]
struct Address { street: String, city: String, zip: String }

fn v_street(s: &str) -> Validated<String> {
    let t = s.trim();
    if t.is_empty() { Validated::Invalid(vec!["Street required".into()]) }
    else if t.len() > 200 { Validated::Invalid(vec!["Street max 200 chars".into()]) }
    else { Validated::Valid(t.into()) }
}

fn v_city(c: &str) -> Validated<String> {
    let t = c.trim();
    if t.len() < 2 || t.len() > 100 {
        Validated::Invalid(vec![format!("City: 2-100 chars, got {}", t.len())])
    } else { Validated::Valid(t.into()) }
}

fn v_zip(z: &str) -> Validated<String> {
    if z.len() == 5 && z.chars().all(|c| c.is_ascii_digit()) {
        Validated::Valid(z.into())
    } else { Validated::Invalid(vec![format!("Zip must be 5 digits, got '{}'", z)]) }
}

fn validate_address(street: &str, city: &str, zip: &str) -> Validated<Address> {
    combine3(v_street(street), v_city(city), v_zip(zip),
        |street, city, zip| Address { street, city, zip })
}
```

</details>

---

**Bài 3** (15 phút): Complete form pipeline

Combine ROP: validate `User` (collect errors) → check business rules (fail-fast) → create account.
- User: name, email, age, address (nested validation)
- Business: email unique (check against list), age ≥ 18
- Output: `RegisteredUser` hoặc all errors

<details><summary>✅ Lời giải Bài 3</summary>

```rust
fn register_full(
    name: &str, email: &str, age: &str,
    street: &str, city: &str, zip: &str,
    existing_emails: &[&str],
) -> Result<String, Vec<String>> {
    // Phase 1: Validate (collect ALL errors)
    let user = combine3(v_name(name), v_email(email), v_age(age), |n, e, a| (n, e, a));
    let addr = combine3(v_street(street), v_city(city), v_zip(zip), |s, c, z| (s, c, z));

    let combined = combine2(user, addr, |u, a| (u, a));
    let ((name, email, age), (street, city, zip)) = match combined {
        Validated::Valid(v) => v,
        Validated::Invalid(errors) => return Err(errors),
    };

    // Phase 2: Business rules (fail-fast OK)
    if existing_emails.contains(&email.as_str()) {
        return Err(vec![format!("Email {} already registered", email)]);
    }

    Ok(format!("Welcome {}! ({}, {}, {})", name, email, age, city))
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| `?` chỉ bắt 1 error | Fail-fast by design | Dùng `Validated` cho multi-error |
| Error types không match trong chain | `Result<T, E1>` vs `Result<T, E2>` | `.map_err()` hoặc `From` impl |
| "Validated quá verbose" | Boilerplate `combine3`, `combine4` | Macro `validate!` hoặc crate `either`/`frunk` |
| Nested validation phức tạp | Address inside User | Validate từng phần, `combine` các phần |

---

## Tóm tắt

- ✅ **Two-track model**: `Result<T,E>` = Success ∥ Failure. `map` transform Ok, `and_then` chain Result→Result.
- ✅ **`?` = fail fast**: Dừng ở lỗi đầu tiên. Tốt cho workflow steps phụ thuộc nhau.
- ✅ **`Validated<T>` = collect all**: `combine3(va, vb, vc, f)` gom TẤT CẢ errors. Tốt cho form validation.
- ✅ **Structured errors**: `ValidationError` enum + `DomainError` hierarchy — không chỉ `String`.
- ✅ **`map_err`**: Enrich errors với context. Route errors to correct handler.
- ✅ **Pattern**: Phase 1 = validate (collect all) → Phase 2 = business rules (fail fast).

## Tiếp theo

→ Chapter 25: **Serialization & Anti-Corruption Layer** — bạn sẽ học `serde` derive, Domain types ↔ DTOs, `From`/`Into` mapping, anti-corruption layer pattern, và JSON/TOML/MessagePack serialization.
