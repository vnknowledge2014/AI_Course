---
id: co-so-du-lieu.ngon-ngu-cua-byte.parse-cau-truy-van
title: "Parse trọn một câu SELECT"
summary: "parse(cay: &mut CayAst, tk: &Vec<Token>) -> CauTruyVan ráp toàn bộ: khớp SELECT, đọc danh sách trường cách nhau bởi dấu phẩy (vòng lặp, không đệ quy — danh sách không cần Pratt), khớp FROM, đọc một định danh làm tên bảng, rồi NẾU token tiếp là WHERE thì parse_bieu_thuc cho phần điều kiện. co_where báo có/không có WHERE — cây AST của WHERE (nếu có) nằm trong cay, không trong CauTruyVan."
locale: vi
track: co-so-du-lieu
module: ngon-ngu-cua-byte
order: 11
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [db.parser-cau-truy-van]
requires: [db.parser-and-or]
concepts: [db.parser-cau-truy-van]
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
`AND`/`OR` gộp đúng độ ưu tiên (bài trước). Còn lại: khung câu —
`SELECT ... FROM ... WHERE ...` — VÀ danh sách trường cách nhau
bởi dấu phẩy.
::::

::::explain{#parse-cau-truy-van}
`parse` ráp TOÀN bộ: khớp `SELECT`, đọc danh sách trường (vòng lặp
qua dấu phẩy — KHÔNG cần Pratt, danh sách không có độ ưu tiên),
khớp `FROM`, đọc tên bảng, rồi TUỲ chọn `WHERE`:

```rust title=readonly
struct CauTruyVan {
    truong: Vec<Vec<char> >,
    bang: Vec<char>,
    co_where: bool,
}

fn parse(cay: &mut CayAst, tk: &Vec<Token>) -> CauTruyVan {
    let mut truong: Vec<Vec<char> > = Vec::new();
    let mut vt: usize = 1;
    match &tk[vt] {
        Token::DinhDanh(ten) => truong.push(ten.clone()),
        _ => {},
    }
    vt = 1 + vt;
    while vt < tk.len() {
        let la_phay = match &tk[vt] {
            Token::Phay => true,
            _ => false,
        };
        if !la_phay {
            break;
        }
        match &tk[1 + vt] {
            Token::DinhDanh(ten) => truong.push(ten.clone()),
            _ => {},
        }
        vt = 2 + vt;
    }
    vt = 1 + vt;
    let bang = match &tk[vt] {
        Token::DinhDanh(ten) => ten.clone(),
        _ => Vec::new(),
    };
    vt = 1 + vt;
    let co_where = match &tk[vt] {
        Token::ODau => true,
        _ => false,
    };
    if co_where {
        parse_bieu_thuc(cay, tk, 1 + vt);
    }
    CauTruyVan { truong: truong, bang: bang, co_where: co_where }
}
```

`vt` bắt đầu Ở `1` (bỏ qua `Token::Chon`, đã BIẾT chắc LÀ token đầu
tiên). Đọc trường đầu TIÊN, RỒI lặp: còn dấu phẩy thì đọc thêm MỘT
trường, hết dấu phẩy thì dừng. `Vec<Vec<char> >` (CÓ dấu cách trước
`>` cuối) — `Vec<Vec<char>>` không dấu cách bị đọc NHẦM thành toán
tử dịch-phải `>>`.
::::

::::example{#khong-co-where}
Câu KHÔNG có `WHERE` — `co_where` LÀ `false`, `cay` KHÔNG hề nhận
thêm nút nào:

```rust title=readonly
let tk2: Vec<Token> = vec![
    Token::Chon,
    Token::DinhDanh(vec!['t','e','n']),
    Token::Tu,
    Token::DinhDanh(vec!['b','a','n']),
    Token::KetThuc,
];
let mut cay2 = CayAst { nut: Vec::new() };
let ctv2 = parse(&mut cay2, &tk2);
println!("{}", ctv2.truong.len());
println!("{:?}", ctv2.bang);
println!("{}", ctv2.co_where);
```

```text title=readonly
1
['b', 'a', 'n']
false
```

Sau khi đọc XONG tên bảng (`vt` trỏ ĐÚNG vị trí `Token::KetThuc`),
`match &tk[vt] { Token::ODau => true, _ => false }` SAI (gặp
`KetThuc`, không phải `ODau`) — `co_where` LÀ `false`, `if co_where
{ ... }` KHÔNG chạy, `parse_bieu_thuc` KHÔNG được gọi. `cay` giữ
NGUYÊN rỗng.
::::

::::predict{#doan-ba-truong commitOnce}
Câu truy vấn CÓ BA trường, cách nhau bởi HAI dấu phẩy:

```rust
let tk3: Vec<Token> = vec![
    Token::Chon,
    Token::DinhDanh(vec!['a']),
    Token::Phay,
    Token::DinhDanh(vec!['b']),
    Token::Phay,
    Token::DinhDanh(vec!['c']),
    Token::Tu,
    Token::DinhDanh(vec!['t']),
    Token::KetThuc,
];
let mut cay3 = CayAst { nut: Vec::new() };
let ctv3 = parse(&mut cay3, &tk3);
println!("{}", ctv3.truong.len());
```

Dòng cuối in ra gì?

:::opt{correct}
`3`
:::

:::opt
`2` — vì vòng lặp đọc dấu phẩy CHỈ chạy được MỘT lần trong thiết kế
NÀY, giống hệt hạn chế thường gặp Ở các vòng lặp "đọc danh sách"
::why
Gần đúng ở việc bạn nghĩ TỚI khả năng vòng lặp bị GIỚI hạn số lần
— một mối lo hợp lý khi đọc code lặp LẦN đầu.

Chỗ lệch: `while vt < tk.len() { ... if !la_phay { break; } ... }`
KHÔNG có giới hạn số lần LẶP cứng nào — nó lặp CHỪNG nào còn gặp
`Token::Phay`. Với hai dấu phẩy, vòng lặp CHẠY đúng hai lần (thêm
`b` RỒI thêm `c`), cộng VỚI trường ĐẦU (`a`, đọc TRƯỚC vòng lặp) LÀ
ba.
::
:::
::::

::::code{#viet_parse}
Hoàn thiện `parse` — phát hiện token `WHERE` (`Token::ODau`) TẠI vị
trí hiện tại.

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
        let trai = (cay.nut.len() - 1) as usize;
        let vt2 = parse_va(cay, tk, 1 + vt);
        let phai = (cay.nut.len() - 1) as usize;
        cay.nut.push(NoAst::HoacNut(trai, phai));
        vt = vt2;
    }
    vt
}

struct CauTruyVan {
    truong: Vec<Vec<char> >,
    bang: Vec<char>,
    co_where: bool,
}

fn parse(cay: &mut CayAst, tk: &Vec<Token>) -> CauTruyVan {
    let mut truong: Vec<Vec<char> > = Vec::new();
    let mut vt: usize = 1;
    match &tk[vt] {
        Token::DinhDanh(ten) => truong.push(ten.clone()),
        _ => {},
    }
    vt = 1 + vt;
    while vt < tk.len() {
        let la_phay = match &tk[vt] {
            Token::Phay => true,
            _ => false,
        };
        if !la_phay {
            break;
        }
        match &tk[1 + vt] {
            Token::DinhDanh(ten) => truong.push(ten.clone()),
            _ => {},
        }
        vt = 2 + vt;
    }
    vt = 1 + vt;
    let bang = match &tk[vt] {
        Token::DinhDanh(ten) => ten.clone(),
        _ => Vec::new(),
    };
    vt = 1 + vt;
    let co_where = match &tk[vt] {
        ___
        _ => false,
    };
    if co_where {
        parse_bieu_thuc(cay, tk, 1 + vt);
    }
    CauTruyVan { truong: truong, bang: bang, co_where: co_where }
}

fn main() {
    let tk: Vec<Token> = vec![
        Token::Chon,
        Token::DinhDanh(vec!['t','u','o','i']),
        Token::Tu,
        Token::DinhDanh(vec!['n','g','u','o','i']),
        Token::ODau,
        Token::DinhDanh(vec!['t','u','o','i']), Token::LonHon, Token::So(18),
        Token::KetThuc,
    ];
    let mut cay = CayAst { nut: Vec::new() };
    let ctv = parse(&mut cay, &tk);
    println!("{}", ctv.co_where);
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

struct CauTruyVan {
    truong: Vec<Vec<char> >,
    bang: Vec<char>,
    co_where: bool,
}

fn parse(cay: &mut CayAst, tk: &Vec<Token>) -> CauTruyVan {
    let mut truong: Vec<Vec<char> > = Vec::new();
    let mut vt: usize = 1;
    match &tk[vt] {
        Token::DinhDanh(ten) => truong.push(ten.clone()),
        _ => {},
    }
    vt = 1 + vt;
    while vt < tk.len() {
        let la_phay = match &tk[vt] {
            Token::Phay => true,
            _ => false,
        };
        if !la_phay {
            break;
        }
        match &tk[1 + vt] {
            Token::DinhDanh(ten) => truong.push(ten.clone()),
            _ => {},
        }
        vt = 2 + vt;
    }
    vt = 1 + vt;
    let bang = match &tk[vt] {
        Token::DinhDanh(ten) => ten.clone(),
        _ => Vec::new(),
    };
    vt = 1 + vt;
    let co_where = match &tk[vt] {
        Token::ODau => true,
        _ => false,
    };
    if co_where {
        parse_bieu_thuc(cay, tk, 1 + vt);
    }
    CauTruyVan { truong: truong, bang: bang, co_where: co_where }
}

fn main() {
    let tk: Vec<Token> = vec![
        Token::Chon,
        Token::DinhDanh(vec!['t','u','o','i']),
        Token::Tu,
        Token::DinhDanh(vec!['n','g','u','o','i']),
        Token::ODau,
        Token::DinhDanh(vec!['t','u','o','i']), Token::LonHon, Token::So(18),
        Token::KetThuc,
    ];
    let mut cay = CayAst { nut: Vec::new() };
    let ctv = parse(&mut cay, &tk);
    println!("{}", ctv.co_where);
}
```

```rust title=test
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
        Token::Chon,
        Token::DinhDanh(vec!['t','u','o','i']),
        Token::Phay,
        Token::DinhDanh(vec!['t','e','n']),
        Token::Tu,
        Token::DinhDanh(vec!['n','g','u','o','i']),
        Token::ODau,
        Token::DinhDanh(vec!['t','u','o','i']), Token::LonHon, Token::So(18),
        Token::KetThuc,
    ];
    let mut cay = CayAst { nut: Vec::new() };
    let ctv = parse(&mut cay, &tk);
    println!("{}", ctv.co_where);
    assert_eq!(ctv.truong.len(), 2, "phai doc dung 2 truong");
    assert_eq!(ctv.bang, vec!['n','g','u','o','i'], "ten bang phai la nguoi");
    assert_eq!(ctv.co_where, true, "phai phat hien co WHERE");
    assert_eq!(cay.nut.len(), 1, "WHERE co dung 1 phep so sanh -- 1 nut");

    let tk2: Vec<Token> = vec![
        Token::Chon,
        Token::DinhDanh(vec!['t','e','n']),
        Token::Tu,
        Token::DinhDanh(vec!['b','a','n']),
        Token::KetThuc,
    ];
    let mut cay2 = CayAst { nut: Vec::new() };
    let ctv2 = parse(&mut cay2, &tk2);
    assert_eq!(ctv2.co_where, false, "khong co WHERE -- co_where phai la false");
    assert_eq!(cay2.nut.len(), 0, "khong WHERE thi cay khong nhan them nut nao");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Nhanh dau tien cua match phai khop Token::ODau, tra ve true -- mot dong."
- kind: strategy
  body: "Token::ODau => true,"
- kind: one-line
  body: "Token::ODau => true,"
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Câu SELECT parse được TRỌN vẹn — trường, bảng, VÀ điều kiện WHERE
(nếu có). Ráp lex+parse thành MỘT pipeline đầy đủ, chạy trên một
câu truy vấn THẬT, trông ra sao?
::::

::::reflect{#nghi-lai}
`parse` LÀ tầng NGOÀI cùng của bộ parser — nó KHÔNG cần Pratt (danh
sách trường chỉ LÀ một vòng lặp đơn giản, không có độ ưu tiên nào
để tính), CHỈ gọi `parse_bieu_thuc` (bài trước) đúng MỘT lần khi gặp
`WHERE`. `CauTruyVan` giữ trường/bảng/cờ `co_where` — cây AST của
`WHERE` (nếu CÓ) nằm TRONG `cay` (arena dùng chung), không nhân đôi
trong `CauTruyVan`. Lexer (nửa đầu quest) VÀ parser (nửa sau) giờ
đã đủ CẢ hai — ráp chúng thành một pipeline `lex → parse` chạy trên
một câu truy vấn ĐẦY đủ trông ra sao?
::::

::::checkpoint{mastery=0.8}
::::
