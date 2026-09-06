---
id: tri-tue-nhan-tao.hybrid-va-rerank.cong-thuc-rrf-day-du
title: "Công thức RRF đầy đủ: hợp nhất NHIỀU xếp hạng thành một"
summary: "rrf_score(d) = tong 1/(k+hang_i(d)) qua MOI he thong xep hang i ma d xuat hien (khong xuat hien = dong gop 0, khong phai phat). Ap dung cho DUNG hai he thong xep_hang_bm25 va xep_hang_cosine (nguyen van tu q8.5d) tren kho 4 doan tu nghi: BM25 xep [2,1,0,3], cosine xep [1,0,2,3] -- hai thu tu KHAC NHAU. RRF (k=1) xep [1,2,0,3] -- MOT thu tu THU BA, khac ca hai, vi doan 1 kha o CA HAI he thong (hang 2 BM25, hang 1 cosine) con doan 2 (hang 1 BM25 nhung hang 3 cosine) tut xuong hang 2."
locale: vi
track: tri-tue-nhan-tao
module: hybrid-va-rerank
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.cong-thuc-rrf-day-du]
requires: [ai.y-tuong-rrf-vi-sao-khong-cong-diem]
concepts: [ai.cong-thuc-rrf-day-du]
gradingMatrix:
  web-chrome: [static, run, tests, output]
  web-firefox: [static, run, tests, output]
  macos: [static, run, tests, output]
  windows: [static, run, tests, output]
  linux: [static, run, tests, output]
  android: [static, run, tests, output]
  ios: [static, run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Bài trước dùng RRF trên đúng HAI hạng cho trước sẵn. Bài này viết công thức
ĐẦY ĐỦ — hợp nhất bất kỳ số hệ thống xếp hạng nào — và áp nó lên hai hệ
thống THẬT: `xep_hang_bm25` và `xep_hang_cosine`.
::::

::::explain{#cong_thuc_day_du}
Công thức RRF đầy đủ cho một tài liệu `d`, hợp nhất qua MỌI hệ thống xếp
hạng mà nó xuất hiện:

```
rrf_score(d) = tổng qua mọi hệ thống i của 1/(k + hang_i(d))
```

`hang_i(d)` là thứ hạng (1-based) của `d` trong hệ thống `i` (dùng
`hang_cua`, q8.5d bài BOSS). Nếu `d` KHÔNG xuất hiện trong một hệ thống nào
đó (ví dụ ngoài top-N của nó), đóng góp của hệ thống đó là `0` — KHÔNG PHẢI
một hình phạt, chỉ đơn giản là hệ thống đó "không có ý kiến" về tài liệu này.
Bài này áp dụng công thức cho ĐÚNG `2` hệ thống — `xep_hang_bm25` và
`xep_hang_cosine` (nguyên văn từ q8.5d bài BOSS, cả hai đều xếp hạng TOÀN BỘ
kho nên không tài liệu nào bị "vắng mặt" ở đây) — rồi xếp lại theo
`rrf_score` giảm dần.

Hai hàm mới: `diem_rrf_mot_doan(chi_so, danh_sach_xep_hang, k)` cộng dồn
`dong_gop_rrf` (bài trước) qua MỌI hệ thống trong `danh_sach_xep_hang`; và
`xep_hang_rrf(kho, danh_sach_xep_hang, k)` tính điểm này cho MỌI tài liệu
trong kho rồi sắp xếp giảm dần — đúng khuôn `xep_hang_bm25`/`xep_hang_cosine`
đã quen thuộc từ q8.5d.
::::

::::example{#hop_nhat_hai_he_thong_that}
Một kho `4` đoạn tự nghĩ, một câu hỏi công nghệ chạm cả `3` mục từ vựng
(`may_tinh`, `phan_mem`, `thuat_toan`) — chạy cả `xep_hang_bm25` VÀ
`xep_hang_cosine`, rồi hợp nhất bằng RRF:

```python title=readonly
import math

TU_VUNG = [
    "may_tinh", "phan_mem", "lap_trinh", "du_lieu", "vi_xu_ly", "ket_noi_mang", "thuat_toan", "ung_dung",
    "mon_an", "cong_thuc", "gia_vi", "nha_bep", "dau_bep", "thuc_pham", "nau_an", "mon_trang_mieng",
]
CUM_TU_GOC = {
    "may_tinh": "may tinh", "phan_mem": "phan mem", "lap_trinh": "lap trinh", "du_lieu": "du lieu",
    "vi_xu_ly": "vi xu ly", "ket_noi_mang": "ket noi mang", "thuat_toan": "thuat toan", "ung_dung": "ung dung",
    "mon_an": "mon an", "cong_thuc": "cong thuc", "gia_vi": "gia vi", "nha_bep": "nha bep",
    "dau_bep": "dau bep", "thuc_pham": "thuc pham", "nau_an": "nau an", "mon_trang_mieng": "mon trang mieng",
}


def chuan_hoa_cum_tu(van_ban, cum_tu_goc):
    kq = van_ban
    for token, cum in cum_tu_goc.items():
        kq = kq.replace(cum, token)
    return kq


def tinh_vector_dem_tu(van_ban, tu_vung, cum_tu_goc):
    vb = chuan_hoa_cum_tu(van_ban.lower(), cum_tu_goc)
    cac_tu = vb.split()
    return [cac_tu.count(tu) for tu in tu_vung]


def tich_vo_huong(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))


def do_dai_vector(v):
    return math.sqrt(sum(x ** 2 for x in v))


def tuong_dong_cosine(v1, v2):
    d1 = do_dai_vector(v1)
    d2 = do_dai_vector(v2)
    if d1 == 0 or d2 == 0:
        return 0.0
    return tich_vo_huong(v1, v2) / (d1 * d2)


def dem_tan_so(tu, van_ban):
    return van_ban.lower().split().count(tu)


def dem_so_doan_chua_tu(tu, kho):
    return sum(1 for doan in kho if tu in doan.lower().split())


def tinh_idf(tu, kho):
    df = dem_so_doan_chua_tu(tu, kho)
    if df == 0:
        return 0.0
    return math.log(len(kho) / df)


def he_so_chuan_hoa_do_dai(do_dai_doan, do_dai_trung_binh, b):
    return 1 - b + b * (do_dai_doan / do_dai_trung_binh)


def diem_bm25_mot_tu(tf, idf_tu, do_dai_doan, do_dai_trung_binh, k1, b):
    chuan_hoa = he_so_chuan_hoa_do_dai(do_dai_doan, do_dai_trung_binh, b)
    tu_so = tf * (k1 + 1)
    mau_so = tf + k1 * chuan_hoa
    return idf_tu * (tu_so / mau_so)


def do_dai_trung_binh(kho):
    return sum(len(doan.lower().split()) for doan in kho) / len(kho)


def diem_bm25_doan(doan, tu_truy_van, kho, k1, b):
    avgdl = do_dai_trung_binh(kho)
    do_dai_doan = len(doan.lower().split())
    tong = 0.0
    for t in tu_truy_van:
        tf_t = dem_tan_so(t, doan)
        if tf_t == 0:
            continue
        tong += diem_bm25_mot_tu(tf_t, tinh_idf(t, kho), do_dai_doan, avgdl, k1, b)
    return tong


def xep_hang_bm25(cau_hoi, kho, k1, b):
    tu_truy_van = cau_hoi.lower().split()
    diem = [diem_bm25_doan(doan, tu_truy_van, kho, k1, b) for doan in kho]
    return sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)


def xep_hang_cosine(cau_hoi, kho, tu_vung, cum_tu_goc):
    v_cau_hoi = tinh_vector_dem_tu(cau_hoi, tu_vung, cum_tu_goc)
    v_kho = [tinh_vector_dem_tu(doan, tu_vung, cum_tu_goc) for doan in kho]
    diem = [tuong_dong_cosine(v_cau_hoi, v) for v in v_kho]
    return sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)


def hang_cua(chi_so_muc_tieu, thu_tu_xep_hang):
    return thu_tu_xep_hang.index(chi_so_muc_tieu) + 1


def dong_gop_rrf(hang, k):
    return 1 / (k + hang)


def diem_rrf_mot_doan(chi_so, danh_sach_xep_hang, k):
    return sum(dong_gop_rrf(hang_cua(chi_so, xep_hang), k) for xep_hang in danh_sach_xep_hang)


def xep_hang_rrf(kho, danh_sach_xep_hang, k):
    diem = [diem_rrf_mot_doan(i, danh_sach_xep_hang, k) for i in range(len(kho))]
    return sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)


KHO = [
    "may tinh giup cong viec hieu qua hon nho thuat toan",
    "may tinh chay phan mem va thuat toan giup cong viec tot hon",
    "phan mem chay tot phan mem on dinh phan mem thuat toan",
    "may tinh gia re van dung duoc lau dai",
]
CAU_HOI = "may tinh phan mem thuat toan nao tot nhat"

hang_bm25 = xep_hang_bm25(CAU_HOI, KHO, 1.5, 0.75)
hang_cosine = xep_hang_cosine(CAU_HOI, KHO, TU_VUNG, CUM_TU_GOC)
hang_rrf = xep_hang_rrf(KHO, [hang_bm25, hang_cosine], 1)

print(hang_bm25)
print(hang_cosine)
print(hang_rrf)
```

```text title=readonly
[2, 1, 0, 3]
[1, 0, 2, 3]
[1, 2, 0, 3]
```

BM25 xếp đoạn `2` đầu tiên (điểm `3,5042` — lặp `"phan mem"` `3` lần cộng
`"thuat toan"` `1` lần cho tf cao); cosine xếp đoạn `1` đầu tiên (điểm `1,0`
— đoạn duy nhất chạm CẢ `3` mục từ vựng của câu hỏi, hướng vector khớp
tuyệt đối). Hai hệ thống BẤT ĐỒNG hoàn toàn ở hạng `1`.

RRF (`k=1`) xếp đoạn `1` đầu tiên (hạng `2` ở BM25 — đóng góp `1/3≈0,3333` —
CỘNG hạng `1` ở cosine — đóng góp `1/2=0,5` — tổng `≈0,8333`), rồi đoạn `2`
(hạng `1` ở BM25 — đóng góp `0,5` — cộng hạng `3` ở cosine — đóng góp
`1/4=0,25` — tổng `0,75`). Kết quả: `[1, 2, 0, 3]` — một thứ tự THỨ BA,
KHÁC cả `[2, 1, 0, 3]` (BM25) lẫn `[1, 0, 2, 3]` (cosine). RRF không chọn
theo MỘT hệ thống — nó thưởng đoạn `1` (khá ở CẢ HAI hệ thống: hạng `2` và
hạng `1`) hơn đoạn `2` (cực tốt ở BM25 nhưng chỉ tầm trung ở cosine: hạng
`1` và hạng `3`).
::::

::::predict{#doan_rrf_ra_thu_tu_thu_ba commitOnce}
Xét đúng ví dụ trên: BM25 xếp đoạn `2` đầu tiên. Cosine xếp đoạn `1` đầu
tiên. Hai hệ thống bất đồng hoàn toàn.

**Trước khi chạy thử**, bạn đoán: RRF (`k=1`, hợp nhất cả hai) sẽ xếp danh
sách `4` đoạn theo thứ tự nào?

:::opt{correct}
Một thứ tự THỨ BA, khác cả BM25 (`[2, 1, 0, 3]`) lẫn cosine (`[1, 0, 2, 3]`)
— vì RRF cộng dồn đóng góp từ CẢ HAI hệ thống cho mỗi đoạn, không chọn theo
một hệ thống duy nhất; kết quả cuối phụ thuộc TỔNG hai đóng góp, không phải
đóng góp của riêng hệ thống nào
:::

:::opt
Đúng theo thứ tự của BM25 (`[2, 1, 0, 3]`) — vì BM25 khớp từ THẬT, không bị
giới hạn trong một từ vựng cố định như cosine, nên đáng tin hơn
::why
Gần đúng ở việc BM25 THẬT SỰ không bị giới hạn trong từ vựng cố định (đúng
điểm mù của cosine, đã đo ở q8.5d bài BOSS) — quan sát đó đúng trong NGỮ
CẢNH đó.

Chỗ lệch: công thức RRF cơ bản không gán "độ tin cậy" khác nhau cho từng hệ
thống — nó cộng dồn đóng góp từ CẢ HAI một cách bình đẳng. Đoạn `1` (hạng
`2` BM25, hạng `1` cosine) có tổng RRF (`0,8333`) CAO HƠN đoạn `2` (hạng `1`
BM25, hạng `3` cosine, tổng `0,75`) — nghĩa là RRF không đơn thuần theo đúng
thứ hạng của BM25.
::
:::

:::opt
Đúng theo thứ tự của cosine (`[1, 0, 2, 3]`) — vì cosine đã chuẩn hoá về
`[0, 1]`, phép đo "sạch" hơn BM25 (vốn không có trần cố định)
::why
Gần đúng ở việc cosine THẬT SỰ đã chuẩn hoá về `[0, 1]` (bài `2` của q8.5a)
— quan sát về tính chuẩn hoá đó đúng.

Chỗ lệch: RRF không dùng ĐỘ LỚN của điểm số (kể cả điểm đã chuẩn hoá) — nó
chỉ dùng THỨ HẠNG. Việc điểm cosine "sạch" hơn không có nghĩa RRF ưu tiên
thứ tự của nó — RRF cộng dồn đóng góp từ CẢ HAI hệ thống, và ở ví dụ này kết
quả hợp nhất KHÁC thứ tự cosine ở vị trí `2`/`3` (đoạn `2` vượt lên trước
đoạn `0` dù cosine xếp đoạn `0` trên đoạn `2`).
::
:::
::::

::::code{#viet_diem_rrf_mot_doan_va_xep_hang_rrf}
Hoàn thiện `diem_rrf_mot_doan` (cộng dồn `dong_gop_rrf` qua MỌI hệ thống
trong `danh_sach_xep_hang`) và `xep_hang_rrf` (tính điểm này cho mọi tài
liệu trong kho rồi sắp xếp giảm dần).

```python title=starter
import math

TU_VUNG = [
    "may_tinh", "phan_mem", "lap_trinh", "du_lieu", "vi_xu_ly", "ket_noi_mang", "thuat_toan", "ung_dung",
    "mon_an", "cong_thuc", "gia_vi", "nha_bep", "dau_bep", "thuc_pham", "nau_an", "mon_trang_mieng",
]
CUM_TU_GOC = {
    "may_tinh": "may tinh", "phan_mem": "phan mem", "lap_trinh": "lap trinh", "du_lieu": "du lieu",
    "vi_xu_ly": "vi xu ly", "ket_noi_mang": "ket noi mang", "thuat_toan": "thuat toan", "ung_dung": "ung dung",
    "mon_an": "mon an", "cong_thuc": "cong thuc", "gia_vi": "gia vi", "nha_bep": "nha bep",
    "dau_bep": "dau bep", "thuc_pham": "thuc pham", "nau_an": "nau an", "mon_trang_mieng": "mon trang mieng",
}


def chuan_hoa_cum_tu(van_ban, cum_tu_goc):
    kq = van_ban
    for token, cum in cum_tu_goc.items():
        kq = kq.replace(cum, token)
    return kq


def tinh_vector_dem_tu(van_ban, tu_vung, cum_tu_goc):
    vb = chuan_hoa_cum_tu(van_ban.lower(), cum_tu_goc)
    cac_tu = vb.split()
    return [cac_tu.count(tu) for tu in tu_vung]


def tich_vo_huong(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))


def do_dai_vector(v):
    return math.sqrt(sum(x ** 2 for x in v))


def tuong_dong_cosine(v1, v2):
    d1 = do_dai_vector(v1)
    d2 = do_dai_vector(v2)
    if d1 == 0 or d2 == 0:
        return 0.0
    return tich_vo_huong(v1, v2) / (d1 * d2)


def dem_tan_so(tu, van_ban):
    return van_ban.lower().split().count(tu)


def dem_so_doan_chua_tu(tu, kho):
    return sum(1 for doan in kho if tu in doan.lower().split())


def tinh_idf(tu, kho):
    df = dem_so_doan_chua_tu(tu, kho)
    if df == 0:
        return 0.0
    return math.log(len(kho) / df)


def he_so_chuan_hoa_do_dai(do_dai_doan, do_dai_trung_binh, b):
    return 1 - b + b * (do_dai_doan / do_dai_trung_binh)


def diem_bm25_mot_tu(tf, idf_tu, do_dai_doan, do_dai_trung_binh, k1, b):
    chuan_hoa = he_so_chuan_hoa_do_dai(do_dai_doan, do_dai_trung_binh, b)
    tu_so = tf * (k1 + 1)
    mau_so = tf + k1 * chuan_hoa
    return idf_tu * (tu_so / mau_so)


def do_dai_trung_binh(kho):
    return sum(len(doan.lower().split()) for doan in kho) / len(kho)


def diem_bm25_doan(doan, tu_truy_van, kho, k1, b):
    avgdl = do_dai_trung_binh(kho)
    do_dai_doan = len(doan.lower().split())
    tong = 0.0
    for t in tu_truy_van:
        tf_t = dem_tan_so(t, doan)
        if tf_t == 0:
            continue
        tong += diem_bm25_mot_tu(tf_t, tinh_idf(t, kho), do_dai_doan, avgdl, k1, b)
    return tong


def xep_hang_bm25(cau_hoi, kho, k1, b):
    tu_truy_van = cau_hoi.lower().split()
    diem = [diem_bm25_doan(doan, tu_truy_van, kho, k1, b) for doan in kho]
    return sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)


def xep_hang_cosine(cau_hoi, kho, tu_vung, cum_tu_goc):
    v_cau_hoi = tinh_vector_dem_tu(cau_hoi, tu_vung, cum_tu_goc)
    v_kho = [tinh_vector_dem_tu(doan, tu_vung, cum_tu_goc) for doan in kho]
    diem = [tuong_dong_cosine(v_cau_hoi, v) for v in v_kho]
    return sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)


def hang_cua(chi_so_muc_tieu, thu_tu_xep_hang):
    return thu_tu_xep_hang.index(chi_so_muc_tieu) + 1


def dong_gop_rrf(hang, k):
    return 1 / (k + hang)


def diem_rrf_mot_doan(chi_so, danh_sach_xep_hang, k):
    return sum(___ for xep_hang in danh_sach_xep_hang)      # dong_gop_rrf(hang_cua(chi_so, xep_hang), k)


def xep_hang_rrf(kho, danh_sach_xep_hang, k):
    diem = [diem_rrf_mot_doan(i, danh_sach_xep_hang, k) for i in range(len(kho))]
    return ___                                                # sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)


KHO = [
    "may tinh giup cong viec hieu qua hon nho thuat toan",
    "may tinh chay phan mem va thuat toan giup cong viec tot hon",
    "phan mem chay tot phan mem on dinh phan mem thuat toan",
    "may tinh gia re van dung duoc lau dai",
]
CAU_HOI = "may tinh phan mem thuat toan nao tot nhat"

hang_bm25 = xep_hang_bm25(CAU_HOI, KHO, 1.5, 0.75)
hang_cosine = xep_hang_cosine(CAU_HOI, KHO, TU_VUNG, CUM_TU_GOC)
hang_rrf = xep_hang_rrf(KHO, [hang_bm25, hang_cosine], 1)

print(hang_bm25)
print(hang_cosine)
print(hang_rrf)
```

```python title=solution
import math

TU_VUNG = [
    "may_tinh", "phan_mem", "lap_trinh", "du_lieu", "vi_xu_ly", "ket_noi_mang", "thuat_toan", "ung_dung",
    "mon_an", "cong_thuc", "gia_vi", "nha_bep", "dau_bep", "thuc_pham", "nau_an", "mon_trang_mieng",
]
CUM_TU_GOC = {
    "may_tinh": "may tinh", "phan_mem": "phan mem", "lap_trinh": "lap trinh", "du_lieu": "du lieu",
    "vi_xu_ly": "vi xu ly", "ket_noi_mang": "ket noi mang", "thuat_toan": "thuat toan", "ung_dung": "ung dung",
    "mon_an": "mon an", "cong_thuc": "cong thuc", "gia_vi": "gia vi", "nha_bep": "nha bep",
    "dau_bep": "dau bep", "thuc_pham": "thuc pham", "nau_an": "nau an", "mon_trang_mieng": "mon trang mieng",
}


def chuan_hoa_cum_tu(van_ban, cum_tu_goc):
    kq = van_ban
    for token, cum in cum_tu_goc.items():
        kq = kq.replace(cum, token)
    return kq


def tinh_vector_dem_tu(van_ban, tu_vung, cum_tu_goc):
    vb = chuan_hoa_cum_tu(van_ban.lower(), cum_tu_goc)
    cac_tu = vb.split()
    return [cac_tu.count(tu) for tu in tu_vung]


def tich_vo_huong(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))


def do_dai_vector(v):
    return math.sqrt(sum(x ** 2 for x in v))


def tuong_dong_cosine(v1, v2):
    d1 = do_dai_vector(v1)
    d2 = do_dai_vector(v2)
    if d1 == 0 or d2 == 0:
        return 0.0
    return tich_vo_huong(v1, v2) / (d1 * d2)


def dem_tan_so(tu, van_ban):
    return van_ban.lower().split().count(tu)


def dem_so_doan_chua_tu(tu, kho):
    return sum(1 for doan in kho if tu in doan.lower().split())


def tinh_idf(tu, kho):
    df = dem_so_doan_chua_tu(tu, kho)
    if df == 0:
        return 0.0
    return math.log(len(kho) / df)


def he_so_chuan_hoa_do_dai(do_dai_doan, do_dai_trung_binh, b):
    return 1 - b + b * (do_dai_doan / do_dai_trung_binh)


def diem_bm25_mot_tu(tf, idf_tu, do_dai_doan, do_dai_trung_binh, k1, b):
    chuan_hoa = he_so_chuan_hoa_do_dai(do_dai_doan, do_dai_trung_binh, b)
    tu_so = tf * (k1 + 1)
    mau_so = tf + k1 * chuan_hoa
    return idf_tu * (tu_so / mau_so)


def do_dai_trung_binh(kho):
    return sum(len(doan.lower().split()) for doan in kho) / len(kho)


def diem_bm25_doan(doan, tu_truy_van, kho, k1, b):
    avgdl = do_dai_trung_binh(kho)
    do_dai_doan = len(doan.lower().split())
    tong = 0.0
    for t in tu_truy_van:
        tf_t = dem_tan_so(t, doan)
        if tf_t == 0:
            continue
        tong += diem_bm25_mot_tu(tf_t, tinh_idf(t, kho), do_dai_doan, avgdl, k1, b)
    return tong


def xep_hang_bm25(cau_hoi, kho, k1, b):
    tu_truy_van = cau_hoi.lower().split()
    diem = [diem_bm25_doan(doan, tu_truy_van, kho, k1, b) for doan in kho]
    return sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)


def xep_hang_cosine(cau_hoi, kho, tu_vung, cum_tu_goc):
    v_cau_hoi = tinh_vector_dem_tu(cau_hoi, tu_vung, cum_tu_goc)
    v_kho = [tinh_vector_dem_tu(doan, tu_vung, cum_tu_goc) for doan in kho]
    diem = [tuong_dong_cosine(v_cau_hoi, v) for v in v_kho]
    return sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)


def hang_cua(chi_so_muc_tieu, thu_tu_xep_hang):
    return thu_tu_xep_hang.index(chi_so_muc_tieu) + 1


def dong_gop_rrf(hang, k):
    return 1 / (k + hang)


def diem_rrf_mot_doan(chi_so, danh_sach_xep_hang, k):
    return sum(dong_gop_rrf(hang_cua(chi_so, xep_hang), k) for xep_hang in danh_sach_xep_hang)


def xep_hang_rrf(kho, danh_sach_xep_hang, k):
    diem = [diem_rrf_mot_doan(i, danh_sach_xep_hang, k) for i in range(len(kho))]
    return sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)


KHO = [
    "may tinh giup cong viec hieu qua hon nho thuat toan",
    "may tinh chay phan mem va thuat toan giup cong viec tot hon",
    "phan mem chay tot phan mem on dinh phan mem thuat toan",
    "may tinh gia re van dung duoc lau dai",
]
CAU_HOI = "may tinh phan mem thuat toan nao tot nhat"

hang_bm25 = xep_hang_bm25(CAU_HOI, KHO, 1.5, 0.75)
hang_cosine = xep_hang_cosine(CAU_HOI, KHO, TU_VUNG, CUM_TU_GOC)
hang_rrf = xep_hang_rrf(KHO, [hang_bm25, hang_cosine], 1)

print(hang_bm25)
print(hang_cosine)
print(hang_rrf)
```

```python title=test
assert hang_bm25 == [2, 1, 0, 3], f"xep hang BM25 sai -- dang ra {hang_bm25}"
assert hang_cosine == [1, 0, 2, 3], f"xep hang cosine sai -- dang ra {hang_cosine}"
assert hang_rrf == [1, 2, 0, 3], f"xep hang RRF sai -- dang ra {hang_rrf}"
assert hang_rrf != hang_bm25, "RRF phai KHAC thu tu cua BM25 -- day chinh la 'thu tu thu ba'"
assert hang_rrf != hang_cosine, "RRF phai KHAC thu tu cua cosine -- day chinh la 'thu tu thu ba'"

diem_doan1 = diem_rrf_mot_doan(1, [hang_bm25, hang_cosine], 1)
diem_doan2 = diem_rrf_mot_doan(2, [hang_bm25, hang_cosine], 1)
assert round(diem_doan1, 4) == 0.8333, f"diem RRF cua doan 1 phai xap xi 0,8333 -- dang ra {diem_doan1}"
assert round(diem_doan2, 4) == 0.75, f"diem RRF cua doan 2 phai la 0,75 -- dang ra {diem_doan2}"
assert diem_doan1 > diem_doan2, "doan 1 (kha o CA HAI he thong) phai co diem RRF CAO HON doan 2 (cuc tot mot he thong, tam trung he thong kia)"

# kiem tra truc tiep tren mot vi du nho, tu tinh tay duoc: 2 doan, 2 he thong doi xung
assert round(diem_rrf_mot_doan(0, [[0, 1], [1, 0]], 1), 4) == round(diem_rrf_mot_doan(1, [[0, 1], [1, 0]], 1), 4), "hai doan doi xung hoan hao (hang 1 o he nay, hang 2 o he kia) phai co diem RRF BANG NHAU"
assert xep_hang_rrf(["a", "b"], [[0, 1], [1, 0]], 1) == [0, 1], "hoa thi sorted on dinh phai giu doan chi so THAP HON len truoc"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `diem_rrf_mot_doan`) là biểu thức bên trong `sum(... for xep_hang in danh_sach_xep_hang)` — với MỖI hệ thống xếp hạng `xep_hang` trong danh sách, tính đóng góp RRF của tài liệu `chi_so` TRONG hệ thống đó (dùng `hang_cua` để lấy hạng, rồi `dong_gop_rrf` để tính đóng góp). Chỗ hai (trong `xep_hang_rrf`) là giá trị TRẢ VỀ — sắp xếp MỌI chỉ số tài liệu theo `diem` giảm dần, đúng khuôn `xep_hang_bm25`/`xep_hang_cosine` đã quen.
- kind: strategy
  body: 'Chỗ đầu: `dong_gop_rrf(hang_cua(chi_so, xep_hang), k)` — gọi `hang_cua` để lấy hạng của `chi_so` trong `xep_hang`, rồi đưa hạng đó vào `dong_gop_rrf`. Chỗ hai: `sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)` — đúng khuôn sắp xếp đã dùng ở `xep_hang_bm25`/`xep_hang_cosine`.'
- kind: one-line
  body: 'Chỗ đầu là `dong_gop_rrf(hang_cua(chi_so, xep_hang), k)`, chỗ hai là `sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT dong_gop_rrf(hang_cua(chi_so, xep_hang), k) de cong don qua MOI he thong (khong duoc bo qua hang_cua hay dong_gop_rrf); VA cho trong hai phai GOI sorted(...) dung khuon xep_hang_bm25/xep_hang_cosine (khong duoc tra ve danh sach chua sap xep)
  requireAst:
  - kind: uses-call, target: dong_gop_rrf, min: 1
  - kind: uses-call, target: hang_cua, min: 1
  - kind: uses-call, target: sorted, min: 3
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [1, 1, 3] cho ba
  # luat theo dung thu tu khai bao o tren.
  # dong_gop_rrf=1: CHI mot lan GOI THAT trong toan bo solution, dung o cho
  # trong dau. Ham nay khong tu goi lai chinh no trong dinh nghia cua no.
  # hang_cua=1: CHI mot lan GOI THAT, dung o cho trong dau (long ben trong
  # loi goi dong_gop_rrf). Khong xuat hien o dau khac trong bai nay.
  # sorted=3 (TONG THAT, da xac nhan bang cong cu, khong doan tay): 2 lan CO
  # SAN trong boilerplate (xep_hang_bm25 va xep_hang_cosine, da day tu q8.5d),
  # 1 lan CHINH la cho trong hai. Neu chi dat min=1 (ngay tho), mot mutant
  # dien "return diem" (khong sap xep gi ca) vao cho trong hai van qua duoc
  # vi con lai 2 lan 'sorted' tu boilerplate -- GOTCHA "boilerplate-
  # threshold-masking"; da tu kiem chung: mutant nay lam hang_rrf tro thanh
  # [0, 1, 2, 3] (thu tu CHI SO goc, khong sap xep), bi bat CA boi static
  # (voi min=3, dung) LAN boi assertion "hang_rrf == [1, 2, 0, 3]".
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua ca
  # kiemAst() that lan python3 that de xac nhan, khong doan tay -- SUA LAI so
  # voi mot ban nhan xet truoc day tung nham lan): dien DUNG GIA TRI cua cho
  # trong hai ("sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)")
  # vao cho trong dau (BEN TRONG sum(___ for xep_hang in danh_sach_xep_hang),
  # boilerplate "for xep_hang in ..." VAN giu nguyen, khong doi) VA dien DUNG
  # GIA TRI cua cho trong dau ("dong_gop_rrf(hang_cua(chi_so, xep_hang), k)")
  # vao cho trong hai (thanh "return dong_gop_rrf(hang_cua(chi_so, xep_hang),
  # k)" -- MOT return don doc, KHONG co "for" nao di kem, vi cho trong hai
  # nam ngay sau tu khoa "return", khong nam trong mot generator/list-comp
  # nao ca) -- ca hai dang hoan doi nay DEU la cu phap Python HOP LE (khong
  # nem SyntaxError o buoc parse) -- da tu goi kiemAst() that xac nhan dat=
  # true (dem AST khong doi: [1, 1, 3], Y HET ban dung, static KHONG bat
  # duoc). Mutant nay BI BAT boi tier 'run': ben trong xep_hang_rrf (tham so
  # kho, danh_sach_xep_hang, k -- KHONG co chi_so, KHONG co xep_hang), bieu
  # thuc moi "dong_gop_rrf(hang_cua(chi_so, xep_hang), k)" dung hai ten CHUA
  # TON TAI trong scope nay -- da tu chay THAT qua python3, xac nhan no nem
  # NameError ("name 'chi_so' is not defined") ngay khi xep_hang_rrf(...)
  # duoc goi -- bi chan boi tier 'run', doc lap voi static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[2, 1, 0, 3\\]\\n\\[1, 0, 2, 3\\]\\n\\[1, 2, 0, 3\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`[1, 2, 0, 3]` — một thứ tự KHÁC cả BM25 lẫn cosine, hợp nhất bằng công
thức RRF đầy đủ trên hai hệ thống THẬT. Bài sau áp công thức này lên đúng
tình huống q8.5d bài BOSS đã đo — sửa điểm mù của cosine trước mã sản phẩm
hiếm `"sp4471"`.
::::

::::reflect{#nghi-lai}
Công thức RRF đầy đủ không phức tạp hơn bài trước bao nhiêu — nó chỉ đổi
"hai hạng cho sẵn" (bài `1`) thành "hạng thật, đọc ra từ MỌI hệ thống xếp
hạng mà tài liệu xuất hiện". Điều thú vị nằm ở KẾT QUẢ: khi hai hệ thống
thật sự bất đồng (BM25 thích đoạn `2`, cosine thích đoạn `1`), RRF không
đứng về phía nào — nó cộng dồn CẢ HAI tín hiệu, và đoạn THẮNG chung cuộc có
thể là đoạn không đứng đầu ở BẤT KỲ hệ thống riêng lẻ nào, miễn nó đủ khá ở
CẢ HAI. Đây chính là lý do người ta gọi cách làm này là "hybrid" — không
chọn một cách, ráp cả hai. Bài sau dùng đúng công thức này để sửa một lỗi
CỤ THỂ đã đo ở q8.5d: cosine mù trước mã sản phẩm hiếm `"sp4471"`.
::::

::::checkpoint{mastery=0.85}
::::
