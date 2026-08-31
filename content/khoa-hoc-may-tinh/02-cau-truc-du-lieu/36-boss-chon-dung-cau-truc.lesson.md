---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.boss-chon-dung-cau-truc
title: "BOSS — Chọn đúng cấu trúc cho đúng việc"
summary: "Không khái niệm mới — năm tình huống thật, mỗi tình huống phải ghép đúng cấu trúc dữ liệu đã học, kèm một câu giải thích: chọn sai thì hỏng đúng ở chỗ nào."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 36
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.choose-structure]
requires: [ds.graph-representation, ds.dict-is-hash-table, ds.tree-traversal, ds.array-vs-linked-tradeoff, ds.stack-vs-queue-choice, core.dict-iter-keys, ctrl.for-range, core.list-append, core.fstring]
concepts: [ds.choose-structure]
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
Ba mươi lăm bài để tới đây. Không bài mới nào dạy thêm — hôm nay chỉ hỏi:
gặp một việc thật, bạn có chọn đúng hộp không?
::::

::::explain{#nam-tinh-huong-that}
Không có khái niệm mới trong bài này. Năm tình huống dưới đây đều dùng
đúng những cấu trúc track này đã mở nắp — việc của bạn là ghép đúng, và
biết vì sao ghép SAI lại hỏng.

1. **Danh bạ tra nhanh** — gõ tên, cần số điện thoại NGAY. Đúng: **bảng
   băm** (bài 19–24). Sai — chọn mảng chưa sắp xếp hay danh sách liên
   kết: mỗi lần tra phải DÒ hoặc ĐI BỘ qua từng mục, không có phép tính
   chỉ số nào nhảy thẳng tới đúng tên như bảng băm làm được.

2. **Hàng chờ máy in** — ai gửi lệnh in trước, tài liệu người đó phải ra
   trước, không ai được chen ngang. Đúng: **hàng đợi** (bài 9). Sai —
   chọn ngăn xếp: tài liệu gửi SAU CÙNG lại in TRƯỚC TIÊN, đảo ngược
   đúng thứ tự người dùng mong đợi.

3. **Thư mục lồng nhau** — thư mục chứa thư mục con, thư mục con lại
   chứa tệp. Đúng: **cây** (bài 25). Sai — chọn đồ thị tổng quát: đồ thị
   cho phép một tệp nằm trong nhiều thư mục cha cùng lúc, và cho phép
   vòng — hai điều thư mục thật không bao giờ cần, và cho phép chúng chỉ
   tổ mở đường cho những lỗi kỳ quặc (một thư mục nằm trong chính nó).

4. **Mạng bạn bè** — ai quen ai, không ai là "cha" của ai, và ba người
   hoàn toàn có thể quen biết khép vòng lẫn nhau. Đúng: **đồ thị** (bài
   34). Sai — chọn cây: cây cấm hẳn việc một đỉnh có nhiều hơn một "cha"
   và cấm mọi vòng — không tả nổi một nhóm bạn quen biết khép kín.

5. **Undo trong trình soạn thảo** — bấm undo phải hoàn tác đúng hành
   động VỪA LÀM gần đây nhất, không phải hành động đầu tiên của phiên
   làm việc. Đúng: **ngăn xếp** (bài 7). Sai — chọn hàng đợi: undo sẽ
   hoàn tác nhầm hành động ĐẦU TIÊN, trong khi người dùng đang muốn gỡ
   thao tác vừa gõ.
::::

::::example{#vi-sao-danh-ba-la-bang-bam}
Xét kỹ tình huống 1. Giả sử một thực tập sinh đề xuất lưu danh bạ bằng
`list` các cặp `(tên, số)`, tra cứu bằng vòng lặp:

```python title=readonly
danh_ba_mang = [("An", "0901..."), ("Bình", "0902..."), ("Chi", "0903...")]

def tra_bang_mang(ten):
    for (t, so) in danh_ba_mang:
        if t == ten:
            return so
    return None

danh_ba_bam = {"An": "0901...", "Bình": "0902...", "Chi": "0903..."}

def tra_bang_bam(ten):
    return danh_ba_bam.get(ten)

print(tra_bang_mang("Chi"))
print(tra_bang_bam("Chi"))
```

```text title=readonly
0903...
0903...
```

Cả hai cho ra ĐÚNG cùng một kết quả — output không phân biệt được chúng.
Khác biệt chỉ lộ ra khi nhìn vào CÁCH LÀM: `tra_bang_mang` phải đi qua
từng mục một, so tên, tới khi tìm đúng hoặc hết danh sách — càng nhiều
người trong danh bạ, càng có thể phải dò xa hơn. `tra_bang_bam` băm cái
tên (bài 19) rồi nhảy thẳng tới đúng ô (bài 20) — không dò ai cả. Với ba
người thì chênh lệch không thấy được; với một triệu người, `tra_bang_mang`
có thể phải đi qua gần hết cả triệu mục, còn `tra_bang_bam` vẫn nhảy
thẳng một bước.
::::

::::predict{#doan-thu-muc-long-nhau commitOnce}
Một thực tập sinh khác đề xuất lưu cấu trúc thư mục bằng **đồ thị tổng
quát** (bảng kề, bài 35) thay vì cây — lý giải: "đồ thị tổng quát hơn
cây, dùng nó thì không bao giờ thiếu khả năng gì."

**Trước khi trả lời**, bạn đoán: đề xuất này có ổn không?

:::opt{correct}
Không ổn — đồ thị tổng quát CHO PHÉP một tệp nằm trong nhiều thư mục cha
cùng lúc và cho phép vòng, hai điều một hệ thống thư mục thật không cần
và không nên có. Dùng cây buộc đúng luật đó ngay từ cấu trúc, không phải
tự canh chừng bằng tay.
:::

:::opt
Ổn — vì đồ thị tổng quát hơn nên nó LUÔN thay thế được cây trong mọi
tình huống, không đánh mất gì cả.
::why
Gần đúng ở việc đồ thị đúng là tổng quát hơn cây theo đúng nghĩa toán học
— mọi cây đều là một đồ thị đặc biệt (bài 34), điều đó không sai.

Chỗ lệch là "tổng quát hơn" không có nghĩa là "luôn nên dùng nó". Cây
CẤM một tệp có nhiều cha và cấm vòng NGAY TỪ CẤU TRÚC — nếu mã của bạn lỡ
tay tạo ra một trong hai điều đó bằng đồ thị tổng quát, không có gì báo
lỗi, và hệ thống tệp của bạn giờ có thể có một thư mục nằm trong chính
nó. Cấu trúc hẹp hơn đôi khi CHÍNH LÀ điểm mạnh, vì nó tự canh giữ luật
thay bạn.
::
:::

:::opt
Ổn — nhưng chỉ vì thư mục máy tính thật ra hiếm khi lồng sâu quá vài
tầng, nên đồ thị hay cây cũng chẳng khác nhau mấy.
::why
Gần đúng ở quan sát thực tế — nhiều thư mục thật đúng là không lồng quá
sâu, quan sát đó không sai.

Chỗ lệch là vấn đề không nằm ở ĐỘ SÂU của cây, mà nằm ở LUẬT của cấu
trúc. Dù thư mục có nông tới đâu, đồ thị tổng quát vẫn không hề cấm một
tệp có hai thư mục cha cùng lúc — độ sâu không liên quan gì tới lỗ hổng
luật này.
::
:::

:::opt
Không ổn — nhưng vì lý do khác: đồ thị luôn tốn nhiều bộ nhớ hơn cây với
CÙNG một số đỉnh, bất kể cách biểu diễn.
::why
Gần đúng ở việc bạn nghi ngờ đúng hướng — đề xuất này thật sự có vấn đề,
kết luận "không ổn" không sai.

Chỗ lệch là lý do bộ nhớ không phải điểm mấu chốt ở đây. Một đồ thị biểu
diễn ĐÚNG một cây (không thêm cạnh thừa) tốn bộ nhớ gần như ngang cây —
vấn đề thật không phải TỐN BAO NHIÊU Ô, mà là đồ thị tổng quát không hề
NGĂN người viết mã tạo ra một hình dạng không còn là cây nữa (nhiều cha,
có vòng), trong khi cây thì ngăn được điều đó ngay từ luật của nó.
::
:::
::::

::::code{#kiem-de-xuat-thuc-tap-sinh}
Một thực tập sinh gửi bảng đề xuất cấu trúc cho cả năm tình huống trên.
Bạn viết đoạn mã so từng đề xuất với đáp án đúng, gom lại những tình
huống bị đề xuất SAI.

```python title=starter
tinh_huong = {
    "danh_ba": "bảng băm",
    "hang_cho_may_in": "hàng đợi",
    "thu_muc": "cây",
    "mang_ban_be": "đồ thị",
    "undo_soan_thao": "ngăn xếp",
}

de_xuat_cua_thuc_tap_sinh = {
    "danh_ba": "bảng băm",
    "hang_cho_may_in": "ngăn xếp",
    "thu_muc": "cây",
    "mang_ban_be": "danh sách liên kết",
    "undo_soan_thao": "ngăn xếp",
}

de_xuat_sai = []
for ten_tinh_huong in tinh_huong:
    if de_xuat_cua_thuc_tap_sinh[ten_tinh_huong] != ___:
        de_xuat_sai.append(___)

print(f"Số đề xuất sai: {len(de_xuat_sai)}")
print(f"Sai ở: {de_xuat_sai}")
```

```python title=solution
tinh_huong = {
    "danh_ba": "bảng băm",
    "hang_cho_may_in": "hàng đợi",
    "thu_muc": "cây",
    "mang_ban_be": "đồ thị",
    "undo_soan_thao": "ngăn xếp",
}

de_xuat_cua_thuc_tap_sinh = {
    "danh_ba": "bảng băm",
    "hang_cho_may_in": "ngăn xếp",
    "thu_muc": "cây",
    "mang_ban_be": "danh sách liên kết",
    "undo_soan_thao": "ngăn xếp",
}

de_xuat_sai = []
for ten_tinh_huong in tinh_huong:
    if de_xuat_cua_thuc_tap_sinh[ten_tinh_huong] != tinh_huong[ten_tinh_huong]:
        de_xuat_sai.append(ten_tinh_huong)

print(f"Số đề xuất sai: {len(de_xuat_sai)}")
print(f"Sai ở: {de_xuat_sai}")
```

```python title=test
assert de_xuat_sai == ["hang_cho_may_in", "mang_ban_be"], f"chỉ đúng hai đề xuất sai — hàng chờ máy in (đề xuất nhầm ngăn xếp) và mạng bạn bè (đề xuất nhầm danh sách liên kết) — đang báo sai ở {de_xuat_sai}"
```

:::hints
- kind: attention
  body: Hai chỗ trống làm hai việc khác nhau — chỗ trong `if` phải SO SÁNH đề xuất với đáp án đúng, chỗ trong `append` phải GHI LẠI tên tình huống đang xét, không phải giá trị nào khác.
- kind: strategy
  body: 'Đáp án đúng của tình huống đang xét nằm ở `tinh_huong[ten_tinh_huong]` — so nó với đề xuất bằng `!=`. Khi khác nhau, cái cần ghi vào `de_xuat_sai` là chính cái TÊN tình huống đó — `ten_tinh_huong` — để biết sai ở đâu, không phải ghi giá trị đúng hay giá trị sai.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `tinh_huong[ten_tinh_huong]` và `ten_tinh_huong`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống trong điều kiện if phải THẬT SỰ tra cứu đáp án đúng từ tinh_huong[ten_tinh_huong] — không gõ thẳng một tên cấu trúc cố định — vì đề xuất đúng hay sai còn tuỳ thuộc tình huống đang xét, không phải một hằng số
  requireAst:
  - kind: uses-name, target: tinh_huong, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Số đề xuất sai: 2\\nSai ở: \\['hang_cho_may_in', 'mang_ban_be'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai đề xuất sai, tìm đúng cả hai. Ba mươi lăm bài trước không dạy bạn một
câu trả lời — chúng dạy bạn cách tự tìm ra đáp án cho từng việc mới.
::::

::::reflect{#nghi-lai}
Một câu hỏi cuối, khép lại cả track này.

Suốt cả track, mọi lời so sánh tốc độ đều dừng ở mức ĐỊNH TÍNH: "phải dò
cả bảng", "phải đi bộ từ đầu", "tính thẳng bằng công thức, không cần
dò". Chưa một bài nào gán cho những câu đó một CON SỐ hay một CÁI TÊN
chính thức.

R3.T3.3 — Thuật toán & độ phức tạp — sẽ đặt tên chính thức cho đúng những
câu định tính ấy: **Big-O**. "Phải dò cả bảng" sẽ có một ký hiệu, "tính
thẳng bằng công thức" sẽ có một ký hiệu khác, và bạn sẽ đo được CHÍNH XÁC
khoảng cách giữa chúng, không chỉ cảm nhận nó qua từng ví dụ như track
này vừa làm.

Hẹn gặp lại ở đó.
::::

::::checkpoint{mastery=0.8}
::::
