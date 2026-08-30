---
id: toan.cam-nhan-so.be-nho-cai-thuoc
title: Bẻ nhỏ cái thước
summary: Khi cái thước không với tới phần thừa thì bẻ chính nó thành mấy phần bằng nhau — một phần trong đó là một đơn vị mới, và tên nó là một phần mấy.
locale: vi
track: toan
module: cam-nhan-so
order: 30
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.unit-fraction]
requires: [math.remainder, math.don-vi-roi-va-lien, core.arithmetic, core.boolean, ctrl.comparison]
concepts: [math.don-vi, math.phan-so, math.thanh-so]
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
Thước không với tới cái mẩu này. Nên mình bẻ luôn cái thước.
::::

::::explain{#thuoc-khong-voi-toi}
Byte đang cầm cái mẩu dây 1 mét, và cả buổi sáng Byte đo bằng đúng một đơn vị:
**đoạn 3 mét**. Sổ ghi 4 đoạn. Còn cái mẩu này ghi là mấy đoạn?

Có hai lối đi, và một lối là ngõ cụt.

Lối thứ nhất: coi như không có. Nhưng cái mẩu ấy sờ được, cầm được, buộc được
vào cọc rào — bỏ nó đi thì sổ sách không còn khớp với cái vườn nữa.

Lối thứ hai: đổi thước. Bài 3 đã dựng sẵn ý này rồi — đổi thước thì **con số
đổi, còn lượng đất thì không đổi tí nào**. Cái mẩu dây vẫn y nguyên chừng ấy;
chỉ là cái thước 3 mét quá thô để nói về nó.

Đổi sang thước nào? Đừng đi tìm ở đâu xa. Lấy chính cái thước đang cầm mà **bẻ
thành mấy phần bằng nhau**.

Bẻ đoạn 3 mét làm 3 phần bằng nhau: mỗi phần dài 1 mét. Và cái mẩu trên tay Byte
đúng bằng **một phần** trong ba phần ấy.

Bây giờ ghi được vào sổ. Cái mẩu ấy là **một phần ba** của đoạn, viết là `1/3`.
Con số dưới gạch nói *cái thước bị bẻ làm mấy phần*; con số trên gạch nói *lấy
mấy phần*. Sổ ghi: 4 đoạn và 1/3 đoạn.
::::

::::explain{#luat-cua-phep-be}
Không phải cứ chặt bừa cái thước ra là được `1/3`. Phép bẻ có đúng một luật, và
cả bài này đứng trên nó:

> Bẻ cái thước thành **mấy phần bằng nhau**. Ghép đủ chừng ấy phần lại thì phải
> dựng lại **đúng** cái thước cũ, không thừa không thiếu.

Đó là toàn bộ nghĩa của **một phần ba**, **một phần tư**, **một phần năm** — con
số dưới gạch chính là số phần vừa bẻ ra. Nếu Byte chặt đoạn 3 mét thành một mẩu
2 mét và một mẩu 1 mét thì có hai mẩu thật, nhưng không mẩu nào là "một phần hai"
cả — chúng không bằng nhau, nên ghép hai mẩu ấy lại chẳng nói lên luật gì.

Hai chỗ cần cẩn thận, vì đây là nơi người ta hay đi chệch:

**Một: con số dưới gạch không phải "cái bánh".** Nó nói cái **đơn vị** bị bẻ làm
mấy phần. Cùng con số 4 dưới gạch: `1/4` mét, `1/4` lon gạo, `1/4` giờ — ba lượng
khác hẳn nhau, vì ba cái thước khác nhau. Giống nhau ở chỗ mỗi cái đều là một
trong bốn phần bằng nhau của cái thước của mình.

**Hai: bẻ càng nhiều phần thì mỗi phần càng ngắn.** Đây không phải một luật thứ
hai — nó là hệ quả thẳng của luật vừa viết ở trên: cái thước có dài thêm đâu, mà
chia cho nhiều chỗ hơn thì mỗi chỗ được ít đi. Chuyện này đi ngược thói quen đếm
— 5 cái bó thì nhiều hơn 2 cái bó, nhưng bẻ làm 5 phần thì mỗi phần ngắn hơn bẻ
làm 2 phần. Con số dưới gạch không đếm phần bạn có; nó đếm **cái thước bị bẻ ra
làm mấy phần bằng nhau**. Nó cũng không đếm nhát bẻ: bẻ làm 4 phần thì chỉ cần
3 nhát cắt, mà con số dưới gạch vẫn là 4.

Và nhớ lại bài 4: chỉ thước **đo** mới bẻ được. Bẻ đôi sải dây thì nửa sải dây
vẫn là dây; bẻ đôi một hạt thì nửa hạt không còn là hạt nào cả.
::::

::::example{#hai-cach-be-mot-soi-day}
Vẽ chuyện lúc nãy ra cho thấy tận mắt. Cái thước là đoạn 3 mét; bẻ nó làm ba
phần bằng nhau; mẩu trên tay Byte đúng bằng một phần trong đó:

```text
thước (3 mét):  [=======================]
bẻ làm 3 phần:  [=======|=======|=======]
mẩu trên tay:   [=======]
```

Bài 29 nói cái mẩu ấy rơi vào **giữa vạch 0 và vạch 1** trên thanh số, mà ở đó
chưa có vạch nào để chỉ vào. Bẻ thước xong thì chỗ trống ấy đã có vạch:

```text
0                       1
├───────────────────────┤     cái thước cũ: đoạn 3 mét
├───────┼───────┼───────┤     bẻ nó làm ba phần bằng nhau
0      1/3     2/3      1     đo bằng cái thước mới
        ▲
        chỗ của cái mẩu trên tay Byte — bài 29 chỉ tay đúng vào đây
```

Bây giờ đem một sợi dây khác ra thử cái luật ấy, và đo bằng **phân** cho tiện —
sợi này dài đúng **1 mét**, tức 100 phân.

Byte bẻ sợi ấy làm **2** phần bằng nhau, mỗi phần 50 phân. An bẻ một sợi 1 mét
khác làm **5** phần bằng nhau, mỗi phần 20 phân.

Bắt máy kiểm luật của phép bẻ: ghép đủ số phần lại có dựng lại 100 phân không?

```python title=readonly
print(50 * 2)
print(20 * 5)
print(50 * 2 == 100)
```

Máy in ra:

```text
100
100
True
```

Hai cách bẻ đều hợp luật: gộp lại đều về đúng 1 mét. Nhưng hai cái phần thì
không giống nhau chút nào — một cái 50 phân, một cái 20 phân. Bẻ làm 5 thì mỗi
phần ngắn hơn bẻ làm 2, đúng như đã nói ở trên. Còn *ngắn hơn bao nhiêu* thì
không phải chuyện đoán: chỉ luật ghép-lại-khít mới chốt được con số.
::::

::::predict{#doan-be-muoi-phan-co-khit-khong commitOnce}
An bẻ thêm một sợi 1 mét nữa, lần này làm **10 phần** bằng nhau. An không đo,
An nhẩm: *"bẻ làm 5 phần đã được 20 phân rồi; làm 10 phần thì nhiều gấp đôi số
phần, chắc mỗi phần ngắn lắm — chừng 5 phân."*

Byte không tranh cãi. Byte đem đúng cái luật của phép bẻ ra hỏi máy: ghép đủ
mười phần ấy lại có dựng lại nổi sợi dây 100 phân không?

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
mot_phan_muoi_theo_an = 5
print(mot_phan_muoi_theo_an * 10 == 100)
```

:::opt{correct}
False
:::

:::opt
True
::why
Gần đúng ở chỗ bạn đang dùng đúng cái luật vừa đọc ở trên: bẻ càng nhiều phần
thì mỗi phần càng ngắn. Luật ấy chuẩn, và An cũng đi đúng **chiều** — 5 phân
thì ngắn hơn 20 phân thật.

Chỗ ra ngoài phạm vi: luật ấy chỉ nói chiều, nó không nói **ngắn bao nhiêu**.
Con số thì do luật ghép-lại-khít chốt, và ở đây nó chốt rất chặt: mười cái mẩu
5 phân ghép lại mới được 50 phân, thiếu mất nửa sợi dây. Nhiều gấp đôi số phần
thì mỗi phần đúng bằng **một nửa** — bẻ làm 10 thì mỗi phần phải là 10 phân,
chứ không phải ngắn bao nhiêu cũng được.
::
:::

:::opt
50
::why
Gần đúng ở chỗ bạn tính không sai một bước: `5 × 10 = 50`, và 50 phân đúng là
chiều dài gộp lại của mười cái mẩu An nói. Chính con số ấy làm lộ ra chỗ hỏng.

Chỗ ra ngoài phạm vi: dòng `print` không in phép nhân, nó in kết quả của dấu
`==`, mà `==` là một câu hỏi có–không (R0 bài 24) — thứ đi ra khỏi nó luôn là
`True` hoặc `False`. Muốn thấy 50 thì viết `print(mot_phan_muoi_theo_an * 10)`.
::
:::

:::opt
Không kết luận được, vì hai vế đo bằng hai cái thước khác nhau
::why
Gần đúng ở chỗ bạn dùng đúng luật bài 11: chỉ gộp và so được những thứ **cùng
một đơn vị**. Luật ấy rất đáng giữ, và ở bài 34 nó sẽ quay lại đúng lúc — khi
phải so `2/3` với `3/4`, hai con số ghi bằng hai cái thước khác nhau thật.

Chỗ ra ngoài phạm vi: ở dòng code này hai vế đã cùng một đơn vị rồi — vế trái là
mười cái mẩu 5 **phân** gộp lại, vế phải là cả sợi dây 100 **phân**. Việc quy về
cùng đơn vị đã làm xong trước khi máy nhìn thấy, nên còn lại chỉ là so hai con
số phân với nhau.
::
:::
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Bẻ đúng thì gộp lại vừa khít. Bẻ sai thì gộp lại hụt đi hoặc phình ra. Bạn kiểm
giúp mình nhé.
::::

::::code{#kiem-hai-cach-be}
Lấy lại đúng cái thước hôm qua: **đoạn 3 mét**, tức **300 phân**. Hôm nay hai
người cùng bẻ nó làm **4 phần bằng nhau**, nhưng nói ra hai con số khác nhau:

- **Byte** bảo mỗi phần dài **75 phân**.
- **An** bảo mỗi phần dài **80 phân**. An nhẩm 300 chia 4 không ra ngay, nên An
  làm tròn 300 lên **320** cho dễ chia: 32 chia 4 là 8, vậy mỗi phần 80 phân.

Làm tròn cho dễ nhẩm là một luật có thật và rất hay dùng — nhưng nó chỉ dùng
được khi bạn cần *ước chừng*. Bẻ thước thì không được ước chừng: làm tròn một
phát là bốn phần ghép lại không còn khít cái thước cũ nữa. Đó đúng là lý do luật
ở đầu bài đòi **không thừa không thiếu**.

Mỗi người in hai dòng: dòng trên là chiều dài gộp lại của cả bốn phần (đã viết
sẵn), dòng dưới là câu kiểm *"gộp lại có dựng đúng cái thước 300 phân không"*.
Hai chỗ trống là hai câu kiểm ấy.

Bài chấm bằng **cả hai** người, và hai người này cho ra hai câu trả lời ngược
nhau. Gõ cứng `True` vào cả hai chỗ thì phần của An lọt; gõ cứng `False` thì
phần của Byte trượt. Và chép thẳng con số cũng không qua: câu kiểm phải so **cái
tên** đang giữ chiều dài gộp lại với 300.

```python title=starter
gop_lai_cua_byte = 75 * 4
print(gop_lai_cua_byte)
print(___)

gop_lai_cua_an = 80 * 4
print(gop_lai_cua_an)
print(___)
```

```python title=solution
gop_lai_cua_byte = 75 * 4
print(gop_lai_cua_byte)
print(gop_lai_cua_byte == 300)

gop_lai_cua_an = 80 * 4
print(gop_lai_cua_an)
print(gop_lai_cua_an == 300)
```

```python title=test
# Hai người, hai con số, hai câu trả lời ngược nhau — rồi ba cỡ bẻ khác nữa để
# chốt rằng luật ghép-lại-khít không phải chuyện riêng của số 4. Cả khối này chỉ
# hỏi đúng một điều: ghép đủ số phần lại có dựng lại đúng cái thước 300 phân không.
assert gop_lai_cua_byte == 300, "bốn phần 75 phân ghép lại đúng bằng cái thước 300 phân"
assert gop_lai_cua_an != 300, "bốn phần 80 phân ghép lại thành 320 phân — dài hơn cả cái thước"
assert 100 * 3 == 300, "bẻ cùng cái thước ấy làm 3 phần thì mỗi phần 100 phân"
assert 60 * 5 == 300, "bẻ làm 5 phần thì mỗi phần 60 phân"
assert 50 * 6 == 300, "bẻ làm 6 phần thì mỗi phần 50 phân"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm trong `print`, ngay dưới một dòng đã in ra chiều dài gộp lại. Việc của chúng không phải in lại con số ấy, mà là hỏi máy xem con số ấy có đúng bằng cái thước cũ không.
- kind: strategy
  body: Câu hỏi ấy gồm ba phần: cái tên đang giữ chiều dài gộp lại, dấu so sánh bằng của R0 bài 24 (hai dấu bằng viết liền nhau), và chiều dài cái thước cũ tính bằng phân. Hai chỗ trống dùng hai cái tên khác nhau nên không chép được của nhau.
- kind: one-line
  body: "Thay `___` thứ nhất bằng `gop_lai_cua_byte == 300` và `___` thứ hai bằng `gop_lai_cua_an == 300`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một câu hỏi so sánh bằng (`==`) giữa chiều dài gộp lại và con số 300 — và phải so cái TÊN đang giữ chiều dài ấy với 300, không phải chép lại con số
  requireAst:
  # `min: 2` vì có hai người bẻ, mỗi người một câu kiểm. Khung chưa có dấu `==`
  # nào, nên luật này chặn được đúng cái đáp án gõ cứng hai chữ True/False.
  - kind: uses-operator, target: ==, min: 2
  # Khung đã ĐỌC mỗi cái tên đúng một lần (`print(gop_lai_cua_byte)`), nên
  # `min: 2` ép chỗ trống phải nhắc lại tên ấy — chặn đáp án `print(300 == 300)`
  # chép cứng con số mà vẫn khớp output lẫn dấu `==`.
  - kind: uses-name, target: gop_lai_cua_byte, min: 2
  - kind: uses-name, target: gop_lai_cua_an, min: 2
- tier: output
  match: regex
  expect: ^300\nTrue\n320\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn phần 75 phân thì vừa khít. Bốn phần 80 phân thì dài hơn cả cái thước.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa có một cái thước mới: bẻ đơn vị làm 4 phần bằng nhau thì được `1/4`, một
cái thước nhỏ đi bốn lần so với cái cũ. Bẻ làm 3 thì được `1/3`. Cái nào cũng là
một đơn vị đàng hoàng, đặt lên đo được như mọi cái thước khác.

Nhưng cả bài này bạn mới chỉ lấy **một** phần. Thế còn `3/4` thì là cái gì?

Byte nghe hai cách giải thích khác nhau ngoài chợ:

- Cách thứ nhất: lấy **ba cái bánh**, mỗi cái chia làm tư, rồi lấy hết chỗ ấy.
- Cách thứ hai: lấy **một cái thước `1/4`** rồi đặt nó **ba lần** liên tiếp,
  đúng như đặt bất kỳ cái thước nào khác.

Hai cách nghe khác hẳn nhau. Cách nào nói đúng `3/4` là gì — và cách kia hỏng ở
chỗ nào? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
