---
id: nen-tang.list-dict-set-tuple.cat-mot-khuc-so
title: Cắt một khúc sổ
summary: Hai con số cách nhau dấu hai chấm xin về nguyên một khúc sổ — đầu lấy, cuối chừa, và khúc đi ra là một danh sách đem dùng tiếp được ngay.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.list-slice]
requires: [core.string-slice, core.list-negative-index, core.list, core.list-index, core.list-append, core.len, core.accumulator, ctrl.for-each, core.function-def, core.function-parameter, core.function-return, core.fstring]
concepts: [core.danh-sach, core.chi-so]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Xin từng khoản một thì mỏi tay. Nói cho mình hai đầu, mình đưa cả khúc.
::::

::::explain{#hai-cai-ghim-tren-cuon-so}
Bài trước kết bằng một dòng phải viết lại từ đầu mỗi lần sếp của Byte đổi ý:

```python
ba_khoan = [so[0], so[1], so[2]]
```

Nhìn kỹ dòng này thì nó có hai chỗ khó chịu, và chỗ thứ hai nặng hơn chỗ thứ
nhất.

Chỗ thứ nhất: số lượng bị đóng cứng vào hình dạng câu lệnh. Muốn mười khoản thì
không phải sửa một con số, mà phải gõ lại cả dòng với mười cặp ngoặc vuông.

Chỗ thứ hai: cuốn sổ mới có hai khoản thì `so[2]` nổ `IndexError` và chương
trình chết ngay giữa lúc làm báo cáo — trong khi điều Byte muốn ở ca ấy chỉ là
"có bao nhiêu thì đưa bấy nhiêu".

Bạn đã có sẵn cách nói ngắn cho đúng việc này, và bạn học nó trên chuỗi. Ở bài
*Cắt một khúc chuỗi*, bà chủ cắm **hai cái ghim** rồi lấy phần nằm giữa:

```text
 chỗ:    0      1      2      3      4      5
      25000  40000  15000  60000  30000  12000
        ↑                   ↑
        │                   │
   lấy từ chỗ này      dừng ở chỗ này — chừa lại
```

Ghim thứ nhất cắm vào chỗ **bắt đầu lấy**. Ghim thứ hai không cắm vào khoản
cuối cùng được lấy; nó cắm vào khoản **đầu tiên bị bỏ lại**. Viết ra thành code
thì đó là `so[0:3]` — hai con số, cách nhau dấu hai chấm, nằm trong cặp ngoặc
vuông bạn đã quen. Người ta gọi nó là **lát cắt**.

Luật đọc hai con số vẫn nguyên như lúc bạn gặp trên chuỗi: **đầu lấy, cuối
chừa.** Và hệ quả tiện của nó cũng nguyên: **số khoản lấy được đúng bằng số sau
trừ số đầu**, không phải đếm lại lần nào.

Vậy cái mới của bài này nằm ở đâu, nếu ký hiệu thì y hệt?

Nó nằm ở **thứ đi ra**. Cắt một chuỗi thì bạn nhận về một chuỗi. Cắt một cuốn
sổ thì bạn nhận về một **danh sách** — một cuốn sổ nhỏ hơn, đủ tư cách đem đi
làm mọi việc mà một danh sách làm được:

- `len` được, để biết khúc ấy có mấy khoản;
- `for` được, để duyệt từng khoản trong khúc;
- đưa thẳng vào một hàm nhận danh sách, ví dụ hàm cộng tiền bạn đã viết ở mạch
  hàm — không phải mở gói ra rồi gói lại.

Đó là chỗ lát cắt hơn hẳn ba dòng ngoặc vuông ghép tay: nó không đưa bạn ba con
số rời, nó đưa bạn **một** thứ dùng tiếp được ngay.
::::

::::example{#cat-thu-mot-khuc}
Sổ tháng Ba của Byte, sáu khoản:

```python title=readonly
so_thang_ba = [25000, 40000, 15000, 60000, 30000, 12000]

ba_khoan_dau = so_thang_ba[0:3]

print(ba_khoan_dau)
print(len(ba_khoan_dau))
print(so_thang_ba)
```

Màn hình hiện ra:

```text title=readonly
[25000, 40000, 15000]
3
[25000, 40000, 15000, 60000, 30000, 12000]
```

Ba dòng, ba chuyện:

- Dòng đầu — khúc cắt ra hiện lên với cặp ngoặc vuông và những dấu phẩy quen
  thuộc. Nó là một danh sách, không phải một con số, cũng không phải một chuỗi.
- Dòng hai — `len` chạy được trên nó, và cho ra 3, đúng bằng số sau trừ số đầu.
  Khoản 60 nghìn ở chỗ số 3 bị chừa lại, vì chỗ số 3 chính là chỗ cắm ghim thứ
  hai.
- Dòng ba — cuốn sổ gốc vẫn đủ sáu khoản như lúc Byte ghi vào. Chữ "cắt" nghe
  như kéo cắt giấy, cắt xong thì tờ giấy ngắn đi; ở đây thì không. Khúc được
  đưa cho bạn hứng lấy, còn cuốn sổ nằm im.

Vì khúc ấy là một danh sách, nó đi thẳng vào hàm cộng tiền được:

```python title=readonly
def cong_tien(khuc):
    """Nhận một danh sách các khoản, trả về tổng của chúng."""
    tong = 0
    for tien in khuc:
        tong = tong + tien
    return tong


so_thang_ba = [25000, 40000, 15000, 60000, 30000, 12000]
print(cong_tien(so_thang_ba[0:3]))
```

```text title=readonly
80000
```

Còn một chuyện nữa, và nó khác hẳn thói quen của cặp ngoặc vuông một số:

```python title=readonly
so_moi = [70000, 90000]
print(so_moi[0:3])
```

```text title=readonly
[70000, 90000]
```

Cuốn sổ này mới có hai khoản, mà bạn xin tới chỗ số 3. Xin **một chỗ** quá cuối
sổ thì máy nổ `IndexError` — bài trước vừa nhắc lại cái bờ ấy. Xin **một khúc**
quá cuối sổ thì máy không nổ: nó đưa những gì đang có rồi thôi. Với báo cáo của
Byte thì đó đúng là thứ cần — sổ đủ ba khoản thì đưa ba, sổ mới có hai thì đưa
hai, một dòng code lo cả hai ca.
::::

::::predict{#doan-khuc-cat-tu-so commitOnce}
Byte cắt một khúc ở giữa sổ tháng Ba rồi in ra hai thứ: khúc vừa cắt, và cuốn
sổ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra hai dòng nào?

```python title=readonly
so_thang_ba = [25000, 40000, 15000, 60000, 30000, 12000]

khuc = so_thang_ba[1:3]

print(khuc)
print(so_thang_ba)
```

:::opt{correct}
[40000, 15000] rồi cả sáu khoản của sổ tháng Ba
:::

:::opt
[40000, 15000, 60000] rồi cả sáu khoản của sổ tháng Ba
::why
Gần đúng ở chỗ bạn khởi hành chính xác: chỗ số 1 là khoản 40 nghìn, và khoản ấy
đúng là nằm trong khúc. Nửa đầu của lát cắt bạn đọc không sai.

Chỗ lệch nằm ở đầu kia. Con số thứ hai không chỉ vào khoản cuối cùng được lấy,
nó chỉ vào chỗ **dừng lại** — khoản đầu tiên bị chừa. Phép kiểm nhanh bắt được
ngay: số sau trừ số đầu ra 2, nên khúc này có đúng hai khoản, mà đáp án bạn
chọn có ba.
::
:::

:::opt
[25000, 40000] rồi cả sáu khoản của sổ tháng Ba
::why
Gần đúng ở chỗ khó nhất: bạn đếm ra đúng hai khoản, tức là luật "cuối chừa" bạn
đã nắm chắc.

Chỗ lệch nằm ở chỗ khởi hành. Bạn đang đếm khoản đầu tiên của sổ là chỗ số 1,
nên cả khúc trượt sang trái một nhịp. Python đếm từ 0, y như hồi Realm 0 dạy
`thuc_don[0]` là món đầu tiên: 25 nghìn ở chỗ 0, 40 nghìn ở chỗ 1, 15 nghìn ở
chỗ 2.
::
:::

:::opt
[40000, 15000] rồi [25000, 60000, 30000, 12000]
::why
Gần đúng ở chỗ bạn tính ra khúc cắt chính xác — hai khoản 40 nghìn và 15 nghìn,
không sai chỗ nào.

Chỗ lệch nằm ở dòng thứ hai, và nó đến từ nghĩa đen của chữ "cắt": cắt cây mía
thì phần bỏ đi mất thật. Ở đây lát cắt không lấy đi khoản nào của cuốn sổ — nó
đọc những khoản nằm giữa hai cái ghim rồi đưa khúc ấy cho bạn, và cuốn sổ vẫn
đủ sáu khoản như lúc Byte ghi vào. Muốn sổ ngắn đi thì phải có một câu lệnh nói
thẳng ra điều đó, và trong đoạn trên không có câu nào như vậy.
::
:::
::::

::::explain{#gom-lai-ba-cau}
Ba câu dùng được ngay:

- **Hình dạng** — `so[đầu:hết]`, hai con số cách nhau dấu hai chấm.
- **Luật** — đầu lấy, cuối chừa; số khoản lấy được đúng bằng số sau trừ số đầu.
  Xin quá cuối sổ thì máy đưa phần đang có chứ không nổ.
- **Thứ đi ra** — một danh sách, dùng tiếp được ngay; cuốn sổ gốc vẫn đủ khoản
  như cũ.

Và giá trị thật của lát cắt so với ba cặp ngoặc vuông ghép tay không nằm ở chỗ
nó ngắn hơn. Nó nằm ở chỗ **số khoản cần lấy trở thành một con số bạn sửa
được**. Sếp của Byte đổi ý muốn mười khoản đầu tháng, bạn đổi `3` thành `10`,
xong. Với cách ghép tay thì đó là viết lại cả dòng.
::::

::::code{#ba-khoan-dau-thang}
Báo cáo đầu tháng của Byte cần hai con số: **ba khoản đầu tháng** là những
khoản nào, và cộng lại hết bao nhiêu.

Viết `ba_khoan_dau` sao cho nó đưa ra một **danh sách**, để hàm `cong_tien` bên
trên nhận được ngay mà không phải mở gói.

Bài chấm trên ba cuốn sổ: một cuốn sáu khoản, một cuốn bốn khoản, và một cuốn
mới có hai khoản — cuốn thứ ba là cuốn nói thật nhất, vì nó là ca "sổ chưa đủ
ba khoản" mà docstring đã hứa.

```python title=starter
def cong_tien(khuc):
    """Nhận một danh sách các khoản, trả về tổng của chúng."""
    tong = 0
    for tien in khuc:
        tong = tong + tien
    return tong


def ba_khoan_dau(so):
    """Nhận cuốn sổ, trả về DANH SÁCH ba khoản đầu sổ.

    Sổ chưa đủ ba khoản thì trả về những khoản đang có.
    """
    return ___


so_thang_ba = [25000, 40000, 15000, 60000, 30000, 12000]
so_thang_tu = [80000, 20000, 35000, 90000]

print(f"Tháng Ba — ba khoản đầu cộng lại: {cong_tien(ba_khoan_dau(so_thang_ba))} đồng")
print(f"Tháng Tư — ba khoản đầu cộng lại: {cong_tien(ba_khoan_dau(so_thang_tu))} đồng")
```

```python title=solution
def cong_tien(khuc):
    """Nhận một danh sách các khoản, trả về tổng của chúng."""
    tong = 0
    for tien in khuc:
        tong = tong + tien
    return tong


def ba_khoan_dau(so):
    """Nhận cuốn sổ, trả về DANH SÁCH ba khoản đầu sổ.

    Sổ chưa đủ ba khoản thì trả về những khoản đang có.
    """
    return so[0:3]


so_thang_ba = [25000, 40000, 15000, 60000, 30000, 12000]
so_thang_tu = [80000, 20000, 35000, 90000]

print(f"Tháng Ba — ba khoản đầu cộng lại: {cong_tien(ba_khoan_dau(so_thang_ba))} đồng")
print(f"Tháng Tư — ba khoản đầu cộng lại: {cong_tien(ba_khoan_dau(so_thang_tu))} đồng")
```

```python title=test
# Năm phép kiểm, mỗi phép chặn một kiểu điền sai khác nhau:
#   một hằng số bất kỳ         → trượt ngay phép kiểm đầu
#   `so[0]`                    → cho một con số, không phải danh sách
#   `so[0:4]` hay `so[1:3]`    → trượt phép kiểm đầu vì lấy nhầm khoản
#   `[so[0], so[1], so[2]]`    → đúng cho hai cuốn đầu, nhưng nổ IndexError
#                                trên cuốn hai khoản, đúng ca mà docstring hứa
#   một danh sách chép cứng    → trượt ở cuốn sổ tháng Tư
assert ba_khoan_dau([25000, 40000, 15000, 60000, 30000, 12000]) == [25000, 40000, 15000], "ba khoản ĐẦU sổ tháng Ba là 25, 40 và 15 nghìn — khúc cắt phải dừng lại trước khoản thứ tư"
assert ba_khoan_dau([80000, 20000, 35000, 90000]) == [80000, 20000, 35000], "sổ tháng Tư dài bốn khoản, và ba khoản đầu của nó không có khoản 90 nghìn đứng cuối"
assert ba_khoan_dau([70000, 90000]) == [70000, 90000], "cuốn sổ này mới có hai khoản, nên ba khoản đầu của nó là đúng hai khoản đang có — xin một khúc quá cuối sổ thì máy đưa phần đang có chứ không nổ lỗi"
assert len(ba_khoan_dau([25000, 40000, 15000, 60000, 30000, 12000])) == 3, "thứ đi ra phải là một DANH SÁCH ba khoản để `cong_tien` duyệt được, không phải một con số lẻ"
assert cong_tien(ba_khoan_dau([25000, 40000, 15000, 60000, 30000, 12000])) == 80000, "ba khoản đầu tháng Ba cộng lại đúng 80 nghìn"
```

:::hints
- kind: attention
  body: Chỗ trống nằm sau `return`, và dòng in bên dưới đưa thẳng kết quả ấy vào `cong_tien` — mà `cong_tien` chạy một vòng `for` trên thứ nó nhận được. Vậy thứ bạn điền phải là một danh sách, không phải một khoản lẻ. Cái tên đang giữ cuốn sổ ở đây là tham số `so`.
- kind: strategy
  body: Dùng lát cắt trên `so` — hai con số trong cặp ngoặc vuông, cách nhau dấu hai chấm. Số đầu là chỗ bắt đầu lấy, và khoản đầu tiên của một cuốn sổ mang số 0. Số sau là chỗ dừng, nhớ phép kiểm nhanh là hiệu hai số phải bằng đúng số khoản cần lấy. Cuốn sổ hai khoản thì đừng lo — xin một khúc quá cuối sổ không nổ lỗi.
- kind: one-line
  body: Viết `so[0:3]` vào chỗ trống.
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^Tháng Ba — ba khoản đầu cộng lại: 80000 đồng\nTháng Tư — ba khoản đầu cộng lại: 135000 đồng\s*$
- tier: static
  onFail: khúc cắt phải đọc RA TỪ cuốn sổ được đưa vào, không chép cứng mấy con số vào thân hàm
  requireAst:
  - kind: uses-name, target: so, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một khúc sổ đi ra vẫn là một cuốn sổ. Mình đưa thẳng nó cho hàm cộng tiền.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đầu tháng thì xong rồi. Byte lật sang phần cuối bản báo cáo: **từ khoản thứ 20
tới hết sổ** — phần chi tiêu nửa cuối tháng.

Khoản thứ 20 nằm ở chỗ số 19, vì chỗ đầu tiên mang số 0. Vậy ghim thứ nhất cắm
ở 19. Còn ghim thứ hai cắm ở đâu?

Ở "hết sổ". Mà "hết sổ" thì mỗi ngày một khác: hôm nay sổ 26 khoản nên ghim
cắm ở 26, tối mai Byte ghi thêm một khoản thì phải sửa thành 27, và tối kia lại
sửa nữa.

Đây đúng là chỗ đau mà bài trước vừa chữa cho đầu bên kia của cuốn sổ, giờ mọc
lại ở đầu này. Cách duy nhất bạn đang có là hỏi sổ dài bao nhiêu rồi thay con
số ấy vào:

```python
so[19:len(so)]
```

Phải gọi `len(so)` rồi thay vào chỗ số cuối à?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
