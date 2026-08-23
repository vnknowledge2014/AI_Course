---
id: toan.dai-so-va-ham-so.dien-vao-thi-ra-so
title: Điền vào thì mới ra số
summary: Một câu tính còn chữ chưa phải con số — điền một giá trị vào mọi chỗ mang chữ ấy thì cả câu thu lại thành đúng một số, và đổi giá trị là đổi luôn con số ấy.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.substitution]
requires: [math.expression, math.letter-names-a-slot, math.placeholder-many-values, math.multiplication, math.order-of-operations, math.negative-number, core.variable, core.reassign, core.print-variable, core.arithmetic]
concepts: [math.o-trong, math.bieu-thuc, math.thay-gia-tri]
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
Câu tính đang chờ một con số. Mình đưa cho nó thì chuyện gì xảy ra?
::::

::::explain{#dua-cho-no-mot-so}
Bài trước để lại một câu hỏi: `15000 × n` là một **cách tính đang chờ**. Chờ
cái gì, và nếu Byte đưa cho nó số 20 thì sao?

Đưa cho nó một số nghĩa là làm đúng một việc: **xoá chữ đi, viết con số vào chỗ
chữ vừa đứng.** Không thêm, không bớt, không đổi chỗ gì khác trong câu.

```text
   15000 × n        →        15000 × 20
```

Việc này có tên: **thay giá trị**. Và nó có đúng hai điều kiện, cả hai đều quan
trọng ngang nhau.

**Điều kiện thứ nhất: điền vào MỌI chỗ mang chữ ấy.** Chữ `n` là *tên của một ô
trống*, không phải một tờ vé dùng một lần. Nếu câu tính có ba chỗ viết `n` thì
cả ba chỗ ấy là **cùng một ô**, nên cả ba nhận **cùng một số**. Bỏ sót một chỗ
thì câu vẫn còn ô trống — vẫn chưa ra số.

**Điều kiện thứ hai: điền xong rồi mới tính, và tính theo đúng luật cũ.** Sau
khi thay, trong câu chỉ còn toàn số. Từ giây phút đó trở đi nó là một câu tính
bình thường, và thứ tự phép toán bạn đã có ở T2.1 áp nguyên vào: nhân chia trước,
cộng trừ sau, trong ngoặc trước ngoài ngoặc.

Kết quả: cả câu **thu lại thành đúng một con số**. Không phải hai số, không phải
một câu ngắn hơn — một con số.

Và đây là chỗ đáng nhớ nhất: **thay số khác thì ra số khác.** Cùng một câu tính,
điền 8 ra một số, điền 20 ra một số khác hẳn. Câu tính không đổi. Chỉ thứ bạn
đưa cho nó đổi.
::::

::::example{#dien-vao-hai-cho}
Xe bánh mì của Byte: mỗi ổ bán 15 000 đ, nguyên liệu cho mỗi ổ hết 5 000 đ. Gọi
`n` là số ổ bán được trong buổi. Tiền lãi cả buổi là:

```text
   15000 × n   −   5000 × n
```

Chữ `n` có mặt **hai** chỗ. Byte bán được 20 ổ. Điền vào:

```text
   15000 × n   −   5000 × n      ← câu tính; chữ n là một ô trống
   15000 × 20  −   5000 × 20     ← điền 20 vào MỌI chỗ mang chữ n
      300000   −      100000     ← mỗi cụm thu lại thành một số
              200000             ← cả câu thu lại thành ĐÚNG MỘT số
```

Bốn dòng, và chỉ dòng thứ hai là việc mới của bài này. Dòng ba, dòng bốn là
những thứ bạn đã làm từ T2.1: nhân trước, trừ sau.

Thử điền một số khác. Byte bán được 8 ổ:

```text
   15000 × 8   −   5000 × 8      ← điền 8 vào cả hai chỗ
      120000   −      40000      ← mỗi cụm thu lại thành một số
              80000              ← đúng một con số, khác hẳn 200000
```

Cùng một câu tính, hai lần điền, hai con số: 200000 và 80000. Câu tính
`15000 × n − 5000 × n` **không phải** con số 200000, cũng không phải 80000. Nó
là cái sinh ra cả hai.

Hỏi máy cho chắc. Trong Python, cái ô trống ấy được gọi bằng một cái tên đọc
được — `so_o` — thay vì một chữ cái; vẫn đúng một ô trống ấy, chỉ là tên dài hơn:

```python title=readonly
so_o = 20
print(15000 * so_o - 5000 * so_o)

so_o = 8
print(15000 * so_o - 5000 * so_o)
```

Máy in ra:

```text
200000
80000
```

Dòng `15000 * so_o - 5000 * so_o` được gõ y hệt nhau hai lần. Thứ đổi là con số
nằm trong `so_o` lúc dòng ấy chạy.
::::

::::predict{#doan-hai-cau commitOnce}
Byte viết hai câu tính rất giống nhau. Chúng dùng đúng ba con số `15000`,
`5000`, `20` và đúng một ô trống `so_o`. Khác nhau ở chỗ ô trống ấy được dán
vào đâu.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
so_o = 20
print(15000 * so_o - 5000 * so_o)
print(15000 - 5000 * so_o)
```

:::opt{correct}
200000 rồi -85000
:::

:::opt
200000 rồi 200000
::why
Gần đúng ở chỗ bạn soi kỹ và thấy hai dòng dùng đúng cùng bộ nguyên liệu: ba con
số ấy, ô trống ấy, dấu trừ ấy. Quy tắc bạn đang dùng — *cùng nguyên liệu thì
cùng kết quả* — đúng hoàn toàn khi hai câu chỉ **đổi chỗ** cho nhau, ví dụ
`5000 × 20` với `20 × 5000`.

Chỗ lệch là hai dòng này không đổi chỗ nguyên liệu, chúng đổi **chỗ đứng của ô
trống**. Dòng trên, `so_o` dán vào cả hai cụm nhân, nên trừ xong còn 200000.
Dòng dưới, `so_o` chỉ dán vào cụm sau; số 15000 đứng trơ một mình, không được
nhân với gì cả. Ô trống nằm ở đâu là một phần của câu tính, không phải chi tiết
phụ.
::
:::

:::opt
295000 rồi -85000
::why
Gần đúng ở chỗ bạn thay 20 vào đúng chỗ đầu tiên và tính `15000 × 20` ra 300000
không sai một li. Và quy tắc bạn dùng cho chỗ thứ hai — *dùng rồi thì thôi* —
đúng với những thứ tiêu đi một lần: một tờ vé, một hạt giống đã gieo.

Chỗ lệch là `so_o` không phải một tờ vé. Nó là **tên của một ô trống**, và mọi
chỗ trong câu mang cái tên ấy đều là cùng một ô. Điền 20 vào ô đó thì cả hai chỗ
cùng thành 20, nên cụm sau là `5000 × 20`, ra 100000 chứ không phải 5000. Bỏ sót
một chỗ thì câu vẫn còn ô trống — vẫn chưa ra số.
::
:::

:::opt
10000 rồi -85000
::why
Gần đúng, và đây là chỗ nhìn sắc: bạn thấy hai cụm cùng mang một ô trống, nên
gom phần số lại trước — `15000 − 5000` ra `10000`. Phần gom ấy **đúng**, và mấy
bài nữa nó sẽ có tên hẳn hoi.

Chỗ lệch nằm ở bước cuối cùng: gom xong thì câu tính còn là `10000 × so_o`, chứ
không phải `10000`. Cái ô trống không biến mất khi bạn gom phần số — nó vẫn đứng
đó chờ được điền. Điền 20 vào thì mới ra `10000 × 20`, tức 200000, đúng bằng con
số lúc tính đường vòng.
::
:::

:::opt
Máy báo lỗi, vì `so_o` là chữ chứ chưa phải số
::why
Gần đúng ở chỗ bạn nhớ đúng điều bài trước dặn: chừng nào còn chữ thì câu tính
chưa phải một con số. Linh cảm ấy là linh cảm chính xác, và nó là lý do cả bài
này tồn tại.

Chỗ lệch nằm ở dòng đầu tiên: `so_o = 20` đã điền sẵn rồi. Trước dòng ấy thì
`so_o` đúng là chưa giữ gì; sau dòng ấy, mọi chỗ viết `so_o` đều đọc ra 20. Máy
không gặp chữ nào chưa được điền, nên nó tính bình thường.
::
:::
::::

::::explain{#doi-so-doi-ket-qua}
Đặt tên cho thứ vừa thấy, để mang đi được:

> **Thay giá trị:** điền một số vào **mọi** chỗ mang cùng một chữ, rồi tính theo
> đúng thứ tự phép toán cũ. Cả câu thu lại thành **đúng một** con số.

Một câu ngắn nhưng có hai chỗ để dành cho các bài sau.

**Chỗ thứ nhất — hai chữ khác nhau là hai ô khác nhau.** Trong `15000 × n − 5000 × n`
chỉ có một chữ, nên chỉ cần một con số để điền đầy. Nếu Byte bán thêm nước và
viết `15000 × n + 8000 × c` thì cần **hai** con số, và chúng không buộc phải
bằng nhau.

**Chỗ thứ hai — điền xong là hết ô trống.** Sau khi thay, câu tính không còn chỗ
nào chờ nữa. Nó thành một con số cụ thể, cứng, không đổi được. Muốn có con số
khác thì phải quay về câu tính gốc và điền lại từ đầu — chứ không sửa được con
số vừa ra.

Chính vì thế mà câu tính có chữ đáng giá hơn một con số: **một câu tính thì
điền lại được, một con số thì không.**
::::

::::code{#ba-ngay-ba-lan-dien}
Ba ngày đầu tuần, xe bánh mì bán được:

- **thứ Hai:** 8 ổ
- **thứ Ba:** 20 ổ
- **thứ Tư:** 12 ổ

Tiền lãi mỗi buổi vẫn là đúng một câu tính: `15000 × so_o − 5000 × so_o`.

Điền ba chỗ trống. **Cả ba chỗ viết giống hệt nhau** — đó chính là điều bài này
muốn bạn thấy: câu tính chỉ có một, thứ đổi là con số nằm trong `so_o` lúc dòng
ấy chạy.

Bài chấm bằng **cả ba ngày**, và ba ngày cho ra ba con số khác nhau. Gõ cứng
`80000` vào mọi chỗ thì thứ Ba sai; gõ cứng `200000` thì thứ Hai sai. Chỉ câu
tính viết thật, có đọc `so_o`, mới qua được cả ba.

```python title=starter
so_o = 8
tien_lai_thu_2 = ___

so_o = 20
tien_lai_thu_3 = ___

so_o = 12
tien_lai_thu_4 = ___

print(tien_lai_thu_2)
print(tien_lai_thu_3)
print(tien_lai_thu_4)
```

```python title=solution
so_o = 8
tien_lai_thu_2 = 15000 * so_o - 5000 * so_o

so_o = 20
tien_lai_thu_3 = 15000 * so_o - 5000 * so_o

so_o = 12
tien_lai_thu_4 = 15000 * so_o - 5000 * so_o

print(tien_lai_thu_2)
print(tien_lai_thu_3)
print(tien_lai_thu_4)
```

```python title=test
# Hai câu `!=` đứng trước: chúng canh cái bẫy "ba chỗ trống chép cùng một con
# số". Xếp sau các câu `==` thì chúng không bao giờ chạy tới.
assert tien_lai_thu_2 != tien_lai_thu_3, "thứ Hai bán 8 ổ, thứ Ba bán 20 ổ — hai lần điền khác nhau phải cho hai con số khác nhau"
assert tien_lai_thu_3 != tien_lai_thu_4, "thứ Ba bán 20 ổ, thứ Tư bán 12 ổ — hai lần điền khác nhau phải cho hai con số khác nhau"
assert tien_lai_thu_2 == 80000, "điền 8 vào MỌI chỗ mang so_o: 15000 × 8 − 5000 × 8, tức 120000 − 40000, ra 80000"
assert tien_lai_thu_3 == 200000, "điền 20 vào MỌI chỗ mang so_o: 300000 − 100000 ra 200000"
assert tien_lai_thu_4 == 120000, "điền 12 vào MỌI chỗ mang so_o: 180000 − 60000 ra 120000"
```

:::hints
- kind: attention
  body: Nhìn dòng ngay phía trên mỗi chỗ trống. Nó cho biết hôm ấy `so_o` giữ con số nào. Chỗ trống thì không được nhắc tới con số ấy — nó phải nhắc tới cái tên, để hôm sau tên đổi thì nó đổi theo.
- kind: strategy
  body: Chép lại đúng câu tính tiền lãi trong đề, giữ nguyên chữ `so_o` ở cả hai chỗ. Ba chỗ trống nhận đúng cùng một dòng chữ, không khác một ký tự nào — máy sẽ tự đọc `so_o` ra ba con số khác nhau vì ba dòng ấy chạy ở ba thời điểm khác nhau.
- kind: one-line
  body: "Cả ba chỗ trống đều là `15000 * so_o - 5000 * so_o`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là câu tính tiền lãi viết bằng chính cái tên `so_o`, không phải con số kết quả — chép cứng con số thì việc "điền vào" mà bài vừa dạy không xảy ra ở đâu cả
  requireAst:
  # Ba chỗ trống, mỗi chỗ phải ĐỌC `so_o` ít nhất một lần. Khung khởi đầu không
  # đọc `so_o` lần nào (ba dòng `so_o = ...` là chỗ ĐẶT tên, không tính), nên
  # `min: 3` chặn được đúng đáp án chép cứng ba con số.
  - kind: uses-name, target: so_o, min: 3
  # Có tên thôi chưa đủ: `so_o` trơ một mình cũng đọc tên. Mỗi chỗ trống phải
  # có ít nhất một phép nhân, vì tiền lãi là mấy nghìn NHÂN số ổ.
  - kind: uses-operator, target: *, min: 3
  forbidAst:
  # Lưới thứ hai, chặn đúng ba con số KẾT QUẢ. Lời giải viết `15000`, `5000` và
  # ba con số ổ, không viết nguyên văn ba số này, nên luật không cản ai làm thật.
  - kind: has-literal, target: 80000
  - kind: has-literal, target: 200000
  - kind: has-literal, target: 120000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^80000\n200000\n120000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba dòng chữ giống hệt nhau mà ra ba con số. Vậy câu tính giữ được nhiều hơn một
con số giữ được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa điền ba lần vào cùng một câu tính và được ba con số. Điền 8 ra 80000,
điền 20 ra 200000, điền 12 ra 120000. Không lần nào là "câu trả lời đúng" hơn
lần nào — cả ba đều đúng, mỗi cái đúng cho ngày của nó.

Byte bán cả tuần, bảy ngày, bảy con số ổ khác nhau. Byte định làm thế này: điền
lần lượt từng ngày vào câu tính, rồi ghi kết quả xuống một cuốn sổ, mỗi ngày một
dòng.

Ghi xong bảy dòng ấy, trong tay Byte là thứ gì? Nó có phải một con số không? Có
phải một câu tính không?

Và câu hỏi khó hơn: nếu Byte không dừng ở bảy ngày mà ghi tiếp — 0 ổ, 1 ổ, 2 ổ,
3 ổ, cứ thế mãi — thì cuốn sổ ấy đang tả cái gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
