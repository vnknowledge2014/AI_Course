---
id: toan.cam-nhan-so.mot-so-luon-la-may-cai-gi
title: Mấy — nhưng mấy cái gì
summary: Một con số viết trần chưa nói được điều gì. Nó luôn là câu trả lời cho "mấy CÁI gì", và cái tên ấy là một nửa của con số.
locale: vi
track: toan
module: cam-nhan-so
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.don-vi]
requires: [core.number-literal, core.variable, core.string-literal, core.print-variable, ctrl.comparison]
concepts: [math.don-vi, math.vuon-cua-byte]
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
Mình đếm xong rồi. Mười hai. Bạn hỏi lại mình đi — mười hai cái gì?
::::

::::explain{#tam-bang-thieu-mot-nua}
Byte có một mảnh vườn. Trong vườn có hai thứ Byte quan tâm: một túi **hạt**
giống, và mấy **luống** đất đã cuốc lên chờ gieo.

Sáng nay Byte đếm xong, cắm giữa vườn một tấm bảng, viết lên đó đúng một chữ:

```text
12
```

Rồi Byte đi tưới nước.

An sang chơi, đọc tấm bảng, và đứng đó. An không làm gì được với con số ấy, vì
An không biết nó nói về cái gì:

- 12 **hạt** trong túi? — vậy thì bốc gọn trong lòng bàn tay.
- 12 **luống** đất? — vậy thì cả buổi sáng gieo chưa xong.
- 12 **sải dây**, tức chiều dài một luống? — vậy thì nó chẳng nói gì về hạt cả.

Ba câu ấy nói về ba thứ khác hẳn nhau. Tấm bảng không giúp An chọn được câu nào.
::::

::::explain{#mot-con-so-co-hai-manh}
Chỗ hụt của tấm bảng không phải là Byte viết thiếu chữ cho lịch sự. Nó là chỗ
hụt thật: **mỗi lần đếm xong, bạn cầm trong tay hai mảnh, không phải một.**

```text
      12                    hạt
      ↑                      ↑
   con số                 đơn vị
  (được mấy)          (mấy CÁI GÌ)
```

Mảnh bên trái nói **bao nhiêu**. Mảnh bên phải nói **bao nhiêu cái gì** — tên
của thứ vừa được đếm. Mảnh bên phải có một cái tên riêng trong toán: **đơn vị**.

Byte chép lên bảng mảnh bên trái và bỏ quên mảnh bên phải. Nên tấm bảng mang
đúng một nửa câu.

Ngoài đời bạn không bao giờ nói nửa câu ấy. Ở chợ không ai nói "cho tôi 2" rồi
thôi — người ta nói *2 mớ rau*, *2 lạng thịt*, *2 chục trứng*. Bảng giá quán phở
ghi *45* thì cả người bán lẫn người mua đều đang ngầm đọc là *45 nghìn đồng*.
Người bán vải nói *3* thì bên trong đó là *3 mét*.

Có lúc người ta nuốt mất chữ đơn vị, nhưng đó là vì cả hai bên đã ngầm hiểu nó
từ trước — chứ không phải vì con số tự nó nói được. Chỉ cần một bên không ngầm
hiểu giống bên kia, chuyện hỏng ngay: người mua nghĩ *45 nghìn*, người bán nghĩ
*45 chục nghìn*.
::::

::::example{#may-in-ra-hai-lan-giong-nhau}
Đưa cả hai chuyện cho máy giữ hộ, mỗi chuyện một cái tên:

```python title=readonly
so_hat_trong_tui = 12
so_sai_day_do_mot_luong = 12

print(so_hat_trong_tui)
print(so_sai_day_do_mot_luong)
```

Máy in ra:

```text
12
12
```

Hai dòng giống hệt nhau, cho hai thứ khác hẳn nhau.

Để ý chỗ này: **cái đơn vị đang nằm trong tên biến, không nằm trong giá trị.**
Chữ `hat` và chữ `sai_day` là bạn viết cho bạn đọc. Thứ máy cất giữ và thứ máy
in ra chỉ có con số `12`, trần trụi, giống nhau từng nét.
::::

::::predict{#doan-may-tra-loi commitOnce}
Giờ hỏi máy một câu có–không: hai thứ ấy có bằng nhau không?

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
so_hat_trong_tui = 12
so_sai_day_do_mot_luong = 12
print(so_hat_trong_tui == so_sai_day_do_mot_luong)
```

:::opt{correct}
True
:::

:::opt
False
::why
Gần đúng ở chỗ bạn đang giữ chắc đúng cái điều bài này muốn dạy: 12 hạt và 12
sải dây là hai thứ khác nhau, và trả lời "bằng nhau" cho hai thứ ấy là một câu
trả lời vô nghĩa ngoài vườn.

Chỗ lệch nằm ở việc máy đang nhìn thấy gì. Bạn chưa hề đưa chữ "hạt" hay chữ
"sải dây" vào trong máy — bạn chỉ viết chúng trong **tên** của hai biến, và tên
thì máy không đọc nghĩa. Hai bên dấu `==` là hai con số `12` và `12`. Với chừng
ấy thông tin, `True` là câu trả lời đúng duy nhất máy có thể đưa ra.
::
:::

:::opt
12
::why
Gần đúng ở chỗ bạn đọc dấu `==` như dấu bằng trong vở toán — chỗ để ghi kết quả
ra. Cách đọc ấy đúng ở mọi trang vở bạn từng viết, và câu `12 = 12` thì không
ai bắt bẻ được.

Chỗ lệch: ở Realm 0 bạn đã gặp `==` với một vai khác hẳn — nó là một **câu hỏi
có–không**, không phải một chỗ ghi kết quả. Thứ đi ra khỏi nó luôn là `True`
hoặc `False`, chưa bao giờ là một con số. Muốn thấy con số thì bỏ hẳn phần
`== so_sai_day_do_mot_luong` đi.
::
:::

:::opt
Máy hỏi lại: "12 cái gì?"
::why
Gần đúng ở chỗ bạn đang đòi máy làm đúng cái việc mà cả bài này đòi: không nhận
một con số trần, phải hỏi cho ra đơn vị đã. Đòi hỏi ấy hoàn toàn đúng chỗ.

Chỗ lệch: người phải hỏi câu đó là bạn, không phải máy. Cái đơn vị chưa bao giờ
đi vào trong máy để nó có thể thắc mắc. Bạn đưa cho nó hai con số `12`, và đó là
tất cả những gì nó có trong tay. Đây chính là lý do đơn vị phải được **viết ra**
ở đâu đó — trên tấm bảng, trong tên biến, hoặc ngay trong dòng chữ in ra.
::
:::
::::

::::explain{#con-so-mang-luong-don-vi-mang-nghia}
Máy trả `True` không phải vì nó ẩu. Nó trả `True` vì nó trả lời **đúng câu bạn
hỏi**: hai con số này có bằng nhau không. Con số thì bằng nhau thật.

Câu bạn thật sự muốn hỏi lại là một câu khác: *12 hạt và 12 sải dây có phải một
thứ không.* Câu đó có chữ "hạt" và chữ "sải dây" trong đấy — và bạn đã bỏ hai
chữ ấy lại bên ngoài.

Rút ra một câu mang theo suốt track này:

> **Con số mang lượng. Đơn vị mang nghĩa. Bỏ đơn vị đi thì con số chưa nói được
> điều gì.**

Nên từ giờ, mỗi lần viết một con số ra, bạn viết đủ hai mảnh.
::::

::::code{#cam-du-hai-tam-bang}
Byte đếm xong hai thứ trong vườn: **12 hạt** trong túi, và **3 luống** đất đã
cuốc. Byte muốn cắm hai tấm bảng, mỗi tấm nói đủ một câu để An đọc là hiểu ngay.

Hai chỗ trống là nội dung hai tấm bảng. Mỗi tấm phải mang đủ **hai mảnh**: con
số, và tên của thứ được đếm.

Bài chấm bằng **cả hai** tấm bảng, và hai tấm mang hai đơn vị khác nhau. Chép
nội dung tấm này sang tấm kia thì tấm kia sai; viết đúng con số mà bỏ đơn vị thì
cũng trượt. Chỉ khi cả hai tấm đủ hai mảnh, đúng mảnh của mình, bài mới xanh.

```python title=starter
bang_cam_o_tui_hat = ___
bang_cam_o_manh_dat = ___

print(bang_cam_o_tui_hat)
print(bang_cam_o_manh_dat)
```

```python title=solution
bang_cam_o_tui_hat = "12 hạt"
bang_cam_o_manh_dat = "3 luống"

print(bang_cam_o_tui_hat)
print(bang_cam_o_manh_dat)
```

```python title=test
# Chấm trên HAI tấm bảng chứ không phải một. Một tấm thì không phân biệt được
# "hiểu bài" với "gõ bừa một câu chữ": tấm nào cũng có chữ và có số. Hai tấm
# mang hai đơn vị khác nhau mới bắt được lỗi chép nhầm chỗ.
assert "12" in bang_cam_o_tui_hat, "tấm bảng ở túi hạt phải mang con số 12"
assert "hạt" in bang_cam_o_tui_hat, "tấm bảng ở túi hạt còn thiếu đơn vị — 12 CÁI GÌ?"
assert "3" in bang_cam_o_manh_dat, "tấm bảng ở mảnh đất phải mang con số 3"
assert "luống" in bang_cam_o_manh_dat, "tấm bảng ở mảnh đất còn thiếu đơn vị — 3 CÁI GÌ?"
assert "hạt" not in bang_cam_o_manh_dat, "mảnh đất đếm bằng luống, không đếm bằng hạt — mỗi tấm mang đơn vị của riêng nó"
```

:::hints
- kind: attention
  body: Nhìn lại bức tranh hai mảnh ở đầu bài. Mảnh bên trái Byte đã có sẵn (12 và 3). Mảnh còn thiếu ở cả hai tấm bảng là mảnh bên phải — tên của thứ vừa được đếm.
- kind: strategy
  body: Tấm bảng là một câu chữ để người ta đọc, nên nó là một chuỗi — viết giữa hai dấu nháy, gồm con số rồi tới đơn vị, cách nhau một dấu cách. Túi kia đếm bằng hạt; mảnh đất kia đếm bằng luống.
- kind: one-line
  body: 'Viết `"12 hạt"` vào chỗ trống thứ nhất và `"3 luống"` vào chỗ trống thứ hai.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi tấm bảng phải mang đủ hai mảnh — con số và đơn vị của nó, viết liền trong một câu chữ
  requireAst:
  # Hai đơn vị khác nhau, nên hai luật này không thể cùng thoả bằng một câu
  # chép hai lần. Điền bừa vào chỗ trống thì không luật nào thoả.
  - kind: has-literal, target: 12 hạt
  - kind: has-literal, target: 3 luống
- tier: output
  match: regex
  expect: ^12 hạt\n3 luống\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai tấm bảng, mỗi tấm đủ hai mảnh. Giờ An đọc là biết ngay, khỏi phải đoán.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đơn vị đã có rồi: *hạt*. Còn con số thì sao — con số `12` ấy ở đâu ra, và nó
chắc chắn tới mức nào?

Byte đổ túi hạt ra chiếu, đếm từ phía hàng rào vào, được 12.

An gom lại, đổ ra lần nữa, đếm từ phía cổng lại — không nhìn Byte, không hỏi
Byte. An cũng được 12.

Hai người, hai thứ tự hoàn toàn khác nhau, mà cùng một con số. Cái gì bảo đảm
chuyện đó? Vì sao An không thể ra 11, hay 13?

Nếu bạn định trả lời "vì đống hạt có 12 hạt" — thì hãy để ý, câu đó đang dùng
chính cái con số mà chúng ta đang hỏi nó ở đâu ra.

Bài sau trả lời, và câu trả lời là một cái luật bạn làm theo từ hồi bé mà chưa
ai nói tên nó ra.
::::

::::checkpoint{mastery=0.8}
::::
