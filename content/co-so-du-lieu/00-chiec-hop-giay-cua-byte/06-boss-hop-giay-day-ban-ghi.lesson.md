---
id: co-so-du-lieu.chiec-hop-giay-cua-byte.boss-hop-giay-day-ban-ghi
title: "BOSS — Chiếc hộp giày đầy bản ghi"
summary: "Ghép TRỌN q00: ghi NHIỀU bản ghi, MỖI bản ghi một sector (vị trí → sector, O(1), KHÔNG cần quét), fsync MỘT LẦN, mô phỏng MẤT điện (crash()), rồi đọc lại HẾT — bản ghi ĐÃ fsync sống sót, bản ghi CHƯA fsync biến mất."
locale: vi
track: co-so-du-lieu
module: chiec-hop-giay-cua-byte
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.decode-record, db.simdisk-write]
concepts: [db.boss-q00]
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
Một bản ghi, ghi VÀ đọc lại ĐÚNG — CHỈ mới chứng minh Ở quy MÔ nhỏ.
Một hộp giày THẬT chứa NHIỀU bản ghi, VÀ phải sống SÓT qua MẤT điện
thật SỰ. Byte SẴN sàng thử.
::::

::::explain{#vi-tri-la-sector}
Fixed-width record CÓ một SIÊU năng lực: bản ghi thứ `i` TRONG một
DANH sách — GHI thẳng VÀO sector `i`. MUỐN đọc bản ghi thứ `i`? Đọc
ĐÚNG sector `i`, KHÔNG cần quét QUA những bản ghi khác:

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

    def crash(self):
        self._cache.clear()

    def co_ghi_chua_fsync(self):
        return len(self._cache) > 0


dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=11)
recs = [
    b'\x00\x07\x19An\x00\x00\x00\x00\x00\x00',
    b'\x01\x2c\x1eByte\x00\x00\x00\x00',
]
for i, ban_ghi in enumerate(recs):
    dia.write(i, ban_ghi)
dia.fsync()

print(dia.read(1) == recs[1])
```

```text title=readonly
True
```

Bản ghi thứ `1` NẰM ngay Ở sector `1` — MỘT phép TÍNH, KHÔNG phải
MỘT vòng quét. `fsync()` gọi ĐÚNG MỘT lần Ở CUỐI (KHÔNG phải MỖI
lần ghi) — bền HẾT trong MỘT lượt, hiệu quả HƠN fsync TỪNG bản ghi.
::::

::::example{#crash-mat-dien}
`crash()` mô phỏng MẤT điện — CHỈ những ghi ĐÃ `fsync()` mới sống
SÓT:

```python title=readonly
dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=3)
dia.write(0, b'xyz')
dia.fsync()
dia.write(1, b'abc')
dia.crash()

print(dia.read(0))
print(dia.read(1))
```

```text title=readonly
b'xyz'
b'\x00\x00\x00'
```

Sector `0` ĐÃ `fsync()` TRƯỚC khi crash — SỐNG sót nguyên VẸN. Sector
`1` GHI xong nhưng CHƯA kịp `fsync()` — `crash()` xoá SẠCH cache,
sector `1` trở LẠI toàn số `0` (như CHƯA từng ghi GÌ).
::::

::::predict{#doan-fsync-mot-lan commitOnce}
Byte ghi BA bản ghi, `fsync()` MỘT lần Ở CUỐI, RỒI mới `crash()`:

```python
dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=3)
dia.write(0, b'aaa')
dia.write(1, b'bbb')
dia.write(2, b'ccc')
dia.fsync()
dia.crash()
print(dia.read(0), dia.read(1), dia.read(2))
```

Dòng cuối in ra gì?

:::opt{correct}
`b'aaa' b'bbb' b'ccc'`
:::

:::opt
`b'aaa' b'\x00\x00\x00' b'\x00\x00\x00'` — vì `fsync()` CHỈ đẩy được
bản ghi ĐẦU tiên trong cache MỖI lần gọi
::why
Gần đúng ở việc bạn HÌNH dung `fsync()` xử LÝ "MỘT phần tử MỘT lần"
— một giả định hợp LÝ NẾU nhìn theo lối vòng lặp XỬ lý từng phần
tử quen thuộc.

Chỗ lệch: `fsync()` LẶP qua **TOÀN BỘ** cache TRONG một lần gọi
(`for sector, data in self._cache.items()`) — CẢ ba bản ghi (`0`,
`1`, `2`) đều ĐANG trong cache LÚC gọi `fsync()`, NÊN cả ba đều
được đẩy XUỐNG platter CÙNG lúc, KHÔNG chỉ MỘT.
::
:::

:::opt
Máy báo lỗi — vì `crash()` gọi NGAY sau `fsync()`, KHÔNG có khoảng
"nghỉ" giữa hai lệnh
::why
Gần đúng ở việc bạn NGHĨ có một ĐIỀU kiện thời GIAN cần tuân THỦ
giữa hai thao TÁC quan trọng.

Chỗ lệch: `SimDisk` KHÔNG có khái niệm "thời gian NGHỈ" — `fsync()`
VÀ `crash()` chỉ LÀ hai lời GỌI phương thức bình THƯỜNG, gọi NGAY
sau nhau HOÀN toàn hợp lệ, KHÔNG lỗi gì.
::
:::
::::

::::code{#viet_xay_hop_giay}
Viết `xay_hop_giay(dia, danh_sach_ban_ghi)` — ghi MỖI bản ghi VÀO
sector đúng VỊ trí của nó TRONG danh sách, RỒI `fsync()` một lần.

```python title=starter
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

    def crash(self):
        self._cache.clear()

    def co_ghi_chua_fsync(self):
        return len(self._cache) > 0


def xay_hop_giay(dia, danh_sach_ban_ghi):
    for i, ban_ghi in enumerate(danh_sach_ban_ghi):
        ___
    dia.fsync()


dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=11)
recs = [
    b'\x00\x07\x19An\x00\x00\x00\x00\x00\x00',
    b'\x01\x2c\x1eByte\x00\x00\x00\x00',
]
xay_hop_giay(dia, recs)
dia.crash()
print(dia.read(0) == recs[0], dia.read(1) == recs[1])
```

```python title=solution
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

    def crash(self):
        self._cache.clear()

    def co_ghi_chua_fsync(self):
        return len(self._cache) > 0


def xay_hop_giay(dia, danh_sach_ban_ghi):
    for i, ban_ghi in enumerate(danh_sach_ban_ghi):
        dia.write(i, ban_ghi)
    dia.fsync()


dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=11)
recs = [
    b'\x00\x07\x19An\x00\x00\x00\x00\x00\x00',
    b'\x01\x2c\x1eByte\x00\x00\x00\x00',
]
xay_hop_giay(dia, recs)
dia.crash()
print(dia.read(0) == recs[0], dia.read(1) == recs[1])
```

```python title=test
dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=11)
recs = [
    b'\x00\x07\x19An\x00\x00\x00\x00\x00\x00',
    b'\x01\x2c\x1eByte\x00\x00\x00\x00',
]
xay_hop_giay(dia, recs)
assert dia.co_ghi_chua_fsync() is False, "phai fsync mot lan sau khi ghi het"
dia.crash()
assert dia.read(0) == recs[0], "ban ghi 0 song sot qua crash"
assert dia.read(1) == recs[1], "ban ghi 1 song sot qua crash"

dia2 = SimDisk(so_luong_sector=2, kich_thuoc_sector=3)
xay_hop_giay(dia2, [b'abc'])
assert dia2.read(0) == b'abc', "mot ban ghi duy nhat van dung"
```

:::hints
- kind: attention
  body: "Trong vong lap, goi dia.write voi sector la i va du lieu la ban_ghi."
- kind: strategy
  body: "dia.write(i, ban_ghi)"
- kind: one-line
  body: "dia.write(i, ban_ghi)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi dia.write(i, ban_ghi) trong vong lap
  requireAst:
  - kind: uses-call, target: write, min: 1
  - kind: uses-name, target: i, min: 1
  - kind: uses-name, target: ban_ghi, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chiếc hộp giày ĐẦU tiên — VỊ trí LÀ sector, `fsync()` MỘT lần, sống
SÓT qua "mất điện". NHƯNG hộp giày cần biết TRƯỚC sẽ có BAO NHIÊU
bản ghi — nếu KHÔNG biết trước, MÀ chỉ liên TỤC thêm bản ghi MỚI VÀO
CUỐI, không GIỚI hạn, thì SAO?
::::

::::reflect{#nghi-lai}
Fixed-width record cho MỘT siêu năng lực: vị TRÍ trong danh sách LÀ
CHÍNH sector — đọc bản ghi thứ `i` LÀ MỘT phép tính, KHÔNG một vòng
quét. `fsync()` MỘT lần Ở cuối bền HẾT MỌI ghi TRONG cache CÙNG lúc;
CHỈ ghi CHƯA kịp fsync mới MẤT khi "mất điện". Nhưng hộp giày NÀY
cần biết TRƯỚC sẽ có bao NHIÊU bản ghi (`so_luong_sector` đặt SẴN
lúc tạo `SimDisk`) — nếu KHÔNG biết trước, mà CHỈ liên tục THÊM bản
ghi MỚI vào CUỐI, không GIỚI hạn nào cả, thì "vị trí LÀ sector" còn
dùng được NỮA không?
::::

::::checkpoint{mastery=0.85}
::::
