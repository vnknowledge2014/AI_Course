---
id: khoa-hoc-may-tinh.may-chay-the-nao.gia-tri-python-nam-o-dau
title: "Giá trị Python nằm ở đâu — ngăn xếp hay đống"
summary: "Mọi object Python (list, dict, chuỗi, kể cả số nguyên) sống trên ĐỐNG cấp phát; cái tên biến chỉ là một ô nhỏ trên khung của ngăn xếp gọi hàm, giữ ĐỊA CHỈ trỏ sang đống — đúng hệt T3.1 đã dạy, giờ biết chính xác nó nằm ở vùng nào."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.value-location]
requires: [may.heap-memory-vs-ds-heap, mem.name-is-reference]
concepts: [may.value-location]
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
Cùng chỗ với giá trị nó trỏ tới? Không đâu — hai vùng khác hẳn nhau.
::::

::::explain{#ten-la-cai-o-tren-khung}
T3.1 đã dạy một điều quan trọng: một cái TÊN trong Python không GIỮ giá
trị — nó chỉ CHỈ TỚI giá trị (`mem.name-is-reference`). Giờ track này
cho biết chính xác điều đó nghĩa là gì, VỀ MẶT VẬT LÝ:

- Mọi **giá trị** Python — một `list`, một `dict`, một chuỗi, kể cả một
  số nguyên bạn vừa tạo ra — sống trên **đống cấp phát** (bài 24).
- Cái **tên** bạn gõ (`don_hang`, `gia`, `ket_qua`,...) chỉ là MỘT Ô NHỎ
  nằm trên **khung** (bài 12) của lời gọi hiện tại — mà khung đó nằm
  trên **ngăn xếp gọi hàm** (bài 23).
- Ô nhỏ đó không giữ bản sao của giá trị. Nó giữ một **địa chỉ** — một
  con số trỏ sang đúng chỗ trên đống nơi giá trị thật đang nằm.

Hai tên khác nhau hoàn toàn có thể giữ CÙNG một địa chỉ — trỏ vào đúng
một object trên đống. Đó chính xác là chuyện "aliasing" T3.1 đã dạy với
`list`: `a is b` đúng nghĩa là "ô của `a` và ô của `b` đang giữ cùng
một địa chỉ". Không có gì huyền bí — chỉ là hai ô trên (có thể) hai
khung khác nhau, cùng trỏ một chỗ trên đống.
::::

::::example{#hai-ten-cung-mot-dia-chi}
Hai tên, một object:

```python title=readonly
danh_sach_a = [1, 2, 3]
danh_sach_b = danh_sach_a       # tên MỚI, KHÔNG object mới

print(id(danh_sach_a) == id(danh_sach_b))
print(danh_sach_a is danh_sach_b)

danh_sach_a.append(4)
print(danh_sach_b)
```

```text title=readonly
True
True
[1, 2, 3, 4]
```

`danh_sach_b = danh_sach_a` không tạo một `list` mới trên đống — nó chỉ
tạo một Ô MỚI (`danh_sach_b`) trên khung hiện tại, và ghi vào ô đó ĐÚNG
địa chỉ mà ô `danh_sach_a` đang giữ. `id(...)` — hàm đọc thẳng con số
địa chỉ đó — xác nhận cả hai giống hệt nhau, và `is` xác nhận lại theo
cách quen thuộc hơn. Sửa qua `danh_sach_a` (`.append(4)`), giá trị thật
trên đống đổi — và `danh_sach_b`, đang trỏ ĐÚNG chỗ đó, thấy sự đổi ấy
ngay lập tức.
::::

::::predict{#doan-doi-qua-ten-thu-hai commitOnce}
```python
gio_hang_a = ["áo"]
gio_hang_b = gio_hang_a

gio_hang_b.append("quần")

print(gio_hang_a)
```

**Trước khi chạy**, bạn đoán dòng in ra là gì?

:::opt{correct}
`['áo', 'quần']` — sửa qua `gio_hang_b` vẫn ăn lên `gio_hang_a`, vì cả
hai cùng trỏ một object trên đống
:::

:::opt
`['áo']` — vì `gio_hang_a` không hề được sửa trực tiếp, chỉ
`gio_hang_b` mới bị `.append`
::why
Gần đúng ở việc bạn đọc đúng DÒNG LỆNH: đúng là chỉ `gio_hang_b` xuất
hiện ở dòng `.append("quần")`, không có dòng nào viết `gio_hang_a.append(...)`.

Chỗ lệch: "sửa qua tên nào" không quan trọng bằng "sửa vào ĐỊA CHỈ nào".
`gio_hang_b = gio_hang_a` làm hai ô trên khung cùng giữ MỘT địa chỉ —
không phải hai object riêng. `.append` gọi qua `gio_hang_b` vẫn sửa
đúng cái object DUY NHẤT đang tồn tại, và `gio_hang_a` trỏ đúng chỗ đó
nên thấy sự đổi ngay.
::
:::

:::opt
Lỗi — không thể `.append` qua `gio_hang_b` vì nó không phải tên "chính
chủ" của danh sách
::why
Gần đúng ở việc bạn nghĩ có một khái niệm "ai là chủ" — bản năng đó
không sai khi nghĩ về sở hữu dữ liệu nói chung.

Chỗ lệch: Python không có khái niệm "tên chính chủ" cho một object trên
đống. Mọi tên trỏ tới cùng một địa chỉ đều có quyền như nhau — không tên
nào "chính", không tên nào "phụ". `gio_hang_b.append(...)` hợp lệ y hệt
`gio_hang_a.append(...)`, vì cả hai đang thao tác trên đúng MỘT object.
::
:::

:::opt
`['áo', 'quần']` cho `gio_hang_a`, nhưng `gio_hang_b` lúc này đã tách
riêng, không còn liên quan tới `gio_hang_a` nữa
::why
Gần đúng ở kết quả IN RA — đúng là `['áo', 'quần']`, con số đó không
sai.

Chỗ lệch nằm ở phần diễn giải "tách riêng". Không có sự tách nào xảy
ra: `gio_hang_a` và `gio_hang_b` VẪN đang giữ đúng một địa chỉ, trỏ
đúng một object, suốt từ đầu tới cuối đoạn mã. Chúng không "tách" — bởi
chưa dòng nào gán lại `gio_hang_b` sang một object khác.
::
:::
::::

::::code{#sua-qua-ten-thu-hai}
Một hàm nhận `don_hang`, thêm món vào, rồi trả về CHÍNH object đó (không
tạo bản sao nào) — đúng vì `don_hang` bên trong hàm chỉ là một ô trên
khung của lời gọi này, giữ ĐỊA CHỈ trỏ sang list đã tồn tại từ trước.

Gọi lại hàm này một lần nữa, lần này QUA TÊN `ket_qua` — tên thứ hai mà
lời gọi đầu tiên vừa trả về — rồi xem sự thay đổi có ăn lên `don_that`
hay không.

```python title=starter
def them_mon(don_hang, mon_moi):
    don_hang.append(mon_moi)
    return don_hang

don_that = ["phở"]
ket_qua = them_mon(don_that, "trà đá")

# don_that và ket_qua chỉ là hai TÊN khác nhau, cùng trỏ một địa chỉ
# trên đống — is xác nhận điều đó
cung_mot_dia_chi = don_that is ket_qua

them_qua_ten_thu_hai = ___          # gọi lại them_mon, QUA TÊN ket_qua,
                                     # thêm "cà phê"

print(don_that)
print(cung_mot_dia_chi)
```

```python title=solution
def them_mon(don_hang, mon_moi):
    don_hang.append(mon_moi)
    return don_hang

don_that = ["phở"]
ket_qua = them_mon(don_that, "trà đá")

cung_mot_dia_chi = don_that is ket_qua

them_qua_ten_thu_hai = them_mon(ket_qua, "cà phê")

print(don_that)
print(cung_mot_dia_chi)
```

```python title=test
assert don_that == ["phở", "trà đá", "cà phê"], f"don_that phải thấy đủ CẢ HAI món thêm — dù món thứ hai thêm QUA TÊN ket_qua — đang ra {don_that}"
assert cung_mot_dia_chi is True, "don_that và ket_qua phải CÙNG một địa chỉ trên đống, vì them_mon trả về chính object nó nhận vào, không tạo bản sao"
```

:::hints
- kind: attention
  body: Chỉ một chỗ trống — một lời GỌI lại hàm them_mon, nhưng lần này phải đi QUA TÊN ket_qua, không phải qua don_that.
- kind: strategy
  body: 'Gọi them_mon(ket_qua, "cà phê") — đúng hàm cũ, đúng món mới, chỉ khác tên biến đưa vào là ket_qua. Vì ket_qua và don_that cùng trỏ một object, sửa qua ket_qua vẫn ăn lên don_that.'
- kind: one-line
  body: 'Chỗ trống là: them_mon(ket_qua, "cà phê")'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ gọi them_mon(...) lần thứ hai, và phải đi QUA TÊN ket_qua — không gọi lại qua don_that, vì bài này đang chứng minh sửa qua TÊN THỨ HAI vẫn ăn lên tên đầu tiên
  requireAst:
  # uses-call min: 2 — đếm thật trên solution: them_mon bị GỌI đúng 2 lần
  # (ket_qua = them_mon(...) đã có sẵn trong khung + chỗ trống).
  # uses-name target ket_qua min: 2 — đếm thật trên solution: ket_qua được
  # ĐỌC đúng 2 lần (dòng cung_mot_dia_chi đã có sẵn trong khung + chỗ
  # trống). Một lời giải khác NHƯNG SAI TRỌNG TÂM — gọi lại
  # them_mon(don_that, "cà phê") thay vì qua ket_qua — vẫn cho ĐÚNG kết quả
  # in ra (don_that giống hệt), nên KHÔNG cổng run/tests/output nào phân
  # biệt được; chỉ luật uses-name target ket_qua min:2 mới chặn được, vì
  # bản đó chỉ đọc ket_qua đúng 1 lần (thiếu lần ở chỗ trống) — ĐÃ THỬ THẬT,
  # đếm được 1, dưới 2, bị chặn đúng. Đề bài đã nói rõ "QUA TÊN ket_qua" nên
  # cổng hẹp này khớp đúng điều đề bài yêu cầu.
  - kind: uses-call, target: them_mon, min: 2
  - kind: uses-name, target: ket_qua, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\['phở', 'trà đá', 'cà phê'\\]\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
"Cà phê" thêm QUA `ket_qua` — mà `don_that` thấy ngay. Hai tên, một địa
chỉ, một giá trị thật trên đống.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã học ở T3.3 rằng đệ quy đi quá sâu sẽ chạm một mức TRẦN —
`RecursionError`. Ngăn xếp gọi hàm (bài 23) có trần.

Đống — chỗ chứa MỌI giá trị bạn tạo ra, list này, dict kia — có trần
tương tự không? Hay nó phình to tới đâu cũng được, không giới hạn nào
cả?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
