---
id: co-so-du-lieu.cay-cua-nguoi-thu-thu.tim-tu-goc-xuong-la
title: Tìm từ gốc xuống lá
summary: "tim_kiem đi TỪ node gốc XUỐNG, gọi chon_nhanh MỖI lần gặp node trong — dừng ĐÚNG khi tới một node lá, RỒI quét khoá TRONG lá đó (bài 3). Khoá bằng đúng khoá phân tách tìm ĐƯỢC bình thường — nó vẫn LÀ một khoá thật, không phải giá trị 'bị dùng riêng' cho việc điều hướng."
locale: vi
track: co-so-du-lieu
module: cay-cua-nguoi-thu-thu
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [db.search-from-root]
requires: [db.internal-node-format]
concepts: [db.search-from-root]
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
`chon_nhanh` (bài TRƯỚC) chọn ĐÚNG một nhánh — MỘT bước. Đi TỪ gốc
xuống TẬN lá, có THỂ cần NHIỀU bước như VẬY — làm sao GHÉP chúng
lại?
::::

::::explain{#tim-kiem-tron-ven}
`tim_kiem` LẶP `chon_nhanh` cho tới khi GẶP một node LÁ (`loai ==
'la'`), RỒI quét khoá TRONG lá đó (bài `tim-trong-mot-la`):

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


def tim_kiem(dia, sector_goc, khoa_can_tim):
    sector = sector_goc
    node = giai_ma_node(dia.read(sector))
    while node['loai'] == 'trong':
        sector = chon_nhanh(node, khoa_can_tim)
        node = giai_ma_node(dia.read(sector))
    for i, k in enumerate(node['khoa']):
        if k == khoa_can_tim:
            return node['gia_tri'][i]
    return None


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
la1 = {'loai': 'la', 'khoa': [1, 2, 3], 'gia_tri': ['a', 'b', 'c'], 'la_tiep': 2}
la2 = {'loai': 'la', 'khoa': [5, 7, 9], 'gia_tri': ['e', 'g', 'i'], 'la_tiep': None}
dia.write(0, ma_hoa_node(root, 200))
dia.write(1, ma_hoa_node(la1, 200))
dia.write(2, ma_hoa_node(la2, 200))
dia.fsync()

print(tim_kiem(dia, 0, 3))
print(tim_kiem(dia, 0, 7))
```

```text title=readonly
c
g
```

`tim_kiem(dia, 0, 3)`: sector `0` LÀ root (TRONG), `chon_nhanh` chọn
sector `1` (`3 < 5`) — sector `1` LÀ lá, quét khớp `k=3` Ở chỉ số
`2`, trả về `gia_tri[2]='c'`. `tim_kiem(dia, 0, 7)`: root chọn sector
`2` (`7` KHÔNG nhỏ hơn `5`), lá `2` khớp `k=7` — trả VỀ `'g'`.
::::

::::example{#khoa-khong-ton-tai}
Tìm khoá `4` (KHÔNG hề CÓ) — vẫn ĐI đúng NHÁNH, chỉ LÀ quét KHÔNG
khớp gì:

```python title=readonly
print(tim_kiem(dia, 0, 4))
```

```text title=readonly
None
```

`4 < 5` ĐÚNG — ĐI vào sector `1` (`[1, 2, 3]`), quét HẾT KHÔNG khớp
`4` — trả VỀ `None`, ĐÚNG như `tim_trong_la` (bài `3`) đã LÀM.
::::

::::predict{#doan-khoa-la-diem-phan-tach commitOnce}
Byte tìm ĐÚNG khoá phân tách của root (`5`) — GIÁ trị này CŨNG là
khoá ĐẦU tiên THẬT của lá phải:

```python
print(tim_kiem(dia, 0, 5))
```

Dòng cuối in ra gì?

:::opt{correct}
`e`
:::

:::opt
`None` — vì khoá `5` LÀ khoá PHÂN tách TRONG node gốc, KHÔNG phải
một khoá THẬT trong lá NÀO
::why
Gần đúng ở việc bạn nghĩ khoá phân TÁCH LÀ một giá trị "ĐẶC biệt",
riêng CHO việc điều hướng — MỘT trực giác hợp lý VỀ vai trò khác
NHAU giữa node trong VÀ node lá.

Chỗ lệch: `chon_nhanh(root, 5)` (bài TRƯỚC) trả VỀ nhánh PHẢI —
sector `2` (`la2`) — RỒI quét khoá `[5, 7, 9]` của `la2` khớp NGAY
`k=5` Ở chỉ số `0`, trả VỀ `gia_tri[0]='e'`. `5` LÀ một khoá THẬT
SỰ tồn tại (`la2`'s khoá ĐẦU), CHỈ tình cờ TRÙNG với khoá phân tách.
::
:::

:::opt
`i` — vì tìm THEO khoá phân TÁCH luôn trả VỀ giá trị của khoá CUỐI
cùng trong nhánh được CHỌN
::why
Gần đúng ở việc bạn nghĩ TỚI một quy TẮC "khoá phân tách ↔ khoá
cuối" — MỘT liên tưởng hợp lý dựa TRÊN vị trí của `5` (đầu nhánh).

Chỗ lệch: vòng lặp QUÉT lá `la2` TỪNG khoá MỘT (`5`, `7`, `9`) VÀ
DỪNG NGAY khi khớp — `5` khớp Ở chỉ số `0` NGAY lập TỨC, trả về
`gia_tri[0]='e'`, KHÔNG hề đi tiếp TỚI `i` (giá trị của khoá `9`).
::
:::
::::

::::code{#viet_tim_kiem}
Hoàn thiện `tim_kiem(dia, sector_goc, khoa_can_tim)` — sau khi đã
tới LÁ, quét khớp khoá TRẢ về giá trị.

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


def tim_kiem(dia, sector_goc, khoa_can_tim):
    sector = sector_goc
    node = giai_ma_node(dia.read(sector))
    while node['loai'] == 'trong':
        sector = chon_nhanh(node, khoa_can_tim)
        node = giai_ma_node(dia.read(sector))
    for i, k in enumerate(node['khoa']):
        ___
    return None


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
la1 = {'loai': 'la', 'khoa': [1, 2, 3], 'gia_tri': ['a', 'b', 'c'], 'la_tiep': 2}
la2 = {'loai': 'la', 'khoa': [5, 7, 9], 'gia_tri': ['e', 'g', 'i'], 'la_tiep': None}
dia.write(0, ma_hoa_node(root, 200))
dia.write(1, ma_hoa_node(la1, 200))
dia.write(2, ma_hoa_node(la2, 200))
dia.fsync()
print(tim_kiem(dia, 0, 3), tim_kiem(dia, 0, 7), tim_kiem(dia, 0, 4))
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


def tim_kiem(dia, sector_goc, khoa_can_tim):
    sector = sector_goc
    node = giai_ma_node(dia.read(sector))
    while node['loai'] == 'trong':
        sector = chon_nhanh(node, khoa_can_tim)
        node = giai_ma_node(dia.read(sector))
    for i, k in enumerate(node['khoa']):
        if k == khoa_can_tim:
            return node['gia_tri'][i]
    return None


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
la1 = {'loai': 'la', 'khoa': [1, 2, 3], 'gia_tri': ['a', 'b', 'c'], 'la_tiep': 2}
la2 = {'loai': 'la', 'khoa': [5, 7, 9], 'gia_tri': ['e', 'g', 'i'], 'la_tiep': None}
dia.write(0, ma_hoa_node(root, 200))
dia.write(1, ma_hoa_node(la1, 200))
dia.write(2, ma_hoa_node(la2, 200))
dia.fsync()
print(tim_kiem(dia, 0, 3), tim_kiem(dia, 0, 7), tim_kiem(dia, 0, 4))
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
assert tim_kiem(dia, 0, 1) == 'a', "khoa dau nhanh trai"
assert tim_kiem(dia, 0, 3) == 'c', "khoa cuoi nhanh trai"
assert tim_kiem(dia, 0, 5) == 'e', "khoa bang khoa phan tach -- nhanh phai"
assert tim_kiem(dia, 0, 9) == 'i', "khoa cuoi nhanh phai"
assert tim_kiem(dia, 0, 4) is None, "khoa khong ton tai"
assert tim_kiem(dia, 0, 100) is None, "khoa qua lon"
```

:::hints
- kind: attention
  body: "Neu k == khoa_can_tim, return node['gia_tri'][i] -- mot dong."
- kind: strategy
  body: "if k == khoa_can_tim: return node['gia_tri'][i]"
- kind: one-line
  body: "if k == khoa_can_tim: return node['gia_tri'][i]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai return node['gia_tri'][i] khi k == khoa_can_tim
  requireAst:
  - kind: uses-name, target: khoa_can_tim, min: 3
  - kind: uses-name, target: i, min: 2
  - kind: uses-name, target: k, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^c g None\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tìm kiếm ĐA tầng — TỪ gốc xuống LÁ, dừng đúng LÚC. NHƯNG tìm ĐƯỢC
mới LÀ một nửa — CHÈN một khoá MỚI cũng CẦN đi qua đúng con ĐƯỜNG
đó trước, chèn Ở đâu?
::::

::::reflect{#nghi-lai}
`tim_kiem` lặp `chon_nhanh` TỚI khi gặp lá — MỘT khoá bằng đúng khoá
phân tách VẪN tìm ĐƯỢC bình thường, nó KHÔNG hề "bị dùng RIÊNG" cho
việc điều hướng. Tìm kiếm giờ đi ĐA tầng — NHƯNG chèn MỘT khoá mới
cũng CẦN đi qua đúng con đường ĐÓ trước khi biết chèn VÀO lá nào.
Chèn qua điều HƯỚNG diễn ra thế nào?
::::

::::checkpoint{mastery=0.8}
::::
