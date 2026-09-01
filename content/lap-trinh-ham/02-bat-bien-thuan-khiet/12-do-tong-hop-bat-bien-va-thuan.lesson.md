---
id: lap-trinh-ham.bat-bien-thuan-khiet.do-tong-hop-bat-bien-va-thuan
title: "Đo tổng hợp: bất biến + thuần khiết trong một hàm"
summary: "Một hàm nhận một @dataclass(frozen=True) làm tham số, trả về MỘT ĐỐI TƯỢNG MỚI bằng replace(), không side-effect, không phụ thuộc nguồn không tất định — ghép đủ bài 5 tới 11 thành một hàm hoàn chỉnh. Không khái niệm mới, chỉ ghép lại."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.review-immutable-pure]
requires: [fp.test-purity]
concepts: [fp.review-immutable-pure]
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
Không khái niệm mới hôm nay. Byte chỉ ghép lại những gì bạn đã có — sáu
bài đầu về dữ liệu, năm bài sau về hàm — thành đúng MỘT hàm.
::::

::::explain{#ghep-hai-nua}
Mười một bài vừa qua chia làm hai nửa, và cả hai nửa đều đang nói về
đúng MỘT kỷ luật, chỉ nhìn từ hai góc:

- **Bài 1-6** nhìn từ phía DỮ LIỆU: đừng sửa tại chỗ, hãy tạo mới.
  `@dataclass(frozen=True)` chặn thật sự việc gán lại field;
  `dataclasses.replace()` là cách "sửa" đúng tinh thần đó — tạo một đối
  tượng mới, field cần đổi thì đổi, field còn lại copy nguyên.
- **Bài 7-11** nhìn từ phía HÀM: một hàm thuần luôn cho cùng kết quả với
  cùng đối số, và không làm gì khác ngoài tính toán — không `print`,
  không đọc `random`, không sửa dữ liệu ngoài tham số của nó.

Một hàm nhận một đối tượng `frozen=True` làm tham số, và trả về một đối
tượng MỚI (dựng bằng `replace()`) phản ánh đúng thay đổi cần có — đó là
nơi hai nửa này khớp vào nhau làm một. Hàm đó không sửa gì (vì đối
tượng nhận vào không sửa được, và hàm cũng không cố sửa nó); nó chỉ tính
ra một giá trị mới rồi trả về; và nó không đọc thứ gì ngoài đối số của
chính nó. Đó là bức tranh trọn vẹn mà track này xây dựng suốt mười một
bài — bài này chỉ đòi bạn LẮP nó lại, không dạy thêm gì.
::::

::::example{#nap-tien-vua-bat-bien-vua-thuan}
Byte viết một hàm nạp tiền vào tài khoản — tài khoản là `frozen=True`,
hàm nạp tiền là hàm thuần.

```python title=readonly
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class TaiKhoan:
    chu_tai_khoan: str
    so_du: int

def nap_tien(tk, so_tien):
    return replace(tk, so_du=tk.so_du + so_tien)

tk1 = TaiKhoan("Lan", 100000)
tk2 = nap_tien(tk1, 50000)

print(f"tk1: {tk1}")
print(f"tk2: {tk2}")
```

```text title=readonly
tk1: TaiKhoan(chu_tai_khoan='Lan', so_du=100000)
tk2: TaiKhoan(chu_tai_khoan='Lan', so_du=150000)
```

`tk1` không hề đổi sau khi gọi `nap_tien` — đúng tinh thần bài 4-6:
`frozen=True` chặn gán lại field, và `replace()` tạo một `TaiKhoan` MỚI
thay vì sửa cái cũ. `nap_tien` cũng đúng tinh thần bài 7-11: gọi nó hai
lần với cùng `(tk1, 50000)` luôn ra cùng một `TaiKhoan` mới, và thân hàm
không có dòng nào ngoài một `return`. Hai kỷ luật, một hàm, không mâu
thuẫn gì với nhau.
::::

::::predict{#doan-goi-hai-lan-nap-tien commitOnce}
Gọi `nap_tien` hai lần liên tiếp trên CÙNG một `tk1`, không gán kết quả
lần đầu vào đâu cả.

**Trước khi bấm chạy**, bạn đoán hai dòng cuối in ra gì?

```python
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class TaiKhoan:
    chu_tai_khoan: str
    so_du: int

def nap_tien(tk, so_tien):
    return replace(tk, so_du=tk.so_du + so_tien)

tk1 = TaiKhoan("Mai", 20000)
nap_tien(tk1, 5000)
nap_tien(tk1, 5000)

print(f"tk1: {tk1}")
print(f"Hai lần nạp ra cùng kết quả: {nap_tien(tk1, 5000) == nap_tien(tk1, 5000)}")
```

:::opt{correct}
`tk1: TaiKhoan(chu_tai_khoan='Mai', so_du=20000)` rồi
`Hai lần nạp ra cùng kết quả: True`
:::

:::opt
`tk1: TaiKhoan(chu_tai_khoan='Mai', so_du=30000)` rồi
`Hai lần nạp ra cùng kết quả: True` — hai lần gọi `nap_tien(tk1, 5000)`
ở giữa đã cộng dồn `5000` hai lần vào `tk1`
::why
Gần đúng ở việc bạn nhớ đúng có HAI lời gọi `nap_tien(tk1, 5000)` xảy
ra trước dòng in — đếm số lần gọi không sai.

Chỗ lệch: hai lời gọi đó không GÁN kết quả vào đâu cả — dòng
`nap_tien(tk1, 5000)` đứng một mình chỉ tạo ra một `TaiKhoan` mới rồi bỏ
đi ngay, không ai giữ lại. `nap_tien` không sửa `tk1` — nó không có khả
năng làm vậy, vì `TaiKhoan` là `frozen=True`. `tk1` chỉ đổi nếu bạn GÁN
LẠI cái tên `tk1` bằng kết quả trả về, và không dòng nào ở đây làm điều
đó.
::
:::

:::opt
`tk1: TaiKhoan(chu_tai_khoan='Mai', so_du=20000)` rồi
`Hai lần nạp ra cùng kết quả: False` — `replace()` tạo hai object khác
nhau nên so sánh bằng `==` phải ra `False`
::why
Gần đúng ở việc bạn nhớ đúng `replace()` tạo ra một object MỚI mỗi lần
gọi — hai object đó thật sự là hai VẬT khác nhau trong bộ nhớ.

Chỗ lệch: `==` giữa hai `@dataclass` không so ĐỊA CHỈ (đó là việc của
`is`) — nó so NỘI DUNG từng field. Hai `TaiKhoan` khác nhau về địa chỉ
vẫn có thể bằng nhau qua `==` nếu mọi field của chúng giống hệt nhau.
`nap_tien(tk1, 5000)` cả hai lần đều cộng đúng `5000` vào CÙNG `so_du`
gốc của `tk1`, ra cùng nội dung, nên `==` phải là `True`.
::
:::

:::opt
Máy dừng lại báo lỗi `FrozenInstanceError`, vì gọi `nap_tien(tk1, 5000)`
hai lần trên cùng một `tk1` là hai lần sửa `tk1`
::why
Gần đúng ở việc bạn cảnh giác đúng chỗ — `FrozenInstanceError` CÓ THẬT,
và nó CÓ xảy ra khi ai đó cố gán lại field của một object `frozen=True`.

Chỗ lệch: `nap_tien` không hề gán lại field nào của `tk1`. Nó gọi
`replace(tk, ...)`, một hàm ĐỌC field của `tk1` để dựng một object HOÀN
TOÀN MỚI — `tk1` chỉ bị ĐỌC, chưa bao giờ bị GHI. Gọi `nap_tien(tk1,
5000)` bao nhiêu lần cũng không đụng gì tới `tk1`, nên không có lỗi nào
xảy ra.
::
:::
::::

::::code{#nap-tien-hoan-chinh}
Viết `nap_tien(tk, so_tien)` — nhận một `TaiKhoan` (đã `frozen=True`) và
số tiền cần nạp, trả về một `TaiKhoan` MỚI với `so_du` đã cộng thêm,
không sửa `tk` gốc, không side-effect nào.

```python title=starter
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class TaiKhoan:
    chu_tai_khoan: str
    so_du: int

def nap_tien(tk, so_tien):
    ___

tk1 = TaiKhoan("Lan", 100000)
tk2 = nap_tien(tk1, 50000)

print(f"tk1: {tk1}")
print(f"tk2: {tk2}")
```

```python title=solution
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class TaiKhoan:
    chu_tai_khoan: str
    so_du: int

def nap_tien(tk, so_tien):
    return replace(tk, so_du=tk.so_du + so_tien)

tk1 = TaiKhoan("Lan", 100000)
tk2 = nap_tien(tk1, 50000)

print(f"tk1: {tk1}")
print(f"tk2: {tk2}")
```

```python title=test
assert tk1 == TaiKhoan("Lan", 100000), "tk1 gốc không được đổi sau khi gọi nap_tien"
assert tk2 == TaiKhoan("Lan", 150000), "tk2 phải có so_du = 100000 + 50000 = 150000, chu_tai_khoan giữ nguyên"
assert tk1 is not tk2, "tk2 phải là một object MỚI, không được là chính tk1"
assert nap_tien(TaiKhoan("Mai", 20000), 5000) == TaiKhoan("Mai", 25000), "phải cộng ĐÚNG so_tien vào so_du gốc, không hardcode một con số cố định"
assert nap_tien(tk1, 50000) == nap_tien(tk1, 50000), "gọi hai lần cùng đối số phải ra cùng kết quả — nap_tien phải THUẦN"
```

:::hints
- kind: attention
  body: Chỗ trống thay cho toàn bộ thân hàm — một dòng return duy nhất, dùng replace() đã import sẵn, không tự gọi constructor TaiKhoan(...) và không print gì cả.
- kind: strategy
  body: 'replace() nhận object gốc làm tham số đầu, rồi field cần đổi dưới dạng tên=giá_trị. Field cần đổi ở đây là so_du, giá trị mới là so_du CŨ (tk.so_du) cộng thêm so_tien.'
- kind: one-line
  body: "Điền `return replace(tk, so_du=tk.so_du + so_tien)` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: nap_tien phải dùng replace() để tạo TaiKhoan MỚI, không tự gọi constructor TaiKhoan(...), và phải THUẦN — tầng static phát hiện dấu hiệu không thuần (print, global, sửa tại chỗ, hoặc gọi nguồn không tất định) trong thân hàm
  requireAst:
  - kind: frozen-dataclass, target: TaiKhoan
  - kind: uses-call, target: replace, min: 1
  - kind: pure-fn, target: nap_tien
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^tk1: TaiKhoan\(chu_tai_khoan='Lan', so_du=100000\)\ntk2: TaiKhoan\(chu_tai_khoan='Lan', so_du=150000\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`tk1` không hay biết. `tk2` mang đúng thay đổi. Hàm không in gì, không
đọc gì ngoài đối số của chính nó. Đó là cả hai nửa track này, trong một
hàm.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Track này giờ có ba công cụ chặn sửa THẬT SỰ: `tuple` (chặn sửa một dãy
có thứ tự), `frozenset` chưa gặp nhưng cùng họ với `tuple`, và
`frozen=True` (chặn gán lại field của một object). Nhưng một cấu trúc
dữ liệu quen thuộc khác — `dict`, dùng để TRA CỨU theo khoá — vẫn chưa
có cách nào khoá lại. Một `dict` cấu hình bạn không muốn ai lỡ tay sửa
vẫn có thể bị gán đè `cau_hinh["cong"] = 9999` bất cứ lúc nào.

Có cách nào khoá một `dict` lại, để gán vào một ô của nó cũng bị chặn
thật sự, giống `tuple` chặn `.append`?

Bài sau bắt đầu cụm mới — bất biến ở những cấu trúc dữ liệu lớn hơn một
object hai field — và trả lời đúng câu hỏi đó.
::::

::::checkpoint{mastery=0.8}
::::
