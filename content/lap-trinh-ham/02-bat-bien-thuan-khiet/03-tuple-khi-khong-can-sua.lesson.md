---
id: lap-trinh-ham.bat-bien-thuan-khiet.tuple-khi-khong-can-sua
title: "`tuple` khi bạn không cần sửa — Python tự chặn giúp"
summary: "tuple không phải Python 'khuyên' bạn đừng sửa — nó THẬT SỰ KHÔNG CÓ phương thức sửa tại chỗ nào. Công cụ đầu tiên trong track này có CHẶN THẬT, không chỉ là quy ước bạn tự nhớ."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.tuple-immutable]
requires: [fp.return-new-data]
concepts: [fp.tuple-immutable]
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
Bài trước để lại một câu hỏi: có kiểu dữ liệu nào khiến việc sửa tại chỗ
KHÔNG VIẾT ĐƯỢC — không chỉ dựa vào trí nhớ? Có. Bạn đã dùng nó từ lâu,
chỉ chưa để ý tới đúng công dụng này của nó.
::::

::::explain{#khong-phai-khuyen-ma-la-khong-co}
`list` cho phép sửa tại chỗ vì nó CÓ những phương thức làm việc đó —
`.append`, `.insert`, `.remove`, gán vào một ô. Muốn một dữ liệu không
sửa được tại chỗ, cách chắc nhất không phải "tự nhắc mình đừng gọi mấy
phương thức đó" — mà là dùng một kiểu dữ liệu VỐN KHÔNG CÓ chúng.

Đó chính là `tuple` — cùng là một dãy có thứ tự như `list`, đọc bằng chỉ
số, duyệt bằng `for`, đo độ dài bằng `len()`, chỉ khác cách viết: dấu
ngoặc TRÒN `(...)` thay vì ngoặc VUÔNG `[...]`. Nhưng `tuple` không hề
có `.append`, `.insert`, `.remove`, `.sort`, hay bất kỳ phương thức sửa
tại chỗ nào — không phải Python CHẶN chúng, mà đơn giản là kiểu dữ liệu
này CHƯA BAO GIỜ ĐƯỢC ĐỊNH NGHĨA CÓ chúng.

Sự khác biệt đó quan trọng: một QUY ƯỚC (như "đừng gọi `.append`") có
thể bị quên. Một phương thức KHÔNG TỒN TẠI thì không ai gọi nhầm được —
gọi thử, Python báo lỗi ngay lập tức, rõ ràng, không đợi tới lúc chạy
sai kết quả rồi mới phát hiện ra.
::::

::::example{#size-ly-co-dinh}
Một quán trà sữa chỉ bán ba size cố định trong suốt một ngày — không
thêm size nào giữa chừng, dù bận rộn tới đâu.

```python title=readonly
size_ly = ("nhỏ", "vừa", "lớn")

print(size_ly[0])
print(len(size_ly))

size_ly.append("khổng lồ")
```

```text title=readonly
nhỏ
3
```

```text title=readonly
AttributeError: 'tuple' object has no attribute 'append'
```

Hai dòng đầu chạy bình thường — đọc theo chỉ số và đo độ dài hoạt động
y hệt `list`. Dòng thứ ba mới lộ ra khác biệt: gọi `.append(...)` trên
`size_ly` khiến Python từ chối NGAY LẬP TỨC, bằng chính thông báo lỗi
thật ở trên — không phải một cảnh báo, mà là chương trình DỪNG LẠI tại
đúng dòng đó.
::::

::::predict{#doan-hai-loai-loi commitOnce}
Đoạn mã dưới đây thử HAI cách "sửa" khác nhau trên cùng một `tuple`.

**Trước khi chạy**, bạn đoán chuyện gì xảy ra?

```python
size_ly = ("nhỏ", "vừa", "lớn")

them_size = size_ly + ("khổng lồ",)
print(them_size)

size_ly[0] = "mini"
print(size_ly)
```

:::opt{correct}
Dòng `print(them_size)` in ra `('nhỏ', 'vừa', 'lớn', 'khổng lồ')`. Sau
đó máy dừng lại, báo `TypeError: 'tuple' object does not support item
assignment` ở dòng `size_ly[0] = "mini"` — dòng `print(size_ly)` cuối
không kịp chạy
:::

:::opt
Cả hai dòng đều in ra bình thường, dòng cuối in
`('mini', 'vừa', 'lớn')`
::why
Gần đúng ở việc bạn tin `size_ly[0] = ...` là cú pháp hợp lệ trong
Python — đúng, với `list` thì cú pháp này chạy tốt.

Chỗ lệch: `size_ly` là `tuple`, không phải `list`. Gán vào MỘT Ô của
`tuple` không có cách nào viết được — Python từ chối ngay bằng
`TypeError: 'tuple' object does not support item assignment`. Dòng
`print(size_ly)` không bao giờ chạy tới.
::
:::

:::opt
Máy báo lỗi ngay từ dòng `them_size = size_ly + ("khổng lồ",)`, vì
tuple không cho phép cộng thêm phần tử
::why
Gần đúng ở việc bạn cảnh giác đúng hướng — `tuple` thật sự chặn một số
phép toán.

Chỗ lệch: `+` giữa hai `tuple` không SỬA TẠI CHỖ — nó TẠO một `tuple`
HOÀN TOÀN MỚI (`them_size`), giữ nguyên `size_ly` cũ y hệt trước đó.
Phép này chạy trót lọt, in ra `('nhỏ', 'vừa', 'lớn', 'khổng lồ')` bình
thường. Lỗi thật chỉ xảy ra ở dòng SAU, khi gán vào một ô có sẵn.
::
:::

:::opt
Máy dừng lại ngay từ dòng `size_ly[0] = "mini"`, báo cùng lỗi
`AttributeError` như khi gọi `.append()`
::why
Gần đúng ở việc bạn nhớ đúng: `tuple` thật sự chặn phép sửa này — trực
giác "đây cũng là một kiểu sửa bị chặn" không sai.

Chỗ lệch: đây là MỘT LOẠI thao tác khác — gán vào một Ô có sẵn
(`size_ly[0] = ...`), không phải gọi một PHƯƠNG THỨC không tồn tại
(`.append()`). Python báo hai loại lỗi khác nhau cho hai tình huống này:
thiếu phương thức thì `AttributeError`, còn cố gán vào ô thì `TypeError`
— cùng nói "tuple không sửa được", nhưng khác hẳn chữ chẩn đoán.
::
:::
::::

::::code{#khai-size-co-dinh}
Byte cần khai đúng danh sách size của quán — ba size, cố định suốt
ngày. Điền chỗ trống bằng một kiểu dữ liệu khiến việc "thêm size" không
viết được, chứ không chỉ là một quy ước bằng lời.

```python title=starter
size_ly = ___          # "nhỏ", "vừa", "lớn" — cố định suốt một ngày bán

print(f"Các size đang bán: {size_ly}")
```

```python title=solution
size_ly = ("nhỏ", "vừa", "lớn")

print(f"Các size đang bán: {size_ly}")
```

```python title=test
assert size_ly == ("nhỏ", "vừa", "lớn"), "size_ly phải đúng ba size này: nhỏ, vừa, lớn"
assert isinstance(size_ly, tuple), "size_ly phải là một tuple — không phải list — để .append() không viết được"
da_chan_dung = False
ten_loi = None
try:
    size_ly.append("khổng lồ")
except AttributeError as loi:
    da_chan_dung = True
    ten_loi = type(loi).__name__
assert da_chan_dung, "gọi size_ly.append(...) phải bị chặn ngay — tuple không có phương thức đó"
assert ten_loi == "AttributeError", f"lỗi ném ra phải là AttributeError, đang là {ten_loi}"
```

:::hints
- kind: attention
  body: Chỗ trống là kiểu dữ liệu cho size_ly — ba size cố định, đúng thứ tự "nhỏ", "vừa", "lớn".
- kind: strategy
  body: Dùng dấu ngoặc TRÒN () thay vì ngoặc VUÔNG [] — đó chính là cách khai một tuple, kiểu dữ liệu không có .append().
- kind: one-line
  body: 'Điền `("nhỏ", "vừa", "lớn")` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Các size đang bán: ('nhỏ', 'vừa', 'lớn')"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`tuple` không "khuyên" bạn đừng sửa — nó thật sự không có cách nào để
sửa. Đó là công cụ bất biến ĐẦU TIÊN của track này.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`tuple` chặn sửa tại chỗ triệt để — nhưng nó chỉ có CHỈ SỐ (`size_ly[0]`,
`size_ly[1]`...), không có TÊN TRƯỜNG rõ nghĩa. Muốn nhóm dữ liệu có tên
gọi hẳn hoi — như một "món ăn" với tên và giá riêng biệt, đọc bằng
`.ten`, `.gia` thay vì chỉ số 0, 1 — Python có một công cụ khác.

Công cụ đó có bất biến sẵn như `tuple` không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
