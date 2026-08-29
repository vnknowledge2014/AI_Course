---
id: nen-tang.gia-tri-bien-kieu.chia-lay-phan-nguyen
title: Phép chia không có phần lẻ
summary: Dấu `//` chia rồi bỏ thẳng phần lẻ — luôn hạ xuống, và giữa hai số nguyên thì kết quả cũng là một số nguyên.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.floor-division]
requires: [core.division, core.float, core.arithmetic, core.variable, core.fstring, core.type-fn]
concepts: [core.so, core.phep-tinh]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Chia tiền thì mình chỉ đi được một hướng: không ai trả nhiều hơn phần của mình.
::::

::::explain{#lam-tron-di-hai-huong}
Bài trước cho bạn `round`, và nó đưa con số về mốc nguyên **gần nhất**. Gần
nhất thì có khi ở phía trên, có khi ở phía dưới — và với việc chia tiền, hai
phía ấy không giống nhau chút nào.

Nhóm ba người ăn bún chả, hoá đơn `100000`. `round(100000 / 3)` cho `33333`. Ba
người đưa mỗi người `33333`, cộng lại được `99999` — thiếu quán một đồng. Một
đồng thì quán bỏ qua.

Nhóm sáu người ăn lẩu, cũng hoá đơn `100000`. `100000 / 3` là `33333.333...`
nên nó hạ xuống, còn `100000 / 6` là `16666.666...` — phần lẻ lớn hơn nửa, nên
`round` **đưa lên** `16667`. Sáu người đưa mỗi người `16667`, cộng lại được
`100002`. Nhóm vừa trả dư hai đồng cho một hoá đơn một trăm nghìn.

Hai đồng thì nhỏ. Cái đáng ngại không phải con số, mà là chuyện `round` không
có hướng cố định: nó lên hay xuống là do phần lẻ quyết định, và bạn không biết
trước. Người thu tiền ngoài đời không làm việc theo kiểu ấy. Họ chia phần chắc
chắn có trước — mỗi người mười sáu nghìn sáu trăm sáu mươi sáu — rồi phần lẻ
tính sau. Nghĩa là **luôn hạ xuống**, không bao giờ đưa lên.

Nhìn kỹ thì đây đúng là phép chia bạn học hồi lớp ba: *một trăm chia ba được ba
mươi ba, còn dư một*. Không ai nói "được xấp xỉ ba mươi ba phẩy ba" cả. Câu hỏi
lúc ấy là "**mấy lần trọn vẹn**", giống hệt câu "một trăm nghìn đổi được mấy tờ
hai chục nghìn". Con số trả lời câu hỏi ấy có tên riêng trong toán: **thương**.

Python cũng có một dấu riêng cho đúng phép ấy — hai gạch chéo liền nhau `//`.
Tên gọi của nó: **chia lấy phần nguyên** (tiếng Anh là *floor division*, nghĩa
đen là "chia rồi hạ xuống sàn").
::::

::::example{#mot-gach-va-hai-gach}
Cùng hai con số, đổi mỗi số gạch chéo:

```python title=readonly
tien_bun = 100000

print(tien_bun / 3)
print(tien_bun // 3)
print(type(tien_bun // 3))
```

Máy in ra:

```text
33333.333333333336
33333
<class 'int'>
```

Ba dòng, mỗi dòng một điều đáng nhớ.

Dòng đầu là phép chia bạn đã biết từ Realm 0: một gạch, và kết quả mang cả cái
đuôi dài phía sau.

Dòng thứ hai là phép chia mới: hai gạch, và phần lẻ biến mất hẳn. Không phải nó
được làm tròn — nó bị **bỏ lại**. `100000 // 6` cho `16666` chứ không phải
`16667`, dù `16666.666...` nằm sát mốc mười bảy nghìn hơn. `//` không đi tìm
mốc gần nhất; nó dừng ở mốc nguyên nằm **dưới**.

Dòng thứ ba là chỗ trả nợ cho bài 1. Cái đuôi `.0` không quay lại: `type` nói
kết quả là `int`, một số nguyên thật. Hai số nguyên chia bằng `//` thì đi ra một
số nguyên, không phải một số thực trông giống số nguyên.

Một ghi chú bên lề, vì luật lây của bài 1 vẫn còn hiệu lực: nếu **một** vế là số
thực thì kết quả cũng là số thực. `45000.0 // 2` cho `22500.0` — vẫn không có
phần lẻ, nhưng nhãn dán trên nó là `float`.
::::

::::predict{#doan-hai-gach commitOnce}
Sổ chi tiêu ghi khoản lẩu `250000`, chia đều cho sáu người.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra con số nào?

```python
tien_lau = 250000
so_nguoi = 6
print(tien_lau // so_nguoi)
```

:::opt{correct}
41666
:::

:::opt
41667
::why
Gần đúng ở chỗ bạn tính đúng phép chia: `250000` chia `6` được
`41666.666...`, và mốc nguyên gần nhất của con số ấy đúng là `41667`. Đó chính
xác là việc `round` sẽ làm, và bạn vừa làm nó không sai một con số nào.

Chỗ lệch nằm ở chữ "gần nhất". `//` không đi tìm mốc gần nhất — nó dừng ở mốc
nguyên nằm dưới, bất kể phần lẻ là `0.001` hay `0.999`. Nên `//` luôn cho ra
`41666`.

Và đây đúng là chỗ hai dấu tách ra: cùng một cặp số, `round` đưa lên còn `//`
hạ xuống.
::
:::

:::opt
41666.666666666664
::why
Gần đúng ở chỗ bạn nhớ chắc kết quả của phép chia mà Realm 0 đã dạy: chia
`250000` cho `6` thì con số thật đúng là như vậy, kể cả cái đuôi dài.

Chỗ lệch nằm ở số gạch chéo. Một gạch `/` giữ nguyên phần lẻ và cho ra số thực
— đó là dòng bạn đang nghĩ tới. Hai gạch `//` là một dấu khác, và việc của nó
là bỏ phần lẻ đi.
::
:::

:::opt
41666.0
::why
Gần đúng ở chỗ bạn nhớ luật của bài 1: phép chia là chỗ cái đuôi `.0` hay chui
ra, nên kết quả nghe có vẻ phải mang nhãn `float`. Với dấu `/` một gạch thì bạn
đúng hoàn toàn — `250000 / 5` cho `50000.0` chứ không cho `50000`.

Chỗ lệch: luật ấy nói về dấu `/`. Còn `//` đặt giữa **hai số nguyên** thì trả về
một `int` thật, không đuôi. Muốn thấy đuôi `.0` với `//` thì phải có một vế là
số thực, ví dụ `250000.0 // 6`.
::
:::
::::

::::explain{#sao-khong-cat-bang-int}
Có một câu hỏi hợp lý ở đây: bài 2 đã cho bạn `int()` cắt phần lẻ rồi, vậy
`int(100000 / 3)` cũng ra `33333`. Cần thêm một dấu mới làm gì?

Con số thì đúng là như nhau. Nhưng `int(a / b)` là **hai** việc nối nhau: chia
ra một số thực đã, rồi cắt số thực ấy. Còn `a // b` là **một** việc, máy không
phải dựng con số thực trung gian nào.

Với những con số của một dòng sổ chi tiêu, hai lối cho cùng kết quả. Nhưng lối
đi vòng qua số thực có một chỗ hở, và hai bài nữa bạn sẽ tự tay mở nó ra. Từ giờ
tới đó, chia lấy phần nguyên thì viết thẳng `//`.
::::

::::code{#chia-hai-khoan-an-chung}
Sổ chi tiêu của Byte có hai khoản ăn chung, mỗi khoản một số người khác nhau:

- Phở `100000đ`, chia cho **3** người.
- Lẩu `250000đ`, chia cho **6** người.

Điền vào hai chỗ trống để mỗi dòng in ra phần **mỗi người phải đưa** — số tiền
chắc chắn có, không đưa lên.

Bài chấm bằng cả hai khoản một lúc, không phải một. Với một khoản thì gõ thẳng
`33333` vào chỗ trống cũng qua, mà gõ thẳng thì đâu còn phép chia nào. Hơn nữa
hai khoản này được chọn để tách `//` khỏi `round`: `100000` chia `3` thì `round`
cũng cho `33333`, nhưng `250000` chia `6` thì `round` cho `41667` còn `//` cho
`41666`.

```python title=starter
tien_bun = 100000
tien_lau = 250000

moi_nguoi_bun = ___
moi_nguoi_lau = ___

print(f"Bún chả chia 3: mỗi người {moi_nguoi_bun}đ")
print(f"Lẩu chia 6: mỗi người {moi_nguoi_lau}đ")
```

```python title=solution
tien_bun = 100000
tien_lau = 250000

moi_nguoi_bun = tien_bun // 3
moi_nguoi_lau = tien_lau // 6

print(f"Bún chả chia 3: mỗi người {moi_nguoi_bun}đ")
print(f"Lẩu chia 6: mỗi người {moi_nguoi_lau}đ")
```

```python title=test
# Chấm trên HAI khoản, và mỗi khoản chặn một kiểu trả lời hụt khác nhau:
#   gõ cứng một con số            → khoản còn lại sai ngay;
#   `round(tien_lau / 6)`         → ra 41667, trượt dòng thứ hai;
#   `tien_lau / 6`                → ra 41666.666..., trượt cả kiểu lẫn con số;
#   `tien_bun / 3` cho cả hai     → sai luôn con số của khoản lẩu.
# Hai assert giữa chốt phần mà bài 1 để lại: `//` giữa hai số nguyên phải trả
# về `int` thật, không phải một số thực trông giống số nguyên. Kiểm bằng cách
# in ra chữ chứ không bằng `isinstance`: bài 13 của chính track này dạy người
# học đè lên cái tên `int`, nên trong bộ chấm cái tên ấy không đáng tin.
assert moi_nguoi_bun == 33333, "100000 chia cho 3 người thì mỗi người 33333đ"
assert moi_nguoi_lau == 41666, "250000 chia cho 6 người thì mỗi người 41666đ (round cho 41667 — đưa lên là thu quá)"
assert f"{moi_nguoi_bun}" == "33333", "kết quả phải là số nguyên, không mang đuôi .0"
assert f"{moi_nguoi_lau}" == "41666", "kết quả phải là số nguyên, không mang đuôi .0"
assert moi_nguoi_bun * 3 <= tien_bun, "tổng ba phần không được vượt quá hoá đơn"
assert moi_nguoi_lau * 6 <= tien_lau, "tổng sáu phần không được vượt quá hoá đơn"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm ở vế phải của hai dòng gán. Mỗi chỗ cần một phép chia giữa số tiền và số người — nhưng là phép chia cho ra con số nguyên đồng, không có phần lẻ và không đưa lên.
- kind: strategy
  body: Dấu chia bạn cần là dấu có hai gạch chéo liền nhau, đặt giữa tên khoản tiền và số người. Số người của hai khoản khác nhau: bún chả 3, lẩu 6 — chép nguyên dòng bún chả xuống dòng lẩu thì con số của lẩu sẽ sai.
- kind: one-line
  body: "Thay `___` thứ nhất bằng `tien_bun // 3` và `___` thứ hai bằng `tien_lau // 6`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: bài này dạy dấu chia hai gạch `//` — hãy dùng chính nó ở cả hai dòng, đừng chia bằng `/` rồi cắt hay làm tròn lại
  requireAst:
  # `min: 2` vì có hai khoản, mỗi khoản một phép chia. Hỏi `uses-name` không
  # được: khung đã nhắc `tien_bun` và `tien_lau` ở dòng gán đầu tiên.
  - kind: uses-operator, target: //, min: 2
  forbidAst:
  # Đi vòng qua số thực rồi cắt/làm tròn cũng ra đúng con số ở khoản bún chả, nên
  # phải chặn bằng hình dạng chứ không chặn được bằng kết quả.
  - kind: uses-operator, target: /
  - kind: uses-call, target: round
- tier: output
  match: regex
  expect: ^Bún chả chia 3: mỗi người 33333đ\nLẩu chia 6: mỗi người 41666đ\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
33333 và 41666. Hai con số nguyên, không đuôi, và không đồng nào thu quá.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`100000 // 3` cho `33333`. Ba người đưa mỗi người `33333`, cộng lại được
`99999`. Hoá đơn là `100000`, nên còn **một đồng** chưa ai trả — nó nằm đâu đó
trong phép chia mà bạn vừa làm.

Muốn biết con số một ấy, bây giờ bạn phải tự làm hai việc: nhân phần mỗi người
với số người, rồi lấy hoá đơn trừ đi — `100000 - 33333 * 3`. Nghĩa là phải cầm
lại kết quả của phép chia rồi mới hỏi được phần thừa. Đổi số người từ 3 sang 6
thì phải sửa con số ở hai chỗ.

Nhưng phép chia hồi lớp ba đâu có cho ra một con số. Nó cho ra **hai**: thương,
và số dư. Python vừa đưa bạn cái thương bằng một dấu gọn lỏn.

Cái số dư có dấu riêng của nó không? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
