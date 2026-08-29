---
id: nen-tang.gia-tri-bien-kieu.doi-chu-co-phan-le-thanh-so
title: Chữ có phần lẻ thì đổi bằng gì
summary: "`int()` đặt lên một chuỗi có dấu chấm thì nổ `ValueError`; muốn đọc con số ấy phải gọi `float()`."
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.float-cast]
requires: [core.don-vi-nho-nhat, core.int-cast, core.value-error, core.float, core.input-returns-str, core.arithmetic, core.type-fn, core.function-def, core.function-parameter, core.function-return, core.function-call]
concepts: [core.kieu-gia-tri, core.loi-khi-chay]
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

::::byte{trigger=enter mood=thinking pose=idle}
Mình đọc được con số bạn viết trong chuỗi. Nhưng mình không tự bỏ bớt chữ nào.
::::

::::explain{#hai-viec-cua-mot-cai-ten}
Bài trước để lại một câu hỏi: người ghi sổ quen ghi bằng nghìn nên gõ `25.5`,
và bạn viết `int("25.5")`. Máy làm gì?

Trước khi trả lời, để ý là bạn đã dùng `int()` cho **hai việc khác hẳn nhau**:

- `int(25.5)` — trong ngoặc là một **số**. Máy cắt phần lẻ, ra `25`. Đó là bài 2.
- `int("57")` — trong ngoặc là một **chuỗi**. Máy đọc mấy ký tự đó ra thành một
  con số, ra `57`. Đó là Realm 0, bài đổi chữ thành số.

Cùng một cái tên `int`, hai việc khác nhau — y như dấu `+` vừa cộng số vừa nối
chữ ở Realm 0. Và việc thứ hai khắt khe hơn việc thứ nhất rất nhiều.

Lý do nằm ở chỗ nó là việc **đọc lại thứ người khác gõ**. Khi bạn tự tay viết
`int(25.5)`, con số 25.5 đã nằm sẵn trong máy, và bạn nói rõ ý mình: bỏ phần
lẻ đi. Còn `int("25.5")` là chuyện khác: một người ngồi trước màn hình gõ vào
bốn ký tự `2`, `5`, `.`, `5` — và máy được giao việc hiểu xem người ấy muốn nói
con số nào. Nếu ở đây máy tự ý bỏ đi hai ký tự cuối, nó đang **sửa lời người
khác** rồi im lặng làm tiếp.

Nên `int()` đặt lên một chuỗi chỉ nhận đúng những gì viết ra một số nguyên.
Thấy dấu chấm, nó dừng lại.
::::

::::example{#may-tu-choi-dau-cham}
Đây là đúng câu hỏi bỏ ngỏ của bài trước, viết ra thành code:

```python title=readonly
o_tien = "25.5"            # người ghi sổ gõ vào, đơn vị nghìn
so_nghin = int(o_tien)
print(so_nghin)
```

Máy không in ra con số nào. Nó dừng lại và nói:

```text
Traceback (most recent call last):
  File "so_chi_tieu.py", line 2, in <module>
    so_nghin = int(o_tien)
ValueError: invalid literal for int() with base 10: '25.5'
```

`ValueError` bạn đã gặp ở Realm 0: **kiểu thì đúng, nội dung thì không**. Máy
nhận được một chuỗi — đúng thứ `int()` biết đọc — nhưng nội dung bên trong
chuỗi ấy không viết ra một số nguyên.

Nhìn kỹ mấy chữ tiếng Anh cuối dòng: *invalid literal for int()* nghĩa là "cái
bạn viết ra không phải một số nguyên hợp lệ". Và cuối cùng máy chép lại nguyên
văn chỗ nó không đọc nổi: `'25.5'`. Đó là thói quen tốt của thông báo lỗi
Python — nó cho bạn xem đúng thứ nó nhận được, nên bạn không phải đoán.

Công cụ đọc chuỗi có dấu chấm là một cái tên khác:

```python title=readonly
o_tien = "25.5"
so_nghin = float(o_tien)
print(so_nghin)
print(type(so_nghin))
```

```text
25.5
<class 'float'>
```

`float()` đặt lên một chuỗi thì đọc mấy ký tự đó thành một **số thực**. Nó nhận
cả chuỗi có dấu chấm (`"25.5"`) lẫn chuỗi không có (`"220"` ra `220.0`) — vì
một số nguyên bao giờ cũng viết được thành số thực, còn chiều ngược lại thì
không.
::::

::::predict{#doan-o-tien commitOnce}
Byte đang chép một dòng sổ. Ô tiền ghi `32.3` — ba mươi hai nghìn ba trăm.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
o_tien = "32.3"
so_nghin = int(o_tien)
print(so_nghin * 1000)
```

:::opt{correct}
Máy không in dòng nào, chỉ hiện một thông báo ValueError
:::

:::opt
32000
::why
Gần đúng ở chỗ bạn nhớ chính xác bài 2: `int()` cắt phần lẻ, nên `32.3` rụng
`.3` còn `32`, nhân 1000 ra 32000. Phép cắt ấy có thật và bạn dùng nó đúng.

Chỗ lệch nằm ở thứ đứng trong ngoặc. Phép cắt chỉ xảy ra khi `int()` nhận một
**số**. Ở đây nó nhận một **chuỗi**, và với chuỗi thì `int()` làm việc khác:
đọc lại con số người ta đã gõ. Trong việc đọc lại đó, cắt bớt ký tự của người
khác không phải quyền của máy.
::
:::

:::opt
32300
::why
Gần đúng ở chỗ bạn đọc `int()` theo nghĩa rộng nhất của nó — "đổi cái này
thành số" — nên `"32.3"` thành `32.3`, nhân 1000 ra 32300. Đó cũng đúng là con
số bạn muốn có ở cuối, và mấy dòng nữa bạn sẽ lấy được nó thật.

Chỗ lệch: cái tên `int` không có nghĩa "thành số", nó có nghĩa "thành **số
nguyên**". Một số nguyên thì không mang dấu chấm nào. Muốn giữ lại phần lẻ,
bạn phải gọi đúng tên của loại số giữ được phần lẻ.
::
:::

:::opt
Máy báo TypeError, vì chuỗi không phải là số
::why
Gần đúng ở chỗ bạn nhận ra chuyện này liên quan tới kiểu, và bạn nhớ đúng
`TypeError` là lỗi về kiểu — hai giá trị mang hai nhãn không đi cùng nhau
được trong một việc.

Chỗ lệch nằm ở chỗ ở đây kiểu **không** sai. `int()` sinh ra chính là để nhận
chuỗi; đưa chuỗi cho nó là đưa đúng thứ nó chờ. Sai là **nội dung** bên trong
chuỗi ấy. Đó là ranh giới giữa hai lỗi: sai nhãn thì `TypeError`, đúng nhãn mà
nội dung không dùng được thì `ValueError`.
::
:::
::::

::::explain{#dua-ve-dong-ngay}
Có `float()` rồi, nhưng bài 7 vừa ra một luật chưa ráo mực: **tiền giữ trong
máy phải là số đồng, kiểu `int`**. Mà `float()` thì trả về `float` — đúng thứ
bài 7 vừa đuổi ra khỏi bảng tiền.

Không mâu thuẫn, nếu bạn nhìn `float` ở đây là một **chặng đi qua**, không phải
chỗ dừng chân. Cả đoạn đường có ba chặng, và mỗi chặng bạn đã học rồi:

```python title=readonly
o_tien = "32.3"                       # ô trên phiếu: chữ
so_nghin = float(o_tien)              # chặng 1: chữ  → số thực (bài này)
tien = round(so_nghin * 1000)         # chặng 2: nghìn → đồng, và về int (bài 7)
print(tien)
```

```text
32300
```

Chặng 1 dùng `float` vì chuỗi có dấu chấm. Chặng 2 nhân 1000 rồi `round` để
quay về `int`. Sau dòng thứ ba, `float` biến mất khỏi chương trình và mọi con
số tiền lại là đồng, đúng luật bài 7.

> Chỗ dễ vấp: viết `int(float(o_tien) * 1000)` trông cũng hợp lý, và với `25.5`
> nó cho ra đúng `25500`. Nhưng với `32.3` thì `32.3 * 1000` máy giữ là
> `32299.999999999996`, và `int()` **cắt** — bạn nhận `32299`. Sai một đồng,
> không một lời báo. Chặng 2 phải là `round`, không phải `int`, đúng như bài 7
> đã chốt.
::::

::::code{#doc-o-tien-tren-phieu}
Trên phiếu ghi sổ của Byte, ô "tiền" là một ô chữ: người ghi sổ gõ số nghìn
vào đó, có thể có phần lẻ (`"32.3"`), cũng có thể không (`"220"`).

Hãy điền chỗ trống để `doc_tien` nhận ô chữ ấy và trả về số tiền tính bằng
**đồng**.

```python title=starter
def doc_tien(o_tien):
    so_nghin = ___
    return round(so_nghin * 1000)

print(doc_tien("32.3"))
print(doc_tien("220"))
```

```python title=solution
def doc_tien(o_tien):
    so_nghin = float(o_tien)
    return round(so_nghin * 1000)

print(doc_tien("32.3"))
print(doc_tien("220"))
```

```python title=test
# Chấm trên BỐN ô khác nhau. Ô "220" không có dấu chấm — nó là ô mà `int()`
# cũng đọc trôi, nên nếu chỉ chấm bằng ô đó thì đáp án sai vẫn qua. Ô "32.3"
# và "25.5" mới là chỗ phân biệt.
assert doc_tien("32.3") == 32300, "ô trên phiếu ghi 32.3 nghìn thì đọc ra ba mươi hai nghìn ba trăm đồng"
assert doc_tien("25.5") == 25500, "ô ghi 25.5 là hai mươi lăm nghìn rưỡi — phần sau dấu chấm cũng là tiền, không được bỏ"
assert doc_tien("220") == 220000, "ô không có dấu chấm vẫn là số nghìn, nên 220 đọc ra là 220 nghìn đồng"
assert doc_tien("7") == 7000, "người ghi sổ gõ mỗi số 7 là bảy nghìn đồng, không phải bảy đồng"
# Ra khỏi hàm thì tiền phải là số ĐẾM, đúng luật bài 7: số nguyên in ra không
# có đuôi `.0`, số thực thì có.
assert f"{doc_tien('32.3')}" == "32300", "ra khỏi hàm thì tiền phải là số đồng đếm được, in ra không kèm đuôi .0"
```

:::hints
- kind: attention
  body: Chỗ trống phải cho ra một con số, vì dòng ngay dưới nó đem `so_nghin` nhân với 1000. Thứ bạn có trong tay là `o_tien` — một chuỗi, có thể mang dấu chấm.
- kind: strategy
  body: "`int()` từ chối chuỗi có dấu chấm, nên nó không dùng được ở đây. Cần cái tên của loại số giữ được phần lẻ — đúng cái tên bạn đã thấy ở phần ví dụ, đặt trước `o_tien` y hệt cách bạn vẫn đặt `int` trước một chuỗi."
- kind: one-line
  body: "Viết `float(o_tien)` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^32300\n220000\s*$
- tier: static
  onFail: chỗ trống phải ĐỌC con số nằm trong `o_tien`, bằng công cụ nhận được dấu chấm
  requireAst:
  # Phải gọi `float`, và phải gọi nó trên chính ô chữ nhận vào: hai điều kiện
  # cùng lúc loại được cả `float(32.3)` chép cứng lẫn đáp án gõ bừa.
  - kind: uses-call, target: float, min: 1
  - kind: uses-name, target: o_tien, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chữ vào, đồng ra. Cái dấu chấm giờ không chặn được mình nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Thử một ô nữa xem. Người ghi sổ mở phiếu ra, tới ô "tiền" thì bấm Enter luôn —
không gõ gì cả. `input()` vẫn trả về một chuỗi, nhưng là chuỗi **rỗng**: `""`.

Đem chuỗi rỗng vào hàm bạn vừa viết:

```text
ValueError: could not convert string to float: ''
```

`float()` cũng chịu. Mà lần này không có công cụ nào cứu được, vì trong ô ấy
thật sự **không có con số nào để đọc**.

Nghĩa là phải hỏi trước: *ô này có ai gõ gì vào không?* Hỏi rồi mới đổi kiểu.
Câu hỏi ấy người ta hay viết gọn thế này:

```python
if o_tien:
    # tới đây mới đổi kiểu
```

Nhìn kỹ dòng `if` đó mà xem. Sau chữ `if` không có phép so sánh nào cả — không
`==`, không `>`, không gì hết. Chỉ có `o_tien`, và `o_tien` là một **chuỗi**.

Từ Realm 0 tới giờ, `if` luôn nhận một câu đúng-hoặc-sai. Vậy khi bạn đặt trần
một chuỗi vào đó, máy lấy đâu ra câu trả lời đúng/sai?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
