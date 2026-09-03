---
id: toan.tap-hop-quan-he-anh-xa.cap-co-thu-tu
title: Cặp có thứ tự
summary: "`(a, b)` — thứ tự CÓ nghĩa, tương phản THẲNG với tập hợp (bài 1: thứ tự KHÔNG nghĩa). `(a,b) ≠ (b,a)` trừ khi a=b, và `(a,b)=(c,d)` chỉ khi CẢ HAI vị trí khớp — Python tuple (T1.4) đã sẵn có tính chất này."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.ordered-pair]
requires: [math.inclusion-exclusion, core.tuple]
concepts: [math.cap-co-thu-tu, math.tuong-phan-voi-tap-hop]
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
"Luống 1 tưới thứ Hai" — viết thành `(luống_1, "Hai")`. Đảo lại thành
`("Hai", luống_1)` — có còn nói ĐÚNG câu ĐÓ không?
::::

::::explain{#cap-co-thu-tu-la-gi}
KHÔNG. **`(a, b)`** — cặp có thứ tự — thứ tự **CÓ Ý NGHĨA**, tương
phản THẲNG với tập hợp (bài 1: thứ tự KHÔNG có nghĩa). Python `tuple`
(T1.4 bài 16) đã sẵn tính chất ĐÓ:

```python title=readonly
cap_1 = ("luong_1", "Hai")
cap_2 = ("Hai", "luong_1")

print(cap_1 == cap_2)
```

```text title=readonly
False
```

Đảo VỊ TRÍ hai phần tử LÀ đảo Ý NGHĨA HOÀN TOÀN — `("luong_1",
"Hai")` nói "luống 1 tưới thứ Hai"; `("Hai", "luong_1")` nói MỘT câu
KHÁC (vô nghĩa, vì "Hai" không phải TÊN luống). Hai cặp KHÔNG bằng
nhau, đúng như VẬY.
::::

::::example{#tuong-phan-voi-tap-hop}
ĐÚNG hai phần tử ĐÓ (`"luong_1"`, `"Hai"`) — nhưng đóng gói THÀNH tập
hợp (bài 1) THAY VÌ cặp — hai cách đóng gói cho HAI kết QUẢ khác hẳn
nhau khi đổi thứ tự viết:

```python title=readonly
tap_a = {"luong_1", "Hai"}
tap_b = {"Hai", "luong_1"}
cap_a = ("luong_1", "Hai")
cap_b = ("Hai", "luong_1")

print(tap_a == tap_b)
print(cap_a == cap_b)
```

```text title=readonly
True
False
```

CÙNG hai giá trị, CÙNG bị đảo thứ tự viết — tập hợp KHÔNG quan tâm
(`tap_a == tap_b` LÀ `True`), cặp có thứ tự THÌ quan TÂM (`cap_a ==
cap_b` LÀ `False`). Bài 1 vừa dạy "thứ tự KHÔNG nghĩa" — bài NÀY dạy
đúng điều NGƯỢC LẠI, cho một cách ĐÓNG GÓI khác.
::::

::::predict{#doan-hai-vi-tri-phai-khop commitOnce}
Byte kiểm TRA MỘT lần nữa, cẩn thận HƠN — hai cặp CÙNG thứ tự
NHƯNG khác Ở VỊ TRÍ thứ hai:

```python
cap_1 = ("luong_1", "Hai")
cap_3 = ("luong_1", "Hai")
cap_4 = ("luong_1", "Ba")

print(cap_1 == cap_3)
print(cap_1 == cap_4)
```

Hai dòng cuối in ra gì?

:::opt{correct}
`True`, rồi `False`
:::

:::opt
`True`, rồi `True` — vì `cap_1` VÀ `cap_4` CÙNG có VỊ TRÍ đầu LÀ
`"luong_1"`, VÀ chỉ CẦN MỘT trong hai vị trí khớp LÀ đủ để hai cặp
được coi LÀ bằng nhau
::why
Gần đúng ở việc bạn để ý CẢ hai cặp CÙNG có `"luong_1"` Ở VỊ TRÍ đầu
— một quan sát đúng về MỘT phần dữ liệu.

Chỗ lệch: `(a,b) = (c,d)` (đã ghi Ở đầu bài) đòi hỏi **CẢ HAI** vị
trí khớp — `a=c` **VÀ** `b=d` — không PHẢI "một trong hai đủ". Vị
trí ĐẦU của `cap_1`/`cap_4` khớp (`"luong_1"`), NHƯNG vị TRÍ hai LỆCH
(`"Hai"` khác `"Ba"`) — MỘT vị trí lệch LÀ ĐỦ để CẢ cặp KHÔNG bằng
nhau. `cap_1 == cap_4` LÀ `False`.
::
:::

:::opt
Máy báo lỗi biên dịch — so sánh BA biến tuple (`cap_1`, `cap_3`,
`cap_4`) liên TIẾP bằng `==` TRONG hai dòng `print` khác nhau đòi
hỏi khai BÁO kiểu RÕ ràng cho từng biến TRƯỚC khi so sánh
::why
Gần đúng ở việc bạn để ý CÓ BA biến tuple riêng biệt Ở ĐÂY — một
quan sát đúng về SỐ LƯỢNG biến.

Chỗ lệch: Python KHÔNG đòi khai kiểu TRƯỚC khi so sánh `==` — MỌI
biến (dù bao NHIÊU) đều so sánh được TRỰC TIẾP, KHÔNG cần khai BÁO gì
thêm. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_dao_cap}
Viết `dao_cap(cap)` — đảo VỊ TRÍ hai phần tử của MỘT cặp.

```python title=starter
def dao_cap(cap):
    a, b = cap
    return ___


print(dao_cap(("luong_1", "Hai")))
```

```python title=solution
def dao_cap(cap):
    a, b = cap
    return (b, a)


print(dao_cap(("luong_1", "Hai")))
```

```python title=test
assert dao_cap(dao_cap(("luong_1", "Hai"))) == ("luong_1", "Hai"), "dao hai lan quay ve cap goc"
assert dao_cap(("x", "x")) == ("x", "x"), "hai phan tu giong het -- dao xong van nhu cu"
assert dao_cap((1, 2)) == (2, 1), "hoat dong voi so, khong chi chuoi"
assert dao_cap(("luong_1", "Hai")) != ("luong_1", "Hai"), "dao mot lan phai KHAC cap goc (tru khi hai phan tu giong nhau)"
```

:::hints
- kind: attention
  body: "Tra ve MOT tuple moi voi thu tu nguoc lai: b truoc, a sau."
- kind: strategy
  body: "(b, a)"
- kind: one-line
  body: "___ = (b, a)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tra ve tuple (b, a) -- dao DUNG thu tu, khong phai (a, b) (khong doi gi) hay mot gia tri khac
  requireAst:
  - kind: uses-name, target: a, min: 1
  - kind: uses-name, target: b, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\('Hai', 'luong_1'\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cặp có thứ tự — đảo vị trí LÀ đảo ý nghĩa. Bài sau: liệt kê TẤT CẢ cặp
CÓ THỂ có giữa hai tập hợp.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte có bảy luống, tuần có bảy ngày. Liệt kê TẤT CẢ cặp (luống, ngày)
CÓ THỂ có — kể cả cặp không tưới thật — thì được bao nhiêu cặp?
::::

::::checkpoint{mastery=0.8}
::::
