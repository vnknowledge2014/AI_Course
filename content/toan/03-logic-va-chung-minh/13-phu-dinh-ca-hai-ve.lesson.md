---
id: toan.logic-va-chung-minh.phu-dinh-ca-hai-ve
title: Phủ định cả hai vế
summary: Giữ nguyên thứ tự hai vế mà phủ định cả hai thì được một câu khác câu gốc — và cột của nó trùng khít cột của mệnh đề đảo.
locale: vi
track: toan
module: logic-va-chung-minh
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [logic.inverse]
requires: [logic.converse, logic.implication, logic.vacuous-truth, logic.equivalence, logic.truth-table, logic.negation, logic.conjunction, logic.not, logic.and, core.boolean, core.variable, core.list, core.list-append, core.print-variable, core.function-def, core.function-call, core.function-parameter, core.function-return, ctrl.for-each, ctrl.nested-loop, ctrl.if]
concepts: [logic.menh-de-phan, logic.phu-dinh-hai-ve, logic.trung-bang-voi-dao]
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
Đổi chỗ hai vế thì ra câu khác. Còn giữ nguyên chỗ mà thêm chữ "không" thì sao?
::::

::::explain{#cau-hoi-bai-truoc}
Bài trước bạn cầm một dòng nội quy, đổi chỗ hai vế của nó, và thấy câu mới
**không phải** câu cũ. Câu hỏi để lại là một cách làm khác: giữ nguyên thứ tự
hai vế, nhưng phủ định cả hai.

> **"Nếu không phải thành viên thì không phải đeo thẻ."**

Câu này có phải câu gốc không? Và bảng của nó giống bảng của ai?

Trước khi dựng bảng, gọi tên cho gọn. Dòng trên tấm bảng nội quy của CLB cờ vua
lớp 6A là cách viết gọn sáu câu — mỗi thành viên một câu. Suốt bài này ta cầm
đúng **một** câu trong sáu ấy, câu về Nam:

- **P** — "Nam là thành viên CLB."
- **Q** — "Nam đeo thẻ."

Bốn câu họ hàng, viết bằng hai cái tên ấy:

| tên gọi | câu | dạng |
|---|---|---|
| câu gốc | Nếu Nam là thành viên thì Nam đeo thẻ. | nếu **P** thì **Q** |
| câu đảo *(bài 12)* | Nếu Nam đeo thẻ thì Nam là thành viên. | nếu **Q** thì **P** |
| câu phản *(hôm nay)* | Nếu Nam không phải thành viên thì Nam không đeo thẻ. | nếu **không P** thì **không Q** |

Câu thứ ba có tên riêng trong nghề:

> **Mệnh đề phản** của một câu "nếu... thì" là câu dựng bằng cách giữ nguyên
> thứ tự hai vế và **phủ định cả hai vế**.

Chữ "phủ định" là công cụ bài 3: từ một câu, dựng một câu luôn mang giá trị
ngược lại. Ở đây nó được đem áp lên **từng vế một**, trước khi hai vế được nối
lại bằng chữ "nếu... thì".
::::

::::explain{#dung-bang-bon-dong}
Hai vế này tự do với nhau. Nam có thể là thành viên hoặc không; Nam có thể đeo
thẻ hoặc không. Và bốn kiểu ghép ấy đều xảy ra được ngoài đời — kể cả kiểu nghe
lạ nhất, người không phải thành viên mà vẫn đang đeo thẻ: trường phát thẻ khách
cho ai vào phòng thi đấu ngồi xem. Bài 11 đã dừng lại đúng ở kiểu ấy, khi hỏi
rằng thấy một người đeo thẻ thì đã kết luận được người đó là thành viên chưa.

Hai vế tự do, mỗi vế hai giá trị, nên bảng có **bốn** dòng — bài 6 bảo đảm bốn
dòng ấy không sót không lặp.

Luật của chữ "nếu... thì" là luật bài 10: câu ấy **sai ở đúng một dòng** — vế
trước đúng mà vế sau sai. Mọi dòng khác nó đúng.

| dòng | P: Nam là thành viên | Q: Nam đeo thẻ | không P | không Q | câu gốc | câu đảo | câu phản |
|---|---|---|---|---|---|---|---|
| 1 | Đ | Đ | S | S | Đ | Đ | Đ |
| 2 | Đ | S | S | Đ | **S** | Đ | Đ |
| 3 | S | Đ | Đ | S | Đ | **S** | **S** |
| 4 | S | S | Đ | Đ | Đ | Đ | Đ |

Đi chậm qua dòng 3, vì đó là dòng đắt nhất:

- **Câu gốc** ở dòng 3: vế trước là "Nam là thành viên", mà dòng 3 ghi S. Vế
  trước sai thì không ai phá được câu — bài 11. Câu gốc **đúng**.
- **Câu phản** ở dòng 3: vế trước của nó là "Nam **không** phải thành viên", và
  dòng 3 ghi cho câu ấy Đ. Vế sau của nó là "Nam **không** đeo thẻ", dòng 3 ghi
  S. Vế trước đúng, vế sau sai — đúng cái dòng duy nhất làm "nếu... thì" sai.
  Câu phản **sai**.

Vậy có một buổi sáng mà câu gốc đúng còn câu phản sai. Hai câu ấy không thể là
một câu. Đọc cả cột thì thấy chúng lệch nhau ở **hai** dòng: dòng 2 và dòng 3.
::::

::::example{#hai-cot-trung-nhau}
Giờ đặt cột **câu đảo** cạnh cột **câu phản**:

| dòng | câu đảo | câu phản |
|---|---|---|
| 1 | Đ | Đ |
| 2 | Đ | Đ |
| 3 | S | S |
| 4 | Đ | Đ |

Ô nào cũng khớp ô nấy. Hai câu viết khác chữ hẳn nhau — một câu nói về việc đeo
thẻ trước, một câu nói về việc không phải thành viên trước — mà cột của chúng
trùng khít.

Bài 8 đã cho chuyện này một cái tên và một quyền: hai câu trùng bảng thì **thay
được cho nhau** ở bất kỳ chỗ nào mà không đổi giá trị của câu bao ngoài. Nên câu
đảo và câu phản luôn đi cùng số phận: buổi nào câu này đúng thì câu kia cũng
đúng, buổi nào câu này sai thì câu kia cũng sai.

Hãy đọc câu đó cho đúng chiều. Nó **không** nói rằng câu phản sai — dòng 1, 2, 4
nó đúng đấy chứ. Nó cũng không nói câu phản vô dụng. Nó nói: nếu bạn đã biết câu
đảo đúng hay sai, thì bạn biết luôn câu phản, không phải kiểm lại lần nữa.
::::

::::predict{#doan-hai-cot commitOnce}
Byte đem hai cột ấy cho máy dựng lại. Hàm `keo_theo` viết đúng luật bài 10: câu
"nếu... thì" chỉ trả `False` ở một dòng — vế trước đúng mà vế sau sai.

`p` là câu "Nam là thành viên", `q` là câu "Nam đeo thẻ". Vòng lặp lồng của bài 6
đi hết bốn dòng.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

cot_dao = []
cot_phan = []

for p in [True, False]:
    for q in [True, False]:
        cot_dao.append(keo_theo(q, p))
        cot_phan.append(keo_theo(not p, not q))

print(cot_dao)
print(cot_phan)
```

:::opt{correct}
`[True, True, False, True]` rồi `[True, True, False, True]` — hai dòng giống hệt nhau
:::

:::opt
`[True, True, False, True]` rồi `[False, False, True, False]`
::why
Gần đúng ở chỗ bạn đọc trúng cả cột đầu, và ở chỗ bạn dùng một lối nghĩ rất tự
nhiên: thêm chữ "không" thì giá trị lật ngược. Với **một** câu thì lối nghĩ ấy
đúng hẳn — đó chính là bài 3.

Chỗ lệch: ở đây chữ "không" không được đặt trước cả câu ghép, nó được đặt trước
**từng vế**, rồi hai vế mới nối lại bằng "nếu... thì". Câu bị lật cả cột phải là
câu "**không phải** (nếu Nam là thành viên thì Nam đeo thẻ)" — một câu khác hẳn,
và không phải câu bài này dựng. Đem phủ định vào trong rồi thì cột ra thế nào
phải hỏi lại luật của "nếu... thì" ở từng dòng, không suy ra từ cột cũ được.
::
:::

:::opt
`[True, False, True, True]` rồi `[True, True, False, True]`
::why
Gần đúng ở chỗ cột thứ hai bạn đọc trúng từng ô, và ở chỗ bạn nhớ đúng cột của
**câu gốc**: `[True, False, True, True]` chính là nó.

Chỗ lệch nằm ở thứ tự hai thứ đưa vào `keo_theo`. Dòng `keo_theo(q, p)` đặt câu
về **thẻ** vào chỗ vế trước và câu về **thành viên** vào chỗ vế sau — đó là câu
đảo, không phải câu gốc. Ở dòng thứ hai của bảng (Nam là thành viên, không đeo
thẻ) câu đảo có vế trước sai, nên nó đúng, trong khi câu gốc thì sai. Đúng ô ấy
là chỗ hai cột tách nhau.
::
:::

:::opt
Máy báo lỗi ở dòng `cot_phan.append(keo_theo(not p, not q))`, vì `not` không đặt được vào trong ngoặc của một lời gọi hàm
::why
Gần đúng ở chỗ bạn để ý tới thứ tự việc máy làm — đó là thói quen tốt và nó sẽ
cứu bạn nhiều lần.

Chỗ lệch: `not p` là một câu Đ/S hoàn chỉnh, y như `p`. Máy tính xong `not p`
thành `True` hay `False` **trước**, rồi mới đưa giá trị ấy vào chỗ `truoc` của
hàm. Cái hàm không bao giờ biết nó được đưa cho `p` hay `not p` — nó chỉ nhận
được một giá trị Đ/S, và làm việc bình thường.
::
:::
::::

::::code{#dung-ba-cot}
Bắt máy dựng ba cột cạnh nhau, rồi để chính ba cột ấy trả lời câu hỏi đầu bài:
câu phản giống ai.

Ba câu cần điền, đúng theo chú thích ghi bên phải mỗi cột:

- **cot_goc** — "Nếu Nam là thành viên thì Nam đeo thẻ."
- **cot_dao** — "Nếu Nam đeo thẻ thì Nam là thành viên."
- **cot_phan** — "Nếu Nam không phải thành viên thì Nam không đeo thẻ."

Viết cả ba bằng hàm `keo_theo` đã có sẵn ở trên, và bằng chữ `not` cho mỗi vế bị
phủ định — đó là hình dạng mà bài này dạy, và cũng là hình dạng mà máy chấm đọc.

**Vế trước** của cả ba câu đã viết sẵn cho bạn; bạn điền vế sau.

Chừa nửa vời như thế là có lý do, và lý do ấy chính là điều bài này sắp chứng
minh. Cột đảo và cột phản **trùng khít nhau ở cả bốn dòng** — đó là kết quả
của bài. Nên nếu chừa cả câu, thì viết câu phản vào chỗ cột đảo và câu đảo vào
chỗ cột phản cũng ra hai cột y hệt, và không cách nào máy biết bạn có nhớ
ngược tên hai khái niệm hay không. Không cách nào — không phải "cổng chưa đủ
tinh", mà là **không thể**, vì hai câu ấy bằng nhau ở mọi dòng.

Chừa sẵn vế trước thì hình dạng bị ép, và lúc ấy máy mới nói được điều gì đó
thật: điền nhầm vế sau ở bất kỳ chỗ nào trong ba chỗ, cột kết quả lệch ngay.
Bài chấm bằng cả ba cột, và cột gốc lệch khỏi hai cột kia ở hai dòng.

```python title=starter
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

cot_goc = []   # "nếu Nam là thành viên thì Nam đeo thẻ"
cot_dao = []   # "nếu Nam đeo thẻ thì Nam là thành viên"
cot_phan = []  # "nếu Nam không phải thành viên thì Nam không đeo thẻ"

for p in [True, False]:
    for q in [True, False]:
        cot_goc.append(keo_theo(p, ___))
        cot_dao.append(keo_theo(q, ___))
        cot_phan.append(keo_theo(not p, ___))

print(cot_goc)
print(cot_dao)
print(cot_phan)
```

```python title=solution
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

cot_goc = []   # "nếu Nam là thành viên thì Nam đeo thẻ"
cot_dao = []   # "nếu Nam đeo thẻ thì Nam là thành viên"
cot_phan = []  # "nếu Nam không phải thành viên thì Nam không đeo thẻ"

for p in [True, False]:
    for q in [True, False]:
        cot_goc.append(keo_theo(p, q))
        cot_dao.append(keo_theo(q, p))
        cot_phan.append(keo_theo(not p, not q))

print(cot_goc)
print(cot_dao)
print(cot_phan)
```

```python title=test
# Ba câu so lệch đứng TRƯỚC. Chúng canh đúng cái bẫy của bài — chép một câu vào
# nhiều chỗ trống — và nếu xếp chúng sau các câu `==` thì một câu `==` sẽ trượt
# trước, người học đọc được thông điệp của nó và bẫy không bao giờ sập.
assert cot_goc != cot_dao, "ở dòng Nam là thành viên mà không đeo thẻ, câu gốc sai còn câu đảo đúng — nên hai cột này không thể trùng nhau"
assert cot_goc != cot_phan, "ở dòng Nam không phải thành viên mà vẫn đeo thẻ, câu gốc đúng còn câu phản sai — nên hai cột này không thể trùng nhau"
assert cot_dao == cot_phan, "đây là chỗ bất ngờ của bài: trên cả bốn dòng, câu đảo và câu phản cho cùng một giá trị — lệch nhau nghĩa là một trong hai chỗ trống chưa viết đúng câu của nó"
assert len(cot_goc) == 4, "hai vế tự do với nhau, mỗi vế hai giá trị: bảng có 4 dòng, nên mỗi cột phải đủ 4 ô"
assert cot_goc == [True, False, True, True], "câu gốc chỉ sai ở dòng thứ hai — Nam là thành viên (vế trước đúng) mà không đeo thẻ (vế sau sai); ba dòng còn lại nó đúng"
assert cot_dao == [True, True, False, True], "câu đảo chỉ sai ở dòng thứ ba — Nam đeo thẻ (vế trước của nó đúng) mà không phải thành viên (vế sau của nó sai)"
assert cot_phan == [True, True, False, True], "câu phản cũng chỉ sai ở dòng thứ ba: ở đó vế trước của nó (Nam không phải thành viên) đúng, mà vế sau của nó (Nam không đeo thẻ) sai"
```

:::hints
- kind: attention
  body: Mỗi chỗ trống là **vế sau** của một câu "nếu... thì" — vế trước thì dòng code đã đưa vào `keo_theo` sẵn rồi. Đọc lại chú thích bên phải mỗi cột: nó ghi rõ vế nào đứng trước, vế nào đứng sau. Trong tay mỗi lượt có `p` (câu "Nam là thành viên") và `q` (câu "Nam đeo thẻ").
- kind: strategy
  body: "Nhìn vế trước đã viết sẵn ở mỗi dòng, rồi hỏi: câu ấy nói vế sau là gì? Cột gốc mở bằng `p` nên vế sau là `q`. Cột đảo mở bằng `q` — nó đổi chỗ hai vế — nên vế sau là `p`. Cột phản mở bằng `not p`: nó giữ nguyên thứ tự của câu gốc mà phủ định cả hai vế, nên vế sau là `q` đã phủ định. Chữ \"không\" của bài 3 viết là `not` đặt trước tên."
- kind: one-line
  body: "Ba chỗ lần lượt là `q`, `p`, và `not q`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: mỗi chỗ trống phải là một câu "nếu... thì" dựng bằng `keo_theo`, viết theo `p` và `q` — gõ cứng một giá trị Đ/S thì ba cột giống hệt nhau và chuyện "cột nào trùng cột nào" không được chứng minh ở đâu cả
  requireAst:
  # Khung khởi đầu KHÔNG gọi `keo_theo` lần nào. Ba cột là ba câu "nếu... thì",
  # nên phải có đúng ba lời gọi. Luật này một mình chặn mọi đáp án gõ cứng
  # `True`/`False` vào chỗ trống.
  - kind: uses-call, target: keo_theo, min: 3
  # Khung khởi đầu có sẵn MỘT `not` (trong thân `keo_theo`). Câu phản cần thêm
  # hai `not` nữa, mỗi vế một cái. Thiếu chúng nghĩa là câu phản chưa được
  # viết ra, mà câu phản chính là khái niệm của bài.
  - kind: uses-operator, target: not, min: 3
  # Cả ba câu đều nói về việc Nam có phải thành viên không: mỗi câu một lần.
  # Dòng `for p in ...` là gán, không phải đọc, nên con số 3 đo đúng phần
  # người học viết ra.
  - kind: uses-name, target: p, min: 3
  # Và cả ba đều nói về việc Nam có đeo thẻ không.
  - kind: uses-name, target: q, min: 3
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[True, False, True, True\]\n\[True, True, False, True\]\n\[True, True, False, True\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cột trùng khít, một cột đi riêng. Mà cột đi riêng lại là cột của câu gốc.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hai bài vừa rồi, bạn thử hai cách sửa một dòng nội quy, và cả hai đều cho ra câu
khác câu gốc:

- **Đổi chỗ** hai vế — ra câu đảo, lệch khỏi câu gốc ở dòng 2 và dòng 3.
- **Phủ định** cả hai vế — ra câu phản, lệch khỏi câu gốc ở đúng hai dòng ấy.

Đổi chỗ thì hỏng. Phủ định thì hỏng. Còn một cách nữa chưa ai thử: làm **cả
hai** cùng lúc — vừa đổi chỗ hai vế, vừa phủ định chúng.

> **"Nếu Nam không đeo thẻ thì Nam không phải thành viên."**

Hai cái hỏng có bù nhau không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
