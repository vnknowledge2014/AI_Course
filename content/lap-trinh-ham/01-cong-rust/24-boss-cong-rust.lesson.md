---
id: lap-trinh-ham.cong-rust.boss-cong-rust
title: "BOSS — Khép cổng Rust"
summary: "Một chương trình nhỏ dùng đủ: struct/enum/match, move và .clone() khi cần, & và &mut đúng chỗ, Option/Result/?. Không dạy khái niệm mới — chỉ đòi ghép lại đúng những gì hai mươi ba bài trước đã dựng."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 24
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.gate-boss]
requires: [rs.gate-review]
concepts: [rs.gate-boss]
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
Hai mươi ba bài để tới đây. Một Ý gốc xuyên suốt tất cả: một giá trị có
ĐÚNG MỘT chủ. Giờ ghép hết lại, trong một chương trình duy nhất.
::::

::::explain{#bon-manh-ghep}
Suốt track này, mọi bài đều là hệ quả của đúng MỘT luật gốc — **một
giá trị có đúng một chủ sở hữu tại một thời điểm**. Bốn mảnh ghép, tất
cả cùng xoay quanh luật đó:

1. **`struct`/`enum`/`match`** (cụm 1) — đặt tên cho một hình dạng dữ
   liệu cố định, hoặc một hình dạng là MỘT TRONG NHIỀU khả năng đã
   liệt kê; `match` buộc xử lý ĐỦ mọi khả năng, kiểm thật lúc biên
   dịch.
2. **move và `.clone()`** (cụm 2) — gán, truyền vào hàm, hay gọi một
   method nhận `self` theo giá trị đều CHUYỂN quyền sở hữu, không sao
   chép; dùng lại sau khi đã chuyển là bị chặn TRƯỚC KHI CHẠY. Muốn
   giữ cả hai bản, gọi `.clone()` tường minh.
3. **`&` và `&mut`** (cụm 3) — mượn để đọc (nhiều người cùng lúc được),
   hoặc mượn để sửa (đúng một người tại một thời điểm) — không lấy
   quyền sở hữu, giá trị gốc vẫn còn chủ cũ sau khi mượn xong.
4. **`Option`/`Result`/`?`** (cụm 4) — một giá trị CÓ THỂ không có gì
   (`Option`), một phép tính CÓ THỂ thất bại (`Result`) — cả hai đều là
   `enum` thật, ép `match` xử lý đủ; `?` là cách viết tắt lan truyền
   lỗi lên trên, không xử lý tại chỗ.

Bốn mảnh đó là toàn bộ nguyên liệu để viết một chương trình Rust nhỏ,
nơi quyền sở hữu của MỌI giá trị đều rõ ràng từ đầu tới cuối. Bài này
ghép chúng vào một chương trình duy nhất.
::::

::::example{#chuong-trinh-ghep-du-bon-manh}
Một cửa hàng nhỏ: một sản phẩm, một loại thẻ khách hàng, tính giá sau
giảm, cộng thêm thuế, rồi bán:

```rust title=readonly
enum LoaiThe {
    ThuongXuyen,
    VIP(i32),
}

struct SanPham {
    ten: String,
    gia: i32,
}

impl SanPham {
    fn mo_ta(&self) -> String {
        format!("{}: {} dong", self.ten, self.gia)
    }

    fn ban(self) -> String {
        format!("Da ban {}", self.ten)
    }
}

fn gia_sau_giam(gia: i32, the: &LoaiThe) -> i32 {
    match the {
        LoaiThe::ThuongXuyen => gia,
        LoaiThe::VIP(phan_tram) => gia - gia * phan_tram / 100,
    }
}

fn cong_thue(gia: &mut i32, thue: i32) {
    *gia += thue;
}

fn kiem_tra_gia(gia: i32) -> Result<i32, String> {
    if gia <= 0 {
        Err(String::from("gia phai duong"))
    } else {
        Ok(gia)
    }
}

fn tao_san_pham(ten: String, gia: i32) -> Result<SanPham, String> {
    let gia_ok = kiem_tra_gia(gia)?;
    Ok(SanPham { ten, gia: gia_ok })
}

fn main() {
    let sp = tao_san_pham(String::from("Ao thun"), 100000).unwrap();
    println!("{}", sp.mo_ta());

    let the = LoaiThe::VIP(10);
    let mut gia_sau = gia_sau_giam(100000, &the);
    println!("Gia sau giam: {}", gia_sau);

    cong_thue(&mut gia_sau, 5000);
    println!("Gia cong thue: {}", gia_sau);

    let ten_luu = sp.ten.clone();
    let thong_bao = sp.ban();
    println!("{}", thong_bao);
    println!("Ten da luu: {}", ten_luu);
}
```

```text title=readonly
Ao thun: 100000 dong
Gia sau giam: 90000
Gia cong thue: 95000
Da ban Ao thun
Ten da luu: Ao thun
```

Đọc ra bốn mảnh: `enum LoaiThe` + `match` (mảnh 1) tính đúng phần trăm
giảm cho từng loại thẻ. `tao_san_pham` dùng `?` (mảnh 4) — nếu
`kiem_tra_gia` trả `Err`, hàm thoát sớm, không dựng `SanPham` nào cả;
`.unwrap()` ở `main` lấy giá trị ra, chấp nhận panic nếu có lỗi.
`gia_sau_giam` MƯỢN thẻ bằng `&LoaiThe` (mảnh 3, mượn để đọc) — `the`
dùng lại được sau đó. `cong_thue` MƯỢN `gia_sau` bằng `&mut i32` (mảnh
3, mượn để sửa) — sửa được giá trị gốc qua `*gia`. Trước khi `sp.ban()`
LẤY HẲN quyền sở hữu `sp` (mảnh 2, method nhận `self`), chương trình
gọi `sp.ten.clone()` để giữ lại một bản tên riêng — không thì `ten_luu`
không thể tồn tại sau khi `sp` đã mất chủ.

Nếu bỏ dòng `.clone()`, dùng thẳng `sp.ten` sau `sp.ban()`:

```rust title=readonly
    let thong_bao = sp.ban();
    println!("{}", thong_bao);
    println!("Ten da luu: {}", sp.ten);
```

```text title=readonly
(không biên dịch được)

lỗi [BR0530]: `sp` đã bị chuyển quyền sở hữu đi nơi khác
```

Đúng luật move (mảnh 2) — `sp.ban()` lấy hẳn `sp`, không còn gì để đọc
`sp.ten` ở dòng sau.
::::

::::predict{#manh-nao-giai-thich commitOnce}
Byte viết đoạn mã sau, KHÔNG chạy thử — nó bị Rust từ chối:

```rust
fn tang_gap_doi(gia: &mut i32) {
    *gia = *gia * 2;
}

fn main() {
    let gia = 50000;
    tang_gap_doi(&gia);
    println!("{}", gia);
}
```

**Trước khi đọc đáp án**, mảnh nào trong bốn mảnh của bài này giải
thích ĐÚNG NHẤT lý do bị từ chối?

:::opt{correct}
Mảnh 3 — `tang_gap_doi` cần `&mut i32` để SỬA giá trị gốc, nhưng lời
gọi truyền `&gia` (mượn để ĐỌC, không phải mượn để sửa), vì `gia` được
khai bằng `let` thường, không phải `let mut`
:::

:::opt
Mảnh 2 — `gia` là kiểu số nguyên nên bị move khi truyền vào
`tang_gap_doi`, sau đó dùng lại `gia` ở dòng `println!` là dùng một
giá trị đã chuyển đi
::why
Gần đúng ở việc bạn nhớ đúng luật move áp dụng cho giá trị truyền vào
hàm (mảnh 2) — luật đó có thật.

Chỗ lệch: hàm ở đây nhận `&mut i32` (một tham chiếu), không nhận `i32`
theo giá trị. Truyền `&gia` không hề move `gia` đi đâu cả — mượn không
bao giờ lấy quyền sở hữu. Lỗi không nằm ở move, mà ở việc `gia` không
được khai `mut` nên không mượn `&mut` được.
::
:::

:::opt
Mảnh 4 — hàm `tang_gap_doi` thiếu khai `-> Result<i32, String>`, nên
không có cách nào báo lỗi khi tham số truyền vào sai
::why
Gần đúng ở việc bạn nhớ đúng `Result` (mảnh 4) là công cụ Rust dùng để
báo một phép tính có thể thất bại.

Chỗ lệch: `tang_gap_doi` không hề THẤT BẠI theo nghĩa cần `Result` —
nó bị từ chối NGAY LÚC BIÊN DỊCH, trước khi có khái niệm "chạy thử rồi
thất bại" nào xảy ra. Đổi kiểu trả về không giải quyết được gì; vấn đề
nằm ở chỗ mượn `&gia` khi cần `&mut`.
::
:::

:::opt
Mảnh 1 — thiếu một `enum` hoặc `match` nào đó khiến trình biên dịch
không xác định được kiểu của `gia`
::why
Gần đúng ở việc `enum`/`match` (mảnh 1) đúng là những công cụ quan
trọng của track này.

Chỗ lệch: kiểu của `gia` không hề mơ hồ — nó là `i32`, suy luận rõ ràng
từ `50000`. Đoạn mã này không dùng `enum` hay `match` ở đâu cả, và lỗi
không liên quan gì tới việc thiếu chúng. Lỗi nằm đúng ở loại mượn
truyền vào `tang_gap_doi`.
::
:::
::::

::::code{#cua-hang-day-du}
Một sản phẩm, một loại thẻ khách hàng, tính giá sau giảm. Điền chỗ
trống cuối cùng để hoàn thành chương trình dùng đủ bốn mảnh ghép của
cổng này.

```rust title=starter
enum LoaiThe {
    ThuongXuyen,
    VIP(i32),
}

struct SanPham {
    ten: String,
    gia: i32,
}

impl SanPham {
    fn mo_ta(&self) -> String {
        format!("{}: {} dong", self.ten, self.gia)
    }

    fn ban(self) -> String {
        format!("Da ban {}", self.ten)
    }
}

fn gia_sau_giam(gia: i32, the: &LoaiThe) -> i32 {
    match the {
        LoaiThe::ThuongXuyen => gia,
        LoaiThe::VIP(phan_tram) => gia - gia * phan_tram / ___,
    }
}

fn cong_thue(gia: &mut i32, thue: i32) {
    *gia += thue;
}

fn kiem_tra_gia(gia: i32) -> Result<i32, String> {
    if gia <= 0 {
        Err(String::from("gia phai duong"))
    } else {
        Ok(gia)
    }
}

fn tao_san_pham(ten: String, gia: i32) -> Result<SanPham, String> {
    let gia_ok = kiem_tra_gia(gia)?;
    Ok(SanPham { ten, gia: gia_ok })
}
```

```rust title=solution
enum LoaiThe {
    ThuongXuyen,
    VIP(i32),
}

struct SanPham {
    ten: String,
    gia: i32,
}

impl SanPham {
    fn mo_ta(&self) -> String {
        format!("{}: {} dong", self.ten, self.gia)
    }

    fn ban(self) -> String {
        format!("Da ban {}", self.ten)
    }
}

fn gia_sau_giam(gia: i32, the: &LoaiThe) -> i32 {
    match the {
        LoaiThe::ThuongXuyen => gia,
        LoaiThe::VIP(phan_tram) => gia - gia * phan_tram / 100,
    }
}

fn cong_thue(gia: &mut i32, thue: i32) {
    *gia += thue;
}

fn kiem_tra_gia(gia: i32) -> Result<i32, String> {
    if gia <= 0 {
        Err(String::from("gia phai duong"))
    } else {
        Ok(gia)
    }
}

fn tao_san_pham(ten: String, gia: i32) -> Result<SanPham, String> {
    let gia_ok = kiem_tra_gia(gia)?;
    Ok(SanPham { ten, gia: gia_ok })
}
```

```rust title=test
fn main() {
    let sp = tao_san_pham(String::from("Ao thun"), 100000).unwrap();
    let mo_ta_ban_dau = sp.mo_ta();
    println!("{}", mo_ta_ban_dau);

    let the = LoaiThe::VIP(10);
    let mut gia_sau = gia_sau_giam(100000, &the);
    println!("Gia sau giam: {}", gia_sau);

    cong_thue(&mut gia_sau, 5000);
    println!("Gia cong thue: {}", gia_sau);

    let ten_luu = sp.ten.clone();
    let thong_bao = sp.ban();
    println!("{}", thong_bao);

    assert_eq!(mo_ta_ban_dau, "Ao thun: 100000 dong", "mo_ta ban dau sai");
    assert_eq!(
        gia_sau, 95000,
        "gia sau khi VIP giam 10% roi cong 5000 tien thue phai la 95000 — dang la {}", gia_sau
    );
    assert_eq!(ten_luu, "Ao thun", "ten_luu sai");
    assert_eq!(thong_bao, "Da ban Ao thun", "thong_bao sai");
}
```

:::hints
- kind: attention
  body: "Giam gia theo PHAN TRAM nghia la chia cho 100 sau khi nhan voi phan_tram — chinh la cong thuc tinh phan tram quen thuoc."
- kind: strategy
  body: 'phan_tram la mot con so nguyen (vi du 10 nghia la 10%). "gia * phan_tram / 100" tinh dung so tien duoc giam. Chỗ trống là mẫu số của phép chia đó.'
- kind: one-line
  body: 'Chỗ trống là: 100'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "Gia cong thue: 95000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`struct`, `enum`, `match`, move, `.clone()`, `&`, `&mut`, `Option`,
`Result`, `?` — mọi mảnh ghép cùng đứng trong một chương trình, không
lệch một luật nào. Cổng Rust khép lại ở đây.
::::

::::reflect{#nghi-lai}
Một câu hỏi cuối, khép lại cả cổng này.

Hai mươi bốn bài, đi từ `fn main()` đầu tiên, tới một chương trình mà
MỌI giá trị đều có đúng một chủ tại một thời điểm — không phải một quy
ước bạn tự nhớ, mà một luật trình biên dịch tự kiểm, TRƯỚC KHI CHƯƠNG
TRÌNH KỊP CHẠY.

Track sau quay lại Python. Nhưng bạn mang theo một câu hỏi mới, một
câu hỏi Python không bao giờ tự trả lời được, và TypeScript (cổng
trước) cũng chưa từng hỏi tới — vì TypeScript không có khái niệm quyền
sở hữu: giá trị này, NGAY LÚC NÀY, ai đang GIỮ nó — và ai chỉ đang
MƯỢN?

Đó là câu hỏi cổng này để lại.
::::

::::checkpoint{mastery=0.85}
::::
