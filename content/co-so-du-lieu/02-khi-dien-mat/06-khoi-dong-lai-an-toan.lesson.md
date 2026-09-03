---
id: co-so-du-lieu.khi-dien-mat.khoi-dong-lai-an-toan
title: Khởi động lại an toàn
summary: "Thay 'tin mù con_tro' (q01 BOSS) bằng tim_diem_dung_that (bài 5): khoi_dong_lai_an_toan tự dò điểm dừng thật RỒI mới xây lại chi_muc. Khác biệt CHỈ lộ ra ở lần put() KẾ TIẾP — bản an toàn tái sử dụng đúng sector đã hỏng, bản tin-mù bỏ phí nó và nhảy quá xa."
locale: vi
track: co-so-du-lieu
module: khi-dien-mat
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [db.safe-restart]
requires: [db.find-true-log-end]
concepts: [db.safe-restart]
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
`khoi_dong_lai` (BOSS q01) nhận THẲNG `con_tro` — VÀ tin nó HOÀN
toàn. NẾU con số ĐÓ sai (sau một lần "mất điện" THẬT, KHÔNG ai nhớ
chính XÁC), điều GÌ xảy ra?
::::

::::explain{#ghep-tim-diem-dung-that}
Thay VÌ tin thẳng, `khoi_dong_lai_an_toan` gọi `tim_diem_dung_that`
(bài TRƯỚC) TRƯỚC — CHỈ xây `chi_muc` từ đúng phần log ĐÃ xác nhận
hợp LỆ:

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


def ghi_kv_crc(khoa, gia_tri, kich_thuoc):
    than = len(khoa).to_bytes(4, 'big') + khoa + len(gia_tri).to_bytes(4, 'big') + gia_tri
    crc = zlib.crc32(than).to_bytes(4, 'big')
    return (crc + than).ljust(kich_thuoc, b'\x00')


def tim_diem_dung_that(dia, con_tro_lac_quan):
    for i in range(con_tro_lac_quan):
        if not doc_kv_crc(dia.read(i))[2]:
            return i
    return con_tro_lac_quan


def xay_lai_chi_muc_crc(dia, diem_dung):
    chi_muc_moi = {}
    for i in range(diem_dung):
        khoa, gia_tri, _ = doc_kv_crc(dia.read(i))
        chi_muc_moi[khoa] = i
    return chi_muc_moi


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


class NhatKyBitcaskCrc:
    def __init__(self, dia):
        self.dia = dia
        self.chi_muc = {}
        self.con_tro = 0

    def put(self, khoa, gia_tri, kich_thuoc):
        du_lieu = ghi_kv_crc(khoa, gia_tri, kich_thuoc)
        self.dia.write(self.con_tro, du_lieu)
        self.dia.fsync()
        self.chi_muc[khoa] = self.con_tro
        self.con_tro += 1

    def get(self, khoa):
        if khoa not in self.chi_muc:
            return None
        sector = self.chi_muc[khoa]
        _, gia_tri, hop_le = doc_kv_crc(self.dia.read(sector))
        return gia_tri if hop_le else None


def khoi_dong_lai_an_toan(dia, con_tro_lac_quan):
    diem_dung = tim_diem_dung_that(dia, con_tro_lac_quan)
    kho_moi = NhatKyBitcaskCrc(dia)
    kho_moi.chi_muc = xay_lai_chi_muc_crc(dia, diem_dung)
    kho_moi.con_tro = diem_dung
    return kho_moi


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
kho = NhatKyBitcaskCrc(dia)
kho.put(b'id', b'7', 28)
kho.put(b'ten', b'Byte', 28)
dia.bo_qua_fsync_ke_tiep()
kho.put(b'diem', b'80', 28)
dia.crash()

kho_moi = khoi_dong_lai_an_toan(dia, 3)
print(kho_moi.con_tro)
```

```text title=readonly
2
```

`NhatKyBitcaskCrc` LÀ `NhatKyBitcask` (q01) NÂNG cấp — `put`/`get`
dùng `ghi_kv_crc`/`doc_kv_crc` (CÓ checksum) thay VÌ `ghi_kv`/`doc_kv`
CŨ. Byte đoán `con_tro_lac_quan=3` (LẠC quan — bản GHI thứ ba đã
`put()` NHƯNG lost fsync). `khoi_dong_lai_an_toan` KHÔNG tin `3` —
NÓ gọi `tim_diem_dung_that` TRƯỚC, tìm ĐÚNG điểm dừng thật LÀ `2`.
::::

::::example{#khac-biet-lo-o-lan-ghi-sau}
`kho_moi.con_tro=2` — LẦN `put()` TIẾP theo sẽ ghi VÀO đúng sector
`2` (TÁI sử dụng chỗ đã HỎNG), KHÔNG nhảy quá:

```python title=readonly
kho_moi.put(b'mau', b'xanh', 28)
print(kho_moi.chi_muc[b'mau'])
```

```text title=readonly
2
```

`chi_muc[b'mau'] = 2` — ĐÚNG sector từng chứa bản ghi HỎNG (`diem`
lost-fsync), giờ được TÁI sử dụng SẠCH sẽ. NẾU tin thẳng
`con_tro_lac_quan=3` (KHÔNG dò LẠI), lần `put()` SAU sẽ nhảy TỚI
sector `3`, bỏ PHÍ sector `2` MÃI mãi.
::::

::::predict{#doan-tin-thang-con-tro-lac-quan commitOnce}
So sánh: MỘT bản "an toàn" (dò LẠI), MỘT bản "lạc quan" (tin THẲNG
`con_tro_lac_quan=3`, KHÔNG gọi `tim_diem_dung_that`) — CẢ hai đều
`put(b'mau', b'xanh', 28)` NGAY sau khi khởi động LẠI. `chi_muc[b'mau']`
của bản **an TOÀN** là bao NHIÊU?

:::opt{correct}
`2`
:::

:::opt
`3` — vì CẢ hai cách khởi động lại đều bắt đầu ghi TIẾP từ đúng vị
trí `con_tro_lac_quan` truyền VÀO
::why
Gần đúng ở việc bạn nghĩ CẢ hai cách "khởi động LẠI" đều xử LÝ
GIỐNG nhau — MỘT giả định hợp lý NẾU chưa phân biệt "an toàn" VÀ
"lạc quan".

Chỗ lệch: bản **an toàn** gọi `tim_diem_dung_that` TRƯỚC, tìm RA
điểm dừng thật LÀ `2` (KHÔNG phải `3`) — `con_tro` của NÓ LÀ `2`,
`put()` tiếp THEO ghi VÀO sector `2`. CHỈ bản "lạc quan" (tin thẳng
`3`, KHÔNG dò lại) MỚI nhảy TỚI sector `3`.
::
:::

:::opt
`4` — vì `SimDisk` TỰ động đánh dấu sector `2` LÀ "đã hỏng", buộc
LẦN ghi tiếp theo phải nhảy QUA nó
::why
Gần đúng ở việc bạn nghĩ TỚI một cơ chế "TRÁNH sector hỏng" hợp lý
CHO một hệ thống lưu trữ THẬT.

Chỗ lệch: `SimDisk` KHÔNG hề "nhớ" sector NÀO từng hỏng — `con_tro`
CHỈ LÀ một con đếm ĐƠN giản TRONG `NhatKyBitcaskCrc`, KHÔNG có bảng
đánh dấu NÀO cả. Sector `2` hoàn TOÀN "sạch" TRỞ lại một khi
`fsync()` mới ghi ĐÈ lên nó.
::
:::
::::

::::code{#viet_khoi_dong_lai_an_toan}
Hoàn thiện `khoi_dong_lai_an_toan(dia, con_tro_lac_quan)` — dò
điểm dừng thật TRƯỚC khi xây lại `chi_muc`.

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


def ghi_kv_crc(khoa, gia_tri, kich_thuoc):
    than = len(khoa).to_bytes(4, 'big') + khoa + len(gia_tri).to_bytes(4, 'big') + gia_tri
    crc = zlib.crc32(than).to_bytes(4, 'big')
    return (crc + than).ljust(kich_thuoc, b'\x00')


def tim_diem_dung_that(dia, con_tro_lac_quan):
    for i in range(con_tro_lac_quan):
        if not doc_kv_crc(dia.read(i))[2]:
            return i
    return con_tro_lac_quan


def xay_lai_chi_muc_crc(dia, diem_dung):
    chi_muc_moi = {}
    for i in range(diem_dung):
        khoa, gia_tri, _ = doc_kv_crc(dia.read(i))
        chi_muc_moi[khoa] = i
    return chi_muc_moi


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


class NhatKyBitcaskCrc:
    def __init__(self, dia):
        self.dia = dia
        self.chi_muc = {}
        self.con_tro = 0

    def put(self, khoa, gia_tri, kich_thuoc):
        du_lieu = ghi_kv_crc(khoa, gia_tri, kich_thuoc)
        self.dia.write(self.con_tro, du_lieu)
        self.dia.fsync()
        self.chi_muc[khoa] = self.con_tro
        self.con_tro += 1

    def get(self, khoa):
        if khoa not in self.chi_muc:
            return None
        sector = self.chi_muc[khoa]
        _, gia_tri, hop_le = doc_kv_crc(self.dia.read(sector))
        return gia_tri if hop_le else None


def khoi_dong_lai_an_toan(dia, con_tro_lac_quan):
    ___
    kho_moi = NhatKyBitcaskCrc(dia)
    kho_moi.chi_muc = xay_lai_chi_muc_crc(dia, diem_dung)
    kho_moi.con_tro = diem_dung
    return kho_moi


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
kho = NhatKyBitcaskCrc(dia)
kho.put(b'id', b'7', 28)
kho.put(b'ten', b'Byte', 28)
dia.bo_qua_fsync_ke_tiep()
kho.put(b'diem', b'80', 28)
dia.crash()
kho_moi = khoi_dong_lai_an_toan(dia, 3)
print(kho_moi.con_tro)
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


def ghi_kv_crc(khoa, gia_tri, kich_thuoc):
    than = len(khoa).to_bytes(4, 'big') + khoa + len(gia_tri).to_bytes(4, 'big') + gia_tri
    crc = zlib.crc32(than).to_bytes(4, 'big')
    return (crc + than).ljust(kich_thuoc, b'\x00')


def tim_diem_dung_that(dia, con_tro_lac_quan):
    for i in range(con_tro_lac_quan):
        if not doc_kv_crc(dia.read(i))[2]:
            return i
    return con_tro_lac_quan


def xay_lai_chi_muc_crc(dia, diem_dung):
    chi_muc_moi = {}
    for i in range(diem_dung):
        khoa, gia_tri, _ = doc_kv_crc(dia.read(i))
        chi_muc_moi[khoa] = i
    return chi_muc_moi


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


class NhatKyBitcaskCrc:
    def __init__(self, dia):
        self.dia = dia
        self.chi_muc = {}
        self.con_tro = 0

    def put(self, khoa, gia_tri, kich_thuoc):
        du_lieu = ghi_kv_crc(khoa, gia_tri, kich_thuoc)
        self.dia.write(self.con_tro, du_lieu)
        self.dia.fsync()
        self.chi_muc[khoa] = self.con_tro
        self.con_tro += 1

    def get(self, khoa):
        if khoa not in self.chi_muc:
            return None
        sector = self.chi_muc[khoa]
        _, gia_tri, hop_le = doc_kv_crc(self.dia.read(sector))
        return gia_tri if hop_le else None


def khoi_dong_lai_an_toan(dia, con_tro_lac_quan):
    diem_dung = tim_diem_dung_that(dia, con_tro_lac_quan)
    kho_moi = NhatKyBitcaskCrc(dia)
    kho_moi.chi_muc = xay_lai_chi_muc_crc(dia, diem_dung)
    kho_moi.con_tro = diem_dung
    return kho_moi


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
kho = NhatKyBitcaskCrc(dia)
kho.put(b'id', b'7', 28)
kho.put(b'ten', b'Byte', 28)
dia.bo_qua_fsync_ke_tiep()
kho.put(b'diem', b'80', 28)
dia.crash()
kho_moi = khoi_dong_lai_an_toan(dia, 3)
print(kho_moi.con_tro)
```

```python title=test
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
kho = NhatKyBitcaskCrc(dia)
kho.put(b'id', b'7', 28)
kho.put(b'ten', b'Byte', 28)
dia.bo_qua_fsync_ke_tiep()
kho.put(b'diem', b'80', 28)
dia.crash()
kho_moi = khoi_dong_lai_an_toan(dia, 3)
assert kho_moi.con_tro == 2, "diem dung that la 2, khong phai 3"
assert kho_moi.get(b'id') == b'7', "ban ghi song sot dau tien"
assert kho_moi.get(b'ten') == b'Byte', "ban ghi song sot thu hai"
assert kho_moi.get(b'diem') is None, "ban ghi lost-fsync khong con"

kho_moi.put(b'mau', b'xanh', 28)
assert kho_moi.chi_muc[b'mau'] == 2, "tai su dung dung sector da hong"

dia2 = SimDisk(so_luong_sector=4, kich_thuoc_sector=28)
kho2 = khoi_dong_lai_an_toan(dia2, 0)
assert kho2.con_tro == 0, "dia rong -- con_tro 0"
```

:::hints
- kind: attention
  body: "Goi tim_diem_dung_that(dia, con_tro_lac_quan), gan vao bien diem_dung."
- kind: strategy
  body: "diem_dung = tim_diem_dung_that(dia, con_tro_lac_quan)"
- kind: one-line
  body: "diem_dung = tim_diem_dung_that(dia, con_tro_lac_quan)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi tim_diem_dung_that(dia, con_tro_lac_quan) va gan vao diem_dung
  requireAst:
  - kind: uses-call, target: tim_diem_dung_that, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^2\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khởi động lại AN toàn — dò điểm dừng thật TRƯỚC, KHÔNG tin mù một
con số. NHƯNG phục hồi KHÔNG chỉ LÀ đọc đúng — cửa hàng CÓ tiếp tục
HOẠT động bình thường SAU đó không?
::::

::::reflect{#nghi-lai}
`khoi_dong_lai_an_toan` thay "tin mù `con_tro`" bằng "dò điểm dừng
thật" — khác biệt CHỈ lộ Ở lần `put()` TIẾP theo: bản an toàn tái
sử dụng ĐÚNG sector đã hỏng, KHÔNG bỏ phí. Byte đã CÓ một cách khởi
động lại ĐÚNG — nhưng phục hồi xong RỒI, cửa hàng CÓ tiếp tục nhận
`put()`/`get()` bình thường NHƯ chưa từng có gì xảy RA không?
::::

::::checkpoint{mastery=0.8}
::::
