---
id: lap-trinh-ham.cong-rust.result-ok-hoac-err
title: "`Result<T, E>` — thay cho ném ngoại lệ"
summary: "`Result<T, E>` — một `enum` có sẵn khác: `Ok(giá_trị)` hoặc `Err(lỗi)`. Rust không có `try`/`except`/`throw` cho lỗi thông thường — hàm CÓ THỂ THẤT BẠI khai tường minh `-> Result<T, E>`, người gọi buộc phải `match` hoặc `.unwrap()` để lấy giá trị ra."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 20
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.result]
requires: [rs.unwrap-panic]
concepts: [rs.result]
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
`Option` nói "có, hoặc không có gì cả" — không lý do. Hôm nay một kiểu
khác: "có, hoặc thất bại, VÀ ĐÂY LÀ VÌ SAO".
::::

::::explain{#ok-hoac-err}
Nhiều ngôn ngữ xử lý lỗi bằng `try`/`catch`/`throw` (hoặc
`try`/`except` ở Python): một hàm THẤT BẠI bất cứ lúc nào, ném một
ngoại lệ, và ngoại lệ đó bay ngược lên qua bao nhiêu tầng hàm cho tới
khi ai đó bắt được nó — hoặc không ai bắt, chương trình sập.

Rust không có cơ chế đó cho lỗi THÔNG THƯỜNG (panic, bạn vừa gặp ở bài
trước, dành cho tình huống bạn KHÔNG MUỐN chương trình chạy tiếp).
Thay vào đó: **`Result<T, E>`** — một `enum` **CÓ SẴN** khác, cùng họ
với `Option<T>`:

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Hai tham số kiểu: `T` là kiểu của giá trị khi THÀNH CÔNG, `E` là kiểu
của LỖI khi THẤT BẠI. Một hàm CÓ THỂ thất bại khai tường minh ngay
trong CHỮ KÝ:

```
fn chia(a: i32, b: i32) -> Result<i32, String>
```

Đọc chữ ký này: hàm `chia` trả về HOẶC `Ok(i32)` — chia thành công, kèm
kết quả — HOẶC `Err(String)` — thất bại, kèm một `String` MÔ TẢ vì sao.
Đây chính là "chữ ký là lời hứa" bạn đã gặp ở T4.0a: hàm nào khai
`Result` là hàm đang hứa "tôi CÓ THỂ thất bại, và khi thất bại tôi luôn
kèm theo lý do" — ngay trong kiểu, không phải một quy ước ngầm bạn phải
tự nhớ đọc tài liệu mới biết.

Người GỌI một hàm trả `Result` bị buộc đối diện y hệt `Option`: dùng
`match` (đủ cả hai nhánh `Ok`/`Err`) hoặc `.unwrap()` (lấy liều, panic
nếu gặp `Err`) — không có đường nào lấy giá trị ra mà bỏ qua khả năng
thất bại.
::::

::::example{#chia-co-ly-do}
Cùng bài toán chia — lần này lỗi mang theo LÝ DO, không chỉ "không có
gì":

```rust title=readonly
fn chia(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("chia cho 0"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let a = chia(10, 2);
    let b = chia(10, 0);
    println!("{:?}", a);
    println!("{:?}", b);

    match a {
        Ok(x) => println!("kết quả: {}", x),
        Err(e) => println!("lỗi: {}", e),
    }
}
```

```text title=readonly
Ok(5)
Err("chia cho 0")
kết quả: 5
```

So với `Option<i32>` ở hai bài trước: `None` chỉ nói "không có gì",
không nói vì sao. `Err(String::from("chia cho 0"))` nói THẲNG lý do —
`match` trên nhánh `Err(e)` bóc được cả LÝ DO đó ra, không chỉ biết
rằng có lỗi.

Gọi `.unwrap()` trên một `Result` đang là `Err` cũng panic, y hệt
`Option`, và thông báo còn kèm luôn nội dung lỗi:

```rust title=readonly
fn chia(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("chia cho 0"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let x = chia(10, 0).unwrap();
    println!("{}", x);
}
```

```text title=readonly
(chương trình dừng giữa chừng)

lỗi [BR0551]: `unwrap()` gọi trên `Err`
  --> dòng 10:13
   |
10 |     let x = chia(10, 0).unwrap();
   |             ^^^^^^^^^^^^^^^^^^^^ chương trình dừng ở đây
   |
  --> dòng 10:13
   |
10 |     let x = chia(10, 0).unwrap();
   |             ----------- giá trị này là `Err("chia cho 0")`
```

Chẩn đoán chỉ thẳng nội dung `Err` gây ra panic — `"chia cho 0"` — chứ
không chỉ nói suông "gặp lỗi".
::::

::::predict{#ok-hay-err commitOnce}
Byte viết một hàm và ba lời gọi, KHÔNG chạy thử:

```rust
fn nua_neu_khong_am(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err(String::from("số âm không xử lý được"))
    } else {
        Ok(x * 0.5)
    }
}

fn main() {
    println!("{:?}", nua_neu_khong_am(4.0));
    println!("{:?}", nua_neu_khong_am(-9.0));
}
```

**Trước khi đọc đáp án**, dòng in thứ hai in ra gì?

:::opt{correct}
`Err("số âm không xử lý được")` — `nua_neu_khong_am(-9.0)` gặp nhánh
`x < 0.0` đúng, trả về `Err` kèm đúng chuỗi lý do đã viết trong hàm
:::

:::opt
`Ok(-4.5)` — hàm vẫn tính `x * 0.5` như bình thường rồi bọc vào `Ok`,
bất kể `x` âm hay dương
::why
Gần đúng ở việc bạn tính đúng phép nhân `-9.0 * 0.5 = -4.5` — con số
đó không sai.

Chỗ lệch: với `x = -9.0`, điều kiện `x < 0.0` đúng, nên hàm rẽ vào
nhánh `Err` NGAY, dòng `Ok(x * 0.5)` không hề chạy tới. Phép nhân
`-4.5` không bao giờ được tính trong lần gọi này.
::
:::

:::opt
Chương trình dừng lại (panic) ngay tại lời gọi `nua_neu_khong_am(-9.0)`,
vì trả về `Err` từ một hàm luôn khiến chương trình dừng
::why
Gần đúng ở việc bạn liên kết `Err` với việc "có gì đó không ổn" — đúng
tinh thần, `Err` đúng là báo hiệu thất bại.

Chỗ lệch: TRẢ VỀ `Err(...)` từ một hàm không tự động panic gì cả — nó
chỉ là một giá trị `Result` bình thường được trả ra, y hệt `Ok(...)`.
Panic chỉ xảy ra khi có ai đó gọi `.unwrap()` trên một `Err` — ở đây
`main` chỉ `println!("{:?}", ...)`, không gọi `.unwrap()`, nên chương
trình chạy trọn vẹn, in ra cả hai dòng.
::
:::

:::opt
Không biên dịch được — kiểu lỗi `E` trong `Result<T, E>` phải là một
kiểu có sẵn của Rust như `i32`, không được dùng `String` tự viết
::why
Gần đúng ở việc bạn để ý `E` là một tham số kiểu — đúng, `Result<T, E>`
có hai chỗ trống kiểu, không chỉ một.

Chỗ lệch: `E` có thể là BẤT KỲ kiểu nào, kể cả `String` — không có giới
hạn nào bắt `E` phải là kiểu "có sẵn đặc biệt". `String` làm kiểu lỗi
là một lựa chọn RẤT phổ biến trong Rust thật, đúng như đã dùng ở đây.
Đã thử thật, chương trình biên dịch và chạy bình thường.
::
:::
::::

::::code{#tuoi-hop-le-result}
Viết lại bài kiểm tra tuổi, lần này bằng `Result<i32, String>`: `Ok`
kèm tuổi nếu hợp lệ, `Err` kèm LÝ DO nếu không — "tuổi âm" nếu nhỏ hơn
`0`, "tuổi quá lớn" nếu lớn hơn ngưỡng cho phép. Điền chỗ trống.

```rust title=starter
fn kiem_tra_tuoi_result(tuoi: i32) -> Result<i32, String> {
    if tuoi < 0 {
        Err(String::from("tuổi âm"))
    } else if tuoi > ___ {
        Err(String::from("tuổi quá lớn"))
    } else {
        Ok(tuoi)
    }
}
```

```rust title=solution
fn kiem_tra_tuoi_result(tuoi: i32) -> Result<i32, String> {
    if tuoi < 0 {
        Err(String::from("tuổi âm"))
    } else if tuoi > 150 {
        Err(String::from("tuổi quá lớn"))
    } else {
        Ok(tuoi)
    }
}
```

```rust title=test
fn main() {
    let a = kiem_tra_tuoi_result(50);
    let b = kiem_tra_tuoi_result(-5);
    let c = kiem_tra_tuoi_result(200);
    println!("{:?}", a);
    assert_eq!(a, Ok(50), "tuổi 50 hợp lệ, phải là Ok(50)");
    assert_eq!(b, Err(String::from("tuổi âm")), "tuổi -5 phải báo lỗi tuổi âm");
    assert_eq!(c, Err(String::from("tuổi quá lớn")), "tuổi 200 phải báo lỗi tuổi quá lớn");
}
```

:::hints
- kind: attention
  body: 'Ngưỡng "tuổi quá lớn" là bao nhiêu? Đây là cùng ngưỡng đã dùng cho tuổi hợp lệ ở hai bài trước.'
- kind: strategy
  body: 'Nhánh else if tuoi > ___ bắt trường hợp tuổi VƯỢT ngưỡng hợp lệ. Ngưỡng hợp lệ tối đa đã dùng trong track này cho tuổi là 150.'
- kind: one-line
  body: 'Chỗ trống là: 150'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Ok(50)"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`Result<T, E>` không chỉ nói "thất bại" — nó ép mọi thất bại phải mang
theo LÝ DO, ngay trong kiểu dữ liệu, không phải một chuỗi in ra rồi
biến mất.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Khi một hàm gọi MỘT hàm khác cũng trả `Result`, rồi hàm khác đó lại
gọi một hàm thứ ba cũng trả `Result` — viết `match` ở MỌI tầng, chỉ để
"nếu lỗi thì trả lỗi lên, nếu không thì lấy giá trị ra dùng tiếp", có
cảm giác lặp đi lặp lại. Rust có cách viết TẮT cho đúng cái khuôn lặp
đó không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
