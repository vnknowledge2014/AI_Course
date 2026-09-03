---
id: toan.do-thi-modular-dai-so-truu-tuong.dong-ho-modular-tong-quat
title: Đồng hồ modular tổng quát
summary: "Modular LÀ một 'đồng hồ' n giờ — CỘNG/nhân rồi 'QUAY VÒNG' khi vượt n; đồng hồ 12 giờ, lịch tưới BA ngày, mã Caesar (bài 19) ĐỀU LÀ cùng MỘT cấu trúc với n KHÁC nhau — tổng kết cụm, chỉ NHÌN LẠI qua MỘT lăng kính chung."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.generalized-modular-arithmetic]
requires: [math.caesar-cipher, math.modular-arithmetic, math.modular-inverse]
concepts: [math.dong-ho-modular]
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
Đồng hồ `12` giờ, lịch tưới BA ngày, mã Caesar `26` chữ — CẢ BA đều
"cộng RỒI quay vòng". Ba thứ NÀY có THẬT SỰ LÀ MỘT cấu trúc, CHỈ
khác con số `n`?
::::

::::explain{#dong-ho-modular}
CÓ. **Modular LÀ một "đồng hồ" `n` giờ** — CỘNG rồi "QUAY VÒNG" khi
VƯỢT `n`. Đồng hồ `12` giờ, lịch tưới BA ngày, mã Caesar `26` chữ
(bài 19) ĐỀU LÀ CÙNG một cấu trúc VỚI `n` khác nhau — KHÔNG khái
niệm TÍNH toán mới, CHỈ nhìn LẠI qua MỘT lăng kính CHUNG:

```python title=readonly
def dong_ho(a, b, n):
    return (a + b) % n


print(dong_ho(9, 5, 12))
```

```text title=readonly
2
```

`9` giờ, THÊM `5` tiếng NỮA — `14` giờ, NHƯNG đồng hồ `12` giờ QUAY
VÒNG về `2` giờ. Y HỆT phép cộng modular (bài 15), CHỈ đổi `n=12`
thay VÌ `n=3`.
::::

::::example{#cung-mot-ham-cho-ca-caesar}
CÙNG hàm `dong_ho` ĐÓ tái tạo LẠI y HỆT mã Caesar (bài 19), CHỈ đổi
`n=26`:

```python title=readonly
def dong_ho(a, b, n):
    return (a + b) % n

def vi_tri(chu):
    return ord(chu) - ord('A')

def chu_tu_vi_tri(v):
    return chr(v + ord('A'))

def ma_hoa_qua_dong_ho(chu, k):
    return chu_tu_vi_tri(dong_ho(vi_tri(chu), k, 26))


print(ma_hoa_qua_dong_ho('Z', 3))
```

```text title=readonly
C
```

`vi_tri('Z')=25`, `dong_ho(25,3,26)=2`, `chu_tu_vi_tri(2)='C'` — Y
HỆT `ma_hoa('Z',3)='C'` Ở bài 19, CHỈ viết LẠI QUA `dong_ho` VỚI
`n=26`. BA bài toán (đồng hồ, lịch tưới, mã Caesar) — MỘT hàm DUY
NHẤT, khác nhau đúng MỘT con số.
::::

::::predict{#doan-dong-ho-lui commitOnce}
Byte dùng `dong_ho` để tính "`2` giờ, LÙI lại `5` tiếng" — TRUYỀN
`b` ÂM:

```python
def dong_ho(a, b, n):
    return (a + b) % n

print(dong_ho(2, -5, 12))
```

Dòng cuối in ra gì?

:::opt{correct}
`9`
:::

:::opt
`-3` — vì `2 - 5 = -3`, VÀ đồng hồ CHỈ "quay VÒNG" khi CỘNG thêm,
KHÔNG áp dụng cho phép LÙI (số ÂM)
::why
Gần đúng ở việc bạn tính ĐÚNG `2-5=-3` — một phép TRỪ chính xác.

Chỗ lệch: `%` trong Python KHÔNG dừng Ở `-3` — nó LUÔN trả VỀ kết
quả trong khoảng `[0, n)` khi `n` dương (đã thấy Ở bài 19's predict:
`giai_ma`). Phần dư của `-3` chia CHO `12` LÀ `9` — đồng hồ QUAY
VÒNG cho CẢ chiều LÙI, y hệt chiều tiến: "`2` giờ, lùi `5` tiếng"
LÀ `9` giờ (đi ngược kim đồng hồ, VÒNG qua `12`).
::
:::

:::opt
Máy báo lỗi khi chạy — `dong_ho` được ĐỊNH nghĩa VỚI tham số `b`
DƯƠNG trong TẤT CẢ ví dụ TRƯỚC, truyền `b` ÂM VI PHẠM kiểu DỮ liệu
mà hàm mong đợi
::why
Gần đúng ở việc bạn để ý ĐÚNG mọi ví dụ TRƯỚC ĐỀU dùng `b` dương —
một quan sát VỀ MẪU hình sử dụng.

Chỗ lệch: `dong_ho` KHÔNG hề "mong đợi" `b` dương — Python KHÔNG có
kiểu DỮ liệu RIÊNG cho số âm/dương, VÀ hàm hoạt ĐỘNG bình thường VỚI
BẤT KỲ số nguyên nào (đã thấy Ở bài 19, `giai_ma` truyền `-k`).
Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_dong_ho}
Viết `dong_ho(a, b, n)` — cộng `a` VỚI `b` RỒI "quay VÒNG" trong
`n`.

```python title=starter
def dong_ho(a, b, n):
    return ___


print(dong_ho(9, 5, 12))
```

```python title=solution
def dong_ho(a, b, n):
    return (a + b) % n


print(dong_ho(9, 5, 12))
```

```python title=test
assert dong_ho(9, 5, 12) == 2, "dong ho 12h -- 9+5 quay vong ve 2"
assert dong_ho(2, 3, 3) == 2, "lich tuoi 3 ngay -- di dung mot chu ky, ve lai vi tri cu"
assert dong_ho(0, 3, 26) == 3, "caesar mod 26 -- A dich 3 ra vi tri 3 (D)"
assert dong_ho(2, -5, 12) == 9, "dong ho lui -- 2 gio lui 5 tieng la 9 gio"
assert dong_ho(0, 0, 5) == 0, "cong 0 -- khong doi"
```

:::hints
- kind: attention
  body: "Cong a voi b, roi lay du cho n -- dung + va %."
- kind: strategy
  body: "(a + b) % n"
- kind: one-line
  body: "___ = (a + b) % n"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai cong a voi b roi lay du cho n
  requireAst:
  - kind: uses-operator, target: '+', min: 1
  - kind: uses-operator, target: '%', min: 1
  - kind: uses-name, target: b, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^2\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
MỘT hàm `dong_ho`, ba bài toán. Modular cộng trên tập `{0,1,...,n−1}`
— kết quả có LUÔN nằm TRONG tập đó không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Modular cộng (bài 15) trên tập `{0,1,...,n−1}` — kết quả CÓ LUÔN nằm
TRONG tập đó không (ĐÓNG), hay CÓ thể "chạy RA ngoài"?
::::

::::checkpoint{mastery=0.8}
::::
