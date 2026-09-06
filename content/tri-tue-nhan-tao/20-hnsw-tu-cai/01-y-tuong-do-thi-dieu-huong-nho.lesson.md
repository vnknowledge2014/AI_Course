---
id: tri-tue-nhan-tao.hnsw-tu-cai.y-tuong-do-thi-dieu-huong-nho
title: "Ý tưởng đồ thị điều hướng nhỏ: nhảy vài bước thay vì so hết"
summary: "Tren mot do thi 20 diem (10 tech + 10 am thuc, moi diem noi toi M=3 lang gieng gan nhat) da DUNG SAN, tim tham lam tu diem 2 toi cau hoi cong nghe chi can DUNG 2 buoc nhay (duong di [2, 8, 5]) de toi diem 5 -- dung DAP AN vet can (q8.5b). Vet can can N=20 phep so sanh cho CUNG cau hoi do. 2 buoc nhay << 20 phep so sanh -- do bang so that, khong suy doan."
locale: vi
track: tri-tue-nhan-tao
module: hnsw-tu-cai
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.y-tuong-do-thi-dieu-huong-nho]
requires: [ai.boss-tim-dung-k-doan-lien-quan]
concepts: [ai.y-tuong-do-thi-dieu-huong-nho]
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
Track `T8.5` "RAG từ số 0" đã đi qua chunking, embedding tự chế, cosine
similarity, và vét cạn. Quest cuối cùng — `hnsw-tu-cai` (q8.5c) — xây một
chỉ mục thật, để tìm kiếm KHÔNG cần so sánh với mọi điểm trong kho.
::::

::::explain{#y_tuong_do_thi_dieu_huong_nho}
Bài `do-toc-do-va-lam-chuan-doi-chieu` (q8.5b, bài `4`) đã đo: vét cạn LUÔN
cần đúng `N` phép so sánh cosine similarity cho một kho `N` phần tử, bất kể
`k` là bao nhiêu. Với kho vài chục nghìn đoạn văn, tính `N` lần similarity
cho MỖI câu hỏi là quá chậm để dùng thực tế.

**HNSW** (Hierarchical Navigable Small World — "đồ thị điều hướng nhỏ nhiều
tầng") là một cách tổ chức kho dữ liệu để tìm kiếm KHÔNG cần so sánh với mọi
điểm. Ý tưởng gốc, ở dạng đơn giản nhất — một **đồ thị điều hướng nhỏ**
(navigable small world) MỘT tầng:

> Mỗi điểm dữ liệu là một đỉnh của đồ thị, nối tới một SỐ ÍT "láng giềng
> gần" — KHÔNG nối tới mọi điểm khác. Tìm kiếm bằng cách đi **tham lam**
> (greedy): từ một điểm hiện tại, xét TẤT CẢ láng giềng của nó; nếu có láng
> giềng nào gần mục tiêu HƠN điểm hiện tại, nhảy sang láng giềng gần nhất
> trong số đó; dừng lại khi không còn láng giềng nào gần hơn.

Vì mỗi bước nhảy chỉ cần xét vài láng giềng (không phải toàn bộ `N` điểm),
và số bước nhảy để đi từ một điểm bất kỳ tới gần mục tiêu thường RẤT NHỎ so
với `N` — đó là lý do đồ thị này được gọi là "nhỏ" (small world): mọi điểm
đều "gần" nhau qua một vài bước nhảy, dù đồ thị thưa (mỗi đỉnh chỉ nối tới
vài láng giềng).

Bài này CHƯA dạy cách XÂY đồ thị đó (bài sau) — chỉ đo TRỰC TIẾP: trên một
đồ thị nhỏ ĐÃ DỰNG SẴN, tìm tham lam cần bao nhiêu bước nhảy để tới gần một
câu hỏi cụ thể, so với `N` phép so sánh mà vét cạn luôn cần.
::::

::::example{#do_buoc_nhay_bang_so}
Một kho `20` điểm (`10` điểm chủ đề công nghệ, `10` điểm chủ đề ẩm thực),
mỗi điểm là một cụm từ ngắn, biến thành vector đếm từ (q8.5a/q8.5b). Đồ thị
đã dựng sẵn: mỗi điểm nối tới `M=3` láng giềng gần nhất (hàm dựng đồ thị này
sẽ được HỌC ở bài sau — ở đây chỉ DÙNG nó như một hộp đen đã hoạt động
đúng):

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


# Đã dựng sẵn cho bạn -- nối mỗi điểm mới tới M láng giềng gần nhất hiện có.
# Sẽ học cách TỰ VIẾT hàm này ở bài sau (xay-do-thi-mot-tang-don-gian).
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

diem_bat_dau = 2
duong_di = [diem_bat_dau]
hien_tai = diem_bat_dau
while True:
    ke_tiep = co_lang_gieng_gan_hon(do_thi, VEC, vector_cau_hoi, hien_tai)
    if ke_tiep is None:
        break
    hien_tai = ke_tiep
    duong_di.append(hien_tai)

so_buoc_nhay = len(duong_di) - 1

print(duong_di)
print(so_buoc_nhay)
print(N)
print(so_buoc_nhay < N)
```

```text title=readonly
[2, 8, 5]
2
20
True
```

Bắt đầu ở điểm `2`, tìm tham lam chỉ cần `2` bước nhảy (`2 → 8 → 5`) để tới
điểm `5` — và điểm `5` chính xác là điểm gần `CAU_HOI` nhất trong TOÀN BỘ
`20` điểm (đối chiếu được với vét cạn của q8.5b, dù bài này chưa gọi lại
hàm đó). Vét cạn cần đúng `N=20` phép so sánh để tìm ra cùng câu trả lời.
`2` bước nhảy so với `20` phép so sánh — một khác biệt đo được bằng số thật,
không phải trực giác.
::::

::::predict{#doan_so_buoc_nhay_it_hon_N commitOnce}
Xét đúng ví dụ trên: đồ thị `20` điểm, mỗi điểm nối tới `M=3` láng giềng gần
nhất. Tìm tham lam từ điểm `2` cần `2` bước nhảy để tới điểm `5` (đáp án
đúng).

**Trước khi chạy thử**, bạn đoán: số bước nhảy (`2`) so với số phép so sánh
mà vét cạn cần (`N=20`) — cái nào ÍT HƠN?

:::opt{correct}
Số bước nhảy (`2`) ít hơn HẲN — tìm tham lam chỉ xét láng giềng của TỪNG
điểm đi qua (tối đa `3` láng giềng mỗi điểm, không phải cả `20` điểm), nên
số bước cần thiết có thể nhỏ hơn `N` rất nhiều
:::

:::opt
Bằng nhau — vì tìm tham lam vẫn phải "chạm" tới mọi điểm trong đồ thị trước
khi kết luận được điểm nào gần nhất, giống như vét cạn
::why
Gần đúng ở việc tìm tham lam CŨNG cần một quy trình có hệ thống để tới được
đáp án đúng — không phải may rủi.

Chỗ lệch: tìm tham lam KHÔNG chạm tới mọi điểm — nó chỉ đi qua NHỮNG điểm
nằm trên đường đi (`2`, `8`, `5` — đúng `3` điểm, không phải `20`), và tại
mỗi điểm chỉ xét láng giềng TRỰC TIẾP của nó (tối đa `3`), không xét toàn bộ
đồ thị. Đó chính là lý do số bước nhảy nhỏ hơn `N` — nó không cần "nhìn
thấy" mọi điểm để tìm ra đáp án.
::
:::

:::opt
Số phép so sánh của vét cạn (`20`) ít hơn — vì vét cạn chỉ cần MỘT vòng lặp
đơn giản, còn tìm tham lam phải lặp qua nhiều bước nhảy, mỗi bước lại xét
nhiều láng giềng nên tổng số phép tính có thể nhiều hơn
::why
Gần đúng ở việc vét cạn CÓ một cấu trúc lặp đơn giản, dễ hình dung — quan
sát đó đúng.

Chỗ lệch: "đơn giản" không có nghĩa là "ít phép tính". Vét cạn ĐƠN GIẢN
nhưng vẫn cần tính cosine similarity với CẢ `20` điểm — không có cách nào
tránh được việc đó. Tìm tham lam phức tạp hơn về LOGIC (có bước nhảy, có
điều kiện dừng) nhưng chỉ chạm tới `3` điểm (`2`, `8`, `5`), mỗi điểm xét
tối đa `3` láng giềng — tổng cộng ít hơn `20` phép so sánh rất nhiều.
::
:::
::::

::::code{#viet_co_lang_gieng_gan_hon}
Hoàn thiện `co_lang_gieng_gan_hon`: xét TẤT CẢ láng giềng của điểm hiện tại,
tìm láng giềng GẦN mục tiêu NHẤT, và chỉ nhảy sang nó nếu nó thật sự gần mục
tiêu HƠN điểm hiện tại (nếu không có láng giềng nào gần hơn, trả về
`None`).

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
        if tuong_dong_cosine(vectors[hang_xom], vector_muc_tieu) ___ tuong_dong_cosine(vectors[tot_nhat], vector_muc_tieu):  # >
            tot_nhat = ___                                                                                                  # hang_xom
    if tot_nhat == nut_hien_tai:
        return None
    return tot_nhat


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

diem_bat_dau = 2
duong_di = [diem_bat_dau]
hien_tai = diem_bat_dau
while True:
    ke_tiep = co_lang_gieng_gan_hon(do_thi, VEC, vector_cau_hoi, hien_tai)
    if ke_tiep is None:
        break
    hien_tai = ke_tiep
    duong_di.append(hien_tai)

so_buoc_nhay = len(duong_di) - 1

print(duong_di)
print(so_buoc_nhay)
print(N)
print(so_buoc_nhay < N)
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

diem_bat_dau = 2
duong_di = [diem_bat_dau]
hien_tai = diem_bat_dau
while True:
    ke_tiep = co_lang_gieng_gan_hon(do_thi, VEC, vector_cau_hoi, hien_tai)
    if ke_tiep is None:
        break
    hien_tai = ke_tiep
    duong_di.append(hien_tai)

so_buoc_nhay = len(duong_di) - 1

print(duong_di)
print(so_buoc_nhay)
print(N)
print(so_buoc_nhay < N)
```

```python title=test
assert duong_di == [2, 8, 5], f"duong di phai la [2, 8, 5] -- dang ra {duong_di}"
assert so_buoc_nhay == 2, f"so buoc nhay phai la 2 -- dang ra {so_buoc_nhay}"
assert N == 20, f"N phai la 20 -- dang ra {N}"
assert so_buoc_nhay < N, "so buoc nhay phai IT HON N -- day chinh la loi ich cua do thi dieu huong nho"

# kiem tra truc tiep co_lang_gieng_gan_hon tren mot do thi do choi nho, tu tinh tay duoc
DO_THI_DO_CHOI = {0: [1, 2], 1: [0], 2: [0]}
VEC_DO_CHOI = [[1, 0], [0, 1], [5, 0]]
assert co_lang_gieng_gan_hon(DO_THI_DO_CHOI, VEC_DO_CHOI, [1, 0], 1) == 0, "tu diem 1, lang gieng 0 (cosine=1.0) gan muc tieu [1,0] hon diem 1 (cosine=0.0)"
assert co_lang_gieng_gan_hon(DO_THI_DO_CHOI, VEC_DO_CHOI, [1, 0], 0) is None, "tu diem 0 (da la tot nhat, cosine=1.0), khong lang gieng nao (0.0 va 1.0-hoa, khong PHAI lon hon) gan hon"

# kiem tra truc tiep tren do thi that: tu diem 8, lang gieng gan hon phai la 5
assert co_lang_gieng_gan_hon(do_thi, VEC, vector_cau_hoi, 8) == 5, "tu diem 8, lang gieng gan hon phai la 5"
assert co_lang_gieng_gan_hon(do_thi, VEC, vector_cau_hoi, 5) is None, "diem 5 da la tot nhat, khong lang gieng nao gan hon"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cùng nằm trong vòng lặp `for hang_xom in do_thi[nut_hien_tai]:`. Chỗ đầu là phép SO SÁNH — láng giềng `hang_xom` có gần mục tiêu HƠN ứng viên tốt nhất hiện tại (`tot_nhat`) hay không. Chỗ hai là việc CẬP NHẬT `tot_nhat` thành láng giềng vừa tìm thấy, để vòng lặp tiếp tục so những láng giềng còn lại với ứng viên MỚI này (không phải với `nut_hien_tai` ban đầu).
- kind: strategy
  body: 'Chỗ đầu: `>` — so sánh nghiêm ngặt (`tuong_dong_cosine(hang_xom) > tuong_dong_cosine(tot_nhat)`). Chỗ hai: `hang_xom` — gán biến vòng lặp vào `tot_nhat`.'
- kind: one-line
  body: 'Chỗ đầu là `>`, chỗ hai là `hang_xom`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai SO SANH NGHIEM NGAT (dung toan tu '>') do tuong dong cua hang_xom voi do tuong dong cua tot_nhat -- khong duoc dao nguoc hay dung '>='; VA cho trong hai phai GAN bien 'hang_xom' (lang gieng vua tim thay) vao 'tot_nhat', khong duoc gan mot ten khac
  requireAst:
  - kind: uses-operator, target: ">", min: 2
  - kind: uses-name, target: hang_xom, min: 2
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [2, 2] cho hai
  # luat theo dung thu tu khai bao o tren.
  # ">"=2 (TONG THAT, da xac nhan bang cong cu): MOT lan CO SAN trong
  # xay_do_thi_mot_tang ("if len(do_thi[j]) > M:", da cho san, khong blank),
  # MOT lan CHINH la cho trong dau. Neu chi dat min=1 (ngay tho), mot mutant
  # dien "tuong_dong_cosine(vectors[tot_nhat], vector_muc_tieu) >
  # tuong_dong_cosine(vectors[hang_xom], vector_muc_tieu)" (dao NGUOC hai ve,
  # sai logic hoan toan) van qua duoc vi con lai 1 lan '>' trong
  # xay_do_thi_mot_tang -- GOTCHA "boilerplate-threshold-masking"; da tu kiem
  # chung: mutant dao nguoc nay lam co_lang_gieng_gan_hon(do_thi, VEC,
  # vector_cau_hoi, 8) tra ve mot gia tri KHAC 5 (hoac None), bi bat CA boi
  # static (voi min=2, dung) LAN boi assertion rieng ve diem 8.
  # "hang_xom"=2 (TONG THAT): MOT lan Load trong dieu kien if
  # ("vectors[hang_xom]"), MOT lan CHINH la cho trong hai ("tot_nhat =
  # hang_xom"). Neu chi dat min=1, mot mutant dien "tot_nhat = nut_hien_tai"
  # (khong bao gio cap nhat, vong lap vo nghia) van qua vi con lai 1 lan
  # "hang_xom" trong dieu kien -- bi chan boi min=2 (dung).
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua
  # python3 de xac nhan, khong doan tay): dien "hang_xom" vao cho trong dau
  # ("if tuong_dong_cosine(...) hang_xom tuong_dong_cosine(...):") VA dien
  # ">" vao cho trong hai ("tot_nhat = >") -- CA HAI deu SAI CU PHAP ngay lap
  # tuc (mot toan tu so sanh khong the la mot ten bien, mot phep gan khong
  # the nhan mot toan tu tran lam gia tri) -- da tu chay qua python3, xac
  # nhan ca hai deu nem SyntaxError ngay o buoc phan tich cu phap, TRUOC CA
  # khi kiemAst() hay 'run' kip chay -- vi cho trong dau la mot VI TRI TOAN
  # TU (giua hai bieu thuc) con cho trong hai la mot VI TRI TEN BIEN (ben
  # phai dau '='), hai vi tri nay KHONG cung kieu cu phap nen khong the hoan
  # doi cho nhau ma van la Python hop le. Rui ro hoan doi o day khong co
  # thuc, da xac nhan bang chay that chu khong doan.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[2, 8, 5\\]\\n2\\n20\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`2` bước nhảy, `20` phép so sánh của vét cạn — một khác biệt đo được, không
suy đoán. Nhưng đồ thị này từ đâu ra? Bài sau: tự xây nó.
::::

::::reflect{#nghi-lai}
Bài này CHƯA dạy cách xây đồ thị — nó đo một hệ quả: MỘT khi đồ thị đã tồn
tại (mỗi điểm nối tới vài láng giềng gần), tìm kiếm bằng cách đi tham lam
qua các cạnh có thể tới gần mục tiêu chỉ sau vài bước nhảy, thay vì so sánh
với TẤT CẢ `N` điểm như vét cạn. Con số cụ thể (`2` bước so với `20` phép so
sánh) không phải một quy luật toán học cố định — nó phụ thuộc vào CẤU TRÚC
của đồ thị (bài sau xây), và ĐIỂM BẮT ĐẦU (bài `gioi-han-mot-tang-va-y-
tuong-nhieu-tang` sẽ đo một trường hợp nơi con số này KHÔNG đẹp như vậy).
Nhưng ý tưởng cốt lõi đã rõ: một đồ thị thưa, đi tham lam, có thể nhanh hơn
vét cạn RẤT NHIỀU — đó là toàn bộ động lực của HNSW.
::::

::::checkpoint{mastery=0.85}
::::
