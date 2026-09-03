---
id: co-so-du-lieu.bon-chu-cai-acid.boss-bon-chu-cai-acid
title: "BOSS — Bốn chữ cái ACID"
summary: "Ghép TRỌN q05: hai giao dịch 2PL kẹt nhau thật sự (deadlock xác nhận qua co_deadlock_hai_ben), huy_giao_dich GIẢI phóng toàn bộ khoá của MỘT giao dịch bị chọn để phá vòng — giao dịch còn lại xin ĐƯỢC khoá nó đang chờ NGAY sau đó, còn giao dịch bị huỷ vĩnh viễn không xin được gì nữa (đã ở pha co lại)."
locale: vi
track: co-so-du-lieu
module: bon-chu-cai-acid
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.deadlock-detection]
concepts: [db.boss-q05]
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
Khoá, 2PL, phát hiện deadlock — ba mảnh. Ghép LẠI: phát hiện XONG
rồi làm gì để hai giao dịch kẹt nhau CÓ thể tiếp tục?
::::

::::explain{#pha-deadlock}
Khi phát hiện deadlock, MỘT giao dịch phải bị "huỷ" (abort) — giải
phóng TOÀN bộ khoá nó đang giữ — để giao DỊCH còn lại có thể tiếp
tục:

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


def co_deadlock_hai_ben(do_thi_cho, a, b):
    a_cho_b = b in do_thi_cho.get(a, set())
    b_cho_a = a in do_thi_cho.get(b, set())
    return a_cho_b and b_cho_a


def huy_giao_dich(gd):
    for khoa in list(gd.khoa_dang_giu):
        gd.tra(khoa)


bang = BangKhoa()
gd1 = GiaoDichHaiPha(bang, "T1")
gd2 = GiaoDichHaiPha(bang, "T2")

gd1.xin("A")
gd2.xin("B")
print(gd1.xin("B"), gd2.xin("A"))

do_thi_cho = {"T1": {"T2"}, "T2": {"T1"}}
print(co_deadlock_hai_ben(do_thi_cho, "T1", "T2"))

huy_giao_dich(gd2)
print(gd2.khoa_dang_giu)
print(gd1.xin("B"))
```

```text title=readonly
False False
True
[]
True
```

`T1` giữ `"A"`, `T2` giữ `"B"` — CẢ hai thử xin thêm khoá của bên
KIA, cả hai thất BẠI (`False False`). Đồ thị chờ xác NHẬN deadlock
(`True`). `huy_giao_dich(gd2)` giải phóng TOÀN bộ khoá `T2` đang
giữ (`khoa_dang_giu` rỗng SAU đó) — `"B"` giờ RẢNH, `T1` xin ĐƯỢC
NGAY (`True`).
::::

::::example{#giao_dich_bi_huy_khong_lam_gi_them}
Giao dịch BỊ huỷ (`gd2`) không thể tiếp tục — dù có xin MỘT khoá
HOÀN toàn mới, nó vẫn bị TỪ chối:

```python title=readonly
print(gd2.xin("C"))
```

```text title=readonly
False
```

`huy_giao_dich` gọi `gd2.tra(...)` cho MỌI khoá `T2` đang giữ —
`tra` LUÔN đặt `self.pha = 'co_lai'`. Sau khi bị huỷ, `gd2` VĨNH
viễn ở pha co LẠI — `gd2.xin("C")` bị chặn NGAY từ điều kiện đầu
tiên của `xin`, dù `"C"` chưa AI đụng tới. Giao dịch bị huỷ PHẢI
bắt đầu lại HOÀN toàn, không thể "tiếp tục" từ giữa chừng.
::::

::::predict{#doan_T1_xin_them_sau_khi_thang commitOnce}
Sau khi `T1` xin được `"B"` (giao dịch KIA đã bị huỷ), `T1` thử xin
THÊM một khoá thứ ba, `"C"` — hoàn toàn RẢNH, chưa ai đụng TỚI:

```python
print(gd1.xin("C"))
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — vì hệ THỐNG vừa trải qua một deadlock, MỌI giao dịch
liên quan (kể cả `T1`, bên "THẮNG") đều bị hạn chế xin THÊM khoá
::why
Gần đúng ở việc bạn thận trọng VỀ trạng thái hệ thống SAU một sự
kiện bất thường như deadlock — MỘT lo lắng hợp lý.

Chỗ lệch: `gd1` (T1) chưa hề gọi `.tra(...)` LẦN nào — `pha` của nó
VẪN LÀ `'tang_truong'`, hoàn toàn KHÔNG bị ảnh hưởng bởi việc `gd2`
bị huỷ. `T1` được PHÉP tiếp tục xin thêm khoá bình THƯỜNG, `"C"`
đang rảnh nên thành CÔNG.
::
:::

:::opt
Máy báo lỗi — vì `"C"` chưa từng xuất hiện trong `do_thi_cho`,
`xin` không biết XỬ lý khoá "lạ" thế nào
::why
Gần đúng ở việc bạn nghĩ TỚI `do_thi_cho` như một danh SÁCH khoá
"đã đăng ký" hợp LỆ — một mô hình hợp lý cho một số hệ thống
KHÁC.

Chỗ lệch: `do_thi_cho` CHỈ LÀ một biến Python bình thường dùng để
minh HOẠ, hoàn toàn TÁCH biệt khỏi `GiaoDichHaiPha`/`BangKhoa` —
`xin` không hề đọc `do_thi_cho`. `"C"` LÀ một chuỗi bất KỲ, hệ
thống khoá xử lý nó y HỆT `"A"` hay `"B"`.
::
:::
::::

::::code{#viet_huy_giao_dich}
Hoàn thiện `huy_giao_dich(gd)` — trả LẠI từng khoá giao dịch đang
giữ, một cách AN toàn (duyệt trên BẢN sao của danh sách).

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


def co_deadlock_hai_ben(do_thi_cho, a, b):
    a_cho_b = b in do_thi_cho.get(a, set())
    b_cho_a = a in do_thi_cho.get(b, set())
    return a_cho_b and b_cho_a


def huy_giao_dich(gd):
    for khoa in list(gd.khoa_dang_giu):
        ___


bang = BangKhoa()
gd1 = GiaoDichHaiPha(bang, "T1")
gd2 = GiaoDichHaiPha(bang, "T2")
gd1.xin("A")
gd2.xin("B")
gd1.xin("B")
gd2.xin("A")
huy_giao_dich(gd2)
print(gd2.khoa_dang_giu, gd1.xin("B"))
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


def co_deadlock_hai_ben(do_thi_cho, a, b):
    a_cho_b = b in do_thi_cho.get(a, set())
    b_cho_a = a in do_thi_cho.get(b, set())
    return a_cho_b and b_cho_a


def huy_giao_dich(gd):
    for khoa in list(gd.khoa_dang_giu):
        gd.tra(khoa)


bang = BangKhoa()
gd1 = GiaoDichHaiPha(bang, "T1")
gd2 = GiaoDichHaiPha(bang, "T2")
gd1.xin("A")
gd2.xin("B")
gd1.xin("B")
gd2.xin("A")
huy_giao_dich(gd2)
print(gd2.khoa_dang_giu, gd1.xin("B"))
```

```python title=test
bang = BangKhoa()
gd1 = GiaoDichHaiPha(bang, "T1")
gd2 = GiaoDichHaiPha(bang, "T2")
assert gd1.xin("A") == True, "T1 xin A thanh cong"
assert gd2.xin("B") == True, "T2 xin B thanh cong"
assert gd1.xin("B") == False, "T1 bi chan boi T2"
assert gd2.xin("A") == False, "T2 bi chan boi T1 -- deadlock"

do_thi_cho = {"T1": {"T2"}, "T2": {"T1"}}
assert co_deadlock_hai_ben(do_thi_cho, "T1", "T2") == True, "phai xac nhan deadlock that su"

huy_giao_dich(gd2)
assert gd2.khoa_dang_giu == [], "sau khi huy, T2 khong con giu khoa nao"
assert gd1.xin("B") == True, "B da ranh sau khi T2 bi huy, T1 phai xin duoc"
assert gd2.xin("C") == False, "T2 da bi huy (o pha co_lai vinh vien), khong the xin them gi"
assert gd1.xin("C") == True, "T1 chua bi anh huong, van xin them binh thuong"
```

:::hints
- kind: attention
  body: "Trong than vong for, goi gd.tra(khoa) de tra tung khoa mot -- mot dong."
- kind: strategy
  body: "gd.tra(khoa)"
- kind: one-line
  body: "gd.tra(khoa)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi gd.tra(khoa) cho tung khoa trong vong lap
  requireAst:
  - kind: uses-name, target: gd, min: 2
  - kind: uses-name, target: khoa, min: 9
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[\] True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khoá, 2PL, phát hiện deadlock, VÀ huỷ một giao dịch để phá vòng —
một hệ thống điều PHỐI đồng thời hoàn chỉnh. q05 hoàn TẤT.
::::

::::reflect{#nghi-lai}
q05 xây dựng đủ BỐN mảnh để nhiều giao dịch cùng tồn TẠI an toàn:
bảng khoá độc quyền (bài 2), chờ CÓ giới hạn thay vì mãi mãi (bài
3), khoá đọc/ghi tách biệt để KHÔNG chặn nhau lãng phí (bài 4), 2PL
đảm bảo mỗi giao dịch chỉ đi MỘT chiều tăng trưởng rồi co lại (bài
5), VÀ khi hai giao dịch vẫn kẹt nhau dù tuân thủ mọi luật — phát
hiện qua đồ thị chờ (bài 6-7) rồi huỷ MỘT bên để bên còn lại tiếp
tục (bài NÀY). Nhưng khoá LÀ một giải pháp "phòng ngừa" — CHẶN
trước khi xung đột xảy RA, đổi lấy việc một số giao dịch phải CHỜ
hoặc bị huỷ. q06 "Nhiều dòng thời gian" đi theo hướng NGƯỢC lại:
thay vì khoá chặn TRƯỚC, mỗi giao dịch nhìn thấy một PHIÊN bản dữ
liệu riêng (snapshot) — MVCC hoạt động ra sao?
::::

::::checkpoint{mastery=0.85}
::::
