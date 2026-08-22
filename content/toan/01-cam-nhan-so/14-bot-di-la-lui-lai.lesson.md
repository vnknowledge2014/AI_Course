---
id: toan.cam-nhan-so.bot-di-la-lui-lai
title: Bớt đi là lùi lại
summary: Bước sang trái trên thanh số có tên là phép trừ — và nó chính là "bớt đi" của tranh đống, vì mỗi bước lùi hoàn tác đúng một bước tiến.
locale: vi
track: toan
module: cam-nhan-so
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.number-line-subtract]
requires: [math.number-line-add, math.thanh-so, math.addition-as-union, core.variable, core.arithmetic, core.print-variable]
concepts: [math.bot-di, math.don-vi, math.moc-khong]
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
Không có gì trên sải dây cấm mình quay người lại. Quay xong thì gọi là gì?
::::

::::explain{#quay-nguoi-lai}
Bài trước dừng ở một câu hỏi: thanh số nằm đó cả hai chiều, vậy bước sang
**trái** là phép gì?

Nó có tên, và bạn đã nghe cái tên ấy từ lâu: **phép trừ**.

Cũng như phép cộng, phép trừ có hai bức tranh, và bài này dựng cả hai cạnh nhau:

- **Tranh đống — bớt đi.** Có một đống, lấy bớt ra khỏi đó một số cái, đếm chỗ
  còn lại. Sâu ăn mất mấy cây thì luống còn ngần ấy cây.
- **Tranh thanh số — lùi lại.** Đứng ở một vạch, quay người, bước sang trái bấy
  nhiêu vạch, xem dừng ở vạch nào.

Nghe thì hai chuyện khác hẳn nhau. Một bên là lấy đồ ra khỏi rổ, một bên là đi
bộ. Nhưng chúng luôn cho cùng một con số, và lý do gọn đúng một câu: **mỗi bước
sang trái hoàn tác đúng một bước sang phải.**

Bài trước đã nói bước sang phải là thêm một đơn vị vào lượng. Vậy bước ngược lại
là bỏ ra một đơn vị. Lùi năm bước là bỏ ra năm cái — đúng việc mà con sâu vừa
làm với luống của Byte.
::::

::::example{#sau-an-mat-nam-cay}
Byte đếm được 12 cây trên luống. Sáng ra, sâu ăn mất 5 cây. Còn mấy cây?

Tranh đống: nhặt 5 cây ra khỏi đống 12 cây, đếm chỗ còn lại — 7 cây.

Tranh thanh số: đứng ở vạch 12, quay người, bước sang trái 5 vạch.

```text
   0   1   2   3   4   5   6   7   8   9  10  11  12
   ├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
                               ├─5─┼─4─┼─3─┼─2─┼─1─┤
```

Hình này đọc **từ phải sang trái**, vì Byte đi từ phải sang trái. Bước số 1 là
bước rời vạch 12 sang vạch 11, bước số 2 sang vạch 10, và bước số 5 đặt chân
xuống vạch 7. Số ghi trong mỗi ô là bước thứ mấy, không phải vạch nào.

Hỏi máy cả hai cách:

```python title=readonly
# tranh đống: đống 12 cây, nhặt ra 5 cây
print(12 - 5)

# tranh thanh số: đứng ở vạch 12, lùi sang trái 5 vạch
vach_dang_dung = 12
so_buoc_lui = 5
print(vach_dang_dung - so_buoc_lui)
```

Máy in ra:

```text
7
7
```

Lại một dấu duy nhất cho hai bức tranh. Máy không biết bạn đang nhặt cây hay
đang đi bộ; nó chỉ đếm đơn vị, và hai chuyện ấy đếm ra cùng một số.
::::

::::predict{#doan-lui-roi-tien commitOnce}
Byte thử một chuyện: lùi 5 bước, rồi tiến lại đúng 5 bước.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(12 - 5)
print(12 - 5 + 5)
```

:::opt{correct}
7 rồi 12
:::

:::opt
7 rồi 7
::why
Gần đúng ở chỗ bạn nhận ra `− 5` rồi `+ 5` triệt tiêu nhau — lùi năm bước rồi
tiến lại năm bước thì cuối cùng chẳng đi đâu cả. Ý nghĩ đó chính là điều bài này
muốn nói, và nó không sai chỗ nào.

Chỗ lệch nằm ở câu *chẳng đi đâu cả — so với vạch nào*. Hai bước đó triệt tiêu
nhau quanh **chỗ xuất phát**, tức vạch 12. Vạch 7 chỉ là nơi Byte tạm dừng giữa
đường; tiến lại 5 bước nghĩa là rời vạch 7 mà đi. Kết quả cuối phải là chỗ Byte
đứng lúc chưa động đậy.
::
:::

:::opt
7 rồi 2
::why
Gần đúng ở chỗ bạn đang dùng một quy tắc thật và rất hay dùng: *cái dấu đứng
trước con số nào thì nói về con số ấy*. Bạn thấy dấu `−` mở đầu và hiểu rằng
dòng này là chuyện lùi — hiểu vậy là trúng nửa dòng.

Chỗ lệch là phạm vi của quy tắc đó: một cái dấu chỉ với tới **đúng một** con số
ngay sau nó, không phủ tiếp sang con số kế. Số 5 thứ hai mang dấu riêng của nó,
và dấu ấy là `+`. Nên dòng này là *lùi 5 rồi tiến 5*, không phải *lùi 5 rồi lùi
tiếp 5*.
::
:::

:::opt
7 rồi 22
::why
Gần đúng ở chỗ bạn đọc cả dòng như một đống số phải gom lại: 12, 5 và 5 cùng
nằm trên một dòng nên cùng góp vào kết quả — 12 gộp 5 gộp 5 là 22. Quy tắc "mọi
số trên dòng đều góp vào tổng" là quy tắc đúng, và bài 10 đã dạy đúng nó.

Chỗ lệch: nó đúng khi mọi dấu đều là `+`. Ở đây có một dấu `−`, và dấu ấy đổi
**chiều đi** của con số đứng sau nó. Trên thanh số, hai bước ngược chiều nhau
thì không cộng dồn — chúng ăn bớt của nhau.
::
:::
::::

::::explain{#lui-hoan-tac-tien}
Đặt tên cho thứ vừa dựng:

> Trên thanh số, `a − b` nghĩa là **đứng ở vạch `a` rồi lùi sang trái `b`
> vạch.**

Ba điều đi kèm, cả ba đều là cùng một câu "lùi hoàn tác tiến" nói lại:

- **Trừ 0 là đứng yên.** `9 − 0` vẫn là 9. Không bước bước nào thì không rời
  vạch nào.
- **Trừ hết thì về đúng mốc 0.** `12 − 12` là 0: lùi đủ 12 bước từ vạch 12 thì
  chân đặt xuống đầu luống. Bên tranh đống, đó là cái rổ sạch trơn.
- **Lùi rồi tiến, hay tiến rồi lùi — chừng ấy bước thì đều về chỗ cũ.**
  `(12 − 5) + 5` là 12, đúng như máy vừa in; và đi vòng kia cũng thế, `(9 + 5) −
  5` là 9. Chiều nào đi trước không quan trọng, miễn số bước hai lượt bằng nhau.

Câu thứ ba đáng ghi lại, vì nó không chỉ là một mẹo thử. Nó nói rằng phép trừ và
phép cộng **tháo được cho nhau**: cái này làm gì thì cái kia gỡ ra được. Byte sẽ
xài đúng câu đó ở bài sau, và xài để trả lời một câu hỏi trông chẳng liên quan
gì tới việc lùi.
::::

::::code{#con-lai-bao-nhieu}
Hai chuyện xảy ra trong vườn sáng nay — mỗi chuyện một bức tranh.

- **Tranh đống.** Luống của Byte có `12` cây. Sâu ăn mất `5` cây. Còn mấy cây?
- **Tranh thanh số.** An đang ở vạch `9` trên sợi dây. An quay người, lùi đủ `9`
  bước. Chân đặt xuống vạch nào?

Điền hai chỗ trống để máy trả lời cả hai.

Bài chấm bằng **cả hai chuyện**, và chúng được chọn để cho ra hai con số khác
nhau: một bên còn dư, một bên về đúng mốc 0. Gõ cứng `7` vào cả hai chỗ thì An
sai; gõ cứng `0` thì luống của Byte sai. Chỉ một phép trừ viết thật mới qua được
cả hai.

```python title=starter
cay_tren_luong = 12
cay_sau_an = 5
cay_con_lai = ___

vach_dang_dung_an = 9
so_buoc_lui_an = 9
vach_toi_an = ___

print(cay_con_lai)
print(vach_toi_an)
```

```python title=solution
cay_tren_luong = 12
cay_sau_an = 5
cay_con_lai = cay_tren_luong - cay_sau_an

vach_dang_dung_an = 9
so_buoc_lui_an = 9
vach_toi_an = vach_dang_dung_an - so_buoc_lui_an

print(cay_con_lai)
print(vach_toi_an)
```

```python title=test
# Hai chuyện cho ra hai con số khác nhau, nên một con số gõ cứng chỉ qua được
# nhiều nhất một câu. Hai câu cuối chốt lại "lùi hoàn tác tiến", theo cả hai
# chiều — đúng hai chiều mà hộp tổng kết vừa nêu.
assert cay_con_lai == 7, "12 cây bớt 5 cây thì còn 7 cây"
assert vach_toi_an == 0, "lùi đủ 9 bước từ vạch 9 thì về đúng mốc 0 — đầu luống"
assert cay_con_lai + cay_sau_an == cay_tren_luong, "trả 5 cây về chỗ cũ thì phải đủ lại 12 — phép cộng gỡ được phép trừ"
assert (cay_con_lai + cay_sau_an) - cay_sau_an == cay_con_lai, "tiến rồi lùi chừng ấy bước cũng về đúng chỗ cũ"
```

:::hints
- kind: attention
  body: Nhìn hai dòng ngay phía trên mỗi chỗ trống. Ở chuyện thứ nhất, một dòng cho biết lúc đầu có bao nhiêu, dòng kia cho biết mất đi bao nhiêu. Ở chuyện thứ hai, một dòng cho biết An đang đứng ở vạch nào, dòng kia cho biết An lùi mấy bước. Chỗ trống nào cũng cần cả hai dòng.
- kind: strategy
  body: Trên thanh số, chỗ tới = chỗ đang đứng lùi sang trái bấy nhiêu vạch — và "bớt đi" của tranh đống cũng viết ra đúng hình dạng ấy. Viết mỗi chỗ trống bằng hai cái tên có sẵn nối bằng dấu trừ, và để ý thứ tự: cái đứng trước dấu trừ là chỗ xuất phát.
- kind: one-line
  body: "Chỗ trống thứ nhất là `cay_tren_luong - cay_sau_an`, chỗ trống thứ hai là `vach_dang_dung_an - so_buoc_lui_an`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một phép trừ giữa hai cái tên đứng ngay trên nó — gõ thẳng con số kết quả thì bạn đã tính hộ máy rồi
  requireAst:
  # `min: 2` vì có hai chuyện, mỗi chuyện một phép trừ. Khung khởi đầu chưa có
  # dấu `-` nào, nên luật này chặn được đúng cái đáp án chép cứng hai con số.
  - kind: uses-operator, target: -, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^7\n0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Còn 7 cây, và về đúng vạch 0. Lùi tới đầu luống vẫn là lùi, mình đếm được hết.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài trước Byte có ba câu hỏi. Hai câu đầu vừa được trả lời: *sâu ăn mất 5 trong
12 cây thì còn 7*, *đong ra 5 trong 12 lon thì trong thùng còn 7*. Cả hai đều là
lùi lại.

Còn câu thứ ba thì chưa: *cuốc được tới vạch 5 rồi, mà cả luống dài 12 — còn
phải cuốc bao xa nữa?*

Câu này không có ai bớt gì của ai cả. Không con sâu nào ăn cây, không ai múc lon
gạo nào ra khỏi thùng. Luống vẫn dài đúng 12 sải như lúc đầu. Nó chỉ hỏi hai cái
vạch nằm cách nhau bao xa.

Thế mà bấm máy tính ra vẫn là 7.

*"12 bớt 5"* và *"từ 5 tới 12 còn cách bao xa"* — hai câu hỏi nghe chẳng dính gì
nhau, cùng ra một con số. Vì sao? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
