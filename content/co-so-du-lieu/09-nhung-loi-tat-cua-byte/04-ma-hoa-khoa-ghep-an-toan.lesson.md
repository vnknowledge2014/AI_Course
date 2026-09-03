---
id: co-so-du-lieu.nhung-loi-tat-cua-byte.ma-hoa-khoa-ghep-an-toan
title: "Mã hoá khoá ghép an toàn"
summary: "ma_hoa_ghep(a, b) nối a, chèn một ký tự PHÂN CÁCH ('\\0', không xuất hiện trong dữ liệu thật) rồi mới nối b. ('ab','c') và ('a','bc') giờ cho hai khoá KHÁC nhau (['a','b','\\0','c'] và ['a','\\0','b','c']) — đụng độ của bài trước biến mất, với ĐIỀU KIỆN dữ liệu thật không bao giờ chứa ký tự phân cách đó."
locale: vi
track: co-so-du-lieu
module: nhung-loi-tat-cua-byte
order: 4
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 11
teaches: [db.composite-key-encoding]
requires: [db.composite-key-pitfall]
concepts: [db.composite-key-encoding]
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
Nối ngây thơ ĐỤNG độ vì mất thông tin ranh giới (bài trước). Chèn
thêm MỘT ký tự đặc biệt giữa hai trường — LÀM sao đủ để CỨU vấn đề
này?
::::

::::explain{#chen-ky-tu-phan-cach}
`ma_hoa_ghep` giống HỆT `noi_ngay_tho` (bài trước), CHỈ thêm một
dòng: chèn ký tự `'\0'` (ký tự RỖNG, mã Unicode `0`) NGAY sau khi
nối xong trường `a`, TRƯỚC khi bắt đầu nối trường `b`:

```rust title=readonly
fn ma_hoa_ghep(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    let mut ra: Vec<char> = Vec::new();
    let mut i = 0;
    while i < a.len() {
        ra.push(a[i]);
        i = 1 + i;
    }
    ra.push('\0');
    let mut j = 0;
    while j < b.len() {
        ra.push(b[j]);
        j = 1 + j;
    }
    ra
}

fn main() {
    let a1: Vec<char> = vec!['a','b'];
    let b1: Vec<char> = vec!['c'];
    let a2: Vec<char> = vec!['a'];
    let b2: Vec<char> = vec!['b','c'];
    let k1 = ma_hoa_ghep(&a1, &b1);
    let k2 = ma_hoa_ghep(&a2, &b2);
    println!("{:?}", k1);
    println!("{:?}", k2);
    println!("{}", k1 == k2);
}
```

```text title=readonly
['a', 'b', '\0', 'c']
['a', '\0', 'b', 'c']
false
```

CẶP `("ab", "c")` giờ cho `['a', 'b', '\0', 'c']`, CẶP `("a", "bc")`
cho `['a', '\0', 'b', 'c']` — hai `Vec<char>` KHÁC nhau (vị trí của
`'\0'` LỆCH nhau). Đụng độ của bài TRƯỚC đã biến mất: ký tự phân
cách "khoá" đúng vị trí ranh GIỚI giữa hai trường VÀO trong chính
khoá kết quả.
::::

::::example{#dieu-kien-can}
Cách sửa NÀY hoạt động với MỘT điều kiện: `'\0'` KHÔNG bao giờ xuất
hiện BÊN trong dữ liệu THẬT của trường `a` hay `b`. Tên bảng, tên
cột — chữ CÁI, chữ SỐ, gạch dưới — KHÔNG hề chứa ký tự mã `0`. Nếu
điều kiện ĐÓ bị vi phạm (một trường DỮ liệu thật sự chứa `'\0'`),
đụng độ vẫn có thể XẢY ra trở lại — VÍ dụ trường `a` LÀ `"x\0y"` VÀ
trường `b` LÀ `"z"` sẽ cho CÙNG khoá với trường `a` LÀ `"x"` VÀ
trường `b` LÀ `"y\0z"`. Chọn một ký tự phân CÁCH "an toàn" nghĩa LÀ
chọn một ký tự BIẾT chắc không xuất hiện trong miền dữ liệu — không
phải một phép mã hoá đúng với MỌI đầu vào bất kỳ.
::::

::::predict{#doan-du-lieu-chua-phan-cach commitOnce}
Trường `a` LÀ `['x', '\0', 'y']` (dữ liệu chứa SẴN ký tự phân cách BÊN
trong), trường `b` LÀ `['z']`. Trường `a2` LÀ `['x']`, trường `b2` LÀ
`['y', '\0', 'z']`. So sánh `ma_hoa_ghep(&a,&b) == ma_hoa_ghep(&a2,&b2)`
cho kết quả gì?

:::opt{correct}
`true` — cả hai đều mã hoá thành `['x','\0','y','\0','z']`, ĐỤNG độ xảy
ra ĐÚNG như cảnh báo Ở trên
:::

:::opt
`false` — `ma_hoa_ghep` LUÔN chèn `'\0'` NGAY sau khi nối xong `a`, VỊ
trí phân cách VẪN đủ để phân biệt hai cặp, bất kể dữ liệu bên TRONG chứa
gì
::why
Gần đúng ở việc bạn nhớ ĐÚNG rằng `ma_hoa_ghep` LUÔN chèn `'\0'` NGAY
sau khi nối xong `a` — một quan sát chính XÁC về CÁCH hàm hoạt động.

Chỗ lệch: hàm KHÔNG hề kiểm tra `a`/`b` CÓ chứa sẵn `'\0'` hay không —
nó chỉ nối RỒI chèn phân cách MÙ quáng theo đúng vị trí ranh giới trường
`a`/`b`, không biết gì VỀ nội dung bên trong. `a=['x','\0','y']` nối VỚI
`b=['z']` cho `['x','\0','y','\0','z']`; `a2=['x']` nối VỚI
`b2=['y','\0','z']` CŨNG cho ĐÚNG `['x','\0','y','\0','z']` — GIỐNG hệt.
Đây chính LÀ điều KIỆN cần Ở trên: `'\0'` PHẢI không xuất hiện trong dữ
liệu THẬT, nếu không đụng ĐỘ vẫn xảy ra.
::
:::
::::

::::code{#viet_ma_hoa_ghep}
Hoàn thiện `ma_hoa_ghep` — chèn ký tự phân cách `'\0'` NGAY sau khi
nối xong trường `a`.

```rust title=starter
fn ma_hoa_ghep(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    let mut ra: Vec<char> = Vec::new();
    let mut i = 0;
    while i < a.len() {
        ra.push(a[i]);
        i = 1 + i;
    }
    ___
    let mut j = 0;
    while j < b.len() {
        ra.push(b[j]);
        j = 1 + j;
    }
    ra
}

fn main() {
    let a1: Vec<char> = vec!['a','b'];
    let b1: Vec<char> = vec!['c'];
    let a2: Vec<char> = vec!['a'];
    let b2: Vec<char> = vec!['b','c'];
    let k1 = ma_hoa_ghep(&a1, &b1);
    let k2 = ma_hoa_ghep(&a2, &b2);
    println!("{:?}", k1);
    println!("{:?}", k2);
    println!("{}", k1 == k2);
}
```

```rust title=solution
fn ma_hoa_ghep(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    let mut ra: Vec<char> = Vec::new();
    let mut i = 0;
    while i < a.len() {
        ra.push(a[i]);
        i = 1 + i;
    }
    ra.push('\0');
    let mut j = 0;
    while j < b.len() {
        ra.push(b[j]);
        j = 1 + j;
    }
    ra
}

fn main() {
    let a1: Vec<char> = vec!['a','b'];
    let b1: Vec<char> = vec!['c'];
    let a2: Vec<char> = vec!['a'];
    let b2: Vec<char> = vec!['b','c'];
    let k1 = ma_hoa_ghep(&a1, &b1);
    let k2 = ma_hoa_ghep(&a2, &b2);
    println!("{:?}", k1);
    println!("{:?}", k2);
    println!("{}", k1 == k2);
}
```

```rust title=test
fn ma_hoa_ghep(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    let mut ra: Vec<char> = Vec::new();
    let mut i = 0;
    while i < a.len() {
        ra.push(a[i]);
        i = 1 + i;
    }
    ra.push('\0');
    let mut j = 0;
    while j < b.len() {
        ra.push(b[j]);
        j = 1 + j;
    }
    ra
}

fn main() {
    let a1: Vec<char> = vec!['a','b'];
    let b1: Vec<char> = vec!['c'];
    let a2: Vec<char> = vec!['a'];
    let b2: Vec<char> = vec!['b','c'];
    let k1 = ma_hoa_ghep(&a1, &b1);
    let k2 = ma_hoa_ghep(&a2, &b2);
    println!("{}", k1 == k2);
    assert_eq!(k1 == k2, false, "hai khoa ghep khac nhau khong duoc dung nhau sau khi ma hoa");

    let k3 = ma_hoa_ghep(&a1, &b1);
    assert_eq!(k1 == k3, true, "ma hoa cung mot cap phai ra cung mot khoa");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Chen ky tu phan cach '\\0' vao ra ngay sau vong lap dau, truoc vong lap thu hai -- dung ra.push, mot dong."
- kind: strategy
  body: "ra.push('\\0');"
- kind: one-line
  body: "ra.push('\\0');"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "false"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khoá ghép AN toàn rồi. Nhưng khoá đã mã hoá xong thì DÙNG để LÀM gì
— một cấu trúc TRA cứu thật sự trông ra sao?
::::

::::reflect{#nghi-lai}
`ma_hoa_ghep` sửa đúng LỖI của bài trước bằng CÁCH khôi phục lại
thông tin ranh GIỚI đã mất — chèn một tín hiệu (ký tự phân cách)
VÀO đúng vị trí biên giữa hai trường. Đây LÀ kỹ thuật CHUNG cho MỌI
khoá ghép từ nhiều trường: hoặc chèn phân CÁCH "an toàn" (như ở
đây — CẦN biết trước miền dữ liệu để chọn đúng ký tự), hoặc mã hoá ĐỘ
dài từng trường TRƯỚC nội dung của nó (length-prefix — tổng QUÁT
hơn, không cần giả định gì VỀ dữ liệu, ngoài phạm vi bài NÀY). Khoá
SỐ (bài 2) VÀ khoá GHÉP (bài này) đều mã hoá XONG — nhưng một khoá
đã mã hoá thì DÙNG để tra cứu NHƯ thế nào, khi bảng CÓ hàng nghìn
hàng?
::::

::::checkpoint{mastery=0.8}
::::
