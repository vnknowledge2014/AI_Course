---
id: lap-trinh-ham.bat-bien-thuan-khiet.boss-bat-bien-thuan-khiet
title: "BOSS — Khép track Bất biến & thuần khiết"
summary: "Một chương trình nhỏ dùng đủ: frozen=True + replace() (cụm 1), hàm thuần + dependency injection (cụm 2), công cụ bất biến đúng chỗ (cụm 3), state machine bất biến dùng match (cụm 4). Không dạy khái niệm mới — chỉ đòi ghép lại."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.gate-boss]
requires: [fp.gate-review]
concepts: [fp.gate-boss]
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
Hai mươi ba bài để tới đây. Một Ý gốc xuyên suốt tất cả: đừng sửa, hãy
tạo mới. Giờ ghép hết lại, trong một chương trình duy nhất.
::::

::::explain{#bon-manh-ghep}
Suốt track này, mọi bài đều là hệ quả của đúng MỘT luật gốc — **không
sửa dữ liệu tại chỗ, luôn tạo mới; một hàm không side-effect luôn cho
cùng input ra cùng output**. Bốn mảnh ghép, tất cả cùng xoay quanh luật
đó:

1. **`frozen=True` + `replace()`** (cụm 1) — một `@dataclass` khai
   `frozen=True` chặn THẬT SỰ việc sửa field, ném
   `dataclasses.FrozenInstanceError`; muốn "sửa", gọi `replace(obj,
   field=gia_tri_moi)` để dựng một bản MỚI, bản cũ đứng yên.
2. **Hàm thuần + tiêm phụ thuộc** (cụm 2) — một hàm THUẦN luôn cho
   cùng kết quả với cùng đối số, không `print`, không đọc biến toàn
   cục, không sửa dữ liệu tại chỗ; nguồn không tất định (tỷ giá, giờ
   hệ thống, ngẫu nhiên) được TRUYỀN VÀO qua tham số, không tự hàm đi
   lấy.
3. **Công cụ bất biến đúng chỗ** (cụm 3) — `tuple` cho một dãy có thứ
   tự, `frozenset` cho một tập hợp, `MappingProxyType` cho một bảng
   tra chỉ đọc — mỗi kiểu dữ liệu có công cụ bất biến RIÊNG của nó,
   không dùng `list`/`dict`/`set` thường khi không cần sửa.
4. **State machine bất biến dùng `match`** (cụm 4) — mỗi trạng thái
   là một `@dataclass(frozen=True)` riêng; một hàm chuyển trạng thái
   dùng `match` khớp đúng kiểu nguồn, `raise ValueError` cho mọi kiểu
   khác; trạng thái CŨ không đổi sau khi trạng thái MỚI được dựng.

Bốn mảnh đó là toàn bộ nguyên liệu để viết một chương trình Python nhỏ
mà KHÔNG một dòng nào sửa dữ liệu tại chỗ. Bài này ghép chúng vào một
chương trình duy nhất: một giỏ hàng nhỏ, tính tiền, rồi thanh toán.
::::

::::example{#chuong-trinh-ghep-du-bon-manh}
Một giỏ hàng, một bảng giá chỉ đọc, một mã giảm giá, rồi thanh toán:

```python title=readonly
from dataclasses import dataclass, replace
from types import MappingProxyType

BANG_GIA = MappingProxyType({"cà phê": 25000, "trà đá": 10000})
MA_GIAM_GIA_HOP_LE = frozenset({"SALE10", "SALE20"})
TY_LE_GIAM = MappingProxyType({"SALE10": 10, "SALE20": 20})

@dataclass(frozen=True)
class GioHang:
    mon: tuple
    ma_giam_gia: str = ""

@dataclass(frozen=True)
class DonDaThanhToan:
    mon: tuple
    tong_tien: int

def them_mon(gio, ten_mon):
    return replace(gio, mon=gio.mon + (ten_mon,))

def ap_dung_giam_gia(ma, bang_ty_le=TY_LE_GIAM):
    if ma not in MA_GIAM_GIA_HOP_LE:
        return 0
    return bang_ty_le.get(ma, 0)

def tinh_tong(gio, bang_gia, bang_ty_le=TY_LE_GIAM):
    tong_goc = sum(bang_gia[m] for m in gio.mon)
    ty_le = ap_dung_giam_gia(gio.ma_giam_gia, bang_ty_le)
    return tong_goc - tong_goc * ty_le // 100

def thanh_toan(gio, bang_gia):
    match gio:
        case GioHang(mon=m):
            return DonDaThanhToan(mon=m, tong_tien=tinh_tong(gio, bang_gia))
        case _:
            raise ValueError("chỉ thanh toán được từ GioHang")

gio = them_mon(GioHang(mon=()), "cà phê")
don = thanh_toan(gio, BANG_GIA)

print(f"gio: {gio}")
print(f"don: {don}")
```

```text title=readonly
gio: GioHang(mon=('cà phê',), ma_giam_gia='')
don: DonDaThanhToan(mon=('cà phê',), tong_tien=25000)
```

Đọc ra bốn mảnh: `GioHang`/`DonDaThanhToan` khai `frozen=True` (mảnh
1) — `them_mon()` "thêm món" bằng `replace()`, dựng một `GioHang` MỚI,
`gio` cũ (nếu còn tên nào giữ nó) không hề đổi. `tinh_tong()` là hàm
THUẦN (mảnh 2) — `bang_gia` và `bang_ty_le` đều là THAM SỐ được TIÊM
vào, không phải biến toàn cục hàm tự đi đọc; gọi `tinh_tong(gio,
BANG_GIA)` hai lần với cùng đối số luôn ra cùng con số. `BANG_GIA`,
`MA_GIAM_GIA_HOP_LE`, `TY_LE_GIAM` mỗi cái dùng đúng công cụ (mảnh 3):
bảng giá và bảng tỷ lệ là `MappingProxyType` (chỉ đọc), mã hợp lệ là
`frozenset` (tập hợp không sửa), `mon` bên trong `GioHang` là `tuple`.
`thanh_toan()` là một hàm chuyển trạng thái dùng `match` (mảnh 4) — chỉ
nhận `GioHang`, trả về `DonDaThanhToan` MỚI, `gio` gốc còn nguyên sau
khi gọi.

Nếu gọi `thanh_toan()` một lần nữa, lần này trên CHÍNH `don` vừa tạo
ra:

```python title=readonly
thanh_toan(don, BANG_GIA)
```

```text title=readonly
Traceback (most recent call last):
  ...
ValueError: chỉ thanh toán được từ GioHang
```

Đúng luật mảnh 4 — `don` là `DonDaThanhToan`, không khớp `case
GioHang(...)`, rơi vào `case _`, và bị từ chối rõ ràng.
::::

::::predict{#doan-giam-gia commitOnce}
Byte thêm một mã giảm giá vào giỏ TRƯỚC KHI thanh toán — dùng
`replace()`, không sửa `gio` gốc:

```python
gio = them_mon(GioHang(mon=()), "cà phê")
gio = them_mon(gio, "trà đá")

gio_sale = replace(gio, ma_giam_gia="SALE10")
don = thanh_toan(gio_sale, BANG_GIA)

print(gio.ma_giam_gia)
print(don.tong_tien)
```

Giá `cà phê` là `25000`, `trà đá` là `10000`. `SALE10` giảm 10%.

**Trước khi chạy**, bạn đoán hai dòng cuối in ra gì?

:::opt{correct}
Dòng đầu in ra chuỗi rỗng (`''`) — `gio` không có mã giảm giá; dòng
sau in ra `31500` — tổng gốc `35000`, giảm 10%
:::

:::opt
Dòng đầu in ra `SALE10` — vì `replace()` sửa `gio` ngay tại chỗ, không
tạo bản mới; dòng sau in ra `31500`
::why
Gần đúng ở việc bạn tính đúng con số cuối, `31500` — đúng 10% của
`35000`.

Chỗ lệch: `replace(gio, ma_giam_gia="SALE10")` không sửa `gio` — nó
TRẢ VỀ một `GioHang` MỚI, gán cho tên `gio_sale`. `gio` không hề được
gán lại sau dòng đó, nên `gio.ma_giam_gia` vẫn là chuỗi rỗng như lúc
khởi tạo — đúng tinh thần "tạo mới, không sửa" xuyên suốt track này.
::
:::

:::opt
Dòng đầu in ra chuỗi rỗng; dòng sau in ra `35000` — mã giảm giá không
có tác dụng vì `thanh_toan()` không hề nhận `ma_giam_gia` làm tham số
riêng
::why
Gần đúng ở việc bạn để ý đúng: `thanh_toan(gio, bang_gia)` quả thật
không có tham số riêng tên `ma_giam_gia`.

Chỗ lệch: `thanh_toan()` gọi `tinh_tong(gio, bang_gia)`, và
`tinh_tong()` tự ĐỌC `gio.ma_giam_gia` (field của chính `gio` được
truyền vào) rồi gọi `ap_dung_giam_gia()` để tính tỷ lệ giảm. Mã giảm
giá đi theo `gio_sale` dưới dạng MỘT FIELD của nó, không cần một tham
số hàm riêng.
::
:::

:::opt
Dòng đầu in ra chuỗi rỗng; dòng sau in ra `35000` — `SALE10` không nằm
trong `MA_GIAM_GIA_HOP_LE` nên bị bỏ qua
::why
Gần đúng ở việc bạn nhớ đúng: `ap_dung_giam_gia()` có kiểm tra mã có
nằm trong `MA_GIAM_GIA_HOP_LE` hay không trước khi áp dụng — luật đó
có thật.

Chỗ lệch: `MA_GIAM_GIA_HOP_LE = frozenset({"SALE10", "SALE20"})` — mã
`"SALE10"` NẰM TRONG tập hợp đó, đúng chính tả, đúng chữ hoa. Kiểm tra
không loại nó ra; `TY_LE_GIAM["SALE10"]` trả về `10`, và tổng bị giảm
đúng 10%, ra `31500`.
::
:::
::::

::::code{#gio-hang-va-thanh-toan}
Chương trình đã đủ ba mảnh — `frozen=True`/`replace()`, hàm thuần với
tham số được tiêm, `tuple`/`frozenset`/`MappingProxyType`. Hoàn thành
mảnh cuối: `thanh_toan()` phải tính đúng số tiền bằng `tinh_tong()`,
không tính tay lại từ đầu.

```python title=starter
from dataclasses import dataclass, replace
from types import MappingProxyType

BANG_GIA = MappingProxyType({"cà phê": 25000, "trà đá": 10000, "bánh mì": 20000})
MA_GIAM_GIA_HOP_LE = frozenset({"SALE10", "SALE20"})
TY_LE_GIAM = MappingProxyType({"SALE10": 10, "SALE20": 20})

@dataclass(frozen=True)
class GioHang:
    mon: tuple
    ma_giam_gia: str = ""

@dataclass(frozen=True)
class DonDaThanhToan:
    mon: tuple
    tong_tien: int

def them_mon(gio, ten_mon):
    """Thêm một món — TẠO giỏ MỚI bằng replace(), không sửa gio."""
    return replace(gio, mon=gio.mon + (ten_mon,))

def ap_dung_giam_gia(ma, bang_ty_le=TY_LE_GIAM):
    """Hàm THUẦN — bảng tỷ lệ được TIÊM qua tham số (dependency
    injection), không tự tra một biến toàn cục nào khác."""
    if ma not in MA_GIAM_GIA_HOP_LE:
        return 0
    return bang_ty_le.get(ma, 0)

def tinh_tong(gio, bang_gia, bang_ty_le=TY_LE_GIAM):
    """Hàm THUẦN: cùng gio + bang_gia + bang_ty_le luôn ra cùng kết quả."""
    tong_goc = sum(bang_gia[m] for m in gio.mon)
    ty_le = ap_dung_giam_gia(gio.ma_giam_gia, bang_ty_le)
    return tong_goc - tong_goc * ty_le // 100

# Việc của bạn: chỉ THANH TOÁN được một GioHang (ném ValueError cho mọi
# thứ khác). Số tiền phải tính bằng tinh_tong(), không tính tay lại.
def thanh_toan(gio, bang_gia):
    match gio:
        case GioHang(mon=m):
            return DonDaThanhToan(mon=m, tong_tien=___)
        case _:
            raise ValueError("chỉ thanh toán được từ GioHang")

gio = them_mon(them_mon(GioHang(mon=()), "cà phê"), "trà đá")
don = thanh_toan(gio, BANG_GIA)

gio_sale = replace(gio, ma_giam_gia="SALE10")
don_sale = thanh_toan(gio_sale, BANG_GIA)

print(f"gio:      {gio}")
print(f"don:      {don}")
print(f"don_sale: {don_sale}")
```

```python title=solution
from dataclasses import dataclass, replace
from types import MappingProxyType

BANG_GIA = MappingProxyType({"cà phê": 25000, "trà đá": 10000, "bánh mì": 20000})
MA_GIAM_GIA_HOP_LE = frozenset({"SALE10", "SALE20"})
TY_LE_GIAM = MappingProxyType({"SALE10": 10, "SALE20": 20})

@dataclass(frozen=True)
class GioHang:
    mon: tuple
    ma_giam_gia: str = ""

@dataclass(frozen=True)
class DonDaThanhToan:
    mon: tuple
    tong_tien: int

def them_mon(gio, ten_mon):
    """Thêm một món — TẠO giỏ MỚI bằng replace(), không sửa gio."""
    return replace(gio, mon=gio.mon + (ten_mon,))

def ap_dung_giam_gia(ma, bang_ty_le=TY_LE_GIAM):
    """Hàm THUẦN — bảng tỷ lệ được TIÊM qua tham số (dependency
    injection), không tự tra một biến toàn cục nào khác."""
    if ma not in MA_GIAM_GIA_HOP_LE:
        return 0
    return bang_ty_le.get(ma, 0)

def tinh_tong(gio, bang_gia, bang_ty_le=TY_LE_GIAM):
    """Hàm THUẦN: cùng gio + bang_gia + bang_ty_le luôn ra cùng kết quả."""
    tong_goc = sum(bang_gia[m] for m in gio.mon)
    ty_le = ap_dung_giam_gia(gio.ma_giam_gia, bang_ty_le)
    return tong_goc - tong_goc * ty_le // 100

# Việc của bạn: chỉ THANH TOÁN được một GioHang (ném ValueError cho mọi
# thứ khác). Số tiền phải tính bằng tinh_tong(), không tính tay lại.
def thanh_toan(gio, bang_gia):
    match gio:
        case GioHang(mon=m):
            return DonDaThanhToan(mon=m, tong_tien=tinh_tong(gio, bang_gia))
        case _:
            raise ValueError("chỉ thanh toán được từ GioHang")

gio = them_mon(them_mon(GioHang(mon=()), "cà phê"), "trà đá")
don = thanh_toan(gio, BANG_GIA)

gio_sale = replace(gio, ma_giam_gia="SALE10")
don_sale = thanh_toan(gio_sale, BANG_GIA)

print(f"gio:      {gio}")
print(f"don:      {don}")
print(f"don_sale: {don_sale}")
```

```python title=test
assert isinstance(don, DonDaThanhToan), "thanh_toan(gio, ...) phải trả về một DonDaThanhToan"
assert don.mon == ("cà phê", "trà đá"), "mon phải giữ nguyên từ gio"
assert don.tong_tien == 35000, f"tong_tien phải là 35000 (25000+10000) — đang là {don.tong_tien}"
assert isinstance(gio, GioHang), "gio gốc không được đổi sau khi gọi thanh_toan"

assert don_sale.tong_tien == 31500, f"don_sale.tong_tien phải là 31500 (giảm 10% của 35000) — đang là {don_sale.tong_tien}"

da_nem = False
try:
    thanh_toan(don, BANG_GIA)
except ValueError:
    da_nem = True
assert da_nem, "thanh_toan() gọi trên một DonDaThanhToan phải ném ValueError"
```

:::hints
- kind: attention
  body: Chỗ trống là một biểu thức duy nhất — giá trị của tong_tien. Đừng tính tay lại bằng sum(...); hàm tinh_tong() đã có sẵn, đúng việc để làm chuyện đó.
- kind: strategy
  body: 'tinh_tong() đã nhận đúng gio và bang_gia — hai tham số cũng đang có sẵn trong thân thanh_toan(). Gọi tinh_tong(gio, bang_gia) và dùng thẳng kết quả đó.'
- kind: one-line
  body: 'Chỗ trống là: tinh_tong(gio, bang_gia)'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: thanh_toan() phải tính tong_tien bằng cách GỌI tinh_tong(gio, bang_gia) — không tính tay lại bằng sum(...) hay điền một con số cố định. Toàn chương trình không được sửa dữ liệu tại chỗ, và GioHang/DonDaThanhToan phải giữ nguyên frozen=True.
  requireAst:
  - kind: match-stmt, min: 1
  - kind: uses-call, target: tinh_tong, min: 1
  - kind: no-mutation
  - kind: frozen-dataclass, target: GioHang, min: 1
  - kind: frozen-dataclass, target: DonDaThanhToan, min: 1
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "don_sale: DonDaThanhToan(mon=('cà phê', 'trà đá'), tong_tien=31500)"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`frozen=True`, `replace()`, hàm thuần, tiêm phụ thuộc, `tuple`,
`frozenset`, `MappingProxyType`, `match` trên trạng thái — mọi mảnh
ghép cùng đứng trong một chương trình, không lệch một luật nào. Track
Bất biến & thuần khiết khép lại ở đây.
::::

::::reflect{#nghi-lai}
Một câu hỏi cuối, khép lại cả track này.

Hai mươi bốn bài, đi từ một hàm âm thầm sửa `list` của người gọi, tới
một chương trình mà KHÔNG một dòng nào sửa dữ liệu tại chỗ — mọi hàm
chỉ nhận, tính, và TRẢ VỀ giá trị mới.

Bạn mang theo đúng MỘT thói quen sang track sau: một hàm không sửa
input, luôn trả về giá trị mới. Track sau (HOF & composition) sẽ hỏi
một câu bạn chưa từng hỏi — nếu một hàm nhận MỘT HÀM KHÁC làm tham số,
hay TRẢ VỀ một hàm, thì ghép nhiều hàm lại với nhau, TỪNG BƯỚC MỘT,
trông như thế nào?

Thói quen "không sửa input" chính là điều kiện để câu hỏi đó có câu trả
lời an toàn — ghép hàm sau vào hàm trước mà không sợ hàm sau âm thầm
phá dữ liệu hàm trước đã tạo ra.
::::

::::checkpoint{mastery=0.85}
::::
