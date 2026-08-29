---
id: nen-tang.gia-tri-bien-kieu.ten-chua-tung-duoc-gan
title: Cái tên chưa từng được gán
summary: Một cái tên chỉ có mặt từ lúc dòng gán của nó chạy qua — trước đó máy không giữ sẵn None nào cho bạn cả.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.name-defined-order]
requires: [core.none, core.is-none, core.name-lookup, err.name-error, core.assignment, core.variable, ctrl.if, ctrl.comparison, ctrl.for-each, core.list, core.list-append, core.output]
concepts: [core.ten, core.thu-tu-buoc, core.loi-khi-chay]
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
Mình không giữ sẵn cái tên nào cho bạn. Có mặt hay chưa là do dòng gán quyết.
::::

::::explain{#ten-co-mat-tu-luc-nao}
Bài trước kết bằng một câu hỏi: nếu bạn **quên hẳn** dòng `ghi_chu = None` rồi
viết `print(ghi_chu)`, máy có tự coi cái tên ấy là "chưa có gì" không?

Câu trả lời gọn: **không**. Và lý do đáng nhớ hơn cả câu trả lời.

Nghĩ về cuốn sổ chi tiêu lần nữa. Trước khi ghi, bạn phải **kẻ ô** đã. Ô kẻ rồi
mà để trắng thì vẫn là một cái ô: nó có bốn cạnh, nó chiếm chỗ trên trang, và ai
lật tới trang ấy cũng nhìn thấy nó. Còn trang giấy chưa kẻ gì thì không có ô nào
để mà trắng — nói "ô thứ ba của trang này đang trống" là nói về một thứ không
tồn tại.

Dòng gán chính là nét kẻ ô. `ghi_chu = None` làm hai việc cùng lúc, và hai việc
ấy khác nhau:

1. **Kẻ ô**: từ đây trở đi cái tên `ghi_chu` có mặt trong chương trình.
2. **Ghi vào ô**: nội dung của nó là `None`, tức "chưa có gì".

Bỏ dòng ấy đi thì bạn mất cả hai, chứ không phải chỉ mất việc thứ hai. Không có
ô nào cả. Và khi bạn hỏi máy về một cái tên chưa được kẻ, nó dừng lại và nói ra
— bằng đúng cái lỗi bạn đã gặp ở Realm 0 khi gõ sai tên biến: **`NameError`**.

Chỗ mới của bài này không phải cái tên lỗi. Chỗ mới là **thời điểm**: máy đọc
chương trình từ trên xuống, và một cái tên chỉ có mặt **từ dòng gán của nó trở
xuống**. Cùng một cái tên, cùng một cách viết, nhưng đặt trên hay đặt dưới dòng
gán là hai số phận khác nhau.
::::

::::example{#hai-dong-doi-cho-cho-nhau}
Hai đoạn dưới đây có đúng hai dòng, và hai dòng ấy y hệt nhau. Chỉ khác thứ tự.

```python title=readonly
print(ghi_chu)
ghi_chu = None
```

```text
Traceback (most recent call last):
  File "so_chi_tieu.py", line 1, in <module>
    print(ghi_chu)
          ^^^^^^^
NameError: name 'ghi_chu' is not defined
```

Máy chạy tới dòng 1, tìm cái tên `ghi_chu`, không thấy, và dừng hẳn. Dòng 2 —
dòng sẽ tạo ra cái tên ấy — nằm ngay dưới, cách có một dòng, nhưng chưa tới lượt
chạy nên nó chưa làm được gì. Câu tiếng Anh cuối dịch sát là *chưa có cái tên
`ghi_chu` nào được định nghĩa*.

Đảo hai dòng lại:

```python title=readonly
ghi_chu = None
print(ghi_chu)
```

```text title=readonly
None
```

Cùng hai dòng ấy, giờ in ra bình thường. Không phải chữ nào sai; chỉ là thứ tự.

Ở hai đoạn trên, cái sai nằm ngay trước mắt: dòng gán đứng dưới dòng dùng, đọc
một cái là thấy. Nhưng có một chỗ chuyện này xảy ra mà bạn **không cố ý chút
nào**, và nhìn thẳng vào file cũng không thấy — vì ở đó dòng gán đứng đúng chỗ
của nó, phía trên hẳn hoi.

Chỗ ấy là khối `predict` ngay dưới đây.
::::

::::predict{#doan-hom-khong-ai-boa commitOnce}
Byte in ghi chú của một dòng sổ. Hôm nay không ai boa, nên `tien_tip` bằng `0`.
**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
tien_tip = 0

if tien_tip > 0:
    ghi_chu = "khách boa thêm"

print(ghi_chu)
```

:::opt{correct}
Máy không in dòng nào, chỉ hiện một thông báo `NameError`
:::

:::opt
Máy in ra `None`
::why
Gần đúng ở chỗ bạn vừa học rằng `None` là giá trị dành riêng cho "chưa có gì", và
ở đây `ghi_chu` đúng là chưa có gì thật. Nối hai chuyện ấy lại là một suy luận
liền mạch — có những ngôn ngữ khác làm đúng như bạn đoán.

Chỗ lệch: `None` là thứ **bạn** đặt vào, không phải thứ máy tự bù. Máy chỉ biết
những cái tên mà một dòng gán đã chạy qua và tạo ra. Cái tên `ghi_chu` chưa được
tạo ra lần nào, nên với máy nó không tồn tại — mà "không tồn tại" thì khác với
"tồn tại và đang giữ `None`".
::
:::

:::opt
Máy in ra một dòng trống
::why
Gần đúng ở chỗ bạn nghĩ một ô chưa ghi gì thì in ra sẽ chẳng thấy gì — đúng như
ô `""` ở bài trước, in ra một dòng trắng.

Chỗ lệch nằm ở việc `""` là một giá trị đã được gán vào một cái tên có mặt hẳn
hoi. Ở đây không có giá trị nào, cũng không có cái tên nào. `print` không có gì
để in ra, kể cả một dòng trắng — nó chưa hề được chạy tới.
::
:::

:::opt
Máy in ra `khách boa thêm`
::why
Gần đúng ở chỗ bạn đọc thấy dòng gán nằm ngay trong file, và một dòng đã viết ra
thì trông như đã có hiệu lực. Cách đọc ấy đúng với mọi dòng nằm sát lề trái.

Chỗ lệch: dòng gán này thụt vào, tức là nó nằm trong thân `if`. Thân `if` chỉ
chạy khi điều kiện đúng, mà `0 > 0` là sai. Dòng ấy hôm nay bị bỏ qua hoàn toàn
— nó có mặt trong file nhưng chưa từng chạy, nên chưa tạo ra cái tên nào.
::
:::
::::

::::explain{#dong-gan-co-mat-nhung-chua-chay}
Đúng như bạn vừa đoán — máy dừng lại và in ra:

```text
Traceback (most recent call last):
  File "so_chi_tieu.py", line 6, in <module>
    print(ghi_chu)
          ^^^^^^^
NameError: name 'ghi_chu' is not defined
```

Dòng gán **có mặt trong file** — bạn nhìn thấy nó, nó nằm ngay đó, và nó đứng
phía trên dòng `print` đàng hoàng. Nhưng hôm nay khách không boa, `0 > 0` là
sai, nên thân `if` không chạy lần nào. Nét kẻ ô nằm trong một khối chưa bao giờ
được đi qua, nên cái ô chưa bao giờ được kẻ.

Nói cho chính xác thì luật là: cái tên có mặt kể từ lúc dòng gán của nó **chạy
qua**, chứ không phải kể từ lúc bạn gõ nó ra.

Và đây là chỗ khó chịu nhất: đoạn code này chạy êm suốt những hôm có khách boa.
Nó chỉ nổ vào đúng cái hôm không ai boa cả — nên nó không nổ trên máy bạn lúc
viết, nó nổ trên máy người khác vào một hôm bạn không có mặt.
::::

::::explain{#ke-o-truoc-roi-hay-hoi}
Cách sửa nằm sẵn trong hai bài vừa rồi: **kẻ ô trước, hỏi sau**.

```text
ghi_chu = None          ← kẻ ô, đặt sẵn nghĩa "chưa có gì"
if tien_tip > 0:
    ghi_chu = "khách boa thêm"   ← có thì ghi đè lên
print(ghi_chu)          ← lúc nào cũng có thứ để in
```

Dòng đầu bảo đảm cái tên `ghi_chu` **luôn** có mặt trước khi bất kỳ ai hỏi tới
nó, dù nhánh `if` có chạy hay không. Hôm khách boa, dòng trong thân `if` dán lại
cái tên lên nội dung mới. Hôm không ai boa, cái tên vẫn còn đó, giữ nguyên
`None` — và `None` ở đây nói đúng sự thật: chưa có ghi chú nào.

Đây chính là lý do bài 10 nói `None` dùng để **giữ chỗ**. Giữ chỗ không phải một
thói quen cho đẹp; nó là cách duy nhất để dòng `print` phía dưới không phụ thuộc
vào chuyện hôm nay khách có boa hay không.

Và nhờ có dòng giữ chỗ ấy, câu hỏi `ghi_chu is None` của bài trước mới dùng được
thật. Không có nó thì bạn còn chẳng hỏi được câu ấy: hỏi về một cái tên chưa
tồn tại thì máy nổ `NameError` ngay ở chỗ đặt câu hỏi, chứ không trả lời `True`.

Gói lại thành ba câu để mang đi:

- Cái tên có mặt kể từ lúc dòng gán của nó chạy qua, không sớm hơn một giây nào.
- Máy không bù sẵn `None` cho cái tên bạn quên khai — nó dừng lại và báo
  `NameError`.
- Dòng gán nằm trong một nhánh chỉ tính khi nhánh ấy thật sự chạy.
::::

::::code{#giu-cho-truoc-vong}
Sổ tiền boa hai buổi: buổi trưa không ai boa (`0`), buổi tối có khách boa
`20000`. Byte muốn in ghi chú của **cả hai** buổi, và buổi nào chưa có ghi chú
thì in `Ghi chú: (chưa có)`.

Đoạn dưới đang nổ `NameError` ngay ở buổi trưa, vì dòng tạo ra `ghi_chu` nằm
trong thân `if` mà buổi trưa thì `if` không chạy.

Điền vào chỗ trống **một dòng** để cái tên `ghi_chu` luôn có mặt ở mọi buổi, kể
cả buổi không ai boa — và nội dung của nó lúc ấy phải nói đúng rằng chưa có ghi
chú nào.

```python title=starter
cac_buoi = [0, 20000]
ket_qua = []

for tien_tip in cac_buoi:
    ___
    if tien_tip > 0:
        ghi_chu = "khách boa thêm"
    ket_qua.append(ghi_chu)
    if ghi_chu is None:
        print("Ghi chú: (chưa có)")
    else:
        print("Ghi chú:", ghi_chu)
```

```python title=solution
cac_buoi = [0, 20000]
ket_qua = []

for tien_tip in cac_buoi:
    ghi_chu = None
    if tien_tip > 0:
        ghi_chu = "khách boa thêm"
    ket_qua.append(ghi_chu)
    if ghi_chu is None:
        print("Ghi chú: (chưa có)")
    else:
        print("Ghi chú:", ghi_chu)
```

```python title=test
# Hai buổi, hai kết quả khác nhau — một dòng giữ chỗ đúng phải làm cả hai
# cùng đúng. Giữ chỗ bằng `0` hay bằng `""` thì buổi trưa lọt xuống nhánh
# `else` và in sai; không giữ chỗ gì thì buổi trưa nổ `NameError`.
assert ket_qua == [None, "khách boa thêm"], "buổi trưa không ai boa nên ghi chú vẫn là chưa có gì, buổi tối có khách boa nên chỗ giữ chỗ được ghi đè bằng nội dung thật"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ngay đầu thân vòng lặp, TRƯỚC câu `if tien_tip > 0:`. Máy đi qua nó ở mọi buổi, kể cả buổi mà thân `if` không chạy.
- kind: strategy
  body: Dòng bạn cần là một dòng gán, và nó phải làm hai việc cùng lúc — tạo ra cái tên `ghi_chu`, và đặt vào đó thứ mang nghĩa "chưa có ghi chú nào". Nhìn dòng `if ghi_chu is None:` phía dưới để biết nó đang chờ đúng giá trị nào.
- kind: one-line
  body: "Viết `ghi_chu = None` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Ghi chú: \(chưa có\)\nGhi chú: khách boa thêm\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Buổi trưa không ai boa mà cái tên vẫn có mặt. Mình có thứ để đọc rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Luật vừa học nghe rất dứt khoát: cái tên nào cũng phải có một dòng gán chạy qua
thì mới có mặt.

Nhưng nhìn lại dòng code đầu tiên bạn từng gõ ở Realm 0:

```text
print("xin chào")
```

`print` cũng là một cái tên. Bạn chưa bao giờ viết `print = ...` ở đâu cả, mà nó
vẫn có mặt ngay từ dòng đầu tiên của mọi chương trình. `type`, `input`, `bool`
cũng thế.

Vậy **ai** đã gán những cái tên ấy, và gán lúc nào?

Và câu hỏi đi kèm, đáng lo hơn: nếu chúng chỉ là những cái tên bình thường như
`ghi_chu`, thì chuyện gì xảy ra khi bạn viết `print = 5`?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
