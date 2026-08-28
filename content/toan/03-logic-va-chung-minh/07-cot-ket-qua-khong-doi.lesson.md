---
id: toan.logic-va-chung-minh.cot-ket-qua-khong-doi
title: Khi cột kết quả không đổi
summary: Có câu ghép đúng ở mọi dòng của bảng và có câu sai ở mọi dòng — cột của chúng đứng yên, nên chuyện đời có xảy ra thế nào cũng không đụng tới chúng.
locale: vi
track: toan
module: logic-va-chung-minh
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [logic.tautology, logic.contradiction]
requires: [logic.truth-table, logic.negation, logic.conjunction, logic.disjunction, logic.and, logic.or, logic.not, logic.parentheses, core.boolean, core.variable, core.list, core.list-append, core.print-variable, ctrl.for-each, ctrl.nested-loop]
concepts: [logic.hang-dung, logic.hang-sai, logic.cot-dung-yen]
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
Cột kết quả toàn Đ. Mình dựng lại ba lần rồi, lần nào cũng ra thế.
::::

::::explain{#dung-lai-cho-cham}
Bài trước để lại một chỗ gợn. Bạn dựng bảng chân lý cho câu ghép:

> **"Nam đã nộp quỹ HOẶC Nam chưa nộp quỹ."**

và cột kết quả ra toàn Đ. Câu hỏi để lại: bạn dựng nhầm ở đâu đó, hay câu ấy có
gì lạ?

Dựng lại thật chậm, không bỏ dòng nào.

Câu này ghép hai vế, mà vế sau đúng là **câu ngược** của vế trước — thứ bài 3 đã
dựng. Nên hai vế không tự do độc lập: hễ vế trước Đ thì vế sau S, và ngược lại.
Sự việc chỉ có hai kiểu xảy ra, nên bảng có hai dòng.

| Nam đã nộp quỹ | Nam chưa nộp quỹ | Nam đã nộp quỹ **HOẶC** Nam chưa nộp quỹ |
|---|---|---|
| Đ | S | Đ |
| S | Đ | Đ |

Đọc từng dòng cho hết:

- **Dòng trên** — Nam đã nộp. Vế trái Đ, vế phải S. Chữ "hoặc" của logic đúng khi
  **ít nhất một** vế đúng (bài 5), mà ở đây vế trái đã đúng, nên cả câu Đ.
- **Dòng dưới** — Nam chưa nộp. Vế trái S, vế phải Đ. Vẫn có một vế đúng, nên cả
  câu vẫn Đ.

Hai dòng, hai lần Đ. Và bài 6 đã bảo đảm cho bạn một điều: bảng ấy **không sót
dòng nào** — sự việc không có kiểu thứ ba để mà xảy ra. Vậy không phải bạn dựng
nhầm. Cột ấy đúng là toàn Đ.
::::

::::explain{#doi-mot-chu}
Giữ nguyên hai vế, đổi đúng một chữ nối: "hoặc" thành "và".

> **"Nam đã nộp quỹ VÀ Nam chưa nộp quỹ."**

| Nam đã nộp quỹ | Nam chưa nộp quỹ | Nam đã nộp quỹ **VÀ** Nam chưa nộp quỹ |
|---|---|---|
| Đ | S | S |
| S | Đ | S |

Chữ "và" đòi **cả hai** vế cùng đúng (bài 4). Dòng trên hụt vế phải, dòng dưới
hụt vế trái. Không dòng nào đủ. Cột toàn S.

Hai cái cột vừa dựng có chung một tính chất, và tính chất ấy đáng được đặt tên:

> Câu ghép mà cột kết quả **toàn Đ** — Đ ở mọi dòng của bảng, dù các vế mang giá
> trị nào — gọi là câu **hằng đúng**. Câu mà cột kết quả **toàn S** ở mọi dòng
> gọi là câu **hằng sai**.

Chữ "hằng" ở đây nghĩa là **không đổi**. Cột của một câu ghép thường thì nhấp
nhô theo dòng; cột của hai câu này đứng yên.
::::

::::example{#cau-nao-noi-duoc-gi}
Đứng yên thì được gì, mất gì? Đặt cạnh một câu ghép thường mới thấy.

Lấy hai người: "Nam đã nộp quỹ" và "Lan đã nộp quỹ". Hai vế này tự do với nhau —
Nam nộp hay chưa chẳng ràng buộc gì Lan — nên bảng có bốn dòng.

| Nam đã nộp | Lan đã nộp | Nam đã nộp **HOẶC** Lan đã nộp |
|---|---|---|
| Đ | Đ | Đ |
| Đ | S | Đ |
| S | Đ | Đ |
| S | S | S |

Cột này có cả Đ lẫn S. Nên nghe thủ quỹ nói câu ấy **đúng**, bạn cầm được một mẩu
tin thật: dòng cuối bị loại. Bốn tình huống rút còn ba. Bạn chưa biết ai đã nộp,
nhưng bạn biết chắc **không phải cả hai cùng chưa nộp**.

Giờ nghe ai đó nói "Nam đã nộp quỹ hoặc Nam chưa nộp quỹ" là câu **đúng**. Bạn
loại được dòng nào? Không dòng nào cả — câu ấy đã Đ sẵn ở mọi dòng, nên nó không
cắt đi cái gì. Nó đúng, và nó không kể cho bạn nghe điều gì về Nam.

Câu hằng sai đi lối ngược lại: nó S ở mọi dòng, nên không tình huống nào trên
đời làm nó đúng được. Ai khẳng định "Nam đã nộp quỹ và Nam chưa nộp quỹ" thì
người ấy đang nói một câu mà sổ quỹ ghi thế nào cũng không cứu nổi.

Điểm chung của hai câu: **giá trị của chúng không do sự việc quyết định.** Muốn
biết chúng Đ hay S, bạn không cần mở sổ quỹ ra xem, không cần hỏi Nam. Nhìn hình
dạng câu là đủ.
::::

::::predict{#doan-bon-dong commitOnce}
Byte đem hai câu ấy cho máy chấm. Trong Python, "Nam đã nộp quỹ" là một giá trị
`True`/`False` giữ trong cái tên `da_nop`; "Nam chưa nộp quỹ" là `not da_nop`.

Byte chạy hai câu ghép ở **cả hai** kiểu sự việc: một lần Nam đã nộp, một lần
Nam chưa nộp.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
da_nop = True
print(da_nop or not da_nop)
print(da_nop and not da_nop)

da_nop = False
print(da_nop or not da_nop)
print(da_nop and not da_nop)
```

:::opt{correct}
True, False, True, False
:::

:::opt
True, False, False, True
::why
Gần đúng ở chỗ bạn đọc trúng hai dòng đầu, và ở chỗ bạn dùng một lối nghĩ rất
tốt cho phần lớn câu ghép: **đổi giá trị của vế thì kết quả đổi theo**. Với câu
"Nam đã nộp quỹ HOẶC Lan đã nộp quỹ" thì lối nghĩ ấy đúng — đổi Lan từ Đ sang S
là cột nhúc nhích thật.

Chỗ lệch nằm ở chỗ ở đây chỉ có **một** vế tự do, và vế kia luôn đi ngược nó.
Khi `da_nop` đổi từ `True` sang `False` thì `not da_nop` cũng đổi, từ `False`
sang `True` — hai vế đổi chỗ cho nhau. Câu "hoặc" vẫn có đúng một vế đúng, câu
"và" vẫn hụt đúng một vế. Nên hai dòng sau lặp lại y hệt hai dòng đầu.
::
:::

:::opt
True, True, True, True
::why
Gần đúng ở chỗ bạn nắm được điều quan trọng nhất của bài: hai câu này không phụ
thuộc vào việc Nam có nộp quỹ hay không, nên bốn dòng phải rơi vào hai giá trị
lặp lại. Đó đúng là hình dạng của câu trả lời.

Chỗ lệch: hai câu ấy đứng yên ở **hai chỗ khác nhau**. Câu nối bằng "hoặc" đứng
yên ở Đ, vì mỗi dòng đều có sẵn một vế đúng. Câu nối bằng "và" đứng yên ở S, vì
mỗi dòng đều hụt một vế — mà "và" thì đòi cả hai. Dòng thứ hai và dòng thứ tư in
ra `False`.
::
:::

:::opt
Máy báo lỗi ở dòng `print(da_nop and not da_nop)`, vì một câu không thể vừa đúng vừa sai
::why
Gần đúng ở chỗ bạn nói ra một sự thật hoàn toàn đúng về **sự việc**: không có
buổi nào Nam vừa đã nộp vừa chưa nộp. Giữ chặt câu đó — bài này dựng lên chính
là để đặt tên cho nó.

Chỗ lệch: câu ấy nói về sự việc, không nói về **cái máy**. Python nhận `True` và
`False` rồi ghép chúng theo đúng luật của `and`; nó chưa từng nghe tới CLB, tới
sổ quỹ, tới Nam. Ghép một giá trị với chính cái ngược của nó là hợp lệ hoàn toàn
— máy trả lời `False` và đi tiếp. Đúng cái `False` đứng yên ấy mới là cách máy
nói ra điều bạn vừa nhận thấy.
::
:::
::::

::::code{#dung-ba-cot}
Bắt máy dựng ba cột cạnh nhau, rồi để chính ba cột ấy nói ra câu nào đứng yên,
câu nào nhấp nhô.

`p` là "Nam đã nộp quỹ", `q` là "Lan đã nộp quỹ". Hai vế tự do với nhau nên bảng
có **bốn** dòng, và vòng lặp lồng của bài 6 đi hết bốn dòng ấy, không sót không
lặp.

Ba câu ghép cần điền, đúng theo chú thích ghi bên cạnh mỗi cột:

- **Câu A** — "Nam đã nộp quỹ **HOẶC** Nam chưa nộp quỹ".
- **Câu B** — "Nam đã nộp quỹ **VÀ** Nam chưa nộp quỹ".
- **Câu C** — "Nam đã nộp quỹ **HOẶC** Lan đã nộp quỹ".

Câu A và câu B không nhắc tới Lan lần nào. Chúng vẫn phải đứng trong đúng cái
bảng bốn dòng ấy — vì chỉ khi ba cột dài bằng nhau bạn mới so chúng với nhau
được, và vì đó là cách nhìn thấy câu A vẫn Đ kể cả ở những dòng Lan đổi ý.

Bài chấm bằng **cả ba cột**, và ba cột được chọn để cư xử khác hẳn nhau: một cột
toàn Đ, một cột toàn S, một cột có cả hai. Gõ cứng một giá trị vào ba chỗ trống
thì ba cột giống hệt nhau và hỏng ngay — phải viết ra câu ghép thật.

```python title=starter
cot_a = []   # câu A: "Nam đã nộp quỹ HOẶC Nam chưa nộp quỹ"
cot_b = []   # câu B: "Nam đã nộp quỹ VÀ Nam chưa nộp quỹ"
cot_c = []   # câu C: "Nam đã nộp quỹ HOẶC Lan đã nộp quỹ"

for p in [True, False]:
    for q in [True, False]:
        cot_a.append(___)
        cot_b.append(___)
        cot_c.append(___)

print(cot_a)
print(cot_b)
print(cot_c)
```

```python title=solution
cot_a = []   # câu A: "Nam đã nộp quỹ HOẶC Nam chưa nộp quỹ"
cot_b = []   # câu B: "Nam đã nộp quỹ VÀ Nam chưa nộp quỹ"
cot_c = []   # câu C: "Nam đã nộp quỹ HOẶC Lan đã nộp quỹ"

for p in [True, False]:
    for q in [True, False]:
        cot_a.append(p or not p)
        cot_b.append(p and not p)
        cot_c.append(p or q)

print(cot_a)
print(cot_b)
print(cot_c)
```

```python title=test
# Ba câu `!=` đứng TRƯỚC. Chúng canh đúng cái bẫy của bài: chép cùng một thứ
# vào ba chỗ trống thì ba cột trùng nhau hết, và cái khác biệt mà cả bài dựng
# lên để nói tới sẽ biến mất. Xếp chúng sau các câu `==` thì một câu `==` trượt
# trước, và bẫy không bao giờ sập.
assert cot_a != cot_b, "câu A nối bằng HOẶC, câu B nối bằng VÀ — hai cột không thể trùng nhau; trùng nghĩa là bạn đã chép một câu vào cả hai chỗ"
assert cot_a != cot_c, "câu C sai ở dòng cuối (Nam chưa nộp, Lan chưa nộp), còn câu A thì không sai dòng nào — hai cột phải lệch nhau ở đúng dòng ấy"
assert cot_b != cot_c, "câu C đúng ở ba dòng đầu, còn câu B thì không đúng dòng nào — hai cột không thể trùng nhau"
assert len(cot_a) == 4, "hai vế tự do, mỗi vế hai giá trị: bảng có 4 dòng, nên mỗi cột phải đủ 4 ô"
assert cot_a == [True, True, True, True], "câu A đúng ở cả bốn dòng: trong 'Nam đã nộp' và 'Nam chưa nộp' luôn có một vế đúng, kể cả ở hai dòng Lan chưa nộp"
assert cot_b == [False, False, False, False], "câu B sai ở cả bốn dòng: không dòng nào cho 'Nam đã nộp' và 'Nam chưa nộp' cùng đúng, mà chữ VÀ thì đòi cả hai"
assert cot_c == [True, True, True, False], "câu C: ba dòng đầu còn ít nhất một người đã nộp nên câu đúng; dòng cuối cả Nam lẫn Lan đều chưa nộp nên câu sai"
```

:::hints
- kind: attention
  body: Mỗi chỗ trống là **một câu ghép**, không phải một giá trị Đ/S gõ sẵn. Đọc lại chú thích nằm ngay bên phải mỗi cột: nó ghi đủ hai vế và chữ nối của câu ấy. Và để ý hai cái tên đang có trong tay ở mỗi lượt — `p` là câu về Nam, `q` là câu về Lan.
- kind: strategy
  body: "Dịch từng chữ một. Chữ HOẶC là `or`, chữ VÀ là `and`, còn \"Nam chưa nộp quỹ\" là câu ngược của \"Nam đã nộp quỹ\" nên nó viết bằng `not` đặt trước `p`. Câu A và câu B đều ghép `p` với câu ngược của chính `p`; chỉ mỗi chữ nối là khác. Câu C thì ghép hai người khác nhau."
- kind: one-line
  body: "Ba chỗ lần lượt là `p or not p`, `p and not p`, và `p or q`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: mỗi chỗ trống phải là một câu ghép viết theo `p` và `q`, không phải một giá trị Đ/S gõ thẳng vào — gõ cứng thì ba cột giống hệt nhau và chuyện "cột đứng yên hay nhấp nhô" không được chứng minh ở đâu cả
  requireAst:
  # Câu A và câu B đều cần chữ "chưa nộp", tức một `not` mỗi câu. Khung khởi
  # đầu không có `not` nào, nên luật này một mình đã chặn mọi đáp án gõ cứng
  # `True`/`False` vào hai chỗ trống ấy.
  - kind: uses-operator, target: not, min: 2
  # Câu A và câu C đều nối bằng HOẶC; khung khởi đầu không có `or` nào.
  - kind: uses-operator, target: or, min: 2
  # Câu B nối bằng VÀ. Không có `and` nào nghĩa là câu B chưa được viết ra —
  # và câu B chính là câu hằng sai, nửa mà cả bài này dựng lên để dạy.
  - kind: uses-operator, target: and, min: 1
  # Ba câu đều nói về Nam: 2 lần trong câu A, 2 lần trong câu B, 1 lần trong
  # câu C. Khung khởi đầu đọc `p` 0 lần (dòng `for p in ...` là gán, không
  # phải đọc), nên con số 5 đo đúng phần người học viết ra.
  - kind: uses-name, target: p, min: 5
  # Riêng câu C nói tới Lan. Thiếu nó thì cột C không còn là cột của một câu
  # ghép hai người, và nó sẽ trùng cột A.
  - kind: uses-name, target: q, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[True, True, True, True\]\n\[False, False, False, False\]\n\[True, True, True, False\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cột đứng yên, một cột nhấp nhô. Giờ mình biết cái nào đang nói chuyện gì.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

"Nam đã nộp quỹ **VÀ** Nam chưa nộp quỹ" — cột toàn S, và nó toàn S dù bạn thay
câu về Nam bằng câu về ai, về chuyện gì. Cột ấy đứng yên ở đáy.

Nhưng có một câu thứ ba, và nó không chịu đứng yên ở đâu cả:

> **"KHÔNG PHẢI là Nam chưa nộp quỹ."**

Dựng bảng cho nó thử xem. Nó lật giá trị của Nam một lần, rồi lật thêm một lần
nữa. Cột ra được là:

| Nam đã nộp quỹ | Nam chưa nộp quỹ | KHÔNG PHẢI là Nam chưa nộp quỹ |
|---|---|---|
| Đ | S | Đ |
| S | Đ | S |

Không toàn Đ, cũng không toàn S. Cột ấy **trùng khít cột của "Nam đã nộp quỹ"** —
ô nào cũng khớp ô nấy.

Hai câu viết khác chữ hẳn nhau mà cùng một cột. Vậy chúng có thay được cho nhau
không — nghĩa là ở bất kỳ chỗ nào đang có câu này, bạn đặt câu kia vào thì cả câu
dài bên ngoài vẫn giữ nguyên giá trị?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
