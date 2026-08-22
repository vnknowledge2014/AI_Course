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
estimatedMinutes: 15
teaches: [math.carry]
requires: [math.like-units, math.dong-goi, core.arithmetic, core.variable, core.print-variable]
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
812
::why
Gần đúng ở chỗ bạn làm đúng nửa việc khó nhất — nhận ra mười hạt lẻ ấy phải
thành một cái bó, và bó thì thuộc cột bên trái. Nên cột bó đúng là `4 + 3 + 1 =
8`, không sai một chút nào.

Chỗ lệch nằm ở chỗ đóng bó là một cuộc **dời chỗ**, không phải một bản sao.
Mười hạt đã đi sang cột bó rồi thì chúng không còn ở cột hạt lẻ nữa, nên cột
hạt lẻ phải là `12 − 10 = 2` chứ không còn là `12`. Đọc `812` ra thì thành tám
trăm mười hai hạt — mười hạt vừa bị đếm hai lần, một lần dưới dạng bó và một
lần dưới dạng hạt.
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

Lần này bốn đống **không** được viết sẵn thành `47` hay `35`. Mỗi đống nằm
nguyên thành hai cột — số bó một tên, số hạt lẻ một tên — vì việc của bài này
nằm đúng ở chỗ ranh giới giữa hai cột ấy. Gõ `47 + 35` thì máy đóng bó hộ bạn
và bạn không chạm vào chỗ ấy một lần nào.

Bạn sẽ đi lại đúng cái bảng ở phần trên, từng dòng một: gộp cột hạt lẻ, tách
mười hạt thừa ra khỏi cột đó, cộng cái bó vừa đóng vào cột bên trái, rồi mới
đọc các cột thành một con số.

Hai chỗ đáng để ý, vì đó là chỗ bài này sống:

- **`- 10` ở cột hạt lẻ.** Mười hạt ấy đã sang cột bó rồi thì chúng không còn
  ở cột hạt lẻ nữa. Không trừ nghĩa là đếm chúng hai lần.
- **`+ 1` ở cột bó.** Cái `1` của "nhớ 1" không phải thứ máy tự làm hộ — nó là
  một dấu cộng bạn gõ ra, ở đúng cột bạn chọn.

Hai buổi được chọn cố ý. Cột hạt lẻ của cả hai đều là `7 + 5`, nên buổi nào
cũng phải đóng một bó. Nhưng buổi chiều gộp cột bó ra `12`, nên nó còn phải
đóng bó **lần thứ hai** — mười cái bó đổi lấy một bó-của-bó — và con số cuối
dài tới ba chữ số.

```python title=starter
# Sáng: Byte 4 bó 7 hạt lẻ, An mang tới 3 bó 5 hạt lẻ.
sang_byte_bo = 4
sang_byte_hat = 7
sang_an_bo = 3
sang_an_hat = 5

# Cột hạt lẻ trước: gộp hai cột hạt lẻ đã.
sang_hat_gop = ___
# Mười hạt trong số đó vừa đóng thành một bó và rời khỏi cột này.
sang_hat_con = ___
# Cột bó: gộp hai cột bó, rồi cộng thêm đúng cái bó vừa đóng.
sang_bo_gop = ___
# Đọc hai cột thành một con số — mỗi bó là mười hạt.
tong_sang = ___

# Chiều: Byte 5 bó 7 hạt lẻ, An mang tới 6 bó 5 hạt lẻ.
chieu_byte_bo = 5
chieu_byte_hat = 7
chieu_an_bo = 6
chieu_an_hat = 5

chieu_hat_gop = ___
chieu_hat_con = ___
chieu_bo_gop = ___

# Cột bó lần này cũng tràn: mười cái bó đóng thành MỘT bó-của-bó và sang đứng
# ở cột thứ ba. Đây là cái `1` được nhớ lần thứ hai, viết sẵn cho bạn.
chieu_bo_cua_bo = 1
chieu_bo_con = ___
# Ba cột — mỗi bó-của-bó là một trăm hạt, mỗi bó là mười hạt.
tong_chieu = ___

print(tong_sang)
print(tong_chieu)
```

```python title=solution
# Sáng: Byte 4 bó 7 hạt lẻ, An mang tới 3 bó 5 hạt lẻ.
sang_byte_bo = 4
sang_byte_hat = 7
sang_an_bo = 3
sang_an_hat = 5

# Cột hạt lẻ trước: gộp hai cột hạt lẻ đã.
sang_hat_gop = sang_byte_hat + sang_an_hat
# Mười hạt trong số đó vừa đóng thành một bó và rời khỏi cột này.
sang_hat_con = sang_hat_gop - 10
# Cột bó: gộp hai cột bó, rồi cộng thêm đúng cái bó vừa đóng.
sang_bo_gop = sang_byte_bo + sang_an_bo + 1
# Đọc hai cột thành một con số — mỗi bó là mười hạt.
tong_sang = sang_bo_gop * 10 + sang_hat_con

# Chiều: Byte 5 bó 7 hạt lẻ, An mang tới 6 bó 5 hạt lẻ.
chieu_byte_bo = 5
chieu_byte_hat = 7
chieu_an_bo = 6
chieu_an_hat = 5

chieu_hat_gop = chieu_byte_hat + chieu_an_hat
chieu_hat_con = chieu_hat_gop - 10
chieu_bo_gop = chieu_byte_bo + chieu_an_bo + 1

# Cột bó lần này cũng tràn: mười cái bó đóng thành MỘT bó-của-bó và sang đứng
# ở cột thứ ba. Đây là cái `1` được nhớ lần thứ hai, viết sẵn cho bạn.
chieu_bo_cua_bo = 1
chieu_bo_con = chieu_bo_gop - 10
# Ba cột — mỗi bó-của-bó là một trăm hạt, mỗi bó là mười hạt.
tong_chieu = chieu_bo_cua_bo * 100 + chieu_bo_con * 10 + chieu_hat_con

print(tong_sang)
print(tong_chieu)
```

```python title=test
# Buổi sáng, đi theo đúng thứ tự của bảng: gộp cột hạt lẻ, dời mười hạt sang
# trái, rồi mới tới cột bó.
assert sang_hat_gop == 12, "7 hạt lẻ gộp với 5 hạt lẻ thì được 12 hạt lẻ"
assert sang_hat_con == 2, "mười hạt trong số đó vừa thành một bó và RỜI cột hạt lẻ, nên cột ấy còn 2"
assert sang_bo_gop == 8, "4 bó gộp 3 bó rồi cộng cái bó vừa đóng: 4 + 3 + 1 = 8"
assert tong_sang == 82, "8 bó và 2 hạt lẻ, viết theo bảng vị trí là 82"

# Buổi chiều: cùng một luật, nhưng phải chạy thêm một lần nữa ở cột bó.
assert chieu_hat_gop == 12, "cột hạt lẻ buổi chiều cũng là 7 + 5"
assert chieu_hat_con == 2, "lại mười hạt đi sang cột bó, cột hạt lẻ còn 2"
assert chieu_bo_gop == 12, "5 bó gộp 6 bó rồi cộng cái bó vừa đóng: 5 + 6 + 1 = 12"
assert chieu_bo_con == 2, "mười cái bó ấy vừa thành một bó-của-bó và RỜI cột bó, nên cột ấy còn 2"
assert tong_chieu == 122, "1 bó-của-bó, 2 bó và 2 hạt lẻ, viết ra là 122"

# Bảo toàn. Đóng bó chỉ đổi hình dạng, không sinh ra và không làm mất hạt nào,
# nên đếm thẳng cả đống về "một hạt" — không đi qua cột nào, đúng cách 2 của
# bài 11 — phải rơi trúng con số bạn vừa dựng lên từ các cột.
kiem_sang = sang_byte_bo * 10 + sang_byte_hat + sang_an_bo * 10 + sang_an_hat
kiem_chieu = chieu_byte_bo * 10 + chieu_byte_hat + chieu_an_bo * 10 + chieu_an_hat
assert tong_sang == kiem_sang, "gộp theo cột rồi đóng bó phải ra đúng chừng ấy hạt như khi đếm thẳng"
assert tong_chieu == kiem_chieu, "buổi chiều cũng vậy, dù phải đóng bó tới hai lần"
```

:::hints
- kind: attention
  body: Nhìn lại cái bảng ở phần trên — bảng ấy có bao nhiêu dòng thì ở đây có bấy nhiêu chỗ trống, và đúng thứ tự đó. Đọc kỹ đuôi mỗi tên để biết nó đếm bó hay đếm hạt lẻ; cột nào chỉ được gộp với cột ấy (bài 11).
- kind: strategy
  body: Làm cột hạt lẻ trước rồi mới sang cột bó, vì cột bó phải chờ xem cột bên phải có đóng được bó nào không. Mười hạt đi sang cột bó thì cột hạt lẻ mất đúng mười — trừ đi. Cột bó thì được thêm đúng một — cộng vào. Dòng cuối mỗi buổi chỉ là đọc các cột thành một con số: bó nhân mười, bó-của-bó nhân một trăm, rồi cộng hết lại.
- kind: one-line
  body: "Buổi sáng: `sang_hat_gop = sang_byte_hat + sang_an_hat`, `sang_hat_con = sang_hat_gop - 10`, `sang_bo_gop = sang_byte_bo + sang_an_bo + 1`, `tong_sang = sang_bo_gop * 10 + sang_hat_con`. Buổi chiều ba dòng đầu y hệt với `chieu_…`, rồi `chieu_bo_con = chieu_bo_gop - 10` và `tong_chieu = chieu_bo_cua_bo * 100 + chieu_bo_con * 10 + chieu_hat_con`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải được DỰNG từ những cái tên đứng phía trên nó — chép sẵn con số kết quả thì chỗ đóng bó, tức toàn bộ bài này, không còn được kiểm gì
  requireAst:
  # Ba dấu `-`: cột hạt lẻ của hai buổi, cộng thêm cột bó của buổi chiều. Đây
  # là luật đắt nhất ở đây — nó chặn đúng lời giải bỏ qua bước dời mười hạt ra
  # khỏi cột và điền thẳng `2` vào.
  - kind: uses-operator, target: -, min: 3
  # Gộp hai cột, cộng cái bó vừa đóng, rồi cộng các cột lại thành một con số.
  - kind: uses-operator, target: +, min: 6
  # Đọc cột thành con số: bó nhân mười, bó-của-bó nhân một trăm.
  - kind: uses-operator, target: *, min: 2
  # Cả tám cái tên đầu vào phải có mặt. Thiếu luật này thì một nửa mỗi phép gộp
  # có thể chép cứng mà vẫn qua — `sang_hat_gop = sang_byte_hat + 5` ra đúng 12.
  - kind: uses-name, target: sang_byte_bo, min: 1
  - kind: uses-name, target: sang_byte_hat, min: 1
  - kind: uses-name, target: sang_an_bo, min: 1
  - kind: uses-name, target: sang_an_hat, min: 1
  - kind: uses-name, target: chieu_byte_bo, min: 1
  - kind: uses-name, target: chieu_byte_hat, min: 1
  - kind: uses-name, target: chieu_an_bo, min: 1
  - kind: uses-name, target: chieu_an_hat, min: 1
  # Con số cuối phải được dựng TỪ CÁC CỘT, không phải tính vòng qua đường khác.
  - kind: uses-name, target: sang_hat_con, min: 1
  - kind: uses-name, target: sang_bo_gop, min: 1
  - kind: uses-name, target: chieu_hat_con, min: 1
  - kind: uses-name, target: chieu_bo_con, min: 1
  - kind: uses-name, target: chieu_bo_cua_bo, min: 1
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
