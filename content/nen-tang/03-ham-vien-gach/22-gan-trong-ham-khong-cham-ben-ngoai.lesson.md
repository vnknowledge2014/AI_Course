---
id: nen-tang.ham-vien-gach.gan-trong-ham-khong-cham-ben-ngoai
title: Gán trong hàm không chạm tới bên ngoài
summary: Mọi phép gán trong thân hàm đều dựng một cái tên cục bộ mới — dù ngoài kia có cái tên viết giống hệt, thứ ở ngoài vẫn nguyên vẹn.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.local-assignment]
requires: [core.global-variable, core.local-variable, core.function-def, core.function-call, core.variable, core.assignment, core.fstring]
concepts: [core.ham, core.bien, core.pham-vi]
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
Mình đọc được bảng giá ngoài kia. Viết lên nó lại là chuyện khác hẳn.
::::

::::explain{#doc-duoc-thi-ghi-duoc-khong}
Bài trước để lại một câu chưa trả lời. Máy tìm một cái tên theo thứ tự: trong
hàm trước, không thấy mới ra ngoài — nên thân hàm **đọc** được cái tên bạn viết
ngoài kia. Câu còn treo là chiều ngược lại: **gán** cho chính cái tên ấy, ngay
trong thân hàm, thì cái tên ở ngoài có đổi theo không?

Hai cách hiểu, và cả hai đều nghe lọt tai:

- máy đã tìm ra được cái tên ngoài lúc đọc, thì lúc gán nó cũng ghi thẳng vào
  đúng chỗ ấy;
- lúc gán, máy dựng một cái tên mới nằm gọn trong hàm, còn cái tên ngoài không
  hay biết gì.

Quay lại quán phở. Trên tường quán có một tấm **bảng giá** — ai đi qua cũng
đọc được, và nó treo ở đó cả ngày. Mỗi nhân viên khi bắt đầu phục vụ một bàn
thì cầm theo một **tờ giấy nháp** của riêng lượt ấy.

Trưa nay quán giảm giá. Nhân viên ghi con số mới lên tờ nháp trong tay mình —
họ không trèo lên tường sửa tấm bảng. Phục vụ xong bàn ấy, tờ nháp bỏ đi, tấm
bảng trên tường vẫn nguyên con số cũ.

Máy làm đúng như vậy. Dấu `=` trong thân hàm luôn dựng một cái tên **mới**,
thuộc về riêng lượt gọi này — đúng thứ mà bài về biến cục bộ đã gọi tên. Nó
không ghi đè cái tên ở ngoài, kể cả khi hai cái tên viết giống nhau từng chữ.

Vậy trong hai cách hiểu lúc nãy, cách thứ hai mới đúng.
::::

::::example{#hai-cho-dung-mot-cach-viet}
Bảng giá ngoài kia ghi 45 nghìn. Trong hàm, Byte gán cho đúng cái tên ấy một
con số khác:

```python title=readonly
gia_pho = 45000

def bao_gia_trua():
    gia_pho = 40000
    print(f"Trong hàm, gia_pho đang là {gia_pho}")

bao_gia_trua()
print(f"Ngoài hàm, gia_pho đang là {gia_pho}")
```

Máy in ra:

```text
Trong hàm, gia_pho đang là 40000
Ngoài hàm, gia_pho đang là 45000
```

Hai dòng, hai con số khác nhau, mà trên màn hình chỉ có đúng một cách viết tên:
`gia_pho`.

Đi lại đúng đường máy đi:

- Dòng đầu tiên dán cái tên `gia_pho` lên số 45000. Cái tên này đứng ngoài mọi
  hàm — nó là tấm bảng trên tường.
- Lời gọi `bao_gia_trua()` mở ra một lượt chạy. Dòng `gia_pho = 40000` là một
  phép gán nằm trong thân hàm, nên máy dựng một cái tên `gia_pho` **mới** chỉ
  sống trong lượt này. Tờ giấy nháp vừa được lấy ra.
- Dòng `print` ngay dưới nó đi tìm `gia_pho`. Máy tìm trong hàm trước, và lần
  này nó tìm thấy ngay — tờ nháp đang nằm sẵn trong tay. Nên con số in ra là
  40000.
- Hàm chạy xong, tờ nháp bỏ đi. Dòng `print` cuối cùng đứng ngoài mọi hàm, nó
  đọc tấm bảng trên tường: 45000, y như lúc đầu.

Điều đáng nhìn nhất: dòng gán trong hàm **có chạy thật**, con số 40000 có thật.
Nó chỉ không chạm tới thứ nằm ngoài.
::::

::::predict{#doan-hai-dong commitOnce}
Sổ của Byte ghi 12 tô đã bán. Byte viết một hàm định đếm lại từ đầu.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra hai dòng nào?

```python title=readonly
so_to_da_ban = 12

def dem_lai():
    so_to_da_ban = 0
    print(f"Trong hàm: {so_to_da_ban}")

dem_lai()
print(f"Ngoài hàm: {so_to_da_ban}")
```

:::opt{correct}
Trong hàm: 0 — rồi Ngoài hàm: 12
:::

:::opt
Trong hàm: 0 — rồi Ngoài hàm: 0
::why
Gần đúng ở chỗ bạn đọc dấu `=` rất chuẩn theo đúng thứ Realm 0 đã dạy: gán lại
làm cái tên trỏ sang giá trị mới, và giá trị cũ không còn ai gọi được. Ở ngoài
hàm, chuyện đó xảy ra đúng như bạn nghĩ.

Chỗ lệch là **cái tên nào** vừa được dán lại. Phép gán nằm trong thân hàm nên
nó dựng một cái tên mới của riêng lượt gọi này, chứ không tìm ra ngoài rồi ghi
đè lên đó. Nhân viên viết lên tờ nháp trong tay, tấm bảng trên tường không hề
bị chạm vào — nên dòng cuối vẫn đọc ra 12.
::
:::

:::opt
Trong hàm: 12 — rồi Ngoài hàm: 12
::why
Gần đúng ở chỗ bạn giữ chắc kết luận quan trọng nhất của bài này: hàm không sửa
được cái tên ở ngoài, nên dòng cuối in ra 12. Nửa sau bạn đọc chính xác.

Chỗ lệch nằm ở dòng đầu. Máy không hề bỏ qua dòng `so_to_da_ban = 0` — nó chạy
dòng ấy đàng hoàng và dựng ra một cái tên cục bộ mang số 0. Dòng `print` ngay
dưới đi tìm tên, tìm trong hàm trước, và thấy ngay tờ nháp mang số 0. Con số 0
có thật; nó chỉ không đi ra khỏi hàm được.
::
:::

:::opt
Máy báo lỗi, vì trong và ngoài hàm cùng dùng một cái tên
::why
Gần đúng ở chỗ bạn thấy đúng một điều đáng ngờ: hai chỗ khác nhau cùng gọi một
cái tên thì người đọc rất dễ nhầm chúng là một. Cảm giác ấy đúng, và nó là lý
do người viết mã thường tránh đặt trùng tên.

Chỗ lệch là ai thấy khó chịu. Máy thì không: với nó, cái tên trong hàm và cái
tên ngoài hàm nằm ở hai chỗ tách bạch, nên chúng không tranh nhau chỗ nào cả.
Đây là loại chuyện máy chạy trơn tru mà chỉ người đọc mới thấy rối — cùng họ
với đảo chỗ hai đối số cùng kiểu số.
::
:::
::::

::::explain{#hai-chieu-khong-giong-nhau}
Ghép bài trước với bài này lại, ra một câu gọn:

> Đọc thì đi ra ngoài được. Gán thì không.

Hai chiều **không** đối xứng, và đó là chỗ dễ vấp nhất của cả khối này. Một cái
tên xuất hiện trong thân hàm có thể mang hai vai hoàn toàn khác nhau, tuỳ vào
việc nó đứng bên nào của dấu `=`:

- Đứng ở chỗ được **đọc** — trong `print`, trong một phép tính, sau `return` —
  máy đi tìm: trong hàm trước, không thấy mới ra ngoài.
- Đứng ở bên **trái** dấu `=` — máy không đi tìm gì cả. Nó dựng thẳng một cái
  tên cục bộ mới cho lượt gọi này.

Hệ quả dùng được ngay: **một phép gán trong hàm không thể vô tình dán lại cái
tên của người khác**. Bạn đặt tên biến trong hàm thoải mái, không phải nhớ xem
ngoài kia đã có ai xài cái tên ấy chưa.

Đó mới là một đường — đường của những cái **tên**. Hàm còn chạm ra ngoài được
bằng đường nào nữa không, thì ba bài tới trả lời.

> Chỗ dễ vấp: đặt trùng tên thì máy không phàn nàn, nhưng người đọc lại tưởng
> hai chỗ ấy là một. Trong hàm, nếu con số bạn đang tính không phải thứ ngoài
> kia đang giữ, hãy đặt cho nó một cái tên khác — `gia_khuyen_mai` chẳng hạn.
> Máy chấp nhận cả hai cách; chỉ có người đọc lại mã sau một tuần là không.
::::

::::code{#gia-khuyen-mai-cua-rieng-ham}
Trưa nay quán giảm giá phở xuống còn 40 nghìn, nhưng tấm bảng treo trên tường
thì Byte chưa muốn sửa — chiều tối bán lại đúng giá cũ.

Hàm `bao_gia_khuyen_mai` phải đưa ra con số của buổi trưa, còn cái tên
`gia_niem_yet` ở ngoài phải giữ nguyên con số trên bảng.

Hãy điền dòng còn thiếu ở đầu thân hàm.

```python title=starter
gia_niem_yet = 45000

def bao_gia_khuyen_mai():
    ___
    print(f"Trong hàm, gia_niem_yet đang là {gia_niem_yet} đồng")
    return gia_niem_yet

gia_hom_nay = bao_gia_khuyen_mai()
print(f"Giá khuyến mãi hôm nay: {gia_hom_nay} đồng")
print(f"Giá niêm yết trên bảng: {gia_niem_yet} đồng")
```

```python title=solution
gia_niem_yet = 45000

def bao_gia_khuyen_mai():
    gia_niem_yet = 40000
    print(f"Trong hàm, gia_niem_yet đang là {gia_niem_yet} đồng")
    return gia_niem_yet

gia_hom_nay = bao_gia_khuyen_mai()
print(f"Giá khuyến mãi hôm nay: {gia_hom_nay} đồng")
print(f"Giá niêm yết trên bảng: {gia_niem_yet} đồng")
```

```python title=test
# Hai phép kiểm hỏi hai chuyện khác nhau, và bài chỉ đạt khi cả hai cùng đúng:
# thứ hàm đưa ra là con số của buổi trưa, còn thứ nằm ngoài thì không suy suyển.
#
# Dòng `print` trong thân hàm có mặt vì cách chấm, không vì câu chuyện. Không
# có nó thì `return 40000` — câu trả lời của người CHƯA hiểu bài — qua sạch cả
# hai assert lẫn hai tier output, ở đúng chỗ trống DUY NHẤT nơi khái niệm mới
# của bài phải được chứng minh. Có nó rồi thì `return 40000` làm dòng ấy đọc
# `gia_niem_yet` ở ngoài, in ra 45000, và màn hình lệch ngay dòng đầu.
assert gia_hom_nay == 40000, "hàm phải đưa ra 40 nghìn — dòng bạn điền dựng một cái tên cục bộ mang giá buổi trưa, và dòng `return` ngay dưới đọc lại đúng cái tên ấy"
assert gia_niem_yet == 45000, "tấm bảng ngoài hàm phải vẫn là 45 nghìn — phép gán trong thân hàm không ghi đè cái tên ở ngoài, nên đừng sửa dòng đầu tiên"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ngay dòng đầu thân hàm, phía trên dòng `print`. Cả dòng `print` lẫn dòng `return` bên dưới đều đi tìm cùng một cái tên, và máy luôn tìm trong hàm trước khi ra ngoài.
- kind: strategy
  body: Cần đúng một phép gán, dán cái tên `gia_niem_yet` lên con số của buổi trưa. Phép gán này nằm trong thân hàm nên nó tự dựng một cái tên cục bộ mới — cái tên ngoài kia không việc gì phải lo.
- kind: one-line
  body: Viết `gia_niem_yet = 40000` vào chỗ trống, thụt vào bốn dấu cách cho thẳng hàng với dòng `return` ngay dưới.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Trong hàm, gia_niem_yet đang là 40000 đồng\nGiá khuyến mãi hôm nay: 40000 đồng\nGiá niêm yết trên bảng: 45000 đồng\s*$
- tier: output
  expect: Giá niêm yết trên bảng: 45000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tờ nháp của mình ghi giá trưa. Bảng trên tường vẫn nguyên con số cũ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Chuyện hàm không chạm được ra ngoài là một chuyện tốt — trong hầu hết trường
hợp. Nhưng không phải mọi trường hợp.

Vậy nếu bạn THỰC SỰ muốn hàm sửa cái tên ngoài đó — một biến `tong_doanh_thu`
cộng dồn cả ngày chẳng hạn, mỗi lần bán được một tô thì nó phải lớn thêm và
phải lớn thêm **thật**, chứ không phải lớn trên một tờ nháp rồi bỏ đi — thì bảo
máy thế nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
