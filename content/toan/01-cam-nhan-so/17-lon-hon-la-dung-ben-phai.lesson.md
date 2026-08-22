---
id: toan.cam-nhan-so.lon-hon-la-dung-ben-phai
title: Lớn hơn nghĩa là đứng bên phải
summary: So sánh hai số là hỏi cái nào đứng bên phải trên thanh số — nên -2 lớn hơn -5, dù rễ sâu 5 phân mới là rễ ăn sâu hơn.
locale: vi
track: toan
module: cam-nhan-so
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.compare-on-number-line]
requires: [math.negative-number, ctrl.comparison, core.boolean, core.variable, core.output]
concepts: [math.thanh-so, math.so-sanh, math.khoang-cach-toi-khong]
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
Hỏi mình số nào lớn hơn, mình chỉ nhìn xem đứa nào đứng bên phải thôi.
::::

::::explain{#hai-cau-hoi-lau-nay-van-di-chung}
Bài trước để lại một câu hỏi: rễ ớt ở vạch `-5`, rễ húng ở vạch `-2`, số nào lớn
hơn?

Trước khi trả lời, phải tách ra **hai câu hỏi** mà tiếng Việt vẫn để dính vào
nhau:

- **Cái nào xa vạch 0 hơn?** Đây là câu hỏi về khoảng cách. Rễ ớt cách mặt đất
  năm phân, rễ húng hai phân — rễ ớt xa hơn. Ai cầm cái cọc đo cũng thấy.
- **Cái nào lớn hơn?** Đây là câu hỏi về **chỗ đứng** trên thanh số.

Suốt từ bài 1 tới bài 15, bạn chưa cần phân biệt hai câu hỏi ấy, và có lý do:
mọi con số bạn gặp đều nằm bên phải vạch 0. Ở nửa bên phải, hai câu hỏi luôn
cho cùng một câu trả lời. Nắm 5 hạt vừa nhiều hơn nắm 2 hạt, vừa nằm xa vạch 0
hơn, vừa đứng bên phải nó. Ba cách nói, một kết quả.

Bài 16 vừa mở nửa bên trái ra. Ở đó hai câu hỏi tách hẳn nhau, và chúng đi
ngược chiều: càng đi xa vạch 0 về bên trái thì càng lùi sâu về bên trái. Nên từ
giờ, hỏi câu nào thì phải nói rõ câu ấy.
::::

::::explain{#mot-luat-duy-nhat}
Luật của cả bài gói trong một câu:

> `a > b` đúng khi **a đứng bên phải b** trên thanh số.

Không có luật thứ hai, và cũng không có ngoại lệ cho nửa bên trái.

Thử lại nó trên chỗ cũ trước, để chắc là nó không phá thứ gì bạn đã có. Số 5
đứng bên phải số 2, nên `5 > 2` — đúng y như xưa. Luật mới không sửa câu trả lời
nào của luật cũ; nó chỉ nói được ở những chỗ mà luật cũ chưa với tới.

Bây giờ áp sang trái. Đứng ở vạch `-5`, muốn tới `-2` thì đi về phía nào? Sang
phải, ba bước. Vậy `-2` đứng bên phải `-5`, nên **`-2 > -5`**.

Vì sao lại chọn định nghĩa này, chứ không chọn "số nào xa vạch 0 hơn thì lớn
hơn"? Vì hai luật trong tay bạn phải khớp với nhau.

Bài 13 nói: **cộng là bước sang phải.** Cộng thêm thì phải lớn lên — không ai
chấp nhận một phép cộng làm số nhỏ đi.

Giả sử ta chọn "xa 0 hơn là lớn hơn". Đứng ở `-5`, cộng thêm 3, tức bước sang
phải ba bước, tới `-2`. Nhưng `-2` gần vạch 0 hơn, nên theo cách chọn ấy nó
**nhỏ hơn** `-5`. Cộng thêm mà số lại nhỏ đi. Hai luật đánh nhau ngay ở phép
tính đầu tiên.

Chọn "đứng bên phải là lớn hơn" thì hai luật khớp khít: cộng luôn đi sang phải,
mà sang phải luôn là lớn lên — ở cả hai nửa thanh số, không cần nhớ nửa nào.
::::

::::byte{trigger=enter mood=thinking pose=point-stage}
Cái cọc của mình đo cả hai cây. Rễ ớt sâu hơn, mà số của nó lại nhỏ hơn.
::::

::::example{#coc-do-hai-cay}
Byte đo hai cây trên cùng một cái cọc, mặt đất vẫn là vạch 0:

```text
    2 ┤
    1 ┤
    0 ┼──── mặt đất
   -1 ┤
   -2 ┤ ←── đầu rễ húng
   -3 ┤
   -4 ┤
   -5 ┤ ←── đầu rễ ớt
```

Đặt cái cọc nằm xuống thì "cao hơn" thành "bên phải hơn":

```text
   ───┬────┬────┬────┬────┬────┬────┬───
      -5   -4   -3   -2   -1    0    1
    rễ ớt          rễ húng    mặt đất
```

Hai bức tranh nói cùng một chuyện. Rễ ớt **ăn sâu hơn** — nó xa mặt đất hơn.
Rễ húng **mang số lớn hơn** — nó đứng bên phải trên thanh số, và nằm cao hơn
trên cái cọc. Hai câu đó không mâu thuẫn; chúng trả lời hai câu hỏi khác nhau.

Hỏi thẳng cái máy ba lần:

```python title=readonly
print(5 > 2)
print(-2 > -5)
print(-5 > -2)
```

Máy in ra:

```text
True
True
False
```

Dòng đầu là chỗ cũ, không có gì lạ. Dòng thứ hai là luật mới nói ra thành lời:
`-2` đứng bên phải `-5`. Dòng thứ ba là cùng câu hỏi ấy hỏi ngược lại, và máy
trả lời `False` — `-5` không đứng bên phải `-2`.
::::

::::predict{#doan-truoc-khi-chay commitOnce}
Byte ghi hai con số vào sổ vườn rồi hỏi máy một câu.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
re_ot = -5
re_hung = -2
print(re_ot > re_hung)
```

:::opt{correct}
False
:::

:::opt
True
::why
Gần đúng ở chỗ bạn dùng một quy tắc bạn tự kiểm được bằng tay hàng nghìn lần:
số 5 lớn hơn số 2, nên cái mang số 5 thì lớn hơn cái mang số 2. Quy tắc ấy
không sai — trong phạm vi những số **đứng bên phải vạch 0**, nơi "mang con số
to hơn" và "đứng bên phải hơn" luôn đi cùng nhau.

Chỗ lệch: qua bên trái vạch 0, hai chuyện ấy tách nhau và đi ngược chiều. `-5`
đúng là xa vạch 0 hơn `-2` thật, nhưng xa về phía **trái** — mà bên trái là
phía nhỏ đi. Muốn từ `-5` tới `-2` thì phải bước sang phải ba bước, nên `-2`
mới là số đứng bên phải.
::
:::

:::opt
-5
::why
Gần đúng ở chỗ bạn đọc dấu `>` như một câu hỏi "cái nào lớn hơn" và chờ máy chỉ
tay vào một trong hai con số. Ý muốn ấy rất hợp lý, và đó cũng đúng là việc bạn
đang cần biết.

Chỗ lệch nằm ở thứ đi ra khỏi dấu `>`. Ở Realm 0, bài về `True` và `False` đã
nói: dấu so sánh đặt ra một câu hỏi **có–không** — ở đây là "vế trái có đứng
bên phải vế phải không?". Câu trả lời chỉ có thể là `True` hoặc `False`, chưa
bao giờ là một con số. Muốn thấy con số thì in thẳng `re_ot` ra.
::
:::

:::opt
Máy dừng lại và báo lỗi, vì hai số âm thì không có gì để so
::why
Gần đúng ở chỗ bạn đang giữ đúng cái nghĩa đầu tiên của "lớn hơn", cái nghĩa mà
mọi người học trước hết: đặt hai nắm hạt cạnh nhau, đếm, xem bên nào ra con số
to hơn. Với cách hiểu ấy thì so hai số âm quả thật vô nghĩa — chẳng có nắm hạt
nào ở đó để đếm cả.

Chỗ lệch: bài này vừa đổi câu hỏi. Dấu `>` bây giờ không hỏi "bên nào nhiều
hơn" nữa, nó hỏi "bên nào đứng bên phải". Mà mọi chỗ trên thanh số — kể cả bên
trái vạch 0 — đều có chỗ đứng, nên câu hỏi ấy luôn có câu trả lời và máy không
phải dừng lại lần nào.
::
:::
::::

::::explain{#hoi-cho-ro-cau-nao}
Rút ra hai câu để mang theo.

**Một:** so sánh là chuyện của **chỗ đứng**, không phải chuyện của khoảng cách
tới vạch 0. Số nào đứng bên phải thì lớn hơn, hết.

**Hai:** ngoài đời, hỏi cho rõ mình đang hỏi câu nào. "Rễ nào ăn sâu hơn" và
"số nào lớn hơn" là hai câu hỏi khác nhau, và với hai cái rễ thì chúng cho hai
câu trả lời ngược nhau. Người trả lời nhầm câu không phải người tính sai — họ
tính đúng câu hỏi kia.

Cái vạch 0 vẫn còn giữ một việc: nó là chỗ mà hai nửa gặp nhau. Mọi số âm đều
đứng bên trái nó, nên mọi số âm đều nhỏ hơn mọi số dương — không cần so từng
cặp một.
::::

::::code{#hai-cau-hoi-cung-mot-kieu}
Byte hỏi máy đúng **một kiểu câu**, hai lần: *"vế trái có đứng bên phải vế phải
không?"*

Hai lần này được chọn để cho ra hai câu trả lời **ngược nhau**: cặp ngọn mầm ở
nửa bên phải vạch 0, cặp đầu rễ ở nửa bên trái. Gõ cứng `True` vào cả hai chỗ
trống thì cặp thứ hai sai; gõ cứng `False` thì cặp thứ nhất sai. Chỉ hai câu hỏi
viết thật mới qua được cả hai.

```python title=starter
# Sổ vườn của Byte, đo bằng phân, mặt đất là vạch 0.
ngon_cai = 5     # ngọn mầm cải, 5 phân TRÊN mặt đất
ngon_hung = 2    # ngọn mầm húng, 2 phân TRÊN mặt đất
re_ot = -5       # đầu rễ ớt, 5 phân DƯỚI mặt đất
re_hung = -2     # đầu rễ húng, 2 phân DƯỚI mặt đất

# Ngọn cải có đứng bên phải ngọn húng trên thanh số không?
ngon_cai_ben_phai = ___

# Đầu rễ ớt có đứng bên phải đầu rễ húng không?
re_ot_ben_phai = ___

print(ngon_cai_ben_phai)
print(re_ot_ben_phai)
```

```python title=solution
# Sổ vườn của Byte, đo bằng phân, mặt đất là vạch 0.
ngon_cai = 5     # ngọn mầm cải, 5 phân TRÊN mặt đất
ngon_hung = 2    # ngọn mầm húng, 2 phân TRÊN mặt đất
re_ot = -5       # đầu rễ ớt, 5 phân DƯỚI mặt đất
re_hung = -2     # đầu rễ húng, 2 phân DƯỚI mặt đất

# Ngọn cải có đứng bên phải ngọn húng trên thanh số không?
ngon_cai_ben_phai = ngon_cai > ngon_hung

# Đầu rễ ớt có đứng bên phải đầu rễ húng không?
re_ot_ben_phai = re_ot > re_hung

print(ngon_cai_ben_phai)
print(re_ot_ben_phai)
```

```python title=test
# Hai cặp cho hai câu trả lời ngược nhau, nên một chữ True hay False gõ cứng
# chỉ qua được nhiều nhất một dòng. Ba câu khẳng định cuối chốt lại chính điều
# bài vừa dạy, để cổng kiểm đỏ lên nếu có ngày nó bị dạy sai đi.
assert ngon_cai_ben_phai is True, "5 đứng bên phải 2 trên thanh số"
assert re_ot_ben_phai is False, "-5 đứng bên TRÁI -2, nên nó không lớn hơn"
assert re_hung > re_ot, "-2 mới là số đứng bên phải trong cặp hai cái rễ"
assert re_hung < ngon_hung, "mọi số âm đều đứng bên trái mọi số dương"
```

:::hints
- kind: attention
  body: Bốn cái tên ở trên đã giữ sẵn bốn con số, bạn không phải gõ lại con số nào. Mỗi chỗ trống chỉ cần ghép hai cái tên lại bằng một dấu.
- kind: strategy
  body: Dấu cần dùng là dấu so sánh "lớn hơn" của Realm 0. Đọc lại câu hỏi trong ghi chú để biết tên nào đứng bên trái dấu — tên nào được hỏi "có đứng bên phải không" thì tên đó viết trước.
- kind: one-line
  body: "Viết `ngon_cai > ngon_hung` vào chỗ trống thứ nhất và `re_ot > re_hung` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một câu hỏi so sánh viết bằng dấu `>` giữa hai cái tên đã có — gõ thẳng True hay False thì không hỏi máy điều gì cả
  requireAst:
  - kind: uses-operator, target: >, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^True\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
True rồi False. Ăn sâu hơn với lớn hơn là hai câu hỏi khác nhau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`5 - 12` giờ đã có câu trả lời đàng hoàng: `-7`. Không còn phép trừ nào phải
dừng lại giữa chừng nữa — đứng ở đâu cũng lùi được, lùi bao nhiêu bước cũng có
chỗ để đặt chân.

Nhưng để ý điều đó nói lên chuyện gì. Bài 13 nói cộng là bước sang phải, bài 14
nói trừ là bước sang trái. Hai luật, hai chiều, hai thứ phải nhớ.

Bây giờ thanh số đã dài về cả hai phía, và mỗi số bên phải vạch 0 đều có một chỗ
đối diện bên trái. Vậy "lùi 5 bước" và "tiến 5 bước về phía bên kia" — có phải
là hai cách nói của cùng một việc không?

Nếu đúng thì phép trừ có còn là một phép riêng nữa không, hay nó là phép cộng
đội lốt? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
