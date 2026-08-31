---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.phat-hien-chu-trinh-bang-dfs
title: "Phát hiện chu trình bằng DFS"
summary: "DFS mà gặp lại một đỉnh ĐANG TRÊN ĐƯỜNG ĐI HIỆN TẠI — không phải một đỉnh đã thăm xong — nghĩa là có một chu trình: nối lại đúng 'hai cái nồi trỏ vào nhau' của T3.2, giờ phát hiện được bằng thuật toán, không cần bộ dọn rác."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 30
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [alg.cycle-detection]
requires: [alg.dfs]
concepts: [alg.cycle-detection]
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
Hai cái nồi trỏ vào nhau, T3.2 gọi đó là một vòng tham chiếu. Hôm nay là
thuật toán tìm ra chính cái vòng đó — bằng tay, không nhờ bộ dọn rác.
::::

::::explain{#dang-di-khac-da-xong}
T3.2 bài "Hai cái nồi trỏ vào nhau" dựng một cấu trúc: nồi A giữ một
tấm thẻ trỏ sang nồi B, nồi B giữ một tấm thẻ trỏ NGƯỢC LẠI sang nồi A —
một **vòng tham chiếu**. Đếm thẻ tham chiếu bó tay ở đó, và phải nhờ một
bộ dọn riêng (bộ dọn theo vòng) mới gỡ được. "Trỏ tới" là một đường CÓ
HƯỚNG — A trỏ tới B không có nghĩa B tự động trỏ lại A, trừ khi có một
dòng lệnh RIÊNG làm việc đó. Bảng kề cho đúng loại quan hệ này gọi là
**đồ thị có hướng**: mỗi đỉnh chỉ liệt kê những đỉnh nó THỰC SỰ trỏ tới,
không tự thêm chiều ngược.

DFS đi trên một đồ thị có hướng có thể tự phát hiện một vòng, không cần
bộ dọn nào cả — nếu bạn phân biệt đúng HAI loại "đã gặp trước đó":

- **Đã XONG** (`da_xong`): đỉnh này đã được xử lý HOÀN TẤT — DFS đã đi
  hết mọi hàng xóm của nó, không còn gì để làm nữa, và đã quay lui khỏi
  nó. Gặp lại một đỉnh đã xong không phải là chu trình — chỉ đơn giản là
  có hai đường KHÁC NHAU cùng dẫn tới nó, một chuyện hoàn toàn hợp lệ ở
  đồ thị (không phải cây).
- **Đang TRÊN ĐƯỜNG ĐI HIỆN TẠI** (`dang_di`): đỉnh này nằm trên chính
  chuỗi lời gọi ĐANG CHẠY DỞ — DFS đã đi vào nó nhưng CHƯA quay lui. Gặp
  lại một đỉnh loại này nghĩa là đường đi vừa QUAY TRỞ LẠI đúng một
  đỉnh nó chưa kịp rời khỏi — đó chính là định nghĩa của một **chu
  trình** (cycle): một đường có hướng khép kín, quay về đúng nơi nó bắt
  đầu.

```python
def co_chu_trinh(bang_ke, dinh, dang_di, da_xong):
    dang_di.add(dinh)
    for hang_xom in bang_ke[dinh]:
        if hang_xom in dang_di:
            return True                 # gặp lại đỉnh ĐANG đi dở -> chu trình
        if hang_xom not in da_xong:
            if co_chu_trinh(bang_ke, hang_xom, dang_di, da_xong):
                return True
    dang_di.remove(dinh)                # rời khỏi đỉnh này -> không còn "đang đi"
    da_xong.add(dinh)                   # xử lý xong hẳn
    return False
```

Đúng lúc một lượt gọi `co_chu_trinh` chuẩn bị TRẢ VỀ — nghĩa là đỉnh của
nó đã hết hàng xóm để thử — nó gỡ chính mình khỏi `dang_di` (không còn
"đang trên đường đi" nữa) TRƯỚC KHI đánh dấu `da_xong` (đã xử lý xong
hẳn). Đây đúng là bức tranh chồng lời gọi bài "Đệ quy chính là một ngăn
xếp" đã vẽ: đỉnh nào còn trên chồng thì còn "đang đi", đỉnh nào đã gỡ
khỏi chồng thì đã "xong".
::::

::::example{#phat-hien-vong-a-b-c}
Ba đỉnh `An → Bình → Chi → An` — một chu trình có hướng. Byte in ra
`dang_di` (đã sắp theo alphabet để đọc ổn định) trước mỗi lần thử một
hàng xóm:

```python title=readonly
bang_ke_vong = {
    "An": ["Bình"],
    "Bình": ["Chi"],
    "Chi": ["An"],
    "Dung": ["Bình"],
}

def co_chu_trinh(bang_ke, dinh, dang_di, da_xong):
    dang_di.add(dinh)
    print(f"đang trên đường đi: {sorted(dang_di)}")
    for hang_xom in bang_ke[dinh]:
        if hang_xom in dang_di:
            print(f"  {hang_xom} đang trên đường đi hiện tại -> có chu trình!")
            return True
        if hang_xom not in da_xong:
            if co_chu_trinh(bang_ke, hang_xom, dang_di, da_xong):
                return True
    dang_di.remove(dinh)
    da_xong.add(dinh)
    return False

def co_vong_nao_khong(bang_ke):
    dang_di = set()
    da_xong = set()
    for dinh in bang_ke:
        if dinh not in da_xong:
            if co_chu_trinh(bang_ke, dinh, dang_di, da_xong):
                return True
    return False

print(f"Có chu trình: {co_vong_nao_khong(bang_ke_vong)}")
```

```text title=readonly
đang trên đường đi: ['An']
đang trên đường đi: ['An', 'Bình']
đang trên đường đi: ['An', 'Bình', 'Chi']
  An đang trên đường đi hiện tại -> có chu trình!
Có chu trình: True
```

Đường đi lớn dần: `An`, rồi `Bình`, rồi `Chi` — ba lượt gọi CHƯA quay
lui, cả ba còn nguyên trên `dang_di`. Tới lượt của `Chi`, hàng xóm của
nó là `An` — và `An` VẪN còn trong `dang_di` (lượt gọi của `An` chưa hề
trả về, nó đang treo lửng chờ `Bình` chờ `Chi`). Đó chính là chu trình:
đường đi vừa khép kín, quay lại đúng đỉnh nó xuất phát mà chưa từng rời
khỏi. Đỉnh `Dung`, dù có mặt trong `bang_ke`, không hề được nhắc tới —
vì `co_vong_nao_khong` trả về `True` NGAY khi tìm thấy chu trình đầu
tiên, không cần kiểm hết mọi đỉnh còn lại.
::::

::::predict{#doan-da-xong-khong-phai-vong commitOnce}
Một đồ thị KHÁC — không có chu trình nào — nhưng có hai đường cùng dẫn
tới một đỉnh:

```python
bang_ke = {
    "An": ["Bình"],
    "Bình": ["Chi"],
    "Chi": [],
    "Dung": ["Bình"],
}
```

`Bình` được trỏ tới bởi CẢ `An` LẪN `Dung`. Giả sử `co_vong_nao_khong`
xử lý xong nhánh bắt đầu từ `An` trước (đi hết `An → Bình → Chi`, không
có chu trình, cả ba đỉnh đã vào `da_xong`), rồi mới xử lý `Dung`.

**Trước khi chạy**, bạn đoán: khi lượt gọi từ `Dung` thử tới hàng xóm
`Bình` — một đỉnh nó đã "gặp" ở nhánh trước — điều gì xảy ra?

:::opt{correct}
Không có gì bất thường — `Bình` lúc này nằm trong `da_xong` (đã xử lý
xong hẳn), không phải `dang_di`, nên điều kiện chu trình không khớp; kết
quả cuối cùng là `False`, không có chu trình nào
:::

:::opt
`co_vong_nao_khong` báo `True` — `Bình` đã bị "gặp lại", đúng dấu hiệu
của một chu trình
::why
Gần đúng ở việc bạn đúng SỰ KIỆN: `Bình` thật sự bị gặp lại lần thứ hai,
qua hai đường khác nhau — quan sát đó không sai.

Chỗ lệch: không phải MỌI lần "gặp lại" đều là chu trình — chỉ gặp lại
một đỉnh còn ĐANG TRÊN ĐƯỜNG ĐI (`dang_di`) mới tính. Khi lượt gọi từ
`Dung` thử tới `Bình`, lượt gọi CŨ của `Bình` (từ nhánh `An`) đã TRẢ VỀ
từ lâu — `Bình` đã bị gỡ khỏi `dang_di` và chuyển hẳn sang `da_xong`.
Dòng `if hang_xom in dang_di` không khớp, dòng `if hang_xom not in
da_xong` cũng không khớp (`Bình` đã có trong đó) — nên DFS không đi vào
`Bình` lần nữa, không có gì được coi là chu trình.
::
:::

:::opt
Máy báo lỗi, vì `Bình` không thể vừa nằm trong `da_xong` vừa được một
đỉnh khác trỏ tới
::why
Gần đúng ở việc bạn cảm thấy có gì đó "kỳ lạ" khi một đỉnh bị trỏ tới
hai lần — cảm giác đó dễ hiểu nếu quen với luật CÂY (mỗi nút chỉ một
cha).

Chỗ lệch: đây là ĐỒ THỊ, không phải cây — T3.2 đã dạy đồ thị cho phép
một đỉnh có nhiều đỉnh trỏ tới nó, hoàn toàn hợp lệ. Không dòng nào
trong `co_chu_trinh` kiểm tra hay cấm việc này; nó chỉ đơn giản KHÔNG đi
vào `Bình` lần nữa vì `Bình` đã có trong `da_xong`, không có lỗi nào nổ
ra cả.
::
:::

:::opt
Kết quả phụ thuộc THỨ TỰ xử lý — nếu `Dung` được xử lý TRƯỚC `An`, kết
quả sẽ khác, có thể thành `True`
::why
Gần đúng ở việc bạn nghi ngờ thứ tự xử lý có thể ảnh hưởng gì đó — nghi
ngờ đó hợp lý với nhiều thuật toán khác nơi thứ tự thật sự đổi kết quả.

Chỗ lệch: với PHÁT HIỆN CHU TRÌNH thì không. Một chu trình là một tính
chất của chính HÌNH DẠNG đồ thị — nó tồn tại hoặc không, không phụ thuộc
DFS bắt đầu từ đỉnh nào hay xử lý đỉnh nào trước. Nếu `Dung` được xử lý
trước `An`, DFS chỉ đơn giản đi `Dung → Bình → Chi` trước rồi mới quay
lại xử lý `An` (khi đó `Bình` đã `da_xong`, DFS từ `An` chỉ ghé `Bình`
rồi dừng ngay) — kết quả cuối cùng vẫn là `False`, vì đồ thị này thật sự
không có chu trình nào, bất kể thứ tự duyệt.
::
:::
::::

::::code{#viet-co-vong-nao-khong}
Hàm `co_chu_trinh` — phần khó nhất, phân biệt `dang_di` với `da_xong` —
đã viết sẵn hoàn chỉnh. Việc của bạn: hoàn thiện `co_vong_nao_khong`, áp
nó lên TỪNG đỉnh chưa xử lý — cần thiết vì đồ thị có thể gồm nhiều mảnh
rời nhau, và chu trình có thể nằm ở bất kỳ mảnh nào.

```python title=starter
bang_ke_vong = {
    "An": ["Bình"],
    "Bình": ["Chi"],
    "Chi": ["An"],
    "Dung": ["Bình"],
}

bang_ke_khong_vong = {
    "An": ["Bình"],
    "Bình": ["Chi"],
    "Chi": [],
    "Dung": ["Bình"],
}

def co_chu_trinh(bang_ke, dinh, dang_di, da_xong):
    dang_di.add(dinh)
    for hang_xom in bang_ke[dinh]:
        if hang_xom in dang_di:
            return True
        if hang_xom not in da_xong:
            if co_chu_trinh(bang_ke, hang_xom, dang_di, da_xong):
                return True
    dang_di.remove(dinh)
    da_xong.add(dinh)
    return False

def co_vong_nao_khong(bang_ke):
    dang_di = ___                          # tập rỗng — chưa đỉnh nào đang trên đường đi
    da_xong = set()
    for dinh in bang_ke:
        if dinh not in da_xong:
            if ___:                        # kiểm chu trình bắt đầu từ dinh -> có thì dừng ngay
                return True
    return False

print(f"bang_ke_vong có chu trình: {co_vong_nao_khong(bang_ke_vong)}")
print(f"bang_ke_khong_vong có chu trình: {co_vong_nao_khong(bang_ke_khong_vong)}")
```

```python title=solution
bang_ke_vong = {
    "An": ["Bình"],
    "Bình": ["Chi"],
    "Chi": ["An"],
    "Dung": ["Bình"],
}

bang_ke_khong_vong = {
    "An": ["Bình"],
    "Bình": ["Chi"],
    "Chi": [],
    "Dung": ["Bình"],
}

def co_chu_trinh(bang_ke, dinh, dang_di, da_xong):
    dang_di.add(dinh)
    for hang_xom in bang_ke[dinh]:
        if hang_xom in dang_di:
            return True
        if hang_xom not in da_xong:
            if co_chu_trinh(bang_ke, hang_xom, dang_di, da_xong):
                return True
    dang_di.remove(dinh)
    da_xong.add(dinh)
    return False

def co_vong_nao_khong(bang_ke):
    dang_di = set()
    da_xong = set()
    for dinh in bang_ke:
        if dinh not in da_xong:
            if co_chu_trinh(bang_ke, dinh, dang_di, da_xong):
                return True
    return False

print(f"bang_ke_vong có chu trình: {co_vong_nao_khong(bang_ke_vong)}")
print(f"bang_ke_khong_vong có chu trình: {co_vong_nao_khong(bang_ke_khong_vong)}")
```

```python title=test
assert co_vong_nao_khong(bang_ke_vong) == True, "bang_ke_vong có chu trình An -> Bình -> Chi -> An thật sự — phải trả về True"
assert co_vong_nao_khong(bang_ke_khong_vong) == False, "bang_ke_khong_vong không có chu trình nào (Chi không trỏ đi đâu cả) — dù Bình bị trỏ tới hai lần, đó không phải chu trình — phải trả về False"
```

:::hints
- kind: attention
  body: Chỗ trống 1 là giá trị KHỞI TẠO của dang_di — nó phải hỗ trợ .add(...) và .remove(...), đúng như co_chu_trinh bên trên đang dùng. Chỗ trống 2 phải THẬT SỰ gọi co_chu_trinh cho đỉnh đang xét trong vòng lặp — đây là chỗ áp thuật toán lên TỪNG đỉnh còn chưa xử lý.
- kind: strategy
  body: 'dang_di phải bắt đầu RỖNG — set(). Kiểm chu trình bắt đầu từ một đỉnh cụ thể bằng đúng lời gọi co_chu_trinh(bang_ke, dinh, dang_di, da_xong) — bốn tham số đúng thứ tự hàm đã định nghĩa ở trên.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `set()` và `co_chu_trinh(bang_ke, dinh, dang_di, da_xong)`.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: chỗ trống 1 phải THẬT SỰ tạo một tập rỗng bằng set() — không gõ cứng True, 1, 0, những giá trị không có .add(...); chỗ trống 2 phải THẬT SỰ gọi co_chu_trinh cho đỉnh đang xét — không phải một câu luôn đúng hay luôn sai
  requireAst:
  - kind: uses-call, target: set, min: 2
  - kind: uses-call, target: co_chu_trinh, min: 2
  # Đếm thật trên solution: set( xuất hiện đúng 2 lần — "da_xong = set()" (có
  # sẵn trong khung) cộng chỗ trống 1. co_chu_trinh( xuất hiện đúng 2 lần —
  # lời gọi đệ quy BÊN TRONG chính co_chu_trinh (có sẵn, đã cho hoàn chỉnh,
  # không blank) cộng lời gọi ở chỗ trống 2. ĐÃ THỬ THẬT bằng kiemAst: điền
  # True/1/0 vào chỗ trống 1 làm set( tụt xuống 1 (dưới 2) — chặn được; điền
  # vào chỗ trống 2 làm co_chu_trinh( tụt xuống 1 (dưới 2) — chặn được. ĐÃ
  # CHẠY THỬ BA CÁCH ĐIỀN BỪA True/1/0 cho CẢ HAI chỗ trống CÙNG LÚC trên
  # chính khối này (bắt buộc: co_chu_trinh là một hàm đệ quy, dù bản thân nó
  # không bị blank, chỗ trống 2 quyết định nó có được GỌI hay không): với
  # True/1 (giá trị đúng), "if True:" luôn đúng ngay tại ĐỈNH ĐẦU TIÊN của
  # bang_ke, trả về True ngay lập tức — kết thúc AN TOÀN, cực nhanh, nhưng
  # SAI cho bang_ke_khong_vong (mong đợi False, ra True) — bị tests bắt. Với 0
  # (giá trị sai), "if 0:" không bao giờ đúng — co_chu_trinh KHÔNG BAO GIỜ
  # được gọi qua chỗ trống này — vòng lặp for chỉ chạy hết các khoá của
  # bang_ke (hữu hạn, 4 đỉnh) rồi trả về False — kết thúc AN TOÀN, không lặp
  # vô hạn, nhưng SAI cho bang_ke_vong (mong đợi True, ra False) — bị tests
  # bắt. Không cách điền nào trong ba cách chạm tới đệ quy vô hạn, vì cả ba
  # đều KHÔNG hề thay đổi thân của co_chu_trinh (hàm đó nguyên vẹn, không bị
  # blank) — chỉ ảnh hưởng có GỌI nó hay không ở tầng ngoài cùng.
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: "^bang_ke_vong có chu trình: True\\nbang_ke_khong_vong có chu trình: False\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
An, Bình, Chi khép thành một vòng — và Bình bị trỏ tới hai lần ở đồ thị
kia mà chẳng hề tạo ra chu trình nào. Khác nhau đúng một chữ: "đang đi"
hay "đã xong".
::::

::::reflect{#nghi-lai}
Một câu hỏi khép lại cả cụm sáu bài vừa qua.

Từ BFS tới DFS, từ tìm đường ngắn nhất tới phát hiện chu trình — mọi
thuật toán trong cụm này đều xét HẾT mọi lựa chọn hợp lệ tại mỗi bước:
BFS giữ cả một lớp đỉnh chờ xử lý, DFS thử từng hàng xóm một, quay lui
khi cần, nhưng không bỏ sót hướng nào.

Có một lớp thuật toán hoàn toàn khác: ở mỗi bước, nó chỉ nhìn cái TỐT
NHẤT ngay-lúc-này, làm luôn, và KHÔNG BAO GIỜ quay lại xem xét lựa chọn
đã bỏ qua — dù sau này có thể hối tiếc. Nhanh hơn nhiều, vì không cần
thử hết mọi khả năng. Nhưng có phải lúc nào cũng ra đáp án đúng như BFS
và DFS vẫn đảm bảo không?

Bài sau vào việc.
::::

::::checkpoint{mastery=0.8}
::::
