---
id: co-so-du-lieu.nhieu-dong-thoi-gian.read-committed-vs-snapshot
title: Read committed và snapshot isolation
summary: "Read committed lấy MỘT timestamp MỚI cho MỖI lần đọc — hai lần đọc trong CÙNG giao dịch có thể thấy giá trị KHÁC nhau nếu có ai ghi xen giữa. Snapshot isolation cố định MỘT timestamp DUY nhất từ lúc bắt đầu — mọi lần đọc trong CÙNG giao dịch luôn nhất quán, dù có ai ghi xen giữa hay không."
locale: vi
track: co-so-du-lieu
module: nhieu-dong-thoi-gian
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [db.two-isolation-levels]
requires: [db.snapshot-isolation-idea]
concepts: [db.two-isolation-levels]
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
Snapshot isolation (bài TRƯỚC) cố định MỘT timestamp từ đầu. Nếu
MỖI lần đọc lại lấy timestamp MỚI thay VÌ cố định, hai lần đọc TRONG
cùng giao dịch có LUÔN ra cùng kết quả không?
::::

::::explain{#hai-cach-lay-timestamp}
`doc_read_committed` lấy MỘT timestamp MỚI cho MỖI lần đọc — hai
lần đọc TRONG cùng giao dịch CÓ thể thấy giá trị KHÁC nhau nếu CÓ
ai ghi xen giữa:

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


def doc_read_committed(kho, khoa):
    ts_hien_tai = kho.timestamp_moi()
    return kho.doc_snapshot(khoa, ts_hien_tai)


kho = KhoLuuMVCC()
ts0 = kho.timestamp_moi()
kho.ghi("acc1", 100, ts0)

doc_lan1 = doc_read_committed(kho, "acc1")
tsX = kho.timestamp_moi()
kho.ghi("acc1", 200, tsX)
doc_lan2 = doc_read_committed(kho, "acc1")

print(doc_lan1, doc_lan2)
```

```text title=readonly
100 200
```

Lần đọc ĐẦU (`doc_lan1`) thấy `100`. GIỮA hai lần đọc, một giao
dịch KHÁC ghi `200`. Lần đọc SAU (`doc_lan2`) — VÌ `doc_read_
committed` lấy timestamp MỚI mỗi lần — thấy `200`. HAI lần đọc
TRONG cùng "giao dịch" (cùng khối code) ra KẾT quả khác nhau.
::::

::::example{#snapshot-isolation-nhat-quan}
Cùng kịch bản, nhưng dùng MỘT timestamp cố định thay VÌ lấy mới mỗi
lần — kết quả HOÀN toàn khác:

```python title=readonly
kho2 = KhoLuuMVCC()
ts0_2 = kho2.timestamp_moi()
kho2.ghi("acc1", 100, ts0_2)

ts_snapshot_co_dinh = kho2.timestamp_moi()
doc_lan1_snap = kho2.doc_snapshot("acc1", ts_snapshot_co_dinh)
tsY = kho2.timestamp_moi()
kho2.ghi("acc1", 200, tsY)
doc_lan2_snap = kho2.doc_snapshot("acc1", ts_snapshot_co_dinh)

print(doc_lan1_snap, doc_lan2_snap)
```

```text title=readonly
100 100
```

`ts_snapshot_co_dinh` được TÍNH đúng MỘT lần, dùng LẠI cho CẢ hai
lần đọc — dù `200` đã ĐƯỢC ghi Ở giữa, `doc_lan2_snap` VẪN thấy
`100`, giống HỆT `doc_lan1_snap`. Đây LÀ "nhất quán đọc lặp lại"
(repeatable read) — thứ read-committed KHÔNG đảm bảo được.
::::

::::predict{#doan-doc-lan-ba commitOnce}
Tiếp tục kịch bản `read_committed` Ở TRÊN (`kho`, `doc_lan1=100`,
`doc_lan2=200`) — MỘT giao dịch thứ ba ghi thêm `300` sau đó, rồi
đọc LẠI bằng `doc_read_committed`:

```python
tsZ = kho.timestamp_moi()
kho.ghi("acc1", 300, tsZ)
print(doc_read_committed(kho, "acc1"))
```

Dòng cuối in ra gì?

:::opt{correct}
`300`
:::

:::opt
`200` — vì `doc_lan2` (LẦN đọc gần nhất TRƯỚC đó) đã "khoá" giá trị
NÀY lại, các lần đọc SAU tiếp tục dùng chung
::why
Gần đúng ở việc bạn nghĩ TỚI một cơ CHẾ "ghi nhớ" giữa các lần đọc
— MỘT trực giác hợp lý CHO snapshot isolation (bài `example` VỪA
thấy Ở trên).

Chỗ lệch: `doc_read_committed` KHÔNG hề lưu lại timestamp CŨ — MỖI
lần gọi đều TÍNH `ts_hien_tai = kho.timestamp_moi()` HOÀN toàn mới,
độc lập với các lần gọi TRƯỚC. Lần gọi NÀY thấy phiên bản MỚI nhất
tính TỚI lúc nó chạy — LÀ `300`.
::
:::

:::opt
Máy báo lỗi — vì đọc BA lần liên tiếp trên CÙNG một khoá với BA
timestamp khác nhau LÀ thao tác không hợp lệ
::why
Gần đúng ở việc bạn nghĩ TỚI việc "nhiều timestamp khác nhau" như
một dấu HIỆU bất thường — một sự thận trọng hợp lý.

Chỗ lệch: MỖI lời gọi `doc_read_committed` ĐỘC lập hoàn toàn, không
có trạng THÁI nào được chia sẻ giữa các lần gọi — không hề `raise`,
mỗi lần chỉ đơn giản đọc phiên bản mới NHẤT tại thời điểm ĐÓ.
::
:::
::::

::::code{#viet_doc_read_committed}
Hoàn thiện `doc_read_committed` — lấy một timestamp MỚI mỗi lần
được gọi, rồi đọc snapshot TẠI đúng thời điểm đó.

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
            if ts <= ts_snapshot and (ung_vien is None or ts > ung_vien[0]):
                ung_vien = (ts, gia_tri)
        return ung_vien[1] if ung_vien else None


def doc_read_committed(kho, khoa):
    ___


kho = KhoLuuMVCC()
ts0 = kho.timestamp_moi()
kho.ghi("acc1", 100, ts0)
doc_lan1 = doc_read_committed(kho, "acc1")
tsX = kho.timestamp_moi()
kho.ghi("acc1", 200, tsX)
doc_lan2 = doc_read_committed(kho, "acc1")
print(doc_lan1, doc_lan2)
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


def doc_read_committed(kho, khoa):
    ts_hien_tai = kho.timestamp_moi()
    return kho.doc_snapshot(khoa, ts_hien_tai)


kho = KhoLuuMVCC()
ts0 = kho.timestamp_moi()
kho.ghi("acc1", 100, ts0)
doc_lan1 = doc_read_committed(kho, "acc1")
tsX = kho.timestamp_moi()
kho.ghi("acc1", 200, tsX)
doc_lan2 = doc_read_committed(kho, "acc1")
print(doc_lan1, doc_lan2)
```

```python title=test
kho2 = KhoLuuMVCC()
ts0 = kho2.timestamp_moi()
kho2.ghi("acc1", 100, ts0)
lan1 = doc_read_committed(kho2, "acc1")
assert lan1 == 100, "lan doc dau phai thay gia tri 100"

tsX = kho2.timestamp_moi()
kho2.ghi("acc1", 200, tsX)
lan2 = doc_read_committed(kho2, "acc1")
assert lan2 == 200, "lan doc sau, gia tri da doi, phai thay 200 -- MOI lan doc lay timestamp MOI"

tsY = kho2.timestamp_moi()
kho2.ghi("acc1", 300, tsY)
lan3 = doc_read_committed(kho2, "acc1")
assert lan3 == 300, "lan doc thu ba phai thay gia tri moi nhat 300"

kho3 = KhoLuuMVCC()
assert doc_read_committed(kho3, "khoa_la") is None, "khoa chua tung ghi phai ra None"
```

:::hints
- kind: attention
  body: "Goi kho.timestamp_moi() de lay timestamp MOI, roi goi kho.doc_snapshot(khoa, timestamp_do) -- hai buoc, co the viet thanh mot ham tra ve truc tiep."
- kind: strategy
  body: "ts_hien_tai = kho.timestamp_moi(); return kho.doc_snapshot(khoa, ts_hien_tai)"
- kind: one-line
  body: "return kho.doc_snapshot(khoa, kho.timestamp_moi())"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai lay mot timestamp MOI (kho.timestamp_moi()) roi doc_snapshot bang dung timestamp do
  requireAst:
  - kind: uses-name, target: kho, min: 8
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^100 200\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai mức cô lập — read committed đổi giữa chừng, snapshot isolation
nhất quán suốt. Nhưng nếu HAI giao dịch CÙNG ghi lên một khoá dựa
trên CÙNG một snapshot cũ, ai thắng?
::::

::::reflect{#nghi-lai}
Read committed LẤY timestamp mới cho MỖI lần đọc — nhanh thấy dữ
liệu MỚI nhất, nhưng KHÔNG đảm bảo hai lần đọc TRONG cùng giao dịch
nhất quán. Snapshot isolation cố định MỘT timestamp — nhất quán
tuyệt đối trong SUỐT giao dịch, đổi lại có THỂ "lỗi thời" so với dữ
liệu THẬT tại thời điểm commit. Nhưng cả hai mới chỉ nói VỀ ĐỌC —
NẾU hai giao dịch, cả hai đều đọc CÙNG một snapshot cũ, rồi CÙNG
ghi lên một khoá, chuyện GÌ xảy ra?
::::

::::checkpoint{mastery=0.8}
::::
