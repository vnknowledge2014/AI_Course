---
id: co-so-du-lieu.ngon-ngu-cua-byte.doc-dau
title: "Đọc dấu: phẩy, so sánh"
summary: "doc_dau(ky_tu: &Vec<char>, vi_tri: usize) match TRỰC TIẾP trên MỘT ký tự tại vi_tri, trả về đúng MỘT trong bốn token (Phay, Bang, LonHon, NhoHon) — không gộp nhiều ký tự như số/định danh, không cần vòng lặp. Khác doc_so/doc_dinh_danh ở CHỖ này: mỗi dấu LUÔN chỉ một ký tự."
locale: vi
track: co-so-du-lieu
module: ngon-ngu-cua-byte
order: 5
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 10
teaches: [db.lexer-symbol]
requires: [db.lexer-keyword]
concepts: [db.lexer-symbol]
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
Số, khoảng trắng, định danh VÀ từ khoá đọc được cả rồi. Ngữ pháp
CÒN bốn ký tự đơn: `,` `=` `>` `<`. Đọc chúng có CẦN một vòng lặp
như số/định danh KHÔNG?
::::

::::explain{#doc-dau}
KHÔNG cần vòng lặp — mỗi dấu LUÔN chỉ MỘT ký tự, `match` trực tiếp
LÀ đủ:

```rust title=readonly
enum Token {
    Phay,
    Bang,
    LonHon,
    NhoHon,
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

fn main() {
    let s = String::from(",=><");
    let ky_tu: Vec<char> = s.chars().collect();
    let t0 = doc_dau(&ky_tu, 0);
    match t0 { Token::Phay => println!("PHAY"), _ => println!("?") }
}
```

```text title=readonly
PHAY
```

Khác HẲN `doc_so`/`doc_dinh_danh` (bài 1, 3) — KHÔNG có vòng `while`
gom nhiều ký tự, KHÔNG có "vị trí kết thúc" cần tính. Đọc MỘT ký tự
TẠI `vi_tri`, `match` NGAY ra token TƯƠNG ứng. Nhánh `_` LÀ dự
phòng — trong ngữ pháp track NÀY, `doc_dau` CHỈ được gọi khi lexer
chính (bài SAU) đã biết chắc ký tự hiện tại LÀ một trong bốn dấu.
::::

::::example{#bon-dau-lien-tiep}
Bốn ký tự dấu LIÊN tiếp, đọc TỪNG cái một bằng bốn lệnh gọi RIÊNG
(mỗi lệnh MỘT vị trí):

```rust title=readonly
let t1 = doc_dau(&ky_tu, 1);
let t2 = doc_dau(&ky_tu, 2);
let t3 = doc_dau(&ky_tu, 3);
match t1 { Token::Bang => println!("BANG"), _ => println!("?") }
match t2 { Token::LonHon => println!("LONHON"), _ => println!("?") }
match t3 { Token::NhoHon => println!("NHOHON"), _ => println!("?") }
```

```text title=readonly
BANG
LONHON
NHOHON
```

Mỗi lệnh gọi `doc_dau` ĐỘC lập, chỉ nhìn ĐÚNG một vị trí — không hề
"nhớ" lệnh gọi trước Đó khác GÌ đã đọc. So VỚI `doc_so`/`doc_dinh_
danh`, `doc_dau` LÀ hàm ĐƠN giản nhất trong lexer: không tích luỹ,
không vòng lặp, chỉ tra bảng MỘT lần.
::::

::::predict{#doan-tuoi-lon-hon commitOnce}
Câu truy vấn thật CÓ dấu `>` nằm GIỮA hai định danh, ví dụ
`"tuoi>18"`. Byte gọi `doc_dau` tại đúng vị trí dấu `>` (vị trí
`4`, sau khi `"tuoi"` đã đọc xong Ở bài TRƯỚC):

```rust
let s2 = String::from("tuoi>18");
let ky_tu2: Vec<char> = s2.chars().collect();
let t = doc_dau(&ky_tu2, 4);
match t {
    Token::LonHon => println!("LONHON"),
    Token::Bang => println!("BANG"),
    _ => println!("khac"),
}
```

Dòng cuối in ra gì?

:::opt{correct}
`LONHON`
:::

:::opt
`khac` — vì `doc_dau` không hề biết TRƯỚC đó LÀ `"tuoi"`, một định
danh, nên không xử LÝ đúng ký tự tiếp theo
::why
Gần đúng ở việc bạn nghĩ TỚI "ngữ cảnh" (điều gì đứng TRƯỚC token
hiện tại) như một yếu tố CÓ thể ảnh hưởng — một trực giác hợp lý
cho MỘT số loại phân tích cú pháp phức tạp hơn.

Chỗ lệch: `doc_dau` hoàn toàn KHÔNG quan tâm ngữ cảnh — nó chỉ nhìn
ĐÚNG một ký tự TẠI `vi_tri` được truyền vào. `ky_tu2[4]` LÀ `'>'`
(đếm: `t`,`u`,`o`,`i`,`>` — chỉ số `0` tới `4`), khớp NHÁNH thứ ba
của `match`, trả VỀ `Token::LonHon`. "tuoi" đứng trước KHÔNG hề ảnh
hưởng gì tới kết quả NÀY.
::
:::
::::

::::code{#viet_doc_dau}
Hoàn thiện `doc_dau` — thêm nhánh CÒN THIẾU cho dấu `<`.

```rust title=starter
enum Token {
    Phay,
    Bang,
    LonHon,
    NhoHon,
}

fn doc_dau(ky_tu: &Vec<char>, vi_tri: usize) -> Token {
    match ky_tu[vi_tri] {
        ',' => Token::Phay,
        '=' => Token::Bang,
        '>' => Token::LonHon,
        ___
        _ => Token::Phay,
    }
}

fn main() {
    let s = String::from("<");
    let ky_tu: Vec<char> = s.chars().collect();
    let t = doc_dau(&ky_tu, 0);
    match t { Token::NhoHon => println!("NHOHON"), _ => println!("?") }
}
```

```rust title=solution
enum Token {
    Phay,
    Bang,
    LonHon,
    NhoHon,
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

fn main() {
    let s = String::from("<");
    let ky_tu: Vec<char> = s.chars().collect();
    let t = doc_dau(&ky_tu, 0);
    match t { Token::NhoHon => println!("NHOHON"), _ => println!("?") }
}
```

```rust title=test
fn main() {
    let s = String::from(",=><");
    let k: Vec<char> = s.chars().collect();

    match doc_dau(&k, 0) {
        Token::Phay => println!("PHAY"),
        _ => panic!("vi tri 0 (',') phai la Token::Phay"),
    }
    match doc_dau(&k, 1) {
        Token::Bang => println!("BANG"),
        _ => panic!("vi tri 1 ('=') phai la Token::Bang"),
    }
    match doc_dau(&k, 2) {
        Token::LonHon => println!("LONHON"),
        _ => panic!("vi tri 2 ('>') phai la Token::LonHon"),
    }
    match doc_dau(&k, 3) {
        Token::NhoHon => println!("NHOHON"),
        _ => panic!("vi tri 3 ('<') phai la Token::NhoHon"),
    }
    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Them mot nhanh match cho ky tu '<', tra ve Token::NhoHon -- mot dong, dat truoc nhanh _."
- kind: strategy
  body: "'<' => Token::NhoHon,"
- kind: one-line
  body: "'<' => Token::NhoHon,"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "NHOHON"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Số, trắng, định danh, từ khoá, dấu — NĂM loại token, tất cả đọc
được rồi. Ráp CHÚNG lại thành một lexer HOÀN chỉnh trông ra sao?
::::

::::reflect{#nghi-lai}
`doc_dau` LÀ hàm đơn giản nhất trong năm hàm đã xây — một `match`
trực tiếp trên MỘT ký tự, không tích luỹ, không vòng lặp. So SÁNH
với `doc_so` VÀ `doc_dinh_danh` (đều cần vòng `while` gom NHIỀU ký
tự) cho thấy: không phải MỌI token đều "phức tạp" như nhau — độ khó
đọc một token PHỤ thuộc vào CHÍNH hình dạng của nó trong ngữ pháp.
Năm mảnh rời (số, trắng, định danh, từ khoá, dấu) đã sẵn sàng — ráp
LẠI thành MỘT vòng lặp `lex` duy nhất, biến toàn bộ câu truy vấn
thành một `Vec<Token>`, trông ra sao?
::::

::::checkpoint{mastery=0.8}
::::
