---
id: toan.dai-so-va-ham-so.cho-bat-dau-cua-duong-thang
title: Chỗ bắt đầu của đường thẳng
summary: Ngoài độ dốc, một đường thẳng còn cần một con số nữa — thứ cái máy nhả ra khi bỏ 0 vào — và hai con số ấy tả trọn cả đường.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.linear-function]
requires: [math.slope, math.function-notation, math.graph-of-expression, math.coordinate-plane, math.value-table, math.negative-number, core.function-def, core.function-call, core.print-variable]
concepts: [math.gia-tri-luc-dau, math.cat-truc-doc, math.hai-duong-song-song]
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
Hai xe dốc y hệt nhau. Vậy cái gì làm chúng khác nhau?
::::

::::explain{#dong-dau-tien-cua-bang}
Bài trước để lại hai cái xe, cùng một câu hỏi: *sau khi bán `n` ổ, đang cầm bao
nhiêu tiền?*

- **Xe An** — bán trước cửa nhà, không mất tiền chỗ.
- **Xe Byte** — ở góc chợ, đã trả trước 30 nghìn tiền thuê chỗ từ lúc dọn hàng.

Cả hai bán 15 nghìn một ổ, nên **cùng độ dốc**. Vậy chúng khác nhau ở đâu?

Bài trước đã dạy chỗ cần nhìn — **cái bảng**, không phải cái hình. Và bảng gợi ý
luôn: hãy xem dòng đầu tiên, dòng `n = 0`, lúc chưa ai bán ổ nào.

```text
  n │  xe An │ xe Byte
───┼────────┼─────────
  0 │      0 │  -30000
  1 │  15000 │  -15000
  2 │  30000 │       0
  3 │  45000 │   15000
  4 │  60000 │   30000
  5 │  75000 │   45000
```

Cột thứ ba của cả hai xe đều là +15000 ở mọi dòng — đúng như đã đo. Nhưng hai
cột kết quả thì lệch nhau **30 000 ở mọi dòng**, không dòng nào nhiều hơn, không
dòng nào ít hơn. Chỗ lệch ấy sinh ra ngay từ dòng đầu và không bao giờ thu hẹp
lại, vì mỗi bước hai xe đều thêm như nhau.

Nên tất cả sự khác biệt gói gọn vào **một con số ở một dòng**: dòng `n = 0`.

- Xe An: chưa bán ổ nào thì trong tay đúng **0** đồng.
- Xe Byte: chưa bán ổ nào mà đã trả 30 nghìn thuê chỗ, nên trong tay
  **−30 000** đồng. Số âm ở đây không có gì lạ — mốc 0 là chỗ **gỡ xong tiền
  thuê chỗ**, còn dưới mốc nghĩa là đang thiếu, đúng như T2.1 đã dựng trên trục
  số. (Đừng lẫn với "hoà vốn" của bài 17: ở đó còn trừ cả 9 000 tiền vốn mỗi ổ
  nữa, nên mốc hoà vốn thật rơi vào 5 ổ. Máy `g` của bài này chỉ đếm tiền thu
  trừ tiền thuê chỗ, chưa đụng tới tiền vốn.)

Viết hai cái máy ra đầy đủ, bằng ký hiệu của bài trước:

> `f(n) = 15000 × n` — xe An
>
> `g(n) = 15000 × n − 30000` — xe Byte

Con số ở dòng `n = 0` chính là **`f(0)`** và **`g(0)`**: thứ cái máy nhả ra khi
bỏ số 0 vào. Bỏ 0 vào thì phần có `n` biến mất — `15000 × 0` là 0 — và thứ còn
lại là đúng cái phần **không dính `n`**.

Ở máy `f` thì phần ấy không có, nên `f(0) = 0`. Ở máy `g` thì phần ấy là
`−30000`, nên `g(0) = −30000`.

Gọi con số ấy là **`b`**: giá trị của máy lúc đầu vào bằng 0.
::::

::::example{#hai-duong-song-song}
Chấm cả hai lên một mặt phẳng. Mỗi ô ngang là 1 ổ, mỗi hàng dọc là 15 nghìn
đồng; `A` là xe An, `B` là xe Byte.

```text
nghìn đồng
  75 ┤              A
  60 ┤           A
  45 ┤        A     B
  30 ┤     A     B
  15 ┤  A     B
   0 A─────B────────────→ n
 -15 ┤  B
 -30 B
     0  1  2  3  4  5
```

Hai chuyện đọc được ngay từ hình:

**Một.** Hai đường nghiêng y hệt nhau. Chúng không cắt nhau ở đâu cả, dù kéo dài
tới đâu — vì cùng độ dốc thì mỗi bước chúng lên bằng nhau, nên khoảng cách giữa
chúng không bao giờ đổi. Đó là hai đường **song song**. Nhìn theo chiều dọc ở
bất kỳ cột nào: `B` luôn ở dưới `A` đúng hai hàng, tức 30 nghìn.

**Hai.** Cột dọc ngoài cùng bên trái là cột `n = 0` — nó chính là **trục dọc**
mà bạn đã dựng ở bài 22. Nhìn xem mỗi đường đụng vào trục ấy ở chỗ nào: đường
của An đụng ngay chỗ số 0, đường của Byte đụng ở chỗ −30.

Đó không phải trùng hợp, và cũng không phải một sự thật thứ hai phải nhớ. Trục
dọc **là** cột `n = 0`. Nên "giá trị lúc đầu vào bằng 0" và "chỗ đường cắt trục
dọc" là đúng một chỗ, chỉ là gọi bằng ngôn ngữ bảng hay ngôn ngữ hình.

> `b` là hai thứ cùng lúc: con số ở dòng đầu của bảng, và chỗ đường cắt trục dọc
> trên hình.

Bây giờ ghép với bài trước. Một đường thẳng cần đúng **hai** con số:

| máy | `a` — độ dốc | `b` — lúc `n = 0` | đường trông thế nào |
|---|---|---|---|
| `f(n) = 15000n` | 15000 | 0 | đi lên, cắt trục dọc ở 0 |
| `g(n) = 15000n − 30000` | 15000 | −30000 | đi lên **y hệt** `f`, nhưng cả đường tụt xuống 30 000 |
| `h(n) = −200n` | −200 | 0 | đi xuống, cắt trục dọc ở 0 |

`a` nói đường **nghiêng thế nào**, `b` nói đường **nằm ở đâu**. Hai con số ấy
không thay nhau được: đổi `a` là đổi hướng, còn đổi `b` là nhấc cả đường lên
hoặc dìm cả đường xuống mà hướng không suy suyển. Biết cả hai là biết trọn đường
thẳng — không cần chấm thêm một điểm nào nữa. Nên mọi đường thẳng đều viết được
dưới đúng một hình dạng:

> **`f(n) = a × n + b`**

Trong đó `a` và `b` là hai con số cụ thể, còn `n` vẫn là ô trống như từ bài 1.

*(Sách toán hay viết hình dạng này thành `y = ax + b`. Không có gì mới trong đó:
`x` là cột "điền gì", `y` là cột "ra gì" — đúng hai cột của cái bảng bài 5, và
`y` chính là con số bạn chấm lên trục dọc từ bài 23. Ba cách viết `15000n`,
`f(n) = 15000n` và `y = 15000x` đang nói cùng một chuyện; bài này dùng cách viết
có tên máy, vì có tên thì gọi được.)*
::::

::::predict{#doan-hai-xe commitOnce}
Byte dựng cả hai cái máy rồi hỏi bốn câu.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def xe_an(n):
    return 15000 * n

def xe_byte(n):
    return 15000 * n - 30000

print(xe_an(0))
print(xe_byte(0))
print(xe_byte(2))
print(xe_an(2) - xe_byte(2))
```

:::opt{correct}
0 rồi -30000 rồi 0 rồi 30000
:::

:::opt
0 rồi 0 rồi 0 rồi 30000
::why
Gần đúng ở chỗ bạn dùng một luật rất mạnh và dùng đúng: **nhân với 0 thì mất
sạch**. `15000 × 0` đúng là 0, không cần tính. Ở máy `xe_an` luật ấy cho ra đúng
đáp án, và ở máy `xe_byte` nó cũng đúng — với phần mà nó có quyền đụng tới.

Ranh giới nằm ở chỗ "phần nào có quyền đụng tới". Số 0 chỉ nuốt được cái đang
**nhân với nó**, tức là cụm `15000 × n`. Còn `−30000` đứng riêng một mình, không
dính `n`, nên phép nhân không chạm được vào nó — nó ở lại nguyên vẹn. Đó chính
là lý do `b` tồn tại: nó là đúng cái phần mà số 0 không nuốt được.
::
:::

:::opt
0 rồi -30000 rồi 0 rồi 0
::why
Gần đúng ở chỗ bạn tính trúng ba dòng đầu, kể cả dòng khó nhất — thấy
`xe_byte(2)` bằng 0 nghĩa là bạn đã hiểu chuyện Byte bán 2 ổ thì vừa gỡ đúng
tiền thuê chỗ.

Chỗ lệch ở dòng cuối, và nó đến từ một luật đúng bị kéo dài quá: *hai xe cùng độ
dốc*. Cùng độ dốc nghĩa là mỗi bước chúng **thêm** như nhau — chứ không phải
chúng **bằng** nhau. Hai đường song song không bao giờ chạm nhau; khoảng cách 30
000 giữa chúng có ở dòng đầu và còn nguyên ở mọi dòng sau. Tại `n = 2`, An đang
cầm 30 000 còn Byte mới về mốc 0.
::
:::

:::opt
Máy báo lỗi ở dòng thứ hai, vì tiền không thể là số âm
::why
Gần đúng ở chỗ bạn đang giữ một thói quen tốt: đối chiếu con số với chuyện thật
ngoài đời. Không ai cầm âm ba mươi nghìn đồng trong tay được, và nghi ngờ một
kết quả nghe vô lý là phản xạ đúng của người làm toán.

Ranh giới là **mốc 0 đang đặt ở đâu**. T2.1 đã dựng chuyện này trên trục số: số
âm không phải "số không tồn tại", nó là một chỗ **nằm dưới mốc**. Ở đây mốc 0
không phải "không có gì" mà là "hoà vốn", nên −30 000 nghĩa là *đang thiếu 30
nghìn* — đúng tình cảnh của Byte lúc vừa trả tiền chỗ mà chưa bán được ổ nào.
Còn máy tính thì không hề biết đó là tiền; với nó, `0 − 30000` chỉ là một phép
trừ.
::
:::
::::

::::explain{#hai-so-la-du}
Thu lại một câu:

> Hai con số `a` và `b` tả trọn một đường thẳng. `a` là lượng đầu ra đổi khi đầu
> vào thêm 1. `b` là đầu ra lúc đầu vào bằng 0.

Câu ấy mạnh hơn vẻ ngoài của nó. Nó nói rằng bạn **không cần** cái bảng nữa: đưa
hai con số là dựng lại được mọi dòng của bảng, mọi chấm của hình, mọi câu trả
lời của máy. Cả một đường dài vô tận gói vào hai con số.

Và nó cũng nói ngược lại: hai cái máy khác nhau thì phải khác nhau ở `a`, hoặc ở
`b`, hoặc ở cả hai — không còn chỗ nào khác để khác nhau.

Thử ba xe cùng lúc, cả ba đều bán 15 nghìn một ổ, để thấy `b` làm gì:

- **An** không mất tiền chỗ: `b = 0`, đường xuất phát ngay từ mốc.
- **Byte** trả trước 30 nghìn thuê chỗ: `b = −30000`, cả đường bị dìm xuống 30
  nghìn.
- **Tí** cũng bán trước cửa nhà, nhưng sáng nay một quán cà phê đã trả trước 45
  nghìn tiền đặt bánh cho buổi chiều — nên chưa bán ổ nào Tí đã cầm sẵn 45
  nghìn: `b = 45000`, cả đường được nhấc lên 45 nghìn.

Ba đường song song nhau, ba chỗ cắt trục dọc khác nhau. `b` không làm đường
nghiêng thêm hay bớt — nó chỉ nâng lên hạ xuống.
::::

::::code{#ba-xe-mot-buoi-sang}
Dựng ba cái máy của ba cái xe, rồi đọc `b` của từng cái **bằng chính cái máy**,
đừng chép con số từ dòng `return`.

Cả ba xe bán cùng giá 15 000 đồng một ổ, và cả ba trả lời cùng một câu: *sau khi
bán `n` ổ, đang cầm bao nhiêu tiền?*

- `xe_an` — không mất tiền chỗ.
- `xe_byte` — đã trả trước 30 000 đồng thuê chỗ.
- `xe_ti` — đã nhận trước 45 000 đồng tiền đặt bánh.

Năm chỗ trống: hai chỗ tả luật của hai cái máy còn thiếu, hai chỗ đọc `b`, và
một chỗ đo khoảng cách dọc giữa đường của Tí và đường của Byte.

Xe An đã làm mẫu sẵn cả hai việc, chép cách làm của nó là được.

```python title=starter
def xe_an(n):
    return 15000 * n

def xe_byte(n):
    return ___

def xe_ti(n):
    return ___

# `b` của một cái máy là thứ nó nhả ra khi bỏ 0 vào. Xe An làm mẫu:
b_an = xe_an(0)
b_byte = ___
b_ti = ___

# Đường của Tí nằm cao hơn đường của Byte bao nhiêu?
chenh = ___

print(b_an)
print(b_byte)
print(b_ti)
print(chenh)
```

```python title=solution
def xe_an(n):
    return 15000 * n

def xe_byte(n):
    return 15000 * n - 30000

def xe_ti(n):
    return 15000 * n + 45000

# `b` của một cái máy là thứ nó nhả ra khi bỏ 0 vào. Xe An làm mẫu:
b_an = xe_an(0)
b_byte = xe_byte(0)
b_ti = xe_ti(0)

# Đường của Tí nằm cao hơn đường của Byte bao nhiêu?
chenh = xe_ti(0) - xe_byte(0)

print(b_an)
print(b_byte)
print(b_ti)
print(chenh)
```

```python title=test
# Ba câu `!=` đứng trước: chúng canh cái bẫy "ba xe hoá một". Xếp sau các câu
# `==` thì chúng không bao giờ chạy tới, và cái bẫy không bao giờ sập.
assert b_byte != b_an, "Byte trả tiền thuê chỗ còn An thì không, nên `b` của hai xe không thể bằng nhau"
assert b_ti != b_byte, "Tí nhận tiền đặt trước còn Byte mất tiền thuê chỗ — hai xe này lệch nhau xa nhất"
assert xe_byte(4) != xe_an(4), "hai đường song song thì không bao giờ chạm nhau, kể cả ở n = 4"
assert xe_byte(5) - xe_byte(4) == xe_an(5) - xe_an(4), "ba xe cùng giá 15 000 một ổ, nên một bước của xe nào cũng phải bằng nhau"
assert xe_ti(9) - xe_ti(8) == 15000, "xe Tí cũng bán 15 000 một ổ, nên bước của nó vẫn là 15 000 — tiền đặt trước không làm đường dốc thêm"
assert b_an == 0, "An chưa bán ổ nào và cũng không mất gì, nên `b` của An là 0"
assert b_byte == -30000, "Byte chưa bán ổ nào mà đã trả 30 000 thuê chỗ, nên `b` của Byte là -30000"
assert b_ti == 45000, "Tí chưa bán ổ nào đã cầm sẵn 45 000 tiền đặt bánh, nên `b` của Tí là 45000"
assert xe_byte(2) == 0, "bán 2 ổ được 30 000, vừa đúng tiền thuê chỗ — máy này về số 0, tức gỡ xong tiền thuê chỗ (chưa trừ tiền vốn mỗi ổ, nên chưa phải hoà vốn của bài 17)"
assert xe_an(4) - xe_byte(4) == 30000, "khoảng cách giữa hai đường song song là 30 000 ở mọi chỗ, kể cả ở n = 4"
assert chenh == 75000, "đường của Tí nằm trên 45 000, đường của Byte nằm dưới 30 000: cách nhau 75 000"
```

:::hints
- kind: attention
  body: Nhìn dòng `return` của xe An. Nó có đúng một phần: phần nhân với `n`. Hai xe còn lại có thêm một phần nữa — phần không dính `n`, đúng cái phần mà bỏ 0 vào thì nó vẫn ở lại. Tiền thuê chỗ làm túi vơi đi, tiền đặt trước làm túi dày lên; hai chuyện ngược nhau nên hai dấu cũng ngược nhau.
- kind: strategy
  body: Ba chỗ trống cuối không cần bạn tính gì cả — chúng cần bạn HỎI cái máy. Dòng `b_an = xe_an(0)` cho sẵn khuôn: tên máy, mở ngoặc, số 0. Chỗ `chenh` thì hỏi hai máy rồi lấy chỗ cao trừ chỗ thấp; viết ngược thứ tự là ra số âm.
- kind: one-line
  body: "Hai luật là `15000 * n - 30000` và `15000 * n + 45000`; ba chỗ còn lại là `xe_byte(0)`, `xe_ti(0)`, `xe_ti(0) - xe_byte(0)`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hai dòng `return` phải tính bằng chính ô trống `n` rồi cộng hoặc trừ phần không dính `n`, còn ba chỗ trống dưới phải GỌI máy chứ không chép con số — chép thì không có cái máy nào chạy cả
  requireAst:
  # Khung khởi đầu đọc `n` đúng một lần (trong `xe_an`). Hai luật máy còn thiếu
  # phải đọc thêm mỗi cái một lần, nên `min: 3` chặn đúng đáp án viết luật bằng
  # một con số thay vì bằng ô trống.
  - kind: uses-name, target: n, min: 3
  - kind: uses-operator, target: *, min: 3
  # Tiền thuê chỗ là trừ, tiền đặt trước là cộng — hai dấu ngược nhau, và bài
  # hỏng nếu chỉ có một trong hai. Dấu trừ còn cần thêm một lần nữa ở `chenh`.
  - kind: uses-operator, target: -, min: 2
  - kind: uses-operator, target: +, min: 1
  # Hai luật dưới đây chặn đáp án gõ cứng `b`: mỗi máy phải bị HỎI ít nhất hai
  # lần (một lần đọc `b`, một lần ở `chenh`). Khung khởi đầu không gọi máy nào
  # trong hai máy này.
  - kind: uses-call, target: xe_byte, min: 2
  - kind: uses-call, target: xe_ti, min: 2
  forbidAst:
  # Lưới thứ hai, chặn con số KẾT QUẢ của chỗ trống cuối. Lời giải thật chỉ chứa
  # 15000, 30000, 45000 và số 0 — không chỗ nào có nguyên văn 75000.
  - kind: has-literal, target: 75000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^0\n-30000\n45000\n75000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai con số là xong một đường. Một số bảo nó nghiêng cỡ nào, một số bảo nó nằm ở
đâu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`a` và `b` đủ để tả **mọi** đường thẳng. Nhưng câu ấy chỉ có nghĩa nếu mọi cái
máy đều vẽ ra đường thẳng — và chưa ai kiểm điều đó.

Sau nhà Byte có một mảnh sân hình vuông. Byte định lát gạch, nên cần biết diện
tích. Cạnh dài `n` mét thì diện tích là `n × n` mét vuông — một cái máy mới:

> `s(n) = n × n`

Nó trông vô hại. Nhưng bài trước đã đưa cho bạn đúng một công cụ để soi mọi cái
máy: lấy **cột thứ ba** ra xem. Lập bảng cho `n` chạy 0, 1, 2, 3, 4, 5 rồi tính
xem mỗi bước `s` thêm bao nhiêu.

Con số ấy có đứng im như 15 000 của xe bánh mì không? Nếu nó **không** đứng im
thì cả bài này sụp: không có `a` nào để ghi, và cái hình cũng không còn quyền
thẳng nữa.

Vậy nó sẽ trông ra sao?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
