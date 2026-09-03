---
id: co-so-du-lieu.chiec-hop-giay-cua-byte.so-nguyen-thanh-byte
title: Số nguyên thành byte
summary: "int.to_bytes(do_dai, 'big') biến một số nguyên thành CHUỖI byte có ĐỘ DÀI CỐ ĐỊNH, big-endian — viên gạch đầu tiên của bản ghi nhị phân, khác hẳn print() (chữ, người đọc) mà file storage cần (byte, máy đọc)."
locale: vi
track: co-so-du-lieu
module: chiec-hop-giay-cua-byte
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [db.int-to-bytes]
requires: [mem.byte-range, mem.hex]
concepts: [db.int-to-bytes]
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
Byte muốn LƯU điểm số `300` VÀO một FILE — nhưng file CHỈ hiểu BYTE
(mỗi ô `0`-`255`, đã học Ở T3.1), KHÔNG hiểu "số nguyên" kiểu Python.
Biến `300` thành BAO NHIÊU byte, ĐÂY?
::::

::::explain{#so-nguyen-thanh-byte}
`n.to_bytes(do_dai, 'big')` — biến số nguyên `n` thành một CHUỖI byte
CÓ ĐỘ DÀI CỐ ĐỊNH `do_dai`, sắp XẾP kiểu `'big'` (byte "NẶNG" nhất —
đóng góp lớn NHẤT vào giá trị — đứng TRƯỚC, giống cách người viết số
`300`: hàng TRĂM trước hàng ĐƠN vị):

```python title=readonly
so = 300
b = so.to_bytes(2, 'big')

print(b)
print(list(b))
```

```text title=readonly
b'\x01,'
[1, 44]
```

`300` CẦN đúng `2` byte: byte ĐẦU LÀ `1`, byte SAU LÀ `44` — VÌ
`1 × 256 + 44 = 300` (mỗi byte LÀ MỘT chữ số Ở HỆ cơ số `256`, đúng
cách `bon-bit-mot-chu-so` T3.1 đã dạy CHO hex CƠ số `16`). `do_dai=2`
LÀ THAM số BẮT buộc — KHÔNG cho Python TỰ chọn bao nhiêu byte LÀ đủ,
VÌ một bản ghi trong file CẦN kích THƯỚC biết TRƯỚC (bài SAU sẽ dùng
đúng tính chất NÀY).
::::

::::example{#doc-lai-tu-byte}
Đọc NGƯỢC — TỪ byte VỀ số nguyên — dùng `int.from_bytes(b, 'big')`:

```python title=readonly
b = bytes([1, 44])
so = int.from_bytes(b, 'big')

print(so)
```

```text title=readonly
300
```

`to_bytes`/`from_bytes` LÀ một CẶP đảo NGƯỢC nhau: mã hoá RỒI giải mã
LẠI đúng VỀ con số BAN đầu — đây LÀ điều kiện BẮT buộc CHO bất KỲ
cách LƯU dữ liệu NÀO, KHÔNG chỉ số nguyên.
::::

::::predict{#doan-vuot-qua-do-dai commitOnce}
Byte thử LƯU `65536` (KHÔNG phải `65535`) VÀO đúng `2` byte:

```python
so = 65536
b = so.to_bytes(2, 'big')
print(b)
```

Dòng cuối in ra gì?

:::opt{correct}
Máy báo lỗi (`OverflowError`)
:::

:::opt
`b'\x00\x00\x01\x00'` — vì Python TỰ ĐỘNG dùng THÊM byte khi `2` byte
KHÔNG đủ chỗ chứa
::why
Gần đúng ở việc bạn nghĩ Python LINH hoạt, TỰ lo liệu CHỖ chứa —
đúng VỚI kiểu `int` (không giới HẠN độ LỚN, đã học) khi KHÔNG ép
VỀ byte.

Chỗ lệch: `to_bytes(2, ...)` ĐÒI đúng `2` byte, KHÔNG hơn KHÔNG kém
— `65536` CẦN tới `3` byte (`0x010000`) MỚI vừa, VƯỢT quá `2` byte
YÊU cầu LÀM Python DỪNG lại NGAY VÀ báo `OverflowError`, KHÔNG tự
"giãn" thêm byte.
::
:::

:::opt
`b'\xff\xff'` — vì số VƯỢT quá GIỚI hạn `2` byte bị "CẮT BỚT" VỀ giá
trị LỚN nhất VỪA hai byte chứa được
::why
Gần đúng ở việc bạn nhớ TỚI tràn số (T3.1 `mem.overflow`) — một liên
TƯỞNG hợp LÝ VỀ giới hạn kích thước cố định.

Chỗ lệch: kiểu cố định (nhƯ `uint16` trong C) MỚI "cắt/quấn VÒNG"
âm thầm khi tràn — `to_bytes` CỦA Python KHÔNG âm thầm cắt BẤT KỲ
điều gì, nó KIỂM tra trước VÀ báo lỗi RÕ ràng thay VÌ trả VỀ một
con số SAI mà KHÔNG ai hay.
::
:::
::::

::::code{#viet_so_thanh_byte}
Viết `so_thanh_byte(n, do_dai)` — trả VỀ `n` dưới dạng byte, ĐỘ dài
`do_dai`, big-endian.

```python title=starter
def so_thanh_byte(n, do_dai):
    return ___


print(so_thanh_byte(300, 2))
```

```python title=solution
def so_thanh_byte(n, do_dai):
    return n.to_bytes(do_dai, 'big')


print(so_thanh_byte(300, 2))
```

```python title=test
assert so_thanh_byte(300, 2) == b'\x01\x2c', "300 -- hai byte 1,44"
assert so_thanh_byte(0, 2) == b'\x00\x00', "0 -- toan so 0"
assert so_thanh_byte(7, 2) == b'\x00\x07', "7 -- byte dau la 0"
assert so_thanh_byte(255, 1) == b'\xff', "255 -- vua khit mot byte"
assert so_thanh_byte(65535, 2) == b'\xff\xff', "65535 -- vua khit hai byte"
```

:::hints
- kind: attention
  body: "Dung dung phuong thuc to_bytes cua so nguyen, voi thu tu 'big'."
- kind: strategy
  body: "n.to_bytes(do_dai, 'big')"
- kind: one-line
  body: "___ = n.to_bytes(do_dai, 'big')"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi n.to_bytes(do_dai, 'big')
  requireAst:
  - kind: uses-call, target: to_bytes, min: 1
  - kind: uses-name, target: do_dai, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^b'\\x01,'\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một con số, giờ LÀ một CHUỖI byte có độ dài BIẾT trước. Nếu BYTE cần
lưu KHÔNG chỉ một con số, MÀ nhiều con số CÙNG lúc — GHÉP chúng LẠI
sao đây?
::::

::::reflect{#nghi-lai}
Một con số, giờ LÀ một chuỗi byte có ĐỘ dài biết TRƯỚC — `to_bytes`
KHÔNG cho Python tự CHỌN, VÌ một bản ghi TRONG file cần kích thước
biết TRƯỚC. Nếu byte cần lưu KHÔNG chỉ một con số, MÀ NHIỀU con số
CÙNG lúc (như một bản ghi CÓ nhiều trường) — ghép CHÚNG lại RA sao?
::::

::::checkpoint{mastery=0.8}
::::
