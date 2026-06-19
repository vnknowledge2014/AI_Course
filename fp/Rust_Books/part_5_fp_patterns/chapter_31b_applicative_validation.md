# Chapter 31b — Applicative Functor: Thu thập mọi lỗi với Validation

> **Bạn sẽ học được**:
> - Sự khác biệt cốt lõi giữa Monad (Fail-fast) và Applicative (Collect-all).
> - Tại sao toán tử `?` của Rust lại phản tác dụng khi Validate Form người dùng.
> - Xây dựng mô hình `Validated` bằng cách sử dụng `Result<T, Vec<E>>`.
> - Cách viết các combinators để gộp (zip) các kết quả độc lập và thu thập danh sách lỗi.
>
> **Yêu cầu trước**: Chapter 31 (Monads & `?`).
> **Thời gian đọc**: ~35 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Biết cách trả về nhiều lỗi cùng lúc cho người dùng thay vì bắt họ sửa từng lỗi một một cách phiền toái.

---

Giả sử bạn đang viết API đăng ký tài khoản người dùng. User gửi lên `username`, `email` và `age`.
Bạn viết một hàm validate dùng toán tử `?` (Monad):

```rust
// Mô phỏng fail-fast
fn register(username: &str, email: &str, age: i32) -> Result<User, String> {
    let u = validate_username(username)?; // Nếu lỗi, DỪNG LẠI NGAY và trả về Err!
    let e = validate_email(email)?;       // Hàm này không bao giờ chạy nếu username lỗi.
    let a = validate_age(age)?;           
    Ok(User { username: u, email: e, age: a })
}
```

Vấn đề là: Người dùng gửi form sai cả 3 trường.
1. Bấm Submit -> Báo lỗi "Username quá ngắn". (Họ sửa username).
2. Bấm Submit -> Báo lỗi "Email sai định dạng". (Họ tức giận sửa email).
3. Bấm Submit -> Báo lỗi "Age phải lớn hơn 18". (Họ chửi thề và bỏ app của bạn).

Đó là hậu quả của **Fail-fast** (Đặc trưng của Monad / `and_then`).
Đối với Form Validation, các trường độc lập với nhau. Chúng ta muốn **Collect-all** (Gom tất cả lỗi): "Bạn có 3 lỗi: Username ngắn, Email sai, Age nhỏ".

Đó là lúc ta cần tới **Applicative Functor**.

---

## 31b.1 — Từ Monad sang Applicative

Monad (toán tử `?` hoặc `and_then`) ngụ ý **sự phụ thuộc tuần tự**. Bước B cần kết quả của Bước A mới chạy được.
Applicative ngụ ý **sự độc lập**. Bước A, B, C không liên quan tới nhau, có thể chạy song song, và sau đó GỘP (zip) kết quả lại.

Để gom nhiều lỗi ở Rust, thay vì dùng `Result<T, E>`, ta sẽ dùng một kiểu dữ liệu mà nhánh Err là một Mảng (Vec): `Result<T, Vec<E>>`.

### Xây dựng hàm `zip`

Ý tưởng: Ta viết một hàm nhận vào hai `Result`.
- Nếu cả hai là `Ok`, trả về `Ok(Tuple)`.
- Nếu một cái `Ok`, một cái `Err`, trả về `Err`.
- **ĐẶC BIỆT**: Nếu CẢ HAI đều là `Err`, ta **CỘNG GỘP (concat)** hai mảng lỗi lại!

```rust
// filename: src/applicative_zip.rs

// Khai báo một bí danh (alias) cho gọn
type Validated<T, E> = Result<T, Vec<E>>;

// Hàm gộp hai Validated độc lập
fn zip_validated<A, B, E>(
    val_a: Validated<A, E>,
    val_b: Validated<B, E>,
) -> Validated<(A, B), E> {
    match (val_a, val_b) {
        // Cả hai OK -> Gộp thành Tuple
        (Ok(a), Ok(b)) => Ok((a, b)),
        
        // Cả hai LỖI -> Nối 2 mảng lỗi lại với nhau (Thu thập lỗi)
        (Err(mut errs_a), Err(mut errs_b)) => {
            errs_a.append(&mut errs_b);
            Err(errs_a)
        }
        
        // Một trong hai lỗi -> Giữ nguyên lỗi đó
        (Err(errs), Ok(_)) => Err(errs),
        (Ok(_), Err(errs)) => Err(errs),
    }
}

// Giả lập các hàm validation độc lập
fn val_name(name: &str) -> Validated<String, String> {
    if name.len() >= 3 { Ok(name.to_string()) } 
    else { Err(vec!["Name too short".to_string()]) }
}

fn val_age(age: i32) -> Validated<i32, String> {
    if age >= 18 { Ok(age) } 
    else { Err(vec!["Too young".to_string()]) }
}

fn main() {
    let result_ok = zip_validated(val_name("Alice"), val_age(20));
    assert_eq!(result_ok, Ok(("Alice".to_string(), 20)));

    let result_err = zip_validated(val_name("Bo"), val_age(15));
    // THU THẬP ĐƯỢC CẢ 2 LỖI!
    assert_eq!(result_err, Err(vec![
        "Name too short".to_string(), 
        "Too young".to_string()
    ]));
}
```

> **💡 Bản chất**: Khi ta gọi `zip_validated`, CẢ HAI hàm `val_name` và `val_age` ĐỀU ĐƯỢC THỰC THI (eager evaluation). Điều này khác hoàn toàn với `and_then` (lazy) — chỉ thực thi hàm thứ 2 nếu hàm thứ 1 thành công. Đây chính là linh hồn của Applicative!

---

## 31b.2 — Map qua Tuple (Gộp thành Struct)

Bây giờ ta đã có kết quả là `Ok((name, age))`. Ta muốn biến Tuple này thành `User` Struct.
Vì nhánh `Ok` là một Functor, ta có thể dễ dàng dùng `.map()`!

```rust
// filename: src/applicative_map.rs

type Validated<T, E> = Result<T, Vec<E>>;

// Hàm zip_validated đã viết ở trên...
fn zip_validated<A, B, E>(a: Validated<A, E>, b: Validated<B, E>) -> Validated<(A, B), E> {
    match (a, b) {
        (Ok(a), Ok(b)) => Ok((a, b)),
        (Err(mut ea), Err(mut eb)) => { ea.append(&mut eb); Err(ea) },
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}

#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: i32,
}

// ...val_name và val_age...
fn val_name(name: &str) -> Validated<String, String> {
    if name.len() >= 3 { Ok(name.to_string()) } else { Err(vec!["Name short".to_string()]) }
}
fn val_age(age: i32) -> Validated<i32, String> {
    if age >= 18 { Ok(age) } else { Err(vec!["Under 18".to_string()]) }
}

fn main() {
    // Pipeline Applicative: Zip -> Map
    let user_result: Validated<User, String> = 
        zip_validated(val_name("Bob"), val_age(15))
        .map(|(n, a)| User { name: n, age: a });

    println!("{:?}", user_result);
    // Output: Err(["Name short", "Under 18"])
}
```

---

## 31b.3 — Mở rộng lên N tham số bằng Macro

Cách dùng `zip` chạy rất tốt cho 2 tham số. Nếu có 3 tham số (Name, Age, Email), bạn phải lồng zip:
`zip(zip(name, age), email)`. Trông khá tệ.

Rust là ngôn ngữ có hệ thống **Macro** mạnh nhất nhì thế giới. Chúng ta có thể tự viết một macro tên là `validate!` để gom N lỗi, cho trải nghiệm viết code siêu việt.

```rust
// filename: src/validate_macro.rs

macro_rules! validate {
    ( $val_a:expr, $val_b:expr ) => {
        zip_validated($val_a, $val_b)
    };
    // Đệ quy macro cho 3 tham số
    ( $val_a:expr, $val_b:expr, $val_c:expr ) => {
        zip_validated(
            zip_validated($val_a, $val_b),
            $val_c
        ).map(|((a, b), c)| (a, b, c))
    };
    // Bạn có thể mở rộng tiếp cho 4, 5... tham số
}

// Vẫn sử dụng các hàm ở trên.

fn val_email(email: &str) -> Validated<String, String> {
    if email.contains("@") { Ok(email.to_string()) } 
    else { Err(vec!["Invalid Email".to_string()]) }
}

fn main() {
    // Thử 3 tham số bị lỗi!
    let tuple_result = validate!(
        val_name("Bo"), 
        val_age(12), 
        val_email("nomail.com")
    );
    
    // Gộp tất cả thành User. Dùng map!
    let user = tuple_result.map(|(n, a, e)| User { name: n, age: a, email: e });
    
    println!("{:#?}", user);
    // Err([
    //    "Name short",
    //    "Under 18",
    //    "Invalid Email",
    // ])
}
```

> **Ghi chú**: Trong thực tế, các crate Rust nổi tiếng như `frunk` cung cấp sẵn kiểu `Validated` và macro để làm chính xác việc này. Tuy nhiên, tự hiểu cơ chế `zip` giúp bạn làm chủ Applicative mà không cần cài thêm dependency.

---

## ✅ Checkpoint 31b.1-31b.3

> Đến đây bạn phải hiểu:
> 1. **Monad (Fail-fast)**: Thích hợp cho pipeline có tính phụ thuộc (A có Ok mới chạy B). Công cụ: `?`, `and_then`.
> 2. **Applicative (Collect-all)**: Thích hợp cho Validation độc lập (A và B chạy không cần chờ nhau, lỗi thì gộp lại).
> 3. **Cấu trúc Validated**: Bản chất là `Result<T, Vec<E>>`. Chìa khóa nằm ở hàm `zip` giúp nối hai mảng `Vec<E>` khi cả hai đều Err.
> 4. **Biến hình qua `map`**: Sau khi zip thành công ra Tuple, dùng `.map()` để biến Tuple thành Struct.
>
> **Test nhanh**: Điều kiện MẤU CHỐT nào của hàm `zip_validated` khiến nó hoạt động như một Applicative thay vì một hàm Fail-fast bình thường?
> <details><summary>Đáp án</summary>Là nhánh match `(Err(mut ea), Err(mut eb))`. Thay vì ném bỏ 1 lỗi, nó thực hiện `ea.append(&mut eb)` để giữ lại cả 2 lỗi. Bất cứ cấu trúc nào làm được trò "cộng gộp" này (nhờ E là một Semigroup - Vec) đều tạo nên Applicative!</details>

---

## 🏋️ Bài tập

**Bài 1** (15 phút): Password Validator

```rust
// Bạn cần validate Password dựa trên 3 tiêu chí:
// 1. Phải có độ dài tối thiểu 8 ký tự.
// 2. Phải chứa ít nhất 1 chữ số (digit).
// 3. Phải chứa ít nhất 1 chữ hoa (uppercase).

// Các hàm kiểm tra trả về Validated<(), String> (Vì ta không cần biến đổi chuỗi, chỉ cần pass).
// YÊU CẦU:
// Hãy hoàn thành hàm `validate_password` gom cả 3 lỗi lại bằng `zip_validated`.
// Nếu thành công, trả về chính password đó.

type Validated<T, E> = Result<T, Vec<E>>;

fn min_length(pw: &str) -> Validated<(), String> {
    if pw.len() >= 8 { Ok(()) } else { Err(vec!["Too short".to_string()]) }
}
fn has_digit(pw: &str) -> Validated<(), String> {
    if pw.chars().any(|c| c.is_digit(10)) { Ok(()) } else { Err(vec!["Missing digit".to_string()]) }
}
fn has_upper(pw: &str) -> Validated<(), String> {
    if pw.chars().any(|c| c.is_uppercase()) { Ok(()) } else { Err(vec!["Missing upper".to_string()]) }
}

// Hãy viết hàm này:
fn validate_password(pw: &str) -> Validated<String, String> {
    // Code của bạn ở đây...
}
```

<details><summary>✅ Lời giải Bài 1</summary>

```rust
// Dùng hàm zip_validated có sẵn
fn zip_validated<A, B, E>(a: Validated<A, E>, b: Validated<B, E>) -> Validated<(A, B), E> {
    match (a, b) {
        (Ok(a), Ok(b)) => Ok((a, b)),
        (Err(mut ea), Err(mut eb)) => { ea.append(&mut eb); Err(ea) },
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}

fn validate_password(pw: &str) -> Validated<String, String> {
    // Zip 3 validation rules
    let rules_result = zip_validated(
        zip_validated(min_length(pw), has_digit(pw)),
        has_upper(pw)
    );
    
    // Nếu pass tất cả (ra được Ok( ((), ()) ) ), map nó thành chính chuỗi pw
    rules_result.map(|_| pw.to_string())
}

// Test
fn main() {
    println!("{:?}", validate_password("weak")); 
    // Err(["Too short", "Missing digit", "Missing upper"])
    
    println!("{:?}", validate_password("Strong123")); 
    // Ok("Strong123")
}
```
</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Không thể dùng toán tử `?` để unwrap bên trong Applicative | `?` là fail-fast | KhÔNG dùng `?` khi bạn muốn gom lỗi. `?` sẽ lập tức `return Err` thoát khỏi hàm, làm mất khả năng chạy tiếp các validation phía sau. |
| Hàm `zip_validated` báo lỗi `String` không có hàm `append` | `String` không phải là `Vec` | Chữ ký của lỗi phải luôn được bọc trong một Collection có khả năng gộp (Semigroup), điển hình nhất là `Vec<E>`. Đừng trả về `Result<T, String>`, hãy trả về `Result<T, Vec<String>>`. |

---

## Tóm tắt

- ✅ **Khác biệt cốt lõi**: Monad chạy tuần tự, phụ thuộc lẫn nhau, Fail-fast. Applicative chạy độc lập, không phụ thuộc, Collect-all.
- ✅ **`Result<T, Vec<E>>`**: Đây là đại diện tiêu biểu nhất cho Applicative Validation.
- ✅ Hàm **`zip`** là trái tim của Applicative. Nhiệm vụ của nó là gom (concat) danh sách lỗi khi cả hai nhánh đều thất bại.
- ✅ Pattern này cực kỳ hữu dụng trong Web Backend (API Request Validation), nơi bạn muốn trả về toàn bộ lỗi Form Validation cho Frontend trong một lần quét duy nhất.

## Tiếp theo

Bạn đã biết cách xử lý 2-3 phần tử độc lập bằng `zip`. Nhưng nếu bạn có một *mảng* chứa 1000 phần tử (ví dụ: `Vec<Result<T, Vec<E>>>`), làm sao bạn gom tất cả chúng lại thành `Result<Vec<T>, Vec<E>>`?
Hẹn gặp bạn ở **Chapter 32b: Traverse & Sequence**.
