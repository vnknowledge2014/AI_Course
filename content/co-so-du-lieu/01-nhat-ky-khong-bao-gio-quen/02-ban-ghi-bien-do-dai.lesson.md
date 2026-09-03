---
id: co-so-du-lieu.nhat-ky-khong-bao-gio-quen.ban-ghi-bien-do-dai
title: Bản ghi có độ dài thay đổi
summary: "Header 4-byte ghi ĐỘ DÀI THẬT của payload trước khi ghi nó — đọc lại cắt ĐÚNG số byte đó, KHÔNG dò tìm byte 0 kết thúc như q00. Length-prefix bền hơn null-padding/rstrip: đọc đúng NGAY CẢ khi payload chứa byte 0 ở giữa (dữ liệu nhị phân thật)."
locale: vi
track: co-so-du-lieu
module: nhat-ky-khong-bao-gio-quen
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [db.variable-length-record]
requires: [db.append-log]
concepts: [db.variable-length-record]
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
MỖI dòng nhật ký Ở bài TRƯỚC rộng ĐÚNG `3` byte — CỐ định. NHƯNG một
dòng THẬT dài NGẮN khác nhau — LÀM sao một sector RỘNG cố định chứa
được NHIỀU độ dài khác NHAU?
::::

::::explain{#header-do-dai}
Ghi TRƯỚC một **header `4` byte** chứa ĐỘ dài THẬT của payload, RỒI
mới TỚI chính payload — phần CÒN lại của sector BỎ trống (đệm `0`,
KHÔNG quan trọng, VÌ header đã nói RÕ dừng Ở đâu):

```python title=readonly
def ghi_bien_do_dai(du_lieu, kich_thuoc):
    do_dai = len(du_lieu).to_bytes(4, 'big')
    return (do_dai + du_lieu).ljust(kich_thuoc, b'\x00')


def doc_bien_do_dai(sector_bytes):
    do_dai = int.from_bytes(sector_bytes[0:4], 'big')
    return sector_bytes[4:4 + do_dai]


s = ghi_bien_do_dai(b'cats', 16)
print(s)
print(doc_bien_do_dai(s))
```

```text title=readonly
b'\x00\x00\x00\x04cats\x00\x00\x00\x00\x00\x00\x00\x00'
b'cats'
```

`4` byte ĐẦU LÀ `len(b'cats')` = `4`, mã HOÁ bằng `to_bytes` (bài 1
Ở q00) — RỒI mới TỚI chính `b'cats'`. Đọc LẠI: đọc header BIẾT ngay
"phần THẬT dài `4` byte", cắt ĐÚNG `4` byte SAU header, KHÔNG cần
đoán.
::::

::::example{#do-dai-khac-nhau}
Payload DÀI hơn — VẪN đọc đúng, VÌ header LUÔN nói THẬT:

```python title=readonly
def ghi_bien_do_dai(du_lieu, kich_thuoc):
    do_dai = len(du_lieu).to_bytes(4, 'big')
    return (do_dai + du_lieu).ljust(kich_thuoc, b'\x00')


def doc_bien_do_dai(sector_bytes):
    do_dai = int.from_bytes(sector_bytes[0:4], 'big')
    return sector_bytes[4:4 + do_dai]


s_ngan = ghi_bien_do_dai(b'cats', 16)
s_dai = ghi_bien_do_dai(b'hello world!', 16)

print(doc_bien_do_dai(s_ngan))
print(doc_bien_do_dai(s_dai))
```

```text title=readonly
b'cats'
b'hello world!'
```

`b'cats'` (`4` byte) VÀ `b'hello world!'` (`12` byte) — HAI độ dài
KHÁC nhau, CÙNG rộng `16` byte sector, ĐỌC lại đúng CẢ hai, VÌ MỖI
header TỰ khai đúng ĐỘ dài của CHÍNH nó.
::::

::::predict{#doan-payload-co-byte-0 commitOnce}
Byte lưu một payload CHỨA byte `0` Ở GIỮA (`b'ab\x00cd'`, `5` byte):

```python
s = ghi_bien_do_dai(b'ab\x00cd', 16)
print(doc_bien_do_dai(s))
```

Dòng cuối in ra gì?

:::opt{correct}
`b'ab\x00cd'`
:::

:::opt
`b'ab'` — vì đọc dừng LẠI khi gặp byte `0` đầu TIÊN, giống chuỗi TÊN
cố định Ở q00
::why
Gần đúng ở việc bạn nhớ ĐÚNG bài `chuoi-co-do-dai-co-dinh` (q00):
`.rstrip(b'\x00')` bỏ đệm `0` Ở CUỐI — một PHẢN xạ hợp lý NẾU áp
dụng ĐÚNG chỗ.

Chỗ lệch: `doc_bien_do_dai` **KHÔNG hề gọi `rstrip`** — nó đọc ĐÚNG
số byte GHI trong header (`4` byte, tính TỪ `len()`), KHÔNG quan
tâm những byte ĐÓ có giá trị `0` hay KHÔNG. Đây chính LÀ điểm MẠNH
của length-prefix SO với đệm/rstrip: dữ liệu nhị PHÂN thật (KHÔNG
chỉ chữ) có THỂ chứa byte `0` Ở BẤT KỲ đâu, VẪN đọc lại đúng.
::
:::

:::opt
Máy báo lỗi — vì `5` byte payload KHÔNG khớp `4` byte header MÀ
Python "tưởng" đã đọc
::why
Gần đúng ở việc bạn LO ngại một sự KHÔNG khớp giữa header VÀ payload
— một mối lo hợp LÝ nếu header VÀ payload có THỂ lệch nhau.

Chỗ lệch: header LUÔN được TÍNH đúng từ `len(du_lieu)` NGAY lúc
ghi (`ghi_bien_do_dai`) — `b'ab\x00cd'` DÀI `5` byte THẬT, header
ghi ĐÚNG `5`, KHÔNG có gì lệch NHAU để báo lỗi.
::
:::
::::

::::code{#viet_doc_bien_do_dai}
Hoàn thiện `doc_bien_do_dai(sector_bytes)` — đọc phần payload THEO
đúng độ dài GHI trong header.

```python title=starter
def ghi_bien_do_dai(du_lieu, kich_thuoc):
    do_dai = len(du_lieu).to_bytes(4, 'big')
    return (do_dai + du_lieu).ljust(kich_thuoc, b'\x00')


def doc_bien_do_dai(sector_bytes):
    do_dai = int.from_bytes(sector_bytes[0:4], 'big')
    return ___


print(doc_bien_do_dai(ghi_bien_do_dai(b'cats', 16)))
```

```python title=solution
def ghi_bien_do_dai(du_lieu, kich_thuoc):
    do_dai = len(du_lieu).to_bytes(4, 'big')
    return (do_dai + du_lieu).ljust(kich_thuoc, b'\x00')


def doc_bien_do_dai(sector_bytes):
    do_dai = int.from_bytes(sector_bytes[0:4], 'big')
    return sector_bytes[4:4 + do_dai]


print(doc_bien_do_dai(ghi_bien_do_dai(b'cats', 16)))
```

```python title=test
assert doc_bien_do_dai(ghi_bien_do_dai(b'cats', 16)) == b'cats', "payload ngan"
assert doc_bien_do_dai(ghi_bien_do_dai(b'hello world!', 16)) == b'hello world!', "payload vua khit"
assert doc_bien_do_dai(ghi_bien_do_dai(b'', 16)) == b'', "payload rong"
assert doc_bien_do_dai(ghi_bien_do_dai(b'ab\x00cd', 16)) == b'ab\x00cd', "payload co byte 0 o giua"
```

:::hints
- kind: attention
  body: "Cat lat sector_bytes tu chi so 4 toi 4 + do_dai."
- kind: strategy
  body: "sector_bytes[4:4 + do_dai]"
- kind: one-line
  body: "___ = sector_bytes[4:4 + do_dai]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai cat lat sector_bytes[4:4 + do_dai]
  requireAst:
  - kind: uses-name, target: do_dai, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^b'cats'\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Độ dài THAY đổi được, VÀ đọc lại VẪN đúng — dù CÓ byte `0` Ở giữa.
Nhật ký giờ có THỂ chứa GÌ — dòng CHỮ đơn thuần, HAY một CẶP khoá
VÀ giá trị?
::::

::::reflect{#nghi-lai}
Length-prefix mạnh HƠN đệm/rstrip: đọc đúng dù payload chứa byte `0`
Ở BẤT kỳ đâu — VÌ header nói THẬT độ dài, KHÔNG cần dò tìm điểm
dừng. Một cuốn nhật KÝ lưu trữ THẬT thường KHÔNG chỉ MỘT chuỗi chữ —
NÓ lưu một **CẶP** khoá VÀ giá trị (như MỘT mục từ ĐIỂN). Ghép hai
bản ghi biến độ DÀI — MỘT làm khoá, MỘT làm giá trị — thành MỘT bản
ghi hoàn CHỈNH, được KHÔNG?
::::

::::checkpoint{mastery=0.8}
::::
