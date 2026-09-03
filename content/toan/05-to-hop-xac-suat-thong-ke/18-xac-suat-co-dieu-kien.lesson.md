---
id: toan.to-hop-xac-suat-thong-ke.xac-suat-co-dieu-kien
title: Xác suất có điều kiện
summary: "P(A|B) = P(A∩B) / P(B) — xác suất của A, BIẾT TRƯỚC B đã xảy ra (thu hẹp Ω xuống còn đúng B)."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.conditional-probability]
requires: [math.independence-chain]
concepts: [math.xac-suat-co-dieu-kien]
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
Byte ĐÃ biết hạt vừa rút là "giống mới". Biết TRƯỚC điều đó có đổi
xác suất nó CŨNG là cà chua không?
::::

::::explain{#dieu-kien-la-gi}
Có thể đổi. **`P(A|B) = P(A∩B) / P(B)`** — xác suất của `A`, BIẾT
TRƯỚC `B` đã xảy ra: THU HẸP không gian mẫu `Ω` xuống còn ĐÚNG `B`
(chỉ những kết quả CÒN khả dĩ SAU khi biết `B`), rồi hỏi tỉ lệ `A`
trong phần thu hẹp ĐÓ:

```python title=readonly
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_dieu_kien(a, b, omega):
    return la_xac_suat(a & b, omega) / la_xac_suat(b, omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
giong_moi = {"h1", "h2", "h3", "h4", "h6", "h7", "h9", "h10"}

print(xac_suat_dieu_kien(ca_chua, giong_moi, omega))
```

```text title=readonly
0.5
```

`giong_moi` có 8 hạt, TRONG ĐÓ 4 hạt LÀ cà chua. Biết TRƯỚC hạt LÀ
giống mới, không gian mẫu THU HẸP xuống 8 hạt ĐÓ — tỉ lệ cà chua
trong 8 hạt này LÀ `4/8=0.5`.
::::

::::example{#chieu-nguoc-lai}
Đổi CHIỀU câu hỏi — "biết TRƯỚC là cà chua, xác suất NÓ cũng giống
mới" — MẪU SỐ đổi, kết quả đổi THEO:

```python title=readonly
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_dieu_kien(a, b, omega):
    return la_xac_suat(a & b, omega) / la_xac_suat(b, omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
giong_moi = {"h1", "h2", "h3", "h4", "h6", "h7", "h9", "h10"}

print(xac_suat_dieu_kien(giong_moi, ca_chua, omega))
```

```text title=readonly
0.8
```

`P(\text{giống mới}|\text{cà chua}) = 0.8` — KHÁC hẳn
`P(\text{cà chua}|\text{giống mới}) = 0.5` Ở TRÊN. Tử số CÙNG LÀ
`P(giao)`, nhưng MẪU SỐ khác (`P(giong_moi)=0.8` so với
`P(ca_chua)=0.5`) — đảo chiều "biết trước cái gì" đổi hẳn kết quả.
::::

::::predict{#doan-dieu-kien-tren-omega commitOnce}
Byte "biết trước" `Ω` (mọi kết quả có thể) — điều kiện KHÔNG loại
trừ gì cả:

```python
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_dieu_kien(a, b, omega):
    return la_xac_suat(a & b, omega) / la_xac_suat(b, omega)

omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}

print(xac_suat_dieu_kien(ca_chua, omega, omega))
print(la_xac_suat(ca_chua, omega))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`0.5`, rồi `0.5`
:::

:::opt
`1.0`, rồi `0.5` — vì "biết trước omega" nghĩa là biết CHẮC CHẮN một
điều gì đó xảy ra, và biết chắc một điều LUÔN đẩy xác suất LÊN `1.0`
::why
Gần đúng ở việc bạn nhớ ĐÚNG `P(omega)=1` (bài 12, omega chắc chắn
xảy ra) — một sự thật đúng.

Chỗ lệch: "biết TRƯỚC omega xảy ra" KHÔNG PHẢI thông tin gì cả —
omega LUÔN xảy ra (bài 12), nên "biết" điều ĐÓ không THU HẸP không
gian mẫu chút nào (`ca_chua & omega = ca_chua`, T2.4: giao với chính
tập chứa nó thì giữ nguyên). `P(\text{ca\_chua}|\Omega) =
P(ca\_chua)/P(\Omega) = 0.5/1.0 = 0.5` — bằng ĐÚNG xác suất KHÔNG
điều kiện, không phải `1.0`.
::
:::

:::opt
Máy báo lỗi khi chạy — điều kiện hoá TRÊN chính `omega` (thay vì một
tập con thật sự) là một phép tính KHÔNG hợp lệ
::why
Gần đúng ở việc bạn để ý `omega` xuất hiện Ở VỊ TRÍ "điều kiện" thay
vì một biến cố NHỎ hơn — một quan sát VỀ hình thức lời gọi.

Chỗ lệch: `omega` CŨNG LÀ một biến cố hợp lệ (bài 11: chính nó `⊆`
chính nó) — điều kiện hoá TRÊN nó hoàn toàn hợp lệ, chỉ đơn giản
KHÔNG THAY ĐỔI gì (như giải thích Ở trên). Biên dịch sạch, chạy
sạch.
::
:::
::::

::::code{#viet_xac_suat_dieu_kien}
Viết `xac_suat_dieu_kien(a, b, omega)` — tính `P(a|b) = P(a∩b) /
P(b)`.

```python title=starter
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_dieu_kien(a, b, omega):
    return ___


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
giong_moi = {"h1", "h2", "h3", "h4", "h6", "h7", "h9", "h10"}

print(xac_suat_dieu_kien(ca_chua, giong_moi, omega))
```

```python title=solution
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_dieu_kien(a, b, omega):
    return la_xac_suat(a & b, omega) / la_xac_suat(b, omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
giong_moi = {"h1", "h2", "h3", "h4", "h6", "h7", "h9", "h10"}

print(xac_suat_dieu_kien(ca_chua, giong_moi, omega))
```

```python title=test
assert xac_suat_dieu_kien(ca_chua, omega, omega) == la_xac_suat(ca_chua, omega), "dieu kien tren omega khong doi gi"
assert xac_suat_dieu_kien(giong_moi, ca_chua, omega) == 0.8, "chieu nguoc -- 4/5 = 0.8"
xa_lach = {"h6", "h7", "h8"}
assert xac_suat_dieu_kien(ca_chua, xa_lach, omega) == 0.0, "cho khong chung -- dieu kien tren xa lach thi ca chua khong the xay ra"
```

:::hints
- kind: attention
  body: "Chia xac suat cua giao (a & b) cho xac suat cua b."
- kind: strategy
  body: "la_xac_suat(a & b, omega) / la_xac_suat(b, omega)"
- kind: one-line
  body: "___ = la_xac_suat(a & b, omega) / la_xac_suat(b, omega)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai chia xac suat cua giao (a & b) cho xac suat cua b
  requireAst:
  - kind: uses-call, target: la_xac_suat, min: 2
  - kind: uses-operator, target: '/', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^0\.5\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`P(cà chua|giống mới) = 0.5` — TRÙNG với `P(cà chua)` không điều
kiện. Trùng hợp, hay hai biến cố này có quan hệ đặc biệt gì?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ở bài này, `P(\text{cà chua}|\text{giống mới}) = 0.5`, TRÙNG KHỚP
với `P(\text{cà chua}) = 0.5` KHÔNG điều kiện (bài 12) — biết TRƯỚC
hạt là giống mới KHÔNG hề đổi cơ hội nó cũng là cà chua. Trùng hợp
ngẫu nhiên, hay điều đó nói lên hai biến cố này có một QUAN HỆ đặc
biệt — quan hệ đã học TÊN riêng ở bài 16?
::::

::::checkpoint{mastery=0.8}
::::
