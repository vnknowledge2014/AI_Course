---
id: lap-trinh-ham.cong-rust.truyen-vao-ham-cung-la-move
title: "Truyền vào hàm theo giá trị CŨNG là move"
summary: "fn in_ra(s: String) {...} in_ra(ten); — gọi hàm CŨNG chuyển quyền sở hữu, ten không dùng lại được sau lời gọi. Đối lập thẳng với Python: truyền một tham chiếu, hàm và người gọi cùng trỏ tới một giá trị, không ai mất quyền gì."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 8
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.move-into-function]
requires: [rs.clone]
concepts: [rs.move-into-function]
gradingMatrix:
  web-chrome: [run, tests, output]
  web-firefox: [run, tests, output]
  macos: [run, tests, output]
  windows: [run, tests, output]
  linux: [run, tests, output]
  android: [run, tests, output]
  ios: [run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Bài trước hỏi: gọi một hàm, truyền một biến làm đối số — Rust xử lý
việc đó có giống move qua `let` không? Câu trả lời: giống hệt.
::::

::::explain{#goi-ham-cung-move}
Ở Python, truyền một biến vào hàm không hề đụng tới quyền gì —
`ham(ten)` chỉ đưa cho hàm MỘT TẤM THẺ KHÁC cùng buộc vào giá trị `ten`
đang giữ (`mem.pass-by-reference`, T3.1 bài cuối). Hàm và người gọi
cùng nhìn vào một giá trị, không ai mất quyền dùng nó sau đó.

Rust không làm vậy khi một tham số nhận kiểu THEO GIÁ TRỊ (viết
`s: String`, không phải `s: &String`) — trường hợp duy nhất bài này
xét tới. Gọi hàm với một biến, khi tham số nhận theo giá trị, hoạt động
ĐÚNG như `let t = s;`: quyền sở hữu CHUYỂN từ biến của người gọi sang
tham số bên trong hàm. Sau lời gọi, biến gốc mất quyền chủ — dùng lại
nó bị chặn bằng đúng `BR0530` đã gặp ở hai bài trước, chỉ khác nơi
quyền sở hữu chuyển tới: không phải một `let` khác, mà là tham số của
hàm.

Đây là một đối lập đáng nhớ: CÙNG một hành động cú pháp — "gọi hàm với
một biến" — cho ra hai luật hoàn toàn khác nhau ở hai ngôn ngữ. Python:
không ai mất quyền. Rust (khi tham số nhận theo giá trị): người gọi
mất quyền, hàm trở thành chủ mới.
::::

::::example{#goi-ham-chuyen-quyen}
Một hàm nhận `String` theo giá trị, gọi nó, rồi thử dùng lại biến gốc:

```rust title=readonly
fn in_ra(s: String) {
    println!("Đang hiển thị: {}", s);
}

fn main() {
    let ten = String::from("Vé số 09");
    in_ra(ten);
    println!("Vẫn còn: {}", ten);
}
```

```text title=readonly
(không biên dịch được)

lỗi [BR0530]: `ten` đã bị chuyển quyền sở hữu đi nơi khác
 --> dòng 9:29
  |
9 |     println!("Vẫn còn: {}", ten);
  |                             ^^^ dùng lại ở đây thì không còn giá trị nữa
  |
 --> dòng 8:11
  |
8 |     in_ra(ten);
  |           --- quyền sở hữu bị chuyển đi tại đây
  |
  vì sao: Rust cho mỗi giá trị đúng MỘT chủ sở hữu. Khi bạn gán nó
  sang biến khác hoặc truyền vào hàm, chủ cũ mất quyền — nhờ luật này
  Rust không cần bộ dọn rác mà vẫn không bao giờ dùng nhầm bộ nhớ đã
  giải phóng.
```

Đọc kỹ dòng "quyền sở hữu bị chuyển đi tại đây" — nó chỉ đích danh
`in_ra(ten);`, một LỜI GỌI HÀM, không phải một dòng `let`. Chẩn đoán
này là bằng chứng trực tiếp: move không chỉ xảy ra ở phép gán, nó xảy
ra ở bất cứ đâu quyền sở hữu đổi tay — kể cả khi "đổi tay" đó là đưa
giá trị vào bên trong một hàm.

Muốn `ten` vẫn còn dùng được sau lời gọi, hai công cụ đã học vẫn áp
dụng nguyên vẹn: gọi `in_ra(ten.clone())` (tốn thêm một bản sao), hoặc
đổi tham số hàm sang nhận MƯỢN thay vì nhận hẳn — công cụ đó là chủ đề
của cụm bài kế tiếp.
::::

::::predict{#ham-nao-move commitOnce}
Byte viết đoạn mã sau, KHÔNG chạy thử:

```rust
fn cong(x: i32, y: i32) -> i32 {
    x + y
}

fn ghep(s: String) -> String {
    format!("{}!", s)
}

fn main() {
    let a = 3;
    let ket_qua = cong(a, 4);
    println!("Tổng: {} (a = {})", ket_qua, a);

    let loi_chao = String::from("Xin chào");
    let loi_chao_moi = ghep(loi_chao);
    println!("{} (loi_chao = {})", loi_chao_moi, loi_chao);
}
```

**Trước khi đọc đáp án**, đoạn mã này có biên dịch được không, và vì
sao?

:::opt{correct}
Không — dòng in cuối dùng lại `loi_chao`, nhưng nó đã move vào hàm
`ghep` ở lời gọi `ghep(loi_chao)` (tham số `s: String` nhận theo giá
trị). Dòng in `a` không sao, vì `i32` là `Copy` — truyền `a` vào
`cong(a, 4)` chỉ tạo bản sao, `a` không hề move
:::

:::opt
Không — cả hai dòng in cuối đều bị chặn, vì gọi hàm luôn move đối số
truyền vào, không phân biệt kiểu dữ liệu
::why
Gần đúng ở việc bạn áp dụng đúng luật "gọi hàm theo giá trị thì move" —
luật đó có thật cho `ghep(loi_chao)`.

Chỗ lệch: luật move chỉ áp dụng cho kiểu KHÔNG `Copy`. `i32` là `Copy`
(đã học từ bài "dùng lại sau move bị chặn") — truyền `a` vào
`cong(a, 4)` tạo một bản sao, không move gì cả. Dòng in `a` sau đó hoàn
toàn hợp lệ; chỉ dòng in `loi_chao` bị chặn thật.
::
:::

:::opt
Có, biên dịch được — `ghep` chỉ ĐỌC tham số `s` để ghép thêm dấu `!`,
không sửa gì cả, nên `loi_chao` không mất quyền chủ
::why
Gần đúng ở việc bạn để ý `ghep` quả thật không sửa nội dung `s`, chỉ
tạo một chuỗi mới từ nó qua `format!`.

Chỗ lệch: luật move không quan tâm hàm có SỬA tham số hay không — nó
chỉ nhìn vào CHỮ KÝ tham số. `s: String` (không có `&`) nhận theo giá
trị, nên lời gọi luôn move, bất kể thân hàm làm gì bên trong. Dòng in
`loi_chao` sau lời gọi vẫn bị chặn thật, đúng mã `BR0530`.
::
:::

:::opt
Không — nhưng lỗi nằm ở dòng `let loi_chao_moi = ghep(loi_chao);`, vì
không thể gọi một hàm trả về `String` rồi gán ngay cho một tên khác
::why
Gần đúng ở việc bạn nghi ngờ đúng khu vực có vấn đề — dòng gọi hàm liên
quan tới `loi_chao`.

Chỗ lệch: gán kết quả một hàm trả về `String` cho một tên là thao tác
hoàn toàn bình thường, không có luật nào cấm — dòng đó biên dịch sạch.
Vấn đề không nằm ở việc GÁN kết quả, mà ở việc dùng lại `loi_chao` (đối
số đã move đi) tại dòng in CUỐI CÙNG.
::
:::
::::

::::code{#mo-ta-ve}
Một hàm nhận một mã vé (kiểu `String`) theo giá trị, trả về một câu mô
tả có chứa đúng mã đó. Điền chỗ trống để hàm nhúng đúng giá trị tham số
vào chuỗi kết quả.

```rust title=starter
fn mo_ta(ve: String) -> String {
    format!("Đang hiển thị: {}", ___)
}
```

```rust title=solution
fn mo_ta(ve: String) -> String {
    format!("Đang hiển thị: {}", ve)
}
```

```rust title=test
fn main() {
    let ve = String::from("Vé số 09");
    let mo_ta_ve = mo_ta(ve);
    println!("{}", mo_ta_ve);
    assert_eq!(
        mo_ta_ve, "Đang hiển thị: Vé số 09",
        "mo_ta phải nhúng đúng giá trị của tham số ve vào chuỗi trả về"
    );
}
```

:::hints
- kind: attention
  body: Chỗ trống nằm trong thân hàm mo_ta, ở vị trí đối số của format!. Nó phải nhúng đúng giá trị tham số hàm nhận được, không phải một chuỗi tự viết tay.
- kind: strategy
  body: Tham số duy nhất của mo_ta tên là ve — dùng đúng cái tên đó trong format!.
- kind: one-line
  body: 'Chỗ trống là: ve'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Đang hiển thị: Vé số 09"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một luật move, dù nó xảy ra qua `let` hay qua một lời gọi hàm.
Người gọi mất quyền, hàm trở thành chủ mới.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Từ đầu track tới giờ, luật move luôn xuất hiện trên một chuỗi dòng lệnh
THẲNG HÀNG — không rẽ nhánh, không vòng lặp. Nếu việc move xảy ra bên
TRONG một khối `if`, rồi bạn dùng lại biến đó SAU khối `if` — Rust có
còn kiểm tra được chuyện đó rõ ràng như đã thấy không, hay có gì đó
khác đi?

Bài sau trả lời — và câu trả lời không đơn giản như bạn nghĩ.
::::

::::checkpoint{mastery=0.8}
::::
