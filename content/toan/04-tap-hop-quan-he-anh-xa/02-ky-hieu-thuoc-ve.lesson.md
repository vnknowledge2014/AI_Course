---
id: toan.tap-hop-quan-he-anh-xa.ky-hieu-thuoc-ve
title: Ký hiệu thuộc về
summary: "`∈` là ký hiệu toán của `in` đã biết — `cà chua ∈ luống_1` và `\"cà chua\" in luong_1` nói CÙNG một câu; ∈ đòi hỏi khớp NGUYÊN VĂN, không phải gần giống."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.set-element]
requires: [math.set, core.set-membership, core.dict, ctrl.for-each, core.list-append, core.list]
concepts: [math.thuoc-ve, math.khop-nguyen-van]
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
Byte viết `cà chua ∈ luống_1` — bằng ký hiệu, không phải bằng chữ Python.
Máy đọc được câu ấy thẳng như vậy không?
::::

::::explain{#ky-hieu-in}
Không thẳng — nhưng bạn đã biết CÂU ẤY MUỐN NÓI GÌ, vì nó chính là câu bạn
đã gõ ở bài trước.

**`∈`** là ký hiệu toán của `in`. `cà chua ∈ luống_1` và `"cà chua" in
luong_1` nói CÙNG một câu — "cà chua có mặt trong luống 1" — chỉ khác chữ
viết: nhà toán học dùng một ký hiệu Hy Lạp, người lập trình dùng ba chữ cái
tiếng Anh.

```python title=readonly
luong_1 = {"cà chua", "xà lách", "cà rốt"}

print("cà chua" in luong_1)
```

```text title=readonly
True
```
::::

::::example{#mot-loai-rau-nhieu-luong}
Cùng một loại rau, tra ở nhiều luống khác nhau — mỗi lần tra là một câu
`∈` riêng:

```python title=readonly
luong_1 = {"cà chua", "xà lách", "cà rốt"}
luong_2 = {"xà lách", "cải bó xôi"}

print("cà chua" in luong_1)
print("cà chua" in luong_2)
```

```text title=readonly
True
False
```

`cà chua ∈ luống_1` đúng, `cà chua ∈ luống_2` sai — hai câu ĐỘC LẬP, mỗi
câu chỉ nói về ĐÚNG một tập hợp.
::::

::::predict{#doan-viet-hoa commitOnce}
Byte gõ nhầm tay, viết hoa chữ đầu:

```python
luong_1 = {"cà chua", "xà lách", "cà rốt"}

print("Cà chua" in luong_1)
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì "Cà chua" và "cà chua" cùng nói về MỘT loại rau THẬT ngoài
vườn, khác biệt viết hoa/viết thường chỉ là chuyện GÕ PHÍM, không phải
chuyện cây cối
::why
Gần đúng ở việc bạn nghĩ TỚI ý nghĩa THẬT ngoài đời — "Cà chua" và "cà
chua" đúng là CÙNG một loại rau khi bạn nói chuyện với người khác.

Chỗ lệch: `luong_1` không chứa "loại rau ngoài đời", nó chứa những CHUỖI
KÝ TỰ cụ thể — và với chuỗi, `"C"` (viết hoa) và `"c"` (viết thường) là
HAI ký tự khác nhau, y hệt cách hai số khác nhau là hai số khác nhau. `∈`
đòi khớp **nguyên văn từng ký tự**, không tự động "hiểu ý" người gõ.
`"Cà chua"` không nằm trong `luong_1` (tập chỉ có `"cà chua"` chữ thường)
— câu `∈` này SAI.
::
:::

:::opt
Máy báo lỗi biên dịch — so sánh hai chuỗi khác nhau CHỮ HOA/CHỮ THƯỜNG
(`"Cà chua"` với các phần tử toàn chữ thường của `luong_1`) là một phép
so sánh không hợp lệ, Python yêu cầu hai vế `in` phải cùng kiểu VIẾT
::why
Gần đúng ở việc bạn để ý `"Cà chua"` viết HOA còn mọi phần tử của
`luong_1` viết THƯỜNG — một quan sát đúng về HÌNH THỨC chuỗi.

Chỗ lệch: Python không có khái niệm "cùng kiểu viết" nào cho phép `in` cả
— nó chỉ so sánh chuỗi VỚI chuỗi, bất kể hoa/thường. `"Cà chua" in
luong_1` biên dịch và chạy hoàn toàn sạch; nó chỉ đơn giản tìm không thấy
và trả `False`.
::
:::
::::

::::code{#viet_nhung_luong_co}
Viết `nhung_luong_co(rau, vuon)` — `vuon` là một `dict` ánh xạ TÊN luống
sang TẬP HỢP loại rau của luống đó; hàm trả về danh sách tên NHỮNG luống
mà `rau ∈` tập hợp của nó.

```python title=starter
def nhung_luong_co(rau, vuon):
    ket_qua = []
    for ten_luong in vuon:
        if ___:
            ket_qua.append(ten_luong)
    return ket_qua


vuon = {
    "luong_1": {"cà chua", "xà lách", "cà rốt"},
    "luong_2": {"xà lách", "cải bó xôi"},
    "luong_3": {"cà rốt", "khoai lang"},
}

print(nhung_luong_co("cà chua", vuon))
```

```python title=solution
def nhung_luong_co(rau, vuon):
    ket_qua = []
    for ten_luong in vuon:
        if rau in vuon[ten_luong]:
            ket_qua.append(ten_luong)
    return ket_qua


vuon = {
    "luong_1": {"cà chua", "xà lách", "cà rốt"},
    "luong_2": {"xà lách", "cải bó xôi"},
    "luong_3": {"cà rốt", "khoai lang"},
}

print(nhung_luong_co("cà chua", vuon))
```

```python title=test
assert nhung_luong_co("xà lách", vuon) == ["luong_1", "luong_2"], "xa lach co trong ca luong_1 va luong_2"
assert nhung_luong_co("cà rốt", vuon) == ["luong_1", "luong_3"], "ca rot co trong ca luong_1 va luong_3"
assert nhung_luong_co("bí đỏ", vuon) == [], "khong luong nao trong vuon nay trong bi do"
assert nhung_luong_co("khoai lang", vuon) == ["luong_3"], "khoai lang chi co o luong_3"
assert nhung_luong_co("Cà chua", vuon) == [], "viet hoa khac viet thuong -- khong khop nguyen van, khong luong nao duoc tinh"
```

:::hints
- kind: attention
  body: "Dieu kien if phai kiem rau CO MAT trong tap hop cua luong dang xet: vuon[ten_luong] la tap hop, dung `in` de tra."
- kind: strategy
  body: "rau in vuon[ten_luong]"
- kind: one-line
  body: "___ = rau in vuon[ten_luong]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: dieu kien phai tra CHINH XAC rau co trong TAP HOP cua tung luong (vuon[ten_luong]) bang toan tu `in` -- so sanh true/false gia hoac bo qua vuon[ten_luong] se khong doc dung du lieu cua tung luong
  requireAst:
  - kind: uses-operator, target: in, min: 1
  - kind: uses-name, target: rau, min: 1
  - kind: uses-name, target: ten_luong, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\['luong_1'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`∈` khớp nguyên văn, không "hiểu ý". Bài sau: viết đúng cái câu NGƯỢC
LẠI — "không có mặt" — bằng một ký hiệu, không phải `not (... ∈ ...)`
dài dòng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

"Khoai tây `∈` luống_1" — Byte tra thấy sai. Có ký hiệu nào nói thẳng
"không có mặt", khỏi phải viết `not (... ∈ ...)` dài dòng không?
::::

::::checkpoint{mastery=0.8}
::::
