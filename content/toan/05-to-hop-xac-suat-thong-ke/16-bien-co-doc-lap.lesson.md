---
id: toan.to-hop-xac-suat-thong-ke.bien-co-doc-lap
title: Biến cố độc lập
summary: "P(A∩B) = P(A) · P(B) khi A, B ĐỘC LẬP (biết A xảy ra không đổi gì cơ hội B xảy ra) — ứng quy tắc nhân (bài 3) lên xác suất. Rời nhau KHÔNG phải độc lập — hai khái niệm khác hẳn nhau."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.probability-independence]
requires: [math.probability-inclusion-exclusion]
concepts: [math.bien-co-doc-lap]
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
Rút MỘT hạt từ luống 1, VÀ một hạt khác từ luống 3 — hai túi TÁCH
RIÊNG. Kết quả của túi này có đổi CƠ HỘI của túi kia không?
::::

::::explain{#doc-lap-la-gi}
Không đổi. **`P(A∩B) = P(A) · P(B)`** khi `A`, `B` ĐỘC LẬP — biết `A`
xảy ra KHÔNG hề đổi cơ hội `B` xảy ra. Ứng quy tắc nhân (bài 3) lên
xác suất — không gian mẫu KẾT HỢP LÀ tích Descartes (T2.4 bài 15)
của hai túi:

```python title=readonly
def la_xac_suat(a, omega):
    return len(a) / len(omega)


bag1 = {"a1", "a2", "a3", "a4"}
ca_chua1 = {"a1", "a2"}
bag3 = {"b1", "b2", "b3", "b4", "b5"}
ca_chua3 = {"b1"}

omega_combo = {(x, y) for x in bag1 for y in bag3}
A = {(x, y) for x in ca_chua1 for y in bag3}
B = {(x, y) for x in bag1 for y in ca_chua3}

print(la_xac_suat(A, omega_combo))
print(la_xac_suat(B, omega_combo))
print(la_xac_suat(A & B, omega_combo))
print(la_xac_suat(A, omega_combo) * la_xac_suat(B, omega_combo))
```

```text title=readonly
0.5
0.2
0.1
0.1
```

`A` LÀ "luống 1 ra cà chua" (không quan tâm túi 3), `B` LÀ "luống 3
ra cà chua" (không quan tâm túi 1). `P(A∩B)=0.1` khớp ĐÚNG
`P(A)×P(B)=0.5×0.2=0.1` — hai túi TÁCH RIÊNG, kết quả túi này không
ảnh hưởng túi kia.
::::

::::example{#hai-tui-tach-la-doc-lap}
Kiểm lại BẰNG hàm `la_doc_lap`, so sánh HAI cách tính:

```python title=readonly
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def la_doc_lap(a, b, omega):
    return la_xac_suat(a & b, omega) == la_xac_suat(a, omega) * la_xac_suat(b, omega)


bag1 = {"a1", "a2", "a3", "a4"}
ca_chua1 = {"a1", "a2"}
bag3 = {"b1", "b2", "b3", "b4", "b5"}
ca_chua3 = {"b1"}

omega_combo = {(x, y) for x in bag1 for y in bag3}
A = {(x, y) for x in ca_chua1 for y in bag3}
B = {(x, y) for x in bag1 for y in ca_chua3}

print(la_doc_lap(A, B, omega_combo))
```

```text title=readonly
True
```

Hai túi TÁCH RIÊNG (một hạt từ túi 1, một hạt KHÁC từ túi 3) LUÔN
độc lập — kết quả bên này không hề "biết" hay "ảnh hưởng" kết quả
bên kia.
::::

::::predict{#doan-roi-nhau-co-doc-lap-khong commitOnce}
Byte kiểm `ca_chua` VÀ `xa_lach` — RỜI NHAU (một hạt không thể vừa
là cà chua vừa là xà lách), CÙNG một túi mười hạt:

```python
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def la_doc_lap(a, b, omega):
    return la_xac_suat(a & b, omega) == la_xac_suat(a, omega) * la_xac_suat(b, omega)

omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
xa_lach = {"h6", "h7", "h8"}

print(la_doc_lap(ca_chua, xa_lach, omega))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — rời nhau nghĩa là "không liên quan gì tới nhau", và "không
liên quan" đúng LÀ định nghĩa của độc lập
::why
Gần đúng ở việc bạn nghe "rời nhau" và "độc lập" đều CÓ vẻ nói về
"không dính dáng gì nhau" — một sự nhầm lẫn RẤT phổ biến, kể cả với
người đã học xác suất một thời gian.

Chỗ lệch: rời nhau nghĩa là `P(A∩B)=P(∅)=0` — biết CHẮC `B` xảy ra
(rút được xà lách) thì `A` (cà chua) HOÀN TOÀN KHÔNG THỂ xảy ra —
đây là quan hệ CỰC KỲ chặt (biết B thì LOẠI TRỪ A), không phải
"không liên quan". Độc lập THẬT (bài này) đòi `P(A∩B)=P(A)·P(B)` —
Ở đây `P(A∩B)=0` NHƯNG `P(A)·P(B)=0.5×0.3=0.15≠0`, nên `False`. Rời
nhau (trừ trường hợp một trong hai có xác suất `0`) hầu như KHÔNG
BAO GIỜ độc lập — hai khái niệm đối lập nhau về bản chất, không phải
đồng nghĩa.
::
:::

:::opt
Máy báo lỗi khi chạy — `ca_chua & xa_lach` cho kết quả rỗng, và chia
`0.0` bằng phép nhân `P(a)*P(b)` (một số khác 0) là một phép so sánh
không hợp lệ
::why
Gần đúng ở việc bạn để ý `ca_chua & xa_lach` LÀ tập rỗng — một quan
sát đúng.

Chỗ lệch: `la_xac_suat(set(), omega)` chỉ đơn giản LÀ `0/10 = 0.0`,
một phép CHIA hợp lệ bình thường (không phải chia CHO 0, mà chia SỐ
0 cho 10). So sánh `0.0 == 0.15` cũng hoàn toàn hợp lệ trong Python
— nó chỉ trả về `False`, không phải lỗi.
::
:::
::::

::::code{#viet_la_doc_lap}
Viết `la_doc_lap(a, b, omega)` — kiểm `P(a∩b)` có bằng `P(a)×P(b)`
hay không.

```python title=starter
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def la_doc_lap(a, b, omega):
    return ___


bag1 = {"a1", "a2", "a3", "a4"}
ca_chua1 = {"a1", "a2"}
bag3 = {"b1", "b2", "b3", "b4", "b5"}
ca_chua3 = {"b1"}
omega_combo = {(x, y) for x in bag1 for y in bag3}
A = {(x, y) for x in ca_chua1 for y in bag3}
B = {(x, y) for x in bag1 for y in ca_chua3}

print(la_doc_lap(A, B, omega_combo))
```

```python title=solution
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def la_doc_lap(a, b, omega):
    return la_xac_suat(a & b, omega) == la_xac_suat(a, omega) * la_xac_suat(b, omega)


bag1 = {"a1", "a2", "a3", "a4"}
ca_chua1 = {"a1", "a2"}
bag3 = {"b1", "b2", "b3", "b4", "b5"}
ca_chua3 = {"b1"}
omega_combo = {(x, y) for x in bag1 for y in bag3}
A = {(x, y) for x in ca_chua1 for y in bag3}
B = {(x, y) for x in bag1 for y in ca_chua3}

print(la_doc_lap(A, B, omega_combo))
```

```python title=test
omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
xa_lach = {"h6", "h7", "h8"}
assert la_doc_lap(ca_chua, xa_lach, omega) is False, "roi nhau -- KHONG doc lap"
assert la_doc_lap(omega, omega, omega) is True, "omega voi chinh no -- P=1=1x1"
assert la_doc_lap(set(), ca_chua, omega) is True, "rong voi bat ky -- 0 = 0 x P(a)"
assert la_doc_lap(A, B, omega_combo) is True, "hai tui tach rieng -- doc lap that"
```

:::hints
- kind: attention
  body: "So sanh la_xac_suat(a & b, omega) voi la_xac_suat(a, omega) * la_xac_suat(b, omega)."
- kind: strategy
  body: "la_xac_suat(a & b, omega) == la_xac_suat(a, omega) * la_xac_suat(b, omega)"
- kind: one-line
  body: "___ = la_xac_suat(a & b, omega) == la_xac_suat(a, omega) * la_xac_suat(b, omega)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai so sanh xac suat cua giao voi tich hai xac suat rieng, dung ==
  requireAst:
  - kind: uses-call, target: la_xac_suat, min: 3
  - kind: uses-operator, target: '*', min: 1
  - kind: uses-operator, target: '==', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Rời nhau KHÔNG phải độc lập — hai khái niệm đối lập. Gieo BỐN hạt
độc lập, xác suất CẢ BỐN cùng nảy mầm tính bằng cách nào?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte gieo BỐN hạt, MỖI hạt độc lập nảy mầm hay không (không hạt nào
ảnh hưởng hạt khác). Quy tắc nhân (bài 3) áp cho HAI biến cố độc lập
— công thức bài này chỉ có `A` và `B`. Nhân xác suất cho BỐN biến cố
độc lập cùng lúc thì viết thế nào?
::::

::::checkpoint{mastery=0.8}
::::
