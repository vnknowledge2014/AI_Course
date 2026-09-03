---
id: co-so-du-lieu.ngon-ngu-cua-byte.parse-so-sanh
title: "Parse một phép so sánh"
summary: "parse_so_sanh(cay: &mut CayAst, tk: &Vec<Token>, vi_tri: usize) -> usize đọc ĐÚNG BA token liên tiếp (định danh, dấu so sánh, số), thêm một NoAst::SoSanh vào arena qua cay.nut.push, trả về vị trí token kế tiếp (3 + vi_tri, KHÔNG vi_tri + 3 — vế trái phép + bị coi là move, đổi thứ tự toán hạng để né). Đây là nút lá của cây bieu_thuc."
locale: vi
track: co-so-du-lieu
module: ngon-ngu-cua-byte
order: 9
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 14
teaches: [db.parser-so-sanh]
requires: [db.parser-precedence]
concepts: [db.parser-so-sanh]
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
Độ ưu tiên đã có SỐ đo được (bài trước). Nhưng TRƯỚC khi ghép AND/
OR, cần parse được MỘT phép so sánh đơn — `tuoi > 18` — thành MỘT
nút.
::::

::::explain{#parse-so-sanh}
`parse_so_sanh` đọc ĐÚNG ba token liên tiếp (định danh, dấu so
sánh, số), thêm MỘT `NoAst::SoSanh` vào arena qua `cay: &mut
CayAst`:

```rust title=readonly
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

fn main() {
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    let tk: Vec<Token> = vec![Token::DinhDanh(tuoi), Token::LonHon, Token::So(18), Token::KetThuc];
    let mut cay = CayAst { nut: Vec::new() };
    let vt = parse_so_sanh(&mut cay, &tk, 0);
    println!("{}", vt);
}
```

```text title=readonly
3
```

Ba token (`DinhDanh("tuoi")`, `LonHon`, `So(18)`) đọc THÀNH một
`NoAst::SoSanh(['t','u','o','i'], '>', 18)`, đẩy VÀO `cay.nut`.
Trả VỀ `3 + vi_tri` (KHÔNG `vi_tri + 3`) — viết `vi_tri` Ở vế TRÁI
phép `+` khiến `vi_tri` bị chấm nhầm LÀ "đã chuyển đi" nếu dùng
lại sau đó; đặt hằng số Ở vế trái LÀ quy ước track NÀY dùng xuyên
suốt cho phép cộng chỉ số.
::::

::::example{#doc-lai-nut-vua-them}
Cây sau khi gọi CÓ đúng một nút — đọc LẠI bằng `match`:

```rust title=readonly
match &cay.nut[0] {
    NoAst::SoSanh(truong, dau, so) => println!("{:?} {} {}", truong, dau, so),
    _ => println!("?"),
}
```

```text title=readonly
['t', 'u', 'o', 'i'] > 18
```

`cay.nut.len()` LÀ `1` NGAY sau lệnh gọi — `parse_so_sanh` KHÔNG
trả về chỉ số nút vừa thêm (chỉ trả VỀ vị trí token), vì chỉ số ĐÓ
LUÔN LÀ `cay.nut.len() - 1` NGAY sau khi push — người gọi tự tính
được, không cần một giá trị trả VỀ thứ hai (tránh luôn vấn đề trả
về tuple, đã gặp Ở bài 1).
::::

::::predict{#doan-toan-tu-bang commitOnce}
Gọi `parse_so_sanh` trên một dãy token KHÁC — dùng dấu `=` thay VÌ
`>`:

```rust
let ten_bang: Vec<char> = vec!['t','e','n'];
let tk2: Vec<Token> = vec![Token::DinhDanh(ten_bang), Token::Bang, Token::So(0)];
let mut cay2 = CayAst { nut: Vec::new() };
let vt2 = parse_so_sanh(&mut cay2, &tk2, 0);
match &cay2.nut[0] {
    NoAst::SoSanh(truong, dau, so) => println!("{:?} {} {}", truong, dau, so),
    _ => println!("?"),
}
```

Dòng cuối in ra gì?

:::opt{correct}
`['t', 'e', 'n'] = 0`
:::

:::opt
Máy báo lỗi — vì `Token::So(0)` mang giá trị `0`, một trường hợp
"rỗng" không hợp lệ cho vế PHẢI một phép so sánh
::why
Gần đúng ở việc bạn cảnh giác VỚI giá trị `0` như một trường hợp
BIÊN đáng ngờ — một phản xạ hợp lý khi đọc code.

Chỗ lệch: `Token::So(0)` hoàn toàn LÀ một số nguyên BÌNH thường —
`match &tk[2 + vi_tri] { Token::So(n) => *n, ... }` chỉ đơn giản
LẤY giá trị `n` ra, không hề phân biệt `0` VỚI bất kỳ số nào khác.
Không có `panic`/`raise` nào cho "giá trị bằng không" — `SoSanh`
được tạo BÌNH thường VỚI `so = 0`.
::
:::
::::

::::code{#viet_parse_so_sanh}
Hoàn thiện `parse_so_sanh` — lấy giá trị số nguyên ra khỏi token
`So`.

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
        ___
        _ => 0,
    };
    cay.nut.push(NoAst::SoSanh(truong, dau, so));
    3 + vi_tri
}

fn main() {
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    let tk: Vec<Token> = vec![Token::DinhDanh(tuoi), Token::LonHon, Token::So(18), Token::KetThuc];
    let mut cay = CayAst { nut: Vec::new() };
    let vt = parse_so_sanh(&mut cay, &tk, 0);
    println!("{}", vt);
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

fn main() {
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    let tk: Vec<Token> = vec![Token::DinhDanh(tuoi), Token::LonHon, Token::So(18), Token::KetThuc];
    let mut cay = CayAst { nut: Vec::new() };
    let vt = parse_so_sanh(&mut cay, &tk, 0);
    println!("{}", vt);
}
```

```rust title=test
fn main() {
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    let tk: Vec<Token> = vec![Token::DinhDanh(tuoi), Token::LonHon, Token::So(18), Token::KetThuc];
    let mut cay = CayAst { nut: Vec::new() };
    let vt = parse_so_sanh(&mut cay, &tk, 0);
    println!("{}", vt);
    assert_eq!(vt, 3, "phai tieu thu dung 3 token");
    assert_eq!(cay.nut.len(), 1, "phai them dung 1 nut vao cay");
    match &cay.nut[0] {
        NoAst::SoSanh(truong, dau, so) => {
            let mau: Vec<char> = vec!['t','u','o','i'];
            assert_eq!(truong, &mau, "truong phai la tuoi");
            assert_eq!(*dau, '>', "dau phai la >");
            assert_eq!(*so, 18, "so phai la 18");
        },
        _ => panic!("nut phai la SoSanh"),
    }

    let dien: Vec<char> = vec!['d','i','e','m'];
    let tk2: Vec<Token> = vec![Token::DinhDanh(dien), Token::NhoHon, Token::So(100)];
    let mut cay2 = CayAst { nut: Vec::new() };
    let vt2 = parse_so_sanh(&mut cay2, &tk2, 0);
    assert_eq!(vt2, 3, "phai tieu thu dung 3 token, bat ke gia tri");
    match &cay2.nut[0] {
        NoAst::SoSanh(_, dau, so) => {
            assert_eq!(*dau, '<', "dau phai la <");
            assert_eq!(*so, 100, "so phai la 100");
        },
        _ => panic!("nut phai la SoSanh"),
    }

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Nhanh Token::So(n) cua match phai lay gia tri n ra, dung dau * de giai tham chieu -- mot dong."
- kind: strategy
  body: "Token::So(n) => *n,"
- kind: one-line
  body: "Token::So(n) => *n,"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một phép so sánh parse được rồi — nút LÁ của cây. Ghép nhiều nút lá
lại bằng `AND`/`OR`, đúng độ ưu tiên đã tính, trông ra sao?
::::

::::reflect{#nghi-lai}
`parse_so_sanh` LÀ đơn vị NHỎ nhất của `bieu_thuc` — đọc đúng ba
token, tạo đúng MỘT nút LÁ. Nó KHÔNG biết gì về AND/OR, KHÔNG biết
gì về độ ưu tiên (bài trước) — CHỈ làm ĐÚNG một việc. Đây LÀ nguyên
tắc thiết kế parser tốt: mỗi hàm parse MỘT tầng ngữ pháp, GHÉP các
hàm nhỏ lại Ở tầng CAO hơn. Tầng cao hơn TIẾP theo: dùng độ ưu tiên
(bài 8) để QUYẾT định khi nào gộp hai `so_sanh` bằng `AND`/`OR`.
::::

::::checkpoint{mastery=0.8}
::::
