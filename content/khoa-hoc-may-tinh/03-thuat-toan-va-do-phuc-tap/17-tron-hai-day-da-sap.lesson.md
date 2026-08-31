---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.tron-hai-day-da-sap
title: "Trộn hai dãy đã sắp thành một dãy sắp"
summary: "Cho hai dãy đã sắp riêng, ghép thành MỘT dãy vẫn sắp bằng cách luôn lấy phần tử nhỏ hơn ở đầu hai dãy con. Đứng riêng chưa cần chia để trị — bài sau mới ghép nó vào làm bước Ghép cho một thuật toán sắp xếp thật."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [alg.merge-step]
requires: [ds.array-contiguous]
concepts: [alg.merge-step]
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
Hai hàng người, mỗi hàng đã tự xếp từ thấp tới cao. Gộp hai hàng thành
MỘT hàng vẫn thấp tới cao — không cần xếp lại từ đầu.
::::

::::explain{#luon-lay-phan-tu-nho-hon}
Bài trước để lại một câu hỏi: nếu hai nửa của một bước Ghép không phải
hai con số, mà là hai DÃY đã tự sắp xếp riêng, ghép chúng thành một dãy
vẫn sắp xếp có còn đơn giản như một phép cộng không?

Có một cách làm không cần XẾP LẠI TỪ ĐẦU. Tận dụng đúng một điều: cả hai
dãy đều ĐÃ sắp xếp, nên trong mỗi dãy, giá trị NHỎ NHẤT còn lại luôn nằm
ở ĐẦU dãy đó. Vậy giá trị nhỏ nhất trong TOÀN BỘ hai dãy gộp lại chỉ có
thể là một trong hai: đầu dãy thứ nhất, hoặc đầu dãy thứ hai. So sánh
đúng hai giá trị đó — không phải so với cả dãy — là đủ biết cái nào nhỏ
hơn.

Cách làm, gọi là **trộn** (merge):

- Giữ hai con trỏ, mỗi cái chỉ vào phần tử ĐẦU CÒN LẠI của một dãy.
- So hai phần tử đang trỏ tới. Lấy cái NHỎ HƠN, đặt vào dãy kết quả, rồi
  đẩy con trỏ của dãy vừa lấy tiến lên một ô.
- Lặp lại, tới khi MỘT trong hai dãy hết phần tử.
- Dãy còn lại (nếu còn) chắc chắn đã tự sắp xếp sẵn rồi — vì mọi phần tử
  của nó đều lớn hơn hoặc bằng mọi phần tử vừa lấy ra trước đó (nếu
  không, nó đã bị lấy ra sớm hơn) — nên việc còn lại là CHÉP NGUYÊN phần
  đó vào cuối, không phải so sánh gì thêm.

Mỗi phần tử trong cả hai dãy đều được ĐỌC và ĐẶT vào kết quả đúng MỘT
lần — không đọc lại, không xếp lại. Đứng một mình, trộn chưa phải chia
để trị — nó không CẮT gì cả, chỉ GHÉP hai thứ đã có sẵn. Bài sau sẽ cho
nó đúng vai trò của một bước Ghép, gắn liền với một bước Cắt phía trước.
::::

::::example{#tron-hai-day-so}
Byte có hai dãy điểm, mỗi dãy đã tự sắp xếp tăng dần, độ dài khác nhau:

```python title=readonly
def tron_hai_day(day1, day2):
    ket_qua = []
    i = 0
    j = 0
    while i < len(day1) and j < len(day2):
        if day1[i] <= day2[j]:
            ket_qua.append(day1[i])
            i += 1
        else:
            ket_qua.append(day2[j])
            j += 1
    while i < len(day1):              # day2 đã hết -> chép nguyên phần còn lại của day1
        ket_qua.append(day1[i])
        i += 1
    while j < len(day2):              # day1 đã hết -> chép nguyên phần còn lại của day2
        ket_qua.append(day2[j])
        j += 1
    return ket_qua

day_le = [3, 8, 15, 21]
day_chan = [2, 4, 6, 9, 12, 30]

print(tron_hai_day(day_le, day_chan))
```

```text title=readonly
[2, 3, 4, 6, 8, 9, 12, 15, 21, 30]
```

Mười phần tử, đúng bằng tổng độ dài hai dãy đầu vào — không thừa, không
thiếu. Nhìn kỹ điểm cuối: `day_le` cạn trước (chỉ có bốn phần tử, dãy
kia có sáu). Ngay khi `i` chạm hết `day_le`, vòng `while` đầu dừng lại,
và vòng `while j < len(day2):` chép nốt `12` rồi `30` vào cuối — không
cần so sánh gì thêm, vì cả hai đều chắc chắn lớn hơn mọi phần tử đã lấy
ra trước đó.
::::

::::predict{#doan-truong-hop-bang-nhau commitOnce}
Hai dãy có một giá trị TRÙNG NHAU: `day_le = [5, 9]` và
`day_chan = [5, 12]`. Trộn bằng đúng hàm `tron_hai_day` ở trên.

**Trước khi chạy**, bạn đoán: phần tử `5` xuất hiện MẤY lần trong kết
quả, và phần tử của DÃY NÀO được lấy trước khi hai giá trị bằng nhau?

:::opt{correct}
`5` xuất hiện đúng HAI lần (một từ mỗi dãy) — và phần tử của `day_le`
(dãy thứ nhất) được lấy trước, vì điều kiện so sánh dùng `<=`, ưu tiên
dãy thứ nhất khi hai giá trị bằng nhau
:::

:::opt
`5` chỉ xuất hiện MỘT lần — hàm coi hai giá trị bằng nhau là TRÙNG LẶP
và tự động bỏ bớt một cái
::why
Gần đúng ở việc bạn cẩn thận nghĩ tới trường hợp trùng giá trị — sự thận
trọng đó đúng hướng.

Chỗ lệch: `tron_hai_day` không hề kiểm tra hay loại bỏ trùng lặp. Nó chỉ
làm đúng một việc — so sánh rồi lấy giá trị nhỏ hơn hoặc bằng — và MỖI
lần vòng lặp chạy, nó lấy đúng MỘT phần tử từ MỘT dãy, đẩy con trỏ dãy
đó lên. Hai số `5` nằm ở hai dãy khác nhau, tại hai VỊ TRÍ khác nhau —
cả hai đều được lấy ra, không cái nào bị bỏ.
::
:::

:::opt
`5` xuất hiện hai lần, nhưng phần tử của `day_chan` (dãy thứ hai) được
lấy trước
::why
Gần đúng ở việc bạn đếm đúng số lần `5` xuất hiện — hai lần, không sai.

Chỗ lệch nằm ở THỨ TỰ. Điều kiện `if day1[i] <= day2[j]:` dùng dấu
`<=` — nhỏ hơn HOẶC BẰNG. Khi `day1[i]` và `day2[j]` bằng nhau, điều
kiện này vẫn ĐÚNG, nên nhánh lấy từ `day1` (dãy thứ nhất) chạy trước,
không phải dãy thứ hai.
::
:::

:::opt
Máy báo lỗi, vì hai dãy có giá trị trùng nhau không được phép trộn với
nhau
::why
Gần đúng ở việc bạn nghi ngờ trùng lặp có thể gây rắc rối — sự cẩn trọng
đó không sai tinh thần trong lập trình nói chung.

Chỗ lệch: không có luật nào trong hàm này cấm hai dãy có giá trị trùng
nhau. Phép so sánh `<=` xử lý ĐƯỢC trường hợp bằng nhau một cách trơn
tru, không cần xử lý riêng, không có `Error` nào xảy ra.
::
:::
::::

::::code{#viet-buoc-tron}
Byte có hai dãy điểm đã tự sắp xếp riêng. Vòng lặp chính và hai vòng lặp
"chép nốt phần còn lại" đã có sẵn trong khung — bạn viết nốt hai chỗ,
đúng phần tử nào được lấy vào `ket_qua` ở mỗi nhánh.

```python title=starter
def tron_hai_day(day1, day2):
    ket_qua = []
    i = 0
    j = 0
    while i < len(day1) and j < len(day2):
        if day1[i] <= day2[j]:
            ket_qua.append(___)        # lấy phần tử ĐANG XÉT của day1
            i += 1
        else:
            ket_qua.append(___)        # lấy phần tử ĐANG XÉT của day2
            j += 1
    while i < len(day1):
        ket_qua.append(day1[i])
        i += 1
    while j < len(day2):
        ket_qua.append(day2[j])
        j += 1
    return ket_qua

day_le = [3, 8, 15, 21]
day_chan = [2, 4, 6, 9, 12, 30]

ket_qua = tron_hai_day(day_le, day_chan)
print(ket_qua)
```

```python title=solution
def tron_hai_day(day1, day2):
    ket_qua = []
    i = 0
    j = 0
    while i < len(day1) and j < len(day2):
        if day1[i] <= day2[j]:
            ket_qua.append(day1[i])
            i += 1
        else:
            ket_qua.append(day2[j])
            j += 1
    while i < len(day1):
        ket_qua.append(day1[i])
        i += 1
    while j < len(day2):
        ket_qua.append(day2[j])
        j += 1
    return ket_qua

day_le = [3, 8, 15, 21]
day_chan = [2, 4, 6, 9, 12, 30]

ket_qua = tron_hai_day(day_le, day_chan)
print(ket_qua)
```

```python title=test
assert ket_qua == [2, 3, 4, 6, 8, 9, 12, 15, 21, 30], f"trộn hai dãy phải ra đúng [2, 3, 4, 6, 8, 9, 12, 15, 21, 30] — đang ra {ket_qua}"
assert len(ket_qua) == len(day_le) + len(day_chan), "kết quả phải có ĐÚNG BẰNG tổng độ dài hai dãy đầu vào — không thừa, không thiếu phần tử nào"
assert ket_qua == sorted(ket_qua), "kết quả phải là một dãy TĂNG DẦN — đúng mục tiêu của việc trộn"
```

:::hints
- kind: attention
  body: Đừng chạm vào hai dòng i += 1 / j += 1, hay điều kiện day1[i] <= day2[j] — chúng đã đúng sẵn. Hai chỗ trống chỉ là GIÁ TRỊ được đưa vào ket_qua ở mỗi nhánh, khớp đúng dãy mà nhánh đó đang xử lý.
- kind: strategy
  body: 'Nhánh if (day1[i] <= day2[j]) đang xử lý trường hợp phần tử của day1 nhỏ hơn hoặc bằng — nên giá trị lấy vào ket_qua phải là day1[i]. Nhánh else xử lý trường hợp còn lại — lấy day2[j].'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `day1[i]` và `day2[j]`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hai chỗ trống phải THẬT SỰ lấy đúng phần tử đang xét của mỗi dãy — day1[i] ở nhánh if, day2[j] ở nhánh else — không phải một giá trị không liên quan như True, 1, 0
  requireAst:
  - kind: uses-name, target: day1, min: 5
  - kind: uses-name, target: day2, min: 5
  # min:5 cho MỖI tên — đếm thật (kiemAst) trên lời giải đúng. Trong khung
  # (không tính hai chỗ trống), day1 đã xuất hiện sẵn BỐN lần: điều kiện
  # "i < len(day1)" (vòng while chính), "day1[i] <= day2[j]" (if), vòng
  # "while i < len(day1):" (tail loop), và "ket_qua.append(day1[i])" bên
  # trong CHÍNH tail loop đó (đã cho sẵn, không phải chỗ trống). Lời giải
  # đúng thêm đúng 1 lần nữa ở chỗ trống thứ nhất, ra tổng 5. Đối xứng y
  # hệt cho day2 với chỗ trống thứ hai.
  #
  # ĐÃ THỬ THẬT: điền True/True (hoặc 1/0, hoặc 0/1) cho hai chỗ trống —
  # cả ba tổ hợp giữ nguyên day1 và day2 ở đúng 4 lần (không chạm blank),
  # dưới ngưỡng 5 — bị chặn ở CẢ HAI luật cùng lúc; điền đúng MỘT chỗ, sai
  # chỗ còn lại cũng bị chặn đúng bởi luật của TÊN bị bỏ sót (đúng bài học
  # "đếm cả hai tên", không chỉ tổng thể). Mọi tổ hợp chạy xong trong vài
  # mili-giây — an toàn, không có while nào mất điều kiện dừng, vì i += 1
  # và j += 1 (thứ quyết định vòng lặp có kết thúc hay không) đều nằm
  # NGOÀI chỗ trống, cố định trong khung.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\[2, 3, 4, 6, 8, 9, 12, 15, 21, 30\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mười phần tử, đúng tổng hai dãy, và không một lần nào phải xếp lại từ
đầu — chỉ so sánh đúng hai đầu dãy, mỗi lần một phép so sánh.
::::

::::reflect{#nghi-lai}
Một câu hỏi khép lại cả cụm bài này.

Bạn đã có đủ ba mảnh: một cách TÌM trong dữ liệu chưa sắp xếp (bài 13),
một cách TÌM nhanh hơn khi dữ liệu đã sắp xếp sẵn (bài 14-15), và một
khuôn chung CẮT-GIẢI-GHÉP (bài 16) cùng một bước Ghép biết trộn hai dãy
đã sắp thành một (bài này).

Nhưng có một câu hỏi chưa ai trả lời thẳng: làm sao BIẾT khi nào nên
dùng cách nào? Nếu dữ liệu trong tay CHƯA sắp xếp, có nên sắp xếp nó
trước rồi mới tìm nhị phân — hay cứ tìm tuyến tính cho xong?

Bài sau trả lời — và sau đó, ai sẽ đi SẮP XẾP dữ liệu chưa sắp xếp đó?
::::

::::checkpoint{mastery=0.8}
::::
