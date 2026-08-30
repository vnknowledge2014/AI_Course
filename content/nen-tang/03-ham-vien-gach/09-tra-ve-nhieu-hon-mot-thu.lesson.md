---
id: nen-tang.ham-vien-gach.tra-ve-nhieu-hon-mot-thu
title: Trả về nhiều hơn một thứ
summary: `return tien_hang, tien_thue` — hai giá trị được buộc lại thành MỘT cái gói rồi mới đi ra khỏi hàm.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.return-multiple]
practices: [core.default-parameter, core.function-return, core.list-index, core.fstring]
requires: [core.default-parameter, core.keyword-argument, core.function-return, core.function-call, core.function-parameter, core.docstring, core.list-index, core.type-fn, core.int-cast, core.fstring, err.type-error]
concepts: [core.ham, core.gia-tri]
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
  reviewed: true
---

::::byte{trigger=enter mood=curious pose=lean-in}
Cửa ra chỉ có một. Nhưng ai bảo mỗi lần ra chỉ được cầm một món?
::::

::::explain{#hai-con-so-mac-ket-trong-ham}
Bên trong `tinh_hoa_don` có hai con số đáng giá:

```python
def tinh_hoa_don(so_to, gia, thue=0.1):
    """Tiền một bàn đã cộng thuế."""
    tien_hang = so_to * gia
    tien_thue = int(tien_hang * thue)
    return tien_hang + tien_thue
```

Hoá đơn quán in cả hai dòng — tiền hàng một dòng, tiền thuế một dòng. Nhưng
dòng `return` cộng chúng lại rồi đưa ra đúng một con số tổng. Hàm chạy xong,
`tien_hang` và `tien_thue` biến mất. Muốn in ra tiền thuế, người gọi phải tự
nhân lại một lần nữa ở ngoài, bằng đúng công thức mà hàm vừa dùng — chép lại
một phép tính đã có sẵn, và chép ở chỗ không ai nghĩ tới lúc sửa công thức.

Cách thoát nằm ở một chỗ bất ngờ: viết hai cái tên sau `return`, cách nhau một
dấu phẩy.

```python
def tinh_hoa_don(so_to, gia, thue=0.1):
    """Đưa ra tiền hàng và tiền thuế của một bàn."""
    tien_hang = so_to * gia
    tien_thue = int(tien_hang * thue)
    return tien_hang, tien_thue
```

Nghe như hàm đưa ra hai thứ. Thật ra không phải, và chỗ này quan trọng: dấu
phẩy **buộc hai giá trị lại thành một cái gói**, rồi đúng một cái gói ấy đi ra
khỏi hàm. Cửa ra vẫn chỉ có một, vẫn đưa ra đúng một thứ — thứ ấy là cái gói.

Hình dung như túi đồ ăn mang về: bà chủ đưa cho bạn một cái túi, dù trong túi
có hộp phở, hộp hành và một cốc trà đá. Bạn nhận **một** túi, không phải ba
lần nhận.
::::

::::example{#nhin-vao-cai-goi}
Nhận cái gói rồi mở ra xem trong đó có gì:

```python title=readonly
def tinh_hoa_don(so_to, gia, thue=0.1):
    """Đưa ra tiền hàng và tiền thuế của một bàn."""
    tien_hang = so_to * gia
    tien_thue = int(tien_hang * thue)
    return tien_hang, tien_thue

goi = tinh_hoa_don(2, 45000)

print(goi)
print(type(goi))
print(goi[0])
print(goi[1])
```

Máy in ra:

```text
(90000, 9000)
<class 'tuple'>
90000
9000
```

Bốn dòng ấy nói ra bốn chuyện:

- **`(90000, 9000)`** — một cái tên `goi` đang giữ cả hai con số. Cặp ngoặc
  tròn là cách máy vẽ ra cái gói khi in nó lên màn hình.
- **`<class 'tuple'>`** — cái gói cũng là một kiểu giá trị đàng hoàng, y như
  `int` và `str` mà bạn đã hỏi `type()` ở Realm 0. Tên của nó là **tuple**.
- **`goi[0]`** và **`goi[1]`** — lấy từng phần ra bằng chỗ đứng, đếm từ 0, đúng
  lối bạn đã lấy món đầu tiên ra khỏi thực đơn ở Realm 0.

Thứ tự trong gói là thứ tự bạn viết ở dòng `return`. Viết `return tien_hang,
tien_thue` thì phần thứ nhất là tiền hàng; đổi hai cái tên ấy cho nhau thì
`goi[0]` thành tiền thuế, và mọi chỗ đọc gói ở bên ngoài đều đọc sai — mà máy
không kêu tiếng nào, vì cả hai đều là số.

Nên dòng `return` nhiều giá trị là một lời hứa về **thứ tự**, và docstring là
chỗ nói lời hứa ấy ra cho người sau đọc được.
::::

::::predict{#goi-hay-hai-lan-in commitOnce}
Byte bỏ hai dòng `goi[0]`, `goi[1]` đi và in thẳng cái tên `ket_qua`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def tinh_hoa_don(so_to, gia, thue=0.1):
    """Đưa ra tiền hàng và tiền thuế của một bàn."""
    tien_hang = so_to * gia
    tien_thue = int(tien_hang * thue)
    return tien_hang, tien_thue

ket_qua = tinh_hoa_don(1, 30000)
print(ket_qua)
```

:::opt{correct}
(30000, 3000)
:::

:::opt
30000 3000
::why
Gần đúng ở chỗ bạn đọc ra được cả hai con số và đúng thứ tự của chúng: tiền
hàng trước, tiền thuế sau. Phần tính toán của bạn không sai chỗ nào.

Chỗ lệch là hình dạng của thứ được in. `30000 3000` là thứ hiện ra khi bạn gọi
`print` với **hai** đối số, cách nhau dấu phẩy — lúc đó `print` nhận hai giá
trị rời và tự đặt một dấu cách vào giữa. Ở đây `print` chỉ nhận đúng **một**
đối số: cái tên `ket_qua`, và cái tên ấy đang giữ một cái gói. In một cái gói
thì máy vẽ luôn cả cặp ngoặc tròn lẫn dấu phẩy, để bạn nhìn ra ranh giới giữa
các phần.
::
:::

:::opt
30000
::why
Gần đúng ở chỗ bạn giữ đúng một luật đã học và giữ rất chắc: `return` đưa ra
một giá trị, không phải hai. Luật ấy vẫn còn nguyên giá trị.

Chỗ lệch là chuyện gì xảy ra với con số thứ hai. Máy không vứt nó đi và cũng
không đưa ra trước rồi thôi — dấu phẩy đã buộc cả hai lại thành **một** giá trị
duy nhất trước khi cửa ra mở. Nên `return` vẫn đưa ra đúng một thứ, và thứ đó
đủ nặng để chứa cả hai con số.
::
:::

:::opt
Máy báo lỗi, vì sau `return` chỉ được viết một giá trị
::why
Gần đúng ở chỗ bạn đang cẩn thận đúng cách: Python có luật rất cứng về số lượng
ở nhiều chỗ — đưa thiếu đối số là `TypeError`, đặt đối số vị trí sau đối số
theo tên là `SyntaxError`. Nghi ngờ `return` cũng có một luật như vậy là suy
luận hợp lý.

Chỗ lệch là ở đây dấu phẩy không tạo ra hai giá trị để mà đếm. Nó là cách viết
một cái gói: `tien_hang, tien_thue` là **một** biểu thức, y như `2 + 3` là một
biểu thức chứ không phải hai con số. `return` nhìn thấy đúng một thứ và đưa
thứ đó ra.
::
:::
::::

::::explain{#goi-di-duoc-nhung-dau-ham-den}
Cái gói đi ra khỏi hàm rồi thì dùng được ngay ở mọi chỗ mà một giá trị dùng
được: đặt cho nó một cái tên, đưa nó cho `print`, hay đưa thẳng vào một hàm
khác.

```python
goi = tinh_hoa_don(3, 40000)
print(f"Tiền hàng: {goi[0]} đồng")
print(f"Tiền thuế: {goi[1]} đồng")
print(f"Phải trả: {goi[0] + goi[1]} đồng")
```

Đây cũng là lúc nhìn lại chữ ký hàm với một con mắt khác. Người gọi
`tinh_hoa_don` mà không mở thân hàm ra xem thì họ biết gì? Họ biết tên hàm, ba
tham số và một điều mới: hàm này đưa ra **một gói hai phần**, phần đầu là tiền
hàng, phần sau là tiền thuế.

Điều mới ấy không tự hiện ra ở đâu cả. Dòng `def` không hề nói hàm trả về mấy
phần — muốn biết thì phải đọc dòng `return`, tức là mở thân hàm ra, đúng thứ mà
cả mạch bài này đang cố tránh. Nên chỗ để nói ra là docstring, và nói bằng
câu chữ của người:

```python
def tinh_hoa_don(so_to, gia, thue=0.1):
    """Đưa ra một gói hai phần: tiền hàng trước, tiền thuế sau."""
```

> Chỗ dễ vấp: quên mất là mình đang cầm một cái gói. `goi + 1000` không cộng
> thêm nghìn đồng nào — máy dừng bằng `TypeError`, vì một cái gói và một con số
> không cộng với nhau được. Muốn cộng thì phải nói rõ đang cộng vào phần nào:
> `goi[0] + 1000`.
::::

::::code{#buoc-hai-con-so-lai}
Hàm dưới đây đã tính xong cả hai con số, nhưng dòng `return` còn để trống — nên
lúc này chưa có gì đi ra khỏi hàm.

Hai dòng `print` bên dưới đang chờ một cái gói hai phần: phần đầu là tiền hàng,
phần sau là tiền thuế. Hãy viết dòng `return` cho đúng lời hứa ấy.

```python title=starter
def tinh_hoa_don(so_to, gia, thue=0.1):
    """Đưa ra một gói hai phần: tiền hàng trước, tiền thuế sau."""
    tien_hang = so_to * gia
    tien_thue = int(tien_hang * thue)
    return ___

goi = tinh_hoa_don(3, 40000)

print(f"Tiền hàng: {goi[0]} đồng")
print(f"Tiền thuế: {goi[1]} đồng")
```

```python title=solution
def tinh_hoa_don(so_to, gia, thue=0.1):
    """Đưa ra một gói hai phần: tiền hàng trước, tiền thuế sau."""
    tien_hang = so_to * gia
    tien_thue = int(tien_hang * thue)
    return tien_hang, tien_thue

goi = tinh_hoa_don(3, 40000)

print(f"Tiền hàng: {goi[0]} đồng")
print(f"Tiền thuế: {goi[1]} đồng")
```

```python title=test
# Một bàn khác hẳn bàn trong khung, để cái gói phải được tính lại chứ không
# phải chép cứng hai con số của bàn kia.
goi_kiem = tinh_hoa_don(2, 45000)
assert len(goi_kiem) == 2, "hàm phải đưa ra một gói gồm ĐÚNG hai phần — một phần thì hai dòng in bên dưới không đủ chỗ lấy, ba phần thì có một phần không ai đọc tới"
# Khái niệm mới của bài là dấu phẩy buộc hai giá trị thành một TUPLE — bài in
# hẳn `<class 'tuple'>` ra màn hình để chốt. Không có dòng này thì
# `return [tien_hang, tien_thue]` — một list — qua sạch mọi assert còn lại.
assert type(goi_kiem) is tuple, "dấu phẩy sau `return` buộc hai con số thành một tuple; gói chúng vào dấu ngoặc VUÔNG thì ra một list — chạy được, nhưng đó là kiểu khác, và cái gói mà bài này nói tới không còn là cái bạn vừa tạo"
assert goi_kiem[0] == 90000, "phần ĐẦU của gói là tiền hàng: hai tô 45000 là 90000 đồng"
assert goi_kiem[1] == 9000, "phần SAU của gói là tiền thuế: mức thường của 90000 tiền hàng là 9000 đồng — nếu hai con số này đổi chỗ cho nhau thì hoá đơn in ngược"
# Gói vẫn phải đúng khi người gọi tự đưa mức thuế khác.
goi_do = tinh_hoa_don(1, 30000, thue=0.08)
assert goi_do[0] == 30000, "một tô 30000 thì tiền hàng là 30000, mức thuế không đụng tới phần này"
assert goi_do[1] == 2400, "thuế 0.08 trên 30000 tiền hàng là 2400 đồng"
```

:::hints
- kind: attention
  body: Hai con số cần đưa ra đã nằm sẵn trong hai cái tên ở hai dòng ngay trên `return`. Không phải tính thêm gì cả, chỉ là nói cho máy biết đưa ra thứ gì.
- kind: strategy
  body: Hai dòng `print` bên dưới đọc `goi[0]` cho tiền hàng và `goi[1]` cho tiền thuế, nên thứ tự bạn viết sau `return` phải khớp đúng như vậy. Thứ nối hai cái tên lại thành một gói là một dấu phẩy, không phải dấu cộng.
- kind: one-line
  body: 'Viết `return tien_hang, tien_thue` — hai cái tên, một dấu phẩy ở giữa.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Tiền hàng: 120000 đồng\nTiền thuế: 12000 đồng\s*$
- tier: output
  expect: Tiền thuế: 12000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một cửa ra, một cái gói, hai con số. Hoá đơn in đủ cả hai dòng rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại đoạn code bạn vừa viết, phía bên ngoài hàm:

```python
goi = tinh_hoa_don(3, 40000)

print(f"Tiền hàng: {goi[0]} đồng")
print(f"Tiền thuế: {goi[1]} đồng")
```

Bên **trong** hàm, hai con số ấy có tên hẳn hoi: `tien_hang` và `tien_thue`.
Đọc dòng nào cũng biết ngay đang nói về cái gì.

Bên **ngoài**, cả hai cái tên đó đã rụng mất. Còn lại đúng một cái tên `goi`,
và muốn nói tới tiền thuế thì phải viết `goi[1]` — một con số thứ tự, không
phải một cái tên. Bài trước vừa mất công đuổi bốn con số vô nghĩa ra khỏi chỗ
gọi hàm, và bây giờ có hai con số vô nghĩa mới bò vào, lần này nằm trong dấu
ngoặc vuông. Thêm một phần nữa vào gói là mọi chỗ đếm ở ngoài đều phải đếm lại.

Nhận cái gói rồi lại phải đếm `[0]`, `[1]` mới lấy được từng phần. Có cách nào
đặt tên thẳng cho từng phần ngay lúc nhận?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
