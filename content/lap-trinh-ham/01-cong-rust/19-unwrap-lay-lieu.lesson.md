---
id: lap-trinh-ham.cong-rust.unwrap-lay-lieu
title: "`.unwrap()` — lấy liều, panic có kiểm soát nếu không có gì"
summary: "`.unwrap()` lấy giá trị bên trong `Some`/`Ok` ngay, hoặc PANIC (dừng chương trình có kiểm soát, chẩn đoán rõ ràng) nếu gặp `None`/`Err` — đã đo thật (BR0551). Không phải cách lười hợp lệ mọi lúc — dùng khi bạn CHẮC CHẮN giá trị luôn có."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 19
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.unwrap-panic]
requires: [rs.option-match]
concepts: [rs.unwrap-panic]
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
`match` đòi viết đủ hai nhánh, mỗi lần. Hôm nay một lối tắt — nhưng là
lối tắt CÓ CÁI GIÁ của nó, và bạn cần biết cái giá đó trước khi dùng.
::::

::::explain{#lay-lieu}
**`.unwrap()`** là một method có sẵn trên cả `Option<T>` lẫn `Result<T,
E>` (bài sau). Nó làm đúng một việc, không rẽ nhánh nào:

- Gặp `Some(giá_trị)` (hoặc `Ok(giá_trị)`) — trả về `giá_trị` NGAY,
  không bọc `Some`/`Ok` gì nữa.
- Gặp `None` (hoặc `Err(...)`) — chương trình **panic**: dừng lại có
  kiểm soát, in ra chẩn đoán rõ ràng chỗ nào gây ra, không phải một cú
  sập mờ mịt.

Đây KHÔNG phải cách "lười" hợp lệ ở mọi nơi thay cho `match`. Dùng
`.unwrap()` là một lời TUYÊN BỐ: "tôi chắc chắn giá trị này luôn có ở
đây — nếu tôi sai, tôi muốn chương trình dừng NGAY và nói rõ tại sao,
thay vì âm thầm chạy tiếp với dữ liệu rỗng." `match` (bài trước) mới là
cách xử lý ĐẦY ĐỦ khi khả năng rỗng có thể xảy ra thật và bạn cần
chương trình VẪN CHẠY TIẾP dù gặp `None`.

Panic không phải "chương trình bị lỗi ngầm" — nó là một điểm dừng CHỦ
Ý, có thông báo, chỉ đích danh dòng nào và vì sao. Khác hẳn một chương
trình tiếp tục chạy với dữ liệu sai mà không ai hay biết.
::::

::::example{#lay-ra-hoac-panic}
Gọi `.unwrap()` trên một `Option<i32>` có giá trị:

```rust title=readonly
fn main() {
    let a: Option<i32> = Some(5);
    let x = a.unwrap();
    println!("{}", x);
}
```

```text title=readonly
5
```

`x` giờ là một `i32` THUẦN, không còn bọc trong `Option` — không cần
`match`, không cần bóc gì thêm. Nhưng gọi đúng `.unwrap()` đó trên một
`Option` đang là `None`:

```rust title=readonly
fn main() {
    let a: Option<i32> = None;
    let x = a.unwrap();
    println!("{}", x);
}
```

```text title=readonly
(chương trình dừng giữa chừng, dòng println! không chạy)

lỗi [BR0551]: `unwrap()` gọi trên `None`
 --> dòng 3:13
  |
3 |     let x = a.unwrap();
  |             ^^^^^^^^^^ chương trình dừng ở đây
  |
 --> dòng 3:13
  |
3 |     let x = a.unwrap();
  |             - giá trị này là `None`
```

Không có `x` nào được gán, dòng `println!` không bao giờ chạy tới —
chương trình dừng đúng tại dòng gọi `.unwrap()`, với thông báo chỉ
thẳng nguyên nhân: giá trị đó là `None`.
::::

::::predict{#in-ra-gi-roi-dung-o-dau commitOnce}
Byte viết một chương trình, KHÔNG chạy thử:

```rust
fn tim_phan_tu_dau(v: Vec<i32>) -> Option<i32> {
    if v.len() > 0 {
        Some(v[0])
    } else {
        None
    }
}

fn main() {
    let a = tim_phan_tu_dau(vec![10, 20]).unwrap();
    println!("{}", a);
    let b: Vec<i32> = Vec::new();
    let c = tim_phan_tu_dau(b).unwrap();
    println!("{}", c);
}
```

**Trước khi đọc đáp án**, chương trình in ra gì, rồi chuyện gì xảy ra?

:::opt{correct}
In ra `10` (dòng `println!("{}", a)` chạy trọn vẹn, vì vec đầu tiên có
phần tử). Sau đó chương trình PANIC ngay tại `tim_phan_tu_dau(b)
.unwrap()` — vec `b` rỗng nên hàm trả `None`, và `.unwrap()` trên
`None` luôn dừng chương trình. Dòng `println!("{}", c)` không bao giờ
chạy tới
:::

:::opt
In ra `10` rồi `0` — khi `.unwrap()` gặp `None`, nó trả về giá trị mặc
định của kiểu đó (`0` cho `i32`) thay vì panic
::why
Gần đúng ở việc bạn nghĩ tới một "giá trị dự phòng" khi gặp rỗng — cảm
giác hợp lý nếu quen ngôn ngữ có giá trị mặc định ngầm.

Chỗ lệch: `.unwrap()` không có khái niệm "giá trị mặc định". Gặp
`None`, nó panic — dừng chương trình — không tự thay bằng `0` hay bất
cứ gì khác. Dòng `println!("{}", c)` không bao giờ chạy tới, không in
ra `0`.
::
:::

:::opt
Không in được gì cả — `.unwrap()` phải đi kèm `match` phía trước để
kiểm tra trước, gọi trực tiếp như vậy là lỗi cú pháp
::why
Gần đúng ở việc bạn liên tưởng `.unwrap()` với việc "cần kiểm tra
trước" — đúng tinh thần cẩn trọng, nhưng không đúng cú pháp Rust đòi
hỏi.

Chỗ lệch: gọi `.unwrap()` trực tiếp trên một `Option`/`Result`, không
cần `match` đứng trước, là cú pháp hợp lệ — đó chính là điểm của
`.unwrap()`, một lối lấy giá trị NGAY. Dòng đầu tiên (`a`) không panic
gì cả, vì vec đó không rỗng — nó in ra `10` bình thường trước khi
chương trình dừng ở dòng `c`.
::
:::

:::opt
In ra `10` rồi `20` — `tim_phan_tu_dau` trả về TOÀN BỘ các phần tử của
vec, không chỉ phần tử đầu
::why
Gần đúng ở việc bạn nhớ đúng vec đầu có hai phần tử, `10` và `20`.

Chỗ lệch: `tim_phan_tu_dau` chỉ trả về `Some(v[0])` — đúng MỘT phần tử
ĐẦU TIÊN, không phải toàn bộ vec. Và lần gọi thứ hai dùng vec RỖNG
(`Vec::new()`), không phải vec đầu — không có `20` nào liên quan tới
lần gọi đó cả. Chương trình panic ở lần gọi thứ hai, không in thêm gì.
::
:::
::::

::::code{#tuoi-hop-le-unwrap}
Một hàm kiểm tra điểm thi hợp lệ (`0` đến `10`), trả về `Option<i32>` —
giống ý bài trước. Lần này, gọi `.unwrap()` trực tiếp trên một điểm mà
BẠN đã biết chắc là hợp lệ. Điền chỗ trống trong điều kiện.

```rust title=starter
fn diem_thi_hop_le(diem: i32) -> Option<i32> {
    if diem >= 0 && diem <= ___ {
        Some(diem)
    } else {
        None
    }
}
```

```rust title=solution
fn diem_thi_hop_le(diem: i32) -> Option<i32> {
    if diem >= 0 && diem <= 10 {
        Some(diem)
    } else {
        None
    }
}
```

```rust title=test
fn main() {
    let diem = diem_thi_hop_le(8).unwrap();
    println!("{}", diem);
    assert_eq!(diem, 8, "diem 8 hợp lệ, unwrap phải lấy ra đúng 8 từ Some(8)");
    let diem2 = diem_thi_hop_le(10).unwrap();
    assert_eq!(diem2, 10, "diem 10 hợp lệ (biên trên), unwrap phải lấy ra đúng 10");
}
```

:::hints
- kind: attention
  body: 'Điểm thi hợp lệ nằm trong khoảng 0 đến bao nhiêu? Đề bài nói rõ: "0 đến 10".'
- kind: strategy
  body: Chỗ trống là biên TRÊN của khoảng hợp lệ trong điều kiện diem <= ___. Đề bài định nghĩa điểm hợp lệ là từ 0 đến 10.
- kind: one-line
  body: 'Chỗ trống là: 10'
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
`.unwrap()` không phải một cách "né" `match` — nó là một lời tuyên bố
tự tin, và tự tin sai thì chương trình dừng ngay, nói rõ vì sao.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`Option<T>` diễn tả "có thể có, có thể không có GÌ CẢ" — không mang
theo lý do vì sao lại không có. Nhưng nhiều tình huống thất bại cần
NÓI RÕ lý do — "chia cho 0" khác hẳn "file không tồn tại", dù cả hai
đều là "không có kết quả". Rust có kiểu nào diễn tả "thất bại, VÀ đây
là lý do" không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
