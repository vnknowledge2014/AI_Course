---
id: nen-tang.ham-vien-gach.may-chay-tu-trong-ra
title: Máy chạy từ trong ra
summary: Đối số phải được tính XONG trước khi hàm ngoài bắt đầu chạy — lớp ngoặc trong cùng đi trước, không có ngoại lệ.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.eval-inside-out]
requires: [core.nested-call, core.rhs-first, core.function-arg-count, core.positional-argument, core.docstring, core.function-def, core.function-parameter, core.function-argument, core.function-return, core.function-call, core.floor-division, core.fstring, ctrl.if, ctrl.comparison]
concepts: [core.ham, core.gia-tri, core.thu-tu-buoc]
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
Mình chưa cầm được thứ gì trong tay thì mình chưa bắt đầu. Đợi lớp trong xong đã.
::::

::::explain{#ba-lop-ngoac}
Bài trước kết bằng một dòng ba lớp ngoặc:

```python
print(dung_cau(tinh_thue(tra_gia("vừa"))))
```

Mắt bạn đọc dòng này từ trái sang phải, nên `dung_cau` là cái tên gặp đầu
tiên. Rất tự nhiên khi nghĩ nó cũng chạy đầu tiên.

Nhưng thử đặt mình vào chỗ cái máy. Muốn chạy `dung_cau`, máy phải điền vào
tham số của nó một giá trị — bài 5 đã nói: thiếu đối số là dừng ngay. Vậy đối
số của `dung_cau` là gì? Nó là `tinh_thue(tra_gia("vừa"))`, mà thứ đó chưa
phải một giá trị: nó mới là một **lời hứa** sẽ cho ra một giá trị. Máy không
điền một lời hứa vào tham số được.

Nên máy chưa động được vào `dung_cau`. Nó phải đi giải quyết lời hứa kia
trước. Mà lời hứa kia lại vướng đúng chuyện y hệt: muốn chạy `tinh_thue` thì
cần một giá trị, và thứ đang nằm trong ngoặc của nó là `tra_gia("vừa")` —
cũng chỉ là một lời hứa.

Cứ thế lùi vào cho tới lớp trong cùng. `tra_gia("vừa")` là chỗ đầu tiên máy
làm việc được, vì đối số của nó — chuỗi `"vừa"` — là một giá trị có sẵn, không
phải đợi ai tính hộ.

Từ đó máy đi ngược trở ra:

- `tra_gia("vừa")` chạy xong, đưa ra một con số. Con số ấy thế chỗ cho cả cụm
  `tra_gia("vừa")` trong dòng.
- Giờ `tinh_thue` mới có một giá trị thật trong ngoặc, nên nó chạy được. Xong,
  nó cũng đưa ra một con số, và con số ấy thế chỗ cho cả cụm `tinh_thue(...)`.
- Tới lúc này `dung_cau` mới có cái để nhận, và nó chạy sau cùng.

Đây không phải luật mới. Bạn đã gặp đúng nó ở bài "vế phải tính xong trước":
`tong = gia * 2` thì phép nhân phải xong trước khi cái tên `tong` nhận được
gì. Một đối số cũng vậy — **thứ nằm trong ngoặc phải trở thành một giá trị
xong xuôi rồi hàm ngoài mới khởi động.**

Cách nói gọn: máy chạy **từ trong ra ngoài**. Đọc thì trái sang phải, chạy thì
trong ra ngoài. Hai chiều ngược nhau, và không có ngoại lệ nào.
::::

::::example{#nhin-tan-mat}
Nói thì dễ tin, nhìn thì chắc hơn. Ba hàm dưới đây làm đúng việc cũ, chỉ thêm
một dòng báo lại lúc chúng bắt đầu chạy:

```python title=readonly
def tra_gia(co):
    """Trả về giá một tô theo cỡ, kèm một câu báo lúc bắt đầu chạy."""
    print("  tra_gia bắt đầu chạy")
    if co == "nhỏ":
        return 40000
    if co == "vừa":
        return 45000
    return 55000


def tinh_thue(tien):
    """Trả về tiền thuế của một số tiền, kèm một câu báo lúc bắt đầu chạy."""
    print("  tinh_thue bắt đầu chạy")
    return tien // 10


def dung_cau(thue):
    """Dựng câu báo thuế, kèm một câu báo lúc bắt đầu chạy."""
    print("  dung_cau bắt đầu chạy")
    return f"Thuế phải nộp: {thue} đồng"


print(dung_cau(tinh_thue(tra_gia("vừa"))))
```

Máy in ra:

```text
  tra_gia bắt đầu chạy
  tinh_thue bắt đầu chạy
  dung_cau bắt đầu chạy
Thuế phải nộp: 4500 đồng
```

Ba dòng thụt vào là ba lời khai của ba hàm, và thứ tự của chúng ngược hẳn thứ
tự mắt bạn đọc tên chúng trên dòng lệnh. `dung_cau` viết ngoài cùng mà khai
sau cùng.

Dòng thứ ba là dòng đáng nhìn kỹ nhất: **`dung_cau` chỉ khai sau khi cả hai
hàm kia đã xong.** Không phải nó khởi động rồi đi đâu đó lấy đối số — nó chưa
hề khởi động. Cả lúc `tra_gia` chạy lẫn lúc `tinh_thue` chạy, `dung_cau` vẫn
chưa bắt đầu chút nào.

Muốn tự kiểm lại luật này ở bất cứ dòng nào, có một mẹo đọc: tìm cặp ngoặc nào
mà bên trong nó **không còn lời gọi hàm nào nữa** — trong dòng trên là
`tra_gia("vừa")`, vì `"vừa"` là một giá trị sẵn có. Đó là chỗ máy bắt đầu.
::::

::::predict{#thu-tu-nao commitOnce}
Byte lồng hai viên gạch của quán vào nhau rồi đưa kết quả cho `dung_cau`. Ba
hàm, ba câu khai, và một dòng lệnh duy nhất ở cuối.

**Trước khi bấm chạy**, bạn đoán bốn dòng trên màn hình xếp theo thứ tự nào?

```python
def them_phi(tien):
    """Cộng 5 nghìn phí phục vụ, kèm một câu báo lúc bắt đầu chạy."""
    print("them_phi bắt đầu chạy")
    return tien + 5000


def tinh_thue(tien):
    """Trả về tiền thuế, kèm một câu báo lúc bắt đầu chạy."""
    print("tinh_thue bắt đầu chạy")
    return tien // 10


def dung_cau(thue):
    """Dựng câu báo thuế, kèm một câu báo lúc bắt đầu chạy."""
    print("dung_cau bắt đầu chạy")
    return f"Thuế phải nộp: {thue} đồng"


print(dung_cau(them_phi(tinh_thue(40000))))
```

:::opt{correct}
`tinh_thue` → `them_phi` → `dung_cau` → rồi `Thuế phải nộp: 9000 đồng`
:::

:::opt
`dung_cau` → `them_phi` → `tinh_thue` → rồi câu thuế
::why
Gần đúng ở chỗ bạn đọc dòng lệnh đúng như mọi người đọc chữ: từ trái sang
phải, tên nào viết trước thì gặp trước. Cách hình dung đi kèm cũng rất hợp lý
— hàm ngoài khởi động, thấy mình còn thiếu, bèn đi lấy dần cho đủ.

Chỗ lệch nằm ở chữ "khởi động". Một hàm chỉ bắt đầu chạy khi **mọi** đối số
của nó đã là giá trị xong xuôi, nên `dung_cau` không có giai đoạn nửa vời "đã
chạy nhưng còn thiếu". Thứ đang nằm trong ngoặc của nó vẫn là một lời gọi chưa
chạy, nên nó phải đợi — và đợi tới lượt cuối cùng.

Thứ tự ĐỌC không phải thứ tự CHẠY.
::
:::

:::opt
`them_phi` → `tinh_thue` → `dung_cau` → rồi câu thuế
::why
Gần đúng ở chỗ quan trọng nhất: bạn đã nắm đúng luật trong ra ngoài, và bạn
xếp `dung_cau` — lớp ngoài cùng — xuống cuối, hoàn toàn chính xác.

Chỗ lệch chỉ là đếm nhầm lớp nào trong cùng. Trong ngoặc của `them_phi` vẫn
còn một lời gọi nữa là `tinh_thue(40000)`, nên `them_phi` cũng đang thiếu giá
trị y như `dung_cau`. Lớp trong cùng là lớp mà **bên trong ngoặc không còn lời
gọi nào** — ở đây là `tinh_thue(40000)`, vì `40000` là một con số có sẵn.

Đếm từ ngoài vào thì dễ lẫn; tìm thẳng cặp ngoặc trong cùng thì chắc hơn.
::
:::

:::opt
Máy báo lỗi, vì `them_phi` bị đưa cho tiền thuế chứ không phải giá một tô
::why
Gần đúng ở chỗ bạn đọc ra một chuyện có thật và đáng lo: tham số của `them_phi`
tên là `tien` và cả quán vẫn hiểu nó là giá tô, thế mà lần này thứ rót vào lại
là tiền thuế. Về mặt ý nghĩa, hoá đơn này đang sai.

Chỗ lệch: máy không đọc được ý nghĩa ấy. Nó chỉ thấy một con số đi vào một
tham số, y như bài 6 đã cho thấy khi hai đối số cùng kiểu bị đảo chỗ — chương
trình chạy trơn tru và cho ra kết quả sai mà không báo một tiếng.

Sai kiểu này không có thông báo lỗi nào đi kèm. Người duy nhất bắt được nó là
người đọc lại thứ tự các cặp ngoặc.
::
:::
::::

::::explain{#doi-cho-la-doi-ket-qua}
Luật trong ra ngoài không phải chuyện lý thuyết cho vui. Nó quyết định con số
cuối cùng, vì đổi thứ tự lồng là đổi kết quả.

Quán Byte có hai bước tính: cộng 5 nghìn phí phục vụ, và tính thuế bằng một
phần mười số tiền. Với một tô 40 nghìn:

- Cộng phí trước rồi mới tính thuế trên số đã cộng — máy chạy `them_phi`
  trước, `tinh_thue` sau, và tiền thuế tính trên 45 nghìn.
- Tính thuế trước rồi cộng phí vào chính tiền thuế — máy chạy `tinh_thue`
  trước, và ra một con số hoàn toàn khác.

Hai cách viết chỉ khác nhau chỗ cặp ngoặc nào nằm trong, mà hoá đơn thì lệch
hẳn. Máy không đoán được bạn muốn cái nào; nó chỉ làm đúng một việc là chạy từ
trong ra ngoài.

> Chỗ dễ vấp: cả hai cách viết đều chạy trơn tru, không lỗi, không cảnh báo,
> và cả hai đều in ra một con số trông rất hợp lý. Đây là loại sai máy không
> bao giờ nhắc bạn — muốn chắc, chỉ có một cách là tự chỉ tay vào cặp ngoặc
> trong cùng và hỏi: *việc này có đúng là việc phải làm đầu tiên không?*
::::

::::code{#long-dung-thu-tu}
Byte đưa bạn hai viên gạch đã viết sẵn: `them_phi` cộng phí phục vụ, `tinh_thue`
tính thuế trên một số tiền.

Quán tính thuế trên số tiền **đã cộng phí phục vụ**. Hãy điền một dòng dùng
lại đúng hai hàm ấy, lồng chúng theo thứ tự cho ra con số đúng.

```python title=starter
def them_phi(tien):
    """Cộng 5 nghìn phí phục vụ vào số tiền đưa vào."""
    return tien + 5000


def tinh_thue(tien):
    """Trả về tiền thuế của một số tiền: một phần mười số ấy."""
    return tien // 10


def thue_cua_don(gia_to):
    """Trả về tiền thuế, tính trên giá tô ĐÃ cộng phí phục vụ."""
    return ___


print(f"Tô 40 nghìn: thuế {thue_cua_don(40000)} đồng")
print(f"Tô 55 nghìn: thuế {thue_cua_don(55000)} đồng")
```

```python title=solution
def them_phi(tien):
    """Cộng 5 nghìn phí phục vụ vào số tiền đưa vào."""
    return tien + 5000


def tinh_thue(tien):
    """Trả về tiền thuế của một số tiền: một phần mười số ấy."""
    return tien // 10


def thue_cua_don(gia_to):
    """Trả về tiền thuế, tính trên giá tô ĐÃ cộng phí phục vụ."""
    return tinh_thue(them_phi(gia_to))


print(f"Tô 40 nghìn: thuế {thue_cua_don(40000)} đồng")
print(f"Tô 55 nghìn: thuế {thue_cua_don(55000)} đồng")
```

```python title=test
# Ba giá tô khác nhau, và lồng ngược thứ tự thì cả ba đều lệch: tính thuế
# trước rồi mới cộng phí sẽ ra 9000, 10500 và 7500 — không con số nào trùng
# với đáp án đúng. Một con số chép cứng cũng chỉ qua nổi một dòng.
assert thue_cua_don(40000) == 4500, "tô 40 nghìn cộng phí phục vụ thành 45 nghìn, và thuế là một phần mười của 45 nghìn"
assert thue_cua_don(55000) == 6000, "tô 55 nghìn cộng phí phục vụ thành 60 nghìn, nên thuế phải là 6 nghìn"
assert thue_cua_don(25000) == 3000, "tô 25 nghìn cộng phí phục vụ thành 30 nghìn; nếu tính thuế trước rồi mới cộng phí thì con số ra khác hẳn"
```

:::hints
- kind: attention
  body: Chỗ trống nằm sau `return`, nên nó phải là một thứ cho ra giá trị. Đọc lại câu mô tả của hàm và để ý chữ ĐÃ trong đó — nó nói bước nào phải xong trước bước nào.
- kind: strategy
  body: Máy chạy từ trong ra ngoài, nên việc làm TRƯỚC phải nằm ở lớp ngoặc TRONG cùng. Ở đây việc làm trước là cộng phí phục vụ vào giá tô, còn tính thuế thì làm sau, trên kết quả vừa cộng xong. Lớp trong cùng nhận `gia_to` — thứ duy nhất có sẵn ngay từ đầu.
- kind: one-line
  body: Viết `tinh_thue(them_phi(gia_to))` vào chỗ trống, ngay sau chữ `return`.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Tô 40 nghìn: thuế 4500 đồng\nTô 55 nghìn: thuế 6000 đồng\s*$
- tier: output
  expect: Tô 55 nghìn: thuế 6000 đồng
- tier: static
  onFail: dòng bạn điền phải GỌI cả hai hàm đã cho, lồng cái nọ trong ngoặc của cái kia
  requireAst:
  - kind: uses-call, target: them_phi, min: 1
  - kind: uses-call, target: tinh_thue, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lớp trong xong thì lớp ngoài mới cầm được. Đúng thứ tự là ra đúng số.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte ghép lại theo lối khác: thay vì lồng ba lớp ngoặc trên một dòng, cho
`dung_cau` tự gọi `tinh_thue` ngay trong thân nó.

```python
def dung_cau(gia):
    """Tính thuế của một giá tô rồi dựng câu báo."""
    thue = tinh_thue(gia)
    return f"Thuế phải nộp: {thue} đồng"
```

Máy đang chạy dòng `thue = tinh_thue(gia)`. Nó rời khỏi dòng ấy để bước vào
`tinh_thue`, mà `dung_cau` thì mới làm được nửa việc — dòng `return` của nó
còn nguyên đó, chưa ai chạm tới.

Lúc `tinh_thue` đang chạy dở thì `dung_cau` nằm ở đâu? Chạy xong, máy biết
đường quay về đúng dòng nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
