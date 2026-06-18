# Chapter 40 — Security Essentials

> **Bạn sẽ học được**:
> - **Password hashing**: argon2, bcrypt — NEVER SHA/MD5
> - **JWT**: header.payload.signature, refresh tokens
> - **OAuth 2.0**: Authorization Code flow, PKCE
> - **Authorization**: RBAC (roles), ABAC (attributes)
> - **Sessions vs Tokens**: khi nào dùng gì
> - Rust crates: `argon2`, `jsonwebtoken`, `axum-login`
>
> **Yêu cầu trước**: Chapter 35 (DI), Chapter 38 (Database).
> **Thời gian đọc**: ~40 phút | **Level**: Principal
> **Kết quả cuối cùng**: Bạn implement authentication + authorization **đúng cách** — không reinvent the wheel.

---

## 40.1 — Password Hashing

### Tại sao KHÔNG dùng SHA/MD5

Năm 2012, LinkedIn bị hack và lộ 117 triệu passwords. Vấn đề không phải họ không mã hóa — họ có hash bằng SHA-1. Vấn đề là SHA quá nhanh: một GPU trung bình có thể thử 10 tỷ SHA hash mỗi giây. Trong vài giờ, kẻ tấn công đã khôi phục hầu hết passwords.

Biện pháp đúng: dùng hash function **cố tình chậm** (100ms+ mỗi lần) và thêm **salt** ngẫu nhiên vào mỗi password. Chậm = kẻ tấn công chỉ thử được vài trăm lần/giây. Salt = cùng password nhưng hash ra khác nhau cho mỗi user, không thể dùng bảng đã tính sẵn.

```
SHA-256("password123") = ef92b778bafe771e...
SHA-256("password123") = ef92b778bafe771e...  ← CÙNG hash → rainbow table attack!

Argon2("password123", random_salt) = $argon2id$v=19$m=19456,t=2,p=1$...
Argon2("password123", random_salt) = $argon2id$v=19$m=19456,t=2,p=1$...  ← KHÁC hash!
```

### Password hashing rules

| ❌ Never | ✅ Always |
|----------|----------|
| SHA-256, SHA-512, MD5 | **Argon2id**, bcrypt, scrypt |
| Store plaintext | Store salted hash |
| Same salt for all | **Random salt per user** |
| Fast hash (μs) | **Slow hash (100ms+)** — intentional! |

### Rust implementation

```rust
// filename: src/main.rs

// Cargo.toml: argon2 = "0.5"

// Simulated argon2 (conceptual — real crate handles salt internally)
mod password {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    /// Hash password with random salt (simulated)
    pub fn hash_password(password: &str) -> String {
        let salt: u64 = rand_simple();
        let mut hasher = DefaultHasher::new();
        format!("{}:{}", salt, password).hash(&mut hasher);
        let hash = hasher.finish();
        // Format: $algo$salt$hash
        format!("$sim${}${:016x}", salt, hash)
    }

    /// Verify password against stored hash
    pub fn verify_password(password: &str, stored: &str) -> bool {
        let parts: Vec<&str> = stored.split('$').collect();
        if parts.len() != 4 { return false; }
        let salt = parts[2];
        let mut hasher = DefaultHasher::new();
        format!("{}:{}", salt, password).hash(&mut hasher);
        let hash = format!("{:016x}", hasher.finish());
        hash == parts[3]
    }

    fn rand_simple() -> u64 {
        use std::time::SystemTime;
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64
    }
}

fn main() {
    let pw = "MyStr0ng!Pass";

    // Hash (registration)
    let hash = password::hash_password(pw);
    println!("Hash: {}", hash);

    // Verify (login)
    println!("Correct: {}", password::verify_password(pw, &hash));
    println!("Wrong: {}", password::verify_password("wrong", &hash));
}
```

> **Production**: Dùng crate `argon2` thật:
> ```rust
> use argon2::{Argon2, PasswordHasher, PasswordVerifier};
> let salt = SaltString::generate(&mut OsRng);
> let hash = Argon2::default().hash_password(pw.as_bytes(), &salt)?;
> ```

---

## 40.2 — JWT (JSON Web Tokens)

Sau khi user login thành công (password đúng), làm sao server biết các request tiếp theo cũng từ người đó? Cách truyền thống: server lưu session và gửi session ID cho client. Nhưng với hệ thống nhiều servers (load-balanced), session lưu ở server nào? Chia sẻ session giữa các server rất phức tạp.

JWT giải quyết bằng cách đặt mọi thứ vào **trong token** — giống như thẻ VIP của câu lạc bộ: thẻ ghi tên bạn, hạng thành viên, ngày hết hạn, và có dấu mộc của quản lý (chữ ký). Khi bạn đưa thẻ, bảo vệ chỉ cần kiểm tra dấu mộc và hạn — không cần gọi về văn phòng hỏi.

### Structure: 3 parts

```
eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NSIsImVtYWlsIjoibWluaEBjby5jb20ifQ.signature
│                      │                                                 │
└── Header (algo)      └── Payload (claims)                              └── Signature
```

### Rust JWT

```rust
// filename: src/main.rs

use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

// ═══ JWT Claims ═══
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,       // subject (user ID)
    email: String,
    role: String,
    exp: u64,          // expiration (unix timestamp)
    iat: u64,          // issued at
}

// ═══ JWT (simplified, conceptual) ═══
mod jwt {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    const SECRET: &str = "super-secret-key-never-hardcode-in-production";

    pub fn encode(claims: &Claims) -> String {
        let header = r#"{"alg":"HS256","typ":"JWT"}"#;
        let payload = serde_json::to_string(claims).unwrap();

        let header_b64 = base64_encode(header);
        let payload_b64 = base64_encode(&payload);

        let signature = sign(&format!("{}.{}", header_b64, payload_b64));
        format!("{}.{}.{}", header_b64, payload_b64, signature)
    }

    pub fn decode(token: &str) -> Result<Claims, String> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 { return Err("Invalid token format".into()); }

        // Verify signature
        let expected_sig = sign(&format!("{}.{}", parts[0], parts[1]));
        if expected_sig != parts[2] { return Err("Invalid signature".into()); }

        // Decode payload
        let payload = base64_decode(parts[1])?;
        let claims: Claims = serde_json::from_str(&payload)
            .map_err(|e| format!("Invalid payload: {}", e))?;

        // Check expiration
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if claims.exp < now { return Err("Token expired".into()); }

        Ok(claims)
    }

    fn sign(data: &str) -> String {
        let mut hasher = DefaultHasher::new();
        format!("{}:{}", data, SECRET).hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }

    fn base64_encode(s: &str) -> String {
        // Simplified: hex encode
        s.as_bytes().iter().map(|b| format!("{:02x}", b)).collect()
    }

    fn base64_decode(hex: &str) -> Result<String, String> {
        let bytes: Result<Vec<u8>, _> = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i+2], 16))
            .collect();
        String::from_utf8(bytes.map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())
    }
}

fn main() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let claims = Claims {
        sub: "user-123".into(),
        email: "minh@co.com".into(),
        role: "admin".into(),
        exp: now + 3600, // 1 hour
        iat: now,
    };

    // Encode
    let token = jwt::encode(&claims);
    println!("Token: {}...", &token[..50]);

    // Decode
    match jwt::decode(&token) {
        Ok(claims) => println!("Decoded: {} ({})", claims.sub, claims.role),
        Err(e) => println!("Error: {}", e),
    }

    // Tampered token
    let tampered = format!("{}X", token);
    println!("Tampered: {:?}", jwt::decode(&tampered));
}
```

### Access Token + Refresh Token

```
┌──────────┐         ┌──────────┐         ┌──────────┐
│  Login   │ ──────▶ │  Server  │ ──────▶ │  Client  │
│          │         │          │         │          │
│ email+pw │         │ verify   │         │ receives │
│          │         │ password │         │ 2 tokens │
└──────────┘         └──────────┘         └──────────┘
                                              │
                         Access Token  ◄──────┤ (short: 15min)
                         Refresh Token ◄──────┘ (long: 7 days)

Access expired?
  → Use Refresh Token to get new Access Token
  → No re-login needed!

Refresh expired?
  → User must login again
```

---

## 40.3 — OAuth 2.0

### Authorization Code Flow (with PKCE)

```
┌──────┐     1. Login with Google      ┌──────────┐
│ User │ ────────────────────────────▶ │ Auth     │
│      │                               │ Provider │
│      │ ◀──── 2. Auth Code ────────── │ (Google) │
│      │                               └──────────┘
│      │                                    │
│      │     3. Exchange Code               │
│      │        for Token                   │
└──────┘                                    │
    │                                       │
    ▼                                       ▼
┌──────┐     4. Code + Client Secret   ┌──────────┐
│ Your │ ────────────────────────────▶ │ Auth     │
│ App  │                               │ Provider │
│      │ ◀──── 5. Access Token ─────── │          │
│      │                               └──────────┘
│      │
│      │     6. Use Token to get user info
│      │        GET /userinfo
└──────┘
```

### Các khái niệm OAuth 2.0 chính

| Thuật ngữ | Ý nghĩa |
|------|---------|
| **Client ID** | Mã định danh công khai của ứng dụng |
| **Client Secret** | Mã bí mật của ứng dụng (chỉ phía server!) |
| **Auth Code** | Mã tạm thời, đổi lấy token |
| **PKCE** | Proof Key for Code Exchange (cho mobile/SPA) |
| **Scope** | Quyền truy cập (`email`, `profile`, `openid`) |
| **Redirect URI** | URL mà Auth Provider gửi code về |

---

## 40.4 — Authorization: RBAC & ABAC

Authentication trả lời "bạn là ai?" (login). Authorization trả lời "bạn được làm gì?" (quyền hạn). Hai cách phổ biến nhất để quản lý quyền hạn:

**RBAC** gán quyền theo **vai trò**: Admin được làm mọi thứ, Editor được sửa bài, Viewer chỉ được xem. Đơn giản, dễ hình dung — phù hợp phần lớn ứng dụng.

**ABAC** gán quyền theo **thuộc tính**: phòng ban, thời gian, loại tài nguyên, chủ sở hữu. Ví dụ: "Nhân viên phòng tài chính chỉ được truy cập báo cáo tài chính, trong giờ hành chính, nếu là dữ liệu của phòng mình." Phức tạp hơn, nhưng linh hoạt hơn nhiều.

### RBAC (Role-Based Access Control)

```rust
// filename: src/main.rs

use std::collections::HashSet;

// ═══ RBAC ═══
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Role { Admin, Editor, Viewer, Moderator }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Permission {
    ReadContent,
    WriteContent,
    DeleteContent,
    ManageUsers,
    ViewAnalytics,
    ModerateComments,
}

fn role_permissions(role: &Role) -> HashSet<Permission> {
    match role {
        Role::Admin => [
            Permission::ReadContent, Permission::WriteContent,
            Permission::DeleteContent, Permission::ManageUsers,
            Permission::ViewAnalytics, Permission::ModerateComments,
        ].into(),
        Role::Editor => [
            Permission::ReadContent, Permission::WriteContent,
            Permission::ViewAnalytics,
        ].into(),
        Role::Moderator => [
            Permission::ReadContent, Permission::ModerateComments,
        ].into(),
        Role::Viewer => [
            Permission::ReadContent,
        ].into(),
    }
}

struct User {
    id: u64,
    name: String,
    roles: Vec<Role>,
}

impl User {
    fn permissions(&self) -> HashSet<Permission> {
        self.roles.iter()
            .flat_map(|r| role_permissions(r))
            .collect()
    }

    fn has_permission(&self, perm: &Permission) -> bool {
        self.permissions().contains(perm)
    }

    fn can(&self, perm: &Permission) -> Result<(), String> {
        if self.has_permission(perm) { Ok(()) }
        else { Err(format!("{} lacks {:?}", self.name, perm)) }
    }
}

fn main() {
    let admin = User {
        id: 1, name: "Minh".into(), roles: vec![Role::Admin],
    };
    let editor = User {
        id: 2, name: "Lan".into(), roles: vec![Role::Editor],
    };
    let viewer = User {
        id: 3, name: "An".into(), roles: vec![Role::Viewer],
    };

    println!("Admin delete: {:?}", admin.can(&Permission::DeleteContent));
    println!("Editor delete: {:?}", editor.can(&Permission::DeleteContent));
    println!("Viewer write: {:?}", viewer.can(&Permission::WriteContent));
    println!("Editor analytics: {:?}", editor.can(&Permission::ViewAnalytics));

    println!("\nAdmin perms: {:?}", admin.permissions().len());
    println!("Viewer perms: {:?}", viewer.permissions().len());
}
```

### ABAC (Attribute-Based Access Control)

```rust
// filename: src/main.rs

use std::collections::HashMap;

// ═══ ABAC: policy-based ═══
#[derive(Debug)]
struct AccessRequest {
    user_id: u64,
    user_role: String,
    user_department: String,
    resource_type: String,
    resource_owner: u64,
    action: String,
    time_hour: u32, // 0-23
}

fn evaluate_policy(req: &AccessRequest) -> Result<(), String> {
    // Policy 1: Admin can do anything
    if req.user_role == "admin" { return Ok(()); }

    // Policy 2: Users can only read/write own resources
    if req.action == "delete" && req.user_id != req.resource_owner {
        return Err("Can only delete own resources".into());
    }

    // Policy 3: No access outside business hours for non-admins
    if req.time_hour < 8 || req.time_hour > 18 {
        return Err("Access denied: outside business hours".into());
    }

    // Policy 4: Sensitive resources require specific department
    if req.resource_type == "financial" && req.user_department != "finance" {
        return Err("Financial resources: finance dept only".into());
    }

    Ok(())
}

fn main() {
    let requests = vec![
        AccessRequest {
            user_id: 1, user_role: "admin".into(), user_department: "tech".into(),
            resource_type: "document".into(), resource_owner: 2, action: "delete".into(), time_hour: 22,
        },
        AccessRequest {
            user_id: 2, user_role: "user".into(), user_department: "tech".into(),
            resource_type: "document".into(), resource_owner: 3, action: "delete".into(), time_hour: 10,
        },
        AccessRequest {
            user_id: 3, user_role: "user".into(), user_department: "sales".into(),
            resource_type: "financial".into(), resource_owner: 3, action: "read".into(), time_hour: 10,
        },
    ];

    for req in &requests {
        match evaluate_policy(req) {
            Ok(_) => println!("✅ User {} {} {}: allowed", req.user_id, req.action, req.resource_type),
            Err(e) => println!("❌ User {} {} {}: {}", req.user_id, req.action, req.resource_type, e),
        }
    }
}
```

### RBAC vs ABAC

| | RBAC | ABAC |
|---|---|---|
| **Dựa trên** | Vai trò (Admin, Editor) | Thuộc tính (phòng ban, thời gian, tài nguyên) |
| **Mức chi tiết** | Thô (coarse) | Tinh (fine-grained) |
| **Độ phức tạp** | Đơn giản | Phức tạp (nhiều policy) |
| **Khi nào dùng** | Phần lớn ứng dụng ✅ | Enterprise, multi-tenant |

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Bảo mật mật khẩu

Bạn thấy code: `hash = sha256(password)`. Liệt kê 3 vấn đề.

<details><summary>✅ Lời giải</summary>

1. **Không có salt** → tấn công rainbow table (cùng password = cùng hash)
2. **Quá nhanh** → brute-force hàng tỷ lần/giây (GPU attack)
3. **Không adaptive** → không thể tăng chi phí khi phần cứng mạnh hơn

Cách sửa: Dùng `argon2id` với random salt và cost parameters phù hợp.

</details>

---

**Bài 2** (10 phút): JWT middleware

Viết function `authenticate(token: &str) -> Result<Claims, AuthError>` cho middleware:
- Decode JWT
- Check expiration
- Check role is at least "user"
- Return Claims or AuthError enum

<details><summary>✅ Lời giải Bài 2</summary>

```rust
enum AuthError { InvalidToken, Expired, InsufficientRole }

fn authenticate(token: &str) -> Result<Claims, AuthError> {
    let claims = jwt::decode(token).map_err(|e| {
        if e.contains("expired") { AuthError::Expired }
        else { AuthError::InvalidToken }
    })?;

    let valid_roles = ["user", "editor", "admin"];
    if !valid_roles.contains(&claims.role.as_str()) {
        return Err(AuthError::InsufficientRole);
    }

    Ok(claims)
}
```

</details>

---

**Bài 3** (15 phút): Bảo vệ quyền truy cập (Permission guard)

Build generic `require_permission` guard:
```rust
fn require_permission<T>(
    user: &User,
    perm: Permission,
    action: impl FnOnce() -> Result<T, String>,
) -> Result<T, String>
```
Kiểm tra permission trước khi chạy action. Test với 3 scenarios.

<details><summary>✅ Lời giải Bài 3</summary>

```rust
fn require_permission<T>(
    user: &User,
    perm: Permission,
    action: impl FnOnce() -> Result<T, String>,
) -> Result<T, String> {
    user.can(&perm)?;
    action()
}

// Usage
fn delete_post(post_id: u64) -> Result<String, String> {
    Ok(format!("Deleted post {}", post_id))
}

fn main() {
    let admin = User { id: 1, name: "Minh".into(), roles: vec![Role::Admin] };
    let viewer = User { id: 2, name: "An".into(), roles: vec![Role::Viewer] };

    // Admin: OK
    let r = require_permission(&admin, Permission::DeleteContent, || delete_post(1));
    assert!(r.is_ok());

    // Viewer: Denied
    let r = require_permission(&viewer, Permission::DeleteContent, || delete_post(1));
    assert!(r.is_err());
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "JWT trong localStorage" | XSS có thể đánh cắp token | HttpOnly cookie + SameSite |
| "Hash password quá nhanh" | Config yếu | argon2: `m=19456, t=2, p=1` tối thiểu |
| "Tấn công redirect OAuth" | Open redirect | Whitelist redirect URIs |
| "Bùng nổ roles" | Quá nhiều roles | Dùng permission-based thay vì role-based |

---

## Tóm tắt

- ✅ **Password hashing**: Argon2id, never SHA/MD5. Random salt, slow hash.
- ✅ **JWT**: header.payload.signature. Short-lived access + long-lived refresh.
- ✅ **OAuth 2.0**: Auth Code + PKCE flow. Never expose Client Secret to frontend.
- ✅ **RBAC**: Role → Permissions. Simple, covers most apps.
- ✅ **ABAC**: Attribute-based policies. Fine-grained, enterprise.
- ✅ **Rust crates**: `argon2`, `jsonwebtoken`, `oauth2`.

## Tiếp theo

→ Chapter 41: **Application Security & Hardening** — OWASP Top 10, input validation, HTTPS, headers, rate limiting.
