---
id: toan.do-thi-modular-dai-so-truu-tuong.ma-hoa-caesar
title: Mã hoá Caesar
summary: "Mã Caesar — DỊCH mỗi chữ cái ĐI k VỊ TRÍ trong bảng chữ cái, LẶP VÒNG khi qua hết (đúng modular, bài 14): (chu+k) mod 26; GIẢI mã LÀ dịch NGƯỢC −k — ứng dụng THẬT đầu tiên của cả cụm modular."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.caesar-cipher]
requires: [math.modular-arithmetic, math.congruence-modulo]
concepts: [math.ma-hoa-caesar]
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
Mã hoá "A" dịch `3` vị trí RA "D". Mã hoá "Z" (chữ CUỐI bảng) dịch
`3` vị trí RA chữ NÀO — có "chạy QUÁ" bảng chữ cái không?
::::

::::explain{#ma-hoa-caesar}
KHÔNG "chạy quá" — MODULAR tự đưa nó QUAY lại đầu. **Mã Caesar** —
DỊCH mỗi chữ cái ĐI `k` VỊ TRÍ trong bảng chữ cái, LẶP VÒNG khi QUA
hết (đúng modular, bài 14): `(chu+k) mod 26`; **giải mã** LÀ dịch
NGƯỢC `−k`:

```python title=readonly
def ma_hoa(chu, k):
    return chr((ord(chu) - ord('A') + k) % 26 + ord('A'))


print(ma_hoa('Z', 3))
```

```text title=readonly
C
```

`Z` LÀ chữ THỨ `25` (đếm từ `0`, `A=0`). `25+3=28`, `28 mod 26 = 2`
— chữ thứ `2` LÀ `C`. Bảng chữ cái "QUAY VÒNG": SAU `Z` LẠI tới `A`,
`B`, `C` — đúng LỐI đồng hồ MODULAR (bài 15's reflect).
::::

::::example{#ma-hoa-mot-tu}
Mã hoá CẢ một từ — MỖI chữ dịch RIÊNG:

```python title=readonly
def ma_hoa(chu, k):
    return chr((ord(chu) - ord('A') + k) % 26 + ord('A'))


thong_diep = "BYTE"
ma = "".join(ma_hoa(c, 3) for c in thong_diep)

print(ma)
```

```text title=readonly
EBWH
```

`B→E`, `Y→B` (quay VÒNG), `T→W`, `E→H` — MỖI chữ dịch ĐỘC LẬP `3`
vị trí, VÒNG lại khi hết bảng chữ cái. `"BYTE"` mã hoá thành
`"EBWH"`.
::::

::::predict{#doan-giai-ma commitOnce}
Byte GIẢI mã chữ `'C'` với `k=3` (dịch NGƯỢC `3` vị trí):

```python
def ma_hoa(chu, k):
    return chr((ord(chu) - ord('A') + k) % 26 + ord('A'))

def giai_ma(chu, k):
    return ma_hoa(chu, -k)

print(giai_ma('C', 3))
```

Dòng cuối in ra gì?

:::opt{correct}
`Z`
:::

:::opt
`X` — vì GIẢI mã LÀ dịch NGƯỢC `3` vị trí TỪ `C`, VÀ đếm NGƯỢC ba
bước TRONG bảng chữ cái THÔNG thường (`C→B→A→...`) sẽ RA `X` sau khi
"mượn" TỪ đầu bảng
::why
Gần đúng ở việc bạn cố đếm NGƯỢC thủ CÔNG TỪNG bước — một cách tiếp
cận HỢP lý VỀ nguyên tắc.

Chỗ lệch: đếm NGƯỢC ba bước TỪ `C` (vị TRÍ `2`) cho RA `-1`, VÀ
modular XỬ LÝ số ÂM đúng CÁCH: trong Python, phần dư của `-1` chia
CHO `26` LÀ `25`, KHÔNG phải `-1` — vị trí `25` LÀ `Z`, không phải
`X`. `giai_ma` GỌI `ma_hoa(chu, -k)`,
TÁI dùng chính công thức `(vị_trí + k) mod 26` VỚI `k` ÂM — modular
tự XỬ lý việc "mượn" đúng, không cần đếm tay.
::
:::

:::opt
Máy báo lỗi khi chạy — `ma_hoa(chu, -k)` truyền một số ÂM (`-3`) LÀM
đối số `k`, mà `ma_hoa` chỉ được THIẾT kế cho `k` DƯƠNG (dịch XUÔI)
::why
Gần đúng ở việc bạn để ý ĐÚNG `-k` LÀ một GIÁ trị ÂM — một quan sát
VỀ dấu của SỐ.

Chỗ lệch: `ma_hoa` HOẠT động ĐÚNG với `k` ÂM — CÔNG thức `(... + k)
% 26` xử LÝ số âm HOÀN TOÀN bình thường TRONG Python (`%` LUÔN trả
VỀ kết quả KHÔNG âm khi số CHIA dương). KHÔNG cần thiết kế RIÊNG cho
`k` âm — CHÍNH sự tổng QUÁT này LÀ lý do `giai_ma` tái DÙNG được
`ma_hoa`. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_ma_hoa}
Viết `ma_hoa(chu, k)` — dịch một chữ CÁI (in hoa) ĐI `k` vị trí,
LẶP vòng khi qua HẾT bảng chữ cái.

```python title=starter
def ma_hoa(chu, k):
    return ___


print(ma_hoa('A', 3))
```

```python title=solution
def ma_hoa(chu, k):
    return chr((ord(chu) - ord('A') + k) % 26 + ord('A'))


print(ma_hoa('A', 3))
```

```python title=test
assert ma_hoa('A', 3) == 'D', "A dich 3 -> D"
assert ma_hoa('Z', 3) == 'C', "Z dich 3, quay vong -> C"
assert ma_hoa('Y', 3) == 'B', "Y dich 3, quay vong -> B"
assert ma_hoa('A', 0) == 'A', "dich 0 -- khong doi"
assert ma_hoa('C', -3) == 'Z', "dich am 3 (giai ma) -- C ve Z"
thong_diep = "BYTE"
ma = "".join(ma_hoa(c, 3) for c in thong_diep)
assert ma == "EBWH", "ma hoa ca tu BYTE"
```

:::hints
- kind: attention
  body: "Doi chu sang vi tri so (ord(chu)-ord('A')), cong k, lay du 26, roi doi nguoc lai thanh chu (cong ord('A'), dua vao chr)."
- kind: strategy
  body: "chr((ord(chu) - ord('A') + k) % 26 + ord('A'))"
- kind: one-line
  body: "___ = chr((ord(chu) - ord('A') + k) % 26 + ord('A'))"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai doi chu sang vi tri so, cong k, lay du 26, roi doi lai thanh chu
  requireAst:
  - kind: uses-call, target: chr, min: 1
  - kind: uses-call, target: ord, min: 2
  - kind: uses-operator, target: '%', min: 1
  - kind: uses-operator, target: '-', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^D\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mã Caesar — ứng dụng THẬT đầu tiên của modular. Đồng hồ 12 giờ, lịch
tưới ba ngày, mã Caesar 26 chữ — CẢ BA đều "cộng RỒI quay vòng".
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đồng hồ 12 giờ, lịch tưới BA ngày, mã Caesar 26 chữ — CẢ BA đều
"cộng RỒI quay vòng". Phép CỘNG modular NÀY có tính chất NÀO GIỐNG
phép cộng số nguyên THƯỜNG (kết hợp? có đơn vị? có nghịch đảo?), và
tính chất NÀO thì KHÔNG?
::::

::::checkpoint{mastery=0.8}
::::
