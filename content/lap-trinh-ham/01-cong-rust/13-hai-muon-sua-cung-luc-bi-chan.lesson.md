---
id: lap-trinh-ham.cong-rust.hai-muon-sua-cung-luc-bi-chan
title: "Hai `&mut` tới cùng một chỗ, cùng lúc, bị chặn"
summary: "`fn cong(a: &mut i32, b: &mut i32) {...} cong(&mut x, &mut x);` — BR0355 (\"mượn x dạng &mut hai lần cùng lúc\"), TỪ CHỐI biên dịch. Luật hẹp thứ hai mà byte-rust kiểm được không cần phân tích luồng: hai &mut trong CÙNG MỘT lời gọi. Vì sao luật này tồn tại: nếu hai chỗ cùng sửa được một biến một lúc, không ai đoán nổi kết quả cuối thuộc về bên nào."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 13
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.double-mut-blocked]
requires: [rs.borrow-mut]
concepts: [rs.double-mut-blocked]
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
Gọi `&mut` hai lần TUẦN TỰ thì được — bài trước vừa thấy. Cùng lúc,
tới CÙNG một chỗ, thì sao?
::::

::::explain{#hai-chia-khoa-mot-o-khoa}
Một hàm nhận HAI tham chiếu khả biến, cộng giá trị bên này vào bên
kia:

```
fn cong(a: &mut i32, b: &mut i32) {
    *a += *b;
}
```

Gọi hàm này với hai BIẾN KHÁC NHAU không có vấn đề gì — hai tham
chiếu, hai chỗ khác nhau. Nhưng nếu gọi với CÙNG MỘT biến ở cả hai
vị trí:

```
let mut x = 5;
cong(&mut x, &mut x);
```

Rust TỪ CHỐI biên dịch. Đây không phải một giới hạn ngẫu nhiên — nó
là hệ quả trực tiếp của luật gốc cả track này xoay quanh: tại một
thời điểm, một chỗ trong bộ nhớ chỉ được ĐÚNG MỘT người có quyền
sửa. `cong(&mut x, &mut x)` đưa CẢ `a` LẪN `b` cùng trỏ vào `x`, cùng
lúc, cùng được phép ghi — nếu điều đó xảy ra thật, không ai (kể cả
chính Rust) đoán nổi thứ tự `*a += *b` sẽ đọc/ghi ra sao, vì `a` và
`b` đang là HAI TÊN cho MỘT ô nhớ. Thay vì để hành vi đó mơ hồ, Rust
xoá bỏ khả năng đó hoàn toàn — ngay lúc biên dịch, không đợi tới lúc
chạy mới phát hiện kết quả sai lệch tuỳ máy tuỳ lần chạy.

Đây là **luật hẹp thứ hai** mà `byte-rust` — công cụ chấm bài của
track này — kiểm được không cần phân tích luồng chương trình: hai
`&mut` tới CÙNG một biến, trong CÙNG MỘT lời gọi hàm. (Luật hẹp thứ
nhất là dùng lại sau move, bài trước đó vài bài.) Cả hai đều nhìn
được ngay tại một điểm trong mã nguồn, không cần đoán chương trình
sẽ chạy qua nhánh nào.
::::

::::example{#bi-chan-va-khong-bi-chan}
`cong(&mut x, &mut y)` — hai biến KHÁC nhau, hợp lệ:

```rust title=readonly
fn cong(a: &mut i32, b: &mut i32) {
    *a += *b;
}

fn main() {
    let mut x = 5;
    let mut y = 3;
    cong(&mut x, &mut y);
    println!("{}", x);
}
```

```text title=readonly
8
```

Đổi `y` thành `x` — cùng một biến ở cả hai vị trí:

```rust title=readonly
fn cong(a: &mut i32, b: &mut i32) {
    *a += *b;
}

fn main() {
    let mut x = 5;
    cong(&mut x, &mut x);
    println!("{}", x);
}
```

```text title=readonly
(không biên dịch được)

lỗi [BR0355]: mượn `x` dạng `&mut` hai lần cùng lúc
 --> dòng 7:5
  |
7 |     cong(&mut x, &mut x);
  |     ^^^^^^^^^^^^^^^^^^^^ hai lần mượn khả biến cùng sống trong một lời gọi
  |
  cách sửa: truyền giá trị thay vì tham chiếu, hoặc tách thành hai lời gọi
```

Cùng một hàm `cong`, cùng cú pháp gọi — chỉ khác nhau ở việc hai tham
số trỏ tới HAI Ô NHỚ khác nhau hay CÙNG một ô nhớ. Rust không kiểm
"bạn có viết `&mut` hai lần không" — nó kiểm "hai `&mut` đó có cùng
trỏ vào một chỗ không".
::::

::::predict{#doi-cho-hai-so commitOnce}
Byte viết một hàm đổi chỗ hai số, gọi nó HAI LẦN, KHÔNG chạy thử:

```rust
fn doi_cho(a: &mut i32, b: &mut i32) {
    let tam = *a;
    *a = *b;
    *b = tam;
}

fn main() {
    let mut m = 1;
    let mut n = 2;
    doi_cho(&mut m, &mut n);
    doi_cho(&mut m, &mut m);
    println!("{} {}", m, n);
}
```

**Trước khi đọc đáp án**, dòng nào (nếu có) bị Rust từ chối, và vì
sao?

:::opt{correct}
Dòng `doi_cho(&mut m, &mut m);` (dòng thứ hai gọi hàm) — hai `&mut`
cùng trỏ vào `m`, cùng lúc, trong cùng một lời gọi (BR0355). Dòng gọi
TRƯỚC đó, `doi_cho(&mut m, &mut n)`, hợp lệ vì `m` và `n` là hai biến
khác nhau
:::

:::opt
Cả hai dòng gọi `doi_cho` đều bị từ chối — hàm này nhận HAI tham số
`&mut i32`, mà luật của Rust là một hàm không được có QUÁ MỘT tham
số kiểu `&mut` trong chữ ký của nó
::why
Gần đúng ở việc bạn nghĩ có một giới hạn liên quan tới SỐ LƯỢNG tham
số `&mut`.

Chỗ lệch: không có luật nào giới hạn số THAM SỐ `&mut` một hàm được
khai. `fn doi_cho(a: &mut i32, b: &mut i32)` hoàn toàn hợp lệ ngay
từ chữ ký. Luật thật chỉ xét lúc GỌI: hai tham chiếu khả biến đó có
trỏ chung một chỗ hay không. `doi_cho(&mut m, &mut n)` trỏ hai chỗ
khác nhau — hợp lệ, đã đo thật (in ra `2 1`).
::
:::

:::opt
Không dòng nào bị từ chối — biến `tam` bên trong `doi_cho` là một
bản sao cục bộ, nên dù gọi với cùng một biến ở cả hai vị trí, việc
đọc qua `tam` vẫn tách bạch hai giá trị ra được, không có xung đột
gì thật sự
::why
Gần đúng ở việc bạn để ý `tam` là một biến RIÊNG, cục bộ trong hàm —
quan sát đúng về cách hàm hoạt động NẾU nó chạy được.

Chỗ lệch: Rust chặn `cong(&mut x, &mut x)`-kiểu lời gọi này TRƯỚC KHI
xét xem thân hàm làm gì với dữ liệu đó. Luật nằm ở LỜI GỌI (hai tham
chiếu khả biến cùng trỏ một ô nhớ), không phải ở việc thân hàm có
"xử lý khéo" được xung đột hay không — dù `tam` có giúp logic đúng
tới đâu, Rust vẫn từ chối biên dịch trước khi thân hàm kịp chạy.
::
:::

:::opt
Dòng `doi_cho(&mut m, &mut n)` (dòng gọi ĐẦU TIÊN) bị từ chối, vì đây
là lần đầu `m` và `n` được mượn khả biến — lần gọi thứ hai thì được
phép vì cả hai "đã quen" bị mượn từ trước
::why
Gần đúng ở việc bạn để ý có một sự khác biệt giữa lần gọi thứ nhất
và thứ hai — đúng là chúng khác nhau thật.

Chỗ lệch: Rust không có khái niệm biến "đã quen bị mượn" nên lần sau
dễ dãi hơn. Mỗi lời gọi được xét ĐỘC LẬP, chỉ dựa trên các tham
chiếu xuất hiện TRONG lời gọi đó. Lời gọi đầu (`m`, `n` — hai biến
khác nhau) hợp lệ; lời gọi sau (`m`, `m` — cùng một biến) mới là chỗ
bị chặn, đúng ngược lại với đáp án này.
::
:::
::::

::::code{#cong-hai-bien-khac-nhau}
Một hàm cộng dồn giá trị này vào giá trị kia, qua hai tham chiếu khả
biến. Điền chỗ trống để phép cộng đúng.

```rust title=starter
fn cong(a: &mut i32, b: &mut i32) {
    *a += ___;
}
```

```rust title=solution
fn cong(a: &mut i32, b: &mut i32) {
    *a += *b;
}
```

```rust title=test
fn main() {
    let mut x = 5;
    let mut y = 3;
    cong(&mut x, &mut y);
    println!("{}", x);
    assert_eq!(
        x, 8,
        "sau cong(&mut x, &mut y), x phải là 8 (5 + 3) — đang là {}", x
    );
}
```

:::hints
- kind: attention
  body: a và b là hai tham chiếu khả biến TỚI HAI BIẾN KHÁC NHAU (x và y) — hợp lệ, không phải tình huống bị chặn ở bài này. Chỗ trống cần đọc giá trị b đang trỏ tới.
- kind: strategy
  body: '*a += ___ đã có sẵn vế trái. Muốn cộng dồn giá trị của b vào a, đọc giá trị mà b đang trỏ tới bằng *b.'
- kind: one-line
  body: 'Chỗ trống là: *b'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "8"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai chỗ khác nhau, mỗi chỗ một chìa — không sao. Cùng một chỗ, hai
chìa cùng lúc — Rust không để chuyện đó xảy ra, dù chỉ một lần.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`&mut` hai lần tới CÙNG một chỗ, cùng lúc, bị chặn — bạn vừa thấy
rõ. Nhưng luật đó nói riêng về `&mut`. Còn `&` (chỉ đọc, bài 11) thì
sao — hai `&` tới cùng một chỗ, cùng lúc, có bị chặn giống vậy
không, hay Rust chỉ khắt khe với việc SỬA?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
