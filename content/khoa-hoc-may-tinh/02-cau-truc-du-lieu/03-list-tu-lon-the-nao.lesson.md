---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.list-tu-lon-the-nao
title: "Vì sao list Python tự lớn được"
summary: "Mảng liền kề có cỡ cố định khi cấp phát; list Python tự lớn bằng cách thỉnh thoảng xin một vùng mới lớn hơn rồi chép hết sang — không phải mọi .append đều rẻ như nhau."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ds.dynamic-array]
requires: [ds.array-index-address, core.list, core.list-append, ctrl.for-range, ctrl.if, core.builtin-function, core.variable, core.assignment]
concepts: [ds.dynamic-array]
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
Dãy tủ khoá đứng liền kề — bài 1 nói vậy. Nhưng nếu thêm một cái tủ vào
CUỐI hàng, phía sau nó là bức tường thì sao?
::::

::::explain{#cap-phat-co-dinh}
Bài 1 định nghĩa mảng là các ô LIỀN KỀ. Điều đó kéo theo một hệ quả ít ai
để ý: khi một dãy ô liền kề được cấp phát, nó có một CỠ CỐ ĐỊNH ngay từ lúc
đó — đúng bằng số ô mà máy đã xin chỗ. Muốn thêm một ô nữa vào NGAY SAU ô
cuối cùng, máy phải chắc chắn ô liền sau đó đang TRỐNG, chưa ai chiếm.
Nhưng phần bộ nhớ ngay sau một vùng đã cấp không có gì đảm bảo còn trống —
rất có thể một chương trình khác, hay chính chương trình này, đã xin chỗ
đó cho việc khác rồi.

Vậy khi bạn gọi `.append(...)` trên một `list` Python đã đầy hết chỗ dành
sẵn, máy không thể "nới" cái vùng cũ ra thêm một ô. Nó phải:

1. Xin một vùng MỚI, lớn hơn, ở một chỗ khác trong bộ nhớ.
2. Chép hết mọi phần tử từ vùng CŨ sang vùng MỚI.
3. Thêm phần tử mới vào cuối vùng MỚI.
4. Trả lại vùng CŨ.

Bốn bước đó tốn công hơn hẳn một `.append` bình thường — vốn chỉ là "ghi
một giá trị vào ô trống có sẵn". Nhưng nếu MỌI lần `.append` đều làm đủ
bốn bước này thì list sẽ chậm khủng khiếp. CPython (bản chạy Python bạn
đang dùng) khôn hơn thế: mỗi lần buộc phải xin vùng mới, nó xin DƯ ra một
ít — nhiều hơn số ô đang cần ngay lúc đó — để những lần `.append` KẾ TIẾP
còn chỗ trống sẵn mà dùng, không phải xin lại ngay.

Hệ quả: không phải mọi lần `.append` đều tốn công như nhau. Phần lớn các
lần chỉ là "ghi vào ô trống có sẵn" — rẻ. Nhưng thỉnh thoảng, khi chỗ dư đã
dùng hết, một lần `.append` phải làm đủ bốn bước trên — đắt hơn hẳn.
::::

::::example{#do-that-cho-nhay}
Python có `sys.getsizeof(...)` — đo được kích thước một `list` đang chiếm
bao nhiêu byte ngay lúc đó. Nếu kích thước đó không đổi sau một lần
`.append`, nghĩa là chỗ dư sẵn còn đủ dùng. Nếu nó NHẢY lên, nghĩa là máy
vừa phải xin vùng mới.

```python title=readonly
import sys

a = []
truoc = sys.getsizeof(a)
for i in range(10):
    a.append(i)
    sau = sys.getsizeof(a)
    if sau != truoc:
        print(f"độ dài {len(a)}: cỡ nhảy từ {truoc} lên {sau} byte")
    truoc = sau
```

```text title=readonly
độ dài 1: cỡ nhảy từ 28 lên 44 byte
độ dài 5: cỡ nhảy từ 44 lên 60 byte
độ dài 9: cỡ nhảy từ 60 lên 92 byte
```

Mười lần `.append`, nhưng chỉ BA lần cỡ thật sự nhảy — ở độ dài 1, 5, và 9.
Bảy lần còn lại (độ dài 2, 3, 4, rồi 6, 7, 8, rồi 10) cỡ đứng yên: chỗ dư
xin từ lần nhảy trước đó vẫn còn, nên `.append` chỉ việc ghi vào ô trống có
sẵn — không cần xin gì thêm.

Nhìn kỹ khoảng cách giữa các lần nhảy: từ độ dài 1 tới độ dài 5 (bốn lần
`.append` liền không nhảy), rồi từ 5 tới 9 (lại bốn lần không nhảy). Chỗ dư
mỗi lần xin không phải một con số cố định mãi mãi — nhưng luôn xin DƯ hơn
mức cần ngay lúc đó, đúng như phần giải thích phía trên đã nói.
::::

::::predict{#lan-6-hay-lan-9 commitOnce}
Byte dựng hai danh sách khác nhau về độ dài, rồi thêm đúng một phần tử vào
mỗi cái — một cái đang ở độ dài 5 (sắp thành độ dài 6), một cái đang ở độ
dài 8 (sắp thành độ dài 9):

```python
import sys

a = [0, 1, 2, 3, 4]
truoc_6 = sys.getsizeof(a)
a.append(99)
sau_6 = sys.getsizeof(a)

b = [0, 1, 2, 3, 4, 5, 6, 7]
truoc_9 = sys.getsizeof(b)
b.append(99)
sau_9 = sys.getsizeof(b)

print(truoc_6 == sau_6)
print(truoc_9 == sau_9)
```

**Trước khi chạy**, bạn đoán hai dòng in ra là gì?

:::opt{correct}
`True`, rồi `False` — độ dài 5 lên 6 không cần vùng mới; độ dài 8 lên 9 thì
cần.
:::

:::opt
`False`, rồi `True`
::why
Gần đúng ở việc bạn nhận ra CÓ một lần cần vùng mới và một lần không —
đúng tinh thần của ví dụ vừa xem.

Chỗ lệch là bạn đảo ngược thứ tự. Ví dụ vừa rồi cho thấy lần nhảy đứng ở
độ dài 5 và 9 — nghĩa là bước từ 8 lên 9 mới là bước CHẠM đúng điểm nhảy,
còn bước từ 5 lên 6 vẫn còn nằm giữa hai lần nhảy, nên không cần vùng mới.
::
:::

:::opt
`False`, rồi `False`
::why
Gần đúng ở việc bạn tin CÓ chuyện gì đó tốn kém xảy ra — cảnh giác đó
không sai với một bài đang nói về chi phí ẩn.

Chỗ lệch là bạn nghĩ MỌI lần `.append` đều tốn kém như nhau. Ví dụ vừa xem
đã chỉ rõ: giữa hai lần nhảy, cỡ đứng yên suốt bốn lần `.append` liên
tiếp — không phải lần nào cũng nhảy.
::
:::

:::opt
`True`, rồi `True`
::why
Gần đúng ở nửa đầu — bước từ độ dài 5 lên 6 đúng là không cần vùng mới.

Chỗ lệch là bạn tin một khi đã xin dư chỗ một lần thì dùng mãi không hết.
Chỗ dư có giới hạn: ví dụ vừa xem cho thấy nó CHẮC CHẮN hết vào một lúc nào
đó — cụ thể là ngay tại độ dài 9 — buộc máy phải xin vùng mới lần nữa.
::
:::
::::

::::code{#tim-cac-lan-nhay}
Byte muốn tự động tìm hết mọi lần "nhảy cỡ" khi thêm liên tiếp 20 phần tử
vào một danh sách rỗng — không đoán bằng mắt như ví dụ trên nữa, mà để
chương trình tự ghi lại.

```python title=starter
import sys

a = []
truoc = sys.getsizeof(a)
lan_nhay = []

for i in range(20):
    a.append(i)
    sau = sys.getsizeof(a)
    if ___:
        lan_nhay.append(len(a))
    truoc = sau

print(lan_nhay)
```

```python title=solution
import sys

a = []
truoc = sys.getsizeof(a)
lan_nhay = []

for i in range(20):
    a.append(i)
    sau = sys.getsizeof(a)
    if sau != truoc:
        lan_nhay.append(len(a))
    truoc = sau

print(lan_nhay)
```

```python title=test
assert lan_nhay == [1, 5, 9, 17], f"lan_nhay phải ghi đúng những độ dài mà cỡ THẬT SỰ đổi — đang ra {lan_nhay}"
assert len(a) == 20, "vòng lặp phải chạy đủ 20 lần .append, đừng sửa range(20)"
```

:::hints
- kind: attention
  body: Điều kiện cần kiểm là "cỡ có vừa NHẢY không" — so sánh `sau` với `truoc`, không phải so sánh với `len(a)` hay một con số cố định nào khác.
- kind: strategy
  body: "`truoc` là cỡ TRƯỚC lần append này, `sau` là cỡ SAU lần append đó. Nếu hai con số khác nhau, cỡ vừa nhảy — đúng lúc cần ghi lại độ dài hiện tại vào lan_nhay."
- kind: one-line
  body: "Điền `sau != truoc` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: chỗ trống phải THẬT SỰ so sánh sau với truoc để phát hiện đúng lúc cỡ vừa đổi — không phải một điều kiện luôn đúng (if True), không phải đoán theo độ dài (len(a) % 4), và không phải chép sẵn bốn con số [1, 5, 9, 17] đã biết trước; bài này đang dạy PHÁT HIỆN lúc cỡ nhảy, không phải nhớ lại kết quả
  requireAst:
  # Không có cổng này, `if len(a) in (1, 5, 9, 17):` — bỏ qua hẳn sau/truoc,
  # chép thẳng bốn con số bài đang hỏi — chạy qua đủ run/tests/output vì
  # range(20) trong khung là CỐ ĐỊNH nên bốn con số đó luôn đúng bất kể cách
  # tính. Đếm thật bằng ast trên chính lời giải: `sau` được đọc 2 lần (một
  # lần ở `truoc = sau` cuối vòng lặp, luôn có sẵn trong khung; một lần nữa
  # trong điều kiện) và `truoc` đọc 1 lần (chỉ trong điều kiện) — cách viết
  # khác `sau > truoc` (đúng về mặt toán học trong kịch bản chỉ tăng này)
  # cho ra ĐÚNG hai con số này, nên min không ép theo riêng một cách viết.
  - kind: uses-name, target: sau, min: 2
  - kind: uses-name, target: truoc, min: 1
- tier: tests
  timeoutMs: 5000
- tier: output
  match: exact
  expect: "[1, 5, 9, 17]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai mươi lần `.append`, nhưng chỉ bốn lần thật sự tốn kém. Giờ bạn thấy
được đúng những lần đó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy `.append` — thêm vào CUỐI dãy — thỉnh thoảng tốn kém, nhưng
phần lớn thì rẻ. Nhưng `.append` chỉ làm đúng một việc: đặt phần tử mới
vào ô TRỐNG NGAY SAU ô cuối cùng.

Nếu Byte không muốn thêm vào cuối, mà muốn CHEN một phần tử vào GIỮA dãy —
ví dụ giữa tủ số 2 và tủ số 3 — trong khi cả hai đầu dãy đều đã có chủ,
không còn ô trống nào ở giữa để ghi vào? Chuyện gì phải xảy ra trước khi
phần tử mới có chỗ đứng?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
