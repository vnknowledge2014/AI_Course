---
id: nen-tang.gia-tri-bien-kieu.tra-tung-ky-tu-mot
title: Tra từng ký tự một
summary: Số âm trong ngoặc vuông đếm ngược từ cuối dãy, nên lấy được ký tự cuối mà không cần biết chuỗi dài bao nhiêu.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.negative-index]
requires: [core.string-sequence, core.len, core.list-index, ctrl.else]
concepts: [core.chuoi, core.chi-so]
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
Đếm từ đầu thì phải biết dây dài bao nhiêu. Đếm từ cuối thì không.
::::

::::explain{#cho-dung-cua-mot-ky-tu}
Chuỗi là một dãy có thứ tự, nên mỗi ký tự trong đó có một **chỗ đứng** riêng.
Và cách xin một chỗ thì bạn đã biết rồi.

Ở Realm 0 bạn gọi một món trong danh sách bằng chỗ đứng của nó: viết tên, mở
ngoặc vuông, đặt số chỗ vào trong, và chỗ đầu tiên mang số `0`. Chuỗi dùng đúng
cái ngoặc vuông ấy, đúng cách đếm ấy:

```python
ten = "cà phê"
ten[0]   # hạt đầu tiên
ten[1]   # hạt thứ hai
```

Không có gì mới ở hai dòng trên — chỉ là cùng một cái ngoặc vuông, lần này đặt
lên một xâu hạt thay vì một rổ đồ.

Cái mới nằm ở đầu bên kia của dây.

Bài trước để Byte lại với việc kiểm dòng sổ: `cà phê 25000đ` — có kết đúng bằng
chữ `đ` không? Chỉ cần nhìn **hạt cuối cùng**. Nhưng hạt cuối cùng là chỗ số
mấy?

Dãy này có 13 ký tự, và vì chỗ đầu mang số 0 nên chỗ cuối mang số 12. Viết
`dong_so[12]` thì chạy đúng — cho đúng dòng này. Dòng sổ kế tiếp dài 11 ký tự,
`[12]` lập tức sai. Chỗ cuối không phải một con số cố định: nó đổi theo từng
chuỗi.

Cách viết đúng cho mọi dòng thì có, và nó dùng đúng thứ bài trước vừa dạy:

```python
dong_so[len(dong_so) - 1]
```

Chạy được thật. Nhưng nhìn xem nó bắt bạn làm gì: gõ tên `dong_so` hai lần trên
cùng một dòng, gọi `len`, rồi nhớ trừ đi một. Ba việc, chỉ để nói một câu rất
ngắn: *hạt cuối*.

Python có một lối tắt cho đúng câu ngắn ấy: **chỉ số âm**.

```python
dong_so[-1]   # hạt cuối cùng
dong_so[-2]   # hạt kế cuối
```

Dấu trừ trong ngoặc vuông **không phải một phép trừ**. Bên trong ngoặc vuông,
con số luôn là một **chỗ đứng**, và dấu âm chỉ đổi **chiều đếm**:

- Số dương: đếm xuôi, xuất phát từ đầu dây, chỗ đầu tiên là `0`.
- Số âm: đếm ngược, xuất phát từ cuối dây, chỗ cuối cùng là `-1`.

Vì sao chiều ngược bắt đầu ở `-1` mà không phải `-0`? Vì `0` đã có chủ — nó là
chỗ đầu tiên rồi, và không có con số nào vừa là `0` vừa là `-0` để mang nghĩa
khác. Nên chiều ngược bắt đầu ngay ở số âm gần nhất: `-1`.

Cái được của lối tắt này: `-1` là hạt cuối của **mọi** chuỗi, dài ngắn thế nào
cũng vậy. Bạn không cần biết dây dài bao nhiêu mới sờ được vào cái đuôi của nó.
::::

::::example{#xem-tan-mat}
Đặt hai chiều đếm cạnh nhau trên cùng một chuỗi sáu ký tự:

```python title=readonly
ten = "cà phê"

print(ten[0])
print(ten[1])
print(ten[-1])
print(ten[-2])
```

Màn hình hiện ra:

```text
c
à
ê
h
```

Bảng dưới đây là cùng một xâu hạt, đọc theo hai chiều. Hàng trên là số chỗ đếm
xuôi, hàng dưới là số chỗ đếm ngược — ô thứ ba là một dấu cách:

```text
  c    à         p    h    ê
  0    1    2    3    4    5
 -6   -5   -4   -3   -2   -1
```

Mỗi hạt có hai cái tên gọi, và cả hai đều chỉ đúng nó. `ten[4]` với `ten[-2]`
là cùng một chữ `h`.

Còn bờ của cái thang này thì sao? Đúng luật bạn đã gặp với danh sách: xin một
chỗ mà dãy không có thì máy dừng lại và nói ra.

```python
ten = "cà phê"
print(ten[20])
```

```text
IndexError: string index out of range
```

Chiều ngược cũng có bờ của nó. Dãy sáu hạt thì chỗ xa nhất về phía sau là `-6`;
`ten[-7]` cho ra đúng cái `IndexError` ấy. Sáu hạt là sáu hạt, đếm chiều nào
cũng không đẻ thêm ra hạt thứ bảy.
::::

::::predict{#doan-ky-tu-cuoi commitOnce}
Byte lấy hạt cuối của một dòng sổ bằng **hai** cách khác nhau: một dòng dùng
lối tắt, một dòng dùng phép trừ. Chuỗi này có 13 ký tự.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra hai dòng nào?

```python
dong_so = "cà phê 25000đ"

print(dong_so[-1])
print(dong_so[len(dong_so) - 1])
```

:::opt{correct}
đ rồi đ — hai dòng in ra y hệt nhau
:::

:::opt
đ rồi 0
::why
Gần đúng ở chỗ bạn tính `len(dong_so) - 1` ra đúng số 12 — phép trừ ấy chính
xác, và dòng đầu bạn cũng đoán trúng.

Chỗ lệch nằm ở cách đọc con số 12. Trong ngoặc vuông, `12` không có nghĩa "ký
tự thứ mười hai" mà là "chỗ số 12". Vì chỗ đầu tiên mang số 0 nên chỗ số 12
chính là ký tự **thứ mười ba** — hạt cuối cùng. Phép trừ một ở đó tồn tại đúng
để bù cho cái chỗ số 0, nên hai dòng gặp nhau ở cùng một hạt.
::
:::

:::opt
cà phê 25000 rồi đ
::why
Gần đúng ở chỗ bạn đọc dấu trừ như một phép trừ: bớt của chuỗi đi một ký tự ở
cuối. Có công cụ làm đúng việc "lấy một khúc" ấy thật, và bạn sẽ gặp nó ở bài
cắt khúc chuỗi.

Chỗ lệch: bên trong ngoặc vuông, con số không phải một phép tính đặt lên chuỗi
— nó là một **chỗ đứng**. Xin một chỗ thì nhận về đúng thứ nằm ở chỗ đó, tức
đúng **một** ký tự, không bao giờ nhiều hơn.
::
:::

:::opt
Dòng đầu báo lỗi IndexError, không in ra hạt nào
::why
Gần đúng ở chỗ bạn nhớ luật đã gặp với danh sách: xin một chỗ mà dãy không có
thì máy báo `IndexError` — và `-1` trông đúng là một chỗ nằm ngoài dãy, vì dãy
bắt đầu ở 0.

Chỗ lệch: Python dành riêng các số âm cho chiều đếm ngược, nên `-1` không phải
"chỗ nằm trước chỗ 0" mà là "chỗ cuối dây". Dãy 13 ký tự này có các chỗ hợp lệ
chạy từ `-13` tới `12`. Ra ngoài khoảng ấy — `dong_so[-20]` chẳng hạn — mới là
`IndexError` thật.
::
:::
::::

::::explain{#vi-sao-lam-tat-dang-gia}
Hai cách viết cho cùng một hạt, vậy chọn cách nào cũng được?

Được, cho tới lúc các dòng sổ dài ngắn khác nhau — mà chúng luôn khác nhau.
`cà phê 25000đ` có 13 ký tự, `gửi xe 5000` có 11. Với `[-1]` bạn viết **một**
câu duy nhất và nó đúng cho cả hai dòng, cho cả những dòng chưa ai ghi. Với
phép trừ, bạn phải kéo `len` của đúng cái tên ấy vào từng dòng, và chép nhầm
tên là câu lệnh đọc sai chuỗi mà không báo lỗi gì cả.

Ngắn hơn ở đây không phải chuyện gõ ít phím. Nó là chuyện câu lệnh nói thẳng ra
điều bạn muốn: *hạt cuối*, chứ không phải *hạt ở chỗ bằng độ dài trừ một*.
::::

::::code{#dong-so-co-ket-bang-d-khong}
Byte rà lại sổ. Mỗi dòng chi tiêu phải kết đúng bằng chữ `đ`; dòng nào quên thì
phải đánh dấu để ghi lại. Trong sổ có hai dòng, và chúng dài ngắn khác nhau.

Hai chỗ trống, mỗi khối một chỗ, đều nằm ngay trước dấu `==`. Điền vào đó thứ
lấy ra **hạt cuối** của dòng tương ứng.

Bài chấm bằng cả hai dòng, và thêm một luật nữa đòi câu trả lời phải thật sự
**đọc từ cái tên** chứ không gõ sẵn ký tự vào. Chấm bằng một dòng thì không
phân biệt được: gõ thẳng `"đ" == "đ"` cũng cho ra đúng câu mà dòng 1 cần.

```python title=starter
dong_1 = "cà phê 25000đ"
dong_2 = "gửi xe 5000"

if ___ == "đ":
    print("dòng 1 — kết đúng bằng đ")
else:
    print("dòng 1 — thiếu chữ đ ở cuối")

if ___ == "đ":
    print("dòng 2 — kết đúng bằng đ")
else:
    print("dòng 2 — thiếu chữ đ ở cuối")
```

```python title=solution
dong_1 = "cà phê 25000đ"
dong_2 = "gửi xe 5000"

if dong_1[-1] == "đ":
    print("dòng 1 — kết đúng bằng đ")
else:
    print("dòng 1 — thiếu chữ đ ở cuối")

if dong_2[-1] == "đ":
    print("dòng 2 — kết đúng bằng đ")
else:
    print("dòng 2 — thiếu chữ đ ở cuối")
```

```python title=test
# Chấm bằng TRỌN VẸN màn hình theo đúng thứ tự dòng (`match: regex`) trên HAI
# dòng sổ, cộng một luật `static` đòi cả hai cái tên `dong_1` và `dong_2` phải
# thật sự được ĐỌC.
#
# Vì sao cần cả hai lớp:
#   `True` / `1` / `0`        → không giá trị nào bằng "đ", cả hai khối rơi
#                               xuống `else`, dòng 1 in sai → trượt.
#   chép `dong_1[-1]` xuống   → dòng 2 in "kết đúng bằng đ" → trượt.
#   gõ sẵn `"đ"` và `"x"`     → OUTPUT đúng, nhưng luật static thấy hai cái tên
#                               chỉ được GÁN chứ không được đọc lần nào → trượt.
#                               Đây là đáp án mà riêng output không bắt được,
#                               và nó hỏng ngay khi ai đó sửa nội dung dòng sổ.
#
# Hai dòng sổ dài 13 và 11 ký tự — cố ý lệch nhau, để một con số chỗ đứng cố
# định điền vào cả hai chỗ trống thì không thể cùng trúng cái đuôi.
#
# Người học chưa biết viết assert nên khối này không thêm phép kiểm nào; nó ở
# đây để nói rõ vì sao hai dòng là hai, không phải một.
pass
```

:::hints
- kind: attention
  body: Vế bên phải dấu `==` là `"đ"` — đúng một ký tự. Vậy thứ điền vào bên trái cũng phải là đúng một ký tự, lấy ra từ cái tên nằm ở đầu chương trình chứ không gõ tay.
- kind: strategy
  body: Xin một chỗ trong dãy thì viết tên rồi mở ngoặc vuông. Ở đây bạn cần hạt cuối, mà hai dòng sổ dài ngắn khác nhau nên không có con số dương nào trúng cả hai. Chiều đếm ngược thì có, và nó bắt đầu ngay ở số âm gần nhất. Nhớ hai khối hỏi về hai cái tên khác nhau.
- kind: one-line
  body: Chỗ trống thứ nhất là `dong_1[-1]`, chỗ trống thứ hai là `dong_2[-1]`.
:::

:::validate
- tier: static
  requireAst:
  - kind: uses-name, target: dong_1, min: 1
  - kind: uses-name, target: dong_2, min: 1
  onFail: hai câu điều kiện phải đọc ký tự cuối TỪ hai cái tên, không gõ sẵn ký tự vào
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^dòng 1 — kết đúng bằng đ\ndòng 2 — thiếu chữ đ ở cuối\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dòng có đuôi `đ`, một dòng không. Mình chỉ phải sờ vào đúng một hạt.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ bạn tra được từng ký tự một, từ hai đầu dây. Vậy còn **sửa** thì sao?

Sổ có một dòng ghi `"cà phê"`, và Byte muốn tên khoản viết hoa chữ đầu cho đẹp
cột: `"Cà phê"`. Chỉ một hạt phải đổi, và bạn biết chính xác nó nằm ở chỗ nào —
chỗ số 0.

Đọc thì bạn viết `ten[0]` và nhận về `"c"`. Vậy viết ngược lại, đặt cái ngoặc
vuông ấy sang bên trái dấu bằng:

```python
ten[0] = "C"
```

Trông hợp lý đến mức khó mà nghi ngờ: đúng chỗ ấy, đúng một hạt, đổi hạt cũ lấy
hạt mới. Realm 0 còn nói thẳng rằng danh sách là thứ **sửa được** — dãy đã tạo
ra rồi vẫn nhận thêm món trong lúc chương trình chạy.

Chuỗi cũng là một dãy. Nó có chịu cho bạn tháo một hạt ra thay hạt khác vào
không? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
