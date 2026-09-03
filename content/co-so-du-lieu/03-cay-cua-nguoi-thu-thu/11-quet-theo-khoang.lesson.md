---
id: co-so-du-lieu.cay-cua-nguoi-thu-thu.quet-theo-khoang
title: Quét theo khoảng
summary: "quet_khoang tìm LÁ bắt đầu bằng cách điều hướng TỚI cận dưới (tái dùng chon_nhanh), rồi đi theo con trỏ la_tiep TỪ lá này sang lá KHÁC, lọc mỗi khoá trong khoảng [lo, hi]. Cận dưới KHÔNG cần là một khoá thật đang tồn tại — điều hướng vẫn tìm đúng lá nó SẼ nằm trong nếu có."
locale: vi
track: co-so-du-lieu
module: cay-cua-nguoi-thu-thu
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [db.range-scan-via-la-tiep]
requires: [db.split-propagates-once]
concepts: [db.range-scan-via-la-tiep]
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
`tim_kiem` tìm MỘT khoá. NHƯNG "mọi khoá từ `3` đến `9`" LÀ nhiều
khoá LIÊN tục — làm sao lấy RA cả một khoảng, không CHỈ một điểm?
::::

::::explain{#quet-khoang}
`quet_khoang` tìm LÁ bắt đầu bằng cách điều hướng TỚI cận dưới
(`lo`, TÁI dùng `chon_nhanh`), rồi đi THEO con trỏ `la_tiep` từ lá
NÀY sang lá khác, lọc mỗi khoá TRONG khoảng `[lo, hi]`:

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


def tim_la_bat_dau(dia, sector_goc, lo):
    sector = sector_goc
    node = giai_ma_node(dia.read(sector))
    while node['loai'] == 'trong':
        sector = chon_nhanh(node, lo)
        node = giai_ma_node(dia.read(sector))
    return sector


def quet_khoang(dia, sector_goc, lo, hi):
    sector = tim_la_bat_dau(dia, sector_goc, lo)
    ket_qua = []
    while sector is not None:
        la = giai_ma_node(dia.read(sector))
        for i, k in enumerate(la['khoa']):
            if lo <= k <= hi:
                ket_qua.append((k, la['gia_tri'][i]))
        sector = la['la_tiep']
    return ket_qua


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
root = {'loai': 'trong', 'khoa': [3, 5], 'con': [1, 3, 2]}
la1 = {'loai': 'la', 'khoa': [1, 2], 'gia_tri': ['a', 'b'], 'la_tiep': 3}
la3 = {'loai': 'la', 'khoa': [3, 4], 'gia_tri': ['c', 'd'], 'la_tiep': 2}
la2 = {'loai': 'la', 'khoa': [5, 7, 9], 'gia_tri': ['e', 'g', 'i'], 'la_tiep': None}
dia.write(0, ma_hoa_node(root, 200))
dia.write(1, ma_hoa_node(la1, 200))
dia.write(3, ma_hoa_node(la3, 200))
dia.write(2, ma_hoa_node(la2, 200))
dia.fsync()

print(quet_khoang(dia, 0, 2, 7))
```

```text title=readonly
[(2, 'b'), (3, 'c'), (4, 'd'), (5, 'e'), (7, 'g')]
```

`quet_khoang(dia, 0, 2, 7)`: `tim_la_bat_dau` điều hướng TỚI sector
`1` (chứa `2`). Quét sector `1` → khớp `2`. `la_tiep=3` → quét
sector `3` → khớp `3`, `4`. `la_tiep=2` → quét sector `2` → khớp
`5`, `7` (loại `9`, VÌ `9 > hi=7`). `la_tiep=None` → dừng. KẾT quả
đi QUA đúng ba lá, theo thứ tự.
::::

::::example{#khoang-trong-mot-la}
Một khoảng NẰM gọn trong MỘT lá duy nhất — quét dừng NGAY sau lá
đầu tiên:

```python title=readonly
print(quet_khoang(dia, 0, 1, 2))
```

```text title=readonly
[(1, 'a'), (2, 'b')]
```

`tim_la_bat_dau` điều hướng TỚI sector `1` (`[1, 2]`) — cả hai khoá
ĐỀU khớp `[1, 2]`. `la_tiep=3` VẪN được đọc (vòng `while` LUÔN đọc
lá tiếp theo TRƯỚC khi kiểm tra), NHƯNG sector `3` chứa `[3, 4]` —
cả hai đều LỚN hơn `hi=2`, KHÔNG khớp gì, VÒNG lặp tiếp tục đọc
sector `2`, cũng KHÔNG khớp, rồi dừng Ở `la_tiep=None`.
::::

::::predict{#doan-khoang-khong-ton-tai commitOnce}
Byte quét khoảng bắt đầu TẠI `8` — một giá trị KHÔNG phải khoá thật
nào cả:

```python
print(quet_khoang(dia, 0, 8, 20))
```

Dòng cuối in ra gì?

:::opt{correct}
`[(9, 'i')]`
:::

:::opt
`[]` — vì `8` không phải MỘT khoá thật sự tồn tại TRONG cây, nên
khoảng bắt đầu TẠI một chỗ "không có GÌ"
::why
Gần đúng ở việc bạn nghĩ TỚI việc `lo` phải LÀ một khoá có THẬT để
"neo" điểm bắt ĐẦU — một trực giác hợp lý VỀ cách chỉ mục hoạt
động.

Chỗ lệch: `tim_la_bat_dau` gọi `chon_nhanh` TÌM lá `8` SẼ nằm
TRONG, nếu CÓ — trên root NÀY, `8` không nhỏ hơn `3` cũng KHÔNG
nhỏ hơn `5`, nên rơi VỀ `con[-1]` (sector `2`, chứa `[5, 7, 9]`).
Quét sector `2` lọc `lo <= k <= hi` — chỉ `9` khớp (`8 <= 9 <= 20`),
`5` VÀ `7` bị loại vì nhỏ hơn `lo`.
::
:::

:::opt
`[(5, 'e'), (7, 'g'), (9, 'i')]` — vì quét bắt đầu Ở lá chứa khoá
GẦN `8` nhất, rồi trả VỀ nguyên cả lá đó
::why
Gần đúng ở việc bạn xác định ĐÚNG lá bắt đầu (sector `2`) — MỘT
bước quan trọng đã ĐÚNG.

Chỗ lệch: vòng lặp KHÔNG trả về nguyên lá — mỗi khoá VẪN phải qua
kiểm tra `if lo <= k <= hi` riêng LẺ. `5` VÀ `7` đều nhỏ hơn
`lo=8`, bị LOẠI ngay trong lá ĐẦU tiên, chỉ `9` sống SÓT.
::
:::
::::

::::code{#viet_quet_khoang}
Hoàn thiện `quet_khoang(dia, sector_goc, lo, hi)` — lọc khoá TRONG
khoảng `[lo, hi]` khi quét MỖI lá.

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


def tim_la_bat_dau(dia, sector_goc, lo):
    sector = sector_goc
    node = giai_ma_node(dia.read(sector))
    while node['loai'] == 'trong':
        sector = chon_nhanh(node, lo)
        node = giai_ma_node(dia.read(sector))
    return sector


def quet_khoang(dia, sector_goc, lo, hi):
    sector = tim_la_bat_dau(dia, sector_goc, lo)
    ket_qua = []
    while sector is not None:
        la = giai_ma_node(dia.read(sector))
        for i, k in enumerate(la['khoa']):
            ___
        sector = la['la_tiep']
    return ket_qua


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
root = {'loai': 'trong', 'khoa': [3, 5], 'con': [1, 3, 2]}
la1 = {'loai': 'la', 'khoa': [1, 2], 'gia_tri': ['a', 'b'], 'la_tiep': 3}
la3 = {'loai': 'la', 'khoa': [3, 4], 'gia_tri': ['c', 'd'], 'la_tiep': 2}
la2 = {'loai': 'la', 'khoa': [5, 7, 9], 'gia_tri': ['e', 'g', 'i'], 'la_tiep': None}
dia.write(0, ma_hoa_node(root, 200))
dia.write(1, ma_hoa_node(la1, 200))
dia.write(3, ma_hoa_node(la3, 200))
dia.write(2, ma_hoa_node(la2, 200))
dia.fsync()

print(quet_khoang(dia, 0, 2, 7))
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


def tim_la_bat_dau(dia, sector_goc, lo):
    sector = sector_goc
    node = giai_ma_node(dia.read(sector))
    while node['loai'] == 'trong':
        sector = chon_nhanh(node, lo)
        node = giai_ma_node(dia.read(sector))
    return sector


def quet_khoang(dia, sector_goc, lo, hi):
    sector = tim_la_bat_dau(dia, sector_goc, lo)
    ket_qua = []
    while sector is not None:
        la = giai_ma_node(dia.read(sector))
        for i, k in enumerate(la['khoa']):
            if lo <= k <= hi:
                ket_qua.append((k, la['gia_tri'][i]))
        sector = la['la_tiep']
    return ket_qua


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
root = {'loai': 'trong', 'khoa': [3, 5], 'con': [1, 3, 2]}
la1 = {'loai': 'la', 'khoa': [1, 2], 'gia_tri': ['a', 'b'], 'la_tiep': 3}
la3 = {'loai': 'la', 'khoa': [3, 4], 'gia_tri': ['c', 'd'], 'la_tiep': 2}
la2 = {'loai': 'la', 'khoa': [5, 7, 9], 'gia_tri': ['e', 'g', 'i'], 'la_tiep': None}
dia.write(0, ma_hoa_node(root, 200))
dia.write(1, ma_hoa_node(la1, 200))
dia.write(3, ma_hoa_node(la3, 200))
dia.write(2, ma_hoa_node(la2, 200))
dia.fsync()

print(quet_khoang(dia, 0, 2, 7))
```

```python title=test
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
root = {'loai': 'trong', 'khoa': [3, 5], 'con': [1, 3, 2]}
la1 = {'loai': 'la', 'khoa': [1, 2], 'gia_tri': ['a', 'b'], 'la_tiep': 3}
la3 = {'loai': 'la', 'khoa': [3, 4], 'gia_tri': ['c', 'd'], 'la_tiep': 2}
la2 = {'loai': 'la', 'khoa': [5, 7, 9], 'gia_tri': ['e', 'g', 'i'], 'la_tiep': None}
dia.write(0, ma_hoa_node(root, 200))
dia.write(1, ma_hoa_node(la1, 200))
dia.write(3, ma_hoa_node(la3, 200))
dia.write(2, ma_hoa_node(la2, 200))
dia.fsync()

assert quet_khoang(dia, 0, 2, 7) == [(2, 'b'), (3, 'c'), (4, 'd'), (5, 'e'), (7, 'g')], "khoang qua nhieu la"
assert quet_khoang(dia, 0, 1, 2) == [(1, 'a'), (2, 'b')], "khoang trong mot la"
assert quet_khoang(dia, 0, 8, 20) == [(9, 'i')], "cai duoi khong phai khoa that"
assert quet_khoang(dia, 0, 100, 200) == [], "khoang qua lon, khong khop gi"
assert quet_khoang(dia, 0, 1, 9) == [(1, 'a'), (2, 'b'), (3, 'c'), (4, 'd'), (5, 'e'), (7, 'g'), (9, 'i')], "toan bo cay"
```

:::hints
- kind: attention
  body: "Neu lo <= k <= hi, ket_qua.append((k, la['gia_tri'][i])) -- mot dong."
- kind: strategy
  body: "if lo <= k <= hi: ket_qua.append((k, la['gia_tri'][i]))"
- kind: one-line
  body: "if lo <= k <= hi: ket_qua.append((k, la['gia_tri'][i]))"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai append (k, la['gia_tri'][i]) vao ket_qua khi lo <= k <= hi
  requireAst:
  - kind: uses-name, target: hi, min: 1
  - kind: uses-name, target: k, min: 3
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[\(2, 'b'\), \(3, 'c'\), \(4, 'd'\), \(5, 'e'\), \(7, 'g'\)\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tìm kiếm, chèn, tách, VÀ giờ quét theo khoảng — bốn mảnh ghép của
một B+Tree THẬT sự. Ghép tất cả LẠI, cây LỚN lên qua NHIỀU lần
tách, sẽ trông thế nào?
::::

::::reflect{#nghi-lai}
`quet_khoang` KHÔNG cần `lo` phải LÀ một khoá thật đang TỒN tại —
điều hướng VẪN tìm đúng lá nó SẼ nằm trong, nếu CÓ. `la_tiep` biến
một cây thành MỘT danh sách liên kết Ở tầng lá, cho phép quét
KHOẢNG mà KHÔNG cần đi lại TỪ gốc mỗi lần. Tìm kiếm, chèn, tách LAN
lên cha, VÀ quét khoảng — bốn kỹ năng ĐÃ đủ. Ghép chúng LẠI thành
một câu chuyện DUY nhất, cây lớn LÊN qua nhiều lần TÁCH, sẽ ra sao?
::::

::::checkpoint{mastery=0.8}
::::
