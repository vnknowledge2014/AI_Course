---
id: co-so-du-lieu.chiec-hop-giay-cua-byte.simdisk-ghi-mot-ban-ghi
title: "SimDisk: ghi một bản ghi"
summary: "SimDisk — đĩa mô phỏng theo sector (write/read/fsync), vì nơi chạy code Python của Byte không mô phỏng nổi một ổ đĩa thật. write(sector, data) LƯU vào cache; fsync() mới đẩy XUỐNG chỗ bền — nhưng read() nhìn cache TRƯỚC, nên đọc lại đúng NGAY cả khi CHƯA fsync."
locale: vi
track: co-so-du-lieu
module: chiec-hop-giay-cua-byte
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [db.simdisk-write]
requires: [db.fixed-width-string]
concepts: [db.simdisk-write]
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
Bản ghi ĐANG sống TRONG một biến Python — TẮT chương trình LÀ MẤT
sạch. Byte cần một nơi SỐNG SÓT qua lần tắt ĐÓ — một CÁI "đĩa".
::::

::::explain{#gioi-thieu-simdisk}
Nơi code Python của Byte chạy KHÔNG mô phỏng nổi một Ổ đĩa THẬT (đó
LÀ chi tiết máy — Byte chưa cần biết TẠI SAO ngay), NÊN Byte TỰ xây
một CÁI đĩa mô phỏng, GỌI LÀ `SimDisk`: chia thành nhiều **sector**
đánh SỐ, MỖI sector rộng ĐÚNG một số byte CỐ định — CHÍNH LÀ chỗ
LƯU một bản ghi rộng CỐ định vừa xây (bài 1-3):

```python title=readonly
class SimDisk:
    def __init__(self, so_luong_sector, kich_thuoc_sector):
        self.so_luong_sector = so_luong_sector
        self.kich_thuoc_sector = kich_thuoc_sector
        self._platter = {}
        self._cache = {}

    def write(self, sector, data):
        if len(data) != self.kich_thuoc_sector:
            raise ValueError(f"can dung {self.kich_thuoc_sector} byte")
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

    def co_ghi_chua_fsync(self):
        return len(self._cache) > 0


dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=11)
rec = b'\x00\x07\x19An\x00\x00\x00\x00\x00\x00'
dia.write(0, rec)
dia.fsync()

print(dia.read(0) == rec)
```

```text title=readonly
True
```

`write(sector, data)` LƯU bản ghi VÀO **cache** (chưa BỀN), `fsync()`
mới đẩy CACHE xuống **platter** (chỗ BỀN — sống SÓT qua "mất điện" mô
phỏng, bài SAU sẽ thấy). `kich_thuoc_sector=11` khớp đúng ĐỘ dài bản
ghi id(`2`)+tuổi(`1`)+tên(`8`) = `11` byte — MỘT bản ghi VỪA đúng
MỘT sector.
::::

::::example{#doc-truoc-khi-fsync}
Đọc `dia.read(sector)` KHÔNG cần `fsync()` trước — `read` LUÔN nhìn
**cache** TRƯỚC:

```python title=readonly
dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=3)
dia.write(0, b'xyz')

print(dia.read(0))
print(dia.co_ghi_chua_fsync())
```

```text title=readonly
b'xyz'
True
```

CHƯA gọi `fsync()`, NHƯNG `read(0)` VẪN thấy `b'xyz'` NGAY — VÌ nó
CÒN trong cache. `co_ghi_chua_fsync()` trả VỀ `True`: CÓ ghi đang
CHỜ, CHƯA bền.
::::

::::predict{#doan-doc-truoc-fsync commitOnce}
Byte ghi RỒI đọc lại NGAY, KHÔNG gọi `fsync()`:

```python
dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=11)
rec = b'\x00\x07\x19An\x00\x00\x00\x00\x00\x00'
dia.write(0, rec)
print(dia.read(0) == rec)
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — vì CHƯA `fsync()` nên đĩa CHƯA thật sự LƯU gì cả
::why
Gần đúng ở việc bạn LO ngại đúng hướng — `fsync()` THẬT sự quan
trọng CHO độ bền (bài SAU sẽ thấy VÌ sao).

Chỗ lệch: `read()` nhìn **cache** TRƯỚC platter — GHI xong LÀ ĐỌC
lại thấy NGAY, dù `fsync()` CHƯA gọi. `fsync()` CHỈ quan trọng NẾU
có "mất điện" (mô phỏng) XẢY ra TRƯỚC khi nó được gọi — CHỨ đọc bình
thường LUÔN thấy dữ liệu MỚI nhất.
::
:::

:::opt
Máy báo lỗi — vì gọi `read()` TRƯỚC `fsync()` LÀ sai TRÌNH tự
::why
Gần đúng ở việc bạn NGHĨ có một trình TỰ bắt buộc phải tuân THỦ —
một trực GIÁC hợp lý VỚI hệ thống có ràng buộc chặt.

Chỗ lệch: `SimDisk` KHÔNG ép buộc trình tự NÀO — `write` RỒI `read`
NGAY, hay `write` RỒI `fsync` RỒI `read`, đều hoạt ĐỘNG bình thường,
CHỈ khác NHAU Ở việc dữ liệu CÓ sống SÓT qua "mất điện" hay KHÔNG.
::
:::
::::

::::code{#viet_ghi_ban_ghi}
Viết `ghi_ban_ghi(dia, sector, du_lieu)` — GHI `du_lieu` VÀO `sector`
CỦA `dia`, RỒI làm CHO nó BỀN ngay.

```python title=starter
class SimDisk:
    def __init__(self, so_luong_sector, kich_thuoc_sector):
        self.so_luong_sector = so_luong_sector
        self.kich_thuoc_sector = kich_thuoc_sector
        self._platter = {}
        self._cache = {}

    def write(self, sector, data):
        if len(data) != self.kich_thuoc_sector:
            raise ValueError(f"can dung {self.kich_thuoc_sector} byte")
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

    def co_ghi_chua_fsync(self):
        return len(self._cache) > 0


def ghi_ban_ghi(dia, sector, du_lieu):
    ___


dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=11)
rec = b'\x00\x07\x19An\x00\x00\x00\x00\x00\x00'
ghi_ban_ghi(dia, 0, rec)
print(dia.read(0) == rec)
```

```python title=solution
class SimDisk:
    def __init__(self, so_luong_sector, kich_thuoc_sector):
        self.so_luong_sector = so_luong_sector
        self.kich_thuoc_sector = kich_thuoc_sector
        self._platter = {}
        self._cache = {}

    def write(self, sector, data):
        if len(data) != self.kich_thuoc_sector:
            raise ValueError(f"can dung {self.kich_thuoc_sector} byte")
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

    def co_ghi_chua_fsync(self):
        return len(self._cache) > 0


def ghi_ban_ghi(dia, sector, du_lieu):
    dia.write(sector, du_lieu)
    dia.fsync()


dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=11)
rec = b'\x00\x07\x19An\x00\x00\x00\x00\x00\x00'
ghi_ban_ghi(dia, 0, rec)
print(dia.read(0) == rec)
```

```python title=test
dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=11)
rec = b'\x00\x07\x19An\x00\x00\x00\x00\x00\x00'
ghi_ban_ghi(dia, 0, rec)
assert dia.read(0) == rec, "doc lai dung sau ghi"
assert dia.co_ghi_chua_fsync() is False, "phai fsync -- cache rong sau khi ghi xong"

rec2 = b'\x00\x08\x1aBi\x00\x00\x00\x00\x00\x00'
ghi_ban_ghi(dia, 1, rec2)
assert dia.read(1) == rec2, "sector khac -- doc dung ban ghi rieng"
assert dia.read(0) == rec, "ghi sector 1 khong lam hong sector 0"
```

:::hints
- kind: attention
  body: "Goi dia.write(sector, du_lieu) roi dia.fsync()."
- kind: strategy
  body: "dia.write(sector, du_lieu); dia.fsync()"
- kind: one-line
  body: "dia.write(sector, du_lieu); dia.fsync()"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi dia.write(sector, du_lieu) roi dia.fsync()
  requireAst:
  - kind: uses-call, target: write, min: 1
  - kind: uses-call, target: fsync, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ghi RỒI fsync — bản ghi giờ đã BỀN. NHƯNG Byte MỚI ghi được MỘT bản
— đọc lại NÓ từ đĩa (KHÔNG chỉ so SÁNH VỚI biến GỐC) thì SAO?
::::

::::reflect{#nghi-lai}
`write` LƯU vào cache, `fsync` đẩy XUỐNG chỗ bền — `read` LUÔN nhìn
cache TRƯỚC nên đọc lại NGAY sau ghi vẫn đúng, DÙ chưa fsync. Byte
mới ghi được MỘT bản ghi — đọc LẠI nó TỪ đĩa (KHÔNG so SÁNH với biến
gốc CÒN nhớ trong Python, MÀ giải mã THẲNG từ byte đọc ĐƯỢC) thì
từng trường CÓ còn đúng KHÔNG?
::::

::::checkpoint{mastery=0.8}
::::
