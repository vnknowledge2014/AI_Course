---
id: nen-tang.gia-tri-bien-kieu.chuoi-la-mot-day-ky-tu
title: Chuỗi là một dãy ký tự
summary: Máy so hai chuỗi bằng cách soi từng ký tự một theo đúng thứ tự, nên hoa/thường khác nhau và một dấu cách thừa cũng khác.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.string-sequence]
requires: [core.string-literal, core.string-concat, ctrl.comparison, core.bang-ma]
concepts: [core.chuoi, core.ky-tu, core.so-sanh]
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
Mình không đọc câu chữ. Mình soi từng hạt một, từ trái sang.
::::

::::explain{#hai-cai-ten-nghe-giong-nhau}
Bài trước để lại một chỗ khó chịu. Khách gõ tên khoản là `"Cà Phê"`, sổ đang
giữ `"cà phê"`, và phép so cho `False`. Người bán hàng nào cũng bảo đó là một
thứ. Máy thì bảo không.

Muốn hiểu vì sao máy chặt đến thế, phải nhìn lại xem một chuỗi thật ra là cái
gì trong tay nó.

Hình dung một **xâu hạt**. Mỗi hạt là một chữ, và chúng được xâu vào dây theo
một thứ tự cố định: hạt `c` trước, rồi `à`, rồi một hạt trắng, rồi `p`, `h`,
`ê`. Tháo ra xâu lại theo thứ tự khác thì bạn có một xâu khác, dù vẫn đúng
những hạt ấy.

Máy so hai xâu bằng cách đặt chúng song song rồi soi từ đầu dây:

- Hạt thứ nhất của xâu này có giống hạt thứ nhất của xâu kia không?
- Rồi hạt thứ hai, hạt thứ ba…
- Và cuối cùng: hai xâu có đúng bằng nhau số hạt không?

Chỉ cần một cặp hạt lệch nhau, hoặc một xâu dài hơn xâu kia một hạt, là câu trả
lời thành "không giống".

Bây giờ nói bằng thuật ngữ. Mỗi hạt gọi là một **ký tự**. Một chuỗi là một
**dãy ký tự có thứ tự** — có đầu, có đuôi, và mỗi ký tự có một chỗ đứng riêng.
Dấu `==` đặt giữa hai chuỗi hỏi đúng một câu: *hai dãy này có trùng nhau từng
ký tự một, theo đúng thứ tự, không?*

Từ đó hai chuyện nhỏ hoá ra là chuyện lớn.

**`C` và `c` là hai ký tự khác nhau.** Realm 0 đã cho bạn thấy máy chỉ giữ được
số, nên mỗi ký tự phải mang một mã số riêng trong bảng mã. `C` mang một mã, `c`
mang một mã khác. Chúng là hai hạt khác nhau, y như hạt đỏ khác hạt xanh.

**Dấu cách cũng là một ký tự.** Nó là một hạt trắng — mắt bạn nhìn xuyên qua
nó, nhưng nó vẫn nằm trên dây và vẫn được đếm.
::::

::::example{#soi-tung-hat}
Ba phép so, viết ra thành code:

```python title=readonly
tren_phieu = "Cà Phê"
trong_so = "cà phê"

print(tren_phieu == trong_so)
print("cà phê" == "cà phê")
print("cà phê " == "cà phê")
```

Màn hình hiện ra:

```text
False
True
False
```

Đi lại đúng đường máy đi.

**Dòng 1.** Hạt đầu tiên: `C` với `c`. Hai mã số khác nhau, nên máy dừng ngay
tại đó và trả về `False`. Năm hạt còn lại nó không cần xem nữa — một hạt lệch
là đủ.

**Dòng 2.** Sáu hạt, đôi một trùng khớp, và hai dãy dài bằng nhau. `True`.

**Dòng 3.** Đây mới là dòng đáng chú ý. Sáu hạt đầu trùng nhau hoàn toàn. Nhưng
chuỗi bên trái còn một hạt thứ bảy — một dấu cách nằm sau chữ `ê`. Bên phải hết
dây ở hạt thứ sáu. Hai dãy không cùng số hạt, nên `False`.

Nhìn hai dòng chữ `"cà phê "` và `"cà phê"` trên màn hình, mắt bạn thấy chúng
y hệt nhau. Máy thì đếm được bảy hạt và sáu hạt.
::::

::::predict{#doan-ba-dong commitOnce}
Byte sắp chạy ba dòng dưới đây. Cả ba đều so `"cà phê"` với một chuỗi khác, và
mỗi chuỗi khác nhau đúng một chi tiết nhỏ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra ba dòng nào?

```python
print("cà phê" == "Cà phê")
print("cà phê" == "cà phê ")
print("cà phê" == "cà phê")
```

:::opt{correct}
False, False, True
:::

:::opt
True, True, True
::why
Gần đúng ở chỗ bạn đọc ba dòng ấy đúng như một người ghi sổ đọc: cả ba đều là
tên của cùng một cốc cà phê, viết hoa hay viết thường thì vẫn là nó, thừa một
dấu cách thì vẫn là nó.

Chỗ lệch nằm ở chỗ máy không đọc **nghĩa** của câu chữ. Nó không biết "cà phê"
là đồ uống. Nó chỉ có một dãy hạt và một dãy hạt khác, rồi soi từng cặp một. Hai
dòng đầu có hạt lệch, nên cả hai đều `False`.
::
:::

:::opt
False, True, True
::why
Gần đúng ở chỗ bạn nhận ra `C` và `c` là hai ký tự khác nhau — dòng đầu bạn
đoán chính xác, và đó chính là phần khó nhất của bài.

Chỗ lệch nằm ở dòng thứ hai. Dấu cách sau chữ `ê` không phải khoảng trống trên
màn hình, nó là một ký tự thật nằm trong dãy — một hạt trắng vẫn được xâu vào
dây. Chuỗi bên trái có sáu hạt, bên phải có bảy. Số hạt khác nhau thì hai dãy
khác nhau, nên dòng hai cũng `False`.
::
:::

:::opt
True, False, True
::why
Gần đúng ở chỗ bạn tính dấu cách cuối vào dãy: dòng hai bạn đoán chính xác, và
đó là chi tiết mà phần lớn người mới bỏ qua.

Chỗ lệch nằm ở dòng đầu. Bạn đang cho rằng máy bỏ qua chuyện hoa hay thường,
như cách ta tra tên trong danh bạ. Nhưng `C` và `c` mang hai mã số khác nhau
trong bảng mã, nên với máy chúng là hai hạt khác nhau — và một hạt lệch ở chỗ
đầu tiên là đủ để cả phép so thành `False`.
::
:::
::::

::::explain{#dung-day-thi-phai-du-hat}
Vì mỗi ký tự đều được tính, nên chuyện ngược lại cũng đúng: khi bạn **dựng** ra
một chuỗi thay vì so nó, bạn phải dựng đủ cả những ký tự mắt không nhìn thấy.

Dấu `+` nối hai chuỗi — thứ bạn đã dùng từ Realm 0 — chỉ nối đúng những gì bạn
đưa cho nó, không thêm gì cả:

```python
print("cà" + "phê")
```

Dòng này in ra `càphê`, dính liền, năm ký tự. Không phải máy quên dấu cách —
bạn chưa hề đưa cho nó một dấu cách nào. Muốn có hạt trắng ở giữa thì phải xâu
hạt trắng ấy vào, và cách viết một ký tự cũng y như cách viết mọi câu chữ khác:
đặt nó giữa hai dấu nháy.
::::

::::code{#ghep-cho-dung-tung-ky-tu}
Trong sổ chi tiêu của Byte, tên khoản được chọn từ một bảng menu chia sẵn thành
hai mảnh: mảnh `"cà"` và mảnh `"phê"`. Chương trình ghép hai mảnh lại, rồi so
với cái tên mà sổ đang giữ.

Điền vào chỗ trống nằm giữa hai mảnh, sao cho `ghep` ra đúng dãy ký tự của sổ.

Bài chấm bằng **hai** phép so chứ không phải một, và hai phép ấy kéo về hai
phía ngược nhau: một phép đòi chuỗi ghép phải bằng `"cà phê"`, một phép đòi nó
**không** được là `"càphê"` dính liền. Điền **thừa** một hạt thì trượt
phép thứ nhất; bỏ trống hẳn thì trượt cả hai, vì lúc ấy `ghep` đúng bằng
`"càphê"` — đúng thứ phép thứ hai dựng ra để chặn.

```python title=starter
ten_chuan = "cà phê"

ghep = "cà" + ___ + "phê"

print(ghep == ten_chuan)
print(ghep == "càphê")
```

```python title=solution
ten_chuan = "cà phê"

ghep = "cà" + " " + "phê"

print(ghep == ten_chuan)
print(ghep == "càphê")
```

```python title=test
# Chấm bằng HAI phép so kéo về hai phía ngược nhau, không phải một.
#
# Một phép so thì không phân biệt được đúng với gần đúng: hỏi mỗi câu "có bằng
# 'cà phê' không" thì mọi đáp án hụt đều cho `False` như nhau, và người học
# không biết mình xâu thiếu hạt hay xâu thừa hạt.
#
#   ""      → ghep là "càphê":   trượt câu 1, và trượt luôn câu 2 (đúng là nó).
#   "  "    → ghep là "cà  phê": trượt câu 1 vì thừa một hạt trắng.
#   "-"     → ghep là "cà-phê":  trượt câu 1 vì hạt giữa sai mã.
#   " "     → qua cả hai.
#
# Chuỗi trong hai câu assert dưới đây được viết ở dạng NFC — mỗi chữ có dấu là
# MỘT ký tự. Đó là dạng bàn phím tiếng Việt gõ ra, và là dạng mà cả track này
# chốt dùng.
assert ghep == "cà phê", f"ghép ra {ghep!r}, còn sổ đang giữ 'cà phê'"
assert ghep != "càphê", "ghép ra dãy dính liền — thiếu ký tự dấu cách ở giữa"
```

:::hints
- kind: attention
  body: Nhìn kỹ cái tên trong sổ, `"cà phê"`, rồi đếm hạt. Giữa mảnh `"cà"` và mảnh `"phê"` có đúng một hạt nữa mà mắt nhìn xuyên qua nhưng máy vẫn đếm vào dãy.
- kind: strategy
  body: Hạt ấy cũng là một ký tự, nên nó phải được viết ra như mọi câu chữ khác — nằm giữa hai dấu nháy. Đúng một ký tự thôi: hai dấu cách thì dãy dài thành bảy hạt và phép so lại hỏng.
- kind: one-line
  body: Chỗ trống gồm ba ký tự gõ liền nhau — một dấu nháy kép, một dấu cách, rồi một dấu nháy kép nữa. Đó là một chuỗi chứa đúng một dấu cách.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^True\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu hạt, đúng thứ tự, đúng cả hạt trắng ở giữa. Giờ mình gật đầu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nếu chuỗi là một dãy, thì nó có một **số lượng** ký tự xác định — không mơ hồ,
không co giãn. Sáu, bảy, mười ba. Một con số.

Con số ấy sắp có việc dùng. Sổ chi tiêu của Byte in ra thành cột, và cột "tên
khoản" rộng đúng **12 chỗ**. `"cà phê"` thì thoải mái. Còn `"cà phê sữa đá"` —
nó có vừa cột không?

Bạn vừa nhẩm đếm bằng mắt. Đếm được mấy? Có tính ba hạt trắng vào không? Đúng
cái mắt vừa bỏ sót ở phép so ban nãy là cái làm hỏng phép đếm bây giờ.

Vậy bảo máy đếm hộ thì gọi nó bằng cái tên nào? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
