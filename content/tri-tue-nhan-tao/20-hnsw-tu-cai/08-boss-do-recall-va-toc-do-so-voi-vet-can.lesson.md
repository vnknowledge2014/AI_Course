---
id: tri-tue-nhan-tao.hnsw-tu-cai.boss-do-recall-va-toc-do-so-voi-vet-can
title: "BOSS — Đo recall và tốc độ so với vét cạn: ráp toàn bộ HNSW, đóng q8.5c"
summary: "Mot tai lieu 24 cau tu nghi (12 tech + 12 am thuc) chia bang chia_theo_cau (q8.5a) thanh 24 diem. Chen HNSW day du (seed=104, M=3), tim kiem tren 8 cau hoi (k=3): recall trung binh = 0,7917 (19/24), tong so phep so sanh HNSW = 102. Vet can (q8.5b) tren CUNG 8 cau hoi: recall LUON 100% nhung tong so phep so sanh = 192 (= 24*8). HNSW GIAM 47% so phep so sanh (102 so voi 192), doi lai recall duoi 100% -- danh doi do bang so that, dong quest hnsw-tu-cai (q8.5c) tai 8/8 bai."
locale: vi
track: tri-tue-nhan-tao
module: hnsw-tu-cai
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-do-recall-va-toc-do-so-voi-vet-can]
requires: [ai.thuat-toan-tim-kiem-hnsw-day-du]
concepts: [ai.boss-do-recall-va-toc-do-so-voi-vet-can]
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
Bảy bài: ý tưởng đồ thị nhỏ, xây đồ thị một tầng, tìm tham lam, giới hạn
một tầng, gán tầng ngẫu nhiên, chèn nhiều tầng, tìm kiếm nhiều tầng. Bài
này ráp CẢ BẢY vào một quy trình duy nhất, trên một tài liệu HOÀN TOÀN MỚI
— và đóng quest `hnsw-tu-cai` (q8.5c) tại `8/8`.
::::

::::explain{#rap_toan_bo_hnsw}
Quy trình HNSW đầy đủ, ĐÚNG bảy bước đã xây, chạy theo thứ tự:

> **(a) Chia tài liệu thành đoạn** (`chia_theo_cau`, q8.5a) — tách tài liệu
> dài thành nhiều điểm dữ liệu nhỏ.
>
> **(b) Tính vector mỗi đoạn** (`tinh_vector_dem_tu`, q8.5a/b).
>
> **(c) Gán tầng ngẫu nhiên có seed** (bài `5`) — mỗi điểm được gán tầng
> cao nhất nó xuất hiện, tất định qua `random.Random(seed)`.
>
> **(d) Chèn HNSW nhiều tầng** (bài `6`) — dùng tìm tham lam (bài `3`) và
> nối cạnh có cắt bớt (bài `2`) tại mỗi tầng.
>
> **(e) Tìm kiếm HNSW nhiều tầng** (bài `7`) — từ điểm vào tầng cao nhất,
> xuống dần tới tầng `0`, trả về `k` ứng viên gần nhất.
>
> **(f) Đối chiếu với vét cạn** (q8.5b) — vét cạn LUÔN đúng (`100%` recall)
> nhưng tốn `N` phép so sánh MỖI câu hỏi; HNSW tốn ÍT HƠN nhưng recall có
> thể dưới `100%`.

Bài này dùng một tài liệu `24` câu, TỰ NGHĨ, chưa từng xuất hiện ở bài nào
trước — `12` câu chủ đề công nghệ, `12` câu chủ đề ẩm thực — và `8` câu hỏi
khác nhau để đo TRUNG BÌNH cả recall lẫn số phép so sánh, không chỉ MỘT câu
hỏi đơn lẻ.
::::

::::example{#boss_do_danh_doi}
Tài liệu `24` câu, chia bằng `chia_theo_cau` (q8.5a), chèn HNSW (`seed=104`,
`M=3`), tìm kiếm `k=3` trên `8` câu hỏi, đối chiếu TRỰC TIẾP với vét cạn:

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


def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


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


def danh_gia_hnsw(do_thi, vectors, cac_cau_hoi_vector, diem_vao, tang_cao_nhat, k):
    tong_recall = 0.0
    tong_so_sanh_hnsw = 0
    for qv in cac_cau_hoi_vector:
        dap_an_dung = tim_k_lan_can_vet_can(qv, vectors, k)
        ket_qua, so_sanh = tim_kiem_hnsw(do_thi, vectors, qv, diem_vao, tang_cao_nhat, k)
        tong_recall += len(set(dap_an_dung) & set(ket_qua)) / k
        tong_so_sanh_hnsw += so_sanh
    so_cau_hoi = len(cac_cau_hoi_vector)
    recall_trung_binh = tong_recall / so_cau_hoi
    so_sanh_vet_can = len(vectors) * so_cau_hoi
    return recall_trung_binh, tong_so_sanh_hnsw, so_sanh_vet_can


TAI_LIEU_BOSS = (
    "may tinh hien dai chay phan mem manh xu ly du lieu nhanh. "
    "lap trinh vien gioi viet thuat toan toi uu cho phan mem. "
    "vi xu ly toc do cao giup thiet bi hoat dong nhanh hon. "
    "ket noi mang on dinh giup ung dung chay muot ma hon. "
    "thuat toan sap xep du lieu can hieu qua va chinh xac. "
    "ung dung tren may tinh xu ly duoc luong du lieu rat lon. "
    "vi xu ly manh giup ung dung phan tich chay nhanh hon nhieu. "
    "phan mem lap trinh dung thuat toan toi uu chay on dinh. "
    "du lieu duoc may tinh xu ly tu dong khong can can thiep. "
    "ung dung ket noi mang de truyen du lieu giua cac thiet bi. "
    "mot he thong may tinh manh co the chay nhieu phan mem cung luc. "
    "lap trinh vien can hieu ro thuat toan truoc khi viet ma. "
    "mon an ngon thuong can gia vi va cong thuc chuan xac. "
    "dau bep gioi nau an rat can than trong nha bep sach se. "
    "thuc pham tuoi giup mon an tro nen ngon va hap dan hon. "
    "cong thuc nau an don gian giup nguoi moi de lam theo. "
    "mon trang mieng ngot ngao thuong duoc dung sau bua an chinh. "
    "nha bep sach se giup dau bep lam viec thoai mai hon. "
    "gia vi dam da giup mon an tro nen hap dan hon nhieu. "
    "dau bep gioi biet cach che bien thuc pham tuoi ngon. "
    "mot mon an ngon can dung cong thuc va gia vi vua du. "
    "thuc pham tuoi ngon co the nau thanh mon trang mieng hay. "
    "moi bua an ngon deu can mon an chinh va mon trang mieng. "
    "cong thuc nau an hay thuong duoc dau bep gioi chia se rong rai."
)

doan = chia_theo_cau(TAI_LIEU_BOSS)
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in doan]
N = len(VEC)

CAU_HOI_BOSS = [
    "may tinh chay ung dung xu ly du lieu bang thuat toan nhanh",
    "mon an ngon voi gia vi va cong thuc nau chuan",
    "phan mem lap trinh toi uu thuat toan hieu qua",
    "dau bep nau mon trang mieng ngot ngao ngon",
    "ket noi mang on dinh cho ung dung chay muot",
    "thuc pham tuoi va nha bep sach se gon gang",
    "vi xu ly du lieu nhanh cho may tinh hien dai",
    "gia vi vua du cho mon an ngon hap dan",
]
CAC_VECTOR_CAU_HOI = [tinh_vector_dem_tu(q, TU_VUNG, CUM_TU_GOC) for q in CAU_HOI_BOSS]

tang_diem = gan_tang_ngau_nhien(104, N, 0.5)
do_thi, diem_vao, tang_cao_nhat = chen_hnsw(VEC, tang_diem, 3)

recall_trung_binh, tong_so_sanh_hnsw, so_sanh_vet_can = danh_gia_hnsw(
    do_thi, VEC, CAC_VECTOR_CAU_HOI, diem_vao, tang_cao_nhat, 3
)

print(N)
print(round(recall_trung_binh, 4))
print(tong_so_sanh_hnsw)
print(so_sanh_vet_can)
print(tong_so_sanh_hnsw < so_sanh_vet_can)
```

```text title=readonly
24
0.7917
102
192
True
```

Tài liệu `24` câu chia đúng `24` điểm. Trên `8` câu hỏi, `k=3`: HNSW đạt
recall trung bình `≈0,7917` (`19` trong tổng `24` kết quả đúng), tốn TỔNG
CỘNG `102` phép so sánh cho CẢ `8` câu hỏi. Vét cạn (đối chiếu bằng
`len(vectors) × so_cau_hoi = 24 × 8 = 192`) LUÔN đạt recall `100%` nhưng tốn
gần GẤP ĐÔI số phép so sánh (`192` so với `102` — HNSW giảm `≈47%`). Đây
chính là sự ĐÁNH ĐỔI cốt lõi của HNSW, đo bằng số thật, không suy đoán:
CHẤP NHẬN recall dưới `100%` để đổi lấy tốc độ.
::::

::::predict{#doan_hnsw_it_hon_han commitOnce}
Xét đúng ví dụ trên: vét cạn tốn `192` phép so sánh (`= 24 × 8`, LUÔN đạt
recall `100%`). HNSW tốn `102` phép so sánh, recall `≈0,7917`.

**Trước khi chạy thử**, bạn đoán: tỉ lệ `tong_so_sanh_hnsw / so_sanh_vet_can`
có nhỏ hơn `0,6` (nghĩa là HNSW tốn ÍT HƠN `60%` so với vét cạn) không?

:::opt{correct}
Có — `102 / 192 ≈ 0,53`, nhỏ hơn `0,6` — HNSW chỉ tốn hơn một nửa số phép so
sánh mà vét cạn cần, đổi lại recall giảm từ `100%` xuống `≈79,17%`
:::

:::opt
Không — vì HNSW vẫn phải đi qua NHIỀU tầng (mỗi tầng một lượt tìm tham
lam), nên tổng số phép so sánh cộng dồn qua các tầng phải xấp xỉ bằng, thậm
chí NHIỀU HƠN vét cạn
::why
Gần đúng ở việc HNSW THẬT SỰ đi qua NHIỀU tầng, mỗi tầng một lượt tìm tham
lam — quan sát về CƠ CHẾ nhiều bước đó đúng.

Chỗ lệch: "nhiều bước" không đồng nghĩa với "nhiều phép so sánh hơn vét
cạn" — mỗi bước ở HNSW chỉ xét MỘT SỐ NHỎ láng giềng (tối đa `M=3`), trong
khi vét cạn phải so sánh với CẢ `N=24` điểm ở MỖI câu hỏi. Cộng dồn qua các
tầng, tổng số phép so sánh của HNSW (`102`) VẪN thấp hơn hẳn vét cạn
(`192`) — đo được bằng số thật, không phải suy đoán.
::
:::

:::opt
Không — tỉ lệ phải xấp xỉ `1,0` (bằng nhau), vì HNSW và vét cạn cùng dùng
MỘT hàm `tuong_dong_cosine` để đo "gần nhau", nên chi phí tính toán phải
tương đương
::why
Gần đúng ở việc CẢ HAI cách đều dùng CHUNG một hàm `tuong_dong_cosine` để
đo similarity — quan sát về phép đo dùng chung đó đúng.

Chỗ lệch: dùng chung MỘT hàm đo similarity không có nghĩa là GỌI hàm đó
CÙNG SỐ LẦN. Vét cạn gọi nó cho MỌI cặp (câu hỏi, điểm trong kho) — không
có cách nào tránh. HNSW chỉ gọi nó cho những điểm NẰM TRÊN đường đi tham lam
qua các tầng — một tập con nhỏ hơn nhiều. Số lần gọi thực tế khác nhau RẤT
nhiều (`102` so với `192`), không xấp xỉ bằng nhau.
::
:::
::::

::::code{#viet_danh_gia_hnsw}
Hoàn thiện `danh_gia_hnsw`: với MỖI câu hỏi, đối chiếu `top-k` HNSW với
`top-k` vét cạn để tính recall; tính tổng số phép so sánh mà vét cạn CẦN
(luôn bằng `N` mỗi câu hỏi) để đối chiếu.

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


def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


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


def danh_gia_hnsw(do_thi, vectors, cac_cau_hoi_vector, diem_vao, tang_cao_nhat, k):
    tong_recall = 0.0
    tong_so_sanh_hnsw = 0
    for qv in cac_cau_hoi_vector:
        dap_an_dung = tim_k_lan_can_vet_can(qv, vectors, k)
        ket_qua, so_sanh = tim_kiem_hnsw(do_thi, vectors, qv, diem_vao, tang_cao_nhat, k)
        tong_recall += len(set(___) & set(ket_qua)) / k    # dap_an_dung
        tong_so_sanh_hnsw += so_sanh
    so_cau_hoi = len(cac_cau_hoi_vector)
    recall_trung_binh = tong_recall / so_cau_hoi
    so_sanh_vet_can = ___(vectors) * so_cau_hoi              # len
    return recall_trung_binh, tong_so_sanh_hnsw, so_sanh_vet_can


TAI_LIEU_BOSS = (
    "may tinh hien dai chay phan mem manh xu ly du lieu nhanh. "
    "lap trinh vien gioi viet thuat toan toi uu cho phan mem. "
    "vi xu ly toc do cao giup thiet bi hoat dong nhanh hon. "
    "ket noi mang on dinh giup ung dung chay muot ma hon. "
    "thuat toan sap xep du lieu can hieu qua va chinh xac. "
    "ung dung tren may tinh xu ly duoc luong du lieu rat lon. "
    "vi xu ly manh giup ung dung phan tich chay nhanh hon nhieu. "
    "phan mem lap trinh dung thuat toan toi uu chay on dinh. "
    "du lieu duoc may tinh xu ly tu dong khong can can thiep. "
    "ung dung ket noi mang de truyen du lieu giua cac thiet bi. "
    "mot he thong may tinh manh co the chay nhieu phan mem cung luc. "
    "lap trinh vien can hieu ro thuat toan truoc khi viet ma. "
    "mon an ngon thuong can gia vi va cong thuc chuan xac. "
    "dau bep gioi nau an rat can than trong nha bep sach se. "
    "thuc pham tuoi giup mon an tro nen ngon va hap dan hon. "
    "cong thuc nau an don gian giup nguoi moi de lam theo. "
    "mon trang mieng ngot ngao thuong duoc dung sau bua an chinh. "
    "nha bep sach se giup dau bep lam viec thoai mai hon. "
    "gia vi dam da giup mon an tro nen hap dan hon nhieu. "
    "dau bep gioi biet cach che bien thuc pham tuoi ngon. "
    "mot mon an ngon can dung cong thuc va gia vi vua du. "
    "thuc pham tuoi ngon co the nau thanh mon trang mieng hay. "
    "moi bua an ngon deu can mon an chinh va mon trang mieng. "
    "cong thuc nau an hay thuong duoc dau bep gioi chia se rong rai."
)

doan = chia_theo_cau(TAI_LIEU_BOSS)
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in doan]
N = len(VEC)

CAU_HOI_BOSS = [
    "may tinh chay ung dung xu ly du lieu bang thuat toan nhanh",
    "mon an ngon voi gia vi va cong thuc nau chuan",
    "phan mem lap trinh toi uu thuat toan hieu qua",
    "dau bep nau mon trang mieng ngot ngao ngon",
    "ket noi mang on dinh cho ung dung chay muot",
    "thuc pham tuoi va nha bep sach se gon gang",
    "vi xu ly du lieu nhanh cho may tinh hien dai",
    "gia vi vua du cho mon an ngon hap dan",
]
CAC_VECTOR_CAU_HOI = [tinh_vector_dem_tu(q, TU_VUNG, CUM_TU_GOC) for q in CAU_HOI_BOSS]

tang_diem = gan_tang_ngau_nhien(104, N, 0.5)
do_thi, diem_vao, tang_cao_nhat = chen_hnsw(VEC, tang_diem, 3)

recall_trung_binh, tong_so_sanh_hnsw, so_sanh_vet_can = danh_gia_hnsw(
    do_thi, VEC, CAC_VECTOR_CAU_HOI, diem_vao, tang_cao_nhat, 3
)

print(N)
print(round(recall_trung_binh, 4))
print(tong_so_sanh_hnsw)
print(so_sanh_vet_can)
print(tong_so_sanh_hnsw < so_sanh_vet_can)
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


def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


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


def danh_gia_hnsw(do_thi, vectors, cac_cau_hoi_vector, diem_vao, tang_cao_nhat, k):
    tong_recall = 0.0
    tong_so_sanh_hnsw = 0
    for qv in cac_cau_hoi_vector:
        dap_an_dung = tim_k_lan_can_vet_can(qv, vectors, k)
        ket_qua, so_sanh = tim_kiem_hnsw(do_thi, vectors, qv, diem_vao, tang_cao_nhat, k)
        tong_recall += len(set(dap_an_dung) & set(ket_qua)) / k
        tong_so_sanh_hnsw += so_sanh
    so_cau_hoi = len(cac_cau_hoi_vector)
    recall_trung_binh = tong_recall / so_cau_hoi
    so_sanh_vet_can = len(vectors) * so_cau_hoi
    return recall_trung_binh, tong_so_sanh_hnsw, so_sanh_vet_can


TAI_LIEU_BOSS = (
    "may tinh hien dai chay phan mem manh xu ly du lieu nhanh. "
    "lap trinh vien gioi viet thuat toan toi uu cho phan mem. "
    "vi xu ly toc do cao giup thiet bi hoat dong nhanh hon. "
    "ket noi mang on dinh giup ung dung chay muot ma hon. "
    "thuat toan sap xep du lieu can hieu qua va chinh xac. "
    "ung dung tren may tinh xu ly duoc luong du lieu rat lon. "
    "vi xu ly manh giup ung dung phan tich chay nhanh hon nhieu. "
    "phan mem lap trinh dung thuat toan toi uu chay on dinh. "
    "du lieu duoc may tinh xu ly tu dong khong can can thiep. "
    "ung dung ket noi mang de truyen du lieu giua cac thiet bi. "
    "mot he thong may tinh manh co the chay nhieu phan mem cung luc. "
    "lap trinh vien can hieu ro thuat toan truoc khi viet ma. "
    "mon an ngon thuong can gia vi va cong thuc chuan xac. "
    "dau bep gioi nau an rat can than trong nha bep sach se. "
    "thuc pham tuoi giup mon an tro nen ngon va hap dan hon. "
    "cong thuc nau an don gian giup nguoi moi de lam theo. "
    "mon trang mieng ngot ngao thuong duoc dung sau bua an chinh. "
    "nha bep sach se giup dau bep lam viec thoai mai hon. "
    "gia vi dam da giup mon an tro nen hap dan hon nhieu. "
    "dau bep gioi biet cach che bien thuc pham tuoi ngon. "
    "mot mon an ngon can dung cong thuc va gia vi vua du. "
    "thuc pham tuoi ngon co the nau thanh mon trang mieng hay. "
    "moi bua an ngon deu can mon an chinh va mon trang mieng. "
    "cong thuc nau an hay thuong duoc dau bep gioi chia se rong rai."
)

doan = chia_theo_cau(TAI_LIEU_BOSS)
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in doan]
N = len(VEC)

CAU_HOI_BOSS = [
    "may tinh chay ung dung xu ly du lieu bang thuat toan nhanh",
    "mon an ngon voi gia vi va cong thuc nau chuan",
    "phan mem lap trinh toi uu thuat toan hieu qua",
    "dau bep nau mon trang mieng ngot ngao ngon",
    "ket noi mang on dinh cho ung dung chay muot",
    "thuc pham tuoi va nha bep sach se gon gang",
    "vi xu ly du lieu nhanh cho may tinh hien dai",
    "gia vi vua du cho mon an ngon hap dan",
]
CAC_VECTOR_CAU_HOI = [tinh_vector_dem_tu(q, TU_VUNG, CUM_TU_GOC) for q in CAU_HOI_BOSS]

tang_diem = gan_tang_ngau_nhien(104, N, 0.5)
do_thi, diem_vao, tang_cao_nhat = chen_hnsw(VEC, tang_diem, 3)

recall_trung_binh, tong_so_sanh_hnsw, so_sanh_vet_can = danh_gia_hnsw(
    do_thi, VEC, CAC_VECTOR_CAU_HOI, diem_vao, tang_cao_nhat, 3
)

print(N)
print(round(recall_trung_binh, 4))
print(tong_so_sanh_hnsw)
print(so_sanh_vet_can)
print(tong_so_sanh_hnsw < so_sanh_vet_can)
```

```python title=test
assert N == 24, f"tai lieu BOSS phai chia thanh 24 diem -- dang ra {N}"
assert round(recall_trung_binh, 4) == 0.7917, f"recall trung binh phai xap xi 0.7917 -- dang ra {recall_trung_binh}"
assert tong_so_sanh_hnsw == 102, f"tong so phep so sanh HNSW phai la 102 -- dang ra {tong_so_sanh_hnsw}"
assert so_sanh_vet_can == 192, f"tong so phep so sanh vet can phai la 192 (=24*8) -- dang ra {so_sanh_vet_can}"
assert tong_so_sanh_hnsw < so_sanh_vet_can, "HNSW phai ton IT HON vet can"
assert tong_so_sanh_hnsw / so_sanh_vet_can < 0.6, "HNSW phai ton duoi 60% so phep so sanh cua vet can"
assert recall_trung_binh < 1.0, "recall HNSW phai DUOI 100% -- day chinh la danh doi so voi vet can (vet can luon dat 100%)"

# kiem tra truc tiep tren mot vi du nho, tu tinh tay duoc (bai truoc da dung)
vec_nho = [[10, 0], [9, 1], [0, 10], [1, 9]]
tang_nho = [1, 0, 1, 0]
do_thi_nho, diem_vao_nho, tang_cao_nhat_nho = chen_hnsw(vec_nho, tang_nho, 2)
ket_qua_nho = danh_gia_hnsw(do_thi_nho, vec_nho, [[10, 0]], diem_vao_nho, tang_cao_nhat_nho, 2)
assert ket_qua_nho == (1.0, 4, 4), f"tren do thi nho voi 1 cau hoi, ket qua phai la (1.0, 4, 4) -- dang ra {ket_qua_nho}"

# bien: neu HNSW tim dung HET (recall=100%), so_sanh_hnsw van phai duoc dem dung, khong duoc gia mao
assert danh_gia_hnsw(do_thi_nho, vec_nho, [[10, 0]], diem_vao_nho, tang_cao_nhat_nho, 2)[1] > 0, "so phep so sanh HNSW phai LON HON 0, khong duoc bo qua viec dem"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai dòng khác nhau trong `danh_gia_hnsw`. Chỗ đầu nằm trong `len(set(...) & set(ket_qua))` — tập ĐÁP ÁN ĐÚNG (biến vừa tính ở dòng phía trên) để giao với tập kết quả HNSW. Chỗ hai là hàm ĐẾM số phần tử trong `vectors` (kho dữ liệu) — dùng để tính số phép so sánh mà vét cạn cần cho MỖI câu hỏi.
- kind: strategy
  body: 'Chỗ đầu: `dap_an_dung` — biến vừa gán bằng `tim_k_lan_can_vet_can(...)` ở dòng trên. Chỗ hai: `len` — `len(vectors)`, số điểm trong kho.'
- kind: one-line
  body: 'Chỗ đầu là `dap_an_dung`, chỗ hai là `len`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai dung DUNG bien 'dap_an_dung' (dap an vet can vua tinh) de giao voi tap ket qua HNSW; VA cho trong hai phai GOI len(vectors) de dem so diem trong kho, khong duoc chep san hay dung mot danh sach khac
  requireAst:
  - kind: uses-name, target: dap_an_dung, min: 1
  - kind: uses-call, target: len, min: 10
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [1, 10] cho hai
  # luat theo dung thu tu khai bao o tren.
  # dap_an_dung=1 (TONG THAT, da xac nhan bang cong cu): CHI mot lan DOC
  # (Load) trong toan bo solution, dung o cho trong dau -- dong gan
  # "dap_an_dung = tim_k_lan_can_vet_can(...)" la Store, khong dem.
  # len=10 (TONG THAT, da xac nhan bang cong cu, khong doan tay): rai rac
  # trong nhieu ham co san (tim_k_lan_can_vet_can, chen_hnsw, v.v.), CONG
  # THEM 1 lan CHINH la cho trong hai. Neu chi dat min=1 (ngay tho), mot
  # mutant BO SOT cho trong hai (vi du dien "24" chep san, DUNG tren du lieu
  # CO DINH nay nhung khong con GOI ham that) van co the qua vi con lai
  # nhieu lan len(...) trong boilerplate -- GOTCHA "boilerplate-threshold-
  # masking"; da tu kiem chung: mutant nay VAN CHO RA output dung tren BO DU
  # LIEU CO DINH (vi 24 dung la gia tri that su cua len(vectors)), NHUNG bi
  # chan boi static (voi min=10, dung, dem giam xuong 9) -- xac nhan static
  # la tang BAT BUOC de bat mutant "chep san so dung tinh co" nay, tests/
  # output KHONG the phat hien tren du lieu CO DINH nay.
  # Da tu ra soat rieng GOTCHA #6 cho luat 'uses-call len': mot mutant dung
  # SAI danh sach nhung VAN la mot loi goi len() hop le -- vi du dien
  # "len(cac_cau_hoi_vector)" (do dai DANH SACH CAU HOI, bang 8, khong phai
  # 24) thay vi "len(vectors)" -- da tu chay THAT qua python3, xac nhan
  # static KHONG bat duoc (dem AST van la [1, 10], vi day VAN la mot loi goi
  # len() hop le, chi khac doi so) NHUNG bi bat boi tier 'tests'/'output':
  # so_sanh_vet_can tro thanh 8 * 8 = 64 (thay vi 192), va dong in cuoi cung
  # ("tong_so_sanh_hnsw < so_sanh_vet_can") tro thanh False (102 khong con
  # nho hon 64) thay vi True -- ca hai deu bi assertion rieng bat duoc.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua
  # python3 de xac nhan, khong doan tay): dien "len(vectors)" vao cho trong
  # dau ("tong_recall += len(set(len(vectors)) & set(ket_qua)) / k") VA dien
  # "dap_an_dung" vao cho trong hai ("so_sanh_vet_can = dap_an_dung *
  # so_cau_hoi") -- ket qua AST van la [1, 10], Y HET ban dung (mot lan doc
  # 'dap_an_dung' va muoi lan goi 'len' khong doi, chi doi VI TRI: 'len' gio
  # xuat hien them o cho trong dau thay vi cho trong hai, 'dap_an_dung' van
  # duoc doc 1 lan nhung o cho trong hai thay vi trong list-comprehension
  # nguyen ban -- vi 'dap_an_dung' o dong gan ban dau van con Store, chua
  # mat, va bien do RO RI ra khoi vong lap 'for' con Python, nen van doc
  # duoc o ben ngoai). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': da tu chay THAT qua python3, xac nhan
  # no nem TypeError: 'int' object is not iterable NGAY LAP TUC (vi
  # set(len(vectors)) goi set() tren MOT SO NGUYEN, khong phai mot danh
  # sach) — bi chan boi tier 'run', doc lap voi static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^24\\n0\\.7917\\n102\\n192\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Recall `≈0,7917`, `102` phép so sánh HNSW so với `192` của vét cạn — một sự
đánh đổi đo được bằng số thật, không suy đoán. Quest `hnsw-tu-cai` (q8.5c)
đóng tại `8/8` bài.
::::

::::reflect{#nghi-lai}
Quest này mở ra với một giới hạn đã đo ở quest trước: vét cạn (q8.5b) LUÔN
đúng nhưng tốn `N` phép so sánh MỖI câu hỏi. Tám bài đã trả lời từng mảnh
một:

> **`y-tuong-do-thi-dieu-huong-nho`** (bài `1`) — trên một đồ thị đã dựng
> sẵn, `2` bước nhảy đủ để tới đáp án đúng, so với `20` phép so sánh của vét
> cạn.
>
> **`xay-do-thi-mot-tang-don-gian`** (bài `2`) — tự xây đồ thị đó: mỗi điểm
> nối tới `M=3` láng giềng gần nhất, có cắt bớt để giữ đồ thị thưa.
>
> **`tim-kiem-tham-lam-tren-do-thi`** (bài `3`) — thuật toán tìm kiếm tham
> lam đầy đủ, đo được `2` bước nhảy, `9` phép so sánh cho một truy vấn cụ
> thể.
>
> **`gioi-han-mot-tang-va-y-tuong-nhieu-tang`** (bài `4`) — đo giới hạn có
> thật: `14/20` điểm bắt đầu cho kết quả SAI trên đồ thị một tầng.
>
> **`gan-tang-ngau-nhien-co-seed`** (bài `5`) — phân bố tầng giảm dần, tất
> định, qua `random.Random(seed)`.
>
> **`thuat-toan-chen-hnsw`** (bài `6`) — ráp thuật toán chèn nhiều tầng đầy
> đủ, đo cấu trúc đồ thị sinh ra.
>
> **`thuat-toan-tim-kiem-hnsw-day-du`** (bài `7`) — ráp thuật toán tìm kiếm
> nhiều tầng, đo recall `≈0,7917` và `13,375` phép so sánh trung bình — ít
> hơn `N=20`.
>
> **`boss-do-recall-va-toc-do-so-voi-vet-can`** (bài `8`, quest này) — ráp
> toàn bộ trên một tài liệu `24` câu mới, đo `102` so với `192` phép so
> sánh (giảm `≈47%`), đổi lấy recall `≈0,7917` thay vì `100%`.

Một điều CỐ Ý xuyên suốt cả `8` bài: HNSW không phải một "hộp đen" — nó là
BA ý tưởng đơn giản (đồ thị thưa, tìm tham lam, nhiều tầng có gán ngẫu
nhiên) ráp lại, mỗi ý tưởng đo được bằng số thật, không suy đoán. Track
`T8.5` "RAG từ số `0`" tiếp tục từ đây với `bm25-va-tim-kiem-tu-khoa`
(q8.5d): một cách tìm kiếm KHÁC hẳn — theo TỪ KHOÁ, không theo vector —
để bổ sung cho những trường hợp mà cả vét cạn LẪN HNSW đều không bắt được
tốt (một tên riêng hiếm gặp, một mã số cụ thể).
::::

::::checkpoint{mastery=0.9}
::::
