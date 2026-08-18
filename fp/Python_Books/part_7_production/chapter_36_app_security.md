# Chapter 36 — Application Security & Hardening (OWASP Top 10)

> **Bạn sẽ học được**:
> - Bản chất của **CORS** và tại sao nó KHÔNG PHẢI là một biện pháp bảo mật server.
> - Cách **SQL Injection** hoạt động và cơ chế phòng thủ bằng Parameterized Queries.
> - Phòng chống XSS (Cross-Site Scripting) và vai trò của CSP (Content Security Policy).
> - Cấu hình các Header bảo mật bắt buộc phải có khi đưa ứng dụng lên Production.
>
> **Yêu cầu trước**: Chapter 35 (Security Essentials)
> **Thời gian đọc**: ~30 phút | **Level**: Principal
> **Kết quả cuối cùng**: Biết cách thiết lập hàng rào bảo mật vững chắc cho Web API, tự tin đối phó với các kịch bản tấn công phổ biến.

---

Bảo mật ứng dụng (AppSec) thường xoay quanh danh sách **OWASP Top 10** — 10 lỗ hổng bảo mật phổ biến nhất. Ở chương này, chúng ta sẽ đi sâu vào 3 lỗ hổng kinh điển nhất: Injection, XSS, và lỗi cấu hình (Misconfiguration).

## 36.1 — SQL Injection (Lỗ hổng Tiêm mã)

Đây là lỗi phổ biến nhất và gây hậu quả thảm khốc nhất (bị rò rỉ hoặc xóa toàn bộ Database). Nó xảy ra khi bạn **nối chuỗi** (string concatenation) input của người dùng trực tiếp vào câu lệnh SQL.

```python
# ❌ THẢM HỌA: Nối chuỗi trực tiếp
def bad_query(user_input: str):
    # Nếu user_input = "'; DROP TABLE users; --"
    # Câu SQL sẽ biến thành: SELECT * FROM users WHERE name = ''; DROP TABLE users; --'
    query = f"SELECT * FROM users WHERE name = '{user_input}'"
```

**Cách phòng thủ duy nhất đúng: Parameterized Queries (Truy vấn có tham số).**
Khi dùng tính năng này, Thư viện kết nối DB (Driver) sẽ gửi *câu lệnh SQL* và *Dữ liệu* qua 2 kênh riêng biệt tới Database Engine. Database Engine sẽ biên dịch câu SQL trước, sau đó mới nhét dữ liệu vào như một giá trị thuần túy (Dù input chứa chữ `DROP TABLE`, nó cũng chỉ được hiểu là chuỗi ký tự, không phải mã lệnh).

```python
# ✅ LUÔN LUÔN DÙNG BIND PARAMETERS
from sqlalchemy import text

def safe_query(engine, user_input: str):
    with engine.connect() as conn:
        # 1. Gửi cấu trúc câu lệnh đi trước (chứa placeholder :name)
        # 2. Gửi dictionary chứa data đi kèm theo
        result = conn.execute(
            text("SELECT * FROM users WHERE name = :name"),
            {"name": user_input}  # An toàn tuyệt đối!
        )
        return result.fetchall()
```

---

## 36.2 — CORS không phải là Bảo Mật!

**CORS (Cross-Origin Resource Sharing)** là một trong những khái niệm bị hiểu nhầm nhiều nhất. 
Mặc định, trình duyệt Web có chính sách **Same-Origin Policy**: Cấm trang web `evil.com` gọi API ngầm tới `your-api.com` để lấy cắp dữ liệu của người dùng. Trình duyệt sẽ chặn kết quả trả về.

Nhiều lập trình viên lười biếng thường set `allow_origins=["*"]` để lách luật. Điều này cực kỳ nguy hiểm nếu API của bạn dùng Cookie/Session.
Tuy nhiên, hãy nhớ: **CORS chỉ hoạt động trên Trình Duyệt**. Nếu hacker dùng Postman, `curl`, hoặc script Python để gọi API của bạn, CORS hoàn toàn vô tác dụng (vì không có trình duyệt nào chặn cả).

Do đó: CORS bảo vệ người dùng của bạn khỏi các trang web độc hại, chứ CORS không bảo vệ Server của bạn khỏi hacker!

```python
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI()

# ✅ Cấu hình chuẩn Production
app.add_middleware(
    CORSMiddleware,
    allow_origins=["https://myfrontend.com"],  # CHỈ CHO PHÉP tên miền của Frontend bạn!
    allow_credentials=True,
    allow_methods=["GET", "POST", "PUT", "DELETE"],
    allow_headers=["Authorization", "Content-Type"],
)
```

---

## 36.3 — XSS và Security Headers

**XSS (Cross-Site Scripting)** xảy ra khi hacker chèn được đoạn mã JavaScript độc hại vào trang web của bạn (ví dụ: gõ `<script>alert('hack')</script>` vào ô Bình luận). Khi user khác đọc bình luận đó, trình duyệt của họ sẽ chạy đoạn JS kia và bị lấy cắp token.

Để phòng chống XSS:
1. **Frontend**: Các framework hiện đại như React/Vue tự động escape HTML (biến `<` thành `&lt;`). Trừ khi bạn dùng `dangerouslySetInnerHTML`.
2. **Backend Headers**: Cấu hình **Content Security Policy (CSP)**.

CSP là một header thông báo cho trình duyệt: *"Trang web của tao chỉ được phép chạy file JavaScript tải từ tao, cấm tuyệt đối chạy JS nhúng trực tiếp (inline) hoặc từ domain khác"*.

### Cấu hình Security Headers trong FastAPI

```python
from fastapi import FastAPI, Request, Response

app = FastAPI()

@app.middleware("http")
async def add_security_headers(request: Request, call_next):
    response: Response = await call_next(request)
    
    # 1. Chống XSS (Content Security Policy)
    response.headers["Content-Security-Policy"] = "default-src 'self'"
    
    # 2. Ngăn trình duyệt tự đoán định dạng file (Chống MIME Sniffing)
    response.headers["X-Content-Type-Options"] = "nosniff"
    
    # 3. Chống Clickjacking (Cấm web khác nhúng web của bạn vào thẻ <iframe>)
    response.headers["X-Frame-Options"] = "DENY"
    
    # 4. Ép trình duyệt luôn dùng HTTPS (HTTP Strict Transport Security - HSTS)
    response.headers["Strict-Transport-Security"] = "max-age=31536000; includeSubDomains"
    
    return response
```

---

---

## ✅ Checkpoint 36

1. SQLAlchemy tự escape tham số. Vậy còn cách nào để vẫn dính SQL injection không?
2. Vì sao "CORS không phải là bảo mật"?
3. `Content-Security-Policy` chặn được loại XSS nào, và **không** chặn được loại nào?

<details>
<summary>Đáp án</summary>

1. Có. `text(f"SELECT * FROM t WHERE id = {user_input}")` — f-string nối chuỗi thì không có tham số hoá nào cả. Ngoài ra tên bảng/cột động cũng không tham số hoá được, phải whitelist.
2. Vì CORS chỉ ràng buộc **trình duyệt**. `curl`, Postman, hay bất kỳ HTTP client nào cũng bỏ qua nó hoàn toàn. CORS bảo vệ người dùng của bạn khỏi trang web khác, chứ không bảo vệ API của bạn.
3. Chặn tốt inline script và script từ nguồn lạ. **Không** chặn được lỗ hổng ở phía server (ví dụ template render dữ liệu chưa escape thành HTML hợp lệ đúng nguồn cho phép). CSP là lớp phòng thủ thứ hai, không thay thế việc escape output.
</details>

---

## 🏋️ Bài tập

**Bài 1 (5 phút).** Viết một endpoint FastAPI **cố tình** dính SQL injection bằng f-string, khai thác nó, rồi sửa bằng tham số hoá. Chạy trên SQLite local.

**Bài 2 (10 phút).** Cấu hình `secure` library để thêm đủ bộ security header. Kiểm tra kết quả bằng `curl -I` và đối chiếu với securityheaders.com.

**Bài 3 (20 phút).** Cài rate limiting bằng `slowapi` cho endpoint login: 5 lần thử/phút theo IP. Rồi trả lời: vì sao giới hạn theo IP là **chưa đủ**, và bạn còn nên giới hạn theo chiều nào nữa?

<details>
<summary>Gợi ý bài 3</summary>

Theo IP chặn được một kẻ tấn công dò nhiều tài khoản. Nhưng **credential stuffing**
đến từ hàng nghìn IP khác nhau, mỗi IP thử đúng một lần vào một tài khoản. Cần
giới hạn thêm theo **username** — và cảnh báo khi tỉ lệ đăng nhập thất bại toàn
hệ thống tăng bất thường.
</details>

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| Lỗi CORS trên trình duyệt dù đã bật middleware | Preflight `OPTIONS` chưa được cho phép | Thêm `allow_methods=["*"]`, kiểm tra `allow_credentials` |
| `allow_origins=["*"]` + cookie không hoạt động | Chuẩn cấm dùng wildcard cùng credentials | Liệt kê origin cụ thể |
| CSP làm hỏng giao diện | Có inline style/script | Dùng nonce/hash, hoặc tách ra file riêng |
| `slowapi` không giới hạn gì | Thiếu middleware hoặc sai key function | Đăng ký `SlowAPIMiddleware`; xác nhận `get_remote_address` lấy đúng IP sau proxy |
| Sau reverse proxy, mọi request cùng một IP | Không đọc `X-Forwarded-For` | Bật `ProxyHeadersMiddleware` của Uvicorn |

## Tóm tắt

- ✅ **SQL Injection**: Luôn dùng ORM hoặc Parameterized Queries. Không bao giờ format chuỗi SQL.
- ✅ **CORS**: Là cơ chế bảo vệ user trên trình duyệt, không phải lá chắn bảo vệ server. Luôn whitelist chính xác domain của bạn thay vì dùng `*`.
- ✅ **Security Headers**: HSTS ép HTTPS, CSP chống XSS, X-Frame-Options chống Clickjacking. Chúng là những công tắc đơn giản nhưng chặn được 80% các kiểu tấn công client-side.

## Tiếp theo

Đến đây, ứng dụng Web của chúng ta đã nhanh, đáng tin cậy và an toàn. Tuy nhiên, hệ thống hiện tại mới chỉ chạy trên một máy chủ duy nhất (Monolith). Khi bạn có hàng triệu user, bạn cần chia nhỏ chúng ra. Chào mừng bạn đến với kỷ nguyên của **Chapter 37: Distributed Systems**.
