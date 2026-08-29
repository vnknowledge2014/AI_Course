---
id: nen-tang.gia-tri-bien-kieu.chuoi-sua-khong-duoc
title: Chuỗi sửa không được
summary: Xin xem một ký tự thì máy đưa, dán đè lên nó thì máy từ chối. Muốn đổi, phải dựng một chuỗi mới rồi gán lại cái tên.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.string-immutable]
requires: [core.string-sequence, core.string-literal, core.list-index, core.variable, core.reassign, core.rhs-first, core.string-concat, err.type-error, core.output]
concepts: [core.chuoi, core.gan-lai, core.loi-khi-chay]
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
Xin xem ký tự nào mình cũng đưa. Nhưng dán đè lên một ký tự thì mình chịu.
::::

::::explain{#tam-bien-da-khac}
Bài trước để lại đúng một câu hỏi: `ten[0]` đã chỉ thẳng vào ký tự đầu rồi, vậy
viết `ten[0] = "C"` để kéo nó thành chữ hoa có được không?

Trước khi chạy thử, ghé tiệm khắc biển đầu ngõ.

Quán đặt một tấm biển inox khắc dòng chữ `cà phê` treo trước cửa. Sáng ra chủ
quán đổi ý: muốn chữ **C** hoa cho ra dáng. Thợ khắc lắc đầu — nét đã ăn xuống
mặt inox thì không mài ngược lên được. Việc thợ làm là **khắc một tấm mới**, rồi
tháo tấm cũ xuống, treo tấm mới lên đúng cái móc ấy.

Chuỗi trong Python đúng như tấm biển đã khắc. Tính chất này có tên: chuỗi là giá
trị **bất biến** — tiếng Anh là *immutable*, nghĩa đen là *không đổi được*. Đã
dựng ra rồi thì không có cách nào sửa một mẩu bên trong nó.

Và cái móc treo biển chính là **cái tên**. Chỗ này dễ nhầm, nên tách cho rõ:

- **Cái tên** thì gỡ ra dán sang giá trị khác được — bạn làm chuyện đó từ Realm
  0, ở bài đổi giá trị của một cái tên.
- **Cái giá trị** — tấm biển — thì không sửa được một nét nào.

Bài này nằm đúng ở chỗ hai chuyện ấy hay bị gộp làm một.
::::

::::example{#doc-thi-duoc-ghi-thi-khong}
Đây là câu hỏi bỏ ngỏ của bài trước, viết ra thành code:

```python title=readonly
ten = "cà phê"
print(ten[0])
ten[0] = "C"
```

Chạy lên, hai dòng đầu êm ru, dòng thứ ba nổ:

```text
c
Traceback (most recent call last):
  File "so_chi_tieu.py", line 3, in <module>
    ten[0] = "C"
    ~~~^^^
TypeError: 'str' object does not support item assignment
```

Chữ `c` in ra được, nên chuyện `ten[0]` chỉ đúng vào ký tự đầu là thật. Máy chỉ
từ chối đúng một việc: **dán đè** lên chỗ đó.

Dòng cuối nói thẳng ra việc ấy, dịch sát từng mảnh: *`'str'` — thứ nằm bên trái
là một chuỗi; `does not support` — không nhận; `item assignment` — phép gán vào
một ô bên trong.* Vẫn là `TypeError`, cái tên bạn gặp từ Realm 0: sai không phải
ở cú pháp, mà ở **kiểu** của thứ bạn đem ra dùng.

Để ý một chi tiết đáng giá: máy không lặng lẽ bỏ qua dòng ấy, cũng không lặng lẽ
sửa giúp. Nó dừng hẳn. Nếu dưới đó còn mười dòng nữa thì mười dòng đó không được
chạy.

Danh sách thì khác hẳn. Ở Realm 0, `mon_da_goi.append("bún chả")` làm **chính
cái danh sách ấy** dài thêm một món — không ai phải dựng danh sách mới. Chuỗi
không có cửa nào như thế. Cùng là "một chỗ chứa nhiều thứ có thứ tự", nhưng một
bên sửa tại chỗ được, một bên thì không.
::::

::::predict{#doan-may-lam-gi commitOnce}
Sổ ghi tên khoản là `"pho bo"`, Byte muốn chữ đầu thành hoa. **Trước khi bấm
chạy**, bạn đoán màn hình hiện ra gì?

```python
mon = "pho bo"
mon[0] = "P"
print(mon)
```

:::opt{correct}
Máy không in dòng nào, chỉ hiện một thông báo TypeError
:::

:::opt
Pho bo
::why
Gần đúng ở chỗ bạn đọc `mon[0]` là *ô đầu tiên của chuỗi* — đúng y như vậy, và
nếu đây là `print(mon[0])` thì bạn đã lấy ra được chữ `p`.

Chỗ lệch nằm ở dấu `=` đặt sau cái ô ấy. Xin **xem** một ô và đòi **dán đè** lên
một ô là hai việc khác nhau, và chuỗi chỉ nhận việc thứ nhất. Tấm biển đã khắc
thì đọc bao nhiêu lần cũng được, mài lại một nét thì không.
::
:::

:::opt
pho bo
::why
Gần đúng ở chỗ bạn kết luận `mon` sẽ không đổi — phần kết luận ấy trúng: sau ba
dòng này, chuỗi `"pho bo"` không suy suyển một ký tự nào.

Chỗ lệch nằm ở cách máy xử sự với dòng nó không làm được. Nó không nhún vai đi
tiếp; nó dừng lại và nói ra. Đây là tính cách Python đã có từ bài `TypeError`
đầu tiên ở Realm 0: thà báo còn hơn để bạn tưởng chương trình chạy đúng.
::
:::

:::opt
Máy báo SyntaxError, từ chối trước khi chạy dòng nào
::why
Gần đúng ở chỗ bạn nhớ rằng có loại lỗi máy bắt được **trước** khi chạy, và
`mon[0] = "P"` trông đúng là loại dòng đáng ngờ.

Chỗ lệch nằm ở thứ `SyntaxError` bắt được: nó chỉ soi **hình dạng** câu lệnh.
Mà `<cái gì đó>[<số>] = <giá trị>` là hình dạng hợp lệ hoàn toàn — với danh sách
thì dòng y hệt vậy chạy ngon lành. Máy chỉ biết bên trái là một chuỗi vào đúng
lúc chạy tới đó, nên đây là lỗi lúc chạy, và chữ `p` ở dòng trên vẫn kịp in ra
nếu có `print`.
::
:::
::::

::::explain{#treo-tam-bien-moi}
Máy đã từ chối. Vậy làm cách nào để có `"Cà phê"`?

Làm đúng việc người thợ khắc làm: **dựng một tấm mới**, rồi treo cái tên lên tấm
đó.

```python title=readonly
ten = "cà phê"
ten_moi = "C" + ten[1] + ten[2] + ten[3] + ten[4] + ten[5]
print(ten_moi)
print(ten)
```

```text title=readonly
Cà phê
cà phê
```

Đọc vế phải từ trái sang: `"C"` là chữ hoa bạn tự viết ra — nó không có sẵn
trong `ten`. Năm mảnh còn lại lấy từ chính `ten`, từng ô một. Dấu `+` dán sáu
mảnh thành một chuỗi, đúng phép nối chuỗi bạn dùng từ Realm 0.

Hai dòng `print` cho thấy trọn câu chuyện:

- `ten_moi` là một chuỗi **khác**, vừa được dựng ra.
- `ten` vẫn nguyên `"cà phê"`. Không ai đụng vào nó, vì không ai đụng được.

Muốn cái tên cũ mang chữ hoa thì gán lại chính nó:

```python title=readonly
ten = "cà phê"
ten = "C" + ten[1] + ten[2] + ten[3] + ten[4] + ten[5]
print(ten)
```

```text title=readonly
Cà phê
```

Dòng giữa có `ten` ở cả hai bên dấu `=`, và nó chạy đúng nhờ luật bài `Vế phải
tính xong hết rồi mới gán`: máy dựng trọn chuỗi mới bằng giá trị **cũ** của
`ten`, xong
rồi mới gỡ cái tên ra dán sang chuỗi mới. Chuỗi cũ không bị sửa — nó chỉ bị bỏ
lại, không còn cái tên nào trỏ vào.

Nói gọn thành một câu đáng nhớ: **chuỗi không sửa được, chỉ có cái tên là đổi
được.**
::::

::::code{#khac-tam-bien-moi}
Khách gõ tên khoản vào sổ là `"trà sữa"`, chữ thường tuốt. Cột "tên khoản" của
sổ quy định chữ đầu phải viết hoa, nên Byte cần `"Trà sữa"`.

`ten` có bảy ô, đánh số từ `0` đến `6`. Ô số `0` đang giữ chữ `t` thường — thứ
bạn phải thay. Sáu ô còn lại giữ nguyên.

Điền vào chỗ trống để dựng ra chuỗi mới. Đừng đụng vào dòng `ten = "trà sữa"`.

```python title=starter
ten = "trà sữa"
ten_moi = ___
print(ten_moi)
print(ten)
```

```python title=solution
ten = "trà sữa"
ten_moi = "T" + ten[1] + ten[2] + ten[3] + ten[4] + ten[5] + ten[6]
print(ten_moi)
print(ten)
```

```python title=test
# Ba câu hỏi khác nhau, một chỗ trống. Phải đúng cả ba mới là hiểu bài.
# 1. Chuỗi mới dựng đúng chưa — sai một ô là lệch ngay.
assert ten_moi == "Trà sữa", "cột tên khoản đòi chữ đầu viết hoa, nên tấm biển mới phải đọc ra là Trà sữa"
# 2. Chuỗi cũ có suy suyển không. Đây là chỗ luật bất biến hiện ra: bất kỳ cách
#    nào "sửa tại chỗ" đều đã nổ TypeError từ trước, nên `ten` buộc phải nguyên.
assert ten == "trà sữa", "tấm biển cũ vẫn nguyên chữ khách gõ — không ai mài lại được một nét của chuỗi đã dựng"
# 3. Kết quả trùng khít với chuỗi ghép từ chính các ô của `ten`.
#
#    Chú ý: assert này KHÔNG bắt được đáp án gõ tay. `ten` không hề đổi, nên vế
#    phải của nó luôn tính ra đúng "Trà sữa", và `ten_moi = "Trà sữa"` gõ thẳng
#    cũng qua. Việc bắt buộc PHẢI ĐỌC `ten` do tier `static` dưới đây làm, không
#    phải dòng này.
assert ten_moi == "T" + ten[1] + ten[2] + ten[3] + ten[4] + ten[5] + ten[6], "sáu ký tự đằng sau phải lấy ra từ chính tên khách gõ, không phải gõ tay lại một tên khác"
```

:::hints
- kind: attention
  body: Nhìn vế trái dòng bạn phải điền — nó là một cái tên MỚI, `ten_moi`. Bài không nhờ bạn sửa `ten`; nó nhờ bạn dựng ra một chuỗi khác. Chữ `T` hoa không nằm trong `ten` ô nào cả, còn sáu ký tự đằng sau thì nằm sẵn ở các ô số 1 đến 6.
- kind: strategy
  body: Bảy mảnh, dán lại bằng dấu `+` như phép nối chuỗi ở Realm 0. Mảnh đầu bạn tự viết ra giữa hai dấu nháy. Sáu mảnh sau lấy từ `ten`, mỗi mảnh một cặp ngoặc vuông, số trong ngoặc chạy từ 1 tới 6 — nhớ là ô đầu tiên mang số 0, nên ô số 6 là ô cuối cùng của một chuỗi bảy ký tự.
- kind: one-line
  body: Viết `"T" + ten[1] + ten[2] + ten[3] + ten[4] + ten[5] + ten[6]` vào chỗ trống.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Trà sữa\ntrà sữa\s*$
- tier: static
  onFail: sáu ký tự sau chữ T hoa phải được ĐỌC ra từ `ten`, không gõ tay lại tên khoản
  requireAst:
  # `min: 2`, không phải 6. Khung đã sẵn `print(ten)` — một lần đọc — nên mọi
  # đáp án gõ tay dừng ở 1 và trượt, còn mọi cách dựng thật đều từ 2 trở lên.
  #
  # Không đặt cao hơn: `"T" + ten[1:]` chỉ đọc `ten` hai lần mà vẫn là một lời
  # giải ĐÚNG. Đặt `min: 4` như số lần đọc của lời giải mẫu sẽ đánh trượt nó —
  # và đánh trượt một người viết đúng thì tệ hơn cho lọt một người viết sai.
  #
  # Con số này ăn theo số lệnh `print` trong khung: sửa khung thì phải sửa đây.
  - kind: uses-name, target: ten, min: 2
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tấm mới treo lên rồi, tấm cũ vẫn y nguyên. Mình chưa mài của bạn nét nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cách dựng chuỗi mới thì chạy được, nhưng nhìn lại dòng vừa viết mà xem: bảy ký
tự là sáu dấu `+` và sáu cặp ngoặc vuông.

Giờ quay về việc thật của cột "tên khoản": cột rộng 12 chỗ, còn `"cà phê sữa đá"`
có 13 ký tự. Muốn lấy 12 ký tự đầu để nhét vừa cột, bạn sẽ phải viết
`ten[0] + ten[1] + ten[2]` … cho tới `ten[11]` — mười hai mảnh, mười một dấu
cộng, và mắt bạn phải tự đếm xem đã đủ chưa.

Tệ hơn: tháng sau bà chủ kẻ lại sổ, cột rộng thành 15 chỗ. Cả dòng ấy viết lại
từ đầu.

Cái bạn cần không phải xin từng ô một. Bạn muốn xin thẳng **một khúc** — từ ô
này tới ô kia, một lần. Máy có cách nói nào cho việc đó không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
