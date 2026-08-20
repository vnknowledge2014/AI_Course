---
id: nen-tang.re-nhanh-va-lap.dieu-kien-trong-dieu-kien
title: Lối rẽ nằm trong lối rẽ
summary: Một `if` đặt được ngay trong thân của một `if` khác, và tầng trong chỉ được hỏi tới khi tầng ngoài đã đúng.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ctrl.if-nested]
requires: [ctrl.if, ctrl.block-indent, ctrl.elif]
concepts: [ctrl.re-nhanh, core.khoi-lenh]
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
Tấm biển thứ hai treo bên trong cổng. Không qua cổng thì không ai đọc tới nó.
::::

::::explain{#tam-bien-ben-trong-cong}
Realm 0 để lại cho bạn một chuỗi rẽ nhánh: `if`, `elif`, `else` — ba lối của
cùng **một** ngã ba, cả ba viết sát cùng một mức lề. Khách gọi tô nhỏ, tô vừa
hay tô lớn thì máy chọn đúng một lối rồi đi tiếp.

Nhưng có loại câu hỏi không xếp ngang hàng như thế được.

Chung cư nhà bác Tư có hai tấm biển. Tấm thứ nhất treo ngoài cổng:

> Chỉ cư dân mới được vào.

Tấm thứ hai treo dưới hầm xe, tức là **bên trong** cổng:

> Xe máy điện đỗ ở khu B.

Người đi ngang đường, không phải cư dân, thì cả đời không đọc tấm biển thứ hai.
Không phải vì tấm biển ấy sai, mà vì họ không đi tới chỗ nó treo.

Hai câu hỏi ấy không ngang hàng nhau. Câu "xe điện hay xe xăng" chỉ có nghĩa
với người **đã** vào được bên trong. Hỏi nó với người còn đứng ngoài cổng thì
hỏi cũng bằng thừa.

Python nói chữ "bên trong" bằng đúng thứ bạn đã học ở Realm 0: **thụt lề**.
Dòng nào lùi vào thì dòng đó nằm trong thân của `if` phía trên.

Và đây là chỗ mở ra một cánh cửa: một dòng `if` cũng là một dòng lệnh như mọi
dòng lệnh khác. Đặt được `print` trong thân một `if` thì cũng đặt được một `if`
nữa vào đúng chỗ đó.

Cách viết này gọi là **lồng nhau** — tiếng Anh là *nested if*. Đó là từ để bạn
tra cứu khi cần.
::::

::::example{#so-chi-tieu-hai-tang}
Byte đang ghi sổ chi tiêu. Mỗi ngày tiêu quá 200 nghìn thì sổ nhắc một câu. Và
trong **những ngày đã bị nhắc ấy**, ngày nào quá 500 nghìn thì đáng ghi chú
riêng ra.

```python title=readonly
tien_hom_nay = 250000

if tien_hom_nay > 200000:
    print("Hôm nay tiêu hơi nhiều.")
    if tien_hom_nay > 500000:
        print("Ngày này phải ghi chú lại.")
print("Đã ghi vào sổ.")
```

Máy in ra hai dòng:

```text
Hôm nay tiêu hơi nhiều.
Đã ghi vào sổ.
```

Cách chắc chắn nhất để đọc đoạn này là nhìn **cột lề** của từng dòng:

- Cột 0 (sát lề trái): `if tien_hom_nay > 200000:` và `print("Đã ghi vào sổ.")`.
  Hai dòng này không nằm trong thân ai cả.
- Cột 4: `print("Hôm nay tiêu hơi nhiều.")` và `if tien_hom_nay > 500000:`. Cả
  hai đều là thân của `if` ngoài, và chúng chạy theo thứ tự từ trên xuống.
- Cột 8: `print("Ngày này phải ghi chú lại.")`. Dòng này là thân của `if` trong.

Bây giờ đi lại đúng đường máy đi. `250000 > 200000` cho `True`, nên máy bước
vào thân tầng ngoài. Việc đầu tiên trong thân là in câu nhắc. Việc thứ hai là
một câu hỏi nữa: `250000 > 500000` — lần này `False`, nên dòng cột 8 bị bỏ qua.
Xong thân tầng ngoài, máy về lại cột 0 và in câu cuối.

Chỗ đáng nhớ nằm ở câu hỏi thứ hai: nó **được hỏi** vì câu hỏi thứ nhất đã
đúng. Nếu hôm nay chỉ tiêu 150 nghìn thì máy không chỉ bỏ qua câu nhắc, nó bỏ
qua luôn cả cái câu hỏi 500 nghìn — y như người không vào được cổng thì không
đọc tới tấm biển dưới hầm.
::::

::::predict{#ngay-tieu-it commitOnce}
Đổi một con số thôi: hôm nay Byte chỉ tiêu 150 nghìn. Để ý là điều kiện của
tầng trong đã đổi thành `> 100000`, và `150000 > 100000` là một câu **đúng**.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python
tien_hom_nay = 150000

if tien_hom_nay > 200000:
    print("Hôm nay tiêu hơi nhiều.")
    if tien_hom_nay > 100000:
        print("Ghi chú lại ngày này.")
print("Đã ghi vào sổ.")
```

:::opt{correct}
Chỉ một dòng: Đã ghi vào sổ.
:::

:::opt
Hai dòng: Ghi chú lại ngày này, rồi Đã ghi vào sổ.
::why
Gần đúng ở chỗ bạn kiểm điều kiện tầng trong rất chính xác: `150000 > 100000`
cho `True` thật. Nếu dòng `if` ấy đứng một mình sát lề trái thì bạn đã đoán
trúng hoàn toàn.

Chỗ lệch nằm ở chỗ đứng của nó. Dòng `if` thứ hai lùi vào bốn dấu cách, nghĩa
là nó **nằm trong thân** của `if` thứ nhất — nó là một việc mà máy chỉ làm khi
đã bước vào được thân ấy. Ở đây `150000 > 200000` cho `False`, nên máy nhảy qua
nguyên cả khối lùi vào, không đọc tới câu hỏi thứ hai lần nào.

Một điều kiện đúng vẫn có thể không được hỏi, nếu nó nằm ở chỗ máy không đi tới.
::
:::

:::opt
Không in ra dòng nào cả
::why
Gần đúng, và gần hơn bạn tưởng: bạn nhận ra `150000 > 200000` là sai nên toàn
bộ khối lùi vào bị bỏ qua. Phần đó bạn suy luận đúng từ đầu đến cuối, kể cả
tầng trong.

Chỗ lệch nằm ở dòng cuối cùng. `print("Đã ghi vào sổ.")` viết **sát lề trái**,
cùng cột với `if` ngoài. Nó không thuộc thân của ai, nên nó chạy trong mọi
trường hợp — hôm nay tiêu nhiều hay tiêu ít thì sổ vẫn được ghi.
::
:::

:::opt
Máy báo lỗi vì một `if` không được đặt bên trong một `if` khác
::why
Gần đúng ở chỗ bạn đang cảnh giác với lỗi khi thấy một cách viết lạ mắt — thói
quen này giúp bạn rất nhiều ở Realm 0 với `SyntaxError`.

Chỗ lệch: Python không giới hạn chuyện này. Thân của một `if` chứa được mọi
loại dòng lệnh, kể cả một `if` khác, và trong `if` khác ấy lại chứa được một
`if` nữa. Mỗi tầng thêm bốn dấu cách. Máy hiểu đoạn code này trọn vẹn — nó chỉ
chọn không chạy phần bên trong mà thôi.
::
:::
::::

::::explain{#lech-mot-cot-la-doi-nghia}
Vì "bên trong" được nói bằng thụt lề, nên bốn dấu cách ở đây không phải chuyện
trình bày cho đẹp mắt. Lệch một cột là đổi hẳn ý nghĩa.

Hai đoạn dưới đây gồm đúng cùng những chữ như nhau, chỉ khác chỗ đứng của dòng
`if` thứ hai:

```python
tien = 150000

if tien > 200000:
    print("Tiêu hơi nhiều.")
    if tien > 100000:
        print("Có ghi chú.")
```

```python
tien = 150000

if tien > 200000:
    print("Tiêu hơi nhiều.")
if tien > 100000:
    print("Có ghi chú.")
```

Đoạn trên không in gì cả. Đoạn dưới in `Có ghi chú.`

Ở đoạn dưới, `if` thứ hai đứng sát lề trái nên nó là một **ngã ba riêng**, luôn
được hỏi, không phụ thuộc gì vào ngã ba phía trên. Ở đoạn trên, nó là câu hỏi
thứ hai *dành cho những ai đã qua được câu hỏi thứ nhất*.

Hai ý nghĩa khác hẳn nhau, và thứ phân biệt chúng chỉ là bốn dấu cách.
::::

::::code{#them-mot-tang}
Sổ chi tiêu hôm nay ghi 600 nghìn. Byte muốn: **trong số những ngày đã tiêu quá
200 nghìn**, ngày nào quá 500 nghìn thì in ra một câu ghi chú riêng.

Dòng `print` đã nằm sẵn ở cột 8. Hãy viết dòng còn thiếu ở cột 4 để câu hỏi thứ
hai nằm đúng bên trong thân của câu hỏi thứ nhất.

```python title=starter
tien_hom_nay = 600000

if tien_hom_nay > 200000:
    ___
        print("Ngày kỷ lục, ghi chú lại.")
```

```python title=solution
tien_hom_nay = 600000

if tien_hom_nay > 200000:
    if tien_hom_nay > 500000:
        print("Ngày kỷ lục, ghi chú lại.")
```

```python title=test
# Chấm bằng OUTPUT: với 600000, cả hai tầng điều kiện đều đúng nên câu ghi chú
# phải hiện ra. Người học chưa biết viết assert nên khối này chỉ khẳng định
# chương trình chạy được tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm ở cột 4 — lùi vào bốn dấu cách so với `if` phía trên. Đó là chỗ dành cho một dòng nằm **trong thân** của `if` đầu tiên.
- kind: strategy
  body: Dòng bạn cần viết là một câu hỏi có–không nữa, hỏi xem số tiền có quá 500000 không. Nó có đủ ba phần như mọi câu hỏi kiểu này: từ khoá mở đầu, điều kiện, và dấu hai chấm cuối dòng.
- kind: one-line
  body: "Thay `___` bằng `if tien_hom_nay > 500000:`, giữ nguyên bốn dấu cách phía trước."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: Ngày kỷ lục, ghi chú lại.
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai tầng câu hỏi. Tầng trong chỉ mở miệng khi tầng ngoài đã gật đầu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hôm nay bạn mới lồng hai tầng. Nhưng chẳng có gì ngăn tầng thứ ba, thứ tư: sổ
chi tiêu thật hay có kiểu "nếu là ngày trong tuần, mà lại có đi chợ, mà số tiền
lại quá 200 nghìn thì...".

Giả sử bạn mở một file của người khác và thấy một dòng `print` nằm sâu ba tầng
thụt lề — mười hai dấu cách. Dòng đó chạy trong trường hợp nào?

Bạn không thể trả lời chỉ bằng cách nhìn vào chính dòng ấy. Vậy phải nhìn vào
đâu, và nhìn theo thứ tự nào?

Bài sau trả lời, và câu trả lời gọn hơn bạn nghĩ.
::::

::::checkpoint{mastery=0.8}
::::
