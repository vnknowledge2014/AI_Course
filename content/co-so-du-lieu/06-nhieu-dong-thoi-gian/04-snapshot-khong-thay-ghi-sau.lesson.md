---
id: co-so-du-lieu.nhieu-dong-thoi-gian.snapshot-khong-thay-ghi-sau
title: Snapshot không thấy ghi sau
summary: "Một giao dịch chụp SNAPSHOT tại một timestamp cố định NGAY khi bắt đầu — mọi lần đọc SAU đó, dù muộn bao lâu, đều dùng ĐÚNG timestamp cố định đó. Kết quả: giao dịch KHÔNG BAO GIỜ thấy một lần ghi xảy ra SAU khi nó bắt đầu, kể cả khi lần ghi đó đã 'committed' từ lâu."
locale: vi
track: co-so-du-lieu
module: nhieu-dong-thoi-gian
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [db.snapshot-isolation-idea]
requires: [db.snapshot-read]
concepts: [db.snapshot-isolation-idea]
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
`doc_snapshot` (bài TRƯỚC) nhận MỘT timestamp làm THAM số — nếu một
giao dịch dùng ĐÚNG một timestamp cố định CHO mọi lần đọc, chuyện
gì xảy ra khi CÓ ai ghi thêm Ở giữa CHỪNG?
::::

::::explain{#doc-lai-van-thay-cu}
Giao dịch `T1` chụp SNAPSHOT tại `ts_T1` NGAY khi bắt đầu. `T2` ghi
VÀ commit một giá trị MỚI sau đó — nhưng `T1` đọc LẠI bằng ĐÚNG
`ts_T1` cũ, vẫn thấy giá trị CŨ:

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
ts_dau = kho.timestamp_moi()
kho.ghi("acc1", 100, ts_dau)

ts_T1 = ts_dau
print(kho.doc_snapshot("acc1", ts_T1))

ts_T2 = kho.timestamp_moi()
kho.ghi("acc1", 999, ts_T2)

print(kho.doc_snapshot("acc1", ts_T1))
```

```text title=readonly
100
100
```

`T1` đọc LẦN đầu, thấy `100` — hoàn toàn bình thường. `T2` sau đó
GHI VÀ commit `999`. `T1` đọc LẠI, nhưng vẫn dùng ĐÚNG `ts_T1` (nó
KHÔNG hề đổi) — kết quả VẪN LÀ `100`, dù `999` đã tồn tại thật SỰ
trong kho từ lâu.
::::

::::example{#doc-bang-snapshot-moi-thay-khac}
Nếu đọc bằng một snapshot MỚI (`ts_T2`, chụp SAU khi `T2` ghi), kết
quả HOÀN toàn khác:

```python title=readonly
print(kho.doc_snapshot("acc1", ts_T2))
```

```text title=readonly
999
```

CÙNG một khoá `"acc1"`, CÙNG kho `kho` — nhưng đọc bằng `ts_T2`
(SAU khi `T2` ghi) thấy `999`. Khác BIỆT hoàn toàn KHÔNG nằm Ở dữ
liệu, mà Ở CHÍNH cái timestamp dùng để đọc — MỘT giao dịch cố định
timestamp của NÓ sẽ luôn thấy MỘT bức tranh nhất quán, bất kể thế
giới bên NGOÀI đã đổi bao NHIÊU lần.
::::

::::predict{#doan-doc-tai-thoi-diem-giua commitOnce}
`T2` ghi VÀ commit `999` tại `ts_T2`. Một giao dịch THỨ ba, `T3`,
chụp snapshot NGAY TRƯỚC khi `T2` ghi (tức LÀ vẫn dùng `ts_dau`,
giống `T1`):

```python
ts_T3 = ts_dau
print(kho.doc_snapshot("acc1", ts_T3))
```

Dòng cuối in ra gì?

:::opt{correct}
`100`
:::

:::opt
`999` — vì `T3` được TẠO ra SAU khi `T2` đã ghi, nên NÓ phải thấy
dữ liệu MỚI nhất tại thời điểm nó XUẤT hiện
::why
Gần đúng ở việc bạn nghĩ TỚI thứ TỰ "tạo ra" của các giao dịch TRONG
code — MỘT trực giác hợp lý nếu snapshot gắn VỚI thời điểm chạy
CODE thay vì một CON số cụ thể.

Chỗ lệch: `doc_snapshot` CHỈ quan tâm ĐÚNG giá trị `ts_T3` được
TRUYỀN vào, không quan TÂM dòng code NÀY chạy lúc nào — `ts_T3 =
ts_dau` gán LẠI giá trị timestamp CŨ (TRƯỚC khi `T2` ghi), nên kết
quả VẪN LÀ `100`, giống HỆT `T1`.
::
:::

:::opt
Máy báo lỗi — vì hai giao dịch (`T1` VÀ `T3`) dùng CHUNG một
timestamp (`ts_dau`) LÀ một xung đột không hợp lệ
::why
Gần đúng ở việc bạn nghĩ TỚI việc "trùng timestamp" như một vấn
đề — một mối lo hợp lý cho một SỐ hệ thống định danh khác.

Chỗ lệch: `doc_snapshot` chỉ ĐƠN thuần so sánh SỐ, KHÔNG hề kiểm tra
xem timestamp ĐÓ có đang "được dùng" Ở đâu KHÁC hay không — nhiều
giao dịch hoàn toàn CÓ thể đọc CÙNG một snapshot mà KHÔNG xung đột
gì (đây chính LÀ điều làm MVCC hữu ích: đọc KHÔNG bao giờ chặn ai).
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một timestamp cố định — một bức tranh nhất QUÁN, dù thế giới ngoài
kia đã đổi. Nhưng NẾU mỗi lần đọc lấy timestamp MỚI thay vì cố
định, khác gì?
::::

::::reflect{#nghi-lai}
Cố định `ts_snapshot` NGAY khi bắt đầu — VÀ dùng đúng CON số đó cho
MỌI lần đọc trong suốt vòng ĐỜI giao dịch — LÀ ý tưởng cốt lõi cho
phép một giao dịch thấy một bức TRANH nhất quán, dù dữ liệu THẬT sự
đã thay đổi Ở đâu đó SAU lưng nó. Nhưng đây CHỈ là MỘT cách dùng
timestamp — nếu MỖI lần đọc lấy một timestamp MỚI (thay VÌ cố định
một lần), hành vi sẽ khác HẲN. Hai cách nÀY LÀ hai mức cô lập khác
nhau — chênh lệch cụ thể LÀ gì?
::::

::::checkpoint{mastery=0.8}
::::
