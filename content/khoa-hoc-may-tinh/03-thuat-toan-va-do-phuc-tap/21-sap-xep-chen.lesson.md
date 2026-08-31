---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.sap-xep-chen
title: "Sắp xếp chèn: chèn từng phần tử vào đúng chỗ trong phần đã sắp"
summary: "Giữ một phần ĐÃ SẮP ở đầu mảng, lấy từng phần tử tiếp theo và chèn nó vào đúng chỗ trong phần đã sắp — đúng cái giá của chèn giữa mảng (dời chỗ) đã học ở T3.2, giờ dùng chính cái giá đó để làm một thuật toán."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.insertion-sort]
requires: [alg.selection-sort]
concepts: [alg.insertion-sort]
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
Xếp bài trên tay: mỗi lá mới rút được, bạn không vứt tung lên rồi sắp
lại từ đầu — bạn luồn nó vào đúng khe hở giữa những lá đã gọn.
::::

::::explain{#giu-phan-da-sap}
**Sắp xếp chèn** (insertion sort) làm đúng việc người xếp bài vẫn làm.
Nó luôn giữ một PHẦN ĐÃ SẮP ở đầu mảng (ban đầu, phần đó chỉ có đúng
một phần tử — một phần tử luôn tự nó đã "sắp xong"). Rồi lần lượt lấy
từng phần tử TIẾP THEO còn chưa sắp, gọi nó là **khoá**, và chèn khoá
đó vào đúng vị trí của nó trong phần đã sắp — không phải chèn vào cuối
rồi sắp lại từ đầu, mà so khoá với từng phần tử trong phần đã sắp, TỪ
PHẢI SANG TRÁI, và dời những phần tử LỚN HƠN khoá sang phải một bước
để nhường chỗ, đúng cái giá của việc chèn giữa mảng mà T3.2 đã đo: mọi
phần tử phía sau vị trí chèn phải dời một bước, không có cách nào né
được việc đó.

Khi gặp một phần tử NHỎ HƠN HOẶC BẰNG khoá (hoặc hết phần đã sắp để so
— tức đã lùi về tận đầu mảng), việc dời dừng lại, và khoá được đặt
đúng vào chỗ trống vừa dời ra.

Ở trường hợp XẤU NHẤT — mảng ngược hẳn — mỗi khoá mới đều nhỏ hơn MỌI
phần tử đã có trong phần đã sắp, nên phải dời TOÀN BỘ phần đã sắp mỗi
lần. Số lần dời cộng dồn lại đúng `n(n-1)/2` — lại một lần nữa **O(n²)**,
cùng hình dạng với nổi bọt và chọn, dù cách viết ra hoàn toàn khác.
::::

::::example{#theo-doi-tung-lan-chen}
Byte chèn từng phần tử của `[4, 1, 3, 2]`, in ra mảng sau MỖI lần chèn
xong một phần tử mới:

```python title=readonly
def sap_xep_chen(mang):
    n = len(mang)
    for i in range(1, n):
        khoa = mang[i]
        j = i - 1
        while j >= 0 and mang[j] > khoa:
            mang[j + 1] = mang[j]
            j -= 1
        mang[j + 1] = khoa
        print(f"Sau khi chèn phần tử thứ {i + 1} (giá trị {khoa}): {mang}")
    return mang

sap_xep_chen([4, 1, 3, 2])
```

```text title=readonly
Sau khi chèn phần tử thứ 2 (giá trị 1): [1, 4, 3, 2]
Sau khi chèn phần tử thứ 3 (giá trị 3): [1, 3, 4, 2]
Sau khi chèn phần tử thứ 4 (giá trị 2): [1, 2, 3, 4]
```

Nhìn dòng cuối kỹ hơn: khoá `2` phải dời CẢ `4` LẪN `3` sang phải —
`4` lớn hơn `2` nên dời, rồi `3` cũng lớn hơn `2` nên dời tiếp, tới khi
gặp `1` (nhỏ hơn `2`) thì dừng, và `2` được đặt vào đúng khe vừa mở ra.
Hai lần dời cho đúng MỘT khoá — không phải một lần đổi chỗ đơn giản.
::::

::::predict{#doan-lan-chen-cuoi commitOnce}
Mảng đang ở trạng thái `[1, 3, 4, 2]` — đã chèn xong ba phần tử đầu
theo đúng thuật toán vừa học. Bước tiếp theo chèn nốt phần tử cuối
cùng, khoá `2`.

**Trước khi chạy**, bạn đoán: mảng trông thế nào SAU KHI chèn xong
khoá `2` này?

:::opt{correct}
`[1, 2, 3, 4]` — khoá `2` so với `4` (lớn hơn, dời), rồi so với `3`
(cũng lớn hơn, dời tiếp), rồi so với `1` (nhỏ hơn, dừng) — cả `4` lẫn
`3` đều phải dời sang phải một bước để nhường chỗ
:::

:::opt
`[1, 3, 2, 4]` — chỉ đổi chỗ `4` và `2` (hai ô liền kề cuối mảng) rồi
dừng lại
::why
Gần đúng ở việc bạn thấy đúng `4` — phần tử ngay cạnh khoá — phải
nhường chỗ trước tiên.

Chỗ lệch: sắp xếp chèn không dừng lại sau một lần đổi chỗ liền kề duy
nhất. Nó tiếp tục so khoá với phần tử KẾ TIẾP về phía trái trong phần
đã sắp — ở đây là `3` — và `3` cũng lớn hơn `2`, nên cũng phải dời.
Việc dời chỉ dừng khi gặp một phần tử NHỎ HƠN HOẶC BẰNG khoá (ở đây là
`1`), không phải sau đúng một bước.
::
:::

:::opt
`[2, 1, 3, 4]` — khoá luôn được chèn thẳng về ĐẦU mảng vì nó nhỏ nhất
trong nhóm đang xét
::why
Gần đúng ở việc `2` đúng là nhỏ hơn cả `3` và `4`.

Chỗ lệch: khoá không nhảy thẳng về vị trí đầu tiên bất kể thế nào — nó
chỉ dời qua những phần tử LỚN HƠN chính nó, rồi dừng lại NGAY khi gặp
một phần tử nhỏ hơn hoặc bằng nó. Ở đây `1` nhỏ hơn `2`, nên `1` vẫn
đứng yên ở đầu mảng — `2` chỉ chèn vào ngay SAU `1`, không đi xa hơn.
::
:::

:::opt
`[1, 3, 4, 2]` — mảng không đổi, vì `2` đã đứng gần cuối và không cần
dời đi đâu cả
::why
Gần đúng ở việc `2` đúng là phần tử đang được xét tới.

Chỗ lệch: phần đã sắp `[1, 3, 4]` không đứng yên khi có một khoá nhỏ
hơn cần chèn vào giữa nó. Mọi phần tử trong phần đã sắp mà LỚN HƠN
khoá đều phải dời sang phải để nhường chỗ — ở đây là cả `3` lẫn `4`,
không phải không ai dời cả.
::
:::
::::

::::code{#viet-chen-va-dem-dich-chuyen}
Hoàn thiện `sap_xep_chen` — điền phần tử được DỜI trong vòng lặp, và
vị trí ĐẶT khoá sau khi vòng lặp dừng lại.

Chú ý: dòng `while` và dòng `j -= 1` giữ nguyên, không đụng tới — hai
chỗ trống dưới đây chỉ ảnh hưởng tới GIÁ TRỊ được ghi vào mảng, không
ảnh hưởng gì tới việc vòng lặp có dừng hay không.

```python title=starter
def sap_xep_chen(mang):
    n = len(mang)
    so_lan_dich_chuyen = 0
    for i in range(1, n):
        khoa = mang[i]
        j = i - 1
        while j >= 0 and mang[j] > khoa:
            mang[j + 1] = mang[___]   # dời phần tử lớn hơn sang phải một ô
            so_lan_dich_chuyen += 1
            j -= 1
        mang[___] = khoa              # đặt khoá vào đúng khe trống vừa dời ra
    return mang, so_lan_dich_chuyen


mang_nho = [6, 5, 4, 3, 2, 1]
ket_qua, dich_nho = sap_xep_chen(mang_nho)
print(f"Đã sắp: {ket_qua}")
print(f"6 phần tử ngược hẳn: {dich_nho} lần dịch chuyển")

mang_10 = list(range(10, 0, -1))
_, dich_10 = sap_xep_chen(mang_10)
mang_20 = list(range(20, 0, -1))
_, dich_20 = sap_xep_chen(mang_20)
print(f"10 phần tử ngược hẳn: {dich_10} lần dịch chuyển")
print(f"20 phần tử ngược hẳn: {dich_20} lần dịch chuyển")
```

```python title=solution
def sap_xep_chen(mang):
    n = len(mang)
    so_lan_dich_chuyen = 0
    for i in range(1, n):
        khoa = mang[i]
        j = i - 1
        while j >= 0 and mang[j] > khoa:
            mang[j + 1] = mang[j]
            so_lan_dich_chuyen += 1
            j -= 1
        mang[j + 1] = khoa
    return mang, so_lan_dich_chuyen


mang_nho = [6, 5, 4, 3, 2, 1]
ket_qua, dich_nho = sap_xep_chen(mang_nho)
print(f"Đã sắp: {ket_qua}")
print(f"6 phần tử ngược hẳn: {dich_nho} lần dịch chuyển")

mang_10 = list(range(10, 0, -1))
_, dich_10 = sap_xep_chen(mang_10)
mang_20 = list(range(20, 0, -1))
_, dich_20 = sap_xep_chen(mang_20)
print(f"10 phần tử ngược hẳn: {dich_10} lần dịch chuyển")
print(f"20 phần tử ngược hẳn: {dich_20} lần dịch chuyển")
```

```python title=test
assert ket_qua == [1, 2, 3, 4, 5, 6], f"mảng sáu phần tử phải sắp thành [1, 2, 3, 4, 5, 6] — đang ra {ket_qua}"
assert dich_nho == 15, f"với 6 phần tử ngược hẳn (trường hợp xấu nhất), số lần dịch chuyển phải đúng 15 — đang đếm được {dich_nho}"
assert dich_10 == 45, f"với 10 phần tử ngược hẳn, số lần dịch chuyển phải đúng 45 — đang đếm được {dich_10}"
assert dich_20 == 190, f"với 20 phần tử ngược hẳn, số lần dịch chuyển phải đúng 190 — đang đếm được {dich_20}"
assert dich_20 > 4 * dich_10, f"gấp đôi dữ liệu (10 lên 20) phải làm số bước tăng nhiều hơn gấp bốn lần, đúng hình dạng O(n²) — đang thấy {dich_10} lên {dich_20}"
```

:::hints
- kind: attention
  body: Chỗ trống 1 nằm TRONG vòng while — phải ghi lại giá trị CŨ của phần tử đang xét (mang[j]) trước khi nó bị ghi đè, đúng ý nghĩa "dời sang phải". Chỗ trống 2 nằm SAU vòng while — đặt khoá vào đúng khe trống mà j đang trỏ tới cộng thêm một.
- kind: strategy
  body: 'Chỗ trống 1: j — mang[j + 1] = mang[j] có nghĩa "ô bên phải nhận đúng giá trị đang có ở ô j". Chỗ trống 2: j + 1 — sau khi vòng while dừng, j đang trỏ vào ô ngay TRƯỚC khe trống, nên khe trống ấy là j + 1.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là j và j + 1.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hai chỗ trống phải THẬT SỰ dùng biến j — chỗ trống 1 đọc lại mang[j] trước khi ghi đè, chỗ trống 2 đặt khoá vào ô j + 1 — không gõ True/1/0 hay một chỉ số cố định khác; bài này còn dạy cách TỰ VIẾT thuật toán sắp xếp bằng tay nên không gọi sorted()/.sort() có sẵn
  requireAst:
  - kind: uses-name, target: j, min: 5
  # min: 5 — đếm thật trên toàn bộ khối: solution có đúng 5 lần đọc "j"
  # (3 lần đã có sẵn trong khung — điều kiện while j >= 0, mang[j] > khoa,
  # j -= 1 — cộng 2 lần ở đúng hai chỗ trống: mang[j] và j + 1). Điền
  # True/True (hoặc 1/1, 0/0) vào hai chỗ trống chỉ còn 3 lần đọc "j" —
  # dưới 5, luật này chặn được. ĐÃ THỬ THẬT qua kiemAst(): solution đạt
  # (5), cả ba cách điền True/1/0 đều bị chặn (3, dưới ngưỡng).
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
15, rồi 45, rồi 190 — cùng con số đúng như nổi bọt, dù cách viết hoàn
toàn khác: một bên đổi chỗ liền kề, một bên dời cả một dải.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ba thuật toán vừa học — nổi bọt, chọn, chèn — trông khác hẳn nhau khi
đọc mã: một bên đổi chỗ liên tục, một bên chỉ tìm-rồi-đổi một lần, một
bên dời cả dải phần tử. Nhưng con số đếm được ở cả ba, trên đúng cùng
một cỡ dữ liệu ngược hẳn, lại giống nhau tới kỳ lạ — cả ba đều 15, đều
45, đều 190.

Có phải trùng hợp không? Nếu đổi sang một bộ dữ liệu KHÁC — không phải
ngược hẳn, mà lộn xộn theo một cách khác — ba con số đó có còn giống
nhau không, hay sẽ lộ ra khác biệt thật sự giữa chúng?

Bài sau đo thẳng trên cùng một dữ liệu, cả ba thuật toán cùng lúc.
::::

::::checkpoint{mastery=0.8}
::::
