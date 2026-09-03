---
id: co-so-du-lieu.bo-nao-cua-truy-van.danh-gia-mot-phep-so-sanh
title: "Đánh giá một phép so sánh trên một hàng"
summary: "danh_gia_so_sanh(hang, ten, dau, so) tra_cuu giá trị trường rồi so sánh THEO dấu ('>', '<', còn lại coi là '='). Trường KHÔNG tồn tại tra_cuu trả về 0 (bài 1) — so sánh dùng LUÔN giá trị 0 đó, không báo lỗi riêng."
locale: vi
track: co-so-du-lieu
module: bo-nao-cua-truy-van
order: 3
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 11
teaches: [db.executor-eval-so-sanh]
requires: [db.executor-table]
concepts: [db.executor-eval-so-sanh]
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
Bảng đọc được (bài trước). `WHERE tuoi > 18` LÀ một phép SO sánh —
đánh giá NÓ trên MỘT hàng cụ thể, ĐÚNG hay SAI?
::::

::::explain{#danh-gia-so-sanh}
`danh_gia_so_sanh` tra cứu giá TRỊ trường (bài 1), RỒI so sánh THEO
đúng dấu:

```rust title=readonly
fn danh_gia_so_sanh(hang: &Vec<Truong>, ten: &Vec<char>, dau: char, so: i64) -> bool {
    let gt = tra_cuu(hang, ten);
    if dau == '>' { return gt > so; }
    if dau == '<' { return gt < so; }
    gt == so
}

fn main() {
    let hang: Vec<Truong> = vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}];
    let ten_tuoi: Vec<char> = vec!['t','u','o','i'];
    println!("{}", danh_gia_so_sanh(&hang, &ten_tuoi, '>', 18));
}
```

```text title=readonly
true
```

`tra_cuu` LẤY `gia_tri` của trường `tuoi` (LÀ `20`), RỒI so `20 >
18` — `true`. Ba NHÁNH `if`: `'>'`, `'<'`, VÀ dòng cuối `gt == so`
LÀM nhánh MẶC định (bao gồm CẢ `'='` VÀ mọi ký tự khác — track NÀY
chỉ CÓ ba dấu, xem q07 bài 5).
::::

::::example{#hai-dau-con-lai}
Dấu `<` VÀ `=` hoạt động TƯƠNG tự:

```rust title=readonly
println!("{}", danh_gia_so_sanh(&hang, &ten_tuoi, '<', 18));
println!("{}", danh_gia_so_sanh(&hang, &ten_tuoi, '=', 20));
```

```text title=readonly
false
true
```

`20 < 18` LÀ `false`. `20 == 20` LÀ `true`. Cả BA dấu dùng CHUNG một
giá trị `gt` tra cứu MỘT lần DUY nhất Ở ĐẦU hàm — không tra cứu LẠI
cho MỖI nhánh so sánh.
::::

::::predict{#doan-truong-thieu commitOnce}
So sánh MỘT trường KHÔNG tồn tại trong hàng:

```rust
let ten_thieu: Vec<char> = vec!['d','i','e','m'];
println!("{}", danh_gia_so_sanh(&hang, &ten_thieu, '>', 0));
```

Hàng CHỈ có trường `tuoi`, không CÓ `diem`. Dòng cuối in ra gì?

:::opt{correct}
`false`
:::

:::opt
Máy báo lỗi — vì so sánh MỘT trường KHÔNG tồn tại VỚI bất kỳ giá
trị NÀO là một phép toán không hợp lệ
::why
Gần đúng ở việc bạn nghĩ TỚI "trường không tồn tại" như một trường
hợp CẦN được BÁO riêng — một trực giác hợp LÝ khi thiết kế hệ thống
truy vấn THẬT (nhiều CSDL thật BÁO lỗi cột không tồn tại LÚC parse,
không LÚC thực thi).

Chỗ lệch: `danh_gia_so_sanh` không hề PHÂN biệt "trường thiếu" VỚI
"trường CÓ giá trị 0" — nó GỌI THẲNG `tra_cuu` (bài 1), VỐN trả VỀ
`0` khi không TÌM thấy. So sánh TRỞ thành `0 > 0`, kết quả `false`
— một hệ QUẢ tự nhiên của thiết kế `tra_cuu`, KHÔNG phải một trường
hợp được xử LÝ riêng.
::
:::
::::

::::code{#viet_danh_gia_so_sanh}
Hoàn thiện `danh_gia_so_sanh` — nhánh so sánh `<`.

```rust title=starter
struct Truong {
    ten: Vec<char>,
    gia_tri: i64,
}

fn tra_cuu(hang: &Vec<Truong>, ten: &Vec<char>) -> i64 {
    let mut i = 0;
    while i < hang.len() {
        if hang[i].ten == *ten { return hang[i].gia_tri; }
        i += 1;
    }
    0
}

fn danh_gia_so_sanh(hang: &Vec<Truong>, ten: &Vec<char>, dau: char, so: i64) -> bool {
    let gt = tra_cuu(hang, ten);
    if dau == '>' { return gt > so; }
    if ___ { return gt < so; }
    gt == so
}

fn main() {
    let hang: Vec<Truong> = vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}];
    let ten_tuoi: Vec<char> = vec!['t','u','o','i'];
    println!("{}", danh_gia_so_sanh(&hang, &ten_tuoi, '<', 30));
}
```

```rust title=solution
struct Truong {
    ten: Vec<char>,
    gia_tri: i64,
}

fn tra_cuu(hang: &Vec<Truong>, ten: &Vec<char>) -> i64 {
    let mut i = 0;
    while i < hang.len() {
        if hang[i].ten == *ten { return hang[i].gia_tri; }
        i += 1;
    }
    0
}

fn danh_gia_so_sanh(hang: &Vec<Truong>, ten: &Vec<char>, dau: char, so: i64) -> bool {
    let gt = tra_cuu(hang, ten);
    if dau == '>' { return gt > so; }
    if dau == '<' { return gt < so; }
    gt == so
}

fn main() {
    let hang: Vec<Truong> = vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}];
    let ten_tuoi: Vec<char> = vec!['t','u','o','i'];
    println!("{}", danh_gia_so_sanh(&hang, &ten_tuoi, '<', 30));
}
```

```rust title=test
struct Truong {
    ten: Vec<char>,
    gia_tri: i64,
}

fn tra_cuu(hang: &Vec<Truong>, ten: &Vec<char>) -> i64 {
    let mut i = 0;
    while i < hang.len() {
        if hang[i].ten == *ten { return hang[i].gia_tri; }
        i += 1;
    }
    0
}

fn main() {
    let hang: Vec<Truong> = vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}];
    let ten_tuoi: Vec<char> = vec!['t','u','o','i'];
    println!("{}", danh_gia_so_sanh(&hang, &ten_tuoi, '<', 30));
    assert_eq!(danh_gia_so_sanh(&hang, &ten_tuoi, '<', 30), true, "20 < 30 phai la true");
    assert_eq!(danh_gia_so_sanh(&hang, &ten_tuoi, '<', 10), false, "20 < 10 phai la false");
    assert_eq!(danh_gia_so_sanh(&hang, &ten_tuoi, '>', 18), true, "20 > 18 phai la true");
    assert_eq!(danh_gia_so_sanh(&hang, &ten_tuoi, '=', 20), true, "20 == 20 phai la true");

    let ten_thieu: Vec<char> = vec!['d','i','e','m'];
    assert_eq!(danh_gia_so_sanh(&hang, &ten_thieu, '>', 0), false, "truong thieu -- tra_cuu ra 0, 0>0 la false");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Dieu kien nhanh thu hai phai so sanh dau voi ky tu '<' -- mot dong."
- kind: strategy
  body: "if dau == '<' { return gt < so; }"
- kind: one-line
  body: "dau == '<'"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một phép so sánh đánh giá được rồi. Nhưng `WHERE` CÓ thể LÀ cả một
CÂY (`AND`/`OR`) — đánh giá TRỌN cây trên một hàng trông ra sao?
::::

::::reflect{#nghi-lai}
`danh_gia_so_sanh` LÀ đơn vị NHỎ nhất của việc THỰC thi `WHERE` —
đúng một trường, một dấu, một SỐ, một quyết định `true`/`false`. Nó
KẾ thừa nguyên vẹn hành vi "trường thiếu → `0`" từ `tra_cuu` (bài
1), không tự thêm luật RIÊNG — một chuỗi trách nhiệm rõ ràng: MỖI
hàm chỉ LÀM đúng việc CỦA nó. `WHERE tuoi > 18 AND diem < 100` LÀ
CẢ một CÂY (`NoAst::VaNut` gộp hai `SoSanh` — q07 bài 7-10) — đánh
giá TRỌN cây đó trên MỘT hàng, đệ quy qua CÂY, trông ra sao?
::::

::::checkpoint{mastery=0.8}
::::
