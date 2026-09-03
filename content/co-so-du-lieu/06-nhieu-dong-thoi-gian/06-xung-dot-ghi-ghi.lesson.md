---
id: co-so-du-lieu.nhieu-dong-thoi-gian.xung-dot-ghi-ghi
title: Xung đột ghi-ghi
summary: "co_xung_dot_ghi(khoa, ts_bat_dau) kiểm tra CÓ ai ghi khoá này SAU khi giao dịch của mình bắt đầu hay không — nếu CÓ, giao dịch phải huỷ nếu giờ muốn ghi (first-committer-wins). Ghi TẠI đúng thời điểm bắt đầu không tính là xung đột — chỉ ghi SAU mới tính."
locale: vi
track: co-so-du-lieu
module: nhieu-dong-thoi-gian
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [db.write-write-conflict]
requires: [db.two-isolation-levels]
concepts: [db.write-write-conflict]
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
Hai giao dịch, CÙNG đọc một snapshot cũ, RỒI cùng ghi lên MỘT khoá
— ai thắng? Cái gì đó phải PHÁT hiện chuyện này TRƯỚC khi cho phép
ghi.
::::

::::explain{#kiem-tra-xung-dot}
`co_xung_dot_ghi(khoa, ts_bat_dau)` kiểm tra CÓ ai ghi khoá NÀY sau
khi giao dịch của MÌNH bắt đầu hay không — "first-committer-wins":
nếu CÓ, giao dịch phải HUỶ nếu giờ muốn ghi:

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


kho = KhoLuuMVCC()
ts0 = kho.timestamp_moi()
kho.ghi("acc1", 100, ts0)

ts_T1 = ts0
ts_T2_ghi = kho.timestamp_moi()
kho.ghi("acc1", 200, ts_T2_ghi)

print(kho.co_xung_dot_ghi("acc1", ts_T1))
```

```text title=readonly
True
```

`T1` "bắt đầu" tại `ts_T1=ts0`. SAU đó `T2` ghi VÀ commit `200` tại
`ts_T2_ghi` — MỘT timestamp LỚN hơn `ts_T1`. `co_xung_dot_ghi`
tìm thấy phiên bản NÀY (`ts_T2_ghi > ts_T1`), trả VỀ `True` — `T1`
KHÔNG được phép ghi "acc1" nữa, vì dữ liệu đã ĐỔI kể từ khi nó bắt
đầu.
::::

::::example{#khong-ai-ghi-them-thi-khong-xung-dot}
Nếu KHÔNG ai ghi thêm sau khi giao dịch bắt đầu, KHÔNG có xung đột:

```python title=readonly
print(kho.co_xung_dot_ghi("acc1", ts_T2_ghi))
```

```text title=readonly
False
```

Kiểm tra TẠI `ts_T2_ghi` (thời điểm SAU cùng lần ghi mới NHẤT) —
KHÔNG còn phiên bản nào có `ts` LỚN hơn nó, vòng `for` chạy HẾT
không khớp gì, trả VỀ `False`.
::::

::::predict{#doan-ghi-dung-luc-bat-dau commitOnce}
Một khoá VỪA được ghi lần đầu tại `ts0`. Kiểm tra xung đột NGAY tại
ĐÚNG `ts0` đó (không SỚM hơn, không MUỘN hơn):

```python
kho2 = KhoLuuMVCC()
ts0_2 = kho2.timestamp_moi()
kho2.ghi("acc1", 100, ts0_2)
print(kho2.co_xung_dot_ghi("acc1", ts0_2))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì "acc1" ĐÃ được ghi Ở đúng `ts0_2`, VÀ đó chính LÀ
timestamp đang KIỂM tra, nên tính LÀ một xung đột
::why
Gần đúng ở việc bạn để Ý ĐÚNG `"acc1"` CÓ một phiên bản Ở CHÍNH
`ts0_2` — một quan SÁT chính xác về dữ liệu.

Chỗ lệch: điều kiện Ở TRONG vòng lặp LÀ `ts > ts_bat_dau` (SO sánh
NGHIÊM ngặt), không phải `ts >= ts_bat_dau` — phiên bản CÓ `ts ==
ts0_2` KHÔNG thoả `ts > ts0_2`. Một khoá được ghi ĐÚNG tại thời
điểm giao dịch bắt đầu KHÔNG tính LÀ "ghi sau" — đó chính LÀ dữ
liệu MÀ giao dịch nhìn thấy khi nó bắt đầu.
::
:::

:::opt
Máy báo lỗi — vì kiểm tra xung đột TẠI đúng timestamp của lần ghi
DUY nhất LÀ một trường hợp biên không hợp LỆ
::why
Gần đúng ở việc bạn nghĩ TỚI một trường hợp "biên" đáng NGỜ — một
sự thận trọng hợp LÝ khi phân tích code.

Chỗ lệch: `co_xung_dot_ghi` chỉ đơn thuần LÀ một vòng LẶP SO sánh số
— KHÔNG có `raise` nào, VÀ `ts == ts_bat_dau` hoàn toàn LÀ một
trường hợp BÌNH thường, được xử lý ĐÚNG bởi phép so sánh `>`.
::
:::
::::

::::code{#viet_co_xung_dot_ghi}
Hoàn thiện `co_xung_dot_ghi` — nếu CÓ phiên bản nào ghi SAU
`ts_bat_dau`, trả VỀ `True` ngay.

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
            ___
        return False


kho = KhoLuuMVCC()
ts0 = kho.timestamp_moi()
kho.ghi("acc1", 100, ts0)
ts_T1 = ts0
ts_T2_ghi = kho.timestamp_moi()
kho.ghi("acc1", 200, ts_T2_ghi)
print(kho.co_xung_dot_ghi("acc1", ts_T1))
print(kho.co_xung_dot_ghi("acc1", ts_T2_ghi))
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


kho = KhoLuuMVCC()
ts0 = kho.timestamp_moi()
kho.ghi("acc1", 100, ts0)
ts_T1 = ts0
ts_T2_ghi = kho.timestamp_moi()
kho.ghi("acc1", 200, ts_T2_ghi)
print(kho.co_xung_dot_ghi("acc1", ts_T1))
print(kho.co_xung_dot_ghi("acc1", ts_T2_ghi))
```

```python title=test
kho2 = KhoLuuMVCC()
ts0 = kho2.timestamp_moi()
kho2.ghi("acc1", 100, ts0)
assert kho2.co_xung_dot_ghi("acc1", ts0) == False, "ghi dung luc bat dau khong tinh la xung dot"

ts_moi = kho2.timestamp_moi()
kho2.ghi("acc1", 200, ts_moi)
assert kho2.co_xung_dot_ghi("acc1", ts0) == True, "co ghi SAU ts0 -- phai la xung dot"
assert kho2.co_xung_dot_ghi("acc1", ts_moi) == False, "khong ai ghi sau ts_moi -- khong xung dot"

kho3 = KhoLuuMVCC()
assert kho3.co_xung_dot_ghi("khoa_la", 5) == False, "khoa chua tung ton tai -- khong xung dot"
```

:::hints
- kind: attention
  body: "Trong than vong for, neu ts > ts_bat_dau thi return True ngay -- mot dong."
- kind: strategy
  body: "if ts > ts_bat_dau: return True"
- kind: one-line
  body: "if ts > ts_bat_dau: return True"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai return True ngay khi tim thay ts > ts_bat_dau trong vong lap
  requireAst:
  - kind: uses-name, target: ts_bat_dau, min: 1
  - kind: uses-name, target: ts, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Xung đột ghi-ghi phát hiện được — first-committer-wins. Nhưng CÓ
một hiện tượng LẠ mà cách kiểm tra NÀY không bắt được — write skew
LÀ gì?
::::

::::reflect{#nghi-lai}
`co_xung_dot_ghi` phát hiện đúng lúc HAI giao dịch cùng GHI lên
CÙNG một khoá dựa TRÊN cùng một snapshot cũ — "first-committer-
wins", giao dịch commit SAU phải huỷ nếu dữ liệu đã ĐỔI. Nhưng cách
kiểm tra NÀY chỉ nhìn VÀO một khoá DUY nhất tại một thời điểm — nếu
hai giao dịch ghi lên HAI khoá KHÁC nhau, dựa TRÊN cùng một snapshot,
`co_xung_dot_ghi` sẽ KHÔNG hề báo gì cả, dù kết quả CUỐI cùng có thể
VI phạm một quy tắc NGẦM nào đó liên kết hai khoá ĐÓ. Đây LÀ write
skew — nó trông NHƯ thế nào?
::::

::::checkpoint{mastery=0.8}
::::
