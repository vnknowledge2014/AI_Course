---
id: lap-trinh-ham.bat-bien-thuan-khiet.do-tong-hop-truoc-boss
title: "Đo tổng hợp trước BOSS"
summary: "Một state machine bất biến nhỏ — ít nhất hai trạng thái, một hàm chuyển hợp lệ dùng match, dữ liệu bên trong dùng tuple/frozen dataclass, không side-effect. Không khái niệm mới — đo bạn có GHÉP ĐÚNG hay không."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.gate-review]
requires: [fp.refactor-to-pure]
concepts: [fp.gate-review]
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
Bạn nói bạn TIN các mảnh đã học khớp được với nhau. Bài này không dạy
gì mới — nó đo thử, đừng tin suông.
::::

::::explain{#nhung-manh-can-ghep}
Không một Ý mới nào trong bài này. Ba mảnh đã học, giờ đứng chung trong
MỘT chương trình nhỏ:

1. **Trạng thái LÀ dữ liệu** (`fp.state-as-data`) — ít nhất hai
   `@dataclass(frozen=True)` khác nhau, mỗi cái đại diện một trạng
   thái, không phải một object tự đổi field.
2. **`match` chuyển trạng thái hợp lệ** (`fp.state-transition-match`)
   — một hàm khớp đúng kiểu trạng thái nguồn, `raise ValueError` cho
   mọi thứ khác.
3. **Không side-effect** (`fp.io-is-side-effect`,
   `fp.refactor-to-pure`) — hàm chuyển trạng thái chỉ TÍNH TOÁN, không
   `print`, không đọc biến toàn cục, không sửa dữ liệu tại chỗ.

Dữ liệu bên trong mỗi trạng thái cũng phải dùng đúng công cụ bất biến
— `tuple` cho một dãy có thứ tự không cần sửa, không phải `list`.
::::

::::example{#ghe-ngoi-may-bay}
Một ghế ngồi máy bay — TRỐNG, hoặc ĐÃ ĐẶT cho một hành khách kèm một
`tuple` yêu cầu đặc biệt:

```python title=readonly
from dataclasses import dataclass

@dataclass(frozen=True)
class GheTrong:
    so_ghe: str

@dataclass(frozen=True)
class GheDaDat:
    so_ghe: str
    ten_khach: str
    yeu_cau: tuple

def dat_ghe(ghe, ten_khach, yeu_cau):
    match ghe:
        case GheTrong(so_ghe=s):
            return GheDaDat(so_ghe=s, ten_khach=ten_khach, yeu_cau=yeu_cau)
        case _:
            raise ValueError("chỉ đặt được một GheTrong")

g = GheTrong("12A")
da_dat = dat_ghe(g, "Minh", ("ăn chay", "gần cửa sổ"))

print(f"g:      {g}")
print(f"da_dat: {da_dat}")
```

```text title=readonly
g:      GheTrong(so_ghe='12A')
da_dat: GheDaDat(so_ghe='12A', ten_khach='Minh', yeu_cau=('ăn chay', 'gần cửa sổ'))
```

`GheTrong` và `GheDaDat` là hai kiểu riêng biệt — không phải một
`Ghe` với field `da_dat: bool` đổi qua đổi lại. `dat_ghe()` chỉ khớp
`GheTrong` qua `match`; gọi nó trên một `GheDaDat` sẽ rơi vào `case _`
và ném lỗi. `yeu_cau` là `tuple` — một dãy yêu cầu đặc biệt không cần
sửa sau khi đặt ghế, đúng công cụ, không phải `list`. `g` vẫn nguyên
vẹn là `GheTrong` sau lời gọi — không hàm nào trong ví dụ này in ra
màn hình, đọc giờ hệ thống, hay sửa field của bất kỳ ai tại chỗ.
::::

::::predict{#doan-dat-hai-lan commitOnce}
Byte lỡ tay gọi `dat_ghe()` HAI LẦN trên cùng một `GheTrong` — lần đầu
đặt cho Minh, lần hai (do copy nhầm dòng) lại gọi trên chính KẾT QUẢ
của lần đầu.

**Trước khi chạy**, chuyện gì xảy ra ở dòng gọi thứ hai?

```python
g = GheTrong("12A")
lan1 = dat_ghe(g, "Minh", ("ăn chay",))
lan2 = dat_ghe(lan1, "Hoa", ())

print(lan2)
```

:::opt{correct}
Máy dừng lại, ném `ValueError` — `lan1` là `GheDaDat`, không phải
`GheTrong`, nên rơi vào `case _` của `dat_ghe`
:::

:::opt
In ra `GheDaDat(so_ghe='12A', ten_khach='Hoa', yeu_cau=())` — lần đặt
sau ĐÈ LÊN lần đặt trước, ghế giờ thuộc về Hoa
::why
Gần đúng ở việc bạn hình dung đúng "đặt ghế lần hai đè lên lần đầu" là
một nguy cơ có thật trong nhiều hệ thống — trực giác cảnh giác đó
không sai.

Chỗ lệch: `dat_ghe()` không hề "đè" — nó chỉ khớp `case GheTrong(...)`. `lan1` mang kiểu `GheDaDat`, không khớp nhánh đó, nên hàm không
bao giờ chạy tới chỗ tạo ra một `GheDaDat` thứ hai. Nó dừng lại ở
`case _` và ném lỗi trước khi có chuyện "đè" nào xảy ra.
::
:::

:::opt
In ra `GheDaDat(so_ghe='12A', ten_khach='Minh', yeu_cau=('ăn chay',))`
— máy bỏ qua lần gọi thứ hai vì ghế "đã có chủ", giữ nguyên lần đặt
đầu
::why
Gần đúng ở việc bạn tin đúng: ghế đã có chủ thì không nên đặt lại được
— tinh thần đó chính xác.

Chỗ lệch nằm ở CÁCH máy phản ứng. `dat_ghe()` không lặng lẽ "bỏ qua"
rồi trả về giá trị cũ — nó KHÔNG BAO GIỜ chạy tới dòng `return` nào cả.
Nhánh `case _` của nó `raise ValueError(...)`, dừng chương trình lại
ngay, không có `GheDaDat` nào (cũ hay mới) được in ra.
::
:::

:::opt
Máy báo lỗi cú pháp, vì `dat_ghe` được định nghĩa để chỉ nhận đúng MỘT
kiểu tham số đầu, gọi hai lần với hai kiểu khác nhau (`GheTrong` rồi
`GheDaDat`) là sai kiểu ngay từ lúc định nghĩa
::why
Gần đúng ở việc bạn để ý đúng: `dat_ghe()` thật sự chỉ XỬ LÝ đúng một
kiểu trạng thái nguồn.

Chỗ lệch: Python không kiểm kiểu tham số lúc ĐỊNH NGHĨA hàm — hàm nhận
bất kỳ giá trị nào, `GheTrong` hay `GheDaDat` đều gọi được, không báo
lỗi cú pháp. Việc từ chối chỉ xảy ra LÚC CHẠY, bên trong thân hàm, qua
nhánh `case _` — đúng bằng `ValueError`, không phải lỗi cú pháp.
::
:::
::::

::::code{#xu-ly-yeu-cau-ho-tro}
Ghép đủ ba mảnh: `YeuCauMoi` (một `tieu_de` và một `tuple` `nhan` các
nhãn) chuyển thành `YeuCauDaXuLy` (thêm `phan_hoi`) qua hàm `xu_ly()`.
Chỉ xử lý được một `YeuCauMoi`. Không `print`, không sửa dữ liệu tại
chỗ.

```python title=starter
from dataclasses import dataclass

@dataclass(frozen=True)
class YeuCauMoi:
    tieu_de: str
    nhan: tuple

@dataclass(frozen=True)
class YeuCauDaXuLy:
    tieu_de: str
    nhan: tuple
    phan_hoi: str

def xu_ly(yc, phan_hoi):
    ___

yc = YeuCauMoi("Máy in hỏng", ("khẩn-cấp", "phần-cứng"))
da_xong = xu_ly(yc, "Đã đổi mực mới")

print(f"yc:      {yc}")
print(f"da_xong: {da_xong}")
```

```python title=solution
from dataclasses import dataclass

@dataclass(frozen=True)
class YeuCauMoi:
    tieu_de: str
    nhan: tuple

@dataclass(frozen=True)
class YeuCauDaXuLy:
    tieu_de: str
    nhan: tuple
    phan_hoi: str

def xu_ly(yc, phan_hoi):
    match yc:
        case YeuCauMoi(tieu_de=t, nhan=n):
            return YeuCauDaXuLy(tieu_de=t, nhan=n, phan_hoi=phan_hoi)
        case _:
            raise ValueError("chỉ xử lý được yêu cầu còn MỚI")

yc = YeuCauMoi("Máy in hỏng", ("khẩn-cấp", "phần-cứng"))
da_xong = xu_ly(yc, "Đã đổi mực mới")

print(f"yc:      {yc}")
print(f"da_xong: {da_xong}")
```

```python title=test
assert isinstance(yc, YeuCauMoi), "yc gốc không được đổi kiểu sau khi gọi xu_ly"
assert yc == YeuCauMoi("Máy in hỏng", ("khẩn-cấp", "phần-cứng")), "yc gốc không được đổi giá trị"
assert isinstance(da_xong, YeuCauDaXuLy), "xu_ly(yc, ...) phải trả về một YeuCauDaXuLy"
assert da_xong.phan_hoi == "Đã đổi mực mới", "phan_hoi phải đúng giá trị truyền vào"
assert da_xong.nhan == ("khẩn-cấp", "phần-cứng"), "nhan phải giữ nguyên từ yc"

da_nem = False
try:
    xu_ly(da_xong, "xử lý lần nữa")
except ValueError:
    da_nem = True
assert da_nem, "xu_ly() gọi trên một YeuCauDaXuLy (không phải YeuCauMoi) phải ném ValueError"
```

:::hints
- kind: attention
  body: Chỗ trống là toàn bộ thân hàm xu_ly — một khối match trên yc, cùng cấu trúc với dat_ghe() ở ví dụ trên.
- kind: strategy
  body: 'case YeuCauMoi(tieu_de=t, nhan=n): lấy hai field ra để dựng YeuCauDaXuLy(tieu_de=t, nhan=n, phan_hoi=phan_hoi). case _: phải raise ValueError(...) — không được return None hay bỏ qua.'
- kind: one-line
  body: "match yc:\n    case YeuCauMoi(tieu_de=t, nhan=n):\n        return YeuCauDaXuLy(tieu_de=t, nhan=n, phan_hoi=phan_hoi)\n    case _:\n        raise ValueError(\"chỉ xử lý được yêu cầu còn MỚI\")"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: xu_ly() phải dùng match để khớp đúng kiểu YeuCauMoi, và phải THUẦN — không print, không global, không sửa dữ liệu tại chỗ ngay trong thân hàm này
  requireAst:
  - kind: match-stmt, min: 1
  - kind: pure-fn, target: xu_ly
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "da_xong: YeuCauDaXuLy(tieu_de='Máy in hỏng', nhan=('khẩn-cấp', 'phần-cứng'), phan_hoi='Đã đổi mực mới')"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`match`, `tuple`, `frozen=True`, không side-effect — bốn mảnh đứng
chung, không lệch một luật nào. Sẵn sàng cho bài cuối.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi vào bài cuối cùng của track này.

Bài này ghép ba mảnh của MỘT cụm. Nhưng cả track, tính từ đầu, đã dựng
ra nhiều hơn thế rất nhiều: bốn công cụ bất biến (`tuple`, `frozenset`,
`frozen=True`, `MappingProxyType`), `replace()`, hàm thuần và tiêm phụ
thuộc, `deepcopy`, và state machine dùng `match`.

Bài cuối không hỏi riêng một cụm nào nữa. Nó đòi bạn viết một chương
trình dùng ĐỦ — cả bốn cụm, cùng một lúc.
::::

::::checkpoint{mastery=0.8}
::::
