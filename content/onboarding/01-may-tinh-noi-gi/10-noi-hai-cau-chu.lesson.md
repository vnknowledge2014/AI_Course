---
id: onboarding.may-tinh-noi-gi.noi-hai-cau-chu
title: Nối hai câu chữ
summary: Cùng một dấu cộng, đổi loại giá trị hai bên thì máy làm một việc khác hẳn.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [core.string-concat]
requires: [core.string-literal, core.arithmetic]
concepts: [core.chuoi, core.noi-chuoi]
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
Vẫn dấu cộng đó. Đổi thứ đứng hai bên, mình làm việc khác.
::::

::::explain{#cong-hai-cau-chu-nghia-la-gi}
Câu hỏi bỏ ngỏ cuối bài trước: `"Phở" + " bò"`.

Hai vế đều nằm trong dấu nháy, nên cả hai đều là chữ. Mà cộng hai câu chữ thì
chẳng ra con số nào — trong toán, việc đó không có nghĩa.

Người làm ra Python đứng trước đúng chỗ khó đó, và họ có hai đường:

- Bắt máy dừng lại báo lỗi, vì chữ thì không cộng được.
- Giao cho dấu `+` một **việc thứ hai**, chỉ dùng khi cả hai bên đều là chữ.

Họ chọn đường thứ hai. Việc thứ hai ấy là: **dán hai câu chữ lại thành một**,
câu sau nối ngay sau câu trước, đúng thứ tự bạn viết.

Hãy hình dung hai mẩu giấy trên bảng thực đơn: một mẩu in *Phở*, một mẩu in
*bò*. Đặt sát nhau, người đọc thấy *Phở bò*. Không ai cộng gì cả — chỉ là hai
mẩu nằm cạnh nhau.

Giờ đến thuật ngữ, để sau này bạn tra cứu được: một câu chữ nằm giữa hai dấu
nháy được gọi là một **chuỗi**. Việc dán hai chuỗi lại thành một gọi là **nối
chuỗi**.
::::

::::example{#dan-hai-mau}
Đây là dòng bạn đã đoán ở cuối bài trước:

```python title=readonly
print("Phở" + " bò")
```

Máy in ra:

```text title=readonly
Phở bò
```

Máy làm hai việc, theo thứ tự y như bài trước — chỉ khác ở việc đầu tiên:

1. **Nối trước.** Lấy nguyên văn `Phở`, rồi nguyên văn ` bò`, dán lại thành một
   chuỗi mới: `Phở bò`.
2. **Nói sau.** Đưa chuỗi mới đó cho `print`.

Bây giờ hãy nhìn thật kỹ vế thứ hai: `" bò"`. Ngay sau dấu nháy mở có một **dấu
cách**, rồi mới tới chữ *bò*.

Dấu cách đó là do bạn gõ. Nó nằm trong nháy, nên nó là một ký tự của chuỗi, y
như chữ *b* hay chữ *ò*. Bỏ nó đi thì nó không còn nữa — máy không tự thêm vào.
::::

::::predict{#quen-dau-cach commitOnce}
Byte xoá đúng một dấu cách so với ví dụ trên. **Trước khi bấm chạy**, bạn đoán
màn hình hiện ra gì?

```python title=readonly
print("Phở" + "bò")
```

:::opt{correct}
Phởbò
:::

:::opt
Phở bò
::why
Gần đúng ở chỗ bạn đọc như một người đọc tiếng Việt: thấy *Phở* rồi thấy *bò*,
đầu bạn tự chèn khoảng trắng vào giữa vì có thế câu mới xuôi. Ai cũng đọc như
bạn.

Máy thì không đọc nghĩa. Nó dán đúng những ký tự nằm giữa hai dấu nháy, không
thêm một ký tự nào. Trong `"Phở"` không có dấu cách, trong `"bò"` cũng không —
nên chuỗi kết quả không có dấu cách nào.

Đây vẫn là chuyện bài 3 đã nói: máy không tự bổ sung thứ bạn quên nói.
::
:::

:::opt
Máy báo lỗi, vì cộng hai câu chữ thì không có nghĩa
::why
Gần đúng, và lập luận của bạn rất chắc: trong toán, cộng hai câu chữ đúng là
không có nghĩa thật. Nếu Python chọn đường thứ nhất ở phần đầu bài, đáp án này
sẽ là đáp án đúng.

Chỗ lệch chỉ là một quy ước mà bạn chưa biết trước được: khi cả hai bên đều là
chuỗi, dấu `+` không còn nghĩa cộng nữa mà mang nghĩa nối. Máy biết rõ phải làm
gì, nên nó không dừng.

Còn khi một bên là chữ, bên kia là số — lúc đó mới có chuyện. Vài bài nữa bạn sẽ
gặp đúng tình huống ấy.
::
:::

:::opt
Phở + bò
::why
Gần đúng ở chỗ bạn nhớ đúng luật quan trọng nhất từ bài 1: thứ nằm giữa hai dấu
nháy được đọc nguyên văn, dấu cộng nằm trong nháy thì cũng chỉ là một nét mực.

Chỗ lệch: dấu `+` ở dòng này nằm **ngoài** cả hai cặp nháy. Nó không thuộc chuỗi
nào cả, nên máy không đọc nó nguyên văn — nó là lệnh nối hai chuỗi hai bên lại.
Cứ nhìn xem dấu `+` nằm trong hay ngoài dấu nháy, bạn sẽ biết nó là chữ hay là
lệnh.
::
:::
::::

::::explain{#noi-nhieu-manh}
Nối được nhiều hơn hai mẩu. Ba mẩu, bốn mẩu, bao nhiêu cũng được — cứ đặt dấu
`+` giữa hai mẩu kề nhau:

```python
print("Phở" + " " + "bò")
```

Mẩu ở giữa, `" "`, là một chuỗi chỉ chứa đúng một dấu cách. Nghe lạ, nhưng dấu
cách cũng là một ký tự như mọi ký tự khác, và một chuỗi hoàn toàn có thể chỉ
gồm nó.

Đây là cách viết dễ nhìn hơn khi bạn muốn thấy rõ chỗ nào có khoảng trắng: thay
vì giấu dấu cách vào đầu `" bò"`, bạn tách nó ra thành một mẩu riêng.
::::

::::code{#ten-mon-day-du}
Bảng thực đơn cần in ra đúng dòng này:

```text title=readonly
Phở bò tái nạm
```

Byte đã viết sẵn mẩu đầu. Hãy điền mẩu còn lại vào chỗ trống.

```python title=starter
print("Phở bò" + ___)
```

```python title=solution
print("Phở bò" + " tái nạm")
```

```python title=test
# Bài này chấm bằng OUTPUT: người học chưa biết hàm nên chưa assert được gì.
# Khối này chỉ khẳng định chương trình chạy được tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Nhìn kỹ dòng cần in ra. Giữa chữ "bò" và chữ "tái" có một dấu cách. Dấu cách đó phải do ai gõ ra?
- kind: strategy
  body: Chỗ trống cần một câu chữ, nên nó phải nằm giữa hai dấu nháy. Bên trong cặp nháy ấy, viết đủ phần còn thiếu — kể cả dấu cách đứng đầu.
- kind: one-line
  body: "Viết `\" tái nạm\"` vào chỗ trống — dấu cách nằm ngay sau dấu nháy mở."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Phở bò tái nạm
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai mẩu chữ rời, một dòng thực đơn. Bạn vừa ghép chuỗi lần đầu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bảng thực đơn của quán có mười hai dòng, dòng nào cũng bắt đầu bằng
`"Phở bò tái nạm"`. Bạn sẽ gõ nguyên câu ấy mười hai lần.

Gõ mười hai lần thì mỏi tay, và thế nào cũng có một lần gõ nhầm thành
`"Phơ bò tái nạm"` — máy sẽ in ra nguyên văn chỗ nhầm đó, vì nó đọc nguyên văn
mọi thứ trong nháy.

Vậy có cách nào nói với máy **một lần** rằng "từ giờ, câu chữ này tôi gọi là
`mon_dac_biet`", rồi những lần sau chỉ gọi cái tên ngắn ấy?

Bài 8 đã cho thấy máy sẵn sàng đi tìm một cái tên — chỉ là nó chưa tìm thấy gì.
Vậy làm sao để có cái cho nó tìm thấy?

Đừng trả lời vội. Bài sau làm đúng việc đó.
::::

::::checkpoint{mastery=0.8}
::::
