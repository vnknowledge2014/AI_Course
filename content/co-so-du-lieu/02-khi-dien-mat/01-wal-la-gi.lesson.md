---
id: co-so-du-lieu.khi-dien-mat.wal-la-gi
title: WAL là gì
summary: "Write-Ahead Log — ghi dữ liệu THẬT xuống đĩa VÀ fsync() TRƯỚC KHI cập nhật trạng thái trong RAM (chi_muc). q01 đã làm ĐÚNG thứ tự này. Đảo ngược (cập nhật chi_muc TRƯỚC) tạo ra một mục chỉ mục MA — trỏ tới sector chưa hề có dữ liệu thật."
locale: vi
track: co-so-du-lieu
module: khi-dien-mat
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [db.wal-ordering]
requires: [db.bitcask-store]
concepts: [db.wal-ordering]
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
Ở q01, `put()` LUÔN gọi `write()`+`fsync()` TRƯỚC, RỒI mới cập nhật
`chi_muc`. Thứ tự NÀY — CÓ Ý nghĩa gì, HAY chỉ tình cờ?
::::

::::explain{#ghi-truoc-cap-nhat-sau}
KHÔNG tình cờ — đây LÀ nguyên tắc **Write-Ahead Log** (WAL, "nhật ký
ghi TRƯỚC"): ghi dữ liệu THẬT xuống đĩa VÀ `fsync()` **TRƯỚC KHI**
coi thao TÁC là hoàn tất (cập nhật trạng thái TRONG RAM):

```python title=readonly
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


def ghi_kv(khoa, gia_tri, kich_thuoc):
    phan_khoa = len(khoa).to_bytes(4, 'big') + khoa
    phan_gia_tri = len(gia_tri).to_bytes(4, 'big') + gia_tri
    return (phan_khoa + phan_gia_tri).ljust(kich_thuoc, b'\x00')


def ghi_an_toan(dia, sector, khoa, gia_tri, kich_thuoc, chi_muc):
    du_lieu = ghi_kv(khoa, gia_tri, kich_thuoc)
    dia.write(sector, du_lieu)
    dia.fsync()
    chi_muc[khoa] = sector    # CHỈ cập nhật SAU KHI durable


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
ghi_an_toan(dia, 0, b'diem', b'80', 24, chi_muc)

print(chi_muc)
```

```text title=readonly
{b'diem': 0}
```

Ba dòng ĐẦU (`write`, `fsync`) LÀM đúng thứ TỰ "ghi Ý định XUỐNG
đĩa TRƯỚC" — CHỈ khi đĩa đã XÁC nhận bền (`fsync()` trả về), dòng
CUỐI mới cập nhật `chi_muc`. `chi_muc[b'diem'] = 0` giờ LÀ một LỜI
HỨA đáng tin: sector `0` THẬT sự CÓ dữ liệu.
::::

::::example{#dao-nguoc-thu-tu-nguy-hiem}
ĐẢO ngược thứ tự — cập nhật `chi_muc` TRƯỚC — RỒI mô phỏng `fsync()`
"nói dối" (`boQuaFsyncKeTiep`, gói `@byte/simdisk` v2) VÀ mất điện
THẬT:

```python title=readonly
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


def ghi_kv(khoa, gia_tri, kich_thuoc):
    phan_khoa = len(khoa).to_bytes(4, 'big') + khoa
    phan_gia_tri = len(gia_tri).to_bytes(4, 'big') + gia_tri
    return (phan_khoa + phan_gia_tri).ljust(kich_thuoc, b'\x00')


def ghi_nguy_hiem(dia, sector, khoa, gia_tri, kich_thuoc, chi_muc):
    chi_muc[khoa] = sector    # cập nhật TRƯỚC
    du_lieu = ghi_kv(khoa, gia_tri, kich_thuoc)
    dia.write(sector, du_lieu)
    dia.fsync()


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
dia.bo_qua_fsync_ke_tiep()
ghi_nguy_hiem(dia, 0, b'diem', b'80', 24, chi_muc)
dia.crash()

print(chi_muc)
print(dia.read(0))
```

```text title=readonly
{b'diem': 0}
b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
```

`chi_muc` VẪN nói "khoá `b'diem'` Ở sector `0`" — NHƯNG sector `0`
TOÀN số `0`, chưa HỀ có dữ liệu thật (fsync "NÓI dối" RỒI crash xoá
sạch cache). Một MỤC chỉ mục **MA** — trỏ TỚI dữ liệu KHÔNG hề tồn
tại.
::::

::::predict{#doan-doc-muc-ma commitOnce}
Byte tra CỨU khoá `b'diem'` sau khi mục chỉ MỤC ma ĐÃ hình thành
(tiếp NỐI ví dụ trên — `chi_muc = {b'diem': 0}`, sector `0` TOÀN số
`0`):

```python
def doc_kv(sector_bytes):
    do_dai_khoa = int.from_bytes(sector_bytes[0:4], 'big')
    khoa = sector_bytes[4:4 + do_dai_khoa]
    vi_tri = 4 + do_dai_khoa
    do_dai_gia_tri = int.from_bytes(sector_bytes[vi_tri:vi_tri + 4], 'big')
    gia_tri = sector_bytes[vi_tri + 4:vi_tri + 4 + do_dai_gia_tri]
    return (khoa, gia_tri)

def tim_bang_chi_muc(dia, chi_muc, khoa_can_tim):
    if khoa_can_tim not in chi_muc:
        return None
    sector = chi_muc[khoa_can_tim]
    return doc_kv(dia.read(sector))[1]

print(tim_bang_chi_muc(dia, chi_muc, b'diem'))
```

Dòng cuối in ra gì?

:::opt{correct}
`b''`
:::

:::opt
`None` — vì `chi_muc` "biết" mục NÀY thật ra KHÔNG hợp lệ, giống
như khoá CHƯA từng tồn tại
::why
Gần đúng ở việc bạn mong CHỜ một tín HIỆU rõ ràng CHO "dữ liệu
không HỢP lệ" — một kỳ vọng hợp LÝ sau khi thấy `tim_bang_chi_muc`
(q01) trả `None` CHO khoá không TỒN tại.

Chỗ lệch: `chi_muc` CHỈ LÀ một `dict` — nó KHÔNG biết (VÀ không thể
biết) mục `b'diem': 0` LÀ "ma" hay THẬT, nó chỉ ĐƠN thuần CÓ khoá
đó. `tim_bang_chi_muc` đọc sector `0` (TOÀN số `0`), giải mã RA
được một cặp khoá/giá TRỊ RỖNG (`b''`, `b''`) — trả về `b''`, KHÔNG
phải `None`.
::
:::

:::opt
Máy báo lỗi — vì sector `0` toàn số `0` KHÔNG phải một bản ghi hợp
lệ
::why
Gần đúng ở việc bạn NHẬN ra ĐÚNG sector `0` KHÔNG chứa dữ liệu
THẬT — một quan SÁT chính xác.

Chỗ lệch: `doc_kv` (q01) CHỈ thuần tuý cắt lát VÀ giải mã, KHÔNG hề
kiểm TRA "có hợp lý" — TOÀN số `0` vẫn giải mã ÊM ru thành cặp
RỖNG, giống HỆT bài `simdisk-doc-lai-ban-ghi` (q00) đã dạy.
::
:::
::::

::::code{#viet_ghi_an_toan}
Hoàn thiện `ghi_an_toan(dia, sector, khoa, gia_tri, kich_thuoc,
chi_muc)` — cập nhật `chi_muc` CHỈ SAU KHI `write`/`fsync` xong.

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


def ghi_kv(khoa, gia_tri, kich_thuoc):
    phan_khoa = len(khoa).to_bytes(4, 'big') + khoa
    phan_gia_tri = len(gia_tri).to_bytes(4, 'big') + gia_tri
    return (phan_khoa + phan_gia_tri).ljust(kich_thuoc, b'\x00')


def ghi_an_toan(dia, sector, khoa, gia_tri, kich_thuoc, chi_muc):
    du_lieu = ghi_kv(khoa, gia_tri, kich_thuoc)
    dia.write(sector, du_lieu)
    dia.fsync()
    ___


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
ghi_an_toan(dia, 0, b'diem', b'80', 24, chi_muc)
print(chi_muc)
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


def ghi_kv(khoa, gia_tri, kich_thuoc):
    phan_khoa = len(khoa).to_bytes(4, 'big') + khoa
    phan_gia_tri = len(gia_tri).to_bytes(4, 'big') + gia_tri
    return (phan_khoa + phan_gia_tri).ljust(kich_thuoc, b'\x00')


def ghi_an_toan(dia, sector, khoa, gia_tri, kich_thuoc, chi_muc):
    du_lieu = ghi_kv(khoa, gia_tri, kich_thuoc)
    dia.write(sector, du_lieu)
    dia.fsync()
    chi_muc[khoa] = sector


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
ghi_an_toan(dia, 0, b'diem', b'80', 24, chi_muc)
print(chi_muc)
```

```python title=test
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
ghi_an_toan(dia, 0, b'diem', b'80', 24, chi_muc)
assert chi_muc == {b'diem': 0}, "chi muc cap nhat dung sau ghi"

dia2 = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc2 = {}
ghi_an_toan(dia2, 3, b'ten', b'Byte', 24, chi_muc2)
assert chi_muc2 == {b'ten': 3}, "sector khac van dung"
assert dia2.read(3) != bytes(24), "du lieu that su da ghi xuong dia"
```

:::hints
- kind: attention
  body: "Gan chi_muc[khoa] = sector, SAU dong dia.fsync() da co san."
- kind: strategy
  body: "chi_muc[khoa] = sector"
- kind: one-line
  body: "chi_muc[khoa] = sector"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai gan chi_muc[khoa] = sector SAU khi write/fsync da chay
  requireAst:
  - kind: uses-name, target: chi_muc, min: 1
  - kind: uses-name, target: khoa, min: 4
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\{b'diem': 0\}\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
"Ghi TRƯỚC, cập nhật SAU" — nguyên tắc WAL. NHƯNG dữ liệu ĐÃ ghi CÓ
thể vẫn SAI — nếu chỉ MỘT phần của nó tới được ĐĨA thì sao?
::::

::::reflect{#nghi-lai}
WAL: ghi dữ liệu THẬT xuống đĩa VÀ `fsync()` TRƯỚC khi coi thao TÁC
xong — q01 đã LÀM đúng, đảo ngược thứ TỰ tạo ra mục chỉ mục MA.
NHƯNG dữ liệu ĐÃ ghi (KHÔNG phải mục ma) vẫn CÓ thể sai — nếu điện
mất đúng LÚC đang ghi, chỉ MỘT PHẦN byte tới được ĐĨA thì sao? Làm
sao BIẾT một bản ghi ĐÃ đủ, hay chỉ ghi được NỬA chừng?
::::

::::checkpoint{mastery=0.8}
::::
