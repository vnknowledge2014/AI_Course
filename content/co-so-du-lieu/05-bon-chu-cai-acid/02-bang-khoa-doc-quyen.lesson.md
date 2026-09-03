---
id: co-so-du-lieu.bon-chu-cai-acid.bang-khoa-doc-quyen
title: Bảng khoá độc quyền
summary: "BangKhoa ghi lại giao dịch nào đang GIỮ mỗi khoá — xin_khoa CHỈ thành công nếu khoá đang RẢNH hoặc CHÍNH giao dịch đó đã giữ nó (tái nhập). tra_khoa CHỈ thả khoá nếu đúng CHỦ đang giữ gọi — giao dịch khác gọi tra_khoa KHÔNG có tác dụng gì."
locale: vi
track: co-so-du-lieu
module: bon-chu-cai-acid
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [db.exclusive-lock-table]
requires: [db.why-need-locks]
concepts: [db.exclusive-lock-table]
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
Mất cập nhật (bài TRƯỚC) xảy ra vì KHÔNG ai ngăn hai giao dịch cùng
động vào MỘT khoá. Cấu trúc đơn giản NHẤT để ngăn điều đó trông NHƯ
thế nào?
::::

::::explain{#bang-khoa}
`BangKhoa` ghi lại giao dịch NÀO đang giữ mỗi khoá — `xin_khoa`
CHỈ thành công nếu khoá đang RẢNH, hoặc CHÍNH giao dịch đó đã giữ
nó rồi (tái nhập):

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


bang = BangKhoa()
print(bang.xin_khoa("acc1", "T1"))
print(bang.xin_khoa("acc1", "T2"))
```

```text title=readonly
True
False
```

`T1` xin khoá `"acc1"` KHI nó đang RẢNH (`_giu_boi` trống) — thành
công, `True`. `T2` xin CÙNG khoá đó NGAY sau — `"acc1"` đã CÓ chủ
(`T1`, khác `T2`) — thất bại, `False`. `T2` phải CHỜ, không được
động vào `"acc1"` cho tới khi `T1` trả khoá.
::::

::::example{#tra-khoa-dung-chu}
`T1` trả khoá — `T2` giờ xin ĐƯỢC, vì khoá đã RẢNH:

```python title=readonly
bang.tra_khoa("acc1", "T1")
print(bang.xin_khoa("acc1", "T2"))
```

```text title=readonly
True
```

`tra_khoa("acc1", "T1")` xoá `"acc1"` khỏi `_giu_boi` — CHỈ vì
`"T1"` ĐÚNG là chủ hiện tại. Khoá RẢNH, `T2` xin thành CÔNG ngay.
::::

::::predict{#doan-tra-khoa-sai-chu commitOnce}
Một bảng khoá MỚI — `T1` xin khoá `"acc1"`, RỒI `T2` (KHÔNG hề giữ
khoá đó) thử TRẢ nó:

```python
bang2 = BangKhoa()
bang2.xin_khoa("acc1", "T1")
bang2.tra_khoa("acc1", "T2")
print(bang2.xin_khoa("acc1", "T2"))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì `tra_khoa` chỉ CẦN biết TÊN khoá, ai gọi CŨNG trả được,
nên `"acc1"` đã RẢNH khi `T2` xin lại
::why
Gần đúng ở việc bạn nghĩ TỚI một API "trả khoá" ĐƠN giản, không
kiểm ai gọi — MỘT thiết kế hợp lý cho một số hệ thống khác.

Chỗ lệch: `tra_khoa` KIỂM tra `self._giu_boi.get(khoa) ==
id_giao_dich` TRƯỚC khi xoá — `T2` KHÔNG phải chủ hiện tại của
`"acc1"` (chủ LÀ `T1`), nên điều kiện SAI, `del` KHÔNG chạy. Khoá
VẪN thuộc về `T1`, `T2` xin LẠI vẫn thất BẠI.
::
:::

:::opt
Máy báo lỗi — vì `T2` gọi `tra_khoa` trên MỘT khoá nó KHÔNG hề giữ
LÀ thao tác không hợp lệ
::why
Gần đúng ở việc bạn nghĩ TỚI một ràng buộc "chỉ chủ mới ĐƯỢC trả"
hợp lý — ĐÚNG tinh thần, sai CƠ chế thực thi.

Chỗ lệch: `tra_khoa` KHÔNG hề `raise` — nó chỉ ÂM thầm KHÔNG làm gì
nếu người gọi KHÔNG phải chủ (điều kiện `if` đơn giản SAI thì bỏ
qua), không có exception nào cả.
::
:::
::::

::::code{#viet_tra_khoa}
Hoàn thiện `tra_khoa(khoa, id_giao_dich)` — CHỈ xoá khoá nếu ĐÚNG
giao dịch đang giữ nó gọi.

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
        ___


bang = BangKhoa()
bang.xin_khoa("acc1", "T1")
bang.tra_khoa("acc1", "T1")
print(bang.xin_khoa("acc1", "T2"))
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


bang = BangKhoa()
bang.xin_khoa("acc1", "T1")
bang.tra_khoa("acc1", "T1")
print(bang.xin_khoa("acc1", "T2"))
```

```python title=test
b = BangKhoa()
b.xin_khoa("acc1", "T1")
b.tra_khoa("acc1", "T2")
assert b.xin_khoa("acc1", "T2") == False, "T2 khong phai chu, tra khoa phai khong co tac dung"
b.tra_khoa("acc1", "T1")
assert b.xin_khoa("acc1", "T2") == True, "T1 dung la chu, tra khoa phai giai phong acc1"

b2 = BangKhoa()
b2.tra_khoa("khoa_chua_ai_giu", "T1")
assert b2.xin_khoa("khoa_chua_ai_giu", "T1") == True, "tra mot khoa chua ai giu khong duoc gay loi"
```

:::hints
- kind: attention
  body: "Neu self._giu_boi.get(khoa) == id_giao_dich, xoa khoa khoi _giu_boi bang del."
- kind: strategy
  body: "if self._giu_boi.get(khoa) == id_giao_dich: del self._giu_boi[khoa]"
- kind: one-line
  body: "if self._giu_boi.get(khoa) == id_giao_dich: del self._giu_boi[khoa]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai xoa khoa khoi _giu_boi CHI khi id_giao_dich dung la chu hien tai
  requireAst:
  - kind: uses-name, target: khoa, min: 4
  - kind: uses-name, target: id_giao_dich, min: 3
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bảng khoá độc quyền — MỘT giao dịch giữ, mọi giao dịch KHÁC bị chặn.
Nhưng khi khoá bận, giao dịch bị chặn phải làm GÌ — dừng hẳn, hay
chờ?
::::

::::reflect{#nghi-lai}
`BangKhoa` đủ để NGĂN hai giao dịch cùng giữ MỘT khoá — nhưng bản
thân `xin_khoa` chỉ trả về `True`/`False` NGAY lập tức, không hề
"chờ". Một giao dịch bị TỪ chối (`False`) hiện tại chỉ CÓ thể bỏ
cuộc — KHÔNG thực tế, vì khoá RẤT có thể sẽ rảnh CHỈ sau một CHÚT.
Chờ khi khoá đã bị GIỮ diễn ra thế nào?
::::

::::checkpoint{mastery=0.8}
::::
