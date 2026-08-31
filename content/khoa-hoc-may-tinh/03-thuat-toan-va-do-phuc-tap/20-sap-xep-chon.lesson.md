---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.sap-xep-chon
title: "Sắp xếp chọn: tìm nhỏ nhất, đưa lên đầu"
summary: "Mỗi lượt chỉ tìm PHẦN TỬ NHỎ NHẤT còn lại rồi đổi nó về đúng vị trí — đúng một lần đổi chỗ mỗi lượt, ít hơn hẳn nổi bọt, nhưng vẫn phải so hết mọi cặp để tìm ra nhỏ nhất, nên vẫn O(n²)."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [alg.selection-sort]
requires: [alg.bubble-sort]
concepts: [alg.selection-sort]
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
Nổi bọt đổi chỗ liên tục, từng bước nhích dần. Có cách nào mỗi phần tử
chỉ cần di chuyển ĐÚNG MỘT LẦN không?
::::

::::explain{#tim-nho-nhat-roi-dua-len-dau}
**Sắp xếp chọn** (selection sort) trả lời câu hỏi đó bằng một cách
nhìn khác hẳn nổi bọt. Thay vì so từng CẶP liền kề và đổi chỗ ngay khi
thấy lệch, nó làm đúng một việc mỗi lượt: quét HẾT phần mảng còn chưa
sắp để tìm ra PHẦN TỬ NHỎ NHẤT trong đó, rồi đổi chỗ phần tử ấy với ô
đầu tiên của phần chưa sắp — đúng MỘT lần đổi chỗ, đưa nó thẳng tới vị
trí cuối cùng của nó luôn, không cần đi lại lần thứ hai.

Lượt kế tiếp lặp lại đúng việc đó trên phần mảng còn lại (đã bỏ ra
ngoài phần tử vừa đưa vào đúng chỗ) — quét, tìm nhỏ nhất, đổi chỗ một
lần. Cứ thế cho tới khi chỉ còn đúng một phần tử, tự nó đã đúng chỗ.

Điểm khác biệt với nổi bọt nằm ở SỐ LẦN ĐỔI CHỖ: nổi bọt có thể đổi
chỗ ở gần như mọi phép so sánh sai thứ tự; sắp xếp chọn đổi chỗ ĐÚNG
MỘT LẦN mỗi lượt, dù mảng ban đầu lộn xộn tới đâu — cao nhất là `n - 1`
lần đổi chỗ cho cả quá trình.

Nhưng có một cái giá không đổi: để biết đâu là nhỏ nhất, vẫn phải SO
HẾT mọi phần tử còn lại trong phần chưa sắp — không có đường tắt nào
bỏ bớt việc so sánh này. Số phép so sánh vẫn cộng dồn thành đúng
`n(n-1)/2` — y hệt cỡ của nổi bọt. Ít đổi chỗ hơn hẳn, nhưng vẫn
**O(n²)** vì phép SO SÁNH — thứ quyết định hình dạng tăng — không hề
giảm.
::::

::::example{#theo-doi-tung-luot-chon}
Byte chạy sắp xếp chọn trên năm số `[5, 3, 8, 4, 2]`, in trạng thái
mảng sau mỗi lượt:

```python title=readonly
def sap_xep_chon(mang):
    n = len(mang)
    for i in range(n - 1):
        idx_nho_nhat = i
        for j in range(i + 1, n):
            if mang[j] < mang[idx_nho_nhat]:
                idx_nho_nhat = j
        if idx_nho_nhat != i:
            mang[i], mang[idx_nho_nhat] = mang[idx_nho_nhat], mang[i]
        print(f"Sau lượt {i + 1}: {mang}")
    return mang

sap_xep_chon([5, 3, 8, 4, 2])
```

```text title=readonly
Sau lượt 1: [2, 3, 8, 4, 5]
Sau lượt 2: [2, 3, 8, 4, 5]
Sau lượt 3: [2, 3, 4, 8, 5]
Sau lượt 4: [2, 3, 4, 5, 8]
```

Nhìn kỹ lượt 2: mảng KHÔNG ĐỔI. Phần tử nhỏ nhất còn lại trong phần
chưa sắp (`3`) đã nằm sẵn đúng chỗ của nó ngay từ đầu, nên
`idx_nho_nhat == i` và câu lệnh `if` bỏ qua bước đổi chỗ. Đây chính là
điều nổi bọt không có: sắp xếp chọn chỉ đổi chỗ khi THẬT SỰ cần, và
không bao giờ đổi chỗ nhiều hơn một lần mỗi lượt.
::::

::::predict{#doan-sau-luot-dau-chon commitOnce}
Cùng mảng `[4, 1, 3, 2]` đã dùng ở bài trước (bài nổi bọt), nhưng lần
này chạy đúng thuật toán sắp xếp CHỌN vừa học.

**Trước khi chạy**, bạn đoán: mảng trông thế nào sau LƯỢT ĐẦU TIÊN?

:::opt{correct}
`[1, 4, 3, 2]` — quét hết bốn phần tử, thấy `1` (ở chỉ số 1) là nhỏ
nhất, đổi chỗ đúng MỘT LẦN với chỉ số 0; `3` và `2` chưa hề bị đụng tới
:::

:::opt
`[1, 3, 2, 4]` — giống kết quả nổi bọt ở bài trước, vì cùng mảng đầu
vào thì hai thuật toán phải cho cùng kết quả sau một lượt
::why
Gần đúng ở việc `[1, 3, 2, 4]` có thật — đó CHÍNH XÁC là kết quả của
NỔI BỌT sau một lượt trên mảng này (bài trước), không sai ở đó.

Chỗ lệch: hai thuật toán khác cơ chế thì khác cả kết quả TRUNG GIAN,
dù kết quả CUỐI CÙNG giống nhau. Sắp xếp chọn không đổi chỗ liên tục
từng cặp liền kề như nổi bọt — nó chỉ quét tìm nhỏ nhất rồi đổi ĐÚNG
MỘT LẦN, nên `3` và `2` đứng nguyên vị trí cũ sau lượt đầu, không dịch
chuyển như ở nổi bọt.
::
:::

:::opt
`[1, 2, 3, 4]` — mảng đã sắp xong hoàn toàn chỉ sau một lượt
::why
Gần đúng ở kết quả CUỐI CÙNG — mảng thật sự sẽ về đúng `[1, 2, 3, 4]`
sau khi hết mọi lượt, điều đó không sai.

Chỗ lệch nằm ở SỐ LƯỢT. Mỗi lượt của sắp xếp chọn chỉ đưa đúng MỘT
phần tử (nhỏ nhất còn lại) về đúng chỗ của nó — ba phần tử còn lại vẫn
phải đợi các lượt sau, không xong hết trong một lượt.
::
:::

:::opt
`[4, 1, 3, 2]` — mảng không đổi, vì `1` đã ở gần đầu mảng nên coi như
đã đúng chỗ
::why
Gần đúng ở việc bạn để ý `1` đứng khá gần vị trí đúng của nó ngay từ
đầu — quan sát đó không sai.

Chỗ lệch: "gần đúng chỗ" không phải "đúng chỗ". Sắp xếp chọn luôn đổi
chỗ phần tử nhỏ nhất TÌM ĐƯỢC về đúng vị trí ĐẦU TIÊN của phần chưa
sắp, dù phải đổi với phần tử nào đang đứng đó — ở đây `1` (chỉ số 1)
vẫn phải đổi chỗ với `4` (chỉ số 0), mảng có thay đổi thật.
::
:::
::::

::::code{#viet-chon-va-dem}
Hoàn thiện `sap_xep_chon` — điền phép so sánh để tìm phần tử nhỏ nhất,
và chỉ số cần ghi nhớ khi tìm thấy một phần tử nhỏ hơn.

```python title=starter
def sap_xep_chon(mang):
    n = len(mang)
    so_sanh = 0
    so_doi_cho = 0
    for i in range(n - 1):
        idx_nho_nhat = i
        for j in range(i + 1, n):
            so_sanh += 1
            if mang[j] ___ mang[idx_nho_nhat]:
                idx_nho_nhat = ___
        if idx_nho_nhat != i:
            mang[i], mang[idx_nho_nhat] = mang[idx_nho_nhat], mang[i]
            so_doi_cho += 1
    return mang, so_sanh, so_doi_cho


mang_nho = [6, 5, 4, 3, 2, 1]
ket_qua, dem_nho, doi_nho = sap_xep_chon(mang_nho)
print(f"Đã sắp: {ket_qua}")
print(f"6 phần tử: {dem_nho} so sánh, {doi_nho} đổi chỗ")

mang_10 = list(range(10, 0, -1))
_, dem_10, doi_10 = sap_xep_chon(mang_10)
mang_20 = list(range(20, 0, -1))
_, dem_20, doi_20 = sap_xep_chon(mang_20)
print(f"10 phần tử: {dem_10} so sánh, {doi_10} đổi chỗ")
print(f"20 phần tử: {dem_20} so sánh, {doi_20} đổi chỗ")

mang_trung = [3, 1, 3, 2]
_, dem_trung, doi_trung = sap_xep_chon(mang_trung)
print(f"Có trùng: {dem_trung} so sánh, {doi_trung} đổi chỗ")
```

```python title=solution
def sap_xep_chon(mang):
    n = len(mang)
    so_sanh = 0
    so_doi_cho = 0
    for i in range(n - 1):
        idx_nho_nhat = i
        for j in range(i + 1, n):
            so_sanh += 1
            if mang[j] < mang[idx_nho_nhat]:
                idx_nho_nhat = j
        if idx_nho_nhat != i:
            mang[i], mang[idx_nho_nhat] = mang[idx_nho_nhat], mang[i]
            so_doi_cho += 1
    return mang, so_sanh, so_doi_cho


mang_nho = [6, 5, 4, 3, 2, 1]
ket_qua, dem_nho, doi_nho = sap_xep_chon(mang_nho)
print(f"Đã sắp: {ket_qua}")
print(f"6 phần tử: {dem_nho} so sánh, {doi_nho} đổi chỗ")

mang_10 = list(range(10, 0, -1))
_, dem_10, doi_10 = sap_xep_chon(mang_10)
mang_20 = list(range(20, 0, -1))
_, dem_20, doi_20 = sap_xep_chon(mang_20)
print(f"10 phần tử: {dem_10} so sánh, {doi_10} đổi chỗ")
print(f"20 phần tử: {dem_20} so sánh, {doi_20} đổi chỗ")

mang_trung = [3, 1, 3, 2]
_, dem_trung, doi_trung = sap_xep_chon(mang_trung)
print(f"Có trùng: {dem_trung} so sánh, {doi_trung} đổi chỗ")
```

```python title=test
assert ket_qua == [1, 2, 3, 4, 5, 6], f"mảng sáu phần tử phải sắp thành [1, 2, 3, 4, 5, 6] — đang ra {ket_qua}"
assert dem_nho == 15, f"số phép so sánh với 6 phần tử phải đúng 15 (n(n-1)/2, không phụ thuộc dữ liệu) — đang đếm được {dem_nho}"
assert doi_nho == 3, f"số lần đổi chỗ với mảng ngược hẳn 6 phần tử phải đúng 3 — đang đếm được {doi_nho}"
assert dem_10 == 45 and dem_20 == 190, f"so sánh phải đúng 45 (n=10) và 190 (n=20) — đang ra {dem_10} và {dem_20}"
assert doi_10 == 5 and doi_20 == 10, f"đổi chỗ phải đúng 5 (n=10) và 10 (n=20) — đang ra {doi_10} và {doi_20}"
assert doi_20 < dem_20, f"số lần đổi chỗ phải ÍT HƠN hẳn số phép so sánh — đây chính là điểm khác nổi bọt: {doi_20} đổi chỗ so với {dem_20} so sánh"
assert doi_trung == 2, f"mảng có phần tử TRÙNG [3, 1, 3, 2] phải đổi chỗ đúng 2 lần — dùng <= thay vì < sẽ coi phần tử trùng cũng là 'nhỏ hơn', ra 3 lần đổi chỗ thay vì 2 — đang đếm được {doi_trung}"
```

:::hints
- kind: attention
  body: Chỗ trống 1 nằm trong điều kiện `if` — hỏi phần tử VỪA XÉT (mang[j]) có NHỎ HƠN phần tử nhỏ nhất tìm được TỪ TRƯỚC (mang[idx_nho_nhat]) hay không. Chỗ trống 2 ghi lại chỉ số của phần tử nhỏ nhất MỚI, khi tìm thấy.
- kind: strategy
  body: 'Chỗ trống 1: dấu < — vì đang tìm phần tử NHỎ NHẤT (mảng tăng dần). Chỗ trống 2: j — chỉ số của phần tử VỪA SO SÁNH, chính là phần tử nhỏ nhất mới, không phải i hay một hằng số cố định.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là < và j.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: bài này dạy cách TỰ VIẾT thuật toán sắp xếp bằng tay — không gọi sorted()/.sort() có sẵn
  forbidAst:
  - kind: uses-call, target: sorted
  - kind: uses-call, target: sort
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Đã sắp: [1, 2, 3, 4, 5, 6]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
15 phép so sánh, y hệt nổi bọt — nhưng chỉ 3 lần đổi chỗ, thay vì gần
như mọi phép so sánh đều kéo theo một lần đổi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sắp xếp chọn nhìn từ đầu phần chưa sắp, tìm nhỏ nhất, đưa nó lên đầu.
Có một cách nhìn NGƯỢC LẠI: nhìn từ đầu phần ĐÃ sắp, cầm lấy phần tử
tiếp theo còn chưa sắp, rồi chèn nó vào ĐÚNG chỗ của nó trong phần đã
sắp — giống hệt cách một người xếp bài trên tay, cầm từng lá bài mới
rút được và luồn nó vào đúng vị trí giữa những lá đã xếp gọn.

Chèn vào GIỮA một dãy đã có — bạn còn nhớ cái giá phải trả cho việc đó
không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
