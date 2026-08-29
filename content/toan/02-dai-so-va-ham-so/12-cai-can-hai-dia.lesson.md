---
id: toan.dai-so-va-ham-so.cai-can-hai-dia
title: Cái cân hai đĩa
summary: Một phương trình đọc được như một cái cân đang thăng bằng — và trên cân thì thấy ngay tay nào làm lệch, tay nào không.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.balance-model]
requires: [math.solution-set, math.equation, math.expression, math.substitution, math.letter-names-a-slot, math.addition-as-union, math.number-line-subtract, core.boolean, ctrl.comparison, core.variable, core.assignment, core.print-variable, core.arithmetic, core.output]
concepts: [math.o-trong, math.phuong-trinh, math.can-hai-dia, math.xe-banh-mi]
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

::::byte{trigger=enter mood=curious pose=lean-in}
Xe mình có cái cân hai đĩa để đong bột. Nó đang bằng nhau — mình chưa dám chạm.
::::

::::explain{#vat-dang-dung}
Bài trước để lại một câu hỏi: có vật gì ngoài đời cũng đang ở thế **"đang
đúng"** — chạm bừa là hỏng, mà chạm khéo thì vẫn nguyên?

Trên xe bánh mì có đúng một vật như vậy. Byte đong bột bằng **cân hai đĩa**:
loại cân không có mặt số, chỉ có một cái đòn ngang và hai cái đĩa. Nó không nói
cho bạn biết vật nặng bao nhiêu. Nó chỉ nói **một** điều: hai đĩa có nặng bằng
nhau hay không.

Sáng nay Byte đặt lên đó:

- **Đĩa trái**: một gói bột chưa bóc nhãn — chưa ai biết nó nặng bao nhiêu — và
  hai quả cân 100 g.
- **Đĩa phải**: năm quả cân 100 g.

Cái đòn nằm ngang.

```text
           ĐĨA TRÁI                         ĐĨA PHẢI
  ┌───────────────────────┐        ┌───────────────────────┐
  │        gói bột        │        │  100 g  100 g  100 g  │
  │     100 g   100 g     │        │      100 g  100 g     │
  └───────────┬───────────┘        └───────────┬───────────┘
              └───────────────┬────────────────┘
                              ▲
                     cân đang THĂNG BẰNG
```

Bây giờ đọc lại cái cân ấy bằng đúng thứ bài 10 vừa dựng. Gọi cân nặng gói bột
là `n` gam.

Để ý chỗ này, vì hai thứ dễ lẫn vào nhau: **gói bột có đúng một cân nặng thật**
— nó nằm trên đĩa cân kia kìa, chỉ là Byte chưa biết con số. Còn `n` **trên
giấy** thì vẫn là một ô trống đúng nghĩa bài 1: một chỗ chờ điền, điền số nào
cũng viết ra được một câu. Cái cân không biến ô trống thành số bị giấu; nó chỉ
thêm vào một tin: trong tất cả những số điền được, có đúng một số làm đòn nằm
ngang.

| trên cân | trên giấy |
|---|---|
| đĩa trái: gói bột và hai quả 100 g | `n + 200` |
| đĩa phải: năm quả 100 g | `500` |
| cái đòn nằm ngang | dấu `=` |
| **cân đang thăng bằng** | **`n + 200 = 500`** |

Ba thứ khớp nhau, không thứ nào phải bẻ:

- **Ô trống vẫn là ô trống.** Gói bột chưa bóc nhãn thì chưa ai biết nó nặng bao
  nhiêu. Đúng như `n`: không phải một con số bị giấu đi cho khó, mà là một chỗ
  chưa điền.
- **Dấu `=` vẫn là một lời khẳng định, không phải một mệnh lệnh.** Cái đòn nằm
  ngang không ra lệnh cho ai làm gì; nó *nói* rằng hai bên nặng bằng nhau.
- **Và lời khẳng định ấy đang ĐÚNG.** Đây là chỗ cái cân cho thêm thứ mà tờ
  giấy không có: bạn **nhìn thấy** nó đang đúng. Đòn ngang, tức đúng. Đòn
  nghiêng, tức sai.
::::

::::example{#mot-tay-lam-lech}
Byte thò tay vào. Thao tác đầu tiên: **nhấc một quả 100 g khỏi đĩa trái**, đĩa
phải không đụng tới.

Đĩa trái nhẹ đi 100 g. Đĩa phải y nguyên. Cái đòn đổ về bên phải.

```text
           ĐĨA TRÁI
  ┌───────────────────────┐
  │        gói bột        │
  │        100 g          │                ĐĨA PHẢI
  └───────────┬───────────┘       ┌───────────────────────┐
              └──────┐            │  100 g  100 g  100 g  │
                     └────┐       │      100 g  100 g     │
                          ▲       └───────────┬───────────┘
                           └──────────────────┘

            LỆCH — đòn đổ xuống bên phải, nên phải nặng hơn
```

Trên giấy, thao tác ấy biến `n + 200 = 500` thành `n + 100 = 500`. Câu mới này
**sai** với gói bột đang nằm trên đĩa: nếu `n + 200` đúng bằng 500 thì `n + 100`
phải nhỏ hơn 500, không thể bằng.

Chỗ này nói ra được một điều mà tờ giấy giấu rất kỹ:

> Thăng bằng **không phải một tính chất của cái cân**. Nó là một **quan hệ giữa
> hai đĩa**. Không có đĩa nào "đúng" hay "sai" một mình; chỉ có hai đĩa hợp
> nhau hay không hợp nhau.

Nghĩ theo kiểu quan hệ thì cái tay vừa rồi hỏng ở chỗ nào cũng rõ luôn: nó động
vào **một** bên. Chênh lệch giữa hai đĩa đang là 0, tay ấy kéo nó thành −100, và
quan hệ mất.

Vậy có kiểu tay nào **không** làm mất quan hệ ấy không?
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Byte cân riêng gói bột rồi: đúng 300 gam. Giờ nhờ máy soi mấy cái tay.
::::

::::predict{#doan-ba-cai-tay commitOnce}
Byte đem gói bột ra cân đồng hồ: **đúng 300 gam**. Biết con số ấy thì không còn
gì để giải nữa — và đó chính là lý do gói bột này dùng được làm **vật thí
nghiệm**. Ta không đi tìm nó nữa; ta đem nó ra thử mấy cái tay.

Ba dòng dưới đây hỏi máy ba câu, mỗi câu về một cái tay khác nhau.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
bot = 300
trai = bot + 200
phai = 500

print(trai == phai)
print(trai - 100 == phai - 100)
print(trai - 100 == phai)
```

:::opt{correct}
`True`, `True`, rồi `False`
:::

:::opt
`True`, `True`, rồi `True`
::why
Gần đúng ở chỗ bạn xuất phát từ một sự thật chắc chắn: cái cân **đang** thăng
bằng, và hai dòng đầu đúng là `True`. Bạn đọc trúng cả xuất phát điểm lẫn thao
tác "bớt đều hai bên".

Ranh giới nằm ở chỗ thăng bằng là **quan hệ**, không phải tính chất. Một tính
chất thì bám vào vật và ở lại: cái cân này bằng đồng thì nhấc quả cân ra nó vẫn
bằng đồng. Còn quan hệ thì chỉ sống được chừng nào **cả hai** bên còn giữ đúng
vai. Dòng thứ ba bớt 100 ở đĩa trái mà không bớt gì ở đĩa phải: 400 với 500,
lệch đúng một quả cân.
::
:::

:::opt
`True`, `False`, rồi `False`
::why
Gần đúng, và cảnh giác của bạn là cảnh giác đúng: phần lớn cách thò tay vào một
cái cân đang thăng bằng đều làm nó lệch. Nếu phải đoán bừa về một cái tay bất
kỳ thì "lệch" là cửa đặt cược khôn hơn.

Ranh giới: có đúng một kiểu tay thoát được, và dòng thứ hai là nó — **làm y hệt
nhau ở cả hai đĩa**. Bớt 100 bên trái và bớt 100 bên phải thì mỗi đĩa nhẹ đi
đúng chừng ấy, nên **chênh lệch** giữa hai đĩa không đổi. Mà chênh lệch đang là
0, và 0 không đổi thì vẫn là 0. Cân vẫn ngang: 400 với 400.
::
:::

:::opt
`True`, rồi hai dòng sau máy báo lỗi, vì `trai` đã được đặt tên rồi thì không
trừ được nữa
::why
Gần đúng ở chỗ bạn nhớ đúng vai của dấu `=` một gạch trong Python: nó **đặt
tên**, và bài 10 đã tách vai ấy ra khỏi vai "khẳng định hai bên bằng nhau". Giữ
được hai vai riêng ra là phần khó.

Ranh giới: đặt tên xong thì cái tên **giữ một giá trị**, và một giá trị thì đem
tính tiếp thoải mái — `trai - 100` chỉ là lấy con số 500 mà `trai` đang giữ, trừ
đi 100, ra 400. Nó không sửa gì `trai` cả; sau dòng ấy `trai` vẫn giữ 500. Trên
cân thì hình ảnh tương ứng là: bạn không phá cái đĩa, bạn chỉ nhìn xem *nếu*
nhấc bớt 100 g ra thì đĩa còn nặng bao nhiêu.
::
:::
::::

::::explain{#hai-loai-tay}
Xếp ba cái tay vừa thử vào hai ngăn:

| cái tay | đĩa trái | đĩa phải | chênh lệch | cân ra sao |
|---|---|---|---|---|
| nhấc 100 g khỏi đĩa trái | 500 → 400 | 500 | 0 → −100 | **LỆCH** |
| nhấc 100 g khỏi **cả hai** đĩa | 500 → 400 | 500 → 400 | 0 → 0 | thăng bằng |
| đặt thêm 300 g lên **cả hai** đĩa | 500 → 800 | 500 → 800 | 0 → 0 | thăng bằng |

Cái phân biệt hai ngăn không phải là *nhấc ra* hay *đặt vào*, cũng không phải
*nặng* hay *nhẹ*. Nó là: **cùng một việc, làm ở cả hai đĩa, hay chỉ ở một đĩa.**

Nói lại bằng chênh lệch cho gọn: hai đĩa đang chênh nhau 0 g. Làm y hệt nhau ở
hai bên thì cả hai cùng dịch, chênh lệch không nhúc nhích. Làm ở một bên thì
chênh lệch đổi, và cái đòn ngã theo.

Đặt tên cho toàn bộ bức tranh này, vì từ đây nó còn đi xa:

> Một **phương trình** đọc được như một **cái cân hai đĩa đang thăng bằng**: vế
> trái là đĩa trái, vế phải là đĩa phải, dấu `=` là cái đòn nằm ngang. Ô trống
> là **gói hàng chưa biết nặng bao nhiêu** đang nằm trên một đĩa. Và cái cân
> cho bạn một thứ tờ giấy không có: bạn **nhìn thấy** thăng bằng còn hay mất
> mỗi lần thò tay vào.

Để ý cái cân hoàn toàn không giúp bạn *tìm* gói bột nặng bao nhiêu. Nó không
biết. Nó chỉ làm đúng một việc: **canh xem quan hệ giữa hai đĩa còn nguyên
không.** Mà đó lại đúng là thứ bài trước bảo là đáng sợ — động vào một câu đang
đúng thì phải giữ cho nó khỏi hoá sai.
::::

::::code{#soi-ba-cai-tay}
Đến lượt bạn đưa ba cái tay ra cho máy soi.

Gói bột nặng đúng 300 g, nên đĩa trái đang nặng `300 + 200 = 500` g và đĩa phải
đang nặng `500` g — cân ngang. Ba thao tác cần thử:

- **Tay A** — nhấc một quả 100 g khỏi **cả hai** đĩa.
- **Tay B** — nhấc một quả 100 g, **chỉ** khỏi đĩa trái. Đĩa phải để nguyên.
- **Tay C** — đặt thêm 300 g lên **cả hai** đĩa.

Điền sáu chỗ trống: sau mỗi cái tay, mỗi đĩa còn nặng bao nhiêu. Viết chúng từ
hai cái tên `trai` và `phai` có sẵn phía trên, cộng hoặc trừ đi phần vừa động
tới — đừng gõ thẳng con số cuối cùng, vì con số ấy chính là thứ bạn đang nhờ máy
tính hộ.

Ba dòng `print` ở cuối không in cân nặng. Chúng hỏi ba câu, mỗi câu về một cái
tay: **sau tay ấy, hai đĩa còn bằng nhau không?**

Bài chấm bằng cả ba cái tay, và chúng cố ý cho hai kết cục khác nhau: A và C
thăng bằng, B lệch. Một câu trả lời chép cứng không qua nổi cả ba.

```python title=starter
bot = 300
trai = bot + 200      # đĩa trái: gói bột và hai quả 100 g
phai = 500            # đĩa phải: năm quả 100 g

# Tay A — nhấc một quả 100 g khỏi CẢ HAI đĩa.
a_trai = ___
a_phai = ___

# Tay B — nhấc một quả 100 g, CHỈ khỏi đĩa trái.
b_trai = ___
b_phai = ___

# Tay C — đặt thêm 300 g lên CẢ HAI đĩa.
c_trai = ___
c_phai = ___

print(a_trai == a_phai)
print(b_trai == b_phai)
print(c_trai == c_phai)
```

```python title=solution
bot = 300
trai = bot + 200      # đĩa trái: gói bột và hai quả 100 g
phai = 500            # đĩa phải: năm quả 100 g

# Tay A — nhấc một quả 100 g khỏi CẢ HAI đĩa.
a_trai = trai - 100
a_phai = phai - 100

# Tay B — nhấc một quả 100 g, CHỈ khỏi đĩa trái.
b_trai = trai - 100
b_phai = phai

# Tay C — đặt thêm 300 g lên CẢ HAI đĩa.
c_trai = trai + 300
c_phai = phai + 300

print(a_trai == a_phai)
print(b_trai == b_phai)
print(c_trai == c_phai)
```

```python title=test
# Câu `!=` đứng TRƯỚC hai câu `==`. Nó canh cái bẫy chính của bài: nếu sáu chỗ
# trống được điền bừa theo một khuôn, tay A và tay B sẽ cho cùng một lời phán —
# và bẫy phải sập ngay tại đây, trước khi mấy câu `==` kịp che nó đi.
assert (a_trai == a_phai) != (b_trai == b_phai), "làm ở cả hai đĩa và làm ở một đĩa không thể cùng một kết cục — nếu tay A và tay B cho chung một lời phán thì có một tay bị viết sai"
assert b_trai != b_phai, "tay B chỉ nhấc bớt ở đĩa trái, nên đĩa trái nhẹ đi mà đĩa phải y nguyên — hai đĩa hết bằng nhau"
# `b_trai` phải là 400: nhấc ĐÚNG MỘT quả 100 g khỏi đĩa trái.
#
# Không có dòng này thì đọc nhầm đề — nhấc 200 g — vẫn qua trọn bộ, vì
# `b_trai != b_phai` đúng với cả 300 lẫn 400, và không câu nào khác đụng
# tới `b_trai`. Cái tay B là chỗ bài này đi tìm, mà nó lại là chỗ duy nhất
# không được chốt bằng một con số.
assert b_trai == 400, "tay B nhấc đúng MỘT quả 100 g khỏi đĩa trái: 500 g bớt 100 g còn 400 g"
assert a_trai == a_phai, "tay A nhấc đúng 100 g ở mỗi đĩa, chênh lệch không đổi, cân vẫn ngang"
assert a_trai == 400, "500 g bớt 100 g còn 400 g, ở cả hai đĩa"
assert b_phai == 500, "tay B không đụng vào đĩa phải, nên nó vẫn nặng đúng 500 g"
assert c_trai == c_phai, "tay C đặt thêm đúng 300 g lên mỗi đĩa, chênh lệch không đổi, cân vẫn ngang"
assert c_trai == 800, "500 g thêm 300 g thành 800 g, ở cả hai đĩa"
```

:::hints
- kind: attention
  body: Nhìn hai cái tên `trai` và `phai` ở đầu chương trình — chúng đang giữ cân nặng của hai đĩa lúc chưa ai chạm vào. Mỗi chỗ trống hỏi cùng một câu: sau cái tay này, đĩa ấy còn nặng bao nhiêu. Và đọc kỹ chữ in hoa trong mỗi lời chú thích: CẢ HAI hay CHỈ.
- kind: strategy
  body: Nhấc bớt thì trừ, đặt thêm thì cộng. Cái tay nào nói CẢ HAI thì hai dòng dưới nó cùng đổi một lượng như nhau; cái tay nào nói CHỈ thì đúng một dòng đổi, dòng còn lại giữ nguyên đĩa cũ — mà "giữ nguyên" cũng phải viết ra, bằng chính cái tên `phai`. Đừng gõ 400 hay 800; hãy để máy tính ra chúng từ `trai` và `phai`.
- kind: one-line
  body: "Tay A là `trai - 100` và `phai - 100`; tay B là `trai - 100` và `phai`; tay C là `trai + 300` và `phai + 300`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: sáu chỗ trống phải dựng từ hai cái tên `trai` và `phai`, cộng hoặc trừ phần vừa động tới — gõ thẳng 400 hay 800 thì bạn đã cân hộ máy, và cái bài này đang soi là mấy cái tay chứ không phải mấy con số
  requireAst:
  # Sáu chỗ trống đọc `trai` ba lần và `phai` ba lần. Khung khởi đầu không ĐỌC
  # hai tên này lần nào (dòng gán không tính là đọc), nên luật phân biệt được.
  - kind: uses-name, target: trai, min: 3
  - kind: uses-name, target: phai, min: 3
  # Ba phép trừ: hai của tay A, một của tay B. Khung khởi đầu không có dấu trừ.
  - kind: uses-operator, target: -, min: 3
  # Ba phép cộng: hai của tay C, cộng dòng `trai = bot + 200` đã có sẵn.
  - kind: uses-operator, target: +, min: 3
  forbidAst:
  # Lưới thứ hai, chặn đúng hai con số là KẾT QUẢ của việc cân. Lời giải thật
  # không chứa nguyên văn con số nào trong hai con số này.
  - kind: has-literal, target: 400
  - kind: has-literal, target: 800
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^True\nFalse\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai tay giữ được cân, một tay làm đổ. Khác nhau ở chỗ làm cả hai đĩa hay một.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Trên cân đồng thì dễ: mắt nhìn cái đòn là biết ngay còn thăng bằng hay không.
Byte có một trọng tài bằng sắt, không cãi được.

Nhưng thứ Byte thật sự phải xoay xở lại không phải cái cân — mà là **dòng chữ
trên giấy**:

```text
n + 200 = 500
```

Ở đây không có đòn nào để nghiêng, không có tiếng lạch cạch nào báo hỏng. Bạn
bớt 200 ở cả hai bên và viết xuống dòng mới:

```text
n = 300
```

Trông thì gọn hẳn — ô trống đã đứng một mình. Nhưng đó là một câu **khác**, do
tay bạn viết ra. Và bài 11 vừa dặn một điều đáng sợ: mỗi câu có cái giỏ nghiệm
của riêng nó.

- Cái giỏ của `n + 200 = 500` đựng những số nào?
- Cái giỏ của `n = 300` đựng những số nào?
- **Hai cái giỏ ấy có đúng là một không** — hay bạn vừa đổi mất câu hỏi mà không
  hay?

Trên cân, thứ phải giữ gìn là **thăng bằng**, và mắt canh được. Trên giấy, thứ
phải giữ gìn là **cái giỏ nghiệm** — mà giỏ thì không nhìn thấy. Vậy làm sao
biết một cái tay là an toàn?

Bài sau trả lời cho hai cái tay bạn vừa thấy: bớt đi, và thêm vào.
::::

::::checkpoint{mastery=0.8}
::::
