---
id: nen-tang.ham-vien-gach.tham-so-co-san-gia-tri
title: Tham số mang sẵn giá trị
summary: Viết `thue=0.1` ngay trong dòng `def` — bỏ trống lúc gọi thì máy điền giá trị đã để sẵn ở đó.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.default-parameter]
practices: [core.keyword-argument, core.function-call, core.int-cast, core.fstring]
requires: [core.keyword-argument, core.function-def, core.function-call, core.function-parameter, core.function-return, core.function-arg-count, core.docstring, core.int-cast, core.fstring, err.type-error, err.syntax-error]
concepts: [core.ham, core.doi-so]
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
Chín dòng giống hệt nhau ở cùng một chỗ. Chắc phải có cách nói một lần.
::::

::::explain{#chin-lan-go-lai-mot-thu}
Mười lời gọi của Byte chiều qua, xếp cạnh nhau, trông như thế này:

```python
tinh_hoa_don(so_to=2, gia=45000, thue=0.1)
tinh_hoa_don(so_to=1, gia=40000, thue=0.1)
tinh_hoa_don(so_to=3, gia=45000, thue=0.1)
tinh_hoa_don(so_to=2, gia=50000, thue=0.08)
tinh_hoa_don(so_to=1, gia=45000, thue=0.1)
```

Cột đầu đổi mỗi dòng. Cột giữa đổi mỗi dòng. Cột cuối thì chín trên mười dòng
là cùng một mẩu chữ.

Chép tay một thứ chín lần không phải chuyện dài dòng cho vui. Nó tốn hai lần:
lần đầu là gõ, lần sau là **sửa** — mai quán đổi mức thuế thường sang `0.09`
thì phải dò lại đủ chín dòng, sót một dòng là hoá đơn của đúng một bàn sai mà
không ai biết.

Chỗ đúng để nói "mức thuế thường là `0.1`" không phải mười chỗ gọi. Nó là chỗ
duy nhất biết `thue` nghĩa là gì: dòng `def`.

Python cho viết thẳng một giá trị vào ngay chỗ ấy:

```python
def tinh_hoa_don(so_to, gia, thue=0.1):
    """Tiền một bàn đã cộng thuế. Không nói thuế thì tính mức thường."""
    tien_hang = so_to * gia
    tien_thue = int(tien_hang * thue)
    return tien_hang + tien_thue
```

Cái `thue=0.1` ở dòng `def` gọi là **tham số mặc định**. Nó nói: lúc gọi mà
không ai đưa giá trị nào cho `thue`, thì máy tự lấy `0.1` điền vào.

Trông giống hệt lối gọi theo tên của bài trước — cùng một cái tên, cùng dấu
`=`, cùng một con số. Nhưng hai chỗ ấy làm hai việc ngược nhau, và đây là chỗ
đáng dừng lại một nhịp:

- Trong dòng **`def`**, `thue=0.1` là lời hứa của người **viết** hàm: thiếu thì
  tôi lấy con số này.
- Trong dòng **gọi**, `thue=0.08` là lời dặn của người **dùng** hàm: lần này
  đừng lấy con số kia, lấy con số của tôi.
::::

::::example{#bo-trong-hay-noi-ro}
Cùng một hàm, hai kiểu gọi:

```python title=readonly
def tinh_hoa_don(so_to, gia, thue=0.1):
    """Tiền một bàn đã cộng thuế. Không nói thuế thì tính mức thường."""
    tien_hang = so_to * gia
    tien_thue = int(tien_hang * thue)
    return tien_hang + tien_thue

print(tinh_hoa_don(2, 45000))
print(tinh_hoa_don(2, 45000, thue=0.08))
```

Máy in ra:

```text
99000
97200
```

Dòng gọi thứ nhất chỉ đưa hai giá trị, trong khi hàm có ba tham số. Ở bài về
đưa thiếu đưa thừa, đưa thiếu là `TypeError` ngay lập tức. Lần này thì không:
`thue` đã có sẵn một giá trị nằm chờ ở dòng `def`, nên nó không còn là chỗ
trống bắt buộc phải điền nữa.

Đi theo con số cho rõ. Hai tô bốn mươi lăm nghìn nên `tien_hang` là 90000.
Máy lấy `thue` từ dòng `def` ra, được `0.1`, nên `tien_thue` là 9000 và tổng là
99000. Lời gọi thứ hai đưa hẳn `0.08` vào nên `tien_thue` thành 7200, tổng
97200.

Vậy ba tham số của hàm này chia làm hai loại:

- `so_to` và `gia` — **bắt buộc**. Không có giá trị sẵn, thiếu là dừng.
- `thue` — **có mặc định**. Đưa thì máy dùng cái bạn đưa, không đưa thì máy
  dùng cái viết sẵn.

> `int(tien_hang * thue)` có mặt để cắt phần lẻ đi. Phép nhân với một số thập
> phân cho ra một số thập phân, mà tiền phở thì không có phần lẻ — nên hàm cắt
> nó ngay tại chỗ, đúng như bạn đã làm với `int()` ở Realm 0.
::::

::::predict{#thu-tu-hai-loai-tham-so commitOnce}
Byte thấy `thue` hay được nói tới hơn `so_tra_da`, nên dọn lại dòng `def` cho
tham số có mặc định đứng lên trước.

**Trước khi bấm chạy**, bạn đoán máy làm gì với đoạn này?

```python
def tinh_hoa_don(thue=0.1, so_to, gia):
    """Tiền một bàn đã cộng thuế."""
    tien_hang = so_to * gia
    return tien_hang + int(tien_hang * thue)

print(tinh_hoa_don(2, 45000))
```

:::opt{correct}
Máy báo `SyntaxError` ngay ở dòng `def`, chưa gọi hàm lần nào
:::

:::opt
In ra `99000` — đổi chỗ tham số trong `def` thì lời gọi cũng đổi theo
::why
Gần đúng ở chỗ bạn giữ đúng một sợi dây quan trọng: chỗ đứng trong `def` và chỗ
đứng trong lời gọi luôn khớp với nhau, nên đổi bên này là bên kia đổi theo. Sợi
dây ấy có thật.

Chỗ lệch là dòng `def` này không tồn tại được để mà khớp với ai. Đặt một tham
số có mặc định lên trước một tham số bắt buộc thì máy từ chối ngay lúc đọc,
bằng câu `parameter without a default follows parameter with a default` — file
chưa chạy được một dòng nào.

Vì sao Python chặn: nếu nó cho phép, con số trần đầu tiên bạn đưa vào sẽ luôn
rơi vào `thue`, nên muốn `thue` dùng giá trị mặc định thì `so_to` với `gia`
bắt buộc phải gọi theo tên — cái mặc định vừa viết ra đã thành vô dụng. Chặn
sớm ở đây là chặn trước một cái bẫy, không phải một quy định tuỳ hứng.
::
:::

:::opt
In ra `99000`, vì máy thấy `thue` đã có sẵn giá trị nên bỏ qua nó và đếm tiếp
::why
Gần đúng ở chỗ bạn mô tả đúng thứ máy **muốn** làm: `thue` có giá trị sẵn rồi,
vậy hai con số `2` và `45000` rơi vào hai chỗ còn lại là hợp lý.

Chỗ lệch là "bỏ qua rồi đếm tiếp" mở ra đúng cái mơ hồ mà bài trước đã nói tới.
Nếu máy làm vậy thì `tinh_hoa_don(2, 45000)` đưa `2` cho `so_to`, còn
`tinh_hoa_don(0.08, 2, 45000)` lại đưa `2` cho `so_to` sau khi đã đưa `0.08`
cho `thue` — cùng một chỗ đứng, hai lần mang hai ý nghĩa khác nhau. Python chặn
chuyện đó từ gốc: tham số bắt buộc phải đứng trước, mặc định đứng sau.
::
:::

:::opt
Máy báo `TypeError` lúc gọi hàm, vì lời gọi đưa thiếu một đối số
::why
Gần đúng ở chỗ bạn nhớ đúng loại lỗi của việc đưa thiếu: `TypeError`, và nó nổ
ra tại dòng gọi.

Chỗ lệch là ở đây máy chưa bao giờ đi tới dòng gọi. Cả dòng `def` mới là chỗ
bị từ chối, và nó bị từ chối lúc máy **đọc** file chứ không phải lúc chạy —
cùng loại với dấu hai chấm bị quên. Chương trình không chạy dòng nào cả, nên
lời gọi ở dưới có đúng có sai cũng chưa tới lượt.
::
:::
::::

::::explain{#luat-thu-tu-va-cho-dat-mac-dinh}
Luật rút ra từ đoạn vừa rồi cũng chỉ một câu: **trong dòng `def`, tham số bắt
buộc đứng trước, tham số có mặc định đứng sau.**

Nó là anh em ruột với luật của bài trước. Bài trước nói về thứ tự trong dấu
ngoặc lúc **gọi**: cái theo vị trí trước, cái theo tên sau. Bài này nói về thứ
tự trong dấu ngoặc lúc **khai báo**: cái bắt buộc trước, cái có sẵn sau. Cả hai
đều sinh ra từ một chuyện duy nhất — máy đếm chỗ từ trái sang phải, nên mọi thứ
làm cho việc đếm chỗ trở nên mơ hồ đều bị cấm.

Đặt giá trị nào làm mặc định thì hợp lý? Câu trả lời không nằm trong Python, nó
nằm ở quán phở: **giá trị đúng trong đa số lần gọi**. Mức thuế thường là `0.1`
vì chín trên mười bàn dùng nó. Nếu chín trên mười bàn xin hoá đơn đỏ thì mặc
định phải là `0.08`, và người viết hàm mới là người biết điều đó.

Còn `so_to` thì không có mặc định nào hợp lý cả. Không ai vào quán rồi không ăn
tô nào, và cũng không có con số nào đoán hộ được. Tham số như vậy phải để bắt
buộc, để người gọi buộc phải nói ra.

> Chỗ dễ vấp: đặt mặc định cho một tham số mà giá trị "thường thấy" thật ra
> không thường tới thế. Lúc đó hàm vẫn chạy trơn tru, và những lời gọi quên đưa
> giá trị sẽ lặng lẽ dùng một con số sai. Mặc định giấu bớt được việc gõ, nhưng
> nó giấu luôn cả cái nó điền vào.
::::

::::code{#dat-muc-thue-thuong}
Hàm dưới đây còn hai chỗ trống.

Chỗ thứ nhất nằm ở dòng `def`: quán lấy mức thuế thường là `0.1`, và đó là con
số máy phải tự điền khi không ai nói gì.

Chỗ thứ hai nằm ở lời gọi cho bàn 2: bàn ấy xin hoá đơn đỏ nên tính thuế `0.08`
— lần này phải nói rõ ra, vì mức của bàn này khác mức thường.

```python title=starter
def tinh_hoa_don(so_to, gia, thue=___):
    """Tiền một bàn đã cộng thuế. Không nói thuế thì tính mức thường."""
    tien_hang = so_to * gia
    tien_thue = int(tien_hang * thue)
    return tien_hang + tien_thue

print(f"Bàn 1: {tinh_hoa_don(2, 45000)} đồng")
print(f"Bàn 2: {tinh_hoa_don(3, 45000, thue=___)} đồng")
```

```python title=solution
def tinh_hoa_don(so_to, gia, thue=0.1):
    """Tiền một bàn đã cộng thuế. Không nói thuế thì tính mức thường."""
    tien_hang = so_to * gia
    tien_thue = int(tien_hang * thue)
    return tien_hang + tien_thue

print(f"Bàn 1: {tinh_hoa_don(2, 45000)} đồng")
print(f"Bàn 2: {tinh_hoa_don(3, 45000, thue=0.08)} đồng")
```

```python title=test
# Gọi thiếu `thue`: con số phải là con số mặc định, không phải con số nào khác.
assert tinh_hoa_don(2, 45000) == 99000, "hai tô 45000 là 90000 tiền hàng, mức thuế thường cho ra 9000 tiền thuế, tổng 99000 — ra số khác nghĩa là giá trị mặc định ở dòng `def` chưa đúng"
# Một bàn khác, cũng gọi thiếu `thue`: mặc định phải là một con số cố định,
# không phải thứ tình cờ khớp với đúng một bàn.
assert tinh_hoa_don(1, 30000) == 33000, "một tô 30000 thì tiền thuế mức thường là 3000, tổng 33000 — mặc định phải đúng cho mọi bàn chứ không riêng bàn 1"
# Nói rõ mức thuế lúc gọi thì máy phải nghe theo người gọi, bỏ mặc định đi.
assert tinh_hoa_don(3, 45000, thue=0.08) == 145800, "ba tô 45000 là 135000 tiền hàng, thuế 0.08 cho ra 10800, tổng 145800 — mức người gọi đưa vào phải thắng mức mặc định"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm ở hai loại dòng khác nhau. Chỗ thứ nhất ở dòng `def` là chỗ hàm tự để dành cho mình; chỗ thứ hai ở dòng gọi là chỗ người dùng hàm nói khác đi.
- kind: strategy
  body: Cả hai chỗ đều cần một con số thuế, và bài đã cho cả hai con số ấy ở đoạn ngay trên khung code — một mức thường cho chín bàn, một mức riêng cho bàn xin hoá đơn đỏ. Dòng in của bàn 1 không nhắc tới `thue` lần nào, nên con số nó dùng phải tới từ dòng `def`.
- kind: one-line
  body: 'Chỗ trống ở dòng `def` điền `0.1`, chỗ trống ở lời gọi của bàn 2 điền `0.08`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Bàn 1: 99000 đồng\nBàn 2: 145800 đồng\s*$
- tier: output
  expect: Bàn 2: 145800 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chín dòng bớt được một mẩu, và mức thuế giờ nằm đúng một chỗ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại hai bài vừa rồi, cả hai đều làm cùng một việc cho cùng một phía của
hàm: phía **đi vào**. Gọi lộn xộn thì đặt tên cho từng đối số. Gọi thiếu thì để
sẵn một giá trị trong `def`. Chỗ trống đi vào hàm giờ mềm dẻo hẳn.

Còn phía **đi ra** thì vẫn y như lúc Realm 0 dạy `return`: đúng một giá trị, một
lần, rồi hết.

Nó thành ra chật ngay trong chính cái hàm bạn vừa viết. Bên trong
`tinh_hoa_don` có hai con số đáng nói: `tien_hang` và `tien_thue`. Hoá đơn của
quán in cả hai dòng ấy. Vậy mà hàm cộng chúng lại rồi chỉ đưa ra một con số
tổng — hai con số kia biến mất cùng lúc hàm chạy xong, và người gọi muốn biết
tiền thuế thì phải tự nhân lại một lần nữa ở bên ngoài, bằng chính công thức mà
hàm vừa dùng.

Chiều VÀO giờ đã linh hoạt: thiếu có mặc định, lộn xộn thì gọi theo tên. Nhưng
chiều RA vẫn kẹt ở đúng một giá trị. Tính hoá đơn xong muốn biết cả tiền hàng
LẪN tiền thuế thì làm sao?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
