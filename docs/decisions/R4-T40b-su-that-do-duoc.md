# R4.T4.0b — sự thật đo được về `byte-rust`, trước khi viết bài

**Đo lúc nào:** 2026-09-01, qua `packages/exec-rust` + `apps/byte/public/wasm/byte_rust.wasm`
(rebuild mới nhất từ `crates/byte-rust/src`), bằng script dò trực tiếp gọi
`BoThucThiRust.chay()` — đúng engine mà `tools/kiem_ma_bai_hoc.mjs` và người học
dùng, không phải suy diễn từ tài liệu.

**Vì sao file này tồn tại:** `MASTERPLAN.md` §4.3.2 liệt kê một "HỢP ĐỒNG ĐÓNG" —
danh sách CÓ/CÓ NHƯNG HẸN v1.1/KHÔNG BAO GIỜ cho `byte-rust` v1.0. Danh sách đó là
ĐÍCH NGẮM, không phải TRẠNG THÁI HIỆN TẠI — `ADR-002` tự ghi nhận engine mới có
~5.700/9.000-13.000 dòng ước tính cho v1.0. Đo trực tiếp lộ ra: một số thứ
MASTERPLAN ghi "CÓ" (`if let`/`while let`) hiện KHÔNG chạy — đúng lớp lệch
"sách nói khác runtime" mà `R3-su-that-do-duoc-tren-pyodide.md` đã ghi nhận cho
Pyodide, giờ lặp lại cho Rust. Track T4.0b phải viết theo TRẠNG THÁI THẬT ở đây,
không theo danh sách §4.3.2.

Ba loại kết quả cần phân biệt rạch ròi khi đọc bảng dưới:

- **Dùng được** — chạy đúng như Rust thật, an toàn cho `code` có chấm điểm sống.
- **`ChuaHoTro` (đã đo, HỢP LỆ theo ADR-002)** — engine THÚ NHẬN không đủ sức
  kiểm (thường vì cần phân tích luồng điều khiển/vùng sống). KHÔNG phải lỗi
  của người học. Chỉ dùng được trong bước `predict` (không chạy code), và câu
  trả lời đúng của `predict` phải xác minh bằng `rustc` thật, không bằng
  `byte-rust` (nó tự thú nhận không biết).
- **Lỗi thật của engine (false-reject)** — `byte-rust` từ chối mã Rust HỢP LỆ,
  không phải vì phạm vi hẹp có chủ đích (ADR-002) mà vì cài đặt còn thiếu/sai.
  KHÔNG BAO GIỜ dùng những cấu trúc này trong bất kỳ bước nào của track — kể cả
  `predict`, vì trích dẫn chúng sẽ dạy sai điều gì Rust thật chấp nhận.

## Dùng được — xác nhận chạy đúng, dùng cho `code` có chấm điểm sống

`let`/`let mut`, mọi kiểu số + `String`/`&str`, hàm nhiều tham số + đệ quy,
mảng `[T;N]`/slice `&[T]`/`Vec` (`vec!`, `.push`, `.len`, chỉ số), tuple
**KHÔNG annotate kiểu tường minh** (chỉ suy luận), `for`/`while`/`loop`,
`if`/`else if`/`else` kể cả làm biểu thức, `println!`/`{:?}` (xem lưu ý dưới),
so sánh/logic, `as` cast, `const`, comment — move của `String`/`Vec` (BR0530,
chặn dùng lại sau move thẳng hàng), `Copy` của kiểu số nguyên (không move),
`.clone()`, truyền theo giá trị vào hàm cũng move, `&T`/`&mut T` (mượn đọc/sửa),
hai `&mut` cùng một lời gọi bị chặn (BR0355), `self` theo giá trị (move) vs
`&self` (mượn) trên method — `struct` (named field) + `impl` + method `&self`,
`enum` **chỉ tuple-variant/unit-variant** (xem cấm bên dưới), `match` đầy đủ
nhánh + exhaustiveness check thật (BR0300) + `_` wildcard, destructure tuple
trong đầu `for` **không annotate kiểu Vec chứa tuple** — `Option<T>` + `match` +
`.unwrap()` (panic có kiểm soát trên `None`, BR0551) — `Result<T,E>` + `match` +
`?` — closure **chỉ dạng IIFE hoặc truyền trực tiếp cho `.map()`/`.filter()`**
(xem cấm bên dưới) — `.iter().map().collect()`, `.into_iter().filter().collect()`,
`.iter().sum()` — bảo vệ vòng lặp vô hạn (BR0500, dừng ~64ms, không treo).

**Lưu ý về `{:?}`:** `byte-rust` in được `{:?}` trên MỌI struct mà KHÔNG cần
`#[derive(Debug)]` — khác Rust thật (sẽ từ chối biên dịch nếu thiếu derive, vì
`#[derive(...)]` tự nó không chạy được ở đây, xem mục cấm). Nội dung KHÔNG được
nói "cần derive Debug mới in được" nếu ví dụ minh hoạ chạy trên `byte-rust`
— nói đúng những gì engine này làm, không nói đúng những gì `rustc` làm trừ khi
đã đối chiếu riêng.

## `ChuaHoTro` — hợp lệ theo ADR-002, CHỈ dùng trong bước `predict`

Cả ba đều đã đối chiếu với `rustc` thật (xem "Xác minh bằng rustc" bên dưới):

- **BR0531** — move một binding bên trong THÂN `if`, dùng lại SAU khối `if`.
  `rustc` thật: **TỪ CHỐI** (E0382) — xác minh bằng file `.rs` biên dịch thật.
- **BR0532** — move bên trong THÂN vòng lặp, dùng lại SAU vòng lặp. Mã lỗi
  RIÊNG với BR0531 (không phải cùng một nhánh code trong engine).
- **BR0310** — `.iter().filter(|x| **x ...)` (idiom tham chiếu kép chuẩn Rust
  cho closure nhận `&&T` từ `.iter()`).
- **Mượn `&`/`&mut` xen kẽ qua NHIỀU câu lệnh (NLL)** — byte-rust không tự báo
  mã cụ thể cho ca này trong khảo sát, nhưng đây CHÍNH là lớp "reborrow/NLL"
  ADR-002 nêu là không quyết định được trên AST. Đã tự xác minh bằng `rustc`
  thật (xem bảng dưới) — dùng cho bài `predict` về NLL.

## CẤM TUYỆT ĐỐI — lỗi thật của engine, không phải phạm vi hẹp có chủ đích

Từng cái đã đối chiếu với mã nguồn `crates/byte-rust/src/tyck.rs` (không chỉ
đoán từ hành vi) — đây là lỗi CÀI ĐẶT, không phải giới hạn ADR-002 công nhận:

1. **`#[derive(...)]`** — cú pháp attribute không được hỗ trợ, lỗi cú pháp cứng
   (BR0001) ngay ở ký tự `#`.
2. **Annotate kiểu tuple tường minh** (`let t: (i32,String) = ...`, hay
   `Vec<(i32,i32)>`) — luôn báo lỗi kiểu sai (BR0303) dù giá trị đúng 100%.
3. **`enum` struct-variant** (`Variant { field: T }`) lúc khởi tạo — luôn báo
   BR0321 ("là biến thể dạng tuple, không phải struct"), bất kể hình dạng thật
   khai trong `enum`. Đối chiếu `tyck.rs` dòng ~356-370/~1807: hình dạng thật
   của variant không được kiểm khi nạp payload — lỗi cấu trúc, không sửa được
   bằng cách viết khác.
4. **`if let`/`while let`** — cả hai lỗi BR0113 ("gặp `let` thay vì giá trị"),
   cùng nguyên nhân (không hỗ trợ `let` làm biểu thức).
5. **Pattern `&x` (dereference) trong `for`/`match`** — BR0108 ("gặp `&` thay vì
   một mẫu").
6. **`.expect(...)`** trên `Option`/`Result` — BR0552, không tồn tại trên cả
   hai kiểu bất kể giá trị bên trong. Dùng `.unwrap()` thay thế.
7. **Gọi closure qua biến đặt tên** (`let f = |x| x+1; f(5)`) — BR0333 ("không
   tìm thấy hàm tên `f`"), LUÔN LUÔN. Đối chiếu `tyck.rs` dòng ~671-707: cú
   pháp gọi `ten(...)` chỉ tra bảng hàm thật, biến closure rơi vào nhánh "tên
   hàm lạ". Chỉ IIFE hoặc truyền trực tiếp cho `.map()`/`.filter()` mới chạy.

## Xác minh bằng `rustc` thật (không chỉ tin ADR-002 nói suông)

Hai ca `ChuaHoTro` dễ gây hiểu lầm nhất — "Byte không kiểm được" không có
nghĩa "không có câu trả lời đúng"; `rustc` LUÔN có câu trả lời, chỉ là
`byte-rust` không tính được nó trên AST. Biên dịch thật bằng
`rustc --edition 2021` (không cần cargo, một file):

```rust
// move trong thân if, dùng sau if — rustc: TỪ CHỐI (E0382)
fn main() {
    let s = String::from("hi");
    if true {
        let t = s;
        println!("{}", t);
    }
    println!("{}", s);   // lỗi: s đã bị move ở nhánh trên
}
```

```rust
// mượn & rồi &mut, & DÙNG XONG trước khi &mut xuất hiện — rustc: CHO PHÉP (NLL)
fn main() {
    let mut x = 5;
    let r = &x;
    println!("{}", r);   // r dùng LẦN CUỐI ở đây — vùng sống của r kết thúc
    let m = &mut x;       // nên &mut này không xung đột
    *m += 1;
    println!("{}", x);
}
```

```rust
// mượn & rồi &mut, & CÒN DÙNG SAU khi &mut đã sửa — rustc: TỪ CHỐI (E0502)
fn main() {
    let mut x = 5;
    let r = &x;
    let m = &mut x;       // xung đột: r vẫn còn "sống" (còn dùng ở dòng dưới)
    *m += 1;
    println!("{} {}", r, x);   // lỗi: r đã bị mượn khả biến chồng lên
}
```

Ba ví dụ này (đã biên dịch thật, không suy diễn) là nền cho các bài `predict`
về NLL/luồng điều khiển trong T4.0b — bài `predict` được PHÉP dạy đúng những gì
`byte-rust` không dạy được, miễn câu trả lời đến từ `rustc` thật, không phải
suy đoán.

## Cho người viết bài sau này (T4.1+, T4.4 Generics/Traits, T4.6 Parser combinators)

Danh sách CẤM ở trên áp dụng cho MỌI track dùng `byte-rust`, không riêng
T4.0b — nếu một track sau cần `#[derive(...)]` hay closure đặt tên, đó là dấu
hiệu cần vá `crates/byte-rust` trước, không phải viết bài né tránh thêm một
lần nữa. Ghi vào đây thay vì để mỗi track tự khám phá lại từ đầu.
