---
id: co-so-du-lieu.khi-dien-mat.boss-song-sot-qua-mat-dien
title: "BOSS — Sống sót qua mất điện"
summary: "Ghép TRỌN q02: ba bản ghi bền, MỘT lần cập nhật bị torn write, crash() thật — khoi_dong_lai_an_toan (dò điểm dừng thật, bỏ qua bản ghi hỏng) phục hồi đúng CẢ ba giá trị CŨ, rồi put() tiếp tục hoạt động bình thường, tái sử dụng đúng sector đã hỏng."
locale: vi
track: co-so-du-lieu
module: khi-dien-mat
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.recovery-continue-operating]
concepts: [db.boss-q02]
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
WAL-ordering, CRC32, torn write, lost fsync, dò điểm dừng, khởi
động AN toàn, tiếp tục hoạt ĐỘNG — Byte đã có ĐỦ mọi mảnh của q02.
Ghép TẤT cả lại thành MỘT câu chuyện DUY nhất.
::::

::::explain{#ba-ban-ghi-mot-loi}
BA bản ghi bền HOÀN toàn, RỒI một lần CẬP nhật bị torn write GIỮA
chừng, RỒI mất điện THẬT:

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


dia = SimDisk(so_luong_sector=10, kich_thuoc_sector=28)
kho = NhatKyBitcaskCrc(dia)
kho.put(b'diem', b'80', 28)
kho.put(b'ten', b'Byte', 28)
kho.put(b'tuoi', b'25', 28)
dia.write(3, ghi_kv_crc(b'diem', b'95', 28))
dia.danh_dau_torn_ghi(3, 10)
dia.fsync()
dia.crash()

kho_phuc_hoi = khoi_dong_lai_an_toan(dia, 10)
print(kho_phuc_hoi.con_tro, kho_phuc_hoi.get(b'diem'), kho_phuc_hoi.get(b'ten'), kho_phuc_hoi.get(b'tuoi'))
```

```text title=readonly
3 b'80' b'Byte' b'25'
```

`con_tro_lac_quan=10` (rất "LẠC quan" — GẤP hơn ba lần số bản ghi
THẬT) — `khoi_dong_lai_an_toan` KHÔNG hề bối rối, `tim_diem_dung_
that` dừng ĐÚNG tại sector `3` (bản CẬP nhật `diem` bị torn write).
CẢ ba giá trị GỐC (`diem=80`, `ten=Byte`, `tuoi=25`) đều PHỤC hồi
đúng — bản cập NHẬT hỏng ĐƠN giản "chưa từng xảy RA" theo góc nhìn
của kho ĐÃ phục hồi.
::::

::::example{#tiep-tuc-hoat-dong}
Cửa hàng đã phục HỒI vẫn `put()` bình THƯỜNG — tái SỬ dụng đúng
sector `3` (chỗ TỪNG chứa bản cập NHẬT hỏng):

```python title=readonly
kho_phuc_hoi.put(b'diem', b'95', 28)

print(kho_phuc_hoi.chi_muc[b'diem'], kho_phuc_hoi.get(b'diem'))
```

```text title=readonly
3 b'95'
```

`chi_muc[b'diem']=3` — ĐÚNG sector từng TORN write, giờ ghi SẠCH
sẽ, thành CÔNG hoàn toàn. Cửa hàng KHÔNG hề "nhớ" sector `3` từng
CÓ vấn đề — nó chỉ đơn THUẦN là sector TIẾP theo trống.
::::

::::predict{#doan-con-tro-sau-phuc-hoi commitOnce}
NGAY sau `khoi_dong_lai_an_toan(dia, 10)` (TRƯỚC khi `put()` thêm
GÌ), `kho_phuc_hoi.con_tro` LÀ bao nhiêu?

:::opt{correct}
`3`
:::

:::opt
`10` — vì `khoi_dong_lai_an_toan` giữ NGUYÊN `con_tro_lac_quan`
truyền VÀO làm con trỏ MỚI
::why
Gần đúng ở việc bạn nghĩ HÀM "tin tưởng" tham SỐ được truyền VÀO —
một giả định hợp LÝ nếu chưa phân biệt "LẠC quan" VÀ "đã xác nhận".

Chỗ lệch: `khoi_dong_lai_an_toan` KHÔNG hề giữ NGUYÊN
`con_tro_lac_quan` — nó gọi `tim_diem_dung_that` để TÌM điểm dừng
THẬT (`3`, nơi bản ghi ĐẦU tiên hỏng), RỒI gán `kho_moi.con_tro =
diem_dung`, KHÔNG phải `con_tro_lac_quan`.
::
:::

:::opt
`4` — vì `tim_diem_dung_that` đếm CẢ bản ghi hỏng LÀ "đã xử LÝ",
rồi TRẢ về vị trí NGAY sau nó
::why
Gần đúng ở việc bạn nghĩ TỚI việc "đếm luôn CẢ bản ghi hỏng" —
một cách TÍNH hợp lý NẾU coi mọi sector đã CHẠM tới đều "đã xử lý".

Chỗ lệch: `tim_diem_dung_that` **DỪNG NGAY** khi gặp bản ghi hỏng
ĐẦU tiên VÀ trả về CHÍNH chỉ số của NÓ (`3`, KHÔNG phải `4`) —
sector `3` ĐƯỢC coi LÀ "chưa hề có gì", sẵn SÀNG ghi ĐÈ lên, KHÔNG
phải "đã dùng RỒI, bỏ qua".
::
:::
::::

::::code{#viet_boss_khoi_dong_lai_an_toan}
Hoàn thiện `khoi_dong_lai_an_toan(dia, con_tro_lac_quan)` — TỔNG
hợp lại toàn bộ q02: dò điểm dừng thật, xây LẠI chỉ mục CHỈ từ phần
log đã xác NHẬN hợp lệ.

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
    ___
    kho_moi = NhatKyBitcaskCrc(dia)
    kho_moi.chi_muc = xay_lai_chi_muc_crc(dia, diem_dung)
    kho_moi.con_tro = diem_dung
    return kho_moi


dia = SimDisk(so_luong_sector=10, kich_thuoc_sector=28)
kho = NhatKyBitcaskCrc(dia)
kho.put(b'diem', b'80', 28)
kho.put(b'ten', b'Byte', 28)
kho.put(b'tuoi', b'25', 28)
dia.write(3, ghi_kv_crc(b'diem', b'95', 28))
dia.danh_dau_torn_ghi(3, 10)
dia.fsync()
dia.crash()
kho_phuc_hoi = khoi_dong_lai_an_toan(dia, 10)
print(kho_phuc_hoi.con_tro, kho_phuc_hoi.get(b'diem'), kho_phuc_hoi.get(b'ten'), kho_phuc_hoi.get(b'tuoi'))
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


dia = SimDisk(so_luong_sector=10, kich_thuoc_sector=28)
kho = NhatKyBitcaskCrc(dia)
kho.put(b'diem', b'80', 28)
kho.put(b'ten', b'Byte', 28)
kho.put(b'tuoi', b'25', 28)
dia.write(3, ghi_kv_crc(b'diem', b'95', 28))
dia.danh_dau_torn_ghi(3, 10)
dia.fsync()
dia.crash()
kho_phuc_hoi = khoi_dong_lai_an_toan(dia, 10)
print(kho_phuc_hoi.con_tro, kho_phuc_hoi.get(b'diem'), kho_phuc_hoi.get(b'ten'), kho_phuc_hoi.get(b'tuoi'))
```

```python title=test
dia = SimDisk(so_luong_sector=10, kich_thuoc_sector=28)
kho = NhatKyBitcaskCrc(dia)
kho.put(b'diem', b'80', 28)
kho.put(b'ten', b'Byte', 28)
kho.put(b'tuoi', b'25', 28)
dia.write(3, ghi_kv_crc(b'diem', b'95', 28))
dia.danh_dau_torn_ghi(3, 10)
dia.fsync()
dia.crash()
kho_phuc_hoi = khoi_dong_lai_an_toan(dia, 10)
assert kho_phuc_hoi.con_tro == 3, "diem dung that la 3"
assert kho_phuc_hoi.get(b'diem') == b'80', "gia tri goc cua diem con nguyen"
assert kho_phuc_hoi.get(b'ten') == b'Byte', "ten con nguyen"
assert kho_phuc_hoi.get(b'tuoi') == b'25', "tuoi con nguyen"

kho_phuc_hoi.put(b'diem', b'95', 28)
assert kho_phuc_hoi.chi_muc[b'diem'] == 3, "tai su dung dung sector torn write"
assert kho_phuc_hoi.get(b'diem') == b'95', "gia tri moi ghi thanh cong"

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
  expect: ^3 b'80' b'Byte' b'25'\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sống SÓT qua mất điện — WAL-ordering, CRC32, torn write, lost
fsync, dò điểm dừng, khởi động AN toàn, tiếp tục hoạt động — TẤT cả
ghép LẠI thành MỘT câu chuyện. q02 khép LẠI, ĐÓNG luôn cả q00-q02
(24 bài — nền lưu trữ MỘT máy).
::::

::::reflect{#nghi-lai}
q00 dạy "vị trí LÀ sector" (biết trước SỐ lượng). q01 dạy "ghi thêm
VÀO cuối" (không cần biết TRƯỚC, đánh đổi bằng chỉ mục RAM phải xây
lại). q02 dạy "TIN nhưng phải KIỂM chứng" — CRC32 phát hiện hỏng,
dò điểm dừng THẬT thay vì tin mù một con SỐ. TẤT cả đều SỐNG trong
một LOG phẳng, quét TUẦN tự. Nếu log DÀI tới mức quét HẾT (dù chỉ để
dò điểm dừng) TRỞ nên quá CHẬM — lưu trữ có cần một cấu TRÚC không
đòi quét TUYẾN tính để tìm thứ GÌ đó không?
::::

::::checkpoint{mastery=0.85}
::::
