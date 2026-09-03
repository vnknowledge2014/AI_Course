---
id: toan.to-hop-xac-suat-thong-ke.hop-hai-bien-co-tong-quat
title: Hợp hai biến cố tổng quát
summary: "P(A∪B) = P(A) + P(B) − P(A∩B) — bù trừ (T2.4 bài 13, bài 2) áp lên xác suất khi A, B chồng lấn; trả lời câu hỏi bài 14 để ngỏ, không đổi công thức nền, chỉ đổi thứ đang đếm."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.probability-inclusion-exclusion]
requires: [math.probability-disjoint-union]
concepts: [math.xac-suat-hop-tong-quat]
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
Bài 1 sửa được đếm khi hai nhãn chồng lấn — trừ lại phần chung. Xác
suất chồng lấn có sửa được BẰNG cách tương tự không?
::::

::::explain{#hop-tong-quat}
Có. **`P(A∪B) = P(A) + P(B) − P(A∩B)`** — bù trừ (T2.4 bài 13, bài 2)
áp LÊN xác suất khi `A`, `B` chồng lấn:

```python title=readonly
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_hop(a, b, omega):
    return la_xac_suat(a, omega) + la_xac_suat(b, omega) - la_xac_suat(a & b, omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
giong_moi = {"h1", "h2"}
chiu_han = {"h2", "h3", "h4"}

print(xac_suat_hop(giong_moi, chiu_han, omega))
print(la_xac_suat(giong_moi | chiu_han, omega))
```

```text title=readonly
0.4
0.4
```

`giong_moi` VÀ `chiu_han` chung `h2`. `P(giong_moi)+P(chiu_han) =
0.2+0.3 = 0.5` đếm `h2` HAI lần; trừ lại `P(giao)=0.1` một lần ra
`0.4` — khớp ĐÚNG xác suất tính TRÊN hợp thật.
::::

::::example{#tu-rut-gon-khi-roi-nhau}
Khi `A`, `B` rời nhau, `P(A∩B) = P(∅) = 0` — công thức TỰ RÚT GỌN về
công thức bài 14, không cần dạy hai công thức tách biệt:

```python title=readonly
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_hop(a, b, omega):
    return la_xac_suat(a, omega) + la_xac_suat(b, omega) - la_xac_suat(a & b, omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
xa_lach = {"h6", "h7", "h8"}

print(xac_suat_hop(ca_chua, xa_lach, omega))
```

```text title=readonly
0.8
```

`ca_chua ∩ xa_lach = ∅`, nên `P(giao) = 0` — trừ đi `0` không đổi
gì. `0.5+0.3−0 = 0.8`, ĐÚNG khớp bài 14. Công thức bài 14 KHÔNG bị
"thay thế"; nó LÀ một trường hợp riêng của công thức này.
::::

::::predict{#doan-hop-voi-chinh-no commitOnce}
Byte áp công thức LÊN một biến cố VÀ chính nó:

```python
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_hop(a, b, omega):
    return la_xac_suat(a, omega) + la_xac_suat(b, omega) - la_xac_suat(a & b, omega)

omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}

print(xac_suat_hop(ca_chua, ca_chua, omega))
```

Dòng cuối in ra gì?

:::opt{correct}
`0.5`
:::

:::opt
`1.0` — vì công thức CỘNG `P(ca_chua)` HAI lần (`0.5+0.5=1.0`), và
phần TRỪ chỉ áp dụng khi `a` và `b` LÀ hai biến cố KHÁC nhau
::why
Gần đúng ở việc bạn đọc ĐÚNG công thức CÓ cộng `P(a)+P(b)` — với
`a=b=ca_chua`, đúng là `0.5+0.5=1.0` Ở BƯỚC ĐẦU.

Chỗ lệch: `a & b` (giao) VẪN được tính, KỂ CẢ khi `a` và `b` LÀ CÙNG
một tập — `ca_chua & ca_chua = ca_chua` (một tập giao với CHÍNH nó
luôn ra CHÍNH nó, T2.4). Nên `P(giao) = P(ca_chua) = 0.5` VẪN bị trừ
lại: `0.5+0.5−0.5 = 0.5`. `A ∪ A` (hợp của một tập với chính nó)
CŨNG luôn là CHÍNH tập đó — `P(A∪A)=P(A)`, không phải `2×P(A)`.
::
:::

:::opt
Máy báo lỗi khi chạy — gọi `xac_suat_hop(ca_chua, ca_chua, omega)`
với HAI đối số TRÙNG nhau, Python từ chối tính giao của một tập với
chính nó
::why
Gần đúng ở việc bạn để ý `ca_chua` xuất hiện Ở CẢ HAI vị trí đối số
— một quan sát đúng VỀ hình thức lời gọi.

Chỗ lệch: `ca_chua & ca_chua` hoàn toàn HỢP LỆ trong Python — nó chỉ
đơn giản trả về `ca_chua` (giao một tập với chính nó LUÔN ra chính
nó). Không có gì đặc biệt hay bị cấm khi hai đối số trùng giá trị.
::
:::
::::

::::code{#viet_xac_suat_hop}
Viết `xac_suat_hop(a, b, omega)` — tính `P(a∪b)` bằng công thức bù
trừ, không dựng hợp trước.

```python title=starter
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_hop(a, b, omega):
    return ___


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
giong_moi = {"h1", "h2"}
chiu_han = {"h2", "h3", "h4"}

print(xac_suat_hop(giong_moi, chiu_han, omega))
```

```python title=solution
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_hop(a, b, omega):
    return la_xac_suat(a, omega) + la_xac_suat(b, omega) - la_xac_suat(a & b, omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
giong_moi = {"h1", "h2"}
chiu_han = {"h2", "h3", "h4"}

print(xac_suat_hop(giong_moi, chiu_han, omega))
```

```python title=test
assert xac_suat_hop(set(), set(), omega) == 0.0, "hai tap rong -- tong 0"
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
xa_lach = {"h6", "h7", "h8"}
assert xac_suat_hop(ca_chua, xa_lach, omega) == 0.8, "roi nhau -- phai khop bai 14"
assert xac_suat_hop(ca_chua, ca_chua, omega) == 0.5, "hop voi chinh no -- van la chinh no"
assert xac_suat_hop(giong_moi, chiu_han, omega) == la_xac_suat(giong_moi | chiu_han, omega), "phai khop xac suat tren hop that"
```

:::hints
- kind: attention
  body: "Cong hai xac suat roi tru xac suat cua giao (a & b)."
- kind: strategy
  body: "la_xac_suat(a, omega) + la_xac_suat(b, omega) - la_xac_suat(a & b, omega)"
- kind: one-line
  body: "___ = la_xac_suat(a, omega) + la_xac_suat(b, omega) - la_xac_suat(a & b, omega)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai cong hai xac suat roi tru xac suat cua giao (a & b)
  requireAst:
  - kind: uses-call, target: la_xac_suat, min: 3
  - kind: uses-operator, target: '+', min: 1
  - kind: uses-operator, target: '-', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^0\.4\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Rời nhau thì công thức tự rút gọn về bài 14 — bài 14 vẫn đúng, chỉ
là một trường hợp riêng. Rút hai luống KHÁC nhau — độc lập không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte kiểm: nếu `A` và `B` rời nhau, `P(A∩B) = P(∅) = 0` — bài 14 tự
động đúng, không cần dạy công thức riêng. Giờ Byte rút MỘT hạt từ
luống 1 VÀ một hạt khác từ luống 3 (hai túi TÁCH RIÊNG, không phải
cùng một túi) — hai biến cố đó có ĐỘC LẬP nhau không, và "độc lập"
nghĩa là gì cho xác suất?
::::

::::checkpoint{mastery=0.8}
::::
