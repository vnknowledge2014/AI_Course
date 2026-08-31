---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.cay-tim-kiem-nhi-phan
title: "Cây tìm kiếm nhị phân: trái nhỏ hơn, phải lớn hơn"
summary: "Thêm một luật lên hình dạng cây nhị phân: tại MỌI nút, toàn bộ nhánh trái đều nhỏ hơn nút, toàn bộ nhánh phải đều lớn hơn. Luật này biến cây từ chỗ chứa dữ liệu thành một cái máy tìm kiếm."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.bst]
requires: [ds.binary-tree, core.dict, ctrl.comparison, core.boolean, core.fstring]
concepts: [ds.bst]
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
Hôm qua cây nhị phân chỉ có hình dạng, không luật. Hôm nay thêm đúng một
luật — và cây biến thành một cái máy tìm kiếm.
::::

::::explain{#luat-trai-nho-phai-lon}
Cây nhị phân bài 26 (`{"gia_tri": ..., "trai": None, "phai": None}`) chỉ
khoá HÌNH DẠNG: tối đa hai nhánh. Giá trị nào đứng ở nhánh nào là tuỳ ý
— đổi `trai` và `phai` cho nhau, cây vẫn hợp lệ như cũ.

**Cây tìm kiếm nhị phân** (binary search tree, gọi tắt BST) thêm đúng
một luật lên trên hình dạng đó:

> **Tại MỌI nút: toàn bộ giá trị trong nhánh trái đều NHỎ HƠN nút; toàn
> bộ giá trị trong nhánh phải đều LỚN HƠN nút.**

Chữ "MỌI" ở đây quan trọng, và dễ hiểu lầm nhất. Luật không chỉ nói về
CON TRỰC TIẾP — nó nói về CẢ NHÁNH, tức mọi hậu duệ, dù nằm sâu bao
nhiêu tầng. Một nút X nằm ở nhánh trái của nút Y — dù X là con, cháu,
hay chắt của Y — thì giá trị của X vẫn phải nhỏ hơn giá trị của Y. Không
đủ nếu chỉ con TRỰC TIẾP tuân luật mà cháu thì không; bài "predict" dưới
đây cho bạn thấy đúng chỗ hiểu lầm này.

Giả sử không có hai giá trị nào trùng nhau trong cây (bài này không bàn
tới trường hợp trùng số).
::::

::::example{#dung-luat-cho-so-bao-danh}
Byte xếp năm số báo danh vào một cây tìm kiếm nhị phân, gốc là 50:

```python title=readonly
n30 = {"gia_tri": 30, "trai": None, "phai": None}
n70 = {"gia_tri": 70, "trai": None, "phai": None}
n20 = {"gia_tri": 20, "trai": None, "phai": None}
n40 = {"gia_tri": 40, "trai": None, "phai": None}

n30["trai"] = n20
n30["phai"] = n40

goc = {"gia_tri": 50, "trai": n30, "phai": n70}

print(goc["gia_tri"], "-> trái:", goc["trai"]["gia_tri"], "| phải:", goc["phai"]["gia_tri"])
print("nhánh trái của goc, tại nút 30 -> trái:", n30["trai"]["gia_tri"], "| phải:", n30["phai"]["gia_tri"])
```

```text title=readonly
50 -> trái: 30 | phải: 70
nhánh trái của goc, tại nút 30 -> trái: 20 | phải: 40
```

Kiểm luật ở TỪNG nút một: tại `goc` (50), trái là 30 (nhỏ hơn 50 ✓),
phải là 70 (lớn hơn 50 ✓). Tại `n30` (30) — chính nó lại là gốc của một
cây con — trái là 20 (nhỏ hơn 30 ✓), phải là 40 (lớn hơn 30 ✓). Cả 20 và
40 cũng đều nhỏ hơn 50, vì chúng nằm trong nhánh trái của `goc` — luật
áp đúng xuyên suốt, không chỉ ở tầng đầu tiên.

Nếu ai đó đặt 60 vào vị trí của 40 (`n30["phai"] = {"gia_tri": 60, ...}`),
hình dạng cây không đổi một chữ, nhưng luật vỡ ngay: 60 nằm trong nhánh
TRÁI của `goc` (vì là hậu duệ của `n30`, con trái của `goc`), mà 60 lại
LỚN HƠN 50. Sai luật không nằm ở chỗ so với cha trực tiếp (60 > 30, đúng
với `n30`) — nó nằm ở chỗ so với TOÀN BỘ tổ tiên, kể cả `goc`.
::::

::::predict{#cay-co-hop-le commitOnce}
Byte dựng một cây khác, cố tình đặt một số hơi khác thường:

```python
n60 = {"gia_tri": 60, "trai": None, "phai": None}
n30 = {"gia_tri": 30, "trai": None, "phai": n60}
n70 = {"gia_tri": 70, "trai": None, "phai": None}
goc = {"gia_tri": 50, "trai": n30, "phai": n70}

ca_nhanh_trai_nho_hon_goc = (
    goc["trai"]["gia_tri"] < goc["gia_tri"]
    and goc["trai"]["phai"]["gia_tri"] < goc["gia_tri"]
)
print(ca_nhanh_trai_nho_hon_goc)
```

**Trước khi chạy**, bạn đoán dòng in ra `True` hay `False`?

:::opt{correct}
`False`
:::

:::opt
`True`
::why
Gần đúng ở vế đầu: `goc["trai"]["gia_tri"] < goc["gia_tri"]` tính ra
`30 < 50`, đúng là `True` — con trực tiếp của gốc, `n30`, thật sự nhỏ
hơn gốc.

Chỗ lệch nằm ở vế sau, nối bằng `and`. `goc["trai"]["phai"]["gia_tri"]`
đi tiếp một bước từ `n30` sang `n60` — và `60 < 50` là `False`. `and`
chỉ trả `True` khi CẢ HAI vế đều `True`; một vế `False` khiến toàn bộ
biểu thức thành `False`, dù vế đầu có đúng cỡ nào.
::
:::

:::opt
Chương trình dừng lại và báo lỗi, vì `goc["trai"]["phai"]` là `None`
::why
Gần đúng ở việc bạn cẩn thận nghĩ tới `None` — phản xạ đúng khi đọc cây.

Chỗ lệch là ở ĐÂY, `goc["trai"]["phai"]` không hề là `None`. Dòng
`n30 = {"gia_tri": 30, "trai": None, "phai": n60}` đã gán hẳn `n60` vào
đó — một nút thật, có khoá `"gia_tri"` hẳn hoi, không gây lỗi gì khi
đọc `["gia_tri"]` từ nó.
::
:::

:::opt
`60`
::why
Gần đúng ở việc `60` đúng là con số nằm trong biểu thức so sánh — bạn
đọc đúng dữ liệu trong cây.

Chỗ lệch là dòng in không in ra một con số lấy thẳng từ cây. Nó in kết
quả của một phép so sánh nối bằng `and` — biểu thức dạng này luôn ra
`True` hoặc `False`, không bao giờ ra một con số như `60`.
::
:::
::::

::::code{#xep-vao-cay-dung-luat}
Byte có sẵn một gốc (50) và ba nút rời: 30, 70, 65. Xếp cả ba vào đúng
vị trí sao cho cây tuân đúng luật BST — dùng chính ba biến `nut_30`,
`nut_70`, `nut_65` đã có, đừng dựng dict mới.

```python title=starter
goc = {"gia_tri": 50, "trai": None, "phai": None}

nut_30 = {"gia_tri": 30, "trai": None, "phai": None}
nut_70 = {"gia_tri": 70, "trai": None, "phai": None}
nut_65 = {"gia_tri": 65, "trai": None, "phai": None}

goc["trai"] = ___                  # 30 nhỏ hơn 50 -> nhánh nào?

goc["phai"] = ___                  # 70 lớn hơn 50 -> nhánh nào?

# 65 so với gốc (50): lớn hơn, nên trước tiên rẽ vào nhánh phải của gốc.
# Rồi so với 70 (nút đang đứng ở nhánh phải): 65 < 70, nên từ đó rẽ trái.
goc["phai"]["trai"] = ___

hop_le = (
    goc["trai"]["gia_tri"] < goc["gia_tri"]
    and goc["phai"]["gia_tri"] > goc["gia_tri"]
    and goc["phai"]["trai"]["gia_tri"] < goc["phai"]["gia_tri"]
)

print(f"Gốc: {goc['gia_tri']}")
print(f"Trái gốc: {goc['trai']['gia_tri']}, Phải gốc: {goc['phai']['gia_tri']}")
print(f"Trái của nhánh phải: {goc['phai']['trai']['gia_tri']}")
print(f"Cây đúng luật BST: {hop_le}")
```

```python title=solution
goc = {"gia_tri": 50, "trai": None, "phai": None}

nut_30 = {"gia_tri": 30, "trai": None, "phai": None}
nut_70 = {"gia_tri": 70, "trai": None, "phai": None}
nut_65 = {"gia_tri": 65, "trai": None, "phai": None}

goc["trai"] = nut_30

goc["phai"] = nut_70

goc["phai"]["trai"] = nut_65

hop_le = (
    goc["trai"]["gia_tri"] < goc["gia_tri"]
    and goc["phai"]["gia_tri"] > goc["gia_tri"]
    and goc["phai"]["trai"]["gia_tri"] < goc["phai"]["gia_tri"]
)

print(f"Gốc: {goc['gia_tri']}")
print(f"Trái gốc: {goc['trai']['gia_tri']}, Phải gốc: {goc['phai']['gia_tri']}")
print(f"Trái của nhánh phải: {goc['phai']['trai']['gia_tri']}")
print(f"Cây đúng luật BST: {hop_le}")
```

```python title=test
assert goc["trai"] is nut_30, "goc['trai'] phải là nut_30 — 30 nhỏ hơn 50 nên đứng bên trái"
assert goc["phai"] is nut_70, "goc['phai'] phải là nut_70 — 70 lớn hơn 50 nên đứng bên phải"
assert goc["phai"]["trai"] is nut_65, "nhánh trái của goc['phai'] phải là nut_65 — 65 nhỏ hơn 70 nên rẽ trái từ đó"
assert hop_le is True, "cây dựng ra phải đúng luật BST ở MỌI nút đã kiểm — nếu hop_le là False thì có ít nhất một nút đặt sai vị trí"
```

:::hints
- kind: attention
  body: Đề bài đã nói rõ dùng ba biến nut_30, nut_70, nut_65 có sẵn — mỗi chỗ trống chỉ cần điền ĐÚNG TÊN biến, không cần gõ lại dict.
- kind: strategy
  body: "So từng số với nút cha nó sắp gắn vào: 30 < 50 nên nằm ở trai của goc; 70 > 50 nên nằm ở phai của goc; 65 so với 50 thì lớn hơn (đi phai trước), nhưng so với 70 (nút đang đứng ở phai) thì nhỏ hơn (đi trai tiếp từ đó) — nên 65 nằm ở goc['phai']['trai']."
- kind: one-line
  body: "Ba chỗ trống lần lượt là nut_30, nut_70, nut_65."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải dùng ĐÚNG một trong ba biến nut_30, nut_70, nut_65 đã có sẵn — đề bài yêu cầu tái dùng chúng, không dựng dict {"gia_tri": ...} mới cho từng chỗ trống
  requireAst:
  - kind: uses-name, target: nut_30, min: 1
  - kind: uses-name, target: nut_70, min: 1
  - kind: uses-name, target: nut_65, min: 1
  # Mỗi tên chỉ cần min:1 vì đó là số lần TỐI THIỂU một lời giải hợp lý
  # phải đọc từng biến — mỗi biến chỉ dùng đúng một lần trong cả ba chỗ
  # trống. Không đặt min cao hơn: làm vậy sẽ phạt một lời giải ĐÚNG chỉ vì
  # nó không lặp lại một cái tên nhiều lần một cách không cần thiết.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Gốc: 50\\nTrái gốc: 30, Phải gốc: 70\\nTrái của nhánh phải: 65\\nCây đúng luật BST: True\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba số, đúng ba chỗ — không phải theo cảm tính, mà theo đúng một luật áp
dụng ở mọi nút.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Luật BST hôm nay nói: nhánh trái luôn nhỏ hơn, nhánh phải luôn lớn hơn.
Nhưng luật đó tự nó không đi TÌM một số nào cả — bạn vẫn phải tự SO
SÁNH và TỰ RẼ NHÁNH bằng tay, đúng như bài này vừa làm.

Nếu có một số báo danh cụ thể cần tìm trong một cây to hơn nhiều — hàng
trăm nút — bạn sẽ không đi bằng tay được nữa. Luật "trái nhỏ, phải lớn"
có giúp bạn viết một quy trình TỰ ĐỘNG rẽ đúng nhánh ở mỗi bước không,
mà không cần nhìn tận mắt từng nút một?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
