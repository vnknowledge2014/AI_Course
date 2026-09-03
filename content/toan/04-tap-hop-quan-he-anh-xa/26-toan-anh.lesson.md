---
id: toan.tap-hop-quan-he-anh-xa.toan-anh
title: Toàn ánh
summary: "Toàn ánh (phủ hết miền giá trị) — MỌI người làm vườn đều có ÍT NHẤT một luống, không ai đứng ngoài. Một ánh xạ có thể đơn ánh mà KHÔNG toàn ánh, toàn ánh mà KHÔNG đơn ánh, hoặc CẢ HAI — hai tính chất ĐỘC LẬP nhau."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.surjective]
requires: [math.injective]
concepts: [math.toan-anh]
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
Byte có ba người làm vườn. Điều kiện "MỌI người đều CÓ việc, không
ai đứng NGOÀI" có tên riêng không?
::::

::::explain{#toan-anh-la-gi}
Có. **Toàn ánh** — mọi phần tử của `B` (miền GIÁ TRỊ) đều được TRỎ
tới BỞI ÍT NHẤT một phần tử của `A`. Kiểm bằng: tập người CÓ việc
phải BẰNG ĐÚNG tập MỌI người:

```python title=readonly
def la_toan_anh(phan_cong, b):
    nguoi_co_viec = {y for (x, y) in phan_cong}
    return nguoi_co_viec == b


nguoi_lam_vuon = {"Lan", "Minh", "Tu"}
phan_cong_toan = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu")}

print(la_toan_anh(phan_cong_toan, nguoi_lam_vuon))
```

```text title=readonly
True
```

`nguoi_co_viec` (rút RA từ `phan_cong_toan`) LÀ `{Lan, Minh, Tu}` —
khớp ĐÚNG `nguoi_lam_vuon`. Toàn ánh.
::::

::::example{#khong-toan-anh}
`Tu` KHÔNG được phân luống NÀO — KHÔNG PHẢI toàn ánh:

```python title=readonly
def la_toan_anh(phan_cong, b):
    nguoi_co_viec = {y for (x, y) in phan_cong}
    return nguoi_co_viec == b

nguoi_lam_vuon = {"Lan", "Minh", "Tu"}
phan_cong_khong_toan = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Lan")}

print(la_toan_anh(phan_cong_khong_toan, nguoi_lam_vuon))
```

```text title=readonly
False
```

`nguoi_co_viec` CHỈ LÀ `{Lan, Minh}` — THIẾU `Tu`. `Tu` thuộc
`nguoi_lam_vuon` NHƯNG KHÔNG có mặt trong `nguoi_co_viec` — hai tập
KHÔNG bằng nhau (bài 7). KHÔNG toàn ánh.
::::

::::predict{#doan-don-anh-khong-toan-anh commitOnce}
Byte thử một bảng CHỈ phân công HAI luống (Ít hơn số người) — MỖI
người KHÁC nhau (đơn ánh, bài 25):

```python
def la_don_anh(phan_cong):
    nguoi = {y for (x, y) in phan_cong}
    return len(nguoi) == len(phan_cong)

def la_toan_anh(phan_cong, b):
    nguoi_co_viec = {y for (x, y) in phan_cong}
    return nguoi_co_viec == b

nguoi_lam_vuon = {"Lan", "Minh", "Tu"}
phan_cong_2luong = {("luong_1", "Lan"), ("luong_2", "Minh")}

print(la_don_anh(phan_cong_2luong))
print(la_toan_anh(phan_cong_2luong, nguoi_lam_vuon))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`True`, rồi `False`
:::

:::opt
`True`, rồi `True` — vì `phan_cong_2luong` ĐÃ đơn ánh (không người
NÀO trùng), VÀ MỘT ánh xạ đơn ánh THÌ TỰ ĐỘNG cũng LÀ toàn ánh
(đơn ánh LÀ điều kiện MẠNH hơn, bao HÀM luôn toàn ánh)
::why
Gần đúng ở việc bạn kiểm ĐÚNG `phan_cong_2luong` đơn ánh (`Lan` VÀ
`Minh` khác nhau) — quan sát ĐÓ chính xác.

Chỗ lệch: đơn ánh VÀ toàn ánh LÀ HAI điều kiện HOÀN TOÀN ĐỘC LẬP,
KHÔNG cái nào "bao hàm" cái kia — như chính TÊN bài viết: "một ánh
xạ có thể đơn ánh MÀ KHÔNG toàn ánh". `phan_cong_2luong` CHỈ có HAI
cặp, nên `nguoi_co_viec` CHỈ LÀ `{Lan, Minh}` — `Tu` KHÔNG hề xuất
hiện. `Tu ∈ nguoi_lam_vuon` NHƯNG `Tu ∉ nguoi_co_viec` — KHÔNG toàn
ánh, dù ĐÃ đơn ánh.
::
:::

:::opt
Máy báo lỗi biên dịch — `phan_cong_2luong` CHỈ có HAI cặp trong khi
`nguoi_lam_vuon` có BA người, Python YÊU CẦU hai tham số của
`la_toan_anh` phải CÙNG kích thước
::why
Gần đúng ở việc bạn để ý HAI TẬP có KÍCH THƯỚC khác nhau (hai cặp
VÀ ba người) — một quan sát đúng về SỐ LƯỢNG.

Chỗ lệch: Python KHÔNG hề đòi hai đối SỐ của một hàm phải "cùng kích
thước" — mỗi tham SỐ nhận GIÁ TRỊ gì cũng được, KÍCH thước bao NHIÊU
cũng được. Biên dịch sạch, chạy sạch — chính SỰ chênh lệch kích
thước NÀY mới LÀ điều `la_toan_anh` đang PHÁT hiện ra.
::
:::
::::

::::code{#viet_la_toan_anh}
Viết `la_toan_anh(phan_cong, b)` — kiểm tra `phan_cong` có toàn ánh
LÊN `b` hay không.

```python title=starter
def la_toan_anh(phan_cong, b):
    nguoi_co_viec = ___
    return nguoi_co_viec == b


nguoi_lam_vuon = {"Lan", "Minh", "Tu"}
phan_cong_toan = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu")}

print(la_toan_anh(phan_cong_toan, nguoi_lam_vuon))
```

```python title=solution
def la_toan_anh(phan_cong, b):
    nguoi_co_viec = {y for (x, y) in phan_cong}
    return nguoi_co_viec == b


nguoi_lam_vuon = {"Lan", "Minh", "Tu"}
phan_cong_toan = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu")}

print(la_toan_anh(phan_cong_toan, nguoi_lam_vuon))
```

```python title=test
phan_cong_khong_toan = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Lan")}
assert la_toan_anh(phan_cong_khong_toan, nguoi_lam_vuon) is False, "Tu khong co viec"
assert la_toan_anh(set(), set()) is True, "tap rong -- toan anh hien nhien"
phan_cong_4luong = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu"), ("luong_4", "Lan")}
assert la_toan_anh(phan_cong_4luong, nguoi_lam_vuon) is True, "toan anh nhung khong don anh -- Lan phu trach hai luong"
```

:::hints
- kind: attention
  body: "Dung set comprehension lay tap hop nguoi co viec (vi tri thu hai) tu phan_cong, roi so == voi b."
- kind: strategy
  body: "{y for (x, y) in phan_cong}"
- kind: one-line
  body: "___ = {y for (x, y) in phan_cong}"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung set comprehension lay tap hop nguoi co viec tu phan_cong, roi so == voi b -- dung == chu khong phai <= hay >=
  requireAst:
  - kind: comprehension, min: 1
  - kind: uses-operator, target: '==', min: 1
  - kind: uses-name, target: b, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Toàn ánh = không ai đứng ngoài. Bài sau: khi CẢ đơn ánh LẪN toàn ánh
cùng đúng — ánh xạ có "đảo ngược" được không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte có đúng bốn người làm vườn, bốn luống, và phân công vừa đơn ánh
vừa toàn ánh. Một ánh xạ có CẢ HAI tính chất ấy có tên riêng không —
và nó có "đảo ngược" được không?
::::

::::checkpoint{mastery=0.8}
::::
