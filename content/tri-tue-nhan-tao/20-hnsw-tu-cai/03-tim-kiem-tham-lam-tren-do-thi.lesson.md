---
id: tri-tue-nhan-tao.hnsw-tu-cai.tim-kiem-tham-lam-tren-do-thi
title: "Tìm kiếm tham lam đầy đủ: lặp tới khi hội tụ"
summary: "tim_tham_lam(do_thi, vectors, vector_muc_tieu, diem_bat_dau) lap goi co_lang_gieng_gan_hon (bai truoc): moi vong, nhay sang lang gieng gan hon NEU CO, dung lai khi khong con lang gieng nao gan hon (cuc tri cuc bo theo do thi). Tren do thi 20 diem (M=3) cua bai truoc, tu diem 2 toi cau hoi cong nghe: duong di [2, 8, 5], DUNG 2 buoc nhay, DUNG 9 phep so sanh (= tong bac cua 3 diem tren duong di: 3+3+3) -- it hon N=20 rat nhieu."
locale: vi
track: tri-tue-nhan-tao
module: hnsw-tu-cai
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.tim-kiem-tham-lam-tren-do-thi]
requires: [ai.xay-do-thi-mot-tang-don-gian]
concepts: [ai.tim-kiem-tham-lam-tren-do-thi]
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
`co_lang_gieng_gan_hon` (bài `1`) làm ĐÚNG MỘT bước — xem có láng giềng nào
gần hơn không, và trả về láng giềng đó (hoặc `None`). Bài này lặp bước đó
tới khi hội tụ, thành một thuật toán tìm kiếm ĐẦY ĐỦ.
::::

::::explain{#tim_tham_lam_day_du}
Thuật toán tìm kiếm tham lam (greedy search) trên đồ thị một tầng: bắt đầu
ở một điểm, LẶP LẠI việc gọi `co_lang_gieng_gan_hon` — mỗi lần gọi, nếu có
láng giềng gần mục tiêu hơn điểm hiện tại, NHẢY sang láng giềng đó; nếu
không còn láng giềng nào gần hơn (`co_lang_gieng_gan_hon` trả về `None`),
DỪNG LẠI. Điểm dừng lại đó là một **cực trị cục bộ theo đồ thị** (local
optimum) — không nhất thiết là điểm gần mục tiêu nhất trong TOÀN BỘ kho
(bài sau sẽ đo một trường hợp cụ thể nơi hai điều đó KHÁC nhau).

Hai con số đáng đo cho MỘT truy vấn cụ thể:

> **Số bước nhảy** — độ dài đường đi trừ `1` (đường đi có `k` điểm thì cần
> `k-1` bước nhảy để đi hết nó).
>
> **Số phép so sánh** (tính cosine similarity) — TẠI MỖI điểm trên đường đi
> (kể cả điểm DỪNG LẠI cuối cùng), thuật toán phải xét TOÀN BỘ láng giềng
> của điểm đó để biết có nên nhảy tiếp hay dừng. Vậy tổng số phép so sánh
> đúng bằng TỔNG BẬC (số láng giềng) của MỌI điểm trên đường đi.
::::

::::example{#do_so_buoc_va_so_sanh}
Dùng lại đồ thị `20` điểm, `M=3` của bài trước — tìm tham lam từ điểm `2`
tới `CAU_HOI` (chủ đề công nghệ):

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

ket_qua, duong_di = tim_tham_lam(do_thi, VEC, vector_cau_hoi, 2)
so_buoc_nhay = len(duong_di) - 1
so_sanh = sum(len(do_thi[n]) for n in duong_di)

print(duong_di)
print(ket_qua)
print(so_buoc_nhay)
print(so_sanh)
```

```text title=readonly
[2, 8, 5]
5
2
9
```

Đường đi `[2, 8, 5]` có `3` điểm, nên `so_buoc_nhay = 3 - 1 = 2`. Tại MỖI
điểm trên đường đi (`2`, `8`, `5`), thuật toán xét toàn bộ láng giềng của nó
— mỗi điểm có đúng `3` láng giềng (bài trước: mọi đỉnh có `M=3` láng giềng)
— nên `so_sanh = 3 + 3 + 3 = 9`. So với vét cạn (`N=20` phép so sánh, q8.5b
bài `4`): `9 < 20` — ít hơn hẳn, đo được bằng số thật, không suy đoán.
::::

::::predict{#doan_so_sanh_bang_tong_bac commitOnce}
Xét đúng ví dụ trên: đường đi `[2, 8, 5]` có `3` điểm, mỗi điểm có đúng `3`
láng giềng (`M=3`, bài trước).

**Trước khi chạy thử**, bạn đoán: `so_sanh` (tổng số phép so sánh) bằng bao
nhiêu?

:::opt{correct}
`9` — tại MỖI trong `3` điểm trên đường đi, thuật toán phải xét TOÀN BỘ `3`
láng giềng của điểm đó (kể cả ở điểm DỪNG LẠI cuối cùng, để biết chắc không
còn láng giềng nào gần hơn) — tổng cộng `3 × 3 = 9`
:::

:::opt
`2` — bằng đúng số bước nhảy, vì mỗi bước nhảy chỉ cần MỘT phép so sánh để
quyết định có nhảy hay không
::why
Gần đúng ở việc MỖI bước nhảy CÓ gắn với một quyết định "nhảy hay không" —
quan sát đó đúng ở mức khái niệm.

Chỗ lệch: để đưa ra MỘT quyết định đó, thuật toán không chỉ so sánh MỘT
lần — nó phải xét TOÀN BỘ láng giềng của điểm hiện tại (ở đây là `3` láng
giềng mỗi điểm) để tìm ra láng giềng gần nhất, không chỉ dừng ở phép so
sánh đầu tiên. `2` bước nhảy nhưng `3` điểm được ghé qua (bao gồm điểm bắt
đầu VÀ điểm dừng), mỗi điểm cần `3` phép so sánh.
::
:::

:::opt
`3` — bằng đúng số điểm trên đường đi, vì mỗi điểm chỉ cần MỘT phép so sánh
đại diện để xác định vị trí của nó so với mục tiêu
::why
Gần đúng ở việc quan sát ĐÚNG số điểm trên đường đi (`3` điểm: `2`, `8`,
`5`) — con số đó đúng.

Chỗ lệch: "một phép so sánh đại diện" không phải cách thuật toán hoạt động
— tại MỖI điểm, nó phải so sánh với TỪNG láng giềng một (không phải một
phép so sánh gộp chung), và mỗi điểm ở đây có `3` láng giềng, nên số phép
so sánh THẬT SỰ là `3` điểm `× 3` láng giềng mỗi điểm `= 9`, không phải `3`.
::
:::
::::

::::code{#viet_tim_tham_lam}
Hoàn thiện `tim_tham_lam`: mỗi vòng lặp, gọi `co_lang_gieng_gan_hon` (bài
trước) để tìm láng giềng gần hơn; nếu có, nhảy sang nó và ghi lại vào đường
đi; nếu không, dừng lại.

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
        ke_tiep = ___(do_thi, vectors, vector_muc_tieu, hien_tai)   # co_lang_gieng_gan_hon
        if ke_tiep is None:
            break
        hien_tai = ke_tiep
        duong_di.___(hien_tai)                                      # append
    return hien_tai, duong_di


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

ket_qua, duong_di = tim_tham_lam(do_thi, VEC, vector_cau_hoi, 2)
so_buoc_nhay = len(duong_di) - 1
so_sanh = sum(len(do_thi[n]) for n in duong_di)

print(duong_di)
print(ket_qua)
print(so_buoc_nhay)
print(so_sanh)
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

ket_qua, duong_di = tim_tham_lam(do_thi, VEC, vector_cau_hoi, 2)
so_buoc_nhay = len(duong_di) - 1
so_sanh = sum(len(do_thi[n]) for n in duong_di)

print(duong_di)
print(ket_qua)
print(so_buoc_nhay)
print(so_sanh)
```

```python title=test
assert duong_di == [2, 8, 5], f"duong di phai la [2, 8, 5] -- dang ra {duong_di}"
assert ket_qua == 5, f"ket qua tim kiem phai la 5 -- dang ra {ket_qua}"
assert so_buoc_nhay == 2, f"so buoc nhay phai la 2 -- dang ra {so_buoc_nhay}"
assert so_sanh == 9, f"so phep so sanh phai la 9 (= 3+3+3) -- dang ra {so_sanh}"
assert so_sanh < N, "so phep so sanh phai IT HON N=20 (loi ich cua tim kiem tren do thi)"

# kiem tra truc tiep tren mot do thi do choi nho, tu tinh tay duoc: duong thang 4 diem
do_thi_duong = {0: [1], 1: [0, 2], 2: [1, 3], 3: [2]}
vec_duong = [[10, 0], [5, 5], [1, 9], [0, 10]]
ket_qua_duong, duong_di_duong = tim_tham_lam(do_thi_duong, vec_duong, [0, 10], 0)
assert ket_qua_duong == 3, f"tu diem 0, di theo duong thang phai toi diem 3 (gan [0,10] nhat) -- dang ra {ket_qua_duong}"
assert duong_di_duong == [0, 1, 2, 3], f"duong di phai di qua het 4 diem -- dang ra {duong_di_duong}"

# bien: diem bat dau DA LA dap an dung -- 0 buoc nhay, khong duoc nhay lung tung
ket_qua_tai_cho, duong_di_tai_cho = tim_tham_lam(do_thi, VEC, vector_cau_hoi, 5)
assert duong_di_tai_cho == [5], f"bat dau tai diem da toi uu (5) thi khong duoc nhay di dau -- dang ra {duong_di_tai_cho}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cả hai đều nằm trong thân vòng lặp `while True:`. Chỗ đầu là TÊN HÀM được gọi để kiểm tra "có láng giềng nào gần hơn không" — chính là hàm đã viết ở bài trước. Chỗ hai là TÊN PHƯƠNG THỨC thêm điểm mới (`hien_tai` sau khi đã cập nhật) vào cuối danh sách `duong_di`.
- kind: strategy
  body: 'Chỗ đầu: `co_lang_gieng_gan_hon` — gọi với đủ bốn đối số `(do_thi, vectors, vector_muc_tieu, hien_tai)`. Chỗ hai: `append` — `duong_di.append(hien_tai)`.'
- kind: one-line
  body: 'Chỗ đầu là `co_lang_gieng_gan_hon`, chỗ hai là `append`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT co_lang_gieng_gan_hon (khong duoc goi thang mot ham khac hay bo qua buoc kiem tra); VA cho trong hai phai GOI phuong thuc append de them diem moi vao duong_di
  requireAst:
  - kind: uses-call, target: co_lang_gieng_gan_hon, min: 1
  - kind: uses-call, target: append, min: 3
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [1, 3] cho hai
  # luat theo dung thu tu khai bao o tren.
  # co_lang_gieng_gan_hon=1: CHI mot lan GOI THAT trong toan bo solution,
  # dung o cho trong dau. Ham nay KHONG tu goi lai chinh no trong dinh nghia.
  # append=3 (TONG THAT, da xac nhan bang cong cu, khong doan tay): HAI lan
  # CO SAN trong xay_do_thi_mot_tang ("do_thi[i].append(j)",
  # "do_thi[j].append(i)"), MOT lan CHINH la cho trong hai. Neu chi dat
  # min=1 (ngay tho), mot mutant BO SOT cho trong hai (vi du dien "pass"
  # thay vi goi append, khong bao gio ghi duong di) van qua duoc vi con lai
  # 2 lan append trong xay_do_thi_mot_tang -- GOTCHA "boilerplate-threshold-
  # masking"; da tu kiem chung: mutant nay lam duong_di CHI CON [2] (khong
  # bao gio them diem moi), bi bat CA boi static (voi min=3, dung) LAN boi
  # assertion "duong_di == [2, 8, 5]".
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua
  # python3 de xac nhan, khong doan tay): dien "append" vao cho trong dau
  # ("ke_tiep = append(do_thi, vectors, vector_muc_tieu, hien_tai)") VA dien
  # "co_lang_gieng_gan_hon" vao cho trong hai
  # ("duong_di.co_lang_gieng_gan_hon(hien_tai)") -- ket qua AST la [1, 3], Y
  # HET ban dung (kiemAst dem CA loi goi ham THUONG lan loi goi PHUONG THUC
  # theo TEN, khong phan biet vi tri). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': da tu chay THAT qua python3, xac nhan
  # no nem NameError: name 'append' is not defined (Python khong co ham
  # toan cuc ten 'append', chi co PHUONG THUC list.append) ngay khi
  # tim_tham_lam(...) duoc goi lan dau -- bi chan boi tier 'run', doc lap
  # voi static.
  # Da tu ra soat them GOTCHA #6 cho ca hai rule uses-call: bon doi so cua
  # loi goi o cho trong dau (do_thi, vectors, vector_muc_tieu, hien_tai) DA
  # duoc cho san trong khung starter (khong nam trong phan blank), nen
  # khong co rui ro "goi dung ham nhung truyen sai bien" o day -- nguoi hoc
  # chi dien TEN HAM/TEN PHUONG THUC, khong dien doi so.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[2, 8, 5\\]\\n5\\n2\\n9\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`2` bước nhảy, `9` phép so sánh — ít hơn `N=20` rất nhiều. Nhưng đường đi
này bắt đầu từ một điểm "may mắn". Bài sau đo trường hợp KHÔNG may mắn.
::::

::::reflect{#nghi-lai}
`tim_tham_lam` không có gì bí ẩn — nó chỉ lặp lại đúng MỘT bước
(`co_lang_gieng_gan_hon`, bài trước) tới khi bước đó không còn tìm ra láng
giềng nào gần hơn. Điểm dừng lại là một CỰC TRỊ CỤC BỘ theo đồ thị — dừng
lại vì KHÔNG CÒN láng giềng trực tiếp nào gần hơn, không phải vì đã "nhìn
thấy" toàn bộ kho như vét cạn. Ở ví dụ này, cực trị cục bộ đó TRÙNG với đáp
án đúng toàn cục (điểm `5`) — nhưng đó không phải một điều được đảm bảo. Bài
sau dựng một trường hợp CỤ THỂ, đo bằng số, nơi hai điều đó KHÁC nhau — và
đó chính là giới hạn cốt lõi của một đồ thị MỘT TẦNG.
::::

::::checkpoint{mastery=0.85}
::::
