---
id: toan.logic-va-chung-minh.xet-mot-so-bat-ky
title: Xét một số bất kỳ
summary: Chữ "bất kỳ" đặt xuống giấy được — cho nó một cái tên, rồi cấm mình dùng bất cứ điều gì mà giả thiết chưa cho; lập luận chạy một lần và đúng cho vô hạn trường hợp.
locale: vi
track: toan
module: logic-va-chung-minh
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.direct-proof]
requires: [logic.use-definition, logic.implication, logic.finite-check-not-proof, logic.counterexample, logic.for-all, logic.exists, logic.open-sentence, math.letter-names-a-slot, math.factor-common, math.multiply-distributive, math.equivalent-expression, math.remainder, math.multiplication, math.exponent, core.boolean, core.variable, core.number-literal, core.arithmetic, core.modulo, core.function-def, core.function-call, core.function-parameter, core.function-return, core.none, core.list, core.tuple, core.print-variable, ctrl.if, ctrl.for-each, ctrl.nested-loop, ctrl.comparison]
concepts: [logic.chung-minh-truc-tiep, logic.dat-ten-cho-bat-ky, logic.chi-dung-gia-thiet]
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
Hai số chẵn bất kỳ. Mình cứ định viết 6 với 10, rồi lại thôi — vì đề có nói 6 với 10 đâu.
::::

::::explain{#bat-ky-viet-ra-la-so-nao}
Bài trước để lại đúng chỗ Byte đang đứng.

Bạn có `n = 2k`. Bạn muốn nói: *tổng hai số chẵn bất kỳ là một số chẵn*. Và hai
lối đi rõ nhất đều bịt:

- **Viết ra hai số cụ thể.** 6 và 10 chẳng hạn. 6 + 10 = 16, mà 16 = 2 × 8 nên
  16 chẵn. Đúng — nhưng bạn vừa nói về đúng 6 và 10. Thêm một trăm cặp nữa cũng
  vẫn là một trăm cặp. Bài 21 đã đóng cửa ấy.
- **Không viết số nào.** Thì chẳng có gì để cộng.

Cửa thứ ba nghe hơi kỳ lúc mới nghe, và nó là cả bài hôm nay:

> **Đặt tên** cho hai số ấy.

Gọi chúng là `a` và `b`.

Chữ `a` ở đây không phải một con số. Nó là **cái tên bạn đặt cho một số bất kỳ
thoả giả thiết** — ai đưa số chẵn nào cũng được, `a` nhận hết. Chữ làm tên cho
một chỗ chưa biết thì T2.2 đã dựng suốt cả track; cái mới nằm ở chỗ bạn được
phép làm gì với cái tên ấy sau đó.

Và luật đi kèm là luật nghiêm nhất của cả nửa sau track này:

> Từ lúc đặt tên xong, bạn **chỉ được dùng đúng những gì giả thiết cho**.

Không được giữa chừng nói "à mà `a` chắc lớn hơn 4". Không được nói "`a` khác
`b`". Không được nói "`a` chia hết cho 4". Giả thiết chỉ cho đúng một điều —
`a` chẵn — và mọi thứ bạn dùng thêm đều là một điều **riêng của một số nào đó**,
mà số nào đó thì không phải mọi số.
::::

::::example{#ba-nhip}
Đây là mệnh đề cần chứng minh, viết cho gọn ghẽ:

> Cho `a` và `b` là hai số nguyên. **Nếu** `a` chẵn và `b` chẵn **thì** `a + b`
> chẵn.

Nó có dạng "nếu… thì" của bài 10, nên việc phải làm rõ ràng: nhận lấy vế trước
làm giả thiết, và đi tới vế sau.

**Nhịp 1 — đặt tên, rồi mở định nghĩa ra.**

Cho `a`, `b` là hai số nguyên bất kỳ, và giả sử `a` chẵn, `b` chẵn.

- `a` chẵn, nên theo bài 22 có một số nguyên `p` sao cho `a = 2p`.
- `b` chẵn, nên có một số nguyên `q` sao cho `b = 2q`.

**Hai chữ khác nhau: `p` và `q`.** Chỗ này là chỗ trượt nhiều nhất, nên dừng lại
một nhịp. Nếu viết cả hai thành `2k` thì bạn vừa lặng lẽ thêm một giả thiết
không ai cho: `a = b`. Định nghĩa hứa "có **một** số nguyên" cho mỗi số, và hai
lời hứa riêng thì hai cái tên riêng. Số bàn của `a` đâu buộc phải bằng số bàn
của `b`.

**Nhịp 2 — biến đổi, chỉ bằng luật đã có.**

```text
a + b = 2p + 2q
      = 2 × (p + q)
```

Bước gom ấy là luật phân phối của T2.1 đọc ngược — đúng cái phép mà T2.2 dùng để
nhận ra hai cách gõ tiền cùng ra một số. Không có gì mới ở đây, và đó là điều
đáng mừng: mỗi bước trong một chứng minh phải là một bước ai cũng kiểm được.

**Nhịp 3 — gói định nghĩa lại.**

Đặt `m = p + q`. `p` và `q` đều là số nguyên, mà tổng hai số nguyên là số
nguyên, nên `m` là số nguyên. Vậy

```text
a + b = 2m,  với m là số nguyên
```

và đó **đúng là dạng** mà chiều gói lại của bài 22 nhận: một số viết được thành
2 nhân một số nguyên thì số ấy chẵn.

Nên `a + b` chẵn. Hết.
::::

::::explain{#vi-sao-mot-lan-la-du}
Giờ làm một việc nhỏ mà đáng: đọc lại toàn bộ lập luận trên và tìm xem có dòng
nào nhắc tới một con số cụ thể không.

Không dòng nào. Từ đầu tới cuối chỉ có `a`, `b`, `p`, `q`, `m` — toàn tên.

Nên ai đưa ra một cặp số chẵn bất kỳ, bạn chép đúng bằng ấy dòng, thay tên bằng
số của họ, và lập luận vẫn chạy. Đưa 6 và 10 thì p = 3, q = 5, m = 8, và
2 × 8 = 16. Đưa 0 và 1000 thì p = 0, q = 500, m = 500. Không cặp nào cần một
dòng riêng.

Đó là **chứng minh trực tiếp**:

> Đặt tên cho một đối tượng **bất kỳ** thoả giả thiết; chỉ dùng giả thiết, định
> nghĩa, và những điều đã chứng minh trước đó; đi thẳng tới kết luận. Vì lập
> luận không dùng gì **riêng** của đối tượng ấy, nó đúng cho **mọi** đối tượng
> thoả giả thiết.

Đặt cạnh cái máy của bài 21 thì thấy ngay chỗ khác nhau, và nó không nhỏ: máy
chạy một nghìn trường hợp thì nói được về một nghìn trường hợp. Lập luận trên
chạy **một lần** và nói về tất cả.

Chỗ hỏng của lối này cũng lộ ra từ chính luật của nó. Giả sử giữa chừng bạn lỡ
viết: *"a chẵn nên a chia cho 4 được số nguyên"*. Câu ấy không phải giả thiết,
nó là một điều riêng của vài số chẵn. Lập luận tiếp theo sẽ chỉ còn đúng cho
những số chia hết cho 4 — và máy tìm ra chỗ hỏng ngay, vì bài 19 đã dạy: một
phản ví dụ là đủ, và tìm nó là đúng việc máy làm được.
::::

::::predict{#doan-san-hai-cau commitOnce}
Byte thả máy đi săn phản ví dụ cho hai câu, trên cùng một danh sách số chẵn.
Câu thứ nhất là câu vừa chứng minh. Câu thứ hai là câu "lỡ viết" ở đoạn trên,
phát biểu đầy đủ: *tổng hai số chẵn chia hết cho 4*.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
so_chan = [0, 2, 4, 6, 8, 10]

def san_cap_pha(danh_sach, chia_cho):
    for a in danh_sach:
        for b in danh_sach:
            if (a + b) % chia_cho != 0:
                return (a, b)
    return None

print(san_cap_pha(so_chan, 2))
print(san_cap_pha(so_chan, 4))
```

:::opt{correct}
`None`, rồi `(0, 2)`
:::

:::opt
`None`, rồi `None`
::why
Gần đúng ở dòng đầu, và ở chỗ bạn nghĩ "hai số chẵn cộng lại thì chẵn hai lần,
nên chắc chia hết cho 4". Lối nghĩ ấy có gốc thật: cả hai số đều mang sẵn một
thừa số 2.

Chỗ lệch nằm ở chỗ hai thừa số 2 ấy **không cộng lại thành 4**. Viết ra thì thấy:
a = 2p, b = 2q, nên a + b = 2 × (p + q). Muốn con số ấy chia hết cho 4 thì cần
thêm một điều nữa — `p + q` phải chẵn — mà giả thiết không hứa gì về `p + q` cả.
Lấy p = 1 và q = 2, tức a = 2 và b = 4: 2 + 4 = 6, và 6 không chia hết cho 4.

Một cặp là đủ để bác bỏ, đúng như bài 19 đã chốt.
::
:::

:::opt
`None`, rồi `(0, 0)`
::why
Gần đúng ở chỗ bạn theo đúng đường đi của máy: nó xét cặp (0, 0) trước tiên, nên
nếu cặp ấy phá câu thì nó dừng ngay ở đó. Cách đọc vòng lồng vòng của bạn là
đúng.

Chỗ lệch ở con số 0. Tổng của cặp ấy là 0, và 0 = 4 × 0 nên 0 **chia hết** cho
4 — phần dư bằng 0, không có gì thừa ra. Cặp (0, 0) không phá câu, nên máy đi
tiếp. Cặp thứ hai nó gặp là (0, 2), tổng bằng 2, phần dư khi chia cho 4 là 2 —
và đó mới là chỗ nó dừng.
::
:::

:::opt
`(0, 0)`, rồi `(0, 0)`
::why
Gần đúng ở chỗ bạn nhất quán: nếu 0 làm hỏng câu thứ hai thì nó cũng phải làm
hỏng câu thứ nhất. Sự nhất quán ấy đáng giữ.

Chỗ lệch: 0 không làm hỏng câu nào cả. Bài 22 đã chốt 0 = 2 × 0 nên 0 chẵn, và
`0 % 2` bằng 0. Dòng đầu ra `None` — máy đi hết 36 cặp mà không gặp cặp nào có
tổng lẻ.

Nhớ đọc chữ `None` ấy cho đúng liều lượng, theo bài 21: nó nói "trong 36 cặp
vừa xét, không cặp nào phá câu". Nó không nói "câu ấy đúng". Chỗ nói được câu ấy
đúng là ba nhịp bạn vừa viết bằng tay, không phải màn hình này.
::
:::
::::

::::code{#viet-hai-manh-cua-chung-minh}
Bắt máy giữ hộ hai mẩu của việc vừa làm — một mẩu là dòng cốt lõi trong chứng
minh, một mẩu là thợ săn phản ví dụ.

Nhắc lại vai của máy ở đây, vì nó không đổi từ bài 21: nó **không** chứng minh
gì. Nó đối chiếu một dòng đại số trên những con số cụ thể, và nó đi tìm chỗ
hỏng. Câu "chưa thấy hỏng" là câu duy nhất nó nói được.

Hai chỗ trống:

1. **`gop_hai_ve`** — vế phải của dòng cốt lõi. Từ `a = 2p` và `b = 2q` suy ra
   `a + b = 2 × (p + q)`; hàm này nhận `p`, `q` và trả về đúng con số ở vế phải.
2. **`cap_pha_cau`** — câu hỏi Đ/S "cặp `(a, b)` này có phá câu không", tức tổng
   của chúng **không** chẵn.

Bài chấm bằng nhiều cặp `p`, `q` khác nhau và bằng **hai danh sách khác nhau**:
một lượt săn trên toàn số chẵn (không có cặp nào phá câu) và một lượt săn trên
số lẻ trộn số chẵn (có ngay). Gõ cứng một con số hay một giá trị Đ/S thì một
trong hai lượt ấy sai ngay.

```python title=starter
so_chan = [0, 2, 4, 6, 8, 10, 12]
so_le = [1, 3, 5, 7, 9, 11, 13]

# Dòng cốt lõi: a = 2p và b = 2q  ⟹  a + b = 2 × (p + q).
def gop_hai_ve(p, q):
    return ___

# "Cặp này có phá câu không?" — tổng của chúng KHÔNG chẵn.
def cap_pha_cau(a, b):
    return ___

def san(danh_sach_a, danh_sach_b):
    for a in danh_sach_a:
        for b in danh_sach_b:
            if cap_pha_cau(a, b):
                return (a, b)
    return None

print(gop_hai_ve(3, 5))
print(san(so_chan, so_chan))
print(san(so_le, so_chan))
```

```python title=solution
so_chan = [0, 2, 4, 6, 8, 10, 12]
so_le = [1, 3, 5, 7, 9, 11, 13]

# Dòng cốt lõi: a = 2p và b = 2q  ⟹  a + b = 2 × (p + q).
def gop_hai_ve(p, q):
    return 2 * (p + q)

# "Cặp này có phá câu không?" — tổng của chúng KHÔNG chẵn.
def cap_pha_cau(a, b):
    return (a + b) % 2 != 0

def san(danh_sach_a, danh_sach_b):
    for a in danh_sach_a:
        for b in danh_sach_b:
            if cap_pha_cau(a, b):
                return (a, b)
    return None

print(gop_hai_ve(3, 5))
print(san(so_chan, so_chan))
print(san(so_le, so_chan))
```

```python title=test
# Ba câu đầu canh ba cái bẫy lớn nhất, nên chúng chạy TRƯỚC — một câu `==` ở
# dưới trượt trước thì ba bẫy này không bao giờ sập.
#
#   · bẫy 1 — thợ săn trả `None` vì nó không bao giờ tìm thấy gì, chứ không
#     phải vì câu đúng. Lượt săn thứ hai phải TÌM RA một cặp.
#   · bẫy 2 — thợ săn trả bừa một cặp: lượt săn trên toàn số chẵn phải `None`.
#   · bẫy 3 — nhớ con số 16 thay vì viết ra phép gom.
assert san(so_chan, so_chan) is None, "trong bảy số chẵn của danh sách, không cặp nào có tổng lẻ — máy chỉ nói được về bảy số ấy, không hơn"
assert san(so_le, so_chan) == (1, 0), "một số lẻ cộng một số chẵn thì tổng lẻ, nên cặp đầu tiên máy gặp — 1 và 0 — đã phá câu"
assert gop_hai_ve(3, 5) == 2 * 3 + 2 * 5, "vế phải 2 × (p + q) phải bằng đúng vế trái 2p + 2q ở p = 3, q = 5 — đó chính là bước gom của nhịp 2"
assert gop_hai_ve(3, 5) == 16, "p = 3 và q = 5, tổng của chúng là 8, mà 2 × 8 = 16"
assert gop_hai_ve(0, 0) == 0, "p = 0 và q = 0 thì tổng là 0, và 2 nhân 0 vẫn là 0"
assert gop_hai_ve(1, 0) == 2, "p = 1 và q = 0 thì tổng là 1, và 2 nhân 1 bằng 2"
assert gop_hai_ve(2, 3) == 10, "p = 2 và q = 3 thì tổng là 5, và 2 nhân 5 bằng 10"
assert gop_hai_ve(4, 1) == gop_hai_ve(1, 4), "p và q chỉ đi vào đúng một phép cộng, nên đổi chỗ hai số ấy không đổi kết quả"
assert cap_pha_cau(2, 3) == True, "2 + 3 bằng 5, mà 5 lẻ, nên cặp này phá câu"
assert cap_pha_cau(2, 4) == False, "2 + 4 bằng 6, mà 6 chẵn, nên cặp này không phá câu"
assert cap_pha_cau(0, 0) == False, "0 + 0 bằng 0, mà bài 22 đã chốt 0 chẵn, nên cặp này không phá câu"
assert san(so_le, so_le) is None, "1 + 1 bằng 2, 1 + 3 bằng 4, 3 + 3 bằng 6 — trong bảy số lẻ ấy không cặp nào có tổng lẻ"
assert san([], so_chan) is None, "danh sách rỗng thì không có cặp nào để xét, nên không có cặp nào phá câu — đúng chỗ bài 19 đã dựng"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất phải trả về một CON SỐ, và chú thích ngay trên hàm đã viết sẵn vế phải cần trả về — để ý cặp ngoặc trong đó, vì `p` và `q` phải cộng xong rồi mới nhân. Chỗ trống thứ hai phải trả về Đ/S, và nó hỏi tổng của hai số có KHÔNG chẵn hay không, tức hỏi ngược lại với câu hỏi "có chẵn không" của bài 22.
- kind: strategy
  body: "Vế phải của dòng cốt lõi đọc thành lời là: cộng `p` với `q` trước, rồi nhân kết quả với 2 — nên cặp ngoặc là bắt buộc, `2 * p + q` là một câu tính khác hẳn. Còn câu hỏi thứ hai: bài 22 viết \"chẵn\" thành `% 2` bằng 0, nên \"không chẵn\" chỉ việc lật đúng dấu so sánh ấy, và nhớ cộng hai số lại trước khi lấy phần dư."
- kind: one-line
  body: "Hai chỗ lần lượt là `2 * (p + q)` và `(a + b) % 2 != 0`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: hai chỗ trống phải viết ra phép tính thật theo `p`, `q` và theo `a`, `b` — gõ cứng một con số hay một giá trị Đ/S thì bước gom của chứng minh không được đối chiếu ở đâu cả
  requireAst:
  # Khung khởi đầu không có dấu nhân nào (chú thích không nằm trong cây cú
  # pháp), nên `min` chặn mọi đáp án không viết ra phép nhân với 2.
  #
  # `max: 1` mới là chỗ đáng nói. Không có nó thì `return 2 * p + 2 * q` đậu
  # sạch bốn nấc — mà đó đúng là VẾ TRÁI của dòng cốt lõi, tức bước GOM chưa
  # làm, tức thứ duy nhất bài này sinh ra để giữ. Hai cách viết bằng nhau ở
  # mọi `p`, `q` nên không assert nào và không tier output nào phân biệt nổi;
  # chỉ đếm dấu nhân mới thấy: dạng đã gom có 1, dạng chưa gom có 2. Đây đúng
  # lớp lỗi mà T2.2 bài 8 đã phải thêm trường `max` để xử.
  #
  # Đã thử: `2 * (p + q)` và `(p + q) * 2` đều QUA, `2 * p + 2 * q` TRƯỢT.
  - kind: uses-operator, target: '*', min: 1, max: 1
  # Khung khởi đầu không có dấu `%` nào. Luật này chặn `return True`,
  # `return a > b`, và mọi câu hỏi không hỏi tới phần dư.
  - kind: uses-operator, target: '%', min: 1
  # Vế phải phải đọc CẢ `p` lẫn `q`. Khung khởi đầu đọc mỗi tên 0 lần.
  - kind: uses-name, target: p, min: 1
  - kind: uses-name, target: q, min: 1
  # Khung khởi đầu đã đọc `a` và `b` mỗi tên 2 lần (trong lời gọi
  # `cap_pha_cau(a, b)` và trong câu `return (a, b)`). Lời giải đọc 3 — chỗ
  # thứ ba nằm đúng trong phép cộng hai số.
  - kind: uses-name, target: a, min: 3
  - kind: uses-name, target: b, min: 3
  forbidAst:
  # Lưới thứ hai, chặn đúng con số là KẾT QUẢ chứ không phải dữ liệu: 16 là giá
  # trị của vế phải tại p = 3, q = 5. Mọi cách viết hợp lệ đều dựng từ `p`,
  # `q`, `a`, `b` và số 2, nên không cách nào chứa nguyên văn con số ấy. (Số 8
  # thì KHÔNG chặn được: nó nằm sẵn trong danh sách `so_chan` của khung.)
  - kind: has-literal, target: 16
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^16\nNone\n\(1, 0\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một lập luận, không con số nào trong đó, mà nó phủ hết. Mình thích cái tỉ lệ này.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đem cái khuôn ba nhịp vừa dựng sang một câu khác, cũng dạng "nếu… thì", cũng chỉ
nói về chẵn lẻ:

> Cho `n` là số nguyên. **Nếu** `n²` chẵn **thì** `n` chẵn.

Nhịp 1 chạy trơn: `n²` chẵn, nên có số nguyên `k` sao cho `n² = 2k`.

Rồi nhịp 2 tắc. Bạn cần đi tới `n = 2m` với `m` nào đó là số nguyên, mà trong
tay chỉ có `n² = 2k` — từ đó không nặn ra được dạng của `n`. Cái `k` ấy nói về
`n²`, không nói gì về `n`.

Đừng bỏ. Thử **lật câu hỏi**: nếu `n` **không** chẵn thì `n²` ra sao?

Câu lật ấy có phải câu gốc không — bảng để kiểm thì bạn đã dựng ở bài 14 rồi.

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
