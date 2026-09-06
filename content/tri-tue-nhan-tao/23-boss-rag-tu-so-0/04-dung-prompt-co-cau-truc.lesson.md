---
id: tri-tue-nhan-tao.boss-rag-tu-so-0.dung-prompt-co-cau-truc
title: "Dựng prompt có cấu trúc từ các đoạn tìm được"
summary: "dung_prompt(cac_doan, cau_hoi) ghep 4 doan top-hybrid ([1,0,6,11], bai truoc) thanh MOT chuoi 'Ngu canh:\\n<doan>...\\n\\nCau hoi: <cau hoi>\\nTra loi dua TREN ngu canh tren:' -- dai DUNG 332 ky tu, CHUA ca 'zt8821' LAN '18 thang' (chi tiet cu the can thiet, nam trong doan 11 vua duoc hybrid cuu o bai truoc). dung_prompt la MOT ham THUAN chi ghep chuoi -- khong tu kiem tra thieu chi tiet gi, khong biet gi ve noi dung ngu canh no nhan duoc."
locale: vi
track: tri-tue-nhan-tao
module: boss-rag-tu-so-0
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.dung-prompt-co-cau-truc]
requires: [ai.tim-kiem-hybrid-rag]
concepts: [ai.dung-prompt-co-cau-truc]
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
Top-`4` đoạn đã tìm được (`[1, 0, 6, 11]`, bài trước) — kể cả đoạn `11` chứa
chi tiết bảo hành mà chỉ hybrid mới cứu được. Bài này ghép chúng thành MỘT
prompt có cấu trúc rõ ràng, sẵn sàng đưa cho một "LLM" (mô phỏng, bài BOSS
sau).
::::

::::explain{#dung_prompt_co_cau_truc}
Tái dùng nguyên tắc đã học ở `T8.4` (`prompt-engineering-nen-tang`, q8.4a —
vai trò rõ ràng của từng phần trong một prompt; `dau-ra-co-cau-truc-va-cong-
cu`, q8.4c — tách bạch NGỮ CẢNH khỏi CÂU HỎI): một prompt RAG cần MỘT cấu
trúc cố định, dễ phân biệt — phần "ngữ cảnh" (những đoạn vừa truy xuất) tách
biệt rõ khỏi phần "câu hỏi" (điều người dùng thực sự hỏi), và một chỉ dẫn
cuối nói rõ "trả lời DỰA TRÊN ngữ cảnh trên" (không phải dựa vào kiến thức
nội tại nào khác).

`dung_prompt(cac_doan, cau_hoi)` là một hàm THUẦN — chỉ ghép chuỗi, không
làm gì khác:

```
Ngu canh:
<đoạn 1>
<đoạn 2>
...

Cau hoi: <câu hỏi>
Tra loi dua TREN ngu canh tren:
```

Điều QUAN TRỌNG cần thấy rõ: `dung_prompt` không hề "biết" các đoạn nó nhận
được có chứa câu trả lời đúng hay không — nó chỉ ghép NGUYÊN VĂN những gì
được truyền vào. Nếu bài trước KHÔNG cứu được đoạn `11` vào top-`k` (ví dụ
chỉ dùng vét cạn cosine), `dung_prompt` vẫn chạy bình thường, cho ra một
prompt có cấu trúc HOÀN TOÀN hợp lệ — chỉ là ngữ cảnh bên trong nó thiếu mất
chi tiết cần thiết. Việc "trả lời có đúng hay không" là việc của bước SAU
(bài BOSS), không phải của hàm ghép prompt này.
::::

::::example{#dung_prompt_tu_top_k_hybrid}
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

print(topk_hybrid)
print(len(prompt))
print("18 thang" in prompt)
print("zt8821" in prompt)
```

```text title=readonly
[1, 0, 6, 11]
332
True
True
```

Prompt dựng từ đúng `4` đoạn top-hybrid (`[1, 0, 6, 11]`) dài `332` ký tự,
CHỨA cả `"zt8821"` (mã sản phẩm) LẪN `"18 thang"` (chi tiết bảo hành, chỉ có
mặt vì hybrid RRF đã cứu đoạn `11` ở bài trước). Nếu top-`k` chỉ là vét cạn
cosine (`[1, 0, 6, 2]`, KHÔNG có đoạn `11`), `dung_prompt` vẫn tạo ra một
prompt hợp lệ — chỉ là `"18 thang"` sẽ KHÔNG có mặt trong đó.
::::

::::predict{#doan_prompt_khong_tu_phat_hien_thieu commitOnce}
Giả sử (một tình huống KHÁC bài này) bước truy xuất trước đó CHỈ dùng vét
cạn cosine, và `topk_hybrid` trở thành `[1, 0, 6, 2]` — KHÔNG chứa đoạn
`11` (thời hạn bảo hành).

**Trước khi chạy thử**, bạn đoán: `dung_prompt` có tự PHÁT HIỆN ra rằng chi
tiết "thời hạn bảo hành" bị thiếu trong ngữ cảnh, và tự thêm một cảnh báo
vào prompt hay không?

:::opt{correct}
Không — `dung_prompt` là một hàm THUẦN chỉ ghép chuỗi (`"\n".join(...)` rồi
lồng vào một f-string cố định); nó không đọc hiểu NỘI DUNG các đoạn được
truyền vào, không biết câu hỏi cần chi tiết gì, nên không có cách nào phát
hiện "thiếu" — nó vẫn tạo ra một prompt có cấu trúc HOÀN TOÀN hợp lệ, chỉ là
ngữ cảnh bên trong thiếu chi tiết cần
:::

:::opt
Có — hàm được thiết kế để so khớp `cau_hoi` với từng đoạn trong `cac_doan`
trước khi ghép, và sẽ thêm dòng cảnh báo nếu không đoạn nào khớp đủ
::why
Gần đúng ở việc một hệ thống RAG THẬT SỰ có thể cần một bước kiểm tra như
vậy — đó là một ý tưởng hợp lý cho một hệ thống production.

Chỗ lệch: `dung_prompt`, đúng như đã viết ở bài này, không có bước so khớp
nào — thân hàm chỉ có đúng hai dòng: nối `cac_doan` bằng `"\n".join(...)`,
rồi lồng kết quả đó cùng `cau_hoi` vào một f-string cố định. Không có lệnh
`if` nào kiểm tra nội dung, nên không thể "phát hiện thiếu" được.
::
:::

:::opt
Có, nhưng chỉ khi `cac_doan` là danh sách RỖNG — khi đó hàm sẽ tự thay bằng
dòng "khong tim thay thong tin lien quan"
::why
Gần đúng ở việc danh sách RỖNG là một trường hợp biên đáng cân nhắc trong
thiết kế thực tế — quan sát về TRƯỜNG HỢP BIÊN đó hợp lý.

Chỗ lệch: `"\n".join([])` trong Python trả về chuỗi RỖNG (`""`), không ném
lỗi, và `dung_prompt` không có nhánh xử lý đặc biệt nào cho trường hợp đó —
nó vẫn ghép bình thường, cho ra một prompt với phần "Ngu canh:" trống, không
tự thay bằng bất kỳ câu cảnh báo nào.
::
:::
::::

::::code{#viet_dung_prompt}
Hoàn thiện `dung_prompt`: ghép các đoạn thành một khối ngữ cảnh, rồi lồng
vào một prompt có cấu trúc cố định.

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
    ngu_canh = ___                                          # "\n".join(cac_doan)
    return ___                                               # f"Ngu canh:\n{ngu_canh}\n\nCau hoi: {cau_hoi}\nTra loi dua TREN ngu canh tren:"


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

print(topk_hybrid)
print(len(prompt))
print("18 thang" in prompt)
print("zt8821" in prompt)
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

print(topk_hybrid)
print(len(prompt))
print("18 thang" in prompt)
print("zt8821" in prompt)
```

```python title=test
assert topk_hybrid == [1, 0, 6, 11], f"topk_hybrid sai -- dang ra {topk_hybrid}"
assert len(prompt) == 332, f"do dai prompt phai la 332 ky tu -- dang ra {len(prompt)}"
assert "18 thang" in prompt, "prompt phai chua chi tiet cu the '18 thang' (tu doan 11, duoc hybrid cuu vao top-k)"
assert "zt8821" in prompt, "prompt phai chua ma san pham 'zt8821'"
assert prompt.startswith("Ngu canh:\n"), "prompt phai bat dau bang 'Ngu canh:'"
assert f"Cau hoi: {CAU_HOI}" in prompt, "prompt phai chua dung dong 'Cau hoi: <cau hoi goc>'"
assert prompt.endswith("Tra loi dua TREN ngu canh tren:"), "prompt phai ket thuc bang dung chi dan 'Tra loi dua TREN ngu canh tren:'"

# kiem tra truc tiep dung_prompt tren vi du nho, tu tinh tay duoc
assert dung_prompt(["a", "b"], "c?") == "Ngu canh:\na\nb\n\nCau hoi: c?\nTra loi dua TREN ngu canh tren:", "dung_prompt tren vi du nho sai"

# bien: danh sach doan RONG -- khong loi, ngu canh la chuoi rong
assert dung_prompt([], "c?") == "Ngu canh:\n\n\nCau hoi: c?\nTra loi dua TREN ngu canh tren:", "danh sach doan rong phai cho ngu canh rong, khong loi"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cả hai trong thân `dung_prompt`. Chỗ đầu ghép danh sách `cac_doan` thành MỘT khối văn bản, mỗi đoạn một dòng. Chỗ hai lồng khối văn bản vừa ghép (biến `ngu_canh`) cùng `cau_hoi` vào một chuỗi có cấu trúc cố định — dùng f-string.
- kind: strategy
  body: 'Chỗ đầu: `"\n".join(cac_doan)` — nối các đoạn bằng dấu xuống dòng. Chỗ hai: `f"Ngu canh:\n{ngu_canh}\n\nCau hoi: {cau_hoi}\nTra loi dua TREN ngu canh tren:"` — đúng cấu trúc bốn phần đã mô tả ở `explain`.'
- kind: one-line
  body: 'Chỗ đầu là `"\n".join(cac_doan)`, chỗ hai là `f"Ngu canh:\n{ngu_canh}\n\nCau hoi: {cau_hoi}\nTra loi dua TREN ngu canh tren:"`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT "\n".join(cac_doan) de ghep cac doan (khong duoc chep san mot chuoi co dinh); VA cho trong hai phai TRA VE mot f-string THAT long ca ngu_canh LAN cau_hoi vao dung cau truc bon phan (khong duoc chep san mot chuoi tinh, khong duoc bo qua bien nao)
  requireAst:
  - kind: uses-call, target: join, min: 1
  - kind: uses-fstring, min: 1
  - kind: uses-name, target: ngu_canh, min: 1
  - kind: uses-name, target: cau_hoi, min: 2
  - kind: uses-name, target: cac_doan, min: 1
  # Da thu that (dung dem_ast.mjs goi THANG kiemAst() that qua dist build +
  # pyodide that, chay tren CHINH van ban solution da trich tu file nay) --
  # ket qua [1, 1, 1, 2, 1] cho nam luat theo dung thu tu khai bao o tren.
  # join=1, cac_doan=1: CHI mot lan GOI/DOC duy nhat trong toan bo solution,
  # dung o cho trong dau ("\"\\n\".join(cac_doan)") -- khong co boilerplate
  # nao khac dung .join(...) hay doc bien 'cac_doan'.
  # uses-fstring=1: CHI mot f-string duy nhat trong toan bo solution, chinh
  # la cho trong hai -- khong co f-string nao khac (moi ham khac deu dung
  # chuoi thuong hoac .format khong co, hoac khong dinh dang chuoi nao ca).
  # ngu_canh=1: bien nay CHI duoc dinh nghia (Store, khong dem) o cho trong
  # dau, va duoc DOC (Load) dung mot lan trong f-string o cho trong hai.
  # cau_hoi=2 (TONG THAT, da xac nhan bang cong cu, khong doan tay): MOT lan
  # CO SAN trong "xep_hang_bm25" ("tu_truy_van = cau_hoi.lower().split()" --
  # tham so ham nay CUNG ten 'cau_hoi', doc no o than ham), MOT lan CHINH la
  # trong f-string o cho trong hai. Neu chi dat min=1 (ngay tho), mot mutant
  # BO {cau_hoi} khoi f-string (chi con "Ngu canh:...Tra loi dua TREN ngu
  # canh tren:" thieu hoan toan cau hoi) van co the qua NEU con lai 1 lan doc
  # 'cau_hoi' o noi khac -- GOTCHA "boilerplate-threshold-masking"; da tu
  # kiem chung bang cong cu: mutant bo {cau_hoi} khoi f-string cho dem
  # cau_hoi=1 (tut duoi nguong 2), bi chan; dong thoi bi chan doc lap boi
  # test "f'Cau hoi: {CAU_HOI}' in prompt" (thieu hoan toan cau hoi trong
  # prompt).
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua
  # python3 VA qua dem_ast.mjs de xac nhan, khong doan tay): dien
  # 'f"Ngu canh:\n{ngu_canh}\n\nCau hoi: {cau_hoi}\nTra loi dua TREN ngu canh
  # tren:"' vao cho trong dau ("ngu_canh = f\"...{ngu_canh}...\"") VA dien
  # '"\n".join(cac_doan)' vao cho trong hai ("return \"\n\".join(cac_doan)")
  # -- da chay THAT qua dem_ast.mjs: ket qua dem la [1, 1, 1, 2, 1], Y HET
  # ban dung (moi ham/bien van duoc goi/doc dung so lan, chi DOI VI TRI giua
  # hai dong). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT DOC LAP boi tier 'run': dong dau tien "ngu_canh =
  # f\"...{ngu_canh}...\"" DOC bien 'ngu_canh' o VE PHAI (ben trong f-string)
  # TRUOC KHI no duoc GAN — vi day la mot ham (dung_prompt), Python coi
  # 'ngu_canh' la bien CUC BO xuyen suot toan than ham (do co dong gan o
  # duoi), nen doc no truoc khi gan la LOI CUC BO, khong phai NameError toan
  # cuc. Da tu chay THAT mutant nay qua python3, xac nhan no nem
  # UnboundLocalError ("cannot access local variable 'ngu_canh' where it is
  # not associated with a value") ngay khi dung_prompt(...) duoc goi, truoc
  # ca khi cham toi dong return.
  # Da tu ra soat GOTCHA #6: khong co bien nao khac trong pham vi ham
  # dung_prompt cung ten/hinh dang co the tinh co thay the 'ngu_canh' hay
  # 'cac_doan' ma van qua duoc — hai ten nay chi xuat hien trong dung ham
  # nay, khong trung voi bat ky bien nao o noi khac trong bai.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[1, 0, 6, 11\\]\\n332\\nTrue\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`332` ký tự, chứa cả `"zt8821"` lẫn `"18 thang"` — prompt có cấu trúc, sẵn
sàng. Bài BOSS cuối cùng đưa nó cho một "LLM" mô phỏng, và đo xem có-RAG so
với không-RAG khác nhau ra sao — đóng cả track `T8.5` tại `34/34`.
::::

::::reflect{#nghi-lai}
`dung_prompt` là hàm ĐƠN GIẢN NHẤT trong cả năm bài của quest này — không
thuật toán, không vòng lặp phức tạp, chỉ hai dòng ghép chuỗi. Nhưng vai trò
của nó không hề nhỏ: nó là ranh giới rõ ràng giữa "những gì hệ thống truy
xuất được" (bốn bài trước — chunking, HNSW, hybrid) và "những gì một LLM sẽ
đọc" (bài BOSS sau). Điểm quan trọng nhất bài này làm rõ: hàm ghép prompt
không hề "thông minh" — nó không kiểm tra, không phát hiện thiếu sót, không
đọc hiểu. Chất lượng của ngữ cảnh nó ghép ra hoàn toàn phụ thuộc vào chất
lượng của bước truy xuất TRƯỚC đó — nếu bài `3` dùng vét cạn cosine đơn
thuần thay vì hybrid, prompt này vẫn "hợp lệ" về cấu trúc, nhưng THIẾU chi
tiết cần thiết, và không có gì trong `dung_prompt` cảnh báo về điều đó. Bài
BOSS cuối cùng đo chính xác hậu quả của sự khác biệt này bằng số.
::::

::::checkpoint{mastery=0.85}
::::
