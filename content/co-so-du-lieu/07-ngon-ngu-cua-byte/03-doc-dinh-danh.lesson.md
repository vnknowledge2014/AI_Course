---
id: co-so-du-lieu.ngon-ngu-cua-byte.doc-dinh-danh
title: "Đọc định danh: tên trường, tên bảng"
summary: "doc_dinh_danh(ky_tu: &Vec<char>, bat_dau: usize) gom mọi ký tự chữ/gạch dưới liên tiếp vào một Vec<char> (KHÔNG String — không .push được), trả về KetQuaChu{ten, vi_tri}. la_chu(c) so sánh trực tiếp thay vì is_alphabetic. Không khớp ký tự nào ngay từ đầu -> trả về Vec rỗng, vi_tri không đổi, không lỗi."
locale: vi
track: co-so-du-lieu
module: ngon-ngu-cua-byte
order: 3
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 12
teaches: [db.lexer-identifier]
requires: [db.lexer-whitespace]
concepts: [db.lexer-identifier]
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
Số VÀ khoảng trắng đọc được rồi. Nhưng `SELECT tuoi FROM nguoi` có
`tuoi`, `nguoi` — chữ cái nối tiếp nhau, không phải số. Đọc kiểu
token NÀY thế nào?
::::

::::explain{#doc-dinh-danh}
`doc_dinh_danh` gom mọi ký tự chữ/gạch dưới LIÊN tiếp vào một
`Vec<char>` — KHÔNG `String` (không `.push` được), y hệt lý do đã
gặp Ở `doc_so`:

```rust title=readonly
struct KetQuaChu {
    ten: Vec<char>,
    vi_tri: usize,
}

fn la_chu(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
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

fn main() {
    let s = String::from("tuoi");
    let ky_tu: Vec<char> = s.chars().collect();
    let kq = doc_dinh_danh(&ky_tu, 0);
    println!("{:?} {}", kq.ten, kq.vi_tri);
}
```

```text title=readonly
['t', 'u', 'o', 'i'] 4
```

Cấu trúc GIỐNG hệt `doc_so`: một vòng `while` gom ký tự KHỚP điều
kiện, dừng ngay khi gặp ký tự KHÔNG khớp. Khác Ở CHỖ tích luỹ — thay
vì cộng dồn một SỐ, `ten.push(...)` đẩy TỪNG ký tự vào cuối `Vec`
(có hỗ trợ `.push`, khác `String`).
::::

::::example{#dung-o-dau-phay}
Định danh dừng đúng TẠI ký tự đầu tiên KHÔNG phải chữ/gạch dưới —
kể cả dấu phẩy:

```rust title=readonly
let s2 = String::from("ten_day_du,ten2");
let ky_tu2: Vec<char> = s2.chars().collect();
let kq2 = doc_dinh_danh(&ky_tu2, 0);
println!("{:?} {}", kq2.ten, kq2.vi_tri);
```

```text title=readonly
['t', 'e', 'n', '_', 'd', 'a', 'y', '_', 'd', 'u'] 10
```

`la_chu(',')` LÀ `false` — vòng lặp dừng NGAY tại `i=10`, đúng vị
trí dấu phẩy. Chú Ý: `ten_day_du` CÓ gạch dưới Ở giữa (`la_chu` cho
phép), nhưng `ten2` (nếu đọc TIẾP từ vị trí `11`) sẽ dừng TRƯỚC chữ
số `'2'` — track NÀY giữ định danh CHỈ gồm chữ/gạch dưới, không số,
để một hàm SO SÁNH duy nhất dùng được cho CẢ ký tự đầu lẫn ký tự
sau (đơn giản hơn is_alphabetic/is_alphanumeric của Rust thật).
::::

::::predict{#doan-bat-dau-bang-so commitOnce}
Gọi `doc_dinh_danh` trên một chuỗi bắt đầu bằng CHỮ SỐ:

```rust
let s3 = String::from("123abc");
let ky_tu3: Vec<char> = s3.chars().collect();
let kq3 = doc_dinh_danh(&ky_tu3, 0);
println!("{:?} {}", kq3.ten, kq3.vi_tri);
```

Dòng cuối in ra gì?

:::opt{correct}
`[] 0`
:::

:::opt
Máy báo lỗi — vì một định danh KHÔNG được phép bắt đầu bằng chữ số
::why
Gần đúng ở việc bạn nhớ ĐÚNG một quy tắc THẬT của định danh (kể cả
trong Rust thật, tên biến không được bắt đầu bằng chữ số) — một
hiểu biết chính XÁC về NGÔN ngữ.

Chỗ lệch: `doc_dinh_danh` KHÔNG hề kiểm tra "đây có phải một định
danh HỢP lệ" rồi từ chối — nó chỉ ĐƠN thuần gom ký tự khớp điều
kiện. `la_chu('1')` LÀ `false` NGAY từ vòng lặp ĐẦU tiên, vòng lặp
KHÔNG chạy lần nào — `ten` giữ nguyên rỗng (`Vec::new()` chưa từng
được thêm gì), `vi_tri` giữ nguyên `bat_dau` (LÀ `0`). Không `panic`
nào cả — việc "123abc không phải định danh hợp lệ" LÀ trách nhiệm
của phần gọi hàm NÀY (lexer chính, bài SAU), không phải của chính
`doc_dinh_danh`.
::
:::

:::opt
`['1', '2', '3'] 3` — vì hàm PHẢI đọc được MỘT thứ gì đó, chữ số
cũng LÀ ký tự hợp lệ để bắt đầu một token
::why
Gần đúng ở việc bạn nghĩ TỚI "hàm nên LUÔN trả về một kết quả có
ích" — một trực giác thiết kế API hợp lý nói chung.

Chỗ lệch: `doc_dinh_danh` chỉ gom ký tự thoả `la_chu`, VÀ `la_chu`
kiểm TRA đúng chữ cái/gạch dưới — chữ SỐ không nằm trong tập đó.
Đọc `'1'`, `'2'`, `'3'` như một định danh sẽ LÀ hành vi của MỘT hàm
khác (`doc_so`, bài trước) — hai hàm CHUYÊN biệt, không hàm nào tự
ý "đọc thay" phần việc của hàm kia.
::
:::
::::

::::code{#viet_doc_dinh_danh}
Hoàn thiện `doc_dinh_danh` — đẩy từng ký tự khớp vào `ten`.

```rust title=starter
struct KetQuaChu {
    ten: Vec<char>,
    vi_tri: usize,
}

fn la_chu(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn doc_dinh_danh(ky_tu: &Vec<char>, bat_dau: usize) -> KetQuaChu {
    let mut ten: Vec<char> = Vec::new();
    let mut i = bat_dau;
    while i < ky_tu.len() && la_chu(ky_tu[i]) {
        ___
        i += 1;
    };
    KetQuaChu { ten: ten, vi_tri: i }
}

fn main() {
    let s = String::from("tuoi");
    let ky_tu: Vec<char> = s.chars().collect();
    let kq = doc_dinh_danh(&ky_tu, 0);
    println!("{:?} {}", kq.ten, kq.vi_tri);
}
```

```rust title=solution
struct KetQuaChu {
    ten: Vec<char>,
    vi_tri: usize,
}

fn la_chu(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
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

fn main() {
    let s = String::from("tuoi");
    let ky_tu: Vec<char> = s.chars().collect();
    let kq = doc_dinh_danh(&ky_tu, 0);
    println!("{:?} {}", kq.ten, kq.vi_tri);
}
```

```rust title=test
fn main() {
    let s1 = String::from("tuoi");
    let k1: Vec<char> = s1.chars().collect();
    let r1 = doc_dinh_danh(&k1, 0);
    println!("{:?} {}", r1.ten, r1.vi_tri);
    let mau1: Vec<char> = vec!['t', 'u', 'o', 'i'];
    assert_eq!(r1.ten, mau1, "doc_dinh_danh tren 'tuoi' phai ra dung 4 ky tu");
    assert_eq!(r1.vi_tri, 4, "vi_tri phai la 4");

    let s2 = String::from("ten_day_du,ten2");
    let k2: Vec<char> = s2.chars().collect();
    let r2 = doc_dinh_danh(&k2, 0);
    assert_eq!(r2.vi_tri, 10, "phai dung truoc dau phay");

    let s3 = String::from("123abc");
    let k3: Vec<char> = s3.chars().collect();
    let r3 = doc_dinh_danh(&k3, 0);
    let rong: Vec<char> = Vec::new();
    assert_eq!(r3.ten, rong, "bat dau bang chu so -- khong doc duoc gi");
    assert_eq!(r3.vi_tri, 0, "vi_tri khong doi khi khong doc duoc gi");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Trong than vong while, day ky_tu[i] vao Vec ten bang .push -- mot dong."
- kind: strategy
  body: "ten.push(ky_tu[i]);"
- kind: one-line
  body: "ten.push(ky_tu[i]);"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "4"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Định danh đọc được rồi. Nhưng `SELECT` VÀ `tuoi` trông GIỐNG hệt
nhau lúc quét ký tự — làm sao lexer biết CÁI nào là từ khoá?
::::

::::reflect{#nghi-lai}
`doc_dinh_danh` dùng LẠI đúng khuôn của `doc_so` (bài 1): một vòng
lặp gom ký tự khớp điều kiện, dừng khi gặp ký tự lạ — chỉ khác Ở
kiểu tích luỹ (`Vec<char>` thay vì số). Không khớp ký tự NÀO ngay
từ đầu KHÔNG phải lỗi — chỉ đơn giản LÀ "chưa đọc được gì", việc xử
lý trường hợp đó LÀ của hàm gọi. Vấn đề LÀ: `SELECT`, `FROM`,
`WHERE` VÀ `tuoi`, `nguoi` đi qua ĐÚNG cùng một hàm `doc_dinh_danh`
— làm sao phân biệt được từ khoá VỚI tên trường bình thường?
::::

::::checkpoint{mastery=0.8}
::::
