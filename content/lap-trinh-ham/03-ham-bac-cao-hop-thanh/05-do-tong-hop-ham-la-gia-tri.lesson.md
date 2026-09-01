---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.do-tong-hop-ham-la-gia-tri
title: "Đo tổng hợp: hàm là giá trị"
summary: "Ghép cả bốn Ý của cụm 1 — một dict ánh xạ tên sang hàm (lambda), một hàm nhận hàm làm đối số qua dict, gọi nó, trả về kết quả. Không khái niệm mới."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [fp.review-first-class]
requires: [fp.lambda-basics]
concepts: [fp.review-first-class]
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
Bốn bài, bốn mảnh: hàm gán vào tên, truyền vào hàm khác, trả về từ hàm
khác, viết gọn bằng `lambda`. Hôm nay ghép cả bốn vào MỘT bài.
::::

::::explain{#ghep-lai}
Không khái niệm mới — chỉ ghép lại những gì đã có:

```python
chinh_sach = {
    "thuong": lambda gia: gia,
    "vip": lambda gia: gia * 0.8,
}

def tinh_gia_cuoi(loai_khach, gia_goc, chinh_sach):
    ham = chinh_sach[loai_khach]   # tra dict (bài 1) — LẤY hàm ra
    return ham(gia_goc)            # gọi hàm vừa lấy (bài 2 — hàm là tham số)

print(tinh_gia_cuoi("vip", 100000, chinh_sach))
```

```text
80000.0
```

`chinh_sach` là một `dict` GIỮ HÀM (bài 1: `phep_toan`). `tinh_gia_cuoi`
NHẬN `chinh_sach` làm THAM SỐ (bài 2: hàm không cần biết trước nó chứa
gì). Cả hai mảnh khớp lại: tra đúng khoá, lấy đúng hàm, gọi đúng hàm —
không một dòng `if/elif` nào đếm từng loại khách hàng.
::::

::::predict{#doan-them-loai-khach-moi commitOnce}
```python
chinh_sach = {
    "thuong": lambda gia: gia,
    "vip": lambda gia: gia * 0.8,
}

def tinh_gia_cuoi(loai_khach, gia_goc, chinh_sach):
    return chinh_sach[loai_khach](gia_goc)

chinh_sach["sinh_vien"] = lambda gia: gia * 0.7

print(tinh_gia_cuoi("sinh_vien", 100000, chinh_sach))
```

`tinh_gia_cuoi` KHÔNG hề được sửa sau khi định nghĩa. Dòng cuối in ra
gì?

:::opt{correct}
`70000.0`
:::

:::opt
Máy báo lỗi — `tinh_gia_cuoi` không "biết" khoá `"sinh_vien"` vì nó
không có mặt lúc hàm được định nghĩa
::why
Gần đúng ở việc bạn cẩn trọng về thứ hàm "biết" lúc định nghĩa — phản xạ
đúng cho nhiều tình huống lập trình khác.

Chỗ lệch: `tinh_gia_cuoi` không cần "biết trước" các khoá — nó chỉ tra
`chinh_sach[loai_khach]` NGAY LÚC ĐƯỢC GỌI, không phải lúc định nghĩa.
Tới lúc gọi `tinh_gia_cuoi("sinh_vien", ...)`, `chinh_sach` ĐÃ có thêm
khoá `"sinh_vien"` (thêm ở dòng trước đó), nên tra được bình thường.
::
:::

:::opt
`100000` — vì thêm một khoá mới vào `chinh_sach` sau khi hàm đã định
nghĩa không có tác dụng, nên `tinh_gia_cuoi` trả về gia_goc không đổi
::why
Gần đúng ở việc bạn nghi ngờ đúng chỗ đáng nghi: "sửa dict SAU khi định
nghĩa hàm" nghe như một hành động muộn màng.

Chỗ lệch: `chinh_sach` là một `dict` — SỬA ĐƯỢC tại chỗ bất cứ lúc nào
(`chinh_sach["sinh_vien"] = ...` thêm một khoá mới, đúng cú pháp gán vào
ô của T4.1). `tinh_gia_cuoi` tra `chinh_sach` value MỚI NHẤT tại THỜI
ĐIỂM GỌI, không phải một bản chụp cũ từ lúc định nghĩa.
::
:::
::::

::::code{#tra-cuu-va-goi-chinh-sach}
Viết `tinh_gia_cuoi(loai_khach, gia_goc, chinh_sach)` — TRA đúng hàm
trong `chinh_sach` theo `loai_khach`, rồi GỌI hàm đó với `gia_goc`.

```python title=starter
chinh_sach = {
    "thuong": lambda gia: gia,
    "vip": lambda gia: gia * 0.8,
    "nhan_vien": lambda gia: gia * 0.5,
}

def tinh_gia_cuoi(loai_khach, gia_goc, chinh_sach):
    ___

print(tinh_gia_cuoi("vip", 100000, chinh_sach))
print(tinh_gia_cuoi("nhan_vien", 100000, chinh_sach))
```

```python title=solution
chinh_sach = {
    "thuong": lambda gia: gia,
    "vip": lambda gia: gia * 0.8,
    "nhan_vien": lambda gia: gia * 0.5,
}

def tinh_gia_cuoi(loai_khach, gia_goc, chinh_sach):
    return chinh_sach[loai_khach](gia_goc)

print(tinh_gia_cuoi("vip", 100000, chinh_sach))
print(tinh_gia_cuoi("nhan_vien", 100000, chinh_sach))
```

```python title=test
assert tinh_gia_cuoi("vip", 100000, chinh_sach) == 80000.0, "vip phải giảm đúng 20%"
assert tinh_gia_cuoi("nhan_vien", 100000, chinh_sach) == 50000.0, "nhan_vien phải giảm đúng 50% — KHÔNG được viết cứng if/elif chỉ xử lý vip"
assert tinh_gia_cuoi("thuong", 30000, chinh_sach) == 30000, "thuong giữ nguyên giá gốc"
```

:::hints
- kind: attention
  body: Chỗ trống là toàn bộ thân hàm — tra chinh_sach[loai_khach] để LẤY hàm, rồi gọi hàm đó với gia_goc.
- kind: strategy
  body: 'chinh_sach[loai_khach] tra ra đúng hàm (một lambda). Thêm (gia_goc) ngay sau để GỌI hàm đó — return chinh_sach[loai_khach](gia_goc).'
- kind: one-line
  body: "return chinh_sach[loai_khach](gia_goc)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: tinh_gia_cuoi phải TRA CỨU qua tham số chinh_sach (dict), không viết cứng if/elif liệt kê từng loại khách hàng.
  requireAst:
  # min:3, không phải 1: uses-name đếm trên TOÀN BỘ mã nộp, không chỉ
  # thân hàm — hai lời gọi print(...) ở ngoài (không đổi khi điền bừa)
  # đã tự có 2 lần đọc chinh_sach. Lời giải thật cộng thêm 1 lần đọc
  # TRONG thân hàm = 3. min:1 sẽ luôn đạt dù thân hàm rỗng — đã đo thật.
  - kind: uses-name, target: chinh_sach, min: 3
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "80000.0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn mảnh nhỏ, ghép lại thành một cách viết KHÔNG có `if/elif` đếm từng
trường hợp — thêm loại khách mới chỉ cần thêm một dòng vào `chinh_sach`,
không sửa `tinh_gia_cuoi`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã tự viết `ap_dung(xs, f)` — áp một hàm lên từng phần tử một `list`.
Python có SẴN một hàm làm đúng việc đó không, khỏi phải tự viết?

Bài sau trả lời — và mở đầu ba công cụ nền tảng của track này:
`map`/`filter`/`reduce`.
::::

::::checkpoint{mastery=0.8}
::::
