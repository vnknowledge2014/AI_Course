---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.duyet-cay-ba-cach
title: "Duyệt cây: ba thứ tự, ba câu chuyện khác nhau"
summary: "Trái-gốc-phải cho một danh sách đã sắp xếp; gốc-trái-phải hay trái-phải-gốc kể hai câu chuyện khác hẳn về cùng một cây. Viết bằng vòng lặp + một ngăn xếp tự quản để phơi ra chính cái ngăn xếp cuộc gọi hàm vẫn quản lý ngầm."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 30
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.tree-traversal]
requires: [ds.bst, ds.stack, ctrl.while, core.list-append]
concepts: [ds.tree-traversal]
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
Không phải tìm MỘT giá trị nữa — hôm nay đi thăm HẾT mọi nút. Và hoá ra
có tới ba cách hợp lý để làm việc đó, không phải một.
::::

::::explain{#ba-thu-tu-mot-ngan-xep}
Đi thăm hết mọi nút của một cây gọi là **duyệt cây** (tree traversal).
Với danh sách liên kết, chỉ có một cách đi thăm hết — từ đầu tới cuối,
không có lựa chọn nào khác. Với cây nhị phân, ở mỗi nút bạn có BA việc
có thể làm: ghé chính nút đó, đi vào nhánh trái, đi vào nhánh phải — và
thứ tự làm ba việc này quyết định một trong BA câu chuyện khác nhau về
cùng một cây:

- **Trái-Gốc-Phải** (in-order): thăm hết nhánh trái trước, rồi tới
  chính nút, rồi mới sang nhánh phải. Trên một cây đúng luật BST (bài
  27), thứ tự này luôn cho ra một danh sách đã SẮP XẾP TĂNG DẦN — không
  phải tình cờ, mà là hệ quả trực tiếp của luật "trái nhỏ hơn, phải lớn
  hơn" áp dụng liên tục.
- **Gốc-Trái-Phải** (pre-order): báo cáo chính nút TRƯỚC, rồi mới xuống
  con. Giống hệt lệnh `tree` liệt kê thư mục — in tên thư mục cha trước,
  rồi mới liệt kê những gì bên trong nó.
- **Trái-Phải-Gốc** (post-order): xử lý xong CẢ HAI nhánh con rồi mới
  báo cáo chính nút. Giống việc tính dung lượng một thư mục — phải cộng
  hết dung lượng các thư mục con xong mới biết tổng của thư mục cha.

Bạn đã học một hàm tự gọi lại chính nó — và học rằng máy phải "nhớ
đường về" mỗi lần gọi thêm một lớp. Duyệt cây ở đây CỐ Ý không viết theo
lối đó. Thay vào đó, viết bằng `while` cộng một **ngăn xếp tự quản**:
một `list` Python dùng đúng như bài 7 đã dựng, `append` để đẩy vào,
`pop` để lấy ra. Danh sách đó đóng đúng vai trò mà "nhớ đường về" vẫn
làm NGẦM phía sau một hàm tự gọi — chỉ khác là ở đây bạn TỰ quản lý nó,
nhìn thấy tận mắt nó lớn lên và co lại từng bước một, thay vì để máy
giấu việc đó đi.
::::

::::example{#trai-goc-phai-cho-danh-sach-sap-xep}
Byte duyệt cây bảy số báo danh quen thuộc theo Trái-Gốc-Phải, bằng vòng
lặp cộng ngăn xếp:

```python title=readonly
goc = {"gia_tri": 50, "trai": None, "phai": None}
goc["trai"] = {"gia_tri": 30, "trai": None, "phai": None}
goc["phai"] = {"gia_tri": 70, "trai": None, "phai": None}
goc["trai"]["trai"] = {"gia_tri": 20, "trai": None, "phai": None}
goc["trai"]["phai"] = {"gia_tri": 40, "trai": None, "phai": None}
goc["phai"]["trai"] = {"gia_tri": 65, "trai": None, "phai": None}
goc["phai"]["phai"] = {"gia_tri": 90, "trai": None, "phai": None}

ngan_xep = []
con_tro = goc
ket_qua = []

while con_tro is not None or ngan_xep:
    while con_tro is not None:            # đi hết về bên trái, ghi nhớ đường đã qua
        ngan_xep.append(con_tro)
        con_tro = con_tro["trai"]
    con_tro = ngan_xep.pop()              # hết trái -> lùi lại nút gần nhất chưa ghé
    ket_qua.append(con_tro["gia_tri"])
    con_tro = con_tro["phai"]             # rồi mới thử nhánh phải của nút đó

print(ket_qua)
```

```text title=readonly
[20, 30, 40, 50, 65, 70, 90]
```

Kết quả là một dãy TĂNG DẦN — đúng bảy số báo danh, sắp đúng thứ tự,
không cần gọi `sorted()` nào cả. Đây chính là hệ quả sống của luật BST:
đi Trái-Gốc-Phải trên một cây đúng luật lúc nào cũng cho ra danh sách đã
sắp xếp.

Nhìn kỹ `ngan_xep`: nó phình to nhất khi vòng lặp trong đi hết về bên
trái (dồn 50, 30, 20 vào ngăn xếp trước khi ghé nút đầu tiên, 20), rồi
co dần lại khi `pop()` lần lượt trả các nút đó về theo đúng thứ tự
ngược — chính là "máy nhớ đường về" mà một lời gọi đệ quy vẫn làm ngầm,
giờ bạn nhìn thấy nó bằng mắt trong biến `ngan_xep`.
::::

::::predict{#nhan-dien-thu-tu commitOnce}
Một cây rất nhỏ, ba nút — gốc "B", trái "A", phải "C":

```python
goc = {
    "gia_tri": "B",
    "trai": {"gia_tri": "A", "trai": None, "phai": None},
    "phai": {"gia_tri": "C", "trai": None, "phai": None},
}
```

Một bạn đồng nghiệp duyệt cây này và in ra đúng dãy `['B', 'A', 'C']`.

**Trước khi đọc tiếp**, bạn đoán: bạn đồng nghiệp đó dùng thứ tự nào
trong ba thứ tự vừa học?

:::opt{correct}
Gốc-Trái-Phải (pre-order) — ghé "B" (gốc) trước, rồi mới xuống "A"
(trái), rồi mới sang "C" (phải)
:::

:::opt
Trái-Gốc-Phải (in-order) — vì đây là thứ tự "chuẩn" nhất, hay dùng nhất
::why
Gần đúng ở việc bạn nhớ đúng TÊN của thứ tự Trái-Gốc-Phải, và đúng là
nó xuất hiện nhiều nhất trong ví dụ (nhờ gắn liền với BST).

Chỗ lệch là dãy `['B', 'A', 'C']` không khớp thứ tự này. Trái-Gốc-Phải
phải ghé "A" (nhánh trái) TRƯỚC "B" (gốc) — dãy đúng của thứ tự này
phải là `['A', 'B', 'C']`, "A" đứng đầu, không phải "B".
::
:::

:::opt
Trái-Phải-Gốc (post-order) — vì "C" xuất hiện ở giữa, giống như đã xử
lý xong nhánh rồi mới report
::why
Gần đúng ở việc bạn để ý đúng: Trái-Phải-Gốc luôn ghé GỐC SAU CÙNG,
không phải đầu tiên — quan sát đó không sai.

Chỗ lệch chính vì lý do đó: dãy `['B', 'A', 'C']` lại có "B" (gốc) đứng
ĐẦU TIÊN, không phải cuối cùng — nên không thể là Trái-Phải-Gốc. Dãy
đúng của thứ tự này phải là `['A', 'C', 'B']`.
::
:::

:::opt
Không thể xác định — với cây chỉ ba nút, nhiều thứ tự có thể trùng nhau
::why
Gần đúng ở sự thận trọng — đúng là với MỘT SỐ cây rất đặc biệt (ví dụ
chỉ có một nút), vài thứ tự trùng kết quả nhau thật.

Chỗ lệch là cây này KHÔNG thuộc trường hợp đặc biệt đó. Với ba giá trị
phân biệt A, B, C và đúng hình dạng đã cho, ba thứ tự cho ra BA dãy khác
nhau hoàn toàn: `['A','B','C']`, `['B','A','C']`, `['A','C','B']` —
không dãy nào trùng dãy nào, nên `['B', 'A', 'C']` xác định được chính
xác một thứ tự duy nhất.
::
:::
::::

::::code{#duyet-goc-trai-phai}
Cây bảy số báo danh quen thuộc (readonly). Bạn viết vòng lặp duyệt
Gốc-Trái-Phải bằng một ngăn xếp — dùng đúng `.append(...)` để đẩy vào,
như bài 7 đã dựng. Mẹo: đẩy nhánh PHẢI vào ngăn xếp trước, nhánh TRÁI
sau — vì ngăn xếp lấy ra theo kiểu vào sau ra trước, đẩy sau cùng nghĩa
là lấy ra trước.

```python title=starter
goc = {"gia_tri": 50, "trai": None, "phai": None}
goc["trai"] = {"gia_tri": 30, "trai": None, "phai": None}
goc["phai"] = {"gia_tri": 70, "trai": None, "phai": None}
goc["trai"]["trai"] = {"gia_tri": 20, "trai": None, "phai": None}
goc["trai"]["phai"] = {"gia_tri": 40, "trai": None, "phai": None}
goc["phai"]["trai"] = {"gia_tri": 65, "trai": None, "phai": None}
goc["phai"]["phai"] = {"gia_tri": 90, "trai": None, "phai": None}

ngan_xep = [goc]
thu_tu_goc_truoc = []

while ngan_xep:
    nut = ngan_xep.pop()
    thu_tu_goc_truoc.append(nut["gia_tri"])
    if nut["phai"] is not None:
        ___                            # đẩy nhánh phải vào ngăn xếp trước
    if nut["trai"] is not None:
        ___                            # rồi đẩy nhánh trái vào SAU (để nó được pop ra TRƯỚC)

print(thu_tu_goc_truoc)
```

```python title=solution
goc = {"gia_tri": 50, "trai": None, "phai": None}
goc["trai"] = {"gia_tri": 30, "trai": None, "phai": None}
goc["phai"] = {"gia_tri": 70, "trai": None, "phai": None}
goc["trai"]["trai"] = {"gia_tri": 20, "trai": None, "phai": None}
goc["trai"]["phai"] = {"gia_tri": 40, "trai": None, "phai": None}
goc["phai"]["trai"] = {"gia_tri": 65, "trai": None, "phai": None}
goc["phai"]["phai"] = {"gia_tri": 90, "trai": None, "phai": None}

ngan_xep = [goc]
thu_tu_goc_truoc = []

while ngan_xep:
    nut = ngan_xep.pop()
    thu_tu_goc_truoc.append(nut["gia_tri"])
    if nut["phai"] is not None:
        ngan_xep.append(nut["phai"])
    if nut["trai"] is not None:
        ngan_xep.append(nut["trai"])

print(thu_tu_goc_truoc)
```

```python title=test
assert thu_tu_goc_truoc == [50, 30, 20, 40, 70, 65, 90], f"thứ tự Gốc-Trái-Phải trên cây này phải là [50, 30, 20, 40, 70, 65, 90] — đang ra {thu_tu_goc_truoc}"
assert thu_tu_goc_truoc[0] == 50, "phần tử ĐẦU TIÊN phải là gốc (50) — đúng tinh thần Gốc-Trái-Phải, ghé gốc trước tiên"
assert len(thu_tu_goc_truoc) == 7, "phải ghé đủ cả bảy nút, không bỏ sót và không lặp lại"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều là lệnh ĐẨY VÀO ngăn xếp — dùng .append(...), đúng công cụ bài 7 đã dựng cho ngăn xếp, không dùng .insert(0, ...) (đó là thao tác của HÀNG ĐỢI, không phải ngăn xếp).
- kind: strategy
  body: 'Ngăn xếp lấy ra theo kiểu vào SAU ra TRƯỚC (LIFO). Muốn nhánh TRÁI được xử lý trước nhánh phải, nhánh trái phải được đẩy vào SAU CÙNG — nên đẩy nut["phai"] trước bằng ngan_xep.append(nut["phai"]), rồi mới đẩy nut["trai"] bằng ngan_xep.append(nut["trai"]).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `ngan_xep.append(nut["phai"])` và `ngan_xep.append(nut["trai"])`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải đẩy vào ngăn xếp bằng .append(...) — đúng công cụ bài 7 đã dựng cho ngăn xếp; dùng .insert(0, ...) biến ngan_xep thành một hàng đợi, không còn là ngăn xếp nữa và sẽ cho SAI thứ tự
  requireAst:
  - kind: uses-call, target: append, min: 3
  # min: 3 — đếm thật trên solution: .append xuất hiện ba lần — một lần đã
  # có sẵn trong khung (thu_tu_goc_truoc.append(nut["gia_tri"])), cộng hai
  # lần ở đúng hai chỗ trống. Một lời giải hụt dùng .insert(0, ...) thay vì
  # .append cho một hoặc cả hai chỗ trống sẽ tụt xuống dưới 3 — VÀ (đã thử
  # trên chính cây bảy nút này) còn cho ra thứ tự SAI hẳn, khác cả về giá
  # trị lẫn độ dài đúng, nên output/tests cũng bắt được độc lập.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\[50, 30, 20, 40, 70, 65, 90\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một cây, ba câu chuyện khác nhau — và cả ba đều đi bằng một ngăn
xếp bạn tự tay quản lý, không giấu gì phía sau.
::::

::::reflect{#nghi-lai}
Một câu hỏi khép lại cả cụm bài về cây.

Ba cách duyệt hôm nay đều ghé HẾT mọi nút, chỉ khác THỨ TỰ — và thứ tự
đó luôn gắn chặt với HÌNH DẠNG cây (trái trước hay gốc trước). Nhưng nếu
bạn không cần thăm hết cây, chỉ cần liên tục LẤY RA phần tử "quan trọng
nhất" đang có — không theo hình dạng cây, không theo trái/phải/gốc, và
cũng không theo thứ tự đã VÀO — hàng đợi bạn dựng ở bài 9 (vào trước ra
trước) còn dùng được không?

Ở một quầy cấp cứu, người tới trước không phải lúc nào cũng được khám
trước — ai nặng hơn ra trước. Bài sau hỏi đúng câu đó.
::::

::::checkpoint{mastery=0.8}
::::
