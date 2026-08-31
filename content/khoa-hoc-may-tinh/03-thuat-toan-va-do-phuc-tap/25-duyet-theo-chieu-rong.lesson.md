---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.duyet-theo-chieu-rong
title: "Duyệt theo chiều rộng: lan ra từng lớp một"
summary: "Bắt đầu từ một đỉnh, dùng đúng hàng đợi FIFO T3.2 đã dựng để thăm hết hàng xóm trực tiếp trước khi sang hàng xóm của hàng xóm — đẩy hàng xóm mới vào cuối, lấy đỉnh cần thăm tiếp từ đầu, lan ra từng lớp một."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [alg.bfs]
requires: [ds.queue, ds.graph-representation]
concepts: [alg.bfs]
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
Không còn một dãy phẳng nữa. Hôm nay là một mạng lưới — và câu hỏi là:
thăm hết nó theo thứ tự nào?
::::

::::explain{#lan-ra-tung-lop}
T3.2 dựng sẵn hai công cụ đúng lúc này cần dùng lại. Bài "Hàng đợi: vào
trước, ra trước" dựng **hàng đợi** — thêm vào ĐUÔI bằng `.append(x)`, lấy
ra ở ĐẦU bằng `.pop(0)`, ai chờ lâu nhất được xử lý trước. Bài "Biểu diễn
đồ thị: bảng kề hay ma trận kề" dựng **bảng kề** — một `dict` mà mỗi đỉnh
trỏ tới `list` các đỉnh hàng xóm của nó.

Ghép đúng hai công cụ đó lại là được một cách đi thăm hết mọi đỉnh của một
đồ thị, gọi là **duyệt theo chiều rộng** (Breadth-First Search, viết tắt
BFS). Luật của nó:

1. Đưa đỉnh xuất phát vào hàng đợi.
2. Lặp lại tới khi hàng đợi rỗng: lấy đỉnh ở ĐẦU hàng đợi ra, ghi nhận đã
   thăm nó, rồi đẩy TẤT CẢ hàng xóm CHƯA THĂM của nó vào CUỐI hàng đợi.

Vì hàng đợi luôn trả đỉnh đợi lâu nhất trước, các hàng xóm TRỰC TIẾP của
điểm xuất phát — được đẩy vào sớm nhất — luôn được xử lý trước các hàng
xóm-của-hàng-xóm — được đẩy vào muộn hơn, sau khi mọi hàng xóm trực tiếp
đã có mặt trong hàng đợi. Kết quả là một cách thăm "lan ra từng lớp": lớp
0 là chính đỉnh xuất phát, lớp 1 là hàng xóm trực tiếp của nó, lớp 2 là
hàng xóm của hàng xóm, cứ thế lan rộng dần — không đào sâu vào bất kỳ
nhánh nào trước khi lan hết lớp hiện tại.

Có một chi tiết bắt buộc, không phải tuỳ chọn: một tập `da_tham` (các
đỉnh ĐÃ đẩy vào hàng đợi rồi), đánh dấu NGAY khi đẩy, không đợi tới lúc
xử lý xong. Thiếu nó, một đỉnh có nhiều đường tới — hoặc một đồ thị có
chu trình — sẽ bị đẩy vào hàng đợi lặp đi lặp lại, không bao giờ dừng.
::::

::::example{#lan-tren-mang-nam-dinh}
Byte duyệt một mạng năm đỉnh, in ra hàng đợi trước mỗi lượt lấy ra, để
nhìn thấy nó lan rộng rồi co lại thế nào:

```python title=readonly
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}

def bfs(bang_ke, bat_dau):
    da_tham = {bat_dau}
    hang_doi = [bat_dau]
    thu_tu_tham = []
    while hang_doi:
        print(f"hàng đợi hiện tại: {hang_doi}")
        dinh = hang_doi.pop(0)
        thu_tu_tham.append(dinh)
        for hang_xom in bang_ke[dinh]:
            if hang_xom not in da_tham:
                da_tham.add(hang_xom)
                hang_doi.append(hang_xom)
    return thu_tu_tham

print(f"Thứ tự thăm: {bfs(bang_ke, 'An')}")
```

```text title=readonly
hàng đợi hiện tại: ['An']
hàng đợi hiện tại: ['Bình', 'Chi']
hàng đợi hiện tại: ['Chi', 'Dung']
hàng đợi hiện tại: ['Dung']
hàng đợi hiện tại: ['Em']
Thứ tự thăm: ['An', 'Bình', 'Chi', 'Dung', 'Em']
```

Nhìn dòng thứ hai: ngay sau khi thăm `An`, hàng đợi có ĐÚNG hai đỉnh —
`Bình` và `Chi`, cả hai hàng xóm trực tiếp của `An`, đúng "lớp 1". Đỉnh
`Dung` — hàng xóm của `Bình`, tức "lớp 2" — chỉ xuất hiện ở dòng thứ ba,
SAU KHI cả hai đỉnh lớp 1 đã có mặt trong hàng đợi. `Dung` không hề chen
ngang trước `Chi`, dù nó được phát hiện ngay từ lượt xử lý `Bình` — hàng
đợi buộc nó phải đợi tới lượt, đúng kỷ luật vào trước ra trước.
::::

::::predict{#doan-thu-tu-bfs commitOnce}
Cũng mạng năm đỉnh như trên, gọi `bfs(bang_ke, "An")`:

```python
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}
```

**Trước khi chạy**, bạn đoán `thu_tu_tham` cuối cùng là gì?

:::opt{correct}
`['An', 'Bình', 'Chi', 'Dung', 'Em']` — hai hàng xóm trực tiếp của `An`
(lớp 1) thăm trước, rồi mới tới `Dung` và `Em` (lớp 2, lớp 3)
:::

:::opt
`['An', 'Chi', 'Bình', 'Dung', 'Em']` — `Chi` được thêm sau `Bình` nên
xử lý trước, đúng thứ tự "mới nhất trước"
::why
Gần đúng ở việc năm đỉnh này đều có mặt, không sót không thừa — quan sát
đó không sai.

Chỗ lệch: "mới thêm vào thì xử lý trước" là luật của NGĂN XẾP (`.pop()`
không tham số), không phải hàng đợi. Hàng đợi dùng `.pop(0)` — luôn lấy
đỉnh đợi LÂU NHẤT, tức đỉnh được thêm vào SỚM NHẤT. `Bình` được đẩy vào
trước `Chi` trong cùng một lượt xử lý `An`, nên `Bình` phải được xử lý
trước `Chi`, không phải ngược lại.
::
:::

:::opt
`['An', 'Bình', 'Dung', 'Em', 'Chi']` — đi thẳng theo nhánh `An-Bình-
Dung-Em` cho hết đường, rồi mới quay lại xử lý `Chi`
::why
Gần đúng ở việc năm đỉnh này đều có mặt, không sót không thừa — quan sát
đó không sai.

Chỗ lệch: thứ tự này đào sâu vào MỘT nhánh trước khi thử nhánh khác — đó
là kỷ luật của một cấu trúc dữ liệu khác hẳn hàng đợi. Hàng đợi buộc
`Chi` (lớp 1, hàng xóm trực tiếp của `An`) phải được xử lý trước `Dung`
và `Em` (lớp 2, lớp 3) — không có chuyện đào sâu một nhánh trước khi lan
hết lớp hiện tại.
::
:::

:::opt
`['An', 'Bình', 'Chi', 'An', 'Dung', 'Em']` — `An` bị thăm lại lần hai,
vì nó cũng là hàng xóm của `Bình`
::why
Gần đúng ở bốn đỉnh đầu — `An`, `Bình`, `Chi` đúng là ba đỉnh được thăm
trước tiên, thứ tự đó không sai.

Chỗ lệch: `An` không hề bị đẩy vào hàng đợi lần hai. `da_tham` đánh dấu
`An` NGAY khi nó được đẩy vào hàng đợi lần đầu (trước cả khi vòng lặp
bắt đầu) — nên khi `Bình` liệt kê `An` là hàng xóm, điều kiện
`hang_xom not in da_tham` chặn nó lại ngay, không đẩy thêm một lần nào
nữa. Mỗi đỉnh chỉ xuất hiện đúng một lần trong `thu_tu_tham`.
::
:::
::::

::::code{#viet-bfs}
Hoàn thiện `bfs`. Việc đánh dấu `da_tham` và kiểm tra hàng xóm đã viết
sẵn — việc của bạn là đúng hai thao tác hàng đợi: lấy đỉnh tiếp theo cần
xử lý, và đẩy một hàng xóm mới phát hiện vào hàng đợi.

```python title=starter
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}

def bfs(bang_ke, bat_dau):
    da_tham = {bat_dau}
    hang_doi = [bat_dau]
    thu_tu_tham = []
    while hang_doi:
        dinh = ___                        # lấy đỉnh đợi LÂU NHẤT ra khỏi hàng đợi
        thu_tu_tham.append(dinh)
        for hang_xom in bang_ke[dinh]:
            if hang_xom not in da_tham:
                da_tham.add(hang_xom)
                ___                        # đẩy hàng xóm mới phát hiện vào CUỐI hàng đợi
    return thu_tu_tham

print(f"Thứ tự BFS từ An: {bfs(bang_ke, 'An')}")
```

```python title=solution
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}

def bfs(bang_ke, bat_dau):
    da_tham = {bat_dau}
    hang_doi = [bat_dau]
    thu_tu_tham = []
    while hang_doi:
        dinh = hang_doi.pop(0)
        thu_tu_tham.append(dinh)
        for hang_xom in bang_ke[dinh]:
            if hang_xom not in da_tham:
                da_tham.add(hang_xom)
                hang_doi.append(hang_xom)
    return thu_tu_tham

print(f"Thứ tự BFS từ An: {bfs(bang_ke, 'An')}")
```

```python title=test
ket_qua = bfs(bang_ke, "An")
assert ket_qua == ["An", "Bình", "Chi", "Dung", "Em"], f"BFS từ An trên mạng này phải cho đúng ['An', 'Bình', 'Chi', 'Dung', 'Em'] — hai hàng xóm trực tiếp (Bình, Chi) thăm trước cả Dung lẫn Em — đang ra {ket_qua}"
assert len(ket_qua) == 5, "phải thăm đủ cả năm đỉnh, không bỏ sót và không lặp lại đỉnh nào"
assert ket_qua[0] == "An", "đỉnh xuất phát phải là đỉnh ĐẦU TIÊN trong thứ tự thăm"
```

:::hints
- kind: attention
  body: Chỗ trống 1 lấy phần tử ra khỏi hàng đợi — đúng công cụ bài "Hàng đợi" đã dựng, không phải .pop() không tham số (đó là ngăn xếp). Chỗ trống 2 đẩy VÀO hàng đợi — dùng .append(...), không phải .insert(0, ...).
- kind: strategy
  body: 'Hàng đợi lấy ra ở ĐẦU bằng .pop(0) — chỗ trống 1 là hang_doi.pop(0). Hàng đợi thêm vào ở ĐUÔI bằng .append(x) — chỗ trống 2 là hang_doi.append(hang_xom), đẩy đúng hàng xóm vừa được đánh dấu ở dòng trên.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `hang_doi.pop(0)` và `hang_doi.append(hang_xom)`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống 1 phải thật sự lấy đỉnh ra khỏi hàng đợi bằng .pop(0) — đúng kỷ luật FIFO của hàng đợi; chỗ trống 2 phải thật sự đẩy hàng xóm mới vào hàng đợi bằng .append(...) — không phải một câu không làm gì như True, 1, 0
  requireAst:
  - kind: uses-call, target: pop, min: 1
  - kind: uses-call, target: append, min: 2
  # Đếm thật trên solution: .pop( xuất hiện đúng 1 lần (chỗ trống 1, dòng
  # "dinh = hang_doi.pop(0)"). .append( xuất hiện đúng 2 lần — một lần đã có
  # sẵn trong khung (thu_tu_tham.append(dinh)), cộng một lần ở chỗ trống 2
  # (hang_doi.append(hang_xom)). ĐÃ THỬ THẬT bằng kiemAst: điền True/1/0 (một
  # câu không làm gì) vào MỘT trong hai chỗ trống làm con số tương ứng tụt
  # xuống dưới ngưỡng — pop còn 0 (dưới 1), hoặc append còn 1 (dưới 2) — luật
  # này chặn được cả hai trường hợp độc lập.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Thứ tự BFS từ An: \\['An', 'Bình', 'Chi', 'Dung', 'Em'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai đỉnh lớp 1 xong trước khi Dung — lớp 2 — kịp chen vào. Hàng đợi
không bao giờ để một đỉnh xa hơn chen ngang trước một đỉnh gần hơn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại ví dụ: `Bình` được thăm ở lượt thứ hai, `Dung` được thăm ở lượt
thứ tư. `Bình` là hàng xóm TRỰC TIẾP của `An` — cách `An` đúng một cạnh.
`Dung` chỉ tới được qua `Bình` — cách `An` đúng hai cạnh.

Thứ tự BFS thăm chúng — `Bình` trước, `Dung` sau — có phải NGẪU NHIÊN
không, hay chính THỨ TỰ THĂM đó đã tự nó chứng minh được `Bình` gần `An`
hơn `Dung`, mà không cần đếm cạnh bằng tay?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
