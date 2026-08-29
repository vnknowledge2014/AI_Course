---
id: nen-tang.ham-vien-gach.dua-thieu-dua-thua
title: Đưa thiếu, đưa thừa
summary: Số thứ đưa vào phải khớp đúng số chỗ trống trong chữ ký — thiếu hay thừa đều dừng ngay bằng `TypeError`.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.function-arg-count]
requires: [core.docstring, core.function-signature, core.function-def, core.function-call, core.function-parameter, core.function-argument, core.fstring, err.type-error, err.traceback]
concepts: [core.ham, core.tham-so]
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
Mảnh giấy dán trên lọ là để bạn đọc. Mình thì đếm chỗ trống.
::::

::::explain{#may-dem-chu-khong-doc}
Câu hỏi cuối bài trước: docstring đã ghi rõ hàm cần hai thứ, vậy đưa một thứ
thôi thì máy đọc nhãn rồi tự bù chứ?

Không. Máy không đọc nhãn.

Bài trước đã nói ra chuyện này rồi, chỉ là chưa dùng tới: `help` **cất lại
nguyên văn rồi đưa lại nguyên văn**. Cất và đưa, hết. Với máy, câu *"không ghi
giá thì cứ tính 45000 một tô"* nằm trong docstring cũng chỉ là một dãy ký tự,
y như câu *"Phở bò tái nạm"* bạn viết ở Realm 0. Nó không phải một lời hứa mà
máy giữ.

Thứ máy thật sự đọc vẫn là đúng cái dòng cũ:

```python title=readonly
def tinh_tien(so_to, gia):
```

Trong ngoặc có **hai** chỗ trống. Chuyện đó không phải lời khuyên, nó là con
số: hai. Lúc bạn gọi `tinh_tien(...)`, máy đếm những thứ bạn đưa vào — mỗi thứ
gọi là một **đối số**, tên bạn đã gặp ở Realm 0 — rồi so với số chỗ trống.

Khớp thì máy điền vào và chạy. Không khớp thì máy dừng lại ngay ở cửa, không
bước vào thân hàm lấy một dòng.

Bà chủ đưa cho bạn cái khay hai lỗ: một lỗ đặt tô, một lỗ đặt bát nước mắm.
Bạn đưa lại cái khay với đúng một tô, không có bát nào — bà không tự múc thêm
một bát cho đủ, và cũng không bưng cái khay thiếu ấy ra bàn. Bà đứng lại và
nói ra chỗ thiếu. Đưa ba thứ cho khay hai lỗ thì cũng vậy: cái thứ ba không có
chỗ nào để đặt.
::::

::::example{#hai-cach-lech}
Cùng một hàm, hai lời gọi lệch, hai thông báo khác nhau.

**Đưa thiếu:**

```python title=readonly
def tinh_tien(so_to, gia):
    """Tính tiền một bàn: mấy tô, giá một tô bao nhiêu."""
    return so_to * gia

print(tinh_tien(2))
```

```text
Traceback (most recent call last):
  File "<stdin>", line 5, in <module>
TypeError: tinh_tien() missing 1 required positional argument: 'gia'
```

**Đưa thừa:**

```python title=readonly
print(tinh_tien(2, 45000, 1))
```

```text
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
TypeError: tinh_tien() takes 2 positional arguments but 3 were given
```

Đọc theo đúng lối Realm 0 đã dạy: **từ dòng cuối lên**.

Dòng cuối của ca thứ nhất nói ba điều, và điều thứ ba là món quà: `missing 1
required positional argument` — thiếu một thứ bắt buộc — rồi `'gia'`, tên đúng
cái chỗ trống chưa ai điền. Máy không kể lể chung chung; nó chỉ thẳng vào chỗ
hở.

Dòng cuối của ca thứ hai: `takes 2 positional arguments but 3 were given` —
nhận hai, mà đưa ba. Cũng là hai con số đặt cạnh nhau cho bạn tự so.

Cả hai đều là `TypeError`, đúng loại lỗi bạn gặp ở Realm 0 khi cộng một câu
chữ với một con số. Cùng một ý nghĩa: **hai thứ này không đi với nhau được**.
Lần trước là một chuỗi không đi với một số; lần này là ba đối số không đi với
một cái khay hai lỗ.

Và hãy để ý một chuyện dễ bỏ qua: ở ca đưa thiếu, màn hình **không in ra con số
nào** trước dòng lỗi. Máy chưa hề bước vào thân hàm. Nó đếm ở cửa, thấy lệch,
và dừng ngay tại lời gọi.
::::

::::predict{#loi-hua-trong-nhan commitOnce}
Byte đọc lại nhãn của mình và thấy nó đã viết sẵn một lối thoát cho người quên
ghi giá.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
def tinh_tien(so_to, gia):
    """Tính tiền một bàn.

    so_to: mấy tô.
    gia: giá một tô, tính bằng đồng.
    Không ghi giá thì cứ tính theo giá 45000 một tô.
    """
    return so_to * gia

print(tinh_tien(2))
```

:::opt{correct}
Máy dừng và báo `TypeError`, nói thiếu đối số `gia`
:::

:::opt
In ra 90000, vì nhãn đã nói rõ không ghi giá thì tính 45000 một tô
::why
Gần đúng ở chỗ rất đáng thông cảm: bài trước bạn vừa thấy máy **đọc lại** đúng
câu ấy ra màn hình khi bạn gõ `help`. Máy đọc được thì máy hiểu — suy luận đó
tự nhiên, và nó đúng với con người.

Chỗ lệch nằm ở chữ *đọc*. Máy chép câu ấy ra, y như chép một câu chữ bất kỳ,
chứ không hiểu nghĩa một chữ nào trong đó. Câu bạn viết trong docstring là lời
hứa của **bạn** với người đọc, không phải lời hứa của máy.

Muốn máy thật sự tự điền giá khi người ta bỏ trống thì phải nói với nó bằng
chỗ khác — không phải bằng docstring. Cái chỗ ấy có thật, và bạn sẽ gặp nó ở
phía trước.
::
:::

:::opt
Máy tính với `gia` để trống, ra kết quả 0
::why
Gần đúng ở chỗ bạn hình dung một cái khay có lỗ nào trống thì coi như lỗ ấy
đựng số không — nhiều bảng biểu ngoài đời làm đúng như vậy, và có những ngôn
ngữ lập trình cũng chọn cách đó thật.

Chỗ lệch: Python không tự điền gì vào chỗ trống. Với nó, "chưa ai điền" khác
hẳn "điền số 0" — số 0 là một giá trị đàng hoàng mà ai đó đã cố ý đưa vào. Lẫn
hai thứ ấy vào nhau nghĩa là mọi lời gọi thiếu sót đều lặng lẽ cho ra một con
số trông như thật, và đó mới là chuyện đáng sợ.
::
:::

:::opt
Máy báo `NameError`, vì `gia` chưa mang giá trị nào
::why
Gần đúng ở một chỗ tinh: bạn nhớ Realm 0 rất chắc — một cái tên không có giá
trị thì máy đi tìm không ra và báo `NameError`.

Chỗ lệch là ai đang tìm cái tên ấy. `gia` không phải cái tên máy đi tìm ở
ngoài; nó là một chỗ trống trong chữ ký, và người có nhiệm vụ điền vào là
**người gọi**. Nên chuyện xảy ra ở đây không phải "tìm không thấy tên" mà là
"lời gọi không khớp với hàm" — hai thứ không đi với nhau được, tức `TypeError`.
Máy dừng ngay tại lời gọi, trước khi có ai kịp đi tìm `gia`.
::
:::
::::

::::explain{#lech-la-dung-ngay}
Gom lại thành một câu dùng được: **số đối số bạn đưa phải bằng đúng số tham số
trong chữ ký**. Thiếu một cũng dừng, thừa một cũng dừng, và cả hai đều dừng
bằng `TypeError`.

Nghe như một luật khó tính. Thật ra đây là chỗ hàm tử tế với bạn nhất.

Một lời gọi lệch mà máy vẫn cho chạy thì sai lầm đi tiếp: hàm chạy với một chỗ
trống rỗng, trả về một con số trông bình thường, con số ấy cộng vào hoá đơn, và
tới lúc khách thắc mắc thì chẳng còn dấu vết nào chỉ về dòng đã gây ra. Dừng
ngay tại lời gọi nghĩa là chỗ hỏng và chỗ báo là **cùng một dòng**.

Có một chỗ dễ vấp: đếm chỗ trống thì dễ, đếm đối số lại hay nhầm khi bạn gọi
một hàm nằm trong một hàm khác. `tinh_tien(gia_to("vừa"))` trông như hai thứ vì
có hai cặp ngoặc, nhưng `tinh_tien` chỉ nhận **một** đối số — cái nằm giữa cặp
ngoặc ngoài cùng. Cách đếm chắc chắn: nhìn đúng cặp ngoặc của hàm đang gọi, rồi
đếm những dấu phẩy ngăn cách bên trong cặp ấy.
::::

::::code{#dua-du-hai-thu}
Bàn ba gọi hai tô thường, giá 45000 một tô — dòng ấy đã viết xong.

Bàn năm gọi **ba tô đặc biệt**, giá **60000** một tô. Byte gõ lời gọi cho bàn
năm mà chưa điền gì vào trong ngoặc.

Hãy điền vào chỗ trống, đủ những thứ mà chữ ký đang đòi, đúng những thứ ấy.

```python title=starter
def tinh_tien(so_to, gia):
    """Tính tiền một bàn.

    so_to: mấy tô.
    gia: giá một tô, tính bằng đồng.
    Trả về tiền của cả bàn.
    """
    return so_to * gia

tien_ban_ba = tinh_tien(2, 45000)
tien_ban_nam = tinh_tien(___)

print(f"Bàn ba: {tien_ban_ba} đồng")
print(f"Bàn năm: {tien_ban_nam} đồng")
```

```python title=solution
def tinh_tien(so_to, gia):
    """Tính tiền một bàn.

    so_to: mấy tô.
    gia: giá một tô, tính bằng đồng.
    Trả về tiền của cả bàn.
    """
    return so_to * gia

tien_ban_ba = tinh_tien(2, 45000)
tien_ban_nam = tinh_tien(3, 60000)

print(f"Bàn ba: {tien_ban_ba} đồng")
print(f"Bàn năm: {tien_ban_nam} đồng")
```

```python title=test
# Điền thiếu hay điền thừa thì chương trình dừng ngay ở lời gọi và không dòng
# nào dưới đây chạy được — đó chính là điều bài này dạy, và nó cũng là cách
# chấm. Hai phép kiểm dưới lo phần còn lại: đưa ĐÚNG hai thứ mà bàn năm gọi.
assert tien_ban_ba == 90000, "hai tô giá 45 nghìn là 90 nghìn — dòng của bàn ba đã viết sẵn, không nên sửa nó"
assert tien_ban_nam == 180000, "bàn năm gọi ba tô đặc biệt giá 60 nghìn một tô, nên tiền của bàn năm phải là 180 nghìn"
```

:::hints
- kind: attention
  body: Nhìn lại dòng `def` xem trong ngoặc có mấy chỗ trống, rồi nhìn dòng đã viết sẵn của bàn ba xem một lời gọi đủ trông ra sao. Đơn của bàn năm nằm ngay trong đề bài: mấy tô, và giá một tô.
- kind: strategy
  body: Hai thứ đặt trong ngoặc, ngăn nhau bằng một dấu phẩy. Thứ thứ nhất đi vào chỗ trống thứ nhất, thứ thứ hai đi vào chỗ trống thứ hai — chữ ký đọc từ trái sang phải là `so_to` rồi tới `gia`.
- kind: one-line
  body: "Viết `3, 60000` vào chỗ trống, giữ nguyên hai dấu ngoặc đang có."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^Bàn ba: 90000 đồng\nBàn năm: 180000 đồng\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai lỗ trên khay, hai thứ đặt vào. Mình đếm xong là bưng đi được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Khớp về **số lượng** rồi: chữ ký đòi hai thứ, bạn đưa hai thứ, máy cho qua cửa.

Nhưng nếu đưa đủ hai thứ mà đảo chỗ cho nhau — gõ `tinh_tien(60000, 3)` thay vì
`tinh_tien(3, 60000)` — máy có phát hiện ra không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
