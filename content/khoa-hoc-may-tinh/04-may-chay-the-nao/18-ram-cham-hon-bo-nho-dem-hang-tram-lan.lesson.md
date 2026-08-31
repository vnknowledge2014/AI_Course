---
id: khoa-hoc-may-tinh.may-chay-the-nao.ram-cham-hon-bo-nho-dem-hang-tram-lan
title: "RAM chậm hơn bộ nhớ đệm hàng trăm lần"
summary: "Khi CPU cần một giá trị KHÔNG có sẵn trong bộ nhớ đệm, nó phải sang RAM lấy — và khoảng cách tốc độ đó lớn cỡ hàng trăm lần, một sự thật phần cứng có thật, không phải phóng đại. Con số này là lý do cả cụm bài này tồn tại."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.ram-vs-cache-speed]
requires: [may.cache]
concepts: [may.ram-vs-cache-speed]
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
Nó phải đi hết quãng đường tới RAM — và quãng đường đó dài hơn bạn tưởng
rất nhiều.
::::

::::explain{#khi-doan-sai}
Bộ nhớ đệm (bài trước) đoán khá giỏi, nhưng không đoán đúng mãi. Khi CPU
cần một giá trị KHÔNG có sẵn trong bộ nhớ đệm — gọi là một lần **trượt**
bộ nhớ đệm (cache miss) — nó không còn lựa chọn nào khác ngoài việc đi
tiếp, sang tận RAM, làm đúng bước "gửi địa chỉ rồi đợi" mà bài T3.1 đã
dạy.

Khoảng cách tốc độ giữa hai chặng đó lớn tới mức nào? Đây là một sự thật
đo được về PHẦN CỨNG thật, không phụ thuộc Python hay Pyodide: đọc một
giá trị đã có sẵn ở tầng gần CPU nhất mất cỡ MỘT đơn vị thời gian rất
nhỏ; đọc đúng giá trị đó, nếu phải đi hết đường tới RAM, mất một khoảng
lớn hơn cỡ MỘT ĐẾN VÀI TRĂM LẦN. Không phải một cách nói phóng đại — đó
là khoảng cách thật giữa "ngay bên cạnh bộ phận tính toán" (bài 16, 17)
và "phải gửi tín hiệu ra ngoài chip, đợi mạch nhớ trả lời".

Cần nói thẳng một điều: khoảng cách chính xác đó KHÔNG đo được trực tiếp
bằng `time.perf_counter()` trong Python chạy trên Pyodide. Giữa mã Python
và con chip thật có rất nhiều tầng chồng lên nhau — trình thông dịch
(bài 8), bộ quản lý bộ nhớ, và cả lớp mô phỏng WASM của chính sandbox này
— mỗi tầng cộng thêm phần chi phí riêng, đủ để nuốt chửng phần chênh lệch
thuần tuý phần cứng mà bài này đang nói tới. Đo trực tiếp bằng Python là
đo SAI THỨ cần đo.

Nhưng một DẤU VẾT của khoảng cách đó vẫn hiện ra được, dù bị che bớt rất
nhiều — xem ví dụ dưới.
::::

::::example{#dau-vet-do-duoc}
So sánh thời gian đọc NGẪU NHIÊN nhiều lần từ một mảng nhỏ (mong nằm gọn
trong bộ nhớ đệm) với một mảng lớn hơn nhiều (chắc chắn vượt quá, buộc
CPU phải trượt ra RAM thường xuyên hơn):

```python title=readonly
import time, random

N_NHO = 1_000
N_LON = 300_000

mang_nho = list(range(N_NHO))
mang_lon = list(range(N_LON))

random.seed(7)
LUOT = 200_000
chi_so_nho = [random.randrange(N_NHO) for _ in range(LUOT)]
chi_so_lon = [random.randrange(N_LON) for _ in range(LUOT)]

def doc_ngau_nhien(mang, chi_so_list):
    tong = 0
    for chi_so in chi_so_list:
        tong += mang[chi_so]
    return tong

t0 = time.perf_counter()
doc_ngau_nhien(mang_nho, chi_so_nho)
t1 = time.perf_counter()

t2 = time.perf_counter()
doc_ngau_nhien(mang_lon, chi_so_lon)
t3 = time.perf_counter()

print(f"mảng nhỏ: {t1 - t0:.5f} giây")
print(f"mảng lớn: {t3 - t2:.5f} giây")
print(f"tỉ lệ: {(t3 - t2) / (t1 - t0):.2f} lần")
```

```text title=readonly
mảng nhỏ: 0.01306 giây
mảng lớn: 0.02745 giây
tỉ lệ: 2.10 lần
```

Chỉ khoảng 2 lần chậm hơn — RẤT xa con số "hàng trăm lần" của phần cứng
thật. Đúng như dự đoán: mọi tầng phần mềm chạy phía trên (Python, trình
thông dịch, sandbox) đã che gần hết chênh lệch thuần phần cứng. Nhưng
chiều của nó vẫn đúng, và đo lại nhiều lần vẫn cho cùng một chiều — mảng
lớn hơn LUÔN chậm hơn mảng nhỏ khi truy cập ngẫu nhiên. Con số chính xác
"hàng trăm lần" là sự thật ở tầng silicon, dưới cả tầng mà Python chạm
tới được.
::::

::::predict{#khi-trot-bo-nho-dem commitOnce}
CPU cần đọc một giá trị, nhưng giá trị đó KHÔNG có sẵn trong bộ nhớ đệm —
một lần trượt (cache miss).

Chuyện gì xảy ra tiếp theo?

:::opt{correct}
CPU phải gửi địa chỉ sang RAM và đợi câu trả lời — chậm hơn lúc có sẵn
trong bộ nhớ đệm cỡ một tới vài trăm lần
:::

:::opt
Chậm hơn một chút, nhưng không đáng để bận tâm khi viết chương trình
::why
Gần đúng ở việc bạn thừa nhận CÓ chậm hơn — điều đó đúng.

Chỗ lệch là quy mô: khoảng cách này KHÔNG phải "một chút". Nó lớn tới
mức cả cụm bài học này (bài 16-21) dựng lên chỉ để nói về nó — một
chương trình chạm RAM liên tục, thay vì tận dụng bộ nhớ đệm, có thể chậm
đi rõ rệt, không phải chuyện lý thuyết suông.
::
:::

:::opt
CPU tính lại giá trị đó từ đầu, thay vì đọc từ RAM, để khỏi phải đợi
::why
Gần đúng ở tinh thần "tìm đường tắt để khỏi đợi" — một bản năng hợp lý
khi nghĩ về tốc độ.

Chỗ lệch: một giá trị đã LƯU sẵn (biến, phần tử mảng,...) không phải thứ
CPU "tính lại" được — nó phải THẬT SỰ đọc đúng ô nhớ đang giữ giá trị
đó. Không có đường tắt nào giả vờ biết trước một giá trị đã lưu mà không
đọc nó.
::
:::

:::opt
Chương trình dừng lại và báo lỗi, vì bộ nhớ đệm không có sẵn giá trị cần
::why
Gần đúng ở việc nhận ra có một khoảnh khắc "thiếu" xảy ra thật.

Chỗ lệch: trượt bộ nhớ đệm không phải lỗi — nó xảy ra liên tục, trong
MỌI chương trình, mọi lúc. CPU chỉ đơn giản làm chậm hơn ở lần đọc đó,
rồi tiếp tục chạy bình thường — không có gì "hỏng" cả.
::
:::
::::

::::code{#doc-theo-chi-so}
Hoàn thành `doc_ngau_nhien`: với mỗi `chi_so` trong `chi_so_list`, cộng
dồn đúng phần tử của `mang` nằm Ở VỊ TRÍ đó.

```python title=starter
mang_nho = list(range(1000))

def doc_ngau_nhien(mang, chi_so_list):
    tong = 0
    for chi_so in chi_so_list:
        tong += mang[___]          # đọc đúng Ô mà chi_so đang trỏ tới
    return tong

chi_so_thu = [37, 802, 5, 999, 640]
print(doc_ngau_nhien(mang_nho, chi_so_thu))
```

```python title=solution
mang_nho = list(range(1000))

def doc_ngau_nhien(mang, chi_so_list):
    tong = 0
    for chi_so in chi_so_list:
        tong += mang[chi_so]
    return tong

chi_so_thu = [37, 802, 5, 999, 640]
print(doc_ngau_nhien(mang_nho, chi_so_thu))
```

```python title=test
assert doc_ngau_nhien(mang_nho, chi_so_thu) == 2483, f"37+802+5+999+640 = 2483 (vì mang_nho[i] == i) — đang ra {doc_ngau_nhien(mang_nho, chi_so_thu)}"
assert doc_ngau_nhien(mang_nho, [0, 1, 2]) == 3, f"chỉ số 0,1,2 phải cộng ra 0+1+2 = 3 — đang ra {doc_ngau_nhien(mang_nho, [0, 1, 2])}"
```

:::hints
- kind: attention
  body: Chỗ trống nằm NGAY TRONG cặp ngoặc vuông của mang[...] — nó phải là chỉ số, không phải giá trị.
- kind: strategy
  body: 'chi_so là biến vòng lặp đang giữ đúng vị trí cần đọc lúc đó. Đọc đúng ô ở vị trí ấy: mang[chi_so].'
- kind: one-line
  body: 'Chỗ trống là: chi_so'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải dùng đúng biến chi_so làm chỉ số — không được gõ một con số cố định vào đó, vì bài này đang dạy đọc THEO chỉ số thay đổi, không phải đọc một ô cố định
  requireAst:
  # Đếm thật trên lời giải: chi_so đọc đúng 1 lần (bên trong mang[chi_so]) —
  # lần xuất hiện ở "for chi_so in ..." là Store, không tính. min:1 chặn
  # mọi cách điền hằng số cố định (mang[0], mang[True],...) vì hằng số
  # không đọc tên chi_so lần nào.
  - kind: uses-name, target: chi_so, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: "2483"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đúng năm vị trí, đúng năm lần CPU phải nhắm đúng ô — dù ô đó có nằm sẵn
trong bộ nhớ đệm hay phải đi xa hơn tới RAM.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hai đoạn mã bài này so sánh đều là MẢNG — chỉ khác NHAU về kích thước.
Nhưng T3.2 còn dạy một cấu trúc khác để giữ một dãy giá trị: danh sách
liên kết, nơi các phần tử KHÔNG cần nằm liền nhau trong bộ nhớ.

Nếu cùng đọc hết một dãy giá trị — một lần bằng mảng, một lần bằng danh
sách liên kết — hai cách đó có tốc độ NHƯ NHAU không? Cùng một số bước
về mặt lý thuyết (T3.2 đã đo), nhưng về tốc độ THẬT thì sao?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
