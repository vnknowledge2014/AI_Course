---
id: co-so-du-lieu.bon-chu-cai-acid.hai-pha-khoa-2pl
title: Hai pha khoá — 2PL
summary: "GiaoDichHaiPha theo dõi pha của MỘT giao dịch — pha TĂNG TRƯỞNG (chỉ xin khoá) rồi pha CO LẠI (chỉ trả khoá), KHÔNG xen kẽ. Trả một khoá LÀ chuyển hẳn sang pha co lại — xin thêm khoá SAU đó bị từ chối, dù khoá đang hoàn toàn rảnh."
locale: vi
track: co-so-du-lieu
module: bon-chu-cai-acid
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [db.two-phase-locking]
requires: [db.shared-exclusive-lock]
concepts: [db.two-phase-locking]
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
Xin khoá lúc NÀO, trả khoá lúc NÀO trong một giao dịch — có QUY tắc
gì không, hay tuỳ Ý muốn làm sao CŨNG được?
::::

::::explain{#hai-pha}
`GiaoDichHaiPha` theo dõi PHA của một giao dịch — pha TĂNG trưởng
(chỉ xin khoá) RỒI pha CO lại (chỉ trả khoá), KHÔNG xen kẽ. Trả một
khoá LÀ chuyển hẳn sang pha co LẠI:

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


class GiaoDichHaiPha:
    def __init__(self, bang, id_giao_dich):
        self.bang = bang
        self.id_giao_dich = id_giao_dich
        self.pha = 'tang_truong'
        self.khoa_dang_giu = []

    def xin(self, khoa):
        if self.pha == 'co_lai':
            return False
        thanh_cong = self.bang.xin_khoa(khoa, self.id_giao_dich)
        if thanh_cong:
            self.khoa_dang_giu.append(khoa)
        return thanh_cong

    def tra(self, khoa):
        self.pha = 'co_lai'
        self.bang.tra_khoa(khoa, self.id_giao_dich)
        self.khoa_dang_giu.remove(khoa)


bang = BangKhoa()
gd = GiaoDichHaiPha(bang, "T1")
print(gd.xin("acc1"))
print(gd.xin("acc2"))
gd.tra("acc1")
print(gd.xin("acc3"))
```

```text title=readonly
True
True
False
```

`gd` xin `"acc1"` VÀ `"acc2"` — cả hai thành CÔNG, đang Ở pha TĂNG
trưởng. `gd.tra("acc1")` chuyển `pha` sang `'co_lai'`. Sau đó
`gd.xin("acc3")` — dù `"acc3"` HOÀN toàn rảnh — bị TỪ chối NGAY, vì
`pha == 'co_lai'`: đã BẮT đầu trả khoá thì KHÔNG được xin thêm.
::::

::::example{#tra-nhieu-khoa-lien-tiep}
Sau khi ĐàỞ pha co lại, mọi lần TRẢ tiếp theo VẪN hoạt động bình
thường — chỉ XIN mới bị chặn:

```python title=readonly
gd.tra("acc2")
print(gd.khoa_dang_giu, gd.pha)
```

```text title=readonly
[] co_lai
```

`gd.tra("acc2")` chạy BÌNH thường — `khoa_dang_giu` giờ RỖNG. `pha`
VẪN LÀ `'co_lai'` (không có đường quay LẠI `'tang_truong'`) — 2PL
CHỈ cho phép đi MỘT chiều: tăng trưởng RỒI co lại, không bao GIỜ
ngược lại.
::::

::::predict{#doan-giao-dich-moi-khong-bi-anh-huong commitOnce}
`gd` (Ở trên) ĐÃ chuyển hẳn sang pha co LẠI. Một giao dịch HOÀN
toàn mới, `gd2`, xin khoá `"acc1"` (giờ đã RẢNH, vì `gd` đã trả):

```python
gd2 = GiaoDichHaiPha(bang, "T2")
print(gd2.xin("acc1"))
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — vì HỆ thống đã Ở pha co LẠI (do `gd` gây ra trước đó),
MỌI giao dịch mới CŨNG bị ảnh hưởng CHO tới khi có MỘT reset
::why
Gần đúng ở việc bạn nghĩ TỚI "pha" như một trạng THÁI toàn cục của
CẢ hệ thống — một mô hình hợp lý cho một SỐ cơ chế đồng bộ khác.

Chỗ lệch: `self.pha` LÀ thuộc tính CỦA riêng từng đối tượng
`GiaoDichHaiPha` — `gd2` LÀ một đối tượng HOÀN toàn mới, `pha` của
NÓ khởi tạo lại từ `'tang_truong'`, không hề bị ảnh HƯỞNG bởi `gd`.
Mỗi giao dịch có PHA của riêng mình.
::
:::

:::opt
Máy báo lỗi — vì `"acc1"` VỪA được `gd` trả LẠI, xin ngay LẬP tức
LÀ thao tác không hợp lệ
::why
Gần đúng ở việc bạn nghĩ TỚI một khoảng CÁCH thời gian tối thiểu
hợp lý giữa trả VÀ xin lại — một trực giác VỀ độ trễ hệ thống thật.

Chỗ lệch: `BangKhoa.xin_khoa` chỉ kiểm TRA `_giu_boi` NGAY tại thời
điểm gọi — `"acc1"` đã bị XOÁ khỏi `_giu_boi` khi `gd.tra("acc1")`
chạy, nên `gd2.xin("acc1")` thấy khoá RẢNH ngay, không hề `raise`.
::
:::
::::

::::code{#viet_giao_dich_hai_pha}
Hoàn thiện `xin` — nếu giao dịch ĐÃ ở pha co lại, từ chối NGAY
trước khi thử xin khoá thật.

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


class GiaoDichHaiPha:
    def __init__(self, bang, id_giao_dich):
        self.bang = bang
        self.id_giao_dich = id_giao_dich
        self.pha = 'tang_truong'
        self.khoa_dang_giu = []

    def xin(self, khoa):
        ___
        thanh_cong = self.bang.xin_khoa(khoa, self.id_giao_dich)
        if thanh_cong:
            self.khoa_dang_giu.append(khoa)
        return thanh_cong

    def tra(self, khoa):
        self.pha = 'co_lai'
        self.bang.tra_khoa(khoa, self.id_giao_dich)
        self.khoa_dang_giu.remove(khoa)


bang = BangKhoa()
gd = GiaoDichHaiPha(bang, "T1")
gd.xin("acc1")
gd.tra("acc1")
print(gd.xin("acc2"))
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


class GiaoDichHaiPha:
    def __init__(self, bang, id_giao_dich):
        self.bang = bang
        self.id_giao_dich = id_giao_dich
        self.pha = 'tang_truong'
        self.khoa_dang_giu = []

    def xin(self, khoa):
        if self.pha == 'co_lai':
            return False
        thanh_cong = self.bang.xin_khoa(khoa, self.id_giao_dich)
        if thanh_cong:
            self.khoa_dang_giu.append(khoa)
        return thanh_cong

    def tra(self, khoa):
        self.pha = 'co_lai'
        self.bang.tra_khoa(khoa, self.id_giao_dich)
        self.khoa_dang_giu.remove(khoa)


bang = BangKhoa()
gd = GiaoDichHaiPha(bang, "T1")
gd.xin("acc1")
gd.tra("acc1")
print(gd.xin("acc2"))
```

```python title=test
bang = BangKhoa()
gd = GiaoDichHaiPha(bang, "T1")
assert gd.xin("acc1") == True, "pha tang truong, xin khoa dau tien phai thanh cong"
assert gd.xin("acc2") == True, "van con o pha tang truong, xin them phai thanh cong"
gd.tra("acc1")
assert gd.pha == 'co_lai', "sau khi tra mot khoa, phai chuyen han sang pha co lai"
assert gd.xin("acc3") == False, "da o pha co lai, xin them khoa moi phai bi tu choi"

gd2 = GiaoDichHaiPha(bang, "T2")
assert gd2.xin("acc1") == True, "gd2 la giao dich moi, pha rieng cua no van la tang_truong"
```

:::hints
- kind: attention
  body: "Neu self.pha == 'co_lai', return False NGAY, truoc khi goi self.bang.xin_khoa."
- kind: strategy
  body: "if self.pha == 'co_lai': return False"
- kind: one-line
  body: "if self.pha == 'co_lai': return False"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tu choi ngay neu self.pha da la 'co_lai', truoc khi thu xin khoa that
  requireAst:
  - kind: uses-name, target: self, min: 17
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^False\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai pha, không xen kẽ — 2PL đảm bảo một giao dịch KHÔNG xin thêm
khoá sau khi đã BẮT đầu trả. Nhưng nếu HAI giao dịch cùng tuân THỦ
2PL mà vẫn KẸT nhau thì sao?
::::

::::reflect{#nghi-lai}
2PL chia MỘT giao dịch thành đúng hai pha — tăng TRƯỞNG (chỉ xin)
rồi co LẠI (chỉ trả), không bao GIỜ ngược lại. Quy tắc NÀY đủ để
đảm bảo TÍNH đúng đắn của việc xếp LỊCH nhiều giao dịch (một chủ đề
lý thuyết SÂU hơn, ngoài phạm vi Ở đây) — nhưng nó KHÔNG hề ngăn
được một tình huống KHÁC: hai giao dịch, cả hai ĐỀU tuân thủ 2PL
hoàn hảo, mỗi bên giữ MỘT khoá và CHỜ khoá của bên KIA. Cả hai kẹt
MÃI mãi. Đó LÀ gì?
::::

::::checkpoint{mastery=0.8}
::::
