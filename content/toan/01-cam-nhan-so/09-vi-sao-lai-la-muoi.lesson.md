---
id: toan.cam-nhan-so.vi-sao-lai-la-muoi
title: Vì sao lại là mười
summary: Con số mười không nằm trong đống hạt — nó nằm ở bàn tay người đếm. Bó theo năm hay theo hai thì đống vẫn nguyên chừng ấy, chỉ tờ giấy là đổi.
locale: vi
track: toan
module: cam-nhan-so
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.number-base]
requires: [math.zero-placeholder, math.place-value, math.dong-goi, core.bit, core.arithmetic, core.number-literal, core.variable, core.assignment, core.print-variable, core.output]
concepts: [math.co-so, math.dong-goi, math.bang-cot]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Mình lật đi lật lại đống hạt mà chẳng thấy con số mười nào nằm trong đó cả.
::::

::::explain{#tim-con-so-muoi-trong-dong-hat}
Ba bài vừa rồi đứng trên đúng một luật: **đủ mười thì lên bó**. Mười hạt thành
một bó (bài 6), mười bó thành một bó-của-bó, mỗi cột chỉ chứa tới chín (bài 7),
và cột nào rỗng thì cắm `0` vào giữ chỗ (bài 8).

Bài trước hỏi: vì sao lại là mười?

Cách trả lời chắc nhất là đi tìm con số mười ấy. Nếu nó là **bản chất của số**
thì nó phải nằm đâu đó trong đống hạt, và ta chỉ tay vào được.

Đổ đống hạt của Byte ra bàn — 12 hạt, đúng đống của bài 1. Trong đó có gì? Có
hạt. Hạt nào cũng như hạt nào, không hạt nào đeo số, không hạt nào đứng ra làm
trưởng nhóm mười. Đống hạt chỉ nói được đúng một điều: *có chừng này hạt*. Nó
chưa bao giờ nói phải gom thành từng nhóm mấy.

Vậy con số mười không ở trong đống. Nó ở chỗ khác — ở **bàn tay người đếm**.
Mười ngón tay, đếm hết một lượt thì hết chỗ, phải bắt đầu lại và nhớ rằng "vừa
xong một lượt". Cái lượt ấy chính là cái bó. Tiếng Việt còn giữ nguyên dấu vết:
một **chục** là đúng một lượt của hai bàn tay.

Nói lại cho gọn, và đây là điều bài này thêm vào:

> Cỡ bó — gọi là **cơ số** — là một **lựa chọn**, không phải một tính chất của
> đống hạt.

Bài 6 đã chọn nó rồi, chỉ là lúc ấy chọn mà không ai nói ra là đang chọn.
::::

::::explain{#bo-theo-nam-thu-xem}
Đã là lựa chọn thì chọn khác được. Vẫn đống 12 hạt ấy, lần này gom **mỗi bó năm
hạt**.

Buộc lần một: hết 5 hạt. Buộc lần hai: hết 10 hạt. Còn 2 hạt trên bàn, không đủ
buộc bó thứ ba. Vậy: **2 bó và 2 hạt lẻ**.

Xếp vào cột như bài 7, chỉ khác một chỗ — cột bên trái giờ to gấp **năm** cột
bên phải, chứ không gấp mười:

| bó (năm hạt) | hạt lẻ |
|---|---|
| 2 | 2 |

Viết liền: `22`.

Bây giờ đặt hai tờ giấy cạnh nhau và nhìn cho kỹ.

| cách bó | cột trái | cột phải | tờ giấy |
|---|---|---|---|
| bó theo mười | 1 bó | 2 hạt lẻ | `12` |
| bó theo năm | 2 bó | 2 hạt lẻ | `22` |

Hai tờ giấy khác nhau. **Đống hạt thì không đổi một hạt nào** — vẫn đúng chừng
ấy hạt nằm trên bàn, chưa ai thêm vào cũng chưa ai bốc ra. Thứ vừa đổi là cách
ta *ghi lại* nó.

Và một chuyện đi kèm, đáng nhớ hơn cả hai tờ giấy: bó theo năm thì cột hạt lẻ
chỉ chứa tới **bốn**. Có 5 hạt lẻ nghĩa là buộc thiếu một bó. Nên cơ số cũng
quyết định luôn **có bao nhiêu chữ số để dùng**: bó theo mười thì dùng 0–9, bó
theo năm thì dùng 0–4.

Đẩy tới cùng: bó theo **hai**. Lúc ấy mỗi cột chỉ còn hai câu trả lời — rỗng
hoặc đầy, `0` hoặc `1`. Cột nào cũng to gấp đôi cột bên phải, nên các cột là 1,
2, 4, 8, 16…

| 8 | 4 | 2 | 1 |
|---|---|---|---|
| 1 | 1 | 0 | 0 |

Tám cộng bốn là mười hai. Tờ giấy ghi `1100`, và nó nói về đúng đống hạt lúc
nãy.

Chỗ này Realm 0 bài 5 đã dựng sẵn nửa kia: trong máy chỉ có những cái ô hai
trạng thái, bật hoặc tắt. Cái ô ấy không đủ chỗ cho chữ số 7, nên máy **buộc**
phải bó theo hai. Không phải máy giỏi hơn hay dở hơn — nó chỉ có hai ngón tay.
Đó cũng là chỗ Byte lấy tên.
::::

::::example{#mot-dong-hai-to-giay}
Bắt máy dựng lại đống hạt từ hai tờ giấy, để xem chúng có gặp nhau không.

```python title=readonly
# Tờ giấy "12" — bó theo MƯỜI: 1 bó mười hạt, 2 hạt lẻ
print(10 + 2)

# Tờ giấy "22" — bó theo NĂM: 2 bó năm hạt, 2 hạt lẻ
print(5 + 5 + 2)
```

Máy in ra:

```text
12
12
```

Hai tờ giấy khác nhau, một đống hạt.

Để ý máy đang làm gì: nó chỉ cộng — mỗi cái bó đổ ra thành chừng ấy hạt, rồi
gộp hết lại. Nó chưa hề nghe tới chữ "cơ số", và cũng không cần. Cỡ bó nằm
trong con số `10` hay con số `5` mà **bạn** gõ vào —
tức là nằm ở người viết, đúng như con số mười nằm ở bàn tay người đếm.

Và đây là chỗ phải cẩn thận từ nay: một dãy chữ số trần, đứng một mình, chưa đủ
để biết nó là đống bao nhiêu hạt. `22` là hai mươi hai hạt nếu bó theo mười, và
là mười hai hạt nếu bó theo năm. Con số muốn nói được điều gì thì phải kèm cỡ
bó — y hệt cách bài 1 nói con số phải kèm đơn vị.
::::

::::predict{#doan-ba-to-giay commitOnce}
Đống của An có nhiều hơn: **23 hạt**. An viết ra ba tờ giấy cho cùng đống ấy.

- Bó theo **mười**: 2 bó, 3 hạt lẻ — tờ giấy `23`.
- Bó theo **năm**: 4 bó, 3 hạt lẻ — tờ giấy `43`.
- Bó theo **hai**: cột 16 và cột 4 và cột 2 và cột 1 đều đầy, cột 8 rỗng — tờ
  giấy `10111`.

Ba dòng dưới đây dựng lại đống hạt từ ba tờ giấy ấy.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(10 + 10 + 3)
print(5 + 5 + 5 + 5 + 3)
print(16 + 4 + 2 + 1)
```

:::opt{correct}
23, 23 rồi 23
:::

:::opt
23, 43 rồi 10111
::why
Gần đúng ở chỗ bạn nhớ chắc ba tờ giấy của An, và bạn chép lại chúng không sai
một chữ số nào. Quy tắc bạn đang dùng — *dãy chữ số viết ra thế nào thì đọc lên
thế ấy* — là quy tắc đúng, và cả bài 7 lẫn bài 8 đều đứng trên nó.

Ranh giới của nó là: nó đúng khi người viết và người đọc **bó theo cùng một
cỡ**. Bỏ ngầm định đó đi thì `43` không còn một cách đọc duy nhất nữa.

Còn một chỗ lệch nhỏ hơn nhưng đáng để ý: ba dòng này không in tờ giấy ra. Chúng
làm việc ngược lại — cầm từng cột đổi về hạt rồi gộp, tức là **dựng lại
đống hạt**. Thứ đi ra là số hạt, không phải dãy chữ số.
::
:::

:::opt
23, 20 rồi 23
::why
Gần đúng ở chỗ bạn giữ đúng tinh thần của bài 6: buộc bó xong thì đi đếm bó, chứ
không đếm lại từng hạt. Đó là toàn bộ lợi ích của việc đóng gói, và bạn không bỏ
sót nó.

Ranh giới nằm ở chỗ **hạt lẻ**. Bốn cái bó của An chứa 20 hạt — con số 20 bạn ra
là có thật. Nhưng 3 hạt lẻ kia theo định nghĩa là những hạt **chưa lọt vào bó
nào**: chúng còn nằm trên bàn, không nằm trong 20 hạt ấy. Bỏ chúng đi thì đống
hạt hụt mất ba hạt, và tờ giấy `43` chỉ còn được đọc có một nửa.
::
:::

:::opt
23 rồi máy báo lỗi ở dòng 2, vì bó theo năm không phải một cách đếm hợp lệ
::why
Gần đúng ở chỗ linh cảm của bạn có nguồn gốc thật: mọi con số bạn từng viết ra
đời đều bó theo mười, và bài 7 phát biểu hẳn thành luật — cột bên trái to gấp
mười cột bên phải. Luật ấy chưa bao giờ sai chỗ nào bạn gặp.

Ranh giới của nó là: nó là luật của **một** cách bó, không phải luật của số. Bài
7 dựng các cột lên từ cái bó mà bài 6 đã buộc, và cái bó ấy buộc mười hạt là do
người chọn. Chọn năm thì cột trái to gấp năm — luật vẫn nguyên hình dạng, chỉ
thay con số.

Còn về phía máy: ba dòng này không có chữ "cơ số" nào cả. Máy chỉ thấy mấy phép
cộng, và nó làm đúng chừng ấy việc.
::
:::
::::

::::byte{trigger=enter mood=curious pose=point-editor}
Ba tờ giấy, mà trên bàn chỉ có hai đống. Mình muốn biết mỗi tờ nói tới mấy hạt.
::::

::::code{#dung-lai-hai-dong-tu-to-giay}
Sáng nay Byte và An cùng buộc bó, cả hai cùng dùng **cỡ bó năm hạt**. Mỗi người
ghi lại kết quả bằng hai con số: mấy bó, và mấy hạt lẻ.

- **Byte**: 2 bó và 2 hạt lẻ — tờ giấy ghi `22`.
- **An**: 4 bó và 3 hạt lẻ — tờ giấy ghi `43`.

Rồi Byte cởi bó của mình ra, buộc lại theo **cỡ bó mười hạt**, và được: 1 bó
mười hạt với 2 hạt lẻ — tờ giấy thứ ba, ghi `12`. Vẫn đống hạt ấy, không thêm
không bớt hạt nào.

Ba chỗ trống là **số hạt thật** mà mỗi tờ giấy nói tới. Đừng chép con số trên
tờ giấy xuống: tờ giấy chỉ ghi mấy bó với mấy lẻ, còn câu hỏi hỏi có mấy hạt.

Chỗ trống thứ hai là chỗ đáng nhìn nhất: nó và chỗ trống thứ nhất nói về **cùng
một đống trên bàn**, chỉ khác cỡ bó — nên hai con số ấy phải bằng nhau, dù hai
tờ giấy ghi khác hẳn nhau.

```python title=starter
# Byte bó theo NĂM: 2 bó và 2 hạt lẻ — tờ giấy ghi "22".
le_byte = 2
dong_byte = ___

# Vẫn đống ấy của Byte, buộc lại theo MƯỜI: 1 bó mười hạt, 2 hạt lẻ — tờ giấy ghi "12".
dong_byte_bo_muoi = ___

# An bó theo NĂM: 4 bó và 3 hạt lẻ — tờ giấy ghi "43".
le_an = 3
dong_an = ___

print(dong_byte)
print(dong_an)
print(dong_byte_bo_muoi)
```

```python title=solution
# Byte bó theo NĂM: 2 bó và 2 hạt lẻ — tờ giấy ghi "22".
le_byte = 2
dong_byte = 5 + 5 + le_byte

# Vẫn đống ấy của Byte, buộc lại theo MƯỜI: 1 bó mười hạt, 2 hạt lẻ — tờ giấy ghi "12".
dong_byte_bo_muoi = 10 + 2

# An bó theo NĂM: 4 bó và 3 hạt lẻ — tờ giấy ghi "43".
le_an = 3
dong_an = 5 + 5 + 5 + 5 + le_an

print(dong_byte)
print(dong_an)
print(dong_byte_bo_muoi)
```

```python title=test
# Năm câu, hai đống khác nhau. Hai câu cuối cùng nói lại chính điều bài dạy:
# vẫn đống ấy trên bàn, đổi cỡ bó thì tờ giấy đổi mà số hạt không đổi.
assert dong_byte == 12, "2 bó năm hạt là 10 hạt, thêm 2 hạt lẻ nữa là 12 hạt"
assert dong_an == 23, "4 bó năm hạt là 20 hạt, thêm 3 hạt lẻ nữa là 23 hạt"
assert dong_byte_bo_muoi == 12, "1 bó mười hạt là 10 hạt, thêm 2 hạt lẻ nữa là 12 hạt"
assert dong_byte_bo_muoi == dong_byte, "đổi cỡ bó thì tờ giấy đổi (22 thành 12), còn đống hạt trên bàn thì không nhúc nhích"
# Câu chốt: chép thẳng tờ giấy xuống là hỏng, vì tờ giấy chưa nói cỡ bó.
assert dong_byte != 22, "tờ giấy ghi 22 nhưng đống chỉ có 12 hạt — dãy chữ số phải kèm cỡ bó mới đọc được"
```

:::hints
- kind: attention
  body: Mỗi chỗ trống hỏi SỐ HẠT, không hỏi con số trên tờ giấy. Lời chú thích ngay trên nó cho biết đủ ba thứ cần: có mấy bó, mỗi bó mấy hạt, và mấy hạt lẻ chưa lọt vào bó nào.
- kind: strategy
  body: Một cái bó năm hạt đổ ra là 5 hạt, nên hai cái bó ấy đổ ra là 5 cộng 5. Hạt lẻ chưa lọt vào bó nào nên phải cộng thêm vào — dùng cái tên `le_byte` (và `le_an`) có sẵn, đừng gõ thẳng con số kết quả. Riêng chỗ trống giữa bó theo mười, nên một cái bó ở đó đổ ra 10 hạt.
- kind: one-line
  body: "Chỗ trống thứ nhất là `5 + 5 + le_byte`, chỗ thứ hai là `10 + 2`, chỗ thứ ba là `5 + 5 + 5 + 5 + le_an`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải đổ từng cái bó ra thành hạt rồi cộng hạt lẻ vào — chép con số trên tờ giấy hay gõ cứng kết quả thì bài không kiểm được gì
  requireAst:
  # Khung khởi đầu chưa có dấu cộng nào, nên luật này chặn đáp án gõ cứng cả ba
  # chỗ. `min: 3` chứ không phải 7, vì `10 + le_byte` cũng là một cách đổ bó
  # hợp lệ và phải được cho qua.
  - kind: uses-operator, target: +, min: 3
  - kind: uses-name, target: le_byte, min: 1
  - kind: uses-name, target: le_an, min: 1
  forbidAst:
  # `uses-operator` đếm trên CẢ FILE, nên hai luật dưới đây mới là thứ chặn
  # được đáp án gõ cứng ĐÚNG MỘT chỗ. Chúng không đụng tới cách đổ bó nào cả:
  # `5 + 5 + le_byte`, `10 + le_byte`, `10 + 2` hay `5 + 5 + 5 + 5 + le_an`
  # đều không chứa nguyên văn 12 hay 23.
  - kind: has-literal, target: 12
  - kind: has-literal, target: 23
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^12\n23\n12\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
12 hạt, 23 hạt — rồi lại 12 hạt. Tờ giấy đổi từ `22` sang `12` thật, mà trên
bàn chẳng hạt nào nhúc nhích.
::::

::::explain{#muoi-la-mot-thoi-quen-cua-loai-nguoi}
Chốt lại thành một câu mang theo cả track:

> **Đổi cơ số là đổi cách ghi, không đổi lượng.** Đống hạt không biết ta bó nó
> theo mấy.

Nếu mười chỉ là một thói quen, thì đáng lẽ phải còn dấu vết của những thói quen
khác. Có thật, và chúng nằm ngay trong nhà:

- Một **tá** trứng là 12 quả, không phải 10. Bó theo mười hai.
- Một giờ có **60** phút, một phút có **60** giây. Người Babylon bó theo sáu
  mươi, và bốn nghìn năm sau cái đồng hồ trên tường vẫn còn giữ.
- Một ngày có **24** giờ, một năm có **12** tháng.
- Trong máy tính thì bó theo **hai**, vì cái ô nhớ chỉ có bật với tắt.

Không cái nào trong số đó sai. Chúng chỉ là những cỡ bó khác, chọn cho những
việc khác — và mỗi lần đổi cỡ bó, dãy chữ số đổi theo còn lượng thì ở nguyên.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bó theo mười, theo năm hay theo hai — đống hạt vẫn nguyên chừng ấy. Chuyện đó
giờ đã chắc.

Nhưng vườn thì không đứng yên. Byte vừa đi một vòng hái thêm được một đống nữa,
mang về, và **đổ chung** vào đống cũ trên bàn.

Việc "đổ chung" ấy, trong toán gọi là gì? Nó phải có tên, vì Byte làm nó suốt
ngày.

Và câu hỏi thực dụng hơn, câu mà Byte đang lo thật: cả buổi sáng Byte đã buộc bó
xong xuôi, đã xếp cột, đã ghi ra giấy. Đổ thêm một đống vào thì mấy cái bó ấy
còn dùng được không, hay phải cởi hết ra rồi đếm lại từ hạt thứ nhất?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
