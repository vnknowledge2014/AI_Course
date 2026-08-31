---
id: khoa-hoc-may-tinh.may-chay-the-nao.moi-lenh-lam-dung-mot-viec-nho
title: "Mỗi lệnh chỉ làm ĐÚNG một việc nhỏ"
summary: "LOAD_FAST chỉ lấy giá trị ra, BINARY_OP chỉ làm một phép tính, RETURN_VALUE chỉ trả một kết quả — kể cả khi hai lệnh LẤY liền nhau in gộp thành một dòng LOAD_FAST_LOAD_FAST, mỗi việc lấy vẫn chỉ lấy đúng một giá trị."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.instruction-atomic]
requires: [may.bytecode]
concepts: [may.instruction-atomic]
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
Câu hỏi cuối bài trước, trả lời ngay: đúng một việc nhỏ, không hơn.
::::

::::explain{#dung-mot-viec}
Bài trước cho xem bốn, rồi sáu lệnh — nhưng chưa nói RANH GIỚI của một
lệnh nằm ở đâu. Ranh giới đó rất hẹp: mỗi lệnh máy chỉ được phép làm
đúng MỘT việc nhỏ.

`LOAD_FAST` chỉ lấy giá trị của MỘT biến cục bộ ra, đặt sẵn để dùng. Nó
không tính toán gì, không so sánh gì, không trả về đâu cả — chỉ lấy.

`BINARY_OP` chỉ làm đúng MỘT phép tính. Nó không tự đi lấy giá trị (đã
có lệnh khác lấy sẵn trước nó), và nó cũng không trả kết quả về cho
người gọi hàm — đó là việc của lệnh khác.

`RETURN_VALUE` chỉ trả MỘT kết quả. Nó không tính toán thêm gì, không
lấy giá trị nào mới — giá trị cần trả đã phải có sẵn từ trước.

Dòng nguồn `return a + b` đọc như MỘT câu, một hành động. Máy không thấy
vậy. Nó thấy ba việc tách rời, phải làm NỐI TIẾP nhau — lấy, tính, trả —
không việc nào được phép làm gộp việc của việc khác.

Một chỗ dễ hiểu lầm: bài trước bạn đã thấy dòng
`LOAD_FAST_LOAD_FAST 1 (a, b)` — MỘT dòng in ra, nhưng bên trong nó vẫn
là HAI việc lấy giá trị riêng biệt, đứng cạnh nhau nên được viết chung
cho gọn. Không có việc lấy nào "hiểu" được cả biểu thức `a + b` cùng
lúc — mỗi việc lấy vẫn chỉ lấy đúng MỘT giá trị, y hệt như khi nó đứng
một mình.
::::

::::example{#cong-hai-so}
```python title=readonly
import dis

def cong(a, b):
    return a + b

dis.dis(cong)
```

```text title=readonly
  3           RESUME                   0

  4           LOAD_FAST_LOAD_FAST      1 (a, b)
              BINARY_OP                0 (+)
              RETURN_VALUE
```

Ba dòng in ra, nhưng bốn việc thật đã xảy ra: lấy `a`, lấy `b` (gộp
chung một dòng vì đứng cạnh nhau), cộng, trả về. `return a + b` là một
câu — máy vẫn cần bốn bước, không rút gọn được bước nào.
::::

::::predict{#du-doan-gap-doi-roi-cong commitOnce}
```python
def gap_doi_roi_cong(a, b):
    return a * 2 + b
```

**Trước khi chạy** `dis.dis(gap_doi_roi_cong)`, bạn đoán dãy TÊN LỆNH
nào hiện ra (bỏ chi tiết trong ngoặc, chỉ tên lệnh, đúng thứ tự)?

:::opt{correct}
RESUME, LOAD_FAST, LOAD_CONST, BINARY_OP, LOAD_FAST, BINARY_OP,
RETURN_VALUE
:::

:::opt
RESUME, LOAD_FAST_LOAD_FAST, LOAD_CONST, BINARY_OP, BINARY_OP,
RETURN_VALUE
::why
Gần đúng ở việc bạn nhớ đúng: CÓ một lệnh gộp `LOAD_FAST_LOAD_FAST` tồn
tại trên máy này — bài trước bạn học đúng.

Chỗ lệch: lệnh gộp đó chỉ xảy ra khi HAI lệnh lấy biến cục bộ đứng NGAY
CẠNH NHAU trong dòng lệnh. Ở đây, giữa lúc lấy `a` và lúc lấy `b` có một
lệnh `LOAD_CONST` và một lệnh `BINARY_OP` xen vào — hai lệnh lấy ấy
không hề đứng cạnh nhau, nên không có gì để gộp.
::
:::

:::opt
RESUME, LOAD_FAST (b), LOAD_FAST (a), LOAD_CONST (2), BINARY_OP,
BINARY_OP, RETURN_VALUE
::why
Gần đúng ở việc bạn liệt kê đủ số lệnh cần có.

Chỗ lệch là thứ tự LẤY. Máy đọc và lấy giá trị từ TRÁI SANG PHẢI, đúng
như mã nguồn viết ra — `a` đứng trước dấu `*` nên được lấy trước; `b`
chỉ được lấy sau khi phép nhân `a * 2` đã xong. Độ ưu tiên toán tử (nhân
trước cộng) quyết định lệnh `BINARY_OP` nào CHẠY trước, không quyết
định thứ tự LẤY giá trị ra.
::
:::

:::opt
RESUME, LOAD_FAST, LOAD_CONST, LOAD_FAST, BINARY_OP, RETURN_VALUE
::why
Gần đúng ở việc bạn liệt kê đủ ba lệnh LẤY giá trị (`a`, `2`, `b`) đúng
thứ tự.

Chỗ lệch: `BINARY_OP` chỉ làm ĐÚNG một phép tính — nó không thể "nhân
rồi cộng" trong một lượt. Biểu thức `a * 2 + b` có HAI phép tính (nhân,
rồi cộng), nên cần đúng HAI lệnh `BINARY_OP`, không phải một.
::
:::
::::

::::code{#tinh-hoa-don}
Quán ăn tính hoá đơn: giá món nhân số lượng, cộng thêm phí giao hàng.
Điền chỗ trống, rồi để máy đếm giùm bạn số lệnh `BINARY_OP` thật sự
xuất hiện.

```python title=starter
import dis

def tinh_hoa_don(gia, so_luong, phi_ship):
    return ___                         # gia nhân so_luong, cộng phi_ship

print(tinh_hoa_don(20000, 3, 15000))
dis.dis(tinh_hoa_don)

so_lan_binary_op = sum(
    1 for lenh in dis.get_instructions(tinh_hoa_don)
    if lenh.opname == "BINARY_OP"
)
print("so_lan_binary_op:", so_lan_binary_op)
```

```python title=solution
import dis

def tinh_hoa_don(gia, so_luong, phi_ship):
    return gia * so_luong + phi_ship

print(tinh_hoa_don(20000, 3, 15000))
dis.dis(tinh_hoa_don)

so_lan_binary_op = sum(
    1 for lenh in dis.get_instructions(tinh_hoa_don)
    if lenh.opname == "BINARY_OP"
)
print("so_lan_binary_op:", so_lan_binary_op)
```

```python title=test
assert tinh_hoa_don(20000, 3, 15000) == 75000, f"20000*3 + 15000 phải ra 75000 — đang ra {tinh_hoa_don(20000, 3, 15000)}"
assert tinh_hoa_don(5000, 4, 2000) == 22000, f"5000*4 + 2000 phải ra 22000 — đang ra {tinh_hoa_don(5000, 4, 2000)}"
assert so_lan_binary_op == 2, f"hai phép tính (nhân rồi cộng) phải ra đúng hai lệnh BINARY_OP thật trong dis — đang đếm được {so_lan_binary_op}"
```

:::hints
- kind: attention
  body: Hai phép tính, không phải một — trước tiên NHÂN gia với so_luong, rồi CỘNG thêm phi_ship vào đúng kết quả đó.
- kind: strategy
  body: Viết gia * so_luong trước, rồi cộng phi_ship vào — dùng cả ba cái tên đã có, đừng gõ số cố định.
- kind: one-line
  body: 'Chỗ trống là: gia * so_luong + phi_ship'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải dùng cả ba tên gia, so_luong, phi_ship và đúng hai phép tính nhân, cộng — không gõ cứng kết quả
  requireAst:
  # Đã thử lời giải khác `phi_ship + gia * so_luong` (đổi chỗ số hạng
  # cộng) — vẫn qua cả static (mỗi tên đọc đúng 1 lần, mỗi toán tử đúng 1
  # lần) lẫn tests, vì phép cộng giao hoán. Đúng luật 2.
  - kind: uses-name, target: gia, min: 1
  - kind: uses-name, target: so_luong, min: 1
  - kind: uses-name, target: phi_ship, min: 1
  - kind: uses-operator, target: "*", min: 1
  - kind: uses-operator, target: "+", min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^75000\\n"
- tier: output
  expect: "so_lan_binary_op: 2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai phép tính viết trên một dòng nguồn, nhưng máy vẫn phải chạy từng
lệnh `BINARY_OP` một — không lệnh nào được nhảy cóc làm gộp việc của
lệnh kia. Đếm được đúng 2, không phải đoán.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`LOAD_FAST` ĐẨY một giá trị RA. Nhưng "ra" thì giá trị đó đi ĐÂU, trong
lúc chờ `BINARY_OP` tới lấy nó? Nó không quay lại nằm trong biến `gia`
hay `so_luong` — hai cái tên đó vẫn giữ nguyên giá trị của mình suốt
lúc tính, không hề mất đi. Vậy giá trị vừa lấy ra đang tạm nằm ở đâu?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
