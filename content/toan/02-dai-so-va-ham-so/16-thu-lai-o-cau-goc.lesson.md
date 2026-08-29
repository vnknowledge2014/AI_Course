---
id: toan.dai-so-va-ham-so.thu-lai-o-cau-goc
title: Thử lại ở câu gốc
summary: Một con số gỡ ra được chưa chắc đúng — và chỉ có câu ban đầu mới bắt được nó sai, vì mọi dòng viết ra SAU chỗ hỏng đều đã ngả theo cái sai ấy.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.check-solution]
requires: [math.substitution, math.equation, math.solution-set, math.equation-add-both-sides, math.letter-names-a-slot, math.multiplication, math.order-of-operations, core.variable, core.assignment, core.arithmetic, core.number-literal, core.print-variable, core.output]
concepts: [math.hai-ve, math.giu-nghiem, math.chuoi-bien-doi]
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
  reviewed: true
---

::::byte{trigger=enter mood=curious pose=lean-in}
Mình gỡ ra một con số. Nhưng lấy gì bảo đảm nó là con số đúng?
::::

::::explain{#cau-hoi-con-treo}
Bài trước để lại một câu hỏi. Chuỗi biến đổi để tách ô trống ra đứng một mình
dài bốn năm dòng. Lỡ sai một dòng ở giữa, bạn vẫn ra một con số — và con số ấy
trông y hệt một đáp án đúng: cũng là một số, cũng nằm ở dòng cuối, cũng đứng
sau chữ `n` và dấu `=`.

Muốn biết nó đúng hay sai thì phải **hỏi lại một thứ gì đó**. Câu hỏi của bài
này là: hỏi lại thứ nào?

Có hai ứng viên, và chúng không tương đương nhau chút nào:

- **Dòng bạn vừa viết ra** — dòng áp chót, ngay trên đáp số.
- **Câu gốc** — cái câu quán đưa cho bạn, trước khi bạn động vào nó.

Nghe thì ứng viên thứ nhất tiện hơn: nó ngắn hơn, gần hơn, đỡ phải lật lên đầu
trang. Nhưng nó có một tật mà bạn không nhìn thấy được từ bên trong.

Mỗi dòng bạn viết ra là **hệ quả của dòng ngay trên nó**. Nếu dòng thứ hai đã
lệch thì dòng thứ ba lệch theo, dòng thứ tư lệch theo, và đáp số cuối cùng khớp
với chúng một cách hoàn hảo. Cả đám dòng ấy **đồng lòng với nhau** — vì chúng
cùng sinh ra từ một chỗ hỏng.

Câu gốc là thứ duy nhất **đứng ngoài chuỗi ấy**. Nó có mặt trước khi bạn cầm
bút, nên không nét bút nào của bạn chạm vào nó được.
::::

::::example{#byte-go-nham}
Chiều nay ở xe bánh mì, câu hỏi giống hôm qua nhưng số tiền khác: mỗi ổ bán
**15 000 đồng**, mỗi buổi trả **30 000 đồng** tiền thuê chỗ, dọn hàng về trong
túi còn **270 000 đồng**. Bán được mấy ổ?

```text
   câu gốc       15000 × n − 30000 = 270000
                 ↓  Byte "chuyển số 30 000 sang đĩa phải"
   dòng giữa             15000 × n = 270000 − 30000 = 240000
   dòng cuối                     n = 16
```

Byte ra **16 ổ**. Byte thử lại — vào dòng ngay trên đáp số:

> `15000 × 16 = 240000`, mà vế phải cũng là `240000`. Khớp.

Khớp thật. Nhưng 16 là con số **sai**.

Chỗ hỏng nằm ở mũi tên. Muốn cái `− 30000` biến mất khỏi đĩa trái thì phải
**cộng** 30 000 vào đĩa trái — và luật hai đĩa bắt cộng luôn 30 000 vào đĩa
phải, ra `15000 × n = 300000`. Byte thì xoá `− 30000` ở đĩa trái (tức là cộng)
mà lại **trừ** 30 000 ở đĩa phải. Hai đĩa bị đối xử khác nhau, nên từ dòng ấy
trở xuống không còn là phương trình cũ nữa.

Và đây là chỗ đáng nhớ: **dòng giữa vẫn gật đầu với 16.** Nó phải gật, vì 16
chính là con số sinh ra từ nó. Hỏi một dòng bị hỏng xem đáp số có đúng không
thì cũng như hỏi người vừa chép sai xem mình chép có đúng không.

Bây giờ hỏi câu gốc — cái câu Byte chưa hề động vào:

```python title=readonly
n_byte = 16
print(15000 * n_byte - 30000)   # vế trái của CÂU GỐC
print(270000)                   # vế phải của CÂU GỐC
```

Máy in ra:

```text
210000
270000
```

`210000` không phải `270000`. Câu gốc bác bỏ ngay, dù dòng giữa vừa gật đầu
xong.
::::

::::predict{#doan-hai-dong commitOnce}
Byte muốn nhìn hai con số cạnh nhau: vế trái của **dòng giữa**, rồi vế trái của
**câu gốc**, cùng thay `n` bằng 16.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
n_byte = 16
print(15000 * n_byte)           # vế trái của dòng giữa:  15000 × n
print(15000 * n_byte - 30000)   # vế trái của câu gốc:    15000 × n − 30000
```

:::opt{correct}
240000 rồi 210000
:::

:::opt
240000 rồi 270000
::why
Gần đúng ở chỗ bạn đang cầm đúng cái luật của bài này: khi `n` là con số đúng
thì hai vế của câu gốc phải ra **cùng một số**, và `270000` đúng là vế phải của
câu gốc.

Chỗ lệch là phạm vi của luật ấy. "Hai vế bằng nhau" là **kết luận** của phép
thử, không phải nguyên liệu để làm phép thử. Ranh giới nằm đúng ở đây: phải
tính vế trái một mình, bằng chính con số đang bị nghi ngờ, rồi mới được đem so
với vế phải. Cho phép mình dùng trước cái kết luận thì mọi con số đều qua, kể
cả số sai — và phép thử mất sạch tác dụng.
::
:::

:::opt
240000 rồi 240000
::why
Gần đúng ở chỗ bạn thấy hai dòng bắt đầu bằng đúng một cụm `15000 * n_byte`,
nên phần đầu của chúng giống hệt nhau. Quan sát ấy chính xác.

Chỗ lệch: dòng thứ hai còn `− 30000` đứng phía sau, và cái `− 30000` ấy **chính
là** chỗ khác nhau duy nhất giữa câu gốc và dòng giữa. Nó cũng chính là chỗ
Byte gỡ hỏng. Bỏ qua nó thì hai dòng in ra giống nhau thật — nhưng lúc đó bạn
không còn thử vào câu gốc nữa, bạn đang thử vào dòng giữa lần thứ hai.
::
:::

:::opt
240000 rồi 239970
::why
Gần đúng ở chỗ bạn trừ đúng phép và trừ đúng chỗ: cái phải bớt đi là tiền thuê
chỗ, và nó phải bớt khỏi tiền thu. Cả câu chuyện bạn đọc trúng.

Chỗ lệch là **đơn vị**. Quy tắc "khoản 30 nghìn thì viết số 30" đúng khi cả câu
tính đang tính bằng nghìn đồng — và nhiều bài toán làm thế thật. Nhưng ở đây
`15000` đã viết bằng đồng, nên 30 nghìn phải viết là `30000` cho cùng thước.
Ranh giới: trong một câu tính, mọi số phải đo bằng **một** đơn vị; trộn hai
đơn vị vào là kiểu sai êm nhất, vì kết quả vẫn ra một con số trông rất hợp lý.
::
:::
::::

::::explain{#dat-ten}
Đặt tên cho thứ vừa làm, để mang đi được:

> **Kiểm nghiệm** — thay con số vừa tìm được vào **câu gốc**, tính riêng từng
> vế. Hai vế ra cùng một số thì con số ấy là nghiệm; ra hai số khác nhau thì nó
> không phải.

Ba điều đi kèm cái tên ấy, và điều nào cũng đáng cầm theo:

- **Thay vào *mọi* chỗ có chữ.** Câu gốc có mấy chữ `n` thì cả mấy chỗ đều nhận
  cùng một con số — vì mọi chỗ mang cùng một chữ là cùng một ô trống. Bỏ sót
  một chỗ là đang thử một câu khác.
- **Tính hai vế riêng rẽ, chưa được nhìn nhau.** Cứ tính vế trái ra một số, vế
  phải ra một số, xong xuôi mới đem hai số ấy đặt cạnh nhau. Đây là chỗ phép
  thử lấy được sức mạnh của nó.
- **Kiểm nghiệm không sửa bài hộ bạn.** Nó chỉ nói *đúng* hoặc *không đúng*. Nó
  không chỉ ra dòng nào hỏng — muốn biết thì vẫn phải dò lại chuỗi.

Và một chỗ dễ hiểu nhầm: kiểm nghiệm **không** đòi bạn tin rằng chuỗi biến đổi
của mình đúng. Nó cố tình không dùng gì trong chuỗi ấy. Đó là lý do nó bắt được
lỗi mà chính người viết ra chuỗi không nhìn thấy.
::::

::::code{#thu-lai-hai-ung-vien}
Hai người cùng gỡ câu gốc `15000 × n − 30000 = 270000` của quán, ra hai con số
khác nhau: Byte ra **16**, người kia ra **20**. Không ai chịu ai.

Đừng đọc lại chuỗi của ai cả. Hỏi thẳng câu gốc.

Ba chỗ trống dưới đây là ba lần tính, mỗi lần một vế trái:

- vế trái của **dòng giữa** (`15000 × n`) với con số của Byte,
- vế trái của **câu gốc** (`15000 × n − 30000`) với con số của Byte,
- vế trái của **câu gốc** với con số của người kia.

Ba lần ấy cố tình cho ra ba số khác nhau, nên một con số gõ cứng vào cả ba chỗ
thì nhiều nhất chỉ đúng được một dòng.

```python title=starter
# Câu GỐC của quán:           15000 × n − 30000 = 270000
# Dòng GIỮA mà Byte viết ra:  15000 × n         = 240000

n_byte = 16    # con số Byte gỡ ra
n_khac = 20    # con số người kia gỡ ra

# Vế trái của DÒNG GIỮA, thay n bằng con số của Byte
giua_trai = ___

# Vế trái của CÂU GỐC, thay n bằng con số của Byte
goc_trai_byte = ___

# Vế trái của CÂU GỐC, thay n bằng con số của người kia
goc_trai_khac = ___

print(giua_trai)
print(goc_trai_byte)
print(goc_trai_khac)
```

```python title=solution
# Câu GỐC của quán:           15000 × n − 30000 = 270000
# Dòng GIỮA mà Byte viết ra:  15000 × n         = 240000

n_byte = 16    # con số Byte gỡ ra
n_khac = 20    # con số người kia gỡ ra

# Vế trái của DÒNG GIỮA, thay n bằng con số của Byte
giua_trai = 15000 * n_byte

# Vế trái của CÂU GỐC, thay n bằng con số của Byte
goc_trai_byte = 15000 * n_byte - 30000

# Vế trái của CÂU GỐC, thay n bằng con số của người kia
goc_trai_khac = 15000 * n_khac - 30000

print(giua_trai)
print(goc_trai_byte)
print(goc_trai_khac)
```

```python title=test
# Hai câu `!=` đứng trước, vì chúng canh đúng cái bẫy của bài: dòng giữa gật
# đầu với một con số mà câu gốc bác bỏ. Xếp chúng sau thì chúng không bao giờ
# chạy tới, và cái bẫy không bao giờ sập.
assert goc_trai_byte != 270000, "thay 16 vào CÂU GỐC thì vế trái không ra 270000 — chính chỗ này bắt được con số sai"
assert giua_trai != goc_trai_byte, "cùng một con số 16 mà hai câu cho hai kết quả khác nhau, vì câu gốc còn `− 30000`"
assert giua_trai == 240000, "15000 × 16 = 240000 — dòng giữa khớp với 16, nên nó không tố giác được gì"
assert goc_trai_byte == 210000, "15000 × 16 − 30000 = 210000, mà vế phải câu gốc là 270000 — 16 không phải nghiệm"
assert goc_trai_khac == 270000, "15000 × 20 − 30000 = 270000, đúng bằng vế phải — 20 mới là nghiệm của câu gốc"
```

:::hints
- kind: attention
  body: Hai dòng chú thích trên cùng đã viết sẵn hai câu tính. Mỗi chỗ trống chỉ cần chép lại **vế trái** của một trong hai câu ấy, rồi thay chữ `n` bằng cái tên đứng ngay phía trên. Chú ý câu gốc có `− 30000` còn dòng giữa thì không.
- kind: strategy
  body: Thay giá trị nghĩa là viết lại đúng cái vế trái ấy, chỗ nào có `n` thì đặt tên vào chỗ đó — đừng tự tính nhẩm rồi gõ con số kết quả, vì con số kết quả chính là thứ bạn đang nhờ máy tìm hộ. Chỗ trống thứ nhất không có phép trừ; hai chỗ sau đều có.
- kind: one-line
  body: "Ba chỗ trống lần lượt là `15000 * n_byte`, `15000 * n_byte - 30000` và `15000 * n_khac - 30000`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là vế trái viết lại bằng CÁI TÊN giữ con số cần thử, không phải kết quả đã tính sẵn — tự tính hộ máy thì phép kiểm nghiệm không còn kiểm gì cả
  requireAst:
  # Ba chỗ trống, ba phép nhân với 15000. Khung khởi đầu không có dấu `*` nào,
  # nên luật này chặn được đúng cái đáp án chép cứng ba con số.
  - kind: uses-operator, target: *, min: 3
  # Hai chỗ trống sau là VẾ TRÁI CÂU GỐC, và câu gốc có `− 30000`. Thiếu phép
  # trừ nghĩa là đang thử vào dòng giữa lần thứ hai — đúng cái sai bài này nói.
  - kind: uses-operator, target: -, min: 2
  # Con số cần thử phải được ĐỌC ra từ cái tên, hai lần: một lần cho dòng giữa,
  # một lần cho câu gốc. Gõ thẳng `15000 * 16` cũng ra số đúng, nhưng lúc đó
  # người viết đã tự thay giá trị bằng tay và máy không thay gì cả.
  - kind: uses-name, target: n_byte, min: 2
  - kind: uses-name, target: n_khac, min: 1
  forbidAst:
  # Lưới thứ hai: ba con số KẾT QUẢ. Lời giải thật dựng chúng từ phép tính nên
  # không chứa nguyên văn cái nào, còn đáp án chép cứng thì chứa đủ cả ba.
  - kind: has-literal, target: 240000
  - kind: has-literal, target: 210000
  - kind: has-literal, target: 270000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^240000\n210000\n270000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Câu gốc bắt được cái sai mà dòng giữa vừa gật đầu cho qua. Mình nhớ rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại mọi câu bạn đã gỡ từ bài 10 tới giờ. Chúng có một điểm chung mà chưa
ai nói ra: **chữ `n` chỉ đứng ở một đĩa.** Đĩa kia lúc nào cũng là một con số
trần — 270 000, 195 000, 300 000. Cả cách gỡ lẫn cách thử lại đều dựa vào
chuyện đó.

Nhưng quán không hứa sẽ luôn như vậy. Tới giờ mình vẫn coi 30 000 đồng thuê chỗ
là khoản duy nhất phải chi. Thật ra mỗi ổ bánh mì còn tốn tiền bột, tiền thịt,
tiền than — và khoản ấy **đi theo số ổ**: bán càng nhiều thì tốn càng nhiều.

Viết ra thì đĩa phải không còn là một con số trần nữa. Nó cũng có chữ `n` bên
trong.

Và câu hỏi của quán cũng đổi theo. Bài này hỏi *"cuối buổi còn bao nhiêu tiền
trong túi"* — một câu tính ra số. Bài sau hỏi câu khác hẳn: *"bán bao nhiêu ổ
thì tiền thu vừa đúng tiền chi"* — tức hai đĩa mang chữ đứng cân nhau.

Lúc ấy hai đĩa đều mang ô trống, và cách gỡ cũ đuổi vòng quanh: dọn xong đĩa
trái thì đĩa phải vẫn còn chữ, dọn xong đĩa phải thì đĩa trái vẫn còn chữ.

Vậy gỡ từ đâu? Cụm mang chữ ở đĩa phải có được tính là một "lượng" mà luật hai
đĩa cho phép bớt đi không — trong khi chưa ai biết nó bằng bao nhiêu?

Bài sau nhận đúng câu hỏi này.
::::

::::checkpoint{mastery=0.8}
::::
