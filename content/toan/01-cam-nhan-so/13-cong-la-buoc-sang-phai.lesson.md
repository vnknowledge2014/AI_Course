---
id: toan.cam-nhan-so.cong-la-buoc-sang-phai
title: Cộng là bước sang phải
summary: Trên thanh số, cộng không phải đổ hai đống vào nhau mà là dời chỗ đứng sang phải — và hai bức tranh không bao giờ cãi nhau.
locale: vi
track: toan
module: cam-nhan-so
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.number-line-add]
requires: [math.thanh-so, math.addition-as-union, core.variable, core.reassign, core.rhs-first, core.arithmetic, core.print-variable]
concepts: [math.buoc-tren-thanh-so, math.don-vi, math.moc-khong]
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
Hai đống hạt thì đổ chung được. Còn hai cái chỗ đứng thì đổ vào đâu?
::::

::::explain{#cau-hoi-con-treo}
Bài trước để lại một câu hỏi. Tới giờ, cộng vẫn luôn là chuyện của hai **đống**:
đổ chung vào nhau, đếm lại, đủ mười cái lẻ thì gói thành một bó. Nhưng bài 5 đã
dựng một bức tranh khác hẳn cho đúng những con số ấy — **thanh số**. Trên thanh
số, một số không phải một đống. Nó là một **chỗ**: một cái vạch, đứng đúng một
nơi, cách vạch bên cạnh đúng một đơn vị.

Cùng con số 5, hai nghĩa nằm cạnh nhau:

- Trong tranh đống, 5 là **năm hạt** đang nằm trong lòng bàn tay.
- Trên thanh số, 5 là **cái vạch thứ năm** trên sải dây, tính từ đầu luống.

Và chỗ khó nằm ngay đó. Đống thì đổ chung được. Hai cái **vạch** thì đổ vào nhau
kiểu gì? Byte đang đứng ở vạch 5 và ai đó bảo "cộng 3" — cộng vào đâu?

Câu trả lời là: trên thanh số, con số thứ hai **không** đóng vai một đống nữa.
Nó đóng vai một **quãng đường**. "Cộng 3" nghĩa là: bước sang phải ba vạch.
::::

::::example{#vach-nam-di-ba-buoc}
Luống đất của Byte đo bằng sải dây, đầu luống là mốc 0. Byte cuốc dở, đang đứng
ở vạch 5. Cuốc thêm 3 sải nữa thì tới vạch nào?

```text
   0   1   2   3   4   5   6   7   8   9  10  11  12
   ├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
                       ├─1─┼─2─┼─3─┤
```

Bước thứ nhất: 5 sang 6. Bước thứ hai: 6 sang 7. Bước thứ ba: 7 sang 8. Ba
bước, dừng ở vạch 8.

Bây giờ kể lại đúng câu chuyện ấy bằng tranh đống: năm hạt, gộp thêm ba hạt, còn
tám hạt. Cũng ra 8.

Hai bức tranh trông chẳng giống nhau tí nào, mà con số thì trùng khít. Không
phải may mắn: **mỗi bước sang phải đúng bằng thêm một đơn vị vào lượng.** Bước
thứ nhất là cái thứ sáu, bước thứ hai là cái thứ bảy, bước thứ ba là cái thứ
tám. Dời chỗ và gộp thêm là hai cách kể **cùng một việc đếm** — cái việc ghép
đôi một–một mà bài 2 đã dựng.

Hỏi thẳng cái máy cả hai cách:

```python title=readonly
# tranh đống: 5 hạt gộp thêm 3 hạt
print(5 + 3)

# tranh thanh số: đứng ở vạch 5, bước sang phải 3 vạch
vach_dang_dung = 5
so_buoc = 3
print(vach_dang_dung + so_buoc)
```

Máy in ra:

```text
8
8
```

Máy chỉ biết một phép cộng. Hai bức tranh là của bạn, không phải của nó — và
đúng vì thế, chúng phải khớp nhau.
::::

::::predict{#doan-ba-buoc commitOnce}
Byte muốn máy kể lại chuyến đi **từng bước một**, chứ không nhảy một phát ba
vạch. Byte viết ba dòng giống hệt nhau, mỗi dòng nhích đúng một vạch.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
vi_tri = 5
vi_tri = vi_tri + 1
vi_tri = vi_tri + 1
vi_tri = vi_tri + 1
print(vi_tri)
print(5 + 3)
```

:::opt{correct}
8 rồi 8
:::

:::opt
3 rồi 8
::why
Gần đúng ở chỗ bạn đếm không sai một cái nào: ba dòng `+ 1` là ba bước, và số 3
có mặt thật trong câu chuyện này. Quy tắc bạn đang dùng — *kết quả là số việc đã
làm* — là quy tắc đúng khi câu hỏi là "đi mấy bước".

Chỗ lệch nằm ở phạm vi của nó. Cái tên `vi_tri` không giữ số bước, nó giữ **chỗ
đứng**. Mỗi dòng bảo Byte rời vạch cũ sang vạch bên phải, chứ không bảo Byte ghi
thêm một gạch vào sổ. Sau ba dòng, Byte không cầm con số 3 trong tay — Byte đang
đứng ở vạch 8. Muốn máy đếm bước thì phải có một cái tên thứ hai làm việc ấy.
::
:::

:::opt
6 rồi 8
::why
Gần đúng ở chỗ bạn nhìn ra mỗi dòng chỉ nhích đúng **một** vạch, không hơn — đó
chính là điều bài này muốn nói, và bạn đọc trúng nó.

Chỗ lệch: quy tắc "ba dòng viết giống hệt nhau thì làm giống hệt nhau" đúng với
lệnh, nhưng không đúng với **kết quả**, vì mỗi dòng đọc lại `vi_tri` ngay tại
lúc nó chạy. Dòng thứ nhất đọc được 5 nên ra 6. Dòng thứ hai đọc được 6 — không
phải 5 nữa — nên ra 7. Chúng không đè lên nhau; chúng nối đuôi nhau.
::
:::

:::opt
Máy báo lỗi, vì `vi_tri = vi_tri + 1` dùng chính cái tên đang được đặt
::why
Gần đúng ở chỗ bạn thấy cùng một cái tên đứng ở cả hai bên dấu `=` và thấy có gì
đó vòng tròn. Trong vở toán thì linh cảm ấy đúng hoàn toàn: câu `x = x + 1` là
một câu **không số nào thoả**, viết ra là sai ngay.

Chỗ lệch là dấu `=` trong Python không phải dấu bằng của vở toán. Nó là lệnh
**đặt lại tên**: làm xong vế phải trước, rồi mới dán cái tên lên kết quả vừa
làm. Máy đọc `vi_tri` ở vế phải — đang là 5 — cộng 1 ra 6, rồi mới cho `vi_tri`
trỏ sang 6. Không có vòng tròn nào cả, chỉ có trước và sau.
::
:::
::::

::::explain{#hai-vai-khac-nhau}
Đặt tên cho thứ vừa thấy, để mang đi được:

> Trên thanh số, `a + b` nghĩa là **đứng ở vạch `a` rồi bước sang phải `b`
> vạch.**

Có một chỗ trong câu ấy mà tranh đống không cho thấy: hai con số **không cùng
vai**. `a` là một *chỗ*. `b` là một *quãng*. Bên tranh đống thì cả hai đều là
đống, giống hệt nhau, đổ chung là xong. Bên thanh số thì một cái đứng yên chờ,
một cái ra lệnh đi.

Sự khác vai ấy trả công ngay:

- **Cộng 0 là đứng yên.** Không bước bước nào thì vẫn ở vạch cũ. Trong tranh
  đống, "gộp thêm không hạt nào" là một câu hơi kỳ — gộp cái gì vào cái gì? Trên
  thanh số thì nó rõ mồn một: đi 0 bước.
- **Cộng số lớn không cần đếm lại từ đầu.** Muốn biết vạch 5 cộng 137, bạn không
  phải đếm lại 5 hạt; bạn chỉ đi tiếp từ chỗ đang đứng.

Và quan trọng nhất: hai bức tranh này sẽ không bao giờ cãi nhau, vì cả hai cùng
đếm một thứ — số đơn vị. Bạn được phép đổi qua đổi lại tuỳ bài toán nào dễ hơn.
::::

::::code{#cuoc-tiep-luong-dat}
Sáng nay hai người cùng ra vườn, mỗi người một luống, cùng đo bằng sải dây và
cùng lấy đầu luống làm mốc 0.

- **Byte** đang ở vạch `4`, cuốc thêm `7` sải nữa.
- **An** đang ở vạch `9`. Trời đổ mưa nên An không cuốc thêm sải nào — `0` bước.
- **Hôm sau, An sang luống mới**, bắt đầu ở vạch `2`. Sáng cuốc thêm `3` sải,
  nghỉ trưa, chiều cuốc thêm `4` sải nữa.

Điền bốn chỗ trống để máy nói ra mỗi chuyến dừng ở vạch nào.

Bài chấm bằng **cả ba chuyến**, và chúng được chọn để cho ra những con số khác
nhau: một người có bước, một người đứng yên, một người đi hai chặng. Gõ cứng
`11` vào mọi chỗ thì An sai; gõ cứng `9` thì Byte sai. Chỉ phép cộng viết thật
mới qua được cả ba.

Chuyến thứ ba có chỗ đáng để ý: cái tên `vi_tri_an` **không** giữ số sải đã
cuốc, nó giữ **chỗ đang đứng** — đúng như `vi_tri` ở bước đoán phía trên. Sau
buổi sáng nó đã dời chỗ, nên buổi chiều phải đi tiếp **từ chỗ mới ấy**.

```python title=starter
cho_dung_byte = 4
so_buoc_byte = 7
cho_toi_byte = ___

cho_dung_an = 9
so_buoc_an = 0
cho_toi_an = ___

# Hôm sau, luống mới. An bắt đầu ở vạch 2.
vi_tri_an = 2
vi_tri_an = ___   # sáng cuốc thêm 3 sải
vi_tri_an = ___   # chiều cuốc thêm 4 sải

print(cho_toi_byte)
print(cho_toi_an)
print(vi_tri_an)
```

```python title=solution
cho_dung_byte = 4
so_buoc_byte = 7
cho_toi_byte = cho_dung_byte + so_buoc_byte

cho_dung_an = 9
so_buoc_an = 0
cho_toi_an = cho_dung_an + so_buoc_an

# Hôm sau, luống mới. An bắt đầu ở vạch 2.
vi_tri_an = 2
vi_tri_an = vi_tri_an + 3   # sáng cuốc thêm 3 sải
vi_tri_an = vi_tri_an + 4   # chiều cuốc thêm 4 sải

print(cho_toi_byte)
print(cho_toi_an)
print(vi_tri_an)
```

```python title=test
# Bốn câu chốt lại đúng những điều bài vừa nói, trên ba chuyến khác nhau — nên
# một con số gõ cứng chỉ qua được nhiều nhất một câu.
assert cho_toi_byte == 11, "đứng ở vạch 4, bước sang phải 7 vạch thì tới vạch 11"
assert cho_toi_an == 9, "An bước 0 bước, nên An vẫn ở vạch cũ"
assert cho_toi_an == cho_dung_an, "cộng 0 là đứng yên — chỗ tới phải trùng chỗ đứng"
assert vi_tri_an == 9, "đi hai chặng từ vạch 2: 3 sải rồi 4 sải, dừng ở vạch 9"
```

:::hints
- kind: attention
  body: Nhìn hai dòng ngay phía trên mỗi chỗ trống. Một dòng cho biết người đó đang đứng ở vạch nào, dòng kia cho biết người đó bước mấy bước. Chỗ trống cần cả hai, không chỉ một. Riêng hai chỗ trống cuối thì chỗ đứng nằm ở dòng ngay trên nó, và nó đổi sau mỗi chặng.
- kind: strategy
  body: Trên thanh số, chỗ tới = chỗ đứng dời sang phải bấy nhiêu vạch. Viết nó bằng hai cái tên có sẵn nối bằng dấu cộng — đừng viết thẳng con số đích, vì con số ấy chính là thứ bạn đang nhờ máy tìm hộ. Ở chuyến thứ ba, chỗ đứng lúc chiều chính là `vi_tri_an` sau buổi sáng, nên tên ấy phải có mặt ở cả hai vế.
- kind: one-line
  body: "Ba chỗ đầu là `cho_dung_byte + so_buoc_byte`, `cho_dung_an + so_buoc_an`, `vi_tri_an + 3`; chỗ cuối là `vi_tri_an + 4`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một phép cộng giữa chỗ đứng và số bước — gõ thẳng con số đích thì máy không tính gì cả, bạn tính hộ nó rồi
  requireAst:
  # `min: 4` vì có bốn chỗ trống, mỗi chỗ một phép cộng. Khung khởi đầu chưa có
  # dấu `+` nào, nên luật này chặn được đúng cái đáp án chép cứng bốn con số.
  - kind: uses-operator, target: +, min: 4
  # Hai chặng của chuyến thứ ba phải ĐỌC LẠI chỗ đang đứng, không được viết
  # `2 + 3` rồi `5 + 4`. `min: 3` = hai lần đọc trong hai chặng, cộng lần đọc
  # ở `print`.
  - kind: uses-name, target: vi_tri_an, min: 3
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^11\n9\n9\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Vạch 11, vạch 9, rồi lại vạch 9. Đi bảy bước, đứng yên, hay đi hai chặng — mình
đều theo chân được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cả bài này thanh số chỉ chạy về **một** phía: sang phải. Mỗi bước sang phải là
thêm một đơn vị, và Byte cuốc được thêm một sải.

Nhưng cái thanh số nằm đó cả hai chiều. Không có gì trên hình cấm Byte quay
người lại và bước ngược về phía đầu luống.

Vậy bước sang **trái** là phép gì? Nó phải có tên, vì Byte hỏi nó suốt. Ba câu
sau đây Byte hỏi gần như mỗi ngày, và cố ý dùng chung đúng một cặp số — `12` với
`5` — để bạn nhìn ra thứ đáng nhìn:

- *Luống có 12 cây, sâu ăn mất 5 cây thì còn mấy cây?*
- *Thùng có 12 lon gạo, đong ra 5 lon thì trong thùng còn mấy lon?*
- *Cả luống dài 12 sải, Byte cuốc tới vạch 5 thì còn phải cuốc mấy sải nữa?*

Ba câu ấy nghe khác nhau — nhưng có thật là chúng cùng một phép không?

Bài sau trả lời vế đầu.
::::

::::checkpoint{mastery=0.8}
::::
