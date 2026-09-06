---
id: tri-tue-nhan-tao.boss-rag-tu-so-0.tim-kiem-hybrid-rag
title: "Tìm kiếm hybrid BM25+HNSW qua RRF: khi 'vắng mặt' lần đầu xảy ra thật"
summary: "Cau hoi 'vi xu ly va bao hanh cua may tinh bang zt8821 la bao lau' (chua chi tiet cu the '18 thang', doan 11). Vet can cosine (q8.5b) top-4: [1,0,6,2] -- KHONG co doan 11. HNSW (q8.5c) top-4: [1,0,6,2] -- GIONG HET vet can (recall@4=100%), cung KHONG co doan 11 -- ca hai deu mu truoc chi tiet nay. BM25 (q8.5d) xep doan 11 hang 1 (diem cao nhat). Hybrid RRF (q8.5e, k_rrf=1) hop nhat BM25 (xep hang TOAN BO kho) voi HNSW (CHI top-4, phan con lai vang mat, dong gop 0 -- ham moi hang_trong_topk thay vi hang_cua de tranh ValueError tu .index()) cho top-4 CUOI CUNG: [1,0,6,11] -- doan 11 duoc CUU vao pipeline, thay the doan 2 (chi lien quan qua cosine, khong chua chi tiet can)."
locale: vi
track: tri-tue-nhan-tao
module: boss-rag-tu-so-0
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.tim-kiem-hybrid-rag]
requires: [ai.xay-chi-muc-hnsw-cho-rag]
concepts: [ai.tim-kiem-hybrid-rag]
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
Chỉ mục HNSW đã xây (bài trước). Bây giờ đặt MỘT câu hỏi cụ thể — có chứa
chi tiết riêng của `zt8821` — và chạy CẢ vét cạn, CẢ HNSW, CẢ BM25, rồi hợp
nhất qua RRF. Đây là nơi ĐẦU TIÊN trong cả track quy tắc "vắng mặt = đóng góp
`0`" (q8.5e bài `2`) áp dụng THẬT SỰ.
::::

::::explain{#hop_nhat_khi_mot_he_thong_khong_xep_toan_bo}
Câu hỏi của bài này: `"vi xu ly va bao hanh cua may tinh bang zt8821 la bao
lau"` — hỏi về thời hạn bảo hành, chứa CHI TIẾT CỤ THỂ cần thiết (đoạn `11`,
`"...la 18 thang"`), cộng thêm hai từ CÓ nằm trong `TU_VUNG` (`"vi xu ly"`,
`"may tinh"`) để vector câu hỏi không toàn số `0`.

q8.5e bài `2` đã nêu quy tắc: *"Nếu `d` KHÔNG xuất hiện trong một hệ thống
nào đó, đóng góp của hệ thống đó là `0`... nhưng"* — bài đó cũng nói rõ —
*"không tài liệu nào bị 'vắng mặt' ở đây"*, vì cả `xep_hang_bm25` và
`xep_hang_cosine` xếp hạng TOÀN BỘ kho. Bài NÀY là nơi khác: `tim_kiem_hnsw`
(q8.5c) chỉ trả về **top-`k`** — KHÔNG PHẢI một thứ hạng đầy đủ cho mọi đoạn.
Nếu dùng thẳng `hang_cua` (gọi `danh_sach.index(chi_so)`) trên một đoạn
KHÔNG nằm trong top-`k` đó, `.index()` ném `ValueError` ngay lập tức — đã tự
xác nhận điều này bằng `python3` thật trước khi viết bài.

Cách xử lý: một hàm nhỏ, `hang_trong_topk(chi_so, danh_sach_topk)` — trả về
hạng CHỈ TÍNH trong số top-`k` đó nếu `chi_so` có mặt, và `None` (KHÔNG PHẢI
lỗi) nếu vắng mặt. `dong_gop_rrf_hoac_vang` biến `None` thành đóng góp `0,0`
đúng quy tắc đã học; hạng còn lại (`None`) không bao giờ được đưa vào công
thức `1/(k+hang)`. `diem_hybrid_mot_doan` cộng đóng góp từ BM25 (dùng
`hang_cua` bình thường, vì `xep_hang_bm25` xếp hạng TOÀN BỘ kho, không có
vắng mặt) với đóng góp từ HNSW (dùng `hang_trong_topk`, có thể vắng mặt).
::::

::::example{#do_ca_bon_cach_tim_kiem}
```python title=readonly
import math
import random

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


def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


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


def tim_k_lan_can_vet_can(vector_cau_hoi, danh_sach_vector_kho, k):
    do_tuong_dong = [tuong_dong_cosine(vector_cau_hoi, v) for v in danh_sach_vector_kho]
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k]


def co_lang_gieng_gan_hon(do_thi, vectors, vector_muc_tieu, nut_hien_tai):
    tot_nhat = nut_hien_tai
    for hang_xom in do_thi[nut_hien_tai]:
        if tuong_dong_cosine(vectors[hang_xom], vector_muc_tieu) > tuong_dong_cosine(vectors[tot_nhat], vector_muc_tieu):
            tot_nhat = hang_xom
    if tot_nhat == nut_hien_tai:
        return None
    return tot_nhat


def tim_tham_lam(do_thi, vectors, vector_muc_tieu, diem_bat_dau):
    hien_tai = diem_bat_dau
    duong_di = [hien_tai]
    while True:
        ke_tiep = co_lang_gieng_gan_hon(do_thi, vectors, vector_muc_tieu, hien_tai)
        if ke_tiep is None:
            break
        hien_tai = ke_tiep
        duong_di.append(hien_tai)
    return hien_tai, duong_di


def gan_tang_ngau_nhien(seed, n_diem, xac_suat):
    rng = random.Random(seed)
    tang = []
    for i in range(n_diem):
        t = 0
        while rng.random() < xac_suat:
            t += 1
        tang.append(t)
    return tang


def noi_canh_va_cat(do_thi_tang, i, j, vectors, M):
    if j not in do_thi_tang[i]:
        do_thi_tang[i].append(j)
    if i not in do_thi_tang[j]:
        do_thi_tang[j].append(i)
    for x in (i, j):
        if len(do_thi_tang[x]) > M:
            xep = sorted(do_thi_tang[x], key=lambda n: tuong_dong_cosine(vectors[x], vectors[n]), reverse=True)
            do_thi_tang[x] = xep[:M]


def chen_hnsw(vectors, tang_diem, M):
    so_tang_max = max(tang_diem) + 1
    do_thi = [dict() for _ in range(so_tang_max)]
    diem_vao = None
    tang_cao_nhat = -1
    for i in range(len(vectors)):
        L = tang_diem[i]
        for t in range(L + 1):
            do_thi[t].setdefault(i, [])
        if diem_vao is None:
            diem_vao = i
            tang_cao_nhat = L
            continue
        hien_tai = diem_vao
        for t in range(tang_cao_nhat, L, -1):
            hien_tai, _ = tim_tham_lam(do_thi[t], vectors, vectors[i], hien_tai)
        for t in range(min(L, tang_cao_nhat), -1, -1):
            hien_tai, _ = tim_tham_lam(do_thi[t], vectors, vectors[i], hien_tai)
            noi_canh_va_cat(do_thi[t], i, hien_tai, vectors, M)
        if L > tang_cao_nhat:
            diem_vao = i
            tang_cao_nhat = L
    return do_thi, diem_vao, tang_cao_nhat


def tim_kiem_hnsw(do_thi, vectors, vector_muc_tieu, diem_vao, tang_cao_nhat, k):
    hien_tai = diem_vao
    tong_so_sanh = 0
    for t in range(tang_cao_nhat, 0, -1):
        hien_tai, duong_di = tim_tham_lam(do_thi[t], vectors, vector_muc_tieu, hien_tai)
        tong_so_sanh += sum(len(do_thi[t][n]) for n in duong_di)
    hien_tai, duong_di = tim_tham_lam(do_thi[0], vectors, vector_muc_tieu, hien_tai)
    tong_so_sanh += sum(len(do_thi[0][n]) for n in duong_di)
    ung_vien = set([hien_tai]) | set(do_thi[0][hien_tai])
    tong_so_sanh += len(ung_vien)
    xep = sorted(ung_vien, key=lambda j: tuong_dong_cosine(vectors[j], vector_muc_tieu), reverse=True)
    return xep[:k], tong_so_sanh


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


def hang_cua(chi_so_muc_tieu, thu_tu_xep_hang):
    return thu_tu_xep_hang.index(chi_so_muc_tieu) + 1


def dong_gop_rrf(hang, k):
    return 1 / (k + hang)


def hang_trong_topk(chi_so, danh_sach_topk):
    if chi_so in danh_sach_topk:
        return danh_sach_topk.index(chi_so) + 1
    return None


def dong_gop_rrf_hoac_vang(hang, k):
    if hang is None:
        return 0.0
    return dong_gop_rrf(hang, k)


def diem_hybrid_mot_doan(chi_so, hang_bm25, topk_hnsw, k_rrf):
    return dong_gop_rrf(hang_cua(chi_so, hang_bm25), k_rrf) + dong_gop_rrf_hoac_vang(hang_trong_topk(chi_so, topk_hnsw), k_rrf)


def xep_hang_hybrid(kho, hang_bm25, topk_hnsw, k_rrf):
    diem = [diem_hybrid_mot_doan(i, hang_bm25, topk_hnsw, k_rrf) for i in range(len(kho))]
    return sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)


TAI_LIEU_MOI = (
    "cong ty ra mat san pham may tinh bang zt8821 vao thang nay. "
    "may tinh bang zt8821 dung vi xu ly moi manh hon the he truoc. "
    "phan mem tren zt8821 duoc lap trinh toi uu cho ung dung van phong. "
    "gia ban chinh thuc cua zt8821 la 8990000 dong. "
    "ket noi mang cua zt8821 ho tro ca wifi va 4g. "
    "du lieu nguoi dung tren zt8821 duoc luu tru bang thuat toan ma hoa rieng. "
    "vi xu ly cua zt8821 giup ung dung khoi dong nhanh hon. "
    "lap trinh vien co the viet phan mem moi cho zt8821 qua bo cong cu rieng. "
    "thuat toan nen du lieu tren zt8821 tiet kiem bo nho luu tru. "
    "ung dung camera cua zt8821 xu ly anh bang thuat toan tri tue nhan tao. "
    "ket noi mang khong day cua zt8821 on dinh trong pham vi rong. "
    "che do bao hanh chinh thuc cua zt8821 la 18 thang. "
    "du lieu ban dau cho thay zt8821 ban duoc 50000 chiec trong tuan dau. "
    "cong ty cam ket cap nhat phan mem cho zt8821 trong 3 nam."
)

DOAN = chia_theo_cau(TAI_LIEU_MOI)
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DOAN]

tang_diem = gan_tang_ngau_nhien(26, len(VEC), 0.5)
do_thi, diem_vao, tang_cao_nhat = chen_hnsw(VEC, tang_diem, 3)

CAU_HOI = "vi xu ly va bao hanh cua may tinh bang zt8821 la bao lau"
K = 4
qv = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)

topk_vetcan = tim_k_lan_can_vet_can(qv, VEC, K)
topk_hnsw, so_sanh_hnsw = tim_kiem_hnsw(do_thi, VEC, qv, diem_vao, tang_cao_nhat, K)
bm25_rank = xep_hang_bm25(CAU_HOI, DOAN, 1.5, 0.75)
xep_hybrid = xep_hang_hybrid(DOAN, bm25_rank, topk_hnsw, 1)
topk_hybrid = xep_hybrid[:K]

print(topk_vetcan)
print(topk_hnsw, so_sanh_hnsw)
print(bm25_rank)
print(topk_hybrid)
print(11 in topk_vetcan, 11 in topk_hnsw, 11 in topk_hybrid)
```

```text title=readonly
[1, 0, 6, 2]
[1, 0, 6, 2] 11
[11, 1, 6, 0, 9, 4, 3, 10, 5, 2, 7, 8, 12, 13]
[1, 0, 6, 11]
False False True
```

Vét cạn cosine (q8.5b) top-`4`: `[1, 0, 6, 2]` — KHÔNG chứa đoạn `11`. HNSW
(q8.5c) top-`4`: `[1, 0, 6, 2]` — GIỐNG HỆT vét cạn (recall@`4` = `100%`,
`11` phép so sánh thay vì quét hết `14` điểm) — nhưng cũng KHÔNG chứa đoạn
`11`: cả hai cách tìm bằng vector đều mù trước chi tiết bảo hành, đúng như
bài `1`/`2` đã thiết lập (vector đoạn `11` toàn số `0`). BM25 xếp đoạn `11`
ở **hạng `1`** — điểm cao nhất trong toàn kho, vì nó khớp CHÍNH XÁC cả
`"zt8821"` lẫn `"bao hanh"`. Hybrid RRF hợp nhất BM25 (xếp hạng toàn bộ `14`
đoạn) với HNSW (chỉ top-`4`, đoạn `2` có mặt còn đoạn `11` VẮNG MẶT — đóng
góp `0` từ phía HNSW cho đoạn `11`) — kết quả top-`4` cuối cùng:
`[1, 0, 6, 11]`. Đoạn `11` được CỨU vào pipeline nhờ điểm BM25 áp đảo, thay
thế đoạn `2` (chỉ liên quan qua cosine, không chứa chi tiết cần).
::::

::::predict{#doan_valueerror_neu_dung_sai_ham commitOnce}
Giả sử (một lỗi thiết kế) ai đó dùng `hang_cua(11, topk_hnsw)` — hàm CŨ
(q8.5d/q8.5e), gọi thẳng `topk_hnsw.index(11)` — thay vì `hang_trong_topk(11,
topk_hnsw)`, trên đúng `topk_hnsw = [1, 0, 6, 2]` ở ví dụ trên (đoạn `11`
KHÔNG có trong danh sách này).

**Trước khi chạy thử**, bạn đoán: điều gì xảy ra?

:::opt{correct}
Chương trình ném `ValueError` ngay lập tức — `hang_cua` gọi thẳng
`thu_tu_xep_hang.index(chi_so_muc_tieu)` mà không kiểm tra `chi_so_muc_tieu`
có mặt trong danh sách hay không trước; `list.index(...)` trên một giá trị
KHÔNG có trong danh sách luôn ném lỗi, không có giá trị mặc định nào được trả
về
:::

:::opt
Trả về một hạng "vô cực" quy ước (ví dụ lớn hơn độ dài danh sách `+1`) — một
cách ngầm hiểu để báo "không tìm thấy"
::why
Gần đúng ở TRỰC GIÁC rằng hệ thống cần một cách biểu diễn "vắng mặt" — trực
giác đó đúng, và chính là lý do `hang_trong_topk` (bài này) trả về `None`
cho trường hợp đó.

Chỗ lệch: `hang_cua`, đúng như đã viết ở q8.5d/q8.5e, KHÔNG có cơ chế quy
ước nào như vậy — nó chỉ có đúng một dòng `thu_tu_xep_hang.index(...) + 1`.
`list.index()` của Python không bao giờ tự tạo ra một "hạng vô cực" — nó ném
`ValueError` thẳng, không có bước trung gian nào khác.
::
:::

:::opt
Trả về `0` — coi như đóng góp RRF của đoạn vắng mặt luôn là `0`, đúng quy
tắc đã học ở q8.5e bài `2`
::why
Gần đúng ở việc quy tắc ĐÚNG cho hệ thống này THẬT SỰ là "đóng góp `0` khi
vắng mặt" — quan sát về QUY TẮC mong muốn đó đúng, và đó chính xác là điều
`dong_gop_rrf_hoac_vang` làm được.

Chỗ lệch: câu hỏi này hỏi về `hang_cua` — hàm CŨ, không hề biết gì về quy
tắc "vắng mặt = `0`". `hang_cua` chỉ là một lệnh gọi `.index()` trần trụi;
quy tắc "vắng mặt = `0`" phải được LẬP TRÌNH RA (chính là `hang_trong_topk`
+ `dong_gop_rrf_hoac_vang`, bài này) — nó không tự động có sẵn chỉ vì đã học
qua lý thuyết ở bài trước.
::
:::
::::

::::code{#viet_hybrid_rrf_vang_mat}
Hoàn thiện `hang_trong_topk` (kiểm tra CÓ MẶT trước khi tra hạng, tránh
`ValueError`) và `diem_hybrid_mot_doan` (cộng đóng góp BM25 với đóng góp
HNSW — có thể vắng mặt, đóng góp `0`).

```python title=starter
import math
import random

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


def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


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


def tim_k_lan_can_vet_can(vector_cau_hoi, danh_sach_vector_kho, k):
    do_tuong_dong = [tuong_dong_cosine(vector_cau_hoi, v) for v in danh_sach_vector_kho]
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k]


def co_lang_gieng_gan_hon(do_thi, vectors, vector_muc_tieu, nut_hien_tai):
    tot_nhat = nut_hien_tai
    for hang_xom in do_thi[nut_hien_tai]:
        if tuong_dong_cosine(vectors[hang_xom], vector_muc_tieu) > tuong_dong_cosine(vectors[tot_nhat], vector_muc_tieu):
            tot_nhat = hang_xom
    if tot_nhat == nut_hien_tai:
        return None
    return tot_nhat


def tim_tham_lam(do_thi, vectors, vector_muc_tieu, diem_bat_dau):
    hien_tai = diem_bat_dau
    duong_di = [hien_tai]
    while True:
        ke_tiep = co_lang_gieng_gan_hon(do_thi, vectors, vector_muc_tieu, hien_tai)
        if ke_tiep is None:
            break
        hien_tai = ke_tiep
        duong_di.append(hien_tai)
    return hien_tai, duong_di


def gan_tang_ngau_nhien(seed, n_diem, xac_suat):
    rng = random.Random(seed)
    tang = []
    for i in range(n_diem):
        t = 0
        while rng.random() < xac_suat:
            t += 1
        tang.append(t)
    return tang


def noi_canh_va_cat(do_thi_tang, i, j, vectors, M):
    if j not in do_thi_tang[i]:
        do_thi_tang[i].append(j)
    if i not in do_thi_tang[j]:
        do_thi_tang[j].append(i)
    for x in (i, j):
        if len(do_thi_tang[x]) > M:
            xep = sorted(do_thi_tang[x], key=lambda n: tuong_dong_cosine(vectors[x], vectors[n]), reverse=True)
            do_thi_tang[x] = xep[:M]


def chen_hnsw(vectors, tang_diem, M):
    so_tang_max = max(tang_diem) + 1
    do_thi = [dict() for _ in range(so_tang_max)]
    diem_vao = None
    tang_cao_nhat = -1
    for i in range(len(vectors)):
        L = tang_diem[i]
        for t in range(L + 1):
            do_thi[t].setdefault(i, [])
        if diem_vao is None:
            diem_vao = i
            tang_cao_nhat = L
            continue
        hien_tai = diem_vao
        for t in range(tang_cao_nhat, L, -1):
            hien_tai, _ = tim_tham_lam(do_thi[t], vectors, vectors[i], hien_tai)
        for t in range(min(L, tang_cao_nhat), -1, -1):
            hien_tai, _ = tim_tham_lam(do_thi[t], vectors, vectors[i], hien_tai)
            noi_canh_va_cat(do_thi[t], i, hien_tai, vectors, M)
        if L > tang_cao_nhat:
            diem_vao = i
            tang_cao_nhat = L
    return do_thi, diem_vao, tang_cao_nhat


def tim_kiem_hnsw(do_thi, vectors, vector_muc_tieu, diem_vao, tang_cao_nhat, k):
    hien_tai = diem_vao
    tong_so_sanh = 0
    for t in range(tang_cao_nhat, 0, -1):
        hien_tai, duong_di = tim_tham_lam(do_thi[t], vectors, vector_muc_tieu, hien_tai)
        tong_so_sanh += sum(len(do_thi[t][n]) for n in duong_di)
    hien_tai, duong_di = tim_tham_lam(do_thi[0], vectors, vector_muc_tieu, hien_tai)
    tong_so_sanh += sum(len(do_thi[0][n]) for n in duong_di)
    ung_vien = set([hien_tai]) | set(do_thi[0][hien_tai])
    tong_so_sanh += len(ung_vien)
    xep = sorted(ung_vien, key=lambda j: tuong_dong_cosine(vectors[j], vector_muc_tieu), reverse=True)
    return xep[:k], tong_so_sanh


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


def hang_cua(chi_so_muc_tieu, thu_tu_xep_hang):
    return thu_tu_xep_hang.index(chi_so_muc_tieu) + 1


def dong_gop_rrf(hang, k):
    return 1 / (k + hang)


def hang_trong_topk(chi_so, danh_sach_topk):
    if chi_so ___ danh_sach_topk:                          # in
        return danh_sach_topk.index(chi_so) + 1
    return None


def dong_gop_rrf_hoac_vang(hang, k):
    if hang is None:
        return 0.0
    return dong_gop_rrf(hang, k)


def diem_hybrid_mot_doan(chi_so, hang_bm25, topk_hnsw, k_rrf):
    return dong_gop_rrf(hang_cua(chi_so, hang_bm25), k_rrf) + ___    # dong_gop_rrf_hoac_vang(hang_trong_topk(chi_so, topk_hnsw), k_rrf)


def xep_hang_hybrid(kho, hang_bm25, topk_hnsw, k_rrf):
    diem = [diem_hybrid_mot_doan(i, hang_bm25, topk_hnsw, k_rrf) for i in range(len(kho))]
    return sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)


TAI_LIEU_MOI = (
    "cong ty ra mat san pham may tinh bang zt8821 vao thang nay. "
    "may tinh bang zt8821 dung vi xu ly moi manh hon the he truoc. "
    "phan mem tren zt8821 duoc lap trinh toi uu cho ung dung van phong. "
    "gia ban chinh thuc cua zt8821 la 8990000 dong. "
    "ket noi mang cua zt8821 ho tro ca wifi va 4g. "
    "du lieu nguoi dung tren zt8821 duoc luu tru bang thuat toan ma hoa rieng. "
    "vi xu ly cua zt8821 giup ung dung khoi dong nhanh hon. "
    "lap trinh vien co the viet phan mem moi cho zt8821 qua bo cong cu rieng. "
    "thuat toan nen du lieu tren zt8821 tiet kiem bo nho luu tru. "
    "ung dung camera cua zt8821 xu ly anh bang thuat toan tri tue nhan tao. "
    "ket noi mang khong day cua zt8821 on dinh trong pham vi rong. "
    "che do bao hanh chinh thuc cua zt8821 la 18 thang. "
    "du lieu ban dau cho thay zt8821 ban duoc 50000 chiec trong tuan dau. "
    "cong ty cam ket cap nhat phan mem cho zt8821 trong 3 nam."
)

DOAN = chia_theo_cau(TAI_LIEU_MOI)
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DOAN]

tang_diem = gan_tang_ngau_nhien(26, len(VEC), 0.5)
do_thi, diem_vao, tang_cao_nhat = chen_hnsw(VEC, tang_diem, 3)

CAU_HOI = "vi xu ly va bao hanh cua may tinh bang zt8821 la bao lau"
K = 4
qv = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)

topk_vetcan = tim_k_lan_can_vet_can(qv, VEC, K)
topk_hnsw, so_sanh_hnsw = tim_kiem_hnsw(do_thi, VEC, qv, diem_vao, tang_cao_nhat, K)
bm25_rank = xep_hang_bm25(CAU_HOI, DOAN, 1.5, 0.75)
xep_hybrid = xep_hang_hybrid(DOAN, bm25_rank, topk_hnsw, 1)
topk_hybrid = xep_hybrid[:K]

print(topk_vetcan)
print(topk_hnsw, so_sanh_hnsw)
print(bm25_rank)
print(topk_hybrid)
print(11 in topk_vetcan, 11 in topk_hnsw, 11 in topk_hybrid)
```

```python title=solution
import math
import random

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


def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


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


def tim_k_lan_can_vet_can(vector_cau_hoi, danh_sach_vector_kho, k):
    do_tuong_dong = [tuong_dong_cosine(vector_cau_hoi, v) for v in danh_sach_vector_kho]
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k]


def co_lang_gieng_gan_hon(do_thi, vectors, vector_muc_tieu, nut_hien_tai):
    tot_nhat = nut_hien_tai
    for hang_xom in do_thi[nut_hien_tai]:
        if tuong_dong_cosine(vectors[hang_xom], vector_muc_tieu) > tuong_dong_cosine(vectors[tot_nhat], vector_muc_tieu):
            tot_nhat = hang_xom
    if tot_nhat == nut_hien_tai:
        return None
    return tot_nhat


def tim_tham_lam(do_thi, vectors, vector_muc_tieu, diem_bat_dau):
    hien_tai = diem_bat_dau
    duong_di = [hien_tai]
    while True:
        ke_tiep = co_lang_gieng_gan_hon(do_thi, vectors, vector_muc_tieu, hien_tai)
        if ke_tiep is None:
            break
        hien_tai = ke_tiep
        duong_di.append(hien_tai)
    return hien_tai, duong_di


def gan_tang_ngau_nhien(seed, n_diem, xac_suat):
    rng = random.Random(seed)
    tang = []
    for i in range(n_diem):
        t = 0
        while rng.random() < xac_suat:
            t += 1
        tang.append(t)
    return tang


def noi_canh_va_cat(do_thi_tang, i, j, vectors, M):
    if j not in do_thi_tang[i]:
        do_thi_tang[i].append(j)
    if i not in do_thi_tang[j]:
        do_thi_tang[j].append(i)
    for x in (i, j):
        if len(do_thi_tang[x]) > M:
            xep = sorted(do_thi_tang[x], key=lambda n: tuong_dong_cosine(vectors[x], vectors[n]), reverse=True)
            do_thi_tang[x] = xep[:M]


def chen_hnsw(vectors, tang_diem, M):
    so_tang_max = max(tang_diem) + 1
    do_thi = [dict() for _ in range(so_tang_max)]
    diem_vao = None
    tang_cao_nhat = -1
    for i in range(len(vectors)):
        L = tang_diem[i]
        for t in range(L + 1):
            do_thi[t].setdefault(i, [])
        if diem_vao is None:
            diem_vao = i
            tang_cao_nhat = L
            continue
        hien_tai = diem_vao
        for t in range(tang_cao_nhat, L, -1):
            hien_tai, _ = tim_tham_lam(do_thi[t], vectors, vectors[i], hien_tai)
        for t in range(min(L, tang_cao_nhat), -1, -1):
            hien_tai, _ = tim_tham_lam(do_thi[t], vectors, vectors[i], hien_tai)
            noi_canh_va_cat(do_thi[t], i, hien_tai, vectors, M)
        if L > tang_cao_nhat:
            diem_vao = i
            tang_cao_nhat = L
    return do_thi, diem_vao, tang_cao_nhat


def tim_kiem_hnsw(do_thi, vectors, vector_muc_tieu, diem_vao, tang_cao_nhat, k):
    hien_tai = diem_vao
    tong_so_sanh = 0
    for t in range(tang_cao_nhat, 0, -1):
        hien_tai, duong_di = tim_tham_lam(do_thi[t], vectors, vector_muc_tieu, hien_tai)
        tong_so_sanh += sum(len(do_thi[t][n]) for n in duong_di)
    hien_tai, duong_di = tim_tham_lam(do_thi[0], vectors, vector_muc_tieu, hien_tai)
    tong_so_sanh += sum(len(do_thi[0][n]) for n in duong_di)
    ung_vien = set([hien_tai]) | set(do_thi[0][hien_tai])
    tong_so_sanh += len(ung_vien)
    xep = sorted(ung_vien, key=lambda j: tuong_dong_cosine(vectors[j], vector_muc_tieu), reverse=True)
    return xep[:k], tong_so_sanh


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


def hang_cua(chi_so_muc_tieu, thu_tu_xep_hang):
    return thu_tu_xep_hang.index(chi_so_muc_tieu) + 1


def dong_gop_rrf(hang, k):
    return 1 / (k + hang)


def hang_trong_topk(chi_so, danh_sach_topk):
    if chi_so in danh_sach_topk:
        return danh_sach_topk.index(chi_so) + 1
    return None


def dong_gop_rrf_hoac_vang(hang, k):
    if hang is None:
        return 0.0
    return dong_gop_rrf(hang, k)


def diem_hybrid_mot_doan(chi_so, hang_bm25, topk_hnsw, k_rrf):
    return dong_gop_rrf(hang_cua(chi_so, hang_bm25), k_rrf) + dong_gop_rrf_hoac_vang(hang_trong_topk(chi_so, topk_hnsw), k_rrf)


def xep_hang_hybrid(kho, hang_bm25, topk_hnsw, k_rrf):
    diem = [diem_hybrid_mot_doan(i, hang_bm25, topk_hnsw, k_rrf) for i in range(len(kho))]
    return sorted(range(len(kho)), key=lambda i: diem[i], reverse=True)


TAI_LIEU_MOI = (
    "cong ty ra mat san pham may tinh bang zt8821 vao thang nay. "
    "may tinh bang zt8821 dung vi xu ly moi manh hon the he truoc. "
    "phan mem tren zt8821 duoc lap trinh toi uu cho ung dung van phong. "
    "gia ban chinh thuc cua zt8821 la 8990000 dong. "
    "ket noi mang cua zt8821 ho tro ca wifi va 4g. "
    "du lieu nguoi dung tren zt8821 duoc luu tru bang thuat toan ma hoa rieng. "
    "vi xu ly cua zt8821 giup ung dung khoi dong nhanh hon. "
    "lap trinh vien co the viet phan mem moi cho zt8821 qua bo cong cu rieng. "
    "thuat toan nen du lieu tren zt8821 tiet kiem bo nho luu tru. "
    "ung dung camera cua zt8821 xu ly anh bang thuat toan tri tue nhan tao. "
    "ket noi mang khong day cua zt8821 on dinh trong pham vi rong. "
    "che do bao hanh chinh thuc cua zt8821 la 18 thang. "
    "du lieu ban dau cho thay zt8821 ban duoc 50000 chiec trong tuan dau. "
    "cong ty cam ket cap nhat phan mem cho zt8821 trong 3 nam."
)

DOAN = chia_theo_cau(TAI_LIEU_MOI)
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DOAN]

tang_diem = gan_tang_ngau_nhien(26, len(VEC), 0.5)
do_thi, diem_vao, tang_cao_nhat = chen_hnsw(VEC, tang_diem, 3)

CAU_HOI = "vi xu ly va bao hanh cua may tinh bang zt8821 la bao lau"
K = 4
qv = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)

topk_vetcan = tim_k_lan_can_vet_can(qv, VEC, K)
topk_hnsw, so_sanh_hnsw = tim_kiem_hnsw(do_thi, VEC, qv, diem_vao, tang_cao_nhat, K)
bm25_rank = xep_hang_bm25(CAU_HOI, DOAN, 1.5, 0.75)
xep_hybrid = xep_hang_hybrid(DOAN, bm25_rank, topk_hnsw, 1)
topk_hybrid = xep_hybrid[:K]

print(topk_vetcan)
print(topk_hnsw, so_sanh_hnsw)
print(bm25_rank)
print(topk_hybrid)
print(11 in topk_vetcan, 11 in topk_hnsw, 11 in topk_hybrid)
```

```python title=test
assert topk_vetcan == [1, 0, 6, 2], f"topk_vetcan sai -- dang ra {topk_vetcan}"
assert topk_hnsw == [1, 0, 6, 2], f"topk_hnsw sai -- dang ra {topk_hnsw}"
assert so_sanh_hnsw == 11, f"so phep so sanh HNSW phai la 11 -- dang ra {so_sanh_hnsw}"
assert bm25_rank == [11, 1, 6, 0, 9, 4, 3, 10, 5, 2, 7, 8, 12, 13], f"bm25_rank sai -- dang ra {bm25_rank}"
assert topk_hybrid == [1, 0, 6, 11], f"topk_hybrid sai -- dang ra {topk_hybrid}"

# bien: ca vet can LAN HNSW deu mu truoc doan 11 -- CHI hybrid cuu duoc no
assert 11 not in topk_vetcan, "vet can cosine khong duoc chua doan 11 (vector toan so 0, khong the 'thay')"
assert 11 not in topk_hnsw, "HNSW khong duoc chua doan 11 (cung mu vi cosine)"
assert 11 in topk_hybrid, "hybrid PHAI cuu duoc doan 11 nho diem BM25 ap dao"

# kiem tra truc tiep hang_trong_topk tren vi du nho, tu tinh tay duoc
assert hang_trong_topk(5, [2, 5, 9]) == 2, "5 o vi tri thu 2 (chi so 1) trong [2,5,9] -- hang phai la 2"
assert hang_trong_topk(2, [2, 5, 9]) == 1, "2 o vi tri dau -- hang phai la 1"
assert hang_trong_topk(7, [2, 5, 9]) is None, "7 khong co trong [2,5,9] -- phai tra ve None, KHONG duoc nem loi"

# kiem tra dong_gop_rrf_hoac_vang: vang mat phai la 0, co mat phai dung cong thuc cu
assert dong_gop_rrf_hoac_vang(None, 1) == 0.0, "vang mat (hang=None) phai dong gop dung 0,0"
assert dong_gop_rrf_hoac_vang(1, 1) == 0.5, "co mat hang 1, k=1 phai dong gop 1/(1+1)=0,5, dung cong thuc dong_gop_rrf"

# bien: diem_hybrid_mot_doan tren mot vi du nho, tu tinh tay duoc -- doan 2
# hang 1 trong bm25 (dong gop 1/(1+1)=0,5) NHUNG vang mat trong topk_hnsw=[0,1]
# (dong gop 0) -> tong dung bang 0,5
assert diem_hybrid_mot_doan(2, [2, 0, 1], [0, 1], 1) == 0.5, "doan 2: co mat BM25 (hang 1, dong gop 0,5) nhung vang mat HNSW (dong gop 0) -- tong phai la 0,5"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `hang_trong_topk`) là toán tử kiểm `chi_so` có nằm trong `danh_sach_topk` hay không — PHẢI kiểm TRƯỚC khi gọi `.index()`, để tránh `ValueError` khi vắng mặt. Chỗ hai (trong `diem_hybrid_mot_doan`) là đóng góp từ HNSW — gọi `hang_trong_topk` để lấy hạng (có thể `None`), rồi `dong_gop_rrf_hoac_vang` để biến `None` thành `0,0`.
- kind: strategy
  body: 'Chỗ đầu: `in` — `if chi_so in danh_sach_topk:`. Chỗ hai: `dong_gop_rrf_hoac_vang(hang_trong_topk(chi_so, topk_hnsw), k_rrf)` — lấy hạng CÓ THỂ vắng mặt trong top-k, rồi cộng đóng góp (có thể `0`) vào đóng góp BM25 đã có.'
- kind: one-line
  body: 'Chỗ đầu là `in`, chỗ hai là `dong_gop_rrf_hoac_vang(hang_trong_topk(chi_so, topk_hnsw), k_rrf)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai kiem tra CO MAT bang toan tu 'in' TRUOC khi goi .index() (khong duoc goi thang .index() ma khong kiem tra, se nem ValueError khi vang mat); VA cho trong hai phai GOI THAT ca hang_trong_topk VA dong_gop_rrf_hoac_vang de cong don dong gop HNSW co the vang mat (khong duoc dung hang_cua hay chep san mot so co dinh)
  requireAst:
  - kind: uses-operator, target: "in", min: 5
  - kind: uses-call, target: dong_gop_rrf_hoac_vang, min: 1
  - kind: uses-call, target: hang_trong_topk, min: 1
  # Da thu that (dung dem_ast.mjs goi THANG kiemAst() that qua dist build +
  # pyodide that, chay tren CHINH van ban solution da trich tu file nay) --
  # ket qua [5, 1, 1] cho ba luat theo dung thu tu khai bao o tren.
  # "in"=5 (TONG THAT, da xac nhan bang cong cu, khong doan tay): mot lan
  # CHINH la cho trong dau ("if chi_so in danh_sach_topk:"), mot lan CO SAN
  # trong "dem_so_doan_chua_tu" ("tu in doan.lower().split()"), VA BA lan
  # trong dong in cuoi cung ("print(11 in topk_vetcan, 11 in topk_hnsw, 11 in
  # topk_hybrid)" -- ba phep so sanh 'in' rieng biet trong CUNG mot loi goi
  # print). Neu chi dat min=1 (ngay tho), mot mutant BO cho trong dau (vi du
  # dien mot dieu kien khac khong dung 'in', roi de .index() nem loi khi
  # vang mat) van co the qua NEU khong cham toi nhanh vang mat trong du lieu
  # nay -- nhung thuc te mutant do se nem ValueError THAT (xem phan run duoi
  # day) nen bi chan doc lap; van dat dung min=5 (TONG THAT) de khong dua
  # hoan toan vao hanh vi runtime rieng le.
  # dong_gop_rrf_hoac_vang=1, hang_trong_topk=1: moi ham CHI duoc GOI dung 1
  # lan trong toan bo solution, chinh la o cho trong hai (dinh nghia "def
  # dong_gop_rrf_hoac_vang(...)"/"def hang_trong_topk(...)" la ten ham, khong
  # phai Call, khong dem) -- day la TONG THAT, khong co boilerplate nao khac
  # goi lai hai ham nay.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua
  # python3 de xac nhan, khong doan tay): dien
  # "dong_gop_rrf_hoac_vang(hang_trong_topk(chi_so, topk_hnsw), k_rrf)" vao
  # cho trong dau ("if chi_so dong_gop_rrf_hoac_vang(hang_trong_topk(chi_so,
  # topk_hnsw), k_rrf) danh_sach_topk:") VA dien "in" vao cho trong hai
  # ("return dong_gop_rrf(hang_cua(chi_so, hang_bm25), k_rrf) + in") -- CA
  # HAI deu SAI CU PHAP NGAY LAP TUC: cho trong dau tao ra hai bieu thuc lien
  # tiep khong co toan tu noi ("chi_so EXPR danh_sach_topk", khong hop le);
  # cho trong hai dat tu khoa 'in' ngay sau toan tu '+' (khong phai mot bieu
  # thuc hop le). Da tu chay qua `ast.parse` (python3) de xac nhan: nem
  # SyntaxError ngay o buoc phan tich cu phap, TRUOC CA khi kiemAst() hay
  # tier 'run' kip chay -- khac voi da so mutant hoan doi khac trong quest
  # nay (thuong la NameError o tier 'run'), o day hai cho trong co VAI TRO CU
  # PHAP hoan toan khac nhau (mot ben la TOAN TU so sanh, mot ben la MOT BIEU
  # THUC goi ham) nen hoan doi khong the tao ra ma hop le duoc.
  # Da tu ra soat GOTCHA #6: khong co bien/ham nao khac trong pham vi bai nay
  # co the tinh co thay the dong_gop_rrf_hoac_vang/hang_trong_topk (hai ham
  # nay co ten VA chu ky doc nhat, khong trung voi bat ky ham nao khac).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[1, 0, 6, 2\\]\\n\\[1, 0, 6, 2\\] 11\\n\\[11, 1, 6, 0, 9, 4, 3, 10, 5, 2, 7, 8, 12, 13\\]\\n\\[1, 0, 6, 11\\]\\nFalse False True\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`False False True` — vét cạn và HNSW đều mù trước đoạn `11`, hybrid RRF là
CÁCH DUY NHẤT cứu được nó vào top-`4`. Bài sau ghép đúng bốn đoạn này thành
một prompt có cấu trúc.
::::

::::reflect{#nghi-lai}
Bài này là nơi ĐẦU TIÊN trong cả track quy tắc "vắng mặt = đóng góp `0`"
(q8.5e bài `2`) áp dụng THẬT SỰ — vì `tim_kiem_hnsw` chỉ trả về top-`k`, khác
với `xep_hang_bm25`/`xep_hang_cosine` (xếp hạng TOÀN BỘ kho). Dùng thẳng
`hang_cua` cũ cho trường hợp này sẽ ném `ValueError` — đã tự xác nhận bằng
`python3` thật, không suy đoán — nên cần một hàm mới, nhỏ, CHỈ làm đúng một
việc: `hang_trong_topk` trả `None` thay vì lỗi khi vắng mặt.

Kết quả đo được củng cố đúng điều hai bài đầu tiên đã thiết lập: một đoạn có
vector toàn số `0` (đoạn `11`, thời hạn bảo hành) hoàn toàn nằm ngoài tầm
với của CẢ vét cạn LẪN HNSW — hai kỹ thuật vector khác nhau, cùng một điểm
mù. BM25 (khớp từ khoá chính xác `"zt8821"`, `"bao hanh"`) không hề mù trước
nó. Hybrid RRF không cần "biết" trước đoạn nào đúng — nó chỉ cộng dồn đóng
góp từ MỌI hệ thống sẵn có, và đủ để đoạn `11` vượt lên trên đoạn `2` (chỉ
liên quan hời hợt qua cosine). Bài sau dùng đúng bốn đoạn này (`[1, 0, 6,
11]`) để dựng một prompt có cấu trúc.
::::

::::checkpoint{mastery=0.85}
::::
