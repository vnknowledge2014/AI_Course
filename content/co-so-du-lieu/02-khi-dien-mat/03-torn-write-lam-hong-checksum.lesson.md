---
id: co-so-du-lieu.khi-dien-mat.torn-write-lam-hong-checksum
title: Torn write làm hỏng checksum
summary: "SimDisk.danhDauTornGhi(sector, N) mô phỏng THẬT một lần ghi dở dang (mất điện giữa chừng khi fsync đang đẩy sector xuống platter) — chỉ N byte đầu tới nơi, phần còn lại vẫn là dữ liệu CŨ. CRC32 (bài 2) phát hiện ra ngay khi N cắt vào phần dữ liệu THẬT — cắt vào phần đệm thừa thì vô hại."
locale: vi
track: co-so-du-lieu
module: khi-dien-mat
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [db.torn-write-detect]
requires: [db.crc32-integrity]
concepts: [db.torn-write-detect]
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
Bài TRƯỚC, Byte tự "lật bit" MỘT byte để giả LẬP hỏng. NHƯNG trong
THỰC tế, điều GÌ thật sự làm MỘT byte đổi giữa lúc GHI?
::::

::::explain{#torn-write-that}
**Torn write** — mất điện ĐÚNG lúc `fsync()` đang đẩy MỘT sector
xuống platter: CHỈ một PHẦN byte tới nơi, PHẦN còn lại vẫn LÀ dữ
liệu CŨ (mô phỏng THẬT bằng `SimDisk.danhDauTornGhi(sector,
soByteThanhCong)`, gói `@byte/simdisk` v2):

```python title=readonly
import zlib


def ghi_kv_crc(khoa, gia_tri, kich_thuoc):
    than = len(khoa).to_bytes(4, 'big') + khoa + len(gia_tri).to_bytes(4, 'big') + gia_tri
    crc = zlib.crc32(than).to_bytes(4, 'big')
    return (crc + than).ljust(kich_thuoc, b'\x00')


def doc_kv_crc(sector_bytes):
    crc_luu = sector_bytes[0:4]
    do_dai_khoa = int.from_bytes(sector_bytes[4:8], 'big')
    khoa = sector_bytes[8:8 + do_dai_khoa]
    vi_tri = 8 + do_dai_khoa
    do_dai_gia_tri = int.from_bytes(sector_bytes[vi_tri:vi_tri + 4], 'big')
    gia_tri = sector_bytes[vi_tri + 4:vi_tri + 4 + do_dai_gia_tri]
    than = sector_bytes[4:vi_tri + 4 + do_dai_gia_tri]
    hop_le = zlib.crc32(than).to_bytes(4, 'big') == crc_luu
    return (khoa, gia_tri, hop_le)


class SimDisk:
    def __init__(self, so_luong_sector, kich_thuoc_sector):
        self.so_luong_sector = so_luong_sector
        self.kich_thuoc_sector = kich_thuoc_sector
        self._platter = {}
        self._cache = {}
        self._torn_sector = {}

    def write(self, sector, data):
        self._cache[sector] = bytes(data)

    def read(self, sector):
        du_lieu = self._cache.get(sector, self._platter.get(sector))
        if du_lieu is None:
            return bytes(self.kich_thuoc_sector)
        return bytes(du_lieu)

    def fsync(self):
        for sector, data in self._cache.items():
            so_byte = self._torn_sector.pop(sector, None)
            if so_byte is None:
                self._platter[sector] = data
                continue
            cu = self._platter.get(sector, bytes(self.kich_thuoc_sector))
            ghi_duoc = bytearray(self.kich_thuoc_sector)
            ghi_duoc[0:so_byte] = data[0:so_byte]
            ghi_duoc[so_byte:] = cu[so_byte:]
            self._platter[sector] = bytes(ghi_duoc)
        self._cache.clear()

    def danh_dau_torn_ghi(self, sector, so_byte_thanh_cong):
        self._torn_sector[sector] = so_byte_thanh_cong


dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=28)
dia.write(0, ghi_kv_crc(b'id', b'7', 28))
dia.danh_dau_torn_ghi(0, 14)   # CHỈ 14/28 byte tới noi
dia.fsync()

print(doc_kv_crc(dia.read(0)))
```

```text title=readonly
(b'id', b'\x00', False)
```

`danh_dau_torn_ghi(0, 14)` NÓI: lần `fsync()` tiếp THEO, sector `0`
CHỈ ghi thành CÔNG `14` byte đầu — PHẦN sau (byte `14`-`27`) VẪN LÀ
`0` (sector CHƯA từng ghi trước ĐÓ). `4` byte header CRC + `10` byte
đầu thân TỚI nơi, phần CÒN lại (giá trị THẬT) mất — `hop_le=False`.
::::

::::example{#torn-vao-phan-dem}
Bản ghi `('ten', 'Byte')` VỚI `kich_thuoc_sector=28` CHỈ dùng `19`
byte THẬT (crc+than), CÒN lại LÀ đệm `0` thừa. Torn write CẮT vào
đúng PHẦN đệm — KHÔNG ảnh hưởng gì:

```python title=readonly
dia2 = SimDisk(so_luong_sector=4, kich_thuoc_sector=28)
rec2 = ghi_kv_crc(b'ten', b'Byte', 28)
dia2.write(0, rec2)
dia2.danh_dau_torn_ghi(0, 19)   # CHỈ giu lai 9 byte dem thua (19..27)
dia2.fsync()

print(doc_kv_crc(dia2.read(0))[2])
```

```text title=readonly
True
```

`so_byte_thanh_cong=19` NGHĨA LÀ `19` byte ĐẦU (đúng TOÀN bộ crc +
than THẬT) tới nơi, CHỈ `9` byte đệm THỪA (vốn ĐÃ là `0`, KHÔNG đổi
gì) bị "torn" — KHÔNG có gì THẬT sự mất, `hop_le` VẪN `True`. Torn
write CHỈ nguy hiểm khi cắt VÀO phần dữ liệu THẬT.
::::

::::predict{#doan-torn-vao-byte-cuoi-that commitOnce}
Byte đánh DẤU torn write với `so_byte_thanh_cong = 18` (THIẾU đúng
`1` byte CUỐI của phần dữ liệu THẬT — chữ `'e'` cuối cùng của giá
trị `'Byte'`, KHÔNG phải phần đệm):

```python
dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=28)
rec = ghi_kv_crc(b'ten', b'Byte', 28)
dia.write(0, rec)
dia.danh_dau_torn_ghi(0, 18)
dia.fsync()
print(doc_kv_crc(dia.read(0))[2])
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — giống HỆT ví dụ trước (`so_byte_thanh_cong=19`), vì đều
"THIẾU vài byte cuối"
::why
Gần đúng ở việc bạn NHỚ đúng ví dụ VỪA thấy — torn write cắt VÀO
phần đệm LÀ vô hại — VÀ áp dụng NÓ vào tình huống MỚI.

Chỗ lệch: `18` KHÔNG nằm trong phần ĐỆM (đệm bắt đầu TỪ byte `19`)
— nó CẮT ĐÚNG VÀO byte cuối cùng của phần dữ liệu THẬT (chữ `'e'`
cuối `'Byte'`). Byte ĐÓ đổi từ `'e'` VỀ `0` (dữ liệu CŨ) — checksum
tính lại KHÁC checksum đã lưu.
::
:::

:::opt
Máy báo lỗi — vì `18` KHÔNG chia hết cho kích thước MỘT trường NÀO
trong bản ghi
::why
Gần đúng ở việc bạn TÌM một quy LUẬT số học ẩn ĐẰNG sau — một PHẢN
xạ hợp lý khi thấy nhiều CON số `4`, `8`.

Chỗ lệch: `danh_dau_torn_ghi` KHÔNG quan tâm "ranh giới trường" NÀO
cả — nó chỉ ĐƠN thuần cắt Ở đúng vị TRÍ byte được TRUYỀN vào, BẤT
kể vị trí đó CÓ "đẹp" về mặt SỐ học hay không.
::
:::
::::

::::code{#viet_doc_neu_hop_le}
Viết `doc_neu_hop_le(dia, sector)` — đọc VÀ giải mã một sector, trả
VỀ giá trị NẾU checksum khớp, `None` NẾU không.

```python title=starter
import zlib


def doc_kv_crc(sector_bytes):
    crc_luu = sector_bytes[0:4]
    do_dai_khoa = int.from_bytes(sector_bytes[4:8], 'big')
    khoa = sector_bytes[8:8 + do_dai_khoa]
    vi_tri = 8 + do_dai_khoa
    do_dai_gia_tri = int.from_bytes(sector_bytes[vi_tri:vi_tri + 4], 'big')
    gia_tri = sector_bytes[vi_tri + 4:vi_tri + 4 + do_dai_gia_tri]
    than = sector_bytes[4:vi_tri + 4 + do_dai_gia_tri]
    hop_le = zlib.crc32(than).to_bytes(4, 'big') == crc_luu
    return (khoa, gia_tri, hop_le)


def doc_neu_hop_le(dia, sector):
    khoa, gia_tri, hop_le = doc_kv_crc(dia.read(sector))
    ___


def ghi_kv_crc(khoa, gia_tri, kich_thuoc):
    than = len(khoa).to_bytes(4, 'big') + khoa + len(gia_tri).to_bytes(4, 'big') + gia_tri
    crc = zlib.crc32(than).to_bytes(4, 'big')
    return (crc + than).ljust(kich_thuoc, b'\x00')


class SimDisk:
    def __init__(self, so_luong_sector, kich_thuoc_sector):
        self.so_luong_sector = so_luong_sector
        self.kich_thuoc_sector = kich_thuoc_sector
        self._platter = {}
        self._cache = {}
        self._torn_sector = {}

    def write(self, sector, data):
        self._cache[sector] = bytes(data)

    def read(self, sector):
        du_lieu = self._cache.get(sector, self._platter.get(sector))
        if du_lieu is None:
            return bytes(self.kich_thuoc_sector)
        return bytes(du_lieu)

    def fsync(self):
        for sector, data in self._cache.items():
            so_byte = self._torn_sector.pop(sector, None)
            if so_byte is None:
                self._platter[sector] = data
                continue
            cu = self._platter.get(sector, bytes(self.kich_thuoc_sector))
            ghi_duoc = bytearray(self.kich_thuoc_sector)
            ghi_duoc[0:so_byte] = data[0:so_byte]
            ghi_duoc[so_byte:] = cu[so_byte:]
            self._platter[sector] = bytes(ghi_duoc)
        self._cache.clear()

    def danh_dau_torn_ghi(self, sector, so_byte_thanh_cong):
        self._torn_sector[sector] = so_byte_thanh_cong


dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=28)
dia.write(0, ghi_kv_crc(b'id', b'7', 28))
dia.fsync()
print(doc_neu_hop_le(dia, 0))
```

```python title=solution
import zlib


def doc_kv_crc(sector_bytes):
    crc_luu = sector_bytes[0:4]
    do_dai_khoa = int.from_bytes(sector_bytes[4:8], 'big')
    khoa = sector_bytes[8:8 + do_dai_khoa]
    vi_tri = 8 + do_dai_khoa
    do_dai_gia_tri = int.from_bytes(sector_bytes[vi_tri:vi_tri + 4], 'big')
    gia_tri = sector_bytes[vi_tri + 4:vi_tri + 4 + do_dai_gia_tri]
    than = sector_bytes[4:vi_tri + 4 + do_dai_gia_tri]
    hop_le = zlib.crc32(than).to_bytes(4, 'big') == crc_luu
    return (khoa, gia_tri, hop_le)


def doc_neu_hop_le(dia, sector):
    khoa, gia_tri, hop_le = doc_kv_crc(dia.read(sector))
    return gia_tri if hop_le else None


def ghi_kv_crc(khoa, gia_tri, kich_thuoc):
    than = len(khoa).to_bytes(4, 'big') + khoa + len(gia_tri).to_bytes(4, 'big') + gia_tri
    crc = zlib.crc32(than).to_bytes(4, 'big')
    return (crc + than).ljust(kich_thuoc, b'\x00')


class SimDisk:
    def __init__(self, so_luong_sector, kich_thuoc_sector):
        self.so_luong_sector = so_luong_sector
        self.kich_thuoc_sector = kich_thuoc_sector
        self._platter = {}
        self._cache = {}
        self._torn_sector = {}

    def write(self, sector, data):
        self._cache[sector] = bytes(data)

    def read(self, sector):
        du_lieu = self._cache.get(sector, self._platter.get(sector))
        if du_lieu is None:
            return bytes(self.kich_thuoc_sector)
        return bytes(du_lieu)

    def fsync(self):
        for sector, data in self._cache.items():
            so_byte = self._torn_sector.pop(sector, None)
            if so_byte is None:
                self._platter[sector] = data
                continue
            cu = self._platter.get(sector, bytes(self.kich_thuoc_sector))
            ghi_duoc = bytearray(self.kich_thuoc_sector)
            ghi_duoc[0:so_byte] = data[0:so_byte]
            ghi_duoc[so_byte:] = cu[so_byte:]
            self._platter[sector] = bytes(ghi_duoc)
        self._cache.clear()

    def danh_dau_torn_ghi(self, sector, so_byte_thanh_cong):
        self._torn_sector[sector] = so_byte_thanh_cong


dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=28)
dia.write(0, ghi_kv_crc(b'id', b'7', 28))
dia.fsync()
print(doc_neu_hop_le(dia, 0))
```

```python title=test
dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=28)
dia.write(0, ghi_kv_crc(b'id', b'7', 28))
dia.fsync()
assert doc_neu_hop_le(dia, 0) == b'7', "ban ghi nguyen ven -- doc dung"

dia.write(1, ghi_kv_crc(b'ten', b'Byte', 28))
dia.danh_dau_torn_ghi(1, 14)
dia.fsync()
assert doc_neu_hop_le(dia, 1) is None, "torn write vao phan that -- None"

dia.write(2, ghi_kv_crc(b'tuoi', b'25', 28))
dia.danh_dau_torn_ghi(2, 28)
dia.fsync()
assert doc_neu_hop_le(dia, 2) == b'25', "torn du 100% -- van dung"
```

:::hints
- kind: attention
  body: "Neu hop_le thi tra ve gia_tri, khong thi tra ve None -- mot dong."
- kind: strategy
  body: "return gia_tri if hop_le else None"
- kind: one-line
  body: "return gia_tri if hop_le else None"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tra ve gia_tri neu hop_le, nguoc lai tra ve None
  requireAst:
  - kind: uses-name, target: gia_tri, min: 4
  - kind: uses-name, target: hop_le, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^b'7'\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Torn write THẬT — CRC32 bắt được NGAY khi cắt VÀO dữ liệu THẬT, trả
VỀ `None` thay VÌ dữ liệu sai. NHƯNG một bản ghi hỏng đơn LẺ khác
HẲN một bản ghi biến MẤT hoàn toàn — điều GÌ xảy ra khi `fsync()`
"nói dối" (bài `wal-la-gi`) CHỨ không phải chỉ ghi dở?
::::

::::reflect{#nghi-lai}
Torn write THẬT (`danhDauTornGhi`) — MẤT điện giữa CHỪNG một lần
ghi, CHỈ một phần byte tới NƠI. CRC32 bắt ĐƯỢC khi cắt VÀO dữ liệu
THẬT, VÔ hại khi chỉ cắt vào PHẦN đệm thừa. NHƯNG "lost fsync" (bài
`wal-la-gi` ĐÃ nhắc) LÀ một kiểu lỗi HOÀN toàn khác — `fsync()` "nói
dối" đã BỀN mà thật ra CHƯA đẩy gì cả. CRC32 CÓ bắt được kiểu lỗi
NÀY không?
::::

::::checkpoint{mastery=0.8}
::::
