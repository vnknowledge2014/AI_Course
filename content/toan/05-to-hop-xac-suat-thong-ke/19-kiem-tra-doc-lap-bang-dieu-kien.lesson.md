---
id: toan.to-hop-xac-suat-thong-ke.kiem-tra-doc-lap-bang-dieu-kien
title: Kiểm tra độc lập bằng điều kiện
summary: "A, B độc lập ⟺ P(A|B) = P(A) — biết B xảy ra rồi mà cơ hội A KHÔNG đổi, đúng nghĩa \"độc lập\" (bài 16) phát biểu lại qua điều kiện (bài 18); hai định nghĩa, MỘT khái niệm."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.independence-via-conditional]
requires: [math.conditional-probability]
concepts: [math.kiem-doc-lap-bang-dieu-kien]
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
`P(cà chua|giống mới) = 0.5`, trùng khớp `P(cà chua)` không điều
kiện. Đó có phải chính là "độc lập" (bài 16), nhìn qua một cửa khác?
::::

::::explain{#doc-lap-qua-dieu-kien}
Đúng vậy. **`A`, `B` độc lập ⟺ `P(A|B) = P(A)`** — biết `B` xảy ra
rồi mà cơ hội `A` KHÔNG đổi, đúng nghĩa "độc lập" (bài 16) phát biểu
LẠI qua điều kiện (bài 18); HAI định nghĩa, MỘT khái niệm (T2.3 bài
15, "khi và chỉ khi"):

```python title=readonly
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_dieu_kien(a, b, omega):
    return la_xac_suat(a & b, omega) / la_xac_suat(b, omega)

def la_doc_lap_qua_dieu_kien(a, b, omega):
    return xac_suat_dieu_kien(a, b, omega) == la_xac_suat(a, omega)


bag1 = {"a1", "a2", "a3", "a4"}
ca_chua1 = {"a1", "a2"}
bag3 = {"b1", "b2", "b3", "b4", "b5"}
ca_chua3 = {"b1"}
omega_combo = {(x, y) for x in bag1 for y in bag3}
A = {(x, y) for x in ca_chua1 for y in bag3}
B = {(x, y) for x in bag1 for y in ca_chua3}

print(la_doc_lap_qua_dieu_kien(A, B, omega_combo))
```

```text title=readonly
True
```

Hai túi TÁCH RIÊNG (bài 16, ĐÃ chứng minh độc lập BẰNG `P(A∩B)=
P(A)·P(B)`) — kiểm LẠI bằng CỬA điều kiện cũng ra `True`. Cùng một
sự thật, hai cách hỏi.
::::

::::example{#roi-nhau-khong-doc-lap-qua-dieu-kien}
Rời nhau (bài 16: KHÔNG độc lập) — kiểm LẠI bằng cửa điều kiện, VẪN
`False`:

```python title=readonly
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_dieu_kien(a, b, omega):
    return la_xac_suat(a & b, omega) / la_xac_suat(b, omega)

def la_doc_lap_qua_dieu_kien(a, b, omega):
    return xac_suat_dieu_kien(a, b, omega) == la_xac_suat(a, omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
xa_lach = {"h6", "h7", "h8"}

print(la_doc_lap_qua_dieu_kien(ca_chua, xa_lach, omega))
```

```text title=readonly
False
```

`P(\text{cà chua}|\text{xà lách}) = 0.0` (biết trước xà lách thì cà
chua KHÔNG THỂ xảy ra) — KHÁC HẲN `P(\text{cà chua})=0.5`. Cùng kết
luận "không độc lập" như bài 16 đã tìm ra, giờ nhìn qua cửa khác.
::::

::::predict{#doan-doi-tui-ca-rot commitOnce}
Byte đổi túi: nhãn "giống mới" GIỜ CHỈ có Ở hai loại cà chua/xà lách
— KHÔNG hạt cà rốt nào mang nhãn ĐÓ:

```python
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_dieu_kien(a, b, omega):
    return la_xac_suat(a & b, omega) / la_xac_suat(b, omega)

def la_doc_lap_qua_dieu_kien(a, b, omega):
    return xac_suat_dieu_kien(a, b, omega) == la_xac_suat(a, omega)

omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_rot = {"h9", "h10"}
giong_moi2 = {"h1", "h2", "h6"}

print(la_doc_lap_qua_dieu_kien(ca_rot, giong_moi2, omega))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì "cà rốt" và "giống mới" là hai NHÃN hoàn toàn KHÁC LOẠI
(một nói về RAU, một nói về GIỐNG) — hai nhãn khác loại luôn độc lập
::why
Gần đúng ở việc bạn để ý `ca_rot` (loại rau) VÀ `giong_moi2` (giống)
LÀ hai NHÃN khác HẲN nội dung — một quan sát đúng VỀ Ý NGHĨA.

Chỗ lệch: độc lập (bài 16, 19) KHÔNG liên quan gì tới hai nhãn có
"cùng LOẠI" hay không — nó CHỈ hỏi `P(A|B)` có bằng `P(A)` hay
không, tính TRÊN dữ liệu THẬT. `giong_moi2` KHÔNG có hạt cà rốt nào
— biết TRƯỚC hạt LÀ giống mới thì cà rốt CHẮC CHẮN KHÔNG xảy ra
(`P=0.0`), khác hẳn `P(\text{cà rốt})=0.2` không điều kiện. Biết
TRƯỚC "giống mới" ĐÃ LOẠI TRỪ khả năng cà rốt — không hề độc lập.
::
:::

:::opt
Máy báo lỗi khi chạy — `giong_moi2` không hề chứa hạt nào chung VỚI
`ca_rot`, Python từ chối tính điều kiện hoá trên hai tập rời nhau
::why
Gần đúng ở việc bạn để ý `ca_rot & giong_moi2 = ∅` (không chung hạt
nào) — một quan sát đúng.

Chỗ lệch: `xac_suat_dieu_kien` tính bình thường KỂ CẢ khi giao LÀ
rỗng — `P(∅)/P(giong_moi2) = 0/0.3 = 0.0`, một phép chia hợp lệ
(chia SỐ 0 cho một số khác 0). Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_la_doc_lap_qua_dieu_kien}
Viết `la_doc_lap_qua_dieu_kien(a, b, omega)` — kiểm `P(a|b)` có bằng
`P(a)` hay không.

```python title=starter
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_dieu_kien(a, b, omega):
    return la_xac_suat(a & b, omega) / la_xac_suat(b, omega)

def la_doc_lap_qua_dieu_kien(a, b, omega):
    return ___


bag1 = {"a1", "a2", "a3", "a4"}
ca_chua1 = {"a1", "a2"}
bag3 = {"b1", "b2", "b3", "b4", "b5"}
ca_chua3 = {"b1"}
omega_combo = {(x, y) for x in bag1 for y in bag3}
A = {(x, y) for x in ca_chua1 for y in bag3}
B = {(x, y) for x in bag1 for y in ca_chua3}

print(la_doc_lap_qua_dieu_kien(A, B, omega_combo))
```

```python title=solution
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_dieu_kien(a, b, omega):
    return la_xac_suat(a & b, omega) / la_xac_suat(b, omega)

def la_doc_lap_qua_dieu_kien(a, b, omega):
    return xac_suat_dieu_kien(a, b, omega) == la_xac_suat(a, omega)


bag1 = {"a1", "a2", "a3", "a4"}
ca_chua1 = {"a1", "a2"}
bag3 = {"b1", "b2", "b3", "b4", "b5"}
ca_chua3 = {"b1"}
omega_combo = {(x, y) for x in bag1 for y in bag3}
A = {(x, y) for x in ca_chua1 for y in bag3}
B = {(x, y) for x in bag1 for y in ca_chua3}

print(la_doc_lap_qua_dieu_kien(A, B, omega_combo))
```

```python title=test
omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
xa_lach = {"h6", "h7", "h8"}
assert la_doc_lap_qua_dieu_kien(ca_chua, xa_lach, omega) is False, "roi nhau -- KHONG doc lap"
ca_rot = {"h9", "h10"}
giong_moi2 = {"h1", "h2", "h6"}
assert la_doc_lap_qua_dieu_kien(ca_rot, giong_moi2, omega) is False, "giong moi loai tru ca rot -- KHONG doc lap"
assert la_doc_lap_qua_dieu_kien(A, B, omega_combo) is True, "hai tui tach rieng -- doc lap that"
```

:::hints
- kind: attention
  body: "So sanh xac_suat_dieu_kien(a, b, omega) voi la_xac_suat(a, omega)."
- kind: strategy
  body: "xac_suat_dieu_kien(a, b, omega) == la_xac_suat(a, omega)"
- kind: one-line
  body: "___ = xac_suat_dieu_kien(a, b, omega) == la_xac_suat(a, omega)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai so sanh xac_suat_dieu_kien(a, b, omega) voi la_xac_suat(a, omega) bang ==
  requireAst:
  - kind: uses-call, target: xac_suat_dieu_kien, min: 1
  - kind: uses-call, target: la_xac_suat, min: 1
  - kind: uses-operator, target: '==', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai định nghĩa độc lập, một khái niệm. Có công thức nào nối HAI
chiều điều kiện (bài 18) lại với nhau không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte tính `P(\text{cà chua}|\text{giống mới})` VÀ
`P(\text{giống mới}|\text{cà chua})` (bài 18) — hai con số bằng công
thức na ná nhau (cùng chia cho `P(A∩B)` Ở tử) nhưng mẫu số khác, và
KHÁC nhau. Có công thức nào nối HAI chiều điều kiện ấy lại, suy
chiều này TỪ chiều kia, không cần đếm lại từ đầu?
::::

::::checkpoint{mastery=0.8}
::::
