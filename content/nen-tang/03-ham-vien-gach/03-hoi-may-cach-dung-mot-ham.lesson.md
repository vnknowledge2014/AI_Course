---
id: nen-tang.ham-vien-gach.hoi-may-cach-dung-mot-ham
title: Hỏi thẳng cái máy
summary: help() in ra chữ ký kèm mô tả — bản hợp đồng của một hàm nằm sẵn trong máy, không phải đi tìm đâu xa.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.help-builtin]
requires: [core.function-signature, core.builtin-function]
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
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Tấm phiếu của `round` không nằm trên bàn bạn. Nó nằm trong máy. Hỏi đi.
::::

::::explain{#hoi-ai-bay-gio}
Bài trước để lại một chỗ hụt rất cụ thể. Tấm phiếu của `tien_ship` là do Byte tự
tay đưa. Còn `round` thì chưa ai đưa phiếu cả — bạn dùng nó từ mạch trước, và tới
hôm nay mới biết ba dòng ấy đáng ra phải nằm ở đâu đó.

Ở đâu đó là ở đâu?

Câu trả lời gần hơn bạn tưởng, và nó nằm đúng chỗ bài đầu mạch này đã chỉ vào.
Nhớ lại: gõ `print(len)`, máy in ra `<built-in function len>`. Máy **biết** `len`
là một hàm có sẵn. Nó biết mà không phải tra ở đâu, vì cái hàm ấy đang nằm ngay
trong máy, cùng với mọi thứ dán trên mình nó.

Vậy nếu máy đã biết tới mức mô tả được một câu, thì hỏi kỹ hơn một chút xem sao.

Python có sẵn một hàm dành riêng cho việc hỏi ấy: **`help`**.
::::

::::example{#hoi-ve-round}
Gõ đúng một dòng:

```python title=readonly
help(round)
```

Máy in ra:

```text
Help on built-in function round in module builtins:

round(number, ndigits=None)
    Round a number to a given precision in decimal digits.

    The return value is an integer if ndigits is omitted or None.  Otherwise
    the return value has the same type as the number.  ndigits may be negative.
```

Đây là tấm phiếu bạn thiếu ở bài trước, và nó ở trong máy suốt từ đầu.

Chữ tiếng Anh vì người viết `round` viết bằng tiếng Anh; đọc từng khối một:

- **Dòng đầu** — "Trợ giúp về hàm có sẵn `round`, thuộc nhóm `builtins`". Nó xác
  nhận đúng thứ bài 1 nói: `round` là hàm, và là hàm có sẵn.
- **Dòng thụt vào một nấc: `round(number, ndigits=None)`** — đây chính là **dòng
  chữ ký**. Tên là `round`, và trong ngoặc là các ô phải điền, viết theo đúng thứ
  tự phải điền.
- **Đoạn cuối** — phần **mô tả**: "Làm tròn một con số về độ chính xác cho
  trước", rồi một câu nói thứ đi ra là số nguyên. Đó là dòng thứ ba của tấm
  phiếu, viết bằng câu chữ cho người đọc.

Có một chỗ trong dòng chữ ký lạ mắt: ô thứ hai viết `ndigits=None` chứ không phải
một cái tên trơ trọi. Đó là một ô bạn **được phép bỏ trống** — và bạn đã bỏ trống
nó suốt từ mạch trước tới giờ mà chưa lần nào bị máy phàn nàn. Dấu `=` ở đó nghĩa
là gì, mạch này còn quay lại nói cho hết. Hôm nay, đọc được tên hàm và ô đầu tiên
là đã đủ để gọi đúng như bạn vẫn gọi.
::::

::::predict{#help-mot-ham-cua-ban commitOnce}
Byte viết một hàm mới tinh, đặt ngay trong file, rồi hỏi máy về chính nó.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
def phi_ship(so_km):
    return 15000 + so_km * 4000

help(phi_ship)
```

:::opt{correct}
Một dòng tiêu đề, rồi đúng dòng `phi_ship(so_km)` — không có đoạn mô tả nào
:::

:::opt
Máy báo lỗi, vì `phi_ship` không nằm trong kho hàm có sẵn nên `help` không biết nó
::why
Gần đúng ở chỗ bạn hình dung `help` như một quyển sổ tra cứu: có tên trong sổ thì
tra được, không có thì chịu. Hình dung ấy hợp lý, và với nhiều công cụ khác thì
nó đúng.

Chỗ lệch: `help` không tra sổ. Nó nhìn thẳng vào thứ bạn đặt trong ngoặc. Mà thứ
ấy có thật trong máy — bài 1 đã cho thấy `print(len)` mô tả được cái hàm đang
nằm đó, và hàm bạn vừa viết cũng nằm đó y như vậy, chẳng kém cạnh gì.
::
:::

:::opt
In ra cả phần thân: dòng `return 15000 + so_km * 4000`
::why
Gần đúng ở chỗ bạn suy luận từ một sự thật: máy đang cầm phần thân trong tay, nên
nó *có thể* in ra. Và chữ "trợ giúp" thì nghe như là cho xem tất cả.

Chỗ lệch nằm ở việc `help` in ra cái gì. Nó in bản **hợp đồng**, không in cách
làm — đúng cái ranh giới bài 1 vẽ ra: người gọi nhìn hai đầu, không nhìn vào
trong. Nếu `help` bày cả phần thân thì nó phá chính ranh giới ấy, và cả mạch này
mất chỗ đứng.
::
:::

:::opt
In ra y hệt `help(round)`: dòng chữ ký kèm một đoạn mô tả
::why
Gần đúng ở một điểm rất chuẩn: `help` đối xử với mọi hàm như nhau, không thiên vị
hàm có sẵn hơn hàm bạn viết. Nó làm đúng như thế thật.

Chỗ lệch là đoạn mô tả từ đâu ra. `help` không tự nghĩ ra câu nào cả — nó **đọc
lại** đoạn mô tả mà chính cái hàm mang theo mình. `round` có mang theo một đoạn.
`phi_ship` thì chưa mang theo gì, nên chỗ ấy ra trống. Cùng một `help`, hai kết
quả khác nhau, và khác nhau ở phía cái hàm chứ không phải phía `help`.
::
:::
::::

::::explain{#hop-dong-nam-san-trong-may}
Ba chuyện gọn lại từ những gì vừa thấy.

**`help` cũng là một hàm có sẵn.** Cùng hình dạng với mọi cái tên ở bài 1: một
cái tên, một cặp ngoặc, một thứ đưa vào. Bạn dùng nó y như dùng `len`, và cũng
chưa từng mở nó ra xem.

**Thứ đưa vào là cái tên hàm, viết KHÔNG có ngoặc.** `help(round)` chứ không phải
`help(round())`. Cặp ngoặc là lệnh "chạy đi" — thêm nó vào là bạn bảo máy chạy
`round` trước, mà `round` chạy trơ trọi thì chẳng có con số nào để làm tròn. Ở
đây bạn đang đưa cho `help` **chính cái hàm**, không phải kết quả của nó. Đúng
cảnh `print(len)` ở bài 1, chỉ khác cái tên đứng ngoài.

**`help` làm việc bằng cách in thẳng lên màn hình.** Nó không đưa gì vào một cái
tên để bạn cầm đi dùng tiếp; nó bày ra cho bạn đọc. Đó là công cụ dành cho người,
không dành cho phần còn lại của chương trình.

Và đây là chỗ đáng nhớ nhất của bài: **bản hợp đồng không nằm trong một quyển
sách nào cả — nó đi kèm cái hàm, nằm sẵn trong máy.** Bạn không cần mạng, không
cần sách, không cần hỏi ai. Gặp một cái tên lạ, hỏi thẳng cái máy đang chạy nó.

> Chỗ dễ vấp: `help` in ra rất nhiều dòng, và người mới hay đọc từ trên xuống
> rồi nản. Nhìn có mục đích thì nhanh hơn — nhưng phải nhìn đủ **hai** chỗ, vì
> tấm phiếu ba điều của bài 2 nằm rải ở hai nơi trong bản in này:
>
> - **tên hàm và những ô phải điền** nằm gọn trên đúng một dòng: dòng có tên
>   hàm kèm cặp ngoặc;
> - **thứ hàm đưa ra** — điều thứ ba, thứ mà bài 2 đã cho thấy bỏ qua là máy
>   nổ — nằm trong mấy dòng chữ bên dưới, thường ở câu bắt đầu bằng *trả về*
>   hoặc *đưa ra*.
>
> Đọc hai chỗ ấy là đủ. Phần còn lại mới là chuyện đọc sau cũng được.
::::

::::code{#hoi-roi-goi}
Đúng nhịp bạn sẽ dùng suốt đời viết code: **hỏi trước, gọi sau**.

Đoạn dưới có một hàm và hai chỗ trống. Chỗ thứ nhất là câu hỏi, chỗ thứ hai là
lời gọi dựa trên câu trả lời — và bạn làm được cả hai mà không đọc phần thân.

```python title=starter
def phi_ship(so_km):
    return 15000 + so_km * 4000

# 1) Hỏi máy bản hợp đồng của `phi_ship`. Che phần thân ở trên lại cũng được.
___

# 2) Máy vừa nói hàm này nhận đúng một thứ. Gọi nó cho quãng 4 ki-lô-mét.
phi = ___

print(f"Ship 4 km hết {phi} đồng")
```

```python title=solution
def phi_ship(so_km):
    return 15000 + so_km * 4000

# 1) Hỏi máy bản hợp đồng của `phi_ship`. Che phần thân ở trên lại cũng được.
help(phi_ship)

# 2) Máy vừa nói hàm này nhận đúng một thứ. Gọi nó cho quãng 4 ki-lô-mét.
phi = phi_ship(4)

print(f"Ship 4 km hết {phi} đồng")
```

```python title=test
# Hai chỗ trống, hai phép kiểm khác loại nhau.
#
# Chỗ thứ nhất chấm bằng chính màn hình: dòng `phi_ship(so_km)` chỉ hiện ra khi
# câu hỏi được đặt đúng, và không có cách nào gõ ra nó bằng tay ở chỗ ấy.
#
# Chỗ thứ hai chấm bằng con số dưới đây — con số phải ĐI RA TỪ lời gọi, nên một
# hằng số chép tay sẽ bị luật static chặn lại.
assert phi == 31000, "phi_ship nhận số ki-lô-mét và đưa ra tiền ship; đi 4 km thì con số ấy là 31000 đồng"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất đứng một mình trên một dòng, không có dấu bằng nào — nghĩa là nó không cất kết quả vào đâu cả, nó chỉ bày một thứ ra màn hình. Cả bài này có đúng một cái tên làm việc đó.
- kind: strategy
  body: Dòng thứ nhất đưa cho công cụ hỏi **chính cái hàm**, nên trong ngoặc là cái tên trơ trọi, không kèm ngoặc của riêng nó. Dòng thứ hai thì ngược lại — ở đó bạn thật sự bảo hàm chạy, nên cái tên phải đi kèm ngoặc, và trong ngoặc là quãng đường của đơn hôm nay.
- kind: one-line
  body: "Chỗ trống thứ nhất là `help(phi_ship)`, chỗ thứ hai là `phi_ship(4)`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  expect: phi_ship(so_km)
- tier: output
  expect: Ship 4 km hết 31000 đồng
- tier: static
  onFail: dòng đầu phải hỏi máy về chính cái hàm, dòng sau phải gọi nó — con số không được chép tay
  requireAst:
  - kind: uses-call, target: help, min: 1
  - kind: uses-call, target: phi_ship, min: 1
  forbidAst:
  - kind: has-literal, target: 31000
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hỏi một dòng, gọi một dòng. Bản hợp đồng nằm sẵn trong máy từ đầu tới giờ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Gõ `help(gia_to)` với hàm bạn tự viết ở Realm 0 xem — hàm tra bảng giá tô phở ấy.
Máy in ra được chữ ký, nhưng chỗ mô tả trống trơn. Ai phải viết chỗ đó?

Và nghĩ thêm một nhịp nữa cho thấm. Người viết `round` chưa từng gặp bạn, chưa
từng biết bạn tồn tại, vậy mà đoạn mô tả của họ vẫn tới được tay bạn hôm nay,
nguyên vẹn, đúng lúc bạn cần. Còn `gia_to` của bạn thì tới tay ai bây giờ?

Bài sau nói tới chỗ ấy.
::::

::::checkpoint{mastery=0.8}
::::
