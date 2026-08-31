---
id: lap-trinh-ham.cong-rust.cung-mot-chuong-trinh-ba-ngon-ngu
title: "Cùng một chương trình, giờ là ngôn ngữ thứ ba"
summary: "Cùng một Ý — biến giữ giá trị, hàm là một khối lệnh gọi lại được — giờ được gõ bằng cú pháp Rust: `fn main()`, `let`, `println!(\"{}\", x)`, dấu `;` cuối dòng. Khác biệt lớn nhất so với hai ngôn ngữ trước: `let` một mình mặc định KHÔNG cho gán lại, phải thêm `mut`."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 1
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.syntax-bridge]
requires: [core.variable, core.function-def]
concepts: [rs.syntax-bridge]
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
Mười tám bài TypeScript vừa khép lại một cổng. Giờ đổi ngôn ngữ lần thứ
hai — Rust, ngôn ngữ thứ ba của khoá. Byte dịch lại đúng một chương
trình nhỏ, ngay trước mắt bạn.
::::

::::explain{#nam-cho-khac-nhau}
Track này không dạy lại tư duy nào cả. Biến vẫn giữ giá trị. Hàm vẫn là
một khối lệnh có tên, gọi lại được nhiều lần. Bạn đã đổi cú pháp một
lần rồi, ở cổng TypeScript — lần này quen tay hơn.

Năm chỗ khác cách gõ, gặp ngay trong một chương trình Rust nhỏ nhất:

1. **Mọi chương trình bắt đầu từ một hàm tên `fn main()`** — không có
   dòng lệnh nào được phép nằm trơ trọi ngoài một hàm. Python chạy
   thẳng từ dòng đầu tệp; Rust luôn cần một điểm vào rõ ràng.
2. **Khai biến bằng `let`** — cú pháp giống TypeScript. Nhưng có một
   khác biệt QUAN TRỌNG: `let` một mình, không thêm gì, mặc định
   **KHÔNG cho gán lại**. Muốn đổi giá trị sau này, phải viết
   `let mut` — `mut` (viết tắt của "mutable", khả biến) là từ khoá mở
   khoá cho việc gán lại. Không có `mut` thì `let` gần giống `const`
   của TypeScript hơn — chỉ khác `const` không có anh em `let` đứng
   cạnh, còn Rust dùng ĐÚNG MỘT từ khoá `let`, thêm `mut` mới đổi hành
   vi.
3. **Mỗi câu lệnh kết thúc bằng dấu chấm phẩy** `;` — giống TypeScript,
   khác Python.
4. **Hàm khai bằng từ khoá `fn`**, không phải `def` hay `function`.
5. **In ra màn hình bằng `println!(...)`** — có dấu `!` sau tên, vì đây
   là một **macro** (một cấu trúc mở rộng mã lúc biên dịch, không phải
   hàm bình thường), không phải `print(...)` hay `console.log(...)`.
   Muốn chèn một giá trị vào chuỗi, dùng cặp ngoặc nhọn rỗng `{}` làm
   chỗ trống, rồi liệt kê giá trị theo sau, cách nhau dấu phẩy:
   `println!("Điểm: {}", so_diem)`.

Trong bài này, không biến số nào cần viết kiểu tường minh (`: i32`) —
Rust tự suy luận được kiểu từ giá trị gán, y hệt cách TypeScript suy
luận khi bạn không viết dấu hai chấm.
::::

::::example{#doi-mot-chuong-trinh-nho}
Cùng một câu chuyện — tên khách cố định, số điểm sẽ đổi — viết bằng
Python rồi bằng Rust:

```python title=readonly
ten_khach = "Lan"
so_diem = 100

so_diem = so_diem + 20

def chao_khach():
    print("Khách:", ten_khach)
    print("Điểm:", so_diem)

chao_khach()
```

```rust title=readonly
fn main() {
    let ten_khach = "Lan";
    let mut so_diem = 100;

    so_diem = so_diem + 20;

    chao_khach(ten_khach, so_diem);
}

fn chao_khach(ten: &str, diem: i32) {
    println!("Khách: {}", ten);
    println!("Điểm: {}", diem);
}
```

```text title=readonly
Khách: Lan
Điểm: 120
```

Cùng một kết quả, ĐÚNG như nhau. `ten_khach` không đổi trong suốt
chương trình nên `let` một mình là đủ; `so_diem` bị cộng thêm ở dòng
thứ ba nên bắt buộc phải viết `let mut` — thiếu `mut`, Rust từ chối
biên dịch ngay dòng gán lại. Toàn bộ chương trình nằm trong `fn main()`
— không dòng lệnh nào được phép đứng một mình ngoài một hàm. Hàm
`chao_khach` nhận `ten` (tham số kiểu `&str`, một dạng chuỗi mượn — bài
sau của track sẽ giải thích dấu `&`, ở đây chỉ cần biết đó là cách một
hàm Rust nhận một chuỗi) và `diem` (kiểu `i32`, số nguyên), rồi in bằng
`println!` với `{}` làm chỗ trống.
::::

::::predict{#gan-lai-thieu-mut commitOnce}
Đoạn Python này khai một biến rồi GÁN LẠI nó ngay dòng sau:

```python
so_du = 100
so_du = so_du + 50
```

Byte dịch nó sang Rust. **Trước khi đọc đáp án**, cách dịch nào dưới
đây đúng?

:::opt{correct}
```
let mut so_du = 100;
so_du = so_du + 50;
```
— biến này bị gán lại ở dòng sau, nên bắt buộc phải thêm `mut` ngay từ
lúc khai báo
:::

:::opt
```
let so_du = 100;
so_du = so_du + 50;
```
— `let` là đủ để khai một biến hợp lệ, dòng gán lại phía sau không cần
thêm gì khác
::why
Gần đúng ở việc `let so_du = 100;` đúng là một cách khai biến hợp lệ
trong Rust — dòng đó tự nó không sai.

Chỗ lệch: `let` một mình mặc định KHÔNG cho gán lại. Dòng thứ hai là
một phép GÁN LẠI, và Rust từ chối đúng phép gán đó — đã thử thật, Byte
báo "không gán lại được cho `so_du`" (mã `BR0400`), chỉ ra thẳng dòng
khai `let so_du` phía trên là nơi biến này bị khoá bất biến. Thiếu
`mut` là nguyên nhân duy nhất.
::
:::

:::opt
```
so_du = 100;
so_du = so_du + 50;
```
— không cần từ khoá nào cả, viết thẳng phép gán như Python
::why
Gần đúng ở việc Python đúng là viết y hệt vậy — một dòng gán trần,
không từ khoá nào đứng trước.

Chỗ lệch: Rust không có kiểu gán trần cho một cái tên CHƯA TỪNG khai
báo bằng `let`. Đã thử thật: Byte báo "không tìm thấy tên `so_du`" (mã
`BR0340`) ngay ở dòng gán đầu tiên — với Rust, dòng đó không phải "gán
giá trị cho một cái tên có sẵn", mà là nhắc tới một cái tên chưa từng
tồn tại.
::
:::

:::opt
```
let mut so_du = 100
so_du = so_du + 50;
```
— dùng `mut` đúng chỗ, chỉ bỏ bớt dấu `;` ở dòng khai vì đằng nào dòng
sau cũng có dấu `;` rồi
::why
Gần đúng ở việc `mut` đã được thêm đúng chỗ — đúng thứ track này vừa
nhấn mạnh.

Chỗ lệch: mỗi câu lệnh Rust cần dấu `;` CỦA RIÊNG NÓ, không dùng chung
với dòng sau. Đã thử thật: Byte báo "thiếu dấu `;` ở cuối câu lệnh" (mã
`BR0109`), chỉ thẳng vào cuối dòng khai `let mut so_du = 100`.
::
:::
::::

::::code{#dich-don-hang}
Một đơn hàng nhỏ: tên khách cố định, điểm thưởng đổi sau một lần cộng
thêm. Viết hàm mô tả điểm, rồi gọi nó — điền đúng MỘT chỗ trống.

```rust title=starter
fn mo_ta_diem(ten_khach: &str, so_diem: i32) -> String {
    format!("{} co {} diem", ten_khach, ___)
}
```

```rust title=solution
fn mo_ta_diem(ten_khach: &str, so_diem: i32) -> String {
    format!("{} co {} diem", ten_khach, so_diem)
}
```

```rust title=test
fn main() {
    let ten_khach = "Mai";
    let mut so_diem = 200;

    so_diem = so_diem + 30;

    let dong_mo_ta = mo_ta_diem(ten_khach, so_diem);

    println!("{}", dong_mo_ta);
    assert_eq!(
        dong_mo_ta, "Mai co 230 diem",
        "dong_mo_ta phải là \"Mai co 230 diem\" — đang là {}", dong_mo_ta
    );
}
```

:::hints
- kind: attention
  body: Một chỗ trống — hàm mo_ta_diem phải chèn ĐÚNG giá trị của tham số so_diem vào chỗ trống thứ hai của format!.
- kind: strategy
  body: 'format! có hai chỗ trống {} theo đúng thứ tự hai tham số ten_khach và so_diem. Chỗ trống thứ nhất đã điền ten_khach, chỗ trống thứ hai (chỗ bạn cần điền) chính là so_diem.'
- kind: one-line
  body: 'Chỗ trống là: so_diem'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Mai co 230 diem"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chương trình Rust đầu tiên của bạn vừa chạy — `fn main()`, `let mut`,
`println!`, dấu chấm phẩy, đúng cả năm chỗ khác. Ý vẫn là Ý cũ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy struct chưa xuất hiện trong bài này — mọi dữ liệu đều là
những giá trị đơn lẻ: một chuỗi, một số. Nhưng chương trình thật hiếm
khi chỉ có giá trị đơn lẻ. Một khách hàng có tên VÀ điểm VÀ hạng thành
viên — nhiều mẩu dữ liệu đi cùng nhau, dưới một cái tên chung.

Rust gói những mẩu dữ liệu đó lại bằng cách nào? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
