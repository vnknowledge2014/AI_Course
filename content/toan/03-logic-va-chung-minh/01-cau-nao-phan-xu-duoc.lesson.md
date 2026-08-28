---
id: toan.logic-va-chung-minh.cau-nao-phan-xu-duoc
title: Câu nào phân xử được
summary: Trong một buổi sinh hoạt, người ta nói đủ thứ câu — nhưng chỉ một số câu là thứ mà chuyện đúng hay sai của nó đem ra phân xử được.
locale: vi
track: toan
module: logic-va-chung-minh
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [logic.proposition]
requires: [core.boolean, ctrl.comparison, core.variable, core.number-literal, core.print-variable, core.arithmetic]
concepts: [logic.menh-de, logic.phan-xu-duoc, logic.cau-khong-mang-gia-tri]
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
Cả buổi mình nghe bảy câu. Có câu gật hay lắc được, có câu thì không.
::::

::::explain{#mon-no-cua-bai-truoc}
Track trước khép lại bằng một món nợ.

Ở bài "khi số nào cũng đúng", bạn gỡ một phương trình tới `0 = 0` rồi đọc ra:
**mọi số đều là nghiệm**. Câu ấy nói về vô hạn con số. Mà bạn đã thử được bao
nhiêu? Bốn dòng trong một cái bảng.

Câu hỏi bỏ ngỏ là: *bạn tin câu ấy vì đã thử vài số, hay vì **chứng minh** được?*

Cả track này đi trả món nợ đó. Nhưng trước khi hỏi "lấy gì bảo đảm câu này
đúng", phải trả lời một câu dễ bị bỏ qua hơn nhiều:

> **Câu nào thì mới có chuyện đúng hay sai để mà bảo đảm?**

Không phải câu nào cũng có. Và chỗ dễ thấy điều đó nhất không nằm trong sách
toán — nó nằm trong một buổi nói chuyện bình thường.
::::

::::example{#bay-cau-trong-mot-buoi-sinh-hoat}
Chiều thứ Năm, CLB cờ vua lớp 6A sinh hoạt. CLB có sáu thành viên: Nam, Lan,
Minh, Hoa, Tú và Khanh. Trên tường có một bảng nội quy, trong ngăn bàn có một
cuốn sổ quỹ.

Byte ngồi một góc và chép lại bảy câu nghe được, theo đúng thứ tự:

1. *"Chiều nay CLB có mấy người đến?"*
2. *"Xếp bàn cờ ra đi."*
3. *"CLB có sáu thành viên."*
4. *"Cờ vua hay hơn cờ tướng."*
5. *"Chiều nay cả CLB đánh xong bốn ván."*
6. *"Ước gì phòng CLB rộng hơn."*
7. *"CLB có nhiều hơn mười bàn cờ."*

Giờ thử làm đúng một việc với từng câu: **gật hay lắc**. Nghĩa là trả lời
"đúng" hoặc "sai" cho chính câu ấy, không phải cho người nói nó.

Câu 1 chịu. *"Chiều nay CLB có mấy người đến?"* — gật vào đây thì gật cái gì?
Câu ấy không khẳng định điều gì cả; nó **hỏi**. Đáp lại nó bằng "đúng" là một
câu trả lời vô nghĩa.

Câu 2 cũng chịu, nhưng chịu theo kiểu khác. *"Xếp bàn cờ ra đi."* — đây là một
mệnh lệnh. Bạn làm theo hoặc không làm theo; bạn không **gật** với nó. Nó chưa
kể lại chuyện gì để mà đối chiếu.

Câu 3 thì làm được ngay: *"CLB có sáu thành viên."* Đếm một cái là xong. Đếm ra
sáu thì gật, đếm ra khác thì lắc. Chuyện gật hay lắc ở đây **do cái CLB quyết
định**, không do ai nói câu ấy.

Câu 4 lại chịu. *"Cờ vua hay hơn cờ tướng."* Nam gật, Tú lắc, và không ai trong
hai bạn nói sai điều gì về mình cả — chữ "hay hơn" ở đây kể lại cái thích của
người nói, chứ không kể lại một chuyện xảy ra ngoài đời để đối chiếu.

Câu 5 làm được: *"Chiều nay cả CLB đánh xong bốn ván."* Mở sổ ra đếm số ván.

Câu 6 chịu: *"Ước gì phòng CLB rộng hơn."* Một lời ước. Nó không kể lại chuyện
gì, nó mong một chuyện chưa có.

Câu 7 làm được — và đây là câu đáng nhìn kỹ nhất. *"CLB có nhiều hơn mười bàn
cờ."* Đếm bàn cờ trong phòng: có ba cái. Ba thì không nhiều hơn mười, nên câu
này **lắc**. Nó sai. Nhưng để nói được nó sai, bạn đã phải đem nó ra đối chiếu
với sự việc — và đối chiếu được chính là điều đang cần.

Xếp lại bảy câu thành hai cột:

| gật hay lắc được | không gật cũng không lắc được |
|---|---|
| 3. CLB có sáu thành viên. | 1. Chiều nay CLB có mấy người đến? |
| 5. Chiều nay cả CLB đánh xong bốn ván. | 2. Xếp bàn cờ ra đi. |
| 7. CLB có nhiều hơn mười bàn cờ. | 4. Cờ vua hay hơn cờ tướng. |
| | 6. Ước gì phòng CLB rộng hơn. |

Cột trái ba câu, cột phải bốn câu. Ba câu cột trái đều là câu **kể** — chúng kể
lại một chuyện, và chuyện ấy đem đối chiếu với cái CLB thật được.
::::

::::explain{#dat-ten-cho-cot-trai}
Đặt tên cho cột trái:

> Một **mệnh đề** là một câu **kể** mà chuyện nó đúng hay sai là chuyện **phân
> xử được**: có một cách đem nó ra đối chiếu với sự việc rồi chốt lại một
> trong hai bên.

Câu hỏi, câu sai khiến, câu nêu ý thích, lời ước — không câu nào trong số đó là
mệnh đề. Không phải vì chúng dở; chúng làm việc khác. Chúng chỉ không phải thứ
mà cả track này sắp đem ra mổ xẻ.

Có một chỗ rất dễ trượt, và nói ngay bây giờ thì đỡ hơn nói sau:

> **Phân xử được không có nghĩa là đúng.**

Câu 7 — *"CLB có nhiều hơn mười bàn cờ"* — là một mệnh đề đàng hoàng, và nó
sai. Câu 3 cũng là mệnh đề, và nó đúng. Cả hai đứng chung một cột, vì cột ấy
không xếp theo "đúng hay sai"; nó xếp theo "**có** đúng-sai để mà xếp hay
không".

Bạn đã gặp đúng hai câu trả lời ấy từ Realm 0 rồi: `True` và `False`. Hồi đó
chúng là thứ máy nhả ra sau một dấu so sánh. Từ bài này trở đi, chúng là **giá
trị của một câu tiếng Việt** — và cả track sẽ sống trên đúng hai giá trị đó.
::::

::::predict{#doan-ba-dong commitOnce}
Byte mở máy, gõ hai con số của sổ vào rồi thử ba dòng. Hai dòng đầu là hai câu
kể của buổi chiều nay, viết lại bằng dấu so sánh. Dòng thứ ba thì không.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
so_thanh_vien = 6
so_ban_co = 3

print(so_ban_co == 3)
print(so_thanh_vien == so_ban_co)
print(so_thanh_vien + so_ban_co)
```

:::opt{correct}
`True`, rồi `False`, rồi `9`
:::

:::opt
`True`, rồi `False`, rồi `True`
::why
Gần đúng ở chỗ bạn đọc trúng hai dòng đầu, và ở chỗ bạn đang chờ cả ba dòng
đều trả lời gật hay lắc — hợp lý thôi, đây là bài mở đầu một track về đúng và
sai.

Chỗ lệch nằm ở dòng cuối: nó **không hỏi gì**. Dấu `+` gộp hai con số lại thành
một con số thứ ba, `6 + 3 = 9`. Mà một con số thì chẳng có ai gật hay lắc trước
nó được — hỏi "số 9 đúng hay sai" cũng vô nghĩa y như hỏi "câu 'xếp bàn cờ ra
đi' đúng hay sai". Đó chính là ranh giới bài này đang vẽ, và nó vẽ cả trong máy
lẫn ngoài đời.
::
:::

:::opt
`True`, rồi `True`, rồi `9`
::why
Gần đúng ở chỗ bạn thấy hai dòng đầu đều là câu kể thật về CLB, đều viết đúng
không sai chữ nào, và bạn đọc dòng cuối rất chuẩn.

Chỗ lệch nằm ở việc dấu `==` hỏi cái gì. Nó không hỏi "hai vế có cùng nói về
CLB không"; nó hỏi "hai vế có đang giữ **cùng một con số** không". `so_thanh_vien`
giữ `6`, `so_ban_co` giữ `3`, và sáu khác ba — nên máy lắc, tức là in `False`.
Câu kể *"số thành viên bằng số bàn cờ"* là một mệnh đề đàng hoàng; nó chỉ tình
cờ là một mệnh đề **sai**.
::
:::

:::opt
`True`, rồi `False`, rồi `63`
::why
Gần đúng ở chỗ bạn nhớ một luật thật của Realm 0: dấu `+` có hai việc, cộng số
và nối chữ. `"6" + "3"` đúng là ra `"63"`, không sai chút nào.

Chỗ lệch là điều kiện để việc nối chữ xảy ra: cả hai vế phải là **chữ**, tức là
có dấu nháy. Ở đây hai dòng đầu của khung đã dán `so_thanh_vien` lên `6` và
`so_ban_co` lên `3` — hai con số trần, không nháy. Với hai con số thì `+` làm
đúng việc quen thuộc của nó, và `6 + 3 = 9`.
::
:::
::::

::::code{#ba-cau-cho-may-phan-xu}
Bắt máy phân xử ba câu kể của buổi chiều nay.

Đây là chỗ cần nói rõ một lần cho cả track: **Python ở đây không phải lời
giải.** Nó không biết câu nào là mệnh đề — chính bạn quyết định điều đó, bằng
cách viết được câu ấy thành một phép hỏi. Việc của máy chỉ là đem sổ ra đối
chiếu rồi nhả về `True` hay `False`, nhanh hơn và không mỏi mắt.

Sổ chiều nay ghi ba con số. Ba câu cần phân xử:

- **Câu 1** — dòng đầu bảng nội quy: *"CLB có ít nhất sáu thành viên."*
- **Câu 2** — Lan đọc sổ: *"CLB có đúng ba bàn cờ."*
- **Câu 3** — Byte đoán: *"Số ván đã đánh bằng số thành viên."*

Câu 3 là chỗ Byte đoán bừa, và bạn sắp thấy máy lắc. Một câu bị lắc vẫn là một
mệnh đề — đó là điều bài này muốn bạn nhìn tận mắt, chứ không phải chỉ đọc qua.

Bài chấm bằng **cả ba câu**, và ba câu được chọn để cư xử khác hẳn nhau: một
câu chạm **đúng cái mốc** nó nêu ra, một câu chốt một con số, một câu so hai con
số khác nhau. Gõ cứng `True` hay `False` vào thì hỏng ngay câu bên cạnh.

```python title=starter
# Sổ CLB, ghi chiều thứ Năm.
so_thanh_vien = 6
so_ban_co = 3
so_van_da_danh = 4

# "CLB có ít nhất sáu thành viên."
cau_1 = ___
# "CLB có đúng ba bàn cờ."
cau_2 = ___
# "Số ván đã đánh bằng số thành viên."
cau_3 = ___

print(cau_1)
print(cau_2)
print(cau_3)
```

```python title=solution
# Sổ CLB, ghi chiều thứ Năm.
so_thanh_vien = 6
so_ban_co = 3
so_van_da_danh = 4

# "CLB có ít nhất sáu thành viên."
cau_1 = so_thanh_vien >= 6
# "CLB có đúng ba bàn cờ."
cau_2 = so_ban_co == 3
# "Số ván đã đánh bằng số thành viên."
cau_3 = so_van_da_danh == so_thanh_vien

print(cau_1)
print(cau_2)
print(cau_3)
```

```python title=test
# Câu bị LẮC đứng trước. Nó canh đúng cái bẫy lớn nhất của bài — nghĩ rằng
# "phân xử được" nghĩa là "đúng". Xếp nó xuống dưới thì một câu `is True` sẽ
# trượt trước, và cái bẫy không bao giờ sập.
assert cau_3 is False, "sổ ghi 4 ván và 6 thành viên; bốn khác sáu, nên câu 'số ván đã đánh bằng số thành viên' phải bị lắc — nó vẫn là một mệnh đề, chỉ là một mệnh đề sai"
assert cau_1 is True, "sổ ghi đúng 6 thành viên, mà sáu thì không ít hơn sáu — 'ít nhất sáu' đúng ngay tại cái mốc sáu, chứ không đòi phải hơn"
assert cau_2 is True, "sổ ghi đúng 3 bàn cờ, nên câu 'CLB có đúng ba bàn cờ' phải được gật"
assert cau_1 != cau_3, "hai câu này phải cho hai giá trị khác nhau: câu về thành viên được gật, câu Byte đoán bị lắc — cùng ra một giá trị nghĩa là một trong hai chỗ trống đang không đọc sổ"
assert so_van_da_danh == 4, "đừng sửa cuốn sổ để câu đoán của Byte thành đúng; sổ ghi 4 ván, và việc của bạn là phân xử câu ấy chứ không phải chiều nó"
```

:::hints
- kind: attention
  body: Mỗi chỗ trống ứng với đúng một câu tiếng Việt viết ngay trên nó. Đọc lại câu ấy và tìm hai thứ: **con số nào của sổ** đang được nói tới, và **chữ so sánh nào** đang được dùng — "ít nhất", "đúng", hay "bằng". Ba dòng trên cùng khung là ba con số của sổ, và chúng có sẵn tên rồi.
- kind: strategy
  body: "Dịch từng chữ một. \"Ít nhất sáu\" nghĩa là sáu cũng được, hơn sáu càng được — đúng cái dấu `>=` của Realm 0. \"Đúng ba\" và \"bằng\" đều là dấu `==`. Vế trái viết cái tên mà sổ đang giữ, vế phải viết thứ câu ấy đem ra so: có khi là một con số câu tự nêu ra, có khi lại là một cái tên khác của sổ. Đừng gõ thẳng `True` hay `False`: bạn đang bắt máy phân xử, không phải tự phân xử hộ nó."
- kind: one-line
  body: "Ba chỗ lần lượt là `so_thanh_vien >= 6`, `so_ban_co == 3`, và `so_van_da_danh == so_thanh_vien`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: mỗi chỗ trống phải là một phép HỎI viết từ tên của sổ, không phải một giá trị Đ/S gõ sẵn — gõ cứng `True` hay `False` là bạn đã tự phân xử hộ máy, mà cả bài này dựng lên để nói rằng việc phân xử phải do sự việc quyết định
  requireAst:
  # Ba câu, ba phép hỏi. Khung khởi đầu không có dấu so sánh nào, nên hai luật
  # này một mình đã chặn mọi đáp án gõ cứng `True`/`False` vào ba chỗ trống.
  - kind: uses-operator, target: >=, min: 1
  - kind: uses-operator, target: ==, min: 2
  # Mỗi câu phải ĐỌC sổ. Khung khởi đầu đọc 0 lần (ba dòng đầu là gán, không
  # phải đọc); lời giải đọc `so_thanh_vien` hai lần — một ở câu 1, một ở vế
  # phải của câu 3.
  - kind: uses-name, target: so_thanh_vien, min: 2
  - kind: uses-name, target: so_ban_co, min: 1
  - kind: uses-name, target: so_van_da_danh, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\nTrue\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai câu được gật, một câu bị lắc. Cả ba đều phân xử xong — đó mới là điều mình cần.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa có một cái sàng: câu hỏi, câu sai khiến, câu nêu ý thích và lời ước rơi
ra ngoài; câu kể phân xử được thì ở lại.

Chiều nay còn một câu nữa mà Byte quên chép vào bảy câu kia:

> **"Sân trường rộng."**

Nghe thì đúng là một câu kể. Không hỏi ai, không sai khiến ai, không ước gì cả.
Nó nằm gọn trong cột trái.

Có điều Nam bảo câu ấy đúng — sân trường thừa chỗ đá bóng. Lan bảo câu ấy sai —
xếp bốn hàng lớp vào là chật cứng. Và **không ai nói dối cả**; hai bạn đều đang
kể đúng thứ mình thấy.

Còn *"CLB có sáu thành viên"* thì khác hẳn: đếm một cái là xong, ai đếm cũng ra
một kết quả.

Hai câu ấy khác nhau ở chỗ nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
