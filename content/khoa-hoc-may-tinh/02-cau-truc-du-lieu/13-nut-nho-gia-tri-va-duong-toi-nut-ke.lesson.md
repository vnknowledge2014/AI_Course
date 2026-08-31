---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.nut-nho-gia-tri-va-duong-toi-nut-ke
title: "Nút nhớ giá trị, và nhớ đường tới nút kế"
summary: "Một nút danh sách liên kết là một dict tự trỏ về cùng loại mình: trường 'tiep' không giữ nút kế, nó CHỈ TỚI nút kế — đúng mô hình tham chiếu T3.1 đã dựng."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ds.linked-node]
requires: [mem.name-is-reference, core.dict]
concepts: [ds.linked-node]
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
Sáu bài vừa rồi, mọi ô đều đứng CẠNH nhau trong bộ nhớ. Bài này phá luật đó.
::::

::::explain{#nut-la-mot-dict}
Mảng, ngăn xếp, hàng đợi — cả ba cụm bài vừa qua đều dựa vào đúng một điều
kiện: các ô nằm LIỀN KỀ nhau, để địa chỉ ô thứ i tính thẳng được (bài 2).
Điều kiện đó không phải luật tự nhiên. Nó là một lựa chọn, và bây giờ bạn
gặp cấu trúc bỏ hẳn lựa chọn ấy đi.

Đơn vị mới gọi là **nút** (node) — một ô nhớ một giá trị, viết bằng đúng
công cụ bạn đã có sẵn từ R1: một `dict`.

```python title=readonly
nut = {"gia_tri": "Cát bụi", "tiep": None}
```

Hai trường, hai việc khác nhau:

- `"gia_tri"` giữ nội dung thật — ở đây là tên một bài hát.
- `"tiep"` là trường đặc biệt: nó không giữ nội dung, nó **trỏ sang một nút
  khác cùng hình dạng** — hoặc `None`, nếu chưa nối đi đâu cả.

Nhớ lại T3.1 bài "cái tên không giữ giá trị": một tấm thẻ không buộc vào
cái nồi, nó chỉ ghi SỐ NHÀ của cái nồi. Trường `"tiep"` của một nút chính
là một tấm thẻ như vậy — nó không GIỮ nút kế tiếp, nó **CHỈ TỚI** nút kế
tiếp, bằng đúng cơ chế tham chiếu bạn đã học.

Vì `"tiep"` trỏ sang một nút CÙNG LOẠI — một dict cũng có `"gia_tri"` và
`"tiep"` — một nút có thể nối sang nút khác, nút đó nối sang nút khác nữa.
Không nút nào cần biết địa chỉ vật lý của nút bên cạnh nó trong bộ nhớ máy
tính. Chúng chỉ cần biết SỐ NHÀ của nhau.
::::

::::example{#mot-nut-va-lien-ket}
Byte dựng danh sách phát nhạc cho một buổi họp xóm. Bắt đầu bằng một nút
duy nhất, giữ bài hát đầu tiên.

```python title=readonly
nut_dau = {"gia_tri": "Cát bụi", "tiep": None}
print(nut_dau)
print(type(nut_dau))
```

```text title=readonly
{'gia_tri': 'Cát bụi', 'tiep': None}
<class 'dict'>
```

Không có gì bí ẩn — `nut_dau` chỉ là một `dict` bình thường, thứ bạn đã
dùng từ R1. Giờ Byte dựng thêm một nút và NỐI nút đầu sang nút này, bằng
cách gán trường `"tiep"`:

```python title=readonly
nut_ke = {"gia_tri": "Hạ trắng", "tiep": None}
nut_dau["tiep"] = nut_ke

print(nut_dau["tiep"] is nut_ke)
print(id(nut_dau["tiep"]) == id(nut_ke))
```

```text title=readonly
True
True
```

Dòng `nut_dau["tiep"] = nut_ke` không SAO CHÉP `nut_ke` vào bên trong
`nut_dau`. Nó làm đúng việc mà một phép gán vẫn luôn làm: ghi lại SỐ NHÀ
của `nut_ke` vào trường `"tiep"`. `nut_dau["tiep"] is nut_ke` ra `True`
chứng minh điều đó — hai bên đang chỉ ĐÚNG MỘT nút, không phải hai nút
giống hệt nhau.
::::

::::predict{#doi-ten-qua-tham-chieu commitOnce}
Byte nối hai nút, rồi ĐỔI TÊN bài hát ở nút thứ hai — không đụng gì tới
nút đầu.

**Trước khi bấm chạy**, bạn đoán dòng cuối in ra gì?

```python
na = {"gia_tri": "Cát bụi", "tiep": None}
nb = {"gia_tri": "Hạ trắng", "tiep": None}
na["tiep"] = nb

nb["gia_tri"] = "Hạ trắng (bản mới)"

print(na["tiep"]["gia_tri"])
```

:::opt{correct}
`Hạ trắng (bản mới)`
:::

:::opt
`Hạ trắng`
::why
Gần đúng ở chỗ đó đúng là giá trị `nb["gia_tri"]` mang lúc `na["tiep"]`
được gán — nếu dừng lại đúng ngay dòng đó thì đáp án này đúng.

Chỗ lệch là `na["tiep"]` không HỨNG một bản sao của giá trị tại thời điểm
gán. Nó ghi số nhà của `nb`, và số nhà đó không đổi dù nội dung BÊN TRONG
`nb` đổi sau đó. Đọc `na["tiep"]["gia_tri"]` là đi tới đúng nút `nb` — ở
đúng thời điểm ĐỌC, không phải thời điểm nối dây.
::
:::

:::opt
Máy dừng lại báo lỗi, vì `nb` đã đổi tên nên `na["tiep"]` không còn hợp lệ
::why
Gần đúng ở chỗ bạn cảnh giác đúng chỗ — sửa một thứ đang được trỏ tới nghe
có vẻ nguy hiểm.

Chỗ lệch là gán lại MỘT TRƯỜNG của `nb` (`nb["gia_tri"] = ...`) không hề
làm `nb` biến thành một vật khác. `nb` vẫn là đúng cái dict đó, chỉ có nội
dung một trường bên trong nó đổi. Không có tấm thẻ nào bị "hỏng" cả.
::
:::

:::opt
`None`, vì `na["tiep"]` chưa từng được cập nhật lại sau khi `nb` đổi
::why
Gần đúng ở chỗ câu hỏi đúng là về việc CÓ cần "cập nhật lại" `na["tiep"]`
hay không sau khi `nb` đổi.

Chỗ lệch là không cần cập nhật gì cả — `na["tiep"]` chưa bao giờ ngừng trỏ
tới `nb`. Nó không lưu một bản sao cần đồng bộ lại; nó lưu số nhà, và số
nhà của `nb` không đổi chỉ vì nội dung trong `nb` đổi.
::
:::
::::

::::code{#noi-hai-nut}
Byte gõ sẵn hai nút bài hát, nhưng chưa nối chúng lại. Bạn viết đúng MỘT
dòng để nối `nut_dau` sang `nut_ke`.

```python title=starter
nut_ke = {"gia_tri": "Nối vòng tay lớn", "tiep": None}
nut_dau = {"gia_tri": "Mưa hồng", "tiep": None}

___

print(nut_dau["gia_tri"])
print(nut_dau["tiep"]["gia_tri"])
```

```python title=solution
nut_ke = {"gia_tri": "Nối vòng tay lớn", "tiep": None}
nut_dau = {"gia_tri": "Mưa hồng", "tiep": None}

nut_dau["tiep"] = nut_ke

print(nut_dau["gia_tri"])
print(nut_dau["tiep"]["gia_tri"])
```

```python title=test
assert nut_dau["gia_tri"] == "Mưa hồng", "đừng đổi trường 'gia_tri' của nut_dau — bài này chỉ cần nối dây"
assert nut_ke["gia_tri"] == "Nối vòng tay lớn", "đừng đổi trường 'gia_tri' của nut_ke"
assert nut_dau["tiep"] is nut_ke, "trường 'tiep' của nut_dau phải CHỈ TỚI đúng nut_ke đã có sẵn — không phải một dict mới có nội dung giống hệt nut_ke"
assert nut_ke["tiep"] is None, "nut_ke chưa nối đi đâu cả — đừng đụng vào trường 'tiep' của nó"
```

:::hints
- kind: attention
  body: Bài hỏi đúng một việc — nối `nut_dau` sang `nut_ke`. Nút nào giữ đường đi, và trường nào của nó làm việc đó?
- kind: strategy
  body: 'Nối hai nút nghĩa là ghi số nhà của nút thứ hai vào trường "tiep" của nút thứ nhất: `nut_dau["tiep"] = nut_ke`. Đừng dựng một dict mới có nội dung giống nut_ke — bài kiểm tra đòi đúng CÙNG một nút, không phải một bản sao trông giống nhau.'
- kind: one-line
  body: 'Điền `nut_dau["tiep"] = nut_ke` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Mưa hồng\nNối vòng tay lớn\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai nút, một sợi dây. `tiep` không giữ nút kia — nó chỉ biết đường tới đó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa nối được đúng HAI nút bằng tay. Nhưng một danh sách phát nhạc thật
có hàng chục bài hát, không phải hai.

Nếu có một nút thứ ba nối sau nút thứ hai, một nút thứ tư nối sau nút thứ
ba — làm sao đi từ nút ĐẦU TIÊN tới nút CUỐI CÙNG, mà không cần biết trước
danh sách có bao nhiêu nút?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
