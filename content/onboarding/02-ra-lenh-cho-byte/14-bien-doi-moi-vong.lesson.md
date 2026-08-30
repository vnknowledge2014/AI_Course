---
id: onboarding.ra-lenh-cho-byte.bien-doi-moi-vong
title: Cái tên đổi giá trị mỗi vòng
summary: Cái tên đứng sau for không nằm im — mỗi lượt máy dán nó lên một giá trị khác.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ctrl.loop-variable]
requires: [ctrl.for-range, core.variable, core.fstring]
concepts: [ctrl.lap, core.bien]
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
Bài trước cái tên đó đứng im không ai đụng tới. Hôm nay mình mở nó ra.
::::

::::explain{#tam-the-treo-truoc-quay}
Câu hỏi bỏ ngỏ của bài trước: cái tên `lan`, đứng giữa `for` và `in`, nằm đó để
làm gì?

Ở quán có một tấm thẻ nhựa treo trước quầy. Mỗi lượt bưng bát ra, bà chủ lật
tấm thẻ sang số mới: lượt đầu số 0, lượt sau số 1, lượt sau nữa số 2.

Tấm thẻ vẫn là **một** tấm thẻ. Chỗ treo không đổi, cái móc không đổi. Chỉ con
số hiện trên nó là đổi theo từng lượt.

Cái tên sau `for` chính là tấm thẻ ấy.

Ở bài đặt tên cho một giá trị, bạn tự tay dán một cái tên lên một giá trị bằng
dấu `=`. Ở bài đổi giá trị của một cái tên, bạn dán lại cái tên đó lên giá trị
khác. Trong vòng lặp, bạn không phải dán gì cả: **máy tự dán lại, mỗi lượt một
lần**, trước khi chạy phần việc.

Cái tên ấy có tên gọi riêng: **biến lặp**.
::::

::::example{#mo-cai-ten-ra}
Lấy đúng vòng lặp của bài trước, chỉ đổi một chỗ: đem cái tên vào trong `print`.

```python title=readonly
for lan in range(3):
    print(lan)
```

Máy in ra:

```text title=readonly
0
1
2
```

Ba lượt, ba giá trị khác nhau. Hai điều đáng ghi lại:

- `lan` trong `print` **không có dấu nháy**, nên máy in ra thứ cái tên đang giữ, không in ba chữ cái l-a-n. Đúng luật bạn đã học ở bài in ra thứ mà cái tên đang giữ.
- `range(3)` cho ba con số, và ba con số đó là **0, 1, 2**. Nó bắt đầu từ 0 chứ không phải từ 1, và con số 3 bạn viết trong ngoặc không bao giờ tự xuất hiện — 3 là *số lượt*, không phải số cuối cùng.

Vì sao lại bắt đầu từ 0 thì vài bài nữa có câu trả lời đầy đủ. Bây giờ chỉ cần
nhớ mặt chữ: `range(3)` đếm 0, 1, 2.
::::

::::example{#ghep-vao-cau}
In trơ một con số thì chưa dùng được vào việc gì. Đem nó vào giữa một câu chữ
bằng f-string — thứ bạn đã dùng ở bài chèn giá trị vào giữa câu:

```python title=readonly
for so in range(3):
    print(f"Tô thứ {so}: 45000 đồng")
```

```text title=readonly
Tô thứ 0: 45000 đồng
Tô thứ 1: 45000 đồng
Tô thứ 2: 45000 đồng
```

Ba dòng khác nhau, viết bằng đúng một câu `print`. Đây là chỗ vòng lặp bắt đầu
có ích thật: phần giống nhau bạn viết một lần, phần khác nhau để biến lặp lo.
::::

::::predict{#to-thu-may commitOnce}
Đổi số lượt lên bốn. **Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
for so in range(4):
    print(f"Tô thứ {so}")
```

:::opt{correct}
Bốn dòng: Tô thứ 0, Tô thứ 1, Tô thứ 2, Tô thứ 3
:::

:::opt
Bốn dòng: Tô thứ 1, Tô thứ 2, Tô thứ 3, Tô thứ 4
::why
Gần đúng gần hết. Bạn đếm đúng bốn lượt, và bạn nắm đúng điều mới của bài này:
mỗi lượt cái tên mang một giá trị khác.

Chỗ lệch chỉ nằm ở điểm xuất phát, và nó lệch vì một lý do rất chính đáng —
ngoài đời ta đếm tô thứ nhất, thứ hai, thứ ba. Máy thì không: `range` khởi hành
từ 0, nên bốn con số nó phát ra là 0, 1, 2 và 3. Con số 4 trong ngoặc là số
lượt, và nó không bao giờ được đưa vào vòng nào cả.
::
:::

:::opt
Bốn dòng giống hệt nhau: Tô thứ so
::why
Gần đúng ở một luật rất thật mà bạn đang nhớ chính xác: cái gì nằm trong dấu
nháy thì máy đọc nguyên văn, không cố hiểu. Cả câu này đúng là nằm trong nháy.

Chỗ lệch là chữ `f` đứng ngay trước dấu nháy mở. Chữ `f` ấy báo cho máy một
ngoại lệ: phần nào nằm giữa cặp ngoặc nhọn `{ }` thì đừng đọc nguyên văn, hãy
coi nó là một cái tên, đi tìm giá trị rồi thay vào. Bỏ chữ `f` đi thì dự đoán
của bạn thành đúng.
::
:::

:::opt
Bốn dòng giống hệt nhau: Tô thứ 0
::why
Gần đúng ở chỗ bạn mang theo đúng kết luận của bài trước: mọi lượt in ra y hệt
nhau. Ở bài trước điều đó có thật.

Nhưng nó có thật vì một lý do cụ thể — ta không hề dùng tới cái tên, nên chẳng
có gì để mà khác. Lần này `so` có mặt trong câu chữ, và mỗi lượt máy dán lại cái
tên đó lên một giá trị mới **trước khi** chạy `print`. Bốn lượt, bốn giá trị,
bốn dòng khác nhau.
::
:::
::::

::::explain{#ten-nao-cung-duoc}
Vài chuyện gọn cần biết về biến lặp:

- **Tên gì cũng được.** `lan`, `so`, `i`, `so_thu_tu` — máy không quan tâm bạn đặt tên gì, nó chỉ quan tâm cái tên ấy đứng đúng chỗ giữa `for` và `in`. Người đọc lại code sau này thì có quan tâm, nên đặt tên nói đúng thứ nó giữ.
- **Muốn đánh số từ 1** thì cộng thêm một lúc in: `print(f"Tô thứ {so + 1}")`. Máy tính `so + 1` xong mới thay vào câu. Phép cộng số bạn đã biết từ bài số thì máy tính được.
- **Bên trong vòng lặp bạn viết được nhiều dòng**, miễn là cùng lùi vào bốn dấu cách. Cả khối ấy chạy lại từ đầu ở mỗi lượt.
::::

::::code{#bang-nhac-bep}
Bà chủ muốn một bảng nhắc bếp cho hai bàn. Bàn 1 gọi **năm** tô, bàn 2 gọi
**ba** tô. Mỗi dòng ghi số thứ tự của một tô.

Máy phải in ra đúng thế này:

```text title=readonly
Bàn 1 - tô số 0
Bàn 1 - tô số 1
Bàn 1 - tô số 2
Bàn 1 - tô số 3
Bàn 1 - tô số 4
Bàn 2 - tô số 0
Bàn 2 - tô số 1
Bàn 2 - tô số 2
```

Hai vòng lặp đã dựng sẵn, và hai chỗ trống điền **cùng một thứ**. Hãy điền thứ
đổi theo từng lượt vào giữa cặp ngoặc nhọn.

Hai vòng dùng chung một cái tên. Vòng sau chạy độc lập với vòng trước: máy dán
lại cái tên ấy từ 0 một lần nữa.

```python title=starter
for so in range(5):
    print(f"Bàn 1 - tô số {___}")

for so in range(3):
    print(f"Bàn 2 - tô số {___}")
```

```python title=solution
for so in range(5):
    print(f"Bàn 1 - tô số {so}")

for so in range(3):
    print(f"Bàn 2 - tô số {so}")
```

```python title=test
# Chấm bằng OUTPUT, và chấm trọn cả tám dòng — không chỉ một dòng.
#
# Hai vòng có số lượt khác nhau, nên chỉ cái tên mới điền đúng được cả hai chỗ:
# nhét một con số cứng vào thì tám dòng hoá giống nhau và bài trượt ngay.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm giữa cặp ngoặc nhọn trong câu chữ. Đó là chỗ máy **không** đọc nguyên văn mà đi tìm một cái tên.
- kind: strategy
  body: Thứ cần thay vào là thứ đổi theo từng lượt. Một con số cứng thì đứng im, mà bàn 1 cần năm số khác nhau còn bàn 2 cần ba. Trong đoạn này chỉ có đúng một cái tên đổi theo lượt, và nó đứng ngay sau chữ `for`.
- kind: one-line
  body: "Viết `so` vào giữa cặp ngoặc nhọn ở **cả hai** chỗ trống, thành `{so}`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Bàn 1 - tô số 0\nBàn 1 - tô số 1\nBàn 1 - tô số 2\nBàn 1 - tô số 3\nBàn 1 - tô số 4\nBàn 2 - tô số 0\nBàn 2 - tô số 1\nBàn 2 - tô số 2\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai câu print, tám dòng khác nhau. Phần khác nhau đó do cái tên gánh hết.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vòng lặp hôm nay chạy rất ngọt — miễn là thứ đổi theo từng lượt **là một con
số**. `range` chỉ biết phát ra số: 0, 1, 2, 3.

Nhưng tấm bảng thực đơn treo trên tường quán không phải 0, 1, 2. Nó là "Phở
tái", "Phở chín", "Phở nạm". Ba câu chữ, có thứ tự hẳn hoi, và bà chủ muốn máy
đọc lần lượt từng món.

Ba câu chữ ấy phải nằm ở **đâu** thì một vòng lặp mới đi qua được từng cái? Đặt
ba cái tên riêng — `mon_1`, `mon_2`, `mon_3` — thì `range` biết gọi cái nào?

Bài sau đưa ra chỗ chứa đó.
::::

::::checkpoint{mastery=0.8}
::::
