---
id: co-so-du-lieu.nhat-ky-khong-bao-gio-quen.cap-nhat-la-ghi-them
title: Cập nhật là ghi thêm
summary: "Sửa một khoá KHÔNG sửa tại chỗ — nhật ký chỉ biết append (bài 1), nên 'cập nhật' LÀ ghi một bản ghi MỚI với cùng khoá, rồi chi_muc TRỎ sang sector mới nhất. Sector cũ vẫn còn nguyên byte cũ — chưa ai xoá nó, chỉ không còn ai TRỎ tới."
locale: vi
track: co-so-du-lieu
module: nhat-ky-khong-bao-gio-quen
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [db.update-is-append]
requires: [db.hash-index]
concepts: [db.update-is-append]
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
Byte muốn SỬA điểm số từ `80` THÀNH `95`. Nhật ký (bài 1) CHỈ biết
"ghi THÊM vào cuối" — KHÔNG hề biết "sửa tại chỗ". Sửa MỘT khoá thì
LÀM sao?
::::

::::explain{#cap-nhat-la-ghi-them}
`ghi_va_luu_chi_muc` (bài 5) đã LÀ câu trả lời — gọi LẠI NÓ với
CÙNG khoá, giá trị MỚI. `chi_muc[khoa]` bị GHI đè sang sector MỚI
nhất, KHÔNG hề "sửa" byte NÀO trên đĩa:

```python title=readonly
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
p = 0
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'80', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'95', 24, chi_muc)

print(chi_muc)
print(tim_bang_chi_muc(dia, chi_muc, b'diem'))
```

```text title=readonly
{b'diem': 1}
b'95'
```

Gọi `ghi_va_luu_chi_muc` LẦN hai VỚI cùng khoá `b'diem'` — bản ghi
MỚI (`b'95'`) NẰM Ở sector `1`, VÀ `chi_muc[b'diem']` giờ TRỎ sang
`1` (KHÔNG còn `0`). "Sửa" một khoá VÀ "thêm" một khoá MỚI LÀ đúng
MỘT thao TÁC — `ghi_va_luu_chi_muc` KHÔNG cần biết khoá đã CÓ hay
CHƯA.
::::

::::example{#sector-cu-van-con}
NHƯNG sector `0` — nơi giá trị `80` từng NẰM — CÓ bị xoá KHÔNG?

```python title=readonly
print(doc_kv(dia.read(0))[1])
```

```text title=readonly
b'80'
```

VẪN còn NGUYÊN — `SimDisk.write` LẦN hai ghi VÀO sector `1` (sector
MỚI, do `sector_tiep_theo` trỏ TỚI), KHÔNG hề chạm tới sector `0`.
Giá trị `80` VẪN nằm ĐÓ, chỉ LÀ KHÔNG còn ai TRỎ tới NÓ nữa (`chi_muc`
đã CẬP nhật sang sector `1`) — một bản ghi **CŨ**, chiếm chỗ MÀ
KHÔNG còn dùng được.
::::

::::predict{#doan-doc-truc-tiep-sector-cu commitOnce}
Byte cập nhật `'diem'` từ `80` LÊN `95`, RỒI đọc **trực tiếp** sector
`0` (bỏ QUA `chi_muc`, KHÔNG dùng `tim_bang_chi_muc`):

```python
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
p = 0
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'80', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'95', 24, chi_muc)
print(doc_kv(dia.read(0))[1])
```

Dòng cuối in ra gì?

:::opt{correct}
`b'80'`
:::

:::opt
`b'95'` — vì ghi LẠI cùng khoá TỰ động "dọn" MỌI sector cũ từng lưu
khoá đó VỀ giá trị mới NHẤT
::why
Gần đúng ở việc bạn nghĩ TỚI một hệ THỐNG "dọn dẹp" hợp LÝ khi
cập nhật — MỘT kỳ vọng tự NHIÊN với dữ liệu cần "LUÔN mới nhất".

Chỗ lệch: `ghi_va_luu_chi_muc` chỉ ĐƠN thuần `append` MỘT bản ghi
MỚI Ở sector MỚI — nó KHÔNG hề biết (và KHÔNG chạm TỚI) những sector
CŨ từng lưu CÙNG khoá này. Sector `0` GIỮ nguyên giá trị `80` MÃI
MÃI cho tới khi CÓ một cơ chế dọn dẹp RIÊNG (chuyện của một quest
SAU).
::
:::

:::opt
Máy báo lỗi — vì đọc TRỰC tiếp sector `0` (bỏ QUA `chi_muc`) LÀ thao
tác KHÔNG hợp lệ
::why
Gần đúng ở việc bạn LO ngại "bỏ qua LỚP trừu tượng" `chi_muc` LÀ sai
quy TẮC — một trực GIÁC hợp lý VỀ việc TÔN trọng giao diện.

Chỗ lệch: `dia.read(0)` chỉ ĐƠN giản đọc RAW byte của sector `0` —
HOÀN toàn hợp LỆ về mặt kỹ THUẬT. `chi_muc` chỉ LÀ một "gợi Ý" tiện
lợi Ở tầng ỨNG dụng, KHÔNG phải một RÀNG buộc bắt BUỘC của `SimDisk`.
::
:::
::::

::::code{#viet_dem_ban_ghi_cu}
Viết `dem_ban_ghi_cu(dia, con_tro, chi_muc)` — đếm BAO nhiêu bản ghi
(TỪ sector `0` tới `con_tro-1`) LÀ **cũ** (`chi_muc` KHÔNG còn trỏ
tới NÓ).

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


def dem_ban_ghi_cu(dia, con_tro, chi_muc):
    so_cu = 0
    for i in range(con_tro):
        khoa, _ = doc_kv(dia.read(i))
        ___
    return so_cu


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
p = 0
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'80', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'95', 24, chi_muc)
print(dem_ban_ghi_cu(dia, p, chi_muc))
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


def dem_ban_ghi_cu(dia, con_tro, chi_muc):
    so_cu = 0
    for i in range(con_tro):
        khoa, _ = doc_kv(dia.read(i))
        if chi_muc.get(khoa) != i:
            so_cu += 1
    return so_cu


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
p = 0
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'80', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'95', 24, chi_muc)
print(dem_ban_ghi_cu(dia, p, chi_muc))
```

```python title=test
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
p = 0
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'80', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'95', 24, chi_muc)
assert dem_ban_ghi_cu(dia, p, chi_muc) == 1, "mot ban ghi cu -- sector 0"

dia2 = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc2 = {}
q = 0
q = ghi_va_luu_chi_muc(dia2, q, b'id', b'7', 24, chi_muc2)
q = ghi_va_luu_chi_muc(dia2, q, b'ten', b'Byte', 24, chi_muc2)
assert dem_ban_ghi_cu(dia2, q, chi_muc2) == 0, "khong co ban ghi cu -- khoa khac nhau"

dia3 = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc3 = {}
r = 0
r = ghi_va_luu_chi_muc(dia3, r, b'diem', b'1', 24, chi_muc3)
r = ghi_va_luu_chi_muc(dia3, r, b'diem', b'2', 24, chi_muc3)
r = ghi_va_luu_chi_muc(dia3, r, b'diem', b'3', 24, chi_muc3)
assert dem_ban_ghi_cu(dia3, r, chi_muc3) == 2, "cap nhat ba lan -- hai ban ghi cu"
```

:::hints
- kind: attention
  body: "Neu chi_muc.get(khoa) khac i, ban ghi o sector i la cu -- tang so_cu."
- kind: strategy
  body: "if chi_muc.get(khoa) != i: so_cu += 1"
- kind: one-line
  body: "if chi_muc.get(khoa) != i: so_cu += 1"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai so sanh chi_muc.get(khoa) voi i, tang so_cu neu khac
  requireAst:
  - kind: uses-name, target: i, min: 2
  - kind: uses-name, target: khoa, min: 6
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^1\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cập nhật LÀ ghi thêm — bản ghi CŨ vẫn còn NGUYÊN, chỉ KHÔNG còn ai
trỏ tới. Nhật ký giờ CÓ thể mọc DÀI mãi VỚI đầy rác — nhưng NÓ sống
SÓT qua "mất điện" KHÔNG?
::::

::::reflect{#nghi-lai}
Cập nhật KHÔNG sửa tại chỗ — NÓ append một bản ghi MỚI, `chi_muc` TRỎ
sang sector mới NHẤT, sector CŨ vẫn còn nguyên byte (chỉ không còn AI
trỏ tới — rác, chờ dọn Ở một quest SAU). NHƯNG `chi_muc` chỉ sống
TRONG RAM — nếu chương trình TẮT rồi khởi động LẠI, `chi_muc` có còn
nhớ GÌ không?
::::

::::checkpoint{mastery=0.8}
::::
