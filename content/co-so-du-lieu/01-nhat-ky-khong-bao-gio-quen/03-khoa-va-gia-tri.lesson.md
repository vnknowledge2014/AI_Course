---
id: co-so-du-lieu.nhat-ky-khong-bao-gio-quen.khoa-va-gia-tri
title: Khoá và giá trị
summary: "Ghép HAI bản ghi biến độ dài liền nhau — [len_khoá][khoá][len_giá_trị][giá_trị] — thành MỘT bản ghi khoá-giá trị hoàn chỉnh, như một mục từ điển. Định dạng chỉ nhìn VỊ TRÍ, không biết (và không cần biết) ý nghĩa thật của dữ liệu."
locale: vi
track: co-so-du-lieu
module: nhat-ky-khong-bao-gio-quen
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [db.key-value-record]
requires: [db.variable-length-record]
concepts: [db.key-value-record]
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
Một nhật ký LƯU trữ thật KHÔNG chỉ MỘT chuỗi chữ — nó lưu một **CẶP**:
khoá VÀ giá trị, như MỘT mục từ ĐIỂN ("tên" TRA ra "Byte"). Ghép HAI
bản ghi biến độ DÀI lại thành MỘT, được KHÔNG?
::::

::::explain{#ghep-khoa-gia-tri}
Nối HAI khối length-prefix (bài TRƯỚC) LIỀN nhau: khối khoá TRƯỚC,
khối giá TRỊ sau:

```python title=readonly
def ghi_kv(khoa, gia_tri, kich_thuoc):
    phan_khoa = len(khoa).to_bytes(4, 'big') + khoa
    phan_gia_tri = len(gia_tri).to_bytes(4, 'big') + gia_tri
    return (phan_khoa + phan_gia_tri).ljust(kich_thuoc, b'\x00')


s = ghi_kv(b'ten', b'Byte', 24)
print(s)
```

```text title=readonly
b'\x00\x00\x00\x03ten\x00\x00\x00\x04Byte\x00\x00\x00\x00\x00\x00\x00\x00\x00'
```

`4` byte ĐẦU LÀ độ dài khoá (`3`, CHO `b'ten'`), RỒI đúng `3` byte
khoá, RỒI `4` byte độ dài giá TRỊ (`4`, cho `b'Byte'`), RỒI đúng `4`
byte giá TRỊ — HAI khối length-prefix XẾP CẠNH nhau, KHÔNG khác gì
so VỚI bài trước, chỉ LÀ giờ có HAI khối THAY vì một.
::::

::::example{#doc-lai-hai-truong}
Đọc LẠI: đọc khối khoá TRƯỚC (biết nó dài BAO nhiêu byte), TÍNH ra
khối giá trị BẮT đầu Ở đâu, RỒI đọc khối giá trị:

```python title=readonly
def doc_kv(sector_bytes):
    do_dai_khoa = int.from_bytes(sector_bytes[0:4], 'big')
    khoa = sector_bytes[4:4 + do_dai_khoa]
    vi_tri = 4 + do_dai_khoa
    do_dai_gia_tri = int.from_bytes(sector_bytes[vi_tri:vi_tri + 4], 'big')
    gia_tri = sector_bytes[vi_tri + 4:vi_tri + 4 + do_dai_gia_tri]
    return (khoa, gia_tri)


s = b'\x00\x00\x00\x03ten\x00\x00\x00\x04Byte\x00\x00\x00\x00\x00\x00\x00\x00\x00'
print(doc_kv(s))
```

```text title=readonly
(b'ten', b'Byte')
```

`vi_tri = 4 + do_dai_khoa` LÀ chỗ khối giá trị BẮT đầu — `4` byte
header khoá CỘNG `do_dai_khoa` byte NỘI dung khoá. TỪ `vi_tri`, LẶP
lại đúng CÙNG một mẫu: đọc `4` byte header, RỒI đọc ĐÚNG số byte NÓ
khai.
::::

::::predict{#doan-doi-cho-khoa-gia-tri commitOnce}
Byte lỡ ĐỔI chỗ hai đối số khi GHI — TRUYỀN `b'Byte'` làm đối số ĐẦU
(vị trí "khoá"), `b'ten'` làm đối số HAI (vị trí "giá trị"):

```python
s = ghi_kv(b'Byte', b'ten', 24)
print(doc_kv(s))
```

Dòng cuối in ra gì?

:::opt{correct}
`(b'Byte', b'ten')`
:::

:::opt
`(b'ten', b'Byte')` — vì `doc_kv` TỰ nhận ra Ý định THẬT, biết `ten`
mới đúng LÀ vai trò "khoá"
::why
Gần đúng ở việc bạn tin HỆ thống đủ "THÔNG minh" hiểu ĐÚNG Ý định
người DÙNG — MỘT niềm tin dễ hiểu SAU những bài trước.

Chỗ lệch: định DẠNG chỉ nhìn **VỊ TRÍ** — đối số ĐẦU truyền vào
`ghi_kv` LUÔN trở thành khối "khoá" TRONG bản ghi, đối số HAI luôn
thành khối "giá trị", BẤT kể tên BIẾN hay Ý nghĩa thật SỰ của dữ
liệu LÀ gì. `doc_kv` KHÔNG biết (VÀ không cần biết) `b'Byte'` "đáng
lẽ" LÀ giá trị — nó chỉ đọc ĐÚNG vị trí đã THOẢ thuận.
::
:::

:::opt
Máy báo lỗi — vì `b'Byte'` KHÔNG hợp lệ Ở vị trí "khoá"
::why
Gần đúng ở việc bạn nghĩ TỚI một RÀNG buộc kiểu dữ liệu HỢP lý.

Chỗ lệch: `ghi_kv`/`doc_kv` xử LÝ `bytes` HOÀN toàn chung CHUNG,
KHÔNG hề kiểm tra "khoá" PHẢI trông NHƯ thế nào — bất KỲ chuỗi byte
NÀO cũng hợp LỆ Ở CẢ hai vị trí.
::
:::
::::

::::code{#viet_doc_kv}
Hoàn thiện `doc_kv(sector_bytes)` — đọc phần `gia_tri`, THEO đúng
mẫu ĐÃ dùng cho `khoa`.

```python title=starter
def doc_kv(sector_bytes):
    do_dai_khoa = int.from_bytes(sector_bytes[0:4], 'big')
    khoa = sector_bytes[4:4 + do_dai_khoa]
    vi_tri = 4 + do_dai_khoa
    do_dai_gia_tri = int.from_bytes(sector_bytes[vi_tri:vi_tri + 4], 'big')
    gia_tri = ___
    return (khoa, gia_tri)


s = b'\x00\x00\x00\x03ten\x00\x00\x00\x04Byte\x00\x00\x00\x00\x00\x00\x00\x00\x00'
print(doc_kv(s))
```

```python title=solution
def doc_kv(sector_bytes):
    do_dai_khoa = int.from_bytes(sector_bytes[0:4], 'big')
    khoa = sector_bytes[4:4 + do_dai_khoa]
    vi_tri = 4 + do_dai_khoa
    do_dai_gia_tri = int.from_bytes(sector_bytes[vi_tri:vi_tri + 4], 'big')
    gia_tri = sector_bytes[vi_tri + 4:vi_tri + 4 + do_dai_gia_tri]
    return (khoa, gia_tri)


s = b'\x00\x00\x00\x03ten\x00\x00\x00\x04Byte\x00\x00\x00\x00\x00\x00\x00\x00\x00'
print(doc_kv(s))
```

```python title=test
def ghi_kv(khoa, gia_tri, kich_thuoc):
    phan_khoa = len(khoa).to_bytes(4, 'big') + khoa
    phan_gia_tri = len(gia_tri).to_bytes(4, 'big') + gia_tri
    return (phan_khoa + phan_gia_tri).ljust(kich_thuoc, b'\x00')

assert doc_kv(ghi_kv(b'ten', b'Byte', 24)) == (b'ten', b'Byte'), "cap kv mau"
assert doc_kv(ghi_kv(b'a', b'', 16)) == (b'a', b''), "gia tri rong"
assert doc_kv(ghi_kv(b'', b'x', 16)) == (b'', b'x'), "khoa rong"
assert doc_kv(ghi_kv(b'tuoi', b'25', 20)) == (b'tuoi', b'25'), "cap kv khac"
```

:::hints
- kind: attention
  body: "Cung mau nhu doc khoa: cat lat tu vi_tri + 4 toi vi_tri + 4 + do_dai_gia_tri."
- kind: strategy
  body: "sector_bytes[vi_tri + 4:vi_tri + 4 + do_dai_gia_tri]"
- kind: one-line
  body: "___ = sector_bytes[vi_tri + 4:vi_tri + 4 + do_dai_gia_tri]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai cat lat sector_bytes[vi_tri+4 : vi_tri+4+do_dai_gia_tri]
  requireAst:
  - kind: uses-name, target: vi_tri, min: 4
  - kind: uses-name, target: do_dai_gia_tri, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(b'ten', b'Byte'\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một bản ghi khoá-giá trị HOÀN chỉnh — VỊ trí quyết định VAI trò,
KHÔNG phải Ý nghĩa dữ liệu. Muốn TÌM giá trị CỦA một khoá — phải làm
SAO?
::::

::::reflect{#nghi-lai}
Ghép khoá VÀ giá trị — hai khối length-prefix LIỀN nhau, VỊ trí
quyết định VAI trò, KHÔNG phải nội dung. Nhật ký giờ CÓ thể lưu
NHIỀU cặp khoá-giá trị NỐI tiếp nhau (mỗi cặp MỘT sector, con trỏ
tăng — bài 1). NHƯNG muốn TÌM giá trị của MỘT khoá cụ THỂ — phải
LÀM sao?
::::

::::checkpoint{mastery=0.8}
::::
