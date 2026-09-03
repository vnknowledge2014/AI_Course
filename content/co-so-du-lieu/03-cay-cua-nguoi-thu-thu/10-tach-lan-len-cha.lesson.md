---
id: co-so-du-lieu.cay-cua-nguoi-thu-thu.tach-lan-len-cha
title: Tách lan lên cha
summary: "cap_nhat_cha_sau_tach nhận một khoá phân tách VÀ sector con MỚI (giả sử một lần tách đã xảy ra ở đâu đó) — tìm vị trí SẮP xếp trong khoá của cha, chèn khoá TẠI vi_tri, chèn sector con MỚI tại vi_tri+1 (lệch một, vì khoá mới luôn NẰM giữa hai con)."
locale: vi
track: co-so-du-lieu
module: cay-cua-nguoi-thu-thu
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [db.split-propagates-once]
requires: [db.insert-via-navigation]
concepts: [db.split-propagates-once]
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
Bài `day-khoa-len-cha` đẩy khoá LÊN một cha CHƯA từng tồn tại. LẦN
này, cha ĐÃ có sẵn — cập nhật MỘT node cha đã TỒN tại khác gì?
::::

::::explain{#cap-nhat-cha}
`cap_nhat_cha_sau_tach` nhận MỘT khoá phân tách VÀ sector con mới
(giả SỬ một lần tách đã xảy RA ở đâu đó) — tìm vị TRÍ sắp xếp trong
`khoa` của CHA, rồi chèn khoá VÀ con LỆCH nhau đúng MỘT vị trí:

```python title=readonly
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


def cap_nhat_cha_sau_tach(dia, sector_cha, khoa_tach, sector_con_moi):
    cha = giai_ma_node(dia.read(sector_cha))
    vi_tri = 0
    while vi_tri < len(cha['khoa']) and cha['khoa'][vi_tri] < khoa_tach:
        vi_tri += 1
    cha['khoa'].insert(vi_tri, khoa_tach)
    cha['con'].insert(vi_tri + 1, sector_con_moi)
    dia.write(sector_cha, ma_hoa_node(cha, dia.kich_thuoc_sector))
    dia.fsync()


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
dia.write(0, ma_hoa_node(root, 200))
dia.fsync()

cap_nhat_cha_sau_tach(dia, 0, 3, 3)
print(giai_ma_node(dia.read(0)))
```

```text title=readonly
{'loai': 'trong', 'khoa': [3, 5], 'con': [1, 3, 2]}
```

Cha ban đầu `{khoa:[5], con:[1,2]}` — sector `1` LÀ con TRÁI của
`5`, sector `2` LÀ con PHẢI. Khoá tách `3` (nhỏ hơn `5`) tìm
`vi_tri=0`, `khoa.insert(0, 3)` → `[3, 5]`. Sector `3` (con MỚI,
chứa khoá bắt đầu TỪ `3`) chèn TẠI `con[vi_tri+1]=con[1]` → `[1, 3,
2]` — sector `3` nằm ĐÚNG giữa `1` VÀ `2`, vì nó LÀ con trái của
khoá `5` NHƯNG con phải của khoá `3` mới CHÈN.
::::

::::example{#node-cha-hai-khoa}
Cha ĐÃ có sẵn hai khoá (`[3, 5]`, ba con `[1, 3, 2]`) — chèn khoá
tách `7` (LỚN hơn cả hai khoá HIỆN có):

```python title=readonly
root2 = {'loai': 'trong', 'khoa': [3, 5], 'con': [1, 3, 2]}
dia.write(0, ma_hoa_node(root2, 200))
dia.fsync()

cap_nhat_cha_sau_tach(dia, 0, 7, 4)
print(giai_ma_node(dia.read(0)))
```

```text title=readonly
{'loai': 'trong', 'khoa': [3, 5, 7], 'con': [1, 3, 2, 4]}
```

`7` lớn hơn CẢ `3` lẫn `5` — vòng `while` chạy HẾT, `vi_tri=2`,
`khoa.insert(2, 7)` → cuối. Sector `4` (con MỚI) chèn TẠI
`con[3]` → cuối CÙNG luôn, giữ đúng bất BIẾN `len(con) ==
len(khoa)+1`.
::::

::::predict{#doan-cha-sau-tach-lon commitOnce}
Cha ban đầu chỉ có MỘT khoá (`{khoa:[5], con:[1,2]}`). Byte cập
nhật với khoá tách `7` (LỚN hơn khoá hiện có), con mới LÀ sector
`3`:

```python
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
dia.write(0, ma_hoa_node(root, 200))
dia.fsync()
cap_nhat_cha_sau_tach(dia, 0, 7, 3)
print(giai_ma_node(dia.read(0)))
```

Dòng cuối in ra gì?

:::opt{correct}
`{'loai': 'trong', 'khoa': [5, 7], 'con': [1, 2, 3]}`
:::

:::opt
`{'loai': 'trong', 'khoa': [7, 5], 'con': [3, 1, 2]}` — vì khoá MỚI
luôn được chèn VÀO đầu danh sách
::why
Gần đúng ở việc bạn nghĩ TỚI một quy tắc "MỚI thì lên đầu" — MỘT
liên tưởng hợp lý với nhiều cấu trúc dữ liệu KHÁC (như stack).

Chỗ lệch: vòng `while` tìm ĐÚNG vị trí sắp xếp — `7` KHÔNG nhỏ hơn
`5`, nên `vi_tri` chạy TỚI `1` (hết danh sách), `khoa.insert(1, 7)`
đặt `7` VÀO sau `5`, giữ `khoa` LUÔN tăng dần: `[5, 7]`.
::
:::

:::opt
Máy báo lỗi — vì `con` sẽ có `3` phần tử trong khi `khoa` chỉ có
`2`, phá vỡ bất BIẾN
::why
Gần đúng ở việc bạn nhớ ĐÚNG bất biến `len(con) == len(khoa)+1` —
MỘT quan sát chính xác về cấu TRÚC node trong.

Chỗ lệch: `3` con VÀ `2` khoá CHÍNH LÀ đúng bất biến đó
(`3 == 2+1`), KHÔNG hề vi phạm gì — code KHÔNG có bất kỳ `raise`
nào kiểm tra ĐIỀU này, và cũng KHÔNG cần, vì kết quả LUÔN đúng.
::
:::
::::

::::code{#viet_cap_nhat_cha_sau_tach}
Hoàn thiện `cap_nhat_cha_sau_tach(dia, sector_cha, khoa_tach,
sector_con_moi)` — chèn khoá VÀ con lệch nhau đúng một vị trí.

```python title=starter
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


def cap_nhat_cha_sau_tach(dia, sector_cha, khoa_tach, sector_con_moi):
    cha = giai_ma_node(dia.read(sector_cha))
    vi_tri = 0
    while vi_tri < len(cha['khoa']) and cha['khoa'][vi_tri] < khoa_tach:
        vi_tri += 1
    ___
    dia.write(sector_cha, ma_hoa_node(cha, dia.kich_thuoc_sector))
    dia.fsync()


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
dia.write(0, ma_hoa_node(root, 200))
dia.fsync()

cap_nhat_cha_sau_tach(dia, 0, 3, 3)
print(giai_ma_node(dia.read(0)))
```

```python title=solution
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


def cap_nhat_cha_sau_tach(dia, sector_cha, khoa_tach, sector_con_moi):
    cha = giai_ma_node(dia.read(sector_cha))
    vi_tri = 0
    while vi_tri < len(cha['khoa']) and cha['khoa'][vi_tri] < khoa_tach:
        vi_tri += 1
    cha['khoa'].insert(vi_tri, khoa_tach)
    cha['con'].insert(vi_tri + 1, sector_con_moi)
    dia.write(sector_cha, ma_hoa_node(cha, dia.kich_thuoc_sector))
    dia.fsync()


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
dia.write(0, ma_hoa_node(root, 200))
dia.fsync()

cap_nhat_cha_sau_tach(dia, 0, 3, 3)
print(giai_ma_node(dia.read(0)))
```

```python title=test
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=200)
root = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
dia.write(0, ma_hoa_node(root, 200))
dia.fsync()

cap_nhat_cha_sau_tach(dia, 0, 3, 3)
cha1 = giai_ma_node(dia.read(0))
assert cha1['khoa'] == [3, 5], "khoa tach nho hon -- chen dau"
assert cha1['con'] == [1, 3, 2], "con moi lech mot vi tri so voi khoa"

root2 = {'loai': 'trong', 'khoa': [5], 'con': [1, 2]}
dia.write(0, ma_hoa_node(root2, 200))
dia.fsync()
cap_nhat_cha_sau_tach(dia, 0, 7, 3)
cha2 = giai_ma_node(dia.read(0))
assert cha2['khoa'] == [5, 7], "khoa tach lon hon -- chen cuoi"
assert cha2['con'] == [1, 2, 3], "con moi vao cuoi"
assert len(cha2['con']) == len(cha2['khoa']) + 1, "bat bien con = khoa + 1"
```

:::hints
- kind: attention
  body: "Chen cha['khoa'] tai vi_tri va cha['con'] tai vi_tri+1 -- hai dong insert."
- kind: strategy
  body: "cha['khoa'].insert(vi_tri, khoa_tach); cha['con'].insert(vi_tri + 1, sector_con_moi)"
- kind: one-line
  body: "cha['khoa'].insert(vi_tri, khoa_tach); cha['con'].insert(vi_tri + 1, sector_con_moi)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai insert khoa_tach vao cha['khoa'] tai vi_tri va sector_con_moi vao cha['con'] tai vi_tri+1
  requireAst:
  - kind: uses-name, target: vi_tri, min: 4
  - kind: uses-name, target: khoa_tach, min: 2
  - kind: uses-name, target: sector_con_moi, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\{'loai': 'trong', 'khoa': \[3, 5\], 'con': \[1, 3, 2\]\}\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cha cập nhật ĐÚNG, dù đã có sẵn khoá HAY chưa. Chèn qua điều hướng,
tách LAN lên cha — cây giờ ĐÃ đủ mọi mảnh GHÉP để lớn LÊN. NHƯNG
tìm MỘT khoảng khoá thay VÌ một khoá LẺ diễn ra thế NÀO?
::::

::::reflect{#nghi-lai}
`cap_nhat_cha_sau_tach` KHÔNG quan tâm cha đã CÓ bao nhiêu khoá —
vòng `while` LUÔN tìm đúng vị trí sắp XẾP, chèn khoá VÀ con LỆCH
đúng một vị TRÍ. Ghép VỚI `tach_la` (bài `la-day-phai-tach`), MỘT
lá đầy giờ ĐÃ có thể tách VÀ báo cho cha CẬP nhật đúng. Nhưng tìm
MỘT khoá lẻ chỉ LÀ một nửa CÂU chuyện — làm sao lấy RA cả một
KHOẢNG khoá liên tục, VÍ dụ "mọi khoá TỪ 3 đến 9"?
::::

::::checkpoint{mastery=0.8}
::::
