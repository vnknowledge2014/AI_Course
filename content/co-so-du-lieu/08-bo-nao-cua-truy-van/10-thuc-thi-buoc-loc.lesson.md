---
id: co-so-du-lieu.bo-nao-cua-truy-van.thuc-thi-buoc-loc
title: "Thực thi bước Lọc"
summary: "hang_qua_duoc(cay, chi_so_dk: &usize, hang, co_loc: &bool) -> bool: không WHERE thì luôn qua, có WHERE thì gọi danh_gia. tiep_loc lặp buoc_quet tới khi tìm hàng qua được hoặc hết bảng — cài đúng ngữ nghĩa Loc mà KHÔNG tính trước toàn bộ kết quả. Tham số bool/usize (không phải reference) bị dùng lại qua nhiều vòng lặp báo lỗi sai — phải khai &bool/&usize."
locale: vi
track: co-so-du-lieu
module: bo-nao-cua-truy-van
order: 10
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 13
teaches: [db.executor-filter-step]
requires: [db.executor-scan-step]
concepts: [db.executor-filter-step]
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
`buoc_quet` (bài trước) trả VỀ MỌI hàng, KHÔNG hề quan tâm `WHERE`.
Ghép điều kiện LỌC vào NGAY trong luồng "kéo TỪNG hàng" — thế nào?
::::

::::explain{#ham-loc}
`hang_qua_duoc` trả LỜI đúng MỘT câu hỏi: hàng NÀY có nên GIỮ lại
không? `tiep_loc` LẶP `buoc_quet` tới khi TÌM được một hàng qua
được, HOẶC hết bảng:

```rust title=readonly
fn hang_qua_duoc(cay: &CayAst, chi_so_dk: &usize, hang: &Vec<Truong>, co_loc: &bool) -> bool {
    if !*co_loc {
        true
    } else {
        danh_gia(cay, *chi_so_dk, hang)
    }
}

fn tiep_loc(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize, cay: &CayAst, chi_so_dk: &usize, co_loc: &bool) -> Option<Vec<Truong> > {
    loop {
        let r = buoc_quet(bang, vi_tri);
        match &r {
            None => return None,
            Some(hang) => {
                if hang_qua_duoc(cay, chi_so_dk, hang, co_loc) {
                    return r;
                }
            }
        }
    }
}
```

`co_loc: &bool` VÀ `chi_so_dk: &usize` LÀ THAM CHIẾU, KHÔNG phải
`bool`/`usize` trực tiếp — bắt BUỘC, vì `tiep_loc` TRUYỀN chúng vào
`hang_qua_duoc` bên TRONG một vòng `loop`; MỘT tham SỐ kiểu giá trị
(không phải tham chiếu) bị dùng LẠI qua NHIỀU vòng lặp NHƯ vậy sẽ
bị BÁO lỗi sai ("chưa kiểm được VIỆC chuyển RA khỏi vòng lặp") dù
code hoàn toàn ĐÚNG — dùng tham chiếu LÀ cách né hoàn TOÀN vấn đề
NÀY (đã thấy Ở dạng khác VỚI tham chiếu bất biến TRONG q07). Khi
hàng KHÔNG qua được, `loop` đơn giản LẶP lại — gọi `buoc_quet` tiếp
để LẤY hàng kế tiếp, KHÔNG hề dừng lại.
::::

::::example{#loc-bo-qua-hang}
Bảng có hàng ĐẦU (`tuoi=15`) không thoả `tuoi > 18` — `tiep_loc`
tự động BỎ QUA nó, trả VỀ thẳng hàng kế TIẾP thoả điều kiện:

```rust title=readonly
let bang: Vec<Vec<Truong> > = vec![
    vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
    vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}],
];
let mut vt: usize = 0;
let co_loc = true;
let r1 = tiep_loc(&bang, &mut vt, &cay, &goc, &co_loc);
match &r1 {
    Some(h) => println!("{}", h[0].gia_tri),
    None => println!("het"),
}
```

```text title=readonly
20
```

Người GỌI `tiep_loc` KHÔNG hề biết (VÀ không cần biết) rằng BÊN
trong NÓ đã GỌI `buoc_quet` HAI lần (một lần cho `15` — bị LOẠI, một
lần cho `20` — được GIỮ) trước khi trả VỀ. Việc "bỏ qua hàng KHÔNG
đạt" nằm GỌN trong `loop`, KHÔNG rò rỉ ra NGOÀI.
::::

::::predict{#doan-khong-loc commitOnce}
CÙNG bảng Ở trên, nhưng `co_loc = false` (câu truy vấn KHÔNG có
`WHERE`). Lần gọi `tiep_loc` ĐẦU tiên trả về hàng NÀO?

:::opt{correct}
`15` — hàng ĐẦU tiên, vì không CÓ `WHERE` thì MỌI hàng đều qua được
:::

:::opt
`20` — vì hàm VẪN gọi `danh_gia` để kiểm TRA `tuoi > 18`, chỉ là
KHÔNG dùng kết quả để LOẠI hàng
::why
Gần đúng ở việc bạn nghĩ TỚI `danh_gia` như một bước LUÔN chạy —
hợp lý nếu hình dung `hang_qua_duoc` "luôn kiểm TRA cho chắc".

Chỗ lệch: nhánh `if !*co_loc { true }` trả VỀ NGAY `true` — KHÔNG hề
gọi `danh_gia` chút NÀO khi `co_loc` LÀ `false`. Đây chính LÀ ý
nghĩa của "không CÓ `WHERE`": hàng ĐẦU tiên (`15`) qua được NGAY
LẬP tức, không CẦN — và không HỀ — đánh giá điều kiện NÀO cả.
::
:::
::::

::::code{#viet_hang_qua_duoc}
Hoàn thiện `hang_qua_duoc` — khi CÓ `WHERE` (`*co_loc` đúng), hàng
qua được KHI VÀ CHỈ KHI `danh_gia` trả về `true`.

```rust title=starter
enum Token {
    So(i64), DinhDanh(Vec<char>), Chon, Tu, ODau, Va, Hoac, Phay, Bang, LonHon, NhoHon, KetThuc,
}
enum NoAst { SoSanh(Vec<char>, char, i64), VaNut(usize, usize), HoacNut(usize, usize) }
struct CayAst { nut: Vec<NoAst> }

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

fn danh_gia(cay: &CayAst, i: usize, hang: &Vec<Truong>) -> bool {
    match &cay.nut[i] {
        NoAst::SoSanh(ten, dau, so) => danh_gia_so_sanh(hang, ten, *dau, *so),
        NoAst::VaNut(trai, phai) => danh_gia(cay, *trai, hang) && danh_gia(cay, *phai, hang),
        NoAst::HoacNut(trai, phai) => danh_gia(cay, *trai, hang) || danh_gia(cay, *phai, hang),
    }
}

fn buoc_quet(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize) -> Option<Vec<Truong> > {
    if *vi_tri >= bang.len() { return None; }
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

fn hang_qua_duoc(cay: &CayAst, chi_so_dk: &usize, hang: &Vec<Truong>, co_loc: &bool) -> bool {
    if !*co_loc {
        true
    } else {
        ___
    }
}

fn tiep_loc(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize, cay: &CayAst, chi_so_dk: &usize, co_loc: &bool) -> Option<Vec<Truong> > {
    loop {
        let r = buoc_quet(bang, vi_tri);
        match &r {
            None => return None,
            Some(hang) => {
                if hang_qua_duoc(cay, chi_so_dk, hang, co_loc) {
                    return r;
                }
            }
        }
    }
}

fn main() {
    let mut cay = CayAst { nut: Vec::new() };
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    cay.nut.push(NoAst::SoSanh(tuoi, '>', 18));
    let goc = (cay.nut.len() - 1) as usize;
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}],
    ];
    let mut vt: usize = 0;
    let co_loc = true;
    let r1 = tiep_loc(&bang, &mut vt, &cay, &goc, &co_loc);
    match &r1 {
        Some(h) => println!("{}", h[0].gia_tri),
        None => println!("het"),
    }
}
```

```rust title=solution
enum Token {
    So(i64), DinhDanh(Vec<char>), Chon, Tu, ODau, Va, Hoac, Phay, Bang, LonHon, NhoHon, KetThuc,
}
enum NoAst { SoSanh(Vec<char>, char, i64), VaNut(usize, usize), HoacNut(usize, usize) }
struct CayAst { nut: Vec<NoAst> }

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

fn danh_gia(cay: &CayAst, i: usize, hang: &Vec<Truong>) -> bool {
    match &cay.nut[i] {
        NoAst::SoSanh(ten, dau, so) => danh_gia_so_sanh(hang, ten, *dau, *so),
        NoAst::VaNut(trai, phai) => danh_gia(cay, *trai, hang) && danh_gia(cay, *phai, hang),
        NoAst::HoacNut(trai, phai) => danh_gia(cay, *trai, hang) || danh_gia(cay, *phai, hang),
    }
}

fn buoc_quet(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize) -> Option<Vec<Truong> > {
    if *vi_tri >= bang.len() { return None; }
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

fn hang_qua_duoc(cay: &CayAst, chi_so_dk: &usize, hang: &Vec<Truong>, co_loc: &bool) -> bool {
    if !*co_loc {
        true
    } else {
        danh_gia(cay, *chi_so_dk, hang)
    }
}

fn tiep_loc(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize, cay: &CayAst, chi_so_dk: &usize, co_loc: &bool) -> Option<Vec<Truong> > {
    loop {
        let r = buoc_quet(bang, vi_tri);
        match &r {
            None => return None,
            Some(hang) => {
                if hang_qua_duoc(cay, chi_so_dk, hang, co_loc) {
                    return r;
                }
            }
        }
    }
}

fn main() {
    let mut cay = CayAst { nut: Vec::new() };
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    cay.nut.push(NoAst::SoSanh(tuoi, '>', 18));
    let goc = (cay.nut.len() - 1) as usize;
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}],
    ];
    let mut vt: usize = 0;
    let co_loc = true;
    let r1 = tiep_loc(&bang, &mut vt, &cay, &goc, &co_loc);
    match &r1 {
        Some(h) => println!("{}", h[0].gia_tri),
        None => println!("het"),
    }
}
```

```rust title=test
enum Token {
    So(i64), DinhDanh(Vec<char>), Chon, Tu, ODau, Va, Hoac, Phay, Bang, LonHon, NhoHon, KetThuc,
}
enum NoAst { SoSanh(Vec<char>, char, i64), VaNut(usize, usize), HoacNut(usize, usize) }
struct CayAst { nut: Vec<NoAst> }

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

fn danh_gia(cay: &CayAst, i: usize, hang: &Vec<Truong>) -> bool {
    match &cay.nut[i] {
        NoAst::SoSanh(ten, dau, so) => danh_gia_so_sanh(hang, ten, *dau, *so),
        NoAst::VaNut(trai, phai) => danh_gia(cay, *trai, hang) && danh_gia(cay, *phai, hang),
        NoAst::HoacNut(trai, phai) => danh_gia(cay, *trai, hang) || danh_gia(cay, *phai, hang),
    }
}

fn buoc_quet(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize) -> Option<Vec<Truong> > {
    if *vi_tri >= bang.len() { return None; }
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
    let mut cay = CayAst { nut: Vec::new() };
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    cay.nut.push(NoAst::SoSanh(tuoi, '>', 18));
    let goc = (cay.nut.len() - 1) as usize;
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
    ];
    let mut vt: usize = 0;
    let co_loc = true;
    let r1 = tiep_loc(&bang, &mut vt, &cay, &goc, &co_loc);
    println!("{}", r1.is_some());
    match &r1 {
        Some(h) => assert_eq!(h[0].gia_tri, 20, "hang qua loc dau tien phai la 20"),
        None => panic!("phai con hang"),
    }
    let r2 = tiep_loc(&bang, &mut vt, &cay, &goc, &co_loc);
    match &r2 {
        Some(h) => assert_eq!(h[0].gia_tri, 25, "hang qua loc thu hai phai la 25"),
        None => panic!("phai con hang"),
    }
    let r3 = tiep_loc(&bang, &mut vt, &cay, &goc, &co_loc);
    match &r3 {
        Some(_) => panic!("phai het sau khi loc het bang"),
        None => {},
    }

    let co_loc2 = false;
    let mut vt2: usize = 0;
    let r4 = tiep_loc(&bang, &mut vt2, &cay, &goc, &co_loc2);
    match &r4 {
        Some(h) => assert_eq!(h[0].gia_tri, 15, "khong loc -- hang dau tien van la 15"),
        None => panic!("phai con hang"),
    }
    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Nhanh else: goi danh_gia tren cay, tai chi so *chi_so_dk, tren hang -- mot dong, khong ngoac nhon."
- kind: strategy
  body: "danh_gia(cay, *chi_so_dk, hang)"
- kind: one-line
  body: "danh_gia(cay, *chi_so_dk, hang)"
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
Quét, rồi lọc — cả hai chạy TRONG cùng một lần "kéo". Ghép nốt
`ChonCot` vào, thành `tiep` đầy đủ, trông ra sao?
::::

::::reflect{#nghi-lai}
`tiep_loc` KHÔNG hề tách RIÊNG "lọc" thành một bước CHẠY sau "quét"
trên TOÀN bộ bảng — nó GHÉP cả hai VÀO chung một `loop`, gọi
`buoc_quet` LẶP LẠI cho tới khi TÌM đúng một hàng qua được. Đây
chính LÀ pushdown (bài 7) THỂ hiện Ở tầng THỰC thi: lọc xảy RA NGAY
khi vừa quét XONG một hàng, không đợi quét HẾT rồi mới LỌC. `co_loc:
&bool` VÀ `chi_so_dk: &usize` (tham CHIẾU, không phải giá TRỊ trực
tiếp) LÀ cách NÉ lỗi sai khi tái sử DỤNG tham số qua NHIỀU vòng lặp.
Còn THIẾU đúng MỘT bước: `ChonCot` — GIỮ lại những cột `SELECT` yêu
cầu. Ghép NÓ vào, `tiep_loc` trở thành `tiep` đầy ĐỦ, trông RA sao?
::::

::::checkpoint{mastery=0.8}
::::
