---
id: toan.tap-hop-quan-he-anh-xa.bang-cua-mot-quan-he
title: Bảng của một quan hệ
summary: "Biểu diễn quan hệ bằng bảng 0/1 — hàng là luống, cột là ngày, ô đánh dấu nếu cặp đó thuộc quan hệ. MỘT quan hệ, HAI cách nhìn (đống cặp / bảng), đúng khớp bài 4 (mô tả tập hợp bằng hai cách)."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.relation-table]
requires: [math.relation, core.dict]
concepts: [math.bang-0-1, math.hai-cach-nhin]
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
Quan hệ tưới nước viết ra là một ĐỐNG cặp rời rạc. Nhìn đống đó có DỄ
thấy "luống nào tưới NHIỀU ngày nhất" không, hay cần cách trình bày
khác?
::::

::::explain{#bang-quan-he}
Cần. **Biểu diễn quan hệ bằng bảng 0/1** — hàng LÀ luống, cột LÀ
ngày, ô ĐÁNH DẤU (`True`) nếu cặp ĐÓ thuộc quan hệ:

```python title=readonly
luong = {"luong_1", "luong_2"}
ngay = {"Hai", "Ba"}
tuoi_nuoc = {("luong_1", "Hai"), ("luong_2", "Ba"), ("luong_2", "Hai")}

bang = {}
for x in luong:
    bang[x] = {}
    for y in ngay:
        bang[x][y] = (x, y) in tuoi_nuoc

print(bang["luong_1"]["Hai"])
print(bang["luong_1"]["Ba"])
```

```text title=readonly
True
False
```

`bang["luong_1"]["Hai"]` LÀ `True` — cặp `("luong_1", "Hai")` CÓ
trong `tuoi_nuoc`. `bang["luong_1"]["Ba"]` LÀ `False` — cặp
`("luong_1", "Ba")` KHÔNG có. `bang` VÀ `tuoi_nuoc` mang CÙNG THÔNG
TIN, chỉ trình bày KHÁC nhau.
::::

::::example{#doc-theo-hang}
Đọc theo HÀNG (một luống, MỌI ngày) trả lời câu hỏi "luống NÀY tưới
MẤY ngày" — điều đống cặp KHÔNG lộ RA ngay:

```python title=readonly
luong = {"luong_1", "luong_2"}
ngay = {"Hai", "Ba"}
tuoi_nuoc = {("luong_1", "Hai"), ("luong_2", "Ba"), ("luong_2", "Hai")}

bang = {}
for x in luong:
    bang[x] = {}
    for y in ngay:
        bang[x][y] = (x, y) in tuoi_nuoc

so_ngay_tuoi = sum(1 for y in ngay if bang["luong_2"][y])
print(so_ngay_tuoi)
```

```text title=readonly
2
```

Đọc HÀNG `"luong_2"`: hai ô `True` (`Hai` VÀ `Ba`) — luống 2 tưới
CẢ hai ngày. Nhìn đống cặp rời rạc, câu ĐÓ phải LỌC bằng mắt; nhìn
BẢNG, chỉ cần đọc MỘT hàng.
::::

::::predict{#doan-doc-theo-cot commitOnce}
Byte đổi hướng — đọc theo CỘT (một ngày, MỌI luống):

```python
luong = {"luong_1", "luong_2"}
ngay = {"Hai", "Ba"}
tuoi_nuoc = {("luong_1", "Hai"), ("luong_2", "Ba"), ("luong_2", "Hai")}

bang = {}
for x in luong:
    bang[x] = {}
    for y in ngay:
        bang[x][y] = (x, y) in tuoi_nuoc

so_luong_tuoi_hai = sum(1 for x in luong if bang[x]["Hai"])
print(so_luong_tuoi_hai)
```

Dòng cuối in ra gì?

:::opt{correct}
`2`
:::

:::opt
`1` — vì hàm `sum(1 for x in luong if bang[x]["Hai"])` chỉ ĐẾM đúng
MỘT lần cho TOÀN BỘ cột `"Hai"`, KHÔNG PHẢI đếm RIÊNG cho từng luống
— nó trả về "cột NÀY CÓ được đánh dấu HAY không" (0 hoặc 1), không
phải "bao nhiêu luống"
::why
Gần đúng ở việc bạn nghĩ TỚI CÁCH biểu thức generator TRẢ VỀ MỘT giá
trị TỔNG QUÁT cho CẢ cột — MỘT trực giác dễ HIỂU LẦM khi mới gặp
`sum(... for ... if ...)`.

Chỗ lệch: `sum(1 for x in luong if bang[x]["Hai"])` duyệt QUA **TỪNG**
luống trong `luong`, VÀ với MỖI luống thoả điều KIỆN (`bang[x]["Hai"]`
LÀ `True`), nó CỘNG THÊM đúng `1` — Y HỆT cách `so_ngay_tuoi` (ví dụ
trên) đếm theo HÀNG, chỉ đổi trục thành CỘT. Cả `"luong_1"` VÀ
`"luong_2"` đều CÓ `bang[x]["Hai"]` LÀ `True` (`tuoi_nuoc` chứa CẢ
`("luong_1", "Hai")` LẪN `("luong_2", "Hai")`) — TỔNG LÀ `2`.
::
:::

:::opt
Máy báo lỗi biên dịch — biểu thức `sum(1 for x in luong if
bang[x]["Hai"])` dùng CHỮ số `1` LÀM giá trị sinh RA (thay vì một
biến), Python CẤM generator sinh RA một HẰNG SỐ cố định
::why
Gần đúng ở việc bạn để ý phần "sinh ra" của generator LÀ `1` (một
HẰNG số), khác với các generator TRƯỚC đó (VD `x in b for x in a` Ở
bài 6) sinh RA một biến — một quan sát đúng về HÌNH THỨC.

Chỗ lệch: Python HOÀN TOÀN cho phép generator sinh RA BẤT KỲ biểu
thức nào, kể CẢ một hằng số cố định như `1` — đây LÀ mẫu CHUẨN để
"đếm số lượt thoả điều kiện" (`sum(1 for ... if ...)`), y hệt cách
`so_ngay_tuoi` Ở ví dụ TRƯỚC đã dùng. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_bang_quan_he}
Viết `bang_quan_he(quan_he, a, b)` — trả về BẢNG (`dict` lồng `dict`)
biểu diễn `quan_he`: `bang[x][y]` LÀ `True` NẾU `(x, y) ∈ quan_he`.

```python title=starter
def bang_quan_he(quan_he, a, b):
    bang = {}
    for x in a:
        bang[x] = {}
        for y in b:
            bang[x][y] = ___
    return bang


luong = {"luong_1", "luong_2"}
ngay = {"Hai", "Ba"}
tuoi_nuoc = {("luong_1", "Hai"), ("luong_2", "Ba"), ("luong_2", "Hai")}

print(bang_quan_he(tuoi_nuoc, luong, ngay)["luong_1"]["Hai"])
```

```python title=solution
def bang_quan_he(quan_he, a, b):
    bang = {}
    for x in a:
        bang[x] = {}
        for y in b:
            bang[x][y] = (x, y) in quan_he
    return bang


luong = {"luong_1", "luong_2"}
ngay = {"Hai", "Ba"}
tuoi_nuoc = {("luong_1", "Hai"), ("luong_2", "Ba"), ("luong_2", "Hai")}

print(bang_quan_he(tuoi_nuoc, luong, ngay)["luong_1"]["Hai"])
```

```python title=test
bang = bang_quan_he(tuoi_nuoc, luong, ngay)
assert bang["luong_1"]["Ba"] is False, "cap nay khong trong quan he"
assert bang["luong_2"]["Hai"] is True, "cap nay co trong quan he"
assert bang["luong_2"]["Ba"] is True, "cap nay co trong quan he"
assert bang_quan_he(set(), luong, ngay)["luong_1"]["Hai"] is False, "quan he rong -- moi o deu False"
```

:::hints
- kind: attention
  body: "Moi o bang[x][y] phai la True/False tuy theo cap (x, y) co nam trong quan_he hay khong -- dung `in`."
- kind: strategy
  body: "(x, y) in quan_he"
- kind: one-line
  body: "___ = (x, y) in quan_he"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: moi o bang[x][y] phai kiem (x, y) co trong quan_he khong bang toan tu in -- gia tri co dinh True/False se sai cho moi truong hop
  requireAst:
  - kind: uses-operator, target: in, min: 1
  - kind: uses-name, target: x, min: 1
  - kind: uses-name, target: y, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một quan hệ, hai cách nhìn — đống cặp và bảng mang cùng thông tin.
Bài sau: hàng và cột của bảng gọi tên đúng những tập hợp nào?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đọc theo HÀNG của luống 1: hai ô đánh dấu. Đọc theo CỘT của thứ Hai:
mấy ô — cột đó nói lên điều gì mà hàng không nói được?
::::

::::checkpoint{mastery=0.8}
::::
