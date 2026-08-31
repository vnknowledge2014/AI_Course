---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.dong-cai-trong-mot-mang
title: "Đống cài được gọn trong một MẢNG, không cần con trỏ"
summary: "Luật hình dạng chặt của đống nghĩa là con của ô i luôn nằm ở ô 2i+1 và 2i+2 — tính thẳng bằng công thức, đúng như bài 2, nên đống cài vừa khít vào một mảng mà không cần trường tiep hay trai/phai nào."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 33
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.heap-array-layout]
requires: [ds.heap, ds.array-index-address, ctrl.for-range, core.arithmetic, core.list-append, core.fstring]
concepts: [ds.heap-array-layout]
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
Bài trước hỏi: luật hình dạng chặt của đống có bỏ được `trai`/`phai`
không? Câu trả lời là một cú lật ngược khá bất ngờ.
::::

::::explain{#hinh-dang-chat-la-manh}
Nhắc lại luật HÌNH DẠNG của đống, thứ chưa nhấn mạnh ở bài trước: đống
luôn là một cây **lấp đầy từng tầng, từ trái sang phải, không để trống ô
nào ở giữa**. Tầng trên phải đầy hẳn trước khi tầng dưới được thêm nút
mới; trong cùng một tầng, các nút lấp từ trái qua phải, không nhảy cóc.

Luật này chặt tới mức bạn có thể LIỆT KÊ hết các nút của một đống theo
đúng thứ tự lấp đầy đó — nút gốc trước, rồi tầng kế lần lượt trái sang
phải, rồi tầng kế nữa — và kết quả là một DÃY phẳng, không có gì rẽ nhánh
cả. Chính dãy phẳng ấy nhét vừa khít vào một MẢNG — mảng ở cụm 1.

Và khi đã nằm trong mảng, quan hệ cha-con không cần trường `trai`/`phai`
nào để ghi nhớ nữa — nó TÍNH ĐƯỢC thẳng từ chỉ số, y hệt công thức bài 2
tính địa chỉ ô thứ `i`:

> Với nút ở chỉ số `i` (đếm từ 0): con trái nằm ở chỉ số **2i + 1**, con
> phải nằm ở chỉ số **2i + 2**. Ngược lại, cha của ô `i` (khi `i > 0`)
> nằm ở chỉ số **(i − 1) // 2**.

Không có tấm thẻ nào phải lưu, không có `id()` nào phải tra — chỉ một
phép nhân và một phép cộng. Đây chính là cú lật ngược: cây (bài 25) vốn
cần tham chiếu vì các nút không nằm cạnh nhau trong bộ nhớ, nhưng đống —
nhờ luật hình dạng chặt của nó — hoá ra cài vừa khít vào đúng thứ cấu
trúc mà track này mở ra ĐẦU TIÊN.
::::

::::example{#doi-chieu-mang-va-cay}
Cây đống ở bài trước, liệt kê theo đúng thứ tự lấp đầy từng tầng, chính là
mảng này:

```python title=readonly
dong = [9, 7, 8, 3, 6, 8, 2]
#        i=0 1  2  3  4  5  6

for i in range(len(dong)):
    con_trai = 2 * i + 1
    con_phai = 2 * i + 2
    print(f"ô {i} (giá trị {dong[i]}): con trái ở ô {con_trai}, con phải ở ô {con_phai}")
```

```text title=readonly
ô 0 (giá trị 9): con trái ở ô 1, con phải ở ô 2
ô 1 (giá trị 7): con trái ở ô 3, con phải ở ô 4
ô 2 (giá trị 8): con trái ở ô 5, con phải ở ô 6
ô 3 (giá trị 3): con trái ở ô 7, con phải ở ô 8
ô 4 (giá trị 6): con trái ở ô 9, con phải ở ô 10
ô 5 (giá trị 8): con trái ở ô 11, con phải ở ô 12
ô 6 (giá trị 2): con trái ở ô 13, con phải ở ô 14
```

Đối chiếu với cây bằng `dict` bài trước: ô 0 (giá trị `9`) là gốc; công
thức nói con trái của nó ở ô `1` (giá trị `7`) và con phải ở ô `2` (giá
trị `8`) — đúng khớp cây cũ. Ô `1` (giá trị `7`) có con ở ô `3` (`3`) và
ô `4` (`6`) — cũng khớp. Từ ô `3` trở đi, công thức tính ra những ô KHÔNG
tồn tại trong mảng bảy phần tử này (ô `7` trở lên) — đó chỉ đơn giản là
những chiếc lá không có con, y hệt việc `trai`/`phai` của chúng từng là
`None`. Muốn biết một ô có con thật hay không, chỉ cần so chỉ số tính
được với `len(dong)`.
::::

::::predict{#doan-chi-so-con commitOnce}
Một đống lưu trong mảng có đúng 10 phần tử, chỉ số chạy từ `0` tới `9`.
Byte đang đứng ở ô số `3`.

**Trước khi tính**, bạn đoán: con trái và con phải của ô số `3` là ô số
mấy?

:::opt{correct}
Con trái ở ô `7`, con phải ở ô `8` — tính bằng `2*3+1` và `2*3+2`.
:::

:::opt
Con trái ở ô `6`, con phải ở ô `7` — tính bằng `2*3` và `2*3+1`.
::why
Gần đúng ở việc bạn dùng đúng phép NHÂN ĐÔI — track này đúng là dùng chỉ
số nhân hai để nhảy xuống một tầng, phản xạ đó không sai hướng.

Chỗ lệch là thiếu mất phần `+1` ở công thức con trái. Công thức đúng là
`2*i + 1` cho con trái, không phải `2*i` — thiếu số `1` đó làm mọi chỉ số
tính ra lệch mất một ô so với cây thật, và hai công thức con trái/con
phải của bạn cũng dính liền nhau sai chỗ, không còn cách nhau đúng `1`
đơn vị.
::
:::

:::opt
Con trái ở ô `4`, con phải ở ô `5` — cộng thêm 1 và 2 vào chỉ số hiện tại.
::why
Gần đúng ở việc bạn hiểu quan hệ cha-con phải LỚN HƠN chỉ số cha — con
luôn nằm ở chỉ số lớn hơn cha, điều đó đúng hướng.

Chỗ lệch là công thức track này dùng KHÔNG phải phép CỘNG đơn giản — nó
là phép NHÂN ĐÔI rồi mới cộng. Cộng `1` và `2` vào chỉ số cha chỉ đúng
cho MỘT cây rất đặc biệt (mỗi nút chỉ có tối đa... một vài con ở tầng sát
ngay dưới cùng); với một đống thật có nhiều tầng, công thức đó tính sai
gần như ngay từ tầng thứ hai trở đi.
::
:::

:::opt
Con trái ở ô `7`, con phải ở ô `9` — dùng `2*3+1` cho con trái, nhưng
`2*3+3` cho con phải, để hai công thức cách đều nhau `2` đơn vị.
::why
Gần đúng ở việc bạn tính đúng con trái — `2*3+1 = 7` khớp hoàn toàn với
công thức track này dùng.

Chỗ lệch là công thức con phải. Nó không phải `2*i + 3`, mà là `2*i + 2`
— chỉ hơn công thức con trái đúng `1` đơn vị, không phải `2`. Với `i=3`,
con phải đúng phải là ô `8`, không phải ô `9`.
::
:::
::::

::::code{#kiem-dong-bang-mang}
Cùng bảy giá trị đống ở ví dụ trên, giờ kiểm luật đống hoàn toàn bằng chỉ
số — không một trường `trai`/`phai` nào, không một `dict` nào.

```python title=starter
dong = [9, 7, 8, 3, 6, 8, 2]

vi_pham = []
for i in range(len(dong)):
    con_trai = ___
    con_phai = ___
    if con_trai < len(dong) and dong[i] < dong[con_trai]:
        vi_pham.append((i, con_trai))
    if con_phai < len(dong) and dong[i] < dong[con_phai]:
        vi_pham.append((i, con_phai))

la_dong_hop_le = len(vi_pham) == 0
print(f"Đúng luật đống: {la_dong_hop_le}")
print(f"Vi phạm: {vi_pham}")
```

```python title=solution
dong = [9, 7, 8, 3, 6, 8, 2]

vi_pham = []
for i in range(len(dong)):
    con_trai = 2 * i + 1
    con_phai = 2 * i + 2
    if con_trai < len(dong) and dong[i] < dong[con_trai]:
        vi_pham.append((i, con_trai))
    if con_phai < len(dong) and dong[i] < dong[con_phai]:
        vi_pham.append((i, con_phai))

la_dong_hop_le = len(vi_pham) == 0
print(f"Đúng luật đống: {la_dong_hop_le}")
print(f"Vi phạm: {vi_pham}")
```

```python title=test
assert la_dong_hop_le is True, f"bảy giá trị này đúng luật đống — đang báo {la_dong_hop_le}"
assert vi_pham == [], f"không cặp chỉ số cha-con nào ở đây vi phạm luật đống — đang báo vi phạm ở {vi_pham}"
assert con_trai == 13, f"con_trai phải LUÔN tính bằng công thức 2*i+1 — ở vòng lặp cuối (i=6), con_trai phải là 13, đang là {con_trai}. Nếu bạn đang thấy 14 ở đây, có thể công thức con_trai và con_phai đã bị ĐẢO CHO NHAU."
assert con_phai == 14, f"con_phai phải LUÔN tính bằng công thức 2*i+2 — ở vòng lặp cuối (i=6), con_phai phải là 14, đang là {con_phai}. Nếu bạn đang thấy 13 ở đây, có thể công thức con_trai và con_phai đã bị ĐẢO CHO NHAU."
```

:::hints
- kind: attention
  body: Hai chỗ trống không đọc dữ liệu trong `dong` — chúng chỉ TÍNH một con số, từ đúng một biến `i` đang có sẵn trong vòng lặp. Đừng nhầm sang việc đọc `dong[i]`.
- kind: strategy
  body: Dùng đúng công thức vừa học — con trái ở chỉ số nhân đôi `i` rồi cộng `1`, con phải ở chỉ số nhân đôi `i` rồi cộng `2`. Cả hai đều là phép tính thuần trên `i`, không cần dò qua phần tử nào của mảng.
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `2 * i + 1` và `2 * i + 2`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải THẬT SỰ tính từ biến i bằng công thức 2*i+1 và 2*i+2 — không được gõ một con số cố định (dù con số đó tình cờ đúng cho ví dụ này), vì công thức phải đúng với MỌI chỉ số i trong vòng lặp, không chỉ đúng cho một ô
  requireAst:
  - kind: uses-name, target: i, min: 6
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Đúng luật đống: True\\nVi phạm: \\[\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không một trường `trai`, không một trường `phai` — chỉ nhân đôi rồi cộng
một hoặc hai. Cả cái cây gọn lại thành một dòng số.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cây và đống đều là những cấu trúc PHÂN CẤP — mỗi nút chỉ nối xuống một số
con giới hạn, và luôn có đúng một gốc, không có vòng quay lại. Nhưng
không phải mọi thứ trong đời thực đều phân cấp gọn như vậy: một mạng bạn
bè, ai cũng có thể nối với bất kỳ ai, không cần theo tầng, và hoàn toàn có
thể có một vòng quay lại chỗ cũ.

Cấu trúc nào mô tả đúng một mạng như vậy — nơi không có "gốc", không có
tầng trên tầng dưới, và một đỉnh có thể nối tới bao nhiêu đỉnh khác cũng
được?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
