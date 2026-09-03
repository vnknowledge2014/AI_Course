---
id: toan.tap-hop-quan-he-anh-xa.anh-xa-la-quan-he-dac-biet
title: Ánh xạ là quan hệ đặc biệt
summary: "Ánh xạ — một quan hệ từ A sang B mà MỖI phần tử của A xuất hiện Ở VỊ TRÍ ĐẦU của ĐÚNG MỘT cặp (không thiếu, không thừa). \"Luống nào ai phụ trách\" là một ánh xạ NẾU mỗi luống có đúng một người."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.function-as-relation]
requires: [math.equivalence-class]
concepts: [math.anh-xa, math.dung-mot-lan]
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
Mỗi luống gán cho ĐÚNG MỘT người phụ trách — gọi là gì?
::::

::::explain{#anh-xa-la-gi}
Gọi LÀ **ánh xạ**. **Ánh xạ** — một quan hệ TỪ `A` sang `B` mà **MỖI**
phần tử của `A` xuất hiện Ở VỊ TRÍ ĐẦU của **ĐÚNG MỘT** cặp (KHÔNG
thiếu, KHÔNG thừa):

```python title=readonly
def la_anh_xa(phan_cong, a):
    for x in a:
        so_nguoi = sum(1 for (luong, nguoi) in phan_cong if luong == x)
        if so_nguoi != 1:
            return False
    return True


luong = {"luong_1", "luong_2", "luong_3"}
phan_cong_du = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Lan")}

print(la_anh_xa(phan_cong_du, luong))
```

```text title=readonly
True
```

MỖI luống trong `phan_cong_du` xuất hiện Ở vị trí ĐẦU của ĐÚNG một
cặp — `luong_1` MỘT lần (VỚI Lan), `luong_2` MỘT lần (VỚI Minh),
`luong_3` MỘT lần (VỚI Lan). Lan phụ trách HAI luống — điều ĐÓ hoàn
toàn ổn (ánh xạ KHÔNG cấm hai `x` khác nhau CÙNG "trỏ" tới một `y`).
::::

::::example{#thieu-mot-luong}
Bỏ SÓT `luong_3` — KHÔNG PHẢI ánh xạ, dù KHÔNG phần tử NÀO bị GÁN
HAI người:

```python title=readonly
def la_anh_xa(phan_cong, a):
    for x in a:
        so_nguoi = sum(1 for (luong, nguoi) in phan_cong if luong == x)
        if so_nguoi != 1:
            return False
    return True

luong = {"luong_1", "luong_2", "luong_3"}
phan_cong_thieu = {("luong_1", "Lan"), ("luong_2", "Minh")}

print(la_anh_xa(phan_cong_thieu, luong))
```

```text title=readonly
False
```

`luong_3` KHÔNG xuất hiện Ở BẤT KỲ cặp nào — `so_nguoi` của nó LÀ
`0`, KHÔNG PHẢI `1`. THIẾU MỘT phần tử LÀ đủ để phá vỡ điều kiện
"ĐÚNG MỘT" — KHÔNG PHẢI ánh xạ hợp lệ.
::::

::::predict{#doan-mot-luong-hai-nguoi commitOnce}
Byte gõ NHẦM — `luong_1` được gán CHO CẢ Lan LẪN Minh:

```python
def la_anh_xa(phan_cong, a):
    for x in a:
        so_nguoi = sum(1 for (luong, nguoi) in phan_cong if luong == x)
        if so_nguoi != 1:
            return False
    return True

luong = {"luong_1", "luong_2", "luong_3"}
phan_cong_thua = {("luong_1", "Lan"), ("luong_1", "Minh"), ("luong_2", "Minh"), ("luong_3", "Lan")}

print(la_anh_xa(phan_cong_thua, luong))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì MỌI luống (`luong_1`, `luong_2`, `luong_3`) ĐỀU CÓ MẶT
Ở ÍT NHẤT một cặp — KHÔNG luống nào bị bỏ SÓT hoàn TOÀN, nên điều
kiện "không thiếu" vẫn được giữ NGUYÊN
::why
Gần đúng ở việc bạn kiểm ĐÚNG rằng KHÔNG luống nào bị bỏ SÓT hoàn
toàn — một quan sát chính xác về phần "không THIẾU" của định nghĩa.

Chỗ lệch: định nghĩa ánh xạ (đầu bài) đòi CẢ HAI vế: "KHÔNG thiếu"
**VÀ** "KHÔNG thừa" — `luong_1` xuất hiện Ở HAI cặp (`("luong_1",
"Lan")` VÀ `("luong_1", "Minh")`), tức `so_nguoi` của `luong_1` LÀ
`2`, KHÔNG PHẢI `1`. "Thừa" (một `x` gán CHO nhiều `y`) VI PHẠM y
hệt như "thiếu" — CẢ HAI đều PHÁ điều kiện "ĐÚNG MỘT". `la_anh_xa`
trả VỀ `False`.
::
:::

:::opt
Máy báo lỗi biên dịch — `phan_cong_thua` khai HAI cặp CÙNG có
`"luong_1"` Ở vị trí đầu (`("luong_1", "Lan")` VÀ `("luong_1",
"Minh")`), Python CẤM một `set` chứa hai TUPLE có PHẦN TỬ đầu TRÙNG
nhau
::why
Gần đúng ở việc bạn để ý HAI cặp CÙNG bắt đầu bằng `"luong_1"` — một
quan sát đúng về DỮ LIỆU.

Chỗ lệch: Python KHÔNG hề cấm điều ĐÓ — MỘT `set` các tuple HOÀN
TOÀN cho phép nhiều tuple CHIA SẺ một phần tử Ở VỊ TRÍ bất kỳ, miễn
là CHÍNH các tuple đó KHÁC nhau (ở đây khác Ở vị trí THỨ HAI). Biên
dịch sạch, chạy sạch — chỉ đơn giản khiến `la_anh_xa` trả `False` VỀ
mặt Ý NGHĨA toán học.
::
:::
::::

::::code{#viet_la_anh_xa}
Viết `la_anh_xa(phan_cong, a)` — kiểm tra `phan_cong` có LÀ một ánh
xạ TỪ `a` hay không (mỗi phần tử của `a` xuất hiện Ở vị trí ĐẦU của
ĐÚNG một cặp).

```python title=starter
def la_anh_xa(phan_cong, a):
    for x in a:
        so_nguoi = ___
        if so_nguoi != 1:
            return False
    return True


luong = {"luong_1", "luong_2", "luong_3"}
phan_cong_du = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Lan")}

print(la_anh_xa(phan_cong_du, luong))
```

```python title=solution
def la_anh_xa(phan_cong, a):
    for x in a:
        so_nguoi = sum(1 for (luong, nguoi) in phan_cong if luong == x)
        if so_nguoi != 1:
            return False
    return True


luong = {"luong_1", "luong_2", "luong_3"}
phan_cong_du = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Lan")}

print(la_anh_xa(phan_cong_du, luong))
```

```python title=test
phan_cong_thieu = {("luong_1", "Lan"), ("luong_2", "Minh")}
assert la_anh_xa(phan_cong_thieu, luong) is False, "thieu luong_3"
phan_cong_thua = {("luong_1", "Lan"), ("luong_1", "Minh"), ("luong_2", "Minh"), ("luong_3", "Lan")}
assert la_anh_xa(phan_cong_thua, luong) is False, "luong_1 gan cho hai nguoi"
assert la_anh_xa(set(), set()) is True, "tap rong -- anh xa hien nhien"
```

:::hints
- kind: attention
  body: "Dem so cap (luong, nguoi) trong phan_cong ma luong bang x -- dung sum() voi generator expression."
- kind: strategy
  body: "sum(1 for (luong, nguoi) in phan_cong if luong == x)"
- kind: one-line
  body: "___ = sum(1 for (luong, nguoi) in phan_cong if luong == x)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dem SO CAP trong phan_cong co vi tri dau bang x, dung sum() voi generator -- gia tri co dinh se khong doc dung du lieu
  requireAst:
  - kind: uses-call, target: sum, min: 1
  - kind: uses-operator, target: '==', min: 1
  - kind: uses-name, target: x, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ánh xạ = mỗi phần tử của A trỏ tới ĐÚNG MỘT phần tử của B. Bài sau:
khi hai luống KHÁC nhau không được chung một người.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bảng phân công: luống 1→Lan, luống 2→Minh, luống 3→Lan, luống 4→
(chưa ai nhận). Đây có phải một ánh xạ từ tập bảy luống sang tập
người làm vườn không — và nếu không, THIẾU đúng chỗ nào so với định
nghĩa?
::::

::::checkpoint{mastery=0.8}
::::
