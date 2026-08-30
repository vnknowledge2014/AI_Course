---
id: onboarding.ra-lenh-cho-byte.may-hoi-lai-ban
title: Máy hỏi lại bạn
summary: Chương trình đầu tiên biết dừng lại giữa chừng và chờ bạn gõ câu trả lời.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [core.input]
requires: [core.output, core.variable]
concepts: [core.nhap-lieu]
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
Từ đầu khoá tới giờ mình chỉ nói. Bài này mình hỏi — rồi ngồi chờ bạn.
::::

::::explain{#loa-phat-thanh-va-quan-pho}
Mọi chương trình bạn viết cho tới lúc này đều giống cái loa phát thanh đầu ngõ:
nó nói xong một lượt rồi thôi. Bạn có đứng nghe hay không, nó vẫn nói đúng ngần
ấy câu, đúng thứ tự ấy, mọi lần chạy.

Quán phở thì khác. Bác chủ quán hỏi *"Anh ăn tô mấy?"* rồi **dừng tay**, chờ.
Bác không tự điền hộ câu trả lời, cũng không bưng đại một tô cho xong. Câu
chuyện chỉ đi tiếp khi bạn nói.

Python có đúng một câu lệnh làm cái dừng-và-chờ ấy. Nó tên là `input` — tiếng
Anh nghĩa là "nhập vào". Thuật ngữ bạn sẽ gặp về sau khi tra cứu: **nhập liệu**,
tức là đưa dữ liệu từ ngoài vào trong chương trình.
::::

::::example{#lan-hoi-dau-tien}
Đây là một chương trình biết hỏi:

```python title=readonly
ten = input("Bạn tên gì? ")
print("Chào " + ten)
```

Chạy nó, màn hình hiện ra một dòng rồi **đứng im**:

```text
Bạn tên gì? ▮
```

Cái vạch nhấp nháy là chỗ chờ bạn gõ. Lúc này dòng `print` chưa chạy — máy còn
đang đứng ở dòng trên. Bạn gõ `Lan`, bấm Enter, và màn hình thành:

```text
Bạn tên gì? Lan
Chào Lan
```

Đọc dòng `input` từ trái sang phải:

- `input` là **tên việc cần làm** — "hỏi rồi chờ".
- `"Bạn tên gì? "` nằm giữa hai dấu nháy, nên nó là chữ. Máy in nguyên văn phần
  này ra làm **lời nhắc**, để người ngồi trước máy biết mình phải gõ gì.
- Sau khi bạn bấm Enter, cả cụm `input("Bạn tên gì? ")` **biến thành** đúng chữ
  bạn vừa gõ. Dấu `=` dán chữ ấy lên cái tên `ten`, y như mọi lần bạn đặt tên
  cho một giá trị.

Để ý dấu cách nằm sau dấu hỏi, bên trong hai dấu nháy: `"Bạn tên gì? "`. Nhờ nó,
chỗ bạn gõ không dính sát vào dấu hỏi. Chi tiết nhỏ, nhưng người dùng thấy dễ
chịu hơn hẳn.
::::

::::predict{#doan-dong-cuoi commitOnce}
Byte sắp chạy đoạn dưới. Người ngồi trước máy sẽ gõ `phở tái` rồi bấm Enter.
**Trước khi bấm chạy**, bạn đoán dòng cuối cùng máy in ra là gì?

```python title=readonly
mon = input("Bạn gọi món gì? ")
print("Nhà bếp ơi, một " + mon)
```

:::opt{correct}
Nhà bếp ơi, một phở tái
:::

:::opt
Nhà bếp ơi, một mon
::why
Gần đúng ở chỗ bạn để ý `mon` trong `print` **không có dấu nháy** — bạn đang
soi dấu nháy rất kỹ, đúng thói quen mà mấy bài trước xây cho bạn.

Chỗ lệch nằm ở bước tiếp theo. Không nháy nghĩa là máy đi tìm một cái tên. Lần
này nó tìm **thấy**: `mon` được tạo ra ở dòng trên. Và khi tìm thấy, máy lấy
**thứ cái tên đang giữ** chứ không in ba chữ cái `m`, `o`, `n`. Thứ mà `mon`
đang giữ chính là chữ người ta vừa gõ.
::
:::

:::opt
Nhà bếp ơi, một Bạn gọi món gì?
::why
Gần đúng ở chỗ bạn nhận ra `"Bạn gọi món gì? "` là một câu chữ nằm giữa hai dấu
nháy — y hệt mọi chuỗi từ đầu khoá tới giờ — nên nó phải đi đâu đó. Đúng vậy
thật: nó được in ra làm lời nhắc.

Chỗ lệch là ở chỗ *đi đâu*. Câu trong ngoặc là thứ máy **hỏi**; còn thứ `input`
**đưa lại** cho bạn là câu người ta **trả lời**. Hai thứ khác nhau, và chỉ có
câu trả lời mới được dán lên cái tên `mon`.
::
:::

:::opt
Nhà bếp ơi, một — máy in ngay tức thì, chẳng chờ ai gõ gì
::why
Gần đúng ở chỗ bạn nhớ đúng một luật đã dùng suốt mười tám bài: máy chạy lần
lượt từ trên xuống, nhanh tới mức mắt không kịp thấy. Cho tới hôm nay, luật ấy
luôn đúng.

`input` là câu lệnh đầu tiên phá lệ. Tới dòng có `input`, máy **dừng hẳn** —
không phải dừng một giây mà dừng cho tới khi có người bấm Enter. Dòng `print`
nằm dưới nên nó chỉ chạy sau đó.
::
:::
::::

::::explain{#may-cho-toi-bao-gio}
Máy chờ bao lâu? Chờ mãi.

Bài về "khi không ai bảo, máy làm gì" đã nói: máy không đoán, không tự bổ sung ý
bạn quên nói. `input` là chỗ điều đó lộ ra rõ nhất — nó đứng đó tới khi có người
gõ, dù là mười giây hay mười phút.

Hai chi tiết nữa, để bạn khỏi bối rối lúc tự thử:

- **Lời nhắc là tuỳ chọn.** Viết `input()` với hai ngoặc rỗng vẫn chạy được.
  Nhưng lúc ấy màn hình đứng im không nói gì, và người dùng ngồi nhìn cái vạch
  nhấp nháy mà không biết máy đang đợi gì. Đợi trong im lặng là một cách làm
  người dùng bực mình.
- **Chỗ gõ ấy chính là terminal** — nơi bạn gõ một dòng, máy trả lời một dòng,
  luân phiên. Bây giờ tới lượt máy hỏi và bạn trả lời.

Có một hệ quả đáng để dừng lại một nhịp: từ bài này, kết quả chương trình
**không còn nằm sẵn trong code** nữa. Cùng một đoạn code, người gõ *Lan* thì máy
chào Lan, người gõ *Hùng* thì máy chào Hùng. Bạn viết cái khuôn; người dùng đổ
nội dung vào.
::::

::::code{#chao-khach-vao-quan}
Đến lượt bạn. Ở máy của bạn, dòng đầu tiên của bài này sẽ là:

```python
ten_khach = input("Bạn tên gì? ")
```

Trang luyện tập chưa nối được vào bàn phím, nên Byte gõ hộ: dòng đầu đã ghi sẵn
đúng thứ mà `input` đưa về khi người ta gõ *Lan*. Phần việc còn lại không đổi
chút nào — dùng cái tên `ten_khach` như một giá trị bình thường.

Hãy làm máy in ra đúng câu: `Chào Lan, mời vào quán`.

```python title=starter
# Byte gõ hộ bàn phím: coi như người ta vừa gõ Lan rồi bấm Enter.
ten_khach = "Lan"

print("Chào " + ___ + ", mời vào quán")
```

```python title=solution
# Byte gõ hộ bàn phím: coi như người ta vừa gõ Lan rồi bấm Enter.
ten_khach = "Lan"

print("Chào " + ten_khach + ", mời vào quán")
```

```python title=test
# Chấm bằng OUTPUT: người học chưa biết hàm nên chưa assert được gì.
# Khối này chỉ khẳng định chương trình chạy được tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm giữa hai dấu `+`. Chỗ ấy cần **thứ người ta vừa gõ**, không phải một câu chữ mới do bạn nghĩ ra.
- kind: strategy
  body: Thứ ấy đang nằm trong một cái tên ở dòng trên. Đặt một cái tên vào `print` mà không có dấu nháy thì máy đưa ra giá trị nó đang giữ.
- kind: one-line
  body: "Viết `ten_khach` vào chỗ trống — không dấu nháy, và giữ nguyên hai dấu `+` hai bên."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Chào Lan, mời vào quán
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Câu vừa rồi đổi theo người ngồi trước máy. Chương trình của bạn hết thuộc lòng rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Quán muốn hỏi tuổi khách, để biết có tặng trà cho người cao tuổi không:

```python
tuoi = input("Bác bao nhiêu tuổi? ")
```

Người ta gõ `25` rồi bấm Enter.

Trong `tuoi` bây giờ là **con số** 25 — thứ mà máy cộng, trừ, so sánh được? Hay
là hai ký tự `2` và `5` đứng cạnh nhau, y như `"Phở"` là ba ký tự đứng cạnh
nhau?

Đừng trả lời vội. Bài sau bạn hỏi thẳng máy bằng `type`, và nó nói ra ngay
trong một dòng.
::::

::::checkpoint{mastery=0.8}
::::
