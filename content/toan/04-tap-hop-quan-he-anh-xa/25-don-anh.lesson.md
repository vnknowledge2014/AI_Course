---
id: toan.tap-hop-quan-he-anh-xa.don-anh
title: Đơn ánh
summary: "Đơn ánh (một-một) — hai luống KHÁC nhau không bao giờ cùng chung một người phụ trách; kiểm bằng cách: số người xuất hiện trong bảng phân công phải bằng ĐÚNG số luống."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.injective]
requires: [math.function-as-relation]
concepts: [math.don-anh]
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
Byte muốn phân công sao cho KHÔNG người nào phải chăm HAI luống —
mỗi người ĐÚNG một luống. Điều kiện ĐÓ có tên riêng không?
::::

::::explain{#don-anh-la-gi}
Có. **Đơn ánh** (một-một) — hai phần tử KHÁC nhau của `A` KHÔNG bao
giờ trỏ tới CÙNG một phần tử của `B`. Kiểm bằng: SỐ người xuất hiện
phải bằng ĐÚNG SỐ cặp (KHÔNG người nào bị LẶP):

```python title=readonly
def la_don_anh(phan_cong):
    nguoi = {y for (x, y) in phan_cong}
    return len(nguoi) == len(phan_cong)


phan_cong_don = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu")}

print(la_don_anh(phan_cong_don))
```

```text title=readonly
True
```

BA cặp, BA người KHÁC nhau (`Lan`, `Minh`, `Tu`) — `len(nguoi)` VÀ
`len(phan_cong)` ĐỀU LÀ `3`, khớp nhau. Đơn ánh.
::::

::::example{#khong-don-anh}
MỘT người phụ trách HAI luống — KHÔNG PHẢI đơn ánh:

```python title=readonly
def la_don_anh(phan_cong):
    nguoi = {y for (x, y) in phan_cong}
    return len(nguoi) == len(phan_cong)

phan_cong_khong_don = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Lan")}

print(la_don_anh(phan_cong_khong_don))
```

```text title=readonly
False
```

BA cặp, NHƯNG chỉ HAI người KHÁC nhau (`Lan` xuất hiện Ở HAI cặp —
tập hợp `nguoi` chỉ giữ `Lan` MỘT lần, bài 1). `len(nguoi)` LÀ `2`,
KHÔNG khớp `len(phan_cong)` LÀ `3` — KHÔNG đơn ánh.
::::

::::predict{#doan-thieu-luong-van-don-anh commitOnce}
`la_don_anh` KHÔNG nhận tham số `a` (tập luống gốc) — CHỈ nhìn vào
CHÍNH `phan_cong`. Byte thử VỚI một bảng THIẾU (bỏ SÓT `luong_3`,
KHÔNG PHẢI ánh xạ đầy đủ theo bài 24):

```python
def la_don_anh(phan_cong):
    nguoi = {y for (x, y) in phan_cong}
    return len(nguoi) == len(phan_cong)

phan_cong_thieu = {("luong_1", "Lan"), ("luong_2", "Minh")}
print(la_don_anh(phan_cong_thieu))
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — vì `phan_cong_thieu` KHÔNG PHẢI một ánh xạ ĐẦY ĐỦ (bỏ sót
`luong_3`, đã học bài 24), và đơn ánh CHỈ có Ý nghĩa KHI xét TRÊN
một ánh xạ ĐÃ hợp lệ — thiếu điều kiện NỀN đó thì `la_don_anh` PHẢI
trả `False`
::why
Gần đúng ở việc bạn nhớ ĐÚNG `phan_cong_thieu` KHÔNG PHẢI một ánh xạ
HOÀN CHỈNH (bài 24) — một quan sát chính xác về điều kiện KHÁC.

Chỗ lệch: `la_don_anh` (đúng NHƯ được ĐỊNH NGHĨA VÀ VIẾT) CHỈ kiểm
MỘT điều: "hai `y` khác nhau có TRÙNG không" — nó KHÔNG hề kiểm
"MỌI `x` trong `a` đã CÓ mặt chưa" (đó LÀ việc của `la_anh_xa`, bài
24, MỘT hàm KHÁC). Đơn ánh VÀ "là ánh xạ đầy đủ" LÀ HAI điều kiện
ĐỘC LẬP — MỘT bảng THIẾU vẫn có thể đơn ánh (KHÔNG người nào bị
LẶP), dù nó KHÔNG PHẢI ánh xạ đầy đủ. `phan_cong_thieu` có HAI cặp,
HAI người KHÁC nhau — đơn ánh, `True`.
::
:::

:::opt
Máy báo lỗi biên dịch — hàm `la_don_anh` chỉ nhận MỘT tham số
(`phan_cong`), NHƯNG để kiểm đơn ánh ĐÚNG NGHĨA toán học thì BẮT
BUỘC phải biết CẢ tập `a` gốc, Python sẽ TỪ CHỐI chạy hàm THIẾU
tham số cần thiết
::why
Gần đúng ở việc bạn nghĩ TỚI định nghĩa TOÁN học ĐẦY ĐỦ của đơn
ánh (thường phát biểu TRÊN cả `A` và `B`) — một trực giác hợp lý VỀ
mặt Ý NGHĨA.

Chỗ lệch: Python KHÔNG hề kiểm tra hàm có "ĐỦ Ý NGHĨA toán học" hay
không — nó CHỈ kiểm số THAM SỐ CÓ khớp lời gọi hay không. `la_don_anh`
được ĐỊNH NGHĨA CHỈ cần MỘT tham số, VÀ được GỌI với ĐÚNG một tham
số — biên dịch sạch, chạy sạch, DÙ định nghĩa NÀY chọn KHÔNG kiểm
tính đầy đủ (đó LÀ lựa chọn THIẾT KẾ, không phải lỗi).
::
:::
::::

::::code{#viet_la_don_anh}
Viết `la_don_anh(phan_cong)` — kiểm tra `phan_cong` có đơn ánh hay
không.

```python title=starter
def la_don_anh(phan_cong):
    nguoi = ___
    return len(nguoi) == len(phan_cong)


phan_cong_don = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu")}

print(la_don_anh(phan_cong_don))
```

```python title=solution
def la_don_anh(phan_cong):
    nguoi = {y for (x, y) in phan_cong}
    return len(nguoi) == len(phan_cong)


phan_cong_don = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu")}

print(la_don_anh(phan_cong_don))
```

```python title=test
phan_cong_khong_don = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Lan")}
assert la_don_anh(phan_cong_khong_don) is False, "Lan phu trach hai luong"
assert la_don_anh(set()) is True, "tap rong -- don anh hien nhien"
assert la_don_anh({("luong_1", "Lan")}) is True, "mot cap duy nhat luon don anh"
```

:::hints
- kind: attention
  body: "Dung set comprehension lay tap hop nhung nguoi (vi tri thu hai) trong phan_cong."
- kind: strategy
  body: "{y for (x, y) in phan_cong}"
- kind: one-line
  body: "___ = {y for (x, y) in phan_cong}"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung set comprehension lay tap hop nguoi (vi tri thu hai) tu phan_cong, roi so do dai voi len(phan_cong)
  requireAst:
  - kind: comprehension, min: 1
  - kind: uses-call, target: len, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đơn ánh = không ai bị gán trùng. Bài sau: khi MỌI người đều có việc,
không ai đứng ngoài.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Phân công đủ bảy luống, không luống nào bỏ trống, NHƯNG Lan phụ
trách cả luống 1 và luống 3. Đây có phải đơn ánh không? Nó có còn là
một ánh xạ hợp lệ (bài 24) không?
::::

::::checkpoint{mastery=0.8}
::::
