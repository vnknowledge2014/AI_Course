---
id: tri-tue-nhan-tao.hybrid-va-rerank.boss-hybrid-sua-ca-hai-diem-mu
title: "BOSS — Hybrid sửa CẢ HAI điểm mù: đóng q8.5e tại 4/4"
summary: "Kho MOI 8 doan, DUNG HAI cau hoi. Cau hoi A (ma hiem 'xk9902', diem mu CUA COSINE): BM25 xep doan dung hang 1 (diem 7,8154), cosine xep hang 2 (mot doan 'may tinh' khong lien quan thang voi cosine=0,7071) -- hybrid RRF khoi phuc hang 1. Cau hoi B (dien dat khac 'vi xu ly' + tu ngu 'hoi thao cong nghe' lam nhieu BM25, diem mu CUA BM25): BM25 xep doan dung hang 4 (bi doan 'hoi thao' lap tu danh lua, diem 12,5347 > 3,9077), cosine xep hang 1 (cosine=1,0, vi_xu_ly la 1 trong 16 muc TU_VUNG) -- hybrid RRF cai thien tu hang 4 len hang 2 (khong dat hang 1 vi doan 'hoi thao' van kha o CA HAI he thong). Ket luan so: hybrid_khong_te_hon_ca_hai True cho CA HAI cau hoi -- hybrid khong bao gio te hon cach TE NHAT trong hai cach rieng le tren du lieu nay."
locale: vi
track: tri-tue-nhan-tao
module: hybrid-va-rerank
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-hybrid-sua-ca-hai-diem-mu]
requires: [ai.sua-diem-mu-cosine-bang-hybrid]
concepts: [ai.boss-hybrid-sua-ca-hai-diem-mu]
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

::::byte{trigger=enter mood=happy pose=jump}
Ba bài: ý tưởng RRF, công thức đầy đủ, sửa điểm mù CỦA COSINE. Bài BOSS này
dựng MỘT kho mới với HAI câu hỏi — một phơi bày điểm mù của cosine (như bài
trước), một phơi bày điểm mù NGƯỢC LẠI của BM25 — và đóng `q8.5e` tại `4/4`.
::::

::::explain{#hai_diem_mu_tren_mot_kho}
q8.5d bài BOSS đã nêu tên cả HAI điểm mù, nhưng chỉ ĐO một: *"Cosine mù
trước một từ KHÔNG nằm trong từ vựng cố định của nó... BM25 (như mọi phép
khớp từ khoá thuần tuý) mù trước một CÁCH DIỄN ĐẠT khác đi cùng một ý."*
Bài này đo CẢ HAI, trên MỘT kho `8` đoạn duy nhất, bằng hai câu hỏi khác
nhau:

> **Câu hỏi A** (điểm mù CỦA COSINE) — chứa mã sản phẩm hiếm `"xk9902"`,
> giống hệt cơ chế `"sp4471"` (q8.5d, bài trước): BM25 khớp chính xác mã đó,
> cosine hoàn toàn mù trước nó (không có trong `TU_VUNG`).
>
> **Câu hỏi B** (điểm mù CỦA BM25) — hỏi về một "vi xu ly" (bộ xử lý,
> `1` trong `16` mục của `TU_VUNG`) NHƯNG dùng thêm cụm "hội thảo công nghệ"
> — một chủ đề PHỤ mà một đoạn KHÁC trong kho lặp lại rất nhiều lần. BM25
> cộng dồn điểm cho MỌI từ của câu hỏi, kể cả những từ không thực sự liên
> quan tới điều người hỏi cần — nó bị "ngợp" bởi đoạn lặp từ "hội thảo",
> xếp SAI. Cosine chỉ "thấy" đúng `16` mục từ vựng cố định — nó hoàn toàn
> không bị phân tâm bởi "hội thảo" (từ đó không nằm trong `TU_VUNG`), nên
> xếp ĐÚNG.

Hai câu hỏi này là ẢNH GƯƠNG của nhau: ở câu hỏi A, BM25 đúng — cosine sai;
ở câu hỏi B, cosine đúng — BM25 sai. Đo xem hybrid RRF xử lý CẢ HAI tình
huống ra sao, bằng số cụ thể.
::::

::::example{#do_ca_hai_cau_hoi}
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


def danh_gia_mot_cau_hoi(chi_so_muc_tieu, cau_hoi, kho, tu_vung, cum_tu_goc, k1, b, k_rrf):
    hang_bm25 = xep_hang_bm25(cau_hoi, kho, k1, b)
    hang_cosine = xep_hang_cosine(cau_hoi, kho, tu_vung, cum_tu_goc)
    hang_rrf = hang_rrf_cua_kho(cau_hoi, kho, tu_vung, cum_tu_goc, k1, b, k_rrf)
    return hang_cua(chi_so_muc_tieu, hang_bm25), hang_cua(chi_so_muc_tieu, hang_cosine), hang_cua(chi_so_muc_tieu, hang_rrf)


def hybrid_khong_te_hon_ca_hai(hang_bm25, hang_cosine, hang_rrf):
    return hang_rrf <= max(hang_bm25, hang_cosine)


KHO = [
    "san pham xk9902 la thiet bi cong nghe moi ra mat duoc nguoi dung danh gia tot",
    "vi xu ly gia re co san hien nay tren thi truong duoc nhieu nguoi lua chon",
    "hoi thao cong nghe hoi thao cong nghe hoi thao lon nhat nam nay quy tu chuyen gia",
    "may tinh hien dai chay phan mem nhanh giup cong viec hieu qua hon truoc",
    "lap trinh vien dung thuat toan de xu ly du lieu lon moi ngay",
    "mon an ngon phu thuoc vao gia vi va cong thuc nau chuan",
    "dau bep gioi luon can than khi chon nguyen lieu tuoi ngon",
    "nha bep sach se giup mon an ngon hon va an toan hon",
]
CAU_HOI_A = "toi muon hoi ve san pham xk9902 nay co dung duoc voi may tinh khong"
CAU_HOI_B = "vi xu ly nao tot cho hoi thao cong nghe lon nhat"

ket_qua_a = danh_gia_mot_cau_hoi(0, CAU_HOI_A, KHO, TU_VUNG, CUM_TU_GOC, 1.5, 0.75, 1)
ket_qua_b = danh_gia_mot_cau_hoi(1, CAU_HOI_B, KHO, TU_VUNG, CUM_TU_GOC, 1.5, 0.75, 1)

hop_le_a = hybrid_khong_te_hon_ca_hai(*ket_qua_a)
hop_le_b = hybrid_khong_te_hon_ca_hai(*ket_qua_b)

print(ket_qua_a)
print(hop_le_a)
print(ket_qua_b)
print(hop_le_b)
```

```text title=readonly
(1, 2, 1)
True
(4, 1, 2)
True
```

**Câu hỏi A** (`ket_qua_a = (1, 2, 1)`): BM25 xếp đoạn `0` (chứa `"xk9902"`)
ở hạng `1` (điểm `7,8154` — khớp mã hiếm cộng nhiều từ khác). Cosine xếp nó
hạng `2` — đoạn `3` (`"may tinh hien dai chay phan mem..."`, hoàn toàn KHÔNG
liên quan tới `xk9902`) tình cờ dùng `"may tinh"`/`"phan mem"`, cho cosine
`0,7071` — vượt qua đoạn `0` (cosine `0,0`, vector toàn số `0`). **RRF hợp
nhất khôi phục đúng hạng `1`** — đúng cơ chế đã đo ở bài trước.

**Câu hỏi B** (`ket_qua_b = (4, 1, 2)`): BM25 xếp đoạn `1` (chứa `"vi xu
ly"`, đoạn ĐÚNG) ở hạng `4` — đoạn `2` (`"hoi thao cong nghe"` lặp `3` lần)
khớp CẢ `4` từ `"hoi"`, `"thao"`, `"cong"`, `"nghe"` của câu hỏi nhiều lần,
cho điểm `12,5347` — GẤP hơn `3` lần điểm của đoạn `1` (`3,9077`, chỉ khớp
`"vi"`, `"xu"`, `"ly"` đúng `1` lần mỗi từ). Cosine xếp đoạn `1` hạng `1`
(cosine `1,0` — `"vi xu ly"` gộp thành đúng `1` mục `TU_VUNG`, không hề bị
"hội thảo công nghệ" làm nhiễu, vì các từ đó không nằm trong `16` mục từ
vựng). **RRF hợp nhất CẢI THIỆN đoạn `1` từ hạng `4` lên hạng `2`** — tốt
hơn nhiều so với BM25 riêng lẻ, nhưng KHÔNG đạt hẳn hạng `1` như cosine —
vì đoạn `2` (hạng `1` BM25, hạng `3` cosine — vẫn "khá" ở cả hai hệ thống)
có tổng RRF `0,75`, nhỉnh hơn đoạn `1` (hạng `4` BM25, hạng `1` cosine, tổng
`0,7`).

`hybrid_khong_te_hon_ca_hai` trả về `True` cho CẢ HAI câu hỏi: `1 ≤
max(1,2)=2` và `2 ≤ max(4,1)=4`. Kết luận số: trên dữ liệu này, hybrid
KHÔNG BAO GIỜ tệ hơn cách TỆ NHẤT trong hai cách riêng lẻ — và ở câu hỏi A,
nó còn TỐT BẰNG cách tốt nhất. Ở câu hỏi B, nó cải thiện rõ rệt (từ hạng `4`
lên hạng `2`) nhưng không chạm hẳn mức tốt nhất, vì một tài liệu THỨ BA (đủ
khá ở cả hai hệ thống) len vào giữa.
::::

::::predict{#doan_hybrid_cau_hoi_b commitOnce}
Xét đúng ví dụ trên: câu hỏi A có BM25 đúng (hạng `1`), cosine sai (hạng
`2`) — hybrid đưa đoạn đúng về hạng `1`. Câu hỏi B thì NGƯỢC LẠI: BM25 sai
(hạng `4`, bị đoạn "hội thảo" đánh lừa), cosine đúng (hạng `1`).

**Trước khi chạy thử**, bạn đoán: ở câu hỏi B, hybrid RRF sẽ đưa đoạn `1`
(đoạn đúng, chứa `"vi xu ly"`) về hạng mấy?

:::opt{correct}
Hạng `2` — cải thiện RÕ RỆT so với hạng `4` của BM25, nhưng KHÔNG đạt hẳn
hạng `1` như cosine, vì đoạn "hội thảo" (hạng `1` BM25, hạng `3` cosine) vẫn
đủ khá ở CẢ HAI hệ thống để giữ một vị trí cao trong tổng RRF, nhỉnh hơn
đoạn đúng một chút
:::

:::opt
Hạng `1`, giống hệt câu hỏi A — vì RRF luôn khôi phục đúng hạng tốt nhất
trong hai phương pháp riêng lẻ, không phụ thuộc tình huống cụ thể nào
::why
Gần đúng ở việc câu hỏi A THẬT SỰ có hybrid khôi phục đúng hạng `1` (bằng
đúng hạng tốt nhất, BM25) — quan sát đó đúng CHO CÂU HỎI A.

Chỗ lệch: đây không phải một QUY LUẬT chung "RRF luôn đạt đúng hạng tốt
nhất". Ở câu hỏi B, đoạn `2` (đoạn "hội thảo", SAI) có tổng RRF `0,75`
(hạng `1` BM25 cộng hạng `3` cosine), CAO HƠN đoạn `1` (đoạn ĐÚNG, tổng
`0,7` — hạng `4` BM25 cộng hạng `1` cosine). Đoạn `2` vẫn đủ "khá" ở cả hai
hệ thống để giữ hạng `1` trong RRF — hybrid chỉ đưa đoạn đúng lên hạng `2`,
không phải hạng `1`.
::
:::

:::opt
Hạng `4`, không đổi gì so với BM25 — vì BM25 đã bị đánh lừa quá nặng (điểm
gấp hơn `3` lần) nên RRF không đủ sức sửa
::why
Gần đúng ở việc chênh lệch điểm BM25 THẬT SỰ lớn (`12,5347` so với
`3,9077`, gấp hơn `3` lần) — quan sát về điểm số gốc đó đúng.

Chỗ lệch: RRF không dùng ĐỘ LỚN chênh lệch của điểm số gốc — nó chỉ dùng
THỨ HẠNG. Dù điểm BM25 chênh lệch gấp `3` lần, về THỨ HẠNG đoạn `1` chỉ kém
đoạn `2` đúng `3` bậc (hạng `4` so với hạng `1`) — và đóng góp `1/(1+1)=0,5`
từ hạng `1` cosine đủ mạnh để kéo đoạn `1` lên hạng `2`, KHÔNG giữ nguyên
hạng `4`.
::
:::
::::

::::code{#viet_danh_gia_va_hybrid_khong_te_hon}
Hoàn thiện `danh_gia_mot_cau_hoi` (tính CẢ BA hạng — BM25, cosine, RRF — của
một tài liệu cho một câu hỏi) và `hybrid_khong_te_hon_ca_hai` (kiểm xem hạng
RRF có tệ hơn cách TỆ NHẤT trong hai cách riêng lẻ hay không).

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


def danh_gia_mot_cau_hoi(chi_so_muc_tieu, cau_hoi, kho, tu_vung, cum_tu_goc, k1, b, k_rrf):
    hang_bm25 = xep_hang_bm25(cau_hoi, kho, k1, b)
    hang_cosine = xep_hang_cosine(cau_hoi, kho, tu_vung, cum_tu_goc)
    hang_rrf = ___                                          # hang_rrf_cua_kho(cau_hoi, kho, tu_vung, cum_tu_goc, k1, b, k_rrf)
    return hang_cua(chi_so_muc_tieu, hang_bm25), hang_cua(chi_so_muc_tieu, hang_cosine), hang_cua(chi_so_muc_tieu, hang_rrf)


def hybrid_khong_te_hon_ca_hai(hang_bm25, hang_cosine, hang_rrf):
    return ___                                              # hang_rrf <= max(hang_bm25, hang_cosine)


KHO = [
    "san pham xk9902 la thiet bi cong nghe moi ra mat duoc nguoi dung danh gia tot",
    "vi xu ly gia re co san hien nay tren thi truong duoc nhieu nguoi lua chon",
    "hoi thao cong nghe hoi thao cong nghe hoi thao lon nhat nam nay quy tu chuyen gia",
    "may tinh hien dai chay phan mem nhanh giup cong viec hieu qua hon truoc",
    "lap trinh vien dung thuat toan de xu ly du lieu lon moi ngay",
    "mon an ngon phu thuoc vao gia vi va cong thuc nau chuan",
    "dau bep gioi luon can than khi chon nguyen lieu tuoi ngon",
    "nha bep sach se giup mon an ngon hon va an toan hon",
]
CAU_HOI_A = "toi muon hoi ve san pham xk9902 nay co dung duoc voi may tinh khong"
CAU_HOI_B = "vi xu ly nao tot cho hoi thao cong nghe lon nhat"

ket_qua_a = danh_gia_mot_cau_hoi(0, CAU_HOI_A, KHO, TU_VUNG, CUM_TU_GOC, 1.5, 0.75, 1)
ket_qua_b = danh_gia_mot_cau_hoi(1, CAU_HOI_B, KHO, TU_VUNG, CUM_TU_GOC, 1.5, 0.75, 1)

hop_le_a = hybrid_khong_te_hon_ca_hai(*ket_qua_a)
hop_le_b = hybrid_khong_te_hon_ca_hai(*ket_qua_b)

print(ket_qua_a)
print(hop_le_a)
print(ket_qua_b)
print(hop_le_b)
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


def danh_gia_mot_cau_hoi(chi_so_muc_tieu, cau_hoi, kho, tu_vung, cum_tu_goc, k1, b, k_rrf):
    hang_bm25 = xep_hang_bm25(cau_hoi, kho, k1, b)
    hang_cosine = xep_hang_cosine(cau_hoi, kho, tu_vung, cum_tu_goc)
    hang_rrf = hang_rrf_cua_kho(cau_hoi, kho, tu_vung, cum_tu_goc, k1, b, k_rrf)
    return hang_cua(chi_so_muc_tieu, hang_bm25), hang_cua(chi_so_muc_tieu, hang_cosine), hang_cua(chi_so_muc_tieu, hang_rrf)


def hybrid_khong_te_hon_ca_hai(hang_bm25, hang_cosine, hang_rrf):
    return hang_rrf <= max(hang_bm25, hang_cosine)


KHO = [
    "san pham xk9902 la thiet bi cong nghe moi ra mat duoc nguoi dung danh gia tot",
    "vi xu ly gia re co san hien nay tren thi truong duoc nhieu nguoi lua chon",
    "hoi thao cong nghe hoi thao cong nghe hoi thao lon nhat nam nay quy tu chuyen gia",
    "may tinh hien dai chay phan mem nhanh giup cong viec hieu qua hon truoc",
    "lap trinh vien dung thuat toan de xu ly du lieu lon moi ngay",
    "mon an ngon phu thuoc vao gia vi va cong thuc nau chuan",
    "dau bep gioi luon can than khi chon nguyen lieu tuoi ngon",
    "nha bep sach se giup mon an ngon hon va an toan hon",
]
CAU_HOI_A = "toi muon hoi ve san pham xk9902 nay co dung duoc voi may tinh khong"
CAU_HOI_B = "vi xu ly nao tot cho hoi thao cong nghe lon nhat"

ket_qua_a = danh_gia_mot_cau_hoi(0, CAU_HOI_A, KHO, TU_VUNG, CUM_TU_GOC, 1.5, 0.75, 1)
ket_qua_b = danh_gia_mot_cau_hoi(1, CAU_HOI_B, KHO, TU_VUNG, CUM_TU_GOC, 1.5, 0.75, 1)

hop_le_a = hybrid_khong_te_hon_ca_hai(*ket_qua_a)
hop_le_b = hybrid_khong_te_hon_ca_hai(*ket_qua_b)

print(ket_qua_a)
print(hop_le_a)
print(ket_qua_b)
print(hop_le_b)
```

```python title=test
assert ket_qua_a == (1, 2, 1), f"cau hoi A: (BM25, cosine, RRF) phai la (1, 2, 1) -- dang ra {ket_qua_a}"
assert ket_qua_b == (4, 1, 2), f"cau hoi B: (BM25, cosine, RRF) phai la (4, 1, 2) -- dang ra {ket_qua_b}"
assert hop_le_a == True, "hybrid o cau hoi A khong duoc te hon hang TE NHAT trong hai cach rieng le (max(1,2)=2)"
assert hop_le_b == True, "hybrid o cau hoi B khong duoc te hon hang TE NHAT trong hai cach rieng le (max(4,1)=4)"
assert ket_qua_a[2] == ket_qua_a[0], "cau hoi A: RRF phai dat DUNG hang tot nhat (BM25, hang 1)"
assert ket_qua_b[2] < ket_qua_b[0], "cau hoi B: RRF phai CAI THIEN so voi BM25 rieng le (hang 2 tot hon hang 4)"
assert ket_qua_b[2] > ket_qua_b[1], "cau hoi B: RRF khong dat hang tot nhat (cosine, hang 1) -- van con mot doan thu ba len truoc"

# kiem tra truc tiep hybrid_khong_te_hon_ca_hai tren cac vi du nho, tu tinh tay duoc
assert hybrid_khong_te_hon_ca_hai(3, 5, 4) == True, "4 <= max(3,5)=5 phai la True"
assert hybrid_khong_te_hon_ca_hai(1, 1, 1) == True, "1 <= max(1,1)=1 phai la True"
assert hybrid_khong_te_hon_ca_hai(2, 4, 5) == False, "5 <= max(2,4)=4 phai la False -- day la ca RRF TE HON ca hai"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `danh_gia_mot_cau_hoi`) là hạng RRF — tái sử dụng `hang_rrf_cua_kho` (bài trước) với đúng bảy đối số nó cần. Chỗ hai (trong `hybrid_khong_te_hon_ca_hai`) là điều kiện so sánh — hạng RRF có KHÔNG LỚN HƠN (`<=`, nhớ hạng thấp hơn = tốt hơn) hạng TỆ NHẤT (`max`) trong hai hạng riêng lẻ hay không.
- kind: strategy
  body: 'Chỗ đầu: `hang_rrf_cua_kho(cau_hoi, kho, tu_vung, cum_tu_goc, k1, b, k_rrf)` — đúng bảy tham số theo đúng thứ tự đã định nghĩa ở bài trước. Chỗ hai: `hang_rrf <= max(hang_bm25, hang_cosine)` — `max` của hai hạng là hạng TỆ NHẤT (số lớn hơn), và `<=` kiểm RRF không vượt quá nó.'
- kind: one-line
  body: 'Chỗ đầu là `hang_rrf_cua_kho(cau_hoi, kho, tu_vung, cum_tu_goc, k1, b, k_rrf)`, chỗ hai là `hang_rrf <= max(hang_bm25, hang_cosine)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT hang_rrf_cua_kho(...) voi dung bay doi so (khong duoc goi lai xep_hang_bm25/xep_hang_cosine rieng le); VA cho trong hai phai SO SANH bang '<=' hang RRF voi max(hang_bm25, hang_cosine) (khong duoc dung toan tu khac hay chep san True/False)
  requireAst:
  - kind: uses-call, target: hang_rrf_cua_kho, min: 1
  - kind: uses-operator, target: "<=", min: 1
  - kind: uses-call, target: max, min: 1
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [1, 1, 1] cho ba
  # luat theo dung thu tu khai bao o tren.
  # hang_rrf_cua_kho=1: CHI mot lan GOI THAT trong toan bo solution, dung o
  # cho trong dau. Ham nay khong tu goi lai chinh no.
  # "<="=1: CHI mot lan duy nhat trong toan bo solution, dung o cho trong
  # hai. Khong co toan tu so sanh nao khac trung dau '<=' o boilerplate (cac
  # ham khac chi dung '==' de kiem vector-khong, khong dung '<=').
  # max=1: CHI mot lan GOI THAT (ham max cua Python), dung o cho trong hai.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua ham
  # _dem de xac nhan, khong doan tay): dien "hang_rrf <= max(hang_bm25,
  # hang_cosine)" vao cho trong dau ("hang_rrf = hang_rrf <= max(hang_bm25,
  # hang_cosine)" trong danh_gia_mot_cau_hoi) VA dien
  # "hang_rrf_cua_kho(cau_hoi, kho, tu_vung, cum_tu_goc, k1, b, k_rrf)" vao
  # cho trong hai ("return hang_rrf_cua_kho(cau_hoi, kho, tu_vung, cum_tu_goc,
  # k1, b, k_rrf)" trong hybrid_khong_te_hon_ca_hai) -- tong so lan goi
  # hang_rrf_cua_kho, tong so lan '<=', VA tong so lan goi max DEU KHONG DOI
  # (van la 1, 1, 1 -- chi doi VI TRI). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong danh_gia_mot_cau_hoi, dong
  # "hang_rrf = hang_rrf <= max(hang_bm25, hang_cosine)" DOC bien 'hang_rrf'
  # o VE PHAI TRUOC KHI no duoc GAN (no la bien local do chinh dong nay tao
  # ra) -- da tu chay THAT mutant nay qua python3, xac nhan no nem
  # UnboundLocalError ("cannot access local variable 'hang_rrf' where it is
  # not associated with a value") ngay khi danh_gia_mot_cau_hoi(...) chay tam
  # dong nay; ben trong hybrid_khong_te_hon_ca_hai (tham so hang_bm25,
  # hang_cosine, hang_rrf -- KHONG co cau_hoi, kho, tu_vung, cum_tu_goc, k1,
  # b, k_rrf), bieu thuc moi "hang_rrf_cua_kho(cau_hoi, ...)" dung nhieu ten
  # CHUA TON TAI trong scope nay, nem NameError ("name 'cau_hoi' is not
  # defined"). Ca hai bi chan boi tier 'run', doc lap voi static.
  # Da tu ra soat GOTCHA #6 cho ca ba luat uses-call/uses-operator: trong
  # pham vi hai ham nay, khong co bien nao khac cung "hinh dang" (cung so
  # doi so, cung kieu tra ve) co the tinh co khop cac luat tren -- rui ro
  # nay da duoc ra soat va KHONG ap dung o day.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\(1, 2, 1\\)\\nTrue\\n\\(4, 1, 2\\)\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`(1, 2, 1)` và `(4, 1, 2)`, cả hai `hop_le` đều `True` — hybrid không bao
giờ tệ hơn cách tệ nhất trong hai cách riêng lẻ, và ở câu hỏi mà BM25 đã
đúng sẵn, hybrid giữ nguyên độ chính xác đó. Quest `hybrid-va-rerank` (q8.5e)
đóng tại `4/4`, bàn giao sang `boss-rag-tu-so-0` (q8.5f) — BOSS đóng CẢ
track `T8.5` tại `34/34`.
::::

::::reflect{#nghi-lai}
Bốn bài đã trả lời từng mảnh một câu hỏi mở đầu: vì sao không cộng điểm số
trực tiếp (bài `1` — hai thang đo khác nhau, cộng thẳng để một thang đo lấn
át tuỳ tiện); công thức RRF đầy đủ (bài `2` — hợp nhất nhiều hệ thống bằng
thứ hạng thuần, cho ra một thứ tự thứ ba); sửa điểm mù của cosine (bài `3`
— khôi phục đúng hạng của một mã sản phẩm hiếm mà cosine không thấy được);
và bài BOSS này — đo CẢ HAI điểm mù trên MỘT kho, HAI câu hỏi khác nhau.

Kết luận không phải "hybrid luôn hoàn hảo". Ở câu hỏi A, hybrid đạt ĐÚNG mức
tốt nhất (hạng `1`, bằng BM25). Ở câu hỏi B, hybrid chỉ đạt hạng `2` — cải
thiện RÕ so với hạng `4` của BM25 riêng lẻ, nhưng không chạm hạng `1` như
cosine riêng lẻ, vì một tài liệu KHÔNG liên quan (đoạn "hội thảo") vẫn đủ
khá ở CẢ HAI hệ thống để giữ một vị trí cao. Điều GIỮ VỮNG qua cả hai câu
hỏi, đo bằng số cụ thể (không suy đoán): hybrid KHÔNG BAO GIỜ tệ hơn cách
TỆ NHẤT trong hai cách riêng lẻ (`hybrid_khong_te_hon_ca_hai` luôn `True`).
Đó là lý do ráp cả hai — không phải vì hybrid luôn thắng tuyệt đối, mà vì nó
không bao giờ phải gánh trọn điểm mù của MỘT phương pháp duy nhất.

Quest `hybrid-va-rerank` (q8.5e) đóng tại `4/4`. Track `T8.5` "RAG từ số 0"
tiếp tục với `boss-rag-tu-so-0` (q8.5f, `5` bài, BOSS đóng CẢ track) — ráp
TOÀN BỘ pipeline: chunking + vector tự chế (q8.5a) → chỉ mục HNSW (q8.5c) →
tìm kiếm hybrid BM25 + vector (q8.5e, vừa xong) → ghép các đoạn tìm được vào
một prompt có cấu trúc → LLM mô phỏng trả lời DỰA TRÊN ngữ cảnh vừa truy
xuất.
::::

::::checkpoint{mastery=0.9}
::::
