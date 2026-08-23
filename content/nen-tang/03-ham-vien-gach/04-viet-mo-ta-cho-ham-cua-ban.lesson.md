---
id: nen-tang.ham-vien-gach.viet-mo-ta-cho-ham-cua-ban
title: Viết lời mô tả cho hàm của bạn
summary: Câu chữ đặt ngay dòng đầu thân hàm được máy cất lại — và đọc y nguyên cho người sau gõ `help()`.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.docstring]
requires: [core.help-builtin, core.function-signature, core.function-def, core.function-parameter, core.function-return, core.function-call]
concepts: [core.ham, core.tham-so]
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
Chữ ký thì máy tự đọc ra được. Còn lời mô tả, máy đang chờ bạn viết.
::::

::::explain{#cho-trong-ay-ai-viet}
Bài trước kết bằng một màn hình hụt hẫng. Gõ `help(round)` — hàm của người
khác — máy trả về hai phần: một dòng chữ ký, rồi mấy dòng mô tả nói hàm ấy làm
gì. Gõ `help(gia_to)`, hàm tra bảng giá tô phở bạn tự viết ở Realm 0, thì phần
thứ nhất vẫn còn nguyên, phần thứ hai trống trơn.

Máy không giấu phần mô tả của bạn ở đâu cả. Nó không có gì để đưa ra.

Nhìn kỹ xem máy lấy chữ ký từ đâu. Nó đọc đúng dòng bạn đã gõ:

```python title=readonly
def tinh_tien(so_to, gia):
```

Dòng ấy nói ra được ba thứ: tên hàm là `tinh_tien`, hàm nhận hai thứ, và hai
thứ đó tên là `so_to` với `gia`. Máy chép lại y nguyên và gọi đó là chữ ký.

Nhưng *"hàm này để làm gì"*, *"`gia` là giá một tô hay giá cả bàn"*, *"đưa ra
con số hay đưa ra một câu chữ"* — không một chữ nào trong dòng `def` nói tới.
Máy không đoán được, và nó không đoán.

Bà chủ quán muối một lọ dưa cải. Nhìn cái lọ, ai cũng đếm được nó cao chừng
nào, miệng rộng chừng nào — đó là thứ tự nó lộ ra. Còn *muối ngày nào, ăn được
tới hôm nào* thì phải có người **dán một mảnh giấy** lên lọ. Không ai dán thì
mảnh giấy ấy không tự mọc ra.

Người viết hàm là người dán nhãn. Đó là bạn.

Trong Python, mảnh giấy ấy có một chỗ đứng cố định: **một câu chữ đặt ngay dòng
đầu tiên của thân hàm**, trước cả dòng lệnh đầu tiên. Câu chữ ấy có tên riêng —
**docstring**, nghĩa đen là "chuỗi tài liệu".

Nó được bọc trong **ba dấu nháy kép** liền nhau, `"""`, thay vì một. Vẫn là câu
chữ y như mọi câu chữ bạn đã viết từ Realm 0; ba dấu nháy chỉ thêm một quyền:
câu được phép xuống dòng thoải mái mà không đứt.
::::

::::example{#dan-nhan-cho-tinh-tien}
Đây là hàm `tinh_tien` quen thuộc, lần này có nhãn:

```python title=readonly
def tinh_tien(so_to, gia):
    """Tính tiền một bàn: số tô nhân với giá một tô."""
    return so_to * gia

help(tinh_tien)
```

Máy in ra:

```text
Help on function tinh_tien in module __main__:

tinh_tien(so_to, gia)
    Tính tiền một bàn: số tô nhân với giá một tô.
```

Hai phần đã đủ cả. Dòng `tinh_tien(so_to, gia)` là phần máy tự đọc từ dòng
`def`. Dòng thụt vào bên dưới là **đúng câu bạn vừa viết**, không thêm không
bớt một chữ.

Đó là toàn bộ việc mà `help` làm với docstring: cất lại nguyên văn, rồi đưa lại
nguyên văn. Máy không hiểu câu ấy nói gì — nó chép hộ bạn cho người sau.

Câu ấy cũng không hề thay đổi cách hàm chạy. Bỏ dòng docstring đi thì
`tinh_tien(2, 45000)` vẫn cho ra 90000 y hệt. Nó nằm đó cho **người** đọc.

Người sau là ai? Có thể là đồng nghiệp. Cũng có thể là chính bạn, ba tuần nữa,
lúc đã quên sạch mình định làm gì với `gia`.
::::

::::predict{#ghi-chu-hay-mo-ta commitOnce}
Byte cũng muốn dán nhãn, nhưng Byte viết câu ấy bằng dấu `#` — kiểu ghi chú
bạn đã gặp ở Realm 0.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
def tinh_tien(so_to, gia):
    # Tính tiền một bàn: số tô nhân với giá một tô.
    return so_to * gia

help(tinh_tien)
```

:::opt{correct}
Vẫn in ra dòng chữ ký, còn chỗ mô tả thì trống trơn
:::

:::opt
In ra chữ ký, rồi in ra câu ghi chú ngay bên dưới
::why
Gần đúng ở chỗ quan trọng nhất: với **người đọc**, hai cách viết ấy làm cùng
một việc. Mở file ra, câu sau dấu `#` và câu trong ba dấu nháy đều nằm cùng một
chỗ, đều nói cùng một điều, và đều giúp ích như nhau.

Chỗ lệch nằm ở thứ máy giữ lại. Dấu `#` bảo máy: *phần còn lại của dòng này
không phải lệnh, bỏ qua đi* — máy bỏ qua thật, và không cất lại gì. Còn câu
trong ba dấu nháy là một câu chữ thật, một giá trị đàng hoàng, nên máy cất được
nó vào cùng chỗ với hàm. `help` chỉ đưa ra được thứ đã được cất.
::
:::

:::opt
Máy báo lỗi, vì hàm không có dòng mô tả hợp lệ
::why
Gần đúng ở chỗ bạn đang cẩn thận đúng hướng: Python khó tính với chỗ đứng của
từng dòng, và bạn ngờ rằng viết sai chỗ thì máy sẽ kêu.

Chỗ lệch: một hàm không có docstring vẫn là hàm hợp lệ, chạy bình thường,
`help` vẫn in ra chữ ký cho nó. Thiếu nhãn không phải hỏng — nó chỉ là một lọ
dưa chưa dán giấy. Máy không đòi, và cũng vì vậy mà không ai nhắc bạn ngoài
chính bạn.
::
:::

:::opt
In ra câu ghi chú, nhưng không in dòng chữ ký
::why
Gần đúng ở chỗ bạn nhớ rằng có một thứ được ưu tiên đưa ra khi ai đó hỏi tới
hàm. Nhưng bạn đang xếp nhầm thứ tự ưu tiên.

Chỗ lệch: chữ ký là phần máy **tự dựng** từ dòng `def`, nên nó luôn có mặt,
không phụ thuộc vào việc bạn viết thêm gì. Phần mô tả mới là phần có thể vắng.
Ở đoạn trên, phần luôn có mặt vẫn có mặt, phần có thể vắng thì vắng.
::
:::
::::

::::explain{#viet-gi-vao-manh-giay}
Chỗ đứng thì đã rõ: dòng đầu tiên của thân hàm. Còn viết gì vào đó?

Nhớ lại trục của cả mạch này: người gọi hàm **không mở hàm ra xem**. Vậy nhãn
phải nói đủ cho một người như thế, và chỉ nói bấy nhiêu:

- **Hàm làm gì** — một câu, bắt đầu bằng một động từ. *"Tính tiền một bàn."*
- **Cần đưa vào những gì** — gọi đúng tên từng chỗ trống, nói rõ mỗi chỗ mong
  nhận cái gì. *"`so_to`: mấy tô. `gia`: giá một tô, tính bằng đồng."*
- **Đưa ra cái gì** — *"Trả về số tiền của cả bàn."*

Và một thứ **không** nên viết vào: cách hàm làm việc bên trong. Câu *"nhân
`so_to` với `gia` rồi trả về"* kể lại đúng dòng code nằm ngay bên dưới, nên nó
không thêm gì cho người đọc — mà ngày nào bạn sửa ruột hàm, nó thành một câu
nói dối nằm sẵn trong file.

> Chỗ dễ vấp: docstring phải là thứ **đầu tiên** trong thân hàm. Chen một dòng
> lệnh lên trên nó — dù chỉ một dòng gán — thì câu chữ ấy tụt xuống hàng thứ
> hai, và máy thôi coi nó là nhãn. Lúc đó nó vẫn nằm trong file cho người đọc,
> nhưng `help` không đưa nó ra nữa. Không có lỗi nào báo, nên đây là loại sai
> bạn phải tự soi.
::::

::::code{#dan-nhan-cho-ham-cua-ban}
Byte vừa viết xong hàm `tien_thua`, tính tiền thối lại cho khách. Hàm chạy
được, nhưng gõ `help(tien_thua)` lên thì chỗ mô tả vẫn trống.

Hãy dán nhãn cho nó. Viết một câu nói rõ hàm làm gì, và nói cho người sau biết
hai chỗ trống `khach_dua` với `tien_hang` mong nhận thứ gì.

Lời văn là của bạn — không có một câu duy nhất nào đúng. Chỗ đứng thì chỉ có
một.

```python title=starter
def tien_thua(khach_dua, tien_hang):
    ___
    return khach_dua - tien_hang

help(tien_thua)
```

```python title=solution
def tien_thua(khach_dua, tien_hang):
    """Tính tiền thối lại cho khách.

    khach_dua: số tiền khách đưa, tính bằng đồng.
    tien_hang: số tiền của cả bàn, tính bằng đồng.
    Trả về số tiền phải thối lại.
    """
    return khach_dua - tien_hang

help(tien_thua)
```

```python title=test
# Chấm đúng hai điều, và không chấm lời văn của bạn.
#
# Điều thứ nhất: máy có CẤT được nhãn không. Chỉ câu chữ nằm ngay dòng đầu thân
# hàm mới được cất; một dòng ghi chú `#`, một dòng gán, hay một câu chữ đặt sau
# dòng `return` đều để chỗ mô tả trống trơn — và đó đúng là chỗ bài này dạy.
#
# Điều thứ hai: hàm phải làm y việc cũ. Nhãn dán vào không được đụng tới ruột.
assert tien_thua.__doc__ is not None, "máy không tìm thấy lời mô tả nào trong hàm tien_thua — câu chữ phải nằm ngay dòng ĐẦU TIÊN của thân hàm, bọc trong ba dấu nháy kép"
assert tien_thua.__doc__.strip() != "", "lời mô tả đang rỗng — viết một câu nói rõ hàm làm gì, và hai chỗ trống mong nhận thứ gì"
assert tien_thua(200000, 135000) == 65000, "khách đưa 200 nghìn cho bàn hết 135 nghìn thì thối lại 65 nghìn — dán nhãn không được sửa dòng tính toán bên dưới"
```

:::hints
- kind: attention
  body: Chỗ trống nằm giữa dòng `def` và dòng `return`, thụt vào bốn dấu cách — nghĩa là nó đã ở đúng chỗ đầu tiên của thân hàm rồi. Thứ bạn viết vào đó phải là một câu chữ, không phải một dòng ghi chú.
- kind: strategy
  body: Câu chữ trong bài này bọc bằng ba dấu nháy kép liền nhau ở đầu và ba dấu nữa ở cuối. Viết trọn trên một dòng cũng được, xuống dòng cho dễ đọc cũng được — mở ở đâu thì đóng ở đó. Nội dung nên có một câu nói hàm làm gì, rồi nói `khach_dua` là tiền khách đưa và `tien_hang` là tiền của cả bàn.
- kind: one-line
  body: 'Viết `"""Tính tiền thối lại cho khách: tiền khách đưa trừ tiền hàng."""` vào chỗ trống, giữ nguyên bốn dấu cách thụt đầu dòng.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  expect: tien_thua(khach_dua, tien_hang)
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lọ dưa đã có giấy dán. Người sau mở tủ ra là đọc được ngay.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Docstring của bạn ghi rõ hàm cần hai thứ: tiền khách đưa và tiền hàng. Câu ấy
nằm ngay trong hàm, và máy vừa đọc lại nó ra màn hình cho bạn xem.

Nếu người ta chỉ đưa một — gõ `tien_thua(200000)` rồi thôi — máy đọc docstring
rồi tự bù thứ còn thiếu chứ?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
