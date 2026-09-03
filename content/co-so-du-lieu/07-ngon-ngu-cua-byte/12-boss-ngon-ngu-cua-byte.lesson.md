---
id: co-so-du-lieu.ngon-ngu-cua-byte.boss-ngon-ngu-cua-byte
title: "BOSS — Ngôn ngữ của Byte"
summary: "Ghép TRỌN quest: lex('SELECT tuoi, ten FROM nguoi WHERE tuoi > 18 AND diem < 100') tạo 15 token, parse thành CauTruyVan (2 trường, bảng nguoi, co_where=true) và một CayAst 3 nút (SoSanh tuoi>18, SoSanh diem<100, VaNut gộp cả hai — nút gốc chính là AND vì WHERE chỉ có một AND, không OR). Cắt phạm vi có chủ đích: giá trị so sánh chỉ số nguyên, không dấu ngoặc tường minh trong WHERE."
locale: vi
track: co-so-du-lieu
module: ngon-ngu-cua-byte
order: 12
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.parser-cau-truy-van]
concepts: [db.boss-q07]
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
Lexer VÀ parser đều xong. Ráp `lex` → `parse` thành MỘT pipeline
đầy đủ, chạy trên một câu truy vấn THẬT sự — trông ra sao?
::::

::::explain{#pipeline-day-du}
`lex` biến chuỗi thành `Vec<Token>`, `parse` biến `Vec<Token>`
thành `CauTruyVan` + một `CayAst` cho phần `WHERE`:

```rust title=readonly
fn main() {
    let s = String::from("SELECT tuoi, ten FROM nguoi WHERE tuoi > 18 AND diem < 100");
    let ky_tu: Vec<char> = s.chars().collect();
    let tk = lex(&ky_tu);
    println!("{}", tk.len());

    let mut cay = CayAst { nut: Vec::new() };
    let ctv = parse(&mut cay, &tk);
    println!("{}", ctv.truong.len());
    println!("{:?}", ctv.truong[0]);
    println!("{:?}", ctv.truong[1]);
    println!("{:?}", ctv.bang);
    println!("{}", ctv.co_where);
    println!("{}", cay.nut.len());
}
```

```text title=readonly
15
2
['t', 'u', 'o', 'i']
['t', 'e', 'n']
['n', 'g', 'u', 'o', 'i']
true
3
```

Mười lăm token (`SELECT`,`tuoi`,`,`,`ten`,`FROM`,`nguoi`,`WHERE`,
`tuoi`,`>`,`18`,`AND`,`diem`,`<`,`100`,`KetThuc`). Hai trường
(`tuoi`, `ten`), bảng `nguoi`, `co_where=true`. `cay.nut.len()=3`:
hai `SoSanh` (`tuoi>18`, `diem<100`) CỘNG một `VaNut` gộp CHÚNG —
`WHERE` NÀY chỉ CÓ một `AND`, không `OR`, nên KHÔNG có `HoacNut`
nào cả.
::::

::::example{#nut-goc-la-and}
Nút GỐC (chỉ số cuối cùng) chính LÀ `AND` — vì `WHERE` chỉ CÓ đúng
một toán tử GHÉP:

```rust title=readonly
fn ten_nut(cay: &CayAst, i: usize) -> Vec<char> {
    match &cay.nut[i] {
        NoAst::SoSanh(truong, dau, so) => truong.clone(),
        NoAst::VaNut(_, _) => vec!['A', 'N', 'D'],
        NoAst::HoacNut(_, _) => vec!['O', 'R'],
    }
}

let goc = (cay.nut.len() - 1) as usize;
println!("{:?}", ten_nut(&cay, goc));
```

```text title=readonly
['A', 'N', 'D']
```

Khác VỚI ví dụ "a>1 OR b>2 AND c>3" (bài 10, nút GỐC LÀ `OR`) — Ở
ĐÂY chỉ CÓ một `AND` DUY nhất, nên nút CUỐI được thêm VÀO arena
(luôn LÀ nút GỐC của cây, đúng cách `parse_bieu_thuc` xây dựng)
CHÍNH LÀ `VaNut` đó.
::::

::::predict{#doan-khong-where commitOnce}
Câu truy vấn KHÔNG có `WHERE`:

```rust
let s2 = String::from("SELECT ten FROM ban");
let ky_tu2: Vec<char> = s2.chars().collect();
let tk2 = lex(&ky_tu2);
let mut cay2 = CayAst { nut: Vec::new() };
let ctv2 = parse(&mut cay2, &tk2);
println!("{} {}", ctv2.co_where, cay2.nut.len());
```

Dòng cuối in ra gì?

:::opt{correct}
`false 0`
:::

:::opt
Máy báo lỗi — vì `cay2` được TẠO ra nhưng KHÔNG hề nhận nút nào,
một arena RỖNG hoàn toàn LÀ trạng thái không hợp lệ
::why
Gần đúng ở việc bạn nghĩ TỚI "arena rỗng" như một trường hợp ĐÁNG
ngờ — một trực giác thận trọng khi làm việc VỚI cấu trúc dữ liệu.

Chỗ lệch: `CayAst { nut: Vec::new() }` LÀ một arena HOÀN toàn hợp
lệ, RỖNG chỉ đơn giản nghĩa LÀ "chưa có nút nào" — trạng thái BAN
đầu tự nhiên của MỌI arena trước khi có gì được `push`. Không CÓ
`WHERE` thì `parse_bieu_thuc` KHÔNG bao giờ được GỌI (bài 11) — arena
giữ nguyên rỗng LÀ kết quả ĐÚNG, không phải lỗi.
::
:::
::::

::::code{#viet_boss}
Hoàn thiện pipeline — sau khi `lex` xong, gọi `parse` để lấy CẢ câu
truy vấn.

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

struct KetQuaDoc { gia_tri: i64, vi_tri: usize }
struct KetQuaChu { ten: Vec<char>, vi_tri: usize }
struct CauTruyVan {
    truong: Vec<Vec<char> >,
    bang: Vec<char>,
    co_where: bool,
}
struct KetQuaPipeline {
    tk: Vec<Token>,
    ctv: CauTruyVan,
}

fn la_so(c: char) -> bool { c >= '0' && c <= '9' }
fn la_chu(c: char) -> bool { (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_' }

fn doc_so(ky_tu: &Vec<char>, bat_dau: usize) -> KetQuaDoc {
    let mut n: i64 = 0;
    let mut i = bat_dau;
    while i < ky_tu.len() && la_so(ky_tu[i]) {
        let chu_so = (ky_tu[i] as i64) - ('0' as i64);
        n = n * 10 + chu_so;
        i += 1;
    };
    KetQuaDoc { gia_tri: n, vi_tri: i }
}

fn doc_dinh_danh(ky_tu: &Vec<char>, bat_dau: usize) -> KetQuaChu {
    let mut ten: Vec<char> = Vec::new();
    let mut i = bat_dau;
    while i < ky_tu.len() && la_chu(ky_tu[i]) {
        ten.push(ky_tu[i]);
        i += 1;
    };
    KetQuaChu { ten: ten, vi_tri: i }
}

fn phan_loai(ten: &Vec<char>) -> Token {
    let chon: Vec<char> = vec!['S','E','L','E','C','T'];
    let tu: Vec<char> = vec!['F','R','O','M'];
    let o_dau: Vec<char> = vec!['W','H','E','R','E'];
    let va: Vec<char> = vec!['A','N','D'];
    let hoac: Vec<char> = vec!['O','R'];
    if *ten == chon { return Token::Chon; }
    if *ten == tu { return Token::Tu; }
    if *ten == o_dau { return Token::ODau; }
    if *ten == va { return Token::Va; }
    if *ten == hoac { return Token::Hoac; }
    Token::DinhDanh(ten.clone())
}

fn doc_dau(ky_tu: &Vec<char>, vi_tri: usize) -> Token {
    match ky_tu[vi_tri] {
        ',' => Token::Phay,
        '=' => Token::Bang,
        '>' => Token::LonHon,
        '<' => Token::NhoHon,
        _ => Token::Phay,
    }
}

fn bo_qua_trang(ky_tu: &Vec<char>, vi_tri: usize) -> usize {
    let mut i = vi_tri;
    while i < ky_tu.len() && ky_tu[i] == ' ' {
        i += 1;
    };
    i
}

fn lex(ky_tu: &Vec<char>) -> Vec<Token> {
    let mut ket_qua: Vec<Token> = Vec::new();
    let mut i = bo_qua_trang(ky_tu, 0) as usize;
    while i < ky_tu.len() {
        if la_so(ky_tu[i]) {
            ket_qua.push(Token::So(doc_so(ky_tu, i).gia_tri));
            i = doc_so(ky_tu, i).vi_tri;
        } else if la_chu(ky_tu[i]) {
            ket_qua.push(phan_loai(&doc_dinh_danh(ky_tu, i).ten));
            i = doc_dinh_danh(ky_tu, i).vi_tri;
        } else {
            ket_qua.push(doc_dau(ky_tu, i));
            i = i + 1;
        }
        i = bo_qua_trang(ky_tu, i) as usize;
    };
    ket_qua.push(Token::KetThuc);
    ket_qua
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

fn chay_pipeline(cay: &mut CayAst, ky_tu: &Vec<char>) -> KetQuaPipeline {
    let tk = lex(ky_tu);
    let ctv = ___;
    KetQuaPipeline { tk: tk, ctv: ctv }
}

fn main() {
    let s = String::from("SELECT tuoi FROM nguoi WHERE tuoi > 18");
    let ky_tu: Vec<char> = s.chars().collect();
    let mut cay = CayAst { nut: Vec::new() };
    let kq = chay_pipeline(&mut cay, &ky_tu);
    println!("{} {} {}", kq.tk.len(), kq.ctv.truong.len(), cay.nut.len());
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

struct KetQuaDoc { gia_tri: i64, vi_tri: usize }
struct KetQuaChu { ten: Vec<char>, vi_tri: usize }
struct CauTruyVan {
    truong: Vec<Vec<char> >,
    bang: Vec<char>,
    co_where: bool,
}
struct KetQuaPipeline {
    tk: Vec<Token>,
    ctv: CauTruyVan,
}

fn la_so(c: char) -> bool { c >= '0' && c <= '9' }
fn la_chu(c: char) -> bool { (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_' }

fn doc_so(ky_tu: &Vec<char>, bat_dau: usize) -> KetQuaDoc {
    let mut n: i64 = 0;
    let mut i = bat_dau;
    while i < ky_tu.len() && la_so(ky_tu[i]) {
        let chu_so = (ky_tu[i] as i64) - ('0' as i64);
        n = n * 10 + chu_so;
        i += 1;
    };
    KetQuaDoc { gia_tri: n, vi_tri: i }
}

fn doc_dinh_danh(ky_tu: &Vec<char>, bat_dau: usize) -> KetQuaChu {
    let mut ten: Vec<char> = Vec::new();
    let mut i = bat_dau;
    while i < ky_tu.len() && la_chu(ky_tu[i]) {
        ten.push(ky_tu[i]);
        i += 1;
    };
    KetQuaChu { ten: ten, vi_tri: i }
}

fn phan_loai(ten: &Vec<char>) -> Token {
    let chon: Vec<char> = vec!['S','E','L','E','C','T'];
    let tu: Vec<char> = vec!['F','R','O','M'];
    let o_dau: Vec<char> = vec!['W','H','E','R','E'];
    let va: Vec<char> = vec!['A','N','D'];
    let hoac: Vec<char> = vec!['O','R'];
    if *ten == chon { return Token::Chon; }
    if *ten == tu { return Token::Tu; }
    if *ten == o_dau { return Token::ODau; }
    if *ten == va { return Token::Va; }
    if *ten == hoac { return Token::Hoac; }
    Token::DinhDanh(ten.clone())
}

fn doc_dau(ky_tu: &Vec<char>, vi_tri: usize) -> Token {
    match ky_tu[vi_tri] {
        ',' => Token::Phay,
        '=' => Token::Bang,
        '>' => Token::LonHon,
        '<' => Token::NhoHon,
        _ => Token::Phay,
    }
}

fn bo_qua_trang(ky_tu: &Vec<char>, vi_tri: usize) -> usize {
    let mut i = vi_tri;
    while i < ky_tu.len() && ky_tu[i] == ' ' {
        i += 1;
    };
    i
}

fn lex(ky_tu: &Vec<char>) -> Vec<Token> {
    let mut ket_qua: Vec<Token> = Vec::new();
    let mut i = bo_qua_trang(ky_tu, 0) as usize;
    while i < ky_tu.len() {
        if la_so(ky_tu[i]) {
            ket_qua.push(Token::So(doc_so(ky_tu, i).gia_tri));
            i = doc_so(ky_tu, i).vi_tri;
        } else if la_chu(ky_tu[i]) {
            ket_qua.push(phan_loai(&doc_dinh_danh(ky_tu, i).ten));
            i = doc_dinh_danh(ky_tu, i).vi_tri;
        } else {
            ket_qua.push(doc_dau(ky_tu, i));
            i = i + 1;
        }
        i = bo_qua_trang(ky_tu, i) as usize;
    };
    ket_qua.push(Token::KetThuc);
    ket_qua
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

fn chay_pipeline(cay: &mut CayAst, ky_tu: &Vec<char>) -> KetQuaPipeline {
    let tk = lex(ky_tu);
    let ctv = parse(cay, &tk);
    KetQuaPipeline { tk: tk, ctv: ctv }
}

fn main() {
    let s = String::from("SELECT tuoi FROM nguoi WHERE tuoi > 18");
    let ky_tu: Vec<char> = s.chars().collect();
    let mut cay = CayAst { nut: Vec::new() };
    let kq = chay_pipeline(&mut cay, &ky_tu);
    println!("{} {} {}", kq.tk.len(), kq.ctv.truong.len(), cay.nut.len());
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
    let s = String::from("SELECT tuoi, ten FROM nguoi WHERE tuoi > 18 AND diem < 100");
    let ky_tu: Vec<char> = s.chars().collect();
    let mut cay = CayAst { nut: Vec::new() };
    let kq = chay_pipeline(&mut cay, &ky_tu);
    println!("{}", kq.tk.len());
    assert_eq!(kq.tk.len(), 15, "cau truy van mau phai co dung 15 token");
    assert_eq!(kq.ctv.truong.len(), 2, "phai doc dung 2 truong");
    assert_eq!(kq.ctv.truong[0], vec!['t','u','o','i'], "truong dau la tuoi");
    assert_eq!(kq.ctv.truong[1], vec!['t','e','n'], "truong hai la ten");
    assert_eq!(kq.ctv.bang, vec!['n','g','u','o','i'], "ten bang phai la nguoi");
    assert_eq!(kq.ctv.co_where, true, "phai phat hien co WHERE");
    assert_eq!(cay.nut.len(), 3, "2 SoSanh + 1 VaNut");

    let goc = (cay.nut.len() - 1) as usize;
    assert_eq!(ten_nut(&cay, goc), vec!['A','N','D'], "nut goc phai la AND (chi mot AND, khong OR)");

    let s2 = String::from("SELECT ten FROM ban");
    let ky_tu2: Vec<char> = s2.chars().collect();
    let mut cay2 = CayAst { nut: Vec::new() };
    let kq2 = chay_pipeline(&mut cay2, &ky_tu2);
    assert_eq!(kq2.ctv.co_where, false, "khong WHERE -- co_where false");
    assert_eq!(cay2.nut.len(), 0, "khong WHERE -- cay rong");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Trong chay_pipeline, goi ham parse voi cay va &tk de lay ve CauTruyVan -- mot dong."
- kind: strategy
  body: "parse(cay, &tk)"
- kind: one-line
  body: "parse(cay, &tk)"
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "15"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chuỗi ký tự đã thành cây cú pháp có cấu trúc — lexer VÀ Pratt parser
hoàn chỉnh. Quest "Ngôn ngữ của Byte" khép LẠI — R6-2 mở màn xong.
::::

::::reflect{#nghi-lai}
q07 xây trọn pipeline `lex → parse` cho một mini-SurrealQL: năm hàm
đọc token (số, khoảng trắng, định danh, từ khoá, dấu) ráp thành
`lex`, RỒI bốn hàm parse theo tầng ưu tiên (`parse_so_sanh` →
`parse_va` → `parse_bieu_thuc` → `parse`) ráp thành một cây `CayAst`
kiểu ARENA. Ba ràng buộc kiến trúc xuất hiện xuyên suốt — arena
thay `Box` (bài 7), `Vec<char>` thay `String` (bài 1-6), VÀ CÁCH
truyền `&mut CayAst` an toàn qua nhiều lời gọi hàm (vá được TRONG
lúc viết quest NÀY, xem `docs/decisions/R4-T40b-su-that-do-duoc.md`)
— đều LÀ hệ quả trực tiếp của việc `byte-rust` (interpreter Rust
CỦA riêng track này) còn LÀ một tập con hẹp, không phải Rust thật.

**Phạm vi cắt có chủ đích**: giá trị so sánh trong `WHERE` CHỈ LÀ số
nguyên (không chuỗi trong dấu nháy — `String` không mutate được để
xây text động từ ký tự rời, xem bài 1). Không CÓ dấu ngoặc tường
minh trong `bieu_thuc` (chỉ độ ưu tiên AND/OR tự nhiên quyết định
nhóm, không `(...)` ghi đè) — cả hai đều nằm NGOÀI phạm vi 12 bài,
để dành cho q08 "Bộ não của truy vấn" nếu cần mở RỘNG ngữ pháp.

q08 dùng THẲNG `CayAst` NÀY làm đầu VÀO cho logical plan VÀ Volcano
executor — cây cú pháp vừa xây SẼ được "chạy" thật, không CHỈ đọc.
::::

::::checkpoint{mastery=0.85}
::::
