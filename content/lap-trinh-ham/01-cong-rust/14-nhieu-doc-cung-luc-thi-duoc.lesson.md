---
id: lap-trinh-ham.cong-rust.nhieu-doc-cung-luc-thi-duoc
title: "Nhiều `&` (đọc) cùng lúc thì ĐƯỢC — chỉ `&mut` mới độc quyền"
summary: "`fn cong(a: &i32, b: &i32) -> i32 { a + b } cong(&x, &x);` — HAI tham chiếu ĐỌC tới cùng một chỗ, cùng lúc, chạy được (đã đo thật) — đối lập trực tiếp bài trước. Luật thật của Rust không phải \"không được mượn hai lần\", mà là \"nhiều người ĐỌC cùng lúc thì được, nhưng ĐÚNG MỘT người SỬA tại một thời điểm\"."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 14
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.multiple-shared-ok]
requires: [rs.double-mut-blocked]
concepts: [rs.multiple-shared-ok]
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
Hai `&mut` tới cùng một chỗ, cùng lúc — bị chặn. Hai `&` (chỉ đọc)
tới cùng một chỗ, cùng lúc — chuyện hoàn toàn khác.
::::

::::explain{#doc-thi-khong-doc-quyen}
Bài trước, `cong(&mut x, &mut x)` bị Rust từ chối — hai người CÙNG có
quyền SỬA một chỗ, cùng lúc, là điều Rust không bao giờ cho phép, vì
không ai đoán được kết quả cuối thuộc về bên nào.

Đổi hàm đó sang chỉ ĐỌC — `&i32` thay vì `&mut i32`:

```
fn cong(a: &i32, b: &i32) -> i32 {
    a + b
}
```

Gọi với cùng một biến ở cả hai vị trí:

```
let x = 5;
let tong = cong(&x, &x);
```

Lần này KHÔNG có gì bị chặn. Lý do nằm ngay trong câu hỏi Rust đặt ra
mỗi lần thấy nhiều tham chiếu cùng trỏ một chỗ: "có ai trong số đó
ĐANG SỬA không?" Nếu tất cả chỉ ĐỌC, không ai thay đổi gì cả — không
có thứ tự nào để tranh cãi, không có kết quả nào mơ hồ, vì đọc bao
nhiêu lần cũng ra đúng một giá trị y hệt nhau. Bao nhiêu người đọc
cùng lúc cũng an toàn.

Luật thật của Rust — không phải "không được mượn cùng một chỗ hai
lần" như bài trước có thể khiến bạn nghĩ, mà chính xác hơn:

> **Nhiều người ĐỌC cùng lúc thì được. Nhưng tại một thời điểm, ĐÚNG
> MỘT người được SỬA — và không được vừa có người đọc vừa có người
> sửa cùng lúc.**

Bài trước minh hoạ nửa sau của luật này (SỬA thì độc quyền). Bài này
minh hoạ nửa đầu (ĐỌC thì không giới hạn số lượng).
::::

::::example{#nhieu-doc-cung-mot-cho}
Ba tham chiếu đọc, cùng trỏ vào một biến, cùng lúc, trong cùng một
lời gọi:

```rust title=readonly
fn ba_so(a: &i32, b: &i32, c: &i32) -> i32 {
    a + b + c
}

fn main() {
    let x = 4;
    let tong = ba_so(&x, &x, &x);
    println!("{}", tong);
}
```

```text title=readonly
12
```

Ba tham chiếu, một biến, một lời gọi — không một cảnh báo nào. So
sánh trực tiếp với hình dạng gần giống hệt của bài trước:

```rust title=readonly
fn cong(a: &mut i32, b: &mut i32) {
    *a += *b;
}

fn main() {
    let mut x = 5;
    cong(&mut x, &mut x);
}
```

```text title=readonly
(không biên dịch được)

lỗi [BR0355]: mượn `x` dạng `&mut` hai lần cùng lúc
```

Khác biệt duy nhất giữa hai chương trình là `&` với `&mut`. Cùng một
biến, cùng lúc, cùng một lời gọi — nhưng một bên là ba người ĐỌC
(an toàn), bên kia là hai người SỬA (không ai đoán được kết quả).
::::

::::predict{#so-sanh-bang-tham-chieu commitOnce}
Byte viết một hàm so sánh hai tham chiếu, KHÔNG chạy thử:

```rust
fn so_sanh(a: &i32, b: &i32) -> bool {
    a == b
}

fn main() {
    let x = 9;
    let ket_qua = so_sanh(&x, &x);
    println!("{}", ket_qua);
}
```

**Trước khi đọc đáp án**, đoạn mã này có biên dịch được không, và
dòng cuối in ra gì?

:::opt{correct}
Có, biên dịch được — `&x` và `&x` đều là tham chiếu ĐỌC tới cùng một
chỗ, cùng lúc thì hợp lệ. `a == b` so sánh giá trị đằng sau hai tham
chiếu (9 và 9), in ra `true`
:::

:::opt
Không biên dịch được — `so_sanh(&x, &x)` giống hệt tình huống hai
`&mut` cùng lúc ở bài trước, chỉ khác chữ `mut` nên vẫn bị chặn cùng
lý do
::why
Gần đúng ở việc bạn nhận ra hình dạng lời gọi RẤT GIỐNG ví dụ bị
chặn ở bài trước — hai tham chiếu, cùng một biến, cùng một lời gọi.

Chỗ lệch: chính chữ `mut` đó mới là điều quyết định, không phải hình
dạng lời gọi. Luật của Rust phân biệt RÕ giữa "ai đó có quyền sửa"
và "ai đó chỉ đọc". `&i32` (không `mut`) không cho quyền sửa gì cả —
nhiều tham chiếu `&i32` cùng trỏ một chỗ, cùng lúc, không hề mơ hồ,
vì không ai thay đổi gì. Đã đo thật: biên dịch và chạy sạch.
::
:::

:::opt
Có, biên dịch được, nhưng in ra `false` — `a` và `b` tuy cùng trỏ
vào `x`, nhưng là HAI tham chiếu khác nhau, nên `a == b` so sánh ĐỊA
CHỈ của chúng, không phải giá trị bên trong, và hai tham chiếu không
bao giờ có cùng địa chỉ
::why
Gần đúng ở việc bạn để ý `a` và `b` là hai THAM CHIẾU riêng biệt — về
mặt cú pháp đúng là hai cái tên khác nhau.

Chỗ lệch: `==` trên `&i32` không so sánh địa chỉ tham chiếu — nó tự
động đi XUYÊN QUA tham chiếu để so sánh GIÁ TRỊ bên dưới, giống hệt
việc bạn so sánh `9 == 9` thẳng. `a` và `b` cùng trỏ tới `x` (giá
trị 9), nên `a == b` cho `true`, đã đo thật.
::
:::

:::opt
Không biên dịch được — hàm `so_sanh` trả về `bool`, nhưng phép so
sánh `a == b` giữa hai tham chiếu `&i32` trả về kiểu `&bool`, không
khớp với `bool` đã khai trong chữ ký
::why
Gần đúng ở việc bạn để ý CHỮ KÝ hàm (`-> bool`) và kiểu của biểu thức
trả về là hai thứ cần khớp nhau — một thói quen kiểm tra đúng đắn.

Chỗ lệch: `a == b` (so sánh bằng) LUÔN cho ra `bool` thẳng, không
phải `&bool`, bất kể `a` và `b` là giá trị hay tham chiếu — phép so
sánh tự đi xuyên qua tham chiếu trước khi so, kết quả cuối cùng
không còn "là tham chiếu" nữa. Chữ ký `-> bool` khớp hoàn toàn.
::
:::
::::

::::code{#tong-ba-tham-chieu}
Viết một hàm nhận ba tham chiếu đọc, cộng cả ba giá trị lại.

```rust title=starter
fn tong_ba(a: &i32, b: &i32, c: &i32) -> i32 {
    a + b + ___
}
```

```rust title=solution
fn tong_ba(a: &i32, b: &i32, c: &i32) -> i32 {
    a + b + c
}
```

```rust title=test
fn main() {
    let x = 4;
    let ket_qua = tong_ba(&x, &x, &x);
    println!("{}", ket_qua);
    assert_eq!(
        ket_qua, 12,
        "tong_ba(&x, &x, &x) với x = 4 phải là 12 (4+4+4) — đang là {}", ket_qua
    );
}
```

:::hints
- kind: attention
  body: Ba tham chiếu đọc tới cùng một chỗ, cùng lúc, hoàn toàn hợp lệ — đây không phải tình huống bị chặn. Chỗ trống chỉ còn thiếu tham chiếu thứ ba trong phép cộng.
- kind: strategy
  body: 'a + b + ___ đã cộng hai tham chiếu đầu. Cộng nốt tham chiếu còn lại — c.'
- kind: one-line
  body: 'Chỗ trống là: c'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "12"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đọc bao nhiêu người cũng được — không ai thay đổi gì thì không có gì
để tranh cãi. Sửa thì khác: đúng một người, tại một thời điểm.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Luật vừa học gói gọn trong MỘT lời gọi hàm — Rust nhìn tất cả tham
chiếu xuất hiện CÙNG lúc trong một dòng, rồi quyết định ngay tại đó.
Nhưng chương trình thật không phải lúc nào cũng gói mọi thứ vào một
dòng: bạn có thể mượn `&x` ở một câu lệnh, DÙNG nó, rồi vài dòng
sau mới mượn `&mut x`.

Thứ tự đó có quan trọng không? Và nếu Byte chỉ nhìn được MỘT lời gọi
tại một thời điểm, nó có còn đủ sức trả lời câu hỏi này không?

Bài sau đi vào đúng chỗ khó nhất của cả track.
::::

::::checkpoint{mastery=0.8}
::::
