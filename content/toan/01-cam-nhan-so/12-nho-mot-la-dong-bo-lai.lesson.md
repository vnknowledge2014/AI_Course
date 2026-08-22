---
id: toan.cam-nhan-so.nho-mot-la-dong-bo-lai
title: Nhớ 1 chính là đóng bó
summary: Cái “1” bạn nhớ khi cộng không phải mẹo tính nhẩm — nó là một cái bó vừa được đóng từ mười hạt lẻ, và bó thì phải sang đứng ở cột bên trái.
locale: vi
track: toan
module: cam-nhan-so
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.carry]
requires: [math.like-units, math.dong-goi, core.arithmetic, core.variable, core.print-variable, core.string-concat, logic.and]
concepts: [math.dong-goi, math.thang-cot, math.gia-tri-vi-tri]
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
Mười hạt lẻ không mất đi đâu cả. Chúng chỉ đổi hình dạng.
::::

::::explain{#hat-thu-muoi-di-dau}
Bài trước để lại đúng một câu hỏi: cột hạt lẻ gộp ra 12, mà cột thì chỉ chứa
nổi tới 9. Cái hạt thứ mười đi đâu?

Nó không đi đâu cả. Nó vẫn nằm trên bàn.

Thứ hết chỗ không phải cái bàn — thứ hết chỗ là **tờ giấy**. Bảng vị trí ở bài
7 cho mỗi cột đúng một ô, và một ô chỉ viết được một chữ số từ `0` tới `9`. Vậy
nên vấn đề không phải "làm gì với hạt", mà là "viết chúng xuống kiểu gì".

Và câu trả lời thì bạn đã có từ bài 6 rồi: **đủ mười thì gom thành một bó.**

```text
12 hạt lẻ:   o o o o o o o o o o   o o
             \_________________/
                mười hạt này
                túm lại thành

12 hạt lẻ  =  [==========]  +  o o
                một bó         hai hạt lẻ
```

Không hạt nào sinh ra, không hạt nào mất đi. Vẫn đúng mười hai hạt trên bàn.
Chỉ có mười hạt trong số đó vừa **đổi hình dạng**: từ mười vật rời thành một
vật gộp.

Nhưng một cái bó thì không được phép nằm ở cột hạt lẻ — cột ấy đo bằng thước
"một hạt". Bó đo bằng thước "một bó", nên theo luật cùng đơn vị của bài trước,
nó phải sang đứng ở **cột bên trái**, chỗ dành cho bó.

Đó là toàn bộ câu chuyện. Và nó có một cái tên mà rất nhiều người thuộc lòng từ
lớp Một mà không ai giải thích: **"nhớ 1"**.

> Cái `1` bạn "nhớ" không phải một con số bí ẩn. Nó là **một cái bó vừa mới
> được đóng**, và nó đi sang trái vì bó thuộc về cột bên trái.
::::

::::example{#mot-cot-mot-cho}
Cho máy làm đúng phép gộp của cột hạt lẻ, không hơn:

```python title=readonly
byte_hat = 7
an_hat = 5

print(byte_hat + an_hat)
```

Máy in ra:

```text
12
```

Con số này đúng — 7 hạt lẻ gộp 5 hạt lẻ thì đúng là 12 hạt lẻ. Nhưng để ý nó
có **hai** chữ số, trong khi cột hạt lẻ trên giấy chỉ có **một** ô.

Đây mới là chỗ chặt: máy trả về một con số, còn bảng vị trí đòi một chữ số. Hai
thứ ấy không khớp nhau, và chỗ không khớp ấy chính là chỗ "nhớ 1" ra đời.
::::

::::predict{#doan-ca-hai-cot commitOnce}
Giờ gộp cả hai đống, không tách cột nữa. Byte có 4 bó 7 hạt lẻ, viết theo bảng
vị trí là `47`. An có 3 bó 5 hạt lẻ, viết là `35`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
byte_co = 47
an_mang = 35
print(byte_co + an_mang)
```

:::opt{correct}
82
:::

:::opt
712
::why
Gần đúng ở chỗ bạn áp **đúng** luật cùng đơn vị của bài 11, và áp không sai một
phép nào: cột hạt lẻ gộp với cột hạt lẻ được `7 + 5 = 12`, cột bó gộp với cột
bó được `4 + 3 = 7`. Hai con số ấy đều chính xác, và chúng đúng là hai kết quả
của bài toán này.

Chỗ lệch là ở bước cuối, lúc viết xuống. Một cột chỉ có một ô (bài 7), nên
không thể nhét cả `12` vào cột hạt lẻ rồi coi như xong — đọc lại "712" thì nó
thành bảy trăm mười hai, tức bảy bó-của-bó, một bó và hai hạt lẻ. Số hạt đã tự
nhân lên gấp mấy lần mà không ai bỏ thêm hạt nào vào bàn. Mười hạt thừa ra phải
đóng thành bó và sang cột bên trái trước khi viết.
::
:::

:::opt
72
::why
Gần đúng ở chỗ bạn nhận ra được điều mà nhiều người bỏ qua: `12` không nằm lại
trong một ô được, nên chỉ chữ số `2` được ở lại cột hạt lẻ. Nhận xét ấy đúng
hoàn toàn, và nó là nửa đầu của bài này.

Chỗ lệch nằm ở nửa sau. Mười hạt còn lại không bị **bỏ đi** — chúng vẫn trên
bàn, chỉ vừa đổi hình dạng thành một cái bó. Cái bó ấy đòi chỗ ở cột bên trái,
nên cột bó phải là `4 + 3 + 1`, chứ không phải `4 + 3`. Cắt bớt cho vừa ô là
làm mất hạt thật, còn đóng bó thì không mất gì.
::
:::

:::opt
4735
::why
Gần đúng ở chỗ bạn nhớ đúng một việc mà dấu `+` làm được thật: đặt hai mảnh
cạnh nhau thành một mảnh dài hơn. Realm 0 dùng nó như vậy để nối hai câu chữ,
và ở đó nghĩ thế là chính xác.

Chỗ lệch nằm ở phạm vi. `+` chỉ ghép khi cả hai bên là **chữ** — có dấu nháy
bao quanh. Ở đây `47` và `35` viết trần nên chúng là số, và với hai con số thì
`+` gộp lượng chứ không xếp chữ cạnh nhau.
::
:::
::::

::::explain{#cung-mot-luat-lap-lai}
Viết cả phép gộp ra thành bảng, từng cột một:

```text
        bó   hạt lẻ
Byte     4      7
An       3      5
       ----   ------
gộp      7     12      ← luật bài 11: từng cột một
```

`12` không nằm được trong một ô. Đóng bó:

```text
        bó   hạt lẻ
gộp      7     12
đóng    +1    −mười    ← mười hạt lẻ đổi lấy đúng một bó
       ----   ------
         8      2      →  viết ra là 82
```

Kiểm lại cho chắc: `82` đọc theo bảng vị trí là 8 bó và 2 hạt lẻ, tức là tám
chục hai hạt. Byte có 47, An có 35 — cộng lại đúng 82 hạt. Không mất hạt nào.

**Và luật này lặp lại.** Bài 6 nói đủ mười hạt thì thành một bó; đủ mười bó thì
lại thành một bó-của-bó. Chuyện vừa xảy ra ở cột hạt lẻ có thể xảy ra y hệt ở
cột bó:

```text
        bó-của-bó   bó   hạt lẻ
Byte                 5      7      (57)
An                   6      5      (65)
                   ----   ------
gộp                 11     12
đóng ở cột hạt lẻ   +1     −mười
                   ----   ------
                    12      2
đóng ở cột bó   +1  −mười
              ----  ----   ------
                1     2      2     →  viết ra là 122
```

Hai lần đóng bó, cùng một luật, chỉ khác cột. Không có luật thứ hai nào phải
học thêm — thứ bạn có ở bài 6 đã đủ dùng cho mọi cột, mãi mãi về bên trái.
::::

::::code{#hai-buoi-hai-lan-dong-bo}
Byte ghi sổ hai buổi. Mỗi buổi hai người dồn hạt vào chung một sọt.

- **Sáng**: Byte có 4 bó 7 hạt lẻ, An mang tới 3 bó 5 hạt lẻ.
- **Chiều**: Byte có 5 bó 7 hạt lẻ, An mang tới 6 bó 5 hạt lẻ.

Mỗi bó đúng mười hạt, nên bốn đống đã được viết sẵn thành bốn con số theo bảng
vị trí (bài 7): 4 bó 7 hạt lẻ viết là `47`, và cứ thế. Việc của bạn là gộp hai
đống của mỗi buổi.

Hai buổi được chọn cố ý. Cột hạt lẻ của cả hai đều là `7 + 5`, nên buổi nào
cũng phải đóng một bó. Nhưng buổi chiều còn phải đóng bó **lần thứ hai** ở cột
bó, nên hai buổi ra hai con số khác hẳn nhau — buổi chiều dài tới ba chữ số.
Chép cứng một kết quả vào cả hai chỗ trống thì hỏng ít nhất một buổi.

```python title=starter
# Sáng: Byte 4 bó 7 hạt lẻ, An mang tới 3 bó 5 hạt lẻ.
sang_byte = 47
sang_an = 35

# Chiều: Byte 5 bó 7 hạt lẻ, An mang tới 6 bó 5 hạt lẻ.
chieu_byte = 57
chieu_an = 65

tong_sang = ___
tong_chieu = ___

print(tong_sang)
print(tong_chieu)
```

```python title=solution
# Sáng: Byte 4 bó 7 hạt lẻ, An mang tới 3 bó 5 hạt lẻ.
sang_byte = 47
sang_an = 35

# Chiều: Byte 5 bó 7 hạt lẻ, An mang tới 6 bó 5 hạt lẻ.
chieu_byte = 57
chieu_an = 65

tong_sang = sang_byte + sang_an
tong_chieu = chieu_byte + chieu_an

print(tong_sang)
print(tong_chieu)
```

```python title=test
assert tong_sang == 82, "47 hạt gộp 35 hạt: đóng một bó, còn 2 hạt lẻ"
assert tong_chieu == 122, "57 hạt gộp 65 hạt: đóng bó hai lần, còn 2 hạt lẻ"

# Đọc ngược hai kết quả ra thành cột (bài 7). Đóng bó không sinh ra và không
# làm mất hạt nào, nên tổng phải tách lại được thành đúng chừng ấy bó và hạt.
assert tong_sang == 80 + 2, "82 là 8 bó và 2 hạt lẻ"
assert tong_chieu == 100 + 20 + 2, "122 là 1 bó-của-bó, 2 bó và 2 hạt lẻ"

# Cột hạt lẻ của hai buổi giống hệt nhau (7 + 5), nên sau khi đóng bó cả hai
# buổi đều phải còn đúng 2 hạt lẻ — dù số bó thì khác xa nhau.
assert 80 + 2 == tong_sang and 120 + 2 == tong_chieu, "cả hai buổi đều còn đúng 2 hạt lẻ"
```

:::hints
- kind: attention
  body: Bốn cái tên phía trên chia làm hai cặp, mỗi cặp một buổi. Đọc đầu mỗi tên để biết nó thuộc buổi nào — đừng gộp một đống buổi sáng với một đống buổi chiều.
- kind: strategy
  body: Mỗi chỗ trống là một phép gộp của bài 10 giữa hai đống trong cùng một buổi. Đừng tự đóng bó trong đầu rồi chép con số cuối vào — chỗ đóng bó là chỗ dễ trượt nhất, cứ để máy làm rồi đối chiếu với bảng cột bạn vừa đọc.
- kind: one-line
  body: "Thay `___` thứ nhất bằng `sang_byte + sang_an`, và `___` thứ hai bằng `chieu_byte + chieu_an`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải GỘP hai cái tên của cùng một buổi bằng dấu `+` — chép sẵn con số tổng thì chỗ đóng bó không còn được kiểm gì
  requireAst:
  # Hai buổi thì hai phép gộp. Khung khởi đầu chưa có dấu `+` nào, nên luật này
  # chặn đúng đáp án chép cứng 82 với 122.
  - kind: uses-operator, target: +, min: 2
  - kind: uses-name, target: sang_an, min: 1
  - kind: uses-name, target: chieu_an, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^82\n122\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
82 và 122. Cái bó mình nhớ được, nó đứng ngay cột bên trái.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại ba bài vừa rồi: cộng luôn là chuyện của **hai đống**. Đổ chung, gộp
từng cột, đóng bó cái nào thừa ra. Bức tranh nào cũng có hai nắm hạt và một cái
bàn.

Nhưng bài 5 đã cho bạn một bức tranh khác hẳn về số. Ở đó không có đống nào cả:
mọi số nằm trên một **thanh số**, mỗi số là một **chỗ**, hai vạch liền nhau
cách nhau đúng một đơn vị. Số 7 ở đó không phải bảy hạt trong tay — nó là một
điểm trên đường thẳng.

Trên bức tranh ấy thì "cộng 3" trông ra sao? Một cái chỗ thì đổ chung với cái
gì được? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
