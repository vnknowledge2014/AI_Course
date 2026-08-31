---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.bfs-hay-dfs
title: "BFS hay DFS — chọn theo câu hỏi đang hỏi"
summary: "Hỏi 'đường ngắn nhất có mấy cạnh' thì BFS trả lời được ngay khi vừa chạm đích, đảm bảo đúng; DFS thì không — nó có thể đi lạc theo một nhánh dài trước. Hỏi 'có đường nào tới được không' thì cả hai đều trả lời được, DFS thường tốn ít bộ nhớ hơn."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 29
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [alg.bfs-vs-dfs]
requires: [alg.bfs-shortest-path, alg.dfs-recursive]
concepts: [alg.bfs-vs-dfs]
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
Cả hai đều thăm hết đồ thị. Nhưng "thăm hết" không phải câu hỏi duy nhất
người ta hay hỏi một đồ thị.
::::

::::explain{#tuy-cau-hoi-dang-hoi}
Không khái niệm thuật toán mới nào ở đây — chỉ là ĐẶT ĐÚNG câu hỏi trước
khi chọn công cụ.

**"Đường ngắn nhất tới đích có mấy cạnh?"** — BFS trả lời được, VÀ đảm
bảo đúng, ngay khi nó vừa lấy đích ra khỏi hàng đợi để xử lý. Bài "BFS
tìm đường đi ngắn nhất" đã chứng minh: BFS thăm các đỉnh theo đúng thứ
tự KHÔNG GIẢM của khoảng cách — nên đỉnh đích, một khi đã bị lấy ra, chắc
chắn đã được lấy ra ở đúng lớp gần nhất có thể, không có đường nào ngắn
hơn còn sót lại phía sau để kiểm tra thêm. DFS thì không có đảm bảo đó:
nó có thể đi lạc theo một nhánh RẤT DÀI trước khi tình cờ chạm tới đích
— và ngay cả khi chạm tới, không có cách nào biết đường nó vừa đi có
phải đường NGẮN NHẤT hay không, vì DFS không hề xử lý các đỉnh theo thứ
tự khoảng cách.

**"Có đường nào tới được đích hay không?"** — câu hỏi này không quan
tâm khoảng cách, chỉ cần biết CÓ hay KHÔNG. Cả hai đều trả lời đúng, vì
cả hai đều thăm hết mọi đỉnh tới được. Khác biệt nằm ở CHI PHÍ: hàng đợi
BFS có thể phải giữ CẢ MỘT LỚP đỉnh cùng lúc — với một đồ thị "rộng"
(một đỉnh có rất nhiều hàng xóm), lớp đó có thể rất lớn. Ngăn xếp/chồng
gọi của DFS chỉ cần giữ đúng ĐƯỜNG ĐI HIỆN TẠI — dài nhất bằng độ SÂU đồ
thị, thường nhỏ hơn nhiều so với độ RỘNG một lớp. Vì lý do đó, DFS
thường tốn ít bộ nhớ hơn khi chỉ cần biết "có đường hay không".
::::

::::example{#dem-buoc-tim-chi}
Cùng mạng năm đỉnh quen thuộc, đo bằng chính công cụ cụm 2 đã dạy — một
biến đếm bước — xem BFS và DFS mất bao nhiêu ĐỈNH ĐÃ XỬ LÝ mới chạm tới
`Chi`:

```python title=readonly
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}

def bfs_tim(bang_ke, bat_dau, dich):
    da_tham = {bat_dau}
    hang_doi = [bat_dau]
    so_buoc = 0
    while hang_doi:
        dinh = hang_doi.pop(0)
        so_buoc += 1
        if dinh == dich:
            return so_buoc
        for hang_xom in bang_ke[dinh]:
            if hang_xom not in da_tham:
                da_tham.add(hang_xom)
                hang_doi.append(hang_xom)
    return so_buoc

def dfs_tim(bang_ke, bat_dau, dich):
    da_tham = set()
    ngan_xep = [bat_dau]
    so_buoc = 0
    while ngan_xep:
        dinh = ngan_xep.pop()
        if dinh in da_tham:
            continue
        da_tham.add(dinh)
        so_buoc += 1
        if dinh == dich:
            return so_buoc
        for hang_xom in reversed(bang_ke[dinh]):
            if hang_xom not in da_tham:
                ngan_xep.append(hang_xom)
    return so_buoc

print(f"BFS tới Chi mất {bfs_tim(bang_ke, 'An', 'Chi')} bước")
print(f"DFS tới Chi mất {dfs_tim(bang_ke, 'An', 'Chi')} bước")
```

```text title=readonly
BFS tới Chi mất 3 bước
DFS tới Chi mất 5 bước
```

`Chi` là hàng xóm TRỰC TIẾP của `An` — cách gốc đúng một cạnh. BFS chạm
tới nó ở bước thứ 3 (sau `An`, `Bình`), vì `Chi` nằm ở lớp gần nhất. DFS
lại lao thẳng vào nhánh dài `Bình → Dung → Em` trước — đúng kỷ luật "đi
tới cùng rồi mới quay lại" — và chỉ chạm `Chi` ở bước CUỐI CÙNG, bước
thứ 5, sau khi đã đi lạc hết cả một nhánh không liên quan gì tới đích.
::::

::::predict{#doan-tim-em commitOnce}
Cùng đúng đoạn mã trên, nhưng đổi đích thành `"Em"` — đỉnh XA gốc nhất
trên mạng này (ba cạnh: `An → Bình → Dung → Em`), và tình cờ cũng chính
là đỉnh nằm SÂU NHẤT trên nhánh DFS lao vào đầu tiên.

**Trước khi chạy**, bạn đoán: lần này BFS hay DFS mất ÍT BƯỚC hơn để
chạm `Em`?

:::opt{correct}
DFS mất ít bước hơn (4, so với 5 của BFS) — lần này `Em` nằm ngay trên
nhánh DFS lao thẳng vào đầu tiên, nên nó KHÔNG hề đi lạc; BFS thì vẫn
phải xử lý hết mọi đỉnh gần hơn trước khi tới lượt `Em`
:::

:::opt
BFS vẫn ít bước hơn — bài học vừa rồi cho thấy BFS luôn nhanh hơn DFS
trong việc TÌM một đỉnh, không riêng gì `Chi`
::why
Gần đúng ở việc bạn tổng quát hoá từ đúng một ví dụ đã thấy — thói quen
đó thường hợp lý khi mẫu số đủ nhiều.

Chỗ lệch: ví dụ trước chỉ chứng minh BFS nhanh hơn cho MỘT đích cụ thể —
`Chi`, một đỉnh GẦN gốc mà lại nằm NGOÀI nhánh DFS chọn đi trước. Bài
học thật của BFS không phải "luôn nhanh hơn khi tìm một đỉnh" — nó là
"luôn tìm ra đường NGẮN NHẤT khi tìm thấy". Tốc độ tìm THẤY phụ thuộc
đích nằm ở ĐÂU so với thứ tự thăm của mỗi thuật toán, và với `Em` — nằm
đúng trên nhánh DFS ưu tiên — DFS lại nhanh hơn.
::
:::

:::opt
Cả hai mất số bước bằng nhau — 4 — vì `Em` là đỉnh cuối cùng trên mạng,
không thuật toán nào "bỏ qua" được nó
::why
Gần đúng ở việc `Em` đúng là đỉnh XA nhất, nằm ở "rìa" của mạng — quan
sát về VỊ TRÍ của `Em` không sai.

Chỗ lệch: "nằm ở rìa" không có nghĩa cả hai thuật toán TỐN CÔNG như
nhau để chạm tới nó. BFS phải xử lý HẾT `An`, `Bình`, `Chi`, `Dung` (bốn
đỉnh) trước khi tới lượt `Em` — vì `Em` ở lớp xa nhất, BFS luôn để nó
lại sau cùng. DFS thì lao thẳng vào đúng nhánh chứa `Em` ngay từ đầu,
không hề ghé qua `Chi` — con số bước vì vậy khác nhau thật.
::
:::

:::opt
Không xác định được nếu không biết trước hình dạng CHÍNH XÁC của đồ
thị, vì tốc độ tìm luôn phụ thuộc dữ liệu
::why
Gần đúng ở việc kết quả THẬT SỰ phụ thuộc hình dạng đồ thị — quan sát
tổng quát đó không sai, và đúng tinh thần "đo trước khi tin" của cả
track này.

Chỗ lệch: hình dạng đồ thị ở đây ĐÃ được biết đầy đủ — đúng mạng năm
đỉnh quen thuộc, đích là `Em`. Với dữ liệu cụ thể này, kết quả hoàn toàn
xác định được bằng cách chạy đúng hai hàm đã viết — không cần thêm
thông tin nào khác. "Phụ thuộc dữ liệu" là lý do để KHÔNG generalize ẩu
sang đồ thị khác, không phải lý do để từ chối trả lời cho chính đồ thị
đang có trước mắt.
::
:::
::::

::::code{#so-sanh-bfs-dfs}
Hoàn thiện hai hàm tìm-và-đếm-bước. Khung `so_buoc` và vòng lặp đã viết
sẵn — việc của bạn là đúng điều kiện dừng sớm: NGAY khi đỉnh vừa xử lý
chính là đích.

```python title=starter
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}

def bfs_tim(bang_ke, bat_dau, dich):
    da_tham = {bat_dau}
    hang_doi = [bat_dau]
    so_buoc = 0
    while hang_doi:
        dinh = hang_doi.pop(0)
        so_buoc += 1
        if ___:                       # đỉnh vừa xử lý chính là đích -> dừng, trả về so_buoc
            return so_buoc
        for hang_xom in bang_ke[dinh]:
            if hang_xom not in da_tham:
                da_tham.add(hang_xom)
                hang_doi.append(hang_xom)
    return so_buoc

def dfs_tim(bang_ke, bat_dau, dich):
    da_tham = set()
    ngan_xep = [bat_dau]
    so_buoc = 0
    while ngan_xep:
        dinh = ngan_xep.pop()
        if dinh in da_tham:
            continue
        da_tham.add(dinh)
        so_buoc += 1
        if ___:                       # đỉnh vừa xử lý chính là đích -> dừng, trả về so_buoc
            return so_buoc
        for hang_xom in reversed(bang_ke[dinh]):
            if hang_xom not in da_tham:
                ngan_xep.append(hang_xom)
    return so_buoc

print(f"BFS tới Chi mất {bfs_tim(bang_ke, 'An', 'Chi')} bước")
print(f"DFS tới Chi mất {dfs_tim(bang_ke, 'An', 'Chi')} bước")
print(f"BFS tới Em mất {bfs_tim(bang_ke, 'An', 'Em')} bước")
print(f"DFS tới Em mất {dfs_tim(bang_ke, 'An', 'Em')} bước")
```

```python title=solution
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}

def bfs_tim(bang_ke, bat_dau, dich):
    da_tham = {bat_dau}
    hang_doi = [bat_dau]
    so_buoc = 0
    while hang_doi:
        dinh = hang_doi.pop(0)
        so_buoc += 1
        if dinh == dich:
            return so_buoc
        for hang_xom in bang_ke[dinh]:
            if hang_xom not in da_tham:
                da_tham.add(hang_xom)
                hang_doi.append(hang_xom)
    return so_buoc

def dfs_tim(bang_ke, bat_dau, dich):
    da_tham = set()
    ngan_xep = [bat_dau]
    so_buoc = 0
    while ngan_xep:
        dinh = ngan_xep.pop()
        if dinh in da_tham:
            continue
        da_tham.add(dinh)
        so_buoc += 1
        if dinh == dich:
            return so_buoc
        for hang_xom in reversed(bang_ke[dinh]):
            if hang_xom not in da_tham:
                ngan_xep.append(hang_xom)
    return so_buoc

print(f"BFS tới Chi mất {bfs_tim(bang_ke, 'An', 'Chi')} bước")
print(f"DFS tới Chi mất {dfs_tim(bang_ke, 'An', 'Chi')} bước")
print(f"BFS tới Em mất {bfs_tim(bang_ke, 'An', 'Em')} bước")
print(f"DFS tới Em mất {dfs_tim(bang_ke, 'An', 'Em')} bước")
```

```python title=test
assert bfs_tim(bang_ke, "An", "Chi") == 3, f"BFS tới Chi phải mất đúng 3 bước — đang ra {bfs_tim(bang_ke, 'An', 'Chi')}"
assert dfs_tim(bang_ke, "An", "Chi") == 5, f"DFS tới Chi phải mất đúng 5 bước — đang ra {dfs_tim(bang_ke, 'An', 'Chi')}"
assert bfs_tim(bang_ke, "An", "Em") == 5, f"BFS tới Em phải mất đúng 5 bước — đang ra {bfs_tim(bang_ke, 'An', 'Em')}"
assert dfs_tim(bang_ke, "An", "Em") == 4, f"DFS tới Em phải mất đúng 4 bước — đang ra {dfs_tim(bang_ke, 'An', 'Em')}"
```

:::hints
- kind: attention
  body: Cả hai chỗ trống làm ĐÚNG MỘT việc — so sánh đỉnh vừa xử lý (dinh) với đích (dich) — chỉ khác nằm trong hàm nào. Đây là một PHÉP SO SÁNH thật, không phải một câu luôn đúng hay luôn sai.
- kind: strategy
  body: 'So một đỉnh có phải chính đích hay không dùng đúng toán tử == đã quen — dinh == dich. Cả hai chỗ trống viết giống hệt nhau, vì cả hai hàm đều dùng cùng tên biến dinh và dich.'
- kind: one-line
  body: 'Cả hai chỗ trống đều là `dinh == dich`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải THẬT SỰ so sánh đỉnh vừa xử lý với đích bằng dinh == dich — không phải một câu luôn đúng hay luôn sai như True, 1, 0, vì bài này đang đo THỜI ĐIỂM chạm đích, không phải chỉ có chạm được hay không
  requireAst:
  - kind: uses-operator, target: "==", min: 2
  # Đếm thật trên solution: phép so sánh == xuất hiện đúng 2 lần trong toàn bộ
  # script — một lần trong mỗi hàm (bfs_tim, dfs_tim), đúng khớp hai chỗ trống.
  # ĐÃ THỬ THẬT bằng kiemAst: điền True/1/0 vào MỘT hoặc CẢ HAI chỗ trống làm
  # số phép so sánh == tụt xuống dưới 2 — luật chặn được. ĐÃ CHẠY THỬ BA CÁCH
  # ĐIỀN BỪA True/1/0 cho CẢ HAI chỗ trống cùng lúc (bắt buộc vì khối có while):
  # điền True/1 khiến CẢ HAI hàm trả về so_buoc = 1 ngay ở đỉnh xuất phát, cho
  # mọi đích — dừng AN TOÀN, rất nhanh (vòng lặp chỉ chạy đúng một lượt), sai
  # so với 3/5/5/4 mong đợi, bị tests bắt. Điền 0 khiến điều kiện luôn sai —
  # không dừng sớm, vòng lặp chạy tới khi CẠN cấu trúc (hàng đợi/ngăn xếp rỗng
  # tự nhiên, vì đồ thị hữu hạn năm đỉnh) rồi trả về tổng số bước thật (5 cho
  # cả hai) — dừng AN TOÀN, không lặp vô hạn, vẫn sai so với giá trị mong đợi
  # ở ít nhất một trong bốn phép gọi, bị tests bắt.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^BFS tới Chi mất 3 bước\\nDFS tới Chi mất 5 bước\\nBFS tới Em mất 5 bước\\nDFS tới Em mất 4 bước\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chi gần hơn với BFS. Em gần hơn với DFS. Không ai thắng tuyệt đối — chỉ
có đúng công cụ cho đúng câu hỏi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

DFS vừa lao thẳng vào một chu trình tiềm ẩn cũng dễ như lao vào một
nhánh dài vô hại — nó không hề biết trước đường nó đang đi có QUAY LẠI
một đỉnh đã ghé qua hay không, cho tới khi thực sự chạm phải. T3.2 từng
gặp đúng vấn đề này ở một chỗ hoàn toàn khác: hai cái nồi trỏ vào nhau,
khiến cách đếm thẻ tham chiếu thông thường không bao giờ đưa được số
đếm về không.

Nếu DFS đang đi trên một đường mà đỉnh nó VỪA GỌI TỚI lại chính là một
đỉnh nó CHƯA RỜI KHỎI trên con đường hiện tại — không phải một đỉnh đã
thăm xong từ lâu — điều đó nói lên chuyện gì về hình dạng của đồ thị
đang duyệt?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
