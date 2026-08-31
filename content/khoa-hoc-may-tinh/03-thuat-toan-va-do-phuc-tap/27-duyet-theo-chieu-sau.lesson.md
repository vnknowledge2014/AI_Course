---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.duyet-theo-chieu-sau
title: "Duyệt theo chiều sâu: đi thẳng tới cùng rồi mới quay lại"
summary: "Đối lập BFS: đi theo MỘT nhánh tới khi hết đường mới quay lại thử nhánh khác, đúng kỷ luật vào-sau-ra-trước của ngăn xếp T3.2 dựng — cùng dữ liệu đầu vào, đổi hàng đợi thành ngăn xếp cho ra hẳn một thứ tự thăm khác."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [alg.dfs]
requires: [ds.stack, ds.graph-representation]
concepts: [alg.dfs]
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
Không giữ cả một lớp cùng lúc nữa. Đi thẳng theo MỘT hướng cho tới hết
đường, rồi mới quay lại.
::::

::::explain{#doi-hang-doi-thanh-ngan-xep}
Bài trước để lại câu hỏi: có cách đi nào không cần giữ cả một lớp trong
hàng đợi cùng lúc? Có — đổi đúng MỘT thứ trong công thức BFS: thay hàng
đợi bằng **ngăn xếp** (bài "Ngăn xếp: vào sau, ra trước", T3.2).

Luật giống hệt BFS ở khung sườn — đẩy đỉnh xuất phát vào, lặp lại tới
khi rỗng: lấy một đỉnh ra, ghi nhận đã thăm, đẩy hàng xóm chưa thăm vào
— chỉ khác đúng chỗ LẤY RA. Ngăn xếp trả về phần tử vừa đẩy vào GẦN ĐÂY
NHẤT (`.pop()` không tham số), không phải phần tử đợi lâu nhất. Kết quả:
ngay khi một đỉnh vừa phát hiện ra một hàng xóm mới, hàng xóm đó — vừa
được đẩy lên TRÊN CÙNG ngăn xếp — sẽ được xử lý ở LƯỢT KẾ TIẾP, trước cả
những đỉnh đã chờ từ lâu. Đây chính là "đi thẳng tới cùng": vừa phát
hiện một hướng mới là lao vào nó ngay, không chờ xử lý xong những hướng
đã biết trước đó. Cách đi này gọi là **duyệt theo chiều sâu** (Depth-
First Search, viết tắt DFS).

Có một chi tiết khác BFS, và nó không phải tuỳ chọn nếu muốn DFS cho ra
đúng CÙNG một thứ tự với bản đệ quy mà bài sau sẽ viết: đánh dấu
`da_tham` NGAY KHI LẤY RA khỏi ngăn xếp (không phải ngay khi đẩy vào),
và bỏ qua (`continue`) nếu đỉnh vừa lấy ra đã thăm rồi. Lý do: một đỉnh
có thể bị ĐẨY vào ngăn xếp nhiều lần — qua nhiều nhánh khác nhau — trước
khi nó thực sự tới lượt xử lý; đánh dấu quá sớm (ngay lúc đẩy) sẽ làm
sai lệch thứ tự những nhánh còn lại được khám phá. Còn một chi tiết nữa:
đẩy hàng xóm vào theo thứ tự NGƯỢC danh sách kề (`reversed(...)`) — hệt
mẹo T3.2 bài "Duyệt cây" từng dùng (đẩy nhánh phải trước, trái sau) — để
hàng xóm ĐẦU TIÊN trong danh sách kề vẫn được xử lý trước, dù ngăn xếp
lấy ra ngược thứ tự đẩy vào.
::::

::::example{#dfs-tren-mang-nam-dinh}
Cùng đúng mạng năm đỉnh bài trước, giờ duyệt bằng ngăn xếp thay vì hàng
đợi:

```python title=readonly
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}

def dfs(bang_ke, bat_dau):
    da_tham = set()
    ngan_xep = [bat_dau]
    thu_tu_tham = []
    while ngan_xep:
        print(f"ngăn xếp hiện tại: {ngan_xep}")
        dinh = ngan_xep.pop()
        if dinh in da_tham:
            continue
        da_tham.add(dinh)
        thu_tu_tham.append(dinh)
        for hang_xom in reversed(bang_ke[dinh]):
            if hang_xom not in da_tham:
                ngan_xep.append(hang_xom)
    return thu_tu_tham

print(f"Thứ tự thăm: {dfs(bang_ke, 'An')}")
```

```text title=readonly
ngăn xếp hiện tại: ['An']
ngăn xếp hiện tại: ['Chi', 'Bình']
ngăn xếp hiện tại: ['Chi', 'Dung']
ngăn xếp hiện tại: ['Chi', 'Em']
ngăn xếp hiện tại: ['Chi']
Thứ tự thăm: ['An', 'Bình', 'Dung', 'Em', 'Chi']
```

So với BFS bài trước — `['An', 'Bình', 'Chi', 'Dung', 'Em']` — đây là
một thứ tự HOÀN TOÀN KHÁC: `['An', 'Bình', 'Dung', 'Em', 'Chi']`. Sau
khi thăm `An`, DFS lao thẳng vào `Bình` — hàng xóm ĐẦU TIÊN của `An` —
rồi từ `Bình` lao tiếp vào `Dung`, rồi `Em`, đi hết cả một nhánh dài tới
tận cùng. Chỉ khi nhánh đó cạn (`Em` không còn hàng xóm chưa thăm nào),
ngăn xếp mới lộ ra `Chi` — đỉnh đã bị "bỏ quên" từ tận lượt đầu tiên,
nằm im ở đáy ngăn xếp suốt cả quãng đường. Cùng một mạng, cùng một đỉnh
xuất phát — chỉ đổi hàng đợi thành ngăn xếp mà thứ tự thăm đổi hẳn.
::::

::::predict{#doan-thu-tu-dfs commitOnce}
Cùng mạng năm đỉnh, nhưng đổi thứ tự liệt kê hàng xóm của `An`:

```python
bang_ke = {
    "An": ["Chi", "Bình"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}
```

Chỉ đổi `["Bình", "Chi"]` thành `["Chi", "Bình"]` — `Chi` giờ đứng ĐẦU
danh sách hàng xóm của `An`.

**Trước khi chạy**, bạn đoán `dfs(bang_ke, "An")` cho ra thứ tự nào?

:::opt{correct}
`['An', 'Chi', 'Bình', 'Dung', 'Em']` — DFS luôn lao vào hàng xóm ĐẦU
TIÊN trong danh sách kề trước; đổi thứ tự liệt kê đổi luôn thứ tự thăm
:::

:::opt
Vẫn `['An', 'Bình', 'Dung', 'Em', 'Chi']` — đổi thứ tự liệt kê hàng xóm
không ảnh hưởng gì, vì tập hàng xóm của `An` vẫn y hệt như cũ
::why
Gần đúng ở việc TẬP hàng xóm của `An` — `{Bình, Chi}` — đúng là không
đổi, dù liệt kê theo thứ tự nào.

Chỗ lệch: DFS không chỉ quan tâm TẬP hàng xóm, nó quan tâm cả THỨ TỰ
liệt kê chúng, vì dòng `reversed(bang_ke[dinh])` đẩy vào ngăn xếp theo
thứ tự ngược lại — hàng xóm ĐẦU TIÊN trong danh sách kề luôn nằm ở đỉnh
ngăn xếp SAU khi đẩy hết, nên nó luôn được `.pop()` ra ĐẦU TIÊN. Đổi
`Chi` lên đầu danh sách nghĩa là `Chi` giờ được thăm trước `Bình`, đảo
ngược đúng cặp đó so với ví dụ gốc.
::
:::

:::opt
`['An', 'Bình', 'Dung', 'Em', 'Chi']` không đổi, vì DFS luôn thăm theo
thứ tự BẢNG CHỮ CÁI của tên đỉnh khi có nhiều lựa chọn
::why
Gần đúng ở việc bạn nhận ra CÓ một quy tắc quyết định DFS chọn hàng xóm
nào trước — quan sát đó đúng hướng.

Chỗ lệch: quy tắc đó không phải bảng chữ cái. DFS thăm theo đúng THỨ TỰ
liệt kê trong `bang_ke[dinh]` — thứ tự do NGƯỜI VIẾT `bang_ke` quyết
định khi gõ danh sách, không liên quan gì tới chữ cái của tên đỉnh.
`bang_ke["An"] = ["Chi", "Bình"]` không hề được Python tự sắp lại theo
alphabet.
::
:::

:::opt
`['An', 'Chi', 'Dung', 'Em', 'Bình']` — `Chi` thăm trước, nhưng nó cũng
"kéo theo" cả nhánh `Dung`, `Em` đi cùng vì chúng cùng gần `Bình`
::why
Gần đúng ở việc bạn đúng đỉnh THỨ HAI trong thứ tự thăm — `Chi` đúng là
được xử lý ngay sau `An`.

Chỗ lệch: `Chi` không hề "kéo theo" nhánh nào cả, vì `Chi` chỉ có đúng
một hàng xóm — `An`, đã thăm rồi. Ngay sau khi xử lý `Chi`, ngăn xếp chỉ
còn lại `Bình` (đẩy từ lượt xử lý `An`) — DFS quay sang thăm `Bình`, rồi
mới từ `Bình` lao vào nhánh `Dung → Em`. `Dung` và `Em` chỉ tới được QUA
`Bình`, không có đường nào từ `Chi`.
::
:::
::::

::::code{#viet-dfs}
Hoàn thiện `dfs`. Vòng lặp và mẹo `reversed(...)` đã viết sẵn — việc của
bạn là đúng hai thao tác ngăn xếp: lấy đỉnh ở trên cùng ra, và đẩy một
hàng xóm chưa thăm lên ngăn xếp.

```python title=starter
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}

def dfs(bang_ke, bat_dau):
    da_tham = set()
    ngan_xep = [bat_dau]
    thu_tu_tham = []
    while ngan_xep:
        dinh = ___                        # lấy đỉnh ở TRÊN CÙNG ngăn xếp
        if dinh in da_tham:
            continue
        da_tham.add(dinh)
        thu_tu_tham.append(dinh)
        for hang_xom in reversed(bang_ke[dinh]):
            if hang_xom not in da_tham:
                ___                        # đẩy hàng xóm CHƯA THĂM lên ngăn xếp
    return thu_tu_tham

print(f"Thứ tự DFS từ An: {dfs(bang_ke, 'An')}")
```

```python title=solution
bang_ke = {
    "An": ["Bình", "Chi"],
    "Bình": ["An", "Dung"],
    "Chi": ["An"],
    "Dung": ["Bình", "Em"],
    "Em": ["Dung"],
}

def dfs(bang_ke, bat_dau):
    da_tham = set()
    ngan_xep = [bat_dau]
    thu_tu_tham = []
    while ngan_xep:
        dinh = ngan_xep.pop()
        if dinh in da_tham:
            continue
        da_tham.add(dinh)
        thu_tu_tham.append(dinh)
        for hang_xom in reversed(bang_ke[dinh]):
            if hang_xom not in da_tham:
                ngan_xep.append(hang_xom)
    return thu_tu_tham

print(f"Thứ tự DFS từ An: {dfs(bang_ke, 'An')}")
```

```python title=test
ket_qua = dfs(bang_ke, "An")
assert ket_qua == ["An", "Bình", "Dung", "Em", "Chi"], f"DFS từ An trên mạng này phải cho đúng ['An', 'Bình', 'Dung', 'Em', 'Chi'] — lao thẳng vào nhánh Bình-Dung-Em trước, Chi thăm cuối cùng — đang ra {ket_qua}"
assert len(ket_qua) == 5, "phải thăm đủ cả năm đỉnh, không bỏ sót và không lặp lại đỉnh nào"
assert ket_qua[-1] == "Chi", "Chi chỉ nối với An — nó bị bỏ lại ở đáy ngăn xếp, phải là đỉnh thăm CUỐI CÙNG"
```

:::hints
- kind: attention
  body: Chỗ trống 1 lấy phần tử ra khỏi ngăn xếp — dùng .pop() KHÔNG tham số, khác hẳn .pop(0) của hàng đợi bài trước. Chỗ trống 2 đẩy VÀO ngăn xếp — dùng .append(...), đúng công cụ quen thuộc từ bài "Ngăn xếp".
- kind: strategy
  body: 'Ngăn xếp lấy ra ở TRÊN CÙNG bằng .pop() không tham số — chỗ trống 1 là ngan_xep.pop(). Đẩy một hàng xóm mới vào ngăn xếp cũng dùng .append(x) như mọi cấu trúc dựa trên list — chỗ trống 2 là ngan_xep.append(hang_xom).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `ngan_xep.pop()` và `ngan_xep.append(hang_xom)`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống 1 phải thật sự lấy đỉnh ra khỏi ngăn xếp bằng .pop() KHÔNG tham số — đúng kỷ luật LIFO của ngăn xếp, không phải .pop(0) của hàng đợi; chỗ trống 2 phải thật sự đẩy hàng xóm mới lên ngăn xếp bằng .append(...) — không phải một câu không làm gì như True, 1, 0
  requireAst:
  - kind: uses-call, target: pop, min: 1
  - kind: uses-call, target: append, min: 2
  # Đếm thật trên solution: .pop( xuất hiện đúng 1 lần (chỗ trống 1). .append(
  # xuất hiện đúng 2 lần — một lần có sẵn trong khung (thu_tu_tham.append(dinh)),
  # cộng một lần ở chỗ trống 2. ĐÃ THỬ THẬT: điền True/1/0 vào MỘT trong hai chỗ
  # trống làm con số tương ứng tụt xuống dưới ngưỡng — luật chặn được cả hai
  # trường hợp độc lập. ĐÃ CHẠY THỬ BA CÁCH ĐIỀN BỪA True/1/0 vào CẢ HAI chỗ
  # trống cùng lúc trên chính khối này (bắt buộc vì đây là một while có đệ quy
  # tiềm ẩn qua cấu trúc lặp): dinh nhận giá trị True/1/0 (không phải một chuỗi
  # tên đỉnh), dòng `for hang_xom in reversed(bang_ke[dinh])` tra bang_ke[dinh]
  # với khoá không tồn tại và NÉM KeyError ngay lập tức — dừng AN TOÀN bằng lỗi
  # CÓ báo, không lặp vô hạn, cho cả ba cách điền.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Thứ tự DFS từ An: \\['An', 'Bình', 'Dung', 'Em', 'Chi'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bình, Dung, Em — cả một nhánh đi liền một mạch, và Chi phải đợi tới tận
lượt cuối cùng mới được nhắc tới.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn kỹ ngăn xếp `da_tham`/`ngan_xep` trong ví dụ vừa rồi: đẩy vào, lấy
ra ở TRÊN CÙNG, quay lại nhánh cũ khi nhánh mới cạn đường. Đó chính xác
là cái "máy nhớ đường về" mà R1.T1.3 dạy — và bài "Đệ quy chính là một
ngăn xếp" đầu track này đã chỉ thẳng: chồng lời gọi của một hàm đệ quy
CŨNG là một ngăn xếp, chỉ do trình thông dịch tự quản.

Nếu vậy, một hàm gọi lại chính nó — không cần bạn tự tay `.append`/
`.pop` một `ngan_xep` nào cả — có thể tự nhiên cho ra ĐÚNG CÙNG một thứ
tự thăm như bản ngăn xếp tự quản vừa viết không?

Bài sau kiểm chứng.
::::

::::checkpoint{mastery=0.8}
::::
