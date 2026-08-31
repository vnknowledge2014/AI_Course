---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.do-thi-la-gi
title: "Đồ thị: đỉnh và cạnh, tổng quát hơn cây"
summary: "Cây cấm một nút có hơn một cha và cấm mọi vòng quay lại; đồ thị bỏ cả hai giới hạn đó — chỉ còn đỉnh và cạnh nối chúng, không luật nào về hình dạng."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 34
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.graph]
requires: [ds.tree, core.tuple, core.tuple-unpack, core.dict-get-default, core.list-membership, ctrl.for-range, core.function-def, core.fstring]
concepts: [ds.graph]
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
Cây không nối lại được với chính nó. Hôm nay gặp một cấu trúc bỏ hẳn luật
đó.
::::

::::explain{#cay-cam-hai-thu}
Bài 25 dựng cây từ danh sách liên kết: nới luật "đúng một `tiep`" thành
"có thể nhiều nút con". Nhưng dù nới ra bao nhiêu nhánh, cây vẫn giữ
NGẦM hai luật chưa ai nói thẳng ra:

1. **Một nút chỉ có ĐÚNG MỘT cha.** Thư mục con chỉ nằm trong đúng một
   thư mục cha, không thể vừa nằm trong `Ảnh/` vừa nằm trong `Nhạc/` cùng
   lúc.
2. **Không có đường nào quay lại một nút đã đi qua.** Từ gốc đi xuống,
   không có cách nào đi vòng trở lại gốc — cây không có VÒNG.

Một **đồ thị** (graph) là cấu trúc bỏ CẢ HAI giới hạn đó. Nó chỉ còn hai
thứ trần trụi:

- **Đỉnh** (vertex/node) — các điểm, giống hệt "nút" mà track này đã
  quen dùng.
- **Cạnh** (edge) — một đường NỐI giữa hai đỉnh, không còn phân biệt
  "cha" với "con" nữa. Hai đỉnh hoặc có nối với nhau, hoặc không.

Mạng bạn bè là một đồ thị chứ không phải cây: bạn có thể quen nhiều
người, mỗi người đó lại quen nhiều người khác, và hoàn toàn có thể ba
người quen biết vòng tròn lẫn nhau — một VÒNG mà cây tuyệt đối cấm. Bản
đồ đường đi cũng vậy: từ ngã tư A có thể có đường tới ngã tư B, và từ B
lại có đường quay về A — hai đỉnh nối nhau theo cả hai chiều, không đỉnh
nào là "cha" của đỉnh nào.
::::

::::example{#tim-vong-trong-mang-ban-be}
Bốn người quen biết nhau theo những cặp sau — mỗi cặp là một cạnh, không
phân biệt ai nối tới ai trước:

```python title=readonly
dinh = ["An", "Bình", "Chi", "Dung"]
canh = [("An", "Bình"), ("Bình", "Chi"), ("Chi", "An"), ("Bình", "Dung")]

so_ket_noi = {}
for (a, b) in canh:
    so_ket_noi[a] = so_ket_noi.get(a, 0) + 1
    so_ket_noi[b] = so_ket_noi.get(b, 0) + 1

print(f"Bậc của từng người: {so_ket_noi}")
```

```text title=readonly
Bậc của từng người: {'An': 2, 'Bình': 3, 'Chi': 2, 'Dung': 1}
```

Mỗi cạnh chạm vào đúng hai đỉnh, nên vòng lặp cộng thêm một vào CẢ HAI
đầu của cặp — đây là cách đếm **bậc** (degree) của một đỉnh: có bao nhiêu
cạnh chạm vào nó. `Bình` có bậc `3` — nối với cả `An`, `Chi`, lẫn `Dung`.
Một cây không bao giờ để một nút (khác gốc) có bậc lớn hơn "một cha cộng
số con" theo một hướng nhất định; ở đồ thị, không có hướng nào bị ép cả.

Ba cạnh đầu tiên — `An-Bình`, `Bình-Chi`, `Chi-An` — tạo thành một VÒNG:
đi từ `An` sang `Bình`, sang `Chi`, rồi có đường quay THẲNG về `An`. Một
cây tuyệt đối không thể có cấu trúc này — đi xuống từ gốc không bao giờ
quay lại được gốc.
::::

::::predict{#doan-co-phai-cay commitOnce}
Byte vẽ một sơ đồ thư mục máy tính bằng đúng cách biểu diễn đỉnh/cạnh vừa
học:

```python title=readonly
dinh = ["gốc", "Ảnh", "Nhạc", "kỳ_nghỉ.jpg", "sinh_nhật.jpg"]
canh = [
    ("gốc", "Ảnh"),
    ("gốc", "Nhạc"),
    ("Ảnh", "kỳ_nghỉ.jpg"),
    ("Ảnh", "sinh_nhật.jpg"),
]
```

**Trước khi kết luận**, bạn đoán: cấu trúc đỉnh/cạnh này CÓ THOẢ được cả
hai luật của một cây không (đúng một cha mỗi đỉnh, không vòng)?

:::opt{correct}
Có — mỗi đỉnh (trừ `gốc`) đều chỉ xuất hiện trong đúng một cạnh với vai
trò "phía sau", và không có cách nào đi từ một đỉnh quay lại chính nó.
:::

:::opt
Không — vì `Ảnh` xuất hiện trong tới BA cạnh khác nhau (`gốc-Ảnh`,
`Ảnh-kỳ_nghỉ.jpg`, `Ảnh-sinh_nhật.jpg`), mà một nút của cây không được
xuất hiện nhiều lần.
::why
Gần đúng ở việc bạn đếm đúng: `Ảnh` thật sự xuất hiện trong ba cạnh —
con số đó không sai.

Chỗ lệch là luật cây không cấm một nút xuất hiện trong NHIỀU cạnh — nó
chỉ cấm một nút có nhiều hơn MỘT CHA. `Ảnh` có một cha (`gốc`) và hai con
(`kỳ_nghỉ.jpg`, `sinh_nhật.jpg`) — ba cạnh chạm vào nó là chuyện hoàn
toàn bình thường của một nút có nhiều con, không phải dấu hiệu của việc
có nhiều cha.
::
:::

:::opt
Không — vì đồ thị này có `5` đỉnh nhưng chỉ có `4` cạnh, thiếu mất một
cạnh mới đủ nối hết mọi đỉnh với nhau.
::why
Gần đúng ở việc bạn đếm đúng số đỉnh (5) và số cạnh (4) — hai con số đó
không sai.

Chỗ lệch là một cây với `n` đỉnh LUÔN có đúng `n − 1` cạnh, không thiếu
không thừa — đây là dấu hiệu của một cây HỢP LỆ, không phải dấu hiệu của
lỗi. `5` đỉnh và `4` cạnh khớp đúng công thức đó.
::
:::

:::opt
Có, nhưng chỉ vì đây là sơ đồ THƯ MỤC — thư mục lúc nào cũng là cây, bất
kể cạnh nối ra sao.
::why
Gần đúng ở kết luận cuối — cấu trúc này ĐÚNG LÀ một cây, kết luận đó
không sai.

Chỗ lệch là lý do. "Là sơ đồ thư mục" không tự động đảm bảo nó là cây —
một tập cạnh MÔ TẢ thư mục vẫn có thể vô tình có một cạnh thừa tạo vòng,
hoặc một tệp bị gán vào hai thư mục cha khác nhau, và khi đó nó không còn
là cây nữa dù vẫn "trông giống" thư mục. Phải KIỂM hai luật (một cha, không
vòng) trên chính tập cạnh, không suy ra từ tên gọi của sơ đồ.
::
:::
::::

::::code{#tim-vong-tay-ba}
Dùng đúng mạng bạn bè ở ví dụ trên. Bạn viết hàm kiểm ba người có tạo
thành một vòng tay ba (mỗi người đều quen cả hai người còn lại) hay
không, rồi thử trên ba bộ ba khác nhau — kể cả cùng một vòng tay ba
nhưng liệt kê theo thứ tự khởi đầu KHÁC (bộ ba thứ ba là chính vòng tay
ba đầu tiên, chỉ đổi ai được nêu tên trước).

```python title=starter
dinh = ["An", "Bình", "Chi", "Dung"]
canh = [("An", "Bình"), ("Bình", "Chi"), ("Chi", "An"), ("Bình", "Dung")]

def co_canh(x, y):
    return (x, y) in canh or (y, x) in canh

bo_ba_can_kiem = [
    ("An", "Bình", "Chi"),
    ("An", "Bình", "Dung"),
    ("Chi", "An", "Bình"),
]

co_vong = []
for (x, y, z) in bo_ba_can_kiem:
    if co_canh(x, y) and co_canh(y, z) and ___:
        co_vong.append((x, y, z))

print(f"Những bộ ba tạo vòng tay ba: {co_vong}")
```

```python title=solution
dinh = ["An", "Bình", "Chi", "Dung"]
canh = [("An", "Bình"), ("Bình", "Chi"), ("Chi", "An"), ("Bình", "Dung")]

def co_canh(x, y):
    return (x, y) in canh or (y, x) in canh

bo_ba_can_kiem = [
    ("An", "Bình", "Chi"),
    ("An", "Bình", "Dung"),
    ("Chi", "An", "Bình"),
]

co_vong = []
for (x, y, z) in bo_ba_can_kiem:
    if co_canh(x, y) and co_canh(y, z) and co_canh(z, x):
        co_vong.append((x, y, z))

print(f"Những bộ ba tạo vòng tay ba: {co_vong}")
```

```python title=test
assert co_vong == [("An", "Bình", "Chi"), ("Chi", "An", "Bình")], f"đúng hai bộ ba khép kín thành vòng tay ba — (An, Bình, Chi) và bộ ba thứ ba (Chi, An, Bình), cùng ba người đó nhưng nêu tên theo thứ tự khác — đang báo {co_vong}"
```

:::hints
- kind: attention
  body: Hai điều kiện đầu (`co_canh(x, y)` và `co_canh(y, z)`) đã kiểm hai cạnh của "đường đi" x→y→z. Chỗ trống còn thiếu đúng MỘT cạnh để đường đi đó KHÉP LẠI thành vòng.
- kind: strategy
  body: Muốn khép vòng, cạnh còn thiếu phải nối điểm CUỐI của đường đi (`z`) quay trở lại điểm ĐẦU (`x`) — dùng đúng hàm `co_canh` đã có sẵn, gọi với hai tên đó.
- kind: one-line
  body: 'Chỗ trống là `co_canh(z, x)`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Những bộ ba tạo vòng tay ba: \\[\\('An', 'Bình', 'Chi'\\), \\('Chi', 'An', 'Bình'\\)\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một bộ ba khép vòng thật, một bộ ba chỉ là một đường đi cụt. Cây không
bao giờ cho bạn thấy sự khác biệt này — đồ thị thì có.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa viết `canh` như một danh sách các CẶP, rồi phải quét cả danh sách
đó (`(x, y) in canh`) mỗi khi muốn biết hai đỉnh có nối nhau không. Với
bốn đỉnh, quét cả danh sách không đáng kể. Nhưng một mạng bạn bè thật có
thể có hàng triệu người — quét lại toàn bộ danh sách cạnh mỗi lần hỏi
"A và B có quen nhau không" thì rất tốn.

Có cách nào cất cùng thông tin đó — ai nối với ai — sao cho câu hỏi
"A và B có nối nhau không" trả lời nhanh hơn, không cần quét hết mọi cạnh?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
