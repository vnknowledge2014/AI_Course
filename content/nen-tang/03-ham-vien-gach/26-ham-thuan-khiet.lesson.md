---
id: nen-tang.ham-vien-gach.ham-thuan-khiet
title: Hàm thuần khiết
summary: Hàm chỉ đọc đối số và chỉ nói qua `return` thì cùng đầu vào luôn cho cùng đầu ra — nên thử được một mình, không cần dựng cả chương trình.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.pure-function]
requires: [core.argument-not-copy, core.function-def, core.function-call, core.function-parameter, core.function-argument, core.function-return, core.variable, core.assignment, core.arithmetic, core.fstring, core.output]
concepts: [core.ham, core.tra-ve, core.tham-so]
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
Có loại hàm mình hỏi lúc nào cũng được. Hỏi mười lần, mười lần một câu trả lời.
::::

::::explain{#can-phong-chi-co-mot-cua}
Câu hỏi bỏ ngỏ của bài trước: loại hàm nào mới đáng tin — loại gọi bao nhiêu
lần, gọi vào lúc nào cũng cho ra cùng một kết quả?

Hai bài vừa rồi cho thấy niềm tin gãy ở đâu. Bài 24: hàm gán vào một biến toàn
cục, nên nó nhớ lần gọi trước. Bài 25: hàm viết vào cuốn sổ của người gọi, nên
nó để lại một bãi phía sau. Hai đường khác nhau, cùng một hình dạng: hàm có
một **cửa** mà chữ ký không hề nhắc tới.

Vậy thì loại hàm đáng tin là loại bịt hết những cửa ấy lại. Hình dung một căn
phòng chỉ có đúng một ô cửa:

- Bạn đẩy nguyên liệu qua ô cửa vào trong.
- Bên trong, người thợ làm việc của mình. Anh ta không có điện thoại, không có
  cửa sau, không với tay ra được cái kệ ngoài hành lang.
- Anh ta đẩy thành phẩm ra qua đúng ô cửa ấy.

Đẩy vào cùng một mớ nguyên liệu, lần nào cũng nhận về cùng một thành phẩm.
Không phải vì người thợ ngoan, mà vì trong phòng chẳng có gì khác để mà khác đi.

Giới lập trình gọi một hàm như vậy là **hàm thuần khiết**. Ba điều làm nên nó:

- Nó **chỉ đọc đối số**. Không đọc biến toàn cục, không hỏi giờ, không hỏi
  người dùng.
- Nó **chỉ nói qua `return`**. Không `print`, không ghi file.
- Nó **không sửa gì của ai**. Không gán vào biến toàn cục, không `.append()`
  vào cuốn sổ được đưa vào.

Gộp ba điều lại thành một câu kiểm được: *cùng đầu vào thì luôn cùng đầu ra, và
sau khi hàm chạy xong, mọi thứ ngoài nó vẫn y như trước.*
::::

::::example{#hai-ham-cung-mot-viec}
Cùng một việc — tính tiền một bàn — viết theo hai lối.

```python title=readonly
tong_ca_ngay = 0

def tien_ban_co_ghi_so(gia_mot_to, so_to):
    """Tính tiền một bàn VÀ cộng luôn vào tổng cả ngày."""
    global tong_ca_ngay
    tien = gia_mot_to * so_to
    tong_ca_ngay = tong_ca_ngay + tien
    return tien

def tien_ban(gia_mot_to, so_to):
    """Đưa vào giá một tô và số tô, đưa ra tiền của riêng bàn ấy."""
    return gia_mot_to * so_to

print(tien_ban_co_ghi_so(45000, 2))
print(tien_ban(45000, 2))
print(f"Tổng cả ngày: {tong_ca_ngay}")
```

Máy in ra:

```text
90000
90000
Tổng cả ngày: 90000
```

Hai hàm cho ra đúng một con số. Khác nhau nằm ở dòng cuối.

`tien_ban_co_ghi_so` để lại dấu vết: `tong_ca_ngay` đã đổi. Gọi nó lần nữa với
đúng hai con số ấy thì con số trả về vẫn là 90 nghìn, nhưng thế giới bên ngoài
lại khác đi thêm một lần nữa. Muốn biết chương trình đang ở đâu, bạn phải nhớ
đã gọi hàm này mấy lần rồi.

`tien_ban` thì không để lại gì. Gọi nó một lần, mười lần, gọi lúc mở quán hay
lúc dọn hàng — `tong_ca_ngay` vẫn nguyên như lúc chưa gọi. Nó là hàm thuần
khiết.

Chú ý một chuyện: `tien_ban` **không** hề vô dụng vì nó chẳng chạm vào ai. Con
số nó trả về đi ra ngoài đàng hoàng qua `return`, và người gọi muốn cộng vào
tổng cả ngày thì tự cộng lấy — ở ngoài, nơi ai đọc cũng thấy.
::::

::::explain{#thu-mot-minh-duoc}
Có một hệ quả rất cụ thể, và nó là lý do người ta thích hàm thuần khiết chứ
không phải vì nó nghe sang.

Muốn thử `tien_ban_co_ghi_so` xem đúng chưa, bạn phải dựng đủ bối cảnh: cái tên
`tong_ca_ngay` phải tồn tại, phải mang đúng giá trị bạn muốn, và bạn phải nhớ
rằng thử xong thì nó đã bị đổi — lần thử sau nằm trên một nền khác.

Muốn thử `tien_ban` thì gõ đúng một dòng, ở bất cứ đâu:

```python
tien_ban(45000, 2)
```

Ra 90 nghìn thì nó đúng. Không cần dựng gì trước, không phải dọn gì sau. Đó là
lý do một hàm thuần khiết kiểm được bằng chính khối test bạn vẫn thấy trong mỗi
bài: `assert` gọi hàm rồi so con số, không cần biết chương trình đang ở đâu.

Và nhìn ngược lại bài 1: bạn tin `len("phở")` được, một phần lớn vì `len` chính
là một hàm thuần khiết. Nó nhận một thứ, đưa ra một con số, và không đụng vào
bất cứ thứ gì của bạn.
::::

::::predict{#doan-bon-dong commitOnce}
Byte viết hai hàm cùng giảm giá 5 nghìn cho một tô phở, nhưng một hàm có ghi sổ
xem đã giảm mấy lần rồi.

**Trước khi bấm chạy**, bạn đoán bốn dòng in ra bốn con số nào?

```python
so_lan_goi = 0

def gia_sau_giam(gia):
    return gia - 5000

def gia_sau_giam_dan(gia):
    global so_lan_goi
    so_lan_goi = so_lan_goi + 1
    return gia - 5000 * so_lan_goi

print(gia_sau_giam(45000))
print(gia_sau_giam(45000))
print(gia_sau_giam_dan(45000))
print(gia_sau_giam_dan(45000))
```

:::opt{correct}
40000, 40000, 40000, 35000
:::

:::opt
40000, 40000, 40000, 40000
::why
Gần đúng ở chỗ bạn đang giữ đúng niềm tin mà cả track này dựng lên: gọi một hàm
với cùng một đối số thì nhận về cùng một kết quả. Ba dòng đầu bạn đọc chính xác
tuyệt đối.

Chỗ lệch nằm ở dòng `global so_lan_goi`. Nó mở một cửa sau: lời gọi thứ nhất
đẩy `so_lan_goi` lên 1, và lời gọi thứ hai bắt đầu trên một nền đã khác. Đối số
`45000` thì vẫn thế, nhưng đối số không còn là thứ duy nhất quyết định kết quả
nữa. Đó đúng là chỗ niềm tin gãy — và cũng là lý do hàm thứ hai không thuần
khiết.
::
:::

:::opt
40000, 35000, 40000, 35000
::why
Gần đúng ở chỗ bạn bắt được đúng nhịp "lần sau khác lần trước" của hàm có ghi
sổ: nó giảm 5 nghìn ở lần đầu rồi 10 nghìn ở lần sau, nên dòng thứ tư ra 35
nghìn là chuẩn.

Chỗ lệch là bạn áp nhịp ấy cho cả hàm thứ nhất. Nhìn kỹ thân `gia_sau_giam`:
nó có đúng một dòng, và dòng đó chỉ nhắc tới `gia`. Không có `global`, không có
cái tên nào ở ngoài, không có gì nhớ được lần gọi trước. Một hàm không có chỗ
để nhớ thì không nhớ được — nó lặp lại chính nó mãi mãi.
::
:::

:::opt
40000, 40000, 35000, 30000
::why
Gần đúng ở chỗ bạn nhớ rằng `so_lan_goi` đếm số lời gọi, và mỗi lần đếm thêm
thì tiền giảm sâu thêm 5 nghìn. Cơ chế bạn nắm đúng.

Chỗ lệch là bạn đếm cả hai lời gọi ở hai dòng đầu vào. Chúng gọi
`gia_sau_giam`, mà thân hàm ấy không có dòng nào chạm tới `so_lan_goi` — nó
không đọc, không gán. Nên lúc dòng thứ ba chạy, `so_lan_goi` vẫn còn là 0, và
lời gọi ấy mới là lời gọi đầu tiên đẩy nó lên 1.
::
:::

:::opt
Máy báo lỗi ở dòng `global so_lan_goi`, vì hai hàm dùng chung một cái tên ở ngoài
::why
Gần đúng ở chỗ bạn để ý tới một chuyện đáng để ý thật: hai hàm đứng cạnh nhau
và cùng liên quan tới một cái tên nằm ngoài — nghe có mùi va chạm.

Chỗ lệch là `global` không đòi độc quyền. Nó chỉ nói với máy rằng phép gán
trong hàm này trỏ ra cái tên ngoài kia chứ đừng đẻ tên cục bộ. Bao nhiêu hàm
khai báo cùng một tên cũng được, và đó chính là chỗ khó chịu: không có va chạm
nào để máy báo, chỉ có một cái tên bị nhiều nơi sửa mà không nơi nào nói ra.
::
:::
::::

::::explain{#khong-phai-ham-nao-cung-thuan-khiet}
Một câu phải nói rõ trước khi bạn viết: bài này không bảo mọi hàm đều phải
thuần khiết.

Chương trình nào rồi cũng phải hỏi người dùng, in ra màn hình, ghi vào sổ. Một
chương trình thuần khiết từ đầu tới cuối thì chạy xong chẳng ai biết nó đã chạy.
`print` và `input` — hai hàm bạn dùng nhiều nhất — đều không thuần khiết, và
chúng vẫn cần thiết.

Điều bài này đưa cho bạn là một **cách chia**. Phần tính toán tách ra thành
những hàm thuần khiết: đưa số vào, nhận số ra, thử một mình được. Phần chạm vào
thế giới — hỏi, in, ghi — gom lại một chỗ, càng ít càng dễ nhìn.

Chia như vậy thì lúc hoá đơn ra sai, bạn biết ngay phải mở chỗ nào: những hàm
thuần khiết đã được `assert` soi từng cái một, chỗ còn lại thì bé.
::::

::::code{#viet-mot-ham-thuan-khiet}
Quán có một cái tên đứng ngoài, `tong_ca_ngay`, cộng dồn tiền cả ngày. Việc cộng
dồn ấy là của người gọi, không phải của hàm.

Hãy viết thân hàm `tien_ban` cho thuần khiết: nó nhận giá một tô và số tô, đưa
ra tiền của riêng bàn ấy, và không chạm vào bất cứ thứ gì nằm ngoài nó.

```python title=starter
tong_ca_ngay = 0

def tien_ban(gia_mot_to, so_to):
    """Đưa vào giá một tô và số tô, đưa ra tiền của riêng bàn ấy."""
    ___

print(tien_ban(45000, 2))
print(tien_ban(45000, 2))
print(tien_ban(60000, 3))
```

```python title=solution
tong_ca_ngay = 0

def tien_ban(gia_mot_to, so_to):
    """Đưa vào giá một tô và số tô, đưa ra tiền của riêng bàn ấy."""
    return gia_mot_to * so_to

print(tien_ban(45000, 2))
print(tien_ban(45000, 2))
print(tien_ban(60000, 3))
```

```python title=test
# Bốn phép kiểm, mỗi phép canh một điều kiện của hàm thuần khiết.
# Ba phép đầu canh "cùng đầu vào thì cùng đầu ra, và đầu ra chỉ do đầu vào
# quyết định". Phép cuối canh "không để lại dấu vết nào bên ngoài" — nó là
# phép duy nhất phân biệt được lời giải thuần khiết với lời giải tiện tay
# cộng luôn vào tổng cả ngày.
assert tien_ban(45000, 2) == 90000, "hai tô phở 45 nghìn thì riêng bàn ấy trả 90 nghìn"
assert tien_ban(45000, 2) == 90000, "gọi lần thứ hai với đúng hai con số cũ phải ra đúng con số cũ — hàm thuần khiết không nhớ gì về lần gọi trước"
assert tien_ban(60000, 3) == 180000, "ba tô 60 nghìn thì bàn ấy trả 180 nghìn — công thức phải dùng cả hai con số đưa vào, không phải một con số viết cứng"
assert tong_ca_ngay == 0, "`tong_ca_ngay` nằm ngoài hàm, và hàm thuần khiết không chạm vào nó — sau ba lời gọi nó vẫn phải bằng 0"
```

:::hints
- kind: attention
  body: Hàm có hai tham số, và cả hai đều cần dùng tới. Thân hàm chỉ có đúng một dòng — dòng đưa con số ra ngoài, chứ không phải dòng in nó lên màn hình.
- kind: strategy
  body: Tiền của một bàn là giá một tô nhân với số tô. Con số ấy phải đi ra bằng đường đưa-ra, không phải đường hiện-lên-màn-hình, vì ba dòng `print` phía dưới đang đợi nhận nó. Và đừng nhắc tới `tong_ca_ngay` trong thân hàm: cái tên ấy nằm ngoài, việc cộng dồn là của người gọi.
- kind: one-line
  body: 'Viết `return gia_mot_to * so_to` vào chỗ trống, thụt vào bốn dấu cách cho thẳng hàng với dòng mô tả ngay trên nó.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^90000\n90000\n180000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hàm này mình hỏi lúc nào cũng được. Hỏi xong, mọi thứ quanh nó vẫn y nguyên.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte cầm luật mới đi viết một hàm ghi món vào thực đơn, và viết thế này:

```python
def them_mon(mon, thuc_don=[]):
```

Soi lại ba điều của bài này xem: không `global`, nên nó không đọc trộm cái tên
nào ở ngoài. Không sửa gì của ai, vì chỗ trống thứ hai bỏ trống thì nó tự có
một thực đơn rỗng để ghi vào. Nó `return` thực đơn ra đàng hoàng. Trông thuần
khiết đúng luật.

Gọi nó hai lần rồi nhìn kỹ kết quả lần thứ hai.

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
