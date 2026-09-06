---
id: tri-tue-nhan-tao.hnsw-tu-cai.gioi-han-mot-tang-va-y-tuong-nhieu-tang
title: "Giới hạn một tầng: kẹt ở cực trị sai, và ý tưởng nhiều tầng"
summary: "Tren DUNG do thi 20 diem (M=3) cua bai truoc: tim tham lam tu diem 0 (chu de cong nghe) DUNG LAI NGAY tai 0 (0 buoc nhay) -- SAI, vi dap an dung (vet can) la diem 5. Ca 3 lang gieng cua diem 0 (7, 3, 2) co cosine <= cosine cua diem 0, khong lang gieng nao THUC SU gan hon. Quet CA 20 diem bat dau: dem_so_diem_bat_dau_sai = 14/20 -- 70% diem bat dau cho ket qua SAI tren do thi MOT TANG nay. Y tuong sua: nhieu tang, moi tang thua hon, cho phep nhay XA truoc khi tinh chinh."
locale: vi
track: tri-tue-nhan-tao
module: hnsw-tu-cai
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.gioi-han-mot-tang-va-y-tuong-nhieu-tang]
requires: [ai.tim-kiem-tham-lam-tren-do-thi]
concepts: [ai.gioi-han-mot-tang-va-y-tuong-nhieu-tang]
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
Bài trước: từ điểm `2`, tìm tham lam tới đúng đáp án chỉ trong `2` bước. Bài
này chọn một điểm bắt đầu KHÁC — và đo một kết quả không đẹp như vậy.
::::

::::explain{#gioi_han_cua_mot_tang}
`tim_tham_lam` (bài trước) dừng lại khi không còn láng giềng nào gần mục
tiêu HƠN điểm hiện tại. Điểm dừng đó là một **cực trị cục bộ theo đồ thị**
— nhưng "cục bộ" nghĩa là nó chỉ xét những láng giềng TRỰC TIẾP, không xét
TOÀN BỘ đồ thị. Nếu điểm bắt đầu nằm ở một khu vực mà TẤT CẢ láng giềng trực
tiếp của cực trị cục bộ đó đều KHÔNG gần mục tiêu hơn — thuật toán dừng lại
ở đó, dù một điểm KHÁC (không phải láng giềng trực tiếp) mới thực sự là đáp
án đúng.

Đây KHÔNG phải một lỗi trong cách viết `tim_tham_lam` — đó là hệ quả TẤT
YẾU của việc dùng một đồ thị THƯA (mỗi đỉnh chỉ nối tới `M` láng giềng, bài
`xay-do-thi-mot-tang-don-gian`): càng thưa, càng dễ xảy ra tình huống này.

Bài này đo CHÍNH XÁC bằng số: trên đồ thị `20` điểm, `M=3` (hai bài trước),
bao nhiêu PHẦN TRĂM điểm bắt đầu cho ra kết quả SAI so với đáp án đúng
(theo vét cạn, q8.5b)?
::::

::::example{#do_ket_qua_sai_tu_diem_0}
Dùng lại ĐÚNG đồ thị `20` điểm, `M=3` và `CAU_HOI` của hai bài trước — lần
này bắt đầu tìm tham lam từ điểm `0`, và quét luôn CẢ `20` điểm bắt đầu:

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


def tim_k_lan_can_vet_can(vector_cau_hoi, danh_sach_vector_kho, k):
    do_tuong_dong = [tuong_dong_cosine(vector_cau_hoi, v) for v in danh_sach_vector_kho]
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k]


def xay_do_thi_mot_tang(vectors, M):
    do_thi = {i: [] for i in range(len(vectors))}
    for i in range(1, len(vectors)):
        ung_vien = list(range(i))
        sap_xep = sorted(((tuong_dong_cosine(vectors[i], vectors[j]), j) for j in ung_vien), reverse=True)
        chon = [j for _, j in sap_xep[:M]]
        for j in chon:
            do_thi[i].append(j)
            do_thi[j].append(i)
            if len(do_thi[j]) > M:
                lg = sorted(((tuong_dong_cosine(vectors[j], vectors[n]), n) for n in do_thi[j]), reverse=True)
                do_thi[j] = [n for _, n in lg[:M]]
    return do_thi


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


def dem_so_diem_bat_dau_sai(do_thi, vectors, vector_muc_tieu, dap_an_dung):
    so_sai = 0
    for diem_bat_dau in range(len(vectors)):
        ket_qua, _ = tim_tham_lam(do_thi, vectors, vector_muc_tieu, diem_bat_dau)
        if ket_qua != dap_an_dung:
            so_sai += 1
    return so_sai


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
do_thi = xay_do_thi_mot_tang(VEC, 3)

CAU_HOI = "may tinh chay ung dung xu ly du lieu bang thuat toan"
vector_cau_hoi = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)
dap_an_dung = tim_k_lan_can_vet_can(vector_cau_hoi, VEC, 1)[0]

ket_qua_tu_0, duong_di_tu_0 = tim_tham_lam(do_thi, VEC, vector_cau_hoi, 0)
so_sai = dem_so_diem_bat_dau_sai(do_thi, VEC, vector_cau_hoi, dap_an_dung)

print(dap_an_dung)
print(ket_qua_tu_0)
print(duong_di_tu_0)
print(so_sai)
```

```text title=readonly
5
0
[0]
14
```

Đáp án ĐÚNG (vét cạn) là điểm `5`. Bắt đầu tìm tham lam từ điểm `0`: đường
đi chỉ có `[0]` — DỪNG NGAY LẬP TỨC, không nhảy bước nào. Vì sao? Ba láng
giềng của điểm `0` là `[7, 3, 2]`, với cosine similarity lần lượt `≈0,2887`,
`≈0,3536`, `≈0,3536` — so với cosine của chính điểm `0` là `≈0,3536`.
KHÔNG láng giềng nào THỰC SỰ lớn hơn (hai láng giềng bằng, một láng giềng
thấp hơn) — nên `co_lang_gieng_gan_hon` trả về `None` ngay từ bước đầu.
Điểm `0` là một cực trị cục bộ SAI: đúng đồ thị, sai đáp án.

Quét CẢ `20` điểm bắt đầu: `so_sai = 14` — nghĩa là `14/20 = 70%` điểm bắt
đầu cho ra kết quả KHÁC với đáp án đúng trên CHÍNH đồ thị một tầng này (phần
lớn là các điểm ẩm thực, dừng ngay tại chính mình vì cosine với câu hỏi
công nghệ luôn bằng `0,0` — không có láng giềng nào "gần hơn `0,0`" để
nhảy sang, nên chúng không bao giờ tới được khu vực công nghệ).
::::

::::predict{#doan_ty_le_sai_cao commitOnce}
Xét đúng ví dụ trên: đáp án đúng (vét cạn) là điểm `5`. Đồ thị có `20`
điểm, `10` điểm chủ đề khác hẳn (ẩm thực) với câu hỏi công nghệ.

**Trước khi chạy thử**, bạn đoán: quét CẢ `20` điểm làm điểm bắt đầu, có
BAO NHIÊU điểm cho ra kết quả SAI (khác điểm `5`)?

:::opt{correct}
Rất nhiều — `14/20` (`70%`) — bao gồm CẢ những điểm ẩm thực (dừng ngay tại
chính mình vì không láng giềng nào có cosine dương hơn `0,0` để nhảy sang)
LẪN một vài điểm công nghệ mắc kẹt ở cực trị cục bộ sai (như điểm `0`)
:::

:::opt
Chỉ khoảng `1` hoặc `2` điểm — vì đồ thị đã được xây bằng cosine similarity
THẬT, nên phần lớn điểm bắt đầu phải tìm ra đúng đáp án, chỉ một vài
trường hợp hiếm mới bị kẹt
::why
Gần đúng ở việc đồ thị THẬT SỰ được xây bằng cosine similarity thật (không
phải ngẫu nhiên) — quan sát về cách xây đồ thị đó đúng.

Chỗ lệch: xây đồ thị bằng similarity thật không đảm bảo MỌI điểm bắt đầu
đều "nhìn thấy đường" tới đáp án đúng — một đồ thị THƯA (`M=3` láng giềng
mỗi đỉnh, không phải nối tới mọi điểm) có thể có nhiều khu vực mà tìm tham
lam dừng lại quá sớm. Ở dữ liệu cụ thể này, số điểm bị kẹt là `14/20`, một
tỉ lệ rất cao — không phải hiếm.
::
:::

:::opt
`0` điểm — vì tìm tham lam luôn hội tụ về đúng đáp án toàn cục, đó chính là
định nghĩa của tìm kiếm tham lam trên một đồ thị điều hướng nhỏ
::why
Gần đúng ở việc "tìm kiếm tham lam" NGHE có vẻ như luôn tìm ra đáp án tốt
nhất — cái tên gợi ý điều đó.

Chỗ lệch: tìm tham lam CHỈ đảm bảo hội tụ về một CỰC TRỊ CỤC BỘ (không còn
láng giềng trực tiếp nào gần hơn) — không có gì đảm bảo cực trị cục bộ đó
trùng với đáp án đúng TOÀN CỤC. Bài này đo đúng một trường hợp nơi hai điều
đó khác nhau, bằng số cụ thể (`14/20`), không suy đoán.
::
:::
::::

::::code{#viet_dem_so_diem_bat_dau_sai}
Hoàn thiện `dem_so_diem_bat_dau_sai`: chạy `tim_tham_lam` (bài trước) từ
MỌI điểm trong kho, đếm bao nhiêu lần kết quả KHÁC với đáp án đúng.

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


def tim_k_lan_can_vet_can(vector_cau_hoi, danh_sach_vector_kho, k):
    do_tuong_dong = [tuong_dong_cosine(vector_cau_hoi, v) for v in danh_sach_vector_kho]
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k]


def xay_do_thi_mot_tang(vectors, M):
    do_thi = {i: [] for i in range(len(vectors))}
    for i in range(1, len(vectors)):
        ung_vien = list(range(i))
        sap_xep = sorted(((tuong_dong_cosine(vectors[i], vectors[j]), j) for j in ung_vien), reverse=True)
        chon = [j for _, j in sap_xep[:M]]
        for j in chon:
            do_thi[i].append(j)
            do_thi[j].append(i)
            if len(do_thi[j]) > M:
                lg = sorted(((tuong_dong_cosine(vectors[j], vectors[n]), n) for n in do_thi[j]), reverse=True)
                do_thi[j] = [n for _, n in lg[:M]]
    return do_thi


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


def dem_so_diem_bat_dau_sai(do_thi, vectors, vector_muc_tieu, dap_an_dung):
    so_sai = 0
    for diem_bat_dau in range(len(vectors)):
        ket_qua, _ = tim_tham_lam(do_thi, vectors, vector_muc_tieu, ___)   # diem_bat_dau
        if ket_qua ___ dap_an_dung:                                        # !=
            so_sai += 1
    return so_sai


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
do_thi = xay_do_thi_mot_tang(VEC, 3)

CAU_HOI = "may tinh chay ung dung xu ly du lieu bang thuat toan"
vector_cau_hoi = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)
dap_an_dung = tim_k_lan_can_vet_can(vector_cau_hoi, VEC, 1)[0]

ket_qua_tu_0, duong_di_tu_0 = tim_tham_lam(do_thi, VEC, vector_cau_hoi, 0)
so_sai = dem_so_diem_bat_dau_sai(do_thi, VEC, vector_cau_hoi, dap_an_dung)

print(dap_an_dung)
print(ket_qua_tu_0)
print(duong_di_tu_0)
print(so_sai)
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


def tim_k_lan_can_vet_can(vector_cau_hoi, danh_sach_vector_kho, k):
    do_tuong_dong = [tuong_dong_cosine(vector_cau_hoi, v) for v in danh_sach_vector_kho]
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k]


def xay_do_thi_mot_tang(vectors, M):
    do_thi = {i: [] for i in range(len(vectors))}
    for i in range(1, len(vectors)):
        ung_vien = list(range(i))
        sap_xep = sorted(((tuong_dong_cosine(vectors[i], vectors[j]), j) for j in ung_vien), reverse=True)
        chon = [j for _, j in sap_xep[:M]]
        for j in chon:
            do_thi[i].append(j)
            do_thi[j].append(i)
            if len(do_thi[j]) > M:
                lg = sorted(((tuong_dong_cosine(vectors[j], vectors[n]), n) for n in do_thi[j]), reverse=True)
                do_thi[j] = [n for _, n in lg[:M]]
    return do_thi


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


def dem_so_diem_bat_dau_sai(do_thi, vectors, vector_muc_tieu, dap_an_dung):
    so_sai = 0
    for diem_bat_dau in range(len(vectors)):
        ket_qua, _ = tim_tham_lam(do_thi, vectors, vector_muc_tieu, diem_bat_dau)
        if ket_qua != dap_an_dung:
            so_sai += 1
    return so_sai


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
do_thi = xay_do_thi_mot_tang(VEC, 3)

CAU_HOI = "may tinh chay ung dung xu ly du lieu bang thuat toan"
vector_cau_hoi = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)
dap_an_dung = tim_k_lan_can_vet_can(vector_cau_hoi, VEC, 1)[0]

ket_qua_tu_0, duong_di_tu_0 = tim_tham_lam(do_thi, VEC, vector_cau_hoi, 0)
so_sai = dem_so_diem_bat_dau_sai(do_thi, VEC, vector_cau_hoi, dap_an_dung)

print(dap_an_dung)
print(ket_qua_tu_0)
print(duong_di_tu_0)
print(so_sai)
```

```python title=test
assert dap_an_dung == 5, f"dap an dung (vet can) phai la 5 -- dang ra {dap_an_dung}"
assert ket_qua_tu_0 == 0, f"tu diem 0, tim tham lam phai dung lai NGAY tai 0 -- dang ra {ket_qua_tu_0}"
assert duong_di_tu_0 == [0], f"duong di tu diem 0 phai la [0] (khong nhay buoc nao) -- dang ra {duong_di_tu_0}"
assert ket_qua_tu_0 != dap_an_dung, "diem 0 phai la mot CUC TRI CUC BO SAI (khac dap an dung)"
assert so_sai == 14, f"so diem bat dau cho ket qua SAI phai la 14 (tren tong 20) -- dang ra {so_sai}"
assert so_sai > N // 2, "hon mot nua so diem bat dau phai cho ket qua SAI -- day chinh la gioi han cua mot tang"

# kiem tra truc tiep tren do thi do choi nho, tu tinh tay duoc (bai truoc da dung)
do_thi_nho = {0: [1], 1: [0, 2], 2: [1]}
vec_nho = [[0, 1], [1, 0], [1, 1]]
target_nho = [1, 1]
assert dem_so_diem_bat_dau_sai(do_thi_nho, vec_nho, target_nho, 2) == 1, "chi diem bat dau 0 phai cho ket qua sai (dung lai tai 0, cosine hoa voi diem 1)"

# bien: neu MOI diem bat dau deu toi dung dap an, so_sai phai la 0 (khong phai luon > 0)
do_thi_day_du = {0: [1, 2], 1: [0, 2], 2: [0, 1]}
vec_day_du = [[0, 0], [0, 1], [1, 1]]
assert dem_so_diem_bat_dau_sai(do_thi_day_du, vec_day_du, [1, 1], 2) == 0, "ca 3 diem bat dau deu phai toi dung diem 2 (cosine=1.0, cao nhat) tren do thi day du nay"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai dòng khác nhau trong vòng lặp `for diem_bat_dau in range(len(vectors)):`. Chỗ đầu là đối số THỨ TƯ của `tim_tham_lam` — điểm bắt đầu của lần chạy này (biến vòng lặp `diem_bat_dau`, KHÔNG phải một hằng số cố định). Chỗ hai là toán tử SO SÁNH giữa kết quả tìm được và đáp án đúng — đếm khi chúng KHÁC nhau.
- kind: strategy
  body: 'Chỗ đầu: `diem_bat_dau` — biến vòng lặp, thay đổi ở mỗi lần lặp. Chỗ hai: `!=` — so sánh khác, đếm khi `ket_qua` không bằng `dap_an_dung`.'
- kind: one-line
  body: 'Chỗ đầu là `diem_bat_dau`, chỗ hai là `!=`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai dung DUNG bien vong lap 'diem_bat_dau' lam diem xuat phat (khong duoc chep cung mot diem co dinh); VA cho trong hai phai SO SANH KHAC (dung toan tu '!=') giua ket_qua va dap_an_dung
  requireAst:
  - kind: uses-name, target: diem_bat_dau, min: 2
  - kind: uses-operator, target: "!=", min: 1
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [2, 1] cho hai
  # luat theo dung thu tu khai bao o tren.
  # diem_bat_dau=2 (TONG THAT, da xac nhan bang cong cu, khong doan tay):
  # MOT lan Load BEN TRONG than ham tim_tham_lam ("hien_tai =
  # diem_bat_dau" -- ham nay CO tham so cung ten "diem_bat_dau", trung hop
  # voi bien vong lap cua dem_so_diem_bat_dau_sai), MOT lan CHINH la cho
  # trong dau. Neu chi dat min=1 (ngay tho), mot mutant dien mot HANG SO co
  # dinh (vi du "0") thay vi bien vong lap van qua duoc vi con lai 1 lan
  # doc "diem_bat_dau" ben trong tim_tham_lam -- GOTCHA "boilerplate-
  # threshold-masking"; da tu kiem chung: mutant dien "0" nay lam so_sai
  # TANG VOT (moi lan lap deu chay lai tu diem 0, ket qua giong het nhau
  # NHUNG van bi dem la "sai" moi khi ket qua != dap_an_dung, boi vi tu
  # diem 0 KET QUA DA LA 0 != 5 -- so_sai se tro thanh 20, khong phai 14),
  # bi bat CA boi static (voi min=2, dung) LAN boi assertion "so_sai == 14".
  # "!="=1 (TONG THAT): CHI mot lan trong toan bo solution, dung o cho
  # trong hai. Khong co lan '!=' nao khac o boilerplate.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua
  # python3 de xac nhan, khong doan tay): dien "!=" vao cho trong dau
  # ("tim_tham_lam(do_thi, vectors, vector_muc_tieu, !=)") VA dien
  # "diem_bat_dau" vao cho trong hai ("if ket_qua diem_bat_dau
  # dap_an_dung:") -- CA HAI deu SAI CU PHAP NGAY LAP TUC (mot loi goi ham
  # khong the nhan mot toan tu tran lam doi so, mot phep so sanh khong the
  # co HAI ten bien lien tiep khong co toan tu o giua) -- da tu chay qua
  # python3, xac nhan ca hai deu nem SyntaxError o buoc phan tich cu phap,
  # TRUOC CA khi kiemAst() hay 'run' kip chay. Rui ro hoan doi o day khong
  # co thuc, da xac nhan bang chay that chu khong doan.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^5\\n0\\n\\[0\\]\\n14\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`14/20` điểm bắt đầu cho kết quả sai — một giới hạn đo được bằng số thật,
không phải một cảnh báo trừu tượng. Bài sau bắt đầu sửa nó: gán mỗi điểm
một TẦNG, ngẫu nhiên có seed.
::::

::::reflect{#nghi-lai}
Bài này không "sửa lỗi" gì cả — nó ĐO một giới hạn có thật của đồ thị MỘT
TẦNG: `70%` điểm bắt đầu (`14/20`) cho ra kết quả khác với đáp án đúng, trên
CHÍNH đồ thị đã xây ở hai bài trước. Nguyên nhân gốc: đồ thị càng THƯA (mỗi
đỉnh chỉ `M=3` láng giềng), càng dễ có những "túi" mà tìm tham lam dừng lại
quá sớm — đặc biệt khi điểm bắt đầu nằm xa khu vực chứa đáp án đúng (ví dụ
mọi điểm ẩm thực, khi câu hỏi thuộc chủ đề công nghệ). Ý tưởng sửa, mà HNSW
thực sự dùng: thay vì MỘT tầng thưa duy nhất, xây NHIỀU tầng — tầng trên
cùng cực THƯA (chỉ vài điểm "may mắn"), cho phép nhảy XA ngay từ đầu, rồi đi
xuống dần tới tầng dưới cùng (chứa TẤT CẢ điểm) để tinh chỉnh. Ba bài tiếp
theo xây đúng ý tưởng đó: gán tầng ngẫu nhiên có seed, thuật toán chèn nhiều
tầng, và thuật toán tìm kiếm nhiều tầng.
::::

::::checkpoint{mastery=0.85}
::::
