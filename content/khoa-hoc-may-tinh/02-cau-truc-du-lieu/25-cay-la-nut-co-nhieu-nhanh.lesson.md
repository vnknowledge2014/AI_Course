---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.cay-la-nut-co-nhieu-nhanh
title: "Cây: một nút có thể có nhiều nhánh"
summary: "Danh sách liên kết là nút có đúng một `tiep`. Cây nới luật đó ra: một nút giữ một danh sách các nút con — thư mục chứa thư mục con là một cây, không phải một chuỗi."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ds.tree]
requires: [ds.linked-node, core.dict, core.list, core.list-append, core.len, core.fstring, core.list-comprehension]
concepts: [ds.tree]
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
Bài trước để lại một câu hỏi: nếu một nút được phép có NHIỀU HƠN MỘT
`tiep` cùng lúc thì sao? Hôm nay trả lời — và nó phá luôn một luật bạn
vừa quen.
::::

::::explain{#mot-nut-nhieu-nhanh}
Nút của danh sách liên kết (bài 13) có đúng một trường trỏ đi tiếp:
`{"gia_tri": ..., "tiep": None}`. Từ một nút, bạn chỉ có ĐÚNG MỘT đường
để đi — tới nút kế, hoặc `None` nếu đã hết dãy.

Cây nới đúng luật đó ra một bước: thay vì một trường `tiep` duy nhất, một
nút cây giữ một **danh sách** các nút con. Nút có thể có 0 con, 1 con, hay
rất nhiều con — không giới hạn:

```python title=readonly
nut = {"gia_tri": ..., "con": []}
```

Nghĩ tới thư mục máy tính của bạn. Một thư mục **Tài liệu** có thể chứa ba
thư mục con: **Ảnh**, **Nhạc**, **Dự án** — không phải MỘT thư mục con
tiếp theo như một dây chuyền, mà BA nhánh cùng xuất phát từ một chỗ. Rồi
**Dự án** lại có thư mục con riêng của nó: **Web**, **Game**. Đó chính là
một cây: mỗi nút (thư mục) giữ một danh sách các nút con (thư mục con) —
và mỗi nút con, tới lượt nó, cũng có thể có danh sách con riêng.

Nút không có con nào — danh sách `"con"` rỗng — gọi là một **lá** (lá
cây): nó là đầu mút, không nhánh nào rẽ tiếp từ đó. Thư mục **Ảnh** trong
ví dụ trên là một lá; **Tài liệu** thì không, vì nó có ba thư mục con.
::::

::::example{#thu-muc-tai-lieu}
Byte dựng cây thư mục **Tài liệu** vừa mô tả, bằng đúng dict lồng list:

```python title=readonly
anh = {"gia_tri": "Ảnh", "con": []}
nhac = {"gia_tri": "Nhạc", "con": []}
web = {"gia_tri": "Web", "con": []}
game = {"gia_tri": "Game", "con": []}

du_an = {"gia_tri": "Dự án", "con": [web, game]}

tai_lieu = {"gia_tri": "Tài liệu", "con": [anh, nhac, du_an]}

print(tai_lieu["gia_tri"])
print(len(tai_lieu["con"]))
print(tai_lieu["con"][2]["gia_tri"])
print(tai_lieu["con"][2]["con"][0]["gia_tri"])
```

```text title=readonly
Tài liệu
3
Dự án
Web
```

`tai_lieu["con"]` là một `list` giữ BA nút — đúng ba thư mục con trực
tiếp. Chỉ số `[2]` lấy đúng nút **Dự án** — nút thứ ba trong danh sách
con của **Tài liệu**. Và `["con"][0]` đi tiếp một tầng nữa, từ **Dự án**
xuống **Web** — cháu của **Tài liệu**, không phải con trực tiếp.

So với danh sách liên kết: ở đó, đi từ một nút chỉ có một hướng
(`nut["tiep"]`). Ở đây, đi từ `tai_lieu` có tới BA hướng khác nhau
(`tai_lieu["con"][0]`, `[1]`, `[2]`), và mỗi hướng lại có thể rẽ tiếp
theo cách riêng của nó. Đó là cái giá phải trả để có PHÂN CẤP: nhiều hơn
một cách để đi tiếp từ một nút.
::::

::::predict{#dem-nhanh-mang commitOnce}
Byte dựng sơ đồ mạng máy tính trong nhà — cũng là một cây thư mục kiểu
này, chỉ khác chủ đề:

```python
mang = {
    "gia_tri": "Mạng",
    "con": [
        {"gia_tri": "Có dây", "con": [
            {"gia_tri": "Cáp A", "con": []},
        ]},
        {"gia_tri": "Không dây", "con": [
            {"gia_tri": "Wifi nhà", "con": []},
            {"gia_tri": "Wifi quán", "con": []},
        ]},
        {"gia_tri": "Bluetooth", "con": []},
    ],
}
```

**Trước khi đếm tay**, bạn đoán: nút gốc `mang` có bao nhiêu nhánh con
**TRỰC TIẾP** — không tính cháu, chỉ tính con ngay bên dưới nó?

:::opt{correct}
3 — "Có dây", "Không dây", "Bluetooth"
:::

:::opt
6 — tổng cộng "Có dây", "Cáp A", "Không dây", "Wifi nhà", "Wifi quán",
"Bluetooth"
::why
Gần đúng ở việc bạn đếm đúng — sáu cái tên đó thật sự đều nằm đâu đó
trong cây, không bịa thêm cái nào.

Chỗ lệch là câu hỏi hỏi con TRỰC TIẾP của `mang`, không phải TOÀN BỘ hậu
duệ của nó. "Cáp A" là cháu của `mang` (con của "Có dây"), không phải
con trực tiếp — nó nằm trong `mang["con"][0]["con"]`, một tầng sâu hơn
`mang["con"]`. Đếm hậu duệ và đếm con trực tiếp là hai câu hỏi khác nhau.
::
:::

:::opt
4 — "Cáp A", "Wifi nhà", "Wifi quán", "Bluetooth", vì đó là những nút
không còn con nào nữa
::why
Gần đúng ở việc bốn cái tên đó đúng là những LÁ của cây này — nút không
có con nào, đầu mút của mọi nhánh. Bạn nhận diện lá đúng.

Chỗ lệch là câu hỏi không hỏi về LÁ. "Con trực tiếp của gốc" và "lá của
cả cây" là hai tập hợp khác hẳn nhau — ba nút con trực tiếp của `mang`
("Có dây", "Không dây", "Bluetooth") không nút nào trong số đó là lá cả,
trừ "Bluetooth".
::
:::

:::opt
1 — vì một nút chỉ nên có một nhánh chính, giống danh sách liên kết
::why
Gần đúng ở việc đó đúng là luật của danh sách liên kết (bài 13) — một
nút, một trường `tiep`, một hướng đi tiếp.

Chỗ lệch là cây KHÔNG giữ luật đó. Bài này vừa nới đúng giới hạn ấy ra:
trường `"con"` là một `list`, có thể giữ nhiều hơn một nút. `mang["con"]`
ở đây có ba phần tử, không phải một — nhìn thẳng vào đoạn mã sẽ thấy
`len(mang["con"])` không phải 1.
::
:::
::::

::::code{#them-thu-muc-con}
Thư mục **Máy tính** đã có sẵn một ổ đĩa. Byte muốn thêm một thư mục con
mới — **Trình duyệt** — vào ĐÚNG bên trong **Máy tính**, không phải vào
bất kỳ thư mục nào khác. Sau đó đếm xem **Ứng dụng** (thư mục cha của
**Máy tính**) có bao nhiêu thư mục con TRỰC TIẾP.

```python title=starter
o_dia_c = {"gia_tri": "Ổ đĩa C", "con": []}
may_tinh = {"gia_tri": "Máy tính", "con": [o_dia_c]}
tien_ich = {"gia_tri": "Tiện ích", "con": []}
ung_dung = {"gia_tri": "Ứng dụng", "con": [may_tinh, tien_ich]}

trinh_duyet = {"gia_tri": "Trình duyệt", "con": []}
___                                # thêm trinh_duyet vào con của may_tinh — GIỮ NGUYÊN o_dia_c đã có

so_con_truc_tiep = len(___)        # đếm số nhánh con TRỰC TIẾP của ung_dung

print(f"Ứng dụng có {so_con_truc_tiep} thư mục con trực tiếp")
print(f"Máy tính có {len(may_tinh['con'])} thư mục con")
print(sorted(c["gia_tri"] for c in may_tinh["con"]))
```

```python title=solution
o_dia_c = {"gia_tri": "Ổ đĩa C", "con": []}
may_tinh = {"gia_tri": "Máy tính", "con": [o_dia_c]}
tien_ich = {"gia_tri": "Tiện ích", "con": []}
ung_dung = {"gia_tri": "Ứng dụng", "con": [may_tinh, tien_ich]}

trinh_duyet = {"gia_tri": "Trình duyệt", "con": []}
may_tinh["con"].append(trinh_duyet)

so_con_truc_tiep = len(ung_dung["con"])

print(f"Ứng dụng có {so_con_truc_tiep} thư mục con trực tiếp")
print(f"Máy tính có {len(may_tinh['con'])} thư mục con")
print(sorted(c["gia_tri"] for c in may_tinh["con"]))
```

```python title=test
assert so_con_truc_tiep == 2, "Ứng dụng chỉ có hai con TRỰC TIẾP (Máy tính, Tiện ích) — thêm Trình duyệt không đụng tới tầng này"
assert len(may_tinh["con"]) == 2, "Máy tính phải có đúng hai thư mục con sau khi thêm — o_dia_c CŨ và trinh_duyet MỚI, không mất cái nào"
assert sorted(c["gia_tri"] for c in may_tinh["con"]) == ["Trình duyệt", "Ổ đĩa C"], "Máy tính phải chứa đúng hai tên này, không thiếu cái cũ và không thừa cái khác"
assert tien_ich["con"] == [], "Trình duyệt phải nằm trong Máy tính, không phải Tiện ích — đừng thêm nhầm nút"
assert ung_dung["con"] == [may_tinh, tien_ich], "đừng sửa danh sách con của ung_dung — chỉ đọc nó để đếm"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất phải THÊM một phần tử vào một list đang có sẵn — không phải TẠO list mới đè lên nó. Đè lên sẽ xoá mất o_dia_c đã có từ trước.
- kind: strategy
  body: 'Chỗ trống thứ nhất: may_tinh["con"] hiện là [o_dia_c] — dùng .append(trinh_duyet) để thêm trinh_duyet vào CUỐI list đó, giữ nguyên o_dia_c. Chỗ trống thứ hai: "con TRỰC TIẾP của ung_dung" nghĩa là chính list ung_dung["con"] — đếm độ dài của nó bằng len(ung_dung["con"]).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `may_tinh["con"].append(trinh_duyet)` và `ung_dung["con"]`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống thứ hai phải THẬT SỰ đếm bằng len(...) trên chính ung_dung["con"] — không được gõ thẳng con số 2 dù nó đúng cho trường hợp này; bài đang dạy cách ĐẾM một danh sách con, không phải cách đoán con số
  requireAst:
  - kind: uses-call, target: len, min: 2
  # min: 2, không phải 1 — đếm thật trên solution cho thấy len() xuất hiện
  # đúng hai lần: một lần ở chỗ trống (len(ung_dung["con"])), một lần đã có
  # sẵn trong khung ở dòng print cuối (len(may_tinh['con'])). Một lời giải
  # hardcode `so_con_truc_tiep = 2` vẫn qua mọi assert (2 đúng là con số
  # thật của TRƯỜNG HỢP NÀY) nhưng chỉ còn 1 lần gọi len() — min: 2 bắt
  # được đúng chỗ hụt này.
  - kind: uses-name, target: ung_dung, min: 1
  # `uses-call len min:2` KHÔNG xét đối số của lời gọi — nó đếm được y hệt
  # dù chỗ trống gọi len(ung_dung["con"]) hay len(may_tinh["con"]). Trên
  # đúng dữ liệu bài này, `len(may_tinh["con"])` sau khi thêm trinh_duyet
  # cũng RA 2 — TRÙNG hệt len(ung_dung["con"]) — nên đổi tên biến đọc
  # trong lời gọi (ung_dung -> may_tinh) qua lọt run/tests/output NGUYÊN
  # VẸN, chấm một cảnh không phân biệt được (đã chạy thử thật trên
  # Pyodide để xác nhận). Đếm thật trên solution: "ung_dung" chỉ bị ĐỌC
  # đúng 1 lần trong toàn khối (chính tại chỗ trống) — min: 1 bắt đúng
  # chỗ hụt: đổi sang may_tinh thì lần đọc "ung_dung" duy nhất đó biến
  # mất.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Ứng dụng có 2 thư mục con trực tiếp\\nMáy tính có 2 thư mục con\\n\\['Trình duyệt', 'Ổ đĩa C'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một nút, ba con, rồi bốn con — không giới hạn. Danh sách liên kết chỉ mơ
được một đường; cây có cả một danh sách đường để chọn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cây hôm nay cho phép một nút có bao nhiêu con TUỲ Ý — không nhánh trên,
không giới hạn dưới. Nhưng "tuỳ ý" cũng có giá của nó: muốn biết một nút
có bao nhiêu con, bạn phải đếm cả một `list`, có thể dài, có thể ngắn,
không đoán trước được.

Nếu ta ép luật đó lại — mỗi nút chỉ được có TỐI ĐA HAI nhánh, không hơn
— việc gì sẽ đơn giản đi? Và cái cây đó có còn xứng đáng gọi là "cây"
không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
