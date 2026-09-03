---
id: co-so-du-lieu.khi-dien-mat.do-diem-dung-that-cua-log
title: Dò điểm dừng thật của log
summary: "Thay vì TIN MÙ vào một con_tro nhớ được, quét TỪ sector 0 tới bản ghi ĐẦU TIÊN có checksum không khớp — đó chính là ĐIỂM DỪNG THẬT của log (mọi bản ghi TRƯỚC nó đều hợp lệ và liên tục, vì log chỉ ghi TUẦN TỰ, không có 'lỗ hổng' giữa chừng)."
locale: vi
track: co-so-du-lieu
module: khi-dien-mat
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [db.find-true-log-end]
requires: [db.count-corrupted]
concepts: [db.find-true-log-end]
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
BOSS q01 truyền `con_tro` VÀO thẳng lúc khởi động LẠI — NHƯNG sau
một lần "mất điện" thật, LÀM sao BIẾT chính XÁC `con_tro` LÀ bao
nhiêu, KHÔNG ai nói CHO Byte biết cả?
::::

::::explain{#quet-toi-ban-ghi-hong-dau-tien}
KHÔNG cần biết TRƯỚC — quét TỪ sector `0`, dừng NGAY tại bản ghi
ĐẦU tiên checksum KHÔNG khớp. Đó chính LÀ **điểm dừng thật** của
log:

```python title=readonly
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


def tim_diem_dung_that(dia, con_tro_lac_quan):
    for i in range(con_tro_lac_quan):
        if not doc_kv_crc(dia.read(i))[2]:
            return i
    return con_tro_lac_quan


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


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
dia.write(0, ghi_kv_crc(b'id', b'7', 28))
dia.fsync()
dia.write(1, ghi_kv_crc(b'ten', b'Byte', 28))
dia.fsync()
dia.write(2, ghi_kv_crc(b'tuoi', b'25', 28))
dia.bo_qua_fsync_ke_tiep()
dia.fsync()
dia.crash()

print(tim_diem_dung_that(dia, 5))
```

```text title=readonly
2
```

Byte ĐOÁN `con_tro_lac_quan=5` (KHÔNG biết chắc — "lạc QUAN" đúng
NHƯ tên gọi). `tim_diem_dung_that` quét sector `0` (hợp LỆ), `1`
(hợp lệ), `2` (lost fsync → KHÔNG hợp lệ) — DỪNG ngay, trả VỀ `2`.
KHÔNG cần đoán ĐÚNG `con_tro` thật — CHỈ cần đoán một CẬN TRÊN đủ
lớn, hàm TỰ tìm ra điểm dừng THẬT.
::::

::::example{#khong-can-hong-nao-ca}
NẾU `con_tro_lac_quan` đúng BẰNG số bản ghi THẬT (không dư, KHÔNG
hỏng), vòng lặp KHÔNG bao giờ tìm THẤY bản ghi hỏng — RƠI xuống
`return con_tro_lac_quan`:

```python title=readonly
dia2 = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
dia2.write(0, ghi_kv_crc(b'id', b'7', 28))
dia2.fsync()
dia2.write(1, ghi_kv_crc(b'ten', b'Byte', 28))
dia2.fsync()

print(tim_diem_dung_that(dia2, 2))
```

```text title=readonly
2
```

CẢ hai bản ghi hợp LỆ, `con_tro_lac_quan=2` khớp ĐÚNG số bản ghi
thật — hàm trả VỀ chính `2` (KHÔNG tìm thấy điểm dừng NÀO sớm hơn).
::::

::::predict{#doan-con-tro-lac-quan-qua-nho commitOnce}
Byte đoán `con_tro_lac_quan=1` (NHỎ hơn số bản ghi hợp LỆ thật —
CÓ `2` bản ghi hợp lệ, NHƯNG chỉ quét `1`):

```python
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
dia.write(0, ghi_kv_crc(b'id', b'7', 28))
dia.fsync()
dia.write(1, ghi_kv_crc(b'ten', b'Byte', 28))
dia.fsync()
print(tim_diem_dung_that(dia, 1))
```

Dòng cuối in ra gì?

:::opt{correct}
`1`
:::

:::opt
`2` — vì `tim_diem_dung_that` TỰ biết còn MỘT bản ghi hợp lệ NỮA
NẰM ngoài phạm vi quét, NÊN vẫn trả về SỐ thật
::why
Gần đúng ở việc bạn tin HÀM đủ "THÔNG minh" tìm ra sự THẬT dù
KHÔNG được YÊU cầu nhìn TỚI đó — một kỳ vọng dễ hiểu SAU khi thấy
NÓ tự dò được điểm dừng.

Chỗ lệch: `tim_diem_dung_that` CHỈ quét TRONG `range(con_tro_lac_
quan)` — với `con_tro_lac_quan=1`, nó CHỈ nhìn sector `0`, KHÔNG hề
BIẾT (và không THỂ biết) sector `1` tồn TẠI. Vòng lặp kết thúc mà
KHÔNG tìm thấy gì hỏng TRONG phạm vi được xem, rơi VỀ
`return con_tro_lac_quan` = `1` — bỏ SÓT một bản ghi hợp lệ THẬT.
::
:::

:::opt
Máy báo lỗi — vì `con_tro_lac_quan` KHÔNG khớp số bản ghi THẬT sự
đã ghi
::why
Gần đúng ở việc bạn LO ngại một sự KHÔNG khớp cần được BÁO ra.

Chỗ lệch: `range(1)` HOÀN toàn hợp lệ VỀ mặt cú pháp — hàm KHÔNG hề
biết "con_tro_lac_quan CÓ đúng hay không", nó chỉ ĐƠN thuần quét
đúng phạm VI được yêu cầu, KHÔNG hơn KHÔNG kém.
::
:::
::::

::::code{#viet_tim_diem_dung_that}
Hoàn thiện `tim_diem_dung_that(dia, con_tro_lac_quan)` — dừng NGAY
tại bản ghi ĐẦU tiên checksum KHÔNG khớp.

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


def tim_diem_dung_that(dia, con_tro_lac_quan):
    for i in range(con_tro_lac_quan):
        ___
    return con_tro_lac_quan


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


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
dia.write(0, ghi_kv_crc(b'id', b'7', 28))
dia.fsync()
dia.write(1, ghi_kv_crc(b'ten', b'Byte', 28))
dia.fsync()
dia.write(2, ghi_kv_crc(b'tuoi', b'25', 28))
dia.bo_qua_fsync_ke_tiep()
dia.fsync()
dia.crash()
print(tim_diem_dung_that(dia, 5))
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


def tim_diem_dung_that(dia, con_tro_lac_quan):
    for i in range(con_tro_lac_quan):
        if not doc_kv_crc(dia.read(i))[2]: return i
    return con_tro_lac_quan


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


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
dia.write(0, ghi_kv_crc(b'id', b'7', 28))
dia.fsync()
dia.write(1, ghi_kv_crc(b'ten', b'Byte', 28))
dia.fsync()
dia.write(2, ghi_kv_crc(b'tuoi', b'25', 28))
dia.bo_qua_fsync_ke_tiep()
dia.fsync()
dia.crash()
print(tim_diem_dung_that(dia, 5))
```

```python title=test
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
dia.write(0, ghi_kv_crc(b'id', b'7', 28))
dia.fsync()
dia.write(1, ghi_kv_crc(b'ten', b'Byte', 28))
dia.fsync()
dia.write(2, ghi_kv_crc(b'tuoi', b'25', 28))
dia.bo_qua_fsync_ke_tiep()
dia.fsync()
dia.crash()
assert tim_diem_dung_that(dia, 5) == 2, "dung dung tai ban ghi hong dau tien"

dia2 = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
dia2.write(0, ghi_kv_crc(b'a', b'1', 28))
dia2.fsync()
assert tim_diem_dung_that(dia2, 1) == 1, "khong co gi hong -- tra ve con_tro_lac_quan"

dia3 = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
assert tim_diem_dung_that(dia3, 0) == 0, "con_tro_lac_quan=0 -- khong quet gi"
```

:::hints
- kind: attention
  body: "Neu doc_kv_crc(dia.read(i))[2] la False thi return i ngay -- mot dong."
- kind: strategy
  body: "if not doc_kv_crc(dia.read(i))[2]: return i"
- kind: one-line
  body: "if not doc_kv_crc(dia.read(i))[2]: return i"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai return i ngay khi doc_kv_crc(dia.read(i))[2] la False
  requireAst:
  - kind: uses-name, target: i, min: 2
  - kind: uses-call, target: doc_kv_crc, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^2\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Điểm dừng thật — dò ĐƯỢC mà KHÔNG cần ai nói CHO Byte biết TRƯỚC.
Byte đã CÓ mọi mảnh: CRC32, dò điểm dừng — GHÉP chúng vào
`khoi_dong_lai` (q01 BOSS) để phục hồi ĐÚNG sau MỌI kiểu mất điện.
::::

::::reflect{#nghi-lai}
Dò điểm dừng THẬT — quét TỚI bản ghi đầu TIÊN hỏng, KHÔNG cần biết
TRƯỚC `con_tro`. Byte giờ có ĐỦ mọi mảnh của q02: WAL-ordering, CRC32,
torn write, lost fsync, VÀ dò điểm dừng. Ghép TẤT cả VÀO
`khoi_dong_lai` (q01 BOSS) — thay VÌ tin MÙ một `con_tro` truyền
VÀO, để `khoi_dong_lai` TỰ dò ra sự THẬT — được KHÔNG?
::::

::::checkpoint{mastery=0.8}
::::
