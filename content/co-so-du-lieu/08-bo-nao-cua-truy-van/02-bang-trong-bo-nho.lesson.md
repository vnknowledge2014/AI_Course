---
id: co-so-du-lieu.bo-nao-cua-truy-van.bang-trong-bo-nho
title: "Bảng trong bộ nhớ"
summary: "Một BẢNG (table) trong bộ nhớ là Vec<Vec<Truong>> — Vec các hàng, mỗi hàng lại là Vec các trường. bang[chi_so] LÀ một hàng cụ thể, tra_cuu_trong_bang gộp chỉ số hàng với tra_cuu (bài trước) để lấy đúng một giá trị."
locale: vi
track: co-so-du-lieu
module: bo-nao-cua-truy-van
order: 2
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 10
teaches: [db.executor-table]
requires: [db.executor-row]
concepts: [db.executor-table]
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
`tra_cuu` (bài trước) đọc ĐÚNG một HÀNG. Một câu truy vấn chạy TRÊN
nhiều hàng — một BẢNG trông ra sao?
::::

::::explain{#bang-la-vec-vec}
Một BẢNG trong bộ nhớ đơn giản LÀ `Vec<Vec<Truong>>` — `Vec` các
HÀNG, mỗi hàng lại LÀ `Vec` các trường:

```rust title=readonly
struct Truong { ten: Vec<char>, gia_tri: i64 }

fn main() {
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}, Truong{ten:vec!['t','e','n'],gia_tri:1}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}, Truong{ten:vec!['t','e','n'],gia_tri:2}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:30}, Truong{ten:vec!['t','e','n'],gia_tri:3}],
    ];
    println!("{}", bang.len());
    println!("{}", bang[0].len());
}
```

```text title=readonly
3
2
```

`bang.len()` LÀ số HÀNG (`3`). `bang[0].len()` LÀ số TRƯỜNG của
hàng ĐẦU tiên (`2` — `tuoi` VÀ `ten`). `bang[i]` LÀ đúng kiểu
`Vec<Truong>` mà `tra_cuu` (bài trước) đã nhận LÀM tham số — INDEX
hai TẦNG (`bang[i][j]`) đơn giản LÀ index LỒNG nhau, không có gì
mới.
::::

::::example{#index-long-nhau}
Đọc trực tiếp MỘT trường của MỘT hàng cụ thể bằng index HAI tầng:

```rust title=readonly
println!("{}", bang[1][0].gia_tri);
println!("{}", bang[2][1].gia_tri);
```

```text title=readonly
15
3
```

`bang[1]` LÀ hàng THỨ hai (`tuoi=15`), `[0]` LÀ trường ĐẦU tiên của
hàng đó — `.gia_tri` LÀ `15`. `bang[2][1]` LÀ trường THỨ hai (`ten`)
của hàng THỨ ba — `3`. Cách VIẾT index-lồng-index nhìn RA CẤU trúc
"bảng LÀ danh sách của danh sách" một cách trực tiếp, không cần
trung gian.
::::

::::predict{#doan-chi-so-vuot-qua commitOnce}
Gọi `bang[3]` — chỉ số `3`, trong khi `bang` chỉ CÓ ba hàng (chỉ số
hợp lệ LÀ `0`, `1`, `2`):

```rust
println!("{}", bang[3][0].gia_tri);
```

Dòng cuối LÀM gì?

:::opt{correct}
Máy báo lỗi lúc CHẠY (panic vì chỉ số vượt quá giới hạn)
:::

:::opt
In ra `0` — vì `Vec` tự động trả VỀ giá trị mặc định KHI chỉ số
vượt quá độ dài, giống hệt `tra_cuu` (bài trước) trả VỀ `0` khi
không TÌM thấy trường
::why
Gần đúng ở việc bạn nhớ ĐÚNG `tra_cuu` (bài trước) có hành vi "trả
về mặc định khi không TÌM thấy" — một liên tưởng hợp lý.

Chỗ lệch: `tra_cuu` TỰ mình kiểm tra "có tìm thấy hay không" bằng
một vòng LẶP tường minh, RỒI CHỦ ĐỘNG `return 0` — đó LÀ CODE do
CHÍNH ta viết. Toán tử INDEX `[]` trên `Vec` LÀ một cơ chế KHÁC
hẳn — nó LUÔN kiểm tra chỉ số CÓ nằm trong phạm vi hay KHÔNG, và
NGAY khi phát hiện chỉ số `3` vượt quá độ dài `3` (chỉ số hợp lệ
LÀ `0..2`), nó `panic` — dừng chương trình NGAY, không hề có giá
trị "mặc định" nào cho trường hợp NÀY.
::
:::
::::

::::code{#viet_tra_cuu_trong_bang}
Hoàn thiện `tra_cuu_trong_bang` — lấy đúng HÀNG rồi tra cứu trường
TRONG hàng đó.

```rust title=starter
struct Truong { ten: Vec<char>, gia_tri: i64 }

fn tra_cuu(hang: &Vec<Truong>, ten: &Vec<char>) -> i64 {
    let mut i = 0;
    while i < hang.len() {
        if hang[i].ten == *ten { return hang[i].gia_tri; }
        i += 1;
    }
    0
}

fn tra_cuu_trong_bang(bang: &Vec<Vec<Truong> >, chi_so: usize, ten: &Vec<char>) -> i64 {
    ___
}

fn main() {
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}, Truong{ten:vec!['t','e','n'],gia_tri:1}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}, Truong{ten:vec!['t','e','n'],gia_tri:2}],
    ];
    let ten_tuoi: Vec<char> = vec!['t','u','o','i'];
    println!("{}", tra_cuu_trong_bang(&bang, 0, &ten_tuoi));
}
```

```rust title=solution
struct Truong { ten: Vec<char>, gia_tri: i64 }

fn tra_cuu(hang: &Vec<Truong>, ten: &Vec<char>) -> i64 {
    let mut i = 0;
    while i < hang.len() {
        if hang[i].ten == *ten { return hang[i].gia_tri; }
        i += 1;
    }
    0
}

fn tra_cuu_trong_bang(bang: &Vec<Vec<Truong> >, chi_so: usize, ten: &Vec<char>) -> i64 {
    tra_cuu(&bang[chi_so], ten)
}

fn main() {
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}, Truong{ten:vec!['t','e','n'],gia_tri:1}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}, Truong{ten:vec!['t','e','n'],gia_tri:2}],
    ];
    let ten_tuoi: Vec<char> = vec!['t','u','o','i'];
    println!("{}", tra_cuu_trong_bang(&bang, 0, &ten_tuoi));
}
```

```rust title=test
fn main() {
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}, Truong{ten:vec!['t','e','n'],gia_tri:1}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}, Truong{ten:vec!['t','e','n'],gia_tri:2}],
    ];
    let ten_tuoi: Vec<char> = vec!['t','u','o','i'];
    println!("{}", tra_cuu_trong_bang(&bang, 0, &ten_tuoi));
    assert_eq!(tra_cuu_trong_bang(&bang, 0, &ten_tuoi), 20, "hang 0, truong tuoi phai la 20");
    assert_eq!(tra_cuu_trong_bang(&bang, 1, &ten_tuoi), 15, "hang 1, truong tuoi phai la 15");

    let ten_la: Vec<char> = vec!['l','a'];
    assert_eq!(tra_cuu_trong_bang(&bang, 0, &ten_la), 0, "truong khong ton tai -- 0");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Goi tra_cuu tren dung hang (bang[chi_so]) va ten -- mot dong."
- kind: strategy
  body: "tra_cuu(&bang[chi_so], ten)"
- kind: one-line
  body: "tra_cuu(&bang[chi_so], ten)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "20"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bảng đọc được rồi. Nhưng `WHERE tuoi > 18` cần SO SÁNH — đánh giá
MỘT phép so sánh trên một hàng trông ra sao?
::::

::::reflect{#nghi-lai}
`Vec<Vec<Truong>>` LÀ biểu diễn ĐƠN giản nhất cho một bảng trong bộ
nhớ — không chỉ mục, không tối ưu, CHỈ LÀ danh sách hàng THEO thứ tự
chèn (giống hệt tinh THẦN "quét toàn bộ" mà B+Tree, LSM Ở R6-1 được
XÂY để TRÁNH — quest NÀY cố tình dùng bản ĐƠN giản nhất để tập trung
VÀO logic thực THI, không phải tối ưu lưu trữ). Index vượt quá giới
hạn `panic` NGAY, không có giá trị mặc định NÀO — khác HẲN `tra_cuu`
tự viết. Bảng đọc được — bước tiếp LÀ đánh giá MỘT điều kiện `WHERE`
(như `tuoi > 18`) trên một hàng CỤ thể.
::::

::::checkpoint{mastery=0.8}
::::
