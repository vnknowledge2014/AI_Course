---
id: nen-tang.ham-vien-gach.ba-dieu-de-goi-mot-ham
title: Ba điều, không hơn
summary: Chữ ký của một hàm gồm đúng ba điều — tên, những thứ phải đưa vào theo thứ tự, và thứ được đưa ra. Biết ba điều đó là gọi được.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.function-signature]
requires: [core.builtin-function, core.function-parameter, core.function-return, err.type-error]
concepts: [core.ham, core.tham-so, core.tra-ve]
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
Bạn không biết gì bên trong mình. Bạn vẫn gọi trúng. Đếm xem bạn biết mấy điều.
::::

::::explain{#dem-cho-het}
Bài trước để lại một việc phải đếm. Lúc bạn gõ `round(45000.7)`, trong đầu bạn
có sẵn những gì?

Đếm thật, từng món một:

- Bạn biết cái tên là `round`, viết đúng bốn chữ ấy, không viết hoa.
- Bạn biết phải đặt **một** con số vào trong ngoặc, và con số đó là thứ cần làm
  tròn chứ không phải thứ gì khác.
- Bạn biết thứ đi ra là một số nguyên, nên bạn dám đem nó cộng, nhân, hay nhét
  vào một câu chữ bằng `{ }`.

Hết. Không còn món nào nữa. Bạn không biết `round` so sánh phần lẻ ra sao, không
biết nó xử lý số âm thế nào, không biết nó dài mấy dòng — và không món nào trong
ba món trên đòi bạn biết những thứ ấy.

Ba điều đó có một cái tên chung: **chữ ký** của hàm.

Chữ ký một tờ giấy tờ là chỗ nhận ra ai đứng sau nó. Chữ ký của một hàm cũng vậy:
nó là phần **nhận ra được từ bên ngoài**, phần duy nhất người gọi cần nhìn. Ba
dòng, không hơn:

1. **Tên** — gõ đúng thì máy tìm ra.
2. **Những thứ phải đưa vào, theo đúng thứ tự.** Bao nhiêu thứ, mỗi chỗ là thứ gì.
3. **Thứ được đưa ra.** Loại gì, để bạn biết cầm nó đi làm gì tiếp.

Phần thân — cái bạn chưa mở ra xem ở bài trước — nằm **ngoài** chữ ký. Đó là chỗ
người viết hàm lo, không phải chỗ người gọi hàm lo.
::::

::::example{#ba-dong-la-du}
Byte viết một hàm mới toanh, và cố tình không cho bạn xem phần thân. Byte chỉ đưa
một tấm phiếu ba dòng:

```text
tên:      tien_ship
đưa vào:  một số — quãng đường, tính bằng ki-lô-mét
đưa ra:   một số — tiền ship, tính bằng đồng
```

Với đúng ba dòng ấy, bạn viết được lời gọi. Thử ngay:

```python title=readonly
def tien_ship(so_km):
    # Phần thân là việc của Byte. Lấy tay che đoạn này lại cũng không sao.
    return 15000 + so_km * 4000

print(tien_ship(4))
```

Máy in ra:

```text
31000
```

Nếu bạn tò mò mà nhìn trộm phần thân thì cũng ra đúng con số đó:
15000 + 4 × 4000 = 31000. Nhưng hãy để ý chuyện quan trọng hơn — **bạn không cần
phải nhìn**. Tấm phiếu đã nói đủ để viết lời gọi cho đúng.

Và đây là chỗ chữ ký khác hẳn một lời khuyên: nó là một **lời hứa**. Ngày mai
Byte sửa cách tính tiền ship, viết lại phần thân từ đầu — lời gọi `tien_ship(4)`
của bạn vẫn chạy, vì tấm phiếu không đổi. Chừng nào ba dòng ấy còn nguyên, thứ
bạn viết còn nguyên.
::::

::::predict{#bo-qua-dong-thu-ba commitOnce}
Byte dựng câu báo giá bằng dấu `+`, và trong lúc gõ thì bỏ qua dòng thứ ba của
tấm phiếu — dòng nói `tien_ship` đưa ra một **số**.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
def tien_ship(so_km):
    return 15000 + so_km * 4000

print("Ship 4 km hết " + tien_ship(4) + " đồng")
```

:::opt{correct}
Máy dừng lại và báo `TypeError`, không in ra dòng nào
:::

:::opt
In ra `Ship 4 km hết 31000 đồng` — máy tự đổi con số thành chữ rồi nối lại
::why
Gần đúng ở chỗ bạn dựa vào một chuyện có thật và bạn đã thấy nó hàng trăm lần:
con số vẫn hiện ra giữa một câu chữ ngon lành.

Chỗ lệch là **ở đâu** thì có quy ước đổi ấy. Bên trong cặp `{ }` của f-string thì
có: đó chính là việc f-string sinh ra để làm. Còn dấu `+` giữa một chuỗi và một
số thì không có quy ước nào cả — đúng như bài `TypeError` ở Realm 0 đã nói. Hai
chỗ nhìn giống nhau, luật khác nhau.
::
:::

:::opt
In ra `Ship 4 km hết  đồng` — máy bỏ qua đúng phần nó không nối được
::why
Gần đúng ở chỗ bạn mong chương trình làm được tới đâu hay tới đó, và mong ấy rất
tự nhiên: hai đầu câu đều là chuỗi, nối chúng lại có gì khó đâu.

Chỗ lệch nằm ở tính nết của máy, thứ Realm 0 đã chốt từ rất sớm: máy không tự bổ
sung, và cũng không tự bớt đi. Gặp chỗ không có quy ước, nó dừng và nói ra chứ
không lặng lẽ để lại một khoảng trống. Một khoảng trống lặng lẽ mới là thứ đáng
sợ — nó trông y như một câu trả lời thật.
::
:::

:::opt
Máy báo `ValueError` — đúng loại việc là nối chuỗi, nhưng nội dung không hợp
::why
Gần đúng ở chỗ bạn còn nhớ cả hai anh em lỗi và nhớ đúng cách phân biệt chúng:
một bên sai **loại**, một bên đúng loại mà sai **nội dung**.

Chỗ lệch là ở đây hỏng cái nào. `ValueError` xảy ra khi thứ đưa vào đúng kiểu mà
nội dung không dùng được — `int("hai mươi")` là ví dụ kinh điển. Còn lần này thứ
bên phải dấu `+` là một con số nguyên vẹn, hoàn toàn hợp lệ; cái hỏng là **kiểu**
của nó không đi cùng chuỗi được. Nên máy chọn `TypeError`.
::
:::
::::

::::explain{#chu-ky-cua-nhung-ham-ban-da-quen}
Đọc ngược lại những hàm bạn đã dùng, lần này viết ra thành phiếu:

| tên | đưa vào (theo thứ tự) | đưa ra |
|---|---|---|
| `len` | một chuỗi | một số nguyên — số ký tự |
| `int` | một chuỗi chữ số | một số nguyên |
| `round` | một số có phần lẻ | một số nguyên |
| `range` | hai số: điểm khởi hành, rồi chỗ dừng | dãy số cho `for` đi qua |
| `tien_ship` | một số — quãng đường | một số — tiền ship |

Hàng `range` là hàng đáng dừng lại, vì nó là hàng đầu tiên có **hai** thứ đưa
vào. Và ở đó, cụm chữ *theo thứ tự* mới lộ hết sức nặng của nó.

`range(8, 15)` và `range(15, 8)` gồm y hệt hai con số ấy. Bạn đã thấy ở mạch
trước: cái đầu chạy bảy lượt, cái sau chạy không lượt nào. Máy không đoán rằng
bạn muốn đi từ số nhỏ lên số lớn. Nó đọc **chỗ đứng**: ô thứ nhất là điểm khởi
hành, ô thứ hai là chỗ dừng, và nó làm đúng như thế.

Nên dòng thứ hai của chữ ký không phải "cần hai con số". Nó là "cần hai con số,
**và đây là ô nào**".

Gộp cả bài lại thành một câu:

> Biết tên, biết đưa vào những gì theo thứ tự nào, biết nhận về thứ gì — là gọi
> được. Không biết một trong ba thì hoặc máy dừng, hoặc tệ hơn: máy chạy ngon
> lành và cho ra một con số sai.

Câu "tệ hơn" ấy chưa xảy ra với bạn lần nào. Nó sẽ xảy ra, và mạch này sẽ dựng
sẵn chỗ để nó xảy ra một cách an toàn.
::::

::::code{#ba-tam-phieu}
Byte gửi ba tấm phiếu cùng một lúc. Ba hàm này **nhận vào y hệt nhau** — đều là
một con số ki-lô-mét — nên thứ duy nhất phân biệt chúng là dòng thứ nhất (tên) và
dòng thứ ba (thứ đưa ra).

```text
tên: tien_ship   đưa vào: số km   đưa ra: một số — tiền ship, đơn vị đồng
tên: gio_giao    đưa vào: số km   đưa ra: một số — thời gian giao, đơn vị phút
tên: ma_don      đưa vào: số km   đưa ra: một chuỗi — mã đơn hàng
```

Đơn hôm nay đi 4 ki-lô-mét. Điền ba chỗ trống cho bản in ra đúng.

```python title=starter
def tien_ship(so_km):
    return 15000 + so_km * 4000

def gio_giao(so_km):
    return 20 + so_km * 6

def ma_don(so_km):
    return f"DH-{so_km}KM"

tien = ___
phut = ___
ma = ___

print(f"Đơn {ma}: ship 4 km hết {tien} đồng, giao trong {phut} phút")
```

```python title=solution
def tien_ship(so_km):
    return 15000 + so_km * 4000

def gio_giao(so_km):
    return 20 + so_km * 6

def ma_don(so_km):
    return f"DH-{so_km}KM"

tien = tien_ship(4)
phut = gio_giao(4)
ma = ma_don(4)

print(f"Đơn {ma}: ship 4 km hết {tien} đồng, giao trong {phut} phút")
```

```python title=test
# Ba tấm phiếu, ba phép kiểm. Hai con số cố ý khác xa nhau để một lời gọi đặt
# nhầm tên lộ ra ngay, và cái tên `ma` phải giữ một CHUỖI chứ không phải số —
# đó là chỗ dòng thứ ba của phiếu nói thật.
assert tien == 31000, "phiếu tien_ship đưa ra tiền ship; đi 4 km thì con số ấy là 31000 đồng"
assert phut == 44, "phiếu gio_giao đưa ra thời gian; đi 4 km thì con số ấy là 44 phút, không phải tiền"
assert ma == "DH-4KM", "phiếu ma_don đưa ra một chuỗi mã đơn; đi 4 km thì mã ấy là DH-4KM"
```

:::hints
- kind: attention
  body: Ba cái tên bên trái dấu bằng đã nói sẵn chúng muốn giữ thứ gì — tiền, phút, mã đơn. Đối chiếu từng cái với dòng "đưa ra" của ba tấm phiếu; chỉ có đúng một phiếu hợp mỗi chỗ.
- kind: strategy
  body: Mỗi chỗ trống là một lời gọi có hình dạng `tên(thứ đưa vào)`. Cả ba phiếu đều nhận vào số ki-lô-mét, và đơn hôm nay đi 4 ki-lô-mét, nên thứ đặt trong ngoặc giống nhau ở cả ba dòng. Thứ khác nhau là cái tên đứng trước ngoặc.
- kind: one-line
  body: "Lần lượt ba chỗ trống là `tien_ship(4)`, `gio_giao(4)` và `ma_don(4)`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Đơn DH-4KM: ship 4 km hết 31000 đồng, giao trong 44 phút
- tier: static
  onFail: mỗi chỗ trống phải là một lời gọi tới đúng hàm trên phiếu, không phải kết quả chép tay
  requireAst:
  - kind: uses-call, target: tien_ship, min: 1
  - kind: uses-call, target: gio_giao, min: 1
  - kind: uses-call, target: ma_don, min: 1
  forbidAst:
  - kind: has-literal, target: 31000
  - kind: has-literal, target: 44
  - kind: has-literal, target: DH-4KM
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba lời gọi, ba tấm phiếu. Mình chưa hé cho bạn xem thân hàm nào cả.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ba điều đó bạn lấy từ đâu? Bạn không viết hàm ấy, cạnh bàn cũng không có quyển
sách nào. Hỏi ai bây giờ?

Tấm phiếu `tien_ship` là do Byte tự tay đưa cho bạn. Nhưng `round` thì không ai
đưa phiếu cả — bạn đã dùng nó từ mạch trước, và tới hôm nay mới biết ba dòng ấy
đáng ra phải tồn tại ở đâu đó. Chúng ở đâu?

Bài sau đi tìm.
::::

::::checkpoint{mastery=0.8}
::::
