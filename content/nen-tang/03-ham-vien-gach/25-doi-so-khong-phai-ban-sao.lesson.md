---
id: nen-tang.ham-vien-gach.doi-so-khong-phai-ban-sao
title: Đối số không phải bản sao
summary: Hàm nhận đúng cuốn sổ của người gọi chứ không phải bản chép — `.append()` bên trong sửa luôn cuốn sổ ngoài kia, dù chẳng có `global` nào.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.argument-not-copy]
requires: [core.function-def, core.function-call, core.function-parameter, core.function-argument, core.function-return, core.list, core.list-append, core.variable, core.assignment, core.fstring, core.output]
concepts: [core.ham, core.tham-so, core.danh-sach, core.sua-duoc]
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
Mình bỏ `global` đi rồi. Vậy mà cuốn sổ ngoài kia vẫn dài thêm một dòng.
::::

::::explain{#dua-cuon-so-hay-dua-ban-photo}
Bài trước rủ bạn thử đúng một việc: bỏ `global` đi, truyền một danh sách vào
hàm, `.append()` một món bên trong, rồi in cái danh sách ở ngoài ra xem.

Suy đoán tự nhiên là: bỏ `global` thì hàm hết đường chạm ra ngoài. Bài 22 còn
nói chắc hơn thế — mọi phép gán trong thân hàm đều đẻ ra một cái tên cục bộ
mới, biến ngoài kia không suy suyển. Vậy thì danh sách ngoài kia cũng phải yên.

Nó không yên. Và lý do nằm ở một chỗ chưa bài nào nói tới: **thứ đi vào hàm là
gì**.

Hình dung bà chủ quán phở đưa cuốn sổ gọi món cho người phụ bếp. Bà có hai
cách đưa:

- **Đưa bản photo.** Người phụ bếp cầm một xấp giấy riêng. Anh ta ghi thêm bao
  nhiêu dòng vào đó cũng được — cuốn sổ thật vẫn nằm trên quầy, y nguyên.
- **Đưa chính cuốn sổ.** Bây giờ hai người cùng đứng trước một cuốn. Anh ta ghi
  một dòng, bà chủ nhìn xuống là thấy dòng ấy.

Python chọn cách thứ hai. Lúc bạn viết `them_mon(so_cua_quan)`, máy **không**
chép cuốn sổ ra thành một bản mới cho hàm. Nó đưa cho tham số bên trong đúng
cuốn sổ đang nằm ngoài kia. Cái tên bên trong khác, cái tên bên ngoài khác —
nhưng chúng chỉ là hai tấm nhãn dán lên **một** cuốn.

Nên khi thân hàm chạy `.append()`, món mới được viết vào cuốn sổ thật. Không có
`global` nào ở đây cả, và cũng chẳng cần: `global` là chuyện của những cái
**tên**, còn chuyện đang xảy ra là chuyện của cuốn **sổ**.
::::

::::example{#nhin-tan-mat}
Đoạn dưới không có một chữ `global` nào. Cả bài chỉ có một hàm ba dòng.

```python title=readonly
def them_mot_mon(so_goi_mon):
    """Ghi thêm món nạm vào cuốn sổ được đưa vào."""
    so_goi_mon.append("nạm")

so_cua_ban_ba = ["tái", "chín"]

print(f"Trước khi gọi hàm: {so_cua_ban_ba}")
them_mot_mon(so_cua_ban_ba)
print(f"Sau khi gọi hàm:   {so_cua_ban_ba}")
```

Máy in ra:

```text
Trước khi gọi hàm: ['tái', 'chín']
Sau khi gọi hàm:   ['tái', 'chín', 'nạm']
```

Hàm không `return` gì. Người gọi không gán lại gì. Vậy mà dòng cuối in ra một
cuốn sổ dài hơn dòng đầu đúng một món.

Đi lại đường máy đi:

- `so_cua_ban_ba` được dán lên một cuốn sổ có hai dòng.
- Lời gọi `them_mot_mon(so_cua_ban_ba)` dán thêm tấm nhãn thứ hai — tấm
  `so_goi_mon` — lên **đúng cuốn ấy**. Hai nhãn, một cuốn.
- Thân hàm chạy `so_goi_mon.append("nạm")`. Món được viết vào cuốn.
- Hàm xong, tấm nhãn `so_goi_mon` bị gỡ đi đúng như bài 20 đã nói. Nhưng dòng
  chữ vừa viết thì không ai gỡ được — nó nằm trong cuốn sổ, không nằm trên
  tấm nhãn.
::::

::::explain{#dan-lai-nhan-khac-viet-vao-so}
Tới đây có một chỗ dễ vấp, và nó là chỗ đáng nhìn kỹ nhất bài này: bài 22 nói
gán trong hàm không chạm tới bên ngoài, còn bài này nói `.append()` chạm tới
bên ngoài. Hai câu ấy không đá nhau, vì chúng nói về hai việc khác nhau.

- `so_goi_mon = ["gì đó khác"]` là một phép **gán**. Nó gỡ tấm nhãn
  `so_goi_mon` khỏi cuốn sổ cũ rồi dán sang một cuốn khác. Cuốn cũ không hề bị
  đụng tới, và tấm nhãn ngoài kia vẫn dán nguyên chỗ cũ.
- `so_goi_mon.append("nạm")` **không** gán gì cả. Không có dấu `=` nào. Nó cầm
  đúng cuốn mà tấm nhãn đang dán vào rồi viết thêm một dòng.

Một câu để nhớ: **gán đổi tấm nhãn, `.append()` sửa cuốn sổ**. Bài 22 nói về
tấm nhãn. Bài này nói về cuốn sổ.

Và đây là chỗ đau thật sự, chứ không phải một chi tiết vui: bạn đọc dòng
`them_mot_mon(so_cua_ban_ba)` mà không mở hàm ra xem thì **không có cách nào**
biết cuốn sổ của mình vừa bị viết thêm. Chữ ký `them_mot_mon(so_goi_mon)` không
nói gì về chuyện đó. Bài 24 gọi tên thứ này rồi: một tác dụng phụ. Nó chỉ đổi
lối vào — lần trước đi qua `global`, lần này đi thẳng qua tham số.
::::

::::predict{#doan-cuon-so-sau-cung commitOnce}
Byte viết một hàm có cả hai thứ vừa nói: một `.append()`, một phép gán, rồi lại
một `.append()` nữa.

**Trước khi bấm chạy**, bạn đoán dòng cuối in ra cuốn sổ nào?

```python
def sua_thuc_don(thuc_don):
    thuc_don.append("nạm")
    thuc_don = ["chỉ còn nước lã"]
    thuc_don.append("gầu")
    return thuc_don

thuc_don_quan = ["tái", "chín"]
sua_thuc_don(thuc_don_quan)

print(thuc_don_quan)
```

:::opt{correct}
`['tái', 'chín', 'nạm']`
:::

:::opt
`['tái', 'chín']`
::why
Gần đúng ở chỗ bạn đang áp một luật có thật và đã được dạy hẳn hoi: bài 22 nói
việc bên trong hàm không chạm tới bên ngoài, nên cuốn sổ của quán phải nguyên
vẹn. Suy luận ấy đúng với mọi phép **gán**.

Chỗ lệch nằm ở dòng đầu thân hàm. `thuc_don.append("nạm")` không có dấu `=`
nào, nên nó không đẻ ra cái tên cục bộ nào cả — nó cầm đúng cuốn sổ mà lời gọi
vừa đưa vào và viết thêm một dòng. Bài 22 canh cửa cho những cái tên; cuốn sổ
thì đi vào bằng cửa khác.
::
:::

:::opt
`['chỉ còn nước lã', 'gầu']`
::why
Gần đúng ở chỗ bạn theo dõi thân hàm rất sát: dòng giữa quả thật dán tên
`thuc_don` sang một cuốn mới, và dòng sau đó ghi "gầu" vào đúng cuốn mới ấy.
Hai dòng ấy bạn đọc không sai một chữ.

Chỗ lệch là **cuốn mới ấy chỉ có tên bên trong hàm**. Phép gán ở dòng giữa gỡ
tấm nhãn `thuc_don` khỏi cuốn sổ của quán rồi dán sang cuốn khác — nó không hề
đụng tới tấm nhãn `thuc_don_quan` ngoài kia. Hàm xong, tấm nhãn bên trong biến
mất cùng cuốn mới, và `print` ngoài kia vẫn đang nhìn cuốn cũ.
::
:::

:::opt
`['tái', 'chín', 'nạm', 'gầu']`
::why
Gần đúng ở chỗ khó nhất: bạn nhận ra `.append()` viết thẳng vào cuốn sổ của
người gọi, nên dòng "nạm" nằm lại được. Đó chính là điều bài này dạy.

Chỗ lệch nằm ở dòng "gầu". Giữa hai lần `.append()` có một phép gán, và phép
gán ấy đã kéo tấm nhãn `thuc_don` sang một cuốn khác. Từ dòng đó trở đi,
`thuc_don.append(...)` viết vào cuốn khác chứ không còn viết vào cuốn của quán
nữa. Thứ tự các dòng quyết định mỗi lần `.append()` rơi vào cuốn nào.
::
:::

:::opt
Máy báo lỗi, vì hàm sửa một danh sách mà không khai báo `global`
::why
Gần đúng ở chỗ bạn nhớ đúng vai của `global`: muốn một phép **gán** trong hàm
trỏ ra biến ngoài thì phải khai báo, không khai báo thì cái tên ngoài kia
không đổi.

Nhưng máy không báo lỗi trong trường hợp ấy. Bài 22 đã cho thấy: thiếu
`global`, phép gán vẫn chạy trót lọt — nó chỉ đẻ ra một cái tên cục bộ mới rồi
bỏ đi cùng lượt gọi. Máy chưa bao giờ chặn chuyện này.

Chỗ lệch là ở đây chẳng có phép gán nào nhắm ra ngoài cả. Cuốn sổ tự đi vào hàm
qua tham số, đàng hoàng, đúng cửa chính — nên máy không có gì để chặn. Đó mới
là điều đáng lo: không một tiếng báo nào, mà cuốn sổ vẫn đổi.
::
:::
::::

::::explain{#vay-thi-nguy-o-cho-nao}
Cùng một cuốn sổ đi vào nhiều hàm khác nhau, và hàm nào cũng có quyền viết vào
đó — nghe thì tiện, nhưng nó phá đúng cái trục mà cả track này dựng lên.

Nhớ lại bài 1: bạn gọi `len("phở")` được mà không cần mở nó ra xem, vì bạn tin
rằng nó đưa vào một thứ và đưa ra một con số, hết. Bây giờ nhìn dòng này:

```python
so_dong = len(so_cua_quan)
kiem_tra_thuc_don(so_cua_quan)
```

Sau dòng thứ hai, `so_cua_quan` còn đúng `so_dong` dòng nữa không? Đọc tên hàm
thì thấy nó chỉ "kiểm tra". Đọc chữ ký thì thấy nó nhận một danh sách. Cả hai
đều im lặng về chuyện nó có viết vào cuốn sổ hay không.

Muốn biết chắc, bạn phải mở hàm ra đọc. Mà "phải mở ra đọc" chính là thứ bài 1
hứa bạn sẽ không phải làm.
::::

::::code{#ghi-vao-dung-cuon-duoc-dua}
Byte muốn một hàm ghi món ăn vào cuốn sổ, và ghi vào **đúng cuốn được đưa
vào** — hôm nay là sổ của quán, mai có thể là sổ của bàn nào đó.

Hàm nhận hai thứ: cuốn sổ và tên món. Nó không `return` gì, vì việc của nó là
viết vào cuốn sổ ấy.

Hãy điền dòng còn thiếu trong thân hàm.

```python title=starter
def ghi_mon(so_goi_mon, mon):
    """Ghi thêm một món vào đúng cuốn sổ được đưa vào."""
    ___

so_cua_quan = ["tái", "chín"]

ghi_mon(so_cua_quan, "nạm")
ghi_mon(so_cua_quan, "gầu")

print(so_cua_quan)
```

```python title=solution
def ghi_mon(so_goi_mon, mon):
    """Ghi thêm một món vào đúng cuốn sổ được đưa vào."""
    so_goi_mon.append(mon)

so_cua_quan = ["tái", "chín"]

ghi_mon(so_cua_quan, "nạm")
ghi_mon(so_cua_quan, "gầu")

print(so_cua_quan)
```

```python title=test
# Chấm trên MỘT CUỐN SỔ THỨ HAI — cuốn mà đoạn code trên không hề nhắc tên.
# Một lời giải ghi thẳng vào `so_cua_quan` (cái tên nằm ngoài, hàm vẫn đọc
# được nó theo bài 21) sẽ cho ra đúng dòng in mong đợi, nhưng cuốn thứ hai
# này thì nó không đụng tới — và đó là chỗ phân biệt.
so_cua_ban_bay = ["nạm"]
ghi_mon(so_cua_ban_bay, "gân")
assert so_cua_ban_bay == ["nạm", "gân"], "hàm phải ghi vào đúng cuốn sổ ĐƯỢC ĐƯA VÀO: đưa cuốn của bàn bảy vào thì món 'gân' phải nằm trong cuốn ấy"
assert so_cua_quan == ["tái", "chín", "nạm", "gầu"], "hai lời gọi phải để lại hai món mới trong sổ của quán, đúng thứ tự đã gọi: nạm trước, gầu sau"
```

:::hints
- kind: attention
  body: Hàm có hai tham số. Một trong hai là cuốn sổ, một là dòng chữ sắp được ghi vào cuốn ấy. Việc cần làm là viết một dòng mới vào cuốn — không phải dán lại tấm nhãn nào.
- kind: strategy
  body: Thêm một phần tử vào cuối một danh sách là việc bạn đã làm từ Realm 0, bằng một lệnh gắn sau tên danh sách và dấu chấm. Dòng này không có dấu `=`, và cũng không cần `return` — chính vì thế mà cuốn sổ ngoài kia đổi theo.
- kind: one-line
  body: 'Viết `so_goi_mon.append(mon)` vào chỗ trống, thụt vào bốn dấu cách cho thẳng hàng với dòng mô tả ngay trên nó.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: ['tái', 'chín', 'nạm', 'gầu']
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cuốn nào được đưa vào thì mình ghi vào đúng cuốn ấy. Không ghi nhầm sang cuốn khác.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hai bài vừa rồi cho bạn hai cách để một hàm chạm ra ngoài: bài 24 đi qua
`global`, bài này đi qua chính đối số. Cả hai đều dẫn tới cùng một chỗ — gọi
hàm hai lần với đúng một đối số mà nhận về hai kết quả khác nhau, hoặc để lại
hai bãi khác nhau phía sau.

Vậy loại hàm nào mới đáng tin — loại gọi bao nhiêu lần, gọi vào lúc nào cũng
cho ra cùng một kết quả?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
