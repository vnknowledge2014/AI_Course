---
id: tri-tue-nhan-tao.bm25-va-tim-kiem-tu-khoa.xep-hang-bm25-tren-kho-tai-lieu
title: "Xếp hạng BM25 trên một kho tài liệu: đối chiếu với TF-IDF thô"
summary: "Kho 9 doan (8 doan cu + 1 doan MOI, dai 78 tu, lap lai 'thuat'/'toan'/'xu'/'ly' dung 3 lan moi tu). Truy van ['thuat','toan','xu','ly']. TF-IDF THO xep doan MOI (chi so 8) o hang 1 voi diem 13,1833 -- CAO NHAT, vi no lap tu nhieu lan nhat, DU khong khop sat truy van hon doan chi so 3 (chua ca 4 tu, moi tu dung 1 lan, dai chi 14 tu). BM25 (k1=1.5, b=0.75) xep doan chi so 3 o hang 1 voi diem 5,07, doan MOI tut xuong hang 2 voi diem 4,2325 -- BM25 XEP DUNG HON nho bao hoa (4 lan tf=3 khong con manh gap ~3 lan tf=1) VA chuan hoa do dai (doan MOI dai 78 tu, gap gan 4 lan do dai trung binh 19,89, bi phat nang)."
locale: vi
track: tri-tue-nhan-tao
module: bm25-va-tim-kiem-tu-khoa
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.xep-hang-bm25-tren-kho-tai-lieu]
requires: [ai.cong-thuc-bm25-day-du]
concepts: [ai.xep-hang-bm25-tren-kho-tai-lieu]
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
Bài trước đo bão hoà và chuẩn hoá độ dài trên TỪNG con số riêng lẻ. Bài này
ráp lại: dùng công thức BM25 đầy đủ để xếp hạng CẢ MỘT kho tài liệu cho một
truy vấn nhiều từ, rồi đối chiếu trực tiếp với TF-IDF thô — tìm một trường
hợp cụ thể nơi BM25 xếp ĐÚNG HƠN.
::::

::::explain{#xep_hang_bm25_toan_kho}
Điểm BM25 của một đoạn văn `d` cho một truy vấn nhiều từ `q` là TỔNG công
thức bài trước qua mọi từ khoá `t` trong `q`:

```
bm25(d, q) = tong bm25(t, d)   voi moi t trong q, ma tf(t,d) > 0
```

Cách xếp hạng: tính điểm này cho MỌI đoạn trong kho, rồi sắp xếp giảm dần —
giống hệt cách TF-IDF (bài `3`) và vét cạn cosine (q8.5b) đã làm, chỉ khác
công thức tính điểm cho mỗi đoạn.

Thêm MỘT đoạn văn thứ `9` vào kho `8` đoạn của các bài trước: một đoạn DÀI
(`78` từ, gấp gần `4` lần độ dài trung bình của kho), nói chung chung về
"công nghệ", nhưng LẶP LẠI cả bốn từ của một truy vấn — `"thuat"`, `"toan"`,
`"xu"`, `"ly"` — đúng `3` lần MỖI từ. So sánh với đoạn chỉ số `3` đã có sẵn
(`"du lieu duoc luu tru va xu ly boi nhieu thuat toan khac nhau"`, chỉ `14`
từ) — đoạn này chứa CẢ BỐN từ của truy vấn, nhưng mỗi từ chỉ đúng `1` lần.
::::

::::example{#tf_idf_vs_bm25_toan_kho}
```python title=readonly
import math

KHO_TAI_LIEU = [
    "may tinh chay nhanh nho co vi xu ly manh me",
    "phan mem tot giup nguoi dung lam viec hieu qua hon truoc",
    "lap trinh vien can hieu ro thuat toan truoc khi viet phan mem",
    "du lieu duoc luu tru va xu ly boi nhieu thuat toan khac nhau",
    "mon an ngon phu thuoc vao gia vi va cong thuc nau chuan",
    "dau bep gioi luon can than khi chon nguyen lieu tuoi ngon",
    "nha bep sach se va giup mon an ngon hon va gon gang hon",
    "mon trang mieng ngot ngao thuong duoc dung sau bua an chinh",
    (
        "cong nghe hien nay phat trien rat nhanh nhieu nguoi quan tam toi thuat toan moi "
        "thuat toan cu deu duoc nghien cuu dua vao du lieu xu ly qua nhieu buoc xu ly khac nhau "
        "moi buoc xu ly deu quan trong nguoi ta cung quan tam toi cach to chuc va sap xep du lieu "
        "sao cho de tim kiem thuat toan sap xep la mot phan quan trong cua khoa hoc may tinh hien dai"
    ),
]


def dem_tan_so(tu, van_ban):
    cac_tu = van_ban.lower().split()
    return cac_tu.count(tu)


def dem_so_doan_chua_tu(tu, kho):
    return sum(1 for doan in kho if tu in doan.lower().split())


def tinh_idf(tu, kho):
    df = dem_so_doan_chua_tu(tu, kho)
    if df == 0:
        return 0.0
    return math.log(len(kho) / df)


def diem_tf_idf(doan, tu_truy_van, kho):
    return sum(dem_tan_so(t, doan) * tinh_idf(t, kho) for t in tu_truy_van)


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


TU_TRUY_VAN = ["thuat", "toan", "xu", "ly"]
K1, B = 1.5, 0.75

diem_tfidf = [diem_tf_idf(doan, TU_TRUY_VAN, KHO_TAI_LIEU) for doan in KHO_TAI_LIEU]
diem_bm25 = [diem_bm25_doan(doan, TU_TRUY_VAN, KHO_TAI_LIEU, K1, B) for doan in KHO_TAI_LIEU]

xep_hang_tfidf = sorted(range(len(KHO_TAI_LIEU)), key=lambda i: diem_tfidf[i], reverse=True)
xep_hang_bm25 = sorted(range(len(KHO_TAI_LIEU)), key=lambda i: diem_bm25[i], reverse=True)

print([round(d, 4) for d in diem_tfidf])
print([round(d, 4) for d in diem_bm25])
print(xep_hang_tfidf[0])
print(xep_hang_bm25[0])
print(xep_hang_tfidf[0] == xep_hang_bm25[0])
```

```text title=readonly
[2.1972, 0.0, 2.1972, 4.3944, 0.0, 0.0, 0.0, 0.0, 13.1833]
[2.7504, 0.0, 2.6029, 5.07, 0.0, 0.0, 0.0, 0.0, 4.2325]
8
3
False
```

TF-IDF THÔ xếp đoạn chỉ số `8` (đoạn mới, dài, lặp từ) ở hạng `1` với điểm
`13,1833` — CAO GẤP HƠN `3` LẦN đoạn chỉ số `3` (điểm `4,3944`), dù đoạn `3`
mới thực sự khớp sát truy vấn (đủ cả bốn từ, ngắn gọn, không lặp thừa). Lý
do: TF-IDF cộng dồn tuyến tính, và đoạn `8` lặp mỗi từ đúng `3` lần thay vì
`1` lần, nhân điểm số lên gần gấp `3`, không có giới hạn nào cản.

BM25 xếp đoạn chỉ số `3` ở hạng `1` với điểm `5,07`, đoạn chỉ số `8` TỤT
XUỐNG hạng `2` với điểm chỉ `4,2325` — THẤP HƠN đoạn `3`. Hai lý do cộng
hưởng: (a) **bão hoà** — `tf=3` không còn mạnh gấp `3` lần `tf=1` như trong
TF-IDF, mà chỉ mạnh hơn một phần (đúng cơ chế bài trước đã đo); (b) **chuẩn
hoá độ dài** — đoạn `8` dài `78` từ, gấp gần `4` lần độ dài trung bình của
kho (`≈19,89`), bị hệ số chuẩn hoá phạt nặng. Kết quả: `xep_hang_tfidf[0] =
8` nhưng `xep_hang_bm25[0] = 3` — BM25 xếp hạng KHÁC, và xếp ĐÚNG hơn, đo
được bằng số cụ thể.
::::

::::predict{#doan_bm25_xep_dung_hon commitOnce}
Xét đúng ví dụ trên: đoạn chỉ số `3` (`14` từ, đủ cả `4` từ khoá, mỗi từ
đúng `1` lần) và đoạn chỉ số `8` (`78` từ, cũng đủ cả `4` từ khoá, nhưng mỗi
từ lặp lại đúng `3` lần).

**Trước khi chạy thử**, bạn đoán: TF-IDF thô và BM25 có chọn CÙNG một đoạn ở
hạng `1` không?

:::opt{correct}
Không — TF-IDF thô chọn đoạn `8` (điểm `13,1833`, cao nhất nhờ lặp từ nhiều
lần không giới hạn); BM25 chọn đoạn `3` (điểm `5,07`), vì bão hoà làm giảm
lợi thế của việc lặp từ và chuẩn hoá độ dài phạt đoạn `8` vì nó dài hơn hẳn
trung bình kho
:::

:::opt
Có, cả hai đều chọn đoạn `8` — vì đoạn đó chứa nhiều lần xuất hiện của MỌI
từ khoá trong truy vấn, nên bất kỳ công thức xếp hạng hợp lý nào cũng phải
ưu tiên nó
::why
Gần đúng ở việc đoạn `8` THẬT SỰ chứa nhiều lần xuất hiện của mọi từ khoá —
quan sát về dữ liệu đó đúng.

Chỗ lệch: "hợp lý" không có nghĩa là "luôn ưu tiên đếm nhiều nhất". Chính
mục tiêu của BM25 là KHÔNG để một đoạn thắng chỉ vì lặp từ nhiều — bão hoà
và chuẩn hoá độ dài được thiết kế RIÊNG để chặn đúng chiến thuật "lặp từ
khoá nhiều lần trong một đoạn dài" mà đoạn `8` đang thể hiện.
::
:::

:::opt
Có, cả hai đều chọn đoạn `3` — vì đoạn `3` ngắn hơn nên luôn được các công
thức tìm kiếm từ khoá ưu ái hơn đoạn dài, bất kể công thức cụ thể là gì
::why
Gần đúng ở KẾT LUẬN CUỐI CÙNG (đoạn `3` đúng là được BM25 ưu tiên) — nhưng
lý do đưa ra sai.

Chỗ lệch: TF-IDF thô KHÔNG hề ưu ái đoạn ngắn — nó không hề biết tới khái
niệm "độ dài trung bình kho" hay có phép chuẩn hoá nào theo độ dài, nên với
TF-IDF thô, đoạn `8` (dài, lặp từ nhiều) vẫn thắng đoạn `3` với điểm số
cao hơn hẳn (`13,1833` so với `4,3944`). Chỉ BM25 — nhờ tham số `b` — mới
chủ động phạt đoạn dài.
::
:::
::::

::::code{#viet_xep_hang_bm25_toan_kho}
Hoàn thiện `do_dai_trung_binh` (độ dài trung bình của mọi đoạn trong kho) và
`diem_bm25_doan` (tổng điểm BM25 qua mọi từ khoá của truy vấn cho một đoạn).

```python title=starter
import math

KHO_TAI_LIEU = [
    "may tinh chay nhanh nho co vi xu ly manh me",
    "phan mem tot giup nguoi dung lam viec hieu qua hon truoc",
    "lap trinh vien can hieu ro thuat toan truoc khi viet phan mem",
    "du lieu duoc luu tru va xu ly boi nhieu thuat toan khac nhau",
    "mon an ngon phu thuoc vao gia vi va cong thuc nau chuan",
    "dau bep gioi luon can than khi chon nguyen lieu tuoi ngon",
    "nha bep sach se va giup mon an ngon hon va gon gang hon",
    "mon trang mieng ngot ngao thuong duoc dung sau bua an chinh",
    (
        "cong nghe hien nay phat trien rat nhanh nhieu nguoi quan tam toi thuat toan moi "
        "thuat toan cu deu duoc nghien cuu dua vao du lieu xu ly qua nhieu buoc xu ly khac nhau "
        "moi buoc xu ly deu quan trong nguoi ta cung quan tam toi cach to chuc va sap xep du lieu "
        "sao cho de tim kiem thuat toan sap xep la mot phan quan trong cua khoa hoc may tinh hien dai"
    ),
]


def dem_tan_so(tu, van_ban):
    cac_tu = van_ban.lower().split()
    return cac_tu.count(tu)


def dem_so_doan_chua_tu(tu, kho):
    return sum(1 for doan in kho if tu in doan.lower().split())


def tinh_idf(tu, kho):
    df = dem_so_doan_chua_tu(tu, kho)
    if df == 0:
        return 0.0
    return math.log(len(kho) / df)


def diem_tf_idf(doan, tu_truy_van, kho):
    return sum(dem_tan_so(t, doan) * tinh_idf(t, kho) for t in tu_truy_van)


def he_so_chuan_hoa_do_dai(do_dai_doan, do_dai_trung_binh, b):
    return 1 - b + b * (do_dai_doan / do_dai_trung_binh)


def diem_bm25_mot_tu(tf, idf_tu, do_dai_doan, do_dai_trung_binh, k1, b):
    chuan_hoa = he_so_chuan_hoa_do_dai(do_dai_doan, do_dai_trung_binh, b)
    tu_so = tf * (k1 + 1)
    mau_so = tf + k1 * chuan_hoa
    return idf_tu * (tu_so / mau_so)


def do_dai_trung_binh(kho):
    return sum(len(doan.lower().split()) for doan in kho) / ___          # len(kho)


def diem_bm25_doan(doan, tu_truy_van, kho, k1, b):
    avgdl = do_dai_trung_binh(kho)
    do_dai_doan = len(doan.lower().split())
    tong = 0.0
    for t in tu_truy_van:
        tf_t = dem_tan_so(t, doan)
        if tf_t == 0:
            continue
        tong += diem_bm25_mot_tu(tf_t, tinh_idf(t, kho), do_dai_doan, avgdl, k1, b)
    return ___                                                            # tong


TU_TRUY_VAN = ["thuat", "toan", "xu", "ly"]
K1, B = 1.5, 0.75

diem_tfidf = [diem_tf_idf(doan, TU_TRUY_VAN, KHO_TAI_LIEU) for doan in KHO_TAI_LIEU]
diem_bm25 = [diem_bm25_doan(doan, TU_TRUY_VAN, KHO_TAI_LIEU, K1, B) for doan in KHO_TAI_LIEU]

xep_hang_tfidf = sorted(range(len(KHO_TAI_LIEU)), key=lambda i: diem_tfidf[i], reverse=True)
xep_hang_bm25 = sorted(range(len(KHO_TAI_LIEU)), key=lambda i: diem_bm25[i], reverse=True)

print([round(d, 4) for d in diem_tfidf])
print([round(d, 4) for d in diem_bm25])
print(xep_hang_tfidf[0])
print(xep_hang_bm25[0])
print(xep_hang_tfidf[0] == xep_hang_bm25[0])
```

```python title=solution
import math

KHO_TAI_LIEU = [
    "may tinh chay nhanh nho co vi xu ly manh me",
    "phan mem tot giup nguoi dung lam viec hieu qua hon truoc",
    "lap trinh vien can hieu ro thuat toan truoc khi viet phan mem",
    "du lieu duoc luu tru va xu ly boi nhieu thuat toan khac nhau",
    "mon an ngon phu thuoc vao gia vi va cong thuc nau chuan",
    "dau bep gioi luon can than khi chon nguyen lieu tuoi ngon",
    "nha bep sach se va giup mon an ngon hon va gon gang hon",
    "mon trang mieng ngot ngao thuong duoc dung sau bua an chinh",
    (
        "cong nghe hien nay phat trien rat nhanh nhieu nguoi quan tam toi thuat toan moi "
        "thuat toan cu deu duoc nghien cuu dua vao du lieu xu ly qua nhieu buoc xu ly khac nhau "
        "moi buoc xu ly deu quan trong nguoi ta cung quan tam toi cach to chuc va sap xep du lieu "
        "sao cho de tim kiem thuat toan sap xep la mot phan quan trong cua khoa hoc may tinh hien dai"
    ),
]


def dem_tan_so(tu, van_ban):
    cac_tu = van_ban.lower().split()
    return cac_tu.count(tu)


def dem_so_doan_chua_tu(tu, kho):
    return sum(1 for doan in kho if tu in doan.lower().split())


def tinh_idf(tu, kho):
    df = dem_so_doan_chua_tu(tu, kho)
    if df == 0:
        return 0.0
    return math.log(len(kho) / df)


def diem_tf_idf(doan, tu_truy_van, kho):
    return sum(dem_tan_so(t, doan) * tinh_idf(t, kho) for t in tu_truy_van)


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


TU_TRUY_VAN = ["thuat", "toan", "xu", "ly"]
K1, B = 1.5, 0.75

diem_tfidf = [diem_tf_idf(doan, TU_TRUY_VAN, KHO_TAI_LIEU) for doan in KHO_TAI_LIEU]
diem_bm25 = [diem_bm25_doan(doan, TU_TRUY_VAN, KHO_TAI_LIEU, K1, B) for doan in KHO_TAI_LIEU]

xep_hang_tfidf = sorted(range(len(KHO_TAI_LIEU)), key=lambda i: diem_tfidf[i], reverse=True)
xep_hang_bm25 = sorted(range(len(KHO_TAI_LIEU)), key=lambda i: diem_bm25[i], reverse=True)

print([round(d, 4) for d in diem_tfidf])
print([round(d, 4) for d in diem_bm25])
print(xep_hang_tfidf[0])
print(xep_hang_bm25[0])
print(xep_hang_tfidf[0] == xep_hang_bm25[0])
```

```python title=test
assert [round(d, 4) for d in diem_tfidf] == [2.1972, 0.0, 2.1972, 4.3944, 0.0, 0.0, 0.0, 0.0, 13.1833], f"diem TF-IDF sai -- dang ra {[round(d, 4) for d in diem_tfidf]}"
assert [round(d, 4) for d in diem_bm25] == [2.7504, 0.0, 2.6029, 5.07, 0.0, 0.0, 0.0, 0.0, 4.2325], f"diem BM25 sai -- dang ra {[round(d, 4) for d in diem_bm25]}"
assert xep_hang_tfidf[0] == 8, f"TF-IDF phai xep doan 8 (dai, lap tu) o hang 1 -- dang ra {xep_hang_tfidf[0]}"
assert xep_hang_bm25[0] == 3, f"BM25 phai xep doan 3 (ngan, khop sat) o hang 1 -- dang ra {xep_hang_bm25[0]}"
assert xep_hang_tfidf[0] != xep_hang_bm25[0], "TF-IDF va BM25 phai xep hang KHAC nhau tren du lieu nay"
assert diem_bm25[8] < diem_bm25[3], "theo BM25, doan 8 (dai) phai co diem THAP hon doan 3 (ngan, khop sat)"
assert diem_tfidf[8] > diem_tfidf[3], "theo TF-IDF, doan 8 (dai) van co diem CAO hon doan 3 -- xac nhan TF-IDF khong sua duoc loi nay"

# kiem tra truc tiep do_dai_trung_binh tren mot kho nho, tu tinh tay duoc
assert do_dai_trung_binh(["a b", "c d e f"]) == 3.0, f"(2+4)/2 = 3.0 -- dang ra {do_dai_trung_binh(['a b', 'c d e f'])}"

# bien: truy van khong khop doan nao phai cho diem 0, khong loi
assert diem_bm25_doan(KHO_TAI_LIEU[0], ["khongtontai"], KHO_TAI_LIEU, K1, B) == 0.0, "truy van khong khop tu nao phai cho diem BM25 = 0.0"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `do_dai_trung_binh`) là MẪU SỐ của phép chia — tổng độ dài chia cho SỐ ĐOẠN trong kho. Chỗ hai (trong `diem_bm25_doan`) là giá trị TRẢ VỀ cuối cùng — biến tích luỹ điểm số qua vòng lặp (`tong`), đã cộng dồn xong ở trên.
- kind: strategy
  body: 'Chỗ đầu: `len(kho)` — số đoạn trong kho. Chỗ hai: `tong` — biến đã tích luỹ điểm BM25 qua mọi từ khoá.'
- kind: one-line
  body: 'Chỗ đầu là `len(kho)`, chỗ hai là `tong`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai chia cho DUNG len(kho) (so doan trong kho, khong phai mot hang so khac); VA cho trong hai phai TRA VE DUNG bien 'tong' da tich luy (khong duoc tra ve mot gia tri khac)
  requireAst:
  - kind: uses-call, target: len, min: 6
  - kind: uses-name, target: tong, min: 1
  # Da thu that (trich nguyen ham _dem cua kiem-ast.ts vao mot file rieng,
  # chay qua python3 TREN CHINH van ban solution da trich tu file nay) -- ket
  # qua [6, 1] cho hai luat theo dung thu tu khai bao o tren.
  #
  # 🔴 GOTCHA "boilerplate-threshold-masking" o muc do MANH: len() xuat hien
  # 6 LAN trong toan bo solution, KHONG PHAI 1 -- (1) "len(doan.lower().split())"
  # co san trong do_dai_trung_binh, (2) "len(kho)" o cho trong dau, (3)
  # "len(doan.lower().split())" co san trong diem_bm25_doan (tinh
  # do_dai_doan), (4) "len(kho)" co san trong tinh_idf, (5)+(6)
  # "len(KHO_TAI_LIEU)" xuat hien HAI LAN trong hai loi goi
  # "sorted(range(len(KHO_TAI_LIEU)), ...)" o cuoi bai. Neu chi dat min=1
  # (ngay tho), mot mutant BO cho trong dau (vi du dien mot hang so "8" thay
  # vi "len(kho)") van qua duoc vi con lai 5 lan len() khac trong solution --
  # da dat min=6 (TONG THAT, do bang cong cu, khong doan tay) de chan dung
  # truong hop nay.
  # tong=1: CHI mot lan DOC (Load) bien 'tong' trong toan bo solution, dung o
  # cho trong hai ("return tong"). Dong "tong = 0.0" la Store (khoi tao,
  # khong tinh), dong "tong += ..." co ve trai la Store trong AST cua Python
  # (khong tinh boi uses-name, vi day la dich cua AugAssign) -- nen tong CHI
  # duoc DOC dung mot lan, o cho trong hai.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua ham
  # _dem de xac nhan, khong doan tay): dien "tong" vao cho trong dau ("return
  # sum(len(doan.lower().split()) for doan in kho) / tong" trong
  # do_dai_trung_binh) VA dien "len(kho)" vao cho trong hai ("return
  # len(kho)" trong diem_bm25_doan) -- ket qua AST van la [6, 1], Y HET ban
  # dung (tong so lan goi len() va tong so lan doc 'tong' khong doi, chi doi
  # VI TRI). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong do_dai_trung_binh(kho), ten
  # 'tong' KHONG TON TAI (day la bien CUC BO cua diem_bm25_doan, mot ham
  # khac) -- da tu chay THAT mutant nay qua python3, xac nhan no nem
  # NameError: name 'tong' is not defined ngay khi do_dai_trung_binh() duoc
  # goi lan dau (tu ben trong diem_bm25_doan) -- bi chan boi tier 'run', doc
  # lap voi static.
  # Da ra soat GOTCHA #6 cho luat uses-call (len): moi loi goi len(...) trong
  # solution deu nhan mot doi so KHAC nhau ve kieu/gia tri (chuoi da tach tu,
  # danh sach kho, danh sach KHO_TAI_LIEU) -- khong co rui ro "dung sai bien
  # cung do dai" vi ham len() luon tra ve mot SO NGUYEN dung voi DOI TUONG
  # duoc truyen, khong co hai doi tuong nao trong scope cung do dai de nham
  # lan lam thay doi KET QUA cuoi cung ma khong bi assert bat.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[2\\.1972, 0\\.0, 2\\.1972, 4\\.3944, 0\\.0, 0\\.0, 0\\.0, 0\\.0, 13\\.1833\\]\\n\\[2\\.7504, 0\\.0, 2\\.6029, 5\\.07, 0\\.0, 0\\.0, 0\\.0, 0\\.0, 4\\.2325\\]\\n8\\n3\\nFalse\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`xep_hang_tfidf[0] = 8` (đoạn lặp từ, sai), `xep_hang_bm25[0] = 3` (đoạn
khớp sát, đúng) — BM25 xếp hạng KHÁC và ĐÚNG HƠN, đo được bằng số cụ thể
trên cả một kho tài liệu. Quest tiếp theo sẽ đối đầu trực tiếp BM25 với
cosine similarity — mỗi cách có một điểm mù riêng.
::::

::::reflect{#nghi-lai}
TF-IDF thô có một điểm mù nghiêm trọng khi áp dụng lên một kho THẬT: nó
không phòng vệ được trước một đoạn văn cố tình (hoặc tình cờ) lặp lại từ
khoá nhiều lần để "trông có vẻ liên quan hơn". BM25 vá đúng lỗ hổng đó bằng
hai cơ chế đã xây ở bài trước — không phải bằng cách đoán mò, mà bằng một
công thức có tham số tường minh (`k1`, `b`) điều khiển được mức độ bão hoà
và mức độ phạt độ dài. Đây là lý do BM25, dù ra đời từ thập niên `1990`, vẫn
là chuẩn công nghiệp cho tìm kiếm từ khoá cho tới tận ngày nay. Nhưng BM25
vẫn có một điểm mù mà VECTOR search (q8.5a-c) không có, và ngược lại — bài
BOSS tiếp theo đo đúng sự khác biệt đó bằng số cụ thể.
::::

::::checkpoint{mastery=0.9}
::::
