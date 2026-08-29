---
id: nen-tang.list-dict-set-tuple.xep-lai-thanh-so-moi
title: Xếp lại thành một sổ mới
summary: '`sorted` đưa lại một danh sách MỚI đã xếp từ nhỏ tới lớn — cuốn sổ gốc vẫn nằm nguyên theo thứ tự ghi.'
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.sorted]
requires: [core.dict, core.dict-items, core.for-unpack, core.tuple, core.tuple-unpack, core.slice-copy, core.list-aliasing, core.list, core.list-append, core.list-remove, ctrl.for-each, core.variable, core.assignment, core.fstring, core.output]
practices: [core.dict-items, core.for-unpack, core.list-append, ctrl.for-each, core.fstring]
concepts: [core.danh-sach, core.gia-tri, core.thu-tu-buoc]
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
Mình không xáo cuốn sổ của bạn. Mình chép ra một cuốn mới rồi xếp trên cuốn ấy.
::::

::::explain{#do-bang-mat-het-bao-lau}
Bài trước bạn đã in được bản báo cáo gọn ghẽ: mỗi vòng nhận cả cặp, mở ra hai
cái tên, in một dòng. Từ đó tới giờ Byte ghi thêm hai khoản — **xăng xe** và
**mua sách** — nên cuốn sổ trong bài này có năm dòng chứ không phải ba.

```python title=readonly
chi = {
    "sửa xe": 500000,
    "cà phê": 25000,
    "biếu bà": 300000,
    "xăng xe": 60000,
    "mua sách": 40000,
}

for ten, tien in chi.items():
    print(f"{ten}: {tien} đồng")
```

Máy in ra:

```text title=readonly
sửa xe: 500000 đồng
cà phê: 25000 đồng
biếu bà: 300000 đồng
xăng xe: 60000 đồng
mua sách: 40000 đồng
```

Năm dòng thì mắt bạn dò được: khoản tốn nhất là sửa xe. Nhưng sổ thật của Byte
cuối tháng có bốn mươi dòng, và thứ tự in ra là **thứ tự ghi sổ** — khoản nào
phát sinh trước thì nằm trên. Thứ tự ấy không nói gì về chuyện tiền nhiều hay ít.

Byte muốn thấy các con số **xếp hàng từ nhỏ tới lớn**. Một hàng đã xếp thì khỏi
phải dò: khoản tốn nhất nằm ở cuối hàng, lần nào cũng vậy.

Trước hết, tách riêng cột tiền ra thành một danh sách. Đồ nghề cho việc này bạn
đã có đủ từ lâu: một danh sách rỗng, một vòng duyệt, và `.append`.

```python title=readonly
tien = []
for ten, so_tien in chi.items():
    tien.append(so_tien)

print(tien)
```

```text title=readonly
[500000, 25000, 300000, 60000, 40000]
```

Năm con số, đúng thứ tự ghi sổ, và lúc này chúng đã rời khỏi tên của mình. Cái
tên đi đâu mất là chuyện phải bàn — nhưng để lát nữa.
::::

::::explain{#mot-cuon-so-thu-hai}
Việc xếp một dãy số theo thứ tự là việc Python đã có sẵn một cái tên để gọi:
`sorted`.

Bạn đưa cho nó một danh sách, nó đưa lại cho bạn **một danh sách khác** — cùng
chừng ấy con số, nhưng đã nằm từ nhỏ tới lớn.

```python title=readonly
da_xep = sorted(tien)
```

Chữ đáng dừng lại là chữ **khác**. `sorted` không lật cuốn sổ của bạn ra rồi
xáo lại các dòng trong đó. Nó đọc cuốn sổ, chép ra một cuốn thứ hai, và xếp trên
cuốn thứ hai ấy. Xong việc, bạn có hai danh sách nằm cạnh nhau:

- `tien` — vẫn đúng thứ tự bạn ghi vào, không suy suyển một chỗ nào;
- `da_xep` — cùng năm con số, đã xếp.

Chuyện này bạn đã gặp một lần rồi, ở bài 5: `so[:]` đưa lại một danh sách
mới chứ không đưa lại chính cuốn cũ. `sorted` cũng thuộc loại đó — hỏi nó một
câu, nó dựng ra một thứ mới để trả lời, chứ không sửa thứ bạn đưa vào.

Và giữ được cuốn sổ gốc là chuyện đáng giá. Thứ tự ghi sổ chính là thứ tự thời
gian: khoản nào tiêu trước, khoản nào tiêu sau. Một công cụ mà xếp xong thì xoá
mất thứ tự ấy là một công cụ lấy đi của bạn một thông tin có thật, mà không hỏi.
::::

::::predict{#so-goc-con-lai-gi commitOnce}
Byte chạy đoạn dưới. Dòng `sorted` bạn vừa gặp; câu đáng đoán nằm ở **dòng in
cuối cùng** — sau khi xếp xong, cuốn sổ gốc còn lại gì.

**Trước khi bấm chạy**, bạn đoán dòng cuối in ra gì?

```python title=readonly
tien = [500000, 25000, 300000, 60000, 40000]
da_xep = sorted(tien)

print(f"Đã xếp: {da_xep}")
print(f"Sổ gốc: {tien}")
```

:::opt{correct}
Sổ gốc: [500000, 25000, 300000, 60000, 40000]
:::

:::opt
Sổ gốc: [25000, 40000, 60000, 300000, 500000]
::why
Gần đúng ở chỗ bạn đang mang theo một kinh nghiệm thật: gần như mọi thứ bạn làm
với danh sách từ đầu mạch tới giờ đều **sửa tại chỗ**. `.append` gắn thêm vào
đúng cuốn sổ ấy, `.remove` móc một khoản ra khỏi đúng cuốn sổ ấy. Nghĩ rằng
`sorted` cũng xếp lại ngay trên cuốn ấy là suy luận theo đúng những gì bạn đã
thấy.

Chỗ lệch nằm ở hình dạng của câu lệnh. `.append` viết là `so.append(x)` — cuốn
sổ đứng trước dấu chấm, nó là kẻ bị sai bảo. Còn `sorted(tien)` thì cuốn sổ nằm
**trong ngoặc**, ở vị trí thứ được đưa vào cho người ta đọc. Đọc thì không sửa.
Và thứ `sorted` đưa ra là một danh sách mới, nên `tien` in ra vẫn nguyên thứ tự
ghi sổ.
::
:::

:::opt
Hai dòng in ra giống hệt nhau, vì `da_xep` và `tien` là hai cái tên của cùng một
danh sách
::why
Gần đúng ở chỗ khó nhất của cả mạch này, và bạn nhớ đúng bài 4: gán một
danh sách cho một cái tên mới thì **không** tạo ra bản mới — hai cái tên cùng
trỏ vào một cuốn sổ, ai sửa cũng thấy.

Chỗ lệch là ở thứ nằm bên phải dấu bằng. Bài ấy nói về `da_xep = tien`, bên phải
là chính cuốn sổ, nên đúng là hai tên một cuốn. Ở đây bên phải là `sorted(tien)`
— một lời gọi, và lời gọi ấy **dựng ra** một danh sách chưa từng tồn tại. Cái
tên `da_xep` dán lên cuốn mới ấy, không dán lên cuốn cũ.
::
:::

:::opt
Sổ gốc: []
::why
Gần đúng ở chỗ bạn hình dung việc xếp hàng như việc dọn đồ: nhấc từng con số ra
khỏi chỗ cũ rồi đặt sang chỗ mới. Dọn kiểu ấy thì chỗ cũ rỗng đi thật, và đó là
cách người ta xếp lại một tủ sách ngoài đời.

Chỗ lệch: `sorted` không nhấc gì đi cả, nó **đọc rồi chép**. Năm con số vẫn nằm
nguyên trong `tien`; năm con số trong `da_xep` là năm giá trị được ghi ra một
chỗ khác. Không có con số nào bị lấy khỏi cuốn sổ gốc, nên nó không rỗng đi.
::
:::
::::

::::example{#hai-danh-sach-nam-canh-nhau}
Chạy thử trọn vẹn, in cả hai danh sách ra để nhìn tận mắt điều bạn vừa đoán.

```python title=readonly
tien = [500000, 25000, 300000, 60000, 40000]

da_xep = sorted(tien)

print(f"Sổ gốc: {tien}")
print(f"Đã xếp: {da_xep}")
```

Máy in ra:

```text title=readonly
Sổ gốc: [500000, 25000, 300000, 60000, 40000]
Đã xếp: [25000, 40000, 60000, 300000, 500000]
```

Hai dòng, hai danh sách. Dòng dưới là hàng đã xếp: `25000` đứng đầu, `500000`
đứng cuối. Dòng trên không đổi một chỗ nào so với lúc bạn viết nó ra.

Cách nói của giới lập trình cho chuyện này: `sorted` **trả về** một danh sách
mới — đúng chữ `return` bạn đã gặp ở mạch Hàm, nghĩa là kết quả được đưa ra
ngoài cho chỗ gọi hứng lấy.

Còn chuyện `sorted` không đụng tới cuốn sổ bạn đưa vào thì là nết riêng của nó,
không phải luật chung của mọi hàm: ở bài 4 bạn đã thấy một hàm nhận cuốn sổ rồi
`.append` thêm một dòng, và ra ngoài thì cuốn sổ của bạn có dòng ấy thật.

Vì kết quả được đưa ra ngoài, bạn phải hứng lấy nó bằng một cái tên. Viết
`sorted(tien)` đứng trơ một mình giữa chương trình thì danh sách mới ấy sinh ra
xong không ai giữ, và nó biến mất ngay dòng sau — y hệt chuyện gọi một hàm có
`return` mà quên gán lại.
::::

::::code{#xep-cot-tien}
Cột tiền của Byte đã nằm sẵn trong `tien`, đúng thứ tự ghi sổ.

Hãy điền chỗ trống để `da_xep` giữ **cùng năm con số ấy nhưng đã xếp từ nhỏ tới
lớn**, và cuốn sổ gốc thì không được đụng vào.

```python title=starter
tien = [500000, 25000, 300000, 60000, 40000]

da_xep = ___

print(f"Sổ gốc: {tien}")
print(f"Đã xếp: {da_xep}")
```

```python title=solution
tien = [500000, 25000, 300000, 60000, 40000]

da_xep = sorted(tien)

print(f"Sổ gốc: {tien}")
print(f"Đã xếp: {da_xep}")
```

```python title=test
# Hai câu, kiểm hai nửa của cùng một lời hứa.
assert da_xep == [25000, 40000, 60000, 300000, 500000], "da_xep phải là năm con số của sổ này nằm từ nhỏ tới lớn: 25000, 40000, 60000, 300000, 500000"
assert tien == [500000, 25000, 300000, 60000, 40000], "cuốn sổ gốc phải còn nguyên thứ tự ghi — 500000 rồi 25000 rồi 300000 rồi 60000 rồi 40000"
```

:::hints
- kind: attention
  body: Chỗ trống nằm bên phải dấu bằng, nên thứ điền vào phải là một thứ **cho ra** một danh sách. Cuốn sổ `tien` đã có sẵn ở dòng trên, bạn không phải gõ lại năm con số ấy lần nữa.
- kind: strategy
  body: Việc xếp một dãy số đã có sẵn một cái tên trong Python, và bạn dùng nó y như mọi hàm khác — tên hàm, rồi cặp ngoặc, rồi thứ cần xếp đặt trong ngoặc. Kết quả nó đưa ra là một danh sách mới, và dòng này hứng lấy danh sách ấy bằng cái tên `da_xep`.
- kind: one-line
  body: Viết `sorted(tien)` vào chỗ trống.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Sổ gốc: \[500000, 25000, 300000, 60000, 40000\]\nĐã xếp: \[25000, 40000, 60000, 300000, 500000\]\s*$
- tier: output
  expect: 'Sổ gốc: [500000, 25000, 300000, 60000, 40000]'
- tier: static
  onFail: dòng này phải nhờ máy xếp cuốn sổ, không phải gõ tay năm con số đã xếp sẵn
  requireAst:
  # Gõ thẳng `[25000, 40000, 60000, 300000, 500000]` vào chỗ trống thì cả hai
  # câu assert đều đạt — bộ số đúng, sổ gốc không bị đụng. Luật này là thứ duy
  # nhất phân biệt được "nhờ máy xếp" với "chép lại kết quả".
  - kind: uses-call, target: sorted, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cuốn sổ. Một cuốn cho thứ tự thời gian, một cuốn cho thứ tự tiền bạc.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vừa rồi bạn xếp một dãy **số**. Muốn xếp được, bạn đã phải tách cột tiền ra khỏi
cột tên — nên cái hàng đã xếp kia nói cho bạn biết khoản tốn nhất là **năm trăm
nghìn**, mà không nói khoản ấy tên gì.

Vậy thì đừng tách nữa. Bạn đã biết `chi.items()` cho ra từng cặp
`(tên, tiền)`, và một danh sách các cặp thì cũng là một danh sách.

Đem `sorted` chạy thẳng trên danh sách **cặp** ấy — đoán trước khi chạy: nó xếp
theo tên hay theo tiền?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
