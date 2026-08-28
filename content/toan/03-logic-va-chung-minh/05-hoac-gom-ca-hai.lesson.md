---
id: toan.logic-va-chung-minh.hoac-gom-ca-hai
title: "Hoặc" gồm cả hai
summary: Chữ "hoặc" của logic vẫn gật ở cái dòng mà tiếng Việt hay lắc đầu — dòng cả hai vế cùng đúng; muốn cái nghĩa "chọn một" thì phải viết thêm vế mà tiếng Việt vẫn ngầm hiểu.
locale: vi
track: toan
module: logic-va-chung-minh
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.disjunction]
requires: [logic.proposition, logic.truth-value, logic.conjunction, logic.negation, logic.and, logic.or, logic.not, logic.parentheses, core.boolean, core.variable, core.list, core.list-append, core.tuple, core.for-unpack, core.print-variable, ctrl.for-each]
concepts: [logic.tuyen, logic.hoac-bao-gom, logic.chon-mot-phai-noi-ra]
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
Mình bê về hai cốc. Giờ không biết mình làm đúng lời cô hay làm sai.
::::

::::explain{#hai-coc-tren-tay}
Bài trước để lại Byte đứng giữa quán với hai cốc trên tay.

Cô chủ nhiệm dặn: *"Mỗi bạn gọi trà hoặc cà phê."* Byte gọi cả hai. Câu cô dặn
lúc này đúng hay sai?

Hỏi ba người thì được hai câu trả lời khác nhau, và cả hai đều nghe lọt tai:

- **"Sai rồi."** Chữ "hoặc" ở quán nghĩa là chọn một. Bê về hai cốc là không
  còn "chọn" nữa.
- **"Đúng chứ."** Cô dặn để không ai ngồi suông. Byte có cốc trên tay, thậm chí
  hai cốc — càng không suông.

Hai người này không ai nói dối. Họ đang đọc **hai câu khác nhau** từ cùng một
dòng chữ. Và bài 4 vừa cho bạn công cụ để tách chúng ra: đừng cãi nhau bằng cảm
giác, hãy xét từng khả năng một rồi xem hai cách đọc lệch nhau ở **dòng nào**.
::::

::::example{#bang-cong-cua-clb}
Đổi sang một chỗ mà chuyện này có hậu quả thật.

Bảng công của CLB cờ vua treo ở cuối lớp. Cô dặn Lan — bạn thư ký — đúng một
câu:

> *"Ghi tên Khanh lên bảng công nếu **Khanh nộp quỹ hoặc Khanh trực nhật**."*

Câu ghép đang xét là "Khanh nộp quỹ **hoặc** Khanh trực nhật". Hai vế, mỗi vế
một mệnh đề về đúng một bạn có tên. Mỗi vế mang Đ hoặc S, nên có bốn kiểu điền:

| kiểu | Khanh nộp quỹ | Khanh trực nhật | Khanh làm được gì |
|---|---|---|---|
| 1 | Đ | Đ | nộp quỹ, và trực nhật luôn |
| 2 | Đ | S | chỉ nộp quỹ |
| 3 | S | Đ | chỉ trực nhật |
| 4 | S | S | không làm việc nào |

Ba kiểu dưới thì không ai cãi nhau. Kiểu 2 và kiểu 3: Khanh làm được một việc,
tên lên bảng. Kiểu 4: Khanh không làm gì, tên không lên.

Cả cuộc cãi nhau nằm gọn ở **kiểu 1** — cái kiểu mà Khanh làm cả hai việc.
::::

::::explain{#logic-chot-dong-dau}
Logic chốt kiểu 1 là **Đ**, và đặt tên cho phép ghép ấy:

> Ghép hai mệnh đề bằng chữ **"hoặc"** cho ra một mệnh đề mới, gọi là **tuyển**
> của hai mệnh đề ấy. Tuyển đúng khi có **ít nhất một** vế đúng — kể cả khi cả
> hai vế cùng đúng.

Cái mới ở đây không phải mấy chữ "ít nhất một vế đúng"; bạn đã gõ `or` với đúng
luật ấy suốt Realm 0. Cái mới là chỗ này: chữ "hoặc" của logic **gồm cả hai**,
còn chữ "hoặc" bạn nói hằng ngày thì thường ngầm hiểu là chọn một. Hai chữ
giống hệt nhau trên mặt giấy, mà lệch nhau ở đúng một dòng của bảng.

Vì sao logic chọn dòng ấy là Đ, chứ không chọn ngược lại? Đọc lại lời cô dặn.
Câu ấy nói về **điều kiện để tên lên bảng**, và Khanh làm cả hai việc thì Khanh
càng xứng đáng chứ không kém đi. Bắt cô phải nói "Khanh nộp quỹ hoặc Khanh trực
nhật, mà nếu làm cả hai thì thôi" mới là chuyện lạ.

Và còn một lý do nữa, nặng hơn: nghĩa "chọn một" **viết ra được** từ nghĩa
"ít nhất một", còn chiều ngược lại thì không tiện bằng. Muốn nói "chọn một",
bạn nói thêm cái vế mà tiếng Việt vẫn ngầm hiểu và giấu đi:

> **"Khanh nộp quỹ hoặc Khanh trực nhật, và không phải Khanh làm cả hai."**

Câu dài ấy ghép từ đúng ba chữ nối bạn đã có: "hoặc", "và", "không phải". Nó
lệch với câu ngắn ở đúng kiểu 1, và trùng với câu ngắn ở cả ba kiểu còn lại.

Nên từ đây trở đi, trong track này, chữ "hoặc" trần luôn là chữ "hoặc" **gồm cả
hai**. Chỗ nào bạn muốn nghĩa "chọn một", bạn phải nói cái vế kia ra thành lời.
::::

::::predict{#doan-bon-dong commitOnce}
Byte đem hai câu ghép cho máy chấm. "Khanh nộp quỹ" giữ trong cái tên
`nop_quy`, "Khanh trực nhật" giữ trong `truc_nhat`.

Byte chạy cả hai câu ở hai kiểu: kiểu Khanh làm cả hai việc, rồi kiểu Khanh chỉ
trực nhật.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
nop_quy = True
truc_nhat = True
print(nop_quy or truc_nhat)
print(nop_quy and truc_nhat)

nop_quy = False
print(nop_quy or truc_nhat)
print(nop_quy and truc_nhat)
```

:::opt{correct}
True, True, True, False
:::

:::opt
False, True, True, False
::why
Gần đúng ở chỗ bạn đọc trúng ba dòng, và ở chỗ bạn đang đọc chữ "hoặc" theo
đúng nghĩa nó mang trong tiếng Việt hằng ngày. *"Ăn phở hoặc ăn bún"* mà bưng
về hai tô thì người nói sẽ ngạc nhiên thật — hiểu như thế trong đời sống là
hiểu đúng, và giữ lấy cái tai ấy.

Chỗ lệch là ranh giới giữa hai chữ "hoặc". Chữ của Python — và từ bài này trở
đi, chữ của logic — gật ngay khi có **ít nhất một** vế đúng, và nó không hỏi
thêm câu nào nữa. Dòng 1 có tới hai vế đúng, tức là quá đủ, nên nó in `True`.
Muốn cái nghĩa "chọn một", bạn phải viết thêm vế "và không phải cả hai" ra
thành lời.
::
:::

:::opt
True, True, False, False
::why
Gần đúng ở chỗ bạn đọc trúng dòng 1 — bạn đã nhận ra chữ "hoặc" vẫn gật khi cả
hai vế cùng đúng, mà đó là chỗ khó nhất của bài.

Chỗ lệch nằm ở dòng 3, và nó lệch vì bạn để **vế đứng trước** quyết định: dòng
`nop_quy = False` hạ vế đầu xuống S, nên bạn cho cả câu xuống theo. Bài 4 vừa
chốt chuyện đó rồi, và nó đúng cho cả chữ "hoặc": câu ghép chỉ nhìn hai giá
trị, không nhìn vế nào đứng trước. Ở dòng 3, `truc_nhat` vẫn đang giữ `True` từ
đầu bài — Khanh vẫn trực nhật — nên vẫn còn một vế đúng, và tuyển vẫn gật.
::
:::

:::opt
True, True, True, True
::why
Gần đúng ở chỗ bạn đọc trúng ba dòng, và ở chỗ bạn nắm chắc điều bài này dạy:
có một vế đúng là chữ "hoặc" gật, không cần cả hai.

Chỗ lệch: bạn mang cái luật ấy sang cho dòng thứ tư, mà dòng thứ tư nối bằng
chữ **"và"**. Hai chữ nối này chia bốn dòng theo hai kiểu ngược nhau — bài 4 đã
dựng bảng cho chữ "và": nó chỉ gật ở đúng một dòng trong bốn, dòng cả hai vế
cùng đúng. Dòng cuối có `nop_quy` là `False`, tức Khanh không nộp quỹ, nên
`and` trả `False`.
::
:::
::::

::::code{#ba-cot-hai-cach-doc}
Bắt máy dựng ba cột cạnh nhau, rồi để chính ba cột ấy chỉ ra hai cách đọc chữ
"hoặc" tách nhau ở đúng dòng nào.

Bốn kiểu điền Đ/S cho hai vế được viết sẵn bằng tay, mỗi kiểu một cặp giá trị.
Vòng lặp lấy ra từng cặp một, đặt vào hai cái tên `nop` và `truc`.

Ba câu ghép cần điền, đúng theo chú thích ghi bên cạnh mỗi cột:

- **Cột hoặc** — "Khanh nộp quỹ **HOẶC** Khanh trực nhật".
- **Cột và** — "Khanh nộp quỹ **VÀ** Khanh trực nhật".
- **Cột chọn một** — "Khanh nộp quỹ hoặc Khanh trực nhật, **và không phải**
  Khanh làm cả hai".

Một luật của bài: **cột thứ ba phải viết bằng ba chữ nối, không viết bằng dấu
so sánh.** Có những cách gõ tắt cho ra đúng cột ấy, nhưng bài này đi tìm chính
chuyện *nghĩa "chọn một" ghép lại được từ "hoặc", "và", "không phải"* — gõ tắt
là bỏ qua đúng cái đang cần thấy.

Bài chấm bằng cả ba cột, và ba cột được chọn để cư xử khác hẳn nhau: cột "hoặc"
gật ba dòng, cột "và" gật một dòng, cột "chọn một" gật hai dòng. Chép một câu
vào cả ba chỗ thì ba cột trùng nhau và hỏng ngay.

```python title=starter
# Bốn kiểu điền Đ/S cho hai vế, viết ra bằng tay — mỗi kiểu một dòng bảng.
bon_kieu = [(True, True), (True, False), (False, True), (False, False)]

hoac = []      # "Khanh nộp quỹ HOẶC Khanh trực nhật"
va = []        # "Khanh nộp quỹ VÀ Khanh trực nhật"
chon_mot = []  # "... hoặc ..., VÀ KHÔNG PHẢI Khanh làm cả hai"

for nop, truc in bon_kieu:
    hoac.append(___)
    va.append(___)
    chon_mot.append(___)

print(hoac)
print(va)
print(chon_mot)
```

```python title=solution
# Bốn kiểu điền Đ/S cho hai vế, viết ra bằng tay — mỗi kiểu một dòng bảng.
bon_kieu = [(True, True), (True, False), (False, True), (False, False)]

hoac = []      # "Khanh nộp quỹ HOẶC Khanh trực nhật"
va = []        # "Khanh nộp quỹ VÀ Khanh trực nhật"
chon_mot = []  # "... hoặc ..., VÀ KHÔNG PHẢI Khanh làm cả hai"

for nop, truc in bon_kieu:
    hoac.append(nop or truc)
    va.append(nop and truc)
    chon_mot.append((nop or truc) and not (nop and truc))

print(hoac)
print(va)
print(chon_mot)
```

```python title=test
# Ba câu `!=` đứng TRƯỚC. Chúng canh đúng cái bẫy của bài: chép một câu ghép
# vào cả ba chỗ trống thì ba cột trùng nhau hết, và hai cách đọc chữ "hoặc" —
# thứ cả bài dựng lên để tách ra — sẽ không còn chỗ nào tách được. Xếp chúng
# sau các câu `==` thì một câu `==` trượt trước, và bẫy không bao giờ sập.
assert hoac != va, "cột HOẶC gật khi có ít nhất một vế đúng, cột VÀ đòi cả hai — hai cột không thể trùng nhau; trùng nghĩa là bạn đã chép một câu vào cả hai chỗ"
assert hoac != chon_mot, "hai cách đọc chữ 'hoặc' phải lệch nhau ở kiểu 1; trùng khít nghĩa là cột thứ ba chưa nói ra cái vế 'và không phải cả hai'"
assert va != chon_mot, "cột VÀ gật đúng ở kiểu Khanh làm cả hai, còn cột chọn-một thì loại đúng kiểu ấy ra — hai cột đi ngược nhau, không thể trùng"
assert len(hoac) == 4, "bảng có bốn kiểu điền, nên mỗi cột phải đủ bốn ô — thiếu ô nào là sót một kiểu"
assert hoac[0] is True, "kiểu 1 — Khanh vừa nộp quỹ vừa trực nhật: chữ HOẶC của logic vẫn gật ở dòng này, và đó chính là dòng cả bài này bàn tới"
assert chon_mot[0] is False, "kiểu 1 — Khanh làm cả hai việc: cách đọc 'chọn một' loại đúng dòng này ra, nên ô đầu của cột chọn-một phải S"
assert hoac[1] == chon_mot[1], "kiểu 2 — Khanh chỉ nộp quỹ: hai cách đọc cùng gật ở dòng này, chúng không lệch nhau ở đây"
assert hoac[2] == chon_mot[2], "kiểu 3 — Khanh chỉ trực nhật: hai cách đọc lại cùng gật, vẫn không phải chỗ chúng lệch"
assert hoac[3] == chon_mot[3], "kiểu 4 — Khanh không làm việc nào: hai cách đọc cùng lắc, nên chỗ lệch chỉ còn đúng một dòng để nằm"
assert hoac == [True, True, True, False], "cột HOẶC: ba kiểu đầu đều còn ít nhất một vế đúng nên câu đúng; kiểu 4 hụt cả hai vế nên câu sai"
assert va == [True, False, False, False], "cột VÀ: chỉ kiểu 1 có cả hai vế cùng đúng, ba kiểu còn lại đều hụt ít nhất một vế"
assert chon_mot == [False, True, True, False], "cột chọn-một: gật ở đúng hai kiểu Khanh làm một việc, lắc ở kiểu làm cả hai và ở kiểu không làm gì"
```

:::hints
- kind: attention
  body: Mỗi chỗ trống là **một câu ghép** viết từ hai cái tên `nop` và `truc`, không phải một giá trị Đ/S gõ sẵn. Đọc lại chú thích bên phải mỗi cột: nó ghi đủ hai vế và các chữ nối. Riêng chú thích của cột thứ ba có tới ba chữ nối, và cả ba đều phải xuất hiện trong câu bạn viết.
- kind: strategy
  body: "Dịch từng chữ một. Chữ HOẶC là `or`, chữ VÀ là `and`, chữ KHÔNG PHẢI là `not`. Hai cột đầu mỗi cột đúng một chữ nối, viết thẳng ra là xong. Cột thứ ba ghép hai câu bạn vừa viết: lấy nguyên cột thứ nhất làm vế trái, lấy chữ `not` đặt trước nguyên cột thứ hai làm vế phải, rồi nối hai vế ấy bằng `and`. Nhớ đóng ngoặc quanh mỗi vế — dấu ngoặc của T1.2.7 nói rõ hai vế nào được nối trước."
- kind: one-line
  body: "Ba chỗ lần lượt là `nop or truc`, `nop and truc`, và `(nop or truc) and not (nop and truc)`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: mỗi chỗ trống phải là một câu ghép viết theo `nop` và `truc`, và riêng cột thứ ba phải dùng đủ ba chữ nối `or`, `and`, `not` — gõ tắt bằng một dấu so sánh thì cho ra đúng cột ấy mà không cho thấy nghĩa "chọn một" ghép lại được từ ba chữ nối, đúng cái bài này đang đi tìm
  requireAst:
  # Cột 1 một chữ `or`, cột 3 một chữ `or` nữa. Khung khởi đầu không có `or`
  # nào, nên luật này một mình đã chặn đáp án gõ cứng `True`/`False`.
  - kind: uses-operator, target: or, min: 2
  # Cột 2 một chữ `and`, cột 3 ít nhất một chữ `and` nữa.
  - kind: uses-operator, target: and, min: 2
  # Chữ "không phải" của cột 3. Thiếu nó thì cột 3 trùng cột 1, và chỗ lệch
  # giữa hai cách đọc không được viết ra ở đâu cả.
  - kind: uses-operator, target: not, min: 1
  # Ba cột đều nói về Khanh nộp quỹ: 1 lần ở cột 1, 1 lần ở cột 2, 2 lần ở
  # cột 3. Khung khởi đầu đọc 0 lần — `for nop, truc in ...` là gán, không
  # phải đọc.
  - kind: uses-name, target: nop, min: 3
  - kind: uses-name, target: truc, min: 3
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[True, True, True, False\]\n\[True, False, False, False\]\n\[False, True, True, False\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba cột, và hai cách đọc chỉ cãi nhau ở đúng một dòng. Mình bê hai cốc là hợp lệ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cả bài này đứng được là nhờ một câu bạn chưa hề kiểm: *"Có bốn kiểu điền Đ/S
cho hai vế."* Bạn vừa xét bốn kiểu ấy, đếm xem hai cách đọc lệch nhau ở dòng
nào, rồi kết luận chúng lệch ở **đúng một** dòng.

Nhưng bốn kiểu ấy do tay bạn viết ra, theo trí nhớ. Lấy gì bảo đảm bốn kiểu ấy
là **đủ**, không sót?

Và nếu câu ghép có **ba** vế thì bao nhiêu kiểu?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
