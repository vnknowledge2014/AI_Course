# R4.T4.4 — sự thật đo được về generics/traits trong `byte-rust`, và quyết định hoãn track

**Đo lúc nào:** 2026-09-02, qua `packages/exec-rust` + `apps/byte/public/wasm/byte_rust.wasm`
(WASM MỚI NHẤT — đã kiểm không có file `crates/byte-rust/src/*.rs` nào mới hơn
file WASM này), bằng `rustprobe.mjs` gọi thẳng `BoThucThiRust.chay()` — đúng
engine mà `tools/kiem_ma_bai_hoc.mjs` và người học dùng.

**Vì sao file này tồn tại:** Chuẩn bị thiết kế xương T4.4 ("Generics · Traits ·
Type classes", 36 bài, Rust — MASTERPLAN §1271), đúng kỷ luật "đo trước khi
viết" đã áp dụng cho mọi track trước. `MASTERPLAN.md` §4.3.2 tự ghi các tính
năng này vào mục **"CÓ NHƯNG HẸN v1.1"** (không phải v1.0), và §"sửa ước lượng"
(dòng 290) ước tính v1.1 (trait/generic/dyn) cần thêm **~5.000 dòng** — một hạng
mục kỹ thuật lớn, không phải vài AST-kind như các track trước. Đo trực tiếp bên
dưới XÁC NHẬN LẠI đúng dự đoán đó của MASTERPLAN, không lệch — nhưng cũng lộ ra
một vài chi tiết cụ thể MASTERPLAN không ghi rõ (bất đối xứng generic function
vs. generic struct/impl), cần thiết cho bất kỳ ai quay lại làm hạ tầng này sau.

## Kết luận: T4.4 bị HOÃN, không thiết kế xương lúc này

Trait bound (`<T: Trait>`) — **thành phần cốt lõi** biến "Generics" và "Traits"
thành MỘT khái niệm thống nhất (Rust's câu trả lời cho type class) — **không
parse được ở BẤT KỲ vị trí nào** (hàm, `impl<T: ...>`), không phải lỗi kiểm
kiểu mà lỗi CÚ PHÁP (`BR0100: thiếu dấu >`). `dyn Trait` cũng không parse được,
cùng lỗi `BR0100`. Không có cách nào dạy "generic bị RÀNG BUỘC bởi trait" —
đúng nội dung chính của track — mà không chạm những cấu trúc này.

Xây track chỉ với PHẦN CÒN LẠI (generic struct qua `impl<T>` không ràng buộc,
trait không có default method, generic enum) sẽ là một track "Generics · Traits
· Type classes" không thực sự dạy được "type classes" — phần LÕI của tên track.
Quyết định: **không viết xương T4.4 cho tới khi hạ tầng dưới đây được bổ sung**;
tiếp tục Realm 4 bằng T4.5 (Đại số của chương trình, TypeScript — không phụ
thuộc byte-rust) trước, quay lại T4.4 sau nếu có đầu tư hạ tầng.

## Bảng đo chi tiết (8 kịch bản, mã nguồn đầy đủ giữ trong `rustprobe.mjs` log
của phiên đo — tóm tắt kết quả dưới đây)

| Kịch bản | Mã ví dụ (rút gọn) | Kết quả | Ghi chú |
|---|---|---|---|
| Generic function KHÔNG ràng buộc | `fn f<T>(x: T) -> T { x }` rồi `f(5)` | **BR0301 sai kiểu** — `T` bị coi là một kiểu CỤ THỂ chưa xác định, không unify với `i32` | Lỗi kiểm kiểu (`tyck.rs`), không phải lỗi cú pháp — hàm PARSE được, nhưng gọi nó với BẤT KỲ kiểu cụ thể nào đều bị từ chối |
| Generic function với trait bound | `fn f<T: PartialOrd>(a: T, b: T) -> T` | **BR0100 lỗi cú pháp** ("thiếu dấu `>`") | Parser dừng ngay tại dấu `:` sau tên kiểu — cú pháp ràng buộc chưa được cài |
| Generic struct, khởi tạo trực tiếp | `struct Hop<T> { gia_tri: T }` rồi `Hop { gia_tri: 5 }` | **BR0324** — trường cần `T` nhưng nhận `{số nguyên}` | Cùng gốc bệnh với generic function: `T` bị coi cố định, không unify |
| Generic struct qua `impl<T>` + hàm dựng | `impl<T> Hop<T> { fn moi(gia_tri: T) -> Hop<T> {...} }` rồi `Hop::moi(5)` | ✅ **CHẠY ĐÚNG** | **BẤT ĐỐI XỨNG quan trọng**: generic PARAM trong `impl<T>` unify đúng, generic PARAM của hàm top-level (`fn f<T>`) thì KHÔNG — hai cơ chế cùng cú pháp nhưng hành vi khác hẳn trong `tyck.rs` |
| `impl<T: Bound>` (trait bound trên impl) | `impl<T: Clone> Hop<T> { ... }` | **BR0100 lỗi cú pháp** | Cùng lỗi cú pháp với hàm — không nơi nào chấp nhận `T: Bound` |
| `dyn Trait` | `fn goi(x: &dyn ChaoHoi) -> String` | **BR0100 lỗi cú pháp** ("thiếu dấu `)`") | Parser không nhận `dyn` như một từ khoá hợp lệ ở vị trí kiểu |
| Trait cơ bản: định nghĩa + `impl ... for` + gọi method | `trait ChaoHoi { fn chao(&self) -> String; }` | ✅ **CHẠY ĐÚNG** | Trait KHÔNG generic, KHÔNG default method, method BẮT BUỘC override — hoạt động tốt |
| Trait với default method | thêm `fn chao(&self) -> String { format!(...) }` làm mặc định, `impl` không override | **BR0552** — method không tồn tại | Default method trong khai báo `trait` không được cài; struct không override thì method coi như không có |
| Generic enum có sẵn (`Option`) VÀ tự định nghĩa | `enum TuyChon<T> { Co(T), Khong }` | ✅ **CHẠY ĐÚNG** | Khác generic STRUCT — generic ENUM (kể cả người học tự viết, không chỉ `Option`/`Result` có sẵn) unify đúng. Không rõ tại sao enum khác struct/function trong `tyck.rs`, nhưng đã đo THẬT |
| `Vec<T>` builtin (không phải generic tự viết) | `let v: Vec<i32> = vec![1,2,3]; v.iter().sum()` | ✅ **CHẠY ĐÚNG** (không đổi) | Xác nhận việc trait/generic tự viết hỏng KHÔNG ảnh hưởng gì tới các kiểu generic BUILT-IN đã có (`Vec`/`Option`/`Result`/`HashMap`...) — track T4.0b/T4.1-T4.3 dùng chúng vẫn an toàn |

## Tóm tắt CÓ/KHÔNG cho ai quay lại làm hạ tầng

**Dùng được ngay** (không cần sửa gì): generic ENUM tự viết (`enum X<T> {...}`),
generic STRUCT qua khuôn `impl<T> Struct<T> { fn constructor(...) -> Struct<T> }`
(struct literal trực tiếp KHÔNG dùng được, phải đi qua một hàm dựng), trait
KHÔNG generic + KHÔNG default method + method bắt buộc override, mọi kiểu
generic built-in (`Vec`/`Option`/`Result`/`HashMap`/`Box`...).

**Cần cài trước khi mở lại T4.4** (ước tính ~5.000 dòng theo MASTERPLAN dòng 290):
1. Cú pháp trait bound `<T: Trait>` — cần sửa PARSER trước (`BR0100`), không
   chỉ `tyck.rs` — hiện tại parser dừng ngay ở dấu `:`.
2. Cú pháp `dyn Trait` — cũng là lỗi PARSER (`BR0100`), độc lập với mục 1.
3. Unify generic PARAM của hàm top-level (`fn f<T>`) — hiện `tyck.rs` coi `T`
   là kiểu cố định thay vì biến kiểu cần suy luận từ đối số. Cơ chế unify
   ĐÃ tồn tại và ĐÚNG cho `impl<T> Struct<T>` — có thể tái dùng logic đó thay vì
   viết lại từ đầu (mục tiêu điều tra đầu tiên khi quay lại việc này).
4. Default method trong khai báo `trait` — cần lưu THÂN hàm mặc định và dùng
   nó khi `impl` không override, thay vì coi method không tồn tại (`BR0552`).
5. Sau khi (1)-(4) xong: đo lại TRAIT BOUND CÓ THẬT SỰ RÀNG BUỘC không (ví dụ
   `f<T: PartialOrd>` có ĐÚNG chặn gọi `f` với kiểu KHÔNG impl `PartialOrd`
   không) — chưa đo được vì (1) chặn ngay từ bước parse.

## Xác minh bằng rustc (mọi kịch bản trên đều là Rust hợp lệ thật)

Cả 9 đoạn mã trong bảng trên đều biên dịch/chạy đúng với `rustc` thật (kiểm tra
bằng tri thức ngôn ngữ Rust chuẩn — cú pháp `<T: Bound>`, `dyn Trait`, default
method trong `trait` đều là tính năng Rust CÓ THẬT, không phải cấu trúc bịa).
Đây KHÔNG phải trường hợp "học liệu dạy sai Rust thật" — đây là trường hợp
"`byte-rust` chưa cài đủ Rust thật", đúng phân loại "lỗi thật của engine, phạm
vi hẹp có chủ đích" theo `ADR-002`, chỉ khác track T4.0b (nơi phạm vi hẹp có
CHỦ ĐÍCH và có thể né tránh bằng cách chọn đúng tập con) — ở đây phạm vi hẹp
CHẶN ĐỨNG chính khái niệm track T4.4 định dạy, không có tập con nào để né.
