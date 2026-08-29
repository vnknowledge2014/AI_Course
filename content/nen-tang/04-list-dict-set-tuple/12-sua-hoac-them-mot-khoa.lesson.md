---
id: nen-tang.list-dict-set-tuple.sua-hoac-them-mot-khoa
title: Sửa một khoá, thêm một khoá
summary: Đặt một ô của sổ tra cứu vào bên trái dấu `=` — khoá có sẵn thì bị ghi đè, khoá chưa có thì mọc ra; cùng một dòng lo cả hai việc.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.dict-assign]
requires: [core.dict, err.key-error, core.dict-get-default, core.assignment, core.reassign, core.list-append, core.fstring]
concepts: [core.so-tra-cuu, core.gan]
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
Muốn đổi giá một món, bạn không chép lại cả thực đơn. Bạn xoá đúng một dòng.
::::

::::explain{#o-cua-so-dung-ben-trai-dau-bang}
Quán tăng giá phở lên 50000 đồng. Bảng giá đang là một cuốn sổ tra cứu ba
khoá, và bạn cần đổi đúng một con số trong đó.

Dựng lại cả cuốn sổ thì được, nhưng nhìn là thấy vô lý: gõ lại ba dòng chỉ để
sửa một, và mỗi lần gõ lại là một lần có thể gõ nhầm hai dòng kia.

Thứ bạn cần đã nằm sẵn trong tay từ lâu. Ở Realm 0, dấu `=` dán một cái tên
lên một giá trị, và gán lại thì cái tên trỏ sang giá trị mới. Ở bài trước nữa,
`sổ[khoá]` là cách gọi ra đúng **một ô** trong cuốn sổ. Ghép hai thứ ấy lại:

```python title=readonly
gia["Phở bò"] = 50000
```

Bên trái dấu `=` lần này không phải một cái tên trần, mà là **một ô của cuốn
sổ**. Đọc thành lời: *ô mang khoá `"Phở bò"` trong sổ `gia`, từ giờ giữ con số
50000.*

Đây là chỗ cuốn sổ tra cứu giống danh sách chứ không giống chuỗi: nó **sửa
được tại chỗ**. Danh sách có `.append` để mọc dài ra; cuốn sổ tra cứu thì dùng
đúng dấu `=` bạn đã quen.
::::

::::explain{#mot-loi-viet-hai-viec}
Câu hỏi tiếp theo tự nhiên phải là: thế còn khoá **chưa có** trong sổ thì sao?

Quán bán thêm quẩy, 10000 đồng một đĩa. Bảng giá chưa hề có khoá `"Quẩy"`.
Viết `gia["Quẩy"] = 10000` thì máy làm gì?

Nó không kêu lên. Nó **tạo thêm khoá ấy** và đặt con số vào. Cuốn sổ đi từ ba
khoá lên bốn khoá.

Vậy cùng một lối viết lo hai việc, và máy tự chọn việc nào theo tình trạng của
cuốn sổ:

- **Khoá đã có** — con số cũ bị con số mới đè lên. Sổ vẫn còn đúng bấy nhiêu
  khoá.
- **Khoá chưa có** — sổ mọc thêm đúng một khoá, mang con số bạn vừa đặt vào.

Đặt cạnh hai bài vừa rồi thì hình dạng hiện ra rất rõ, và nó chỉ có một câu:

| việc | khoá lạ thì máy làm gì |
|---|---|
| **đọc** bằng `sổ[khoá]` | dừng chương trình, kêu `KeyError` |
| **đọc** bằng `sổ.get(khoá, dự_phòng)` | đưa ra con số dự phòng, sổ không đổi |
| **ghi** bằng `sổ[khoá] = giá_trị` | im lặng tạo khoá mới |

Đọc một khoá lạ là dấu hiệu có gì đó sai, nên máy kêu. Ghi vào một khoá lạ thì
thường là bạn đang cố ý thêm mới, nên máy im và làm theo.

> **Cái giá của sự im lặng ấy.** Máy im nghĩa là gõ nhầm tên khoá cũng không
> ai báo: `gia["Phơ bò"] = 50000` sẽ đẻ ra một khoá thứ tư trông rất giống
> khoá cũ, còn `"Phở bò"` thì vẫn 45000 như chưa có chuyện gì. Đây đúng loại
> lỗi biết im mà cả track này đang đuổi theo từ hai dãy song song. Khi ghi, hãy
> chép tên khoá từ chính cuốn sổ ra chứ đừng gõ lại theo trí nhớ.
::::

::::example{#ba-khoa-thanh-bon-khoa}
Cả hai việc trong một đoạn, và bản in ra ở hai thời điểm để bạn thấy chỗ đổi:

```python title=readonly
gia = {
    "Phở bò": 45000,
    "Lẩu gà": 60000,
    "Trà đá": 5000,
}

print(f"Phở bò lúc đầu: {gia['Phở bò']} đồng")
print(f"Quẩy lúc đầu: {gia.get('Quẩy', 0)} đồng")

gia["Phở bò"] = 50000
gia["Quẩy"] = 10000

print(f"Phở bò lúc sau: {gia['Phở bò']} đồng")
print(f"Quẩy lúc sau: {gia['Quẩy']} đồng")
```

Máy in ra:

```text
Phở bò lúc đầu: 45000 đồng
Quẩy lúc đầu: 0 đồng
Phở bò lúc sau: 50000 đồng
Quẩy lúc sau: 10000 đồng
```

Dòng thứ hai phải hỏi bằng `.get` mới in ra được, vì lúc đó khoá `"Quẩy"` chưa
tồn tại — hỏi bằng ngoặc vuông ở chỗ ấy thì chương trình đã dừng từ đầu. Còn
dòng cuối thì ngoặc vuông dùng được, vì khoá đã mọc ra ở dòng ghi phía trên.

Và đây là chỗ trả nợ cho cuối mạch Hàm. Ngày ấy tên món nằm ở `ten_mon`, giá
món nằm ở `gia_mon`, hai dãy rời nhau; thêm `"Quẩy"` vào dãy tên mà quên
`10000` ở dãy giá thì hoá đơn ra một con số sai không kèm tiếng báo nào. Bây
giờ tên món và giá món đi vào sổ bằng **cùng một dòng**, nên không có cách nào
thêm được nửa món.
::::

::::predict{#doan-gia-pho commitOnce}
Byte muốn tăng giá phở, và gõ khoá bằng trí nhớ: `"Phở Bò"`, chữ B viết hoa.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
gia = {
    "Phở bò": 45000,
    "Lẩu gà": 60000,
    "Trà đá": 5000,
}

gia["Phở Bò"] = 50000

print(gia["Phở bò"])
```

:::opt{correct}
In ra `45000`
:::

:::opt
In ra `50000`
::why
Gần đúng ở chỗ bạn đọc được ý định của người viết, và ý định ấy đúng là tăng
giá phở. Nếu đưa hai dòng này cho một người bán hàng đọc, họ cũng hiểu như
bạn.

Chỗ lệch là máy không đọc ý định, nó so khoá theo từng ký tự — y hệt phép `==`
giữa hai chuỗi, nơi `"Cà Phê"` và `"cà phê"` cho ra `False`. `"Phở Bò"` và
`"Phở bò"` lệch nhau một chữ, nên với máy chúng là hai khoá khác nhau hoàn
toàn. Con số 50000 đã được đặt vào một khoá mới toanh, còn khoá cũ không ai
đụng tới.
::
:::

:::opt
Máy dừng lại với `KeyError: 'Phở Bò'`
::why
Gần đúng ở chỗ bạn nhớ chính xác bài trước nữa: một khoá không có trong sổ đúng
là gây ra `KeyError`. Bạn cũng nhận ra `"Phở Bò"` không nằm trong ba khoá ban
đầu — nhận xét ấy chính xác.

Chỗ lệch nằm ở việc dòng ấy đang **đọc** hay đang **ghi**. `KeyError` là phản
ứng khi máy phải *lấy ra* thứ không có. Ở đây ô của sổ đứng bên trái dấu `=`,
nghĩa là máy đang *đặt vào* — mà đặt vào một chỗ chưa có thì chỗ ấy được tạo
ra, không có gì để mà thiếu.
::
:::

:::opt
Máy báo lỗi vì cuốn sổ không được chứa hai khoá giống nhau đến vậy
::why
Gần đúng ở chỗ bạn cảm thấy có gì đó không ổn — và cảm giác ấy đúng, đoạn này
thật sự có một con bọ. Trực giác của bạn về chỗ hỏng là chính xác.

Chỗ lệch là ai sẽ báo cho bạn. Máy chỉ so khoá bằng nhau hay khác nhau, nó
không có khái niệm "giống nhau đến mức đáng ngờ". Hai khoá lệch một chữ là hai
khoá khác nhau, hết chuyện — nên nó im lặng làm theo, và con bọ này không ai
chỉ ra hộ bạn.
::
:::
::::

::::code{#tang-gia-va-them-mon}
Bảng giá của quán đang có ba món. Sáng nay có hai thay đổi:

- phở bò tăng lên 50000 đồng;
- quán bán thêm quẩy, 10000 đồng một đĩa.

Điền hai dòng vào hai chỗ trống. Cả hai đều **đặt vào cuốn sổ đang có** —
đừng dựng lại một cuốn sổ mới, và đừng đụng tới hai món còn lại.

```python title=starter
gia = {
    "Phở bò": 45000,
    "Lẩu gà": 60000,
    "Trà đá": 5000,
}

___

___

print(f"Phở bò: {gia['Phở bò']} đồng")
print(f"Quẩy: {gia['Quẩy']} đồng")
print(f"Lẩu gà: {gia['Lẩu gà']} đồng")
```

```python title=solution
gia = {
    "Phở bò": 45000,
    "Lẩu gà": 60000,
    "Trà đá": 5000,
}

gia["Phở bò"] = 50000

gia["Quẩy"] = 10000

print(f"Phở bò: {gia['Phở bò']} đồng")
print(f"Quẩy: {gia['Quẩy']} đồng")
print(f"Lẩu gà: {gia['Lẩu gà']} đồng")
```

```python title=test
# Bốn câu, bốn vai. Hai câu đầu canh hai việc của cùng một lối viết: ghi đè
# một khoá đã có, và làm mọc ra một khoá chưa có. Hai câu sau canh chuyện dễ
# hỏng nhất khi ai đó dựng lại cả cuốn sổ thay vì sửa đúng chỗ.
assert gia["Phở bò"] == 50000, "quán tăng giá phở lên 50000 đồng, nên khoá `Phở bò` đã có sẵn phải mang con số mới thay cho 45000"
assert gia["Quẩy"] == 10000, "bảng giá lúc đầu không có khoá `Quẩy`, nên phép ghi phải làm khoá ấy mọc ra và mang giá 10000 đồng"
assert gia["Lẩu gà"] == 60000, "chỉ hai khoá được đụng tới trong đoạn này, nên lẩu gà vẫn phải là 60000 đồng như lúc đầu"
assert gia["Trà đá"] == 5000, "chỉ hai khoá được đụng tới trong đoạn này, nên trà đá vẫn phải là 5000 đồng như lúc đầu"
```

:::hints
- kind: attention
  body: Cuốn sổ đã được dựng xong ở đầu đoạn rồi, đừng dựng lại nó. Thứ bạn cần viết là hai dòng lẻ, mỗi dòng nói về đúng một ô — và ô ấy phải đứng ở bên TRÁI dấu `=`.
- kind: strategy
  body: Hai dòng có cùng một hình dạng, chỉ khác khoá và con số. Dòng thứ nhất nói về một khoá đã nằm trong sổ, nên con số cũ sẽ bị đè; dòng thứ hai nói về một khoá chưa có, nên sổ sẽ mọc thêm khoá. Chép tên khoá `Phở bò` từ chính cuốn sổ ở trên xuống, đừng gõ lại theo trí nhớ.
- kind: one-line
  body: 'Hai dòng lần lượt là `gia["Phở bò"] = 50000` và `gia["Quẩy"] = 10000`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Phở bò: 50000 đồng\nQuẩy: 10000 đồng\nLẩu gà: 60000 đồng\s*$
- tier: output
  expect: Quẩy: 10000 đồng
- tier: static
  onFail: hai chỗ trống phải ĐẶT VÀO cuốn sổ đang có, mỗi dòng một ô — không được dựng lại cả cuốn sổ
  requireAst:
  # Đếm số chỗ ĐỌC cái tên `gia`. Ba lệnh `print` đã dùng nó ba lần, và mỗi
  # dòng `gia[...] = ...` là một lần nữa — nên lời giải đúng có 5. Ai dựng lại
  # cuốn sổ bằng `gia = {...}` thì vế trái ấy là chỗ GÁN, không tính, và con số
  # đứng nguyên ở 3.
  - kind: uses-name, target: gia, min: 5
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dòng đè giá cũ, một dòng đẻ khoá mới. Hai món kia không ai đụng vào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã đủ đồ nghề để sống với một cuốn sổ tra cứu: tra bằng khoá, tra an toàn
bằng `.get`, sửa một ô, thêm một ô. Suốt tháng vừa rồi, mỗi lần phát sinh một
nhóm chi chưa có trong sổ, bạn lại viết thêm một dòng `chi[nhom] = tien`, và
đến cuối tháng cuốn sổ chi tiêu đã dày lên hai mươi khoá.

Giờ bạn muốn in cả cuốn sổ ra để xem lại. Hai mươi dòng `print` gõ tay thì
không xong, và tháng sau con số hai mươi ấy lại khác. Thứ bạn cần là một vòng
lặp — mà vòng lặp thì bạn đã có từ Realm 0: `for` lấy lần lượt từng phần tử
của một danh sách.

Đem đúng vòng lặp ấy đặt lên một cuốn sổ tra cứu:

```python title=readonly
for x in chi:
    print(x)
```

Sổ đã hai mươi khoá. Mỗi vòng, `x` là khoá, là giá trị, hay cả cặp?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.85}
::::
