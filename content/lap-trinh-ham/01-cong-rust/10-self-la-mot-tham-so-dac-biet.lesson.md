---
id: lap-trinh-ham.cong-rust.self-la-mot-tham-so-dac-biet
title: "`self` là một tham số đặc biệt — theo giá trị hay theo tham chiếu?"
summary: "fn an(self) -> i32 (không &) NUỐT struct — gọi d.an() rồi dùng lại d sau đó bị chặn move. fn tong(&self) -> i32 (có &) chỉ MƯỢN — d vẫn dùng được sau. self/&self chính là move-vs-borrow áp dụng lên tham số đầu tiên của method."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 10
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.self-vs-ref-self]
requires: [rs.move-through-branch-unchecked, rs.struct]
concepts: [rs.self-vs-ref-self]
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
Ngay bài `struct` đầu track, `&self` xuất hiện như một cú pháp "cần có
để gọi được", không giải thích thêm. Hôm nay Byte quay lại giữ đúng
lời hứa đó.
::::

::::explain{#self-la-tham-so}
`fn tong(&self) -> i32 { self.x + self.y }` — nhìn kỹ, `&self` chỉ là
một cách viết TẮT của `self: &Self` (`Self` ở đây nghĩa là kiểu đang
`impl`, ví dụ `Diem`). Nó là THAM SỐ ĐẦU TIÊN của method, y hệt mọi
tham số khác của một hàm — chỉ khác ở việc không cần ghi rõ kiểu, Rust
tự hiểu "kiểu của `self` chính là kiểu của `struct` đang `impl`".

Và vì `self` là một tham số bình thường, TOÀN BỘ luật move-vs-borrow đã
học suốt cụm bài này áp dụng lên nó y hệt bất cứ tham số nào khác:

- `fn an(self) -> i32` — `self` viết KHÔNG có `&`, nhận THEO GIÁ TRỊ.
  Gọi `d.an()` chuyển hẳn quyền sở hữu của `d` vào bên trong method,
  đúng luật đã học ở bài "truyền vào hàm cũng là move". Sau lời gọi,
  `d` mất quyền chủ — dùng lại nó bị chặn, đúng `BR0530`.
- `fn tong(&self) -> i32` — `self` viết CÓ `&`, chỉ MƯỢN. Gọi
  `d.tong()` không lấy quyền sở hữu của `d` đi đâu cả — `d` vẫn còn
  nguyên, dùng lại được ngay sau đó, gọi lại `d.tong()` lần nữa cũng
  được (cụm bài sau đào sâu đúng cơ chế mượn `&` này).

`self`/`&self` không phải hai cách viết tuỳ thích, chọn cái nào cũng
được. Đó CHÍNH LÀ lựa chọn move-hay-borrow, chỉ là áp dụng lên đúng vị
trí tham số đầu tiên của một method thay vì một tham số thường.
::::

::::example{#nuot-va-muon}
Một `struct Ve` với hai method: một mượn (`&self`), một nuốt hẳn
(`self`, không `&`):

```rust title=readonly
struct Ve {
    ma: String,
}

impl Ve {
    fn hien_thi(&self) -> String {
        format!("Vé: {}", self.ma)
    }

    fn huy(self) -> String {
        format!("Đã huỷ vé: {}", self.ma)
    }
}
```

Gọi `hien_thi` (mượn) hai lần liên tiếp — cả hai lần đều hợp lệ:

```rust title=readonly
fn main() {
    let v = Ve { ma: String::from("07") };
    println!("{}", v.hien_thi());
    println!("{}", v.hien_thi());
}
```

```text title=readonly
Vé: 07
Vé: 07
```

`&self` chỉ mượn — gọi bao nhiêu lần cũng không làm `v` mất quyền chủ.
Giờ gọi `huy` (nuốt hẳn), rồi thử gọi `hien_thi` sau đó:

```rust title=readonly
struct Ve {
    ma: String,
}

impl Ve {
    fn hien_thi(&self) -> String {
        format!("Vé: {}", self.ma)
    }

    fn huy(self) -> String {
        format!("Đã huỷ vé: {}", self.ma)
    }
}

fn main() {
    let v = Ve { ma: String::from("07") };
    let thong_bao = v.huy();
    let mo_ta = v.hien_thi();
    println!("{} {}", thong_bao, mo_ta);
}
```

```text title=readonly
(không biên dịch được)

lỗi [BR0530]: `v` đã bị chuyển quyền sở hữu đi nơi khác
  --> dòng 18:17
   |
18 |     let mo_ta = v.hien_thi();
   |                 ^ dùng lại ở đây thì không còn giá trị nữa
   |
  --> dòng 17:21
   |
17 |     let thong_bao = v.huy();
   |                     - quyền sở hữu bị chuyển đi tại đây
   |
  vì sao: Rust cho mỗi giá trị đúng MỘT chủ sở hữu. Khi bạn gán nó
  sang biến khác hoặc truyền vào hàm, chủ cũ mất quyền — nhờ luật này
  Rust không cần bộ dọn rác mà vẫn không bao giờ dùng nhầm bộ nhớ đã
  giải phóng.
```

Đúng chẩn đoán `BR0530` đã gặp nhiều lần trước — chỉ khác nơi quyền sở
hữu chuyển đi lần này là lời gọi `v.huy()`, một method nhận `self`
không `&`. Gọi `v.huy()` xong, `v` không còn là chủ của gì cả; gọi
tiếp `v.hien_thi()` bị chặn y hệt mọi lượt dùng-lại-sau-move khác.
::::

::::predict{#an-het-hay-muon commitOnce}
Một `struct TaiKhoan` với hai method, Byte viết đoạn mã sau, KHÔNG chạy
thử:

```rust
struct TaiKhoan {
    so_du: i32,
}

impl TaiKhoan {
    fn xem(&self) -> i32 {
        self.so_du
    }

    fn dong(self) -> String {
        format!("Đã đóng tài khoản, số dư cuối: {}", self.so_du)
    }
}

fn main() {
    let tk = TaiKhoan { so_du: 500 };
    let bao_cao = tk.dong();
    let so_du_hien_tai = tk.xem();
    println!("{} — hiện tại: {}", bao_cao, so_du_hien_tai);
}
```

**Trước khi đọc đáp án**, đoạn mã này có biên dịch được không, và vì
sao?

:::opt{correct}
Không — `tk.dong()` gọi method `dong(self)` (không `&`), nuốt hẳn
`tk`, chuyển quyền sở hữu vào bên trong method. Dòng sau đó,
`tk.xem()`, cố dùng lại `tk` — đã mất quyền chủ — nên bị chặn (`BR0530`
thật)
:::

:::opt
Có, biên dịch được — `dong` chỉ ĐỌC `self.so_du` để tạo chuỗi báo cáo,
không sửa giá trị nào, nên `tk` không mất quyền chủ
::why
Gần đúng ở việc bạn để ý `dong` quả thật không sửa `so_du` — thân
method chỉ đọc nó qua `format!`.

Chỗ lệch: luật move không quan tâm thân method có SỬA hay không — nó
chỉ nhìn CHỮ KÝ tham số `self`. `fn dong(self)` (không `&`) nhận theo
giá trị, nên gọi `tk.dong()` luôn move `tk`, bất kể bên trong đọc hay
sửa gì. Dòng `tk.xem()` sau đó vẫn bị chặn thật.
::
:::

:::opt
Không — nhưng lỗi nằm ở `fn xem(&self) -> i32`, vì một `struct` không
được phép có cả method nhận `self` theo giá trị lẫn method nhận `&self`
cùng lúc
::why
Gần đúng ở việc bạn nghi ngờ có xung đột nào đó giữa hai method — một
cảm giác hợp lý khi thấy hai chữ ký khác nhau trên cùng một `struct`.

Chỗ lệch: một `struct` hoàn toàn được phép có nhiều method với `self`
khai theo nhiều cách khác nhau — `&self`, `&mut self`, hay `self` —
không hề xung đột nhau ở mức khai báo. `impl TaiKhoan` với cả `xem` và
`dong` như trên biên dịch sạch. Lỗi thật chỉ xảy ra ở CÁCH GỌI trong
`main`: gọi `dong` (nuốt hẳn) rồi gọi lại `xem` trên cùng một biến sau
đó.
::
:::

:::opt
Có, biên dịch được — `tk.dong()` được gán ngay vào `bao_cao`, nên Rust
hiểu là `tk` chỉ "cho mượn tạm" trong đúng một dòng đó, rồi được trả
lại nguyên vẹn cho dòng sau
::why
Gần đúng ở việc bạn cảm nhận đúng: `&self` (như ở `xem`) THẬT SỰ hoạt
động kiểu "mượn tạm rồi trả lại" — trực giác đó không sai cho method
nhận `&self`.

Chỗ lệch: `dong` không nhận `&self`, nó nhận `self` không có `&` —
KHÔNG có khái niệm "mượn tạm rồi trả lại" ở đây, chỉ có move dứt khoát.
Việc gán kết quả vào `bao_cao` không làm quyền sở hữu của `tk` "quay
về" — quyền đó đã chuyển hẳn vào bên trong method `dong` từ lúc gọi
`tk.dong()`, không có đường lùi.
::
:::
::::

::::code{#huy-ve}
Một `struct Ve` với hai method: `hien_thi` (mượn, đọc mô tả) và `huy`
(nuốt hẳn, trả về thông báo huỷ). Điền chỗ trống để `huy` nhúng đúng mã
vé vào thông báo trả về.

```rust title=starter
struct Ve {
    ma: String,
}

impl Ve {
    fn hien_thi(&self) -> String {
        format!("Vé: {}", self.ma)
    }

    fn huy(self) -> String {
        format!("Đã huỷ vé: {}", ___)
    }
}
```

```rust title=solution
struct Ve {
    ma: String,
}

impl Ve {
    fn hien_thi(&self) -> String {
        format!("Vé: {}", self.ma)
    }

    fn huy(self) -> String {
        format!("Đã huỷ vé: {}", self.ma)
    }
}
```

```rust title=test
fn main() {
    let v = Ve { ma: String::from("07") };
    let mo_ta = v.hien_thi();
    println!("{}", mo_ta);
    let thong_bao = v.huy();
    println!("{}", thong_bao);
    assert_eq!(mo_ta, "Vé: 07", "hien_thi phải trả về đúng mô tả vé");
    assert_eq!(
        thong_bao, "Đã huỷ vé: 07",
        "huy phải trả về đúng thông báo chứa mã vé"
    );
}
```

:::hints
- kind: attention
  body: Bên trong method huy, self chính là đối tượng Ve đã gọi nó. Trường ma của nó đọc qua self.ma, đúng cách hien_thi đã làm ở trên.
- kind: strategy
  body: huy nhận self theo giá trị (không &), nhưng cách đọc trường bên trong vẫn dùng self.ten_truong, y hệt &self. Dùng self.ma.
- kind: one-line
  body: 'Chỗ trống là: self.ma'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Vé: 07"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`self` không phải một từ khoá bí ẩn — nó là tham số đầu tiên của
method, và mọi luật move-vs-borrow bạn đã học suốt cụm bài này áp dụng
lên nó y hệt bất cứ tham số nào khác.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Suốt cụm bài vừa qua, mỗi lần một hàm hay method cần ĐỌC một giá trị mà
không lấy hẳn quyền sở hữu, bạn chỉ có hai lựa chọn: chấp nhận mất
quyền sở hữu bản gốc (move), hoặc trả giá bằng một bản sao tốn bộ nhớ
(`.clone()`). `&self` hé lộ có một cách thứ ba — MƯỢN — nhưng track chỉ
mới dùng nó ở đúng một vị trí: tham số `self`.

Cách mượn đó có dùng được cho MỌI tham số khác không — không chỉ
`self`, mà bất cứ đối số nào bạn truyền vào bất cứ hàm nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
