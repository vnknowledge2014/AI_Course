---
id: co-so-du-lieu.nhat-ky-khong-bao-gio-quen.gop-thanh-mot-cua-hang
title: Gộp thành một cửa hàng
summary: "Gói dia, chi_muc, con_tro vào MỘT class NhatKyBitcask — put()/get() thay vì truyền tay ba biến rời rạc qua mọi hàm. Quên cập nhật self.con_tro trong put() là bug SILENT nguy hiểm nhất: ghi đè liên tục lên CÙNG một sector, khoá CŨ đọc RA giá trị của khoá MỚI."
locale: vi
track: co-so-du-lieu
module: nhat-ky-khong-bao-gio-quen
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [db.bitcask-store]
requires: [db.rebuild-index]
concepts: [db.bitcask-store]
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
`dia`, `chi_muc`, `con_tro` — BA biến RỜI rạc, phải truyền TAY qua
MỌI hàm. Gói CẢ ba VÀO một chỗ, được KHÔNG?
::::

::::explain{#goi-vao-mot-class}
Một `class` giữ CẢ ba làm THUỘC tính, lộ RA đúng hai HÀM: `put` VÀ
`get`:

```python title=readonly
class NhatKyBitcask:
    def __init__(self, dia):
        self.dia = dia
        self.chi_muc = {}
        self.con_tro = 0

    def put(self, khoa, gia_tri, kich_thuoc):
        self.con_tro = ghi_va_luu_chi_muc(self.dia, self.con_tro, khoa, gia_tri, kich_thuoc, self.chi_muc)

    def get(self, khoa):
        return tim_bang_chi_muc(self.dia, self.chi_muc, khoa)


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
kho = NhatKyBitcask(dia)
kho.put(b'diem', b'80', 24)
kho.put(b'ten', b'Byte', 24)
kho.put(b'diem', b'95', 24)

print(kho.get(b'diem'))
print(kho.get(b'ten'))
```

```text title=readonly
b'95'
b'Byte'
```

`kho.put(...)` GỌI đúng `ghi_va_luu_chi_muc` (bài 5) NHƯ trước —
NHƯNG giờ `self.con_tro` TỰ cập nhật BÊN trong class, người GỌI
`kho.put(...)` KHÔNG cần tự theo dõi con TRỎ nữa.
::::

::::example{#khong-can-truyen-tay}
So SÁNH: TRƯỚC đây phải truyền `dia`, `p`, `chi_muc` qua MỌI lời
gọi; giờ CHỈ cần MỘT đối tượng `kho`:

```python title=readonly
print(kho.con_tro)
```

```text title=readonly
3
```

`kho.con_tro` giờ LÀ `3` — TỰ tăng qua BA lần `put`, KHÔNG một dòng
code NÀO bên NGOÀI class từng chạm VÀO nó trực tiếp.
::::

::::predict{#doan-quen-cap-nhat-con-tro-trong-class commitOnce}
Một PHIÊN bản `NhatKyBitcaskSai` — `put` QUÊN gán lại `self.con_tro`
(gọi `ghi_va_luu_chi_muc` NHƯNG KHÔNG lưu kết QUẢ trả về):

```python
class NhatKyBitcaskSai:
    def __init__(self, dia):
        self.dia = dia
        self.chi_muc = {}
        self.con_tro = 0

    def put(self, khoa, gia_tri, kich_thuoc):
        ghi_va_luu_chi_muc(self.dia, self.con_tro, khoa, gia_tri, kich_thuoc, self.chi_muc)

    def get(self, khoa):
        return tim_bang_chi_muc(self.dia, self.chi_muc, khoa)


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=10)
kho = NhatKyBitcaskSai(dia)
kho.put(b'a', b'1', 10)
kho.put(b'b', b'2', 10)
print(kho.get(b'a'))
```

Dòng cuối in ra gì?

:::opt{correct}
`b'2'`
:::

:::opt
`b'1'` — vì `chi_muc[b'a']` đã LƯU sẵn giá trị `b'1'` TỪ lần `put`
đầu, KHÔNG đổi
::why
Gần đúng ở việc bạn nghĩ TỚI "khoá `a` giữ giá TRỊ riêng của NÓ" —
một trực GIÁC hợp lý VỚI `dict` thông THƯỜNG.

Chỗ lệch: `self.con_tro` KHÔNG hề tăng (quên gán LẠI) — CẢ hai lần
`put` đều ghi VÀO sector `0`! Lần `put(b'b', ...)` GHI ĐÈ sector `0`
BẰNG bản ghi CỦA `b`, VÀ `chi_muc[b'a']` (VẪN trỏ sector `0`, KHÔNG
ai cập nhật) giờ ĐỌC nhầm sang bản ghi của `b` — trả VỀ `b'2'`,
KHÔNG phải giá trị CỦA chính `a`.
::
:::

:::opt
Máy báo lỗi — vì hai khoá KHÁC nhau (`a`, `b`) KHÔNG thể cùng chia
sẻ MỘT sector
::why
Gần đúng ở việc bạn LO ngại đúng hướng — hai khoá CHUNG một sector
LÀ một tình huống BẤT thường, ĐÁNG ngờ.

Chỗ lệch: `SimDisk.write(sector, data)` (bài 4, q00) KHÔNG hề kiểm
tra "sector NÀY đã dùng CHO khoá khác chưa" — nó chỉ ĐƠN giản GHI
đè, KHÔNG hỏi han GÌ, đúng NHƯ bài `khi-khong-biet-truoc-so-luong`
đã CẢNH báo TỪ đầu quest NÀY.
::
:::
::::

::::code{#viet_put}
Hoàn thiện `put(self, khoa, gia_tri, kich_thuoc)` — gọi
`ghi_va_luu_chi_muc` VÀ **LƯU lại** con trỏ MỚI vào `self.con_tro`.

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


class NhatKyBitcask:
    def __init__(self, dia):
        self.dia = dia
        self.chi_muc = {}
        self.con_tro = 0

    def put(self, khoa, gia_tri, kich_thuoc):
        ___

    def get(self, khoa):
        return tim_bang_chi_muc(self.dia, self.chi_muc, khoa)


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=10)
kho = NhatKyBitcask(dia)
kho.put(b'a', b'1', 10)
kho.put(b'b', b'2', 10)
print(kho.get(b'a'))
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


class NhatKyBitcask:
    def __init__(self, dia):
        self.dia = dia
        self.chi_muc = {}
        self.con_tro = 0

    def put(self, khoa, gia_tri, kich_thuoc):
        self.con_tro = ghi_va_luu_chi_muc(self.dia, self.con_tro, khoa, gia_tri, kich_thuoc, self.chi_muc)

    def get(self, khoa):
        return tim_bang_chi_muc(self.dia, self.chi_muc, khoa)


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=10)
kho = NhatKyBitcask(dia)
kho.put(b'a', b'1', 10)
kho.put(b'b', b'2', 10)
print(kho.get(b'a'))
```

```python title=test
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=10)
kho = NhatKyBitcask(dia)
kho.put(b'a', b'1', 10)
kho.put(b'b', b'2', 10)
assert kho.get(b'a') == b'1', "a khong bi ghi de -- con_tro phai tang"
assert kho.get(b'b') == b'2', "b dung"
assert kho.con_tro == 2, "con tro tang dung hai lan"

kho.put(b'a', b'9', 10)
assert kho.get(b'a') == b'9', "cap nhat a"
assert kho.con_tro == 3, "con tro tang lan thu ba"
assert kho.get(b'khong_co') is None, "khoa khong ton tai"
```

:::hints
- kind: attention
  body: "Goi ghi_va_luu_chi_muc va GAN LAI ket qua vao self.con_tro."
- kind: strategy
  body: "self.con_tro = ghi_va_luu_chi_muc(self.dia, self.con_tro, khoa, gia_tri, kich_thuoc, self.chi_muc)"
- kind: one-line
  body: "self.con_tro = ghi_va_luu_chi_muc(self.dia, self.con_tro, khoa, gia_tri, kich_thuoc, self.chi_muc)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi ghi_va_luu_chi_muc va gan lai vao self.con_tro
  requireAst:
  - kind: uses-call, target: ghi_va_luu_chi_muc, min: 1
  - kind: uses-name, target: khoa, min: 7
  - kind: uses-name, target: gia_tri, min: 5
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^b'1'\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`NhatKyBitcask` — MỘT cửa hàng put/get HOÀN chỉnh, tự quản CẢ đĩa
lẫn chỉ mục. CÒN thiếu đúng MỘT điều: sống SÓT qua "mất điện" thật
sự.
::::

::::reflect{#nghi-lai}
Gộp `dia`, `chi_muc`, `con_tro` VÀO một `class` — `put`/`get` gọn
GÀNG, KHÔNG cần truyền tay BA biến rời rạc. NHƯNG quên cập nhật
`self.con_tro` LÀ bug ÂM thầm nguy hiểm nhất quest NÀY — khoá CŨ đọc
RA giá trị của khoá MỚI, KHÔNG lỗi gì báo RA. Byte đã CÓ đủ mọi
mảnh — ghép TẤT cả lại, CHỨNG minh nó sống SÓT qua một lần "mất
điện" thật sự.
::::

::::checkpoint{mastery=0.8}
::::
