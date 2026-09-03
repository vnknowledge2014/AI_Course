---
id: co-so-du-lieu.nhung-loi-tat-cua-byte.canh-do-thi-la-hai-muc-chi-muc
title: "Cạnh đồ thị là hai mục chỉ mục"
summary: "Canh{tu, den} (arena) là MỘT cạnh có hướng. Thay vì một cấu trúc đồ thị RIÊNG, mỗi cạnh sinh ra HAI mục MucChiMuc (bài 6, TÁI DÙNG nguyên struct): chỉ mục 'đi' (khoá=mã hoá tu) và chỉ mục 'đến' (khoá=mã hoá den) — cả hai cạnh chỉ dùng LẠI đúng kỹ thuật tra cứu theo khoá đã học, không có cấu trúc đồ thị mới nào."
locale: vi
track: co-so-du-lieu
module: nhung-loi-tat-cua-byte
order: 7
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 10
teaches: [db.graph-edge-as-index]
requires: [db.secondary-index-build]
concepts: [db.graph-edge-as-index]
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
`MucChiMuc{khoa, vi_tri}` ánh xạ (giá trị TRƯỜNG) → (vị trí hàng).
Một CẠNH đồ thị — "đỉnh A nối TỚI đỉnh B" — có phải LÀ một quan hệ
GIỐNG vậy không?
::::

::::explain{#canh-la-arena}
Một cạnh có HƯỚNG LÀ cặp `(tu, den)` — đỉnh XUẤT phát VÀ đỉnh ĐÍCH.
Theo đúng quy ước ĐÃ dùng xuyên suốt track (`docs/decisions/R4-T40b
-su-that-do-duoc.md`: KHÔNG `Box` đệ quy), MỌI cạnh nằm TRONG một
arena — `Vec<Canh>` — đỉnh LÀ chỉ số `usize`, KHÔNG phải con TRỎ:

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

fn main() {
    let canh: Vec<Canh> = vec![
        Canh{tu:0,den:1},
        Canh{tu:0,den:2},
        Canh{tu:1,den:3},
        Canh{tu:2,den:3},
    ];
    let chi_muc_di = xay_chi_muc_di(&canh);
    println!("{}", chi_muc_di.len());
    println!("{:?}", chi_muc_di[0].khoa);
    println!("{}", chi_muc_di[0].vi_tri);
}
```

```text title=readonly
4
[0, 0]
0
```

`xay_chi_muc_di` giống HỆT `xay_chi_muc` (bài 6) — CHỈ đổi khoá TỪ
"giá trị TRƯỜNG" sang "đỉnh XUẤT phát của cạnh" (`canh[i].tu`), VÀ
đổi "vị trí HÀNG" sang "vị trí CẠNH trong arena". `MucChiMuc` KHÔNG
đổi — TÁI dùng nguyên STRUCT.
::::

::::example{#hai-chieu}
MỘT chỉ mục CHỈ trả lời được "đỉnh NÀO nối ĐI từ X" — muốn biết "đỉnh
NÀO nối ĐẾN Y", cần MỘT chỉ mục KHÁC, khoá THEO `den` thay VÌ `tu`:

```rust title=readonly
fn xay_chi_muc_den(canh: &Vec<Canh>) -> Vec<MucChiMuc> {
    let mut chi_muc: Vec<MucChiMuc> = Vec::new();
    let mut i = 0;
    while i < canh.len() {
        chi_muc.push(MucChiMuc { khoa: ma_hoa_so(canh[i].den as i64), vi_tri: i });
        i = 1 + i;
    }
    chi_muc
}

let chi_muc_den = xay_chi_muc_den(&canh);
println!("{}", chi_muc_den.len());
```

```text title=readonly
4
```

`chi_muc_den` cũng CÓ đúng bốn mục — MỘT mục cho MỖI cạnh, y hệt
`chi_muc_di`. MỘT cạnh `(tu, den)` sinh ra ĐÚNG hai mục — MỘT trong
`chi_muc_di` (khoá `tu`), MỘT trong `chi_muc_den` (khoá `den`) —
không CÓ cấu trúc "đồ thị" nào MỚI được xây, chỉ LÀ hai chỉ mục
DÙNG lại CÙNG một kỹ thuật.
::::

::::predict{#doan-so-muc-hai-chieu commitOnce}
Một đồ thị CÓ `10` cạnh. Tổng số MỤC trong `chi_muc_di` CỘNG số mục
trong `chi_muc_den` LÀ bao nhiêu?

:::opt{correct}
`20` — MỖI cạnh góp đúng MỘT mục vào MỖI chỉ mục (`10` vào `chi_muc
_di`, `10` vào `chi_muc_den`)
:::

:::opt
`10` — mỗi cạnh CHỈ cần MỘT mục DUY nhất, dùng chung cho CẢ hai
hướng tra cứu
::why
Gần đúng ở việc bạn nghĩ TỚI "một cạnh, một bản ghi" như CÁCH tiết
kiệm không GIAN hợp lý nhất — đúng LÀ chỉ CẦN một `Canh` gốc trong
arena (`canh[i]`).

Chỗ lệch: NHƯNG mỗi CHỈ mục (`chi_muc_di`, `chi_muc_den`) LÀ một
danh sách RIÊNG, được XÂY bằng cách quét TOÀN bộ `canh` VÀ trích
MỘT thông tin khác nhau (`tu` cho `chi_muc_di`, `den` cho `chi_muc
_den`). `xay_chi_muc_di` đẩy đúng `canh.len()` mục VÀO `chi_muc_di`
— HOÀN toàn ĐỘC lập với `xay_chi_muc_den` cũng đẩy đúng `canh.len()`
mục VÀO `chi_muc_den`. Mười cạnh sinh RA mười mục Ở MỖI chỉ mục,
tổng CỘNG hai mươi.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai chỉ mục — "đi" VÀ "đến" — đã xây xong. Dùng chúng để TÌM hàng
xóm của một đỉnh, theo CẢ hai chiều, trông ra sao?
::::

::::reflect{#nghi-lai}
Ý tưởng cốt LÕI: một cạnh có HƯỚNG LÀ một quan hệ hai VẾ (`tu`,
`den`) — VÀ `MucChiMuc{khoa, vi_tri}` (bài 5-6) ĐÃ đủ tổng quát để
biểu diễn CẢ hai vế đó, MỖI vế MỘT chỉ mục RIÊNG. Đây LÀ "lối tắt"
đúng nghĩa của tên QUEST: không cần một cấu trúc "đồ thị" CHUYÊN
biệt (danh sách kề, ma trận kề) — TÁI dùng đúng kỹ thuật khoá-mã-hoá
+ chỉ mục ĐÃ học, áp dụng hai LẦN theo hai chiều. Có chỉ mục "đi" VÀ
"đến", tìm hàng XÓM của một đỉnh (CẢ hai chiều) trông RA sao?
::::

::::checkpoint{mastery=0.8}
::::
