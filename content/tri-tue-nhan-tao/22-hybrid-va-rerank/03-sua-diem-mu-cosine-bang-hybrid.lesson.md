---
id: tri-tue-nhan-tao.hybrid-va-rerank.sua-diem-mu-cosine-bang-hybrid
title: "Áp dụng hybrid RRF: sửa điểm mù của cosine trước mã hiếm \"sp4471\""
summary: "Dung lai NGUYEN VAN KHO_TAI_LIEU/CAU_HOI (ma san pham hiem 'sp4471') tu q8.5d bai BOSS. Nhac lai: BM25 xep doan dung o HANG 1 (diem 8,7117), cosine xep no o HANG 2 (vector TOAN SO 0 vi TU_VUNG khong chua 'sp4471', bi doan 1 -- mot doan cong nghe khong lien quan -- vuot qua nho tinh co dung 'may tinh'/'phan mem'). RRF hybrid (k=1) xep doan dung tro lai HANG 1 -- diem RRF cua doan dung va doan 1 HOA TUYET DOI o 0,8333 (moi doan co dung mot hang 1 va mot hang 2 giua hai he thong), va sorted on dinh giu doan chi so THAP HON (doan 0, dung chi so cua doan can tim) dung truoc."
locale: vi
track: tri-tue-nhan-tao
module: hybrid-va-rerank
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.sua-diem-mu-cosine-bang-hybrid]
requires: [ai.cong-thuc-rrf-day-du]
concepts: [ai.sua-diem-mu-cosine-bang-hybrid]
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
Bài trước viết công thức RRF đầy đủ trên một ví dụ tự nghĩ. Bài này quay lại
đúng tình huống q8.5d bài BOSS đã đo — nơi cosine xếp SAI một đoạn — và kiểm
xem hybrid RRF có sửa được nó không.
::::

::::explain{#nhac_lai_diem_mu_cosine}
Bài `boss-bm25-doi-dau-cosine-vector` (q8.5d, bài BOSS) đã dựng một tình
huống cụ thể: kho `7` đoạn, một câu hỏi chứa mã sản phẩm HIẾM `"sp4471"` —
chỉ xuất hiện ở ĐÚNG một đoạn (chỉ số `0`). Kết quả đo được:

> BM25 xếp đoạn `0` ở **hạng `1`** (điểm `8,7117`) — nó khớp CHÍNH XÁC từ
> `"sp4471"` (hiếm, `idf` cao) cộng nhiều từ khác của câu hỏi.
>
> Cosine xếp đoạn `0` ở **hạng `2`** — KHÔNG PHẢI vì tính sai, mà vì
> `TU_VUNG` (`16` mục cố định) không chứa `"sp4471"` cũng như bất kỳ từ nào
> khác mà đoạn `0` dùng — vector của nó là TOÀN SỐ `0`, cosine similarity
> đúng bằng `0,0`. Đoạn `1` (một đoạn công nghệ KHÔNG liên quan gì tới sản
> phẩm `sp4471`) tình cờ dùng `"may tinh"`/`"phan mem"` — hai mục CÓ trong
> `TU_VUNG` — nên có cosine `≈0,7071`, vượt qua đoạn `0`.

Bài này hỏi: nếu hợp nhất CẢ HAI hệ thống bằng RRF (bài trước), đoạn `0` sẽ
đứng ở hạng nào?
::::

::::example{#hybrid_sua_hang_doan_0}
Dùng lại NGUYÊN VĂN `KHO_TAI_LIEU`/`CAU_HOI` (chứa `"sp4471"`) từ q8.5d bài
BOSS:

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


def hang_rrf_cua_kho(cau_hoi, kho, tu_vung, cum_tu_goc, k1, b, k_rrf):
    hang_bm25 = xep_hang_bm25(cau_hoi, kho, k1, b)
    hang_cosine = xep_hang_cosine(cau_hoi, kho, tu_vung, cum_tu_goc)
    return xep_hang_rrf(kho, [hang_bm25, hang_cosine], k_rrf)


def so_sanh_ba_hang(chi_so_muc_tieu, hang_bm25, hang_cosine, hang_rrf):
    return hang_cua(chi_so_muc_tieu, hang_bm25), hang_cua(chi_so_muc_tieu, hang_cosine), hang_cua(chi_so_muc_tieu, hang_rrf)


KHO_TAI_LIEU = [
    "san pham sp4471 la mot thiet bi moi duoc nhieu nguoi dung danh gia cao ve do ben",
    "may tinh hien dai chay phan mem nhanh giup cong viec hieu qua hon truoc",
    "lap trinh vien dung thuat toan de xu ly du lieu lon moi ngay",
    "mon an ngon phu thuoc vao gia vi va cong thuc nau chuan",
    "dau bep gioi luon can than khi chon nguyen lieu tuoi ngon",
    "nha bep sach se giup mon an ngon hon va an toan hon",
    "mon trang mieng ngot ngao thuong duoc dung sau bua an chinh",
]
CAU_HOI = "toi muon hoi ve san pham sp4471 nay co dung duoc voi may tinh khong"

hang_bm25_v = xep_hang_bm25(CAU_HOI, KHO_TAI_LIEU, 1.5, 0.75)
hang_cosine_v = xep_hang_cosine(CAU_HOI, KHO_TAI_LIEU, TU_VUNG, CUM_TU_GOC)
hang_rrf_v = hang_rrf_cua_kho(CAU_HOI, KHO_TAI_LIEU, TU_VUNG, CUM_TU_GOC, 1.5, 0.75, 1)
ket_qua = so_sanh_ba_hang(0, hang_bm25_v, hang_cosine_v, hang_rrf_v)

print(hang_rrf_v)
print(ket_qua)
```

```text title=readonly
[0, 1, 2, 6, 3, 4, 5]
(1, 2, 1)
```

`ket_qua = (1, 2, 1)`: BM25 xếp đoạn `0` hạng `1` (như đã biết), cosine xếp
nó hạng `2` (như đã biết) — và **RRF hybrid xếp nó TRỞ LẠI hạng `1`**. Đoạn
`0` có đóng góp RRF `1/(1+1) + 1/(1+2) = 0,5 + 0,3333 = 0,8333` (hạng `1`
BM25 cộng hạng `2` cosine). Đoạn `1` có đóng góp `1/(1+2) + 1/(1+1) =
0,3333 + 0,5 = 0,8333` — CÙNG một tổng, vì hai đoạn này ĐỐI XỨNG hoàn hảo
(mỗi đoạn có đúng một hạng `1` và một hạng `2`, chỉ khác hệ thống nào cho
hạng nào — đúng hiện tượng đã đo ở bài `1`). Khi hoà, `sorted` ổn định giữ
nguyên thứ tự chỉ số GỐC — đoạn `0` (chỉ số thấp hơn) đứng TRƯỚC đoạn `1`,
nên đoạn `0` — đoạn ĐÚNG, chứa `"sp4471"` — thắng hạng `1` trong danh sách
hợp nhất, đúng như câu trả lời mong muốn.
::::

::::predict{#doan_hybrid_sua_dung_hang commitOnce}
Xét đúng ví dụ trên: đoạn `0` (chứa `"sp4471"`) được BM25 xếp hạng `1`,
cosine xếp hạng `2`. Đoạn `1` (không liên quan) được BM25 xếp hạng `2`,
cosine xếp hạng `1` — NGƯỢC LẠI đoạn `0` ở cả hai hệ thống.

**Trước khi chạy thử**, bạn đoán: RRF hybrid (`k=1`) sẽ xếp đoạn `0` ở hạng
mấy?

:::opt{correct}
Hạng `1` — đoạn `0` và đoạn `1` có tổng RRF BẰNG NHAU TUYỆT ĐỐI (`0,8333`,
vì mỗi đoạn có đúng một hạng `1` và một hạng `2` giữa hai hệ thống), và khi
hoà, `sorted` ổn định giữ đoạn có CHỈ SỐ THẤP HƠN (đoạn `0`) đứng trước
:::

:::opt
Hạng `2`, giống hệt cosine — vì cosine đã "nhìn thấy" đoạn `1` liên quan hơn
(cosine `≈0,7071` so với `0,0` của đoạn `0`), và RRF không thể đảo ngược một
chênh lệch điểm số lớn như vậy
::why
Gần đúng ở việc cosine THẬT SỰ cho đoạn `1` một điểm dương (`≈0,7071`) trong
khi đoạn `0` chỉ được `0,0` — quan sát về điểm số gốc đó đúng.

Chỗ lệch: RRF không dùng ĐỘ LỚN chênh lệch của điểm số gốc — nó chỉ dùng
THỨ HẠNG. Ở góc nhìn thứ hạng, đoạn `0` (hạng `1` BM25, hạng `2` cosine) và
đoạn `1` (hạng `2` BM25, hạng `1` cosine) đối xứng hoàn hảo, nên RRF cho ra
tổng bằng nhau — không hề bị "chênh lệch điểm số lớn" của cosine chi phối.
::
:::

:::opt
Hạng `3` hoặc thấp hơn — vì hai hệ thống bất đồng nhau về đoạn `0` (một xếp
hạng `1`, một xếp hạng `2`), và khi bất đồng, RRF thường đẩy tài liệu đó
xuống dưới CẢ HAI mức đã có
::why
Gần đúng ở việc hai hệ thống THẬT SỰ bất đồng về đoạn `0` — quan sát đó
đúng.

Chỗ lệch: RRF CỘNG DỒN đóng góp dương từ cả hai hệ thống, nó không "trừng
phạt" một tài liệu vì bị bất đồng. Đoạn `0` vẫn nhận được đóng góp `0,5`
đầy đủ từ hạng `1` BM25 — một đóng góp LỚN — cộng thêm `0,3333` từ hạng `2`
cosine, cho tổng `0,8333`, đủ để đứng đầu (hoà với đoạn `1`, thắng nhờ chỉ
số thấp hơn), không hề bị đẩy xuống hạng `3` trở xuống.
::
:::
::::

::::code{#viet_hang_rrf_cua_kho_va_so_sanh}
Hoàn thiện `hang_rrf_cua_kho` (hợp nhất `xep_hang_bm25` và `xep_hang_cosine`
qua `xep_hang_rrf`) và `so_sanh_ba_hang` (đọc hạng của một tài liệu trong cả
ba danh sách xếp hạng).

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
    return sum(dong_gop_rrf(hang_cua(chi_so, xep_hang), k) for xep_hang in danh_sach_xep_hang)


def xep_hang_rrf(kho, danh_sach_xep_hang, k):
    diem = [diem_rrf_mot_doan(i, danh_sach_xep_hang, k) for i in range(len(kho))]
    return sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)


def hang_rrf_cua_kho(cau_hoi, kho, tu_vung, cum_tu_goc, k1, b, k_rrf):
    hang_bm25 = xep_hang_bm25(cau_hoi, kho, k1, b)
    hang_cosine = xep_hang_cosine(cau_hoi, kho, tu_vung, cum_tu_goc)
    return ___                                              # xep_hang_rrf(kho, [hang_bm25, hang_cosine], k_rrf)


def so_sanh_ba_hang(chi_so_muc_tieu, hang_bm25, hang_cosine, hang_rrf):
    return hang_cua(chi_so_muc_tieu, hang_bm25), hang_cua(chi_so_muc_tieu, hang_cosine), ___  # hang_cua(chi_so_muc_tieu, hang_rrf)


KHO_TAI_LIEU = [
    "san pham sp4471 la mot thiet bi moi duoc nhieu nguoi dung danh gia cao ve do ben",
    "may tinh hien dai chay phan mem nhanh giup cong viec hieu qua hon truoc",
    "lap trinh vien dung thuat toan de xu ly du lieu lon moi ngay",
    "mon an ngon phu thuoc vao gia vi va cong thuc nau chuan",
    "dau bep gioi luon can than khi chon nguyen lieu tuoi ngon",
    "nha bep sach se giup mon an ngon hon va an toan hon",
    "mon trang mieng ngot ngao thuong duoc dung sau bua an chinh",
]
CAU_HOI = "toi muon hoi ve san pham sp4471 nay co dung duoc voi may tinh khong"

hang_bm25_v = xep_hang_bm25(CAU_HOI, KHO_TAI_LIEU, 1.5, 0.75)
hang_cosine_v = xep_hang_cosine(CAU_HOI, KHO_TAI_LIEU, TU_VUNG, CUM_TU_GOC)
hang_rrf_v = hang_rrf_cua_kho(CAU_HOI, KHO_TAI_LIEU, TU_VUNG, CUM_TU_GOC, 1.5, 0.75, 1)
ket_qua = so_sanh_ba_hang(0, hang_bm25_v, hang_cosine_v, hang_rrf_v)

print(hang_rrf_v)
print(ket_qua)
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


def hang_rrf_cua_kho(cau_hoi, kho, tu_vung, cum_tu_goc, k1, b, k_rrf):
    hang_bm25 = xep_hang_bm25(cau_hoi, kho, k1, b)
    hang_cosine = xep_hang_cosine(cau_hoi, kho, tu_vung, cum_tu_goc)
    return xep_hang_rrf(kho, [hang_bm25, hang_cosine], k_rrf)


def so_sanh_ba_hang(chi_so_muc_tieu, hang_bm25, hang_cosine, hang_rrf):
    return hang_cua(chi_so_muc_tieu, hang_bm25), hang_cua(chi_so_muc_tieu, hang_cosine), hang_cua(chi_so_muc_tieu, hang_rrf)


KHO_TAI_LIEU = [
    "san pham sp4471 la mot thiet bi moi duoc nhieu nguoi dung danh gia cao ve do ben",
    "may tinh hien dai chay phan mem nhanh giup cong viec hieu qua hon truoc",
    "lap trinh vien dung thuat toan de xu ly du lieu lon moi ngay",
    "mon an ngon phu thuoc vao gia vi va cong thuc nau chuan",
    "dau bep gioi luon can than khi chon nguyen lieu tuoi ngon",
    "nha bep sach se giup mon an ngon hon va an toan hon",
    "mon trang mieng ngot ngao thuong duoc dung sau bua an chinh",
]
CAU_HOI = "toi muon hoi ve san pham sp4471 nay co dung duoc voi may tinh khong"

hang_bm25_v = xep_hang_bm25(CAU_HOI, KHO_TAI_LIEU, 1.5, 0.75)
hang_cosine_v = xep_hang_cosine(CAU_HOI, KHO_TAI_LIEU, TU_VUNG, CUM_TU_GOC)
hang_rrf_v = hang_rrf_cua_kho(CAU_HOI, KHO_TAI_LIEU, TU_VUNG, CUM_TU_GOC, 1.5, 0.75, 1)
ket_qua = so_sanh_ba_hang(0, hang_bm25_v, hang_cosine_v, hang_rrf_v)

print(hang_rrf_v)
print(ket_qua)
```

```python title=test
assert hang_bm25_v == [0, 1, 6, 2, 3, 4, 5], f"xep hang BM25 sai -- dang ra {hang_bm25_v}"
assert hang_cosine_v == [1, 0, 2, 3, 4, 5, 6], f"xep hang cosine sai -- dang ra {hang_cosine_v}"
assert hang_rrf_v == [0, 1, 2, 6, 3, 4, 5], f"xep hang RRF sai -- dang ra {hang_rrf_v}"
assert ket_qua == (1, 2, 1), f"ket_qua phai la (1, 2, 1): BM25 hang 1, cosine hang 2, RRF hang 1 -- dang ra {ket_qua}"
assert ket_qua[0] != ket_qua[1], "BM25 va cosine phai xep doan 0 o HAI HANG KHAC NHAU -- day chinh la diem mu can sua"
assert ket_qua[2] == ket_qua[0], "RRF phai KHOI PHUC dung hang cua BM25 (hang 1) cho doan chua 'sp4471'"

diem_doan0 = diem_rrf_mot_doan(0, [hang_bm25_v, hang_cosine_v], 1)
diem_doan1 = diem_rrf_mot_doan(1, [hang_bm25_v, hang_cosine_v], 1)
assert round(diem_doan0, 4) == 0.8333, f"diem RRF cua doan 0 phai xap xi 0,8333 -- dang ra {diem_doan0}"
assert diem_doan0 == diem_doan1, "doan 0 va doan 1 phai HOA TUYET DOI ve diem RRF (doi xung hang giua hai he thong)"

# kiem tra truc tiep tren mot vi du nho, tu tinh tay duoc
assert hang_rrf_cua_kho("a b", ["a b", "b"], TU_VUNG, CUM_TU_GOC, 1.5, 0.75, 1) == [0, 1], "kho 2 doan, doan 0 khop het cau hoi phai dung truoc"
assert so_sanh_ba_hang(0, [0, 1], [1, 0], [0, 1]) == (1, 2, 1), f"so_sanh_ba_hang doc sai hang -- dang ra {so_sanh_ba_hang(0, [0, 1], [1, 0], [0, 1])}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `hang_rrf_cua_kho`) là giá trị TRẢ VỀ — hợp nhất `hang_bm25` và `hang_cosine` (đã tính ở hai dòng trên) bằng `xep_hang_rrf`, với `k_rrf` làm hằng số làm mượt. Chỗ hai (trong `so_sanh_ba_hang`) là phần tử THỨ BA của tuple trả về — hạng của `chi_so_muc_tieu` TRONG `hang_rrf` (đối số thứ tư của hàm), dùng `hang_cua`.
- kind: strategy
  body: 'Chỗ đầu: `xep_hang_rrf(kho, [hang_bm25, hang_cosine], k_rrf)` — gói hai danh sách xếp hạng vào một list rồi hợp nhất. Chỗ hai: `hang_cua(chi_so_muc_tieu, hang_rrf)` — cùng khuôn với hai lời gọi `hang_cua` đã cho sẵn ngay phía trước, chỉ đổi đối số thứ hai thành `hang_rrf`.'
- kind: one-line
  body: 'Chỗ đầu là `xep_hang_rrf(kho, [hang_bm25, hang_cosine], k_rrf)`, chỗ hai là `hang_cua(chi_so_muc_tieu, hang_rrf)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT xep_hang_rrf(kho, [hang_bm25, hang_cosine], k_rrf) de hop nhat hai xep hang (khong duoc tra ve rieng mot trong hai); VA cho trong hai phai DOC dung bien 'hang_rrf' qua hang_cua (khong duoc dung lai hang_bm25 hay hang_cosine)
  requireAst:
  - kind: uses-call, target: xep_hang_rrf, min: 1
  - kind: uses-name, target: hang_rrf, min: 1
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [1, 1] cho hai
  # luat theo dung thu tu khai bao o tren.
  # xep_hang_rrf=1: CHI mot lan GOI THAT trong toan bo solution, dung o cho
  # trong dau. Ham nay khong tu goi lai chinh no trong dinh nghia cua no.
  # hang_rrf=1 (uses-name, chi dem cho DOC/Load, khong dem tham so ham): CHI
  # mot lan DOC, dung o cho trong hai (doi so thu hai cua hang_cua). Ten
  # 'hang_rrf' o dau khac trong bai nay CHI xuat hien lam TEN THAM SO cua
  # so_sanh_ba_hang va bien top-level 'hang_rrf_v' (mot ten KHAC, khong khop).
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua ham
  # _dem de xac nhan, khong doan tay): dien "hang_cua(chi_so_muc_tieu,
  # hang_rrf)" vao cho trong dau ("return hang_cua(chi_so_muc_tieu, hang_rrf)"
  # trong hang_rrf_cua_kho) VA dien "xep_hang_rrf(kho, [hang_bm25,
  # hang_cosine], k_rrf)" vao cho trong hai (thay cho phan tu thu ba cua
  # tuple, trong so_sanh_ba_hang) -- tong so lan goi xep_hang_rrf VA tong so
  # lan doc ten hang_rrf KHONG DOI (van la 1 va 1, chi doi VI TRI). Static
  # KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong hang_rrf_cua_kho (tham so
  # cau_hoi, kho, tu_vung, cum_tu_goc, k1, b, k_rrf -- KHONG co
  # chi_so_muc_tieu, KHONG co hang_rrf), bieu thuc moi "hang_cua(chi_so_muc_
  # tieu, hang_rrf)" dung hai ten CHUA HE TON TAI trong scope nay; ben trong
  # so_sanh_ba_hang (tham so chi_so_muc_tieu, hang_bm25, hang_cosine,
  # hang_rrf -- KHONG co cau_hoi, kho, tu_vung, cum_tu_goc, k1, b, k_rrf),
  # bieu thuc moi "xep_hang_rrf(kho, [hang_bm25, hang_cosine], k_rrf)" dung
  # nhieu ten CHUA TON TAI trong scope nay. Da tu chay THAT mutant nay qua
  # python3, xac nhan no nem NameError ("name 'chi_so_muc_tieu' is not
  # defined") ngay khi hang_rrf_cua_kho(...) duoc goi lan dau -- bi chan boi
  # tier 'run', doc lap voi static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[0, 1, 2, 6, 3, 4, 5\\]\\n\\(1, 2, 1\\)\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`(1, 2, 1)` — BM25 hạng `1`, cosine hạng `2`, RRF hợp nhất KHÔI PHỤC hạng
`1`. Hybrid giữ được đúng ưu điểm của BM25 (khớp mã hiếm chính xác) mà không
mất gì. Bài BOSS tiếp theo dựng MỘT kho mới với HAI câu hỏi — một sửa điểm
mù của cosine (như bài này), một sửa điểm mù NGƯỢC LẠI của BM25 — đóng
`q8.5e` tại `4/4`.
::::

::::reflect{#nghi-lai}
Bài này không cần một ví dụ MỚI để chứng minh RRF hoạt động — nó quay lại
đúng tình huống mà q8.5d bài BOSS đã dùng để phơi bày điểm mù của cosine, và
đo xem hybrid có sửa được nó không. Câu trả lời không phải "RRF luôn thắng
tuyệt đối" — ở đây, RRF thắng nhờ một TRẬN HOÀ (đoạn `0` và đoạn `1` có tổng
RRF bằng nhau tuyệt đối) được phân định bằng thứ tự chỉ số gốc, không phải
bằng một chênh lệch điểm số rõ ràng. Đây là một kết luận trung thực hơn là
"hybrid hoàn hảo": nó cho thấy CƠ CHẾ cụ thể — đóng góp `0,5` từ hạng `1`
BM25 đủ lớn để kéo đoạn `0` lên ngang hàng với đối thủ mạnh nhất của cosine,
dù cosine tự nó xếp đoạn `0` thấp hơn. Bài BOSS tiếp theo kiểm tra cơ chế
này trên MỘT tình huống khó hơn: một kho mới, HAI câu hỏi khác nhau, mỗi
câu hỏi phơi bày điểm mù của MỘT phương pháp khác nhau.
::::

::::checkpoint{mastery=0.85}
::::
