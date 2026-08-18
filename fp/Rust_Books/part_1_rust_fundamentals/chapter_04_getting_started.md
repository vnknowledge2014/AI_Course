# Chapter 4 — The Rust Ecosystem & Tooling

> **Bạn sẽ học được**:
> - Hệ sinh thái công cụ của Rust: `rustup`, `rustc`, và `cargo` — và vai trò của từng công cụ.
> - Cấu trúc một project Rust chuẩn.
> - **Linh hồn của trải nghiệm lập trình**: Cài đặt IDE với `rust-analyzer` — tại sao không có nó thì học Rust cực kỳ khó.
> - `clippy` — Không chỉ là bộ kiểm tra mã (linter), mà là một "Senior Developer" miễn phí luôn bên cạnh dạy bạn viết code chuẩn.
> - `rustfmt` — Tự động định dạng code.
> - REPL với `evcxr` — Môi trường chạy code nhanh.
>
> **Yêu cầu trước**: Không cần! Nếu bạn bỏ qua Part 0 vì quá nhiều lý thuyết, bạn đang ở đúng chỗ để bắt đầu thực hành.
> **Thời gian đọc**: ~30 phút | **Level**: Beginner
> **Kết quả cuối cùng**: Bạn có một môi trường lập trình Rust chuẩn chỉ, chuyên nghiệp, với trợ lý ảo `rust-analyzer` và `clippy` sẵn sàng giúp bạn sửa mọi lỗi cú pháp.

---

## 4.1 — Tại sao Rust đáng để học?

Bạn có thể đang tự hỏi: đã có Python, JavaScript, Go — tại sao lại thêm Rust? Câu trả lời nằm ở **trade-off cơ bản** mà mọi ngôn ngữ phải chọn: **an toàn hay nhanh, dễ viết hay dễ bảo trì?**

Python dễ viết nhưng chậm và dễ phát sinh lỗi ngầm do không ép kiểu. C/C++ nhanh nhưng rò rỉ bộ nhớ (memory leaks) là cơn ác mộng. Go cân bằng tốt nhưng hệ thống thu gom rác (garbage collector) thỉnh thoảng làm giật lag.

Rust là ngôn ngữ hiếm hoi giải quyết được mâu thuẫn này: **An toàn bộ nhớ tuyệt đối tại compile time (lúc biên dịch)** mà không cần garbage collector. Code bạn viết bằng Rust sẽ chạy nhanh như C/C++, nhưng an toàn tuyệt đối khỏi các lỗi sập chương trình đột ngột.

Và để làm được điều đó, hệ sinh thái công cụ của Rust được thiết kế cực kỳ xuất sắc.

---

## 4.2 — Toolchain: Bộ Đồ Nghề Của Thợ Lập Trình Rust

Khi bạn cài đặt Rust qua lệnh chuẩn từ trang chủ `rustup.rs`, bạn thực chất nhận được một "Hộp đồ nghề" (Toolchain) gồm 3 công cụ chính:

| Công cụ | Vai trò thực tế | Tương đương trong ngôn ngữ khác |
|---------|---------|-------|
| `rustup` | Quản lý phiên bản Rust trên máy bạn. Dùng để cập nhật Rust lên bản mới. | Giống `nvm` cho Node.js |
| `rustc` | Trình biên dịch (Compiler) — dịch code chữ thành file chạy `*.exe`. | Giống `gcc` cho C |
| `cargo` | Trình quản lý dự án & gói (Package manager). Bạn sẽ dùng cái này 99% thời gian. | Giống `npm` + `webpack` gộp lại |

Trong thực tế, bạn **gần như không bao giờ** gọi lệnh `rustc` trực tiếp. Mọi thao tác đều thông qua `cargo`.

---

## 4.3 — Cargo: Trái Tim Của Mọi Dự Án Rust

Hãy thử tạo dự án đầu tiên của bạn:

```bash
# Tạo một dự án mới có tên là `cafe_order`
cargo new cafe_order

# Di chuyển vào thư mục dự án
cd cafe_order
```

Lệnh `cargo new` tạo ra cấu trúc như sau:

```text
cafe_order/
├── Cargo.toml       # "Sổ hộ khẩu" của dự án — chứa tên, version, và thư viện bên thứ 3
├── .gitignore       # (Nếu dùng git) Bỏ qua file rác
└── src/
    └── main.rs      # File nguồn chính — Chương trình bắt đầu chạy từ đây
```

### Các lệnh Cargo bạn sẽ dùng hàng ngày

```bash
# 1. Chạy thử chương trình (Sẽ tự động biên dịch rồi chạy)
cargo run

# 2. Chỉ biên dịch (Không chạy), kết quả tạo ra file chạy ở `target/debug/`
cargo build

# 3. Biên dịch phiên bản siêu tối ưu (Chạy nhanh nhất, dùng khi mang lên Server)
cargo build --release

# 4. Kiểm tra lỗi nhanh (Rất quan trọng!)
cargo check
```

> **💡 Mẹo cực hay (Pro tip)**: Khi đang viết code, đừng dùng `cargo build` để xem code có lỗi không. Hãy dùng `cargo check`. Lệnh này chỉ đọc code và báo lỗi (nếu có), không tốn thời gian tạo file chạy nên tốc độ siêu nhanh!

---

## 4.4 — IDE & rust-analyzer: Không Có Nó, Học Rust Rất Khổ!

Rust là một ngôn ngữ "giao tiếp" rất nhiều với lập trình viên thông qua Compiler. Nếu bạn dùng Notepad hoặc một trình soạn thảo cơ bản để viết Rust, bạn sẽ thấy nó cực kỳ khó học.

Bạn **bắt buộc** phải cài đặt **rust-analyzer** vào IDE của mình (VSCode, RustRover, hoặc Zed).

### Hướng dẫn cài đặt trên VSCode (Phổ biến nhất):
1. Mở VSCode, vào tab **Extensions** (phím tắt `Ctrl + Shift + X`).
2. Gõ tìm kiếm `rust-analyzer` (của tác giả *The Rust Programming Language*).
3. Bấm **Install**.

### Tại sao rust-analyzer lại thần thánh đến vậy?

Tính năng quan trọng nhất của nó là **Inlay Hints (Gợi ý kiểu dữ liệu ngầm)**.
Trong Rust, bạn không cần khai báo kiểu dữ liệu cho mọi biến, Rust sẽ tự suy ra (Type Inference). 

Ví dụ bạn viết:
```rust
let age = 25;
let name = "Minh";
```
Nhờ `rust-analyzer`, trên màn hình VSCode của bạn sẽ tự động hiện chữ xám mờ mờ để báo cho bạn biết kiểu dữ liệu thực sự:
```text
let age: i32 = 25;
let name: &str = "Minh";
```

Nhờ những "chữ xám mờ" này, bạn sẽ **học được cách Rust hiểu dữ liệu** mà không cần phải đoán. Khi code bạn bị lỗi kiểu dữ liệu (chuyện xảy ra như cơm bữa khi mới học), `rust-analyzer` sẽ gạch chân đỏ ngay lập tức và gợi ý cách sửa *ngay trên IDE* trước cả khi bạn lưu file.

---

## 4.5 — Clippy: Vị "Senior Developer" Khó Tính Nhưng Tốt Bụng

Nếu `rust-analyzer` giúp bạn viết code không bị lỗi, thì **Clippy** giúp bạn viết code **chuẩn phong cách Rust (Idiomatic Rust)**.

Clippy là một linter (bộ soi lỗi phong cách). Để dùng Clippy, gõ lệnh:
```bash
cargo clippy
```

Nếu bạn viết một đoạn code "chạy được, nhưng ngốc nghếch", Clippy sẽ hiện cảnh báo.

**Ví dụ:**
```rust
let pi = 3.14;
if pi == 3.14 {
    println!("Bằng nhau!");
}
```
Clippy sẽ báo ngay:
> `warning: strict comparison of f32 or f64`
> *Giải thích: Bạn không bao giờ nên dùng dấu `==` để so sánh hai số thập phân, vì sai số máy tính sẽ làm nó chạy sai. Hãy kiểm tra xem khoảng cách giữa chúng có đủ nhỏ không.*

Clippy chứa hàng ngàn quy tắc kiểm tra như vậy. **Khi mới học, việc đọc các thông báo của Clippy là cách học Rust nhanh nhất.** Nó giống như có một Senior Developer luôn đứng sau lưng review code cho bạn miễn phí vậy.

---

## 4.6 — Rustfmt: Tạm Biệt Cãi Vã Về Xuống Dòng

Code của bạn trông lộn xộn, khoảng trắng thò ra thụt vào? Không cần tự sửa tay.
Chạy lệnh:
```bash
cargo fmt
```
Toàn bộ dự án sẽ tự động được format lại gọn gàng, đẹp mắt theo đúng tiêu chuẩn chung của cộng đồng Rust.
*Mẹo:* Bạn có thể cấu hình VSCode tự động chạy format mỗi khi bấm `Ctrl + S` (Format On Save).

---

## 4.7 — Tổng hợp: Workflow Làm Việc Hàng Ngày

Để trải nghiệm học Rust được mượt mà nhất, hãy thiết lập thói quen làm việc như sau:

1. **Luôn mở project trong VSCode có cài sẵn `rust-analyzer`.**
2. Cài thêm package `cargo-watch` (chạy lệnh: `cargo install cargo-watch`).
3. Mở Terminal trong VSCode, gõ:
   ```bash
   cargo watch -q -c -x 'clippy'
   ```
   *Lệnh này nghĩa là: Cứ mỗi khi bạn lưu file, nó sẽ xóa màn hình (`-c`), và tự động chạy `cargo clippy`. Bạn sẽ thấy mọi lỗi cú pháp hoặc gợi ý tối ưu hiện ra tức thì ở terminal phía dưới màn hình!*

---

## 4.8 — Chạy Thử Code Nhanh (REPL) với evcxr

Đôi khi bạn chỉ muốn gõ `1 + 1` hoặc thử nghiệm 1 hàm nhỏ mà không muốn tạo hẳn một dự án bằng `cargo new`. Giống như màn hình Console của Python hay Chrome.
Rust cũng có công cụ đó, tên là `evcxr` (đọc là "eviscerator").

Cài đặt:
```bash
cargo install evcxr_repl
```

Sử dụng: Mở terminal, gõ `evcxr`.
```rust
Welcome to evcxr. For help, type :help
>> let x = 5;
>> let y = 10;
>> x + y
15
>> 
```
Bấm `Ctrl + D` hoặc gõ `:quit` để thoát.

---

## 🎉 Tóm tắt
- Bắt đầu với lệnh `cargo new ten_du_an`.
- Chạy code bằng `cargo run`. Soi lỗi nhanh bằng `cargo check`.
- Bắt buộc cài đặt **rust-analyzer** vào IDE.
- Tập thói quen dùng `cargo clippy` để cải thiện tư duy viết code Rust.

Môi trường của bạn đã hoàn hảo. Hãy chuyển sang **Chapter 5** để học những dòng code đầu tiên nhé!

---

## ✅ Checkpoint 4

1. `cargo check` và `cargo build` khác nhau ở đâu, và khi nào dùng cái nào?
2. Vì sao Inlay Hints của rust-analyzer lại đặc biệt hữu ích khi **học** Rust?
3. Clippy khác gì rustc warning?

<details>
<summary>Đáp án</summary>

1. `cargo check` chỉ phân tích và kiểm kiểu, **không** sinh mã máy — nhanh hơn nhiều lần. Dùng nó trong vòng lặp viết code; chỉ `build` khi thật sự cần chạy.
2. Vì Rust suy luận kiểu rất mạnh, nên code thường không viết kiểu ra. Inlay Hints hiển thị kiểu mà compiler đã suy ra — bạn thấy được `&str` hay `String`, `i32` hay `usize`, ngay tại chỗ. Đó là kênh phản hồi tốt nhất khi mới học ownership.
3. rustc báo thứ **sai hoặc nguy hiểm**. Clippy báo thứ **đúng nhưng không idiomatic** — ví dụ `if x == true`, hoặc `.iter().count()` khi đã có `.len()`. Clippy dạy bạn viết Rust như người Rust.
</details>

---

## 🏋️ Bài tập

**Bài 1 (5 phút).** `cargo new hello`, viết chương trình in ra 10 số Fibonacci đầu tiên. Chạy `cargo clippy` và sửa mọi gợi ý.

**Bài 2 (10 phút).** Bật Inlay Hints trong editor. Viết `let x = vec![1,2,3].iter().sum();` và xem compiler đòi bạn annotate gì — vì sao nó không tự suy ra được?

**Bài 3 (10 phút).** So sánh thời gian `cargo check` và `cargo build` trên cùng một project. Chênh lệch bao nhiêu lần?

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| rust-analyzer không hoạt động | Không thấy `Cargo.toml` ở root workspace | Mở đúng thư mục chứa `Cargo.toml` |
| Build lần đầu rất lâu | Biên dịch toàn bộ dependency | Bình thường; lần sau đã có cache trong `target/` |
| `target/` chiếm hàng GB | Artifact tích luỹ | `cargo clean` định kỳ; thêm `target/` vào `.gitignore` |
| Clippy báo quá nhiều trên code cũ | Bật hết lint cùng lúc | Bật theo nhóm; `#[allow(...)]` tạm cho từng chỗ |
| `edition` khác nhau gây lỗi lạ | Trộn edition giữa các crate | Thống nhất `edition` trong workspace |
