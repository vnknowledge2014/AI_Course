---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.them-vao-dau-danh-sach-lien-ket
title: "Thêm vào ĐẦU danh sách liên kết — không cần dời ai"
summary: "Đối lập trực tiếp với bài 4: chèn vào đầu MẢNG phải dời mọi ô sau; chèn vào đầu DANH SÁCH LIÊN KẾT chỉ đổi một trường tiep, không ai phải dời chỗ."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ds.linked-insert-head]
requires: [ds.singly-linked-list]
concepts: [ds.linked-insert-head]
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
Bài 4: chèn vào đầu mảng, mọi phần tử phải dời chỗ. Danh sách liên kết
không chơi luật đó.
::::

::::explain{#chi-can-doi-mot-tiep}
Nhớ lại bài 4: chèn một phần tử vào ĐẦU một mảng 1000 ô nghĩa là dời cả
1000 ô sang phải một bước, TRƯỚC KHI đặt phần tử mới vào ô 0. Cái giá đó
tới từ đúng một điều kiện: mảng phải LIỀN KỀ, không được có lỗ.

Danh sách liên kết không có điều kiện đó — các nút không cần nằm cạnh
nhau. Vậy thêm một nút MỚI làm đầu danh sách cần đúng hai việc:

1. Dựng nút mới, với trường `"tiep"` trỏ tới đầu CŨ của danh sách.
2. Đổi biến `dau` (đầu danh sách) sang nút MỚI này.

```python title=readonly
def them_vao_dau(dau, gia_tri_moi):
    nut_moi = {"gia_tri": gia_tri_moi, "tiep": dau}
    return nut_moi
```

Không nút nào trong danh sách CŨ bị đụng tới. Không ai bị dời chỗ — vì
"chỗ" của một nút trong danh sách liên kết chưa bao giờ có nghĩa là một vị
trí vật lý cố định, chỉ có nghĩa là "ai đang trỏ tới nó".
::::

::::example{#dia-chi-khong-doi}
Byte đo bằng đúng công cụ đã quen: `id(...)`. Trước khi thêm nút mới vào
đầu, ghi lại số nhà của ba nút đang có. Sau khi thêm, kiểm lại — có nút
nào bị dựng lại, dời đi không?

```python title=readonly
def them_vao_dau(dau, gia_tri_moi):
    nut_moi = {"gia_tri": gia_tri_moi, "tiep": dau}
    return nut_moi

nut_c = {"gia_tri": "Diễm xưa", "tiep": None}
nut_b = {"gia_tri": "Hạ trắng", "tiep": nut_c}
nut_a = {"gia_tri": "Cát bụi", "tiep": nut_b}
dau = nut_a

id_c_truoc, id_b_truoc, id_a_truoc = id(nut_c), id(nut_b), id(nut_a)

dau = them_vao_dau(dau, "Sương trắng")

print(id(nut_c) == id_c_truoc)
print(id(nut_b) == id_b_truoc)
print(id(nut_a) == id_a_truoc)
print(dau["gia_tri"])
print(dau["tiep"] is nut_a)
```

```text title=readonly
True
True
True
Sương trắng
True
```

Ba nút cũ giữ nguyên số nhà — không ai bị dựng lại. Chỉ có MỘT thứ mới
xuất hiện: nút `"Sương trắng"`, với `"tiep"` trỏ thẳng vào `nut_a` cũ. So
với bài 4 (dời cả mảng), đây là cái giá gần như bằng không, bất kể danh
sách đang dài bao nhiêu.
::::

::::predict{#hai-lan-them-dau commitOnce}
Byte thêm vào đầu HAI LẦN liên tiếp, vào đúng danh sách ba bài ở ví dụ
trên (`dau` ban đầu là `nut_a`, tức "Cát bụi").

**Trước khi chạy**, bạn đoán `dau["tiep"]["gia_tri"]` — bài đứng ở vị trí
THỨ HAI trong danh sách CUỐI CÙNG — là bài nào?

```python
dau = nut_a
dau = them_vao_dau(dau, "Sương trắng")
dau = them_vao_dau(dau, "Bèo dạt mây trôi")
```

:::opt{correct}
`"Sương trắng"`
:::

:::opt
`"Bèo dạt mây trôi"`
::why
Gần đúng ở chỗ đó đúng là bài MỚI NHẤT, và `dau["gia_tri"]` (không phải
`dau["tiep"]["gia_tri"]`) đúng là bài đó — bạn nhớ đúng ai đang là đầu.

Chỗ lệch là câu hỏi hỏi về `dau["tiep"]`, tức là bước MỘT LẦN từ đầu vào
trong. Sau hai lần thêm đầu, đầu mới nhất trỏ tới đầu NGAY TRƯỚC nó — mà
đầu ngay trước lần thêm cuối chính là "Sương trắng", không phải chính nó.
::
:::

:::opt
`"Cát bụi"`
::why
Gần đúng ở chỗ "Cát bụi" đúng là có thật trong danh sách, và nó đúng là
đầu BAN ĐẦU trước khi có lần thêm nào.

Chỗ lệch là bạn đi QUÁ xa — hai lần thêm vào đầu chỉ đẩy "Cát bụi" lùi
xuống vị trí thứ BA, không phải thứ hai. Vị trí thứ hai là bài được thêm
ở LẦN THÊM ĐẦU TIÊN trong hai lần, tức "Sương trắng".
::
:::

:::opt
Không đoán được, vì thứ tự thêm vào đầu không quyết định thứ tự cuối cùng
::why
Gần đúng ở chỗ bạn đúng khi nghi ngờ — thứ tự KHÔNG phải lúc nào cũng dễ
đoán khi có nhiều thao tác chồng lên nhau.

Chỗ lệch là ở ĐÚNG bài này, thứ tự thêm hoàn toàn quyết định được kết
quả: mỗi lần thêm vào đầu, người mới nhất đứng NGOÀI CÙNG, người cũ hơn
lùi vào trong đúng một bậc — y hệt xếp một chồng đĩa, đĩa mới luôn nằm
trên đỉnh.
::
:::
::::

::::code{#viet-ham-them-dau}
Viết hàm `them_vao_dau(dau, gia_tri_moi)`: dựng một nút mới giữ
`gia_tri_moi`, nối nó trỏ tới `dau` hiện tại, rồi trả về nút mới đó (đầu
MỚI của danh sách).

```python title=starter
def them_vao_dau(dau, gia_tri_moi):
    ___
    return nut_moi

nut3 = {"gia_tri": "Diễm xưa", "tiep": None}
nut2 = {"gia_tri": "Hạ trắng", "tiep": nut3}
nut1 = {"gia_tri": "Cát bụi", "tiep": nut2}

dau = nut1
dau = them_vao_dau(dau, "Sương trắng")
dau = them_vao_dau(dau, "Bèo dạt mây trôi")

print(dau["gia_tri"])
print(dau["tiep"]["gia_tri"])
print(dau["tiep"]["tiep"]["gia_tri"])
```

```python title=solution
def them_vao_dau(dau, gia_tri_moi):
    nut_moi = {"gia_tri": gia_tri_moi, "tiep": dau}
    return nut_moi

nut3 = {"gia_tri": "Diễm xưa", "tiep": None}
nut2 = {"gia_tri": "Hạ trắng", "tiep": nut3}
nut1 = {"gia_tri": "Cát bụi", "tiep": nut2}

dau = nut1
dau = them_vao_dau(dau, "Sương trắng")
dau = them_vao_dau(dau, "Bèo dạt mây trôi")

print(dau["gia_tri"])
print(dau["tiep"]["gia_tri"])
print(dau["tiep"]["tiep"]["gia_tri"])
```

```python title=test
assert dau["gia_tri"] == "Bèo dạt mây trôi", "sau hai lần thêm, dau phải là bài thêm SAU CÙNG"
assert dau["tiep"]["gia_tri"] == "Sương trắng", "vị trí thứ hai phải là bài thêm ở lần đầu tiên trong hai lần"
assert dau["tiep"]["tiep"] is nut1, "nút thứ ba trong dây phải là ĐÚNG nut1 cũ — không phải một nút mới có cùng nội dung"
assert nut1["gia_tri"] == "Cát bụi", "đừng sửa nut1 — thêm vào đầu không được đụng tới nút cũ"
assert nut1["tiep"] is nut2, "đừng sửa trường 'tiep' của nut1"
```

:::hints
- kind: attention
  body: Hàm cần TẠO một nút hoàn toàn mới, không phải sửa nút đang có sẵn. Trường "tiep" của nút mới đó phải trỏ đi đâu để nối nó lên trước danh sách cũ?
- kind: strategy
  body: 'Dựng một dict mới với gia_tri là gia_tri_moi, và tiep trỏ thẳng vào chính dau đang có (đầu CŨ). Đừng đổi bất cứ trường nào của dau — nút cũ phải còn nguyên, chỉ có một nút mới xuất hiện, đứng trước nó.'
- kind: one-line
  body: 'Điền `nut_moi = {"gia_tri": gia_tri_moi, "tiep": dau}` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Bèo dạt mây trôi\nSương trắng\nCát bụi\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai lần thêm vào đầu, không nút cũ nào bị đụng tới. Cái giá gần như bằng
không.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Thêm vào ĐẦU thì rẻ — bạn vừa thấy, và đo được bằng `id(...)`. Nhưng nếu
Byte muốn chèn một bài hát vào GIỮA danh sách — không phải đầu, không
phải cuối, mà ngay sau một bài cụ thể nào đó — hàm `them_vao_dau` còn
dùng được không?

Bài sau trả lời, và cái giá lần này không còn gần như bằng không nữa.
::::

::::checkpoint{mastery=0.8}
::::
