---
id: nen-tang.gia-tri-bien-kieu.cat-mot-khuc-chuoi
title: Cắt một khúc chuỗi
summary: Hai con số cách nhau dấu hai chấm xin về nguyên một khúc chuỗi — đầu lấy, cuối chừa, và khúc ấy luôn là một chuỗi mới.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.string-slice]
requires: [core.string-immutable, core.string-sequence, core.len, core.string-literal, core.list-index, core.list, core.list-append, ctrl.for-each, core.output]
concepts: [core.chuoi, core.gia-tri]
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
Xin từng ô thì mỏi tay. Nói cho mình biết hai đầu, mình đưa cả khúc.
::::

::::explain{#hai-cai-ghim}
Bài trước kết bằng một dòng dài lê thê: mười hai mảnh nối bằng mười một dấu
cộng, chỉ để lấy 12 ký tự đầu của một tên khoản. Bài này là cách nói ngắn cho
đúng việc ấy.

Trải tên khoản ra trang sổ kẻ ly, mỗi ký tự một ô:

```text
 ô:   0  1  2  3  4  5  6  7  8  9 10 11 12
      c  à     p  h  ê     s  ữ  a     đ  á
```

Bây giờ bà chủ muốn lấy phần vừa cột 12 chỗ. Bà cắm **hai cái ghim** rồi lấy
phần nằm giữa chúng — nhưng để ý kỹ chỗ bà cắm cái ghim thứ hai:

```text
 ô:   0  1  2  3  4  5  6  7  8  9 10 11 12
      c  à     p  h  ê     s  ữ  a     đ  á
      ↑                                   ↑
      │                                   │
  lấy từ ô này                      dừng ở ô này — chừa lại
```

Ghim thứ nhất cắm vào ô **bắt đầu lấy**. Ghim thứ hai không cắm vào ô cuối cùng
được lấy; nó cắm vào ô **đầu tiên bị bỏ lại** — cái mốc bảo "tới đây thì thôi".

Hai cái ghim ấy viết ra thành code là `ten[0:12]` — hai con số, cách nhau dấu
hai chấm, tất cả nằm trong cặp ngoặc vuông bạn đã quen. Người ta gọi nó là **lát
cắt** — tiếng Anh là *slice*.

Cách đọc hai con số, một lần cho nhớ luôn: **đầu lấy, cuối chừa.**

- Số đầu là chỗ **bắt đầu lấy**: ô mang số đó nằm trong khúc.
- Số sau là chỗ **dừng lại**: ô mang số đó **không** nằm trong khúc.

Chính vì cái ghim thứ hai cắm vào ô đầu tiên bị chừa, có một hệ quả tiện đến mức
nên dùng làm cách kiểm nhanh: **số ký tự lấy được đúng bằng số sau trừ số đầu.**
`[0:12]` cho 12 ký tự. `[4:6]` cho 2 ký tự. Không phải đếm lại lần nào.
::::

::::example{#cat-thu-mot-khuc}
Đem đúng tên khoản ban nãy ra cắt:

```python title=readonly
ten = "cà phê sữa đá"
print(len(ten))
print(ten[0:12])
print(ten)
```

```text title=readonly
13
cà phê sữa đ
cà phê sữa đá
```

Ba dòng, ba chuyện:

- `13` — tên khoản dài 13 ký tự, dài hơn cột đúng một ô.
- `cà phê sữa đ` — khúc cắt ra, đếm lại đúng 12 ký tự, và chữ `á` cuối cùng bị
  chừa lại vì nó nằm ở ô số 12 — đúng cái ô có ghim thứ hai.
- `cà phê sữa đá` — `ten` vẫn nguyên vẹn.

Dòng thứ ba mới là dòng đáng nhớ. Chữ "cắt" nghe như kéo cắt giấy: cắt xong thì
tờ giấy ngắn đi. Ở đây thì không, và bài trước đã nói trước lý do — chuỗi là giá
trị bất biến, không ai sửa được nó, kể cả lát cắt. Cho nên lát cắt **dựng ra một
chuỗi mới** và đưa chuỗi ấy cho bạn; bản gốc nằm im. Muốn giữ khúc vừa cắt thì
phải hứng nó vào một cái tên:

```python title=readonly
ten = "cà phê sữa đá"
ten_cot = ten[0:12]
print(ten_cot)
```

```text title=readonly
cà phê sữa đ
```

Còn một chuyện nữa, và nó khác hẳn thói quen của dấu ngoặc vuông một số:

```python title=readonly
print("bún chả"[0:12])
```

```text title=readonly
bún chả
```

`"bún chả"` chỉ có 7 ô, mà bạn xin tới ô số 12. Xin **một ô** quá cuối dãy thì
máy nổ `IndexError` — bài `Tra từng ký tự một` đã cho bạn thấy. Xin **một khúc**
quá cuối dãy thì máy không nổ: nó đưa những gì còn lại rồi thôi. Với cột sổ thì
đúng là thứ bạn cần — tên nào dài quá thì bị cắt, tên nào ngắn thì giữ nguyên,
một dòng code lo cả hai.
::::

::::predict{#doan-khuc-cat commitOnce}
Byte cắt một khúc từ tên món rồi in ra hai thứ. **Trước khi bấm chạy**, bạn đoán
màn hình hiện ra gì?

```python
mon = "bun cha"
khuc = mon[4:6]
print(khuc)
print(mon)
```

:::opt{correct}
Hai dòng: `ch` rồi `bun cha`
:::

:::opt
Hai dòng: `cha` rồi `bun cha`
::why
Gần đúng ở chỗ bạn đọc `[4:6]` là *từ ô số 4 tới ô số 6* — cách đọc rất tự nhiên,
và nó trúng ở đầu này: ô số 4 đúng là được lấy, chữ `c`.

Chỗ lệch nằm ở đầu kia. Con số thứ hai không chỉ vào một ô được lấy, nó chỉ vào
chỗ **dừng lại**: ô số 6 là ô đầu tiên bị chừa. Phép kiểm nhanh bắt được ngay —
`6 - 4` bằng 2, nên khúc này chỉ có 2 ký tự, mà `cha` thì có 3.
::
:::

:::opt
Hai dòng: `ha` rồi `bun cha`
::why
Gần đúng ở chỗ bạn đếm đúng hai ký tự — bạn đã dùng phép trừ `6 - 4` và ra số 2,
tức là nửa luật "cuối chừa" bạn nắm chắc rồi.

Chỗ lệch nằm ở chỗ khởi hành. Bạn đếm ô đầu tiên của `"bun cha"` là ô số 1, nên
mọi thứ trượt sang phải một nhịp. Python đếm từ 0, y như bài `Đếm từ 0` ở Realm
0 và y như lúc bạn tra `mon[0]`: `b` là ô 0, `u` là ô 1, `n` là ô 2, dấu cách là
ô 3, `c` là ô 4.
::
:::

:::opt
Hai dòng: `ch` rồi `ch`
::why
Gần đúng ở chỗ bạn tính ra khúc cắt chính xác — `ch`, không sai một ký tự nào.

Chỗ lệch nằm ở dòng thứ hai, và nó đến từ nghĩa đen của chữ "cắt": cắt cây mía
thì phần bỏ đi là mất thật. Bài trước vừa cho thấy chuỗi không sửa được một ô
nào, nên lát cắt cũng không có quyền làm `mon` ngắn lại. Nó dựng ra một chuỗi
mới và đưa cho `khuc`; `mon` vẫn đủ bảy ký tự như lúc bạn viết ra.
::
:::
::::

::::explain{#mot-dong-lo-ca-cot}
Gom lại thành ba câu dùng được ngay:

- **Hình dạng**: `chuoi[đầu:hết]`, hai con số cách nhau dấu hai chấm.
- **Luật**: đầu lấy, cuối chừa — nên `hết - đầu` chính là số ký tự lấy được.
- **Kết quả**: luôn là một chuỗi **mới**; bản gốc không suy suyển, và nếu không
  gán vào đâu thì khúc vừa cắt rơi mất.

Và cái giá trị thật của lát cắt so với bài trước không nằm ở chỗ nó ngắn hơn.
Nó nằm ở chỗ **bề rộng cột trở thành một con số bạn sửa được**. Tháng sau bà chủ
kẻ lại sổ rộng 15 chỗ, bạn đổi `12` thành `15`, xong. Với cách nối từng ô thì đó
là viết lại cả dòng.
::::

::::code{#nhet-vua-cot-so}
Sổ tay của Byte có ba tên khoản, dài ngắn khác nhau. Cột "tên khoản" rộng đúng
**12 chỗ**, nên tên nào dài hơn thì phải cắt cho vừa, tên nào ngắn hơn thì để
nguyên.

Điền vào chỗ trống để mỗi vòng xếp đúng một khúc vừa cột vào danh sách `cot`.

```python title=starter
so_tay = ["cà phê sữa đá", "bún chả", "trà tắc"]
cot = []

for ten in so_tay:
    cot.append(___)

for o in cot:
    print(o)
```

```python title=solution
so_tay = ["cà phê sữa đá", "bún chả", "trà tắc"]
cot = []

for ten in so_tay:
    cot.append(ten[0:12])

for o in cot:
    print(o)
```

```python title=test
# Ba tên khoản là ba tình huống khác nhau, cố ý chọn vậy:
#   "cà phê sữa đá" — 13 ký tự, DÀI hơn cột, phải bị cắt còn 12
#   "bún chả"       —  7 ký tự, NGẮN hơn cột, phải giữ nguyên (xin khúc quá
#                      cuối dãy không nổ, chỉ đưa phần còn lại)
#   "trà tắc"       —  7 ký tự, thêm một ca ngắn để cắt sai kiểu `[0:7]` cũng
#                      lộ ra ở tên trên
assert cot == ["cà phê sữa đ", "bún chả", "trà tắc"]
# Lát cắt dựng chuỗi mới; sổ gốc không được suy suyển ký tự nào.
assert so_tay == ["cà phê sữa đá", "bún chả", "trà tắc"]
```

:::hints
- kind: attention
  body: Chỗ trống nằm bên trong `cot.append(...)`, nên thứ bạn điền phải là **một chuỗi** — đúng cái khúc sẽ nằm trong cột. Mỗi vòng, `ten` đang giữ một tên khoản khác nhau, và cột thì lúc nào cũng rộng 12 chỗ.
- kind: strategy
  body: Dùng lát cắt trên `ten`: hai con số trong cặp ngoặc vuông, cách nhau dấu hai chấm. Số đầu là ô bắt đầu lấy — ô đầu tiên của một chuỗi mang số 0. Số sau là chỗ dừng, và nhớ phép kiểm nhanh: hiệu hai số phải bằng đúng bề rộng cột. Đừng lo hai tên ngắn hơn 12 — xin một khúc quá cuối dãy thì máy đưa phần còn lại chứ không nổ.
- kind: one-line
  body: Viết `ten[0:12]` vào chỗ trống.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: cà phê sữa đ
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba tên, một dòng code, cột nào cũng vừa. Đổi 12 thành 15 là xong cả sổ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Tên khoản đã vừa cột. Nhưng còn việc bà chủ nhờ từ mấy bài trước mà chưa ai làm
xong: khách gõ `"Cà Phê"`, sổ đang ghi `"cà phê"`, và bà muốn biết hai dòng ấy
có phải cùng một khoản không.

Thử cắt cho cùng bề rộng xem sao:

```text
"Cà Phê"[0:12]  →  "Cà Phê"
"cà phê"[0:12]  →  "cà phê"
```

Vẫn `False`. Lát cắt không đụng tới chuyện hoa hay thường — nó chỉ chọn lấy ô
nào, chứ không đổi thứ nằm trong ô.

Mà đây không phải chuyện hiếm gặp: mười khách gõ tên một món thì ra tám kiểu
viết hoa. Ngồi đối chiếu từng kiểu là việc không có hồi kết.

Cái bạn cần là kéo **cả hai** chuỗi về cùng một dạng trước khi đem so — chẳng
hạn ép tất cả xuống chữ thường. Nhưng ép kiểu gì? Bạn không thể sửa từng ô (bài
trước đã chặn), và lát cắt thì không đổi được ký tự nào.

Việc này phải nhờ tới một thứ đi sẵn kèm mỗi chuỗi. Nó là gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
