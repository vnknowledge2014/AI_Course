---
id: co-so-du-lieu.bo-nao-cua-truy-van.boss-bo-nao-cua-truy-van
title: "BOSS — Bộ não của truy vấn"
summary: "Ghép TRỌN quest: 'SELECT ten FROM nguoi WHERE tuoi > 18' đi qua lex→parse (q07, nguyên vẹn) ra CauTruyVan+CayAst (1 nút SoSanh), xay_ke_hoach ra KeHoach 3 bước, rồi thuc_thi đọc kế hoạch để rút tham số cho tiep — lặp gọi tiep tới None, in đúng những hàng qua lọc VÀ đã chọn cột. Không WHERE thì KeHoach chỉ 2 bước, mọi hàng đều qua."
locale: vi
track: co-so-du-lieu
module: bo-nao-cua-truy-van
order: 12
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.executor-project-step]
concepts: [db.boss-q08]
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
Lexer, parser (q07), kế hoạch, VÀ executor Volcano (bài 5-11) đều
xong. Ráp TẤT cả thành MỘT pipeline, chạy trên một câu truy vấn
THẬT — trông ra sao?
::::

::::explain{#thuc-thi-doc-ke-hoach}
`thuc_thi` đọc `KeHoach` (bài 5-6) để RÚT ra tham số cho `tiep`
(bài 9-11), RỒI lặp gọi `tiep` tới khi HẾT hàng:

```rust title=readonly
fn thuc_thi(ke_hoach: &Vec<BuocKeHoach>, cay: &CayAst, bang: &Vec<Vec<Truong> >) {
    let mut co_loc = false;
    let mut chi_so_dk: usize = 0;
    let mut cot: Vec<Vec<char> > = Vec::new();
    let mut i = 0;
    while i < ke_hoach.len() {
        match &ke_hoach[i] {
            BuocKeHoach::Quet(_) => {},
            BuocKeHoach::Loc(idx) => { co_loc = true; chi_so_dk = *idx; },
            BuocKeHoach::ChonCot(danh_sach) => { cot = danh_sach.clone(); },
        }
        i = 1 + i;
    }
    let mut vi_tri: usize = 0;
    loop {
        let r = tiep(bang, &mut vi_tri, cay, &chi_so_dk, &co_loc, &cot);
        match &r {
            None => break,
            Some(hang) => println!("{}", hang[0].gia_tri),
        }
    }
}
```

Vòng ĐẦU (`while i < ke_hoach.len()`) chỉ ĐỌC kế hoạch — KHÔNG chạm
gì tới dữ liệu — để biết BA điều: CÓ lọc không (`co_loc`), lọc theo
điều kiện NÀO (`chi_so_dk` — chỉ số nút TRONG `CayAst`), VÀ giữ cột
nào (`cot`). Vòng SAU (`loop`) mới THỰC sự chạy `tiep`, LẶP lại tới
khi trả VỀ `None`.
::::

::::example{#chay-tron-mot-cau}
`"SELECT ten FROM nguoi WHERE tuoi > 18"` — parse (q07) RỒI xây kế
hoạch (bài 6) RỒI thực THI (bài này) TRÊN một bảng ba hàng:

```rust title=readonly
let s = String::from("SELECT ten FROM nguoi WHERE tuoi > 18");
let ky_tu: Vec<char> = s.chars().collect();
let tk = lex(&ky_tu);
let mut cay = CayAst { nut: Vec::new() };
let ctv = parse(&mut cay, &tk);
println!("{}", cay.nut.len());
let goc = (cay.nut.len() - 1) as usize;
let ke_hoach = xay_ke_hoach(&ctv, goc);
println!("{}", ke_hoach.len());

let bang: Vec<Vec<Truong> > = vec![
    vec![Truong{ten:vec!['t','e','n'],gia_tri:10}, Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
    vec![Truong{ten:vec!['t','e','n'],gia_tri:20}, Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
    vec![Truong{ten:vec!['t','e','n'],gia_tri:30}, Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
];
thuc_thi(&ke_hoach, &cay, &bang);
```

```text title=readonly
1
3
20
30
```

`WHERE` chỉ CÓ một phép SO sánh (`tuoi > 18`, KHÔNG `AND`/`OR`) nên
`cay.nut.len()=1`. Kế hoạch CÓ đủ ba bước (`Quet`, `Loc`, `ChonCot`)
nên `ke_hoach.len()=3`. Hàng ĐẦU (`tuoi=15`) bị LOẠI; hàng hai VÀ ba
(`tuoi=25`, `tuoi=30`) qua được, MỖI hàng chỉ CÒN đúng cột `ten` —
`20` RỒI `30`.
::::

::::predict{#doan-khong-where-boss commitOnce}
CÙNG bảng ba hàng Ở trên, nhưng câu truy vấn LÀ `"SELECT ten FROM
nguoi"` — KHÔNG có `WHERE`. `thuc_thi` sẽ IN ra những gì?

:::opt{correct}
`10`, `20`, `30` — CẢ ba hàng, THEO đúng thứ tự trong bảng
:::

:::opt
Không in gì CẢ, vì `chi_so_dk` khởi TẠO bằng `0` nhưng `cay.nut` lại
RỖNG (không CÓ `WHERE` nên KHÔNG có nút nào được PARSE) — chỉ số `0`
trỏ VÀO một arena rỗng LÀ lỗi
::why
Gần đúng ở việc bạn để Ý đúng chi TIẾT hiểm: `chi_so_dk` mang giá
trị `0` NGAY cả khi `cay.nut` rỗng — MỘT quan sát sắc SẢO về việc
`thuc_thi` khởi tạo `chi_so_dk` bằng MỘT con số "giả" trước khi ĐỌC
kế hoạch.

Chỗ lệch: `chi_so_dk=0` KHÔNG bao giờ được DÙNG tới trong trường hợp
NÀY, vì `co_loc` VẪN LÀ `false` (nhánh `BuocKeHoach::Loc` KHÔNG hề
xuất hiện TRONG kế hoạch hai bước — không `WHERE` thì `xay_ke_hoach`
không thêm `Loc`, bài 6). `hang_qua_duoc` (bài 10) kiểm TRA `!*co_loc`
TRƯỚC tiên — SAI thì trả `true` NGAY, không hề chạm tới `danh_gia`
hay `chi_so_dk`. Cả ba hàng đều qua được, in RA đủ `10`, `20`, `30`.
::
:::
::::

::::code{#viet_boss_q08}
Hoàn thiện `thuc_thi` — gọi `tiep` với đúng các tham số ĐÃ rút ra từ
kế hoạch, để lấy hàng tiếp theo.

```rust title=starter
enum Token {
    So(i64), DinhDanh(Vec<char>), Chon, Tu, ODau, Va, Hoac, Phay, Bang, LonHon, NhoHon, KetThuc,
}
enum NoAst { SoSanh(Vec<char>, char, i64), VaNut(usize, usize), HoacNut(usize, usize) }
struct CayAst { nut: Vec<NoAst> }
struct KetQuaDoc { gia_tri: i64, vi_tri: usize }
struct KetQuaChu { ten: Vec<char>, vi_tri: usize }
struct CauTruyVan { truong: Vec<Vec<char> >, bang: Vec<char>, co_where: bool }

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
    while i < ky_tu.len() && ky_tu[i] == ' ' { i += 1; };
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
        let tiep = match &tk[vt] { Token::Va => true, _ => false };
        if !tiep { break; }
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
        let tiep = match &tk[vt] { Token::Hoac => true, _ => false };
        if !tiep { break; }
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
    match &tk[vt] { Token::DinhDanh(ten) => truong.push(ten.clone()), _ => {} }
    vt = 1 + vt;
    while vt < tk.len() {
        let la_phay = match &tk[vt] { Token::Phay => true, _ => false };
        if !la_phay { break; }
        match &tk[1 + vt] { Token::DinhDanh(ten) => truong.push(ten.clone()), _ => {} }
        vt = 2 + vt;
    }
    vt = 1 + vt;
    let bang = match &tk[vt] { Token::DinhDanh(ten) => ten.clone(), _ => Vec::new() };
    vt = 1 + vt;
    let co_where = match &tk[vt] { Token::ODau => true, _ => false };
    if co_where { parse_bieu_thuc(cay, tk, 1 + vt); }
    CauTruyVan { truong: truong, bang: bang, co_where: co_where }
}

struct Truong { ten: Vec<char>, gia_tri: i64 }
enum BuocKeHoach { Quet(Vec<char>), Loc(usize), ChonCot(Vec<Vec<char> >) }

fn xay_ke_hoach(ctv: &CauTruyVan, chi_so_where: usize) -> Vec<BuocKeHoach> {
    let mut ke_hoach: Vec<BuocKeHoach> = Vec::new();
    ke_hoach.push(BuocKeHoach::Quet(ctv.bang.clone()));
    if ctv.co_where { ke_hoach.push(BuocKeHoach::Loc(chi_so_where)); }
    ke_hoach.push(BuocKeHoach::ChonCot(ctv.truong.clone()));
    ke_hoach
}

fn tra_cuu(hang: &Vec<Truong>, ten: &Vec<char>) -> i64 {
    let mut ket_qua = 0;
    let mut i = 0;
    while i < hang.len() {
        if hang[i].ten == *ten { ket_qua = hang[i].gia_tri; }
        i = 1 + i;
    }
    ket_qua
}

fn danh_gia_so_sanh(hang: &Vec<Truong>, ten: &Vec<char>, dau: char, so: i64) -> bool {
    let gia_tri = tra_cuu(hang, ten);
    if dau == '>' { gia_tri > so } else if dau == '<' { gia_tri < so } else { gia_tri == so }
}

fn danh_gia(cay: &CayAst, i: usize, hang: &Vec<Truong>) -> bool {
    match &cay.nut[i] {
        NoAst::SoSanh(ten, dau, so) => danh_gia_so_sanh(hang, ten, *dau, *so),
        NoAst::VaNut(trai, phai) => danh_gia(cay, *trai, hang) && danh_gia(cay, *phai, hang),
        NoAst::HoacNut(trai, phai) => danh_gia(cay, *trai, hang) || danh_gia(cay, *phai, hang),
    }
}

fn buoc_quet(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize) -> Option<Vec<Truong> > {
    if *vi_tri >= bang.len() { return None; }
    let hang = &bang[*vi_tri];
    let mut sao_chep: Vec<Truong> = Vec::new();
    let mut i = 0;
    while i < hang.len() {
        sao_chep.push(Truong { ten: hang[i].ten.clone(), gia_tri: hang[i].gia_tri });
        i = 1 + i;
    };
    *vi_tri = 1 + *vi_tri;
    Some(sao_chep)
}

fn hang_qua_duoc(cay: &CayAst, chi_so_dk: &usize, hang: &Vec<Truong>, co_loc: &bool) -> bool {
    if !*co_loc { true } else { danh_gia(cay, *chi_so_dk, hang) }
}

fn tiep_loc(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize, cay: &CayAst, chi_so_dk: &usize, co_loc: &bool) -> Option<Vec<Truong> > {
    loop {
        let r = buoc_quet(bang, vi_tri);
        match &r {
            None => return None,
            Some(hang) => { if hang_qua_duoc(cay, chi_so_dk, hang, co_loc) { return r; } }
        }
    }
}

fn chon_cot(hang: &Vec<Truong>, cot: &Vec<Vec<char> >) -> Vec<Truong> {
    let mut ket_qua: Vec<Truong> = Vec::new();
    let mut i = 0;
    while i < hang.len() {
        let mut j = 0;
        let mut giu = false;
        while j < cot.len() {
            if hang[i].ten == cot[j] { giu = true; }
            j = 1 + j;
        }
        if giu { ket_qua.push(Truong { ten: hang[i].ten.clone(), gia_tri: hang[i].gia_tri }); }
        i = 1 + i;
    }
    ket_qua
}

fn tiep(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize, cay: &CayAst, chi_so_dk: &usize, co_loc: &bool, cot: &Vec<Vec<char> >) -> Option<Vec<Truong> > {
    let r = tiep_loc(bang, vi_tri, cay, chi_so_dk, co_loc);
    match &r {
        None => None,
        Some(hang) => Some(chon_cot(hang, cot)),
    }
}

fn thuc_thi(ke_hoach: &Vec<BuocKeHoach>, cay: &CayAst, bang: &Vec<Vec<Truong> >) {
    let mut co_loc = false;
    let mut chi_so_dk: usize = 0;
    let mut cot: Vec<Vec<char> > = Vec::new();
    let mut i = 0;
    while i < ke_hoach.len() {
        match &ke_hoach[i] {
            BuocKeHoach::Quet(_) => {},
            BuocKeHoach::Loc(idx) => { co_loc = true; chi_so_dk = *idx; },
            BuocKeHoach::ChonCot(danh_sach) => { cot = danh_sach.clone(); },
        }
        i = 1 + i;
    }
    let mut vi_tri: usize = 0;
    loop {
        let r = ___;
        match &r {
            None => break,
            Some(hang) => println!("{}", hang[0].gia_tri),
        }
    }
}

fn main() {
    let s = String::from("SELECT ten FROM nguoi WHERE tuoi > 18");
    let ky_tu: Vec<char> = s.chars().collect();
    let tk = lex(&ky_tu);
    let mut cay = CayAst { nut: Vec::new() };
    let ctv = parse(&mut cay, &tk);
    println!("{}", cay.nut.len());
    let goc = (cay.nut.len() - 1) as usize;
    let ke_hoach = xay_ke_hoach(&ctv, goc);
    println!("{}", ke_hoach.len());
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','e','n'],gia_tri:10}, Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','e','n'],gia_tri:20}, Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
        vec![Truong{ten:vec!['t','e','n'],gia_tri:30}, Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
    ];
    thuc_thi(&ke_hoach, &cay, &bang);
}
```

```rust title=solution
enum Token {
    So(i64), DinhDanh(Vec<char>), Chon, Tu, ODau, Va, Hoac, Phay, Bang, LonHon, NhoHon, KetThuc,
}
enum NoAst { SoSanh(Vec<char>, char, i64), VaNut(usize, usize), HoacNut(usize, usize) }
struct CayAst { nut: Vec<NoAst> }
struct KetQuaDoc { gia_tri: i64, vi_tri: usize }
struct KetQuaChu { ten: Vec<char>, vi_tri: usize }
struct CauTruyVan { truong: Vec<Vec<char> >, bang: Vec<char>, co_where: bool }

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
    while i < ky_tu.len() && ky_tu[i] == ' ' { i += 1; };
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
        let tiep = match &tk[vt] { Token::Va => true, _ => false };
        if !tiep { break; }
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
        let tiep = match &tk[vt] { Token::Hoac => true, _ => false };
        if !tiep { break; }
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
    match &tk[vt] { Token::DinhDanh(ten) => truong.push(ten.clone()), _ => {} }
    vt = 1 + vt;
    while vt < tk.len() {
        let la_phay = match &tk[vt] { Token::Phay => true, _ => false };
        if !la_phay { break; }
        match &tk[1 + vt] { Token::DinhDanh(ten) => truong.push(ten.clone()), _ => {} }
        vt = 2 + vt;
    }
    vt = 1 + vt;
    let bang = match &tk[vt] { Token::DinhDanh(ten) => ten.clone(), _ => Vec::new() };
    vt = 1 + vt;
    let co_where = match &tk[vt] { Token::ODau => true, _ => false };
    if co_where { parse_bieu_thuc(cay, tk, 1 + vt); }
    CauTruyVan { truong: truong, bang: bang, co_where: co_where }
}

struct Truong { ten: Vec<char>, gia_tri: i64 }
enum BuocKeHoach { Quet(Vec<char>), Loc(usize), ChonCot(Vec<Vec<char> >) }

fn xay_ke_hoach(ctv: &CauTruyVan, chi_so_where: usize) -> Vec<BuocKeHoach> {
    let mut ke_hoach: Vec<BuocKeHoach> = Vec::new();
    ke_hoach.push(BuocKeHoach::Quet(ctv.bang.clone()));
    if ctv.co_where { ke_hoach.push(BuocKeHoach::Loc(chi_so_where)); }
    ke_hoach.push(BuocKeHoach::ChonCot(ctv.truong.clone()));
    ke_hoach
}

fn tra_cuu(hang: &Vec<Truong>, ten: &Vec<char>) -> i64 {
    let mut ket_qua = 0;
    let mut i = 0;
    while i < hang.len() {
        if hang[i].ten == *ten { ket_qua = hang[i].gia_tri; }
        i = 1 + i;
    }
    ket_qua
}

fn danh_gia_so_sanh(hang: &Vec<Truong>, ten: &Vec<char>, dau: char, so: i64) -> bool {
    let gia_tri = tra_cuu(hang, ten);
    if dau == '>' { gia_tri > so } else if dau == '<' { gia_tri < so } else { gia_tri == so }
}

fn danh_gia(cay: &CayAst, i: usize, hang: &Vec<Truong>) -> bool {
    match &cay.nut[i] {
        NoAst::SoSanh(ten, dau, so) => danh_gia_so_sanh(hang, ten, *dau, *so),
        NoAst::VaNut(trai, phai) => danh_gia(cay, *trai, hang) && danh_gia(cay, *phai, hang),
        NoAst::HoacNut(trai, phai) => danh_gia(cay, *trai, hang) || danh_gia(cay, *phai, hang),
    }
}

fn buoc_quet(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize) -> Option<Vec<Truong> > {
    if *vi_tri >= bang.len() { return None; }
    let hang = &bang[*vi_tri];
    let mut sao_chep: Vec<Truong> = Vec::new();
    let mut i = 0;
    while i < hang.len() {
        sao_chep.push(Truong { ten: hang[i].ten.clone(), gia_tri: hang[i].gia_tri });
        i = 1 + i;
    };
    *vi_tri = 1 + *vi_tri;
    Some(sao_chep)
}

fn hang_qua_duoc(cay: &CayAst, chi_so_dk: &usize, hang: &Vec<Truong>, co_loc: &bool) -> bool {
    if !*co_loc { true } else { danh_gia(cay, *chi_so_dk, hang) }
}

fn tiep_loc(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize, cay: &CayAst, chi_so_dk: &usize, co_loc: &bool) -> Option<Vec<Truong> > {
    loop {
        let r = buoc_quet(bang, vi_tri);
        match &r {
            None => return None,
            Some(hang) => { if hang_qua_duoc(cay, chi_so_dk, hang, co_loc) { return r; } }
        }
    }
}

fn chon_cot(hang: &Vec<Truong>, cot: &Vec<Vec<char> >) -> Vec<Truong> {
    let mut ket_qua: Vec<Truong> = Vec::new();
    let mut i = 0;
    while i < hang.len() {
        let mut j = 0;
        let mut giu = false;
        while j < cot.len() {
            if hang[i].ten == cot[j] { giu = true; }
            j = 1 + j;
        }
        if giu { ket_qua.push(Truong { ten: hang[i].ten.clone(), gia_tri: hang[i].gia_tri }); }
        i = 1 + i;
    }
    ket_qua
}

fn tiep(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize, cay: &CayAst, chi_so_dk: &usize, co_loc: &bool, cot: &Vec<Vec<char> >) -> Option<Vec<Truong> > {
    let r = tiep_loc(bang, vi_tri, cay, chi_so_dk, co_loc);
    match &r {
        None => None,
        Some(hang) => Some(chon_cot(hang, cot)),
    }
}

fn thuc_thi(ke_hoach: &Vec<BuocKeHoach>, cay: &CayAst, bang: &Vec<Vec<Truong> >) {
    let mut co_loc = false;
    let mut chi_so_dk: usize = 0;
    let mut cot: Vec<Vec<char> > = Vec::new();
    let mut i = 0;
    while i < ke_hoach.len() {
        match &ke_hoach[i] {
            BuocKeHoach::Quet(_) => {},
            BuocKeHoach::Loc(idx) => { co_loc = true; chi_so_dk = *idx; },
            BuocKeHoach::ChonCot(danh_sach) => { cot = danh_sach.clone(); },
        }
        i = 1 + i;
    }
    let mut vi_tri: usize = 0;
    loop {
        let r = tiep(bang, &mut vi_tri, cay, &chi_so_dk, &co_loc, &cot);
        match &r {
            None => break,
            Some(hang) => println!("{}", hang[0].gia_tri),
        }
    }
}

fn main() {
    let s = String::from("SELECT ten FROM nguoi WHERE tuoi > 18");
    let ky_tu: Vec<char> = s.chars().collect();
    let tk = lex(&ky_tu);
    let mut cay = CayAst { nut: Vec::new() };
    let ctv = parse(&mut cay, &tk);
    println!("{}", cay.nut.len());
    let goc = (cay.nut.len() - 1) as usize;
    let ke_hoach = xay_ke_hoach(&ctv, goc);
    println!("{}", ke_hoach.len());
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','e','n'],gia_tri:10}, Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','e','n'],gia_tri:20}, Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
        vec![Truong{ten:vec!['t','e','n'],gia_tri:30}, Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
    ];
    thuc_thi(&ke_hoach, &cay, &bang);
}
```

```rust title=test
fn main() {
    let s = String::from("SELECT ten FROM nguoi WHERE tuoi > 18");
    let ky_tu: Vec<char> = s.chars().collect();
    let tk = lex(&ky_tu);
    let mut cay = CayAst { nut: Vec::new() };
    let ctv = parse(&mut cay, &tk);
    assert_eq!(cay.nut.len(), 1, "WHERE co dung 1 dieu kien so sanh");
    let goc = (cay.nut.len() - 1) as usize;
    let ke_hoach = xay_ke_hoach(&ctv, goc);
    assert_eq!(ke_hoach.len(), 3, "co WHERE -- ke hoach 3 buoc");

    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','e','n'],gia_tri:10}, Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','e','n'],gia_tri:20}, Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
        vec![Truong{ten:vec!['t','e','n'],gia_tri:30}, Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
    ];
    thuc_thi(&ke_hoach, &cay, &bang);
    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Trong loop cua thuc_thi, goi tiep voi bang, &mut vi_tri, cay, &chi_so_dk, &co_loc, &cot -- mot dong."
- kind: strategy
  body: "tiep(bang, &mut vi_tri, cay, &chi_so_dk, &co_loc, &cot)"
- kind: one-line
  body: "let r = tiep(bang, &mut vi_tri, cay, &chi_so_dk, &co_loc, &cot);"
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "20"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Từ chuỗi ký tự tới kết quả cuối cùng — TRỌN một pipeline. Quest
"Bộ não của truy vấn" khép LẠI — R6-2 đi được thêm một quest nữa.
::::

::::reflect{#nghi-lai}
q08 xây TRỌN "bộ NÃO" chạy phía SAU `CayAst`/`CauTruyVan` (q07):
`BuocKeHoach`/`KeHoach` (bài 5-6) biến câu truy vấn ĐÃ hiểu thành
một danh SÁCH hành động THEO đúng thứ tự (`Quet → Loc → ChonCot` —
KHÔNG phải tuỳ Ý, bài 7 cho thấy đảo thứ TỰ gây sai KẾT quả, không
chỉ chậm hơn); mô hình Volcano (bài 8) rồi `tiep` (bài 9-11) THI
HÀNH kế hoạch đó, TỪNG hàng một, không tính TRƯỚC toàn bộ; `thuc_thi`
(bài NÀY) khép vòng bằng cách ĐỌC `KeHoach` để biết GỌI `tiep` với
tham số NÀO.

Ba ràng buộc kiến trúc xuyên SUỐT q08 — `struct Truong` thay tuple
(annotation kiểu tuple luôn bị từ chối SAI, `docs/decisions/R4-T40b
-su-that-do-duoc.md`), tham chiếu (`&bool`/`&usize`, KHÔNG phải giá
trị trực tiếp) khi một tham SỐ bị dùng lại qua NHIỀU vòng lặp, VÀ
`match &r` (không phải `match r`) trên một `Option` được gán LẠI
trong `loop` — đều LÀ hệ quả trực tiếp của `byte-rust` còn LÀ một
tập con hẹp, giống hệt những ràng buộc đã thấy Ở q07.

`CayAst`/`Token`/`CauTruyVan` (q07, PHÂN tích câu chữ) VÀ `KeHoach`/
`tiep` (q08, kế HOẠCH + thi hành) CỘNG lại LÀ một "bộ não truy vấn"
mini HOÀN chỉnh — đọc chuỗi, hiểu Ý, RỒI chạy thật.
::::

::::checkpoint{mastery=0.85}
::::
