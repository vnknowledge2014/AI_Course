---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.sap-xep-tron
title: "Sắp xếp trộn: chia để trị đúng nghĩa"
summary: "Ghép hai công cụ đã có: CẮT mảng làm đôi tới khi còn một phần tử, rồi TRỘN từng cặp mảng con đã sắp lại — đây chính là bước ghép phức tạp hơn mà chia để trị đã hứa hẹn."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [alg.merge-sort]
requires: [alg.divide-and-conquer, alg.merge-step, alg.quadratic-sorts-compare]
concepts: [alg.merge-sort]
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
Chia mảng làm đôi, sắp từng nửa riêng, rồi ghép lại. Hai mảnh ghép đã
có sẵn từ trước — hôm nay chỉ còn việc lắp chúng vào nhau.
::::

::::explain{#ghep-chia-de-tri-va-tron}
Bài trước để lại một câu hỏi: chia mảng làm đôi, sắp xong từng nửa,
rồi GHÉP hai nửa đã sắp lại — có tránh được việc so từng cặp một
không? **Sắp xếp trộn** (merge sort) trả lời bằng cách LẮP đúng hai
công cụ đã học lại với nhau, không thêm công cụ mới nào:

1. **Chia để trị** (bài 16): cắt mảng làm đôi, gọi đệ quy sắp xếp
   từng nửa — CẮT tiếp tới khi mỗi nửa chỉ còn ĐÚNG một phần tử. Một
   phần tử luôn tự nó đã sắp xong — đây chính là ca dừng của đệ quy
   (R1.T1.3), không thể cắt nhỏ hơn được nữa.
2. **Trộn hai dãy đã sắp** (bài 17): khi hai nửa con đã sắp xong (nhờ
   đệ quy tự lo), TRỘN chúng lại thành một dãy sắp bằng cách luôn lấy
   phần tử nhỏ hơn ở đầu hai dãy con — đúng kỹ thuật đã học, không đổi
   gì cả.

Bài 16 từng hứa hẹn "bước ghép phức tạp hơn" so với tìm nhị phân (nơi
ghép chỉ đơn giản là "lấy kết quả của nửa đúng"). Đây chính là bước
ghép đó: không chỉ chọn MỘT nửa, mà phải TRỘN CẢ HAI nửa lại với nhau,
từng phần tử một, giữ đúng thứ tự.

Điểm mấu chốt cần giữ nguyên khi viết: mảng phải được chia thành hai
NỬA GẦN BẰNG NHAU ở mỗi bước — không phải "một phần tử, phần còn lại".
Việc trộn vẫn cho ra kết quả ĐÚNG dù chia lệch cỡ nào đi nữa (trộn chỉ
cần hai dãy đã sắp là đủ, không quan tâm chúng dài ngắn ra sao) —
nhưng chia LỆCH sẽ làm mất đúng lợi thế tốc độ mà bài sau sẽ đo, nên
phải cẩn thận dùng đúng điểm chia GIỮA mảng.
::::

::::example{#tron-hai-day-lai}
Hàm trộn hai dãy đã sắp — đúng công cụ bài 17 đã dựng, dùng lại
nguyên vẹn ở đây:

```python title=readonly
def tron_hai_day(trai, phai):
    ket_qua = []
    i, j = 0, 0
    while i < len(trai) and j < len(phai):
        if trai[i] <= phai[j]:
            ket_qua.append(trai[i])
            i += 1
        else:
            ket_qua.append(phai[j])
            j += 1
    ket_qua.extend(trai[i:])
    ket_qua.extend(phai[j:])
    return ket_qua

print(tron_hai_day([2, 5, 8], [1, 3, 9]))
```

```text title=readonly
[1, 2, 3, 5, 8, 9]
```

Với mảng bốn phần tử `[8, 3, 5, 1]`, đệ quy chia đôi liên tiếp thế
này: `[8, 3, 5, 1]` cắt thành `[8, 3]` và `[5, 1]`; `[8, 3]` cắt tiếp
thành `[8]` và `[3]` — cả hai chỉ còn một phần tử, ca dừng. Vì dòng
gọi nửa TRÁI luôn được viết TRƯỚC dòng gọi nửa PHẢI, Python đi hết
toàn bộ nhánh trái — kể cả mọi tầng đệ quy sâu bên trong nó — trước
khi đụng tới bất kỳ phần tử nào ở nhánh phải. Cặp `[8]` và `[3]` vì
vậy là cặp được TRỘN ĐẦU TIÊN trong cả quá trình, cho ra `[3, 8]`.
::::

::::predict{#doan-cap-tron-dau-tien commitOnce}
Mảng `[8, 3, 5, 1]`, sắp xếp bằng merge sort đúng như vừa mô tả ở
trên: cắt đôi tới khi còn một phần tử, rồi trộn dần lên.

**Trước khi chạy**, bạn đoán: cặp phần tử NÀO được trộn (merge) với
nhau ĐẦU TIÊN, trong toàn bộ quá trình?

:::opt{correct}
`8` và `3` — vì lời gọi đệ quy cho nửa TRÁI luôn được viết và chạy
XONG trước khi đụng tới nửa phải, nên nhánh trái ngoài cùng
(`[8, 3]` → `[8]` và `[3]`) chạm ca dừng và trộn lại trước tiên
:::

:::opt
`5` và `1` — vì chúng nằm ở nửa SAU của mảng, và mã luôn xử lý xong
nửa sau trước khi quay lại xử lý nửa đầu
::why
Gần đúng ở việc bạn nhận ra đúng mảng bị chia làm hai nửa `[8, 3]` và
`[5, 1]` — quan sát về việc chia đôi đó không sai.

Chỗ lệch nằm ở THỨ TỰ xử lý. Dòng gọi đệ quy cho nửa TRÁI
(`sap_xep_tron(mang[:giua])`) luôn được VIẾT TRƯỚC dòng gọi nửa phải,
nên Python luôn chạy XONG toàn bộ nhánh trái — không chỉ một bước, mà
mọi tầng đệ quy bên trong nó — trước khi bắt đầu đụng tới nhánh phải.
::
:::

:::opt
Cả bốn phần tử `8, 3, 5, 1` được trộn cùng lúc, trong một lần gọi
`tron_hai_day` duy nhất
::why
Gần đúng ở việc `tron_hai_day` đúng là hàm DUY NHẤT đảm nhận việc trộn
trong cả thuật toán — không có hàm trộn nào khác.

Chỗ lệch: nó không nhận nguyên bốn phần tử cùng lúc. Chia để trị (bài
16) cắt đôi liên tục tới khi mỗi bên chỉ còn ĐÚNG một phần tử, rồi
`tron_hai_day` mới bắt đầu chạy — nhiều LẦN, từng cặp nhỏ trước, ghép
dần lên thành cặp lớn hơn, không phải một lần duy nhất cho cả mảng.
::
:::

:::opt
`8` không trộn với ai — vì mảng một phần tử là ca dừng, tự nó đã "sắp
xong", không cần trộn nữa
::why
Gần đúng ở việc `[8]` đúng là một ca dừng — hàm trả về `[8]` ngay,
không đệ quy thêm nữa. Quan sát đó có thật.

Chỗ lệch: đó không phải điểm DỪNG của cả thuật toán, chỉ là điểm dừng
của MỘT lời gọi đệ quy. Cấp gọi ngay bên trên nó — đang giữ `[8, 3]` —
vẫn phải TRỘN kết quả `[8]` với kết quả của lời gọi anh em `[3]` lại
với nhau, trước khi trả kết quả `[3, 8]` lên cấp cao hơn nữa.
::
:::
::::

::::code{#viet-sap-xep-tron}
Hoàn thiện `sap_xep_tron` — điền điểm cắt cho hai nửa trái và phải.
Biến `giua` đã được tính sẵn ngay phía trên; dùng đúng nó cho CẢ HAI
chỗ trống, để mảng luôn được chia thành hai NỬA GẦN BẰNG NHAU (không
phải một phần tử với phần còn lại).

```python title=starter
def tron_hai_day(trai, phai):
    ket_qua = []
    i, j = 0, 0
    while i < len(trai) and j < len(phai):
        if trai[i] <= phai[j]:
            ket_qua.append(trai[i])
            i += 1
        else:
            ket_qua.append(phai[j])
            j += 1
    ket_qua.extend(trai[i:])
    ket_qua.extend(phai[j:])
    return ket_qua


def sap_xep_tron(mang):
    if len(mang) <= 1:
        return mang
    giua = len(mang) // 2
    trai = sap_xep_tron(mang[:___])
    phai = sap_xep_tron(mang[___:])
    return tron_hai_day(trai, phai)


print(sap_xep_tron([5, 2, 8, 1, 9, 3]))
print(sap_xep_tron([9, 9, 1, 1, 5]))
print(sap_xep_tron([]))
print(sap_xep_tron([7]))
```

```python title=solution
def tron_hai_day(trai, phai):
    ket_qua = []
    i, j = 0, 0
    while i < len(trai) and j < len(phai):
        if trai[i] <= phai[j]:
            ket_qua.append(trai[i])
            i += 1
        else:
            ket_qua.append(phai[j])
            j += 1
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


print(sap_xep_tron([5, 2, 8, 1, 9, 3]))
print(sap_xep_tron([9, 9, 1, 1, 5]))
print(sap_xep_tron([]))
print(sap_xep_tron([7]))
```

```python title=test
assert sap_xep_tron([5, 2, 8, 1, 9, 3]) == [1, 2, 3, 5, 8, 9], "mảng sáu phần tử phải sắp đúng thành [1, 2, 3, 5, 8, 9]"
assert sap_xep_tron([9, 9, 1, 1, 5]) == [1, 1, 5, 9, 9], "mảng có phần tử trùng nhau phải sắp đúng thành [1, 1, 5, 9, 9]"
assert sap_xep_tron([]) == [], "mảng rỗng sắp xong vẫn phải là mảng rỗng"
assert sap_xep_tron([7]) == [7], "mảng một phần tử sắp xong vẫn là chính nó — đây chính là ca dừng của đệ quy"
```

:::hints
- kind: attention
  body: Cả hai chỗ trống phải dùng đúng biến `giua` đã tính sẵn ở dòng ngay phía trên — không gõ một con số cố định, không gõ True/1/0. Việc trộn vẫn ra kết quả đúng dù chia lệch cỡ, nhưng bài này đang dạy CHIA ĐÔI, và bài sau đo tốc độ dựa đúng vào việc chia đôi này.
- kind: strategy
  body: 'Nửa TRÁI là mọi phần tử TỪ ĐẦU tới trước giua: mang[:giua]. Nửa PHẢI là mọi phần tử TỪ giua trở đi: mang[giua:]. Hai chỗ trống đều điền đúng cái tên giua.'
- kind: one-line
  body: 'Cả hai chỗ trống đều là giua.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: hai chỗ trống phải dùng đúng biến giua đã tính sẵn — chia mảng thành hai NỬA GẦN BẰNG NHAU; nếu không dùng giua (ví dụ điền True, 1, 0, hay một con số cố định khác), kết quả có thể vẫn đúng — vì trộn luôn cho ra dãy sắp đúng bất kể chia lệch cỡ nào — nhưng cách cắt sẽ không còn là CHIA ĐÔI nữa, và bài sau đo tốc độ dựa đúng vào việc chia đôi này
  requireAst:
  - kind: uses-name, target: giua, min: 2
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "[1, 2, 3, 5, 8, 9]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không viết thêm thuật toán trộn mới nào — chỉ lắp đúng hai công cụ đã
có, và mảng tự nó sắp xong.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sắp xếp trộn viết ra trông gọn hơn hẳn ba thuật toán trước — không có
vòng lặp lồng vòng lặp nào cả, chỉ có đệ quy chia đôi và một lần trộn.
Nhưng "gọn hơn" và "nhanh hơn" là hai chuyện khác nhau — bài này còn
chưa hề đếm một bước nào của nó.

Nếu đem đúng bộ đếm bước đã dùng suốt cụm này áp lên sắp xếp trộn, con
số sẽ tăng theo hình dạng nào khi dữ liệu gấp đôi — vẫn gấp bốn như ba
thuật toán trước, hay khác hẳn?

Bài sau đo thật.
::::

::::checkpoint{mastery=0.8}
::::
