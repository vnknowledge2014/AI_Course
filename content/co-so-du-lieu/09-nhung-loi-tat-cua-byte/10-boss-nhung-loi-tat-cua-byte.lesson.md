---
id: co-so-du-lieu.nhung-loi-tat-cua-byte.boss-nhung-loi-tat-cua-byte
title: "BOSS — Những lối tắt của Byte"
summary: "Ráp TRỌN quest: ban_be_trong_khoang_tuoi xây chỉ mục PHỤ trên trường tuoi (ma_hoa_so + xay_chi_muc, bài 2/5-6), tìm mọi hàng trong một khoảng tuổi (trong_khoang, bài 6), RỒI với MỖI hàng tìm được, tra chỉ mục 'đi' của đồ thị bạn bè (xay_chi_muc_di + hang_xom_di, bài 7-8) để lấy bạn bè của họ — một pipeline nhỏ kết hợp khoá-mã-hoá + chỉ mục PHỤ + cạnh đồ thị."
locale: vi
track: co-so-du-lieu
module: nhung-loi-tat-cua-byte
order: 10
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.hnsw-lite-idea]
concepts: [db.boss-q09]
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
Mã hoá khoá GIỮ thứ tự, chỉ mục PHỤ, cạnh đồ thị LÀ hai mục chỉ
mục, tìm gần đúng — TẤT cả đã học. Ráp CHÚNG lại thành MỘT pipeline
duy nhất, chạy TRÊN dữ liệu thật, trông ra SAO?
::::

::::explain{#ban-be-trong-khoang-tuoi}
`ban_be_trong_khoang_tuoi` ghép BA kỹ thuật: xây chỉ mục PHỤ trên
`tuoi` (bài 5-6), lọc RA những hàng trong MỘT khoảng tuổi (bài 6),
RỒI với MỖI hàng tìm được, tra chỉ mục "đi" của đồ thị bạn BÈ (bài
7-8) để lấy bạn bè của HỌ:

```rust title=readonly
fn ban_be_trong_khoang_tuoi(bang: &Vec<Vec<Truong> >, ten: &Vec<char>, tu: &Vec<u8>, den: &Vec<u8>, canh: &Vec<Canh>) -> Vec<usize> {
    let chi_muc_tuoi = xay_chi_muc(bang, ten);
    let cac_vi_tri = trong_khoang(&chi_muc_tuoi, tu, den);
    let chi_muc_di = xay_chi_muc_di(canh);
    let mut ket_qua: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < cac_vi_tri.len() {
        let ban_be = hang_xom_di(canh, &chi_muc_di, cac_vi_tri[i]);
        let mut j = 0;
        while j < ban_be.len() {
            ket_qua.push(ban_be[j]);
            j = 1 + j;
        }
        i = 1 + i;
    }
    ket_qua
}

fn main() {
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:40}],
    ];
    let canh: Vec<Canh> = vec![
        Canh{tu:1,den:4},
        Canh{tu:2,den:4},
        Canh{tu:3,den:0},
    ];
    let tu = ma_hoa_so(18);
    let den = ma_hoa_so(27);
    let r = ban_be_trong_khoang_tuoi(&bang, &tuoi, &tu, &den, &canh);
    println!("{:?}", r);
}
```

```text title=readonly
[4, 4]
```

Năm người Ở vị trí `0..4`, TUỔI lần lượt `15, 20, 25, 30, 40`.
Khoảng `[18, 27]` khớp vị TRÍ `1` (tuổi `20`) VÀ vị trí `2` (tuổi
`25`). Đồ thị bạn BÈ CÓ cạnh `1→4` VÀ `2→4` — CẢ hai người trong
khoảng tuổi ĐỀU có bạn LÀ vị trí `4`. Kết quả `[4, 4]` — vị TRÍ `4`
xuất hiện HAI lần, vì CẢ hai người Ở vị trí `1` VÀ `2` đều CÓ đúng
người ĐÓ LÀ bạn.
::::

::::predict{#doan-khoang-khac commitOnce}
CÙNG bảng năm người VÀ đồ thị bạn bè Ở trên, nhưng khoảng tuổi ĐỔI
thành `[28, 35]` (chỉ khớp vị trí `3`, tuổi `30`). Cạnh `3→0` LÀ cạnh
DUY nhất xuất phát từ vị trí `3`. `ban_be_trong_khoang_tuoi` trả VỀ
gì?

:::opt{correct}
`[0]` — chỉ MỘT người (vị trí `3`) khớp khoảng tuổi, VÀ bạn duy
nhất của họ LÀ vị trí `0`
:::

:::opt
Danh sách rỗng — vì vị TRÍ `0` (bạn của vị trí `3`) CÓ tuổi `15`,
nằm NGOÀI khoảng `[28, 35]`, nên bị LOẠI khỏi kết quả
::why
Gần đúng ở việc bạn để Ý đúng rằng tuổi CỦA vị trí `0` (`15`) nằm
NGOÀI khoảng `[28, 35]` — một quan sát THẬT về dữ liệu.

Chỗ lệch: `trong_khoang` (bài 6) chỉ lọc THEO tuổi Ở bước ĐẦU —
XÁC định AI (những vị trí NÀO) nằm trong khoảng, KHÔNG áp dụng lại
điều kiện tuổi lần NỮA cho bạn bè của HỌ. Sau khi vị trí `3` (tuổi
`30`, TRONG khoảng) đã được CHỌN, `hang_xom_di` chỉ đơn thuần LẤY
bạn CỦA họ (vị trí `0`) — bất kể TUỔI của vị trí `0` LÀ bao nhiêu.
Kết quả VẪN LÀ `[0]`, không rỗng.
::
:::
::::

::::code{#viet_boss_q09}
Hoàn thiện `ban_be_trong_khoang_tuoi` — với MỖI vị trí tìm được
TRONG khoảng tuổi, tra chỉ mục "đi" để lấy bạn bè CỦA họ.

```rust title=starter
struct Truong { ten: Vec<char>, gia_tri: i64 }

fn tra_cuu(hang: &Vec<Truong>, ten: &Vec<char>) -> i64 {
    let mut i = 0;
    while i < hang.len() {
        if hang[i].ten == *ten {
            return hang[i].gia_tri;
        }
        i += 1;
    }
    0
}

fn ma_hoa_so(n: i64) -> Vec<u8> {
    let cao: u8 = (n / 256) as u8;
    let thap: u8 = (n % 256) as u8;
    let mut ra: Vec<u8> = Vec::new();
    ra.push(cao);
    ra.push(thap);
    ra
}

struct MucChiMuc { khoa: Vec<u8>, vi_tri: usize }

fn xay_chi_muc(bang: &Vec<Vec<Truong> >, ten: &Vec<char>) -> Vec<MucChiMuc> {
    let mut chi_muc: Vec<MucChiMuc> = Vec::new();
    let mut i = 0;
    while i < bang.len() {
        let gia_tri = tra_cuu(&bang[i], ten);
        chi_muc.push(MucChiMuc { khoa: ma_hoa_so(gia_tri), vi_tri: i });
        i = 1 + i;
    }
    chi_muc
}

fn trong_khoang(chi_muc: &Vec<MucChiMuc>, tu: &Vec<u8>, den: &Vec<u8>) -> Vec<usize> {
    let mut ket_qua: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < chi_muc.len() {
        if chi_muc[i].khoa >= *tu && chi_muc[i].khoa <= *den {
            ket_qua.push(chi_muc[i].vi_tri);
        }
        i = 1 + i;
    }
    ket_qua
}

fn tim_theo_khoa(chi_muc: &Vec<MucChiMuc>, khoa: &Vec<u8>) -> Vec<usize> {
    let mut ket_qua: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < chi_muc.len() {
        if chi_muc[i].khoa == *khoa {
            ket_qua.push(chi_muc[i].vi_tri);
        }
        i = 1 + i;
    }
    ket_qua
}

struct Canh { tu: usize, den: usize }

fn xay_chi_muc_di(canh: &Vec<Canh>) -> Vec<MucChiMuc> {
    let mut chi_muc: Vec<MucChiMuc> = Vec::new();
    let mut i = 0;
    while i < canh.len() {
        chi_muc.push(MucChiMuc { khoa: ma_hoa_so(canh[i].tu as i64), vi_tri: i });
        i = 1 + i;
    }
    chi_muc
}

fn hang_xom_di(canh: &Vec<Canh>, chi_muc_di: &Vec<MucChiMuc>, dinh: usize) -> Vec<usize> {
    let cs = tim_theo_khoa(chi_muc_di, &ma_hoa_so(dinh as i64));
    let mut ket_qua: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        ket_qua.push(canh[cs[i]].den);
        i = 1 + i;
    }
    ket_qua
}

fn ban_be_trong_khoang_tuoi(bang: &Vec<Vec<Truong> >, ten: &Vec<char>, tu: &Vec<u8>, den: &Vec<u8>, canh: &Vec<Canh>) -> Vec<usize> {
    let chi_muc_tuoi = xay_chi_muc(bang, ten);
    let cac_vi_tri = trong_khoang(&chi_muc_tuoi, tu, den);
    let chi_muc_di = xay_chi_muc_di(canh);
    let mut ket_qua: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < cac_vi_tri.len() {
        let ban_be = ___;
        let mut j = 0;
        while j < ban_be.len() {
            ket_qua.push(ban_be[j]);
            j = 1 + j;
        }
        i = 1 + i;
    }
    ket_qua
}

fn main() {
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:40}],
    ];
    let canh: Vec<Canh> = vec![
        Canh{tu:1,den:4},
        Canh{tu:2,den:4},
        Canh{tu:3,den:0},
    ];
    let tu = ma_hoa_so(18);
    let den = ma_hoa_so(27);
    let r = ban_be_trong_khoang_tuoi(&bang, &tuoi, &tu, &den, &canh);
    println!("{:?}", r);
}
```

```rust title=solution
struct Truong { ten: Vec<char>, gia_tri: i64 }

fn tra_cuu(hang: &Vec<Truong>, ten: &Vec<char>) -> i64 {
    let mut i = 0;
    while i < hang.len() {
        if hang[i].ten == *ten {
            return hang[i].gia_tri;
        }
        i += 1;
    }
    0
}

fn ma_hoa_so(n: i64) -> Vec<u8> {
    let cao: u8 = (n / 256) as u8;
    let thap: u8 = (n % 256) as u8;
    let mut ra: Vec<u8> = Vec::new();
    ra.push(cao);
    ra.push(thap);
    ra
}

struct MucChiMuc { khoa: Vec<u8>, vi_tri: usize }

fn xay_chi_muc(bang: &Vec<Vec<Truong> >, ten: &Vec<char>) -> Vec<MucChiMuc> {
    let mut chi_muc: Vec<MucChiMuc> = Vec::new();
    let mut i = 0;
    while i < bang.len() {
        let gia_tri = tra_cuu(&bang[i], ten);
        chi_muc.push(MucChiMuc { khoa: ma_hoa_so(gia_tri), vi_tri: i });
        i = 1 + i;
    }
    chi_muc
}

fn trong_khoang(chi_muc: &Vec<MucChiMuc>, tu: &Vec<u8>, den: &Vec<u8>) -> Vec<usize> {
    let mut ket_qua: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < chi_muc.len() {
        if chi_muc[i].khoa >= *tu && chi_muc[i].khoa <= *den {
            ket_qua.push(chi_muc[i].vi_tri);
        }
        i = 1 + i;
    }
    ket_qua
}

fn tim_theo_khoa(chi_muc: &Vec<MucChiMuc>, khoa: &Vec<u8>) -> Vec<usize> {
    let mut ket_qua: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < chi_muc.len() {
        if chi_muc[i].khoa == *khoa {
            ket_qua.push(chi_muc[i].vi_tri);
        }
        i = 1 + i;
    }
    ket_qua
}

struct Canh { tu: usize, den: usize }

fn xay_chi_muc_di(canh: &Vec<Canh>) -> Vec<MucChiMuc> {
    let mut chi_muc: Vec<MucChiMuc> = Vec::new();
    let mut i = 0;
    while i < canh.len() {
        chi_muc.push(MucChiMuc { khoa: ma_hoa_so(canh[i].tu as i64), vi_tri: i });
        i = 1 + i;
    }
    chi_muc
}

fn hang_xom_di(canh: &Vec<Canh>, chi_muc_di: &Vec<MucChiMuc>, dinh: usize) -> Vec<usize> {
    let cs = tim_theo_khoa(chi_muc_di, &ma_hoa_so(dinh as i64));
    let mut ket_qua: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        ket_qua.push(canh[cs[i]].den);
        i = 1 + i;
    }
    ket_qua
}

fn ban_be_trong_khoang_tuoi(bang: &Vec<Vec<Truong> >, ten: &Vec<char>, tu: &Vec<u8>, den: &Vec<u8>, canh: &Vec<Canh>) -> Vec<usize> {
    let chi_muc_tuoi = xay_chi_muc(bang, ten);
    let cac_vi_tri = trong_khoang(&chi_muc_tuoi, tu, den);
    let chi_muc_di = xay_chi_muc_di(canh);
    let mut ket_qua: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < cac_vi_tri.len() {
        let ban_be = hang_xom_di(canh, &chi_muc_di, cac_vi_tri[i]);
        let mut j = 0;
        while j < ban_be.len() {
            ket_qua.push(ban_be[j]);
            j = 1 + j;
        }
        i = 1 + i;
    }
    ket_qua
}

fn main() {
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:40}],
    ];
    let canh: Vec<Canh> = vec![
        Canh{tu:1,den:4},
        Canh{tu:2,den:4},
        Canh{tu:3,den:0},
    ];
    let tu = ma_hoa_so(18);
    let den = ma_hoa_so(27);
    let r = ban_be_trong_khoang_tuoi(&bang, &tuoi, &tu, &den, &canh);
    println!("{:?}", r);
}
```

```rust title=test
fn main() {
    let tuoi: Vec<char> = vec!['t','u','o','i'];
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:25}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:30}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:40}],
    ];
    let canh: Vec<Canh> = vec![
        Canh{tu:1,den:4},
        Canh{tu:2,den:4},
        Canh{tu:3,den:0},
    ];
    let tu = ma_hoa_so(18);
    let den = ma_hoa_so(27);
    let r = ban_be_trong_khoang_tuoi(&bang, &tuoi, &tu, &den, &canh);
    println!("{:?}", r);
    assert_eq!(r.len(), 2, "phai co dung 2 ban be tim duoc");
    assert_eq!(r[0], 4, "ban be dau tien phai la vi tri 4");
    assert_eq!(r[1], 4, "ban be thu hai cung phai la vi tri 4");

    let tu2 = ma_hoa_so(28);
    let den2 = ma_hoa_so(35);
    let r2 = ban_be_trong_khoang_tuoi(&bang, &tuoi, &tu2, &den2, &canh);
    assert_eq!(r2.len(), 1, "khoang 28..35 chi khop 1 nguoi (vi tri 3)");
    assert_eq!(r2[0], 0, "ban be duy nhat cua vi tri 3 la vi tri 0");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "ban_be la ket qua goi hang_xom_di voi canh, &chi_muc_di, cac_vi_tri[i] -- mot dong."
- kind: strategy
  body: "hang_xom_di(canh, &chi_muc_di, cac_vi_tri[i])"
- kind: one-line
  body: "let ban_be = hang_xom_di(canh, &chi_muc_di, cac_vi_tri[i]);"
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "[4, 4]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mã hoá khoá, chỉ mục phụ, cạnh đồ thị, tìm gần đúng — ráp thành MỘT
pipeline chạy được. Quest "Những lối tắt của Byte" khép LẠI — R6-2 đi
được thêm một quest nữa.
::::

::::reflect{#nghi-lai}
q09 xây MỘT bộ công cụ "lối tắt" — không phải bằng cách thêm cấu
trúc dữ liệu MỚI cho mỗi bài toán, mà bằng cách TÁI dùng đúng một ý
tưởng (khoá MÃ hoá giữ thứ tự → `MucChiMuc{khoa, vi_tri}` → quét-và
-so-sánh trên `Vec<u8>`) BỐN lần khác nhau: giá TRỊ trường (bài 5-6,
range QUERY), đỉnh xuất phát của cạnh (bài 7-8, "đi"), VÀ ngầm định
cả đỉnh ĐÍCH (`chi_muc_den`, bài 7, không cần TỚI trong pipeline
này). `ban_be_trong_khoang_tuoi` (bài NÀY) chỉ LÀ việc GHÉP đúng thứ
tự: lọc trước (dùng chỉ mục GIÁ trị), duyệt sau (dùng chỉ mục ĐỒ
thị) — CHÍNH LÀ pushdown (q08 bài 7) áp dụng lại Ở một NGỮ cảnh
khác. `ma_hoa_so`/`ma_hoa_ghep` (bài 2, 4), `MucChiMuc`/`xay_chi_muc`
/`trong_khoang`/`tim_theo_khoa` (bài 5-8) VÀ ý tưởng đồ thị tìm gần
đúng (bài 9, KHÔNG xuất hiện TRONG code chấm điểm của boss NÀY,
nhưng LÀ hệ quả trực tiếp CỦA chính kỹ thuật chỉ mục vừa xây) —
CỘNG lại LÀ "những lối tắt của Byte": tra cứu NHANH hơn quét toàn
bộ, TRẢ giá bằng việc phải MÃ hoá khoá đúng CÁCH ngay từ ĐẦU.
::::

::::checkpoint{mastery=0.85}
::::
