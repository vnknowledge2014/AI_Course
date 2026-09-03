---
id: co-so-du-lieu.bo-nao-cua-truy-van.tu-cau-truy-van-ra-ke-hoach
title: "Từ câu truy vấn ra kế hoạch"
summary: "xay_ke_hoach(ctv: &CauTruyVan, chi_so_where: usize) -> Vec<BuocKeHoach> luôn thêm Quet trước, ChonCot sau — CHỈ thêm Loc ở GIỮA nếu ctv.co_where. chi_so_where (chỉ số nút gốc của WHERE trong CayAst) được TÍNH bên ngoài (cay.nut.len()-1 ngay sau parse), không tự xay_ke_hoach đoán."
locale: vi
track: co-so-du-lieu
module: bo-nao-cua-truy-van
order: 6
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 12
teaches: [db.plan-builder]
requires: [db.logical-plan]
concepts: [db.plan-builder]
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
Kế hoạch dựng tay được (bài trước). Xây nó TỰ ĐỘNG TỪ `CauTruyVan`
— kết quả `parse` của q07 — trông ra sao?
::::

::::explain{#xay-ke-hoach}
`xay_ke_hoach` LUÔN thêm `Quet` TRƯỚC, `ChonCot` SAU — CHỈ thêm
`Loc` Ở GIỮA nếu `ctv.co_where`:

```rust title=readonly
fn xay_ke_hoach(ctv: &CauTruyVan, chi_so_where: usize) -> Vec<BuocKeHoach> {
    let mut ke_hoach: Vec<BuocKeHoach> = Vec::new();
    ke_hoach.push(BuocKeHoach::Quet(ctv.bang.clone()));
    if ctv.co_where {
        ke_hoach.push(BuocKeHoach::Loc(chi_so_where));
    }
    ke_hoach.push(BuocKeHoach::ChonCot(ctv.truong.clone()));
    ke_hoach
}

fn main() {
    let ctv1 = CauTruyVan {
        truong: vec![vec!['t','e','n']],
        bang: vec!['n','g','u','o','i'],
        co_where: true,
    };
    let kh1 = xay_ke_hoach(&ctv1, 2);
    println!("{}", kh1.len());
}
```

```text title=readonly
3
```

`ctv: &CauTruyVan` LÀ tham chiếu — đọc `.bang`/`.co_where`/`.truong`
liên TIẾP không hề gặp vấn đề GÌ (tham chiếu bất biến, không phải
GIÁ trị sở hữu — khác hẳn TRƯỜNG hợp đọc nhiều trường của một GIÁ
trị `CauTruyVan` sở hữu TRỰC tiếp Ở nơi GỌI, sẽ thấy Ở bài BOSS).
`chi_so_where` (chỉ số nút GỐC của `WHERE` trong `CayAst`) được
TÍNH bên NGOÀI hàm (`cay.nut.len() - 1` NGAY sau `parse`, q07 bài
12) — `xay_ke_hoach` KHÔNG tự đi TÌM nó.
::::

::::example{#khong-co-where-2-buoc}
Câu truy vấn KHÔNG có `WHERE` — kế hoạch CHỈ có hai bước:

```rust title=readonly
let ctv2 = CauTruyVan {
    truong: vec![vec!['t','e','n']],
    bang: vec!['b','a','n'],
    co_where: false,
};
let kh2 = xay_ke_hoach(&ctv2, 0);
println!("{}", kh2.len());
match &kh2[0] {
    BuocKeHoach::Quet(t) => println!("{:?}", t),
    _ => println!("?"),
}
```

```text title=readonly
2
['b', 'a', 'n']
```

`ctv2.co_where` LÀ `false` — nhánh `if` KHÔNG chạy, `Loc` KHÔNG được
thêm VÀO. `chi_so_where=0` được TRUYỀN vào nhưng HOÀN toàn KHÔNG
dùng tới — một giá trị "GIẢ" hợp lệ khi `co_where=false`, vì nhánh
CẦN nó không hề CHẠY.
::::

::::predict{#doan-thu-tu-buoc commitOnce}
Với `ctv1` (CÓ WHERE) Ở TRÊN, kiểm tra ĐÚNG thứ TỰ ba bước:

```rust
match &kh1[0] {
    BuocKeHoach::Quet(_) => println!("Quet"),
    BuocKeHoach::Loc(_) => println!("Loc"),
    BuocKeHoach::ChonCot(_) => println!("ChonCot"),
}
match &kh1[2] {
    BuocKeHoach::Quet(_) => println!("Quet"),
    BuocKeHoach::Loc(_) => println!("Loc"),
    BuocKeHoach::ChonCot(_) => println!("ChonCot"),
}
```

Hai dòng CUỐI in ra gì?

:::opt{correct}
`Quet` rồi `ChonCot`
:::

:::opt
`Quet` rồi `Loc` — vì `Loc` được thêm SAU CÙNG trong thân hàm nếu
`co_where` đúng, nên phải LÀ bước cuối
::why
Gần đúng ở việc bạn nhớ ĐÚNG thứ TỰ code trong thân `xay_ke_hoach`
— `Loc` ĐÚNG là được thêm SAU `Quet` VỀ mặt code.

Chỗ lệch: NHƯNG SAU `if co_where { push Loc }`, hàm CÒN một dòng
NỮA: `ke_hoach.push(BuocKeHoach::ChonCot(...))` — dòng NÀY LUÔN
chạy, VÀ chạy SAU CÙNG, bất KỂ `co_where` LÀ gì. Với `ctv1` (`co_
where=true`), thứ tự THẬT SỰ LÀ `Quet`(0), `Loc`(1), `ChonCot`(2)
— chỉ số `2` LUÔN LÀ `ChonCot`, không phải `Loc`.
::
:::
::::

::::code{#viet_xay_ke_hoach}
Hoàn thiện `xay_ke_hoach` — thêm bước `Loc` khi `WHERE` tồn tại.

```rust title=starter
struct CauTruyVan { truong: Vec<Vec<char> >, bang: Vec<char>, co_where: bool }
enum BuocKeHoach {
    Quet(Vec<char>),
    Loc(usize),
    ChonCot(Vec<Vec<char> >),
}

fn xay_ke_hoach(ctv: &CauTruyVan, chi_so_where: usize) -> Vec<BuocKeHoach> {
    let mut ke_hoach: Vec<BuocKeHoach> = Vec::new();
    ke_hoach.push(BuocKeHoach::Quet(ctv.bang.clone()));
    if ctv.co_where {
        ___
    }
    ke_hoach.push(BuocKeHoach::ChonCot(ctv.truong.clone()));
    ke_hoach
}

fn main() {
    let ctv = CauTruyVan {
        truong: vec![vec!['t','e','n']],
        bang: vec!['n','g','u','o','i'],
        co_where: true,
    };
    let kh = xay_ke_hoach(&ctv, 2);
    println!("{}", kh.len());
}
```

```rust title=solution
struct CauTruyVan { truong: Vec<Vec<char> >, bang: Vec<char>, co_where: bool }
enum BuocKeHoach {
    Quet(Vec<char>),
    Loc(usize),
    ChonCot(Vec<Vec<char> >),
}

fn xay_ke_hoach(ctv: &CauTruyVan, chi_so_where: usize) -> Vec<BuocKeHoach> {
    let mut ke_hoach: Vec<BuocKeHoach> = Vec::new();
    ke_hoach.push(BuocKeHoach::Quet(ctv.bang.clone()));
    if ctv.co_where {
        ke_hoach.push(BuocKeHoach::Loc(chi_so_where));
    }
    ke_hoach.push(BuocKeHoach::ChonCot(ctv.truong.clone()));
    ke_hoach
}

fn main() {
    let ctv = CauTruyVan {
        truong: vec![vec!['t','e','n']],
        bang: vec!['n','g','u','o','i'],
        co_where: true,
    };
    let kh = xay_ke_hoach(&ctv, 2);
    println!("{}", kh.len());
}
```

```rust title=test
struct CauTruyVan { truong: Vec<Vec<char> >, bang: Vec<char>, co_where: bool }
enum BuocKeHoach {
    Quet(Vec<char>),
    Loc(usize),
    ChonCot(Vec<Vec<char> >),
}

fn main() {
    let ctv1 = CauTruyVan { truong: vec![vec!['t','e','n']], bang: vec!['n','g','u','o','i'], co_where: true };
    let kh1 = xay_ke_hoach(&ctv1, 2);
    println!("{}", kh1.len());
    assert_eq!(kh1.len(), 3, "co WHERE -- 3 buoc");
    match &kh1[1] {
        BuocKeHoach::Loc(i) => assert_eq!(*i, 2, "chi so loc phai la 2"),
        _ => panic!("buoc thu 2 phai la Loc"),
    }

    let ctv2 = CauTruyVan { truong: vec![vec!['t','e','n']], bang: vec!['b','a','n'], co_where: false };
    let kh2 = xay_ke_hoach(&ctv2, 0);
    assert_eq!(kh2.len(), 2, "khong WHERE -- 2 buoc");
    match &kh2[1] {
        BuocKeHoach::ChonCot(_) => {},
        _ => panic!("buoc thu 2 (khong WHERE) phai la ChonCot ngay"),
    }

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Trong nhanh if, day BuocKeHoach::Loc(chi_so_where) vao ke_hoach -- mot dong."
- kind: strategy
  body: "ke_hoach.push(BuocKeHoach::Loc(chi_so_where));"
- kind: one-line
  body: "ke_hoach.push(BuocKeHoach::Loc(chi_so_where));"
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
Kế hoạch xây tự động được rồi. Nhưng THỨ TỰ ba bước có QUAN trọng
không — lọc TRƯỚC hay chọn cột TRƯỚC?
::::

::::reflect{#nghi-lai}
`xay_ke_hoach` biến MỘT `CauTruyVan` (dữ liệu THỤ động, kết quả
parse) thành MỘT `KeHoach` (danh sách HÀNH động) — bước ĐẦU tiên
biến "hiểu câu truy vấn NÓI gì" thành "biết PHẢI làm gì, THEO thứ
tự nào". Thứ TỰ trong code (`Quet` → `Loc` → `ChonCot`) trông NHƯ
MỘT lựa chọn TUỲ ý — nhưng LIỆU đổi thứ tự CÓ ảnh hưởng gì tới KẾT
quả cuối CÙNG không? Câu trả LỜI KHÔNG đơn giản như VẺ ngoài.
::::

::::checkpoint{mastery=0.8}
::::
