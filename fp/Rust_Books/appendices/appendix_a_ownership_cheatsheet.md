# Appendix A — Rust Ownership: Bản tóm tắt cho hành trình an toàn

> Chào mừng bạn đến với phần phụ lục. Dù mang tên là "Cheat Sheet" (bản tóm tắt), nhưng tôi muốn kể cho bạn nghe lại câu chuyện về Ownership — linh hồn của Rust — một cách ngắn gọn, dễ hiểu và dễ tra cứu nhất. Bạn có thể xem đây là một trạm dừng chân nhỏ để ôn lại kiến thức trước khi tiếp tục hành trình, hoặc in ra và dán ngay cạnh màn hình! 🖨️

---

## A.1 — Ba Quy Tắc Vàng của Ownership

Mọi thứ trong Rust đều xoay quanh ba quy tắc cốt lõi này. Hãy coi chúng như ba định luật vật lý không thể bị phá vỡ trong vũ trụ Rust:

| Quy tắc | Diễn giải bằng Code |
|------|---------|
| **1. Mỗi giá trị (value) đều có một chủ sở hữu (owner) duy nhất tại một thời điểm.** | Khi bạn viết `let s = String::from("hi")`, biến `s` chính là chủ sở hữu hợp pháp của chuỗi đó. |
| **2. Khi chủ sở hữu đi ra khỏi phạm vi (scope), giá trị sẽ bị hủy (dropped).** | `{ let s = ...; }` — Ngay khi dấu `}` đóng lại, `s` kết thúc vòng đời và bộ nhớ lập tức được giải phóng. |
| **3. Phép gán mặc định là chuyển giao quyền sở hữu (Move)** đối với các kiểu dữ liệu phức tạp. | `let s2 = s;` — Sau dòng này, quyền sở hữu đã chuyển sang `s2`, và bạn không thể dùng `s` được nữa. |

---

## A.2 — Move, Copy và Clone: Ứng xử với dữ liệu

Khi bạn truyền dữ liệu từ biến này sang biến khác, Rust có ba cách ứng xử tùy thuộc vào loại dữ liệu bạn đang nắm giữ.

### Move: Chuyển nhà (Mặc định cho dữ liệu trên Heap)
Khi dữ liệu phức tạp (như `String`), việc sao chép sẽ rất tốn kém. Thay vì copy, Rust chuyển luôn quyền sở hữu.
```rust
let s1 = String::from("hello");
let s2 = s1;          // s1 đã CHUYỂN GIAO (MOVED) dữ liệu cho s2
// println!("{}", s1); // ❌ Lỗi! s1 không còn quyền truy cập dữ liệu nữa.
```

### Copy: Nhân bản tự động (Dành cho dữ liệu nhỏ trên Stack)
Các kiểu dữ liệu cơ bản (số nguyên, boolean) cực kỳ nhẹ. Rust tự động nhân bản chúng khi gán, mà không gây rắc rối gì về quyền sở hữu.
```rust
let x: i32 = 5;
let y = x;            // x tự động COPIED sang y
println!("{}", x);    // ✅ x vẫn dùng được bình thường.
```

*Nhận diện kiểu Copy (Copy types)*: Tất cả số nguyên (`i32`, `u64`), số thực (`f64`), `bool`, `char`, và các Tuple/Array chỉ chứa kiểu Copy. Ngược lại, `String`, `Vec<T>`, `Box<T>` không phải là Copy types.

### Clone: Nhân bản sâu có chủ đích
Khi bạn thực sự cần tạo ra một bản sao thứ hai của dữ liệu trên Heap, hãy dùng lệnh `clone()` một cách rõ ràng:
```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // Tạo ra một bản sao độc lập (deep copy)
println!("{}", s1);   // ✅ Cả hai biến đều hợp lệ.
```

---

## A.3 — References & Borrowing: Nghệ thuật mượn đồ

Rust không bắt bạn lúc nào cũng phải "trao đi" quyền sở hữu. Bạn có thể "cho mượn" dữ liệu thông qua các tham chiếu (References).

### Cho mượn đọc (Shared Reference: `&T`)
Giống như việc bạn cho nhiều người cùng đọc chung một cuốn sách:
```rust
fn len(s: &String) -> usize { s.len() }   // Hàm này chỉ mượn để đọc, không tước quyền sở hữu
let s = String::from("hello");
let n = len(&s);     // Ký hiệu &s nghĩa là "cho mượn đọc"
println!("{}", s);   // ✅ s vẫn an toàn và hợp lệ ở đây
```

### Cho mượn để sửa (Mutable Reference: `&mut T`)
Nếu bạn đưa bản thảo cho ai đó sửa, chỉ một người được cầm bút tại một thời điểm:
```rust
fn push_hi(s: &mut String) { s.push_str(" hi"); }
let mut s = String::from("hello");
push_hi(&mut s);     // Cho mượn để chỉnh sửa
println!("{}", s);   // Kết quả: "hello hi"
```

**Quy tắc mượn đồ (Borrowing Rules):** Bạn có thể có *vô số* người mượn để đọc (`&T`), HOẶC chỉ *duy nhất một* người mượn để sửa (`&mut T`). Không bao giờ được phép vừa có người đang đọc vừa có người đang sửa cùng một lúc.

---

## A.4 — Lifetimes: Vòng đời của tham chiếu

Lifetimes là cách trình biên dịch đảm bảo rằng không có ai cố gắng mượn một món đồ đã bị hủy. Hầu hết thời gian, trình biên dịch đủ thông minh để tự suy luận vòng đời, nhưng đôi khi bạn phải chỉ rõ cho nó biết:

```rust
// 'a là một cái tên đại diện cho "vòng đời".
// Hàm này nói rằng: "Giá trị trả về sẽ sống lâu bằng tham số nào có vòng đời ngắn nhất giữa x và y"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Nếu một `Struct` chứa tham chiếu, nó cũng cần được đánh dấu lifetime để đảm bảo Struct không "sống dai" hơn dữ liệu mà nó đang trỏ tới:
```rust
struct Excerpt<'a> {
    text: &'a str, 
}
```

---

## A.5 — Sơ đồ ra quyết định nhanh (Decision Tree)

Mỗi khi đứng trước ngã ba đường không biết dùng cấu trúc nào, hãy tự hỏi:

**Bạn có cần tiếp tục sử dụng giá trị sau khi truyền vào hàm không?**
- Không cần nữa → Hãy chuyển giao quyền (Pass by value / move / copy)
- Vẫn cần dùng →
    - Chỉ đọc thôi → Dùng `&T` (shared borrow)
    - Cần thay đổi nó → Dùng `&mut T` (mutable borrow)

**Bạn có cần nhiều người cùng chia sẻ quyền sở hữu không (Shared ownership)?**
- Dùng cho luồng đơn (Single thread) → `Rc<T>`
- Dùng cho đa luồng (Multi thread) → `Arc<T>`

**Bạn cần đưa dữ liệu lên vùng nhớ Heap?**
- Dữ liệu tĩnh đơn lẻ → `Box<T>`
- Mảng động thay đổi kích thước → `Vec<T>`

---

## A.6 — Những Pattern Phổ Biến

Đừng ngần ngại sử dụng những mẫu mã nguồn (patterns) sau để làm cho code của bạn thanh lịch hơn:

**Trả về dữ liệu sở hữu (Owned value) — An toàn và đơn giản nhất:**
```rust
fn create() -> String { String::from("hello") }
```

**Nhận vào chuỗi mượn (Borrow) — Tăng tính linh hoạt:**
```rust
// Hàm này nhận cả &String và &str!
fn process(s: &str) { /* read-only */ }
```

**Sử dụng `Into` để tăng trải nghiệm người dùng (Ergonomic):**
```rust
fn greet(name: impl Into<String>) {
    let name = name.into();
    println!("Hello, {}!", name);
}
greet("world");                    // Trình biên dịch tự hiểu &str → String
greet(String::from("world"));      // Trình biên dịch tự hiểu String → String
```

**Clone on Write (Cow) — Tuyệt kỹ tối ưu hiệu năng:**
```rust
use std::borrow::Cow;
fn maybe_modify(s: &str) -> Cow<str> {
    if s.contains("bad") {
        // Chỉ cấp phát bộ nhớ mới khi thực sự cần thay đổi
        Cow::Owned(s.replace("bad", "good"))  
    } else {
        // Nếu không có gì đổi, chỉ trả về tham chiếu, tốn 0 chi phí!
        Cow::Borrowed(s)  
    }
}
```

Hy vọng bản tóm tắt này sẽ như một người bạn đồng hành, giúp bạn vượt qua những "ma trận" của hệ thống Ownership trong Rust một cách tự tin hơn.
