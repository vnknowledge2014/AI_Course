---
id: lap-trinh-ham.cong-rust.map-filter-sum-qua-iterator
title: "`.map()`/`.filter()`/`.sum()` — biến đổi một dãy giá trị"
summary: "`v.iter().map(|x| x * 2).collect()` nhân đôi từng phần tử; `v.into_iter().filter(|x| x % 2 == 0).collect()` giữ lại phần tử thoả điều kiện; `v.iter().sum()` cộng dồn cả dãy. Closure luôn truyền TRỰC TIẾP cho `.map()`/`.filter()` — không gán vào một cái tên rồi gọi lại sau."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 22
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.iterator-basics]
requires: [rs.question-mark]
concepts: [rs.iterator-basics]
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
`?` vừa khép lại cụm xử lý lỗi. Giờ một việc khác hẳn: biến đổi cả một
dãy giá trị, không phải bằng vòng lặp viết tay, mà bằng ba mảnh ghép
lại được với nhau.
::::

::::explain{#closure-va-ba-manh}
Một **closure** là một hàm không tên, viết ngay tại chỗ cần dùng nó:

```
|x| x * 2
```

Đọc: nhận một tham số `x`, trả về `x * 2` — không `fn`, không tên hàm,
không dấu ngoặc nhọn nếu thân chỉ có một biểu thức. Rust tự suy luận
kiểu của `x` từ ngữ cảnh dùng nó.

Ba phương thức trên một dãy giá trị (`Vec<T>`), ghép được với closure:

- **`.iter()`** — cho một luồng THAM CHIẾU tới từng phần tử (`&T`), Vec
  gốc còn nguyên, dùng lại được sau đó.
- **`.into_iter()`** — cho một luồng GIÁ TRỊ (`T`), Vec gốc bị move,
  không dùng lại được — đúng luật move đã học từ cụm 2, áp dụng lên cả
  Vec.
- **`.map(closure)`** — áp closure lên TỪNG phần tử, trả một luồng mới
  cùng số lượng phần tử, giá trị đã biến đổi.
- **`.filter(closure)`** — GIỮ LẠI phần tử mà closure trả về `true`,
  bỏ phần tử trả về `false` — số lượng phần tử có thể ÍT hơn dãy gốc.
- **`.collect()`** — gom luồng lại thành một tập cụ thể (thường là
  `Vec<T>`); cần biết gom thành KIỂU gì, nên luôn đi kèm một khai kiểu ở
  biến nhận, ví dụ `let ket_qua: Vec<i32> = ...`.
- **`.sum()`** — cộng dồn cả luồng thành một con số duy nhất, cũng cần
  khai kiểu con số đó ở biến nhận.

Một điều cần tránh, không phải vì Ý sai mà vì cách viết đó khiến `.map()`
không tìm ra được closure của bạn: đừng đặt closure vào một cái tên
riêng rồi gọi lại tên đó ở chỗ khác — luôn viết closure NGAY TẠI chỗ
gọi `.map(...)` hay `.filter(...)`, như mọi ví dụ trong bài này.
::::

::::example{#ba-vi-du-chay-that}
Nhân đôi từng phần tử, giữ Vec gốc còn nguyên:

```rust title=readonly
fn main() {
    let so = vec![1, 2, 3, 4];
    let gap_doi: Vec<i32> = so.iter().map(|x| x * 2).collect();
    println!("{:?}", gap_doi);
}
```

```text title=readonly
[2, 4, 6, 8]
```

`so.iter()` cho từng phần tử dưới dạng `&i32`. `.map(|x| x * 2)` nhân
đôi từng cái — Rust tự lo phần lấy giá trị ra khỏi tham chiếu để nhân,
không cần bạn viết `*x`. `.collect()` gom kết quả vào một `Vec<i32>`
mới; `so` không hề bị đổi hay bị lấy đi.

Giữ lại số chẵn, lần này dùng `.into_iter()`:

```rust title=readonly
fn main() {
    let so = vec![1, 2, 3, 4, 5, 6];
    let chan: Vec<i32> = so.into_iter().filter(|x| x % 2 == 0).collect();
    println!("{:?}", chan);
}
```

```text title=readonly
[2, 4, 6]
```

`.into_iter()` lấy hẳn quyền sở hữu từng phần tử — `so` bị move ở dòng
này, không dùng lại được sau đó (đúng luật move của cụm 2). Đổi lại,
closure trong `.filter()` nhận thẳng giá trị `i32`, không phải tham
chiếu, so sánh `x % 2 == 0` viết tự nhiên.

Cộng dồn cả dãy thành một con số:

```rust title=readonly
fn main() {
    let so = vec![1, 2, 3, 4];
    let tong: i32 = so.iter().sum();
    println!("{}", tong);
}
```

```text title=readonly
10
```

`.sum()` không cần closure — nó cộng dồn tất cả phần tử luồng lại,
theo đúng kiểu con số bạn khai ở biến nhận (`i32` ở đây).
::::

::::predict{#doan-ket-qua-chuoi commitOnce}
Byte viết đoạn mã sau, KHÔNG chạy thử:

```rust
fn main() {
    let v = vec![2, 4, 6];
    let tong: i32 = v.iter().map(|x| x + 1).sum();
    println!("{}", tong);
}
```

**Trước khi đọc đáp án**, dòng cuối in ra gì?

:::opt{correct}
`15` — `.map(|x| x + 1)` biến `[2, 4, 6]` thành luồng `[3, 5, 7]`, rồi
`.sum()` cộng dồn: `3 + 5 + 7 = 15`
:::

:::opt
`12` — `.sum()` cộng dồn ba phần tử gốc `[2, 4, 6]`, closure `|x| x + 1`
chỉ có tác dụng lọc, không đổi giá trị từng phần tử
::why
Gần đúng ở việc `2 + 4 + 6 = 12` tính đúng phép cộng của DÃY GỐC — con
số đó không hề bịa ra.

Chỗ lệch: `.map()` không phải `.filter()`. Nó áp closure lên TỪNG phần
tử và THAY THẾ giá trị cũ bằng kết quả — không lọc bớt phần tử nào cả.
Luồng đi vào `.sum()` đã là `[3, 5, 7]` (mỗi phần tử CỘNG THÊM 1),
không còn là `[2, 4, 6]` gốc nữa.
::
:::

:::opt
`13` — cộng dồn dãy gốc trước (`2 + 4 + 6 = 12`), rồi cộng thêm `1` một
lần duy nhất vào kết quả cuối
::why
Gần đúng ở việc bạn đúng là có "cộng thêm 1" ở đâu đó trong phép tính —
con số `1` trong `|x| x + 1` không phải bạn tưởng tượng ra.

Chỗ lệch: `+ 1` nằm TRONG closure của `.map()`, áp dụng cho TỪNG phần
tử của luồng, TRƯỚC khi `.sum()` chạy — không phải một bước cộng thêm
một lần ở cuối, sau khi đã cộng dồn xong. `.map()` chạy trước, biến đổi
cả ba phần tử, rồi `.sum()` mới cộng dồn dãy ĐÃ ĐỔI đó.
::
:::

:::opt
`3` — kết quả là SỐ LƯỢNG phần tử trong `v` (ba phần tử), không phải
tổng giá trị của chúng
::why
Gần đúng ở việc `v` đúng là có ba phần tử — đếm đúng số lượng.

Chỗ lệch: `.sum()` cộng dồn GIÁ TRỊ của luồng, không đếm SỐ LƯỢNG phần
tử. Đếm số lượng là việc của một phương thức khác (`.count()`), không
xuất hiện ở đây. Chuỗi `.iter().map(...).sum()` luôn trả về một tổng
giá trị.
::
:::
::::

::::code{#tong-cac-so-chan}
Viết hàm `tong_so_chan` — nhận một `Vec<i32>` theo giá trị, trả về
TỔNG các số CHẴN trong đó. Dùng `.into_iter()`, `.filter()`, `.sum()`.
Điền đúng MỘT chỗ trống.

```rust title=starter
fn tong_so_chan(v: Vec<i32>) -> i32 {
    v.into_iter().filter(|x| x % ___ == 0).sum()
}
```

```rust title=solution
fn tong_so_chan(v: Vec<i32>) -> i32 {
    v.into_iter().filter(|x| x % 2 == 0).sum()
}
```

```rust title=test
fn main() {
    let so = vec![1, 2, 3, 4, 5, 6];
    let kq = tong_so_chan(so);
    println!("{}", kq);
    assert_eq!(
        kq, 12,
        "tổng các số chẵn trong [1,2,3,4,5,6] phải là 12 (2+4+6) — đang là {}", kq
    );
}
```

:::hints
- kind: attention
  body: Một số CHẴN là số chia hết cho 2, tức số dư của phép chia cho 2 phải bằng 0. Chỗ trống là số bạn chia dư vào.
- kind: strategy
  body: 'Điều kiện giữ lại một phần tử chẵn là "x chia dư cho MỘT SỐ NÀO ĐÓ ra 0" — số đó chính là 2, định nghĩa của số chẵn.'
- kind: one-line
  body: 'Chỗ trống là: 2'
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
Ba phương thức, một dãy giá trị, không một vòng lặp viết tay nào. Đây
mới là hé mở — track sau (functional programming) đào sâu hẳn vào
những công cụ như thế này.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Suốt hai mươi hai bài, mỗi bài dựng đúng MỘT Ý mới: `struct`, `enum`,
move, `&`, `&mut`, `Option`, `Result`, `?`, và giờ là iterator. Nhưng
một chương trình thật không tách rời từng Ý — nó cần NHIỀU Ý cùng lúc,
đúng chỗ, đúng thứ tự.

Bạn đã GHÉP được chưa? Bài sau không dạy gì mới cả — nó chỉ hỏi đúng
câu đó, bằng một chương trình đòi bạn dùng nhiều hơn một Ý cùng lúc.
::::

::::checkpoint{mastery=0.8}
::::
