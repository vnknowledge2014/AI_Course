---
id: nen-tang.ham-vien-gach.goi-hai-lan-ra-hai-ket-qua
title: Cùng đầu vào, khác đầu ra
summary: Hàm chạm vào thứ nằm ngoài mình thì kết quả của nó phụ thuộc vào những gì đã xảy ra trước đó — chỗ chạm ấy gọi là tác dụng phụ.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.side-effect]
requires: [core.global-keyword, core.local-assignment, core.function-def, core.function-parameter, core.function-return, core.function-call, core.floor-division, core.fstring]
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
Mình hỏi nó hai lần y hệt nhau. Nó trả lời hai kiểu khác nhau.
::::

::::explain{#cung-cau-hoi-khac-cau-tra-loi}
Bài trước kết thúc bằng một chuyện lạ. Hai lời gọi giống nhau từng chữ:

```python title=readonly
ghi_doanh_thu(45000)
ghi_doanh_thu(45000)
```

Lần đầu báo tổng 45 nghìn, lần sau báo 90 nghìn. Cùng cái tên hàm, cùng đối số,
mà hai câu trả lời khác nhau.

So với `len("phở")` ở đầu mạch này thì lạ thật. `len` gọi bao nhiêu lần, gọi
lúc nào, xen giữa bao nhiêu việc khác — vẫn đưa ra đúng một con số, vì nó chỉ
nhìn vào đúng thứ bạn đưa cho nó.

`ghi_doanh_thu` thì nhìn thêm một thứ nữa. Câu trả lời của nó không chỉ dựa vào
`tien` bạn đưa vào, mà còn dựa vào con số đang nằm trên tấm bảng ngoài kia — mà
con số đó thì đổi theo những gì đã xảy ra trước đó.

Nghĩ về một cái máy tính bỏ túi. Bấm `45000 + 0` thì hôm nay hay ngày mai cũng
ra một kết quả. Còn hỏi bà chủ quán "sáng nay bán được bao nhiêu rồi?" thì câu
trả lời phụ thuộc vào lúc bạn hỏi: hỏi lúc 7 giờ khác, hỏi lúc 11 giờ khác. Bà
không hề trả lời sai — chỉ là câu trả lời của bà **có nhớ** những gì xảy ra
trước đó.

Chỗ khiến một hàm giống bà chủ quán hơn giống cái máy tính bỏ túi có một cái
tên: **tác dụng phụ**.

Một hàm có tác dụng phụ là hàm **chạm vào thứ nằm ngoài mình**. Chạm bằng nhiều
đường:

- ghi vào một biến toàn cục — đúng thứ `global` vừa mở ra ở bài trước;
- in một dòng lên màn hình;
- ghi thêm một dòng vào một file trong máy.

Cả ba đều để lại dấu vết ở bên ngoài, và dấu vết ấy còn nguyên khi lượt gọi đã
xong. Nên lượt gọi sau bước vào một thế giới đã khác lượt trước.
::::

::::example{#hai-ham-dat-canh-nhau}
Cách nhìn ra nhanh nhất là đặt hai hàm cạnh nhau rồi gọi mỗi hàm đúng hai lần,
với đối số y hệt nhau:

```python title=readonly
tong_doanh_thu = 0

def ghi_doanh_thu(tien):
    global tong_doanh_thu
    tong_doanh_thu = tong_doanh_thu + tien
    return tong_doanh_thu

def tinh_thue(tien):
    return tien // 10

print(ghi_doanh_thu(45000))
print(ghi_doanh_thu(45000))
print(tinh_thue(45000))
print(tinh_thue(45000))
```

Máy in ra:

```text
45000
90000
4500
4500
```

Hai hàm, hai kiểu cư xử khác hẳn nhau.

`tinh_thue` chỉ có đúng một đường vào — cái tham số `tien` — và đúng một đường
ra là `return`. Nó không đọc gì ngoài, không ghi gì ngoài. Nên hai dòng cuối
giống hệt nhau, và chúng sẽ còn giống hệt nhau dù bạn gọi nó lúc nào, gọi lần
thứ mấy, hay chen bao nhiêu việc khác vào giữa.

`ghi_doanh_thu` thì có một đường vào thứ hai và một đường ra thứ hai, cả hai
đều không hiện trong chữ ký: nó **đọc** `tong_doanh_thu` và **ghi** vào chính
cái tên ấy. Hai đường ngầm đó nối lượt gọi này với lượt gọi trước. Nên số 45000
ở dòng đầu và số 90000 ở dòng sau đều đúng cả — chúng chỉ trả lời cho hai thời
điểm khác nhau.
::::

::::predict{#doan-hai-dong-in commitOnce}
Byte gom tiền hai bàn vào cùng một cái tên sống ngoài hàm. Hai bàn cùng gọi một
tô 10 nghìn.

**Trước khi bấm chạy**, bạn đoán hai dòng in ra hai con số nào?

```python title=readonly
tong = 0

def cong_them(tien):
    global tong
    tong = tong + tien
    return tong

print(cong_them(10000))
print(cong_them(10000))
```

:::opt{correct}
10000 rồi 20000
:::

:::opt
10000 rồi 10000
::why
Gần đúng ở chỗ bạn đang dùng một luật rất chắc và rất đáng tin ở mọi nơi khác:
cùng một hàm, cùng một đối số thì phải cùng một kết quả. Với `len`, với
`round`, với `tinh_thue` vừa rồi, luật ấy đúng nguyên.

Chỗ lệch là cái tên `tong` không sinh ra mới ở mỗi lượt gọi. Nó nằm **ngoài**
hàm, và dòng `global` cho phép lượt gọi đầu ghi thẳng vào đó. Lượt gọi thứ hai
đọc ra 10000 chứ không phải 0, vì lượt trước đã để lại dấu vết. Đó chính là
tác dụng phụ — và nó là lý do luật quen thuộc kia không áp được vào đây.
::
:::

:::opt
20000 rồi 20000
::why
Gần đúng ở chỗ bạn đọc ra con số cuối cùng của `tong` không sai một đồng: sau
hai lượt cộng, cái tên ấy mang 20000 thật.

Chỗ lệch là thời điểm. `print` không đợi tới lúc chương trình kết thúc rồi mới
nhìn lại; nó in ngay tại dòng của nó, với con số mà `return` vừa đưa ra ở đúng
lượt gọi ấy. Lúc dòng đầu chạy, hàm mới cộng được một lần — nên nó in 10000.
Con số 20000 chỉ có mặt từ lượt gọi thứ hai trở đi.
::
:::

:::opt
0 rồi 10000
::why
Gần đúng ở chỗ bạn bám đúng vào giá trị khởi đầu: `tong` sinh ra bằng 0, và
chuyện lượt sau đọc được thứ lượt trước để lại thì bạn nắm chắc rồi — hai con
số của bạn cách nhau đúng 10000.

Chỗ lệch nằm ở thứ tự hai dòng trong thân hàm. Dòng cộng chạy **trước**, dòng
`return` chạy sau và đọc lại `tong` tại đúng thời điểm nó chạy tới. Nên thứ đi
ra ngoài là con số **sau** khi đã cộng, không phải con số lúc vừa bước vào hàm.
::
:::
::::

::::explain{#cai-gia-cua-mot-duong-ngam}
Tác dụng phụ không phải một lỗi. Không có nó thì chương trình chẳng nói được
câu nào ra màn hình, và cái tên `tong_doanh_thu` của Byte cũng không cộng dồn
nổi. Cả hai bài vừa rồi đều là những việc thật, cần làm thật.

Nhưng nó có giá, và giá gồm ba khoản:

- **Thứ tự gọi trở thành một phần của lời giải.** Đổi chỗ hai lời gọi
  `tinh_thue` cho nhau thì không ai nhận ra khác biệt. Đổi chỗ hai lời gọi có
  tác dụng phụ thì kết quả đổi theo — và loại lỗi ấy không báo gì cả, nó chỉ
  cho ra con số sai.
- **Thử riêng một hàm không còn đủ.** Muốn biết `tinh_thue` chạy đúng không,
  bạn gọi nó một lần với một con số là xong. Muốn biết `ghi_doanh_thu` chạy
  đúng không, bạn còn phải dựng đúng cái tên ngoài kia với đúng giá trị của nó
  ở đúng thời điểm ấy.
- **Chữ ký nói không hết.** Nhìn `def ghi_doanh_thu(tien)` thì tưởng hàm chỉ
  cần một con số. Thật ra nó còn cần biết `tong_doanh_thu` đang là bao nhiêu,
  và nó còn sửa cái tên ấy nữa. Hai chuyện đó nằm trong thân hàm, không nằm
  trên dòng `def` — mà bài 2 thì hứa rằng đọc chữ ký là gọi được.

> Chỗ dễ vấp: đừng đọc bài này thành "tác dụng phụ là xấu". Hãy đọc nó thành
> "tác dụng phụ là thứ bạn phải biết mình đang có". Một hàm có tác dụng phụ mà
> bạn cố ý viết ra và nhớ nó ở đó thì không sao; một hàm có tác dụng phụ mà
> bạn quên mất mới là chỗ sinh ra những buổi tối đi tìm lỗi.
::::

::::code{#dat-hai-ham-len-can}
Cách chắc chắn nhất để thấy tác dụng phụ là gọi cùng một hàm hai lần với cùng
một đối số, rồi giữ lại cả hai kết quả để đặt cạnh nhau.

Đoạn dưới đã gọi mỗi hàm **một** lần. Hãy điền hai chỗ trống sao cho mỗi hàm
được gọi thêm đúng một lần nữa, vẫn với con số 45000 y như lần đầu.

```python title=starter
tong_doanh_thu = 0

def ghi_doanh_thu(tien):
    global tong_doanh_thu
    tong_doanh_thu = tong_doanh_thu + tien
    return tong_doanh_thu

def tinh_thue(tien):
    return tien // 10

lan_dau_ghi = ghi_doanh_thu(45000)
lan_sau_ghi = ___

lan_dau_thue = tinh_thue(45000)
lan_sau_thue = ___

print(f"ghi_doanh_thu(45000) hai lần: {lan_dau_ghi} rồi {lan_sau_ghi}")
print(f"tinh_thue(45000) hai lần: {lan_dau_thue} rồi {lan_sau_thue}")
```

```python title=solution
tong_doanh_thu = 0

def ghi_doanh_thu(tien):
    global tong_doanh_thu
    tong_doanh_thu = tong_doanh_thu + tien
    return tong_doanh_thu

def tinh_thue(tien):
    return tien // 10

lan_dau_ghi = ghi_doanh_thu(45000)
lan_sau_ghi = ghi_doanh_thu(45000)

lan_dau_thue = tinh_thue(45000)
lan_sau_thue = tinh_thue(45000)

print(f"ghi_doanh_thu(45000) hai lần: {lan_dau_ghi} rồi {lan_sau_ghi}")
print(f"tinh_thue(45000) hai lần: {lan_dau_thue} rồi {lan_sau_thue}")
```

```python title=test
# Bốn con số này là cả bài học gói lại: hai con số đầu KHÁC nhau dù lời gọi
# giống hệt nhau, hai con số sau GIỐNG nhau vì hàm kia không chạm gì bên ngoài.
assert lan_dau_ghi == 45000, "lượt ghi đầu tiên bước vào lúc cái tên ngoài hàm còn là 0, nên nó phải đưa ra 45 nghìn"
assert lan_sau_ghi == 90000, "lượt ghi thứ hai phải đưa ra 90 nghìn — nó đọc được dấu vết mà lượt đầu để lại ngoài hàm, chứ không bắt đầu lại từ 0"
assert lan_dau_thue == 4500, "thuế của 45 nghìn là 4500 — hàm này chỉ nhìn vào đối số của nó"
assert lan_sau_thue == 4500, "gọi lần thứ hai với cùng con số phải ra cùng kết quả, vì `tinh_thue` không đọc và không ghi thứ gì nằm ngoài nó"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm ngay dưới hai dòng đã viết sẵn, và mỗi chỗ trống chỉ cần lặp lại đúng lời gọi ở dòng ngay trên nó. Bài không hỏi bạn tính ra con số nào — nó hỏi bạn gọi thêm một lần rồi nhìn kết quả.
- kind: strategy
  body: Chỗ trống thứ nhất gọi lại hàm có dòng `global` bên trong, chỗ thứ hai gọi lại hàm chỉ có mỗi `return`. Cả hai lần đều đưa vào 45000 — giữ nguyên con số ấy thì hai cặp kết quả mới so được với nhau.
- kind: one-line
  body: Lần lượt hai chỗ trống là `ghi_doanh_thu(45000)` và `tinh_thue(45000)`.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: ghi_doanh_thu(45000) hai lần: 45000 rồi 90000
- tier: output
  expect: tinh_thue(45000) hai lần: 4500 rồi 4500
- tier: static
  onFail: hai chỗ trống phải là hai lời GỌI hàm, không phải con số chép sẵn
  requireAst:
  # Mỗi hàm phải xuất hiện đúng trong hai lời gọi. Khung đã có sẵn MỘT lời gọi
  # cho mỗi hàm, nên `min: 2` chính là chỗ trống đã được điền bằng một lời gọi.
  - kind: uses-call, target: ghi_doanh_thu, min: 2
  - kind: uses-call, target: tinh_thue, min: 2
  forbidAst:
  # Chép thẳng kết quả vào chỗ trống thì bài mất hết ý nghĩa: cái đáng nhìn là
  # hai con số ấy do đâu mà khác nhau, không phải bản thân hai con số.
  - kind: has-literal, target: 90000
  - kind: has-literal, target: 4500
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một hàm nhớ chuyện cũ, một hàm thì không. Giờ mình phân biệt được rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cả bài này chỉ có đúng một thủ phạm bị chỉ mặt: dòng `global`. Nó là đường ngầm
nối hàm với thứ nằm ngoài, và bỏ nó đi thì `ghi_doanh_thu` không ghi ra ngoài
được nữa — bài trước đã cho thấy như vậy.

Vậy bỏ `global` đi là hết tác dụng phụ chứ?

Thử một chuyện trước khi tin điều đó. Thử truyền một list vào hàm, `.append()`
một món bên trong, rồi in cái list ở ngoài ra xem. Trong hàm ấy chẳng có dòng
`global` nào cả.

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
