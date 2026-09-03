---
id: co-so-du-lieu.nhat-ky-khong-bao-gio-quen.boss-nhat-ky-bitcask
title: "BOSS — Nhật ký Bitcask"
summary: "Ghép TRỌN q01: khoi_dong_lai(dia, con_tro) tạo một NhatKyBitcask MỚI, xây lại chi_muc bằng quét log — chứng minh get() vẫn đúng SAU một lần crash() thật sự. con_tro SAI (thiếu ghi cuối) làm chỉ mục trỏ về bản ghi CŨ, không báo lỗi."
locale: vi
track: co-so-du-lieu
module: nhat-ky-khong-bao-gio-quen
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.bitcask-store]
concepts: [db.boss-q01]
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
`NhatKyBitcask` ĐÃ put/get được. Giờ chứng MINH điều quan trọng
nhất: NÓ sống SÓT qua một lần "mất điện" THẬT sự — KHÔNG chỉ trong
CÙNG một lần chạy.
::::

::::explain{#khoi-dong-lai-that}
`khoi_dong_lai(dia, con_tro)` tạo một `NhatKyBitcask` MỚI TOANH, rồi
XÂY lại `chi_muc` bằng CÁCH quét log (bài 7) — `con_tro` PHẢI được
biết TRƯỚC (LÀM sao biết CHÍNH xác — câu hỏi CHO q02 "Khi điện
mất"):

```python title=readonly
def khoi_dong_lai(dia, con_tro):
    kho_moi = NhatKyBitcask(dia)
    kho_moi.chi_muc = xay_lai_chi_muc(dia, con_tro)
    kho_moi.con_tro = con_tro
    return kho_moi


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
kho = NhatKyBitcask(dia)
kho.put(b'diem', b'80', 24)
kho.put(b'ten', b'Byte', 24)
kho.put(b'diem', b'95', 24)
con_tro_truoc_crash = kho.con_tro

dia.crash()

kho_moi = khoi_dong_lai(dia, con_tro_truoc_crash)
print(kho_moi.get(b'diem'))
print(kho_moi.get(b'ten'))
```

```text title=readonly
b'95'
b'Byte'
```

SAU `dia.crash()` — `kho` (đối tượng CŨ) đã VÔ dụng (`chi_muc` của
nó VẪN còn trong RAM Ở phiên NÀY, nhưng trên MỘT hệ thống thật, cả
CHƯƠNG trình đã tắt). `kho_moi` LÀ một đối tượng HOÀN toàn MỚI, chỉ
biết `dia` VÀ `con_tro_truoc_crash` — VẪN đọc ĐÚNG `b'95'` (giá trị
MỚI nhất) VÀ `b'Byte'`.
::::

::::example{#du-lieu-da-fsync-song-sot}
MỌI `put` đều gọi `fsync()` NGAY (qua `ghi_va_luu_chi_muc` → 
`ghi_tiep`, bài 1-5) — NÊN mọi dữ liệu ĐÃ ghi đều sống SÓT qua
`crash()`, KHÔNG mất bản GHI nào:

```python title=readonly
print(kho_moi.con_tro == con_tro_truoc_crash)
```

```text title=readonly
True
```

`kho_moi.con_tro` khớp ĐÚNG con trỏ TRƯỚC crash — nhật ký KHÔNG
"quên" bất KỲ bản ghi nào ĐÃ fsync.
::::

::::predict{#doan-con-tro-sai-khi-khoi-dong commitOnce}
Byte khởi ĐỘNG lại NHƯNG truyền NHẦM `con_tro` (thiếu MẤT `1` — bỏ
sót bản ghi CUỐI cùng):

```python
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
kho = NhatKyBitcask(dia)
kho.put(b'diem', b'80', 24)
kho.put(b'diem', b'95', 24)
con_tro_that = kho.con_tro
dia.crash()

kho_sai = khoi_dong_lai(dia, con_tro_that - 1)
print(kho_sai.get(b'diem'))
```

Dòng cuối in ra gì?

:::opt{correct}
`b'80'`
:::

:::opt
`b'95'` — vì `khoi_dong_lai` LUÔN tìm ĐÚNG giá trị MỚI nhất, BẤT kể
`con_tro` truyền VÀO có ĐÚNG hay không
::why
Gần đúng ở việc bạn tin `khoi_dong_lai` đủ "AN toàn" để TỰ sửa một
tham SỐ sai — MỘT kỳ vọng dễ hiểu SAU khi vừa thấy nó phục hồi ĐÚNG
Ở ví dụ TRƯỚC.

Chỗ lệch: `xay_lai_chi_muc` chỉ quét ĐÚNG `con_tro` sector ĐẦU tiên
— truyền `con_tro_that - 1` NGHĨA LÀ bỏ SÓT sector cuối (nơi giá
trị `95` NẰM). `chi_muc` xây lại CHỈ thấy được bản ghi ĐẦU (`80`),
KHÔNG hề biết bản ghi THỨ hai TỒN tại.
::
:::

:::opt
Máy báo lỗi — vì `con_tro` sai SỐ lượng sector THẬT sự đã ghi
::why
Gần đúng ở việc bạn LO ngại đúng hướng — `con_tro` SAI đúng LÀ một
lỗi nghiêm TRỌNG.

Chỗ lệch: `xay_lai_chi_muc(dia, con_tro)` chỉ ĐƠN giản LẶP
`range(con_tro)` — MỘT con số NHỎ hơn thật vẫn HOÀN toàn hợp lệ về
mặt CÚ pháp/kiểu dữ liệu, chỉ LÀ bỏ SÓT dữ liệu, KHÔNG có gì để
Python phát hiện VÀ báo lỗi.
::
:::
::::

::::code{#viet_khoi_dong_lai}
Hoàn thiện `khoi_dong_lai(dia, con_tro)` — xây LẠI `chi_muc` BẰNG
cách quét log.

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


def ghi_tiep(dia, sector_tiep_theo, du_lieu):
    dia.write(sector_tiep_theo, du_lieu)
    dia.fsync()
    return sector_tiep_theo + 1


def ghi_kv(khoa, gia_tri, kich_thuoc):
    phan_khoa = len(khoa).to_bytes(4, 'big') + khoa
    phan_gia_tri = len(gia_tri).to_bytes(4, 'big') + gia_tri
    return (phan_khoa + phan_gia_tri).ljust(kich_thuoc, b'\x00')


def doc_kv(sector_bytes):
    do_dai_khoa = int.from_bytes(sector_bytes[0:4], 'big')
    khoa = sector_bytes[4:4 + do_dai_khoa]
    vi_tri = 4 + do_dai_khoa
    do_dai_gia_tri = int.from_bytes(sector_bytes[vi_tri:vi_tri + 4], 'big')
    gia_tri = sector_bytes[vi_tri + 4:vi_tri + 4 + do_dai_gia_tri]
    return (khoa, gia_tri)


def ghi_va_luu_chi_muc(dia, sector_tiep_theo, khoa, gia_tri, kich_thuoc, chi_muc):
    sector_ghi = sector_tiep_theo
    du_lieu = ghi_kv(khoa, gia_tri, kich_thuoc)
    sector_moi = ghi_tiep(dia, sector_tiep_theo, du_lieu)
    chi_muc[khoa] = sector_ghi
    return sector_moi


def tim_bang_chi_muc(dia, chi_muc, khoa_can_tim):
    if khoa_can_tim not in chi_muc:
        return None
    sector = chi_muc[khoa_can_tim]
    return doc_kv(dia.read(sector))[1]


def xay_lai_chi_muc(dia, con_tro):
    chi_muc_moi = {}
    for i in range(con_tro):
        khoa, _ = doc_kv(dia.read(i))
        chi_muc_moi[khoa] = i
    return chi_muc_moi


class NhatKyBitcask:
    def __init__(self, dia):
        self.dia = dia
        self.chi_muc = {}
        self.con_tro = 0

    def put(self, khoa, gia_tri, kich_thuoc):
        self.con_tro = ghi_va_luu_chi_muc(self.dia, self.con_tro, khoa, gia_tri, kich_thuoc, self.chi_muc)

    def get(self, khoa):
        return tim_bang_chi_muc(self.dia, self.chi_muc, khoa)


def khoi_dong_lai(dia, con_tro):
    kho_moi = NhatKyBitcask(dia)
    ___
    kho_moi.con_tro = con_tro
    return kho_moi


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
kho = NhatKyBitcask(dia)
kho.put(b'diem', b'80', 24)
kho.put(b'diem', b'95', 24)
con_tro_truoc_crash = kho.con_tro
dia.crash()

kho_moi = khoi_dong_lai(dia, con_tro_truoc_crash)
print(kho_moi.get(b'diem'))
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


def ghi_tiep(dia, sector_tiep_theo, du_lieu):
    dia.write(sector_tiep_theo, du_lieu)
    dia.fsync()
    return sector_tiep_theo + 1


def ghi_kv(khoa, gia_tri, kich_thuoc):
    phan_khoa = len(khoa).to_bytes(4, 'big') + khoa
    phan_gia_tri = len(gia_tri).to_bytes(4, 'big') + gia_tri
    return (phan_khoa + phan_gia_tri).ljust(kich_thuoc, b'\x00')


def doc_kv(sector_bytes):
    do_dai_khoa = int.from_bytes(sector_bytes[0:4], 'big')
    khoa = sector_bytes[4:4 + do_dai_khoa]
    vi_tri = 4 + do_dai_khoa
    do_dai_gia_tri = int.from_bytes(sector_bytes[vi_tri:vi_tri + 4], 'big')
    gia_tri = sector_bytes[vi_tri + 4:vi_tri + 4 + do_dai_gia_tri]
    return (khoa, gia_tri)


def ghi_va_luu_chi_muc(dia, sector_tiep_theo, khoa, gia_tri, kich_thuoc, chi_muc):
    sector_ghi = sector_tiep_theo
    du_lieu = ghi_kv(khoa, gia_tri, kich_thuoc)
    sector_moi = ghi_tiep(dia, sector_tiep_theo, du_lieu)
    chi_muc[khoa] = sector_ghi
    return sector_moi


def tim_bang_chi_muc(dia, chi_muc, khoa_can_tim):
    if khoa_can_tim not in chi_muc:
        return None
    sector = chi_muc[khoa_can_tim]
    return doc_kv(dia.read(sector))[1]


def xay_lai_chi_muc(dia, con_tro):
    chi_muc_moi = {}
    for i in range(con_tro):
        khoa, _ = doc_kv(dia.read(i))
        chi_muc_moi[khoa] = i
    return chi_muc_moi


class NhatKyBitcask:
    def __init__(self, dia):
        self.dia = dia
        self.chi_muc = {}
        self.con_tro = 0

    def put(self, khoa, gia_tri, kich_thuoc):
        self.con_tro = ghi_va_luu_chi_muc(self.dia, self.con_tro, khoa, gia_tri, kich_thuoc, self.chi_muc)

    def get(self, khoa):
        return tim_bang_chi_muc(self.dia, self.chi_muc, khoa)


def khoi_dong_lai(dia, con_tro):
    kho_moi = NhatKyBitcask(dia)
    kho_moi.chi_muc = xay_lai_chi_muc(dia, con_tro)
    kho_moi.con_tro = con_tro
    return kho_moi


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
kho = NhatKyBitcask(dia)
kho.put(b'diem', b'80', 24)
kho.put(b'diem', b'95', 24)
con_tro_truoc_crash = kho.con_tro
dia.crash()

kho_moi = khoi_dong_lai(dia, con_tro_truoc_crash)
print(kho_moi.get(b'diem'))
```

```python title=test
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
kho = NhatKyBitcask(dia)
kho.put(b'diem', b'80', 24)
kho.put(b'ten', b'Byte', 24)
kho.put(b'diem', b'95', 24)
p = kho.con_tro
dia.crash()

kho_moi = khoi_dong_lai(dia, p)
assert kho_moi.get(b'diem') == b'95', "gia tri moi nhat song sot qua crash"
assert kho_moi.get(b'ten') == b'Byte', "khoa khac cung song sot"
assert kho_moi.get(b'khong_co') is None, "khoa khong ton tai -- van None"
assert kho_moi.con_tro == p, "con tro khoi phuc dung"

dia2 = SimDisk(so_luong_sector=4, kich_thuoc_sector=10)
kho2 = khoi_dong_lai(dia2, 0)
assert kho2.get(b'a') is None, "khoi dong voi con_tro=0 -- chi muc rong"
```

:::hints
- kind: attention
  body: "Goi xay_lai_chi_muc(dia, con_tro), gan ket qua vao kho_moi.chi_muc."
- kind: strategy
  body: "kho_moi.chi_muc = xay_lai_chi_muc(dia, con_tro)"
- kind: one-line
  body: "kho_moi.chi_muc = xay_lai_chi_muc(dia, con_tro)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi xay_lai_chi_muc(dia, con_tro) va gan vao kho_moi.chi_muc
  requireAst:
  - kind: uses-call, target: xay_lai_chi_muc, min: 1
  - kind: uses-name, target: con_tro, min: 3
  - kind: uses-name, target: dia, min: 8
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^b'95'\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhật ký Bitcask — put, get, cập nhật, khởi động lại — TẤT cả sống
SÓT qua một lần "mất điện" thật SỰ. Đây LÀ track lưu trữ THẬT đầu
tiên của Byte.
::::

::::reflect{#nghi-lai}
q00 dạy "vị trí LÀ sector" — cần biết TRƯỚC số lượng. q01 dạy "ghi
thêm VÀO cuối" — KHÔNG cần biết trước, ĐÁNH đổi bằng một chỉ mục
TRONG RAM phải xây LẠI mỗi lần khởi ĐỘNG. Cả hai đều tin CẬY vào
MỘT điều: `fsync()` đã gọi RỒI thì dữ liệu chắc CHẮN còn đó. NHƯNG
`fsync()` CÓ THỂ thất bại GIỮA chừng — ghi DỞ dang, mất điện đúng
lúc CHƯA xong. Điều GÌ xảy ra khi ĐIỆN mất ngay GIỮA một lần ghi?
::::

::::checkpoint{mastery=0.85}
::::
