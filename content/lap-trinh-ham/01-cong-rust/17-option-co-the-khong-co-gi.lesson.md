---
id: lap-trinh-ham.cong-rust.option-co-the-khong-co-gi
title: "`Option<T>` — có thể có, có thể không có gì cả"
summary: "`Option<T>` là một `enum` có sẵn: `Some(giá_trị)` hoặc `None`. Rust không có `null`/`undefined` riêng — mọi khả năng 'có thể không có gì' đều đi qua `Option`, một kiểu dữ liệu THẬT, không phải một giá trị đặc biệt lén lút."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 17
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.option]
requires: [rs.ownership-borrow-together, rs.enum-match]
concepts: [rs.option]
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
Mọi giá trị bạn vừa ghép move và mượn vào nhau đều CÓ THẬT — không giá
trị nào từng "trống". Hôm nay một câu hỏi khác: giá trị có thể KHÔNG CÓ
GÌ CẢ, Rust nói về điều đó thế nào?
::::

::::explain{#khong-co-null}
Nhiều ngôn ngữ có một giá trị đặc biệt đại diện cho "không có gì" —
`null` ở nhiều ngôn ngữ, `undefined` ở JavaScript/TypeScript. Giá trị
đó len lỏi vào bất cứ đâu, và bạn chỉ biết nó có mặt khi chương trình
đã sập.

Rust không có `null`. Không một dòng Rust nào từng viết ra chữ đó. Thay
vào đó, **`Option<T>`** — một `enum` **CÓ SẴN** trong Rust, không cần tự
khai — nói thẳng khả năng vắng mặt ngay trong kiểu dữ liệu:

```
enum Option<T> {
    Some(T),
    None,
}
```

Đọc dòng này đúng như cách đọc `enum` ở bài trước: một giá trị kiểu
`Option<T>` LUÔN LUÔN là ĐÚNG MỘT trong hai biến thể — `Some(T)`, mang
theo một giá trị kiểu `T` bên trong, hoặc `None`, không mang gì cả.
Chữ `T` là chỗ trống cho MỌI kiểu — `Option<i32>` là "có thể có một
`i32`, có thể không có gì", `Option<String>` là "có thể có một
`String`, có thể không có gì".

Điểm khác biệt cốt lõi so với `null`: `null` là một giá trị NGẦM, có
thể xuất hiện ở bất cứ đâu bạn từng nghĩ là "chắc chắn có giá trị".
`Option<T>` là một **kiểu THẬT**, phải khai TƯỜNG MINH trong chữ ký hàm
hay khai báo biến. Nếu một hàm KHÔNG khai `Option`, giá trị nó trả về
CHẮC CHẮN luôn có — không có đường lén nào để trả về "không có gì" mà
không đổi cả chữ ký.

Đối lập trực tiếp với TypeScript (T4.0a): ở đó, một giá trị có thể
`undefined` chỉ được TypeScript CẢNH BÁO nếu bạn khai đúng kiểu
(`ts.possibly-undefined`) — nhưng bản thân `undefined` vẫn là một giá
trị JavaScript có thật, tồn tại độc lập, len vào được nhiều nơi khác
nhau. Rust đi xa hơn: không có giá trị `undefined`/`null` NÀO tồn tại
trong ngôn ngữ. Muốn khả năng "có thể không có gì" — bạn ĐÒI phải khai
`Option<T>`, không có cách nào lách qua.
::::

::::example{#chia-co-the-khong-chia-duoc}
Một phép chia có thể không thực hiện được — chia cho `0`. Thay vì để
chương trình sập, hàm khai rõ nó CÓ THỂ không trả về gì:

```rust title=readonly
fn chia(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    let ket_qua = chia(10, 2);
    let ket_qua_loi = chia(10, 0);
    println!("{:?}", ket_qua);
    println!("{:?}", ket_qua_loi);
}
```

```text title=readonly
Some(5)
None
```

Chữ ký `-> Option<i32>` là một lời hứa: hàm này CÓ THỂ trả một `i32`
(bọc trong `Some`), CÓ THỂ không trả gì (`None`) — và người GỌI hàm bắt
buộc phải tính tới cả hai khả năng, vì kiểu trả về không còn là `i32`
trần trụi nữa.

Đúng vậy — không còn là `i32` trần trụi. Thử gán thẳng kết quả của
`chia` vào một biến khai kiểu `i32`, bỏ qua lớp bọc `Option`:

```rust title=readonly
fn chia(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    let x: i32 = chia(10, 2);
    println!("{}", x);
}
```

```text title=readonly
(không biên dịch được)

lỗi [BR0303]: khai báo kiểu i32 nhưng giá trị là Option
  --> dòng 10:18
   |
10 |     let x: i32 = chia(10, 2);
   |                  ^^^^^^^^^^^ giá trị này là Option
   |
  --> dòng 10:12
   |
10 |     let x: i32 = chia(10, 2);
   |            --- nhưng ở đây ghi i32
```

`chia` trả về `Option<i32>`, không phải `i32` — dù bên trong `Some` rõ
ràng đang giữ một `i32`, Rust không tự "mở lớp bọc" ra giúp bạn. Phải
LẤY giá trị ra khỏi `Option` bằng một cách tường minh — bài sau
(`match`) và bài sau nữa (`.unwrap()`) là hai cách làm việc đó.
::::

::::predict{#dong-nao-bi-tu-choi commitOnce}
Byte viết một chương trình, KHÔNG chạy thử:

```rust
fn chia(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    let ket_qua: Option<i32> = chia(10, 2);
    let x: i32 = chia(10, 2);
    let y: Option<i32> = Some(5);
    println!("{:?}", ket_qua);
    println!("{:?}", y);
}
```

**Trước khi đọc đáp án**, dòng nào bị từ chối, và vì sao?

:::opt{correct}
Dòng khai `let x: i32 = chia(10, 2);` — `chia` trả về `Option<i32>`,
không phải `i32`. Hai dòng còn lại (`ket_qua` và `y`) đều gán một
`Option<i32>` vào một biến khai đúng `Option<i32>`, hợp lệ
:::

:::opt
Dòng `let ket_qua: Option<i32> = chia(10, 2);` — muốn gán được, phải
gọi `.unwrap()` trước để lấy giá trị ra khỏi lớp bọc `Option`
::why
Gần đúng ở việc `.unwrap()` đúng là một cách LẤY giá trị ra khỏi
`Option` — bạn nhớ đúng khái niệm sẽ gặp ở bài sau.

Chỗ lệch: `ket_qua` khai kiểu `Option<i32>`, và `chia(10, 2)` cũng trả
về `Option<i32>` — hai vế CÙNG kiểu, gán trực tiếp hợp lệ, không cần mở
lớp bọc gì cả. `.unwrap()` chỉ cần khi muốn lấy giá trị `i32` THUẦN ra,
như dòng `x` đang cố làm (và bị từ chối vì chưa làm điều đó).
::
:::

:::opt
Dòng `let y: Option<i32> = Some(5);` — `Some(5)` phải viết tường minh
là `Some::<i32>(5)` thì Rust mới biết kiểu bên trong là gì
::why
Gần đúng ở việc bạn để ý `Some(5)` không tự nói rõ `5` là kiểu gì —
một nghi ngờ hợp lý khi mới gặp kiểu có tham số như `Option<T>`.

Chỗ lệch: Rust suy luận được kiểu bên trong `Some(...)` từ chú thích
kiểu bên trái (`Option<i32>`), y hệt cách nó suy luận kiểu biến thường
không cần annotate. Không cần viết tường minh `Some::<i32>(5)`. Dòng
`y` hợp lệ, đã thử thật.
::
:::

:::opt
Không dòng nào bị từ chối — `Option<i32>` và `i32` là hai cách viết
khác nhau của cùng một kiểu, Rust tự quy đổi qua lại khi cần
::why
Gần đúng ở cảm giác `Option<i32>` "gần giống" `i32` — bên trong `Some`
đúng là một `i32` thật.

Chỗ lệch: "gần giống" không phải "cùng một kiểu". `Option<i32>` là một
`enum` bọc quanh `i32`, hai kiểu HOÀN TOÀN khác nhau với Rust — không
có quy đổi ngầm nào cả, đúng luật đã gặp từ những bài đầu track này
(Rust không tự chuyển đổi kiểu ngầm). Dòng `x` vẫn bị từ chối thật,
đúng mã `BR0303`.
::
:::
::::

::::code{#tuoi-hop-le}
Một điểm thi hợp lệ nếu nằm trong khoảng từ `0` đến `150`. Viết hàm trả
về `Option<i32>`: `Some(tuổi)` nếu hợp lệ, `None` nếu không. Điền chỗ
trống.

```rust title=starter
fn kiem_tra_tuoi(tuoi: i32) -> Option<i32> {
    if tuoi >= 0 && tuoi <= 150 {
        Some(___)
    } else {
        None
    }
}
```

```rust title=solution
fn kiem_tra_tuoi(tuoi: i32) -> Option<i32> {
    if tuoi >= 0 && tuoi <= 150 {
        Some(tuoi)
    } else {
        None
    }
}
```

```rust title=test
fn main() {
    let a = kiem_tra_tuoi(50);
    let b = kiem_tra_tuoi(-5);
    let c = kiem_tra_tuoi(100);
    println!("{:?}", a);
    println!("{:?}", b);
    assert_eq!(a, Some(50), "tuổi 50 hợp lệ, phải trả về Some(50)");
    assert_eq!(b, None, "tuổi -5 không hợp lệ, phải trả về None");
    assert_eq!(c, Some(100), "tuổi 100 hợp lệ, phải trả về Some(100)");
}
```

:::hints
- kind: attention
  body: Nhánh Some(___) chạy khi tuổi ĐÃ hợp lệ (điều kiện if đã đúng). Giá trị cần bọc vào Some là chính cái tuổi đó — đừng bịa một con số cố định.
- kind: strategy
  body: Tham số của hàm tên là tuoi. Khi điều kiện if đúng, tuoi chính là giá trị hợp lệ cần trả về, bọc trong Some(...).
- kind: one-line
  body: 'Chỗ trống là: tuoi'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Some(50)"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`Option<T>` không phải một mẹo — nó là một kiểu dữ liệu thật, với đúng
hai hình dạng. Khả năng "không có gì" giờ nằm ngay trong chữ ký hàm,
không lén lút ở đâu cả.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa THẤY `Option<i32>` không thể gán thẳng vào một biến `i32` —
phải lấy giá trị ra bằng cách nào đó tường minh. Nhưng lấy ra như thế
nào, để Rust vẫn ép bạn xử lý ĐỦ cả hai khả năng (`Some` lẫn `None`),
không cho bạn âm thầm quên mất trường hợp rỗng?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
