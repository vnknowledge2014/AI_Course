---
id: nen-tang.gia-tri-bien-kieu.dem-bang-don-vi-nho-nhat
title: Đếm thì giữ bằng đơn vị nhỏ nhất
summary: Tiền là thứ đếm chứ không phải thứ đo, nên trong máy nó phải nằm ở đơn vị nhỏ nhất — đồng — để mọi con số đều tròn.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.don-vi-nho-nhat]
requires: [core.float, core.arithmetic, core.type-of-value, core.type-fn, core.variable, core.fstring, ctrl.comparison, core.function-def, core.function-parameter, core.function-return, core.function-call]
concepts: [core.kieu-gia-tri]
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
Đơn vị nằm trong đầu bạn chứ không nằm trong con số. Mình chỉ giữ đúng thứ bạn đưa.
::::

::::explain{#do-va-dem}
Bài trước để lại một câu hỏi thẳng: nếu `==` trên số thực có ngày sai, thì tiền
phải lưu bằng kiểu gì?

Ra chợ một vòng đã có câu trả lời. Hàng rau cân cho bạn một bó cải: kim cân
dừng đâu đó quanh vạch 300 gam — nhích tay một cái là 302, đặt lại lần nữa là
298. Hàng trứng thì đếm cho bạn 12 quả. Không ai đếm ra 12 quả rưỡi, và đếm lại
mười lần vẫn đúng 12.

Đó là hai loại con số khác nhau, và máy có sẵn hai kiểu cho chúng:

- Thứ **đo** được thì chia nhỏ mãi cũng còn chia được, nên bao giờ cũng có chỗ
  lệch. Kiểu của nó là `float`.
- Thứ **đếm** được thì có một đơn vị nhỏ nhất, dưới mức đó không còn gì. Kiểu
  của nó là `int`.

Tiền thuộc vế nào? Vế đếm. Đồng bạc lẻ nhất còn tiêu được ở Việt Nam là tờ 200
đồng, và không có món hàng nào giá nửa đồng. Tiền là thứ **đếm bằng đồng**.

Vậy mà năm bài vừa rồi bạn đã thấy `float` xen vào bảng tính tiền không dưới
một lần. Nó xen vào ở đâu?
::::

::::example{#so-ghi-bang-nghin}
Nó xen vào ngay lúc bạn cầm bút. Sổ chi tiêu người Việt hay ghi bằng **nghìn**
cho gọn: tiền điện 153,2 — nghĩa là 153 nghìn 2 trăm.

```python title=readonly
# Sổ tháng này, ghi bằng NGHÌN — đúng như trên giấy
tien_dien = 153.2
tien_nuoc = 48.6
tien_mang = 220.0

tong = tien_dien + tien_nuoc + tien_mang
print(tong)
print(tong == 421.8)
```

Cầm máy tính bấm tay thì ba số ấy cộng lại đúng bằng 421,8. Máy in ra:

```text
421.79999999999995
False
```

Không có gì mới ở đây — đúng chuyện bài trước vừa nói: `153.2` máy giữ không
tròn, `48.6` cũng vậy, và ba chỗ lệch tí xíu cộng lại thành một cái đuôi nhìn
thấy được.

Chỗ đáng chú ý nằm ở nguyên nhân sâu hơn. Số tiền này không hề có phần lẻ nào
cả: 153.200 đồng là một số nguyên đồng, 48.600 cũng thế. Phần lẻ chỉ xuất hiện
vì **bạn đã đổi đơn vị**: chia hết cho 1000 để viết cho ngắn. Bạn tự tay biến
một thứ đếm được thành một thứ đo được, rồi lãnh đủ chỗ lệch của thứ đo được.
::::

::::explain{#giu-o-don-vi-nho-nhat}
Từ đó ra luật của bài này:

> **Giữ tiền trong máy ở đơn vị nhỏ nhất — đồng — dưới dạng `int`.**

Không phải "tránh dùng `float`" chung chung. Cụ thể hơn thế: chọn cái **đơn
vị** làm cho mọi con số của bạn trở thành số nguyên. Với tiền Việt, đơn vị đó
là đồng. `153200` là một số nguyên tròn trịa, máy giữ đúng từng đơn vị một,
cộng bao nhiêu lần cũng không sinh ra cái đuôi nào.

Giới lập trình gọi cách làm này là giữ giá trị ở **đơn vị nhỏ nhất**. Phần mềm
ngân hàng, phần mềm bán hàng, ví điện tử — chúng đều lưu tiền như vậy: đồng,
xu, cent. Không phần mềm tính tiền nghiêm túc nào lưu số tiền bằng `float`.

Và có một điều dễ tưởng nhầm ở đây: **máy không hề biết đơn vị của bạn là gì.**
`153200` với `153.2` đều là hai con số trần trụi; chữ "đồng" và chữ "nghìn" chỉ
nằm trong đầu bạn và trong cái tên bạn đặt. Vì máy không giữ đơn vị hộ bạn, cả
chương trình phải thống nhất **một** đơn vị duy nhất — chọn đồng thì mọi con số
tiền trong chương trình là đồng, từ đầu tới cuối, không có ngoại lệ.
::::

::::predict{#doan-so-nguyen commitOnce}
Vẫn ba khoản ấy, cùng số tiền ấy, chỉ khác chỗ ghi bằng **đồng**.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra hai dòng nào?

```python title=readonly
tien_dien = 153200
tien_nuoc = 48600
tien_mang = 220000

tong = tien_dien + tien_nuoc + tien_mang
print(tong)
print(tong == 421800)
```

:::opt{correct}
421800 rồi True
:::

:::opt
421800.0 rồi True
::why
Gần đúng ở chỗ bạn nhớ luật lây của bài 1: chỉ cần một `float` lẫn vào là cả
biểu thức ra `float`, kèm cái đuôi `.0`. Luật ấy đúng và bạn sẽ còn dùng nó dài
dài.

Chỗ lệch là ở đây không có `float` nào để mà lây. Cả ba con số đều viết trần,
không dấu chấm nào, nên cả ba mang nhãn `int`. `int` cộng `int` ra `int`, và
`int` thì không có đuôi để in.
::
:::

:::opt
421800 rồi False
::why
Gần đúng ở chỗ bạn nhận ra `==` so hai bên đúng từng chút một, không châm chước
gì cả — nó vẫn so đúng như thế, và ở ví dụ trên nó đã cho `False` thật.

Chỗ lệch nằm ở thứ đem ra so. Lần trước hai bên là số thực, mà số thực máy giữ
xấp xỉ nên lệch ở chữ số thứ mười lăm. Lần này ba số nguyên cộng lại ra một số
nguyên, và số nguyên máy giữ **đúng từng đơn vị**. Không có chỗ nào để sinh ra
chênh lệch.
::
:::

:::opt
421.8 rồi True
::why
Gần đúng ở chỗ bạn đọc con số theo đơn vị bạn quen dùng: sổ ghi bằng nghìn thì
tổng phải đọc là 421,8 nghìn. Người đọc sổ nào cũng hiểu đúng như bạn.

Chỗ lệch: đơn vị chỉ nằm trong đầu bạn. Máy nhận vào ba con số `153200`,
`48600`, `220000` và cộng chúng lại — nó không biết đâu là nghìn, đâu là đồng,
nên nó không tự chia cho 1000 giúp bạn. Muốn màn hình hiện ra con số theo nghìn
thì phải có ai đó bảo nó làm vậy.
::
:::
::::

::::explain{#tu-nghin-ve-dong}
Còn lại đúng một việc: sổ trên giấy vẫn ghi bằng nghìn, nên phải có chỗ đổi
`153.2` thành `153200` trước khi đem vào tính.

Đổi thì nhân với 1000. Nhưng nhân xong bạn vẫn đang cầm một số **thực**, và số
thực thì hay lệch ở chữ số cuối:

```text
>>> 32.3 * 1000
32299.999999999996
```

Tới đây có hai cách kéo nó về `int`, và chúng cho hai kết quả khác nhau:

- `int(32299.999999999996)` — **cắt** phần lẻ, ra `32299`. Mất một đồng, im
  lặng, không báo lỗi nào. Đúng cái bẫy bài 2 đã dựng.
- `round(32299.999999999996)` — lấy **số nguyên gần nhất**, ra `32300`. Đúng số
  tiền thật.

Nên bước cuối của phép đổi đơn vị là `round`, không phải `int`.

> Chỗ dễ vấp: `32.3 * 1000` thiếu một chút, nhưng `20.1 * 1000` lại **thừa**
> một chút. Bạn không đoán được số nào lệch về phía nào, và nhìn bằng mắt cũng
> không thấy. Vì vậy đừng thử từng số rồi kết luận "số của mình không sao" —
> cứ dùng `round` cho mọi số, đó là cách duy nhất không phải đoán.
::::

::::code{#doi-mot-khoan-sang-dong}
Byte đang chép sổ giấy vào máy. Trên giấy mọi khoản ghi bằng nghìn; trong máy
mọi khoản phải là số đồng, kiểu `int`.

Hãy điền chỗ trống để `sang_dong` nhận một số **nghìn** và trả về đúng số
**đồng**.

```python title=starter
def sang_dong(so_nghin):
    return ___

print(sang_dong(153.2))
print(sang_dong(48.6))
print(sang_dong(153.2) + sang_dong(48.6) + sang_dong(220.0) == 421800)
```

```python title=solution
def sang_dong(so_nghin):
    return round(so_nghin * 1000)

print(sang_dong(153.2))
print(sang_dong(48.6))
print(sang_dong(153.2) + sang_dong(48.6) + sang_dong(220.0) == 421800)
```

```python title=test
# Chấm trên BỐN khoản, không phải một: một chỗ trống mà chỉ thử đúng một con
# số thì đáp án chép cứng cũng lọt. Khoản 32.3 là khoản gắt nhất — nó là chỗ
# duy nhất phân biệt phép làm tròn với phép cắt.
assert sang_dong(153.2) == 153200
assert sang_dong(48.6) == 48600
assert sang_dong(220.0) == 220000
assert sang_dong(32.3) == 32300
# Trả về phải là số ĐẾM chứ không phải số đo: số nguyên in ra không có đuôi
# `.0`, số thực thì có — nên `so_nghin * 1000` để trần sẽ trượt đúng câu này.
assert f"{sang_dong(48.6)}" == "48600"
# Và cộng ba khoản lại phải khớp tuyệt đối, không xê xích.
assert sang_dong(153.2) + sang_dong(48.6) + sang_dong(220.0) == 421800
```

:::hints
- kind: attention
  body: Chỗ trống nằm sau `return`, và cái tên duy nhất bạn có trong tay là `so_nghin`. Nó đang mang một số ĐO, đơn vị nghìn; thứ phải trả về là một số ĐẾM, đơn vị đồng.
- kind: strategy
  body: Một nghìn đồng là 1000 đồng, nên phép đầu tiên là nhân với 1000. Nhưng nhân xong bạn vẫn cầm một số thực có thể lệch — `32.3 * 1000` máy giữ là `32299.999999999996`. Cần thêm một bước kéo nó về số nguyên GẦN NHẤT, và bước ấy không phải phép cắt.
- kind: one-line
  body: "Viết `round(so_nghin * 1000)` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: 153200
- tier: static
  onFail: chỗ trống phải TÍNH ra số đồng từ `so_nghin`, không chép sẵn con số nào
  requireAst:
  # Phải có phép nhân VÀ phải đọc tới `so_nghin`. Hai điều kiện cùng lúc loại
  # được cả đáp án gõ bừa một giá trị lẫn đáp án chép cứng một con số.
  - kind: uses-operator, target: *, min: 1
  - kind: uses-name, target: so_nghin, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đồng thì mình đếm được. Cộng bao nhiêu khoản cũng không mọc thêm cái đuôi nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ mọi con số tiền trong đầu bạn đã có chỗ đứng: giữ bằng đồng, kiểu `int`, và
chỉ đổi ra nghìn lúc **in cho người đọc** — món nợ ấy để dành cuối track, khi
bạn học cách bảo máy in một con số theo đúng ý mình.

Nhưng có một con số bạn chưa thật sự cầm được: **con số đầu tiên**. Nó không do
bạn gõ vào code, nó do người ghi sổ gõ vào bàn phím — và Realm 0 đã nói rõ,
`input()` bao giờ cũng trả về **chữ**.

Người ghi sổ quen ghi bằng nghìn nên gõ `25.5`. Bạn muốn nó thành số, và công cụ bạn có
trong tay từ Realm 0 là `int()`. Bạn viết:

```python
so_nghin = int("25.5")
```

Đoán thử xem máy làm gì. Nó cắt phần lẻ ra `25` như `int(25.5)` vẫn làm? Hay
chuyện khác hẳn xảy ra?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
