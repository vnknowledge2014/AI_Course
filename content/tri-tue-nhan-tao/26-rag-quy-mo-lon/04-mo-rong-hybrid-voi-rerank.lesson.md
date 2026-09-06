---
id: tri-tue-nhan-tao.rag-quy-mo-lon.mo-rong-hybrid-voi-rerank
title: "Mở rộng hybrid với rerank: một tiêu chí KHÁC xếp lại top-N"
summary: "so_tu_khoa_trung_khop(cau_hoi, doan) = len(tu_cau_hoi & tu_doan) -- so TU KHOA phan biet, khop CHINH XAC giua cau hoi va doan (khac han tuong dong ngu nghia cua cosine hay diem BM25 co trong so IDF). rerank(cac_chi_so_ung_vien, cau_hoi, danh_sach_tai_lieu, k) xep LAI CHI trong so ung vien do theo diem nay, tra ve top-k SAU rerank. Tren kho 4 doan va CAU_HOI ve 'vi xu ly...anh huong...hieu nang...may tinh', hang_rrf_cua_kho (tai dung tu q8.5e) cho thu tu day du [0, 2, 1, 3] -- doan 0 hang 1. Rerank tren top-3 [0, 2, 1] theo so tu khop CHINH XAC (5, 10, 1) doi thu tu thanh [2, 0, 1] -- doan 2 (hang 2 truoc rerank, 10 tu khop) VUOT LEN hang 1, doan 0 (hang 1 truoc rerank, 5 tu khop) lui xuong hang 2. top3_truoc_rerank[0] == top2_sau_rerank[0] la False -- rerank THAT SU doi hang 1."
locale: vi
track: tri-tue-nhan-tao
module: rag-quy-mo-lon
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.mo-rong-hybrid-voi-rerank]
requires: [ai.cach-ly-theo-khach-hang]
concepts: [ai.mo-rong-hybrid-voi-rerank]
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
`q8.5e` đã xây hybrid RRF (BM25 hợp nhất cosine). `Chương 39.4` mục `3` thêm
MỘT bước NỮA sau đó: **"Dùng một model nhỏ (Reranker) chấm điểm lại `20` kết
quả thu được trước khi nhét vào Prompt."** Vì sao cần THÊM một bước, sau khi
đã có hybrid?
::::

::::explain{#mot_tieu_chi_khac_xep_lai_topn}
Hybrid RRF (q8.5e) hợp nhất **thứ hạng** từ nhiều hệ thống — nó KHÔNG dùng
độ lớn điểm số gốc, chỉ dùng vị trí xếp hạng. Điều này mạnh (chống lại một
hệ thống lấn át hệ thống khác), nhưng cũng có nghĩa: một tài liệu "khá" ở
CẢ HAI hệ thống (BM25 lẫn cosine) có thể xếp CAO trong RRF, dù không có tài
liệu nào trong số đó thật sự khớp CHÍNH XÁC với TỪ NGỮ của câu hỏi.

Reranker giải quyết đúng chỗ đó: SAU khi có top-N ứng viên từ hybrid RRF, áp
một phép đo THỨ BA — KHÁC HẲN cả BM25 (khớp từ khoá có trọng số IDF) lẫn
cosine (tương đồng ngữ nghĩa qua vector) — CHỈ trong số N ứng viên đó. Bài
này dùng một phép đo đơn giản: **số từ khoá PHÂN BIỆT khớp CHÍNH XÁC** giữa
câu hỏi và tài liệu (tập hợp các từ chung, không quan tâm tần suất, không
quan tâm độ hiếm) — một phép đo "khớp cú pháp thô" khác hẳn cả hai tầng
trước.

```
so_tu_khoa_trung_khop(cau_hoi, doan) = |TU(cau_hoi) GIAO TU(doan)|

rerank(cac_chi_so_ung_vien, cau_hoi, danh_sach_tai_lieu, k):
  diem = [so_tu_khoa_trung_khop(cau_hoi, danh_sach_tai_lieu[i]) CHO MOI i TRONG cac_chi_so_ung_vien]
  thu_tu = sap xep GIAM DAN theo diem (CHI trong pham vi cac_chi_so_ung_vien)
  tra ve k chi so DAU cua thu_tu do (anh xa nguoc ve chi so GOC)
```

Vì `rerank` chỉ nhìn vào những ứng viên ĐÃ được hybrid RRF chọn ra (không
quét lại toàn kho), một tài liệu hạng THẤP trong top-N ban đầu CÓ THỂ vượt
lên hạng CAO NHẤT nếu nó khớp từ khoá chính xác hơn — kể cả đổi hẳn hạng
`1`.
::::

::::example{#rerank_doi_hang_1}
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


def so_tu_khoa_trung_khop(cau_hoi, doan):
    tu_cau_hoi = set(cau_hoi.lower().split())
    tu_doan = set(doan.lower().split())
    return len(tu_cau_hoi & tu_doan)


def rerank(cac_chi_so_ung_vien, cau_hoi, danh_sach_tai_lieu, k):
    diem = [so_tu_khoa_trung_khop(cau_hoi, danh_sach_tai_lieu[i]) for i in cac_chi_so_ung_vien]
    thu_tu = sorted(range(len(cac_chi_so_ung_vien)), key=lambda j: diem[j], reverse=True)
    return [cac_chi_so_ung_vien[j] for j in thu_tu[:k]]


KHO = [
    "vi xu ly la thanh phan quan trong nhat trong may tinh",
    "gia vi va cong thuc nau an ngon giup mon an hap dan hon",
    "may tinh hien dai anh huong toi hieu nang cua nguoi dung rat nhieu khi lam viec",
    "dau bep gioi can than khi chon nguyen lieu tuoi ngon moi ngay",
]
CAU_HOI = "vi xu ly nao anh huong toi hieu nang cua may tinh hien dai"

hang_rrf_toan_kho = hang_rrf_cua_kho(CAU_HOI, KHO, TU_VUNG, CUM_TU_GOC, 1.5, 0.75, 1)
top3_truoc_rerank = hang_rrf_toan_kho[:3]
top2_sau_rerank = rerank(top3_truoc_rerank, CAU_HOI, KHO, 2)

print(hang_rrf_toan_kho)
print(top3_truoc_rerank)
print(top2_sau_rerank)
print(top3_truoc_rerank[0] == top2_sau_rerank[0])
```

```text title=readonly
[0, 2, 1, 3]
[0, 2, 1]
[2, 0]
False
```

Hybrid RRF (`hang_rrf_toan_kho`) xếp hạng TOÀN kho: `[0, 2, 1, 3]` — đoạn
`0` (`"vi xu ly la thanh phan quan trong nhat..."`) hạng `1`, đoạn `2`
(`"may tinh hien dai anh huong toi hieu nang..."`) hạng `2`. `top3_truoc_
rerank = [0, 2, 1]` — ba ứng viên đầu. Đếm từ khoá khớp CHÍNH XÁC giữa
`CAU_HOI` và từng ứng viên: đoạn `0` có `5` từ khoá chung, đoạn `2` có `10`
(đoạn này lặp lại gần như NGUYÊN VĂN nhiều từ của câu hỏi: `"anh huong"`,
`"hieu nang"`, `"may tinh"`, `"hien dai"`, `"cua"`), đoạn `1` có `1`. Rerank
sắp xếp lại CHỈ trong số `3` ứng viên này theo số đếm đó: đoạn `2` (`10` từ
khoá) vượt LÊN hạng `1`, đoạn `0` (`5` từ khoá) lùi xuống hạng `2` —
`top2_sau_rerank = [2, 0]`. `top3_truoc_rerank[0] == top2_sau_rerank[0]` LÀ
`False`: hạng `1` THẬT SỰ đổi, từ đoạn `0` (trước rerank) sang đoạn `2` (sau
rerank) — không phải một sắp xếp lại nhẹ ở cuối danh sách, mà đổi NGAY vị
trí đầu.
::::

::::predict{#doan_gia_tri_hang_1_sau_rerank commitOnce}
Xét đúng ví dụ trên. **Trước khi chạy thử**, bạn đoán: `top2_sau_rerank[0]`
(chỉ số đoạn đứng HẠNG `1` SAU rerank) LÀ bao nhiêu?

:::opt{correct}
`2` — đoạn `2` có `10` từ khoá khớp chính xác với câu hỏi (nhiều hơn HẲN
đoạn `0`, chỉ có `5`), nên dù đoạn `2` chỉ đứng hạng `2` TRƯỚC rerank (theo
hybrid RRF), nó vượt LÊN hạng `1` sau khi áp tiêu chí khớp từ khoá chính xác
:::

:::opt
`0` — giữ nguyên hạng `1` như hybrid RRF đã xếp, vì rerank chỉ tinh chỉnh
NHẸ thứ tự, không đủ sức đổi hẳn vị trí đầu
::why
Gần đúng ở việc rerank THƯỜNG giữ nguyên một phần thứ tự ban đầu khi các
ứng viên có điểm khá gần nhau — quan sát đó có thể đúng trong NHIỀU trường
hợp khác.

Chỗ lệch: rerank không có "giới hạn" nào buộc nó giữ nguyên hạng `1` — nó
chỉ đơn giản sắp xếp LẠI theo điểm `so_tu_khoa_trung_khop`, không quan tâm
thứ hạng TRƯỚC đó là gì. Ở đây chênh lệch từ khoá (`10` so với `5`) đủ lớn
để đổi hẳn vị trí đầu — không có cơ chế nào trong `rerank` "bảo vệ" hạng
`1` cũ.
::
:::

:::opt
`1` — đoạn xếp hạng `2` TRONG `top3_truoc_rerank` (theo thứ tự chỉ số trong
danh sách `[0, 2, 1]`, phần tử ở VỊ TRÍ chỉ số `1` của danh sách đó, tức
giá trị `2`... nhưng đoán nhầm giá trị TRẢ VỀ LÀ chỉ số VỊ TRÍ, không phải
chỉ số TÀI LIỆU)
::why
Gần đúng ở việc `top3_truoc_rerank[1] == 2` — nếu bạn tính "vị trí thứ `1`
(đếm từ `0`) trong danh sách ứng viên" thì ĐÚNG là giá trị `2` nằm ở đó.

Chỗ lệch: câu hỏi yêu cầu GIÁ TRỊ của `top2_sau_rerank[0]` — một CHỈ SỐ TÀI
LIỆU (chỉ số trong `KHO`), không phải một VỊ TRÍ trong danh sách
`top3_truoc_rerank`. `rerank` trả về danh sách CÁC CHỈ SỐ TÀI LIỆU đã sắp
xếp lại, và giá trị đứng đầu danh sách đó (sau khi tính điểm) LÀ `2` —
trùng CON SỐ với ví dụ trên nhưng vì một LÝ DO khác (đoạn `2` thắng vì
`10` từ khoá khớp, không phải vì nó ở "vị trí `1`" nào).
::
:::
::::

::::code{#viet_so_tu_khoa_va_rerank}
Hoàn thiện `so_tu_khoa_trung_khop` (đếm từ khoá PHÂN BIỆT khớp chính xác
giữa câu hỏi và một đoạn) và `rerank` (sắp xếp lại CHỈ trong số ứng viên đã
cho, theo điểm đó).

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
    return xep_hang_rrf(kho, [hang_bm25, hang_cosine], k_rrf)


def so_tu_khoa_trung_khop(cau_hoi, doan):
    tu_cau_hoi = set(cau_hoi.lower().split())
    tu_doan = set(doan.lower().split())
    return len(___)                                          # tu_cau_hoi & tu_doan


def rerank(cac_chi_so_ung_vien, cau_hoi, danh_sach_tai_lieu, k):
    diem = [so_tu_khoa_trung_khop(cau_hoi, danh_sach_tai_lieu[i]) for i in cac_chi_so_ung_vien]
    thu_tu = sorted(range(len(cac_chi_so_ung_vien)), key=lambda j: diem[j], reverse=True)
    return ___                                               # [cac_chi_so_ung_vien[j] for j in thu_tu[:k]]


KHO = [
    "vi xu ly la thanh phan quan trong nhat trong may tinh",
    "gia vi va cong thuc nau an ngon giup mon an hap dan hon",
    "may tinh hien dai anh huong toi hieu nang cua nguoi dung rat nhieu khi lam viec",
    "dau bep gioi can than khi chon nguyen lieu tuoi ngon moi ngay",
]
CAU_HOI = "vi xu ly nao anh huong toi hieu nang cua may tinh hien dai"

hang_rrf_toan_kho = hang_rrf_cua_kho(CAU_HOI, KHO, TU_VUNG, CUM_TU_GOC, 1.5, 0.75, 1)
top3_truoc_rerank = hang_rrf_toan_kho[:3]
top2_sau_rerank = rerank(top3_truoc_rerank, CAU_HOI, KHO, 2)

print(hang_rrf_toan_kho)
print(top3_truoc_rerank)
print(top2_sau_rerank)
print(top3_truoc_rerank[0] == top2_sau_rerank[0])
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


def so_tu_khoa_trung_khop(cau_hoi, doan):
    tu_cau_hoi = set(cau_hoi.lower().split())
    tu_doan = set(doan.lower().split())
    return len(tu_cau_hoi & tu_doan)


def rerank(cac_chi_so_ung_vien, cau_hoi, danh_sach_tai_lieu, k):
    diem = [so_tu_khoa_trung_khop(cau_hoi, danh_sach_tai_lieu[i]) for i in cac_chi_so_ung_vien]
    thu_tu = sorted(range(len(cac_chi_so_ung_vien)), key=lambda j: diem[j], reverse=True)
    return [cac_chi_so_ung_vien[j] for j in thu_tu[:k]]


KHO = [
    "vi xu ly la thanh phan quan trong nhat trong may tinh",
    "gia vi va cong thuc nau an ngon giup mon an hap dan hon",
    "may tinh hien dai anh huong toi hieu nang cua nguoi dung rat nhieu khi lam viec",
    "dau bep gioi can than khi chon nguyen lieu tuoi ngon moi ngay",
]
CAU_HOI = "vi xu ly nao anh huong toi hieu nang cua may tinh hien dai"

hang_rrf_toan_kho = hang_rrf_cua_kho(CAU_HOI, KHO, TU_VUNG, CUM_TU_GOC, 1.5, 0.75, 1)
top3_truoc_rerank = hang_rrf_toan_kho[:3]
top2_sau_rerank = rerank(top3_truoc_rerank, CAU_HOI, KHO, 2)

print(hang_rrf_toan_kho)
print(top3_truoc_rerank)
print(top2_sau_rerank)
print(top3_truoc_rerank[0] == top2_sau_rerank[0])
```

```python title=test
assert hang_rrf_toan_kho == [0, 2, 1, 3], f"hang_rrf_toan_kho phai la [0, 2, 1, 3] -- dang ra {hang_rrf_toan_kho}"
assert top3_truoc_rerank == [0, 2, 1], f"top3_truoc_rerank phai la [0, 2, 1] -- dang ra {top3_truoc_rerank}"
assert top2_sau_rerank == [2, 0], f"top2_sau_rerank phai la [2, 0] -- dang ra {top2_sau_rerank}"
assert top3_truoc_rerank[0] != top2_sau_rerank[0], "rerank PHAI doi hang 1 (doan 0 truoc rerank, doan 2 sau rerank)"

assert so_tu_khoa_trung_khop(CAU_HOI, KHO[0]) == 5, f"so tu khoa khop cua doan 0 phai la 5 -- dang ra {so_tu_khoa_trung_khop(CAU_HOI, KHO[0])}"
assert so_tu_khoa_trung_khop(CAU_HOI, KHO[2]) == 10, f"so tu khoa khop cua doan 2 phai la 10 -- dang ra {so_tu_khoa_trung_khop(CAU_HOI, KHO[2])}"
assert so_tu_khoa_trung_khop(CAU_HOI, KHO[1]) == 1, f"so tu khoa khop cua doan 1 phai la 1 -- dang ra {so_tu_khoa_trung_khop(CAU_HOI, KHO[1])}"
assert so_tu_khoa_trung_khop("a b c", "a b c") == 3, f"trung khop hoan toan phai dem 3 -- dang ra {so_tu_khoa_trung_khop('a b c', 'a b c')}"
assert so_tu_khoa_trung_khop("a b c", "x y z") == 0, f"khong trung tu nao phai dem 0 -- dang ra {so_tu_khoa_trung_khop('a b c', 'x y z')}"

# bien: tham so k THAT SU rang buoc rerank
assert rerank(top3_truoc_rerank, CAU_HOI, KHO, 1) == [2], f"rerank voi k=1 phai la [2] -- dang ra {rerank(top3_truoc_rerank, CAU_HOI, KHO, 1)}"
assert rerank(top3_truoc_rerank, CAU_HOI, KHO, 3) == [2, 0, 1], f"rerank voi k=3 (het ca 3 ung vien) phai la [2, 0, 1] -- dang ra {rerank(top3_truoc_rerank, CAU_HOI, KHO, 3)}"
assert rerank([0, 2, 1], CAU_HOI, KHO, 1) == rerank([0, 2, 1], CAU_HOI, KHO, 3)[:1], "k=1 va lay [:1] cua k=3 phai cho CUNG phan tu dau -- kiem tra nhat quan giua hai gia tri k khac nhau"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `so_tu_khoa_trung_khop`) là GIÁ TRỊ đưa vào `len(...)` — phần GIAO của hai tập từ vừa tạo (`tu_cau_hoi`, `tu_doan`). Chỗ hai (trong `rerank`) là GIÁ TRỊ TRẢ VỀ CUỐI CÙNG — ánh xạ `k` chỉ số ĐẦU của `thu_tu` (thứ tự sau khi sắp theo điểm) trở lại giá trị THẬT trong `cac_chi_so_ung_vien`.
- kind: strategy
  body: 'Chỗ đầu: `tu_cau_hoi & tu_doan` — toán tử GIAO của hai `set`, cho ra tập các từ XUẤT HIỆN Ở CẢ HAI. Chỗ hai: `[cac_chi_so_ung_vien[j] for j in thu_tu[:k]]` — mỗi `j` trong `thu_tu[:k]` là một VỊ TRÍ (đã sắp theo điểm); `cac_chi_so_ung_vien[j]` tra ra CHỈ SỐ TÀI LIỆU thật ở vị trí đó.'
- kind: one-line
  body: 'Chỗ đầu là `tu_cau_hoi & tu_doan`, chỗ hai là `[cac_chi_so_ung_vien[j] for j in thu_tu[:k]]`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai la phep GIAO (&) giua tu_cau_hoi VA tu_doan, dua vao len(...) (khong duoc dung len rieng cua mot tap); cho trong hai phai la MOT list comprehension ANH XA thu_tu[:k] qua cac_chi_so_ung_vien (khong duoc tra ve thang thu_tu[:k], se la VI TRI chu khong phai CHI SO TAI LIEU that)
  requireAst:
  - kind: uses-name, target: tu_doan, min: 1
  - kind: uses-name, target: tu_cau_hoi, min: 1
  - kind: uses-name, target: thu_tu, min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, dist build that, trich
  # solution TU CHINH file nay -- gom CA toolkit BM25/cosine/RRF copy
  # nguyen van tu q8.5e) -- xac nhan CHINH XAC (min VA min+1): tu_doan=1,
  # tu_cau_hoi=1, thu_tu=1. CA BA ten nay la bien CUC BO CHI xuat hien
  # trong hai ham moi (so_tu_khoa_trung_khop, rerank) -- khong bien nao
  # trong toan bo toolkit BM25/cosine/RRF (da tai dung nguyen van tu q8.5e)
  # trung ten voi ba bien nay.
  # tu_doan=1: DUY NHAT o cho trong dau (tu_cau_hoi & tu_doan) -- dong gan
  # "tu_doan = set(doan.lower().split())" LA Store, khong duoc uses-name
  # dem. tu_cau_hoi=1: tuong tu, DUY NHAT o cho trong dau. thu_tu=1: DUY
  # NHAT o cho trong hai (thu_tu[:k]) -- dong gan "thu_tu = sorted(...)" LA
  # Store, khong dem.
  # Dien bua "True" vao ca hai cho trong ("return len(True)" -- se NEM LOI
  # ngay o tier run vi len() khong nhan int, nhung o tier static rieng le
  # thi tu_doan=0 VA tu_cau_hoi=0; "return True" cho thu_tu=0) -- CA BA luat
  # CHAN DUNG (da CHAY THAT xac nhan qua kiemAst).
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho
  # trong -- xac dinh ranh gioi TU CHINH khoi starter -- va CHAY THAT qua
  # kiemAst() THAT VA python3 THAT): dien
  # "[cac_chi_so_ung_vien[j] for j in thu_tu[:k]]" vao cho trong dau
  # ("return len([cac_chi_so_ung_vien[j] for j in thu_tu[:k]])" trong
  # so_tu_khoa_trung_khop) VA dien "tu_cau_hoi & tu_doan" vao cho trong hai
  # ("return tu_cau_hoi & tu_doan" trong rerank) -- da CHAY THAT qua
  # kiemAst(): CA BA con so (tu_doan=1, tu_cau_hoi=1, thu_tu=1) tren TOAN BO
  # solution DEU KHONG DOI (chi doi VI TRI) -- static KHONG bat duoc mutant
  # nay.
  # Mutant nay BI BAT boi tier 'run': ben trong so_tu_khoa_trung_khop (tham
  # so la cau_hoi, doan -- KHONG CO "cac_chi_so_ung_vien"/"thu_tu"/"k" nao
  # trong scope nay), bieu thuc moi doc CA BA ten CHUA TON TAI -- da tu
  # chay THAT qua python3, xac nhan NameError "name 'cac_chi_so_ung_vien' is
  # not defined" ngay khi ham duoc goi lan dau. Ben trong rerank (KHONG CO
  # "tu_cau_hoi"/"tu_doan" nao trong scope nay, chi co tham so
  # cac_chi_so_ung_vien, cau_hoi, danh_sach_tai_lieu, k), bieu thuc moi
  # "return tu_cau_hoi & tu_doan" doc hai ten CHUA TON TAI -- da tu chay
  # THAT xac nhan NameError "name 'tu_cau_hoi' is not defined". Ca hai bi
  # chan boi tier 'run', doc lap voi static.
  # Da tu ra soat GOTCHA #6: tu_cau_hoi/tu_doan/thu_tu la ten bien CUC BO
  # RIENG cua hai ham moi trong bai nay, khong trung voi bat ky bien nao
  # trong toolkit BM25/cosine/RRF da copy tu q8.5e (ten bien o do la
  # v_cau_hoi, v_kho, tu_truy_van, hang_bm25, hang_cosine, v.v. -- khong
  # bien nao la "tu_cau_hoi"/"tu_doan"/"thu_tu").
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[0, 2, 1, 3\\]\\n\\[0, 2, 1\\]\\n\\[2, 0\\]\\nFalse\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`[2, 0]` — hạng `1` đổi từ đoạn `0` sang đoạn `2` chỉ nhờ MỘT tiêu chí khớp
từ khoá chính xác, áp CHỈ trong số `3` ứng viên hybrid đã chọn. Bài tiếp
theo ráp CẢ BỐN khái niệm — nạp, phân mảnh, cô lập, rerank — thành MỘT
pipeline đa khách hàng hoàn chỉnh.
::::

::::reflect{#nghi-lai}
Rerank không "sửa lỗi" của hybrid RRF — cả hai đều là những phép đo HỢP LỆ,
chỉ khác NHAU về tiêu chí. Hybrid RRF hỏi "tài liệu nào khá ở CẢ hai hệ
thống (BM25 + cosine)"; rerank hỏi một câu khác hẳn: "tài liệu nào khớp
CHÍNH XÁC nhất, từng từ một, với câu hỏi". Trên ví dụ này, đoạn `2` thua ở
câu hỏi đầu (chỉ hạng `2`) nhưng thắng áp đảo ở câu hỏi sau (`10` từ khoá
so với `5`) — đủ để đổi hẳn hạng `1`. Đây CHÍNH LÀ lý do `Chương 39.4` đặt
reranker SAU hybrid, không THAY THẾ nó: mỗi tầng bắt một loại tín hiệu khác
nhau, và tầng sau có cơ hội sửa một quyết định mà tầng trước KHÔNG THẤY
được.

Bài tiếp theo ráp lại CẢ BỐN mảnh của quest này — nạp theo sự kiện (bài `1`),
phân mảnh (bài `2`), cô lập theo khách hàng (bài `3`), và rerank (bài này)
— thành MỘT pipeline RAG đa khách hàng hoàn chỉnh, đo trên một kho có ÍT
NHẤT `2` khách hàng.
::::

::::checkpoint{mastery=0.82}
::::
