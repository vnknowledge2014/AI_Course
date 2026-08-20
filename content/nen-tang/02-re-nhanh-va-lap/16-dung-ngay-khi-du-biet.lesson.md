---
id: nen-tang.re-nhanh-va-lap.dung-ngay-khi-du-biet
title: Dừng ngay khi đã đủ biết
summary: Một lệnh cắt ngang vòng lặp ngay tại chỗ nó đứng, không đợi hết lượt và không đợi điều kiện sai.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ctrl.break]
requires: [ctrl.for-each, ctrl.while, ctrl.if]
concepts: [ctrl.lap, ctrl.re-nhanh]
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
Biết đủ rồi thì mình gấp sổ. Đọc nốt cho hết chỉ tốn công thôi.
::::

::::explain{#gap-so-lai}
Bài trước để lại một câu hỏi. Tìm ngày tiêu nhiều nhất thì buộc phải xem hết 30
ngày — bỏ sót một ngày là có thể bỏ sót đúng ngày kỷ lục. Nhưng câu hỏi

> Cả tháng có ngày nào tiêu quá 500 nghìn không?

thì khác hẳn: thấy **một** ngày như vậy là đã trả lời xong. Ngày thứ tư tiêu 620
nghìn thì 26 ngày còn lại có tiêu bao nhiêu cũng không đổi được câu trả lời nữa.

Bà chủ quán dò sổ đúng theo kiểu đó. Lật từng trang, tới trang nào thấy con số
vượt 500 nghìn là **gấp sổ lại**, không đọc tiếp.

Vòng lặp bạn viết tới giờ chưa biết gấp sổ. Nó đi hết danh sách, hoặc chạy tới
khi điều kiện `while` thành sai — hai lối ra ấy đều nằm ở **đầu lượt**, và cả
hai đều bắt máy đi cho hết đường.

Python có một từ để nói "gấp sổ ngay tại đây": **`break`**. Tiếng Anh nghĩa là
*cắt ngang*, và nó cắt ngang đúng nghĩa đen.
::::

::::example{#break-dau-tien}
Sổ chi tiêu sáu ngày đầu tháng. Byte đi tìm ngày đầu tiên vượt 500 nghìn:

```python title=readonly
chi_tieu = [120000, 95000, 340000, 620000, 80000, 510000]
ngay = 1

for tien in chi_tieu:
    print(f"Đang xem ngày {ngay}")
    if tien > 500000:
        print(f"Ngày {ngay} tiêu {tien} đồng — quá 500 nghìn")
        break
    ngay = ngay + 1

print("Gấp sổ lại")
```

Màn hình hiện ra:

```text title=readonly
Đang xem ngày 1
Đang xem ngày 2
Đang xem ngày 3
Đang xem ngày 4
Ngày 4 tiêu 620000 đồng — quá 500 nghìn
Gấp sổ lại
```

Danh sách có sáu con số, mà dòng *Đang xem* chỉ hiện bốn lần. Ngày 5 và ngày 6
không hề được đọc tới.

Nhìn kỹ chuyện xảy ra ở lượt thứ tư:

- `tien` đang giữ `620000`. Điều kiện `tien > 500000` đúng, nên máy vào thân `if`.
- Dòng `print` trong nhánh chạy — nó đứng **trước** `break` nên vẫn được chạy.
- Tới `break`, máy rời khỏi vòng lặp **ngay tại dòng đó**.
- Dòng `ngay = ngay + 1` nằm dưới, thuộc phần còn lại của lượt thứ tư. Nó
  **không** chạy. Đó là lý do con số in ra là `4` chứ không phải `5`.
- `print("Gấp sổ lại")` viết sát lề trái, nằm ngoài vòng lặp, nên nó là dòng
  đầu tiên máy chạy sau khi thoát.

Gọn lại trong một câu: `break` đưa máy nhảy thẳng tới dòng đầu tiên **nằm sau**
vòng lặp, bỏ luôn phần còn lại của lượt đang dở.
::::

::::predict{#doan-luot-nao-dung commitOnce}
Lần này là vòng `while`, và điều kiện của nó cho phép chạy tới 5 lượt. **Trước
khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python title=readonly
lan = 0

while lan < 5:
    lan = lan + 1
    print(f"Lượt {lan}")
    if lan == 3:
        break
    print("xong lượt")

print("Ra khỏi vòng")
```

:::opt{correct}
Lượt 1, xong lượt, Lượt 2, xong lượt, Lượt 3, rồi Ra khỏi vòng
:::

:::opt
Lượt 1, xong lượt, Lượt 2, xong lượt, Lượt 3, xong lượt, rồi Ra khỏi vòng
::why
Gần đúng gần hết bài: bạn đếm đúng ba lượt, và bạn nhớ rằng dòng sát lề trái
vẫn chạy sau khi vòng lặp kết thúc. Chỉ lệch đúng một dòng.

Chỗ lệch là chữ *xong lượt* ở lượt thứ ba. Cách đọc của bạn đang coi `break`
như một lời dặn *"lượt này là lượt cuối"* — máy nghe xong rồi vẫn làm nốt lượt
cho trọn. Nhưng `break` không hẹn, nó cắt: tới dòng đó là máy đi ra, mọi dòng
bên dưới trong cùng lượt đều bị bỏ lại. `print("xong lượt")` nằm bên dưới, nên
lượt ba không in nó.
::
:::

:::opt
Năm lượt đầy đủ, vì điều kiện `lan < 5` mới là thứ quyết định lúc nào dừng
::why
Gần đúng ở một điều bạn học rất kỹ ở bài "Điều kiện được xem lại lúc nào":
vòng `while` xem lại điều kiện ở đầu mỗi lượt, và `lan < 5` đúng là cho phép
chạy tới lượt thứ năm.

Chỗ lệch là bài này thêm cho vòng lặp một lối ra **thứ hai**. Điều kiện ở đầu
vẫn còn nguyên tác dụng, nhưng `break` không cần hỏi nó: gặp `break` là máy ra
khỏi vòng, kể cả khi điều kiện vẫn đang đúng. Ở lượt ba, `lan` là 3 và `3 < 5`
vẫn đúng — vòng lặp vẫn kết thúc.
::
:::

:::opt
Lượt 1, xong lượt, Lượt 2, xong lượt, Lượt 3 — rồi hết, không có dòng nào nữa
::why
Gần đúng ở toàn bộ phần trong vòng lặp: ba lượt, và lượt ba dừng ngay sau khi
in *Lượt 3*. Bạn đọc `break` chính xác.

Chỗ lệch nằm ở phạm vi của chữ "dừng". `break` dừng **vòng lặp**, không dừng
chương trình. Máy vẫn đi tiếp từ trên xuống như mọi bài trước, và dòng đầu tiên
nó gặp sau vòng lặp là `print("Ra khỏi vòng")`.
::
:::
::::

::::explain{#break-nam-o-dau}
Ba điều đáng nhớ về `break`, gom lại:

- Nó dùng được trong **cả hai** loại vòng lặp — `for` cũng như `while`. Chỗ nào
  có vòng lặp, chỗ đó viết được `break`.
- Nó gần như luôn nằm trong một `if`. Một `break` viết trần trong thân vòng thì
  vòng nào cũng chỉ chạy đúng một lượt — có vòng lặp cũng như không.
- Nhưng `break` **không thuộc về** cái `if` ấy. Cái `if` chỉ quyết định *có
  chạy tới dòng `break` hay không*. Còn chỗ `break` đưa máy tới thì luôn là dòng
  ngay sau vòng lặp bao quanh nó.

Viết `break` ở nơi không có vòng lặp nào bao quanh thì máy từ chối ngay trước
khi chạy, bằng loại lỗi bạn đã gặp từ Realm 0:

```text title=readonly
SyntaxError: 'break' outside loop
```

Dịch sát: *`break` nằm ngoài vòng lặp*. Gặp dòng này, hãy nhìn xem `break` của
bạn có thật sự thụt vào bên trong thân một `for` hay `while` không.
::::

::::code{#tim-ngay-vuot-nguong}
Sổ chi tiêu tuần này đã có sẵn. Bà chủ chỉ cần biết **ngày đầu tiên** tiêu quá
500 nghìn, biết rồi thì thôi, không dò tiếp.

Đoạn dưới đã in đúng câu cần in. Còn thiếu dòng bảo máy gấp sổ lại ngay sau đó.

```python title=starter
chi_tieu = [120000, 95000, 340000, 620000, 80000, 510000]
ngay = 1

for tien in chi_tieu:
    if tien > 500000:
        print(f"Ngày {ngay} tiêu {tien} đồng")
        ___
    ngay = ngay + 1

print("Xem tới đây thôi")
```

```python title=solution
chi_tieu = [120000, 95000, 340000, 620000, 80000, 510000]
ngay = 1

for tien in chi_tieu:
    if tien > 500000:
        print(f"Ngày {ngay} tiêu {tien} đồng")
        break
    ngay = ngay + 1

print("Xem tới đây thôi")
```

```python title=test
# Vòng lặp phải thoát ngay ở ngày 4, trước khi dòng đếm ngày kịp chạy.
# Nếu thiếu lệnh cắt ngang, `ngay` sẽ chạy tiếp và không còn bằng 4.
assert ngay == 4
```

:::hints
- kind: attention
  body: Chỗ trống nằm trong nhánh đúng của `if`, ngay dưới câu `print`. Tới được dòng đó nghĩa là máy đã tìm thấy thứ cần tìm — và không còn việc gì để làm với những ngày sau nữa.
- kind: strategy
  body: Bạn cần đúng một từ khoá, viết một mình trên một dòng, thụt vào cùng mức với dòng `print` phía trên. Nó là từ tiếng Anh nghĩa là "cắt ngang".
- kind: one-line
  body: "Viết `break` vào chỗ trống, thụt vào đúng bằng dòng `print` ngay trên nó."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Ngày 4 tiêu 620000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu ngày trong sổ, mình chỉ đọc bốn. Câu trả lời vẫn y nguyên.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vòng lặp trong bài này có tới hai lối ra. Một lối nằm ở **đầu** vòng — điều kiện
`lan < 5`, được xem lại mỗi lượt. Một lối nằm ở **giữa thân** — dòng `break`.

Mà lối ra thật sự, lần nào cũng là `break`. Vòng lặp chưa lần nào ra bằng cửa
trước.

Vậy điều kiện đặt ở đầu `while` còn để làm gì, khi chỗ dừng thật sự nằm ở giữa
thân? Nếu bạn không nghĩ ra điều kiện nào để viết ở đó, phải viết gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
