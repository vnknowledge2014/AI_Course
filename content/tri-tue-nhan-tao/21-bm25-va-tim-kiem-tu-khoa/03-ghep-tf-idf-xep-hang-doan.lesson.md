---
id: tri-tue-nhan-tao.bm25-va-tim-kiem-tu-khoa.ghep-tf-idf-xep-hang-doan
title: "Ghép TF-IDF: xếp hạng đoạn văn cho một truy vấn"
summary: "diem_tf_idf(doan, tu_truy_van, kho) = tong tf(t,doan)*idf(t,kho) qua moi tu t trong truy van. Tren KHO_TAI_LIEU 8 doan voi truy van ['va','may']: xep hang theo TF THO (khong idf) chon doan chi so 6 (co 'va' x2, tu pho bien) o hang 1; xep hang theo TF-IDF chon doan chi so 0 (co 'may' x1, tu hiem) o hang 1 -- HAI CACH XEP HANG KHAC NHAU tren CUNG mot du lieu, vi tf-idf cho tu hiem 'may' (idf=2,0794) trong luong hon han tu pho bien 'va' lap lai (idf=0,9808 x2 = 1,9617 < 2,0794)."
locale: vi
track: tri-tue-nhan-tao
module: bm25-va-tim-kiem-tu-khoa
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.ghep-tf-idf-xep-hang-doan]
requires: [ai.do-hiem-cua-tu-idf]
concepts: [ai.ghep-tf-idf-xep-hang-doan]
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
Hai mảnh đã có: TF (bài `1`, đếm số lần) và IDF (bài `2`, đo độ hiếm). Bài
này ghép cả hai thành MỘT điểm số duy nhất cho một đoạn văn, cho một truy
vấn nhiều từ — và cho thấy điều đó thật sự đổi kết quả xếp hạng.
::::

::::explain{#ghep_tf_idf}
Một truy vấn thường có NHIỀU từ khoá, không chỉ một. Điểm **TF-IDF** của một
đoạn văn `d` cho một truy vấn `q` là TỔNG, qua mọi từ khoá `t` trong `q`, của
tích `tf(t,d) * idf(t)`:

```
diem_tf_idf(d, q) = tong tf(t,d) * idf(t)   voi moi t trong q
```

Trực giác: một từ khoá xuất hiện NHIỀU lần trong đoạn (`tf` cao) VÀ hiếm trên
toàn kho (`idf` cao) đóng góp một điểm số LỚN. Một từ khoá xuất hiện nhiều
lần nhưng PHỔ BIẾN khắp kho (`idf` thấp) chỉ đóng góp một điểm số nhỏ, dù
`tf` của nó cao — đúng điều `idf` được sinh ra để làm ở bài trước: hạ giá trị
của những từ không giúp phân biệt.

Đây là chỗ khác biệt thật sự so với xếp hạng bằng TF thô (bài `1`, không
nhân `idf`): **TF-IDF có thể đảo ngược một xếp hạng mà TF thô đã chọn**, nếu
đoạn thắng theo TF thô chỉ thắng nhờ LẶP LẠI một từ PHỔ BIẾN, trong khi đoạn
thua theo TF thô chứa một từ HIẾM hơn — dù ít lần hơn.
::::

::::example{#tf_idf_doi_nguoc_xep_hang}
Trên `KHO_TAI_LIEU` `8` đoạn của bài trước, truy vấn hai từ `["va", "may"]`
— `"va"` là một liên từ phổ biến (`idf ≈ 0,9808`), `"may"` là một từ hiếm
(`idf ≈ 2,0794`). Đoạn chỉ số `6` lặp lại `"va"` đúng `2` lần; đoạn chỉ số
`0` chứa `"may"` đúng `1` lần:

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


def diem_tf_tho(doan, tu_truy_van):
    return sum(dem_tan_so(t, doan) for t in tu_truy_van)


def diem_tf_idf(doan, tu_truy_van, kho):
    return sum(dem_tan_so(t, doan) * tinh_idf(t, kho) for t in tu_truy_van)


TU_TRUY_VAN = ["va", "may"]

diem_tho = [diem_tf_tho(doan, TU_TRUY_VAN) for doan in KHO_TAI_LIEU]
diem_idf = [diem_tf_idf(doan, TU_TRUY_VAN, KHO_TAI_LIEU) for doan in KHO_TAI_LIEU]

xep_hang_tho = sorted(range(len(KHO_TAI_LIEU)), key=lambda i: diem_tho[i], reverse=True)
xep_hang_idf = sorted(range(len(KHO_TAI_LIEU)), key=lambda i: diem_idf[i], reverse=True)

print(diem_tho)
print([round(d, 4) for d in diem_idf])
print(xep_hang_tho[0])
print(xep_hang_idf[0])
print(xep_hang_tho[0] == xep_hang_idf[0])
```

```text title=readonly
[1, 0, 0, 1, 1, 0, 2, 0]
[2.0794, 0.0, 0.0, 0.9808, 0.9808, 0.0, 1.9617, 0.0]
6
0
False
```

Xếp hạng theo TF THÔ: đoạn chỉ số `6` có điểm `2` (lặp `"va"` đúng `2` lần,
`"may"` `0` lần) — CAO NHẤT, đứng hạng `1`. Xếp hạng theo TF-IDF: đoạn chỉ số
`6` được `2 * idf("va") = 2 * 0,9808 ≈ 1,9617`, nhưng đoạn chỉ số `0` (chứa
`"may"` đúng `1` lần, không có `"va"`) được `1 * idf("may") = 1 * 2,0794 =
2,0794` — CAO HƠN. Kết quả: `xep_hang_tho[0] = 6` nhưng `xep_hang_idf[0] =
0` — HAI CÁCH XẾP HẠNG CHỌN HAI ĐOẠN KHÁC NHAU ở hạng `1`, trên CÙNG một dữ
liệu. Một lần lặp lại DUY NHẤT của một từ HIẾM có trọng số lớn hơn HAI lần
lặp lại của một từ PHỔ BIẾN.
::::

::::predict{#doan_tfidf_chon_doan_khac commitOnce}
Xét đúng ví dụ trên: đoạn chỉ số `6` có `2` lần `"va"` (`idf ≈ 0,9808`),
đoạn chỉ số `0` có `1` lần `"may"` (`idf ≈ 2,0794`). TF thô xếp đoạn `6` hạng
`1` (điểm `2 > 1`).

**Trước khi chạy thử**, bạn đoán: TF-IDF có xếp đoạn `6` ở hạng `1` giống TF
thô không?

:::opt{correct}
Không — TF-IDF của đoạn `6` là `2 * 0,9808 ≈ 1,9617`, thấp hơn TF-IDF của
đoạn `0` là `1 * 2,0794 = 2,0794`; trọng số IDF cao của từ hiếm `"may"` đủ để
một lần xuất hiện DUY NHẤT vượt qua hai lần lặp lại của từ phổ biến `"va"`
:::

:::opt
Có — vì đoạn `6` vẫn có tổng số lần xuất hiện từ khoá (`2` lần) nhiều hơn
đoạn `0` (`1` lần), và TF-IDF vẫn CỘNG DỒN theo số lần xuất hiện nên số lần
nhiều hơn phải luôn thắng
::why
Gần đúng ở việc TF-IDF THẬT SỰ cộng dồn theo số lần xuất hiện — quan sát về
phép CỘNG đó đúng, TF-IDF không bỏ qua số lần lặp lại.

Chỗ lệch: TF-IDF không chỉ cộng số lần xuất hiện — mỗi lần xuất hiện được
NHÂN thêm với `idf` của đúng từ đó trước khi cộng. Hai từ khác nhau có `idf`
khác nhau, nên "nhiều lần hơn" của một từ CÓ IDF THẤP không tự động thắng
"ít lần hơn" của một từ CÓ IDF CAO — phép nhân trọng số này chính là điều
TF thô (không nhân gì cả) hoàn toàn thiếu.
::
:::

:::opt
Có, nhưng chỉ vì đoạn `0` đứng trước đoạn `6` trong danh sách gốc — khi hai
đoạn có điểm số gần bằng nhau, `sorted` ưu tiên đoạn xuất hiện trước
::why
Gần đúng ở việc `sorted` ổn định THẬT SỰ giữ nguyên thứ tự gốc khi hai điểm
số BẰNG NHAU tuyệt đối — quan sát về tính ổn định của `sorted` đó đúng.

Chỗ lệch: ở đây không có tình huống "gần bằng nhau" nào — `2,0794` (đoạn
`0`) và `1,9617` (đoạn `6`) là hai giá trị THỰC SỰ khác nhau, cách nhau rõ
rệt, không phải bị hoà rồi phân định bằng thứ tự xuất hiện. Đoạn `0` thắng
vì điểm SỐ của nó cao hơn, không vì vị trí của nó trong danh sách gốc.
::
:::
::::

::::code{#viet_diem_tf_tho_va_tf_idf}
Hoàn thiện `diem_tf_tho` (tổng TF thô qua mọi từ trong truy vấn, không nhân
`idf`) và `diem_tf_idf` (tổng `tf(t,doan) * idf(t,kho)` qua mọi từ trong
truy vấn).

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


def diem_tf_tho(doan, tu_truy_van):
    return sum(dem_tan_so(t, doan) for t in ___)                       # tu_truy_van


def diem_tf_idf(doan, tu_truy_van, kho):
    return sum(dem_tan_so(t, doan) * ___ for t in tu_truy_van)         # tinh_idf(t, kho)


TU_TRUY_VAN = ["va", "may"]

diem_tho = [diem_tf_tho(doan, TU_TRUY_VAN) for doan in KHO_TAI_LIEU]
diem_idf = [diem_tf_idf(doan, TU_TRUY_VAN, KHO_TAI_LIEU) for doan in KHO_TAI_LIEU]

xep_hang_tho = sorted(range(len(KHO_TAI_LIEU)), key=lambda i: diem_tho[i], reverse=True)
xep_hang_idf = sorted(range(len(KHO_TAI_LIEU)), key=lambda i: diem_idf[i], reverse=True)

print(diem_tho)
print([round(d, 4) for d in diem_idf])
print(xep_hang_tho[0])
print(xep_hang_idf[0])
print(xep_hang_tho[0] == xep_hang_idf[0])
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


def diem_tf_tho(doan, tu_truy_van):
    return sum(dem_tan_so(t, doan) for t in tu_truy_van)


def diem_tf_idf(doan, tu_truy_van, kho):
    return sum(dem_tan_so(t, doan) * tinh_idf(t, kho) for t in tu_truy_van)


TU_TRUY_VAN = ["va", "may"]

diem_tho = [diem_tf_tho(doan, TU_TRUY_VAN) for doan in KHO_TAI_LIEU]
diem_idf = [diem_tf_idf(doan, TU_TRUY_VAN, KHO_TAI_LIEU) for doan in KHO_TAI_LIEU]

xep_hang_tho = sorted(range(len(KHO_TAI_LIEU)), key=lambda i: diem_tho[i], reverse=True)
xep_hang_idf = sorted(range(len(KHO_TAI_LIEU)), key=lambda i: diem_idf[i], reverse=True)

print(diem_tho)
print([round(d, 4) for d in diem_idf])
print(xep_hang_tho[0])
print(xep_hang_idf[0])
print(xep_hang_tho[0] == xep_hang_idf[0])
```

```python title=test
assert diem_tho == [1, 0, 0, 1, 1, 0, 2, 0], f"diem TF tho sai -- dang ra {diem_tho}"
assert [round(d, 4) for d in diem_idf] == [2.0794, 0.0, 0.0, 0.9808, 0.9808, 0.0, 1.9617, 0.0], f"diem TF-IDF sai -- dang ra {[round(d, 4) for d in diem_idf]}"
assert xep_hang_tho[0] == 6, f"TF tho phai xep doan 6 o hang 1 -- dang ra {xep_hang_tho[0]}"
assert xep_hang_idf[0] == 0, f"TF-IDF phai xep doan 0 o hang 1 -- dang ra {xep_hang_idf[0]}"
assert xep_hang_tho[0] != xep_hang_idf[0], "TF tho va TF-IDF phai chon HAI doan KHAC nhau o hang 1 tren du lieu nay"

# kiem tra truc tiep tren mot vi du nho, tu tinh tay duoc
kho_nho = ["a a b", "c"]
assert diem_tf_tho("a a b", ["a", "b"]) == 3, f"'a' x2 + 'b' x1 = 3 -- dang ra {diem_tf_tho('a a b', ['a', 'b'])}"
assert diem_tf_tho("a a b", ["z"]) == 0, "tu khong xuat hien phai cho diem 0"

# bien: truy van CHI mot tu phai bang dung tf(t,doan)*idf(t,kho) cua tu do
diem_mot_tu = diem_tf_idf(KHO_TAI_LIEU[0], ["may"], KHO_TAI_LIEU)
assert round(diem_mot_tu, 4) == 2.0794, f"truy van mot tu 'may' tren doan 0 phai la 2,0794 -- dang ra {diem_mot_tu}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `diem_tf_tho`) là danh sách được duyệt qua trong `for t in ___` — chính là truy vấn nhiều từ (tham số `tu_truy_van`). Chỗ hai (trong `diem_tf_idf`) là thừa số NHÂN THÊM vào `dem_tan_so(t, doan) * ___` — điểm `idf` của từ đang xét, tính bằng lời gọi `tinh_idf(t, kho)` đã xây ở bài trước.
- kind: strategy
  body: 'Chỗ đầu: `tu_truy_van` — duyệt qua đúng danh sách từ khoá truyền vào. Chỗ hai: `tinh_idf(t, kho)` — gọi đúng hàm IDF trên từ `t` đang xét và kho `kho`.'
- kind: one-line
  body: 'Chỗ đầu là `tu_truy_van`, chỗ hai là `tinh_idf(t, kho)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai duyet DUNG danh sach 'tu_truy_van' (khong duoc duyet mot danh sach khac); VA cho trong hai phai GOI THAT tinh_idf(t, kho) de nhan trong so IDF (khong duoc bo qua idf hay dung mot ham khac)
  requireAst:
  - kind: uses-name, target: tu_truy_van, min: 2
  - kind: uses-call, target: tinh_idf, min: 1
  # Da thu that (trich nguyen ham _dem cua kiem-ast.ts vao mot file rieng,
  # chay qua python3 TREN CHINH van ban solution da trich tu file nay) -- ket
  # qua [2, 1] cho hai luat theo dung thu tu khai bao o tren.
  #
  # GOTCHA "boilerplate-threshold-masking": 'tu_truy_van' KHONG chi duoc doc
  # o cho trong dau -- no CON duoc doc mot lan NUA trong chinh dinh nghia
  # diem_tf_idf, o dong "for t in tu_truy_van" (dong nay la boilerplate CO
  # SAN, khong phai cho trong). Neu chi dat min=1, mot mutant dien SAI cho
  # trong dau (vi du dien "kho" thay vi "tu_truy_van", lam diem_tf_tho luon
  # tra ve mot con so KHAC voi y nghia that) van qua duoc vi con lai 1 lan
  # doc 'tu_truy_van' trong diem_tf_idf -- da dat min=2 (TONG THAT) de chan
  # dung truong hop nay.
  # tinh_idf=1: CHI mot lan GOI THAT trong toan bo solution, dung o cho trong
  # hai (ben trong diem_tf_idf). Ham tinh_idf khong tu goi lai chinh no.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua ham
  # _dem de xac nhan, khong doan tay): dien "tinh_idf(t, doan)" vao cho trong
  # dau ("for t in tinh_idf(t, doan)" trong diem_tf_tho) VA dien "tu_truy_van"
  # vao cho trong hai ("dem_tan_so(t, doan) * tu_truy_van" trong diem_tf_idf,
  # ma tham so cua ham nay THAT SU co ten 'tu_truy_van') -- ket qua AST van
  # la [2, 1] Y HET ban dung (tong so lan doc 'tu_truy_van' va tong so lan
  # goi 'tinh_idf' khong doi, chi doi VI TRI). Static KHONG bat duoc mutant
  # nay.
  # Mutant nay BI BAT boi tier 'run': o cho trong dau, bieu thuc
  # "tinh_idf(t, doan)" trong "for t in tinh_idf(t, doan)" doc bien 't' NGAY
  # TRUOC KHI 't' duoc gan boi chinh vong lap do (iterable cua for-clause
  # DAU TIEN trong mot generator expression duoc tinh trong scope BAO QUANH,
  # truoc khi bien vong lap ton tai) -- da tu chay THAT mutant nay qua
  # python3, xac nhan no nem NameError: name 't' is not defined ngay khi
  # diem_tf_tho() duoc goi -- bi chan boi tier 'run', doc lap voi static.
  #
  # Da ra soat GOTCHA #6 cho luat uses-call (tinh_idf): mot cheat co the goi
  # tinh_idf(kho, t) (dao nguoc thu tu doi so) ma van dem la 1 lan goi hop le
  # -- da tu kiem chung bang tay: goi nhu vay lam dem_so_doan_chua_tu(kho, t)
  # so sanh SAI kieu (kho la list, khong bao gio == mot phan tu cua
  # t.lower().split()), khien idf luon tra ve 0.0 cho MOI tu -- nhung day
  # KHONG lam qua duoc bai, vi test da assert TRUC TIEP tren gia tri so
  # "diem_idf == [2.0794, 0.0, 0.0, 0.9808, ...]" (khac toan bo so 0) VA rieng
  # tren "diem_mot_tu == 2.0794" -- ca hai deu that bai neu idf luon la 0.0,
  # doc lap hoan toan voi luat static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[1, 0, 0, 1, 1, 0, 2, 0\\]\\n\\[2\\.0794, 0\\.0, 0\\.0, 0\\.9808, 0\\.9808, 0\\.0, 1\\.9617, 0\\.0\\]\\n6\\n0\\nFalse\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`xep_hang_tho[0] = 6` nhưng `xep_hang_idf[0] = 0` — hai cách xếp hạng chọn
hai đoạn khác nhau, trên cùng dữ liệu, chỉ vì một cách có trọng số theo độ
hiếm còn cách kia thì không. Bài sau thêm hai cải tiến cuối để có công thức
BM25 đầy đủ: bão hoà và chuẩn hoá độ dài.
::::

::::reflect{#nghi-lai}
TF-IDF không phải một công thức bí ẩn — nó là TF (bài `1`) nhân với IDF (bài
`2`), cộng dồn qua mọi từ khoá trong truy vấn. Nhưng phép nhân đơn giản đó đã
đủ để lật ngược một xếp hạng: một từ HIẾM xuất hiện `1` lần có thể quan trọng
hơn một từ PHỔ BIẾN xuất hiện `2` lần, đo được bằng số cụ thể
(`2,0794 > 1,9617`). TF-IDF vẫn còn hai lỗ hổng chưa sửa: nó KHÔNG bão hoà
(một từ lặp `100` lần vẫn cộng dồn tuyến tính, không có giới hạn), và nó
KHÔNG chuẩn hoá theo độ dài đoạn văn (đúng lỗ hổng bài `1` đã đo cho TF thô —
TF-IDF vẫn kế thừa nó, vì `tf(t,d)` bên trong công thức vẫn là TF thô chưa
sửa). Bài sau vá cả hai lỗ hổng đó cùng lúc: công thức BM25 đầy đủ.
::::

::::checkpoint{mastery=0.85}
::::
