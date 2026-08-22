---
id: toan.cam-nhan-so.ti-so-so-hai-thu-khac-loai
title: Tỉ số — so hai thứ không cùng loại
summary: Phần trăm cần một cái toàn thể chứa phần được so. Khi hai thứ chẳng cái nào chứa cái nào, người ta ghép đôi chúng lại và đếm theo cặp.
locale: vi
track: toan
module: cam-nhan-so
order: 40
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.ratio]
requires: [core.arithmetic, core.variable, core.assignment, core.print-variable, core.number-literal]
concepts: [math.ti-so, math.don-vi-ghep]
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
Gáo nước không nằm trong gốc cây. Mình vẫn ghép đôi hai thứ ấy được.
::::

::::explain{#phan-tram-khong-cam-noi}
Bài trước đóng lại ở một chỗ chặt: mỗi con số phần trăm luôn dính vào một **cái
toàn thể** = 100%. "30%" chỉ đủ nghĩa khi nói rõ 30% *của cái gì*, và cái gì ấy
phải **chứa** phần được lấy ra. 30% vườn là 30 phần trong 100 phần của chính
cái vườn ấy.

Bây giờ tới chuyện phần trăm không cầm nổi.

Sáng nay Byte tưới vườn bằng gáo. Múc từ lu, tưới tới lúc hết nước thì múc
tiếp. Cuối buổi đếm lại: **8 gáo nước**, **20 gốc cây**.

Hỏi thử: "gáo nước chiếm bao nhiêu phần trăm số gốc cây?"

Câu hỏi ấy hỏng ngay từ chữ *chiếm*. Gáo nước không nằm trong số gốc cây, gốc
cây cũng không nằm trong số gáo. Chúng là hai loại khác nhau — bài 1 gọi đó là
hai **đơn vị** khác nhau, và bài 11 đã chốt bằng một câu: chỉ gộp được thứ cùng
đơn vị. Không có cái toàn thể nào ở đây để làm mốc 100%.

Vậy mà rõ ràng có một quan hệ thật giữa hai con số ấy: hết 8 gáo thì tưới xong
20 gốc, không phải 5 gốc, cũng không phải 200 gốc. Cần một cách cầm quan hệ đó
mà không phải bắt bên nào chứa bên nào.
::::

::::explain{#ghep-doi-hai-thu-khac-loai}
Cách cầm nó nằm ngay trong việc Byte vừa làm.

Byte không đổ ào 8 gáo một lúc. Byte múc **2 gáo**, tưới đủ **5 gốc** rồi hết
nước. Múc thêm 2 gáo, tưới đủ 5 gốc nữa. Bốn lượt như thế thì vườn xong.

Cả buổi sáng gói lại thành một câu duy nhất:

> **Cứ 2 gáo thì tưới được 5 gốc.**

Câu ấy có tên trong toán: **tỉ số** giữa gáo và gốc, viết `2 : 5` và đọc là
"hai trên năm" hoặc "hai ăn năm".

Ba điều phải để ý ngay, vì mỗi điều đều là chỗ tỉ số khác hẳn những con số bạn
đã gặp:

- **Hai vế mang hai tên đơn vị khác nhau.** `2 : 5` chưa nói gì cho tới khi bạn
  đọc kèm "gáo" và "gốc". Đây vẫn là luật bài 1, chỉ là bây giờ một con số phải
  kéo theo *hai* cái tên.
- **Thứ tự có nghĩa.** `2 : 5` là gáo trước, gốc sau. `5 : 2` là câu khác hẳn —
  cứ 5 gáo cho 2 gốc, một cái vườn úng nước.
- **Không bên nào phải chứa bên nào.** Đây đúng là chỗ tỉ số làm được mà phần
  trăm không: nó so hai thứ đứng cạnh nhau, chứ không so một phần với cái bọc
  quanh nó.
::::

::::example{#chep-len-nhieu-lan}
Byte chỉ cần nhớ đúng một bộ đôi `2 : 5`, rồi chép nó lên nhiều lần cho vườn to
nhỏ khác nhau. Vẽ ra thì thấy ngay — đây là thanh số kép, hai thanh số của bài 5
chạy song song, mỗi vạch trên là một bộ đôi:

```text
gáo   :   2    4    6    8   10   12   14
gốc   :   5   10   15   20   25   30   35
```

Cũng chừng ấy chuyện, vẽ bằng sơ đồ dải thì lộ ra mặt khác — mỗi khối là một
lượt múc, và hai dải luôn có **cùng số khối**:

```text
Vườn 4 lượt:  gáo  [██][██][██][██]                = 8 gáo
              gốc  [█████][█████][█████][█████]    = 20 gốc
```

Hỏi thẳng cái máy, chép bộ đôi lên 3 lần rồi 10 lần:

```python title=readonly
gao_mau = 2
goc_mau = 5

print(gao_mau * 3, goc_mau * 3)
print(gao_mau * 10, goc_mau * 10)
```

```text
6 15
20 50
```

`6 : 15`, `20 : 50` và `2 : 5` là **cùng một tỉ số**, dù ba cặp số trông chẳng
giống nhau. Vì sao được phép nói vậy? Không phải vì có luật cho phép. Vì chép
một bộ đôi ra nhiều bản thì mỗi bản vẫn nói đúng câu cũ — "2 gáo cho 5 gốc" —
và không bản nào tự thêm bớt điều gì. Đây đúng là bài 33 nói lại ở chỗ mới:
nhân cả hai vế với cùng một số thì lượng thật không đổi, chỉ có cách đếm đổi.

Chỗ này có một cái bẫy, và nó là cái bẫy phổ biến nhất của cả bài. "Cùng nhau"
nghĩa là cùng **nhân**, không phải cùng **cộng**:

```text
Chép bộ đôi (nhân):   2 : 5  →  4 : 10  →  6 : 15  →  8 : 20
Cộng thêm 2 vào cả hai:   2 : 5  →  4 : 7   →  6 : 9   →  8 : 11
```

Dòng dưới sai, và sai kiểm được ngoài vườn: cầm 4 gáo đi tưới thật thì được 10
gốc, không phải 7. Cộng một lượng bằng nhau vào hai vế mang hai đơn vị khác
nhau chính là thứ bài 11 cấm — "2" ở vế trên là gáo, "2" ở vế dưới là gốc, hai
con số ấy không phải cùng một thứ để đem cộng song song.
::::

::::predict{#doan-hai-con-so commitOnce}
Byte chép bộ đôi `2 : 5` lên bốn lần cho vườn nhà An.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
gao = 2
goc = 5
print(gao * 4, goc * 4)
```

:::opt{correct}
8 20
:::

:::opt
8 5
::why
Gần đúng ở chỗ bạn giữ một vế đứng yên — và đó đúng là cách phần trăm làm việc
ở bài 38 với bài 39: cái toàn thể luôn cố định ở 100, chỉ có phần lấy ra mới
thay đổi. Nếu `2 : 5` là một phép chia phần thì bạn đã trúng.

Chỗ lệch là ở tỉ số không có vế nào đóng vai cái toàn thể. Cả gáo lẫn gốc đều
là thứ đếm được ngoài vườn, và mỗi lượt múc sinh ra **cả hai** cùng lúc. Chép
bộ đôi lên bốn lần nghĩa là làm bốn lượt múc, nên bốn lượt ấy vừa cho 8 gáo vừa
cho 20 gốc. Đổ 8 gáo lên đúng 5 gốc là tưới úng.
::
:::

:::opt
6 9
::why
Gần đúng ở chỗ bạn cho hai vế thay đổi **cùng nhau**. Đó chính là luật của tỉ
số, và người chỉ động vào một vế mới là người đi lạc — bạn nắm đúng phần cốt
lõi.

Chỗ lệch nằm ở chữ "cùng": cùng **nhân**, không phải cùng **cộng**. Cộng 4 vào
hai vế cho `6 : 9`, nhưng đi múc thật 6 gáo thì tưới được 15 gốc chứ không phải
9 — bạn đếm lại trên thanh số kép ở trên là thấy. Phép cộng ở đây còn vướng
thêm một chuyện nữa: con số 4 cộng vào vế gáo mang đơn vị "gáo", con số 4 cộng
vào vế gốc mang đơn vị "gốc". Chúng chỉ trông giống nhau trên giấy.
::
:::

:::opt
2 5
::why
Gần đúng ở chỗ bạn nhớ đúng câu quan trọng nhất của bài: nhân cả hai vế lên thì
**tỉ số không đổi**. Câu đó không sai một chữ nào.

Chỗ lệch là thứ không đổi ấy là *quan hệ* giữa hai vế, chứ không phải hai con
số đang đứng trên màn hình. `2 : 5` và `8 : 20` là cùng một tỉ số, nhưng chúng
là hai cái vườn khác cỡ hẳn nhau — một cái 5 gốc, một cái 20 gốc. Dòng `print`
này hỏi hai con số của cái vườn đã chép lên bốn lần, không hỏi tên của quan hệ.
::
:::
::::

::::explain{#mot-cau-cho-moi-co-vuon}
Rút lại thành một câu mang theo được: **tỉ số là một câu "cứ … thì …", và câu
ấy vẫn đúng nguyên văn khi bạn chép nó lên bao nhiêu lần cũng được.**

Nhờ vậy Byte không phải nhớ từng cái vườn. Nhớ `2 : 5` là đủ cho mọi cỡ vườn:
vườn 4 lượt, vườn 7 lượt, vườn 100 lượt — tất cả đều là cùng một câu, chép ra
nhiều bản.

Còn một chỗ cần nói thẳng trước khi bạn tự tay làm: ở bài này bạn luôn được cho
sẵn **số lượt**. Chuyện gì xảy ra khi người ta chỉ đưa bạn hai con số cuối cùng
và không nói chép mấy lần — đó là chuyện của cuối bài.
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Hai cái vườn, một câu luật. Bạn chép câu ấy ra cho cả hai giúp mình nhé.
::::

::::code{#chep-luat-tuoi}
Luật tưới của Byte vẫn thế: **cứ 2 gáo thì tưới được 5 gốc**.

Hai nhà hàng xóm nhờ Byte tính hộ, và hai nhà có cỡ vườn khác nhau:

- **Vườn nhà An** chạy đúng **4 lượt** múc.
- **Vườn nhà Tú** chạy đúng **7 lượt** múc.

Điền bốn chỗ trống sao cho mỗi vườn được chép ra từ đúng bộ đôi mẫu.

Bài chấm bằng **cả hai vườn cùng lúc**, và hai vườn được chọn khác số lượt
đúng vì lý do đó: một cặp số gõ cứng vào cả bốn chỗ thì hỏng ít nhất một vườn,
mà nhân đúng một vế thì hỏng cả hai. Chỉ có chép cả bộ đôi, ở cả hai vườn, mới
qua được.

```python title=starter
gao_mau = 2
goc_mau = 5

gao_an = ___
goc_an = ___

gao_tu = ___
goc_tu = ___

print(gao_an, goc_an)
print(gao_tu, goc_tu)
```

```python title=solution
gao_mau = 2
goc_mau = 5

gao_an = gao_mau * 4
goc_an = goc_mau * 4

gao_tu = gao_mau * 7
goc_tu = goc_mau * 7

print(gao_an, goc_an)
print(gao_tu, goc_tu)
```

```python title=test
# Hai vườn, hai số lượt khác nhau — nên không có cặp số nào điền được vào cả
# bốn chỗ mà vẫn qua. Bốn dòng đầu chốt từng con số; hai dòng cuối chốt điều
# bài này thật sự dạy: mỗi vườn phải là bộ đôi mẫu chép lên, ở CẢ HAI vế cùng
# một số lần.
assert gao_an == 8, "4 lượt múc, mỗi lượt 2 gáo — vế gáo của vườn An phải là 8"
assert goc_an == 20, "4 lượt múc, mỗi lượt tưới 5 gốc — vế gốc của vườn An phải là 20"
assert gao_tu == 14, "7 lượt múc, mỗi lượt 2 gáo — vế gáo của vườn Tú phải là 14"
assert goc_tu == 35, "7 lượt múc, mỗi lượt tưới 5 gốc — vế gốc của vườn Tú phải là 35"
assert gao_an == gao_mau * 4 and goc_an == goc_mau * 4, "vườn An phải nhân CẢ HAI vế với 4"
assert gao_tu == gao_mau * 7 and goc_tu == goc_mau * 7, "vườn Tú phải nhân CẢ HAI vế với 7"
```

:::hints
- kind: attention
  body: Hai dòng đầu đã có sẵn bộ đôi mẫu `gao_mau` và `goc_mau`. Bốn chỗ trống không hỏi bốn con số mới — chúng hỏi bộ đôi ấy được chép lên mấy lần.
- kind: strategy
  body: Chép bộ đôi lên 4 lần nghĩa là nhân với 4 ở CẢ hai vế, một vế lấy `gao_mau`, vế kia lấy `goc_mau`. Vườn nhà Tú làm y hệt, chỉ thay 4 bằng 7 — và cũng phải thay ở cả hai vế.
- kind: one-line
  body: "Viết `gao_mau * 4` rồi `goc_mau * 4` cho vườn An, `gao_mau * 7` rồi `goc_mau * 7` cho vườn Tú."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải chép bộ đôi mẫu lên — tức là nhân `gao_mau` hoặc `goc_mau` với số lượt, chứ không gõ thẳng con số kết quả
  requireAst:
  - kind: uses-operator, target: *, min: 4
  - kind: uses-name, target: goc_mau, min: 2
- tier: output
  match: regex
  expect: ^8 20\n14 35\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai vườn khác cỡ, mà vẫn đúng một câu: cứ 2 gáo thì 5 gốc.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Suốt bài này bạn luôn được cho **số lượt** — chép 4 lần, chép 7 lần — nên chép
xong là xong. Ngoài đời thì người ta hiếm khi đưa bạn số lượt.

Chiều nay Byte và An mỗi người đi gieo hạt, gieo xong mới đếm:

- Vườn Byte: **90 hạt** trên **6 luống**.
- Vườn An: **75 hạt** trên **5 luống**.

Hai tỉ số: `90 : 6` và `75 : 5`. Nhìn vào thì chẳng giống nhau, và cũng không
có cách nào chép cái này ra thành cái kia — nhân `75 : 5` lên 1 lần thì chưa
tới, lên 2 lần thì quá.

Byte muốn biết vườn nào **gieo dày hơn**, tức là mỗi luống chịu nhiều hạt hơn.
Vườn Byte nhiều hạt hơn thật, nhưng cũng nhiều luống hơn — nên "nhiều hạt hơn"
chưa trả lời được câu nào.

Hai bộ đôi khác hẳn nhau thì đem so kiểu gì? Bài sau trả lời, và câu trả lời là
kéo cả hai về cùng một cỡ mà bạn đã biết trước.
::::

::::checkpoint{mastery=0.8}
::::
