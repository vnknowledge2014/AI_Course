---
id: toan.tap-hop-quan-he-anh-xa.hop-hai-tap
title: Hợp của hai tập hợp
summary: "`∪` là ký hiệu toán của `|` đã biết (T1.4) — A ∪ B gồm phần tử thuộc A HOẶC B (hoặc cả hai). Gộp NHIỀU tập hợp (không chỉ hai) dùng lại đúng phép `|` trong một vòng lặp cộng dồn."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.set-union]
requires: [math.set-equality, core.set-union, core.accumulator]
concepts: [math.hop, math.gop-nhieu-tap]
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
Luống 1 trồng {cà chua, xà lách}, luống 3 trồng {xà lách, cà rốt}. Gộp
CẢ HAI lại thành một tập — xà lách xuất hiện Ở CẢ HAI, vậy nó bị đếm
hai lần trong tập kết quả không?
::::

::::explain{#hop-la-gi}
**`∪`** (hợp) là ký hiệu toán của `|` đã biết (T1.4 bài 33). `A ∪ B`
gồm mọi phần tử thuộc A HOẶC B (hoặc CẢ HAI — đúng nghĩa "hoặc" của
T2.3 bài 5, không phải "một trong hai nhưng không cả hai").

```python title=readonly
luong_1 = {"cà chua", "xà lách"}
luong_3 = {"xà lách", "cà rốt"}

print(sorted(luong_1 | luong_3))
print(len(luong_1 | luong_3))
```

```text title=readonly
['cà chua', 'cà rốt', 'xà lách']
3
```

Ba loại, KHÔNG PHẢI bốn — `xà lách` xuất hiện Ở CẢ luống 1 lẫn luống 3,
NHƯNG tập hợp (bài 1) KHÔNG có khái niệm "đếm mấy lần": nó CHỈ hỏi CÓ
mặt hay KHÔNG. `luống_1 ∪ luống_3` giữ `xà lách` đúng MỘT lần, dù nó
"xứng đáng" xuất hiện từ hai nguồn.
::::

::::example{#hop-khong-quan-tam-thu-tu}
`∪` không quan tâm gộp bên nào TRƯỚC — đổi chỗ A và B vẫn ra CÙNG một
tập hợp:

```python title=readonly
luong_1 = {"cà chua", "xà lách"}
luong_3 = {"xà lách", "cà rốt"}

print((luong_1 | luong_3) == (luong_3 | luong_1))
```

```text title=readonly
True
```

`luống_1 ∪ luống_3` và `luống_3 ∪ luống_1` là CÙNG một tập hợp (đúng
tính chất giao hoán, T2.1 bài 20) — hợp lý, vì `∪` chỉ hỏi "có mặt Ở
MỘT trong hai", câu hỏi ĐÓ không phân biệt "hỏi A trước hay B trước".
::::

::::predict{#doan-hop-voi-tap-rong commitOnce}
Byte gộp một luống có rau với một luống trống hoàn toàn:

```python
luong_4 = {"bí đỏ"}
luong_6 = set()

print(sorted(luong_4 | luong_6))
print(len(luong_4 | luong_6))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`['bí đỏ']`, rồi `1`
:::

:::opt
Máy báo lỗi biên dịch — `luong_6` là `set()` RỖNG, và toán tử `|` yêu
cầu CẢ HAI vế phải có ÍT NHẤT một phần tử để "gộp", không thể gộp với
một tập TRỐNG TRƠN
::why
Gần đúng ở việc bạn để ý `luong_6` HOÀN TOÀN rỗng — một quan sát đúng
về DỮ LIỆU đầu vào.

Chỗ lệch: `|` KHÔNG hề đòi hỏi điều kiện đó — gộp với tập rỗng LUÔN
hợp lệ, và kết quả đơn giản LÀ tập KIA nguyên vẹn (đúng vai trò của
`∅` — "phần tử trung tính" của phép hợp, giống cách `0` là phần tử
trung tính của phép cộng). `luống_4 ∪ ∅` = `luống_4`. Biên dịch sạch,
chạy sạch.
::
:::

:::opt
`[]`, rồi `0` — vì gộp một tập CÓ phần tử với một tập RỖNG thì kết quả
"trung hoà" lẫn nhau về mức RỖNG, giống cách một số CỘNG với số ÂM của
chính nó ra `0`
::why
Gần đúng ở việc bạn liên tưởng tới phép TOÁN quen thuộc (cộng với số
đối ra `0`, T2.1 bài 18) — một phép loại suy hợp lý nhưng SAI chỗ áp
dụng.

Chỗ lệch: `∅` KHÔNG PHẢI "số đối" của `luống_4` — nó chỉ đơn giản LÀ
"không có gì để thêm hay bớt". Gộp `luống_4` với `∅` không "triệt
tiêu" gì cả — nó GIỮ NGUYÊN `luống_4`, vì `∪` chỉ THÊM VÀO những phần
tử MỚI mà `∅` không hề có. Kết quả LÀ `luống_4` nguyên vẹn: `['bí
đỏ']`, độ dài `1`.
::
:::
::::

::::code{#viet_tat_ca_loai_rau}
Viết `tat_ca_loai_rau(vuon)` — gộp TẤT CẢ tập hợp loại rau của MỌI
luống trong `vuon` thành MỘT tập hợp DUY NHẤT (không chỉ hai luống —
CẢ vườn).

```python title=starter
def tat_ca_loai_rau(vuon):
    ket_qua = set()
    for ten_luong in vuon:
        ket_qua = ___
    return ket_qua


vuon = {
    "luong_1": {"cà chua", "xà lách"},
    "luong_2": {"xà lách", "cải bó xôi"},
    "luong_3": {"cà rốt"},
}

print(sorted(tat_ca_loai_rau(vuon)))
```

```python title=solution
def tat_ca_loai_rau(vuon):
    ket_qua = set()
    for ten_luong in vuon:
        ket_qua = ket_qua | vuon[ten_luong]
    return ket_qua


vuon = {
    "luong_1": {"cà chua", "xà lách"},
    "luong_2": {"xà lách", "cải bó xôi"},
    "luong_3": {"cà rốt"},
}

print(sorted(tat_ca_loai_rau(vuon)))
```

```python title=test
assert tat_ca_loai_rau(vuon) == {"cà chua", "xà lách", "cải bó xôi", "cà rốt"}, "gop dung bon loai, khong lap"
assert tat_ca_loai_rau({}) == set(), "vuon rong -- hop rong"
assert tat_ca_loai_rau({"a": set(), "b": set()}) == set(), "moi luong deu rong -- hop van rong"
assert tat_ca_loai_rau({"a": {"x"}}) == {"x"}, "chi mot luong -- hop bang chinh no"
```

:::hints
- kind: attention
  body: "Moi luot lap, cong don ket_qua bang phep hop (|) voi tap hop cua luong dang xet."
- kind: strategy
  body: "ket_qua | vuon[ten_luong]"
- kind: one-line
  body: "___ = ket_qua | vuon[ten_luong]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: moi luot lap phai CONG DON ket_qua bang phep hop (|) voi tap hop cua luong dang xet -- thieu ket_qua cu se lam mat du lieu cac luot truoc
  requireAst:
  - kind: uses-name, target: ket_qua, min: 2
  - kind: uses-name, target: ten_luong, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\['cà chua', 'cà rốt', 'cải bó xôi', 'xà lách'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`∪` gộp mọi thứ CÓ MẶT, không đếm mấy lần. Bài sau: chỉ giữ những gì
CHUNG cả hai, không giữ phần riêng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Có cặp luống nào mà phép `∩` (chung) ra `∅` không? Nếu có, hai luống ấy
quan hệ gì với nhau — và tên riêng cho quan hệ "không chung gì" đó là
gì?
::::

::::checkpoint{mastery=0.8}
::::
