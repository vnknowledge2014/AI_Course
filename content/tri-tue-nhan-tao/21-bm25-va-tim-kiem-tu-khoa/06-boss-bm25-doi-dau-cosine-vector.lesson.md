---
id: tri-tue-nhan-tao.bm25-va-tim-kiem-tu-khoa.boss-bm25-doi-dau-cosine-vector
title: "BOSS — BM25 đối đầu cosine vector: đóng q8.5d tại 6/6"
summary: "Kho 7 doan, cau hoi chua ma san pham hiem 'sp4471' (chi xuat hien o DUNG 1 doan, doan chi so 0). BM25 (tren TU THAT, tu vung mo) xep doan 0 o HANG 1 (diem 8,7117, cao nhat) vi no khop CHINH XAC nhieu tu cua cau hoi, ke ca 'sp4471' hiem (idf=1,9459, df=1/7). Cosine (tren TU_VUNG CO DINH 16 muc chu de cong nghe/am thuc, KHONG chua 'sp4471') cho doan 0 vector TOAN SO 0 -- tuong dong cosine DUNG BANG 0,0, xep doan 0 o HANG 2, DUNG SAU doan 1 (mot doan cong nghe chung chung, cosine=0,7071, KHONG lien quan gi toi san pham sp4471) chi vi doan 1 tinh co dung 'may tinh'/'phan mem' -- hai tu CO trong TU_VUNG. BM25 dung, cosine sai: dong q8.5d tai 6/6, dong luc cho q8.5e 'hybrid' ket hop ca hai."
locale: vi
track: tri-tue-nhan-tao
module: bm25-va-tim-kiem-tu-khoa
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-bm25-doi-dau-cosine-vector]
requires: [ai.xep-hang-bm25-tren-kho-tai-lieu]
concepts: [ai.boss-bm25-doi-dau-cosine-vector]
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
Năm bài: TF thô, IDF, TF-IDF, công thức BM25 đầy đủ, xếp hạng cả kho. Bài
này đối đầu trực tiếp BM25 với cosine similarity (q8.5a-c) trên CÙNG một
câu hỏi — và đóng quest `bm25-va-tim-kiem-tu-khoa` (q8.5d) tại `6/6`.
::::

::::explain{#bm25_vs_cosine_diem_mu_khac_nhau}
Toàn bộ quest `chunking-va-embedding-tu-che` (q8.5a) và `tim-kiem-vector-va-
do-tuong-dong`/`hnsw-tu-cai` (q8.5b/c) xây một "embedding" tự chế theo CÁCH
sau: chọn trước một **từ vựng cố định** `TU_VUNG` — một danh sách hữu hạn
các từ/cụm từ liên quan tới chủ đề tài liệu (`16` mục, nửa công nghệ nửa ẩm
thực) — rồi biến mỗi đoạn văn thành một vector đếm từ TRÊN ĐÚNG từ vựng đó.
Đây là điểm mù thứ nhất của cách làm này, chưa từng được nói thẳng: **nếu
một từ KHÔNG nằm trong `TU_VUNG`, vector hoàn toàn không "thấy" được nó —
không phải thấy mờ, mà thấy = KHÔNG GÌ CẢ**, vì `TU_VUNG` không có chỉ số
nào dành cho từ đó.

BM25 (năm bài vừa xây) hoạt động hoàn toàn khác: nó không có một từ vựng cố
định nào cả. Nó tính `tf`/`idf`/điểm số trên NGUYÊN VĂN các từ THẬT xuất
hiện trong câu hỏi và trong kho — bất kể từ đó là gì, kể cả một mã sản phẩm
hiếm gặp hay một tên riêng chưa từng thấy trước đó.

Bài này dựng một tình huống cụ thể để đo đúng sự khác biệt này bằng số: một
câu hỏi chứa một mã sản phẩm HIẾM — `"sp4471"` — chỉ xuất hiện ở ĐÚNG một
đoạn trong cả kho. BM25 khớp CHÍNH XÁC từ đó (từ càng hiếm, `idf` càng cao,
theo đúng bài `2`), nên xếp đúng đoạn đó ở hạng `1`. Cosine, dùng `TU_VUNG`
cố định KHÔNG chứa `"sp4471"`, hoàn toàn mù trước từ đó — và xếp SAI.
::::

::::example{#doi_dau_bm25_cosine}
```python title=readonly
import math

# ---- may do "vector tu che" (nguyen van tu q8.5a/b) ----
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


# ---- may do BM25 (nguyen van tu bai 1-5) ----
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


KHO_TAI_LIEU = [
    "san pham sp4471 la mot thiet bi moi duoc nhieu nguoi dung danh gia cao ve do ben",
    "may tinh hien dai chay phan mem nhanh giup cong viec hieu qua hon truoc",
    "lap trinh vien dung thuat toan de xu ly du lieu lon moi ngay",
    "mon an ngon phu thuoc vao gia vi va cong thuc nau chuan",
    "dau bep gioi luon can than khi chon nguyen lieu tuoi ngon",
    "nha bep sach se giup mon an ngon hon va an toan hon",
    "mon trang mieng ngot ngao thuong duoc dung sau bua an chinh",
]
CAU_HOI = "toi muon hoi ve san pham sp4471 nay co dung duoc voi may tinh khong"

hang_bm25 = xep_hang_bm25(CAU_HOI, KHO_TAI_LIEU, 1.5, 0.75)
hang_cosine = xep_hang_cosine(CAU_HOI, KHO_TAI_LIEU, TU_VUNG, CUM_TU_GOC)

print(hang_bm25)
print(hang_cosine)
print(hang_cua(0, hang_bm25))
print(hang_cua(0, hang_cosine))
```

```text title=readonly
[0, 1, 6, 2, 3, 4, 5]
[1, 0, 2, 3, 4, 5, 6]
1
2
```

Đoạn chỉ số `0` — đoạn ĐÚNG, duy nhất nói về sản phẩm `sp4471` — được BM25
xếp ở **hạng `1`**: từ `"sp4471"` chỉ xuất hiện ở đoạn này (`df=1/7`), cho
`idf ≈ 1,9459`, và đoạn này còn khớp CHÍNH XÁC thêm nhiều từ khác của câu
hỏi (`"san"`, `"pham"`, `"ve"`, `"dung"`, `"duoc"`) — BM25 nhìn thấy TẤT CẢ
những từ thật đó, vì nó không giới hạn trong một từ vựng định sẵn.

Cosine xếp đoạn `0` ở **hạng `2`** — KHÔNG PHẢI vì nó tính sai, mà vì
`TU_VUNG` (`16` mục cố định) không chứa `"sp4471"`, cũng không chứa bất kỳ
từ nào khác mà đoạn `0` dùng (`"thiet_bi"`, `"san_pham"`, `"danh_gia"`,
`"nguoi_dung"` — không mục nào có trong `TU_VUNG`). Vector của đoạn `0` là
`[0, 0, ..., 0]` — TOÀN SỐ `0` — nên `tuong_dong_cosine` trả về ĐÚNG `0,0`
(theo đúng quy ước xử lý vector-không đã học ở q8.5b). Trong khi đó, đoạn
chỉ số `1` (một đoạn công nghệ CHUNG CHUNG, không liên quan gì tới sản phẩm
`sp4471`) tình cờ dùng `"may tinh"` và `"phan mem"` — CẢ HAI đều có trong
`TU_VUNG` — cho cosine `≈ 0,7071`, CAO HƠN đoạn `0`. Cosine không "thấy" một
chút manh mối nào về đoạn `0` cả — nó bị BUỘC về đúng `0,0`, y hệt như MỌI
đoạn hoàn toàn không liên quan khác trong kho (đoạn `3` đến `6`). Đoạn `0`
chỉ đứng trên chúng nhờ THỨ TỰ xuất hiện ban đầu trong danh sách, không phải
vì cosine thấy được bất kỳ liên quan nào.
::::

::::predict{#doan_bm25_dung_cosine_sai commitOnce}
Xét đúng ví dụ trên: đoạn chỉ số `0` (chứa `"sp4471"`, đúng đoạn cần tìm)
được BM25 xếp hạng `1`. `TU_VUNG` (từ vựng cố định của cosine) không chứa
`"sp4471"` và cũng không chứa từ nào khác mà đoạn `0` dùng.

**Trước khi chạy thử**, bạn đoán: cosine có xếp đoạn `0` ở hạng `1`, giống
BM25, không?

:::opt{correct}
Không — vector của đoạn `0` là TOÀN SỐ `0` (không mục nào của `TU_VUNG` xuất
hiện trong đoạn), nên `tuong_dong_cosine` trả về đúng `0,0`; đoạn `1` (không
liên quan tới sản phẩm `sp4471`) tình cờ dùng `"may tinh"`/`"phan mem"` —
hai mục CÓ trong `TU_VUNG` — nên có cosine `≈0,7071` CAO HƠN, xếp trên đoạn
`0`
:::

:::opt
Có — vì đoạn `0` vẫn là đoạn liên quan nhất về mặt NỘI DUNG, và cosine
similarity đo mức độ liên quan về nội dung nên phải xếp đúng đoạn đó ở hạng
đầu, giống BM25
::why
Gần đúng ở việc đoạn `0` THẬT SỰ là đoạn liên quan nhất về nội dung — quan
sát đó đúng, và đúng là mục tiêu LÝ TƯỞNG của mọi phép đo tương đồng.

Chỗ lệch: `tuong_dong_cosine` không đo "nội dung" một cách trực tiếp — nó
chỉ đo hai VECTOR ĐẾM TỪ đã được rút gọn qua đúng `16` mục của `TU_VUNG`.
Nếu nội dung thật sự của một đoạn nằm HOÀN TOÀN ngoài `16` mục đó, vector
của nó không còn giữ lại bất kỳ dấu vết nào của nội dung ấy — cosine không
"thấy mờ", nó thấy KHÔNG GÌ CẢ.
::
:::

:::opt
Có, nhưng chỉ vì đoạn `0` đứng đầu danh sách gốc — cosine similarity của các
đoạn hoàn toàn không liên quan luôn bằng nhau (`0,0`), và `sorted` giữ
nguyên thứ tự xuất hiện khi điểm bằng nhau
::why
Gần đúng ở việc `sorted` ổn định THẬT SỰ giữ nguyên thứ tự gốc khi các điểm
số BẰNG NHAU — quan sát về tính ổn định đó đúng, và đúng là lý do đoạn `0`
đứng TRÊN các đoạn `3` đến `6` (cũng `0,0`).

Chỗ lệch: đoạn `0` không đứng hạng `1` — nó đứng hạng `2`, SAU đoạn `1`
(cosine `≈0,7071`, một giá trị THỰC SỰ khác `0,0`, không phải hoà). Việc
`sorted` ổn định chỉ giải thích tại sao đoạn `0` đứng trên các đoạn CÙNG
điểm `0,0` khác — nó không giải thích được vì sao đoạn `0` lại đứng SAU đoạn
`1`.
::
:::
::::

::::code{#viet_diem_bm25_doan_va_hang_cua}
Hoàn thiện `diem_bm25_doan` (cộng dồn điểm BM25 của từng từ khoá vào biến
tích luỹ) và `hang_cua` (tìm hạng của một đoạn cụ thể trong một danh sách đã
xếp hạng).

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
        tong += ___                                    # diem_bm25_mot_tu(tf_t, tinh_idf(t, kho), do_dai_doan, avgdl, k1, b)
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
    return ___.index(chi_so_muc_tieu) + 1           # thu_tu_xep_hang


KHO_TAI_LIEU = [
    "san pham sp4471 la mot thiet bi moi duoc nhieu nguoi dung danh gia cao ve do ben",
    "may tinh hien dai chay phan mem nhanh giup cong viec hieu qua hon truoc",
    "lap trinh vien dung thuat toan de xu ly du lieu lon moi ngay",
    "mon an ngon phu thuoc vao gia vi va cong thuc nau chuan",
    "dau bep gioi luon can than khi chon nguyen lieu tuoi ngon",
    "nha bep sach se giup mon an ngon hon va an toan hon",
    "mon trang mieng ngot ngao thuong duoc dung sau bua an chinh",
]
CAU_HOI = "toi muon hoi ve san pham sp4471 nay co dung duoc voi may tinh khong"

hang_bm25 = xep_hang_bm25(CAU_HOI, KHO_TAI_LIEU, 1.5, 0.75)
hang_cosine = xep_hang_cosine(CAU_HOI, KHO_TAI_LIEU, TU_VUNG, CUM_TU_GOC)

print(hang_bm25)
print(hang_cosine)
print(hang_cua(0, hang_bm25))
print(hang_cua(0, hang_cosine))
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


KHO_TAI_LIEU = [
    "san pham sp4471 la mot thiet bi moi duoc nhieu nguoi dung danh gia cao ve do ben",
    "may tinh hien dai chay phan mem nhanh giup cong viec hieu qua hon truoc",
    "lap trinh vien dung thuat toan de xu ly du lieu lon moi ngay",
    "mon an ngon phu thuoc vao gia vi va cong thuc nau chuan",
    "dau bep gioi luon can than khi chon nguyen lieu tuoi ngon",
    "nha bep sach se giup mon an ngon hon va an toan hon",
    "mon trang mieng ngot ngao thuong duoc dung sau bua an chinh",
]
CAU_HOI = "toi muon hoi ve san pham sp4471 nay co dung duoc voi may tinh khong"

hang_bm25 = xep_hang_bm25(CAU_HOI, KHO_TAI_LIEU, 1.5, 0.75)
hang_cosine = xep_hang_cosine(CAU_HOI, KHO_TAI_LIEU, TU_VUNG, CUM_TU_GOC)

print(hang_bm25)
print(hang_cosine)
print(hang_cua(0, hang_bm25))
print(hang_cua(0, hang_cosine))
```

```python title=test
assert hang_bm25 == [0, 1, 6, 2, 3, 4, 5], f"xep hang BM25 sai -- dang ra {hang_bm25}"
assert hang_cosine == [1, 0, 2, 3, 4, 5, 6], f"xep hang cosine sai -- dang ra {hang_cosine}"
assert hang_cua(0, hang_bm25) == 1, f"BM25 phai xep doan 0 (chua 'sp4471') o HANG 1 -- dang ra {hang_cua(0, hang_bm25)}"
assert hang_cua(0, hang_cosine) == 2, f"cosine phai xep doan 0 o HANG 2 (khong phai hang 1) -- dang ra {hang_cua(0, hang_cosine)}"
assert hang_cua(0, hang_bm25) != hang_cua(0, hang_cosine), "BM25 va cosine phai xep doan 0 o HAI HANG KHAC NHAU -- day chinh la diem mu can do"

v0 = tinh_vector_dem_tu(KHO_TAI_LIEU[0], TU_VUNG, CUM_TU_GOC)
assert v0 == [0] * 16, f"vector cua doan 0 phai la TOAN SO 0 (TU_VUNG khong chua tu nao doan nay dung) -- dang ra {v0}"
assert tuong_dong_cosine(tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC), v0) == 0.0, "cosine giua cau hoi va doan 0 phai dung bang 0,0"

diem_bm25_doan0 = diem_bm25_doan(KHO_TAI_LIEU[0], CAU_HOI.lower().split(), KHO_TAI_LIEU, 1.5, 0.75)
assert round(diem_bm25_doan0, 4) == 8.7117, f"diem BM25 cua doan 0 phai la 8,7117 -- dang ra {diem_bm25_doan0}"

# kiem tra truc tiep hang_cua tren mot vi du nho, tu tinh tay duoc
assert hang_cua(5, [3, 5, 1]) == 2, f"5 dung o vi tri chi so 1 (0-based), hang = 1+1 = 2 -- dang ra {hang_cua(5, [3, 5, 1])}"
assert hang_cua(3, [3, 5, 1]) == 1, f"3 dung o vi tri chi so 0, hang = 0+1 = 1 -- dang ra {hang_cua(3, [3, 5, 1])}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `diem_bm25_doan`) là giá trị được CỘNG THÊM vào biến tích luỹ `tong` — điểm BM25 của TỪ ĐANG XÉT (`t`), tính bằng lời gọi `diem_bm25_mot_tu(...)` với đúng sáu đối số: tần số của từ trong đoạn (`tf_t`), IDF của từ đó trên kho (`tinh_idf(t, kho)`), độ dài đoạn, độ dài trung bình, `k1`, `b`. Chỗ hai (trong `hang_cua`) là danh sách được tìm chỉ số trong đó — chính là tham số `thu_tu_xep_hang` (một danh sách chỉ số đã xếp hạng).
- kind: strategy
  body: 'Chỗ đầu: `diem_bm25_mot_tu(tf_t, tinh_idf(t, kho), do_dai_doan, avgdl, k1, b)` — gọi đúng hàm điểm-một-từ đã xây ở bài `4`. Chỗ hai: `thu_tu_xep_hang` — tham số danh sách đã xếp hạng, gọi `.index(...)` trên nó.'
- kind: one-line
  body: 'Chỗ đầu là `diem_bm25_mot_tu(tf_t, tinh_idf(t, kho), do_dai_doan, avgdl, k1, b)`, chỗ hai là `thu_tu_xep_hang`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT diem_bm25_mot_tu(...) de cong don diem cua tu dang xet vao 'tong' (khong duoc bo qua hay dung cong thuc khac); VA cho trong hai phai DOC DUNG tham so 'thu_tu_xep_hang' (khong duoc dung mot ten khac)
  requireAst:
  - kind: uses-call, target: diem_bm25_mot_tu, min: 1
  - kind: uses-name, target: thu_tu_xep_hang, min: 1
  # Da thu that (trich nguyen ham _dem cua kiem-ast.ts vao mot file rieng,
  # chay qua python3 TREN CHINH van ban solution da trich tu file nay) -- ket
  # qua [1, 1] cho hai luat theo dung thu tu khai bao o tren.
  # diem_bm25_mot_tu=1: CHI mot lan GOI THAT trong toan bo solution, dung o
  # cho trong dau (ben trong diem_bm25_doan). Ham nay khong tu goi lai chinh
  # no trong dinh nghia cua no.
  # thu_tu_xep_hang=1: CHI mot lan DOC (Load), dung o cho trong hai (lam doi
  # tuong cho .index(...)). Day la ten THAM SO CUA CHINH ham hang_cua, khong
  # xuat hien o bat ky ham nao khac trong solution.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua ham
  # _dem de xac nhan, khong doan tay): dien "thu_tu_xep_hang" vao cho trong
  # dau ("tong += thu_tu_xep_hang" trong diem_bm25_doan) VA dien
  # "diem_bm25_mot_tu(tf_t, tinh_idf(t, kho), do_dai_doan, avgdl, k1, b)" vao
  # cho trong hai ("return diem_bm25_mot_tu(tf_t, tinh_idf(t, kho),
  # do_dai_doan, avgdl, k1, b).index(chi_so_muc_tieu) + 1" trong hang_cua) --
  # ket qua AST van la [1, 1], Y HET ban dung (moi luat van dem dung 1 lan,
  # chi doi VI TRI). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong diem_bm25_doan, ten
  # 'thu_tu_xep_hang' KHONG TON TAI (day la tham so CUA HAM hang_cua, mot ham
  # khac hoan toan, khong lien quan) -- da tu chay THAT mutant nay qua
  # python3, xac nhan no nem NameError: name 'thu_tu_xep_hang' is not defined
  # ngay khi diem_bm25_doan() duoc goi lan dau (tu ben trong xep_hang_bm25)
  # -- bi chan boi tier 'run', doc lap voi static.
  # Da ra soat GOTCHA #6 cho luat uses-call (diem_bm25_mot_tu): mot cheat co
  # the hoan doi THU TU sau doi so trong loi goi (vi du dua do_dai_doan lam
  # tf_t) ma van dem la 1 lan goi hop le -- da tu kiem chung: cac assertion
  # rieng "diem_bm25_doan0 == 8.7117" va "hang_bm25 == [0, 1, 6, 2, 3, 4, 5]"
  # deu tinh TREN DUNG SO nen bat duoc ngay bat ky hoan doi doi so nao lam
  # sai gia tri so, doc lap hoan toan voi luat static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[0, 1, 6, 2, 3, 4, 5\\]\\n\\[1, 0, 2, 3, 4, 5, 6\\]\\n1\\n2\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
BM25 xếp đoạn đúng ở hạng `1`, cosine xếp NÓ ở hạng `2` — cùng dữ liệu, cùng
câu hỏi, khác nhau vì một cách nhìn thấy từng TỪ THẬT còn cách kia bị giới
hạn trong một từ vựng cố định. Quest `bm25-va-tim-kiem-tu-khoa` (q8.5d) đóng
tại `6/6`.
::::

::::reflect{#nghi-lai}
Sáu bài đã trả lời từng mảnh một câu hỏi mở đầu: TF thô (bài `1`) đo được
lỗ hổng thiên vị độ dài; IDF (bài `2`) đo được độ hiếm của một từ; TF-IDF
(bài `3`) ghép cả hai và cho thấy từ hiếm quan trọng hơn; công thức BM25
(bài `4`) thêm bão hoà và chuẩn hoá độ dài; xếp hạng BM25 trên cả kho (bài
`5`) cho thấy nó sửa đúng lỗi mà TF-IDF không sửa được. Bài BOSS này đặt
BM25 cạnh cosine similarity (q8.5a-c) và đo bằng số: **không cách nào "tốt
hơn" tuyệt đối cả hai đều có điểm mù RIÊNG**. Cosine mù trước một từ KHÔNG
nằm trong từ vựng cố định của nó — dù từ đó quan trọng tới đâu. BM25 (như
mọi phép khớp từ khoá thuần tuý) mù trước một CÁCH DIỄN ĐẠT khác đi cùng
một ý — nếu câu hỏi dùng từ đồng nghĩa mà đoạn văn đúng không hề nhắc tới
nguyên văn, BM25 sẽ không khớp được, dù cosine (nếu từ đồng nghĩa đó tình cờ
nằm trong từ vựng) có thể vẫn thấy được sự liên quan về CHỦ ĐỀ. Đây chính là
động lực cho quest tiếp theo, `hybrid-va-rerank` (q8.5e): kết hợp CẢ HAI
cách tìm — vector VÀ từ khoá — qua một công thức hợp nhất thứ hạng, để không
cách nào phải một mình gánh hết điểm mù của mình.
::::

::::checkpoint{mastery=0.9}
::::
