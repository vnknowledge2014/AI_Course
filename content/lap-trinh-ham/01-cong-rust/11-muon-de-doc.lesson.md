---
id: lap-trinh-ham.cong-rust.muon-de-doc
title: "`&` — mượn để ĐỌC, không lấy quyền sở hữu"
summary: "`fn in_ra(s: &String) { println!(\"{}\", s); } in_ra(&ten);` truyền một THAM CHIẾU, không phải move. `ten` vẫn dùng được BÌNH THƯỜNG sau lời gọi (đã đo thật) — công cụ giải quyết đúng vấn đề bài trước nêu ra: đọc một giá trị mà không lấy mất quyền sở hữu."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 11
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.borrow-shared]
requires: [rs.self-vs-ref-self]
concepts: [rs.borrow-shared]
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
`self` lấy hẳn, `&self` chỉ mượn. Cùng luật đó bây giờ mở ra khỏi
method — áp dụng cho MỌI tham số hàm.
::::

::::explain{#muon-thay-vi-lay-han}
Nhớ lại: truyền một `String` vào hàm theo giá trị là **move** — hàm
nhận quyền sở hữu, biến gốc không dùng lại được nữa. Nhưng phần lớn
lúc gọi hàm, bạn không MUỐN hàm giữ luôn giá trị — bạn chỉ muốn nó
ĐỌC, rồi trả quyền lại ngay.

**`&`** đứng trước một kiểu trong chữ ký hàm khai một **tham chiếu**
(reference) — "mượn để đọc", không lấy quyền sở hữu:

```
fn in_ra(s: &String) {
    println!("{}", s);
}
```

Tham số `s` có kiểu `&String`, không phải `String` — đọc là "một tham
chiếu tới `String`". Lúc gọi, dùng dấu `&` trước biến để MƯỢN nó thay
vì đưa hẳn:

```
let ten = String::from("Lan");
in_ra(&ten);
println!("{}", ten);   // ten vẫn dùng được — chỉ CHO MƯỢN, không đưa hẳn
```

Khác hẳn bài trước (truyền `ten` thẳng vào `in_ra(s: String)`, tiêu
thụ `ten`): `&ten` chỉ đưa cho `in_ra` một CON ĐƯỜNG tới giá trị, y
hệt việc cho ai đó xem một cuốn sổ qua vai bạn — họ đọc được, nhưng
cuốn sổ vẫn ở trong tay bạn suốt. Hàm chạy xong, tham chiếu hết hạn,
`ten` vẫn còn nguyên chủ.
::::

::::example{#so-sanh-dua-va-cho-muon}
Cùng một `ten`, hai cách gọi. Trước tiên, đưa hẳn (move, đã học ở
bài trước) — dùng lại `ten` sau đó bị chặn:

```rust title=readonly
fn in_ra(s: String) {
    println!("{}", s);
}

fn main() {
    let ten = String::from("Lan");
    in_ra(ten);
    println!("{}", ten);
}
```

```text title=readonly
(không biên dịch được)

lỗi [BR0530]: `ten` đã bị chuyển quyền sở hữu đi nơi khác
 --> dòng 8:20
  |
8 |     println!("{}", ten);
  |                    ^^^ dùng lại ở đây thì không còn giá trị nữa
  |
 --> dòng 7:11
  |
7 |     in_ra(ten);
  |           --- quyền sở hữu bị chuyển đi tại đây
  |
  cách sửa: nếu muốn giữ cả hai, hãy nhân bản: `ten.clone()`
  cách sửa: hoặc chỉ mượn thay vì lấy hẳn: `&ten`
```

Đúng như thông báo gợi ý ở dòng cuối — đổi tham số thành `&String`,
gọi bằng `&ten`:

```rust title=readonly
fn in_ra(s: &String) {
    println!("{}", s);
}

fn main() {
    let ten = String::from("Lan");
    in_ra(&ten);
    println!("{}", ten);
}
```

```text title=readonly
Lan
Lan
```

Hai dòng `Lan` — một do `in_ra` in ra, một do `println!` sau đó, vẫn
đọc được `ten` bình thường. Không có gì bị chuyển đi: `&ten` chỉ cho
`in_ra` MƯỢN một con đường tới giá trị, không lấy quyền sở hữu.
::::

::::predict{#muon-roi-dua-han commitOnce}
Byte viết một chương trình dùng CẢ HAI cách — mượn rồi mới đưa hẳn,
KHÔNG chạy thử:

```rust
fn do_dai(s: &String) -> usize {
    s.len()
}

fn in_ra(s: String) {
    println!("{}", s);
}

fn main() {
    let ten = String::from("Mai");
    let dai = do_dai(&ten);
    in_ra(ten);
    println!("{} {}", dai, ten);
}
```

**Trước khi đọc đáp án**, đoạn mã này có biên dịch được không, và vì
sao?

:::opt{correct}
Không — `do_dai(&ten)` chỉ mượn nên `ten` vẫn còn sau dòng đó, nhưng
`in_ra(ten)` ngay sau lại đưa hẳn (move), tiêu thụ `ten`. Dòng
`println!` cuối dùng lại `ten` sau khi đã move — bị chặn (BR0530)
:::

:::opt
Không — `do_dai(&ten)` đã tiêu thụ `ten` ngay từ dòng đó, vì bất cứ
lời gọi hàm nào cũng luôn là move, `&` không thay đổi được điều đó
::why
Gần đúng ở việc bạn nhớ đúng: gọi hàm CÓ THỂ là move — đúng bản chất
của bài trước.

Chỗ lệch: `&` đứng trước kiểu tham số (`s: &String`) chính là cú
pháp đổi hẳn luật đó — nó khai một THAM CHIẾU, không phải kiểu gốc.
`do_dai(&ten)` mượn, không move; `ten` vẫn còn sau dòng đó, đã đo
thật. Lỗi thật sự tới từ dòng `in_ra(ten)` phía sau — đó MỚI là move
thật, vì `in_ra` nhận `s: String` (không `&`).
::
:::

:::opt
Có, biên dịch được — `do_dai(&ten)` chỉ mượn nên không đụng gì tới
quyền sở hữu, và `in_ra(ten)` cũng không sao vì `ten` đã "được giải
phóng" khỏi việc mượn trước đó rồi
::why
Gần đúng ở nửa đầu: `do_dai(&ten)` đúng là chỉ mượn, không đụng
quyền sở hữu — nhận xét đó chính xác.

Chỗ lệch: mượn (`&ten`) và move là hai chuyện khác nhau hoàn toàn,
không phải "hết mượn rồi thì lại y như ban đầu, mọi lời gọi sau đều
an toàn". `in_ra(ten)` là một move THẬT — không liên quan gì tới
việc `ten` có từng bị mượn trước đó hay không. Sau `in_ra(ten)`,
`ten` mất quyền sở hữu; dòng `println!` cuối dùng lại nó bị chặn.
::
:::

:::opt
Không — nhưng lỗi nằm ở `do_dai(&ten)`, vì hàm này khai kiểu trả về
`usize` trong khi `String::from("Mai")` có 3 ký tự, một con số `i32`
bình thường, không phải `usize`
::why
Gần đúng ở việc bạn để ý tới kiểu trả về của `do_dai` — một chỗ đáng
kiểm tra khi đọc chữ ký hàm.

Chỗ lệch: `usize` KHÔNG phải "một con số bình thường phải là i32" —
đó là kiểu chuẩn Rust dùng cho độ dài, chỉ số mảng, và đúng là kiểu
`.len()` trả về. `do_dai(&ten) -> usize` khai đúng, không lệch kiểu
gì cả. Lỗi thật nằm ở dòng `println!` cuối, dùng lại `ten` sau khi
nó đã bị `in_ra(ten)` move đi.
::
:::
::::

::::code{#do-dai-chuoi}
Viết một hàm mượn một `String` để đọc độ dài của nó, KHÔNG lấy quyền
sở hữu.

```rust title=starter
fn dem_ky_tu(s: &String) -> usize {
    ___
}
```

```rust title=solution
fn dem_ky_tu(s: &String) -> usize {
    s.len()
}
```

```rust title=test
fn main() {
    let ten = String::from("Lan");
    let so_ky_tu = dem_ky_tu(&ten);
    println!("{}", so_ky_tu);
    println!("{}", ten);
    assert_eq!(
        so_ky_tu, 3,
        "dem_ky_tu(&ten) phải trả về 3 — đang là {}", so_ky_tu
    );
    assert_eq!(ten, "Lan", "ten phải còn dùng được sau khi mượn qua &ten");
}
```

:::hints
- kind: attention
  body: s là một tham chiếu tới String (&String). Đọc độ dài của chuỗi đằng sau tham chiếu đó, không cần lấy quyền sở hữu gì cả.
- kind: strategy
  body: 'String có sẵn một phương thức đếm số ký tự. Gọi thẳng phương thức đó trên s.'
- kind: one-line
  body: 'Chỗ trống là: s.len()'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mượn xong, trả lại nguyên vẹn — chủ vẫn là chủ. `&` không lấy gì cả,
nó chỉ mở một con đường tạm thời tới giá trị.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`&` cho mượn để ĐỌC. Nhưng đọc thôi không đủ — đôi lúc bạn cần một
hàm THAY ĐỔI được giá trị gốc, mà vẫn không muốn lấy hẳn quyền sở
hữu của nó (lấy hẳn thì sau đó biến gốc mất luôn, y hệt bài trước).

Rust có công cụ nào cho việc "mượn, nhưng được phép sửa" không? Bài
sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
