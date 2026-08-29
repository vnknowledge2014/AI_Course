---
id: nen-tang.re-nhanh-va-lap.khi-khong-tim-thay-gi
title: Khi chẳng tìm thấy gì
summary: Đặt sẵn một giá trị mang nghĩa "chưa có" trước vòng, rồi kiểm nó sau vòng để biết mình có tìm được gì không.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ctrl.sentinel-value]
requires: [ctrl.break, ctrl.for-each, core.reassign, ctrl.else]
concepts: [ctrl.tim-kiem, core.gia-tri-canh]
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
Tìm được thì bạn biết. Còn tìm không ra, làm sao mình nói cho bạn biết?
::::

::::explain{#o-tra-loi-con-trong}
Bài trước bạn thay lá cờ `True/False` bằng một biến giữ **chính** thứ tìm được:

```python title=readonly
ngay_tim_duoc = ...
```

Và bài trước để lại đúng một câu hỏi: chỗ ba chấm ấy điền gì? Nếu cả tháng không
ngày nào vượt ngưỡng thì dòng in ra cuối cùng sẽ nói gì?

Nghĩ về phòng bảo vệ một chung cư. Đầu ca, người trực mở sổ và kẻ sẵn một dấu
gạch `—` vào mọi ô của cột *Giờ khách đến*. Trong ca, có khách nào tới thì gạch
đi, ghi giờ vào. Cuối ca, người ca sau lật sổ ra: ô nào **vẫn còn dấu gạch**
nghĩa là ô đó không có ai.

Dấu gạch ấy làm được việc vì hai lý do, và cả hai đều quan trọng:

- Nó **có mặt sẵn từ đầu**. Ô trống trơn thì người ca sau không biết là chưa ai
  tới hay là người trực quên ghi.
- Nó **không thể bị nhầm với một câu trả lời thật**. Không có giờ nào trong ngày
  viết ra thành dấu gạch.

Trong code, cái dấu gạch đó là một giá trị bạn tự chọn. Nó có tên: **giá trị
canh** — tiếng Anh gọi là *sentinel value*, nghĩa đen là "người lính gác": nó
đứng giữ chỗ cho tới khi có câu trả lời thật tới thay.
::::

::::example{#dau-gach-la-so-am-mot}
Sổ chi tiêu bảy ngày của Byte, tính bằng nghìn đồng. Câu hỏi: **ngày đầu tiên
tiêu quá 200 là ngày thứ mấy?**

```python title=readonly
chi_tieu = [120, 250, 90, 310, 150, 80, 240]
ngay = 0
ngay_vuot = -1

for tien in chi_tieu:
    ngay += 1
    if tien > 200:
        ngay_vuot = ngay
        break

print(ngay_vuot)
```

Máy in ra:

```text title=readonly
2
```

Ba dòng trước vòng lặp, đọc lần lượt:

- `chi_tieu` — sổ, mỗi ô một ngày.
- `ngay = 0` — cái đếm lượt bạn đã dùng ở bài đếm những lượt đáng kể. Mỗi lượt
  nó cộng thêm 1, nên trong thân vòng nó luôn cho biết đang ở ngày thứ mấy.
- `ngay_vuot = -1` — **dấu gạch**. Đây là dòng mới của bài này.

Vì sao lại là `-1`? Vì ngày trong tháng luôn là 1, 2, 3… trở lên. Không có ngày
âm một. Nên khi bạn nhìn thấy `-1` nằm trong `ngay_vuot`, chỉ có đúng một cách
giải thích: thân `if` chưa lần nào chạy, không ngày nào vượt ngưỡng.

Đổi sổ thành một tuần tiêu dè, không ngày nào quá 200:

```python title=readonly
chi_tieu = [120, 90, 150, 80, 110, 70, 130]
ngay = 0
ngay_vuot = -1

for tien in chi_tieu:
    ngay += 1
    if tien > 200:
        ngay_vuot = ngay
        break

print(ngay_vuot)
```

```text title=readonly
-1
```

Vòng lặp vẫn chạy đủ bảy lượt, `ngay` vẫn đếm tới 7, `break` không lần nào tới
lượt. Và `ngay_vuot` vẫn nguyên vẹn cái giá trị bạn đặt cho nó trước vòng.
::::

::::predict{#doan-so-in-ra commitOnce}
Sổ bốn ngày dưới đây không có ngày nào quá 200. **Trước khi bấm chạy**, bạn đoán
màn hình hiện ra con số nào?

```python title=readonly
chi_tieu = [120, 90, 150, 80]
ngay = 0
ngay_vuot = -1

for tien in chi_tieu:
    ngay += 1
    if tien > 200:
        ngay_vuot = ngay
        break

print(ngay_vuot)
```

:::opt{correct}
-1
:::

:::opt
0
::why
Gần đúng ở chỗ bạn hiểu rất chính xác ý nghĩa của kết quả: không tìm thấy ngày
nào, và `0` đúng là con số người ta hay dùng cho "không có gì". Ý bạn đọc ra
hoàn toàn đúng.

Chỗ lệch nằm ở chỗ ai đặt con số ấy vào. `ngay_vuot` không tự đếm và không tự
về không — nó chỉ đổi giá trị khi có một dòng gán đè lên nó. Trong cả đoạn này
chỉ có hai dòng gán vào `ngay_vuot`: dòng `= -1` trước vòng, và dòng `= ngay`
nằm trong thân `if`. Thân `if` chưa lần nào chạy, nên dòng còn hiệu lực là dòng
đầu.

Muốn thấy `0` thì viết `ngay_vuot = 0` trước vòng — và bài sẽ nói vì sao đó là
lựa chọn nguy hiểm hơn `-1`.
::
:::

:::opt
4
::why
Gần đúng ở chỗ bạn theo dõi vòng lặp rất sát: bốn ô thì bốn lượt, mỗi lượt
`ngay += 1` một lần, nên sau vòng `ngay` đúng bằng 4. Phần suy luận đó chính xác
từng bước.

Chỗ lệch nằm ở cái tên trong `print`. Dòng cuối in `ngay_vuot`, không in `ngay`.
Hai cái tên khác nhau thì giữ hai giá trị khác nhau, dù chúng nhìn giống nhau
trên màn hình.
::
:::

:::opt
Máy dừng lại và báo `NameError`, vì `ngay_vuot` chưa lần nào được gán trong vòng
::why
Gần đúng ở chỗ bạn nhớ đúng luật: đọc một cái tên chưa từng được gán bao giờ thì
máy báo `NameError`. Luật đó có thật và bạn sẽ còn dùng nó nhiều.

Chỗ lệch: `ngay_vuot` **đã** được gán rồi — ở dòng thứ ba, trước khi vòng lặp
bắt đầu. Cái tên đã tồn tại từ lúc đó. Việc thân `if` không chạy chỉ có nghĩa là
không ai gán đè lên nó, chứ không làm nó biến mất.

Và đây chính là lý do dòng `ngay_vuot = -1` phải nằm **trước** vòng: đặt nó
trong thân vòng thì tuần không có ngày nào vượt sẽ cho ra đúng `NameError` mà
bạn vừa nghĩ tới.
::
:::
::::

::::explain{#chon-dau-gach-cho-dung}
Giá trị canh chỉ làm được việc nếu nó **không bao giờ là một câu trả lời thật**.

Ở đây câu trả lời thật là số thứ tự ngày: 1, 2, 3… Nên `-1` an toàn. Còn `0` thì
sao? Ngày 0 không tồn tại, nên `0` cũng dùng được. Nhưng nếu mai kia bạn đổi
sang lưu **chỗ đứng trong danh sách** thay vì số thứ tự ngày, chỗ đứng đầu tiên
lại đúng bằng 0 — và lúc đó `0` mang hai nghĩa cùng lúc: "ô đầu tiên" và "không
có gì". Người đọc code không có cách nào phân biệt.

`-1` không dính vào bẫy đó, nên nó là giá trị canh quen tay nhất của lập trình
viên Python khi đi tìm một vị trí.

Đặt xong giá trị canh mới là nửa việc. Nửa còn lại là **kiểm nó sau vòng**:

```python title=readonly
if ngay_vuot == -1:
    print("Cả tuần không ngày nào tiêu quá 200")
else:
    print(f"Ngày đầu tiên vượt ngưỡng: ngày {ngay_vuot}")
```

Khối `if/else` này nằm sát lề trái, ngoài vòng lặp — nó chạy đúng một lần, sau
khi vòng đã đi xong. Câu hỏi nó đặt ra không phải "tiền có quá 200 không" mà là
"dấu gạch còn nguyên không".
::::

::::code{#doc-lai-o-tra-loi}
Dưới đây là **hai** tuần chi tiêu, mỗi tuần một khối. Hai khối giống nhau từng
chữ một, chỉ khác đúng dãy số trong `chi_tieu`: tuần này Byte tiêu dè cả bảy
ngày; tuần trước cũng dè sáu ngày, tới ngày thứ bảy thì đãi bạn một bữa.

Cùng một chỗ trống, điền hai lần y hệt nhau. Điền đúng thì hai khối tự khắc nói
ra hai chuyện khác nhau — vì cái quyết định câu trả lời là dấu gạch còn hay mất,
chứ không phải câu bạn gõ.

```python title=starter
chi_tieu = [120, 90, 150, 80, 110, 70, 130]
ngay = 0
ngay_vuot = -1

for tien in chi_tieu:
    ngay += 1
    if tien > 200:
        ngay_vuot = ngay
        break

if ___:
    print("Tuần này: cả tuần không ngày nào tiêu quá 200")
else:
    print(f"Tuần này: ngày đầu tiên vượt ngưỡng là ngày {ngay_vuot}")

chi_tieu = [120, 90, 150, 80, 110, 70, 240]
ngay = 0
ngay_vuot = -1

for tien in chi_tieu:
    ngay += 1
    if tien > 200:
        ngay_vuot = ngay
        break

if ___:
    print("Tuần trước: cả tuần không ngày nào tiêu quá 200")
else:
    print(f"Tuần trước: ngày đầu tiên vượt ngưỡng là ngày {ngay_vuot}")
```

```python title=solution
chi_tieu = [120, 90, 150, 80, 110, 70, 130]
ngay = 0
ngay_vuot = -1

for tien in chi_tieu:
    ngay += 1
    if tien > 200:
        ngay_vuot = ngay
        break

if ngay_vuot == -1:
    print("Tuần này: cả tuần không ngày nào tiêu quá 200")
else:
    print(f"Tuần này: ngày đầu tiên vượt ngưỡng là ngày {ngay_vuot}")

chi_tieu = [120, 90, 150, 80, 110, 70, 240]
ngay = 0
ngay_vuot = -1

for tien in chi_tieu:
    ngay += 1
    if tien > 200:
        ngay_vuot = ngay
        break

if ngay_vuot == -1:
    print("Tuần trước: cả tuần không ngày nào tiêu quá 200")
else:
    print(f"Tuần trước: ngày đầu tiên vượt ngưỡng là ngày {ngay_vuot}")
```

```python title=test
# Chấm bằng TRỌN VẸN hai dòng output, không phải một dòng.
#
# Một khối thì chỉ có hai kết cục, nên gõ bừa `True` vào chỗ trống là trúng một
# nửa số lần. Hai khối với hai dãy số khác nhau thì mọi câu gõ bừa đều cho hai
# dòng cùng kiểu — `True` cho hai dòng "cả tuần không ngày nào", `0` cho hai
# dòng "ngày đầu tiên vượt ngưỡng", trong đó tuần tiêu dè hoá ra "ngày -1".
# Chỉ câu hỏi đúng về dấu gạch mới tách được hai tuần ra.
#
# Hai tuần cùng chạy trọn bảy lượt, nên `ngay` bằng 7 ở cả hai — chính vì thế
# `ngay` không phân biệt được tìm thấy với không tìm thấy, chỉ `ngay_vuot` mới
# làm được. Khối này khẳng định vòng lặp thứ hai đã gặp `break` ở ngày 7.
assert ngay == 7, "cả hai tuần đều là sổ bảy ngày, nên đếm tới ngày cuối là đếm tới 7 — con số này giống nhau ở tuần tiêu dè lẫn tuần có ngày vượt, nên một mình nó không nói được là có tìm thấy gì không"
assert ngay_vuot == 7, "tuần trước tiêu dè sáu ngày, tới ngày 7 mới đãi bạn một bữa quá 200 nghìn — ô trả lời bị gạch đi và ghi vào đúng ngày ấy"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm ở cùng một vị trí trong hai khối: giữa `if` và dấu hai chấm, ở khối sát lề trái ngay sau vòng lặp. Nhìn lại dòng thứ ba của mỗi khối — con số đặt vào `ngay_vuot` trước vòng là con số nào?
- kind: strategy
  body: Câu hỏi cần hỏi là "dấu gạch còn nguyên không", tức là "`ngay_vuot` có còn đúng bằng cái giá trị canh đặt trước vòng không". So sánh bằng nhau viết bằng hai dấu bằng liền nhau. Cùng một câu hỏi ấy điền vào cả hai chỗ — hai tuần khác nhau là do dãy số khác nhau, không phải do bạn gõ khác đi.
- kind: one-line
  body: "Viết `ngay_vuot == -1` vào cả hai chỗ trống, giữ nguyên dấu hai chấm ở cuối mỗi dòng."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Tuần này: cả tuần không ngày nào tiêu quá 200\nTuần trước: ngày đầu tiên vượt ngưỡng là ngày 7\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một tuần còn nguyên dấu gạch, một tuần bị gạch đi. Cùng một câu hỏi, hai câu
trả lời — giờ mình nói được cả chuyện không tìm thấy gì.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đếm lại xem cách này bắt bạn nhớ mấy việc. Một: đặt `ngay_vuot = -1` **trước**
vòng. Hai: kiểm `ngay_vuot == -1` **sau** vòng. Quên việc thứ nhất thì được
`NameError`; quên việc thứ hai thì tuần trống trơn vẫn in ra "ngày -1" tỉnh bơ.

Hai việc ấy nằm cách nhau cả một vòng lặp, và con số `-1` phải khớp ở cả hai
chỗ. Đó là ba thứ phải đúng cùng lúc cho một câu hỏi rất đời thường: *đi hết
lượt rồi mà không gặp thì làm gì.*

Chuyện "đi hết lượt mà không gặp" xảy ra với mọi vòng lặp đi tìm. Vậy trong
chính cú pháp vòng lặp của Python, có sẵn một chỗ dành riêng cho nhánh ấy không
— một chỗ không cần bạn tự bịa ra con số canh nào cả?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
