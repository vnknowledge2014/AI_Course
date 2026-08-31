---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.dong-la-cay-nhi-phan-dac-biet
title: "Đống: cây nhị phân luôn giữ đúng một luật"
summary: "Đống là một cây nhị phân giữ đúng MỘT luật — mỗi nút lớn hơn hoặc bằng cả hai con của nó, không quan tâm trái hay phải — lỏng hơn luật BST nên dựng nhanh hơn."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 32
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.heap]
requires: [ds.binary-tree, ds.priority-queue, core.dict, core.default-parameter, ctrl.while, core.list-append, core.fstring]
concepts: [ds.heap]
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
Quét cả hàng chờ mỗi lần muốn biết ai khẩn cấp nhất — bài trước để lại
đúng câu hỏi đó. Hôm nay là một cái cây trả lời nó.
::::

::::explain{#luat-long-hon-bst}
Bài 27 dựng **cây tìm kiếm nhị phân** (BST) bằng một luật khá chặt: tại
MỌI nút, cả nhánh trái phải nhỏ hơn nút, cả nhánh phải phải lớn hơn. Luật
đó mạnh — nó biến cây thành một cái máy tìm kiếm (bài 28) — nhưng cũng dễ
vỡ: bài 29 cho thấy chỉ cần chèn dữ liệu đã sắp xếp sẵn là cây lệch hẳn
sang một bên, mất sạch điểm mạnh.

Một **đống** (heap) là cây nhị phân (bài 26) nhưng giữ một luật LỎNG HƠN
nhiều — đúng một điều, không hơn:

> **Mỗi nút phải lớn hơn hoặc bằng cả hai con của nó** (đống lớn nhất ở
> gốc — max-heap), **hoặc** mỗi nút phải nhỏ hơn hoặc bằng cả hai con
> (min-heap). Track này dùng max-heap.

Chú ý điều đống KHÔNG đòi: nó không quan tâm con trái nhỏ hơn hay con
phải nhỏ hơn — chỉ cần cả hai con đều KHÔNG lớn hơn cha là đủ. So với BST
(phải phân biệt rạch ròi bên trái, bên phải), đống lỏng tay hơn hẳn. Và
chính vì lỏng hơn, dựng một đống nhanh hơn dựng một BST cân đối — ít ràng
buộc phải giữ hơn mỗi khi thêm một giá trị mới.

Cái giá trị lớn nhất trong đống LUÔN nằm ở gốc — hệ quả trực tiếp của
luật trên, vì gốc phải lớn hơn hoặc bằng cả hai nhánh con, mà hai nhánh
con đó lại lớn hơn hoặc bằng CÁC nút bên dưới chúng. Đây chính xác là thứ
bài trước cần: hàng đợi ưu tiên (bài 31) đòi hỏi tìm giá trị lớn nhất
NGAY LẬP TỨC, không dò cả hàng — mà một đống thì luôn có sẵn giá trị lớn
nhất ở đúng một chỗ cố định: gốc.
::::

::::example{#kiem-tra-mot-dong}
Một nút cây (bài 25) vẫn là `dict` với `trai`/`phai` như track đã dùng từ
đầu. Dựng một cây ứng viên và kiểm luật đống bằng vòng lặp cùng một ngăn
xếp tự quản (cụm 2 và bài 30) — không đệ quy:

```python title=readonly
def nut(gia_tri, trai=None, phai=None):
    return {"gia_tri": gia_tri, "trai": trai, "phai": phai}

goc = nut(9,
    nut(7, nut(3), nut(6)),
    nut(8, nut(5), nut(2)),
)

vi_pham = []
ngan_xep = [goc]
while ngan_xep:
    hien_tai = ngan_xep.pop()
    for con in (hien_tai["trai"], hien_tai["phai"]):
        if con is not None:
            if not (hien_tai["gia_tri"] >= con["gia_tri"]):
                vi_pham.append((hien_tai["gia_tri"], con["gia_tri"]))
            ngan_xep.append(con)

print(f"Đúng luật đống: {len(vi_pham) == 0}")
```

```text title=readonly
Đúng luật đống: True
```

Đẩy gốc vào ngăn xếp, lấy ra, kiểm nó với cả hai con — `9 >= 7` và
`9 >= 8`, cả hai đúng. Rồi đẩy tiếp hai con đó vào ngăn xếp để kiểm tiếp
xuống dưới, y hệt cách bài 30 duyệt cây: dùng một `list` làm ngăn xếp tự
quản, không gọi hàm đệ quy nào. `7 >= 3`, `7 >= 6`, `8 >= 5`, `8 >= 2` —
mọi cặp cha con đều đúng luật, không có cặp nào lệch. Chú ý: `3` và `6`
KHÔNG cần so với nhau — đống không quan tâm anh em cùng cha ai lớn ai
nhỏ, chỉ quan tâm cha với từng con.
::::

::::predict{#doan-dong-vi-pham commitOnce}
Byte dựng một cây khác:

```python title=readonly
goc = nut(10,
    nut(8, nut(4), nut(9)),
    nut(6),
)
```

Vẽ ra: gốc `10` có con trái `8` và con phải `6`; `8` lại có con trái `4`
và con phải `9`; `6` không có con nào.

**Trước khi chạy**, bạn đoán: cây này có đúng luật đống không? Nếu không,
cặp cha-con nào vi phạm?

:::opt{correct}
Không đúng — cặp `8` và con phải `9` của nó vi phạm, vì `9 > 8`: con lớn
hơn cha.
:::

:::opt
Đúng — vì con trái luôn nhỏ hơn con phải ở mọi nút có đủ hai con (`4 < 9`
ở nút `8`), đúng thứ tự tăng dần từ trái sang phải.
::why
Gần đúng ở phép so `4 < 9` — hai con số đó đúng là `4` nhỏ hơn `9`, không
tính sai.

Chỗ lệch là luật bạn vừa dùng — "trái nhỏ hơn phải" — là luật của CÂY TÌM
KIẾM NHỊ PHÂN (bài 27), không phải luật đống. Đống không quan tâm trái
phải bên nào lớn hơn bên nào; nó chỉ đòi CHA lớn hơn hoặc bằng CẢ HAI con.
Ở đây cha là `8`, con phải là `9` — `8 >= 9` sai, bất kể `4` và `9` so với
nhau ra sao.
::
:::

:::opt
Không đúng — cặp gốc `10` và con trái `8` vi phạm, vì `8` không phải số
lớn thứ nhì đúng vị trí trong cây.
::why
Gần đúng ở việc bạn kết luận đúng: cây này KHÔNG đúng luật đống — kết luận
"không" không sai.

Chỗ lệch là cặp bạn chỉ ra lại không hề vi phạm gì: `10 >= 8` là đúng, cha
lớn hơn con, hoàn toàn hợp luật. Chỗ vi phạm thật nằm sâu hơn một tầng —
ở nút `8` và con phải `9` của nó, không phải ở gốc.
::
:::

:::opt
Đúng — vì mọi giá trị đều xuất hiện đúng một lần, không lặp, và cây có
hình dạng cân đối, đủ hai tầng con.
::why
Gần đúng ở việc bạn quan sát đúng hình dạng — cây này thật sự không lặp
giá trị nào, và cách xếp các nút cũng không có gì kỳ quặc về mặt hình
dạng.

Chỗ lệch là luật đống không nói gì về việc giá trị có lặp hay không, cũng
không chỉ xét hình dạng suông. Luật đống là một phép SO SÁNH giữa từng
cha và từng con của nó — và ở đây, con `9` của nút `8` lớn hơn cha, nên
luật bị phá dù hình dạng cây trông vẫn "gọn gàng".
::
:::
::::

::::code{#kiem-tra-dong-that}
Cây dưới đây có một chỗ hai cha con BẰNG NHAU — đống vẫn cho phép điều đó
(`>=`, không phải `>`). Điền hai chỗ trống để hoàn thành phép kiểm.

```python title=starter
def nut(gia_tri, trai=None, phai=None):
    return {"gia_tri": gia_tri, "trai": trai, "phai": phai}

goc = nut(9,
    nut(7, nut(3), nut(6)),
    nut(8, nut(8), nut(2)),
)

vi_pham = []
ngan_xep = [goc]
while ngan_xep:
    hien_tai = ngan_xep.pop()
    for con in (hien_tai["trai"], hien_tai["phai"]):
        if con is not None:
            if not (___):
                vi_pham.append((hien_tai["gia_tri"], con["gia_tri"]))
            ngan_xep.append(___)

la_dong_hop_le = len(vi_pham) == 0
print(f"Đúng luật đống: {la_dong_hop_le}")
print(f"Vi phạm: {vi_pham}")
```

```python title=solution
def nut(gia_tri, trai=None, phai=None):
    return {"gia_tri": gia_tri, "trai": trai, "phai": phai}

goc = nut(9,
    nut(7, nut(3), nut(6)),
    nut(8, nut(8), nut(2)),
)

vi_pham = []
ngan_xep = [goc]
while ngan_xep:
    hien_tai = ngan_xep.pop()
    for con in (hien_tai["trai"], hien_tai["phai"]):
        if con is not None:
            if not (hien_tai["gia_tri"] >= con["gia_tri"]):
                vi_pham.append((hien_tai["gia_tri"], con["gia_tri"]))
            ngan_xep.append(con)

la_dong_hop_le = len(vi_pham) == 0
print(f"Đúng luật đống: {la_dong_hop_le}")
print(f"Vi phạm: {vi_pham}")
```

```python title=test
assert la_dong_hop_le is True, f"cây này ĐÚNG luật đống (kể cả cặp bằng nhau 8-8) — đang báo {la_dong_hop_le}"
assert vi_pham == [], f"không cặp cha-con nào trong cây này vi phạm luật đống — đang báo vi phạm ở {vi_pham}"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất là một phép so sánh, chỗ trống thứ hai là một cái tên có sẵn trong vòng lặp. Đừng lẫn hai việc — một chỗ KIỂM luật, một chỗ ĐI TIẾP xuống cây.
- kind: strategy
  body: 'Chỗ trống thứ nhất phải diễn đúng câu "cha lớn hơn hoặc bằng con", dùng đúng hai cái tên `hien_tai` và `con` đã có sẵn trong vòng lặp — chú ý dấu `>=`, không phải `>`, vì đống CHO PHÉP bằng nhau. Chỗ trống thứ hai: muốn kiểm tiếp xuống dưới, phải đẩy đúng NÚT vừa xét (`con`) vào ngăn xếp — không phải giá trị của nó, không phải nút hiện tại.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `hien_tai["gia_tri"] >= con["gia_tri"]` và `con`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống thứ nhất phải thật sự so sánh gia_tri của hien_tai với gia_tri của con (không gõ thẳng True/False), và cả hai cái tên hien_tai lẫn con phải xuất hiện đủ trong toàn đoạn mã — đếm được đúng 4 lần đọc tên con và đúng 4 lần đọc tên hien_tai nếu làm đúng lối bài dạy
  requireAst:
  - kind: uses-name, target: con, min: 4
  # Thiếu dòng dưới thì đổi MỌI chỗ đọc hien_tai ở chỗ trống 1 thành con
  # (cho `con["gia_tri"] >= con["gia_tri"]`, tự-so-với-chính-mình, luôn
  # True) vẫn qua: số lần đọc `con` chỉ TĂNG (4→5), không hề giảm dưới
  # min:4 ở trên. hien_tai đọc đúng 4 lần trong lời giải thật — hai lần ở
  # dòng `for con in (hien_tai["trai"], hien_tai["phai"]):` đã có sẵn
  # trong khung, một lần ở chỗ trống 1, một lần ở dòng `vi_pham.append`
  # kề dưới cũng có sẵn. Mất lần đọc ở chỗ trống thì tụt xuống 3, dưới
  # min:4.
  - kind: uses-name, target: hien_tai, min: 4
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Đúng luật đống: True\\nVi phạm: \\[\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cả bốn cặp cha-con đều đúng luật — kể cả cặp bằng nhau. Đống không đòi
hỏi hơn thế.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa kiểm luật đống bằng đúng những trường `trai`/`phai` — kiểu con
trỏ nút cây mà track này dùng từ cụm 5. Nhưng đống có thêm một luật HÌNH
DẠNG chặt hơn cây thường rất nhiều: nó luôn lấp đầy từng tầng từ trái
sang phải, không để trống một chỗ nào ở giữa — không giống cây bài 29,
vốn có thể lệch hẳn về một phía.

Luật hình dạng chặt đó có giúp bỏ hẳn được các trường `trai`/`phai`,
đổi bằng một cách cất giữ khác gọn hơn không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
