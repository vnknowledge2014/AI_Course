---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.merge-sort-nhanh-hon-o-dau
title: "Merge sort nhanh hơn ở đâu — Big-O nói được vì sao"
summary: "Đếm bước merge sort trên cùng kiểu dữ liệu đã dùng ở bài trước: tăng chậm hơn hẳn ba thuật toán bậc hai khi dữ liệu lớn dần — dù ở cỡ rất nhỏ, đo thật lại cho thấy nó có khi chậm hơn cả sắp xếp chèn. Big-O nói về xu hướng khi dữ liệu LỚN, không phải tốc độ tuyệt đối ở mọi cỡ."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [alg.merge-sort-complexity]
requires: [alg.merge-sort]
concepts: [alg.merge-sort-complexity]
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
Gọn hơn khi VIẾT chưa chắc đã nhanh hơn khi CHẠY. Hôm nay đếm thật,
không đoán.
::::

::::explain{#dem_buoc_tron}
Muốn đếm bước của sắp xếp trộn, áp đúng kỹ thuật đã dùng từ bài 6: một
biến toàn cục, tăng lên 1 mỗi khi chạm đúng chỗ cần đếm. Chỗ đáng đếm
ở đây là mỗi lần `tron_hai_day` phải QUYẾT ĐỊNH lấy phần tử nào trong
hai dãy con — mỗi lần so `trai[i]` với `phai[j]` là một bước.

Nhìn lại chia để trị (bài 16, 23): mảng bị cắt đôi liên tục, tạo ra
nhiều TẦNG — mỗi tầng chia đôi cỡ mảng của tầng trên, tới khi chỉ còn
một phần tử. Số tầng đó tăng rất chậm khi `n` tăng (gấp đôi `n` chỉ
thêm ĐÚNG MỘT tầng) — đây chính là phần "log n" trong tên gọi quen
thuộc O(n log n). Ở MỖI tầng, tổng số phần tử cần trộn cộng lại đúng
bằng `n` (mọi phần tử đều nằm trong đúng một cặp đang trộn ở tầng đó).
Số tầng nhân với công việc mỗi tầng — `n` nhân `log n` — tăng chậm hơn
hẳn `n` nhân `n` của ba thuật toán trước.

Nhưng con số cụ thể vẫn phải ĐO, không đoán bằng công thức. Bài này đo
thật, trên đúng kiểu dữ liệu đã dùng ở bài trước.
::::

::::example{#bang_do_that}
Byte đo cả bốn thuật toán — nổi bọt, chọn, chèn, trộn — trên cùng kiểu
dữ liệu lộn xộn `(i * 7) % n`, ở năm cỡ tăng dần:

```text title=readonly
n=  10   nổi bọt=    42   chọn=    45   chèn=    18   trộn=    24
n=  20   nổi bọt=   169   chọn=   190   chèn=    63   trộn=    63
n=  40   nổi bọt=   744   chọn=   780   chèn=   333   trộn=   163
n=  80   nổi bọt=  3082   chọn=  3160   chèn=  1353   trộn=   375
n= 160   nổi bọt= 12369   chọn= 12720   chèn=  5313   trộn=   838
```

Ba điều đáng dừng lại nhìn kỹ.

**Ở n=10, trộn (24) nhiều bước HƠN chèn (18).** Không phải lỗi đo —
đây là sự thật của Big-O: nó chỉ nói về XU HƯỚNG khi dữ liệu đủ LỚN,
không hứa hẹn thắng ở MỌI cỡ. Với mảng quá nhỏ, việc chia thành nhiều
tầng của trộn chưa kịp trả lại lợi ích, trong khi chèn trên dữ liệu ít
lộn xộn lại rất rẻ.

**Ở n=20, trộn và chèn ngang nhau tuyệt đối — cả hai đều 63.** Đây là
vùng chuyển giao: dữ liệu vừa đủ lớn để hai cách tiếp cận hoà nhau.

**Từ n=40 trở đi, trộn bỏ xa mọi thuật toán khác, và khoảng cách CÀNG
LÚC CÀNG RỘNG.** Ở n=160, chèn (5313) đã gấp hơn sáu lần trộn (838);
nổi bọt và chọn đã gấp hơn mười bốn lần. Gấp đôi dữ liệu từ 80 lên
160, ba thuật toán bậc hai đều tăng hơn gấp bốn lần đúng như O(n²) —
còn trộn chỉ tăng hơn gấp đôi một chút, đúng hình dạng O(n log n).
::::

::::predict{#doan_ai_it_buoc_nhat commitOnce}
Ở đúng cỡ `n = 10`, chưa xem lại bảng phía trên.

**Trước khi xem lại**, bạn đoán: trong BỐN thuật toán đã học cả cụm
này — nổi bọt, chọn, chèn, trộn — thuật toán nào có SỐ BƯỚC ÍT NHẤT ở
cỡ dữ liệu rất nhỏ này?

:::opt{correct}
Chèn — vì ở n=10, mảng chưa đủ lớn để lợi thế chia-tầng của trộn kịp
trả lại kết quả, trong khi chèn trên dữ liệu ít lệch chỗ lại rất rẻ
:::

:::opt
Trộn — vì Big-O đã xếp O(n log n) tăng chậm hơn O(n²), nên trộn phải
thắng ở MỌI cỡ dữ liệu, kể cả cỡ nhỏ
::why
Gần đúng ở việc bạn nhớ đúng thứ hạng Big-O — O(n log n) đúng là tăng
CHẬM HƠN O(n²) khi `n` đủ lớn, quan hệ đó có thật.

Chỗ lệch: Big-O nói về XU HƯỚNG khi dữ liệu lớn dần, không hứa hẹn
thắng ở MỌI cỡ, kể cả cỡ rất nhỏ. Đo thật ở n=10 cho thấy chèn (18
bước) còn ít bước hơn trộn (24 bước) — ngược với trực giác "big-O thấp
hơn thì luôn nhanh hơn".
::
:::

:::opt
Nổi bọt — vì với mảng rất nhỏ, thuật toán đơn giản nhất luôn là thuật
toán ít việc nhất
::why
Gần đúng ở cảm giác "mảng nhỏ thì thuật toán đơn giản cũng đủ dùng" —
cảm giác đó không sai một cách chung chung.

Chỗ lệch: đo thật ở n=10 cho nổi bọt 42 bước — nhiều hơn hẳn cả chèn
(18) lẫn trộn (24). "Viết đơn giản" không đồng nghĩa "chạy ít bước".
::
:::

:::opt
Chọn — vì mỗi lượt nó chỉ đổi chỗ đúng một lần (bài 20), nên tưởng
tổng công việc cũng ít nhất
::why
Gần đúng ở việc chọn đúng là đổi CHỖ ít nhất (bài 20) — quan sát đó có
thật.

Chỗ lệch: bài này đang đếm PHÉP SO SÁNH (hoặc bước quyết định của
trộn), không đếm phép đổi chỗ. Chọn luôn so sánh đúng `n(n-1)/2` cặp,
bất kể dữ liệu — ở n=10 đó là 45, NHIỀU NHẤT trong cả bốn thuật toán,
không phải ít nhất.
::
:::
::::

::::code{#dem_buoc_tron_va_kiem_hinh_dang}
Thêm bộ đếm bước sống vào `tron_hai_day` — chỗ trống duy nhất nằm
TRONG vòng `while`, không đụng gì tới điều kiện hay bước tiến của
vòng lặp đó, nên an toàn dù điền gì vào cũng không làm nó chạy mãi.

```python title=starter
so_buoc = 0

def tron_hai_day(trai, phai):
    global so_buoc
    ket_qua = []
    i, j = 0, 0
    while i < len(trai) and j < len(phai):
        ___
        if trai[i] <= phai[j]:
            ket_qua.append(trai[i]); i += 1
        else:
            ket_qua.append(phai[j]); j += 1
    ket_qua.extend(trai[i:])
    ket_qua.extend(phai[j:])
    return ket_qua


def sap_xep_tron(mang):
    if len(mang) <= 1:
        return mang
    giua = len(mang) // 2
    trai = sap_xep_tron(mang[:giua])
    phai = sap_xep_tron(mang[giua:])
    return tron_hai_day(trai, phai)


mang_40 = [(i * 7) % 40 for i in range(40)]
mang_80 = [(i * 7) % 80 for i in range(80)]

so_buoc = 0
sap_xep_tron(mang_40)
buoc_40 = so_buoc

so_buoc = 0
sap_xep_tron(mang_80)
buoc_80 = so_buoc

print(f"n=40: {buoc_40} bước trộn")
print(f"n=80: {buoc_80} bước trộn")
print(f"Tăng chưa tới ba lần khi n gấp đôi: {buoc_80 < 3 * buoc_40}")
```

```python title=solution
so_buoc = 0

def tron_hai_day(trai, phai):
    global so_buoc
    ket_qua = []
    i, j = 0, 0
    while i < len(trai) and j < len(phai):
        so_buoc += 1
        if trai[i] <= phai[j]:
            ket_qua.append(trai[i]); i += 1
        else:
            ket_qua.append(phai[j]); j += 1
    ket_qua.extend(trai[i:])
    ket_qua.extend(phai[j:])
    return ket_qua


def sap_xep_tron(mang):
    if len(mang) <= 1:
        return mang
    giua = len(mang) // 2
    trai = sap_xep_tron(mang[:giua])
    phai = sap_xep_tron(mang[giua:])
    return tron_hai_day(trai, phai)


mang_40 = [(i * 7) % 40 for i in range(40)]
mang_80 = [(i * 7) % 80 for i in range(80)]

so_buoc = 0
sap_xep_tron(mang_40)
buoc_40 = so_buoc

so_buoc = 0
sap_xep_tron(mang_80)
buoc_80 = so_buoc

print(f"n=40: {buoc_40} bước trộn")
print(f"n=80: {buoc_80} bước trộn")
print(f"Tăng chưa tới ba lần khi n gấp đôi: {buoc_80 < 3 * buoc_40}")
```

```python title=test
assert buoc_40 == 163, f"số bước trộn ở n=40 phải đúng 163 — đang đếm được {buoc_40}"
assert buoc_80 == 375, f"số bước trộn ở n=80 phải đúng 375 — đang đếm được {buoc_80}"
assert buoc_80 < 3 * buoc_40, f"gấp đôi dữ liệu (40 lên 80) phải làm số bước trộn tăng CHƯA TỚI ba lần — khác hẳn ba thuật toán bậc hai, nơi bước luôn tăng HƠN BA lần — đang thấy {buoc_40} lên {buoc_80}"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ngay đầu thân vòng while, TRƯỚC câu lệnh if — đây là nơi MỖI lần vòng lặp còn cả hai dãy con để so sánh, không quan tâm cuối cùng chọn phần tử của dãy trái hay dãy phải.
- kind: strategy
  body: 'Đúng kỹ thuật bài 6, 7, 19-21 đã dùng: một biến toàn cục (so_buoc, đã khai global ở đầu hàm) tăng thêm đúng 1 mỗi lần chạm chỗ cần đếm.'
- kind: one-line
  body: 'Chỗ trống là so_buoc += 1.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: chỗ trống phải THẬT SỰ tăng biến đếm so_buoc lên 1 mỗi lần vòng lặp còn cả hai dãy con để so — không phải một câu không làm gì như True, 1, 0, hay pass
  requireAst:
  - kind: uses-operator, target: +, min: 3
  # min: 3 — đếm thật trên solution: dấu + kiểu AugAssign xuất hiện đúng ba
  # lần trong khối while — "so_buoc += 1" (chỗ trống) cộng với "i += 1" và
  # "j += 1" đã có sẵn trong khung (không phải chỗ trống). Điền True/1/0/pass
  # vào chỗ trống chỉ còn lại 2 lần (hai dòng có sẵn) — dưới 3, luật này chặn
  # được. ĐÃ THỬ THẬT cả ba cách True/1/0/pass: cả ba dừng AN TOÀN và NHANH
  # (không lặp vô hạn — chỗ trống không đụng gì tới i, j hay điều kiện while),
  # nhưng cho so_buoc == 0 (True/pass) hoặc so_buoc == 1 cố định (gán lại),
  # sai hẳn so với 163/375 mà tests bắt độc lập.
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "n=80: 375 bước trộn"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
163, rồi 375 — chưa tới gấp ba, trong khi ba thuật toán trước luôn
tăng hơn gấp ba. Cùng là "sắp xếp", nhưng hai cách tăng khác hẳn nhau.
::::

::::reflect{#nghi-lai}
Một câu hỏi khép lại cả cụm bài về sắp xếp.

Sáu bài vừa qua đều làm việc trên một cấu trúc duy nhất: MẢNG — một
dãy thẳng, phần tử này đứng ngay sau phần tử kia, chỉ có đúng một
hướng "trước" và một hướng "sau". Cả bốn thuật toán sắp xếp đều dựa
vào chính tính chất đó: so sánh, đổi chỗ, chia đôi — tất cả đều giả
định biết rõ đâu là "phần tử kế tiếp".

Nhưng T3.2 còn dạy một cấu trúc mà "kế tiếp" không rõ ràng như thế:
**đồ thị**, nơi một đỉnh có thể nối tới NHIỀU đỉnh khác cùng lúc,
không theo hàng lối nào cả. Không thể "sắp xếp" một đồ thị theo nghĩa
vừa học — nhưng vẫn cần một cách đi THĂM HẾT mọi đỉnh của nó.

Bài sau bắt đầu đúng câu hỏi đó, dùng lại chính cái hàng đợi T3.2 đã
dựng.
::::

::::checkpoint{mastery=0.8}
::::
