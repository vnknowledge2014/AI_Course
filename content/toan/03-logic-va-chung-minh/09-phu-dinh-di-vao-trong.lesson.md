---
id: toan.logic-va-chung-minh.phu-dinh-di-vao-trong
title: Phủ định đi vào trong thì "và" hoá "hoặc"
summary: Đẩy chữ "không phải" từ ngoài ngoặc vào trong thì mỗi vế bị lật và chữ nối đổi mặt — "và" thành "hoặc", "hoặc" thành "và".
locale: vi
track: toan
module: logic-va-chung-minh
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.de-morgan]
requires: [logic.equivalence, logic.truth-table, logic.tautology, logic.negation, logic.conjunction, logic.disjunction, logic.and, logic.or, logic.not, logic.parentheses, core.boolean, core.variable, core.list, core.list-append, core.print-variable, ctrl.for-each, ctrl.nested-loop]
concepts: [logic.de-morgan, logic.phu-dinh-vao-trong, logic.lat-chu-noi]
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
Mình đẩy chữ "không phải" vào trong ngoặc, giữ nguyên chữ "và" — và bảng đổi mất.
::::

::::explain{#hai-cach-doc}
Bài trước để lại một chỗ cãi nhau.

Thủ quỹ nhìn sổ rồi nói:

> **"Không phải cả Nam và Lan đều đã nộp quỹ."**

Nhiều người nghe xong hiểu thành:

> **"Cả hai đều chưa nộp."**

Cách hiểu ấy đi theo một lối rất tự nhiên: chữ "không phải" đứng ngoài, đẩy nó
vào cho mỗi người một chữ, còn chữ nối thì để nguyên. Nghe xuôi tai. Bài 8 vừa
cho bạn cách phân xử mà không cần cãi bằng cảm giác — dựng hai bảng, đặt hai cột
cạnh nhau, đi tìm một dòng lệch.

`p` là "Nam đã nộp quỹ", `q` là "Lan đã nộp quỹ". Hai người tự do với nhau nên
bảng có bốn dòng.

| Nam đã nộp | Lan đã nộp | cả Nam và Lan đều đã nộp | **KHÔNG PHẢI** cả Nam và Lan đều đã nộp | cả hai đều **chưa** nộp |
|---|---|---|---|---|
| Đ | Đ | Đ | S | S |
| Đ | S | S | **Đ** | **S** |
| S | Đ | S | **Đ** | **S** |
| S | S | S | Đ | Đ |

Đọc lại cột thứ tư cho chắc. "Cả Nam và Lan đều đã nộp" là câu ghép bằng chữ
"và", nên nó chỉ đúng ở dòng đầu (bài 4). Câu ngược của nó thì sai ở đúng dòng
đầu và đúng ở ba dòng còn lại (bài 3).

Cột thứ năm là cách đọc kia: "Nam chưa nộp" **và** "Lan chưa nộp". Chữ "và" đòi
cả hai vế cùng đúng, mà cả hai cùng chưa nộp thì chỉ xảy ra ở dòng cuối.

Hai cột lệch nhau ở **hai** dòng — dòng hai và dòng ba, hai buổi mà đúng một
người quên nộp. Theo bài 8, một dòng lệch đã đủ để nói hai câu không thay được
cho nhau. Ở đây có hai dòng.

Nên cách đọc quen thuộc ấy **không** phải phủ định của "cả hai đều đã nộp". Nó là
một câu khác, nói một chuyện khác, chặt hơn hẳn.
::::

::::explain{#cau-nao-moi-dung}
Nhìn lại đúng hai dòng lệch. Ở dòng hai, Nam đã nộp còn Lan chưa. Ở dòng ba, Lan
đã nộp còn Nam chưa. Cả hai buổi ấy, câu "cả hai đều đã nộp" hỏng — mà nó hỏng vì
**một** người, không phải vì cả hai.

Vậy nói ngược câu "cả hai đều đã nộp" là nói: có người chưa nộp. Nam chưa,
**hoặc** Lan chưa, hoặc cả hai. Đúng chữ "hoặc" của bài 5 — chữ hoặc gồm cả hai.

Dựng thử cột ấy:

| Nam đã nộp | Lan đã nộp | **KHÔNG PHẢI** cả Nam và Lan đều đã nộp | Nam chưa nộp **HOẶC** Lan chưa nộp |
|---|---|---|---|
| Đ | Đ | S | S |
| Đ | S | Đ | Đ |
| S | Đ | Đ | Đ |
| S | S | Đ | Đ |

Trùng khít cả bốn dòng. Theo bài 8, hai câu ấy thay được cho nhau ở bất kỳ chỗ
nào.

Và cái cách đọc nhầm lúc nãy cũng không vứt đi đâu cả — nó là phủ định của một
câu khác. Dựng nốt cặp còn lại:

| Nam đã nộp | Lan đã nộp | **KHÔNG PHẢI** (Nam đã nộp **HOẶC** Lan đã nộp) | Nam chưa nộp **VÀ** Lan chưa nộp |
|---|---|---|---|
| Đ | Đ | S | S |
| Đ | S | S | S |
| S | Đ | S | S |
| S | S | Đ | Đ |

Lại trùng khít. "Không có ai trong hai người đã nộp" đúng là "Nam chưa nộp và Lan
chưa nộp".

Xếp hai cặp cạnh nhau thì thấy chúng đi theo cùng một khuôn:

> **Luật De Morgan.** Đẩy chữ "không phải" từ ngoài ngoặc vào trong thì hai việc
> xảy ra cùng lúc: **mỗi vế bị lật**, và **chữ nối đổi mặt**.
>
> - KHÔNG PHẢI (P **và** Q) trùng khít với (không P) **hoặc** (không Q).
> - KHÔNG PHẢI (P **hoặc** Q) trùng khít với (không P) **và** (không Q).

Chỗ mà cách đọc quen thuộc bỏ sót chính là nửa sau: nó lật hai vế xong rồi để
nguyên chữ nối. Lật một nửa thì ra một câu khác.

Có một cách nhớ không phải học thuộc. Muốn phá câu "cả hai đều đã nộp", bạn phá
được bằng cách bắt lỗi **một** người — nên câu ngược của nó nhẹ, và chữ "hoặc"
đúng là chữ nhẹ. Ngược lại, muốn phá câu "có ít nhất một người đã nộp", bạn phải
chỉ ra **cả hai** người đều chưa — nên câu ngược của nó nặng, và chữ "và" đúng là
chữ nặng. Chữ nối đổi mặt vì gánh nặng đổi vai.
::::

::::example{#dem-luat-ra-cho}
Đem luật ra khỏi CLB một chút, để thấy nó không dính vào chuyện sổ quỹ.

Thầy giám thị đứng ở cổng trường, nhìn Khanh rồi ghi vào sổ: *"Không phải Khanh
vừa mặc đồng phục vừa đeo bảng tên."* Gọi P là "Khanh mặc đồng phục", Q là
"Khanh đeo bảng tên". Câu bị phủ định có dạng P **và** Q, nên đẩy chữ "không
phải" vào trong cho ra:

> Khanh **không** mặc đồng phục **hoặc** **không** đeo bảng tên.

Nghĩa là: dòng thầy ghi đúng khi Khanh thiếu **một** trong hai thứ. Khanh thiếu
cả hai thì dòng ấy vẫn đúng — chữ "hoặc" của bài 5 gồm cả hai.

Chiều kia. Byte viết vào sổ: *"Chiều nay mình không đi tập cờ, cũng không đi
thư viện."* Câu ấy phủ định một câu ghép bằng chữ "hoặc" — "mình đi tập cờ hoặc
mình đi thư viện" — nên đẩy vào trong thì được:

> mình **không** đi tập cờ **và** mình **không** đi thư viện.

Đây đúng là chỗ tiếng Việt và logic gặp nhau chứ không cãi nhau: chữ "cũng
không" trong câu gốc đã làm sẵn việc đổi chữ nối, và người Việt nói câu ấy hằng
ngày mà không ai nhầm.

Một chỗ dễ va, đáng dừng lại. Luật này nói về **chữ nối bên trong ngoặc**, không
nói rằng cứ có chữ "không phải" thì câu đổi nghĩa theo kiểu nào đó. Nếu câu bên
trong ngoặc chỉ có một vế thì chẳng có chữ nối nào để đổi, và bạn quay về đúng
bài 3: lật một lần là ra câu ngược, lật hai lần là về chỗ cũ.
::::

::::predict{#doan-ba-cau commitOnce}
Byte hỏi thẳng máy, ở một buổi cụ thể: Nam đã nộp quỹ, Lan thì chưa.

Ba dòng in ra ba câu: câu thủ quỹ nói, cách đọc quen thuộc, và câu mà bảng vừa
chỉ ra là trùng khít.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
p = True     # Nam đã nộp quỹ
q = False    # Lan chưa nộp quỹ

print(not (p and q))
print(not p and not q)
print(not p or not q)
```

:::opt{correct}
True, False, True
:::

:::opt
True, True, True
::why
Gần đúng ở chỗ bạn đọc trúng dòng đầu và dòng cuối, và ở chỗ bạn dùng đúng cái
lối nghĩ mà cả bài này dựng lên để soi: đẩy chữ "không phải" vào trong, lật mỗi
vế, giữ nguyên chữ nối. Với rất nhiều câu tiếng Việt thì lối nghĩ ấy nghe xuôi.

Chỗ lệch nằm ở dòng thứ hai. `not p` là `False` — Nam đã nộp, nên "Nam chưa nộp"
sai. Chữ `and` đòi cả hai vế cùng đúng, mà vế đầu đã sai, nên cả câu là `False`.
Buổi này đúng là một trong hai buổi mà hai cột lệch nhau: câu thủ quỹ nói thì
đúng, còn "cả hai đều chưa nộp" thì sai, vì Nam đã nộp rồi.
::
:::

:::opt
False, False, True
::why
Gần đúng ở chỗ bạn đọc trúng hai dòng sau, và ở chỗ bạn đang cẩn thận với thứ tự
tính — chuyện mà bài 6 đã bắt bạn để mắt tới.

Chỗ lệch: cặp ngoặc ở dòng đầu đặt chữ `and` vào **bên trong**, nên máy tính
`p and q` trước rồi mới lật kết quả. `p and q` là `True and False`, tức `False`;
lật `False` được `True`. Nếu bỏ cặp ngoặc đi thì mới thành `not p` rồi mới `and q`
— một câu khác hẳn, và nó cho `False` đúng như bạn đoán. Cặp ngoặc ấy là chỗ khác
nhau giữa hai câu.
::
:::

:::opt
True, False, False
::why
Gần đúng ở chỗ bạn đọc trúng hai dòng đầu — cả dòng khó nhất. Và ở chỗ bạn giữ
một thói quen tốt: gặp chữ "không phải" thì tìm cách gom nó lại một chỗ.

Chỗ lệch nằm ở chỗ gom. Dòng thứ ba là `not p or not q`, hai chữ `not` đứng riêng
ở hai vế, chứ không phải `not (p or q)` với một chữ `not` ôm cả ngoặc. Máy tính
`not p` được `False`, `not q` được `True`; chữ `or` cần ít nhất một vế đúng, mà vế
sau đúng, nên cả câu là `True`. Đúng cặp ngoặc ấy là thứ cả bài hôm nay nói tới:
kéo chữ `not` ra ngoài ngoặc mà không đổi chữ nối là đổi sang một câu khác.
::
:::
::::

::::code{#bon-cot-hai-cap}
Bắt máy dựng bốn cột cùng một lúc, rồi để chính chúng nói ra hai cặp nào trùng
khít.

`p` là "Nam đã nộp quỹ", `q` là "Lan đã nộp quỹ". Bảng bốn dòng của bài 6 đi hết
mọi kiểu sự việc, không sót không lặp.

Bốn chỗ cần điền, đúng theo chú thích ghi bên cạnh mỗi cột:

- **`phu_dinh_va`** — "KHÔNG PHẢI (Nam đã nộp quỹ VÀ Lan đã nộp quỹ)". Chữ
  "không phải" ôm cả ngoặc.
- **`hoac_hai_phu`** — "Nam chưa nộp quỹ HOẶC Lan chưa nộp quỹ". Hai chữ phủ định
  nằm riêng ở hai vế.
- **`phu_dinh_hoac`** — "KHÔNG PHẢI (Nam đã nộp quỹ HOẶC Lan đã nộp quỹ)".
- **`va_hai_phu`** — "Nam chưa nộp quỹ VÀ Lan chưa nộp quỹ".

Hai cột đầu là một cặp, hai cột sau là cặp kia. Bốn cột này không thể viết giống
nhau: cặp đầu cho ra một cột, cặp sau cho ra cột khác hẳn, và bài chấm bằng cả
bốn. Gõ cứng một giá trị Đ/S vào thì bốn cột trùng nhau hết và hai cặp không còn
gì để so.

```python title=starter
phu_dinh_va = []     # KHÔNG PHẢI (Nam đã nộp quỹ VÀ Lan đã nộp quỹ)
hoac_hai_phu = []    # Nam chưa nộp quỹ HOẶC Lan chưa nộp quỹ
phu_dinh_hoac = []   # KHÔNG PHẢI (Nam đã nộp quỹ HOẶC Lan đã nộp quỹ)
va_hai_phu = []      # Nam chưa nộp quỹ VÀ Lan chưa nộp quỹ

for p in [True, False]:
    for q in [True, False]:
        phu_dinh_va.append(___)
        hoac_hai_phu.append(___)
        phu_dinh_hoac.append(___)
        va_hai_phu.append(___)

print(phu_dinh_va == hoac_hai_phu)
print(phu_dinh_hoac == va_hai_phu)
print(phu_dinh_va)
print(va_hai_phu)
```

```python title=solution
phu_dinh_va = []     # KHÔNG PHẢI (Nam đã nộp quỹ VÀ Lan đã nộp quỹ)
hoac_hai_phu = []    # Nam chưa nộp quỹ HOẶC Lan chưa nộp quỹ
phu_dinh_hoac = []   # KHÔNG PHẢI (Nam đã nộp quỹ HOẶC Lan đã nộp quỹ)
va_hai_phu = []      # Nam chưa nộp quỹ VÀ Lan chưa nộp quỹ

for p in [True, False]:
    for q in [True, False]:
        phu_dinh_va.append(not (p and q))
        hoac_hai_phu.append(not p or not q)
        phu_dinh_hoac.append(not (p or q))
        va_hai_phu.append(not p and not q)

print(phu_dinh_va == hoac_hai_phu)
print(phu_dinh_hoac == va_hai_phu)
print(phu_dinh_va)
print(va_hai_phu)
```

```python title=test
# Câu đầu canh đúng cái bẫy của bài, nên nó chạy TRƯỚC. Xếp nó sau các câu
# `==` thì một câu `==` trượt trước, và bẫy không bao giờ sập.
#
#   · bẫy — đọc "không phải cả hai đều đã nộp" thành "cả hai đều chưa nộp",
#     tức là điền cùng một câu vào `phu_dinh_va` và `va_hai_phu`. Hai cột ấy
#     phải KHÁC nhau, và chúng khác ở đúng hai dòng có một người quên nộp.
assert phu_dinh_va != va_hai_phu, "'không phải cả hai đều đã nộp' và 'cả hai đều chưa nộp' cho ra hai cột khác nhau: ở hai dòng có đúng một người đã nộp, câu trước đúng còn câu sau sai"
assert len(phu_dinh_va) == 4, "hai vế tự do, mỗi vế hai giá trị: bảng có 4 dòng, nên mỗi cột phải đủ 4 ô"
assert phu_dinh_va == [False, True, True, True], "câu 'cả hai đều đã nộp' chỉ đúng ở dòng đầu, nên câu ngược của nó sai ở dòng đầu và đúng ở ba dòng còn lại"
assert hoac_hai_phu == [False, True, True, True], "'Nam chưa nộp HOẶC Lan chưa nộp' sai ở dòng đầu (cả hai đã nộp) và đúng ở ba dòng còn lại, vì mỗi dòng ấy có ít nhất một người chưa nộp"
assert phu_dinh_hoac == [False, False, False, True], "'Nam đã nộp HOẶC Lan đã nộp' đúng ở ba dòng đầu, nên câu ngược của nó chỉ đúng ở dòng cuối"
assert va_hai_phu == [False, False, False, True], "'Nam chưa nộp VÀ Lan chưa nộp' đòi cả hai cùng chưa nộp, chuyện chỉ xảy ra ở dòng cuối"
assert phu_dinh_va == hoac_hai_phu, "đẩy 'không phải' vào trong ngoặc của một câu VÀ thì hai vế bị lật và chữ nối thành HOẶC — nên hai cột này phải khớp từng ô"
assert phu_dinh_hoac == va_hai_phu, "đẩy 'không phải' vào trong ngoặc của một câu HOẶC thì hai vế bị lật và chữ nối thành VÀ — nên hai cột này phải khớp từng ô"
```

:::hints
- kind: attention
  body: Đọc kỹ chú thích bên phải mỗi cột, và để mắt vào **cặp ngoặc**. Hai cột có chữ "KHÔNG PHẢI" đứng trước cả ngoặc — một chữ phủ định ôm lấy cả câu ghép. Hai cột kia có chữ "chưa" nằm trong từng vế — mỗi vế một chữ phủ định riêng.
- kind: strategy
  body: "Dịch từng chữ một. Chữ VÀ là `and`, chữ HOẶC là `or`, chữ \"không phải\"/\"chưa\" là `not`. Với hai cột đầu tiên của mỗi cặp, viết câu ghép trong ngoặc trước rồi đặt một chữ `not` trước cả cặp ngoặc ấy. Với hai cột còn lại, đặt `not` trước từng vế rồi mới nối hai vế lại."
- kind: one-line
  body: "Bốn chỗ lần lượt là `not (p and q)`, `not p or not q`, `not (p or q)`, và `not p and not q`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: mỗi chỗ trống phải là một câu ghép viết theo `p` và `q`, và cặp ngoặc phải nằm đúng chỗ — một chữ `not` ôm cả ngoặc ở hai cột, hai chữ `not` nằm riêng từng vế ở hai cột kia; gõ cứng Đ/S hay chép cùng một câu vào hai chỗ thì bài không so được cặp nào với cặp nào
  requireAst:
  # Đếm hết: 1 (ôm ngoặc VÀ) + 2 (hai vế) + 1 (ôm ngoặc HOẶC) + 2 (hai vế).
  # Khung khởi đầu không có `not` nào, nên luật này một mình chặn mọi đáp án
  # gõ cứng `True`/`False`.
  - kind: uses-operator, target: not, min: 6
  # Một chữ `not` phải ôm TRỰC TIẾP một câu ghép bằng VÀ — đó là cột
  # `phu_dinh_va`. `not p and not q` không khớp luật này, vì ở đó chữ `and`
  # nằm ngoài, `not` nằm trong.
  - kind: nesting, target: not/and, min: 1
  # Và một chữ `not` phải ôm trực tiếp một câu ghép bằng HOẶC — cột
  # `phu_dinh_hoac`.
  - kind: nesting, target: not/or, min: 1
  # Hai cột nối bằng VÀ: `p and q` bên trong ngoặc, và `not p and not q`.
  - kind: uses-operator, target: and, min: 2
  # Hai cột nối bằng HOẶC: `not p or not q`, và `p or q` bên trong ngoặc.
  - kind: uses-operator, target: or, min: 2
  # Nam có mặt trong cả bốn câu, Lan cũng vậy. Khung khởi đầu đọc `p` và `q`
  # 0 lần (hai dòng `for` là gán, không phải đọc), nên hai con số này đo đúng
  # phần người học viết ra.
  - kind: uses-name, target: p, min: 4
  - kind: uses-name, target: q, min: 4
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\nTrue\n\[False, True, True, True\]\n\[False, False, False, True\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chữ "không phải" đi vào trong, và chữ nối đổi mặt theo. Hai cặp, hai lần khớp.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Tới đây bạn đã ghép câu bằng ba chữ: "không", "và", "hoặc". Ba chữ ấy đủ để nói
về sổ quỹ, về thẻ, về đồng phục.

Nhưng bảng nội quy dán ở cửa phòng CLB có một dòng không dùng chữ nào trong ba
chữ ấy:

> **"Nếu là thành viên thì phải đeo thẻ."**

Dòng này cũng ghép hai vế — "là thành viên" và "đeo thẻ" — nhưng chữ nối không
phải "và", cũng không phải "hoặc". Thử đặt nó vào bảng bốn dòng quen thuộc mà
xem: mỗi dòng là một người, với hai câu hỏi "có phải thành viên không" và "có
đeo thẻ không".

**Khi nào** thì dòng nội quy ấy bị coi là sai?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
