---
id: onboarding.ra-lenh-cho-byte.dat-ten-cho-mot-viec
title: Đặt tên cho một việc
summary: Gói nhiều dòng lệnh lại dưới một cái tên, rồi gọi tên đó khi cần.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [core.function-def, core.function-call]
requires: [core.variable, ctrl.block-indent]
concepts: [core.ham]
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
Việc nào làm nhiều lần thì đặt cho nó một cái tên. Gọi tên là xong.
::::

::::explain{#chep-lai-ba-lan}
Ở quán phở, mỗi lần có khách vào, chủ quán không dặn nhân viên từng bước:
*"chần bánh, xếp thịt, chan nước, rắc hành"*. Ông chỉ nói: **"làm một tô tái"**.

Bốn việc, một cái tên. Ai trong quán cũng biết cái tên đó nghĩa là gì, nên nói
một câu là đủ.

Chương trình của bạn đang thiếu đúng chuyện đó. Đoạn chào khách gồm ba dòng
`print`, và bạn đã chép nguyên ba dòng ấy sang ba chỗ khác nhau. Sửa một chữ thì
phải nhớ sửa cả ba nơi — quên một nơi là chương trình nói hai kiểu khác nhau
trong cùng một buổi.

Python cho bạn cách đặt tên cho một nhóm lệnh, y như chủ quán đặt tên cho bốn
thao tác kia. Nhóm lệnh có tên đó gọi là một **hàm** (tiếng Anh: `function`).
::::

::::example{#hinh-dang-mot-ham}
Đây là một hàm. Đọc từ trên xuống:

```python title=readonly
def chao_khach():
    print("Quán Phở Thìn xin chào")
    print("Mời anh chị ngồi bàn trống")

chao_khach()
```

- `def` là lời báo với máy: *"tôi sắp đặt tên cho một việc"*.
- `chao_khach` là **cái tên** bạn đặt. Bạn tự chọn, giống như đặt tên cho một
  giá trị ở bài trước.
- Hai dấu ngoặc `()` và dấu hai chấm `:` khép lại dòng đầu.
- Hai dòng `print` **thụt vào** — đúng như trong `if`, thụt lề là cách máy biết
  dòng nào nằm *bên trong* cái tên này.
- Dòng cuối `chao_khach()` — không thụt vào, nên nó nằm **ngoài**. Đây là lúc
  bạn **gọi** cái tên đó ra dùng.

Hai việc hoàn toàn khác nhau đang xảy ra ở đây, và đây là chỗ dễ nhầm nhất:

- Từ `def` trở xuống chỉ **ghi lại** việc cần làm. Máy đọc, nhớ cái tên, rồi đi
  tiếp. Nó chưa in gì cả.
- Dòng `chao_khach()` mới là lúc máy **thật sự làm** hai việc đã ghi.

Giống tờ giấy công thức dán trên tường bếp: dán lên tường không làm ra tô phở
nào. Phải có người đọc nó và bắt tay vào làm.
::::

::::predict{#doan-thu-tu commitOnce}
Byte sắp chạy đoạn dưới. **Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
def chao_khach():
    print("Quán Phở Thìn xin chào")

print("Mở cửa")
```

:::opt{correct}
Chỉ một dòng: `Mở cửa`
:::

:::opt
Hai dòng: `Quán Phở Thìn xin chào` rồi `Mở cửa`
::why
Gần đúng ở chỗ bạn đọc code từ trên xuống dưới — đúng thứ tự máy đọc thật. Chỗ
lệch nằm ở chữ `def`: nó chỉ bảo máy **ghi nhớ** cái tên `chao_khach` cùng việc
kèm theo, chứ chưa bảo máy làm.

Trong cả đoạn này không có dòng nào gọi `chao_khach()`, nên câu chào ấy nằm im
mãi mãi. Công thức dán trên tường, không ai đọc.
::
:::

:::opt
Hai dòng: `Mở cửa` rồi `Quán Phở Thìn xin chào`
::why
Gần đúng ở chỗ bạn nhận ra `def` chưa chạy ngay — điều này chính xác. Nhưng bạn
đang chờ nó **tự chạy sau**, và máy không làm thế: nó không tự đoán lúc nào bạn
muốn dùng cái tên đó.

Máy chỉ chạy phần bên trong khi có một dòng gọi tên ra. Không gọi thì không chạy,
dù chờ đến bao giờ.
::
:::

:::opt
Máy báo lỗi vì `chao_khach` được đặt tên mà không dùng
::why
Gần đúng ở chỗ bạn cảm thấy có gì đó thừa — và đúng là thừa thật, viết một hàm
rồi không gọi thì vô ích. Nhưng "vô ích" khác "sai": máy không có quy tắc nào
buộc bạn phải dùng một cái tên đã đặt.

Máy chỉ báo lỗi khi nó **không hiểu** hoặc **không tìm thấy** thứ bạn nhắc tới.
Ở đây nó hiểu hết, và làm đúng từng chữ bạn viết.
::
:::
::::

::::explain{#vi-sao-tach-hai-buoc}
Việc tách làm hai bước — ghi lại một lần, gọi nhiều lần — chính là chỗ hàm trả
công cho bạn:

```python title=readonly
def chao_khach():
    print("Quán Phở Thìn xin chào")

chao_khach()
chao_khach()
chao_khach()
```

Ba khách vào, ba lần gọi, câu chào giống hệt nhau cả ba lần. Hôm nào quán đổi
tên, bạn sửa **đúng một dòng** bên trong hàm — cả ba chỗ gọi đổi theo, không sót
chỗ nào.

Đó là lý do thật sự người ta viết hàm. Không phải để code ngắn hơn, mà để mỗi
việc chỉ có **một chỗ duy nhất** chịu trách nhiệm về nó.
::::

::::code{#viet-ham-dau-tien}
Đoạn dưới đã ghi sẵn hàm `chao_khach`, nhưng chạy lên màn hình vẫn trắng trơn.
Hãy thêm dòng còn thiếu để câu chào thật sự hiện ra.

```python title=starter
def chao_khach():
    print("Quán Phở Thìn xin chào")

___
```

```python title=solution
def chao_khach():
    print("Quán Phở Thìn xin chào")

chao_khach()
```

```python title=test
# Byte bịt màn hình lại một lát, tự gọi hàm, rồi xem hàm có nói đúng câu không.
import io
import contextlib

ghi_lai = io.StringIO()
with contextlib.redirect_stdout(ghi_lai):
    chao_khach()

assert "Quán Phở Thìn xin chào" in ghi_lai.getvalue(), "việc chào khách khi được gọi ra phải nói đúng câu chào của quán — gọi mà màn hình vẫn trắng nghĩa là chưa có ai làm việc đã ghi"
```

:::hints
- kind: attention
  body: Hàm đã được ghi lại rồi. Thứ còn thiếu là câu bảo máy **làm** nó.
- kind: strategy
  body: Nhìn lại ví dụ đầu bài. Dòng gọi hàm viết sát lề trái, gồm đúng cái tên bạn đã đặt, kèm hai dấu ngoặc.
- kind: one-line
  body: "`chao_khach()` — nhớ hai dấu ngoặc, và đừng thụt vào."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: static
  onFail: hàm đã được ghi sẵn ở trên rồi — việc còn thiếu là GỌI nó ra, chứ không phải gõ lại câu chào bằng tay
  requireAst:
  # Không có luật này thì `print("Quán Phở Thìn xin chào")` gõ tay cũng in ra
  # đúng dòng ấy và đậu — ở đúng bài mà đặt tên cho một việc rồi gọi lại là
  # khái niệm mới duy nhất.
  - kind: uses-call, target: chao_khach, min: 1
- tier: output
  expect: Quán Phở Thìn xin chào
:::
::::

::::reflect{#nghi-lai}
Hàm `chao_khach` của bạn chạy tốt, nhưng nó có một tật: gặp ai nó cũng nói y
một câu.

Quán muốn chào đích danh — *"Chào chị Hoa"*, rồi khách sau là *"Chào anh Nam"*.
Câu chào giống nhau, chỉ mỗi cái tên đổi.

Bạn có thể viết hai hàm, `chao_chi_hoa` và `chao_anh_nam`. Nhưng khách thứ một
trăm thì sao?

Nghĩ thử: làm cách nào để **một** hàm nhận được cái tên khác nhau ở mỗi lần gọi?
::::

::::checkpoint{mastery=0.8}
::::
