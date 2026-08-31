---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.dat-ten-cho-hinh-dang-tang
title: "Đặt tên cho hình dạng tăng: hằng, tuyến tính, bậc hai"
summary: "Ba hình dạng đo được ở bài trước có tên: KHÔNG đổi dù dữ liệu gấp đôi gọi là hằng số; tăng ĐÚNG GẤP ĐÔI theo dữ liệu gọi là tuyến tính; tăng GẤP BỐN khi dữ liệu gấp đôi — vì vòng lặp lồng vòng lặp — gọi là bậc hai."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.growth-shapes]
requires: [alg.growth-observed]
concepts: [alg.growth-shapes]
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
Có tên chứ. Ba cái tên, không phải một.
::::

::::explain{#ba-cai-ten}
Bài trước đo được ba hình dạng tăng bằng con số thật. Giờ đặt tên cho
từng hình dạng, để lần sau gặp lại không cần tả dài dòng nữa.

**Hằng số** (constant): số bước KHÔNG phụ thuộc cỡ dữ liệu. Dù danh
sách có 10 phần tử hay 10 triệu phần tử, đoạn mã chỉ nhìn `mang[0]`
vẫn tốn đúng một bước. "Hằng" ở đây nghĩa là con số ấy đứng yên, không
nhích lên dù dữ liệu phình to cỡ nào.

**Tuyến tính** (linear): số bước tỉ lệ THUẬN với cỡ dữ liệu — gấp đôi
dữ liệu thì gấp đôi số bước, gấp ba dữ liệu thì gấp ba số bước. Một
vòng lặp DUY NHẤT, không lồng, đi qua từng phần tử đúng một lần, luôn
cho hình dạng này. Cái tên "tuyến tính" mượn từ đồ thị: vẽ số bước theo
cỡ dữ liệu, ta được một ĐƯỜNG THẲNG.

**Bậc hai** (quadratic): số bước tỉ lệ với BÌNH PHƯƠNG cỡ dữ liệu — gấp
đôi dữ liệu thì số bước gấp BỐN (vì 2² = 4), gấp ba dữ liệu thì số bước
gấp CHÍN (vì 3² = 9). Hai vòng lặp LỒNG nhau, mỗi vòng đều chạy hết
`n` lượt, sinh ra hình dạng này — đúng đoạn mã bài trước vừa đo.

Ba hình dạng này không phải khái niệm mới. Chúng là chính ba con số đã
đo được ở bài trước, giờ có tên gọi chính thức để dùng lại mà không
phải tả lại từ đầu mỗi lần.
::::

::::example{#nhan-dien-qua-so-lieu-that}
Gom lại ba bảng số liệu đã đo (bài trước và phần giải thích trên) vào
một chỗ, gắn tên cho từng bảng:

```python title=readonly
print("hằng số   :", [1, 1, 1, 1])          # dem_buoc_chi_nhin_dau
print("tuyến tính:", [10, 20, 40, 80])       # dem_buoc_mot_vong
print("bậc hai   :", [100, 400, 1600, 6400]) # dem_buoc_hai_vong
```

```text title=readonly
hằng số   : [1, 1, 1, 1]
tuyến tính: [10, 20, 40, 80]
bậc hai   : [100, 400, 1600, 6400]
```

Nhìn theo hàng dọc từng bảng, mỗi lần cỡ dữ liệu gấp đôi (từ cột này
sang cột kế): hàng "hằng số" đứng yên (1 → 1); hàng "tuyến tính" gấp
đôi (10 → 20 → 40 → 80, mỗi bước đúng gấp đôi bước trước); hàng
"bậc hai" gấp bốn (100 → 400 → 1600 → 6400, mỗi bước đúng gấp bốn bước
trước). Ba cột đầu và cột cuối cách nhau tận tám lần cỡ dữ liệu
(10 → 80), mà con số ở hàng "bậc hai" đã cách nhau tới 6400 lần
(100 → 6400) — khoảng cách giữa các hình dạng chỉ càng lộ rõ khi dữ
liệu càng lớn.
::::

::::predict{#doan-hai-vong-doc-lap commitOnce}
Một đoạn mã có HAI vòng `for` — nhưng không lồng nhau, mà chạy NỐI TIẾP
nhau, mỗi vòng đi qua danh sách một lượt riêng:

```python
def hai_vong_doc_lap(mang):
    so_buoc = 0
    for x in mang:
        so_buoc += 1
    for x in mang:
        so_buoc += 1
    return so_buoc

print(hai_vong_doc_lap(list(range(10))))
print(hai_vong_doc_lap(list(range(20))))
```

**Trước khi chạy**, bạn đoán: đoạn mã này thuộc hình dạng nào?

:::opt{correct}
Tuyến tính — hai dòng in ra 20 rồi 40, và 40 vẫn đúng gấp đôi 20; hai
vòng lặp CHẠY NỐI TIẾP không nhân số bước lên, chỉ CỘNG thêm
:::

:::opt
Bậc hai — vì đoạn mã có HAI vòng lặp, giống hệt đoạn mã bậc hai ở bài
trước cũng có hai vòng lặp
::why
Gần đúng ở việc bạn đếm đúng số vòng `for` xuất hiện trong mã — đúng là
có hai vòng, không phải một.

Chỗ lệch nằm ở CÁCH hai vòng lặp đó QUAN HỆ với nhau. Đoạn mã bậc hai ở
bài trước có vòng trong nằm BÊN TRONG vòng ngoài — LỒNG nhau, nên mỗi
lượt của vòng ngoài kéo theo trọn một lượt của vòng trong, nhân lên.
Ở đây hai vòng `for` đứng NGANG HÀNG nhau, vòng sau chạy sau khi vòng
trước đã chạy xong hẳn — không lồng, nên không nhân, chỉ cộng: `n + n`,
vẫn tỉ lệ thuận với `n`.
::
:::

:::opt
Hằng số — vì hai vòng lặp "triệt tiêu" lẫn nhau, số bước cuối cùng
không đổi
::why
Gần đúng ở trực giác rằng hai thứ đối xứng có thể "cân bằng" nhau —
trực giác đó đúng ở một số phép toán khác (ví dụ cộng rồi trừ cùng một
số).

Chỗ lệch: hai vòng lặp này không hề đối nghịch nhau — cả hai đều CỘNG
THÊM vào `so_buoc`, không có vòng nào trừ đi hay huỷ tác dụng của vòng
kia. Số bước không đứng yên; nó tăng từ 20 lên 40 khi dữ liệu tăng từ
10 lên 20, đúng bằng chứng cho thấy đây không phải hằng số.
::
:::

:::opt
Không xác định được — vì hai vòng lặp độc lập cho ra một hình dạng
"lai", không khớp với ba tên vừa học
::why
Gần đúng ở sự cẩn trọng khi thấy một cấu trúc mã hơi khác các ví dụ đã
quen — thận trọng trước khi kết luận là một thói quen tốt.

Chỗ lệch: `n + n` vẫn là một hàm TỈ LỆ THUẬN với `n` — nhân đôi `n` thì
`n + n` cũng nhân đôi (`2n → 4n`, và `4n / 2n = 2`, đúng gấp đôi). Hệ số
nhân phía trước `n` có thể khác nhau (ở đây gấp đôi hệ số so với một
vòng lặp đơn), nhưng HÌNH DẠNG tăng — tỉ lệ thuận, đường thẳng — vẫn là
tuyến tính, không phải một hình dạng thứ tư.
::
:::
::::

::::code{#tinh-ti-le-tang}
Viết hai hàm đếm bước — một tuyến tính (`buoc_tuyen_tinh`, một vòng
lặp, đã cho sẵn), một bậc hai (`buoc_bac_hai`, hai vòng lặp lồng nhau,
đã cho sẵn) — rồi TỰ TÍNH tỉ lệ tăng của từng hàm khi cỡ dữ liệu gấp
đôi (từ 10 lên 20), để thấy con số 2 và con số 4 hiện ra từ chính phép
chia, không phải chép từ bài học.

```python title=starter
def buoc_tuyen_tinh(n):
    mang = list(range(n))
    so_buoc = 0
    for x in mang:
        so_buoc += 1
    return so_buoc

def buoc_bac_hai(n):
    mang = list(range(n))
    so_buoc = 0
    for x in mang:
        for y in mang:
            so_buoc += 1
    return so_buoc

buoc_tuyen_tinh_10 = buoc_tuyen_tinh(10)
buoc_tuyen_tinh_20 = buoc_tuyen_tinh(20)
ti_le_tuyen_tinh = ___                # buoc ở n=20 CHIA cho buoc ở n=10

buoc_bac_hai_10 = buoc_bac_hai(10)
buoc_bac_hai_20 = buoc_bac_hai(20)
ti_le_bac_hai = ___                   # buoc ở n=20 CHIA cho buoc ở n=10

print(f"tuyến tính: {buoc_tuyen_tinh_10} -> {buoc_tuyen_tinh_20}, tỉ lệ {ti_le_tuyen_tinh}")
print(f"bậc hai: {buoc_bac_hai_10} -> {buoc_bac_hai_20}, tỉ lệ {ti_le_bac_hai}")
```

```python title=solution
def buoc_tuyen_tinh(n):
    mang = list(range(n))
    so_buoc = 0
    for x in mang:
        so_buoc += 1
    return so_buoc

def buoc_bac_hai(n):
    mang = list(range(n))
    so_buoc = 0
    for x in mang:
        for y in mang:
            so_buoc += 1
    return so_buoc

buoc_tuyen_tinh_10 = buoc_tuyen_tinh(10)
buoc_tuyen_tinh_20 = buoc_tuyen_tinh(20)
ti_le_tuyen_tinh = buoc_tuyen_tinh_20 / buoc_tuyen_tinh_10

buoc_bac_hai_10 = buoc_bac_hai(10)
buoc_bac_hai_20 = buoc_bac_hai(20)
ti_le_bac_hai = buoc_bac_hai_20 / buoc_bac_hai_10

print(f"tuyến tính: {buoc_tuyen_tinh_10} -> {buoc_tuyen_tinh_20}, tỉ lệ {ti_le_tuyen_tinh}")
print(f"bậc hai: {buoc_bac_hai_10} -> {buoc_bac_hai_20}, tỉ lệ {ti_le_bac_hai}")
```

```python title=test
assert ti_le_tuyen_tinh == 2.0, f"tuyến tính: gấp đôi dữ liệu phải cho tỉ lệ 2.0 — đang ra {ti_le_tuyen_tinh}"
assert ti_le_bac_hai == 4.0, f"bậc hai: gấp đôi dữ liệu phải cho tỉ lệ 4.0 — đang ra {ti_le_bac_hai}"
assert ti_le_bac_hai == ti_le_tuyen_tinh ** 2, "tỉ lệ của bậc hai phải đúng bằng BÌNH PHƯƠNG tỉ lệ của tuyến tính (2 gấp lên thành 4)"
```

:::hints
- kind: attention
  body: Cả hai chỗ trống làm CÙNG một việc — chia số bước ở n=20 cho số bước ở n=10 — chỉ khác đang tính cho hàm nào. Đừng gõ thẳng số 2.0 hay 4.0 vào; phép CHIA mới là thứ bài này muốn bạn tự tay làm.
- kind: strategy
  body: 'Chỗ trống đầu: buoc_tuyen_tinh_20 / buoc_tuyen_tinh_10 — dùng đúng hai biến vừa gán ở hai dòng ngay phía trên. Chỗ trống sau: buoc_bac_hai_20 / buoc_bac_hai_10 — cùng khuôn, đổi tên biến.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `buoc_tuyen_tinh_20 / buoc_tuyen_tinh_10` và `buoc_bac_hai_20 / buoc_bac_hai_10`.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: cả hai chỗ trống phải THẬT SỰ chia hai số bước đã đo cho nhau bằng dấu / — không được gõ thẳng con số 2.0 hay 4.0 vào, và không được để một câu không làm gì như True, 1, 0
  requireAst:
  - kind: uses-operator, target: "/", min: 2
  # min: 2 — đếm thật trên solution: dấu / xuất hiện đúng hai lần trong
  # mã nguồn, một lần ở mỗi chỗ trống. Điền True/1/0 (một câu không
  # dùng phép chia) vào MỘT chỗ trống chỉ còn 1 lần chia — dưới 2, luật
  # chặn được; điền cả hai chỉ còn 0. ĐÃ THỬ THẬT cả ba cách True/1/0
  # cho cả hai chỗ trống: cả ba dừng NGAY (không có while/đệ quy trong
  # khối này, chỉ là hai phép gán đơn) và cả ba cho ti_le_tuyen_tinh và
  # ti_le_bac_hai bằng True/1/0 thay vì 2.0/4.0, sai hẳn, bị tests bắt
  # độc lập. Đã thử thêm một lời giải ĐÚNG khác — gọi lại
  # buoc_tuyen_tinh(20) / buoc_tuyen_tinh(10) thay vì dùng hai biến đã
  # gán sẵn — vẫn dùng đúng một dấu / mỗi chỗ trống, luật không đánh
  # trượt nhầm (đã chạy thử, ra đúng 2.0 và 4.0).
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: "^tuyến tính: 10 -> 20, tỉ lệ 2\\.0\\nbậc hai: 100 -> 400, tỉ lệ 4\\.0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tỉ lệ 2, tỉ lệ 4 — không phải chép từ bài học, mà tự tay chia ra từ
con số đo thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ba cái tên tiếng Việt vừa đặt — hằng số, tuyến tính, bậc hai — dùng tốt
giữa người với người. Nhưng cả thế giới lập trình lại quen gọi ba hình
dạng này bằng một KÝ HIỆU ngắn hơn nhiều, chỉ vài ký tự thay vì cả một
cụm từ. Ký hiệu đó viết ra làm sao, và nó có thêm điều gì mới không,
hay chỉ là một cách viết gọn cho đúng ba thứ vừa đặt tên?
::::

::::checkpoint{mastery=0.8}
::::
