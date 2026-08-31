---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.bieu-dien-do-thi-hai-cach
title: "Biểu diễn đồ thị: bảng kề hay ma trận kề"
summary: "Bảng kề — mỗi đỉnh trỏ tới danh sách hàng xóm bằng bảng băm — rẻ bộ nhớ khi ít cạnh; ma trận kề — một mảng vuông đánh dấu mọi cặp đỉnh — tra một cạnh tức thời nhưng tốn ô với đồ thị thưa."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 35
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.graph-representation]
requires: [ds.graph, ds.dict-is-hash-table, core.nested-list-dict, core.dict-comprehension, ctrl.for-range, core.list-append, core.fstring]
concepts: [ds.graph-representation]
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
Quét cả danh sách cạnh mỗi lần hỏi "hai đỉnh này có nối nhau không" — bài
trước để lại đúng câu hỏi đó. Hôm nay là hai cách cất dữ liệu trả lời
nhanh hơn.
::::

::::explain{#hai-cach-cat-do-thi}
Danh sách cạnh (`[("An", "Bình"), ...]`) của bài trước đúng, nhưng chậm
để hỏi "A và B có nối nhau không" — phải dò từng cặp. Có hai cách cất lại
CÙNG một đồ thị để câu hỏi đó trả lời nhanh hơn.

**Bảng kề** (adjacency list): mỗi đỉnh trỏ tới một `list` các đỉnh hàng
xóm của nó — dùng đúng bảng băm cụm 4 (`dict`) làm khung tra cứu tên
đỉnh. Muốn biết hàng xóm của `A` là ai, tra thẳng `bang_ke["A"]` — một
lần băm, không dò (bài 20: tra `dict` không cần dò cả bảng).

**Ma trận kề** (adjacency matrix): một mảng VUÔNG, `n` hàng `n` cột với
`n` là số đỉnh — ô `[i][j]` đánh dấu `1` nếu đỉnh `i` và đỉnh `j` có nối,
`0` nếu không, cho MỌI cặp đỉnh, kể cả những cặp không hề nối. Muốn biết
`A` và `B` có nối không, tra thẳng `ma_tran[chi_so_A][chi_so_B]` — một
phép tính chỉ số, đúng công thức bài 2: địa chỉ ô tính thẳng, không dò.

Hai cách đều tra cứu MỘT cạnh nhanh như nhau về mặt định tính. Khác nhau
ở chỗ TỐN BAO NHIÊU Ô. Bảng kề chỉ tốn đúng số ô bằng số LẦN xuất hiện
thật của một cạnh (mỗi cạnh góp mặt ở đúng hai chỗ, một cho mỗi đầu). Ma
trận kề tốn `n × n` ô LUÔN LUÔN, bất kể đồ thị có bao nhiêu cạnh thật —
kể cả khi hầu hết các ô đều là `0`. Một đồ thị THƯA (ít cạnh so với số
đỉnh, như mạng bạn bè — không ai quen hết mọi người) thì bảng kề rẻ hơn
nhiều; một đồ thị DÀY (gần như mọi cặp đỉnh đều nối) thì khoảng cách đó
thu hẹp lại, vì ma trận kề dù sao cũng chỉ tốn đúng `n × n` ô, không đổi
theo số cạnh.
::::

::::example{#dung-hai-cach-tren-cung-do-thi}
Cùng mạng bạn bè bài trước, dựng cả hai cách biểu diễn:

```python title=readonly
dinh = ["An", "Bình", "Chi", "Dung"]
canh = [("An", "Bình"), ("Bình", "Chi"), ("Chi", "An"), ("Bình", "Dung")]

bang_ke = {ten: [] for ten in dinh}
for (a, b) in canh:
    bang_ke[a].append(b)
    bang_ke[b].append(a)

chi_so = {}
for i in range(len(dinh)):
    chi_so[dinh[i]] = i

n = len(dinh)
ma_tran = [[0] * n for _ in range(n)]
for (a, b) in canh:
    i, j = chi_so[a], chi_so[b]
    ma_tran[i][j] = 1
    ma_tran[j][i] = 1

print("Bảng kề:", bang_ke)
print("Ma trận kề:")
for hang in ma_tran:
    print(hang)
```

```text title=readonly
Bảng kề: {'An': ['Bình', 'Chi'], 'Bình': ['An', 'Chi', 'Dung'], 'Chi': ['Bình', 'An'], 'Dung': ['Bình']}
Ma trận kề:
[0, 1, 1, 0]
[1, 0, 1, 1]
[1, 1, 0, 0]
[0, 1, 0, 0]
```

Mỗi cạnh được ghi ở CẢ HAI đầu — vì đây là đồ thị KHÔNG CÓ HƯỚNG (bạn bè
là hai chiều: `A` quen `B` thì `B` cũng quen `A`), nên `bang_ke["An"]`
chứa `Bình`, VÀ `bang_ke["Bình"]` cũng chứa `An`. Ma trận cũng vậy: ô
`[0][1]` và ô `[1][0]` cùng là `1` — đối xứng qua đường chéo. Bốn đỉnh
nhưng ma trận có `16` ô, trong khi chỉ có `4` cạnh (tức `8` lượt đánh dấu
thật) — quá nửa số ô của ma trận là `0`, vẫn phải tốn chỗ cho chúng dù
chẳng có cạnh nào ở đó.
::::

::::predict{#doan-do-thi-thua commitOnce}
Một đồ thị có `1000` đỉnh, nhưng chỉ có `5` cạnh — cực kỳ THƯA, gần như
chẳng ai nối với ai.

**Trước khi tính**, bạn đoán: cách biểu diễn nào tốn ít Ô NHỚ hơn cho đồ
thị này?

:::opt{correct}
Bảng kề — nó chỉ tốn số ô tương ứng với `5` cạnh thật (khoảng `10` mục
trong các `list` hàng xóm), trong khi ma trận kề vẫn tốn `1000 × 1000`
ô bất kể có bao nhiêu cạnh.
:::

:::opt
Ma trận kề — vì tra một cạnh trong ma trận nhanh hơn, nên nó cũng tốn ít
ô nhớ hơn để đạt được tốc độ đó.
::why
Gần đúng ở việc tra một cạnh trong ma trận đúng là nhanh — một phép tính
chỉ số, không cần dò. Điều đó không sai.

Chỗ lệch là TỐC ĐỘ tra cứu và LƯỢNG Ô NHỚ tốn là hai câu hỏi khác nhau
hoàn toàn, không đi cùng chiều với nhau ở đây. Ma trận `1000 × 1000` tốn
đúng một triệu ô cho DÙ chỉ có `5` cạnh thật — tốc độ tra nhanh không hề
làm giảm số ô nó chiếm.
::
:::

:::opt
Cả hai tốn ngang nhau — vì cả hai đều phải lưu đủ thông tin về `1000`
đỉnh và `5` cạnh, không cách nào né được việc đó.
::why
Gần đúng ở việc cả hai cách đều PHẢI biểu diễn đúng cùng một đồ thị —
không cách nào được phép thiếu thông tin.

Chỗ lệch là "đủ thông tin" không có nghĩa là tốn ô NHƯ NHAU. Bảng kề chỉ
ghi những gì THẬT SỰ có (5 cạnh); ma trận kề phải dành sẵn một ô cho MỌI
cặp đỉnh có thể có, kể cả `999995` cặp không hề nối với nhau — số ô thừa
đó là khoảng cách rất lớn giữa hai cách.
::
:::

:::opt
Bảng kề — nhưng chỉ vì `dict` luôn tốn ít ô nhớ hơn `list` trong mọi
trường hợp, không liên quan gì tới số cạnh của đồ thị.
::why
Gần đúng ở kết luận cuối cùng — bảng kề đúng là cách tốn ít ô hơn cho đồ
thị này, kết luận đó không sai.

Chỗ lệch là lý do. Đây không phải vì `dict` "luôn" rẻ hơn `list` trong
mọi hoàn cảnh — với một đồ thị DÀY (gần như mọi cặp đỉnh đều nối), bảng
kề phải lưu gần hết các cặp đó dưới dạng danh sách hàng xóm dài, và
khoảng cách với ma trận kề thu hẹp lại. Lý do đúng nằm ở việc đồ thị này
THƯA — ít cạnh so với số đỉnh — không nằm ở bản chất `dict` so với
`list`.
::
:::
::::

::::code{#dung-hai-bieu-dien}
Dựng lại cả hai cách biểu diễn cho đúng mạng bạn bè ở ví dụ trên. Bốn chỗ
trống: hai chỗ hoàn thiện bảng kề (mỗi cạnh phải ghi ở CẢ HAI đầu), hai
chỗ hoàn thiện ma trận kề (cũng phải đối xứng).

```python title=starter
dinh = ["An", "Bình", "Chi", "Dung"]
canh = [("An", "Bình"), ("Bình", "Chi"), ("Chi", "An"), ("Bình", "Dung")]

bang_ke = {ten: [] for ten in dinh}
for (a, b) in canh:
    bang_ke[a].append(b)
    bang_ke[___].append(___)

chi_so = {}
for i in range(len(dinh)):
    chi_so[dinh[i]] = i

n = len(dinh)
ma_tran = [[0] * n for _ in range(n)]
for (a, b) in canh:
    i, j = chi_so[a], chi_so[b]
    ma_tran[i][j] = 1
    ma_tran[___][___] = 1

print(f"Hàng xóm của Bình (bảng kề): {bang_ke['Bình']}")
print(f"An và Dung có nối không (ma trận): {ma_tran[chi_so['An']][chi_so['Dung']] == 1}")
```

```python title=solution
dinh = ["An", "Bình", "Chi", "Dung"]
canh = [("An", "Bình"), ("Bình", "Chi"), ("Chi", "An"), ("Bình", "Dung")]

bang_ke = {ten: [] for ten in dinh}
for (a, b) in canh:
    bang_ke[a].append(b)
    bang_ke[b].append(a)

chi_so = {}
for i in range(len(dinh)):
    chi_so[dinh[i]] = i

n = len(dinh)
ma_tran = [[0] * n for _ in range(n)]
for (a, b) in canh:
    i, j = chi_so[a], chi_so[b]
    ma_tran[i][j] = 1
    ma_tran[j][i] = 1

print(f"Hàng xóm của Bình (bảng kề): {bang_ke['Bình']}")
print(f"An và Dung có nối không (ma trận): {ma_tran[chi_so['An']][chi_so['Dung']] == 1}")
```

```python title=test
assert bang_ke == {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Chi", "Dung"],
    "Chi": ["Bình", "An"],
    "Dung": ["Bình"],
}, f"bảng kề phải ghi mỗi cạnh ở CẢ HAI đầu (đồ thị không có hướng) — đang ra {bang_ke}"
assert ma_tran == [
    [0, 1, 1, 0],
    [1, 0, 1, 1],
    [1, 1, 0, 0],
    [0, 1, 0, 0],
], f"ma trận kề phải đối xứng — ô [i][j] và ô [j][i] cùng đánh dấu — đang ra {ma_tran}"
```

:::hints
- kind: attention
  body: Cả bốn chỗ trống làm đúng một việc — ghi lại cạnh vừa thấy ở CHIỀU NGƯỢC LẠI. Dòng ngay phía trên mỗi chỗ trống đã ghi một chiều rồi; chỗ trống chỉ cần lặp lại đúng việc đó, đảo hai cái tên cho nhau.
- kind: strategy
  body: 'Bảng kề: dòng trên ghi `bang_ke[a].append(b)` — chỗ trống ghi chiều ngược, `bang_ke[b].append(a)`. Ma trận kề: dòng trên ghi `ma_tran[i][j] = 1` — chỗ trống ghi chiều ngược, `ma_tran[j][i] = 1`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `b`, `a`, `j`, `i`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Hàng xóm của Bình \\(bảng kề\\): \\['An', 'Chi', 'Dung'\\]\\nAn và Dung có nối không \\(ma trận\\): False\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một đồ thị, hai cách cất — và cả hai đều trả lời đúng: An và Dung
không hề quen nhau.
::::

::::reflect{#nghi-lai}
Một câu hỏi khép lại cả track này.

Suốt ba mươi lăm bài, bạn đã mở nắp bảy, tám hộp đen khác nhau — mảng,
ngăn xếp, hàng đợi, danh sách liên kết, bảng băm, cây, đống, đồ thị — và
mỗi hộp đều có một chỗ nó LÀM TỐT, một chỗ nó TRẢ GIÁ. Không có hộp nào
thắng tuyệt đối mọi hộp còn lại.

Nếu ai đó đưa bạn một bài toán thật — không nói tên cấu trúc nào cả, chỉ
tả tình huống — bạn có nhận ra ngay nên mở hộp nào không?

Bài sau kiểm đúng điều đó.
::::

::::checkpoint{mastery=0.8}
::::
