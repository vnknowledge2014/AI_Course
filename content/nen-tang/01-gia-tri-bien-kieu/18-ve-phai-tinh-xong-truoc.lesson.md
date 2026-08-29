---
id: nen-tang.gia-tri-bien-kieu.ve-phai-tinh-xong-truoc
title: Vế phải tính xong hết rồi mới gán
summary: Dấu `=` không nói hai bên bằng nhau — máy làm trọn vế phải, cầm sẵn kết quả, rồi mới dán tên lên.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.rhs-first]
requires: [core.multi-assign, core.variable, core.assignment, core.reassign, core.name-lookup, core.fstring, core.arithmetic]
concepts: [core.bien, core.gan, core.thu-tu-buoc]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Mình làm xong hết bên phải, cầm kết quả trong tay, rồi mới đi dán tên.
::::

::::explain{#dau-bang-doc-tu-phai-sang}
Câu hỏi bỏ ngỏ của bài trước: hai dòng viết ra để đổi chỗ, mà chạy xong cả hai
cái tên cùng thành một giá trị.

```python title=readonly
tien_hom_qua = 30000
tien_hom_nay = 50000

tien_hom_qua = tien_hom_nay
tien_hom_nay = tien_hom_qua

print(f"Hôm qua {tien_hom_qua}đ, hôm nay {tien_hom_nay}đ")
```

```text
Hôm qua 50000đ, hôm nay 50000đ
```

Con số `30000` biến mất khỏi chương trình.

Chỗ hỏng nằm ở cách đọc dấu `=`. Trong vở toán, `a = b` là một câu **mô tả**:
hai bên bằng nhau, và câu ấy đúng theo cả hai chiều. Trong Python, `=` không
mô tả gì cả — nó là một **mệnh lệnh có thứ tự**, và thứ tự ấy chạy từ phải
sang trái, đúng hai bước:

1. Làm trọn **vế phải**. Đọc tên, tính toán, cho tới khi ra một giá trị nằm
   sẵn trong tay.
2. Xong rồi mới dán cái tên bên trái lên giá trị vừa cầm được.

Phần bên phải dấu `=` gọi là **vế phải** — tiếng Anh là *right-hand side*.
Hai bước trên không bao giờ đảo, và bước 2 không bao giờ chen vào giữa bước 1.

Soi lại hai dòng hỏng bằng luật ấy:

- Dòng `tien_hom_qua = tien_hom_nay`: vế phải đọc ra `50000`. Rồi mảnh giấy
  `tien_hom_qua` được gỡ khỏi `30000` và dán lên `50000`. Từ giây đó, không
  cái tên nào còn dán trên `30000` — và một giá trị không tên thì không có
  cách nào gọi lại.
- Dòng `tien_hom_nay = tien_hom_qua`: vế phải đọc `tien_hom_qua`, mà cái tên
  ấy vừa bị dán sang `50000` xong. Nên `tien_hom_nay` nhận lại đúng con số nó
  đang giữ.

Dòng thứ hai không sai cú pháp, cũng không sai chính tả. Nó chỉ đến muộn: thứ
nó cần đọc đã bị dòng trước xoá dấu vết.
::::

::::example{#mot-dong-cam-ca-hai}
Bài trước cho một dòng gán có nhiều tên bên trái và nhiều giá trị bên phải.
Đem luật "làm trọn vế phải trước" áp lên dòng ấy:

```python title=readonly
tien_hom_qua = 30000
tien_hom_nay = 50000

tien_hom_qua, tien_hom_nay = tien_hom_nay, tien_hom_qua

print(f"Hôm qua {tien_hom_qua}đ, hôm nay {tien_hom_nay}đ")
```

```text
Hôm qua 50000đ, hôm nay 30000đ
```

Đổi chỗ xong, và không con số nào mất.

Đi lại từng bước cho dòng ở giữa:

**Bước 1 — làm trọn vế phải.** Máy đọc `tien_hom_nay, tien_hom_qua`, **cả
hai**, và cầm sẵn cặp `50000, 30000`. Ở khoảnh khắc này chưa cái tên nào bị
đụng tới: `tien_hom_qua` vẫn đang dán trên `30000`.

**Bước 2 — dán tên.** Bây giờ mới tới lượt bên trái: `tien_hom_qua` dán lên
`50000`, `tien_hom_nay` dán lên `30000`.

Cặp giá trị đã nằm trong tay từ bước 1, nên việc dán tên ở bước 2 không thể
xoá mất thứ bước 1 vừa đọc. Đó là toàn bộ lý do một dòng làm được việc mà hai
dòng không làm nổi — không phải vì nó gõ ngắn hơn.

Luật này không chỉ dùng cho phép đổi chỗ. Nó đúng cho mọi dòng `=`, kể cả khi
vế phải là một phép tính dài:

```python title=readonly
tien_an = 60000
tien_an = tien_an + 25000
print(tien_an)
```

```text
85000
```

Dòng 2 nhìn như một câu toán vô lý — một số bằng chính nó cộng thêm. Đọc theo
hai bước thì không vô lý chút nào: vế phải `tien_an + 25000` được tính trọn
bằng giá trị **cũ**, ra `85000`; xong rồi cái tên mới được dán sang con số
mới.
::::

::::predict{#doan-hai-dong-doi-cho commitOnce}
Một dòng sổ khác, hai khoản khác, nhưng vẫn đúng hai dòng gán ấy. **Trước khi
bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
tien_sang = 20000
tien_trua = 65000

tien_sang = tien_trua
tien_trua = tien_sang

print(f"Sáng {tien_sang}đ, trưa {tien_trua}đ")
```

:::opt{correct}
Sáng 65000đ, trưa 65000đ
:::

:::opt
Sáng 65000đ, trưa 20000đ
::why
Gần đúng ở chỗ bạn đọc ra đúng ý định của hai dòng ấy: chúng được viết ra để
đổi chỗ, và đây chính là kết quả người viết muốn có. Ý định thì không sai chỗ
nào.

Chỗ lệch nằm ở dòng thứ hai. Lúc nó chạy, `tien_sang` không còn giữ `20000`
nữa — dòng trước vừa dán cái tên ấy sang `65000` xong. Nên vế phải của dòng
hai đọc ra `65000`, và `tien_trua` nhận lại đúng con số nó đang có. Muốn
kết quả này thì phải có ai đó còn giữ `20000` cho tới lúc ấy.
::
:::

:::opt
Sáng 20000đ, trưa 20000đ
::why
Gần đúng ở chỗ bạn thấy hai con số cuối cùng phải giống nhau — đúng, một giá
trị đã mất và cái còn lại chiếm cả hai cái tên. Bạn đọc trúng cái bẫy của đoạn
này.

Chỗ lệch là ở chỗ giá trị nào sống sót. Dòng `tien_sang = tien_trua`
chạy trước, và nó dán `tien_sang` sang giá trị của `tien_trua`, tức
`65000`. Cái mất là `20000`. Đọc mỗi dòng gán theo thứ tự phải-rồi-trái sẽ chỉ
ra ngay ai còn ai mất.
::
:::

:::opt
Sáng 20000đ, trưa 65000đ
::why
Gần đúng ở chỗ bạn ngờ rằng hai dòng ấy triệt tiêu nhau: dòng sau làm ngược
lại dòng trước, nên sổ quay về như cũ. Với hai việc thật sự đối xứng thì cách
nghĩ đó đúng.

Chỗ lệch: dòng sau không làm ngược lại dòng trước, vì nó đọc `tien_sang`
**sau khi** dòng trước đã dán cái tên ấy đi rồi. Muốn quay về như cũ thì phải
còn ai đó giữ `20000` — mà từ cuối dòng thứ nhất, không cái tên nào giữ nó
nữa.
::
:::
::::

::::code{#doi-cho-hai-ngay}
Sổ ghi ngược hai ngày: `tien_hom_qua` đang giữ con số của hôm nay, và ngược
lại. Hãy đổi chỗ hai cái tên bằng **một** dòng gán.

Hai dòng gán ở trên là dữ liệu của tháng này — đừng sửa chúng, và cũng đừng
viết thẳng con số ra. Sổ tháng sau sẽ mang hai con số khác, mà dòng bạn viết
phải chạy đúng cho mọi cặp số.

```python title=starter
tien_hom_qua = 50000
tien_hom_nay = 30000

___

print(f"Hôm qua {tien_hom_qua}đ, hôm nay {tien_hom_nay}đ")
```

```python title=solution
tien_hom_qua = 50000
tien_hom_nay = 30000

tien_hom_qua, tien_hom_nay = tien_hom_nay, tien_hom_qua

print(f"Hôm qua {tien_hom_qua}đ, hôm nay {tien_hom_nay}đ")
```

```python title=test
# Hai cái tên, hai con số khác nhau, và chúng phải ĐỔI CHỖ chứ không phải cùng
# nhận một giá trị. Bỏ trống hoặc điền bừa thì dòng đầu trượt; đổi nửa vời —
# kiểu `tien_hom_qua = tien_hom_nay` — thì dòng thứ hai trượt.
assert tien_hom_qua == 30000, "sau khi đổi, hôm qua phải là 30000"
assert tien_hom_nay == 50000, "sau khi đổi, hôm nay phải là 50000"
```

:::hints
- kind: attention
  body: Dòng bạn cần viết có hai cái tên ở bên trái dấu `=`, đúng hình dạng dòng gán nhiều của bài trước. Việc còn lại là quyết định bên phải viết hai cái tên ấy theo thứ tự nào.
- kind: strategy
  body: Vế phải được làm trọn trước, nên lúc máy đọc nó thì hai cái tên vẫn đang giữ giá trị cũ. Bạn muốn `tien_hom_qua` nhận giá trị cũ của `tien_hom_nay`, nên bên trái xếp thế nào thì bên phải xếp ngược lại.
- kind: one-line
  body: "Viết `tien_hom_qua, tien_hom_nay = tien_hom_nay, tien_hom_qua` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Hôm qua 30000đ, hôm nay 50000đ
- tier: static
  onFail: dòng bạn điền phải ĐỌC cả hai cái tên cũ, không chép cứng hai con số ra
  requireAst:
  # `min: 2` cho mỗi tên: dòng `print` cuối đã đọc mỗi tên một lần, nên lần đọc
  # thứ hai chỉ có thể đến từ vế phải của dòng đổi chỗ. Chép cứng `30000` và
  # `50000` vào hai dòng gán thì mỗi tên chỉ được đọc đúng một lần, và luật này
  # chặn lại.
  - kind: uses-name, target: tien_hom_qua, min: 2
  - kind: uses-name, target: tien_hom_nay, min: 2
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cầm sẵn cả hai rồi mới dán tên. Không con số nào rơi mất giữa đường.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Con số và cái tên thì bạn đã cầm chắc: chọn được kiểu, đặt được tên, khai được
hằng, và biết máy làm gì với mỗi dấu `=`.

Còn **tên khoản** thì chưa. Khách đọc "Cà Phê", bạn ghi sổ là "cà phê", rồi
lúc đối chiếu hai cuốn sổ bạn viết:

```python
print("Cà Phê" == "cà phê")
```

Máy trả lời `False`. Hai chuỗi ấy, người nào đọc lên cũng thành một tên món.
Máy thì không.

Vậy khi so hai chuỗi, máy so cái gì với cái gì mà chặt đến thế?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
