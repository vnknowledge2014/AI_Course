---
id: onboarding.may-tinh-noi-gi.khi-hai-kieu-khong-hop
title: Khi hai kiểu không đi cùng nhau
summary: Máy không có quy ước nào để cộng một câu chữ với một con số. Nó dừng lại và nói ra.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [err.type-error]
requires: [core.type-of-value, core.string-concat]
concepts: [core.kieu-gia-tri, core.loi-khi-chay]
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
Có lúc mình không đoán bừa được. Lúc đó mình dừng lại và nói cho bạn biết.
::::

::::explain{#mot-dau-cong-hai-viec}
Bài trước để lại một câu hỏi: viết `"45000"` — có dấu nháy — rồi cộng thêm
`5000`, thì máy ghép hai câu chữ hay cộng hai con số?

Trước khi trả lời, nhìn lại dấu `+` bạn đã dùng hai lần cho hai việc khác nhau:

- `45000 + 5000` — hai con số. Máy **tính**, ra `50000`.
- `"Phở" + " bò"` — hai câu chữ. Máy **ghép**, ra `"Phở bò"`.

Cùng một phím trên bàn phím, hai việc khác hẳn nhau. Vậy máy dựa vào đâu để
chọn việc nào? Dựa vào cái nhãn dán sẵn trên hai giá trị hai bên — thứ bài 14
gọi là **kiểu**.

Hai bên cùng nhãn `int`, máy biết phải tính. Hai bên cùng nhãn `str`, máy biết
phải ghép. Còn một bên `str`, một bên `int` thì sao?

Không có quy ước nào cho trường hợp đó. Và đây không phải chuyện máy lười: câu
hỏi ấy thật sự không có một câu trả lời đúng duy nhất. `"45000"` với `5000` thì
ra `"450005000"` nếu ghép, hay `50000` nếu tính? Hai kết quả đều nghe lọt tai,
và chúng cách nhau một trời một vực.
::::

::::example{#may-dung-lai-va-noi}
Đây là đúng câu hỏi bỏ ngỏ của bài trước, viết ra thành code:

```python title=readonly
gia_tren_bang = "45000"
tien_them_quay = 5000
print(gia_tren_bang + tien_them_quay)
```

Để ý dòng 1: `"45000"` nằm giữa hai dấu nháy, nên nhãn của nó là `str` — một
câu chữ, dù mọi ký tự bên trong đều là chữ số. Dòng 2 không có nháy, nhãn của
nó là `int`.

Chạy lên, máy không in ra con số nào. Nó in ra thế này:

```text
Traceback (most recent call last):
  File "quan_pho.py", line 3, in <module>
    print(gia_tren_bang + tien_them_quay)
          ~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~
TypeError: can only concatenate str (not "int") to str
```

Năm dòng chữ tiếng Anh. Bài này Byte chỉ nhờ bạn nhìn đúng **một chỗ** trong
đó: chữ đứng đầu dòng dưới cùng.

`TypeError` ghép từ *type* (kiểu) và *error* (lỗi). Dịch sát: **lỗi kiểu**. Nó
có nghĩa: *hai giá trị bạn đưa cho tôi mang hai kiểu không đi cùng nhau được
trong việc này.*

Và có một điều còn đáng nhớ hơn cả cái tên: máy **dừng hẳn**. Dòng `print`
không in ra gì cả. Nếu bên dưới còn mười dòng nữa, mười dòng đó cũng không được
chạy.
::::

::::predict{#doan-xem-may-lam-gi commitOnce}
Quán đếm được 3 tô. Byte sắp chạy đoạn dưới. **Trước khi bấm chạy**, bạn đoán
màn hình hiện ra gì?

```python
so_to = 3
print("Số tô: " + so_to)
```

:::opt{correct}
Máy không in dòng nào, chỉ hiện một thông báo TypeError
:::

:::opt
Số tô: 3
::why
Gần đúng ở chỗ bạn dùng dấu `+` đúng như bài nối chuỗi đã dạy: đặt nó giữa hai
mảnh để dán chúng thành một câu. Ý định của bạn chính xác, và đây đúng là câu
mà người viết đoạn code này muốn in ra.

Chỗ lệch nằm ở vế bên phải. `3` viết không có dấu nháy, nên nhãn của nó là
`int`, không phải `str`. Phép ghép chỉ chạy được khi **cả hai** vế cùng là chữ.
::
:::

:::opt
Máy hiểu ý bạn muốn nối, nên tự đổi 3 thành "3" rồi ghép
::why
Gần đúng ở chỗ suy nghĩ này rất hợp lý — và có những ngôn ngữ lập trình khác
làm đúng như vậy thật. Bạn không nghĩ sai về nguyên tắc.

Chỗ lệch nằm ở tính cách của Python. Nhớ lại bài 3: máy **không tự bổ sung** ý
bạn quên nói. Nếu ở đây nó tự đổi kiểu giúp bạn, thì lần khác bạn thật sự muốn
cộng hai số mà lỡ để một dấu nháy, nó cũng sẽ lặng lẽ ghép — và bạn sẽ không
bao giờ biết. Dừng lại để hỏi là cách nó giữ cho bạn khỏi sai âm thầm.
::
:::

:::opt
Máy in được phần chữ "Số tô: " rồi mới dừng
::why
Gần đúng ở chỗ bạn hình dung máy làm việc lần lượt từ trái sang phải, tới đâu
xong tới đó. Cách hình dung ấy đúng cho rất nhiều chuyện khác.

Chỗ lệch: `print` chỉ in khi đã có sẵn **một thứ hoàn chỉnh** trong tay. Máy
phải ghép xong `"Số tô: " + so_to` rồi mới đưa kết quả cho `print`. Phép ghép
hỏng từ trước đó, nên `print` chưa hề nhận được gì để in.
::
:::
::::

::::explain{#lam-cho-hai-ve-cung-kieu}
`TypeError` không phải chuyện hỏng hóc. Nó là một câu hỏi máy đặt ngược lại cho
bạn: *hai thứ này, bạn muốn tôi tính hay muốn tôi ghép?*

Bạn trả lời câu hỏi đó bằng cách làm cho hai vế **cùng một kiểu**:

- Muốn máy **tính** thì bỏ dấu nháy đi, để cả hai vế là số.
- Muốn máy **ghép** thì thêm dấu nháy vào, để cả hai vế là chữ.

Trước mắt bạn có đúng hai cách trên: sửa lại ngay chỗ mình viết giá trị ra. Còn
khi con số đến từ nơi khác — chẳng hạn do người ngồi trước màn hình gõ vào —
thì cần một công cụ riêng để đổi kiểu. Công cụ đó có thật, và bạn sẽ gặp nó ở
module sau.
::::

::::code{#cho-may-cong-duoc}
Sổ chi tiêu hôm nay: đi chợ hết 20 nghìn, rồi mua thêm bó rau 5 nghìn. Bạn muốn
máy in ra **tổng tiền**.

Byte viết `tien_ban_dau = "20000"` và nhận đúng thông báo `TypeError` như ví dụ
trên. Hãy điền vào chỗ trống sao cho máy **tính** được tổng.

```python title=starter
tien_ban_dau = ___
tien_bo_them = 5000
print(tien_ban_dau + tien_bo_them)
```

```python title=solution
tien_ban_dau = 20000
tien_bo_them = 5000
print(tien_ban_dau + tien_bo_them)
```

```python title=test
# Chấm bằng OUTPUT: người học chưa biết hàm nên chưa assert được gì.
# Khối này chỉ khẳng định chương trình chạy được tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Nhìn dòng thứ hai — `tien_bo_them` là `5000`, viết không có dấu nháy. Hai vế của dấu `+` cần mang cùng một kiểu.
- kind: strategy
  body: Bạn muốn máy tính chứ không muốn nó ghép, nên cả hai vế phải là số. Số thì viết trần, không bọc trong dấu nào cả.
- kind: one-line
  body: "Viết `20000` vào chỗ trống, không có dấu nháy."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: 25000
:::
::::

::::byte{trigger=success mood=happy pose=jump}
25000. Hai vế cùng kiểu thì mình biết ngay phải làm gì với chúng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Lúc nãy máy đổ ra năm dòng chữ, và Byte chỉ nhờ bạn nhìn một chỗ: chữ
`TypeError` ở đầu dòng dưới cùng. Bốn dòng còn lại thì sao — chúng nói gì?

Trong đó có một dòng cho bạn biết lỗi xảy ra ở **dòng số mấy** trong chương
trình của bạn. Với một bài ba dòng thì bạn tự dò ra được. Với một chương trình
hai trăm dòng thì đó là dòng đáng giá nhất trên màn hình.

Vậy năm dòng ấy đọc theo thứ tự nào, và dòng nào đáng đọc trước nhất? Bài sau
trả lời.
::::

::::checkpoint{mastery=0.8}
::::
