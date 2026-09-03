---
id: co-so-du-lieu.nhat-ky-khong-bao-gio-quen.khi-khong-biet-truoc-so-luong
title: Khi không biết trước số lượng
summary: "Hộp giày (q00) cần biết TRƯỚC so_luong_sector. Không biết trước thì GHI TIẾP vào sector kế tiếp (con trỏ tăng dần), thay vì CHỌN vị trí sẵn — quên cập nhật con trỏ là GHI ĐÈ âm thầm lên bản ghi cũ."
locale: vi
track: co-so-du-lieu
module: nhat-ky-khong-bao-gio-quen
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [db.append-log]
requires: [db.simdisk-write]
concepts: [db.append-log]
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
Hộp giày (q00) CẦN biết TRƯỚC `so_luong_sector`. NHƯNG một cuốn NHẬT
ký thì KHÁC — Byte đâu biết TRƯỚC hôm nay sẽ VIẾT bao nhiêu DÒNG?
::::

::::explain{#con-tro-tang-dan}
KHÔNG chọn TRƯỚC vị trí — GHI tiếp VÀO sector **kế tiếp**, giữ MỘT
**con trỏ** (con SỐ) nhớ "sector TRỐNG tiếp theo LÀ đâu", TĂNG con
trỏ đó lên `1` SAU mỗi lần ghi thành CÔNG:

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


def ghi_tiep(dia, sector_tiep_theo, du_lieu):
    dia.write(sector_tiep_theo, du_lieu)
    dia.fsync()
    return sector_tiep_theo + 1


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=3)
p = 0
p = ghi_tiep(dia, p, b'aaa')
p = ghi_tiep(dia, p, b'bbb')

print(p)
print(dia.read(0), dia.read(1))
```

```text title=readonly
2
b'aaa' b'bbb'
```

`ghi_tiep` TRẢ về con trỏ MỚI (`sector_tiep_theo + 1`) THAY vì SỬA
một biến toàn CỤC — người GỌI tự CẬP nhật `p = ghi_tiep(...)`. Sau
hai lần GHI, `p = 2` nghĩa LÀ sector `0` VÀ `1` đã CÓ dữ liệu, sector
TRỐNG tiếp theo LÀ `2`.
::::

::::example{#nhat-ky-dai-them-mai}
Ghi THÊM một dòng NỮA — con trỏ LẠI tăng, nhật KÝ dài THÊM, KHÔNG
cần biết TRƯỚC sẽ dừng Ở đâu:

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


def ghi_tiep(dia, sector_tiep_theo, du_lieu):
    dia.write(sector_tiep_theo, du_lieu)
    dia.fsync()
    return sector_tiep_theo + 1


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=3)
p = 0
p = ghi_tiep(dia, p, b'aaa')
p = ghi_tiep(dia, p, b'bbb')
p = ghi_tiep(dia, p, b'ccc')

print(p)
print(dia.read(2))
```

```text title=readonly
3
b'ccc'
```

MỖI lần Byte GHI thêm, nhật ký DÀI thêm ĐÚNG một sector — KHÁC hẳn
q00 (số lượng CỐ định, biết TRƯỚC), Ở đây "còn bao NHIÊU nữa" LÀ câu
hỏi KHÔNG cần trả lời TRƯỚC.
::::

::::predict{#doan-quen-cap-nhat-con-tro commitOnce}
Byte GHI xong dòng đầu NHƯNG **quên** cập nhật `p` — gọi `ghi_tiep`
LẦN hai VẪN với `p = 0` (thay VÌ `1`):

```python
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=3)
p = 0
p = ghi_tiep(dia, p, b'aaa')
ghi_tiep(dia, 0, b'bbb')
print(dia.read(0))
```

Dòng cuối in ra gì?

:::opt{correct}
`b'bbb'`
:::

:::opt
`b'aaa'` — vì `SimDisk` GIỮ bản ghi ĐẦU tiên ghi VÀO một sector, KHÔNG
cho ghi ĐÈ lần hai
::why
Gần đúng ở việc bạn nghĩ TỚI một cơ chế BẢO vệ hợp lý — "đừng cho
GHI đè dữ liệu ĐÃ có".

Chỗ lệch: `SimDisk.write(sector, data)` KHÔNG hề kiểm TRA "sector
NÀY đã có gì CHƯA" — nó chỉ ĐƠN giản GHI đè bất KỲ thứ gì ĐANG có Ở
sector ĐÓ, KHÔNG hỏi han GÌ. "Đừng ghi ĐÈ" LÀ trách nhiệm CỦA người
GỌI (theo dõi đúng con TRỎ), KHÔNG phải của `SimDisk`.
::
:::

:::opt
Máy báo lỗi — vì gọi `ghi_tiep` HAI lần với CÙNG một `sector`
::why
Gần đúng ở việc bạn LO ngại đúng hướng — gọi LẶP LẠI cùng sector RÕ
ràng LÀ một lỗi LOGIC nghiêm trọng.

Chỗ lệch: VỀ mặt CÚ pháp VÀ kiểu dữ liệu, `ghi_tiep(dia, 0, b'bbb')`
HOÀN toàn hợp LỆ — KHÔNG có gì để Python PHÁT hiện VÀ báo lỗi. Bug
NÀY LÀ lỗi LOGIC (quên cập nhật con trỏ), KHÔNG phải lỗi CÚ pháp/
kiểu — máy KHÔNG có cách nào TỰ nhận RA.
::
:::
::::

::::code{#viet_ghi_tiep}
Viết `ghi_tiep(dia, sector_tiep_theo, du_lieu)` — ghi `du_lieu` VÀO
`sector_tiep_theo`, làm BỀN ngay, trả VỀ con trỏ MỚI.

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
    ___


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=3)
p = 0
p = ghi_tiep(dia, p, b'aaa')
p = ghi_tiep(dia, p, b'bbb')
print(p, dia.read(0), dia.read(1))
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


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=3)
p = 0
p = ghi_tiep(dia, p, b'aaa')
p = ghi_tiep(dia, p, b'bbb')
print(p, dia.read(0), dia.read(1))
```

```python title=test
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=3)
p = 0
p = ghi_tiep(dia, p, b'aaa')
assert p == 1, "con tro tang len 1 sau lan ghi dau"
p = ghi_tiep(dia, p, b'bbb')
assert p == 2, "con tro tang len 2 sau lan ghi hai"
assert dia.read(0) == b'aaa', "sector 0 giu ban ghi dau"
assert dia.read(1) == b'bbb', "sector 1 giu ban ghi hai"

dia2 = SimDisk(so_luong_sector=4, kich_thuoc_sector=2)
q = ghi_tiep(dia2, 0, b'xy')
assert q == 1 and dia2.read(0) == b'xy', "hoat dong dung tu sector 0"
```

:::hints
- kind: attention
  body: "Goi dia.write roi dia.fsync, roi tra ve sector_tiep_theo + 1."
- kind: strategy
  body: "dia.write(sector_tiep_theo, du_lieu); dia.fsync(); return sector_tiep_theo + 1"
- kind: one-line
  body: "dia.write(sector_tiep_theo, du_lieu); dia.fsync(); return sector_tiep_theo + 1"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi dia.write roi dia.fsync roi tra ve sector_tiep_theo + 1
  requireAst:
  - kind: uses-call, target: write, min: 1
  - kind: uses-call, target: fsync, min: 1
  - kind: uses-operator, target: '+', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^2 b'aaa' b'bbb'\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Con trỏ tăng DẦN, nhật ký dài THÊM — KHÔNG cần biết trước sẽ dừng Ở
đâu. NHƯNG mỗi bản GHI Ở đây vẫn rộng ĐÚNG `3` byte CỐ định — một
NHẬT ký thật SỰ thì MỖI dòng dài KHÁC nhau chứ?
::::

::::reflect{#nghi-lai}
Không biết TRƯỚC số lượng? Ghi tiếp VÀO sector kế tiếp, con trỏ TỰ
tăng — quên cập NHẬT con trỏ LÀ ghi ĐÈ âm thầm, KHÔNG lỗi gì báo ra.
Nhưng mỗi "dòng nhật ký" Ở đây vẫn rộng ĐÚNG một số byte CỐ định —
một CÂU nhật ký thật (hay MỘT cặp khoá-giá trị THẬT) đâu phải LÚC
nào cũng cùng ĐỘ dài. Làm sao LƯU một bản ghi có ĐỘ dài THAY đổi?
::::

::::checkpoint{mastery=0.8}
::::
