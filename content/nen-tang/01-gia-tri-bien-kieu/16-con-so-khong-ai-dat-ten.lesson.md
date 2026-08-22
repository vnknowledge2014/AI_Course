---
id: nen-tang.gia-tri-bien-kieu.con-so-khong-ai-dat-ten
title: Con số trần nằm giữa công thức
summary: Giá trị không đổi thì tách ra một dòng riêng ở đầu file và đặt tên VIẾT_HOA — máy không khoá nó, giao ước là giữa người với người.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.constant]
requires: [core.variable, core.assignment, core.reassign, core.number-literal, core.arithmetic, core.fstring, core.naming-convention, core.float-contagion, core.floor-division, core.don-vi-nho-nhat]
concepts: [core.bien, core.gan, core.ten-va-chu]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Con số này mình chép ba chỗ. Hôm nào nó đổi, mình phải nhớ đủ cả ba.
::::

::::explain{#con-so-khong-co-ten}
Bài trước chốt một luật: cái tên phải nói ra **nội dung** — `tien_ca_phe` chứ
không phải `x`. Luật ấy vừa chỉ ra một chỗ hở mà chính nó không vá được.

Nhìn đoạn tính tiền phải trả cho ba món, thuế 10%:

```python title=readonly
tien_ca_phe = 25000
tien_pho = 50000
tien_com_ga = 75000

phai_tra_ca_phe = tien_ca_phe + tien_ca_phe * 10 // 100
phai_tra_pho = tien_pho + tien_pho * 10 // 100
phai_tra_com_ga = tien_com_ga + tien_com_ga * 10 // 100
```

Mọi giá trị **có tên** ở đây đều có tên tốt. Nhưng con số `10` xuất hiện ba
lần và không mang tên nào cả. Nó nằm trần giữa công thức.

Con số trần ấy giấu hai chuyện.

**Chuyện thứ nhất: ba tháng sau bạn mở lại file này.** `10` là gì? Thuế? Chiết
khấu cho khách quen? Phần trăm phí ship? Cái tên `tien_pho` tự nói ra nó là
tiền phở; con số `10` không tự nói được gì, và người đọc phải đoán.

**Chuyện thứ hai, đắt hơn nhiều: ngày thuế đổi.** Nhà nước hạ thuế từ 10%
xuống 8%, bạn mở file ra sửa. Sửa hai chỗ, sót một chỗ — máy vẫn chạy, vẫn in
ra ba con số, và một trong ba con số ấy sai. Không lỗi nào nổ, không dòng chữ
đỏ nào hiện lên. Bạn chỉ biết khi khách kêu.

Quán phở giải chuyện này từ lâu: giá dán **một tấm bảng ở quầy**, ai cũng nhìn
vào đó. Hôm tăng giá, chủ quán sửa đúng một tấm. Còn nếu giá được viết tay lên
từng tờ hoá đơn thì hôm ấy phải chạy theo từng tờ, và tờ nào sót thì khách
tính tiền mới biết.

Trong code, tấm bảng ở quầy có tên riêng. Một giá trị **không đổi suốt chương
trình** gọi là **hằng** — tiếng Anh là *constant*, nghĩa đen cũng là "không
đổi". Cách viết một hằng gồm ba điều, và cả ba đều là chuyện của mắt người
đọc:

- Khai **một chỗ**, ở đầu file, trước mọi dòng dùng tới nó.
- Tên viết **HOA HẾT**, các chữ ngăn nhau bằng gạch dưới: `PHAN_TRAM_THUE`,
  `PHI_SHIP`, `NGUONG_CANH_BAO`.
- Bên dưới thân chương trình thì chỉ **đọc** nó, không gán lại lần nào.

Chữ hoa không phải trang trí. Nó là dấu hiệu người đọc nhận ra ngay giữa một
trang code: *cái tên này khai một lần ở trên kia, và nó không đổi trong lúc
chạy.* Tên thường thì phải dò xem nó bị dán lại ở đâu chưa; tên hoa thì không
phải dò.
::::

::::example{#may-khong-giu-gium-ban}
Đây là đoạn trên viết lại với một hằng:

```python title=readonly
PHAN_TRAM_THUE = 10

tien_pho = 50000
phai_tra_pho = tien_pho + tien_pho * PHAN_TRAM_THUE // 100
print(f"Phở {phai_tra_pho}đ")
```

```text
Phở 55000đ
```

Ngày thuế đổi, bạn sửa đúng **một** dòng ở đầu file, và mọi công thức bên dưới
đổi theo. Không còn chỗ nào để sót.

Nhưng phải nói thẳng một chuyện, vì nó là chỗ người mới hay hiểu sai nhất:
**Python không khoá cái tên ấy lại.**

```python title=readonly
PHAN_TRAM_THUE = 10
PHAN_TRAM_THUE = 8
print(PHAN_TRAM_THUE)
```

```text
8
```

Máy nhận cả hai dòng, không kêu một tiếng — đúng như bài `int = 0` vừa cho
thấy: cái tên là mảnh giấy dán, gỡ ra dán sang giá trị khác lúc nào cũng được,
và chữ hoa nằm trong cái tên chứ không nằm trong máy.

Vậy hằng không phải thứ máy giữ giùm bạn. Nó là **giao ước giữa người với
người**: chữ hoa là lời hứa *"tôi không đổi cái này"*, và người đọc code sau
bạn tin lời hứa đó mà không phải đọc lại cả file. Máy đứng ngoài giao ước ấy.
::::

::::explain{#hang-cung-mang-mot-kieu}
Còn một cái bẫy nữa, và nó nằm ở chỗ không ai ngờ: **giá trị bạn chọn cho
hằng**.

Thuế 10% viết ra tự nhiên nhất là `THUE_SUAT = 0.1`. Đọc lên xuôi tai, tên
cũng tốt. Nhưng đặt tên không đổi được **kiểu** của giá trị: `0.1` viết có dấu
chấm nên nó là `float`, và theo luật lây của bài 1, chỉ một `float` lẫn vào là
cả phép tính ra `float`:

```python title=readonly
THUE_SUAT = 0.1
tien_pho = 50000
print(tien_pho + tien_pho * THUE_SUAT)
```

```text
55000.0
```

Cái đuôi `.0` quay lại — lần này nó lẻn vào qua một cái tên trông rất đàng
hoàng ở đầu file. Tiền vừa rơi ra khỏi `int`, đúng thứ mà bài 7 dặn phải giữ:
*tiền trong máy nằm ở đơn vị nhỏ nhất — đồng — dưới dạng `int`.*

Cách giữ nằm ngay ở chỗ chọn giá trị. Đừng khai hằng bằng tỉ lệ, hãy khai bằng
**phần trăm** — một số nguyên — rồi nhân trước, chia sau:

```python title=readonly
PHAN_TRAM_THUE = 10
tien_pho = 50000
print(tien_pho + tien_pho * PHAN_TRAM_THUE // 100)
```

```text
55000
```

`50000 * 10` là `int`, `// 100` cũng cho `int`, nên tiền ở nguyên trong `int`
từ đầu tới cuối. Hằng vẫn là hằng — chỉ khác chỗ giá trị của nó được chọn cho
hợp với luật kiểu, chứ không chọn cho xuôi tai.
::::

::::predict{#doan-hang-co-doi-khong commitOnce}
Byte viết một đoạn tính tiền phở. Đầu file có hằng thuế, và ở giữa file Byte
để lại thêm một dòng nữa. **Trước khi bấm chạy**, bạn đoán màn hình hiện ra
gì?

```python title=readonly
PHAN_TRAM_THUE = 10
tien_pho = 50000

PHAN_TRAM_THUE = 8

print(tien_pho + tien_pho * PHAN_TRAM_THUE // 100)
```

:::opt{correct}
54000
:::

:::opt
55000
::why
Gần đúng ở chỗ bạn đọc cái tên VIẾT_HOA đúng như người viết muốn nó được đọc:
đây là con số khai một lần ở đầu file rồi thôi, nên giá trị đáng tin là giá
trị ở dòng 1. Đó chính là lý do người ta viết hoa.

Chỗ lệch: chữ hoa nằm trong cái tên, không nằm trong máy. Với Python, dòng
`PHAN_TRAM_THUE = 8` có hình dạng y hệt `tien_pho = 50000` — một cái tên bên
trái, một giá trị bên phải — nên nó gỡ mảnh giấy ra và dán sang số `8`. Lúc
`print` chạy, thứ cái tên đang giữ là `8`, nên phần thuế là `4000` chứ không
phải `5000`.
::
:::

:::opt
Máy báo lỗi, vì một cái tên VIẾT_HOA thì không được gán lần thứ hai
::why
Gần đúng ở chỗ bạn trông đợi máy giữ đúng lời hứa mà cái tên vừa nêu ra — và
có những ngôn ngữ lập trình làm đúng thế thật: khai một hằng ở đó rồi gán lại
là chương trình không dịch được, chặn ngay tại chỗ.

Chỗ lệch nằm ở Python: nó không có khái niệm "tên này bị khoá". Chữ hoa với nó
chỉ là những ký tự trong tên, y như chữ thường. Vì vậy hằng ở Python là giao
ước giữa người với người, và cổng gác duy nhất là mắt người đọc code.
::
:::

:::opt
54000.0
::why
Gần đúng ở chỗ bạn nhớ ra luật lây và mang nó ra dùng: chỉ một `float` lẫn vào
là cả phép tính ra `float`. Bạn vừa thấy đúng chuyện đó xảy ra với `0.1` ở
đoạn trên, nên đem soi tiếp là phản xạ đúng.

Chỗ lệch: trong đoạn này không có `float` nào cả. `8` viết không có dấu chấm
nên nó là `int`, `50000 * 8` là `int`, và `// 100` — phép chia lấy phần nguyên
— cũng trả về `int`. Cái đuôi `.0` chỉ mọc ra khi có `/` hoặc khi có một số
thực tham gia, mà ở đây không có cái nào.
::
:::
::::

::::code{#dan-mot-tam-bang-gia}
Nhà nước vừa hạ thuế từ 10% xuống **8%**. Sổ của quán có ba món, và bản cũ của
đoạn code này chép con số phần trăm trần ra cả ba công thức — đúng ba chỗ vừa
được gỡ đi, để lại ba chỗ trống.

Hằng thì đã khai sẵn ở dòng đầu file. Việc của bạn là nối ba công thức vào nó,
để từ nay chỉ một dòng điều khiển cả ba số tiền.

```python title=starter
PHAN_TRAM_THUE = 8

tien_ca_phe = 25000
tien_pho = 50000
tien_com_ga = 75000

phai_tra_ca_phe = tien_ca_phe + tien_ca_phe * ___ // 100
phai_tra_pho = tien_pho + tien_pho * ___ // 100
phai_tra_com_ga = tien_com_ga + tien_com_ga * ___ // 100

print(f"Cà phê {phai_tra_ca_phe}đ")
print(f"Phở {phai_tra_pho}đ")
print(f"Cơm gà {phai_tra_com_ga}đ")
```

```python title=solution
PHAN_TRAM_THUE = 8

tien_ca_phe = 25000
tien_pho = 50000
tien_com_ga = 75000

phai_tra_ca_phe = tien_ca_phe + tien_ca_phe * PHAN_TRAM_THUE // 100
phai_tra_pho = tien_pho + tien_pho * PHAN_TRAM_THUE // 100
phai_tra_com_ga = tien_com_ga + tien_com_ga * PHAN_TRAM_THUE // 100

print(f"Cà phê {phai_tra_ca_phe}đ")
print(f"Phở {phai_tra_pho}đ")
print(f"Cơm gà {phai_tra_com_ga}đ")
```

```python title=test
# Ba món, ba số tiền khác nhau. Nối sót một chỗ thì đúng một trong ba dòng
# assert đổ, nên cách chấm này chỉ đạt khi cả ba công thức cùng nhìn vào hằng.
assert phai_tra_ca_phe == 27000, "cà phê 25000 cộng 8% phải ra 27000"
assert phai_tra_pho == 54000, "phở 50000 cộng 8% phải ra 54000"
assert phai_tra_com_ga == 81000, "cơm gà 75000 cộng 8% phải ra 81000"
# Nối vào một hằng dạng tỉ lệ (`0.08`) vẫn ra ba con số gần đúng, nhưng tiền
# rơi khỏi `int` — luật bài 7 gãy mà không lỗi nào nổ.
assert isinstance(phai_tra_pho, int), "tiền phải ở nguyên trong int đồng"
```

:::hints
- kind: attention
  body: Dòng trên cùng đã khai sẵn một cái tên, và nó đang giữ đúng con số phần trăm mới. Ba chỗ trống nằm ở đúng vị trí mà con số ấy vốn bị chép trần ra.
- kind: strategy
  body: Cả ba chỗ trống điền cùng một thứ, và thứ đó không phải một con số — nếu điền số thì bạn vừa chép trần lại lần nữa, và ngày thuế đổi tiếp thì vẫn phải sửa ba chỗ. Điền cái tên ở dòng đầu file, chép đúng từng chữ hoa.
- kind: one-line
  body: "Viết `PHAN_TRAM_THUE` vào cả ba chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Phở 54000đ
- tier: static
  onFail: ba công thức phải nhìn vào cái hằng ở đầu file, không chép trần lại con số phần trăm
  requireAst:
  # `min: 3` vì có ba công thức. Chép con số `8` vào ba chỗ trống thì cái tên
  # không được đọc lần nào, và điền bừa cũng vậy — cả hai đều bị chặn ở đây.
  - kind: uses-name, target: PHAN_TRAM_THUE, min: 3
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dòng đổi, cả ba con số đổi theo. Mình không phải nhớ đủ ba chỗ nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đầu file giờ đã có tấm bảng giá: `PHAN_TRAM_THUE`, rồi mai mốt thêm `PHI_SHIP`,
`NGUONG_CANH_BAO`. Ba hằng ấy đứng riêng mỗi cái một dòng, và nên đứng riêng
thật — chúng là ba chuyện không liên quan gì nhau, khai một lần rồi thôi.

Nhưng xuống dưới thân chương trình thì có một nhóm không giống thế. Mỗi lần
bắt đầu ghi **một khoản mới** vào sổ, bạn phải dựng lại đủ ba ô của một dòng
sổ:

```python
ten = ""
tien = 0
ghi_chu = None
```

Ba dòng này khác hẳn ba hằng ở trên: chúng không phải ba chuyện riêng, chúng
là **ba ô của cùng một dòng sổ**. Chúng sinh ra cùng lúc và đi với nhau. Quên
một dòng thì dòng sổ thiếu một ô, và cái tên còn thiếu ấy chỉ nổ ra ở tận nơi
bạn đem đi in.

Xếp dọc ba dòng `=` như vậy, mắt không thấy chúng thuộc về nhau. Có cách nào
cho ba cái tên ấy đứng chung **một** dòng gán, để nhìn phát là biết chúng đi
cùng nhau không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
