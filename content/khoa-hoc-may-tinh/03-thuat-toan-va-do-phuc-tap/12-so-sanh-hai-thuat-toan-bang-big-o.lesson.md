---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.so-sanh-hai-thuat-toan-bang-big-o
title: "So sánh hai thuật toán cùng việc bằng Big-O"
summary: "Bài chốt cụm, không khái niệm mới: cho hai đoạn mã cùng làm một việc, đếm bước thật (bài 7), quan sát hình dạng tăng (bài 8-9), gọi tên bằng Big-O (bài 10), rồi nói đoạn nào THUA ở ca xấu nhất (bài 11) — ghép cả bốn công cụ lại làm một."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [alg.compare-by-big-o]
requires: [alg.worst-case]
concepts: [alg.compare-by-big-o]
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
Bốn công cụ trong tay. Giờ ghép chúng lại, so hai đoạn mã thật.
::::

::::explain{#bon-buoc-so-sanh}
Không có khái niệm mới trong bài này — chỉ ghép lại đúng bốn thứ năm
bài trước đã dựng, thành một quy trình bốn bước để so sánh HAI đoạn mã
khác nhau cùng làm một việc:

1. **Đếm bước thật** cho cả hai đoạn mã, ở vài cỡ dữ liệu khác nhau
   (bài 7) — không đoán, không nhìn số dòng code mà suy ra tốc độ.
2. **Đo ở CA XẤU NHẤT**, không phải ca thuận lợi (bài 11) — chọn dữ
   liệu buộc cả hai đoạn mã phải làm nhiều việc nhất có thể.
3. **Quan sát hình dạng tăng** khi dữ liệu gấp đôi (bài 8-9) — số bước
   đứng yên, gấp đôi, hay gấp bốn?
4. **Gọi tên bằng Big-O** (bài 10), rồi so sánh hai nhãn — `O(n)` luôn
   thắng `O(n²)` khi dữ liệu ĐỦ LỚN, dù ở dữ liệu rất nhỏ hai bên có
   thể gần bằng nhau.

Xét một việc cụ thể: đếm xem một danh sách có TRÙNG LẶP hay không —
nghĩa là có ít nhất hai phần tử giống nhau. Hai cách viết khác hẳn nhau
cho ra cùng một câu trả lời đúng:

- **Cách A — so mọi cặp**: với mỗi phần tử, so nó với MỌI phần tử đứng
  sau nó, xem có cặp nào trùng không. Hai vòng lặp lồng nhau.
- **Cách B — nhớ những gì đã thấy**: đi qua danh sách đúng MỘT LƯỢT,
  giữ một `set` các giá trị đã gặp; mỗi phần tử mới chỉ phải tra xem nó
  đã có trong `set` đó chưa. Một vòng lặp duy nhất.
::::

::::example{#do-hai-cach-that}
Byte đo cả hai cách trên CÙNG một việc — TRƯỜNG HỢP XẤU NHẤT cho cả
hai: một danh sách KHÔNG có phần tử trùng lặp nào cả, buộc cả hai cách
phải xét trọn vẹn dữ liệu trước khi kết luận "không trùng":

```python title=readonly
def dem_trung_cach_a(mang):
    so_buoc = 0
    n = len(mang)
    for i in range(n):
        for j in range(i + 1, n):
            so_buoc += 1
    return so_buoc

def dem_trung_cach_b(mang):
    so_buoc = 0
    da_thay = set()
    for x in mang:
        so_buoc += 1
        da_thay.add(x)
    return so_buoc

mang_3 = list(range(3))
print("n=3  :  A =", dem_trung_cach_a(mang_3), " B =", dem_trung_cach_b(mang_3))

mang_10 = list(range(10))
print("n=10 :  A =", dem_trung_cach_a(mang_10), " B =", dem_trung_cach_b(mang_10))
```

```text title=readonly
n=3  :  A = 3  B = 3
n=10 :  A = 45  B = 10
```

Ở `n = 3`, hai cách BẰNG NHAU — 3 bước mỗi bên, không phân biệt được
cách nào tốt hơn. Nhưng ở `n = 10`, khoảng cách đã lộ rõ: cách A tốn 45
bước, cách B chỉ tốn 10 — gấp 4.5 lần. Cách A là hai vòng lặp lồng
nhau (bậc hai, `O(n²)`); cách B là một vòng lặp duy nhất (tuyến tính,
`O(n)`). Dữ liệu càng lớn, khoảng cách giữa `O(n²)` và `O(n)` càng
doãng rộng — đúng bài học "bốn bước so sánh" vừa nêu.
::::

::::predict{#doan-o-du-lieu-lon commitOnce}
Vẫn hai cách đếm trùng ở trên. Đã đo: ở `n = 3`, cả hai cho CÙNG một
con số (3 bước mỗi bên).

**Trước khi đọc tiếp**, bạn đoán: ở `n = 1000`, chuyện gì xảy ra?

:::opt{correct}
Cách B (O(n)) sẽ tốn ÍT bước hơn HẲN cách A (O(n²)) — khoảng cách giữa
hai nhãn Big-O càng lớn thì càng lộ rõ khi dữ liệu càng lớn, đúng như
đã thấy khi n tăng từ 3 lên 10
:::

:::opt
Vẫn bằng nhau — vì cả hai cách đều làm CÙNG một việc (đếm xem có trùng
lặp hay không), nên số bước phải như nhau ở mọi cỡ dữ liệu
::why
Gần đúng ở việc hai cách đúng là cùng MỤC ĐÍCH — cả hai đều trả lời
đúng câu hỏi "danh sách này có trùng lặp không", không cách nào sai.

Chỗ lệch: CÙNG mục đích không có nghĩa CÙNG cách làm, và CÙNG cách làm
mới quyết định số bước. Cách A phải so MỌI CẶP (tăng theo bình phương),
cách B chỉ dùng MỘT LƯỢT (tăng tuyến tính) — hai hình dạng khác nhau
càng doãng rộng khi `n` lớn, đúng như bảng số liệu ở `n=3` và `n=10`
đã bắt đầu hé lộ (bằng nhau ở 3, cách xa ở 10).
::
:::

:::opt
Cách A sẽ nhanh hơn — vì đoạn mã của nó ngắn gọn, đơn giản hơn cách B
(không cần tạo `set`)
::why
Gần đúng ở việc nhận xét ĐỘ NGẮN GỌN của mã nguồn — cách A quả thật ít
dòng hơn, không dùng thêm cấu trúc dữ liệu nào khác ngoài hai vòng lặp.

Chỗ lệch: Big-O đo số BƯỚC máy phải làm khi CHẠY, không đo số DÒNG code
khi VIẾT. Một đoạn mã ngắn hoàn toàn có thể làm máy chạy nhiều bước hơn
một đoạn mã dài hơn — đúng như bảng số liệu vừa đo: cách A ít dòng hơn
cách B, nhưng lại tốn NHIỀU bước hơn hẳn ở `n=10` (45 so với 10).
::
:::

:::opt
Không so sánh được nếu chưa biết nội dung cụ thể của danh sách 1000
phần tử đó
::why
Gần đúng ở việc dữ liệu cụ thể đúng là ảnh hưởng tới số bước thật của
nhiều đoạn mã — bài trước (ca thuận lợi và ca xấu nhất) đã cho thấy rõ
điều này.

Chỗ lệch: câu hỏi ở đây đã CHỌN SẴN ca xấu nhất cho cả hai cách — một
danh sách không có phần tử trùng lặp, buộc cả A lẫn B phải xét hết dữ
liệu. Với ca xấu nhất đã cố định, hình dạng Big-O của mỗi cách
(`O(n²)` với `O(n)`) đã đủ để kết luận cách nào thắng khi `n` đủ lớn,
không cần biết thêm nội dung cụ thể của từng phần tử.
::
:::
::::

::::code{#ai-thang-o-du-lieu-lon}
Đo cả hai cách (đã cho sẵn — readonly) trên hai cỡ dữ liệu khác nhau,
`n=10` (nhỏ) và `n=100` (lớn), rồi TỰ QUYẾT ĐỊNH bằng phép so sánh số
bước xem cách nào thắng ở dữ liệu LỚN.

```python title=starter
def dem_trung_cach_a(mang):
    so_buoc = 0
    n = len(mang)
    for i in range(n):
        for j in range(i + 1, n):
            so_buoc += 1
    return so_buoc

def dem_trung_cach_b(mang):
    so_buoc = 0
    da_thay = set()
    for x in mang:
        so_buoc += 1
        da_thay.add(x)
    return so_buoc

mang_nho = list(range(10))
mang_lon = list(range(100))

buoc_a_nho = dem_trung_cach_a(mang_nho)
buoc_a_lon = dem_trung_cach_a(mang_lon)
buoc_b_nho = dem_trung_cach_b(mang_nho)
buoc_b_lon = dem_trung_cach_b(mang_lon)

thang_o_du_lieu_lon = ___          # "B" nếu buoc_b_lon ít bước hơn, ngược lại "A"

print(f"nhỏ (n=10): A={buoc_a_nho}, B={buoc_b_nho}")
print(f"lớn (n=100): A={buoc_a_lon}, B={buoc_b_lon}")
print(f"thắng ở dữ liệu lớn: {thang_o_du_lieu_lon}")
```

```python title=solution
def dem_trung_cach_a(mang):
    so_buoc = 0
    n = len(mang)
    for i in range(n):
        for j in range(i + 1, n):
            so_buoc += 1
    return so_buoc

def dem_trung_cach_b(mang):
    so_buoc = 0
    da_thay = set()
    for x in mang:
        so_buoc += 1
        da_thay.add(x)
    return so_buoc

mang_nho = list(range(10))
mang_lon = list(range(100))

buoc_a_nho = dem_trung_cach_a(mang_nho)
buoc_a_lon = dem_trung_cach_a(mang_lon)
buoc_b_nho = dem_trung_cach_b(mang_nho)
buoc_b_lon = dem_trung_cach_b(mang_lon)

thang_o_du_lieu_lon = "B" if buoc_b_lon < buoc_a_lon else "A"

print(f"nhỏ (n=10): A={buoc_a_nho}, B={buoc_b_nho}")
print(f"lớn (n=100): A={buoc_a_lon}, B={buoc_b_lon}")
print(f"thắng ở dữ liệu lớn: {thang_o_du_lieu_lon}")
```

```python title=test
assert buoc_a_nho == 45, f"cách A trên 10 phần tử phải tốn 45 bước, đang ra {buoc_a_nho}"
assert buoc_a_lon == 4950, f"cách A trên 100 phần tử phải tốn 4950 bước, đang ra {buoc_a_lon}"
assert buoc_b_nho == 10, f"cách B trên 10 phần tử phải tốn 10 bước, đang ra {buoc_b_nho}"
assert buoc_b_lon == 100, f"cách B trên 100 phần tử phải tốn 100 bước, đang ra {buoc_b_lon}"
assert thang_o_du_lieu_lon == "B", f"ở dữ liệu lớn (n=100), cách B (O(n)) phải thắng cách A (O(n²)) — đang báo '{thang_o_du_lieu_lon}'"
```

:::hints
- kind: attention
  body: Chỗ trống là một biểu thức so sánh THẬT giữa buoc_b_lon và buoc_a_lon (dữ liệu LỚN, n=100) — không phải giữa buoc_b_nho và buoc_a_nho (dữ liệu nhỏ), vì câu hỏi đang hỏi ai thắng khi dữ liệu ĐỦ LỚN.
- kind: strategy
  body: 'Dùng biểu thức có điều kiện: "B" if buoc_b_lon < buoc_a_lon else "A" — so hai số bước đo được ở n=100, không gõ thẳng chữ "B" vào. Cách viết if/else nhiều dòng cũng được, miễn là kết quả cuối cùng đúng bằng phép so sánh thật giữa hai con số đã đo.'
- kind: one-line
  body: 'Chỗ trống là `"B" if buoc_b_lon < buoc_a_lon else "A"`.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: chỗ trống phải THẬT SỰ so sánh buoc_b_lon với buoc_a_lon bằng một phép so sánh (ví dụ <) — không được gõ thẳng chữ "B" vào, và không được là một câu không làm gì như True, 1, 0
  requireAst:
  - kind: uses-name, target: buoc_b_lon, min: 2
  - kind: uses-name, target: buoc_a_lon, min: 2
  # đếm thật trên solution: buoc_b_lon được ĐỌC đúng hai lần (một lần
  # trong phép so sánh ở chỗ trống, một lần trong f-string in kết quả);
  # buoc_a_lon cũng vậy (một lần so sánh, một lần f-string) = 2 mỗi
  # biến. Điền True/1/0 vào chỗ trống xoá mất lần đọc trong phép so
  # sánh, tụt CẢ HAI xuống 1 (<2) — luật chặn được cả hai phía, không
  # chỉ một phía (bài học từ T3.1/T3.2: một luật chỉ đếm MỘT trong hai
  # tên tham gia so sánh thì đổi tên một phía vẫn lọt). ĐÃ THỬ THẬT cả
  # ba cách True/1/0: cả ba dừng NGAY (không có while/đệ quy trong toàn
  # bộ khối này, chỉ là các vòng for bị chặn bởi range cố định) và cả
  # ba cho thang_o_du_lieu_lon == "True"/"1"/"0" (không phải chuỗi hợp
  # lệ "A" hay "B"), sai hẳn, bị tests bắt độc lập. Đã thử thêm một lời
  # giải ĐÚNG khác — viết if/else nhiều dòng thay vì biểu thức một dòng
  # — vẫn đọc buoc_b_lon và buoc_a_lon đủ số lần cần thiết, luật không
  # đánh trượt nhầm.
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: "^nhỏ \\(n=10\\): A=45, B=10\\nlớn \\(n=100\\): A=4950, B=100\\nthắng ở dữ liệu lớn: B\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
45 so với 10, rồi 4950 so với 100 — khoảng cách càng doãng rộng khi dữ
liệu càng lớn. Đúng bốn công cụ, ghép lại thành một kết luận chắc chắn.
::::

::::reflect{#nghi-lai}
Một câu hỏi khép lại cả cụm bài này.

Suốt cụm vừa qua, một thao tác cứ lặp đi lặp lại đứng sau nhiều ví dụ:
đi từ ô 0 tới hết, so từng ô, không bỏ sót ô nào — chính là
`do_tung_o` của bài 11, dùng lại để đo ca thuận lợi và ca xấu nhất.
Đứng riêng, thao tác ấy chưa từng có một cái tên chính thức — chỉ được
tả dài dòng mỗi lần gặp lại.

Đã đến lúc đặt tên chính thức cho đúng thao tác đó chưa?
::::

::::checkpoint{mastery=0.8}
::::
