---
id: toan.logic-va-chung-minh.neu-thi-la-mot-luat
title: "Nếu... thì" là một lời hứa
summary: Một dòng nội quy viết bằng "nếu... thì" chỉ bị phá ở đúng một kiểu buổi sáng — vế trước đúng mà vế sau sai; và đó là chỗ khác hẳn chữ "và".
locale: vi
track: toan
module: logic-va-chung-minh
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.implication]
requires: [logic.de-morgan, logic.equivalence, logic.tautology, logic.contradiction, logic.truth-table, logic.disjunction, logic.conjunction, logic.negation, logic.truth-value, logic.proposition, logic.and, logic.or, logic.not, core.boolean, core.variable, core.list, core.list-append, core.tuple, core.for-unpack, core.print-variable, core.function-def, core.function-call, core.function-parameter, core.function-return, ctrl.for-each, ctrl.nested-loop, ctrl.if]
concepts: [logic.keo-theo, logic.loi-hua, logic.sai-o-dung-mot-dong]
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
Dòng nội quy ở cửa phòng CLB là một lời hứa. Mình đi tìm xem lời hứa ấy bị bội lúc nào.
::::

::::explain{#dong-noi-quy}
Bài trước để lại một dòng chữ dán ở cửa phòng CLB cờ vua lớp 6A:

> **"Nếu là thành viên thì phải đeo thẻ."**

Ba chữ nối bạn đã có — "không", "và", "hoặc" — không chữ nào đọc ra dòng này.
"Là thành viên **và** đeo thẻ" đòi cả hai chuyện cùng xảy ra, mà nội quy thì
không đòi ai phải là thành viên cả. "Là thành viên **hoặc** đeo thẻ" lại gật đầu
với người ngoài CLB đang cầm một tấm thẻ khách, chuyện chẳng liên quan gì tới
nội quy.

Trước khi mổ xẻ, phải nói rõ một chuyện về chính dòng chữ ấy.

Bài 1 đòi ở một mệnh đề đúng một chuyện: phải phân xử được. Muốn phân xử thì
phải biết đem câu ấy đối chiếu với sự việc nào. Dòng trên tấm bảng không nêu tên
ai, nên chưa biết phải đối chiếu với ai — nó chưa phải một câu phân xử được, nó
là **cách viết gọn sáu câu**, mỗi thành viên một câu:

> Nếu **Nam** là thành viên thì **Nam** đeo thẻ. · Nếu **Lan** là thành viên thì
> **Lan** đeo thẻ. · … và tương tự cho Minh, Hoa, Tú, Khanh.

Chỗ trống trên tấm bảng ấy — cái chỗ mà ta thay tên vào — là một món nợ. Ta để
nó tới bài 16, và tới đó nó có tên riêng. Suốt bài này ta cầm đúng **một** câu
trong sáu ấy, câu về Nam:

- **P** — "Nam là thành viên CLB."
- **Q** — "Nam đeo thẻ."

Câu ta xét hôm nay là: **nếu P thì Q**.
::::

::::explain{#khi-nao-loi-hua-bi-boi}
Đọc dòng ấy như một **lời hứa** mà CLB đưa ra về Nam. Câu hỏi của bài 9 hỏi rất
đúng chỗ: **khi nào** thì lời hứa ấy bị coi là sai?

Đứng ở cửa phòng một buổi sáng, hai chuyện có thể xảy ra hoặc không, độc lập với
nhau: Nam có thể là thành viên hoặc không, và Nam có thể đang đeo thẻ hoặc
không. Hai vế, mỗi vế hai giá trị, nên bảng có bốn dòng — bài 6 bảo đảm bốn dòng
ấy không sót không lặp.

Bài trước gợi ý đọc mỗi dòng là **một người**. Ở đây ta đọc chặt hơn một chút:
mỗi dòng là **một buổi sáng có thể xảy ra của Nam**. Hai cách đọc cho cùng một
cái bảng, nhưng cách sau giữ đúng luật bài 1 — câu ta đang chấm nói về một người
có tên, nên mỗi dòng cũng phải nói về đúng người ấy.

| dòng | P: Nam là thành viên | Q: Nam đeo thẻ | buổi sáng ấy trông như thế nào |
|---|---|---|---|
| 1 | Đ | Đ | Nam là thành viên, và đang đeo thẻ |
| 2 | Đ | S | Nam là thành viên, mà không đeo thẻ |
| 3 | S | Đ | Nam không phải thành viên, mà đang đeo thẻ |
| 4 | S | S | Nam không phải thành viên, và không đeo thẻ |

Giờ hỏi từng dòng một câu duy nhất: **buổi sáng này có bắt quả tang lời hứa bị
bội không?**

- **Dòng 2**: có. Nam là thành viên — nghĩa vụ đeo thẻ đã phát sinh — mà Nam
  không đeo. Ai cầm tấm ảnh buổi sáng ấy cũng chỉ được vào tấm bảng mà nói: dòng
  chữ này sai.
- **Ba dòng còn lại**: không. Dòng 1 là lời hứa được giữ đúng. Dòng 3 và dòng 4
  thì Nam không phải thành viên, nên chẳng có nghĩa vụ nào để mà không làm.

Vậy có **đúng một** kiểu buổi sáng phá được câu. Đó là toàn bộ nội dung của chữ
"nếu... thì":

> Câu **kéo theo** "nếu P thì Q" **sai** ở đúng một dòng — dòng vế trước đúng mà
> vế sau sai. Ở mọi dòng khác nó **đúng**.

Nửa sau của luật ấy không phải một quy ước ai đó nghĩ ra. Nó đến từ bài 2: mỗi
mệnh đề mang **đúng một** trong hai giá trị, không có cửa thứ ba. Ta vừa chỉ ra
trọn vẹn chỗ nào làm câu sai; ba dòng còn lại không sai, mà không sai thì chỉ
còn một chỗ để đứng — chúng đúng.
::::

::::example{#khong-phai-chu-va}
Đặt cột mới cạnh hai cột cũ thì thấy nó là một cột chưa từng có.

| dòng | P | Q | P **và** Q | P **hoặc** Q | **nếu P thì Q** |
|---|---|---|---|---|---|
| 1 | Đ | Đ | Đ | Đ | Đ |
| 2 | Đ | S | S | Đ | **S** |
| 3 | S | Đ | S | Đ | Đ |
| 4 | S | S | S | S | Đ |

Cột "và" đúng ở một dòng, cột "hoặc" đúng ở ba dòng, cột mới cũng đúng ở ba dòng
— nhưng **không phải ba dòng ấy**. "Hoặc" gật ở dòng 2 và lắc ở dòng 4; câu kéo
theo làm ngược lại. Bài 8 đã cho tiêu chuẩn: hai câu chỉ thay được cho nhau khi
cột của chúng trùng khít ở **mọi** dòng. Hai cột này lệch nhau ở hai dòng, nên
"nếu... thì" là một chữ nối thật, không phải cách nói khác của chữ nào đã có.

Một chuyện đáng nhớ nữa: dòng 2 là dòng **duy nhất** làm câu sai, nên muốn bác
bỏ một dòng nội quy thì phải trưng ra đúng kiểu buổi sáng ấy — một thành viên
không đeo thẻ. Trưng ra người ngoài CLB không đeo thẻ thì không bác bỏ được gì.
::::

::::predict{#doan-cot-keo-theo commitOnce}
Byte viết luật vừa chốt thành một cái máy nhỏ: `keo_theo` nhận giá trị Đ/S của
hai vế và trả về giá trị của cả câu. Rồi cho máy chạy qua bốn dòng của bảng.

`bon_dong` là bốn dòng ấy viết ra thành danh sách: mỗi dòng một cặp (Nam là
thành viên?, Nam đeo thẻ?), xếp đúng thứ tự dòng 1 tới dòng 4.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

bon_dong = [(True, True), (True, False), (False, True), (False, False)]

cot = []
so_dong_sai = 0
for p, q in bon_dong:
    gia_tri = keo_theo(p, q)
    cot.append(gia_tri)
    if not gia_tri:
        so_dong_sai = so_dong_sai + 1

print(cot)
print(so_dong_sai)
```

:::opt{correct}
`[True, False, True, True]` rồi `1`
:::

:::opt
`[True, False, False, True]` rồi `2`
::why
Gần đúng ở chỗ bạn đọc câu nội quy như một lời cam kết chặt chẽ: thành viên thì
đeo thẻ, mà đã đeo thẻ thì hẳn phải là thành viên. Cách đọc ấy rất tự nhiên
trong tiếng Việt, và nó có tên riêng — bài 15 sẽ dựng hẳn một chữ nối cho nó.

Chỗ lệch nằm ở dòng 3. Nam không phải thành viên mà đang đeo thẻ khách: nội quy
CLB không hề cấm chuyện đó, vì nó chỉ đặt nghĩa vụ lên **thành viên**. Không có
nghĩa vụ nào bị bỏ thì không có gì bị phá, nên dòng 3 vẫn ghi Đ, và cả cột chỉ
có một ô S.
::
:::

:::opt
`[True, False, False, False]` rồi `3`
::why
Gần đúng ở chỗ bạn khoanh trúng dòng 1 và dòng 2: lời hứa được giữ ở dòng 1 và
bị bội ở dòng 2, đúng như bài vừa nói.

Chỗ lệch: cột bạn viết ra chính là cột của chữ **và** — đúng ở một dòng duy
nhất, dòng cả hai vế cùng đúng. Câu "Nam là thành viên **và** Nam đeo thẻ" đòi
Nam phải là thành viên; còn dòng nội quy thì không đòi ai vào CLB cả, nó chỉ nói
điều gì xảy ra **nếu** đã vào. Hai câu ấy lệch nhau ở hai dòng cuối bảng.
::
:::

:::opt
`[True, True, True, True]` rồi `0`
::why
Gần đúng ở chỗ bạn đọc kỹ thân hàm và thấy dòng `return True` nằm cuối, không
nằm trong nhánh nào — nên có vẻ lượt nào cũng chạy tới nó.

Chỗ lệch là cách `return` làm việc, thứ bài R0.39 đã chốt: `return` **trả kết
quả về rồi thoát khỏi hàm ngay lập tức**, không chạy tiếp dòng nào phía dưới.
Ở dòng 2 của bảng thì `truoc` là `True` và `not sau` cũng là `True`, nên nhánh
`if` chạy, `return False` bắn ra, và dòng `return True` bên dưới không tới lượt.
::
:::
::::

::::code{#dung-hai-cot}
Giờ đến lượt bạn dựng đủ hai cột để đặt cạnh nhau: cột của dòng nội quy, và cột
của câu "Nam là thành viên **và** Nam đeo thẻ".

Hai chỗ trống:

1. Trong thân `keo_theo`, sau chữ `if` — điều kiện mô tả **đúng cái dòng duy
   nhất phá được câu**. Trong tay có `truoc` (giá trị của vế trước) và `sau`
   (giá trị của vế sau).
2. Sau `cot_va.append(` — giá trị của câu ghép bằng chữ "và", viết theo `p` và
   `q` của lượt chạy ấy.

Hai chỗ được chấm bằng hai đường khác nhau, nên không chỗ nào trốn được: cột nội
quy bị đo ô một ô, cột "và" bị đo cả cột, và dòng in cuối cùng bắt hai cột phải
khác nhau. Chép cùng một câu vào cả hai chỗ thì dòng cuối vỡ ngay.

```python title=starter
def keo_theo(truoc, sau):
    if ___:
        return False
    return True

cot_noi_quy = []  # "nếu Nam là thành viên thì Nam đeo thẻ"
cot_va = []       # "Nam là thành viên VÀ Nam đeo thẻ"

for p in [True, False]:
    for q in [True, False]:
        cot_noi_quy.append(keo_theo(p, q))
        cot_va.append(___)

print(cot_noi_quy)
print(cot_va)
print(cot_noi_quy == cot_va)
```

```python title=solution
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

cot_noi_quy = []  # "nếu Nam là thành viên thì Nam đeo thẻ"
cot_va = []       # "Nam là thành viên VÀ Nam đeo thẻ"

for p in [True, False]:
    for q in [True, False]:
        cot_noi_quy.append(keo_theo(p, q))
        cot_va.append(p and q)

print(cot_noi_quy)
print(cot_va)
print(cot_noi_quy == cot_va)
```

```python title=test
# Bốn ô của cột nội quy được hỏi riêng từng ô trước khi hỏi cả cột: hỏi cả cột
# trước thì một ô sai cũng chỉ nói được "cột không khớp", còn hỏi riêng thì
# thông điệp chỉ đúng vào buổi sáng nào đang bị chấm sai.
assert keo_theo(True, True) is True, "dòng 1: Nam là thành viên và đang đeo thẻ — lời hứa được giữ đúng, nên câu nội quy về Nam đúng"
assert keo_theo(True, False) is False, "dòng 2: Nam là thành viên mà không đeo thẻ — đây là buổi sáng duy nhất bắt quả tang được lời hứa bị bội, nên câu phải SAI ở đúng chỗ này"
assert keo_theo(False, True) is True, "dòng 3: Nam không phải thành viên mà đang đeo thẻ — nội quy không đặt nghĩa vụ nào lên người ngoài CLB, nên không có gì bị phá và câu vẫn đúng"
assert keo_theo(False, False) is True, "dòng 4: Nam không phải thành viên và không đeo thẻ — cũng không có nghĩa vụ nào bị bỏ, nên câu vẫn đúng"
assert cot_noi_quy == [True, False, True, True], "cả cột nội quy chỉ có một ô S, và ô ấy nằm ở dòng thứ hai"
assert cot_va == [True, False, False, False], "cột của chữ VÀ chỉ đúng ở dòng đầu — dòng cả hai vế cùng đúng; ba dòng còn lại nó sai"
assert cot_noi_quy != cot_va, "hai cột này phải lệch nhau: chúng khác nhau ở dòng 3 và dòng 4, nơi Nam không phải thành viên"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm trong `if`, nên nó phải là một câu Đ/S nói về **một buổi sáng cụ thể**. Đọc lại bảng bốn dòng và tìm dòng duy nhất có ô S ở cột nội quy: dòng ấy nói gì về `truoc`, và nói gì về `sau`? Chỗ trống thứ hai thì đọc thẳng chú thích bên phải `cot_va` — chữ in hoa trong chú thích chính là chữ nối cần dùng.
- kind: strategy
  body: "Dòng phá được câu đòi HAI chuyện cùng lúc: vế trước phải đúng, và vế sau phải sai. Hai chuyện cùng lúc thì nối bằng `and`, còn \"phải sai\" là chữ `not` của bài 3 đặt trước tên. Chỗ trống thứ hai đơn giản hơn: chỉ là hai vế của lượt chạy nối bằng `and`, không thêm chữ nào."
- kind: one-line
  body: "Chỗ thứ nhất là `truoc and not sau`; chỗ thứ hai là `p and q`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: chỗ trống thứ nhất phải nói ra đủ cả hai vế của dòng phá luật (`truoc` đúng VÀ `sau` sai), chỗ thứ hai phải nối `p` với `q` bằng chữ "và" — gõ cứng một giá trị Đ/S thì cả hai cột đứng yên và không cột nào nói được điều gì về bảng
  requireAst:
  # Khung khởi đầu không có `not` nào. Điều kiện phá luật cần đúng một chữ
  # `not` — đặt trước vế sau. Luật này một mình chặn mọi đáp án gõ cứng
  # `True`/`False` vào chỗ trống thứ nhất.
  - kind: uses-operator, target: not, min: 1
  # Hai câu ghép bằng "và": một trong thân `keo_theo`, một ở cột `cot_va`.
  # Khung khởi đầu có 0.
  - kind: uses-operator, target: and, min: 2
  # `truoc` và `sau` là tham số của hàm; khung khởi đầu không ĐỌC chúng lần
  # nào (dòng `def` là chỗ đặt tên, không phải chỗ đọc). Điều kiện phá luật
  # phải đọc cả hai.
  - kind: uses-name, target: truoc, min: 1
  - kind: uses-name, target: sau, min: 1
  # Khung khởi đầu đọc `p` hai lần (lời gọi `keo_theo` và không đâu khác);
  # cột "và" đọc thêm lần nữa. Dòng `for p in ...` là gán, không phải đọc.
  - kind: uses-name, target: p, min: 2
  - kind: uses-name, target: q, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[True, False, True, True\]\n\[True, False, False, False\]\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một ô S trong cả cột. Lời hứa ấy chỉ có đúng một cách để bị bội.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bảng có bốn dòng, và bài này mới nói kỹ về hai dòng đầu — hai dòng mà Nam **là**
thành viên. Ở đó mọi thứ nghe xuôi tai: giữ lời thì đúng, bội lời thì sai.

Còn hai dòng dưới, hai dòng "vế trước sai", thì ta mới ghi Đ vào mà chưa ai bàn
tới. Thử đọc thành lời một dòng trong đó xem có xuôi tai không.

Bình học cùng lớp 6A nhưng không vào CLB cờ vua, và sáng nay Bình không đeo thẻ
nào cả. Câu nội quy về Bình — "nếu Bình là thành viên thì Bình đeo thẻ" — rơi
đúng vào dòng 4.

Vậy nội quy có bị phá không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
