---
id: toan.cam-nhan-so.mang-chu-nhat-xoay-mot-goc
title: Mảng chữ nhật xoay một góc
summary: Xếp cây thành mảng hàng × cột, xoay 90° là đổi vai hai con số mà không thêm bớt cây nào — nên `a × b` và `b × a` buộc phải bằng nhau.
locale: vi
track: toan
module: cam-nhan-so
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.multiply-commutative]
requires: [math.multiplication, core.arithmetic, core.boolean, ctrl.comparison, core.variable, core.print-variable, core.output]
concepts: [math.mang-chu-nhat, math.doi-cho-hai-thua-so]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Hai cái vườn khác nhau thật. Mình xoay một cái đi rồi đếm lại xem.
::::

::::explain{#hai-day-cong-chang-giong-nhau}
Bài trước để lại câu hỏi: vườn 5 luống mỗi luống 8 cây, và vườn 8 luống mỗi
luống 5 cây — số cây có khác nhau không?

Thử trả lời bằng đúng thứ bạn đang có, là phép cộng:

```text
vườn thứ nhất:  8 + 8 + 8 + 8 + 8
vườn thứ hai:   5 + 5 + 5 + 5 + 5 + 5 + 5 + 5
```

Hai dãy này không giống nhau chỗ nào cả. Một dãy có năm số hạng, dãy kia có
tám. Một dãy toàn số 8, dãy kia toàn số 5. Cộng ra thì cả hai đều bằng 40 —
nhưng nhìn vào hai dãy ấy, không có gì nói cho bạn biết trước rằng chúng phải
bằng nhau. Bạn chỉ biết sau khi đã cộng xong cả hai.

Mà "cộng xong rồi thấy trùng" thì chưa phải một lý do. Nếu đổi sang 17 và 23,
bạn lại phải cộng hai dãy dài ngoằng nữa mới dám nói. Cái ta cần là một cách
nhìn khiến chuyện bằng nhau **lộ ra trước khi tính**.
::::

::::explain{#xep-cay-thanh-mang}
Byte không xếp cây thành hàng dài. Byte xếp thành **mảng chữ nhật**: cây đứng
thẳng hàng theo cả hai chiều, hàng nào cũng đủ chừng ấy cây.

Vườn thứ nhất — 5 luống, mỗi luống 8 cây:

```text
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
```

Vườn thứ hai — 8 luống, mỗi luống 5 cây:

```text
● ● ● ● ●
● ● ● ● ●
● ● ● ● ●
● ● ● ● ●
● ● ● ● ●
● ● ● ● ●
● ● ● ● ●
● ● ● ● ●
```

Bây giờ cầm tờ giấy có hình thứ nhất, **xoay đi một phần tư vòng**. Hình thứ
hai hiện ra. Không phải "trông giống" — đúng là nó, từng cây một.

Và đây là chỗ trả lời câu "vì sao": xoay tờ giấy thì **không cây nào bị thêm
vào, không cây nào rơi ra**. Bài 2 đã dựng sẵn lý do — đếm là ghép mỗi vật với
đúng một tên số, nên hễ vẫn từng ấy vật thì vẫn ra từng ấy số, đếm kiểu nào
cũng thế. Xoay giấy không đụng vào vật nào cả.

Nên `8 × 5` và `5 × 8` không phải hai con số tình cờ trùng nhau. Chúng là **một
con số**, đọc từ hai phía của cùng một mảng: đọc theo hàng ngang thì thấy "5 lô,
mỗi lô 8"; đọc theo cột dọc thì thấy "8 lô, mỗi lô 5".

Tính chất này có tên: phép nhân **giao hoán**. Viết gọn: `a × b = b × a`.
::::

::::explain{#vai-van-khac-nhau}
Có một chỗ rất dễ hiểu quá tay ở đây, nên nói thẳng ra:

Bài 19 nói hai con số trong phép nhân đứng hai vai khác nhau. Bài này **không**
xoá điều đó đi.

Hai cái vườn vẫn là hai cái vườn khác nhau. Một cái dài 8 cây và có 5 luống,
một cái dài 5 cây và có 8 luống. Đi mua lưới che, đi rải phân, đi kéo ống nước
— hai vườn ấy cần những thứ khác nhau, và bạn không được phép nhầm.

Thứ bằng nhau chỉ có đúng một: **số cây**. Dấu `×` là cái máy đếm cây, và cái
máy ấy cho ra cùng một kết quả dù bạn nạp hai con số theo thứ tự nào.

Cẩn thận thêm một tầng nữa: giao hoán được chứng minh bằng **cái mảng chữ
nhật**, mà chỉ phép nhân mới có mảng. Đừng vội đem nó đi cho mượn — ngay ở
phần dưới đây bạn sẽ đem đúng câu hỏi "đổi chỗ hai con số" ấy hỏi một phép
khác, và kết quả không như bạn nghĩ.
::::

::::predict{#doan-ba-cau-hoi commitOnce}
Byte bắt máy làm trọng tài cho ba câu "đổi chỗ hai con số thì có bằng nhau
không". Cả ba câu đều dùng đúng hai con số 8 và 5.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(8 * 5 == 5 * 8)
print(8 + 5 == 5 + 8)
print(8 - 5 == 5 - 8)
```

:::opt{correct}
True rồi True rồi False
:::

:::opt
True rồi True rồi True
::why
Gần đúng ở chỗ bạn vừa nắm được một sự thật hẳn hoi: xoay cái mảng đi thì số
cây không đổi, nên `8 × 5` và `5 × 8` bằng nhau. Suy nghĩ ấy đúng ở hai dòng
đầu — phép cộng cũng đổi chỗ được, vì bài 10 đã nói đổ đống nào vào trước cũng
ra một đống chung như nhau.

Chỗ lệch là **phạm vi** của sự thật ấy. Nó được chứng minh bằng cái mảng chữ
nhật, và phép trừ thì không có mảng nào để xoay. Bài 15 nói phép trừ là
**khoảng cách có hướng**: từ 5 tới 8 là 3 bước sang phải, từ 8 tới 5 là 3 bước
sang trái. Cùng độ dài, ngược chiều. Nên `8 − 5` ra 3 còn `5 − 8` ra −3, và hai
số ấy khác nhau.
::
:::

:::opt
False rồi True rồi False
::why
Gần đúng ở chỗ bạn giữ chặt điều bài 19 dạy, và giữ đúng: hai con số trong phép
nhân đứng hai vai khác nhau, không thay nhau được. Vườn 5 luống × 8 cây và vườn
8 luống × 5 cây là hai cái vườn khác nhau thật — bạn không hề nhầm chuyện đó.

Chỗ lệch: dấu `==` không hỏi "hai cái vườn có giống nhau không". Nó hỏi "hai
**con số** này có bằng nhau không". Hai vườn khác hình dạng mà vẫn cùng số cây —
đó đúng là điều bài này đem ra khoe, và cái mảng xoay một góc là bằng chứng.
::
:::

:::opt
True rồi True rồi một dòng báo lỗi
::why
Gần đúng ở chỗ bạn nhớ `5 − 8` phải lùi qua bên trái số 0, và có những cái máy
tính bỏ túi đời cũ thật sự nhấp nháy chữ "E" ở đúng chỗ đó.

Chỗ lệch: bài 16 đã kéo hẳn thanh số chạy tiếp về bên trái số 0, nên `5 − 8` có
một chỗ đứng đàng hoàng, tên là −3. Python không dừng ở 0; nó tính ra `-3`, so
với `3`, thấy khác nhau, rồi in `False` và đi tiếp.
::
:::
::::

::::explain{#dem-mot-lan-dung-hai-lan}
Giao hoán không phải một câu để thuộc lòng. Nó là một chỗ **đỡ việc**.

Muốn biết 4 nhân 25 là bao nhiêu mà thấy cộng 4 hai mươi lăm lần thì mệt: xoay
mảng lại, thành cộng 25 bốn lần — `25 + 25 + 25 + 25`, ra 100, nhẩm được trong
đầu.

Ngoài chợ cũng thế. "12 bó rau, mỗi bó 5 nghìn" và "5 bó rau, mỗi bó 12 nghìn"
là hai chuyện mua bán khác hẳn nhau, nhưng cùng phải trả 60 nghìn. Người bán
nhẩm theo chiều nào dễ hơn thì nhẩm theo chiều ấy.

Từ giờ, mỗi lần gặp một phép nhân khó, bạn có quyền xoay nó lại trước khi tính.
::::

::::code{#trong-tai-hai-cau}
Byte muốn máy chốt lại hai chuyện, và hai chuyện này cố tình cho ra **hai câu
trả lời ngược nhau** — nên một chữ `True` hay `False` gõ cứng chỉ qua được
nhiều nhất một dòng.

- **Chỗ trống 1**: Byte đếm xong mảng **6 hàng, mỗi hàng 9 cây** — 54 cây. Bây
  giờ xoay mảng ấy đi một góc: nó thành **9 hàng**. Mỗi hàng lúc này mấy cây?
  Điền con số ấy, **đừng nhân lại** — hãy xoay.
- **Chỗ trống 2**: cũng đổi chỗ hai con số như thế, nhưng với phép trừ — "12
  bớt 5" và "5 bớt 12" có bằng nhau không? Chỗ này phải là một **câu hỏi** viết
  bằng dấu `==`, không phải câu trả lời chép sẵn.

```python title=starter
thua_so = ___          # xoay mảng 6 hàng × 9 cột lại: 9 hàng, mỗi hàng mấy cây?
xoay_phep_tru = ___    # "12 bớt 5" và "5 bớt 12" có bằng nhau không?

print(9 * thua_so == 54)
print(xoay_phep_tru)
```

```python title=solution
thua_so = 6
xoay_phep_tru = 12 - 5 == 5 - 12

print(9 * thua_so == 54)
print(xoay_phep_tru)
```

```python title=test
# Chỗ trống 1 chấm đúng cái việc xoay mảng: chỉ một con số duy nhất lọt qua,
# và tìm ra nó nghĩa là đã đọc được mảng 6 × 9 theo chiều kia. Chỗ trống 2 giữ
# vai đối chứng — xoay mảng thì bằng, xoay phép trừ thì không.
assert thua_so == 6, "xoay mảng 6 hàng × 9 cột thành 9 hàng × 6 cột — mỗi hàng 6 cây"
assert 9 * thua_so == 54, "9 hàng, mỗi hàng chừng ấy cây, phải về lại đúng 54 cây"
assert xoay_phep_tru is False, "12 − 5 và 5 − 12 ngược chiều nhau — không bằng nhau"
assert 6 * 9 == 54 and 9 * 6 == 54, "cả hai chiều đều đếm ra 54 cây"
assert 12 - 5 == 7 and 5 - 12 == -7, "một bên 7, bên kia −7: cùng khoảng cách, khác phía"
```

:::hints
- kind: attention
  body: Chỗ trống 1 không đòi bạn nhân gì cả — mảng 6 hàng × 9 cột đã đếm xong là 54 cây rồi. Cầm tờ giấy xoay đi một phần tư vòng rồi nhìn lại: bây giờ có 9 hàng, mỗi hàng bao nhiêu cây?
- kind: strategy
  body: Xoay mảng thì hàng thành cột, cột thành hàng — không cây nào thêm vào, không cây nào rơi ra, nên số cây mỗi hàng mới chính là số hàng cũ. Còn chỗ trống 2 là một câu hỏi ba phần: vế trái, dấu so sánh bằng (hai dấu bằng viết liền nhau), vế phải — so `12 - 5` với phép trừ đổi chỗ của nó.
- kind: one-line
  body: "Viết `6` vào chỗ trống thứ nhất và `12 - 5 == 5 - 12` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống thứ hai phải là một câu so sánh bằng (`==`) giữa hai phép trừ — gõ thẳng `False` thì bạn không hỏi máy điều gì cả
  requireAst:
  - kind: uses-operator, target: ==, min: 2
  - kind: uses-operator, target: -, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^True\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Vườn thì xoay được, phép trừ thì không. Mình vừa biết thêm một chỗ phải cẩn thận.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte gieo một mảng **7 hàng, mỗi hàng 13 cây**.

Muốn biết bao nhiêu cây, bạn có ba đường, và cả ba đều đắt:

- Đếm từng cây một: 91 lần đếm.
- Cộng 13 bảy lần: `13 + 13 + 13 + 13 + 13 + 13 + 13`.
- Xoay mảng lại rồi cộng 7 mười ba lần — còn dài hơn.

Nhưng nhìn cái mảng ấy lâu một chút. Nó là một hình chữ nhật nằm im trên đất, và
bạn có thể lấy sợi dây căng ngang qua nó ở bất cứ chỗ nào để **chia nó thành hai
miếng**. Chẳng hạn cắt 13 cột thành 10 cột và 3 cột.

Cắt như thế thì mỗi miếng có dễ đếm hơn cái mảng ban đầu không — và quan trọng
hơn: cộng hai miếng lại có ra đúng chừng ấy cây không, hay cắt là mất mát?

Bài sau cầm sợi dây lên.
::::

::::checkpoint{mastery=0.8}
::::
