---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.bat-tat-tung-o
title: Bật, tắt, hỏi từng ô một
summary: "`&` và `|` chạy đúng bảng bốn dòng mà `and`/`or` của Realm 1 đã dạy, chỉ khác là chúng chạy bảng đó cho từng cặp bit một, tám lần liền, thay vì cho một cặp Đúng/Sai."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [mem.bitwise-and-or]
requires: [mem.twos-complement, logic.and, logic.or]
concepts: [mem.and-tung-bit, mem.or-tung-bit, mem.byte-la-tam-cau-hoi]
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
Một byte còn là tám câu hỏi có–không đứng liền nhau. Hỏi cả tám cùng lúc.
::::

::::explain{#tam-cau-hoi-dung-lien-nhau}
Từ bài "một ô chỉ chứa có hoặc không" ở Realm 0, bạn đã biết bảng điện tám
công tắc của cô Bảy ghi lại được bằng đúng tám ký tự: `11011100`. Mỗi công
tắc là MỘT câu trả lời có–không riêng — bật là `1`, tắt là `0` — và tám câu
trả lời đó xếp cạnh nhau thì thành một byte.

Realm 1 dạy bạn hai từ nối gộp câu trả lời có–không: `and` đòi CẢ HAI vế
cùng đúng, `or` chỉ cần MỘT vế đúng. Cả hai đều chạy trên đúng một cặp
`True`/`False`.

Câu hỏi hôm nay: nếu có HAI byte — tức là hai dãy tám câu trả lời có–không
— đứng cạnh nhau theo từng cột, gộp chúng bằng `and`/`or` được không?

Thử ngay xem sao.
::::

::::example{#dung-lai-and-that}
Bảng điện tối nay của cô Bảy, viết theo đúng thứ tự tám công tắc của Realm
0 (đèn biển hiệu, đèn trong nhà, quạt trần, quạt đứng, tủ mát, nồi nước
dùng, đèn nhà sau, loa):

```python title=readonly
trang_thai_dang_bat = 0b11001100
```

Còn có một quy định tiết kiệm điện, ghi công tắc nào ĐƯỢC PHÉP bật qua
đêm:

```python title=readonly
duoc_phep_qua_dem = 0b10101010
```

Byte thử dùng lại `and` — đúng từ nối Realm 1 vừa nhắc — để hỏi "công tắc
nào vừa đang bật, vừa được phép":

```python title=readonly
print(trang_thai_dang_bat and duoc_phep_qua_dem)
```

Máy in ra `170`. Kiểm lại: `170` là `duoc_phep_qua_dem` — nguyên vẹn,
không đổi một bit nào. `and` không hề nhìn vào TỪNG cột. Nó chỉ hỏi đúng
MỘT câu có–không cho cả con số: "vế trái có phải số khác 0 không?" `170`
(khác `0`) là có, nên nó trả lại nguyên vẹn vế phải — đúng thói quen Python
mà Realm 1 đã dạy, chỉ là áp nhầm chỗ ở đây.

Muốn hỏi TỪNG cột một, cần một phép khác: `&`, đọc là **phép và theo từng
ô** — chạy đúng bảng bốn dòng của `and` cho MỖI cặp bit đứng cùng cột, tám
lần liên tiếp, độc lập với nhau.

```python title=readonly
print(bin(trang_thai_dang_bat & duoc_phep_qua_dem))
```

Máy in ra `0b10001000`.

Xếp hai dãy tám bit chồng lên nhau rồi soi từng cột:

```text title=readonly
trang_thai_dang_bat   1 1 0 0 1 1 0 0
duoc_phep_qua_dem     1 0 1 0 1 0 1 0
                      ---------------
                      1 0 0 0 1 0 0 0
```

Cột 1: `1` và `1`, cả hai đúng — theo bảng `and`, kết quả `1`. Cột 2: `1`
và `0` — một vế sai, kết quả `0`. Cứ thế cho cả tám cột. Đúng bảng bốn dòng
Realm 1 đã dạy, chỉ chạy nó TÁM LẦN thay vì một.

Đọc kết quả: đèn biển hiệu (cột 1) và tủ mát (cột 5) là hai công tắc duy
nhất vừa đang bật, vừa nằm trong danh sách được phép — an toàn, không cần
tắt.

`|` làm việc tương tự với `or`: **phép hoặc theo từng ô**, chạy bảng bốn
dòng của `or` cho từng cặp cột.

```python title=readonly
print(bin(trang_thai_dang_bat | duoc_phep_qua_dem))
```

Máy in ra `0b11101110` — công tắc nào bật sẵn hoặc được phép bật (hoặc cả
hai) thì cột đó lên `1`; chỉ cột nào cả hai đều `0` mới còn `0`.
::::

::::predict{#and-that-hay-and-gia commitOnce}
Byte đổi sang hai con số khác: `204` (`0b11001100`) và `170`
(`0b10101010`) — quen mắt vì cùng hình dạng bảng điện, chỉ đổi số.

```python
a = 204
b = 170

print(a and b)
print(a & b)
print((a and b) == (a & b))
```

**Trước khi bấm chạy**, bạn đoán ba dòng trên in ra gì?

:::opt{correct}
170, rồi 136, rồi False
:::

:::opt
170, rồi 170, rồi True
::why
Gần đúng ở chỗ dòng đầu: `a and b` đúng là `170`, vì `204` khác `0` nên
`and` trả nguyên vẹn vế phải — y hệt ví dụ bảng điện vừa rồi.

Chỗ lệch là bạn cho rằng `&` cũng làm y vậy. Không: `&` soi TỪNG CỘT. `204`
là `11001100`, `170` là `10101010`, so từng cột theo bảng `and` ra
`10001000`, tức `136` — khác hẳn `170`. Hai dòng đầu KHÔNG bằng nhau, nên
dòng ba phải là `False`.
::
:::

:::opt
136, rồi 136, rồi True
::why
Gần đúng ở chỗ bạn tính đúng phép `&`: `204 & 170` đúng là `136`, và bạn
đọc từng cột chuẩn xác.

Chỗ lệch nằm ở dòng MỘT — `a and b`. `and` không soi cột nào cả; nó chỉ hỏi
"vế trái khác `0` không", thấy có thì trả nguyên `b`, tức `170`, không
phải `136`. Hai phép này cho ra hai con số khác nhau — đó chính là điều bài
này muốn bạn thấy tận mắt.
::
:::

:::opt
0, rồi 136, rồi False
::why
Gần đúng ở chỗ bạn cảnh giác đúng: nghi rằng `and`/`or` không hợp với việc
soi từng bit, nên đoán nó cho kết quả kỳ quặc — dòng suy luận ấy đúng tinh
thần bài này.

Chỗ lệch là `and` không trả về `0` hay báo lỗi gì cả trong ca này. Nó chỉ
kiểm ĐÚNG MỘT LẦN: vế trái (`204`) có khác `0` không. Có, nên nó trả về
nguyên vế phải, tức `170`, chứ không phải `0`.
::
:::
::::

::::code{#den-nao-can-kiem}
Cô Bảy muốn một hàm hỏi đúng câu hỏi mà `&` trả lời được: công tắc nào
ĐANG bật VÀ CŨNG được phép bật qua đêm.

```python title=starter
def cong_tac_an_toan(dang_bat, duoc_phep):
    """Trả về dãy bit: cột nào 1 nghĩa là công tắc đó vừa đang bật, vừa được phép."""
    return ___


print(bin(cong_tac_an_toan(0b11001100, 0b10101010)))
print(bin(cong_tac_an_toan(0b11111111, 0b00000000)))
```

```python title=solution
def cong_tac_an_toan(dang_bat, duoc_phep):
    """Trả về dãy bit: cột nào 1 nghĩa là công tắc đó vừa đang bật, vừa được phép."""
    return dang_bat & duoc_phep


print(bin(cong_tac_an_toan(0b11001100, 0b10101010)))
print(bin(cong_tac_an_toan(0b11111111, 0b00000000)))
```

```python title=test
# Năm cảnh. Cảnh cuối là bảng điện của bài — 204 và 170 — nơi `and` (logic
# Realm 1, sai tầng) và `|` (đúng bảng, sai từ nối) đều lộ mặt so với lời
# giải đúng.
assert cong_tac_an_toan(0b11001100, 0b10101010) == 0b10001000, "soi từng cột theo bảng VÀ: chỉ cột nào cả hai cùng 1 mới còn 1"
assert cong_tac_an_toan(0b11111111, 0b00000000) == 0, "vế phải toàn 0 — không cột nào được phép — nên không cột nào an toàn, dù vế trái bật hết"
assert cong_tac_an_toan(0b00000000, 0b11111111) == 0, "vế trái toàn 0 — không công tắc nào đang bật — thì chẳng có gì để mà an toàn"
assert cong_tac_an_toan(0b11111111, 0b11111111) == 0b11111111, "cả hai toàn 1 thì mọi cột đều vừa bật vừa được phép"
assert cong_tac_an_toan(204, 170) == 136, "204 và 170 là đúng cặp bảng điện của bài — kết quả phải là 136, không phải 170 (đó là kết quả của and, không phải của phép soi từng cột)"
```

:::hints
- kind: attention
  body: Chỗ trống là toàn bộ dòng `return`. Câu hỏi "vừa đang bật, VỪA được phép" là câu hỏi CẢ HAI cùng đúng — đúng tinh thần `and` của Realm 1, nhưng lần này phải chạy cho từng cột.
- kind: strategy
  body: Đừng dùng `and` — nó chỉ hỏi một lần cho cả con số, không soi từng cột. Phép soi từng cột theo bảng VÀ có ký hiệu riêng, một dấu duy nhất, khác hẳn chữ `and` bốn ký tự.
- kind: one-line
  body: 'Điền `dang_bat & duoc_phep`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^0b10001000\n0b0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một bảng bốn dòng, chạy đủ tám lần, mỗi ô một lần.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ô còn làm được một việc nữa mà bạn chưa thử: DI CHUYỂN. Bài "đọc một dãy
bit ra số" ở đầu track này đã chốt: mỗi cột của một số nhị phân đứng ở một
luỹ thừa của 2 — cột này gấp đôi cột ngay bên phải nó. Track T2.1, bài
"bảng vị trí kéo sang phải" đã dạy bạn kéo cả bảng vị trí của hệ MƯỜI sang
phải là CHIA cho mười.

Nếu kéo cả dãy bit của hệ HAI sang một hướng — không phải sang phải, mà
sang TRÁI — chuyện gì xảy ra với giá trị của nó? Hệ mười kéo một cột là
nhân mười; hệ hai kéo một cột thì nhân theo mấy?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
