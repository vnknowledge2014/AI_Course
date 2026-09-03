---
id: co-so-du-lieu.khi-dien-mat.phuc-hoi-va-tiep-tuc
title: Phục hồi và tiếp tục
summary: "Phục hồi không chỉ LÀ đọc đúng dữ liệu CŨ — cửa hàng phải TIẾP TỤC hoạt động bình thường SAU đó. phuc_hoi_va_cap_nhat khởi động lại AN TOÀN (bài trước) RỒI put() ngay — những khoá KHÔNG bị đụng tới vẫn giữ nguyên giá trị."
locale: vi
track: co-so-du-lieu
module: khi-dien-mat
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [db.recovery-continue-operating]
requires: [db.safe-restart]
concepts: [db.recovery-continue-operating]
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
`khoi_dong_lai_an_toan` (bài TRƯỚC) đọc lại ĐÚNG dữ liệu CŨ. NHƯNG
phục hồi thật sự CÓ nghĩa LÀ gì — chỉ đọc ĐÚNG một lần, HAY cửa hàng
phải TIẾP TỤC hoạt động bình thường sau ĐÓ?
::::

::::explain{#phuc-hoi-roi-ghi-tiep}
`phuc_hoi_va_cap_nhat` khởi động LẠI an toàn, RỒI gọi `put()` NGAY —
CHỨNG minh cửa hàng vẫn HOẠT động, không CHỈ "đọc được" mà THÔI:

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

    def crash(self):
        self._cache.clear()

    def danh_dau_torn_ghi(self, sector, so_byte_thanh_cong):
        self._torn_sector[sector] = so_byte_thanh_cong


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


def phuc_hoi_va_cap_nhat(dia, con_tro_lac_quan, khoa, gia_tri_moi, kich_thuoc):
    kho = khoi_dong_lai_an_toan(dia, con_tro_lac_quan)
    kho.put(khoa, gia_tri_moi, kich_thuoc)
    return kho


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
kho0 = NhatKyBitcaskCrc(dia)
kho0.put(b'diem', b'80', 28)
kho0.put(b'ten', b'Byte', 28)
dia.write(2, ghi_kv_crc(b'diem', b'95', 28))
dia.danh_dau_torn_ghi(2, 10)
dia.fsync()
dia.crash()

kho_moi = phuc_hoi_va_cap_nhat(dia, 5, b'diem', b'95', 28)
print(kho_moi.get(b'diem'), kho_moi.get(b'ten'), kho_moi.chi_muc[b'diem'])
```

```text title=readonly
b'95' b'Byte' 2
```

`kho0` ghi `diem=80` (sector `0`), `ten=Byte` (sector `1`) — CẢ hai
bền. RỒI một lần CẬP nhật `diem=95` (sector `2`) bị TORN write, mất
điện THẬT. `phuc_hoi_va_cap_nhat` khởi động lại AN toàn (thấy điểm
dừng thật LÀ `2`), RỒI `put(b'diem', b'95', 28)` NGAY — ghi VÀO
đúng sector `2` (TÁI sử dụng chỗ hỏng). `ten` KHÔNG hề bị đụng TỚI
— vẫn `b'Byte'`.
::::

::::example{#khoa-khac-khong-doi}
Chứng MINH `ten` thật sự KHÔNG đổi — đọc LẠI sau khi `phuc_hoi_va_
cap_nhat` đã chạy XONG:

```python title=readonly
print(kho_moi.get(b'ten') == b'Byte')
```

```text title=readonly
True
```

Phục hồi CHỈ xây lại `chi_muc` từ dữ liệu ĐÃ có VÀ append bản GHI
mới Ở sector TIẾP theo (`diem_dung`) — nó KHÔNG hề chạm VÀO những
sector khác (`ten` VẪN Ở sector `1`, KHÔNG ai đụng tới).
::::

::::predict{#doan-ten-con-dung-khong commitOnce}
SAU `phuc_hoi_va_cap_nhat(dia, 5, b'diem', b'95', 28)`, khoá `ten`
(KHÔNG hề được truyền VÀO hàm) có CÒN đúng `b'Byte'` không?

:::opt{correct}
Có — VẪN đúng `b'Byte'`
:::

:::opt
KHÔNG — `ten` cũng BỊ ghi đè hoặc MẤT theo, vì "phục hồi" ĐỘNG tới
TOÀN bộ đĩa
::why
Gần đúng ở việc bạn LO ngại một thao TÁC "phục hồi toàn CỤC" có
thể ẢNH hưởng tới MỌI thứ trên đĩa — một MỐI lo hợp lý VỚI các hệ
thống PHỨC tạp hơn.

Chỗ lệch: `khoi_dong_lai_an_toan` CHỈ đọc LẠI (`dia.read`), KHÔNG
hề GHI đè bất KỲ sector nào — `xay_lai_chi_muc_crc` thuần tuý XÂY
`dict` TRONG RAM. `put()` sau ĐÓ chỉ ghi VÀO đúng MỘT sector mới
(`diem_dung`), KHÔNG chạm sector CỦA `ten`.
::
:::

:::opt
Máy báo lỗi — vì gọi `put()` NGAY sau khi vừa phục hồi, CHƯA "ổn
định" xong
::why
Gần đúng ở việc bạn nghĩ TỚI một khoảng THỜI gian "chờ ổn định"
hợp lý SAU một sự CỐ.

Chỗ lệch: `NhatKyBitcaskCrc` KHÔNG có khái niệm "chưa sẵn SÀNG" —
NGAY khi `khoi_dong_lai_an_toan` trả VỀ, đối tượng ĐÃ hoàn toàn
dùng ĐƯỢC, `put()` gọi NGAY hoàn toàn hợp LỆ.
::
:::
::::

::::code{#viet_phuc_hoi_va_cap_nhat}
Hoàn thiện `phuc_hoi_va_cap_nhat(dia, con_tro_lac_quan, khoa,
gia_tri_moi, kich_thuoc)` — khởi động lại AN toàn, RỒI `put()`
ngay giá trị MỚI.

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

    def crash(self):
        self._cache.clear()

    def danh_dau_torn_ghi(self, sector, so_byte_thanh_cong):
        self._torn_sector[sector] = so_byte_thanh_cong


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


def phuc_hoi_va_cap_nhat(dia, con_tro_lac_quan, khoa, gia_tri_moi, kich_thuoc):
    kho = khoi_dong_lai_an_toan(dia, con_tro_lac_quan)
    ___
    return kho


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
kho0 = NhatKyBitcaskCrc(dia)
kho0.put(b'diem', b'80', 28)
kho0.put(b'ten', b'Byte', 28)
dia.write(2, ghi_kv_crc(b'diem', b'95', 28))
dia.danh_dau_torn_ghi(2, 10)
dia.fsync()
dia.crash()
kho_moi = phuc_hoi_va_cap_nhat(dia, 5, b'diem', b'95', 28)
print(kho_moi.get(b'diem'), kho_moi.get(b'ten'), kho_moi.chi_muc[b'diem'])
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

    def crash(self):
        self._cache.clear()

    def danh_dau_torn_ghi(self, sector, so_byte_thanh_cong):
        self._torn_sector[sector] = so_byte_thanh_cong


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


def phuc_hoi_va_cap_nhat(dia, con_tro_lac_quan, khoa, gia_tri_moi, kich_thuoc):
    kho = khoi_dong_lai_an_toan(dia, con_tro_lac_quan)
    kho.put(khoa, gia_tri_moi, kich_thuoc)
    return kho


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
kho0 = NhatKyBitcaskCrc(dia)
kho0.put(b'diem', b'80', 28)
kho0.put(b'ten', b'Byte', 28)
dia.write(2, ghi_kv_crc(b'diem', b'95', 28))
dia.danh_dau_torn_ghi(2, 10)
dia.fsync()
dia.crash()
kho_moi = phuc_hoi_va_cap_nhat(dia, 5, b'diem', b'95', 28)
print(kho_moi.get(b'diem'), kho_moi.get(b'ten'), kho_moi.chi_muc[b'diem'])
```

```python title=test
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
kho0 = NhatKyBitcaskCrc(dia)
kho0.put(b'diem', b'80', 28)
kho0.put(b'ten', b'Byte', 28)
dia.write(2, ghi_kv_crc(b'diem', b'95', 28))
dia.danh_dau_torn_ghi(2, 10)
dia.fsync()
dia.crash()
kho_moi = phuc_hoi_va_cap_nhat(dia, 5, b'diem', b'95', 28)
assert kho_moi.get(b'diem') == b'95', "gia tri moi da duoc ap dung"
assert kho_moi.get(b'ten') == b'Byte', "khoa khac khong doi"
assert kho_moi.chi_muc[b'diem'] == 2, "tai su dung dung sector hong"

dia2 = SimDisk(so_luong_sector=8, kich_thuoc_sector=28)
kho2 = phuc_hoi_va_cap_nhat(dia2, 0, b'a', b'1', 28)
assert kho2.get(b'a') == b'1', "dia rong -- put ngay van dung"
```

:::hints
- kind: attention
  body: "Goi kho.put(khoa, gia_tri_moi, kich_thuoc) sau khi da khoi_dong_lai_an_toan."
- kind: strategy
  body: "kho.put(khoa, gia_tri_moi, kich_thuoc)"
- kind: one-line
  body: "kho.put(khoa, gia_tri_moi, kich_thuoc)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi kho.put(khoa, gia_tri_moi, kich_thuoc)
  requireAst:
  - kind: uses-name, target: gia_tri_moi, min: 1
  - kind: uses-name, target: kho, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^b'95' b'Byte' 2\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Phục hồi RỒI tiếp tục — khoá KHÔNG bị đụng TỚI vẫn giữ nguyên,
khoá vừa CẬP nhật tái sử dụng đúng sector HỎNG. q02 gần XONG — chỉ
còn CHỨNG minh TẤT cả sống sót qua NHIỀU kiểu lỗi cùng LÚC.
::::

::::reflect{#nghi-lai}
Phục hồi KHÔNG chỉ LÀ đọc đúng dữ liệu CŨ — cửa hàng phải TIẾP tục
hoạt động BÌNH thường: `put()` ngay SAU khi khởi động lại VẪN hoạt
động ĐÚNG, khoá không bị đụng TỚI vẫn nguyên VẸN. Byte đã có ĐỦ:
WAL-ordering, CRC32, torn write, lost fsync, dò điểm dừng, khởi
động AN toàn, VÀ tiếp tục hoạt động. Ghép TẤT cả lại — chứng MINH
nó sống sót qua NHIỀU lỗi CÙNG lúc, được KHÔNG?
::::

::::checkpoint{mastery=0.8}
::::
