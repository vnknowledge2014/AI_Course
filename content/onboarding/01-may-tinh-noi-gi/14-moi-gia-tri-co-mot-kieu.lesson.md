---
id: onboarding.may-tinh-noi-gi.moi-gia-tri-co-mot-kieu
title: Mỗi giá trị có một kiểu
summary: Máy dán sẵn một cái nhãn lên mọi giá trị — và bạn hỏi được cái nhãn đó.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 9
teaches: [core.type-of-value, core.type-fn]
requires: [core.string-literal]
concepts: [core.kieu-gia-tri]
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
Mình không nhìn giá trị bằng mắt. Mình đọc cái nhãn dán sẵn trên nó.
::::

::::explain{#cai-nhan-dan-san}
Bài trước bạn để hai thứ vào hai cái tên:

```python
ten_quan = "Phở Thìn"
gia_pho = 45000
```

Với mắt bạn, hai dòng này khác nhau rõ ràng: một bên là chữ, một bên là số.
Câu hỏi cuối bài trước là: máy có phân biệt được như bạn không?

Có. Và nó phân biệt dứt khoát hơn bạn nhiều.

Mỗi giá trị nằm trong máy đều mang sẵn một **kiểu** — một cái nhãn cho biết nó
thuộc loại nào. Cái nhãn đó không phải bạn dán thêm. Nó dính vào giá trị ngay
lúc bạn viết giá trị ra:

- Viết giữa hai dấu nháy → nhãn **`str`** (đọc là "ét-tơ-rờ", viết tắt của
  *string*: một chuỗi ký tự).
- Viết không nháy, toàn chữ số → nhãn **`int`** (đọc là "in-tơ", viết tắt của
  *integer*: số nguyên).

Cái nhãn này không để trang trí. Nó quyết định máy được phép làm gì với giá trị
đó — chuyện này sẽ rõ dần ở mấy bài tới.
::::

::::example{#hoi-thang-cai-nhan}
Có cách hỏi thẳng máy: `type`.

```python title=readonly
gia_pho = 45000
print(type(gia_pho))
```

Máy trả lời:

```text
<class 'int'>
```

Câu trả lời hơi khô. Đọc nó thế này: `class` ở đây nghĩa là "loại", và loại của
`45000` là `int` — số nguyên.

Chú ý chỗ dễ lẫn: `type` **không** đưa lại giá trị, nó đưa lại **loại** của giá
trị. Hai việc khác hẳn nhau:

- `print(gia_pho)` in ra `45000` — con số.
- `print(type(gia_pho))` in ra `<class 'int'>` — cái nhãn của con số.
::::

::::predict{#doan-nhan-cua-chu commitOnce}
Byte sắp chạy đoạn dưới. **Trước khi bấm chạy**, bạn đoán nó in ra gì?

```python
ten_quan = "Phở Thìn"
print(type(ten_quan))
```

:::opt{correct}
&lt;class 'str'&gt;
:::

:::opt
Phở Thìn
::why
Gần đúng ở chỗ bạn nhớ đúng bài trước: đặt một cái tên vào `print` thì máy in ra
giá trị mà tên đó đang giữ. Đúng vậy thật.

Chỗ lệch: ở đây `ten_quan` không đi thẳng vào `print`. Nó ghé qua `type` trước.
`type` nhận vào một giá trị và đưa ra **cái nhãn** của giá trị đó, chứ không đưa
lại chính giá trị. Cái đến tay `print` là cái nhãn.
::
:::

:::opt
str
::why
Gần đúng — gần nhất trong ba đáp án. Tên loại đúng là `str`, bạn nhận ra ngay
`"Phở Thìn"` là chữ vì nó nằm giữa hai dấu nháy.

Chỗ lệch chỉ là hình thức: máy in ra **nguyên văn** câu trả lời của nó, và câu
đó là `<class 'str'>` chứ không phải mỗi chữ `str`. Khi đọc, bạn cứ bỏ qua phần
`<class ...>` và nhìn vào cái tên nằm giữa hai dấu nháy đơn.
::
:::
::::

::::code{#hoi-nhan-cua-so}
Quán đếm được số tô bán trong ngày. Hãy hỏi máy xem giá trị đó thuộc **loại** gì.

```python title=starter
so_to_da_ban = 128
print(___)
```

```python title=solution
so_to_da_ban = 128
print(type(so_to_da_ban))
```

```python title=test
# Bài này chấm bằng OUTPUT: người học chưa biết hàm, nên chưa assert được gì.
# Khối này chỉ khẳng định chương trình chạy được tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Câu hỏi là "giá trị này thuộc loại nào", không phải "giá trị này bằng bao nhiêu". Hai câu hỏi khác nhau thì lệnh cũng khác nhau.
- kind: strategy
  body: Nhìn lại ví dụ ở trên. Ở đó có hai lớp ngoặc lồng nhau: lớp trong bọc cái tên, lớp ngoài là của `print`.
- kind: one-line
  body: Viết `type(so_to_da_ban)` vào chỗ trống.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: <class 'int'>
:::
::::

::::reflect{#nghi-lai}
Bạn vừa biết máy dán nhãn `int` cho số nguyên. Giờ thử nghĩ tiếp:

`45000` chia cho 2 được `22500` — vẫn nguyên vẹn, không dư. Chia cho 4 được
`11250` — cũng vẫn nguyên. Vậy máy có dán nhãn `int` cho kết quả không?

Đừng trả lời vội. Bài sau bạn sẽ chạy đúng phép chia đó và xem máy in ra gì.
::::

::::checkpoint{mastery=0.8}
::::
