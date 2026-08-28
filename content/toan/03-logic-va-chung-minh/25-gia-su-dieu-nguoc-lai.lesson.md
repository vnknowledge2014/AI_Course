---
id: toan.logic-va-chung-minh.gia-su-dieu-nguoc-lai
title: Giả sử điều ngược lại
summary: Câu không có vế trước thì không có gì để nắm — trừ phủ định của chính nó. Giả sử phủ định ấy đúng, đi tới khi cầm được hai câu ngược nhau, và điều giả sử tự sập.
locale: vi
track: toan
module: logic-va-chung-minh
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [logic.proof-by-contradiction]
requires: [logic.proof-by-contrapositive, logic.direct-proof, logic.use-definition, logic.finite-check-not-proof, logic.contradiction, logic.tautology, logic.truth-table, logic.equivalence, logic.contrapositive, logic.implication, logic.vacuous-truth, logic.negation, logic.truth-value, logic.counterexample, logic.for-all, logic.exists, logic.quantifier-negation, logic.conjunction, logic.not, logic.or, math.multiplication, math.multiply-commutative, math.expand-brackets, math.factor-common, math.exponent, math.remainder, core.boolean, core.variable, core.number-literal, core.arithmetic, core.list, core.list-comprehension, core.list-index, core.len, core.builtin-function, core.function-def, core.function-call, core.function-parameter, core.function-return, core.accumulator, core.print-variable, ctrl.if, ctrl.for-each, ctrl.for-range, ctrl.comparison]
concepts: [logic.phan-chung, logic.gia-su-phu-dinh, logic.suy-ra-hang-sai]
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
Câu này không có chữ "nếu" nào để mình nắm. Nhưng phủ định của nó thì có.
::::

::::explain{#khong-co-ve-truoc-thi-nam-vao-dau}
Bài trước dừng ở một câu không cho bạn chỗ đặt tay:

> Không có phân số nào mà đem nhân với chính nó lại ra đúng 2.

Cả hai lối đã có đều mở đầu bằng cùng một việc — **nhặt vế trước lên làm giả
thiết**. Lối trực tiếp của bài 23 nhặt nó rồi đi thẳng; lối phản đảo của bài 24
đổi chỗ hai vế rồi mới nhặt. Câu trên không có vế trước nào để nhặt, nên cả hai
lối cùng đứng im ở dòng đầu.

Nhưng có một thứ mà **mọi** mệnh đề đều có, kể cả câu này: **phủ định của nó**.
Bài 3 dựng phép phủ định cho một câu bất kỳ, không đòi câu ấy phải có hình dạng
gì. Nên viết ra được ngay:

> Có một phân số mà đem nhân với chính nó ra đúng 2.

Và câu vừa viết **có** thứ mà câu gốc không có: nó cho bạn một vật để cầm. Nó
nói có một phân số như thế; đặt tên cho nó rồi làm việc với cái tên — đúng cái
việc bài 22 đã dạy khi mở chữ "chẵn" ra thành `n = 2k`.

Giờ đến chỗ trả giá. Nếu bạn **giả sử** câu phủ định ấy đúng và đi tới cùng, bạn
được gì? Bài 2 chốt: một mệnh đề chỉ có hai giá trị, Đ hoặc S, không có cửa thứ
ba. Nên câu gốc và phủ định của nó không bao giờ cùng Đ, cũng không bao giờ cùng
S — hễ chứng minh được phủ định **sai** là câu gốc **đúng**.

Vậy chỉ còn một câu hỏi: làm sao chứng minh được một câu là sai?

Bài 7 để sẵn câu trả lời từ mười tám bài trước. Ở đó bạn dựng bảng cho *"Nam đã
nộp quỹ **và** Nam chưa nộp quỹ"* và thấy cột kết quả **toàn S** — không dòng
nào cứu được, không sổ sách nào cứu được. Bài 7 gọi tên nó: câu **hằng sai**.

Ghép hai mảnh lại thì ra cả bài hôm nay:

> Nếu từ điều bạn giả sử mà suy ra được một câu **hằng sai**, thì điều bạn giả
> sử ấy không thể đúng. Vì một câu đúng chỉ dẫn tới câu đúng, mà câu hằng sai
> thì không có tình huống nào trên đời làm nó đúng được.

Và câu hằng sai dễ nhận nhất chính là câu bài 7 dựng: **"P và không P"** — cầm
được hai câu ngược hẳn nhau cùng lúc.
::::

::::example{#so-quy-va-mot-thang-nao-do}
Trước khi mang cỗ máy ấy sang câu về phân số, chạy nó một lượt ở chỗ dễ nhìn.

Quỹ CLB cờ vua lớp 6A: sổ ghi sáu tháng, mỗi tháng một khoản.

| tháng | 1 | 2 | 3 | 4 | 5 | 6 |
|---|---|---|---|---|---|---|
| thu được (đồng) | 40 000 | 25 000 | 60 000 | 15 000 | 30 000 | 45 000 |

Cộng sáu khoản ấy lại thì được 40 000 + 25 000 + 60 000 + 15 000 + 30 000 + 45 000 = 215 000 đồng,
nên **tổng quỹ sáu tháng là 215 000 đồng** — đúng con số bạn sẽ gặp lại ở cuối track.

Câu cần chứng minh:

> **Có ít nhất một tháng thu được quá 35 000 đồng.**

**Nhịp 1 — viết phủ định ra, rồi giả sử nó đúng.**

Câu trên là câu "tồn tại" của bài 18. Bài 20 đã cho luật phủ định nó: ¬∃ thành
∀¬. Nên phủ định là *"mọi tháng đều thu **không quá** 35 000 đồng"*.

Giả sử điều đó đúng.

**Nhịp 2 — đi tới cùng, chỉ bằng luật đã có.**

Có sáu tháng, và điều giả sử nói tháng nào cũng không quá 35 000. Sáu khoản, mỗi
khoản nhiều nhất 35 000, thì cộng lại nhiều nhất là 6 × 35 000 = 210 000 đồng.

Nên điều giả sử kéo tới: **tổng quỹ không quá 210 000 đồng.**

Nhưng sổ đang mở ra trước mặt, và tổng của nó là 215 000. Mà 215 000 lớn hơn
210 000, nên **không phải là "tổng quỹ không quá 210 000 đồng"**.

**Nhịp 3 — chỉ vào câu hằng sai, rồi kết.**

Đặt hai câu vừa cầm được cạnh nhau:

```text
  (1)  tổng quỹ không quá 210 000            ← do điều giả sử
  (2)  KHÔNG phải (tổng quỹ không quá 210 000) ← do cuốn sổ
```

Ghép bằng chữ "và" thì được đúng hình dạng "P và không P" của bài 7: một câu
hằng sai. Nên điều giả sử không đúng được. Nên **có ít nhất một tháng thu quá
35 000 đồng.**

Mở sổ ra xem thì đúng thật — tháng 3 thu 60 000. Nhưng chú ý một chi tiết đáng
tiền: **lập luận trên không nhìn tháng nào cả.** Nó chỉ nhìn con số tổng. Nó
không nói được tháng nào, không nói được có mấy tháng như thế — nó chỉ nói *có*.

Và phải nói thẳng chỗ này: sổ có sáu dòng, nên câu ấy dò tay cũng ra. Ta dựng nó
ở đây **để nhìn cho rõ hình dạng ba nhịp**, không phải vì không còn cách nào
khác. Chỗ bắt buộc phải dùng hình dạng ấy nằm ngay dưới đây.
::::

::::explain{#dat-canh-phan-dao}
Đặt tên cho lối vừa đi:

> **Chứng minh bằng phản chứng** — giả sử **phủ định** của điều cần chứng minh
> là đúng, đi tới khi cầm được một câu **hằng sai** (thường là "P và không P"),
> rồi kết: điều giả sử sai, nên điều cần chứng minh đúng.

Lối này va vào lối bài 24 rất dễ, vì cả hai đều có chữ "phủ định" trong tên.
Chúng khác nhau ở gần như mọi chỗ còn lại:

| | phản đảo (bài 24) | phản chứng (hôm nay) |
|---|---|---|
| dùng được cho | câu có dạng "nếu… thì" | câu bất kỳ |
| việc phải làm | đổi chỗ hai vế **rồi** phủ định cả hai, được một câu MỚI | giữ nguyên câu, thêm **phủ định của nó** vào kho giả thiết |
| chứng minh xong cái gì | câu mới ấy | một câu hằng sai |
| quyền kết luận đến từ | trùng bảng — bài 14 | hằng sai — bài 7 |

Một chỗ trượt nữa, và nó tốn thời gian chứ không tốn điểm: **dừng đúng lúc.**
Phản chứng chỉ kết thúc khi bạn cầm được hai câu **ngược hẳn nhau**. Suy ra một
câu nghe lạ tai thì chưa phải. Nếu từ điều giả sử bạn suy ra "tử số của phân số
ấy lớn hơn một triệu", đó mới chỉ là một câu bất ngờ — nó vẫn có thể đúng, và
bảng của nó vẫn có dòng ghi Đ. Chưa hằng sai thì chưa có gì để kết.
::::

::::example{#khong-phan-so-nao-nhan-voi-chinh-no-ra-hai}
Giờ câu đã treo từ cuối bài trước.

> Không có phân số nào mà đem nhân với chính nó lại ra đúng 2.

Trước khi chứng minh, nhìn xem vì sao nó khó chịu. Thử vài phân số:

- `7/5 × 7/5 = 49/25`, mà 2 viết trên cùng cái thước ấy là `50/25 = 2`. Hụt một
  phần hai mươi lăm.
- `99/70 × 99/70 = 9801/4900`, mà `9800/4900 = 2`. Hụt một phần bốn nghìn chín
  trăm — nhỏ hơn nhiều, nhưng vẫn hụt.

Thử mãi thế này thì rơi đúng vào chỗ bài 21 đã đóng: máy chỉ nói được về những
phân số nó đã thử, mà phân số thì thử không bao giờ hết. Nên phải chứng minh.

**Nhịp 1 — viết phủ định ra, rồi giả sử nó đúng.**

Giả sử **có** một phân số như thế. Gọi tử của nó là `a` và mẫu là `b`, với `a`
và `b` là số nguyên và `b` khác 0.

Còn một việc trước khi đi tiếp, và nó là chìa khoá của cả bài: **rút gọn phân số
ấy hết cỡ** trước khi đặt tên. T2.1 bài 33 đã dựng phép rút gọn — chia cả tử lẫn
mẫu cho cùng một số thì phân số không đổi chỗ trên thanh số. Rút gọn hết cỡ
nghĩa là không còn chia chung được nữa; nói riêng, `a` và `b` **không cùng
chẵn** (còn cùng chẵn thì còn chia đôi cả hai được, tức là chưa hết cỡ).

Ghi câu ấy lại, vì lát nữa còn dùng:

```text
  (1)  a và b KHÔNG cùng chẵn
```

**Nhịp 2 — đi tới cùng, chỉ bằng luật đã có.**

Điều giả sử nói phân số ấy nhân với chính nó ra 2:

```text
  (a/b) × (a/b) = 2
```

Nhân một phân số với một phân số là nhân tử với tử, mẫu với mẫu, nên vế trái là
`(a × a)/(b × b)`. Nhân cả hai vế với `b × b` để dọn mẫu đi:

```text
  a × a = 2 × (b × b)
```

Vế phải là 2 nhân một số nguyên. Đó đúng là dạng mà **chiều gói lại** của bài 22
nhận cho chữ "chẵn". Nên `a × a` chẵn.

Và bài 24 vừa chứng minh xong đúng câu cần ở đây: *nếu `n²` chẵn thì `n` chẵn.*
Áp nó cho `n = a`:

```text
  a chẵn
```

Mở định nghĩa ra (bài 22): có một số nguyên `c` sao cho `a = 2c`. Thay vào:

```text
  (2c) × (2c) = 2 × (b × b)
  4 × (c × c) = 2 × (b × b)
```

Chia cả hai vế cho 2:

```text
  2 × (c × c) = b × b
```

Lần này vế trái là 2 nhân một số nguyên, nên `b × b` chẵn — và bài 24 lại nói:

```text
  b chẵn
```

Gom hai dòng vừa rút ra:

```text
  (2)  a chẵn VÀ b chẵn,  tức là  a và b cùng chẵn
```

**Nhịp 3 — chỉ vào câu hằng sai, rồi kết.**

Đặt (1) cạnh (2):

```text
  a và b cùng chẵn   VÀ   KHÔNG phải (a và b cùng chẵn)
```

Đúng hình dạng "P và không P" của bài 7 — cột toàn S, không tình huống nào cứu
được. Nên điều giả sử ở nhịp 1 sai.

Và nếu "có một phân số như thế" là sai thì "không có phân số nào như thế" đúng.
Xong.

Để ý điều lạ mà lối này vừa làm được: bạn vừa nói một câu chắc chắn về **mọi**
phân số trên đời — vô hạn phân số — mà chưa nhân thử phân số nào. Chỗ bài 21
đóng lại, phản chứng mở ra bằng một cửa khác.
::::

::::predict{#doan-cho-va commitOnce}
Byte bắt máy dựng lại đúng cú va của cuốn sổ.

Nhớ vai của máy, vẫn không đổi từ bài 21: nó **không** chứng minh gì. Nó chỉ
cộng sổ và so hai con số — chỗ chứng minh là ba nhịp bạn vừa đọc.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
so_quy = [40000, 25000, 60000, 15000, 30000, 45000]

def tong_so(cot):
    tong = 0
    for x in cot:
        tong = tong + x
    return tong

# Điều giả sử kéo tới đâu: sáu tháng, mỗi tháng không quá 35 000.
gioi_han = 6 * 35000

print(tong_so(so_quy))
print(gioi_han)
print(tong_so(so_quy) <= gioi_han)
```

:::opt{correct}
`215000`, rồi `210000`, rồi `False`
:::

:::opt
`215000`, rồi `210000`, rồi `True`
::why
Gần đúng ở hai dòng đầu, và ở một lối nghĩ rất tự nhiên: đây là *điều giả sử*
của bài, mà đã giả sử thì máy phải chấp nhận cho — nên dòng cuối phải là `True`.

Chỗ lệch: máy không nhận lời giả sử nào cả, nó chỉ cộng sổ rồi so. Và đó chính
là chỗ cả bài xoay quanh — dòng thứ ba là nơi điều giả sử **va vào** cuốn sổ, và
nó ghi `False` đúng ở chỗ va. Nếu dòng ấy ra `True` thì chẳng có câu nào ngược
câu nào, và phản chứng không có gì để kết.
::
:::

:::opt
`215000`, rồi `35000`, rồi `False`
::why
Gần đúng ở chỗ bạn đọc `gioi_han` theo đúng nghĩa mà điều giả sử phát biểu:
giả sử nói về **từng tháng**, và trần của từng tháng đúng là 35 000.

Chỗ lệch nằm ở việc thứ đem so với cuốn sổ không phải một tháng mà là **tổng
sáu tháng**. Muốn so được thì hai bên phải cùng loại, nên trần của từng tháng
phải nhân với số tháng trước đã: 6 × 35 000 = 210 000. Dòng `gioi_han = 6 * 35000`
làm đúng phép nhân ấy, nên nó in ra 210 000 chứ không phải 35 000.
::
:::

:::opt
`210000`, rồi `210000`, rồi `True`
::why
Gần đúng ở chỗ bạn giữ lời giả sử thật nghiêm túc: đã giả sử mọi tháng không quá
35 000 thì cuốn sổ phải theo, nên tổng đọc ra cũng phải nằm dưới trần.

Chỗ lệch là **một lời giả sử không sửa được cuốn sổ**. Danh sách `so_quy` vẫn
nguyên sáu con số ấy, `tong_so` vẫn cộng ra 215 000. Và đó không phải chi tiết
kỹ thuật — nó chính là lý do phản chứng chạy được: bạn được quyền giả sử bất cứ
điều gì, nhưng những sự thật đã có thì vẫn nằm đó, chờ để va vào.
::
:::
::::

::::code{#hai-loi-mot-ket-luan}
Giờ tự tay dựng hai lối đi tới cùng một kết luận, rồi bắt chúng nói giống nhau.

- **Lối phản chứng** chỉ được nhìn con số **tổng**. Nó không mở sổ ra xem tháng
  nào cả — đúng như lập luận ba nhịp bạn vừa đọc.
- **Lối đi săn** thì ngược lại: xét từng tháng một, đúng cách bài 19 bác bỏ một
  câu "với mọi" bằng một phản ví dụ.

Ba chỗ trống:

1. **`tran_tong`** — điều giả sử kéo tới đâu. `so_thang` tháng, mỗi tháng không
   quá `tran`, thì tổng nhiều nhất là bao nhiêu?
2. **`bang_phan_chung`** — điền nốt vế phải của phép so. Hàm đã có sẵn lớp
   `khong` bọc ngoài (phép phủ định của bài 3, viết thành máy): nó trả `True`
   đúng khi tổng thật **vượt** cái trần mà điều giả sử cho phép — tức là đúng
   khi hai câu ngược nhau xuất hiện.
3. **`bang_di_san`** — đi tìm xem có tháng nào thu **quá** `tran` không.

Bài chấm bằng hai cuốn sổ. Cuốn thứ hai — `deu_ba_muoi`, ba tháng đều đúng
30 000 — có mặt để canh đúng một chỗ: ở đó tổng **vừa khít** cái trần, không
hơn một đồng nào, và không tháng nào **vượt** trần. Chữ "quá" và chữ "từ… trở
lên" tách nhau đúng tại chỗ vừa khít ấy, nên viết nhầm một dấu là cuốn sổ ấy báo
ngay.

```python title=starter
# Bài 3 dựng phép phủ định. Hàm dưới chỉ là phép ấy viết thành máy.
def khong(p):
    return not p

so_quy = [40000, 25000, 60000, 15000, 30000, 45000]
deu_ba_muoi = [30000, 30000, 30000]

def tong_so(cot):
    tong = 0
    for x in cot:
        tong = tong + x
    return tong

# Điều giả sử kéo tới đâu: `so_thang` tháng, mỗi tháng không quá `tran`.
def tran_tong(so_thang, tran):
    return ___

# Kết luận rút ra BẰNG PHẢN CHỨNG — chỉ được nhìn TỔNG, không nhìn tháng nào.
def bang_phan_chung(cot, so_thang, tran):
    return khong(tong_so(cot) <= ___)

# Cùng kết luận ấy, kiểm bằng cách xét từng tháng một.
def bang_di_san(cot, tran):
    return ___

print(tran_tong(6, 35000))
print(bang_phan_chung(so_quy, 6, 35000))
print(bang_di_san(so_quy, 35000))
print(bang_phan_chung(deu_ba_muoi, 3, 30000))
print(bang_di_san(deu_ba_muoi, 30000))
```

```python title=solution
# Bài 3 dựng phép phủ định. Hàm dưới chỉ là phép ấy viết thành máy.
def khong(p):
    return not p

so_quy = [40000, 25000, 60000, 15000, 30000, 45000]
deu_ba_muoi = [30000, 30000, 30000]

def tong_so(cot):
    tong = 0
    for x in cot:
        tong = tong + x
    return tong

# Điều giả sử kéo tới đâu: `so_thang` tháng, mỗi tháng không quá `tran`.
def tran_tong(so_thang, tran):
    return so_thang * tran

# Kết luận rút ra BẰNG PHẢN CHỨNG — chỉ được nhìn TỔNG, không nhìn tháng nào.
def bang_phan_chung(cot, so_thang, tran):
    return khong(tong_so(cot) <= tran_tong(so_thang, tran))

# Cùng kết luận ấy, kiểm bằng cách xét từng tháng một.
def bang_di_san(cot, tran):
    return any([x > tran for x in cot])

print(tran_tong(6, 35000))
print(bang_phan_chung(so_quy, 6, 35000))
print(bang_di_san(so_quy, 35000))
print(bang_phan_chung(deu_ba_muoi, 3, 30000))
print(bang_di_san(deu_ba_muoi, 30000))
```

```python title=test
assert tran_tong(6, 35000) == 210000, "sáu tháng, mỗi tháng nhiều nhất 35 000 đồng thì tổng nhiều nhất là 210 000 đồng"
assert tran_tong(3, 30000) == 90000, "ba tháng, mỗi tháng nhiều nhất 30 000 đồng thì tổng nhiều nhất là 90 000 đồng"
assert tran_tong(1, 35000) == 35000, "một tháng thôi thì trần của tổng đúng bằng trần của tháng ấy, là 35 000 đồng"
assert tran_tong(0, 35000) == 0, "không tháng nào thì không thu được đồng nào, nên trần của tổng là 0"
assert bang_phan_chung(so_quy, 6, 35000) == True, "sổ sáu tháng cộng ra 215 000 mà điều giả sử chỉ cho tới 210 000 — hai câu ngược nhau, nên giả sử sai"
assert bang_phan_chung(so_quy, 6, 36000) == False, "trần 36 000 cho sáu tháng là 216 000, mà sổ chỉ có 215 000, nên không có câu nào ngược câu nào ở ca này"
assert bang_phan_chung(deu_ba_muoi, 3, 30000) == False, "ba tháng đều đúng 30 000 cộng ra 90 000, vừa khít trần 90 000 — vừa khít thì chưa vượt"
assert bang_phan_chung(deu_ba_muoi, 3, 29000) == True, "trần 29 000 cho ba tháng là 87 000, mà cuốn sổ ấy cộng ra 90 000, nên nó vượt"
assert bang_di_san(so_quy, 35000) == True, "trong sổ sáu tháng, tháng 1 thu 40 000 và tháng 3 thu 60 000 — cả hai đều quá 35 000"
assert bang_di_san(so_quy, 60000) == False, "khoản lớn nhất trong sổ sáu tháng là 60 000, mà đúng 60 000 thì chưa QUÁ 60 000"
assert bang_di_san(so_quy, 59000) == True, "khoản 60 000 của tháng 3 quá 59 000"
assert bang_di_san(deu_ba_muoi, 30000) == False, "ba tháng đều đúng 30 000, mà đúng 30 000 thì chưa QUÁ 30 000"
assert bang_di_san(deu_ba_muoi, 29000) == True, "ở cuốn sổ ba tháng ấy, mỗi khoản 30 000 đều quá 29 000"
assert bang_phan_chung(so_quy, 6, 35000) == bang_di_san(so_quy, 35000), "hai lối phải cho cùng một kết luận trên sổ sáu tháng với trần 35 000 — một lối chỉ nhìn tổng, lối kia xét từng tháng"
assert bang_phan_chung(deu_ba_muoi, 3, 30000) == bang_di_san(deu_ba_muoi, 30000), "hai lối cũng phải trùng nhau ở cuốn sổ ba tháng với trần 30 000, chỗ tổng vừa khít trần"
```

:::hints
- kind: attention
  body: Chỗ trống đầu trả về một CON SỐ ghép từ đúng hai tham số của hàm — đọc lại câu "sáu khoản, mỗi khoản nhiều nhất 35 000, thì cộng lại nhiều nhất là bao nhiêu" trong phần chứng minh ba nhịp, và để ý phép tính nào đứng giữa hai con số ấy. Chỗ trống thứ hai đứng ngay sau dấu `<=`, nên nó phải là cái TRẦN của tổng, mà trần ấy vừa có sẵn một hàm tính hộ ở ngay trên. Chỗ trống thứ ba hỏi "có tháng nào không" — bài 18 đã đặt tên cho chữ "có" và bài 17 cho chữ "mọi"; hai chữ ấy có hai hàm dựng sẵn khác nhau trong Python.
- kind: strategy
  body: "`tran_tong` nhân trần của một tháng với số tháng. `bang_phan_chung` đừng tính lại cái trần bằng tay — gọi thẳng `tran_tong(so_thang, tran)`, vì đó đúng là dòng lập luận: điều giả sử kéo tới cái trần ấy, rồi tổng thật va vào nó. `bang_di_san` dựng một danh sách Đ/S cho từng tháng theo kiểu bài 17–19 (`[... for x in cot]`), rồi hỏi danh sách ấy có Đ ở dòng nào không. Chú ý chữ QUÁ: quá 30 000 nghĩa là lớn hơn 30 000, còn đúng 30 000 thì chưa quá."
- kind: one-line
  body: "Ba chỗ lần lượt là `so_thang * tran`; rồi `tran_tong(so_thang, tran)`; rồi `any([x > tran for x in cot])`."
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: trần của tổng phải TÍNH ra từ `so_thang` và `tran` chứ không gõ cứng, và lối phản chứng phải gọi lại chính hàm ấy — gõ thẳng con số 210 000 vào thì bài đang chấm một thứ nó không hề kiểm
  requireAst:
  # Khung khởi đầu không có dấu nhân nào. Trần của tổng là một phép nhân.
  - kind: uses-operator, target: '*', min: 1
  # Khung khởi đầu đã gọi `tran_tong` một lần (trong `print`). Lời giải gọi hai
  # lần — lần thứ hai nằm trong `bang_phan_chung`, đúng chỗ điều giả sử kéo tới
  # cái trần. Ai tính lại trần bằng tay trong đó thì con số 2 không đạt.
  - kind: uses-call, target: tran_tong, min: 2
  # `tran` phải được ĐỌC ở cả ba chỗ trống. Khung khởi đầu đọc nó 0 lần.
  - kind: uses-name, target: tran, min: 3
  # `so_thang` phải được đọc ở hai chỗ trống đầu. Khung khởi đầu đọc 0 lần.
  - kind: uses-name, target: so_thang, min: 2
  forbidAst:
  # Lưới thứ hai, chặn đúng hai con số là KẾT QUẢ chứ không phải nguyên liệu.
  - kind: has-literal, target: 210000
  - kind: has-literal, target: 90000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: ^210000\nTrue\nTrue\nFalse\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai lối, hai cách nhìn khác hẳn nhau, một kết luận. Mà lối phản chứng chưa mở sổ ra xem tháng nào cả.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Phản chứng đưa bạn tới **một** câu, xong là hết. Bạn giả sử phủ định của nó, đi
tới chỗ va, và câu ấy được chốt.

Nhưng nhìn câu này:

> 1 + 2 + … + n = n(n+1)/2, với mọi số nguyên dương n.

Nó không phải một câu. Nó là **vô hạn câu**, mỗi `n` một câu: câu cho n = 1, câu
cho n = 2, câu cho n = 3, và cứ thế không hết. Phản chứng chốt được một câu; nó
không có cách nào chốt hết một dãy không có dòng cuối.

Có một chỗ gợn, và nó gợn theo hướng dễ chịu. Câu thứ k+1 và câu thứ k gần nhau
đến kỳ lạ: vế trái của câu sau chỉ là vế trái của câu trước **cộng thêm đúng một
số hạng**. Không phải một câu mới toanh — chỉ là câu cũ, thêm một mẩu.

Tận dụng được chỗ "chỉ hơn một chút" đó không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
