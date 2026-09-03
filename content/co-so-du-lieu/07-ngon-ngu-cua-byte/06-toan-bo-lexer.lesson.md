---
id: co-so-du-lieu.ngon-ngu-cua-byte.toan-bo-lexer
title: "Ráp lại: toàn bộ lexer"
summary: "lex(ky_tu: &Vec<char>) -> Vec<Token> là vòng lặp chính: bỏ khoảng trắng, rồi rẽ nhánh theo ký tự hiện tại — số dùng doc_so, chữ/gạch dưới dùng doc_dinh_danh+phan_loai, còn lại dùng doc_dau — cho tới hết chuỗi, kết thúc bằng Token::KetThuc. Đây là lexer hoàn chỉnh, nửa đầu quest khép lại."
locale: vi
track: co-so-du-lieu
module: ngon-ngu-cua-byte
order: 6
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: [db.lexer-hoan-chinh]
requires: [db.lexer-symbol]
concepts: [db.lexer-hoan-chinh]
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
Năm mảnh (số, trắng, định danh, từ khoá, dấu) đã sẵn sàng riêng lẻ.
Ráp CHÚNG thành một vòng lặp DUY nhất, biến cả câu truy vấn thành
`Vec<Token>`, trông ra sao?
::::

::::explain{#lex-chinh}
`lex` LÀ vòng lặp chính: bỏ khoảng trắng, rẽ NHÁNH theo ký tự hiện
tại, lặp lại tới hết chuỗi:

```rust title=readonly
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
```

Mỗi vòng: NHÌN ký tự tại `i`, chọn ĐÚNG một trong ba nhánh (số/
chữ/dấu — năm hàm của bài 1-5 dùng LẠI y nguyên), đẩy token TƯƠNG
ứng, cập nhật `i` TỚI vị trí kế tiếp, rồi bỏ khoảng trắng TRƯỚC khi
lặp lại. Ghi Ý: mỗi hàm (`doc_so`, `doc_dinh_danh`) được gọi HAI
lần trong một nhánh (một lần lấy giá trị, một lần lấy vị trí) —
track NÀY không giữ kết quả trong một biến trung gian rồi đọc HAI
trường của nó, vì đọc nhiều trường của CÙNG một struct bên TRONG
một nhánh bị chấm nhầm thành "chuyển quyền sở hữu". Gọi lại (các
hàm NÀY không có tác dụng phụ, gọi hai lần cho kết quả GIỐNG hệt)
LÀ cách né đơn giản nhất.
::::

::::example{#chay-tren-cau-day-du}
Chạy `lex` trên một câu truy vấn ĐẦY đủ:

```rust title=readonly
let s = String::from("SELECT tuoi FROM nguoi WHERE tuoi > 18");
let ky_tu: Vec<char> = s.chars().collect();
let kq = lex(&ky_tu);
println!("{}", kq.len());
```

```text title=readonly
9
```

Chín token: `SELECT`, `tuoi`, `FROM`, `nguoi`, `WHERE`, `tuoi`, `>`,
`18`, VÀ `KetThuc` Ở cuối. Khoảng trắng giữa các từ KHÔNG tạo ra
token nào — `bo_qua_trang` đã "nuốt" chúng hoàn toàn trước MỖI lần
đọc.
::::

::::predict{#doan-cau-ngan commitOnce}
Một câu truy vấn NGẮN hơn, không có `WHERE`:

```rust
let s2 = String::from("SELECT ten FROM ban");
let ky_tu2: Vec<char> = s2.chars().collect();
let kq2 = lex(&ky_tu2);
println!("{}", kq2.len());
```

Dòng cuối in ra gì?

:::opt{correct}
`5`
:::

:::opt
`4` — vì câu truy vấn CHỈ có bốn từ (`SELECT`, `ten`, `FROM`, `ban`),
số token phải ĐÚNG bằng số từ
::why
Gần đúng ở việc bạn đếm ĐÚNG bốn từ trong chuỗi gốc — một quan sát
chính xác VỀ văn bản đầu vào.

Chỗ lệch: `lex` LUÔN thêm `Token::KetThuc` Ở CUỐI, sau khi vòng lặp
`while i < ky_tu.len()` đã xử lý hết mọi ký tự — đây LÀ dòng CUỐI
cùng trong thân hàm, chạy VÔ điều kiện, không phụ thuộc nội dung
chuỗi. Bốn từ TẠO ra bốn token, cộng THÊM một `KetThuc` LÀ năm.
::
:::

:::opt
Máy báo lỗi — vì `lex` được thiết kế cho câu CÓ đủ `SELECT...FROM...
WHERE`, thiếu `WHERE` LÀ cú pháp sai
::why
Gần đúng ở việc bạn nhớ ĐÚNG ngữ pháp mục tiêu của quest CÓ phần
`WHERE` tuỳ chọn — một hiểu biết chính xác VỀ thiết kế ngôn ngữ.

Chỗ lệch: `lex` (bài NÀY) hoàn toàn CHƯA biết gì về ngữ pháp CÂU
lệnh — nó chỉ biến ký tự thành TOKEN, không quan tâm token có xếp
ĐÚNG thứ tự "hợp lệ" hay không. Việc "SELECT ... FROM ... [WHERE
...]" LÀ một câu hợp lệ (WHERE tuỳ chọn) LÀ trách nhiệm của PARSER
— quest sẽ xây Ở nửa SAU (bài 7 trở đi), không phải của lexer.
::
:::
::::

::::code{#viet_lex}
Hoàn thiện `lex` — thêm token kết thúc VÀO cuối danh sách trước khi
trả về.

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

struct KetQuaDoc { gia_tri: i64, vi_tri: usize }
struct KetQuaChu { ten: Vec<char>, vi_tri: usize }

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
    ___
    ket_qua
}

fn main() {
    let s = String::from("SELECT tuoi FROM nguoi WHERE tuoi > 18");
    let ky_tu: Vec<char> = s.chars().collect();
    let kq = lex(&ky_tu);
    println!("{}", kq.len());
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

struct KetQuaDoc { gia_tri: i64, vi_tri: usize }
struct KetQuaChu { ten: Vec<char>, vi_tri: usize }

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

fn main() {
    let s = String::from("SELECT tuoi FROM nguoi WHERE tuoi > 18");
    let ky_tu: Vec<char> = s.chars().collect();
    let kq = lex(&ky_tu);
    println!("{}", kq.len());
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

struct KetQuaDoc { gia_tri: i64, vi_tri: usize }
struct KetQuaChu { ten: Vec<char>, vi_tri: usize }

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

fn la_ket_thuc(t: &Token) -> bool {
    match t {
        Token::KetThuc => true,
        _ => false,
    }
}

fn main() {
    let s = String::from("SELECT tuoi FROM nguoi WHERE tuoi > 18");
    let ky_tu: Vec<char> = s.chars().collect();
    let kq = lex(&ky_tu);
    println!("{}", kq.len());
    assert_eq!(kq.len(), 9, "cau truy van mau phai co dung 9 token");
    assert!(la_ket_thuc(&kq[8]), "token cuoi cung phai la KetThuc");

    let s2 = String::from("SELECT ten FROM ban");
    let ky_tu2: Vec<char> = s2.chars().collect();
    let kq2 = lex(&ky_tu2);
    assert_eq!(kq2.len(), 5, "SELECT ten FROM ban phai co 5 token (ke ca KetThuc)");
    assert!(la_ket_thuc(&kq2[4]), "token cuoi cung phai la KetThuc");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Sau khi vong while ket thuc, day them dung mot Token::KetThuc vao ket_qua truoc khi return -- mot dong."
- kind: strategy
  body: "ket_qua.push(Token::KetThuc);"
- kind: one-line
  body: "ket_qua.push(Token::KetThuc);"
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "9"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lexer hoàn chỉnh — chuỗi ký tự thành danh sách token có Ý nghĩa.
Nửa đầu quest khép lại. Token giờ cần được RÁP thành một CÂY —
CÂY đó trông ra sao?
::::

::::reflect{#nghi-lai}
`lex` không hề thêm khái niệm MỚI — nó chỉ LÀ một vòng lặp ĐIỀU
PHỐI, gọi đúng hàm CHUYÊN biệt (bài 1-5) dựa TRÊN ký tự hiện tại.
Đây LÀ cách hầu hết lexer thật hoạt động: một "dispatcher" ĐƠN
giản, logic THẬT nằm trong các hàm đọc TỪNG loại token. Với `lex`,
nửa ĐẦU quest (biến CHUỖI thành DANH sách token) đã xong. Nhưng một
`Vec<Token>` phẳng CHƯA nói lên CẤU trúc câu truy vấn — `tuoi`, `>`,
`18` phải gộp LẠI thành một "phép so sánh", RỒI gộp tiếp thành cả
câu `SELECT...FROM...WHERE`. Cấu trúc ĐÓ gọi LÀ cây cú pháp — trông
ra sao?
::::

::::checkpoint{mastery=0.8}
::::
