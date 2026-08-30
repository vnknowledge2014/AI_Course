---
id: onboarding.ra-lenh-cho-byte.nhieu-loi-re
title: Nhiều hơn hai lối rẽ
summary: Ba cỡ tô, ba mức giá. Một từ mới cho phép chương trình rẽ nhiều hơn hai đường.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ctrl.elif]
requires: [ctrl.if, ctrl.else]
concepts: [ctrl.re-nhanh, ctrl.chuoi-dieu-kien]
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
Hai nhánh vừa đủ cho câu hỏi có–không. Bảng giá quán phở đâu chỉ có hai dòng.
::::

::::explain{#ba-co-to}
Bài trước bạn đã có đủ hai lối: `if` lo phần câu trả lời là `True`, `else` lo
toàn bộ phần còn lại. Với một câu hỏi có–không thì hai lối là vừa khít — hoặc
có, hoặc không, hết.

Nhưng bảng giá treo trên tường quán phở không phải câu hỏi có–không:

> Tô nhỏ — 35000 đồng
>
> Tô vừa — 45000 đồng
>
> Tô lớn — 55000 đồng

Ba lối đi, không phải hai. Khách gọi "tô vừa" thì không rơi vào lối nào trong
hai lối `if` / `else` mà bạn đang có.

Người phục vụ ngoài đời xử lý chuyện này bằng cách **hỏi lần lượt**:

- "Tô nhỏ nhé?" — khách lắc đầu.
- "Vậy tô vừa nhé?" — khách gật.

Tới đây người phục vụ **ngừng hỏi**. Không ai hỏi tiếp "thế có phải tô lớn
không?" — hỏi thêm là vô nghĩa, vì khách đã gật rồi.

Hai chi tiết đó — hỏi lần lượt, và ngừng ngay khi có người gật — chính là thứ
Python gói vào một từ: `elif`. Đó là hai chữ `else` và `if` dính vào nhau, và
nghĩa của nó đúng là *"còn không thì, nếu..."*.
::::

::::example{#bang-gia-ba-dong}
Bảng giá ba dòng viết bằng Python:

```python title=readonly
co_to = "vua"

if co_to == "nho":
    print("35000 đồng")
elif co_to == "vua":
    print("45000 đồng")
else:
    print("55000 đồng")
```

Máy in ra đúng một dòng: `45000 đồng`.

Đọc lại theo đúng thứ tự máy đọc:

1. `co_to == "nho"` — cái tên đang giữ `"vua"`, mà `"vua"` không phải `"nho"`,
   nên câu trả lời là `False`. Máy bỏ qua khối thụt vào bên dưới.
2. Gặp `elif`. Vì nhánh phía trên vừa sai, máy mới chịu hỏi câu tiếp:
   `co_to == "vua"` — lần này `True`. Máy chạy khối bên dưới và in ra 45000.
3. Còn `else`? Máy không ngó tới nữa. Một nhánh đã đúng thì toàn bộ phần còn lại
   của chuỗi bị bỏ qua, y như người phục vụ ngừng hỏi khi khách đã gật.

Để ý chỗ đứng của ba dòng `if`, `elif`, `else`: cả ba đều **sát lề trái**, cùng
một mức. Chúng không lồng vào nhau — chúng là ba lối rẽ của cùng một ngã ba.
Chỉ những dòng việc bên dưới mỗi lối mới thụt vào bốn dấu cách.
::::

::::predict{#dung-o-nhanh-dau-tien commitOnce}
Đoạn dưới có chỗ đáng ngờ: với `tien` là 60000, **cả hai** điều kiện đều đúng
(60000 lớn hơn 40000, và cũng lớn hơn 55000).

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python
tien = 60000

if tien >= 40000:
    print("Mời bạn tô nhỏ")
elif tien >= 55000:
    print("Mời bạn tô lớn")
else:
    print("Chưa đủ tiền một tô")
```

:::opt{correct}
Chỉ một dòng: Mời bạn tô nhỏ
:::

:::opt
Cả hai dòng: Mời bạn tô nhỏ, rồi Mời bạn tô lớn
::why
Gần đúng ở chỗ bạn đã kiểm cả hai điều kiện và thấy cả hai đều đúng —
`60000 >= 40000` đúng, `60000 >= 55000` cũng đúng. Phần tính toán đó chính xác
hoàn toàn.

Chỗ lệch nằm ở chữ `elif`. Nó không mang nghĩa "hỏi thêm một câu nữa", mà mang
nghĩa "**còn không thì** mới hỏi tiếp". Máy chỉ đọc tới một `elif` khi mọi nhánh
phía trên đều đã sai. Ở đây nhánh đầu đã đúng, nên cả chuỗi dừng ngay tại đó —
dòng `elif` không được hỏi lần nào.
::
:::

:::opt
Mời bạn tô lớn
::why
Gần đúng ở chỗ bạn đang chọn nhánh **hợp lý nhất** cho 60000 đồng. Người bán
hàng ngoài đời cũng làm đúng thế: có nhiều tiền thì mời tô to.

Nhưng máy không so xem nhánh nào khớp hơn. Nó đọc từ trên xuống và dừng ở nhánh
đúng **đầu tiên** nó gặp. Nhánh 40000 nằm trên, nên nhánh đó thắng.

Muốn máy mời tô lớn, bạn phải đổi **thứ tự** hai nhánh — đưa điều kiện chặt hơn
(`>= 55000`) lên trước. Trong một chuỗi `if / elif`, thứ tự các nhánh là một
phần ý nghĩa của chương trình, không phải chuyện sắp xếp cho gọn mắt.
::
:::

:::opt
Máy báo lỗi vì hai điều kiện chồng lên nhau
::why
Gần đúng ở chỗ bạn nhận ra hai điều kiện chồng lấn nhau. Nhìn ra được chuyện đó
là một thói quen đọc code rất tốt, vì đây đúng là chỗ hay sinh ra kết quả ngoài
ý muốn.

Chỗ lệch: chồng lấn không phải lỗi ngữ pháp. Mỗi dòng vẫn đọc được đàng hoàng,
máy vẫn chạy trơn tru từ đầu tới cuối. Nó chỉ lặng lẽ chọn nhánh đầu tiên rồi
thôi.

Đây là loại sai khó chịu hơn cả `SyntaxError`: chương trình không kêu ca một
tiếng nào, chỉ cho ra kết quả không phải cái bạn muốn. Cách duy nhất để bắt được
nó là đọc lại thứ tự các nhánh bằng mắt mình.
::
:::
::::

::::explain{#luat-cua-chuoi}
Gom lại ba luật của một chuỗi rẽ nhánh:

- **Máy đi từ trên xuống, dừng ở nhánh đúng đầu tiên.** Đúng một nhánh được
  chạy, không bao giờ hai.
- **`elif` viết được bao nhiêu cái cũng được.** Quán thêm cỡ tô đặc biệt thì
  thêm một `elif` nữa vào giữa chuỗi.
- **`else` đứng cuối và không kèm điều kiện.** Nó là cái lưới hứng mọi trường
  hợp còn sót. `else` cũng có thể không có — lúc đó nếu mọi nhánh đều sai thì
  máy chẳng làm gì cả rồi đi tiếp.

> Dễ nhầm: `elif` **luôn phải có một điều kiện** phía sau và một dấu hai chấm
> cuối dòng, y như `if`. Viết `elif:` trơn là `SyntaxError`. Còn `else` thì
> ngược lại — nó không nhận điều kiện nào, chỉ có `else:`.
::::

::::code{#viet-bang-gia}
Bảng giá dưới đây thiếu đúng một từ. Khách đang gọi tô vừa, nên chương trình
phải in ra `45000 đồng`.

Hãy điền vào chỗ trống để nhánh giữa trở thành một lối rẽ của cùng ngã ba đó.

```python title=starter
co_to = "vua"

if co_to == "nho":
    print("35000 đồng")
___ co_to == "vua":
    print("45000 đồng")
else:
    print("55000 đồng")
```

```python title=solution
co_to = "vua"

if co_to == "nho":
    print("35000 đồng")
elif co_to == "vua":
    print("45000 đồng")
else:
    print("55000 đồng")
```

```python title=test
# Chấm bằng OUTPUT: với cỡ tô "vua", màn hình phải hiện đúng mức giá 45000.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm ở **đầu dòng**, sát lề trái, cùng mức với `if` và `else`. Đó là chỗ của một từ khoá, không phải chỗ của một điều kiện.
- kind: strategy
  body: Dòng này phải làm hai việc một lúc — vừa nói "nhánh trên sai rồi", vừa mang theo một câu hỏi mới, vì ngay sau chỗ trống đã có sẵn `co_to == "vua"`. Trong bài có đúng một từ làm được cả hai.
- kind: one-line
  body: Thay `___` bằng `elif`, giữ nguyên phần `co_to == "vua":` phía sau.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^45000 đồng\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bảng giá ba dòng vừa chạy được. Quán thêm cỡ tô thì bạn thêm một `elif`, thế thôi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Chuỗi `if / elif / else` hôm nay hỏi lần lượt, mỗi nhánh đúng **một** câu hỏi.

Sáng nay quán dán thêm một tờ giấy nữa:

> Học sinh đến trước 8 giờ: giảm 5000 đồng.

Đọc kỹ tờ giấy đó. Nó không phải ba lối rẽ, cũng không phải hai. Nó là **một**
lối rẽ duy nhất, nhưng muốn đi vào thì phải thoả **hai** điều cùng lúc: vừa là
học sinh, vừa đến trước 8 giờ. Bạn học sinh nhưng 9 giờ mới tới thì không giảm.
Bác đến từ 7 giờ nhưng không phải học sinh thì cũng không giảm.

Mà giữa chữ `if` và dấu hai chấm chỉ vừa chỗ cho đúng một câu hỏi có–không.

Vậy làm sao nhét hai câu hỏi vào một chỗ, lại còn bắt cả hai phải cùng đúng?

Đừng trả lời vội. Bài sau là đúng một từ ngắn để buộc hai câu hỏi lại với nhau.
::::

::::checkpoint{mastery=0.8}
::::
