---
id: co-so-du-lieu.nhat-ky-khong-bao-gio-quen.quet-de-tim
title: Quét để tìm
summary: "Tìm giá trị của một khoá bằng cách quét TỪ ĐẦU nhật ký, giải mã từng bản ghi, so sánh khoá — O(n): khoá KHÔNG tồn tại buộc phải quét HẾT toàn bộ (con_tro bản ghi) mới biết chắc, không có cách dừng sớm."
locale: vi
track: co-so-du-lieu
module: nhat-ky-khong-bao-gio-quen
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [db.linear-scan]
requires: [db.key-value-record, db.append-log]
concepts: [db.linear-scan]
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
Nhật ký giờ CÓ nhiều cặp khoá-giá trị nối TIẾP nhau. Muốn TÌM giá
trị của khoá `'ten'` — Byte phải LÀM sao?
::::

::::explain{#quet-tu-dau}
Cách ĐƠN giản nhất: quét TỪ sector `0` tới sector CUỐI ĐÃ ghi, giải
MÃ từng bản ghi, SO sánh khoá — khớp THÌ trả về giá trị NGAY:

```python title=readonly
def tim_bang_quet(dia, con_tro, khoa_can_tim):
    for i in range(con_tro):
        khoa, gia_tri = doc_kv(dia.read(i))
        if khoa == khoa_can_tim:
            return gia_tri
    return None


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
p = 0
p = ghi_tiep(dia, p, ghi_kv(b'id', b'7', 24))
p = ghi_tiep(dia, p, ghi_kv(b'ten', b'Byte', 24))
p = ghi_tiep(dia, p, ghi_kv(b'tuoi', b'25', 24))

print(tim_bang_quet(dia, p, b'ten'))
```

```text title=readonly
b'Byte'
```

`tim_bang_quet` dừng NGAY khi khớp — TÌM `'ten'` (bản ghi THỨ hai)
chỉ CẦN đọc VÀ giải mã hai bản GHI (sector `0` VÀ `1`), KHÔNG cần
đọc sector `2`.
::::

::::example{#khoa-khong-ton-tai}
Tìm một khoá KHÔNG hề tồn tại — vòng LẶP chạy HẾT `con_tro` bản ghi,
KHÔNG khớp lần nào, RỒI mới trả VỀ `None`:

```python title=readonly
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
p = 0
p = ghi_tiep(dia, p, ghi_kv(b'id', b'7', 24))
p = ghi_tiep(dia, p, ghi_kv(b'ten', b'Byte', 24))
p = ghi_tiep(dia, p, ghi_kv(b'tuoi', b'25', 24))

print(tim_bang_quet(dia, p, b'mau_sac'))
```

```text title=readonly
None
```

`'mau_sac'` KHÔNG hề được GHI — vòng lặp đọc HẾT cả `3` bản ghi (id,
ten, tuổi), KHÔNG khớp cái NÀO, RỒI thoát vòng LẶP, chạy TỚI
`return None`. TÌM một khoá KHÔNG tồn tại LUÔN đắt NHẤT — phải quét
HẾT mới biết CHẮC "không có".
::::

::::predict{#doan-tim-khoa-vua-ghi-cuoi commitOnce}
Byte ghi BỐN cặp, RỒI tìm đúng khoá CUỐI cùng vừa ghi
(`'mau_sac'`):

```python
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
p = 0
p = ghi_tiep(dia, p, ghi_kv(b'id', b'7', 24))
p = ghi_tiep(dia, p, ghi_kv(b'ten', b'Byte', 24))
p = ghi_tiep(dia, p, ghi_kv(b'tuoi', b'25', 24))
p = ghi_tiep(dia, p, ghi_kv(b'mau_sac', b'xanh', 24))
print(tim_bang_quet(dia, p, b'mau_sac'))
```

Cần đọc VÀ giải mã BAO NHIÊU bản ghi để TÌM ra kết quả?

:::opt{correct}
`4` (đọc HẾT, vì khoá cần TÌM nằm Ở bản ghi CUỐI)
:::

:::opt
`1` — vì `tim_bang_quet` biết TRƯỚC khoá CUỐI cùng ghi LUÔN nằm Ở
bản ghi mới NHẤT, nên đọc thẳng sector CUỐI
::why
Gần đúng ở việc bạn nghĩ TỚI một "MẸO" hợp lý — kiểm TRA bản ghi
MỚI nhất TRƯỚC — MỘT ý tưởng đúng hướng CHO các hệ thống thật (bài
SAU sẽ xây đúng Ý tưởng này BẰNG chỉ mục).

Chỗ lệch: `tim_bang_quet` NHƯ đã viết quét THEO thứ TỰ TĂNG dần
(`for i in range(con_tro)`) — TỪ sector `0` TRỞ đi, KHÔNG hề "biết
trước" khoá NẰM ở đâu. Muốn tìm khoá `'mau_sac'` (ghi CUỐI, sector
`3`), nó PHẢI đọc qua CẢ ba bản ghi TRƯỚC (id, ten, tuổi) — không
khớp cái NÀO — RỒI mới TỚI bản ghi thứ TƯ mới khớp.
::
:::

:::opt
`0` — vì Python TỐI ưu sẵn, TỰ tìm thẳng ĐÚNG bản ghi mà KHÔNG cần
đọc qua NHỮNG bản ghi khác
::why
Gần đúng ở việc bạn TIN Python "THÔNG minh" tối ưu HOÁ code TỰ
động — một niềm TIN hợp lý VỚI nhiều thao TÁC built-in khác.

Chỗ lệch: `tim_bang_quet` LÀ vòng lặp DO người viết TỰ định nghĩa,
KHÔNG phải một thao TÁC built-in được tối ưu NGẦM — Python chạy
ĐÚNG những gì vòng lặp VIẾT RA, KHÔNG "đoán" ĐƯỢC bản ghi nào cần
đọc mà bỏ qua PHẦN còn lại.
::
:::
::::

::::code{#viet_tim_bang_quet}
Hoàn thiện `tim_bang_quet(dia, con_tro, khoa_can_tim)` — so SÁNH
khoá, trả về giá TRỊ nếu khớp.

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


def tim_bang_quet(dia, con_tro, khoa_can_tim):
    for i in range(con_tro):
        khoa, gia_tri = doc_kv(dia.read(i))
        ___
    return None


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
p = 0
p = ghi_tiep(dia, p, ghi_kv(b'id', b'7', 24))
p = ghi_tiep(dia, p, ghi_kv(b'ten', b'Byte', 24))
print(tim_bang_quet(dia, p, b'ten'))
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


def tim_bang_quet(dia, con_tro, khoa_can_tim):
    for i in range(con_tro):
        khoa, gia_tri = doc_kv(dia.read(i))
        if khoa == khoa_can_tim:
            return gia_tri
    return None


dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
p = 0
p = ghi_tiep(dia, p, ghi_kv(b'id', b'7', 24))
p = ghi_tiep(dia, p, ghi_kv(b'ten', b'Byte', 24))
print(tim_bang_quet(dia, p, b'ten'))
```

```python title=test
dia = SimDisk(so_luong_sector=8, kich_thuoc_sector=24)
p = 0
p = ghi_tiep(dia, p, ghi_kv(b'id', b'7', 24))
p = ghi_tiep(dia, p, ghi_kv(b'ten', b'Byte', 24))
p = ghi_tiep(dia, p, ghi_kv(b'tuoi', b'25', 24))
assert tim_bang_quet(dia, p, b'ten') == b'Byte', "tim thay khoa giua"
assert tim_bang_quet(dia, p, b'id') == b'7', "tim thay khoa dau"
assert tim_bang_quet(dia, p, b'tuoi') == b'25', "tim thay khoa cuoi"
assert tim_bang_quet(dia, p, b'khong_co') is None, "khoa khong ton tai -- None"
assert tim_bang_quet(dia, 0, b'id') is None, "con_tro = 0 -- chua ghi gi -- None"
```

:::hints
- kind: attention
  body: "So sanh khoa voi khoa_can_tim; neu bang thi return gia_tri."
- kind: strategy
  body: "if khoa == khoa_can_tim: return gia_tri"
- kind: one-line
  body: "if khoa == khoa_can_tim: return gia_tri"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai so sanh khoa == khoa_can_tim roi return gia_tri neu khop
  requireAst:
  - kind: uses-name, target: khoa, min: 2
  - kind: uses-name, target: gia_tri, min: 2
  - kind: uses-operator, target: '==', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^b'Byte'\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tìm được — NHƯNG quét TỪ đầu MỖI lần LÀ chậm, nhật ký CÀNG dài CÀNG
chậm. CÓ cách nào NHỚ luôn khoá nằm Ở đâu, KHÔNG cần quét lại TỪ
đầu?
::::

::::reflect{#nghi-lai}
Quét TỪ đầu tìm ĐƯỢC, nhưng khoá KHÔNG tồn tại buộc PHẢI đọc HẾT
toàn bộ — CÀNG nhiều bản ghi, càng CHẬM, VÀ tệ nhất LÀ trường hợp
"không CÓ". Nếu Byte NHỚ luôn "khoá NÀY nằm Ở sector NÀO" — thay VÌ
quét lại TỪ đầu MỖI lần — thì SAO?
::::

::::checkpoint{mastery=0.8}
::::
