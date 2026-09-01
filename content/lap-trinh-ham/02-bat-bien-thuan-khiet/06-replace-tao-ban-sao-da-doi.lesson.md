---
id: lap-trinh-ham.bat-bien-thuan-khiet.replace-tao-ban-sao-da-doi
title: "`dataclasses.replace()` — 'sửa' bằng cách tạo bản sao đã đổi"
summary: "replace(cfg, port=3000) không sửa cfg — nó tạo một đối tượng MỚI với field port đã đổi, mọi field khác giữ nguyên copy từ cfg. Không động từ SỬA nào chạm vào cfg — chỉ có TẠO MỚI."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.dataclasses-replace]
requires: [fp.frozen-dataclass]
concepts: [fp.dataclasses-replace]
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
Field không sửa được nữa — nhưng đơn hàng vẫn cần đổi trạng thái. Byte
chỉ bạn cách đúng: đừng sửa, hãy TẠO MỚI.
::::

::::explain{#tao-moi-thay-vi-sua-field}
`frozen=True` chặn đúng MỘT việc: gán lại field của một đối tượng ĐÃ
TỒN TẠI. Nó không chặn việc TẠO một đối tượng khác — và đó chính là lối
ra. Muốn có một "đơn hàng đã đổi trạng thái", đừng sửa đơn hàng cũ; hãy
dựng một đơn hàng MỚI, giống hệt đơn cũ ở mọi field, chỉ khác đúng field
cần đổi.

Gõ tay lại toàn bộ field không đổi mỗi lần chỉ để đổi một field thì rất
dễ gõ nhầm, và cực kỳ phiền khi đối tượng có nhiều field. `dataclasses`
có sẵn một hàm làm đúng việc đó: `replace()`.

```python
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class DonHang:
    ma_don: str
    trang_thai: str

don = DonHang("DH-88", "cho_xac_nhan")
don_moi = replace(don, trang_thai="da_giao")
```

`replace(don, trang_thai="da_giao")` đọc TOÀN BỘ field của `don`, dựng
một `DonHang` MỚI với `trang_thai` đã đổi thành `"da_giao"`, còn
`ma_don` giữ nguyên — COPY thẳng từ `don`, không cần bạn gõ lại. Không
động từ SỬA nào chạm vào `don` — chỉ có TẠO MỚI, đúng tinh thần xuyên
suốt track này.
::::

::::example{#don-cu-van-nguyen}
Cùng `DonHang` ở trên, quan sát cả hai đối tượng sau khi gọi `replace()`:

```python title=readonly
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class DonHang:
    ma_don: str
    trang_thai: str

don = DonHang("DH-88", "cho_xac_nhan")
don_cap_nhat = replace(don, trang_thai="da_giao")

print(f"don:          {don}")
print(f"don_cap_nhat: {don_cap_nhat}")
print(f"Cùng object không: {don is don_cap_nhat}")
```

```text title=readonly
don:          DonHang(ma_don='DH-88', trang_thai='cho_xac_nhan')
don_cap_nhat: DonHang(ma_don='DH-88', trang_thai='da_giao')
Cùng object không: False
```

`don` vẫn giữ nguyên `trang_thai='cho_xac_nhan'` — hoàn toàn không bị
động tới. `don_cap_nhat` là một `DonHang` khác hẳn (`is` ra `False`),
mang `trang_thai` mới, còn `ma_don` thì COPY nguyên vẹn từ `don`, dù lời
gọi `replace()` không hề nhắc tới `ma_don`.
::::

::::predict{#doan-ba-field commitOnce}
`MonAn` lần này có BA field. `replace()` chỉ đổi MỘT trong số đó.

**Trước khi chạy**, bạn đoán hai dòng cuối in ra gì?

```python
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class MonAn:
    ten: str
    gia: int
    con_hang: bool

mon = MonAn("Bún bò", 45000, True)
mon_giam_gia = replace(mon, gia=35000)

print(f"mon:          {mon}")
print(f"mon_giam_gia: {mon_giam_gia}")
```

:::opt{correct}
`mon:          MonAn(ten='Bún bò', gia=45000, con_hang=True)` và
`mon_giam_gia: MonAn(ten='Bún bò', gia=35000, con_hang=True)`
:::

:::opt
`mon:          MonAn(ten='Bún bò', gia=45000, con_hang=True)` và
`mon_giam_gia: MonAn(ten='Bún bò', gia=35000, con_hang=False)` — field
con_hang không được nhắc tới nên trở về giá trị mặc định
::why
Gần đúng ở việc `gia` bạn tính đúng — đổi thành `35000` chính là điều
`replace()` làm.

Chỗ lệch: `replace()` không "dựng lại từ đầu rồi để trống field còn
lại" — nó COPY nguyên giá trị CŨ cho MỌI field bạn không nêu tên trong
lời gọi. `con_hang` không hề được nhắc tới trong `replace(mon, gia=35000)`,
nên nó giữ nguyên `True` — đúng giá trị đang có trong `mon`.
::
:::

:::opt
Máy báo lỗi thiếu tham số, vì `replace()` cũng cần đủ cả ba field như
gọi constructor `MonAn(...)` trực tiếp
::why
Gần đúng ở việc bạn nhớ đúng: gọi constructor `MonAn(...)` trực tiếp
ĐÚNG LÀ cần đủ mọi field, đúng thứ tự.

Chỗ lệch: `replace()` không phải constructor trần — nó là một hàm RIÊNG,
tự đọc field còn thiếu từ chính đối tượng gốc (`mon`) trước khi dựng
đối tượng mới. Bạn chỉ cần nêu tên field muốn đổi, không phải liệt kê
lại toàn bộ.
::
:::

:::opt
`mon:          MonAn(ten='Bún bò', gia=35000, con_hang=True)` và
`mon_giam_gia: MonAn(ten='Bún bò', gia=35000, con_hang=True)` — cả hai
đều đổi, vì replace() sửa luôn đối tượng gốc
::why
Gần đúng ở việc bạn cảnh giác — nhiều hàm trong Python đúng là sửa tại
chỗ đối số của chúng (`list.sort()` chẳng hạn).

Chỗ lệch: `replace()` không đụng gì tới `mon` — nó luôn TRẢ VỀ một đối
tượng HOÀN TOÀN MỚI, đúng tinh thần track này: không sửa, chỉ tạo mới.
`mon` phải giữ nguyên `gia=45000` y hệt trước khi gọi `replace()`.
::
:::
::::

::::code{#mon-het-hang}
Trà đá vừa hết hàng. Cập nhật `con_hang` thành `False` cho một `MonAn`
MỚI — không đụng `ten`/`gia`, và không sửa `mon` gốc.

```python title=starter
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class MonAn:
    ten: str
    gia: int
    con_hang: bool

mon = MonAn("Trà đá", 5000, True)

# Hết hàng — tạo một MonAn MỚI với con_hang=False, KHÔNG đụng gì tới
# ten/gia, và KHÔNG sửa mon gốc.
mon_het_hang = ___

print(f"mon:          {mon}")
print(f"mon_het_hang: {mon_het_hang}")
```

```python title=solution
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class MonAn:
    ten: str
    gia: int
    con_hang: bool

mon = MonAn("Trà đá", 5000, True)

# Hết hàng — tạo một MonAn MỚI với con_hang=False, KHÔNG đụng gì tới
# ten/gia, và KHÔNG sửa mon gốc.
mon_het_hang = replace(mon, con_hang=False)

print(f"mon:          {mon}")
print(f"mon_het_hang: {mon_het_hang}")
```

```python title=test
assert mon == MonAn("Trà đá", 5000, True), "mon gốc không được đổi"
assert mon_het_hang == MonAn("Trà đá", 5000, False), "mon_het_hang phải giữ nguyên ten, gia, chỉ đổi con_hang thành False"
assert mon is not mon_het_hang, "mon_het_hang phải là object MỚI, không được là chính mon"
```

:::hints
- kind: attention
  body: Chỗ trống là một lời gọi hàm — dùng replace() đã import sẵn ở dòng đầu, không phải gọi lại constructor MonAn(...).
- kind: strategy
  body: 'replace() nhận đối tượng gốc làm tham số đầu, rồi các field muốn đổi dưới dạng tên=giá_trị. Chỉ cần nêu con_hang=False — ten và gia sẽ tự COPY từ mon.'
- kind: one-line
  body: 'Điền `replace(mon, con_hang=False)` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải gọi replace(mon, con_hang=False) — dùng đúng hàm replace() đã import, không gọi lại constructor MonAn(...) hay gán chính mon
  requireAst:
  - kind: uses-call, target: replace, min: 1
  - kind: no-mutation
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^mon:          MonAn\(ten='Trà đá', gia=5000, con_hang=True\)\nmon_het_hang: MonAn\(ten='Trà đá', gia=5000, con_hang=False\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`mon` không hề hay biết. `mon_het_hang` mang đúng thay đổi cần có. Bạn
vừa ghép đủ `frozen=True` và `replace()` — chặn được, và vẫn đổi được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sáu bài vừa qua, bạn học cách KHÔNG SỬA DỮ LIỆU TẠI CHỖ — luôn tạo mới.
Nhưng thử tưởng tượng: một hàm dùng đúng `frozen=True` và `replace()`,
không field nào bị sửa tại chỗ — nhưng giữa chừng, nó gọi `print()` để
ghi log, hay đọc `random.random()` để tính một mã giảm giá ngẫu nhiên.

Không field nào bị sửa. Vậy hàm đó có còn ĐÁNG TIN như một hàm "sạch"
không — hay bất biến dữ liệu chỉ là MỘT PHẦN của câu chuyện?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
