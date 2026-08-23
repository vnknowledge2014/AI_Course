---
id: nen-tang.ham-vien-gach.bay-cua-gia-tri-mac-dinh
title: Cái bẫy nằm trong giá trị mặc định
summary: Giá trị mặc định được tạo một lần lúc máy đọc dòng `def`, rồi mọi lời gọi về sau dùng chung — nên một danh sách mặc định nhớ luôn dữ liệu của lần gọi trước.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [core.mutable-default]
requires: [core.pure-function, core.argument-not-copy, core.function-def, core.function-call, core.function-parameter, core.function-argument, core.function-return, core.list, core.list-append, core.len, core.variable, core.assignment, core.output]
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
Mình gọi hai lần. Lần thứ hai, thực đơn có sẵn món của lần thứ nhất trong đó.
::::

::::explain{#lan-thu-hai-khong-giong-lan-dau}
Bài trước để bạn ra về với một hàm trông sạch sẽ:

```python
def them_mon(mon, thuc_don=[]):
    """Ghi một món vào thực đơn rồi đưa thực đơn ra."""
    thuc_don.append(mon)
    return thuc_don
```

Soi ba điều của hàm thuần khiết thì nó qua hết: không `global`, không đọc cái
tên nào ở ngoài, có `return` đàng hoàng. Chỗ trống thứ hai bỏ trống thì nó tự
có một thực đơn rỗng để ghi vào — nghe rất hợp lý.

Gọi ba lần, mỗi lần một món:

```python
print(them_mon("tái"))
print(them_mon("nạm"))
print(them_mon("gầu"))
```

Nếu nó thuần khiết thật thì ba dòng in ra phải là ba thực đơn một món:
`['tái']`, `['nạm']`, `['gầu']`. Ba lời gọi độc lập, ba thực đơn rỗng, mỗi cái
nhận đúng một món.

Máy in ra thứ khác. Bài này nói vì sao.
::::

::::example{#ba-lan-goi-mot-cuon-so}
Chạy thật, không sửa gì:

```python title=readonly
def them_mon(mon, thuc_don=[]):
    """Ghi một món vào thực đơn rồi đưa thực đơn ra."""
    thuc_don.append(mon)
    return thuc_don

print(them_mon("tái"))
print(them_mon("nạm"))
print(them_mon("gầu"))
```

Máy in ra:

```text
['tái']
['tái', 'nạm']
['tái', 'nạm', 'gầu']
```

Thực đơn dài thêm mỗi lần gọi. Lời gọi thứ ba nhận về một thực đơn đã có sẵn
hai món mà nó chưa từng thấy.

Lý do nằm ở một chuyện chưa bài nào nói ra: **dòng `def` cũng là một mệnh lệnh,
và máy chạy nó đúng một lần.**

Từ trước tới giờ bạn nhìn `def` như một tờ giấy dán tường — chữ nghĩa nằm đó,
chờ ai gọi thì mới sống dậy. Đúng với phần *thân* hàm: thân chỉ chạy lúc có lời
gọi. Nhưng bản thân dòng `def` thì máy chạy ngay khi đọc tới, y như một dòng
gán. Chạy để làm gì? Để dựng cái hàm lên và đặt tên cho nó.

Và ngay tại giây phút ấy, máy cũng tính luôn mọi giá trị mặc định viết trên
dòng đó. Cái `[]` kia được tạo ra **một lần**, lúc máy đọc dòng `def` — trước
cả lời gọi đầu tiên. Nó được cất đi cùng với cái hàm.

Mỗi lời gọi bỏ trống chỗ ấy, máy không tạo danh sách mới. Nó lấy đúng cuốn sổ
đã cất sẵn từ hôm dựng hàm và đưa vào. Ba lời gọi bỏ trống là ba lần cùng một
cuốn sổ đi vào — và bài 25 vừa nói rồi, `.append()` viết thẳng vào cuốn được
đưa vào.

Nên hàm ấy không thuần khiết chút nào. Nó nhớ mọi thứ, chỉ là nó nhớ ở một chỗ
không ai nghĩ tới: bên trong chính dòng `def`.
::::

::::explain{#vi-sao-so-thi-khong-sao}
Câu hỏi tiếp theo là câu đáng hỏi: vậy mọi giá trị mặc định đều là bẫy à? Bài 8
dùng `thue=0.1` khắp nơi mà có sao đâu.

Không sao thật, và lý do đã nằm sẵn trong bài 25.

Giá trị mặc định nào cũng chỉ được tạo một lần, kể cả `0.1`. Khác biệt là ở
chỗ thân hàm **làm gì** với nó:

- Với một con số, thứ duy nhất bạn làm được là **gán**: `thue = thue + 0.05`.
  Mà gán thì đổi tấm nhãn chứ không sửa cái vật — con số cất sẵn trong dòng
  `def` không hề bị đụng tới, lời gọi sau vẫn nhận đúng `0.1`.
- Với một danh sách, bạn còn làm được một việc nữa: `.append()`. Việc ấy sửa
  thẳng cái vật, và cái vật thì chỉ có một.

Cùng một câu của bài 25, lần này rơi vào chỗ khác: **gán đổi tấm nhãn,
`.append()` sửa cuốn sổ.** Số, chuỗi, `True`, `None` — bạn không sửa được
chúng, nên đặt làm mặc định thì an toàn. Danh sách thì sửa được, nên một danh
sách đặt làm mặc định sẽ mang dữ liệu của lần gọi trước sang lần gọi sau.

Còn hôm nay, cách chắc chắn nhất để thoát bẫy là đừng để lời gọi nào bỏ trống
chỗ ấy: cuốn sổ nào cần ghi thì đưa thẳng cuốn ấy vào, đúng như bài 25 đã làm.
::::

::::predict{#doan-bon-con-so commitOnce}
Byte viết hai hàm cùng có một giá trị mặc định. Một hàm để mặc định là số `0`,
một hàm để mặc định là danh sách rỗng.

**Trước khi bấm chạy**, bạn đoán bốn dòng in ra bốn con số nào?

```python
def dem_ban(mon, so_ban=0):
    so_ban = so_ban + 1
    return so_ban

def ghi_mon(mon, so=[]):
    so.append(mon)
    return len(so)

print(dem_ban("tái"))
print(dem_ban("nạm"))
print(ghi_mon("tái"))
print(ghi_mon("nạm"))
```

:::opt{correct}
1, 1, 1, 2
:::

:::opt
1, 1, 1, 1
::why
Gần đúng ở chỗ bạn đọc dòng `def` theo đúng nghĩa mà nó gợi ra: bỏ trống thì
lấy giá trị viết sẵn, nên lần nào cũng bắt đầu lại từ đầu. Với `so_ban=0` thì
đó chính xác là chuyện xảy ra — hai dòng đầu bạn đọc không sai.

Chỗ lệch nằm ở chữ *bắt đầu lại*. Máy không tạo lại `[]` cho mỗi lời gọi; nó
tạo đúng một lần lúc đọc dòng `def`, rồi mọi lời gọi bỏ trống đều nhận về đúng
cuốn ấy. Lời gọi thứ nhất viết "tái" vào, nên lời gọi thứ hai mở ra đã thấy một
món nằm sẵn.
::
:::

:::opt
1, 2, 1, 2
::why
Gần đúng ở chỗ khó nhất: bạn nhận ra giá trị mặc định chỉ được tạo một lần và
sống sót qua các lời gọi. Với `ghi_mon` thì suy luận ấy đúng từng chữ.

Chỗ lệch là `dem_ban` không sửa được cái vật cất sẵn. Dòng
`so_ban = so_ban + 1` là một phép gán: nó gỡ tấm nhãn `so_ban` khỏi số 0 rồi
dán sang số 1. Số 0 cất trong dòng `def` vẫn nằm nguyên đó, nên lời gọi sau lại
nhận về 0 và lại ra 1. Gán đổi tấm nhãn; muốn sửa cái vật thì phải có `.append()`
hoặc thứ tương tự, mà con số thì không có.
::
:::

:::opt
1, 1, 2, 3
::why
Gần đúng ở chỗ bạn giữ đúng ý "cuốn sổ dùng chung dài thêm mỗi lời gọi", và hai
dòng đầu bạn đọc chuẩn.

Chỗ lệch là điểm xuất phát của cuốn sổ ấy. Lúc dòng thứ ba chạy, `ghi_mon` mới
được gọi lần đầu tiên, và cuốn sổ mặc định của nó còn trống trơn — hai lời gọi
phía trên là gọi `dem_ban`, một hàm khác, với một chỗ mặc định khác hẳn. Ghi
một món vào cuốn trống thì `len` cho ra 1.
::
:::

:::opt
Máy báo lỗi ở dòng `def ghi_mon(mon, so=[])`, vì Python không cho đặt danh sách làm giá trị mặc định
::why
Gần đúng ở chỗ bạn suy luận theo một lối rất lành mạnh: bài đang nói tới một
cái bẫy, mà ngôn ngữ thường chặn sẵn những thứ nguy hiểm.

Chỗ lệch là Python cho phép chuyện này hoàn toàn — `[]` là một giá trị hợp lệ y
như `0`, và dòng `def` chạy trót lọt. Cái bẫy không nằm ở chỗ viết được hay
không viết được, mà ở chỗ nó chạy êm ru rồi cho ra kết quả sai mà không báo một
tiếng nào. Đó là loại lỗi tệ nhất, và bạn đã gặp nó ở bài 6 lẫn bài 25.
::
:::
::::

::::code{#dua-thang-cuon-so-can-ghi}
Hàm `them_mon` dưới đây vẫn mang nguyên cái bẫy: chỗ `thuc_don=[]` là một cuốn
sổ dùng chung, sinh ra một lần lúc máy đọc dòng `def`.

Byte có hai bàn, và hai bàn phải có hai cuốn sổ riêng. Bàn một gọi món tái. Bàn
hai gọi món nạm rồi gọi thêm gầu.

Hãy điền vào ba chỗ trống cuốn sổ mà mỗi lời gọi cần ghi vào — đừng để lời gọi
nào bỏ trống chỗ ấy, vì bỏ trống là ghi chung vào cuốn của dòng `def`.

```python title=starter
def them_mon(mon, thuc_don=[]):
    """Ghi một món vào cuốn thực đơn được đưa vào, rồi đưa cuốn ấy ra."""
    thuc_don.append(mon)
    return thuc_don

so_ban_mot = []
so_ban_hai = []

them_mon("tái", ___)
them_mon("nạm", ___)
them_mon("gầu", ___)

print(so_ban_mot)
print(so_ban_hai)
```

```python title=solution
def them_mon(mon, thuc_don=[]):
    """Ghi một món vào cuốn thực đơn được đưa vào, rồi đưa cuốn ấy ra."""
    thuc_don.append(mon)
    return thuc_don

so_ban_mot = []
so_ban_hai = []

them_mon("tái", so_ban_mot)
them_mon("nạm", so_ban_hai)
them_mon("gầu", so_ban_hai)

print(so_ban_mot)
print(so_ban_hai)
```

```python title=test
# Hai phép kiểm đầu canh chuyện hai bàn không lẫn sổ của nhau.
# Phép kiểm thứ ba là phép nói thật về bài này: nó gọi hàm mà BỎ TRỐNG chỗ
# cuốn sổ, nên nó nhận về đúng cuốn sinh ra ở dòng `def`. Cuốn ấy chỉ còn
# trống nếu cả ba lời gọi phía trên đều đã đưa sổ của mình vào; chỉ cần một
# lời gọi bỏ trống là món của nó đã nằm sẵn trong đó và câu trả lời dài hơn.
assert so_ban_mot == ["tái"], "bàn một chỉ gọi mỗi món tái, nên sổ của bàn một phải có đúng một dòng"
assert so_ban_hai == ["nạm", "gầu"], "bàn hai gọi nạm trước rồi gầu sau, nên sổ của bàn hai phải có đúng hai dòng theo thứ tự ấy"
assert them_mon("gân") == ["gân"], "cuốn sổ mặc định của dòng `def` phải còn trống: gọi mà bỏ trống chỗ cuốn sổ thì chỉ được nhận về đúng một món 'gân' — dài hơn nghĩa là có lời gọi phía trên đã ghi chung vào cuốn ấy"
```

:::hints
- kind: attention
  body: Ba chỗ trống nằm ở chỗ thứ hai của ba lời gọi, ngay sau tên món. Hai cuốn sổ đã được dựng sẵn ở hai dòng phía trên, mỗi cuốn một cái tên. Đọc lại xem bàn nào gọi những món nào.
- kind: strategy
  body: Mỗi chỗ trống là tên cuốn sổ của bàn đang gọi món ấy. Bàn một chỉ xuất hiện một lần, bàn hai xuất hiện hai lần — nên trong ba chỗ trống sẽ có một cái tên lặp lại. Đừng viết `[]` vào chỗ trống: một cuốn sổ mới dựng ngay tại lời gọi thì ghi xong không cái tên nào giữ nó lại.
- kind: one-line
  body: 'Lần lượt ba chỗ trống là `so_ban_mot`, `so_ban_hai`, và `so_ban_hai`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: ['nạm', 'gầu']
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai bàn, hai cuốn sổ. Cuốn của dòng `def` nằm đó, trống nguyên, không ai ghi nhờ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ba bài vừa rồi đều là một chuyện: hàm giấu một đường vào hoặc một đường ra mà
chữ ký không hề nói.

- Bài 24: `ghi_doanh_thu(tien)` giấu một đường ra — nó gán vào một biến toàn
  cục.
- Bài 25: `ghi_mon(so, mon)` giấu một đường ra — nó viết vào cuốn sổ của người
  gọi.
- Bài này: `them_mon(mon, thuc_don=[])` giấu một đường vào — nó nhận thêm mọi
  thứ mà những lần gọi trước đã để lại.

Ba lần, ba cửa khác nhau, cùng một hậu quả: đọc chữ ký xong bạn vẫn chưa biết
hàm ấy làm gì với thế giới quanh nó.

Vậy một chữ ký không giấu gì thì trông thế nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
