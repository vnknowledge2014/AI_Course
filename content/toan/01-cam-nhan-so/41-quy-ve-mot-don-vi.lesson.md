---
id: toan.cam-nhan-so.quy-ve-mot-don-vi
title: Quy về một đơn vị
summary: Hai bộ đôi không chép được cho nhau thì kéo cả hai về cùng một cỡ — "mấy hạt trên một luống" — rồi mới so.
locale: vi
track: toan
module: cam-nhan-so
order: 41
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.unit-rate]
requires: [math.ratio, core.division, core.float, core.variable, core.print-variable, core.boolean, ctrl.comparison]
concepts: [math.ti-le, math.don-vi-ghep]
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
Hai vườn khác cỡ. Mình kéo cả hai về đúng một luống rồi hẵng so.
::::

::::explain{#hai-bo-doi-khong-chep-duoc-cho-nhau}
Bài trước để lại hai bộ đôi đếm được ngoài vườn:

- Vườn Byte: `90 : 6` — chín mươi hạt trên sáu luống.
- Vườn An: `75 : 5` — bảy mươi lăm hạt trên năm luống.

Câu hỏi là vườn nào **gieo dày hơn** — mỗi luống chịu nhiều hạt hơn.

Cách của bài trước không dùng được ở đây. Ở đó bạn luôn có một bộ đôi mẫu rồi
chép nó lên: `2 : 5` chép bốn lần ra `8 : 20`. Còn `75 : 5` thì chép một lần
chưa tới `90 : 6`, chép hai lần đã vọt qua. Không có số lượt **nguyên** nào
biến bộ đôi này thành bộ đôi kia, nên chép tới chép lui cũng không đặt được
chúng cạnh nhau mà đọc.

So thẳng từng vế cũng không xong. 90 nhiều hạt hơn 75 — đúng, nhưng vườn Byte
cũng nhiều luống hơn. Hai vế đều lớn hơn thì chưa nói được gì về chuyện dày
mỏng.

Bài 34 đã gặp đúng thế bí này một lần rồi, hồi so `2/3` với `3/4`: hai lượng
đang đo bằng hai cái thước khác nhau thì không đọ được. Cách gỡ hôm ấy là **quy
cả hai về cùng một thước**. Hôm nay cũng vậy, chỉ khác chỗ cái thước là gì.
::::

::::explain{#quy-ve-mot-luong}
Cái thước chung mà hai vườn nào cũng có: **một luống**.

Làm nhỏ trước cho dễ nhìn. Byte có một khoảnh con: **12 hạt trên 3 luống**.
Kéo bộ đôi `12 : 3` xuống cho tới khi vế luống còn đúng `1`.

Sơ đồ dải — cắt 12 hạt thành 3 phần bằng nhau, mỗi luống lấy một phần:

```text
12 hạt  [████████████]
         ↓ chia đều cho 3 luống
luống 1  [████]   luống 2  [████]   luống 3  [████]
```

Thanh số kép — bộ đôi ấy chép ngược lại về phía 1:

```text
luống :   1    2    3
hạt   :   4    8   12
```

Mô hình vùng — xếp 12 hạt thành mảng 3 hàng, mỗi hàng là một luống:

```text
luống 1  • • • •
luống 2  • • • •
luống 3  • • • •
```

Ba bức tranh, một kết quả: `12 : 3` và `4 : 1` là **cùng một bộ đôi**. Việc vừa
làm là bài 40 chạy ngược — thay vì nhân cả hai vế lên, ta chia cả hai vế xuống
cho cùng một số, và quan hệ vẫn nguyên vẹn. Bài 26 gọi việc đó bằng tên riêng
của nó: chia đều 12 cho 3 phần.

Vế phải bằng `1` là chỗ toàn bộ bài này nằm ở đó. Khi vế luống bằng 1, con số
còn lại đọc thành một câu duy nhất:

> **4 hạt trên một luống.**

"Hạt **trên một** luống" là một **đơn vị ghép** — một cái thước mới, ghép từ
hai đơn vị cũ. Nó không phải hạt, cũng không phải luống; nó là *độ dày của một
cái vườn*. Và vì mọi vườn đều quy về được cái thước này, hai vườn bất kỳ cũng
đọ được với nhau bằng đúng một con số.

Con số ấy có tên: **tỉ lệ**.
::::

::::predict{#doan-cau-tra-loi commitOnce}
Byte đem đúng cách ấy ra hỏi máy về hai cái vườn ban đầu.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
day_byte = 90 / 6
day_an = 75 / 5
print(day_byte > day_an)
```

:::opt{correct}
False
:::

:::opt
True
::why
Gần đúng ở chỗ bạn so hai vườn theo đống hạt, và đống hạt của Byte lớn hơn
thật: 90 nhiều hơn 75, không bàn cãi. Nếu câu hỏi là "vườn nào tốn nhiều hạt
giống hơn" thì bạn đã trả lời đúng.

Chỗ lệch nằm ở thứ mà hai cái tên đang giữ. Sau hai dòng đầu, `day_byte` không
còn giữ 90 hạt nữa — nó giữ số hạt **trên một luống**, tức là 90 đã bị chia cho
6. Cùng lúc đó 75 cũng bị chia cho 5. Câu hỏi ở dòng cuối là câu hỏi về độ dày,
và độ dày thì phải tính xong mới biết bên nào hơn.
::
:::

:::opt
15.0
::why
Gần đúng ở chỗ bạn tính ra đúng con số mà hai vế cho ra — `90 / 6` thật sự là
`15.0`, và đó là con số quan trọng nhất trong cả bài. Bạn làm xong phần khó.

Chỗ lệch là ở việc dấu `>` không chọn hộ bạn cái lớn hơn. Realm 0 bài 24 đã dạy
nó là một **câu hỏi có–không**: thứ đi ra khỏi nó luôn là `True` hoặc `False`,
chưa bao giờ là một con số. Muốn thấy con số thì in thẳng `day_byte`, đừng bọc
nó trong một câu hỏi.
::
:::

:::opt
15.0 > 15.0
::why
Gần đúng ở chỗ bạn hình dung máy thay từng cái tên bằng giá trị của nó trước
khi in — và nó làm đúng như vậy thật, R0 bài 13 đã cho thấy. Bạn đọc đúng bước
đầu tiên.

Chỗ lệch là máy không dừng lại ở đó. Thay tên xong, nó vẫn còn một câu hỏi trên
tay và phải **trả lời** câu hỏi ấy; thứ nó đưa cho `print` là câu trả lời, chứ
không phải câu hỏi đã thay tên. Muốn thấy đúng dòng chữ ấy thì phải viết nó
trong dấu nháy, và lúc đó máy chỉ chép lại chứ không tính gì.
::
:::
::::

::::example{#hoi-thang-cai-may}
Chạy lên, thêm một dòng in ra hai con số:

```python title=readonly
day_byte = 90 / 6
day_an = 75 / 5

print(day_byte, day_an)
print(day_byte > day_an)
```

```text
15.0 15.0
False
```

Hai vườn trông khác hẳn nhau — 90 hạt 6 luống với 75 hạt 5 luống — mà quy về
một luống thì **bằng nhau đúng khít**: cả hai đều 15 hạt trên một luống. Không
vườn nào dày hơn vườn nào.

Đó là điều mà hai bộ đôi thô không cho thấy được. Phải kéo cả hai về cùng một
cỡ mới đọc ra.

Cái dấu chấm trong `15.0` không có gì lạ: R0 bài 15 đã nói phép chia bằng dấu
`/` luôn trả về một số có phần lẻ, kể cả khi phần lẻ bằng không. Ở đây điều đó
lại tiện: độ dày của vườn hiếm khi tròn, và bạn sẽ cần chỗ cho phần lẻ ấy.
::::

::::explain{#mot-ky-hieu-ba-cach-doc}
Trước khi tự tay làm, dừng lại ở một chuyện đáng để ý.

Bạn vừa viết `90 : 6` — trong Python là `90 / 6` — và đọc nó là *"15 hạt trên
một luống"*. Nhưng bạn đã gặp đúng ký hiệu ấy hai lần rồi, mỗi lần đọc một kiểu
khác hẳn:

- **Bài 26** đọc là: *chia 90 hạt đều vào 6 luống, mỗi luống được mấy hạt?*
  Câu trả lời mang đơn vị **hạt**.
- **Bài 27** đọc là: *6 lọt vào 90 được mấy lần?* Câu trả lời mang đơn vị
  **lần**.
- **Bài này** đọc là: *cứ một luống thì mấy hạt?* Câu trả lời mang một đơn vị
  ghép, **hạt trên một luống**.

Ba câu hỏi khác nhau, ba đơn vị khác nhau cho câu trả lời, mà trên giấy chỉ có
đúng một dòng `90 / 6` và đúng một con số 15. Hãy giữ chuyện này trong đầu —
cuối bài sau nó sẽ có việc.

Và một chỗ dễ trượt chân: **thứ tự hai vế vẫn có nghĩa**, y như bài 40. `90 / 6`
là hạt trên một luống. Viết ngược thành `6 / 90` thì máy vẫn trả lời, ra
`0.0666…`, và con số ấy cũng đúng — nó là *số luống trên một hạt*. Hai câu đều
thật, chỉ là bạn phải biết mình đang hỏi câu nào.
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Ba vườn lần này. Vẫn đúng một cách: kéo mỗi vườn về một luống.
::::

::::code{#ba-vuon-mot-thuoc}
Ba nhà cùng gieo xong, đếm được:

- **Vườn nhà Tú**: 84 hạt trên 7 luống.
- **Vườn nhà Hà**: 60 hạt trên 4 luống.
- **Vườn nhà Sen**: 96 hạt trên 8 luống.

Sáu con số đếm được đã đặt sẵn tên ở đầu bài. Điền ba chỗ trống bằng độ dày của
từng vườn — số hạt trên một luống — rồi để máy trả lời hai câu hỏi so sánh ở
hai dòng cuối.

Bài chấm bằng **cả ba vườn và cả hai câu hỏi**. Hai câu hỏi ấy được chọn để cho
ra hai câu trả lời **ngược nhau**: một câu `True`, một câu `False`. Nên gõ cứng
một con số vào cả ba chỗ trống thì hỏng ít nhất một vườn, và không có con số
nào làm cả hai câu hỏi cùng đúng.

```python title=starter
hat_tu = 84
luong_tu = 7

hat_ha = 60
luong_ha = 4

hat_sen = 96
luong_sen = 8

day_tu = ___
day_ha = ___
day_sen = ___

print(day_tu, day_ha, day_sen)
print(day_ha > day_tu)
print(day_tu > day_sen)
```

```python title=solution
hat_tu = 84
luong_tu = 7

hat_ha = 60
luong_ha = 4

hat_sen = 96
luong_sen = 8

day_tu = hat_tu / luong_tu
day_ha = hat_ha / luong_ha
day_sen = hat_sen / luong_sen

print(day_tu, day_ha, day_sen)
print(day_ha > day_tu)
print(day_tu > day_sen)
```

```python title=test
# Ba vườn, ba phép quy về một luống. Hai câu so sánh cho hai câu trả lời ngược
# nhau, nên không có đáp án gõ cứng nào qua được cả hai.
assert day_tu == 12.0, "84 hạt trên 7 luống — quy về một luống phải ra 12"
assert day_ha == 15.0, "60 hạt trên 4 luống — quy về một luống phải ra 15"
assert day_sen == 12.0, "96 hạt trên 8 luống — quy về một luống phải ra 12"
# Hai vườn khác hẳn cỡ mà cùng một độ dày: đây đúng là điều bài 40 hứa và bài
# này kiểm được — chia cả hai vế cho cùng một số thì quan hệ không đổi.
assert day_tu == day_sen, "vườn Tú và vườn Sen khác cỡ nhưng phải dày như nhau"
assert day_ha > day_tu, "vườn Hà phải dày hơn vườn Tú"
# Quy về một luống rồi chép trở lại đủ số luống thì phải ra đúng đống hạt cũ —
# không có hạt nào sinh ra hay mất đi trong lúc quy đổi.
assert day_tu * luong_tu == hat_tu, "12 hạt một luống, 7 luống — chép lại phải ra đúng 84 hạt"
assert day_ha * luong_ha == hat_ha, "15 hạt một luống, 4 luống — chép lại phải ra đúng 60 hạt"
```

:::hints
- kind: attention
  body: Mỗi vườn đã có sẵn hai cái tên — một tên giữ số hạt, một tên giữ số luống. Chỗ trống hỏi độ dày, mà độ dày luôn tính trên đúng MỘT luống.
- kind: strategy
  body: Kéo bộ đôi xuống cho vế luống còn 1 nghĩa là chia cả hai vế cho số luống. Vế luống chia cho chính nó thành 1, nên chỉ còn phải viết ra vế hạt đã chia. Dấu chia trong Python là `/`, và nó luôn cho ra số có phần lẻ.
- kind: one-line
  body: "Viết `hat_tu / luong_tu`, rồi `hat_ha / luong_ha`, rồi `hat_sen / luong_sen`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một phép chia thật — cái tên giữ số hạt chia cho cái tên giữ số luống — chứ không phải con số kết quả gõ sẵn
  requireAst:
  - kind: uses-operator, target: /, min: 3
  - kind: uses-name, target: luong_tu, min: 1
  - kind: uses-name, target: luong_ha, min: 1
  - kind: uses-name, target: luong_sen, min: 1
- tier: output
  match: regex
  expect: ^12\.0 15\.0 12\.0\nTrue\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Vườn Tú và vườn Sen khác cỡ hẳn, mà luống nào cũng đúng 12 hạt.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sáng mai Byte gieo tiếp ở khoảnh mới, và ghi buổi gieo vào sổ. Trong túi còn
**3 hạt lẻ** chưa gieo. Ngoài khoảnh có **4 luống nhỏ**, **mỗi luống 5 hạt**.

Byte muốn cả hai chuyện nằm gọn trong một dòng, nên viết:

```text
3 + 4 × 5
```

An đọc dòng ấy từ trái sang phải, đúng như đọc chữ: 3 cộng 4 được 7, rồi 7 nhân
5 được 35. An nói vườn có 35 hạt.

Byte đọc kiểu khác và ra 23.

Cùng một dòng chữ, hai người, hai kết quả. Mà ngoài khoảnh thì chỉ có đúng một
số hạt — đi đếm tay là ra, và nó không thể vừa 35 vừa 23.

Ai đúng, và quan trọng hơn: **vì sao** người kia sai, khi cách đọc từ trái sang
phải nghe chẳng có gì vô lý? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
