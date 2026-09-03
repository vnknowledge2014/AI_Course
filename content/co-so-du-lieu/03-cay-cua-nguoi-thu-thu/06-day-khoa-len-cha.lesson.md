---
id: co-so-du-lieu.cay-cua-nguoi-thu-thu.day-khoa-len-cha
title: Đẩy khoá lên cha
summary: "Khi lá GỐC (chưa có cha) bị tách, tao_goc_moi tạo một node TRONG mới — chỉ MỘT khoá (khoa_tach) và HAI con trỏ (số sector của hai lá). Đây LÀ lần đầu tiên cây TĂNG chiều cao: từ một lá đơn độc thành một root có hai con."
locale: vi
track: co-so-du-lieu
module: cay-cua-nguoi-thu-thu
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [db.push-key-to-parent]
requires: [db.leaf-split]
concepts: [db.push-key-to-parent]
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
Lá GỐC (chưa CÓ cha) vừa TÁCH thành hai. `khoa_tach` cần LƯU Ở một
node CHA — NHƯNG khi CHƯA có node cha NÀO cả, phải TẠO một node
HOÀN toàn mới.
::::

::::explain{#tao-node-trong-dau-tien}
`tao_goc_moi` tạo một node **TRONG** (internal) — CHỈ một khoá
(`khoa_tach`) VÀ hai con TRỎ (số sector của hai LÁ):

```python title=readonly
import json


def ma_hoa_node(node, kich_thuoc):
    d = json.dumps(node).encode('utf-8')
    return d.ljust(kich_thuoc, b' ')


def giai_ma_node(sector_bytes):
    return json.loads(sector_bytes.rstrip(b' ').decode('utf-8'))


def tach_la(la, bac_toi_da):
    if len(la['khoa']) <= bac_toi_da:
        return None
    giua = len(la['khoa']) // 2
    la_moi = {'loai': 'la', 'khoa': la['khoa'][giua:], 'gia_tri': la['gia_tri'][giua:], 'la_tiep': la['la_tiep']}
    la['khoa'] = la['khoa'][:giua]
    la['gia_tri'] = la['gia_tri'][:giua]
    khoa_tach = la_moi['khoa'][0]
    return (khoa_tach, la_moi)


def tao_goc_moi(khoa_tach, sector_trai, sector_phai):
    return {'loai': 'trong', 'khoa': [khoa_tach], 'con': [sector_trai, sector_phai]}


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


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=128)
la_goc = {'loai': 'la', 'khoa': [1, 2, 3, 4], 'gia_tri': ['a', 'b', 'c', 'd'], 'la_tiep': None}
khoa_tach, la_moi = tach_la(la_goc, 3)

dia.write(0, ma_hoa_node(la_goc, 128))
dia.write(1, ma_hoa_node(la_moi, 128))
dia.fsync()

goc_moi = tao_goc_moi(khoa_tach, 0, 1)
dia.write(2, ma_hoa_node(goc_moi, 128))
dia.fsync()

print(giai_ma_node(dia.read(2)))
```

```text title=readonly
{'loai': 'trong', 'khoa': [3], 'con': [0, 1]}
```

Lá GỐC (`[1,2,3,4]`) TÁCH tại `tach_la` — nửa ĐẦU (`[1,2]`) Ở LẠI
sector `0`, nửa SAU (`[3,4]`) THÀNH node mới Ở sector `1`.
`tao_goc_moi(3, 0, 1)` tạo root MỚI: khoá `[3]` (mọi khoá `< 3` Ở
`con[0]`=sector `0`, mọi khoá `>= 3` Ở `con[1]`=sector `1`), ghi VÀO
sector `2` — CÂY vừa TĂNG chiều cao TỪ một lá đơn ĐỘC thành MỘT root
có HAI con.
::::

::::example{#hai-la-doc-lap}
Đọc lại HAI lá TỪ sector `0` VÀ `1` — CẢ hai vẫn hoạt ĐỘNG độc lập,
`root` chỉ LƯU cách phân BIỆT chúng:

```python title=readonly
print(giai_ma_node(dia.read(0))['khoa'])
print(giai_ma_node(dia.read(1))['khoa'])
```

```text title=readonly
[1, 2]
[3, 4]
```

`con[0]=0` trỏ TỚI lá chứa khoá NHỎ (`[1, 2]`), `con[1]=1` trỏ TỚI
lá chứa khoá LỚN (`[3, 4]`) — ĐÚNG khớp `khoa=[3]` của root: mọi
khoá `< 3` Ở NHÁNH trái, `>= 3` Ở nhánh PHẢI.
::::

::::predict{#doan-so-luong-con commitOnce}
Root MỚI CÓ đúng `1` khoá (`khoa_tach=3`). SỐ lượng con trỏ (`con`)
của NÓ là bao nhiêu?

```python
print(len(goc_moi['con']))
```

:::opt{correct}
`2`
:::

:::opt
`1` — vì node TRONG chỉ CÓ đúng một khoá, NÊN cũng CHỈ có MỘT con
trỏ TƯƠNG ứng
::why
Gần đúng ở việc bạn nghĩ "MỘT khoá ↔ một con TRỎ" theo tỉ lệ 1-1 —
MỘT liên tưởng hợp lý NẾU node trong hoạt động GIỐNG node lá (`khoa`
VÀ `gia_tri` cùng ĐỘ dài).

Chỗ lệch: node TRONG LUÔN có **`len(khoa) + 1`** con trỏ — MỖI khoá
LÀ một "ranh GIỚI" phân chia HAI vùng LÂN cận, NÊN `n` ranh giới tạo
RA `n+1` vùng. VỚI đúng `1` khoá, CÓ `2` vùng (nhỏ hơn `3`, VÀ lớn
hơn HOẶC bằng `3`) — `2` con trỏ.
::
:::

:::opt
`0` — vì `tao_goc_moi` CHỈ tạo khung node, chưa THẬT sự gán con trỏ
NÀO
::why
Gần đúng ở việc bạn nghĩ TỚI một quy trình "tạo KHUNG rồi điền
SAU" — một MẪU thiết kế hợp lý CHO một số hệ thống KHÁC.

Chỗ lệch: `tao_goc_moi(khoa_tach, sector_trai, sector_phai)` NHẬN
VÀO CẢ hai con trỏ NGAY từ tham số, VÀ gán CHÚNG thẳng vào `'con':
[sector_trai, sector_phai]` — KHÔNG có bước "điền SAU" nào cả.
::
:::
::::

::::code{#viet_tao_goc_moi}
Hoàn thiện `tao_goc_moi(khoa_tach, sector_trai, sector_phai)` —
tạo node TRONG với đúng MỘT khoá VÀ hai con trỏ.

```python title=starter
import json


def ma_hoa_node(node, kich_thuoc):
    d = json.dumps(node).encode('utf-8')
    return d.ljust(kich_thuoc, b' ')


def giai_ma_node(sector_bytes):
    return json.loads(sector_bytes.rstrip(b' ').decode('utf-8'))


def tach_la(la, bac_toi_da):
    if len(la['khoa']) <= bac_toi_da:
        return None
    giua = len(la['khoa']) // 2
    la_moi = {'loai': 'la', 'khoa': la['khoa'][giua:], 'gia_tri': la['gia_tri'][giua:], 'la_tiep': la['la_tiep']}
    la['khoa'] = la['khoa'][:giua]
    la['gia_tri'] = la['gia_tri'][:giua]
    khoa_tach = la_moi['khoa'][0]
    return (khoa_tach, la_moi)


def tao_goc_moi(khoa_tach, sector_trai, sector_phai):
    ___


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


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=128)
la_goc = {'loai': 'la', 'khoa': [1, 2, 3, 4], 'gia_tri': ['a', 'b', 'c', 'd'], 'la_tiep': None}
khoa_tach, la_moi = tach_la(la_goc, 3)

dia.write(0, ma_hoa_node(la_goc, 128))
dia.write(1, ma_hoa_node(la_moi, 128))
dia.fsync()

goc_moi = tao_goc_moi(khoa_tach, 0, 1)
dia.write(2, ma_hoa_node(goc_moi, 128))
dia.fsync()

print(giai_ma_node(dia.read(2)))
```

```python title=solution
import json


def ma_hoa_node(node, kich_thuoc):
    d = json.dumps(node).encode('utf-8')
    return d.ljust(kich_thuoc, b' ')


def giai_ma_node(sector_bytes):
    return json.loads(sector_bytes.rstrip(b' ').decode('utf-8'))


def tach_la(la, bac_toi_da):
    if len(la['khoa']) <= bac_toi_da:
        return None
    giua = len(la['khoa']) // 2
    la_moi = {'loai': 'la', 'khoa': la['khoa'][giua:], 'gia_tri': la['gia_tri'][giua:], 'la_tiep': la['la_tiep']}
    la['khoa'] = la['khoa'][:giua]
    la['gia_tri'] = la['gia_tri'][:giua]
    khoa_tach = la_moi['khoa'][0]
    return (khoa_tach, la_moi)


def tao_goc_moi(khoa_tach, sector_trai, sector_phai):
    return {'loai': 'trong', 'khoa': [khoa_tach], 'con': [sector_trai, sector_phai]}


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


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=128)
la_goc = {'loai': 'la', 'khoa': [1, 2, 3, 4], 'gia_tri': ['a', 'b', 'c', 'd'], 'la_tiep': None}
khoa_tach, la_moi = tach_la(la_goc, 3)

dia.write(0, ma_hoa_node(la_goc, 128))
dia.write(1, ma_hoa_node(la_moi, 128))
dia.fsync()

goc_moi = tao_goc_moi(khoa_tach, 0, 1)
dia.write(2, ma_hoa_node(goc_moi, 128))
dia.fsync()

print(giai_ma_node(dia.read(2)))
```

```python title=test
def kiem_tra():
    dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=128)
    la_goc = {'loai': 'la', 'khoa': [1, 2, 3, 4], 'gia_tri': ['a', 'b', 'c', 'd'], 'la_tiep': None}
    khoa_tach, la_moi = tach_la(la_goc, 3)
    dia.write(0, ma_hoa_node(la_goc, 128))
    dia.write(1, ma_hoa_node(la_moi, 128))
    dia.fsync()
    return khoa_tach, tao_goc_moi(khoa_tach, 0, 1)

khoa_tach, root = kiem_tra()
assert root['loai'] == 'trong', "root la node trong"
assert root['khoa'] == [3], "root co dung mot khoa"
assert root['con'] == [0, 1], "root tro dung hai sector con"
assert len(root['con']) == len(root['khoa']) + 1, "so con luon bang so khoa cong 1"

root2 = tao_goc_moi(10, 5, 6)
assert root2 == {'loai': 'trong', 'khoa': [10], 'con': [5, 6]}, "hoat dong dung voi tham so khac"
```

:::hints
- kind: attention
  body: "Tra ve dict co loai='trong', khoa=[khoa_tach], con=[sector_trai, sector_phai]."
- kind: strategy
  body: "return {'loai': 'trong', 'khoa': [khoa_tach], 'con': [sector_trai, sector_phai]}"
- kind: one-line
  body: "return {'loai': 'trong', 'khoa': [khoa_tach], 'con': [sector_trai, sector_phai]}"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tra ve dict voi loai='trong', khoa=[khoa_tach], con=[sector_trai, sector_phai]
  requireAst:
  - kind: uses-name, target: khoa_tach, min: 3
  - kind: uses-name, target: sector_trai, min: 1
  - kind: uses-name, target: sector_phai, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\{'loai': 'trong', 'khoa': \[3\], 'con': \[0, 1\]\}\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cây ĐẦU tiên — MỘT root, hai LÁ. Chiều cao đã TĂNG từ `1` lên `2`.
NHƯNG tìm kiếm giờ CẦN đi QUA node trong TRƯỚC khi tới lá — điều
hướng THẾ nào?
::::

::::reflect{#nghi-lai}
`tao_goc_moi` tạo node TRONG đầu TIÊN — đúng một khoá, HAI con trỏ,
CÂY tăng chiều CAO từ một lá ĐƠN độc thành root+hai LÁ. Node trong
LUÔN có `len(khoa)+1` con — MỖI khoá LÀ một RANH giới. Giờ tìm MỘT
khoá KHÔNG còn đơn giản LÀ đọc MỘT lá nữa — phải BẮT đầu từ root,
SO sánh khoá để CHỌN đúng nhánh, RỒI mới TỚI lá. Điều hướng ĐÓ diễn
ra THẾ nào?
::::

::::checkpoint{mastery=0.8}
::::
