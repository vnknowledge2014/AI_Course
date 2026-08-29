---
id: nen-tang.gia-tri-bien-kieu.muon-cat-so-phai-doi-thanh-chu
title: Muốn đếm con số, phải đổi nó thành chữ
summary: Con số không phải một dãy nên `len` từ chối nó; `str()` viết con số ấy ra thành chữ, và lúc đó mới đếm được.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.str-cast]
requires: [core.int-cast, err.type-error]
concepts: [core.doi-kieu, core.chuoi]
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
Con số thì mình có một. Chữ số để viết nó ra thì mình phải viết mới có.
::::

::::explain{#con-so-khong-phai-mot-day}
Bài trước kết bằng một lời rủ: gõ `len(25000)` xem máy nói gì. Nó không cho về
`5`. Nó dừng lại.

Để hiểu vì sao, nhìn lại `len` đã dùng được ở đâu. `len("cà phê")` cho `6` —
chuỗi là một **dãy ký tự**: có ký tự thứ nhất, ký tự thứ hai, có chỗ đầu và chỗ
cuối. Danh sách bạn gặp ở Realm 0 cũng xếp hàng đúng kiểu ấy, mỗi khoản một
chỗ, và `len` đo được nó y như đo chuỗi. Cái thước này đo **những thứ xếp
hàng**.

`25000` không xếp hàng. Nó là **một** giá trị, một lượng — bằng đúng số tiền
một cốc cà phê, không hơn không kém. Hỏi "nó có mấy phần tử" là hỏi một câu
không có nghĩa, y như hỏi cân nặng của bạn dài mấy chữ.

Vậy còn năm chữ số `2`, `5`, `0`, `0`, `0` mà bạn nhìn thấy? Chúng không nằm
trong con số. Chúng nằm trong **cách viết** con số ấy ra giấy. Cùng một lượng
ấy, người La Mã viết thành một dãy ký tự khác hẳn, và số ký tự cũng khác.

Đây là hai thứ tách rời nhau, và bài này đứng đúng trên chỗ tách rời đó:

- **Con số** — thứ để cộng, trừ, chia. Không có ký tự nào bên trong.
- **Dãy chữ số viết ra con số ấy** — thứ để đếm, để cắt, để xếp cho vừa cột.

Muốn có cái thứ hai, bạn phải bảo máy **viết con số ra**. Lệnh đó là **`str()`**
— viết tắt của *string*, đúng cái tên kiểu chuỗi mà bạn đã quen từ Realm 0.

`str()` là anh em soi gương với `int()` bạn dùng cả chục bài nay: `int("25000")`
đọc một dãy chữ số rồi cho ra con số; `str(25000)` cầm con số rồi viết ra dãy
chữ số. Một cái đọc vào, một cái viết ra.
::::

::::example{#thuoc-do-tu-choi-con-so}
Trước hết, xem đúng câu bài trước rủ bạn gõ:

```python title=readonly
tien = 25000
print(len(tien))
```

```text
Traceback (most recent call last):
  File "so_chi_tieu.py", line 2, in <module>
    print(len(tien))
          ~~~^^^^^^
TypeError: object of type 'int' has no len()
```

Dòng dưới cùng nói đúng chuyện vừa bàn, dịch sát: *một vật thuộc kiểu `int` thì
không có độ dài*. Không phải máy chưa biết đếm — mà thứ bạn đưa cho nó không có
gì để đếm.

Thêm một chữ là xong:

```python title=readonly
tien = 25000
chu_so = str(tien)

print(chu_so)
print(len(chu_so))
print(tien + 1000)
```

Màn hình:

```text
25000
5
26000
```

Ba dòng, ba chuyện.

Dòng đầu in ra `25000` — nhìn y hệt như in thẳng `tien`, và có lý do: `print`
vẫn luôn gọi `str` giúp bạn trước khi in. Bạn đã dùng `str` hàng trăm lần mà
không biết; hôm nay chỉ là lần đầu bạn tự gọi nó ra.

Dòng thứ hai cho `5`. Cái thước `len` không đổi tính nết chút nào — chỉ là lần
này nó được đưa cho một dãy ký tự thật.

Dòng thứ ba là chỗ quan trọng nhất: `tien` vẫn cộng được, vẫn ra `26000`. Đúng
luật bạn đã gặp ở phương thức chuỗi — `str(tien)` **trả về một giá trị mới** và
không hề đụng vào `tien`. Sổ vẫn giữ tiền bằng số nguyên để mà tính; dãy chữ số
chỉ là bản chép ra để đo cho vừa cột.

`str()` nhận mọi thứ, không riêng số. `str(45000.0)` cho `"45000.0"`,
`str(True)` cho `"True"`, và `str(None)` — cái giá trị "chưa có gì" của bài
trước — cho `"None"`. Bất kỳ giá trị nào cũng viết ra chữ được.
::::

::::predict{#doan-con-so-va-day-chu-so commitOnce}
Byte cầm số tiền một khoản, viết nó ra thành dãy chữ số, rồi vẫn tính tiếp trên
con số cũ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
tien = 45000
ma_so = str(tien)
print(len(ma_so))
print(ma_so + "đ")
print(tien + 1)
```

:::opt{correct}
5, rồi 45000đ, rồi 45001
:::

:::opt
5, rồi 45000đ, rồi một lỗi TypeError ở dòng cuối
::why
Gần đúng ở chỗ bạn theo dõi kiểu của từng giá trị rất sát: bạn thấy `tien` đã
đi qua `str` và kết luận nó giờ là chữ, mà chữ cộng với số thì `TypeError` —
đúng luật bạn học từ Realm 0.

Chỗ lệch nằm ở chỗ `str` đặt kết quả vào đâu. Nó **không sửa `tien`**, nó trả
về một giá trị mới, và giá trị mới ấy được hứng lấy dưới cái tên `ma_so`. Sau
dòng 2, trong chương trình có hai vật: `tien` vẫn là số `45000`, `ma_so` là dãy
chữ `"45000"`. Nên `tien + 1` vẫn là phép cộng số, ra `45001`.
::
:::

:::opt
5, rồi một lỗi TypeError ở dòng `print(ma_so + "đ")`, chương trình dừng ở đó
::why
Gần đúng ở chỗ bạn nhớ chính xác luật hai vế phải cùng kiểu: cộng một con số
với một câu chữ thì máy không có quy ước nào để chọn giữa tính và ghép, nên nó
dừng. Luật ấy có thật và bạn áp đúng chỗ.

Chỗ lệch là kiểu của `ma_so`. Nó không còn là số nữa — `str(tien)` đã viết con
số ấy ra thành dãy ký tự, nên `ma_so` là chuỗi `"45000"`. Hai vế của dấu `+`
lúc này cùng là chữ, và với hai câu chữ thì `+` nghĩa là ghép: `45000đ`.
::
:::

:::opt
7, rồi 45000đ, rồi 45001
::why
Gần đúng ở chỗ bạn nhớ rằng một chuỗi trong code luôn nằm giữa hai dấu nháy —
`"45000"` viết ra đúng là bảy ký hiệu trên màn hình, bạn đếm không sai một cái
nào.

Chỗ lệch: dấu nháy là **cách viết** một chuỗi trong code, không phải ký tự nằm
trong chuỗi. Nó là bờ rào chỉ chỗ chuỗi bắt đầu và kết thúc, giống như dấu
ngoặc của `print` không bị in ra. `len` đếm phần nằm bên trong hàng rào, nên
`5`.
::
:::
::::

::::code{#do-be-rong-cot-tien}
Cột "tiền" trên sổ rộng 10 chỗ. Byte cần biết mỗi khoản chiếm bao nhiêu chỗ
trong cột ấy — sau này còn tính xem phải chèn bao nhiêu khoảng trắng cho các
dòng thẳng hàng nhau.

Ba khoản có độ dài khác hẳn nhau: `25000` (cà phê), `5000` (gửi xe) và
`1250000` (đi chợ). Ba chỗ trống nằm trong ngoặc của `len`. Điền vào đó thứ mà
`len` đo được.

Ba con số khác nhau ở đây là cố ý: chấm bằng một khoản thì không phân biệt nổi
đúng với gặp may, còn ba khoản 5, 4 và 7 chữ số thì một đáp án chép cứng sẽ sai
ít nhất hai chỗ.

```python title=starter
tien_ca_phe = 25000
tien_gui_xe = 5000
tien_di_cho = 1250000

cho_ca_phe = len(___)
cho_gui_xe = len(___)
cho_di_cho = len(___)

print(cho_ca_phe, cho_gui_xe, cho_di_cho)
```

```python title=solution
tien_ca_phe = 25000
tien_gui_xe = 5000
tien_di_cho = 1250000

cho_ca_phe = len(str(tien_ca_phe))
cho_gui_xe = len(str(tien_gui_xe))
cho_di_cho = len(str(tien_di_cho))

print(cho_ca_phe, cho_gui_xe, cho_di_cho)
```

```python title=test
# Ba khoản, ba độ dài khác nhau — 5, 4 và 7 chữ số. Một chỗ trống điền bừa cho
# ra cùng một con số ở cả ba dòng thì trượt ít nhất hai assert.
assert cho_ca_phe == 5, "khoản cà phê 25000 đồng viết ra giấy chiếm 5 chỗ trong cột"
assert cho_gui_xe == 4, "khoản gửi xe 5000 đồng ít hơn một chữ số nên chỉ chiếm 4 chỗ"
assert cho_di_cho == 7, "khoản đi chợ 1250000 đồng chiếm 7 chỗ — vẫn lọt cột mười chỗ"
# Và ba khoản tiền phải còn là SỐ để còn cộng trừ được: `str` viết ra bản chép,
# nó không biến cái tên cũ thành chữ.
assert tien_ca_phe + tien_gui_xe == 30000, "cà phê với gửi xe cộng lại là 30 nghìn: viết một khoản ra thành chữ chỉ là chép lại, khoản tiền vẫn phải cộng trừ được"
```

:::hints
- kind: attention
  body: Nhìn ba dòng đầu — `25000`, `5000`, `1250000` đều viết không có dấu nháy, nên chúng là số. Cái thước `len` vừa từ chối đúng loại giá trị này ở phần ví dụ.
- kind: strategy
  body: Bạn cần đưa cho `len` một dãy ký tự chứ không phải một con số, mà dãy ký tự ấy phải là bản viết ra của chính con số đang có tên. Có một lệnh làm đúng việc viết ra đó, và nó đọc ngược lại với `int()` bạn dùng cho `input()`.
- kind: one-line
  body: "Điền `str(tien_ca_phe)` vào chỗ trống thứ nhất, `str(tien_gui_xe)` vào chỗ thứ hai, `str(tien_di_cho)` vào chỗ thứ ba."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^5 4 7\s*$
- tier: static
  onFail: mỗi chỗ trống cần một lời gọi viết con số ra thành chữ, không phải một dãy chữ số chép cứng
  requireAst:
  - kind: uses-call, target: str, min: 3
:::
::::

::::byte{trigger=success mood=happy pose=jump}
5, 4, 7 — đều lọt cột mười chỗ. Giờ mình biết chừa bao nhiêu khoảng trắng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ba khoản vừa rồi đều vừa cột. Nhưng báo cáo cuối tháng còn một dòng nữa: **trung
bình mỗi ngày tiêu bao nhiêu**.

Đây là chỗ đúng luật để một số thực xuất hiện. Tiền từng khoản thì bạn ĐẾM bằng
đồng nên giữ số nguyên; còn trung bình là một phép ĐO, nó không có nghĩa là một
khoản tiền ai đó thật sự trả. Tổng tháng `1360000` chia cho `30` ngày:

```python
trung_binh = 1360000 / 30
print(trung_binh)
print(len(str(trung_binh)))
```

Máy in ra `45333.333333333336` — cái đuôi dài của bài số thực xấp xỉ quay lại
chào bạn — rồi `18`. Mười tám chỗ, trong khi cột chỉ có mười.

Cắt bớt bằng lát cắt thì được `45333.3333`, nhưng đó là cắt **chữ**: nó không
biết đâu là dấu chấm, và đổi cột thành 8 chỗ thì ra `45333.33` còn đổi thành 6
chỗ thì ra `45333.`, cụt lủn. Bạn muốn nói một câu khác hẳn: *in cho tôi đúng
hai chữ số sau dấu chấm*.

Bạn đã có f-string từ Realm 0 để chèn giá trị vào câu. Trong cặp `{}` ấy, bạn
mới chỉ đặt vào một cái tên. Có chỗ nào trong đó để dặn thêm **viết ra thế nào**
không? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
