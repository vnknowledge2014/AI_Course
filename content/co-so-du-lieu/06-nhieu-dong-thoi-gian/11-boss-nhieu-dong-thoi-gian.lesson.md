---
id: co-so-du-lieu.nhieu-dong-thoi-gian.boss-nhieu-dong-thoi-gian
title: "BOSS — Nhiều dòng thời gian"
summary: "Ghép TRỌN q06: thu_commit_ssi kiểm tra CẢ xung đột ghi-ghi lẫn rw-conflict TRƯỚC khi cho phép commit — dựng lại kịch bản hai bác sĩ trực, T1 commit trước thành công, T2 bị CHẶN vì đã đọc đúng khoá T1 vừa ghi. Kết quả cuối: LUÔN còn ít nhất một bác sĩ trực — SSI đã ngăn được write skew mà snapshot isolation đơn thuần bỏ sót."
locale: vi
track: co-so-du-lieu
module: nhieu-dong-thoi-gian
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.version-gc]
concepts: [db.boss-q06]
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
Version chain, đọc theo snapshot, hai mức cô lập, xung đột ghi-ghi,
write skew, SSI, dọn phiên bản cũ — bảy mảnh. Ghép TẤT cả để NGĂN
được write skew hai bác sĩ trực trông ra sao?
::::

::::explain{#commit-qua-ssi}
`thu_commit_ssi` kiểm tra CẢ xung đột ghi-ghi LẪN rw-conflict TRƯỚC
khi cho phép commit — dựng LẠI kịch bản hai bác sĩ, `T1` commit
TRƯỚC:

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


def thu_commit_ssi(kho, nhat_ky_doc, id_giao_dich, ts_bat_dau, khoa_ghi, gia_tri_moi):
    if kho.co_xung_dot_ghi(khoa_ghi, ts_bat_dau):
        return False
    if co_rw_conflict(kho, nhat_ky_doc, id_giao_dich, ts_bat_dau):
        return False
    ts_commit = kho.timestamp_moi()
    kho.ghi(khoa_ghi, gia_tri_moi, ts_commit)
    return True


kho = KhoLuuMVCC()
ts0 = kho.timestamp_moi()
kho.ghi("bacsi1", "truc", ts0)
kho.ghi("bacsi2", "truc", ts0)

ts_T1 = ts0
nk = {}
ghi_lai_da_doc(nk, "T1", "bacsi1")
ghi_lai_da_doc(nk, "T1", "bacsi2")

ts_T2 = ts0
ghi_lai_da_doc(nk, "T2", "bacsi1")
ghi_lai_da_doc(nk, "T2", "bacsi2")

ok1 = thu_commit_ssi(kho, nk, "T1", ts_T1, "bacsi1", "nghi")
print(ok1)
```

```text title=readonly
True
```

`T1` VÀ `T2` cùng đọc snapshot ban ĐẦU (`ts0`), cả hai đã ĐỌC cả
hai bác sĩ (`ghi_lai_da_doc`). `T1` commit TRƯỚC — CHƯA ai ghi
`"bacsi1"` sau `ts_T1`, VÀ `T1` chưa hề đọc phải khoá NÀO bị ghi
sau — commit thành CÔNG, `"bacsi1"` giờ LÀ `"nghi"`.
::::

::::example{#t2-bi-chan}
`T2` giờ mới thử commit — nó đã ĐỌC `"bacsi1"`, và `"bacsi1"` VỪA
bị `T1` ghi:

```python title=readonly
ok2 = thu_commit_ssi(kho, nk, "T2", ts_T2, "bacsi2", "nghi")
print(ok2)
```

```text title=readonly
False
```

`co_rw_conflict` kiểm tra MỌI khoá `T2` đã đọc — GẶP `"bacsi1"`,
`co_xung_dot_ghi("bacsi1", ts_T2)` trả VỀ `True` (vì `T1` VỪA ghi
nó SAU `ts_T2`) — `T2` bị CHẶN commit, dù bản THÂN nó chỉ định ghi
`"bacsi2"`, một khoá HOÀN toàn khác.
::::

::::predict{#doan-trang-thai-cuoi-cung commitOnce}
Sau khi `T1` commit thành công VÀ `T2` bị chặn, đọc trạng THÁI cuối
cùng của cả hai bác sĩ:

```python
ts_cuoi = kho.timestamp_moi()
print(kho.doc_snapshot("bacsi1", ts_cuoi), kho.doc_snapshot("bacsi2", ts_cuoi))
```

Dòng cuối in ra gì?

:::opt{correct}
`nghi truc`
:::

:::opt
`nghi nghi` — vì CẢ hai giao dịch ĐỀU đã cố gắng ghi `"nghi"`, dù
`T2` bị TỪ chối, giá trị NÓ định ghi vẫn có thể "lọt" VÀO
::why
Gần đúng ở việc bạn nhớ ĐÚNG CẢ hai giao dịch đều có Ý định ghi
`"nghi"` — MỘT quan sát chính xác VỀ mục tiêu của chúng.

Chỗ lệch: `thu_commit_ssi` trả VỀ `False` NGAY tại dòng kiểm tra
`co_rw_conflict`, TRƯỚC khi chạm tới `kho.ghi(...)` — hàm `return
False` CHẶN đứng, `kho.ghi("bacsi2", "nghi", ...)` KHÔNG hề chạy.
`"bacsi2"` vẫn giữ nguyên `"truc"`.
::
:::

:::opt
`truc truc` — vì `T1` CŨNG lẽ ra phải bị chặn giống `T2`, không AI
được phép ghi khi đang CÓ một kịch bản write skew tiềm ẩn
::why
Gần đúng ở việc bạn thận trọng VỀ cả hai giao dịch — một PHẢN xạ
hợp lý sau khi thấy `T2` bị chặn.

Chỗ lệch: `T1` commit ĐẦU tiên, và TẠI thời điểm nó commit, CHƯA hề
có xung đột NÀO (chưa ai ghi trước NÓ) — `thu_commit_ssi` cho `T1`
qua ĐÚNG luật, trả về `True`, VÀ `kho.ghi("bacsi1", "nghi", ...)`
CHẠY thành công thật SỰ.
::
:::
::::

::::code{#viet_thu_commit_ssi}
Hoàn thiện `thu_commit_ssi` — sau khi đã kiểm tra xung đột ghi-ghi,
KIỂM tra thêm rw-conflict trước khi cho phép commit.

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


def thu_commit_ssi(kho, nhat_ky_doc, id_giao_dich, ts_bat_dau, khoa_ghi, gia_tri_moi):
    if kho.co_xung_dot_ghi(khoa_ghi, ts_bat_dau):
        return False
    ___
    ts_commit = kho.timestamp_moi()
    kho.ghi(khoa_ghi, gia_tri_moi, ts_commit)
    return True


kho = KhoLuuMVCC()
ts0 = kho.timestamp_moi()
kho.ghi("bacsi1", "truc", ts0)
kho.ghi("bacsi2", "truc", ts0)

ts_T1 = ts0
nk = {}
ghi_lai_da_doc(nk, "T1", "bacsi1")
ghi_lai_da_doc(nk, "T1", "bacsi2")

ts_T2 = ts0
ghi_lai_da_doc(nk, "T2", "bacsi1")
ghi_lai_da_doc(nk, "T2", "bacsi2")

ok1 = thu_commit_ssi(kho, nk, "T1", ts_T1, "bacsi1", "nghi")
ok2 = thu_commit_ssi(kho, nk, "T2", ts_T2, "bacsi2", "nghi")
print(ok1, ok2)
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


def thu_commit_ssi(kho, nhat_ky_doc, id_giao_dich, ts_bat_dau, khoa_ghi, gia_tri_moi):
    if kho.co_xung_dot_ghi(khoa_ghi, ts_bat_dau):
        return False
    if co_rw_conflict(kho, nhat_ky_doc, id_giao_dich, ts_bat_dau):
        return False
    ts_commit = kho.timestamp_moi()
    kho.ghi(khoa_ghi, gia_tri_moi, ts_commit)
    return True


kho = KhoLuuMVCC()
ts0 = kho.timestamp_moi()
kho.ghi("bacsi1", "truc", ts0)
kho.ghi("bacsi2", "truc", ts0)

ts_T1 = ts0
nk = {}
ghi_lai_da_doc(nk, "T1", "bacsi1")
ghi_lai_da_doc(nk, "T1", "bacsi2")

ts_T2 = ts0
ghi_lai_da_doc(nk, "T2", "bacsi1")
ghi_lai_da_doc(nk, "T2", "bacsi2")

ok1 = thu_commit_ssi(kho, nk, "T1", ts_T1, "bacsi1", "nghi")
ok2 = thu_commit_ssi(kho, nk, "T2", ts_T2, "bacsi2", "nghi")
print(ok1, ok2)
```

```python title=test
kho2 = KhoLuuMVCC()
ts0 = kho2.timestamp_moi()
kho2.ghi("bacsi1", "truc", ts0)
kho2.ghi("bacsi2", "truc", ts0)

ts_T1 = ts0
nk = {}
ghi_lai_da_doc(nk, "T1", "bacsi1")
ghi_lai_da_doc(nk, "T1", "bacsi2")
ts_T2 = ts0
ghi_lai_da_doc(nk, "T2", "bacsi1")
ghi_lai_da_doc(nk, "T2", "bacsi2")

ok1 = thu_commit_ssi(kho2, nk, "T1", ts_T1, "bacsi1", "nghi")
assert ok1 == True, "T1 commit truoc, chua ai xung dot -- phai thanh cong"

ok2 = thu_commit_ssi(kho2, nk, "T2", ts_T2, "bacsi2", "nghi")
assert ok2 == False, "T2 da doc bacsi1, bi T1 ghi sau -- phai bi chan"

ts_cuoi = kho2.timestamp_moi()
assert kho2.doc_snapshot("bacsi1", ts_cuoi) == "nghi", "bacsi1 phai da doi thanh nghi"
assert kho2.doc_snapshot("bacsi2", ts_cuoi) == "truc", "bacsi2 phai VAN con truc -- bat bien duoc giu"

kho3 = KhoLuuMVCC()
ts0b = kho3.timestamp_moi()
kho3.ghi("acc1", 100, ts0b)
nk3 = {}
ok3 = thu_commit_ssi(kho3, nk3, "TX", ts0b, "acc1", 200)
assert ok3 == True, "khong ai xung dot, khong doc gi truoc -- phai thanh cong"
ts_c = kho3.timestamp_moi()
assert kho3.doc_snapshot("acc1", ts_c) == 200, "acc1 phai da doi thanh 200"
```

:::hints
- kind: attention
  body: "Sau khi kiem tra xung dot ghi-ghi, goi co_rw_conflict(kho, nhat_ky_doc, id_giao_dich, ts_bat_dau) -- neu True thi return False ngay -- mot dong."
- kind: strategy
  body: "if co_rw_conflict(kho, nhat_ky_doc, id_giao_dich, ts_bat_dau): return False"
- kind: one-line
  body: "if co_rw_conflict(kho, nhat_ky_doc, id_giao_dich, ts_bat_dau): return False"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai kiem tra co_rw_conflict va return False neu co xung dot, truoc khi commit
  requireAst:
  - kind: uses-name, target: kho, min: 10
  - kind: uses-name, target: nhat_ky_doc, min: 3
  - kind: uses-name, target: id_giao_dich, min: 3
  - kind: uses-name, target: ts_bat_dau, min: 4
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True False\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một hệ thống MVCC hoàn chỉnh — snapshot nhất quán, xung đột ghi-ghi
bị chặn, write skew bị SSI ngăn CHẶN. q06 hoàn TẤT — VÀ cùng với
đó, R6-1 "nền lưu trữ một máy" khép LẠI ở đủ 68 bài.
::::

::::reflect{#nghi-lai}
q06 xây MVCC từ số 0: mỗi khoá giữ một chuỗi PHIÊN bản thay vì một
giá trị DUY nhất, đọc theo snapshot cố định KHÔNG bao giờ thấy ghi
xảy ra SAU, hai mức cô lập (read committed lấy timestamp MỚI mỗi
lần đọc, snapshot isolation cố định MỘT lần), xung đột ghi-ghi theo
"first-committer-wins", VÀ SSI mở rộng kiểm tra sang CẢ khoá đã đọc
để bắt được write skew — kịch bản hai bác sĩ trực chứng minh ĐIỀU
đó ngay Ở trên. Phiên bản CŨ không còn ai cần được dọn qua watermark.

**Phạm vi cắt có chủ đích**: MASTERPLAN liệt kê q06 gồm cả "ARIES-
lite" (phân tích/redo/undo qua WAL) — 11 bài NÀY KHÔNG bao gồm nó.
WAL cơ bản đã dạy Ở q02 ("Khi điện mất"), và ghép ARIES đầy đủ VỚI
MVCC sẽ vượt xa phạm vi một quest giới thiệu. Quyết định NÀY khớp
với ghi chú thu hẹp mục tiêu của MASTERPLAN cho BOSS R6: "heap file
+ B+Tree + WAL + MVCC 2 mức isolation" — ĐÚNG những gì q06 đã dạy.

Với q06 hoàn tất, **R6-1 "nền lưu trữ một máy" đã ĐỦ 68/68 bài**:
byte/file cố định (q00) → log ghi thêm (q01) → WAL/CRC32/recovery
(q02) → B+Tree (q03) → LSM (q04) → khoá/2PL/deadlock (q05) → MVCC
(q06). R6-2 chuyển hướng hoàn toàn: xây một NGÔN ngữ truy vấn VÀ bộ
thực thi TRÊN nền lưu trữ vừa xây — bắt đầu bằng "Ngôn ngữ của
Byte" (lexer, Pratt parser, AST cho một mini-SurrealQL). Cây cú
pháp trông NHƯ thế nào?
::::

::::checkpoint{mastery=0.85}
::::
