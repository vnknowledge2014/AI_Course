---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.ngan-xep-lam-gi
title: "Ngăn xếp dùng để làm gì — ngoặc có khớp không"
summary: "Kiểm một chuỗi ngoặc (){}[] có khớp không bằng ngăn xếp: mở thì đẩy vào, đóng thì lấy ra so đúng loại — đúng việc mọi trình biên dịch phải làm trước khi đọc mã của bạn."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.stack-application]
requires: [ds.stack, core.function-def, core.function-parameter, core.function-return, core.function-call, core.dict, core.list, core.list-append, core.list-membership, ctrl.for-each, ctrl.if, ctrl.elif, core.fstring]
concepts: [ds.stack-application]
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
Mỗi lần bạn gõ thiếu một dấu ngoặc đóng, một chương trình nào đó đã dùng
đúng ngăn xếp để phát hiện ra.
::::

::::explain{#mo-day-dong-lay}
Câu hỏi cuối bài trước: làm sao biết mọi dấu ngoặc mở trong một chuỗi đều
có đúng một dấu ngoặc đóng CÙNG LOẠI, lồng vào nhau đúng thứ tự? Chỉ đếm
số dấu mở và số dấu đóng là không đủ — chuỗi `"([)]"` có đúng hai dấu mở
và hai dấu đóng, nhưng nó KHÔNG khớp: dấu `)` đóng sai chỗ, đáng lẽ phải
là `]` mới đúng thứ tự lồng.

Ngăn xếp giải đúng bài toán này, bằng một luật rất ngắn — đi qua chuỗi
từng ký tự một:

- Gặp dấu **mở** (`(`, `[`, `{`): đẩy nó vào ngăn xếp. Bạn vừa "nợ" một
  dấu đóng cùng loại, ghi nợ lại để nhớ.
- Gặp dấu **đóng** (`)`, `]`, `}`): lấy đỉnh ngăn xếp ra — đó phải là dấu
  mở GẦN ĐÂY NHẤT còn nợ. Nếu nó đúng loại (mở tương ứng), món nợ đã trả,
  đi tiếp. Nếu sai loại, hoặc ngăn xếp đang RỖNG (đóng mà chẳng nợ ai),
  chuỗi hỏng ngay lập tức.
- Đi hết chuỗi mà ngăn xếp còn sót lại dấu mở nào chưa trả — cũng hỏng:
  vẫn còn nợ chưa thanh toán.

Đây chính là lý do ngăn xếp hợp với bài toán này: dấu đóng luôn phải khớp
với dấu mở GẦN ĐÂY NHẤT còn treo — đúng luật "vào sau, ra trước" bài
trước vừa dựng. Không phải bài toán giả định cho vui: trình biên dịch của
mọi ngôn ngữ lập trình, kể cả Python đang chạy đoạn mã này, đều phải làm
đúng việc này trước khi đọc hiểu bất kỳ dòng nào bạn viết.
::::

::::example{#theo-doi-ngan-xep}
Byte kiểm chuỗi `"a(b[c]d)e"` — chữ cái không phải ngoặc thì bỏ qua, chỉ
ngoặc mới đẩy vào hay lấy ra khỏi ngăn xếp.

```python title=readonly
dong_mo = {")": "(", "]": "[", "}": "{"}
ngan_xep = []

for ky_tu in "a(b[c]d)e":
    if ky_tu in "([{":
        ngan_xep.append(ky_tu)
        print(f"đẩy {ky_tu!r}  -> ngăn xếp: {ngan_xep}")
    elif ky_tu in ")]}":
        dinh = ngan_xep.pop()
        print(f"lấy {dinh!r} so với {ky_tu!r} -> ngăn xếp: {ngan_xep}")

print("còn sót:", ngan_xep)
```

```text title=readonly
đẩy '('  -> ngăn xếp: ['(']
đẩy '['  -> ngăn xếp: ['(', '[']
lấy '[' so với ']' -> ngăn xếp: ['(']
lấy '(' so với ')' -> ngăn xếp: []
còn sót: []
```

Mỗi dấu đóng luôn so với đỉnh ngăn xếp NGAY LÚC ĐÓ, không phải dấu mở đầu
tiên trong chuỗi. Khi `]` xuất hiện, đỉnh đang là `[` — mới vừa đẩy vào,
gần đây nhất — nên khớp đúng ngay. Kết thúc, ngăn xếp rỗng: không dấu mở
nào còn treo nợ.
::::

::::predict{#ngoac-xen-ke commitOnce}
Byte kiểm chuỗi `"([)]"` bằng đúng luật vừa học — không chạy tay từng
dòng, chỉ suy luận.

**Trước khi lần theo từng ký tự**, bạn đoán: chuỗi này có khớp không, và
vì sao?

:::opt{correct}
Không khớp — khi gặp `)`, đỉnh ngăn xếp lúc đó là `[`, không phải `(`.
:::

:::opt
Khớp — vì mỗi loại ngoặc đều xuất hiện đủ cặp: hai dấu tròn `()`, hai dấu
vuông `[]`.
::why
Gần đúng ở việc đếm — chuỗi này đúng là có đủ số lượng: hai dấu tròn, hai
dấu vuông, không thiếu dấu nào.

Chỗ lệch là bài toán không hỏi SỐ LƯỢNG, mà hỏi THỨ TỰ LỒNG. `(` đẩy vào
trước, `[` đẩy vào sau — đỉnh ngăn xếp lúc này là `[`. Ký tự tiếp theo là
`)`, nó đòi đúng một `(` ở đỉnh, nhưng đỉnh đang là `[` — sai loại, dù cả
hai đều "còn nợ" thật.
::
:::

:::opt
Không khớp — vì `(` và `[` là hai loại ngoặc khác nhau, không được phép
đứng lồng vào nhau.
::why
Gần đúng ở kết luận cuối — chuỗi này đúng là không khớp.

Chỗ lệch là LÝ DO. Lồng khác LOẠI ngoặc vào nhau hoàn toàn hợp lệ — chuỗi
`"([])"` khớp tuyệt đối, `(` chứa trọn `[]` bên trong nó. Vấn đề của
`"([)]"` không phải việc trộn hai loại ngoặc, mà là chúng đóng SAI THỨ
TỰ: `)` đến trước khi `[` đã được đóng lại.
::
:::

:::opt
Khớp — vì mỗi dấu mở cuối cùng đều có một dấu đóng, ngăn xếp rỗng khi đi
hết chuỗi.
::why
Gần đúng ở việc chuỗi đúng là 4 ký tự, mở-mở-đóng-đóng, và nếu chỉ đếm độ
sâu (mở thì +1, đóng thì −1) thì độ sâu quả thật về 0 ở cuối — cảm giác
"cân bằng" không phải vô căn cứ.

Chỗ lệch là đúng chỗ mà cả bài này tồn tại để sửa: đếm độ sâu không kiểm
tra ĐÚNG LOẠI. Khi `)` xuất hiện, luật đòi so nó với đỉnh ngăn xếp — mà
đỉnh lúc đó là `[`, không phải `(`. Sai loại phải chặn ngay, không đợi
tới cuối chuỗi mới xét ngăn xếp có rỗng hay không.
::
:::
::::

::::code{#ham-kiem-ngoac}
Byte viết một hàm `khop_ngoac(chuoi)` trả về `True` nếu mọi ngoặc trong
`chuoi` khớp đúng thứ tự, `False` nếu không.

```python title=starter
def khop_ngoac(chuoi):
    dong_mo = {")": "(", "]": "[", "}": "{"}
    ngan_xep = []
    for ky_tu in chuoi:
        if ky_tu in "([{":
            ngan_xep.append(ky_tu)
        elif ky_tu in ")]}":
            if not ngan_xep:
                return False
            dinh = ngan_xep.pop()
            if ___:                     # dinh vừa lấy ra có ĐÚNG LOẠI mở cho ky_tu này không?
                return False
    return ___                          # còn dấu mở nào chưa trả nợ không?

print(khop_ngoac("a(b[c]d)e"))
print(khop_ngoac("(]"))
print(khop_ngoac("(()"))
print(khop_ngoac(")("))
print(khop_ngoac(""))
```

```python title=solution
def khop_ngoac(chuoi):
    dong_mo = {")": "(", "]": "[", "}": "{"}
    ngan_xep = []
    for ky_tu in chuoi:
        if ky_tu in "([{":
            ngan_xep.append(ky_tu)
        elif ky_tu in ")]}":
            if not ngan_xep:
                return False
            dinh = ngan_xep.pop()
            if dinh != dong_mo[ky_tu]:
                return False
    return len(ngan_xep) == 0

print(khop_ngoac("a(b[c]d)e"))
print(khop_ngoac("(]"))
print(khop_ngoac("(()"))
print(khop_ngoac(")("))
print(khop_ngoac(""))
```

```python title=test
assert khop_ngoac("a(b[c]d)e") == True, "ba loại ngoặc lồng đúng thứ tự phải khớp"
assert khop_ngoac("(]") == False, "'(' phải khớp với ')', không phải ']' — sai loại"
assert khop_ngoac("(()") == False, "còn một dấu '(' chưa được đóng — không được khớp"
assert khop_ngoac(")(") == False, "dấu đóng đến trước khi có dấu mở nào để trả nợ — phải trượt ngay, không phải đợi tới cuối"
assert khop_ngoac("") == True, "chuỗi rỗng không nợ dấu mở nào — mặc định khớp"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất hỏi đúng câu bài vừa học — "dinh" (đỉnh vừa lấy ra) có phải dấu mở TƯƠNG ỨNG với ky_tu hiện tại không. Từ điển dong_mo đã dịch sẵn "dấu đóng nào cần dấu mở nào" — đừng so sánh dinh với ky_tu trực tiếp, chúng vốn không cùng loại ký tự.
- kind: strategy
  body: 'Chỗ trống thứ nhất: dong_mo[ky_tu] là dấu mở ĐÚNG mà ky_tu này đòi hỏi — so sánh "dinh != dong_mo[ky_tu]"; khác thì sai loại, phải trả về False. Chỗ trống thứ hai: hàm chỉ được coi là khớp nếu KHÔNG còn dấu mở nào bị bỏ quên trong ngan_xep sau khi đi hết chuỗi — kiểm tra "len(ngan_xep) == 0".'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `dinh != dong_mo[ky_tu]` và `len(ngan_xep) == 0`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống thứ nhất phải thật sự so sánh dinh với dong_mo[ky_tu] (không phải so ky_tu trực tiếp, không phải một điều kiện luôn đúng/luôn sai), và chỗ trống thứ hai phải thật sự kiểm tra ngan_xep còn phần tử hay không — bài này đang dạy CƠ CHẾ so khớp đúng loại và đúng lúc rỗng, không phải một hàm chỉ đúng cho đúng năm câu gọi thử ở trên
  requireAst:
  # Đếm thật bằng ast trên khối solution: toán tử "!=" xuất hiện đúng 1 lần —
  # đúng tại chỗ trống 1. Không có cổng này, "if dinh != ky_tu:" (so trực
  # tiếp, bỏ qua từ điển dong_mo — luôn cho kết quả sai loại vì '(' không bao
  # giờ bằng ')') vẫn có phép "!=" nên một cổng chỉ đếm toán tử suông sẽ không
  # phân biệt được — cổng dưới đếm luôn số lần đọc tên "dong_mo" để chặn đúng
  # kiểu hụt đó. Đếm thật: dong_mo được ĐỌC đúng 1 lần trong lời giải (chỉ ở
  # dong_mo[ky_tu] của chỗ trống 1 — dòng gán "dong_mo = {...}" là GHI, không
  # phải ĐỌC, nên uses-name không tính) — min:2 sẽ đánh trượt cả lời giải
  # đúng, đúng lớp lỗi tệ nhất; min:1 mới khớp thứ bài thật sự đòi.
  - kind: uses-operator, target: "!=", min: 1
  - kind: uses-name, target: dong_mo, min: 1
  # ngan_xep được ĐỌC 4 lần trong lời giải: append(), "if not ngan_xep",
  # pop(), và len(ngan_xep) ở chỗ trống 2. Thiếu chỗ trống 2 thật sự kiểm tra
  # ngan_xep (ví dụ hardcode "return True") thì chỉ còn 3 lần đọc — min:4
  # chặn đúng nó. Cách viết khác "ngan_xep == []" cũng đọc ngan_xep 1 lần ở
  # đó, ra cùng số 4, nên min không ép riêng một cách viết.
  - kind: uses-name, target: ngan_xep, min: 4
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^True\\nFalse\\nFalse\\nFalse\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm chuỗi, năm câu trả lời đúng — kể cả cái xảo quyệt nhất, `"([)]"`,
không lọt qua được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ngăn xếp luôn phục vụ người, việc, hay dấu ngoặc ĐẾN SAU CÙNG trước. Đó là
đúng luật cho việc undo, cho việc so ngoặc. Nhưng nhiều tình huống đời
thực lại đòi hỏi điều NGƯỢC HẲN: ai đến trước phải được phục vụ trước, ai
đến sau phải chờ, dù có sốt ruột đến mấy — xếp hàng mua vé là ví dụ rõ
nhất.

Ngăn xếp không làm được việc đó — luật "vào sau, ra trước" của nó đi
ngược lại đúng thứ bạn cần ở một hàng chờ công bằng. Cấu trúc nào làm
được?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
