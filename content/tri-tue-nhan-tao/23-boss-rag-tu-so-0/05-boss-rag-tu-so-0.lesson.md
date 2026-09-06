---
id: tri-tue-nhan-tao.boss-rag-tu-so-0.boss-rag-tu-so-0
title: "BOSS — RAG từ số 0: LLM mô phỏng CÓ so với KHÔNG có ngữ cảnh, đóng T8.5 tại 34/34"
summary: "LLM mo phong (ham THUAN, tat dinh, tra bang/regex -- dung khuon T8.4, khong mang that): neu prompt CO 'Ngu canh:' VA trich duoc '(\\d+ thang)' ngay sau 'bao hanh' trong phan ngu canh, tra ve cau tra loi CHUA dung thoi han; neu khong, tra ve cau co dinh 'toi khong co thong tin...'. tra_loi_co_rag = llm_mo_phong(prompt tu bai 4, chua ca 'zt8821' lan '18 thang') = 'che do bao hanh cua san pham zt8821 la 18 thang' -- CHUA '18 thang'. tra_loi_khong_rag = llm_mo_phong(CAU_HOI tran, khong ngu canh) = 'toi khong co thong tin...' -- KHONG chua '18 thang'. Doi chieu bang assert: '18 thang' in tra_loi_co_rag (True) va '18 thang' not in tra_loi_khong_rag (True). Dong quest q8.5f tai 5/5 va CA track T8.5 tai 34/34."
locale: vi
track: tri-tue-nhan-tao
module: boss-rag-tu-so-0
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-rag-tu-so-0]
requires: [ai.dung-prompt-co-cau-truc]
concepts: [ai.boss-rag-tu-so-0]
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
Bốn bài: chunking + vector trên tài liệu mới, chỉ mục HNSW, tìm kiếm hybrid
qua RRF, prompt có cấu trúc. Bài này ráp MẢNH CUỐI CÙNG — một "LLM" mô
phỏng — và đo bằng số: có ngữ cảnh (RAG) so với không có, khác nhau ra sao
trên đúng câu hỏi cần chi tiết cụ thể. Đóng `q8.5f` tại `5/5`, và đóng CẢ
track `T8.5` tại `34/34`.
::::

::::explain{#llm_mo_phong_co_rag_khong_rag}
Xuyên suốt `T8.4` (`dùng LLM đúng cách`), MỌI "LLM" trong track này đều là
một hàm Python THUẦN, TẤT ĐỊNH, tra bảng hoặc áp luật cố định — không mạng
nơ-ron thật, không API thật, không mô phỏng độ trễ mạng (bài BOSS `T8.4`,
`boss-tro-ly-dung-llm-dung-cach`, đã nêu rõ nguyên tắc này và áp dụng xuyên
suốt `28` bài). Bài này dùng ĐÚNG khuôn đó cho `llm_mo_phong`:

> Nếu prompt đưa vào CÓ chứa đánh dấu `"Ngu canh:"` (nghĩa là CÓ ngữ cảnh
> được truy xuất, tức có-RAG) VÀ trích được một con số thời hạn ngay sau cụm
> `"bao hanh"` TRONG phần ngữ cảnh đó (dùng biểu thức chính quy — tra cứu
> chuỗi con, KHÔNG "hiểu" ngôn ngữ) — trả về một câu trả lời CHỨA đúng thời
> hạn đó.
>
> Nếu KHÔNG (không có đánh dấu `"Ngu canh:"` — nghĩa là chỉ có câu hỏi trần,
> không-RAG; HOẶC có ngữ cảnh nhưng không trích được thời hạn) — trả về MỘT
> câu cố định: `"toi khong co thong tin de tra loi cau hoi nay"`.

Đo trên ĐÚNG một câu hỏi (`CAU_HOI`, bài `3`/`4`): `tra_loi_co_rag =
llm_mo_phong(prompt)` (prompt đầy đủ, bài `4` — CHỨA `"18 thang"` nhờ hybrid
RRF cứu được đoạn `11`) so với `tra_loi_khong_rag = llm_mo_phong(CAU_HOI)`
(câu hỏi TRẦN, không kèm bất kỳ ngữ cảnh nào — mô phỏng đúng tình huống
"không có RAG", một LLM phải tự đoán mà không có tài liệu nào để tra cứu —
tái hiện phép đo ảo giác của q8.4e). Chi tiết cụ thể cần đối chiếu:
`CHI_TIET_CU_THE = "18 thang"`.
::::

::::example{#do_co_rag_khong_rag}
```python title=readonly
import math
import random
import re

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


def dung_prompt(cac_doan, cau_hoi):
    ngu_canh = "\n".join(cac_doan)
    return f"Ngu canh:\n{ngu_canh}\n\nCau hoi: {cau_hoi}\nTra loi dua TREN ngu canh tren:"


CHI_TIET_CU_THE = "18 thang"
CAU_TRA_LOI_KHONG_BIET = "toi khong co thong tin de tra loi cau hoi nay"


def trich_thoi_han_bao_hanh(ngu_canh):
    m = re.search(r"bao hanh.*?(\d+ thang)", ngu_canh)
    if m:
        return m.group(1)
    return None


def llm_mo_phong(prompt_hoac_cau_hoi):
    if "Ngu canh:" not in prompt_hoac_cau_hoi:
        return CAU_TRA_LOI_KHONG_BIET
    ngu_canh = prompt_hoac_cau_hoi.split("Cau hoi:")[0]
    thoi_han = trich_thoi_han_bao_hanh(ngu_canh)
    if thoi_han is None:
        return CAU_TRA_LOI_KHONG_BIET
    return f"che do bao hanh cua san pham zt8821 la {thoi_han}"


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

topk_hnsw, so_sanh_hnsw = tim_kiem_hnsw(do_thi, VEC, qv, diem_vao, tang_cao_nhat, K)
bm25_rank = xep_hang_bm25(CAU_HOI, DOAN, 1.5, 0.75)
xep_hybrid = xep_hang_hybrid(DOAN, bm25_rank, topk_hnsw, 1)
topk_hybrid = xep_hybrid[:K]

cac_doan_top = [DOAN[i] for i in topk_hybrid]
prompt = dung_prompt(cac_doan_top, CAU_HOI)

tra_loi_co_rag = llm_mo_phong(prompt)
tra_loi_khong_rag = llm_mo_phong(CAU_HOI)

co_rag_dung = CHI_TIET_CU_THE in tra_loi_co_rag
khong_rag_dung = CHI_TIET_CU_THE in tra_loi_khong_rag
ket_luan = co_rag_dung and not khong_rag_dung

print(tra_loi_co_rag)
print(tra_loi_khong_rag)
print(co_rag_dung, khong_rag_dung)
print(ket_luan)
```

```text title=readonly
che do bao hanh cua san pham zt8821 la 18 thang
toi khong co thong tin de tra loi cau hoi nay
True False
True
```

**Có RAG** (`tra_loi_co_rag`): `llm_mo_phong(prompt)` — `prompt` (bài `4`)
chứa đánh dấu `"Ngu canh:"` VÀ trích được `"18 thang"` ngay sau `"bao hanh"`
trong đoạn `11` (được hybrid RRF cứu vào top-`4` ở bài `3`) — trả về
`"che do bao hanh cua san pham zt8821 la 18 thang"`, CHỨA đúng chi tiết cần.

**Không RAG** (`tra_loi_khong_rag`): `llm_mo_phong(CAU_HOI)` — `CAU_HOI`
(câu hỏi trần) KHÔNG chứa `"Ngu canh:"` — hàm trả về NGAY câu cố định
`"toi khong co thong tin de tra loi cau hoi nay"`, không hề "đoán" ra
`"18 thang"` — đúng như một LLM thật, khi không có tài liệu tham khảo nào,
không thể biết một chi tiết CỤ THỂ (thời hạn bảo hành của một sản phẩm hư
cấu) mà nó chưa từng "học" qua.

`co_rag_dung=True`, `khong_rag_dung=False` — CÓ RAG trả lời đúng, KHÔNG RAG
từ chối đúng cách (không bịa). `ket_luan=True`: RAG cải thiện khả năng trả
lời đúng chi tiết cụ thể, đo bằng số cụ thể — không suy đoán.
::::

::::predict{#doan_khong_rag_tra_loi_gi commitOnce}
Xét đúng `CAU_HOI = "vi xu ly va bao hanh cua may tinh bang zt8821 la bao
lau"` — một câu hỏi TRẦN, không kèm theo bất kỳ đoạn ngữ cảnh nào (không có
chuỗi `"Ngu canh:"` ở đâu cả).

**Trước khi chạy thử**, bạn đoán: `llm_mo_phong(CAU_HOI)` trả về gì?

:::opt{correct}
`"toi khong co thong tin de tra loi cau hoi nay"` — bước kiểm ĐẦU TIÊN của
`llm_mo_phong` là `if "Ngu canh:" not in prompt_hoac_cau_hoi:`; `CAU_HOI`
không chứa chuỗi đó, nên hàm trả về NGAY câu cố định này, không hề chạm tới
bước tách `"Cau hoi:"` hay trích thời hạn nào cả
:::

:::opt
`llm_mo_phong` sẽ tự tìm trong CHÍNH `CAU_HOI` xem có từ khoá `"zt8821"` hay
`"bao hanh"` hay không, và nếu có, vẫn cố trả lời dựa trên những gì nó "nhớ"
được về sản phẩm này
::why
Gần đúng ở trực giác rằng một hệ thống LLM thật CÓ THỂ có kiến thức nội tại
về một số chủ đề — đó là một khả năng THẬT của các mô hình ngôn ngữ lớn nói
chung.

Chỗ lệch: `llm_mo_phong`, đúng như đã viết ở bài này, KHÔNG có kiến thức nội
tại nào về `"zt8821"` — nó chỉ có đúng MỘT nguồn thông tin: phần văn bản
đứng SAU đánh dấu `"Ngu canh:"`. Không có đánh dấu đó, hàm không có cơ chế
nào khác để "tra cứu" — nó trả về câu từ chối cố định NGAY LẬP TỨC, đúng
mô phỏng một LLM không có tài liệu tham khảo nào cho một chi tiết nó chưa
từng "học" qua.
::
:::

:::opt
Chương trình sẽ ném lỗi, vì `CAU_HOI` không có định dạng `"Cau hoi:"` như
một prompt thật, nên bước tách chuỗi `prompt_hoac_cau_hoi.split("Cau
hoi:")[0]` sẽ thất bại
::why
Gần đúng ở việc `CAU_HOI` THẬT SỰ không có định dạng đầy đủ của một prompt
— quan sát về CẤU TRÚC đó đúng.

Chỗ lệch: `llm_mo_phong` kiểm tra `"Ngu canh:"` TRƯỚC TIÊN, và trả về NGAY
nếu không thấy — dòng `prompt_hoac_cau_hoi.split("Cau hoi:")[0]` không bao
giờ được CHẠM TỚI trong trường hợp này (dòng lệnh nằm SAU nhánh `return` đã
thoát hàm). Không có lỗi nào xảy ra, vì bước có thể gây lỗi không hề chạy.
::
:::
::::

::::code{#viet_boss_rag_ket_luan}
Hoàn thiện phần cuối: gọi `llm_mo_phong` cho CẢ hai trường hợp (có prompt
đầy đủ, và chỉ câu hỏi trần), rồi tổng hợp thành một kết luận duy nhất.

```python title=starter
import math
import random
import re

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


def dung_prompt(cac_doan, cau_hoi):
    ngu_canh = "\n".join(cac_doan)
    return f"Ngu canh:\n{ngu_canh}\n\nCau hoi: {cau_hoi}\nTra loi dua TREN ngu canh tren:"


CHI_TIET_CU_THE = "18 thang"
CAU_TRA_LOI_KHONG_BIET = "toi khong co thong tin de tra loi cau hoi nay"


def trich_thoi_han_bao_hanh(ngu_canh):
    m = re.search(r"bao hanh.*?(\d+ thang)", ngu_canh)
    if m:
        return m.group(1)
    return None


def llm_mo_phong(prompt_hoac_cau_hoi):
    if "Ngu canh:" not in prompt_hoac_cau_hoi:
        return CAU_TRA_LOI_KHONG_BIET
    ngu_canh = prompt_hoac_cau_hoi.split("Cau hoi:")[0]
    thoi_han = trich_thoi_han_bao_hanh(ngu_canh)
    if thoi_han is None:
        return CAU_TRA_LOI_KHONG_BIET
    return f"che do bao hanh cua san pham zt8821 la {thoi_han}"


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

topk_hnsw, so_sanh_hnsw = tim_kiem_hnsw(do_thi, VEC, qv, diem_vao, tang_cao_nhat, K)
bm25_rank = xep_hang_bm25(CAU_HOI, DOAN, 1.5, 0.75)
xep_hybrid = xep_hang_hybrid(DOAN, bm25_rank, topk_hnsw, 1)
topk_hybrid = xep_hybrid[:K]

cac_doan_top = [DOAN[i] for i in topk_hybrid]
prompt = dung_prompt(cac_doan_top, CAU_HOI)

tra_loi_co_rag = ___                                        # llm_mo_phong(prompt)
tra_loi_khong_rag = ___                                      # llm_mo_phong(CAU_HOI)

co_rag_dung = CHI_TIET_CU_THE in tra_loi_co_rag
khong_rag_dung = CHI_TIET_CU_THE in tra_loi_khong_rag
ket_luan = ___                                              # co_rag_dung and not khong_rag_dung

print(tra_loi_co_rag)
print(tra_loi_khong_rag)
print(co_rag_dung, khong_rag_dung)
print(ket_luan)
```

```python title=solution
import math
import random
import re

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


def dung_prompt(cac_doan, cau_hoi):
    ngu_canh = "\n".join(cac_doan)
    return f"Ngu canh:\n{ngu_canh}\n\nCau hoi: {cau_hoi}\nTra loi dua TREN ngu canh tren:"


CHI_TIET_CU_THE = "18 thang"
CAU_TRA_LOI_KHONG_BIET = "toi khong co thong tin de tra loi cau hoi nay"


def trich_thoi_han_bao_hanh(ngu_canh):
    m = re.search(r"bao hanh.*?(\d+ thang)", ngu_canh)
    if m:
        return m.group(1)
    return None


def llm_mo_phong(prompt_hoac_cau_hoi):
    if "Ngu canh:" not in prompt_hoac_cau_hoi:
        return CAU_TRA_LOI_KHONG_BIET
    ngu_canh = prompt_hoac_cau_hoi.split("Cau hoi:")[0]
    thoi_han = trich_thoi_han_bao_hanh(ngu_canh)
    if thoi_han is None:
        return CAU_TRA_LOI_KHONG_BIET
    return f"che do bao hanh cua san pham zt8821 la {thoi_han}"


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

topk_hnsw, so_sanh_hnsw = tim_kiem_hnsw(do_thi, VEC, qv, diem_vao, tang_cao_nhat, K)
bm25_rank = xep_hang_bm25(CAU_HOI, DOAN, 1.5, 0.75)
xep_hybrid = xep_hang_hybrid(DOAN, bm25_rank, topk_hnsw, 1)
topk_hybrid = xep_hybrid[:K]

cac_doan_top = [DOAN[i] for i in topk_hybrid]
prompt = dung_prompt(cac_doan_top, CAU_HOI)

tra_loi_co_rag = llm_mo_phong(prompt)
tra_loi_khong_rag = llm_mo_phong(CAU_HOI)

co_rag_dung = CHI_TIET_CU_THE in tra_loi_co_rag
khong_rag_dung = CHI_TIET_CU_THE in tra_loi_khong_rag
ket_luan = co_rag_dung and not khong_rag_dung

print(tra_loi_co_rag)
print(tra_loi_khong_rag)
print(co_rag_dung, khong_rag_dung)
print(ket_luan)
```

```python title=test
assert tra_loi_co_rag == "che do bao hanh cua san pham zt8821 la 18 thang", f"tra_loi_co_rag sai -- dang ra {tra_loi_co_rag!r}"
assert tra_loi_khong_rag == "toi khong co thong tin de tra loi cau hoi nay", f"tra_loi_khong_rag sai -- dang ra {tra_loi_khong_rag!r}"
assert co_rag_dung == True, "co_rag_dung phai la True -- CO RAG phai tra loi CHUA chi tiet cu the"
assert khong_rag_dung == False, "khong_rag_dung phai la False -- KHONG RAG khong duoc chua chi tiet cu the (khong duoc bia)"
assert ket_luan == True, "ket_luan phai la True -- ca hai bang chung (co_rag_dung, not khong_rag_dung) deu phai dung"

# doi chieu truc tiep bang chi tiet cu the -- dung assert tren CHINH chuoi tra ve
assert CHI_TIET_CU_THE in tra_loi_co_rag, "'18 thang' PHAI co mat trong tra_loi_co_rag"
assert CHI_TIET_CU_THE not in tra_loi_khong_rag, "'18 thang' KHONG DUOC co mat trong tra_loi_khong_rag"

# kiem tra truc tiep llm_mo_phong tren vi du nho, tu tinh tay duoc
assert llm_mo_phong("cau hoi tran, khong co ngu canh gi ca") == CAU_TRA_LOI_KHONG_BIET, "khong co 'Ngu canh:' phai tra ve cau co dinh"
assert llm_mo_phong("Ngu canh:\nmot doan khong lien quan gi\n\nCau hoi: x?\nTra loi:") == CAU_TRA_LOI_KHONG_BIET, "co 'Ngu canh:' nhung khong trich duoc thoi han bao hanh van phai tra ve cau co dinh"
assert llm_mo_phong("Ngu canh:\nsan pham abc co bao hanh 24 thang\n\nCau hoi: bao hanh may thang?\nTra loi:") == "che do bao hanh cua san pham zt8821 la 24 thang", "trich dung thoi han '24 thang' tu ngu canh khac"

# bien: dam bao doan 11 (bao hanh) THAT SU co trong cac_doan_top -- day la
# ly do prompt chua duoc chi tiet
assert 11 in topk_hybrid, "doan 11 (bao hanh) phai co trong topk_hybrid de prompt chua duoc chi tiet can"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ đầu và chỗ hai gọi `llm_mo_phong` với hai đầu vào KHÁC nhau — `prompt` đầy đủ (có ngữ cảnh) cho chỗ đầu, `CAU_HOI` trần (không ngữ cảnh) cho chỗ hai. Chỗ ba tổng hợp HAI điều kiện (`co_rag_dung`, `not khong_rag_dung`) bằng toán tử kết hợp CẢ HAI phải đúng.
- kind: strategy
  body: 'Chỗ đầu: `llm_mo_phong(prompt)`. Chỗ hai: `llm_mo_phong(CAU_HOI)`. Chỗ ba: `co_rag_dung and not khong_rag_dung` — CÓ RAG phải đúng, VÀ KHÔNG RAG phải KHÔNG đúng (không được bịa).'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `llm_mo_phong(prompt)`, `llm_mo_phong(CAU_HOI)`, và `co_rag_dung and not khong_rag_dung`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: hai cho trong dau phai GOI THAT llm_mo_phong voi dung hai dau vao khac nhau (prompt VA CAU_HOI, khong duoc goi trung mot dau vao); VA cho trong ba phai la phep 'and' THAT giua co_rag_dung va 'not khong_rag_dung' (khong duoc chep san True, khong doi thanh 'or')
  requireAst:
  - kind: uses-call, target: llm_mo_phong, min: 2
  - kind: uses-name, target: co_rag_dung, min: 2
  - kind: uses-name, target: khong_rag_dung, min: 2
  - kind: uses-operator, target: and, min: 1
  - kind: uses-operator, target: "not", min: 1
  # Da thu that (dung dem_ast.mjs goi THANG kiemAst() that qua dist build +
  # pyodide that, chay tren CHINH van ban solution da trich tu file nay) --
  # ket qua [2, 2, 2, 1, 1] cho nam luat theo dung thu tu khai bao o tren.
  # llm_mo_phong=2: dung hai lan GOI, chinh la cho trong 1 va 2 -- dinh nghia
  # ham khong tinh la Call. Day la TONG THAT -- khong co boilerplate nao
  # khac goi lai ham nay.
  # co_rag_dung=2, khong_rag_dung=2 (TONG THAT, da xac nhan bang cong cu,
  # khong doan tay): moi bien duoc GAN (Store, khong dem) mot lan, roi DOC
  # (Load) hai lan -- mot lan CO SAN trong "print(co_rag_dung,
  # khong_rag_dung)" (luon chay, khong bi cho trong), mot lan CHINH la trong
  # cho trong ba. Neu chi dat min=1 (ngay tho), mot mutant chep san
  # "ket_luan = True" (bo qua hoan toan bieu thuc that o cho trong ba) van
  # qua duoc vi con lai 1 lan doc moi bien trong dong print -- GOTCHA
  # "boilerplate-threshold-masking"; da tu kiem chung bang cong cu: mutant
  # do cho dem co_rag_dung=1 va khong_rag_dung=1 (ca hai deu tut duoi nguong
  # 2), bi chan boi static; dong thoi van bi chan doc lap boi test rieng
  # "ket_luan == True" tren du lieu THAT (vi ca hai eu that su True/False nhu
  # mong doi nen "True" tinh co dung -- static la tang BAT BUOC o day, khong
  # the chi dua vao output).
  # and=1, not=1: moi toan tu CHI xuat hien dung mot lan trong toan bo
  # solution, chinh la cho trong ba ("co_rag_dung and not khong_rag_dung") --
  # khong co "and"/"not" nao khac trong boilerplate (ham llm_mo_phong dung
  # "not in", la toan tu KHAC, muc tieu rieng "not in" khong phai "not").
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" — DA THU HAI CAP (ca hai da tu dung
  # mutant va CHAY THAT qua python3 VA dem_ast.mjs de xac nhan, khong doan
  # tay):
  # (a) hoan doi cho trong 1 va cho trong 2 (dien "llm_mo_phong(CAU_HOI)" vao
  # cho trong 1, "llm_mo_phong(prompt)" vao cho trong 2): dem AST la
  # [2, 2, 2, 1, 1] -- Y HET ban dung (ca hai deu la lenh GOI llm_mo_phong,
  # chi doi doi so). Static KHONG bat duoc. Mutant nay KHONG nem loi nao (ca
  # 'prompt' lan 'CAU_HOI' deu la chuoi hop le de truyen vao llm_mo_phong) --
  # no chay xong binh thuong nhung cho ra ket qua NGUOC HOAN TOAN: da tu
  # chay that xac nhan tra_loi_co_rag tro thanh "toi khong co thong tin..."
  # (dang ra phai la cau tra loi dung) va ket_luan tro thanh False (dang ra
  # phai True) -- bi bat DOC LAP boi cac assert "tra_loi_co_rag == ..."/
  # "ket_luan == True" o tren, KHONG phai boi tier 'run'.
  # (b) hoan doi cho trong 1 va cho trong 3 (dien "co_rag_dung and not
  # khong_rag_dung" vao cho trong 1, "llm_mo_phong(prompt)" vao cho trong 3):
  # da tu chay that qua python3, xac nhan no nem NameError ("name
  # 'co_rag_dung' is not defined") NGAY LAP TUC o dong dau tien -- vi
  # 'co_rag_dung'/'khong_rag_dung' chua tung duoc gan o bat ky dau truoc do
  # trong toan bo chuong trinh (day la lan dau tien hai ten nay xuat hien).
  # Bi bat boi tier 'run', doc lap voi static.
  # Da tu ra soat GOTCHA #6: khong co bien/ham nao khac trong pham vi bai
  # nay cung ten/hinh dang co the tinh co thay the llm_mo_phong/co_rag_dung/
  # khong_rag_dung ma van qua duoc — ca ba deu la ten DUY NHAT trong toan bo
  # solution.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^che do bao hanh cua san pham zt8821 la 18 thang\\ntoi khong co thong tin de tra loi cau hoi nay\\nTrue False\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`True False` rồi `True` — CÓ RAG trả lời đúng chi tiết cụ thể, KHÔNG RAG từ
chối đúng cách thay vì bịa. Quest `boss-rag-tu-so-0` (q8.5f) đóng tại `5/5`.
Track `T8.5` "RAG từ số `0`" khép lại tại `34/34` bài.
::::

::::reflect{#nghi-lai}
Track `T8.5` mở ra với một câu hỏi từ `T8.4`: một LLM (mô phỏng, vì sandbox
này không có mạng) hữu ích tới đâu nếu nó không có cách nào tra cứu một tài
liệu KHÔNG nằm trong dữ liệu huấn luyện của nó? Sáu quest, `34` bài, đã trả
lời câu đó từng mảnh một:

> **`chunking-va-embedding-tu-che`** (q8.5a, `6` bài) — chia văn bản dài
> thành đoạn nhỏ theo câu (`chia_theo_cau`) và theo cửa sổ trượt có chồng
> lấn; dựng một "embedding" TỰ CHẾ (vector đếm từ trên từ vựng cố định,
> KHÔNG phải mô hình thật, vì Pyodide không có mạng); đo hai đoạn cùng chủ
> đề cho vector "gần nhau" hơn bằng số cụ thể.
>
> **`tim-kiem-vector-va-do-tuong-dong`** (q8.5b, `5` bài) — tích vô hướng
> đo mức "cùng hướng", cosine chuẩn hoá độ dài, vét cạn k-NN làm chuẩn đối
> chiếu (chậm nhưng LUÔN đúng) cho mọi quest sau.
>
> **`hnsw-tu-cai`** (q8.5c, `8` bài, lõi kỹ thuật khó nhất track) — đồ thị
> điều hướng nhỏ nhiều tầng, gán tầng ngẫu nhiên CÓ SEED (tất định), thuật
> toán chèn và tìm kiếm đầy đủ, đo recall (`≈0,79` trên ví dụ `20` điểm) và
> số phép so sánh (ít hơn hẳn vét cạn) — đánh đổi cốt lõi của tìm kiếm GẦN
> ĐÚNG.
>
> **`bm25-va-tim-kiem-tu-khoa`** (q8.5d, `6` bài) — vì sao cần khớp từ khoá
> CHÍNH XÁC bên cạnh vector (một mã hiếm/tên riêng vector không "học" được);
> TF, IDF, công thức BM25 đầy đủ với `k1`/`b`.
>
> **`hybrid-va-rerank`** (q8.5e, `4` bài) — Reciprocal Rank Fusion hợp nhất
> thứ hạng (không cộng điểm số khác thang đo trực tiếp); đo hybrid sửa được
> CẢ điểm mù của cosine (mã hiếm) LẪN điểm mù của BM25 (diễn đạt khác từ)
> trên cùng một kho.
>
> **`boss-rag-tu-so-0`** (q8.5f, `5` bài, quest này) — ráp TOÀN BỘ: chunking
> + vector trên tài liệu MỚI → chỉ mục HNSW → tìm kiếm hybrid (nơi ĐẦU TIÊN
> quy tắc "vắng mặt = đóng góp `0`" áp dụng thật, vì HNSW chỉ trả top-`k`) →
> prompt có cấu trúc → LLM mô phỏng, đo CÓ RAG (`tra_loi_co_rag` chứa đúng
> `"18 thang"`) so với KHÔNG RAG (`tra_loi_khong_rag` từ chối đúng cách,
> không bịa).

Sợi chỉ xuyên suốt, không đổi từ bài đầu tới bài cuối của cả track: MỌI
"vector"/"embedding" là tự chế (đếm từ trên từ vựng cố định, không mô hình
sản xuất thật), MỌI "LLM" là một hàm THUẦN tất định (tra bảng/regex, đúng
khuôn `T8.4`) — không phải vì sandbox không làm được gì tốt hơn, mà vì luận
điểm trung tâm của CẢ hai track là: những CƠ CHẾ quyết định một hệ thống RAG
có tìm đúng và trả lời đúng hay không — chunking, đo tương đồng, cấu trúc đồ
thị nhiều tầng, TF-IDF, hợp nhất thứ hạng, dựng prompt — đều là logic có thể
VIẾT RA, ĐO ĐƯỢC, và KIỂM CHỨNG bằng số, hoàn toàn độc lập với việc mô hình
ngôn ngữ hay mô hình embedding đứng sau nó "thông minh" tới đâu. `18 thang`
xuất hiện trong `tra_loi_co_rag` không phải vì LLM "giỏi" — mà vì NĂM LỚP KỸ
THUẬT phía trước nó (chunking đúng, chỉ mục đúng, hybrid cứu đúng đoạn, prompt
ghép đúng cấu trúc) đã đưa đúng thông tin tới đúng chỗ. Track `T8.5` "RAG từ
số `0"` khép lại tại `34/34` bài, bàn giao sang `T8.6` "Hạ tầng & vận hành
AI" — tái dùng nguyên vẹn hạ tầng đã xây, không còn thuật toán mới nào cần
viết.
::::

::::checkpoint{mastery=0.95}
::::
