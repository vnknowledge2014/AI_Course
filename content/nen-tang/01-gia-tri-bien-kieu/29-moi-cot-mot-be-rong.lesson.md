---
id: nen-tang.gia-tri-bien-kieu.moi-cot-mot-be-rong
title: Mỗi cột một bề rộng
summary: Con số đặt trong phần định dạng là bề rộng của ô — chữ ngắn thì máy chèn khoảng trắng cho đủ chỗ.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 29
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.field-width]
requires: [core.fstring, core.thousands-sep]
concepts: [core.dinh-dang, core.chuoi]
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
Cuốn sổ giấy kẻ cột trước rồi mới ghi. Mình cũng làm được đúng vậy.
::::

::::explain{#cot-ke-truoc-roi-moi-ghi}
Ba dòng sổ của bài trước dễ đọc từng dòng một, nhưng nhìn cả ba thì cột tiền
chạy loạn:

```text
cà phê 25,000đ
bún chả 1,250,000đ
gửi xe 5,000đ
```

Lỗi không nằm ở con số. Nó nằm ở chỗ mỗi dòng bắt đầu ghi tiền ngay sau khi tên
khoản hết chữ — mà tên khoản dài ngắn khác nhau, nên chỗ ấy mỗi dòng một khác.

Cuốn sổ giấy không làm thế. Người ta kẻ cột **trước**: cột tên khoản rộng 12
chỗ, cột tiền rộng 10 chỗ. Ghi "cà phê" hết 6 chỗ thì 6 chỗ còn lại của ô bỏ
trống, chứ ô không co lại theo chữ. Nhờ vậy mọi dòng đều bắt đầu và kết thúc ở
cùng một mốc.

Phần định dạng nói được đúng chuyện đó. Bạn đặt vào đó một **con số**, và con
số ấy là **bề rộng của ô** — tính bằng số chỗ:

```python
f"{ten:12}"
```

Đọc: *dành cho giá trị này một ô rộng 12 chỗ.* Nội dung ngắn hơn ô thì máy chèn
khoảng trắng vào cho đủ 12. Tiếng Anh gọi con số này là *field width*, còn ta
gọi gọn: bề rộng ô.

Chú ý một chuyện dễ nhầm: con số này không nói giá trị có mấy chữ số, cũng
không liên quan gì tới `.2f` hay dấu phẩy. Nó chỉ nói **ô rộng bao nhiêu chỗ**.
::::

::::example{#mot-o-rong-12-cho}
Dấu ngoặc vuông trong hai dòng dưới đây là của Byte, đặt sát hai đầu ô để bạn
nhìn thấy ô rộng tới đâu:

```python title=readonly
ten = "cà phê"
tien = 25000

print(f"[{ten}]")
print(f"[{ten:12}]")
print(f"[{tien:10}]")
```

```text
[cà phê]
[cà phê      ]
[     25000]
```

Dòng đầu không có ô: máy in đúng 6 ký tự rồi thôi. Dòng hai có ô 12 chỗ: 6 ký
tự chữ, 6 khoảng trắng chèn thêm. Dòng ba có ô 10 chỗ: `25000` chiếm 5, còn 5
khoảng trắng.

Để ý chỗ khoảng trắng rơi vào: với **chữ** nó nằm bên phải, với **số** nó nằm
bên trái. Giữ ý đó lại, phần sau sẽ nói vì sao.

Bây giờ ghép hai ô lại thành một dòng sổ, và in cả ba khoản:

```python title=readonly
print(f"{'cà phê':12}{25000:10,}đ")
print(f"{'bún chả':12}{1250000:10,}đ")
print(f"{'gửi xe':12}{5000:10,}đ")
```

```text
cà phê          25,000đ
bún chả      1,250,000đ
gửi xe           5,000đ
```

Chữ `đ` của ba dòng rơi vào đúng một cột, vì ô tên luôn hết ở chỗ thứ 12 và ô
tiền luôn hết ở chỗ thứ 22 — bất kể bên trong ô có bao nhiêu chữ. Cột tiền dồn
về bên phải nên hàng đơn vị của ba con số thẳng nhau, cộng nhẩm được ngay.
::::

::::predict{#doan-o-rong commitOnce}
Byte in một khoản mới, mỗi ô một dòng, vẫn kẹp giữa hai dấu ngoặc vuông.
Lần này tên khoản **dài hơn** ô đã kẻ: `"cà phê sữa đá"` có 13 ký tự trong khi
ô rộng 12. Còn `125000` thì 6 chữ số, vẫn lọt ô 10.

**Trước khi bấm chạy**, bạn đoán hai dòng nào hiện ra?

```python
ten = "cà phê sữa đá"
tien = 125000

print(f"[{ten:12}]")
print(f"[{tien:10}]")
```

:::opt{correct}
`[cà phê sữa đá]` rồi `[    125000]`
:::

:::opt
`[cà phê sữa đ]` rồi `[    125000]`
::why
Gần đúng ở dòng dưới, và gần đúng ở một suy luận rất hợp lý cho dòng trên: ô
rộng 12 thì cái gì quá 12 phải bị cắt cho vừa, y như một cái khay có vách.

Chỗ lệch: con số ấy là chỗ **tối thiểu**, không phải chỗ tối đa. Nội dung ngắn
hơn thì máy chèn khoảng trắng cho đủ; nội dung dài hơn thì máy để nguyên và ô
nở ra. Nó thà làm lệch cột còn hơn cắt mất chữ của bạn — vì một cột lệch thì
bạn nhìn ra ngay, còn một cái tên bị cắt cụt thì trông vẫn như một cái tên.
::
:::

:::opt
`[cà phê sữa đá]` rồi `[125000    ]`
::why
Gần đúng ở dòng trên: tên dài hơn ô nên ô nở ra, bạn đọc đúng.

Chỗ lệch nằm ở dòng dưới, ở chuyện chèn khoảng trắng vào **bên nào**. Máy chọn
bên theo kiểu của giá trị: chữ dạt về trái, số dạt về phải. Đó là mặc định của
Python, và ở đây nó rơi đúng vào ý bạn muốn — tên khoản đọc từ lề trái, tiền
so hàng đơn vị từ lề phải.
::
:::

:::opt
`[cà phê sữa đá]` rồi `[125000]`
::why
Gần đúng ở dòng trên, và gần đúng ở một cách nghĩ liền mạch cho dòng dưới: nếu
ô không ép được cái tên 13 ký tự vào 12 chỗ, thì có lẽ ô chỉ là một lời đề
nghị, và máy cứ in nguyên thứ nó có.

Chỗ lệch: ô là chỗ **tối thiểu**. Hai dòng ấy chạy theo hai vế khác nhau của
cùng một luật, chứ không phải hai luật. Tên 13 ký tự đã quá 12 nên không còn
gì để chèn — ô nở ra. Còn `125000` mới 6 chữ số, chưa đầy ô 10, nên máy chèn
thêm bốn khoảng trắng cho đủ. Ô chỉ buông tay khi nội dung đã vượt nó.
::
:::
::::

::::explain{#o-day-cho-va-ben-nao}
Hai điều cần nói rõ trước khi bạn tự kẻ cột.

**Bề rộng là chỗ tối thiểu, không phải chỗ tối đa.** Nội dung dài hơn ô thì máy
in trọn, không cắt bớt chữ nào — và ô ấy phình ra, kéo lệch cả dòng:

```python title=readonly
print(f"{'cà phê sữa đá':12}{5000:10,}đ")
print(f"{'gửi xe':12}{5000:10,}đ")
```

```text
cà phê sữa đá     5,000đ
gửi xe           5,000đ
```

`"cà phê sữa đá"` có 13 ký tự, ô chỉ có 12 chỗ, nên dòng trên đội lên một chỗ.
Máy không tự quyết định bỏ chữ nào của bạn. Muốn ô đúng 12 thì bạn phải tự cắt
tên trước khi đưa vào — bằng lát cắt `ten[0:12]`, đúng công cụ đã học ở bài cắt
khúc chuỗi.

**Bên chèn khoảng trắng là mặc định theo kiểu, không phải lựa chọn của bạn.**
Chữ dạt trái, số dạt phải. Ở cuốn sổ này mặc định ấy đang trùng với ý bạn, nên
bài này không phải làm gì thêm.

Nhưng nó là **mặc định**, và mặc định thì đổi khi giá trị đổi kiểu — im lặng,
không báo một tiếng. Ngày nào cột tiền của bạn nhận một chuỗi thay vì một số —
chẳng hạn `"—"` cho khoản chưa rõ giá — cả ô ấy sẽ lật sang trái trong khi các
dòng khác vẫn dạt phải.

Python có hai ký hiệu đặt ngay **trước** con số bề rộng để bạn tự nói ra bên
nào: `<` dạt trái, `>` dạt phải. `f"{tien:>10}"` là "ô 10 chỗ, dạt phải, dù bên
trong là gì đi nữa". Việc đó gọi là **căn lề** (*alignment*). Bài này chưa cần
tới nó vì mặc định đang đúng ý; điều đáng nhớ bây giờ là **vì sao** nó đúng ý —
và rằng nó sẽ thôi đúng ý vào ngày kiểu trong ô thay đổi.
::::

::::code{#hai-dong-thang-cot}
Byte ghi hai khoản của hôm nay vào sổ: cà phê `25000` đồng và bún chả `1250000`
đồng. Cột tên khoản rộng **12** chỗ, cột tiền rộng **10** chỗ — đúng hai con số
mà cuốn sổ đã kẻ sẵn từ mấy bài trước.

Bốn chỗ trống, mỗi dòng hai chỗ: một cho ô tên, một cho ô tiền. Dấu phẩy nằm
sẵn trong ô tiền là dấu ngăn nhóm của bài trước, cứ để nguyên.

Hai khoản này chọn cố ý: tên dài ngắn khác nhau (6 và 7 ký tự) và tiền cũng dài
ngắn khác nhau (6 và 9 chỗ khi đã ngăn nhóm). Chấm bằng một dòng thì không phân
biệt nổi bề rộng đúng với một con số gặp may; hai dòng lệch nhau cả bốn phía thì
chỉ đúng bề rộng mới cho ra hai dòng thẳng cột.

```python title=starter
ten_1 = "cà phê"
tien_1 = 25000
ten_2 = "bún chả"
tien_2 = 1250000

print(f"{ten_1:___}{tien_1:___,}đ")
print(f"{ten_2:___}{tien_2:___,}đ")
```

```python title=solution
ten_1 = "cà phê"
tien_1 = 25000
ten_2 = "bún chả"
tien_2 = 1250000

print(f"{ten_1:12}{tien_1:10,}đ")
print(f"{ten_2:12}{tien_2:10,}đ")
```

```python title=test
# Chấm bằng TRỌN VẸN màn hình (`match: regex`) trên HAI dòng, đếm đúng từng
# khoảng trắng. Hai dòng ấy bắt được mọi câu trả lời hụt:
#   bỏ trống bề rộng (điền 0 hay 1) → hai dòng dính liền, chữ đ mỗi dòng một chỗ;
#   điền 12 cho cả bốn chỗ          → ô tiền rộng 12, chữ đ lùi ra hai chỗ;
#   điền đúng ở dòng một, sai ở dòng hai → hai dòng lệch nhau, lộ ngay.
# Chỉ 12 và 10 mới cho ra hai chữ đ nằm cùng một cột thứ 22.
pass
```

:::hints
- kind: attention
  body: Mỗi chỗ trống nằm ngay sau một dấu hai chấm, và nó chờ một con số. Con số ấy là số chỗ của ô — đề bài đã cho sẵn hai con số này ở đoạn trên.
- kind: strategy
  body: Ô tên và ô tiền có bề rộng khác nhau, nhưng hai dòng thì phải dùng cùng một cặp bề rộng; lệch một chỗ ở một dòng là cả cột lệch. Đừng đụng vào dấu phẩy đứng sau chỗ trống của ô tiền — đó là dấu ngăn nhóm, không phải bề rộng.
- kind: one-line
  body: "Hai chỗ trống của ô tên điền `12`, hai chỗ trống của ô tiền điền `10`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^cà phê {10}25,000đ\nbún chả {6}1,250,000đ\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai dòng, một cột. Giờ thêm bao nhiêu khoản nữa cũng thẳng hàng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại chặng đường của một dòng sổ. Bạn đã biết: nhận thứ người ta gõ vào và
biết nó luôn là chữ; cắt khoảng trắng thừa, đưa chữ hoa về chữ thường; đổi chữ
thành số và chọn kiểu số cho đúng việc — `int` để đếm tiền, `float` cho phép đo;
chia phần và lấy phần dư; đặt tên cho giá trị, đặt tên VIẾT HOA cho hằng; phân
biệt "chưa có gì" với số 0; và bây giờ là in ra thành cột thẳng hàng.

Từng mảnh một bạn đều làm được. Nhưng suốt hai mươi chín bài, mỗi bài chỉ cầm
một mảnh.

Một dòng sổ thật thì cần cả xâu: người gõ `"  Bún Chả  "` và `"245.5"` nghìn,
quán tính thêm phần trăm phí phục vụ, bốn người ăn chung chia đều, ô ghi chú
chưa ai viết gì — và cuối cùng phải ra đúng một dòng thẳng cột.

Ghép tất cả lại thành một chương trình chạy được chưa? Bài sau là chỗ bạn tự
ghép.
::::

::::checkpoint{mastery=0.8}
::::
