# Chapter 40 — Security Essentials

> **Bạn sẽ học được**:
> - Tại sao MD5 và SHA-256 đã "chết" trong việc lưu trữ mật khẩu.
> - Kỹ thuật mã hóa mật khẩu hiện đại: Salt, Pepper và thuật toán cố tình làm chậm (Argon2id).
> - Bản chất thực sự của JWT (JSON Web Token) và cách nó giải quyết bài toán Session phân tán.
> - OAuth 2.0: Luồng xác thực an toàn nhất hiện nay (Authorization Code + PKCE).
> - Phân quyền hệ thống: RBAC (Theo vai trò) và ABAC (Theo thuộc tính).
>
> **Yêu cầu trước**: Chapter 35 (DI), Chapter 38 (Database).
> **Thời gian đọc**: ~45 phút | **Level**: Principal
> **Kết quả cuối cùng**: Nắm vững các nguyên tắc bảo mật cốt lõi, không bao giờ tự "phát minh lại bánh xe" (reinvent the wheel) trong bảo mật.

---

Bảo mật không phải là cài đặt một bức tường lửa đắt tiền. Bảo mật là một chuỗi các quyết định thiết kế cẩn trọng. Một lỗ hổng nhỏ ở thuật toán Hash mật khẩu có thể khiến hàng triệu tài khoản người dùng bị rao bán trên dark web (như sự kiện LinkedIn lộ 117 triệu mật khẩu năm 2012).

Trong chương này, chúng ta sẽ đi qua các trụ cột của bảo mật: Xác thực (Ai đang đăng nhập?) và Phân quyền (Họ được làm gì?).

## 40.1 — Nghệ thuật lưu trữ Mật khẩu

Nhiều lập trình viên nghĩ rằng: "Chỉ cần băm (hash) mật khẩu bằng SHA-256 là an toàn". **Sai lầm chết người!**

Thuật toán SHA-256 được sinh ra để chạy *càng nhanh càng tốt*. Một card đồ họa (GPU) trung bình hiện nay có thể thử **hàng chục tỷ** phép tính SHA-256 mỗi giây. Nếu hacker lấy được Database của bạn, chúng chỉ mất vài giờ để dịch ngược toàn bộ mật khẩu bằng phương pháp Brute-force hoặc Rainbow Tables (Bảng tra cứu hash định sẵn).

### Giải pháp 1: Salt (Muối)
Để chống lại Rainbow Tables, trước khi Hash, chúng ta rắc thêm một chuỗi ngẫu nhiên (Salt) vào mật khẩu. 
Mỗi User có một Salt khác nhau lưu ngay trong Database.

`Hash("password123" + "RandomSaltABC") = 8f9a3...`

### Giải pháp 2: Thuật toán Cố-tình-chậm (Argon2id)
Dù có Salt, GPU vẫn có thể đoán 10 tỷ lần/giây. Để chặn GPU, các nhà mật mã học tạo ra các thuật toán *cố tình chạy chậm* và ngốn rất nhiều RAM (Memory-hard).
**Argon2id** là tiêu chuẩn vàng hiện tại. Nó bắt CPU phải tốn 100-200ms và chiếm vài chục MB RAM chỉ để tính toán 1 cái hash.
Với tốc độ rùa bò này, hacker có siêu máy tính cũng phải khóc thét khi định Brute-force.

```rust
// Mô phỏng cách Argon2 hoạt động (Chỉ dùng để học, Production hãy dùng crate `argon2`)
mod password_security {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;

    // 1. Tạo Salt ngẫu nhiên
    fn generate_salt() -> String {
        let nanos = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        format!("{:x}", nanos)
    }

    // 2. Hàm Hash chậm (Mô phỏng)
    pub fn hash_password(password: &str) -> String {
        let salt = generate_salt();
        let mut hasher = DefaultHasher::new();
        
        // Trộn Password + Salt
        format!("{}:{}", salt, password).hash(&mut hasher);
        let hash_result = format!("{:016x}", hasher.finish());
        
        // Chuỗi lưu vào DB: Thuật toán $ Salt $ Hash
        format!("$argon2_sim${}${}", salt, hash_result)
    }

    // 3. Hàm Verify
    pub fn verify_password(input_password: &str, db_hash: &str) -> bool {
        let parts: Vec<&str> = db_hash.split('$').collect();
        if parts.len() != 4 { return false; }
        
        let salt = parts[2];
        let original_hash = parts[3];
        
        let mut hasher = DefaultHasher::new();
        format!("{}:{}", salt, input_password).hash(&mut hasher);
        let current_hash = format!("{:016x}", hasher.finish());
        
        current_hash == original_hash
    }
}
```

> **💡 Best Practice**: Tuyệt đối không tự viết thuật toán mã hóa. Hãy dùng Crate `argon2` hoặc `bcrypt` có sẵn của cộng đồng Rust.

---

## 40.2 — Kỷ nguyên của Tokens (JWT)

Khi có 1 máy chủ (Server), dùng Session rất dễ. User login -> Server cấp 1 Session_ID lưu trong RAM -> Trả về Cookie cho user. Lần sau User mang Cookie lên, Server tra RAM là biết ai.

Nhưng khi bạn có 100 Server (Load Balancing), User login ở Server 1, sau đó request thứ hai trúng vào Server 2. Server 2 không có Session đó trong RAM! Bắt User login lại ư? Bạn có thể dùng Redis để share Session, nhưng nó tạo ra nút thắt cổ chai.

**JWT (JSON Web Token)** ra đời để giải quyết triệt để bài toán này. JWT là một cái thẻ căn cước *không cần tra cứu*. Mọi thông tin (Tên, Role) được ghi trực tiếp lên thẻ, và được đóng dấu (Signature) bằng Secret Key của Server. Khi Server 2 nhận thẻ, nó chỉ cần check chữ ký hợp lệ là cho qua!

### Cấu trúc 3 phần của JWT:
1. **Header**: Thuật toán ký (Ví dụ: HS256).
2. **Payload**: Thông tin User (Claims) (Ví dụ: `{"user_id": 123, "role": "admin"}`). Chú ý: Phần này AI CŨNG ĐỌC ĐƯỢC, cấm lưu mật khẩu ở đây!
3. **Signature**: Bằng chứng chống giả mạo. Tính bằng công thức: `Hash(Header + Payload + Secret_Key)`.

```rust
// Mô phỏng JWT (Conceptual)
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Claims {
    user_id: u64,
    role: String,
    exp: u64, // Hạn sử dụng (Expiration)
}

// Nếu User sửa "role": "admin", Signature sẽ bị sai ngay lập tức vì họ không có Secret_Key của Server để tính lại Signature mới!
```

> ⚠️ **Hố tử thần**: JWT đã cấp ra thì **không thể thu hồi** cho đến khi hết hạn. Do đó, Access Token phải có tuổi thọ cực ngắn (15 phút). Để user không phải đăng nhập liên tục, ta dùng cặp **Access Token (Ngắn hạn) + Refresh Token (Dài hạn 7 ngày, lưu vào Database để có thể thu hồi/Revoke)**.

---

## 40.3 — OAuth 2.0 và Đăng nhập Google

Bạn có bao giờ thắc mắc nút "Log in with Google" hoạt động như nào không? Nó dùng giao thức **OAuth 2.0**.
Nguyên tắc tối thượng: **App của bạn tuyệt đối không được nhìn thấy Mật khẩu Google của User.**

Luồng **Authorization Code Flow** an toàn nhất:
1. User bấm nút -> App của bạn chuyển hướng họ sang trang web của Google.
2. User gõ Pass vào Google. Google hỏi: "App này muốn xin email của bạn, đồng ý không?"
3. User bấm Đồng Ý -> Google chuyển hướng về App của bạn kèm theo một mã số rác gọi là **Auth Code**.
4. Frontend của bạn gửi Auth Code này xuống Backend.
5. Backend của bạn cầm cái Auth Code đó, lén chạy cửa sau (Back-channel HTTP) lên Google, nộp kèm với `Client_Secret` (Chìa khóa bí mật của bạn).
6. Google check đúng, trả về **Access Token**. Đăng nhập thành công!

> **Tại sao phải vòng vèo sinh ra Auth Code?** Để ngăn chặn hacker ở trình duyệt (Frontend) chôm được Access Token. Token thực sự chỉ được trao đổi ở Backend thông qua Back-channel an toàn.

---

## 40.4 — Phân Quyền (Authorization)

Xác thực (Authentication) chỉ trả lời câu hỏi: "Bạn là ai?". Còn Phân Quyền (Authorization) trả lời câu hỏi: "Bạn được phép làm gì?".

### 1. RBAC (Role-Based Access Control)
Phân quyền theo **Vai trò**. Cách này cực kỳ phổ biến vì đơn giản.
Bạn định nghĩa các Role (Admin, Editor, Viewer). Mỗi Role có một danh sách Permissions (Delete Post, Write Post, Read Post).

```rust
enum Role { Admin, Editor, Viewer }
enum Permission { DeletePost, WritePost, ReadPost }

// Pure function ánh xạ Role -> Permissions
fn get_permissions(role: &Role) -> Vec<Permission> {
    match role {
        Role::Admin => vec![Permission::DeletePost, Permission::WritePost, Permission::ReadPost],
        Role::Editor => vec![Permission::WritePost, Permission::ReadPost],
        Role::Viewer => vec![Permission::ReadPost],
    }
}
```

### 2. ABAC (Attribute-Based Access Control)
RBAC bất lực trước các yêu cầu phức tạp: *"Tài khoản Editor được sửa bài viết, NHƯNG chỉ được sửa bài DO CHÍNH HỌ VIẾT"*.
Lúc này ta dùng **ABAC**: Phân quyền dựa trên **Thuộc tính** (Ai? Của ai? Mấy giờ?).

```rust
struct AccessRequest {
    user_id: u64,
    resource_owner_id: u64,
    action: String, // "edit"
}

// Hàm Policy đánh giá theo Logic thay vì Roles
fn evaluate_policy(req: &AccessRequest) -> Result<(), String> {
    if req.action == "edit" && req.user_id != req.resource_owner_id {
        return Err("Bạn chỉ được sửa tài nguyên của chính mình!".into());
    }
    Ok(())
}
```

## Tóm tắt

- ✅ **Băm mật khẩu**: MD5/SHA quá nhanh. Hãy dùng Argon2/Bcrypt vì chúng có Salt và cố tình chạy rất chậm.
- ✅ **JWT**: Giải quyết việc Load Balancing mà không cần share Session. Nhớ bài toán thu hồi bằng Refresh Token.
- ✅ **OAuth 2.0**: Luôn dùng luồng Authorization Code để Access Token không bao giờ lộ trên trình duyệt.
- ✅ **RBAC vs ABAC**: RBAC dễ hiểu, ABAC tinh tế. Hãy dùng RBAC làm nền, và ABAC để rào các trường hợp đặc biệt.

## Tiếp theo

Security không chỉ có mật khẩu và token. Hệ thống của bạn có chống được SQL Injection, XSS, hay CSRF không? 
Hãy cùng trang bị áo giáp cho máy chủ ở **Chapter 41: Application Security**.
