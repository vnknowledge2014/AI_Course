---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.dfs-bang-de-quy
title: "DFS viết bằng đệ quy"
summary: "DFS viết bằng ngăn xếp tự quản (bài trước) và DFS viết bằng đệ quy ra đúng CÙNG một thứ tự thăm — vì chúng là MỘT thuật toán, chỉ khác ai giữ ngăn xếp: bạn, hay trình thông dịch."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.dfs-recursive]
requires: [alg.dfs, alg.recursion-vs-loop]
concepts: [alg.dfs-recursive]
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
Không `.append`, không `.pop` nào cả. Chỉ một hàm gọi lại chính nó — và
cùng một thứ tự thăm hiện ra.
::::

::::explain{#khong-can-tu-quan-ngan-xep}
Bài "Viết lại đúng việc cũ bằng đệ quy" đã kiểm chứng đúng việc này trên
CÂY: bản `while` cộng ngăn xếp tự quản, và bản đệ quy, ra cùng một thứ
tự Trái-Gốc-Phải — vì trình thông dịch tự giữ ngăn xếp giùm bạn, đúng cơ
chế bài "Đệ quy chính là một ngăn xếp" đã chỉ ra.

Đồ thị không khác gì cả. DFS viết bằng đệ quy không cần biến `ngan_xep`,
không cần `.pop()`, không cần dòng `reversed(...)` để "sửa" thứ tự đẩy
vào — chỉ cần một hàm: thăm đỉnh hiện tại, rồi GỌI LẠI CHÍNH NÓ cho từng
hàng xóm chưa thăm, theo ĐÚNG thứ tự liệt kê trong `bang_ke`:

```python
def dfs_de_quy(bang_ke, dinh, da_tham, thu_tu_tham):
    da_tham.add(dinh)
    thu_tu_tham.append(dinh)
    for hang_xom in bang_ke[dinh]:
        if hang_xom not in da_tham:
            dfs_de_quy(bang_ke, hang_xom, da_tham, thu_tu_tham)
```

Không có mẹo `reversed(...)` nào ở đây — mà cũng không cần. Lời gọi
`dfs_de_quy(bang_ke, hang_xom, ...)` cho hàng xóm ĐẦU TIÊN trong danh
sách kề chạy TRỌN VẸN — đi hết cả nhánh của nó, đẩy rồi gỡ hết mọi tờ
phiếu — TRƯỚC KHI dòng `for` kịp thử tới hàng xóm thứ hai. Đây đúng là
lý do bài "Viết lại đúng việc cũ bằng đệ quy" từng chỉ ra: ngăn xếp của
trình thông dịch tự khớp đúng thứ tự tự nhiên, không cần đảo gì. Bản
ngăn xếp TỰ QUẢN (bài trước) phải dùng `reversed(...)` chính là để BÙ
LẠI cho việc `.pop()` lấy ra ngược thứ tự đẩy vào — một việc mà đệ quy
không bao giờ gặp phải, vì trình thông dịch không hề "lấy ra ngược" bất
cứ thứ gì.

Đánh dấu `da_tham` ở đây làm NGAY LÚC VÀO HÀM — đúng đối xứng với việc
ngăn xếp tự quản bài trước đánh dấu ngay lúc `.pop()` ra khỏi ngăn xếp:
cả hai đều đánh dấu đúng lúc một đỉnh THỰC SỰ được xử lý, không phải lúc
nó chỉ mới được PHÁT HIỆN.
::::

::::example{#dfs-de-quy-tren-mang-nam-dinh}
Đúng mạng năm đỉnh hai bài trước, giờ duyệt bằng đệ quy:

```python title=readonly
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}

def dfs_de_quy(bang_ke, dinh, da_tham, thu_tu_tham):
    da_tham.add(dinh)
    thu_tu_tham.append(dinh)
    print(f"vào {dinh}, chồng gọi hiện tại có: {thu_tu_tham}")
    for hang_xom in bang_ke[dinh]:
        if hang_xom not in da_tham:
            dfs_de_quy(bang_ke, hang_xom, da_tham, thu_tu_tham)

da_tham = set()
thu_tu_tham = []
dfs_de_quy(bang_ke, "An", da_tham, thu_tu_tham)
print(f"Thứ tự thăm: {thu_tu_tham}")
```

```text title=readonly
vào An, chồng gọi hiện tại có: ['An']
vào Bình, chồng gọi hiện tại có: ['An', 'Bình']
vào Dung, chồng gọi hiện tại có: ['An', 'Bình', 'Dung']
vào Em, chồng gọi hiện tại có: ['An', 'Bình', 'Dung', 'Em']
vào Chi, chồng gọi hiện tại có: ['An', 'Bình', 'Dung', 'Em', 'Chi']
Thứ tự thăm: ['An', 'Bình', 'Dung', 'Em', 'Chi']
```

Đúng — TỪNG CHỮ — thứ tự bản ngăn xếp tự quản bài trước cho ra:
`['An', 'Bình', 'Dung', 'Em', 'Chi']`. Đọc kỹ dòng thứ năm: `Chi` chỉ
được gọi SAU CÙNG, sau khi lời gọi vào `Bình` — và toàn bộ chồng lời gọi
bên trong nó (`Dung`, rồi `Em`) — đã chạy XONG HẲN và trả về. Dòng `for
hang_xom in bang_ke["An"]` không hề "quên" `Chi`; nó chỉ đơn giản CHƯA
tới lượt, vì lời gọi `dfs_de_quy(..., "Bình", ...)` phải chạy trọn vẹn
trước khi dòng `for` ở lượt gọi `"An"` được phép thử phần tử tiếp theo.
::::

::::predict{#doan-neu-goi-chi-truoc commitOnce}
Cùng mạng, nhưng đổi thứ tự liệt kê hàng xóm của `An` — giống hệt bài
trước đã làm với bản ngăn xếp:

```python
bang_ke = {
    "An": ["Chi", "Bình"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}
```

**Trước khi chạy**, bạn đoán `dfs_de_quy` cho ra thứ tự nào?

:::opt{correct}
`['An', 'Chi', 'Bình', 'Dung', 'Em']` — đúng thứ tự bản ngăn xếp tự
quản cho ra khi đổi thứ tự liệt kê y hệt vậy ở bài trước, vì đây là MỘT
thuật toán
:::

:::opt
`['An', 'Bình', 'Dung', 'Em', 'Chi']` — bản đệ quy không cần `reversed`
nên không bị ảnh hưởng bởi thứ tự liệt kê trong `bang_ke`
::why
Gần đúng ở việc bạn nhớ đúng: bản đệ quy KHÔNG dùng `reversed(...)` —
quan sát đó có thật, đúng như bài vừa giải thích.

Chỗ lệch: "không cần `reversed`" không có nghĩa "không phụ thuộc thứ tự
liệt kê". Dòng `for hang_xom in bang_ke[dinh]:` vẫn duyệt qua danh sách
kề theo ĐÚNG thứ tự nó được viết ra — chỉ là bản đệ quy đi theo thứ tự
đó THUẬN, còn bản ngăn xếp phải đảo ngược trước khi đẩy để ra cùng kết
quả THUẬN đó. Đổi `Chi` lên đầu danh sách của `An` thì cả hai bản đều
thăm `Chi` trước `Bình`, không có bản nào "miễn nhiễm".
::
:::

:::opt
`['An', 'Chi', 'Dung', 'Em', 'Bình']` — `Chi` thăm trước nên "chiếm"
lượt gọi tiếp theo, kéo cả nhánh `Dung`, `Em` vào giữa
::why
Gần đúng ở việc bạn đúng đỉnh THỨ HAI — `Chi` thật sự được gọi ngay sau
`An`.

Chỗ lệch: lời gọi `dfs_de_quy(..., "Chi", ...)` chạy XONG HẲN trước khi
lượt gọi của `An` được thử phần tử tiếp theo trong `for` — mà `Chi` chỉ
có đúng một hàng xóm, `An`, đã thăm rồi, nên lời gọi vào `Chi` không gọi
thêm ai cả, TRẢ VỀ ngay. Chỉ sau đó, lượt gọi của `An` mới thử tới
`Bình`, và từ `Bình` mới lao vào `Dung` rồi `Em`. `Dung`, `Em` không hề
"chen" vào giữa `Chi` và `Bình`.
::
:::

:::opt
Không xác định được — đệ quy có thể gọi hai nhánh theo bất kỳ thứ tự
nào, tuỳ trình thông dịch quyết định lúc chạy
::why
Gần đúng ở sự thận trọng khi nói về một thứ "máy tự quyết định" — thái
độ đó đúng với một số cơ chế khác của máy tính (ví dụ lịch chạy tiến
trình).

Chỗ lệch: một dòng `for hang_xom in bang_ke[dinh]:` không hề "tự quyết
định" thứ tự — nó LUÔN đi qua `list` theo đúng thứ tự các phần tử nằm
trong đó, từ đầu tới cuối, mỗi lần chạy đều giống hệt nhau. Không có
phần nào của Python được phép đảo thứ tự một `list` khi lặp qua nó.
::
:::
::::

::::code{#viet-dfs-de-quy}
Hoàn thiện `dfs_de_quy`. Việc đánh dấu và ghi nhận đã viết sẵn — việc
của bạn là khởi tạo đúng cấu trúc rỗng cho `da_tham`, và gọi lại chính
hàm này cho một hàng xóm chưa thăm.

```python title=starter
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}

def dfs_de_quy(bang_ke, dinh, da_tham, thu_tu_tham):
    da_tham.add(dinh)
    thu_tu_tham.append(dinh)
    for hang_xom in bang_ke[dinh]:
        if hang_xom not in da_tham:
            ___                        # gọi lại CHÍNH HÀM NÀY cho hang_xom

da_tham = ___                          # tập rỗng, sẵn sàng nhận đỉnh đã thăm
thu_tu_tham = []
dfs_de_quy(bang_ke, "An", da_tham, thu_tu_tham)
print(f"Thứ tự DFS đệ quy từ An: {thu_tu_tham}")
```

```python title=solution
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}

def dfs_de_quy(bang_ke, dinh, da_tham, thu_tu_tham):
    da_tham.add(dinh)
    thu_tu_tham.append(dinh)
    for hang_xom in bang_ke[dinh]:
        if hang_xom not in da_tham:
            dfs_de_quy(bang_ke, hang_xom, da_tham, thu_tu_tham)

da_tham = set()
thu_tu_tham = []
dfs_de_quy(bang_ke, "An", da_tham, thu_tu_tham)
print(f"Thứ tự DFS đệ quy từ An: {thu_tu_tham}")
```

```python title=test
assert thu_tu_tham == ["An", "Bình", "Dung", "Em", "Chi"], f"DFS đệ quy từ An phải cho đúng cùng thứ tự bản ngăn xếp tự quản bài trước — ['An', 'Bình', 'Dung', 'Em', 'Chi'] — đang ra {thu_tu_tham}"
assert len(thu_tu_tham) == 5, "phải thăm đủ cả năm đỉnh, không bỏ sót và không lặp lại đỉnh nào"
assert isinstance(da_tham, set), f"da_tham phải là một TẬP RỖNG (set()) để .add(...) trong hàm hoạt động đúng — đang là {type(da_tham).__name__}"
```

:::hints
- kind: attention
  body: Chỗ trống 1 nằm TRONG thân hàm, là lời GỌI LẠI CHÍNH dfs_de_quy — không phải .append, không phải while. Chỗ trống 2 nằm NGOÀI hàm, là giá trị khởi tạo của da_tham — nó phải hỗ trợ .add(...), như bên trong hàm đang dùng.
- kind: strategy
  body: 'Chỗ trống 1: gọi lại chính hàm này với hang_xom thay cho dinh — dfs_de_quy(bang_ke, hang_xom, da_tham, thu_tu_tham), giữ nguyên hai tham số da_tham và thu_tu_tham để MỌI lời gọi con cùng ghi vào một chỗ. Chỗ trống 2: da_tham phải bắt đầu RỖNG — set().'
- kind: one-line
  body: 'Chỗ trống 1 là `dfs_de_quy(bang_ke, hang_xom, da_tham, thu_tu_tham)`; chỗ trống 2 là `set()`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống 1 phải THẬT SỰ gọi lại chính dfs_de_quy cho hang_xom — đây là đệ quy, không phải vòng lặp hay .append; chỗ trống 2 phải THẬT SỰ tạo một tập rỗng bằng set() — không gõ cứng True, 1, 0, những giá trị không có .add(...)
  requireAst:
  - kind: uses-call, target: dfs_de_quy, min: 2
  - kind: uses-call, target: set, min: 1
  # Đếm thật trên solution: dfs_de_quy( xuất hiện đúng 2 lần — lời gọi đệ quy
  # bên trong thân hàm (chỗ trống 1) cộng lời gọi ngoài cùng khởi động cả DFS
  # (có sẵn trong khung, "dfs_de_quy(bang_ke, 'An', da_tham, thu_tu_tham)").
  # set( xuất hiện đúng 1 lần — chỗ trống 2. ĐÃ THỬ THẬT bằng kiemAst: điền
  # True/1/0 vào chỗ trống 1 làm dfs_de_quy( tụt xuống 1 (dưới 2) — chặn được;
  # điền vào chỗ trống 2 làm set( tụt xuống 0 (dưới 1) — chặn được. ĐÃ CHẠY
  # THỬ BA CÁCH ĐIỀN BỪA True/1/0 cho CẢ HAI chỗ trống CÙNG LÚC (bắt buộc vì
  # khối có đệ quy): chỗ trống 1 thành một câu không làm gì -> không gọi đệ quy
  # thêm lần nào, thu_tu_tham chỉ còn ['An'] — kết thúc AN TOÀN, nhanh, không hề
  # đệ quy thêm bước nào (không phụ thuộc chỗ trống 2). Nếu chỗ trống 2 CŨNG bị
  # điền bừa cùng lúc (da_tham = True/1/0), lệnh da_tham.add("An") ngay lượt gọi
  # ĐẦU TIÊN ném AttributeError ngay lập tức ('bool'/'int' object has no
  # attribute 'add') — dừng AN TOÀN bằng lỗi CÓ báo, không lặp vô hạn, cho cả
  # ba cách điền.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Thứ tự DFS đệ quy từ An: \\['An', 'Bình', 'Dung', 'Em', 'Chi'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không một dòng `.pop` nào — mà thứ tự thăm giống hệt bản ngăn xếp tự
quản, tới từng chữ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ trong tay có hai cách đi thăm một đồ thị: BFS (hàng đợi, lan đều
từng lớp, đo được khoảng cách theo cạnh) và DFS (ngăn xếp hoặc đệ quy,
lao thẳng theo một nhánh). Cả hai đều thăm được HẾT mọi đỉnh tới được từ
điểm xuất phát — không đỉnh nào là "chỉ BFS mới tới được" hay "chỉ DFS
mới tới được".

Nếu cả hai đều làm được cùng một việc — thăm hết đồ thị — thì có bao giờ
việc CHỌN đúng cái nào lại thực sự quan trọng không? Hay chỉ cần chọn
đại một cái, kết quả cũng như nhau?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
