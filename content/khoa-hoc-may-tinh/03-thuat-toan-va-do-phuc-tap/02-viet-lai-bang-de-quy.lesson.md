---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.viet-lai-bang-de-quy
title: "Viết lại đúng việc cũ bằng đệ quy, so với bản vòng lặp"
summary: "Cây bảy số báo danh, cùng một cách duyệt — bản cũ dùng while + ngăn xếp tự quản (list bạn tự đẩy/lấy), bản mới dùng đệ quy. Đặt cạnh nhau lộ ra: đệ quy chỉ là để trình thông dịch tự làm hộ việc đẩy/lấy đó, và không cần mẹo đảo thứ tự đẩy như bản ngăn xếp phải dùng."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [alg.recursion-vs-loop]
requires: [alg.recursion-is-stack, ds.tree-traversal]
concepts: [alg.recursion-vs-loop]
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
Cùng một cây, cùng một thứ tự — hai cách viết. Đặt cạnh nhau xem chênh
lệch nằm ở đâu.
::::

::::explain{#hai-ban-cung-mot-viec}
Bài "Duyệt cây: ba thứ tự, ba câu chuyện khác nhau" duyệt cây theo
Trái-Gốc-Phải bằng `while` cộng một `ngan_xep` tự quản — bạn tự
`.append` khi đi sâu về bên trái, tự `.pop()` khi hết trái phải lùi
lại. Bài trước vừa chốt: đệ quy làm đúng việc ấy, chỉ là ngăn xếp do
trình thông dịch giữ, không phải bạn.

Giờ kiểm chứng bằng cách viết lại **chính việc đó** — đúng cây bảy số
báo danh cũ, đúng thứ tự Trái-Gốc-Phải — bằng đệ quy:

```python title=readonly
goc = {"gia_tri": 50, "trai": None, "phai": None}
goc["trai"] = {"gia_tri": 30, "trai": None, "phai": None}
goc["phai"] = {"gia_tri": 70, "trai": None, "phai": None}
goc["trai"]["trai"] = {"gia_tri": 20, "trai": None, "phai": None}
goc["trai"]["phai"] = {"gia_tri": 40, "trai": None, "phai": None}
goc["phai"]["trai"] = {"gia_tri": 65, "trai": None, "phai": None}
goc["phai"]["phai"] = {"gia_tri": 90, "trai": None, "phai": None}

def tham_trai_goc_phai(nut, ket_qua):
    if nut is None:
        return
    tham_trai_goc_phai(nut["trai"], ket_qua)
    ket_qua.append(nut["gia_tri"])
    tham_trai_goc_phai(nut["phai"], ket_qua)

ket_qua = []
tham_trai_goc_phai(goc, ket_qua)
print(ket_qua)
```

```text title=readonly
[20, 30, 40, 50, 65, 70, 90]
```

Đúng dãy tăng dần y hệt bản `while`. Nhưng nhìn kỹ thân hàm: chỉ năm
dòng, không có `ngan_xep`, không có `while`, không `con_tro`. Nhánh
dừng (`if nut is None: return`) chính là chỗ một tờ phiếu không đẩy
gì thêm — đúng "trường hợp cơ sở" bạn đã học từ trước, không phải khái
niệm mới.

Có một chỗ lệch đáng để dừng lại: bản `while` phải đẩy nhánh **phải**
vào ngăn xếp trước, nhánh **trái** sau — một mẹo, vì ngăn xếp lấy ra
kiểu vào-sau-ra-trước nên phải đẩy ngược thứ tự muốn xử lý. Bản đệ quy
ở trên gọi `tham_trai_goc_phai(nut["trai"], ...)` **trước**, đúng
thứ tự tự nhiên muốn đi — không cần đảo gì cả. Vì sao? Vì ngăn xếp của
trình thông dịch tự khớp đúng: lời gọi trái được đẩy lên và cũng được
xử lý xong (gỡ ra) hoàn toàn trước khi dòng gọi phải chạy tới — không
ai đẩy cả hai lên rồi mới lấy, như bản `while` phải làm.
::::

::::example{#doi-nhanh-tham-truoc}
Đổi thứ tự hai lời gọi đệ quy — thăm nhánh **phải** trước, **trái**
sau — trên đúng cây bảy số báo danh:

```python title=readonly
def tham_phai_goc_trai(nut, ket_qua):
    if nut is None:
        return
    tham_phai_goc_trai(nut["phai"], ket_qua)
    ket_qua.append(nut["gia_tri"])
    tham_phai_goc_trai(nut["trai"], ket_qua)

ket_qua = []
tham_phai_goc_trai(goc, ket_qua)
print(ket_qua)
```

```text title=readonly
[90, 70, 65, 50, 40, 30, 20]
```

Giảm dần — đúng ngược lại bản tăng dần lúc nãy, không sai sót gì.
Trong bản `while`, đổi thứ tự thăm buộc phải đổi luôn thứ tự đẩy vào
ngăn xếp (nhớ đúng cách nó lấy ra ngược). Trong bản đệ quy, chỉ cần
đảo đúng hai dòng gọi — không phải nghĩ thêm về việc gì bị lấy ra
trước, vì trình thông dịch tự lo phần đó, đúng như cách nó vẫn lo cho
mọi lời gọi hàm từ trước tới giờ.
::::

::::predict{#doan-goc-truoc-phai-truoc commitOnce}
Cây bảy số báo danh như cũ. Một đoạn mã ghé **chính nút** trước rồi
mới xuống hai nhánh con — Gốc-Trái-Phải:

```python
def tham_goc_truoc(nut):
    if nut is None:
        return
    thu_tu_goc_truoc.append(nut["gia_tri"])
    tham_goc_truoc(nut["trai"])
    tham_goc_truoc(nut["phai"])

thu_tu_goc_truoc = []
tham_goc_truoc(goc)
```

**Trước khi chạy**, bạn đoán `thu_tu_goc_truoc` cuối cùng là gì?

:::opt{correct}
`[50, 30, 20, 40, 70, 65, 90]` — 50 đứng đầu vì được ghi ngay khi vừa
vào thân hàm, trước cả hai lời gọi con
:::

:::opt
`[20, 30, 40, 50, 65, 70, 90]` — dãy tăng dần, giống bản Trái-Gốc-Phải
::why
Gần đúng ở việc bạn nhớ đúng: cây này ĐÃ từng cho ra dãy tăng dần —
nhưng đó là kết quả của Trái-Gốc-Phải (`tham_trai_goc_phai`), một
đoạn mã khác.

Chỗ lệch: đoạn mã ở đây ghi giá trị nút **trước** cả hai lời gọi con
(`ket_qua.append(...)` nằm trên hai dòng gọi đệ quy), không phải ở
giữa như bản trước. Thứ tự ghi đổi thì kết quả cũng đổi hẳn — dãy tăng
dần chỉ đúng khi ghi nút SAU khi đã thăm xong nhánh trái.
::
:::

:::opt
`[20, 30, 40, 50, 65, 70, 90]` bị đảo ngược, ra `[90, 70, 65, 50, 40, 30, 20]`
::why
Gần đúng ở việc bạn để ý hai lời gọi con vẫn theo thứ tự trái-rồi-
phải trong dòng lệnh — quan sát đó không sai.

Chỗ lệch: đoạn mã này KHÔNG đổi thứ tự trái/phải (đó là ví dụ ở phần
trước, `tham_phai_goc_trai`). Đoạn ở đây chỉ đổi chỗ dòng ghi giá trị
— đưa nó lên TRƯỚC hai lời gọi con — chứ không đổi trái/phải. Dãy
không hề bị đảo ngược, chỉ khác ở CHỖ mỗi giá trị được chèn vào.
::
:::

:::opt
Giống hệt `[50, 30, 20, 40, 70, 65, 90]`, nhưng phải gọi
`tham_goc_truoc(goc)` hai lần vì hàm không có `return`
::why
Gần đúng ở việc bạn để ý hàm này không viết `return` nào — quan sát
đó đúng.

Chỗ lệch: hàm không cần `return` để làm việc, vì nó không TRẢ kết quả
qua giá trị hàm — nó GHI trực tiếp vào `thu_tu_goc_truoc`, một danh
sách tồn tại BÊN NGOÀI hàm và mọi lời gọi (kể cả lời gọi con) đều
cùng nhìn thấy, cùng ghi vào. Gọi đúng một lần là đủ để duyệt hết cây.
::
:::
::::

::::code{#tham-goc-truoc-de-quy}
Cây bảy số báo danh (readonly). Viết hàm duyệt Gốc-Trái-Phải bằng đệ
quy — đúng việc bài "Duyệt cây" từng làm bằng `while` cộng ngăn xếp,
lấy lại đúng kết quả `[50, 30, 20, 40, 70, 65, 90]`.

Nhánh dừng và dòng ghi giá trị đã viết sẵn. Việc của bạn: gọi lại
đúng hàm này cho nhánh trái, rồi cho nhánh phải — đúng thứ tự tự
nhiên, không cần đảo gì.

```python title=starter
goc = {"gia_tri": 50, "trai": None, "phai": None}
goc["trai"] = {"gia_tri": 30, "trai": None, "phai": None}
goc["phai"] = {"gia_tri": 70, "trai": None, "phai": None}
goc["trai"]["trai"] = {"gia_tri": 20, "trai": None, "phai": None}
goc["trai"]["phai"] = {"gia_tri": 40, "trai": None, "phai": None}
goc["phai"]["trai"] = {"gia_tri": 65, "trai": None, "phai": None}
goc["phai"]["phai"] = {"gia_tri": 90, "trai": None, "phai": None}

thu_tu_goc_truoc = []

def tham_goc_truoc(nut):
    if nut is None:
        return
    thu_tu_goc_truoc.append(nut["gia_tri"])
    ___                            # ghé nhánh TRÁI, đúng cách gọi lại chính hàm này
    ___                            # rồi ghé nhánh PHẢI

tham_goc_truoc(goc)
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

thu_tu_goc_truoc = []

def tham_goc_truoc(nut):
    if nut is None:
        return
    thu_tu_goc_truoc.append(nut["gia_tri"])
    tham_goc_truoc(nut["trai"])
    tham_goc_truoc(nut["phai"])

tham_goc_truoc(goc)
print(thu_tu_goc_truoc)
```

```python title=test
assert thu_tu_goc_truoc == [50, 30, 20, 40, 70, 65, 90], f"thứ tự Gốc-Trái-Phải trên cây này phải là [50, 30, 20, 40, 70, 65, 90] — đang ra {thu_tu_goc_truoc}"
assert len(thu_tu_goc_truoc) == 7, "phải ghé đủ cả bảy nút, không bỏ sót và không lặp lại"
assert thu_tu_goc_truoc[0] == 50, "phần tử ĐẦU TIÊN phải là gốc (50) — Gốc-Trái-Phải ghi gốc trước cả hai nhánh con"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều là LỜI GỌI LẠI CHÍNH HÀM tham_goc_truoc — không phải .append, không phải while. Đây là đệ quy, không phải ngăn xếp tự quản.
- kind: strategy
  body: 'Gọi tham_goc_truoc(nut["trai"]) trước, rồi tham_goc_truoc(nut["phai"]) sau — đúng thứ tự tự nhiên bạn muốn thăm, không cần đảo như bản .append/.pop từng phải làm. Nhánh dừng if nut is None: return đã lo sẵn việc không đi lạc khi chạm tới None.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là tham_goc_truoc(nut["trai"]) và tham_goc_truoc(nut["phai"]).'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải là LỜI GỌI LẠI chính tham_goc_truoc — một cho nhánh trái, một cho nhánh phải — không phải .append, không phải while
  requireAst:
  - kind: uses-call, target: tham_goc_truoc, min: 3
  # min: 3 — luật này đếm trên TOÀN BỘ mã nguồn, không chỉ trong thân hàm, nên
  # lời gọi ngoài cùng tham_goc_truoc(goc) cũng được tính. Đếm thật trên
  # solution: 1 (lời gọi ngoài cùng, có sẵn trong khung) + 2 (hai chỗ trống)
  # = 3. Điền True/1/0 (một câu không làm gì) vào một hoặc cả hai chỗ trống —
  # ĐÃ THỬ THẬT bằng kiemAst thật: cả ba cách hụt đều cho đúng 1 (chỉ còn lời
  # gọi ngoài cùng) — dưới 3, luật này chặn được. Cả ba cũng dừng AN TOÀN,
  # không đệ quy thêm bước nào (nhánh dừng vẫn nguyên trong khung, không phụ
  # thuộc chỗ trống), và tests/output bắt độc lập vì thu_tu_goc_truoc chỉ còn
  # [50] thay vì đủ bảy phần tử.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\[50, 30, 20, 40, 70, 65, 90\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm dòng, không .append ngăn xếp nào, không while nào — mà ra đúng
kết quả bản mười lăm dòng từng ra. Trình thông dịch giữ chồng giùm
bạn thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cây bảy số báo danh chỉ sâu ba tầng. Chồng lời gọi lúc `tham_goc_truoc`
chạy tới nút sâu nhất cũng chỉ cao ba tờ phiếu — quá xa so với mức
`RecursionError` từng gặp trong bài "Hàm gọi chính nó".

Nếu cây không có ba tầng mà có một nghìn tầng — hay đơn giản hơn, một
hàm đệ quy cứ gọi mãi không có gì ngăn nó lại — thì đúng bao nhiêu
tầng chồng lời gọi mới vỡ? Con số ấy có thật, và đo được — trên
CHÍNH máy bạn đang chạy, không phải chép từ đâu đó.

Bài sau đo thẳng con số đó.
::::

::::checkpoint{mastery=0.8}
::::
