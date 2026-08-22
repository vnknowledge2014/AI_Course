---
id: toan.cam-nhan-so.so-khong-giu-cho
title: Số 0 giữ chỗ
summary: Trong một dãy chữ số, `0` nhận thêm một việc nữa — nói rằng cột này rỗng, và nhờ nó mà 307 khác hẳn 37.
locale: vi
track: toan
module: cam-nhan-so
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.zero-placeholder]
requires: [math.place-value, math.dong-goi, core.arithmetic, core.number-literal, core.variable, core.assignment, core.print-variable, core.fstring, core.string-literal]
concepts: [math.don-vi, math.bang-cot]
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
Cột giữa rỗng vẫn phải có người đứng, không thì mấy cột bên trái tụt hết chỗ.
::::

::::explain{#o-rong-van-phai-co-mat}
Bài trước để lại đúng cái ca khó: Byte buộc bó xong, được **3 bó-của-bó, không
bó lẻ nào, 7 hạt**. Hai cột có hàng, một cột rỗng.

| bó-của-bó | bó | hạt lẻ |
|---|---|---|
| 3 | *(rỗng)* | 7 |

Thử bỏ trống ô giữa rồi viết liền: `37`. Người đọc lại tờ giấy ấy sẽ đếm cột từ
phải sang, đúng luật bài 7: 7 ở cột hạt lẻ, 3 ở cột bó. Ra ba mươi bảy hạt —
trong khi đống thật của Byte có ba trăm lẻ bảy hạt. Muốn đống 37 hạt đầy lên
bằng đống thật thì phải đổ thêm **270 hạt** nữa.

Chuyện gì vừa xảy ra? Bỏ trống một ô **không** làm mất một ô. Nó làm mọi chữ số
bên trái **tụt xuống một cột**: chữ số 3 đang đếm bó-của-bó bỗng bị đọc thành
đếm bó. Cả hàng ô chỉ chạy được khi đếm từ phải sang mà không hụt ô nào — mà tờ
giấy thì không giữ nổi một ô trống, vì chỗ trống nào trên giấy trông cũng như
chỗ trống nào.

Nên phải cắm vào ô rỗng một cái mốc, một ký hiệu nói rõ: *ô này có mặt, và nó
rỗng*. Ký hiệu ấy là `0`.

| bó-của-bó | bó | hạt lẻ |
|---|---|---|
| 3 | 0 | 7 |

Viết liền: `307`. Ba ô, ba chữ số, đọc lại chỉ ra đúng một đống.
::::

::::explain{#hai-viec-cua-so-khong}
Chỗ này cần tách bạch, vì `0` đang làm **hai** việc và người ta hay trộn chúng
vào nhau.

- **Việc cũ, vẫn đúng như bạn vẫn hiểu.** Khi trả lời câu hỏi "mấy bó?", `0`
  nghĩa là không có cái bó nào. Nó là một câu trả lời thật, và nó đếm đúng.
- **Việc mới, chỉ có khi nó đứng trong một dãy chữ số.** Lúc ấy `0` còn giữ chỗ
  cho một cột rỗng, để những cột bên trái nó không bị tụt xuống. Đây mới là
  điều bài này thêm vào.

Tiếng Việt có hẳn một từ riêng cho việc thứ hai. Đọc `307` là "ba trăm **linh**
bảy" (hoặc "ba trăm **lẻ** bảy"). Chữ "linh" ấy không đếm gì cả — nó chỉ báo
rằng cột chục đã bị nhảy qua. Chữ số `0` làm đúng việc của chữ "linh", chỉ
ngắn hơn.

Một hệ quả đi kèm, và nó cho thấy `0` giữ chỗ **cho ai**: cột rỗng nằm ngoài
cùng bên trái thì không cần giữ chỗ. Chẳng ai viết `037`, vì bên trái con số
`0` ấy không còn cột nào để đỡ. `0` chỉ có việc khi còn thứ gì đó đứng bên
trái nó.
::::

::::example{#to-giay-va-dong-hat}
Bốn dòng: hai dòng đầu là **tờ giấy**, hai dòng sau là **đống hạt** ứng với
từng tờ giấy ấy.

```python title=readonly
bo_cua_bo = 3
bo = 0
hat = 7

print(f"{bo_cua_bo}{bo}{hat}")
print(f"{bo_cua_bo}{hat}")
print(100 + 100 + 100 + 0 + 7)
print(10 + 10 + 10 + 7)
```

Máy in ra:

```text
307
37
307
37
```

Dòng 1 ghi đủ ba ô nên đọc lại ra đúng đống của Byte. Dòng 2 bỏ ô giữa, và tờ
giấy chỉ còn hai ký tự.

Dòng 3 và dòng 4 là hai đống hạt mà hai tờ giấy ấy mô tả. Một bên ba trăm lẻ
bảy hạt, một bên ba mươi bảy hạt. Một ô bị bỏ quên trên giấy làm hụt 270 hạt
ngoài vườn — và không ai nhìn tờ giấy `37` mà biết được là nó đã hụt.
::::

::::predict{#doan-ba-dong commitOnce}
Ba dòng dưới đây trông giống nhau tới mức dễ lẫn, nhưng chúng khác nhau ở chỗ
quan trọng nhất của bài.

- Dòng 1 viết đủ ba cột, kể cả cột rỗng.
- Dòng 2 **bỏ đi khúc `+ 0`** — bỏ một số hạng khỏi phép cộng.
- Dòng 3 **bỏ hẳn cột bó** — đúng cái xảy ra khi tờ giấy viết `37`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
day_du_cot = 100 + 100 + 100 + 0 + 7
bo_khuc_cong_khong = 100 + 100 + 100 + 7
bo_han_cot_bo = 10 + 10 + 10 + 7

print(day_du_cot)
print(bo_khuc_cong_khong)
print(bo_han_cot_bo)
```

:::opt{correct}
307, 307 rồi 37
:::

:::opt
307, 37 rồi 37
::why
Gần đúng ở chỗ bạn nắm được điều nặng ký nhất của bài: bỏ cột rỗng đi thì con
số hỏng, và hỏng nặng. Ý ấy đúng — chỉ là nó hỏng ở **tờ giấy**, chưa hỏng ở
dòng cộng này.

Chỗ lệch: dòng 2 không bỏ cột nào cả, nó chỉ bỏ khúc `+ 0`. Trong một phép
cộng viết ra như thế, mỗi cột **tự khai** cỡ của mình bằng chính con số của nó
— ba cái `100` vẫn đang nói "tôi là ba bó-của-bó", dù khúc `+ 0` có mặt hay
không. Chỗ đứng chỉ gánh việc khai cỡ khi ta viết thành dãy chữ số, và lúc ấy
mới cần `0`.
::
:::

:::opt
307, 307 rồi 307
::why
Gần đúng ở chỗ bạn giữ chắc rằng ba dòng đều đang nói về cùng một đống hạt của
Byte — và với hai dòng đầu thì bạn trúng.

Chỗ lệch nằm ở dòng 3. Nó không cộng cái `100` nào cả; nó cộng ba cái `10`. Đó
đúng là cái xảy ra khi chữ số 3 tụt từ cột bó-của-bó xuống cột bó: mỗi cái nó
đếm nhỏ đi mười lần. Ba trăm hạt teo còn ba mươi, và đống hạt trên tờ giấy ấy
không còn là đống của Byte nữa.
::
:::

:::opt
0, 307 rồi 37
::why
Gần đúng ở chỗ bạn nhớ chắc một luật không bao giờ sai: dính tới số 0 thì kết
quả hay bị kéo về 0.

Chỗ lệch là phạm vi của luật ấy — nó đúng với phép **nhân**, chứ không đúng với
phép cộng. Ở đây `0` chỉ là phần góp của một cột rỗng: cột bó có 0 cái bó, nên
nó đóng vào đống đúng 0 hạt. Cộng thêm 0 hạt vào một đống thì đống ấy đứng
nguyên, không hạt nào mất đi.
::
:::
::::

::::code{#ghi-dong-co-cot-rong}
Byte ghi lại đống hạt có cột rỗng ấy, bằng cả hai cách: một tờ giấy và một con
số hạt.

Hai chỗ trống hỏi hai thứ khác hẳn nhau, nên không chỗ nào chép được của chỗ
kia:

1. `day_chu_so` — **tờ giấy**: mỗi cột đúng một chữ số, ghép liền lại thành một
   dãy chữ.
2. `so_hat` — **đống hạt**: cả đống ấy là mấy hạt, ráp từ các cột (một bó-của-bó
   là 100 hạt, một bó là 10 hạt).

```python title=starter
# 3 bó-của-bó, KHÔNG bó lẻ nào, 7 hạt lẻ
bo_cua_bo = 3
bo = 0
hat = 7

# 1) Tờ giấy: ghép chữ số của cả ba cột lại, từ trái sang phải.
day_chu_so = ___

# 2) Đống hạt: cộng phần của từng cột lại.
so_hat = ___

print(day_chu_so)
print(so_hat)
```

```python title=solution
# 3 bó-của-bó, KHÔNG bó lẻ nào, 7 hạt lẻ
bo_cua_bo = 3
bo = 0
hat = 7

# 1) Tờ giấy: ghép chữ số của cả ba cột lại, từ trái sang phải.
day_chu_so = f"{bo_cua_bo}{bo}{hat}"

# 2) Đống hạt: cộng phần của từng cột lại.
so_hat = 100 + 100 + 100 + 0 + 7

print(day_chu_so)
print(so_hat)
```

```python title=test
# Bốn câu, hai bộ dữ liệu khác kiểu nhau: một dãy chữ và một con số hạt. Hai
# câu `!=` chốt lại đúng cái bẫy của bài — bỏ ô rỗng thì cả tờ giấy lẫn đống
# hạt đều rơi về `37`, và không câu nào trong bốn câu này còn đạt.
assert day_chu_so == "307", "ba cột thì ba chữ số — cột bó rỗng vẫn phải có `0` đứng giữ chỗ"
assert day_chu_so != "37", "bỏ ô rỗng đi thì chữ số 3 tụt xuống cột bó, và tờ giấy nói về một đống khác"
assert so_hat == 307, "3 bó-của-bó là 300 hạt, cột bó góp 0 hạt, thêm 7 hạt lẻ nữa"
assert so_hat == 37 + 270, "cái ô rỗng bị bỏ quên đáng đúng 270 hạt"
```

:::hints
- kind: attention
  body: Đống này có BA cột chứ không phải hai: cột bó-của-bó, cột bó, cột hạt lẻ. Cột giữa rỗng, nhưng nó vẫn nằm trong hàng và vẫn phải xuất hiện ở cả hai chỗ trống.
- kind: strategy
  body: Chỗ trống thứ nhất ghép ba CHỮ SỐ lại thành một dãy chữ — cách chèn giá trị vào giữa một câu chữ thì Realm 0 đã dạy, dùng chữ f đứng trước dấu nháy. Chỗ trống thứ hai cộng phần HẠT của từng cột lại: mỗi bó-của-bó góp 100, mỗi bó góp 10, cột rỗng góp 0.
- kind: one-line
  body: Chỗ trống thứ nhất viết f"{bo_cua_bo}{bo}{hat}", chỗ thứ hai viết `100 + 100 + 100 + 0 + 7`.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: tờ giấy phải được ghép từ chữ số của cả ba cột (kể cả cột rỗng), chứ không phải chép sẵn dãy `307` vào
  requireAst:
  # Ba luật này chặn ba kiểu đi tắt khác nhau: chép cứng chuỗi "307" (không có
  # f-string), ghép mà bỏ qua cột rỗng (không đọc tên `bo`), và chép cứng con
  # số 307 (không có phép cộng nào).
  - kind: uses-fstring, min: 1
  - kind: uses-name, target: bo, min: 1
  - kind: uses-operator, target: +, min: 3
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^307\n307\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
307 chứ không phải 37. Cái ô rỗng ấy đáng đúng 270 hạt đấy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ba bài liền vừa rồi đứng trên đúng một luật, và luật ấy chưa bao giờ được giải
thích: **đủ mười thì lên bó**. Mười hạt thành một bó, mười bó thành một
bó-của-bó, và mỗi cột chỉ chứa tới chín.

Vì sao lại là mười?

Thử tìm con số mười trong đống hạt xem. Đống hạt chỉ có chừng ấy hạt, không hơn
không kém; nó chưa bao giờ nói phải gom thành từng nhóm mấy. Vậy con số mười ấy
từ đâu ra — ai chọn nó?

Và nếu là chọn, thì chọn số khác — chọn năm chẳng hạn — có được không? Đống hạt
lúc ấy còn nguyên chừng ấy không? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
