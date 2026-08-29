---
id: toan.dai-so-va-ham-so.nhung-so-lam-cau-do-dung
title: Những số làm câu đó đúng
summary: Đáp án của một phương trình không phải một con số mà là cái giỏ gom hết mọi số làm nó đúng — và quét từng số chỉ nói được về đúng khoảng bạn đã quét.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.solution-set]
requires: [math.equation, math.substitution, math.value-table, math.letter-names-a-slot, math.multiplication, core.boolean, ctrl.comparison, ctrl.if, ctrl.block-indent, ctrl.for-range, ctrl.loop-variable, core.list, core.list-append, core.variable, core.assignment, core.print-variable, core.arithmetic, core.output]
concepts: [math.o-trong, math.phuong-trinh, math.nghiem, math.xe-banh-mi]
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
  reviewed: true
---

::::byte{trigger=enter mood=thinking pose=lean-in}
Mình thử bốn số rồi bảo "đáp án là 20". Bốn số thôi mà dám nói thế à?
::::

::::explain{#dich-cau-hoi-hoi-cho}
Trước hết, trả nốt câu dịch còn nợ. Hôm hội chợ Byte đếm được **1 200 000
đồng**, vẫn 15 000 đồng một ổ. Dịch y hệt cách trưa nay:

- Thứ chưa biết là số ổ đã bán → đặt tên `n`.
- Bán `n` ổ thì thu `15000 × n` đồng.
- Số đếm được là `1200000`.

```text
15000 × n = 1200000
```

Hai dòng, một khuôn. Chỉ đổi con số bên phải.

```text
15000 × n = 300000       ← trưa thường
15000 × n = 1200000      ← hôm hội chợ
```

Bây giờ tới câu khó của bài trước: **có bao nhiêu số làm một dòng như thế
đúng?**

Câu hỏi ấy nghe kỳ, vì thói quen bảo rằng một bài toán có **một** đáp án. Nhưng
thói quen ấy đến từ đâu? Từ những bài kiểu "tính 7 × 8" — ở đó đúng là chỉ có
một kết quả. Còn ở đây bạn không đang tính; bạn đang **lọc**. Bạn cầm cả một dãy
số và hỏi từng số một: "cậu có làm dòng này đúng không?"

Việc lọc thì không hứa trước là lọc ra mấy cái.
::::

::::explain{#nghiem-va-cai-gio}
Đặt tên cho hai thứ, vì từ đây chúng còn dùng dài.

> Một số điền vào làm phương trình **hoá đúng** thì gọi là một **nghiệm** của
> phương trình ấy.
>
> Gom **tất cả** nghiệm lại thì được **tập nghiệm** — cái giỏ đựng mọi số làm
> câu đó đúng. Tập nghiệm mới là đáp án; một nghiệm lẻ chỉ là một món trong giỏ.

Chỗ khác nhau giữa "một nghiệm" và "tập nghiệm" nghe như chẻ sợi tóc, nhưng nó
là chỗ Byte vừa hớ:

- "20 **là một** nghiệm của `15000 × n = 300000`" — câu này Byte **đã chứng
  minh**: điền 20 vào, máy phán ĐÚNG. Kiểm một số thì kết luận được về một số.
- "Đáp án **là** 20" — câu này Byte **chưa** chứng minh. Nói thế là nói thêm một
  vế nữa: *và không còn số nào khác*. Byte mới thử bốn số. Còn 10, còn 19, còn
  21 đã thử — thế còn 7? còn 100? còn 3 000 000?

Câu thứ hai nói về **cả cái giỏ**. Muốn nói về cả giỏ thì phải biết gì đó về mọi
số, chứ không phải về bốn số.

Đây không phải chuyện bắt bẻ chữ nghĩa. Nó là chỗ khác nhau giữa "tôi tìm được
một cái" và "tôi tìm hết rồi".
::::

::::example{#quet-mot-day-so}
Cách thật thà nhất để đi tìm cả giỏ: **quét**. Lấy từng số một, điền vào, xem
máy phán gì, phán ĐÚNG thì bỏ vào giỏ.

Byte quét từ 0 tới 40 ổ — một buổi trưa thì khó bán hơn thế.

| điền `n` | `15000 × n` | dòng `= 300000` | vào giỏ? |
|---|---|---|---|
| 0 | 0 | SAI | không |
| 1 | 15 000 | SAI | không |
| … | … | … | … |
| 19 | 285 000 | SAI | không |
| 20 | 300 000 | ĐÚNG | **có** |
| 21 | 315 000 | SAI | không |
| … | … | … | … |
| 40 | 600 000 | SAI | không |

Cuối buổi quét, trong giỏ có đúng một món: `20`. Viết cái giỏ ấy ra:

```text
tập nghiệm của 15000 × n = 300000, quét trong 0…40:   {20}
```

Byte biết **41 số** rồi. Con số 20 không còn là linh cảm nữa.

Nhưng để ý cho kỹ cái đuôi của câu vừa viết: *quét trong 0…40*. Cái đuôi ấy
không phải chú thích cho đẹp — nó là **giới hạn của điều Byte biết**. Ngoài
khoảng ấy, Byte vẫn chưa biết gì cả.

Và ngay hôm sau, cái đuôi ấy cắn.
::::

::::byte{trigger=enter mood=curious pose=point-editor}
Hôm hội chợ mình cũng quét từ 0 tới 40. Cái giỏ về rỗng. Rỗng nghĩa là sao?
::::

::::predict{#doan-cai-gio commitOnce}
Byte nhờ máy quét hộ. Máy chạy 41 lượt, mỗi lượt điền một số vào `n`, và lượt
nào phán ĐÚNG thì bỏ số ấy vào giỏ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
gia_mot_o = 15000
gio = []
for n in range(41):
    if gia_mot_o * n == 300000:
        gio.append(n)
print(gio)
```

:::opt{correct}
`[20]`
:::

:::opt
`20`
::why
Gần đúng — bạn tìm ra đúng con số mà cả bài này đi tìm, và nếu câu hỏi là "bán
mấy ổ" thì `20` là câu trả lời chuẩn.

Ranh giới nằm ở **thứ đang được in**. `gio` không phải một con số, nó là một cái
giỏ (bạn đã gặp ở Realm 0: một chỗ chứa nhiều giá trị). In một cái giỏ thì máy
in cả giỏ, kèm cặp ngoặc vuông, **dù trong giỏ chỉ có một món**. Và ở bài này
cặp ngoặc ấy đáng giá: nó nhắc rằng thứ bạn cầm là cái giỏ chứ không phải con
số — cái giỏ *lần này* có một món, chứ không phải cái giỏ *luôn* có một món.
::
:::

:::opt
`[300000]`
::why
Gần đúng ở chỗ bạn nhìn ra máy đang gom một thứ gì đó vào giỏ, và 300 000 đúng
là con số mà cả đoạn này xoay quanh — nó là cái mốc để so.

Chỗ lệch nằm ở **món được bỏ vào giỏ**: dòng `gio.append(n)` bỏ vào `n`, tức số
**ổ**, chứ không phải số **tiền**. 300 000 đứng bên phải dấu `==`, ở vai người
gác cửa: nó quyết định lượt nào được vào giỏ, rồi đứng nguyên đó. Nghiệm của
`15000 × n = 300000` là một số điền vào chỗ trống — mà chỗ trống ở đây là số ổ.
::
:::

:::opt
Cả 41 số, từ `0` tới `40`
::why
Gần đúng ở chỗ bạn đếm số lượt không sai một lượt nào: `range(41)` đúng là chạy
41 lượt, từ 0 tới 40, và `gio.append(n)` đúng là nằm bên trong vòng lặp ấy.

Ranh giới là **dòng `if` chen vào giữa**. `append` thụt vào sâu hơn `if`, nên nó
thuộc về `if`, không thuộc thẳng về `for`. Vòng lặp vẫn chạy đủ 41 lượt — nhưng
mỗi lượt, câu hỏi `gia_mot_o * n == 300000` được hỏi lại, và chỉ lượt nào trả
lời `True` thì `append` mới được chạy. 40 lượt kia đi qua giỏ mà không bỏ gì
vào. Đó đúng là việc "lọc": mọi số đều được hỏi, ít số được nhận.
::
:::
::::

::::explain{#giot-rong-khong-phai-khong-co}
Bây giờ tới chuyện hôm hội chợ. Cùng cái máy quét ấy, chỉ đổi con số bên phải
thành `1200000`, quét vẫn từ 0 tới 40:

```text
tập nghiệm của 15000 × n = 1200000, quét trong 0…40:   { }
```

Giỏ về rỗng. Byte đọc vội thành: *"không có số nào cả."*

Đọc thế là hớ lần thứ hai, và hớ đúng kiểu cũ — nói về **cả cái giỏ** trong khi
mới kiểm được **một khúc**. Quét lại, lần này tới 100:

```text
tập nghiệm của 15000 × n = 1200000, quét trong 0…100:  {80}
```

`15000 × 80 = 1200000`. Nghiệm vẫn nằm đó, nguyên vẹn, suốt từ đầu — chỉ là nó
ở ngoài tầm tay Byte quơ.

Rút ra một điều, và nó là điều đáng mang đi nhất của bài:

> Quét là một cách **tìm** nghiệm, không phải một cách **biết hết** nghiệm. Kết
> quả một lần quét chỉ nói về đúng những số đã được hỏi. Giỏ rỗng nghĩa là
> "chưa thấy trong khoảng này", không phải "không có".

Chỗ này đáng dừng lâu một chút, vì nó là hai câu khác hẳn nhau và người ta nhầm
suốt:

| câu | quét 0…40 nói được không? |
|---|---|
| "80 là một nghiệm" | không — 80 chưa được hỏi |
| "trong 0…40 không có nghiệm **nguyên** nào" | được — 41 số nguyên ấy đều đã bị hỏi |
| "phương trình này không có nghiệm" | không — còn vô số số chưa hỏi |

Cái quét làm được là chuyện của một khoảng, và trong khoảng ấy cũng chỉ là
chuyện của những số nó thật sự bước qua. Cái bạn muốn nói lại là chuyện của mọi
số.
::::

::::code{#quet-hai-cai-gio}
Đến lượt bạn dựng cái máy quét, và dựng nó **ba lần** để chính bạn nhìn thấy
chuyện vừa nói.

Ba lần quét, dùng chung một giá bánh:

1. Trưa thường — `15000 × n = 300000`, quét `0…40`.
2. Hội chợ — `15000 × n = 1200000`, quét `0…40` (đúng khoảng Byte đã quét).
3. Hội chợ lần nữa — cùng dòng ấy, nhưng quét rộng ra `0…100`.

Điền ba chỗ trống. Mỗi chỗ là **câu hỏi gác cửa** của một lần quét: điền số đang
xét vào phương trình rồi hỏi hai vế có đúng bằng nhau không. Dựng nó từ những
cái tên có sẵn — `gia_mot_o`, `n`, và số tiền của lần quét ấy.

Hai lần quét sau dùng **cùng một phương trình** và chỉ khác nhau ở bề rộng
khoảng quét. Nếu bạn gõ cứng một con số vào chỗ trống thì hai lần ấy sẽ ra giống
hệt nhau, mà bài này đứng hay đổ chính ở chỗ chúng **phải khác nhau**.

```python title=starter
gia_mot_o = 15000
tien_trua = 300000
tien_hoi_cho = 1200000

# Lần 1 — trưa thường, quét 0…40.
gio_trua = []
for n in range(41):
    if ___:
        gio_trua.append(n)

# Lần 2 — hội chợ, vẫn quét 0…40.
gio_hoi_cho_hep = []
for n in range(41):
    if ___:
        gio_hoi_cho_hep.append(n)

# Lần 3 — vẫn hội chợ, nhưng quét rộng ra 0…100.
gio_hoi_cho_rong = []
for n in range(101):
    if ___:
        gio_hoi_cho_rong.append(n)

print(gio_trua)
print(gio_hoi_cho_hep)
print(gio_hoi_cho_rong)
```

```python title=solution
gia_mot_o = 15000
tien_trua = 300000
tien_hoi_cho = 1200000

# Lần 1 — trưa thường, quét 0…40.
gio_trua = []
for n in range(41):
    if gia_mot_o * n == tien_trua:
        gio_trua.append(n)

# Lần 2 — hội chợ, vẫn quét 0…40.
gio_hoi_cho_hep = []
for n in range(41):
    if gia_mot_o * n == tien_hoi_cho:
        gio_hoi_cho_hep.append(n)

# Lần 3 — vẫn hội chợ, nhưng quét rộng ra 0…100.
gio_hoi_cho_rong = []
for n in range(101):
    if gia_mot_o * n == tien_hoi_cho:
        gio_hoi_cho_rong.append(n)

print(gio_trua)
print(gio_hoi_cho_hep)
print(gio_hoi_cho_rong)
```

```python title=test
# Hai câu `!=` đứng TRƯỚC. Câu đầu canh cái bẫy chính của bài: hai lần quét sau
# hỏi CÙNG một phương trình, nên nếu chỗ trống bị gõ cứng thì hai giỏ ra như
# nhau và bẫy phải sập ngay tại đây.
assert gio_hoi_cho_hep != gio_hoi_cho_rong, "cùng một phương trình, chỉ khác bề rộng khoảng quét — mà hai giỏ phải khác nhau, vì khoảng 0…40 KHÔNG với tới 80"
assert gio_trua != gio_hoi_cho_rong, "300 000 đồng và 1 200 000 đồng là hai phương trình khác nhau, nên hai giỏ không thể trùng"
assert gio_trua == [20], "trong 0…40, chỉ 20 ổ cho đúng 300 000 đồng — nên giỏ có đúng một món"
assert gio_hoi_cho_hep == [], "quét tới 40 thì chưa chạm 80, nên giỏ về rỗng — rỗng ở đây nghĩa là chưa thấy, không phải không có"
assert gio_hoi_cho_rong == [80], "80 × 15 000 = 1 200 000, và lần này khoảng quét đủ rộng để hỏi tới 80"
```

:::hints
- kind: attention
  body: Ba dòng đầu tiên giữ sẵn ba con số, và tên của chúng nói rõ con số nào thuộc lần quét nào. Nhìn thêm dòng `for` ngay trên mỗi chỗ trống: nó vừa đặt ra cái tên `n` — số đang được xét ở lượt này. Chỗ trống cần cả `n` lẫn hai cái tên kia.
- kind: strategy
  body: Mỗi chỗ trống là chính cái phương trình của lần quét ấy, đã điền `n` vào, đem hỏi máy. Vế trái là tiền thu khi bán `n` ổ — giá một ổ nhân với `n`. Vế phải là số tiền đếm được của lần ấy. Nối hai vế bằng `==`. Lần 2 và lần 3 dùng chung một số tiền, nên hai chỗ trống ấy giống hệt nhau — thứ khác nhau nằm ở dòng `range` phía trên, không nằm trong chỗ trống.
- kind: one-line
  body: "Lần 1 là `gia_mot_o * n == tien_trua`; lần 2 và lần 3 đều là `gia_mot_o * n == tien_hoi_cho`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một câu hỏi so sánh `==` dựng từ `gia_mot_o`, `n` và số tiền của lần quét ấy — gõ cứng một con số thì hai lần quét cuối hoá giống hệt nhau và bài mất chỗ đứng
  requireAst:
  # Ba chỗ trống, ba câu hỏi gác cửa. Khung khởi đầu không có dấu `==` nào.
  - kind: uses-operator, target: ==, min: 3
  # Vế trái phải dựng từ tên, không phải từ số tự nhân sẵn.
  - kind: uses-operator, target: *, min: 3
  - kind: uses-name, target: gia_mot_o, min: 3
  # `n` là số đang xét ở lượt này. Thiếu nó thì câu hỏi gác cửa không đổi theo
  # từng lượt, và cả vòng lặp hoá vô nghĩa.
  - kind: uses-name, target: n, min: 6
  # Vế phải cũng phải là tên: `tien_hoi_cho` dùng ở hai lần quét cuối.
  - kind: uses-name, target: tien_trua, min: 1
  - kind: uses-name, target: tien_hoi_cho, min: 2
  forbidAst:
  # Lưới thứ hai, chặn đúng hai đáp án đã biết trước kết quả. Lời giải thật
  # không chứa nguyên văn con số nào trong hai con số này.
  - kind: has-literal, target: 20
  - kind: has-literal, target: 80
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^\[20\]\n\[\]\n\[80\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một giỏ một món, một giỏ rỗng, một giỏ một món. Cùng câu hỏi, khác tầm tay quơ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Quét thì thật thà, nhưng nó có một chỗ hụt không vá được bằng cách quét rộng
thêm. Byte quét tới 100 thì biết về 101 số. Quét tới 1 000 thì biết về 1 001 số.
Còn con số 1 000 ấy do ai chọn? Do Byte. Và ngoài nó vẫn là chỗ Byte chưa hỏi.

Chỗ hụt ấy không phải chuyện máy chạy chậm. Nó là chuyện **loại**: quét trả lời
được câu "trong khúc này có gì", không trả lời được câu "trong **mọi** số có
gì". Mà đó mới là câu bạn muốn hỏi. Thêm nữa, có những phương trình mà nghiệm
không phải số nguyên — Byte cân bột theo ki-lô-gam chứ không theo ổ, và nghiệm
rơi vào chỗ giữa hai vạch thì quét từng số nguyên đi ngang qua nó mà không thấy.

Vậy nếu không thử từng số thì còn cách nào? Chỉ còn một hướng: **động thẳng vào
chính cái dòng ấy** — đổi nó thành một dòng dễ đọc hơn.

Nhưng đây mới là chỗ đáng sợ. `15000 × n = 300000` đang là một lời khẳng định
**đang đúng** cho đúng những số trong giỏ của nó. Thò tay vào sửa mà làm nó hoá
sai, hoặc làm giỏ phình ra thêm một món lạ, thì bạn đã đổi mất câu hỏi và câu
trả lời tìm được chẳng còn là của ai.

Ngoài đời cũng có những vật đang ở thế "đang đúng" như vậy — chạm vào là hỏng
nếu chạm bừa, mà chạm khéo thì vẫn còn nguyên. Bạn nghĩ ra vật nào? Và khi động
vào vật ấy, người ta phải **giữ gìn cái gì**?

Bài sau mượn đúng một vật như thế.
::::

::::checkpoint{mastery=0.8}
::::
