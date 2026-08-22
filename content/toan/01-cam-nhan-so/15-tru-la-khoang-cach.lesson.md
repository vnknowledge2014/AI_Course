---
id: toan.cam-nhan-so.tru-la-khoang-cach
title: Trừ là khoảng cách
summary: "`a − b` còn đọc được cách thứ hai: khoảng cách từ b tới a — tức phải cộng thêm bao nhiêu vào b thì tới a."
locale: vi
track: toan
module: cam-nhan-so
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.subtraction-as-distance]
requires: [math.number-line-subtract, math.number-line-add, math.thanh-so, math.addition-as-union, core.variable, core.arithmetic, core.boolean, ctrl.comparison, core.print-variable]
concepts: [math.khoang-cach, math.so-do-dai, math.don-vi]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Hai câu hỏi chẳng dính gì nhau mà ra cùng một số. Mình muốn biết vì sao.
::::

::::explain{#hai-cau-hoi-mot-con-so}
Bài trước dừng ở chỗ khó chịu này. Đặt hai câu hỏi cạnh nhau:

- **Câu A — bớt đi.** Luống có 12 cây, sâu ăn mất 5. Còn mấy cây?
- **Câu B — còn bao xa.** Cả luống dài 12 sải, Byte cuốc tới vạch 5. Còn phải
  cuốc mấy sải nữa?

Câu B không có ai bớt gì của ai. Không con sâu nào ăn cây, không ai lấy đi một
sải đất nào. Luống vẫn dài đúng 12 sải như lúc mặt trời chưa lên. Câu B chỉ hỏi
**hai cái vạch nằm cách nhau bao xa**.

Nhìn kỹ hơn nữa thì hai câu còn khác nhau ở chỗ *cái chưa biết nằm đâu*:

- Câu A: biết chỗ xuất phát (12), biết đi mấy bước (5), hỏi **dừng ở vạch nào**.
- Câu B: biết chỗ xuất phát (5), biết dừng ở vạch nào (12), hỏi **đi mấy bước**.

Hai câu hỏi ngược nhau như hỏi *"đi ba tiếng thì tới đâu"* với *"tới đó thì đi
mấy tiếng"*. Thế mà cả hai đều ra 7, và cả hai đều viết được bằng đúng một dòng
`12 − 5`.

Đây không phải trùng hợp. Nó buộc phải thế, và bài 14 vừa đưa đủ đồ để chứng
minh.
::::

::::example{#vi-sao-buoc-phai-the}
Gọi cái chưa biết của câu B là `d` — số bước từ vạch 5 tới vạch 12. Nói `d` là
khoảng cách ấy nghĩa là:

```text
5 + d = 12
```

(đứng ở vạch 5, bước sang phải `d` vạch thì tới đúng vạch 12 — đó là bài 13.)

Bây giờ đi tính câu A, tức `12 − 5`. Thay `12` bằng chính cái nó bằng:

```text
12 − 5  =  (5 + d) − 5
        =  (d + 5) − 5      ← bài 10: gộp đống nào trước cũng ra một kết quả
        =  d                ← bài 14: tiến 5 rồi lùi 5 thì về đúng chỗ cũ
```

Vậy `12 − 5` **chính là** `d`. Không phải hai chuyện tình cờ ra cùng số; chúng
là một chuyện, kể từ hai đầu.

Có một bức tranh thứ ba làm chuyện này nhìn thấy được ngay, gọi là **sơ đồ
dải** — vẽ cả lượng thành một dải, rồi cắt:

```text
                   cả luống: 12 sải
   ├───────────────────────────────────────────────┤
   ├───────────────────┼───────────────────────────┤
      đã cuốc: 5 sải       còn phải cuốc: ? sải
```

Trên hình này, hỏi câu A hay câu B đều là chỉ tay vào **cùng một mảnh** — mảnh
bên phải. Câu A gọi nó là *chỗ còn lại sau khi bớt*. Câu B gọi nó là *chỗ thiếu
để cho đủ*. Mảnh thì chỉ có một, nên số đo của nó cũng chỉ có một.

Và một chỗ rất dễ trượt chân khi nhìn hình này: **đếm mảnh, đừng đếm vạch.** Từ
vạch 5 tới vạch 12 có tám cái vạch — 5, 6, 7, 8, 9, 10, 11, 12 — nhưng chỉ bảy
cái khoảng nằm giữa chúng. Sải dây nằm ở khoảng chứ không nằm ở vạch, y như bài
3 đã dặn: đo là đếm số lần đặt thước xuống. Hàng rào cũng vậy — tám cây cột thì
chỉ kẹp được bảy đoạn rào.

Hỏi máy, và hỏi luôn cả câu kiểm:

```python title=readonly
ca_luong = 12
da_cuoc = 5
con_phai_cuoc = ca_luong - da_cuoc

print(con_phai_cuoc)
print(da_cuoc + con_phai_cuoc)
```

Máy in ra:

```text
7
12
```

Dòng dưới là chỗ đáng nhìn: cộng phần còn thiếu vào phần đã có thì được lại
đúng cả luống. Đó là sơ đồ dải nói bằng số — hai mảnh ghép lại thành dải.
::::

::::predict{#doan-cau-kiem commitOnce}
Byte muốn hỏi máy hai câu **có–không**, và hai câu ấy chỉ khác nhau ở chỗ cộng
phần còn thiếu vào **mốc nào**:

- *Cộng phần còn phải cuốc vào phần đã cuốc, có ra đúng cả luống không?*
- *Cộng phần còn phải cuốc vào cả luống, có ra đúng phần đã cuốc không?*

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
ca_luong = 12
da_cuoc = 5
con_phai_cuoc = ca_luong - da_cuoc

print(con_phai_cuoc)
print(da_cuoc + con_phai_cuoc == ca_luong)
print(ca_luong + con_phai_cuoc == da_cuoc)
```

:::opt{correct}
7, True, rồi False
:::

:::opt
8, False, rồi False
::why
Gần đúng ở chỗ bạn đếm bằng đúng cách bài 2 dạy — ghép mỗi thứ với đúng một tên
số: 5, 6, 7, 8, 9, 10, 11, 12 là tám cái tên, không thừa không thiếu cái nào.
Phép đếm ấy không sai một chỗ.

Chỗ lệch là bạn đang đếm **vạch**, còn câu hỏi đòi đếm **sải dây**. Bài 3 đã nói
đo là đếm **số lần đặt thước xuống** — mà mỗi lần đặt là một cái *khoảng*, không
phải một cái *vạch*; bài 5 cũng hỏi "hai cái khoảng có bằng nhau không" chứ
không hỏi tên ba cái vạch. Đúng như sơ đồ dải phía trên vừa dặn: tám cái vạch
chỉ kẹp được bảy cái khoảng. Quy tắc "đếm tên số" đúng cho vật rời — hạt, cây,
người — chứ không đúng cho khoảng cách. Cứ đếm cây cột hàng rào thì ra tám, mà
đếm đoạn rào giữa chúng thì ra bảy.

Dòng giữa bắt được ngay chỗ lệch ấy, và đó là điều hay của câu thử lại: nếu còn
phải cuốc 8 sải thật thì `5 + 8` phải ra `12`, mà nó ra `13`.
::
:::

:::opt
7, 12, rồi 19
::why
Gần đúng ở chỗ bạn tính đúng cả hai vế trái, không lệch một đơn vị: `5 + 7` là
12 và `12 + 7` là 19. Riêng việc dòng giữa ra đúng cả luống chính là điều bài
này muốn bạn thấy.

Chỗ lệch nằm ở cái dấu. Trong vở toán, dấu `=` nói *"vế trái ra bao nhiêu"*, nên
đọc như bạn là đọc theo thói quen đúng suốt mười mấy năm. Nhưng `==` — hai dấu
bằng viết liền — là một **câu hỏi có–không**, thứ Realm 0 đã dựng. Thứ đi ra
khỏi nó luôn là `True` hoặc `False`, chưa bao giờ là một con số. Muốn thấy 12
thì bỏ hẳn phần `== ca_luong` đi, như dòng cuối của ví dụ phía trên.
::
:::

:::opt
7, True, rồi True
::why
Gần đúng ở chỗ bạn nắm đúng cái lõi của bài: **cộng phần còn thiếu vào thì phải
ra đủ.** Câu ấy đúng, và nó chính là câu thử lại mà bạn sẽ dùng suốt track.

Chỗ lệch là phạm vi: phần còn thiếu ấy thiếu **của một mốc cụ thể**, không thiếu
chung cho cả hai. Bảy sải là quãng từ vạch 5 *tới* vạch 12 — nên cộng nó vào
vạch 5 thì tới vạch 12, còn cộng nó vào vạch 12 là đi thêm bảy sải nữa, vượt
khỏi đầu kia của luống: `12 + 7` là 19, không phải 5. Đổi chỗ hai cái mốc là đổi
hẳn câu hỏi. Phép cộng thì đổi chỗ hai đống vẫn ra một đống chung (bài 10),
nhưng "từ đâu tới đâu" thì không có luật ấy — nó có chiều đi.
::
:::
::::

::::explain{#mot-phep-hai-cach-doc}
Gói lại thành một câu mang đi được:

> `a − b` là **số phải cộng thêm vào `b` để tới `a`**.

Đây là cách nói chắc chân nhất, vì nó không đòi bên nào phải lớn hơn bên nào —
nó chỉ hỏi *đi từ `b` tới `a` thì đi thế nào*. Nó biến mọi phép trừ thành một
phép cộng còn thiếu một chỗ, và cho bạn một cách thử lại không cần nhớ luật gì:
`b + (a − b)` phải ra đúng `a`. Cộng lại không ra thì trừ hỏng.

Hai cách nói quen tai hơn cùng chỉ vào **một mảnh** trên sơ đồ dải:

> *"bớt `b` khỏi `a`"* — đó là bài 14; và *"từ `b` tới `a` còn cách bao xa"* —
> **chừng nào `b` chưa vượt qua `a` trên thanh số**.

Cái vế in đậm ấy là **phạm vi**, và nó không phải chữ thừa. "Còn cách bao xa" là
hỏi một chiều dài, mà chiều dài thì luôn là một con số đếm được, không bao giờ
ít hơn không. Khi `b` đã đứng bên phải `a` thì hỏi khoảng cách vẫn hỏi được,
nhưng câu trả lời không còn là `a − b` nữa. Cuối bài Byte sẽ dẫm đúng vào chỗ ấy
— và đó là chỗ bài sau bắt đầu.

Cách đọc mới trả tiền ở đúng chỗ mà tranh "bớt đi" chịu thua: những câu hỏi
**không ai bớt gì của ai**.

- Tàu chạy lúc 9 giờ, bây giờ 6 giờ — còn 3 tiếng. Không ai lấy đi 6 tiếng của
  ai cả; hai cái mốc giờ chỉ đứng cách nhau 3 tiếng.
- Cần 20 nghìn mua rau, trong túi có 12 nghìn — thiếu 8 nghìn. Số 8 ấy chưa hề
  tồn tại để mà bớt.
- An cao 138 phân, Byte cao 125 phân — An cao hơn 13 phân. Chẳng ai lùn đi.

Và một chỗ nhỏ đáng để ý, vì bài 1 đã dặn: cùng ra con số 7, nhưng **7 cái gì**
thì hai câu khác nhau. Câu A ra *7 cây* — thứ đếm được. Câu B ra *7 sải dây* —
thứ đo được. Con số dùng chung, đơn vị thì không.
::::

::::code{#con-bao-xa-nua}
Hai câu hỏi "còn bao xa", ở hai chỗ chẳng liên quan gì nhau.

- **Luống của Byte**: cả luống dài `12` sải, đã cuốc tới vạch `5`. Còn phải cuốc
  mấy sải?
- **Chuyến tàu về quê**: bây giờ mới `6` giờ, tàu chạy lúc `9` giờ. Còn mấy
  tiếng nữa?

Điền hai chỗ trống. Hai dòng `print` cuối là **câu thử lại** đã viết sẵn: cộng
phần còn thiếu vào phần đã có, phải ra đúng cái đích — nên nếu bạn điền đúng,
cả hai dòng ấy phải nói `True`.

Bài chấm bằng **cả hai chuyện**, và chúng cho ra hai con số khác nhau (`7` và
`3`). Gõ cứng một con số vào cả hai chỗ thì có một chỗ sai, và câu thử lại sẽ
nói `False` ngay tại chỗ ấy.

Còn một chỗ cố ý nữa: hai chuyện xếp hai cái mốc theo **hai thứ tự khác nhau**.
Chép máy móc "cái tên dòng trên trừ cái tên dòng dưới" thì một trong hai chuyện
sẽ hỏng — và câu thử lại sẽ nói `False` ngay tại chuyện ấy.

```python title=starter
ca_luong = 12
da_cuoc = 5
con_phai_cuoc = ___

gio_bay_gio = 6
gio_tau_chay = 9
con_may_tieng = ___

print(con_phai_cuoc)
print(con_may_tieng)
print(da_cuoc + con_phai_cuoc == ca_luong)
print(gio_bay_gio + con_may_tieng == gio_tau_chay)
```

```python title=solution
ca_luong = 12
da_cuoc = 5
con_phai_cuoc = ca_luong - da_cuoc

gio_bay_gio = 6
gio_tau_chay = 9
con_may_tieng = gio_tau_chay - gio_bay_gio

print(con_phai_cuoc)
print(con_may_tieng)
print(da_cuoc + con_phai_cuoc == ca_luong)
print(gio_bay_gio + con_may_tieng == gio_tau_chay)
```

```python title=test
# Hai chuyện cho ra hai con số khác nhau, nên một con số gõ cứng chỉ qua được
# nhiều nhất một câu. Hai câu cuối là chính cách đọc mà bài này dạy: cộng phần
# còn thiếu vào phần đã có thì phải ra đúng cái đích.
assert con_phai_cuoc == 7, "cả luống 12 sải, đã cuốc 5, thì còn 7 sải"
assert con_may_tieng == 3, "6 giờ tới 9 giờ là 3 tiếng"
assert da_cuoc + con_phai_cuoc == ca_luong, "5 sải đã cuốc cộng phần còn lại phải ra đúng 12 sải"
assert gio_bay_gio + con_may_tieng == gio_tau_chay, "6 giờ cộng phần chờ phải ra đúng giờ tàu chạy"
```

:::hints
- kind: attention
  body: "Hai dòng ngay trên mỗi chỗ trống cho bạn hai cái mốc: một cái đích và một chỗ đang đứng. Câu hỏi là hai mốc ấy cách nhau bao xa. Đọc kỹ xem ở mỗi chuyện, cái nào là đích — vì hai chuyện không xếp giống nhau."
- kind: strategy
  body: "Khoảng cách từ mốc đang đứng tới mốc đích viết bằng một phép trừ, và thứ tự có quan trọng: cái đích đứng trước dấu trừ, chỗ đang đứng đứng sau. Thử lại bằng câu ở dòng print cuối — cộng kết quả vào chỗ đang đứng phải ra đúng cái đích."
- kind: one-line
  body: "Chỗ trống thứ nhất là `ca_luong - da_cuoc`, chỗ trống thứ hai là `gio_tau_chay - gio_bay_gio`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một phép trừ giữa hai cái mốc — gõ thẳng con số khoảng cách thì bạn đã đo hộ máy rồi
  requireAst:
  # `min: 2` vì có hai chuyện, mỗi chuyện một phép trừ. Khung khởi đầu không có
  # dấu `-` nào (hai dòng cuối chỉ có `+` và `==`), nên luật này chặn được đúng
  # cái đáp án chép cứng hai con số.
  - kind: uses-operator, target: -, min: 2
  # Có dấu `-` thôi thì chưa đủ: `con_phai_cuoc = 12 - 5` cũng là một phép trừ
  # mà không đọc cái tên nào giữ mốc — mà cả bài này là chuyện HAI CÁI MỐC nằm
  # cách nhau bao xa, nên hai cái mốc phải có mặt bằng tên.
  #
  # `min: 2` chứ không phải `min: 1`: khung khởi đầu đã đọc sẵn mỗi tên một lần
  # ở hai dòng thử lại (`da_cuoc + con_phai_cuoc == ca_luong` và dòng dưới nó),
  # nên `min: 1` thoả ngay cả khi chỗ trống chép cứng con số. Lần đọc thứ hai
  # chỉ có thể tới từ chính chỗ trống.
  - kind: uses-name, target: ca_luong, min: 2
  - kind: uses-name, target: da_cuoc, min: 2
  - kind: uses-name, target: gio_tau_chay, min: 2
  - kind: uses-name, target: gio_bay_gio, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^7\n3\nTrue\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bảy sải và ba tiếng. Chẳng ai bớt gì của ai, mà phép trừ vẫn đo được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Câu vừa gói lại ở trên có một cái ngưỡng — *chừng nào `b` chưa vượt qua `a`*.
Bây giờ là lúc cố tình đem nó ra ngoài ngưỡng ấy xem chuyện gì xảy ra.

Khoảng cách thì không phân biệt chiều. Từ vạch 5 tới vạch 12 là 7 bước sang
phải; từ vạch 12 về vạch 5 cũng đúng 7 bước, chỉ là quay đầu lại. Sợi dây nằm
giữa hai vạch dài 7 sải, đi kiểu nào cũng thế.

Nhưng hai phép trừ thì không đối xử với nhau như vậy. `12 − 5` bạn viết ra được
ngay: đứng ở vạch 12, lùi 5 bước, tới vạch 7.

Còn `5 − 12` thì sao? Đọc theo kiểu "còn cách bao xa" thì nó phải ra 7 — nhưng
đó đúng là chỗ cách đọc ấy vừa hết hiệu lực, vì 12 đã vượt qua 5 rồi. Còn đọc
theo câu chắc chân — *phải cộng thêm bao nhiêu vào 12 để tới 5* — thì câu hỏi
vẫn đứng vững, chỉ là chưa con số nào bạn biết trả lời nổi nó.

Cứ đi bộ thử xem. Cùng một luật của bài 14 — đứng ở vạch 5, lùi 12 bước. Đi
được 5 bước thì chân chạm mốc 0, đầu luống. Vẫn còn 7 bước nữa phải lùi.

**Bên trái số 0 có gì?** Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
