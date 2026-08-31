---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.tim-tuyen-tinh
title: "Tìm tuyến tính: dò từng ô, không bỏ sót"
summary: "Đi từ ô 0 tới ô cuối, so từng ô một với giá trị cần tìm. Không đòi hỏi dữ liệu đã sắp xếp — và chính vì vậy, ca xấu nhất luôn phải chạm ĐỦ MỌI ô, không có đường tắt nào."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [alg.linear-search]
requires: [ds.array-index-address, alg.worst-case]
concepts: [alg.linear-search]
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
Mười hai bài vừa rồi học cách ĐO. Giờ đo việc đầu tiên có tên hẳn hoi: lục
tìm một giá trị trong một dãy chưa ai sắp xếp.
::::

::::explain{#di-tu-o-0-toi-het}
Bài `ds.array-index-address` (T3.2) dạy một điều mạnh: biết chỉ số `i`,
máy tính thẳng ra ĐỊA CHỈ của ô đó bằng một phép nhân, không cần đi bộ qua
ô nào trước nó. Nhưng biết ĐỊA CHỈ không có nghĩa biết ô đó đang CHỨA GÌ —
muốn biết nội dung, phải thật sự ĐỌC ô đó ra và so với giá trị đang tìm.

Với một dãy chưa ai sắp xếp, không có cách nào đoán trước giá trị cần tìm
nằm ở ô nào. Chỉ còn một cách chắc chắn: đọc ô 0, so sánh; không khớp thì
đọc ô 1, so sánh; cứ thế, không bỏ qua ô nào, cho tới khi khớp hoặc hết
dãy. Cách làm này có tên: **tìm tuyến tính** (linear search) — "tuyến
tính" vì số ô phải đọc tỉ lệ thẳng theo độ dài dãy, đúng hình dạng bài 9
đã đặt tên.

Đây chính là thứ bài 11 đã ngầm nhắc tới khi nói về ca xấu nhất: giá trị
cần tìm nằm Ở Ô CUỐI CÙNG, hoặc không hề có mặt trong dãy. Ở cả hai tình
huống đó, tìm tuyến tính buộc phải đọc và so sánh ĐỦ MỌI ô — không có
đường nào bỏ bớt việc, không có cách nào biết trước mà không đọc.

Điểm mạnh của nó nằm ngay trong cái giá phải trả: tìm tuyến tính không
đòi hỏi GÌ về thứ tự dữ liệu. Dữ liệu lộn xộn tới đâu, nó vẫn chạy đúng.
Đây là cách DUY NHẤT dùng được khi dữ liệu còn chưa sắp xếp — bài sau sẽ
cho xem một cách nhanh hơn, nhưng cách đó đòi một điều kiện mà tìm tuyến
tính không cần.
::::

::::example{#tim-diem-thi}
Byte có tám điểm thi, nhập lộn xộn, chưa hề sắp xếp. Tìm ba giá trị khác
nhau, đếm số bước (số lần so sánh) mỗi lần:

```python title=readonly
def tim_tuyen_tinh(danh_sach, can_tim):
    so_buoc = 0
    for i in range(len(danh_sach)):
        so_buoc += 1
        if danh_sach[i] == can_tim:
            return i, so_buoc
    return -1, so_buoc

diem_thi = [67, 92, 45, 78, 23, 89, 56, 34]

print(tim_tuyen_tinh(diem_thi, 89))
print(tim_tuyen_tinh(diem_thi, 34))
print(tim_tuyen_tinh(diem_thi, 100))
```

```text title=readonly
(5, 6)
(7, 8)
(-1, 8)
```

`89` nằm gần cuối dãy — sáu bước là tìm ra. `34` là phần tử CUỐI CÙNG —
phải đọc đủ tám ô (đúng bằng độ dài dãy) mới chạm tới nó. `100` không hề
có mặt trong dãy — cũng phải đọc đủ tám ô mới biết chắc điều đó.

Nhìn kỹ hai dòng cuối: một ca "tìm thấy" (ở vị trí cuối) và một ca "không
tìm thấy" cho ra CÙNG một số bước — tám. Đó chính là ca xấu nhất của tìm
tuyến tính: cho tới khi đã đọc hết dãy, nó không có cách nào phân biệt
"giá trị nằm ở tít cuối" với "giá trị không hề tồn tại".
::::

::::predict{#doan-ca-xau-nhat commitOnce}
Một dãy có N ô, hoàn toàn chưa sắp xếp — N có thể là 8, có thể là
8.000.000. Tìm tuyến tính chạy trên dãy đó.

**Trước khi đọc tiếp**, bạn đoán: ở ca XẤU NHẤT, số bước (số lần so
sánh) tìm tuyến tính phải thực hiện là bao nhiêu, tính theo N?

:::opt{correct}
Đúng N bước — dù giá trị cần tìm nằm ở ô cuối cùng hay không hề có mặt,
cả hai đều buộc phải đọc và so sánh đủ N ô, không thiếu ô nào
:::

:::opt
Khoảng N/2 bước, vì tính trung bình giá trị cần tìm hay rơi vào đâu đó ở
giữa dãy
::why
Gần đúng ở một chỗ có thật: nếu đo TRUNG BÌNH trên rất nhiều lần tìm với
vị trí ngẫu nhiên, con số N/2 đúng là hợp lý — quan sát đó không sai.

Chỗ lệch là câu hỏi hỏi về ca XẤU NHẤT, không phải ca trung bình. Bài 11
đã định nghĩa rõ: ca xấu nhất là tình huống TỆ NHẤT có thể xảy ra, không
phải tình huống hay gặp nhất. Với tìm tuyến tính, ca xấu nhất luôn là N
bước đủ, không bao giờ dừng ở N/2.
::
:::

:::opt
Đúng 1 bước, vì máy tính thẳng ra địa chỉ của bất kỳ ô nào chỉ bằng một
phép nhân, không cần đi qua ô nào trước
::why
Gần đúng ở việc bạn nhớ đúng bài trước: TÍNH ĐỊA CHỈ của ô thứ `i` đúng
là một phép toán, không phụ thuộc `i` lớn hay nhỏ — điều đó không sai.

Chỗ lệch: tính ra địa chỉ không phải là xong việc. Còn phải ĐỌC ô ở địa
chỉ đó ra và SO SÁNH nó với giá trị cần tìm. Chính bước đọc-và-so-sánh ấy
mới là thứ phải LẶP LẠI, không phải bước tính địa chỉ — và với dữ liệu
chưa sắp xếp, không có cách nào bỏ bớt việc so sánh đó.
::
:::

:::opt
Khoảng log N bước, giống những thuật toán "khôn" vẫn hay được nhắc tới
::why
Gần đúng ở việc có TỒN TẠI những cách tìm nhanh hơn nhiều so với đọc hết
dãy — trực giác đó đúng, và một cách như vậy sẽ xuất hiện ngay bài sau.

Chỗ lệch: cách nhanh hơn đó đòi một ĐIỀU KIỆN mà bài này cố tình chưa
có — dữ liệu phải được sắp xếp SẴN. Dãy trong bài này hoàn toàn lộn xộn,
không có gì để dựa vào ngoài việc đọc từng ô một, nên số bước ca xấu
nhất vẫn phải là N, không thể ít hơn.
::
:::
::::

::::code{#dem-buoc-tim-tuyen-tinh}
Byte có tám điểm thi. Viết nốt hàm `tim_tuyen_tinh`: đi từng ô một, đếm
số bước, và báo đúng vị trí khi tìm thấy (hoặc `-1` khi không có).

```python title=starter
def tim_tuyen_tinh(danh_sach, can_tim):
    so_buoc = 0
    for i in range(len(danh_sach)):
        so_buoc += 1
        if ___:                        # ô hiện tại có đúng bằng giá trị cần tìm không?
            return i, so_buoc
    return ___, so_buoc                # không tìm thấy sau khi đã đọc hết dãy

diem_thi = [67, 92, 45, 78, 23, 89, 56, 34]

vi_tri_89, buoc_89 = tim_tuyen_tinh(diem_thi, 89)
vi_tri_34, buoc_34 = tim_tuyen_tinh(diem_thi, 34)
vi_tri_100, buoc_100 = tim_tuyen_tinh(diem_thi, 100)

print(f"Tìm 89: vị trí {vi_tri_89}, mất {buoc_89} bước")
print(f"Tìm 34: vị trí {vi_tri_34}, mất {buoc_34} bước")
print(f"Tìm 100: vị trí {vi_tri_100}, mất {buoc_100} bước")
```

```python title=solution
def tim_tuyen_tinh(danh_sach, can_tim):
    so_buoc = 0
    for i in range(len(danh_sach)):
        so_buoc += 1
        if danh_sach[i] == can_tim:
            return i, so_buoc
    return -1, so_buoc

diem_thi = [67, 92, 45, 78, 23, 89, 56, 34]

vi_tri_89, buoc_89 = tim_tuyen_tinh(diem_thi, 89)
vi_tri_34, buoc_34 = tim_tuyen_tinh(diem_thi, 34)
vi_tri_100, buoc_100 = tim_tuyen_tinh(diem_thi, 100)

print(f"Tìm 89: vị trí {vi_tri_89}, mất {buoc_89} bước")
print(f"Tìm 34: vị trí {vi_tri_34}, mất {buoc_34} bước")
print(f"Tìm 100: vị trí {vi_tri_100}, mất {buoc_100} bước")
```

```python title=test
assert (vi_tri_89, buoc_89) == (5, 6), f"tìm 89 phải ra vị trí 5, mất 6 bước — đang ra {(vi_tri_89, buoc_89)}"
assert (vi_tri_34, buoc_34) == (7, 8), f"34 nằm ở ô CUỐI CÙNG (vị trí 7) — phải mất đủ 8 bước mới chạm tới, đang ra {(vi_tri_34, buoc_34)}"
assert (vi_tri_100, buoc_100) == (-1, 8), f"100 không có mặt trong dãy — phải trả về -1 sau khi đã đọc đủ 8 ô, đang ra {(vi_tri_100, buoc_100)}"
assert buoc_34 == len(diem_thi), "ca xấu nhất (tìm thấy ở ô cuối) phải mất ĐÚNG BẰNG độ dài dãy, không hơn không kém"
assert buoc_100 == len(diem_thi), "ca không tìm thấy cũng phải mất ĐÚNG BẰNG độ dài dãy — không có cách nào biết sớm hơn khi dữ liệu chưa sắp xếp"
```

:::hints
- kind: attention
  body: Chỗ trống đầu nằm trong điều kiện `if` — phải THẬT SỰ so sánh nội dung ô `danh_sach[i]` với `can_tim`, không phải một câu luôn đúng hay luôn sai. Chỗ trống sau nằm ở dòng cuối, chạy khi vòng lặp đã đi hết dãy mà chưa `return` — nghĩa là không tìm thấy.
- kind: strategy
  body: 'Chỗ trống 1 hỏi "ô hiện tại có khớp không" — so sánh bằng `==` giữa `danh_sach[i]` và `can_tim`. Chỗ trống 2 là giá trị báo "không tìm thấy" — theo đúng quy ước dòng test đang chờ, đó là `-1`, một chỉ số không bao giờ có thật trong một dãy.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `danh_sach[i] == can_tim` và `-1`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống 1 phải THẬT SỰ so sánh danh_sach[i] với can_tim bằng == — không phải một câu luôn đúng/luôn sai như True, 1, 0; máy phải ĐỌC ô và SO với giá trị cần tìm, đúng trọng tâm bài này
  requireAst:
  - kind: uses-name, target: can_tim, min: 1
  - kind: uses-name, target: danh_sach, min: 2
  - kind: uses-operator, target: "=="
  # can_tim min:1 — trong khung (không tính chỗ trống), can_tim chỉ xuất
  # hiện làm THAM SỐ hàm (không phải Name-Load, không bị đếm) — nên số đếm
  # thật là 0 nếu chỗ trống không chạm tới nó, và đúng 1 nếu chỗ trống viết
  # đúng "danh_sach[i] == can_tim". ĐÃ THỬ True/1/0: cả ba đều cho can_tim
  # đếm được 0, dưới ngưỡng 1 — chặn được.
  #
  # danh_sach min:2 — trong khung, danh_sach xuất hiện sẵn đúng 1 lần ở
  # "range(len(danh_sach))"; lời giải đúng thêm đúng 1 lần nữa ở
  # "danh_sach[i]" trong chỗ trống, ra tổng 2. ĐÃ THỬ True/1/0: cả ba giữ
  # nguyên ở 1, dưới ngưỡng 2 — chặn được cả ba, không chỉ chặn MỘT phía
  # của phép so sánh (đúng bài học "đếm cả hai tên tham gia so sánh").
  #
  # ĐÃ THỬ THẬT (script ngoài, ba cách điền hụt True/1/0 cho cả hai chỗ
  # trống, chín tổ hợp): mọi tổ hợp chạy XONG trong vài mili-giây (vòng
  # `for` có `range` cố định, không có rủi ro treo), và ra kết quả SAI
  # khác hẳn (5,6)/(7,8)/(-1,8) — bị tests và output bắt độc lập.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Tìm 89: vị trí 5, mất 6 bước\\nTìm 34: vị trí 7, mất 8 bước\\nTìm 100: vị trí -1, mất 8 bước\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu bước tìm ra 89. Tám bước để biết chắc 34 nằm ở tít cuối — và đúng tám
bước y hệt để biết chắc 100 không hề có mặt. Không có đường tắt nào khi
dữ liệu còn lộn xộn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Tám bước để tìm trong tám ô — đúng bằng số ô. Nhưng giả sử bạn được PHÉP
đòi hỏi thêm một điều về dữ liệu trước khi tìm — chẳng hạn, dãy đó đã
được SẮP XẾP TĂNG DẦN từ trước, không còn lộn xộn nữa.

T3.2 đã cho bạn thấy một cây tìm kiếm nhị phân tận dụng đúng kiểu trật tự
đó để bỏ hẳn một nửa cây mỗi bước, không cần ghé qua từng nút. Trên một
CÂY thì làm được. Vậy trên một MẢNG — dãy ô liền kề, đánh số thẳng — thứ
trật tự "đã sắp xếp" đó có tận dụng được theo cách tương tự không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
