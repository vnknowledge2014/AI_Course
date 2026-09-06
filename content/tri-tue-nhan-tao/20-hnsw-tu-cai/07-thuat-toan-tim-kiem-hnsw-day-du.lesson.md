---
id: tri-tue-nhan-tao.hnsw-tu-cai.thuat-toan-tim-kiem-hnsw-day-du
title: "Thuật toán tìm kiếm HNSW đầy đủ: từ tầng cao nhất xuống tầng 0"
summary: "tim_kiem_hnsw(do_thi, vectors, vector_muc_tieu, diem_vao, tang_cao_nhat, k) bat dau o diem_vao cua tang cao nhat, TIM THAM LAM (bai 3) xuong tung tang toi tang 1 (chi de dinh vi), roi o tang 0 tim tham lam LAN CUOI va lay ca diem hoi tu LAN lang gieng truc tiep cua no lam ung vien, tra ve k ung vien gan nhat. Tren N=20 diem (seed=29, M=3), trung binh tren 8 cau hoi: recall@3 = 0,7917 (19/24), TRUNG BINH 13,375 phep so sanh -- IT HON N=20 that su, dung mot lan chay that."
locale: vi
track: tri-tue-nhan-tao
module: hnsw-tu-cai
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.thuat-toan-tim-kiem-hnsw-day-du]
requires: [ai.thuat-toan-chen-hnsw]
concepts: [ai.thuat-toan-tim-kiem-hnsw-day-du]
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
Đồ thị nhiều tầng đã dựng xong (bài trước). Bài này viết thuật toán TÌM
KIẾM trên nó — và đo xem cấu trúc nhiều tầng có thật sự sửa được giới hạn
đã đo ở bài `4` hay không.
::::

::::explain{#thuat_toan_tim_kiem_hnsw}
Tìm kiếm trên đồ thị HNSW nhiều tầng: bắt đầu ở **điểm vào** (`diem_vao`)
của **tầng cao nhất** (`tang_cao_nhat`) — hai giá trị này do `chen_hnsw`
(bài trước) trả về. Đi XUỐNG dần:

> **Từ tầng cao nhất xuống tầng `1`** — mỗi tầng, tìm tham lam (bài `3`)
> CHỈ để cập nhật điểm hiện tại thành điểm gần mục tiêu nhất tìm được TẠI
> TẦNG ĐÓ; kết quả trở thành điểm bắt đầu cho tầng NGAY DƯỚI nó (tầng thưa
> hơn, cạnh "xa" hơn, giúp nhảy nhanh qua nhiều điểm).
>
> **Tại tầng `0`** — tầng chứa TẤT CẢ điểm, cạnh "gần" hơn: tìm tham lam
> LẦN CUỐI để hội tụ, rồi lấy tập ỨNG VIÊN gồm điểm hội tụ đó VÀ các láng
> giềng TRỰC TIẾP của nó tại tầng `0` — không chỉ MỘT điểm, một danh sách
> nhỏ để chọn ra `k` kết quả tốt nhất trong số đó (đơn giản hoá hợp lý:
> đây là "tinh chỉnh" cuối cùng, tận dụng tầng dày đặc nhất).

Đo **recall@k**: tỉ lệ trong `k` kết quả HNSW trả về THẬT SỰ nằm trong `k`
kết quả ĐÚNG của vét cạn (q8.5b) — vét cạn LUÔN đúng, nên nó là "đáp án
chuẩn" để đối chiếu. Đo **số phép so sánh**: tổng số lần `tuong_dong_cosine`
được gọi thật trong suốt quá trình tìm kiếm (tại mỗi tầng đi qua, cộng dồn
số láng giềng đã xét).
::::

::::example{#do_recall_va_so_sanh}
Dùng lại ĐÚNG đồ thị nhiều tầng của bài trước (`N=20`, `seed=29`, `M=3`) —
tìm kiếm `k=3` trên `8` câu hỏi khác nhau, đối chiếu với vét cạn:

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


DIEM = [
    "may tinh chay phan mem manh", "lap trinh vien viet thuat toan hay", "vi xu ly xu ly du lieu nhanh",
    "ket noi mang giup ung dung on dinh", "thuat toan sap xep du lieu chuan", "ung dung may tinh xu ly du lieu lon",
    "vi xu ly manh giup ung dung nhanh", "phan mem lap trinh toi uu thuat toan", "du lieu duoc may tinh xu ly tu dong",
    "ung dung ket noi mang truyen du lieu",
    "mon an ngon can gia vi cong thuc", "dau bep nau an trong nha bep sach", "thuc pham tuoi giup mon an ngon",
    "cong thuc nau an don gian de lam", "mon trang mieng ngot ngao sau bua an", "nha bep sach se giup dau bep thoai mai",
    "gia vi dam da giup mon an hap dan", "dau bep gioi che bien thuc pham tuoi", "mon an voi cong thuc gia vi vua an",
    "thuc pham nau thanh mon trang mieng ngon",
]
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DIEM]
N = len(VEC)

tang_diem = gan_tang_ngau_nhien(29, N, 0.5)
do_thi, diem_vao, tang_cao_nhat = chen_hnsw(VEC, tang_diem, 3)

QUERIES = [
  "may tinh chay ung dung xu ly du lieu bang thuat toan",
  "mon an ngon voi gia vi va cong thuc nau",
  "phan mem lap trinh toi uu thuat toan hay",
  "dau bep nau mon trang mieng ngot ngao",
  "ket noi mang on dinh cho ung dung chay",
  "thuc pham tuoi va nha bep sach se",
  "vi xu ly du lieu nhanh cho may tinh",
  "gia vi vua an cho mon an ngon",
]

tong_recall = 0.0
tong_so_sanh = 0
for q in QUERIES:
    qv = tinh_vector_dem_tu(q, TU_VUNG, CUM_TU_GOC)
    dung3 = tim_k_lan_can_vet_can(qv, VEC, 3)
    ket_qua, so_sanh = tim_kiem_hnsw(do_thi, VEC, qv, diem_vao, tang_cao_nhat, 3)
    tong_recall += len(set(dung3) & set(ket_qua)) / 3
    tong_so_sanh += so_sanh

recall_trung_binh = tong_recall / len(QUERIES)
so_sanh_trung_binh = tong_so_sanh / len(QUERIES)

print(round(recall_trung_binh, 4))
print(so_sanh_trung_binh)
print(so_sanh_trung_binh < N)
```

```text title=readonly
0.7917
13.375
True
```

Trên `8` câu hỏi: recall@3 trung bình `≈0,7917` (`19` trong tổng `24` kết
quả đúng, vì `8 × 3 = 24`) — không hoàn hảo (vét cạn luôn `100%`), nhưng
CAO. Số phép so sánh trung bình `13,375` — ÍT HƠN `N=20` thật sự, đo được
bằng số. Đây chính là sự ĐÁNH ĐỔI cốt lõi của HNSW: recall không tuyệt đối,
nhưng số phép so sánh giảm — bài BOSS đo đánh đổi này trên một kho lớn hơn.
::::

::::predict{#doan_recall_khong_hoan_hao commitOnce}
Xét đúng ví dụ trên: vét cạn (q8.5b) LUÔN cho recall `100%` (nó không bỏ
sót phần tử nào). HNSW cho recall trung bình `≈0,7917` trên `8` câu hỏi.

**Trước khi chạy thử**, bạn đoán: recall của HNSW có luôn bằng `100%` như
vét cạn không?

:::opt{correct}
Không — HNSW đi tham lam qua các tầng, có thể HỘI TỤ ở một cực trị cục bộ
không hoàn toàn trùng với `top-k` đúng (giống giới hạn đã đo ở bài `4`, dù
đã giảm bớt nhờ nhiều tầng); recall trung bình đo được là `≈0,7917`, không
phải `1,0`
:::

:::opt
Có — vì đồ thị nhiều tầng đã được xây để GIẢI QUYẾT đúng giới hạn của một
tầng (bài `4`), nên tìm kiếm trên nó phải luôn tìm ra đúng `top-k`, giống
hệt vét cạn
::why
Gần đúng ở việc nhiều tầng THẬT SỰ cải thiện tình hình so với một tầng
(bài `4` đo `14/20` điểm bắt đầu sai; ở đây với nhiều tầng, hầu hết câu hỏi
đều đúng HOÀN TOÀN hoặc gần hoàn toàn) — quan sát về sự CẢI THIỆN đó đúng.

Chỗ lệch: "cải thiện" không có nghĩa là "hoàn hảo tuyệt đối". Đi tham lam
VẪN có thể hội tụ ở một cực trị cục bộ không trùng khớp `100%` với `top-k`
thật — recall đo được là `≈0,7917`, cao nhưng KHÔNG bằng `1,0`. Đây chính
là bản chất "gần đúng" (approximate) của HNSW — nó đánh đổi độ chính xác
tuyệt đối lấy tốc độ.
::
:::

:::opt
Có, luôn luôn — bất kỳ thuật toán tìm kiếm nào dùng cosine similarity thật
(không xấp xỉ) đều phải cho ra đáp án đúng tuyệt đối, vì phép đo "gần nhau"
không hề sai
::why
Gần đúng ở việc `tuong_dong_cosine` THẬT SỰ tính đúng, không có sai số nào
trong phép đo similarity từng cặp — quan sát đó đúng.

Chỗ lệch: việc TỪNG phép đo similarity đúng không đảm bảo THUẬT TOÁN duyệt
qua ĐỦ các điểm cần thiết để tìm ra top-k thật — HNSW chỉ xét một tập NHỎ
ứng viên (những điểm nằm trên đường đi tham lam qua các tầng), không so
sánh với MỌI điểm như vét cạn. Chính việc bỏ qua phần lớn điểm mới là lý do
recall có thể dưới `100%`, dù mỗi phép so sánh riêng lẻ đều chính xác.
::
:::
::::

::::code{#viet_tim_kiem_hnsw}
Hoàn thiện `tim_kiem_hnsw`: đi từ tầng cao nhất XUỐNG tầng `1`, rồi tại
tầng `0`, gộp điểm hội tụ VÀ láng giềng trực tiếp của nó làm ứng viên.

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
    for t in range(___, 0, -1):                                       # tang_cao_nhat
        hien_tai, duong_di = tim_tham_lam(do_thi[t], vectors, vector_muc_tieu, hien_tai)
        tong_so_sanh += sum(len(do_thi[t][n]) for n in duong_di)
    hien_tai, duong_di = tim_tham_lam(do_thi[0], vectors, vector_muc_tieu, hien_tai)
    tong_so_sanh += sum(len(do_thi[0][n]) for n in duong_di)
    ung_vien = set([hien_tai]) | set(do_thi[0][___])                    # hien_tai
    tong_so_sanh += len(ung_vien)
    xep = sorted(ung_vien, key=lambda j: tuong_dong_cosine(vectors[j], vector_muc_tieu), reverse=True)
    return xep[:k], tong_so_sanh


DIEM = [
    "may tinh chay phan mem manh", "lap trinh vien viet thuat toan hay", "vi xu ly xu ly du lieu nhanh",
    "ket noi mang giup ung dung on dinh", "thuat toan sap xep du lieu chuan", "ung dung may tinh xu ly du lieu lon",
    "vi xu ly manh giup ung dung nhanh", "phan mem lap trinh toi uu thuat toan", "du lieu duoc may tinh xu ly tu dong",
    "ung dung ket noi mang truyen du lieu",
    "mon an ngon can gia vi cong thuc", "dau bep nau an trong nha bep sach", "thuc pham tuoi giup mon an ngon",
    "cong thuc nau an don gian de lam", "mon trang mieng ngot ngao sau bua an", "nha bep sach se giup dau bep thoai mai",
    "gia vi dam da giup mon an hap dan", "dau bep gioi che bien thuc pham tuoi", "mon an voi cong thuc gia vi vua an",
    "thuc pham nau thanh mon trang mieng ngon",
]
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DIEM]
N = len(VEC)

tang_diem = gan_tang_ngau_nhien(29, N, 0.5)
do_thi, diem_vao, tang_cao_nhat = chen_hnsw(VEC, tang_diem, 3)

QUERIES = [
  "may tinh chay ung dung xu ly du lieu bang thuat toan",
  "mon an ngon voi gia vi va cong thuc nau",
  "phan mem lap trinh toi uu thuat toan hay",
  "dau bep nau mon trang mieng ngot ngao",
  "ket noi mang on dinh cho ung dung chay",
  "thuc pham tuoi va nha bep sach se",
  "vi xu ly du lieu nhanh cho may tinh",
  "gia vi vua an cho mon an ngon",
]

tong_recall = 0.0
tong_so_sanh = 0
for q in QUERIES:
    qv = tinh_vector_dem_tu(q, TU_VUNG, CUM_TU_GOC)
    dung3 = tim_k_lan_can_vet_can(qv, VEC, 3)
    ket_qua, so_sanh = tim_kiem_hnsw(do_thi, VEC, qv, diem_vao, tang_cao_nhat, 3)
    tong_recall += len(set(dung3) & set(ket_qua)) / 3
    tong_so_sanh += so_sanh

recall_trung_binh = tong_recall / len(QUERIES)
so_sanh_trung_binh = tong_so_sanh / len(QUERIES)

print(round(recall_trung_binh, 4))
print(so_sanh_trung_binh)
print(so_sanh_trung_binh < N)
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


DIEM = [
    "may tinh chay phan mem manh", "lap trinh vien viet thuat toan hay", "vi xu ly xu ly du lieu nhanh",
    "ket noi mang giup ung dung on dinh", "thuat toan sap xep du lieu chuan", "ung dung may tinh xu ly du lieu lon",
    "vi xu ly manh giup ung dung nhanh", "phan mem lap trinh toi uu thuat toan", "du lieu duoc may tinh xu ly tu dong",
    "ung dung ket noi mang truyen du lieu",
    "mon an ngon can gia vi cong thuc", "dau bep nau an trong nha bep sach", "thuc pham tuoi giup mon an ngon",
    "cong thuc nau an don gian de lam", "mon trang mieng ngot ngao sau bua an", "nha bep sach se giup dau bep thoai mai",
    "gia vi dam da giup mon an hap dan", "dau bep gioi che bien thuc pham tuoi", "mon an voi cong thuc gia vi vua an",
    "thuc pham nau thanh mon trang mieng ngon",
]
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DIEM]
N = len(VEC)

tang_diem = gan_tang_ngau_nhien(29, N, 0.5)
do_thi, diem_vao, tang_cao_nhat = chen_hnsw(VEC, tang_diem, 3)

QUERIES = [
  "may tinh chay ung dung xu ly du lieu bang thuat toan",
  "mon an ngon voi gia vi va cong thuc nau",
  "phan mem lap trinh toi uu thuat toan hay",
  "dau bep nau mon trang mieng ngot ngao",
  "ket noi mang on dinh cho ung dung chay",
  "thuc pham tuoi va nha bep sach se",
  "vi xu ly du lieu nhanh cho may tinh",
  "gia vi vua an cho mon an ngon",
]

tong_recall = 0.0
tong_so_sanh = 0
for q in QUERIES:
    qv = tinh_vector_dem_tu(q, TU_VUNG, CUM_TU_GOC)
    dung3 = tim_k_lan_can_vet_can(qv, VEC, 3)
    ket_qua, so_sanh = tim_kiem_hnsw(do_thi, VEC, qv, diem_vao, tang_cao_nhat, 3)
    tong_recall += len(set(dung3) & set(ket_qua)) / 3
    tong_so_sanh += so_sanh

recall_trung_binh = tong_recall / len(QUERIES)
so_sanh_trung_binh = tong_so_sanh / len(QUERIES)

print(round(recall_trung_binh, 4))
print(so_sanh_trung_binh)
print(so_sanh_trung_binh < N)
```

```python title=test
assert round(recall_trung_binh, 4) == 0.7917, f"recall trung binh phai xap xi 0.7917 -- dang ra {recall_trung_binh}"
assert so_sanh_trung_binh == 13.375, f"so phep so sanh trung binh phai la 13.375 -- dang ra {so_sanh_trung_binh}"
assert so_sanh_trung_binh < N, "so phep so sanh trung binh phai IT HON N=20"
assert round(tong_recall, 4) == round(19 / 3, 4), f"tong recall (truoc khi chia deu) phai xap xi 19/3 -- dang ra {tong_recall}"

# kiem tra truc tiep tren mot vi du nho, tu tinh tay duoc: 4 diem, 2 tang (bai truoc da dung)
vec_nho = [[10, 0], [9, 1], [0, 10], [1, 9]]
tang_nho = [1, 0, 1, 0]
do_thi_nho, diem_vao_nho, tang_cao_nhat_nho = chen_hnsw(vec_nho, tang_nho, 2)
ket_qua_nho, so_sanh_nho = tim_kiem_hnsw(do_thi_nho, vec_nho, [10, 0], diem_vao_nho, tang_cao_nhat_nho, 2)
assert ket_qua_nho == [0, 1], f"top-2 gan [10,0] nhat phai la [0, 1] -- dang ra {ket_qua_nho}"
assert so_sanh_nho == 4, f"so phep so sanh tren do thi nho phai la 4 -- dang ra {so_sanh_nho}"

# bien: k=1 phai tra ve DUNG 1 ket qua, la chi so dau tien cua k=2
ket_qua_k1, _ = tim_kiem_hnsw(do_thi_nho, vec_nho, [10, 0], diem_vao_nho, tang_cao_nhat_nho, 1)
assert ket_qua_k1 == [ket_qua_nho[0]], f"k=1 phai cho dung chi so dau tien cua k=2 -- dang ra {ket_qua_k1}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai vị trí khác nhau. Chỗ đầu là ĐIỂM BẮT ĐẦU của vòng lặp `range(...)` — tầng CAO NHẤT hiện có trong đồ thị, nơi việc đi xuống bắt đầu. Chỗ hai là CHỈ SỐ dùng để tra cứu láng giềng tại tầng `0` của điểm VỪA hội tụ — chính là biến vừa được gán ở dòng ngay phía trên.
- kind: strategy
  body: 'Chỗ đầu: `tang_cao_nhat` — tầng cao nhất hiện có trong đồ thị (đối số truyền vào hàm). Chỗ hai: `hien_tai` — điểm vừa hội tụ ở tầng `0`, dùng để lấy danh sách láng giềng trực tiếp của nó.'
- kind: one-line
  body: 'Chỗ đầu là `tang_cao_nhat`, chỗ hai là `hien_tai`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai dung DUNG bien 'tang_cao_nhat' lam diem bat dau cua range (khong duoc chep san mot so tang co dinh); VA cho trong hai phai dung DUNG bien 'hien_tai' de lay lang gieng cua diem vua hoi tu tai tang 0
  requireAst:
  - kind: uses-name, target: tang_cao_nhat, min: 6
  - kind: uses-name, target: hien_tai, min: 11
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [6, 11] cho hai
  # luat theo dung thu tu khai bao o tren.
  # tang_cao_nhat=6 (TONG THAT, da xac nhan bang cong cu, khong doan tay):
  # doc lai o chen_hnsw ("tang_cao_nhat = L" la Store khong dem, nhung
  # "for t in range(tang_cao_nhat, L, -1):" (1), "if L > tang_cao_nhat:"
  # (1) CO doc), o dinh nghia ham tim_kiem_hnsw (tham so, khong dem, la
  # ast.arg), VA o CHINH THAN ham (cho trong dau, 1 lan). Neu chi dat min=1
  # (ngay tho), mot mutant dien mot HANG SO co dinh (vi du "4") van co the
  # qua NEU dung DUNG bang gia tri that su cua tang_cao_nhat=4 tren du lieu
  # nay (trung hop VE MAT SO, sai VE MAT Y NGHIA neu doi seed/N) -- da tu
  # kiem chung: dien "4" van cho RA DUNG output tren BO DU LIEU CO DINH nay
  # (vi tang_cao_nhat THAT SU bang 4), NHUNG bi chan boi static (dem AST
  # giam xuong duoi min=6, vi khong con doc bien 'tang_cao_nhat' o cho
  # trong nua) -- xac nhan static la tang BAT BUOC cho mutant "hang so
  # trung hop" nay, tests/output KHONG the phat hien.
  # hien_tai=11 (TONG THAT, da xac nhan bang cong cu): rat nhieu lan doc rai
  # rac trong CA than ham tim_kiem_hnsw (gan lai, doc de goi tim_tham_lam
  # nhieu lan qua cac tang, doc o dong "ung_vien = set([hien_tai]) |
  # set(do_thi[0][hien_tai])" CO SAN mot lan, VA cho trong hai la lan THEM).
  # Neu chi dat min=1, mot mutant dien "diem_vao" (ten khac, cung la mot
  # diem hop le trong do_thi[0]) van co the CHAY duoc (khong loi) nhung SAI
  # Y NGHIA -- da tu kiem chung: bi chan CA boi static (voi min=11, dung)
  # LAN boi assertion "ket_qua_nho == [0, 1]" tren vi du nho (danh sach ung
  # vien sai se cho ket qua khac).
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua
  # python3 de xac nhan, khong doan tay): dien "hien_tai" vao cho trong dau
  # ("for t in range(hien_tai, 0, -1):") VA dien "tang_cao_nhat" vao cho
  # trong hai ("set(do_thi[0][tang_cao_nhat])") -- ket qua AST la [6, 11], Y
  # HET ban dung (hai ten hoan toan doi cho, tong so lan doc moi ten khong
  # doi). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'tests'/'output', KHONG phai 'run' (khac voi
  # phan lon mutant hoan doi khac trong quest nay): da tu chay THAT qua
  # python3, xac nhan no KHONG nem loi nao (ca 'hien_tai' -- gia tri diem_vao
  # ban dau -- lan 'tang_cao_nhat' -- mot chi so diem hop le trong do_thi[0]
  # tren du lieu N=20 nay -- deu la gia tri HOP LE de dung lam range/chi so)
  # nhung cho ra ket qua SAI HAN: recall_trung_binh doi tu 0,7917 xuong con
  # 0,4583, so_sanh_trung_binh doi tu 13,375 thanh 14,5 -- CA HAI deu khac
  # gia tri mong doi, bi bat boi assertion rieng cho tung so.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^0\\.7917\\n13\\.375\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Recall `≈0,7917`, `13,375` phép so sánh trung bình — ít hơn `N=20` thật sự.
Bài BOSS ráp toàn bộ HNSW trên một kho tài liệu lớn hơn, đo đánh đổi này một
lần cuối.
::::

::::reflect{#nghi-lai}
`tim_kiem_hnsw` không phải một thuật toán tách biệt — nó là `tim_tham_lam`
(bài `3`) được gọi LẶP LẠI, mỗi lần trên MỘT tầng, đi từ thưa (nhảy xa,
nhanh) tới dày đặc (tinh chỉnh, chính xác hơn). Kết quả đo được trên `8`
câu hỏi: recall `≈0,7917` (không hoàn hảo — vẫn còn dấu vết của giới hạn đã
đo ở bài `4`, dù đã giảm bớt nhiều nhờ nhiều tầng), và số phép so sánh
trung bình `13,375` — RÕ RÀNG ít hơn `N=20` mà vét cạn luôn cần. Đây chính
là bản chất "tìm kiếm GẦN ĐÚNG" (approximate nearest neighbor) của HNSW —
không cam kết đúng tuyệt đối như vét cạn, nhưng đổi lại tốc độ. Bài BOSS đo
lại đúng sự đánh đổi này, trên một kho tài liệu MỚI, với nhiều câu hỏi hơn.
::::

::::checkpoint{mastery=0.85}
::::
