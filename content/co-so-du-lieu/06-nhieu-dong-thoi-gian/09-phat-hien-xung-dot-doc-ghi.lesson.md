---
id: co-so-du-lieu.nhieu-dong-thoi-gian.phat-hien-xung-dot-doc-ghi
title: Phát hiện xung đột đọc-ghi
summary: "co_rw_conflict duyệt qua MỌI khoá một giao dịch đã ĐỌC (ghi lại bởi ghi_lai_da_doc), kiểm tra TỪNG khoá bằng co_xung_dot_ghi — nếu BẤT KỲ khoá nào trong số đó bị GHI sau khi giao dịch bắt đầu, đó LÀ một rw-conflict. Giao dịch KHÔNG hề đọc một khoá nào đó thì việc GHI khoá ấy không ảnh hưởng gì tới nó."
locale: vi
track: co-so-du-lieu
module: nhieu-dong-thoi-gian
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [db.read-write-conflict-detection]
requires: [db.ssi-idea]
concepts: [db.read-write-conflict-detection]
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
SSI (bài TRƯỚC) cần theo dõi MỌI khoá một giao dịch đã ĐỌC — cài
đặt việc GHI lại đó VÀ kiểm tra xung đột TRÊN từng khoá trông NHƯ
thế nào?
::::

::::explain{#ghi-lai-va-kiem-tra}
`ghi_lai_da_doc` ghi lại KHOÁ một giao dịch vừa ĐỌC. `co_rw_conflict`
duyệt qua MỌI khoá giao dịch ĐÓ đã đọc, kiểm tra TỪNG khoá bằng
`co_xung_dot_ghi` — CHỈ cần MỘT khoá bị ghi sau LÀ đủ để báo nguy
hiểm:

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

    def co_xung_dot_ghi(self, khoa, ts_bat_dau):
        for (ts, gia_tri) in self._phien_ban.get(khoa, []):
            if ts > ts_bat_dau:
                return True
        return False


def ghi_lai_da_doc(nhat_ky_doc, id_giao_dich, khoa):
    nhat_ky_doc.setdefault(id_giao_dich, set()).add(khoa)


def co_rw_conflict(kho, nhat_ky_doc, id_giao_dich, ts_bat_dau):
    for khoa in nhat_ky_doc.get(id_giao_dich, set()):
        if kho.co_xung_dot_ghi(khoa, ts_bat_dau):
            return True
    return False


kho = KhoLuuMVCC()
ts0 = kho.timestamp_moi()
kho.ghi("bacsi1", "truc", ts0)
kho.ghi("bacsi2", "truc", ts0)

ts_T1 = ts0
nhat_ky_doc = {}
ghi_lai_da_doc(nhat_ky_doc, "T1", "bacsi1")
ghi_lai_da_doc(nhat_ky_doc, "T1", "bacsi2")

ts_ghi = kho.timestamp_moi()
kho.ghi("bacsi2", "nghi", ts_ghi)

print(co_rw_conflict(kho, nhat_ky_doc, "T1", ts_T1))
```

```text title=readonly
True
```

`T1` đã đọc CẢ `"bacsi1"` LẪN `"bacsi2"` (ghi lại qua `ghi_lai_
da_doc`). Sau đó, `"bacsi2"` bị GHI bởi giao dịch KHÁC. `co_rw_
conflict` duyệt QUA cả hai khoá `T1` đã đọc — gặp `"bacsi2"`,
`co_xung_dot_ghi` báo `True`, hàm dừng NGAY, trả về `True`.
::::

::::example{#khong-doc-thi-khong-anh-huong}
Nếu `T1` KHÔNG hề đọc khoá bị GHI, việc ghi đó không ảnh hưởng gì
tới `T1`:

```python title=readonly
nhat_ky_doc2 = {}
ghi_lai_da_doc(nhat_ky_doc2, "T2", "bacsi1")
print(co_rw_conflict(kho, nhat_ky_doc2, "T2", ts_T1))
```

```text title=readonly
False
```

`T2` chỉ đọc `"bacsi1"` — nó CHƯA từng đọc `"bacsi2"`. Dù `"bacsi2"`
ĐÃ bị ghi (sau `ts_T1`), điều đó KHÔNG hề nằm trong tập khoá `T2`
quan TÂM — `co_rw_conflict` trả VỀ `False`.
::::

::::predict{#doan-khong-doc-gi-ca commitOnce}
Một giao dịch `T3` CHƯA từng gọi `ghi_lai_da_doc` — tức LÀ chưa hề
đọc khoá NÀO cả:

```python
nhat_ky_doc3 = {}
print(co_rw_conflict(kho, nhat_ky_doc3, "T3", ts_T1))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì `T3` KHÔNG hề "khai báo" khoá nào NÓ quan tâm, hệ thống
phải AN toàn mặc định NGHI ngờ mọi khoá đều có THỂ xung đột
::why
Gần đúng ở việc bạn nghĩ TỚI một nguyên tắc "AN toàn mặc định" hợp
lý — một triết lý thiết kế thận trọng cho MỘT số hệ thống bảo mật
khác.

Chỗ lệch: `nhat_ky_doc.get(id_giao_dich, set())` trả VỀ một `set`
RỖNG khi `"T3"` chưa từng xuất hiện TRONG `nhat_ky_doc` — vòng `for`
KHÔNG chạy vòng nào, hàm rơi thẳng TỚI `return False`. Không đọc GÌ
nghĩa LÀ không có gì để xung đột — `False` LÀ kết quả đúng, không
phải một "lỗ hổng".
::
:::

:::opt
Máy báo lỗi — vì gọi `co_rw_conflict` cho một giao dịch CHƯA từng
đọc gì LÀ thao tác không hợp lệ
::why
Gần đúng ở việc bạn nghĩ TỚI một ràng buộc "phải đọc TRƯỚC khi kiểm
tra" hợp lý cho MỘT số quy trình khác.

Chỗ lệch: `.get(id_giao_dich, set())` LÀ một cách TRA cứu AN toàn,
KHÔNG hề `raise` khi khoá không tồn TẠI — nó chỉ đơn giản trả VỀ
giá trị mặc định (`set()` rỗng), một hành vi HOÀN toàn bình thường
của `dict.get`.
::
:::
::::

::::code{#viet_co_rw_conflict}
Hoàn thiện `co_rw_conflict` — với MỖI khoá giao dịch đã đọc, nếu CÓ
xung đột ghi, trả VỀ `True` ngay.

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

    def co_xung_dot_ghi(self, khoa, ts_bat_dau):
        for (ts, gia_tri) in self._phien_ban.get(khoa, []):
            if ts > ts_bat_dau:
                return True
        return False


def ghi_lai_da_doc(nhat_ky_doc, id_giao_dich, khoa):
    nhat_ky_doc.setdefault(id_giao_dich, set()).add(khoa)


def co_rw_conflict(kho, nhat_ky_doc, id_giao_dich, ts_bat_dau):
    for khoa in nhat_ky_doc.get(id_giao_dich, set()):
        ___
    return False


kho = KhoLuuMVCC()
ts0 = kho.timestamp_moi()
kho.ghi("bacsi1", "truc", ts0)
kho.ghi("bacsi2", "truc", ts0)
ts_T1 = ts0
nhat_ky_doc = {}
ghi_lai_da_doc(nhat_ky_doc, "T1", "bacsi1")
ghi_lai_da_doc(nhat_ky_doc, "T1", "bacsi2")
ts_ghi = kho.timestamp_moi()
kho.ghi("bacsi2", "nghi", ts_ghi)
print(co_rw_conflict(kho, nhat_ky_doc, "T1", ts_T1))
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

    def co_xung_dot_ghi(self, khoa, ts_bat_dau):
        for (ts, gia_tri) in self._phien_ban.get(khoa, []):
            if ts > ts_bat_dau:
                return True
        return False


def ghi_lai_da_doc(nhat_ky_doc, id_giao_dich, khoa):
    nhat_ky_doc.setdefault(id_giao_dich, set()).add(khoa)


def co_rw_conflict(kho, nhat_ky_doc, id_giao_dich, ts_bat_dau):
    for khoa in nhat_ky_doc.get(id_giao_dich, set()):
        if kho.co_xung_dot_ghi(khoa, ts_bat_dau):
            return True
    return False


kho = KhoLuuMVCC()
ts0 = kho.timestamp_moi()
kho.ghi("bacsi1", "truc", ts0)
kho.ghi("bacsi2", "truc", ts0)
ts_T1 = ts0
nhat_ky_doc = {}
ghi_lai_da_doc(nhat_ky_doc, "T1", "bacsi1")
ghi_lai_da_doc(nhat_ky_doc, "T1", "bacsi2")
ts_ghi = kho.timestamp_moi()
kho.ghi("bacsi2", "nghi", ts_ghi)
print(co_rw_conflict(kho, nhat_ky_doc, "T1", ts_T1))
```

```python title=test
kho2 = KhoLuuMVCC()
ts0 = kho2.timestamp_moi()
kho2.ghi("bacsi1", "truc", ts0)
kho2.ghi("bacsi2", "truc", ts0)
ts_T1 = ts0

nhat_ky_doc = {}
ghi_lai_da_doc(nhat_ky_doc, "T1", "bacsi1")
ghi_lai_da_doc(nhat_ky_doc, "T1", "bacsi2")
ts_ghi = kho2.timestamp_moi()
kho2.ghi("bacsi2", "nghi", ts_ghi)
assert co_rw_conflict(kho2, nhat_ky_doc, "T1", ts_T1) == True, "bacsi2 da doc, bi ghi sau -- phai la rw-conflict"

nhat_ky_doc2 = {}
ghi_lai_da_doc(nhat_ky_doc2, "T2", "bacsi1")
assert co_rw_conflict(kho2, nhat_ky_doc2, "T2", ts_T1) == False, "T2 chi doc bacsi1, khong bi anh huong boi ghi tren bacsi2"

nhat_ky_doc3 = {}
assert co_rw_conflict(kho2, nhat_ky_doc3, "T3", ts_T1) == False, "khong doc gi ca -- khong the co rw-conflict"
```

:::hints
- kind: attention
  body: "Trong than vong for, neu kho.co_xung_dot_ghi(khoa, ts_bat_dau) la True thi return True ngay -- mot dong."
- kind: strategy
  body: "if kho.co_xung_dot_ghi(khoa, ts_bat_dau): return True"
- kind: one-line
  body: "if kho.co_xung_dot_ghi(khoa, ts_bat_dau): return True"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi kho.co_xung_dot_ghi cho tung khoa, return True ngay khi tim thay xung dot
  requireAst:
  - kind: uses-name, target: kho, min: 7
  - kind: uses-name, target: khoa, min: 4
  - kind: uses-name, target: ts_bat_dau, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Phát hiện được rw-conflict — SSI hoàn chỉnh hơn hẳn kiểm tra ghi-
ghi đơn thuần. Nhưng chuỗi phiên bản CÀNG dài, càng tốn KHÔNG gian
— dọn phiên bản CŨ thế nào?
::::

::::reflect{#nghi-lai}
`co_rw_conflict` hoàn thiện Ý tưởng SSI thành CODE thật: theo dõi
MỌI khoá một giao dịch đã đọc, kiểm TRA từng khoá bằng chính hàm
xung đột ghi-ghi ĐÃ có — không cần MỘT cơ chế hoàn toàn mới, chỉ
cần ÁP dụng nó cho ĐÚNG tập khoá (đã đọc, không CHỈ sẽ ghi). Kết
hợp VỚI `co_xung_dot_ghi` (bài trước), hệ THỐNG giờ bắt được cả
write-write LẪN write skew. Nhưng mỗi lần GHI đều thêm một phiên
bản MỚI, không hề xoá bản CŨ — chuỗi phiên bản CHỈ dài thêm mãi.
Dọn dẹp những phiên bản KHÔNG ai còn cần diễn RA thế nào?
::::

::::checkpoint{mastery=0.8}
::::
