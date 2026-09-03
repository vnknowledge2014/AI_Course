---
id: toan.to-hop-xac-suat-thong-ke.khong-gian-mau-va-bien-co
title: Không gian mẫu và biến cố
summary: "Không gian mẫu Ω — tập hợp mọi kết quả có thể (T2.4 bài 1). Biến cố A — một tập con của Ω (T2.4 bài 6, ⊆), tức một nhóm kết quả gộp lại vì cùng chung một điều kiện."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.sample-space-event]
requires: [math.subset-count-power-of-two, math.subset]
concepts: [math.khong-gian-mau, math.bien-co]
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
Túi có 10 hạt giống. Byte rút MỘT hạt — hạt nào ra là chuyện may
rủi. Có cách nào NÓI VỀ chuyện may rủi ấy bằng ngôn ngữ tập hợp đã
biết?
::::

::::explain{#khong-gian-mau-la-gi}
Có. **Không gian mẫu `Ω`** — tập hợp MỌI kết quả CÓ THỂ (T2.4 bài 1,
tập hợp). **Biến cố `A`** — MỘT tập con của `Ω` (T2.4 bài 6, `⊆`),
tức một NHÓM kết quả gộp lại vì cùng chung một điều kiện:

```python title=readonly
def la_tap_con(a, b):
    return all(x in b for x in a)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}

print(la_tap_con(ca_chua, omega))
```

```text title=readonly
True
```

`omega` LÀ mười hạt trong túi — MỌI kết quả có thể khi rút một hạt.
`ca_chua` (năm hạt cà chua) LÀ MỘT biến cố — nó `⊆ omega`, đúng định
nghĩa. Rút được MỘT trong năm hạt đó GỌI LÀ "biến cố `ca_chua` xảy
ra".
::::

::::example{#bien-co-rong-va-toan-bo}
Hai biến cố ĐẶC BIỆT: `∅` (không kết quả nào) VÀ chính `Ω` (mọi kết
quả) — CẢ HAI đều `⊆ omega`, đều hợp lệ:

```python title=readonly
def la_tap_con(a, b):
    return all(x in b for x in a)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}

print(la_tap_con(set(), omega))
print(la_tap_con(omega, omega))
```

```text title=readonly
True
True
```

`∅ ⊆ omega` LUÔN đúng (T2.4 bài 6: không phần tử nào để phản chứng —
chân lý rỗng). `∅` LÀ biến cố "KHÔNG BAO GIỜ xảy ra" (rút một hạt thì
KHÔNG thể ra một kết quả THUỘC tập rỗng). `omega ⊆ omega` cũng LUÔN
đúng — biến cố "LUÔN xảy ra" (rút hạt nào cũng thuộc `omega`).
::::

::::predict{#doan-tap-khong-thuoc-omega commitOnce}
Byte thử một "biến cố" chứa một hạt KHÔNG hề có trong túi (`z1`, một
id BỊA ra):

```python
def la_tap_con(a, b):
    return all(x in b for x in a)

omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
gia = {"z1"}

print(la_tap_con(gia, omega))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
Máy báo lỗi khi chạy — `z1` không hề tồn tại trong `omega`, Python
từ chối kiểm `⊆` khi gặp một phần tử "lạ"
::why
Gần đúng ở việc bạn để ý ĐÚNG `z1` không có mặt trong `omega` — quan
sát đó chính xác.

Chỗ lệch: `la_tap_con` (T2.4 bài 6) KHÔNG hề coi một phần tử "lạ" là
lỗi — nó chỉ KIỂM từng phần tử của `gia` có `in omega` hay không, và
`"z1" in omega` đơn giản LÀ `False`. Kiểm tra `all(...)` trên MỘT
điều kiện sai trả VỀ `False` — biên dịch sạch, chạy sạch, kết quả
đúng là "KHÔNG phải tập con", không phải một lỗi chương trình.
::
:::

:::opt
`True` — vì `gia` chỉ có ĐÚNG một phần tử, và một tập QUÁ NHỎ (một
phần tử) luôn TỰ ĐỘNG là tập con của bất kỳ tập nào khác
::why
Gần đúng ở việc bạn để ý `gia` rất NHỎ (chỉ một phần tử) — một quan
sát VỀ KÍCH THƯỚC.

Chỗ lệch: định nghĩa tập con (T2.4 bài 6) KHÔNG liên quan gì tới
KÍCH THƯỚC — nó đòi HỎI MỌI phần tử của `gia` phải CÓ MẶT trong
`omega`, bất kể `gia` nhỏ hay lớn. `gia` chỉ có một phần tử (`z1`),
NHƯNG phần tử đó không có mặt trong `omega` — nên `gia` KHÔNG phải
tập con, dù nhỏ tới đâu.
::
:::
::::

::::code{#viet_la_bien_co_hop_le}
Viết `la_bien_co_hop_le(a, omega)` — kiểm `a` có LÀ một biến cố hợp
lệ của `omega` hay không (`a ⊆ omega`).

```python title=starter
def la_bien_co_hop_le(a, omega):
    return ___


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}

print(la_bien_co_hop_le(ca_chua, omega))
```

```python title=solution
def la_bien_co_hop_le(a, omega):
    return all(x in omega for x in a)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}

print(la_bien_co_hop_le(ca_chua, omega))
```

```python title=test
assert la_bien_co_hop_le(set(), omega) is True, "tap rong luon la bien co hop le"
assert la_bien_co_hop_le(omega, omega) is True, "chinh omega luon la bien co hop le"
assert la_bien_co_hop_le({"z1"}, omega) is False, "phan tu la khong thuoc omega"
assert la_bien_co_hop_le({"h9", "z2"}, omega) is False, "mot phan tu la la du de KHONG hop le"
```

:::hints
- kind: attention
  body: "Dung all() voi generator: moi phan tu x trong a phai co mat trong omega."
- kind: strategy
  body: "all(x in omega for x in a)"
- kind: one-line
  body: "___ = all(x in omega for x in a)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung all() voi generator, kiem tung phan tu cua a co in omega
  requireAst:
  - kind: uses-call, target: all, min: 1
  - kind: uses-operator, target: in, min: 1
  - kind: comprehension, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Biến cố là một tập con. Nhưng tập con có mấy phần tử thì nói được gì
về ĐỘ MAY RỦI của nó?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Túi có 10 hạt: 5 cà chua, 3 xà lách, 2 cà rốt. `Ω` có 10 phần tử.
Biến cố "rút được cà chua" có 5 phần tử. Hai con số ấy (5 và 10) nói
được gì về ĐỘ MAY RỦI của việc rút trúng cà chua, so với rút trúng
cà rốt?
::::

::::checkpoint{mastery=0.8}
::::
