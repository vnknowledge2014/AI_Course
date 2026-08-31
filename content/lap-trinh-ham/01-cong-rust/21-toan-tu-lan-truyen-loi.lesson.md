---
id: lap-trinh-ham.cong-rust.toan-tu-lan-truyen-loi
title: "`?` — lan truyền lỗi lên trên, không phải xử lý tại chỗ"
summary: "`let x = ham_co_the_loi()?;` — nếu `Err`, HÀM HIỆN TẠI trả về `Err` đó NGAY; nếu `Ok`, lấy giá trị bên trong gán cho `x`, chạy tiếp bình thường. Đã đo thật qua hai tầng hàm lồng nhau. `?` là VIẾT TẮT của một match lặp đi lặp lại."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 21
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.question-mark]
requires: [rs.result]
concepts: [rs.question-mark]
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
Gọi một hàm trả `Result`, rồi hàm gọi nó CŨNG trả `Result` — viết
`match` ở mọi tầng chỉ để "lỗi thì đẩy lên, không lỗi thì lấy ra dùng
tiếp" có cảm giác lặp lại. Hôm nay dấu `?`.
::::

::::explain{#dau-hoi}
Nghĩ tới khuôn lặp bạn sẽ gặp mỗi lần gọi một hàm trả `Result` rồi
muốn TIẾP TỤC dùng giá trị bên trong, viết bằng `match`:

```
let x = match ham_co_the_loi() {
    Ok(gia_tri) => gia_tri,
    Err(loi) => return Err(loi),
};
```

Đọc: nếu `Ok`, lấy giá trị ra gán cho `x`, chạy tiếp; nếu `Err`, thoát
khỏi HÀM HIỆN TẠI ngay, trả `Err` đó lên trên. Khuôn này lặp lại ở gần
như MỌI hàm xử lý lỗi nối tiếp nhau — đủ phổ biến để Rust có một dấu
viết tắt riêng cho nó: **`?`** (dấu hỏi, đặt ngay sau lời gọi hàm).

```
let x = ham_co_the_loi()?;
```

Một dòng thay cho năm dòng `match` ở trên — làm ĐÚNG việc y hệt: `Ok`
thì bóc giá trị gán cho `x`, chạy tiếp; `Err` thì HÀM HIỆN TẠI (không
phải chương trình, không phải `main`) trả về `Err` đó NGAY LẬP TỨC,
bỏ qua mọi dòng còn lại trong hàm.

`?` không phải cú pháp bí ẩn tách biệt — nó CHÍNH LÀ đoạn `match` ở
trên, viết tắt. Điều kiện để dùng được `?`: hàm ĐANG VIẾT phải có kiểu
trả về là `Result` (cùng kiểu lỗi `E`, hoặc kiểu chuyển đổi được) — vì
`?` cần MỘT NƠI để "trả `Err` lên" khi gặp thất bại, và nơi đó chính là
kiểu trả về của hàm hiện tại.
::::

::::example{#lan-truyen-qua-hai-tang}
Hai hàm lồng nhau: `chia` có thể thất bại, `chia_roi_nhan_doi` GỌI
`chia` rồi muốn dùng tiếp kết quả:

```rust title=readonly
fn chia(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("chia cho 0"))
    } else {
        Ok(a / b)
    }
}

fn chia_roi_nhan_doi(a: i32, b: i32) -> Result<i32, String> {
    let x = chia(a, b)?;
    println!("đã tới đây, x = {}", x);
    Ok(x * 2)
}

fn main() {
    let ket_qua = chia_roi_nhan_doi(10, 0);
    println!("{:?}", ket_qua);

    let ket_qua2 = chia_roi_nhan_doi(10, 2);
    println!("{:?}", ket_qua2);
}
```

```text title=readonly
Err("chia cho 0")
đã tới đây, x = 5
Ok(10)
```

Đọc kỹ thứ tự: lần gọi ĐẦU (`10, 0`) — `chia` trả `Err`, `?` khiến
`chia_roi_nhan_doi` trả `Err` đó NGAY, dòng `println!("đã tới đây...")`
**KHÔNG BAO GIỜ CHẠY** — bị bỏ qua hoàn toàn, không phải chạy rồi mới
dừng. Lần gọi THỨ HAI (`10, 2`) — `chia` trả `Ok(5)`, `?` bóc giá trị
`5` ra gán cho `x`, hàm chạy TIẾP bình thường tới dòng cuối, trả về
`Ok(10)`.

`chia_roi_nhan_doi` chưa từng viết chữ `match` nào, nhưng vẫn xử lý
đúng cả hai khả năng của `Result` mà `chia` trả về — `?` đã làm việc
đó thay, ở đúng MỘT ký tự.
::::

::::predict{#dau-hoi-chan-dong-nao commitOnce}
Byte dùng lại đúng chương trình ở phần trên, KHÔNG chạy thử.

**Trước khi đọc đáp án**: nếu gọi `chia_roi_nhan_doi(10, 0)`, dòng
`println!("đã tới đây, x = {}", x);` bên trong hàm CÓ chạy không? Và
chương trình `main` có dừng hẳn (như panic) ngay tại đó không?

:::opt{correct}
Dòng `println!` đó KHÔNG chạy — `?` gặp `Err` từ `chia(10, 0)` khiến
`chia_roi_nhan_doi` thoát ra NGAY, bỏ qua mọi dòng phía sau. Nhưng
`main` KHÔNG dừng hẳn: nó chỉ nhận về giá trị `Err("chia cho 0")` từ
`chia_roi_nhan_doi`, rồi tiếp tục chạy bình thường tới dòng
`println!("{:?}", ket_qua)`
:::

:::opt
Dòng `println!` đó VẪN chạy, vì nó nằm ở một dòng RIÊNG, tách biệt với
dòng có `?` — `?` chỉ ảnh hưởng tới đúng dòng nó xuất hiện, không ảnh
hưởng gì tới các dòng sau
::why
Gần đúng ở việc mỗi dòng lệnh Rust đúng là được xử lý tuần tự, dòng nào
ra dòng đó — một trực giác hợp lý.

Chỗ lệch: `?` không chỉ "ảnh hưởng đúng một dòng" — khi gặp `Err`, nó
khiến CẢ HÀM thoát RA NGAY LẬP TỨC, bỏ qua HOÀN TOÀN phần thân hàm còn
lại, không chỉ dòng chứa nó. `println!` nằm SAU dòng có `?`, nên khi
`?` gặp `Err`, dòng đó không bao giờ tới lượt chạy.
::
:::

:::opt
Chương trình `main` dừng hẳn (giống panic), không in được gì thêm sau
đó — gặp `Err` từ `?` khiến toàn bộ chương trình sập, không chỉ hàm
đang chạy
::why
Gần đúng ở việc bạn liên tưởng `?` với việc dừng lại đột ngột — có
phần đúng, NHƯNG chỉ đúng ở PHẠM VI hàm, không phải toàn chương trình.

Chỗ lệch: `?` chỉ khiến HÀM HIỆN TẠI (`chia_roi_nhan_doi`) trả về sớm —
nó KHÔNG panic, không dừng cả chương trình. Đây chính là khác biệt lớn
với `.unwrap()` (bài trước): `.unwrap()` panic ngay khi gặp `Err`,
dừng hẳn; `?` chỉ LAN TRUYỀN `Err` đó lên trên, để người gọi (ở đây là
`main`) tự quyết định làm gì tiếp — và `main` ở đây chỉ đơn giản in nó
ra rồi chạy tiếp bình thường.
::
:::

:::opt
Dòng `println!` đó chạy, nhưng in ra giá trị `x = 0` — vì `chia(10, 0)`
trả `Err`, và Rust tự gán `0` làm giá trị mặc định cho `x` khi gặp lỗi
::why
Gần đúng ở việc bạn nghĩ tới một "giá trị dự phòng" khi gặp lỗi — quen
thuộc nếu từng dùng ngôn ngữ có giá trị mặc định ngầm khi lỗi.

Chỗ lệch: `?` không có khái niệm "giá trị mặc định thay cho lỗi". Gặp
`Err`, nó không gán gì cho `x` cả — nó khiến hàm thoát NGAY, dòng gán
`x` (và cả dòng `println!` sau đó) chưa từng chạy tới, không có `x`
nào tồn tại để mang giá trị `0` hay bất cứ gì khác trong lần gọi này.
::
:::
::::

::::code{#chia-roi-nhan-doi}
Viết một hàm gọi `chia` (đã có sẵn) bằng `?`, rồi nhân đôi kết quả.
Điền chỗ trống.

```rust title=starter
fn chia(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("chia cho 0"))
    } else {
        Ok(a / b)
    }
}

fn chia_roi_nhan_doi(a: i32, b: i32) -> Result<i32, String> {
    let x = chia(a, b)?;
    Ok(x * ___)
}
```

```rust title=solution
fn chia(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("chia cho 0"))
    } else {
        Ok(a / b)
    }
}

fn chia_roi_nhan_doi(a: i32, b: i32) -> Result<i32, String> {
    let x = chia(a, b)?;
    Ok(x * 2)
}
```

```rust title=test
fn main() {
    let a = chia_roi_nhan_doi(10, 2);
    let b = chia_roi_nhan_doi(10, 0);
    let c = chia_roi_nhan_doi(9, 3);
    println!("{:?}", a);
    println!("{:?}", b);
    assert_eq!(a, Ok(10), "10 chia 2 nhân đôi phải là Ok(10)");
    assert_eq!(b, Err(String::from("chia cho 0")), "chia cho 0 phải lan truyền lỗi nguyên vẹn");
    assert_eq!(c, Ok(6), "9 chia 3 nhân đôi phải là Ok(6)");
}
```

:::hints
- kind: attention
  body: 'Hàm phải NHÂN ĐÔI kết quả của chia(a, b) rồi mới bọc vào Ok. Chỗ trống là con số nhân với x, không phải chính x.'
- kind: strategy
  body: 'x * ___ cần cho ra gấp đôi x, đúng nghĩa "nhân đôi" đề bài yêu cầu. Con số nhân để gấp đôi một giá trị là 2.'
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
  expect: "Ok(10)"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một ký tự, `?`, thay cho cả một khuôn `match` lặp đi lặp lại — lỗi vẫn
được xử lý đủ, chỉ là không còn phải viết ra tay mỗi lần.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Từ đầu track, mọi thứ bạn xử lý là MỘT giá trị tại một thời điểm — một
`i32`, một `Option`, một `Result`. Nhưng phần lớn dữ liệu thật không
đến lẻ tẻ — chúng đến thành DÃY: một `Vec` cả trăm số, cần nhân đôi
từng phần tử, hay chỉ giữ lại những phần tử thoả một điều kiện. Viết
`for` thủ công cho từng việc đó được — nhưng có cách nào NGẮN GỌN hơn,
nói thẳng "áp phép biến đổi này lên MỌI phần tử" không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
