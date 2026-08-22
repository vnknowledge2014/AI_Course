---
id: toan.cam-nhan-so.tru-la-cong-so-doi
title: Trừ là cộng với số đối
summary: Mỗi số có một số đối ở phía bên kia vạch 0. Lùi b bước và tiến (-b) bước là cùng một mũi tên, nên phép trừ tan hẳn vào phép cộng.
locale: vi
track: toan
module: cam-nhan-so
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.additive-inverse]
requires: [math.negative-number, math.compare-on-number-line, core.arithmetic, core.variable, core.print-variable, ctrl.comparison, core.output]
concepts: [math.thanh-so, math.so-doi, math.mui-ten-cong]
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
Mình chỉ còn nhớ một phép thôi. Phép kia mình gấp gọn vào trong nó rồi.
::::

::::explain{#dem-xem-dang-giu-may-luat}
Bài trước để lại một câu hỏi: nếu phép trừ nào cũng lùi được, thì trừ có còn là
một phép riêng nữa không?

Trước khi trả lời, đếm xem bạn đang phải nhớ mấy luật.

- **Cộng** (bài 13): đứng ở a, bước sang **phải** b bước.
- **Trừ** (bài 14): đứng ở a, bước sang **trái** b bước.

Hai luật, hai chiều. Nghe thì gọn, nhưng mỗi luật là một chỗ để nhớ nhầm, và
càng về sau — khi số âm chen vào giữa — càng dễ nhầm.

Bây giờ nhìn lại thanh số sau bài 16. Mỗi chỗ bên phải vạch 0 có một chỗ đối
diện bên trái, **cách vạch 0 đúng bằng nhau**:

```text
      ┌──────── 5 bước ────────┬──────── 5 bước ────────┐
   ───┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬───
      -5   -4   -3   -2   -1    0    1    2    3    4    5
      ▲                                                 ▲
      └─────────────── một cặp song sinh ───────────────┘
```

`5` và `-5` là một cặp. `2` và `-2` là một cặp. Còn `0` thì bạn song sinh của nó
là chính nó — nó đứng ngay chỗ hai nửa gặp nhau, cách chính mình không bước nào.

Cặp ấy có tên: hai số trong một cặp gọi là **số đối** của nhau. Số đối của `5`
là `-5`; số đối của `-5` là `5`, vì cái cặp không có chiều — đứng ở đầu nào nhìn
sang đầu kia cũng là cùng một cặp.
::::

::::explain{#mot-mui-ten-hai-cach-goi}
Có một cách vẽ phép cộng làm mọi thứ lộ ra: vẽ nó thành **mũi tên**.

Mỗi phép cộng dán một mũi tên vào chỗ bạn đang đứng. Mũi tên ấy có hai đặc điểm,
và cả hai đều nằm sẵn trong cái tên hai mảnh của bài 16:

- **dài bao nhiêu** — bằng khoảng cách từ số đó tới vạch 0;
- **chỉ về đâu** — bằng phía mà số đó nằm.

Vậy thì:

- `+ 5` là mũi tên dài 5, chỉ sang **phải**.
- `+ (-5)` là mũi tên dài 5, chỉ sang **trái** — vì `-5` nằm bên trái vạch 0.
- `- 5` (phép trừ, theo bài 14) là lùi 5 bước, tức mũi tên dài 5 chỉ sang
  **trái**.

Hai dòng dưới cùng mô tả **đúng một mũi tên**. Không phải hai mũi tên tình cờ
cho cùng kết quả trong vài ví dụ — chúng là một, cùng độ dài, cùng chiều. Dán
một mũi tên vào chỗ nào thì cũng ra chỗ ấy, dù bạn xuất phát từ đâu.

Viết luật ra:

> `a − b = a + (−b)`

Đọc bằng lời: **lùi b bước cũng là tiến số đối của b bước.** Nên từ đây bạn
không cần hai luật nữa — chỉ cần phép cộng, cộng với việc biết mũi tên chỉ về
phía nào.

Đó cũng là câu trả lời cho bài trước: phép trừ không biến mất, nó **tan vào**
phép cộng. Ta vẫn viết dấu trừ cho gọn, nhưng nó không còn là một luật riêng
phải nhớ.
::::

::::byte{trigger=enter mood=thinking pose=point-stage}
Mũi tên đang chỉ sang trái. Bảo mình lùi nó, thì mình quay nó lại thôi.
::::

::::example{#thu-ba-cap}
Hỏi thẳng cái máy ba cặp, mỗi cặp một dòng trừ và một dòng cộng số đối:

```python title=readonly
print(12 - 5)
print(12 + (-5))

print(3 - 8)
print(3 + (-8))

print(12 - (-5))
print(12 + 5)
```

Máy in ra:

```text
7
7
-5
-5
17
17
```

Cặp một là chỗ quen: đứng ở 12, mũi tên dài 5 chỉ sang trái, dừng ở 7.

Cặp hai là chỗ bài 16 vừa mở ra: đứng ở 3, mũi tên dài 8 chỉ sang trái, đi qua
vạch 0 và dừng ở `-5`. Hai cách viết vẫn khớp nhau.

Cặp ba là chỗ đáng nhìn kỹ nhất. `12 - (-5)`: luật bảo lùi `-5` bước. Mà mũi tên
của `-5` là mũi tên dài 5 **đang chỉ sang trái** — lùi nó nghĩa là quay nó
ngược lại, thành mũi tên dài 5 chỉ sang **phải**. Vậy `12 - (-5)` chính là
`12 + 5`, và cả hai ra `17`.

Không có luật mới nào ở cặp ba. Vẫn đúng một câu `a − b = a + (−b)`, với `b` là
`-5` và số đối của `-5` là `5`.
::::

::::predict{#doan-truoc-khi-chay commitOnce}
Byte viết hai dòng, một dòng trừ và một dòng cộng số đối.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(10 - 6)
print(10 + (-6))
```

:::opt{correct}
Hai dòng cùng in ra 4
:::

:::opt
Dòng trên in 4, dòng dưới in 16
::why
Gần đúng ở chỗ bạn dùng một quy tắc đã đúng với bạn hàng nghìn lần: thấy dấu
`+` thì đống to ra. Quy tắc ấy thật — trong phạm vi những thứ được cộng vào
**đứng bên phải vạch 0**. Đổ thêm 6 hạt vào nắm hạt thì nắm ấy nhiều lên, không
cãi được.

Chỗ lệch: `-6` không nằm bên phải vạch 0. Cộng nó vào là dán một mũi tên dài 6
chỉ sang **trái**, nên chỗ đến nằm bên trái chỗ xuất phát. Nói cho gọn: cộng
làm số lớn lên khi và chỉ khi thứ được cộng vào là số dương.
::
:::

:::opt
Dòng trên in 4, dòng dưới in -4
::why
Gần đúng ở chỗ bạn để ý đúng cái dấu trừ trong `(-6)` và tin rằng nó phải để lại
dấu vết ở kết quả. Nó có để lại thật — nếu không thì viết cái dấu ấy làm gì.

Chỗ lệch nằm ở chỗ nó để lại dấu vết vào đâu. Dấu ấy nói về **chiều của mũi
tên**, chứ không dán thẳng lên kết quả. Đứng ở vạch 10 mà đi sang trái 6 bước
thì dừng ở vạch 4 — vẫn còn bên phải vạch 0, nên kết quả không mang dấu. Kết
quả chỉ mang dấu âm khi chỗ đến rơi qua bên trái vạch 0, như `3 + (-8)` ra `-5`
ở ví dụ ban nãy.
::
:::

:::opt
Dòng trên in 4, dòng dưới báo lỗi vì dấu `+` và dấu `-` đứng cạnh nhau
::why
Gần đúng ở chỗ bạn nhớ đúng một luật viết mà máy giữ rất chặt: hai phép tính
không được đứng liền nhau. Gõ `10 + * 6` thì máy từ chối ngay trước khi chạy,
đúng như bài lỗi cú pháp ở Realm 0 đã cho thấy.

Chỗ lệch nằm ở vai của dấu `-` trong `(-6)`. Ở đó nó không phải một phép tính,
nó là **một phần tên** của con số — tên của cái chỗ cách vạch 0 sáu bước về bên
trái, đúng như bài 16 vừa đặt. Cặp ngoặc là cách nói cho **mắt bạn** thấy rõ
điều đó: bên trong ngoặc là một con số, không phải một việc. Máy thì không cần
cặp ngoặc ấy — `10 + -6` nó vẫn đọc được — nhưng người đọc thì cần, nên ta cứ
viết.
::
:::
::::

::::explain{#mot-luat-thay-cho-hai}
Ba thứ để mang theo.

**Một:** mỗi số có đúng một **số đối** — cùng khoảng cách tới vạch 0, khác phía.
Số đối của số đối là chính nó. Số đối của `0` là `0`.

**Hai:** `a − b` và `a + (−b)` không phải hai phép tính cho cùng đáp số. Chúng
là **một phép tính viết bằng hai cách**, vì chúng dán đúng cùng một mũi tên.

**Ba:** cộng một số với số đối của nó thì về vạch 0 — `5 + (-5)` ra `0`: đứng ở
vạch 5, dán mũi tên dài 5 chỉ sang trái, mà 5 cũng đúng là quãng từ vạch 5 về
tới vạch 0, nên chân dừng ngay tại 0. Đó là lý do cặp song sinh ấy đáng có một
cái tên riêng.
::::

::::code{#ba-cap-mot-luat}
Byte muốn tự tay kiểm cái luật `a − b = a + (−b)` trên ba cặp số, và ba cặp này
được chọn để rơi vào **ba chỗ khác nhau** trên thanh số:

- cặp một dừng lại khi còn bên phải vạch 0;
- cặp hai đi qua hẳn sang bên trái;
- cặp ba lùi một mũi tên đang chỉ sang trái, nên chỗ đến lại xa hơn về bên phải.

Dòng trừ đã viết sẵn. Việc của bạn là viết dòng cộng cho khớp — mỗi chỗ trống là
**số đối** của con số bị trừ ở dòng ngay trên nó. Chép cứng một con số vào cả ba
chỗ trống thì nhiều nhất cũng chỉ qua được một cặp.

```python title=starter
# Vườn của Byte đo bằng phân, mặt đất là vạch 0.
# Luật của bài: lùi b bước cũng là tiến số đối của b bước.
# Dấu phẩy trong `print(a, b)` cho phép in hai giá trị trên một dòng,
# cách nhau một dấu cách.

# Cặp một: từ vạch 14 lùi 6 bước.
tru_mot = 14 - 6
cong_mot = 14 + ___

# Cặp hai: từ vạch 4 lùi 9 bước.
tru_hai = 4 - 9
cong_hai = 4 + ___

# Cặp ba: từ vạch 4 lùi -9 bước — quay ngược cái mũi tên đang chỉ sang trái.
tru_ba = 4 - (-9)
cong_ba = 4 + ___

print(tru_mot, cong_mot)
print(tru_hai, cong_hai)
print(tru_ba, cong_ba)
```

```python title=solution
# Vườn của Byte đo bằng phân, mặt đất là vạch 0.
# Luật của bài: lùi b bước cũng là tiến số đối của b bước.
# Dấu phẩy trong `print(a, b)` cho phép in hai giá trị trên một dòng,
# cách nhau một dấu cách.

# Cặp một: từ vạch 14 lùi 6 bước.
tru_mot = 14 - 6
cong_mot = 14 + (-6)

# Cặp hai: từ vạch 4 lùi 9 bước.
tru_hai = 4 - 9
cong_hai = 4 + (-9)

# Cặp ba: từ vạch 4 lùi -9 bước — quay ngược cái mũi tên đang chỉ sang trái.
tru_ba = 4 - (-9)
cong_ba = 4 + 9

print(tru_mot, cong_mot)
print(tru_hai, cong_hai)
print(tru_ba, cong_ba)
```

```python title=test
# Ba cặp rơi vào ba chỗ khác nhau trên thanh số (8, -5, 13), nên một con số
# chép cứng vào cả ba chỗ trống chỉ qua được nhiều nhất một cặp.
#
# Ba câu khẳng định đầu chấm bài; ba câu sau chốt lại chính điều bài vừa dạy,
# để cổng kiểm đỏ lên chứ không dạy sai lặng lẽ nếu có ngày luật này bị sửa.
assert cong_mot == tru_mot == 8, "14 lùi 6 bước, hay 14 tiến -6 bước, đều tới vạch 8"
assert cong_hai == tru_hai == -5, "4 lùi 9 bước thì đi qua vạch 0 và tới -5"
assert cong_ba == tru_ba == 13, "lùi -9 bước là quay mũi tên lại, thành tiến 9 bước"

assert 7 + (-7) == 0, "cộng một số với số đối của nó thì về đúng vạch 0"
assert 7 - 0 == 7 + 0, "số đối của 0 là chính 0, nên trừ 0 và cộng 0 là cùng một mũi tên dài 0"
assert 4 - (-9) == 4 + 9, "số đối của -9 là 9, nên hai cách viết là một"
```

:::hints
- kind: attention
  body: Nhìn dòng trừ ngay phía trên mỗi chỗ trống. Con số đứng sau dấu trừ ở dòng đó chính là con số bạn phải tìm số đối.
- kind: strategy
  body: Số đối là bạn song sinh ở phía bên kia vạch 0 — cùng khoảng cách, khác phía. Số đối của một số dương thì mang dấu trừ; số đối của một số đã mang dấu trừ thì bỏ dấu ấy đi. Cặp ba là chỗ luật này trả công nhiều nhất.
- kind: one-line
  body: "Ba chỗ trống lần lượt là `(-6)`, `(-9)` và `9`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^8 8\n-5 -5\n13 13\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một luật thay cho hai. Từ giờ mình chỉ còn phải nhớ mũi tên chỉ về phía nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Phép cộng vừa nuốt gọn phép trừ. Nhưng phép cộng vẫn làm việc theo đúng một
kiểu: gộp từng ít một, mỗi lần một mũi tên.

Byte ra vườn đếm cây. Mỗi luống trồng 8 cây, vườn có 5 luống. Cộng `8 + 8 + 8 +
8 + 8` là xong — năm mũi tên, dán liên tiếp, mất chừng mươi giây.

Sang vườn của bác Tư thì 50 luống. Vẫn mỗi luống 8 cây. Dán 50 cái mũi tên?

Cái vườn ấy có gì mà năm cái mũi tên đầu tiên chưa nói ra được — hay nó chỉ là
cùng một việc lặp lại, và có cách ghi lại việc lặp ấy cho gọn?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
