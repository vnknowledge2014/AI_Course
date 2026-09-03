---
id: co-so-du-lieu.nhieu-dong-thoi-gian.moi-ban-ghi-nhieu-phien-ban
title: Mỗi bản ghi nhiều phiên bản
summary: "ghi(khoa, gia_tri, ts) KHÔNG sửa giá trị cũ tại chỗ — nó THÊM một phiên bản mới vào một danh sách, giữ NGUYÊN mọi phiên bản trước đó. Mỗi khoá trở thành một chuỗi phiên bản (version chain), mỗi mắt xích gắn với đúng thời điểm nó được ghi."
locale: vi
track: co-so-du-lieu
module: nhieu-dong-thoi-gian
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [db.version-chain]
requires: [db.why-multiple-versions]
concepts: [db.version-chain]
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
Mỗi giao dịch cần nhìn thấy MỘT phiên bản riêng (bài TRƯỚC) — vậy
ghi một giá trị mới có XOÁ mất giá trị CŨ không, hay giữ LẠI cả hai?
::::

::::explain{#ghi-them-phien-ban}
`ghi(khoa, gia_tri, ts)` KHÔNG sửa giá trị cũ tại CHỖ — nó THÊM một
phiên bản mới VÀO một danh sách, giữ NGUYÊN mọi phiên bản trước đó:

```python title=readonly
class KhoLuuMVCC:
    def __init__(self):
        self._phien_ban = {}
        self._dong_ho = 0

    def timestamp_moi(self):
        self._dong_ho += 1
        return self._dong_ho

    def ghi(self, khoa, gia_tri, ts):
        self._phien_ban.setdefault(khoa, []).append((ts, gia_tri))


kho = KhoLuuMVCC()
ts1 = kho.timestamp_moi()
kho.ghi("acc1", 100, ts1)
ts2 = kho.timestamp_moi()
kho.ghi("acc1", 150, ts2)
print(kho._phien_ban["acc1"])
```

```text title=readonly
[(1, 100), (2, 150)]
```

Hai lần GHI cùng khoá `"acc1"` — thay VÌ sửa `100` thành `150`,
danh sách phiên BẢN có ĐỦ cả hai: `(1, 100)` (phiên bản CŨ) VÀ
`(2, 150)` (phiên bản MỚI). Giá trị `100` KHÔNG hề biến mất — nó
vẫn tồn tại, gắn với timestamp `1`.
::::

::::example{#nhieu-khoa-doc-lap}
Ghi vào một khoá KHÁC không hề ảnh hưởng danh sách phiên bản của
`"acc1"`:

```python title=readonly
ts3 = kho.timestamp_moi()
kho.ghi("acc2", 999, ts3)
print(kho._phien_ban["acc1"], kho._phien_ban["acc2"])
```

```text title=readonly
[(1, 100), (2, 150)] [(3, 999)]
```

`_phien_ban.setdefault(khoa, [])` tạo MỘT danh sách RIÊNG cho mỗi
khoá — ghi vào `"acc2"` KHÔNG hề đụng tới danh sách của `"acc1"`,
chúng LÀ hai chuỗi phiên bản HOÀN toàn độc lập.
::::

::::predict{#doan-so-phien-ban-sau-hai-lan-ghi commitOnce}
Ghi cùng MỘT khoá hai lần LIÊN tiếp — đếm SỐ phiên bản còn LẠI:

```python
kho2 = KhoLuuMVCC()
tsA = kho2.timestamp_moi()
kho2.ghi("x", "a", tsA)
tsB = kho2.timestamp_moi()
kho2.ghi("x", "b", tsB)
print(len(kho2._phien_ban["x"]))
```

Dòng cuối in ra gì?

:::opt{correct}
`2`
:::

:::opt
`1` — vì lần ghi THỨ hai (`"b"`) ghi ĐÈ lên lần ghi đầu (`"a"`), CHỈ
phiên bản MỚI nhất được giữ lại
::why
Gần đúng ở việc bạn nghĩ TỚI hành vi "ghi ĐÈ" điển hình của MỘT
dict/biến thông thường — một trực GIÁC hợp lý với đa SỐ cách lưu
trữ khác.

Chỗ lệch: `ghi` dùng `.append(...)`, KHÔNG dùng gán trực tiếp
(`= ...`) — mỗi lần gọi THÊM một phần tử MỚI vào danh sách, chưa hề
xoá phần tử NÀO trước đó. Sau hai lần ghi, danh sách CÓ đủ hai
phiên bản: `[(tsA, "a"), (tsB, "b")]`.
::
:::

:::opt
Máy báo lỗi — vì ghi HAI lần liên tiếp lên CÙNG một khoá `"x"` LÀ
thao tác không hợp lệ
::why
Gần đúng ở việc bạn nghĩ TỚI một ràng buộc "một khoá — một giá trị"
hợp lý cho một số HỆ lưu trữ khác.

Chỗ lệch: `ghi` KHÔNG hề `raise` — nó chỉ ĐƠN thuần gọi `.append`,
một thao tác LUÔN thành công trên MỘT `list` Python, bất kể danh
sách đã CÓ bao nhiêu phần tử.
::
:::
::::

::::code{#viet_ghi}
Hoàn thiện `ghi(khoa, gia_tri, ts)` — thêm MỘT phiên bản mới vào
danh sách của đúng khoá đó.

```python title=starter
class KhoLuuMVCC:
    def __init__(self):
        self._phien_ban = {}
        self._dong_ho = 0

    def timestamp_moi(self):
        self._dong_ho += 1
        return self._dong_ho

    def ghi(self, khoa, gia_tri, ts):
        ___


kho = KhoLuuMVCC()
ts1 = kho.timestamp_moi()
kho.ghi("acc1", 100, ts1)
ts2 = kho.timestamp_moi()
kho.ghi("acc1", 150, ts2)
print(kho._phien_ban["acc1"])
```

```python title=solution
class KhoLuuMVCC:
    def __init__(self):
        self._phien_ban = {}
        self._dong_ho = 0

    def timestamp_moi(self):
        self._dong_ho += 1
        return self._dong_ho

    def ghi(self, khoa, gia_tri, ts):
        self._phien_ban.setdefault(khoa, []).append((ts, gia_tri))


kho = KhoLuuMVCC()
ts1 = kho.timestamp_moi()
kho.ghi("acc1", 100, ts1)
ts2 = kho.timestamp_moi()
kho.ghi("acc1", 150, ts2)
print(kho._phien_ban["acc1"])
```

```python title=test
kho2 = KhoLuuMVCC()
tsA = kho2.timestamp_moi()
kho2.ghi("x", "a", tsA)
tsB = kho2.timestamp_moi()
kho2.ghi("x", "b", tsB)
assert kho2._phien_ban["x"] == [(tsA, "a"), (tsB, "b")], "phai giu ca hai phien ban, dung thu tu"

kho3 = KhoLuuMVCC()
ts0 = kho3.timestamp_moi()
kho3.ghi("y", 5, ts0)
assert kho3._phien_ban["y"] == [(ts0, 5)], "ghi lan dau tao dung mot phien ban"

kho4 = KhoLuuMVCC()
tsX = kho4.timestamp_moi()
kho4.ghi("m", 1, tsX)
tsY = kho4.timestamp_moi()
kho4.ghi("n", 2, tsY)
assert kho4._phien_ban["m"] == [(tsX, 1)], "khoa khac nhau phai co danh sach rieng"
assert kho4._phien_ban["n"] == [(tsY, 2)], "khoa khac nhau khong duoc lan sang nhau"
```

:::hints
- kind: attention
  body: "Dung self._phien_ban.setdefault(khoa, []) de lay (hoac tao) danh sach cua khoa, roi .append((ts, gia_tri)) -- mot dong."
- kind: strategy
  body: "self._phien_ban.setdefault(khoa, []).append((ts, gia_tri))"
- kind: one-line
  body: "self._phien_ban.setdefault(khoa, []).append((ts, gia_tri))"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai them (ts, gia_tri) vao danh sach phien ban cua khoa
  requireAst:
  - kind: uses-name, target: khoa, min: 1
  - kind: uses-name, target: gia_tri, min: 1
  - kind: uses-name, target: ts, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[\(1, 100\), \(2, 150\)\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mỗi khoá giờ LÀ một chuỗi phiên bản — giá trị CŨ không hề mất. Đọc
thì phải chọn ĐÚNG phiên bản NÀO trong chuỗi đó?
::::

::::reflect{#nghi-lai}
`ghi` KHÔNG sửa gì tại chỗ — nó CHỈ thêm, giữ nguyên VẸN toàn bộ
lịch sử. Mỗi khoá trở thành một "chuỗi phiên bản" (version chain),
mỗi mắt xích gắn với đúng THỜI điểm nó được ghi. Nhưng chuỗi CÀNG
dài, đọc CÀNG cần biết chọn đúng phiên bản NÀO — một giao dịch bắt
đầu Ở một thời điểm CỤ thể phải thấy đúng phiên bản TỒN tại LÚC đó,
không sớm hơn, không MUỘN hơn. Đọc theo "thời điểm" (snapshot) đó
hoạt động RA sao?
::::

::::checkpoint{mastery=0.8}
::::
