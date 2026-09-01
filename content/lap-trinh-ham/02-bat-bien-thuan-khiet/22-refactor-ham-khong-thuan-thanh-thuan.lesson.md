---
id: lap-trinh-ham.bat-bien-thuan-khiet.refactor-ham-khong-thuan-thanh-thuan
title: "Refactor một hàm KHÔNG thuần thành THUẦN hoàn toàn"
summary: "Một hàm dùng global total_orders + print() để đếm và ghi log — viết lại thành một @dataclass(frozen=True) giữ trạng thái và một hàm THUẦN nhận trạng thái cũ, trả về trạng thái mới. Không global, không print."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.refactor-to-pure]
requires: [fp.state-machine-compose]
concepts: [fp.refactor-to-pure]
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
Bài trước hỏi: một hàm không sửa dữ liệu tại chỗ, nhưng có `print()` hay
đọc biến toàn cục — còn ĐÁNG TIN không? Hôm nay Byte đưa ra một hàm THẬT
phạm đúng hai lỗi đó, và bạn viết lại nó thành THUẦN.
::::

::::explain{#ham-dem-don-khong-thuan}
Một hàm đếm đơn hàng, kiểu rất quen thuộc:

```python
tong_don_toan_cuc = 0

def xu_ly_don_khong_thuan(so_tien):
    global tong_don_toan_cuc
    tong_don_toan_cuc += 1
    print(f"Đơn #{tong_don_toan_cuc}: {so_tien}")
```

Hàm này phạm CẢ HAI lỗi track này đã dạy riêng lẻ: `global
tong_don_toan_cuc` (`fp.nondeterministic-impure` — phụ thuộc trạng
thái BÊN NGOÀI, không nằm trong tham số) và `print(...)`
(`fp.io-is-side-effect` — tác động ra thế giới bên ngoài lời gọi hàm).
Gọi `xu_ly_don_khong_thuan(100)` hai lần cho HAI KẾT QUẢ khác nhau
trên màn hình (`Đơn #1` rồi `Đơn #2`), dù đối số truyền vào giống hệt
nhau — không thể viết một `assert` cố định nào cho hàm này, vì kết quả
phụ thuộc vào SỐ LẦN đã gọi trước đó, một thứ nằm ngoài tầm nhìn của
riêng lời gọi đang xét.

Cách sửa: gói `tong_don_toan_cuc` và nhật ký log vào MỘT trạng thái bất
biến, rồi viết một hàm THUẦN nhận trạng thái CŨ, trả về trạng thái MỚI
— đúng khuôn `fp.state-as-data` đã dựng suốt cụm này, áp dụng cho một
bộ đếm thay vì một bài viết:

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class TrangThaiDon:
    so_don: int
    nhat_ky: tuple

def xu_ly_don_thuan(trang_thai, so_tien):
    so_don_moi = trang_thai.so_don + 1
    dong_log = f"Đơn #{so_don_moi}: {so_tien}"
    return TrangThaiDon(so_don=so_don_moi, nhat_ky=trang_thai.nhat_ky + (dong_log,))
```

Không `global` — `trang_thai` được TRUYỀN VÀO, không đọc từ đâu ngoài
tham số. Không `print` — dòng log được THÊM VÀO một `tuple` trả về,
không in ra màn hình; ai gọi hàm tự quyết định có in nó ra hay không.
::::

::::example{#cung-input-cung-output}
Gọi hàm THUẦN hai lần, với CÙNG trạng thái đầu vào:

```python title=readonly
from dataclasses import dataclass

@dataclass(frozen=True)
class TrangThaiDon:
    so_don: int
    nhat_ky: tuple

def xu_ly_don_thuan(trang_thai, so_tien):
    so_don_moi = trang_thai.so_don + 1
    dong_log = f"Đơn #{so_don_moi}: {so_tien}"
    return TrangThaiDon(so_don=so_don_moi, nhat_ky=trang_thai.nhat_ky + (dong_log,))

s0 = TrangThaiDon(0, ())
a = xu_ly_don_thuan(s0, 100)
b = xu_ly_don_thuan(s0, 100)

print(a == b)
print(a is b)
```

```text title=readonly
True
False
```

`a == b` là `True` — cùng `s0`, cùng `so_tien=100`, luôn ra một
`TrangThaiDon` có giá trị GIỐNG HỆT nhau. Đây chính là `assert f(x) ==
f(x)` của `fp.test-purity`, giờ hoạt động thật trên một hàm đúng nghĩa
thuần. `a is b` là `False` — hai lời gọi tạo ra hai OBJECT khác nhau
trong bộ nhớ, dù giá trị bên trong giống hệt; hàm thuần không hứa "ra
cùng một object", chỉ hứa "ra giá trị bằng nhau".
::::

::::predict{#doan-hai-lan-goi commitOnce}
Byte gọi hàm thuần BA lần liên tiếp, mỗi lần đưa TRẠNG THÁI MỚI NHẤT
vào lần sau — không lần nào dùng lại `s0` cũ.

**Trước khi chạy**, bạn đoán dòng cuối in ra gì?

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class TrangThaiDon:
    so_don: int
    nhat_ky: tuple

def xu_ly_don_thuan(trang_thai, so_tien):
    so_don_moi = trang_thai.so_don + 1
    dong_log = f"Đơn #{so_don_moi}: {so_tien}"
    return TrangThaiDon(so_don=so_don_moi, nhat_ky=trang_thai.nhat_ky + (dong_log,))

s0 = TrangThaiDon(0, ())
s1 = xu_ly_don_thuan(s0, 100)
s2 = xu_ly_don_thuan(s1, 200)
s3 = xu_ly_don_thuan(s2, 300)

print(s0.so_don)
```

:::opt{correct}
`0`
:::

:::opt
`3` — `s0` là điểm bắt đầu của cả chuỗi, nên nó "đi theo" tới trạng
thái cuối cùng
::why
Gần đúng ở việc bạn theo dõi đúng chuỗi: `s0` → `s1` → `s2` → `s3`,
mỗi bước cộng thêm một đơn.

Chỗ lệch: mỗi bước TẠO MỘT `TrangThaiDon` MỚI, không sửa cái cũ.
`s0` không "đi theo" chuỗi — nó là một giá trị ĐỨNG YÊN, được ĐỌC một
lần duy nhất để tạo ra `s1`. `s0.so_don` mãi mãi là `0`, bất kể có bao
nhiêu bước được tạo ra từ nó sau đó.
::
:::

:::opt
Máy báo lỗi, vì `s0` đã bị "dùng hết" sau khi được truyền vào
`xu_ly_don_thuan` để tạo `s1`
::why
Gần đúng ở việc bạn cẩn trọng về vòng đời một giá trị — phản xạ đó
đúng cho ngôn ngữ có luật sở hữu như Rust.

Chỗ lệch: Python không có khái niệm "dùng hết" một giá trị khi truyền
nó vào hàm. `xu_ly_don_thuan(s0, 100)` chỉ ĐỌC field của `s0`, không hề
làm `s0` mất hiệu lực. `s0` vẫn dùng được, đọc được, bình thường sau
lời gọi đó — mãi mãi.
::
:::

:::opt
`1` — vì `s0` bị `xu_ly_don_thuan` sửa tại chỗ ngay ở lần gọi đầu tiên,
trước khi có `s1`, `s2`, `s3`
::why
Gần đúng ở việc bạn cảnh giác đúng chỗ: "sửa tại chỗ" đúng là điều
track này liên tục cảnh báo.

Chỗ lệch: `xu_ly_don_thuan` không sửa gì cả — nó `return` một
`TrangThaiDon(...)` MỚI. Không có dòng nào trong thân hàm gán vào một
field hay một ô của `trang_thai`; hàm chỉ ĐỌC `trang_thai.so_don` và
`trang_thai.nhat_ky` để TÍNH ra giá trị cho object mới.
::
:::
::::

::::code{#refactor-thanh-thuan}
Viết `xu_ly_don_thuan(trang_thai, so_tien)` — bản THUẦN của
`xu_ly_don_khong_thuan` phía trên (chỉ để THAM KHẢO, không cần sửa).
Không `global`, không `print`. `TrangThaiDon` giữ `so_don` (số đơn đã
xử lý) và `nhat_ky` (`tuple` các dòng log dạng `"Đơn #N: so_tien"`).

```python title=starter
from dataclasses import dataclass

# Phiên bản KHÔNG THUẦN — chỉ để THAM KHẢO, không cần sửa đoạn này.
tong_don_toan_cuc = 0

def xu_ly_don_khong_thuan(so_tien):
    global tong_don_toan_cuc
    tong_don_toan_cuc += 1
    print(f"Đơn #{tong_don_toan_cuc}: {so_tien}")

@dataclass(frozen=True)
class TrangThaiDon:
    so_don: int
    nhat_ky: tuple

# Việc của bạn: viết bản THUẦN, không global, không print.
def xu_ly_don_thuan(trang_thai, so_tien):
    ___

s0 = TrangThaiDon(0, ())
s1 = xu_ly_don_thuan(s0, 100)
s2 = xu_ly_don_thuan(s1, 200)

print(f"s0: {s0}")
print(f"s1: {s1}")
print(f"s2: {s2}")
```

```python title=solution
from dataclasses import dataclass

# Phiên bản KHÔNG THUẦN — chỉ để THAM KHẢO, không cần sửa đoạn này.
tong_don_toan_cuc = 0

def xu_ly_don_khong_thuan(so_tien):
    global tong_don_toan_cuc
    tong_don_toan_cuc += 1
    print(f"Đơn #{tong_don_toan_cuc}: {so_tien}")

@dataclass(frozen=True)
class TrangThaiDon:
    so_don: int
    nhat_ky: tuple

# Việc của bạn: viết bản THUẦN, không global, không print.
def xu_ly_don_thuan(trang_thai, so_tien):
    so_don_moi = trang_thai.so_don + 1
    dong_log = f"Đơn #{so_don_moi}: {so_tien}"
    return TrangThaiDon(so_don=so_don_moi, nhat_ky=trang_thai.nhat_ky + (dong_log,))

s0 = TrangThaiDon(0, ())
s1 = xu_ly_don_thuan(s0, 100)
s2 = xu_ly_don_thuan(s1, 200)

print(f"s0: {s0}")
print(f"s1: {s1}")
print(f"s2: {s2}")
```

```python title=test
assert s0 == TrangThaiDon(0, ()), "s0 không được đổi bởi các lần gọi sau"
assert s1 == TrangThaiDon(1, ("Đơn #1: 100",)), "s1 sai — kiểm lại so_don và nội dung dòng log"
assert s2 == TrangThaiDon(2, ("Đơn #1: 100", "Đơn #2: 200")), "s2 sai — nhat_ky phải CHỨA CẢ hai dòng log, theo đúng thứ tự"

x = xu_ly_don_thuan(s2, 999)
y = xu_ly_don_thuan(s2, 999)
assert x == y, "gọi hai lần với CÙNG trạng thái và CÙNG so_tien phải ra kết quả BẰNG NHAU — đây là dấu hiệu của một hàm thuần"
```

:::hints
- kind: attention
  body: Chỗ trống là toàn bộ thân hàm xu_ly_don_thuan. Không dùng global, không gọi print — chỉ tính toán rồi return một TrangThaiDon MỚI.
- kind: strategy
  body: 'Tính so_don_moi = trang_thai.so_don + 1. Dựng chuỗi log bằng f-string, ví dụ f"Đơn #{so_don_moi}: {so_tien}". nhat_ky mới là trang_thai.nhat_ky CỘNG một tuple một phần tử chứa dòng log đó — dùng dấu phẩy sau phần tử để nó thành tuple một phần tử: (dong_log,).'
- kind: one-line
  body: "so_don_moi = trang_thai.so_don + 1\ndong_log = f\"Đơn #{so_don_moi}: {so_tien}\"\nreturn TrangThaiDon(so_don=so_don_moi, nhat_ky=trang_thai.nhat_ky + (dong_log,))"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  # CHỈ soi bằng `pure-fn` với target, KHÔNG thêm `no-global` đứng riêng:
  # `no-global` không nhận target, nó quét TOÀN BỘ mã nộp — mà đoạn tham
  # khảo `xu_ly_don_khong_thuan` (cố ý giữ nguyên trong bài, không cần
  # sửa) CÓ `global` thật. Thêm `no-global` đứng riêng sẽ làm ngay cả lời
  # giải ĐÚNG cũng trượt — luật static phải soi ĐÚNG một hàm `tg` đang
  # được chấm, không soi cả file.
  onFail: xu_ly_don_thuan() phải THUẦN — không global/nonlocal, không gọi print (hay bất kỳ hàm IO nào), không sửa dữ liệu tại chỗ, ngay TRONG THÂN của chính hàm này. Trả về một TrangThaiDon MỚI bằng cách tính toán từ trang_thai và so_tien.
  requireAst:
  - kind: pure-fn, target: xu_ly_don_thuan
  # `pure-fn` CHỈ xác nhận VẮNG MẶT tạp chất — một thân hàm rỗng (điền
  # bừa `___` bằng `True`) cũng "thuần" theo đúng nghĩa hẹp đó (không
  # global, không IO, không sửa tại chỗ — vì nó không làm GÌ cả). Luật
  # này một mình không phân biệt được điền bừa; thêm `uses-name` đếm số
  # lần THẬT SỰ đọc `trang_thai` (đúng 2: `.so_don` và `.nhat_ky`) để
  # đòi thân hàm có TÍNH TOÁN THẬT, không chỉ "không phạm lỗi".
  - kind: uses-name, target: trang_thai, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "s2: TrangThaiDon(so_don=2, nhat_ky=('Đơn #1: 100', 'Đơn #2: 200'))"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng ý tưởng — đếm đơn, ghi log — nhưng giờ `xu_ly_don_thuan` có thể
`assert` được, test được, và gọi lại bao nhiêu lần cũng không sợ kết
quả trôi. `global` và `print` đã bị đẩy hẳn ra khỏi thân hàm.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Track này đã đi qua bốn cụm: công cụ bất biến (`tuple`, `frozenset`,
`frozen=True`, `MappingProxyType`), hàm thuần và tiêm phụ thuộc, bất
biến trên cấu trúc dữ liệu lớn hơn, và state machine bất biến dùng
`match`. Bốn mảnh đó có ghép được vào MỘT chương trình duy nhất không —
dùng đủ cả bốn, không thiếu mảnh nào?

Hai bài cuối kiểm đúng điều đó.
::::

::::checkpoint{mastery=0.8}
::::
