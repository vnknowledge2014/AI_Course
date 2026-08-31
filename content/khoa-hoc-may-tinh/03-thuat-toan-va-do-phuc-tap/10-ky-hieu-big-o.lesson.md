---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.ky-hieu-big-o
title: "Ký hiệu Big-O: viết gọn cho hình dạng tăng"
summary: "T3.2 hứa 'nhanh hơn/chậm hơn' giờ có tên chính thức. O(1), O(n), O(n²) là cách viết gọn cho đúng ba hình dạng bài trước vừa đặt tên bằng tiếng Việt — không phải ba khái niệm mới, chỉ là một ký hiệu mới cho thứ đã đo được, và nó cố ý bỏ qua hệ số nhân đứng trước n."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [alg.big-o-notation]
requires: [alg.growth-shapes]
concepts: [alg.big-o-notation]
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
Ba chữ cái, hai dấu ngoặc, một chữ n. Không có gì mới bên trong — chỉ
gọn hơn hẳn để viết ra.
::::

::::explain{#ba-ky-hieu}
Chín bài trước của track này liên tục dùng cụm từ "nhanh hơn", "chậm
hơn" — kể cả tận T3.2, khi so sánh tra cứu bằng bảng băm với tra cứu
bằng cách dò từng mục, lời hứa đã ghi rõ: một ngày nào đó những chữ
"nhanh hơn/chậm hơn" ấy sẽ có tên chính thức. Hôm nay là ngày đó.

Ba hình dạng bài trước đặt tên bằng tiếng Việt, giờ viết bằng một ký
hiệu gọi là **Big-O** — chữ "O" viết hoa, cộng một cặp ngoặc chứa biểu
thức mô tả hình dạng tăng:

- **Hằng số** → **O(1)** — đọc là "O của 1". Số bước không phụ thuộc
  cỡ dữ liệu, nên bên trong ngoặc là con số 1, không chứa `n`.
- **Tuyến tính** → **O(n)** — đọc là "O của n". Chữ `n` bên trong ngoặc
  chính là cỡ dữ liệu — cùng chữ `n` mà mọi bài từ đầu track đã dùng
  để gọi tên số phần tử.
- **Bậc hai** → **O(n²)** — đọc là "O của n bình phương". Số 2 nhỏ phía
  trên chữ `n` là số mũ — đúng dấu hiệu của phép nhân `n` với chính nó.

Đây không phải ba khái niệm mới cần học lại. Big-O chỉ là một CÁCH VIẾT
— thay vì viết cả câu "số bước tăng đúng theo bình phương cỡ dữ liệu",
giờ viết gọn thành `O(n²)`. Nội dung không đổi, chỉ đổi cách viết ra.

Có một điều Big-O CỐ Ý bỏ qua: hệ số nhân đứng trước `n`. Một đoạn mã
tốn `n` bước và một đoạn mã tốn `2n` bước — hay thậm chí `10n` bước —
đều được viết là `O(n)`, không phải `O(2n)` hay `O(10n)`. Lý do: cả ba
đều là một ĐƯỜNG THẲNG khi vẽ theo `n` — chỉ khác độ DỐC của đường
thẳng đó, còn HÌNH DẠNG (đường thẳng, không phải đường cong) là như
nhau. Big-O nói về hình dạng, không nói về độ dốc.
::::

::::example{#gan-nhan-cho-so-lieu-that}
Gắn đúng nhãn Big-O cho ba bảng số liệu đã đo ở hai bài trước:

```python title=readonly
print("O(1)  :", [1, 1, 1, 1])            # không đổi dù n gấp đôi
print("O(n)  :", [10, 20, 40, 80])         # gấp đôi khi n gấp đôi
print("O(n²) :", [100, 400, 1600, 6400])   # gấp bốn khi n gấp đôi
```

```text title=readonly
O(1)  : [1, 1, 1, 1]
O(n)  : [10, 20, 40, 80]
O(n²) : [100, 400, 1600, 6400]
```

Ba nhãn này không đo được gì mới — chúng gọi tên đúng ba dãy số liệu
bài trước đã đo bằng bộ đếm thật. Nhớ lại đoạn mã "hai vòng lặp chạy
NỐI TIẾP, không lồng" ở phần dự đoán bài trước: nó tốn `n + n = 2n`
bước, và theo đúng luật "bỏ hệ số nhân" vừa nêu, nó cũng được viết là
`O(n)` — CÙNG một nhãn với đoạn mã chỉ tốn `n` bước, dù số bước THẬT
của hai đoạn mã khác nhau ở mọi cỡ dữ liệu.
::::

::::predict{#doan-nhan-cho-doan-ma-moi commitOnce}
Một đoạn mã với hai vòng `for` KHÔNG lồng nhau — giống hệt đoạn mã ở
phần dự đoán bài trước:

```python
def hai_vong_doc_lap(mang):
    so_buoc = 0
    for x in mang:
        so_buoc += 1
    for x in mang:
        so_buoc += 1
    return so_buoc
```

Đã biết: đây là hình dạng TUYẾN TÍNH (bài trước đã xác nhận bằng phép
chia tỉ lệ).

**Trước khi đọc tiếp**, bạn đoán: nhãn Big-O ĐÚNG cho đoạn mã này là gì?

:::opt{correct}
O(n) — dù có hai vòng lặp, chúng không lồng nhau nên số bước là `2n`,
không phải `n²`; và Big-O bỏ qua hệ số nhân 2, chỉ giữ lại hình dạng `n`
:::

:::opt
O(n²) — vì đoạn mã có hai vòng lặp, và hai vòng lặp thì phải nhân với
nhau
::why
Gần đúng ở việc đếm đúng số vòng `for` — có hai vòng thật.

Chỗ lệch: `O(n²)` chỉ đúng khi một vòng lặp nằm LỒNG bên trong vòng
kia — mỗi lượt vòng ngoài kéo theo cả một lượt vòng trong, nên nhân
lên. Ở đây hai vòng đứng NGANG HÀNG, chạy nối tiếp — vòng sau bắt đầu
khi vòng trước đã CHẠY XONG HẲN, không có sự lồng nào để nhân cả. Số
lượng vòng lặp không tự nó quyết định số mũ; CÁCH chúng LỒNG NHAU mới
quyết định.
::
:::

:::opt
O(2n) — vì đoạn mã thật sự tốn `2n` bước, và ký hiệu Big-O phải phản
ánh đúng con số đó
::why
Gần đúng ở phép tính `2n` — con số đó không sai, đúng là tổng số bước
thật của đoạn mã này.

Chỗ lệch: Big-O CỐ Ý không viết hệ số nhân ra ngoài — phần giải thích ở
trên đã nêu lý do: `n` và `2n` cùng là một đường thẳng, chỉ khác độ
dốc, và Big-O chỉ quan tâm hình dạng (đường thẳng hay đường cong), bỏ
qua độ dốc. Viết `O(2n)` không sai về mặt toán học, nhưng không đúng
QUY ƯỚC viết Big-O — quy ước luôn rút gọn về `O(n)`.
::
:::

:::opt
O(1) — vì hai vòng lặp "bù trừ" cho nhau nên tổng thể vẫn coi như không
đổi
::why
Gần đúng ở trực giác "hai thứ đối xứng có thể cân bằng nhau" — trực
giác đó có lý ở một số phép toán khác.

Chỗ lệch: giống chính câu hỏi dự đoán ở bài trước đã chỉ ra — cả hai
vòng lặp đều CỘNG THÊM vào `so_buoc`, không vòng nào trừ đi. Số bước
tăng dần theo `n` (2, 4, 6, 8... khi n = 1, 2, 3, 4), không hề đứng
yên, nên không thể là `O(1)`.
::
:::
::::

::::code{#phan-loai-bang-ti-le}
Viết một hàm nhận vào hai con số đã ĐO ĐƯỢC (số bước ở cỡ `n`, và số
bước ở cỡ `2n`), rồi trả về đúng nhãn Big-O dựa trên TỈ LỆ giữa chúng
— đúng phép chia bài trước vừa tự tay làm.

```python title=starter
def phan_loai_big_o(buoc_o_n, buoc_o_2n):
    ti_le = buoc_o_2n / buoc_o_n
    if ti_le == 1:
        return "O(1)"
    elif ti_le == ___:            # tỉ lệ của hình dạng TUYẾN TÍNH
        return "O(n)"
    elif ti_le == ___:            # tỉ lệ của hình dạng BẬC HAI
        return "O(n²)"
    return "không xác định"

def buoc_hang(n):
    return 1

def buoc_tuyen_tinh(n):
    so_buoc = 0
    for x in range(n):
        so_buoc += 1
    return so_buoc

def buoc_bac_hai(n):
    so_buoc = 0
    for x in range(n):
        for y in range(n):
            so_buoc += 1
    return so_buoc

nhan_hang = phan_loai_big_o(buoc_hang(10), buoc_hang(20))
nhan_tuyen_tinh = phan_loai_big_o(buoc_tuyen_tinh(10), buoc_tuyen_tinh(20))
nhan_bac_hai = phan_loai_big_o(buoc_bac_hai(10), buoc_bac_hai(20))

print(f"{nhan_hang} {nhan_tuyen_tinh} {nhan_bac_hai}")
```

```python title=solution
def phan_loai_big_o(buoc_o_n, buoc_o_2n):
    ti_le = buoc_o_2n / buoc_o_n
    if ti_le == 1:
        return "O(1)"
    elif ti_le == 2:
        return "O(n)"
    elif ti_le == 4:
        return "O(n²)"
    return "không xác định"

def buoc_hang(n):
    return 1

def buoc_tuyen_tinh(n):
    so_buoc = 0
    for x in range(n):
        so_buoc += 1
    return so_buoc

def buoc_bac_hai(n):
    so_buoc = 0
    for x in range(n):
        for y in range(n):
            so_buoc += 1
    return so_buoc

nhan_hang = phan_loai_big_o(buoc_hang(10), buoc_hang(20))
nhan_tuyen_tinh = phan_loai_big_o(buoc_tuyen_tinh(10), buoc_tuyen_tinh(20))
nhan_bac_hai = phan_loai_big_o(buoc_bac_hai(10), buoc_bac_hai(20))

print(f"{nhan_hang} {nhan_tuyen_tinh} {nhan_bac_hai}")
```

```python title=test
assert nhan_hang == "O(1)", f"buoc_hang không đổi dù n gấp đôi -> phải phân loại O(1), đang ra {nhan_hang}"
assert nhan_tuyen_tinh == "O(n)", f"buoc_tuyen_tinh gấp đôi khi n gấp đôi -> phải phân loại O(n), đang ra {nhan_tuyen_tinh}"
assert nhan_bac_hai == "O(n²)", f"buoc_bac_hai gấp bốn khi n gấp đôi -> phải phân loại O(n²), đang ra {nhan_bac_hai}"
```

:::hints
- kind: attention
  body: Hai chỗ trống là hai con số — đúng tỉ lệ TĂNG mà bài trước đã tự tay tính bằng phép chia cho hai hình dạng tuyến tính và bậc hai. Không phải đoán bừa; đó là con số đã đo được hai bài liên tiếp trước bài này.
- kind: strategy
  body: 'Tuyến tính: gấp đôi dữ liệu cho tỉ lệ 2 (bài trước đã tính ti_le_tuyen_tinh == 2.0). Bậc hai: gấp đôi dữ liệu cho tỉ lệ 4 (ti_le_bac_hai == 4.0). Điền đúng 2 và 4 vào hai chỗ trống.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `2` và `4`.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: hai chỗ trống phải là đúng hai con số 2 và 4 — tỉ lệ tăng đo được của hình dạng tuyến tính và bậc hai khi dữ liệu gấp đôi, không phải một câu không làm gì như True, 1, 0
  requireAst:
  - kind: has-literal, target: 2, min: 1
  - kind: has-literal, target: 4, min: 1
  # đếm thật trên solution: literal 2 xuất hiện đúng một lần (chỗ trống
  # đầu), literal 4 xuất hiện đúng một lần (chỗ trống sau) — không có
  # số 2 hay số 4 nào khác trong mã (các cỡ dữ liệu dùng là 10 và 20,
  # không phải 2 hay 4). Điền True/1/0 vào MỘT hoặc CẢ HAI chỗ trống
  # xoá mất literal 2 hoặc 4 tương ứng khỏi mã nguồn — luật chặn được.
  # ĐÃ THỬ THẬT cả ba cách True/1/0 cho cả hai chỗ trống cùng lúc trên
  # chính hàm phan_loai_big_o: cả ba dừng NGAY (đây chỉ là một chuỗi
  # if/elif, không có while/đệ quy nào, không có rủi ro lặp vô hạn) và
  # cả ba cho nhan_tuyen_tinh == "không xác định" thay vì "O(n)" (vì
  # ti_le 2.0 không khớp True/1/0 theo đúng nghĩa so sánh số học ở đây
  # — 1 chỉ khớp nhánh O(1) đã có sẵn), sai hẳn, bị tests bắt độc lập.
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: "^O\\(1\\) O\\(n\\) O\\(n²\\)\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
O(1), O(n), O(n²) — ba cái tên tiếng Việt bài trước vừa đặt, giờ có
thêm một cách viết ngắn gọn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Big-O tả một hình dạng tăng CHUNG cho cả một đoạn mã — nhưng một LẦN
CHẠY cụ thể lại có thể may mắn. Ví dụ đoạn mã dò từng ô một danh sách
để tìm một giá trị: nếu giá trị đó tình cờ nằm ngay Ô ĐẦU TIÊN, chỉ
tốn đúng MỘT bước, bất kể danh sách dài bao nhiêu.

Nhãn `O(n)` của đoạn mã dò từng ô ấy có tính đến ca may mắn một-bước đó
không? Nếu không, nó dựa vào ca nào để gọi tên `n`?
::::

::::checkpoint{mastery=0.8}
::::
