---
id: toan.logic-va-chung-minh.dung-sai-do-su-viec
title: Đúng hay sai do sự việc, không do người nói
summary: Mỗi mệnh đề mang đúng một trong hai giá trị, và giá trị ấy do sự việc quyết định — "chưa biết" là chuyện của người đang xét, không phải một giá trị thứ ba.
locale: vi
track: toan
module: logic-va-chung-minh
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [logic.truth-value]
requires: [logic.proposition, core.boolean, ctrl.comparison, core.variable, core.dict, core.string-literal, core.none, core.is-none, core.print-variable]
concepts: [logic.gia-tri-chan-ly, logic.do-su-viec-quyet-dinh, logic.chua-biet-khong-phai-gia-tri]
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
Nam bảo đúng, Lan bảo sai, và mình tin cả hai. Vậy câu ấy rốt cuộc thế nào?
::::

::::explain{#cho-hai-ban-cung-noi-that}
Bài trước để lại một câu khó chịu:

> **"Sân trường rộng."**

Nam bảo nó đúng, Lan bảo nó sai, không ai nói dối. Còn *"CLB có sáu thành
viên"* thì ai đếm cũng ra một kết quả. Hai câu ấy khác nhau ở chỗ nào?

Chỗ khác nằm ở đúng một chữ: **rộng**.

Muốn đối chiếu *"CLB có sáu thành viên"* với sự việc, bạn đếm người. Đếm xong
là chốt được, và cái CLB ngoài kia không đổi theo người đang đếm.

Muốn đối chiếu *"sân trường rộng"* với sự việc, bạn phải đo — nhưng đo xong rồi
so với **cái gì**? Chữ "rộng" không nêu ra một cái mốc nào. Nam mang trong đầu
mốc "đủ chỗ đá bóng"; Lan mang mốc "đủ chỗ cho bốn hàng lớp". Hai mốc khác
nhau, nên hai bạn chốt ra hai bên — và cả hai đều thành thật.

Vậy câu ấy đúng hay sai? Câu hỏi ấy chưa trả lời được, và chỗ hụt không nằm ở
sân trường. Nó nằm ở **câu**: câu chưa nói ra cái mốc mà nó muốn đem đi so.

Chữa được. Đặt mốc vào:

> **"Sân trường rộng hơn 500 mét vuông."**

Giờ thì Nam và Lan không cãi nhau nữa. Cầm thước ra đo, so với 500, rồi chốt.
Ai đo cũng ra một kết quả, và kết quả ấy **do cái sân quyết định**.

Nói cho gọn:

> Một câu kể chỉ là mệnh đề khi chuyện đúng-sai của nó do **sự việc** quyết
> định, chứ không do **người đang nói** quyết định. Câu nào đổi giá trị theo
> người phát ngôn thì nó đang kể về người ấy, không kể về sự việc.
::::

::::example{#cuon-so-quy}
Đem cái sàng ấy về CLB, chỗ có sẵn một cuốn sổ.

Đầu tháng, Lan — bạn giữ sổ của CLB — mở sổ quỹ ra và ghi lại một việc rất
gọn: mỗi thành viên đã nộp phần quỹ của mình hay chưa. Sáu dòng, mỗi dòng một
cái tên và một dấu tích.

| thành viên | đã nộp quỹ tháng này |
|---|---|
| Nam | Đ |
| Lan | Đ |
| Minh | S |
| Hoa | Đ |
| Tú | Đ |
| Khanh | S |

Sổ này không phán xử ai cả. Nó chỉ **ghi lại** một chuyện đã xảy ra rồi.

Giờ lấy ra sáu câu, mỗi thành viên một câu, tất cả cùng một khuôn:

> *"«tên» đã nộp quỹ tháng này."*

Sáu câu ấy đều là mệnh đề: câu kể, có mốc rõ ("đã nộp" hay "chưa"), đối chiếu
với sổ là chốt được. Và mỗi câu, sau khi đối chiếu, dừng lại ở **đúng một**
trong hai bên — không có câu nào dừng ở giữa, không có câu nào dừng ở cả hai.

Đặt tên cho cái "một trong hai bên" ấy:

> **Giá trị chân lý** của một mệnh đề là thứ nó mang sau khi đem đối chiếu với
> sự việc: **đúng** hoặc **sai**. Mỗi mệnh đề mang **đúng một** giá trị chân
> lý, không hơn không kém.

"Không hơn" nghĩa là không có câu nào vừa đúng vừa sai. "Không kém" nghĩa là
không có câu nào chẳng mang giá trị nào.

Trong Python, hai giá trị ấy đã có tên sẵn từ Realm 0: `True` và `False`. Từ
đây, khi cần bắt máy giữ giá trị chân lý của sáu câu, cuốn sổ trên viết thành
một cuốn sổ máy đọc được:

```text
so_quy = {"Nam": True, "Lan": True, "Minh": False, "Hoa": True, "Tú": True, "Khanh": False}
```

Một cái tên, một giá trị. Sáu câu, sáu giá trị.
::::

::::explain{#chua-biet-la-chuyen-cua-nguoi-xet}
Còn một chỗ nữa hay bị nhầm, và nó nhầm theo kiểu rất tự nhiên.

Chiều thứ Năm, Byte đứng ngoài cửa phòng CLB, sổ quỹ thì nằm trong ngăn bàn.
Có ai hỏi Byte: *"Câu 'Khanh đã nộp quỹ tháng này' đúng hay sai?"*

Byte đáp thật lòng: **"Mình chưa biết."**

Nghe xong, rất dễ kết luận rằng câu ấy đang mang một giá trị thứ ba, một giá
trị tên là "chưa biết", nằm đâu đó giữa đúng và sai. Nhiều người dừng lại ở kết
luận ấy cả đời, và nó làm hỏng mọi thứ về sau.

Nhìn kỹ xem "chưa biết" là chuyện của ai.

Khanh đã nộp hay chưa nộp — chuyện ấy **xảy ra rồi**, từ trước khi có ai hỏi.
Thủ quỹ đã ghi vào sổ, và cuốn sổ nằm im trong ngăn bàn suốt buổi. Giá trị chân
lý của câu ấy có sẵn ở đó. Thứ còn thiếu không phải giá trị — thứ còn thiếu là
**Byte chưa cầm được nó**.

> "Chưa biết" mô tả **người đang xét**, không mô tả **câu**. Mở sổ ra thì Byte
> đổi, còn câu thì không đổi gì cả.

Python có sẵn một cách viết cho cái ô trống trong đầu Byte: `None`. Bạn đã gặp
nó ở Realm 1 — nó là "chưa có gì ở đây". Và `None` **không phải** `False`:

```text
byte_ghi_ve_khanh = None    ← ô trống trong đầu Byte
so_quy["Khanh"]   = False   ← giá trị chân lý của câu, sổ đã ghi từ đầu tháng
```

Hai dòng ấy nói hai chuyện khác loại nhau. Trộn chúng làm một là đúng cái nhầm
lẫn mà cả track sau này phải trả giá — vì tới bài 21, bạn sẽ gặp một cái máy
kiểm nghìn lần mà vẫn phải nói "chưa thấy sai" chứ không được nói "đúng". Chỗ
tách bạch hôm nay là thứ làm cho câu ấy có nghĩa.
::::

::::predict{#doan-ba-dong commitOnce}
Byte gõ ba dòng để nhìn tận mắt chỗ hai thứ ấy khác nhau.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
byte_ghi = None
so_quy = {"Khanh": False}

print(byte_ghi)
print(so_quy["Khanh"])
print(byte_ghi == so_quy["Khanh"])
```

:::opt{correct}
`None`, rồi `False`, rồi `False`
:::

:::opt
`None`, rồi `False`, rồi `True`
::why
Gần đúng ở chỗ bạn đọc trúng hai dòng đầu, và ở chỗ bạn nghe ra hai câu tiếng
Việt đứng sau chúng nghe rất giống nhau: *"chưa biết"* và *"chưa nộp"* đều có
vẻ là "không có gì".

Chỗ lệch nằm ở việc hai thứ ấy nói về hai đối tượng khác nhau. `None` là cái ô
còn trống **trong đầu Byte**; `False` là giá trị chân lý **của câu về Khanh**,
do cuốn sổ quyết định từ đầu tháng. Dấu `==` hỏi "hai vế có đang giữ cùng một
thứ không" — mà một cái ô trống với một giá trị chân lý thì không phải cùng một
thứ, nên máy lắc.
::
:::

:::opt
`False`, rồi `False`, rồi `True`
::why
Gần đúng ở chỗ bạn nhớ một luật thật của Realm 1: đem `None` ra chỗ đòi một câu
trả lời đúng-sai thì Python coi nó là phía "không". Luật ấy có thật, và nó cứu
bạn ở nhiều bài khác.

Chỗ lệch là `print` không đòi một câu trả lời đúng-sai. Nó in ra **đúng thứ nó
nhận**, không đổi thứ ấy thành cái khác. Nhận `None` thì nó in ra bốn chữ cái
`None`. Muốn thấy Python coi `None` như phía "không" thì phải đặt nó vào chỗ
biết hỏi, chẳng hạn sau chữ `if` — chứ không phải trong ngoặc của `print`.
::
:::

:::opt
`None`, rồi `None`, rồi `True`
::why
Gần đúng ở chỗ bạn giữ chắc một điều rất quan trọng của bài này, và giữ nó
đúng: câu về Khanh chưa được ai đem ra đối chiếu ở đâu trong đoạn mã này cả.

Chỗ lệch: chuyện "chưa ai đối chiếu" là chuyện của người xét, không phải chuyện
của cuốn sổ. Dòng thứ hai của khung đã ghi thẳng `{"Khanh": False}` — sổ ghi
xong từ trước, và nó nằm đó dù có ai mở ra hay không. Nên `so_quy["Khanh"]` lấy
về đúng cái đã ghi, tức `False`.
::
:::
::::

::::code{#doc-gia-tri-tu-so}
Bắt máy đọc giá trị chân lý của bốn câu, và mọi giá trị đều lấy từ **sổ**, không
lấy từ ai nói.

Nhắc lại luật của track: **Python không phải lời giải.** Nó không quyết định
câu nào đúng — cuốn sổ quyết định. Máy chỉ tra sổ hộ bạn, sáu dòng một nháy.

Bốn chỗ trống, bốn câu:

- **Câu về Nam** — *"Nam đã nộp quỹ tháng này."*
- **Câu về Minh** — *"Minh đã nộp quỹ tháng này."*
- **Hai câu cùng giá trị?** — *"Câu về Minh và câu về Khanh mang cùng một giá
  trị chân lý."* Đây cũng là một mệnh đề, chỉ có điều nó nói **về hai câu kia**.
- **Có giá trị thứ ba không?** — *"Câu về Khanh mang một giá trị 'chưa biết'."*
  Đây là chỗ bài này muốn bạn tự tay bác bỏ.

Bài chấm bằng **cả bốn câu**, và bốn câu ấy cư xử khác hẳn nhau: một câu được
gật, một câu bị lắc, một câu so hai giá trị với nhau, một câu hỏi xem có ô trống
nào không. Gõ cứng `True` hay `False` vào thì hỏng ngay câu bên cạnh, và chép
tên thành viên này sang chỗ của thành viên kia cũng hỏng.

```python title=starter
# Sổ quỹ tháng này. Sổ không phán xử ai; nó ghi lại việc đã xảy ra rồi.
so_quy = {"Nam": True, "Lan": True, "Minh": False, "Hoa": True, "Tú": True, "Khanh": False}

# "Nam đã nộp quỹ tháng này."
cau_nam = ___
# "Minh đã nộp quỹ tháng này."
cau_minh = ___
# "Câu về Minh và câu về Khanh mang cùng một giá trị chân lý."
hai_cau_cung_gia_tri = ___
# "Câu về Khanh mang một giá trị 'chưa biết'."
cau_khanh_con_o_trong = ___

print(cau_nam)
print(cau_minh)
print(hai_cau_cung_gia_tri)
print(cau_khanh_con_o_trong)
```

```python title=solution
# Sổ quỹ tháng này. Sổ không phán xử ai; nó ghi lại việc đã xảy ra rồi.
so_quy = {"Nam": True, "Lan": True, "Minh": False, "Hoa": True, "Tú": True, "Khanh": False}

# "Nam đã nộp quỹ tháng này."
cau_nam = so_quy["Nam"]
# "Minh đã nộp quỹ tháng này."
cau_minh = so_quy["Minh"]
# "Câu về Minh và câu về Khanh mang cùng một giá trị chân lý."
hai_cau_cung_gia_tri = so_quy["Minh"] == so_quy["Khanh"]
# "Câu về Khanh mang một giá trị 'chưa biết'."
cau_khanh_con_o_trong = so_quy["Khanh"] is None

print(cau_nam)
print(cau_minh)
print(hai_cau_cung_gia_tri)
print(cau_khanh_con_o_trong)
```

```python title=test
# Hai câu về giá trị thứ ba đứng TRƯỚC. Chúng canh đúng cái bẫy của bài — nghĩ
# rằng "chưa biết" là một giá trị mà câu có thể mang. Xếp chúng xuống dưới thì
# một câu `is True` sẽ trượt trước, và cái bẫy không bao giờ sập.
assert cau_khanh_con_o_trong is False, "sổ đã ghi giá trị của câu về Khanh từ đầu tháng, nên câu ấy KHÔNG còn ô trống nào — 'chưa biết' là chuyện của người chưa mở sổ"
assert cau_khanh_con_o_trong is not None, "chỗ trống này phải cho ra một câu trả lời gật hay lắc, chứ bản thân nó cũng không được là một ô trống"
assert cau_nam is True, "sổ ghi Nam đã nộp, nên câu 'Nam đã nộp quỹ tháng này' mang giá trị đúng"
assert cau_minh is False, "sổ ghi Minh chưa nộp, nên câu 'Minh đã nộp quỹ tháng này' mang giá trị sai — nó vẫn là một mệnh đề, chỉ là một mệnh đề sai"
assert cau_nam != cau_minh, "câu về Nam được gật còn câu về Minh bị lắc; hai chỗ này mà cho cùng một giá trị thì có ít nhất một chỗ đang không tra sổ"
assert hai_cau_cung_gia_tri is True, "sổ ghi Minh chưa nộp và Khanh cũng chưa nộp, nên hai câu ấy cùng mang giá trị sai — cùng sai vẫn là cùng một giá trị"
```

:::hints
- kind: attention
  body: Cuốn sổ nằm ngay dòng đầu khung, và nó là nơi duy nhất có câu trả lời. Mỗi chỗ trống ứng với một câu tiếng Việt viết ngay trên nó; đọc câu ấy rồi hỏi **nó nói về ai**, và **nó hỏi gì về người ấy**. Hai chỗ cuối không hỏi về một người nữa — chúng hỏi về chính những giá trị vừa lấy ra.
- kind: strategy
  body: "Tra sổ theo đúng cách đã quen ở Realm 1: viết tên cuốn sổ, rồi cái tên cần tra đặt trong ngoặc vuông và trong dấu nháy. Chỗ thứ ba hỏi hai giá trị có **cùng nhau** không, nên nó cần hai lần tra sổ và một dấu `==` ở giữa. Chỗ thứ tư hỏi giá trị lấy về có phải một ô trống không — Realm 1 đã cho bạn đúng một cách hỏi câu ấy, bằng chữ `is` và chữ `None`. Đừng gõ thẳng `True` hay `False`: giá trị phải đi ra từ sổ."
- kind: one-line
  body: "Bốn chỗ lần lượt là `so_quy[\"Nam\"]`, `so_quy[\"Minh\"]`, `so_quy[\"Minh\"] == so_quy[\"Khanh\"]`, và `so_quy[\"Khanh\"] is None`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: mọi giá trị phải đi ra từ cuốn sổ — gõ thẳng `True` hay `False` vào chỗ trống là bạn đang tự phán xử, mà cả bài này dựng lên để nói rằng giá trị chân lý do sự việc quyết định chứ không do người nói
  requireAst:
  # Năm lần tra sổ: một ở câu về Nam, một ở câu về Minh, hai ở chỗ so hai giá
  # trị, một ở chỗ hỏi ô trống. Khung khởi đầu đọc `so_quy` 0 lần (dòng đầu là
  # gán, không phải đọc), nên luật này một mình đã chặn mọi đáp án gõ cứng.
  - kind: uses-name, target: so_quy, min: 5
  # Chỗ thứ ba là một phép SO hai giá trị với nhau, không phải một giá trị.
  - kind: uses-operator, target: ==, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\nFalse\nTrue\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sổ ghi sao thì câu mang giá trị ấy. Mình biết hay chưa biết là chuyện của mình.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ mỗi mệnh đề trong tay bạn đều mang đúng một trong hai giá trị, và giá trị
ấy do sự việc quyết định.

Sổ ghi Nam đã nộp, nên câu **"Nam đã nộp quỹ"** là đúng.

Chiều nay Tú kể lại chuyện ấy theo lối khác: *"Nam chưa nộp quỹ."*

Câu của Tú cũng là một câu kể, cũng phân xử được, nên nó cũng có một giá trị.
Vậy nó mang giá trị gì?

Và hai câu ấy — *"Nam đã nộp quỹ"* với *"Nam chưa nộp quỹ"* — có bao giờ **cùng
đúng** không? Có bao giờ **cùng sai** không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
