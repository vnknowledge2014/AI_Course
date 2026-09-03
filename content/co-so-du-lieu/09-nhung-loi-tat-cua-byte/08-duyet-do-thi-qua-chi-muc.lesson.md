---
id: co-so-du-lieu.nhung-loi-tat-cua-byte.duyet-do-thi-qua-chi-muc
title: "Duyệt đồ thị qua chỉ mục"
summary: "hang_xom_di(canh, chi_muc_di, dinh) tìm mọi hàng xóm 'đi' của dinh: tim_theo_khoa(chi_muc_di, ma_hoa_so(dinh)) trả về CHỈ SỐ CẠNH (không phải id đỉnh!) khớp khoá, rồi canh[cs[i]].den mới là đỉnh đích thật. Nhầm chỉ số cạnh với id đỉnh là lỗi phổ biến nhất khi duyệt qua chỉ mục kiểu này."
locale: vi
track: co-so-du-lieu
module: nhung-loi-tat-cua-byte
order: 8
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 12
teaches: [db.graph-traverse-index]
requires: [db.graph-edge-as-index]
concepts: [db.graph-traverse-index]
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
Hai chỉ mục "đi" VÀ "đến" (bài trước) đã xây xong. Tìm HÀNG xóm của
một đỉnh — TỪ một chỉ mục — trông ra sao?
::::

::::explain{#tim-theo-khoa-chinh-xac}
`tim_theo_khoa` giống HỆT `trong_khoang` (bài 6), CHỈ đổi điều kiện
TỪ "trong khoảng" sang "khớp CHÍNH xác" (`==` thay VÌ `>=`/`<=`) —
kết QUẢ trả về LÀ danh sách CHỈ SỐ CẠNH (vị trí TRONG arena `canh`),
KHÔNG phải id đỉnh:

```rust title=readonly
struct Canh { tu: usize, den: usize }

fn ma_hoa_so(n: i64) -> Vec<u8> {
    let cao: u8 = (n / 256) as u8;
    let thap: u8 = (n % 256) as u8;
    let mut ra: Vec<u8> = Vec::new();
    ra.push(cao);
    ra.push(thap);
    ra
}

struct MucChiMuc { khoa: Vec<u8>, vi_tri: usize }

fn xay_chi_muc_di(canh: &Vec<Canh>) -> Vec<MucChiMuc> {
    let mut chi_muc: Vec<MucChiMuc> = Vec::new();
    let mut i = 0;
    while i < canh.len() {
        chi_muc.push(MucChiMuc { khoa: ma_hoa_so(canh[i].tu as i64), vi_tri: i });
        i = 1 + i;
    }
    chi_muc
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

fn main() {
    let canh: Vec<Canh> = vec![
        Canh{tu:0,den:1},
        Canh{tu:0,den:2},
        Canh{tu:1,den:3},
        Canh{tu:2,den:3},
    ];
    let chi_muc_di = xay_chi_muc_di(&canh);
    let cs = tim_theo_khoa(&chi_muc_di, &ma_hoa_so(0));
    println!("{:?}", cs);
}
```

```text title=readonly
[0, 1]
```

`cs` LÀ `[0, 1]` — CHỈ SỐ của hai cạnh TRONG arena `canh` (`canh[0]`
VÀ `canh[1]`), KHÔNG phải "đỉnh 0 VÀ đỉnh 1". Cả hai cạnh ĐÓ CÓ
`tu=0` (đỉnh xuất PHÁT LÀ `0`), nên khoá `ma_hoa_so(0)` khớp CẢ hai.
Muốn biết đỉnh ĐÍCH thật sự, PHẢI tra ngược VÀO `canh[cs[i]].den`.
::::

::::example{#nham-chi-so-voi-dinh}
Nếu NHẦM chỉ số cạnh VỚI id đỉnh (bỏ qua bước tra ngược VÀO
`canh`), kết quả sẽ SAI:

```rust title=readonly
println!("{}", canh[cs[0]].den);
println!("{}", canh[cs[1]].den);
```

```text title=readonly
1
2
```

Đỉnh ĐÍCH thật của hai cạnh XUẤT phát từ `0` LÀ `1` VÀ `2` — KHÁC
hoàn toàn với `cs = [0, 1]` (chỉ số CẠNH). Coi `cs[0]=0` LÀ "đỉnh
đích 0" sẽ SAI hoàn toàn — đây LÀ lỗi phổ BIẾN nhất khi duyệt qua
loại chỉ mục NÀY: `tim_theo_khoa` trả VỀ vị trí TRONG cấu trúc gốc
(Ở đây LÀ `canh`), không phải bản THÂN giá trị cần tìm.
::::

::::predict{#doan-hang-xom-dinh-3 commitOnce}
CÙNG đồ thị bốn cạnh Ở trên (`0→1`, `0→2`, `1→3`, `2→3`). Gọi
`tim_theo_khoa(&chi_muc_di, &ma_hoa_so(3))` — danh sách CHỈ SỐ cạnh
trả về CÓ độ dài BAO nhiêu?

:::opt
`2` — vì đỉnh `3` LÀ đích của HAI cạnh (`1→3` VÀ `2→3`)
::why
Gần đúng ở việc bạn đếm ĐÚNG rằng đỉnh `3` LÀ đích của hai cạnh — MỘT
quan sát THẬT về đồ thị.

Chỗ lệch: `chi_muc_di` khoá THEO `tu` (đỉnh XUẤT phát), KHÔNG phải
`den` — `ma_hoa_so(3)` chỉ khớp những cạnh CÓ `tu=3`. Trong bốn
cạnh, KHÔNG cạnh nào có `tu=3` (đỉnh `3` chỉ xuất hiện Ở VỊ trí
`den`) — `tim_theo_khoa` trên `chi_muc_di` VỚI khoá `3` trả VỀ danh
sách RỖNG. Muốn tìm "MỌI cạnh ĐẾN đỉnh 3", phải tra TRÊN `chi_muc
_den` (khoá theo `den`), không phải `chi_muc_di`.
::
:::

:::opt{correct}
`0` — `chi_muc_di` khoá theo `tu`, VÀ không cạnh nào có `tu=3`
:::
::::

::::code{#viet_hang_xom_di}
Hoàn thiện `hang_xom_di` — với MỖI chỉ số cạnh tìm được, lấy đúng
đỉnh ĐÍCH của cạnh đó (`canh[...].den`) đẩy VÀO kết quả.

```rust title=starter
struct Canh { tu: usize, den: usize }

fn ma_hoa_so(n: i64) -> Vec<u8> {
    let cao: u8 = (n / 256) as u8;
    let thap: u8 = (n % 256) as u8;
    let mut ra: Vec<u8> = Vec::new();
    ra.push(cao);
    ra.push(thap);
    ra
}

struct MucChiMuc { khoa: Vec<u8>, vi_tri: usize }

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
        ___
        i = 1 + i;
    }
    ket_qua
}

fn main() {
    let canh: Vec<Canh> = vec![
        Canh{tu:0,den:1},
        Canh{tu:0,den:2},
        Canh{tu:1,den:3},
        Canh{tu:2,den:3},
    ];
    let chi_muc_di = xay_chi_muc_di(&canh);
    let hx_0 = hang_xom_di(&canh, &chi_muc_di, 0);
    println!("{:?}", hx_0);
}
```

```rust title=solution
struct Canh { tu: usize, den: usize }

fn ma_hoa_so(n: i64) -> Vec<u8> {
    let cao: u8 = (n / 256) as u8;
    let thap: u8 = (n % 256) as u8;
    let mut ra: Vec<u8> = Vec::new();
    ra.push(cao);
    ra.push(thap);
    ra
}

struct MucChiMuc { khoa: Vec<u8>, vi_tri: usize }

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

fn main() {
    let canh: Vec<Canh> = vec![
        Canh{tu:0,den:1},
        Canh{tu:0,den:2},
        Canh{tu:1,den:3},
        Canh{tu:2,den:3},
    ];
    let chi_muc_di = xay_chi_muc_di(&canh);
    let hx_0 = hang_xom_di(&canh, &chi_muc_di, 0);
    println!("{:?}", hx_0);
}
```

```rust title=test
struct Canh { tu: usize, den: usize }

fn ma_hoa_so(n: i64) -> Vec<u8> {
    let cao: u8 = (n / 256) as u8;
    let thap: u8 = (n % 256) as u8;
    let mut ra: Vec<u8> = Vec::new();
    ra.push(cao);
    ra.push(thap);
    ra
}

struct MucChiMuc { khoa: Vec<u8>, vi_tri: usize }

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

fn main() {
    let canh: Vec<Canh> = vec![
        Canh{tu:0,den:1},
        Canh{tu:0,den:2},
        Canh{tu:1,den:3},
        Canh{tu:2,den:3},
    ];
    let chi_muc_di = xay_chi_muc_di(&canh);
    let hx_0 = hang_xom_di(&canh, &chi_muc_di, 0);
    println!("{:?}", hx_0);
    assert_eq!(hx_0.len(), 2, "dinh 0 phai co dung 2 hang xom di");
    assert_eq!(hx_0[0], 1, "hang xom dau tien phai la dinh 1");
    assert_eq!(hx_0[1], 2, "hang xom thu hai phai la dinh 2");

    let hx_1 = hang_xom_di(&canh, &chi_muc_di, 1);
    assert_eq!(hx_1.len(), 1, "dinh 1 chi co 1 hang xom di");
    assert_eq!(hx_1[0], 3, "hang xom cua dinh 1 phai la dinh 3");

    let hx_3 = hang_xom_di(&canh, &chi_muc_di, 3);
    assert_eq!(hx_3.len(), 0, "dinh 3 khong co hang xom di nao");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "Voi moi chi so canh trong cs, day dung canh[cs[i]].den (dinh dich thuc su, khong phai cs[i]) vao ket_qua -- mot dong."
- kind: strategy
  body: "ket_qua.push(canh[cs[i]].den);"
- kind: one-line
  body: "ket_qua.push(canh[cs[i]].den);"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "[1, 2]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Duyệt đồ thị QUA chỉ mục — xong. Một cấu trúc TÌM kiếm gần đúng
(approximate) TRÊN một đồ thị như vậy, để TÌM "hàng xóm gần NHẤT" mà
không quét HẾT mọi đỉnh, trông ra sao?
::::

::::reflect{#nghi-lai}
`hang_xom_di` khép LẠI vòng "cạnh LÀ hai mục chỉ mục" (bài 7): xây
chỉ mục (`xay_chi_muc_di`), tra CHÍNH xác (`tim_theo_khoa`), RỒI
DỊCH kết quả (chỉ số CẠNH) trở lại thành GIÁ trị thật cần (đỉnh
ĐÍCH, `canh[...].den`) — bước DỊCH cuối CÙNG này LÀ chỗ dễ nhầm
NHẤT, VÀ LÀ lý do `tim_theo_khoa`/`trong_khoang` LUÔN trả về vị trí
TRONG cấu trúc gốc, không phải giá TRỊ cuối cùng — GIỮ chúng tách
biệt cho phép TÁI dùng CÙNG một hàm tra cứu CHO nhiều loại dữ liệu
khác nhau (giá trị TRƯỜNG Ở bài 6, đỉnh đồ thị Ở bài NÀY). Duyệt
CHÍNH xác từng hàng xóm đã xong — tìm GẦN đúng, không quét HẾT mọi
đỉnh, trông RA sao?
::::

::::checkpoint{mastery=0.8}
::::
