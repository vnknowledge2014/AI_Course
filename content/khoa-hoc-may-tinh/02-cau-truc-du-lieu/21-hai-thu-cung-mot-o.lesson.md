---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.hai-thu-cung-mot-o
title: "Khi hai thứ băm ra cùng một ô — đụng độ"
summary: "Mảng có cỡ hữu hạn, số chuỗi có thể có là vô hạn — hai tên khác nhau chắc chắn có lúc băm trùng một ô, và cách cất đơn giản của bài trước âm thầm ghi đè, mất dữ liệu không báo lỗi."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.hash-collision]
requires: [ds.hash-table, core.list-comprehension, core.set, core.function-def, core.fstring]
concepts: [ds.hash-collision]
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
Bảng của Byte có 10 ô. Tên người thì gần như vô hạn cách viết. Hôm nay hai
cái tên đụng nhau, và chuyện không đẹp như bạn tưởng.
::::

::::explain{#tai-sao-dung-do-la-tat-yeu}
Bảng băm bài 20 có `co_bang` ô — một con số HỮU HẠN, cố định ngay từ lúc
dựng bảng. Nhưng số chuỗi có thể đặt làm tên thì không hữu hạn: "An",
"Ân", "Ann", "Anna", hàng triệu cách viết khác nhau, còn nhiều hơn cả dân
số một quốc gia.

Nhét một số lượng KHÔNG GIỚI HẠN khả năng vào một số lượng HỮU HẠN ô — sớm
muộn gì cũng phải có hai chuỗi khác nhau rơi vào đúng một ô. Đây gọi là
**đụng độ** (collision), và nó không phải một lỗi thiết kế bạn có thể né
bằng cách viết công thức băm khéo hơn. Nó là hệ quả BẮT BUỘC của việc hữu
hạn ô phải chứa vô hạn khả năng — công thức băm nào cũng phải đối mặt với
nó, không có ngoại lệ.

Nguy hiểm hơn cả việc đụng độ CÓ THỂ xảy ra, là cách bài 20 xử lý nó khi nó
xảy ra: `bang[chi_so] = gia_tri` là một phép GHI ĐÈ đơn thuần. Nếu ô đó đã
có ai ở đó từ trước, Python không hỏi, không báo, không giữ lại — cứ ghi
đè thẳng. Cái cũ biến mất, không dấu vết, không tiếng động.
::::

::::example{#quan-de-lai-an}
Byte dùng lại đúng `chen` và `tra_cuu` của bài 20 — chưa sửa gì — rồi chèn
"An" trước, "Quân" sau, vào đúng bảng 10 ô:

```python title=readonly
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
print("sau khi chèn An:", danh_ba[5])

chen(danh_ba, "Quân", "0907777777", co_bang)
print("sau khi chèn Quân:", danh_ba[5])

print("tra An:", tra_cuu(danh_ba, "An", co_bang))
```

```text title=readonly
sau khi chèn An: {'ten': 'An', 'so_dien_thoai': '0901111111'}
sau khi chèn Quân: {'ten': 'Quân', 'so_dien_thoai': '0907777777'}
tra An: 0907777777
```

`bam("An", 10)` và `bam("Quân", 10)` cùng ra `5` — hai cái tên hoàn toàn
khác nhau, cùng một ô. Chèn "Quân" sau GHI ĐÈ đúng ô 5, và số của "An"
biến mất — không lỗi, không cảnh báo. Dòng cuối là hậu quả rõ nhất: hỏi
số điện thoại của "An", máy đưa ra số của "Quân". Không phải máy "không
tìm thấy" An — nó đưa một câu trả lời CÓ VẺ ĐÚNG, chỉ là sai người. Một
câu trả lời sai mà trông như đúng còn nguy hiểm hơn một lỗi báo thẳng.
::::

::::predict{#doan-ket-qua-tra-an commitOnce}
Đúng kịch bản ở ví dụ trên: bảng 10 ô, chèn "An" trước, "Quân" sau — cả
hai cùng băm ra ô 5.

**Trước khi chạy**, bạn đoán `tra_cuu(danh_ba, "An", 10)` — gọi SAU khi cả
hai đã được chèn — trả về gì?

:::opt{correct}
`"0907777777"` — số của Quân, dù câu hỏi là số của An.
:::

:::opt
`"0901111111"` — số đúng của An, vì bảng phải "nhớ" ai đã cất trước
::why
Gần đúng ở chỗ bạn tin đúng: mỗi cái tên PHẢI có một số điện thoại của
riêng nó, và đó là điều một danh bạ đúng đắn phải giữ được.

Chỗ lệch là ô 5 của `danh_ba` giờ chỉ còn ĐÚNG MỘT giá trị — của Quân.
Dòng `bang[chi_so] = ...` không "thêm vào", nó THAY THẾ hoàn toàn những gì
đang có ở ô đó. Dict của An đã bị ghi đè, không còn dấu vết nào sót lại
để "nhớ" nó từng ở đó.
::
:::

:::opt
Máy dừng lại, báo lỗi vì hai người trùng ô
::why
Gần đúng ở việc bạn cảm thấy đúng: hai người trùng một ô LÀ một chuyện bất
thường, đáng phải có phản ứng gì đó.

Chỗ lệch là Python không tự phát hiện chuyện này. Phép gán
`bang[chi_so] = gia_tri` là một câu lệnh hoàn toàn hợp lệ — gán vào một ô
đã có giá trị trước đó không phải lỗi cú pháp hay lỗi kiểu dữ liệu gì cả,
chỉ đơn giản là GHI ĐÈ, âm thầm, không báo hiệu.
::
:::

:::opt
`None` — vì An coi như đã bị xoá khỏi bảng
::why
Gần đúng ở chỗ "An biến mất" đúng là cảm giác thật — dữ liệu của An không
còn truy được nữa.

Chỗ lệch là ô 5 KHÔNG rỗng. `tra_cuu` chỉ trả `None` khi ô đọc được là
`None` (chưa ai chiếm) — nhưng ô 5 giờ có một dict THẬT, của Quân. Hàm
không hề biết ô đó "lẽ ra" phải là của An; nó chỉ đọc thấy một giá trị
hợp lệ và trả về đúng số điện thoại nằm trong đó, dù là của sai người.
::
:::
::::

::::code{#phat-hien-dung-do}
Trước khi vá lỗi mất dữ liệu (bài sau), bạn viết một đoạn kiểm tra: cho
một danh sách tên, có cặp nào đụng độ nhau ở `co_bang` cho trước không?

So sánh SỐ TÊN với SỐ Ô KHÁC NHAU mà các tên đó băm ra: nếu số ô khác
nhau ÍT HƠN số tên, chắc chắn có ít nhất một ô bị hai tên trở lên dùng
chung. Bạn hoàn thành hàm `dem_o_khac_nhau`: nhận một danh sách chỉ số đã
băm ra, trả về SỐ GIÁ TRỊ KHÁC NHAU có trong đó.

```python title=starter
def bam(ten, co_bang):
    return sum(ten.encode("utf-8")) % co_bang

def dem_o_khac_nhau(cac_chi_so):
    return ___

danh_sach_ten = ["An", "Hoa", "Lan", "Quân"]
co_bang = 10

cac_chi_so = [bam(ten, co_bang) for ten in danh_sach_ten]
so_ten = len(danh_sach_ten)
so_o_khac_nhau = dem_o_khac_nhau(cac_chi_so)

co_dung_do = so_o_khac_nhau < so_ten

print(cac_chi_so)
print(f"Số tên: {so_ten}, số ô khác nhau: {so_o_khac_nhau}")
print(f"Có đụng độ: {co_dung_do}")
```

```python title=solution
def bam(ten, co_bang):
    return sum(ten.encode("utf-8")) % co_bang

def dem_o_khac_nhau(cac_chi_so):
    return len(set(cac_chi_so))

danh_sach_ten = ["An", "Hoa", "Lan", "Quân"]
co_bang = 10

cac_chi_so = [bam(ten, co_bang) for ten in danh_sach_ten]
so_ten = len(danh_sach_ten)
so_o_khac_nhau = dem_o_khac_nhau(cac_chi_so)

co_dung_do = so_o_khac_nhau < so_ten

print(cac_chi_so)
print(f"Số tên: {so_ten}, số ô khác nhau: {so_o_khac_nhau}")
print(f"Có đụng độ: {co_dung_do}")
```

```python title=test
assert cac_chi_so == [5, 0, 3, 5], f"cac_chi_so phải là [5, 0, 3, 5] — đang ra {cac_chi_so}"
assert so_o_khac_nhau == 3, f"An và Quân cùng băm ra 5, nên chỉ có 3 ô khác nhau được dùng (5, 0, 3) cho 4 tên — đang ra {so_o_khac_nhau}"
assert co_dung_do is True, "4 tên mà chỉ dùng 3 ô khác nhau — chắc chắn có đụng độ, co_dung_do phải là True"
# Kiểm dem_o_khac_nhau trên những danh sách chỉ số KHÁC HẲN — không phải chỉ
# đúng một bộ dữ liệu cố định — để một chỗ trống gõ thẳng con số 3 (đúng cho
# MỖI mình danh_sach_ten ở trên) không thể lọt qua được nữa.
assert dem_o_khac_nhau([1, 1, 1, 1]) == 1, "4 chỉ số giống hệt nhau thì chỉ có 1 GIÁ TRỊ khác nhau — dem_o_khac_nhau([1,1,1,1]) phải ra 1, không phải 4 (đếm phần tử) hay 3 (một con số cố định)"
assert dem_o_khac_nhau([1, 2, 3, 4, 5]) == 5, "5 chỉ số đều khác nhau thì phải có đúng 5 giá trị khác nhau"
assert dem_o_khac_nhau([]) == 0, "danh sách rỗng thì không có ô nào khác nhau cả"
```

:::hints
- kind: attention
  body: "`cac_chi_so` là một list CÓ THỂ có số lặp lại — mỗi tên một chỉ số, kể cả khi hai tên trùng chỉ số. Hàm cần đếm bao nhiêu GIÁ TRỊ KHÁC NHAU có trong list đó, không phải đếm bao nhiêu phần tử."
- kind: strategy
  body: "R1 đã dạy một kiểu dữ liệu chỉ giữ các giá trị KHÁC NHAU, tự động bỏ trùng — dùng nó để lọc `cac_chi_so`, rồi đếm số phần tử còn lại bằng `len`."
- kind: one-line
  body: "Điền `len(set(cac_chi_so))` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\[5, 0, 3, 5\\]\\nSố tên: 4, số ô khác nhau: 3\\nCó đụng độ: True\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
4 tên, chỉ 3 ô khác nhau — đúng như bạn vừa chứng minh, đụng độ không phải
chuyện hiếm, nó là chuyện TẤT YẾU.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy vấn đề rõ ràng: hai tên cùng ô, cái sau ghi đè cái trước, dữ
liệu mất không báo hiệu. Đổi công thức băm khéo hơn không giải quyết được
gốc rễ — hữu hạn ô, vô hạn tên, đụng độ vẫn sẽ xảy ra ở đâu đó.

Vậy phải sửa KHÔNG PHẢI công thức băm, mà sửa CÁCH MỘT Ô CẤT GIỮ THỨ NÓ
GIỮ. Nếu một ô, thay vì chỉ giữ ĐÚNG MỘT giá trị, có thể giữ NHIỀU giá trị
cùng lúc — bạn đã có sẵn công cụ nào, từ đúng track này, làm được việc
"giữ nhiều thứ, nối tiếp nhau" đó chưa?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
