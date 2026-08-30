---
id: onboarding.ra-lenh-cho-byte.gui-thong-tin-vao-ham
title: Gửi thông tin vào hàm
summary: Chừa một chỗ trống trong hàm, rồi điền vào chỗ đó mỗi lần gọi.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.function-parameter, core.function-argument]
requires: [core.function-def, core.fstring]
concepts: [core.tham-so]
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
Chừa một chỗ trống trong hàm. Mỗi lần gọi, bạn điền vào chỗ đó.
::::

::::explain{#cho-trong-trong-lenh}
Chủ quán không hô *"làm một tô tái"* rồi thôi. Ông hô: **"làm một tô tái cho
bàn ba"**.

Phần *"làm một tô tái"* lần nào cũng giống nhau. Phần *"bàn ba"* thì mỗi lần một
khác — bàn năm, bàn bảy, bàn ngoài hiên. Cái tên việc có sẵn một **chỗ trống**,
và người hô điền vào chỗ đó lúc hô.

Hàm của bạn cũng chừa được chỗ trống như vậy. Bạn đặt cho chỗ trống ấy một cái
tên, và lúc gọi hàm, bạn đưa vào một giá trị để điền.

Cái tên đặt cho chỗ trống gọi là **tham số**.
::::

::::example{#ham-co-cho-trong}
Đây là hàm chào khách ở bài trước, nay có một chỗ trống mang tên `ten_khach`:

```python title=readonly
def chao_khach(ten_khach):
    print(f"Chào {ten_khach}, mời vào quán Phở Thìn")

chao_khach("chị Hoa")
```

Đọc kỹ ba chỗ dùng đến `ten_khach`:

- `def chao_khach(ten_khach):` — giữa hai dấu ngoặc lúc **ghi lại** hàm, bạn
  khai chỗ trống và đặt tên cho nó.
- `{ten_khach}` bên trong câu chữ — chỗ trống được dùng, y như một cái tên giữ
  giá trị mà bạn đã quen.
- `chao_khach("chị Hoa")` — giữa hai dấu ngoặc lúc **gọi**, bạn đưa vào giá trị
  thật để điền.

Điểm đáng chú ý: nhìn khắp chương trình không thấy dòng nào viết
`ten_khach = "chị Hoa"`. Vậy mà `ten_khach` vẫn có giá trị.

Chính lời gọi làm việc dán tên đó. Lúc bạn viết `chao_khach("chị Hoa")`, máy
dán cái tên `ten_khach` lên chuỗi `"chị Hoa"`, rồi mới chạy phần bên trong hàm.

Cái tên `ten_khach` chỉ sống bên trong hàm, và chỉ sống trong đúng lần gọi đó.
Gọi xong, nó biến mất.
::::

::::predict{#doan-hai-lan-goi commitOnce}
Byte sắp chạy đoạn dưới. **Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
def chao_khach(ten_khach):
    print(f"Chào {ten_khach}, mời vào quán")

chao_khach("chị Hoa")
chao_khach("anh Nam")
```

:::opt{correct}
`Chào chị Hoa, mời vào quán` rồi `Chào anh Nam, mời vào quán`
:::

:::opt
`Chào anh Nam, mời vào quán` hai lần
::why
Gần đúng ở chỗ bạn nhớ điều đã học thật: dán một cái tên lên giá trị mới thì giá
trị cũ không còn ai gọi được nữa. Với biến bình thường, `"anh Nam"` đúng là sẽ
đè lên `"chị Hoa"`.

Chỗ lệch: hai lời gọi này **không xảy ra cùng lúc**. Máy chạy trọn vẹn lần gọi
thứ nhất — dán tên, in câu chào, rồi bỏ luôn cái tên `ten_khach` đi. Xong xuôi
mới tới lần gọi thứ hai, và lần đó dán lại từ đầu.

Máy in ngay lúc chào, chứ không để dành cả hai câu đến cuối chương trình mới in.
::
:::

:::opt
`Chào ten_khach, mời vào quán` hai lần
::why
Gần đúng ở chỗ bạn đang cảnh giác với một điều rất đáng cảnh giác: chữ nằm trong
dấu nháy thì máy đọc nguyên văn, không suy diễn.

Nhưng cặp ngoặc nhọn `{ }` chính là chỗ ngoại lệ mà bạn đã học: nó bảo máy
*"chỗ này đừng đọc nguyên văn — đi tìm cái tên bên trong và thay bằng giá trị
của nó"*. Còn phần `Chào ` và `, mời vào quán` nằm ngoài ngoặc nhọn thì đúng là
đọc nguyên văn thật.
::
:::

:::opt
Máy báo lỗi vì `ten_khach` chưa được gán giá trị nào
::why
Gần đúng ở chỗ bạn tìm dòng gán giá trị — và tìm không ra thật, trong cả đoạn
không có dấu `=` nào cả. Một cái tên chưa từng được gán thì máy quả thật sẽ báo
lỗi.

Chỗ lệch: lời gọi `chao_khach("chị Hoa")` **chính là** việc gán đó. Đưa một giá
trị vào giữa hai dấu ngoặc lúc gọi cũng là dán tên, chỉ khác là dán bằng vị trí
chứ không bằng dấu `=`.
::
:::
::::

::::explain{#hai-cai-ten-hai-vai}
Hai từ dễ lẫn, nên tách rõ một lần cho xong:

- `ten_khach` trong dòng `def` là **tham số** — chỗ trống, chưa có gì trong đó.
  Nó chỉ là cái tên bạn hứa sẽ điền.
- `"chị Hoa"` trong lời gọi là **giá trị thật** được đưa vào để điền.

Chỗ trống là cái khuôn, giá trị là thứ đổ vào khuôn. Cùng một khuôn, đổ vào thứ
gì thì ra thứ ấy.

Và vì tham số cũng chỉ là một cái tên giữ giá trị, mọi thứ bạn làm được với biến
đều làm được với nó: đem in ra, đem so sánh trong `if`, đem tính toán.

```python title=readonly
def bao_gia(so_to):
    print(f"{so_to} tô, tổng cộng {so_to * 45000} đồng")

bao_gia(3)
```

Ở đây chỗ trống nhận vào một con số, và hàm đem con số ấy nhân với giá một tô.
::::

::::code{#khai-cho-trong}
Đoạn dưới gọi hàm với tên khách đàng hoàng, nhưng dòng `def` còn thiếu chỗ
trống nên máy không biết cất `"chị Hoa"` vào đâu. Hãy khai chỗ trống đó.

```python title=starter
def chao_khach(___):
    print(f"Chào {ten_khach}, mời vào quán Phở Thìn")

chao_khach("chị Hoa")
```

```python title=solution
def chao_khach(ten_khach):
    print(f"Chào {ten_khach}, mời vào quán Phở Thìn")

chao_khach("chị Hoa")
```

```python title=test
# Byte bịt màn hình lại, gọi hàm với một cái tên khác hẳn, rồi xem hàm có chào
# đúng người vừa đưa vào không — chứ không phải chào cứng một người.
import io
import contextlib

ghi_lai = io.StringIO()
with contextlib.redirect_stdout(ghi_lai):
    chao_khach("bác Tư")

assert "Chào bác Tư" in ghi_lai.getvalue(), "người bước vào quán lần này là bác Tư, nên câu chào phải gọi đúng bác Tư — chào cứng một cái tên khác nghĩa là hàm không nghe ai đưa vào cả"
```

:::hints
- kind: attention
  body: Dòng `print` đang nhắc tới một cái tên. Máy phải nhận được cái tên đó ở đâu đó thì mới tìm ra.
- kind: strategy
  body: Chỗ trống khai giữa hai dấu ngoặc của dòng `def`, và phải trùng từng chữ với cái tên trong ngoặc nhọn.
- kind: one-line
  body: "Viết `ten_khach` vào chỗ trống — không có dấu nháy, vì đây là một cái tên chứ không phải câu chữ."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Chào chị Hoa, mời vào quán Phở Thìn
:::
::::

::::reflect{#nghi-lai}
Hàm `bao_gia` ở trên tính đúng tiền, và in ra màn hình rất rõ ràng.

Nhưng giờ quán cần cộng tiền ba bàn lại thành một hoá đơn. Bạn gọi
`bao_gia(3)`, con số hiện lên màn hình — mắt bạn đọc được, còn **chương trình**
thì không cầm được con số ấy để cộng tiếp.

Thử viết `tong = bao_gia(3)` xem. Bạn nghĩ `tong` sẽ giữ thứ gì?
::::

::::checkpoint{mastery=0.8}
::::
