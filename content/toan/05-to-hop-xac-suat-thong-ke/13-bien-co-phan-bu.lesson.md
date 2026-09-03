---
id: toan.to-hop-xac-suat-thong-ke.bien-co-phan-bu
title: Biến cố phần bù
summary: "P(A') = 1 − P(A) — phần bù của biến cố (T2.4 bài 12, Aᶜ = Ω − A) có xác suất bằng phần CÒN LẠI của 1."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.complement-probability]
requires: [math.classical-probability]
concepts: [math.bien-co-phan-bu]
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
`P(cà chua) = 0.5`. Xác suất "KHÔNG rút được cà chua" — có cần cộng
xà lách với cà rốt lại mới ra không?
::::

::::explain{#phan-bu-la-gi}
Không cần. **`P(A') = 1 − P(A)`** — phần bù của biến cố (T2.4 bài
12, `Aᶜ = Ω − A`) có xác suất bằng phần CÒN LẠI của `1`:

```python title=readonly
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_phan_bu(a, omega):
    return 1 - la_xac_suat(a, omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}

print(xac_suat_phan_bu(ca_chua, omega))
```

```text title=readonly
0.5
```

`P(\text{không cà chua}) = 1 − 0.5 = 0.5` — không cần biết XÀ LÁCH
hay CÀ RỐT là bao nhiêu phần trăm riêng, chỉ cần LẤY `1` trừ đi xác
suất cà chua.
::::

::::example{#kiem-lai-bang-cong-thang}
Kiểm lại BẰNG cách cộng thẳng hai loại còn lại — hai cách RA cùng
một số:

```python title=readonly
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_phan_bu(a, omega):
    return 1 - la_xac_suat(a, omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
xa_lach = {"h6", "h7", "h8"}
ca_rot = {"h9", "h10"}

print(xac_suat_phan_bu(ca_chua, omega))
print(la_xac_suat(xa_lach, omega) + la_xac_suat(ca_rot, omega))
```

```text title=readonly
0.5
0.5
```

Hai cách RA cùng `0.5`. Cách trừ (bài này) NHANH hơn khi biến cố
phần bù GỘP NHIỀU loại khác (không cần liệt kê TỪNG loại rồi cộng
tay); cách cộng chỉ tiện khi CHỈ có vài loại rõ ràng.
::::

::::predict{#doan-phan-bu-hai-dau-mut commitOnce}
Byte tính phần bù CỦA hai biến cố đặc biệt: chính `omega` (mọi kết
quả) VÀ tập rỗng (không kết quả nào):

```python
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_phan_bu(a, omega):
    return 1 - la_xac_suat(a, omega)

omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}

print(xac_suat_phan_bu(omega, omega))
print(xac_suat_phan_bu(set(), omega))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`0.0`, rồi `1.0`
:::

:::opt
`1.0`, rồi `0.0` — vì `omega` LÀ tập LỚN NHẤT có thể (xác suất cao
nhất), nên phần bù của nó cũng phải LÀ số CAO nhất
::why
Gần đúng ở việc bạn nhớ ĐÚNG `omega` là biến cố "chắc chắn xảy ra"
(`P(omega)=1`, bài 12) — một sự thật đúng.

Chỗ lệch: PHẦN BÙ đi NGƯỢC chiều với biến cố gốc, không CÙNG chiều —
`P(omega)=1` (chắc chắn xảy ra) thì phần bù của nó `P(omega')` PHẢI
là `1−1=0` (chắc chắn KHÔNG xảy ra, đúng: "không rút được kết quả
nào NGOÀI omega" là điều LUÔN đúng, tầm thường). Ngược lại, `P(∅)=0`
(chắc chắn không xảy ra) thì phần bù `P(∅')=1−0=1` (chắc chắn xảy
ra — "rút được một kết quả KHÔNG thuộc tập rỗng" luôn đúng).
::
:::

:::opt
Máy báo lỗi khi chạy — `xac_suat_phan_bu(omega, omega)` tính phần bù
của CHÍNH `omega` so với `omega`, Python từ chối phép trừ MỘT tập
cho CHÍNH nó
::why
Gần đúng ở việc bạn để ý `omega` xuất hiện Ở CẢ HAI đối số — một
quan sát đúng VỀ hình thức lời gọi.

Chỗ lệch: `xac_suat_phan_bu(a, omega)` KHÔNG hề "trừ tập cho chính
nó" — nó tính `1 − len(a)/len(omega)`, một phép TRỪ giữa hai SỐ (một
số nguyên `1` và một phân số), hoàn toàn hợp lệ dù `a` và `omega`
trùng nhau hay không. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_xac_suat_phan_bu}
Viết `xac_suat_phan_bu(a, omega)` — tính `P(a') = 1 − P(a)`.

```python title=starter
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_phan_bu(a, omega):
    return ___


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}

print(xac_suat_phan_bu(ca_chua, omega))
```

```python title=solution
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_phan_bu(a, omega):
    return 1 - la_xac_suat(a, omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}

print(xac_suat_phan_bu(ca_chua, omega))
```

```python title=test
assert xac_suat_phan_bu(omega, omega) == 0.0, "phan bu cua omega la 0"
assert xac_suat_phan_bu(set(), omega) == 1.0, "phan bu cua rong la 1"
xa_lach = {"h6", "h7", "h8"}
assert xac_suat_phan_bu(xa_lach, omega) == 0.7, "1 - 0.3 = 0.7"
ca_rot = {"h9", "h10"}
assert xac_suat_phan_bu(ca_rot, omega) == 0.8, "1 - 0.2 = 0.8"
```

:::hints
- kind: attention
  body: "Lay 1 tru di la_xac_suat(a, omega), dung ham da co."
- kind: strategy
  body: "1 - la_xac_suat(a, omega)"
- kind: one-line
  body: "___ = 1 - la_xac_suat(a, omega)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai lay 1 tru di la_xac_suat(a, omega)
  requireAst:
  - kind: uses-call, target: la_xac_suat, min: 1
  - kind: uses-operator, target: '-', min: 1
  - kind: has-literal, target: '1'
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^0\.5\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Phần bù xong. Nhưng "cà chua" và "xà lách" — rời nhau — cộng thẳng
xác suất được không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`P(\text{cà chua hoặc xà lách})` — hai biến cố RỜI NHAU (một hạt
không thể vừa là cà chua vừa là xà lách). Cộng thẳng `P(\text{cà
chua}) + P(\text{xà lách})` có ra đúng xác suất "rút được cà chua
HOẶC xà lách" không?
::::

::::checkpoint{mastery=0.8}
::::
