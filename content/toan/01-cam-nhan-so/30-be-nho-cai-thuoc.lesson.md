---
id: toan.cam-nhan-so.be-nho-cai-thuoc
title: Bẻ nhỏ cái thước
summary: Khi cái thước không với tới phần thừa thì bẻ chính nó thành b phần bằng nhau — một phần trong đó là một đơn vị mới, và tên của nó là 1/b.
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

> Bẻ cái thước thành **b phần bằng nhau**. Ghép đủ b phần ấy lại thì phải dựng
> lại **đúng** cái thước cũ, không thừa không thiếu.

Đó là toàn bộ nghĩa của `1/b`. Nếu Byte chặt đoạn 3 mét thành một mẩu 2 mét và
một mẩu 1 mét thì có hai mẩu thật, nhưng không mẩu nào là "một phần hai" cả —
chúng không bằng nhau, nên ghép hai mẩu ấy lại chẳng nói lên luật gì.

Hai chỗ cần cẩn thận, vì đây là nơi người ta hay đi chệch:

**Một: con số dưới gạch không phải "cái bánh".** Nó nói cái **đơn vị** bị bẻ làm
mấy phần. Cùng con số 4 dưới gạch: `1/4` mét, `1/4` lon gạo, `1/4` giờ — ba lượng
khác hẳn nhau, vì ba cái thước khác nhau. Giống nhau ở chỗ mỗi cái đều là một
trong bốn phần bằng nhau của cái thước của mình.

**Hai: bẻ càng nhiều phần thì mỗi phần càng ngắn.** Chuyện này đi ngược thói quen
đếm — 5 cái bó thì nhiều hơn 2 cái bó, nhưng bẻ làm 5 phần thì mỗi phần ngắn hơn
bẻ làm 2 phần. Con số dưới gạch không đếm phần bạn có; nó đếm số nhát bẻ.

Và nhớ lại bài 4: chỉ thước **đo** mới bẻ được. Bẻ đôi sải dây thì nửa sải dây
vẫn là dây; bẻ đôi một hạt thì nửa hạt không còn là hạt nào cả.
::::

::::example{#hai-cach-be-mot-soi-day}
Đem một sợi dây dài đúng **1 mét** ra làm ví dụ, và đo bằng **phân** cho tiện —
1 mét là 100 phân.

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
không giống nhau chút nào — một cái 50 phân, một cái 20 phân.
::::

::::predict{#doan-phan-nao-dai-hon commitOnce}
Byte cầm phần của mình (`1/2` mét), An cầm phần của An (`1/5` mét). Hai người
để cạnh nhau xem phần nào dài hơn.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
mot_phan_hai = 50
mot_phan_nam = 20
print(mot_phan_hai > mot_phan_nam)
```

:::opt{correct}
True
:::

:::opt
False
::why
Gần đúng ở chỗ bạn đọc `5` lớn hơn `2` rồi kết luận phần của An lớn hơn. Với số
đếm thì luật ấy chuẩn không chệch: 5 cái bó nhiều hơn 2 cái bó, 5 hạt nhiều hơn
2 hạt, và suốt hai mươi mấy bài vừa rồi nó chưa hỏng lần nào.

Chỗ ra ngoài phạm vi: con số dưới gạch không đếm thứ bạn đang có, nó đếm **số
phần cái thước bị bẻ ra**. Bẻ một sợi dây làm 5 thì mỗi mẩu ngắn hơn bẻ làm 2 —
sợi dây có dài thêm đâu, chỉ là chia cho nhiều chỗ hơn. Số càng lớn thì phần càng
ngắn, đúng chiều ngược với số đếm.
::
:::

:::opt
100
::why
Gần đúng ở chỗ bạn nhớ đúng điều ví dụ vừa chỉ ra: cả hai cách bẻ đều dựng lại
đúng 100 phân, nên 100 là con số chung của hai bên. Nắm được chỗ đó là nắm được
luật của phép bẻ.

Chỗ ra ngoài phạm vi: dòng cuối không gộp các phần lại. Nó đặt dấu `>` giữa hai
cái tên, mà `>` là một câu hỏi có–không (R0 bài 6) — thứ đi ra khỏi nó luôn là
`True` hoặc `False`. Muốn thấy 100 thì viết `print(mot_phan_hai * 2)`.
::
:::

:::opt
Không so được, vì hai phần ấy đo bằng hai cái thước khác nhau
::why
Gần đúng ở chỗ bạn dùng đúng luật bài 11: chỉ gộp và so được những thứ **cùng
một đơn vị**. Luật ấy rất đáng giữ, và ở bài 34 nó sẽ quay lại đúng lúc — khi
phải so `2/3` với `3/4`, hai con số ghi bằng hai cái thước khác nhau thật.

Chỗ ra ngoài phạm vi: ở dòng code này hai con số đã được ghi bằng **cùng một
đơn vị** rồi — cả `50` lẫn `20` đều tính bằng phân. Cái việc quy về cùng đơn vị
đã làm xong trước khi máy nhìn thấy chúng, nên còn lại chỉ là so hai con số phân
với nhau.
::
:::
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Bẻ đúng thì gộp lại vừa khít. Bẻ sai thì gộp lại phình ra. Bạn kiểm giúp mình nhé.
::::

::::code{#kiem-hai-cach-be}
Lấy lại đúng cái thước hôm qua: **đoạn 3 mét**, tức **300 phân**. Hôm nay hai
người cùng bẻ nó làm **4 phần bằng nhau**, nhưng nói ra hai con số khác nhau:

- **Byte** bảo mỗi phần dài **75 phân**.
- **An** bảo mỗi phần dài **80 phân** — An lỡ lấy con số gần với 4 phần cho tròn.

Mỗi người in hai dòng: dòng trên là chiều dài gộp lại của cả bốn phần (đã viết
sẵn), dòng dưới là câu kiểm *"gộp lại có dựng đúng cái thước 300 phân không"*.
Hai chỗ trống là hai câu kiểm ấy.

Bài chấm bằng **cả hai** người, và hai người này cho ra hai câu trả lời ngược
nhau. Gõ cứng `True` vào cả hai chỗ thì phần của An lọt; gõ cứng `False` thì
phần của Byte trượt. Chỉ một câu hỏi viết thật mới qua được cả hai.

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
# Hai người, hai con số, hai câu trả lời ngược nhau — và ba cách bẻ khác nữa để
# chốt rằng luật này không phải chuyện riêng của số 4.
assert gop_lai_cua_byte == 300, "bốn phần 75 phân ghép lại đúng bằng cái thước 300 phân"
assert gop_lai_cua_an != 300, "bốn phần 80 phân ghép lại thành 320 phân — dài hơn cả cái thước"
assert 100 * 3 == 300, "bẻ cùng cái thước ấy làm 3 phần thì mỗi phần 100 phân"
assert 60 * 5 == 300, "bẻ làm 5 phần thì mỗi phần 60 phân"
assert 75 < 100, "bẻ làm 4 phần thì mỗi phần phải NGẮN hơn khi bẻ làm 3 phần"
assert 60 < 75, "bẻ làm 5 phần thì lại ngắn hơn nữa — bẻ càng nhiều, phần càng ngắn"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm trong `print`, ngay dưới một dòng đã in ra chiều dài gộp lại. Việc của chúng không phải in lại con số ấy, mà là hỏi máy xem con số ấy có đúng bằng cái thước cũ không.
- kind: strategy
  body: Câu hỏi ấy gồm ba phần: cái tên đang giữ chiều dài gộp lại, dấu so sánh bằng của R0 bài 6 (hai dấu bằng viết liền nhau), và chiều dài cái thước cũ tính bằng phân. Hai chỗ trống dùng hai cái tên khác nhau nên không chép được của nhau.
- kind: one-line
  body: "Thay `___` thứ nhất bằng `gop_lai_cua_byte == 300` và `___` thứ hai bằng `gop_lai_cua_an == 300`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một câu hỏi so sánh bằng (`==`) giữa chiều dài gộp lại và con số 300 — gõ thẳng `True` hay `False` thì không kiểm được luật nào cả
  requireAst:
  # `min: 2` vì có hai người bẻ, mỗi người một câu kiểm. Khung chưa có dấu `==`
  # nào, nên luật này chặn được đúng cái đáp án gõ cứng hai chữ True/False.
  - kind: uses-operator, target: ==, min: 2
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
