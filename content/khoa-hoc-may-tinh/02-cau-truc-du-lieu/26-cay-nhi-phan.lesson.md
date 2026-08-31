---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.cay-nhi-phan
title: "Cây nhị phân: mỗi nút tối đa hai nhánh"
summary: "Giới hạn số nhánh của một nút xuống đúng hai — trái và phải — để có một cấu trúc đơn giản, đều đặn, dễ suy luận. Phần lớn cây trong thực hành lập trình là cây nhị phân, không phải cây tổng quát."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.binary-tree]
requires: [ds.tree, core.dict, core.fstring]
concepts: [ds.binary-tree]
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
Cây bài trước cho một nút bao nhiêu nhánh tuỳ thích. Hôm nay mình khoá nó
lại: đúng hai, không hơn.
::::

::::explain{#trai-va-phai}
Bài 25 để trường `"con"` là một `list` — dài bao nhiêu tuỳ số nút con
thật có. Linh hoạt, nhưng cũng khó đoán: một nút có thể có 0 con, có thể
có 5 con, và mã xử lý nó phải lo cho MỌI trường hợp đó.

**Cây nhị phân** (binary tree) đổi luật: bỏ `list` "con" đi, thay bằng
đúng HAI trường cố định, mỗi trường giữ MỘT nút (hoặc `None` nếu nhánh
đó không có ai):

```python title=readonly
nut = {"gia_tri": ..., "trai": None, "phai": None}
```

`trai` là nhánh bên trái, `phai` là nhánh bên phải. Một nút không có
nhánh nào thì cả hai đều `None` — đó là một lá. Một nút chỉ có một nhánh
thì trường còn lại vẫn phải có mặt, chỉ là mang giá trị `None` — không
được bỏ hẳn trường đó đi, vì mọi nút cây nhị phân đều có ĐÚNG hai trường
này, dù đang dùng hay không.

Đổi lại sự linh hoạt, bạn được một cấu trúc ĐỀU: đi từ bất kỳ nút nào,
chỉ có đúng hai câu hỏi cần hỏi — "có nhánh trái không, có nhánh phải
không" — không bao giờ phải đếm độ dài một `list` trước khi biết phải
lặp bao nhiêu lần. Phần lớn cây bạn sẽ gặp trong lập trình — cây tìm
kiếm, cây biểu thức, cây quyết định — đều là cây nhị phân, không phải
cây tổng quát của bài 25.
::::

::::example{#gia-dau-loai-truc-tiep}
Byte dựng sơ đồ một giải đấu loại trực tiếp bốn đội — mỗi trận đấu có
đúng hai đối thủ, không hơn không kém, khớp y hệt luật `trai`/`phai`:

```python title=readonly
doi_rong = {"gia_tri": "Đội Rồng", "trai": None, "phai": None}
doi_ho = {"gia_tri": "Đội Hổ", "trai": None, "phai": None}
doi_soi = {"gia_tri": "Đội Sói", "trai": None, "phai": None}
doi_cao = {"gia_tri": "Đội Cáo", "trai": None, "phai": None}

ban_ket_1 = {"gia_tri": "Bán kết 1", "trai": doi_rong, "phai": doi_ho}
ban_ket_2 = {"gia_tri": "Bán kết 2", "trai": doi_soi, "phai": doi_cao}

chung_ket = {"gia_tri": "Chung kết", "trai": ban_ket_1, "phai": ban_ket_2}

print(chung_ket["gia_tri"])
print(chung_ket["trai"]["gia_tri"])
print(chung_ket["trai"]["trai"]["gia_tri"])
```

```text title=readonly
Chung kết
Bán kết 1
Đội Rồng
```

Bốn đội là bốn LÁ (cả `trai` lẫn `phai` đều `None`). Hai trận bán kết là
hai nút ở giữa, mỗi nút có đúng hai nhánh trỏ tới hai đội. Trận chung
kết là gốc, hai nhánh của nó trỏ tới hai trận bán kết — không phải tới
đội, mà tới CẢ MỘT TRẬN, chính là một cây con.

`chung_ket["trai"]["trai"]["gia_tri"]` đi hai bước liên tiếp: từ chung
kết rẽ trái tới **Bán kết 1**, rồi từ đó rẽ trái tiếp tới **Đội Rồng**.
Mỗi bước `["trai"]` hay `["phai"]` là một lần rẽ nhánh — chuỗi chỉ số
càng dài, bạn càng đi sâu vào cây.
::::

::::predict{#doan-mot-buoc commitOnce}
Byte dựng một mảnh cây khác — không phải giải đấu, chỉ ba nút để thử
chỉ số:

```python
la = {"gia_tri": "Sói", "trai": None, "phai": None}
nhanh = {"gia_tri": "Vòng 1", "trai": la, "phai": None}
goc = {"gia_tri": "Chung kết", "trai": nhanh, "phai": la}

print(goc["trai"]["trai"]["gia_tri"])
```

**Trước khi chạy**, bạn đoán dòng in ra là gì?

:::opt{correct}
`Sói`
:::

:::opt
`Vòng 1`
::why
Gần đúng ở việc bạn đi ĐÚNG một bước đầu tiên: `goc["trai"]` thật sự là
`nhanh`, nút mang giá trị "Vòng 1" — bước đó bạn đọc không sai.

Chỗ lệch là biểu thức còn chỉ số `["trai"]` MỘT LẦN NỮA sau đó. Phải đi
tiếp thêm một bước, từ `nhanh` rẽ trái xuống `la` — dừng lại ở "Vòng 1"
là dừng giữa đường, chưa hết chuỗi chỉ số.
::
:::

:::opt
`Chung kết`
::why
Gần đúng ở việc "Chung kết" đúng là giá trị của biến `goc` — bạn không
đọc nhầm tên biến.

Chỗ lệch là dòng in không in `goc["gia_tri"]`, nó in
`goc["trai"]["trai"]["gia_tri"]` — đã rẽ khỏi gốc những HAI bước rồi mới
đọc `"gia_tri"`. Giá trị ở gốc không còn liên quan tới kết quả cuối
cùng nữa.
::
:::

:::opt
Chương trình dừng lại và báo lỗi, vì `la` không có nhánh trái
::why
Gần đúng ở việc bạn để ý đúng: `la["trai"]` là `None`, không có nhánh
nào ở đó — phản xạ cảnh giác với `None` là đúng khi làm việc với cây.

Chỗ lệch là biểu thức KHÔNG đi tới `la["trai"]`. Đếm lại số lần `["trai"]`
xuất hiện: đúng hai lần, `goc["trai"]["trai"]`, và `goc["trai"]["trai"]`
CHÍNH LÀ `la` — biểu thức dừng lại ở đó để đọc `["gia_tri"]` của `la`,
chưa từng đi thêm một bước `["trai"]` thứ ba nào để chạm tới `None`.
::
:::
::::

::::code{#them-ban-ket-hai}
Giải đấu có bốn đội: Rồng, Hổ, Sói, Cáo. Byte đã dựng xong Bán kết 1
(Rồng gặp Hổ). Bạn dựng nốt Bán kết 2 (Sói bên trái, Cáo bên phải), ghép
vào Chung kết, rồi lấy tên đội đứng bên TRÁI của Bán kết 2 — bằng cách
đi qua `chung_ket`, không phải gọi thẳng tên biến `doi_soi`.

```python title=starter
doi_rong = {"gia_tri": "Đội Rồng", "trai": None, "phai": None}
doi_ho = {"gia_tri": "Đội Hổ", "trai": None, "phai": None}
doi_soi = {"gia_tri": "Đội Sói", "trai": None, "phai": None}
doi_cao = {"gia_tri": "Đội Cáo", "trai": None, "phai": None}

ban_ket_1 = {"gia_tri": "Bán kết 1", "trai": doi_rong, "phai": doi_ho}
ban_ket_2 = ___                    # Bán kết 2: trái là doi_soi, phải là doi_cao — dùng đúng hai biến này

chung_ket = {"gia_tri": "Chung kết", "trai": ban_ket_1, "phai": ban_ket_2}

doi_trai_ban_ket_2 = ___           # lấy tên đội bên TRÁI của Bán kết 2, đi từ chung_ket

print(chung_ket["gia_tri"])
print(chung_ket["trai"]["gia_tri"])
print(chung_ket["phai"]["gia_tri"])
print(doi_trai_ban_ket_2)
```

```python title=solution
doi_rong = {"gia_tri": "Đội Rồng", "trai": None, "phai": None}
doi_ho = {"gia_tri": "Đội Hổ", "trai": None, "phai": None}
doi_soi = {"gia_tri": "Đội Sói", "trai": None, "phai": None}
doi_cao = {"gia_tri": "Đội Cáo", "trai": None, "phai": None}

ban_ket_1 = {"gia_tri": "Bán kết 1", "trai": doi_rong, "phai": doi_ho}
ban_ket_2 = {"gia_tri": "Bán kết 2", "trai": doi_soi, "phai": doi_cao}

chung_ket = {"gia_tri": "Chung kết", "trai": ban_ket_1, "phai": ban_ket_2}

doi_trai_ban_ket_2 = chung_ket["phai"]["trai"]["gia_tri"]

print(chung_ket["gia_tri"])
print(chung_ket["trai"]["gia_tri"])
print(chung_ket["phai"]["gia_tri"])
print(doi_trai_ban_ket_2)
```

```python title=test
assert ban_ket_2["trai"] is doi_soi, "Bán kết 2 phải có doi_soi ở nhánh trai — đừng đổi biến"
assert ban_ket_2["phai"] is doi_cao, "Bán kết 2 phải có doi_cao ở nhánh phai — đừng đổi biến"
assert chung_ket["phai"] is ban_ket_2, "chung_ket phải nối tới ban_ket_2 ở nhánh phai — dòng này đã có sẵn, đừng sửa"
assert doi_trai_ban_ket_2 == "Đội Sói", f"đội bên trái của Bán kết 2 phải là 'Đội Sói' — đang ra {doi_trai_ban_ket_2!r}"
```

:::hints
- kind: attention
  body: Chỗ trống thứ hai phải đi qua chung_ket, không phải gọi thẳng tên biến doi_soi hay ban_ket_2 — đề bài yêu cầu đi từ gốc xuống, đúng như ví dụ phía trên đã làm với chung_ket["trai"]["trai"].
- kind: strategy
  body: 'Chỗ trống thứ nhất: sao chép đúng khuôn của ban_ket_1, chỉ đổi trai thành doi_soi và phai thành doi_cao. Chỗ trống thứ hai: chung_ket["phai"] chính là ban_ket_2 (Bán kết 2, vì bạn vừa gán nó vào phai của chung_ket) — rẽ thêm một bước ["trai"] nữa từ đó, rồi lấy ["gia_tri"].'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `{"gia_tri": "Bán kết 2", "trai": doi_soi, "phai": doi_cao}` và `chung_ket["phai"]["trai"]["gia_tri"]`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống thứ hai phải đi qua chung_ket rồi rẽ nhánh — không được lấy thẳng tên biến doi_soi hay ban_ket_2; bài này đang dạy cách ĐI QUA CÂY từ gốc, không phải cách nhớ tên biến
  requireAst:
  - kind: uses-name, target: chung_ket, min: 4
  # min: 4, không phải 1 — đếm thật trên solution: while while chung_ket bị
  # ĐỌC bốn lần — ba lần trong ba dòng print có sẵn trong khung
  # (print(chung_ket["gia_tri"]), print(chung_ket["trai"]["gia_tri"]),
  # print(chung_ket["phai"]["gia_tri"])), cộng một lần nữa ở chính chỗ trống
  # (chung_ket["phai"]["trai"]["gia_tri"]). Một lời giải hụt kiểu
  # `doi_trai_ban_ket_2 = doi_soi["gia_tri"]` vẫn ra ĐÚNG chữ "Đội Sói" (vì
  # doi_soi và chung_ket["phai"]["trai"] là CÙNG một nút) và qua hết mọi
  # assert giá trị — chỉ còn 3 lần đọc chung_ket, dưới ngưỡng 4, nên static
  # bắt được đúng chỗ hụt mà test/output không thấy.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Chung kết\\nBán kết 1\\nBán kết 2\\nĐội Sói\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai nhánh, không hơn không kém, ở MỌI nút trong toàn giải đấu — dù cây
có bao nhiêu tầng đi nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cây nhị phân hôm nay chỉ định HÌNH DẠNG: trái, phải, đúng hai chỗ. Nhưng
chưa có luật nào nói GIÁ TRỊ nào phải đứng ở nhánh nào — Đội Sói đứng
bên trái hay bên phải Bán kết 2 hoàn toàn tuỳ Byte gõ, chẳng theo quy tắc
gì cả. Đổi `trai` và `phai` cho nhau, cây vẫn hợp lệ y như cũ.

Nếu ta BẮT BUỘC thêm một luật: tại mọi nút, nhánh trái chỉ được chứa
những giá trị NHỎ HƠN nút đó, nhánh phải chỉ được chứa giá trị LỚN HƠN —
cái cây có luật đó dùng để làm được việc gì mà cây tự do hôm nay chưa
làm được?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
