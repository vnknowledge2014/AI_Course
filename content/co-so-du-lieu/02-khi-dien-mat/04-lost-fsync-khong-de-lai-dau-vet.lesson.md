---
id: co-so-du-lieu.khi-dien-mat.lost-fsync-khong-de-lai-dau-vet
title: Lost fsync không để lại dấu vết
summary: "boQuaFsyncKeTiep() (lost fsync) + crash() TRẢ về hop_le=False y hệt torn write — CRC32 phát hiện được CẢ hai kiểu lỗi, không cần phân biệt. NHƯNG chỉ SAU một crash() THẬT — đọc lại NGAY trong cùng phiên (chưa crash) vẫn thấy hop_le=True, vì read() nhìn cache trước, chưa lộ lời nói dối."
locale: vi
track: co-so-du-lieu
module: khi-dien-mat
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [db.count-corrupted]
requires: [db.torn-write-detect]
concepts: [db.count-corrupted]
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
Bài `wal-la-gi` (đầu q02) cho THẤY lost fsync tạo MỤC chỉ mục MA.
NHƯNG với CRC32 (bài SAU đó), lost fsync CÓ bị phát hiện GIỐNG torn
write không?
::::

::::explain{#lost-fsync-cung-bi-bat}
Ghi MỘT bản ghi CÓ CRC32, mô phỏng `fsync()` "nói dối"
(`boQuaFsyncKeTiep`), RỒI mất điện THẬT (`crash()`):

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
        self._mat_fsync_ke_tiep = False

    def write(self, sector, data):
        self._cache[sector] = bytes(data)

    def read(self, sector):
        du_lieu = self._cache.get(sector, self._platter.get(sector))
        if du_lieu is None:
            return bytes(self.kich_thuoc_sector)
        return bytes(du_lieu)

    def fsync(self):
        if self._mat_fsync_ke_tiep:
            self._mat_fsync_ke_tiep = False
            return
        for sector, data in self._cache.items():
            self._platter[sector] = data
        self._cache.clear()

    def crash(self):
        self._cache.clear()

    def bo_qua_fsync_ke_tiep(self):
        self._mat_fsync_ke_tiep = True


dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=28)
dia.write(0, ghi_kv_crc(b'diem', b'80', 28))
dia.bo_qua_fsync_ke_tiep()
dia.fsync()
dia.crash()

print(doc_kv_crc(dia.read(0)))
```

```text title=readonly
(b'', b'', False)
```

`hop_le=False` — GIỐNG hệt torn write! CRC32 KHÔNG cần phân BIỆT
"lost fsync" hay "torn write" — CẢ hai đều LÀM `than` đọc được KHÔNG
khớp checksum đã LƯU (Ở đây, sector chưa từng CÓ dữ liệu THẬT nào
lưu bền, `than` đọc RA toàn số `0`, KHÔNG khớp `crc_luu` cũng toàn
số `0` — VÌ `zlib.crc32` của MỘT chuỗi byte `0` khác rỗng KHÔNG
BAO GIỜ bằng `0`).
::::

::::example{#chi-lo-sau-crash-that}
NHƯNG lời nói dối CHỈ lộ RA sau `crash()` — đọc NGAY sau `fsync()`
"nói dối", CHƯA crash, VẪN thấy `hop_le=True`:

```python title=readonly
dia2 = SimDisk(so_luong_sector=4, kich_thuoc_sector=28)
dia2.write(0, ghi_kv_crc(b'diem', b'80', 28))
dia2.bo_qua_fsync_ke_tiep()
dia2.fsync()

print(doc_kv_crc(dia2.read(0))[2])
```

```text title=readonly
True
```

`read()` LUÔN nhìn **cache** TRƯỚC (q00 bài 4) — cache VẪN còn
nguyên dữ liệu ĐÚNG (fsync "nói dối" KHÔNG hề chạm vào CACHE), nên
đọc lại NGAY thấy đúng. Chỉ khi `crash()` xoá SẠCH cache, sự THẬT
mới lộ ra.
::::

::::predict{#doan-doc-lai-truoc-crash commitOnce}
Byte ghi MỘT bản ghi, mô phỏng lost fsync, RỒI đọc lại HAI lần —
lần MỘT trước `crash()`, lần HAI sau:

```python
dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=28)
dia.write(0, ghi_kv_crc(b'ten', b'Byte', 28))
dia.bo_qua_fsync_ke_tiep()
dia.fsync()
truoc = doc_kv_crc(dia.read(0))[2]
dia.crash()
sau = doc_kv_crc(dia.read(0))[2]
print(truoc, sau)
```

Dòng cuối in ra gì?

:::opt{correct}
`True False`
:::

:::opt
`False False` — vì CRC32 phát hiện lời nói DỐI ngay LẬP tức, KHÔNG
cần đợi `crash()`
::why
Gần đúng ở việc bạn TIN CRC32 đủ "MẠNH" để phát hiện MỌI thứ NGAY
lập tức — MỘT kỳ vọng dễ hiểu SAU khi thấy nó bắt torn write NGAY.

Chỗ lệch: CRC32 CHỈ kiểm TRA byte ĐỌC được KHỚP checksum đã lưu HAY
không — nó KHÔNG "biết" fsync đã nói DỐI, nó chỉ THẤY những GÌ
`read()` trả VỀ. TRƯỚC `crash()`, `read()` trả về dữ liệu ĐÚNG (từ
cache) NÊN CRC khớp — `truoc = True`. CHỈ `crash()` mới xoá cache,
LÀM `read()` trả VỀ dữ liệu CŨ (chưa bao GIỜ ghi bền), khi ĐÓ CRC
mới lệch — `sau = False`.
::
:::

:::opt
`True True` — vì `crash()` chỉ ảnh HƯỞNG những ghi ĐANG chờ
`fsync()`, MÀ ở đây `fsync()` đã ĐƯỢC gọi rồi
::why
Gần đúng ở việc bạn nhớ ĐÚNG quy TẮC `crash()` (q00): CHỈ xoá cache,
KHÔNG đụng platter — MỘT quy tắc ĐÚNG, áp dụng SAI ngữ cảnh.

Chỗ lệch: `fsync()` ĐÃ được gọi, NHƯNG vì `boQuaFsyncKeTiep()` đang
BẬT, lần gọi ĐÓ LÀ no-op — dữ liệu KHÔNG hề được đẩy XUỐNG platter,
VẪN nằm nguyên TRONG cache (chưa "thật sự" fsync). `crash()` xoá
cache NHƯ mọi lần, VÀ ở đây cache LÀ nơi DUY nhất giữ dữ liệu — MẤT
sạch.
::
:::
::::

::::code{#viet_dem_ban_ghi_hong}
Viết `dem_ban_ghi_hong(dia, con_tro)` — đếm BAO nhiêu sector (TỪ `0`
tới `con_tro-1`) có checksum KHÔNG khớp.

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


def dem_ban_ghi_hong(dia, con_tro):
    dem = 0
    for i in range(con_tro):
        ___
    return dem


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


def ghi_kv_crc(khoa, gia_tri, kich_thuoc):
    than = len(khoa).to_bytes(4, 'big') + khoa + len(gia_tri).to_bytes(4, 'big') + gia_tri
    crc = zlib.crc32(than).to_bytes(4, 'big')
    return (crc + than).ljust(kich_thuoc, b'\x00')


dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=28)
dia.write(0, ghi_kv_crc(b'id', b'7', 28))
dia.fsync()
print(dem_ban_ghi_hong(dia, 1))
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


def dem_ban_ghi_hong(dia, con_tro):
    dem = 0
    for i in range(con_tro):
        if not doc_kv_crc(dia.read(i))[2]:
            dem += 1
    return dem


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


def ghi_kv_crc(khoa, gia_tri, kich_thuoc):
    than = len(khoa).to_bytes(4, 'big') + khoa + len(gia_tri).to_bytes(4, 'big') + gia_tri
    crc = zlib.crc32(than).to_bytes(4, 'big')
    return (crc + than).ljust(kich_thuoc, b'\x00')


dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=28)
dia.write(0, ghi_kv_crc(b'id', b'7', 28))
dia.fsync()
print(dem_ban_ghi_hong(dia, 1))
```

```python title=test
class SimDisk:
    def __init__(self, so_luong_sector, kich_thuoc_sector):
        self.so_luong_sector = so_luong_sector
        self.kich_thuoc_sector = kich_thuoc_sector
        self._platter = {}
        self._cache = {}
        self._mat_fsync_ke_tiep = False

    def write(self, sector, data):
        self._cache[sector] = bytes(data)

    def read(self, sector):
        du_lieu = self._cache.get(sector, self._platter.get(sector))
        if du_lieu is None:
            return bytes(self.kich_thuoc_sector)
        return bytes(du_lieu)

    def fsync(self):
        if self._mat_fsync_ke_tiep:
            self._mat_fsync_ke_tiep = False
            return
        for sector, data in self._cache.items():
            self._platter[sector] = data
        self._cache.clear()

    def crash(self):
        self._cache.clear()

    def bo_qua_fsync_ke_tiep(self):
        self._mat_fsync_ke_tiep = True


def ghi_kv_crc(khoa, gia_tri, kich_thuoc):
    than = len(khoa).to_bytes(4, 'big') + khoa + len(gia_tri).to_bytes(4, 'big') + gia_tri
    crc = zlib.crc32(than).to_bytes(4, 'big')
    return (crc + than).ljust(kich_thuoc, b'\x00')


dia = SimDisk(so_luong_sector=4, kich_thuoc_sector=28)
dia.write(0, ghi_kv_crc(b'id', b'7', 28))
dia.fsync()
assert dem_ban_ghi_hong(dia, 1) == 0, "mot ban ghi nguyen ven -- 0 hong"

dia.write(1, ghi_kv_crc(b'diem', b'80', 28))
dia.bo_qua_fsync_ke_tiep()
dia.fsync()
dia.crash()
assert dem_ban_ghi_hong(dia, 2) == 1, "them mot ban ghi lost-fsync -- 1 hong"

dia2 = SimDisk(so_luong_sector=4, kich_thuoc_sector=28)
assert dem_ban_ghi_hong(dia2, 0) == 0, "con_tro=0 -- khong dem gi"
```

:::hints
- kind: attention
  body: "Doc doc_kv_crc(dia.read(i))[2] -- neu False thi tang dem."
- kind: strategy
  body: "if not doc_kv_crc(dia.read(i))[2]: dem += 1"
- kind: one-line
  body: "if not doc_kv_crc(dia.read(i))[2]: dem += 1"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi doc_kv_crc(dia.read(i)) va tang dem neu hop_le la False
  requireAst:
  - kind: uses-name, target: i, min: 1
  - kind: uses-call, target: doc_kv_crc, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
CRC32 bắt được CẢ torn write LẪN lost fsync — MỘT tín hiệu DUY
nhất cho "đừng tin dữ liệu này". Byte đã biết ĐẾM bản ghi hỏng —
giờ dùng NÓ để bỏ QUA chúng khi PHỤC hồi.
::::

::::reflect{#nghi-lai}
CRC32 KHÔNG phân biệt torn write hay lost fsync — CẢ hai đều LÀ
`hop_le=False`, VÀ chỉ lộ RA sau một `crash()` thật, KHÔNG phải
đọc lại NGAY. Byte giờ đếm ĐƯỢC bao nhiêu bản ghi hỏng — bước tiếp
theo LÀ dùng thông tin ĐÓ để xây LẠI `chi_muc` (q01) MÀ **bỏ QUA**
những bản ghi hỏng, thay VÌ tin nhầm chúng.
::::

::::checkpoint{mastery=0.8}
::::
