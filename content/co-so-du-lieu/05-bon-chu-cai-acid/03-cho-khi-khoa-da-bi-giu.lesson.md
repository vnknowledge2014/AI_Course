---
id: co-so-du-lieu.bon-chu-cai-acid.cho-khi-khoa-da-bi-giu
title: Chờ khi khoá đã bị giữ
summary: "xin_khoa_co_cho thử xin khoá LẶP LẠI tối đa so_lan_thu lần (vòng for có GIỚI HẠN, không chờ mãi) — thành công NGAY khi một lần thử ra True, thất bại HẲN nếu hết số lần thử mà vẫn không xin được. Nếu so_lan_thu=0, hàm KHÔNG hề thử lần nào, trả về False dù khoá có đang rảnh."
locale: vi
track: co-so-du-lieu
module: bon-chu-cai-acid
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [db.wait-for-lock]
requires: [db.exclusive-lock-table]
concepts: [db.wait-for-lock]
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
`xin_khoa` (bài TRƯỚC) trả về `False` NGAY khi khoá bận — nhưng bỏ
cuộc ngay LẬP tức không thực tế, vì khoá RẤT có thể sẽ rảnh chỉ sau
một CHÚT. Chờ trông như THẾ nào?
::::

::::explain{#cho-co-gioi-han}
`xin_khoa_co_cho` thử xin khoá LẶP lại tối đa `so_lan_thu` lần —
một vòng `for` CÓ giới hạn, không chờ MÃI mãi — thành công NGAY khi
một lần thử ra `True`:

```python title=readonly
class BangKhoa:
    def __init__(self):
        self._giu_boi = {}

    def xin_khoa(self, khoa, id_giao_dich):
        chu_hien_tai = self._giu_boi.get(khoa)
        if chu_hien_tai is None or chu_hien_tai == id_giao_dich:
            self._giu_boi[khoa] = id_giao_dich
            return True
        return False

    def tra_khoa(self, khoa, id_giao_dich):
        if self._giu_boi.get(khoa) == id_giao_dich:
            del self._giu_boi[khoa]


def xin_khoa_co_cho(bang, khoa, id_giao_dich, so_lan_thu):
    for _ in range(so_lan_thu):
        if bang.xin_khoa(khoa, id_giao_dich):
            return True
    return False


bang = BangKhoa()
bang.xin_khoa("acc1", "T1")
print(xin_khoa_co_cho(bang, "acc1", "T2", 3))
```

```text title=readonly
False
```

`T1` giữ `"acc1"` VÀ không hề trả — MỖI lần `T2` thử (`3` lần) đều
gặp khoá BẬN, `xin_khoa` trả `False` liên tục. Hết `3` lần thử,
vòng `for` chạy XONG, hàm trả VỀ `False` — `T2` từ bỏ, KHÔNG chờ
mãi mãi.
::::

::::example{#khoa-da-ranh-tu-dau}
Nếu khoá đã RẢNH ngay từ đầu, hàm thành CÔNG ở lần thử ĐẦU tiên,
không cần dùng hết `so_lan_thu`:

```python title=readonly
bang2 = BangKhoa()
print(xin_khoa_co_cho(bang2, "acc2", "T3", 3))
```

```text title=readonly
True
```

`"acc2"` chưa AI giữ — lần thử ĐẦU tiên trong vòng `for` đã thành
CÔNG, `xin_khoa_co_cho` `return True` NGAY, không đi tiếp lần thử
thứ hai HAY ba.
::::

::::predict{#doan-so-lan-thu-bang-0 commitOnce}
Một khoá HOÀN toàn rảnh (`"acc3"`, chưa ai đụng tới), NHƯNG gọi
`xin_khoa_co_cho` với `so_lan_thu=0`:

```python
bang3 = BangKhoa()
print(xin_khoa_co_cho(bang3, "acc3", "T4", 0))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì `"acc3"` đang RẢNH, `xin_khoa_co_cho` sẽ vẫn thử xin ÍT
nhất MỘT lần bất kể `so_lan_thu` là gì
::why
Gần đúng ở việc bạn nhớ ĐÚNG `"acc3"` THẬT sự đang rảnh — quan sát
chính XÁC về trạng thái khoá.

Chỗ lệch: `range(0)` LÀ một dãy RỖNG — vòng `for _ in range(0):`
KHÔNG chạy dù chỉ MỘT lần, thân vòng lặp (nơi GỌI `bang.xin_khoa`)
không hề được THỰC thi. Hàm rơi thẳng TỚI `return False`, dù khoá
rảnh — nó CHƯA hề được thử xin.
::
:::

:::opt
Máy báo lỗi — vì `so_lan_thu=0` LÀ một tham số không hợp LỆ cho một
hàm "thử lặp lại"
::why
Gần đúng ở việc bạn nghĩ TỚI `0` như một giá trị "vô nghĩa" cho một
tham số ĐẾM số lần — một trực giác hợp lý.

Chỗ lệch: `range(0)` KHÔNG hề gây lỗi — nó chỉ đơn giản LÀ một dãy
rỗng, vòng `for` chạy XONG ngay (0 vòng lặp), KHÔNG có exception
nào. Python xử lý `range(0)` hoàn TOÀN bình thường.
::
:::
::::

::::code{#viet_xin_khoa_co_cho}
Hoàn thiện `xin_khoa_co_cho` — mỗi lần thử TRONG vòng `for`, nếu
`xin_khoa` thành công thì trả VỀ `True` ngay.

```python title=starter
class BangKhoa:
    def __init__(self):
        self._giu_boi = {}

    def xin_khoa(self, khoa, id_giao_dich):
        chu_hien_tai = self._giu_boi.get(khoa)
        if chu_hien_tai is None or chu_hien_tai == id_giao_dich:
            self._giu_boi[khoa] = id_giao_dich
            return True
        return False

    def tra_khoa(self, khoa, id_giao_dich):
        if self._giu_boi.get(khoa) == id_giao_dich:
            del self._giu_boi[khoa]


def xin_khoa_co_cho(bang, khoa, id_giao_dich, so_lan_thu):
    for _ in range(so_lan_thu):
        ___
    return False


bang = BangKhoa()
print(xin_khoa_co_cho(bang, "acc1", "T2", 3))
```

```python title=solution
class BangKhoa:
    def __init__(self):
        self._giu_boi = {}

    def xin_khoa(self, khoa, id_giao_dich):
        chu_hien_tai = self._giu_boi.get(khoa)
        if chu_hien_tai is None or chu_hien_tai == id_giao_dich:
            self._giu_boi[khoa] = id_giao_dich
            return True
        return False

    def tra_khoa(self, khoa, id_giao_dich):
        if self._giu_boi.get(khoa) == id_giao_dich:
            del self._giu_boi[khoa]


def xin_khoa_co_cho(bang, khoa, id_giao_dich, so_lan_thu):
    for _ in range(so_lan_thu):
        if bang.xin_khoa(khoa, id_giao_dich):
            return True
    return False


bang = BangKhoa()
print(xin_khoa_co_cho(bang, "acc1", "T2", 3))
```

```python title=test
b = BangKhoa()
assert xin_khoa_co_cho(b, "acc1", "T2", 3) == True, "khoa dang ranh, phai xin duoc ngay lan dau"

b2 = BangKhoa()
b2.xin_khoa("acc1", "T1")
assert xin_khoa_co_cho(b2, "acc1", "T2", 3) == False, "T1 giu mai khong tra, T2 phai het luot ma van khong xin duoc"

b3 = BangKhoa()
assert xin_khoa_co_cho(b3, "acc1", "T2", 0) == False, "so_lan_thu=0 khong duoc thu lan nao ca, du khoa dang ranh"

b4 = BangKhoa()
assert b4.xin_khoa("acc1", "T2") == True, "sau khi xin_khoa_co_cho tra ve True, khoa phai THAT SU duoc giu"
```

:::hints
- kind: attention
  body: "Trong than vong for, neu bang.xin_khoa(khoa, id_giao_dich) tra ve True thi return True ngay."
- kind: strategy
  body: "if bang.xin_khoa(khoa, id_giao_dich): return True"
- kind: one-line
  body: "if bang.xin_khoa(khoa, id_giao_dich): return True"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi bang.xin_khoa va return True ngay khi thanh cong, ben trong vong for
  requireAst:
  - kind: uses-name, target: bang, min: 2
  - kind: uses-name, target: khoa, min: 5
  - kind: uses-name, target: id_giao_dich, min: 4
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chờ CÓ giới hạn — không mãi mãi. Nhưng chờ ĐỂ xin khoá ĐỘC quyền
cho MỌI thao tác (kể cả chỉ ĐỌC) có phung phí không — hai giao dịch
CÙNG chỉ đọc thì có cần chặn nhau không?
::::

::::reflect{#nghi-lai}
`xin_khoa_co_cho` giới hạn số lần THỬ — không chờ mãi mãi, tránh
một giao dịch bị TREO vô thời hạn. Nhưng `BangKhoa` (bài trước) chỉ
CÓ một loại khoá: ĐỘC quyền — dù hai giao dịch CHỈ đọc (không hề
ghi gì), chúng VẪN chặn nhau, vì `xin_khoa` không phân biệt "đọc"
hay "ghi". Đọc thì NHIỀU giao dịch làm CÙNG lúc được — chỉ ghi mới
cần ĐỘC quyền thật sự. Khoá đọc VÀ khoá ghi khác nhau THẾ nào?
::::

::::checkpoint{mastery=0.8}
::::
