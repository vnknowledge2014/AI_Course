---
id: nen-tang.ham-vien-gach.chu-ky-khong-giau-gi
title: Chữ ký không giấu gì
summary: Mọi thứ hàm cần đi vào qua tham số, mọi thứ hàm tạo ra đi ra qua `return` — hai cửa, không hơn.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.closed-signature]
requires: [core.pure-function, core.mutable-default, core.side-effect, core.argument-not-copy, core.global-keyword, core.function-signature, core.function-parameter, core.function-return, core.docstring, core.list-append, core.string-method, core.len, core.fstring]
concepts: [core.ham, core.tham-so, core.tra-ve]
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
Ba lần vừa rồi hàm đều có một cái cửa mà chữ ký không hề nói. Giờ bịt hết lại.
::::

::::explain{#ba-cai-cua-khong-ai-khai}
Ba bài vừa rồi để lại đúng một câu hỏi: **một chữ ký không giấu gì thì trông
thế nào?**

Nhìn lại ba chỗ đã làm bạn khốn đốn, và nhìn chúng cạnh nhau:

- Hàm gán vào một cái tên nằm ngoài mình — hoặc đi lấy một cái tên ngoài mình
  về dùng. Người gọi đưa vào đúng một con số, mà kết quả lại phụ thuộc thêm một
  thứ họ không đưa, và một thứ ngoài kia lặng lẽ đổi theo.
- Hàm nhận một danh sách rồi `.append()` vào đó. Danh sách của người gọi đổi
  theo, dù họ không bảo hàm sửa gì cả.
- Hàm mang một giá trị mặc định sống dai, nhớ luôn dữ liệu của lần gọi trước.

Ba chuyện nghe khác nhau, nhưng chúng là **một** chuyện: hàm có một đường vào
hoặc một đường ra mà dòng `def` không khai ra.

Hãy gọi những đường ấy là **cửa sau**.

Một cái quán tử tế có đúng hai cửa, và cả hai đều nhìn thấy được từ ngoài
đường: cửa trước để hàng đi vào, cửa sau kho để hàng đi ra. Ai đứng ngoài cũng
biết cái gì vào, cái gì ra, mà không phải bước vào bếp. Cái quán làm bạn sợ là
cái quán có một lối nhỏ ăn thông sang nhà bên cạnh — không ai vẽ nó trên bản đồ,
nhưng hàng vẫn đi qua đó.

Hàm cũng vậy, và luật của nó gọn hết mức:

> Mọi thứ hàm **cần**, người gọi đưa vào qua **tham số**.
> Mọi thứ hàm **tạo ra**, hàm đưa ra qua **`return`**.
> Không cửa nào khác.

Một hàm theo đúng hai câu ấy thì gọi là có **chữ ký kín**. Và chữ ký kín trả
lại đúng lời hứa của bài 2: đọc tên hàm, đọc danh sách tham số, đọc thứ nó trả
về — biết ba điều đó là gọi được, không phải mở thân hàm ra xem.

Với hàm không kín thì ba điều đó chưa đủ. Bạn vẫn phải mở nó ra, vì cái quyết
định kết quả có khi nằm ở dòng thứ chín trong thân.
::::

::::example{#cua-sau-thu-nhat}
Cửa sau dễ thấy nhất là cửa **vào**: hàm tự đi lấy một thứ mà chữ ký không xin.

Hai hàm dưới đây tính cùng một việc — tiền món cộng phí ship. Hàm thứ nhất tự đi
lấy phí ship ở ngoài; hàm thứ hai bắt người gọi đưa vào.

```python title=readonly
phi_ship = 15000

def tong_ho(gia_mon):
    return gia_mon + phi_ship

def tong_kin(gia_mon, phi):
    return gia_mon + phi

print(tong_ho(45000))
print(tong_kin(45000, 15000))

phi_ship = 30000

print(tong_ho(45000))
print(tong_kin(45000, 15000))
```

Máy in ra:

```text title=readonly
60000
60000
75000
60000
```

Bốn dòng, mà hai dòng cuối tách hẳn ra làm hai số phận.

`tong_kin(45000, 15000)` cho `60000` ở cả hai lần. Nó không hề biết trên đời có
cái tên `phi_ship`; thứ nó cộng vào là con số đã đi qua cửa trước. Đọc dòng
`def tong_kin(gia_mon, phi)` là biết trọn: hai con số vào, một con số ra.

`tong_ho(45000)` thì đổi từ `60000` thành `75000` — cùng một lời gọi, cùng một
đối số, hai kết quả. Dòng `def tong_ho(gia_mon)` nói rằng hàm cần **một** thứ.
Nó nói dối: hàm cần hai, và thứ thứ hai nó tự đi lấy qua cửa sau.

Chỗ đắt nhất của chuyện này: `tong_ho` không hỏng. Nó chạy, nó cho ra một con số
trông rất hợp lý, và không có dòng lỗi nào. Người đọc chỉ phát hiện ra khi ngồi
dò từng dòng — hoặc khi khách phàn nàn về hoá đơn.
::::

::::predict{#cua-sau-thu-hai commitOnce}
Cửa sau còn có loại khó thấy hơn: cửa **ra**.

Byte viết một hàm thêm món vào thực đơn. Nó nhận danh sách qua cửa trước đàng
hoàng, và nó có cả một dòng `return`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra hai dòng nào?

```python title=readonly
thuc_don = ["Phở bò"]

def ghi_them(danh_sach, mon):
    danh_sach.append(mon)
    return len(danh_sach)

so_mon = ghi_them(thuc_don, "Trà đá")
print(so_mon)
print(thuc_don)
```

:::opt{correct}
2 rồi ['Phở bò', 'Trà đá']
:::

:::opt
2 rồi ['Phở bò']
::why
Gần đúng ở chỗ khó: bạn đọc `return len(danh_sach)` sau khi đã thêm một món,
nên ra `2`. Phần ấy chính xác từng bước.

Chỗ lệch nằm ở cái tên `thuc_don` ngoài kia. Bài 25 đã cho thấy: hàm nhận đúng
danh sách của người gọi chứ không phải một bản chép. `danh_sach` bên trong và
`thuc_don` bên ngoài là hai cái tên dán lên **cùng một** danh sách, nên
`.append` bên trong làm danh sách ngoài dài thêm thật.

Nghĩ rằng hàm được phát một bản sao là suy nghĩ tự nhiên — và nó đúng với con
số, chỉ không đúng với danh sách.
::
:::

:::opt
1 rồi ['Phở bò', 'Trà đá']
::why
Gần đúng ở chỗ bạn nhận ra danh sách ngoài kia có đổi — đó là nửa khó của bài
này, và bạn đọc đúng.

Chỗ lệch là thứ tự hai dòng trong thân hàm. Máy chạy từ trên xuống:
`danh_sach.append(mon)` xong rồi mới tới `return len(danh_sach)`. Lúc đếm, món
mới đã nằm trong danh sách rồi, nên con số đếm được là `2`.

Nếu hai dòng ấy đổi chỗ cho nhau thì `1` mới là đáp án — và đó là một cách tốt
để tự kiểm: đọc thân hàm theo đúng thứ tự dòng, đừng đọc theo ý nghĩa cái tên.
::
:::

:::opt
None rồi ['Phở bò', 'Trà đá']
::why
Gần đúng ở chỗ quan trọng nhất: bạn thấy hàm này sửa thứ của người gọi, và đó
đúng là điều bài đang muốn bạn nhìn ra.

Chỗ lệch là ở chữ "hoặc". Bạn đang đọc như thể một hàm phải chọn: hoặc sửa đồ
của người khác, hoặc trả một giá trị về. Hàm này làm **cả hai** — dòng
`return len(danh_sach)` vẫn đưa ra một con số đàng hoàng.

Và đúng chỗ ấy mới là cái bẫy: chữ ký có khai một đường ra, nên người đọc yên
tâm rằng đường ra chỉ có một. Đường thứ hai vẫn mở, lặng lẽ, ngay dòng trên.
::
:::
::::

::::explain{#hai-cua-va-khong-hon}
`ghi_them` có một cửa ra được khai — `return len(danh_sach)` — và một cửa ra
không được khai: cái danh sách đi vào rồi đi ra khác lúc vào.

Nên câu luật lúc nãy cần đọc kỹ hơn một chút. Thứ bị cấm không phải là *sửa
đồ của người gọi* — mà là **sửa lén**:

> Mọi thứ hàm tạo ra đi ra qua `return`. Còn nếu hàm sinh ra để sửa chính thứ
> nó nhận, thì cái tên và dòng mô tả phải nói thẳng điều đó ra.

`ghi_them` phạm đúng vế sau: cái tên hứa "thêm một món vào thực đơn" nghe như
một việc của riêng nó, dòng mô tả không nhắc gì tới chuyện thực đơn của người
gọi dài thêm, mà nó vẫn dài thêm. Một hàm tên `ghi_vao_thuc_don(thuc_don, mon)`
làm đúng việc ấy thì không lén chút nào — bạn đọc tên là biết.

Vậy khi bạn thật sự muốn thực đơn dài thêm thì làm sao? Đưa việc gắn vào cho
**người gọi**. Hàm lo phần tính, người gọi lo phần cất:

```python title=readonly
thuc_don = ["Phở bò"]

def mon_viet_hoa(mon):
    """Nhận tên một món, trả về tên ấy viết in hoa."""
    return mon.upper()

thuc_don.append(mon_viet_hoa("trà đá"))
print(thuc_don)
```

Máy in ra:

```text title=readonly
['Phở bò', 'TRÀ ĐÁ']
```

Danh sách vẫn dài thêm, nhưng dòng làm nó dài thêm nằm **ngoài** hàm, ở chỗ ai
đọc cũng thấy. `mon_viet_hoa` giữ đúng hai cửa: một tên món vào, một tên món ra.

Đây là cách đọc lại toàn bộ track bằng một câu. Bài 2 nói: biết tên, biết thứ
vào, biết thứ ra là gọi được. Bài 20 tới 27 cho thấy bốn cách khiến câu ấy sai.
Bài này chốt lại: câu ấy đúng khi và chỉ khi hàm có đúng hai cửa và không giấu
cửa nào.
::::

::::code{#dong-het-cua-sau}
Byte đang viết máy tính tiền cho quán, và bản đầu tiên có `global`:

```python title=readonly
tong_thu = 0

def ban_mot_mon(gia, so_luong):
    global tong_thu
    tong_thu = tong_thu + gia * so_luong
```

Hàm này có một cửa vào khai đủ, nhưng cửa ra thì giấu: nó không `return` gì cả,
mà kết quả vẫn ra ngoài được — qua `tong_thu`.

Bản kín để hàm lo đúng một việc: tính tiền của **riêng lần bán này** rồi đưa con
số ấy ra. Ai muốn cộng dồn thì tự cộng, ở ngoài.

Sáng nay quán bán hai lượt: hai tô phở 45 nghìn một tô, rồi ba suất cơm rang 25
nghìn một suất. Hai chỗ trống nằm ở hai phía của cùng một cái cửa ra — một chỗ
là nơi con số đi ra, một chỗ là nơi nó được nhận về.

```python title=starter
def tien_mot_mon(gia, so_luong):
    """Nhận giá một món và số lượng, trả về thành tiền của riêng lần bán này."""
    ___


tong_thu = 0
tong_thu = tong_thu + ___
tong_thu = tong_thu + tien_mot_mon(25000, 3)
print(f"Tổng thu: {tong_thu} đồng")
```

```python title=solution
def tien_mot_mon(gia, so_luong):
    """Nhận giá một món và số lượng, trả về thành tiền của riêng lần bán này."""
    return gia * so_luong


tong_thu = 0
tong_thu = tong_thu + tien_mot_mon(45000, 2)
tong_thu = tong_thu + tien_mot_mon(25000, 3)
print(f"Tổng thu: {tong_thu} đồng")
```

```python title=test
# Ba phép kiểm cho ba chuyện khác nhau, và phép giữa là phép nói về chữ ký kín.
assert tien_mot_mon(20000, 4) == 80000, "bốn món hai mươi nghìn thì lần bán ấy thu 80 nghìn — hàm phải ĐƯA con số đó ra qua `return`, in nó lên màn hình thì người gọi không nhận được gì"
assert tien_mot_mon(45000, 0) == 0, "bán không món nào thì lần bán ấy thu 0 đồng, và hàm vẫn phải trả về một con số chứ không phải None"
truoc = tong_thu
tien_mot_mon(99000, 9)
assert tong_thu == truoc, "gọi `tien_mot_mon` mà `tong_thu` nhúc nhích nghĩa là hàm vẫn còn một cửa sau — mọi thứ hàm tạo ra phải đi ra qua `return`"
assert tong_thu == 165000, "hai tô phở 45 nghìn rồi ba suất cơm rang 25 nghìn: chỗ trống thứ hai phải nhận về tiền của LƯỢT BÁN ĐẦU"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất là dòng duy nhất trong thân hàm, nên nó phải làm trọn việc mà docstring hứa — và docstring nói "trả về". Chỗ trống thứ hai nằm ở dòng cộng dồn thứ nhất, ngay cạnh một dòng cộng dồn đã viết sẵn cho bạn xem mẫu.
- kind: strategy
  body: Thành tiền của một lượt bán là giá một món nhân với số lượng, và con số ấy phải đi ra ngoài chứ không dừng lại trong hàm. Ở dòng cộng dồn, thứ cần cộng vào là kết quả của lượt bán đầu — hai tô phở, mỗi tô 45 nghìn — nên hãy gọi chính cái hàm bạn vừa viết với đúng hai con số ấy.
- kind: one-line
  body: "Chỗ trống thứ nhất là `return gia * so_luong`; chỗ trống thứ hai là `tien_mot_mon(45000, 2)`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  expect: Tổng thu: 165000 đồng
- tier: static
  onFail: cả hai lượt bán đều phải đi qua hàm `tien_mot_mon` — chép sẵn một con số vào dòng cộng dồn là bỏ mất cái cửa trước
  requireAst:
  - kind: uses-call, target: tien_mot_mon, min: 2
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cửa, và cả hai đều nằm trên dòng `def`. Không còn lối nào đi vòng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte đưa bạn một hàm mới. Nó kín tuyệt đối theo đúng luật bạn vừa học: không
`global`, không sửa thứ của người gọi, không giá trị mặc định nhớ dai. Mọi thứ
cần thì đi vào qua tham số, mọi thứ tạo ra thì đi ra qua `return`.

Chữ ký của nó là `xu_ly(a, b, c, d)`.

Nó kín hoàn toàn theo đúng luật trên, mà vẫn chẳng ai dám gọi. Nó còn thiếu cái
gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.85}
::::
