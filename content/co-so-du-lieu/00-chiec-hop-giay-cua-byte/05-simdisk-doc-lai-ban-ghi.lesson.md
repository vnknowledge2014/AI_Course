---
id: co-so-du-lieu.chiec-hop-giay-cua-byte.simdisk-doc-lai-ban-ghi
title: "SimDisk: đọc lại một bản ghi"
summary: "Giải mã một bản ghi ĐỌC từ SimDisk — không so với biến gốc còn nhớ trong Python, mà cắt lát TỪNG trường theo đúng biên giới đã thoả thuận (id[0:2], tuổi[2:3], tên[3:11]) rồi giải mã ngược lại số/chữ. Sector chưa từng ghi giải mã êm ru thành (0, 0, '') — không báo lỗi gì."
locale: vi
track: co-so-du-lieu
module: chiec-hop-giay-cua-byte
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [db.decode-record]
requires: [db.simdisk-write]
concepts: [db.decode-record]
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
Đĩa ĐÃ có bản ghi RỒI. NHƯNG lần NÀY, đừng so SÁNH với biến Python
gốc CÒN nhớ — đọc THẲNG từ chuỗi byte, giải MÃ lại TỪNG trường, xem
CÓ đúng KHÔNG.
::::

::::explain{#giai-ma-tung-truong}
Ghép LẠI ba MẢNH đã học: cắt LÁT đúng biên GIỚI (bài 2), rồi giải MÃ
số (bài 1) HOẶC chữ (bài 3) CHO từng trường:

```python title=readonly
def giai_ma_ban_ghi(ban_ghi):
    idd = int.from_bytes(ban_ghi[0:2], 'big')
    tuoi = int.from_bytes(ban_ghi[2:3], 'big')
    ten = ban_ghi[3:11].rstrip(b'\x00').decode('utf-8')
    return (idd, tuoi, ten)


rec = b'\x00\x07\x19An\x00\x00\x00\x00\x00\x00'
print(giai_ma_ban_ghi(rec))
```

```text title=readonly
(7, 25, 'An')
```

BA biên giới `[0:2]`, `[2:3]`, `[3:11]` ĐÚNG khớp thứ TỰ đã ghép Ở
bài 2 (id `2` byte, tuổi `1` byte, tên `8` byte) — TỔNG cộng đúng
`11` byte. `rstrip(b'\x00')` bỏ đệm TRƯỚC khi `decode`, đúng NHƯ bài
3.
::::

::::example{#doc-tu-dia-that}
Ghi VÀO `SimDisk` (bài 4), RỒI đọc VÀ giải mã LẠI — KHÔNG chạm biến
`rec` gốc lần NÀO nữa:

```python title=readonly
class SimDisk:
    def __init__(self, so_luong_sector, kich_thuoc_sector):
        self.so_luong_sector = so_luong_sector
        self.kich_thuoc_sector = kich_thuoc_sector
        self._platter = {}
        self._cache = {}

    def write(self, sector, data):
        self._cache[sector] = bytes(data)

    def read(self, sector):
        du_lieu = self._cache.get(sector, self._platter.get(sector))
        if du_lieu is None:
            return bytes(self.kich_thuoc_sector)
        return bytes(du_lieu)

    def fsync(self):
        for sector, data in self._cache.items():
            self._platter[sector] = data
        self._cache.clear()


def giai_ma_ban_ghi(ban_ghi):
    idd = int.from_bytes(ban_ghi[0:2], 'big')
    tuoi = int.from_bytes(ban_ghi[2:3], 'big')
    ten = ban_ghi[3:11].rstrip(b'\x00').decode('utf-8')
    return (idd, tuoi, ten)


dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=11)
dia.write(0, b'\x01\x2c\x1eByte\x00\x00\x00\x00')
dia.fsync()

doc_duoc = dia.read(0)
print(giai_ma_ban_ghi(doc_duoc))
```

```text title=readonly
(300, 30, 'Byte')
```

`dia.read(0)` trả VỀ đúng chuỗi byte đã GHI, VÀ `giai_ma_ban_ghi`
đọc RA lại đúng `(300, 30, 'Byte')` — bản ghi SỐNG SÓT qua "vòng
BỀN": mã hoá → ghi → đọc → giải MÃ.
::::

::::predict{#doan-sector-chua-ghi commitOnce}
Byte giải MÃ một sector CHƯA từng ghi (toàn số `0`):

```python
ban_ghi_trong = bytes(11)
print(giai_ma_ban_ghi(ban_ghi_trong))
```

Dòng cuối in ra gì?

:::opt{correct}
`(0, 0, '')`
:::

:::opt
Máy báo lỗi — vì `11` byte toàn số `0` KHÔNG phải một bản ghi hợp lệ
::why
Gần đúng ở việc bạn nghĩ tới một khái niệm HỢP LÝ trong CSDL thật
— "bản ghi RỖNG có nên bị coi LÀ không hợp lệ".

Chỗ lệch: `giai_ma_ban_ghi` chỉ THUẦN TUÝ cắt lát RỒI giải mã, KHÔNG
hề kiểm tra "có hợp LÝ" — toàn số `0` vẫn cắt VÀ giải mã ÊM ru,
KHÔNG lỗi gì, VÌ Python KHÔNG biết (và KHÔNG cần biết) sector NÀY
"chưa từng ghi" hay "ghi số 0 thật".
::
:::

:::opt
`(0, 0, '\x00\x00\x00\x00\x00\x00\x00\x00')` — vì trường tên KHÔNG
hề bị bỏ đệm
::why
Gần đúng ở việc bạn nhớ ĐÚNG CÓ `8` byte đệm cho trường tên.

Chỗ lệch: `giai_ma_ban_ghi` CÓ gọi `.rstrip(b'\x00')` TRƯỚC khi
`decode` — MỌI byte `0` đệm CUỐI bị bỏ HẾT, nên khi TOÀN bộ `8` byte
đều LÀ `0`, phần CÒN lại rỗng HOÀN toàn, `decode` MỘT chuỗi rỗng CHO
ra đúng `''`, KHÔNG phải chuỗi chứa ký tự `\x00`.
::
:::
::::

::::code{#viet_giai_ma_ban_ghi}
Điền TRƯỜNG `tuổi` còn thiếu TRONG `giai_ma_ban_ghi`.

```python title=starter
def giai_ma_ban_ghi(ban_ghi):
    idd = int.from_bytes(ban_ghi[0:2], 'big')
    tuoi = ___
    ten = ban_ghi[3:11].rstrip(b'\x00').decode('utf-8')
    return (idd, tuoi, ten)


rec = b'\x00\x07\x19An\x00\x00\x00\x00\x00\x00'
print(giai_ma_ban_ghi(rec))
```

```python title=solution
def giai_ma_ban_ghi(ban_ghi):
    idd = int.from_bytes(ban_ghi[0:2], 'big')
    tuoi = int.from_bytes(ban_ghi[2:3], 'big')
    ten = ban_ghi[3:11].rstrip(b'\x00').decode('utf-8')
    return (idd, tuoi, ten)


rec = b'\x00\x07\x19An\x00\x00\x00\x00\x00\x00'
print(giai_ma_ban_ghi(rec))
```

```python title=test
rec = b'\x00\x07\x19An\x00\x00\x00\x00\x00\x00'
assert giai_ma_ban_ghi(rec) == (7, 25, 'An'), "ban ghi mau"
rec2 = b'\x01\x2c\x1eByte\x00\x00\x00\x00'
assert giai_ma_ban_ghi(rec2) == (300, 30, 'Byte'), "id lon hon mot byte"
rec3 = bytes(11)
assert giai_ma_ban_ghi(rec3) == (0, 0, ''), "ban ghi rong -- toan so 0"
```

:::hints
- kind: attention
  body: "Cat lat byte thu ba (chi so 2 toi 3) roi doi ve so nguyen, giong dong tren."
- kind: strategy
  body: "int.from_bytes(ban_ghi[2:3], 'big')"
- kind: one-line
  body: "tuoi = int.from_bytes(ban_ghi[2:3], 'big')"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi int.from_bytes(ban_ghi[2:3], 'big') cho truong tuoi
  requireAst:
  - kind: uses-call, target: from_bytes, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(7, 25, 'An'\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một bản ghi, ghi RỒI đọc lại ĐÚNG. NHƯNG Byte cần LƯU CẢ một HỘP đầy
bản ghi — VÀ phải sống SÓT qua MẤT điện thật SỰ, KHÔNG chỉ đọc lại
trong CÙNG một lần chạy.
::::

::::reflect{#nghi-lai}
Giải mã KHÔNG cần biến gốc — CHỈ cần đúng biên giới đã thoả thuận,
VÀ một sector chưa TỪNG ghi giải mã êm ru THÀNH `(0, 0, '')`, KHÔNG
báo lỗi. Byte đã ghi VÀ đọc lại ĐƯỢC một bản ghi, NHƯNG mới CHỈ trong
CÙNG một lần chạy — MỘT hộp giày THẬT chứa NHIỀU bản ghi, VÀ phải
sống SÓT qua một lần "MẤT điện" thật sự. Byte SẴN sàng thử CHƯA?
::::

::::checkpoint{mastery=0.8}
::::
