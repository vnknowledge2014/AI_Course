---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.truong-hop-xau-nhat
title: "Trường hợp xấu nhất, không phải trường hợp may mắn"
summary: "Dò từng ô một danh sách có thể may mắn thấy ngay ở ô đầu — 1 bước. Big-O nói về ca XẤU NHẤT: thứ tìm nằm ở ô cuối, hoặc không có mặt. Đo bằng ca xấu, không đo bằng ca may."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.worst-case]
requires: [alg.big-o-notation]
concepts: [alg.worst-case]
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
Ca xấu nhất, luôn luôn. Big-O không hứa "thường thì", nó hứa "không
bao giờ tệ hơn thế này".
::::

::::explain{#ca-may-va-ca-xau}
Cùng một đoạn mã, chạy hai lần với hai bộ dữ liệu khác nhau, có thể ra
hai con số bước hoàn toàn khác nhau. Xét đoạn mã dò từng ô một danh
sách tìm một giá trị cụ thể — đi từ ô đầu, so từng ô, dừng lại ngay khi
thấy:

- Nếu giá trị cần tìm nằm ngay Ô ĐẦU TIÊN: chỉ tốn **1 bước**. Đây là
  **trường hợp thuận lợi** — dữ liệu tình cờ xếp theo cách có lợi cho
  lần chạy này.
- Nếu giá trị cần tìm nằm ở Ô CUỐI CÙNG, hoặc KHÔNG CÓ MẶT trong danh
  sách: phải chạm tới **hết mọi ô**, tốn đúng `n` bước — với `n` là số
  phần tử. Đây là **trường hợp xấu nhất** — không còn cách nào tệ hơn
  cho đoạn mã này.

Cùng một đoạn mã, cùng một `n`, mà số bước THẬT có thể dao động từ 1
tới `n` tuỳ dữ liệu cụ thể của lần chạy đó. Nhãn `O(n)` gắn cho đoạn mã
dò từng ô không hề nói "MỌI lần chạy đều tốn `n` bước" — nó nói "KHÔNG
CÓ lần chạy nào tốn NHIỀU HƠN một hằng số nhân với `n` bước". Đó là một
lời hứa về giới hạn TRÊN, không phải một lời hứa về con số chính xác
của mọi lần chạy.

Theo quy ước, khi nói "đoạn mã này là O(n)" mà không nói rõ thêm, quy
ước ngầm định luôn là: đo ở TRƯỜNG HỢP XẤU NHẤT. Lý do rất thực tế —
trường hợp thuận lợi là may mắn, không đảm bảo lần chạy sau cũng may
mắn y vậy. Trường hợp xấu nhất mới là con số bạn CHẮC CHẮN không bị
vượt qua, dù dữ liệu có tệ tới đâu.
::::

::::example{#do-ca-may-va-ca-xau}
Byte đo cả hai trường hợp trên CÙNG một danh sách tám phần tử, chỉ đổi
giá trị cần tìm:

```python title=readonly
def do_tung_o(mang, dich):
    so_buoc = 0
    for i in range(len(mang)):
        so_buoc += 1
        if mang[i] == dich:
            return i, so_buoc
    return -1, so_buoc

mang = [7, 2, 9, 4, 1, 8, 3, 6]

vi_tri, buoc = do_tung_o(mang, 7)      # 7 nằm ngay ô đầu (chỉ số 0)
print(f"tìm 7  -> vị trí {vi_tri}, {buoc} bước")

vi_tri, buoc = do_tung_o(mang, 6)      # 6 nằm ở ô CUỐI (chỉ số 7)
print(f"tìm 6  -> vị trí {vi_tri}, {buoc} bước")

vi_tri, buoc = do_tung_o(mang, 100)    # 100 không có mặt
print(f"tìm 100 -> vị trí {vi_tri}, {buoc} bước")
```

```text title=readonly
tìm 7  -> vị trí 0, 1 bước
tìm 6  -> vị trí 7, 8 bước
tìm 100 -> vị trí -1, 8 bước
```

Ba lần gọi CÙNG một hàm, CÙNG một danh sách tám phần tử — mà số bước
đo được là 1, 8, và 8. Hai trường hợp sau (tìm ở ô cuối, và tìm giá trị
không có mặt) đều tốn đúng `n = 8` bước — đây chính là trường hợp xấu
nhất của đoạn mã này, và đúng bằng nhãn `O(n)` đã gắn cho nó. Trường
hợp đầu tiên tốt hơn hẳn con số đó, nhưng KHÔNG được dùng để đại diện
cho tốc độ của đoạn mã.
::::

::::predict{#doan-ai-dung commitOnce}
Bạn nói với một đồng nghiệp rằng đoạn mã dò từng ô ở trên là `O(n)`.
Đồng nghiệp phản bác: "Không đúng đâu, tôi vừa chạy thử trên một danh
sách 1000 phần tử và nó chỉ mất ĐÚNG 1 bước!"

**Trước khi trả lời**, bạn đoán: ai đúng?

:::opt{correct}
Bạn đúng — con số 1 bước đồng nghiệp đo được là CA THUẬN LỢI (giá trị
cần tìm tình cờ nằm ngay ô đầu). Nhãn O(n) mô tả CA XẤU NHẤT (1000 bước
nếu giá trị nằm ở ô cuối hoặc không có mặt), không phải mọi lần chạy
:::

:::opt
Đồng nghiệp đúng — đo thật trên máy luôn đáng tin hơn một ký hiệu lý
thuyết, và con số 1 là con số đo THẬT
::why
Gần đúng ở việc coi trọng số đo THẬT — cả track này từ đầu tới giờ
luôn nhấn mạnh "đo trước khi tin", và con số 1 bước quả thật là một
số đo có thật, không phải bịa ra.

Chỗ lệch: một số đo thật của MỘT LẦN CHẠY CỤ THỂ không phủ định được
một lời hứa về GIỚI HẠN TRÊN áp dụng cho MỌI lần chạy có thể xảy ra.
`O(n)` không nói "lần nào cũng tốn `n` bước" để đồng nghiệp có thể bác
bỏ bằng một lần chạy nhanh — nó nói "không lần nào tốn nhiều hơn một
hằng số nhân `n` bước", và lần chạy 1 bước đó hoàn toàn nằm TRONG giới
hạn đó, không hề mâu thuẫn với nó.
::
:::

:::opt
Cả hai đều sai — phải nói TRUNG BÌNH bao nhiêu bước, không phải nói ca
tốt nhất hay ca xấu nhất
::why
Gần đúng ở việc "trung bình" là một khái niệm có thật và hữu ích trong
khoa học máy tính — nhiều bài toán thật sự cần tới số đo trung bình,
không chỉ ca xấu nhất.

Chỗ lệch: khi một ai đó nói "đoạn mã này là O(n)" mà không nói thêm gì,
theo đúng quy ước phần giải thích vừa nêu, ngầm định đó luôn là ca xấu
nhất — không phải ca trung bình. Cả bạn lẫn đồng nghiệp đều đang tranh
luận về đúng quy ước đó, và giữa "ca thuận lợi 1 bước" với "ca xấu nhất
n bước", câu trả lời đúng là bạn nói tới ca xấu nhất, không phải cả hai
đều sai.
::
:::

:::opt
Không thể biết ai đúng nếu không biết `n` cụ thể của danh sách khi
tranh luận
::why
Gần đúng ở việc `n` đúng là một phần quan trọng của mọi phép đo Big-O —
không có `n` thì không đo được số bước cụ thể là bao nhiêu.

Chỗ lệch: câu tranh luận ở đây không hỏi "số bước cụ thể là bao nhiêu"
— nó hỏi "nhãn `O(n)` có SAI không khi một lần chạy chỉ tốn 1 bước".
Câu hỏi đó trả lời được mà không cần biết `n` chính xác, vì nó chỉ đòi
hỏi hiểu ĐÚNG nghĩa của nhãn `O(n)`: một giới hạn trên cho ca xấu nhất,
không phải một lời hứa cho mọi lần chạy.
::
:::
::::

::::code{#do-hai-ca-cung-mot-danh-sach}
Đo cả hai trường hợp trên CÙNG một danh sách tám phần tử: ca thuận lợi
(giá trị cần tìm nằm ngay ô đầu) và ca xấu nhất (giá trị cần tìm KHÔNG
có mặt trong danh sách, buộc phải chạm hết mọi ô).

```python title=starter
def do_tung_o(mang, dich):
    so_buoc = 0
    for i in range(len(mang)):
        ___                              # tăng bộ đếm mỗi lần NHÌN vào một ô
        if mang[i] == dich:
            return i, so_buoc
    return -1, so_buoc

mang_tam = [7, 2, 9, 4, 1, 8, 3, 6]

vi_tri_tot, so_buoc_tot = do_tung_o(mang_tam, 7)     # 7 nằm ngay ô đầu
vi_tri_xau, so_buoc_xau = ___                        # tìm một giá trị KHÔNG có trong mang_tam

print(f"ca thuận lợi: {so_buoc_tot} bước, ca xấu nhất: {so_buoc_xau} bước")
```

```python title=solution
def do_tung_o(mang, dich):
    so_buoc = 0
    for i in range(len(mang)):
        so_buoc += 1
        if mang[i] == dich:
            return i, so_buoc
    return -1, so_buoc

mang_tam = [7, 2, 9, 4, 1, 8, 3, 6]

vi_tri_tot, so_buoc_tot = do_tung_o(mang_tam, 7)
vi_tri_xau, so_buoc_xau = do_tung_o(mang_tam, 100)

print(f"ca thuận lợi: {so_buoc_tot} bước, ca xấu nhất: {so_buoc_xau} bước")
```

```python title=test
assert so_buoc_tot == 1, f"7 nằm ngay ô đầu tiên -> phải tốn đúng 1 bước, đang ra {so_buoc_tot}"
assert so_buoc_xau == 8, f"giá trị không có mặt -> phải chạm hết cả tám ô, đang ra {so_buoc_xau}"
assert vi_tri_xau == -1, f"giá trị không có mặt -> vị trí phải là -1, đang ra {vi_tri_xau}"
assert so_buoc_tot < so_buoc_xau, "ca thuận lợi phải tốn ÍT bước hơn hẳn ca xấu nhất trên cùng một danh sách"
```

:::hints
- kind: attention
  body: Chỗ trống đầu nằm BÊN TRONG thân hàm do_tung_o — nó phải tăng bộ đếm ở MỌI ô được nhìn tới, bất kể ô đó có đúng giá trị cần tìm hay không. Chỗ trống sau gọi lại chính hàm do_tung_o, nhưng với một giá trị KHÔNG có trong mang_tam — để ép hàm chạy tới ca xấu nhất.
- kind: strategy
  body: 'Chỗ trống đầu: so_buoc += 1, đặt NGAY ĐẦU thân vòng lặp, trước cả câu if — đúng bài học "đặt bộ đếm sai chỗ là đếm thiếu" từ hai bài trước. Chỗ trống sau: do_tung_o(mang_tam, 100) — hoặc bất kỳ số nào khác không xuất hiện trong [7, 2, 9, 4, 1, 8, 3, 6] — buộc vòng lặp phải chạy hết cả tám ô rồi mới trả về -1.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `so_buoc += 1` và `do_tung_o(mang_tam, 100)`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống đầu phải THẬT SỰ tăng so_buoc mỗi lần nhìn vào một ô; chỗ trống sau phải THẬT SỰ gọi lại do_tung_o với một giá trị không có trong mang_tam — không phải một câu không làm gì như True, 1, 0
  requireAst:
  - kind: gan-ten, target: so_buoc, min: 2
  - kind: uses-call, target: do_tung_o, min: 2
  # gan-ten so_buoc min 2 — đếm thật trên solution: "so_buoc = 0" có sẵn
  # trong khung (1) cộng "so_buoc += 1" ở chỗ trống đầu (2). uses-call
  # do_tung_o min 2 — đếm thật: một lần gọi có sẵn trong khung (đo ca
  # thuận lợi) cộng một lần gọi ở chỗ trống sau (đo ca xấu nhất) = 2.
  # Điền True/1/0 vào chỗ trống đầu tụt gan-ten xuống 1 (<2), chặn được.
  # Điền True/1/0 vào chỗ trống sau — ví dụ
  # `vi_tri_xau, so_buoc_xau = True` — tụt uses-call do_tung_o xuống 1
  # (<2), chặn được; và Python còn nổ TypeError ngay lập tức vì không
  # thể unpack một giá trị bool thành hai biến — một lỗi CÓ báo, không
  # phải lặp vô hạn. ĐÃ THỬ THẬT cả ba cách True/1/0 cho cả hai chỗ
  # trống cùng lúc: cả ba dừng NGAY (vòng lặp trong do_tung_o là for
  # bị chặn bởi range(len(mang)), không phải while/đệ quy, không có
  # rủi ro lặp vô hạn dù chỗ trống đầu bị điền bừa) — hai cách (1, 0)
  # dừng bằng lỗi unpack ở dòng gán thứ hai; cách True cũng dừng bằng
  # cùng lỗi unpack — cả ba đều bị bắt, không cái nào lặng lẽ cho qua.
  # Đã thử thêm một lời giải ĐÚNG khác — gọi do_tung_o(mang_tam, 999)
  # (một số khác 100, vẫn không có mặt) — vẫn đếm đủ 2 lần gọi, luật
  # không đánh trượt nhầm.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^ca thuận lợi: 1 bước, ca xấu nhất: 8 bước\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
1 bước, 8 bước — cùng một đoạn mã, cùng một danh sách, hai câu chuyện
khác hẳn nhau. Đo bằng ca xấu, không đo bằng ca may.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ trong tay có đủ bốn công cụ: đếm bước thật thay vì đoán (bài 7),
nhìn ra hình dạng tăng khi dữ liệu gấp đôi (bài 8-9), gọi tên hình dạng
đó bằng Big-O (bài 10), và biết phải đo ở CA XẤU NHẤT chứ không phải
ca may (vừa xong).

Nếu có HAI đoạn mã khác nhau, viết theo hai cách khác hẳn nhau, nhưng
cùng làm một việc — ghép cả bốn công cụ đó lại thế nào để nói chắc
chắn đoạn nào THUA, không chỉ đoán bằng cảm giác?
::::

::::checkpoint{mastery=0.8}
::::
