---
id: co-so-du-lieu.cay-cua-nguoi-thu-thu.chen-qua-dieu-huong
title: Chèn qua điều hướng
summary: "chen_khong_tach đi TỪ gốc xuống lá đúng (tái dùng bài 8), RỒI tìm vị trí chèn TRONG lá bằng một vòng while sắp xếp riêng, chèn khoá VÀ giá trị song song bằng list.insert. Hàm KHÔNG kiểm tra lá đã đầy chưa — nó luôn chèn, KHÔNG giới hạn, phần 'khi nào phải tách' LÀ việc của bài sau."
locale: vi
track: co-so-du-lieu
module: cay-cua-nguoi-thu-thu
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [db.insert-via-navigation]
requires: [db.search-from-root]
concepts: [db.insert-via-navigation]
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
Tìm kiếm (bài TRƯỚC) đi từ gốc xuống LÁ, dừng lại. CHÈN một khoá
MỚI cũng cần đi qua CON đường đó — nhưng dừng ở LÁ để LÀM gì?
::::

::::explain{#chen-khong-tach}
`chen_khong_tach` đi TỪ gốc xuống lá (TÁI dùng đúng cách điều hướng
bài TRƯỚC), rồi tìm VỊ trí chèn bằng một vòng `while` SẮP xếp
riêng, RỒI chèn khoá VÀ giá trị song SONG bằng `list.insert`:

```python title=readonly
def chon_nhanh(node_trong, khoa_can_tim):
    for i, k in enumerate(node_trong['khoa']):
        if khoa_can_tim < k:
            return node_trong['con'][i]
    return node_trong['con'][-1]


import json


def ma_hoa_node(node, kich_thuoc):
    return json.dumps(node).encode('utf-8').ljust(kich_thuoc, b' ')


def giai_ma_node(sector_bytes):
    return json.loads(sector_bytes.rstrip(b' ').decode('utf-8'))


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


def chen_khong_tach(dia, sector_goc, khoa_moi, gia_tri_moi):
    sector = sector_goc
    node = giai_ma_node(dia.read(sector))
    while node['loai'] == 'trong':
        sector = chon_nhanh(node, khoa_moi)
        node = giai_ma_node(dia.read(sector))
    vi_tri = 0
    while vi_tri < len(node['khoa']) and node['khoa'][vi_tri] < khoa_moi:
        vi_tri += 1
    node['khoa'].insert(vi_tri, khoa_moi)
    node['gia_tri'].insert(vi_tri, gia_tri_moi)
    dia.write(sector, ma_hoa_node(node, dia.kich_thuoc_sector))
    dia.fsync()


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
la1 = {'loai': 'la', 'khoa': [1, 2, 3], 'gia_tri': ['a', 'b', 'c'], 'la_tiep': 2}
la2 = {'loai': 'la', 'khoa': [5, 7, 9], 'gia_tri': ['e', 'g', 'i'], 'la_tiep': None}
dia.write(0, ma_hoa_node(root, 200))
dia.write(1, ma_hoa_node(la1, 200))
dia.write(2, ma_hoa_node(la2, 200))
dia.fsync()

chen_khong_tach(dia, 0, 4, 'd')
print(giai_ma_node(dia.read(1)))
```

```text title=readonly
{'loai': 'la', 'khoa': [1, 2, 3, 4], 'gia_tri': ['a', 'b', 'c', 'd'], 'la_tiep': 2}
```

`chen_khong_tach(dia, 0, 4, 'd')`: `chon_nhanh(root, 4)` chọn sector
`1` (`4 < 5`) — vòng `while` thứ HAI tìm `vi_tri=3` (`4` lớn hơn cả
`1`, `2`, `3`) — `khoa.insert(3, 4)` VÀ `gia_tri.insert(3, 'd')`
chèn ĐÚNG vào cuối, giữ THỨ tự tăng dần.
::::

::::example{#chen-vao-giua}
Chèn khoá `6` vào lá PHẢI (`la2`, khoá `[5, 7, 9]`) — vị trí chèn
KHÔNG phải cuối, mà Ở GIỮA:

```python title=readonly
chen_khong_tach(dia, 0, 6, 'f')
print(giai_ma_node(dia.read(2)))
```

```text title=readonly
{'loai': 'la', 'khoa': [5, 6, 7, 9], 'gia_tri': ['e', 'f', 'g', 'i'], 'la_tiep': None}
```

`6` lớn hơn `5` NHƯNG nhỏ hơn `7` — vòng `while` DỪNG Ở `vi_tri=1`,
`khoa.insert(1, 6)` chèn ĐÚNG giữa `5` VÀ `7`, giữ danh sách LUÔN
sắp xếp.
::::

::::predict{#doan-so-khoa-sau-chen commitOnce}
Lá `la1` (sector `1`) đang có ĐÚNG `3` khoá (`[1, 2, 3]`). Byte chèn
THÊM khoá `4`, rồi đếm SỐ khoá còn lại:

```python
chen_khong_tach(dia, 0, 4, 'd')
print(len(giai_ma_node(dia.read(1))['khoa']))
```

Dòng cuối in ra gì?

:::opt{correct}
`4`
:::

:::opt
`3` — vì `chen_khong_tach` kiểm tra lá đã ĐẦY chưa TRƯỚC khi chèn,
từ chối NẾU đã đủ `3` khoá
::why
Gần đúng ở việc bạn nghĩ TỚI một giới hạn "SỨC chứa" hợp lý cho một
trang đĩa — MỘT trực giác đúng cho hệ thống THẬT.

Chỗ lệch: TÊN hàm `chen_khong_tach` ("chèn KHÔNG tách") nói ĐÚNG
điều nó LÀM — KHÔNG có bất KỲ điều kiện kiểm tra sức chứa nào cả,
nó LUÔN chèn, lá lớn LÊN không giới hạn. Quyết định KHI nào cần tách
LÀ việc của một hàm KHÁC, bài sau.
::
:::

:::opt
Máy báo lỗi — vì lá đã ĐẦY, `list.insert` từ chối thêm PHẦN tử thứ
tư
::why
Gần đúng ở việc bạn nghĩ TỚI một ràng buộc hợp lý CHO cấu trúc dữ
liệu cố định KÍCH thước.

Chỗ lệch: `list.insert` của Python KHÔNG hề giới hạn độ dài — nó
CHÈN được bao nhiêu phần tử CŨNG được, KHÔNG raise lỗi nào. `khoa`
VÀ `gia_tri` chỉ LÀ list Python bình thường.
::
:::
::::

::::code{#viet_chen_khong_tach}
Hoàn thiện `chen_khong_tach(dia, sector_goc, khoa_moi, gia_tri_moi)`
— sau khi đã tìm ĐÚNG `vi_tri`, chèn khoá VÀ giá trị song song.

```python title=starter
def chon_nhanh(node_trong, khoa_can_tim):
    for i, k in enumerate(node_trong['khoa']):
        if khoa_can_tim < k:
            return node_trong['con'][i]
    return node_trong['con'][-1]


import json


def ma_hoa_node(node, kich_thuoc):
    return json.dumps(node).encode('utf-8').ljust(kich_thuoc, b' ')


def giai_ma_node(sector_bytes):
    return json.loads(sector_bytes.rstrip(b' ').decode('utf-8'))


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


def chen_khong_tach(dia, sector_goc, khoa_moi, gia_tri_moi):
    sector = sector_goc
    node = giai_ma_node(dia.read(sector))
    while node['loai'] == 'trong':
        sector = chon_nhanh(node, khoa_moi)
        node = giai_ma_node(dia.read(sector))
    vi_tri = 0
    while vi_tri < len(node['khoa']) and node['khoa'][vi_tri] < khoa_moi:
        vi_tri += 1
    ___
    dia.write(sector, ma_hoa_node(node, dia.kich_thuoc_sector))
    dia.fsync()


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
la1 = {'loai': 'la', 'khoa': [1, 2, 3], 'gia_tri': ['a', 'b', 'c'], 'la_tiep': 2}
la2 = {'loai': 'la', 'khoa': [5, 7, 9], 'gia_tri': ['e', 'g', 'i'], 'la_tiep': None}
dia.write(0, ma_hoa_node(root, 200))
dia.write(1, ma_hoa_node(la1, 200))
dia.write(2, ma_hoa_node(la2, 200))
dia.fsync()

chen_khong_tach(dia, 0, 4, 'd')
print(giai_ma_node(dia.read(1)))
```

```python title=solution
def chon_nhanh(node_trong, khoa_can_tim):
    for i, k in enumerate(node_trong['khoa']):
        if khoa_can_tim < k:
            return node_trong['con'][i]
    return node_trong['con'][-1]


import json


def ma_hoa_node(node, kich_thuoc):
    return json.dumps(node).encode('utf-8').ljust(kich_thuoc, b' ')


def giai_ma_node(sector_bytes):
    return json.loads(sector_bytes.rstrip(b' ').decode('utf-8'))


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


def chen_khong_tach(dia, sector_goc, khoa_moi, gia_tri_moi):
    sector = sector_goc
    node = giai_ma_node(dia.read(sector))
    while node['loai'] == 'trong':
        sector = chon_nhanh(node, khoa_moi)
        node = giai_ma_node(dia.read(sector))
    vi_tri = 0
    while vi_tri < len(node['khoa']) and node['khoa'][vi_tri] < khoa_moi:
        vi_tri += 1
    node['khoa'].insert(vi_tri, khoa_moi)
    node['gia_tri'].insert(vi_tri, gia_tri_moi)
    dia.write(sector, ma_hoa_node(node, dia.kich_thuoc_sector))
    dia.fsync()


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
la1 = {'loai': 'la', 'khoa': [1, 2, 3], 'gia_tri': ['a', 'b', 'c'], 'la_tiep': 2}
la2 = {'loai': 'la', 'khoa': [5, 7, 9], 'gia_tri': ['e', 'g', 'i'], 'la_tiep': None}
dia.write(0, ma_hoa_node(root, 200))
dia.write(1, ma_hoa_node(la1, 200))
dia.write(2, ma_hoa_node(la2, 200))
dia.fsync()

chen_khong_tach(dia, 0, 4, 'd')
print(giai_ma_node(dia.read(1)))
```

```python title=test
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
la1 = {'loai': 'la', 'khoa': [1, 2, 3], 'gia_tri': ['a', 'b', 'c'], 'la_tiep': 2}
la2 = {'loai': 'la', 'khoa': [5, 7, 9], 'gia_tri': ['e', 'g', 'i'], 'la_tiep': None}
dia.write(0, ma_hoa_node(root, 200))
dia.write(1, ma_hoa_node(la1, 200))
dia.write(2, ma_hoa_node(la2, 200))
dia.fsync()

chen_khong_tach(dia, 0, 4, 'd')
la1_sau = giai_ma_node(dia.read(1))
assert la1_sau['khoa'] == [1, 2, 3, 4], "chen vao cuoi la trai"
assert la1_sau['gia_tri'] == ['a', 'b', 'c', 'd'], "gia tri di dung theo khoa"

chen_khong_tach(dia, 0, 6, 'f')
la2_sau = giai_ma_node(dia.read(2))
assert la2_sau['khoa'] == [5, 6, 7, 9], "chen vao giua la phai"
assert la2_sau['gia_tri'] == ['e', 'f', 'g', 'i'], "gia tri chen dung vi tri"

chen_khong_tach(dia, 0, 0, 'z')
la1_sau2 = giai_ma_node(dia.read(1))
assert la1_sau2['khoa'] == [0, 1, 2, 3, 4], "chen vao dau la trai"
```

:::hints
- kind: attention
  body: "Chen node['khoa'] va node['gia_tri'] tai vi_tri -- hai dong list.insert."
- kind: strategy
  body: "node['khoa'].insert(vi_tri, khoa_moi); node['gia_tri'].insert(vi_tri, gia_tri_moi)"
- kind: one-line
  body: "node['khoa'].insert(vi_tri, khoa_moi); node['gia_tri'].insert(vi_tri, gia_tri_moi)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai insert khoa_moi va gia_tri_moi vao dung vi_tri trong ca hai list
  requireAst:
  - kind: uses-name, target: vi_tri, min: 4
  - kind: uses-name, target: khoa_moi, min: 3
  - kind: uses-name, target: node, min: 8
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\{'loai': 'la', 'khoa': \[1, 2, 3, 4\], 'gia_tri': \['a', 'b', 'c', 'd'\], 'la_tiep': 2\}\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chèn qua điều hướng — lá lớn LÊN không giới hạn. NHƯNG một lá KHÔNG
thể phình mãi TRÊN một sector CÓ hạn — khi nào PHẢI tách, và chuyện
GÌ xảy ra với node CHA?
::::

::::reflect{#nghi-lai}
`chen_khong_tach` LUÔN chèn, KHÔNG hề kiểm tra sức chứa — TÊN hàm
đã nói THẲNG điều đó. Một lá thật SỰ chỉ có chỗ CHO một số khoá
GIỚI hạn (một sector CÓ kích thước cố định). Khi lá ĐẦY, nó phải
TÁCH làm đôi — VÀ khoá phân tách MỚI phải được đẩy LÊN node cha,
giống HỆT bài `day-khoa-len-cha` đã LÀM cho lần tách ĐẦU tiên. Lần
NÀY, node cha ĐÃ tồn tại từ trước — cập nhật NÓ diễn ra thế nào?
::::

::::checkpoint{mastery=0.8}
::::
