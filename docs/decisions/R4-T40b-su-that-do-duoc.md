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

## Cập nhật 2026-09-03 — điều tra trước R6-2 q07 "Ngôn ngữ của Byte"

Đo trực tiếp qua `cargo run --example` (crates/byte-rust, tạo file tạm rồi
xoá) khi kiểm tính khả thi của lexer/parser đệ quy — chủ đề q07-q09.

### ĐÃ VÁ — không còn là false-reject

- **`&T` (tham chiếu BẤT BIẾN) dùng ở nhiều nhánh trong hàm đệ quy** — trước
  đây LUÔN báo nhầm `BR0531` dù không cần phân tích luồng điều khiển (xem
  commit `fix(byte-rust): tham chiếu bất biến không còn bị move_check báo
  nhầm trong nhánh`). Từ nay: hàm đệ quy nhận `&T`/`&self`/`&Vec<T>` và dùng
  tham số đó ở CẢ nhánh cơ bản lẫn nhánh đệ quy — **dùng được bình thường**,
  không cần workaround. `&mut T` KHÔNG đổi (đúng luật thật, không phải Copy).

### CẤM/CHUA_HO_TRO mới phát hiện — dùng cho track R6 (lexer/parser/AST)

1. **`Box::new(...)`** — CHƯA cài Ở `interp.rs` (không có nhánh dispatch cho
   `("Box", "new")`, khác với `("Vec","new")`/`("String","new"/"from")` đã
   có). Hệ quả: **kiểu cây đệ quy cổ điển `enum Node { ..., Con(Box<Node>) }`
   KHÔNG dùng được.** Track R6 cần AST/cây PHẢI dùng **kiểu arena/index**:
   `struct Cay { nut: Vec<NodeEnum> }`, con trỏ LÀ `usize` chỉ vào `nut`,
   KHÔNG BAO GIỜ `Box`. Đây LÀ một lựa chọn thiết kế Rust thật hợp lệ (nhiều
   parser thật, ví dụ rust-analyzer, cũng dùng arena thay Box đệ quy để né
   đúng vấn đề mượn/vòng đời mà cây Box gây ra) — không phải một "giả vờ",
   ghi rõ lý do này trong lesson giới thiệu AST của track.
2. **`String` — CHỈ đọc, KHÔNG có phương thức mutate.** `.push`/`.push_str`
   không tồn tại (`BR0552`, danh sách gợi ý của engine: `len, to_uppercase,
   to_lowercase, trim, chars, split, contains`). Muốn "xây" một chuỗi từ ký
   tự rời: dùng `Vec<char>` (CÓ `.push`, CÓ so sánh `==` TRỰC TIẾP giữa hai
   `Vec<char>`, CÓ index + `.len()`) thay cho `String` xuyên suốt phần lexer
   — chỉ chuyển sang `String` (qua `String::from(...)`) ở BIÊN NGOÀI khi cần
   in kết quả.
3. **`.iter().collect()` từ `Vec<char>` KHÔNG suy luận ra được kiểu `String`**
   dù `let x: String = ...` khai tường minh — báo lỗi kiểu sai `BR0303`
   ("giá trị là Vec<char>"). Không có "target-type-driven" overload cho
   `collect()` — nó luôn suy `Vec<char>`. TRÁNH pattern này; dùng `Vec<char>`
   trực tiếp làm biểu diễn "chuỗi" (xem mục 2).
4. **Nối chuỗi bằng `+` và tự-gán lại BÊN TRONG một vòng lặp** (kiểu
   accumulator: `s = s + &x;` trong `while`/`for`) — rơi vào `ChuaHoTro`
   `BR0532` giống hệt move-trong-vòng-lặp thường, dù trong Rust thật đây LÀ
   idiom hoàn toàn hợp lệ (NLL biết `s` "sống lại" ngay sau phép gán). Đây
   LÀ giới hạn CFG đã biết (ADR-002), không phải lỗi cài đặt — dùng
   `predict` nếu cần dạy khái niệm này, KHÔNG dùng `code` chấm điểm sống.
5. **`char` hầu như không có method dựng sẵn** — chỉ `clone`, `to_string`.
   KHÔNG có `is_ascii_digit`/`is_alphabetic`/... Thay bằng SO SÁNH ký tự trực
   tiếp — **dùng được, sạch**: `c >= '0' && c <= '9'` (chữ số),
   `(c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'` (chữ/gạch
   dưới). Xác nhận: char hỗ trợ đầy đủ `==`/`!=`/`>=`/`<=` so với char khác.
6. **Slice bằng range trên `Vec`** (`v[1..4]`, kể cả chain `.iter().collect()`
   sau đó) — `ChuaHoTro` `BR0310` Ở TẦNG KIỂU, và vì lỗi tĩnh chặn TOÀN BỘ
   chương trình (không chỉ dòng đó) trước khi chạy dòng nào — **TRÁNH hoàn
   toàn** trong `code` có chấm điểm sống. Duyệt range bằng vòng `while`/`for`
   với chỉ số tường minh (`i` chạy từ `bat_dau` tới `ket_thuc`) thay thế.
7. **`&mut T` + gán qua CHỈ SỐ** (`v[i] += 1` với `v: &mut Vec<T>` tham số)
   — báo nhầm `BR0400` ("không gán lại được cho `v`") từ `mut_check.rs`,
   dường như đòi tham số phải khai `mut v` dù đang mutate QUA tham chiếu
   chứ không reassign chính `v`. **CHƯA điều tra/vá** (khác lỗ hổng move_
   check đã vá ở trên — nằm ở file khác). TRÁNH pattern `&mut T` + đệ quy +
   gán qua chỉ số trong thiết kế bài — nếu q08/q09 (executor mutate cây)
   cần pattern này, điều tra/vá riêng lúc đó, đừng suy luận lại từ đầu.

### Dùng được, xác nhận thêm (không có trong bảng gốc ở trên)

`s.chars().collect()` → `Vec<char>` (dùng được, đã xác nhận cho lexer).
`vec![a, b, c]` với phần tử `char`. In `Vec<char>` từng ký tự qua vòng lặp +
`print!("{}", c)` (không cần `{:?}` trên cả Vec). So sánh hai `Vec<char>`
bằng `==` cho kết quả đúng (so từng phần tử, đúng ngữ nghĩa `PartialEq` thật
của `Vec`). `enum` payload kiểu `usize` (dùng làm chỉ số arena) — dùng được
y hệt các kiểu số khác đã xác nhận.
