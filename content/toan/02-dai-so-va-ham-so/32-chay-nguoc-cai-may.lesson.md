---
id: toan.dai-so-va-ham-so.chay-nguoc-cai-may
title: Chạy ngược cái máy
summary: Máy ngược nhận đầu ra rồi trả lại đúng đầu vào đã sinh ra nó — dựng nó bằng cách gỡ từng việc của máy xuôi theo thứ tự ngược lại.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 32
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.inverse-function]
requires: [math.exponential-beats-linear, math.function-notation, math.equation-add-both-sides, math.equation-multiply-both-sides, math.division-partitive, math.additive-inverse, math.order-of-operations, math.parentheses, core.function-def, core.function-parameter, core.function-return, core.function-call, core.float, core.division, core.variable, core.arithmetic, core.print-variable]
concepts: [math.may-nguoc, math.go-nguoc-thu-tu, math.vong-xuoi-nguoc]
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
Ngón tay mình dò được cột kết quả. Nhưng ngón tay đâu phải cái máy.
::::

::::explain{#cau-hoi-con-treo}
Bài trước kết bằng một việc Byte làm rất tự nhiên mà chưa cái máy nào trong tủ
làm được: đặt ngón tay lên **cột kết quả**, dò tới con số cần, rồi gióng sang
**cột đầu vào** để đọc.

Mọi cái máy từ bài 25 tới giờ đều chạy đúng một chiều:

```text
   bỏ vào  ──►  [ máy ]  ──►  nhả ra
   số giờ                     số phần men
   số ổ                       số tiền
```

Nhưng câu hỏi ở quán thường đi ngược mũi tên. *Trong két có 350 nghìn thì đã bán
mấy ổ?* *Muốn có 1000 phần men thì ủ từ mấy giờ?* Người hỏi cầm sẵn **đầu ra**
và đang đòi lại **đầu vào**.

Thứ ta cần là một cái máy thứ hai, quay đầu:

```text
   bỏ vào  ──►  [ máy ngược ]  ──►  nhả ra
   số tiền                         số ổ
```

Với máy men thì việc này khó — bài trước đã thấy: giờ 9 được 512 phần, giờ 10
được 1024 phần, không giờ tròn nào cho đúng 1000. Bảng không có dòng để dò. Nên
hãy bắt đầu bằng cái máy hiền nhất trong tủ, cái máy đếm tiền, và xem việc "quay
đầu" thật ra là làm gì.
::::

::::example{#coi-giay-truoc-coi-tat-sau}
Cái két của xe bánh mì, một buổi:

- mỗi ổ bán `15000` đồng, tiền bỏ thẳng vào két;
- đầu buổi Byte đã bỏ sẵn `50000` đồng tiền lẻ để thối, nên két chưa bao giờ
  rỗng.

Máy xuôi, viết theo ký hiệu bài 25:

> `ket(n) = 15000 × n + 50000`

Đây đúng là dạng `an + b` của bài 27, và con số `b` ở đây có mặt mũi hẳn hoi:
`ket(0) = 50000` — bán không ổ nào thì trong két vẫn còn nguyên chỗ tiền lẻ.

Nhìn kỹ, máy xuôi làm **hai việc, theo một thứ tự**:

```text
   n  ──►  nhân 15000  ──►  cộng 50000   ──►  ket(n)
```

Muốn quay đầu thì phải gỡ cả hai việc. Và đây là chỗ dễ trượt: gỡ thì **gỡ theo
thứ tự ngược lại**. Sáng ra Byte đi tất rồi mới xỏ giày; tối về muốn cởi tất,
không ai cởi tất trước — phải tháo giày ra đã.

```text
   ket(n)  ──►  trừ 50000  ──►  chia 15000  ──►  n
```

Thử với một buổi có thật. Bán `12` ổ:

- máy xuôi: `15000 × 12 = 180000`, cộng `50000` thành `230000` đồng trong két.
- máy ngược: `230000` trừ `50000` còn `180000`, chia `15000` được `12`.

Đúng con số đã bỏ vào. Đó là toàn bộ việc mà một máy ngược phải làm được: nhận
đầu ra, trả lại **đúng** đầu vào đã sinh ra nó.

```python title=readonly
def ket(n):
    return 15000 * n + 50000

def ket_nguoc(t):
    return (t - 50000) / 15000

print(ket(12))
print(ket_nguoc(230000))
```

Máy in ra:

```text
230000
12.0
```

`12.0` chứ không phải `12`, vì phép chia trong Python luôn trả về số thập phân —
điều bạn đã gặp ở Realm 0. Nó vẫn là mười hai ổ, chỉ mặc áo khác.
::::

::::predict{#doan-hai-ban-nhap commitOnce}
Byte viết hai bản nháp cho máy ngược. Cả hai đều dùng đúng hai phép **trừ
50000** và **chia 15000** — chỉ khác nhau ở thứ tự làm.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
tien = 230000

# bản nháp A: chia trước, trừ sau
a = tien / 15000 - 50000

# bản nháp B: trừ trước, chia sau
b = (tien - 50000) / 15000

print(a)
print(b)
```

:::opt{correct}
`-49984.666666666664` rồi `12.0`
:::

:::opt
`12.0` rồi `12.0` — hai bản nháp chỉ khác chỗ đặt ngoặc, kết quả phải như nhau
::why
Gần đúng ở chỗ bạn đang dùng một quy tắc thật, và T2.1 bài 43 đã dựng nó cẩn
thận: trong một chuỗi **toàn cộng và trừ**, hay một chuỗi **toàn nhân và chia**,
cặp ngoặc đặt theo đúng chiều trái-sang-phải không đổi kết quả — nó chỉ viết ra
cái thứ tự vốn đã có sẵn.

Ranh giới nằm ở hai chữ *toàn*. Chuỗi ở đây **trộn hai họ**: một phép chia và
một phép trừ. Bài 42 của T2.1 đã cho thấy đúng lúc hai họ gặp nhau thì thứ tự
quyết định kết quả, và cặp ngoặc thôi làm trang trí — nó đổi hẳn việc nào chạy
trước. Bản A chia `230000` cho `15000` được `15,33` rồi mới lấy con số bé xíu ấy
trừ đi `50000`. Bản B trừ trên khoản tiền, đúng chỗ `50000` đồng tiền lẻ thật
sự nằm.
::
:::

:::opt
Máy báo lỗi ở bản nháp A, vì kết quả ra số ổ âm — không ai bán được âm ổ bánh mì
::why
Gần đúng ở chỗ bạn nhận ra một điều rất đáng nhận ra: `-49984` **không thể** là
số ổ bánh mì. Đọc ra sự vô lý ấy là việc của người làm toán, và bạn làm đúng.

Ranh giới: cái máy không biết gì về bánh mì. Nó nhận số, làm hai phép tính, trả
về số. `-49984.666666666664` là một con số hoàn toàn hợp lệ — nó chỉ vô lý khi
bạn dán lại cái nhãn "ổ" lên nó. Nên máy không báo lỗi; nó lặng lẽ đưa kết quả,
và **sự vô lý là tín hiệu dành cho bạn**, không phải sự cố của nó. Bài 28 đã tách
bạch đúng chỗ này khi nói máy `n × n` nhận cả số âm còn mảnh sân thì không — và
bài sau sẽ sống nhờ đúng chỗ tách bạch ấy.
::
:::

:::opt
Máy báo lỗi, vì `tien / 15000` ra số thập phân mà `50000` là số nguyên — hai
kiểu khác nhau thì trừ nhau không được
::why
Gần đúng ở chỗ bạn nhớ rằng Python **có** để ý tới kiểu, và có những phép trộn
kiểu thì nó từ chối thẳng: cộng một chuỗi chữ với một con số là báo lỗi ngay,
Realm 0 đã cho bạn xem cái vết lỗi ấy.

Ranh giới: luật đó áp cho những kiểu **không cùng họ**, như chữ với số. Số
nguyên và số thập phân thì cùng họ số, và Python tự nâng số nguyên thành số thập
phân rồi tính tiếp — cũng chính là cái bạn đã thấy khi một phép chia làm cả biểu
thức hoá thập phân. `15.33 - 50000` chạy trơn tru, chỉ ra một con số vô lý.
::
:::
::::

::::explain{#dat-ten}
Đặt tên cho thứ vừa dựng:

> **Máy ngược** của `f` là cái máy nhận **đầu ra** của `f` rồi trả lại **đúng
> đầu vào** đã sinh ra đầu ra ấy. Viết là `f⁻¹`, đọc là *ép ngược*.

Con số `−1` nhỏ ở trên **không phải số mũ**. Nó là dấu hiệu "quay đầu", giống
một mũi tên vòng lại. `ket⁻¹(230000)` không có nghĩa là chia gì cho `ket(230000)`
cả; nó có nghĩa là *cái số ổ mà máy xuôi biến thành 230000 đồng*.

Dựng máy ngược thì làm đúng hai bước:

1. Kể ra máy xuôi làm những việc gì, theo thứ tự.
2. Gỡ từng việc bằng việc ngược của nó, **đi từ việc cuối về việc đầu**.

Với `ket(n) = 15000n + 50000`:

> `ket⁻¹(t) = (t − 50000) ÷ 15000`

Và bây giờ tới chỗ trả nợ. Bài 13–15 đã dạy bạn giải một phương trình bậc nhất:
muốn biết bán mấy ổ thì két có `350000`, bạn viết `15000n + 50000 = 350000`,
trừ cả hai vế đi `50000`, rồi chia cả hai vế cho `15000`, ra `n = 20`.

Đọc lại hai bước ấy mà xem: **trừ 50000, rồi chia 15000**. Đó chính là máy
ngược, từng chữ một.

> Giải một phương trình bậc nhất là chạy ngược cái máy tại **một** điểm. Máy
> ngược là chính lời giải ấy, đóng gói lại để dùng cho **mọi** điểm.

Byte không phải giải lại từ đầu mỗi lần số tiền trong két đổi. Byte giải một
lần, cất cái máy ngược vào tủ, rồi bỏ khoản tiền nào vào cũng được.
::::

::::code{#dung-may-nguoc}
Dựng máy ngược của cái két, cho nó chạy trên hai buổi, rồi kiểm nó bằng một vòng
xuôi–ngược.

- **Buổi sáng** két có `350000` đồng — đã bán mấy ổ?
- **Buổi chiều** két có `185000` đồng — đã bán mấy ổ?
- **Vòng kiểm**: `so_o` đã chạy qua máy xuôi thành `tien_trong_ket`. Cho
  `tien_trong_ket` chạy ngược lại thì phải ra đúng `so_o`.

Hai chỗ trống. Chỗ thứ nhất là **cả cái máy ngược** — viết nó theo chữ `t`, đừng
viết theo một con số cụ thể, vì nó còn phải chạy với ba khoản tiền khác nhau.

Bài chấm bằng **cả ba lần chạy**, và ba lần cho ba số ổ khác nhau: `20`, `9`,
`12`. Gõ cứng `20` thì hai lần sau sai. Chỉ một cái máy viết thật mới qua cả ba.

Nhớ thứ tự gỡ: tháo giày trước, cởi tất sau.

```python title=starter
# Máy xuôi: bỏ SỐ Ổ vào, nhả ra SỐ TIỀN TRONG KÉT.
def ket(n):
    return 15000 * n + 50000

# Máy ngược: bỏ SỐ TIỀN vào, đòi lại SỐ Ổ.
# Gỡ 50000 tiền lẻ ra trước, rồi mới chia cho 15000 một ổ.
def ket_nguoc(t):
    return ___

o_sang = ket_nguoc(350000)
o_chieu = ket_nguoc(185000)

# Vòng xuôi rồi ngược trên một buổi có thật.
so_o = 12
tien_trong_ket = ket(so_o)
o_lay_lai = ___

print(o_sang)
print(o_chieu)
print(o_lay_lai)
```

```python title=solution
# Máy xuôi: bỏ SỐ Ổ vào, nhả ra SỐ TIỀN TRONG KÉT.
def ket(n):
    return 15000 * n + 50000

# Máy ngược: bỏ SỐ TIỀN vào, đòi lại SỐ Ổ.
# Gỡ 50000 tiền lẻ ra trước, rồi mới chia cho 15000 một ổ.
def ket_nguoc(t):
    return (t - 50000) / 15000

o_sang = ket_nguoc(350000)
o_chieu = ket_nguoc(185000)

# Vòng xuôi rồi ngược trên một buổi có thật.
so_o = 12
tien_trong_ket = ket(so_o)
o_lay_lai = ket_nguoc(tien_trong_ket)

print(o_sang)
print(o_chieu)
print(o_lay_lai)
```

```python title=test
# Hai câu `!=` đứng TRƯỚC. Chúng canh hai cái bẫy: một cái máy ngược trả về cùng
# một con số cho mọi khoản tiền, và chuyện nhầm máy ngược với việc đưa lại chính
# khoản tiền vừa bỏ vào. Xếp chúng sau các câu `==` thì không bao giờ chạy tới,
# và hai bẫy ấy không bao giờ sập.
assert o_sang != o_chieu, "hai khoản tiền khác nhau thì máy ngược phải trả về hai số ổ khác nhau — bằng nhau nghĩa là nó chẳng nhìn vào `t`"
assert o_lay_lai != tien_trong_ket, "máy ngược nhận tiền nhưng nhả ra SỐ Ổ, chứ không đưa lại chính khoản tiền vừa bỏ vào"
assert o_sang == 20, "két có 350000, gỡ 50000 tiền lẻ còn 300000, chia 15000 một ổ được 20 ổ"
assert o_chieu == 9, "két có 185000, gỡ 50000 còn 135000, chia 15000 được 9 ổ"
assert o_lay_lai == so_o, "bỏ 12 vào máy xuôi rồi cho kết quả chạy ngược thì phải nhận lại đúng 12 — chính điều này làm nó xứng đáng gọi là máy ngược"
assert ket(o_sang) == 350000, "cho số ổ vừa tìm chạy lại máy xuôi thì phải ra đúng khoản tiền ban đầu"
assert ket_nguoc(ket(7)) == 7, "vòng xuôi–ngược phải khít ở MỌI điểm, không riêng gì 12 — thử 7 ổ cũng phải trả về 7"
assert ket_nguoc(ket(0)) == 0, "kể cả buổi ế không bán ổ nào: két còn đúng 50000 tiền lẻ, và máy ngược đọc ra 0 ổ"
```

:::hints
- kind: attention
  body: Đọc lại dòng chú thích ngay trên `def ket_nguoc`. Nó kể ra hai việc phải làm, và kể theo đúng thứ tự — gỡ cái gì trước, chia cho cái gì sau. Chỗ trống cần cả hai việc, không chỉ một. Còn chỗ trống thứ hai thì nhìn hai dòng `o_sang` và `o_chieu`: chúng đã viết sẵn khuôn bạn cần, chỉ khác thứ bỏ vào.
- kind: strategy
  body: Máy xuôi nhân rồi mới cộng, nên máy ngược trừ rồi mới chia. Muốn phép trừ chạy trước phép chia thì phải có một cặp ngoặc ôm lấy nó — không có ngoặc thì Python chia trước, đúng như bước đoán vừa cho thấy. Viết cả cái máy theo chữ `t`, đừng thay `t` bằng một con số. Chỗ trống cuối thì bỏ thẳng cái tên vừa tính được vào máy ngược.
- kind: one-line
  body: "Hai chỗ là `(t - 50000) / 15000` và `ket_nguoc(tien_trong_ket)`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: máy ngược phải gỡ 50000 rồi mới chia 15000, và phải viết theo chữ `t` — gõ thẳng số ổ vào thì máy chẳng tính gì cả, bạn tính hộ nó rồi
  requireAst:
  # Khung khởi đầu không có dấu `/` hay `-` nào, nên hai luật này một mình đã
  # chặn được đáp án chép cứng số ổ.
  - kind: uses-operator, target: /, min: 1
  - kind: uses-operator, target: -, min: 1
  # Máy ngược phải viết theo `t`, không theo một con số. Khung khởi đầu không đọc
  # `t` lần nào — `t` mới chỉ là tên tham số, chưa được dùng.
  - kind: uses-name, target: t, min: 1
  # Chỗ trống cuối phải CHẠY máy ngược, không được gõ 12 hay viết `so_o`. Khung
  # khởi đầu gọi `ket_nguoc` 2 lần; lời giải gọi 3.
  - kind: uses-call, target: ket_nguoc, min: 3
  # ...và phải chạy nó trên đúng khoản tiền vừa sinh ra, chứ không trên một con
  # số khác. Khung khởi đầu chưa ĐỌC `tien_trong_ket` lần nào.
  - kind: uses-name, target: tien_trong_ket, min: 1
  forbidAst:
  # `uses-operator` đếm trên cả file nên không chặn nổi đáp án chép cứng ĐÚNG
  # MỘT chỗ. Bốn luật dưới chặn từng số ổ một. Chúng không đụng tới cách viết
  # hợp lệ nào: `(t - 50000) / 15000` không chứa nguyên văn 20 hay 9.
  - kind: has-literal, target: 20
  - kind: has-literal, target: 9
  - kind: has-literal, target: 20.0
  - kind: has-literal, target: 9.0
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^20\.0\n9\.0\n12\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai mươi ổ, chín ổ, rồi đúng mười hai ổ mình đã bỏ vào. Cái máy này quay đầu
được thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Máy két quay đầu ngon lành: mỗi khoản tiền trả về đúng một số ổ, không lần nào
lưỡng lự.

Bây giờ lôi cái máy ở bài 28 ra — máy đo diện tích mảnh sân hình vuông:

> `san(n) = n × n`

Bài 28 đã tách bạch một chuyện, và nó chốt bằng đúng một dòng kiểm:
`san(-3)` bằng `9`. **Cái máy** `n × n` nhận mọi con số, kể cả số âm, vì
T2.1 đã dựng đủ số âm cho nó; còn **mảnh sân** chỉ là một lần người ta đem cái
máy ấy ra dùng.

Giờ dựng máy ngược cho nó. Bỏ `9` mét vuông vào, đòi lại cạnh.

- `3 × 3 = 9`. Vậy trả về `3`.
- Nhưng `(−3) × (−3)` cũng bằng `9` — hai dấu trừ triệt tiêu nhau. Vậy trả về
  `−3`.

Cả hai đều là câu trả lời thật. Vậy máy ngược phải nhả ra cái nào — `3` hay
`−3`? Mà nếu nó nhả ra **cả hai**, thì bài 24 có còn cho phép gọi nó là máy nữa
không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
