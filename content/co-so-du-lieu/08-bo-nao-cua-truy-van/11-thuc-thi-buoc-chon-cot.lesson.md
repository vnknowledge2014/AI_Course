---
id: co-so-du-lieu.bo-nao-cua-truy-van.thuc-thi-buoc-chon-cot
title: "Thực thi bước Chọn cột"
summary: "chon_cot(hang, cot) giữ lại đúng các Truong có ten khớp một tên trong cot — cột KHÔNG tồn tại trong hàng đơn giản không được thêm vào (không tự sinh Truong rỗng). tiep = tiep_loc (bài trước) rồi chon_cot trên kết quả — Quét→Lọc→ChọnCột trọn vẹn trong MỘT lần kéo, không tính trước toàn bộ bảng."
locale: vi
track: co-so-du-lieu
module: bo-nao-cua-truy-van
order: 11
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 12
teaches: [db.executor-project-step]
requires: [db.executor-filter-step]
concepts: [db.executor-project-step]
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
`tiep_loc` (bài trước) trả về CẢ hàng — MỌI cột. `SELECT ten` chỉ
MUỐN đúng MỘT cột. Bước cuối CÙNG của kế hoạch làm việc NÀY thế nào?
::::

::::explain{#ham-chon-cot}
`chon_cot` GIỮ lại đúng những `Truong` có `ten` KHỚP một tên trong
`cot` — DUYỆT qua từng trường của HÀNG, kiểm TRA có mặt trong danh
sách cột CẦN giữ không:

```rust title=readonly
fn chon_cot(hang: &Vec<Truong>, cot: &Vec<Vec<char> >) -> Vec<Truong> {
    let mut ket_qua: Vec<Truong> = Vec::new();
    let mut i = 0;
    while i < hang.len() {
        let mut j = 0;
        let mut giu = false;
        while j < cot.len() {
            if hang[i].ten == cot[j] {
                giu = true;
            }
            j = 1 + j;
        }
        if giu {
            ket_qua.push(Truong { ten: hang[i].ten.clone(), gia_tri: hang[i].gia_tri });
        }
        i = 1 + i;
    }
    ket_qua
}
```

Vòng NGOÀI (`i`) duyệt TỪNG trường CỦA hàng. Vòng TRONG (`j`) kiểm
tra tên trường đó có NẰM trong `cot` không — CÓ THÌ `giu = true`.
Chỉ những trường CÓ `giu = true` mới được SAO chép VÀO `ket_qua`.
::::

::::example{#ghep-thanh-tiep}
`tiep` GHÉP `tiep_loc` (bài trước — Quét + Lọc) VỚI `chon_cot` —
gọi `tiep_loc` TRƯỚC, nếu CÒN hàng thì áp `chon_cot` LÊN kết quả:

```rust title=readonly
fn tiep(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize, cay: &CayAst, chi_so_dk: &usize, co_loc: &bool, cot: &Vec<Vec<char> >) -> Option<Vec<Truong> > {
    let r = tiep_loc(bang, vi_tri, cay, chi_so_dk, co_loc);
    match &r {
        None => None,
        Some(hang) => Some(chon_cot(hang, cot)),
    }
}

fn main() {
    let mut cay = CayAst { nut: Vec::new() };
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    cay.nut.push(NoAst::SoSanh(tuoi, '>', 18));
    let goc = (cay.nut.len() - 1) as usize;

    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','e','n'],gia_tri:200}, Truong{ten:vec!['t','u','o','i'],gia_tri:20}],
        vec![Truong{ten:vec!['t','e','n'],gia_tri:250}, Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
    ];
    let mut vt: usize = 0;
    let co_loc = true;
    let cot: Vec<Vec<char> > = vec![vec!['t','e','n']];
    let r1 = tiep(&bang, &mut vt, &cay, &goc, &co_loc, &cot);
    match &r1 {
        Some(h) => { println!("{}", h.len()); println!("{}", h[0].gia_tri); },
        None => println!("het"),
    }
}
```

```text title=readonly
1
200
```

Bảng CÓ ba hàng (`tuoi` = 15, 20, 25), `WHERE tuoi > 18`, `SELECT
ten`. Hàng đầu (`tuoi=15`) bị `tiep_loc` LOẠI ngay TỪ bên trong —
`tiep` không hề THẤY nó. Hàng thứ hai (`tuoi=20`, `ten=200`) qua
được `tiep_loc`, RỒI `chon_cot` chỉ GIỮ `ten` — kết quả `h.len()`
LÀ `1` (chỉ CÒN một trường), `h[0].gia_tri` LÀ `200`.
::::

::::predict{#doan-cot-khong-ton-tai commitOnce}
`SELECT ten, diem FROM ...` — nhưng HÀNG dữ liệu THỰC tế chỉ CÓ hai
trường `ten` VÀ `tuoi` (KHÔNG hề có `diem`). `cot` truyền VÀO
`chon_cot` LÀ `[ten, diem]`. Kết quả `chon_cot(&hang, &cot).len()`
LÀ bao nhiêu?

:::opt{correct}
`1` — chỉ `ten` được giữ, `diem` không hề xuất hiện trong kết quả
:::

:::opt
`2` — vì `SELECT` yêu cầu HAI cột, `chon_cot` phải trả về ĐỦ hai
trường (`diem` sẽ mang giá trị mặc định)
::why
Gần đúng ở việc bạn nghĩ TỚI `SELECT ten, diem` như một "khuôn CỐ
định" hai CHỖ trống mà kết quả PHẢI khớp đủ — một trực giác hợp lý
nếu tưởng tượng SQL thật LUÔN trả về `NULL` cho cột thiếu.

Chỗ lệch: `chon_cot` KHÔNG hề tạo `Truong` MỚI cho những tên trong
`cot` mà HÀNG không có — nó chỉ DUYỆT `hang` (vòng NGOÀI `while i <
hang.len()`) rồi lọc theo `cot`, không bao GIỜ đi chiều ngược lại
(duyệt `cot` rồi tìm trong `hang`). `diem` không TỒN tại trong
`hang` nên đơn giản KHÔNG có trường NÀO để giữ lại cho nó — kết quả
chỉ CÒN đúng `ten`, độ dài `1`.
::
:::
::::

::::code{#viet_chon_cot}
Hoàn thiện `chon_cot` — khi `giu` LÀ `true`, sao chép trường đó VÀO
`ket_qua`.

```rust title=starter
struct Truong { ten: Vec<char>, gia_tri: i64 }

fn chon_cot(hang: &Vec<Truong>, cot: &Vec<Vec<char> >) -> Vec<Truong> {
    let mut ket_qua: Vec<Truong> = Vec::new();
    let mut i = 0;
    while i < hang.len() {
        let mut j = 0;
        let mut giu = false;
        while j < cot.len() {
            if hang[i].ten == cot[j] {
                giu = true;
            }
            j = 1 + j;
        }
        if giu {
            ___
        }
        i = 1 + i;
    }
    ket_qua
}

fn main() {
    let hang: Vec<Truong> = vec![
        Truong{ten:vec!['t','e','n'],gia_tri:200},
        Truong{ten:vec!['t','u','o','i'],gia_tri:20},
    ];
    let cot: Vec<Vec<char> > = vec![vec!['t','e','n']];
    let ket_qua = chon_cot(&hang, &cot);
    println!("{}", ket_qua.len());
}
```

```rust title=solution
struct Truong { ten: Vec<char>, gia_tri: i64 }

fn chon_cot(hang: &Vec<Truong>, cot: &Vec<Vec<char> >) -> Vec<Truong> {
    let mut ket_qua: Vec<Truong> = Vec::new();
    let mut i = 0;
    while i < hang.len() {
        let mut j = 0;
        let mut giu = false;
        while j < cot.len() {
            if hang[i].ten == cot[j] {
                giu = true;
            }
            j = 1 + j;
        }
        if giu {
            ket_qua.push(Truong { ten: hang[i].ten.clone(), gia_tri: hang[i].gia_tri });
        }
        i = 1 + i;
    }
    ket_qua
}

fn main() {
    let hang: Vec<Truong> = vec![
        Truong{ten:vec!['t','e','n'],gia_tri:200},
        Truong{ten:vec!['t','u','o','i'],gia_tri:20},
    ];
    let cot: Vec<Vec<char> > = vec![vec!['t','e','n']];
    let ket_qua = chon_cot(&hang, &cot);
    println!("{}", ket_qua.len());
}
```

```rust title=test
struct Truong { ten: Vec<char>, gia_tri: i64 }

fn main() {
    let hang: Vec<Truong> = vec![
        Truong{ten:vec!['t','e','n'],gia_tri:200},
        Truong{ten:vec!['t','u','o','i'],gia_tri:20},
    ];
    let cot: Vec<Vec<char> > = vec![vec!['t','e','n']];
    let ket_qua = chon_cot(&hang, &cot);
    println!("{}", ket_qua.len());
    assert_eq!(ket_qua.len(), 1, "chi giu dung 1 cot ten");
    assert_eq!(ket_qua[0].gia_tri, 200, "gia tri cot ten phai la 200");

    let cot2: Vec<Vec<char> > = vec![vec!['t','e','n'], vec!['d','i','e','m']];
    let ket_qua2 = chon_cot(&hang, &cot2);
    assert_eq!(ket_qua2.len(), 1, "cot khong ton tai (diem) khong duoc them vao");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Khi giu la true, sao chep truong nay (ten.clone(), gia_tri) vao ket_qua bang push -- mot dong."
- kind: strategy
  body: "ket_qua.push(Truong { ten: hang[i].ten.clone(), gia_tri: hang[i].gia_tri });"
- kind: one-line
  body: "ket_qua.push(Truong { ten: hang[i].ten.clone(), gia_tri: hang[i].gia_tri });"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Quét, lọc, chọn cột — cả ba GHÉP trong một `tiep`. Chạy TRỌN một
câu truy vấn thật, từ chữ tới KẾT quả cuối cùng — boss cuối chờ sẵn.
::::

::::reflect{#nghi-lai}
`chon_cot` hoàn TẤT bước CUỐI của kế hoạch — VÀ `tiep` (ghép
`tiep_loc` VỚI `chon_cot`) LÀ bản THI HÀNH đầy đủ của CẢ ba bước
`Quet → Loc → ChonCot`, TRỌN trong MỘT lần "kéo" MỘT hàng. Không hề
CÓ bước "tính trước TOÀN bộ bảng RỒI lọc RỒI chọn cột" Ở đâu CẢ —
mỗi lần GỌI `tiep`, đúng MỘT hàng (nếu CÒN) đi TRỌN vẹn qua cả BA
bước trước khi trả VỀ. Từ `Token`/`CayAst`/`CauTruyVan` (q07, phân
TÍCH câu chữ) tới `KeHoach` (bài 5-6, kế hoạch LOGIC) tới `tiep`
(bài NÀY, thi HÀNH thật) — mọi mảnh GHÉP đã sẵn SÀNG. Chạy TRỌN một
câu truy vấn, từ CHUỖI ký tự tới kết QUẢ cuối cùng, trông RA sao?
::::

::::checkpoint{mastery=0.8}
::::
