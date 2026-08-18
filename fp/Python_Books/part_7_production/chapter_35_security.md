# Chapter 35 — Security Essentials: Hashing, Auth & Validation

> **Bạn sẽ học được**:
> - Tại sao MD5 hay SHA-256 lại là "thảm họa" khi dùng để băm mật khẩu. Sự khác biệt giữa Fast Hashing và Slow Hashing (Bcrypt, Argon2).
> - Cơ chế hoạt động của JSON Web Token (JWT) và cách nó giúp hệ thống Stateless. Điểm yếu chí mạng của JWT.
> - Khái niệm "Defense in Depth" (Phòng thủ chiều sâu) bắt đầu từ Input Validation với Pydantic.
>
> **Yêu cầu trước**: Chapter 34 (Advanced Data)
> **Thời gian đọc**: ~35 phút | **Level**: Principal
> **Kết quả cuối cùng**: Hiểu rõ bản chất của việc xác thực (Authentication) thay vì chỉ biết copy-paste code từ thư viện.

---

Bảo mật không phải là một tính năng bạn "thêm vào" ở cuối dự án. Trong mô hình Functional & Domain-Driven Design, bảo mật bắt đầu ngay từ lúc bạn định nghĩa Data Types. Ở chương này, chúng ta tập trung vào **Authentication** (Xác thực người dùng) và **Data Integrity** (Tính vẹn toàn của dữ liệu).

## 35.1 — Mật mã học cơ bản: Password Hashing

Quy tắc số 1 của bảo mật Web: **KHÔNG BAO GIỜ LƯU PASSWORD DƯỚI DẠNG CLEAR-TEXT (chữ thuần)**.
Nếu Database bị lộ, hacker sẽ có toàn bộ mật khẩu của user (và thường user dùng chung 1 pass cho nhiều website).

Chúng ta phải dùng hàm Băm (Hashing). Nhưng dùng hàm nào?
- ❌ **MD5, SHA-1, SHA-256**: Đây là các hàm băm tốc độ cao (Fast Hashing). CPU hiện đại có thể thử (brute-force) hàng chục TỶ chuỗi SHA-256 mỗi giây. Nếu bạn dùng SHA-256, hacker có thể crack password "123456" trong chưa tới 0.001 giây.
- ✅ **Bcrypt, Scrypt, Argon2**: Đây là các hàm băm **CỐ TÌNH LÀM CHẬM** (Slow Hashing). Chúng yêu cầu thuật toán chạy tốn CPU và RAM. Nếu mỗi lần băm tốn 0.5 giây, hacker dùng siêu máy tính cũng phải khóc thét.

### Salt là gì?
Nếu 2 user cùng đặt pass là "123456", mã băm của họ sẽ giống hệt nhau. Hacker chỉ cần lập một bảng tra cứu (Rainbow table) là xong. 
**Salt** là một chuỗi ngẫu nhiên sinh ra cho MỖI user, cộng vào password trước khi băm. Nó khiến mã băm của mỗi người là độc nhất, đập tan thuật toán Rainbow Table.

```python
import hashlib
import secrets

# Một ví dụ MÔ PHỎNG Slow Hashing bằng PBKDF2
# Trong thực tế, hãy dùng thư viện `passlib` kết hợp `bcrypt` hoặc `argon2-cffi`
def hash_password(password: str) -> str:
    # 1. Sinh Salt ngẫu nhiên (16 bytes)
    salt = secrets.token_hex(16)
    
    # 2. Băm lặp đi lặp lại 100,000 lần (Làm chậm quá trình)
    hashed = hashlib.pbkdf2_hmac("sha256", password.encode(), salt.encode(), 100_000)
    
    # 3. Lưu cả Salt và Hash vào Database
    return f"{salt}:{hashed.hex()}"

def verify_password(password: str, stored: str) -> bool:
    salt, hash_hex = stored.split(":")
    # Dùng lại đúng Salt đó để băm password user nhập vào
    hashed = hashlib.pbkdf2_hmac("sha256", password.encode(), salt.encode(), 100_000)
    # So sánh kết quả
    return hashed.hex() == hash_hex

stored = hash_password("my_secret_123")
assert verify_password("my_secret_123", stored)
assert not verify_password("wrong_password", stored)
```

---

## 35.2 — JWT Authentication (JSON Web Token)

Trong hệ thống cũ, khi user đăng nhập, server tạo ra một Session ID lưu vào RAM hoặc Redis, rồi gửi Session ID đó về Cookie. Mỗi lần user request, server phải móc Database/Redis ra xem Session ID còn hạn không. Cực kỳ tốn kém khi scale!

**JWT** giải quyết bài toán này bằng cơ chế **Stateless** (Không lưu trạng thái ở server). 
Server đóng gói thông tin user `{"user_id": 42}` thành một token, **ký mã hóa (Sign)** bằng một SECRET KEY bí mật, rồi gửi cho Client. Client cứ cầm token đó mà request. Server chỉ cần xác minh Chữ Ký (Signature) là biết token có hợp lệ hay bị giả mạo không.

JWT có 3 phần, cách nhau bởi dấu chấm `.`:
1. **Header**: Thuật toán ký (vd: HS256)
2. **Payload**: Data (vd: user_id, thời gian hết hạn exp)
3. **Signature**: Chữ ký = Hash(Header + Payload + Secret)

> ⚠️ **CẢNH BÁO**: Ai cũng có thể ĐỌC được Header và Payload (chúng chỉ encode base64, không encrypt). **Tuyệt đối không lưu Password, Thẻ tín dụng vào JWT Payload**.

```python
import json, base64, hmac, hashlib, time

SECRET = "super-secret-key-change-in-production"

def create_jwt(payload: dict, expiry_seconds: int = 3600) -> str:
    # 1. Header
    header = base64.urlsafe_b64encode(json.dumps({"alg": "HS256"}).encode()).decode()
    
    # 2. Payload (Thêm exp - Thời gian hết hạn)
    payload["exp"] = int(time.time()) + expiry_seconds
    body = base64.urlsafe_b64encode(json.dumps(payload).encode()).decode()
    
    # 3. Signature (Dùng HMAC-SHA256 băm Header + Body + Secret)
    signature = hmac.new(SECRET.encode(), f"{header}.{body}".encode(), hashlib.sha256).hexdigest()
    
    return f"{header}.{body}.{signature}"

def verify_jwt(token: str) -> dict | None:
    parts = token.split(".")
    if len(parts) != 3: return None
    header, body, signature = parts
    
    # Tính lại chữ ký dựa trên Header & Body client gửi lên
    expected = hmac.new(SECRET.encode(), f"{header}.{body}".encode(), hashlib.sha256).hexdigest()
    
    # Nếu chữ ký sai -> Có kẻ giả mạo token!
    if signature != expected: return None
    
    payload = json.loads(base64.urlsafe_b64decode(body + "=="))
    
    # Kiểm tra token đã hết hạn chưa
    if payload.get("exp", 0) < time.time(): return None
    return payload
```

*Nhược điểm chí mạng của JWT*: Rất khó để Thu hồi (Revoke) token trước khi nó hết hạn (ví dụ khi user ấn Đăng Xuất). Người ta thường kết hợp JWT (Sống ngắn 15 phút) và Refresh Token (Sống dài 7 ngày, lưu ở DB) để giải quyết.

---

## 35.3 — Input Validation: Phòng thủ chiều sâu

Lỗ hổng bảo mật thường sinh ra do chúng ta quá tin tưởng dữ liệu người dùng nhập vào. 
Trong mô hình Domain-Driven Design, chúng ta cấm dữ liệu rác (invalid data) xâm nhập vào tầng Domain. Pydantic chính là "biên phòng" vững chắc nhất.

```python
from pydantic import BaseModel, Field, field_validator
import re

class LoginRequest(BaseModel):
    email: str
    password: str = Field(min_length=8, max_length=128)

    @field_validator("email")
    @classmethod
    def validate_email(cls, v: str) -> str:
        # Nếu ai đó nhét mã SQL/XSS vào email, Regex này sẽ chặn đứng ngay lập tức!
        if not re.match(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$", v):
            raise ValueError("Invalid email format")
        return v.lower()

# Pydantic validation = First line of defense
```

**Defense in Depth** (Phòng thủ chiều sâu) nghĩa là bạn không chỉ bảo vệ ở 1 lớp. 
- Lớp 1: Pydantic validate Input (chặn XSS/Injection).
- Lớp 2: Type System chặn gán nhầm kiểu (User Id không thể gán cho Admin Id).
- Lớp 3: ORM (SQLAlchemy) chặn SQL Injection qua cơ chế Bind Parameters.

---

---

## ✅ Checkpoint 35

1. Vì sao **không bao giờ** dùng SHA-256 để hash mật khẩu, dù nó là hàm băm mã hoá?
2. JWT lưu ở `localStorage` hay cookie `HttpOnly`? Mỗi lựa chọn phơi bày rủi ro gì?
3. Access token và refresh token nên có thời hạn thế nào, và vì sao lại tách hai loại?

<details>
<summary>Đáp án</summary>

1. Vì SHA-256 được thiết kế để **nhanh** — GPU thử được hàng tỷ lần/giây. Hash mật khẩu phải **chậm có chủ đích** và tốn bộ nhớ: `argon2id`, `bcrypt`, `scrypt`.
2. `localStorage` → JavaScript đọc được → dính XSS. Cookie `HttpOnly` → JS không đọc được, nhưng trình duyệt tự gửi kèm → dính CSRF. Thực tế: cookie `HttpOnly` + `SameSite=Lax` + CSRF token là cân bằng tốt nhất.
3. Access token ngắn (5–15 phút), refresh token dài (ngày–tuần) và **thu hồi được**. Tách ra vì access token không thể thu hồi (server không giữ state), nên phải để nó hết hạn nhanh; refresh token thì có lưu ở server nên thu hồi được ngay khi phát hiện bất thường.
</details>

---

## 🏋️ Bài tập

**Bài 1 (5 phút).** Đo thời gian `argon2` hash một mật khẩu trên máy bạn. Chỉnh tham số để mất ~250ms. Vì sao mốc đó là cân bằng hợp lý giữa UX và chống brute-force?

**Bài 2 (15 phút).** Cài refresh token rotation: mỗi lần dùng refresh token thì cấp cái mới và vô hiệu cái cũ. Nếu một token cũ được dùng lại → thu hồi cả họ token đó.

**Bài 3 (20 phút).** Viết FastAPI dependency `require_role("admin")` trả 403 khi thiếu quyền. Dùng nó như một guard — đây chính là smart constructor của Chapter 14 áp vào tầng HTTP.

<details>
<summary>Gợi ý bài 2</summary>

Token cũ bị dùng lại gần như luôn nghĩa là nó đã bị đánh cắp: người dùng thật đã
đổi sang token mới rồi. Thu hồi cả họ token là phản ứng đúng, dù nó buộc người
dùng thật phải đăng nhập lại.
</details>

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| JWT hợp lệ sau khi user đã logout | JWT stateless, không thu hồi được | Access token ngắn hạn + denylist trên Redis |
| `passlib` cảnh báo về bcrypt | Không tương thích bcrypt 4.x | Ghim `bcrypt<4`, hoặc chuyển sang `argon2` |
| Đăng nhập chậm hẳn | Tham số argon2 quá nặng | Giảm `time_cost`/`memory_cost`, nhắm ~250ms |
| Token hết hạn sớm bất thường | Lệch đồng hồ giữa các server | Đồng bộ NTP; đặt `leeway` nhỏ khi verify |
| Secret key nằm trong git | Hardcode trong source | `pydantic-settings` + biến môi trường; xoay key đã lộ |

## Tóm tắt

- ✅ **Hashing**: Password phải được băm bằng thuật toán chậm (Bcrypt/Argon2) kèm theo Salt ngẫu nhiên.
- ✅ **JWT**: Trái tim của Stateless Auth. Giảm tải cho Database. Lưu ý không chứa dữ liệu nhạy cảm trong JWT Payload.
- ✅ **Validation**: Luôn kiểm duyệt gắt gao dữ liệu đầu vào. "Make illegal states unrepresentable" chính là nguyên tắc bảo mật tối thượng.

## Tiếp theo

Bạn đã nắm được Auth cơ bản. Nhưng thế giới web đầy rẫy hiểm nguy: Hacker lừa user bấm link lạ (CSRF), chèn mã độc vào script (XSS), v.v. Làm sao để bảo vệ Web Server trước top 10 lỗ hổng nguy hiểm nhất (OWASP Top 10)? 
Hẹn gặp bạn ở **Chapter 36: Application Security & Hardening**.
