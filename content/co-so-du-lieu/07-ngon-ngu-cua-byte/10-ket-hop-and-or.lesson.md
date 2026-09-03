---
id: co-so-du-lieu.ngon-ngu-cua-byte.ket-hop-and-or
title: "Kết hợp bằng AND/OR theo đúng độ ưu tiên"
summary: "parse_va lặp: parse một so_sanh, gặp AND thì gộp với so_sanh KẾ TIẾP thành VaNut, lặp lại — AND bó chặt trước. parse_bieu_thuc lặp tương tự trên OR, gọi parse_va (không phải parse_so_sanh) cho mỗi vế — nhờ đó mọi chuỗi AND liên tiếp gộp XONG trước khi OR chạm tới. Chỉ số trai PHẢI chụp TRƯỚC khi đệ quy vế phải (vế phải có thể tự nó là nhiều nút), không phải cay.nut.len()-2 sau đó."
locale: vi
track: co-so-du-lieu
module: ngon-ngu-cua-byte
order: 10
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [db.parser-and-or]
requires: [db.parser-so-sanh]
concepts: [db.parser-and-or]
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
Một phép so sánh parse được rồi (bài trước). Ghép NHIỀU phép so
sánh bằng `AND`/`OR`, đúng độ ưu tiên (bài 8) — AND bó CHẶT hơn —
trông ra sao?
::::

::::explain{#parse-va}
`parse_va` lặp: parse một `so_sanh`, gặp `AND` (`Token::Va`) thì
GỘP với `so_sanh` KẾ TIẾP thành `NoAst::VaNut`, lặp lại tới khi hết
`AND` liên tiếp:

```rust title=readonly
fn parse_va(cay: &mut CayAst, tk: &Vec<Token>, vi_tri: usize) -> usize {
    let mut vt = parse_so_sanh(cay, tk, vi_tri);
    while vt < tk.len() {
        let tiep = match &tk[vt] {
            Token::Va => true,
            _ => false,
        };
        if !tiep {
            break;
        }
        let trai = (cay.nut.len() - 1) as usize;
        let vt2 = parse_so_sanh(cay, tk, 1 + vt);
        let phai = (cay.nut.len() - 1) as usize;
        cay.nut.push(NoAst::VaNut(trai, phai));
        vt = vt2;
    }
    vt
}
```

`trai` PHẢI chụp NGAY TRƯỚC khi gọi `parse_so_sanh` cho vế PHẢI —
`cay.nut.len() - 1` TẠI thời điểm đó LÀ chỉ số nút bên TRÁI hiện có
(dù đó LÀ một `SoSanh` đơn hay một `VaNut` đã gộp TỪ vòng lặp
trước). Nếu chụp `trai` SAU khi gọi `parse_so_sanh` (như `phai - 1`
chẳng hạn), kết quả SẼ sai bất cứ khi nào vế phải tự nó thêm NHIỀU
hơn một nút. `cay: &mut CayAst` giờ dùng LẠI được xuyên suốt nhiều
lời gọi `parse_so_sanh` — vá được nhờ `&mut T` truyền làm đối số
LÀ mượn lại (reborrow), không phải move.
::::

::::example{#parse-bieu-thuc-goi-va}
`parse_bieu_thuc` lặp TƯƠNG tự trên `OR` (`Token::Hoac`) — nhưng gọi
`parse_va` (KHÔNG phải `parse_so_sanh`) cho MỖI vế:

```rust title=readonly
fn parse_bieu_thuc(cay: &mut CayAst, tk: &Vec<Token>, vi_tri: usize) -> usize {
    let mut vt = parse_va(cay, tk, vi_tri);
    while vt < tk.len() {
        let tiep = match &tk[vt] {
            Token::Hoac => true,
            _ => false,
        };
        if !tiep {
            break;
        }
        let trai = (cay.nut.len() - 1) as usize;
        let vt2 = parse_va(cay, tk, 1 + vt);
        let phai = (cay.nut.len() - 1) as usize;
        cay.nut.push(NoAst::HoacNut(trai, phai));
        vt = vt2;
    }
    vt
}
```

`"a > 1 OR b > 2 AND c > 3"`: `parse_bieu_thuc` gọi `parse_va` cho
vế TRÁI của `OR` đầu tiên — `parse_va` CHỈ thấy `a > 1` (token TIẾP
theo LÀ `OR`, không phải `AND`), trả VỀ ngay SAU một nút. Rồi gặp
`OR`, gọi `parse_va` LẦN nữa cho phần CÒN lại — LẦN này `parse_va`
thấy `b > 2 AND c > 3`, tự GỘP chúng thành MỘT `VaNut` TRƯỚC khi trả
về. Kết quả: `a > 1 OR (b > 2 AND c > 3)` — đúng độ ưu tiên, vì
`AND` được xử lý HOÀN toàn Ở TẦNG trong (`parse_va`) trước khi tầng
NGOÀI (`parse_bieu_thuc`) chạm tới.
::::

::::predict{#doan-chi-co-and commitOnce}
Một biểu thức CHỈ có `AND`, không `OR`: `"a > 1 AND b > 2 AND c > 3"`.
Gọi `parse_bieu_thuc` TRÊN dãy token TƯƠNG ứng, rồi đếm số nút
trong cây:

```rust
let tk: Vec<Token> = vec![
    Token::DinhDanh(vec!['a']), Token::LonHon, Token::So(1),
    Token::Va,
    Token::DinhDanh(vec!['b']), Token::LonHon, Token::So(2),
    Token::Va,
    Token::DinhDanh(vec!['c']), Token::LonHon, Token::So(3),
    Token::KetThuc,
];
let mut cay = CayAst { nut: Vec::new() };
parse_bieu_thuc(&mut cay, &tk, 0);
println!("{}", cay.nut.len());
```

Dòng cuối in ra gì?

:::opt{correct}
`5`
:::

:::opt
`3` — vì chỉ có BA phép so sánh (`a>1`, `b>2`, `c>3`), số nút phải
đúng BẰNG số phép so sánh
::why
Gần đúng ở việc bạn đếm ĐÚNG ba phép so sánh trong biểu thức GỐC —
một quan sát chính xác VỀ nội dung.

Chỗ lệch: MỖI lần gộp bằng `AND` cũng thêm MỘT nút MỚI vào `cay.nut`
(`NoAst::VaNut`), không CHỈ ba nút `SoSanh`. Ba `SoSanh` (`a>1`,
`b>2`, `c>3`) CỘNG hai `VaNut` (gộp `a>1` VỚI `b>2`, RỒI gộp kết
quả ĐÓ với `c>3`) LÀ năm nút.
::
:::

:::opt
`2` — vì `parse_va` chỉ tạo MỘT nút `VaNut` DUY nhất cho CẢ chuỗi
ba phép so sánh, cộng với NÚT gốc LÀ hai
::why
Gần đúng ở việc bạn nghĩ TỚI "gộp cả chuỗi thành MỘT nút duy nhất"
— MỘT thiết kế cây khác (n-ary, mỗi nút CHỨA một DANH sách con thay
vì đúng hai) cũng hoàn toàn hợp LÝ, chỉ khác thiết kế NÀY.

Chỗ lệch: `NoAst::VaNut` LUÔN có ĐÚNG hai chỉ số con (`trai`,
`phai`) — cây NHỊ phân, không phải n-ary. Ba phép so sánh nối bằng
HAI `AND` cần HAI lần gộp NHỊ phân riêng biệt: `(a>1 AND b>2)` LÀ
một `VaNut`, RỒI `(kết quả đó) AND c>3` LÀ `VaNut` THỨ hai.
::
:::
::::

::::code{#viet_parse_bieu_thuc}
Hoàn thiện `parse_bieu_thuc` — chụp chỉ số nút TRÁI TRƯỚC khi đệ
quy vào vế PHẢI.

```rust title=starter
enum Token {
    So(i64),
    DinhDanh(Vec<char>),
    Chon,
    Tu,
    ODau,
    Va,
    Hoac,
    Phay,
    Bang,
    LonHon,
    NhoHon,
    KetThuc,
}

enum NoAst {
    SoSanh(Vec<char>, char, i64),
    VaNut(usize, usize),
    HoacNut(usize, usize),
}

struct CayAst {
    nut: Vec<NoAst>,
}

fn toan_tu_cua(t: &Token) -> char {
    match t {
        Token::Bang => '=',
        Token::LonHon => '>',
        Token::NhoHon => '<',
        _ => '?',
    }
}

fn parse_so_sanh(cay: &mut CayAst, tk: &Vec<Token>, vi_tri: usize) -> usize {
    let truong = match &tk[vi_tri] {
        Token::DinhDanh(ten) => ten.clone(),
        _ => Vec::new(),
    };
    let dau = toan_tu_cua(&tk[1 + vi_tri]);
    let so = match &tk[2 + vi_tri] {
        Token::So(n) => *n,
        _ => 0,
    };
    cay.nut.push(NoAst::SoSanh(truong, dau, so));
    3 + vi_tri
}

fn parse_va(cay: &mut CayAst, tk: &Vec<Token>, vi_tri: usize) -> usize {
    let mut vt = parse_so_sanh(cay, tk, vi_tri);
    while vt < tk.len() {
        let tiep = match &tk[vt] {
            Token::Va => true,
            _ => false,
        };
        if !tiep {
            break;
        }
        let trai = (cay.nut.len() - 1) as usize;
        let vt2 = parse_so_sanh(cay, tk, 1 + vt);
        let phai = (cay.nut.len() - 1) as usize;
        cay.nut.push(NoAst::VaNut(trai, phai));
        vt = vt2;
    }
    vt
}

fn parse_bieu_thuc(cay: &mut CayAst, tk: &Vec<Token>, vi_tri: usize) -> usize {
    let mut vt = parse_va(cay, tk, vi_tri);
    while vt < tk.len() {
        let tiep = match &tk[vt] {
            Token::Hoac => true,
            _ => false,
        };
        if !tiep {
            break;
        }
        ___
        let vt2 = parse_va(cay, tk, 1 + vt);
        let phai = (cay.nut.len() - 1) as usize;
        cay.nut.push(NoAst::HoacNut(trai, phai));
        vt = vt2;
    }
    vt
}

fn main() {
    let tk: Vec<Token> = vec![
        Token::DinhDanh(vec!['a']), Token::LonHon, Token::So(1),
        Token::Hoac,
        Token::DinhDanh(vec!['b']), Token::LonHon, Token::So(2),
        Token::Va,
        Token::DinhDanh(vec!['c']), Token::LonHon, Token::So(3),
        Token::KetThuc,
    ];
    let mut cay = CayAst { nut: Vec::new() };
    parse_bieu_thuc(&mut cay, &tk, 0);
    println!("{}", cay.nut.len());
}
```

```rust title=solution
enum Token {
    So(i64),
    DinhDanh(Vec<char>),
    Chon,
    Tu,
    ODau,
    Va,
    Hoac,
    Phay,
    Bang,
    LonHon,
    NhoHon,
    KetThuc,
}

enum NoAst {
    SoSanh(Vec<char>, char, i64),
    VaNut(usize, usize),
    HoacNut(usize, usize),
}

struct CayAst {
    nut: Vec<NoAst>,
}

fn toan_tu_cua(t: &Token) -> char {
    match t {
        Token::Bang => '=',
        Token::LonHon => '>',
        Token::NhoHon => '<',
        _ => '?',
    }
}

fn parse_so_sanh(cay: &mut CayAst, tk: &Vec<Token>, vi_tri: usize) -> usize {
    let truong = match &tk[vi_tri] {
        Token::DinhDanh(ten) => ten.clone(),
        _ => Vec::new(),
    };
    let dau = toan_tu_cua(&tk[1 + vi_tri]);
    let so = match &tk[2 + vi_tri] {
        Token::So(n) => *n,
        _ => 0,
    };
    cay.nut.push(NoAst::SoSanh(truong, dau, so));
    3 + vi_tri
}

fn parse_va(cay: &mut CayAst, tk: &Vec<Token>, vi_tri: usize) -> usize {
    let mut vt = parse_so_sanh(cay, tk, vi_tri);
    while vt < tk.len() {
        let tiep = match &tk[vt] {
            Token::Va => true,
            _ => false,
        };
        if !tiep {
            break;
        }
        let trai = (cay.nut.len() - 1) as usize;
        let vt2 = parse_so_sanh(cay, tk, 1 + vt);
        let phai = (cay.nut.len() - 1) as usize;
        cay.nut.push(NoAst::VaNut(trai, phai));
        vt = vt2;
    }
    vt
}

fn parse_bieu_thuc(cay: &mut CayAst, tk: &Vec<Token>, vi_tri: usize) -> usize {
    let mut vt = parse_va(cay, tk, vi_tri);
    while vt < tk.len() {
        let tiep = match &tk[vt] {
            Token::Hoac => true,
            _ => false,
        };
        if !tiep {
            break;
        }
        let trai = (cay.nut.len() - 1) as usize;
        let vt2 = parse_va(cay, tk, 1 + vt);
        let phai = (cay.nut.len() - 1) as usize;
        cay.nut.push(NoAst::HoacNut(trai, phai));
        vt = vt2;
    }
    vt
}

fn main() {
    let tk: Vec<Token> = vec![
        Token::DinhDanh(vec!['a']), Token::LonHon, Token::So(1),
        Token::Hoac,
        Token::DinhDanh(vec!['b']), Token::LonHon, Token::So(2),
        Token::Va,
        Token::DinhDanh(vec!['c']), Token::LonHon, Token::So(3),
        Token::KetThuc,
    ];
    let mut cay = CayAst { nut: Vec::new() };
    parse_bieu_thuc(&mut cay, &tk, 0);
    println!("{}", cay.nut.len());
}
```

```rust title=test
fn ten_nut(cay: &CayAst, i: usize) -> Vec<char> {
    match &cay.nut[i] {
        NoAst::SoSanh(truong, dau, so) => truong.clone(),
        NoAst::VaNut(_, _) => vec!['A', 'N', 'D'],
        NoAst::HoacNut(_, _) => vec!['O', 'R'],
    }
}

fn main() {
    let tk: Vec<Token> = vec![
        Token::DinhDanh(vec!['a']), Token::LonHon, Token::So(1),
        Token::Hoac,
        Token::DinhDanh(vec!['b']), Token::LonHon, Token::So(2),
        Token::Va,
        Token::DinhDanh(vec!['c']), Token::LonHon, Token::So(3),
        Token::KetThuc,
    ];
    let mut cay = CayAst { nut: Vec::new() };
    let vt = parse_bieu_thuc(&mut cay, &tk, 0);
    println!("{}", cay.nut.len());
    assert_eq!(vt, 11, "phai tieu thu ca 11 token (chua ke KetThuc)");
    assert_eq!(cay.nut.len(), 5, "3 SoSanh + 1 VaNut + 1 HoacNut");

    let goc = (cay.nut.len() - 1) as usize;
    let ten_goc = ten_nut(&cay, goc);
    assert_eq!(ten_goc, vec!['O', 'R'], "nut goc phai la OR -- OR bo chat hon nen o ngoai cung");

    match &cay.nut[goc] {
        NoAst::HoacNut(trai, phai) => {
            let ten_trai = ten_nut(&cay, *trai);
            let ten_phai = ten_nut(&cay, *phai);
            assert_eq!(ten_trai, vec!['a'], "ve trai cua OR phai la a (mot SoSanh don)");
            assert_eq!(ten_phai, vec!['A', 'N', 'D'], "ve phai cua OR phai la AND (b>2 AND c>3 da gop truoc)");
        },
        _ => panic!("nut goc phai la HoacNut"),
    }

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Truoc khi goi parse_va cho ve phai, chup lai chi so nut TRAI hien tai (cay.nut.len() - 1) vao bien trai -- mot dong."
- kind: strategy
  body: "let trai = (cay.nut.len() - 1) as usize;"
- kind: one-line
  body: "let trai = (cay.nut.len() - 1) as usize;"
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "5"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`AND` VÀ `OR` gộp đúng độ ưu tiên. Còn thiếu: `SELECT`, `FROM`, VÀ
danh sách trường — ráp TOÀN bộ câu truy vấn trông ra sao?
::::

::::reflect{#nghi-lai}
`parse_va` VÀ `parse_bieu_thuc` LÀ lõi Pratt/precedence-climbing của
quest: MỖI tầng ngữ pháp gọi tầng NGAY dưới nó (`parse_bieu_thuc`
gọi `parse_va`, `parse_va` gọi `parse_so_sanh`) thay VÌ gọi thẳng
xuống tầng thấp nhất — nhờ VẬY, mọi `AND` liên tiếp GỘP xong Ở TẦNG
trong trước khi `OR` Ở tầng NGOÀI kịp chạm tới, cho ra đúng nhóm
`a>1 OR (b>2 AND c>3)`. Chụp `trai` TRƯỚC khi đệ quy (không phải
tính NGƯỢC bằng `phai - 1` sau đó) LÀ chi tiết dễ sai NHẤT — vế
phải CÓ thể tự nó LÀ một cây con nhiều nút. Token VÀ biểu thức đã
parse được — còn thiếu khung CÂU: `SELECT ... FROM ... WHERE ...`.
::::

::::checkpoint{mastery=0.8}
::::
