---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.danh-sach-lien-ket-hai-chieu
title: "Danh sách liên kết hai chiều — đi lùi được"
summary: "Thêm một trường truoc bên cạnh tiep. Trả giá bằng một tham chiếu nữa mỗi nút, đổi lấy khả năng đi NGƯỢC mà danh sách một chiều không có."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ds.doubly-linked-list]
requires: [ds.linked-insert-middle]
concepts: [ds.doubly-linked-list]
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
Một trường nữa cho mỗi nút, và danh sách của bạn học được cách đi lùi.
::::

::::explain{#them-truong-truoc}
Bài trước để lại đúng một câu hỏi: muốn biết nút đứng NGAY TRƯỚC một nút
cho trước, danh sách một chiều bắt bạn đi lại từ đầu và đếm — vì `"tiep"`
chỉ trỏ được một hướng.

Cách chữa không phức tạp: thêm một trường thứ hai vào MỖI nút, gọi là
`"truoc"`, trỏ ngược lại nút đứng trước nó.

```python title=readonly
nut = {"gia_tri": ..., "tiep": None, "truoc": None}
```

Đây gọi là **danh sách liên kết hai chiều** (doubly linked list). Mỗi nút
giờ mang HAI sợi dây thay vì một: `"tiep"` đi tới, `"truoc"` đi lui. Cái
giá là mỗi nút tốn thêm một tham chiếu — với mọi nút, không chỉ nút đặc
biệt nào. Cái được là đi ngược từ bất kỳ nút nào cũng rẻ y hệt đi xuôi,
không cần đi lại từ đầu.

Chèn một nút mới vào giữa giờ có thêm một việc: không chỉ nối nút MỚI vào
đúng cả hai hướng, mà nút đứng NGAY SAU chỗ chèn cũng phải được cập nhật
lại trường `"truoc"` của chính nó — nếu không, đi ngược từ phía sau sẽ đi
lạc, bỏ qua đúng nút vừa chèn.

```python title=readonly
def them_sau(nut_truoc, gia_tri_moi):
    nut_moi = {"gia_tri": gia_tri_moi, "tiep": nut_truoc["tiep"], "truoc": nut_truoc}
    if nut_truoc["tiep"] is not None:
        nut_truoc["tiep"]["truoc"] = nut_moi
    nut_truoc["tiep"] = nut_moi
    return nut_moi
```

Dòng kiểm `if nut_truoc["tiep"] is not None` xử lý đúng một trường hợp
đặc biệt: chèn vào SAU nút đang là cuối dây. Lúc đó chưa có nút nào đứng
sau để cập nhật `"truoc"` cả.
::::

::::example{#di-xuoi-va-di-nguoc}
Byte dựng ba nút bằng `them_sau`, rồi duyệt CẢ HAI HƯỚNG — xuôi từ đầu,
và ngược từ cuối — bằng đúng hai trường khác nhau.

```python title=readonly
d1 = {"gia_tri": "Cát bụi", "tiep": None, "truoc": None}
d2 = them_sau(d1, "Hạ trắng")
d3 = them_sau(d2, "Diễm xưa")

xuoi = []
h = d1
while h is not None:
    xuoi.append(h["gia_tri"])
    h = h["tiep"]
print(xuoi)

nguoc = []
h = d3
while h is not None:
    nguoc.append(h["gia_tri"])
    h = h["truoc"]
print(nguoc)
```

```text title=readonly
['Cát bụi', 'Hạ trắng', 'Diễm xưa']
['Diễm xưa', 'Hạ trắng', 'Cát bụi']
```

Cùng một dây, đi được cả hai chiều — chỉ cần đổi trường đang đọc từ
`"tiep"` sang `"truoc"`, và đổi điểm xuất phát từ đầu sang cuối.
::::

::::predict{#chen-giua-roi-di-nguoc commitOnce}
Byte có đúng ba nút như ví dụ trên (`d1` "Cát bụi", `d2` "Hạ trắng", `d3`
"Diễm xưa"), rồi gọi thêm một lần `them_sau` để chèn "X" vào ngay SAU
`d2` — tức là giữa "Hạ trắng" và "Diễm xưa":

```python
them_sau(d2, "X")
```

**Trước khi chạy**, bạn đoán: duyệt NGƯỢC bắt đầu từ `d3` (dùng trường
`"truoc"`), danh sách bài hát in ra theo thứ tự nào?

:::opt{correct}
`['Diễm xưa', 'X', 'Hạ trắng', 'Cát bụi']`
:::

:::opt
`['Diễm xưa', 'Hạ trắng', 'Cát bụi']` — "X" không xuất hiện, vì nó được
chèn SAU khi duyệt ngược đã bắt đầu
::why
Gần đúng ở chỗ nếu "X" được chèn SAU KHI vòng lặp duyệt đã chạy xong thì
đúng là nó sẽ không kịp xuất hiện — thứ tự thời gian ở đây có ý nghĩa
thật.

Chỗ lệch là `them_sau(d2, "X")` chạy TRƯỚC dòng duyệt ngược, không phải
sau. Tính tới lúc bắt đầu duyệt, "X" đã là một nút thật trong dây, với cả
`"tiep"` lẫn `"truoc"` đã được nối đúng chỗ.
::
:::

:::opt
`['Diễm xưa', 'Hạ trắng', 'X', 'Cát bụi']` — "X" đứng giữa "Hạ trắng" và
"Cát bụi" khi duyệt ngược
::why
Gần đúng ở chỗ "X" đúng là CÓ xuất hiện trong kết quả — bạn không quên
mất nó.

Chỗ lệch là vị trí. `them_sau(d2, "X")` chèn "X" vào NGAY SAU `d2`, tức
giữa "Hạ trắng" và "Diễm xưa" khi đọc XUÔI. Khi đọc NGƯỢC (bắt đầu từ nút
đứng sau cùng), thứ tự chỉ đảo lại — "X" vẫn đứng sát cạnh "Hạ trắng",
nhưng ở phía "Diễm xưa", không phải phía "Cát bụi".
::
:::

:::opt
Máy dừng lại báo lỗi, vì duyệt ngược một danh sách vừa mới chèn thêm là
không an toàn
::why
Gần đúng ở chỗ nghe có vẻ hợp lý rằng vừa sửa cấu trúc xong thì nên "cẩn
thận" khi đọc lại nó ngay sau đó.

Chỗ lệch là không có luật nào như vậy trong Python hay trong danh sách
liên kết. Miễn `them_sau` cập nhật ĐÚNG cả hai trường `"tiep"` và
`"truoc"` của mọi nút liên quan — đúng như hàm ở phần giải thích đã làm —
duyệt ngược ngay sau đó hoàn toàn an toàn và cho kết quả đúng.
::
:::
::::

::::code{#viet-ham-chen-hai-chieu}
Hàm `them_sau` đã có sẵn khung, nhưng thiếu đúng phần cập nhật trường
`"truoc"` của nút đứng NGAY SAU chỗ chèn — phần khiến duyệt NGƯỢC nhìn
thấy được nút mới.

```python title=starter
def them_sau(nut_truoc, gia_tri_moi):
    nut_moi = {"gia_tri": gia_tri_moi, "tiep": nut_truoc["tiep"], "truoc": nut_truoc}
    ___
    nut_truoc["tiep"] = nut_moi
    return nut_moi

d1 = {"gia_tri": "Cát bụi", "tiep": None, "truoc": None}
d2 = them_sau(d1, "Hạ trắng")
d3 = them_sau(d2, "Diễm xưa")

xuoi = []
h = d1
while h is not None:
    xuoi.append(h["gia_tri"])
    h = h["tiep"]
print(xuoi)

nguoc = []
h = d3
while h is not None:
    nguoc.append(h["gia_tri"])
    h = h["truoc"]
print(nguoc)
```

```python title=solution
def them_sau(nut_truoc, gia_tri_moi):
    nut_moi = {"gia_tri": gia_tri_moi, "tiep": nut_truoc["tiep"], "truoc": nut_truoc}
    if nut_truoc["tiep"] is not None:
        nut_truoc["tiep"]["truoc"] = nut_moi
    nut_truoc["tiep"] = nut_moi
    return nut_moi

d1 = {"gia_tri": "Cát bụi", "tiep": None, "truoc": None}
d2 = them_sau(d1, "Hạ trắng")
d3 = them_sau(d2, "Diễm xưa")

xuoi = []
h = d1
while h is not None:
    xuoi.append(h["gia_tri"])
    h = h["tiep"]
print(xuoi)

nguoc = []
h = d3
while h is not None:
    nguoc.append(h["gia_tri"])
    h = h["truoc"]
print(nguoc)
```

```python title=test
assert xuoi == ["Cát bụi", "Hạ trắng", "Diễm xưa"], xuoi
assert nguoc == ["Diễm xưa", "Hạ trắng", "Cát bụi"], nguoc
assert d2["truoc"] is d1, "trường truoc của d2 phải là d1"
assert d3["truoc"] is d2, "trường truoc của d3 phải là d2"
assert d1["truoc"] is None, "d1 là đầu dây, truoc của nó phải là None"

# chèn ở GIỮA (không phải cuối) — chỗ luật chấm chỉ nhìn xuoi/nguoc phía
# trên không phân biệt nổi: cả "quên cập nhật truoc" lẫn lời giải đúng
# đều cho ra đúng hai list xuoi/nguoc như nhau ở phần trên, vì phần trên
# chỉ chèn nối tiếp vào CUỐI dây (nút đứng sau luôn là None). Chỉ chèn
# xen vào giữa hai nút đã có mới lộ ra chỗ khác biệt.
e1 = {"gia_tri": "A", "tiep": None, "truoc": None}
e2 = them_sau(e1, "B")
emid = them_sau(e1, "X")
assert e2["truoc"] is emid, "sau khi chèn X vào giữa A và B, truoc của B phải là X — không còn là A nữa"
assert e1["tiep"] is emid, "sau khi chèn X vào giữa A và B, tiep của A phải là X — không còn là B nữa"
assert emid["tiep"] is e2, "tiep của X (nút vừa chèn) phải là B — X đứng đúng giữa A và B, không phải cuối dây"
```

:::hints
- kind: attention
  body: Chỗ trống không lo phần nối "tiep" của nut_truoc — dòng đó đã có sẵn NGAY SAU chỗ trống. Việc còn thiếu là cập nhật trường "truoc" của nút đang đứng NGAY SAU chỗ chèn, để nó không còn trỏ ngược về nut_truoc nữa mà trỏ về nút mới.
- kind: strategy
  body: 'Nút đứng sau chỗ chèn, TRƯỚC KHI bị nối lại, chính là nut_truoc["tiep"] — dòng ngay trên chỗ trống đã lưu giá trị đó vào nut_moi["tiep"] rồi, nhưng bản thân nut_truoc["tiep"] chưa đổi. Nếu nút đó tồn tại (khác None — chèn ở cuối dây thì không có nút nào cả), gán "truoc" của nó thành nut_moi.'
- kind: one-line
  body: 'Điền `if nut_truoc["tiep"] is not None: nut_truoc["tiep"]["truoc"] = nut_moi` vào chỗ trống (viết trên hai dòng, thụt lề đúng).'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^\['Cát bụi', 'Hạ trắng', 'Diễm xưa'\]\n\['Diễm xưa', 'Hạ trắng', 'Cát bụi'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Xuôi bằng "tiep", ngược bằng "truoc" — cùng một dây, hai hướng đi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ bạn đi được cả hai hướng trong một danh sách liên kết. Nhưng TÌM một
bài hát cụ thể — biết tên, không biết nó nằm ở đâu trong dây — vẫn chỉ có
một cách: đi từng nút một, xuôi hoặc ngược, không khác gì nhau về công
sức.

Mảng tính THẲNG được địa chỉ của ô thứ i, không cần dò (bài 2). Có cách
nào đổi một cái TÊN — như tên bài hát — thành một CON SỐ, để tính thẳng
được vị trí của nó, mà không cần các ô nằm liền kề nhau như mảng đòi hỏi?

Bài sau bắt đầu trả lời.
::::

::::checkpoint{mastery=0.8}
::::
