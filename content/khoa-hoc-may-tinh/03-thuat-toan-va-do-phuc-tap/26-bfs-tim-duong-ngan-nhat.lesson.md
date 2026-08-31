---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.bfs-tim-duong-ngan-nhat
title: "BFS tìm đường đi ngắn nhất — theo SỐ CẠNH"
summary: "Vì BFS lan đều từng lớp, đỉnh nào được thăm ở lớp thứ k chắc chắn cách đỉnh gốc ĐÚNG k cạnh, không hơn — ghi lại khoảng cách ngay lúc phát hiện một đỉnh, không cần thử hết mọi đường đi."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.bfs-shortest-path]
requires: [alg.bfs]
concepts: [alg.bfs-shortest-path]
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
Thứ tự BFS thăm không hề ngẫu nhiên. Nó tự nói ra khoảng cách, nếu bạn
biết cách nghe.
::::

::::explain{#lop-thu-may-la-canh-thu-may}
Bài trước để lại câu hỏi: `Bình` (lớp 1) thăm trước `Dung` (lớp 2) —
đúng vì `Bình` gần `An` hơn, hay chỉ là trùng hợp?

Không trùng hợp. Nhớ lại đúng luật BFS: một đỉnh chỉ được đẩy vào hàng
đợi khi nó là hàng xóm của một đỉnh VỪA được lấy ra xử lý. Đỉnh xuất
phát nằm ở lớp 0. Mọi đỉnh được phát hiện ngay từ lượt xử lý lớp 0 nằm ở
lớp 1 — cách gốc đúng MỘT cạnh. Mọi đỉnh được phát hiện từ lượt xử lý
một đỉnh lớp 1 nằm ở lớp 2 — đi qua đúng một đỉnh lớp 1 để tới, tức cách
gốc đúng HAI cạnh. Cứ thế: lớp thứ `k` luôn cách gốc đúng `k` cạnh,
không hơn.

Đây không phải một quan sát may mắn trên một ví dụ nhỏ — nó là hệ quả
BẮT BUỘC của cách hàng đợi hoạt động. Hàng đợi không bao giờ để một đỉnh
lớp `k+1` chen vào hàng TRƯỚC một đỉnh lớp `k`, vì đỉnh lớp `k+1` chỉ có
thể được đẩy vào SAU khi đỉnh lớp `k` phát hiện ra nó — mà "đẩy vào sau"
trên một hàng đợi luôn có nghĩa "xử lý sau". Vì vậy: **BFS thăm các đỉnh
theo đúng thứ tự KHÔNG GIẢM của khoảng cách tới gốc, tính bằng số
cạnh.** Không cần thử hết mọi đường đi có thể có giữa hai đỉnh — chỉ cần
tin đúng thứ tự lan ra của hàng đợi.

Ghi lại khoảng cách rất rẻ: một `dict` `khoang_cach`, gán `0` cho đỉnh
xuất phát, và mỗi khi phát hiện một hàng xóm mới, gán cho nó đúng
`khoang_cach[dinh] + 1` — khoảng cách của đỉnh vừa xử lý, cộng thêm một
cạnh để tới hàng xóm này.
::::

::::example{#do-khoang-cach-nam-dinh}
Cùng mạng năm đỉnh bài trước, giờ ghi thêm khoảng cách:

```python title=readonly
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}

def bfs_khoang_cach(bang_ke, bat_dau):
    da_tham = {bat_dau}
    khoang_cach = {bat_dau: 0}
    hang_doi = [bat_dau]
    while hang_doi:
        dinh = hang_doi.pop(0)
        for hang_xom in bang_ke[dinh]:
            if hang_xom not in da_tham:
                da_tham.add(hang_xom)
                khoang_cach[hang_xom] = khoang_cach[dinh] + 1
                hang_doi.append(hang_xom)
    return khoang_cach

kc = bfs_khoang_cach(bang_ke, "An")
for dinh in bang_ke:
    print(f"{dinh}: cách An {kc[dinh]} cạnh")
```

```text title=readonly
An: cách An 0 cạnh
Bình: cách An 1 cạnh
Chi: cách An 1 cạnh
Dung: cách An 2 cạnh
Em: cách An 3 cạnh
```

Đúng thứ tự BFS bài trước thăm — `An`, rồi `Bình`/`Chi`, rồi `Dung`, rồi
`Em` — cũng chính là thứ tự khoảng cách KHÔNG GIẢM: `0, 1, 1, 2, 3`.
Không có con số nào "nhảy cóc" hay "thụt lùi". `Em` — đỉnh cuối cùng
được thăm — cũng là đỉnh XA gốc nhất, đúng ba cạnh: `An → Bình → Dung →
Em`. Không ai đi tính lại con số `3` này bằng cách thử từng đường đi có
thể có — nó rơi ra thẳng từ chính lúc BFS phát hiện ra `Em`.
::::

::::predict{#doan-khoang-cach-them-dinh commitOnce}
Byte thêm một đỉnh `Giang`, nối với cả `Chi` LẪN `Dung`:

```python
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An", "Giang"],
    "Dung": ["Bình", "Em", "Giang"],
    "Em": ["Dung"],
    "Giang": ["Chi", "Dung"],
}
```

`Giang` giờ có HAI đường tới từ `An`: qua `Chi` (2 cạnh: `An → Chi →
Giang`) hoặc qua `Dung` (3 cạnh: `An → Bình → Dung → Giang`).

**Trước khi chạy**, bạn đoán `khoang_cach["Giang"]` sau khi chạy
`bfs_khoang_cach(bang_ke, "An")` là bao nhiêu?

:::opt{correct}
`2` — BFS ghi khoảng cách ngay khi PHÁT HIỆN một đỉnh lần đầu, mà lượt
xử lý `Chi` (cách `An` 1 cạnh) tới trước lượt xử lý `Dung` cùng lớp, nên
`Giang` được phát hiện qua đường ngắn hơn trước
:::

:::opt
`3` — vì `Giang` cũng là hàng xóm của `Dung`, và đường qua `Dung` mới là
đường "đúng luật" thăm cây từ gốc xuống
::why
Gần đúng ở việc bạn để ý đúng: `Giang` THẬT SỰ là hàng xóm của cả `Chi`
lẫn `Dung`, cả hai cạnh đó đều có trong `bang_ke`.

Chỗ lệch: không có khái niệm "đường đúng luật" nào ở đây cả — BFS chỉ
ghi khoảng cách ở lần ĐẦU TIÊN một đỉnh được phát hiện, và không bao giờ
ghi đè lại sau đó (dòng `if hang_xom not in da_tham` chặn mọi lần phát
hiện sau). `Chi` nằm ở LỚP 1 (hàng xóm trực tiếp của `An`); `Dung` nằm ở
LỚP 2 (chỉ tới được qua `Bình`). BFS xử lý xong TOÀN BỘ lớp 1 rồi mới
đụng tới lớp 2 — nên `Chi` chắc chắn được xử lý trước `Dung`, bất kể thứ
tự giữa hai đỉnh cùng lớp 1 ra sao. Khi `Chi` xử lý trước và phát hiện
`Giang`, con số `2` được ghi ngay, khoá lại.
::
:::

:::opt
Cả hai con số `2` VÀ `3` cùng được ghi — `khoang_cach["Giang"]` sẽ là
một `list` chứa cả hai
::why
Gần đúng ở việc bạn nhận ra có HAI đường thật sự dẫn tới `Giang` — quan
sát đó đúng, đồ thị này thật sự có hai đường.

Chỗ lệch: `khoang_cach[hang_xom] = ...` là một phép GÁN, không phải một
phép CỘNG DỒN — nó ghi ĐÈ giá trị hiện tại. Nhưng dòng gán đó chỉ chạy
đúng MỘT lần cho mỗi đỉnh, vì nó nằm bên trong khối `if hang_xom not in
da_tham` — sau lần đầu, đỉnh đã được đánh dấu `da_tham`, lần phát hiện
thứ hai (qua `Dung`) bị chặn lại hoàn toàn, không chạm tới dòng gán này
nữa.
::
:::

:::opt
Máy báo lỗi, vì `Giang` có hai cha khác nhau — điều luật đồ thị không
cho phép
::why
Gần đúng ở việc bạn nhớ đúng một luật CÓ THẬT — nhưng đó là luật của
CÂY (T3.2 bài 34: một nút chỉ có đúng một cha), không phải của đồ thị.

Chỗ lệch: đồ thị KHÔNG cấm một đỉnh có nhiều đường tới nó — đó chính là
lý do đồ thị tổng quát hơn cây. `Giang` có hai hàng xóm hoàn toàn hợp
lệ, không có luật nào bị vi phạm, và không dòng nào trong `bfs_khoang_
cach` ném lỗi vì chuyện này.
::
:::
::::

::::code{#viet-bfs-khoang-cach}
Hoàn thiện `bfs_khoang_cach`. Vòng lặp hàng đợi đã viết sẵn — việc của
bạn là khởi tạo khoảng cách của đỉnh xuất phát, và tính khoảng cách cho
mỗi hàng xóm mới phát hiện.

```python title=starter
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}

def bfs_khoang_cach(bang_ke, bat_dau):
    da_tham = {bat_dau}
    khoang_cach = {bat_dau: ___}          # đỉnh xuất phát cách chính nó 0 cạnh
    hang_doi = [bat_dau]
    while hang_doi:
        dinh = hang_doi.pop(0)
        for hang_xom in bang_ke[dinh]:
            if hang_xom not in da_tham:
                da_tham.add(hang_xom)
                khoang_cach[hang_xom] = ___    # cách dinh đúng MỘT cạnh nữa
                hang_doi.append(hang_xom)
    return khoang_cach

print(f"Khoảng cách từ An: {bfs_khoang_cach(bang_ke, 'An')}")
```

```python title=solution
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}

def bfs_khoang_cach(bang_ke, bat_dau):
    da_tham = {bat_dau}
    khoang_cach = {bat_dau: 0}
    hang_doi = [bat_dau]
    while hang_doi:
        dinh = hang_doi.pop(0)
        for hang_xom in bang_ke[dinh]:
            if hang_xom not in da_tham:
                da_tham.add(hang_xom)
                khoang_cach[hang_xom] = khoang_cach[dinh] + 1
                hang_doi.append(hang_xom)
    return khoang_cach

print(f"Khoảng cách từ An: {bfs_khoang_cach(bang_ke, 'An')}")
```

```python title=test
kc = bfs_khoang_cach(bang_ke, "An")
assert kc == {"An": 0, "Bình": 1, "Chi": 1, "Dung": 2, "Em": 3}, f"khoảng cách từ An phải đúng {{'An': 0, 'Bình': 1, 'Chi': 1, 'Dung': 2, 'Em': 3}} — đang ra {kc}"
assert kc["Em"] == 3, f"Em xa An nhất trên mạng này, đúng ba cạnh (An-Bình-Dung-Em) — đang ra {kc['Em']}"
```

:::hints
- kind: attention
  body: Chỗ trống 1 là một con số cố định, không phải một phép tính. Chỗ trống 2 phải tham chiếu tới khoảng cách của DINH — đỉnh vừa xử lý — không phải một con số gõ cứng, vì khoảng cách của mỗi đỉnh khác nhau tuỳ nó ở lớp nào.
- kind: strategy
  body: 'Đỉnh xuất phát cách chính nó 0 cạnh — chỗ trống 1 là 0. Một hàng xóm mới cách gốc đúng thêm MỘT cạnh so với đỉnh vừa phát hiện ra nó — chỗ trống 2 là khoang_cach[dinh] + 1, lấy khoảng cách đã biết của dinh rồi cộng thêm một cạnh.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `0` và `khoang_cach[dinh] + 1`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống 2 phải THẬT SỰ tính dựa trên khoang_cach[dinh] và cộng thêm một cạnh — không gõ cứng một con số cố định, vì khoảng cách của mỗi hàng xóm phụ thuộc khoảng cách của đỉnh vừa phát hiện ra nó
  requireAst:
  - kind: uses-name, target: khoang_cach, min: 3
  - kind: uses-name, target: dinh, min: 2
  # Đếm thật trên solution (toàn bộ script): uses-name (đếm chỗ ĐỌC, kể cả vế
  # trái của một phép gán vào Ô — a[b] = c vẫn đọc tên a) cho khoang_cach = 3
  # lần: "khoang_cach = {bat_dau: 0}" (Store, không tính), "khoang_cach[hang_xom]
  # = ..." (Load cho tên gốc khoang_cach), "khoang_cach[dinh]" bên vế phải, và
  # "return khoang_cach" — đếm thật bằng ast cho ra đúng 3. dinh(Load) = 2:
  # "bang_ke[dinh]" (khung có sẵn) và "khoang_cach[dinh]" (chỗ trống 2). ĐÃ THỬ
  # THẬT: điền True/1/0 cho CẢ HAI chỗ trống cùng lúc làm khoang_cach(Load) tụt
  # xuống 2 (dưới 3) và dinh(Load) tụt xuống 1 (dưới 2) — luật chặn được, VÀ
  # chạy AN TOÀN, không lặp vô hạn (vòng lặp hàng đợi không phụ thuộc hai chỗ
  # trống này), chỉ ra kết quả sai (mọi khoảng cách đều bằng giá trị điền bừa),
  # bị tests bắt độc lập.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Khoảng cách từ An: \\{'An': 0, 'Bình': 1, 'Chi': 1, 'Dung': 2, 'Em': 3\\}\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không thử một đường đi nào cả — chỉ tin đúng thứ tự hàng đợi lan ra, mà
vẫn đo đúng khoảng cách ngắn nhất tới từng đỉnh.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

BFS luôn lan ĐỀU ra mọi hướng cùng lúc — xử lý hết lớp 1 rồi mới sang
lớp 2, không thiên vị nhánh nào. Cách đó đúng để đo khoảng cách, nhưng
nó cũng có nghĩa: nếu bạn chỉ cần biết "có đường nào tới được đích hay
không" — không cần biết CHÍNH XÁC bao nhiêu cạnh — BFS vẫn phải giữ
TOÀN BỘ một lớp trong hàng đợi cùng lúc trước khi được phép đi tiếp.

Có một cách đi khác hẳn: chọn MỘT nhánh, đi thẳng theo nó tới tận cùng,
rồi mới quay lại thử nhánh khác — không cần giữ cả một lớp cùng lúc.
Cách đó có tên riêng, và nó không dùng hàng đợi.

Bài sau vào việc.
::::

::::checkpoint{mastery=0.8}
::::
