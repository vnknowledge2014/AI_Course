---
id: onboarding.may-tinh-noi-gi.so-khong-can-nhay
title: Số thì máy tính được
summary: Bỏ hai dấu nháy đi, con số vẫn là con số — và máy làm được phép tính với nó.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [core.number-literal, core.arithmetic]
requires: [core.string-literal, core.name-lookup]
concepts: [core.so, core.phep-tinh]
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
Bài trước có chữ trong nháy và tên không nháy. Còn một loại thứ ba nữa.
::::

::::explain{#thu-thu-ba}
Bài trước bạn học được một luật để đọc mọi thứ nằm giữa hai dấu ngoặc của
`print`:

- Có dấu nháy → đó là **chữ**. Máy đọc nguyên văn, không cố hiểu.
- Không có dấu nháy → đó là một **tên**. Máy đi tìm xem cái tên đó đang chỉ vào
  thứ gì. Tìm không ra thì nó dừng lại và báo `NameError`.

Câu hỏi bỏ ngỏ cuối bài trước là: `print(2 + 3)`. Số `2` không có nháy. Vậy máy
có đi tìm một cái tên gọi là `2` không?

Không. Vì có một luật đặt tên rất chặt: **một cái tên phải mở đầu bằng chữ cái**
(hoặc dấu gạch dưới). Thứ gì mở đầu bằng chữ số thì không thể là tên — nên máy
không mất công đi tìm. Nó nhận ra ngay: đây là một **con số**.

Và con số thì khác hẳn chữ ở một chỗ: máy **tính toán** được với nó.
::::

::::explain{#bang-gia-va-tien-that}
Hãy nghĩ tới bảng giá dán trên tường quán phở, dòng chữ *45.000đ*.

Với tờ giấy dán tường, đó là mấy nét mực. Tờ giấy không cộng được gì cả — nó chỉ
mang hình dạng của con số.

Với chị chủ quán lúc ngồi đếm tiền cuối ngày, cũng con số ấy lại là một **số
tiền**: cộng được, trừ được, nhân với số tô bán ra được.

Máy tính phân biệt đúng hai chuyện đó, và nó phân biệt bằng dấu nháy:

- `"45000"` — có nháy — là nét mực. Máy đọc nguyên văn.
- `45000` — không nháy — là số tiền. Máy tính được.
::::

::::example{#bo-hai-dau-nhay}
Đây là hai dòng gần giống hệt nhau. Khác đúng hai dấu nháy.

```python title=readonly
print("2 + 3")
print(2 + 3)
```

Máy in ra:

```text title=readonly
2 + 3
5
```

Dòng đầu bạn đã gặp ở bài số 1: `2 + 3` nằm trong nháy nên nó là năm ký tự xếp
liền nhau — hai dấu cách cũng tính — và máy đọc nguyên văn cả năm.

Dòng thứ hai không có dấu nháy nào. Nên với máy, ở đây có hai con số và một dấu
`+` thật. Máy làm hai việc, theo đúng thứ tự này:

1. **Tính trước.** Cộng `2` với `3`, được `5`.
2. **Nói sau.** Đưa `5` cho `print`.

Ngoài dấu `+` (cộng), máy còn hiểu `-` (trừ) và `*` (nhân — dấu sao, không phải
chữ x). Dấu chia có một chuyện riêng khá bất ngờ, vài bài nữa mới tới.
::::

::::predict{#doan-tien-hai-mon commitOnce}
Khách gọi một tô phở 20.000đ cho trẻ con và một tô 25.000đ. Byte sắp chạy dòng
dưới. **Trước khi bấm chạy**, bạn đoán nó in ra gì?

```python title=readonly
print(20000 + 25000)
```

:::opt{correct}
45000
:::

:::opt
20000 + 25000
::why
Gần đúng ở chỗ bạn nhớ rất chắc bài số 1: `print("2 + 3")` in ra nguyên văn
`2 + 3`, không tính toán gì cả. Điều đó đúng — nhưng lúc đó phép cộng nằm **trong
dấu nháy**.

Ở dòng này không có dấu nháy nào. Nên hai chữ số không còn là nét mực nữa; chúng
là hai con số, và dấu `+` đứng giữa hai con số là một phép cộng thật. Máy cộng
xong rồi mới nói.
::
:::

:::opt
Máy báo `NameError`, vì `20000` không có nháy mà cũng chẳng phải cái tên nào bạn đặt
::why
Gần đúng, và bạn đang dùng đúng luật vừa học ở bài trước: gặp thứ không nháy,
máy đi tìm một cái tên; tìm không ra thì `NameError`.

Chỗ lệch nằm ở chữ "đi tìm". Máy chỉ đi tìm khi thứ nó thấy **có thể** là một
tên, mà tên thì phải mở đầu bằng chữ cái. `20000` mở đầu bằng chữ số, nên máy
biết ngay đây là số và không đi tìm ai cả.
::
:::

:::opt
2000025000
::why
Gần đúng ở một chỗ đáng giá: bạn đang nghi ngờ dấu `+` có thể làm việc gì đó
khác chứ không chỉ cộng. Giữ lấy mối nghi ấy — bài sau sẽ cho thấy bạn nghi
không sai.

Nhưng ở dòng này, cả hai bên dấu `+` đều là số viết không nháy. Đứng giữa hai
con số, dấu `+` chỉ có đúng một nghĩa: cộng.
::
:::
::::

::::explain{#gia-tri-may-tu-lam-ra}
Có một chuyện mới xảy ra ở đây, đáng dừng lại một nhịp.

Con số `45000` **không nằm ở đâu trong chương trình của bạn cả**. Bạn viết
`20000`, viết `25000`, viết dấu `+`. Còn `45000` là thứ máy tự làm ra.

Từ bài 1 đến giờ, máy chỉ đọc lại thứ bạn đưa cho nó. Đây là lần đầu tiên nó
tạo ra một giá trị mới mà bạn chưa từng gõ.

Cả nghề lập trình nằm trong chỗ đó: bạn không đưa sẵn câu trả lời, bạn đưa cách
làm ra câu trả lời.
::::

::::code{#tinh-tien-mot-luot}
Khách gọi một tô phở giá 45.000đ và một đĩa quẩy giá 10.000đ.

Hãy bảo máy **tính** tổng số tiền rồi nói ra. Đừng tự cộng nhẩm rồi gõ kết quả —
để máy làm việc đó.

```python title=starter
print(___)
```

```python title=solution
print(45000 + 10000)
```

```python title=test
# Bài này chấm bằng OUTPUT: người học chưa biết hàm nên chưa assert được gì.
# Khối này chỉ khẳng định chương trình chạy được tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm giữa hai dấu ngoặc của `print`. Lần này thứ đặt vào đó không phải một câu chữ — nên đừng gõ dấu nháy nào cả.
- kind: strategy
  body: Hai con số đã có sẵn trong đề bài là 45000 và 10000. Việc cần làm giữa chúng là cộng. Viết đúng phép cộng đó ra, máy sẽ tính hộ.
- kind: one-line
  body: Viết `45000 + 10000` vào chỗ trống, không dấu nháy.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: static
  onFail: đề bài cấm gõ thẳng kết quả — hãy để MÁY tính ra con số ấy từ hai số của bài
  requireAst:
  - kind: uses-operator, target: +, min: 1
  forbidAst:
  - kind: has-literal, target: 55000
- tier: output
  expect: 55000
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Con số 55000 không có trong dòng lệnh của bạn. Máy vừa tự làm ra nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hôm nay dấu `+` đứng giữa hai con số và nó cộng. Nhưng thực đơn quán phở không
phải số — nó là chữ.

Nếu bạn viết dòng này, **cả hai vế đều có dấu nháy**:

```text
print("Phở" + " bò")
```

thì máy "cộng" hai câu chữ kiểu gì? Nó dừng lại báo lỗi vì chữ thì không cộng
được, hay nó làm một việc khác hẳn?

Đừng trả lời vội. Bài sau bạn chạy đúng dòng này.
::::

::::checkpoint{mastery=0.8}
::::
