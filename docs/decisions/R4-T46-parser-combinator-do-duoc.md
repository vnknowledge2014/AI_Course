# R4.T4.6 — sự thật đo được về closures-là-giá-trị và branch-sensitive move trong `byte-rust`, và quyết định hoãn track

**Đo lúc nào:** 2026-09-02, qua `packages/exec-rust` + `apps/byte/public/wasm/byte_rust.wasm`
(cùng WASM hiện hành, cùng cách đo đã dùng cho `R4-T44-generics-traits-do-duoc.md`),
bằng `rustprobe.mjs` gọi thẳng `BoThucThiRust.chay()` — đúng engine mà
`tools/kiem_ma_bai_hoc.mjs` và người học dùng.

**Vì sao file này tồn tại:** Chuẩn bị thiết kế xương T4.6 ("Parser combinators
& interpreter", 38 bài, Rust — MASTERPLAN §1271), đúng kỷ luật "đo trước khi
viết". T4.6 KHÔNG phụ thuộc trực tiếp vào generics/traits (lý do T4.4 hoãn) —
nhưng đo trực tiếp lộ ra **HAI lỗ hổng hạ tầng ĐỘC LẬP, mỗi lỗ hổng ĐỦ để tự nó
chặn đứng track**, không liên quan gì tới trait bound.

## Kết luận: T4.6 bị HOÃN, không thiết kế xương lúc này

Hai chiến lược cài đặt "parser + interpreter" trong Rust đều CHẠM một giới hạn
hạ tầng KHÁC NHAU:

1. **Parser combinator (phong cách hàm — closure là giá trị, hàm bậc cao ghép
   closure)**: closure gán vào biến rồi GỌI QUA TÊN BIẾN không hoạt động —
   `goi_ham` trong `interp.rs` chỉ tra bảng hàm `fn` khai báo tên khi biểu thức
   gọi là một `DuongDan` (đường dẫn/tên bare), KHÔNG kiểm biến cục bộ có giữ
   closure hay không. Cộng thêm: không có CÚ PHÁP KIỂU nào để khai tham số hàm
   là "một closure" — `fn(T) -> U` (con trỏ hàm), `impl Fn(T) -> U`, và
   `F: Fn(T) -> U` (generic bound, đã biết hỏng từ T4.4) đều KHÔNG parse được.
   Không có cách viết một hàm NHẬN một closure làm tham số. Combinator (hàm
   NHẬN parser, TRẢ parser) là chính nội dung cốt lõi track định dạy — không
   có tập con nào né được giới hạn này.
2. **Recursive-descent (phong cách thủ tục — hàm đệ quy đặt tên gọi lẫn nhau,
   KHÔNG cần closure)**: hoạt động cho phần "tokenize số, đệ quy xuống cây" cơ
   bản (xem kịch bản 8 trong bảng — CHẠY ĐÚNG). Nhưng bộ kiểm mượn/di chuyển
   (`move_check.rs`/`borrow_check.rs`) từ chối THÚ NHẬN ("chưa hỗ trợ",
   `BR0531`) bất kỳ biến non-Copy nào khai báo TRƯỚC một `if`/`else` mà bị
   CHẠM (đọc field, clone, hay move — cả ba đều bị) ở NHIỀU HƠN MỘT nhánh —
   dù thao tác đó AN TOÀN THẬT (đúng luật Rust, `rustc` chấp nhận). Đây CHÍNH
   XÁC là khuôn "phân tích cái đã có, rồi hoặc MỞ RỘNG nó hoặc TRẢ NGUYÊN nó"
   — khuôn bắt buộc phải có ở BẤT KỲ recursive-descent parser thật nào (ví dụ:
   parse một số hạng; nếu theo sau là `+` thì GHÉP nó vào một cây lớn hơn, nếu
   không thì TRẢ nguyên số hạng đó). "Chưa hỗ trợ" khiến mã KHÔNG CHẠY ĐƯỢC
   (`ok: false`, `xuat: ""`) — tác động THỰC TẾ giống hệt một lỗi cứng đối với
   một bài `code` cần `tier: run`/`tests` — không phải một cảnh báo vô hại.

Không tìm được cách viết lại tránh CẢ HAI giới hạn mà vẫn còn là một parser
recursive-descent CÓ Ý NGHĨA (mọi cách né đều rơi vào "không dùng biến chung
qua nhánh nào cả" — nghĩa là không có state trung gian nào tái sử dụng được,
phá vỡ chính hình dạng của một parser thật). Quyết định: **không viết xương
T4.6 cho tới khi hạ tầng dưới đây được bổ sung**.

## Bảng đo chi tiết (17 kịch bản, mã nguồn đầy đủ giữ trong log phiên đo —
tóm tắt kết quả dưới đây)

| # | Kịch bản | Kết quả | Ghi chú |
|---|---|---|---|
| 1 | `let add_one = \|x: i32\| x + 1; add_one(5)` | **BR0333** — không tìm thấy hàm `add_one` | Closure PARSE được (không lỗi ở dòng `let`), nhưng GỌI qua tên biến thất bại — `goi_ham` chỉ tra bảng `fn`, không tra biến cục bộ, khi biểu thức gọi là `DuongDan` |
| 2 | `let add_n = move \|x: i32\| x + n; add_n(5)` | **BR0109** lỗi cú pháp ("thiếu `;`") | `move` closure CÓ tham số kiểu — cú pháp khác lỗi, KHÔNG parse được luôn (dừng ngay ở `\|x: i32\|`) |
| 3 | `\|x: i32\| -> i32 { x + 1 }` (closure có kiểu trả về tường minh) | **BR0113** lỗi cú pháp ("cần một giá trị", gặp `->`) | `be_quan()` trong `parser.rs` không xử lý `->` sau danh sách tham số — chỉ gọi thẳng `self.bieu_thuc()` |
| 4 | `(add_one)(5)` (gọi qua ngoặc đơn, né `DuongDan` trực tiếp) | **BR0333** — vẫn không tìm thấy | Ngoặc đơn không đổi việc biểu thức bên trong vẫn là `DuongDan`, cùng lỗi #1 |
| 5 | `fn apply(f: fn(i32) -> i32, x: i32) -> i32` (kiểu con trỏ hàm) | **BR0106** lỗi cú pháp ("cần một kiểu", gặp `fn`) | Không có kiểu con trỏ hàm nào parse được ở vị trí tham số |
| 6 | `fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32` | **BR0106** lỗi cú pháp ("cần một kiểu", gặp `impl`) | `impl Trait` ở vị trí kiểu tham số không parse được |
| 7 | `struct Wrapper { f: i32 }` rồi `w.f` | ✅ **CHẠY ĐÚNG** | Struct field ĐỌC bình thường — không liên quan closure, xác nhận struct cơ bản không hỏng |
| 8 | Tokenize số bằng ĐỆ QUY thuần (không `while`, không closure): `parse_chu_so_tu` đệ quy tích luỹ | ✅ **CHẠY ĐÚNG** khi ĐỨNG MỘT MÌNH | Xác nhận: đệ quy + `Vec<char>` + so sánh `char` bằng literal (`'0'`..`'9'`) hoạt động tốt — nền tảng recursive-descent CƠ BẢN ổn |
| 9 | `char` có phương thức `.is_ascii_digit()` không | **BR0552** — method không tồn tại | `char` chỉ có `clone`/`to_string` — phải so sánh bằng literal `'0'`/`'9'` trực tiếp, không dùng `as u8 as char` (kịch bản kế) |
| 10 | So sánh `chars[i] >= (0x30 as u8 as char)` | **BR0520** — không so sánh được `char` với `i64` | Chuỗi cast `as u8 as char` không cho ra giá trị kiểu `char` thật — PHẢI dùng literal `'0'` trực tiếp (kịch bản 8 dùng cách này, chạy đúng) |
| 11 | `while j < len && ...digit... { j = j + 1 }`, `j` dùng SAU vòng lặp | **BR0532** "chưa hỗ trợ" — chuyển `j` ra khỏi vòng lặp | Bộ kiểm mượn từ chối phân tích luồng điều khiển của `while` khi biến bên trong bị dùng lại sau — TRÁNH ĐƯỢC bằng cách viết lại thành đệ quy thuần (kịch bản 8) |
| 12 | Parser đầy đủ: `parse_so` rồi `if trai.vi_tri < ... { let phai = ...; KetQuaParse{ cay: ..Box::new(trai.cay).., ..} } else { trai }` | **BR0310 + 2×BR0531** "chưa hỗ trợ" | Nhánh `if` trả kiểu tuple/struct lệch (BR0310, xem kịch bản 13), VÀ `trai` bị CHẠM khác nhau ở hai nhánh (BR0531) |
| 13 | `if la_chu_so(...) { đệ_quy(...) } else { (tich_luy, i) }` — CẢ HAI nhánh trả CÙNG kiểu `(i32, usize)` | **BR0310** "chưa hỗ trợ" — "hai nhánh if lệch kiểu" | Bộ kiểm kiểu không unify được kiểu trả về của lời gọi ĐỆ QUY với kiểu literal tuple ở nhánh kia, dù thực sự CÙNG kiểu |
| 14 | `struct{x,y}` trả trực tiếp `a`/`c` (biến CHỈ dùng ở MỘT nhánh riêng) từ `if`/`else` | ✅ **CHẠY ĐÚNG** | Xác nhận: trả biến sở hữu từ MỘT nhánh (không chia sẻ giữa hai nhánh) hoạt động bình thường |
| 15 | `enum Trang::Bat(n)` / `Trang::Tat` xây trực tiếp trong `if`/`else` (không biến trung gian chia sẻ) | ✅ **CHẠY ĐÚNG** | Cùng kết luận #14 cho enum — construct MỚI trong từng nhánh, không tái dùng biến, luôn ổn |
| 16 | `let x = moi(...); if b { Cap{a:99, b: x.b} } else { x }` — `x` bị CHẠM (partial move + whole move) ở HAI nhánh | **BR0531** "chưa hỗ trợ" | Trọng tâm phát hiện: MỘT biến non-Copy khai TRƯỚC `if`, chạm ở ≥2 nhánh (dù an toàn thật) → checker từ chối |
| 17 | Y hệt #16 nhưng thay move bằng `.clone()` ở CẢ HAI nhánh (không hề move `x`) | **BR0531** "chưa hỗ trợ" (vẫn) | Xác nhận: KHÔNG PHẢI vấn đề an toàn move — chỉ cần "biến bị chạm ở ≥2 nhánh" là bị từ chối, kể cả khi thao tác chỉ là ĐỌC/CLONE, không hề chuyển quyền sở hữu |

## Tóm tắt CÓ/KHÔNG cho ai quay lại làm hạ tầng

**Dùng được ngay** (không cần sửa gì): hàm đệ quy thuần gọi lẫn nhau (không
`while`, không closure) — kịch bản 8; so sánh `char` bằng LITERAL trực tiếp
(`c >= '0' && c <= '9'`), không dùng `as u8 as char`; struct/enum ĐỆ QUY qua
`Box` (đã biết từ T4.3); TRẢ MỚI một giá trị từ MỘT nhánh `if`/`else` (không
chia sẻ biến giữa các nhánh) — kịch bản 14, 15; closure truyền TRỰC TIẾP (nội
tuyến, không gán tên) cho các method built-in như `.map()`/`.filter()` (đã
dùng thành công xuyên suốt T4.0b, xem `content/lap-trinh-ham/01-cong-rust/
22-map-filter-sum-qua-iterator.lesson.md`).

**Cần cài trước khi mở lại T4.6** (ước tính KHÔNG có sẵn trong MASTERPLAN,
cần đo lại quy mô khi quay lại việc này):
1. **`goi_ham` phải tra biến cục bộ giữ closure** khi biểu thức gọi là một
   `DuongDan` MỘT-ĐOẠN không khớp `fn`/hàm dựng nào — hiện tại nhánh
   `DuongDan` LUÔN return lỗi BR0550 trước khi bao giờ thử "gọi closure giữ
   trong biến" (nhánh đó tồn tại ở cuối `goi_ham`, nhưng KHÔNG BAO GIỜ chạy
   tới vì nhánh `DuongDan` return sớm).
2. **Cú pháp kiểu cho "một closure"**: hoặc `fn(T) -> U` (con trỏ hàm, đơn
   giản nhất — không cần generic/trait), hoặc `impl Fn(T) -> U`, để một hàm
   tự viết CÓ THỂ khai tham số kiểu closure.
3. **`be_quan()` (parser.rs) phải xử lý `-> Kieu` tuỳ chọn** sau danh sách
   tham số closure, trước khi gọi `self.bieu_thuc()`.
4. **Bộ kiểm kiểu unify kiểu trả về CỦA lời gọi đệ quy với literal cùng hình
   dạng** ở hai nhánh `if`/`else` (kịch bản 13) — hiện coi hai nhánh "lệch
   kiểu" dù cùng kiểu `(i32, usize)`.
5. **Bộ kiểm mượn/di chuyển phân tích ĐƯỢC "biến bị chạm ở nhiều nhánh
   loại-trừ-lẫn-nhau"** (mutually exclusive branches) — đây là hạng mục LỚN
   NHẤT, vì hiện tại NÓ TỪ CHỐI TOÀN BỘ lớp chương trình "tính một lần, dùng
   khác nhau tuỳ nhánh sau", không riêng gì parser. Việc này rất có thể chạm
   TỚI kiến trúc lõi của `move_check.rs`/`borrow_check.rs` (phân tích luồng
   điều khiển theo nhánh), không phải một bản vá nhỏ.

## Xác minh: mọi kịch bản trên đều là Rust hợp lệ thật

Tất cả 9 đoạn mã BỊ TỪ CHỐI trong bảng trên (kịch bản 1-6, 9-13, 16-17) đều
biên dịch/chạy đúng với `rustc` thật — closure gán biến rồi gọi lại, tham số
kiểu `fn(T)->U`/`impl Fn`, và "tính một biến rồi dùng khác nhau tuỳ nhánh
`if`" đều là những khuôn Rust CỰC KỲ phổ biến (không phải cấu trúc hiếm/lạ).
Đây KHÔNG phải trường hợp "học liệu dạy sai Rust thật" — đây là trường hợp
"`byte-rust` chưa cài đủ Rust thật", cùng phân loại `ADR-002` với T4.4 — và
giống T4.4, phạm vi hẹp Ở ĐÂY CHẶN ĐỨNG chính khái niệm track T4.6 định dạy
(closure-là-giá-trị cho combinator; state-tái-dùng-qua-nhánh cho
recursive-descent), không có tập con nào để né tránh mà vẫn còn dạy được
"parser".

## Gợi ý cho lần quay lại Realm 4 sau khi đầu tư hạ tầng

Nếu hạng mục (1)-(3) ở trên được cài trước hạng mục (5) (rẻ hơn nhiều — chỉ
là tra bảng biến + một kiểu cú pháp mới, không đụng kiến trúc phân tích luồng
điều khiển), T4.6 CÓ THỂ mở lại theo hướng **parser combinator ĐƠN GIẢN HOÁ**:
mỗi "parser" là MỘT closure kiểu `fn(&[char], usize) -> Option<(T, usize)>`,
combinator là hàm bậc cao GHÉP hai closure — tránh HOÀN TOÀN vấn đề #5 (không
cần chia sẻ biến qua nhánh, vì mỗi combinator chỉ TRẢ MỘT closure MỚI, không
bao giờ đọc lại một biến cũ theo nhiều cách). Đây SẼ là hướng ưu tiên đo lại
đầu tiên nếu (1)-(3) được đầu tư — rẻ hơn nhiều so với chờ (5) (bộ kiểm mượn
theo nhánh) được viết lại.
