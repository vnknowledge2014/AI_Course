---
id: toan.to-hop-xac-suat-thong-ke.quy-tac-nhan-cho-day-phep-thu
title: Quy tắc nhân cho một dãy phép thử
summary: "P(A₁∩A₂∩...∩Aₙ) = P(A₁)·P(A₂)·...·P(Aₙ) khi mọi Aᵢ độc lập — mở rộng bài 16 lên n phép thử liên tiếp. Gieo bốn hạt, mỗi hạt độc lập nảy mầm với xác suất riêng, xác suất CẢ BỐN cùng nảy là tích bốn xác suất."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.independence-chain]
requires: [math.probability-independence]
concepts: [math.quy-tac-nhan-xac-suat]
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
Bốn hạt, MỖI hạt độc lập nảy mầm với xác suất riêng. Xác suất CẢ BỐN
cùng nảy mầm — nhân HAI xác suất (bài 16) thôi có đủ không?
::::

::::explain{#nhan-cho-day-phep-thu}
Không đủ — cần NHÂN đủ BỐN. **`P(A₁∩A₂∩...∩Aₙ) = P(A₁)·P(A₂)·...·
P(Aₙ)`** khi MỌI `Aᵢ` độc lập — mở rộng bài 16 (chỉ hai biến cố) lên
`n` phép thử liên tiếp, đúng cấu trúc quy tắc nhân (bài 3, nhân dồn):

```python title=readonly
def xac_suat_tat_ca(danh_sach_xs):
    ket_qua = 1
    for p in danh_sach_xs:
        ket_qua = ket_qua * p
    return ket_qua


print(xac_suat_tat_ca([0.9, 0.8, 0.7, 0.6]))
```

```text title=readonly
0.3024
```

Bốn hạt với xác suất nảy mầm riêng `0.9, 0.8, 0.7, 0.6` — xác suất
CẢ BỐN cùng nảy LÀ tích `0.9×0.8×0.7×0.6=0.3024`. Vòng lặp NHÂN DỒN
(bài 3) áp thẳng lên MỌI SỐ trong danh sách, không riêng hai số đầu.
::::

::::example{#xac-suat-bang-nhau}
Khi MỌI hạt CÙNG xác suất nảy mầm — tích trở thành LUỸ THỪA:

```python title=readonly
def xac_suat_tat_ca(danh_sach_xs):
    ket_qua = 1
    for p in danh_sach_xs:
        ket_qua = ket_qua * p
    return ket_qua


print(xac_suat_tat_ca([0.5, 0.5, 0.5, 0.5]))
print(0.5 ** 4)
```

```text title=readonly
0.0625
0.0625
```

Bốn hạt CÙNG xác suất `0.5` — nhân bốn lần `0.5` khớp ĐÚNG `0.5⁴`.
Đây LÀ trường hợp riêng của "nhân dồn một danh sách" khi MỌI phần tử
GIỐNG nhau — quy tắc nhân (bài 3) đã dạy điều này cho phép ĐẾM, giờ
áp lại cho XÁC SUẤT.
::::

::::predict{#doan-mot-hat-chac-chan-khong-nay commitOnce}
Byte thêm MỘT hạt CHẮC CHẮN KHÔNG nảy mầm (xác suất `0.0` — hạt bị
hỏng):

```python
def xac_suat_tat_ca(danh_sach_xs):
    ket_qua = 1
    for p in danh_sach_xs:
        ket_qua = ket_qua * p
    return ket_qua

print(xac_suat_tat_ca([0.9, 0.5, 0.8, 0.0]))
```

Dòng cuối in ra gì?

:::opt{correct}
`0.0`
:::

:::opt
`0.36` — vì `0.9×0.5×0.8=0.36`, và MỘT hạt hỏng "bỏ qua", không
tính vào tích chung — chỉ nhân những hạt CÓ khả năng nảy mầm
::why
Gần đúng ở việc bạn tính ĐÚNG `0.9×0.5×0.8=0.36` (tích BA số đầu) —
phép nhân đó chính xác.

Chỗ lệch: hạt THỨ TƯ (xác suất `0.0`) KHÔNG hề bị "bỏ qua" — vòng
lặp nhân dồn (bài 3) đi qua MỌI phần tử của danh sách, kể cả `0.0`.
Nhân BẤT KỲ số nào với `0` LUÔN ra `0` — một hạt CHẮC CHẮN không nảy
(xác suất `0`) khiến "CẢ BỐN cùng nảy" trở thành KHÔNG THỂ, đúng
logic: muốn cả bốn cùng nảy thì HẠT THỨ TƯ cũng phải nảy, mà nó
không bao giờ nảy.
::
:::

:::opt
Máy báo lỗi khi chạy — `0.0` là một xác suất KHÔNG hợp lệ, xác suất
phải LỚN HƠN `0` mới tính được
::why
Gần đúng ở việc bạn nghĩ TỚI xác suất `0` như một trường hợp "đặc
biệt" cần xử lý riêng — một trực giác thận trọng.

Chỗ lệch: `0.0` LÀ một xác suất HOÀN TOÀN hợp lệ (bài 12: `P(∅)=0`,
biến cố chắc chắn không xảy ra) — Python nhân với `0.0` chạy sạch,
không có gì "không hợp lệ". Không cần xử lý riêng.
::
:::
::::

::::code{#viet_xac_suat_tat_ca}
Viết `xac_suat_tat_ca(danh_sach_xs)` — nhân dồn TOÀN BỘ xác suất
trong `danh_sach_xs`.

```python title=starter
def xac_suat_tat_ca(danh_sach_xs):
    ket_qua = 1
    for p in danh_sach_xs:
        ket_qua = ___
    return ket_qua


print(xac_suat_tat_ca([0.9, 0.8, 0.7, 0.6]))
```

```python title=solution
def xac_suat_tat_ca(danh_sach_xs):
    ket_qua = 1
    for p in danh_sach_xs:
        ket_qua = ket_qua * p
    return ket_qua


print(xac_suat_tat_ca([0.9, 0.8, 0.7, 0.6]))
```

```python title=test
assert xac_suat_tat_ca([]) == 1, "day rong -- khong phep thu nao, giu nguyen 1"
assert xac_suat_tat_ca([0.5]) == 0.5, "mot phep thu -- ket qua bang chinh xac suat do"
assert xac_suat_tat_ca([0.5, 0.5, 0.5, 0.5]) == 0.5 ** 4, "bon xac suat bang nhau -- khop luy thua"
assert xac_suat_tat_ca([0.9, 0.8, 0.7, 0.0]) == 0.0, "mot hat chac chan khong nay -- toan bo ve 0"
assert xac_suat_tat_ca([1.0, 1.0, 1.0]) == 1.0, "moi hat chac chan nay -- toan bo van chac chan"
```

:::hints
- kind: attention
  body: "Nhan don ket_qua voi p moi luot -- dung cau truc nhan don da co (bai 3)."
- kind: strategy
  body: "ket_qua * p"
- kind: one-line
  body: "ket_qua = ket_qua * p"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai nhan don ket_qua voi p moi luot
  requireAst:
  - kind: uses-operator, target: '*', min: 1
  - kind: uses-name, target: ket_qua, min: 1
  - kind: uses-name, target: p, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^0\.3024\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhân xác suất cho cả dãy cần MỌI biến cố ĐỘC LẬP. Nhưng nếu Byte ĐÃ
biết trước một hạt là giống mới — xác suất nó cũng là cà chua có
tính giống hệt như chưa biết gì không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Quy tắc nhân (bài này) cần các biến cố ĐỘC LẬP. Nhưng Byte ĐÃ rút
được một hạt và biết TRƯỚC nó là "giống mới" — xác suất hạt ĐÓ cũng
LÀ cà chua có còn tính bằng `P(\text{cà chua})` như cũ không, hay
phải tính LẠI vì đã có THÊM một thông tin?
::::

::::checkpoint{mastery=0.8}
::::
