---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.dem-bang-hai-ngon-tay
title: Đếm khi chỉ có hai ngón tay
summary: "Luật lên bó không đổi khi đổi số ngón tay đếm — chỉ ngưỡng lên bó đổi. Bảng công tắc quen thuộc từ Realm 0 hoá ra đếm được, nếu bật đúng thứ tự."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [mem.binary-counting]
requires: [core.bit, core.byte]
concepts: [mem.binary-counting]
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
Cái bảng công tắc sau quầy cô Bảy mà bạn đã quen mặt — hôm nay mình dùng nó
để đếm.
::::

::::explain{#luat-len-bo-khong-doi}
Bạn đã đếm bằng mười ngón tay từ nhỏ, và luật của cách đếm ấy chỉ gói trong
một câu: **đủ mười thì lên bó** — chín hạt xong tới hạt thứ mười thì cả mười
hạt gom lại, đứng sang cột bên trái, còn cột bên phải trở lại rỗng.

Câu hỏi hôm nay: nếu bàn tay không có mười ngón mà chỉ có **hai** — nghĩa là
mỗi cột chỉ chịu được đúng hai trạng thái, không hơn — thì luật ấy còn dùng
được không?

Được, và không đổi một chữ. Chỉ đổi con số ngưỡng:

> **Đủ hai thì lên bó.**

Bạn đã có sẵn thứ mang đúng hai trạng thái ấy: cái **bit** từ Realm 0, ô chỉ
chứa `0` hoặc `1`, không có nấc giữa. Đếm bằng bit tức là đếm bằng đúng hai
ngón tay — tắt là ngón co lại, bật là ngón vươn ra, không có ngón cong nửa
chừng.

Vậy đếm bằng bit là chuyện dùng lại đúng công thức bạn đã thuộc, chỉ hạ ngưỡng
lên bó từ mười xuống hai.
::::

::::example{#dem-ba-cong-tac}
Trở lại bảng công tắc bạn đã gặp: đèn biển hiệu, đèn trong nhà, quạt trần.
Byte muốn đếm — bật lần lượt các kiểu theo đúng luật lên bó, bắt đầu từ tắt
sạch cả ba.

Viết cột phải cùng (quạt trần) trước, đúng như hàng đơn vị đứng bên phải hàng
chục.

```text title=readonly
lần đếm 1:  0 0 0     tắt sạch cả ba
lần đếm 2:  0 0 1     quạt trần bật
lần đếm 3:  0 1 0     quạt trần đủ hai — lên bó: nó rơi về 0, đèn trong nhà bật
lần đếm 4:  0 1 1     quạt trần bật lại
```

Nhìn kỹ bước từ lần đếm 2 sang lần đếm 3: quạt trần vừa đủ hai trạng thái của
riêng nó (tắt rồi bật), nên nó rơi về tắt và **báo sang cột bên trái** một
tiếng "lên bó" — giống hệt hàng đơn vị đầy thì hàng chục nhích thêm một, chỉ
khác đây ngưỡng đầy là hai chứ không phải mười.

Đếm tiếp tới lần đếm 5, cả hai cột bên phải cùng lúc đủ ngưỡng:

```text title=readonly
lần đếm 4:  0 1 1
lần đếm 5:  1 0 0     hai cột phải cùng lên bó một lượt
```

Quạt trần đủ hai, lên bó cho đèn trong nhà. Nhưng đèn trong nhà lúc đó cũng
đang bật — vừa nhận thêm một thì NÓ cũng đủ hai, nên nó lại lên bó tiếp cho
đèn biển hiệu. Một tiếng lên bó kéo theo một tiếng lên bó nữa, y hệt cảnh quen
thuộc trong hệ mười: 99 cộng 1 thành 100, hai tiếng lên bó dồn liền nhau.

Không có gì trong luật đổi cả. Ngưỡng đầy chỉ là hai, nên tiếng lên bó gõ
thường xuyên hơn nhiều so với đếm bằng mười ngón — cột bên phải cùng gần như
lúc nào cũng vừa mới đầy.
::::

::::predict{#doi-den-bien-hieu commitOnce}
Byte đếm tiếp từ lần đếm 5 (`1 0 0`) cho tới khi cả ba đèn cùng bật
(`1 1 1`), đúng theo luật lên bó vừa thấy.

**Trước khi đếm tay**, bạn đoán xem: tính từ lần đếm 1 cho tới hết bảng, đèn
biển hiệu — cột bên trái cùng — đổi trạng thái (từ tắt sang bật, hoặc ngược
lại) đúng bao nhiêu lần?

:::opt{correct}
Đúng một lần — nó tắt suốt bốn lần đếm đầu, rồi bật và giữ nguyên tới hết.
:::

:::opt
Bảy lần — cứ mỗi lần đếm là một lần đổi trạng thái, y như quạt trần.
::why
Gần đúng ở chỗ quạt trần — cột phải cùng — đúng là đổi trạng thái ở MỌI lần
đếm, không sót lần nào. Bạn nhớ đúng luật cho đúng một cột.

Chỗ lệch là không phải cột nào cũng đổi nhanh như nhau. Đèn biển hiệu chỉ nhận
được một tiếng lên bó khi CẢ hai cột bên phải nó đã đầy cùng lúc — mà điều đó
hiếm hơn nhiều so với việc một mình quạt trần đầy. Cột càng đứng bên trái,
tiếng lên bó tới nó càng thưa.
::
:::

:::opt
Bốn lần — đúng bằng số lần đếm mà nó đang bật.
::why
Gần đúng ở chỗ con số 4 có thật trong bảng: đèn biển hiệu đúng là bật ở bốn
lần đếm cuối, không hơn không kém — đếm dòng bạn đếm không sai.

Chỗ lệch là câu hỏi hỏi số lần **đổi** trạng thái, không hỏi nó bật ở mấy
dòng. Đèn biển hiệu chỉ đổi đúng một lần — ngay khoảnh khắc chuyển từ tắt
sang bật — rồi nó giữ nguyên bật cho bốn lần đếm còn lại. Bật liên tục bốn lần
đếm không phải bốn lần đổi.
::
:::

:::opt
Hai lần — lên rồi lại xuống, như mọi công tắc khác trong bảng này.
::why
Gần đúng ở chỗ vài cột trong bảng này đúng là đổi rồi đổi lại nhiều lần —
đèn trong nhà (cột giữa) đổi tới ba lần trước khi bảng kết thúc, và bạn nhận
ra bảng này không phải cột nào cũng chỉ đổi một chiều.

Chỗ lệch là đèn biển hiệu lại là cột đổi ÍT nhất, không phải hai lần đối
xứng. Bảng dừng lại ngay khi vừa đủ `1 1 1`, đúng lúc đèn biển hiệu vừa bật —
chưa kịp có thêm một vòng đếm nào để nó lại đầy và rơi về tắt.
::
:::
::::

::::code{#dien-tiep-bang-dem}
Byte gõ sẵn sáu lần đếm đầu vào một danh sách Python, rồi bỏ trống hai lần
đếm cuối. Bạn điền tiếp — không phải bằng cách tính ra số, chỉ bằng cách
**đếm tiếp** đúng luật lên bó vừa học.

```python title=starter
day_dem = [
    "000",
    "001",
    "010",
    "011",
    "100",
    "101",
    ___,
    ___,
]

for dong in day_dem:
    print(dong)
```

```python title=solution
day_dem = [
    "000",
    "001",
    "010",
    "011",
    "100",
    "101",
    "110",
    "111",
]

for dong in day_dem:
    print(dong)
```

```python title=test
# Tám lần đếm phải khác nhau từng cái một — nếu hai chỗ trống trùng nhau
# hoặc trùng một dòng đã có sẵn thì bảng đếm hỏng ngay từ đây.
assert len(day_dem) == 8, "bảng đếm ba công tắc phải có đủ tám lần đếm, từ 000 tới 111"
assert len(set(day_dem)) == 8, "tám lần đếm phải khác nhau hết — không lần nào lặp lại lần khác"
assert day_dem[:6] == ["000", "001", "010", "011", "100", "101"], "sáu dòng đầu Byte đã gõ sẵn, đừng sửa chúng"
assert day_dem[6] == "110", "lần đếm 101 đủ hai ở cột phải cùng, lên bó: cột phải về 0, cột giữa nhích thêm một — thành 110"
assert day_dem[7] == "111", "lần đếm 110 chỉ có cột phải cùng bật thêm một, không cột nào đầy — thành 111"
```

:::hints
- kind: attention
  body: Nhìn dòng ngay phía trên chỗ trống thứ nhất — `"101"`. Hỏi đúng câu bài vừa dạy — cột phải cùng của nó đang ở trạng thái nào, và nó đã đủ hai chưa.
- kind: strategy
  body: "Từ 101 đếm tiếp: cột phải cùng (1) đủ hai rồi, lên bó — nó về 0, và cột giữa (0) nhận thêm một thành 1. Cột trái cùng (1) không bị đụng tới, vì tiếng lên bó chỉ đi tới đúng MỘT cột. Kết quả là 110. Đếm thêm một lần nữa từ đó, lần này chỉ cột phải cùng đổi, không cột nào lên bó."
- kind: one-line
  body: 'Chỗ trống thứ nhất là "110", chỗ trống thứ hai là "111".'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^000\n001\n010\n011\n100\n101\n110\n111\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tám lần đếm, không lần nào lặp. Luật lên bó không hề đổi — chỉ ngưỡng đổi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa đếm đúng THỨ TỰ — dòng nào tới trước dòng nào, đúng như đếm 1, 2, 3
trên mười ngón tay. Nhưng bạn chưa hề nói dòng nào **đáng giá bao nhiêu**.

Dòng `101` đứng ở lần đếm thứ sáu trong bảng. Nếu dòng ấy là một con số —
đúng nghĩa một con số, như bạn vẫn hiểu — thì nó là số mấy?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
