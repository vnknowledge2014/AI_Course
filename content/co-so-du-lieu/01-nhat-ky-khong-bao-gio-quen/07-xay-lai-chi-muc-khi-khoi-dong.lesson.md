---
id: co-so-du-lieu.nhat-ky-khong-bao-gio-quen.xay-lai-chi-muc-khi-khoi-dong
title: Xây lại chỉ mục khi khởi động
summary: "chi_muc chỉ sống trong RAM — khởi động lại PHẢI quét toàn bộ log MỘT lần, gán chi_muc_moi[khoá] = sector cho MỖI bản ghi THEO thứ tự TĂNG DẦN, để bản ghi mới nhất luôn ghi đè sau cùng. Quét NGƯỢC thứ tự cho ra chỉ mục SAI — trỏ về bản ghi CŨ."
locale: vi
track: co-so-du-lieu
module: nhat-ky-khong-bao-gio-quen
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [db.rebuild-index]
requires: [db.update-is-append]
concepts: [db.rebuild-index]
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
`chi_muc` sống TRONG RAM — tắt chương trình LÀ mất SẠCH nó. Khởi
động LẠI, `chi_muc` rỗng TOANG — nhưng nhật ký TRÊN đĩa VẪN còn
nguyên. LÀM sao dựng LẠI `chi_muc` TỪ đó?
::::

::::explain{#quet-lai-tu-dau}
Quét TOÀN bộ log TỪ sector `0`, gán `chi_muc_moi[khoá] = sector`
CHO mỗi bản ghi — bản ghi THEO SAU (mới HƠN) tự động GHI ĐÈ:

```python title=readonly
def xay_lai_chi_muc(dia, con_tro):
    chi_muc_moi = {}
    for i in range(con_tro):
        khoa, _ = doc_kv(dia.read(i))
        chi_muc_moi[khoa] = i
    return chi_muc_moi


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
p = 0
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'80', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'ten', b'Byte', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'95', 24, chi_muc)

dia.crash()
chi_muc_khoi_dong_lai = xay_lai_chi_muc(dia, p)
print(chi_muc_khoi_dong_lai)
```

```text title=readonly
{b'diem': 2, b'ten': 1}
```

`b'diem'` GHI hai lần (sector `0` VÀ `2`) — quét TỪ đầu tới CUỐI,
`chi_muc_moi[b'diem']` bị GÁN LẠI Ở sector `2` sau CÙNG, ĐÈ mất giá
trị `0` gán TRƯỚC đó. Kết quả GIỐNG HỆT `chi_muc` gốc (đã theo dõi
SỐNG lúc ghi) — mặc DÙ `chi_muc` gốc đã MẤT sau `crash()`.
::::

::::example{#ket-qua-giong-het}
So SÁNH trực tiếp: `chi_muc` gốc (còn LƯU trước khi crash) VÀ
`chi_muc` xây LẠI SAU crash:

```python title=readonly
print(chi_muc == chi_muc_khoi_dong_lai)
```

```text title=readonly
True
```

Y HỆT nhau — VÌ nhật ký (bài 1-3) LÀ nguồn sự thật DUY nhất, `chi_muc`
CHỈ LÀ một bản "TÓM tắt" tiện lợi của nó. Mất `chi_muc` KHÔNG mất
DỮ liệu, chỉ mất SỰ tiện lợi (phải tốn thời GIAN quét lại một LẦN).
::::

::::predict{#doan-quet-nguoc-thu-tu commitOnce}
Byte thử `xay_lai_chi_muc` NHƯNG quét **NGƯỢC** — TỪ sector cuối
CÙNG lùi VỀ `0` — thay VÌ từ đầu:

```python
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
p = 0
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'80', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'95', 24, chi_muc)
dia.crash()

chi_muc_sai = {}
for i in range(p - 1, -1, -1):
    khoa, _ = doc_kv(dia.read(i))
    chi_muc_sai[khoa] = i
print(chi_muc_sai)
```

Dòng cuối in ra gì?

:::opt{correct}
`{b'diem': 0}`
:::

:::opt
`{b'diem': 1}` — vì bản ghi MỚI nhất (sector `1`, giá trị `95`) LUÔN
"thắng", BẤT kể quét theo THỨ tự nào
::why
Gần đúng ở việc bạn TIN "mới nhất luôn thắng" LÀ một quy TẮC chung
— ĐÚNG với `xay_lai_chi_muc` gốc (quét XUÔI), MỘT trực giác HỢP lý
sau khi vừa thấy NÓ hoạt động.

Chỗ lệch: "mới nhất THẮNG" chỉ ĐÚNG vì gán DIỄN RA THEO thứ tự
TĂNG dần, khiến lần GÁN cuối CÙNG (mới nhất) đè lên TRƯỚC. Quét
**NGƯỢC** đảo lộn thứ tự GÁN — lần gán CUỐI cùng (VÒNG lặp cuối) LÀ
sector `0` (CŨ nhất), NÊN nó mới LÀ cái "thắng" — chi_muc SAI, trỏ
VỀ bản ghi CŨ.
::
:::

:::opt
Máy báo lỗi — vì `range(p - 1, -1, -1)` KHÔNG hợp lệ CHO một vòng
lặp lùi
::why
Gần đúng ở việc bạn NGHI ngờ cú pháp `range` VỚI bước ÂM — một cú
pháp DỄ nhầm lẫn.

Chỗ lệch: `range(bắt_đầu, dừng, bước)` VỚI `bước = -1` LÀ cú pháp
HOÀN toàn hợp lệ trong Python, ĐẾM lùi từ `bắt_đầu` TỚI (nhưng
KHÔNG bao gồm) `dừng` — `range(1, -1, -1)` sinh RA `1, 0`, đúng
NHƯ mong đợi.
::
:::
::::

::::code{#viet_xay_lai_chi_muc}
Hoàn thiện `xay_lai_chi_muc(dia, con_tro)` — gán `chi_muc_moi[khoá]
= sector` cho MỖI bản ghi, THEO thứ tự tăng DẦN.

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


def xay_lai_chi_muc(dia, con_tro):
    chi_muc_moi = {}
    for i in range(con_tro):
        khoa, _ = doc_kv(dia.read(i))
        ___
    return chi_muc_moi


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
p = 0
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'80', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'95', 24, chi_muc)
dia.crash()
print(xay_lai_chi_muc(dia, p))
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


def xay_lai_chi_muc(dia, con_tro):
    chi_muc_moi = {}
    for i in range(con_tro):
        khoa, _ = doc_kv(dia.read(i))
        chi_muc_moi[khoa] = i
    return chi_muc_moi


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
p = 0
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'80', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'95', 24, chi_muc)
dia.crash()
print(xay_lai_chi_muc(dia, p))
```

```python title=test
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
chi_muc = {}
p = 0
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'80', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'ten', b'Byte', 24, chi_muc)
p = ghi_va_luu_chi_muc(dia, p, b'diem', b'95', 24, chi_muc)
dia.crash()
chi_muc_lai = xay_lai_chi_muc(dia, p)
assert chi_muc_lai == chi_muc, "xay lai giong het chi_muc goc"
assert chi_muc_lai[b'diem'] == 2, "diem tro sector moi nhat"
assert chi_muc_lai[b'ten'] == 1, "ten tro dung sector"

dia2 = SimDisk(so_luong_sector=4, kich_thuoc_sector=24)
assert xay_lai_chi_muc(dia2, 0) == {}, "con_tro = 0 -- chi muc rong"
```

:::hints
- kind: attention
  body: "Gan chi_muc_moi[khoa] = i, dung tu ten bien i (khong phai sector_ghi hay bien khac)."
- kind: strategy
  body: "chi_muc_moi[khoa] = i"
- kind: one-line
  body: "chi_muc_moi[khoa] = i"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai gan chi_muc_moi[khoa] = i trong vong lap
  requireAst:
  - kind: uses-name, target: i, min: 2
  - kind: uses-name, target: khoa, min: 6
  - kind: uses-name, target: chi_muc_moi, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\{b'diem': 1\}\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chỉ mục xây LẠI đúng — VÌ nhật ký trên đĩa LÀ nguồn sự thật DUY
nhất, `chi_muc` chỉ LÀ bản tóm tắt. Ghép put/get + xây lại KHI khởi
động — Byte đã CÓ đủ mảnh CHO một Bitcask nhỏ HOÀN chỉnh chưa?
::::

::::reflect{#nghi-lai}
`chi_muc` chỉ LÀ một bản TÓM tắt tiện lợi — mất NÓ không mất dữ liệu,
CHỈ tốn thời gian quét LẠI một lần, THEO đúng thứ tự (cũ TRƯỚC, mới
SAU) để bản ghi MỚI nhất luôn thắng. Byte giờ CÓ đủ mọi mảnh: ghi
nối tiếp, bản ghi biến ĐỘ dài, khoá-giá trị, tra CỨU O(1), cập nhật
LÀ ghi thêm, VÀ xây lại chỉ mục — ghép TẤT cả lại thành MỘT cửa hàng
put/get HOÀN chỉnh, được KHÔNG?
::::

::::checkpoint{mastery=0.8}
::::
