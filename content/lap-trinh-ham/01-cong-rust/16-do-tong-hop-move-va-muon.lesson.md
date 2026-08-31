---
id: lap-trinh-ham.cong-rust.do-tong-hop-move-va-muon
title: "Đo tổng hợp: ghép move và mượn trong một chương trình nhỏ"
summary: "Một struct với một phương thức &self (đọc) và một phương thức &mut self (sửa), gọi cả hai đúng thứ tự trong main. Không luật nào mới — chỉ ghép lại move (cụm trước) và &/&mut (bài 11-15), mọi tình huống nằm trong hai luật hẹp byte-rust kiểm được: không rẽ nhánh, không mượn xen kẽ qua nhiều statement."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 16
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.ownership-borrow-together]
requires: [rs.mixed-borrow-nll]
concepts: [rs.ownership-borrow-together]
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
Không có luật mới hôm nay. Chỉ có một câu hỏi: bạn ghép lại được
move và mượn, đúng chỗ, trong một chương trình chưa?
::::

::::explain{#ghep-lai-tat-ca}
Từ đầu track tới giờ, bạn đã học hai nửa của cùng một bức tranh:

- **Move** (cụm bài trước): gán, truyền vào hàm theo giá trị, hoặc
  gọi phương thức `fn an(self)` (không `&`) — quyền sở hữu CHUYỂN đi,
  biến gốc mất quyền dùng.
- **Mượn** (bài 11-15): `&`/`&mut` — quyền sở hữu KHÔNG chuyển,
  biến gốc vẫn dùng được sau khi hàm/phương thức chạy xong.

`struct` (bài 2) và `impl` (cũng bài 2) là nơi cả hai gặp nhau rõ
nhất — vì tham số ĐẦU TIÊN của một phương thức, `self`, cũng tuân
đúng luật move-vs-mượn đó (bài 10): `&self` mượn để đọc, `&mut self`
mượn để sửa.

```
struct TaiKhoan {
    so_du: i32,
}

impl TaiKhoan {
    fn xem_so_du(&self) -> i32 {
        self.so_du
    }

    fn nap_tien(&mut self, so_tien: i32) {
        (*self).so_du += so_tien;
    }
}
```

`xem_so_du` chỉ ĐỌC (`&self`) — gọi bao nhiêu lần cũng không đụng gì
tới `so_du`. `nap_tien` SỬA (`&mut self`) — thân hàm viết
`(*self).so_du`: `self` bên trong một phương thức `&mut self` chính
là một tham chiếu khả biến (`&mut TaiKhoan`), và `*self` giải tham
chiếu đó để chạm thẳng vào giá trị `TaiKhoan` thật — cùng một dấu `*`
bạn đã dùng ở bài 12 (`*x += 1`), chỉ khác đối tượng đứng sau nó giờ
là một `struct` thay vì một số.

Gọi cả hai phương thức, đúng thứ tự, trong `main`:

```
let mut tk = TaiKhoan { so_du: 100 };
println!("{}", tk.xem_so_du());   // đọc
tk.nap_tien(50);                   // sửa
println!("{}", tk.xem_so_du());   // đọc lại, thấy giá trị mới
```

Không có gì ở đây vượt ra khỏi hai luật hẹp bạn đã học: không rẽ
nhánh, không có tham chiếu nào mượn rồi xen kẽ qua nhiều câu lệnh
theo kiểu bài 15 — mỗi lời gọi phương thức tự đóng gọn, mượn xong rồi
hết hạn ngay khi phương thức chạy xong.
::::

::::example{#doc-sua-doc-lai}
Chương trình `TaiKhoan` đầy đủ, chạy được:

```rust title=readonly
struct TaiKhoan {
    so_du: i32,
}

impl TaiKhoan {
    fn xem_so_du(&self) -> i32 {
        self.so_du
    }

    fn nap_tien(&mut self, so_tien: i32) {
        (*self).so_du += so_tien;
    }
}

fn main() {
    let mut tk = TaiKhoan { so_du: 100 };
    println!("{}", tk.xem_so_du());
    tk.nap_tien(50);
    println!("{}", tk.xem_so_du());
}
```

```text title=readonly
100
150
```

Đọc, rồi sửa, rồi đọc lại — ba lời gọi phương thức, không lời gọi
nào lấy quyền sở hữu của `tk`. `tk` vẫn là chủ của chính nó suốt từ
đầu tới cuối `main`.
::::

::::predict{#truyen-han-struct commitOnce}
Byte viết một chương trình khác — lần này truyền hẳn (không mượn)
một `struct` vào hàm, KHÔNG chạy thử:

```rust
struct DemDon {
    so_luong: i32,
}

impl DemDon {
    fn xem(&self) -> i32 {
        self.so_luong
    }
}

fn nhan(d: DemDon) {
    println!("{}", d.xem());
}

fn main() {
    let don = DemDon { so_luong: 3 };
    println!("{}", don.xem());
    nhan(don);
    println!("{}", don.xem());
}
```

**Trước khi đọc đáp án**, đoạn mã này có biên dịch được không, và vì
sao?

:::opt{correct}
Không — `nhan(d: DemDon)` nhận tham số THEO GIÁ TRỊ (không `&`), nên
`nhan(don)` là một MOVE thật, tiêu thụ `don`. Dòng `println!` cuối
dùng lại `don` sau khi đã move — bị chặn (BR0530), đúng luật đã học
từ cụm bài trước, không liên quan gì tới việc `DemDon` có phương
thức `&self` hay không
:::

:::opt
Có, biên dịch được — `DemDon` có một phương thức `xem(&self)`, và hễ
một `struct` đã có phương thức nhận `&self` thì MỌI cách truyền
struct đó đi đâu cũng tự động trở thành mượn, không bao giờ move
::why
Gần đúng ở việc bạn để ý `DemDon` có định nghĩa một phương thức
`&self` — một chi tiết có thật trong chương trình.

Chỗ lệch: việc một `struct` có phương thức `&self` không hề ảnh
hưởng tới việc THAM SỐ HÀM ở một chỗ khác được khai là gì. `xem(&self)`
chỉ quyết định CÁCH GỌI `xem`, không quyết định cách `nhan` nhận
tham số của NÓ. `fn nhan(d: DemDon)` (không `&`) vẫn move như bình
thường — hai chuyện độc lập nhau hoàn toàn.
::
:::

:::opt
Không — nhưng lỗi nằm ở chính dòng `nhan(don);`, vì hàm `nhan` không
có kiểu trả về nên không được phép gọi nó và bỏ qua kết quả như một
câu lệnh độc lập
::why
Gần đúng ở việc bạn xét tới chữ ký hàm `nhan` — một thói quen đúng
hướng khi đọc mã Rust.

Chỗ lệch: gọi một hàm không có kiểu trả về (ngầm định trả về `()`,
"đơn vị rỗng") như một câu lệnh độc lập là cách dùng HOÀN TOÀN bình
thường trong Rust — không có luật nào cấm điều đó, và dòng
`nhan(don);` tự nó không gây lỗi gì. Lỗi thật nằm ở dòng
`println!("{}", don.xem())` NGAY SAU nó — dùng lại `don` sau khi nó
đã bị `nhan(don)` move đi.
::
:::

:::opt
Có, biên dịch được — `don.xem()` ở dòng đầu đã "tiêu tốn" hết một
lượt đọc của `don`, nên tới lượt `nhan(don)`, Rust hiểu đây là lượt
dùng CUỐI CÙNG và tự động chuyển nó thành mượn thay vì move, để
chương trình chạy được trọn vẹn
::why
Gần đúng ở việc bạn cảm nhận đúng: Rust đôi lúc "linh hoạt" theo NGỮ
CẢNH dùng (đúng tinh thần vùng sống ở bài trước).

Chỗ lệch: sự linh hoạt đó (non-lexical lifetimes) áp dụng cho MƯỢN —
quyết định một tham chiếu còn sống tới đâu. Nó KHÔNG tự ý đổi một
lời gọi hàm move thành mượn chỉ vì "có vẻ tiện". `fn nhan(d: DemDon)`
đã khai tường minh nhận theo giá trị — Rust luôn tôn trọng đúng chữ
ký đó, không suy đoán ý định khác. `nhan(don)` luôn luôn là move.
::
:::
::::

::::code{#nap-them-tien}
Một `struct TaiKhoan` như ví dụ trên. Điền chỗ trống để `nap_tien`
cộng đúng số tiền vào số dư hiện có.

```rust title=starter
struct TaiKhoan {
    so_du: i32,
}

impl TaiKhoan {
    fn xem_so_du(&self) -> i32 {
        self.so_du
    }

    fn nap_tien(&mut self, so_tien: i32) {
        (*self).so_du += ___;
    }
}
```

```rust title=solution
struct TaiKhoan {
    so_du: i32,
}

impl TaiKhoan {
    fn xem_so_du(&self) -> i32 {
        self.so_du
    }

    fn nap_tien(&mut self, so_tien: i32) {
        (*self).so_du += so_tien;
    }
}
```

```rust title=test
fn main() {
    let mut tk = TaiKhoan { so_du: 100 };
    println!("{}", tk.xem_so_du());
    tk.nap_tien(50);
    println!("{}", tk.xem_so_du());
    assert_eq!(
        tk.xem_so_du(), 150,
        "sau nap_tien(50), so_du phải là 150 (100 + 50) — đang là {}",
        tk.xem_so_du()
    );
}
```

:::hints
- kind: attention
  body: nap_tien nhận một tham số so_tien — chỗ trống phải cộng đúng số tiền ĐÓ vào số dư hiện có, không phải một con số cố định.
- kind: strategy
  body: '(*self).so_du += ___ đã có sẵn vế trái (số dư hiện tại). Giá trị cần cộng vào chính là tham số của phương thức — so_tien.'
- kind: one-line
  body: 'Chỗ trống là: so_tien'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "150"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đọc, sửa, đọc lại — không có luật nào mới, chỉ có move và mượn dùng
đúng chỗ, đúng lúc. Bạn vừa ghép được toàn bộ nửa đầu của track này
lại thành một chương trình duy nhất.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Từ đầu track, mọi giá trị bạn tạo ra LUÔN LUÔN có mặt — một `i32`
luôn là một con số thật, một `String` luôn có nội dung, một
`TaiKhoan` luôn có `so_du`. Nhưng có những lúc một giá trị THẬT SỰ có
thể "không có gì cả" — tìm một khách hàng không tồn tại trong danh
sách, đọc một trường có thể trống.

Python có `None`. Nhiều ngôn ngữ khác có `null`. Rust không có cả
hai. Vậy Rust biểu diễn "có thể không có gì" bằng cách nào?

Bài sau trả lời — và câu trả lời chính là một `enum` (bài 3) bạn đã
biết cách dùng từ lâu.
::::

::::checkpoint{mastery=0.8}
::::
