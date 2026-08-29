---
id: toan.dai-so-va-ham-so.khi-so-nao-cung-dung
title: Khi số nào cũng đúng
summary: Có câu điền số nào vào cũng đúng — vì hai vế của nó vốn là hai cách viết của cùng một câu tính, chứ không phải một câu đố đang chờ lời giải.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.identity]
requires: [math.order-of-operations, math.parentheses, math.multiply-distributive, math.negative-number, math.division-by-zero, core.variable, core.reassign, core.print-variable, core.arithmetic, core.number-literal, core.boolean, ctrl.comparison]
concepts: [math.dong-nhat-thuc, math.tap-nghiem, math.phep-giu-nghiem, math.hieu-hai-ve]
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
Hôm qua có câu không số nào làm đúng. Hôm nay mình gặp câu ngược hẳn lại.
::::

::::explain{#cau-hoi-nguoc-lai}
Bài trước kết thúc ở một chỗ lạ: gỡ mãi thì câu còn lại `0 = 30000`. Không còn ô
trống nào để điền, mà câu thì sai. Đọc ra: **không số nào** làm nó đúng — tập
nghiệm rỗng.

Bài trước để lại một câu hỏi: có câu nào ngược lại không — điền số nào vào cũng
đúng?

Có. Và nó không hiếm chút nào. Xe bánh mì của Byte đẻ ra một câu như thế ngay
trong buổi sáng đầu tiên có hai người cùng đứng bán.
::::

::::example{#hai-nguoi-hai-cach-go-tien}
Xe bán combo: mỗi combo gồm **một ổ bánh mì 15 000 đ** và **một ly trà đá
5 000 đ**. Khách mua `n` combo.

Hai người tính tiền hai kiểu, và cả hai đều tính đúng theo cách của mình:

- **Chị Ba** gõ từng món: `15000 × n` tiền bánh, cộng `5000 × n` tiền trà đá.
- **Byte** gõ gộp: mỗi combo tròn 20 000 đ, nên `20000 × n`.

An đứng cạnh, thấy hai màn hình hai công thức khác nhau, bèn hỏi một câu rất
hợp lý: *bán bao nhiêu combo thì hai cách gõ ra cùng một số tiền?*

Viết câu hỏi ấy thành phương trình:

```text
15000 × n + 5000 × n = 20000 × n
```

Giờ gỡ nó bằng đúng những phép đã học. Vế trái có hai cụm cùng mang chữ `n`, mà
hai cụm cùng mang một chữ thì gom lại được — đó là luật phân phối của T2.1, đọc
ngược: `15000 × n + 5000 × n` là `(15000 + 5000) × n`, tức `20000 × n`. Rồi
**bớt `20000 × n` ở cả hai đĩa** — bớt cùng một lượng ở hai bên là phép giữ
nguyên tập nghiệm:

```text
15000 × n + 5000 × n = 20000 × n
           20000 × n = 20000 × n
20000 × n − 20000 × n = 20000 × n − 20000 × n
                    0 = 0
```

`0 = 0`. Lại hết ô trống. Nhưng lần này câu còn lại **đúng**.

Đọc nó cho cẩn thận. Mỗi bước vừa đi đều không làm mất và không làm thêm nghiệm
nào, nên tập nghiệm cuối cùng chính là tập nghiệm ban đầu. Tập nghiệm của
`0 = 0` là gì? Câu ấy đúng, và nó đúng mà chẳng thèm nhìn `n`. Nên **mọi** số
đều là nghiệm.

Bảng giá trị (bài 5) cho thấy đúng chuyện đó, không sót lần điền nào:

```text
  n │ 15000×n + 5000×n │ 20000×n
  ──┼──────────────────┼────────
  0 │                0 │       0
  1 │            20000 │   20000
  4 │            80000 │   80000
  9 │           180000 │  180000
```

Hai cột phải trùng nhau ở mọi dòng, mãi mãi. Vì hai câu tính ấy **là hai biểu
thức tương đương** — đúng cái tên bài 6 đã đặt. Đặt dấu `=` giữa hai biểu thức
tương đương thì được một câu chẳng hỏi gì cả. Nó là một **đồng nhất thức**.

> **Đồng nhất thức** là một phương trình mà **mọi** giá trị điền vào đều làm nó
> đúng. Hai vế của nó vốn là hai cách viết của cùng một câu tính; dấu `=` ở đây
> không đặt ra câu đố nào, nó chỉ ghi lại một sự thật.
::::

::::explain{#khong-phai-moi-cai-0-bang-0-deu-noi-the}
Có một chỗ dễ va, và phải nói ngay.

Bài 14 cũng cho bạn thấy `0 = 0`, nhưng ở đó nó là chuyện **hỏng**: nhân hai vế
với 0 thì phương trình nào cũng biến thành `0 = 0`, kể cả câu chỉ có đúng một
nghiệm. Thử với `n = 7` — một câu có nghiệm duy nhất:

```text
n = 7
n × 0 = 7 × 0
    0 = 0
```

Ra `0 = 0` thật, mà `n` đâu có nhận mọi giá trị.

Vậy cùng một dòng chữ `0 = 0`, khi nào đọc là "mọi số", khi nào đọc là "chẳng
nói gì"? Câu trả lời không nằm ở dòng cuối — nó nằm ở **đường đi**:

> Bạn được đọc `0 = 0` là "mọi số đều là nghiệm" **chỉ khi** mọi bước bạn vừa
> đi đều là phép giữ nghiệm: cộng, trừ cùng một lượng ở hai vế, hoặc nhân, chia
> hai vế cho một số **khác 0**. Lỡ nhân hai vế với 0 thì `0 = 0` không nói gì
> hết — nó chỉ là dấu vết của việc bạn vừa xoá sạch câu hỏi.

Ở ví dụ combo phía trên, cả đường đi chỉ có một phép: bớt `20000 × n` ở hai vế.
Không có phép nhân với 0 nào. Nên `0 = 0` ở đó nói thật.
::::

::::predict{#doan-bon-dong commitOnce}
Byte đặt hai câu cạnh nhau và bắt máy chấm hộ:

- **Câu A** là câu combo vừa gỡ: `15000n + 5000n = 20000n`.
- **Câu C** là câu tiền vốn: thu vào `15000n`, mà mỗi ổ tốn `9000` đ nguyên liệu
  cộng `30000` đ than với chỗ ngồi mỗi buổi — *bán bao nhiêu ổ thì thu đúng
  bằng vốn?* → `15000n = 9000n + 30000`.

Dấu `==` trong Python hỏi "hai vế có ra cùng một số **với cái `n` đang giữ**
không", rồi trả `True` hoặc `False`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
n = 7
print(15000 * n + 5000 * n == 20000 * n)
print(15000 * n == 9000 * n + 30000)

n = 5
print(15000 * n + 5000 * n == 20000 * n)
print(15000 * n == 9000 * n + 30000)
```

:::opt{correct}
True, False, True, True
:::

:::opt
True, False, True, False
::why
Gần đúng ở chỗ bạn đọc trúng hai dòng đầu, và ở chỗ bạn tin rằng hai câu khác
nhau thì có tập nghiệm khác nhau. Quy tắc bạn đang dùng — *hai phương trình
khác nhau thì hiếm khi cùng đúng ở một số* — là quy tắc rất tốt khi cả hai câu
đều chỉ có một nghiệm.

Chỗ lệch nằm ở phạm vi của nó. Câu A không có "một nghiệm" nào để mà lệch với
câu C: tập nghiệm của nó là **mọi số**, nên nó chồng lên tập nghiệm của bất kỳ
câu nào khác, kể cả câu C. Ở `n = 5` cả hai cùng đúng — câu C đúng vì 5 là
nghiệm riêng của nó, câu A đúng vì câu A đúng ở khắp nơi.
::
:::

:::opt
True, True, True, True
::why
Gần đúng ở chỗ bạn thấy cả hai dòng đều là công thức tính tiền có thật của
quán, viết đúng, không sai đồng nào. Điều đó đúng: `15000n` là tiền thu thật,
`9000n + 30000` là tiền vốn thật.

Chỗ lệch: `==` không hỏi "công thức này viết đúng chưa". Nó hỏi "hai vế có ra
**cùng một con số** ngay lúc này không". Ở `n = 7`, thu là 105 000 còn vốn là
93 000 — hai con số khác nhau, nên máy trả `False`. Một công thức đúng vẫn có
thể cho hai vế lệch nhau; đó chính là lý do phương trình mới có gì để hỏi.
::
:::

:::opt
False, False, True, True
::why
Gần đúng ở chỗ bạn dùng đúng phép kiểm nghiệm của bài 16: thay nghiệm vào thì
hai vế ra cùng một số, nên ở `n = 5` mọi thứ mới sáng lên. Và `n = 5` đúng là
nghiệm của câu C thật.

Chỗ lệch là chiều ngược lại của quy tắc ấy. "Không phải nghiệm thì hai vế lệch"
chỉ đúng **cho chính câu đang xét**, không lan sang câu khác. `n = 7` không
phải nghiệm của câu C, nhưng nó chẳng liên quan gì tới câu A — mà câu A thì
không có số nào rơi ra ngoài tập nghiệm cả.
::
:::
::::

::::explain{#ba-loai-cau-nhin-bang-mot-thuoc-do}
Tới đây bạn đã gặp đủ ba loại câu trả lời. Có một cách nhìn xếp gọn cả ba vào
một chỗ: **hiệu hai vế**.

Bớt cả vế phải sang bên trái — đó vẫn là phép "bớt cùng một lượng ở hai đĩa" của
bài 17 — thì mọi phương trình đều thành dạng `(vế trái − vế phải) = 0`. Câu hỏi
"số nào làm phương trình đúng" trở thành "số nào làm cái hiệu ấy bằng 0".

Và cái hiệu ấy cư xử theo đúng ba kiểu:

| hiệu hai vế cư xử thế nào | tập nghiệm | tên gọi |
|---|---|---|
| luôn bằng 0, ở mọi lần điền | mọi số | **đồng nhất thức** |
| là một số khác 0, không đổi theo `n` | rỗng | **vô nghiệm** |
| đổi theo `n`, có lúc âm có lúc dương | đúng chỗ nó bằng 0 | phương trình bậc nhất thường |

Dòng cuối đáng nhìn kỹ. Với câu C, hiệu hai vế đi từ âm sang dương:

```text
  n │ thu = 15000×n │ vốn = 9000×n + 30000 │ hiệu hai vế
  ──┼───────────────┼──────────────────────┼────────────
  3 │         45000 │                57000 │      -12000
  4 │         60000 │                66000 │       -6000
  5 │         75000 │                75000 │           0
  6 │         90000 │                84000 │        6000
  7 │        105000 │                93000 │       12000
```

Mỗi ổ bán thêm kéo cái hiệu lên đúng 6 000. Nó đi lên đều đặn, cắt qua mốc 0
đúng một lần — ở `n = 5`. Đó là lý do sâu xa vì sao một phương trình bậc nhất
**thuộc loại thứ ba trong bảng** — loại có hiệu hai vế đổi theo `n` — có đúng
một nghiệm chứ không phải hai. Hai loại kia thì hiệu đứng yên, nên hoặc nó nằm
lì trên mốc 0 (số nào cũng đúng), hoặc nó chẳng bao giờ chạm mốc (không số nào
đúng).
::::

::::code{#do-hieu-hai-ve}
Đo hiệu hai vế của ba câu, mỗi câu ở **hai** lần điền khác nhau, rồi để chính
con số nói ra câu ấy thuộc loại nào.

- **Câu A** — combo: `15000n + 5000n` so với `20000n`.
- **Câu B** — hai xe cùng bán 15 000 đ một ổ, xe đầu hẻm trả 30 000 đ tiền chỗ,
  xe cuối hẻm trả 26 000 đ. *Bán bao nhiêu ổ thì hai xe lãi bằng nhau?* →
  `15000n − 30000` so với `15000n − 26000`.
- **Câu C** — thu so với vốn: `15000n` so với `9000n + 30000`.

Sáu chỗ trống, mỗi chỗ là **vế trái trừ vế phải** của một câu, tại cái `n` đang
đứng ngay phía trên.

Và một luật của bài: **chép nguyên hai vế vào, đừng rút gọn trước.** Câu C viết
gọn được thành `6000n − 30000` — đúng, nhưng viết thế là bạn đã trừ hộ máy mất
rồi, mà cái bài này đi tìm chính là chuyện *để máy trừ* hai vế khác hình dạng
rồi xem con số nói gì. Rút gọn là việc của bài sau.

Bài chấm bằng cả ba câu ở hai lần điền, và ba câu được chọn để cư xử khác hẳn
nhau: một câu hiệu luôn bằng 0, một câu hiệu là số khác 0 không đổi, một câu
hiệu đổi hẳn dấu. Gõ cứng một con số vào thì hỏng ngay câu bên cạnh — nên phải
viết ra phép tính thật.

```python title=starter
n = 4
hieu_a_4 = ___
hieu_b_4 = ___
hieu_c_4 = ___

n = 9
hieu_a_9 = ___
hieu_b_9 = ___
hieu_c_9 = ___

print(hieu_a_4, hieu_a_9)
print(hieu_b_4, hieu_b_9)
print(hieu_c_4, hieu_c_9)
```

```python title=solution
n = 4
hieu_a_4 = (15000 * n + 5000 * n) - 20000 * n
hieu_b_4 = (15000 * n - 30000) - (15000 * n - 26000)
hieu_c_4 = 15000 * n - (9000 * n + 30000)

n = 9
hieu_a_9 = (15000 * n + 5000 * n) - 20000 * n
hieu_b_9 = (15000 * n - 30000) - (15000 * n - 26000)
hieu_c_9 = 15000 * n - (9000 * n + 30000)

print(hieu_a_4, hieu_a_9)
print(hieu_b_4, hieu_b_9)
print(hieu_c_4, hieu_c_9)
```

```python title=test
# Hai câu `!=` đứng trước: chúng canh đúng hai cái bẫy của bài — "hiệu bằng 0
# nghĩa là câu nào cũng đúng" và "hiệu là số nào cũng được". Xếp chúng sau thì
# một câu `==` trượt trước, và hai cái bẫy không bao giờ sập.
assert hieu_b_4 != 0, "câu B: hai xe lệch nhau một khoản cố định, hiệu KHÔNG được bằng 0 — nếu bằng 0 thì bạn đang trừ nhầm một cụm"
assert hieu_c_4 != hieu_c_9, "câu C: hiệu hai vế phải ĐỔI khi n đổi — đó chính là lý do nó chỉ đúng ở đúng một chỗ"
assert hieu_a_4 == 0, "câu A ở n = 4: 15000n + 5000n và 20000n là hai cách viết cùng một số, hiệu phải bằng 0"
assert hieu_a_9 == 0, "câu A ở n = 9: vẫn bằng 0 — đồng nhất thức đúng ở MỌI lần điền, không chỉ ở n = 4"
assert hieu_b_9 == hieu_b_4, "câu B: chữ n triệt tiêu hết, nên hiệu là một số cố định, không đổi theo n"
assert hieu_c_4 == -6000, "câu C ở n = 4: thu 60000, vốn 66000 — còn thiếu 6000, nên hiệu âm"
assert hieu_c_9 == 24000, "câu C ở n = 9: thu 135000, vốn 111000 — dư 24000, nên hiệu dương"
```

:::hints
- kind: attention
  body: Mỗi chỗ trống hỏi đúng một việc — lấy vế TRÁI trừ vế PHẢI của câu ấy. Đọc lại ba gạch đầu dòng ngay trên khung mã: mỗi gạch nêu rõ vế trái là câu tính nào và vế phải là câu tính nào. Và để ý dòng `n = 4` với `n = 9` đứng trên: ba chỗ trống dưới mỗi dòng dùng đúng cái `n` của dòng ấy.
- kind: strategy
  body: "Viết `(vế trái) - (vế phải)`, giữ nguyên dấu ngoặc ở vế phải khi vế phải là một tổng hay một hiệu — vì trừ đi cả cụm chứ không phải chỉ trừ số hạng đầu. Đừng tự nhẩm ra con số rồi gõ vào: sáu chỗ trống này dùng chung một khuôn, viết khuôn ra thì cả sáu chỗ đều xong, còn nhẩm thì phải nhẩm sáu lần."
- kind: one-line
  body: "Chỗ đầu là `(15000 * n + 5000 * n) - 20000 * n`; chỗ thứ hai là `(15000 * n - 30000) - (15000 * n - 26000)`; chỗ thứ ba là `15000 * n - (9000 * n + 30000)`. Ba chỗ dưới `n = 9` chép y hệt ba dòng trên."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải chép NGUYÊN hai vế rồi để máy trừ, không rút gọn trước và không nhẩm sẵn — viết thẳng `6000 * n - 30000` là bạn đã làm hộ máy đúng cái việc bài này giao cho nó
  requireAst:
  # Sáu chỗ trống, mỗi chỗ đọc `n` ít nhất một lần. Khung khởi đầu không đọc
  # `n` lần nào (ba dòng `n = 4`/`n = 9` là gán, không phải đọc), nên luật này
  # chặn đúng đáp án chép cứng sáu con số.
  - kind: uses-name, target: n, min: 6
  # Mỗi vế đều là "giá tiền nhân số ổ". Không có phép nhân nào thì người viết
  # đã tự nhân hộ máy rồi.
  - kind: uses-operator, target: *, min: 6
  # Mỗi chỗ trống là một phép TRỪ (vế trái trừ vế phải). Sáu chỗ, sáu dấu trừ
  # là sàn thấp nhất — cách viết nào cũng đạt.
  - kind: uses-operator, target: -, min: 6
  forbidAst:
  # Lưới thứ hai, chặn đúng bốn con số là KẾT QUẢ. Mọi cách viết hợp lệ đều
  # dựng từ các giá tiền 15000 / 5000 / 20000 / 30000 / 26000 / 9000, nên
  # không cách nào chứa nguyên văn bốn số dưới đây — luật không cản ai làm
  # thật. `0` nằm trong danh sách vì hiệu của câu A đúng bằng 0: không chặn
  # thì gõ thẳng `0` vào hai chỗ ấy là qua mà chẳng tính gì.
  - kind: has-literal, target: 0
  - kind: has-literal, target: 4000
  - kind: has-literal, target: 6000
  - kind: has-literal, target: 24000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^0 0\n-4000 -4000\n-6000 24000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhìn cột hiệu là biết ngay: câu nào hỏi thật, câu nào chỉ nói lại một sự thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ bạn có đủ ba loại câu trả lời cho một phương trình: **một số**, **không số
nào**, **mọi số**. Ba loại ấy phủ kín — không còn loại thứ tư.

Nhưng hãy nhìn lại cái bảng hiệu hai vế của câu C. Byte không hỏi nó vì tò mò.
Byte hỏi vì cuối buổi phải biết mình có bù nổi tiền vốn hay không, và câu hỏi
thật trong đầu Byte là:

> *Sáng nay bán bao nhiêu ổ thì **đủ bù** tiền vốn?*

Thử viết đúng câu ấy thành một câu tính, như bạn vừa làm với "thu đúng bằng
vốn". Và để ý một chữ: **đủ bù** không có nghĩa là *đúng bằng*. Bán 5 ổ thì
huề. Bán 9 ổ thì dư 24 000 — Byte càng thích, chứ có ai chê đâu.

Vậy `n = 5` là câu trả lời đúng, hay mới chỉ là **cái mốc** của câu trả lời? Và
nếu là cái mốc, thì cái dấu `=` — thứ duy nhất bạn có tới giờ để nối hai vế —
nó nói được chuyện "hơn" không?

Bài sau đổi dấu.
::::

::::checkpoint{mastery=0.8}
::::
