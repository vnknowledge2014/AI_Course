---
id: co-so-du-lieu.bo-nao-cua-truy-van.danh-gia-ca-cay-where
title: "Đánh giá cả cây WHERE trên một hàng"
summary: "danh_gia(cay, i, hang) đệ quy qua CayAst: nút SoSanh gọi danh_gia_so_sanh (bài 3), nút VaNut/HoacNut đệ quy vào cả hai nhánh con rồi gộp bằng && / ||. cay: &CayAst dùng lại được ở cả hai nhánh đệ quy — tham chiếu bất biến, không cần workaround."
locale: vi
track: co-so-du-lieu
module: bo-nao-cua-truy-van
order: 4
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 13
teaches: [db.executor-eval-cay]
requires: [db.executor-eval-so-sanh]
concepts: [db.executor-eval-cay]
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
Một phép so sánh đánh giá được (bài trước). `WHERE tuoi > 18 AND
diem < 100` LÀ cả một CÂY (q07) — đánh giá TRỌN cây trên một hàng?
::::

::::explain{#danh-gia-de-quy}
`danh_gia` đệ quy QUA `CayAst`: nút `SoSanh` gọi `danh_gia_so_sanh`
(bài trước), nút `VaNut`/`HoacNut` đệ quy VÀO cả hai nhánh con rồi
gộp bằng `&&`/`||`:

```rust title=readonly
fn danh_gia(cay: &CayAst, i: usize, hang: &Vec<Truong>) -> bool {
    match &cay.nut[i] {
        NoAst::SoSanh(ten, dau, so) => danh_gia_so_sanh(hang, ten, *dau, *so),
        NoAst::VaNut(trai, phai) => danh_gia(cay, *trai, hang) && danh_gia(cay, *phai, hang),
        NoAst::HoacNut(trai, phai) => danh_gia(cay, *trai, hang) || danh_gia(cay, *phai, hang),
    }
}
```

`cay: &CayAst` — tham chiếu BẤT BIẾN — dùng LẠI được Ở CẢ hai nhánh
đệ quy TRONG cùng một biểu thức (`danh_gia(cay, *trai, hang) &&
danh_gia(cay, *phai, hang)`) — không CẦN workaround GÌ, đúng LỚP đã
vá TỪ q07 (tham chiếu bất biến dùng Ở nhiều nhánh). `NoAst::SoSanh`
LÀ nút LÁ — đệ quy DỪNG Ở đó, gọi THẲNG `danh_gia_so_sanh`.
::::

::::example{#hoac-mot-dung-mot-sai}
Một cây `HoacNut` — `tuoi < 18 OR diem > 90` — CHỈ cần MỘT nhánh
đúng:

```rust title=readonly
let hang1: Vec<Truong> = vec![
    Truong{ten:vec!['t','u','o','i'],gia_tri:15},
    Truong{ten:vec!['d','i','e','m'],gia_tri:50},
];
println!("{}", danh_gia(&cay, goc, &hang1));

let hang2: Vec<Truong> = vec![
    Truong{ten:vec!['t','u','o','i'],gia_tri:20},
    Truong{ten:vec!['d','i','e','m'],gia_tri:50},
];
println!("{}", danh_gia(&cay, goc, &hang2));
```

```text title=readonly
true
false
```

`hang1`: `tuoi=15<18` LÀ `true` — `OR` NGẮN mạch (Rust `||` cũng
đánh giá cả hai VẾ NẾU cần, nhưng KẾT quả CUỐI đúng LÀ `true` khi
MỘT vế `true`). `hang2`: `tuoi=20<18` SAI, `diem=50>90` CŨNG sai —
`false OR false` LÀ `false`.
::::

::::predict{#doan-hoac-ca-hai-dung commitOnce}
Hàng với CẢ hai điều kiện ĐỀU đúng — `tuoi < 18` SAI nhưng
`diem > 90` ĐÚNG:

```rust
let hang3: Vec<Truong> = vec![
    Truong{ten:vec!['t','u','o','i'],gia_tri:20},
    Truong{ten:vec!['d','i','e','m'],gia_tri:95},
];
println!("{}", danh_gia(&cay, goc, &hang3));
```

Dòng cuối in ra gì?

:::opt{correct}
`true`
:::

:::opt
Máy báo lỗi — vì cây gốc LÀ `HoacNut` nhưng chỉ CÓ đúng MỘT nhánh
con thoả điều kiện, không CẢ hai — một trường hợp "nửa vời" không
xác định được
::why
Gần đúng ở việc bạn nghĩ TỚI khái niệm "cả hai nhánh PHẢI nhất
quán" — MỘT trực giác quen thuộc TỪ `AND` (nơi CẢ hai nhánh THẬT
sự phải đúng).

Chỗ lệch: `OR` (`HoacNut`) CHỈ cần MỘT nhánh đúng LÀ đủ, KHÔNG có
khái niệm "nửa vời". `tuoi=20 < 18` LÀ `false`, `diem=95 > 90` LÀ
`true` — `false || true` LÀ `true`, một kết quả HOÀN toàn xác
định, không MƠ hồ gì cả.
::
:::
::::

::::code{#viet_danh_gia}
Hoàn thiện `danh_gia` — nhánh `HoacNut` gộp hai kết quả con bằng
`||`.

```rust title=starter
enum Token {
    So(i64), DinhDanh(Vec<char>), Chon, Tu, ODau, Va, Hoac, Phay, Bang, LonHon, NhoHon, KetThuc,
}
enum NoAst { SoSanh(Vec<char>, char, i64), VaNut(usize, usize), HoacNut(usize, usize) }
struct CayAst { nut: Vec<NoAst> }

struct Truong {
    ten: Vec<char>,
    gia_tri: i64,
}

fn tra_cuu(hang: &Vec<Truong>, ten: &Vec<char>) -> i64 {
    let mut i = 0;
    while i < hang.len() {
        if hang[i].ten == *ten { return hang[i].gia_tri; }
        i += 1;
    }
    0
}

fn danh_gia_so_sanh(hang: &Vec<Truong>, ten: &Vec<char>, dau: char, so: i64) -> bool {
    let gt = tra_cuu(hang, ten);
    if dau == '>' { return gt > so; }
    if dau == '<' { return gt < so; }
    gt == so
}

fn danh_gia(cay: &CayAst, i: usize, hang: &Vec<Truong>) -> bool {
    match &cay.nut[i] {
        NoAst::SoSanh(ten, dau, so) => danh_gia_so_sanh(hang, ten, *dau, *so),
        NoAst::VaNut(trai, phai) => danh_gia(cay, *trai, hang) && danh_gia(cay, *phai, hang),
        NoAst::HoacNut(trai, phai) => ___,
    }
}

fn main() {
    let mut cay = CayAst { nut: Vec::new() };
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    cay.nut.push(NoAst::SoSanh(tuoi, '<', 18));
    let diem: Vec<char> = vec!['d','i','e','m'];
    cay.nut.push(NoAst::SoSanh(diem, '>', 90));
    cay.nut.push(NoAst::HoacNut(0, 1));
    let goc = (cay.nut.len() - 1) as usize;

    let hang: Vec<Truong> = vec![
        Truong{ten:vec!['t','u','o','i'],gia_tri:20},
        Truong{ten:vec!['d','i','e','m'],gia_tri:95},
    ];
    println!("{}", danh_gia(&cay, goc, &hang));
}
```

```rust title=solution
enum Token {
    So(i64), DinhDanh(Vec<char>), Chon, Tu, ODau, Va, Hoac, Phay, Bang, LonHon, NhoHon, KetThuc,
}
enum NoAst { SoSanh(Vec<char>, char, i64), VaNut(usize, usize), HoacNut(usize, usize) }
struct CayAst { nut: Vec<NoAst> }

struct Truong {
    ten: Vec<char>,
    gia_tri: i64,
}

fn tra_cuu(hang: &Vec<Truong>, ten: &Vec<char>) -> i64 {
    let mut i = 0;
    while i < hang.len() {
        if hang[i].ten == *ten { return hang[i].gia_tri; }
        i += 1;
    }
    0
}

fn danh_gia_so_sanh(hang: &Vec<Truong>, ten: &Vec<char>, dau: char, so: i64) -> bool {
    let gt = tra_cuu(hang, ten);
    if dau == '>' { return gt > so; }
    if dau == '<' { return gt < so; }
    gt == so
}

fn danh_gia(cay: &CayAst, i: usize, hang: &Vec<Truong>) -> bool {
    match &cay.nut[i] {
        NoAst::SoSanh(ten, dau, so) => danh_gia_so_sanh(hang, ten, *dau, *so),
        NoAst::VaNut(trai, phai) => danh_gia(cay, *trai, hang) && danh_gia(cay, *phai, hang),
        NoAst::HoacNut(trai, phai) => danh_gia(cay, *trai, hang) || danh_gia(cay, *phai, hang),
    }
}

fn main() {
    let mut cay = CayAst { nut: Vec::new() };
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    cay.nut.push(NoAst::SoSanh(tuoi, '<', 18));
    let diem: Vec<char> = vec!['d','i','e','m'];
    cay.nut.push(NoAst::SoSanh(diem, '>', 90));
    cay.nut.push(NoAst::HoacNut(0, 1));
    let goc = (cay.nut.len() - 1) as usize;

    let hang: Vec<Truong> = vec![
        Truong{ten:vec!['t','u','o','i'],gia_tri:20},
        Truong{ten:vec!['d','i','e','m'],gia_tri:95},
    ];
    println!("{}", danh_gia(&cay, goc, &hang));
}
```

```rust title=test
enum Token {
    So(i64), DinhDanh(Vec<char>), Chon, Tu, ODau, Va, Hoac, Phay, Bang, LonHon, NhoHon, KetThuc,
}
enum NoAst { SoSanh(Vec<char>, char, i64), VaNut(usize, usize), HoacNut(usize, usize) }
struct CayAst { nut: Vec<NoAst> }

struct Truong {
    ten: Vec<char>,
    gia_tri: i64,
}

fn tra_cuu(hang: &Vec<Truong>, ten: &Vec<char>) -> i64 {
    let mut i = 0;
    while i < hang.len() {
        if hang[i].ten == *ten { return hang[i].gia_tri; }
        i += 1;
    }
    0
}

fn danh_gia_so_sanh(hang: &Vec<Truong>, ten: &Vec<char>, dau: char, so: i64) -> bool {
    let gt = tra_cuu(hang, ten);
    if dau == '>' { return gt > so; }
    if dau == '<' { return gt < so; }
    gt == so
}

fn main() {
    let mut cay = CayAst { nut: Vec::new() };
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    cay.nut.push(NoAst::SoSanh(tuoi, '<', 18));
    let diem: Vec<char> = vec!['d','i','e','m'];
    cay.nut.push(NoAst::SoSanh(diem, '>', 90));
    cay.nut.push(NoAst::HoacNut(0, 1));
    let goc = (cay.nut.len() - 1) as usize;

    let hang1: Vec<Truong> = vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}, Truong{ten:vec!['d','i','e','m'],gia_tri:50}];
    println!("{}", danh_gia(&cay, goc, &hang1));
    assert_eq!(danh_gia(&cay, goc, &hang1), true, "tuoi<18 dung -- OR true");

    let hang2: Vec<Truong> = vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}, Truong{ten:vec!['d','i','e','m'],gia_tri:50}];
    assert_eq!(danh_gia(&cay, goc, &hang2), false, "ca hai sai -- OR false");

    let hang3: Vec<Truong> = vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}, Truong{ten:vec!['d','i','e','m'],gia_tri:95}];
    assert_eq!(danh_gia(&cay, goc, &hang3), true, "diem>90 dung -- OR true");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Goi danh_gia de quy tren ca trai va phai, gop bang toan tu ||  -- mot dong."
- kind: strategy
  body: "danh_gia(cay, *trai, hang) || danh_gia(cay, *phai, hang)"
- kind: one-line
  body: "danh_gia(cay, *trai, hang) || danh_gia(cay, *phai, hang)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cả cây `WHERE` đánh giá được rồi — trên MỘT hàng. Chạy điều này TRÊN
cả bảng cần một kế HOẠCH — kế hoạch logic LÀ gì?
::::

::::reflect{#nghi-lai}
`danh_gia` đóng vòng đệ quy ĐÚNG cách: nút LÁ (`SoSanh`) gọi hàm
CHUYÊN biệt (bài 3), nút TRONG (`VaNut`/`HoacNut`) tự đệ quy VÀO
CẢ hai nhánh con RỒI gộp bằng đúng phép toán boolean. `cay: &CayAst`
tái sử dụng được TRONG cả biểu thức `&&`/`||` — không hạn CHẾ gì,
đúng lớp tham chiếu BẤT biến đã vá TỪ đầu q07. Bây giờ CÓ thể đánh
giá `WHERE` TRÊN một hàng — nhưng CHẠY một câu truy vấn nghĩa LÀ áp
dụng NÓ trên MỌI hàng của bảng, VÀ chỉ giữ đúng những trường được
`SELECT`. Trình tự "quét → lọc → chọn cột" đó gọi LÀ một KẾ HOẠCH
LOGIC — nó LÀ gì?
::::

::::checkpoint{mastery=0.8}
::::
