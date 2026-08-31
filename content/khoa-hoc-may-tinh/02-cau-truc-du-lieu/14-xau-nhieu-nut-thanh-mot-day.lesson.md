---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.xau-nhieu-nut-thanh-mot-day
title: "Xâu nhiều nút thành một dãy"
summary: "Nối tiep của nút này sang nút kia dựng ra một chuỗi các ô không cần nằm cạnh nhau trong bộ nhớ — đi từ đầu tới cuối là đi theo dây, không phải nhảy theo địa chỉ."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ds.singly-linked-list]
requires: [ds.linked-node]
concepts: [ds.singly-linked-list]
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
Hai nút thì dễ. Bây giờ xâu cả một danh sách phát nhạc bằng cách đó.
::::

::::explain{#duyet-bang-vong-lap}
Bài trước nối được đúng hai nút. Nối thêm một nút thứ ba sau nút thứ hai,
một nút thứ tư sau nút thứ ba — cứ thế — và bạn có một **danh sách liên
kết một chiều** (singly linked list): một chuỗi nút, mỗi nút chỉ biết
đường tới ĐÚNG một nút kế tiếp.

Nút đầu tiên của chuỗi có một cái tên riêng: **đầu** (head) — thường được
giữ trong một biến gọi là `dau`. Từ `dau`, bạn tới được MỌI nút còn lại,
bằng cách đi theo dây `"tiep"` hết nút này tới nút khác — không có cách
nào khác. Không có chỉ số để nhảy thẳng như mảng (bài 2); chỉ có dây để đi
theo.

Cách đi theo dây, viết bằng vòng lặp `while`:

```python title=readonly
hien = dau        # hien: nút đang đứng, bắt đầu từ đầu dây
while hien is not None:
    ...            # làm gì đó với hien["gia_tri"]
    hien = hien["tiep"]   # bước sang nút kế tiếp
```

Vòng lặp dừng đúng lúc `hien` là `None` — nghĩa là vừa đi qua nút CUỐI
CÙNG của dây, nút có `"tiep"` bằng `None` vì nó chưa nối đi đâu nữa.
::::

::::example{#duyet-mot-danh-sach-phat}
Byte dựng một danh sách phát bốn bài, nối từ nút cuối ngược lên nút đầu —
việc dựng dây không quan tâm bạn viết dòng nào trước, chỉ quan tâm trường
`"tiep"` trỏ đi đâu.

```python title=readonly
nut4 = {"gia_tri": "Mùa thu cho em", "tiep": None}
nut3 = {"gia_tri": "Diễm xưa", "tiep": nut4}
nut2 = {"gia_tri": "Hạ trắng", "tiep": nut3}
nut1 = {"gia_tri": "Cát bụi", "tiep": nut2}

dau = nut1
hien = dau
ten_bai = []
while hien is not None:
    ten_bai.append(hien["gia_tri"])
    hien = hien["tiep"]

print(ten_bai)
```

```text title=readonly
['Cát bụi', 'Hạ trắng', 'Diễm xưa', 'Mùa thu cho em']
```

Bốn dòng gán đầu tiên KHÔNG chạy theo đúng thứ tự sẽ duyệt — `nut4` được
gõ ra TRƯỚC `nut1`. Nhưng thứ tự duyệt không phụ thuộc bạn gõ dòng nào
trước; nó phụ thuộc hoàn toàn vào trường `"tiep"` của từng nút đang trỏ đi
đâu.
::::

::::predict{#nut-nao-la-cuoi-day commitOnce}
Byte gõ lại đúng bốn dòng dựng nút ở ví dụ trên (không có dòng `dau = ...`
lần này) — chỉ ba nút, để dễ nhìn:

```python
c = {"gia_tri": "Diễm xưa", "tiep": None}
b = {"gia_tri": "Hạ trắng", "tiep": c}
a = {"gia_tri": "Cát bụi", "tiep": b}
```

**Không chạy code**, chỉ đọc ba dòng trên: nút nào đang có trường `"tiep"`
bằng `None` — tức là nút đứng CUỐI của dây?

:::opt{correct}
`c` — nút được gõ ĐẦU TIÊN trong mã, nhưng lại đứng CUỐI dây
:::

:::opt
`a` — vì đó là biến được gõ CUỐI CÙNG trong đoạn mã
::why
Gần đúng ở chỗ `a` đúng là dòng cuối cùng Byte gõ — bạn đọc mã không sai.

Chỗ lệch là "dòng cuối cùng trong mã" và "nút cuối cùng của dây" là hai
thứ khác nhau hoàn toàn. `a["tiep"]` là `b`, không phải `None` — vậy `a`
đứng ĐẦU dây (nó là nút không ai trỏ tới nó), không phải cuối.
::
:::

:::opt
`b` — vì nó đứng giữa hai nút kia trong đoạn mã
::why
Gần đúng ở chỗ `b` đúng là dòng ở giữa khi đọc mã từ trên xuống.

Chỗ lệch là vị trí GIỮA trong đoạn mã không nói lên gì về vị trí trong
dây. `b["tiep"]` là `c`, không phải `None` — `b` vẫn còn nối đi tiếp,
chưa phải nút cuối.
::
:::

:::opt
Cả ba nút — vì không dòng nào gán lại `"tiep"` sau khi tạo
::why
Gần đúng ở chỗ đúng là không có dòng nào GÁN LẠI trường `"tiep"` sau khi
mỗi nút được tạo ra.

Chỗ lệch là mỗi nút chỉ cần MỘT lần gán `"tiep"` lúc dựng — không cần gán
lại. `a["tiep"]` và `b["tiep"]` đều đã được gán một giá trị khác `None`
ngay từ dòng đầu tiên tạo ra chúng.
::
:::
::::

::::code{#lay-danh-sach-bai-hat}
Viết hàm `lay_danh_sach_bai_hat(dau)`: nhận nút đầu của một danh sách phát,
trả về một `list` tên các bài hát, theo đúng thứ tự đi từ `dau` tới hết
dây.

```python title=starter
def lay_danh_sach_bai_hat(dau):
    ten_bai = []
    hien = dau
    while hien is not None:
        ___
        hien = hien["tiep"]
    return ten_bai

nut4 = {"gia_tri": "Mùa thu cho em", "tiep": None}
nut3 = {"gia_tri": "Diễm xưa", "tiep": nut4}
nut2 = {"gia_tri": "Hạ trắng", "tiep": nut3}
nut1 = {"gia_tri": "Cát bụi", "tiep": nut2}

print(lay_danh_sach_bai_hat(nut1))
print(lay_danh_sach_bai_hat(nut3))
```

```python title=solution
def lay_danh_sach_bai_hat(dau):
    ten_bai = []
    hien = dau
    while hien is not None:
        ten_bai.append(hien["gia_tri"])
        hien = hien["tiep"]
    return ten_bai

nut4 = {"gia_tri": "Mùa thu cho em", "tiep": None}
nut3 = {"gia_tri": "Diễm xưa", "tiep": nut4}
nut2 = {"gia_tri": "Hạ trắng", "tiep": nut3}
nut1 = {"gia_tri": "Cát bụi", "tiep": nut2}

print(lay_danh_sach_bai_hat(nut1))
print(lay_danh_sach_bai_hat(nut3))
```

```python title=test
assert lay_danh_sach_bai_hat(nut1) == ["Cát bụi", "Hạ trắng", "Diễm xưa", "Mùa thu cho em"], "phải duyệt đủ và đúng thứ tự từ nut1"
assert lay_danh_sach_bai_hat(nut3) == ["Diễm xưa", "Mùa thu cho em"], "gọi hàm từ nut3 chỉ được lấy phần dây TỪ nut3 trở đi, không phải cả dây"
assert lay_danh_sach_bai_hat(nut4) == ["Mùa thu cho em"], "gọi hàm từ nut4 (nút cuối) phải trả về danh sách một phần tử"
```

:::hints
- kind: attention
  body: Vòng lặp đã có sẵn — nó dừng đúng lúc `hien` là `None`. Việc còn thiếu là làm gì với `hien` ở MỖI bước, trước khi nó nhảy sang nút kế.
- kind: strategy
  body: Bạn cần GÓP tên bài hát của nút đang đứng (`hien["gia_tri"]`) vào danh sách `ten_bai`. Dùng đúng `list.append` — đừng dùng `dau["gia_tri"]` (dau không đổi suốt vòng lặp, chỉ hien mới đổi), và đừng gán lại cả `ten_bai`.
- kind: one-line
  body: 'Điền `ten_bai.append(hien["gia_tri"])` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^\['Cát bụi', 'Hạ trắng', 'Diễm xưa', 'Mùa thu cho em'\]\n\['Diễm xưa', 'Mùa thu cho em'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Từ một nút, bạn đi hết cả dây — không cần biết trước có bao nhiêu nút.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Danh sách phát của Byte đang có bốn bài, xếp sẵn từ lúc dựng dây. Nếu có
một bài MỚI muốn nghe đầu tiên — đứng trước cả `nut1` — bạn sẽ chèn nó vào
đâu, và cần đổi bao nhiêu nút đang có để làm việc đó?

So với bài 4 (chèn vào đầu MẢNG, mọi phần tử phải dời chỗ), bạn đoán câu
trả lời ở đây sẽ rẻ hơn hay đắt hơn?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
