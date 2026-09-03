---
id: co-so-du-lieu.ngon-ngu-cua-byte.tu-khoa-vs-dinh-danh
title: "Từ khoá và định danh: cùng hình dạng, khác ý nghĩa"
summary: "phan_loai(ten: &Vec<char>) so sánh Vec<char> vừa đọc được với các mẫu từ khoá đã biết (so sánh == giữa hai Vec<char>, đúng ngữ nghĩa so từng phần tử) — khớp SELECT/FROM/WHERE/AND/OR thì trả về token từ khoá riêng, không khớp gì thì trả về Token::DinhDanh(ten.clone()). SELECT và tuoi đi qua CÙNG một hàm doc_dinh_danh, chỉ khác ở bước phân loại SAU đó."
locale: vi
track: co-so-du-lieu
module: ngon-ngu-cua-byte
order: 4
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 13
teaches: [db.lexer-keyword]
requires: [db.lexer-identifier]
concepts: [db.lexer-keyword]
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
`doc_dinh_danh` (bài TRƯỚC) đọc `SELECT` VÀ `tuoi` giống HỆT nhau —
cả hai đều chỉ LÀ chữ cái nối tiếp. Làm sao lexer biết cái NÀO là
lệnh, cái nào LÀ tên trường?
::::

::::explain{#phan-loai}
`phan_loai` SO SÁNH `Vec<char>` vừa đọc được VỚI các mẫu từ khoá đã
biết — so sánh `==` giữa hai `Vec<char>` (đã xác nhận đúng ngữ nghĩa
so TỪNG phần tử, không phải so địa chỉ):

```rust title=readonly
enum Token {
    So(i64),
    DinhDanh(Vec<char>),
    Chon,
    Tu,
    ODau,
    Va,
    Hoac,
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

fn main() {
    let ten1: Vec<char> = vec!['S','E','L','E','C','T'];
    let t1 = phan_loai(&ten1);
    match t1 {
        Token::Chon => println!("CHON"),
        _ => println!("khac"),
    }
}
```

```text title=readonly
CHON
```

`ten` (kết quả TỪ `doc_dinh_danh`, bài trước) được so KHỚP lần lượt
VỚI từng mẫu từ khoá. Khớp `chon` (đúng SÁU ký tự `S,E,L,E,C,T`) —
trả VỀ `Token::Chon` NGAY, không đi tiếp các dòng SAU. `phan_loai`
nhận `ten: &Vec<char>` (tham CHIẾU) — dùng được Ở NHIỀU dòng `if`
liên tiếp mà không hề bị "chuyển quyền sở hữu" mất, vì tham chiếu
bất biến LUÔN dùng lại được.
::::

::::example{#khong-khop-la-dinh-danh}
Không khớp mẫu từ khoá NÀO — trả về `Token::DinhDanh`, giữ NGUYÊN
text gốc (qua `.clone()`, vì `ten` chỉ LÀ tham chiếu, không lấy hẳn
được):

```rust title=readonly
let ten2: Vec<char> = vec!['t','u','o','i'];
let t2 = phan_loai(&ten2);
match t2 {
    Token::DinhDanh(v) => println!("DINHDANH {:?}", v),
    _ => println!("khac"),
}
```

```text title=readonly
DINHDANH ['t', 'u', 'o', 'i']
```

`['t','u','o','i']` KHÔNG khớp bất kỳ mẫu nào trong NĂM từ khoá —
năm dòng `if` đều SAI, hàm chạy tới dòng CUỐI: `Token::DinhDanh(ten.
clone())`. `tuoi` được XỬ lý y hệt `SELECT` cho TỚI đúng bước so
khớp NÀY — sự khác biệt CHỈ nằm Ở kết quả phân loại, không phải Ở
cách ĐỌC ký tự.
::::

::::predict{#doan-vietnam-la-dinh-danh commitOnce}
`phan_loai` nhận một tên trường TRÙNG với một phần của từ khoá,
nhưng KHÔNG khớp trọn vẹn:

```rust
let ten3: Vec<char> = vec!['S','E','L','E','C','T','S'];
let t3 = phan_loai(&ten3);
match t3 {
    Token::Chon => println!("CHON"),
    Token::DinhDanh(v) => println!("DINHDANH {:?}", v),
    _ => println!("khac"),
}
```

`ten3` LÀ `"SELECTS"` — bảy ký tự, `SELECT` cộng thêm một `S`. Dòng
cuối in ra gì?

:::opt{correct}
`DINHDANH ['S', 'E', 'L', 'E', 'C', 'T', 'S']`
:::

:::opt
`CHON` — vì `ten3` BẮT ĐẦU bằng đúng sáu ký tự của `SELECT`, phần
đầu ĐÃ khớp là đủ
::why
Gần đúng ở việc bạn để Ý ĐÚNG sáu ký tự ĐẦU của `ten3` TRÙNG khớp
hoàn toàn VỚI `chon` — một quan sát chính XÁC về phần chung.

Chỗ lệch: so sánh `*ten == chon` LÀ so sánh HAI `Vec` bằng nhau
TOÀN bộ — CẢ độ dài lẫn TỪNG phần tử phải khớp. `ten3` có BẢY phần
tử, `chon` chỉ có SÁU — hai `Vec` khác ĐỘ dài thì KHÔNG THỂ bằng
nhau, bất kể phần ĐẦU trùng khớp ra sao. `SELECTS` LÀ một định danh
bình thường (có thể LÀ tên một bảng thật), không phải từ khoá.
::
:::

:::opt
Máy báo lỗi — vì `SELECTS` vừa GIỐNG một từ khoá vừa KHÔNG khớp
hẳn, đây LÀ một trường hợp không xác định được
::why
Gần đúng ở việc bạn cảm thấy `SELECTS` "MƠ hồ" — một trực giác dễ
hiểu khi nhìn CHỮ.

Chỗ lệch: với MÁY, không hề có gì mơ hồ — `*ten == chon` chỉ LÀ một
phép so sánh boolean, kết quả LUÔN dứt khoát `true` hay `false`,
không có trạng thái "không rõ". Ở đây kết quả LÀ `false` (khác độ
dài), nên hàm ĐƠN giản rơi xuống dòng cuối — không có nhánh nào xử
lý "trường hợp không xác định".
::
:::
::::

::::code{#viet_phan_loai}
Hoàn thiện `phan_loai` — thêm nhánh so khớp CÒN THIẾU cho từ khoá
`AND`.

```rust title=starter
enum Token {
    So(i64),
    DinhDanh(Vec<char>),
    Chon,
    Tu,
    ODau,
    Va,
    Hoac,
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
    ___
    if *ten == hoac { return Token::Hoac; }
    Token::DinhDanh(ten.clone())
}

fn main() {
    let ten: Vec<char> = vec!['A','N','D'];
    let t = phan_loai(&ten);
    match t {
        Token::Va => println!("VA"),
        _ => println!("khac"),
    }
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

fn main() {
    let ten: Vec<char> = vec!['A','N','D'];
    let t = phan_loai(&ten);
    match t {
        Token::Va => println!("VA"),
        _ => println!("khac"),
    }
}
```

```rust title=test
fn main() {
    let ten1: Vec<char> = vec!['A','N','D'];
    match phan_loai(&ten1) {
        Token::Va => println!("VA"),
        _ => panic!("AND phai phan loai thanh Token::Va"),
    }

    let ten2: Vec<char> = vec!['O','R'];
    match phan_loai(&ten2) {
        Token::Hoac => {},
        _ => panic!("OR phai phan loai thanh Token::Hoac"),
    }

    let ten3: Vec<char> = vec!['S','E','L','E','C','T'];
    match phan_loai(&ten3) {
        Token::Chon => {},
        _ => panic!("SELECT phai phan loai thanh Token::Chon"),
    }

    let ten4: Vec<char> = vec!['t','u','o','i'];
    match phan_loai(&ten4) {
        Token::DinhDanh(v) => {
            let mau: Vec<char> = vec!['t','u','o','i'];
            assert_eq!(v, mau, "dinh danh phai giu nguyen text");
        },
        _ => panic!("tuoi phai phan loai thanh Token::DinhDanh"),
    }

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Them mot dong if giong het cac dong tren, so khop *ten voi bien va, tra ve Token::Va."
- kind: strategy
  body: "if *ten == va { return Token::Va; }"
- kind: one-line
  body: "if *ten == va { return Token::Va; }"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "VA"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Từ khoá VÀ định danh giờ phân biệt được. Còn hai loại token cuối
CÙNG: dấu phẩy VÀ các dấu so sánh — đọc chúng KHÁC hẳn đọc chữ/số.
::::

::::reflect{#nghi-lai}
`phan_loai` không hề thay đổi CÁCH đọc ký tự — `doc_dinh_danh` (bài
trước) vẫn LÀ hàm DUY nhất gom chữ cái thành `Vec<char>`. Sự khác
biệt giữa từ khoá VÀ định danh chỉ xuất hiện Ở một bước RIÊNG, SAU
khi đã đọc xong: so khớp text VỚI danh sách mẫu đã biết. Đây LÀ
cách hầu hết lexer thật LÀM — "đọc trước, phân loại sau", không
trộn lẫn hai việc. Còn hai loại ký tự CHƯA đọc được: dấu phẩy VÀ
các dấu so sánh (`,` `=` `>` `<`) — chúng KHÔNG gộp nhiều ký tự như
số/định danh, đọc thế NÀO?
::::

::::checkpoint{mastery=0.8}
::::
