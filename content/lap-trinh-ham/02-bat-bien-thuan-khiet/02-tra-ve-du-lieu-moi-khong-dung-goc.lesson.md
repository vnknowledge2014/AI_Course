---
id: lap-trinh-ham.bat-bien-thuan-khiet.tra-ve-du-lieu-moi-khong-dung-goc
title: "Trả về dữ liệu MỚI, không đụng vào dữ liệu gốc"
summary: "Viết lại hàm sửa tại chỗ bằng một list comprehension: TẠO một list mới thay vì sửa list cũ — không hỏi 'hàm có đúng không', mà hỏi 'hàm có đúng VÀ có để yên input không'."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.return-new-data]
requires: [fp.mutation-surprise]
concepts: [fp.return-new-data]
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
Bài trước hỏi: có cách nào tính đúng mà không đụng list gốc không? Có —
và cách đó không cần một dòng lệnh SỬA nào cả.
::::

::::explain{#tao-moi-thay-vi-sua}
Vấn đề bài trước nằm ở đúng MỘT dòng: `gia[i] += phi` — một phép SỬA
TẠI CHỖ, viết đè lên đúng cái nồi mà tham số đang trỏ tới. Muốn hàm
không chạm vào list gốc, cách chắc chắn nhất không phải "cẩn thận hơn"
khi sửa — mà là ĐỪNG SỬA GÌ CẢ. Thay vào đó, dựng hẳn một list MỚI, chứa
kết quả đã tính, rồi trả về đúng list mới đó.

Python có sẵn một cú pháp gọn cho việc này: **list comprehension**
(gộp lại thành một list bằng một biểu thức duyệt). Viết
`[g + phi for g in gia]` nghĩa là: với MỖI phần tử `g` trong `gia`, tính
`g + phi`, rồi gom tất cả kết quả đó vào một list HOÀN TOÀN MỚI — không
đọc hay ghi bất kỳ ô nào của `gia`, chỉ ĐỌC từng giá trị của nó rồi thôi.

Khuôn mẫu này sẽ lặp lại suốt phần còn lại của track: một hàm ĐÚNG không
chỉ cần TÍNH đúng số — nó còn phải để nguyên dữ liệu được đưa vào. Từ
giờ, câu hỏi không còn là "hàm có đúng không", mà là "hàm có đúng VÀ có
để yên input không".
::::

::::example{#hai-ham-hai-ung-xu}
Cùng bài toán bài trước — cộng thêm phí giao hàng — viết lại bằng list
comprehension:

```python title=readonly
def tra_ve_gia_da_cong_phi(gia, phi):
    return [g + phi for g in gia]

bang_gia = [40000, 55000, 60000]
gia_giao_hang = tra_ve_gia_da_cong_phi(bang_gia, 5000)

print(f"Giá giao hàng: {gia_giao_hang}")
print(f"Bảng giá gốc:  {bang_gia}")
print(f"Cùng một list không: {bang_gia is gia_giao_hang}")
```

```text title=readonly
Giá giao hàng: [45000, 60000, 65000]
Bảng giá gốc:  [40000, 55000, 60000]
Cùng một list không: False
```

Kết quả tính ra ĐÚNG Y HỆT bài trước — `[45000, 60000, 65000]`. Khác
biệt duy nhất nằm ở dòng thứ hai: `bang_gia` giờ giữ nguyên giá trị gốc,
vì thân hàm không còn dòng sửa tại chỗ nào cả. Dòng cuối xác nhận bằng
`is`: `gia_giao_hang` là một list HOÀN TOÀN KHÁC, không trùng địa chỉ
với `bang_gia` — đúng nghĩa "list mới", không phải "cùng list, nội dung
khác".
::::

::::predict{#doan-giam-gia-thanh-ly commitOnce}
Một hàm áp dụng giảm giá bằng list comprehension — không sửa tại chỗ.

**Trước khi chạy**, bạn đoán hai dòng cuối in ra gì?

```python
def ap_dung_giam_gia(gia, muc_giam):
    gia_moi = [g - muc_giam for g in gia]
    return gia_moi

gia_ban = [100000, 150000, 120000]
gia_sale = ap_dung_giam_gia(gia_ban, 20000)

print(f"gia_ban:  {gia_ban}")
print(f"gia_sale: {gia_sale}")
```

:::opt{correct}
`gia_ban:  [100000, 150000, 120000]` và
`gia_sale: [80000, 130000, 100000]`
:::

:::opt
`gia_ban:  [80000, 130000, 100000]` và
`gia_sale: [80000, 130000, 100000]` — cả hai đều đổi, giống bài trước
::why
Gần đúng ở chỗ `gia_sale` bạn tính đúng — ba con số đó khớp hoàn toàn
với kết quả thật.

Chỗ lệch là `gia_ban`. Thân hàm không còn dòng `gia[i] = ...` hay
`gia[i] -= ...` nào nữa — `gia_moi = [g - muc_giam for g in gia]` chỉ
ĐỌC từng phần tử của `gia`, không GHI vào ô nào của nó cả. Không có
phép sửa tại chỗ nào để mà chạm ra ngoài, nên `gia_ban` phải giữ nguyên.
::
:::

:::opt
`gia_ban:  [80000, 130000, 100000]` và
`gia_sale: [100000, 150000, 120000]` — ngược lại, gia_ban đổi còn
gia_sale giữ nguyên
::why
Gần đúng ở phần khó nhất: bạn nhận ra một bên phải đổi, một bên phải
giữ nguyên — đúng là bài này có một bên đổi thật.

Chỗ lệch là bạn gán ngược vai trò. `gia_ban` là tham số ĐƯA VÀO — hàm
chỉ ĐỌC nó, không sửa gì. `gia_sale` mới là biến GIỮ KẾT QUẢ trả về —
list MỚI, mang giá trị đã giảm. Đổi lại đúng chỗ: `gia_ban` đứng yên,
`gia_sale` là bên có giá đã giảm.
::
:::

:::opt
`gia_ban:  [100000, 150000, 120000]` và
`gia_sale: [100000, 150000, 120000]` — cả hai giống nhau, vì gia_sale
chỉ là một bản sao chưa qua tính toán
::why
Gần đúng ở việc `gia_ban` bạn tính đúng — nó thật sự giữ nguyên, không
đổi.

Chỗ lệch nằm ở `gia_sale`. List comprehension không chỉ "sao chép" —
biểu thức `g - muc_giam` THẬT SỰ chạy cho từng phần tử, trừ đi
`muc_giam` trước khi gom vào list mới. `gia_sale` phải mang giá trị ĐÃ
GIẢM, không phải bản sao y nguyên của `gia_ban`.
::
:::
::::

::::code{#giao-hang-khong-dung-goc}
Viết lại đúng bài toán "cộng phí giao hàng" — lần này KHÔNG được đụng
vào bảng giá gốc. Điền một dòng duy nhất: thân hàm, trả về một list MỚI.

```python title=starter
def tang_gia_khong_dung_goc(gia, phi):
    """Trả về một list MỚI — mỗi giá đã cộng thêm phí — không đụng gia gốc."""
    ___

bang_gia = [40000, 55000, 60000]
bang_gia_moi = tang_gia_khong_dung_goc(bang_gia, 5000)

print(f"bang_gia (gốc):     {bang_gia}")
print(f"bang_gia_moi (mới): {bang_gia_moi}")
print(f"Cùng một list không: {bang_gia is bang_gia_moi}")
```

```python title=solution
def tang_gia_khong_dung_goc(gia, phi):
    """Trả về một list MỚI — mỗi giá đã cộng thêm phí — không đụng gia gốc."""
    return [g + phi for g in gia]

bang_gia = [40000, 55000, 60000]
bang_gia_moi = tang_gia_khong_dung_goc(bang_gia, 5000)

print(f"bang_gia (gốc):     {bang_gia}")
print(f"bang_gia_moi (mới): {bang_gia_moi}")
print(f"Cùng một list không: {bang_gia is bang_gia_moi}")
```

```python title=test
assert bang_gia == [40000, 55000, 60000], "bang_gia (list gốc) không được đổi — hàm phải TẠO list mới, không sửa list được đưa vào"
assert bang_gia_moi == [45000, 60000, 65000], "bang_gia_moi phải là list mới với mỗi giá đã cộng thêm phi"
assert bang_gia is not bang_gia_moi, "bang_gia_moi phải là một list MỚI — không được là chính bang_gia (kiểm bằng is)"
```

:::hints
- kind: attention
  body: Chỗ trống là toàn bộ thân hàm — chỉ cần MỘT dòng return, dùng list comprehension, không dùng vòng lặp for sửa tại chỗ.
- kind: strategy
  body: 'Viết `[g + phi for g in gia]` — với mỗi phần tử g trong gia, tính g cộng phi, gom vào một list mới. Không có dòng nào bên trong hàm được viết gia[...] = ... hay gia.append(...).'
- kind: one-line
  body: 'Điền `return [g + phi for g in gia]` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hàm phải trả về một list MỚI dựng bằng list comprehension — không được sửa tại chỗ trên tham số gia (không .append, không gán vào ô, không del)
  requireAst:
  - kind: comprehension, min: 1
  - kind: no-mutation
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^bang_gia \(gốc\):     \[40000, 55000, 60000\]\nbang_gia_moi \(mới\): \[45000, 60000, 65000\]\nCùng một list không: False\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một kết quả, nhưng lần này bảng giá gốc không hề hay biết đã có ai
gọi hàm. Đó là dấu hiệu của một hàm đáng tin.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cách bạn vừa dùng — dựng list mới thay vì sửa list cũ — dựa vào đúng
MỘT thứ: bạn NHỚ đừng viết `gia.append(...)` hay `gia[i] = ...` trong
thân hàm. Không có gì THẬT SỰ NGĂN bạn lỡ tay viết một trong hai dòng
đó — Python vẫn cho phép, không báo lỗi gì cả.

Có kiểu dữ liệu nào khiến việc sửa tại chỗ KHÔNG VIẾT ĐƯỢC — chứ không
chỉ dựa vào trí nhớ của người viết hàm?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
