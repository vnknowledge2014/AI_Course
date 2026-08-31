---
id: khoa-hoc-may-tinh.ngon-ngu-va-may-truu-tuong.trang-thai-may-bo-nho-nho-cay-dang-danh-gia
title: "Trạng thái máy: một bộ nhớ nhỏ, một cây đang đánh giá"
summary: "Máy trừu tượng bài 16 cần NHỚ nó đang ở đâu trong cây, giống con trỏ lệnh T3.4 bài 7 — nhưng với đánh giá đệ quy, 'đang ở đâu' chính là ngăn xếp cuộc gọi hàm (T3.3 bài 1, T3.4 bài 23) của chính lời gọi hàm đánh giá, không phải một con số riêng."
locale: vi
track: khoa-hoc-may-tinh
module: ngon-ngu-va-may-truu-tuong
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ngu.machine-state]
requires: [ngu.abstract-machine]
concepts: [ngu.machine-state]
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
Không có biến con trỏ nào bạn tự viết ra cả — nhưng máy vẫn biết chính
xác nó đang đứng ở đâu. Bí mật nằm ở một chỗ bạn đã quen: chồng lời gọi.
::::

::::explain{#trang-thai-nam-o-dau}
T3.4 bài 7 dạy: máy thật cần một **con trỏ lệnh** — một con số riêng,
tự quản, cho biết chính xác nó đang đứng ở LỆNH THỨ MẤY trong danh sách
lệnh phẳng.

Bộ đánh giá cây của bạn không có danh sách phẳng nào, và không có biến
con trỏ nào bạn tự viết. Vậy nó "đang ở đâu trong cây" bằng cách nào?

Câu trả lời nằm ở đúng chỗ T3.3 bài 1 đã dạy: mỗi lần một hàm được gọi,
trình thông dịch tự đặt một **khung** (frame) mới lên **ngăn xếp cuộc
gọi hàm** — khung đó ghi lại tham số của đúng lời gọi ấy (T3.4 bài 23).
Mỗi lần `danh_gia(node)` được gọi — kể cả khi nó gọi lại chính nó — một
khung mới được đặt lên, và khung đó GHI CHÍNH XÁC: tham số `node` của
lần gọi này là NÚT NÀO trong cây.

> **"Đang ở đâu trong cây" = "khung nào đang ở ĐỈNH ngăn xếp cuộc gọi
> hàm, ngay lúc đó."** Khung ở đỉnh luôn ứng với nút ĐANG được xử lý —
> đúng vai trò con trỏ lệnh vẫn làm cho máy thật, chỉ khác: con trỏ lệnh
> là MỘT con số duy nhất, còn ở đây "vị trí hiện tại" nằm rải trên CẢ
> CHỒNG khung, một khung cho mỗi tầng đang dang dở.

Mỗi khung còn giữ thêm một việc nữa: với một `ast.BinOp`, khung đó tạm
giữ `trai` sau khi lời gọi con bên trái trả về, CHỜ `phai` tính xong mới
gộp cả hai lại. Đây chính là **bộ nhớ nhỏ** của trạng thái máy — không
phải một biến DÙNG CHUNG cho mọi lời gọi, mà mỗi khung có bản `trai`,
`phai` RIÊNG của nó, không đụng nhau, kể cả khi hai khung đang xử lý
CÙNG một hàm `danh_gia` cùng lúc (một lời gọi đệ quy lồng trong lời gọi
khác).

Tóm lại: **trạng thái của máy trừu tượng này = toàn bộ ngăn xếp cuộc gọi
hàm hiện có, mỗi khung giữ (nút đang xử lý, phần kết quả con đã tính
được cho tới lúc này).**
::::

::::example{#do-sau-hien-hinh}
Làm cho "đang ở đâu" hiện hình bằng một con số: một biến `do_sau`, TĂNG
lên 1 mỗi khi `danh_gia` được GỌI, GIẢM đi 1 ngay trước khi nó TRẢ VỀ —
đúng con số khung đang chồng lên nhau lúc đó.

```python title=readonly
import ast

def nhan_dang(node):
    if isinstance(node, ast.Constant):
        return f"Constant({node.value})"
    return f"BinOp({type(node.op).__name__})"

do_sau = 0
nhat_ky = []

def danh_gia(node):
    global do_sau
    do_sau += 1
    nhat_ky.append(f"[sâu {do_sau}] ĐẨY {nhan_dang(node)}")
    if isinstance(node, ast.Constant):
        ket_qua = node.value
    else:
        trai = danh_gia(node.left)
        phai = danh_gia(node.right)
        if isinstance(node.op, ast.Add):
            ket_qua = trai + phai
        elif isinstance(node.op, ast.Mult):
            ket_qua = trai * phai
    nhat_ky.append(f"[sâu {do_sau}] LẤY {nhan_dang(node)} = {ket_qua}")
    do_sau -= 1
    return ket_qua

cay = ast.parse("2 + 3 * 4", mode="eval").body
print(danh_gia(cay))
for dong in nhat_ky:
    print(dong)
```

```text title=readonly
14
[sâu 1] ĐẨY BinOp(Add)
[sâu 2] ĐẨY Constant(2)
[sâu 2] LẤY Constant(2) = 2
[sâu 2] ĐẨY BinOp(Mult)
[sâu 3] ĐẨY Constant(3)
[sâu 3] LẤY Constant(3) = 3
[sâu 3] ĐẨY Constant(4)
[sâu 3] LẤY Constant(4) = 4
[sâu 2] LẤY BinOp(Mult) = 12
[sâu 1] LẤY BinOp(Add) = 14
```

`do_sau` chạm mốc `3` đúng lúc máy đang xử lý `Constant(3)` hoặc
`Constant(4)` — hai lá NẰM SÂU NHẤT trong cây, vì cả hai đều nằm trong
nhánh `3 * 4`, chính nhánh mà độ ưu tiên toán tử (bài 8) buộc phải tính
TRƯỚC. `do_sau` không phải một khái niệm mới — nó chỉ là con số ĐẾM
khung đang có mặt trên ngăn xếp cuộc gọi hàm, làm hiện hình thứ vẫn ẩn
phía sau mỗi lần bạn gọi một hàm đệ quy.
::::

::::predict{#do-sau-toi-da commitOnce}
Biểu thức `"(6 - 2) * (1 + 4)"`. Cây của nó: gốc là `BinOp(Mult)`, nhánh
trái là `BinOp(Sub)` chứa hai lá `Constant(6)`, `Constant(2)`, nhánh
phải là `BinOp(Add)` chứa hai lá `Constant(1)`, `Constant(4)`.

Dùng đúng bộ đánh giá có đếm `do_sau` như ví dụ trên (không đọc lại toàn
bộ code, chỉ cần nhớ: mỗi lần GỌI cộng 1, mỗi lần TRẢ VỀ trừ 1).

**Trước khi chạy máy**, `do_sau` đạt giá trị LỚN NHẤT là bao nhiêu trong
suốt quá trình đánh giá biểu thức này?

:::opt{correct}
`3` — đạt được khi máy xử lý một trong bốn lá (`Constant(6)`,
`Constant(2)`, `Constant(1)`, hoặc `Constant(4)`)
:::

:::opt
`2` — vì cây chỉ có hai TẦNG con của gốc (nhánh trái và nhánh phải)
::why
Gần đúng ở việc bạn đếm đúng SỐ TẦNG BinOp — gốc (`Mult`) và ngay dưới
nó là `Sub`/`Add`, đúng hai tầng `BinOp`.

Chỗ lệch: bạn dừng đếm ở tầng `BinOp` cuối cùng, quên rằng MỖI lá
(`Constant`) cũng được `danh_gia` GỌI riêng — tức cũng có khung riêng
của nó, thêm một tầng SÂU HƠN nữa. Gốc (sâu 1) → `Sub`/`Add` (sâu 2) →
từng lá (sâu 3) — ba tầng, không phải hai.
::
:::

:::opt
`4` — vì bốn lá của cây khiến `do_sau` phải tăng thêm một lần nữa cho
mỗi lá
::why
Gần đúng ở việc bạn để ý đúng: cây này có BỐN lá — đúng số lượng.

Chỗ lệch: SỐ LƯỢNG lá không quyết định ĐỘ SÂU tối đa — cả bốn lá đều nằm
CÙNG MỘT tầng (sâu 3), không lá nào nằm sâu hơn lá nào. `do_sau` tăng
thêm một lần MỖI KHI có một khung MỚI được đặt LÊN TRÊN khung khác đang
dở — không phải mỗi khi có thêm một lá bất kỳ ở đâu trong cây.
::
:::

:::opt
Không đủ dữ kiện để biết, vì còn phụ thuộc Python đánh giá nhánh trái
hay nhánh phải của phép nhân trước
::why
Gần đúng ở sự thận trọng — đúng là THỨ TỰ trái/phải có thật (bài 12: nút
trong luôn hỏi con trái trước, rồi mới hỏi con phải).

Chỗ lệch: thứ tự trái/phải chỉ đổi THỨ TỰ các dòng nhật ký, không đổi
ĐỘ SÂU TỐI ĐA đạt được. Dù đánh giá nhánh trái (`Sub`) trước hay nhánh
phải (`Add`) trước, khung sâu nhất luôn là một khung LÁ nằm ngay dưới
một khung `BinOp` tầng hai — độ sâu tối đa vẫn là 3 trong cả hai trường
hợp.
::
:::
::::

::::code{#hien-hinh-do-sau}
Viết đúng bộ đánh giá có đếm `do_sau` cho biểu thức
`"10 - (4 + 2) * 3"`. Hai chỗ trống đều nằm bên trong `danh_gia`: một
chỗ ngay khi VÀO một lời gọi mới (TĂNG `do_sau`), một chỗ ngay TRƯỚC khi
lời gọi đó TRẢ VỀ (GIẢM `do_sau`) — đúng cặp lệnh làm hiện hình một khung
được đặt lên, rồi gỡ xuống.

```python title=starter
import ast

do_sau = 0
do_sau_toi_da = 0

def danh_gia(node):
    global do_sau, do_sau_toi_da
    ___                                # VÀO một lời gọi mới: TĂNG do_sau
    do_sau_toi_da = max(do_sau_toi_da, do_sau)
    if isinstance(node, ast.Constant):
        ket_qua = node.value
    elif isinstance(node, ast.BinOp):
        trai = danh_gia(node.left)
        phai = danh_gia(node.right)
        if isinstance(node.op, ast.Add):
            ket_qua = trai + phai
        elif isinstance(node.op, ast.Sub):
            ket_qua = trai - phai
        elif isinstance(node.op, ast.Mult):
            ket_qua = trai * phai
        elif isinstance(node.op, ast.Div):
            ket_qua = trai / phai
        else:
            raise ValueError(f"chưa hiểu phép toán {type(node.op).__name__}")
    else:
        raise ValueError(f"chưa hiểu loại nút {type(node).__name__}")
    ___                                # RA khỏi lời gọi: GIẢM do_sau, đúng lúc trả về
    return ket_qua

cay = ast.parse("10 - (4 + 2) * 3", mode="eval").body
ket_qua_cuoi = danh_gia(cay)

print(ket_qua_cuoi)
print(do_sau_toi_da)
print(do_sau)
```

```python title=solution
import ast

do_sau = 0
do_sau_toi_da = 0

def danh_gia(node):
    global do_sau, do_sau_toi_da
    do_sau += 1
    do_sau_toi_da = max(do_sau_toi_da, do_sau)
    if isinstance(node, ast.Constant):
        ket_qua = node.value
    elif isinstance(node, ast.BinOp):
        trai = danh_gia(node.left)
        phai = danh_gia(node.right)
        if isinstance(node.op, ast.Add):
            ket_qua = trai + phai
        elif isinstance(node.op, ast.Sub):
            ket_qua = trai - phai
        elif isinstance(node.op, ast.Mult):
            ket_qua = trai * phai
        elif isinstance(node.op, ast.Div):
            ket_qua = trai / phai
        else:
            raise ValueError(f"chưa hiểu phép toán {type(node.op).__name__}")
    else:
        raise ValueError(f"chưa hiểu loại nút {type(node).__name__}")
    do_sau -= 1
    return ket_qua

cay = ast.parse("10 - (4 + 2) * 3", mode="eval").body
ket_qua_cuoi = danh_gia(cay)

print(ket_qua_cuoi)
print(do_sau_toi_da)
print(do_sau)
```

```python title=test
assert ket_qua_cuoi == -8, f"10 - (4 + 2) * 3 phải là 10 - 18 = -8 — đang ra {ket_qua_cuoi}"
assert do_sau_toi_da == 4, f"độ sâu tối đa của cây này phải là 4 (gốc Sub -> nhánh Mult -> nhánh Add -> lá Constant) — đang ra {do_sau_toi_da}; do_sau phải TĂNG mỗi lần VÀO một lời gọi mới"
assert do_sau == 0, f"sau khi mọi lời gọi đã TRẢ VỀ hết, do_sau phải quay lại đúng 0 — đang ra {do_sau}; do_sau phải GIẢM đúng lúc mỗi lời gọi trả về"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều chỉ chạm vào biến do_sau — không chạm gì khác. Chỗ trống đầu TĂNG nó lên, chỗ trống sau GIẢM nó xuống.
- kind: strategy
  body: 'Ngay khi hàm được GỌI (đầu thân hàm) — trước khi biết node là loại gì — do_sau đã có một khung mới: do_sau += 1. Ngay TRƯỚC dòng return ket_qua (sau khi ket_qua đã có giá trị, không phân biệt nhánh lá hay nhánh trong) — khung này sắp bị gỡ: do_sau -= 1.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là: do_sau += 1  và  do_sau -= 1'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: hai chỗ trống phải THẬT SỰ gán lại do_sau (TĂNG lúc vào, GIẢM lúc ra) — điền một câu không làm gì (True/1/0) không hề chạm tới do_sau, nên độ sâu sẽ không bao giờ tăng
  requireAst:
  # min: 3 — đếm thật trên solution: "gan-ten" (Assign + AugAssign) với
  # target do_sau xuất hiện 3 lần trong mã nguồn: dòng khởi tạo
  # "do_sau = 0" có sẵn trong khung, cộng "do_sau += 1" và "do_sau -= 1"
  # ở hai chỗ trống. (do_sau_toi_da là một TÊN KHÁC, không tính vào đếm
  # này.) ĐÃ THỬ THẬT bằng cả ba cách True/1/0 cho CẢ HAI chỗ trống: đệ
  # quy vẫn chạy trên đúng cây cố định của "10 - (4 + 2) * 3" (4 nút lá,
  # sâu tối đa 4 tầng thật) — không cách điền nào ảnh hưởng số lần gọi
  # đệ quy hay khiến nó chạy quá sâu/vô hạn, cả ba dừng ngay lập tức.
  # Cả ba đều còn đúng 1 lần gán do_sau (chỉ dòng khởi tạo) < min 3, nên
  # static trượt độc lập; và cả ba cho do_sau_toi_da == 0 (không phải 4)
  # vì do_sau không hề tăng — assert bắt được độc lập với luật static này.
  - kind: gan-ten, target: do_sau, min: 3
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^-8\\n4\\n0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn tầng khung, một con số hiện hình đúng lúc đúng chỗ — trạng thái máy
không giấu ở đâu xa, nó nằm ngay trên chồng lời gọi bạn đã quen từ lâu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Trạng thái máy của bạn hiện nằm RẢI trên nhiều khung của ngăn xếp cuộc
gọi hàm — do trình thông dịch tự quản, bạn không tự tay `.append`/`.pop`
lên nó được, chỉ nhìn thấy nó qua một con số đếm như `do_sau`. Nhưng máy
tính bằng ngăn xếp THẬT (T3.4 bài 3) lại có một ngăn xếp tính toán DUY
NHẤT, TƯỜNG MINH — `LOAD` đẩy vào, `BINARY_OP` lấy ra tính, không có
"khung" ẩn nào cả.

Muốn đưa bộ đánh giá cây của bạn về ĐÚNG kiểu một ngăn xếp tường minh đó
— cây cú pháp lồng nhau (bài 7-10) cần biến thành CÁI GÌ trước? Bài sau
trả lời.
::::

::::checkpoint{mastery=0.8}
::::
