---
id: lap-trinh-ham.cong-rust.struct-nhom-du-lieu-co-ten
title: "struct — nhóm dữ liệu lại dưới một cái tên"
summary: "`struct Diem { x: i32, y: i32 }` gói nhiều trường dữ liệu lại dưới một cái tên. Khởi tạo phải khai ĐỦ mọi trường — thiếu một trường bị Rust từ chối biên dịch (đã đo thật: BR0323). `impl Diem { fn tong(&self) -> i32 { ... } }` gắn một phương thức, gọi bằng `d.tong()`."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 2
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.struct]
requires: [rs.syntax-bridge]
concepts: [rs.struct]
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
Bài trước, mọi dữ liệu là một giá trị đơn lẻ: một chuỗi, một số. Hôm
nay: gói NHIỀU mẩu dữ liệu lại dưới MỘT cái tên.
::::

::::explain{#goi-du-lieu-lai}
Một khách hàng có tên VÀ điểm — hai mẩu dữ liệu đi cùng nhau. Viết hai
biến rời `ten_khach` và `so_diem` thì được, nhưng không có gì buộc
chúng đi CHUNG với nhau: truyền một khách hàng vào hàm nghĩa là truyền
hai tham số rời rạc, không phải MỘT thứ.

**`struct`** (viết tắt của "structure", cấu trúc) giải quyết đúng việc
đó: khai một hình dạng dữ liệu có tên, liệt kê từng trường và kiểu của
nó:

```
struct Diem {
    x: i32,
    y: i32,
}
```

Từ giờ `Diem` là một KIỂU dữ liệu mới, y hệt `i32` hay `String` — chỉ
khác là kiểu này TỰ ĐỊNH NGHĨA bằng cách liệt kê các trường. Khởi tạo
một giá trị kiểu `Diem` bằng đúng tên struct, theo sau là cặp ngoặc
nhọn liệt kê giá trị cho TỪNG trường:

```
let d = Diem { x: 3, y: 4 };
```

Đọc trường bằng dấu chấm: `d.x`, `d.y`.

Muốn gắn một HÀNH VI vào struct — một phép tính chỉ có ý nghĩa với dữ
liệu của nó — dùng khối `impl` (viết tắt của "implementation"):

```
impl Diem {
    fn tong(&self) -> i32 {
        self.x + self.y
    }
}
```

Một hàm khai bên trong `impl Diem` gọi là **phương thức** (method), gọi
bằng `d.tong()` thay vì `tong(d)`. Tham số đầu tiên `&self` là cách
phương thức nhận CHÍNH giá trị đang gọi nó — `self.x` bên trong thân
hàm chính là `d.x` lúc gọi từ `d.tong()`. Dấu `&` đứng trước `self` ở
đây CHƯA cần hiểu sâu — coi nó như một phần cú pháp bắt buộc để viết
được một phương thức chỉ ĐỌC dữ liệu. Track này sẽ quay lại giải thích
đúng ý nghĩa của `&self` khi tới đúng lúc.
::::

::::example{#du-truong-thieu-truong}
Một `struct Diem`, khởi tạo đủ hai trường, đọc trường và gọi phương
thức:

```rust title=readonly
struct Diem {
    x: i32,
    y: i32,
}

impl Diem {
    fn tong(&self) -> i32 {
        self.x + self.y
    }
}

fn main() {
    let d = Diem { x: 3, y: 4 };
    println!("{} {}", d.x, d.y);
    println!("{}", d.tong());
}
```

```text title=readonly
3 4
7
```

Còn nếu khởi tạo mà THIẾU một trường:

```rust title=readonly
struct Diem {
    x: i32,
    y: i32,
}

fn main() {
    let d = Diem { x: 3 };
    println!("{}", d.x);
}
```

```text title=readonly
(không biên dịch được)

lỗi [BR0323]: thiếu trường `y` khi dựng `Diem`
 --> dòng 7:13
  |
7 |     let d = Diem { x: 3 };
  |             ^^^^^^^^^^^^^ chưa gán giá trị cho `y`
```

Rust không có `null` và không tự điền giá trị mặc định cho một trường
bị bỏ sót. Dựng một `Diem` mà thiếu `y` bị từ chối NGAY LÚC BIÊN DỊCH —
không có chuyện chương trình chạy rồi mới phát hiện `d.y` rỗng.
::::

::::predict{#dong-nao-bi-tu-choi commitOnce}
Byte viết một `struct` và ba giá trị, KHÔNG chạy thử:

```rust
struct SanPham {
    ten: String,
    gia: i32,
}

fn main() {
    let a = SanPham { ten: String::from("Bánh"), gia: 20000 };
    let b = SanPham { ten: String::from("Kẹo") };
    let c = SanPham { gia: 10000, ten: String::from("Nước") };
}
```

**Trước khi đọc đáp án**, dòng khai nào bị Rust từ chối, và vì sao?

:::opt{correct}
Dòng khai `b` — `SanPham { ten: String::from("Kẹo") }` thiếu trường
`gia`, mà `SanPham` đòi có cả `ten` VÀ `gia`. `a` và `c` đều đủ hai
trường, đúng kiểu từng trường, nên hợp lệ
:::

:::opt
Dòng khai `a` — `SanPham` được định nghĩa trước cả ba biến, nên biến
ĐẦU TIÊN dùng struct này (`a`) luôn bị Rust kiểm nghiêm ngặt hơn hai
biến sau
::why
Gần đúng ở việc bạn nghĩ tới THỨ TỰ khai báo có thể ảnh hưởng.

Chỗ lệch: Rust kiểm MỌI giá trị khai kiểu `SanPham` bằng đúng một luật
như nhau — không có biến nào bị kiểm "nghiêm hơn" chỉ vì đứng trước.
`a` có đủ `ten` và `gia`, đúng kiểu cả hai — hợp lệ, đã thử thật.
::
:::

:::opt
Dòng khai `c` — các trường được liệt kê SAI THỨ TỰ (`gia` trước `ten`,
trong khi struct khai `ten` trước `gia`), nên bị từ chối
::why
Gần đúng ở việc bạn để ý `c` viết các trường theo thứ tự khác struct đã
khai — một quan sát đúng.

Chỗ lệch: Rust không quan tâm THỨ TỰ liệt kê trường lúc khởi tạo, chỉ
quan tâm ĐỦ TÊN và ĐÚNG KIỂU. `c` liệt kê `gia` trước `ten` nhưng vẫn
đủ cả hai, đúng kiểu cả hai — hợp lệ, đã thử thật.
::
:::

:::opt
Không dòng nào bị từ chối — struct chỉ là một GỢI Ý cho người đọc mã
nguồn về hình dạng dữ liệu, không phải một luật thật sự được kiểm khi
biên dịch
::why
Gần đúng ở việc struct đúng là giúp NGƯỜI ĐỌC hiểu hình dạng dữ liệu
nhanh hơn — đó là một lợi ích có thật.

Chỗ lệch: lợi ích đó không thay thế việc KIỂM THẬT. Thiếu một trường
khi khởi tạo một giá trị kiểu struct bị trình biên dịch từ chối, đã đo
bằng đúng mã lỗi `BR0323` — dòng khai `b` (thiếu `gia`) bị chặn thật.
::
:::
::::

::::code{#tong-toa-do}
Một điểm 2D với hai toạ độ, và một phương thức tính tổng toạ độ. Điền
chỗ trống để phương thức tính ĐÚNG.

```rust title=starter
struct Diem {
    x: i32,
    y: i32,
}

impl Diem {
    fn tong(&self) -> i32 {
        self.x + ___
    }
}
```

```rust title=solution
struct Diem {
    x: i32,
    y: i32,
}

impl Diem {
    fn tong(&self) -> i32 {
        self.x + self.y
    }
}
```

```rust title=test
fn main() {
    let d = Diem { x: 3, y: 4 };
    println!("{}", d.tong());
    assert_eq!(
        d.tong(), 7,
        "d.tong() phải là 7 (3 + 4) — đang là {}", d.tong()
    );
}
```

:::hints
- kind: attention
  body: tong() phải cộng CẢ HAI trường của struct — self.x và self.y. Chỗ trống là trường còn thiếu.
- kind: strategy
  body: 'self.x đã có sẵn ở vế trái phép cộng. Diem có đúng hai trường, x và y — trường còn lại cần cộng vào là self.y.'
- kind: one-line
  body: 'Chỗ trống là: self.y'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "7"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai trường, một cái tên, một phương thức đọc cả hai. Thiếu một trường
lúc khởi tạo — Rust biết ngay, trước khi chương trình kịp chạy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`Diem` chỉ có MỘT hình dạng cố định: luôn luôn có `x` và `y`, không hơn
không kém. Nhưng có những dữ liệu không có hình dạng cố định như vậy —
một hình học có thể là hình TRÒN (cần một bán kính) HOẶC hình CHỮ NHẬT
(cần hai cạnh), không bao giờ là cả hai cùng lúc, và số trường cần thiết
cũng khác nhau tuỳ hình dạng nào.

`struct` không mô tả được kiểu dữ liệu "một trong nhiều hình dạng khác
nhau" đó. Rust có công cụ nào cho việc này? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
