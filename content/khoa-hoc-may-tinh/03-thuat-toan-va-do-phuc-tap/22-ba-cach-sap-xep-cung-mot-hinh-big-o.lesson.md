---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.ba-cach-sap-xep-cung-mot-hinh-big-o
title: "Ba cách sắp xếp trên, cùng một hình Big-O"
summary: "Đếm bước cả ba thuật toán (nổi bọt, chọn, chèn) trên cùng một dữ liệu lộn xộn thật — cả ba đều O(n²), nhưng số bước cụ thể khác nhau hẳn. Cùng hình dạng tăng không có nghĩa cùng tốc độ thật."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.quadratic-sorts-compare]
requires: [alg.insertion-sort]
concepts: [alg.quadratic-sorts-compare]
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
Trên mảng ngược hẳn, cả ba ra cùng con số — 15, 45, 190. Đổi dữ liệu
đi, chuyện đó còn đúng không?
::::

::::explain{#do-tren-cung-du-lieu}
Ba bài trước đo cả ba thuật toán trên đúng một loại dữ liệu: mảng
NGƯỢC HẲN — trường hợp XẤU NHẤT (bài 11) cho cả ba. Đó là lý do ba con
số trùng nhau: ở trường hợp xấu nhất, nổi bọt không bao giờ dừng sớm,
chọn luôn quét hết bất kể dữ liệu, chèn luôn phải dời tới tận đầu mảng
— cả ba đều chạm số bước LỚN NHẤT có thể, và con số lớn nhất đó tình
cờ bằng nhau ở cả ba: đúng `n(n-1)/2`.

Nhưng một mảng LỘN XỘN bình thường — không xấu tuyệt đối, không tốt
tuyệt đối — sẽ không ép cả ba vào cùng con số đó nữa:

- **Chọn** không có đường tắt nào: nó LUÔN quét hết phần chưa sắp để
  tìm nhỏ nhất, bất kể dữ liệu đã gần sắp xong hay còn lộn xộn. Số
  phép so sánh của chọn CỐ ĐỊNH ở `n(n-1)/2`, không phụ thuộc dữ liệu.
- **Nổi bọt** có thể dừng SỚM nếu một lượt trôi qua không đổi chỗ nào
  — dữ liệu càng gần sắp xong, nó càng ít việc.
- **Chèn** chỉ dời đúng số phần tử THẬT SỰ sai chỗ — dữ liệu càng gần
  sắp xong, số lần dời càng ít.

Cả ba vẫn là **O(n²)** — hình dạng tăng của chúng, xét theo bài 9,
không đổi. Nhưng "cùng hình dạng" không có nghĩa "cùng tốc độ thật" ở
một dữ liệu cụ thể. Bài này đo thẳng cả ba, cùng lúc, trên cùng một
mảng lộn xộn thật, để thấy rõ khoảng cách đó.
::::

::::example{#do-that-tren-du-lieu-lon-xon}
Byte dùng lại đúng ba hàm đã viết ở ba bài trước, chạy cả ba trên cùng
một mảng mười phần tử lộn xộn (không phải ngược hẳn):

```python title=readonly
mang_lon_xon = [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]

_, sb = sap_xep_noi_bot(mang_lon_xon)
_, sc = sap_xep_chon(mang_lon_xon)
_, sd = sap_xep_chen(mang_lon_xon)

print(f"Nổi bọt: {sb} phép so sánh")
print(f"Chọn:    {sc} phép so sánh")
print(f"Chèn:    {sd} lần dịch chuyển")
```

```text title=readonly
Nổi bọt: 42 phép so sánh
Chọn:    45 phép so sánh
Chèn:    18 lần dịch chuyển
```

Ba con số khác hẳn nhau, trên đúng một dữ liệu. `Chọn` giữ nguyên `45`
— công thức `n(n-1)/2` với `n = 10` — không nhích một chút nào so với
lúc dữ liệu ngược hẳn. `Nổi bọt` giảm nhẹ (từ 45 lý thuyết xuống 42)
nhờ vài lượt dừng sớm. `Chèn` giảm mạnh nhất (còn 18, chưa bằng nửa
`chọn`) vì phần lớn phần tử trong dữ liệu này đã không quá lệch xa
chỗ đúng của chúng.
::::

::::predict{#doan-thuat-toan-nao-nhieu-nhat commitOnce}
Vẫn đúng mảng `[0, 7, 4, 1, 8, 5, 2, 9, 6, 3]` ở trên.

**Trước khi xem lại bảng**, bạn đoán: trong ba thuật toán, thuật toán
nào cho ra SỐ BƯỚC LỚN NHẤT trên đúng dữ liệu này?

:::opt{correct}
Chọn — vì nó không có cách nào "nhận ra" dữ liệu đã gần sắp xong để
dừng sớm hay bỏ bớt việc; nó luôn quét hết, bất kể thế nào
:::

:::opt
Nổi bọt — vì nó đổi chỗ liên tục nên chắc phải tốn nhiều bước nhất
::why
Gần đúng ở việc nổi bọt đúng là đổi CHỖ nhiều (gần như mọi lần so sánh
sai thứ tự đều kéo theo một lần đổi) — quan sát đó có thật, từ bài 19.

Chỗ lệch: bài này đang đếm PHÉP SO SÁNH, không đếm phép đổi chỗ. Và
nổi bọt có một lợi thế chọn không có — nó có thể DỪNG SỚM khi một lượt
không đổi chỗ nào. Trên dữ liệu này, lợi thế đó đủ để nổi bọt (42) ít
hơn chọn (45), dù không chênh nhiều.
::
:::

:::opt
Chèn — vì nó phải dời cả một dải phần tử mỗi lần, nghe có vẻ tốn công
nhất trong ba cách
::why
Gần đúng ở việc CHÈN đúng là phải dời một DẢI phần tử, không chỉ một
ô — nghe có vẻ nặng, cảm giác đó dễ hiểu.

Chỗ lệch: số lần dời của chèn phụ thuộc dữ liệu THẬT SỰ lệch bao xa,
không phụ thuộc việc "dời một dải nghe có vẻ nặng". Trên dữ liệu này,
phần lớn phần tử không lệch quá xa chỗ đúng, nên tổng số lần dời (18)
lại là con số NHỎ NHẤT trong ba thuật toán, không phải lớn nhất.
::
:::

:::opt
Cả ba bằng nhau — vì tất cả đều là O(n²), nên trên cùng một dữ liệu
chúng phải cho cùng một con số
::why
Gần đúng ở việc bạn nhớ đúng cả ba đều O(n²) (bài 19-21) — điều đó
không sai.

Chỗ lệch: O(n²) chỉ nói về HÌNH DẠNG TĂNG khi dữ liệu lớn dần, không
hứa hẹn ba thuật toán cùng hình dạng phải cho ra CÙNG MỘT CON SỐ cụ
thể trên một dữ liệu nhất định. Con số cụ thể phụ thuộc CÁCH LÀM của
từng thuật toán — ba cách khác nhau ắt cho ba con số khác nhau, dù
cùng lớn theo kiểu bình phương.
::
:::
::::

::::code{#do-hai-co-va-so-sanh}
Gọi đúng ba hàm đã viết (nổi bọt, chọn, chèn — đều nhận sẵn) trên hai
mảng lộn xộn cỡ 10 và cỡ 20, tạo bằng công thức `(i * 7) % n`.

```python title=starter
def sap_xep_noi_bot(mang):
    mang = list(mang)
    n = len(mang)
    so_sanh = 0
    for luot in range(n - 1):
        da_doi_cho = False
        for i in range(n - 1 - luot):
            so_sanh += 1
            if mang[i] > mang[i + 1]:
                mang[i], mang[i + 1] = mang[i + 1], mang[i]
                da_doi_cho = True
        if not da_doi_cho:
            break
    return mang, so_sanh


def sap_xep_chon(mang):
    mang = list(mang)
    n = len(mang)
    so_sanh = 0
    for i in range(n - 1):
        idx_nho_nhat = i
        for j in range(i + 1, n):
            so_sanh += 1
            if mang[j] < mang[idx_nho_nhat]:
                idx_nho_nhat = j
        if idx_nho_nhat != i:
            mang[i], mang[idx_nho_nhat] = mang[idx_nho_nhat], mang[i]
    return mang, so_sanh


def sap_xep_chen(mang):
    mang = list(mang)
    n = len(mang)
    so_lan_dich_chuyen = 0
    for i in range(1, n):
        khoa = mang[i]
        j = i - 1
        while j >= 0 and mang[j] > khoa:
            mang[j + 1] = mang[j]
            so_lan_dich_chuyen += 1
            j -= 1
        mang[j + 1] = khoa
    return mang, so_lan_dich_chuyen


mang_10 = [(i * 7) % 10 for i in range(10)]
mang_20 = [(i * 7) % 20 for i in range(20)]

_, sb10 = sap_xep_noi_bot(___)
_, sb20 = sap_xep_noi_bot(___)
_, sc10 = sap_xep_chon(___)
_, sc20 = sap_xep_chon(___)
_, sd10 = sap_xep_chen(___)
_, sd20 = sap_xep_chen(___)

print(f"Nổi bọt:  n=10 -> {sb10} bước, n=20 -> {sb20} bước")
print(f"Chọn:     n=10 -> {sc10} bước, n=20 -> {sc20} bước")
print(f"Chèn:     n=10 -> {sd10} bước, n=20 -> {sd20} bước")
```

```python title=solution
def sap_xep_noi_bot(mang):
    mang = list(mang)
    n = len(mang)
    so_sanh = 0
    for luot in range(n - 1):
        da_doi_cho = False
        for i in range(n - 1 - luot):
            so_sanh += 1
            if mang[i] > mang[i + 1]:
                mang[i], mang[i + 1] = mang[i + 1], mang[i]
                da_doi_cho = True
        if not da_doi_cho:
            break
    return mang, so_sanh


def sap_xep_chon(mang):
    mang = list(mang)
    n = len(mang)
    so_sanh = 0
    for i in range(n - 1):
        idx_nho_nhat = i
        for j in range(i + 1, n):
            so_sanh += 1
            if mang[j] < mang[idx_nho_nhat]:
                idx_nho_nhat = j
        if idx_nho_nhat != i:
            mang[i], mang[idx_nho_nhat] = mang[idx_nho_nhat], mang[i]
    return mang, so_sanh


def sap_xep_chen(mang):
    mang = list(mang)
    n = len(mang)
    so_lan_dich_chuyen = 0
    for i in range(1, n):
        khoa = mang[i]
        j = i - 1
        while j >= 0 and mang[j] > khoa:
            mang[j + 1] = mang[j]
            so_lan_dich_chuyen += 1
            j -= 1
        mang[j + 1] = khoa
    return mang, so_lan_dich_chuyen


mang_10 = [(i * 7) % 10 for i in range(10)]
mang_20 = [(i * 7) % 20 for i in range(20)]

_, sb10 = sap_xep_noi_bot(mang_10)
_, sb20 = sap_xep_noi_bot(mang_20)
_, sc10 = sap_xep_chon(mang_10)
_, sc20 = sap_xep_chon(mang_20)
_, sd10 = sap_xep_chen(mang_10)
_, sd20 = sap_xep_chen(mang_20)

print(f"Nổi bọt:  n=10 -> {sb10} bước, n=20 -> {sb20} bước")
print(f"Chọn:     n=10 -> {sc10} bước, n=20 -> {sc20} bước")
print(f"Chèn:     n=10 -> {sd10} bước, n=20 -> {sd20} bước")
```

```python title=test
assert (sb10, sb20) == (42, 169), f"nổi bọt phải ra 42 (n=10) và 169 (n=20) — đang ra {sb10} và {sb20}"
assert (sc10, sc20) == (45, 190), f"chọn phải ra 45 (n=10) và 190 (n=20) — đang ra {sc10} và {sc20}"
assert (sd10, sd20) == (18, 63), f"chèn phải ra 18 (n=10) và 63 (n=20) — đang ra {sd10} và {sd20}"
assert len({sb10, sc10, sd10}) == 3, f"ba thuật toán trên CÙNG một dữ liệu (n=10) phải cho ba con số KHÁC NHAU — đang thấy {sb10}, {sc10}, {sd10}"
assert sb20 > 3 * sb10 and sc20 > 3 * sc10 and sd20 > 3 * sd10, "gấp đôi dữ liệu (10 lên 20) phải làm CẢ BA thuật toán tăng hơn gấp ba lần — đúng hình dạng O(n²) chung, dù con số cụ thể khác nhau"
```

:::hints
- kind: attention
  body: Sáu chỗ trống đều là THAM SỐ truyền vào lời gọi hàm — không có công thức nào để tính, chỉ cần chọn ĐÚNG mảng (mang_10 hay mang_20) khớp với biến kết quả đang gán ở vế trái (sb10 dùng mang_10, sb20 dùng mang_20, cứ thế).
- kind: strategy
  body: 'Nhìn tên biến nhận kết quả để biết mảng nào cần truyền: biến có đuôi 10 (sb10, sc10, sd10) gọi hàm với mang_10; biến có đuôi 20 (sb20, sc20, sd20) gọi hàm với mang_20.'
- kind: one-line
  body: 'Theo đúng thứ tự: mang_10, mang_20, mang_10, mang_20, mang_10, mang_20.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: sáu chỗ trống phải THẬT SỰ truyền mang_10 hoặc mang_20 vào lời gọi hàm — không gõ True/1/0 hay một giá trị cố định khác; bài này còn đo lại đúng ba thuật toán đã viết ở ba bài trước nên không gọi sorted()/.sort() có sẵn để thay chúng
  requireAst:
  - kind: uses-name, target: mang_10, min: 3
  - kind: uses-name, target: mang_20, min: 3
  # min: 3 mỗi tên — đếm thật: solution truyền mang_10 vào đúng ba lời gọi
  # (nổi bọt, chọn, chèn ở cỡ 10) và mang_20 vào đúng ba lời gọi khác — cả
  # hai tên đều Load đúng 3 lần trên toàn khối. Điền True vào cả sáu chỗ
  # trống thì "mang_10" và "mang_20" không còn xuất hiện ở dạng ĐỌC lần
  # nào (0 lần mỗi tên, vì hai dòng gán mang_10=... /mang_20=... phía trên
  # là Store, không tính) — dưới 3, luật này chặn được cả hai. ĐÃ THỬ THẬT
  # qua kiemAst(): solution đạt (3 và 3), điền True cho cả sáu chỗ trống bị
  # chặn (0 và 0, dưới ngưỡng).
  forbidAst:
  - kind: uses-call, target: sorted
  - kind: uses-call, target: sort
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "Chèn:     n=10 -> 18 bước, n=20 -> 63 bước"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba con số khác hẳn nhau — 42, 45, 18 — mà vẫn cùng chung một hình
dạng: gấp đôi dữ liệu, cả ba đều tăng hơn gấp ba lần.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cả ba thuật toán vừa đo đều có chung một điểm nghẽn: muốn biết vị trí
đúng của một phần tử, chúng phải SO SÁNH nó với từng phần tử khác, gần
như từng cặp một. Đó là lý do cả ba đều rơi vào O(n²) — số cặp cần so
tăng theo bình phương số phần tử.

Có cách nào tránh việc so từng CẶP một không? Nếu chia mảng làm đôi,
sắp xong TỪNG NỬA một cách độc lập — rồi chỉ cần GHÉP hai nửa đã sắp
lại với nhau — liệu tổng công sức có ít hơn hẳn so hết mọi cặp không?

Bài sau bắt đầu dựng đúng ý tưởng đó.
::::

::::checkpoint{mastery=0.8}
::::
