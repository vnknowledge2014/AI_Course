---
id: toan.do-thi-modular-dai-so-truu-tuong.uoc-chung-lon-nhat
title: Ước chung lớn nhất
summary: "gcd(a,b) — số LỚN NHẤT chia HẾT cả a LẪN b; thuật toán Euclid: gcd(a,b) = gcd(b, a mod b), LẶP tới khi số DƯ LÀ 0 — MỖI bước THU NHỎ bài toán."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.gcd-euclid]
requires: [math.division-with-remainder, logic.loop-invariant]
concepts: [math.uoc-chung-lon-nhat, math.thuat-toan-euclid]
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
`gcd(48, 18)` — LẶP bằng TAY vài bước theo công thức `gcd(a,b) =
gcd(b, a mod b)`. Số LẦN lặp có NHỎ hơn hẳn số lần kiểm TỪNG ước số
một không?
::::

::::explain{#gcd-euclid}
NHỎ hơn RẤT nhiều. **`gcd(a,b)`** — số LỚN NHẤT chia HẾT cả `a` lẫn
`b`. **Thuật toán Euclid**: `gcd(a,b) = gcd(b, a mod b)`, LẶP tới
khi số DƯ LÀ `0` — MỖI bước THU NHỎ bài toán (T2.3: bất biến VÒNG
lặp, "điều đúng LẠI sau MỖI lượt" — Ở đây LÀ `gcd` KHÔNG đổi qua MỖI
bước, chỉ số CÀNG nhỏ dần):

```python title=readonly
def gcd(a, b):
    while b != 0:
        a, b = b, a % b
    return a


print(gcd(48, 18))
```

```text title=readonly
6
```

`48 mod 18=12` → `gcd(18,12)`; `18 mod 12=6` → `gcd(12,6)`; `12 mod
6=0` → dừng, trả VỀ `6`. BA bước — thu nhỏ NHANH.
::::

::::example{#dem_buoc}
So VỚI thử TỪNG ước một (TỪ `18` LÙI xuống `1`), Euclid nhanh HƠN
HẲN:

```python title=readonly
def gcd(a, b):
    while b != 0:
        a, b = b, a % b
    return a

def gcd_dem_buoc(a, b):
    buoc = 0
    while b != 0:
        a, b = b, a % b
        buoc += 1
    return (a, buoc)


print(gcd_dem_buoc(48, 18))
```

```text title=readonly
(6, 3)
```

Euclid tìm RA `gcd(48,18)=6` CHỈ sau `3` bước LẶP. Thử TỪNG ước MỘT
(kiểm `18, 17, 16,...` cho tới khi tìm ước CHUNG) mất tới `13` bước
mới CHẠM `6` — Euclid THU NHỎ bài toán theo cấp SỐ NHÂN (chia lấy
dư), thử ước CHỈ giảm theo cấp SỐ CỘNG (trừ dần từng đơn vị).
::::

::::predict{#doan-gcd-voi-so-0 commitOnce}
Byte tính `gcd(48, 0)` — MỘT trong hai số LÀ `0`:

```python
def gcd(a, b):
    while b != 0:
        a, b = b, a % b
    return a

print(gcd(48, 0))
```

Dòng cuối in ra gì?

:::opt{correct}
`48`
:::

:::opt
`0` — vì bất kỳ số nào NHÂN với `0` cũng RA `0`, VÀ ước CHUNG LỚN
nhất "phải" tuân THEO quy luật ĐÓ
::why
Gần đúng ở việc bạn LIÊN hệ tới quy luật "nhân VỚI 0 ra 0" — một
trực giác quen THUỘC TỪ phép NHÂN.

Chỗ lệch: `gcd` KHÔNG phải phép NHÂN — nó hỏi "số LỚN nhất chia HẾT
cả hai", VÀ MỌI số ĐỀU chia hết `0` (`0 = k×0` VỚI MỌI `k`). Vòng
lặp `while b != 0` kiểm `b=0` NGAY từ ĐẦU — ĐIỀU kiện SAI, vòng lặp
KHÔNG chạy LẦN nào, trả VỀ `a` NGUYÊN vẹn — `48`.
::
:::

:::opt
Máy báo lỗi khi chạy — dòng `a % b` cố CHIA `a` cho `b=0`, VÀ chia
CHO `0` LUÔN gây lỗi trong Python
::why
Gần đúng ở việc bạn nhớ ĐÚNG chia cho `0` gây lỗi — MỘT quy tắc THẬT
của Python (`ZeroDivisionError`).

Chỗ lệch: dòng `a % b` KHÔNG BAO GIỜ chạy khi `b=0` NGAY từ đầu, VÌ
`while b != 0` kiểm điều kiện TRƯỚC — `b=0` LÀM điều kiện SAI ngay,
vòng lặp bị BỎ QUA hoàn toàn, KHÔNG hề chạm tới `a % b`. Biên dịch
sạch, chạy sạch.
::
:::
::::

::::code{#viet_gcd}
Viết PHẦN CÒN LẠI của `gcd(a, b)` — MỖI bước, thay `(a, b)` bằng
`(b, a mod b)`.

```python title=starter
def gcd(a, b):
    while b != 0:
        ___
    return a


print(gcd(48, 18))
```

```python title=solution
def gcd(a, b):
    while b != 0:
        a, b = b, a % b
    return a


print(gcd(48, 18))
```

```python title=test
assert gcd(48, 18) == 6, "vi du chinh"
assert gcd(48, 0) == 48, "mot so la 0 -- gcd la so con lai"
assert gcd(0, 48) == 48, "so 0 o vi tri dau -- vong lap chay ngay, ra 48"
assert gcd(17, 5) == 1, "hai so nguyen to cung nhau -- gcd la 1"
assert gcd(12, 12) == 12, "hai so bang nhau -- gcd la chinh no"
```

:::hints
- kind: attention
  body: "Gan dong thoi: a nhan gia tri cu cua b, b nhan gia tri a mod b."
- kind: strategy
  body: "a, b = b, a % b"
- kind: one-line
  body: "a, b = b, a % b"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai gan dong thoi a, b = b, a % b
  requireAst:
  - kind: uses-operator, target: '%', min: 1
  - kind: gan-ten, target: b, min: 1
  - kind: uses-name, target: a, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^6\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`gcd` tìm ra số LỚN nhất chia HẾT cả hai. `3` CÓ "nghịch đảo" nào
khi CHIA modulo `7` — một số NHÂN vào ra ĐÚNG `1` không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`3` có nghịch đảo modulo `7` không? Thử NHÂN `3` VỚI từng số TỪ `1`
tới `6`, lấy dư CHO `7` — số NÀO cho kết quả `1`?
::::

::::checkpoint{mastery=0.8}
::::
