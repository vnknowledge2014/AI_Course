---
id: onboarding.ra-lenh-cho-byte.neu-khong-thi
title: Còn không thì
summary: Khi câu trả lời là False, máy không đứng im nữa. else là lối đi thứ hai của cùng một ngã ba.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ctrl.else]
requires: [ctrl.if, ctrl.block-indent]
concepts: [ctrl.re-nhanh, core.khoi-lenh]
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
Bài trước, câu trả lời sai thì máy im lặng. Bài này nó có câu để nói.
::::

::::explain{#mot-nga-ba-chu-khong-phai-hai-viec}
Bài trước để lại một chỗ hở, và bạn đã nhìn thấy nó: lúc 20 giờ, `gio < 8` cho
ra `False`, máy nhảy qua cả khối lệnh rồi đi tiếp — không nói với khách một
tiếng nào về chuyện khuyến mãi.

Người bán hàng thật không im như thế. Người ta nói: *"Giờ này hết khuyến mãi
rồi, bác ạ."*

Tờ giấy dán tường trong bếp, viết cho đủ, phải là hai dòng dính vào nhau:

> Nếu còn thịt tái thì chần tái cho khách.
>
> Còn không thì báo khách hết tái, mời khách chín nạm.

Đây là một **ngã ba**, không phải hai việc rời nhau. Người phụ bếp hỏi đúng một
lần — *còn thịt tái không?* — rồi rẽ đúng một lối. Không bao giờ đi cả hai lối.
Cũng không bao giờ đứng im giữa ngã ba.

Python có một từ dành riêng cho lối thứ hai: **`else`** — tiếng Anh nghĩa là
"phần còn lại", "còn không thì".
::::

::::example{#else-dau-tien}
Đây là tờ giấy dán tường viết cho đủ hai lối:

```python title=readonly
gio = 20
if gio < 8:
    print("Giảm giá buổi sáng, bác ơi")
else:
    print("Giờ này hết khuyến mãi rồi, bác ạ")
```

Lúc 20 giờ, máy in ra:

```text title=readonly
Giờ này hết khuyến mãi rồi, bác ạ
```

Đọc dòng `else` từng chi tiết một:

- Nó viết **sát lề trái, ngang bằng** với `if` — không thụt vào. Nó là nửa còn
  lại của cùng một ngã ba, chứ không phải một dòng việc nằm bên trong `if`.
- Cuối dòng có **dấu hai chấm**, y như `if`, vì nó cũng mở ra một khối lệnh.
- Sau `else` **không có câu hỏi nào cả**. Câu hỏi đã hỏi ở dòng `if` rồi;
  `else` nhận đúng phần còn lại của câu hỏi ấy. Viết `else gio >= 8:` là máy
  không đọc nổi và báo `SyntaxError`.
- Dòng dưới `else` **thụt vào bốn dấu cách** — luật khối lệnh của bài trước, y
  nguyên, không có ngoại lệ nào.

Bây giờ đổi đúng một con số, `gio = 6`:

```python title=readonly
gio = 6
if gio < 8:
    print("Giảm giá buổi sáng, bác ơi")
else:
    print("Giờ này hết khuyến mãi rồi, bác ạ")
```

```text title=readonly
Giảm giá buổi sáng, bác ơi
```

`6 < 8` là `True`, nên máy chạy khối trên và **bỏ hẳn** khối `else`.

Đây là điều đáng nhớ nhất của cả bài: trong một lần chạy, **đúng một trong hai
khối được chạy**. Không bao giờ cả hai. Không bao giờ không khối nào.
::::

::::predict{#du-tien-hay-khong commitOnce}
Khách có 30000 đồng, tô phở giá 45000. **Trước khi bấm chạy**, bạn đoán màn hình
hiện ra những dòng nào, theo thứ tự nào?

```python title=readonly
tien = 30000
if tien >= 45000:
    print("Mời bác vào ăn phở")
else:
    print("Còn thiếu tiền, bác ạ")
print("Cảm ơn bác")
```

:::opt{correct}
Hai dòng: Còn thiếu tiền, bác ạ — rồi Cảm ơn bác
:::

:::opt
Cả ba dòng, theo đúng thứ tự viết trong file
::why
Gần đúng ở chỗ bạn giữ một thói quen đọc rất chắc: máy chạy từ trên xuống, dòng
nào cũng tới lượt. Với mọi bài trước `if`, điều đó luôn đúng.

Chỗ lệch: `if` và `else` là hai lối của **cùng một** ngã ba. Máy hỏi một lần,
rẽ một lối, và lối kia không hề chạy trong lần chạy này. Ở đây `30000 >= 45000`
cho `False`, nên dòng "Mời bác vào ăn phở" bị bỏ qua trọn vẹn.

Còn dòng `print("Cảm ơn bác")` thì bạn xếp đúng — nó nằm ngoài ngã ba nên vẫn
chạy.
::
:::

:::opt
Chỉ một dòng: Còn thiếu tiền, bác ạ
::why
Gần đúng — và gần rất sát. Bạn chọn đúng lối rẽ: `30000 >= 45000` là `False`,
nên máy đi vào khối `else`. Phần khó nhất của bài này bạn đã làm đúng.

Chỗ lệch nằm ở dòng cuối. `print("Cảm ơn bác")` viết **sát lề trái**, không thụt
vào — nên theo luật thụt đầu dòng của bài trước, nó không thuộc khối nào cả. Nó
là một dòng bình thường của chương trình, chạy sau khi ngã ba đã đi xong.

Câu cảm ơn dành cho mọi khách, đủ tiền hay không.
::
:::

:::opt
Hai dòng: Mời bác vào ăn phở — rồi Cảm ơn bác
::why
Gần đúng ở chỗ bạn đã nắm được luật lớn nhất của bài: chỉ **một** trong hai khối
được chạy, và dòng sát lề trái thì chạy trong mọi trường hợp. Cấu trúc bạn đọc
hoàn toàn chính xác.

Chỗ lệch nằm ở chiều của phép so sánh. Vế **bên trái** là chủ ngữ của câu hỏi:
*"30000 có lớn hơn hoặc bằng 45000 không?"* Ba mươi nghìn không mua nổi tô phở
bốn mươi lăm nghìn, nên câu trả lời là `False`, và máy rẽ vào lối `else`.

Nếu khách cầm 50000 thì đáp án của bạn mới là đáp án đúng.
::
:::
::::

::::explain{#sao-khong-viet-hai-cau-if}
Có thể bạn đang nghĩ: viết hai câu `if` là xong, cần gì tới `else`?

```python title=readonly
if tien >= 45000:
    print("Mời bác vào ăn phở")
if tien < 45000:
    print("Còn thiếu tiền, bác ạ")
```

Đoạn này chạy ra đúng kết quả. Nó không sai. Nhưng nó có hai chỗ yếu, và cả hai
đều là chỗ yếu **về sau**, chứ không phải hôm nay:

- Máy phải hỏi **hai lần** thay vì một lần, dù câu hỏi thứ hai đã có sẵn đáp án
  từ câu hỏi thứ nhất.
- Ngày mai giá phở lên 50000. Bạn sửa con số ở dòng 1, và **phải nhớ** sửa cả
  dòng 3. Quên một chỗ thì có lúc chương trình chẳng nói gì, có lúc nói cả hai
  câu ngược nhau — mà nó vẫn chạy ngon lành, không báo lỗi gì cho bạn biết.

`else` không chứa con số nào để mà quên. Nó luôn là "phần còn lại" của câu hỏi
nằm ngay trên nó: bạn sửa câu hỏi, nó tự đúng theo.

Đó là lý do `else` tồn tại — không phải để gõ ít chữ hơn, mà để hai lối rẽ
không bao giờ rời nhau ra được.
::::

::::code{#het-pho-roi}
Quán hết phở. `so_to_con` đang giữ số `0`.

Đoạn dưới mới có một lối rẽ. Hãy điền vào chỗ trống để mở ra lối còn lại, sao
cho khi hết phở máy nói: `Hôm nay hết phở rồi, bác ạ`

```python title=starter
so_to_con = 0
if so_to_con > 0:
    print("Mời bác vào, còn phở")
___
    print("Hôm nay hết phở rồi, bác ạ")
```

```python title=solution
so_to_con = 0
if so_to_con > 0:
    print("Mời bác vào, còn phở")
else:
    print("Hôm nay hết phở rồi, bác ạ")
```

```python title=test
# Chấm bằng OUTPUT: hết phở thì chỉ được hiện đúng câu báo hết phở.
pass
```

:::hints
- kind: attention
  body: Nhìn chỗ đứng của chỗ trống: nó sát lề trái, ngang hàng với `if`. Còn dòng ngay dưới nó thì thụt vào — nghĩa là chỗ trống này đang mở ra một khối lệnh mới.
- kind: strategy
  body: Chỗ này là lối rẽ "còn không thì". Nó không cần câu hỏi nào — điều kiện của nó chính là phần còn lại của câu hỏi ở dòng `if`. Vậy nó chỉ gồm một từ khoá, cộng thêm cái dấu mà mọi dòng mở khối lệnh đều phải có ở cuối.
- kind: one-line
  body: "Viết `else:` vào chỗ trống — sát lề trái, và có dấu hai chấm ở cuối dòng."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Hôm nay hết phở rồi, bác ạ\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Từ giờ máy không đứng im khi câu trả lời là `False`. Nó có lối rẽ thứ hai.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ngã ba của bạn có đúng hai lối, và mỗi lần chạy máy đi đúng một lối. Với câu hỏi
*"đủ tiền hay không"* thì hai lối là vừa đủ, vì câu hỏi ấy chỉ có hai câu trả
lời.

Nhưng ra quán mà xem. Quán có ba cỡ tô: tô nhỏ 40000, tô thường 45000, tô đặc
biệt 60000. Khách nói cỡ nào, máy phải báo đúng giá cỡ đó — **ba** câu trả lời
khác nhau, chứ không phải hai.

`if` lo một lối, `else` lo tất cả phần còn lại. Hai lối có đủ chỗ cho ba cỡ tô
không? Nếu bạn thử nhét thêm một `if` nữa vào **bên trong** khối `else`, rồi lại
một `else` nữa bên trong đó — chương trình sẽ thụt vào sâu dần thành hình bậc
thang. Có cách nào phẳng hơn không?

Đừng trả lời vội. Bài sau là một từ khoá đứng chen vào giữa `if` và `else`, sinh
ra đúng để giải chuyện này.
::::

::::checkpoint{mastery=0.8}
::::
