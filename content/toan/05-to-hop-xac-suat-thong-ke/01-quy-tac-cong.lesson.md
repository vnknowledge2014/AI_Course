---
id: toan.to-hop-xac-suat-thong-ke.quy-tac-cong
title: Quy tắc cộng
summary: "|A ∪ B| = |A| + |B| khi A ∩ B = ∅ — trường hợp riêng, không chồng lấn, của đếm bù trừ (T2.4). Ba túi hạt giống KHÔNG trộn: tổng hạt là cộng thẳng ba số."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.rule-of-sum]
requires: [math.set-disjoint, math.inclusion-exclusion]
concepts: [math.quy-tac-cong]
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
Ba túi hạt giống: cà chua, xà lách, cà rốt — KHÔNG hạt nào ở hai túi
cùng lúc. Đếm TỔNG số hạt cả ba túi, cách nào NHANH nhất?
::::

::::explain{#quy-tac-cong-la-gi}
**Quy tắc cộng** — khi hai tập RỜI NHAU (bài 10, T2.4: `A ∩ B = ∅`),
`|A ∪ B| = |A| + |B|`. KHÔNG cần dựng `A ∪ B` rồi đếm nó; cộng thẳng
hai con số `|A|`, `|B|` là đủ:

```python title=readonly
def hop(a, b):
    return a | b


hat_ca_chua = {"h1", "h2", "h3", "h4", "h5"}
hat_xa_lach = {"h6", "h7", "h8"}

print(hat_ca_chua & hat_xa_lach == set())
print(len(hat_ca_chua) + len(hat_xa_lach))
print(len(hop(hat_ca_chua, hat_xa_lach)))
```

```text title=readonly
True
8
8
```

`hat_ca_chua` và `hat_xa_lach` RỜI NHAU (giao rỗng — dòng đầu `True`).
Cộng thẳng `5 + 3 = 8` khớp ĐÚNG với đếm hợp thật `len(hop(...))`.
Khi rời nhau, KHÔNG cần dựng hợp mới đếm được.
::::

::::example{#ba-tui-roi-nhau}
Ba túi cùng lúc — quy tắc cộng áp DÃY, không chỉ hai:

```python title=readonly
hat_ca_chua = {"h1", "h2", "h3", "h4", "h5"}
hat_xa_lach = {"h6", "h7", "h8"}
hat_ca_rot = {"h9", "h10"}

print(len(hat_ca_chua) + len(hat_xa_lach) + len(hat_ca_rot))
```

```text title=readonly
10
```

Ba túi ĐÔI MỘT rời nhau (không hạt nào chung id giữa BẤT KỲ cặp túi
nào — `h1`-`h5`, `h6`-`h8`, `h9`-`h10` không chồng lên nhau). Cộng
thẳng ba số `5 + 3 + 2 = 10` là tổng hạt CẢ vườn.
::::

::::predict{#doan-quen-kiem-roi-nhau commitOnce}
Byte viết xong `dem_tong_khi_roi_nhau` (bài này) rồi LỠ TAY gọi nó
trên hai tập KHÔNG kiểm rời nhau trước — `giong_moi` và `chiu_han`,
hai nhãn có thể CÙNG gắn lên một hạt (`h2`, `h3` mang CẢ HAI):

```python
def dem_tong_khi_roi_nhau(a, b):
    return len(a) + len(b)

giong_moi = {"h1", "h2", "h3"}
chiu_han = {"h2", "h3", "h4"}

print(dem_tong_khi_roi_nhau(giong_moi, chiu_han))
```

Dòng cuối in ra gì?

:::opt{correct}
`6` — chạy SẠCH, KHÔNG báo lỗi, NHƯNG con số này KHÔNG PHẢI số hạt
mang ÍT NHẤT một trong hai nhãn (số đó thật ra là 4) — hàm ÂM THẦM
sai vì `giong_moi` và `chiu_han` KHÔNG rời nhau
:::

:::opt
Máy báo lỗi khi chạy — hàm phát hiện `giong_moi` VÀ `chiu_han` chung
nhau `h2`, `h3` rồi TỪ CHỐI cộng
::why
Gần đúng ở việc bạn để ý ĐÚNG `h2` VÀ `h3` xuất hiện Ở CẢ HAI tập —
một quan sát đúng VỀ dữ liệu.

Chỗ lệch: `dem_tong_khi_roi_nhau` (bài này viết) CHỈ làm ĐÚNG một
việc — `len(a) + len(b)` — nó KHÔNG hề kiểm `a` VÀ `b` có rời nhau
hay không trước khi cộng. Python KHÔNG tự động kiểm điều kiện toán
học của một công thức; hàm chạy TRÊN bất kỳ hai tập nào được đưa
vào, dù công thức chỉ ĐÚNG khi chúng rời nhau.
::
:::

:::opt
In ra `4` — hàm tự động PHÁT HIỆN phần chung rồi trừ lại, luôn trả
về đúng số hạt mang ít nhất một nhãn dù hai tập có chồng lấn hay
không
::why
Gần đúng ở việc bạn nhớ ĐÚNG số hạt mang ít nhất một nhãn THẬT là 4
(`h1`, `h2`, `h3`, `h4`) — con số đó đúng.

Chỗ lệch: `dem_tong_khi_roi_nhau` KHÔNG hề "tự động phát hiện" gì cả
— nó chỉ có ĐÚNG một dòng, `len(a) + len(b)`, cộng thẳng hai độ dài
BẤT KỂ chúng có chung phần tử hay không. Muốn ra `4` thì phải TRỪ lại
phần chung — đúng công thức bù trừ T2.4 bài 13 — chứ hàm này (chỉ
cộng thẳng) không tự làm việc đó.
::
:::
::::

::::code{#viet_dem_tong_khi_roi_nhau}
Viết `dem_tong_khi_roi_nhau(a, b)` — trả về tổng số phần tử của `a`
VÀ `b`, ÁP DỤNG quy tắc cộng (chỉ cộng thẳng độ dài, không dựng hợp).

```python title=starter
def dem_tong_khi_roi_nhau(a, b):
    return ___


hat_ca_chua = {"h1", "h2", "h3", "h4", "h5"}
hat_xa_lach = {"h6", "h7", "h8"}

print(dem_tong_khi_roi_nhau(hat_ca_chua, hat_xa_lach))
```

```python title=solution
def dem_tong_khi_roi_nhau(a, b):
    return len(a) + len(b)


hat_ca_chua = {"h1", "h2", "h3", "h4", "h5"}
hat_xa_lach = {"h6", "h7", "h8"}

print(dem_tong_khi_roi_nhau(hat_ca_chua, hat_xa_lach))
```

```python title=test
assert dem_tong_khi_roi_nhau(set(), set()) == 0, "hai tap rong -- tong rong"
assert dem_tong_khi_roi_nhau({"a"}, set()) == 1, "mot tap rong -- tong bang tap con lai"
hat_ca_rot = {"h9", "h10"}
assert dem_tong_khi_roi_nhau(hat_xa_lach, hat_ca_rot) == 5, "3 + 2 = 5"
assert dem_tong_khi_roi_nhau(hat_ca_chua, hat_xa_lach) == len(hat_ca_chua | hat_xa_lach), "khi roi nhau, cong thang phai khop dem hop that"
```

:::hints
- kind: attention
  body: "Quy tac cong: cong THANG do dai hai tap, khong dung hop truoc."
- kind: strategy
  body: "len(a) + len(b)"
- kind: one-line
  body: "___ = len(a) + len(b)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai cong THANG len(a) va len(b), khong dung phep hop truoc
  requireAst:
  - kind: uses-call, target: len, min: 2
  - kind: uses-operator, target: '+', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^8\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Rời nhau thì cộng thẳng — chồng lấn thì sai âm thầm. T2.4 bài 13 đã
sửa được cho HAI tập. Ba tập chồng lấn cùng lúc thì sao?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`giong_moi` và `chiu_han` chồng lấn — T2.4 bài 13 đã có công thức
sửa cho ĐÚNG HAI tập (`|A∪B| = |A|+|B| − |A∩B|`). Byte thêm nhãn thứ
ba, "ít nước", cũng chồng lấn với cả hai nhãn kia. Công thức HAI tập
đó còn đủ không, hay phải trừ/cộng lại NHIỀU lần hơn?
::::

::::checkpoint{mastery=0.8}
::::
