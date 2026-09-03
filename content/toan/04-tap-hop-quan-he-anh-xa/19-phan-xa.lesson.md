---
id: toan.tap-hop-quan-he-anh-xa.phan-xa
title: Phản xạ
summary: "Quan hệ phản xạ — MỌI phần tử quan hệ với CHÍNH NÓ. Quan hệ MỚI \"cùng khu vườn với\" trên tập bảy luống: luống nào cũng cùng vườn với chính nó → phản xạ."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.relation-reflexive]
requires: [math.relation-domain-range, logic.for-all]
concepts: [math.phan-xa]
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
Bảy luống, MỘT quan hệ MỚI: "cùng khu vườn với". Mọi luống có cùng
khu vườn VỚI CHÍNH NÓ không?
::::

::::explain{#phan-xa-la-gi}
Có — cùng khu vườn với CHÍNH MÌNH LUÔN đúng. Đó chính LÀ tính chất
**phản xạ**: MỘT quan hệ LÀ phản xạ khi MỌI phần tử quan hệ VỚI
CHÍNH NÓ.

```python title=readonly
luong = {"luong_1", "luong_2", "luong_3"}
cung_vuon = {(x, y) for x in luong for y in luong}

def la_phan_xa(quan_he, a):
    return all((x, x) in quan_he for x in a)

print(la_phan_xa(cung_vuon, luong))
```

```text title=readonly
True
```

`cung_vuon` LÀ TOÀN BỘ tích Descartes (bài 15) — MỌI luống quan hệ
với MỌI luống, kể CẢ chính nó. `la_phan_xa` kiểm ĐÚNG định nghĩa:
`(x, x) ∈ quan_he` VỚI **MỌI** `x ∈ a` (dùng `all()`, T2.3).
::::

::::example{#khong-phai-quan-he-nao-cung-phan-xa}
KHÔNG PHẢI quan hệ nào cũng phản xạ — "luống 1 tưới TRƯỚC luống 3"
(một quan hệ THỨ TỰ) thì KHÔNG:

```python title=readonly
luong = {"luong_1", "luong_2", "luong_3"}
tuoi_truoc = {("luong_1", "luong_2"), ("luong_2", "luong_3")}

def la_phan_xa(quan_he, a):
    return all((x, x) in quan_he for x in a)

print(la_phan_xa(tuoi_truoc, luong))
```

```text title=readonly
False
```

`tuoi_truoc` KHÔNG chứa cặp `("luong_1", "luong_1")` — luống 1 KHÔNG
"tưới trước" CHÍNH nó (điều đó vô nghĩa). MỘT phản ví dụ LÀ đủ để LOẠI
tính phản xạ — KHÔNG PHẢI phản xạ.
::::

::::predict{#doan-phan-xa-tren-tap-rong commitOnce}
Byte kiểm tra tính phản xạ TRÊN một tập LUỐNG rỗng (KHÔNG luống nào
để mà xét):

```python
tuoi_truoc = {("luong_1", "luong_2"), ("luong_2", "luong_3")}

def la_phan_xa(quan_he, a):
    return all((x, x) in quan_he for x in a)

print(la_phan_xa(set(), set()))
print(la_phan_xa(tuoi_truoc, set()))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`True`, rồi `True`
:::

:::opt
`True`, rồi `False` — vì `tuoi_truoc` (dòng HAI) KHÔNG PHẢI tập
RỖNG, nó CÓ SẴN hai cặp KHÔNG phản xạ — nên DÙ tập luống ĐEM xét là
rỗng, quan hệ VẪN "mang tiếng" không phản xạ TỪ trước
::why
Gần đúng ở việc bạn nhớ ĐÚNG `tuoi_truoc` (bài trước) KHÔNG phản xạ
TRÊN tập BA luống — một quan sát đúng về QUAN HỆ đó Ở NGỮ CẢNH khác.

Chỗ lệch: tính phản xạ LUÔN được kiểm THEO một tập `a` CỤ THỂ (câu
hỏi LÀ "MỌI `x ∈ a` có `(x,x) ∈ quan_he` không", KHÔNG PHẢI "quan hệ
này có gì trong nó"). VỚI `a = set()` (KHÔNG luống nào), `all()` lặp
qua CHUỖI rỗng — chân lý rỗng (bài 6): LUÔN `True`, BẤT KỂ `quan_he`
chứa gì. `tuoi_truoc` có "xấu" tới đâu CŨNG không quan trọng, vì
KHÔNG có `x` nào để mà kiểm tra `(x,x)` cả.
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `la_phan_xa` với THAM SỐ THỨ HAI LÀ
`set()` rỗng trong khi tham số ĐẦU (`quan_he`) CÓ phần tử là một
sự KHÔNG khớp, Python yêu cầu CẢ HAI tham số phải cùng RỖNG hoặc
cùng KHÔNG rỗng
::why
Gần đúng ở việc bạn để ý HAI đối số CÓ "độ đầy" khác nhau (một RỖNG,
một KHÔNG) — một quan sát đúng về HÌNH THỨC lời gọi.

Chỗ lệch: Python không hề đòi HAI tham số của MỘT hàm phải "khớp độ
đầy" — MỖI tham số HOÀN TOÀN độc lập, nhận GIÁ TRỊ gì cũng được, bất
kể tham số KHÁC. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_la_phan_xa}
Viết `la_phan_xa(quan_he, a)` — kiểm tra `quan_he` có phản xạ TRÊN
tập `a` hay không.

```python title=starter
def la_phan_xa(quan_he, a):
    return ___


luong = {"luong_1", "luong_2", "luong_3"}
cung_vuon = {(x, y) for x in luong for y in luong}

print(la_phan_xa(cung_vuon, luong))
```

```python title=solution
def la_phan_xa(quan_he, a):
    return all((x, x) in quan_he for x in a)


luong = {"luong_1", "luong_2", "luong_3"}
cung_vuon = {(x, y) for x in luong for y in luong}

print(la_phan_xa(cung_vuon, luong))
```

```python title=test
tuoi_truoc = {("luong_1", "luong_2"), ("luong_2", "luong_3")}
assert la_phan_xa(tuoi_truoc, luong) is False, "khong co (x,x) nao trong tuoi_truoc"
assert la_phan_xa(set(), set()) is True, "tap rong -- phan xa hien nhien"
assert la_phan_xa({("a", "a"), ("b", "b")}, {"a", "b"}) is True, "du (x,x) cho ca hai phan tu"
assert la_phan_xa({("a", "a")}, {"a", "b"}) is False, "thieu (b,b)"
```

:::hints
- kind: attention
  body: "Dung all() kiem MOI x trong a co cap (x, x) nam trong quan_he khong."
- kind: strategy
  body: "all((x, x) in quan_he for x in a)"
- kind: one-line
  body: "___ = all((x, x) in quan_he for x in a)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung all() kiem MOI phan tu x cua a co cap (x, x) trong quan_he -- thieu all() se khong kiem het MOI phan tu
  requireAst:
  - kind: uses-call, target: all, min: 1
  - kind: uses-operator, target: in, min: 1
  - kind: uses-name, target: a, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Phản xạ = mọi phần tử quan hệ với chính nó. Bài sau: nếu A quan hệ
với B, B có quan hệ ngược lại với A không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

"Luống A trồng nhiều loại HƠN luống B" — luống nào có nhiều loại hơn
CHÍNH NÓ không? Quan hệ này có phản xạ không, và nó thiếu đúng cái gì
so với phản xạ?
::::

::::checkpoint{mastery=0.8}
::::
