---
id: khoa-hoc-may-tinh.may-chay-the-nao.vi-sao-mang-lien-ke-nhanh-hon-that
title: "Vì sao mảng liền kề nhanh hơn — THẬT, không chỉ lý thuyết"
summary: "T3.2 so mảng với danh sách liên kết bằng LÝ THUYẾT (số bước). Bài này trả lời PHẦN CỨNG: đọc ô kề bên trong mảng thường đã nằm sẵn trong bộ nhớ đệm vì máy tải cả dải liền kề một lượt; đuổi theo con trỏ trong danh sách liên kết thường buộc chờ RAM mỗi bước — cùng độ phức tạp lý thuyết, khác hẳn tốc độ thật, đo được."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.cache-locality]
requires: [may.ram-vs-cache-speed, ds.array-vs-linked-tradeoff]
concepts: [may.cache-locality]
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
Không như nhau — và lần này không cần tin lý thuyết suông, đo được thật.
::::

::::explain{#ly-thuyet-khac-thuc-te}
Bài "Mảng và danh sách liên kết: đổi cái gì lấy cái gì" (T3.2) so hai cấu
trúc bằng SỐ BƯỚC — đi qua từng phần tử của một dãy `n` giá trị, cả mảng
lẫn danh sách liên kết đều mất đúng `n` bước, không hơn không kém. Về mặt
lý thuyết, ĐỌC HẾT một dãy nhanh như nhau ở cả hai cấu trúc.

Bài này trả lời câu hỏi khác: tốc độ THẬT, đo bằng đồng hồ, có giống lý
thuyết không? Câu trả lời là KHÔNG — và lý do nằm đúng ở bài 17 vừa dạy:
tính cục bộ theo KHÔNG GIAN (spatial locality).

Một mảng bắt buộc các phần tử nằm LIỀN NHAU trong bộ nhớ — địa chỉ của
phần tử thứ hai luôn ngay sau địa chỉ phần tử thứ nhất (T3.1, T3.2 đã
dạy). Khi CPU trượt bộ nhớ đệm (bài 18) và phải sang RAM lấy MỘT phần
tử, phần cứng thường không lấy đúng MỘT giá trị lẻ loi — nó lấy nguyên
một DẢI byte liền kề quanh địa chỉ đó, đưa cả dải vào bộ nhớ đệm một
lượt. Với mảng, dải đó tình cờ CHÍNH LÀ vài phần tử tiếp theo bạn sắp
đọc — bộ nhớ đệm "đoán trúng" gần như miễn phí.

Một danh sách liên kết thì khác hẳn: mỗi nút được cấp phát ở một chỗ bất
kỳ trên đống (không cần liền nhau — đúng điều T3.2 nói "không cần liền
kề" khi so sánh hai cấu trúc). Đi từ nút này sang nút kế bằng trường
`"tiep"` gần như CHẮC CHẮN nhảy sang một địa chỉ xa, không nằm trong dải
vừa tải. Mỗi bước đều có nguy cơ trượt bộ nhớ đệm lại — đúng cái giá bài
18 vừa đo.

Cùng `n` bước về lý thuyết. Khác hẳn nhau về việc mỗi bước có phải chờ
RAM hay không.
::::

::::example{#do-that-mang-vs-lien-ket}
Dựng một mảng và một danh sách liên kết CÙNG giữ đúng những giá trị như
nhau — chỉ khác cách các nút được NỐI: danh sách liên kết nối theo một
thứ tự đã bị XÁO TRỘN so với thứ tự chúng được tạo ra trong bộ nhớ, y
hệt cách một chương trình thật cấp phát và nối các nút không theo thứ tự
liền mạch:

```python title=readonly
import time, random

N = 50_000

mang = list(range(N))

nut = [{"gia_tri": i, "tiep": None} for i in range(N)]
thu_tu = list(range(N))
random.seed(42)
random.shuffle(thu_tu)
for k in range(N - 1):
    nut[thu_tu[k]]["tiep"] = nut[thu_tu[k + 1]]
dau = nut[thu_tu[0]]

def tong_mang(m):
    tong = 0
    for x in m:
        tong += x
    return tong

def tong_lien_ket(d):
    tong = 0
    hien = d
    while hien is not None:
        tong += hien["gia_tri"]
        hien = hien["tiep"]
    return tong

t0 = time.perf_counter()
tong_mang(mang)
t1 = time.perf_counter()

t2 = time.perf_counter()
tong_lien_ket(dau)
t3 = time.perf_counter()

print(f"mảng: {t1 - t0:.5f} giây")
print(f"danh sách liên kết: {t3 - t2:.5f} giây")
print(f"tỉ lệ: {(t3 - t2) / (t1 - t0):.2f} lần")
```

```text title=readonly
mảng: 0.00320 giây
danh sách liên kết: 0.01169 giây
tỉ lệ: 3.66 lần
```

Cả hai đều đọc đúng `N` giá trị — cùng số bước, đúng như T3.2 tính. Nhưng
danh sách liên kết chậm hơn gấp mấy lần, đo lặp lại nhiều lần vẫn ra cùng
chiều. Con số chính xác dao động theo máy chạy, nhưng chiều thì không đổi
— mảng liền kề luôn thắng khi đọc tuần tự, vì bộ nhớ đệm tải sẵn được cả
dải kề nhau; danh sách liên kết thì không có gì để đoán trước, vì nút kế
tiếp có thể nằm ở bất cứ đâu trên đống.
::::

::::predict{#doan-cai-nao-nhanh-hon commitOnce}
Byte đọc HẾT một dãy 10.000 giá trị theo đúng thứ tự, hai lần — một lần
giữ trong mảng, một lần giữ trong danh sách liên kết mà thứ tự nối KHÔNG
trùng thứ tự các nút nằm trong bộ nhớ.

T3.2 đã tính: cả hai đều tốn đúng 10.000 bước. Vậy về THỜI GIAN THẬT đo
bằng đồng hồ, điều gì đúng?

:::opt{correct}
Mảng nhanh hơn — vì các phần tử của nó nằm liền nhau, bộ nhớ đệm tải sẵn
được cả dải trong một lượt, còn danh sách liên kết thì không
:::

:::opt
Như nhau — vì T3.2 đã chứng minh cả hai tốn đúng số bước bằng nhau
::why
Gần đúng ở việc bạn nhớ ĐÚNG kết luận của T3.2 — cùng `n` bước, không
sai.

Chỗ lệch: "cùng số bước" là một câu về LÝ THUYẾT — đếm THAO TÁC logic,
không đếm THỜI GIAN thật mỗi thao tác tốn. Bài này vừa đo bằng đồng hồ
thật: cùng số bước, nhưng khác hẳn tốc độ, vì một số bước phải CHỜ RAM
(bài 18) còn số khác thì không.
::
:::

:::opt
Danh sách liên kết nhanh hơn — vì nó không cần dời chỗ phần tử nào khi
đọc, còn mảng phải kiểm tra biên mỗi lần
::why
Gần đúng ở việc bạn nhớ đúng một lợi thế THẬT của danh sách liên kết —
chèn/xoá không cần dời chỗ (T3.2 bài 17). Lợi thế đó có thật.

Chỗ lệch: lợi thế đó chỉ áp dụng cho việc CHÈN/XOÁ, không áp dụng cho
việc ĐỌC TUẦN TỰ mà bài này đang so. Đọc tuần tự không hề "dời chỗ" ở
mảng — nó chỉ tận dụng việc các ô đã nằm sẵn cạnh nhau, đúng lợi thế
ngược lại với điều bạn vừa nêu.
::
:::

:::opt
Không thể biết trước — tốc độ phụ thuộc hoàn toàn vào máy đang chạy, mỗi
lần đo có thể ra kết quả khác hẳn
::why
Gần đúng ở việc CON SỐ chính xác thật sự dao động theo máy — điều đó
đúng, ví dụ vừa nêu ở trên cũng nói rõ điều này.

Chỗ lệch: dù con số dao động, CHIỀU của kết quả không đổi — mảng liền kề
LUÔN thắng khi đọc tuần tự, trên mọi máy có bộ nhớ đệm (tức là mọi máy
hiện đại). Đây không phải may rủi, mà là hệ quả trực tiếp của cách bộ
nhớ đệm hoạt động (bài 17).
::
:::
::::

::::code{#tinh-tong-lien-ket}
Hoàn thành `tong_lien_ket`: đi qua từng nút bắt đầu từ `dau`, cộng dồn
giá trị của MỖI nút đi qua.

```python title=starter
mang = [10, 20, 30, 40, 50]

nut5 = {"gia_tri": 50, "tiep": None}
nut4 = {"gia_tri": 40, "tiep": nut5}
nut3 = {"gia_tri": 30, "tiep": nut4}
nut2 = {"gia_tri": 20, "tiep": nut3}
nut1 = {"gia_tri": 10, "tiep": nut2}

def tong_mang(m):
    tong = 0
    for x in m:
        tong += x
    return tong

def tong_lien_ket(dau):
    tong = 0
    hien = dau
    while hien is not None:
        tong += ___                # cộng giá trị đang đứng tại nút hiện tại
        hien = hien["tiep"]
    return tong

print(tong_mang(mang))
print(tong_lien_ket(nut1))
```

```python title=solution
mang = [10, 20, 30, 40, 50]

nut5 = {"gia_tri": 50, "tiep": None}
nut4 = {"gia_tri": 40, "tiep": nut5}
nut3 = {"gia_tri": 30, "tiep": nut4}
nut2 = {"gia_tri": 20, "tiep": nut3}
nut1 = {"gia_tri": 10, "tiep": nut2}

def tong_mang(m):
    tong = 0
    for x in m:
        tong += x
    return tong

def tong_lien_ket(dau):
    tong = 0
    hien = dau
    while hien is not None:
        tong += hien["gia_tri"]
        hien = hien["tiep"]
    return tong

print(tong_mang(mang))
print(tong_lien_ket(nut1))
```

```python title=test
assert tong_mang(mang) == 150, f"10+20+30+40+50 = 150 — đang ra {tong_mang(mang)}"
assert tong_lien_ket(nut1) == 150, f"danh sách liên kết giữ đúng năm giá trị như mang — phải cộng ra 150 — đang ra {tong_lien_ket(nut1)}"
assert tong_lien_ket(nut3) == 120, f"bắt đầu từ nut3 chỉ còn đi qua 30, 40, 50 = 120 — đang ra {tong_lien_ket(nut3)}"
```

:::hints
- kind: attention
  body: Dòng bước sang nút kế (hien = hien["tiep"]) đã có sẵn trong khung, không được đụng vào. Chỗ trống chỉ lo việc CỘNG giá trị của nút hiện tại.
- kind: strategy
  body: 'Giá trị của nút đang đứng nằm ở khoá "gia_tri" của hien — đúng lối bạn đã quen từ T3.2: hien["gia_tri"].'
- kind: one-line
  body: 'Chỗ trống là: hien["gia_tri"]'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ đọc gia_tri của nút hiện tại (hien["gia_tri"]) — không được gõ thẳng một con số, vì bài này đang dạy đúng việc đi qua TỪNG NÚT thật, không phải đoán trước tổng
  requireAst:
  # Đếm thật trên lời giải: hien đọc 3 lần — điều kiện while (có sẵn trong
  # khung), RHS của hien = hien["tiep"] (có sẵn trong khung), và trong
  # chỗ trống. Hai lần đầu LUÔN có mặt bất kể chỗ trống điền gì (vì nằm
  # ngoài chỗ trống) — nên hollow fill (999, True, 0,...) chỉ còn đếm được
  # 2, dưới min:3. Một lời giải đúng khác (tong = tong + hien["gia_tri"])
  # vẫn đọc hien đúng 1 lần trong chỗ trống, tổng vẫn 3.
  - kind: uses-name, target: hien, min: 3
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^150\\n150\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng năm giá trị, cùng tổng 150 — chỉ khác cách bộ nhớ đệm nhìn thấy
chúng: liền một dải, hay rải rác khắp đống.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mảng và danh sách liên kết cả hai đều sống trong RAM — khác nhau chỉ ở
CÁCH RAM được chạm tới. Nhưng RAM tự nó chỉ giữ được dữ liệu trong lúc
máy còn BẬT NGUỒN.

R1.T1.5 đã cho bạn mở file, đọc và ghi dữ liệu — dữ liệu đó nằm Ở ĐÂU,
nếu không phải RAM? Và đọc từ đó có nhanh như đọc một biến đã có sẵn
trong bộ nhớ không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
