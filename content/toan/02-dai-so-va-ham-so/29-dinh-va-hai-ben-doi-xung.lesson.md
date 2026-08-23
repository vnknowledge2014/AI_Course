---
id: toan.dai-so-va-ham-so.dinh-va-hai-ben-doi-xung
title: Đỉnh, và hai bên đối xứng
summary: Đường lãi theo giá có một cái gương — hai mức giá khác hẳn nhau cho đúng cùng một khoản lãi, và điểm duy nhất không có bạn qua gương chính là đỉnh.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 29
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.parabola-symmetry, math.vertex]
practices: [math.quadratic-function, math.value-table, math.multiply-commutative, math.negative-number]
requires: [math.quadratic-function, math.value-table, math.function-notation, math.graph-of-expression, math.multiply-commutative, math.multiplication, math.multiply-distributive, math.negative-number, math.compare-on-number-line, core.function-def, core.function-call, core.function-parameter, core.function-argument, core.function-return, core.variable, core.assignment, core.arithmetic, core.print-variable, core.number-literal, core.output]
concepts: [math.truc-doi-xung, math.do-thi, math.bang-gia-tri, math.xe-banh-mi]
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
Tăng giá thì lãi mỗi ổ cao hơn, mà bán được ít ổ đi. Tăng bao nhiêu là vừa?
::::

::::explain{#cau-hoi-tang-gia}
Bài trước để lại đúng câu hỏi này, nên hôm nay dựng nó ra thành một câu tính.

Tình hình hiện tại của xe bánh mì:

- bán **15 nghìn** một ổ,
- mỗi ổ **lãi 5 nghìn** (10 nghìn còn lại là tiền vốn),
- mỗi ngày hết **60 ổ**.

Byte hỏi mấy người bán quen và rút ra một quy luật: **cứ tăng giá 1 nghìn thì
mỗi ngày bán hụt đi 4 ổ.**

Ô trống của bài này không phải giá bán, mà là **tăng bao nhiêu nghìn so với giá
đang bán**. Gọi nó là `t`. Chọn ô trống kiểu này có một cái lợi mà lát nữa sẽ
trả công: `t` được phép là **số âm** — `t = -2` nghĩa là **hạ giá** 2 nghìn,
xuống còn 13 nghìn một ổ. Hoàn toàn có nghĩa ngoài đời.

Điền `t` vào, hai thứ đổi cùng lúc và đổi ngược chiều nhau:

```text
  lãi mỗi ổ (nghìn)  :  5 + t          -- tăng giá thì phần này lớn lên
  số ổ bán được      :  60 - 4t        -- tăng giá thì phần này nhỏ đi
```

Lãi cả ngày là lãi mỗi ổ nhân số ổ:

> `L(t) = (5 + t) × (60 - 4t)`

Hai cụm, mỗi cụm chứa `t`, nhân với nhau — nên khi mở ngoặc ra, `t` sẽ gặp `t`.
Theo bài trước, đó là dấu hiệu của một máy **bậc hai**, và đồ thị của nó phải
cong.
::::

::::example{#bang-lai}
Điền thử `t` từ 0 lên 6 rồi ghi lại, đúng như bài 5 đã dạy.

```text
  t   │ L(t) │ cột 3: bước từ dòng trên │ cột 4: bước của cột 3
──────┼──────┼──────────────────────────┼──────────────────────
    0 │  300 │                          │
    1 │  336 │                      +36 │
    2 │  364 │                      +28 │                    -8
    3 │  384 │                      +20 │                    -8
    4 │  396 │                      +12 │                    -8
    5 │  400 │                       +4 │                    -8
    6 │  396 │                       -4 │                    -8
```

Cột thứ ba không đứng im, cột thứ tư thì có — đúng dấu hiệu của một máy bậc
hai, y như mảnh sân bài trước. Nhưng lần này con số cố định ấy **âm**, và cái
âm đó đổi hẳn hình dáng câu chuyện: mỗi bước thêm ít hơn bước trước 8 nghìn,
nên sớm muộn cột thứ ba phải tụt xuống dưới 0 và đường quay đầu đi xuống.

Nó quay đầu ở đâu thì bảng chỉ thẳng: bước từ `t = 4` sang `t = 5` còn thêm
được 4 nghìn; bước tiếp theo mất 4 nghìn. Cao nhất là `t = 5`, tức bán 20 nghìn
một ổ, được 400 nghìn một ngày.

Vẽ ra cho thấy cái gò:

```text
  t =   0  |##########           | 300
  t =   1  |#############        | 336
  t =   2  |################     | 364
  t =   3  |##################   | 384
  t =   4  |###################  | 396
  t =   5  |#################### | 400
  t =   6  |###################  | 396
```

Hai thanh `t = 4` và `t = 6` dài bằng nhau. Hai mức giá khác nhau, cùng một
khoản lãi. Đó là chuyện tình cờ, hay là dấu hiệu của cái gì?
::::

::::predict{#doan-ba-cau commitOnce}
Byte gõ cái máy lãi vào Python đúng như đã viết trên giấy, rồi hỏi nó ba câu —
cả ba đều nằm **ngoài** khúc bảng vừa dựng.

Câu thứ hai: tăng giá tận 10 nghìn (bán 25 nghìn một ổ). Câu thứ ba: hạ giá 2
nghìn (bán 13 nghìn một ổ).

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def lai(t):
    return (5 + t) * (60 - 4 * t)

print(lai(0))
print(lai(10))
print(lai(-2))
```

:::opt{correct}
`300`, rồi `300`, rồi `204`
:::

:::opt
`300`, rồi `900`, rồi `204`
::why
Chỗ đúng trong suy nghĩ của bạn là một luật buôn bán thật sự đúng: **lãi mỗi ổ
cao hơn thì lãi cả ngày cao hơn.** Tăng 10 nghìn thì mỗi ổ lãi 15 nghìn thay vì
5 — gấp ba. Nhân với 60 ổ ra 900. Phép tính không sai chỗ nào.

Ranh giới nằm ở ba chữ bị bỏ quên: *nếu số ổ bán được giữ nguyên*. Luật ấy đúng
khi chỉ có **một** thứ đổi. Ở đây tăng giá kéo theo bán hụt: `60 - 4 × 10 = 20`
ổ, chứ không còn 60. Lãi cả ngày là `15 × 20 = 300`. Cả bài này chỉ nói về đúng
chỗ đó — hai cụm cùng chứa `t` và kéo ngược nhau, nên không cụm nào một mình
quyết định được kết quả.
::
:::

:::opt
`300`, rồi `300`, rồi `-204`
::why
Chỗ đúng: bạn nhìn thấy số âm và nhớ ngay rằng nó lật kết quả sang bên kia mốc
0. Với `-2 × 102` thì linh cảm ấy đúng tuyệt đối.

Ranh giới là **`-2` đang đứng ở vai nào**. Nó không phải một thừa số của phép
nhân cuối cùng; nó là **đầu vào**, và nó chui vào hai cụm trước khi có phép
nhân nào. Cụm thứ nhất `5 + (-2)` thành 3 — vẫn dương. Cụm thứ hai
`60 - 4 × (-2)` thành `60 + 8 = 68` — dương và còn to hơn. Nhân hai số dương
thì ra `3 × 68 = 204`. Hạ giá thì lãi ít đi thật, nhưng "ít đi" không có nghĩa
là "âm": Byte vẫn bán được bánh và vẫn thu về tiền.
::
:::

:::opt
`300`, rồi `300`, rồi `364`
::why
Chỗ đúng ở đây là một quan sát rất sắc, và nó là hạt nhân của cả bài: **parabol
có hai bên đối xứng nhau.** Bạn còn nhớ đúng bảng của bài trước, chỗ máy
`s(n) = n × n` cho `s(-3)` và `s(3)` cùng ra 9 — trái phải soi gương qua mốc 0.
Áp lên đây, `t = -2` sẽ soi ra `t = 2`, tức `364`.

Ranh giới là **cái gương nằm ở đâu**. Với `n × n` thì gương đúng là ở 0, vì đổi
dấu `n` không đổi tích. Còn máy lãi thì cái gò của nó nằm ở `t = 5` chứ không ở
0 — bảng phía trên đã cho thấy nó vẫn còn đang leo lên suốt từ 0 tới 5. Gương
phải đi qua chỗ cao nhất, nên bạn của `-2` không phải `2`. Mục ngay sau đây tìm
ra nó là số mấy.
::
:::
::::

::::explain{#cai-guong}
Kéo dài bảng ra cả hai phía, từ hạ giá 2 nghìn tới tăng giá 12 nghìn:

```text
  t:      -2   -1    0    1    2    3    4    5    6    7    8    9   10   11   12
  lãi:   204  256  300  336  364  384  396  400  396  384  364  336  300  256  204
```

Đọc hàng dưới từ hai đầu vào giữa: `204` gặp `204`, `256` gặp `256`, `300` gặp
`300`… Mọi con số đều có một bạn, trừ đúng một số.

```text
  t = -2  |                     | 204        ┐
  t = -1  |#####                | 256       ┐│
  t =  0  |##########           | 300      ┐││
  t =  1  |#############        | 336     ┐│││
  t =  2  |################     | 364    ┐││││
  t =  3  |##################   | 384   ┐│││││
  t =  4  |###################  | 396  ┐││││││
  t =  5  |#################### | 400  │││││││  <-- đỉnh: không có bạn
  t =  6  |###################  | 396  ┘││││││
  t =  7  |##################   | 384   ┘│││││
  t =  8  |################     | 364    ┘││││
  t =  9  |#############        | 336     ┘│││
  t = 10  |##########           | 300      ┘││
  t = 11  |#####                | 256       ┘│
  t = 12  |                     | 204        ┘
```

Cả cái gò gập đôi lại vừa khít qua đường kẻ ở `t = 5`. Đó là một **cái gương**.

**Vì sao có gương?** Không phải trùng hợp. Lấy hai điểm ghép đôi ở trên và viết
hẳn phép nhân ra:

```text
  t = 2 :  lãi mỗi ổ  7 nghìn  ×  52 ổ  =  364
  t = 8 :  lãi mỗi ổ 13 nghìn  ×  28 ổ  =  364
```

Nhìn kỹ bốn con số: `52` chính là `4 × 13`, và `28` chính là `4 × 7`. Nên hai
phép nhân ấy là:

```text
  t = 2 :   7 × (4 × 13)
  t = 8 :  13 × (4 ×  7)
```

Cùng ba con số `7`, `4`, `13` — chỉ khác thứ tự. Bài 20 của track trước đã chốt
chuyện này bằng một mảng chữ nhật xoay một góc: **đổi thứ tự các thừa số thì
tích không đổi.** Hai mức giá ấy cho cùng một khoản lãi vì chúng là cùng một
mảng, nhìn từ hai phía.

Chuyện đó lặp lại ở mọi cặp: đi từ `t` sang `10 - t` thì hai cụm **đổi chỗ cho
nhau**, còn tích thì đứng yên. Nên luật gương viết được thành một dòng:

> Bạn của `t` qua gương là `10 - t`.

Thử: bạn của `2` là `8` ✓. Bạn của `-2` là `12` ✓. Bạn của `0` là `10` ✓ — đúng
là hai ô cùng ra 300 trong bảng.

Bây giờ tới chỗ đáng nhớ nhất của bài. Hỏi ngược: **có số nào là bạn của chính
nó không?** Cần `t = 10 - t`, tức `t` phải đứng đúng giữa 0 và 10:

> `t = 5`

Đúng cái ô lẻ loi trong bảng. Nó không lẻ loi vì Byte chọn nó; nó lẻ loi vì mọi
ô khác đều đi thành cặp, còn nó thì soi gương ra chính mình. Chỗ ấy có tên:

> **Đỉnh** của một parabol là điểm nằm ngay trên trục gương — điểm duy nhất
> không có bạn đối xứng. Gò quay lên thì đỉnh là chỗ cao nhất; gò quay xuống
> thì đỉnh là chỗ thấp nhất.

Và đó là câu trả lời cho bài trước, phần "trên hình có gì giúp nhận ra chỗ cao
nhất": **gấp đôi tờ giấy sao cho hai bên đường cong chồng khít lên nhau; nếp
gấp đi qua đỉnh.** Không phải dò từng số một.
::::

::::explain{#van-la-mot-cai-may}
Có một chỗ dễ gợn, và đáng gợn — vì nó chạm vào chính định nghĩa ở bài 24.

`lai(2)` và `lai(8)` cùng ra `364`. Hai đầu vào khác nhau, một đầu ra. Vậy
`lai` còn là **hàm số** nữa không?

Đọc lại luật của bài 24 cho kỹ:

> Mỗi đầu vào cho ra **đúng một** đầu ra.

Luật ấy nói về chiều **vào → ra**. Hỏi `lai(2)` thì lúc nào cũng chỉ được đúng
một câu trả lời — `364`, không bao giờ hai câu. Còn chuyện một đầu ra được
nhiều đầu vào cùng trỏ tới thì luật **không hề cấm**.

Nghĩ theo đời thật thì nó tự nhiên: hỏi "bán 20 nghìn một ổ thì lãi bao nhiêu"
phải có đúng một đáp án, không thì cái quán không tính sổ được. Còn "lãi 364
nghìn" hoàn toàn có thể tới từ hai cách bán khác nhau — và hôm nay bạn vừa thấy
đúng hai cách ấy.

Giữ chỗ này lại. Vài bài nữa, khi bạn muốn chạy cái máy **ngược** — đưa 364 vào
để đòi lại `t` — đúng chỗ này sẽ là chỗ hỏng.
::::

::::code{#soi-guong}
Bắt máy kiểm cái gương giúp bạn.

Bạn viết luật gương thành một cái máy nhỏ tên `soi_guong`, rồi dùng nó để tìm
bạn của hai điểm và so lãi ở từng cặp.

Bài chấm bằng cả hai cặp, và hai cặp cho hai khoản lãi khác nhau — nên gõ cứng
một con số vào mọi chỗ thì hỏng ngay ở cặp thứ hai. Riêng `soi_guong` phải là
một cái **luật viết theo `t`**, vì lát nữa nó bị hỏi cả bằng số âm.

```python title=starter
def lai(t):
    return (5 + t) * (60 - 4 * t)

# Trục gương nằm ở t = 5. Điểm nào cách gương bao nhiêu về bên này thì bạn của
# nó cách gương đúng bấy nhiêu về bên kia.
def soi_guong(t):
    return ___

ban_cua_2 = soi_guong(2)
ban_cua_am_2 = soi_guong(-2)

lai_tai_2 = lai(2)
lai_tai_ban_cua_2 = ___

lai_tai_am_2 = lai(-2)
lai_tai_ban_cua_am_2 = ___

print(ban_cua_2)
print(ban_cua_am_2)
print(lai_tai_2)
print(lai_tai_ban_cua_2)
print(lai_tai_am_2)
print(lai_tai_ban_cua_am_2)
```

```python title=solution
def lai(t):
    return (5 + t) * (60 - 4 * t)

# Trục gương nằm ở t = 5. Điểm nào cách gương bao nhiêu về bên này thì bạn của
# nó cách gương đúng bấy nhiêu về bên kia.
def soi_guong(t):
    return 10 - t

ban_cua_2 = soi_guong(2)
ban_cua_am_2 = soi_guong(-2)

lai_tai_2 = lai(2)
lai_tai_ban_cua_2 = lai(ban_cua_2)

lai_tai_am_2 = lai(-2)
lai_tai_ban_cua_am_2 = lai(ban_cua_am_2)

print(ban_cua_2)
print(ban_cua_am_2)
print(lai_tai_2)
print(lai_tai_ban_cua_2)
print(lai_tai_am_2)
print(lai_tai_ban_cua_am_2)
```

```python title=test
# Hai câu != đứng trước: nếu `soi_guong` trả về cùng một chỗ cho mọi đầu vào,
# hoặc hai cặp cho cùng một khoản lãi, thì mọi câu == phía dưới có đúng cũng
# không chứng minh được cái gương nào cả.
assert ban_cua_2 != ban_cua_am_2, "hai điểm khác nhau phải soi ra hai bạn khác nhau — trả về cùng một số cho mọi t thì đó không phải cái gương"
assert lai_tai_2 != lai_tai_am_2, "t = 2 và t = -2 cách gương khác nhau nên lãi phải khác nhau: 364 với 204"
assert ban_cua_2 == 8, "t = 2 cách gương (t = 5) ba bước về bên trái, nên bạn nó cách gương ba bước về bên phải: t = 8"
assert ban_cua_am_2 == 12, "t = -2 cách gương bảy bước về bên trái, nên bạn nó là t = 12"
assert lai_tai_2 == 364, "tăng 2 nghìn: mỗi ổ lãi 7 nghìn, bán 52 ổ, cả ngày 364 nghìn"
assert lai_tai_ban_cua_2 == lai_tai_2, "hai đầu vào khác nhau cho cùng một đầu ra — chính là cái gương: 7 x (4 x 13) và 13 x (4 x 7)"
assert lai_tai_am_2 == 204, "hạ 2 nghìn: mỗi ổ lãi 3 nghìn, bán 68 ổ, cả ngày 204 nghìn"
assert lai_tai_ban_cua_am_2 == lai_tai_am_2, "cặp thứ hai cũng khít: hạ giá 2 nghìn và tăng giá 12 nghìn cho đúng cùng một khoản lãi"
assert soi_guong(5) == 5, "đỉnh là điểm duy nhất soi gương ra chính nó — đó là lý do nó không có bạn"
assert lai(5) == 400, "ở đỉnh, mỗi ổ lãi 10 nghìn và bán được 40 ổ: 400 nghìn, cao nhất cả bảng"
```

:::hints
- kind: attention
  body: Gương nằm ở `t = 5`. Thử một cặp đã biết trước từ bảng ở trên — `t = 4` soi ra `t = 6` — rồi hỏi xem phải làm gì với số 4 để ra số 6, mà làm đúng thao tác ấy với số 6 thì lại quay về 4. Hai chỗ trống còn lại nhìn dòng `lai_tai_2` ngay phía trên: nó đã viết sẵn khuôn bạn cần, chỉ khác chỗ bỏ vào.
- kind: strategy
  body: Điểm và bạn của nó cách gương bằng nhau nhưng ngược phía, nên hai số ấy cộng lại luôn ra gấp đôi vị trí gương. Từ đó suy ra bạn của `t` bằng cái tổng ấy trừ đi `t` — viết bằng chữ `t`, đừng viết một con số cụ thể, vì hàm này còn phải chạy với số âm. Hai chỗ trống cuối thì đừng gõ số vào: bỏ thẳng cái tên vừa tính được vào máy `lai`.
- kind: one-line
  body: "Ba chỗ trống lần lượt là `10 - t`, `lai(ban_cua_2)` và `lai(ban_cua_am_2)`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: `soi_guong` phải là một luật viết theo `t`, và hai chỗ trống cuối phải bỏ cái tên vừa tìm được vào máy `lai` — gõ thẳng con số vào là bạn tra bảng hộ máy, cái gương không được kiểm ở đâu cả
  requireAst:
  # Khung khởi đầu đọc `t` đúng 2 lần, cả hai nằm trong thân hàm `lai`. Chỗ
  # trống thứ nhất là chỗ duy nhất thêm được lần thứ ba.
  - kind: uses-name, target: t, min: 3
  # Khung khởi đầu gọi `lai` 2 lần; lời giải gọi 4. Điền số vào hai chỗ trống
  # cuối thì con số này tụt xuống.
  - kind: uses-call, target: lai, min: 4
  # Hai chỗ trống cuối phải ĐỌC LẠI tên vừa tính. Khung khởi đầu đọc mỗi tên
  # đúng 1 lần (trong `print`).
  - kind: uses-name, target: ban_cua_2, min: 2
  - kind: uses-name, target: ban_cua_am_2, min: 2
  # Luật gương là một phép trừ. Khung khởi đầu mới có 1 phép trừ, trong `lai`.
  - kind: uses-operator, target: -, min: 2
  forbidAst:
  # Lưới thứ hai, chặn đúng những con số là KẾT QUẢ. Lời giải thật không chứa
  # nguyên văn cái nào, nên luật này không cản ai làm thật.
  - kind: has-literal, target: 8
  - kind: has-literal, target: 12
  - kind: has-literal, target: 364
  - kind: has-literal, target: 204
  - kind: has-literal, target: 400
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^8\n12\n364\n364\n204\n204\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tăng 2 nghìn hay tăng 8 nghìn — cùng 364. Cái gương không bỏ sót cặp nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại hai loại máy đã gặp, và chỉ nhìn vào **cách một bước đi hoạt động**:

```text
  bậc nhất :  mỗi bước THÊM đúng một lượng cố định   ( +15000, +15000, ... )
  bậc hai  :  mỗi bước THÊM một lượng đổi đều        ( +36, +28, +20, ... )
```

Khác nhau nhiều, nhưng cả hai đều là **thêm**. Cái bước đi luôn là một phép
cộng; chỉ có lượng cộng vào là thay đổi hay không.

Bây giờ sang một góc khác của xe bánh mì. Byte nuôi một hũ men để ủ bột. Men
sống, và cứ mỗi giờ nó **nhân đôi**: đang một phần thì thành hai phần, hai
phần thành bốn phần.

Ở đây bước đi không còn là "thêm bao nhiêu" nữa. Nó là "**nhân với** bao
nhiêu".

Vậy bảng của hũ men trông thế nào? Đem đúng bộ đồ nghề của hai bài vừa rồi ra
đo nó — dựng cột thứ ba, rồi dựng cột thứ tư — thì có tìm ra con số nào chịu
đứng im không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
