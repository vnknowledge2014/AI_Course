---
id: co-so-du-lieu.bo-nao-cua-truy-van.thuc-thi-buoc-quet
title: "Thực thi bước Quét qua một vị trí con trỏ"
summary: "buoc_quet(bang, vi_tri: &mut usize) -> Option<Vec<Truong>> hiện thực hoá BuocKeHoach::Quet theo mô hình Volcano (bài trước): None khi *vi_tri >= bang.len(), ngược lại sao chép hàng tại *vi_tri rồi TỰ tăng *vi_tri lên một. Trạng thái con trỏ sống Ở NGOÀI hàm — mỗi lần gọi lại chỉ tiến thêm một bước, không tính lại từ đầu."
locale: vi
track: co-so-du-lieu
module: bo-nao-cua-truy-van
order: 9
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 12
teaches: [db.executor-scan-step]
requires: [db.volcano-idea]
concepts: [db.executor-scan-step]
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
Mô hình Volcano kéo TỪNG hàng một (bài trước). `BuocKeHoach::Quet`
LÀ bước ĐẦU tiên trong mọi kế hoạch — thực thi nó CỤ thể ra sao?
::::

::::explain{#buoc-quet-cu-the}
`buoc_quet` NHẬN `vi_tri: &mut usize` — con TRỎ đang Ở đâu — trả VỀ
`None` khi HẾT bảng, ngược LẠI sao chép hàng TẠI `*vi_tri` rồi TỰ
TĂNG con trỏ lên MỘT:

```rust title=readonly
fn buoc_quet(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize) -> Option<Vec<Truong> > {
    if *vi_tri >= bang.len() {
        return None;
    }
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
```

Điều kiện DỪNG (`*vi_tri >= bang.len()`) LUÔN kiểm TRƯỚC — nếu con
TRỎ đã vượt quá số hàng, TRẢ `None` NGAY, không hề ĐỘNG tới `bang[*
vi_tri]` (chỉ số ngoài phạm VI sẽ gây lỗi). Chỉ khi CÒN hàng, hàm mới
sao CHÉP từng `Truong` (bài 1) sang MỘT `Vec` mới, RỒI tăng con trỏ.
::::

::::example{#goi-ba-lan}
Một bảng BA hàng — gọi `buoc_quet` LẶP lại BỐN lần:

```rust title=readonly
let bang: Vec<Vec<Truong> > = vec![
    vec![Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
    vec![Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
    vec![Truong{ten:vec!['t','u','o','i'],gia_tri:40}],
];
let mut vt: usize = 0;
let r1 = buoc_quet(&bang, &mut vt);
match &r1 {
    Some(h) => println!("{}", h[0].gia_tri),
    None => println!("het"),
}
let r2 = buoc_quet(&bang, &mut vt);
match &r2 {
    Some(h) => println!("{}", h[0].gia_tri),
    None => println!("het"),
}
```

```text title=readonly
30
25
```

Mỗi lần GỌI, `vt` (khai BÁO một LẦN, TRUYỀN vào bằng `&mut`) tự
TĂNG THÊM một — hàm KHÔNG hề "nhớ" trạng thái CỦA riêng nó, TOÀN bộ
trạng thái sống Ở BIẾN `vt` bên NGOÀI.
::::

::::predict{#doan-lan-goi-thu-ba commitOnce}
BẢNG Ở trên có BA hàng (`30`, `25`, `40`). Gọi `buoc_quet` LẦN thứ
BA (SAU hai lần Ở trên):

```rust
let r3 = buoc_quet(&bang, &mut vt);
match &r3 {
    Some(h) => println!("{}", h[0].gia_tri),
    None => println!("het"),
}
```

Dòng CUỐI in ra gì?

:::opt{correct}
`40`
:::

:::opt
`het` — vì bảng Ở bài TRƯỚC (bài 8) chỉ có HAI hàng, nên SAU hai
lần gọi LÀ đã hết
::why
Gần đúng ở việc bạn nhớ ĐÚNG ví dụ bài TRƯỚC (bảng HAI hàng, hai
lần gọi LÀ hết) — MỘT phản xạ hợp lý khi hai bài LIÊN tiếp dùng
CÙNG một hàm.

Chỗ lệch: bảng Ở BÀI này (`vt`) có BA hàng, KHÔNG phải hai — `30`,
`25`, VÀ `40`. Kết quả `buoc_quet` phụ THUỘC vào chính `bang.len()`
được TRUYỀN vào, không phải MỘT con số cố ĐỊNH nhớ từ ví dụ trước.
Lần gọi THỨ ba VẪN còn hàng THỨ ba (`40`) để trả VỀ — `het` chỉ
xuất hiện Ở lần gọi thứ TƯ.
::
:::
::::

::::code{#viet_buoc_quet}
Hoàn thiện `buoc_quet` — SAU khi sao chép xong một hàng, TIẾN con
trỏ lên MỘT vị trí.

```rust title=starter
struct Truong {
    ten: Vec<char>,
    gia_tri: i64,
}

fn buoc_quet(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize) -> Option<Vec<Truong> > {
    if *vi_tri >= bang.len() {
        return None;
    }
    let hang = &bang[*vi_tri];
    let mut sao_chep: Vec<Truong> = Vec::new();
    let mut i = 0;
    while i < hang.len() {
        sao_chep.push(Truong { ten: hang[i].ten.clone(), gia_tri: hang[i].gia_tri });
        i = 1 + i;
    };
    ___
    Some(sao_chep)
}

fn main() {
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
    ];
    let mut vt: usize = 0;
    let r1 = buoc_quet(&bang, &mut vt);
    match &r1 {
        Some(h) => println!("{}", h[0].gia_tri),
        None => println!("het"),
    }
    let r2 = buoc_quet(&bang, &mut vt);
    match &r2 {
        Some(h) => println!("{}", h[0].gia_tri),
        None => println!("het"),
    }
}
```

```rust title=solution
struct Truong {
    ten: Vec<char>,
    gia_tri: i64,
}

fn buoc_quet(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize) -> Option<Vec<Truong> > {
    if *vi_tri >= bang.len() {
        return None;
    }
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

fn main() {
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
    ];
    let mut vt: usize = 0;
    let r1 = buoc_quet(&bang, &mut vt);
    match &r1 {
        Some(h) => println!("{}", h[0].gia_tri),
        None => println!("het"),
    }
    let r2 = buoc_quet(&bang, &mut vt);
    match &r2 {
        Some(h) => println!("{}", h[0].gia_tri),
        None => println!("het"),
    }
}
```

```rust title=test
fn main() {
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
    ];
    let mut vt: usize = 0;
    let r1 = buoc_quet(&bang, &mut vt);
    println!("{}", r1.is_some());
    match &r1 {
        Some(h) => assert_eq!(h[0].gia_tri, 30, "hang dau tien phai la 30"),
        None => panic!("phai con hang"),
    }
    let r2 = buoc_quet(&bang, &mut vt);
    match &r2 {
        Some(h) => assert_eq!(h[0].gia_tri, 25, "hang thu hai phai la 25"),
        None => panic!("phai con hang"),
    }
    let r3 = buoc_quet(&bang, &mut vt);
    match &r3 {
        Some(_) => panic!("bang chi co 2 hang -- phai het sau lan goi thu ba"),
        None => {},
    }
    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Sau khi sao chep xong hang, tien vi_tri len mot -- mot dong, gan qua dau *."
- kind: strategy
  body: "*vi_tri = 1 + *vi_tri;"
- kind: one-line
  body: "*vi_tri = 1 + *vi_tri;"
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
Quét xong một hàng — nhưng CHƯA lọc gì cả. Ghép `Loc` VÀO ngay
trong bước THỰC thi, trông ra sao?
::::

::::reflect{#nghi-lai}
`buoc_quet` LÀ hiện thực CỤ thể đầu tiên của Ý tưởng Volcano (bài
trước): TRẠNG thái (`vi_tri`) sống Ở NGOÀI hàm, mỗi lần GỌI chỉ
LÀM đúng hai việc — kiểm TRA còn hàng không, RỒI (nếu còn) sao chép
MỘT hàng VÀ tiến con trỏ. Đây MỚI CHỈ LÀ bước `Quet` — chưa hề LỌC
theo `WHERE`, chưa hề CHỌN cột theo `SELECT`. Bước TIẾP theo trong
kế hoạch LÀ `Loc` — ghép nó VÀO ngay TRONG luồng thực thi NÀY, mà
KHÔNG phá vỡ mô hình "kéo từng hàng một", trông ra SAO?
::::

::::checkpoint{mastery=0.8}
::::
