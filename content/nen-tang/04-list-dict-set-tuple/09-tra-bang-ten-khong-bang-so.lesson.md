---
id: nen-tang.list-dict-set-tuple.tra-bang-ten-khong-bang-so
title: Tra bằng tên, không bằng chỗ đứng
summary: "`dict` cất tên ngay cạnh giá trị của nó, và mỗi giá trị được tra bằng một khoá — `chi[\"sửa xe\"]` thay cho việc phải nhớ khoản ấy đứng ô số mấy."
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.dict]
requires: [core.list, core.list-index, core.list-append, core.parallel-lists, ctrl.for-each, core.variable, core.assignment, core.fstring, core.output, core.string-literal]
concepts: [core.so-tra-cuu, core.khoa, core.gia-tri]
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
Viết tên ngay cạnh tiền, trong cùng một chỗ. Thế là máy cũng biết chúng dính nhau.
::::

::::explain{#viet-soi-day-ra-giay}
Bài trước để lại một cuốn sổ báo cáo sai mà không kêu tiếng nào, và lý do thì đã
tìm ra rồi: sợi dây nối tên với tiền nằm trong đầu bạn, không nằm trong chương
trình. Máy chỉ thấy hai danh sách rời nhau.

Cách chữa, nói cho gọn, là **viết sợi dây ấy ra giấy** — cất tên và tiền vào cùng
một chỗ, dính liền nhau, để máy cũng giữ được quan hệ đó.

Python có sẵn một chỗ chứa làm đúng việc này. Thay vì đánh số các ô 0, 1, 2, 3,
bạn tự **đặt tên** cho từng ô:

```python title=readonly
chi = {
    "ăn sáng": 85000,
    "sửa xe": 500000,
    "biếu bà": 300000,
    "đổ xăng": 120000,
}
```

Ngoặc ở đây là ngoặc **nhọn** `{ }`, không phải ngoặc vuông của danh sách. Bên
trong là các **cặp**, mỗi cặp một dòng, cách nhau bởi dấu phẩy. Trong một cặp:

- bên trái dấu hai chấm là **khoá** — cái tên bạn tự đặt cho ô ấy;
- bên phải là **giá trị** — thứ ô ấy đang giữ.

Chỗ chứa này tên là `dict`. Cứ gọi nó là **sổ tra cứu** cho dễ hình dung: một
cuốn sổ mà bạn không lật theo số trang, bạn tra theo tên.

Tra thì viết thế này:

```python title=readonly
chi["sửa xe"]
```

Vẫn là ngoặc vuông quen thuộc, nhưng thứ đặt trong ngoặc đã đổi hẳn nghĩa. Với
một danh sách, `tien[1]` nghĩa là *"ô đứng thứ hai, đếm từ 0"* — bạn phải biết
khoản ấy nằm ở đâu. Với sổ tra cứu, `chi["sửa xe"]` nghĩa là *"ô mang tên sửa
xe"* — bạn không cần biết nó nằm ở đâu, và cũng không có "đâu" nào để mà biết.

Có một chỗ nữa vừa thay đổi, và nó chính là thứ bài này đi tìm: **tên và tiền
được viết trong cùng một dòng**. Bạn không thể viết tên mà quên tiền — một cặp
thiếu vế phải thì máy từ chối ngay lúc đọc chương trình, chứ không đợi tới lúc
in báo cáo mới nói dối bạn.
::::

::::example{#tra-bang-ten}
Ba khoản, và ba cách tra:

```python title=readonly
chi = {
    "ăn sáng": 85000,
    "sửa xe": 500000,
    "biếu bà": 300000,
}

print(chi["sửa xe"])
print(chi["ăn sáng"])

tong_hai_khoan = chi["sửa xe"] + chi["biếu bà"]
print(f"Sửa xe và biếu bà cộng lại: {tong_hai_khoan} đồng")

khoan_can_xem = "biếu bà"
print(f"Khoản {khoan_can_xem} hết {chi[khoan_can_xem]} đồng")
```

Máy in ra:

```text title=readonly
500000
85000
Sửa xe và biếu bà cộng lại: 800000 đồng
Khoản biếu bà hết 300000 đồng
```

Ba chỗ đáng dừng lại nhìn:

- **Thứ tra ra là một con số bình thường.** `chi["sửa xe"]` cho về đúng 500000,
  đem cộng trừ được ngay — y như `tien[1]` ở bài trước cho về một con số. Chỉ có
  cách gọi nó ra là khác.
- **Khoá viết trong dấu nháy.** Vì khoá ở đây là một dãy chữ, và dãy chữ thì viết
  trong nháy, đúng như từ Realm 0 tới giờ.
- **Khoá cũng có thể đang nằm trong một biến.** Dòng cuối không gõ lại chữ "biếu
  bà" trong ngoặc; nó đặt vào đó cái tên `khoan_can_xem`, và máy dùng **giá trị**
  của cái tên ấy để tra. Đây chính là chỗ cho phép bạn tra hàng loạt trong một
  vòng lặp.
::::

::::predict{#doan-thu-tu-co-doi-gi-khong commitOnce}
Byte chép cuốn sổ tra cứu ra làm hai bản. Hai bản ghi đúng ba cặp như nhau, chỉ
khác thứ tự các dòng.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
chi_a = {"ăn sáng": 85000, "sửa xe": 500000, "biếu bà": 300000}
chi_b = {"biếu bà": 300000, "ăn sáng": 85000, "sửa xe": 500000}

print(chi_a["sửa xe"])
print(chi_b["sửa xe"])
```

:::opt{correct}
`500000` rồi `500000`
:::

:::opt
`500000` rồi `300000`
::why
Gần đúng ở chỗ bạn theo dõi rất sát: trong `chi_b`, cặp nằm ở dòng đầu đúng là
`"biếu bà": 300000` thật. Bạn đọc không sai một chữ.

Chỗ lệch nằm ở nghĩa của thứ đặt trong ngoặc vuông. Suốt tám bài vừa rồi, ngoặc
vuông ăn một **chỗ đứng**, nên mắt quen đọc `chi_b["sửa xe"]` thành "lấy cái đầu
tiên" hay "lấy cái thứ hai". Nhưng ở đây trong ngoặc là một **cái tên**, và máy
đi tìm đúng cái tên ấy, nó nằm ở dòng nào cũng được. Trong `chi_b`, tên "sửa xe"
được viết ở dòng cuối, và tiền viết ngay cạnh nó vẫn là 500000.
::
:::

:::opt
`500000` rồi máy báo lỗi, vì `chi_b` viết sai thứ tự
::why
Gần đúng ở chỗ bạn đang cẩn thận đúng cách sau bài 8: ở đó hai dãy lệch thứ tự là
hỏng thật, nên nghi ngờ thứ tự là phản xạ tốt.

Chỗ lệch: thứ tự chỉ thành vấn đề khi có **hai** chỗ chứa phải khớp nhau, mà mỗi
bên tự nó không mang đủ thông tin. Ở đây mỗi cặp tự nó đã đủ — tên đi kèm tiền
ngay trong cặp — nên không có gì để mà lệch. Hai cuốn sổ ghi cùng ba cặp mà khác
thứ tự dòng sẽ cho cùng một câu trả lời cho mọi lần **tra bằng khoá**.
::
:::

:::opt
Cả hai dòng đều in ra `sửa xe`
::why
Gần đúng ở chỗ bạn nhớ đúng cách ngoặc vuông làm việc với danh sách: đưa vào một
chỗ đứng, nhận về thứ nằm ở chỗ đó. Áp cùng lối nghĩ ấy sang đây thì đưa vào một
cái tên, nhận về cái tên — nghe rất có lý.

Chỗ lệch là ở chiều của phép tra. Cái tên là thứ bạn **đang cầm trong tay** rồi;
nhận lại nó thì bạn chẳng biết thêm gì. Thứ bạn thiếu là số tiền, nên sổ tra cứu
đi theo chiều tên → tiền: đưa vào khoá, nhận về giá trị nằm cạnh khoá đó.
::
:::
::::

::::code{#bao-cao-tra-bang-ten}
Byte đưa một sổ tra cứu bốn khoản, và một danh sách những khoản cần đưa vào báo
cáo — ba khoản, xếp theo thứ tự Byte muốn đọc, **không** theo thứ tự đã ghi
trong sổ.

Đó chính là chỗ sổ tra cứu tỏ ra dễ chịu: bạn không phải biết khoản nào đứng ô
số mấy, chỉ cần cầm cái tên đi tra.

```python title=starter
chi = {
    "ăn sáng": 85000,
    "sửa xe": 500000,
    "biếu bà": 300000,
    "đổ xăng": 120000,
}

can_bao = ["sửa xe", "đổ xăng", "ăn sáng"]

dong_bao_cao = []
for ten in can_bao:
    dong_bao_cao.append(f"{ten}: {___} đồng")

for dong in dong_bao_cao:
    print(dong)
```

```python title=solution
chi = {
    "ăn sáng": 85000,
    "sửa xe": 500000,
    "biếu bà": 300000,
    "đổ xăng": 120000,
}

can_bao = ["sửa xe", "đổ xăng", "ăn sáng"]

dong_bao_cao = []
for ten in can_bao:
    dong_bao_cao.append(f"{ten}: {chi[ten]} đồng")

for dong in dong_bao_cao:
    print(dong)
```

```python title=test
# Ba dòng, ba khoá khác nhau, và thứ tự báo cáo cố tình khác thứ tự ghi sổ —
# nên một chỗ trống chỉ tình cờ đúng ở dòng đầu sẽ lộ ra ở dòng thứ hai.
assert dong_bao_cao == [
    "sửa xe: 500000 đồng",
    "đổ xăng: 120000 đồng",
    "ăn sáng: 85000 đồng",
], "ba dòng phải theo đúng thứ tự của danh sách can_bao, và mỗi dòng lấy số tiền mà sổ tra cứu đang giữ dưới đúng cái tên ấy: sửa xe 500000, đổ xăng 120000, ăn sáng 85000"
assert chi["biếu bà"] == 300000, "làm báo cáo là việc chỉ đọc: khoản biếu bà không có trong báo cáo lần này, nhưng trong sổ nó vẫn phải còn nguyên 300000"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ngay sau dấu hai chấm trong câu chữ, tức là chỗ đáng ra phải hiện số tiền. Ở lượt này bạn đang cầm trong tay đúng một thứ — cái tên mà vòng lặp vừa lấy ra từ `can_bao`.
- kind: strategy
  body: Bạn có cái tên, bạn cần số tiền, và sổ tra cứu đi đúng chiều đó. Viết tên cuốn sổ rồi đặt khoá vào trong ngoặc vuông. Khoá đang nằm sẵn trong một biến, nên đừng gõ lại một cái tên cố định trong dấu nháy — làm vậy thì cả ba dòng báo cáo sẽ mang cùng một số tiền.
- kind: one-line
  body: "Viết `chi[ten]` vào chỗ trống, giữ nguyên chữ `đồng` phía sau."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: "sửa xe: 500000 đồng"
- tier: output
  expect: "ăn sáng: 85000 đồng"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tên với tiền nằm chung một dòng. Giờ thì không dãy nào lệch được nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sổ tra cứu vừa dập tắt cả một lớp lỗi im lặng: tên và tiền dính liền nhau, không
có chỗ đứng nào để mà trượt.

Nhưng Byte vừa hỏi một câu chưa ai chuẩn bị. Tháng này Byte đóng học phí, và
trước khi ghi vào sổ, Byte gõ thử `chi["học phí"]` — một cái tên mà cuốn sổ chưa
từng ghi lần nào.

Tra một khoá chưa từng ghi thì máy đưa ra cái gì? Số 0, vì chưa tiêu đồng nào cho
khoản đó? Một khoảng trống? Hay một thứ khác hẳn?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
