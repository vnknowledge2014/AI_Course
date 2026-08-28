---
id: toan.logic-va-chung-minh.noi-nguoc-lai-mot-cau
title: Nói ngược lại một câu
summary: Từ một mệnh đề dựng ra một mệnh đề mới luôn mang giá trị ngược lại — và trong hai câu ấy luôn có đúng một câu đúng.
locale: vi
track: toan
module: logic-va-chung-minh
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.negation]
requires: [logic.truth-value, logic.proposition, logic.not, core.boolean, ctrl.comparison, core.variable, core.dict, core.list, core.list-append, core.string-literal, core.print-variable, ctrl.for-each]
concepts: [logic.phu-dinh, logic.dung-mot-cau-dung, logic.khong-phai-doi-cuc]
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
Cùng một chuyện, Tú kể ngược lại. Hai câu ấy có bao giờ cùng đúng không?
::::

::::explain{#cau-ke-nguoc-lai-cua-tu}
Bài trước dừng ở hai câu đứng cạnh nhau:

> **"Nam đã nộp quỹ."** — sổ ghi Nam đã nộp, nên câu này đúng.
>
> **"Nam chưa nộp quỹ."** — Tú kể lại cùng chuyện ấy theo lối ngược.

Câu của Tú cũng là câu kể, cũng phân xử được, nên nó cũng mang một giá trị chân
lý. Mở sổ ra là chốt được ngay: sổ ghi Nam đã nộp, nên câu *"Nam chưa nộp quỹ"*
**sai**.

Một câu đúng, một câu sai. Và điều đó không phải chuyện may rủi của riêng Nam.
Thử với Minh — sổ ghi Minh chưa nộp:

> **"Minh đã nộp quỹ."** — sai.
>
> **"Minh chưa nộp quỹ."** — đúng.

Lại một đúng một sai, chỉ đổi bên. Đi hết sáu thành viên thì sáu lần đều thế.

Đáng để dừng lại ở đây một chút, vì tiếng Việt đang làm một việc rất khéo mà ta
hay dùng mà không để ý: nó **dựng một câu mới từ một câu cũ**. Câu mới không kể
thêm chuyện gì; nó kể đúng chuyện cũ, và nó luôn ngả về phía ngược lại.

Tiếng Việt lại có nhiều chữ để làm việc ấy, tuỳ câu:

| câu gốc | tiếng Việt nói ngược |
|---|---|
| Nam đã nộp quỹ. | Nam **chưa** nộp quỹ. |
| Nam đeo thẻ. | Nam **không** đeo thẻ. |
| Hôm nay có sinh hoạt. | Hôm nay **không** có sinh hoạt. |

Ba chữ khác nhau — "chưa", "không", "không có" — cho cùng một việc. Logic không
giữ ba chữ ấy. Nó chỉ giữ đúng một phép, và phép ấy làm được cho **mọi** mệnh
đề mà không cần biết câu ấy nói về chuyện gì.
::::

::::explain{#dat-ten-cho-phep-noi-nguoc}
Đặt tên:

> **Phủ định** của một mệnh đề P là mệnh đề dựng ra bằng cách dán *"không
> phải"* vào trước cả câu P. Viết là **không P**. Nó luôn mang giá trị **ngược
> lại** giá trị của P.

Toàn bộ luật của nó gói trong hai dòng:

| P | không P |
|---|---|
| đúng | sai |
| sai | đúng |

Bạn đã gõ đúng hai dòng này ở Realm 1, bằng chữ `not`. Cái mới ở đây không phải
`not`. Cái mới là chỗ đứng của nó: `not` hồi đó lật một **giá trị**; phủ định
hôm nay dựng một **câu** — một mệnh đề mới, có nghĩa riêng, kể được thành tiếng
Việt, và có thể đem đi hỏi người khác.

Đọc bảng hai dòng ấy theo chiều ngang thì ra một điều mà bài trước còn để ngỏ:

> Trong hai câu **P** và **không P**, luôn có **đúng một** câu đúng.

"Đúng một" nói ra hai chuyện cùng lúc, và cả hai đều cần:

- **Không bao giờ cùng đúng.** P đúng thì dòng đầu bắt "không P" phải sai.
- **Không bao giờ cùng sai.** P sai thì dòng sau bắt "không P" phải đúng.

Đó chính là chỗ hai câu ấy khác hẳn hai câu bất kỳ nhặt ra từ buổi sinh hoạt.
*"Nam đã nộp quỹ"* và *"Minh đã nộp quỹ"* có thể cùng đúng, cùng sai, hay lệch
nhau — chẳng có luật nào ràng chúng với nhau. Còn một câu với phủ định của
chính nó thì bị buộc chặt: biết một cái là biết luôn cái kia.
::::

::::example{#khong-thang-khong-co-nghia-la-thua}
Đây là chỗ trượt hay gặp nhất, và nó trượt vì một lý do rất tự nhiên: tiếng
Việt có sẵn cho nhiều chuyện một cái **đối cực**, và đối cực nghe rất giống phủ
định.

Chiều nay Nam đánh với Lan. Ván cờ kết thúc **hoà**.

Xét ba câu:

- **P** — *"Nam thắng ván này."* Ván hoà, nên P **sai**.
- **Câu đối cực** — *"Nam thua ván này."* Ván hoà, nên câu này cũng **sai**.
- **Phủ định của P** — *"Nam không thắng ván này."* Ván hoà, mà hoà thì không
  phải thắng, nên câu này **đúng**.

Hai câu giữa và cuối nghe na ná nhau trong đời thường, mà giá trị của chúng
lệch hẳn. Lý do nằm ở luật vừa dựng: P và phủ định của P **không bao giờ cùng
sai**. Ở đây *"Nam thắng"* đã sai rồi; nếu *"Nam thua"* là phủ định của nó thì
câu ấy buộc phải đúng. Nó lại sai. Vậy nó không phải phủ định.

Chỗ hụt là **ván hoà**. Cờ vua có ba kết cục chứ không phải hai, nên "thua"
không phủ kín phần còn lại của "thắng". Phủ định thì phủ kín, luôn luôn — vì nó
không đi tìm một kết cục nào cả, nó chỉ nói "chuyện kia không xảy ra".

Cách dựng phủ định cho chắc tay, dùng được cho mọi câu:

> Đừng đi tìm một chữ ngược nghĩa. Cứ giữ **nguyên văn** câu gốc rồi đặt *"không
> phải"* lên trước cả câu: *"không phải Nam thắng ván này"*. Nói xuôi tai được
> thì gọt lại thành *"Nam không thắng ván này"*; gọt không được thì cứ để
> nguyên như thế.

Một cái bẫy cùng họ, để ý luôn cho đỡ vấp về sau: phủ định của *"Hoa cao đúng 1
mét 50"* không phải *"Hoa thấp"*, mà là *"Hoa không cao đúng 1 mét 50"*. Hai câu
ấy lệch nhau ở mọi bạn cao hơn 1 mét 50: bạn ấy không cao đúng 1 mét 50 nên câu
phủ định gật, mà bạn ấy cũng không thấp nên câu đối cực lắc.
::::

::::predict{#doan-ba-dong commitOnce}
Byte đem đúng ván cờ hoà ấy cho máy chấm. `nam_thang` giữ giá trị chân lý của
câu *"Nam thắng ván này"*, `nam_thua` giữ giá trị của câu *"Nam thua ván này"*.
Ván hoà nên cả hai đều `False`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
nam_thang = False
nam_thua = False

print(not nam_thang)
print(nam_thua)
print(nam_thua == (not nam_thang))
```

:::opt{correct}
`True`, rồi `False`, rồi `False`
:::

:::opt
`True`, rồi `False`, rồi `True`
::why
Gần đúng ở chỗ bạn đọc trúng hai dòng đầu, và ở chỗ bạn đang dùng một lối nghĩ
rất tốt: hai câu nghe ngược nhau thì đem so phải khớp. Với **thật sự** một cặp
câu ngược nhau thì lối nghĩ ấy đúng hoàn toàn, và dòng cuối sẽ in `True`.

Chỗ lệch nằm ở chỗ *"Nam thua"* không phải câu ngược của *"Nam thắng"*. Ván này
hoà: `nam_thua` giữ `False`, còn `not nam_thang` lật `False` thành `True`. Đem
`False` so với `True` thì máy lắc. Và chính cái `False` ở dòng cuối là bằng
chứng cho điều cả bài này nói: đối cực không phải phủ định.
::
:::

:::opt
`False`, rồi `False`, rồi `True`
::why
Gần đúng ở chỗ bạn giữ chắc một sự thật của ván cờ này: Nam không thắng. Đúng,
ván hoà thì không ai thắng cả.

Chỗ lệch nằm ở việc `not` lật cái gì. Dòng `print(not nam_thang)` không in ra
"Nam có thắng không"; nó in ra giá trị của câu ***"Nam không thắng"***. Mà câu
ấy, trong một ván hoà, là câu **đúng** — nên máy in `True`. Chữ `not` biến một
câu sai thành một câu đúng, chứ không giữ nguyên giá trị cũ.
::
:::

:::opt
`True`, rồi `True`, rồi `True`
::why
Gần đúng ở chỗ bạn đọc dòng đầu chuẩn xác: *"Nam không thắng"* đúng là câu đúng
trong một ván hoà.

Chỗ lệch nằm ở dòng thứ hai. Nó không hỏi *"Nam có không-thua không"*; nó in
thẳng giá trị đang nằm trong `nam_thua`, tức giá trị của câu *"Nam thua ván
này"*. Ván hoà thì Nam cũng không thua, nên câu ấy sai, và máy in `False`. Ba
kết cục của một ván cờ — thắng, thua, hoà — làm cho hai câu ấy cùng sai được, và
đó là chuyện mà một cặp câu phủ định của nhau không bao giờ làm được.
::
:::
::::

::::code{#dung-cot-phu-dinh-cho-ca-sau-nguoi}
Bắt máy dựng cột phủ định cho cả sáu thành viên, rồi dùng chính cột ấy đi soát
một cuốn sổ chép tay.

Nhắc lại luật của track: **Python không phải lời giải.** Nó không biết *"chưa
nộp"* là nói ngược của *"đã nộp"* — bạn biết, và bạn viết điều đó ra. Việc của
máy là làm sáu lần cho khỏi sót.

Chuyện của chiều nay: Lan chép sổ quỹ sang một cột mới theo lối ngược lại — ở
cột ấy, `True` nghĩa là **chưa nộp**. Chép sáu dòng bằng tay thì có thể nhầm,
nên phải soát.

Hai chỗ trống, cả hai nằm trong thân vòng lặp:

- **Chỗ thứ nhất** — dựng phủ định của câu *"«tên» đã nộp quỹ"* từ giá trị `goc`
  vừa tra được.
- **Chỗ thứ hai** — cột của Lan ở dòng này có đúng là phủ định không? Đem hai
  giá trị ra so.

Bài chấm bằng cả hai cột trên **cả sáu dòng**, và cuốn sổ chép tay có đúng một
dòng lệch. Gõ cứng `True` vào chỗ thứ hai thì dòng lệch ấy biến mất, và cái
việc mà bài này giao cho bạn — đi soát — không được làm ở đâu cả.

```python title=starter
# Vẫn cuốn sổ quỹ của bài trước, không sửa dòng nào.
thanh_vien = ["Nam", "Lan", "Minh", "Hoa", "Tú", "Khanh"]
so_quy = {"Nam": True, "Lan": True, "Minh": False, "Hoa": True, "Tú": True, "Khanh": False}

# Cột Lan chép tay, theo lối NGƯỢC: True ở đây nghĩa là CHƯA nộp.
cot_chua_nop = {"Nam": False, "Lan": False, "Minh": True, "Hoa": True, "Tú": False, "Khanh": True}

cau_phu_dinh = []      # giá trị của câu "«tên» KHÔNG nộp quỹ"
khop_cot_cua_lan = []  # cột Lan chép có khớp cột phủ định không

for ten in thanh_vien:
    goc = so_quy[ten]
    nguoc = ___
    cau_phu_dinh.append(nguoc)
    khop_cot_cua_lan.append(___)

print(cau_phu_dinh)
print(khop_cot_cua_lan)
```

```python title=solution
# Vẫn cuốn sổ quỹ của bài trước, không sửa dòng nào.
thanh_vien = ["Nam", "Lan", "Minh", "Hoa", "Tú", "Khanh"]
so_quy = {"Nam": True, "Lan": True, "Minh": False, "Hoa": True, "Tú": True, "Khanh": False}

# Cột Lan chép tay, theo lối NGƯỢC: True ở đây nghĩa là CHƯA nộp.
cot_chua_nop = {"Nam": False, "Lan": False, "Minh": True, "Hoa": True, "Tú": False, "Khanh": True}

cau_phu_dinh = []      # giá trị của câu "«tên» KHÔNG nộp quỹ"
khop_cot_cua_lan = []  # cột Lan chép có khớp cột phủ định không

for ten in thanh_vien:
    goc = so_quy[ten]
    nguoc = not goc
    cau_phu_dinh.append(nguoc)
    khop_cot_cua_lan.append(nguoc == cot_chua_nop[ten])

print(cau_phu_dinh)
print(khop_cot_cua_lan)
```

```python title=test
# Dòng LỆCH đứng trước. Nó canh đúng cái bẫy của bài — dựng xong cột phủ định
# rồi gật bừa cho cả sáu dòng. Xếp nó xuống dưới thì một câu về độ dài trượt
# trước, và dòng lệch không bao giờ lộ ra.
assert khop_cot_cua_lan[3] is False, "sổ ghi Hoa ĐÃ nộp, nên câu 'Hoa không nộp quỹ' phải sai; cột Lan chép lại ghi True ở dòng Hoa, nên đúng dòng thứ tư này phải lệch"
assert khop_cot_cua_lan == [True, True, True, False, True, True], "Lan chép nhầm đúng một dòng — dòng của Hoa; năm dòng còn lại phải khớp"
assert len(cau_phu_dinh) == 6, "CLB có sáu thành viên, nên cột phủ định phải đủ sáu ô — thiếu ô nào là bỏ sót một người"
assert cau_phu_dinh[0] is False, "sổ ghi Nam đã nộp, nên câu 'Nam không nộp quỹ' mang giá trị sai"
assert cau_phu_dinh[2] is True, "sổ ghi Minh chưa nộp, nên câu 'Minh không nộp quỹ' mang giá trị đúng"
assert cau_phu_dinh == [False, False, True, False, False, True], "cột phủ định phải ngược hẳn cột của sổ ở CẢ SÁU dòng, không sót dòng nào"
assert cau_phu_dinh[0] != so_quy["Nam"], "một câu và phủ định của nó không bao giờ cùng đúng, cũng không bao giờ cùng sai — hai ô này phải khác nhau"
assert cau_phu_dinh[2] != so_quy["Minh"], "vẫn luật ấy, lần này ở phía ngược lại: Minh chưa nộp nên câu gốc sai và câu phủ định đúng"
```

:::hints
- kind: attention
  body: Cả hai chỗ trống nằm trong thân vòng, nên mỗi lượt chúng nói về **một** thành viên. Dòng ngay trên chỗ trống thứ nhất đã tra sổ hộ bạn và đặt kết quả vào `goc`. Chỗ trống thứ hai thì nhìn hai thứ: giá trị bạn vừa dựng ở dòng trên, và ô tương ứng trong cột Lan chép — mà tra cột ấy thì làm y như tra sổ quỹ.
- kind: strategy
  body: "Chỗ thứ nhất là phép nói ngược, và Realm 1 đã cho bạn đúng một từ để làm việc ấy: nó đứng TRƯỚC thứ cần lật. Đừng tra sổ lại lần nữa, cứ lật cái `goc` đang có. Chỗ thứ hai hỏi hai giá trị có bằng nhau không, nên nó cần một dấu `==` với hai vế: bên này là giá trị bạn vừa dựng, bên kia là ô của cột Lan chép ứng với cái tên đang xét. Đừng gõ thẳng `True` vào đó — nếu cả sáu dòng đều khớp thì Lan đâu có gì để soát."
- kind: one-line
  body: "Hai chỗ lần lượt là `not goc` và `nguoc == cot_chua_nop[ten]`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: chỗ thứ nhất phải LẬT giá trị `goc` bằng phép nói ngược, và chỗ thứ hai phải đem giá trị vừa dựng SO với ô tương ứng của cột Lan chép — gõ cứng `True` vào là bạn đang gật sẵn cho cả sáu dòng, mà bài này giao cho bạn việc đi soát
  requireAst:
  # Khung khởi đầu không có phép nói ngược nào và không có dấu so sánh nào,
  # nên hai luật này một mình đã chặn mọi đáp án gõ cứng `True`/`False`.
  - kind: uses-operator, target: not, min: 1
  - kind: uses-operator, target: ==, min: 1
  # Cột phủ định phải dựng TỪ giá trị vừa tra sổ, không dựng từ chỗ khác.
  - kind: uses-name, target: goc, min: 1
  # Chỗ soát phải thật sự mở cột Lan chép ra. Thiếu tên này thì dòng lệch của
  # Hoa không được đối chiếu với gì cả.
  - kind: uses-name, target: cot_chua_nop, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[False, False, True, False, False, True\]\n\[True, True, True, False, True, True\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu dòng, một dòng lệch. Cột phủ định chỉ thẳng vào chỗ Lan chép nhầm.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ bạn có hai thứ để làm việc với một câu: đọc giá trị chân lý của nó, và dựng
ra câu ngược của nó. Cả hai đều chỉ đụng tới **một** câu mỗi lần.

Nhưng chiều thứ Năm ấy còn một chuyện nữa. Trong lúc chạy ra bảng nội quy, Nam
vấp chân ngã, rồi khóc. Byte kể lại bằng một câu ghép hai vế:

> **"Nam ngã và Nam khóc."**

Tú nghe xong, kể lại theo thứ tự ngược:

> **"Nam khóc và Nam ngã."**

Đọc lên thì hai câu này khác nhau thật. Câu đầu kể một chuyện quen: ngã trước,
đau, rồi khóc. Câu sau kể một chuyện khác hẳn: đang khóc sẵn, rồi mới ngã. Tai
người Việt nghe ra ngay chỗ khác ấy.

Còn logic thì sao — nó có nghe ra chỗ khác nhau đó không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
