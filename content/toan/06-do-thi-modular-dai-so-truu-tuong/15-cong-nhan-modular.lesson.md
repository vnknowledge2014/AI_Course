---
id: toan.do-thi-modular-dai-so-truu-tuong.cong-nhan-modular
title: Cộng, nhân modular
summary: "(a+b) mod n = ((a mod n)+(b mod n)) mod n, TƯƠNG TỰ cho nhân — CỘNG/NHÂN rồi LẤY dư CHO kết quả GIỐNG hệt lấy dư TRƯỚC rồi cộng/nhân; hữu ích khi SỐ quá LỚN."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.modular-arithmetic]
requires: [math.congruence-modulo]
concepts: [math.cong-modular, math.nhan-modular]
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
Byte tính ngày thứ `1000` rơi VÀO ngày NÀO (chu kỳ BA ngày) — nhân
TRỰC TIẾP một số LỚN có TIỆN bằng lấy dư TRƯỚC rồi mới nhân KHÔNG?
::::

::::explain{#cong-nhan-modular}
TIỆN — VÀ ra CÙNG kết quả. **`(a+b) mod n = ((a mod n)+(b mod n))
mod n`**, TƯƠNG TỰ cho NHÂN — CỘNG/NHÂN rồi LẤY dư CHO kết quả GIỐNG
HỆT lấy dư TRƯỚC rồi cộng/nhân; hữu ích khi số quá LỚN (không cần
tính số ĐẦY ĐỦ trước khi lấy dư):

```python title=readonly
def nhan_modular(a, b, n):
    return (a * b) % n


print(nhan_modular(37, 26, 3))
print(((37 % 3) * (26 % 3)) % 3)
```

```text title=readonly
2
2
```

Nhân TRỰC TIẾP (`37×26=962`, RỒI lấy dư) VÀ lấy dư TRƯỚC RỒI mới
nhân (`1×2=2`, dư SẴN rồi) — CÙNG ra `2`. Với số CÀNG LỚN, cách THỨ
HAI CÀNG tiện (giữ số LUÔN nhỏ, không cần nhân số khổng lồ).
::::

::::example{#cong-modular}
ĐÚNG TƯƠNG TỰ cho phép CỘNG:

```python title=readonly
def cong_modular(a, b, n):
    return (a + b) % n


print(cong_modular(1000, 2000, 3))
print(((1000 % 3) + (2000 % 3)) % 3)
```

```text title=readonly
0
0
```

`1000+2000=3000`, RỒI lấy dư (`0`) — hoặc lấy dư TRƯỚC (`1+2=3`,
RỒI dư TIẾP LẦN NỮA cho `3 mod 3 = 0`) — CÙNG kết quả.
::::

::::predict{#doan-hai-cach-cong commitOnce}
Byte thử HAI cách TÍNH `(157 + 289) mod 4` — cộng TRỰC TIẾP RỒI lấy
dư, VÀ lấy dư TỪNG số TRƯỚC rồi mới cộng:

```python
def cong_modular(a, b, n):
    return (a + b) % n

print(cong_modular(157, 289, 4))
print(((157 % 4) + (289 % 4)) % 4)
```

Dòng cuối in ra gì SO VỚI dòng TRƯỚC?

:::opt{correct}
Hai dòng in RA CÙNG một số
:::

:::opt
Hai dòng in RA số KHÁC nhau — quy tắc "lấy dư trước" CHỈ đúng khi
`n` LÀ một số NGUYÊN TỐ (như `3` Ở ví dụ TRƯỚC), CÒN `4` KHÔNG phải
số nguyên TỐ nên quy tắc KHÔNG áp dụng được
::why
Gần đúng ở việc bạn để ý ĐÚNG `4` KHÔNG phải số nguyên TỐ (`4=2×2`)
— một quan sát VỀ tính chất SỐ học chính xác.

Chỗ lệch: đẳng thức `(a+b) mod n = ((a mod n)+(b mod n)) mod n`
đúng VỚI **MỌI** `n>0`, KHÔNG PHÂN biệt nguyên tố HAY hợp số — nó
LÀ hệ QUẢ của phép chia có DƯ (bài 13), KHÔNG liên quan tính nguyên
TỐ. Số nguyên TỐ chỉ QUAN trọng cho NGHỊCH đảo modular (bài 17),
KHÔNG cho cộng/nhân modular.
::
:::

:::opt
Máy báo lỗi khi chạy — `((157 % 4) + (289 % 4)) % 4` lồng BA cặp
ngoặc TRÒN VÀO nhau, Python giới hạn ĐỘ sâu lồng ngoặc trong MỘT
biểu thức
::why
Gần đúng ở việc bạn để ý ĐÚNG biểu thức CÓ nhiều lớp ngoặc lồng NHAU
— một quan sát VỀ hình thức đúng.

Chỗ lệch: Python KHÔNG giới hạn ĐỘ sâu lồng ngoặc theo cách THỰC tế
hay gặp (ba lớp LÀ RẤT nông) — biểu thức tính ĐÚNG thứ tự TỪ trong
ra ngoài, KHÔNG lỗi gì. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_nhan_modular}
Viết `nhan_modular(a, b, n)` — tính `(a×b) mod n`.

```python title=starter
def nhan_modular(a, b, n):
    return ___


print(nhan_modular(37, 26, 3))
```

```python title=solution
def nhan_modular(a, b, n):
    return (a * b) % n


print(nhan_modular(37, 26, 3))
```

```python title=test
assert nhan_modular(37, 26, 3) == 2, "37x26=962, 962 mod 3 = 2"
assert nhan_modular(37, 26, 3) == ((37 % 3) * (26 % 3)) % 3, "nhan truc tiep phai khop nhan qua mod truoc"
assert nhan_modular(0, 5, 4) == 0, "nhan voi 0 -- luon 0"
assert nhan_modular(157, 289, 4) == ((157 % 4) * (289 % 4)) % 4, "dung voi n khong nguyen to"
```

:::hints
- kind: attention
  body: "Nhan a voi b truoc, roi lay du cho n -- dung * va %."
- kind: strategy
  body: "(a * b) % n"
- kind: one-line
  body: "___ = (a * b) % n"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai nhan a voi b roi lay du cho n
  requireAst:
  - kind: uses-operator, target: '*', min: 1
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
Cộng, nhân modular — LUÔN khớp dù lấy dư TRƯỚC hay SAU. Tìm ước
CHUNG lớn nhất của hai số — có cách NÀO nhanh hơn thử TỪNG ước một
không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`gcd(48, 18)` — LẶP bằng TAY vài bước theo công thức `gcd(a,b) =
gcd(b, a mod n)`. Số LẦN lặp có NHỎ hơn hẳn số lần kiểm TỪNG ước số
một không?
::::

::::checkpoint{mastery=0.8}
::::
