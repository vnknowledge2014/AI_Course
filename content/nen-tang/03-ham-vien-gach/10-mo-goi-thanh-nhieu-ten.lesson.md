---
id: nen-tang.ham-vien-gach.mo-goi-thanh-nhieu-ten
title: Mở gói thành nhiều cái tên
summary: Đặt nhiều cái tên bên trái dấu bằng thì cái gói hàm đưa ra được mở ngay lúc nhận — mỗi phần một tên, và số tên phải khớp đúng số phần.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.tuple-unpack]
requires: [core.function-return, core.function-call, core.function-parameter, core.multi-assign, core.value-error, core.list-index, core.variable, core.assignment, core.fstring]
concepts: [core.ham, core.gia-tri, core.bien]
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
Cái gói mở ra được ngay lúc nhận. Mình không phải đếm số thứ tự nữa.
::::

::::explain{#dem-so-thu-tu-la-viec-nang}
Bài trước để lại đúng một chỗ vướng. Hàm đưa ra được hai con số rồi, nhưng lúc
nhận thì cái gói vẫn còn nguyên đai nguyên kiện:

```python
goi = tinh_hoa_don(45000, 2)
print(goi[0])
print(goi[1])
```

Đoạn này chạy đúng. Nó chỉ có một chuyện: `goi[0]` và `goi[1]` không nói cho ai
biết bên trong là gì. Đọc lại sau một tuần, bạn phải mở hàm ra xem `return` xếp
thứ tự nào trước — mà cả mạch này dựng lên để bạn **không** phải mở hàm ra xem.

Nghĩ về cái túi ni lông bà chủ đưa lúc bạn mua mang về. Trong túi có hai hộp:
hộp bánh phở và hộp nước dùng. Về tới nhà, không ai giữ nguyên cái túi rồi mỗi
lần cần lại thò tay đếm "hộp thứ nhất, hộp thứ hai". Người ta **mở túi ra**, đặt
hộp bánh lên một chỗ, hộp nước lên một chỗ, và từ đó gọi thẳng tên từng thứ.

Python cho làm đúng động tác ấy. Đặt **hai cái tên** bên trái dấu `=`, ngăn nhau
bằng dấu phẩy:

```python
tien_hang, tien_thue = tinh_hoa_don(45000, 2)
print(tien_hang)
print(tien_thue)
```

Bên phải vẫn là **một** lời gọi hàm, đưa ra **một** cái gói. Bên trái là hai cái
tên. Máy mở gói ra và chia theo đúng thứ tự đứng: phần thứ nhất về tên thứ nhất,
phần thứ hai về tên thứ hai. Cách viết này gọi là **mở gói** — tiếng Anh là
*unpacking*, và bạn sẽ thấy đúng chữ ấy trong thông báo lỗi lát nữa.

Hình dạng này bạn đã gặp ở mạch trước, dưới tên gán nhiều:
`ten, tien, ghi_chu = "cà phê", 25000, None`. Khác biệt duy nhất nằm ở vế phải.
Lần trước vế phải là ba giá trị bạn tự gõ ra; lần này vế phải là một lời gọi hàm,
và ba giá trị ấy do hàm quyết định. Máy xử lý cả hai y hệt nhau: đếm hai bên, rồi
chia theo thứ tự.

Và có một chuyện đáng để ý: hai cái tên bên trái là **tên của bạn**, không phải
tên của hàm. Bên trong hàm chúng có thể mang tên khác hẳn. Người viết hàm xếp
thứ tự; người nhận đặt tên.
::::

::::example{#mo-goi-tai-cho-nhan}
Cả đoạn, viết trọn một lần. Hàm bên trong dùng hai cái tên ngắn, người nhận bên
ngoài đặt hai cái tên dài hơn cho dễ đọc.

```python title=readonly
def tinh_hoa_don(gia_mot_to, so_to):
    hang = gia_mot_to * so_to
    thue = hang // 10
    return hang, thue

tien_hang, tien_thue = tinh_hoa_don(45000, 2)

print(f"Tiền hàng: {tien_hang} đồng")
print(f"Tiền thuế: {tien_thue} đồng")
```

Máy in ra:

```text
Tiền hàng: 90000 đồng
Tiền thuế: 9000 đồng
```

Đi lại đúng đường máy đi ở dòng mở gói:

- Vế phải chạy trước, y như mọi dòng gán. `tinh_hoa_don(45000, 2)` chạy trọn thân
  hàm, `hang` thành 90000, `thue` thành 9000, và `return hang, thue` buộc hai con
  số ấy thành một gói hai phần.
- Máy đếm: bên trái hai tên, bên phải gói hai phần. Khớp.
- Máy chia: phần đứng trước về `tien_hang`, phần đứng sau về `tien_thue`.

Hai dòng `print` bên dưới không còn con số thứ tự nào. Đọc `tien_thue` là biết nó
là tiền thuế, không phải đi tra `return` xem ai đứng trước ai.

Còn một chỗ máy **không** đỡ được cho bạn, giống hệt chỗ máy không đỡ ở gán
nhiều: nó đếm số lượng, nhưng nó không đọc được ý nghĩa. Viết
`tien_thue, tien_hang = tinh_hoa_don(45000, 2)` thì dòng gán vẫn chạy êm, và từ
đó `tien_thue` giữ 90000. Thứ tự bên trái phải khớp thứ tự trong `return`, và
người giữ việc đó là bạn.
::::

::::predict{#doan-ba-ten-hai-phan commitOnce}
Byte muốn ghi thêm tiền ship vào hoá đơn nên viết sẵn ba cái tên bên trái. Hàm
thì vẫn là hàm cũ, vẫn `return` đúng hai thứ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def tinh_hoa_don(gia_mot_to, so_to):
    hang = gia_mot_to * so_to
    thue = hang // 10
    return hang, thue

tien_hang, tien_thue, tien_ship = tinh_hoa_don(45000, 2)
print(f"Tiền hàng: {tien_hang} đồng")
```

:::opt{correct}
Máy không in dòng nào, chỉ hiện một thông báo `ValueError`
:::

:::opt
Máy in `Tiền hàng: 90000 đồng`, còn `tien_ship` được máy để là `None`
::why
Gần đúng ở chỗ bạn chọn trúng thứ Python dành cho một ô chưa có gì. `None` sinh
ra đúng cho loại ô ấy, và nếu có ai phải lấp vào chỗ trống thì `None` là thứ
được lấp.

Chỗ lệch: `None` là thứ **bạn** viết ra, không phải thứ máy tự điền. Máy đếm ba
tên bên trái, hai phần bên phải, thấy lệch thì dừng ngay tại dòng đó — chưa cái
tên nào được dán, kể cả `tien_hang`, nên dòng `print` bên dưới cũng không tới
lượt chạy. Nếu máy tự bù, thì cái ngày bạn xin nhầm số phần, chương trình vẫn
chạy trơn tru với một ô rỗng mà không ai biết.
::
:::

:::opt
Máy in `Tiền hàng: 90000 đồng` xong rồi mới báo lỗi
::why
Gần đúng ở chỗ bạn hình dung máy chạy lần lượt từ trên xuống, xong dòng nào thì
tới dòng sau — cách hình dung ấy đúng cho phần lớn chương trình, và bạn đã gặp
những cú nổ xảy ra sau khi màn hình đã in được mấy dòng.

Chỗ lệch nằm ở dòng nổ. Ở đây cú nổ xảy ra ngay tại **dòng mở gói**, trước khi
máy đọc tới dòng `print`. Một dòng nổ là chương trình dừng hẳn, mọi dòng bên dưới
không được chạy — y như `TypeError` ở Realm 0.
::
:::

:::opt
Máy báo `TypeError`, vì hàm chỉ đưa ra hai thứ mà bạn xin ba
::why
Gần đúng ở chỗ khó nhất: bạn nhận ra đây là chuyện lệch số lượng giữa thứ hàm
đưa ra và thứ bạn xin. Phần đọc tình huống của bạn chính xác.

Chỗ lệch nằm ở **loại** lỗi. `TypeError` dành cho chuyện sai loại — cộng một câu
chữ với một con số chẳng hạn. Ở đây loại thì đúng: bên phải đúng là một cái gói,
đúng thứ mở ra được. Chỉ **nội dung** của nó không hợp — gói có hai phần mà bạn
xin ba. Đúng loại việc, sai nội dung, đó là `ValueError`, và dòng cuối của thông
báo nói thẳng: *not enough values to unpack (expected 3, got 2)*.
::
:::
::::

::::explain{#dem-truoc-khi-chia}
Luật cứng của dòng mở gói nằm ở **số lượng**, và nó chặt y như ở gán nhiều:

- Số tên bên trái phải bằng **đúng** số phần trong gói.
- Thiếu thì `not enough values to unpack`, thừa thì `too many values to unpack`.
  Cả hai đều là `ValueError`.
- Máy đếm **trước** khi dán. Lệch thì không cái tên nào được động tới, kể cả cái
  tên đứng đầu.

Chuyện này đặt ra một câu hỏi thực tế: làm sao biết hàm đưa ra mấy phần để mà đặt
cho đúng số tên? Bạn đã có sẵn hai đường, và không đường nào bắt bạn mở thân hàm
ra đọc. `help(tinh_hoa_don)` in lại docstring — chỗ người viết hàm nói ra hàm đưa
gì về. Còn nếu docstring chưa nói rõ, gán tạm vào **một** cái tên rồi `print` nó
ra: cái gói hiện nguyên hình cùng số phần của nó.

> Chỗ dễ vấp: một cái tên bên trái thì **không** phải mở gói. `goi = tinh_hoa_don(45000, 2)`
> cho `goi` giữ trọn cái gói, còn `tien_hang, tien_thue = tinh_hoa_don(45000, 2)`
> chia gói ra hai. Dấu phẩy bên trái dấu `=` mới là thứ bật chế độ mở gói lên, và
> nó nhỏ đến mức rất dễ gõ sót.
::::

::::code{#nhan-hai-phan-hai-ten}
Byte mua **hai** tô phở, mỗi tô **45000** đồng. Lời gọi hàm đã viết sẵn ở vế
phải, và nó đưa ra một cái gói hai phần.

Chỗ trống là **vế trái**. Hai dòng `print` bên dưới đang chờ hai cái tên; hãy đặt
đúng hai cái tên ấy, đúng thứ tự mà `return` trong thân hàm đã xếp.

```python title=starter
def tinh_hoa_don(gia_mot_to, so_to):
    hang = gia_mot_to * so_to
    thue = hang // 10
    return hang, thue

___ = tinh_hoa_don(45000, 2)
print(f"Tiền hàng: {tien_hang} đồng")
print(f"Tiền thuế: {tien_thue} đồng")
```

```python title=solution
def tinh_hoa_don(gia_mot_to, so_to):
    hang = gia_mot_to * so_to
    thue = hang // 10
    return hang, thue

tien_hang, tien_thue = tinh_hoa_don(45000, 2)
print(f"Tiền hàng: {tien_hang} đồng")
print(f"Tiền thuế: {tien_thue} đồng")
```

```python title=test
# Hai cái tên phải nhận đúng hai phần của gói, và nhận đúng thứ tự mà `return`
# đã xếp. Đảo hai cái tên cho nhau thì cả hai phép kiểm dưới đây đều trượt.
assert tien_hang == 90000, "hai tô phở 45 nghìn thì tiền hàng là 90 nghìn — đó là phần ĐỨNG TRƯỚC trong gói, nên nó thuộc về cái tên đứng trước dấu phẩy"
assert tien_thue == 9000, "thuế bằng một phần mười tiền hàng, tức 9 nghìn — đó là phần ĐỨNG SAU trong gói, nên nó thuộc về cái tên đứng sau dấu phẩy"
```

:::hints
- kind: attention
  body: Hai dòng `print` bên dưới đang gọi hai cái tên mà chưa dòng nào dựng ra. Đọc kỹ hai cái tên ấy — chỗ trống phải dựng ra đúng chúng, và chỉ có một dòng để dựng cả hai.
- kind: strategy
  body: Một cái tên đứng một mình bên trái thì nhận trọn cả gói, không phải thứ hai dòng `print` đang chờ. Muốn máy mở gói ra thì bên trái phải có nhiều hơn một tên, ngăn nhau bằng dấu phẩy. Thứ tự bên trái phải khớp thứ tự trong `return` của thân hàm — tiền hàng trước, tiền thuế sau.
- kind: one-line
  body: "Viết `tien_hang, tien_thue` vào chỗ trống, giữ nguyên dấu bằng và lời gọi hàm đã có sẵn ở bên phải."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Tiền hàng: 90000 đồng\nTiền thuế: 9000 đồng\s*$
- tier: output
  expect: Tiền thuế: 9000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mở gói xong thì mỗi phần có tên riêng. Nhìn là biết, khỏi đếm.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Dòng mở gói vừa rồi đứng được là nhờ một chuyện: bên phải dấu `=` có một thứ đi
ra khỏi hàm. Hàm `tinh_hoa_don` viết `return`, nên nó đưa ra một cái gói, và cái
gói ấy chia được.

Bây giờ nhìn sang một hàm khác. Hàm `chao(ten)` ở Realm 0 chỉ có đúng một dòng
`print` trong thân, không có `return` nào cả:

```python
def chao(ten):
    print(f"Xin chào {ten}")
```

Gọi `chao("Lan")` thì màn hình hiện ra câu chào, chuyện đó bạn đã thấy. Nhưng thử
đặt một cái tên ở bên trái mà xem.

Hàm `chao(ten)` chỉ `print` chứ không `return` gì cả. Vậy `x = chao("Lan")` —
`x` đang giữ cái gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
