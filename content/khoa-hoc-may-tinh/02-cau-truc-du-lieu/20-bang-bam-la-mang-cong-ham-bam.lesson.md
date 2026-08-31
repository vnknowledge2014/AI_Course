---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.bang-bam-la-mang-cong-ham-bam
title: "Bảng băm là một mảng, cộng một hàm băm"
summary: "Lấy số băm của một tên làm chỉ số vào một mảng — cất và tra một giá trị không còn phải dò từng ô, chỉ cần băm cái tên rồi nhảy thẳng tới đúng ô."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.hash-table]
requires: [ds.hash-function, ds.array-index-address, core.function-def, core.function-return, core.function-call, core.function-parameter, core.dict, core.fstring, core.variable, core.assignment]
concepts: [ds.hash-table]
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
Bạn có một hàm biến TÊN thành SỐ. Giờ dùng đúng con số đó làm chỉ số vào
một mảng — xem danh bạ điện thoại của Byte tra cứu nhanh cỡ nào.
::::

::::explain{#mang-cong-ham-bam}
Một **bảng băm** (hash table) chỉ là hai thứ ghép lại, cả hai bạn đã có
sẵn:

- Một **mảng** cỡ cố định (cụm 1) — gọi nó `co_bang` ô, đánh số từ 0.
- **Hàm băm** của bài 19 — biến một cái tên thành một chỉ số nằm gọn trong
  `[0, co_bang - 1]`.

**Chèn** một cặp (tên, giá trị): băm cái tên ra một chỉ số, rồi CẤT giá
trị thẳng vào đúng ô đó.

```python title=readonly
chi_so = bam(ten, co_bang)
bang[chi_so] = gia_tri
```

**Tra cứu**: không dò qua từng ô như tìm trong một mảng chưa sắp xếp,
không đi bộ từng nút như danh sách liên kết (bài 16). Băm LẠI đúng cái
tên đó bằng đúng công thức — chắc chắn ra đúng con số cũ (luật bài 19) —
rồi đọc thẳng ô đó.

```python title=readonly
chi_so = bam(ten, co_bang)
gia_tri = bang[chi_so]
```

Một điều bắt buộc phải giữ: **chỉ_số lúc CHÈN và chỉ_số lúc TRA phải tính
từ cùng một `co_bang`.** Hàm băm nhận `co_bang` làm tham số (bài 19) —
đổi `co_bang` là đổi luôn công thức, ra chỉ số khác hẳn. Chèn với
`co_bang = 10` mà tra với `co_bang = 7` thì con số ra không còn liên quan
gì tới ô đã cất giá trị nữa.
::::

::::example{#danh-ba-dau-tien}
Byte dựng một danh bạ điện thoại — mảng 10 ô, mọi ô bắt đầu rỗng
(`None`) — rồi chèn bốn người:

```python title=readonly
def bam(ten, co_bang):
    return sum(ten.encode("utf-8")) % co_bang

def chen(bang, ten, so_dien_thoai, co_bang):
    chi_so = bam(ten, co_bang)
    bang[chi_so] = {"ten": ten, "so_dien_thoai": so_dien_thoai}

co_bang = 10
danh_ba = [None] * co_bang

chen(danh_ba, "An", "0901111111", co_bang)
chen(danh_ba, "Hoa", "0902222222", co_bang)
chen(danh_ba, "Lan", "0903333333", co_bang)
chen(danh_ba, "Nam", "0904444444", co_bang)

print(danh_ba)
```

```text title=readonly
[{'ten': 'Hoa', 'so_dien_thoai': '0902222222'}, None, None, {'ten': 'Lan', 'so_dien_thoai': '0903333333'}, {'ten': 'Nam', 'so_dien_thoai': '0904444444'}, {'ten': 'An', 'so_dien_thoai': '0901111111'}, None, None, None, None]
```

`"An"` băm ra `5`, nên nằm ở ô 5; `"Hoa"` băm ra `0`, nằm ở ô 0 — mỗi
người rơi đúng vào ô mà công thức bài 19 tính ra, không hơn không kém.
Sáu ô còn lại vẫn `None` — chưa ai chiếm.

Giờ tra cứu — không dò, không đi bộ, băm rồi đọc thẳng:

```python title=readonly
def tra_cuu(bang, ten, co_bang):
    chi_so = bam(ten, co_bang)
    o = bang[chi_so]
    if o is None:
        return None
    return o["so_dien_thoai"]

print(tra_cuu(danh_ba, "Lan", co_bang))
```

```text title=readonly
0903333333
```

`tra_cuu` băm `"Lan"` ra `3`, đọc thẳng `danh_ba[3]` — đúng một phép tính
cộng một lần đọc mảng, xong. Không có vòng lặp nào chạy qua ô 0, 1, 2 để
"tìm" tới ô 3 cả.
::::

::::predict{#doan-nham-co-bang commitOnce}
Byte lỡ tay gõ nhầm khi tra cứu — dùng `co_bang = 7` thay vì `10` (đúng
`co_bang` lúc chèn) khi tính chỉ số cho tên `"Dung"`:

```python
chi_so_sai = bam("Dung", 7)
ket_qua = danh_ba[chi_so_sai]
```

(`danh_ba` vẫn là bảng 10 ô ở ví dụ trên, và `"Dung"` CHƯA được chèn vào
bảng này.)

**Trước khi tính**, bạn đoán `ket_qua` là gì?

:::opt{correct}
`None` — chỉ số tính sai (dùng `co_bang` sai) rơi vào một ô đang còn
trống trong `danh_ba`.
:::

:::opt
Máy dừng lại, báo lỗi vì `7` không phải cỡ bảng thật của `danh_ba`
::why
Gần đúng ở chỗ bạn cảm thấy đúng có gì đó "sai" khi lỡ dùng `co_bang`
khác với lúc chèn — cảnh giác đó không thừa.

Chỗ lệch là Python không hề biết `danh_ba` "thật ra" được chèn bằng
`co_bang` nào — nó chỉ thấy `bam("Dung", 7)` trả về một số nguyên, và số
đó vẫn là một chỉ số HỢP LỆ để đọc `danh_ba` (miễn nằm trong khoảng
0–9). Không có báo lỗi nào cả — máy đọc thẳng một ô, chỉ là ô SAI.
::
:::

:::opt
Một dict `{"ten": ..., "so_dien_thoai": ...}` của một người khác đang có
trong bảng
::why
Gần đúng ở chỗ điều đó CÓ THỂ xảy ra với `co_bang` sai khác — chỉ số lệch
có thể vô tình rơi trúng một ô đã có người ở đó, đọc nhầm số của họ.

Chỗ lệch là với đúng con số này: `bam("Dung", 7)` ra `6`, mà ô 6 của
`danh_ba` chưa ai chiếm — nó vẫn là `None`. Chỉ số sai không PHẢI lúc nào
cũng trúng một ô có người; nó chỉ đơn giản là chỉ số SAI, và sai theo
kiểu nào phụ thuộc vào đúng con số tính ra.
::
:::

:::opt
`"0906666666"` — số điện thoại thật của Dung, vì Python vẫn "nhớ" đúng
người dù `co_bang` gõ nhầm
::why
Gần đúng ở chỗ bạn tin công thức bài 19 đủ mạnh để luôn tìm ra đúng người
— niềm tin đó ĐÚNG, nhưng chỉ khi `co_bang` lúc tra khớp với lúc chèn.

Chỗ lệch là hàm băm không "nhớ" gì cả — nó chỉ tính lại từ đầu mỗi lần
gọi, và `co_bang` là một PHẦN của công thức đó (bài 19: `tong % co_bang`).
Đổi `co_bang` là đổi công thức, ra một con số hoàn toàn khác, không liên
quan gì tới ô đã cất "Dung" (mà ở đây "Dung" thậm chí còn chưa được chèn
vào `danh_ba` này).
::
:::
::::

::::code{#tra-cuu-danh-ba}
Byte đã viết `chen` — hoàn chỉnh, đừng sửa. Bạn hoàn thành `tra_cuu`: chỗ
trống phải tính đúng chỉ số bằng đúng công thức đã băm lúc chèn.

```python title=starter
def bam(ten, co_bang):
    return sum(ten.encode("utf-8")) % co_bang

def chen(bang, ten, so_dien_thoai, co_bang):
    chi_so = bam(ten, co_bang)
    bang[chi_so] = {"ten": ten, "so_dien_thoai": so_dien_thoai}

def tra_cuu(bang, ten, co_bang):
    chi_so = ___
    o = bang[chi_so]
    if o is None:
        return None
    return o["so_dien_thoai"]

co_bang = 10
danh_ba = [None] * co_bang
chen(danh_ba, "An", "0901111111", co_bang)
chen(danh_ba, "Hoa", "0902222222", co_bang)
chen(danh_ba, "Lan", "0903333333", co_bang)
chen(danh_ba, "Dung", "0906666666", co_bang)

print(tra_cuu(danh_ba, "Lan", co_bang))
print(tra_cuu(danh_ba, "Dung", co_bang))
print(tra_cuu(danh_ba, "Hoa", co_bang))
```

```python title=solution
def bam(ten, co_bang):
    return sum(ten.encode("utf-8")) % co_bang

def chen(bang, ten, so_dien_thoai, co_bang):
    chi_so = bam(ten, co_bang)
    bang[chi_so] = {"ten": ten, "so_dien_thoai": so_dien_thoai}

def tra_cuu(bang, ten, co_bang):
    chi_so = bam(ten, co_bang)
    o = bang[chi_so]
    if o is None:
        return None
    return o["so_dien_thoai"]

co_bang = 10
danh_ba = [None] * co_bang
chen(danh_ba, "An", "0901111111", co_bang)
chen(danh_ba, "Hoa", "0902222222", co_bang)
chen(danh_ba, "Lan", "0903333333", co_bang)
chen(danh_ba, "Dung", "0906666666", co_bang)

print(tra_cuu(danh_ba, "Lan", co_bang))
print(tra_cuu(danh_ba, "Dung", co_bang))
print(tra_cuu(danh_ba, "Hoa", co_bang))
```

```python title=test
assert tra_cuu(danh_ba, "Lan", co_bang) == "0903333333", "tra_cuu('Lan') phải trả đúng số của Lan"
assert tra_cuu(danh_ba, "Dung", co_bang) == "0906666666", "tra_cuu('Dung') phải trả đúng số của Dung — chỉ số của Dung (8) khác chỉ số của Lan (3), phải tính LẠI cho từng tên"
assert tra_cuu(danh_ba, "Hoa", co_bang) == "0902222222", "tra_cuu('Hoa') phải trả đúng số của Hoa"
assert tra_cuu(danh_ba, "Vy", co_bang) is None, "'Vy' chưa từng được chèn — ô của nó vẫn None, tra_cuu phải trả về None, không phải số của ai khác"

# Một bảng KHÁC HẲN, cỡ 3 — nếu chỗ trống hardcode co_bang=10 (thay vì dùng đúng
# tham số co_bang được truyền vào), nó sẽ tính chỉ số bằng công thức của bảng
# SAI, đọc lệch ô hoặc vỡ ra khỏi biên mảng cỡ 3.
danh_ba_nho = [None] * 3
chen(danh_ba_nho, "Vy", "0908888888", 3)
assert tra_cuu(danh_ba_nho, "Vy", 3) == "0908888888", "tra_cuu phải tính chỉ số bằng ĐÚNG co_bang được truyền vào — ở đây là 3 (một cỡ bảng khác hẳn 10 đã dùng ở trên), không phải hardcode con số 10"
```

:::hints
- kind: attention
  body: Nhìn lại hàm `chen` ngay phía trên — nó tính `chi_so` bằng cách nào để CẤT giá trị vào đúng ô? `tra_cuu` phải tính chỉ số bằng ĐÚNG công thức đó, để đọc lại đúng ô đã cất.
- kind: strategy
  body: "Chỗ trống nhận hai thứ đang có sẵn trong hàm: `ten` (tên cần tra) và `co_bang` (cỡ bảng, phải khớp với lúc chèn). Gọi `bam` với đúng hai tham số này, giống hệt dòng đầu tiên của `chen`."
- kind: one-line
  body: 'Điền `bam(ten, co_bang)` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^0903333333\\n0906666666\\n0902222222\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Băm cái tên, đọc thẳng một ô — không dò, không đi bộ. Bốn người, bốn phép
tra cứu, mỗi lần đúng một bước nhảy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bảng của bạn có 10 ô. Số tên có thể đặt cho một người thì gần như KHÔNG
GIỚI HẠN — "An", "Ân", "Àn", "Ann", hàng triệu cách viết khác nhau.

Nếu số CÁI TÊN CÓ THỂ CÓ nhiều hơn hẳn số Ô của bảng — mà chắc chắn là
vậy — thì có bao giờ HAI tên khác nhau lại băm ra CÙNG một ô không? Và
nếu có, chuyện gì xảy ra với cái đã cất ở đó từ trước?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
