---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.hang-doi-vao-truoc-ra-truoc
title: "Hàng đợi: vào trước, ra trước"
summary: "Hàng đợi (FIFO) là gương đối xứng của ngăn xếp: thêm vào ĐUÔI bằng .append như cũ, nhưng lấy ra ở ĐẦU — ai tới trước ra trước, đúng luật một hàng chờ công bằng."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ds.queue]
requires: [ds.dynamic-array, core.list, core.list-append, core.variable, core.assignment, core.fstring]
concepts: [ds.queue]
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
Xếp hàng mua vé: ai tới trước ra trước, không phải ai tới sau ra trước.
Ngăn xếp không làm được việc này.
::::

::::explain{#guong-doi-xung}
Bài 7 dựng ngăn xếp trên đúng một quy tắc: thêm và lấy đều xảy ra ở ĐUÔI
dãy — vào sau, ra trước. Giờ đổi đúng MỘT nửa của quy tắc đó, và được một
cấu trúc hoàn toàn khác.

Vẫn thêm phần tử mới vào đuôi — dùng `.append(x)` y hệt trước, không đổi
gì cả. Nhưng lấy ra thì đổi đầu: không lấy ở đuôi nữa, mà lấy ở **đầu**
dãy — phần tử đã đứng chờ LÂU NHẤT trong tất cả. Cấu trúc này gọi là
**hàng đợi** (queue), và luật của nó là **vào trước, ra trước** — tiếng
Anh gọi là FIFO (First In, First Out).

Hình dung một hàng người xếp hàng mua vé: người mới tới luôn nối vào CUỐI
hàng, không ai chen ngang. Nhưng quầy vé phục vụ người ở ĐẦU hàng —
người đã đứng chờ lâu nhất, không phải người vừa mới tới. Đó là lẽ công
bằng của một hàng chờ: tới trước thì ra trước.

Với `list`, lấy phần tử ở đầu dùng `.pop(0)` — truyền chỉ số `0` cho
`.pop`, khác hẳn `.pop()` không tham số của ngăn xếp. Cùng một phương
thức `.pop`, nhưng tham số quyết định lấy ở đầu hay ở đuôi.
::::

::::example{#hang-cho-mua-ve}
Bốn người xếp hàng mua vé theo thứ tự Lan, Kim, Mai, Nga. Quầy vé phục vụ
hai người đầu tiên.

```python title=readonly
hang_doi = []
hang_doi.append("Lan")
hang_doi.append("Kim")
hang_doi.append("Mai")
hang_doi.append("Nga")
print(hang_doi)

nguoi_1 = hang_doi.pop(0)
nguoi_2 = hang_doi.pop(0)
print(f"Phục vụ: {nguoi_1}, rồi {nguoi_2}")
print(hang_doi)
```

```text title=readonly
['Lan', 'Kim', 'Mai', 'Nga']
Phục vụ: Lan, rồi Kim
['Mai', 'Nga']
```

"Lan" xếp hàng đầu tiên nên được phục vụ đầu tiên — đúng thứ tự cô đứng
vào hàng, không phải thứ tự ngược lại. So với ví dụ chồng đĩa của bài 7:
cùng dùng `.append` để đưa vào, nhưng lấy ra ở ĐẦU (`pop(0)`) thay vì ĐUÔI
(`pop()`) — một chữ số `0` thôi mà đổi hẳn luật ra vào.
::::

::::predict{#hang-cho-xen-ke commitOnce}
Hai người xếp hàng, một người được phục vụ, rồi hai người nữa xếp thêm,
rồi phục vụ tiếp một người:

```python
hang_doi = []
hang_doi.append("Bao")
hang_doi.append("Chau")

nguoi_phuc_vu_1 = hang_doi.pop(0)

hang_doi.append("Duy")
hang_doi.append("En")

nguoi_phuc_vu_2 = hang_doi.pop(0)

print(hang_doi)
print(nguoi_phuc_vu_2)
```

**Trước khi chạy**, bạn đoán hai dòng in ra là gì?

:::opt{correct}
`['Duy', 'En']`, rồi `Chau`
:::

:::opt
`['Duy', 'En']`, rồi `En`
::why
Gần đúng ở dòng đầu — danh sách còn lại cuối cùng đúng là `['Duy', 'En']`.

Chỗ lệch là bạn đang lấy ra người MỚI NHẤT ("En") thay vì người đã chờ
LÂU NHẤT. Đó là luật của ngăn xếp (bài 7), không phải hàng đợi. Sau khi
"Bao" đã được phục vụ ở lượt trước, người chờ lâu nhất còn lại trong hàng
là "Chau" — không phải "En", người vừa mới xếp vào.
::
:::

:::opt
`['Chau', 'Duy', 'En']`, rồi `Bao`
::why
Gần đúng ở việc bạn nhận ra "Bao" có liên quan tới lượt phục vụ đầu
tiên — đúng là "Bao" bị lấy ra ở đó.

Chỗ lệch là lượt `pop(0)` đầu tiên đã XOÁ "Bao" khỏi hàng đợi thật sự,
không chỉ "đánh dấu" nó. Khi hai lệnh `.append` tiếp theo chạy, "Bao"
không còn trong `hang_doi` nữa để mà lấy ra lần thứ hai — người đang chờ
lâu nhất lúc đó là "Chau".
::
:::

:::opt
`['Duy', 'En']`, rồi `Bao`
::why
Gần đúng ở dòng đầu — `['Duy', 'En']` đúng là trạng thái cuối cùng của
`hang_doi`.

Chỗ lệch là bạn in nhầm biến. `nguoi_phuc_vu_1` (giá trị "Bao") đã được
lấy ra ở LƯỢT ĐẦU và không xuất hiện trong dòng in cuối cùng của chương
trình — dòng đó in `nguoi_phuc_vu_2`, kết quả của lượt `pop(0)` THỨ HAI,
là "Chau".
::
:::
::::

::::code{#quay-ve-them-nguoi}
Bốn người đã xếp hàng: Lan, Kim, Mai. Byte cho thêm "Nga" nối vào cuối
hàng, rồi quầy vé phục vụ hai người đầu tiên liên tiếp.

```python title=starter
hang_doi = []

hang_doi.append("Lan")
hang_doi.append("Kim")
hang_doi.append("Mai")
___                                # "Nga" xếp vào CUỐI hàng

nguoi_phuc_vu_1 = ___               # phục vụ người đang đứng ĐẦU HÀNG
nguoi_phuc_vu_2 = hang_doi.pop(0)

print(hang_doi)
print(f"Đã phục vụ: {nguoi_phuc_vu_1}, rồi {nguoi_phuc_vu_2}")
```

```python title=solution
hang_doi = []

hang_doi.append("Lan")
hang_doi.append("Kim")
hang_doi.append("Mai")
hang_doi.append("Nga")

nguoi_phuc_vu_1 = hang_doi.pop(0)
nguoi_phuc_vu_2 = hang_doi.pop(0)

print(hang_doi)
print(f"Đã phục vụ: {nguoi_phuc_vu_1}, rồi {nguoi_phuc_vu_2}")
```

```python title=test
assert hang_doi == ["Mai", "Nga"], f"sau hai lượt phục vụ, hàng chỉ còn đúng Mai và Nga — đang ra {hang_doi}"
assert nguoi_phuc_vu_1 == "Lan", f"lượt phục vụ đầu tiên phải lấy đúng người đứng ĐẦU hàng lúc đó — Lan — đang ra {nguoi_phuc_vu_1!r}"
assert nguoi_phuc_vu_2 == "Kim", f"lượt phục vụ thứ hai phải lấy đúng người đứng đầu MỚI, sau khi Lan đã rời hàng — Kim — đang ra {nguoi_phuc_vu_2!r}"
```

:::hints
- kind: attention
  body: Hàng đợi thêm vào ở ĐUÔI (giống ngăn xếp bài 7) nhưng lấy ra ở ĐẦU (khác ngăn xếp). Đừng lẫn công cụ lấy ra của hai bài.
- kind: strategy
  body: 'Chỗ trống thứ nhất: "Nga" xếp vào cuối hàng bằng .append("Nga"), đúng công cụ quen thuộc từ bài 7. Chỗ trống thứ hai: lấy người ĐẦU HÀNG bằng .pop(0) — chỉ số 0 nghĩa là lấy ở đầu, không phải ở đuôi.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `hang_doi.append("Nga")` và `hang_doi.pop(0)`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải thật sự dùng .append (thêm vào đuôi) và .pop(0) (lấy ra ở đầu) — bài này đang dạy đúng kỷ luật hàng đợi động vào HAI đầu khác nhau, không phải .insert(), không phải .pop() không tham số, không phải đọc trực tiếp qua chỉ số
  requireAst:
  # Khung có sẵn 3 lượt append cố định (Lan, Kim, Mai) và không lượt pop nào.
  # Lời giải đúng cộng thêm 1 append (Nga) và 2 pop — đếm thật trên solution:
  # append() 4 lần, pop() 2 lần. Thiếu cổng này, hang_doi.insert(0, "Nga")
  # (đẩy Nga lên ĐẦU thay vì nối vào CUỐI hàng) vẫn còn 3 append cũ nếu chỉ
  # đếm lỏng lẻo — min:4 chặn đúng nó vì mất một lượt append thì chỉ còn 3.
  - kind: uses-call, target: append, min: 4
  - kind: uses-call, target: pop, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\['Mai', 'Nga'\\]\\nĐã phục vụ: Lan, rồi Kim\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lan xếp hàng đầu tiên, Lan được phục vụ đầu tiên — công bằng, đúng như
một hàng chờ phải vậy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa lấy người đứng đầu hàng bằng `.pop(0)`. Nhớ lại bài 5: xoá phần tử
ở CHỈ SỐ 0 của một mảng là đúng chỗ tốn kém nhất — mọi phần tử phía sau
đều phải dồn lên một bước để lấp chỗ trống.

Vậy `hang_doi.pop(0)` — công cụ tự nhiên nhất để lấy người đầu hàng — có
đang âm thầm trả đúng cái giá đó, ở MỖI LẦN phục vụ một người, hay không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
