---
id: co-so-du-lieu.nhung-loi-tat-cua-byte.hnsw-lite-la-gi
title: "HNSW-lite: tìm gần đúng bằng đồ thị"
summary: "tim_gan_nhat_tham_lam đi TỪ đỉnh xuất phát, mỗi bước nhảy tới HÀNG XÓM gần đích hơn đỉnh hiện tại, DỪNG khi không hàng xóm nào gần hơn nữa — quét CHỈ vài đỉnh, không quét toàn bộ. Nhưng đây là XẤP XỈ: nếu đồ thị thiếu 'lối tắt' nối tới đỉnh thật sự gần nhất, thuật toán dừng ở một CỰC TIỂU CỤC BỘ — bỏ lỡ đỉnh gần hơn nằm ở một nhánh khác."
locale: vi
track: co-so-du-lieu
module: nhung-loi-tat-cua-byte
order: 9
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 11
teaches: [db.hnsw-lite-idea]
requires: [db.graph-traverse-index]
concepts: [db.hnsw-lite-idea]
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
`hang_xom_di` (bài trước) tìm hàng XÓM trực tiếp của một đỉnh. Muốn
tìm đỉnh "gần MỘT đích cho trước NHẤT" trong CẢ một đồ thị, mà
KHÔNG so sánh với TỪNG đỉnh — có cách nào không?
::::

::::explain{#tim-tham-lam}
`tim_gan_nhat_tham_lam` bắt ĐẦU từ một đỉnh XUẤT phát, MỖI bước
kiểm tra CÁC hàng xóm trực tiếp của đỉnh HIỆN tại — nếu CÓ hàng xóm
gần đích HƠN đỉnh hiện tại, NHẢY tới đó; nếu KHÔNG hàng xóm nào gần
hơn, DỪNG lại:

```rust title=readonly
struct Dinh { gia_tri: i64, hang_xom: Vec<usize> }

fn khoang_cach(a: i64, b: i64) -> i64 {
    if a > b { a - b } else { b - a }
}

fn tim_gan_nhat_tham_lam(do_thi: &Vec<Dinh>, bat_dau: usize, dich: i64) -> usize {
    let mut hien_tai = bat_dau;
    let mut khoang_cach_hien_tai = khoang_cach(do_thi[hien_tai].gia_tri, dich);
    loop {
        let mut tot_nhat = hien_tai;
        let mut khoang_cach_tot_nhat = khoang_cach_hien_tai;
        let mut i = 0;
        while i < do_thi[hien_tai].hang_xom.len() {
            let hx = do_thi[hien_tai].hang_xom[i];
            let kc = khoang_cach(do_thi[hx].gia_tri, dich);
            if kc < khoang_cach_tot_nhat {
                tot_nhat = hx;
                khoang_cach_tot_nhat = kc;
            }
            i = 1 + i;
        }
        if tot_nhat == hien_tai {
            return hien_tai;
        }
        hien_tai = tot_nhat;
        khoang_cach_hien_tai = khoang_cach_tot_nhat;
    }
}

fn main() {
    let do_thi: Vec<Dinh> = vec![
        Dinh { gia_tri: 0, hang_xom: vec![1] },
        Dinh { gia_tri: 30, hang_xom: vec![0, 2] },
        Dinh { gia_tri: 45, hang_xom: vec![1] },
        Dinh { gia_tri: 100, hang_xom: vec![4] },
        Dinh { gia_tri: 48, hang_xom: vec![3] },
    ];
    let dich = 50;
    let ket_qua = tim_gan_nhat_tham_lam(&do_thi, 0, dich);
    println!("{}", ket_qua);
    println!("{}", do_thi[ket_qua].gia_tri);
}
```

```text title=readonly
2
45
```

Bắt đầu Ở đỉnh `0` (giá trị `0`, cách đích `50` LÀ `50`). Hàng xóm
CỦA nó CHỈ có đỉnh `1` (giá trị `30`, cách đích `20`) — gần HƠN, NHẢY
tới `1`. Hàng xóm của `1` LÀ đỉnh `0` (xa hơn) VÀ đỉnh `2` (giá trị
`45`, cách đích `5`) — gần hơn, NHẢY tới `2`. Hàng xóm của `2` chỉ CÓ
đỉnh `1` (xa hơn `5`) — KHÔNG hàng xóm nào gần hơn, DỪNG Ở đỉnh `2`.
Toàn bộ hành TRÌNH chỉ chạm BA đỉnh (`0 → 1 → 2`), không hề nhìn
TỚI đỉnh `3` hay `4`.
::::

::::example{#xap-xi-khong-phai-dung-tuyet-doi}
Quét TOÀN bộ NĂM đỉnh (không qua đồ thị) để tìm đỉnh gần đích
NHẤT thật sự:

```rust title=readonly
let mut gan_nhat_that: usize = 0;
let mut kc_nho_nhat = khoang_cach(do_thi[0].gia_tri, dich);
let mut j = 0;
while j < do_thi.len() {
    let kc = khoang_cach(do_thi[j].gia_tri, dich);
    if kc < kc_nho_nhat {
        kc_nho_nhat = kc;
        gan_nhat_that = j;
    }
    j = 1 + j;
}
println!("{}", gan_nhat_that);
println!("{}", do_thi[gan_nhat_that].gia_tri);
```

```text title=readonly
4
48
```

Quét HẾT cả năm đỉnh cho THẤY đỉnh `4` (giá trị `48`, cách đích chỉ
`2`) MỚI thật sự LÀ gần nhất — GẦN hơn đỉnh `2` (cách `5`) MÀ
`tim_gan_nhat_tham_lam` tìm được. Đỉnh `4` chỉ nối VỚI đỉnh `3`
(giá trị `100`, xa đích), VÀ đỉnh `3` không nằm TRONG đường đi tham
lam TỪ đỉnh `0` — đồ thị THIẾU một "lối TẮT" từ nhánh `0-1-2` SANG
nhánh `3-4`, nên thuật TOÁN tham lam KHÔNG BAO GIỜ chạm tới đỉnh `4`,
dừng LẠI Ở đỉnh `2` — một CỰC tiểu CỤC bộ, không phải cực tiểu toàn
CỤC.
::::

::::predict{#doan-neu-co-loi-tat commitOnce}
NẾU đồ thị Ở trên CÓ thêm một cạnh NỐI đỉnh `2` VỚI đỉnh `4` (một
"lối tắt" MỚI, hai chiều), chạy LẠI `tim_gan_nhat_tham_lam` từ đỉnh
`0` VỚI cùng đích `50` — kết quả CÓ đổi không?

:::opt{correct}
CÓ — giờ thuật toán CÓ thể đi TIẾP từ đỉnh `2` (cách đích `5`) sang
đỉnh `4` (cách đích `2`, gần HƠN), nên sẽ TRẢ về đỉnh `4`
:::

:::opt
KHÔNG — thuật toán tham lam LUÔN dừng Ở cực tiểu cục BỘ đầu tiên tìm
được, bất kể đồ thị CÓ thêm cạnh nào MỚI hay không
::why
Gần đúng ở việc bạn nhớ ĐÚNG rằng thuật toán THAM lam có THỂ mắc
kẹt Ở cực tiểu cục BỘ — chính XÁC LÀ điều đã xảy ra Ở ví dụ TRÊN.

Chỗ lệch: "mắc KẸT" chỉ xảy ra khi hàng XÓM trực tiếp của đỉnh hiện
TẠI không CÓ ai gần đích hơn — thêm CẠNH `2↔4` nghĩa LÀ đỉnh `2` giờ
CÓ thêm một hàng XÓM mới (`4`, cách đích `2`), GẦN hơn khoảng cách
hiện tại (`5`). Vòng lặp `while i < do_thi[hien_tai].hang_xom.len()`
sẽ THẤY hàng xóm mới NÀY VÀ nhảy tới nó — thuật toán KHÔNG "cố định"
dừng lại, nó chỉ dừng khi ĐÚNG là không CÒN hàng xóm nào tốt hơn.
Thêm đúng MỘT cạnh "lối tắt" đã đủ sửa kết QUẢ Ở ví dụ này.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ý tưởng cốt lõi CỦA HNSW đã thấy: đồ thị VỚI đủ "lối tắt" giúp tìm
gần đúng NHANH. Ráp mọi kỹ thuật của quest NÀY thành một pipeline
NHỎ trông ra sao?
::::

::::reflect{#nghi-lai}
`tim_gan_nhat_tham_lam` LÀ một phiên bản CỰC kỳ rút gọn ("lite") của
ý tưởng ĐỨNG sau HNSW (Hierarchical Navigable Small World) — MỘT
cấu trúc tìm KIẾM gần đúng (approximate nearest NEIGHBOR) dùng THẬT
trong tìm kiếm VECTOR: quét MỘT vài đỉnh THEO đường đi tham lam,
KHÔNG so sánh VỚI toàn bộ tập dữ liệu. Cái GIÁ phải trả đổi LẠI tốc
độ đó LÀ ĐÚNG như tên gọi "approximate" — kết quả có THỂ không phải
đỉnh gần NHẤT thật sự, nếu đồ thị THIẾU những cạnh "lối TẮT" nối
các NHÁNH xa nhau (ví dụ Ở TRÊN). HNSW thật giải quyết vấn đề NÀY
bằng NHIỀU tầng đồ thị (tầng CAO thưa, cạnh DÀI, dùng để nhảy XA
nhanh; tầng THẤP dày, cạnh ngắn, dùng để tinh CHỈNH) — phiên bản
"lite" Ở đây chỉ CÓ MỘT tầng, đủ để thấy sự ĐÁNH đổi cốt lõi: đồ
thị CÀNG thưa "lối tắt", càng dễ mắc kẹt Ở cực tiểu cục BỘ. Toàn bộ
quest — mã hoá khoá, chỉ mục PHỤ, cạnh đồ thị — ráp LẠI thành một
pipeline nhỏ trông RA sao?
::::

::::checkpoint{mastery=0.75}
::::
