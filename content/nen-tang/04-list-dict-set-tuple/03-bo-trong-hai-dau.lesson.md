---
id: nen-tang.list-dict-set-tuple.bo-trong-hai-dau
title: Bỏ trống một đầu
summary: Chỗ trống trước dấu hai chấm nghĩa là "từ đầu sổ", chỗ trống sau nó nghĩa là "tới hết sổ" — nên `so[-3:]` là ba khoản gần nhất, viết một lần đúng mãi.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.slice-open-end]
requires: [core.list-slice, core.list-negative-index, core.negative-index, core.list, core.list-index, core.len, core.accumulator, ctrl.for-each, core.function-def, core.function-parameter, core.function-return, core.fstring]
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

::::byte{trigger=enter mood=curious pose=lean-in}
Chỗ nào mình đoán được thì bạn khỏi phải nói. Cứ để trống ở đó.
::::

::::explain{#cho-trong-cung-la-mot-cau-tra-loi}
Bài trước để bạn lại với dòng này:

```python
so[19:len(so)]
```

Nó chạy. Nhưng đọc lên thì nó nói một câu vòng vo: *từ chỗ 19 tới chỗ bằng độ
dài cuốn sổ*. Điều Byte muốn nói ngắn hơn nhiều: *từ chỗ 19 tới hết*.

Và cái giá của câu vòng vo thì đúng cái giá bài 1 đã kể: tên cuốn sổ phải gõ
hai lần trên một dòng. Ngày nào đó Byte đổi tên biến, sửa chỗ này quên sửa chỗ
kia — câu lệnh lấy khúc của cuốn sổ **này** theo độ dài của cuốn sổ **kia**, ra
một khúc sai mà không một tiếng báo lỗi.

Python có một cách nói thẳng cho đúng ý ấy: **bỏ trống một đầu**.

```python
so[19:]
```

Dấu hai chấm vẫn còn, nên máy vẫn biết đây là một lát cắt. Chỉ con số sau nó là
không có. Chỗ trống ấy không phải chỗ bạn quên điền — nó là một câu trả lời, và
câu trả lời là: **tới hết sổ**.

Cái ghim thứ hai không cắm vào một chỗ cụ thể nào cả; nó cắm vào cuối sổ, dù
cuối sổ hôm nay ở đâu và tối mai dời tới đâu.

Đầu bên kia cũng vậy, theo đúng phép đối xứng:

```python
so[:3]
```

Chỗ trống trước dấu hai chấm nghĩa là **từ đầu sổ**. Nên `so[0:3]` của bài
trước viết gọn lại thành `so[:3]`, và số `0` biến mất vì nó chẳng nói thêm được
gì — bắt đầu từ đầu là chuyện máy tự hiểu.

Bây giờ tới chỗ đáng giá nhất của bài này. Ghim thứ nhất vẫn nhận số âm, y như
bài 1:

```python
so[-3:]
```

Đọc từng mảnh: ghim thứ nhất cắm ở chỗ `-3`, tức chỗ thứ ba đếm ngược từ cuối;
ghim thứ hai bỏ trống, tức tới hết sổ. Ghép lại: **ba khoản gần nhất**.

Câu ấy đúng cho cuốn sổ hôm nay, đúng cho cuốn sổ tối mai dài thêm một khoản,
và đúng cho cuốn sổ tháng sau chưa ai mở ra. Bạn viết nó một lần.

Còn bỏ trống **cả hai** đầu thì sao? `so[:]` — từ đầu sổ tới hết sổ, tức là cả
cuốn. Ngay lúc này nó trông như một cách viết vòng vo của `so`, và đúng là in
ra thì hai câu cho ra y hệt nhau. Cứ để câu hỏi ấy đó; nó sẽ được trả lời.
::::

::::example{#bon-cach-cat-mot-cuon-so}
Sổ tháng Ba của Byte, giờ đã bảy khoản:

```python title=readonly
so_thang_ba = [25000, 40000, 15000, 60000, 30000, 12000, 55000]

print(so_thang_ba[:3])
print(so_thang_ba[3:])
print(so_thang_ba[-3:])
print(so_thang_ba[:])
```

Màn hình hiện ra:

```text title=readonly
[25000, 40000, 15000]
[60000, 30000, 12000, 55000]
[30000, 12000, 55000]
[25000, 40000, 15000, 60000, 30000, 12000, 55000]
```

Đọc bốn dòng cạnh nhau:

- `[:3]` — ba khoản đầu sổ. Đây là `[0:3]` của bài trước, viết bớt một con số.
- `[3:]` — bỏ ba khoản đầu, lấy tất cả phần còn lại. Hai dòng đầu ghép lại vừa
  đúng cả cuốn sổ, không thừa không thiếu khoản nào: đó là hệ quả trực tiếp của
  luật đầu lấy cuối chừa.
- `[-3:]` — ba khoản cuối sổ. Để ý nó khác dòng ngay trên: `[3:]` neo vào chỗ
  số 3 đếm xuôi nên cho bốn khoản, `[-3:]` neo vào cái đuôi nên luôn cho ba
  khoản. Sổ càng dài thêm thì hai khúc này càng lệch nhau.
- `[:]` — cả cuốn.

Và đây là chỗ chiều đếm ngược trả công. Byte ghi thêm khoản của tối nay:

```python title=readonly
so_thang_ba = [25000, 40000, 15000, 60000, 30000, 12000, 55000]
so_thang_ba.append(18000)

print(so_thang_ba[-3:])
print(so_thang_ba[4:])
```

```text title=readonly
[12000, 55000, 18000]
[30000, 12000, 55000, 18000]
```

`[-3:]` vẫn đưa ra đúng ba khoản gần nhất, và ba khoản ấy đã đổi theo cuốn sổ.
Còn `[4:]` thì mỗi tối lại dài thêm một khoản, vì nó neo vào một chỗ đứng đếm
xuôi mà cuốn sổ thì chỉ dài về phía sau.

> Chỗ dễ vấp: `so[-3:]` và `so[:-3]` trông giống nhau tới mức khó phân biệt
> bằng mắt, mà chúng nói hai chuyện khác hẳn. Ở `so[-3:]` số âm là ghim **bắt
> đầu**, nên bạn nhận ba khoản cuối. Ở `so[:-3]` số âm là ghim **dừng lại**, nên
> bạn nhận mọi khoản trừ ba khoản cuối — trên cuốn sổ bảy khoản bên trên thì đó
> là bốn khoản đầu. Cả hai đều chạy, không câu nào báo lỗi, nên đây là loại sai
> máy không nhắc: bạn phải tự đọc xem dấu hai chấm nằm bên nào của con số.
::::

::::predict{#doan-hai-khuc-cuoi commitOnce}
Byte lấy lại cuốn sổ tháng Tư — năm khoản — rồi cắt hai khúc, cả hai đều bỏ
trống một đầu. Nhưng lần này một khúc bỏ trống đầu **sau**, một khúc bỏ trống
đầu **trước**, và con số của khúc thứ hai là số âm.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra hai dòng nào?

```python title=readonly
so_thang_tu = [80000, 20000, 35000, 90000, 45000]

print(so_thang_tu[2:])
print(so_thang_tu[:-2])
```

:::opt{correct}
[35000, 90000, 45000] rồi [80000, 20000, 35000]
:::

:::opt
[35000, 90000, 45000] rồi [90000, 45000]
::why
Gần đúng ở chỗ dòng đầu: bạn đọc `[2:]` chuẩn xác — bắt đầu ở chỗ số 2, chạy
tới hết sổ, ba khoản.

Chỗ lệch nằm ở việc dấu hai chấm đứng bên nào của số `-2`. Trong `so[:-2]` thì
`-2` đứng **sau** dấu hai chấm, nên nó là chỗ **dừng**: cắt từ đầu sổ tới
trước khoản thứ hai đếm ngược. Hai khoản cuối mà bạn chọn chính là kết quả của
`so[-2:]` — câu ấy có thật, chỉ là nó không phải câu đang chạy ở đây.
::
:::

:::opt
[80000, 20000] rồi [80000, 20000, 35000]
::why
Gần đúng ở chỗ dòng thứ hai: `[:-2]` cho đúng ba khoản đầu sổ, bạn đọc chiều
đếm ngược không sai.

Chỗ lệch nằm ở dòng đầu. Trong `so[2:]` con số đứng **trước** dấu hai chấm, nên
nó là ghim bắt đầu chứ không phải chỗ dừng — cắt từ chỗ số 2 trở đi. Hai khoản
đầu mà bạn chọn là kết quả của `so[:2]`, tức là cùng con số ấy nhưng đặt sang
bên kia dấu hai chấm.
::
:::

:::opt
Máy báo `SyntaxError`, vì trong ngoặc vuông thiếu mất một con số
::why
Gần đúng ở chỗ bạn cảnh giác đúng loại: một câu lệnh viết thiếu thì máy hay từ
chối ngay lúc đọc, và bạn đã gặp `SyntaxError` từ Realm 0.

Chỗ lệch: ở đây không có gì thiếu. Dấu hai chấm vẫn nằm nguyên trong ngoặc
vuông, nên máy vẫn nhận ra đây là một lát cắt; chỗ trống bên cạnh nó là một cách
viết hợp lệ, mang sẵn nghĩa "từ đầu sổ" hoặc "tới hết sổ" tuỳ nó nằm bên nào.
Máy chỉ báo `SyntaxError` khi nó không đọc nổi câu bạn viết.
::
:::
::::

::::explain{#mot-lan-viet-dung-mai}
Gom lại thành ba câu:

- Chỗ trống **trước** dấu hai chấm nghĩa là *từ đầu sổ*.
- Chỗ trống **sau** dấu hai chấm nghĩa là *tới hết sổ*.
- Bỏ trống cả hai thì được cả cuốn: `so[:]`.

Cái được lớn nhất không phải bớt được vài ký tự. Nó là chuyện câu lệnh thôi
không phụ thuộc vào độ dài cuốn sổ nữa. `so[19:len(so)]` phải hỏi sổ dài bao
nhiêu, và câu trả lời đổi mỗi tối; `so[19:]` không hỏi gì cả, nên nó không có gì
để mà sai.

Đó là cùng một món lợi mà `[-1]` đem lại ở bài 1, lần này áp lên cả một khúc
thay vì một khoản: **viết ra điều mình muốn, đừng viết ra cách tính điều mình
muốn.**
::::

::::code{#hai-khuc-cho-ban-bao-cao}
Bản báo cáo của Byte cần thêm hai dòng, và mỗi dòng là một hàm.

- `ba_khoan_gan_nhat` — ba khoản ghi gần đây nhất, tức ba khoản nằm cuối sổ.
- `tu_khoan_thu_tu_tro_di` — từ khoản thứ tư tới hết sổ. Khoản thứ tư nằm ở chỗ
  số 3, vì chỗ đầu tiên mang số 0.

Cả hai hàm phải đúng cho **mọi** cuốn sổ, dài ngắn thế nào cũng vậy — nên không
hàm nào được phép hỏi sổ dài bao nhiêu.

Bài chấm trên ba cuốn sổ có độ dài khác nhau, trong đó có một cuốn ngắn hơn cả
khúc được xin.

```python title=starter
def cong_tien(khuc):
    """Nhận một danh sách các khoản, trả về tổng của chúng."""
    tong = 0
    for tien in khuc:
        tong = tong + tien
    return tong


def ba_khoan_gan_nhat(so):
    """Nhận cuốn sổ, trả về DANH SÁCH ba khoản nằm cuối sổ.

    Sổ chưa đủ ba khoản thì trả về những khoản đang có.
    """
    return ___


def tu_khoan_thu_tu_tro_di(so):
    """Nhận cuốn sổ, trả về DANH SÁCH các khoản từ khoản thứ tư tới hết sổ.

    Sổ chưa có khoản thứ tư thì trả về một danh sách rỗng.
    """
    return ___


so_thang_ba = [25000, 40000, 15000, 60000, 30000, 12000, 55000]

print(f"Ba khoản gần nhất cộng lại: {cong_tien(ba_khoan_gan_nhat(so_thang_ba))} đồng")
print(f"Từ khoản thứ tư trở đi cộng lại: {cong_tien(tu_khoan_thu_tu_tro_di(so_thang_ba))} đồng")
```

```python title=solution
def cong_tien(khuc):
    """Nhận một danh sách các khoản, trả về tổng của chúng."""
    tong = 0
    for tien in khuc:
        tong = tong + tien
    return tong


def ba_khoan_gan_nhat(so):
    """Nhận cuốn sổ, trả về DANH SÁCH ba khoản nằm cuối sổ.

    Sổ chưa đủ ba khoản thì trả về những khoản đang có.
    """
    return so[-3:]


def tu_khoan_thu_tu_tro_di(so):
    """Nhận cuốn sổ, trả về DANH SÁCH các khoản từ khoản thứ tư tới hết sổ.

    Sổ chưa có khoản thứ tư thì trả về một danh sách rỗng.
    """
    return so[3:]


so_thang_ba = [25000, 40000, 15000, 60000, 30000, 12000, 55000]

print(f"Ba khoản gần nhất cộng lại: {cong_tien(ba_khoan_gan_nhat(so_thang_ba))} đồng")
print(f"Từ khoản thứ tư trở đi cộng lại: {cong_tien(tu_khoan_thu_tu_tro_di(so_thang_ba))} đồng")
```

```python title=test
# Ba phép kiểm cho chỗ trống thứ nhất, ba phép cho chỗ trống thứ hai.
#
# Vì sao mỗi chỗ cần tới ba cuốn sổ:
#   `so[4:]` điền nhầm vào chỗ thứ nhất  → trúng cuốn bảy khoản, trượt cuốn năm
#                                          khoản, nên một mình cuốn đầu không
#                                          nói thật được
#   `so[:3]` điền nhầm vào chỗ thứ hai   → trượt ngay phép kiểm thứ tư
#   `so[3:6]` điền vào chỗ thứ hai       → trúng cuốn năm khoản, trượt cuốn bảy
#                                          khoản vì bỏ rơi khoản 55 nghìn
#   `True` hay một danh sách chép cứng   → trượt ngay phép kiểm đầu của mỗi chỗ
assert ba_khoan_gan_nhat([25000, 40000, 15000, 60000, 30000, 12000, 55000]) == [30000, 12000, 55000], "cuốn sổ bảy khoản này kết bằng 30, 12 và 55 nghìn, nên ba khoản gần nhất của nó là ba khoản ấy"
assert ba_khoan_gan_nhat([80000, 20000, 35000, 90000, 45000]) == [35000, 90000, 45000], "cuốn sổ năm khoản này kết bằng 35, 90 và 45 nghìn — một chỗ đứng đếm xuôi cố định không thể vừa trúng đuôi sổ bảy khoản vừa trúng đuôi sổ năm khoản"
assert ba_khoan_gan_nhat([70000, 90000]) == [70000, 90000], "cuốn sổ này mới có hai khoản, nên xin ba khoản cuối chỉ nhận được đúng hai khoản đang có"
assert tu_khoan_thu_tu_tro_di([25000, 40000, 15000, 60000, 30000, 12000, 55000]) == [60000, 30000, 12000, 55000], "khoản thứ tư của cuốn sổ này là khoản 60 nghìn, và khúc phải chạy tới hết bảy khoản chứ không dừng lại giữa sổ"
assert tu_khoan_thu_tu_tro_di([80000, 20000, 35000, 90000, 45000]) == [90000, 45000], "cuốn sổ năm khoản này chỉ còn 90 và 45 nghìn kể từ khoản thứ tư"
assert tu_khoan_thu_tu_tro_di([25000, 40000, 15000]) == [], "cuốn sổ này mới có ba khoản nên chưa có khoản thứ tư nào — khúc ấy rỗng, và một danh sách rỗng vẫn là câu trả lời đàng hoàng"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều nằm sau `return`, và cả hai kết quả đều được đưa thẳng vào `cong_tien` — nên cả hai phải là danh sách. Đọc lại docstring của từng hàm rồi hỏi mình cái ghim nào cắm vào chỗ cụ thể, cái ghim nào cắm vào cuối sổ.
- kind: strategy
  body: Cả hai hàm đều chạy tới hết sổ, nên cả hai đều bỏ trống cái ghim thứ hai. Khác nhau ở cái ghim thứ nhất. Hàm thứ hai neo vào một chỗ đếm xuôi, và khoản thứ tư nằm ở chỗ số 3. Hàm thứ nhất phải bám theo cái đuôi của cuốn sổ dù sổ dài bao nhiêu, nên chỗ đứng của nó đếm theo chiều ngược của bài 1.
- kind: one-line
  body: Chỗ trống thứ nhất là `so[-3:]`, chỗ trống thứ hai là `so[3:]`.
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^Ba khoản gần nhất cộng lại: 97000 đồng\nTừ khoản thứ tư trở đi cộng lại: 157000 đồng\s*$
- tier: output
  expect: Từ khoản thứ tư trở đi cộng lại: 157000 đồng
- tier: static
  onFail: hai khúc phải đọc RA TỪ cuốn sổ được đưa vào, và không hàm nào được hỏi sổ dài bao nhiêu — bỏ `len` đi, để trống cái ghim thứ hai
  requireAst:
  # `min: 2` vì có hai hàm, mỗi hàm phải đọc tên `so` đúng một lần. Hỏi
  # `min: 1` thì điền đúng một chỗ và chép cứng chỗ kia cũng thoả.
  - kind: uses-name, target: so, min: 2
  forbidAst:
  - kind: uses-call, target: len
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba khoản gần nhất. Tối mai sổ dài thêm, câu ấy vẫn đúng, mình không sửa gì.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa nhặt thêm `so[:]` — cả cuốn sổ, không chừa khoản nào, viết bằng đúng
một cặp ngoặc vuông và một dấu hai chấm. Bài này chưa nói nó dùng để làm gì, và
nó sẽ nằm đó chờ.

Trong lúc chờ, có một chuyện về cuốn sổ mà bạn nên nhìn lại.

Byte viết một hàm dọn sổ:

```python
def don_so(so):
    """Ghi một dòng đánh dấu vào cuối sổ."""
    so.append(0)
```

Bạn đưa **cả cuốn sổ** cho hàm `don_so(so)`; trong hàm nó `.append` thêm một
dòng. Ra ngoài, cuốn sổ của **bạn** có dòng đó không?

Mạch hàm đã trả lời cho riêng ca này rồi, ở bài *Đối số không phải bản sao*: có.
Bạn không phải đoán lại.

Nhưng bài ấy chưa nói **vì sao** chuyện đó xảy ra — nó chỉ cho bạn xem kết quả.
Và nó cũng chưa nói chuyện gì xảy ra khi **không có hàm nào cả**, khi bạn chỉ
viết đúng một dòng gán trần:

```python
so_cu = so
so.append(0)
```

`so_cu` bây giờ có dòng đánh dấu ấy không? Và câu trả lời cho dòng gán này với
câu trả lời cho lời gọi hàm bên trên — là một chuyện, hay hai chuyện khác nhau?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
