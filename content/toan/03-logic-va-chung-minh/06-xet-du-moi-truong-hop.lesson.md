---
id: toan.logic-va-chung-minh.xet-du-moi-truong-hop
title: Xét đủ mọi trường hợp
summary: Bảng chân lý là cách đi qua mọi tổ hợp giá trị của các vế mà không sót không lặp — và mỗi lần thêm một vế thì số dòng gấp đôi.
locale: vi
track: toan
module: logic-va-chung-minh
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.truth-table]
requires: [logic.proposition, logic.truth-value, logic.disjunction, logic.conjunction, logic.negation, logic.and, logic.or, logic.parentheses, logic.precedence, core.boolean, core.variable, core.list, core.list-append, core.len, core.print-variable, ctrl.for-each, ctrl.nested-loop]
concepts: [logic.bang-chan-ly, logic.khong-sot-khong-lap, logic.gap-doi-moi-ve]
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
Bốn kiểu ấy mình nhớ ra. Mình muốn một cách đi mà nhớ sót cũng không sót được.
::::

::::explain{#bon-kieu-do-tri-nho-viet-ra}
Bài trước kết ở một chỗ hơi hụt chân.

Cả kết luận của bài — hai cách đọc chữ "hoặc" lệch nhau ở **đúng một** dòng —
đứng trên một câu bạn chưa hề kiểm: *"Có bốn kiểu điền Đ/S cho hai vế."*

Bốn kiểu ấy do tay bạn viết ra. Viết đúng thật, nhưng đúng vì trí nhớ tốt. Với
hai vế thì trí nhớ còn theo kịp. Với ba vế, bốn vế thì nó thua — mà "lệch ở
đúng một dòng" là câu chỉ nói được khi bạn chắc mình đã nhìn hết mọi dòng.

Cái cần không phải một trí nhớ tốt hơn. Cái cần là một **cách đi**: đi theo nó
thì không sót được, và cũng không lặp được.
::::

::::example{#kim-gio-va-kim-phut}
Cách đi ấy bạn đã có sẵn từ T1.2, ở bài về vòng lặp lồng nhau.

Nhìn mặt đồng hồ: kim giờ nhích từ 8 sang 9 đúng một nấc, và trong khoảng đó
kim phút quay **trọn một vòng**. Rồi kim giờ mới nhích tiếp, và kim phút lại
quay trọn một vòng nữa, đếm lại từ đầu.

Áp đúng hình dạng ấy vào hai vế. Vế thứ nhất là kim giờ — nó có hai nấc, Đ rồi
S. Vế thứ hai là kim phút — với **mỗi** nấc của vế thứ nhất, nó chạy trọn cả
hai giá trị của nó.

```python title=readonly
for nop in [True, False]:
    for truc in [True, False]:
        print(nop, truc)
```

```text title=readonly
True True
True False
False True
False False
```

Bốn dòng, đúng bốn kiểu bạn viết tay ở bài trước, đúng cả thứ tự.

Và lần này bốn dòng ấy không do trí nhớ. Chúng do cách đi:

- **Không sót.** Muốn sót một dòng thì phải có một cặp giá trị mà máy không đi
  qua. Nhưng vòng ngoài đi hết mọi giá trị của vế đầu, và với mỗi giá trị ấy
  vòng trong đi hết mọi giá trị của vế sau. Không cặp nào lọt ra ngoài.
- **Không lặp.** Hai dòng muốn trùng nhau thì phải cùng nấc vòng ngoài **và**
  cùng nấc vòng trong. Mà trong một nấc vòng ngoài, vòng trong không đi lại nấc
  nào hai lần.

Cái bảng dựng theo cách ấy có tên:

> **Bảng chân lý** của một câu ghép là bảng liệt kê **mọi** tổ hợp giá trị Đ/S
> của các vế — mỗi tổ hợp đúng một lần, không sót và không lặp — kèm giá trị
> mà câu ghép nhận ở từng tổ hợp.

Từ bài 4 bạn đã biết vì sao chỉ cần bấy nhiêu: giá trị câu ghép **chỉ** do các
giá trị Đ/S của các vế quyết định. Đi hết các tổ hợp giá trị là đã đi hết mọi
chuyện có thể xảy ra với câu ghép ấy.
::::

::::explain{#them-mot-ve-thi-gap-doi}
Giờ trả nốt câu hỏi thứ hai của bài trước: **ba vế thì bao nhiêu kiểu?**

Buổi họp CLB cờ vua chỉ tiến hành khi đủ người. Ba bạn được nhắc tên trong lời
cô dặn: Nam, Lan, Minh. Ba mệnh đề, mỗi mệnh đề mang Đ hoặc S.

Thêm một vòng nữa vào trong cùng, và đọc kết quả theo đúng lối vừa rồi: mỗi
dòng cũ của bảng hai vế bây giờ **nở ra hai dòng** — một dòng Minh có mặt, một
dòng Minh vắng. Bảng cũ có 4 dòng, nên bảng mới có 4 × 2 = 8 dòng.

| dòng | Nam có mặt | Lan có mặt | Minh có mặt |
|---|---|---|---|
| 1 | Đ | Đ | Đ |
| 2 | Đ | Đ | S |
| 3 | Đ | S | Đ |
| 4 | Đ | S | S |
| 5 | S | Đ | Đ |
| 6 | S | Đ | S |
| 7 | S | S | Đ |
| 8 | S | S | S |

Đếm lại cho chắc: 2 × 2 × 2 = 8.

Và cái luật ấy không dừng ở ba. Thêm một vế nữa thì mỗi dòng lại nở ra hai, nên
bảng bốn vế có 8 × 2 = 16 dòng.

> Mỗi lần thêm một vế vào câu ghép, **số dòng của bảng gấp đôi**.

Chép tay 4 dòng thì được. Chép tay 16 dòng thì bắt đầu sai. Đó là lúc cái vòng
lặp lồng nhau đáng tiền.
::::

::::explain{#ba-ve-thi-phai-noi-ro-noi-ai-truoc}
Có một chuyện chỉ xuất hiện khi câu ghép có từ ba vế trở lên, và phải nói ngay
trước khi dựng bảng.

Cô dặn Lan một câu:

> *"Buổi họp tiến hành khi Nam có mặt và Lan có mặt hoặc Minh có mặt."*

Đọc kỹ thì câu ấy **chưa nói rõ ý**. Nó nối được theo hai kiểu, và hai kiểu là
hai câu khác nhau:

- **Câu A** — "Nam có mặt **và** (Lan có mặt **hoặc** Minh có mặt)". Nam phải có
  mặt, đó là điều kiện không bỏ được; thêm vào đó cần ít nhất một trong hai bạn
  kia.
- **Câu B** — "(Nam có mặt **và** Lan có mặt) **hoặc** Minh có mặt". Có Nam lẫn
  Lan thì họp; mà chỉ mình Minh cũng đủ họp.

T1.2 đã dạy chuyện này rồi, ở hai bài liền nhau: máy có một thứ tự ngầm để
quyết định ai được tính trước, và **dấu ngoặc** là cách bạn nói rõ ý mình thay
vì phó mặc cho thứ tự ngầm ấy. Ở đây dấu ngoặc không phải trang trí — nó chọn
giữa hai câu.

Hai câu ấy khác nhau tới đâu? Lấy đúng **dòng 5** của bảng trên mà thử: Nam
vắng, Lan có mặt, Minh có mặt.

- Câu A đòi Nam có mặt trước đã. Nam vắng, nên câu A **S**.
- Câu B gật ngay khi Minh có mặt, chẳng cần hỏi Nam. Nên câu B **Đ**.

Một dòng đủ để biết hai câu là hai câu. Nhưng muốn biết chúng khác nhau ở
**những dòng nào**, và giống nhau ở những dòng nào, thì phải có cả tám dòng
trước mắt. Đó là việc của cái bảng.
::::

::::predict{#doan-tam-dong commitOnce}
Byte cho máy đi hết ba vế, chưa ghép câu gì cả — chỉ in ra từng dòng của bảng.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
for nam in [True, False]:
    for lan in [True, False]:
        for minh in [True, False]:
            print(nam, lan, minh)
```

:::opt{correct}
Tám dòng. Dòng đầu `True True True`, dòng cuối `False False False`, và cột `minh` là cột đổi giá trị nhanh nhất.
:::

:::opt
Sáu dòng
::why
Gần đúng ở chỗ bạn đang đếm chứ không đoán bừa, và ở chỗ bạn thấy mỗi vòng có
hai nấc — điều đó đúng, cả ba vòng đều đúng hai nấc.

Chỗ lệch là phép tính gộp ba con số ấy lại. Bạn đang cộng: 2 + 2 + 2 = 6. Nhưng
ba vòng này lồng vào nhau chứ không nối đuôi nhau — T1.2 đã chốt đúng chuyện
này ở bài vòng lặp lồng nhau: vòng trong chạy **trọn một lượt của chính nó cho
mỗi lượt vòng ngoài**, nên số lượt nhân với nhau. Ở đây là 2 × 2 × 2 = 8.
::
:::

:::opt
Tám dòng, nhưng cột `nam` là cột đổi giá trị nhanh nhất — `True True True`, rồi `False True True`, rồi `True False True`…
::why
Gần đúng ở phần khó nhất: bạn đếm ra tám dòng, tức là bạn đã nắm được chuyện số
lượt nhân với nhau chứ không cộng vào nhau. Tám dòng đúng là toàn bộ bảng.

Chỗ lệch chỉ là thứ tự, và nó lệch vì bạn cho cái tên ngoài cùng đổi nhanh
nhất. Máy làm ngược lại: vòng trong là **phần việc** của một lượt vòng ngoài,
mà phần việc thì phải xong hẳn rồi vòng ngoài mới dám nhích. Nên `minh` chạy
hết `True` rồi `False`, sau đó `lan` mới đổi một nấc — và `nam` là cái đổi chậm
nhất, đúng bốn dòng một lần.
::
:::

:::opt
Hai dòng: `True True True` rồi `False False False`
::why
Gần đúng ở chỗ bạn nhận ra ba cái tên này cùng chạy trên một danh sách hai giá
trị, nên bạn hình dung chúng bước sóng đôi với nhau — như ba người đi hàng
ngang. Với ba danh sách chạy cùng nhịp thì cách hình dung ấy chính xác, và bạn
sẽ gặp đúng cảnh đó về sau.

Chỗ lệch: ở đây ba vòng **lồng vào nhau**, không đi hàng ngang. Mà lồng vào
nhau chính là thứ bài này cần — đi hàng ngang thì chỉ được hai dòng, tức là bỏ
sót sáu tổ hợp, kể cả tổ hợp `True False True`. Bảng chân lý sinh ra để không
sót dòng nào, nên nó phải lồng.
::
:::
::::

::::code{#dung-bang-cho-hai-cach-dat-ngoac}
Bắt máy dựng đủ cả tám dòng cho **cả hai** câu, rồi để hai cột nói ra chúng
khác nhau ở đâu.

Ba vòng lồng nhau đã viết sẵn; chúng đi qua tám dòng của bảng, không sót không
lặp. Việc của bạn là điền hai câu ghép, đúng theo chú thích bên cạnh mỗi cột:

- **Cột `hop_duoc`** — "Nam có mặt **VÀ** (Lan có mặt **HOẶC** Minh có mặt)".
- **Cột `doi_ngoac`** — "(Nam có mặt **VÀ** Lan có mặt) **HOẶC** Minh có mặt".

Hai câu dùng đúng ba cái tên ấy, đúng ba chữ ấy, đúng thứ tự ấy. Chỗ khác nhau
duy nhất là cặp ngoặc.

Một luật của bài: **chép đúng câu ghi ở chú thích, kể cả cặp ngoặc.** Có những
cách viết khác cho ra cùng một cột, nhưng bài này đi tìm chính chuyện *hai chỗ
đặt ngoặc trong câu này cho hai cột khác nhau* — viết sang dạng khác là bỏ mất
cái đang cần nhìn.

Bài chấm bằng cả hai cột, và hai cột phải khác nhau. Chép một câu vào cả hai
chỗ thì hai cột trùng khít, và chuyện dấu ngoặc chọn giữa hai câu sẽ không được
chứng minh ở đâu cả.

```python title=starter
hop_duoc = []   # "Nam có mặt VÀ (Lan có mặt HOẶC Minh có mặt)"
doi_ngoac = []  # "(Nam có mặt VÀ Lan có mặt) HOẶC Minh có mặt"

for nam in [True, False]:
    for lan in [True, False]:
        for minh in [True, False]:
            hop_duoc.append(___)
            doi_ngoac.append(___)

print(len(hop_duoc))
print(hop_duoc)
print(doi_ngoac)
```

```python title=solution
hop_duoc = []   # "Nam có mặt VÀ (Lan có mặt HOẶC Minh có mặt)"
doi_ngoac = []  # "(Nam có mặt VÀ Lan có mặt) HOẶC Minh có mặt"

for nam in [True, False]:
    for lan in [True, False]:
        for minh in [True, False]:
            hop_duoc.append(nam and (lan or minh))
            doi_ngoac.append((nam and lan) or minh)

print(len(hop_duoc))
print(hop_duoc)
print(doi_ngoac)
```

```python title=test
# Câu `!=` đứng TRƯỚC. Nó canh đúng cái bẫy của bài: chép một câu vào cả hai
# chỗ trống thì hai cột trùng khít, và chuyện cặp ngoặc chọn giữa hai câu sẽ
# không còn chỗ nào lộ ra. Xếp nó sau các câu `==` thì một câu `==` trượt
# trước, và bẫy không bao giờ sập.
assert hop_duoc != doi_ngoac, "hai câu chỉ khác nhau ở chỗ đặt cặp ngoặc, mà hai cột thì phải khác nhau — trùng khít nghĩa là bạn đã chép một câu vào cả hai chỗ"
assert len(hop_duoc) == 8, "ba vế, mỗi vế hai giá trị: bảng hai vế có 4 dòng, thêm một vế nữa thì gấp đôi thành 8 — cột ngắn hơn 8 ô là đã sót dòng"
assert len(doi_ngoac) == 8, "cột thứ hai chạy trên đúng cái bảng ấy, nên nó cũng phải có đủ 8 ô"
assert hop_duoc[0] == doi_ngoac[0], "dòng 1 — cả ba bạn cùng có mặt: hai câu cùng gật ở dòng này, nên chỗ khác nhau giữa chúng không nằm ở đây"
assert hop_duoc[4] is False, "dòng 5 — Nam vắng, Lan có mặt, Minh có mặt: câu thứ nhất đòi Nam có mặt trước đã, nên ở riêng dòng này nó S"
assert doi_ngoac[4] is True, "dòng 5 — Nam vắng, Lan có mặt, Minh có mặt: câu thứ hai gật ngay khi Minh có mặt, nên ở riêng dòng này nó Đ"
assert hop_duoc == [True, True, True, False, False, False, False, False], "cột câu thứ nhất: bốn dòng đầu là bốn dòng Nam có mặt, và trong bốn dòng ấy chỉ dòng Lan lẫn Minh cùng vắng mới S; bốn dòng sau Nam vắng nên S hết"
assert doi_ngoac == [True, True, True, False, True, False, True, False], "cột câu thứ hai: nó Đ ở mọi dòng Minh có mặt, cộng thêm dòng Nam và Lan cùng có mặt mà Minh vắng"
```

:::hints
- kind: attention
  body: Mỗi chỗ trống là **một câu ghép** viết từ ba cái tên `nam`, `lan`, `minh` — không phải một giá trị Đ/S gõ sẵn. Đọc lại chú thích bên phải mỗi cột: hai chú thích ấy có đúng những chữ như nhau, đứng đúng thứ tự như nhau, và chỉ khác nhau ở chỗ đặt cặp ngoặc. Chép cả cặp ngoặc sang.
- kind: strategy
  body: "Dịch từng chữ một. Chữ VÀ là `and`, chữ HOẶC là `or`, và cặp ngoặc trong chú thích thì gõ thành cặp ngoặc trong Python. Cột thứ nhất ôm hai bạn Lan với Minh vào trong ngoặc rồi mới nối với Nam; cột thứ hai ôm Nam với Lan vào trong ngoặc rồi mới nối với Minh. Đừng bỏ ngoặc đi cho gọn — bỏ ngoặc là để cho thứ tự ngầm của máy chọn hộ, mà cả bài này nói về chuyện tự mình chọn."
- kind: one-line
  body: "Hai chỗ lần lượt là `nam and (lan or minh)` và `(nam and lan) or minh`."
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: mỗi chỗ trống phải là một câu ghép viết từ ba cái tên `nam`, `lan`, `minh`, và phải giữ đúng cặp ngoặc mà chú thích ghi — một cột nối `and` ra ngoài cùng, một cột nối `or` ra ngoài cùng; viết sang dạng khác thì hai cột vẫn ra đúng số, nhưng chuyện "cặp ngoặc chọn giữa hai câu" không còn nhìn thấy được ở đâu
  requireAst:
  # Mỗi cột một chữ `and` và một chữ `or`. Khung khởi đầu không có chữ nào,
  # nên hai luật này một mình đã chặn đáp án gõ cứng `True`/`False`.
  - kind: uses-operator, target: and, min: 2
  - kind: uses-operator, target: or, min: 2
  # Ba cái tên, mỗi tên xuất hiện một lần ở mỗi cột.
  - kind: uses-name, target: nam, min: 2
  - kind: uses-name, target: lan, min: 2
  - kind: uses-name, target: minh, min: 2
  # Hai HÌNH DẠNG ngoặc khác nhau, và đây là chỗ luật đếm không với tới được.
  # `and/or` là một chữ `and` ôm trực tiếp một chữ `or` — đúng cột thứ nhất.
  # `or/and` là chiều ngược lại — đúng cột thứ hai. Chép một câu vào cả hai
  # chỗ thì chỉ có một trong hai hình dạng, và luật này bắt được ngay cả khi
  # số dấu `and`, `or`, số lần đọc tên đều đủ.
  - kind: nesting, target: and/or, min: 1
  - kind: nesting, target: or/and, min: 1
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: ^8\n\[True, True, True, False, False, False, False, False\]\n\[True, True, True, False, True, False, True, False\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tám dòng, không sót dòng nào. Giờ hai cặp ngoặc ấy hết chỗ trốn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn có một cái bảng đi được tới mọi trường hợp. Thử nó trên một câu ghép nữa,
câu này:

> **"Nam đã nộp quỹ HOẶC Nam chưa nộp quỹ."**

Vế sau là câu ngược của vế trước — thứ bài 3 đã dựng. Dựng bảng cho câu ấy rồi
đọc cột kết quả: nó **toàn Đ**.

Và nó vẫn toàn Đ dù bạn thay câu về Nam bằng câu về Lan, về Khanh, về trời mưa,
về bất cứ chuyện gì.

Bạn dựng nhầm ở đâu đó, hay câu ấy có gì lạ?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
