---
id: onboarding.ra-lenh-cho-byte.doi-chu-thanh-so
title: Đổi chữ thành số
summary: Một lệnh ba chữ cái biến chuỗi chữ số thành con số thật, tính toán được.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [core.int-cast]
requires: [core.input-returns-str, core.type-of-value]
concepts: [core.doi-kieu, core.kieu-gia-tri]
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
Chữ "25" và số 25 là hai thứ. Bài này là cây cầu bắc giữa chúng.
::::

::::explain{#nguoi-doc-to-giay}
Bài trước để lại một chỗ kẹt: trong tay bạn là chữ `"25"`, còn việc cần làm là
cộng thêm 1.

Quay lại cái quán. Người phụ quán đưa tờ giấy có hai nét mực *2* và *5* cho bác
thu ngân. Bác không cộng tờ giấy vào sổ — bác **đọc** nó, ra được con số hai
mươi lăm trong đầu, rồi ghi con số ấy vào cột số. Tờ giấy vẫn nằm nguyên trên
bàn, không mất đi đâu; thứ mới sinh ra là con số.

Python có đúng một lệnh làm việc đọc ấy: **`int`**.

Cái tên này bạn đã nhìn thấy rồi. Nó chính là chữ nằm trong nhãn
`<class 'int'>` — viết tắt của *integer*, số nguyên. Từ giờ nó có hai vai: vừa
là tên của một kiểu, vừa là tên của việc *đổi sang kiểu ấy*.

Thuật ngữ để bạn tra cứu về sau: **ép kiểu** (hoặc *chuyển kiểu*) — biến một giá
trị thuộc kiểu này thành một giá trị thuộc kiểu khác.
::::

::::example{#doc-to-giay-ra-so}
Đây là chỗ kẹt của bài trước, được gỡ bằng một dòng:

```python title=readonly
tuoi_go_vao = "25"
tuoi = int(tuoi_go_vao)
print(tuoi + 1)
```

Máy in ra:

```text
26
```

Đọc dòng giữa từ trong ra ngoài:

- `tuoi_go_vao` là cái tên đang giữ chữ `"25"` — tờ giấy.
- `int( ... )` nhận chữ ấy và **làm ra một giá trị mới**: con số 25, mang nhãn
  `int`.
- Dấu `=` dán giá trị mới ấy lên cái tên `tuoi`.

Từ dòng thứ ba trở đi, `tuoi` là một con số đàng hoàng. Máy cộng, trừ, so sánh
với nó được hết — như mọi con số bạn tự viết trong code.

Muốn kiểm chứng thì hỏi nhãn, đúng cách bạn đã làm ở bài trước:

```python
print(type(tuoi_go_vao))   # <class 'str'> — tờ giấy vẫn là chữ
print(type(tuoi))          # <class 'int'> — giá trị mới là số
```
::::

::::predict{#doan-tien-hai-to commitOnce}
Khách gõ giá tiền một tô vào máy, quán muốn cộng thêm 5000 tiền quẩy. Byte sắp
chạy đoạn dưới.

**Trước khi bấm chạy**, bạn đoán nó in ra gì?

```python title=readonly
gia_go_vao = "45000"
gia = int(gia_go_vao)
print(gia + 5000)
```

:::opt{correct}
50000
:::

:::opt
450005000
::why
Gần đúng ở chỗ bạn giữ chắc bài học vừa xong: `"45000"` là chữ, mà cộng hai câu
chữ thì chúng dính vào nhau chứ không cộng. Nếu dòng giữa không có, phản xạ ấy
đưa bạn đi rất xa.

Chỗ lệch nằm ở dòng giữa. `int(gia_go_vao)` đã làm ra một giá trị **mới** — con
số 45000 — và `gia` đang giữ con số ấy chứ không giữ chữ nữa. Số cộng với số thì
máy cộng thật.

Một chi tiết nhỏ nữa: kể cả khi `gia` vẫn là chữ, máy cũng không cho ra
`450005000`. Vế phải là `5000` viết trần — một con số — nên máy sẽ dừng lại,
đúng như bạn đã thấy ở bài trước.
::
:::

:::opt
Máy dừng lại và báo TypeError, y như bài trước
::why
Gần đúng ở chỗ bạn nhận ra bài trước máy đã dừng đúng tại một phép cộng trông
hệt thế này, và bạn đang cảnh giác — đó là thói quen của người đọc code kỹ.

Chỗ lệch là hai vế bây giờ đã cùng loại. `TypeError` xuất hiện khi máy không có
quy ước nào để ghép chữ với số. Ở đây `int` đã đổi vế trái thành số rồi, nên quy
ước cộng hai số áp dụng được và máy làm bình thường.
::
:::

:::opt
50000 nhưng in ra kèm dấu nháy, thành "50000"
::why
Gần đúng ở chỗ bạn để ý tới dấu nháy — thứ đã theo bạn suốt từ bài đầu tiên và
là ranh giới giữa chữ với số.

Chỗ lệch: dấu nháy chỉ tồn tại **trong code**, để bạn báo cho máy biết phần này
là chữ. Lúc in ra màn hình, `print` đưa nội dung chứ không đưa dấu nháy. Và ở
đây `gia + 5000` vốn đã là một con số, càng không có dấu nháy nào để mà in.
::
:::
::::

::::explain{#int-khong-dong-vao-cai-cu}
Hai điều đáng nhớ về `int`, cả hai đều hay làm người mới vấp:

**Nó không sửa cái tên cũ.** Sau khi chạy `tuoi = int(tuoi_go_vao)`, thứ nằm
trong `tuoi_go_vao` vẫn là chữ `"25"` như trước. `int` không đụng vào tờ giấy —
nó đọc tờ giấy rồi sinh ra một giá trị mới. Giá trị mới ấy chỉ được giữ lại nếu
bạn dán nó lên một cái tên. Viết `int(tuoi_go_vao)` trơ trọi một mình thì máy
đổi xong rồi bỏ đấy, chẳng ai giữ.

**Người viết code lâu năm hay gộp hai dòng thành một:**

```python
tuoi = int(input("Bác bao nhiêu tuổi? "))
```

Hai lớp ngoặc lồng nhau, đọc từ trong ra ngoài như cách bạn đã đọc
`print(type(...))`: `input` hỏi và chờ → nó đưa về một chuỗi → `int` đổi chuỗi
ấy thành số → dấu `=` dán số lên `tuoi`.

Dạng gộp ngắn hơn, nhưng lúc hỏng thì khó nhìn ra hỏng ở lớp nào. Bạn cứ viết
hai dòng cho tới khi thấy dạng gộp dễ đọc với mình — hai cách chạy y hệt nhau.
::::

::::code{#sang-nam-bac-bao-nhieu}
Đến lượt bạn gỡ nốt chỗ kẹt của bài trước.

Ở máy của bạn, dòng đầu sẽ là `tuoi_go_vao = input("Bác bao nhiêu tuổi? ")`.
Trang luyện tập chưa nối được vào bàn phím nên Byte gõ hộ, đúng thứ `input` đưa
về khi bác khách gõ *25*.

Hãy điền vào chỗ trống để `tuoi` giữ một **con số**, rồi dòng dưới in ra tuổi
của bác vào sang năm.

```python title=starter
# Byte gõ hộ bàn phím: đây đúng là thứ input() đưa về khi bác khách gõ 25.
tuoi_go_vao = "25"

tuoi = ___
print(tuoi + 1)
```

```python title=solution
# Byte gõ hộ bàn phím: đây đúng là thứ input() đưa về khi bác khách gõ 25.
tuoi_go_vao = "25"

tuoi = int(tuoi_go_vao)
print(tuoi + 1)
```

```python title=test
# Chấm bằng OUTPUT: người học chưa biết hàm nên chưa assert được gì.
# Khối này chỉ khẳng định chương trình chạy được tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm bên phải dấu `=`. Dòng dưới làm phép `tuoi + 1`, nên thứ bạn đặt vào `tuoi` phải là một con số chứ không phải chữ.
- kind: strategy
  body: Bạn cần một giá trị mới, vẫn là 25 nhưng mang nhãn `int`. Lệnh làm việc đó trùng tên với chính cái nhãn ấy, và nó nhận vào chuỗi cần đọc, đặt giữa hai dấu ngoặc.
- kind: one-line
  body: "Viết `int(tuoi_go_vao)` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải ĐỌC con số đang nằm trong `tuoi_go_vao` bằng `int` — đừng gõ thẳng con số 25, và cũng đừng gõ lại chuỗi "25"
  requireAst:
  # Hai điều kiện cùng lúc. Thiếu điều kiện thứ nhất thì `tuoi = 25` chép cứng
  # đi lọt; thiếu điều kiện thứ hai thì `int("25")` đi lọt — và cả hai đều bỏ
  # qua đúng thứ bài dạy: đọc ô chữ mà `input` vừa đưa về.
  - kind: uses-call, target: int, min: 1
  - kind: uses-name, target: tuoi_go_vao, min: 1
- tier: output
  expect: 26
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cộng được rồi. Cây cầu giữa chữ và số hoá ra chỉ dài đúng ba chữ cái.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bây giờ chương trình của bạn hỏi tuổi, đọc câu trả lời thành số, rồi tính. Nó
chạy ngon với `25`, với `70`, với `9`.

Nhưng một bác khách không gõ chữ số. Bác gõ nguyên câu **hai mươi lăm** rồi bấm
Enter.

`int("hai mươi lăm")` sẽ ra cái gì? Máy đoán bừa thành 25? Trả về 0 cho xong
chuyện? Hay dừng lại và nói ra?

Đừng trả lời vội. Bài sau bạn cho máy nhận đúng câu đó và xem nó xử trí ra sao —
lỗi hiện lên sẽ là một loại bạn chưa gặp, và nó khác `TypeError` ở một điểm rất
cụ thể.
::::

::::checkpoint{mastery=0.8}
::::
