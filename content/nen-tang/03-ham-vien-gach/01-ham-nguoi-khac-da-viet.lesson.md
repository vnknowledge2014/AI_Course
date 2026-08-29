---
id: nen-tang.ham-vien-gach.ham-nguoi-khac-da-viet
title: Hàm bạn chưa từng mở ra xem
summary: len, int, round, input đều là hàm do người khác viết — bạn gọi được chúng bằng cách tin cái tên, không đọc bên trong.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.builtin-function]
requires: [core.function-call, core.function-return, core.len, core.int-cast, core.round]
concepts: [core.ham, core.ten-co-san]
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
Mình đếm chữ giúp bạn cả trăm lần rồi. Bạn chưa hỏi mình đếm bằng cách nào.
::::

::::explain{#cai-ten-ban-chua-mo-ra-xem}
Mạch trước khép lại bằng một câu hỏi về việc đặt tên cho cả một đoạn việc. Realm 0
đã đưa bạn cái tên ấy rồi: bạn viết `def gia_to(co_to):`, gọi `gia_to("vừa")`, và
cả nhóm lệnh nằm trong đó chạy.

Nhưng ngay cạnh chuyện quen thuộc ấy có một chuyện lạ, lạ tới mức chưa lần nào
bạn dừng lại nhìn nó.

Bạn đã gõ `len("phở")` và nhận về `3`. Bạn đã gõ `int("25")` và nhận về số 25.
Bạn đã gõ `round(45000.7)` và nhận về `45001`. Bạn đã gõ `input()` và chương
trình dừng lại chờ bạn.

Bây giờ mở lại đúng những file ấy, và tìm giúp Byte một dòng: dòng `def len(...)`.

Nó không có ở đó. Không có trong file của bạn, không có trong bất cứ dòng nào bạn
từng gõ. Bạn chưa một lần viết nó, chưa một lần đọc nó — vậy mà bạn dùng nó suốt
hai Realm vừa rồi, và nó chưa sai lần nào.

Đó là câu hỏi mở đầu cả mạch này. Nó không phải câu hỏi về cú pháp.
::::

::::example{#hai-thu-cung-mot-hinh-dang}
Đặt hai dòng cạnh nhau. Một dòng gọi hàm **bạn** viết, một dòng gọi thứ kia.

```python title=readonly
def gia_to(co_to):
    if co_to == "vừa":
        return 45000
    return 0

print(gia_to("vừa"))
print(len("phở"))
```

Máy in ra:

```text
45000
3
```

Hai dòng cuối có cùng một hình dạng, đúng từng mảnh một:

| mảnh | dòng của bạn | dòng kia |
|---|---|---|
| một cái tên | `gia_to` | `len` |
| một cặp ngoặc | `( )` | `( )` |
| thứ đưa vào trong ngoặc | `"vừa"` | `"phở"` |
| thứ nhận được sau đó | `45000` | `3` |

Bạn biết dòng bên trái hoạt động ra sao, vì chính bạn viết phần thân của nó.
Dòng bên phải thì không: phần thân của `len` nằm ở một nơi bạn chưa mở bao giờ.

Và điều đáng nói là **chuyện đó không cản trở bạn một chút nào**. Bạn vẫn gọi
đúng, vẫn nhận đúng, vẫn ghép được con số ấy vào câu tiếp theo.
::::

::::predict{#len-tron-troi commitOnce}
Byte thử một chuyện: viết `len` một lần **có** ngoặc, một lần **không** ngoặc.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
mon = "phở"

print(len)
print(len(mon))
```

:::opt{correct}
Hai dòng: `<built-in function len>` rồi `3`
:::

:::opt
Hai dòng: `len` rồi `3`
::why
Gần đúng ở chỗ bạn áp một luật đã học và áp không sai một chữ: `print` in ra thứ
mà cái tên đang giữ, chứ không in ra chính cái tên.

Chỗ lệch nằm ở thứ mà `len` đang giữ. Theo bài "chữ và tên", không có nháy nghĩa
là máy phải đi tìm một cái tên — và lần này nó tìm **thấy**. Thứ nó tìm thấy
không phải chữ "len", mà là một hàm; nên máy tự mô tả cái hàm ấy ra:
`<built-in function len>` — nguyên văn là "hàm có sẵn tên len".
::
:::

:::opt
Máy báo lỗi ở dòng `print(len)` vì thiếu cặp ngoặc sau `len`
::why
Gần đúng ở chỗ bạn nhớ rất kỹ hình dạng quen thuộc: từ Realm 0 tới giờ, `len`
lúc nào cũng đi kèm một cặp ngoặc, nên `len` đứng trơ trọi trông như một câu
viết dở.

Chỗ lệch: cặp ngoặc không phải một phần của cái tên. Nó là lệnh **"chạy đi"**.
Bỏ ngoặc đi thì bạn mới chỉ *nhắc tới* cái hàm chứ chưa bảo nó làm gì — mà nhắc
tới một cái tên có thật thì không có gì sai cả, y như `print(mon)` nhắc tới `mon`.
::
:::

:::opt
Một dòng duy nhất: `3` — dòng `print(len)` không in gì, vì `len` chưa được gọi
::why
Gần đúng ở chỗ khó nhất, và bạn nhận ra nó rất chuẩn: `len` chưa được gọi thật.
Không có cặp ngoặc thì không có lần chạy nào.

Chỗ lệch nằm ở chuyện `print` cần gì để in. Nó in ra thứ nó nhận được, và lần
này nó nhận được một thứ có thật — nguyên cái hàm. "Chưa được gọi" không có
nghĩa là "không có gì": cái hàm vẫn nằm đó, vẫn có tên, vẫn mô tả được.
::
:::
::::

::::explain{#ham-co-san}
Dòng `<built-in function len>` là máy tự khai ra hai chuyện.

**Thứ nhất: `len` là một hàm.** Cùng một loại với `gia_to` bạn viết ở Realm 0 —
có tên, nhận thứ đưa vào, đưa ra một kết quả.

**Thứ hai: nó *có sẵn*.** Python mang nó theo từ trước khi file của bạn được
mở. Có người đã viết nó, đã thử nó, đã đóng gói nó lại; và từ đó ai gõ `len` cũng
gọi được, kể cả người không biết một dòng nào về bên trong.

Người ta gọi nhóm này là **hàm có sẵn**. Bạn đã dùng ít nhất bốn cái:

| tên | bạn đưa vào | bạn nhận về |
|---|---|---|
| `len` | một chuỗi | số ký tự trong chuỗi ấy |
| `int` | một chuỗi chữ số | chính con số đó, dùng tính toán được |
| `round` | một số có phần lẻ | số nguyên gần nó nhất |
| `input` | (không đưa gì) | thứ người ngồi trước máy vừa gõ |

Nhìn cả bảng một lượt thì thấy một chuyện: để dùng đúng bốn cái tên ấy, chưa lần
nào bạn cần biết bên trong chúng có gì. Bạn tin cái tên, đưa vào đúng thứ nó
cần, và cầm lấy thứ nó đưa ra.

> Cách nói của giới lập trình: bạn dùng chúng như một **hộp đen** — nhìn được
> hai đầu, không nhìn vào trong. Đó không phải chỗ thiếu sót của bạn; đó là cách
> chúng được làm ra để dùng.

Và chuyện này lớn hơn nó trông. Ai đó ở đâu đó viết `len` một lần, còn hàng
triệu người gõ `len(...)` mỗi ngày mà không ai phải viết lại. Cả mạch này sống
trên đúng một câu hỏi: **nhờ đâu mà tin được như vậy?**
::::

::::code{#goi-ba-hop-den}
Byte đưa ba mẩu dữ liệu và một bản báo cáo còn hở ba chỗ. Cả ba chỗ đều điền
bằng một lời gọi tới hàm **người khác đã viết** — bạn không phải viết `def` nào.

Ba mẩu dữ liệu cố ý khác kiểu nhau, nên mỗi chỗ trống hợp với đúng một cái tên
trong bảng vừa rồi.

```python title=starter
ten_mon = "Phở bò tái nạm"
gia_chu = "45000"
diem_trung_binh = 4.6

so_ky_tu = ___
gia_so = ___
diem_lam_tron = ___

print(f"Tên món dài {so_ky_tu} ký tự")
print(f"Hai tô hết {gia_so * 2} đồng")
print(f"Quán được {diem_lam_tron} sao")
```

```python title=solution
ten_mon = "Phở bò tái nạm"
gia_chu = "45000"
diem_trung_binh = 4.6

so_ky_tu = len(ten_mon)
gia_so = int(gia_chu)
diem_lam_tron = round(diem_trung_binh)

print(f"Tên món dài {so_ky_tu} ký tự")
print(f"Hai tô hết {gia_so * 2} đồng")
print(f"Quán được {diem_lam_tron} sao")
```

```python title=test
# Ba phép kiểm cho ba hộp đen. Con số nào cũng phải ĐI RA TỪ dữ liệu ở trên,
# không phải do bạn tự gõ vào — nên mỗi dòng dưới đây trượt ngay nếu chỗ trống
# được điền bằng một hằng số.
assert so_ky_tu == 14, "chuỗi 'Phở bò tái nạm' có 14 ký tự, kể cả hai dấu cách giữa các chữ — con số này phải do hàm đếm chữ đưa ra"
assert gia_so == 45000, "gia_chu đang là CHUỖI '45000'; gia_so phải là con số 45000 thật, vì dòng dưới còn đem nó nhân đôi"
assert gia_so * 2 == 90000, "nhân đôi một con số cho 90000; nếu gia_so vẫn là chuỗi thì phép nhân sẽ chép chuỗi ấy hai lần chứ không cộng tiền"
assert diem_lam_tron == 5, "4.6 nằm gần cọc 5 hơn cọc 4, nên số nguyên gần nó nhất là 5"
```

:::hints
- kind: attention
  body: Nhìn kiểu của ba mẩu dữ liệu ở trên trước. Một mẩu là chuỗi chữ, một mẩu là chuỗi chữ số, một mẩu là số có phần lẻ. Bảng hàm có sẵn ở phần trên có đúng một cái tên hợp với mỗi loại.
- kind: strategy
  body: Mỗi chỗ trống là một lời gọi có hình dạng `tên(thứ đưa vào)`, và thứ đưa vào là chính cái tên đứng ở nhóm ba dòng đầu — không phải chép lại giá trị của nó. Dòng thứ hai của bản báo cáo còn đem kết quả đi nhân đôi, nên thứ nó cầm phải là số chứ không phải chuỗi.
- kind: one-line
  body: "Lần lượt ba chỗ trống là `len(ten_mon)`, `int(gia_chu)` và `round(diem_trung_binh)`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Tên món dài 14 ký tự\nHai tô hết 90000 đồng\nQuán được 5 sao\s*$
- tier: output
  expect: Hai tô hết 90000 đồng
- tier: output
  expect: Quán được 5 sao
- tier: static
  onFail: mỗi chỗ trống phải là một lời gọi tới hàm có sẵn, đọc từ cái tên ở trên — không phải một con số chép tay
  requireAst:
  - kind: uses-call, target: len, min: 1
  - kind: uses-call, target: int, min: 1
  - kind: uses-call, target: round, min: 1
  - kind: uses-name, target: ten_mon, min: 1
  - kind: uses-name, target: gia_chu, min: 1
  - kind: uses-name, target: diem_trung_binh, min: 1
  forbidAst:
  - kind: has-literal, target: 14
  - kind: has-literal, target: 5
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba hộp đen, ba câu trả lời đúng. Bạn không mở cái nào ra cả.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn gọi `len("phở")` được vì đoán ra nó cần một thứ và đưa lại một số. Còn
`round` — một hàm lạ hoắc — bạn phải biết đúng mấy điều mới dám gọi?

Thử đếm thật. Lúc bạn gõ `round(45000.7)`, trong đầu bạn có sẵn những gì? Chắc
chắn không có phần thân của nó. Vậy phần còn lại gồm mấy món, và món nào bỏ đi
được, món nào bỏ đi là gọi sai?

Bài sau đếm cho hết.
::::

::::checkpoint{mastery=0.8}
::::
