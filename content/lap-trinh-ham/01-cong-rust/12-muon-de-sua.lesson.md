---
id: lap-trinh-ham.cong-rust.muon-de-sua
title: "`&mut` — mượn để SỬA"
summary: "`fn tang(x: &mut i32) { *x += 1; } tang(&mut so);` mượn KHẢ BIẾN, sửa được giá trị gốc qua tham chiếu — dấu `*` giải tham chiếu để CHẠM tới giá trị thật. Đã đo thật: so đổi giá trị sau lời gọi. Khác `&` (chỉ đọc): `&mut` sửa được, và cú pháp tường minh ở CẢ tham số lẫn lúc gọi."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 12
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.borrow-mut]
requires: [rs.borrow-shared]
concepts: [rs.borrow-mut]
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
`&` cho mượn để đọc. Hôm nay: mượn để SỬA — và Rust bắt bạn nói rõ
điều đó ngay trong chữ ký hàm.
::::

::::explain{#muon-kha-bien}
`&String` (bài trước) chỉ cho ĐỌC — thử ghi qua nó bị chặn ngay lúc
biên dịch, không phải lúc chạy. Muốn một hàm SỬA được giá trị gốc mà
vẫn không lấy hẳn quyền sở hữu, cần một loại tham chiếu khác:

**`&mut`** (viết tắt của "mutable", khả biến) khai một tham chiếu
ĐƯỢC PHÉP GHI:

```
fn tang(x: &mut i32) {
    *x += 1;
}
```

Tham số `x` có kiểu `&mut i32` — một tham chiếu khả biến tới `i32`.
Bên trong thân hàm, `*x` (dấu `*` là **giải tham chiếu**, dereference)
đi XUYÊN QUA tham chiếu để chạm tới giá trị thật đằng sau nó — `*x +=
1` nghĩa là "cộng 1 vào giá trị mà `x` đang trỏ tới", không phải "đổi
`x` thành một tham chiếu khác".

Lúc gọi, cũng cần khai tường minh `&mut`:

```
let mut so = 5;
tang(&mut so);
println!("{}", so);   // 6 — giá trị GỐC đã đổi
```

Hai điều tường minh phải khớp nhau: biến gốc phải khai `mut` (nếu
không, không ai mượn khả biến được nó — luật bất biến-mặc-định của
bài trước quay lại đây), và lời gọi phải viết `&mut so`, không phải
`&so`. Thiếu chữ `mut` ở BẤT KỲ đâu trong hai chỗ đó, Rust từ chối
ngay, không đợi tới lúc chạy mới phát hiện giá trị không đổi.
::::

::::example{#doc-khong-sua-duoc}
Gọi hai lần liên tiếp — mỗi lần `tang` cộng thêm 1, không lần nào
xung đột vì chúng chạy TUẦN TỰ, không cùng lúc:

```rust title=readonly
fn tang(x: &mut i32) {
    *x += 1;
}

fn main() {
    let mut so = 5;
    tang(&mut so);
    tang(&mut so);
    println!("{}", so);
}
```

```text title=readonly
7
```

Còn nếu thử SỬA qua một tham chiếu chỉ-đọc (`&`, không `&mut`):

```rust title=readonly
fn tang(x: &i32) {
    *x += 1;
}

fn main() {
    let so = 5;
    tang(&so);
    println!("{}", so);
}
```

```text title=readonly
(không biên dịch được)

lỗi [BR0401]: không sửa được qua `x`
 --> dòng 2:5
  |
2 |     *x += 1;
  |     ^^ ghi qua tham chiếu ở đây
  |
 --> dòng 1:9
  |
1 | fn tang(x: &i32) {
  |         - đây là `&T`, chỉ đọc
  |
  cách sửa: đổi tham số thành `&mut T`, và bên gọi truyền `&mut x`
```

`&T` cho quyền ĐỌC, `&mut T` mới cho quyền GHI — Rust tách hẳn hai
quyền này ngay trong kiểu, không gộp chung "tham chiếu" thành một
khái niệm duy nhất như một số ngôn ngữ khác.
::::

::::predict{#hai-lan-tang commitOnce}
Byte viết một chương trình gọi `tang` hai lần, KHÔNG chạy thử:

```rust
fn tang(x: &mut i32) {
    *x += 1;
}

fn main() {
    let mut so = 5;
    tang(&mut so);
    tang(&mut so);
    println!("{}", so);
}
```

**Trước khi đọc đáp án**, dòng cuối in ra số nào?

:::opt{correct}
`7` — `so` bắt đầu là 5, mỗi lần gọi `tang(&mut so)` cộng thêm 1 vào
CHÍNH giá trị gốc (không phải một bản sao), và hai lời gọi chạy tuần
tự, không cùng lúc, nên không có gì bị chặn
:::

:::opt
`5` — `tang` chỉ sửa được BẢN SAO cục bộ bên trong hàm, giống hệt
truyền theo giá trị bình thường; giá trị gốc `so` ở `main` không hề
đổi
::why
Gần đúng ở việc bạn nhớ đúng: truyền THEO GIÁ TRỊ (không `&`) đúng là
chỉ đưa một bản sao (với kiểu `Copy` như `i32`) hoặc move hẳn.

Chỗ lệch: `x: &mut i32` không phải truyền theo giá trị — nó là một
THAM CHIẾU tới đúng `so` trong `main`. `*x += 1` cộng thẳng vào giá
trị mà tham chiếu đang trỏ tới, tức chính là `so` — không phải một
bản sao. Đã đo thật: giá trị gốc đổi theo.
::
:::

:::opt
Không biên dịch được — gọi `tang(&mut so)` hai lần trên cùng một
biến `so` giống hệt tình huống "hai `&mut` cùng lúc" bị Rust chặn
::why
Gần đúng ở việc bạn cảnh giác với "hai lần mượn khả biến" — phản xạ
đúng hướng cho phần sắp học.

Chỗ lệch: luật bị chặn là hai `&mut` tới CÙNG một chỗ TRONG CÙNG MỘT
lời gọi (ví dụ `cong(&mut so, &mut so)`). Ở đây có HAI lời gọi tách
rời, chạy lần lượt — lời gọi đầu xong hẳn, tham chiếu của nó hết
hạn, RỒI lời gọi thứ hai mới bắt đầu. Không có hai tham chiếu nào
cùng sống một lúc, nên không gì bị chặn cả. Đã đo thật: biên dịch và
chạy sạch.
::
:::

:::opt
`6` — chỉ lần gọi ĐẦU TIÊN có hiệu lực; lần gọi thứ hai bị Rust âm
thầm bỏ qua vì `so` "đã bị mượn rồi"
::why
Gần đúng ở việc bạn nghĩ có một giới hạn nào đó về số lần mượn — một
trực giác không sai cho track này, chỉ chưa đúng chỗ áp dụng ở đây.

Chỗ lệch: Rust không có khái niệm "mượn rồi thì thôi, lần sau bị bỏ
qua". Mỗi lời gọi `tang(&mut so)` tạo một tham chiếu khả biến MỚI,
sống trong đúng thời gian lời gọi đó chạy, rồi hết hạn hoàn toàn.
Không có gì bị "nhớ" hay bỏ qua giữa hai lần gọi — cả hai đều thực
thi đầy đủ, cộng dồn thành `7`.
::
:::
::::

::::code{#nhan-doi-qua-tham-chieu}
Viết một hàm mượn khả biến một số nguyên, nhân đôi giá trị NGAY TẠI
chỗ nó đang trỏ tới.

```rust title=starter
fn nhan_doi(x: &mut i32) {
    *x = ___;
}
```

```rust title=solution
fn nhan_doi(x: &mut i32) {
    *x = *x * 2;
}
```

```rust title=test
fn main() {
    let mut so = 5;
    nhan_doi(&mut so);
    println!("{}", so);
    assert_eq!(
        so, 10,
        "sau nhan_doi(&mut so), so phải là 10 (5 * 2) — đang là {}", so
    );
}
```

:::hints
- kind: attention
  body: x là &mut i32. *x đọc giá trị mà x đang trỏ tới; chỗ trống phải gán lại đúng giá trị NHÂN ĐÔI vào chính chỗ đó.
- kind: strategy
  body: 'Vế trái *x = ___ đã có sẵn. Vế phải cần đọc *x (giá trị hiện tại) rồi nhân với 2.'
- kind: one-line
  body: 'Chỗ trống là: *x * 2'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "10"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`&` chỉ mở cửa sổ để nhìn. `&mut` đưa cho bạn cả chìa khoá để sửa —
nhưng Rust chỉ phát MỘT chìa cho MỘT chỗ tại một thời điểm. Bài sau
kiểm chứng đúng luật đó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa gọi `tang(&mut so)` HAI LẦN, tuần tự — không sao cả, vì lời
gọi đầu xong hẳn trước khi lời gọi sau bắt đầu.

Nhưng nếu một hàm cần HAI tham chiếu khả biến CÙNG MỘT LÚC, trong
CÙNG một lời gọi — ví dụ muốn cộng dồn một số vào chính nó — chuyện
gì xảy ra? Rust có cho phép một biến bị mượn khả biến hai lần đồng
thời không?

Bài sau trả lời — và câu trả lời không phải điều bạn nghĩ.
::::

::::checkpoint{mastery=0.8}
::::
