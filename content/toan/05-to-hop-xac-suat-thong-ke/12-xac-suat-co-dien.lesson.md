---
id: toan.to-hop-xac-suat-thong-ke.xac-suat-co-dien
title: Xác suất cổ điển
summary: "P(A) = |A| / |Ω| khi mọi kết quả trong Ω đều ĐỀU khả năng (mỗi hạt như nhau, không hạt nào dễ rút hơn) — biến độ may rủi thành một con số giữa 0 và 1."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.classical-probability]
requires: [math.sample-space-event]
concepts: [math.xac-suat-co-dien]
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
Túi có 10 hạt: 5 cà chua, 3 xà lách, 2 cà rốt. Rút cà chua "dễ" hơn
rút cà rốt — nhưng "dễ hơn" NHIỀU cỡ nào, tính bằng con số nào?
::::

::::explain{#xac-suat-co-dien-la-gi}
**Xác suất cổ điển `P(A) = |A| / |Ω|`** — khi MỌI kết quả trong `Ω`
đều ĐỀU khả năng (mỗi hạt như nhau, không hạt nào "dễ rút hơn"), tỉ
lệ SỐ kết quả THUỘC `A` trên TỔNG số kết quả biến ĐỘ MAY RỦI thành
một con số giữa `0` và `1`:

```python title=readonly
def la_xac_suat(a, omega):
    return len(a) / len(omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}

print(la_xac_suat(ca_chua, omega))
```

```text title=readonly
0.5
```

`P(ca_chua) = 5/10 = 0.5` — rút được cà chua với XÁC SUẤT `0.5`, tức
"một nửa các hạt" LÀ cà chua. Con số CÀNG gần `1` thì biến cố CÀNG
"dễ" xảy ra; càng gần `0` thì càng "khó".
::::

::::example{#hai-dau-mut}
Hai đầu MÚT: `P(Ω) = 1` (chắc chắn xảy ra) VÀ `P(∅) = 0` (chắc chắn
KHÔNG xảy ra):

```python title=readonly
def la_xac_suat(a, omega):
    return len(a) / len(omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}

print(la_xac_suat(omega, omega))
print(la_xac_suat(set(), omega))
```

```text title=readonly
1.0
0.0
```

Rút MỘT hạt BẤT KỲ thì LUÔN thuộc `omega` (`P(Ω)=1`, chắc chắn xảy
ra). Không hạt nào thuộc tập RỖNG (`P(∅)=0`, chắc chắn không xảy
ra). MỌI xác suất khác nằm GIỮA hai đầu mút này.
::::

::::predict{#doan-cong-tat-ca-xac-suat commitOnce}
Byte tính xác suất CẢ BA loại rồi CỘNG lại:

```python
def la_xac_suat(a, omega):
    return len(a) / len(omega)

omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
xa_lach = {"h6", "h7", "h8"}
ca_rot = {"h9", "h10"}

print(la_xac_suat(ca_chua, omega) + la_xac_suat(xa_lach, omega) + la_xac_suat(ca_rot, omega))
```

Dòng cuối in ra gì?

:::opt{correct}
`1.0`
:::

:::opt
`0.5` — vì ba loại rau khác nhau thì xác suất PHẢI chia đều, và giá
trị TRUNG BÌNH của ba phần (không phải TỔNG) mới là con số có Ý
NGHĨA
::why
Gần đúng ở việc bạn nghĩ TỚI một cách TÓM TẮT ba con số bằng MỘT con
số — một thao tác hợp lý (sẽ học kỹ hơn Ở phần thống kê, bài 27).

Chỗ lệch: dòng code TRÊN CỘNG THẲNG ba xác suất, KHÔNG chia cho `3`
— đó LÀ phép TỔNG, không phải phép TRUNG BÌNH. `0.5+0.3+0.2=1.0`,
không phải `0.5`. (Trung bình BA con số này mới là `1.0/3 ≈ 0.33`,
một câu hỏi KHÁC hẳn.)
::
:::

:::opt
Máy báo lỗi khi chạy — cộng ba số THẬP PHÂN (không phải số nguyên)
bằng `+` là một phép toán KHÔNG hợp lệ trong Python
::why
Gần đúng ở việc bạn để ý CẢ BA con số đều LÀ số thập phân (`0.5`,
`0.3`, `0.2`) — quan sát ĐÓ đúng VỀ KIỂU dữ liệu.

Chỗ lệch: Python CỘNG số thập phân (kiểu `float`) hoàn toàn bình
thường bằng `+`, giống hệt số nguyên — không có gì "không hợp lệ".
Biên dịch sạch, chạy sạch, ra đúng `1.0`.
::
:::
::::

::::code{#viet_la_xac_suat}
Viết `la_xac_suat(a, omega)` — tính `P(a) = |a| / |omega|`.

```python title=starter
def la_xac_suat(a, omega):
    return ___


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}

print(la_xac_suat(ca_chua, omega))
```

```python title=solution
def la_xac_suat(a, omega):
    return len(a) / len(omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}

print(la_xac_suat(ca_chua, omega))
```

```python title=test
assert la_xac_suat(omega, omega) == 1.0, "P(omega) = 1"
assert la_xac_suat(set(), omega) == 0.0, "P(rong) = 0"
xa_lach = {"h6", "h7", "h8"}
assert la_xac_suat(xa_lach, omega) == 0.3, "3/10 = 0.3"
ca_rot = {"h9", "h10"}
assert la_xac_suat(ca_rot, omega) == 0.2, "2/10 = 0.2"
```

:::hints
- kind: attention
  body: "Chia len(a) cho len(omega), dung phep chia thuc / (khong phai chia nguyen //)."
- kind: strategy
  body: "len(a) / len(omega)"
- kind: one-line
  body: "___ = len(a) / len(omega)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai chia len(a) cho len(omega) bang phep chia thuc /
  requireAst:
  - kind: uses-call, target: len, min: 2
  - kind: uses-operator, target: '/', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^0\.5\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba xác suất cộng lại đúng `1.0`. Có cách nào tính "KHÔNG rút được cà
chua" mà không cần cộng hai loại còn lại?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`P(cà chua) = 0.5`, `P(xà lách) = 0.3`, `P(cà rốt) = 0.2`. Cộng ba
con số lại — `1.0`. Trùng hợp, hay `P(A)` của MỌI kết quả CÓ THỂ
trong `Ω` LUÔN cộng lại đúng 1, và VÌ SAO?
::::

::::checkpoint{mastery=0.8}
::::
