---
id: co-so-du-lieu.cay-cua-nguoi-thu-thu.boss-cay-b-cong
title: "BOSS — Cây B+ cộng"
summary: "Ghép TRỌN q03: chen_va_tach_neu_can điều hướng, chèn, kiểm tra tràn, tách (nếu cần), VÀ cập nhật cha — qua HAI lần chèn gây tách, cây vẫn giữ đúng bất biến, tim_kiem VÀ quet_khoang đều đúng trên MỌI lá, kể cả khoá từng được dùng LÀM khoá phân tách."
locale: vi
track: co-so-du-lieu
module: cay-cua-nguoi-thu-thu
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.range-scan-via-la-tiep]
concepts: [db.boss-q03]
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
Tìm kiếm, chèn, tách LAN lên cha, quét theo khoảng — bốn mảnh RIÊNG
lẻ. Ghép TẤT cả vào một hàm DUY nhất, cây lớn lên qua NHIỀU lần
chèn, trông THẾ nào?
::::

::::explain{#ghep-tron-ven}
`chen_va_tach_neu_can` điều hướng (bài `tim-tu-goc-xuong-la`), chèn
(bài `chen-qua-dieu-huong`), kiểm TRA tràn — nếu KHÔNG tràn thì
dừng LUÔN, nếu TRÀN thì tách (bài `la-day-phai-tach`) VÀ cập nhật
cha (bài `tach-lan-len-cha`):

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


def chen_va_tach_neu_can(dia, sector_goc, khoa_moi, gia_tri_moi, bac_toi_da, sector_trong):
    root = giai_ma_node(dia.read(sector_goc))
    sector = chon_nhanh(root, khoa_moi)
    la = giai_ma_node(dia.read(sector))
    vi_tri = 0
    while vi_tri < len(la['khoa']) and la['khoa'][vi_tri] < khoa_moi:
        vi_tri += 1
    la['khoa'].insert(vi_tri, khoa_moi)
    la['gia_tri'].insert(vi_tri, gia_tri_moi)
    if len(la['khoa']) <= bac_toi_da:
        dia.write(sector, ma_hoa_node(la, dia.kich_thuoc_sector))
        dia.fsync()
        return
    giua = len(la['khoa']) // 2
    la_moi = {'loai': 'la', 'khoa': la['khoa'][giua:], 'gia_tri': la['gia_tri'][giua:], 'la_tiep': la['la_tiep']}
    la['khoa'] = la['khoa'][:giua]
    la['gia_tri'] = la['gia_tri'][:giua]
    khoa_tach = la_moi['khoa'][0]
    la['la_tiep'] = sector_trong
    dia.write(sector, ma_hoa_node(la, dia.kich_thuoc_sector))
    dia.write(sector_trong, ma_hoa_node(la_moi, dia.kich_thuoc_sector))
    dia.fsync()
    vi_tri_goc = 0
    while vi_tri_goc < len(root['khoa']) and root['khoa'][vi_tri_goc] < khoa_tach:
        vi_tri_goc += 1
    root['khoa'].insert(vi_tri_goc, khoa_tach)
    root['con'].insert(vi_tri_goc + 1, sector_trong)
    dia.write(sector_goc, ma_hoa_node(root, dia.kich_thuoc_sector))
    dia.fsync()


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
la1 = {'loai': 'la', 'khoa': [1, 2, 3], 'gia_tri': ['a', 'b', 'c'], 'la_tiep': 2}
la2 = {'loai': 'la', 'khoa': [5, 7, 9], 'gia_tri': ['e', 'g', 'i'], 'la_tiep': None}
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
dia.write(0, ma_hoa_node(root, 200))
dia.write(1, ma_hoa_node(la1, 200))
dia.write(2, ma_hoa_node(la2, 200))
dia.fsync()

chen_va_tach_neu_can(dia, 0, 4, 'd', 3, 3)
chen_va_tach_neu_can(dia, 0, 0, 'z', 3, 5)
chen_va_tach_neu_can(dia, 0, -1, 'y', 3, 4)
print(giai_ma_node(dia.read(0)))
print(quet_khoang(dia, 0, 0, 5))
```

```text title=readonly
{'loai': 'trong', 'khoa': [1, 3, 5], 'con': [1, 4, 3, 2]}
[(0, 'z'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd'), (5, 'e')]
```

Ba lần chèn: `4` gây TÁCH lần đầu (`la1` `[1,2,3,4]` → `[1,2]` /
`[3,4]` sector `3`, cha thành `{khoa:[3,5], con:[1,3,2]}`). `0`
KHÔNG gây tách (`la1` mới còn `[0,1,2]`, chỉ `3` khoá, KHÔNG vượt
`bac_toi_da=3`). `-1` gây TÁCH lần hai (`[-1,0,1,2]` → `[-1,0]` /
`[1,2]` sector `4`, cha thành `{khoa:[1,3,5], con:[1,4,3,2]}`) —
CHIỀU cao cây VẪN giữ nguyên, chỉ CÓ thêm lá mới, quét theo khoảng
đi ĐÚNG qua cả bốn lá theo thứ TỰ, dù chúng nằm rải RÁC trên đĩa.
::::

::::example{#tim-sau-hai-lan-tach}
Sau hai lần tách, tìm khoá `9` (chưa TỪNG đụng tới) VÀ `100` (KHÔNG
tồn tại):

```python title=readonly
print(tim_kiem(dia, 0, 9))
print(tim_kiem(dia, 0, 100))
```

```text title=readonly
i
None
```

Cả HAI vẫn đúng — `9` VẪN ở sector `2` (chưa hề bị chạm TỚI qua hai
lần tách), `100` không khớp khoá NÀO trong bất kỳ lá nào, quét HẾT
rồi trả về `None`. Điều hướng CHÍNH xác dù cây ĐÃ đổi hình dạng hai
LẦN.
::::

::::predict{#doan-khoa-tung-la-phan-tach commitOnce}
Khoá `3` từng được dùng LÀM khoá phân tách trong lần TÁCH đầu tiên
(`cha['khoa']` giờ chứa `3`). Byte tìm ĐÚNG khoá đó SAU cả hai lần
tách:

```python
print(tim_kiem(dia, 0, 3))
```

Dòng cuối in ra gì?

:::opt{correct}
`c`
:::

:::opt
`None` — vì `3` từng được dùng LÀM khoá phân tách trong node gốc Ở
lần tách đầu TIÊN, nên nó không còn LÀ một khoá thật trong lá NÀO
nữa
::why
Gần đúng ở việc bạn nghĩ TỚI vai trò "đặc biệt" của khoá PHÂN tách
— giống Y hệt trực giác bài `tim-tu-goc-xuong-la` từng gặp.

Chỗ lệch: `khoa_tach` LÀ một BẢN sao được đẩy LÊN cha để điều
hướng — bản GỐC vẫn nằm nguyên VẸN trong lá của nó (sector `3`,
`khoa=[3, 4]`), hoàn TOÀN tìm được bình thường. `chon_nhanh({'khoa':
[1, 3, 5], ...}, 3)`: `3<1`? không. `3<3`? không. `3<5`? có → trả
về `con[2]`=sector `3`, quét khớp `k=3` Ở chỉ số `0`, trả VỀ
`gia_tri[0]='c'`.
::
:::

:::opt
`d` — vì tìm kiếm rơi vào ĐÚNG lá, nhưng lệch một VỊ trí, khớp khoá
KẾ tiếp thay VÌ khoá cần tìm
::why
Gần đúng ở việc bạn xác định ĐÚNG lá đích (sector `3`) — một bước
QUAN trọng đã đúng.

Chỗ lệch: vòng quét lá SO sánh `k == khoa_can_tim` cho TỪNG khoá
riêng lẻ, DỪNG ngay khi khớp — `3` khớp NGAY ở chỉ số `0`
(`khoa=[3, 4]`), trả VỀ `gia_tri[0]='c'`, không hề đi TIẾP tới `4`
(giá trị `'d'`).
::
:::
::::

::::code{#viet_chen_va_tach_neu_can}
Hoàn thiện `chen_va_tach_neu_can` — sau khi đã tách (nếu CẦN), cập
nhật node cha giống HỆT bài `tach-lan-len-cha`.

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


def chen_va_tach_neu_can(dia, sector_goc, khoa_moi, gia_tri_moi, bac_toi_da, sector_trong):
    root = giai_ma_node(dia.read(sector_goc))
    sector = chon_nhanh(root, khoa_moi)
    la = giai_ma_node(dia.read(sector))
    vi_tri = 0
    while vi_tri < len(la['khoa']) and la['khoa'][vi_tri] < khoa_moi:
        vi_tri += 1
    la['khoa'].insert(vi_tri, khoa_moi)
    la['gia_tri'].insert(vi_tri, gia_tri_moi)
    if len(la['khoa']) <= bac_toi_da:
        dia.write(sector, ma_hoa_node(la, dia.kich_thuoc_sector))
        dia.fsync()
        return
    giua = len(la['khoa']) // 2
    la_moi = {'loai': 'la', 'khoa': la['khoa'][giua:], 'gia_tri': la['gia_tri'][giua:], 'la_tiep': la['la_tiep']}
    la['khoa'] = la['khoa'][:giua]
    la['gia_tri'] = la['gia_tri'][:giua]
    khoa_tach = la_moi['khoa'][0]
    la['la_tiep'] = sector_trong
    dia.write(sector, ma_hoa_node(la, dia.kich_thuoc_sector))
    dia.write(sector_trong, ma_hoa_node(la_moi, dia.kich_thuoc_sector))
    dia.fsync()
    vi_tri_goc = 0
    while vi_tri_goc < len(root['khoa']) and root['khoa'][vi_tri_goc] < khoa_tach:
        vi_tri_goc += 1
    ___
    dia.write(sector_goc, ma_hoa_node(root, dia.kich_thuoc_sector))
    dia.fsync()


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
la1 = {'loai': 'la', 'khoa': [1, 2, 3], 'gia_tri': ['a', 'b', 'c'], 'la_tiep': 2}
la2 = {'loai': 'la', 'khoa': [5, 7, 9], 'gia_tri': ['e', 'g', 'i'], 'la_tiep': None}
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
dia.write(0, ma_hoa_node(root, 200))
dia.write(1, ma_hoa_node(la1, 200))
dia.write(2, ma_hoa_node(la2, 200))
dia.fsync()

chen_va_tach_neu_can(dia, 0, 4, 'd', 3, 3)
chen_va_tach_neu_can(dia, 0, 0, 'z', 3, 5)
chen_va_tach_neu_can(dia, 0, -1, 'y', 3, 4)
print(tim_kiem(dia, 0, 3))
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


def chen_va_tach_neu_can(dia, sector_goc, khoa_moi, gia_tri_moi, bac_toi_da, sector_trong):
    root = giai_ma_node(dia.read(sector_goc))
    sector = chon_nhanh(root, khoa_moi)
    la = giai_ma_node(dia.read(sector))
    vi_tri = 0
    while vi_tri < len(la['khoa']) and la['khoa'][vi_tri] < khoa_moi:
        vi_tri += 1
    la['khoa'].insert(vi_tri, khoa_moi)
    la['gia_tri'].insert(vi_tri, gia_tri_moi)
    if len(la['khoa']) <= bac_toi_da:
        dia.write(sector, ma_hoa_node(la, dia.kich_thuoc_sector))
        dia.fsync()
        return
    giua = len(la['khoa']) // 2
    la_moi = {'loai': 'la', 'khoa': la['khoa'][giua:], 'gia_tri': la['gia_tri'][giua:], 'la_tiep': la['la_tiep']}
    la['khoa'] = la['khoa'][:giua]
    la['gia_tri'] = la['gia_tri'][:giua]
    khoa_tach = la_moi['khoa'][0]
    la['la_tiep'] = sector_trong
    dia.write(sector, ma_hoa_node(la, dia.kich_thuoc_sector))
    dia.write(sector_trong, ma_hoa_node(la_moi, dia.kich_thuoc_sector))
    dia.fsync()
    vi_tri_goc = 0
    while vi_tri_goc < len(root['khoa']) and root['khoa'][vi_tri_goc] < khoa_tach:
        vi_tri_goc += 1
    root['khoa'].insert(vi_tri_goc, khoa_tach)
    root['con'].insert(vi_tri_goc + 1, sector_trong)
    dia.write(sector_goc, ma_hoa_node(root, dia.kich_thuoc_sector))
    dia.fsync()


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
la1 = {'loai': 'la', 'khoa': [1, 2, 3], 'gia_tri': ['a', 'b', 'c'], 'la_tiep': 2}
la2 = {'loai': 'la', 'khoa': [5, 7, 9], 'gia_tri': ['e', 'g', 'i'], 'la_tiep': None}
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
dia.write(0, ma_hoa_node(root, 200))
dia.write(1, ma_hoa_node(la1, 200))
dia.write(2, ma_hoa_node(la2, 200))
dia.fsync()

chen_va_tach_neu_can(dia, 0, 4, 'd', 3, 3)
chen_va_tach_neu_can(dia, 0, 0, 'z', 3, 5)
chen_va_tach_neu_can(dia, 0, -1, 'y', 3, 4)
print(tim_kiem(dia, 0, 3))
```

```python title=test
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
la1 = {'loai': 'la', 'khoa': [1, 2, 3], 'gia_tri': ['a', 'b', 'c'], 'la_tiep': 2}
la2 = {'loai': 'la', 'khoa': [5, 7, 9], 'gia_tri': ['e', 'g', 'i'], 'la_tiep': None}
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
dia.write(0, ma_hoa_node(root, 200))
dia.write(1, ma_hoa_node(la1, 200))
dia.write(2, ma_hoa_node(la2, 200))
dia.fsync()

chen_va_tach_neu_can(dia, 0, 4, 'd', 3, 3)
chen_va_tach_neu_can(dia, 0, 0, 'z', 3, 5)
chen_va_tach_neu_can(dia, 0, -1, 'y', 3, 4)

root_sau = giai_ma_node(dia.read(0))
assert root_sau['khoa'] == [1, 3, 5], "root co du ba khoa sau hai lan tach"
assert root_sau['con'] == [1, 4, 3, 2], "root tro dung bon la"
assert len(root_sau['con']) == len(root_sau['khoa']) + 1, "bat bien con = khoa + 1"

assert tim_kiem(dia, 0, -1) == 'y', "khoa nho nhat, la moi nhat"
assert tim_kiem(dia, 0, 0) == 'z', "khoa khong gay tach"
assert tim_kiem(dia, 0, 3) == 'c', "khoa tung la diem phan tach van tim duoc"
assert tim_kiem(dia, 0, 9) == 'i', "khoa chua tung bi dung toi"
assert tim_kiem(dia, 0, 100) is None, "khoa khong ton tai"
```

:::hints
- kind: attention
  body: "Chen root['khoa'] tai vi_tri_goc va root['con'] tai vi_tri_goc+1 -- hai dong insert, giong het bai truoc."
- kind: strategy
  body: "root['khoa'].insert(vi_tri_goc, khoa_tach); root['con'].insert(vi_tri_goc + 1, sector_trong)"
- kind: one-line
  body: "root['khoa'].insert(vi_tri_goc, khoa_tach); root['con'].insert(vi_tri_goc + 1, sector_trong)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai insert khoa_tach vao root['khoa'] tai vi_tri_goc va sector_trong vao root['con'] tai vi_tri_goc+1
  requireAst:
  - kind: uses-name, target: vi_tri_goc, min: 4
  - kind: uses-name, target: khoa_tach, min: 2
  - kind: uses-name, target: sector_trong, min: 3
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^c\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một cây THẬT — tìm, chèn, tách, quét khoảng, TẤT cả cùng hoạt động,
qua NHIỀU lần tách liên tiếp. q03 hoàn TẤT.
::::

::::reflect{#nghi-lai}
q03 xây một B+Tree THẬT: tìm kiếm từ gốc XUỐNG lá, chèn qua điều
hướng, tách LAN lên cha (kể cả tạo root MỚI lần đầu), quét theo
khoảng qua liên kết `la_tiep` — TẤT cả không cần MỘT lần quét
tuyến tính TOÀN bộ kho dữ liệu. HAI điều cố Ý bỏ qua: xoá một khoá
(cần logic gộp lại/redistribute khi lá QUÁ thưa, một cây THẬT cần
điều này, NHƯNG nằm ngoài phạm VI ở đây), VÀ tự động chọn sector
trống (bài học NÀY luôn TRUYỀN `sector_trong` bằng tay — một hệ
thống THẬT cần một free list theo dõi sector NÀO đang thật sự rảnh
để tái sử DỤNG, nhất là khi có xoá). q04 "Thác dữ liệu" chuyển
HƯỚNG hoàn toàn: thay vì một cây CẬP nhật tại chỗ, ghi TRƯỚC vào bộ
nhớ, rồi xả xuống các tệp đã sắp XẾP, gộp NGẦM ở nền — LSM hoạt
động ra sao?
::::

::::checkpoint{mastery=0.85}
::::
