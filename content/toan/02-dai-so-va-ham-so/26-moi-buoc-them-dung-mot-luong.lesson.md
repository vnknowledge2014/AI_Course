---
id: toan.dai-so-va-ham-so.moi-buoc-them-dung-mot-luong
title: Mỗi bước thêm đúng một lượng
summary: Đồ thị thẳng không phải chuyện may — nó thẳng vì đầu vào thêm 1 thì đầu ra luôn đổi đúng cùng một lượng, và lượng ấy có tên là độ dốc.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.slope]
requires: [math.function-notation, math.function, math.graph-of-expression, math.value-table, math.negative-number, math.subtraction-as-distance, core.function-def, core.function-call, core.print-variable]
concepts: [math.toc-do-doi, math.buoc-deu, math.doc-am]
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

::::byte{trigger=enter mood=thinking pose=idle}
Bảy cái chấm thẳng hàng. Mình muốn biết cái gì bắt chúng phải thẳng.
::::

::::explain{#nhin-vao-bang-dung-nhin-vao-hinh}
Bài trước để lại một câu hỏi: máy `t(n) = 15000 × n` chấm lên mặt phẳng thì ra
một hàng chấm thẳng băng — **vì sao?**

Nhìn vào hình sẽ không tìm được câu trả lời, vì hình là *kết quả*. Muốn biết vì
sao, phải quay lại thứ đã đẻ ra hình: **cái bảng** của bài 5.

Bảng ấy có hai cột — điền gì, ra gì. Bài này thêm một cột thứ ba, và cột thứ ba
mới là cột nói ra sự thật. Nó không ghi kết quả; nó ghi **kết quả đổi bao nhiêu
so với dòng ngay trên**:

```text
  n │     t(n) │ bước từ dòng trên
───┼──────────┼───────────────────
  0 │        0 │
  1 │    15000 │            +15000
  2 │    30000 │            +15000
  3 │    45000 │            +15000
  4 │    60000 │            +15000
```

Cột thứ hai đổi liên tục: 0, 15000, 30000, 45000 — không dòng nào giống dòng
nào. Cột thứ ba thì ngược lại: nó **đứng im**. Xuống bao nhiêu dòng nữa nó cũng
là 15000.

Con số đứng im ấy đáng có một cái tên:

> **Độ dốc** của một cái máy là lượng đầu ra đổi khi đầu vào thêm **đúng 1**.

Độ dốc của `t` là 15 000. Nói ra bằng tiếng Việt: *bán thêm một ổ thì thu thêm
15 nghìn* — câu này bạn biết từ lâu rồi, và bây giờ nó có chỗ đứng trên bảng.

**Bây giờ mới tới phần trả lời câu hỏi.** Vì sao đều thì thẳng?

Nghĩ về việc đi từ chấm này sang chấm kế bên trên mặt phẳng. Mỗi lần như thế
bạn làm đúng hai việc: sang phải **1 ô**, rồi lên **15 000**. Lần nào cũng đúng
hai con số ấy — không lần nào sang phải nhiều hơn, không lần nào lên ít hơn.

Hai đoạn nối liên tiếp mà có cùng "sang phải bao nhiêu, lên bao nhiêu" thì chúng
chỉ cùng một hướng. Mà nối tiếp nhau và cùng hướng thì không thể gãy khúc —
chúng nằm gọn trên một đường thẳng.

Đảo ngược câu ấy để thấy nó gánh nặng thế nào: nếu cột thứ ba **không** đứng im
— lúc +15000, lúc +40000 — thì đoạn này dốc hơn đoạn kia, và chỗ nối giữa chúng
là một chỗ gãy. Đường thẳng biến mất ngay lập tức. Vậy cái làm nên đường thẳng
không phải phép nhân, không phải con số 15 000; nó là **sự cố định** của cột thứ
ba.
::::

::::example{#buoc-deu-thi-thang}
Đây là sáu cái chấm đầu tiên của `thu`, mỗi hàng dọc cách nhau đúng 15 000:

```text
  t(n)
 75000 ┤                B
 60000 ┤             B
 45000 ┤          B
 30000 ┤       B
 15000 ┤    B
     0 ┼─B─────────────────→ n
         0  1  2  3  4  5
```

Sang phải một ô, lên một hàng. Sang phải một ô, lên một hàng. **Năm** lần như
nhau — sáu cái chấm thì chỉ có năm khoảng giữa chúng — nên đặt cây thước lên là
trúng cả sáu chấm.

Muốn chắc thì hỏi thẳng cái máy, và hỏi ở những chỗ xa nhau trên bảng:

```python title=readonly
def thu(n):
    return 15000 * n

print(thu(1) - thu(0))
print(thu(5) - thu(4))
print(thu(100) - thu(99))
```

Máy in ra:

```text
15000
15000
15000
```

Đầu bảng, giữa bảng, hay chỗ mà bạn sẽ không bao giờ đủ kiên nhẫn chấm tay tới —
vẫn 15 000. Cột thứ ba đứng im ở **mọi** chỗ, không phải ở vài chỗ đầu.

Chú ý cách viết trong ba dòng `print` ấy, vì nó dùng đúng thứ bài trước vừa dựng:
`thu(100)` là một **con số**, `thu(99)` cũng là một con số, nên trừ chúng cho
nhau là chuyện bình thường. Nếu `thu` một mình mà đem trừ nhau thì mới vô nghĩa —
đó là trừ hai cái máy.
::::

::::predict{#doan-ba-hieu commitOnce}
Byte đo cột thứ ba ở ba chỗ. Hai chỗ đầu là một bước; chỗ thứ ba thì hãy đọc kỹ
hai con số trong ngoặc.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def thu(n):
    return 15000 * n

print(thu(1) - thu(0))
print(thu(8) - thu(7))
print(thu(8) - thu(6))
```

:::opt{correct}
15000 rồi 15000 rồi 30000
:::

:::opt
15000 cả ba dòng
::why
Gần đúng ở chỗ bạn đang cầm đúng luật của bài này và cầm chắc: *máy `thu` thêm
đều 15 000, ở đầu bảng hay cuối bảng cũng thế*. Luật ấy thật, và bạn áp nó không
sai một chữ nào cho hai dòng đầu.

Ranh giới của luật nằm ngay trong cách phát biểu nó: 15 000 là lượng đổi khi đầu
vào thêm **đúng 1**. Dòng thứ ba đi từ 6 lên 8 — hai bước, không phải một. Hai
bước thì thêm hai lần 15 000. Luật không sai, nó chỉ đang được hỏi một câu khác
với câu nó trả lời.
::
:::

:::opt
1 rồi 1 rồi 2
::why
Gần đúng ở chỗ bạn đọc ra đúng thứ mà cả bài này xoay quanh: khoảng cách giữa
hai đầu vào là 1, 1 và 2. Nhìn ra được con số 2 ở dòng cuối là đã vượt qua cái
bẫy chính rồi.

Chỗ lệch là dấu trừ đang đứng ở đâu. Nó không nằm giữa hai con số trong ngoặc —
nó nằm giữa `thu(8)` và `thu(6)`, tức là giữa hai **thứ máy nhả ra**. Máy nhận
số ổ nhưng nhả ra tiền, nên phép trừ ấy cho một khoản tiền, không cho một số ổ.
Muốn máy trả lời "cách nhau mấy ổ" thì phải viết `8 - 6`, không có `thu` nào cả.
::
:::

:::opt
Máy báo lỗi, vì không trừ hai cái máy cho nhau được
::why
Gần đúng ở chỗ bạn nhớ rất đúng một điều bài trước vừa dặn, và dặn nghiêm túc:
`thu` một mình **không** phải con số — nó là một cái máy, và trừ máy này cho máy
kia thì không có nghĩa gì.

Ranh giới là cặp ngoặc. `thu` là cái máy, nhưng `thu(8)` thì không: chỗ trống đã
được điền, máy đã chạy xong, và thứ còn lại trên bàn là con số 120 000. Cái đang
bị trừ ở đây là hai **kết quả**, không phải hai cái máy — nên phép trừ hoàn toàn
hợp lệ.
::
:::
::::

::::explain{#doc-am-la-duong-di-xuong}
Xe bánh mì còn một cái máy nữa, và nó cho thấy luật vừa học rộng hơn ta tưởng.

Byte nướng lại bánh trên lò than. Mỗi ổ nướng hết **200 gam than**. Lấy lúc mới
nhóm lò làm **mốc 0** — đúng kiểu mốc 0 của T2.1 — thì máy này trả lời câu: *sau
n ổ, chỗ than trong lò đổi bao nhiêu gam so với lúc mới nhóm?*

> `h(n) = −200 × n`

Bảng của nó, vẫn ba cột:

```text
  n │     h(n) │ bước từ dòng trên
───┼──────────┼───────────────────
  0 │        0 │
  1 │     -200 │              -200
  2 │     -400 │              -200
  3 │     -600 │              -200
  4 │     -800 │              -200
```

Cột thứ ba lại đứng im. Nó là **−200**, và số âm ở đây không có gì bí ẩn: mỗi ổ
làm chỗ than **hụt** 200 gam, mà hụt thì đi xuống dưới mốc.

Trên hình, sự đổi dấu ấy hiện ra thành một chuyện duy nhất — đường quay đầu:

```text
  h(n)
     0 ┼─H─────────────────→ n
  -200 ┤    H
  -400 ┤       H
  -600 ┤          H
  -800 ┤             H
 -1000 ┤                H
         0  1  2  3  4  5
```

Vẫn sang phải một ô, vẫn xuống đúng một hàng, sáu lần như nhau — nên nó vẫn
**thẳng**. Không có luật thứ hai nào ở đây cả:

> Đầu ra đổi một lượng cố định thì đồ thị thẳng. Lượng ấy dương thì đường đi
> lên, âm thì đường đi xuống, và **0** thì đường nằm ngang.

Ba trường hợp, một luật. Cái quyết định thẳng hay không thẳng là **cố định hay
không cố định**, chứ không phải dương hay âm.
::::

::::code{#do-cot-thu-ba}
Đo cột thứ ba của hai cái máy, bằng chính hai cái máy — đừng chép con số ra từ
dòng `return`.

Đo ở **hai chỗ khác nhau** trên mỗi bảng, vì đó là cách duy nhất kiểm được lời
hứa "lượng đổi cố định": đo một chỗ thì chỉ ra một con số, đo hai chỗ mới so
được.

Năm chỗ trống:

- `buoc_thu` — máy `thu` đổi bao nhiêu khi `n` đi từ 0 lên 1.
- `buoc_than` — máy `than` đổi bao nhiêu khi `n` đi từ 0 lên 1.
- `buoc_thu_cho_khac` — vẫn máy `thu`, nhưng `n` đi từ 8 lên 9.
- `buoc_than_cho_khac` — vẫn máy `than`, nhưng `n` đi từ 5 lên 6.
- `hai_buoc_thu` — máy `thu`, `n` đi từ 7 lên 9. Hai bước liền, không phải một.

```python title=starter
def thu(n):
    return 15000 * n

def than(n):
    return -200 * n

# Một bước: n thêm đúng 1.
buoc_thu = ___
buoc_than = ___

# Đo lại ở chỗ khác trên bảng. Nếu luật đúng thì hai con số phải trùng nhau.
buoc_thu_cho_khac = ___
buoc_than_cho_khac = ___

# Hai bước liền nhau.
hai_buoc_thu = ___

print(buoc_thu)
print(buoc_than)
print(hai_buoc_thu)
```

```python title=solution
def thu(n):
    return 15000 * n

def than(n):
    return -200 * n

# Một bước: n thêm đúng 1.
buoc_thu = thu(1) - thu(0)
buoc_than = than(1) - than(0)

# Đo lại ở chỗ khác trên bảng. Nếu luật đúng thì hai con số phải trùng nhau.
buoc_thu_cho_khac = thu(9) - thu(8)
buoc_than_cho_khac = than(6) - than(5)

# Hai bước liền nhau.
hai_buoc_thu = thu(9) - thu(7)

print(buoc_thu)
print(buoc_than)
print(hai_buoc_thu)
```

```python title=test
# Ba câu `!=` đứng trước. Chúng canh cái bẫy "điền chung một con số cho mọi chỗ
# trống"; xếp chúng sau các câu `==` thì chúng không bao giờ chạy tới.
assert buoc_than != buoc_thu, "máy thu đi lên, máy than đi xuống — hai độ dốc này không thể là cùng một số"
assert hai_buoc_thu != buoc_thu, "hai bước liền phải đổi nhiều hơn một bước, không được ra cùng một số"
assert buoc_thu != 0, "máy thu có đổi thật khi n thêm 1, nên bước của nó không phải 0"
assert buoc_thu == buoc_thu_cho_khac, "đây là điều cần kiểm: đo ở đầu bảng hay giữa bảng, bước của máy thu vẫn phải y hệt nhau"
assert buoc_than == buoc_than_cho_khac, "bước của máy than cũng phải y hệt nhau ở mọi chỗ trên bảng"
assert buoc_thu == 15000, "n thêm 1 thì thu thêm đúng 15 000"
assert buoc_than == -200, "n thêm 1 thì chỗ than hụt 200 gam, nên bước là -200 chứ không phải 200"
assert hai_buoc_thu == 2 * buoc_thu, "đi hai bước thì đổi gấp đôi một bước"
assert hai_buoc_thu == 30000, "từ 7 lên 9 là hai bước, mỗi bước 15 000"
```

:::hints
- kind: attention
  body: Cột thứ ba trong bảng ở đầu bài được tính bằng cách nào? Nó lấy kết quả của một dòng trừ đi kết quả của dòng ngay trên. Ở đây bạn có sẵn hai cái máy để lấy kết quả của bất kỳ dòng nào — chỉ cần gọi chúng ở hai chỗ rồi trừ.
- kind: strategy
  body: Mỗi chỗ trống cần HAI lần gọi máy, không phải một, vì "đổi bao nhiêu" là câu hỏi về hai dòng chứ không phải một dòng. Viết kết quả ở chỗ SAU trừ đi kết quả ở chỗ TRƯỚC, đúng thứ tự đó — đảo ngược lại thì dấu cũng đảo theo. Đừng chép con số 15000 hay 200 từ dòng `return` xuống: chép thì bạn đang tin lời hứa chứ chưa kiểm nó.
- kind: one-line
  body: "Năm chỗ lần lượt là `thu(1) - thu(0)`, `than(1) - than(0)`, `thu(9) - thu(8)`, `than(6) - than(5)`, `thu(9) - thu(7)`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải GỌI cái máy ở hai chỗ rồi trừ hai kết quả cho nhau — chép con số từ dòng `return` xuống thì bạn chưa đo gì cả, chỉ đang tin lời hứa
  requireAst:
  # Năm chỗ trống, mỗi chỗ một phép trừ. Khung khởi đầu không có dấu trừ nào —
  # `-200 * n` là dấu âm dính vào con số, không phải phép trừ hai vế — nên
  # `min: 5` chặn đúng đáp án chép cứng năm con số.
  - kind: uses-operator, target: -, min: 5
  # Có dấu trừ thôi chưa đủ: `15000 - 0` cũng là một phép trừ mà không gọi máy
  # nào. Hai luật dưới buộc mỗi máy phải được HỎI đúng số lần: `thu` ở ba chỗ
  # trống (2 lần gọi mỗi chỗ) và `than` ở hai chỗ trống.
  - kind: uses-call, target: thu, min: 6
  - kind: uses-call, target: than, min: 4
  forbidAst:
  # Lưới thứ hai, chặn con số KẾT QUẢ của chỗ trống cuối. Lời giải thật chỉ
  # chứa 15000, 200, và các số ổ (0, 1, 5, 6, 7, 8, 9) — không chỗ nào có
  # nguyên văn 30000.
  - kind: has-literal, target: 30000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^15000\n-200\n30000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đo ở đầu bảng, đo ở giữa bảng — vẫn ra một con số. Thảo nào cái hình nó thẳng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sáng nay bên cạnh Byte có thêm một xe bánh mì nữa, của An. Hai xe bán **cùng
giá**: 15 nghìn một ổ. Khác một chỗ — An mới tập bán nên đẩy xe ra ngay trước
cửa nhà mình, không mất đồng nào tiền chỗ; còn Byte vẫn ở góc chợ và mỗi buổi
phải trả **30 nghìn tiền thuê chỗ**, trả trước từ lúc dọn hàng ra.

Hỏi cả hai cùng một câu: *sau khi bán n ổ, đang cầm bao nhiêu tiền?*

Đo cột thứ ba của hai xe thì thấy chúng bằng nhau: bán thêm một ổ, xe nào cũng
cầm thêm đúng 15 nghìn. Tức là **hai đường có cùng độ dốc**.

Nhưng hai xe rõ ràng không giống nhau. Bán được 2 ổ, An đã cầm 30 nghìn trong
tay, còn Byte thì vừa đủ gỡ tiền thuê chỗ.

Vậy nếu chấm cả hai lên **cùng một mặt phẳng**, chúng khác nhau ở chỗ nào? Cùng
độ dốc thì cùng hướng, mà cùng hướng thì hai đường ấy không thể cắt nhau. Chúng
sẽ nằm ở đâu so với nhau — và có con số nào trên bảng nói ra chỗ khác biệt ấy
không?

Thử dòng đầu tiên của hai cái bảng xem: dòng `n = 0`, lúc chưa ai bán ổ nào.

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
