---
id: co-so-du-lieu.chiec-hop-giay-cua-byte.chuoi-co-do-dai-co-dinh
title: Chuỗi có độ dài cố định
summary: "encode('utf-8').ljust(N, b'\\x00')[:N] ép MỘT chuỗi chữ về đúng N byte — đệm 0 nếu ngắn, CẮT BỚT nếu dài; rstrip(b'\\x00').decode('utf-8') đọc lại. Trường tên giờ CŨNG rộng cố định như trường số."
locale: vi
track: co-so-du-lieu
module: chiec-hop-giay-cua-byte
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [db.fixed-width-string]
requires: [db.fixed-width-record, mem.utf8-bytes]
concepts: [db.fixed-width-string]
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
Trường `id` LÀ số — RỘNG cố định, DỄ. NHƯNG một trường TÊN LÀ chữ,
VÀ tên MỖI người MỘT độ dài KHÁC nhau — làm sao Ép nó VỀ đúng MỘT
độ rộng byte cố định?
::::

::::explain{#dem-va-cat}
Hai BƯỚC: `.encode('utf-8')` (đã học Ở T3.1 — chữ THÀNH byte) RỒI
`.ljust(do_dai, b'\x00')` ĐỆM thêm byte `0` cho đủ `do_dai`, RỒI
`[:do_dai]` CẮT bớt phần THỪA nếu tên DÀI hơn `do_dai`:

```python title=readonly
ten = 'Byte'
b = ten.encode('utf-8').ljust(8, b'\x00')[:8]

print(b)
print(len(b))
```

```text title=readonly
b'Byte\x00\x00\x00\x00'
8
```

`'Byte'` chiếm `4` byte (mỗi CHỮ CÁI ASCII đúng `1` byte), `.ljust(8,
b'\x00')` ĐỆM thêm `4` byte `0` cho đủ `8`. Chữ TIẾNG VIỆT có DẤU
chiếm NHIỀU byte hơn MỘT (`mem.utf8-bytes` đã học — VÍ dụ `'ư'` LÀ
`2` byte), NÊN cùng "SỐ ký tự" KHÔNG có nghĩa LÀ cùng "SỐ byte".
::::

::::example{#doc-lai-chuoi}
Đọc NGƯỢC: bỏ phần đệm `0` (`rstrip`) RỒI giải mã VỀ chữ:

```python title=readonly
b = b'Byte\x00\x00\x00\x00'
ten = b.rstrip(b'\x00').decode('utf-8')

print(repr(ten))
```

```text title=readonly
'Byte'
```

`rstrip(b'\x00')` bỏ HẾT byte `0` Ở CUỐI (đúng phần `.ljust` vừa
thêm), CÒN lại đúng `4` byte GỐC, `.decode('utf-8')` đọc CHÚNG thành
chữ TRỞ lại.
::::

::::predict{#doan-ten-qua-dai commitOnce}
Byte thử ép TÊN `'Alexandria'` (`10` ký tự) VÀO đúng `8` byte:

```python
ten = 'Alexandria'
b = ten.encode('utf-8').ljust(8, b'\x00')[:8]
print(b)
```

Dòng cuối in ra gì?

:::opt{correct}
`b'Alexandr'`
:::

:::opt
Máy báo lỗi — vì TÊN `10` ký tự KHÔNG thể nào vừa VÀO `8` byte
::why
Gần đúng ở việc bạn NHẬN ra RÕ ràng có SỰ THIẾU chỗ — một quan sát
ĐÚNG hướng.

Chỗ lệch: `[:8]` KHÔNG hề báo lỗi khi CẮT một chuỗi/byte quá DÀI —
nó ĐƠN giản CẮT LẤY đúng `8` byte ĐẦU rồi bỏ phần CÒN lại, im lặng,
KHÔNG báo hiệu GÌ.
::
:::

:::opt
`b'Alexandria'` — vì `[:8]` chỉ CẮT nếu chuỗi có ĐÚNG hơn `8` KÝ TỰ,
mà đây chưa vượt quá `do_dai` byte tính THEO ký tự
::why
Gần đúng ở việc bạn phân BIỆT "ký tự" VÀ "byte" (đúng NĂNG lực bài
NÀY đang dạy) — nhưng ÁP dụng NGƯỢC hướng.

Chỗ lệch: `'Alexandria'` LÀ toàn chữ ASCII, MỖI ký tự đúng `1` byte,
NÊN `10` ký tự = `10` byte — VƯỢT `8` byte YÊU cầu, và `[:8]` (cắt
TRÊN `bytes`, SAU khi `.encode()`) LUÔN cắt theo ĐÚNG số byte, KHÔNG
theo số ký tự.
::
:::
::::

::::code{#viet_ten_thanh_byte}
Viết `ten_thanh_byte(ten, do_dai)` — Ép `ten` VỀ đúng `do_dai` byte:
đệm `0` NẾU ngắn, cắt bớt NẾU dài.

```python title=starter
def ten_thanh_byte(ten, do_dai):
    return ___


print(ten_thanh_byte('An', 8))
```

```python title=solution
def ten_thanh_byte(ten, do_dai):
    return ten.encode('utf-8').ljust(do_dai, b'\x00')[:do_dai]


print(ten_thanh_byte('An', 8))
```

```python title=test
assert ten_thanh_byte('An', 8) == b'An\x00\x00\x00\x00\x00\x00', "ten ngan -- dem 0"
assert ten_thanh_byte('Byte', 8) == b'Byte\x00\x00\x00\x00', "vua nua -- dem it hon"
assert ten_thanh_byte('Alexandria', 8) == b'Alexandr', "ten dai -- cat bot"
assert ten_thanh_byte('', 8) == b'\x00' * 8, "ten rong -- toan dem 0"
assert len(ten_thanh_byte('X', 8)) == 8, "luon dung do_dai byte"
```

:::hints
- kind: attention
  body: "encode('utf-8') truoc, roi ljust(do_dai, b'\\x00'), roi cat [:do_dai]."
- kind: strategy
  body: "ten.encode('utf-8').ljust(do_dai, b'\\x00')[:do_dai]"
- kind: one-line
  body: "___ = ten.encode('utf-8').ljust(do_dai, b'\\x00')[:do_dai]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai encode('utf-8') roi ljust(do_dai, b'\x00') roi cat [:do_dai]
  requireAst:
  - kind: uses-call, target: encode, min: 1
  - kind: uses-call, target: ljust, min: 1
  - kind: uses-name, target: do_dai, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^b'An\\x00\\x00\\x00\\x00\\x00\\x00'\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Số CŨNG rộng cố định, chữ giờ CŨNG rộng cố định. Ghép id (`2` byte) +
tuổi (`1` byte) + tên (`8` byte) — MỘT bản ghi ĐẦY đủ, sẵn sàng LƯU
VÀO đĩa thật SỰ chưa?
::::

::::reflect{#nghi-lai}
Chữ, giờ CŨNG ép VỀ được đúng một độ rộng byte cố định — đệm `0` nếu
ngắn, cắt lặng LẼ nếu dài. Ghép id + tuổi + tên LẠI thành MỘT bản ghi
ĐẦY đủ — nhưng bản ghi đó đang SỐNG trong biến Python, MẤT NGAY khi
chương trình tắt. LÀM sao LƯU nó VÀO một nơi SỐNG SÓT qua lần tắt
đó?
::::

::::checkpoint{mastery=0.8}
::::
