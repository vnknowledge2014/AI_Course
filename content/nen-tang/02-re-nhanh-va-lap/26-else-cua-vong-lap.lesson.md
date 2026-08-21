---
id: nen-tang.re-nhanh-va-lap.else-cua-vong-lap
title: 'Nhánh else của vòng lặp'
summary: Vòng lặp có nhánh else riêng, và nó chỉ chạy khi vòng đi hết lượt mà chưa lần nào gặp break.
locale: vi
track: nen-tang
module: 02-re-nhanh-va-lap
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ctrl.loop-else]
requires: [ctrl.sentinel-value, ctrl.break, ctrl.else, ctrl.for-each]
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
Vòng lặp có một nhánh `else` của riêng nó. Không dính gì tới `if` đâu.
::::

::::explain{#di-het-mot-luot-ma-khong-gap}
Bài trước bạn phải tự bịa ra con số `-1`, đặt nó trước vòng, rồi nhớ kiểm lại
sau vòng. Câu hỏi bỏ ngỏ là: Python có sẵn chỗ nào dành riêng cho nhánh "đi hết
lượt mà không gặp" không?

Có. Và để thấy nó tự nhiên, hãy nhìn cách quán phở làm việc này mỗi tối.

Chủ quán nhờ anh phục vụ: *"Đi một vòng các bàn, thấy bàn nào còn khách thì
dừng lại rót thêm trà rồi thôi."*

Anh phục vụ đi từ bàn 1. Gặp khách ở bàn 5 thì anh rót trà, xong việc, quay vào
bếp. Còn nếu đi hết bàn cuối cùng mà không gặp ai, anh làm một việc **khác
hẳn**: ra báo chủ quán *"hết khách rồi, dọn được chưa ạ"*.

Câu báo ấy có một tính chất đáng để ý: nó chỉ được nói khi anh **đi trọn** cả
vòng. Dừng giữa chừng vì gặp khách thì không có câu báo nào cả.

Python có đúng một chỗ để viết câu báo đó. Nó là từ khoá `else`, đặt ngay sau
vòng lặp — và người ta gọi cấu trúc này là `for ... else` (`while` cũng có
`else` của riêng nó, viết y hệt).
::::

::::example{#for-else-dau-tien}
Vẫn sổ chi tiêu bảy ngày, tính bằng nghìn đồng. Vẫn câu hỏi: ngày đầu tiên tiêu
quá 200 là ngày thứ mấy? Lần này viết không cần giá trị canh:

```python title=readonly
chi_tieu = [120, 90, 150, 80, 110, 70, 130]
ngay = 0

for tien in chi_tieu:
    ngay += 1
    if tien > 200:
        print(f"Ngày đầu tiên vượt ngưỡng: ngày {ngay}")
        break
else:
    print("Cả tuần không ngày nào tiêu quá 200")
```

Máy in ra:

```text title=readonly
Cả tuần không ngày nào tiêu quá 200
```

Nhìn kỹ chỗ đặt chữ `else`. Nó viết **sát lề trái**, thẳng hàng với chữ `for`.

Đây đúng là luật bạn đã học ở bài `else` này của `if` nào: mỗi `else` thuộc về
cái đứng **cùng mức thụt lề** với nó. Ở đây cùng mức thụt lề với `else` là
`for`, nên `else` này là `else` của vòng lặp. Nếu bạn thụt nó vào bốn dấu cách
cho thẳng hàng với `if`, nó sẽ thành `else` của `if` — một cấu trúc khác hẳn,
chạy mỗi lượt một lần.

Đổi sổ thành tuần có ngày tiêu mạnh:

```python title=readonly
chi_tieu = [120, 250, 90, 310, 150, 80, 240]
ngay = 0

for tien in chi_tieu:
    ngay += 1
    if tien > 200:
        print(f"Ngày đầu tiên vượt ngưỡng: ngày {ngay}")
        break
else:
    print("Cả tuần không ngày nào tiêu quá 200")
```

```text title=readonly
Ngày đầu tiên vượt ngưỡng: ngày 2
```

Lượt 2 gặp 250, in ra dòng chữ, rồi `break` cắt ngang vòng lặp. Vòng lặp không
đi trọn, nên nhánh `else` **không chạy**. Chỉ có một dòng trên màn hình.
::::

::::predict{#doan-may-in-may-dong commitOnce}
Sổ ba ngày dưới đây có một ngày vượt ngưỡng. **Trước khi bấm chạy**, bạn đoán
màn hình hiện ra những dòng nào?

```python title=readonly
chi_tieu = [120, 250, 90]
ngay = 0

for tien in chi_tieu:
    ngay += 1
    if tien > 200:
        print(f"Thấy rồi: ngày {ngay}")
        break
else:
    print("Không có ngày nào")
```

:::opt{correct}
Đúng một dòng: Thấy rồi: ngày 2
:::

:::opt
Cả hai dòng: Thấy rồi: ngày 2, rồi Không có ngày nào
::why
Gần đúng ở chỗ bạn đọc dòng đầu hoàn toàn chính xác — lượt 2 gặp 250, in ra
*Thấy rồi: ngày 2*. Và cách bạn nhìn dòng `else` cũng có lý: nó viết sát lề
trái, mà từ trước tới nay mọi dòng sát lề trái đều chạy sau khi vòng lặp xong.

Chỗ lệch: `else` viết sát lề trái nhưng nó **không** là một khối độc lập. Nó
dính vào `for` thành một cấu trúc, y như `else` dính vào `if`. Việc nó có chạy
hay không phụ thuộc vào chuyện xảy ra bên trong vòng: gặp `break` thì bỏ, đi hết
lượt thì chạy. Ở đây `break` đã gặp ở lượt 2.

Muốn một dòng luôn chạy sau vòng thì viết nó thành `print(...)` sát lề trái,
không có chữ `else` phía trước.
::
:::

:::opt
Đúng một dòng: Không có ngày nào
::why
Gần đúng ở chỗ bạn đọc `else` theo đúng nghĩa nó có trong `if`: hai nhánh đối
lập nhau, đúng một nhánh được chạy. Với `if` thì điều đó chính xác, và cách nghĩ
ấy sẽ còn đúng suốt.

Chỗ lệch nằm ở chỗ **cặp đối lập là gì**. Với `for ... else`, hai nhánh đối lập
không phải "thân vòng" và "else", mà là "có gặp `break`" và "đi hết lượt mà
không gặp `break`". Thân vòng vẫn cứ chạy bình thường trong cả hai trường hợp —
và ở đây nó đã kịp in ra một dòng trước khi `break` cắt ngang.
::
:::

:::opt
Máy dừng lại và báo `SyntaxError`, vì `else` phải đi với `if`
::why
Gần đúng ở chỗ bạn dựa vào toàn bộ kinh nghiệm đã có: mọi `else` bạn từng gặp
đều đi với `if`, nên một `else` thẳng hàng với `for` trông rất lạ. Cảnh giác như
vậy là thói quen tốt.

Chỗ lệch: Python cho phép cả `for` và `while` có `else` của riêng chúng. Cấu
trúc này hợp lệ, không có lỗi cú pháp nào. Nó hiếm gặp hơn `if ... else` nhiều,
và đó là lý do trông nó lạ — chứ không phải vì nó sai.
::
:::
::::

::::explain{#doc-chu-else-cho-dung}
Cái tên `else` ở đây gây nhầm cho gần như tất cả mọi người, kể cả người viết
Python đã lâu. Nghe "else" thì đầu tự dịch thành "ngược lại", mà ngược lại với
cái gì thì không rõ.

Có một cách đọc chữa được chỗ nhầm này. Mỗi lần thấy `else` thẳng hàng với `for`
hay `while`, hãy đọc thầm trong đầu:

> **"không gặp `break` thì…"**

Đọc lại đoạn ví dụ theo lối ấy: *đi qua từng ngày, thấy ngày vượt ngưỡng thì in
ra rồi dừng; **không gặp `break` thì** in ra là cả tuần không có ngày nào.*

Từ đó suy ra được luôn hai trường hợp lề mà bạn sẽ gặp:

- Vòng lặp chạy **0 lượt** — sổ rỗng, hoặc `while` sai ngay từ đầu. Không lượt
  nào chạy thì cũng chẳng lượt nào gặp `break`, nên `else` **chạy**.
- Thân vòng **không có `break`** nào cả. Vậy thì `else` chạy sau mọi lần, lần
  nào cũng vậy — và lúc đó chữ `else` chỉ làm người đọc code hoang mang. `for
  ... else` chỉ đáng viết khi trong thân có `break`.

So với bài trước, bạn đổi được ba thứ phải nhớ lấy một: không còn con số canh
phải bịa, không còn dòng khởi tạo trước vòng, không còn phép so sánh sau vòng.
Nhánh "không tìm thấy" nằm ngay tại chỗ, dính liền với vòng lặp sinh ra nó.
::::

::::code{#chuyen-tau-cuoi-ngay}
Ga tàu có năm chuyến trong ngày, khởi hành lúc 5, 7, 9, 11 và 13 giờ. Bạn ra ga,
nhìn đồng hồ, rồi muốn biết chuyến gần nhất còn đi được là chuyến nào.

Đoạn dưới là **hai** lần ra ga, mỗi lần một khối. Hai khối giống nhau từng chữ
một, chỉ khác đúng con số trong `gio_muon`: lần đầu bạn tới lúc 8 giờ và còn kịp
chuyến; lần sau tới lúc 14 giờ, muộn hơn cả chuyến cuối cùng.

Cả hai khối đã đi qua từng chuyến và đã có `break`. Cùng một chỗ trống, điền hai
lần y hệt nhau. Điền đúng thì hai khối tự khắc nói ra hai chuyện khác nhau — vì
cái quyết định câu trả lời là vòng lặp có gặp `break` hay không, chứ không phải
câu bạn gõ.

```python title=starter
gio_tau = [5, 7, 9, 11, 13]

gio_muon = 8
for gio in gio_tau:
    if gio >= gio_muon:
        print(f"Ra ga lúc {gio_muon} giờ: đi được chuyến {gio} giờ")
        break
___:
    print(f"Ra ga lúc {gio_muon} giờ: hôm nay hết tàu rồi")

gio_muon = 14
for gio in gio_tau:
    if gio >= gio_muon:
        print(f"Ra ga lúc {gio_muon} giờ: đi được chuyến {gio} giờ")
        break
___:
    print(f"Ra ga lúc {gio_muon} giờ: hôm nay hết tàu rồi")
```

```python title=solution
gio_tau = [5, 7, 9, 11, 13]

gio_muon = 8
for gio in gio_tau:
    if gio >= gio_muon:
        print(f"Ra ga lúc {gio_muon} giờ: đi được chuyến {gio} giờ")
        break
else:
    print(f"Ra ga lúc {gio_muon} giờ: hôm nay hết tàu rồi")

gio_muon = 14
for gio in gio_tau:
    if gio >= gio_muon:
        print(f"Ra ga lúc {gio_muon} giờ: đi được chuyến {gio} giờ")
        break
else:
    print(f"Ra ga lúc {gio_muon} giờ: hôm nay hết tàu rồi")
```

```python title=test
# Chấm bằng TRỌN VẸN hai dòng output, không phải một dòng.
#
# Một khối thì không phân biệt được gì: gõ `if True` vào chỗ trống cũng in ra
# đúng câu mà khối ấy mong đợi. Hai khối thì `if True` lộ ngay — khối đầu in ra
# cả hai câu cùng lúc, vừa "đi được chuyến 9 giờ" vừa "hôm nay hết tàu rồi",
# thành ba dòng trên màn hình thay vì hai.
#
# Khối này khẳng định thêm rằng vòng thứ hai đã đi trọn năm chuyến: cái tên
# `gio` còn giữ giờ của chuyến cuối cùng, nghĩa là không lượt nào gặp `break`.
assert gio == 13
assert gio_muon == 14
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm ở cùng một vị trí trong hai khối: đầu dòng, sát lề trái — thẳng hàng với chữ `for` phía trên, chứ không thẳng hàng với `if`. Mức thụt lề đó cho biết từ khoá này thuộc về ai.
- kind: strategy
  body: Nhánh cần viết là nhánh chỉ chạy khi vòng đi hết lượt mà không lần nào gặp `break`. Python dùng lại đúng một từ khoá bạn đã biết cho việc này, chỉ đổi chỗ đặt nó. Cùng từ khoá ấy điền vào cả hai chỗ — hai lần ra ga khác nhau là do con số `gio_muon` khác nhau, không phải do bạn gõ khác đi.
- kind: one-line
  body: "Viết `else` vào **cả hai** chỗ trống, giữ nguyên dấu hai chấm ở cuối dòng."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Ra ga lúc 8 giờ: đi được chuyến 9 giờ\nRa ga lúc 14 giờ: hôm nay hết tàu rồi\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đi hết lượt mà không gặp ai — giờ chuyện đó có chỗ đứng đàng hoàng trong code.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Từng câu hỏi một thì bạn trả lời được hết rồi: tổng chi cả tuần, số ngày vượt
ngưỡng, ngày tiêu nhiều nhất, ngày đầu tiên vượt, và cả trường hợp chẳng có ngày
nào.

Nhưng cuối tháng, chủ nhà trọ hỏi Byte một lượt năm con số đó. Cách thẳng nhất
là viết năm vòng lặp: vòng một cộng tổng, vòng hai đếm ngày vượt, vòng ba tìm
ngày kỷ lục, vòng bốn tìm ngày đầu tiên vượt…

Cùng một cuốn sổ ba mươi dòng, lật đi lật lại năm lượt. Trong khi người ghi sổ
ngoài đời chỉ lật đúng một lượt, tay vừa cộng vừa đếm vừa để ý con số lớn nhất.

Một lượt duyệt có làm được nhiều việc cùng lúc như vậy không? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
