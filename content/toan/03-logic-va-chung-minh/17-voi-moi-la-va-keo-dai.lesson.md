---
id: toan.logic-va-chung-minh.voi-moi-la-va-keo-dai
title: "Với mọi" là chữ "và" kéo dài
summary: Một câu "với mọi" thu cả cột giá trị Đ/S về một giá trị duy nhất, và nó thu bằng đúng cái luật của chữ "và" — chỉ là nối sáu lần thay vì một.
locale: vi
track: toan
module: logic-va-chung-minh
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.for-all]
requires: [logic.open-sentence, logic.conjunction, logic.and, logic.proposition, logic.truth-value, core.boolean, core.dict, core.list, core.list-append, core.function-def, core.function-parameter, core.function-return, core.function-call, core.builtin-function, core.variable, core.print-variable, ctrl.for-each]
concepts: [logic.voi-moi, logic.va-keo-dai, logic.mien-cua-cau]
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
Sáu giá trị nằm thành một cột. Mà dòng nội quy thì có mỗi một câu.
::::

::::explain{#sau-thu-ve-mot}
Bài trước để lại đúng chỗ này.

Câu mở `___ đeo thẻ` cộng với sáu cái tên cho bạn **sáu** mệnh đề và sáu giá trị:

```text
    Nam đeo thẻ      Đ
    Lan đeo thẻ      Đ
    Minh đeo thẻ     Đ
    Hoa đeo thẻ      S
    Tú đeo thẻ       Đ
    Khanh đeo thẻ    Đ
```

Còn dòng trên bảng nội quy thì viết một câu:

> Mọi thành viên đều đeo thẻ.

Một câu — nên nó mang một giá trị, không mang sáu. Câu hỏi để lại là: nó thu sáu
giá trị kia về thành một bằng cách nào?

Bạn đã có sẵn công cụ để thu. Từ bài 4 tới giờ, cả track chỉ làm mỗi việc ấy: lấy
những giá trị Đ/S rồi ghép chúng thành một giá trị Đ/S mới. Việc còn lại là chọn
đúng chữ nối.
::::

::::explain{#doc-lai-dong-noi-quy}
Đọc dòng nội quy thật chậm, và đọc nó như một lời hứa của cả sáu người cùng lúc.

Thầy phụ trách nói "mọi thành viên đều đeo thẻ". Thầy đang khẳng định chuyện gì?
Thầy khẳng định Nam đeo thẻ. **Và** Lan đeo thẻ. **Và** Minh đeo thẻ. **Và** Hoa
đeo thẻ. **Và** Tú đeo thẻ. **Và** Khanh đeo thẻ.

Viết liền một dòng:

> Nam đeo thẻ **và** Lan đeo thẻ **và** Minh đeo thẻ **và** Hoa đeo thẻ **và**
> Tú đeo thẻ **và** Khanh đeo thẻ.

Câu dài này và dòng nội quy nói **cùng một chuyện**. Dòng nội quy chỉ là cách viết
gọn của nó — gọn hơn hẳn, và không phải chép lại sáu cái tên.

Nên chữ nối đã lộ ra: chữ **"và"**, nối năm lần để buộc sáu vế lại.

Bài 4 đã chốt luật của chữ "và" cho hai vế: câu ghép đúng khi **cả hai** vế cùng
đúng, và một vế sai là cả câu ghép sai. Nối sáu vế thì luật ấy không đổi gì —
nó chỉ được dùng nhiều lần hơn:

> **Với mọi thành viên: người ấy đeo thẻ** đúng khi **tất cả** sáu mệnh đề con
> cùng đúng. Có một mệnh đề con sai thì cả câu sai.

Câu bắt đầu bằng chữ "với mọi" như thế gọi là một câu có **lượng từ với mọi**.
Nghề toán viết tắt lượng từ ấy bằng ký hiệu **∀**, và đọc ký hiệu ấy đúng là "với
mọi". Bài này dùng chữ tiếng Việt; ký hiệu để đó, biết mặt là đủ.

Và đây là chỗ đáng ghi lại: câu mở của bài trước chưa mang giá trị nào. Đặt chữ
"với mọi" vào trước nó thì chỗ trống được lấp bằng cả một danh sách, và cái thu
được là một **mệnh đề** đầy đủ — có đúng một giá trị, phân xử được.
::::

::::example{#hai-cau-voi-moi}
Đem luật ấy chấm hai câu, dùng lại đúng cuốn sổ ba cột của bài trước.

| thành viên | đeo thẻ (sáng thứ Hai) | đã nộp quỹ tháng này | có mặt buổi họp |
|---|---|---|---|
| Nam | có | rồi | có |
| Lan | có | rồi | có |
| Minh | có | **chưa** | có |
| Hoa | **không** | rồi | có |
| Tú | có | rồi | có |
| Khanh | có | **chưa** | có |

**Câu thứ nhất — "Với mọi thành viên: người ấy có mặt buổi họp."**

Cột "có mặt buổi họp" ghi "có" ở cả sáu dòng, nên sáu mệnh đề con đều đúng. Chuỗi
"và" sáu vế không hụt vế nào, nên câu này mang giá trị **Đ**.

**Câu thứ hai — "Với mọi thành viên: người ấy đeo thẻ."**

Chuỗi "và" sáu vế: Đ, Đ, Đ, **S**, Đ, Đ. Chữ "và" đòi cả hai vế mỗi lần nó xuất
hiện, nên chỉ cần chuỗi hụt ở một mắt là cả chuỗi hụt. Câu này mang giá trị **S**.

Để ý một chuyện: câu thứ hai sai, mà năm người trong sáu người vẫn đeo thẻ đầy đủ.
Chữ "và" không đếm phiếu và không chia phần trăm — bài 4 đã dựng nó như thế ngay
từ đầu, và ở đây nó vẫn xử y hệt.

**Còn một chỗ nữa phải nói rõ: mọi *ai*?**

Cả hai câu trên đều chạy trên danh sách sáu thành viên CLB. Nhưng CLB còn chia
**tổ cờ nhanh** gồm ba người: Nam, Lan, Minh. Đem đúng câu mở `___ đeo thẻ` chạy
trên riêng tổ ấy:

> Với mọi người trong tổ cờ nhanh: người ấy đeo thẻ.

Chuỗi "và" giờ chỉ có ba vế — Đ, Đ, Đ — nên câu này mang giá trị **Đ**.

Cùng một câu mở, cùng một cuốn sổ, hai giá trị khác nhau. Không có gì mâu thuẫn ở
đây: đó là **hai câu khác nhau**, vì hai chuỗi "và" ấy nối những vế khác nhau. Nên
một câu "với mọi" luôn phải nói rõ nó chạy trên danh sách nào; bỏ lửng chỗ ấy thì
chưa có câu nào để phân xử.
::::

::::predict{#doan-all commitOnce}
Python có sẵn một chữ để nối "và" bao nhiêu lần cũng được: `all`. Bỏ vào cho nó
một danh sách giá trị Đ/S, nó trả về `True` khi **tất cả** đều là `True`.

Byte gõ ba dòng, cùng một chuỗi giá trị viết theo hai cách, rồi thêm một chuỗi
ngắn hơn.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(True and True and True and False and True and True)
print(all([True, True, True, False, True, True]))
print(all([True, True, True]))
```

:::opt{correct}
`False`, rồi `False`, rồi `True`
:::

:::opt
`False`, rồi `True`, rồi `True`
::why
Gần đúng ở chỗ bạn đọc trúng dòng đầu — chuỗi `and` sáu vế có một vế `False` nên
cả chuỗi `False`, không bàn cãi — và ở chỗ bạn nhận ra dòng hai và dòng ba là hai
câu hỏi khác nhau chứ không phải một.

Chỗ lệch nằm ở việc `all` cân đo theo kiểu gì. Nó không đếm xem bên nào nhiều
hơn: năm `True` với một `False` không làm nó nghiêng về `True`. Nó làm đúng việc
mà chuỗi `and` ở dòng trên vừa làm — hễ vấp một `False` là cả chuỗi thành `False`.
Nên dòng một và dòng hai in ra cùng một thứ, và đó chính là chỗ bài này nói tới:
`all` là chữ "và" kéo dài, không phải một phép bỏ phiếu.
::
:::

:::opt
`False`, rồi `False`, rồi `False`
::why
Gần đúng ở chỗ bạn đọc trúng hai dòng đầu, và ở chỗ bạn giữ đúng một thói quen
rất tốt: nghe "với mọi" thì nghĩ ngay tới cả CLB sáu người, không để ai rơi ra
ngoài.

Chỗ lệch: `all` chỉ nhìn thấy đúng những gì nằm trong cái danh sách bạn đưa cho
nó. Danh sách ở dòng ba có ba giá trị, cả ba đều `True`, nên chuỗi "và" ba vế
không hụt chỗ nào và nó trả `True`. Nó không biết CLB có sáu người và cũng không
đi tìm ba người còn lại.

Đây đúng là chỗ mà phần "mọi *ai*?" ở trên vừa dựng lên: câu "với mọi" chạy trên
tổ cờ nhanh và câu "với mọi" chạy trên cả CLB là **hai câu khác nhau**, nên chúng
được phép mang hai giá trị khác nhau.
::
:::

:::opt
Máy báo lỗi ở dòng đầu, vì `and` chỉ nối được hai vế
::why
Gần đúng ở chỗ bạn nhớ đúng nơi mình gặp `and` lần đầu: Realm 0 dựng nó trên hai
điều kiện, và bảng chân lý của bài 4 cũng đúng hai cột vế. Từ đó mà nghĩ `and`
làm việc theo cặp là một suy luận thẳng thớm.

Chỗ lệch: `and` nối được bao nhiêu vế tuỳ ý, và nó nối bằng cách làm đi làm lại
đúng cái luật của một cặp — ghép hai vế đầu thành một giá trị, rồi ghép giá trị
ấy với vế thứ ba, cứ thế. Bài 6 đã cho bạn dựng bảng cho câu ghép ba vế, nên
chuyện "nhiều hơn hai vế" không mới. Máy chạy dòng ấy bình thường và in ra một
giá trị Đ/S.
::
:::
::::

::::code{#thu-sau-gia-tri-ve-mot}
Giờ bắt máy làm đủ hai việc: **dựng cột sáu giá trị**, rồi **thu cột ấy về một
giá trị**.

Bạn viết ba chỗ trống:

1. Trong thân `bang_dung_sai` — điền từng cái tên vào chỗ trống của câu mở
   `___ đeo thẻ`, tức là tra cuốn sổ `deo_the` theo cái tên đang cầm.
2. `ca_clb` — giá trị của câu "với mọi thành viên CLB: người ấy đeo thẻ", tức là
   chuỗi "và" nối **sáu** vế.
3. `ca_to` — giá trị của câu "với mọi người trong tổ cờ nhanh: người ấy đeo thẻ",
   tức là chuỗi "và" nối **ba** vế. Dùng lại đúng cái danh sách `to_co_nhanh` đã
   có sẵn ở đầu khung.

Bài chấm bằng **hai danh sách chạy trên cùng một cuốn sổ**, và hai danh sách ấy
được chọn để cho hai câu trả lời ngược nhau: cả CLB thì có Hoa nên chuỗi hụt một
mắt, riêng tổ cờ nhanh thì không hụt mắt nào. Gõ cứng một giá trị Đ/S vào hai chỗ
cuối thì hai dòng ấy giống hệt nhau và hỏng ngay.

```python title=starter
thanh_vien = ["Nam", "Lan", "Minh", "Hoa", "Tú", "Khanh"]
to_co_nhanh = ["Nam", "Lan", "Minh"]
deo_the = {"Nam": True, "Lan": True, "Minh": True,
           "Hoa": False, "Tú": True, "Khanh": True}

# Điền từng cái tên trong danh sách vào chỗ trống của câu mở "___ đeo thẻ",
# thu lại đúng bấy nhiêu giá trị Đ/S.
def bang_dung_sai(danh_sach):
    ra = []
    for ten in danh_sach:
        ra.append(___)
    return ra

# "Với mọi thành viên CLB: người ấy đeo thẻ" — chữ "và" nối sáu vế.
ca_clb = ___

# "Với mọi người trong tổ cờ nhanh: người ấy đeo thẻ" — chữ "và" nối ba vế.
ca_to = ___

print(bang_dung_sai(thanh_vien))
print(ca_clb)
print(ca_to)
```

```python title=solution
thanh_vien = ["Nam", "Lan", "Minh", "Hoa", "Tú", "Khanh"]
to_co_nhanh = ["Nam", "Lan", "Minh"]
deo_the = {"Nam": True, "Lan": True, "Minh": True,
           "Hoa": False, "Tú": True, "Khanh": True}

# Điền từng cái tên trong danh sách vào chỗ trống của câu mở "___ đeo thẻ",
# thu lại đúng bấy nhiêu giá trị Đ/S.
def bang_dung_sai(danh_sach):
    ra = []
    for ten in danh_sach:
        ra.append(deo_the[ten])
    return ra

# "Với mọi thành viên CLB: người ấy đeo thẻ" — chữ "và" nối sáu vế.
ca_clb = all(bang_dung_sai(thanh_vien))

# "Với mọi người trong tổ cờ nhanh: người ấy đeo thẻ" — chữ "và" nối ba vế.
ca_to = all(bang_dung_sai(to_co_nhanh))

print(bang_dung_sai(thanh_vien))
print(ca_clb)
print(ca_to)
```

```python title=test
# Câu `!=` đứng ĐẦU. Nó canh cái bẫy lớn nhất của bài: chép cùng một dòng vào
# cả hai chỗ trống cuối, hoặc gõ cứng một giá trị Đ/S vào cả hai. Xếp nó xuống
# dưới các câu `==` thì một câu `==` trượt trước và bẫy không bao giờ sập.
assert ca_clb != ca_to, "cả CLB có Hoa quên thẻ nên chuỗi 'và' hụt một mắt, còn tổ cờ nhanh thì không hụt mắt nào — hai câu ấy không thể cùng một giá trị"
assert bang_dung_sai(thanh_vien) == [True, True, True, False, True, True], "cột giá trị của câu mở `___ đeo thẻ` trên sáu thành viên: chỉ riêng ô của Hoa là S, năm ô còn lại là Đ"
assert bang_dung_sai(to_co_nhanh) == [True, True, True], "tổ cờ nhanh có ba người, cả ba đều đeo thẻ sáng thứ Hai, nên cột này có đúng ba ô và cả ba đều Đ"
assert ca_clb is False, "chuỗi 'và' sáu vế hụt đúng một vế — vế của Hoa — nên câu 'với mọi thành viên CLB: người ấy đeo thẻ' mang giá trị S"
assert ca_to is True, "ba vế của tổ cờ nhanh đều Đ, nên chuỗi 'và' ba vế không hụt chỗ nào và câu ấy mang giá trị Đ"
# Năm Đ trên sáu vẫn là S. Câu này tồn tại để phân biệt chữ "và" với một phép
# đếm phiếu: nếu ai đó thu cột về bằng "quá nửa số ô là Đ" thì danh sách năm
# người dưới đây cho Đ, trong khi luật của chữ "và" cho S.
assert all(bang_dung_sai(["Nam", "Lan", "Hoa", "Tú", "Khanh"])) is False, "trong năm người này chỉ mình Hoa không đeo thẻ, nhưng chữ 'và' hụt một vế là hụt cả chuỗi, nên kết quả vẫn là S"
assert all(bang_dung_sai(["Hoa"])) is False, "danh sách chỉ có mỗi Hoa: chuỗi 'và' đúng một vế, và vế ấy sai, nên cả chuỗi sai"
assert all(bang_dung_sai(["Tú", "Khanh"])) is True, "Tú và Khanh đều đeo thẻ sáng thứ Hai, nên chuỗi 'và' hai vế này không hụt vế nào"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm trong ngoặc của `ra.append`, và nó phải nói về đúng người đang cầm trong tay ở lượt này — tên người ấy là `ten`, còn cuốn sổ tra thẻ tên là `deo_the`. Hai chỗ trống sau thì nhìn chú thích ngay trên mỗi dòng: chú thích ghi rõ chuỗi "và" ấy nối mấy vế, và số vế đó đến từ danh sách nào trong hai danh sách có sẵn ở đầu khung.
- kind: strategy
  body: "Tra sổ `dict` theo một cái tên là chuyện Realm 1 đã dựng: tên cuốn sổ, rồi cặp ngoặc vuông, rồi cái cần tra. Hai chỗ sau thì làm hai việc nối tiếp nhau — trước hết dựng cột giá trị bằng chính hàm `bang_dung_sai` vừa viết xong, sau đó thu cột ấy về một giá trị bằng chữ nối \"và\" kéo dài mà phần đoán trước vừa dùng. Hai dòng ấy chỉ khác nhau ở chỗ đưa danh sách nào cho `bang_dung_sai`."
- kind: one-line
  body: "Ba chỗ lần lượt là `deo_the[ten]`, `all(bang_dung_sai(thanh_vien))` và `all(bang_dung_sai(to_co_nhanh))`."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: cột giá trị phải được DỰNG THẬT từ cuốn sổ, và hai câu "với mọi" phải được thu từ chính cột ấy — gõ cứng một giá trị Đ/S thì chuyện "sáu giá trị thu về một" không được làm ở đâu cả
  requireAst:
  # Cuốn sổ phải được ĐỌC. Dòng `deo_the = {...}` ở đầu khung là gán, không phải
  # đọc, nên khung khởi đầu đọc `deo_the` 0 lần. Luật này một mình chặn mọi đáp
  # án gõ cứng `True`/`False` vào chỗ trống thứ nhất.
  - kind: uses-name, target: deo_the, min: 1
  # Chỗ trống thứ nhất phải nói về người đang cầm trong tay. Khung khởi đầu đọc
  # `ten` 0 lần (`for ten in ...` là gán, không phải đọc).
  - kind: uses-name, target: ten, min: 1
  # Hai chỗ trống cuối phải thu cột về bằng chữ "và" kéo dài. Khung khởi đầu
  # không gọi `all` lần nào.
  - kind: uses-call, target: all, min: 2
  # ... và phải thu từ CỘT THẬT, không phải từ một danh sách gõ tay. Khung khởi
  # đầu gọi `bang_dung_sai` đúng 1 lần (ở dòng `print`), lời giải gọi 3.
  - kind: uses-call, target: bang_dung_sai, min: 3
  # Hai câu "với mọi" phải chạy trên HAI danh sách khác nhau — đó là chỗ cả bài
  # dựng lên để nói tới. Khung khởi đầu đọc `thanh_vien` 1 lần và `to_co_nhanh`
  # 0 lần; lời giải đọc 2 và 1.
  - kind: uses-name, target: thanh_vien, min: 2
  - kind: uses-name, target: to_co_nhanh, min: 1
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^\[True, True, True, False, True, True\]\nFalse\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu ô thu về một chữ. Và cái chữ nối thì vẫn đúng cái chữ "và" của bài 4.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hoá ra "với mọi" chẳng phải công cụ mới. Nó là chữ "và" của bài 4, nối nhiều lần
hơn, gói lại thành một dòng ngắn.

Nhưng bài 4 không đứng một mình. Ngay sau nó, bài 5 dựng chữ nối thứ hai —
**"hoặc"** — và chốt rằng "hoặc" của logic đúng khi có **ít nhất một** vế đúng,
kể cả khi cả hai cùng đúng.

Vậy thì chữ "hoặc" cũng kéo dài được. Nối sáu vế bằng "hoặc":

> Nam chưa nộp quỹ **hoặc** Lan chưa nộp quỹ **hoặc** Minh chưa nộp quỹ **hoặc**
> Hoa chưa nộp quỹ **hoặc** Tú chưa nộp quỹ **hoặc** Khanh chưa nộp quỹ.

Chuỗi "và" kéo dài gói lại được thành ba chữ ngắn gọn: "mọi thành viên đều...".

Còn chuỗi "hoặc" kéo dài kia thì gói lại thành câu tiếng Việt nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
