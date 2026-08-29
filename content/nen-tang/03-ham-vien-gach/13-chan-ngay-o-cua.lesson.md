---
id: nen-tang.ham-vien-gach.chan-ngay-o-cua
title: Chặn ngay ở cửa
summary: Mỗi trường hợp xấu một `return` đặt ngay đầu hàm — thân hàm còn lại phẳng ra và chỉ lo đúng việc chính.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.guard-clause]
requires: [core.return-exits, core.implicit-return-none, core.docstring, core.function-def, core.function-parameter, core.function-return, core.function-call, ctrl.if, ctrl.if-nested, ctrl.block-indent, ctrl.comparison, core.floor-division, core.fstring]
concepts: [core.ham, core.tra-ve, ctrl.re-nhanh]
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
Chuyện hỏng thì nói ngay ở cửa. Nói xong là xong, khỏi mang vào trong.
::::

::::explain{#cai-hinh-thang}
Đây là hàm bài trước để lại. Nó chạy đúng, và đọc thì mỏi mắt:

```python
def tinh_hoa_don(ten_mon, gia_mot_to, so_to):
    """Tính tổng tiền một bàn, hoặc đưa ra 0 nếu đơn không tính được."""
    if so_to >= 1:
        if gia_mot_to >= 1:
            if ten_mon != "":
                hang = gia_mot_to * so_to
                thue = hang // 10
                return hang + thue
    return 0
```

Nhìn cái hình thang nó vẽ trên màn hình: ba dòng làm việc chính — ba dòng duy
nhất mà cả hàm sinh ra để làm — nằm ở tầng trong cùng, cách lề mười sáu dấu
cách. Muốn biết chúng chạy trong hoàn cảnh nào, bạn phải ngước lên đọc ngược
đủ ba dòng `if` rồi giữ cả ba trong đầu cùng một lúc.

Thêm một trường hợp xấu nữa là thêm một tầng, và lề lại lùi xa hơn.

Bây giờ lấy đúng một câu bài trước để lại và soi vào đây:

> `return` kết thúc lượt chạy của hàm **ngay tại chỗ**. Mọi dòng nằm sau nó
> không bao giờ chạy.

Nếu một dòng `return` đã cắt đứt lượt chạy như vậy, thì phần nằm dưới nó đâu
cần ai bọc quanh để che chở nữa. Nó chỉ chạy được khi lời `return` kia
**không** chạy. Vậy thì ba tầng `if` bọc bên ngoài chẳng còn việc gì để làm.

Nên hàm viết lại được theo chiều ngược hẳn. Thay vì hỏi *"đơn tốt hả, mời vào
tầng trong"*, ta hỏi thẳng *"đơn hỏng hả, mời ra ngay"*:

| Cách bọc `if` quanh trường hợp tốt | Cách chặn ngay ở cửa |
|---|---|
| `if so_to >= 1:` rồi lồng tiếp | `if so_to < 1:` rồi `return 0` |
| `if gia_mot_to >= 1:` rồi lồng tiếp | `if gia_mot_to < 1:` rồi `return 0` |
| `if ten_mon != "":` rồi lồng tiếp | `if ten_mon == "":` rồi `return 0` |

Mỗi dòng ở cột phải là một **cổng chặn sớm**: một câu hỏi về đúng một trường
hợp xấu, và ngay dưới nó là lời `return` tiễn trường hợp ấy ra khỏi hàm. Đơn
nào lọt qua hết mọi cổng thì mới đi tiếp được vào thân hàm.
::::

::::example{#phang-ra}
Ba cổng xếp thành một hàng ở đầu hàm, rồi mới tới việc chính:

```python title=readonly
def tinh_hoa_don(ten_mon, gia_mot_to, so_to):
    """Tính tổng tiền một bàn, hoặc đưa ra 0 nếu đơn không tính được."""
    if so_to < 1:
        return 0
    if gia_mot_to < 1:
        return 0
    if ten_mon == "":
        return 0
    hang = gia_mot_to * so_to
    thue = hang // 10
    return hang + thue


print(tinh_hoa_don("phở tái", 45000, 2))
print(tinh_hoa_don("phở tái", 45000, 0))
print(tinh_hoa_don("", 45000, 2))
```

Máy in ra:

```text
99000
0
0
```

Đi lại đường máy đi ở từng lời gọi:

- **Đơn đủ ba thứ** — cổng một hỏi `2 < 1`, sai, không vào thân cổng. Cổng hai
  hỏi `45000 < 1`, sai. Cổng ba hỏi `"phở tái" == ""`, sai. Ba cổng đều mở,
  máy đi thẳng xuống ba dòng cuối và tính ra tổng.
- **Đơn ghi không tô nào** — cổng một hỏi `0 < 1`, đúng. Máy chạy `return`
  ngay tại đó, lượt chạy của hàm chấm hết, và hai cổng còn lại lẫn ba dòng
  tính tiền không được ngó tới.
- **Đơn để trống tên món** — hai cổng đầu mở, cổng ba hỏi `"" == ""`, đúng, và
  hàm ra về từ chỗ ấy.

So hai bản với nhau, cùng một hàm, cùng một kết quả:

- Ba dòng làm việc chính từ chỗ cách lề mười sáu dấu cách lùi về sát lề của
  thân hàm. Đọc tới đó bạn không phải nhớ điều kiện nào — mọi trường hợp xấu
  đã bị chặn ở phía trên rồi, nên tới đây chắc chắn là đơn lành lặn.
- Cái `return 0` cô đơn ở cuối bản cũ, thứ phải đoán mãi mới biết nó lo cho ba
  trường hợp nào, giờ được tách thành ba dòng và mỗi dòng nằm sát ngay điều
  kiện sinh ra nó.
- Thêm trường hợp xấu thứ tư thì chỉ thêm một cổng nữa vào hàng, không tầng
  nào lùi thêm dấu cách nào.
::::

::::predict{#quen-lat-dieu-kien commitOnce}
Byte hào hứng chuyển hàm sang lối chặn sớm, nhưng chép vội nên bê nguyên điều
kiện cũ của cổng đầu tiên xuống mà quên lật nó lại.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def tinh_hoa_don(ten_mon, gia_mot_to, so_to):
    """Tính tổng tiền một bàn, hoặc đưa ra 0 nếu đơn không tính được."""
    if so_to >= 1:
        return 0
    if gia_mot_to < 1:
        return 0
    if ten_mon == "":
        return 0
    hang = gia_mot_to * so_to
    thue = hang // 10
    return hang + thue


print(tinh_hoa_don("phở tái", 45000, 2))
```

:::opt{correct}
0
:::

:::opt
99000
::why
Gần đúng ở chỗ bạn đọc hàm theo đúng ý định của người viết: một đơn có tên
món, có giá, có hai tô là đơn lành lặn, mà cổng chặn thì sinh ra để bắt đơn
hỏng — nên đơn này phải đi lọt tới dòng tính tiền. Ý định ấy chính là ý định
Byte đang có trong đầu.

Chỗ lệch: máy không đọc được ý định, nó chỉ đọc điều kiện. Dòng cổng đang hỏi
`so_to >= 1` — *số tô có từ 1 trở lên không*. Với hai tô, câu trả lời là đúng,
nên máy vào thân cổng và `return 0` ngay tại đó.

Chuyển sang chặn sớm là phải **lật** điều kiện, vì cổng hỏi về trường hợp xấu
chứ không phải trường hợp tốt. Lật hụt một chỗ thì chương trình vẫn chạy trơn
tru và vẫn in ra một con số trông tử tế — loại lỗi không ai báo cho bạn.
::
:::

:::opt
None
::why
Gần đúng ở chỗ bạn nhớ đúng bài 11: một hàm chạy hết thân mà không gặp `return`
nào thì vẫn đưa ra một giá trị, và giá trị đó là `None`. Luật ấy chính xác.

Chỗ lệch là ở chuyện hàm này có gặp `return` hay không. Nó gặp ngay dòng thứ
hai của thân: cổng đầu tiên mở ra và `return 0` chạy. Có `return` chạy thì thứ
đi ra là thứ viết sau chữ `return`, ở đây là số 0.

`None` chỉ xuất hiện khi máy đi hết thân hàm mà chẳng gặp dòng `return` nào —
chuyện đó không xảy ra ở đây.
::
:::

:::opt
Máy báo lỗi, vì một hàm không được có tới bốn dòng `return`
::why
Gần đúng ở chỗ bạn đang giữ một hình dung gọn gàng về hàm: đưa vào một thứ,
nhận ra một thứ, nên chắc cũng chỉ có một lối ra. Nhiều tài liệu cũng khuyên
viết hàm theo lối một lối ra ấy thật.

Chỗ lệch: Python không đếm số dòng `return` trong hàm và cũng không có luật nào
cấm. Ở mỗi lượt gọi, đúng một trong số chúng được chạy — cái nào tới trước thì
cái đó — nên viết bao nhiêu dòng `return` cũng hợp lệ.

Cả lối chặn sớm đứng được là nhờ đúng chuyện đó.
::
:::
::::

::::explain{#hai-chuyen-de-y}
Đổi sang lối chặn sớm có hai chỗ phải cẩn thận, và bạn vừa gặp cả hai.

- **Điều kiện phải lật.** Cổng hỏi *"có phải trường hợp xấu không"*, ngược hẳn
  với `if` bọc ngoài vốn hỏi *"có phải trường hợp tốt không"*.
- **Thứ tự các cổng là thứ tự trả lời.** Một đơn có thể hỏng theo hai kiểu
  cùng lúc; cổng nào đứng trên thì cổng ấy chặn, cổng dưới không được hỏi tới.
  Nên cổng nào lo chuyện quan trọng hơn thì đặt lên trước.

Còn một chỗ dễ thừa tay: sau mỗi cổng **không cần** `else`. Viết `else` vào
cũng không sai, chương trình vẫn chạy đúng — nhưng nó lại đẩy phần còn lại của
hàm thụt vào một tầng, tức là mang đúng cái hình thang vừa dỡ bỏ quay trở lại.

Tách ra thành từng cổng riêng còn được thêm một món quà. Ở bản trên cả ba cổng
đều `return 0`, nên nhìn vào con số 0 người gọi chẳng biết đơn hỏng ở đâu. Giờ
mỗi cổng đứng riêng một dòng, muốn cổng nào nói lời của cổng ấy thì chỉ việc
viết câu ấy vào — mỗi cửa một lời từ chối khác nhau.

> Chỗ dễ vấp: một cổng chặn phải **kết bằng `return`**. Nếu bạn chỉ viết
> `if so_to < 1:` rồi `print("Đơn chưa ghi số tô")` mà quên `return`, máy in
> câu báo xong vẫn chạy tiếp xuống thân hàm và vẫn tính tiền cho một đơn hỏng.
> Cổng không có `return` thì không phải cổng, nó chỉ là một lời than.
::::

::::code{#cong-cho-don-giao-hang}
Quán Byte nhận giao hàng, và lần này mỗi cửa nói một lời riêng.

- Đơn chưa ghi quãng đường — số cây số bằng 0, hoặc âm vì ai đó gõ nhầm — thì
  quán chưa báo được phí.
- Đơn xa hơn 10 cây số thì quán không giao tới. Đúng 10 cây số thì vẫn giao.

Đơn nào qua được cả hai cổng thì tính phí: mỗi cây số 5 nghìn đồng.

Hãy điền điều kiện cho hai cổng chặn.

```python title=starter
def bao_gia_ship(so_km):
    """Báo phí giao hàng, hoặc nói rõ vì sao quán chưa giao được."""
    if ___:
        return "Chưa biết quãng đường"
    if ___:
        return "Xa hơn 10 km, quán chưa giao tới"
    phi = so_km * 5000
    return f"Phí giao: {phi} đồng"


print(bao_gia_ship(3))
print(bao_gia_ship(0))
print(bao_gia_ship(12))
```

```python title=solution
def bao_gia_ship(so_km):
    """Báo phí giao hàng, hoặc nói rõ vì sao quán chưa giao được."""
    if so_km <= 0:
        return "Chưa biết quãng đường"
    if so_km > 10:
        return "Xa hơn 10 km, quán chưa giao tới"
    phi = so_km * 5000
    return f"Phí giao: {phi} đồng"


print(bao_gia_ship(3))
print(bao_gia_ship(0))
print(bao_gia_ship(12))
```

```python title=test
# Năm lời gọi, và ba trong số đó đứng sát mép của một cổng: 0 phải bị chặn,
# 10 phải lọt, 11 phải bị chặn. Một cổng lật hụt hay lệch một nấc so sánh
# đều lộ ra ở đúng ba lời gọi ấy.
assert bao_gia_ship(3) == "Phí giao: 15000 đồng", "ba cây số là đơn lành lặn: cả hai cổng phải mở, và hàm dựng câu phí giao chứ không báo lỗi"
assert bao_gia_ship(10) == "Phí giao: 50000 đồng", "đúng 10 cây số thì quán vẫn giao — cổng thứ hai chỉ được chặn khi XA HƠN 10 km"
assert bao_gia_ship(0) == "Chưa biết quãng đường", "đơn chưa ghi quãng đường phải bị cổng thứ nhất chặn ngay, không được đi tiếp xuống dòng tính phí"
assert bao_gia_ship(-2) == "Chưa biết quãng đường", "số cây số âm cũng là đơn chưa ghi tử tế: cổng thứ nhất phải bắt được cả nó, không riêng số 0"
assert bao_gia_ship(11) == "Xa hơn 10 km, quán chưa giao tới", "quá 10 cây số thì cổng thứ hai chặn, hàm không được tính phí cho đơn này"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều là điều kiện của một cổng chặn, và mỗi cổng đứng ngay trên câu báo của chính nó. Đọc câu báo trước đã, rồi mới nhìn lại chỗ trống: cổng phải mở đúng vào lúc câu báo ấy là sự thật.
- kind: strategy
  body: Cổng chặn hỏi về trường hợp XẤU, ngược hẳn với lối bọc `if` quanh trường hợp tốt. Cổng thứ nhất lo đơn chưa có quãng đường, mà chưa có nghĩa là số cây số không lớn hơn 0 — gồm cả số 0 lẫn số âm. Cổng thứ hai lo đơn quá xa, và đúng 10 cây số thì quán vẫn nhận, nên nó chỉ được đóng lại khi số cây số vượt lên trên 10.
- kind: one-line
  body: Chỗ trống thứ nhất là `so_km <= 0`, chỗ thứ hai là `so_km > 10`; giữ nguyên dấu hai chấm cuối mỗi dòng.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Phí giao: 15000 đồng\nChưa biết quãng đường\nXa hơn 10 km, quán chưa giao tới\s*$
- tier: output
  expect: Chưa biết quãng đường
- tier: output
  expect: Xa hơn 10 km, quán chưa giao tới
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đơn hỏng ra về ngay ở cửa. Vào tới trong thì chỉ còn việc chính.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Thân hàm phẳng lại thật. Nhưng `tinh_hoa_don` vẫn dài 30 dòng: vừa tra giá
theo cỡ tô, vừa tính thuế trên số tiền ấy, vừa dựng câu chữ để đưa ra cho
người đọc.

Ba việc khác hẳn nhau nằm chung một chỗ. Bạn đã có `def` để đặt tên cho một
việc, nên cắt ra thành ba hàm — một hàm tra giá, một hàm tính thuế, một hàm
dựng câu — nghe hoàn toàn làm được.

Cắt ra làm ba thì ghép lại bằng cách nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
