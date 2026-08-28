---
id: toan.logic-va-chung-minh.doi-cho-hai-ve
title: Đổi chỗ hai vế
summary: Đổi chỗ vế trước với vế sau của một câu "nếu... thì" cho ra một câu khác hẳn — cột của nó lệch cột câu gốc ở hai dòng, nên có buổi sáng câu này đúng mà câu kia sai.
locale: vi
track: toan
module: logic-va-chung-minh
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [logic.converse]
requires: [logic.vacuous-truth, logic.implication, logic.de-morgan, logic.equivalence, logic.tautology, logic.contradiction, logic.truth-table, logic.disjunction, logic.conjunction, logic.negation, logic.truth-value, logic.proposition, logic.and, logic.or, logic.not, core.boolean, core.variable, core.list, core.list-append, core.tuple, core.print-variable, core.function-def, core.function-call, core.function-parameter, core.function-return, ctrl.for-each, ctrl.nested-loop, ctrl.if]
concepts: [logic.menh-de-dao, logic.doi-cho-thi-hong, logic.hai-cot-lech-nhau]
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
Thấy một người đeo thẻ. Mình muốn kết luận người ấy là thành viên — mà nội quy có cho phép đâu.
::::

::::explain{#doc-nguoc-dong-noi-quy}
Bài trước để lại một chỗ ngập ngừng. Nội quy CLB **đúng**, và bạn nhìn thấy một
người **đang đeo thẻ**. Kết luận "người này là thành viên" — chắc chưa?

Kiên đã trả lời hộ rồi: sáng ấy Kiên đeo thẻ khách vào phòng ngồi xem, Kiên
không phải thành viên, mà câu nội quy về Kiên vẫn đúng. Vậy cái kết luận kia
không rút ra được từ nội quy.

Nhìn kỹ thì cái kết luận ấy là một **câu khác**. Vẫn hai vế cũ về Nam:

- **P** — "Nam là thành viên CLB."
- **Q** — "Nam đeo thẻ."

thì hai câu đang bị đem ra so là:

| | câu | dạng |
|---|---|---|
| câu gốc *(bài 10)* | Nếu Nam là thành viên thì Nam đeo thẻ. | nếu **P** thì **Q** |
| câu mới *(hôm nay)* | Nếu Nam đeo thẻ thì Nam là thành viên. | nếu **Q** thì **P** |

Câu thứ hai có tên riêng trong nghề:

> **Mệnh đề đảo** của một câu "nếu... thì" là câu dựng bằng cách **đổi chỗ hai
> vế** cho nhau, không thêm bớt chữ nào.

Chỗ đáng ngờ là ở đây: với chữ "và" thì đổi chỗ hai vế chẳng đổi gì. Bài 4 đã
chứng minh chuyện đó bằng bảng — "Nam ngã và Nam khóc" cùng cột với "Nam khóc và
Nam ngã". Chữ "hoặc" cũng vậy. Nên nếu chỉ nhớ mang máng rằng "ghép hai vế thì
đổi chỗ được", người ta sẽ tưởng câu đảo là câu gốc đọc theo chiều khác.

Cách duy nhất để biết chắc là dựng hai bảng và đặt cạnh nhau — tiêu chuẩn của
bài 8.
::::

::::explain{#dat-hai-cot-canh-nhau}
Bảng vẫn bốn dòng, vẫn hai vế tự do với nhau, và luật chấm vẫn là luật bài 10:
câu "nếu... thì" sai ở đúng một dòng — vế trước đúng mà vế sau sai.

Chấm câu đảo thì phải nhớ vế trước của **nó** là Q, còn vế sau của **nó** là P.

| dòng | P: Nam là thành viên | Q: Nam đeo thẻ | câu gốc | câu đảo |
|---|---|---|---|---|
| 1 | Đ | Đ | Đ | Đ |
| 2 | Đ | S | **S** | Đ |
| 3 | S | Đ | Đ | **S** |
| 4 | S | S | Đ | Đ |

Hai dòng đáng đọc chậm, vì chúng là chỗ hai cột tách nhau:

- **Dòng 2** — Nam là thành viên mà không đeo thẻ. Câu gốc có vế trước đúng, vế
  sau sai, nên câu gốc **sai**. Còn câu đảo: vế trước của nó là "Nam đeo thẻ",
  dòng 2 ghi S. Vế trước sai thì cả câu đúng — chân lý rỗng của bài 11. Câu đảo
  **đúng**.
- **Dòng 3** — Nam không phải thành viên mà đang đeo thẻ. Câu gốc có vế trước
  sai, nên nó đúng theo kiểu rỗng. Câu đảo thì ngược lại: vế trước của nó ("Nam
  đeo thẻ") đúng, vế sau của nó ("Nam là thành viên") sai. Đúng cái dòng duy
  nhất phá được một câu kéo theo. Câu đảo **sai**.

Vậy hai cột lệch nhau ở **hai** dòng: dòng 2 và dòng 3. Bài 8 nói rõ tiêu chuẩn
— chỉ khi hai cột trùng khít ở mọi dòng thì hai câu mới thay được cho nhau. Lệch
một dòng đã đủ hỏng, huống chi hai.

> Câu đảo là một câu **khác** câu gốc. Câu gốc đúng không kéo theo câu đảo đúng,
> và câu đảo đúng cũng không kéo theo câu gốc đúng.

Cần nói rõ một chuyện để khỏi nhớ quá tay: "khác nhau" ở đây **không** có nghĩa
là hai câu luôn trái nhau. Nhìn dòng 1 và dòng 4 mà xem — ở đó cả hai cùng đúng.
Chuyện đúng là: có những buổi sáng chúng khác nhau, nên **không được** dùng câu
này thay câu kia.
::::

::::example{#buoi-sang-cua-kien}
Sáng hôm trước ở cửa phòng CLB, Kiên rơi đúng vào dòng 3: không phải thành viên,
mà đang đeo thẻ khách.

Đứng ngay chỗ ấy, hai câu về Kiên đọc ra hai kết cục trái ngược:

- **"Nếu Kiên là thành viên thì Kiên đeo thẻ"** — vế trước sai, nên câu này
  **đúng**. Nội quy CLB không hề bị Kiên làm sao cả.
- **"Nếu Kiên đeo thẻ thì Kiên là thành viên"** — vế trước đúng (Kiên đang đeo
  thẻ thật), vế sau sai (Kiên không phải thành viên). Câu này **sai**, và Kiên
  chính là người bắt quả tang nó.

Một buổi sáng, một con người, hai câu — một đúng một sai. Chuyện ấy đủ để chốt
rằng hai câu không phải một, mà không cần dựng lại cả bảng.

Đây cũng là câu trả lời cho chỗ ngập ngừng đầu bài: nội quy đúng **không** cho
bạn quyền nhìn tấm thẻ rồi kết luận về tư cách thành viên. Muốn có quyền ấy,
trường phải dán thêm một dòng nội quy nữa — chính là câu đảo — và đó là một lời
hứa khác, phải được bảo đảm riêng.
::::

::::predict{#doan-hai-cau-o-dong-ba commitOnce}
Byte đặt máy vào đúng dòng 3 và hỏi hai câu: giá trị của câu gốc, rồi giá trị
của câu đảo.

Cả hai đều gọi `keo_theo`, chỉ khác thứ tự hai thứ đem vào.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

# Dòng 3 của bảng: Nam KHÔNG phải thành viên, mà Nam ĐANG đeo thẻ khách.
p = False
q = True

print(keo_theo(p, q))
print(keo_theo(q, p))
```

:::opt{correct}
`True` rồi `False`
:::

:::opt
`True` rồi `True`
::why
Gần đúng ở chỗ bạn nhớ một sự thật có thật: bài 4 đã chứng minh rằng đổi chỗ hai
vế của chữ "và" thì cột không đổi, và chữ "hoặc" cũng thế. Đem thói quen ấy sang
đây là chuyện tự nhiên.

Chỗ lệch: `keo_theo` không đối xử với hai chỗ như nhau. Nó chỉ trả về `False` khi
**chỗ thứ nhất** đúng và **chỗ thứ hai** sai — hai chỗ có vai khác nhau hẳn. Với
`p` là `False` và `q` là `True`, lời gọi thứ nhất đưa `False` vào chỗ thứ nhất
nên không bao giờ chạm nhánh `if`; lời gọi thứ hai đưa `True` vào chỗ thứ nhất
và `False` vào chỗ thứ hai, nên nó chạm đúng nhánh ấy.
::
:::

:::opt
`False` rồi `True`
::why
Gần đúng ở chỗ bạn nắm được điều quan trọng nhất của bài: hai lời gọi ấy cho hai
giá trị khác nhau, nên hai câu không phải một.

Chỗ lệch là hai giá trị bị đặt nhầm chỗ. Lời gọi thứ nhất là câu **gốc**: vế
trước "Nam là thành viên" đang sai, mà vế trước sai thì không ai phá được câu —
bài 11 — nên nó `True`. Lời gọi thứ hai là câu **đảo**: vế trước của nó là "Nam
đeo thẻ", đang đúng, còn vế sau của nó đang sai, nên nó `False`.
::
:::

:::opt
`False` rồi `False`
::why
Gần đúng ở chỗ bạn thấy buổi sáng này có chỗ chướng: một người không phải thành
viên mà lại đang đeo thẻ, nghe như có gì đó không ổn.

Chỗ lệch: nội quy CLB chỉ đặt nghĩa vụ lên thành viên, nên nó không cấm người
ngoài đeo thẻ khách. Câu gốc ở dòng này có vế trước sai và vì thế **đúng** theo
kiểu rỗng — đúng điều bài 11 vừa dựng. Chỉ có câu đảo mới sai ở đây, vì chính nó
là câu đòi hỏi rằng ai đeo thẻ cũng phải là thành viên.
::
:::
::::

::::code{#dung-cot-cau-dao}
Giờ đến lượt bạn dựng cả hai cột trên đủ bốn dòng, và bắt máy chỉ ra những dòng
mà hai cột **lệch nhau**.

Hai chỗ trống:

1. Sau `dao =` — câu đảo: vẫn dùng `keo_theo`, vẫn hai thứ ấy, đổi chỗ chúng cho
   nhau.
2. Sau chữ `if` — điều kiện để dòng này được ghi vào `dong_lech`: giá trị của
   câu gốc và giá trị của câu đảo **khác nhau**.

Hai chỗ được chấm bằng hai đường khác nhau: cột câu đảo bị đo cả bốn ô, còn danh
sách dòng lệch bị đo cả nội dung lẫn thứ tự. Viết `keo_theo(p, q)` vào chỗ thứ
nhất — tức chép lại câu gốc — thì cột câu đảo trùng cột câu gốc, danh sách dòng
lệch rỗng, và cả hai đường cùng vỡ.

```python title=starter
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

cot_goc = []    # "nếu Nam là thành viên thì Nam đeo thẻ"
cot_dao = []    # "nếu Nam đeo thẻ thì Nam là thành viên"
dong_lech = []  # dòng nào hai cột cho hai giá trị khác nhau

for p in [True, False]:
    for q in [True, False]:
        goc = keo_theo(p, q)
        dao = ___
        cot_goc.append(goc)
        cot_dao.append(dao)
        if ___:
            dong_lech.append((p, q))

print(cot_goc)
print(cot_dao)
print(dong_lech)
```

```python title=solution
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

cot_goc = []    # "nếu Nam là thành viên thì Nam đeo thẻ"
cot_dao = []    # "nếu Nam đeo thẻ thì Nam là thành viên"
dong_lech = []  # dòng nào hai cột cho hai giá trị khác nhau

for p in [True, False]:
    for q in [True, False]:
        goc = keo_theo(p, q)
        dao = keo_theo(q, p)
        cot_goc.append(goc)
        cot_dao.append(dao)
        if goc != dao:
            dong_lech.append((p, q))

print(cot_goc)
print(cot_dao)
print(dong_lech)
```

```python title=test
# Hai ô lệch được hỏi riêng TRƯỚC khi hỏi cả cột, vì chúng là chỗ bài đang dạy:
# hỏi cả cột trước thì một ô sai cũng chỉ nói được "cột không khớp", còn hỏi
# riêng thì thông điệp chỉ vào đúng buổi sáng đang bị chấm sai.
assert cot_dao[1] is True, "dòng 2 (Nam là thành viên mà không đeo thẻ): vế trước của câu đảo là 'Nam đeo thẻ', dòng này ghi S cho nó — vế trước sai thì cả câu đúng, nên ô này của câu đảo phải là True"
assert cot_dao[2] is False, "dòng 3 (Nam không phải thành viên mà đang đeo thẻ): vế trước của câu đảo đúng, vế sau của nó sai — đây là ô duy nhất làm câu đảo sai"
assert cot_dao == [True, True, False, True], "cả cột câu đảo chỉ có một ô S, và ô ấy nằm ở dòng thứ ba"
assert cot_goc == [True, False, True, True], "cột câu gốc chỉ có một ô S, và ô ấy nằm ở dòng thứ hai — đó là luật bài 10"
assert dong_lech == [(True, False), (False, True)], "hai cột lệch nhau ở đúng hai dòng, ghi theo thứ tự bảng: dòng 2 (thành viên mà không đeo thẻ) rồi dòng 3 (đeo thẻ mà không phải thành viên)"
assert cot_goc != cot_dao, "hai cột này phải khác nhau: nếu chúng trùng khít thì chỗ trống thứ nhất đang chép lại câu gốc chứ chưa đổi chỗ hai vế"
```

:::hints
- kind: attention
  body: Dòng ngay bên trên chỗ trống thứ nhất đã viết sẵn câu gốc là `keo_theo(p, q)`. Câu đảo cũng là một lời gọi `keo_theo`, cũng đúng hai thứ ấy — chỉ khác chỗ nào đứng trước. Chỗ trống thứ hai thì đọc chú thích bên phải `dong_lech`: nó gom những dòng mà hai giá trị **khác nhau**, và trong tay lượt ấy bạn đang giữ hai cái tên `goc` và `dao`.
- kind: strategy
  body: "Vế trước của câu đảo là vế sau của câu gốc, và ngược lại — nên chỉ việc viết lại lời gọi với hai thứ hoán vị. Còn \"hai giá trị khác nhau\" thì Python có sẵn một dấu so sánh nói đúng điều đó, và nó là dấu ngược nghĩa với `==` mà bài R0.24 đã dạy."
- kind: one-line
  body: "Chỗ thứ nhất là `keo_theo(q, p)`; chỗ thứ hai là `goc != dao`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: chỗ trống thứ nhất phải là một lời gọi `keo_theo` với hai vế đổi chỗ, chỗ thứ hai phải so hai giá trị `goc` và `dao` xem chúng có KHÁC nhau không — gõ cứng một giá trị Đ/S vào đó thì cột câu đảo đứng yên và danh sách dòng lệch không đo được gì
  requireAst:
  # Khung khởi đầu gọi `keo_theo` một lần (câu gốc). Câu đảo là lời gọi thứ
  # hai. Luật này một mình chặn mọi đáp án gõ cứng `True`/`False` vào chỗ
  # trống thứ nhất.
  - kind: uses-call, target: keo_theo, min: 2
  # Khung khởi đầu không có dấu `!=` nào. Chỗ trống thứ hai phải hỏi "hai giá
  # trị này có KHÁC nhau không" — hỏi `==` thì gom đúng những dòng TRÙNG, tức
  # là hai dòng còn lại.
  - kind: uses-operator, target: "!=", min: 1
  # Khung khởi đầu đọc `p` hai lần (lời gọi câu gốc, và lần ghi cặp vào danh
  # sách); câu đảo đọc thêm lần nữa. Dòng `for p in ...` là gán, không phải
  # đọc. `q` cũng vậy.
  - kind: uses-name, target: p, min: 3
  - kind: uses-name, target: q, min: 3
  # Khung khởi đầu đọc `goc` một lần và `dao` một lần (hai dòng `append`);
  # phép so sánh ở chỗ trống thứ hai đọc thêm mỗi tên một lần nữa.
  - kind: uses-name, target: goc, min: 2
  - kind: uses-name, target: dao, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[True, False, True, True\]\n\[True, True, False, True\]\n\[\(True, False\), \(False, True\)\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cột, hai ô S nằm ở hai dòng khác nhau. Đổi chỗ hai vế là ra một câu khác thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đổi chỗ hai vế thì hỏng. Nhưng đó không phải cách duy nhất người ta hay sửa một
dòng nội quy.

Thử cách khác: **giữ nguyên thứ tự** hai vế, nhưng phủ định cả hai.

> **"Nếu Nam không phải thành viên thì Nam không phải đeo thẻ."**

Nghe cũng xuôi tai, và nhiều người đọc nó như một cách nói lại nội quy cho gọn.

Câu này có phải câu gốc không? Và nếu không phải, thì bảng của nó giống bảng của
ai?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
