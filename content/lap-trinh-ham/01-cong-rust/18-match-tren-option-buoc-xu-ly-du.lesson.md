---
id: lap-trinh-ham.cong-rust.match-tren-option-buoc-xu-ly-du
title: "`match` trên Option buộc xử lý ĐỦ cả hai khả năng"
summary: "`match ket_qua { Some(x) => ..., None => ... }` — exhaustiveness (bài enum) áp dụng lên `Option`. THIẾU nhánh `None` là lỗi biên dịch thật (đã đo: BR0300), không phải một trường hợp bạn tự quên rồi chương trình sập lúc chạy."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 18
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.option-match]
requires: [rs.option]
concepts: [rs.option-match]
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
Bài trước: `Option<i32>` không tự "mở lớp bọc" ra thành `i32`. Hôm nay
công cụ để mở lớp bọc đó ra — theo cách Rust không cho bạn quên mất một
khả năng nào.
::::

::::explain{#match-tren-option}
`Option<T>` là một `enum` — bạn đã biết cách xử lý `enum` từ hai bài
trước: dùng `match`, liệt kê từng biến thể. `Option` chỉ có đúng hai
biến thể, nên `match` trên nó chỉ cần đúng hai nhánh:

```
match ket_qua {
    Some(x) => ...,   // có giá trị, x là giá trị BÊN TRONG Some
    None => ...,      // không có gì
}
```

Luật **exhaustiveness** (xử lý đủ mọi biến thể) bạn đã gặp ở bài `enum`
áp dụng NGUYÊN VẸN lên `Option`: THIẾU nhánh `None` — dù bạn "chắc
chắn" giá trị luôn có, dù bạn chỉ đơn giản quên — Rust từ chối biên
dịch, không đợi tới lúc chạy mới lộ ra.

Đây là khác biệt quan trọng so với kiểm tra `null`/`undefined` ở nhiều
ngôn ngữ khác: ở đó, quên kiểm tra là một lỗi LÚC CHẠY (`NullPointer-
Exception`, `TypeError: Cannot read property of undefined`) — chương
trình vẫn biên dịch/chạy được, và bạn chỉ biết mình quên khi người
dùng thật gặp đúng ca rỗng đó, thường là sau khi đã phát hành. Rust
biến "quên xử lý rỗng" thành lỗi biên dịch — bạn đối diện nó ngay lúc
viết, không phải lúc người dùng gặp phải.
::::

::::example{#du-va-thieu-nhanh}
Một hàm mô tả kết quả kiểm tra tuổi (bài trước), xử lý đủ cả hai
biến thể:

```rust title=readonly
fn mo_ta_tuoi(ket_qua: Option<i32>) -> String {
    match ket_qua {
        Some(t) => format!("hợp lệ: {}", t),
        None => String::from("không hợp lệ"),
    }
}

fn main() {
    println!("{}", mo_ta_tuoi(Some(50)));
    println!("{}", mo_ta_tuoi(None));
}
```

```text title=readonly
hợp lệ: 50
không hợp lệ
```

Nhánh `Some(t)` **bóc** giá trị bên trong ra, đặt tên `t` — tên này tự
chọn, không liên quan gì tới tên tham số của hàm gốc. Nhánh `None`
không bóc gì cả, vì `None` không mang dữ liệu.

Còn nếu `match` THIẾU nhánh `None`:

```rust title=readonly
fn mo_ta_tuoi(ket_qua: Option<i32>) -> String {
    match ket_qua {
        Some(t) => format!("hợp lệ: {}", t),
    }
}
```

```text title=readonly
(không biên dịch được)

lỗi [BR0300]: `match` chưa phủ hết mọi khả năng
 --> dòng 2:5
  |
2 |     match ket_qua {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ chưa có nhánh nào bắt `None`
```

Đúng mã lỗi `BR0300` đã gặp ở bài `enum` — `Option` không phải một
ngoại lệ được ưu ái, nó là một `enum` như mọi `enum` khác, chịu đúng
luật exhaustiveness đó.
::::

::::predict{#ham-nao-khong-bien-dich commitOnce}
Byte viết hai hàm, KHÔNG chạy thử:

```rust
fn ten_nguoi_dung(id: Option<i32>) -> String {
    match id {
        Some(x) => format!("nguoi dung {}", x),
        None => String::from("chua dang nhap"),
    }
}

fn trang_thai(diem: Option<i32>) -> String {
    match diem {
        Some(d) => format!("diem: {}", d),
    }
}
```

**Trước khi đọc đáp án**, hàm nào KHÔNG biên dịch được, và vì sao?

:::opt{correct}
`trang_thai` — `match` của nó chỉ có nhánh `Some(d)`, thiếu hẳn nhánh
`None`. `ten_nguoi_dung` xử lý đủ cả hai biến thể (`Some`, `None`),
biên dịch được bình thường
:::

:::opt
`ten_nguoi_dung` — nhánh `Some(x)` trả về bằng `format!`, còn nhánh
`None` trả về bằng `String::from`, hai cách viết khác nhau nên hai
nhánh trả về hai kiểu khác nhau
::why
Gần đúng ở việc bạn để ý hai nhánh dùng hai CÚ PHÁP khác nhau để tạo ra
chuỗi — một quan sát tinh ý.

Chỗ lệch: `format!(...)` và `String::from(...)` đều cho ra cùng MỘT
kiểu — `String`. Cú pháp tạo ra giá trị khác nhau không có nghĩa là
KIỂU của giá trị khác nhau. Cả hai nhánh của `ten_nguoi_dung` cùng trả
`String`, khớp đúng kiểu trả về của hàm — hợp lệ, đã thử thật.
::
:::

:::opt
Cả hai hàm đều không biên dịch được — thiếu nhánh `_ => ...` để bắt các
trường hợp còn lại, `match` nào cũng cần một nhánh mặc định như vậy
::why
Gần đúng ở việc bạn nhớ `_` là một cách hợp lệ để "gom" các trường hợp
còn lại trong `match` — đúng, nó tồn tại và hữu ích khi có NHIỀU biến
thể chưa liệt kê hết.

Chỗ lệch: `_` chỉ BẮT BUỘC khi còn biến thể chưa được nêu TÊN RIÊNG.
`Option` chỉ có hai biến thể; liệt đủ tên riêng cho cả hai (`Some(...)`
và `None`) là đủ để thoả exhaustiveness, không cần thêm `_`.
`ten_nguoi_dung` đã liệt đủ tên riêng, biên dịch được bình thường —
vấn đề chỉ nằm ở `trang_thai`, nơi tên riêng `None` bị bỏ sót hoàn
toàn.
::
:::

:::opt
Không hàm nào lỗi — nếu `match` thiếu một nhánh, Rust tự hiểu nhánh đó
trả về một `String` rỗng làm giá trị mặc định
::why
Gần đúng ở việc bạn tin Rust "lo liệu" giúp trường hợp thiếu sót — một
phản xạ tự nhiên nếu quen ngôn ngữ có giá trị mặc định ngầm.

Chỗ lệch: Rust không có giá trị mặc định ngầm nào cho một nhánh `match`
bị thiếu. Nó từ chối biên dịch thẳng, chỉ đích danh biến thể còn thiếu
— đúng như `trang_thai` bị từ chối thật với mã `BR0300`.
::
:::
::::

::::code{#nhan-doi-hoac-khong}
Một hàm nhận `Option<i32>`: nếu có giá trị, NHÂN ĐÔI nó rồi trả về; nếu
không có gì, trả về `0`. Điền chỗ trống.

```rust title=starter
fn gap_doi_hoac_khong(ket_qua: Option<i32>) -> i32 {
    match ket_qua {
        Some(x) => x * ___,
        None => 0,
    }
}
```

```rust title=solution
fn gap_doi_hoac_khong(ket_qua: Option<i32>) -> i32 {
    match ket_qua {
        Some(x) => x * 2,
        None => 0,
    }
}
```

```rust title=test
fn main() {
    let a = gap_doi_hoac_khong(Some(5));
    let b = gap_doi_hoac_khong(None);
    let c = gap_doi_hoac_khong(Some(7));
    println!("{}", a);
    println!("{}", b);
    assert_eq!(a, 10, "Some(5) nhân đôi phải ra 10");
    assert_eq!(b, 0, "None phải trả về 0");
    assert_eq!(c, 14, "Some(7) nhân đôi phải ra 14");
}
```

:::hints
- kind: attention
  body: Nhánh Some(x) phải NHÂN ĐÔI giá trị x bóc ra được — chỗ trống là con số nhân với x, không phải chính x hay 0.
- kind: strategy
  body: 'x * ___ cần cho ra gấp đôi x. Con số nhân vào để "gấp đôi" một giá trị là 2.'
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
  expect: "10"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai nhánh, không thiếu nhánh nào — `match` buộc bạn đối diện cả `Some`
lẫn `None` ngay lúc viết, không phải lúc chương trình chạy tới đúng ca
rỗng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`match` an toàn, nhưng đòi viết ra CẢ HAI nhánh, mỗi lần. Có những chỗ
bạn tự tin gần như tuyệt đối rằng giá trị LUÔN có — viết đủ hai nhánh
mỗi lần cho một trường hợp gần như không bao giờ xảy ra có cảm giác
thừa thãi. Rust có cách nào lấy giá trị ra NGAY, chấp nhận rủi ro, khi
bạn thật sự chắc chắn không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
