---
id: tri-tue-nhan-tao.bm25-va-tim-kiem-tu-khoa.cong-thuc-bm25-day-du
title: "Công thức BM25 đầy đủ: bão hoà và chuẩn hoá độ dài"
summary: "diem_bm25_mot_tu(tf, idf_tu, do_dai_doan, do_dai_trung_binh, k1, b) = idf_tu * (tf*(k1+1)) / (tf + k1*(1-b+b*do_dai_doan/do_dai_trung_binh)), voi k1=1.5, b=0.75, do_dai_trung_binh=12.625 (do dai TRUNG BINH cua kho). BAO HOA (do dai doan = do dai trung binh, idf=1.0): tf=1 cho diem 1,0; tf=2 cho 1,4286; tf=5 cho 1,9231; tf=10 cho 2,1739; tf=100 cho 2,4631 -- tang tu tf=1 len tf=2 them 0,4286, nhung tang tu tf=10 len tf=100 (them 90) chi them 0,2892: cang tf cao cang tang CHAM, tien toi tran (k1+1)=2,5. CHUAN HOA DO DAI (tf=3 co dinh, idf=1.0): doan dai = do dai trung binh cho diem 1,6667; doan dai GAP DOI trung binh cho diem 1,3333 -- CUNG mot tf nhung doan DAI HON bi phat, diem THAP hon."
locale: vi
track: tri-tue-nhan-tao
module: bm25-va-tim-kiem-tu-khoa
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.cong-thuc-bm25-day-du]
requires: [ai.ghep-tf-idf-xep-hang-doan]
concepts: [ai.cong-thuc-bm25-day-du]
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
Bài trước chốt lại: TF-IDF vẫn còn hai lỗ hổng — không bão hoà, không chuẩn
hoá độ dài. Bài này vá cả hai cùng lúc, bằng đúng hai tham số mà BM25 nổi
tiếng vì chúng: `k1` và `b`.
::::

::::explain{#hai_cai_tien_bm25}
**Cải tiến thứ nhất — BÃO HOÀ, qua tham số `k1`.** TF-IDF cộng dồn `tf(t,d)`
TUYẾN TÍNH — một từ xuất hiện `100` lần đóng góp gấp đúng `100` lần một từ
xuất hiện `1` lần. Nhưng trực giác đúng lại khác: từ `1` lần lên `2` lần,
một đoạn văn "nói về" từ đó rõ ràng hơn hẳn; nhưng từ `50` lần lên `100`
lần, đoạn văn không "nói về" nó gấp đôi — nó chỉ đơn giản LẶP LẠI nhiều hơn.
BM25 thay `tf(t,d)` bằng một dạng BÃO HOÀ:

```
tf * (k1 + 1) / (tf + k1)
```

Khi `tf` tăng, biểu thức này tăng CHẬM DẦN, tiến tới một TRẦN là `k1 + 1`,
không bao giờ vượt qua. `k1` (thường chọn `1,5`) điều khiển bão hoà nhanh
hay chậm.

**Cải tiến thứ hai — CHUẨN HOÁ ĐỘ DÀI, qua tham số `b`.** Bài `1` đã đo: một
đoạn DÀI hơn tự nhiên có TF cao hơn, dù không liên quan hơn. BM25 sửa đúng
chỗ đó bằng một **hệ số chuẩn hoá độ dài**:

```
he_so_chuan_hoa_do_dai = 1 - b + b * (do_dai_doan / do_dai_trung_binh)
```

với `do_dai_trung_binh` là độ dài TRUNG BÌNH của mọi đoạn trong kho. Nếu một
đoạn dài ĐÚNG BẰNG độ dài trung bình, hệ số này bằng `1` (không phạt, không
thưởng). Nếu đoạn DÀI HƠN trung bình, hệ số này LỚN HƠN `1` — và hệ số này
nhân với `k1` rồi CỘNG vào MẪU SỐ của công thức bão hoà ở trên, làm điểm số
GIẢM. `b` (thường chọn `0,75`, nằm trong khoảng `[0,1]`) điều khiển mức độ
phạt: `b=0` nghĩa là không chuẩn hoá gì cả (giống TF-IDF), `b=1` nghĩa là
chuẩn hoá đầy đủ.

**Ghép cả hai vào công thức đầy đủ** cho MỘT từ khoá:

```
bm25(t, d) = idf(t) * (tf(t,d) * (k1+1)) / (tf(t,d) + k1 * he_so_chuan_hoa_do_dai)
```

Điểm BM25 của cả một truy vấn nhiều từ là TỔNG công thức trên qua mọi từ
khoá — giống hệt cách TF-IDF tổng hợp ở bài trước, chỉ khác công thức cho
MỖI từ.
::::

::::example{#do_bao_hoa_va_chuan_hoa_bang_so}
Dùng `k1=1,5`, `b=0,75`, và độ dài trung bình `12,625` (độ dài trung bình
của kho `8` đoạn ở hai bài trước). Để nhìn RIÊNG từng cải tiến, cố định
`idf_tu = 1,0` (loại bỏ ảnh hưởng của độ hiếm, chỉ còn bão hoà và chuẩn hoá):

```python title=readonly
def he_so_chuan_hoa_do_dai(do_dai_doan, do_dai_trung_binh, b):
    return 1 - b + b * (do_dai_doan / do_dai_trung_binh)


def diem_bm25_mot_tu(tf, idf_tu, do_dai_doan, do_dai_trung_binh, k1, b):
    chuan_hoa = he_so_chuan_hoa_do_dai(do_dai_doan, do_dai_trung_binh, b)
    tu_so = tf * (k1 + 1)
    mau_so = tf + k1 * chuan_hoa
    return idf_tu * (tu_so / mau_so)


K1 = 1.5
B = 0.75
AVGDL = 12.625

# BAO HOA: do dai doan = do dai trung binh (khong bi phat/thuong do dai),
# chi doi tf.
diem_tf1 = diem_bm25_mot_tu(1, 1.0, AVGDL, AVGDL, K1, B)
diem_tf2 = diem_bm25_mot_tu(2, 1.0, AVGDL, AVGDL, K1, B)
diem_tf5 = diem_bm25_mot_tu(5, 1.0, AVGDL, AVGDL, K1, B)
diem_tf10 = diem_bm25_mot_tu(10, 1.0, AVGDL, AVGDL, K1, B)
diem_tf100 = diem_bm25_mot_tu(100, 1.0, AVGDL, AVGDL, K1, B)

# CHUAN HOA DO DAI: tf co dinh = 3, chi doi do dai doan.
diem_ngan = diem_bm25_mot_tu(3, 1.0, AVGDL, AVGDL, K1, B)
diem_dai = diem_bm25_mot_tu(3, 1.0, 2 * AVGDL, AVGDL, K1, B)

print(round(diem_tf1, 4))
print(round(diem_tf2, 4))
print(round(diem_tf5, 4))
print(round(diem_tf10, 4))
print(round(diem_tf100, 4))
print(round(diem_ngan, 4))
print(round(diem_dai, 4))
print(diem_ngan > diem_dai)
```

```text title=readonly
1.0
1.4286
1.9231
2.1739
2.4631
1.6667
1.3333
True
```

**Bão hoà**: từ `tf=1` (điểm `1,0`) lên `tf=2` (điểm `1,4286`), điểm tăng
thêm `0,4286`. Từ `tf=10` (điểm `2,1739`) lên `tf=100` (điểm `2,4631`) — TĂNG
THÊM `90` LẦN xuất hiện — điểm chỉ tăng thêm `0,2892`, ÍT HƠN mức tăng từ
`tf=1` lên `tf=2`. Điểm số tiến dần tới trần `k1+1 = 2,5`, không bao giờ vượt
qua, dù `tf` có lớn tới đâu.

**Chuẩn hoá độ dài**: với CÙNG `tf=3`, một đoạn có độ dài ĐÚNG BẰNG trung
bình cho điểm `1,6667`; một đoạn DÀI GẤP ĐÔI trung bình — vẫn cùng `tf=3` —
chỉ cho điểm `1,3333`, THẤP HƠN. `diem_ngan > diem_dai` — cùng một số lần
xuất hiện, đoạn NGẮN hơn (tương đối so với trung bình kho) được điểm CAO
HƠN. Đây chính là điều TF-IDF (bài trước) không làm được.
::::

::::predict{#doan_bm25_bao_hoa_va_chuan_hoa commitOnce}
Xét đúng ví dụ trên: từ `tf=10` (điểm `2,1739`) lên `tf=100` (điểm `2,4631`)
— tăng `90` lần xuất hiện nhưng điểm chỉ tăng thêm `0,2892`.

**Trước khi chạy thử**, bạn đoán: nếu tiếp tục tăng `tf` lên `1000`, điểm số
sẽ tăng thêm một lượng LỚN HƠN hay NHỎ HƠN so với mức tăng từ `tf=10` lên
`tf=100`?

:::opt{correct}
NHỎ HƠN — công thức bão hoà tiến dần tới trần `k1+1 = 2,5` và không bao giờ
vượt qua; càng gần trần, mỗi lần tăng `tf` càng đóng góp ÍT HƠN, bất kể mức
tăng tuyệt đối của `tf` lớn tới đâu
:::

:::opt
LỚN HƠN — vì `tf=1000` lớn hơn nhiều so với `tf=100`, và công thức vẫn nhân
`tf` lên nên số càng lớn thì kết quả càng phải tăng nhanh hơn
::why
Gần đúng ở việc `tf` THẬT SỰ xuất hiện trong công thức (`tf * (k1+1)`) —
quan sát về việc `tf` có mặt trong tử số đúng.

Chỗ lệch: `tf` cũng xuất hiện Ở MẪU SỐ (`tf + k1*chuan_hoa`), và khi `tf`
rất lớn, cả tử số lẫn mẫu số đều được `tf` chi phối gần như hoàn toàn — tỉ
số `tf*(k1+1) / tf` tiến gần `(k1+1)`, một HẰNG SỐ không phụ thuộc `tf` nữa.
Đây chính là cơ chế bão hoà: `tf` càng lớn, ảnh hưởng của việc TĂNG THÊM
`tf` càng nhỏ đi.
::
:::

:::opt
Bằng nhau — vì công thức có `tf` ở cả tử số và mẫu số nên hai ảnh hưởng đó
luôn triệt tiêu nhau, khiến mức tăng luôn cố định bất kể `tf` đang ở đâu
::why
Gần đúng ở việc `tf` THẬT SỰ xuất hiện đối xứng ở cả tử số và mẫu số — quan
sát về cấu trúc công thức đó đúng.

Chỗ lệch: "xuất hiện ở cả hai vế" không có nghĩa là "mức tăng luôn cố định"
— chính vì mẫu số CỘNG THÊM một hằng số `k1*chuan_hoa` không phụ thuộc
`tf`, tỉ lệ `tf / (tf + hang_so)` thay đổi THEO `tf`: ở `tf` nhỏ, hằng số đó
chiếm phần LỚN của mẫu số nên tỉ lệ tăng nhanh; ở `tf` lớn, hằng số đó trở
nên KHÔNG ĐÁNG KỂ so với `tf`, nên tỉ lệ gần như không đổi nữa — đây chính
là bão hoà, một mức tăng NHỎ DẦN, không phải một mức tăng cố định.
::
:::
::::

::::code{#viet_diem_bm25_mot_tu}
Hoàn thiện `he_so_chuan_hoa_do_dai` (công thức chuẩn hoá độ dài) và
`diem_bm25_mot_tu` (ghép bão hoà, chuẩn hoá độ dài và IDF thành điểm BM25 cho
một từ khoá).

```python title=starter
def he_so_chuan_hoa_do_dai(do_dai_doan, do_dai_trung_binh, b):
    return 1 - b + ___                                        # b * (do_dai_doan / do_dai_trung_binh)


def diem_bm25_mot_tu(tf, idf_tu, do_dai_doan, do_dai_trung_binh, k1, b):
    chuan_hoa = he_so_chuan_hoa_do_dai(do_dai_doan, do_dai_trung_binh, b)
    tu_so = tf * (k1 + 1)
    mau_so = tf + k1 * chuan_hoa
    return idf_tu * (___)                                      # tu_so / mau_so


K1 = 1.5
B = 0.75
AVGDL = 12.625

diem_tf1 = diem_bm25_mot_tu(1, 1.0, AVGDL, AVGDL, K1, B)
diem_tf2 = diem_bm25_mot_tu(2, 1.0, AVGDL, AVGDL, K1, B)
diem_tf5 = diem_bm25_mot_tu(5, 1.0, AVGDL, AVGDL, K1, B)
diem_tf10 = diem_bm25_mot_tu(10, 1.0, AVGDL, AVGDL, K1, B)
diem_tf100 = diem_bm25_mot_tu(100, 1.0, AVGDL, AVGDL, K1, B)

diem_ngan = diem_bm25_mot_tu(3, 1.0, AVGDL, AVGDL, K1, B)
diem_dai = diem_bm25_mot_tu(3, 1.0, 2 * AVGDL, AVGDL, K1, B)

print(round(diem_tf1, 4))
print(round(diem_tf2, 4))
print(round(diem_tf5, 4))
print(round(diem_tf10, 4))
print(round(diem_tf100, 4))
print(round(diem_ngan, 4))
print(round(diem_dai, 4))
print(diem_ngan > diem_dai)
```

```python title=solution
def he_so_chuan_hoa_do_dai(do_dai_doan, do_dai_trung_binh, b):
    return 1 - b + b * (do_dai_doan / do_dai_trung_binh)


def diem_bm25_mot_tu(tf, idf_tu, do_dai_doan, do_dai_trung_binh, k1, b):
    chuan_hoa = he_so_chuan_hoa_do_dai(do_dai_doan, do_dai_trung_binh, b)
    tu_so = tf * (k1 + 1)
    mau_so = tf + k1 * chuan_hoa
    return idf_tu * (tu_so / mau_so)


K1 = 1.5
B = 0.75
AVGDL = 12.625

diem_tf1 = diem_bm25_mot_tu(1, 1.0, AVGDL, AVGDL, K1, B)
diem_tf2 = diem_bm25_mot_tu(2, 1.0, AVGDL, AVGDL, K1, B)
diem_tf5 = diem_bm25_mot_tu(5, 1.0, AVGDL, AVGDL, K1, B)
diem_tf10 = diem_bm25_mot_tu(10, 1.0, AVGDL, AVGDL, K1, B)
diem_tf100 = diem_bm25_mot_tu(100, 1.0, AVGDL, AVGDL, K1, B)

diem_ngan = diem_bm25_mot_tu(3, 1.0, AVGDL, AVGDL, K1, B)
diem_dai = diem_bm25_mot_tu(3, 1.0, 2 * AVGDL, AVGDL, K1, B)

print(round(diem_tf1, 4))
print(round(diem_tf2, 4))
print(round(diem_tf5, 4))
print(round(diem_tf10, 4))
print(round(diem_tf100, 4))
print(round(diem_ngan, 4))
print(round(diem_dai, 4))
print(diem_ngan > diem_dai)
```

```python title=test
assert round(diem_tf1, 4) == 1.0, f"tf=1 phai cho diem 1,0 -- dang ra {diem_tf1}"
assert round(diem_tf2, 4) == 1.4286, f"tf=2 phai cho diem 1,4286 -- dang ra {diem_tf2}"
assert round(diem_tf5, 4) == 1.9231, f"tf=5 phai cho diem 1,9231 -- dang ra {diem_tf5}"
assert round(diem_tf10, 4) == 2.1739, f"tf=10 phai cho diem 2,1739 -- dang ra {diem_tf10}"
assert round(diem_tf100, 4) == 2.4631, f"tf=100 phai cho diem 2,4631 -- dang ra {diem_tf100}"
assert diem_tf100 < 2.5, "diem BM25 khong bao gio duoc vuot qua tran k1+1=2,5, du tf co lon toi dau"
assert (diem_tf100 - diem_tf10) < (diem_tf2 - diem_tf1), "muc tang tu tf=10 len tf=100 phai NHO HON muc tang tu tf=1 len tf=2 -- day chinh la bao hoa"

assert round(diem_ngan, 4) == 1.6667, f"doan dai = trung binh phai cho diem 1,6667 -- dang ra {diem_ngan}"
assert round(diem_dai, 4) == 1.3333, f"doan dai gap doi trung binh phai cho diem 1,3333 -- dang ra {diem_dai}"
assert diem_ngan > diem_dai, "cung tf nhung doan DAI HON (so voi trung binh) phai bi phat, diem THAP hon"

# kiem tra truc tiep he_so_chuan_hoa_do_dai tren gia tri bien
assert he_so_chuan_hoa_do_dai(AVGDL, AVGDL, B) == 1.0, "do dai dung bang trung binh phai cho he so chuan hoa = 1.0 (khong phat, khong thuong)"
assert round(he_so_chuan_hoa_do_dai(2 * AVGDL, AVGDL, B), 4) == 1.75, f"do dai gap doi trung binh phai cho he so 1,75 -- dang ra {he_so_chuan_hoa_do_dai(2 * AVGDL, AVGDL, B)}"

# bien: tf=0 (tu khong xuat hien) phai cho diem 0, khong loi
assert diem_bm25_mot_tu(0, 1.0, AVGDL, AVGDL, K1, B) == 0.0, "tf=0 phai cho diem BM25 = 0.0"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `he_so_chuan_hoa_do_dai`) là phần được CỘNG THÊM vào `1 - b` — tỉ lệ giữa độ dài đoạn văn và độ dài trung bình, NHÂN với `b`. Chỗ hai (trong `diem_bm25_mot_tu`) là biểu thức bên trong `idf_tu * (___)` — tỉ số giữa `tu_so` (đã tính ở dòng trên, phần bão hoà) và `mau_so` (đã tính ở dòng trên, mẫu số có chuẩn hoá độ dài).
- kind: strategy
  body: 'Chỗ đầu: `b * (do_dai_doan / do_dai_trung_binh)` — nhân `b` với tỉ lệ độ dài. Chỗ hai: `tu_so / mau_so` — chia phần bão hoà cho mẫu số đã chuẩn hoá.'
- kind: one-line
  body: 'Chỗ đầu là `b * (do_dai_doan / do_dai_trung_binh)`, chỗ hai là `tu_so / mau_so`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai NHAN 'b' voi ti le do_dai_doan/do_dai_trung_binh (dung ca hai toan tu '*' va '/'); VA cho trong hai phai CHIA tu_so cho mau_so (dung toan tu '/')
  requireAst:
  - kind: uses-operator, target: "*", min: 5
  - kind: uses-operator, target: "/", min: 2
  # Da thu that (trich nguyen ham _dem cua kiem-ast.ts vao mot file rieng,
  # chay qua python3 TREN CHINH van ban solution da trich tu file nay) -- ket
  # qua [5, 2] cho hai luat theo dung thu tu khai bao o tren.
  #
  # 🔴 GOTCHA "boilerplate-threshold-masking" o muc do MANH (da do THAT, khong
  # doan tay): '*' xuat hien 5 LAN trong toan bo solution, KHONG PHAI 1 -- (1)
  # "b * (...)" o cho trong dau, (2) "tf * (k1+1)" co san (boilerplate cua
  # tu_so), (3) "k1 * chuan_hoa" co san (boilerplate cua mau_so), (4)
  # "idf_tu * (tu_so/mau_so)" co san BAO QUANH ca cho trong hai, (5)
  # "2 * AVGDL" trong loi goi tinh diem_dai o cuoi bai. Neu chi dat min=1
  # (ngay tho), mot mutant BO HOAN TOAN phep nhan o cho trong dau (vi du dien
  # "do_dai_doan / do_dai_trung_binh" khong nhan voi b) van qua duoc vi con
  # lai 4 lan '*' khac trong solution -- day la vi du RO RANG NHAT cua GOTCHA
  # nay tu truoc den gio trong quest. Da dat min=5 (TONG THAT, do bang cong
  # cu) de CHI ban dung moi dat.
  # '/' xuat hien 2 lan: (1) "do_dai_doan / do_dai_trung_binh" o cho trong
  # dau, (2) "tu_so / mau_so" o cho trong hai -- khong co boilerplate '/' nao
  # khac, nen min=2 (TONG THAT) khop dung ca hai cho trong.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua ham
  # _dem de xac nhan, khong doan tay): dien "tu_so / mau_so" vao cho trong
  # dau ("return 1 - b + tu_so / mau_so" trong he_so_chuan_hoa_do_dai) VA
  # dien "b * (do_dai_doan / do_dai_trung_binh)" vao cho trong hai ("return
  # idf_tu * (b * (do_dai_doan / do_dai_trung_binh))" trong diem_bm25_mot_tu)
  # -- ket qua AST van la [5, 2], Y HET ban dung (tong so lan '*' va '/'
  # khong doi, chi doi VI TRI). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong he_so_chuan_hoa_do_dai(do_dai_doan,
  # do_dai_trung_binh, b), hai ten 'tu_so' va 'mau_so' KHONG TON TAI (day la
  # bien CUC BO cua diem_bm25_mot_tu, mot ham khac) -- da tu chay THAT mutant
  # nay qua python3, xac nhan no nem NameError: name 'tu_so' is not defined
  # ngay khi he_so_chuan_hoa_do_dai() duoc goi lan dau (tu ben trong
  # diem_bm25_mot_tu) -- bi chan boi tier 'run', doc lap voi static.
  # Da ra soat GOTCHA #6: khong co luat uses-call nao o day (chi uses-operator),
  # nen khong ap dung.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^1\\.0\\n1\\.4286\\n1\\.9231\\n2\\.1739\\n2\\.4631\\n1\\.6667\\n1\\.3333\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`diem_tf100 = 2,4631`, không bao giờ vượt trần `2,5` — bão hoà hoạt động.
`diem_ngan = 1,6667 > diem_dai = 1,3333` — chuẩn hoá độ dài hoạt động. Bài
sau áp dụng công thức đầy đủ này để xếp hạng CẢ MỘT kho tài liệu, đối chiếu
trực tiếp với TF-IDF thô.
::::

::::reflect{#nghi-lai}
BM25 không phải một công thức hoàn toàn mới — nó là TF-IDF (bài `3`), với
`tf(t,d)` được thay bằng một dạng BÃO HOÀ (`k1`) và mẫu số được nhân thêm
một HỆ SỐ CHUẨN HOÁ ĐỘ DÀI (`b`). Hai tham số đó không phải phép thuật: `k1`
trả lời câu hỏi "lặp lại một từ nhiều lần có nên tính hoài không giới hạn
không?" (câu trả lời BM25 chọn: không — có một trần), còn `b` trả lời câu
hỏi "đoạn văn dài hơn trung bình kho có nên bị thiệt không?" (câu trả lời
BM25 chọn: có, tuỳ mức `b`). Bài sau dùng đúng công thức này để xếp hạng
TOÀN BỘ một kho tài liệu cho một truy vấn nhiều từ, và so trực tiếp với
TF-IDF thô — tìm một trường hợp cụ thể nơi hai cách xếp hạng cho ra kết quả
khác nhau.
::::

::::checkpoint{mastery=0.85}
::::
