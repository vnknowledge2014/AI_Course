---
id: nen-tang.re-nhanh-va-lap.xem-lai-dieu-kien-luc-nao
title: Điều kiện được xem lại lúc nào
summary: Máy đọc điều kiện của `while` đúng một lần ở đầu mỗi lượt — thân đã vào thì chạy trọn, sai ngay từ đầu thì vòng chạy 0 lượt.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ctrl.while-check-timing]
requires: [ctrl.while, ctrl.if, core.reassign, core.fstring]
concepts: [ctrl.lap, core.dung-sai]
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
Mình không vừa làm vừa liếc điều kiện. Mình nhìn nó đúng một chỗ.
::::

::::explain{#nhin-vao-noi-luc-nao}
Bài trước để lại hai cách hiểu về cùng một vòng lặp, và cả hai đều nghe lọt tai:

- máy canh chừng điều kiện suốt lượt, hễ thấy sai là dừng ngay giữa thân;
- máy đợi thân chạy hết lượt rồi mới hỏi lại.

Quay lại quán phở của bà chủ. Trước khi múc một tô, bà nhìn vào nồi: còn nước
dùng thì múc. Nhưng khi đã cầm muôi lên, bà làm trọn tô ấy — chan nước, rắc
hành, bưng ra bàn — kể cả khi muôi vừa rồi vét đúng giọt cuối cùng. Không ai bỏ
dở nửa tô phở giữa chừng chỉ vì nồi vừa cạn.

Chỗ bà nhìn vào nồi là **một chỗ cố định**: lúc chuẩn bị múc tô mới. Không phải
lúc đang chan, không phải lúc đang rắc hành.

Máy làm đúng như vậy với `while`. Điều kiện được đọc ở **đầu mỗi lượt**, ngay
trước khi máy bước vào thân:

- Đọc ra `True` — máy vào thân và chạy **hết** mọi dòng trong đó, không ngoái
  lại nhìn điều kiện lần nào giữa chừng.
- Chạy xong dòng cuối của thân, máy mới quay lên đọc điều kiện lần nữa. Lần đọc
  ấy chính là đầu lượt kế tiếp.
- Đọc ra `False` — máy không vào thân, nó nhảy thẳng xuống dòng đầu tiên nằm
  ngoài vòng lặp.

Cách nói gọn của giới lập trình: `while` là vòng lặp **kiểm điều kiện ở đầu
vòng**. Vậy trong hai cách hiểu lúc nãy, cách thứ hai mới đúng.
::::

::::example{#luot-cuoi-van-chay-tron}
Nồi sáng nay còn hai tô. Thân vòng có ba dòng, và dòng cuối cùng báo lại số tô
còn trong nồi:

```python title=readonly
to_con_lai = 2

while to_con_lai > 0:
    print("Múc một tô cho khách")
    to_con_lai = to_con_lai - 1
    print(f"Trong nồi còn {to_con_lai} tô")

print("Treo biển nghỉ")
```

Máy in ra:

```text
Múc một tô cho khách
Trong nồi còn 1 tô
Múc một tô cho khách
Trong nồi còn 0 tô
Treo biển nghỉ
```

Dòng đáng nhìn nhất là dòng thứ tư: **Trong nồi còn 0 tô**.

Lúc dòng ấy được in, `to_con_lai` đã là 0 — nghĩa là điều kiện `to_con_lai > 0`
đã sai từ trước đó, sai ngay khi dòng trừ vừa chạy xong. Vậy mà dòng nằm dưới
nó vẫn in ra bình thường.

Đó là bằng chứng nhìn thấy được: máy không canh chừng điều kiện giữa thân. Đã
cho lượt này vào thì lượt này chạy trọn.

Đi lại đúng đường máy đi, ghi rõ mỗi lần điều kiện được đọc:

- **Đầu lượt 1** — đọc `2 > 0`, ra `True`. Vào thân: in câu múc tô, hạ
  `to_con_lai` xuống 1, in "Trong nồi còn 1 tô". Hết thân.
- **Đầu lượt 2** — đọc `1 > 0`, ra `True`. Vào thân: in câu múc tô, hạ
  `to_con_lai` xuống 0, in "Trong nồi còn 0 tô". Hết thân.
- **Đầu lượt 3** — đọc `0 > 0`, ra `False`. Không vào thân. Máy nhảy xuống dòng
  treo biển.

Ba lần đọc điều kiện, hai lượt chạy thân. Lần đọc cuối cùng bao giờ cũng là lần
cho ra `False`, và đó là lần duy nhất không kèm theo một lượt nào.
::::

::::predict{#noi-rong-tu-dau commitOnce}
Trưa nay Byte chỉ còn 30 nghìn trong ví, mà tô phở thì 45 nghìn.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python title=readonly
tien_con = 30000

while tien_con >= 45000:
    print("Ăn một tô")
    tien_con = tien_con - 45000

print(f"Còn {tien_con} đồng")
```

:::opt{correct}
Chỉ một dòng: Còn 30000 đồng
:::

:::opt
Ăn một tô, rồi Còn -15000 đồng
::why
Gần đúng ở chỗ bạn giữ một hình dung hợp lý về chữ *lặp*: đã lặp thì phải làm
một lần rồi mới tính chuyện có làm nữa hay không. Có những ngôn ngữ lập trình
khác cho sẵn một kiểu vòng lặp chạy đúng như thế thật.

Chỗ lệch nằm ở thứ tự trong `while`: lần đọc điều kiện **đầu tiên** xảy ra
trước cả lượt đầu tiên. Máy đọc `30000 >= 45000`, ra `False`, nên thân không
được vào lần nào. Bà chủ nhìn vào nồi trước khi cầm muôi — nồi rỗng thì cái
muôi không hề nhấc lên.
::
:::

:::opt
Màn hình trống trơn, vì vòng không chạy lượt nào
::why
Gần đúng ở chỗ khó nhất: bạn nhận ra `30000 >= 45000` sai ngay từ lần đọc đầu,
nên thân không chạy lượt nào. Phần suy luận về vòng lặp của bạn chính xác.

Chỗ lệch nằm ở dòng cuối cùng. `print(f"Còn {tien_con} đồng")` viết sát lề
trái, không thụt vào, nên nó **không thuộc** thân vòng — giống hệt chuyện một
dòng `print` nằm ngoài `if` thì chạy trong mọi trường hợp. Vòng chạy 0 lượt
cũng không ngăn được dòng nằm sau nó.
::
:::

:::opt
Máy báo lỗi, vì một vòng lặp không chạy lượt nào là chuyện không ổn
::why
Gần đúng ở chỗ bạn đang áp một suy luận hợp lý: đã gọi là vòng **lặp** thì phải
lặp, nên một vòng không chạy lượt nào nghe như một câu lệnh vô nghĩa — mà thứ
vô nghĩa thì máy hay bắt lỗi.

Chỗ lệch: 0 lượt là một kết quả hợp lệ, đàng hoàng, y như `False` là câu trả
lời đàng hoàng cho một `if`. Nghĩ về nghĩa ngoài đời mà xem: ví không đủ tiền
thì ăn 0 tô, và đó là chuyện đúng phải xảy ra chứ không phải hỏng hóc. Máy chỉ
báo lỗi khi nó không hiểu bạn viết gì.
::
:::
::::

::::explain{#hai-he-qua}
Một câu duy nhất — *điều kiện chỉ được đọc ở đầu mỗi lượt* — kéo theo hai
chuyện bạn dùng được ngay:

- **Thân đã vào thì chạy trọn.** Điều kiện có sai đi giữa chừng cũng không cắt
  ngang lượt đang chạy. Muốn dừng ngay tại một dòng nằm giữa thân thì cần một
  công cụ riêng, và bạn sẽ gặp nó ở phía trước.
- **Sai ngay lần đọc đầu thì vòng chạy 0 lượt.** Không dòng nào trong thân được
  chạy, không lỗi nào được báo, chương trình đi thẳng xuống dòng nằm sau vòng
  lặp. Nhiều chương trình dựa hẳn vào chuyện này: đoạn hỏi lại khách ở bài
  trước chạy 0 lượt khi khách gõ đúng ngay lần đầu — và đó chính là điều bạn
  mong muốn.

Ở chỗ này `while` giống `if` hơn bạn tưởng: cả hai đều đọc điều kiện trước rồi
mới quyết định có vào thân hay không. Một vòng `while` sai điều kiện ngay từ
đầu thì cư xử y hệt một `if` sai điều kiện — không làm gì cả. Khác biệt giữa
chúng vẫn nằm đúng chỗ cũ: `while` quay lại đọc lần nữa, `if` thì không.

> Chỗ dễ vấp: đặt dòng báo cáo ở **đầu** thân hay ở **cuối** thân cho ra hai
> màn hình khác nhau. Ở đầu thân, con số in ra là con số *trước* khi lượt này
> sửa nó; ở cuối thân là con số *sau* khi sửa. Cả hai cách đều chạy được, không
> cách nào báo lỗi — nên đây là loại sai máy không nhắc bạn, bạn phải tự đọc
> lại thứ tự các dòng trong thân.
::::

::::code{#bao-lai-sau-moi-to}
Byte cầm 100 nghìn đi ăn phở, mỗi tô 45 nghìn, và ăn chừng nào còn đủ tiền cho
một tô nữa.

Byte muốn sau **mỗi** tô màn hình báo lại số tiền còn trong ví — kể cả tô cuối,
lúc số tiền còn lại đã không đủ mua thêm tô nào.

Hãy điền dòng còn thiếu ở cuối thân vòng.

```python title=starter
tien_con = 100000

while tien_con >= 45000:
    tien_con = tien_con - 45000
    ___

print("Về nhà")
```

```python title=solution
tien_con = 100000

while tien_con >= 45000:
    tien_con = tien_con - 45000
    print(f"Còn {tien_con} đồng")

print("Về nhà")
```

```python title=test
# Dòng bạn điền chỉ được BÁO số tiền, không được sửa nó: sau lượt cuối ví
# phải còn đúng 10 nghìn — không đủ một tô nữa nên vòng dừng ở đó.
assert tien_con == 10000, "một trăm nghìn ăn được hai tô phở 45 nghìn, ví còn đúng 10 nghìn — dòng báo cáo chỉ nói lại số tiền đang có chứ không được tiêu thêm đồng nào"
```

:::hints
- kind: attention
  body: Dòng cần điền nằm trong thân vòng, ngay sau dòng trừ tiền. Ở chỗ đó `tien_con` đã mang con số mới của lượt này rồi.
- kind: strategy
  body: Câu cần in có kèm một con số đang nằm trong một cái tên, nên dùng đúng lối in kèm giá trị bạn đã quen. Một trăm nghìn mua được hai tô, nên đúng một dòng ấy phải in ra hai con số khác nhau — 55000 sau tô đầu, rồi 10000 sau tô thứ hai. Ở lượt cuối điều kiện đã sai, nhưng thân vẫn chạy nốt dòng này.
- kind: one-line
  body: 'Viết `print(f"Còn {tien_con} đồng")` vào chỗ trống, thụt vào bốn dấu cách cho thẳng hàng với dòng ngay trên nó.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Còn 55000 đồng\nCòn 10000 đồng\nVề nhà\s*$
- tier: output
  expect: Còn 10000 đồng
- tier: static
  onFail: dòng bạn điền phải BÁO con số đang có trong ví, không in ra một chuỗi cố định
  requireAst:
  # `min: 2` vì khung đã có sẵn MỘT lệnh print ("Về nhà") — dòng bạn điền phải
  # là cái thứ hai. Hỏi `uses-name: tien_con` không được: khung đã nhắc tên ấy
  # hai lần ở dòng `while` và dòng trừ tiền, nên luật thoả bất kể điền gì.
  - kind: uses-call, target: print, min: 2
  - kind: uses-name, target: tien_con, min: 3
  forbidAst:
  - kind: has-literal, target: 10000
  - kind: has-literal, target: 55000
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lượt cuối vẫn chạy trọn. Mình đã cho vào thân thì mình làm cho xong.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ bạn biết chính xác máy đọc điều kiện ở đâu: đúng một lần, ở đầu mỗi lượt.
Hãy lấy chính điều đó soi ngược lại một chuyện.

Trong mọi đoạn code hai bài vừa rồi, thân vòng luôn có một dòng chạm tới cái
tên nằm trong điều kiện — nồi vơi một tô, ví bớt một tô phở, câu trả lời được
hỏi lại. Nhờ vậy mỗi lần đọc lại, máy nhìn thấy một con số khác con số lần
trước.

Bây giờ hình dung một vòng `while` mà **suốt thân không có dòng nào** động tới
thứ nằm trong điều kiện. Chẳng hạn `to_con_lai = 3`, còn thân chỉ có mỗi dòng
`print("Múc một tô cho khách")`.

Đầu lượt sau, máy đọc lại điều kiện. Nó đọc được gì? Rồi sau đó thì sao?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
