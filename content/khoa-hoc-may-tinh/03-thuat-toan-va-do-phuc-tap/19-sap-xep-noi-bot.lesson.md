---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.sap-xep-noi-bot
title: "Sắp xếp nổi bọt: so hai ô cạnh nhau, đổi chỗ nếu sai thứ tự"
summary: "Đi hết mảng, so từng cặp ô liền kề, đổi chỗ nếu sai thứ tự, lặp lại tới khi không còn đổi chỗ nào. Đếm bước thật cho thấy số phép so sánh tăng theo BÌNH PHƯƠNG cỡ dữ liệu — O(n²), vì vòng ngoài lặp n lần, vòng trong cũng vậy."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.bubble-sort]
requires: [alg.compare-by-big-o, ds.array-index-address]
concepts: [alg.bubble-sort]
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
"Ai — hay cái gì — biến một dãy lộn xộn thành một dãy đã sắp xếp?"
Không ai cả. Một thuật toán, làm đi làm lại đúng một việc nhỏ.
::::

::::explain{#so-lien-ke-doi-cho}
Bài trước để lại đúng câu hỏi đó: tìm nhị phân chỉ dùng được khi dữ
liệu ĐÃ sắp xếp sẵn, nhưng chưa mảng nào tự sắp xếp lấy chính nó — một
lúc nào đó, một dãy lộn xộn phải được biến thành một dãy đã sắp. Cách
đầu tiên để làm việc đó — cũng là cách dễ hình dung nhất — có tên
**sắp xếp nổi bọt** (bubble sort), và luật của nó chỉ có một câu: đi
từ đầu mảng tới cuối, so từng CẶP Ô LIỀN KỀ một, hễ ô bên trái lớn hơn
ô bên phải thì đổi chỗ hai ô đó ngay.

Đi hết một lượt như vậy chưa chắc mảng đã sắp xong — chỉ riêng phần tử
LỚN NHẤT chắc chắn đã trồi hẳn về cuối mảng, giống một bọt khí nổi dần
lên mặt nước qua từng lần so sánh (đây là gốc của cái tên). Muốn sắp
xong cả mảng, phải lặp lại nhiều LƯỢT như vậy — mỗi lượt sau có thể bỏ
qua đúng phần đuôi đã chắc chắn đúng chỗ từ lượt trước, vì phần tử lớn
nhất, nhì, ba... đã lần lượt trồi lên đó.

Có một điểm dừng sớm đáng nhớ: nếu một lượt trọn vẹn trôi qua mà KHÔNG
đổi chỗ nào cả, nghĩa là mọi cặp liền kề đều đã đúng thứ tự — mảng đã
sắp xong, không cần lượt nào nữa.

Đếm bước (kỹ thuật bài 6, 7) sẽ cho thấy điều đáng chú ý nhất của thuật
toán này: ở trường hợp XẤU NHẤT (bài 11) — mảng đảo ngược hoàn toàn —
lượt nào cũng phải đi hết, không có đường tắt. Vòng ngoài lặp gần `n`
lần, vòng trong cũng lặp gần `n` lần mỗi lượt. Nhân hai con số cùng cỡ
`n` với nhau ra một con số cỡ `n²` — **O(n²)**, đúng ký hiệu bài 10 đã
đặt tên cho hình dạng "gấp đôi dữ liệu, bước tăng gấp bốn" của bài 9.
::::

::::example{#theo-doi-tung-luot}
Byte cho chạy nổi bọt trên bốn số `[5, 2, 8, 1]`, in ra trạng thái mảng
sau MỖI lượt để nhìn rõ nó nổi dần thế nào:

```python title=readonly
def sap_xep_noi_bot(mang):
    n = len(mang)
    for luot in range(n - 1):
        for i in range(n - 1 - luot):
            if mang[i] > mang[i + 1]:
                mang[i], mang[i + 1] = mang[i + 1], mang[i]
        print(f"Sau lượt {luot + 1}: {mang}")
    return mang

sap_xep_noi_bot([5, 2, 8, 1])
```

```text title=readonly
Sau lượt 1: [2, 5, 1, 8]
Sau lượt 2: [2, 1, 5, 8]
Sau lượt 3: [1, 2, 5, 8]
```

Nhìn kỹ lượt 1: `8` — số lớn nhất — trồi thẳng về cuối mảng ngay trong
lượt đầu tiên, đúng như "bọt khí" đi tới đâu cũng nổi hết cỡ trong một
lượt. Những số còn lại (`5, 2, 1`) mới chỉ đổi chỗ MỘT phần — `5` và
`2` đã đổi chỗ nhau, nhưng `1` (số nhỏ nhất) còn cách xa vị trí đúng
của nó (đầu mảng), phải đợi thêm lượt sau mới trồi dần lên được.

Vòng trong ở lượt sau NGẮN HƠN lượt trước đúng một bước — `range(n - 1
- luot)` — vì phần đuôi đã chắc chắn đúng chỗ, không cần so lại nữa.
::::

::::predict{#doan-sau-luot-dau commitOnce}
Mảng `[4, 1, 3, 2]`, chạy đúng thuật toán nổi bọt vừa học ở trên.

**Trước khi chạy**, bạn đoán: mảng trông thế nào NGAY SAU LƯỢT ĐẦU
TIÊN (chỉ một lượt, chưa sắp xong cả mảng)?

:::opt{correct}
`[1, 3, 2, 4]` — vòng trong đi hết cả lượt: so `(4,1)` đổi chỗ, so
`(4,3)` đổi chỗ, so `(4,2)` đổi chỗ — số lớn nhất `4` trồi thẳng về
cuối, còn `1, 3, 2` mới đổi chỗ một phần
:::

:::opt
`[1, 4, 3, 2]` — chỉ cặp đầu tiên lệch thứ tự nên chỉ đổi chỗ đúng một
lần rồi dừng lượt đó lại
::why
Gần đúng ở việc bạn thấy đúng cặp đổi chỗ ĐẦU TIÊN — `4` và `1` — và
đúng là chúng đổi chỗ trước tiên.

Chỗ lệch: vòng lặp TRONG của một lượt không dừng lại sau lần đổi chỗ
đầu tiên. Nó đi tiếp hết mọi cặp còn lại trong CÙNG lượt đó — sau khi
đổi `(4,1)`, mảng thành `[1, 4, 3, 2]`, rồi lập tức so tiếp `(4, 3)` ở
vị trí kế, thấy lệch nên đổi tiếp, rồi so `(4, 2)`, đổi tiếp lần nữa.
::
:::

:::opt
`[1, 2, 3, 4]` — mảng chỉ có bốn phần tử nên nổi bọt sắp xong luôn
trong một lượt
::why
Gần đúng ở kết quả CUỐI CÙNG — mảng thật sự sẽ về đúng `[1, 2, 3, 4]`,
điều đó không sai.

Chỗ lệch nằm ở SỐ LƯỢT cần để tới đó. Đó là kết quả sau BA lượt, không
phải sau lượt đầu tiên. Mỗi lượt chỉ chắc chắn đưa đúng MỘT phần tử
(lớn nhất còn lại) về đúng chỗ; các phần tử khác có thể vẫn còn sai vị
trí, phải đợi lượt sau.
::
:::

:::opt
`[4, 3, 2, 1]` — so cặp liền kề, đổi chỗ nếu bên trái NHỎ HƠN bên phải
::why
Gần đúng ở việc bạn có so sánh từng cặp liền kề và có đổi chỗ đúng lúc
chúng lệch nhau — đúng cơ chế chung của nổi bọt.

Chỗ lệch nằm ở CHIỀU của phép so sánh. Bài đang sắp mảng TĂNG DẦN, nên
chỉ đổi chỗ khi ô bên trái LỚN HƠN ô bên phải (`mang[i] > mang[i +
1]`) — điều kiện bạn dùng bị đảo ngược, và đảo ngược điều kiện đó sẽ
xếp mảng GIẢM DẦN thay vì tăng dần.
::
:::
::::

::::code{#viet-noi-bot-va-dem-buoc}
Hoàn thiện `sap_xep_noi_bot` — thêm một biến đếm `so_sanh` (kỹ thuật
bài 6, 7) và chính lệnh ĐỔI CHỖ khi phát hiện một cặp sai thứ tự.

```python title=starter
def sap_xep_noi_bot(mang):
    n = len(mang)
    so_sanh = 0
    for luot in range(n - 1):
        da_doi_cho = False
        for i in range(n - 1 - luot):
            so_sanh += 1
            if mang[i] > mang[i + 1]:
                mang[i], mang[i + 1] = ___, ___
                da_doi_cho = True
        if not da_doi_cho:
            break
    return mang, so_sanh


mang_nho = [6, 5, 4, 3, 2, 1]
ket_qua, dem_nho = sap_xep_noi_bot(mang_nho)
print(f"Đã sắp: {ket_qua}")
print(f"6 phần tử ngược hẳn: {dem_nho} phép so sánh")

mang_10 = list(range(10, 0, -1))
_, dem_10 = sap_xep_noi_bot(mang_10)
mang_20 = list(range(20, 0, -1))
_, dem_20 = sap_xep_noi_bot(mang_20)
print(f"10 phần tử ngược hẳn: {dem_10} phép so sánh")
print(f"20 phần tử ngược hẳn: {dem_20} phép so sánh")
```

```python title=solution
def sap_xep_noi_bot(mang):
    n = len(mang)
    so_sanh = 0
    for luot in range(n - 1):
        da_doi_cho = False
        for i in range(n - 1 - luot):
            so_sanh += 1
            if mang[i] > mang[i + 1]:
                mang[i], mang[i + 1] = mang[i + 1], mang[i]
                da_doi_cho = True
        if not da_doi_cho:
            break
    return mang, so_sanh


mang_nho = [6, 5, 4, 3, 2, 1]
ket_qua, dem_nho = sap_xep_noi_bot(mang_nho)
print(f"Đã sắp: {ket_qua}")
print(f"6 phần tử ngược hẳn: {dem_nho} phép so sánh")

mang_10 = list(range(10, 0, -1))
_, dem_10 = sap_xep_noi_bot(mang_10)
mang_20 = list(range(20, 0, -1))
_, dem_20 = sap_xep_noi_bot(mang_20)
print(f"10 phần tử ngược hẳn: {dem_10} phép so sánh")
print(f"20 phần tử ngược hẳn: {dem_20} phép so sánh")
```

```python title=test
assert ket_qua == [1, 2, 3, 4, 5, 6], f"mảng sáu phần tử phải sắp thành [1, 2, 3, 4, 5, 6] — đang ra {ket_qua}"
assert dem_nho == 15, f"với 6 phần tử ngược hẳn (trường hợp xấu nhất), số phép so sánh phải đúng 5+4+3+2+1 = 15 — đang đếm được {dem_nho}"
assert dem_10 == 45, f"với 10 phần tử ngược hẳn, số phép so sánh phải đúng 9+8+...+1 = 45 — đang đếm được {dem_10}"
assert dem_20 == 190, f"với 20 phần tử ngược hẳn, số phép so sánh phải đúng 19+18+...+1 = 190 — đang đếm được {dem_20}"
assert dem_20 > 4 * dem_10, f"gấp đôi dữ liệu (10 lên 20) phải làm số bước tăng NHIỀU HƠN gấp bốn lần, đúng hình dạng O(n²) — đang thấy {dem_10} lên {dem_20}"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm trên CÙNG một dòng, cùng làm MỘT việc — đổi chỗ hai ô mang[i] và mang[i + 1] cho nhau. Đây chính là hành động "nổi bọt" của thuật toán.
- kind: strategy
  body: 'Muốn đổi chỗ mang[i] và mang[i + 1], vế phải của phép gán phải là GIÁ TRỊ MỚI cho từng vế trái, theo đúng thứ tự: mang[i] nhận giá trị cũ của mang[i + 1], còn mang[i + 1] nhận giá trị cũ của mang[i].'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là mang[i + 1] và mang[i].'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hai chỗ trống phải THẬT SỰ đọc lại hai ô mang[i] và mang[i + 1] để đổi chỗ chúng — không gõ True/1/0 hay một giá trị cố định khác; bài này còn dạy cách TỰ VIẾT thuật toán sắp xếp bằng tay nên không gọi sorted()/.sort() có sẵn, vì như vậy sẽ không đếm được một phép so sánh nào cả, và không đo được hình dạng O(n²) mà bài đang chứng minh
  requireAst:
  - kind: uses-name, target: mang, min: 8
  # min: 8 — đếm thật trên toàn bộ khối (hàm + phần gọi bên dưới): solution
  # có đúng 8 lần đọc "mang" (6 lần đã có sẵn trong khung — mang[i],
  # mang[i+1] ở dòng if, tham số mang lặp trong các lời gọi hàm ở dưới —
  # cộng 2 lần ở đúng hai chỗ trống: mang[i + 1] và mang[i]). Điền True/True
  # (hoặc 1/1, 0/0) vào hai chỗ trống chỉ còn 6 lần đọc "mang" — dưới 8,
  # luật này chặn được. ĐÃ THỬ THẬT qua kiemAst(): solution đạt (8), cả ba
  # cách điền True/1/0 đều bị chặn (6, dưới ngưỡng).
  forbidAst:
  - kind: uses-call, target: sorted
  - kind: uses-call, target: sort
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Đã sắp: [1, 2, 3, 4, 5, 6]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
15, rồi 45, rồi 190 — đúng nhịp O(n²): gấp đôi dữ liệu, bước tăng gần
gấp bốn, không phải gấp đôi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nổi bọt đổi chỗ RẤT NHIỀU lần — gần như mỗi phép so sánh sai thứ tự
đều kéo theo một lần đổi chỗ ngay lập tức, dù phần tử đó còn phải đi
xa mới tới đúng vị trí cuối cùng. Với mảng sáu phần tử ngược hẳn ở
trên, có tới 15 phép so sánh — và gần như tất cả đều dẫn tới một lần
đổi chỗ.

Có cách nào so sánh gần bằng chừng đó, nhưng đổi chỗ ÍT HƠN hẳn — mỗi
phần tử chỉ cần di chuyển ĐÚNG MỘT LẦN, thẳng tới vị trí cuối cùng của
nó, thay vì nhích dần qua nhiều lượt?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
