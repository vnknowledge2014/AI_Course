---
id: onboarding.ra-lenh-cho-byte.ham-tra-ket-qua-ve
title: Hàm đưa kết quả ra ngoài
summary: Phân biệt hàm hô kết quả lên màn hình với hàm đưa kết quả về tận tay.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.function-return]
requires: [core.function-parameter, err.type-error]
concepts: [core.tra-ve]
gradingMatrix:
  web-chrome: [static, run, tests, output]
  web-firefox: [static, run, tests, output]
  macos: [static, run, tests, output]
  windows: [static, run, tests, output]
  linux: [static, run, tests, output]
  android: [static, run, tests, output]
  ios: [static, run, tests, output]
provenance:
  authoredBy: human
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Hô to cho cả quán nghe, hay đưa tận tay người gọi? Hai việc khác nhau.
::::

::::explain{#ho-to-hay-dua-tan-tay}
Trong bếp quán phở có hai cách "trả lời" một yêu cầu.

Cách thứ nhất: đầu bếp **hô to** — *"Tô tái bàn ba, bốn lăm nghìn!"*. Cả quán
nghe thấy. Bạn ngồi ngoài cũng nghe. Nhưng tiếng hô tan trong không khí: người
thu ngân muốn cộng vào hoá đơn thì phải nghe kịp rồi tự nhớ lấy.

Cách thứ hai: đầu bếp **đưa tận tay** người phục vụ tờ phiếu ghi *45000*. Không
ai trong quán nghe thấy gì. Nhưng người phục vụ đang **cầm** con số trong tay,
đem cộng với phiếu bàn khác được, đem kẹp vào hoá đơn được.

`print` là hô to. Bạn đã dùng nó suốt từ bài đầu tiên: nó đưa chữ lên màn hình
cho **người** đọc, và hết nhiệm vụ.

Còn khi chương trình cần **dùng tiếp** kết quả, hàm phải đưa tận tay. Từ khoá
làm việc đó là `return`.
::::

::::example{#hai-ham-canh-nhau}
Hai hàm dưới đây tính ra cùng một con số, nhưng làm hai việc khác nhau với nó:

```python title=readonly
def bao_gia(so_to):
    print(so_to * 45000)

def tinh_tien(so_to):
    return so_to * 45000
```

`bao_gia` hô con số lên màn hình rồi thôi.

`tinh_tien` không in gì cả — chạy nó lên màn hình vẫn trắng trơn. Nó **đưa** con
số về chỗ đã gọi nó, và ở đó bạn cầm được:

```python title=readonly
tien_ban_ba = tinh_tien(2)
tien_ban_nam = tinh_tien(1)

print(f"Hai bàn tổng cộng {tien_ban_ba + tien_ban_nam} đồng")
```

Chỗ `tinh_tien(2)` đứng, sau khi hàm chạy xong, biến thành `90000`. Cả dòng
thành `tien_ban_ba = 90000` — y như bạn tự gõ con số vào đó.

Đó là lý do một hàm có `return` đem cộng được, so sánh được trong `if`, cất vào
danh sách được. Một hàm chỉ `print` thì không.
::::

::::predict{#doan-print-vs-return commitOnce}
Byte sắp chạy đoạn dưới. Hàm này dùng `print` chứ **không** dùng `return`.
**Trước khi bấm chạy**, bạn đoán điều gì xảy ra?

```python title=readonly
def bao_gia(so_to):
    print(so_to * 45000)

tong = bao_gia(2)
print(tong + 10000)
```

:::opt{correct}
In ra `90000`, rồi máy báo lỗi
:::

:::opt
In ra `90000` rồi in ra `100000`
::why
Gần đúng ở chỗ `90000` thật sự đã được tính đúng và đã hiện lên màn hình — bạn
theo dõi rất sát.

Chỗ lệch nằm ở dòng `tong = bao_gia(2)`. Con số đó đã bay lên màn hình cho bạn
đọc, chứ chưa hề được đưa về cho `tong`. Hàm không có `return`, nên chỗ
`bao_gia(2)` đứng biến thành `None` — cách Python nói *"tay không, chẳng có gì
cả"*.

`None` cộng với `10000` thì máy chịu: nó không có quy ước nào để cộng chỗ rỗng
với một con số. Đúng loại lỗi bạn đã gặp khi cộng câu chữ với số.
::
:::

:::opt
Chỉ in ra `100000`
::why
Gần đúng ở chỗ bạn hiểu rằng gọi hàm thì phần bên trong sẽ chạy — hoàn toàn
chính xác.

Nhưng phần bên trong hàm này là một lệnh `print`, và `print` đã chạy thì con số
`90000` phải hiện lên màn hình. Máy không giữ lại tiếng hô đó để dùng thay cho
kết quả. Nó hô thật, rồi trả về tay không.
::
:::

:::opt
Máy báo lỗi ngay, không in ra dòng nào
::why
Gần đúng ở chỗ bạn đoán được có lỗi — và có lỗi thật.

Chỗ lệch là **thời điểm**. Loại lỗi này chỉ lộ ra lúc máy chạy tới dòng gây lỗi,
mà dòng đó là dòng cuối. Ba dòng trước đã chạy trọn vẹn, nên `90000` kịp hiện ra
trước khi chương trình dừng.

Lỗi thấy được trước cả khi chạy dòng đầu tiên là loại khác: viết sai cú pháp,
thiếu ngoặc, thiếu nháy. Ở đây câu lệnh nào cũng viết đúng ngữ pháp.
::
:::
::::

::::explain{#return-dung-luon-ket-thuc}
Một điều cần biết ngay, vì nó gây bất ngờ cho hầu hết người mới: `return` **kết
thúc hàm ngay tại đó**.

```python title=readonly
def tinh_tien(so_to):
    return so_to * 45000
    print("Đã tính xong")

print(tinh_tien(2))
```

Dòng `print("Đã tính xong")` không bao giờ chạy. Hàm đã đưa kết quả ra ngoài rồi
thì coi như xong việc, mọi dòng phía sau bị bỏ lại.

Nghĩ theo hình ảnh bếp núc thì hợp lý: đầu bếp đưa phiếu ra khỏi cửa bếp là kết
thúc đơn hàng đó. Còn dặn dò gì thì phải nói **trước** khi đưa phiếu.
::::

::::code{#viet-return}
Hàm `tinh_tien` dưới đây tính đúng, nhưng con số không ra khỏi được hàm nên dòng
cuối in ra sai. Hãy thêm từ khoá còn thiếu vào chỗ trống.

```python title=starter
def tinh_tien(so_to):
    ___ so_to * 45000

hoa_don = tinh_tien(2)
print(f"Khách trả {hoa_don} đồng")
```

```python title=solution
def tinh_tien(so_to):
    return so_to * 45000

hoa_don = tinh_tien(2)
print(f"Khách trả {hoa_don} đồng")
```

```python title=test
# Byte tự gọi hàm với vài số tô khác nhau và cầm lấy kết quả để so.
# Cầm được kết quả như thế này chỉ có thể khi hàm đã `return`.
assert tinh_tien(2) == 90000
assert tinh_tien(1) == 45000
assert tinh_tien(0) == 0
```

:::hints
- kind: attention
  body: Con số đã tính đúng rồi. Thứ còn thiếu là lệnh đưa nó ra khỏi hàm, về chỗ đã gọi.
- kind: strategy
  body: Nếu chỗ trống này là `print`, màn hình sẽ hiện con số nhưng `hoa_don` vẫn tay không. Bạn cần từ khoá kia.
- kind: one-line
  body: "`return so_to * 45000`"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Khách trả 90000 đồng
:::
::::

::::reflect{#nghi-lai}
Nhìn lại xem bạn đã có trong tay những gì:

- Hỏi người dùng một câu và nhận lời đáp về (`input`).
- Đổi lời đáp thành con số dùng được (`int`).
- Rẽ nhánh theo điều kiện (`if` / `elif` / `else`).
- Lặp lại một việc nhiều lần (`for`).
- Giữ nhiều giá trị trong một chỗ (`list`).
- Và giờ là gói một việc dưới một cái tên, nhận thông tin vào, đưa kết quả ra.

Đủ để làm một chương trình biết trò chuyện rồi đấy.

Nghĩ trước cho bài sau: một cái máy hỏi khách *"anh chị muốn tô cỡ nào?"*, nghe
câu trả lời, rồi báo đúng giá — bạn sẽ ghép những mảnh trên theo thứ tự nào?
::::

::::checkpoint{mastery=0.8}
::::
