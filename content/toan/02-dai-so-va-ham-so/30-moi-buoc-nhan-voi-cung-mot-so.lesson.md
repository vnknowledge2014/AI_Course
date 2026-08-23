---
id: toan.dai-so-va-ham-so.moi-buoc-nhan-voi-cung-mot-so
title: Mỗi bước nhân với cùng một số
summary: Hũ men nhân đôi mỗi giờ — đào bao nhiêu cột chênh lệch cũng không tới đáy, vì bước đi cố định ở đây là một phép nhân chứ không phải phép cộng.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 30
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.exponential-function]
practices: [math.exponent, math.quadratic-function, math.slope, math.value-table, math.division-quotative]
requires: [math.quadratic-function, math.linear-function, math.slope, math.value-table, math.function-notation, math.exponent, math.multiplication, math.multiply-as-scaling, math.division-quotative, math.compare-on-number-line, core.function-def, core.function-call, core.function-parameter, core.function-argument, core.function-return, core.variable, core.assignment, core.arithmetic, core.print-variable, core.number-literal, core.output]
concepts: [math.chenh-lech-cua-chenh-lech, math.xe-banh-mi]
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
Hũ men cứ mỗi giờ lại nhân đôi. Mấy cái cột của mình đo nổi nó không?
::::

::::explain{#hu-men}
Muốn bánh mì nở thì phải có men. Byte nuôi một hũ **bột cái** ở góc xe: một hũ
bột chua, trong đó có men sống. Men ăn bột, sinh thêm men, và người bán bánh
lâu năm đều biết một điều: **cứ khoảng một giờ thì lượng men trong hũ nhân
đôi.**

Đầu giờ sáng Byte múc bớt ra, để lại đúng **1 phần** men trong hũ. Rồi cứ để
đấy.

Ô trống ở đây là **số giờ đã trôi qua**. Gọi nó là `n`, và gọi cái máy là `m`:
sau `n` giờ, trong hũ có `m(n)` phần men?

Điền `n` từ 0 tới 7 rồi ghi lại:

```text
  n   │ m(n)
──────┼──────
    0 │    1
    1 │    2
    2 │    4
    3 │    8
    4 │   16
    5 │   32
    6 │   64
    7 │  128
```

Giờ thứ 0 là lúc Byte vừa múc xong, men chưa kịp nhân đôi lần nào, nên hũ còn
đúng 1 phần.
::::

::::example{#dao-hoai-khong-toi-day}
Bạn có sẵn hai cái cột để soi một cái bảng. Đem cả hai ra dùng.

**Cột thứ ba — bài 26:** kết quả đổi bao nhiêu so với dòng ngay trên?

**Cột thứ tư — bài 28:** cột thứ ba đổi bao nhiêu so với dòng ngay trên?

```text
  n   │ m(n) │ cột 3: bước từ dòng trên │ cột 4: bước của cột 3
──────┼──────┼──────────────────────────┼──────────────────────
    0 │    1 │                          │
    1 │    2 │                       +1 │
    2 │    4 │                       +2 │                    +1
    3 │    8 │                       +4 │                    +2
    4 │   16 │                       +8 │                    +4
    5 │   32 │                      +16 │                    +8
    6 │   64 │                      +32 │                   +16
    7 │  128 │                      +64 │                   +32
```

Cột thứ ba không đứng im — vậy không phải bậc nhất, không có độ dốc nào để ghi.

Cột thứ tư cũng không đứng im — vậy cũng không phải bậc hai.

Và đây là chỗ đáng chú ý nhất: đọc riêng cột thứ tư — `1, 2, 4, 8, 16, 32` —
nó **y hệt** cột thứ hai, chỉ tụt xuống mấy dòng. Nên nếu bạn đào thêm cột thứ
năm, rồi cột thứ sáu, thì lần nào cũng ra đúng cái hàng ấy. Cái bảng này
**không chịu dẹt xuống**. Đào bao nhiêu tầng cũng không tới đáy.

Chuyện đó không phải vì bảng lộn xộn. Nó lộn xộn với đúng **một loại cột**. Đọc
lại đề bài một lần nữa: *mỗi giờ lượng men **nhân đôi***. Từ khoá không phải
"thêm", mà là "nhân". Vậy dựng một cột khác hẳn: mỗi dòng **gấp** dòng trên mấy
lần? Đó là phép đo của bài 27 track trước — xem số sau chứa số trước mấy lần.

```text
  n   │ m(n) │ cột mới: gấp mấy lần dòng trên
──────┼──────┼──────────────────────────────
    0 │    1 │
    1 │    2 │                            x2
    2 │    4 │                            x2
    3 │    8 │                            x2
    4 │   16 │                            x2
    5 │   32 │                            x2
    6 │   64 │                            x2
    7 │  128 │                            x2
```

Đứng im ngay từ dòng đầu tiên.

Cái bảng này vẫn có một bước đi cố định — cố định y như 15 000 của xe bánh mì.
Chỉ khác **loại** bước: một bên là cộng thêm một lượng, một bên là nhân với một
số.

Viết nó thành một cái máy. Sau `n` giờ, men đã nhân đôi `n` lần, tức nhân với 2
lặp lại `n` lần. Bài 24 của track trước đã cho ký hiệu viết gọn đúng việc đó:

> `m(n) = 2ⁿ`

Thử lại vài chỗ: `m(3) = 2 × 2 × 2 = 8` ✓. `m(0)` là nhân với 2 **không lần
nào**, tức để nguyên 1 phần ban đầu — đúng ô đầu bảng ✓.

Đặt tên cho cái vừa thấy, để mang đi được:

> Một máy mà mỗi bước là **nhân với cùng một số** — như `m(n) = 2ⁿ` — gọi là
> **hàm mũ**. Dấu hiệu nhận ra nó trên bảng: không cột chênh lệch nào chịu đứng
> im, dù đào bao nhiêu tầng; nhưng cột **gấp mấy lần** thì đứng im ngay tầng
> đầu.

Ba loại máy, cùng một câu hỏi "một bước đi làm gì":

```text
  bậc nhất  :  bước là CỘNG một lượng cố định     ( +15000 mỗi bước )
  bậc hai   :  bước là CỘNG một lượng đổi đều     ( +1, +3, +5, ... )
  hàm mũ    :  bước là NHÂN với một số cố định    ( x2 mỗi bước )
```
::::

::::predict{#doan-ba-cau commitOnce}
Byte gõ hai cái máy vào Python để đặt cạnh nhau: máy men, và máy mảnh sân của
bài 28. Trông chúng na ná nhau — cả hai đều là "nhân lặp lại" — nên đáng hỏi
máy xem chúng có thật sự giống nhau không.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def men(n):
    return 2 ** n

def san(n):
    return n * n

print(men(3))
print(san(3))
print(men(4) - men(3))
```

:::opt{correct}
`8`, rồi `9`, rồi `8`
:::

:::opt
`6`, rồi `9`, rồi `8`
::why
Chỗ đúng: bạn nhớ chính xác điều mà bài 24 track trước đã dạy — **dấu mũ là
cách viết gọn của phép nhân**, không phải một phép toán lạ nào cả. Đó là ý quan
trọng nhất về luỹ thừa, và bạn cầm chắc nó.

Ranh giới nằm ở chỗ **cái gì được lặp lại**. `2ⁿ` là nhân lặp lại chính **cơ
số** — ba lần con số 2: `2 × 2 × 2 = 8`. Còn `2 × 3` là cộng lặp lại: ba lần
con số 2 **cộng** vào nhau. Hai chuyện khác nhau, và chúng tình cờ trùng nhau ở
đúng một chỗ: `2¹ = 2` cũng bằng `2 × 1 = 2`. Ngoài chỗ ấy thì chúng rời nhau
rất nhanh — `2⁷ = 128`, còn `2 × 7 = 14`.
::
:::

:::opt
`8`, rồi `9`, rồi `2`
::why
Chỗ đúng, và nó là một chỗ đúng lớn: bạn nhận ra cái máy này có một **bước đi
cố định**, và bạn đọc trúng con số cố định đó — chính là số 2. Cả bài này xoay
quanh đúng nhận xét ấy.

Ranh giới là **số 2 ấy đóng vai gì**. Ở bài 26, con số cố định là lượng được
*cộng thêm*, nên nó cũng chính là ô của cột thứ ba — hỏi "thêm bao nhiêu" thì
nó trả lời. Ở đây số 2 là thứ được *nhân vào*, nên nó nằm ở cột **gấp mấy lần**
chứ không nằm ở cột thứ ba. Từ giờ thứ 3 sang giờ thứ 4, men đi từ 8 phần lên
16 phần: gấp 2 lần, nhưng **thêm** tận 8 phần. Dấu `-` trong dòng lệnh đang hỏi
câu thứ hai.
::
:::

:::opt
`8`, rồi `9`, rồi `4`
::why
Chỗ đúng: bạn không đoán mò mà đi đo — bạn dựng cột thứ ba rồi tìm cái đứng im
trong cột thứ tư, đúng động tác mà bài 28 đã dạy. Với mảnh sân thì động tác ấy
tìm ra ngay: cột thứ ba là `+1, +3, +5, +7` và cột thứ tư đứng im ở 2, nên đoán
được ô kế tiếp mà không cần tính lại.

Ranh giới là bảng này **không dẹt xuống**. Ở mảnh sân, cột thứ ba hơn nhau đúng
một lượng cố định, nên ô sau suy ra được từ hai ô trước: 1, 2, 3, 4… Ở đây cột
thứ ba là `+1, +2, +4, +8` — nó không cộng thêm 1 mỗi bước mà **nhân đôi** mỗi
bước, y hệt cột thứ hai. Nên bước từ giờ 3 sang giờ 4 không phải 4 mà là 8:
đúng bằng số men đang có, vì thêm bấy nhiêu nữa mới thành gấp đôi.
::
:::
::::

::::explain{#mu-khac-binh-phuong}
Đáng dừng lại ở chỗ `men(3) = 8` mà `san(3) = 9`, vì hai cách viết này rất dễ
lẫn: `2ⁿ` và `n × n` đều là nhân lặp lại, chỉ khác chỗ đặt `n`.

Đặt hai bảng cạnh nhau thì hết lẫn:

```text
  n   │ s(n) = n x n │ m(n) = 2 mũ n
──────┼──────────────┼──────────────
    0 │            0 │             1
    1 │            1 │             2
    2 │            4 │             4
    3 │            9 │             8
    4 │           16 │            16
    5 │           25 │            32
    6 │           36 │            64
    7 │           49 │           128
```

- Ở `n × n`, chữ `n` là **thứ được nhân**, còn số lần nhân thì cố định: luôn
  hai lần.
- Ở `2ⁿ`, chữ `n` là **số lần nhân**, còn thứ được nhân thì cố định: luôn số 2.

Hai cột cắt nhau đúng hai chỗ — `n = 2` cùng ra 4, `n = 4` cùng ra 16 — và ở
giữa hai chỗ ấy mảnh sân còn nhỉnh hơn (9 so với 8). Nhưng từ `n = 5` trở đi
thì chúng rời nhau, và rời rất nhanh: tới `n = 7` đã là 49 với 128.

Sự khác nhau ấy có nghĩa ngoài đời. Với mảnh sân, muốn diện tích gấp đôi thì
nới cạnh thêm một chút là được. Với hũ men, muốn gấp đôi thì phải **chờ thêm
đúng một giờ** — không có cách nào ngắn hơn. Còn chờ hai giờ thì không phải gấp
bốn cộng thêm gì cả: đúng gấp bốn.
::::

::::code{#hai-cach-do-mot-hu-men}
Cho máy đo hũ men bằng cả hai kiểu cột, để chính nó nói ra kiểu nào dùng được.

Bạn điền ba chỗ trống: cái máy men, thêm một ô của cột thứ ba, và thêm một lần
"nhân thử". Hai dòng `nhan_thu_*` kiểm đúng lời hứa của bài — **lấy dòng trước
nhân 2 thì phải ra đúng dòng sau**.

Bài chấm bằng cả bốn con số, và chúng không bằng nhau — gõ cứng `16` vào mọi
chỗ thì hỏng ở ba chỗ. Chỉ cái máy viết thật mới qua được cả bốn.

```python title=starter
def men(n):
    return ___                    # sau n giờ, một phần men thành mấy phần?

# Cột thứ ba: mỗi bước THÊM bao nhiêu?
them_3_4 = men(4) - men(3)
them_4_5 = ___

# Cột mới: lấy dòng trước NHÂN 2, xem có ra đúng dòng sau không.
nhan_thu_4 = men(3) * 2
nhan_thu_7 = ___

print(them_3_4)
print(them_4_5)
print(nhan_thu_4)
print(nhan_thu_7)
```

```python title=solution
def men(n):
    return 2 ** n                 # sau n giờ, một phần men thành mấy phần?

# Cột thứ ba: mỗi bước THÊM bao nhiêu?
them_3_4 = men(4) - men(3)
them_4_5 = men(5) - men(4)

# Cột mới: lấy dòng trước NHÂN 2, xem có ra đúng dòng sau không.
nhan_thu_4 = men(3) * 2
nhan_thu_7 = men(6) * 2

print(them_3_4)
print(them_4_5)
print(nhan_thu_4)
print(nhan_thu_7)
```

```python title=test
# Ba câu != đứng trước. Chúng canh đúng ba cách hiểu sai mà bài vừa nói tới;
# xếp sau các câu == thì chúng không bao giờ chạy tới, và cái bẫy không sập.
assert them_3_4 != them_4_5, "hai ô liền nhau của cột thứ ba phải KHÁC nhau — bằng nhau nghĩa là cột thứ ba đứng im, tức máy bạn viết là bậc nhất chứ không phải hàm mũ"
assert nhan_thu_4 != nhan_thu_7, "giờ thứ tư và giờ thứ bảy cách nhau xa, không thể ra cùng một số phần men"
assert men(3) != 3 * 3, "2 mũ 3 là 2 x 2 x 2 = 8, còn 3 x 3 = 9 — dấu mũ lặp lại CƠ SỐ, không lặp lại số mũ"
assert them_3_4 == 8, "từ giờ 3 (8 phần) sang giờ 4 (16 phần) thì thêm 8 phần"
assert them_4_5 == 16, "từ giờ 4 (16 phần) sang giờ 5 (32 phần) thì thêm 16 phần — bước sau to gấp đôi bước trước"
assert them_4_5 == men(4), "mỗi bước thêm đúng bằng lượng đang có, vì thêm bấy nhiêu nữa mới thành gấp đôi"
assert nhan_thu_4 == men(4), "lấy giờ thứ 3 nhân 2 phải ra đúng giờ thứ 4 — đó chính là lời hứa 'mỗi giờ nhân đôi'"
assert nhan_thu_7 == men(7), "lời hứa ấy đúng ở mọi dòng, không riêng dòng đầu: giờ thứ 6 nhân 2 ra đúng giờ thứ 7"
assert men(7) == 128, "sau bảy giờ, một phần men thành 128 phần"
assert men(0) == 1, "giờ thứ 0 là lúc chưa nhân đôi lần nào, nên hũ vẫn đúng một phần men"
```

:::hints
- kind: attention
  body: Chỗ trống đầu tiên nằm sau `return`, và hàm ấy chỉ nhận được đúng một cái tên là `n`. Đọc lại cột "gấp mấy lần" trong bảng — nó cho biết con số nào được nhân đi nhân lại, còn cột `n` cho biết nhân bao nhiêu lần. Hai chỗ trống còn lại thì nhìn dòng ngay phía trên chúng: khuôn đã viết sẵn, bạn chỉ đổi số giờ.
- kind: strategy
  body: Sau n giờ, men đã nhân đôi n lần, tức nhân với 2 lặp lại n lần — bài 24 của track trước có sẵn ký hiệu viết gọn cho đúng việc lặp phép nhân, và trong Python nó là dấu mũ. Cẩn thận chỗ đặt số 2 và chỗ đặt `n`: thứ được nhân là con số cố định, thứ đếm số lần là ô trống. Dòng `them_4_5` là bước từ giờ 4 sang giờ 5, còn `nhan_thu_7` phải lấy giờ ngay TRƯỚC giờ 7.
- kind: one-line
  body: "Ba chỗ trống lần lượt là `2 ** n`, `men(5) - men(4)` và `men(6) * 2`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hàm phải dùng dấu mũ với `n` làm số mũ, và hai dòng còn lại phải GỌI hàm ấy — gõ thẳng con số vào là bạn tra bảng hộ máy, cái luật "mỗi giờ nhân đôi" không được kiểm ở đâu cả
  requireAst:
  # Khung khởi đầu không có dấu mũ nào và không đọc `n` lần nào — chỗ trống
  # thứ nhất là chỗ duy nhất sinh ra cả hai.
  - kind: uses-operator, target: **, min: 1
  - kind: uses-name, target: n, min: 1
  # Khung khởi đầu gọi `men` 3 lần; lời giải gọi 6. Điền số vào hai chỗ trống
  # cuối thì con số này tụt xuống.
  - kind: uses-call, target: men, min: 6
  # Hai phép trừ (hai ô của cột thứ ba) và hai phép nhân (hai lần "nhân thử").
  # Khung khởi đầu mới có 1 mỗi loại.
  - kind: uses-operator, target: -, min: 2
  - kind: uses-operator, target: *, min: 2
  forbidAst:
  # Lưới thứ hai, chặn đúng ba con số là KẾT QUẢ. Lời giải thật không chứa
  # nguyên văn cái nào, nên luật này không cản ai làm thật.
  - kind: has-literal, target: 8
  - kind: has-literal, target: 16
  - kind: has-literal, target: 128
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^8\n16\n16\n128\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Thêm 8, rồi thêm 16 — chẳng ô nào giống ô nào. Nhưng nhân 2 thì lần nào cũng khít.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sau bảy giờ, hũ men của Byte có **128 phần**. Nghe thì to, nhưng đặt cạnh hàng
xóm là thấy nó bé tí.

Nhà bên cạnh có một cái máy trộn bột chạy điện. Máy ấy không nhân đôi gì cả —
nó chỉ làm ra đều đặn **1000 phần bột mỗi giờ**, không hơn không kém. Một hàm
bậc nhất, độ dốc 1000, cột thứ ba đứng im ở +1000.

Đọ thử ở giờ thứ bảy:

```text
  máy trộn:   1000 x 7  =  7000 phần
  hũ men:            2⁷ =   128 phần
```

Máy trộn bỏ xa, gấp hơn năm chục lần. Ở giờ thứ nhất, thứ hai, thứ ba — giờ nào
máy trộn cũng dẫn trước, và khoảng cách còn đang **nới rộng ra**.

Vậy thì hũ men thua, và thua đứt.

Nhưng hãy hỏi tiếp một câu. Máy trộn mỗi giờ cộng thêm đúng 1000. Hũ men mỗi
giờ nhân đôi — mà nhân đôi thì cái được thêm cũng nhân đôi theo: giờ này thêm
64 phần, giờ sau thêm 128 phần, giờ sau nữa thêm 256 phần…

Sau **20 giờ** thì ai hơn ai? Còn nếu máy trộn khoẻ hơn nữa — một triệu phần
mỗi giờ — thì câu trả lời có đổi không, hay chỉ đổi cái **lúc nào**?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
