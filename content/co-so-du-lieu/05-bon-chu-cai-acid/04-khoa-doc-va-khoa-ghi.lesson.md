---
id: co-so-du-lieu.bon-chu-cai-acid.khoa-doc-va-khoa-ghi
title: Khoá đọc và khoá ghi
summary: "BangKhoaSE tách hai loại khoá — khoá ĐỌC (nhiều giao dịch giữ cùng lúc được) và khoá GHI (độc quyền, chặn cả người đọc lẫn người ghi khác). xin_khoa_ghi CHỈ thành công nếu KHÔNG ai khác đang đọc HAY ghi cùng khoá đó."
locale: vi
track: co-so-du-lieu
module: bon-chu-cai-acid
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [db.shared-exclusive-lock]
requires: [db.wait-for-lock]
concepts: [db.shared-exclusive-lock]
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
`BangKhoa` (bài TRƯỚC) chỉ có MỘT loại khoá — độc quyền. Hai giao
dịch CHỈ đọc, không hề ghi gì, VẪN bị chặn nhau. Có lãng PHÍ không?
::::

::::explain{#khoa-doc-nhieu-nguoi}
`BangKhoaSE` tách HAI loại khoá — khoá ĐỌC (nhiều giao dịch giữ
CÙNG lúc được) và khoá GHI (độc quyền thật SỰ, chặn cả người đọc
lẫn người ghi KHÁC):

```python title=readonly
class BangKhoaSE:
    def __init__(self):
        self._doc = {}
        self._ghi = {}

    def xin_khoa_doc(self, khoa, id_giao_dich):
        chu_ghi = self._ghi.get(khoa)
        if chu_ghi is not None and chu_ghi != id_giao_dich:
            return False
        self._doc.setdefault(khoa, set()).add(id_giao_dich)
        return True

    def xin_khoa_ghi(self, khoa, id_giao_dich):
        nguoi_doc = self._doc.get(khoa, set())
        if nguoi_doc - {id_giao_dich}:
            return False
        chu_ghi = self._ghi.get(khoa)
        if chu_ghi is not None and chu_ghi != id_giao_dich:
            return False
        self._ghi[khoa] = id_giao_dich
        return True


b = BangKhoaSE()
print(b.xin_khoa_doc("acc1", "T1"), b.xin_khoa_doc("acc1", "T2"))
print(b.xin_khoa_ghi("acc1", "T1"))
```

```text title=readonly
True True
False
```

`T1` VÀ `T2` cùng xin khoá ĐỌC `"acc1"` — CẢ hai thành công, vì
`_ghi` chưa có AI. Sau đó `T1` thử xin khoá GHI CÙNG khoá — thất
bại: `nguoi_doc = {'T1', 'T2'}`, trừ ĐI `{'T1'}` (chính nó) VẪN còn
`{'T2'}` — CÓ người khác đang đọc, `xin_khoa_ghi` từ chối.
::::

::::example{#nhieu-nguoi-doc-cung-luc}
Một người ĐỌC thứ ba tham gia — vẫn thành CÔNG, vì đọc không giới
hạn số lượng:

```python title=readonly
print(b.xin_khoa_doc("acc1", "T3"))
```

```text title=readonly
True
```

`"acc1"` giờ có BA giao dịch cùng giữ khoá ĐỌC (`T1`, `T2`, `T3`)
— KHÔNG có giới hạn nào cho SỐ người đọc, vì `_doc` LÀ một `set`,
CHỈ `_ghi` mới cần ĐỘC quyền thật sự.
::::

::::predict{#doan-nguoi-thu-tu-xin-ghi commitOnce}
Một giao dịch THỨ tư (`T4`, chưa hề đọc gì) thử xin khoá GHI TRÊN
`"acc1"` — nơi `T1`, `T2`, `T3` đang giữ khoá ĐỌC:

```python
print(b.xin_khoa_ghi("acc1", "T4"))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì `T4` chưa hề xin khoá đọc TRƯỚC đó, nó KHÔNG nằm trong
tập người đọc, nên KHÔNG có gì cản nó xin khoá GHI
::why
Gần đúng ở việc bạn để Ý ĐÚNG `T4` không CÓ mặt trong `_doc["acc1"]`
— một quan SÁT chính xác về trạng thái của `T4`.

Chỗ lệch: `nguoi_doc - {id_giao_dich}` trừ ĐI CHÍNH `id_giao_dich`
(ở đây LÀ `"T4"`) khỏi tập người đọc — nhưng `T1`, `T2`, `T3` (BA
giao dịch KHÁC hoàn toàn) VẪN còn nguyên trong tập SAU khi trừ, nên
biểu thức VẪN không rỗng — `xin_khoa_ghi` từ chối, bất kể AI đang
gọi.
::
:::

:::opt
Máy báo lỗi — vì `T4` xin khoá GHI mà chưa hề CÓ khoá đọc nào TRÊN
`"acc1"` trước đó LÀ thao tác không hợp LỆ
::why
Gần đúng ở việc bạn nghĩ TỚI một trình tự "phải đọc TRƯỚC khi ghi"
hợp lý cho MỘT số quy trình khác.

Chỗ lệch: `xin_khoa_ghi` KHÔNG hề yêu cầu người GỌI đã từng đọc
trước — nó chỉ kiểm TRA xem CÓ ai khác (đọc hay ghi) đang giữ
KHÔNG. Không có `raise` nào, chỉ trả VỀ `True`/`False`.
::
:::
::::

::::code{#viet_xin_khoa_doc}
Hoàn thiện `xin_khoa_doc` — sau khi đã CHẮC không ai đang GHI, thêm
giao dịch VÀO tập người đọc.

```python title=starter
class BangKhoaSE:
    def __init__(self):
        self._doc = {}
        self._ghi = {}

    def xin_khoa_doc(self, khoa, id_giao_dich):
        chu_ghi = self._ghi.get(khoa)
        if chu_ghi is not None and chu_ghi != id_giao_dich:
            return False
        ___
        return True

    def xin_khoa_ghi(self, khoa, id_giao_dich):
        nguoi_doc = self._doc.get(khoa, set())
        if nguoi_doc - {id_giao_dich}:
            return False
        chu_ghi = self._ghi.get(khoa)
        if chu_ghi is not None and chu_ghi != id_giao_dich:
            return False
        self._ghi[khoa] = id_giao_dich
        return True


b = BangKhoaSE()
print(b.xin_khoa_doc("acc1", "T1"), b.xin_khoa_doc("acc1", "T2"))
print(b.xin_khoa_ghi("acc1", "T1"))
```

```python title=solution
class BangKhoaSE:
    def __init__(self):
        self._doc = {}
        self._ghi = {}

    def xin_khoa_doc(self, khoa, id_giao_dich):
        chu_ghi = self._ghi.get(khoa)
        if chu_ghi is not None and chu_ghi != id_giao_dich:
            return False
        self._doc.setdefault(khoa, set()).add(id_giao_dich)
        return True

    def xin_khoa_ghi(self, khoa, id_giao_dich):
        nguoi_doc = self._doc.get(khoa, set())
        if nguoi_doc - {id_giao_dich}:
            return False
        chu_ghi = self._ghi.get(khoa)
        if chu_ghi is not None and chu_ghi != id_giao_dich:
            return False
        self._ghi[khoa] = id_giao_dich
        return True


b = BangKhoaSE()
print(b.xin_khoa_doc("acc1", "T1"), b.xin_khoa_doc("acc1", "T2"))
print(b.xin_khoa_ghi("acc1", "T1"))
```

```python title=test
b = BangKhoaSE()
assert b.xin_khoa_doc("acc1", "T1") == True, "khoa doc dau tien phai thanh cong"
assert b.xin_khoa_doc("acc1", "T2") == True, "khoa doc thu hai cung phai thanh cong -- nhieu nguoi doc"
assert b.xin_khoa_ghi("acc1", "T1") == False, "T2 van dang doc, T1 khong duoc nang cap len khoa ghi"
assert b.xin_khoa_doc("acc1", "T3") == True, "nguoi doc thu ba van thanh cong"
assert b.xin_khoa_ghi("acc1", "T4") == False, "T4 chua tung doc, van bi chan boi T1/T2/T3 dang doc"

b2 = BangKhoaSE()
assert b2.xin_khoa_ghi("acc2", "T1") == True, "khong ai doc, T1 xin khoa ghi thanh cong"
assert b2.xin_khoa_doc("acc2", "T2") == False, "T1 dang giu khoa ghi, T2 khong doc duoc"
```

:::hints
- kind: attention
  body: "Sau khi kiem tra khong ai dang ghi, them id_giao_dich vao set nguoi doc cua khoa do -- dung self._doc.setdefault(khoa, set()).add(id_giao_dich)."
- kind: strategy
  body: "self._doc.setdefault(khoa, set()).add(id_giao_dich)"
- kind: one-line
  body: "self._doc.setdefault(khoa, set()).add(id_giao_dich)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai them id_giao_dich vao tap nguoi doc cua khoa trong self._doc
  requireAst:
  - kind: uses-name, target: khoa, min: 5
  - kind: uses-name, target: id_giao_dich, min: 5
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True True\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đọc nhiều được, ghi phải ĐỘC quyền — đúng như dữ liệu THẬT cần.
Nhưng xin khoá LÚC nào trong một giao dịch, VÀ trả khoá lúc NÀO —
có quy TẮC nào không?
::::

::::reflect{#nghi-lai}
`BangKhoaSE` phân biệt ĐỌC (chia sẻ được) và GHI (độc quyền tuyệt
đối) — `xin_khoa_ghi` CHỈ thành công khi KHÔNG ai khác đang đụng
vào khoá đó, dù LÀ đọc hay ghi. Nhưng CẢ hai bài trước lẫn bài NÀY
đều chưa nói TỚI: một giao dịch nên xin khoá LÚC nào, và trả khoá
lúc NÀO? Nếu xin THÊM khoá SAU khi đã trả MỘT khoá khác, có VẤN đề
gì không? Đây LÀ câu hỏi mà 2PL (hai pha khoá) trả LỜI.
::::

::::checkpoint{mastery=0.8}
::::
