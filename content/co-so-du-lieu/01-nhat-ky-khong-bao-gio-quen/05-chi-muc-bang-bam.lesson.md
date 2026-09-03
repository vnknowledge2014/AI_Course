---
id: co-so-du-lieu.nhat-ky-khong-bao-gio-quen.chi-muc-bang-bam
title: Chỉ mục bằng băm
summary: "Giữ một dict TRONG RAM (khoá → sector), cập nhật MỖI lần ghi — tra cứu O(1), KHÔNG cần quét. Khoá không tồn tại trả lời NGAY từ dict, KHÔNG đọc đĩa lần nào — khác hẳn quét (bài trước), nơi một miss buộc đọc hết."
locale: vi
track: co-so-du-lieu
module: nhat-ky-khong-bao-gio-quen
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [db.hash-index]
requires: [db.linear-scan]
concepts: [db.hash-index]
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
Quét TỪ đầu MỖI lần LÀ chậm. NẾU Byte NHỚ luôn "khoá NÀY nằm Ở
sector NÀO" — MỘT cuốn SỔ tay TRONG đầu — thì SAO?
::::

::::explain{#dict-lam-chi-muc}
Giữ một `dict` TRONG RAM: khoá → sector — CẬP nhật NÓ MỖI lần ghi
MỘT bản ghi MỚI:

```python title=readonly
def ghi_va_luu_chi_muc(dia, sector_tiep_theo, khoa, gia_tri, kich_thuoc, chi_muc):
    sector_ghi = sector_tiep_theo
    du_lieu = ghi_kv(khoa, gia_tri, kich_thuoc)
    sector_moi = ghi_tiep(dia, sector_tiep_theo, du_lieu)
    chi_muc[khoa] = sector_ghi
    return sector_moi


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
p = 0
p = ghi_va_luu_chi_muc(dia, p, b'id', b'7', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'ten', b'Byte', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'tuoi', b'25', 24, chi_muc)

print(chi_muc)
```

```text title=readonly
{b'id': 0, b'ten': 1, b'tuoi': 2}
```

`chi_muc` giờ nhớ ĐÚNG: `b'id'` Ở sector `0`, `b'ten'` Ở sector `1`,
`b'tuoi'` Ở sector `2` — KHÔNG cần đọc đĩa để BIẾT điều NÀY, `dict`
TRONG RAM trả lời NGAY.
::::

::::example{#tra-cuu-o1}
Tra cứu: HỎI `dict` trước (O(1)) — CÓ khoá thì mới đọc ĐÚNG một
sector, KHÔNG quét gì cả:

```python title=readonly
def tim_bang_chi_muc(dia, chi_muc, khoa_can_tim):
    if khoa_can_tim not in chi_muc:
        return None
    sector = chi_muc[khoa_can_tim]
    return doc_kv(dia.read(sector))[1]


print(tim_bang_chi_muc(dia, chi_muc, b'tuoi'))
```

```text title=readonly
b'25'
```

`b'tuoi'` NẰM trong `chi_muc` Ở sector `2` — CHỈ đọc ĐÚNG MỘT sector
(`dia.read(2)`), KHÔNG cần đọc sector `0` HAY `1` trước — KHÁC hẳn
`tim_bang_quet` (bài TRƯỚC) phải đọc TUẦN tự từ ĐẦU.
::::

::::predict{#doan-tra-cuu-mot-cai-that commitOnce}
Byte tra cứu MỘT khoá KHÔNG tồn tại (`'khong_co'`):

```python
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
p = 0
p = ghi_va_luu_chi_muc(dia, p, b'id', b'7', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'ten', b'Byte', 24, chi_muc)
print(tim_bang_chi_muc(dia, chi_muc, b'khong_co'))
```

Dòng cuối in ra gì?

:::opt{correct}
`None`
:::

:::opt
Máy báo lỗi — vì tra `chi_muc[khoa_can_tim]` VỚI khoá KHÔNG tồn tại
LUÔN ném `KeyError`
::why
Gần đúng ở việc bạn nhớ ĐÚNG: tra thẳng `dict[khoa]` VỚI khoá KHÔNG
có LUÔN ném `KeyError` — MỘT phản xạ Python CHUẨN xác.

Chỗ lệch: `tim_bang_chi_muc` kiểm TRA `if khoa_can_tim not in
chi_muc` **TRƯỚC** khi tra `chi_muc[khoa_can_tim]` — khoá KHÔNG có
THÌ trả về `None` NGAY, dòng CÓ thể ném `KeyError` KHÔNG BAO GIỜ
được chạy TỚI.
::
:::

:::opt
`b'7'` — vì `chi_muc` tra cứu "GẦN đúng", trả VỀ khoá GẦN giống nhất
khi khoá CHÍNH xác không có
::why
Gần đúng ở việc bạn nghĩ TỚI một cơ chế TÌM kiếm mờ (fuzzy) — hợp LÝ
Ở một SỐ hệ thống tra cứu KHÁC.

Chỗ lệch: `dict` Python CHỈ khớp **CHÍNH XÁC** — `b'khong_co'` hoàn
toàn KHÁC `b'id'`, `dict` KHÔNG có khái niệm "gần giống" NÀO cả.
::
:::
::::

::::code{#viet_tim_bang_chi_muc}
Hoàn thiện `tim_bang_chi_muc(dia, chi_muc, khoa_can_tim)` — đọc ĐÚNG
sector đã LƯU trong chỉ mục, trả VỀ giá trị.

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
    ___


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
p = 0
p = ghi_va_luu_chi_muc(dia, p, b'id', b'7', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'ten', b'Byte', 24, chi_muc)
print(tim_bang_chi_muc(dia, chi_muc, b'ten'))
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


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
p = 0
p = ghi_va_luu_chi_muc(dia, p, b'id', b'7', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'ten', b'Byte', 24, chi_muc)
print(tim_bang_chi_muc(dia, chi_muc, b'ten'))
```

```python title=test
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
p = 0
p = ghi_va_luu_chi_muc(dia, p, b'id', b'7', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'ten', b'Byte', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'tuoi', b'25', 24, chi_muc)
assert tim_bang_chi_muc(dia, chi_muc, b'ten') == b'Byte', "tim dung khoa giua"
assert tim_bang_chi_muc(dia, chi_muc, b'id') == b'7', "tim dung khoa dau"
assert tim_bang_chi_muc(dia, chi_muc, b'tuoi') == b'25', "tim dung khoa cuoi"
assert tim_bang_chi_muc(dia, chi_muc, b'khong_co') is None, "khoa khong ton tai -- None, khong loi"
assert tim_bang_chi_muc(dia, {}, b'id') is None, "chi muc rong -- None"
```

:::hints
- kind: attention
  body: "Doc dia.read(sector) roi doc_kv de tach khoa/gia_tri, lay phan tu thu hai."
- kind: strategy
  body: "return doc_kv(dia.read(sector))[1]"
- kind: one-line
  body: "return doc_kv(dia.read(sector))[1]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai doc_kv(dia.read(sector)) roi lay phan tu thu hai (gia tri)
  requireAst:
  - kind: uses-name, target: sector, min: 5
  - kind: uses-call, target: read, min: 1
  - kind: uses-call, target: doc_kv, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^b'Byte'\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tra cứu O(1) — HỎI dict TRƯỚC, đọc đúng MỘT sector NẾU có. NHƯNG
nếu Byte SỬA một khoá ĐÃ có — GHI đè lên bản ghi CŨ hay THÊM một bản
ghi MỚI?
::::

::::reflect{#nghi-lai}
Chỉ mục bằng BĂM — dict trong RAM nhớ khoá NẰM Ở sector nào, tra cứu
O(1), khoá KHÔNG tồn tại trả lời NGAY từ RAM, KHÔNG đọc đĩa lần NÀO.
NHƯNG nếu Byte muốn SỬA giá trị của một khoá ĐÃ ghi rồi — nhật ký
CHỈ biết "ghi THÊM vào cuối" (bài 1) — VẬY sửa MỘT khoá nghĩa LÀ
GHI đè, hay THÊM một bản ghi MỚI?
::::

::::checkpoint{mastery=0.8}
::::
