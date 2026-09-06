---
id: tri-tue-nhan-tao.bm25-va-tim-kiem-tu-khoa.do-hiem-cua-tu-idf
title: "Độ hiếm của một từ: inverse document frequency (IDF)"
summary: "Kho KHO_TAI_LIEU gom 8 doan (4 cong nghe, 4 am thuc). tinh_idf(tu, kho) = log(N / df(tu)), voi df(tu) la SO DOAN chua tu do (khong phai tong so lan xuat hien). 'va' xuat hien trong 3/8 doan (idf = 0,9808), 'thuat' xuat hien trong 2/8 doan (idf = 1,3863), 'may' xuat hien trong dung 1/8 doan (idf = 2,0794). idf(may) > idf(thuat) > idf(va) -- do bang so THAT: tu CANG HIEM (xuat hien trong CANG IT doan) thi IDF CANG CAO, bat ke tu do co lap lai nhieu lan trong tung doan hay khong."
locale: vi
track: tri-tue-nhan-tao
module: bm25-va-tim-kiem-tu-khoa
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.do-hiem-cua-tu-idf]
requires: [ai.tan-so-tu-tho-va-thien-vi-do-dai]
concepts: [ai.do-hiem-cua-tu-idf]
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
Bài trước đo được: TF thô thiên vị đoạn dài. Nhưng có một lỗ hổng KHÁC, độc
lập với độ dài: TF thô cũng không phân biệt được một từ HIẾM (như một thuật
ngữ chuyên môn) với một từ PHỔ BIẾN (như "và", "là", "của"). Bài này đo đúng
sự khác biệt đó.
::::

::::explain{#idf_la_gi}
Trực giác: nếu một từ khoá xuất hiện ở HẦU HẾT mọi đoạn trong kho ngữ liệu,
nó không giúp phân biệt đoạn nào liên quan hơn đoạn nào — mọi đoạn đều có nó.
Ngược lại, nếu một từ khoá chỉ xuất hiện ở MỘT VÀI đoạn hiếm hoi, thì việc
một đoạn CÓ từ đó là một tín hiệu mạnh: đoạn đó nhiều khả năng liên quan tới
đúng điều truy vấn đang hỏi.

**Document frequency** (`df`, "tần số tài liệu") của một từ là **số đoạn văn
bản** (không phải tổng số lần xuất hiện) chứa từ đó ít nhất một lần. Một từ
xuất hiện `5` lần trong CÙNG một đoạn nhưng không xuất hiện ở đoạn nào khác
vẫn có `df = 1` — `df` đếm ĐOẠN, không đếm LẦN.

**Inverse document frequency** (`idf`, "nghịch đảo tần số tài liệu") biến
`df` thành một điểm số ĐỘ HIẾM:

```
idf(t) = log(N / df(t))
```

với `N` là tổng số đoạn trong kho, `df(t)` là số đoạn chứa từ `t`. Khi `df(t)`
nhỏ (từ hiếm, xuất hiện ở ít đoạn), tỉ số `N / df(t)` lớn, nên `log` của nó
lớn — `idf` CAO. Khi `df(t)` gần bằng `N` (từ phổ biến, xuất hiện ở hầu hết
đoạn), tỉ số `N / df(t)` gần `1`, `log` của nó gần `0` — `idf` THẤP. Trường
hợp `df(t) = N` (từ có mặt ở MỌI đoạn) cho `idf = log(1) = 0` — từ đó hoàn
toàn không có giá trị phân biệt. Trường hợp một từ không xuất hiện ở đoạn
nào (`df(t) = 0`), quy ước trả về `idf = 0.0` thay vì để `log(N / 0)` ném
lỗi chia cho `0`.
::::

::::example{#do_idf_ba_tu}
Một kho `8` đoạn văn (`4` đoạn công nghệ, `4` đoạn ẩm thực), đo `idf` của ba
từ: `"va"` (một liên từ phổ biến), `"thuat"` (một phần của cụm "thuật toán",
xuất hiện vừa phải), `"may"` (một từ hiếm, chỉ xuất hiện đúng một lần):

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
]


def dem_so_doan_chua_tu(tu, kho):
    return sum(1 for doan in kho if tu in doan.lower().split())


def tinh_idf(tu, kho):
    df = dem_so_doan_chua_tu(tu, kho)
    if df == 0:
        return 0.0
    return math.log(len(kho) / df)


idf_va = tinh_idf("va", KHO_TAI_LIEU)
idf_thuat = tinh_idf("thuat", KHO_TAI_LIEU)
idf_may = tinh_idf("may", KHO_TAI_LIEU)

print(dem_so_doan_chua_tu("va", KHO_TAI_LIEU))
print(dem_so_doan_chua_tu("thuat", KHO_TAI_LIEU))
print(dem_so_doan_chua_tu("may", KHO_TAI_LIEU))
print(round(idf_va, 4))
print(round(idf_thuat, 4))
print(round(idf_may, 4))
print(idf_may > idf_thuat > idf_va)
```

```text title=readonly
3
2
1
0.9808
1.3863
2.0794
True
```

`"va"` xuất hiện ở `3` trong `8` đoạn (`df=3`) — một liên từ phổ biến, gặp ở
cả đoạn công nghệ lẫn ẩm thực — cho `idf = log(8/3) ≈ 0,9808`, THẤP. `"thuat"`
(một phần của "thuật toán") xuất hiện ở `2` đoạn (`df=2`) — vừa phải — cho
`idf = log(8/2) ≈ 1,3863`. `"may"` chỉ xuất hiện ở đúng `1` đoạn (`df=1`,
hiếm nhất) — cho `idf = log(8/1) ≈ 2,0794`, CAO NHẤT. Xếp hạng
`idf(may) > idf(thuat) > idf(va)` khớp đúng thứ tự NGƯỢC với độ phổ biến
(`df=1 < df=2 < df=3`) — càng hiếm càng có giá trị phân biệt cao, đo được
bằng số thật.
::::

::::predict{#doan_idf_hiem_cao_hon commitOnce}
Xét đúng ba từ ở ví dụ trên: `"va"` xuất hiện ở `3/8` đoạn, `"may"` xuất
hiện ở đúng `1/8` đoạn.

**Trước khi chạy thử**, bạn đoán: `idf("va")` so với `idf("may")` — từ nào
có `idf` CAO HƠN?

:::opt{correct}
`"may"` cao hơn — nó hiếm hơn (`df=1` so với `df=3`), và `idf = log(N/df)`
tỉ lệ NGHỊCH với `df`: `df` càng nhỏ thì `N/df` càng lớn, nên `log` của nó
càng lớn
:::

:::opt
`"va"` cao hơn, vì nó xuất hiện ở nhiều đoạn hơn (`3` đoạn so với `1` đoạn)
nên phải "quan trọng" hơn — xuất hiện nhiều hơn thường có nghĩa là có ảnh
hưởng lớn hơn
::why
Gần đúng ở việc `"va"` THẬT SỰ xuất hiện ở nhiều đoạn hơn (`df=3 > df=1`) —
quan sát đó đúng.

Chỗ lệch: `idf` đo GIÁ TRỊ PHÂN BIỆT, không đo "mức độ quan trọng chung". Một
từ xuất hiện ở CÀNG NHIỀU đoạn thì CÀNG ÍT giúp phân biệt đoạn nào khác đoạn
nào — vì hầu như đoạn nào cũng có nó. `idf` được định nghĩa để PHẠT sự phổ
biến đó, không thưởng nó.
::
:::

:::opt
Cả hai bằng nhau, vì `idf` chỉ phụ thuộc vào kích thước kho `N` (ở đây cố
định là `8`), không phụ thuộc vào từng từ cụ thể
::why
Gần đúng ở việc `N` (kích thước kho) THẬT SỰ là một hằng số cố định trong
công thức (`8` cho cả hai từ) — quan sát đó đúng, `N` không đổi.

Chỗ lệch: công thức `idf(t) = log(N / df(t))` có HAI đại lượng, không chỉ
`N` — mẫu số `df(t)` THAY ĐỔI theo từng từ cụ thể (`df("va")=3` khác
`df("may")=1`), và chính sự khác biệt đó ở mẫu số làm hai giá trị `idf` khác
nhau, dù tử số `N` giống nhau.
::
:::
::::

::::code{#viet_dem_df_va_idf}
Hoàn thiện `dem_so_doan_chua_tu` (đếm số ĐOẠN chứa một từ, không đếm tổng số
lần xuất hiện) và `tinh_idf` (áp dụng công thức `log(N / df)`).

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
]


def dem_so_doan_chua_tu(tu, kho):
    return sum(1 for doan in kho if ___ in doan.lower().split())    # tu


def tinh_idf(tu, kho):
    df = dem_so_doan_chua_tu(tu, kho)
    if df == 0:
        return 0.0
    return math.log(len(kho) / ___)                                # df


idf_va = tinh_idf("va", KHO_TAI_LIEU)
idf_thuat = tinh_idf("thuat", KHO_TAI_LIEU)
idf_may = tinh_idf("may", KHO_TAI_LIEU)

print(dem_so_doan_chua_tu("va", KHO_TAI_LIEU))
print(dem_so_doan_chua_tu("thuat", KHO_TAI_LIEU))
print(dem_so_doan_chua_tu("may", KHO_TAI_LIEU))
print(round(idf_va, 4))
print(round(idf_thuat, 4))
print(round(idf_may, 4))
print(idf_may > idf_thuat > idf_va)
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
]


def dem_so_doan_chua_tu(tu, kho):
    return sum(1 for doan in kho if tu in doan.lower().split())


def tinh_idf(tu, kho):
    df = dem_so_doan_chua_tu(tu, kho)
    if df == 0:
        return 0.0
    return math.log(len(kho) / df)


idf_va = tinh_idf("va", KHO_TAI_LIEU)
idf_thuat = tinh_idf("thuat", KHO_TAI_LIEU)
idf_may = tinh_idf("may", KHO_TAI_LIEU)

print(dem_so_doan_chua_tu("va", KHO_TAI_LIEU))
print(dem_so_doan_chua_tu("thuat", KHO_TAI_LIEU))
print(dem_so_doan_chua_tu("may", KHO_TAI_LIEU))
print(round(idf_va, 4))
print(round(idf_thuat, 4))
print(round(idf_may, 4))
print(idf_may > idf_thuat > idf_va)
```

```python title=test
assert dem_so_doan_chua_tu("va", KHO_TAI_LIEU) == 3, f"'va' phai xuat hien o 3 doan -- dang ra {dem_so_doan_chua_tu('va', KHO_TAI_LIEU)}"
assert dem_so_doan_chua_tu("thuat", KHO_TAI_LIEU) == 2, f"'thuat' phai xuat hien o 2 doan -- dang ra {dem_so_doan_chua_tu('thuat', KHO_TAI_LIEU)}"
assert dem_so_doan_chua_tu("may", KHO_TAI_LIEU) == 1, f"'may' phai xuat hien o 1 doan -- dang ra {dem_so_doan_chua_tu('may', KHO_TAI_LIEU)}"
assert round(idf_va, 4) == 0.9808, f"idf('va') phai xap xi 0,9808 -- dang ra {idf_va}"
assert round(idf_thuat, 4) == 1.3863, f"idf('thuat') phai xap xi 1,3863 -- dang ra {idf_thuat}"
assert round(idf_may, 4) == 2.0794, f"idf('may') phai xap xi 2,0794 -- dang ra {idf_may}"
assert idf_may > idf_thuat > idf_va, "tu CANG HIEM (df cang nho) thi idf phai CANG CAO"

# bien QUAN TRONG: tu KHONG xuat hien o doan nao phai cho idf = 0.0, khong loi
assert tinh_idf("khongtontai", KHO_TAI_LIEU) == 0.0, "tu khong xuat hien o doan nao phai cho idf = 0.0, khong duoc nem loi"

# kiem tra truc tiep dem_so_doan_chua_tu tren mot kho nho, tu dem tay duoc
kho_nho = ["a b c", "a d e", "f g h"]
assert dem_so_doan_chua_tu("a", kho_nho) == 2, f"'a' xuat hien o 2/3 doan -- dang ra {dem_so_doan_chua_tu('a', kho_nho)}"
assert dem_so_doan_chua_tu("f", kho_nho) == 1, f"'f' xuat hien o 1/3 doan -- dang ra {dem_so_doan_chua_tu('f', kho_nho)}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `dem_so_doan_chua_tu`) là điều kiện của generator expression `sum(1 for doan in kho if ___ in doan.lower().split())` — kiểm tra xem từ đang xét (tham số `tu`) có nằm trong danh sách từ của đoạn hay không. Chỗ hai (trong `tinh_idf`) là MẪU SỐ của phép chia `len(kho) / ___` — số đoạn chứa từ đó, đã tính sẵn ở dòng trên và lưu trong biến `df`.
- kind: strategy
  body: 'Chỗ đầu: `tu in doan.lower().split()` — kiểm tra từ có mặt trong đoạn. Chỗ hai: `len(kho) / df` — chia tổng số đoạn cho số đoạn chứa từ.'
- kind: one-line
  body: 'Chỗ đầu là `tu`, chỗ hai là `df`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai kiem tra DUNG bien 'tu' co trong danh sach tu cua doan hay khong (khong duoc kiem tra mot ten khac); VA cho trong hai phai chia cho DUNG bien 'df' da tinh o dong tren (khong duoc chia cho len(kho) hay mot hang so khac)
  requireAst:
  - kind: uses-name, target: tu, min: 2
  - kind: uses-name, target: df, min: 2
  # Da thu that (trich nguyen ham _dem cua kiem-ast.ts vao mot file rieng,
  # chay qua python3 TREN CHINH van ban solution da trich tu file nay) -- ket
  # qua [2, 2] cho hai luat theo dung thu tu khai bao o tren.
  #
  # 🔴 GOTCHA "boilerplate-threshold-masking" (da do THAT, khong doan tay):
  # 'tu' KHONG chi doc mot lan o cho trong dau -- no CON duoc doc mot lan NUA
  # ben trong tinh_idf, o dong "df = dem_so_doan_chua_tu(tu, kho)" (truyen
  # tham so 'tu' cua chinh tinh_idf sang ham kia). Neu chi dat min=1 (ngay
  # tho), mot mutant dien SAI cho trong dau (vi du dien "kho" thay vi "tu",
  # lam dieu kien luon False vi so sanh mot LIST voi tung PHAN TU cua
  # .split()) van qua duoc vi con lai dung 1 lan doc 'tu' o loi goi
  # dem_so_doan_chua_tu(tu, kho) trong tinh_idf -- phai dat min=2 (TONG THAT)
  # thi mutant nay moi tut xuong 1, duoi nguong, bi chan.
  # Tuong tu, 'df' KHONG chi duoc doc o cho trong hai -- no CON duoc doc mot
  # lan NUA o dong "if df == 0:" ngay tren. Neu chi dat min=1, mot mutant
  # dien SAI cho trong hai (vi du dien "len(kho)" thay vi "df", lam idf luon
  # bang log(1)=0 cho moi tu) van qua duoc vi con lai 1 lan doc 'df' o dong
  # kiem tra "if df == 0". Da dat min=2 (TONG THAT) cho ca hai, xac nhan
  # dung bang dem THAT o tren, khong doan tay.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua ham
  # _dem de xac nhan, khong doan tay): dien "df" vao cho trong dau ("if df in
  # doan.lower().split()") VA dien "tu" vao cho trong hai ("return
  # math.log(len(kho) / tu)") -- ket qua AST van la [2, 2], Y HET ban dung
  # (moi ten van duoc doc dung 2 lan tong cong, chi doi VI TRI cua MOT trong
  # hai lan doc). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong dem_so_doan_chua_tu(tu, kho),
  # ten 'df' KHONG ton tai trong scope cua ham nay (df la bien CUC BO cua
  # tinh_idf, o mot ham khac hoan toan) -- da tu chay THAT mutant nay qua
  # python3, xac nhan no nem NameError: name 'df' is not defined ngay khi
  # tinh_idf() goi dem_so_doan_chua_tu() lan dau tien -- bi chan boi tier
  # 'run', doc lap voi static.
  # Da ra soat GOTCHA #6 (uses-name khong phai uses-call nen khong co rui ro
  # "dung sai bien cung do dai" o day -- ca hai luat deu la uses-name, kiem
  # dung TEN duoc doc, khong lien quan toi doi so ham).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^3\\n2\\n1\\n0\\.9808\\n1\\.3863\\n2\\.0794\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`idf(may) > idf(thuat) > idf(va)` — càng hiếm càng có giá trị phân biệt cao,
đo được bằng số thật, không suy đoán. Bài sau ghép TF với IDF thành một điểm
số duy nhất cho mỗi đoạn văn: TF-IDF.
::::

::::reflect{#nghi-lai}
`idf` không quan tâm một từ được LẶP LẠI bao nhiêu lần trong một đoạn — nó
chỉ quan tâm từ đó có mặt ở BAO NHIÊU đoạn trên toàn kho. Đây là mảnh ghép mà
TF thô (bài trước) hoàn toàn thiếu: TF chỉ nhìn vào MỘT đoạn tại một thời
điểm, không biết gì về phần còn lại của kho ngữ liệu. `idf` nhìn ra TOÀN BỘ
kho, và trả lời đúng câu hỏi "từ này có ĐẶC BIỆT không, hay nó xuất hiện ở
khắp mọi nơi?". Bài sau ghép cả hai đại lượng lại: `tf(t,d) * idf(t)` — một
điểm số vừa biết một từ xuất hiện NHIỀU hay ÍT trong một đoạn cụ thể, vừa
biết từ đó có HIẾM hay không trên toàn kho.
::::

::::checkpoint{mastery=0.85}
::::
