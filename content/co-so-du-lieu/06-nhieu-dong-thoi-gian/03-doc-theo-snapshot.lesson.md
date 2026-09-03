---
id: co-so-du-lieu.nhieu-dong-thoi-gian.doc-theo-snapshot
title: Đọc theo snapshot
summary: "doc_snapshot(khoa, ts_snapshot) tìm phiên bản MỚI NHẤT trong chuỗi có timestamp KHÔNG vượt quá ts_snapshot — bỏ qua mọi phiên bản được ghi SAU thời điểm đó, dù chúng đã tồn tại trong kho. Đọc tại một timestamp SỚM hơn mọi lần ghi trả về None — chưa có phiên bản nào tồn tại lúc đó."
locale: vi
track: co-so-du-lieu
module: nhieu-dong-thoi-gian
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [db.snapshot-read]
requires: [db.version-chain]
concepts: [db.snapshot-read]
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
Một chuỗi phiên bản (bài TRƯỚC) có THỂ dài — đọc tại MỘT thời điểm
cụ thể phải chọn ĐÚNG phiên bản nào?
::::

::::explain{#doc-dung-phien-ban}
`doc_snapshot(khoa, ts_snapshot)` tìm phiên bản MỚI nhất trong chuỗi
có timestamp KHÔNG vượt quá `ts_snapshot` — bỏ QUA mọi phiên bản
ghi SAU thời điểm đó:

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

    def doc_snapshot(self, khoa, ts_snapshot):
        ung_vien = None
        for (ts, gia_tri) in self._phien_ban.get(khoa, []):
            if ts <= ts_snapshot and (ung_vien is None or ts > ung_vien[0]):
                ung_vien = (ts, gia_tri)
        return ung_vien[1] if ung_vien else None


kho = KhoLuuMVCC()
ts1 = kho.timestamp_moi()
kho.ghi("acc1", 100, ts1)
ts2 = kho.timestamp_moi()
kho.ghi("acc1", 150, ts2)
print(kho.doc_snapshot("acc1", ts1))
print(kho.doc_snapshot("acc1", ts2))
```

```text title=readonly
100
150
```

Đọc TẠI `ts1` (thời điểm NGAY sau lần ghi ĐẦU): chỉ phiên bản
`(ts1, 100)` thoả `ts <= ts1` — trả về `100`. Đọc TẠI `ts2` (sau CẢ
hai lần ghi): CẢ hai phiên bản thoả điều kiện, nhưng `ung_vien`
LUÔN giữ phiên bản có `ts` LỚN nhất trong SỐ đó — `150`.
::::

::::example{#chua-tung-ghi}
Đọc một khoá CHƯA từng ghi — không CÓ phiên bản nào thoả điều
kiện, trả VỀ `None`:

```python title=readonly
print(kho.doc_snapshot("acc2", ts2))
```

```text title=readonly
None
```

`"acc2"` chưa hề xuất hiện trong `_phien_ban` — `.get(khoa, [])`
trả về danh sách RỖNG, vòng `for` KHÔNG chạy vòng nào, `ung_vien`
giữ nguyên `None`, hàm trả VỀ `None`.
::::

::::predict{#doan-doc-truoc-khi-ghi commitOnce}
Một khoá `"acc1"` được GHI lần đầu tại `ts1`. Byte đọc TẠI thời
điểm `0` — TRƯỚC cả lần ghi đầu tiên:

```python
kho3 = KhoLuuMVCC()
ts_ghi = kho3.timestamp_moi()
kho3.ghi("acc1", 100, ts_ghi)
print(kho3.doc_snapshot("acc1", 0))
```

Dòng cuối in ra gì?

:::opt{correct}
`None`
:::

:::opt
`100` — vì `"acc1"` RỐT cuộc CÓ một phiên bản (`100`), VÀ đó LÀ giá
trị GẦN nhất từng tồn tại
::why
Gần đúng ở việc bạn nhớ ĐÚNG `"acc1"` THẬT sự có một phiên bản đã
ghi — MỘT quan sát chính xác về TRẠNG thái kho.

Chỗ lệch: `doc_snapshot("acc1", 0)` CHỈ chấp nhận phiên bản có
`ts <= 0` — nhưng `ts_ghi` (kết quả của `timestamp_moi()` LẦN đầu
tiên gọi) LUÔN LÀ `1`, không phải `0` HAY nhỏ hơn. Điều kiện `ts <=
ts_snapshot` (`1 <= 0`) SAI cho MỌI phiên bản, `ung_vien` giữ
nguyên `None`.
::
:::

:::opt
Máy báo lỗi — vì đọc TẠI timestamp `0`, MỘT giá trị KHÔNG hề tồn
tại trong kho, LÀ thao tác không hợp LỆ
::why
Gần đúng ở việc bạn nghĩ TỚI `0` như một giá trị "LẠ" cho một
timestamp — MỘT trực giác dễ hiểu.

Chỗ lệch: `ts_snapshot` chỉ LÀ một số nguyên bình THƯỜNG được so
sánh — `doc_snapshot` không hề kiểm TRA nó có "hợp lệ" hay không,
`0` hoàn TOÀN dùng được, chỉ đơn giản KHÔNG có phiên bản nào thoả
điều kiện.
::
:::
::::

::::code{#viet_doc_snapshot}
Hoàn thiện `doc_snapshot` — trong vòng lặp, giữ lại phiên bản MỚI
nhất thoả điều kiện `ts <= ts_snapshot`.

```python title=starter
class KhoLuuMVCC:
    def __init__(self):
        self._phien_ban = {}
        self._dong_ho = 0

    def timestamp_moi(self):
        self._dong_ho += 1
        return self._dong_ho

    def ghi(self, khoa, gia_tri, ts):
        self._phien_ban.setdefault(khoa, []).append((ts, gia_tri))

    def doc_snapshot(self, khoa, ts_snapshot):
        ung_vien = None
        for (ts, gia_tri) in self._phien_ban.get(khoa, []):
            ___
        return ung_vien[1] if ung_vien else None


kho = KhoLuuMVCC()
ts1 = kho.timestamp_moi()
kho.ghi("acc1", 100, ts1)
ts2 = kho.timestamp_moi()
kho.ghi("acc1", 150, ts2)
print(kho.doc_snapshot("acc1", ts1))
print(kho.doc_snapshot("acc1", ts2))
print(kho.doc_snapshot("acc2", ts2))
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

    def doc_snapshot(self, khoa, ts_snapshot):
        ung_vien = None
        for (ts, gia_tri) in self._phien_ban.get(khoa, []):
            if ts <= ts_snapshot and (ung_vien is None or ts > ung_vien[0]):
                ung_vien = (ts, gia_tri)
        return ung_vien[1] if ung_vien else None


kho = KhoLuuMVCC()
ts1 = kho.timestamp_moi()
kho.ghi("acc1", 100, ts1)
ts2 = kho.timestamp_moi()
kho.ghi("acc1", 150, ts2)
print(kho.doc_snapshot("acc1", ts1))
print(kho.doc_snapshot("acc1", ts2))
print(kho.doc_snapshot("acc2", ts2))
```

```python title=test
kho2 = KhoLuuMVCC()
ts1 = kho2.timestamp_moi()
kho2.ghi("acc1", 100, ts1)
ts2 = kho2.timestamp_moi()
kho2.ghi("acc1", 150, ts2)
assert kho2.doc_snapshot("acc1", ts1) == 100, "doc tai ts1 phai thay ban ghi 100"
assert kho2.doc_snapshot("acc1", ts2) == 150, "doc tai ts2 phai thay ban ghi moi nhat 150"
assert kho2.doc_snapshot("acc2", ts2) is None, "khoa chua tung ghi phai ra None"
assert kho2.doc_snapshot("acc1", 0) is None, "doc truoc moi lan ghi phai ra None"

kho3 = KhoLuuMVCC()
tsA = kho3.timestamp_moi()
kho3.ghi("x", "a", tsA)
tsB = kho3.timestamp_moi()
kho3.ghi("x", "b", tsB)
tsC = kho3.timestamp_moi()
kho3.ghi("x", "c", tsC)
assert kho3.doc_snapshot("x", tsB) == "b", "doc giua chung phai thay phien ban giua, khong phai phien ban cuoi"
```

:::hints
- kind: attention
  body: "Neu ts <= ts_snapshot VA (ung_vien la None hoac ts > ung_vien[0]), gan ung_vien = (ts, gia_tri) -- mot dong."
- kind: strategy
  body: "if ts <= ts_snapshot and (ung_vien is None or ts > ung_vien[0]): ung_vien = (ts, gia_tri)"
- kind: one-line
  body: "if ts <= ts_snapshot and (ung_vien is None or ts > ung_vien[0]): ung_vien = (ts, gia_tri)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai giu lai phien ban moi nhat thoa ts <= ts_snapshot trong vong lap
  requireAst:
  - kind: uses-name, target: ts_snapshot, min: 1
  - kind: uses-name, target: ung_vien, min: 4
  - kind: uses-name, target: ts, min: 4
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^100\n150\nNone\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đọc đúng phiên bản THEO thời điểm — không hề thấy những gì ghi
SAU. Điều đó có nghĩa LÀ gì cho tính đúng ĐẮN của một giao dịch?
::::

::::reflect{#nghi-lai}
`doc_snapshot` chọn phiên bản MỚI nhất KHÔNG vượt quá một mốc thời
gian — mọi phiên bản ghi SAU mốc đó hoàn toàn KHÔNG hiện diện, dù
đã "committed" từ lâu. Đây LÀ nền tảng của một mức cô lập (isolation
level) — một giao dịch dùng ĐÚNG một `ts_snapshot` cố định SẼ luôn
thấy một bức TRANH nhất quán, không đổi TRONG suốt vòng đời của nó.
Nhưng nếu MỖI lần đọc lại lấy một `ts_snapshot` MỚI thay vì cố
định, chuyện GÌ xảy ra?
::::

::::checkpoint{mastery=0.8}
::::
