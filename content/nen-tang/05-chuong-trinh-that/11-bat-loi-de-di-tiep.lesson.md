---
id: nen-tang.chuong-trinh-that.bat-loi-de-di-tiep
title: Bắt lấy lỗi để đi tiếp
summary: "`try` canh chừng một đoạn có thể nổ; `except` là chỗ chương trình rơi vào khi nó nổ thật — nhờ vậy một dòng sổ hỏng không giết cả cuốn sổ."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [err.try-except]
requires: [io.text-is-str, core.string-split, core.strip-newline, io.readlines, core.with-open, core.file-read, core.file-write, core.int-cast, core.value-error, err.traceback, core.accumulator, core.counter-if, core.list, core.list-index, core.len, core.fstring, core.variable, core.assignment, core.output, ctrl.for-each, ctrl.block-indent]
concepts: [core.loi-khi-chay, core.file, core.khoi-lenh]
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
Một ổ gà giữa đường thì tránh rồi đi tiếp. Không ai bỏ cả chuyến đi vì nó.
::::

::::explain{#mot-dong-hong-giet-ca-cuon-so}
Bài trước để lại đúng chỗ đau này: sổ dài sáu chục dòng, dòng thứ 41 gõ nhầm
thành `cà phê,hai lăm nghìn`, và `int("hai lăm nghìn")` nổ `ValueError`. Bốn
mươi khoản đã cộng xong nằm trong `tong` cũng mất trắng, vì `print` ở cuối
không bao giờ chạy tới.

Đáng nói là chương trình **không** viết sai. Nó làm đúng điều nó phải làm: gặp
một chuỗi không viết ra con số nào, `int` không có quyền bịa ra một con số, nên
nó dừng và nói. Thứ hỏng là **dữ liệu**, không phải mã. Mà dữ liệu do người gõ
thì lúc nào cũng có dòng hỏng.

Nên điều bạn cần không phải là làm cho lỗi biến mất. Điều bạn cần là nói trước
với máy: *đoạn này có thể nổ; nếu nó nổ thì đừng dừng cả chương trình, hãy làm
việc kia thay vào rồi đi tiếp*.

Python viết câu ấy bằng hai khối đi liền nhau:

```python title=readonly
try:
    tong = tong + int(o_tien)
except:
    so_dong_hong = so_dong_hong + 1
```

Đọc thành tiếng Việt: **thử** chạy khối trên; nếu khối ấy nổ ở bất cứ đâu, bỏ
dở ngay tại chỗ và nhảy xuống chạy khối dưới, **trừ khi** không có gì nổ — lúc
đó khối dưới bị bỏ qua hoàn toàn.

Hình dạng thì đúng hình dạng bạn đã quen: một từ khoá, một dấu hai chấm, rồi
một khối thụt vào. Giống `for` và `if` ở chỗ đó.

Ba điều đáng ghi lại, vì cả ba đều dễ hiểu ngược:

- **`except` chỉ chạy khi có lỗi.** Không lỗi thì nó không chạy dòng nào.
- **`try` bỏ dở ngay tại dòng nổ.** Những dòng phía dưới trong cùng khối `try`
  không chạy nữa. Nhưng những dòng đã chạy xong trước đó thì đã xong thật —
  giá trị chúng gán vẫn còn nguyên.
- **Xong `except` là đi tiếp bình thường.** Nếu cặp `try`/`except` nằm trong
  một vòng lặp, vòng lặp bước sang lượt kế như chưa có chuyện gì. Nó không
  giống `break` chút nào.

Điều thứ ba mới là thứ cứu cuốn sổ: đặt cặp ấy vào **trong** vòng lặp thì một
dòng hỏng chỉ làm hỏng đúng lượt của nó.
::::

::::example{#so-co-mot-dong-hong}
Bốn khoản, trong đó một khoản ghi tiền bằng chữ:

```python title=readonly
cac_o_tien = ["25000", "hai lăm nghìn", "15000", "120000"]

tong = 0
so_dong_hong = 0
for o_tien in cac_o_tien:
    try:
        tong = tong + int(o_tien)
    except:
        so_dong_hong = so_dong_hong + 1

print(f"Cộng được: {tong} đồng")
print(f"Bỏ qua {so_dong_hong} dòng hỏng")
```

Máy in ra:

```text title=readonly
Cộng được: 160000 đồng
Bỏ qua 1 dòng hỏng
```

Ba chỗ đáng dừng lại nhìn:

- **Chương trình chạy tới hết.** Hai dòng `print` cuối cùng đã hiện ra, tức là
  không có gì giết nó giữa chừng. Trước bài này, cùng dữ liệu ấy thì màn hình
  chỉ có một cái traceback.
- **Ba khoản đọc được đều vào tổng.** Hai mươi lăm nghìn, mười lăm nghìn và
  một trăm hai mươi nghìn — cộng lại thành 160000 đồng. Lượt hỏng nằm ở giữa
  mà hai lượt sau nó vẫn chạy: `except` xong là vòng lặp đi tiếp.
- **Cuốn sổ tự nói ra nó bẩn chỗ nào.** Dòng `Bỏ qua 1 dòng hỏng` không phải
  trang trí: nó là cách chương trình thú nhận rằng con số phía trên chưa gồm
  hết mọi khoản. Một chương trình lặng lẽ bỏ qua dòng hỏng còn nguy hơn một
  chương trình dừng lại.
::::

::::predict{#doan-tong-sau-loi commitOnce}
Byte đặt cặp `try`/`except` vào trong vòng lặp, và đặt dòng hỏng vào **giữa**
ba ô tiền chứ không phải ở cuối.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
cac_o_tien = ["25000", "hai lăm nghìn", "15000"]

tong = 0
for o_tien in cac_o_tien:
    try:
        tong = tong + int(o_tien)
    except:
        print("bỏ qua ô:", o_tien)

print("tổng:", tong)
```

:::opt{correct}
`bỏ qua ô: hai lăm nghìn` rồi `tổng: 40000`
:::

:::opt
`bỏ qua ô: hai lăm nghìn` rồi `tổng: 25000`
::why
Gần đúng ở chỗ bạn theo dõi rất sát hai lượt đầu: lượt một cộng được 25000
thật, lượt hai rơi vào `except` thật.

Chỗ lệch là ở lượt thứ ba. Bạn đang đọc `except` như thể nó kết thúc vòng lặp,
giống `break` — và đó là cách hiểu rất tự nhiên, vì cả hai đều làm chương
trình "nhảy đi chỗ khác". Nhưng `except` chỉ nhảy ra khỏi **khối `try` của
lượt này**. Chạy hết khối `except` là vòng lặp bước sang lượt kế bình thường,
nên `"15000"` vẫn được cộng vào.
::
:::

:::opt
`bỏ qua ô: hai lăm nghìn` rồi `tổng: 0`
::why
Gần đúng ở chỗ bạn cẩn thận với một chuyện đáng cẩn thận: lỗi xảy ra giữa
chừng thì kết quả đang dở dang có còn tin được không.

Chỗ lệch là `except` không dọn dẹp gì cả. Nó không trả `tong` về giá trị lúc
đầu, cũng không huỷ những gì đã gán xong. Lượt một đã cộng 25000 vào `tong` và
việc ấy đã xong hẳn trước khi lượt hai bắt đầu, nên `tong` giữ nguyên số ấy rồi
cộng tiếp 15000 ở lượt ba.
::
:::

:::opt
Máy dừng và in traceback `ValueError`
::why
Gần đúng ở chỗ đó chính là điều xảy ra ở bài trước, với đúng dữ liệu này. Bạn
nhớ đúng hành vi cũ.

Chỗ lệch là dòng `int(o_tien)` bây giờ nằm **trong** một khối `try`. Lỗi sinh
ra bên trong khối ấy không đi ra tới ngoài nữa: nó bị khối `except` ngay dưới
đón lấy, và cái traceback không bao giờ được in. Đó đúng là việc mà cặp
`try`/`except` sinh ra để làm.
::
:::
::::

::::code{#doc-so-co-dong-hong}
Cuốn sổ bốn dòng, một dòng ghi tiền bằng chữ. Chương trình phải cộng được ba
dòng còn lại **và** nói ra nó đã bỏ qua mấy dòng.

Ba chỗ trống: hai chỗ mở khối, một chỗ nằm giữa.

```python title=starter
with open("so_bai_muoi_mot.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,hai lăm nghìn\n")
    f.write("vở ghi,15000\n")
    f.write("sách,120000\n")

with open("so_bai_muoi_mot.txt", "r") as f:
    cac_dong = f.readlines()

tong = 0
so_dong_hong = 0
for dong in cac_dong:
    manh = dong.strip().split(",")
    ___:
        tong = tong + ___
    ___:
        so_dong_hong = so_dong_hong + 1

print(f"Cộng được: {tong} đồng")
print(f"Bỏ qua {so_dong_hong} dòng hỏng")
```

```python title=solution
with open("so_bai_muoi_mot.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,hai lăm nghìn\n")
    f.write("vở ghi,15000\n")
    f.write("sách,120000\n")

with open("so_bai_muoi_mot.txt", "r") as f:
    cac_dong = f.readlines()

tong = 0
so_dong_hong = 0
for dong in cac_dong:
    manh = dong.strip().split(",")
    try:
        tong = tong + int(manh[1])
    except:
        so_dong_hong = so_dong_hong + 1

print(f"Cộng được: {tong} đồng")
print(f"Bỏ qua {so_dong_hong} dòng hỏng")
```

```python title=test
# Ba chỗ trống, và mỗi chỗ có ít nhất một câu vỡ nếu điền sai.
#
#   Chỗ 1 và chỗ 3 (hai từ khoá mở khối): thiếu lớp canh chừng thì dòng
#     "bún bò,hai lăm nghìn" làm `int` nổ ValueError, chương trình dừng, và
#     không câu nào dưới đây chạy tới.
#   Chỗ 2 (`int(manh[1])`): quên `int` thì `0 + "25000"` nổ TypeError. Điền
#     một hằng số thì không dòng nào nổ nữa, `so_dong_hong` đứng yên ở 0 và
#     câu thứ hai vỡ. Lấy nhầm `manh[0]` thì cả bốn dòng đều nổ, `tong` ở 0.
assert tong == 160000, "ba dòng đọc được số là 25000, 15000 và 120000; cộng lại phải ra 160000 — dòng ghi tiền bằng chữ không góp đồng nào vào tổng"
assert so_dong_hong == 1, "trong bốn dòng của sổ này chỉ có đúng một dòng làm int() nổ, là dòng 'bún bò,hai lăm nghìn'"
assert len(cac_dong) == 4, "cả bốn dòng đều phải được đọc lên và duyệt qua: bắt lỗi là để ĐI TIẾP, không phải để dừng sớm"
```

:::hints
- kind: attention
  body: Hai chỗ trống kết thúc bằng dấu hai chấm và có một khối thụt vào bên dưới — đó là chỗ của một từ khoá mở khối, giống `for` và `if` bạn đã quen. Chỗ trống thứ ba nằm sau dấu `+`, đúng chỗ bài trước vừa điền.
- kind: strategy
  body: Dòng có thể nổ là dòng đổi mảnh tiền thành số, nên nó phải nằm trong khối được canh chừng; khối đứng dưới là chỗ chương trình rơi vào khi có lỗi, và ở đó nó đếm thêm một dòng hỏng. Chỗ trống giữa thì lấy mảnh đứng sau dấu phẩy của dòng đang xét rồi đổi nó thành số thật.
- kind: one-line
  body: "Chỗ đầu viết `try`, chỗ giữa viết `int(manh[1])`, chỗ cuối viết `except`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  expect: "Cộng được: 160000 đồng"
- tier: output
  expect: "Bỏ qua 1 dòng hỏng"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sổ bẩn một dòng, chương trình vẫn về tới đích. Và nó khai luôn dòng bẩn ấy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cái lưới bạn vừa giăng có một tính chất mà lúc viết ta chưa để ý: `except:`
trơn không hỏi gì cả. Nó bắt **mọi** lỗi sinh ra trong khối `try`, bất kể lỗi
ấy từ đâu ra.

Giả sử ngày mai bạn nới khối `try` ra thành hai dòng cho dễ đọc:

```python title=readonly
    try:
        tien = int(manh[1])
        tong = tong + tein
    except:
        so_dong_hong = so_dong_hong + 1
```

Dòng giữa gõ nhầm: `tein` thay cho `tien`. Realm 0 đã dạy máy sẽ nổ `NameError`
ở đó, vì không có cái tên nào như thế. Nhưng lần này cái nổ ấy sinh ra bên
trong `try`, nên cái lưới của bạn đón luôn nó, đếm thêm một dòng vào
`so_dong_hong`, rồi đi tiếp.

Chương trình vẫn chạy tới hết. Vẫn in ra một con số, vẫn in ra một câu thống kê
đọc lên nghe rất bình thường — và câu thống kê ấy đổ lỗi cho cuốn sổ, trong khi
thứ hỏng là dòng bạn vừa gõ. Không một dòng traceback nào hiện ra để chỉ vào
chỗ ấy, vì cái lưới đã đón mất nó.

Cái lưới không phân biệt được "dòng dữ liệu hỏng" với "code của bạn hỏng", nên
nó giấu luôn lỗi gõ nhầm của chính bạn.

Nguy ở chỗ nào, và làm sao bảo cái lưới chỉ bắt đúng thứ bạn muốn bắt?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
