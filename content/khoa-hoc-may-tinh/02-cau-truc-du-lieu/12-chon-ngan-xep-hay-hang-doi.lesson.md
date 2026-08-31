---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.chon-ngan-xep-hay-hang-doi
title: "Chọn ngăn xếp hay hàng đợi cho đúng việc"
summary: "Bài chốt cụm: cho vài tình huống thật — undo trong trình soạn thảo, hàng chờ máy in, chuỗi hàm gọi nhau — chọn đúng ngăn xếp hay hàng đợi, và nói ra được TẠI SAO cấu trúc kia sai."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.stack-vs-queue-choice]
requires: [ds.stack-application, ds.circular-queue, core.list, core.list-append, core.variable, core.assignment, core.fstring]
concepts: [ds.stack-vs-queue-choice]
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
Không có bài học mới hôm nay — chỉ có việc dùng đúng thứ bạn đã có, cho
đúng việc.
::::

::::explain{#hai-cau-hoi-de-chon}
Sáu bài vừa qua dựng hai cấu trúc, và mỗi cấu trúc trả lời đúng một câu
hỏi:

- **Ngăn xếp** (bài 7–8): "Việc nào GẦN ĐÂY NHẤT phải được xử lý trước?"
  Dùng khi thứ tự xử lý phải NGƯỢC với thứ tự xảy ra — undo trong trình
  soạn thảo (huỷ thao tác vừa làm gần nhất, không phải thao tác đầu
  tiên), nút "Quay lại" của trình duyệt (về trang vừa rời, không phải
  trang đầu tiên bạn từng mở).
- **Hàng đợi** (bài 9–11): "Ai ĐẾN TRƯỚC phải được phục vụ trước?" Dùng
  khi công bằng theo thứ tự đến là điều bắt buộc — hàng chờ máy in (tài
  liệu gửi trước phải in trước, không được chen ngang), hàng chờ mua vé.

Có một chỗ ngăn xếp xuất hiện mà bạn chưa từng gọi đúng tên nó: R1.T1.3
từng dạy "máy nhớ đường về" — hàm này gọi hàm kia, máy phải nhớ ĐƯỜNG QUAY
LẠI đúng thứ tự ngược. Hàm gọi SAU CÙNG luôn phải chạy xong TRƯỚC, rồi mới
quay về đúng hàm đã gọi nó — vào sau, ra trước. Đó chính là một ngăn xếp,
dù bài học ngày đó chưa gọi tên nó.

Chọn sai cấu trúc không phải lỗi cú pháp — chương trình vẫn chạy, chỉ là
nó chạy SAI THỨ TỰ. Một hàng chờ máy in dựng bằng ngăn xếp sẽ in tài liệu
mới nhất trước, bỏ đói tài liệu gửi từ đầu buổi sáng.
::::

::::example{#hai-viec-mot-luc}
Byte xử lý cùng lúc hai việc khác hẳn nhau: một lượt "hoàn tác" (undo)
trong trình soạn thảo, và hai tài liệu được máy in xử lý xong.

```python title=readonly
lich_su = []
lich_su.append("gõ tiêu đề")
lich_su.append("gõ đoạn mở đầu")
lich_su.append("gõ chữ ký")

hanh_dong_vua_undo = lich_su.pop()
print(f"Undo: {hanh_dong_vua_undo}")
print(f"Còn lại: {lich_su}")

hang_in = []
hang_in.append("báo cáo")
hang_in.append("hoá đơn")

tai_lieu_da_in = hang_in.pop(0)
print(f"Đã in: {tai_lieu_da_in}")
print(f"Hàng in còn lại: {hang_in}")
```

```text title=readonly
Undo: gõ chữ ký
Còn lại: ['gõ tiêu đề', 'gõ đoạn mở đầu']
Đã in: báo cáo
Hàng in còn lại: ['hoá đơn']
```

`lich_su.pop()` không tham số lấy đúng thao tác GẦN ĐÂY NHẤT ("gõ chữ
ký") — undo phải huỷ việc vừa làm, không phải việc đầu tiên. `hang_in.pop(0)`
lấy đúng tài liệu ĐẾN TRƯỚC nhất ("báo cáo") — máy in phải công bằng theo
thứ tự gửi, không được ưu tiên tài liệu mới hơn. Cùng dùng `.append` để
đưa vào cả hai nơi, nhưng cách lấy ra hoàn toàn khác nhau, vì hai việc đòi
hỏi hai luật khác nhau.
::::

::::predict{#chuoi-goi-ham commitOnce}
Chương trình của Byte gọi hàm `A()`. Bên trong `A()`, nó gọi `B()`. Bên
trong `B()`, nó gọi `C()`. Máy phải nhớ đường quay lại ở mỗi lượt gọi —
đúng cơ chế R1.T1.3 đã dạy.

**Trước khi trả lời**, bạn đoán: ngay khi `C()` vừa chạy xong, hàm nào
tiếp tục chạy tiếp?

:::opt{correct}
Hàm `B()` — vì nó là hàm đã GỌI `C()`, và máy luôn quay về đúng lượt gọi
GẦN ĐÂY NHẤT trước, giống hệt ngăn xếp.
:::

:::opt
Hàm `A()` — vì `A()` là hàm ĐẦU TIÊN được gọi trong cả chuỗi, nên máy ưu
tiên quay lại nó trước.
::why
Gần đúng ở việc bạn nhớ đúng `A()` có mặt trong chuỗi gọi và cuối cùng
CŨNG sẽ chạy tiếp — điều đó đúng, chỉ là chưa phải NGAY LÚC NÀY.

Chỗ lệch là bạn đang áp dụng luật "đến trước, phục vụ trước" — luật của
HÀNG ĐỢI — vào một cơ chế vốn là ngăn xếp. Máy phải trả xong "món nợ" gần
nhất trước: `C()` nợ quay về `B()`, phải trả xong nợ đó rồi `B()` mới tới
lượt trả nợ của chính nó, là quay về `A()`.
::
:::

:::opt
Cả `A()` và `B()` cùng lúc — máy quay lại toàn bộ chuỗi gọi trong một
lượt.
::why
Gần đúng ở việc bạn nhận ra CẢ HAI hàm này đều còn "treo", đang chờ được
tiếp tục — đúng, cả hai đều chưa chạy xong.

Chỗ lệch là máy không quay lại nhiều hàm cùng lúc. Mỗi lần một hàm chạy
xong, máy chỉ LẤY RA đúng một "món nợ" gần đây nhất từ ngăn xếp lời gọi —
`B()` trước, rồi tới lượt `B()` chạy xong mới lấy tiếp món nợ kế, là
`A()`. Từng bước một, không gộp lại.
::
:::

:::opt
Không hàm nào — chương trình dừng lại hẳn ngay khi `C()` chạy xong.
::why
Gần đúng ở việc `C()` đúng là điểm SÂU NHẤT trong chuỗi gọi — không có gì
sai khi để ý điều đó.

Chỗ lệch là "chạy xong một hàm" không có nghĩa "chương trình kết thúc".
`B()` gọi `C()` rồi ĐỢI kết quả — máy vẫn còn ghi nhớ đúng chỗ trong
`B()` cần quay lại tiếp tục, và `A()` cũng vẫn đang đợi `B()` y như vậy.
::
:::
::::

::::code{#hai-cau-truc-mot-chuong-trinh}
Trình soạn thảo có ba thao tác vừa gõ, cần hoàn tác lượt gần nhất. Hàng
chờ máy in có ba tài liệu, cần in đúng hai tài liệu đến trước nhất. Điền
đúng công cụ cho từng việc.

```python title=starter
lich_su = []
lich_su.append("gõ tiêu đề")
lich_su.append("gõ đoạn mở đầu")
lich_su.append("gõ chữ ký")

hanh_dong_vua_undo = ___                 # hoàn tác thao tác GẦN ĐÂY NHẤT

hang_in = []
hang_in.append("bao-cao.pdf")
hang_in.append("hoa-don.pdf")
hang_in.append("hop-dong.pdf")

tai_lieu_in_1 = ___                      # in tài liệu ĐẾN TRƯỚC nhất
tai_lieu_in_2 = hang_in.pop(0)

print(f"Vừa undo: {hanh_dong_vua_undo}")
print(f"Lịch sử còn lại: {lich_su}")
print(f"Đã in xong: {tai_lieu_in_1}, rồi {tai_lieu_in_2}")
print(f"Hàng in còn lại: {hang_in}")
```

```python title=solution
lich_su = []
lich_su.append("gõ tiêu đề")
lich_su.append("gõ đoạn mở đầu")
lich_su.append("gõ chữ ký")

hanh_dong_vua_undo = lich_su.pop()

hang_in = []
hang_in.append("bao-cao.pdf")
hang_in.append("hoa-don.pdf")
hang_in.append("hop-dong.pdf")

tai_lieu_in_1 = hang_in.pop(0)
tai_lieu_in_2 = hang_in.pop(0)

print(f"Vừa undo: {hanh_dong_vua_undo}")
print(f"Lịch sử còn lại: {lich_su}")
print(f"Đã in xong: {tai_lieu_in_1}, rồi {tai_lieu_in_2}")
print(f"Hàng in còn lại: {hang_in}")
```

```python title=test
assert hanh_dong_vua_undo == "gõ chữ ký", f"undo phải hoàn tác thao tác GẦN NHẤT — 'gõ chữ ký' — đang ra {hanh_dong_vua_undo!r}"
assert lich_su == ["gõ tiêu đề", "gõ đoạn mở đầu"], f"lịch sử còn lại phải đúng hai thao tác đầu — đang ra {lich_su}"
assert tai_lieu_in_1 == "bao-cao.pdf", f"tài liệu in đầu tiên phải là tài liệu ĐẾN TRƯỚC nhất — bao-cao.pdf — đang ra {tai_lieu_in_1!r}"
assert tai_lieu_in_2 == "hoa-don.pdf", f"tài liệu in thứ hai phải đúng thứ tự gửi tiếp theo — hoa-don.pdf — đang ra {tai_lieu_in_2!r}"
assert hang_in == ["hop-dong.pdf"], f"hàng in chỉ còn đúng một tài liệu chưa xử lý — đang ra {hang_in}"
```

:::hints
- kind: attention
  body: Hai chỗ trống thuộc về hai cấu trúc khác nhau, dù cả hai đều bắt đầu bằng .append(). undo cần luật ngăn xếp; in ấn cần luật hàng đợi — đừng dùng chung một công cụ cho cả hai.
- kind: strategy
  body: 'undo phải huỷ đúng thao tác vừa làm gần nhất — đó là luật ngăn xếp, dùng .pop() không tham số trên lich_su. In ấn phải công bằng theo thứ tự gửi — đó là luật hàng đợi, dùng .pop(0) trên hang_in, đúng công cụ dòng kế bên (tai_lieu_in_2) đã dùng sẵn.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `lich_su.pop()` và `hang_in.pop(0)`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống hoàn tác phải dùng .pop() không tham số (luật ngăn xếp — lấy thao tác GẦN NHẤT), chỗ trống in ấn phải dùng .pop(0) (luật hàng đợi — lấy tài liệu ĐẾN TRƯỚC nhất) — dùng lộn công cụ giữa hai cấu trúc là đúng lỗi bài này đang dạy cách tránh
  requireAst:
  # Đếm thật trên solution: .pop( xuất hiện đúng 3 lần toàn khối (undo, và
  # hai lượt in — một ở chỗ trống, một đã có sẵn trong khung). Không có cổng
  # này, hanh_dong_vua_undo = "gõ chữ ký" (hardcode đúng giá trị, không gọi
  # pop) vẫn qua static nếu chỉ nhìn suông — nhưng lich_su sẽ không co lại,
  # lộ ra ở tests/output; cổng dưới chặn sớm hơn.
  - kind: uses-call, target: pop, min: 3
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Vừa undo: gõ chữ ký\\nLịch sử còn lại: \\['gõ tiêu đề', 'gõ đoạn mở đầu'\\]\\nĐã in xong: bao-cao\\.pdf, rồi hoa-don\\.pdf\\nHàng in còn lại: \\['hop-dong\\.pdf'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một chương trình, hai luật khác nhau, đúng chỗ nào dùng luật nào — không
lẫn lộn.
::::

::::reflect{#nghi-lai}
Một câu hỏi khép lại cả cụm sáu bài vừa qua.

Ngăn xếp và hàng đợi — kể cả bản vòng vừa né được cái bẫy dồn chỗ — đều
dựng trên MẢNG, và mảng luôn kéo theo một ràng buộc: các ô phải nằm LIỀN
KỀ nhau trong bộ nhớ. Hàng đợi vòng phải biết trước sức chứa tối đa; ngăn
xếp thỉnh thoảng phải xin hẳn một vùng nhớ mới và chép hết sang khi mảng
cũ đầy.

T3.1 bài 20 từng dạy một điều tưởng chừng không liên quan: một cái tên
không GIỮ giá trị, nó chỉ TRỎ TỚI giá trị — qua một con số địa chỉ. Nếu
một Ô không cần đứng liền kề ô kế của nó trong bộ nhớ — nếu mỗi ô chỉ cần
TỰ GHI NHỚ đường tới ô tiếp theo, giống hệt một cái tên trỏ tới một giá
trị — thì có cần "mảng" nữa không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
