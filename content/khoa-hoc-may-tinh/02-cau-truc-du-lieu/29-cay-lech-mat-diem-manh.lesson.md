---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.cay-lech-mat-diem-manh
title: "Cây lệch mất hết điểm mạnh"
summary: "Chèn dữ liệu đã SẴN SẮP XẾP vào một BST theo đúng luật bài 27 dựng ra một cái cây chỉ có nhánh phải — về hình dạng, nó chính là danh sách liên kết đội lốt cây. 'Bỏ một nửa' của bài trước không còn đúng nữa."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 29
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.tree-unbalanced]
requires: [ds.bst-search, ctrl.for-each, core.list-slice, core.len]
concepts: [ds.tree-unbalanced]
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
Cùng đúng luật BST, cùng dict `trai`/`phai` y hệt bài trước — nhưng có
một cách chèn khiến "bỏ một nửa" hoá thành lời hứa suông.
::::

::::explain{#chen-du-lieu-da-sap-xep}
Mọi ví dụ từ bài 26 tới giờ đều có một điểm chung, dù không ai nói ra:
dữ liệu chèn vào không theo thứ tự nào cả — 50 rồi 30 rồi 70, nhảy lung
tung. Nhờ vậy cây có nhánh trái, có nhánh phải, cân đối tương đối.

Nhưng nếu dữ liệu tới theo đúng thứ tự TĂNG DẦN thì sao? Đó không phải
tình huống hiếm — mã đơn hàng, số thứ tự vé, dấu thời gian: rất nhiều dữ
liệu thật SẴN đã tăng dần khi chúng xuất hiện.

Theo đúng luật bài 27 (trái nhỏ hơn, phải lớn hơn), khi mỗi giá trị mới
LUÔN LỚN HƠN mọi giá trị đã có trong cây, nó LUÔN phải rẽ phải, ở MỌI
nút nó đi qua — không lần nào có cơ hội rẽ trái. Kết quả: một cây mà mọi
nút chỉ có `phai`, còn `trai` mãi mãi là `None`.

Nhìn lại hình dạng đó: một dây các nút, mỗi nút trỏ đúng một hướng tới
nút kế — đó CHÍNH XÁC là danh sách liên kết của cụm 3 (bài 13), chỉ đổi
tên trường `tiep` thành `phai`. Về mặt hình dạng, nó không còn là "cây"
theo tinh thần bài 25 — không nhánh nào rẽ ra cả.
::::

::::example{#don-hang-den-tang-dan}
Byte nhận năm mã đơn hàng, đến đúng theo thứ tự tăng dần (thực tế vẫn
hay xảy ra vậy), và chèn từng cái vào một BST theo đúng luật bài 27:

```python title=readonly
don_hang = [201, 202, 203, 204, 205]

goc = {"gia_tri": don_hang[0], "trai": None, "phai": None}
hien_tai = goc
for ma in don_hang[1:]:
    nut_moi = {"gia_tri": ma, "trai": None, "phai": None}
    hien_tai["phai"] = nut_moi          # mã sau LUÔN lớn hơn -> luôn rẽ phải
    hien_tai = nut_moi

print(goc["trai"])
print(goc["phai"]["gia_tri"])
print(goc["phai"]["phai"]["gia_tri"])
```

```text title=readonly
None
202
203
```

`goc["trai"]` là `None` — mãi mãi không ai từng đứng ở đó, vì không mã
nào tới sau lại nhỏ hơn 201 cả. Đi theo `["phai"]` liên tiếp,
`goc["phai"]["phai"]["gia_tri"]` cho 203 — đúng bước thứ ba trong dãy
tăng dần, y hệt cách bạn từng đi `nut["tiep"]["tiep"]` trên một danh
sách liên kết.
::::

::::predict{#do-sau-cay-lech commitOnce}
Byte chèn bốn số tăng dần khác vào một cây rỗng, rồi đếm xem đi được bao
nhiêu bước liên tiếp theo `"phai"` kể từ gốc:

```python
vals = [5, 10, 15, 20]
goc = {"gia_tri": vals[0], "trai": None, "phai": None}
hien_tai = goc
for v in vals[1:]:
    moi = {"gia_tri": v, "trai": None, "phai": None}
    hien_tai["phai"] = moi
    hien_tai = moi

do_sau = 0
con_tro = goc
while con_tro["phai"] is not None:
    con_tro = con_tro["phai"]
    do_sau += 1

print(do_sau)
```

**Trước khi chạy**, bạn đoán dòng in ra là bao nhiêu?

:::opt{correct}
3
:::

:::opt
4
::why
Gần đúng ở việc 4 là con số có thật trong bài — đúng độ dài của `vals`,
đúng tổng số nút trong cả cây (kể cả gốc). Bạn không đếm sai `vals`.

Chỗ lệch là `do_sau` không đếm TỔNG SỐ NÚT — nó đếm số LẦN bước
`["phai"]` được thực hiện, bắt đầu TỪ gốc. Gốc (5) không tự đếm chính
nó; vòng lặp chỉ cộng thêm mỗi khi thật sự RỜI một nút để sang nút kế.
Bốn nút, nhưng chỉ có BA cạnh nối chúng lại — ba bước, không phải bốn.
::
:::

:::opt
1
::why
Gần đúng ở việc bạn hiểu đúng NGUYÊN LÝ "mỗi bước là một lần rẽ
`phai`" — bạn không nhầm sang đếm nút.

Chỗ lệch là vòng lặp `while con_tro["phai"] is not None` không dừng
sau MỘT bước. Nó lặp tiếp CHỪNG NÀO nút hiện tại còn có `"phai"` — với
cây lệch hoàn toàn này, nút nào (trừ nút cuối) cũng có `"phai"`, nên
vòng lặp chạy hết cả ba cạnh trước khi con_tro["phai"] cuối cùng là
None.
::
:::

:::opt
0 — cây này không có nhánh lệch nào vì nó chỉ vừa mới được dựng
::why
Gần đúng ở cảm giác thận trọng: một cây "mới dựng" nghe có vẻ chưa kịp
lệch — phản xạ đó không vô lý ở bài học đầu tiên về khái niệm này.

Chỗ lệch là chính CÁCH dựng cây trong đoạn mã này — chèn dãy `vals` đã
tăng dần — LUÔN cho ra cây lệch hoàn toàn về một phía, không cần "thời
gian" hay thêm thao tác nào khác. Đây đúng là trọng tâm bài học: lệch
xảy ra ngay từ lúc DỰNG, không phải một hiện tượng xảy ra dần dần sau đó.
::
:::
::::

::::code{#tim-trong-cay-lech}
Năm mã đơn hàng tới tăng dần: 101, 102, 103, 104, 105. Bạn dựng cây theo
đúng luật BST (chỉ có thể rẽ phải, vì mỗi mã sau luôn lớn hơn), rồi dùng
LẠI đúng vòng lặp tìm kiếm của bài trước để tìm mã cuối cùng, 105 — và
so sánh số bước phải đi với tổng số nút trong cây.

```python title=starter
don_hang = [101, 102, 103, 104, 105]

goc = {"gia_tri": don_hang[0], "trai": None, "phai": None}
hien_tai = goc
for ma in don_hang[1:]:
    nut_moi = {"gia_tri": ma, "trai": None, "phai": None}
    ___                       # mã sau LUÔN lớn hơn mã trước -> gắn nut_moi vào nhánh nào của hien_tai?
    hien_tai = nut_moi

can_tim = 105
duong_di = []
con_tro = goc
while con_tro is not None:
    duong_di.append(con_tro["gia_tri"])
    if can_tim == con_tro["gia_tri"]:
        break
    elif can_tim < con_tro["gia_tri"]:
        con_tro = con_tro["trai"]
    else:
        con_tro = con_tro["phai"]

so_buoc = len(duong_di)
tong_so_nut = len(don_hang)

print(f"Đường đi tìm {can_tim}: {duong_di}")
print(f"Số bước phải đi: {so_buoc} / tổng số nút: {tong_so_nut}")
print(f"Đi qua HẾT mọi nút: {so_buoc == tong_so_nut}")
```

```python title=solution
don_hang = [101, 102, 103, 104, 105]

goc = {"gia_tri": don_hang[0], "trai": None, "phai": None}
hien_tai = goc
for ma in don_hang[1:]:
    nut_moi = {"gia_tri": ma, "trai": None, "phai": None}
    hien_tai["phai"] = nut_moi
    hien_tai = nut_moi

can_tim = 105
duong_di = []
con_tro = goc
while con_tro is not None:
    duong_di.append(con_tro["gia_tri"])
    if can_tim == con_tro["gia_tri"]:
        break
    elif can_tim < con_tro["gia_tri"]:
        con_tro = con_tro["trai"]
    else:
        con_tro = con_tro["phai"]

so_buoc = len(duong_di)
tong_so_nut = len(don_hang)

print(f"Đường đi tìm {can_tim}: {duong_di}")
print(f"Số bước phải đi: {so_buoc} / tổng số nút: {tong_so_nut}")
print(f"Đi qua HẾT mọi nút: {so_buoc == tong_so_nut}")
```

```python title=test
assert duong_di == [101, 102, 103, 104, 105], f"đường đi tìm 105 trên cây lệch phải ghé QUA HẾT năm nút theo thứ tự chèn — đang ra {duong_di}"
assert so_buoc == 5, f"so_buoc phải là 5 — đang ra {so_buoc}"
assert tong_so_nut == 5, "tong_so_nut phải là 5, tính từ len(don_hang)"
assert so_buoc == tong_so_nut, "trên cây lệch hoàn toàn, số bước tìm kiếm phải bằng đúng tổng số nút — không bỏ được nút nào"
```

:::hints
- kind: attention
  body: Chỗ trống là một CÂU LỆNH gán, không phải một biểu thức lồng trong chỗ khác — hãy gắn nut_moi vào đúng một trường của hien_tai.
- kind: strategy
  body: 'don_hang đã tăng dần, nên mọi mã tới sau đều LỚN HƠN nút hiện tại đang đứng cuối cây — theo luật bài 27, giá trị lớn hơn luôn rẽ phải. Gán hien_tai["phai"] = nut_moi.'
- kind: one-line
  body: 'Chỗ trống là `hien_tai["phai"] = nut_moi`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải gắn nut_moi vào TRƯỜNG PHAI của CHÍNH hien_tai hiện tại — không phải trường trai, và không phải luôn gắn vào goc cố định; bài này đang dạy nối tiếp vào ĐUÔI đang lớn dần của cây, không phải vào một điểm cố định
  requireAst:
  - kind: uses-name, target: hien_tai, min: 1
  # min: 1 — trong CHÍNH DÒNG chỗ trống, không tính các dòng khác. Một lời
  # giải hụt phổ biến là gắn nhầm vào goc thay vì hien_tai (goc["phai"] =
  # nut_moi mọi vòng lặp): điều đó khiến CÁC nút cũ bị ĐÈ mất, không tăng
  # min lên vì goc đã bị đọc ở dòng khởi tạo phía trên — nhưng "hien_tai"
  # thì hoàn toàn không xuất hiện trong chỗ trống đó, nên min:1 bắt được.
  # Đã thử: một lời giải hụt khác, gắn field sai (hien_tai["trai"] = ...),
  # vẫn đọc "hien_tai" nên qua được tầng static — nhưng lộ ra ngay ở test
  # (duong_di sẽ chỉ còn [101], không phải năm phần tử), đúng như luật 1
  # đòi hỏi: một tầng không đủ, cần nhiều tầng cùng bắt.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Đường đi tìm 105: \\[101, 102, 103, 104, 105\\]\\nSố bước phải đi: 5 / tổng số nút: 5\\nĐi qua HẾT mọi nút: True\\s*$"
:::
::::

::::byte{trigger=success mood=thinking pose=lean-in}
Cùng luật, cùng dict, nhưng "bỏ một nửa" của bài trước biến mất hoàn
toàn — mọi nút, không sót nút nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Tìm kiếm hôm nay chậm đi vì cây lệch hẳn về một phía. Nhưng có một việc
khác hoàn toàn không quan tâm cây cân đối hay lệch: không phải TÌM một
giá trị, mà LIỆT KÊ HẾT mọi giá trị trong cây — không bỏ sót ai, dù cây
hình dạng gì.

Nếu phải đi thăm HẾT mọi nút của một cây, bạn sẽ đi theo thứ tự nào?
Có phải chỉ có đúng MỘT cách hợp lý để làm việc đó không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
