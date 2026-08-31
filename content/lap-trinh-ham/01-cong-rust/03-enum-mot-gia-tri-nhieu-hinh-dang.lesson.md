---
id: lap-trinh-ham.cong-rust.enum-mot-gia-tri-nhieu-hinh-dang
title: "enum — một giá trị là MỘT TRONG NHIỀU hình dạng"
summary: "`enum HinhDang { Tron(f64), ChuNhat(i32, i32) }` — một giá trị kiểu HinhDang luôn là ĐÚNG MỘT trong các biến thể đã liệt kê. `match` trên nó buộc xử lý ĐỦ mọi biến thể — thiếu một nhánh là lỗi biên dịch thật (đã đo: BR0300), không phải gợi ý."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 3
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.enum-match]
requires: [rs.struct]
concepts: [rs.enum-match]
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
`struct` khoá một hình dạng CỐ ĐỊNH. Hôm nay: một kiểu dữ liệu mà giá
trị của nó có thể là MỘT TRONG NHIỀU hình dạng khác nhau.
::::

::::explain{#mot-trong-nhieu-hinh-dang}
Một hình học có thể là hình TRÒN (cần một bán kính, một con số) hoặc
hình CHỮ NHẬT (cần hai cạnh, hai con số) — không bao giờ là cả hai cùng
lúc, và số lượng dữ liệu đi kèm cũng khác nhau tuỳ hình dạng nào.
`struct` không mô tả được điều này: nó chỉ có MỘT hình dạng cố định.

**`enum`** (viết tắt của "enumeration", kiểu liệt kê) giải quyết đúng
việc đó: liệt kê TẤT CẢ hình dạng khả dĩ, gọi mỗi hình dạng là một
**biến thể** (variant):

```
enum HinhDang {
    Tron(f64),
    ChuNhat(i32, i32),
}
```

Đọc dòng này: một giá trị kiểu `HinhDang` LUÔN LUÔN là ĐÚNG MỘT trong
hai biến thể — hoặc `Tron`, mang theo một `f64` (bán kính), hoặc
`ChuNhat`, mang theo hai `i32` (hai cạnh). Không có giá trị `HinhDang`
nào mang CẢ HAI, và cũng không có giá trị `HinhDang` nào KHÔNG mang gì.
Dựng một giá trị bằng tên biến thể, kèm dữ liệu trong ngoặc tròn:

```
let hd = HinhDang::Tron(2.0);
```

Đọc dữ liệu bên trong một `enum` cần **`match`** — so khớp giá trị với
từng biến thể có thể, lấy dữ liệu ra theo tên biến đặt trong ngoặc:

```
match hd {
    HinhDang::Tron(ban_kinh) => ...,
    HinhDang::ChuNhat(canh_a, canh_b) => ...,
}
```

Đây là chỗ `enum` khác hẳn một dãy `if`/`else if` thông thường: Rust
BẮT BUỘC `match` phải xử lý ĐỦ mọi biến thể đã khai trong `enum`. Thiếu
một nhánh — chương trình không được biên dịch, không phải một cảnh báo
lịch sự.
::::

::::example{#day-du-va-thieu-nhanh}
Một `enum HinhDang`, hàm tính diện tích, `match` đủ cả hai nhánh:

```rust title=readonly
enum HinhDang {
    Tron(f64),
    ChuNhat(i32, i32),
}

fn dien_tich(h: HinhDang) -> f64 {
    match h {
        HinhDang::Tron(r) => 3.14 * r * r,
        HinhDang::ChuNhat(a, b) => (a * b) as f64,
    }
}

fn main() {
    let hd1 = HinhDang::Tron(2.0);
    let hd2 = HinhDang::ChuNhat(3, 4);
    println!("{}", dien_tich(hd1));
    println!("{}", dien_tich(hd2));
}
```

```text title=readonly
12.56
12
```

Còn nếu `match` THIẾU một nhánh:

```rust title=readonly
enum HinhDang {
    Tron(f64),
    ChuNhat(i32, i32),
}

fn dien_tich(h: HinhDang) -> f64 {
    match h {
        HinhDang::Tron(r) => 3.14 * r * r,
    }
}
```

```text title=readonly
(không biên dịch được)

lỗi [BR0300]: `match` chưa phủ hết mọi khả năng
 --> dòng 7:5
  |
7 |     match h {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ chưa có nhánh nào bắt `ChuNhat(_, _)`
```

Không cần bạn tự nhớ xem đã xử lý đủ hay chưa — Rust tự đối chiếu danh
sách biến thể đã khai trong `enum` với danh sách nhánh trong `match`,
và chỉ đích danh biến thể nào còn thiếu.

Đây là một điểm đáng chú ý so với TypeScript: `string | number` (union
type, T4.0a bài 15) cũng nói "giá trị này là MỘT TRONG NHIỀU khả năng
đã liệt kê" — giống hệt ý của `enum`. Nhưng TypeScript chỉ ép kiểm tra
LÚC TRUY CẬP một thành viên riêng của từng kiểu (T4.0a bài 16, gọi
`.toUpperCase()` phải thu hẹp `typeof` trước). `enum` của Rust ép SỚM
HƠN: kiểm tra ngay lúc `match`, trước cả khi bạn kịp đọc dữ liệu bên
trong — thiếu một nhánh là lỗi, dù nhánh đó có đọc dữ liệu gì hay
không. Cùng một Ý ("một giá trị, nhiều khả năng đã liệt kê"), khác
nhau ở CHỖ bị ép kiểm, không phải Ở CÓ BỊ ép hay không.
::::

::::predict{#nhanh-nao-thieu commitOnce}
Byte viết một `enum` và một hàm `match` trên nó, KHÔNG chạy thử:

```rust
enum TrangThai {
    DangXuLy,
    HoanThanh(i32),
    Loi(String),
}

fn mo_ta(t: TrangThai) -> String {
    match t {
        TrangThai::DangXuLy => String::from("đang xử lý"),
        TrangThai::HoanThanh(ma) => format!("hoàn thành, mã {}", ma),
    }
}
```

**Trước khi đọc đáp án**, đoạn mã này có biên dịch được không, và vì
sao?

:::opt{correct}
Không — `match` thiếu nhánh cho biến thể `Loi(String)`. `TrangThai` có
BA biến thể (`DangXuLy`, `HoanThanh`, `Loi`), nhưng `match` chỉ xử lý
hai. Rust từ chối biên dịch, chỉ đích danh biến thể còn thiếu
:::

:::opt
Có, biên dịch được — hai nhánh đã viết đủ chi tiết, và biến thể
`Loi(String)` không bắt buộc phải xử lý vì nó đứng CUỐI danh sách khai
trong `enum`
::why
Gần đúng ở việc hai nhánh `DangXuLy` và `HoanThanh` đúng là viết đủ chi
tiết, không thiếu cú pháp gì.

Chỗ lệch: vị trí một biến thể đứng ở ĐÂU trong khai báo `enum` (đầu,
giữa, cuối) không ảnh hưởng gì tới việc nó có BẮT BUỘC xử lý hay
không. Rust đòi xử lý MỌI biến thể, không phân biệt thứ tự khai. Thiếu
`Loi(String)` vẫn bị từ chối thật, đúng mã `BR0300`.
::
:::

:::opt
Có, biên dịch được — `match` chỉ bắt buộc đủ nhánh khi TẤT CẢ biến thể
đều mang dữ liệu; `DangXuLy` không mang dữ liệu gì nên không tính vào
luật bắt buộc
::why
Gần đúng ở việc bạn để ý `DangXuLy` khác hai biến thể kia — nó không
mang dữ liệu đi kèm (biến thể "unit", trong khi `HoanThanh` và `Loi`
là biến thể "tuple", có mang dữ liệu).

Chỗ lệch: luật bắt buộc xử lý ĐỦ nhánh áp dụng như nhau cho MỌI biến
thể, có mang dữ liệu hay không. `DangXuLy` VẪN được xử lý đủ ở đây (có
nhánh `TrangThai::DangXuLy => ...`) — vấn đề nằm ở `Loi(String)` bị bỏ
sót hoàn toàn, không liên quan gì tới việc `DangXuLy` mang dữ liệu hay
không.
::
:::

:::opt
Không — nhưng không phải vì thiếu nhánh; lỗi thật sự là `HoanThanh(ma)`
dùng tên biến `ma` khác với tên đã khai trong `enum` (`HoanThanh(i32)`
không đặt tên `ma` cho tham số)
::why
Gần đúng ở việc bạn để ý cú pháp bên trong nhánh `HoanThanh(ma)` — một
chỗ đáng nghi ngờ khi mới gặp `enum`.

Chỗ lệch: `enum HinhDang { HoanThanh(i32) }` chỉ khai KIỂU của dữ liệu
đi kèm (`i32`), không đặt tên cho nó — tên đó chỉ xuất hiện lúc `match`
BÓC dữ liệu ra, và người viết `match` được tự đặt tên gì cũng được
(`ma`, `x`, bất cứ gì). `HoanThanh(ma)` hoàn toàn hợp lệ. Lỗi thật sự
nằm ở việc thiếu hẳn một nhánh cho `Loi(String)`.
::
:::
::::

::::code{#dien-tich-hinh}
Một `enum HinhDang` như ví dụ trên. Điền chỗ trống để tính đúng diện
tích hình chữ nhật.

```rust title=starter
enum HinhDang {
    Tron(f64),
    ChuNhat(i32, i32),
}

fn dien_tich(h: HinhDang) -> f64 {
    match h {
        HinhDang::Tron(r) => 3.14 * r * r,
        HinhDang::ChuNhat(a, b) => (a * ___) as f64,
    }
}
```

```rust title=solution
enum HinhDang {
    Tron(f64),
    ChuNhat(i32, i32),
}

fn dien_tich(h: HinhDang) -> f64 {
    match h {
        HinhDang::Tron(r) => 3.14 * r * r,
        HinhDang::ChuNhat(a, b) => (a * b) as f64,
    }
}
```

```rust title=test
fn main() {
    let hd = HinhDang::ChuNhat(3, 4);
    let dt = dien_tich(hd);
    println!("{}", dt);
    assert_eq!(
        dt, 12.0,
        "diện tích hình chữ nhật 3x4 phải là 12.0 — đang là {}", dt
    );
}
```

:::hints
- kind: attention
  body: Nhánh ChuNhat bóc ra hai cạnh a và b. Diện tích hình chữ nhật là tích của HAI cạnh — chỗ trống là cạnh còn thiếu trong phép nhân.
- kind: strategy
  body: 'a đã đứng ở vế trái phép nhân (a * ___). Nhánh ChuNhat(a, b) bóc ra đúng hai biến, a và b — biến còn lại cần nhân vào là b.'
- kind: one-line
  body: 'Chỗ trống là: b'
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
Một kiểu, nhiều hình dạng, và `match` không cho bạn quên xử lý bất cứ
hình dạng nào. Trình biên dịch đối chiếu giúp bạn, không phải bạn tự
nhớ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp, mở sang phần quan trọng nhất của track
này.

Ba bài vừa qua, bạn đã tạo ra rất nhiều GIÁ TRỊ: `let d = Diem { ... }`,
`let hd = HinhDang::Tron(2.0)`, `let ten_khach = "Lan"`. Ở Python, bạn
đã học một điều về cái tên và giá trị: một cái tên không GIỮ giá trị,
nó chỉ CHỈ TỚI giá trị — và nhiều cái tên có thể cùng chỉ tới một giá
trị, không luật nào ngăn cả.

Rust có luật nào lên đúng bức tranh "cái tên chỉ tới giá trị" đó
không? Hay Rust để mọi cái tên tự do trỏ vào bất cứ đâu, y hệt Python?

Bài sau trả lời — và đây chính là Ý mà cả track Rust này xoay quanh.
::::

::::checkpoint{mastery=0.8}
::::
