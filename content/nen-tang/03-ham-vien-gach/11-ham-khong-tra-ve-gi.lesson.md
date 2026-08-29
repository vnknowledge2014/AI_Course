---
id: nen-tang.ham-vien-gach.ham-khong-tra-ve-gi
title: Hàm không trả về gì
summary: Hàm nào cũng đưa ra một thứ khi chạy xong — thân hàm không có `return` thì thứ đưa ra là `None`, và in ra màn hình khác hẳn đưa ra cho chương trình.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.implicit-return-none]
requires: [core.tuple-unpack, core.function-return, core.function-call, core.function-parameter, core.none, core.is-none, core.type-fn, err.type-error, err.traceback, core.print-variable, core.variable, core.assignment, core.fstring]
concepts: [core.ham, core.gia-tri, core.o-trong]
gradingMatrix:
  web-chrome: [run, tests, output]
  web-firefox: [run, tests, output]
  macos: [run, tests, output]
  windows: [run, tests, output]
  linux: [run, tests, output]
  android: [run, tests, output]
  ios: [run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: true
---

::::byte{trigger=enter mood=thinking pose=lean-in}
Nghe thấy con số không có nghĩa là bạn cầm được nó.
::::

::::explain{#nghe-thay-khong-phai-cam-duoc}
Câu hỏi bỏ ngỏ của bài trước: `x = chao("Lan")` thì `x` giữ cái gì?

Cách duy nhất để biết là nhìn:

```python
def chao(ten):
    print(f"Xin chào {ten}")

x = chao("Lan")
print(x)
```

Màn hình hiện ra hai dòng:

```text
Xin chào Lan
None
```

Dòng thứ nhất là câu chào, đúng như bạn chờ. Dòng thứ hai mới là chỗ đáng dừng
lại: `x` không rỗng, không phải chuỗi trống, không phải lỗi. Nó đang giữ một giá
trị thật, và giá trị ấy là `None` — đúng cái `None` bạn đã gặp ở mạch trước, thứ
Python dành cho một ô chưa ai chạm bút vào.

Vì sao lại thế? Quay lại quán phở, lần này để ý hai động tác khác nhau của bà chủ.

Bà đọc to "Chín mươi nghìn!" cho cả quán nghe — ai ngồi trong quán cũng nghe
thấy con số ấy. Đó là `print`: chữ nghĩa được đẩy lên **màn hình**, dành cho
người đang nhìn.

Rồi bà xé tờ hoá đơn, đặt vào tay bạn. Đó là `return`: một giá trị được đưa về
**chỗ gọi**, dành cho chương trình cầm đi dùng tiếp.

Hai động tác này độc lập nhau. Bà có thể chỉ đọc to mà không xé hoá đơn — bạn
nghe thấy con số nhưng tay không cầm gì. Hàm `chao` làm đúng như vậy: nó đẩy chữ
lên màn hình, và không đưa gì về chỗ gọi.

Chỗ khác biệt với đời thường nằm ở đây: **máy không có kiểu "không đưa gì cả"**.
Mỗi lượt gọi hàm đều phải kết thúc bằng việc đưa ra một thứ, vì bên gọi đang chờ
một thứ để dán vào `x`. Khi thân hàm chạy hết dòng cuối mà không gặp `return` nào,
máy vẫn phải đưa ra một cái gì đó — và cái nó đưa ra là `None`.

Nói cho gọn: `None` chính là câu "tôi chẳng có gì để đưa cho anh cả", viết thành
một giá trị.
::::

::::example{#in-ra-khac-han-dua-ra}
Chuyện trên nghe hiền lành cho tới lúc bạn đem cái `None` ấy đi tính toán.

```python title=readonly
def in_tien(so_to):
    print(so_to * 45000)

tong = in_tien(2)
print(tong + 5000)
```

Máy in ra một dòng rồi nổ:

```text
90000
Traceback (most recent call last):
  File "quan_pho.py", line 5, in <module>
    print(tong + 5000)
          ~~~~~^~~~~~
TypeError: unsupported operand type(s) for +: 'NoneType' and 'int'
```

Đọc từ dòng cuối lên, đúng lối Realm 0 đã dạy. `TypeError` — sai kiểu. Và kiểu
sai là `NoneType`: kiểu của `None`. Máy đang được bảo cộng "chẳng có gì" với
5000, mà nó không có quy ước nào cho phép làm việc đó.

Dòng `90000` in ra ở trên chính là chỗ gài bẫy. Con số đúng đã hiện lên màn hình,
nên nhìn thoáng qua thì hàm có vẻ chạy ngon. Nhưng nó hiện lên **màn hình**, còn
`tong` thì vẫn tay không.

Sửa lại bằng một chữ:

```python
def dua_tien(so_to):
    return so_to * 45000

tong = dua_tien(2)
print(tong + 5000)
```

```text
95000
```

Cùng một phép tính, hai số phận khác nhau. `print` gửi con số cho **người nhìn**;
`return` gửi con số cho **chương trình**. Muốn tính tiếp thì phải là chữ thứ hai.

Còn muốn kiểm tra thẳng xem một hàm có đưa gì ra hay không, bạn đã có sẵn câu hỏi
dành riêng cho `None` từ mạch trước:

```python
ket_qua = in_tien(2)
print(ket_qua is None)
```

```text
90000
True
```
::::

::::predict{#doan-byte-ghi-duoc-gi commitOnce}
Byte viết một hàm báo giá cho quán, rồi định ghi con số ấy vào sổ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python
def bao_gia(so_to):
    tien = so_to * 45000
    print(f"Quán báo: {tien} đồng")

gia = bao_gia(2)
print(f"Byte ghi vào sổ: {gia}")
```

:::opt{correct}
Quán báo: 90000 đồng — rồi Byte ghi vào sổ: None
:::

:::opt
Quán báo: 90000 đồng — rồi Byte ghi vào sổ: 90000
::why
Gần đúng ở chỗ bạn theo dõi thân hàm rất chuẩn: `tien` đúng là 90000, và con số
ấy đúng là con số Byte muốn ghi vào sổ. Phần tính toán bạn đọc không sai một chữ.

Chỗ lệch nằm ở đường đi của con số. Nó được `print` đẩy lên **màn hình** rồi
dừng ở đó; không có dòng nào đưa nó về chỗ gọi. Vì bạn vừa nhìn thấy nó hiện lên,
cảm giác "vậy là nó ra rồi" rất tự nhiên — nhưng cái ra là hình ảnh trên màn hình,
không phải giá trị trong tay `gia`.
::
:::

:::opt
Chỉ một dòng: Quán báo: 90000 đồng — dòng sau không in ra vì `gia` rỗng
::why
Gần đúng ở chỗ bạn nhận ra `gia` không nhận được con số — phần suy luận khó nhất
bạn đã làm đúng.

Chỗ lệch nằm ở chữ *rỗng*. `gia` không rỗng: nó đang giữ `None`, một giá trị thật
và đàng hoàng. Mà `print` thì in ra bất cứ giá trị nào nó nhận được, kể cả `None`
— lúc đó màn hình hiện đúng bốn chữ cái `None`. Một dòng `print` bao giờ cũng in
ra một dòng.
::
:::

:::opt
Máy báo lỗi ở dòng `gia = bao_gia(2)`, vì hàm không có `return` để mà gán
::why
Gần đúng ở chỗ bạn đang đòi hỏi đúng thứ nên đòi hỏi: xin một giá trị từ chỗ
không sinh ra giá trị nào thì lẽ ra phải có ai đó lên tiếng.

Chỗ lệch: Python không lên tiếng ở đây. Mỗi lượt gọi hàm luôn kết thúc bằng việc
đưa ra một thứ, và khi thân hàm hết dòng mà chưa gặp `return`, thứ đưa ra là
`None` — hợp lệ, im lặng, không một lời cảnh báo. Chính sự im lặng ấy làm lỗi
"quên `return`" khó thấy: chương trình chỉ nổ ở chỗ khác, muộn hơn, khi ai đó
đem `None` đi cộng.
::
:::
::::

::::explain{#hai-cau-hoi-truoc-khi-viet-mot-ham}
Gom lại thành một câu để mang đi: **thân hàm không có `return` thì lượt gọi ấy
đưa ra `None`.** Không có ngoại lệ, và máy không báo gì.

Từ đó có hai câu đáng tự hỏi mỗi lần viết một hàm:

- **Ai là người cần kết quả này?** Người đang ngồi nhìn màn hình thì dùng `print`.
  Chương trình — một dòng gán, một phép tính tiếp theo, một lời gọi hàm khác —
  thì dùng `return`.
- **Có ai đặt cái tên ở bên trái lời gọi không?** Nếu có, hàm buộc phải `return`.
  Không thì cái tên ấy nhận `None`, và cú nổ sẽ tới muộn, ở một dòng khác hẳn.

Có những hàm sinh ra chỉ để làm việc thứ nhất, và chúng hoàn toàn đúng đắn — một
hàm `in_hoa_don` lo bày hoá đơn ra màn hình thì chẳng có gì để đưa về cả. Điều
cần tránh không phải là hàm không `return`, mà là **nhầm** một hàm như thế với
một hàm có đưa ra giá trị.

> Chỗ dễ vấp: `print` bên trong thân hàm trông rất giống một kết quả, vì bạn thấy
> con số ngay khi chạy thử. Cách phân biệt chắc chắn là đặt lời gọi vào bên phải
> một dấu `=` rồi in cái tên ấy ra. Hiện `None` thì hàm không đưa gì cả.
::::

::::code{#dua-con-so-ra-khoi-ham}
Byte viết hàm tính tiền cho cả bàn, nhưng thân hàm mới dừng ở chỗ tính xong —
con số vẫn còn nằm bên trong.

Nhìn dòng cuối bài: nó đem `tien_ban_an` đi cộng thêm hai tô nữa. Muốn cộng được
thì cái tên ấy phải đang giữ một con số thật.

Hãy điền dòng còn thiếu ở cuối thân hàm.

```python title=starter
def tinh_tien_ban(gia_mot_to, so_to):
    tien = gia_mot_to * so_to
    ___

tien_ban_an = tinh_tien_ban(45000, 3)
print(f"Bàn ăn hết {tien_ban_an} đồng")
print(f"Thêm hai tô nữa là {tien_ban_an + 90000} đồng")
```

```python title=solution
def tinh_tien_ban(gia_mot_to, so_to):
    tien = gia_mot_to * so_to
    return tien

tien_ban_an = tinh_tien_ban(45000, 3)
print(f"Bàn ăn hết {tien_ban_an} đồng")
print(f"Thêm hai tô nữa là {tien_ban_an + 90000} đồng")
```

```python title=test
# Ba phép kiểm, mỗi phép đóng một lối đi sai.
# Dòng đầu: con số phải ĐI RA tới cái tên ngoài kia, không dừng ở màn hình.
# Hai dòng sau gọi lại hàm với hai bộ số khác — một hàm chép cứng sẵn 135000
# sẽ qua được dòng đầu rồi trượt ngay ở dòng thứ hai.
assert tien_ban_an == 135000, "ba tô phở 45 nghìn hết 135 nghìn, và con số ấy phải đi ra khỏi hàm thì cái tên ngoài kia mới cầm được — in lên màn hình thì không ai cầm được gì"
assert tinh_tien_ban(45000, 2) == 90000, "gọi lại hàm với hai tô phải đưa ra 90 nghìn; hàm chỉ in mà không đưa ra thì phép so sánh này nhận về None và trượt"
assert tinh_tien_ban(50000, 3) == 150000, "đổi giá một tô thành 50 nghìn thì ba tô hết 150 nghìn — con số đưa ra phải tính từ hai thứ nhận vào chứ không phải một số chép sẵn"
```

:::hints
- kind: attention
  body: Chỗ trống nằm trong thân hàm, ngay dưới dòng vừa tính xong con số. Ở chỗ đó cái tên `tien` đã giữ đúng số tiền của cả bàn rồi — việc còn thiếu là chuyện xảy ra sau khi tính.
- kind: strategy
  body: Câu hỏi ở đây là ai cần con số này. Không phải người ngồi nhìn màn hình, mà là dòng gán ở bên ngoài và phép cộng ở dòng cuối — tức là chương trình. Hai việc "đẩy lên màn hình" và "đưa về chỗ gọi" có hai từ khoá khác nhau, và Realm 0 đã dạy cả hai.
- kind: one-line
  body: "Viết `return tien` vào chỗ trống, thụt vào bốn dấu cách cho thẳng hàng với dòng ngay trên nó."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Bàn ăn hết 135000 đồng\nThêm hai tô nữa là 225000 đồng\s*$
- tier: output
  expect: Thêm hai tô nữa là 225000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Con số ra khỏi hàm rồi. Giờ mới có thứ để cộng tiếp.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa đặt `return` ở dòng cuối thân hàm, chỗ ai cũng đặt. Bây giờ thử dời nó
lên giữa thân, vào trong một nhánh `if`:

```python
def bao_con_hang(so_to_con):
    if so_to_con == 0:
        return "Hết phở rồi"
    print("Quán vẫn còn phở")
    return "Còn hàng"
```

Hai dòng cuối viết sát lề của thân hàm, không thụt vào trong `if`. Theo bài thụt
đầu dòng ở Realm 0, dòng nằm ngoài `if` thì chạy trong mọi trường hợp — kể cả khi
điều kiện đúng.

Trong một hàm có `if`, bạn đặt `return` ở nhánh đầu tiên. Những dòng nằm dưới nó
có còn chạy nữa không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
