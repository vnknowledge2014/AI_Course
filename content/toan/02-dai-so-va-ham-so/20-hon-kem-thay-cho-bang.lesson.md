---
id: toan.dai-so-va-ham-so.hon-kem-thay-cho-bang
title: Hơn kém thay cho bằng
summary: Đổi dấu `=` thành `≥` thì câu hỏi đổi từ "đúng bằng" sang "đủ bù", và câu trả lời không còn là một điểm mà là cả một khoảng trên trục số.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.inequality]
requires: [math.thanh-so, math.compare-on-number-line, math.negative-number, math.order-of-operations, math.parentheses, core.variable, core.reassign, core.print-variable, core.arithmetic, core.number-literal, core.boolean, ctrl.comparison]
concepts: [math.bat-phuong-trinh, math.tap-nghiem-khoang, math.mut-dac, math.mut-rong]
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
Bán đúng 5 ổ là huề vốn. Nhưng bán 9 ổ mình cũng có kêu ca gì đâu?
::::

::::explain{#dau-bang-hoi-hep-qua}
Bài trước bạn tìm ra `n = 5`: bán đúng 5 ổ thì tiền thu bằng chằn chặn tiền vốn.

Nhưng đó không phải câu hỏi Byte muốn hỏi. Byte muốn biết mình **đủ bù** vốn hay
chưa — và đủ bù nghĩa là *bằng* **hoặc** *hơn*. Bán 6 ổ thì dư 6 000, bán 9 ổ
thì dư 24 000; cả hai đều là "đủ bù". Dấu `=` không nhận chúng, vì `=` chỉ hỏi
đúng một chuyện: hai vế có ra **cùng một con số** không.

Cái quán cần một dấu khác. Bạn đã có sẵn nó từ T2.1: trên thanh số, số nào đứng
bên phải thì lớn hơn. Hai dấu gốc là `<` và `>`; ghép thêm "hoặc bằng" vào mỗi
cái thì thành bốn:

| viết trên giấy | đọc | Python viết |
|---|---|---|
| `a < b` | a bé hơn b | `a < b` |
| `a > b` | a lớn hơn b | `a > b` |
| `a ≤ b` | a bé hơn **hoặc bằng** b | `a <= b` |
| `a ≥ b` | a lớn hơn **hoặc bằng** b | `a >= b` |

Thay dấu `=` bằng một trong bốn dấu ấy thì được một **bất phương trình**. Vẫn
là hai vế, vẫn là một ô trống, nhưng câu hỏi đã đổi: không còn hỏi "số nào làm
hai vế bằng nhau" mà hỏi "**những** số nào làm vế trái đứng đúng phía ấy so với
vế phải".

Chữ **những** là chỗ mọi thứ đổi.
::::

::::example{#do-bu-von-la-mot-tia}
Viết câu hỏi thật của Byte ra:

> *thu vào* ≥ *bỏ ra* → `15000 × n ≥ 9000 × n + 30000`

Gỡ nó bằng đúng những phép cũ. Bớt `9000 × n` ở hai vế:

```text
15000 × n ≥ 9000 × n + 30000
15000 × n − 9000 × n ≥ 9000 × n + 30000 − 9000 × n
             6000 × n ≥ 30000
```

Rồi chia hai vế cho 6 000 — một số **dương**:

```text
n ≥ 5
```

Tới đây chuyện khác hẳn phương trình. `n = 5` là **một** số. `n ≥ 5` không phải
một số nào cả — nó là một lời mô tả về **cả một đám** số: 5, 6, 7, 8, ... và
mọi số lớn hơn nữa. Tập nghiệm không còn là một chấm; nó là một **tia** chạy mãi
sang phải.

Vẽ nó lên **thanh số** của T2.1 — từ đây gọi là **trục số**, vẫn đúng cái
thanh ấy, chỉ đổi tên cho khớp với lúc nó nằm ngang dưới một mặt phẳng:

```text
   0   1   2   3   4   5   6   7   8   9  10
   ├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼──▶
                       ●──────────────────────▶
```

Cái chấm ở vạch 5 được tô **đặc**. Nó nói: *số 5 nằm trong tập nghiệm*. Đúng
vậy — bán 5 ổ là huề, mà huề thì đã đủ bù rồi.

Bây giờ hỏi một câu hơi khác: *bán bao nhiêu ổ thì có lãi thật, tức thu **hơn**
vốn?* Đổi `≥` thành `>`, gỡ y hệt, ra `n > 5`. Cùng một mốc, mà hình khác:

```text
   0   1   2   3   4   5   6   7   8   9  10
   ├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼──▶
                       ○──────────────────────▶
```

Cái chấm ở vạch 5 giờ **rỗng**: mốc vẫn ở đó để chỉ chỗ, nhưng chính số 5 **bị
loại**, vì bán 5 ổ thì lãi bằng 0, chưa hơn đồng nào.

> **Mút đặc ●** — mốc thuộc tập nghiệm (`≥`, `≤`).
> **Mút rỗng ○** — mốc chỉ đánh dấu chỗ, không thuộc tập nghiệm (`>`, `<`).

Bảng giá trị cho thấy hai cột ấy chỉ khác nhau đúng một dòng — dòng của cái mốc:

```text
  n │ thu = 15000×n │ vốn = 9000×n + 30000 │ thu ≥ vốn │ thu > vốn
  ──┼───────────────┼──────────────────────┼───────────┼──────────
  3 │         45000 │                57000 │     False │     False
  4 │         60000 │                66000 │     False │     False
  5 │         75000 │                75000 │      True │     False
  6 │         90000 │                84000 │      True │      True
  7 │        105000 │                93000 │      True │      True
```

Cả cột `thu ≥ vốn` lẫn cột `thu > vốn` đều là những cột **đổi đúng một lần** từ
`False` sang `True` rồi ở lì bên `True`. Đó là hình dạng của một tia. Cái
phương trình `thu = vốn` của bài trước cho đúng một dòng `True`; đổi nó sang
bất phương trình thì được **cả nửa bảng**. (Không phải phương trình nào cũng
cho đúng một dòng — bài 19 vừa bày ra ba loại. Ở đây ta đang nói về đúng cái
phương trình trong bảng này.)
::::

::::predict{#doan-bon-dong commitOnce}
Byte bắt máy chấm hộ bốn câu. Ba câu dùng `>=`, một câu dùng `>`, và có hai câu
rơi đúng vào cái mốc.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
n = 4
print(15000 * n >= 9000 * n + 30000)

n = 5
print(15000 * n >= 9000 * n + 30000)
print(15000 * n > 9000 * n + 30000)

n = 12
print(15000 * n >= 9000 * n + 30000)
```

:::opt{correct}
False, True, False, True
:::

:::opt
False, True, True, True
::why
Gần đúng ở chỗ bạn đọc trúng ba dòng, và ở chỗ bạn hiểu đúng cái mốc: `n = 5`
là chỗ quán vừa đủ bù vốn, nên nó phải bật lên `True`. Quy tắc bạn đang dùng —
*qua được mốc rồi thì mọi câu hỏi kiểu "nhiều hơn" đều đúng* — là quy tắc đúng
với mọi `n` **lớn hơn** 5.

Chỗ lệch nằm đúng tại mốc, và chỉ tại mốc. Ở `n = 5` hai vế bằng chằn chặn nhau:
75 000 với 75 000. `>=` nhận, vì nó có chữ "hoặc bằng" trong tên. `>` từ chối,
vì nó đòi vế trái phải **nhỉnh hơn**, mà 75 000 chẳng nhỉnh hơn 75 000 chút nào.
Hai dấu ấy khác nhau đúng một điểm — và đó chính là điểm bạn vừa được hỏi.
::
:::

:::opt
True, True, True, True
::why
Gần đúng ở chỗ bạn thấy bán càng nhiều thì càng lợi — điều ấy đúng: mỗi ổ bán ra
kéo phần chênh lên thêm 6 000 đ, không ổ nào kéo xuống.

Chỗ lệch là điểm xuất phát. 30 000 đ tiền than với chỗ ngồi phải trả **trước**,
bán được hay không cũng mất. Nên quán khởi hành từ chỗ âm 30 000 chứ không phải
từ 0, và bốn ổ đầu tiên chỉ mới đắp được 24 000 của cái hố ấy. Ở `n = 4` thu là
60 000 mà vốn là 66 000 — còn thiếu 6 000, nên máy trả `False`.
::
:::

:::opt
False, False, False, True
::why
Gần đúng ở chỗ bạn phân biệt rất chặt giữa "huề vốn" và "có lãi": ở `n = 5`
quán chưa kiếm được đồng nào, và điều đó hoàn toàn đúng.

Chỗ lệch là câu hỏi đang được hỏi. Dòng thứ hai không hỏi "có lãi chưa", nó hỏi
`thu >= vốn` — tức "đã **đủ bù** chưa". Huề vốn thì đúng là đã đủ bù: không nợ
ai đồng nào. Trong hai dấu `≥` và `>`, chính chữ "hoặc bằng" là thứ quyết định
mốc 5 được tô đặc hay tô rỗng, nên phải đọc kỹ dấu trước khi trả lời.
::
:::
::::

::::explain{#mot-diem-thanh-mot-khoang}
Đặt tên cho thứ vừa thấy, để mang đi được:

> Một **bất phương trình** là hai vế nối bằng `<`, `>`, `≤` hoặc `≥`. Tập nghiệm
> của nó thường không phải một điểm mà là cả một **khoảng** trên trục số, và cái
> mút của khoảng được tô **đặc** hay **rỗng** tuỳ dấu có chữ "hoặc bằng" hay
> không.

Có hai chỗ đáng để ý.

**Một, cái mốc vẫn được tìm bằng dấu `=`.** Muốn biết khoảng bắt đầu từ đâu, bạn
giải đúng cái phương trình đã học từ bài 15: `15000n = 9000n + 30000` cho
`n = 5`. Không có gì bị bỏ đi — cách gỡ phương trình thành **bước đầu** của bài
này. Cái mới chỉ là hai câu hỏi tiếp theo: *bên nào của mốc mới là bên đúng, và
bản thân cái mốc có được tính không?*

**Hai, phải luôn nhìn lại hiện vật.** `n ≥ 5` nói "mọi số từ 5 trở lên", mà
"mọi số" gồm cả 5,5 và 7,3. Với **ổ bánh mì** thì nửa ổ bán cho ai? Ở đây tập
nghiệm toán học là cả một tia, còn tập câu trả lời có nghĩa cho quán chỉ là
những vạch nguyên trên tia ấy: 5, 6, 7, ... Cùng một bất phương trình, hai câu
trả lời khác nhau tuỳ hiện vật — và cái quyết định là hiện vật, không phải phép
toán.
::::

::::code{#tia-nghiem-cua-quan}
Dựng ba dòng của bảng trên, mỗi dòng hai cột: *đủ bù chưa* và *lãi thật chưa*.
Ba giá trị `n` được chọn để kẹp lấy cái mốc — một dưới mốc, một **đúng** mốc,
một trên mốc.

Sáu chỗ trống. Mỗi chỗ là một câu **so sánh** giữa tiền thu `15000 × n` và tiền
vốn `9000 × n + 30000`; chỗ nào hỏi *đủ bù* thì dùng dấu có chữ "hoặc bằng",
chỗ nào hỏi *lãi thật* thì dùng dấu chặt.

Bài chấm bằng cả sáu ô. Ba dòng cho ba kết quả khác nhau (`False False`,
`True False`, `True True`), nên gõ cứng `True` hay `False` vào là hỏng ngay ô
bên cạnh — chỉ câu so sánh viết thật mới qua được cả sáu.

```python title=starter
n = 4
du_bu_4 = ___
lai_that_4 = ___

n = 5
du_bu_5 = ___
lai_that_5 = ___

n = 6
du_bu_6 = ___
lai_that_6 = ___

print(du_bu_4, lai_that_4)
print(du_bu_5, lai_that_5)
print(du_bu_6, lai_that_6)
```

```python title=solution
n = 4
du_bu_4 = 15000 * n >= 9000 * n + 30000
lai_that_4 = 15000 * n > 9000 * n + 30000

n = 5
du_bu_5 = 15000 * n >= 9000 * n + 30000
lai_that_5 = 15000 * n > 9000 * n + 30000

n = 6
du_bu_6 = 15000 * n >= 9000 * n + 30000
lai_that_6 = 15000 * n > 9000 * n + 30000

print(du_bu_4, lai_that_4)
print(du_bu_5, lai_that_5)
print(du_bu_6, lai_that_6)
```

```python title=test
# Hai câu `!=` đứng trước, vì chúng canh đúng hai cái bẫy của bài: "≥ với >
# thì cũng thế cả" và "đủ bù thì lúc nào cũng đủ bù". Xếp chúng xuống dưới thì
# một câu `==` trượt trước và hai cái bẫy không bao giờ sập.
assert du_bu_5 != lai_that_5, "n = 5 là MÚT của khoảng: thu bằng đúng vốn, nên `đủ bù` phải True mà `lãi thật` phải False — hai dấu ≥ và > khác nhau đúng ở điểm này"
assert du_bu_4 != du_bu_5, "n = 4 và n = 5 nằm hai bên mốc, nên hai ô `đủ bù` không thể giống nhau"
assert du_bu_4 == False, "n = 4: thu 60000, vốn 66000 — còn thiếu 6000, chưa đủ bù"
assert lai_that_4 == False, "n = 4: chưa đủ bù thì càng chưa có lãi"
assert du_bu_5 == True, "n = 5: thu 75000 bằng đúng vốn 75000 — huề vốn vẫn là đã đủ bù, nên ≥ nhận mốc này"
assert lai_that_5 == False, "n = 5: lãi đúng bằng 0 đồng, mà > đòi phải HƠN, nên mốc này bị loại — mút rỗng"
assert du_bu_6 == True, "n = 6: thu 90000, vốn 84000 — đã qua mốc"
assert lai_that_6 == True, "n = 6: dư 6000 đồng, đây mới là lãi thật"
```

:::hints
- kind: attention
  body: Nhìn hai cột cuối của cái bảng trong phần ví dụ — cột `thu ≥ vốn` và cột `thu > vốn`. Ba dòng bạn đang phải dựng chính là ba dòng `n = 4`, `n = 5`, `n = 6` của bảng ấy. Và để ý dòng `n = 4`, `n = 5`, `n = 6` đứng ngay trên mỗi cặp chỗ trống: hai ô dưới mỗi dòng dùng đúng cái `n` của dòng ấy.
- kind: strategy
  body: "Mỗi chỗ trống là MỘT câu so sánh, viết thẳng ra chứ không cần biến trung gian: bên trái dấu là tiền thu, bên phải là tiền vốn. Sáu ô dùng chung đúng hai khuôn — một khuôn cho `đủ bù`, một khuôn cho `lãi thật` — nên viết xong hai khuôn là chép được cả sáu. Đừng nhẩm ra True/False rồi gõ vào: cái bài đang hỏi là bạn chọn dấu nào, chứ không phải bạn tính nhẩm có nhanh không."
- kind: one-line
  body: "Ô `đủ bù` là `15000 * n >= 9000 * n + 30000`; ô `lãi thật` là `15000 * n > 9000 * n + 30000`. Hai khuôn ấy lặp lại y nguyên cho cả ba giá trị của `n`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một câu so sánh dựng trên `n` — `đủ bù` dùng `>=`, `lãi thật` dùng `>`; gõ thẳng `True` hay `False` thì máy không so gì cả, bạn so hộ nó rồi
  requireAst:
  # Ba ô `đủ bù` dùng `>=`, ba ô `lãi thật` dùng `>`. Hai luật này là thứ duy
  # nhất phân biệt được "hiểu mút đặc" với "hiểu mút rỗng": thiếu một trong
  # hai nghĩa là người viết đã dùng chung một dấu cho cả sáu ô.
  - kind: uses-operator, target: >=, min: 3
  - kind: uses-operator, target: >, min: 3
  # Sáu ô, mỗi ô đọc `n` ít nhất một lần. Khung khởi đầu không đọc `n` lần nào
  # (ba dòng `n = 4/5/6` là gán, không phải đọc), nên luật này chặn đúng đáp án
  # chép cứng sáu giá trị đúng/sai. `min: 6` chứ không phải 12, để cách viết đã
  # rút gọn — `6000 * n >= 30000` — vẫn được nhận.
  - kind: uses-name, target: n, min: 6
  - kind: uses-operator, target: *, min: 6
  forbidAst:
  # Lưới thứ hai. Sáu ô này nhận giá trị đúng/sai, nên đáp án chép cứng ở đây
  # không phải một con số mà là chính chữ `True`/`False`. Lời giải thật không
  # chứa hai chữ ấy ở đâu cả — nó để MÁY tự so ra.
  - kind: has-literal, target: True
  - kind: has-literal, target: False
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^False False\nTrue False\nTrue True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Dòng giữa mới là dòng đáng tiền: cùng một mốc mà hai câu trả lời khác nhau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa giải `15000n ≥ 9000n + 30000` bằng đúng hai phép của cái cân: **bớt
cùng một cụm ở hai vế**, rồi **chia hai vế cho 6 000**. Cả hai đều chạy ngon, và
cả hai đều là phép bạn đã dùng cho dấu `=` từ bài 13.

Nên câu hỏi tự nhiên là: cái cân có dùng lại được **nguyên xi** cho dấu `≥`
không?

Thử một phép mà bài 14 nói là hợp lệ với dấu `=`: **nhân hai vế với một số khác
0**. Lấy `−1` cho gọn. Bạn đã có `n ≥ 5`. Nhân hai vế với `−1`, giữ nguyên dấu:

```text
n ≥ 5   →   −n ≥ −5
```

Giờ kiểm nghiệm như bài 16 dạy, bằng một con số cụ thể. Lấy `n = 6` — chắc chắn
là nghiệm, vì 6 ≥ 5:

```text
−6 ≥ −5 ?
```

Trên trục số, −6 nằm bên **trái** −5. Câu ấy sai.

Một nghiệm thật mà biến đổi xong lại thành sai. Nghĩa là phép vừa dùng **không**
giữ nghiệm — trong khi với dấu `=` thì chính nó lại giữ. Có gì đó ở phép nhân
với số âm mà cái cân không kể được.

Bài sau đi tìm cái đó, và tìm luôn xem cộng trừ có bị dính không.
::::

::::checkpoint{mastery=0.8}
::::
