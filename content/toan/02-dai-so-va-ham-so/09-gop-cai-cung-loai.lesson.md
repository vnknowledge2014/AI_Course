---
id: toan.dai-so-va-ham-so.gop-cai-cung-loai
title: Chỉ gộp được thứ cùng loại
summary: `3n + 5n = 8n` không phải luật mới — đó là rút chính chữ ra ngoài; còn `8n + 6` thì dừng lại, và dừng lại không có nghĩa là sai.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.like-terms]
requires: [math.factor-common, math.expand-brackets, math.expression, math.letter-names-a-slot, math.like-units, math.multiplication, err.type-error, core.variable, core.arithmetic, core.print-variable, core.output]
concepts: [math.o-trong, math.cau-truc, math.rut-gon]
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
Ba gói cộng năm gói ra tám gói. Còn sáu ổ lẻ thì không chui vào gói nào.
::::

::::explain{#chu-cung-la-cai-chung}
Bài trước để lại một câu tính và một câu hỏi:

```text
3n + 5n
```

Khách quen của Byte ai cũng mua đúng như nhau, mỗi người `n` ổ. Sáng có 3 khách
quen, chiều có 5 khách quen. (Vẫn cái `n` của bài trước dặn: nó đếm **số ổ mỗi
khách quen**, không phải cái `n` đếm số ổ bán thêm ở bài 6–8. Một chữ chỉ có
nghĩa trong đúng cái đề đang nói.) Hai cụm này không chung nhau con số nào — 3
với 5 chẳng chung gì.

Nhưng luật hôm qua không hề nói phần chung phải là một con số. Nó nói phần chung
là *thứ có mặt ở mọi cụm*. Và ở đây có đúng một thứ như thế, nằm ngay trên mặt
giấy: chữ **`n`**.

Nên rút nó ra, y hệt cách rút 15 000 ra hôm qua:

```text
3n + 5n  =  3 × n + 5 × n  =  (3 + 5) × n  =  8n
```

Không có luật mới nào ở đây cả. Đây là luật của bài 8, đọc trên một phần chung
khác — và đó cũng là lý do `3n + 5n = 8n` không phải một điều thứ tư phải học
thuộc. Nó là hệ quả của cái khay bánh.

Chỉ khác một chỗ, và chỗ ấy đáng để ý: hôm qua rút xong thì trong ngoặc còn một
câu tính **còn ô trống** (`n + 2`). Hôm nay rút xong thì trong ngoặc còn `3 + 5`
— hai con số cụ thể, cộng ra được ngay. Nên cái ngoặc ấy tự tan, để lại `8n`.
::::

::::explain{#ba-goi-cong-nam-goi}
Vẽ ra thì rõ hơn. Coi mỗi khách quen là một **gói** — gói nào cũng đựng đúng `n`
ổ, dù `n` bằng bao nhiêu:

```text
    3n              5n                   8n
  ┌─┬─┬─┐       ┌─┬─┬─┬─┬─┐       ┌─┬─┬─┬─┬─┬─┬─┬─┐
  │n│n│n│   +   │n│n│n│n│n│   =   │n│n│n│n│n│n│n│n│
  └─┴─┴─┘       └─┴─┴─┴─┴─┘       └─┴─┴─┴─┴─┴─┴─┴─┘
```

Ba gói đổ chung với năm gói ra tám gói — đếm gói, không cần mở gói nào ra xem
trong đó có mấy ổ. Đó là toàn bộ nội dung của phép gộp.

Bạn đã gặp đúng chuyện này ở T2.1: 3 chục cộng 5 chục ra 8 chục, cộng được vì
hai bên **cùng một đơn vị**. Cái "chục" ngày ấy và cái "gói `n`" hôm nay giữ
đúng một vai.

Giờ thêm dì Tư vào. Dì lấy sẵn **6 ổ**, và 6 là 6 — nó không phụ thuộc `n`, nó
không phải một gói nào cả:

```text
         8n                    6
  ┌─┬─┬─┬─┬─┬─┬─┬─┐
  │n│n│n│n│n│n│n│n│   +   ● ● ● ● ● ●
  └─┴─┴─┴─┴─┴─┴─┴─┘
    tám cái gói n            6 ổ lẻ
```

Sáu ổ lẻ không chui vào gói nào được, vì gói phải đựng đúng `n` ổ mà 6 thì chưa
chắc bằng `n`. Nên câu tính dừng lại ở đây:

```text
8n + 6
```

Thử rút xem có được không: cái **chữ** đặt ra trước ngoặc được thì phải có mặt ở
**cả hai** cụm. `n` chỉ có ở cụm đầu, còn `6` không mang chữ nào cả. Nên không
còn chữ nào để đặt ra trước ngoặc.

(Rút một con số thì vẫn còn: `8n + 6` cũng viết được thành `2 × (4n + 3)`, vì cả
8 lẫn 6 đều chia hết cho 2. Nhưng đó là rút **số**, không phải gộp hạng tử — nó
không làm câu tính ngắn đi chỗ nào, và mạch này đang đi tìm cái khác.)

Những cụm gộp được với nhau — cùng mang đúng một chữ, như `3n` với `5n` — gọi
là **hạng tử đồng dạng**. Gộp chúng lại chính là rút chữ ấy ra ngoài.
::::

::::example{#hoi-thang-cai-may}
Người mới học rất hay viết tiếp một bước nữa: `8n + 6 = 14n`. Bắt máy làm trọng
tài, trên hai giá trị khác nhau:

```python title=readonly
n = 2
print(8 * n + 6)
print(14 * n)

n = 10
print(8 * n + 6)
print(14 * n)
```

Máy in ra:

```text
22
28
86
140
```

Không lần nào bằng nhau, và càng điền số lớn thì càng lệch xa. Máy không giải
thích vì sao — nó chỉ bác bỏ. Lý do thì cái hình ở trên đã nói: 8 đếm **gói**,
còn 6 đếm **ổ lẻ**, cộng hai con số đếm hai thứ khác nhau thì ra một con số
không đếm gì cả.
::::

::::predict{#doan-bon-dong commitOnce}
Byte gõ bốn dòng để tự kiểm lại buổi sáng.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
n = 2
print(3 * n + 5 * n)
print(8 * n)
print(8 * n + 6)
print(14 * n)
```

:::opt{correct}
16, 16, 22 rồi 28
:::

:::opt
16, 16, 22 rồi 22
::why
Gần đúng ở chỗ bạn vừa dùng đúng cái luật của bài này: `3n + 5n` gộp thành `8n`
bằng cách cộng hai con số đứng trước chữ. Luật ấy đúng, và hai dòng đầu là bằng
chứng.

Ranh giới của nó nằm ở chữ **đứng sau** hai con số ấy. `3n` và `5n` cộng được vì
cả hai đều là "bấy nhiêu gói `n`", nên rút `n` ra là xong. Còn `6` thì không có
`n` nào để rút — nó là 6 ổ lẻ, không phải 6 gói. Cộng 8 với 6 lúc này là cộng
số gói với số ổ, hai thứ khác đơn vị. Máy vừa nói ra chênh lệch ấy: 22 với 28.
::
:::

:::opt
16, 16, 64 rồi 28
::why
Gần đúng ở chỗ bạn cảm thấy `n` và `6` phải dính với nhau — và cảm giác ấy có
gốc thật: khi viết gọn `8n + 6`, mắt rất dễ đọc thành "8 nhân với cả phần còn
lại", tức `8 × (n + 6)`, ra 64.

Ranh giới nằm ở chỗ dấu nhân **trùm tới đâu**. Nó chỉ dính vào cụm ngay sát nó,
không trùm ra xa hơn; muốn nó trùm cả `n + 6` thì phải có một cặp ngoặc thật,
mà ở đây không có. Nên `8n` là một cụm, `6` là một cụm khác đứng riêng.

Và đúng cái ranh giới ấy là lý do hai cụm không gộp được: chúng là hai cụm tách
rời, mỗi cụm đếm một thứ.
::
:::

:::opt
16, 16 rồi máy dừng lại ở dòng ba, vì `8n` với `6` không cùng loại thì không cộng được
::why
Gần đúng ở hai chỗ cùng lúc, và cả hai đều là điều bạn nhớ chính xác: R0.16 nói
rằng hai thứ không cùng kiểu thì máy dừng lại chứ không đoán bừa; còn bài hôm
nay nói `8n` với `6` không cùng loại.

Ranh giới nằm giữa hai chữ **gộp** và **cộng**. "Không gộp được" chỉ có nghĩa là
không viết gọn lại thành một cụm duy nhất. Nó không có nghĩa là phép cộng ấy vô
nghĩa: `8n + 6` là một câu tính hoàn chỉnh, hợp lệ, có bảng giá trị đàng hoàng —
điền `n = 2` vào thì nó ra 22 ổ, đúng số bánh cả ngày hôm ấy.

Chỗ máy dừng lại ở R0.16 là chuyện khác hẳn: ở đó bạn bảo nó cộng một chuỗi chữ
với một con số, hai thứ không có phép cộng nào định nghĩa giữa chúng. Còn ở đây
`n` đang giữ số 2, nên `8 * n + 6` chỉ là `16 + 6`. Máy không có gì để phàn nàn.
::
:::
::::

::::explain{#het-gop-khong-phai-sai}
Đặt tên cho thứ vừa thấy:

> Hai cụm **gộp được** khi chúng mang đúng cùng một chữ. Gộp chúng lại là rút
> chữ ấy ra ngoài: `3n + 5n = (3 + 5) × n = 8n`.
>
> Cụm không mang chữ ấy thì đứng riêng. Câu tính dừng ở đó.

Ba chỗ nên để ý:

- **Dừng lại không phải là sai, cũng không phải là chưa xong.** `8n + 6` là một
  câu tính hoàn chỉnh. Nó có hai cụm, và hai cụm đó không rút gọn thêm được —
  chấm hết. Rất nhiều người mới học thấy một dấu cộng còn sót lại thì tưởng mình
  làm dở dang rồi cố gộp bừa; chính chỗ đó đẻ ra `14n`.
- **Gộp bao nhiêu cụm cũng được, miễn cùng chữ.** `2n + 7n + n` gộp thành `10n`
  — cái `n` đứng trơ một mình là `1 × n`, đếm là một gói.
- **Khác chữ là khác loại.** `3n + 5m` thì dừng ngay, dù nhìn rất giống
  `3n + 5n`. `n` với `m` là hai ô trống khác nhau, điền vào hai số khác nhau
  được, nên chúng không phải cùng một thứ để mà đếm chung.
::::

::::code{#so-banh-mot-ngay}
Byte tổng kết một ngày. Khách quen ai cũng mua đúng `n` ổ như nhau; ngoài họ ra
còn 6 ổ dì Tư lấy sẵn.

Hai chỗ trống, hai việc khác nhau:

- Chỗ thứ nhất: **gộp** hai cụm cùng loại thành một cụm duy nhất, viết dưới dạng
  *mấy lần `n`*.
- Chỗ thứ hai: tổng cả ngày. Chỗ này **hết gộp được** — viết đúng như nó là.

Bài chấm bằng cả hai chỗ, và còn chấm thêm một câu nữa: tổng cả ngày phải
**khác** `14n`. Đó chính là cái bẫy bài này nói suốt, nên nó được canh riêng.

```python title=starter
# Khách quen ai cũng mua ĐÚNG n ổ như nhau.
n = 4
sang = 3 * n     # buổi sáng có 3 khách quen
chieu = 5 * n    # buổi chiều có 5 khách quen
di_tu = 6        # dì Tư lấy sẵn 6 ổ — con số này không phụ thuộc n

# Gộp hai cụm CÙNG LOẠI thành một cụm duy nhất, viết dưới dạng "mấy lần n".
gop_khach_quen = ___

# Tổng cả ngày. Cụm này hết gộp được rồi — viết đúng như nó là.
tong_ca_ngay = ___

print(gop_khach_quen)
print(tong_ca_ngay)
```

```python title=solution
# Khách quen ai cũng mua ĐÚNG n ổ như nhau.
n = 4
sang = 3 * n     # buổi sáng có 3 khách quen
chieu = 5 * n    # buổi chiều có 5 khách quen
di_tu = 6        # dì Tư lấy sẵn 6 ổ — con số này không phụ thuộc n

# Gộp hai cụm CÙNG LOẠI thành một cụm duy nhất, viết dưới dạng "mấy lần n".
gop_khach_quen = 8 * n

# Tổng cả ngày. Cụm này hết gộp được rồi — viết đúng như nó là.
tong_ca_ngay = 8 * n + 6

print(gop_khach_quen)
print(tong_ca_ngay)
```

```python title=test
# Hai câu `!=` đứng trước: chúng canh hai cái bẫy của bài. Xếp sau các câu `==`
# thì chúng không bao giờ chạy tới, và bẫy không bao giờ sập.
assert gop_khach_quen != tong_ca_ngay, "phần khách quen và tổng cả ngày lệch nhau đúng 6 ổ của dì Tư — không thể là cùng một số"
assert tong_ca_ngay != 14 * n, "gộp 8n với 6 thành 14n là gộp gói với ổ lẻ: 14 × 4 ra 56, còn tổng thật là 38"
assert gop_khach_quen == sang + chieu, "gộp 3n với 5n phải bằng đúng cộng thẳng hai buổi lại"
assert tong_ca_ngay == sang + chieu + di_tu, "tổng cả ngày là hai buổi cộng thêm 6 ổ của dì Tư"
assert gop_khach_quen == 32, "8 khách quen, mỗi người 4 ổ, là 32 ổ"
assert tong_ca_ngay == 38, "32 ổ của khách quen cộng 6 ổ của dì Tư là 38 ổ"
```

:::hints
- kind: attention
  body: Hai dòng `sang` và `chieu` đều có dạng "mấy lần n". Đếm xem cả ngày có tất cả mấy cái gói n. Còn dòng `di_tu` thì không có chữ n nào — hỏi xem nó có vào chung được không.
- kind: strategy
  body: Gộp hai cụm cùng chữ là cộng hai con số đứng trước chữ ấy, rồi giữ nguyên chữ. Chỗ trống thứ hai thì VIẾT LẠI cụm vừa gộp — `8 * n`, chứ không phải cái tên `gop_khach_quen` — rồi cộng thêm 6 và dừng lại. Đừng cộng 6 vào con số đứng trước n.
- kind: one-line
  body: "Chỗ trống thứ nhất là `8 * n`, chỗ trống thứ hai là `8 * n + 6`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống thứ nhất phải là số gói nhân với `n`, chỗ trống thứ hai là cụm ấy cộng thêm 6 — gõ thẳng 32 với 38 thì phép gộp mà bài này dạy không xuất hiện ở đâu cả
  requireAst:
  # Con số 8 là kết quả của phép gộp `3 + 5`, và nó không có sẵn ở đâu trong
  # khung khởi đầu (khung chỉ có 4, 3, 5, 6). Đây là lưới chính.
  - kind: has-literal, target: 8
  # Chữ `n` phải được ĐỌC thêm hai lần nữa, một lần cho mỗi chỗ trống. Khung
  # khởi đầu đọc nó đúng hai lần.
  - kind: uses-name, target: n, min: 4
  - kind: uses-operator, target: *, min: 4
  # Khung khởi đầu không có dấu cộng nào — `di_tu = 6` chỉ là một phép gán.
  # Dấu cộng duy nhất của lời giải nằm ở chỗ trống thứ hai, chỗ câu tính DỪNG.
  - kind: uses-operator, target: +, min: 1
  # Số 6 phải xuất hiện lần thứ hai, trong chính cụm không gộp được.
  - kind: has-literal, target: 6, min: 2
  forbidAst:
  # Lưới thứ hai, chặn hai con số KẾT QUẢ. Lời giải thật không chứa nguyên văn
  # chúng, nên luật này không cản ai làm thật.
  - kind: has-literal, target: 32
  - kind: has-literal, target: 38
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^32\n38\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
32 ổ của khách quen, thêm 6 ổ của dì Tư là 38. Gói ra gói, ổ lẻ ra ổ lẻ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại chín bài vừa qua. Bạn đã mở ngoặc, gấp ngoặc, rút cái chung, gộp cái
cùng loại — bốn cái tên, mà như bài này vừa chỉ ra, chúng là **một luật đọc
theo bốn chiều**. Rút gọn hết cỡ rồi,
câu tính của quán còn lại đúng thế này:

```text
8n + 6
```

Và nó **vẫn còn ô trống**. Vẫn chưa ra một con số nào. Bẻ tới bẻ lui bao nhiêu
lần cũng thế, vì mọi phép hôm nay chỉ đổi hình dạng chứ không điền vào ô.

Để ý một chuyện: suốt chín bài, chưa một lần nào ai hỏi **"`n` bằng bao nhiêu"**.
Không phải quên. Câu hỏi ấy được để dành, vì tới giờ nó vẫn chưa có nghĩa —
`n` là một ô trống, mà ô trống thì nhận được mọi số, hỏi nó "bằng bao nhiêu"
thì cũng như hỏi một cái hộp rỗng đựng gì.

Vậy phải thêm cái gì vào mặt giấy thì câu hỏi ấy mới bắt đầu có nghĩa?

Bài sau bắt dấu `=` làm một việc nó chưa từng làm trong track này. Tới giờ mọi
dấu `=` bạn gặp — `3n + 5n = 8n`, `a × (b + c) = a × b + a × c` — đều thuộc loại
LUÔN ĐÚNG: nó nối hai câu tính vốn là một, điền số nào vào cũng khớp. Bài sau
đưa ra một dấu `=` khác hẳn: loại chỉ đúng với **một vài** con số, và việc của
bạn là đi tìm chúng.
::::

::::checkpoint{mastery=0.8}
::::
