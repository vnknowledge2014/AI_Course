---
id: onboarding.ra-lenh-cho-byte.boss-may-tra-loi-tu-dong
title: BOSS — Máy trả lời tự động
summary: Ghép hỏi, đổi kiểu, rẽ nhánh, danh sách và hàm thành một cái máy biết trả lời khách.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
practices: [core.input, core.int-cast, ctrl.elif, core.list-index, core.function-return, core.fstring]
requires: [core.function-return, core.function-parameter, ctrl.elif, core.list-index, core.int-cast]
concepts: [core.chuong-trinh, core.thu-tu-buoc, core.ham]
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
Hôm nay không có thứ gì mới. Chỉ có việc ghép những thứ bạn đã cầm sẵn trong tay.
::::

::::explain{#bon-phan-cua-mot-cai-may}
Bài trước Byte để lại một câu hỏi: một cái máy hỏi khách *"anh chị muốn tô cỡ
nào?"*, nghe câu trả lời, rồi báo đúng giá — bạn sẽ ghép những mảnh đã học theo
thứ tự nào?

Đây là câu trả lời.

Máy gọi món ở quán ăn, tổng đài ngân hàng, ô tìm kiếm trên điện thoại — thứ nào
cũng gồm đúng bốn phần, và bốn phần ấy luôn đứng theo một thứ tự:

1. **Bày sẵn** — những gì không đổi: bảng giá, danh sách cỡ tô, và những việc đã
   được đặt tên (các hàm).
2. **Hỏi** — nói ra câu hỏi, rồi dừng lại chờ người trả lời.
3. **Tính** — cầm lấy câu trả lời, đổi nó thành thứ tính toán được, rồi rẽ nhánh
   theo nội dung câu trả lời.
4. **Đáp** — nói ra một câu cho người nghe.

Thứ tự này không phải quy ước riêng của Python. Nó là thứ tự trong bếp: bảng giá
được dán lên tường **trước** khi mở cửa quán, chứ không phải lúc khách đang đứng
chờ mới đi tìm.
::::

::::example{#ca-cai-may}
Đây là cả cái máy, viết ra hết. Từng dòng bạn đều đã gặp ở đâu đó trong Realm 0;
điều mới duy nhất là chúng đứng cạnh nhau.

```python title=readonly
cac_co_to = ["nhỏ", "vừa", "to"]

def gia_to(co_to):
    if co_to == "nhỏ":
        return 40000
    elif co_to == "vừa":
        return 45000
    elif co_to == "to":
        return 55000
    else:
        return 0

print("PHỞ THÌN — bảng giá hôm nay")
for mot_co in cac_co_to:
    print(f"- tô {mot_co}: {gia_to(mot_co)} đồng")

print("Anh chị muốn tô cỡ nào?")
co_khach_chon = input()

print("Lấy mấy tô ạ?")
so_to = int(input())

gia_mot_to = gia_to(co_khach_chon)

if gia_mot_to == 0:
    print("Quán mình chưa có cỡ tô này ạ")
else:
    print(f"{so_to} tô {co_khach_chon}, tổng cộng {gia_mot_to * so_to} đồng")
```

Một lần chạy thật, với khách gõ vào `vừa` rồi `2`:

```text title=readonly
PHỞ THÌN — bảng giá hôm nay
- tô nhỏ: 40000 đồng
- tô vừa: 45000 đồng
- tô to: 55000 đồng
Anh chị muốn tô cỡ nào?
vừa
Lấy mấy tô ạ?
2
2 tô vừa, tổng cộng 90000 đồng
```

Bốn phần nằm đúng chỗ của chúng:

- **Bày sẵn** — hai khối trên cùng: danh sách `cac_co_to` và hàm `gia_to`.
- **Hỏi** — hai cặp `print` rồi `input()`. Byte tách câu hỏi ra một dòng `print`
  riêng cho dễ đọc.
- **Tính** — `int()` đổi câu trả lời thành số, `gia_to()` tra bảng giá và đưa
  con số về.
- **Đáp** — khối `if` cuối cùng, nói ra một trong hai câu.

Để ý dòng `so_to = int(input())`. Thứ khách gõ vào luôn là **chữ**, kể cả khi
khách gõ toàn chữ số. Bỏ `int()` đi thì `so_to` vẫn là hai ký tự `2` — trông y
hệt một con số trên màn hình, nhưng đem nhân với giá tiền sẽ ra một thứ chẳng
giống hoá đơn nào cả.
::::

::::explain{#vi-sao-def-nam-tren}
Trong bốn phần trên, phần **bày sẵn** đứng đầu tiên. Đó không phải chuyện xếp
cho gọn mắt.

Máy đọc chương trình từ trên xuống, mỗi lần một dòng, và không nhìn trước xuống
dưới.

Dòng `def gia_to(co_to):` không chạy phần bên trong hàm. Nó làm đúng một việc:
dán cái tên `gia_to` lên nhóm lệnh nằm trong đó — y như dấu `=` dán một cái tên
lên một giá trị. Từ dòng ấy trở đi, và chỉ từ dòng ấy trở đi, `gia_to` mới là
một cái tên mà máy tìm ra được.

Byte để bạn tự kiểm chứng điều đó ngay bây giờ.
::::

::::predict{#doan-goi-truoc-khi-ghi commitOnce}
Đoạn dưới có đủ mọi thứ cần thiết, chỉ khác một chỗ: lời gọi hàm nằm **trên**
dòng `def`. **Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
print("PHỞ THÌN xin chào")
print(gia_to("vừa"))

def gia_to(co_to):
    if co_to == "vừa":
        return 45000
    else:
        return 0
```

:::opt{correct}
In ra `PHỞ THÌN xin chào`, rồi máy dừng lại và báo lỗi không tìm thấy tên `gia_to`
:::

:::opt
In ra `PHỞ THÌN xin chào` rồi `45000`
::why
Gần đúng ở chỗ bạn nhìn cả trang giấy một lượt và thấy hàm `gia_to` nằm ngay đó
— nó có tồn tại thật, bạn đọc không sai.

Chỗ lệch là ở chỗ đứng: bạn đọc cả trang cùng lúc, còn máy đọc lần lượt từng
dòng. Tới dòng thứ hai, máy đi tìm cái tên `gia_to`, mà lúc ấy nó chưa đọc tới
dòng `def` nên chưa dán cái tên đó lên việc nào cả. Tìm không ra một cái tên thì
máy báo `NameError` — đúng loại lỗi bạn gặp khi viết một cái tên chưa từng được
gán.

Đổi chỗ hai khối cho nhau là chạy được ngay.
::
:::

:::opt
Máy báo lỗi ngay, chưa kịp in ra dòng nào
::why
Gần đúng ở chỗ bạn đoán có lỗi — và có lỗi thật.

Chỗ lệch nằm ở **thời điểm**. Loại lỗi này chỉ lộ ra khi máy chạy tới đúng dòng
gây lỗi, mà dòng đó là dòng thứ hai. Dòng đầu đã chạy xong xuôi, nên câu chào
kịp hiện lên trước khi chương trình dừng.

Loại lỗi máy thấy được trước cả khi chạy dòng đầu tiên là loại khác: câu lệnh
viết sai ngữ pháp, thiếu ngoặc, thiếu nháy. Ở đây mọi dòng đều viết đúng ngữ
pháp.
::
:::

:::opt
Máy bỏ qua dòng gọi hàm rồi chạy tiếp cho hết chương trình
::why
Gần đúng ở chỗ bạn nghĩ máy sẽ không làm gì với thứ nó chưa biết — nó chưa biết
`gia_to` thật, phần đó bạn suy luận chính xác.

Chỗ lệch: máy không lặng lẽ bỏ qua. Nó dừng lại và nói ra. Đó lại là điều tốt
nhất nó có thể làm cho bạn — nếu nó im lặng đi tiếp, bạn sẽ nhận một hoá đơn
thiếu tiền mà không bao giờ biết vì sao.

Máy chờ lệnh và làm đúng lệnh; nó không tự sắp xếp lại chương trình giúp bạn.
::
:::
::::

::::code{#viet-nhanh-to}
Bắt đầu ghép, từ phần **bày sẵn**.

Hàm `gia_to` dưới đây đã có nhánh cho tô nhỏ và tô vừa. Nhánh cho tô to còn
thiếu dòng mở đầu. Hãy viết dòng đó.

```python title=starter
def gia_to(co_to):
    if co_to == "nhỏ":
        return 40000
    elif co_to == "vừa":
        return 45000
    ___
        return 55000
    else:
        return 0

print(gia_to("to"))
```

```python title=solution
def gia_to(co_to):
    if co_to == "nhỏ":
        return 40000
    elif co_to == "vừa":
        return 45000
    elif co_to == "to":
        return 55000
    else:
        return 0

print(gia_to("to"))
```

```python title=test
# Byte gọi hàm với cả ba cỡ tô, và thêm một cỡ không có trong thực đơn để xem
# nhánh cuối có đỡ được trường hợp lạ không.
assert gia_to("nhỏ") == 40000, "bảng giá quán ghi tô nhỏ 40 nghìn"
assert gia_to("vừa") == 45000, "bảng giá quán ghi tô vừa 45 nghìn"
assert gia_to("to") == 55000, "bảng giá quán ghi tô to 55 nghìn"
assert gia_to("khổng lồ") == 0, "quán không bán cỡ tô này, và cỡ nào không có trong bảng thì tra ra 0 đồng — đó là con số để cái máy biết mà xin lỗi khách thay vì báo bừa một giá"
```

:::hints
- kind: attention
  body: Ba cỡ tô, ba mức giá. Hai nhánh đầu đã viết sẵn — đọc kỹ dòng mở đầu của nhánh thứ hai xem nó gồm những phần nào.
- kind: strategy
  body: Nhánh còn thiếu là một khả năng nữa được kiểm tra sau khi hai khả năng trên đều sai, nên nó viết y hệt nhánh `vừa`, chỉ đổi câu chữ đem ra so sánh. Đừng bỏ dấu hai chấm ở cuối dòng.
- kind: one-line
  body: "Dòng còn thiếu là `elif co_to == \"to\":`"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: 55000
:::
::::

::::explain{#byte-dong-vai-khach}
Còn một chuyện thực tế phải nói rõ trước khi bạn ghép cả cái máy.

Ô chấm điểm của Byte chạy chương trình của bạn lúc không có ai ngồi trước bàn
phím. Gặp `input()`, chương trình sẽ dừng lại chờ một người không có mặt.

Nên ở bài chấm điểm này, Byte đóng vai khách: hai câu trả lời được ghi sẵn trong
một danh sách.

```python title=readonly
khach_noi = ["vừa", "2"]
```

`khach_noi[0]` là câu trả lời cho câu hỏi thứ nhất, `khach_noi[1]` cho câu hỏi
thứ hai — đúng cách bạn lấy món đầu tiên và món thứ hai trong thực đơn.

Chỗ đáng chú ý: cả hai câu đều nằm giữa hai dấu nháy, kể cả `"2"`. Byte cố ý giữ
nguyên như vậy, vì đó chính là thứ `input()` đưa về cho bạn — chữ, luôn luôn là
chữ. Nhờ vậy mọi dòng còn lại của cái máy chẳng cần biết mình đang nghe người
thật hay nghe hai câu ghi sẵn.
::::

::::assemble{#ghep-ca-may}
Đây là cả cái máy, còn hở ba chỗ, mỗi chỗ ở một phần khác nhau. Hãy điền vào để
nó chạy trọn vẹn từ bảng giá tới câu báo tiền.

```python title=starter
cac_co_to = ["nhỏ", "vừa", "to"]

def gia_to(co_to):
    if co_to == "nhỏ":
        return 40000
    elif co_to == "vừa":
        return 45000
    elif co_to == "to":
        return 55000
    else:
        return 0

# Hai câu Byte ghi sẵn, thay cho hai lần input(). Cả hai đều là chữ.
khach_noi = ["vừa", "2"]

print("PHỞ THÌN — bảng giá hôm nay")
for mot_co in ___:
    print(f"- tô {mot_co}: {gia_to(mot_co)} đồng")

co_khach_chon = khach_noi[0]
so_to = ___(khach_noi[1])
gia_mot_to = gia_to(co_khach_chon)

if gia_mot_to == 0:
    print("Quán mình chưa có cỡ tô này ạ")
else:
    print(f"{so_to} tô {co_khach_chon}, tổng cộng {___} đồng")
```

```python title=solution
cac_co_to = ["nhỏ", "vừa", "to"]

def gia_to(co_to):
    if co_to == "nhỏ":
        return 40000
    elif co_to == "vừa":
        return 45000
    elif co_to == "to":
        return 55000
    else:
        return 0

# Hai câu Byte ghi sẵn, thay cho hai lần input(). Cả hai đều là chữ.
khach_noi = ["vừa", "2"]

print("PHỞ THÌN — bảng giá hôm nay")
for mot_co in cac_co_to:
    print(f"- tô {mot_co}: {gia_to(mot_co)} đồng")

co_khach_chon = khach_noi[0]
so_to = int(khach_noi[1])
gia_mot_to = gia_to(co_khach_chon)

if gia_mot_to == 0:
    print("Quán mình chưa có cỡ tô này ạ")
else:
    print(f"{so_to} tô {co_khach_chon}, tổng cộng {gia_mot_to * so_to} đồng")
```

```python title=test
# Byte kiểm ba chỗ, mỗi chỗ ứng với một chỗ trống bạn vừa điền.
# Bảng giá đã in đủ ba cỡ hay chưa thì phần chấm theo màn hình lo.
assert gia_to("to") == 55000, "bảng giá bày sẵn từ đầu chương trình vẫn phải tra ra tô to 55 nghìn"
assert so_to == 2, "khách trả lời lấy hai tô, nên tới lúc tính tiền số tô phải là một con số đếm được bằng 2, không còn là câu chữ khách gõ vào"
assert gia_mot_to * so_to == 90000, "hai tô vừa, mỗi tô 45 nghìn, thì hoá đơn báo cho khách là 90 nghìn"
```

:::hints
- kind: attention
  body: Ba chỗ trống nằm ở ba phần khác nhau của cái máy. Chỗ thứ nhất trong phần bày bảng giá, chỗ thứ hai trong phần tính, chỗ thứ ba trong câu đáp cuối cùng.
- kind: strategy
  body: Chỗ thứ nhất cần cái tên đang giữ cả ba cỡ tô. Chỗ thứ hai cần việc đổi một câu chữ thành số thật. Chỗ thứ ba là giá một tô nhân với số tô, và hai giá trị đó đã nằm sẵn trong hai cái tên ngay phía trên.
- kind: one-line
  body: "Lần lượt ba chỗ trống là `cac_co_to`, `int`, và `gia_mot_to * so_to`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: tô to: 55000 đồng
- tier: output
  expect: 2 tô vừa, tổng cộng 90000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Máy của bạn vừa hỏi, vừa tính, vừa đáp. Hết Realm 0.
::::

::::reflect{#nghi-lai}
Nhìn lại một chút. Bạn vừa viết một chương trình biết hỏi, biết nghe, biết chọn
đường đi, và biết trả lời — bằng đúng những mảnh đã học, không cần thêm thứ gì
mới.

Nhưng nó có một chỗ hở, và chỗ hở ấy sẽ theo bạn sang chặng sau.

Cái máy chạy xong thì tắt. Bảng giá, câu khách nói, con số tiền vừa tính — tất
cả nằm trong bộ nhớ tạm, và tắt chương trình là mất sạch. Mở lại, nó không nhớ
hôm qua có ai vào quán, cũng không nhớ đã bán được bao nhiêu tô.

Ở module đầu tiên bạn đã gặp một thứ không mất khi tắt máy: file — cái hộp có
tên nằm lại trên đĩa. Nhưng suốt từ bài đầu tới giờ, chương trình của bạn chưa
một lần chạm vào cái hộp đó.

Câu hỏi mang sang chặng sau: làm sao để chương trình **ghi** sổ chi tiêu của hôm
nay xuống, rồi ngày mai mở lên **đọc lại** được?

Đừng trả lời vội. Đó là việc đầu tiên của Realm 1.
::::

::::checkpoint{mastery=0.85}
::::
