---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.bon-bit-mot-chu-so
title: Vì sao lập trình viên đọc hex
summary: Một nhóm bốn bit vừa khít đúng một chữ số hệ mười sáu — không thừa không thiếu — nên một byte luôn viết gọn thành đúng hai chữ số.
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [mem.hex]
requires: [mem.byte-range, mem.binary-to-int, mem.int-to-binary, core.floor-division, core.modulo, core.function-def, core.function-parameter, core.function-return, core.list-index, core.string-concat, core.string-literal, core.variable, core.print-variable]
concepts: [mem.hex, mem.nhom-bon-bit]
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
Bạn vừa tự tính ra 255 — con số lớn nhất một byte chứa nổi. Giờ thử viết
dãy tám bit của nó ra giấy xem tay có mỏi không.
::::

::::explain{#tam-ky-tu-la-qua-dai}
255 viết dưới dạng bit là `11111111`. Tám ký tự, đọc cũng chưa khó lắm.
Nhưng byte của bạn hiếm khi chỉ đứng một mình. Một tấm ảnh nhỏ đã có tới
hàng nghìn byte, mỗi byte một dãy tám ký tự riêng.

Thử chép tay ba byte liền nhau: `11011010 00101011 11110000`. Đọc tới ký
tự thứ mười lăm thì mắt đã lẫn — chệch một chữ số `0` hay `1` thôi là ra
hẳn một byte khác, và không cách nào liếc qua mà biết ngay chỗ sai.

Cần một cách viết GỌN hơn, mà vẫn nói đúng dãy bit gốc — không làm tròn,
không mất thông tin, chỉ đổi cách trình bày.
::::

::::explain{#nhom-bon-bit-vua-khit}
Cách gọn ấy bắt đầu từ một câu hỏi: một **nhóm** bao nhiêu bit thì vừa
đủ để đứng riêng thành một ký hiệu?

Bài 1 của track này đã chỉ ra luật: thêm một bit thì số kiểu **gấp
đôi**. Một nhóm bốn bit thì có 2 × 2 × 2 × 2 = 16 kiểu — từ `0000` tới
`1111`, không kiểu nào ngoài phạm vi đó.

Mười sáu kiểu nghĩa là cần đúng mười sáu ký hiệu để đặt tên cho từng
kiểu — không thừa một cái, không thiếu một cái. Chữ số quen thuộc chỉ có
mười, từ `0` tới `9`. Thiếu sáu ký hiệu.

Người ta mượn sáu chữ cái đầu bảng chữ cái La-tinh để bù vào chỗ thiếu:
`a` mang giá trị 10, `b` là 11, cứ thế tới `f` là 15.

```text title=readonly
nhóm bit    giá trị    ký hiệu
0000        0          0
0001        1          1
...         ...        ...
1001        9          9
1010        10         a
1011        11         b
1100        12         c
1101        13         d
1110        14         e
1111        15         f
```

Cách viết dùng mười sáu ký hiệu này gọi là **hệ mười sáu** (tiếng Anh:
*hexadecimal*, hay gọi tắt *hex*). Một chữ số hệ mười sáu luôn đứng thay
cho ĐÚNG một nhóm bốn bit, không hơn không kém — vì mười sáu kiểu của
nhóm bit khớp thẳng vào mười sáu ký hiệu của hệ mười sáu. Đó không phải
một sự trùng hợp đẹp mắt. Đó là vì 16 chính là 2⁴ — luỹ thừa của 2, cùng
gốc với chuyện đếm bit.

Thử hệ tám (tám ký hiệu, `0`-`7`) thì không vừa khít như vậy: ba bit cho
đúng tám kiểu (2³), khớp với hệ tám thật, nhưng một byte có tám bit —
không chia hết cho ba. Hệ mười (mười ký hiệu) còn tệ hơn: không có số
nguyên lần bit nào gói đúng mười kiểu cả. Hệ mười sáu là hệ ĐẦU TIÊN mà
một nhóm bit tròn trịa (bốn) khớp thẳng số ký hiệu của chính nó.
::::

::::example{#tach-mot-byte-lam-hai}
Vì một byte có tám bit, và một chữ số hex vừa khít bốn bit, một byte
luôn tách gọn thành đúng HAI nhóm bốn bit — không dư, không thiếu ký tự
nào. Viết hex của một byte vì vậy luôn ra đúng hai chữ số.

Lấy byte `218` (một giá trị trong tầm 0-255 mà bài 4 đã chốt). Viết
thành bit trước:

```text title=readonly
218 = 11011010
```

Tách làm hai nhóm bốn:

```text title=readonly
1101   1010
```

Đọc từng nhóm theo đúng bảng trên: `1101` là 13, tức chữ số `d`. `1010`
là 10, tức chữ số `a`. Ghép lại: `da`.

```python title=readonly
print(bin(218))
print(hex(218))
```

```text title=readonly
0b11011010
0xda
```

Python tự làm đúng phép tách ấy cho bạn: `bin` in ra dạng bit (tiền tố
`0b`), `hex` in ra dạng hex (tiền tố `0x`). Cả hai tiền tố không phải
một phần của con số — chúng chỉ báo "đây là hệ nào" cho người đọc. Bỏ
tiền tố đi, phần còn lại `da` đúng là hai chữ số bạn vừa tính tay.

Byte lớn nhất — `255` — luôn là hai chữ số `ff` (cả hai nhóm bốn bit đều
toàn số 1: `1111` là 15, tức `f`). Byte `0` là `00`, không phải một chữ
số `0` trơn — nhóm bit đầu vẫn toàn số 0, và nó vẫn cần MỘT ký tự để
đứng vào chỗ của mình. "Luôn hai chữ số" áp dụng cả khi chữ số đó là
`0`.
::::

::::predict{#doan-hex-cua-mot-byte commitOnce}
Byte tách một byte khác thành hai nhóm bốn bit:

```text title=readonly
1011    0101
```

**Trước khi xem đáp án**, bạn đoán byte này viết theo hex là gì?

:::opt{correct}
`b5` — nhóm đầu là 11 (chữ `b`), nhóm sau là 5 (chữ `5`).
:::

:::opt
`115` — nhóm đầu là 11, nhóm sau là 5, ghép hai con số đó lại.
::why
Gần đúng ở chỗ hai giá trị 11 và 5 bạn tính đúng cả — đọc bảng vị trí
của từng nhóm không sai đâu.

Chỗ lệch nằm ở cách VIẾT giá trị 11. Một chữ số hex chỉ được là ĐÚNG MỘT
ký tự — đó chính là lý do bài này cần mượn `a` tới `f`. Viết `11` là
dùng hai ký tự cho một nhóm bit, mà một nhóm bit chỉ có quyền chiếm một
ký tự. Giá trị 11 phải viết bằng chữ `b`, không phải hai chữ số `1` với
`1`.
::
:::

:::opt
`5b` — nhóm đầu 11 thành `b`, nhóm sau 5 thành `5`, nhưng viết `5`
trước.
::why
Gần đúng ở chỗ cả hai ký tự `b` và `5` bạn chọn đúng — không sai giá trị
của nhóm bit nào.

Chỗ lệch là thứ tự. Nhóm bit đứng bên TRÁI (`1011`) là nhóm mang trọng
số lớn hơn — nó đọc trước. Chữ số hex của nó cũng phải đứng trước. Đảo
thứ tự hai chữ số nghĩa là đảo cả ý nghĩa con số, y như đảo `51` thành
`15` trong hệ mười.
::
:::

:::opt
`32` — cộng các chữ số 1 trong mỗi nhóm, cụ thể 1+0+1+1=3, rồi 0+1+0+1=2.
::why
Gần đúng ở chỗ bạn nhìn đúng các chữ số 1 và 0 có mặt trong từng nhóm —
không đếm nhầm chữ số nào.

Chỗ lệch: bạn đang CỘNG các chữ số lại, thay vì đọc chúng theo vị trí.
Bài 2 của track này đã chỉ ra mỗi cột trong một dãy bit mang một luỹ
thừa khác nhau của 2 — cột phải nhất là 1, kế đó 2, rồi 4, rồi 8. Đọc
`1011` theo vị trí: 8 + 0 + 2 + 1 = 11, không phải phép cộng bốn chữ số
rời rạc.
::
:::
::::

::::explain{#khong-phai-vi-dep}
Tổng kết lại đúng một câu, vì cả bài chỉ xoay quanh nó: hệ mười sáu
không được chọn vì "đẹp mắt". Nó được chọn vì 4 chia hết 8 — một byte
tách gọn làm hai — và 2⁴ = 16 vừa khít số ký hiệu cần dùng. Không phép
tính nào bị dư, không phép tính nào bị thiếu.
::::

::::code{#viet-ham-doi-hex}
Đến lượt bạn viết lại đúng phép tách bạn vừa làm bằng tay — lần này
bằng Python, và làm được cho MỌI byte chứ không riêng gì `218`.

Bảng chữ số hex đã có sẵn, xếp đúng thứ tự giá trị 0 tới 15:
`HEX_CHU_SO[10]` gọi ra chữ `a`, đúng như bảng ở đầu bài.

Hai chỗ trống tính ra hai nhóm bốn bit — viết dưới dạng một con số
0-15, không phải chuỗi bit — bằng đúng phép chia bài 3 đã dạy: nhóm ĐẦU
là kết quả CHIA (bỏ phần dư), nhóm SAU là phần DƯ của phép chia đó.

```python title=starter
HEX_CHU_SO = "0123456789abcdef"

def so_thanh_hex(n):
    cao = ___
    thap = ___
    return HEX_CHU_SO[cao] + HEX_CHU_SO[thap]

print(so_thanh_hex(0))
print(so_thanh_hex(10))
print(so_thanh_hex(218))
print(so_thanh_hex(255))
```

```python title=solution
HEX_CHU_SO = "0123456789abcdef"

def so_thanh_hex(n):
    cao = n // 16
    thap = n % 16
    return HEX_CHU_SO[cao] + HEX_CHU_SO[thap]

print(so_thanh_hex(0))
print(so_thanh_hex(10))
print(so_thanh_hex(218))
print(so_thanh_hex(255))
```

```python title=test
# Bốn giá trị, mỗi giá trị chặn một kiểu nhầm khác nhau:
#   so_thanh_hex(0)   — cả hai nhóm đều 0, dò lỗi thô nhất trước.
#   so_thanh_hex(10)  — nhóm cao đúng bằng 0: nếu lời giải bỏ số 0 dẫn
#                       đầu (nghĩ "một chữ số thì khỏi cần thêm chữ số
#                       kia"), giá trị này bắt ngay — "0a" mà thiếu "0"
#                       thì chỉ còn "a", trượt liền.
#   so_thanh_hex(218) — hai nhóm khác nhau (13 và 10): đảo ngược thứ tự
#                       hai nhóm cũng bắt được ở đây, vì "da" và "ad" là
#                       hai chuỗi khác nhau.
#   so_thanh_hex(255) — hai nhóm đều là 15, chữ số lớn nhất bảng chữ.
assert so_thanh_hex(0) == "00", "so_thanh_hex(0) phải là chuỗi '00' — cả hai nhóm bit của 0 đều là 0000"
assert so_thanh_hex(10) == "0a", "so_thanh_hex(10) phải là '0a' — nhóm cao là 0 (viết '0'), nhóm thấp là 10 (viết 'a'); một byte luôn có ĐỦ hai chữ số, kể cả khi một chữ số là '0'"
assert so_thanh_hex(218) == "da", "so_thanh_hex(218) phải là 'da' — nhóm cao 218 // 16 = 13 ('d'), nhóm thấp 218 % 16 = 10 ('a')"
assert so_thanh_hex(255) == "ff", "so_thanh_hex(255) phải là 'ff' — byte lớn nhất, cả hai nhóm đều là 15"
```

:::hints
- kind: attention
  body: Hai chỗ trống tính ra hai con số từ 0 tới 15 — con số, không phải chuỗi bit. Chỗ đầu là nhóm CAO (bốn bit bên trái), chỗ sau là nhóm THẤP (bốn bit bên phải). Dòng `return` bên dưới đã ghép chúng lại đúng thứ tự rồi, bạn không cần đụng vào dòng đó.
- kind: strategy
  body: Bài 3 dạy đúng cặp phép tính này cho việc đổi số ra bit — chia lấy phần nguyên cho ra "còn được bao nhiêu nhóm mười sáu trọn vẹn", chia lấy phần dư cho ra "phần lẻ còn sót lại". Ở đây mẫu số không phải 2 như bài 3 mà là 16, vì một nhóm bốn bit gói đúng 16 kiểu.
- kind: one-line
  body: "Chỗ trống thứ nhất là `n // 16`, chỗ trống thứ hai là `n % 16`."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^00\n0a\nda\nff\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`da`, `0a`, `ff` — mỗi byte gọn xuống đúng hai ký tự, không thiếu ký tự
nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa tự viết một hàm để đổi ra hex. Nhưng hex đâu phải chuyện chỉ có
lúc IN RA màn hình. Có khi bạn muốn GÕ THẲNG một con số hex ngay trong
mã nguồn — chẳng hạn để ghi rõ "byte này chính là `da`" mà khỏi phải gọi
hàm nào cả.

Python có cho gõ thẳng một con số kiểu `0xda` vào code, để máy tự hiểu
đó là hệ mười sáu, không phải máy phải tự đoán?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
