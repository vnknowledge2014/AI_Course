---
id: lap-trinh-ham.cong-rust.do-tong-hop-truoc-boss
title: "Đo tổng hợp trước BOSS"
summary: "Một struct có trường Option<T>, một method &self đọc, một method nhận self theo giá trị — ba mảnh đã học riêng lẻ, giờ ghép chung trong một chương trình. Không khái niệm mới, chỉ đo bạn có GHÉP ĐÚNG hay không."
locale: vi
track: lap-trinh-ham
module: cong-rust
order: 23
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [rs.gate-review]
requires: [rs.iterator-basics]
concepts: [rs.gate-review]
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
Bạn nói bạn TIN các mảnh đã học khớp được với nhau. Bài này không dạy
gì mới — nó đo thử, đừng tin suông.
::::

::::explain{#nhung-manh-can-ghep}
Không một Ý mới nào trong bài này. Ba mảnh đã học riêng lẻ, giờ đứng
chung trong một `struct`:

1. **Một trường kiểu `Option<T>`** (cụm 4, bài 17) — một mẩu dữ liệu CÓ
   THỂ không có gì, khai tường minh ngay trong hình dạng `struct`.
2. **Một method `&self`** (cụm 2, bài 10) — MƯỢN struct để đọc, gọi
   xong struct gốc vẫn dùng được.
3. **Một method nhận `self` theo giá trị** (cũng bài 10) — LẤY HẲN
   quyền sở hữu struct, gọi xong struct gốc KHÔNG dùng lại được nữa,
   đúng luật move của cụm 2.

Ba mảnh này không mới. Cái mới là NHÌN THẤY chúng đứng cạnh nhau trong
cùng một `struct`, và phải nhớ ĐÚNG cái nào mượn, cái nào lấy hẳn — vì
cú pháp của hai loại method chỉ khác nhau đúng một dấu `&`.
::::

::::example{#the-thanh-vien}
Một tấm thẻ thành viên — có thể có điểm thưởng, có thể chưa:

```rust title=readonly
struct TheThanhVien {
    ten: String,
    diem_thuong: Option<i32>,
}

impl TheThanhVien {
    fn mo_ta(&self) -> String {
        match &self.diem_thuong {
            Some(d) => format!("{} co {} diem thuong", self.ten, d),
            None => format!("{} chua co diem thuong", self.ten),
        }
    }

    fn dong_the(self) -> String {
        format!("The cua {} da dong", self.ten)
    }
}

fn main() {
    let the1 = TheThanhVien { ten: String::from("Lan"), diem_thuong: Some(120) };
    println!("{}", the1.mo_ta());

    let the2 = TheThanhVien { ten: String::from("Nam"), diem_thuong: None };
    println!("{}", the2.mo_ta());

    let thong_bao = the1.dong_the();
    println!("{}", thong_bao);
}
```

```text title=readonly
Lan co 120 diem thuong
Nam chua co diem thuong
The cua Lan da dong
```

`mo_ta(&self)` chỉ MƯỢN — gọi `the1.mo_ta()` xong, `the1` vẫn còn
nguyên, gọi tiếp `the1.dong_the()` được. Bên trong, `match &self.diem_thuong`
so khớp trên một THAM CHIẾU tới trường `Option`, không lấy nó đi — `d`
nhận về là một tham chiếu tới con số bên trong `Some`, in ra bằng `{}`
bình thường.

`dong_the(self)` (không `&`) LẤY HẲN quyền sở hữu `the1` — sau dòng
`the1.dong_the()`, `the1` không còn dùng lại được nữa. Chương trình
trên không đụng tới `the1` sau dòng đó, nên chạy sạch.
::::

::::predict{#dung-lai-sau-dong-the commitOnce}
Byte thêm một dòng vào chương trình trên — gọi `mo_ta()` trên `the1`
NGAY SAU khi đã gọi `dong_the()`:

```rust
fn main() {
    let the1 = TheThanhVien { ten: String::from("Lan"), diem_thuong: Some(120) };

    let thong_bao = the1.dong_the();
    println!("{}", thong_bao);

    println!("{}", the1.mo_ta());
}
```

**Trước khi đọc đáp án**, đoạn mã này có biên dịch được không?

:::opt{correct}
Không — `dong_the(self)` không có `&`, nên nó LẤY HẲN quyền sở hữu
`the1` (move). Sau dòng `the1.dong_the()`, `the1` đã mất chủ; gọi
`the1.mo_ta()` ở dòng sau là dùng lại một giá trị đã bị chuyển đi, bị
chặn (`BR0530`)
:::

:::opt
Có, biên dịch được — cả hai đều là method của `TheThanhVien`, gọi
liên tiếp trên cùng một biến là chuyện bình thường, không liên quan gì
tới `&`
::why
Gần đúng ở việc "gọi liên tiếp hai method trên cùng một biến" đúng là
một thao tác bình thường — với NHIỀU method khác, ví dụ gọi `mo_ta()`
hai lần liên tiếp, sẽ chạy được.

Chỗ lệch: hai method ở đây KHÔNG giống nhau ở đúng chỗ dấu `&`.
`mo_ta(&self)` mượn, gọi bao nhiêu lần cũng được. `dong_the(self)`
lấy hẳn — gọi nó xong, biến gốc không còn là chủ của giá trị đó nữa,
bất kể method gọi sau là gì.
::
:::

:::opt
Có, biên dịch được, nhưng `the1.mo_ta()` sẽ in ra chuỗi rỗng vì dữ
liệu bên trong đã bị `dong_the()` xoá sạch
::why
Gần đúng ở việc bạn cảm nhận đúng có gì đó "mất" sau `dong_the()` —
phản xạ đó không sai.

Chỗ lệch: Rust không cho đoạn mã này chạy TỚI mức in ra bất cứ gì cả —
nó bị TỪ CHỐI BIÊN DỊCH (`BR0530`) trước khi chương trình kịp chạy một
dòng nào. Không có "chuỗi rỗng" nào được in ra; không dòng lệnh nào
sau lỗi được thực thi.
::
:::

:::opt
Không — nhưng lý do là `TheThanhVien` gọi được method tối đa MỘT lần
trong cả chương trình, bất kể đó là method nào
::why
Gần đúng ở việc bạn đúng là có một giới hạn số lần gọi liên quan tới
struct này — giới hạn đó có thật, chỉ là không áp dụng cho MỌI method.

Chỗ lệch: `mo_ta(&self)` gọi được bao nhiêu lần tuỳ thích trên cùng một
biến, không giới hạn — nó chỉ MƯỢN, không đụng gì tới quyền sở hữu.
Giới hạn "gọi một lần rồi thôi" chỉ áp dụng cho method THỨ HAI,
`dong_the(self)`, vì nó lấy hẳn quyền sở hữu — không phải một luật
chung cho cả struct.
::
:::
::::

::::code{#ghep-option-va-move}
Một `TheKhachHang` — tên, và một trường `giam_gia: Option<i32>` (phần
trăm giảm giá, có thể chưa có). Một method `&self` mô tả, một method
`self` đóng thẻ. Điền chỗ trống trong nhánh `Some`.

```rust title=starter
struct TheKhachHang {
    ten: String,
    giam_gia: Option<i32>,
}

impl TheKhachHang {
    fn mo_ta(&self) -> String {
        match &self.giam_gia {
            Some(d) => format!("{} co {} phan tram giam gia", self.ten, ___),
            None => format!("{} khong co giam gia", self.ten),
        }
    }

    fn dong_the(self) -> String {
        format!("The cua {} da dong", self.ten)
    }
}
```

```rust title=solution
struct TheKhachHang {
    ten: String,
    giam_gia: Option<i32>,
}

impl TheKhachHang {
    fn mo_ta(&self) -> String {
        match &self.giam_gia {
            Some(d) => format!("{} co {} phan tram giam gia", self.ten, d),
            None => format!("{} khong co giam gia", self.ten),
        }
    }

    fn dong_the(self) -> String {
        format!("The cua {} da dong", self.ten)
    }
}
```

```rust title=test
fn main() {
    let the1 = TheKhachHang { ten: String::from("Lan"), giam_gia: Some(15) };
    let mo_ta1 = the1.mo_ta();
    println!("{}", mo_ta1);

    let the2 = TheKhachHang { ten: String::from("Nam"), giam_gia: None };
    let mo_ta2 = the2.mo_ta();
    println!("{}", mo_ta2);

    let dong = the1.dong_the();
    println!("{}", dong);

    assert_eq!(mo_ta1, "Lan co 15 phan tram giam gia", "mo_ta1 sai");
    assert_eq!(mo_ta2, "Nam khong co giam gia", "mo_ta2 sai");
    assert_eq!(dong, "The cua Lan da dong", "dong sai");
}
```

:::hints
- kind: attention
  body: Nhánh Some(d) đã bóc con số giảm giá ra và đặt tên nó là d — chỗ trống chỉ cần dùng lại đúng cái tên đó.
- kind: strategy
  body: 'match &self.giam_gia { Some(d) => ... } gán tên d cho giá trị bên trong Some. Chỗ trống nằm ngay trong nhánh đó — dùng lại d, không cần tính toán gì thêm.'
- kind: one-line
  body: 'Chỗ trống là: d'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "15 phan tram giam gia"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`Option`, `&self`, `self` theo giá trị — ba mảnh riêng lẻ, một struct
duy nhất, không lệch một luật nào. Sẵn sàng cho bài cuối.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi vào bài cuối cùng của cổng này.

Bài này ghép BA Ý. Nhưng cả track, tính từ đầu, đã dựng ra nhiều hơn
thế rất nhiều: `struct`, `enum` và `match`, move và `.clone()`, `&` và
`&mut`, `Option`, `Result` và `?`, rồi cả iterator.

Bài cuối không hỏi riêng một Ý nào nữa. Nó đòi bạn viết một chương
trình dùng ĐỦ — không phải ba mảnh, mà gần như toàn bộ track — cùng
một lúc.
::::

::::checkpoint{mastery=0.8}
::::
