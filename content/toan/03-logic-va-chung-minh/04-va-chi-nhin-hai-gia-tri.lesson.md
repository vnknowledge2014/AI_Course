---
id: toan.logic-va-chung-minh.va-chi-nhin-hai-gia-tri
title: Phép "và" chỉ nhìn hai giá trị
summary: Ghép hai mệnh đề bằng chữ "và" thì giá trị câu ghép chỉ do hai giá trị Đ/S của hai vế quyết định — không do nội dung, không do thứ tự, không do chuyện nào gây ra chuyện nào.
locale: vi
track: toan
module: logic-va-chung-minh
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [logic.conjunction]
requires: [logic.proposition, logic.truth-value, logic.negation, logic.and, logic.not, core.boolean, core.variable, core.list, core.list-append, core.list-index, core.parallel-lists, core.print-variable, ctrl.for-range]
concepts: [logic.hoi, logic.chi-nhin-gia-tri, logic.doi-cho-hai-ve]
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
Đổi chỗ hai vế thì câu nghe khác hẳn. Máy có nghe ra chỗ khác ấy không?
::::

::::explain{#cho-goi-bai-truoc-de-lai}
Bài trước để lại đúng một chỗ gợn.

Bạn ghép hai câu về Nam bằng chữ "và":

> **"Nam ngã và Nam khóc."**

rồi đổi chỗ hai vế:

> **"Nam khóc và Nam ngã."**

Đọc lên thì hai câu này khác nhau thật. Câu đầu kể một chuyện quen: ngã trước,
đau, rồi khóc. Câu sau kể một chuyện khác hẳn: đang khóc sẵn, rồi mới ngã. Tai
người Việt nghe ra ngay chỗ khác ấy, và nghe đúng — tiếng Việt có chở theo thứ
tự thời gian trong chữ "và".

Câu hỏi bỏ ngỏ là: **logic có nghe ra chỗ khác đó không?**

Muốn trả lời thì phải xét cho hết. Mỗi vế là một mệnh đề, mà mệnh đề thì mang
đúng một trong hai giá trị Đ hoặc S (bài 2). Nên đi qua từng khả năng một.
::::

::::example{#bon-buoi-cua-so-lan}
Lan giữ quyển sổ của CLB cờ vua. Bốn buổi sinh hoạt vừa rồi, sổ ghi lại hai
chuyện về Nam, mỗi chuyện một cột Đ/S:

| buổi | Nam ngã | Nam khóc |
|---|---|---|
| 1 | Đ | Đ |
| 2 | Đ | S |
| 3 | S | Đ |
| 4 | S | S |

Giờ điền thêm hai cột nữa — hai câu ghép, xuôi và ngược.

Chữ "và" đòi **cả hai** vế cùng đúng thì cả câu mới đúng. Đó là luật bạn đã gõ
suốt Realm 0: `and` chỉ cho `True` ở đúng một dòng trong bốn, dòng cả hai vế
cùng đúng.

| buổi | Nam ngã | Nam khóc | Nam ngã **VÀ** Nam khóc | Nam khóc **VÀ** Nam ngã |
|---|---|---|---|---|
| 1 | Đ | Đ | Đ | Đ |
| 2 | Đ | S | S | S |
| 3 | S | Đ | S | S |
| 4 | S | S | S | S |

Hai cột cuối trùng khít nhau, ô nào cũng khớp ô nấy.

Vì sao trùng thì nhìn dòng 2 là ra. Dòng ấy có một vế Đ và một vế S. Câu xuôi
hụt vế phải, câu ngược hụt vế trái — nhưng chữ "và" không hỏi **vế nào** hụt,
nó chỉ hỏi **có hụt vế nào không**. Hụt là không gật, hụt bên nào cũng thế.

Vậy câu trả lời là: **logic không nghe ra chỗ khác ấy.** Và đó không phải chỗ
hỏng của logic, đó là chỗ nó cố ý bỏ đi. Chữ "và" của logic bỏ lại phần thời
gian mà tiếng Việt chở theo, để giữ lại đúng một việc: gộp hai giá trị Đ/S
thành một giá trị Đ/S.
::::

::::explain{#dat-ten-va-thu-mot-cap-cau-chang-lien-quan}
Đặt tên cho phép ghép ấy:

> Ghép hai mệnh đề bằng chữ **"và"** cho ra một mệnh đề mới, gọi là **hội** của
> hai mệnh đề ấy. Hội đúng khi **cả hai** vế cùng đúng, và sai trong cả ba
> trường hợp còn lại.

Nhưng câu quan trọng của bài này chưa phải câu vừa rồi. Nó là câu này:

> Giá trị của một hội **chỉ do hai giá trị Đ/S của hai vế quyết định.** Không
> do nội dung hai vế, không do thứ tự chúng đứng, không do chuyện nào gây ra
> chuyện nào.

"Không do thứ tự" thì bảng vừa rồi đã cho thấy. Còn "không do nội dung" thì thử
bằng một cặp câu **chẳng liên quan gì tới nhau**. Cũng bốn buổi ấy, sổ của Lan
còn ghi hai chuyện nữa:

| buổi | Trời mưa | Nam đeo thẻ |
|---|---|---|
| 1 | Đ | Đ |
| 2 | Đ | S |
| 3 | S | Đ |
| 4 | S | S |

Hai chuyện này không dính dáng gì nhau: trời mưa hay tạnh chẳng ép Nam phải đeo
thẻ hay bỏ thẻ ở nhà. Nhưng bốn buổi ấy xếp ra **đúng hai cột giá trị** của
"Nam ngã" và "Nam khóc" phía trên — cột đầu Đ Đ S S, cột sau Đ S Đ S.

Nên câu ghép "Trời mưa **và** Nam đeo thẻ" phải cho ra đúng cột kết quả cũ:
Đ S S S. Không cần biết nó nói về chuyện gì. Hai giá trị vào, một giá trị ra.
::::

::::predict{#doan-bon-dong commitOnce}
Byte đem đúng chuyện đó cho máy chấm. Trong Python, "Nam ngã" là một giá trị
`True`/`False` giữ trong cái tên `nga`, "Nam khóc" giữ trong `khoc`.

Byte chạy hai câu ghép — xuôi rồi ngược — ở hai buổi khác nhau: buổi Nam vừa
ngã vừa khóc, rồi buổi Nam ngã mà không khóc.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
nga = True
khoc = True
print(nga and khoc)
print(khoc and nga)

khoc = False
print(nga and khoc)
print(khoc and nga)
```

:::opt{correct}
True, True, False, False
:::

:::opt
True, False, False, False
::why
Gần đúng ở chỗ bạn đọc trúng ba trong bốn dòng, và ở chỗ bạn đang nghe câu
tiếng Việt cho thật kỹ — kỹ hơn phần lớn người đọc. "Nam ngã rồi Nam khóc" kể
được một chuyện; "Nam khóc rồi Nam ngã" kể một chuyện khác, và bạn không muốn
gật bừa cho một chuyện chưa chắc đã xảy ra như thế.

Chỗ lệch nằm ở chữ mà tiếng Việt thêm vào mà logic không thêm: chữ **rồi**.
Chữ "và" của logic không chở thứ tự thời gian. Nó nhận hai giá trị Đ/S và gộp
lại, thế thôi. Ở dòng 2, cả `nga` lẫn `khoc` đều đang giữ `True`, nên `khoc and
nga` là `True and True` — và theo bảng bốn dòng của Realm 0, kết quả là `True`.
::
:::

:::opt
True, True, True, True
::why
Gần đúng ở chỗ bạn giữ chắc một điều thật của bài này: đổi chỗ hai vế thì kết
quả không đổi. Bốn dòng trên đúng là hai cặp, mỗi cặp hai dòng giống nhau, và
bạn nhìn ra cấu trúc ấy.

Chỗ lệch là **giá trị** của cặp thứ hai. Dòng `khoc = False` hạ vế "Nam khóc"
xuống S, nên hai dòng cuối chạy trên một buổi khác hẳn: Nam ngã mà không khóc.
Chữ "và" đòi cả hai vế cùng đúng, mà buổi này hụt mất một vế — hụt vế nào cũng
là hụt. Hai dòng cuối in ra `False`.
::
:::

:::opt
True, True, False, True
::why
Gần đúng ở chỗ bạn đọc trúng ba dòng, và ở chỗ bạn thấy hai dòng cuối chạy trên
một buổi mới — bạn không quên dòng `khoc = False`.

Chỗ lệch nằm ở dòng cuối, và nó lệch vì bạn để cho **vế đang đúng** kéo cả câu
lên. Ở `khoc and nga`, vế sau là `nga` và nó đang đúng thật. Nhưng đó là luật
của chữ "hoặc", không phải của chữ "và": bảng bốn dòng của `and` chỉ cho `True`
ở đúng một dòng, dòng **cả hai** vế cùng đúng. Một vế đúng, một vế sai thì
`and` trả `False`, đứng bên nào cũng vậy.
::
:::
::::

::::code{#ba-cot-cung-mot-ket-qua}
Bắt máy dựng ba cột cạnh nhau, rồi để chính ba cột ấy nói ra: hội chỉ nhìn hai
giá trị.

Bốn buổi trong sổ của Lan, bốn ô mỗi cột. Ba câu ghép cần điền, đúng theo chú
thích ghi bên cạnh mỗi cột:

- **Cột xuôi** — "Nam ngã **VÀ** Nam khóc".
- **Cột ngược** — "Nam khóc **VÀ** Nam ngã".
- **Cột chuyện khác** — "Trời mưa **VÀ** Nam đeo thẻ".

Hai cột đầu là cùng hai vế, đổi chỗ. Cột thứ ba là hai vế khác hẳn, nói về
chuyện khác hẳn — chỉ trùng với hai vế kia ở đúng một thứ: hai cột giá trị Đ/S.

Vế **trái** của cả ba câu ghép đã viết sẵn cho bạn; bạn điền vế phải.

Sẵn tiện nói luôn vì sao lại chừa nửa vời như thế, vì nó chính là chuyện bài
này đang dạy. Điều bài muốn bạn thấy — đổi chỗ hai vế thì cột kết quả không
đổi — làm cho hai cột đầu **bằng nhau dù bạn viết thế nào**. Nên nếu chừa cả
câu, thì viết `nga and khoc` vào cả hai chỗ cũng ra hai cột y hệt, và không có
cách nào máy biết bạn có thật sự đổi chỗ hay không. Không cách nào — không
phải "cổng chưa đủ tinh", mà là **không thể**: hai câu ấy bằng nhau ở mọi
buổi, đó đúng là điều phải chứng minh.

Chừa sẵn vế trái thì hình dạng bị ép, và lúc ấy máy mới nói được điều gì đó
thật: điền nhầm một cái tên vào bất kỳ chỗ nào trong ba chỗ, cột kết quả lệch
ngay và bài trượt.

```python title=starter
# Bốn buổi sinh hoạt, đọc từ sổ của Lan.
nam_nga = [True, True, False, False]
nam_khoc = [True, False, True, False]

# Hai chuyện khác hẳn, cũng bốn buổi ấy — và chúng xếp ra đúng hai cột trên.
troi_mua = [True, True, False, False]
nam_deo_the = [True, False, True, False]

xuoi = []         # "Nam ngã VÀ Nam khóc"
nguoc = []        # "Nam khóc VÀ Nam ngã"
chuyen_khac = []  # "Trời mưa VÀ Nam đeo thẻ"

for i in range(4):
    nga = nam_nga[i]
    khoc = nam_khoc[i]
    mua = troi_mua[i]
    deo_the = nam_deo_the[i]
    xuoi.append(nga and ___)
    nguoc.append(khoc and ___)
    chuyen_khac.append(mua and ___)

print(xuoi)
print(nguoc)
print(chuyen_khac)
```

```python title=solution
# Bốn buổi sinh hoạt, đọc từ sổ của Lan.
nam_nga = [True, True, False, False]
nam_khoc = [True, False, True, False]

# Hai chuyện khác hẳn, cũng bốn buổi ấy — và chúng xếp ra đúng hai cột trên.
troi_mua = [True, True, False, False]
nam_deo_the = [True, False, True, False]

xuoi = []         # "Nam ngã VÀ Nam khóc"
nguoc = []        # "Nam khóc VÀ Nam ngã"
chuyen_khac = []  # "Trời mưa VÀ Nam đeo thẻ"

for i in range(4):
    nga = nam_nga[i]
    khoc = nam_khoc[i]
    mua = troi_mua[i]
    deo_the = nam_deo_the[i]
    xuoi.append(nga and khoc)
    nguoc.append(khoc and nga)
    chuyen_khac.append(mua and deo_the)

print(xuoi)
print(nguoc)
print(chuyen_khac)
```

```python title=test
# Hai câu về ĐỘ DÀI đứng trước. Chúng canh cái bẫy dễ sập nhất của một bài
# dựng cột: điền thiếu một buổi thì mọi câu so cột phía dưới vẫn có thể đúng
# trên phần còn lại, và chỗ sót đi qua lặng lẽ.
assert len(xuoi) == 4, "sổ ghi bốn buổi, nên cột xuôi phải có đủ bốn ô — thiếu ô nào là bỏ sót một buổi"
assert len(chuyen_khac) == 4, "cột 'chuyện khác' cũng chạy trên đúng bốn buổi ấy, nên nó cũng phải có đủ bốn ô"
assert xuoi[0] is True, "buổi 1 — Nam ngã Đ và Nam khóc Đ: chữ VÀ đòi cả hai vế cùng đúng, mà buổi này có đủ cả hai"
assert xuoi[1] is False, "buổi 2 — Nam ngã Đ nhưng Nam khóc S: hụt vế phải, nên câu ghép ở riêng buổi này là S"
assert xuoi[2] is False, "buổi 3 — Nam ngã S còn Nam khóc Đ: lần này hụt vế trái, và hụt bên nào thì chữ VÀ cũng không gật"
assert xuoi == [True, False, False, False], "cột 'Nam ngã VÀ Nam khóc': chỉ buổi 1 có cả hai vế cùng đúng, ba buổi còn lại đều hụt ít nhất một vế"
assert nguoc == xuoi, "đổi chỗ hai vế của chữ VÀ mà cột kết quả đổi theo thì bạn đã viết ra một câu ghép khác: hụt vế trái hay hụt vế phải, chữ VÀ cũng đều không gật"
assert chuyen_khac == xuoi, "'Trời mưa' mang đúng cột giá trị của 'Nam ngã', 'Nam đeo thẻ' mang đúng cột của 'Nam khóc' — hai giá trị vào thì một giá trị ra, nên cột kết quả phải trùng"
```

:::hints
- kind: attention
  body: Mỗi chỗ trống là **vế phải** của một câu ghép, không phải một giá trị Đ/S gõ sẵn. Đọc lại chú thích nằm ngay bên phải mỗi cột: nó ghi đủ hai vế của câu ấy, mà vế trái thì dòng code đã có sẵn. Rồi nhìn bốn dòng ngay phía trên — chúng vừa lấy ra bốn cái tên, mỗi tên giữ giá trị Đ/S của một câu **tại buổi đang xét**.
- kind: strategy
  body: "Mỗi dòng đã có sẵn vế trái và chữ `and`; bạn chỉ điền vế phải. Đọc chú thích nằm bên phải mỗi cột: nó ghi đủ hai vế, và vế bạn cần là vế mà dòng ấy chưa có. Để ý cột xuôi và cột ngược dùng chung đúng hai cái tên, chỉ đảo thứ tự — đó là chỗ đáng nhìn kỹ. Cột thứ ba dùng hai cái tên còn lại."
- kind: one-line
  body: "Ba chỗ lần lượt là `khoc`, `nga`, và `deo_the`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: mỗi chỗ trống phải là một câu ghép viết từ đúng hai cái tên của cột ấy, không phải một giá trị Đ/S gõ thẳng vào — và cột thứ ba phải nhắc tới `mua` với `deo_the`, vì chuyện "hai vế khác hẳn vẫn cho cùng cột kết quả" chỉ chứng minh được khi hai vế ấy có mặt thật
  requireAst:
  # Ba câu ghép, ba chữ `and`. Khung khởi đầu không có `and` nào, nên luật này
  # một mình đã chặn mọi đáp án gõ cứng `True`/`False` vào ba chỗ trống.
  - kind: uses-operator, target: and, min: 3
  # Cột xuôi và cột ngược mỗi cột đọc `nga` một lần, nên lời giải đọc 2 lần.
  # Khung khởi đầu đọc 0 lần: dòng `nga = nam_nga[i]` là gán, không phải đọc.
  - kind: uses-name, target: nga, min: 2
  - kind: uses-name, target: khoc, min: 2
  # Cột thứ ba. Thiếu hai cái tên này thì nó chỉ là bản sao của cột xuôi, và
  # điều bài dựng lên để nói — nội dung hai vế không quyết định gì — không
  # được kiểm ở đâu cả.
  - kind: uses-name, target: mua, min: 1
  - kind: uses-name, target: deo_the, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[True, False, False, False\]\n\[True, False, False, False\]\n\[True, False, False, False\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba cột, ba câu khác chữ, một kết quả. Chữ "và" đúng là chỉ nhìn hai giá trị.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy chữ "và" bỏ lại một phần nghĩa mà tiếng Việt chở theo — phần thứ
tự thời gian — để đổi lấy một luật gọn: hai giá trị vào, một giá trị ra.

Cuối tháng CLB liên hoan. Cô chủ nhiệm dặn: *"Mỗi bạn gọi trà hoặc cà phê."*

Ở quán, **"gọi trà hoặc cà phê"** nghĩa là chọn một. Nếu bạn bê về cả hai cốc,
câu ấy còn đúng không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
