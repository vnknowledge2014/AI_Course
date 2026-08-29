---
id: nen-tang.list-dict-set-tuple.hoi-mot-khoa-la
title: Hỏi một khoá không có
summary: Tra một khoá chưa từng ghi thì máy không đưa ra số 0, cũng không đưa ra khoảng trống — nó dừng lại và gọi tên đúng cái khoá nó không tìm thấy.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [err.key-error]
requires: [core.dict, core.value-error, err.type-error, err.traceback, core.fstring, core.variable]
concepts: [core.so-tra-cuu, core.loi-khi-chay]
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
Hỏi mình một cái nhãn mình không giữ, mình không đoán bừa. Mình kêu lên.
::::

::::explain{#ba-cau-tra-loi-nghe-deu-lot-tai}
Byte vừa gõ thử `chi["học phí"]` — một khoá mà cuốn sổ chưa từng ghi lần nào —
và bài trước dừng lại đúng ở câu hỏi: **máy đưa ra cái gì?**

Ba câu trả lời nghe đều lọt tai, và mỗi câu đều có lý riêng của nó:

- Số `0` — sổ chưa ghi nhóm ấy, nghĩa là nhóm ấy chưa tiêu đồng nào.
- Một khoảng trống — ô ấy còn để trống thì đưa ra thứ trống.
- `None` — cái tên mà Realm 0 đã đặt cho *chưa có gì*.

Máy không chọn câu nào trong ba. Nó **dừng chương trình lại**, và trên màn hình
hiện ra một loại lỗi mới:

```text title=readonly
KeyError: 'học phí'
```

`KeyError` — lỗi khoá. Dòng ấy đọc theo đúng lối bạn học ở Realm 0: bên trái
dấu hai chấm là **loại** lỗi, bên phải là chi tiết. Và chi tiết ở đây quý hơn
mọi lời giải thích: nó in ra nguyên văn **cái khoá** mà máy đã đi tìm và không
thấy.

Vì sao máy chọn cách này, chứ không lặng lẽ đưa ra số `0`? Vì hai chuyện dưới
đây trông giống hệt nhau nếu cùng cho ra `0`:

- tháng này nhóm *học phí* có phát sinh, và tổng của nó đúng bằng 0 đồng;
- tháng này bạn gõ nhầm tên nhóm, nên cuốn sổ không hề có khoá ấy.

Chuyện thứ nhất là một câu trả lời. Chuyện thứ hai là một lỗi. Máy không có
cách nào biết bạn đang ở tình huống nào, nên nó không trả lời thay bạn.
::::

::::example{#no-ngay-tai-dong-tra}
Cuốn sổ chi tiêu của Byte, đúng cuốn ở bài trước:

```python title=readonly
chi = {
    "ăn sáng": 85000,
    "sửa xe": 500000,
    "biếu bà": 300000,
    "đổ xăng": 120000,
}

tien = chi["học phí"]
print(f"Học phí: {tien} đồng")
print("Đã in xong báo cáo")
```

Màn hình hiện ra:

```text
Traceback (most recent call last):
  File "bao_cao.py", line 8, in <module>
    tien = chi["học phí"]
           ~~~^^^^^^^^^^^
KeyError: 'học phí'
```

Đọc từ dòng cuối lên, đúng như bài traceback ở Realm 0:

- **Dòng cuối** nói loại lỗi và chi tiết: `KeyError`, khoá `'học phí'`.
- **Dòng trên nữa** chép lại chính dòng lệnh gây ra chuyện, kèm mấy dấu chỉ vào
  đúng cặp ngoặc vuông có lỗi.
- **Dòng trên nữa** nói lỗi nằm ở dòng số 8.

Còn một chuyện quan trọng nữa không nằm trong traceback — nó nằm ở chỗ
**thiếu**, và khối đoán phía dưới sẽ hỏi thẳng bạn về nó.
::::

::::explain{#ho-hang-voi-value-error}
`KeyError` không phải một loại lỗi lạ. Nó là họ hàng gần của một loại bạn đã
gặp rồi.

Ở Realm 0, `int("hai lăm")` cho ra `ValueError`. Đọc kỹ thì lỗi ấy nói: *việc
bạn nhờ tôi làm — đổi một câu chữ thành số — là việc tôi biết làm; nhưng cái
nội dung bạn đưa vào thì tôi không đổi được.* Đúng loại việc, sai nội dung.

`chi["học phí"]` cũng vậy: *tra một cuốn sổ bằng một khoá là việc tôi biết
làm; nhưng cái khoá bạn đưa vào thì cuốn sổ này không có.* Cũng đúng loại
việc, cũng sai nội dung.

Hãy đặt nó cạnh `TypeError` để thấy chỗ khác. `TypeError` xảy ra khi việc bạn
nhờ **không tồn tại**: không có quy ước nào cộng một câu chữ với một con số.
Còn `KeyError` thì việc vẫn tồn tại, chỉ là cái khoá không có mặt.

> **Đây là một món quà, không phải một tai nạn.** Cả track này đang đuổi theo
> đúng một thứ: lỗi không tự nói ra. Ở bài hai dãy song song, bạn xoá một tên
> mà quên xoá tiền, rồi máy in tên một đằng tiền một nẻo — **không một tiếng
> báo**. Với cuốn sổ tra bằng khoá thì **khi ĐỌC**, chuyện đó không
> xảy ra được nữa: hỏi nhầm một khoá là máy dừng lại và gọi thẳng tên cái khoá
> ấy ra. Một lỗi biết kêu bao giờ cũng rẻ hơn một con số sai biết im.
>
> (Còn khi GHI vào sổ thì máy lại im — hai bài nữa bạn sẽ gặp, và sẽ thấy vì
> sao nó buộc phải im.)
::::

::::predict{#doan-man-hinh commitOnce}
Cuốn sổ ấy CÓ khoá `"ăn sáng"`. Nhưng Byte gõ vội, và gõ hoa chữ đầu.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
chi = {
    "ăn sáng": 85000,
    "sửa xe": 500000,
    "biếu bà": 300000,
    "đổ xăng": 120000,
}

tien = chi["Ăn sáng"]
print(f"Ăn sáng: {tien} đồng")
print("Đã in xong báo cáo")
```

:::opt{correct}
Máy dừng lại với `KeyError: 'Ăn sáng'`, và không lệnh `print` nào kịp in ra chữ nào
:::

:::opt
In `Ăn sáng: 85000 đồng`, rồi in `Đã in xong báo cáo`
::why
Gần đúng ở chỗ bạn đọc ra đúng ý người viết: họ muốn tra khoản ăn sáng, và
khoản ấy có thật trong sổ, đúng 85000 đồng. Ý định thì không sai chỗ nào.

Chỗ lệch: cuốn sổ không tra bằng ý định, nó tra bằng khoá. Và khoá của Python
so từng ký tự một — `"Ăn sáng"` khác `"ăn sáng"` ngay ký tự đầu, y như
`"Trà sữa"` khác `"trà sữa"` ở bài chuỗi. Máy không có cách nào biết bạn định
gõ chữ thường; nó chỉ thấy một khoá nó chưa từng ghi.
::
:::

:::opt
In `Ăn sáng: 0 đồng`, rồi in `Đã in xong báo cáo`
::why
Gần đúng ở chỗ bạn thấy khoá này lệch so với sổ, và cho rằng máy sẽ trả về
một thứ trung tính thay vì dừng hẳn. Có công cụ làm đúng như vậy thật — bài
sau sẽ đưa bạn một cái.

Chỗ lệch: `0` là một câu trả lời, không phải một lời từ chối. Nếu cặp ngoặc
vuông trả `0` thì cuốn sổ ghi *ăn sáng hết 0 đồng* — một con số sai mà không
kêu tiếng nào. Cặp ngoặc vuông chọn kêu.
::
:::

:::opt
Máy dừng lại với `KeyError`, nhưng dòng `Ăn sáng:` đã kịp in ra trước đó
::why
Gần đúng ở chỗ khó nhất: bạn nhận ra dòng tra là dòng gây lỗi, và bạn hình
dung máy đi từ trên xuống. Cả hai đều đúng.

Chỗ lệch nằm ở thứ tự bên trong một dòng. Dòng `tien = chi["Ăn sáng"]` đứng
TRƯỚC hai lệnh `print`, và nó nổ ngay tại đó — nên chưa lệnh `print` nào tới
lượt. Máy dừng đúng chỗ nổ, không chạy nốt phần còn lại rồi mới báo.
::
:::
::::

::::explain{#hai-lenh-print-khong-kip-chay}
Đúng như bạn vừa đoán: hai lệnh `print` phía dưới không in được chữ nào.

Máy dừng ngay tại dòng 8. Mọi dòng sau đó, dù viết đúng tới đâu, cũng không
được chạy — y hệt cách `ValueError` đã cắt ngang chương trình đổi kiểu ở
Realm 0. Một chương trình chết giữa chừng không để lại nửa bản báo cáo; nó để
lại không có gì, và đó thường là điều may, vì nửa bản báo cáo thì trông y như
một bản đầy đủ.
::::

::::code{#tra-dung-khoa-so-dang-giu}
Vẫn cuốn sổ ấy. Lần này Byte muốn in ra khoản biếu bà, và đã gõ như sau:

```text title=readonly
tien = chi["biếu"]
```

Máy dừng lại ngay, với `KeyError: 'biếu'`.

Chỗ trống dưới đây chính là chỗ ấy. Hãy đọc lời báo lỗi, so nó với những khoá
mà cuốn sổ **thật sự** đang giữ, rồi điền vào khoá đúng.

```python title=starter
chi = {
    "ăn sáng": 85000,
    "sửa xe": 500000,
    "biếu bà": 300000,
    "đổ xăng": 120000,
}

tien = chi[___]

print(f"Biếu bà tháng này: {tien} đồng")
```

```python title=solution
chi = {
    "ăn sáng": 85000,
    "sửa xe": 500000,
    "biếu bà": 300000,
    "đổ xăng": 120000,
}

tien = chi["biếu bà"]

print(f"Biếu bà tháng này: {tien} đồng")
```

```python title=test
# Chỉ một chỗ trống, và nó nằm trong cặp ngoặc vuông. Ba khoá còn lại của sổ
# cho ra ba con số khác hẳn, còn một khoá không có trong sổ thì làm chương
# trình dừng trước khi dòng dưới đây kịp chạy.
assert tien == 300000, "sổ này ghi khoản biếu bà là 300000 đồng; khoá `ăn sáng` cho ra 85000, khoá `sửa xe` cho ra 500000, khoá `đổ xăng` cho ra 120000, nên chỉ khoá `biếu bà` mới lấy được đúng con số đang cần"
```

:::hints
- kind: attention
  body: Lời báo lỗi in ra nguyên văn cái khoá máy đã đi tìm — `'biếu'`. Đặt nó cạnh bốn khoá đang nằm trong cuốn sổ ở đầu đoạn và so từng chữ một.
- kind: strategy
  body: Khoá của cuốn sổ này là một câu chữ, nên chỗ trống cũng phải là một câu chữ có nháy. Máy so khoá theo đúng từng ký tự, y như phép `==` giữa hai chuỗi — thừa hay thiếu một chữ là một khoá khác hẳn, chứ không phải một khoá gần giống.
- kind: one-line
  body: 'Viết `"biếu bà"` vào chỗ trống, giữ nguyên cặp ngoặc vuông ở hai bên.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Biếu bà tháng này: 300000 đồng
- tier: static
  onFail: chỗ trống phải là cái khoá viết đúng như cuốn sổ đang giữ, không phải một khoá gần giống
  requireAst:
  # `min: 2` vì chuỗi `biếu bà` đã xuất hiện MỘT lần trong khung, ở dòng khoá
  # thứ ba của cuốn sổ. Chỗ trống điền đúng là lần thứ hai.
  - kind: has-literal, target: biếu bà, min: 2
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khoá khớp rồi. Mình không phải kêu lên lần nào cả.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bây giờ bạn biết máy làm gì khi bị hỏi một khoá lạ: nó dừng lại và gọi tên
khoá ấy ra. Ở bài này, dừng lại là chuyện tốt — nó cứu bạn khỏi một con số
sai.

Nhưng hãy nghĩ tới bản báo cáo cuối tháng. Byte muốn một bản báo cáo dùng lại
được cho mọi tháng, nên nó có những dòng cố định, tháng nào cũng in đủ. Một
trong những dòng ấy là *sửa nhà* — mà tháng này Byte chẳng sửa gì trong nhà
cả, nên cuốn sổ không hề có khoá đó. Chương trình chạy tới dòng tra *sửa nhà*
thì chết đứng, trong khi thứ bạn muốn in ra chỉ là một dòng thật thà:
`Sửa nhà: 0 đồng`.

Không muốn cả chương trình chết chỉ vì hỏi một nhóm chi chưa từng phát sinh.
Có cách hỏi mà không có thì **nhận về 0** không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
