---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.tam-o-la-mot-byte
title: Tám ô là một byte, và nó chứa được tới đâu
summary: "Máy đếm khách của cô Bảy chỉ có tám đèn. Cộng đáng giá cả tám cột lại — không ai đưa sẵn — ra đúng con số lớn nhất một byte còn thắp nổi."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [mem.byte-range]
requires: [mem.int-to-binary]
concepts: [mem.byte-range]
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
Tám đèn, không cái thứ chín. Mình muốn biết chúng thắp nổi tới con số nào.
::::

::::explain{#tam-cot-cong-lai}
Máy đếm khách của cô Bảy có đúng tám đèn — tám ô hai trạng thái đứng cạnh
nhau, đúng thứ Realm 0 gọi là một **byte**. Bài trước bạn học đọc và viết
dãy bit qua đáng giá từng cột; bây giờ hỏi thẳng: tám cột ấy, cộng đáng giá
lại hết, được bao nhiêu?

Con số nhỏ nhất mà tám đèn thắp được là lúc tất cả cùng tắt: `00000000`,
đáng giá 0.

Con số lớn nhất là lúc tất cả cùng bật: `11111111`. Đáng giá của nó là tổng
đáng giá cả tám cột cộng lại — không ai đưa sẵn con số này, phải tự cộng.
Đáng giá mỗi cột gấp đôi cột bên phải nó, bắt đầu từ 1:

```text title=readonly
1 + 2 = 3
3 + 4 = 7
7 + 8 = 15
15 + 16 = 31
31 + 32 = 63
63 + 64 = 127
127 + 128 = ?
```

Nhìn dãy kết quả — 3, 7, 15, 31, 63, 127 — mỗi số đều **kém đúng một** so với
lần gấp đôi kế tiếp: 3 kém 4 một, 7 kém 8 một, 15 kém 16 một. Không phải
trùng hợp: cộng hết mọi cột nhỏ hơn một cột nào đó luôn còn thiếu đúng một
đơn vị mới đầy tới cột đó, giống hệt 9 kém 10 một trong hệ mười, hay 99 kém
100 một. Bạn tự bấm nốt phép cộng cuối để ra con số lớn nhất mà một byte
chứa được.
::::

::::example{#cong-du-tam-cot}
Bấm bằng vòng lặp cho chắc, thay vì cộng tay dễ sót:

```python title=readonly
cot = 1
tong = 0
for _ in range(8):
    tong = tong + cot
    cot = cot * 2

print(tong)
```

Máy in ra:

```text title=readonly
255
```

Vòng lặp chạy đúng tám lượt — bằng số cột của một byte. Mỗi lượt cộng đáng
giá cột hiện tại vào tổng, rồi gấp đôi đáng giá cho lượt sau. **255**, không
phải 256 — con số 256 là tổng số KIỂU khác nhau mà tám đèn tạo được (đúng
con số bạn đã gặp ở Realm 0), còn 255 là con số LỚN NHẤT trong số các kiểu
đó, vì phép đếm bắt đầu từ 0. Hai trăm năm mươi sáu kiểu, mà kiểu đầu tiên
mang giá trị 0 chứ không phải 1 — nên kiểu cuối cùng chỉ mang giá trị 255.

Cách viết gọn hơn của đúng phép cộng này là `2**8 - 1` — luỹ thừa cho tổng số
kiểu, trừ một cho chỗ đếm bắt đầu từ 0. Từ giờ dùng được cách viết ấy; vòng
lặp phía trên là chỗ nó thật sự đến từ đâu.
::::

::::predict{#doan-bay-cot commitOnce}
Byte thử cộng đáng giá của **bảy** cột đầu — thiếu đúng cột thứ tám, cột
đáng giá 128.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
tong_bay_cot = 1 + 2 + 4 + 8 + 16 + 32 + 64
print(tong_bay_cot)
print(tong_bay_cot == 255)
```

:::opt{correct}
127, rồi False.
:::

:::opt
127, rồi True.
::why
Gần đúng ở chỗ 127 đúng là tổng bảy số hạng ấy cộng lại — bạn cộng tay không
sai một số nào.

Chỗ lệch là 127 không bằng 255. 255 là giá trị lớn nhất của MỘT BYTE — tám
cột — còn phép cộng này chỉ có bảy số hạng, thiếu hẳn cột đáng giá 128 cuối
cùng. Bảy cột là một dãy NGẮN HƠN byte đúng một cột, nên giá trị lớn nhất
của nó cũng nhỏ hơn hẳn, không phải bằng nhau.
::
:::

:::opt
255, rồi True.
::why
Gần đúng ở chỗ 255 đúng là con số quan trọng nhất của cả bài này — giá trị
lớn nhất một byte chứa được, và bạn nhớ đúng con số ấy.

Chỗ lệch là phép cộng trên dòng đầu chỉ có bảy số hạng: `1 + 2 + 4 + 8 + 16
+ 32 + 64`, dừng lại trước khi tới 128. Thiếu cột cuối cùng — cột nặng nhất —
thì tổng không thể chạm tới 255 được, dù bảy cột kia cộng đúng tuyệt đối.
::
:::

:::opt
128, rồi False.
::why
Gần đúng ở chỗ 128 có thật trong bài — nó chính là cột thứ tám, cột duy
nhất KHÔNG có mặt trong phép cộng này. Bạn nhớ đúng con số, chỉ nhầm vai trò
của nó.

Chỗ lệch là 128 là đáng giá của MỘT cột, còn câu hỏi hỏi TỔNG của bảy cột
khác cộng lại. Cộng bảy số hạng đầu — không đụng tới 128 — ra 127, một con
số nhỏ hơn 128 đúng một, cùng kiểu "kém một" mà bài này vừa nói tới.
::
:::
::::

::::code{#kiem-tra-vua-byte}
Máy đếm khách của cô Bảy chỉ có một byte để cất số khách. Tối nay đông bất
thường: **300** khách đã ghé qua.

Byte đã gõ sẵn khung tính giá trị lớn nhất của một byte — giống hệt ví dụ
trên — rồi dùng nó để hỏi 300 có vừa không. Hai chỗ trống là hai chỗ hụt
trong khung ấy.

```python title=starter
cot = 1
gia_tri_lon_nhat = 0
for _ in range(8):
    gia_tri_lon_nhat = gia_tri_lon_nhat + ___
    cot = ___

so_khach_hom_nay = 300
vua_mot_byte = 0 <= so_khach_hom_nay <= gia_tri_lon_nhat

print(gia_tri_lon_nhat)
print(vua_mot_byte)
```

```python title=solution
cot = 1
gia_tri_lon_nhat = 0
for _ in range(8):
    gia_tri_lon_nhat = gia_tri_lon_nhat + cot
    cot = cot * 2

so_khach_hom_nay = 300
vua_mot_byte = 0 <= so_khach_hom_nay <= gia_tri_lon_nhat

print(gia_tri_lon_nhat)
print(vua_mot_byte)
```

```python title=test
# gia_tri_lon_nhat phải đúng 255 — không chỉ đủ để 300 báo "không vừa", vì
# một khung tính sai (ví dụ dừng ở 8) cũng cho ra "không vừa" y hệt cho
# riêng con số 300. Kiểm thêm đúng ranh giới: 255 phải vừa, 256 thì không.
assert gia_tri_lon_nhat == 255, "cộng đáng giá cả tám cột của một byte (1,2,4,8,16,32,64,128) phải ra đúng 255"
assert cot == 256, "sau đúng tám lượt gấp đôi kể từ 1, cot phải đứng ở 256 — cột kế tiếp, NGOÀI byte"
assert vua_mot_byte == False, "300 lớn hơn 255 nên không vừa một byte"
assert 0 <= 255 <= gia_tri_lon_nhat, "255 phải vừa khít trong một byte — đó chính là con số bài này đi tìm"
assert not (0 <= 256 <= gia_tri_lon_nhat), "256 phải KHÔNG vừa — nó là kiểu thứ 257, dư ra đúng một"
```

:::hints
- kind: attention
  body: Khung này giống hệt ví dụ phía trên, chỉ khoét trống hai chỗ trong vòng lặp — chỗ CỘNG đáng giá cột hiện tại vào tổng, và chỗ GẤP ĐÔI đáng giá cho lượt sau.
- kind: strategy
  body: Đáng giá cột hiện tại đã có tên sẵn — `cot`. Cộng nó vào tổng ở chỗ trống thứ nhất; gấp đôi nó cho lượt kế tiếp ở chỗ trống thứ hai. Đừng gõ cứng con số 255 hay 256 — nếu gõ cứng thì hai dòng kiểm biên ở cuối bài sẽ không còn đúng ý nghĩa của chúng nữa.
- kind: one-line
  body: "Chỗ trống thứ nhất là `cot`, chỗ trống thứ hai là `cot * 2`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^255\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
255. Quá con số đó, tám đèn hết chỗ — dù khách vẫn cứ bước vào quán.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

255 viết ra là tám ký tự dài dằng dặc: `11111111`. Byte lớn hơn thì dãy còn
dài hơn nữa — cứ thêm tám ô là thêm tám ký tự `0` với `1` phải đếm bằng mắt.

Có cách nào viết gọn hơn không, mà vẫn giữ nguyên cấu trúc tám-ô-một-nhóm,
không đánh mất thông tin nào trong đó?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
