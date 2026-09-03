---
id: co-so-du-lieu.bo-nao-cua-truy-van.ke-hoach-la-danh-sach-buoc
title: "Kế hoạch logic là một danh sách bước"
summary: "BuocKeHoach { Quet(Vec<char>), Loc(usize), ChonCot(Vec<Vec<char>>) } — ba loại bước một câu truy vấn đơn giản cần. KeHoach = Vec<BuocKeHoach>. Ngữ pháp quest này không có JOIN nên kế hoạch TUYẾN TÍNH (một danh sách) là đủ — không cần cây/arena như CayAst."
locale: vi
track: co-so-du-lieu
module: bo-nao-cua-truy-van
order: 5
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 10
teaches: [db.logical-plan]
requires: [db.executor-eval-cay]
concepts: [db.logical-plan]
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
`WHERE` đánh giá được trên MỘT hàng (bài trước). Chạy MỘT câu truy
vấn nghĩa LÀ LÀM ba việc: quét bảng, lọc hàng, chọn cột — thứ TỰ đó
gọi LÀ gì?
::::

::::explain{#buoc-ke-hoach}
`BuocKeHoach` LÀ một trong BA loại bước — `KeHoach` LÀ một DANH
SÁCH các bước, THEO đúng thứ tự thực thi:

```rust title=readonly
enum BuocKeHoach {
    Quet(Vec<char>),
    Loc(usize),
    ChonCot(Vec<Vec<char> >),
}

fn main() {
    let mut ke_hoach: Vec<BuocKeHoach> = Vec::new();
    let ten_bang: Vec<char> = vec!['n','g','u','o','i'];
    ke_hoach.push(BuocKeHoach::Quet(ten_bang));
    ke_hoach.push(BuocKeHoach::Loc(0));
    let cot: Vec<Vec<char> > = vec![vec!['t','e','n']];
    ke_hoach.push(BuocKeHoach::ChonCot(cot));
    println!("{}", ke_hoach.len());
}
```

```text title=readonly
3
```

`Quet(ten_bang)` — quét bảng NÀO. `Loc(chi_so)` — lọc THEO điều
kiện Ở chỉ số NÀO trong `CayAst` (bài 4 — `KeHoach` KHÔNG chứa lại
cây điều kiện, chỉ GIỮ chỉ số TRỎ vào `CayAst` dùng CHUNG). `ChonCot`
— danh sách TÊN cột giữ LẠI. Ngữ pháp quest NÀY KHÔNG có `JOIN`, nên
`KeHoach` LÀ một `Vec` TUYẾN TÍNH — KHÔNG cần cây/arena như `CayAst`
(q07 bài 7).
::::

::::example{#doc-mot-buoc}
Đọc LẠI một bước cụ thể bằng `match`:

```rust title=readonly
match &ke_hoach[1] {
    BuocKeHoach::Loc(i) => println!("loc tai chi so {}", i),
    _ => println!("khac"),
}
```

```text title=readonly
loc tai chi so 0
```

`ke_hoach[1]` LÀ `BuocKeHoach::Loc(0)` — trích RA `0`, đúng CHỈ số
điều kiện WHERE trong `CayAst`. Mỗi loại `BuocKeHoach` mang ĐÚNG
dữ liệu nó CẦN, không THỪA không THIẾU.
::::

::::predict{#doan-khong-co-loc commitOnce}
Một câu truy vấn KHÔNG có `WHERE` — kế hoạch của NÓ chỉ CÓ `Quet`
VÀ `ChonCot`, không CÓ `Loc`:

```rust
let mut ke_hoach2: Vec<BuocKeHoach> = Vec::new();
let ten_bang2: Vec<char> = vec!['b','a','n'];
ke_hoach2.push(BuocKeHoach::Quet(ten_bang2));
let cot2: Vec<Vec<char> > = vec![vec!['t','e','n']];
ke_hoach2.push(BuocKeHoach::ChonCot(cot2));
println!("{}", ke_hoach2.len());
```

Dòng cuối in ra gì?

:::opt{correct}
`2`
:::

:::opt
Máy báo lỗi — vì một `KeHoach` HỢP LỆ luôn phải CÓ đủ ba bước
(`Quet`, `Loc`, `ChonCot`), thiếu MỘT bước LÀ dữ liệu sai
::why
Gần đúng ở việc bạn nghĩ TỚI "ba bước LÀ một bộ ĐẦY đủ" như một
RÀNG buộc — MỘT trực giác hợp lý nếu nhìn ba bước LÀ MỘT khuôn cố
định.

Chỗ lệch: `KeHoach` chỉ đơn thuần LÀ `Vec<BuocKeHoach>` — MỘT danh
sách BÌNH thường, độ dài BAO nhiêu cũng ĐƯỢC, tuỳ nội DUNG câu truy
vấn. `WHERE` LÀ TUỲ chọn (q07 bài 11) — không CÓ `WHERE` thì đơn
giản KHÔNG có bước `Loc` nào ĐƯỢC thêm VÀO, kế hoạch chỉ CÒN hai
bước. Không CÓ kiểm tra "phải đủ BA" Ở đâu cả.
::
:::
::::

::::code{#viet_ke_hoach}
Thêm bước `ChonCot` VÀO kế hoạch — chọn giữ LẠI đúng MỘT cột `ten`.

```rust title=starter
enum BuocKeHoach {
    Quet(Vec<char>),
    Loc(usize),
    ChonCot(Vec<Vec<char> >),
}

fn main() {
    let mut ke_hoach: Vec<BuocKeHoach> = Vec::new();
    let ten_bang: Vec<char> = vec!['n','g','u','o','i'];
    ke_hoach.push(BuocKeHoach::Quet(ten_bang));
    ke_hoach.push(BuocKeHoach::Loc(0));
    let cot: Vec<Vec<char> > = vec![vec!['t','e','n']];
    ___
    println!("{}", ke_hoach.len());
}
```

```rust title=solution
enum BuocKeHoach {
    Quet(Vec<char>),
    Loc(usize),
    ChonCot(Vec<Vec<char> >),
}

fn main() {
    let mut ke_hoach: Vec<BuocKeHoach> = Vec::new();
    let ten_bang: Vec<char> = vec!['n','g','u','o','i'];
    ke_hoach.push(BuocKeHoach::Quet(ten_bang));
    ke_hoach.push(BuocKeHoach::Loc(0));
    let cot: Vec<Vec<char> > = vec![vec!['t','e','n']];
    ke_hoach.push(BuocKeHoach::ChonCot(cot));
    println!("{}", ke_hoach.len());
}
```

```rust title=test
fn main() {
    let mut ke_hoach: Vec<BuocKeHoach> = Vec::new();
    let ten_bang: Vec<char> = vec!['n','g','u','o','i'];
    ke_hoach.push(BuocKeHoach::Quet(ten_bang));
    ke_hoach.push(BuocKeHoach::Loc(0));
    let cot: Vec<Vec<char> > = vec![vec!['t','e','n']];
    ke_hoach.push(BuocKeHoach::ChonCot(cot));
    println!("{}", ke_hoach.len());
    assert_eq!(ke_hoach.len(), 3, "phai co dung 3 buoc");

    match &ke_hoach[2] {
        BuocKeHoach::ChonCot(c) => {
            assert_eq!(c.len(), 1, "chon dung 1 cot");
            assert_eq!(c[0], vec!['t','e','n'], "cot phai la ten");
        },
        _ => panic!("buoc thu 3 phai la ChonCot"),
    }

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Day mot BuocKeHoach::ChonCot(cot) vao ke_hoach -- mot dong."
- kind: strategy
  body: "ke_hoach.push(BuocKeHoach::ChonCot(cot));"
- kind: one-line
  body: "ke_hoach.push(BuocKeHoach::ChonCot(cot));"
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
Kế hoạch có hình DẠNG rồi — TỰ tay dựng. Xây nó TỰ ĐỘNG từ một
`CauTruyVan` (q07) trông ra sao?
::::

::::reflect{#nghi-lai}
`BuocKeHoach`/`KeHoach` biến MỘT câu truy vấn thành một DANH SÁCH
hành động CỤ thể — `Quet` LẤY bảng nào, `Loc` áp dụng điều kiện Ở
đâu, `ChonCot` GIỮ cột nào. Ngữ pháp quest NÀY không CÓ `JOIN` nên
kế hoạch KHÔNG cần cây — một DANH sách TUYẾN TÍNH LÀ đủ, đơn giản
hơn hẳn `CayAst`. Kế hoạch NÀY hiện ĐANG tự tay dựng TỪNG bước — xây
nó TỰ ĐỘNG từ `CauTruyVan` (kết quả `parse`, q07) trông ra sao?
::::

::::checkpoint{mastery=0.8}
::::
