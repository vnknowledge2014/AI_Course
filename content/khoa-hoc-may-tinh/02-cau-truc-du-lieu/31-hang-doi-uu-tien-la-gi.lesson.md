---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.hang-doi-uu-tien-la-gi
title: "Hàng đợi ưu tiên: ra theo độ ưu tiên, không theo thứ tự vào"
summary: "Hàng đợi thường cho ai tới trước ra trước; hàng đợi ưu tiên nới đúng luật đó — ra là người có độ ưu tiên cao nhất, bất kể tới trước hay sau."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 31
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ds.priority-queue]
requires: [ds.queue, core.list-of-dicts, core.max-tracker, ctrl.for-range, core.list-index, core.fstring, core.list-comprehension]
concepts: [ds.priority-queue]
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
Cụm cây vừa khép lại. Giờ quay về một cấu trúc quen — hàng đợi — nhưng
hỏi nó một câu chưa ai hỏi: ra trước có phải luôn đúng không?
::::

::::explain{#uu-tien-noi-luat-fifo}
Bài 9 dựng **hàng đợi**: vào trước, ra trước — ai xếp hàng mua vé sớm hơn
thì được gọi sớm hơn. Luật đó công bằng, và đúng cho rất nhiều việc.

Nhưng phòng cấp cứu không hoạt động theo luật ấy. Một người bị trầy xước
tới lúc 8 giờ, một người đau ngực dữ dội tới lúc 8 giờ 15 — không ai gọi
người trầy xước trước chỉ vì họ tới trước. Người đau ngực được gọi trước,
dù tới SAU. Thứ tự tới không còn là thứ quyết định ai ra trước nữa; **độ
ưu tiên** mới là thứ quyết định.

Một **hàng đợi ưu tiên** (priority queue) là một hàng đợi nới đúng luật đó:
mỗi phần tử mang thêm một con số độ ưu tiên, và lấy ra luôn là phần tử có
độ ưu tiên CAO NHẤT đang có mặt — bất kể nó vào hàng từ lúc nào. Vào hàng
(thêm một người) vẫn rẻ như hàng đợi thường — chỉ việc thêm vào cuối danh
sách. Nhưng lấy ra thì khác hẳn bài 9: không còn là "lấy đúng người đứng
đầu", mà là "tìm đúng người có độ ưu tiên cao nhất trong CẢ hàng", nghĩa
là phải nhìn qua tất cả mọi người đang chờ để biết ai đó.
::::

::::example{#tim-nguoi-khan-cap-nhat}
Ba người đang chờ ở một quầy hỗ trợ kỹ thuật, mỗi người mang một mức độ
khẩn cấp riêng — số càng lớn càng khẩn cấp:

```python title=readonly
hang_cho = [
    {"ten": "Lan", "do_khan_cap": 2},
    {"ten": "Minh", "do_khan_cap": 5},
    {"ten": "Hạnh", "do_khan_cap": 3},
]

chi_so_cao_nhat = 0
for i in range(len(hang_cho)):
    if hang_cho[i]["do_khan_cap"] > hang_cho[chi_so_cao_nhat]["do_khan_cap"]:
        chi_so_cao_nhat = i

print(hang_cho[chi_so_cao_nhat]["ten"])
```

```text title=readonly
Minh
```

`Lan` vào hàng TRƯỚC `Minh` — nếu đây là hàng đợi thường (bài 9), `Lan`
phải ra trước. Nhưng độ khẩn cấp của `Lan` chỉ là 2, còn `Minh` là 5. Vòng
lặp đi qua TỪNG người, so độ khẩn cấp của người đó với người khẩn cấp nhất
đã thấy TỪ TRƯỚC (giữ trong `chi_so_cao_nhat`), và cập nhật lại mỗi khi gặp
một người khẩn cấp hơn. Không có phép tính nào nhảy thẳng tới đúng người —
phải đi qua hết cả ba mới chắc chắn không bỏ sót ai khẩn cấp hơn `Minh`.

Đây chính là cái giá của hàng đợi ưu tiên xây thô như thế này: THÊM một
người thì rẻ (chỉ nối vào cuối), nhưng LẤY người khẩn cấp nhất thì phải
nhìn qua mọi người đang chờ, không có đường tắt nào cả.
::::

::::predict{#doan-benh-nhan-tiep-theo commitOnce}
Bốn bệnh nhân đã ghi danh theo đúng thứ tự này, mỗi người kèm độ khẩn cấp
(số càng lớn càng khẩn cấp):

```text title=readonly
1. Tuấn   — khẩn cấp 4   (ghi danh lúc 8:00)
2. Yến    — khẩn cấp 1   (ghi danh lúc 8:05)
3. Đạt    — khẩn cấp 4   (ghi danh lúc 8:10)
4. Hương  — khẩn cấp 6   (ghi danh lúc 8:15)
```

Y tá dùng đúng cách tìm người khẩn cấp nhất ở ví dụ trên (đi từ đầu tới
cuối, so với người khẩn cấp nhất ĐÃ THẤY, cập nhật khi có người khẩn cấp
hơn — không cập nhật khi chỉ BẰNG).

**Trước khi tính**, bạn đoán: bệnh nhân nào được gọi ĐẦU TIÊN?

:::opt{correct}
Hương — độ khẩn cấp 6, cao nhất trong cả bốn người, dù ghi danh sau cùng.
:::

:::opt
Tuấn — vì anh ghi danh sớm nhất trong bốn người.
::why
Gần đúng ở chỗ Tuấn thật sự ghi danh sớm nhất — 8:00, trước cả ba người
kia. Nếu đây là hàng đợi THƯỜNG (bài 9), Tuấn đúng là người ra trước.

Chỗ lệch là đây không phải hàng đợi thường — đây là hàng đợi ƯU TIÊN. Thứ
tự ghi danh không còn quyết định ai ra trước nữa; độ khẩn cấp mới quyết
định. Tuấn khẩn cấp 4, thấp hơn Hương (6), nên Hương ra trước dù ghi danh
sau Tuấn tới hơn một tiếng.
::
:::

:::opt
Đạt — vì anh là người khẩn cấp mức 4 xuất hiện SAU CÙNG trong số những
người mức 4.
::why
Gần đúng ở chỗ bạn để ý đúng: có hai người CÙNG mức khẩn cấp 4 (Tuấn và
Đạt), và bạn biết phải chọn một quy tắc để phân xử khi bằng nhau — phản xạ
đó đúng hướng.

Chỗ lệch là mức 4 không phải mức cao nhất trong bốn người. Hương mang mức
6, cao hơn cả Tuấn lẫn Đạt. Câu hỏi "ai khẩn cấp NHẤT" phải so trên toàn
bộ bốn người, không dừng lại ở cặp bằng nhau vừa thấy.
::
:::

:::opt
Yến — vì cô ghi danh thứ hai, ngay sau người đầu tiên.
::why
Gần đúng ở chỗ Yến đúng là người ghi danh thứ hai theo thời gian — con số
thứ tự đó không sai.

Chỗ lệch là Yến lại mang độ khẩn cấp THẤP nhất trong cả bốn người (1) —
đây chính là trường hợp ngược hẳn với việc được gọi trước. Thứ tự ghi danh
của Yến không liên quan gì tới việc cô được gọi sớm hay muộn trong một
hàng đợi ưu tiên.
::
:::
::::

::::code{#lay-nguoi-khan-cap-nhat}
Bốn người đang chờ ở quầy cấp cứu. Bạn viết đoạn tìm và gọi đúng người
khẩn cấp nhất — người ĐÓ được lấy ra khỏi hàng chờ, những người còn lại ở
lại nguyên vị trí tương đối với nhau.

```python title=starter
hang_cho = [
    {"ten": "Bé An", "do_khan_cap": 3},
    {"ten": "Ông Bảy", "do_khan_cap": 2},
    {"ten": "Chị Hoa", "do_khan_cap": 5},
    {"ten": "Anh Long", "do_khan_cap": 4},
    {"ten": "Dì Sáu", "do_khan_cap": 5},
]

chi_so_cao_nhat = 0
for i in range(len(hang_cho)):
    if hang_cho[i]["do_khan_cap"] > hang_cho[___]["do_khan_cap"]:
        chi_so_cao_nhat = ___

nguoi_duoc_goi = hang_cho.pop(chi_so_cao_nhat)
print(f"Gọi: {nguoi_duoc_goi['ten']} (độ khẩn cấp {nguoi_duoc_goi['do_khan_cap']})")
print(f"Còn lại trong hàng chờ: {[nguoi['ten'] for nguoi in hang_cho]}")
```

```python title=solution
hang_cho = [
    {"ten": "Bé An", "do_khan_cap": 3},
    {"ten": "Ông Bảy", "do_khan_cap": 2},
    {"ten": "Chị Hoa", "do_khan_cap": 5},
    {"ten": "Anh Long", "do_khan_cap": 4},
    {"ten": "Dì Sáu", "do_khan_cap": 5},
]

chi_so_cao_nhat = 0
for i in range(len(hang_cho)):
    if hang_cho[i]["do_khan_cap"] > hang_cho[chi_so_cao_nhat]["do_khan_cap"]:
        chi_so_cao_nhat = i

nguoi_duoc_goi = hang_cho.pop(chi_so_cao_nhat)
print(f"Gọi: {nguoi_duoc_goi['ten']} (độ khẩn cấp {nguoi_duoc_goi['do_khan_cap']})")
print(f"Còn lại trong hàng chờ: {[nguoi['ten'] for nguoi in hang_cho]}")
```

```python title=test
assert nguoi_duoc_goi == {"ten": "Chị Hoa", "do_khan_cap": 5}, f"người được gọi phải là người có độ khẩn cấp CAO NHẤT — hoà thì người tới TRƯỚC thắng, nên phải là Chị Hoa chứ không phải Dì Sáu (cùng mức 5, tới sau) — đang gọi nhầm {nguoi_duoc_goi}"
assert hang_cho == [
    {"ten": "Bé An", "do_khan_cap": 3},
    {"ten": "Ông Bảy", "do_khan_cap": 2},
    {"ten": "Anh Long", "do_khan_cap": 4},
    {"ten": "Dì Sáu", "do_khan_cap": 5},
], f"bốn người còn lại phải giữ nguyên thứ tự tương đối với nhau, chỉ thiếu đúng Chị Hoa — đang còn lại {hang_cho}"
```

:::hints
- kind: attention
  body: Hai chỗ trống làm đúng MỘT việc — theo dõi chỉ số của người khẩn cấp nhất ĐÃ THẤY, tính tới thời điểm hiện tại trong vòng lặp. Chưa xét hết cả bốn người thì chưa thể chắc `chi_so_cao_nhat` là câu trả lời cuối cùng.
- kind: strategy
  body: 'Chỗ trống thứ nhất phải so `hang_cho[i]` với người khẩn cấp nhất ĐÃ TÌM ĐƯỢC, không phải với người đầu tiên cố định — dùng lại đúng cái tên `chi_so_cao_nhat` để so. Chỗ trống thứ hai: khi tìm thấy người khẩn cấp hơn, chỉ số mới cần nhớ chính là `i` — vị trí của người đó trong vòng lặp, không phải một con số đếm cứng.'
- kind: one-line
  body: 'Cả hai chỗ trống cùng dùng những cái tên đã có sẵn trong bài — chỗ đầu là `chi_so_cao_nhat`, chỗ sau là `i`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống thứ hai phải cập nhật chi_so_cao_nhat bằng đúng chỉ số i của vòng lặp — không phải một con số gõ cứng, vì con số đúng cho dữ liệu này chưa chắc đúng cho một hàng chờ khác
  requireAst:
  - kind: uses-name, target: i, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Gọi: Chị Hoa \\(độ khẩn cấp 5\\)\\nCòn lại trong hàng chờ: \\['Bé An', 'Ông Bảy', 'Anh Long', 'Dì Sáu'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chị Hoa vào hàng ở vị trí thứ ba, nhưng ra trước cả bốn người kia — kể cả
Dì Sáu, người CÙNG mức khẩn cấp 5 nhưng tới sau. Độ khẩn cấp thắng thứ tự
tới; hoà độ khẩn cấp thì thứ tự tới mới lên tiếng, và Chị Hoa tới trước.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy: mỗi lần cần người khẩn cấp nhất, chương trình phải NHÌN QUA
TOÀN BỘ hàng chờ, từng người một, không sót ai — chỉ để chắc chắn không có
ai khẩn cấp hơn người đang giữ kỷ lục. Nếu hàng chờ có một trăm người thay
vì năm, mỗi lần gọi bệnh nhân lại phải quét lại từ đầu một lần nữa.

Có cách nào sắp xếp trước những người đang chờ, sao cho việc tìm người
khẩn cấp nhất không cần nhìn qua tất cả mọi người mỗi lần không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
