---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.boss-chon-dung-thuat-toan
title: "BOSS — Chọn đúng thuật toán, đo bằng Big-O, chứng minh bằng đếm bước"
summary: "Không khái niệm mới — vài bài toán thật (sắp xếp danh sách lớn, tìm đường trên bản đồ, tính Fibonacci lặp lại), mỗi cái phải CHỌN đúng thuật toán, rồi TỰ ĐO bằng bộ đếm bước để chứng minh lựa chọn đúng, không đoán."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 34
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [alg.choose-algorithm]
requires: [alg.memoization, alg.greedy-fails, alg.bfs-vs-dfs, alg.merge-sort-complexity, ds.choose-structure]
concepts: [alg.choose-algorithm]
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
Ba mươi ba bài để tới đây. Không bài mới nào dạy thêm hôm nay — chỉ hỏi:
gặp một việc thật, bạn có rút đúng công cụ ra không, và có tự ĐO được để
chứng minh mình đúng không?
::::

::::explain{#nam-tinh-huong-that}
Không có khái niệm mới trong bài này. Năm tình huống dưới đây đều dùng
đúng những công cụ track này đã mở nắp — việc của bạn là chọn đúng, và
biết vì sao chọn SAI lại hỏng.

1. **Sắp xếp danh sách khách hàng, RẤT LỚN** (hàng chục nghìn dòng) theo
   tên. Đúng: **sắp xếp trộn** — O(n log n). Sai — chọn nổi bọt, chọn,
   hay chèn: cả ba đều O(n²), và với n lớn, khoảng cách giữa n log n và
   n² không còn là chuyện nhỏ — nó là chuyện chờ vài giây hay chờ vài
   giờ.

2. **Tìm đường đi dùng ÍT TUYẾN ĐƯỜNG NHẤT** giữa hai thành phố trên một
   bản đồ không ghi độ dài từng tuyến. Đúng: **BFS**. Sai — chọn DFS: DFS
   vẫn tìm ra MỘT đường đi nếu có, nhưng không có gì đảm bảo đó là đường
   ít tuyến nhất — nó có thể đi lạc theo một nhánh dài trước khi quay
   lại.

3. **Tính Fibonacci của nhiều số lớn, có khả năng lặp lại số đã hỏi.**
   Đúng: **đệ quy có ghi nhớ**. Sai — chọn đệ quy trần trụi: đúng kết
   quả, nhưng số lượt tính thật tăng theo cấp số nhân, trong khi bản có
   sổ chỉ cần tính đúng bằng số giá trị khác nhau từng được hỏi.

4. **Trả tiền thừa với hệ mệnh giá THƯỜNG GẶP** (kiểu 1, 2, 5, 10, 20,
   50...). Đúng: **tham lam** — nhanh, và với hệ mệnh giá này luôn ra ít
   đồng nhất. Cảnh báo đi kèm: tham lam chỉ đáng tin SAU KHI đã tự kiểm
   chứng hệ mệnh giá, không phải một luật đúng với mọi bộ mệnh giá bất
   kỳ.

5. **Tra cứu thông tin theo tên, lặp lại rất nhiều lần** — một danh bạ.
   Đúng: **bảng băm** (`dict`). Sai — chọn mảng chưa sắp xếp: mỗi lần tra
   phải dò từng mục, không có phép tính nào nhảy thẳng tới đúng chỗ như
   bảng băm làm được.

Không tình huống nào cần một thuật toán CHƯA từng học. Cái khó không phải
"nhớ công thức" — cái khó là ĐỌC ĐÚNG tình huống để biết công thức nào áp
được, và biết CHỨNG MINH lựa chọn đó bằng số đo được, không phải bằng
cảm giác "chắc là đúng".
::::

::::example{#do-that-tinh-huong-1}
Xét kỹ tình huống 1. Byte đếm THẬT số phép so sánh của nổi bọt và của
trộn, trên cùng một danh sách đã đảo ngược (ca xấu nhất), ở ba cỡ dữ liệu
tăng dần:

```python title=readonly
def dem_so_sanh_noi_bot(mang):
    mang = mang[:]
    so_sanh = 0
    n = len(mang)
    for i in range(n):
        for j in range(n - 1 - i):
            so_sanh += 1
            if mang[j] > mang[j + 1]:
                mang[j], mang[j + 1] = mang[j + 1], mang[j]
    return so_sanh

def tron(mang):
    if len(mang) <= 1:
        return mang, 0
    giua = len(mang) // 2
    trai, dem_trai = tron(mang[:giua])
    phai, dem_phai = tron(mang[giua:])
    ket_qua = []
    i = j = 0
    so_sanh = dem_trai + dem_phai
    while i < len(trai) and j < len(phai):
        so_sanh += 1
        if trai[i] <= phai[j]:
            ket_qua.append(trai[i]); i += 1
        else:
            ket_qua.append(phai[j]); j += 1
    ket_qua += trai[i:]
    ket_qua += phai[j:]
    return ket_qua, so_sanh

for n in [20, 40, 80]:
    mang = list(range(n, 0, -1))
    sb = dem_so_sanh_noi_bot(mang)
    _, sm = tron(mang)
    print(f"n={n}: nổi bọt {sb}, trộn {sm}")
```

```text title=readonly
n=20: nổi bọt 190, trộn 48
n=40: nổi bọt 780, trộn 116
n=80: nổi bọt 3160, trộn 272
```

Mỗi lần `n` gấp đôi, nổi bọt tăng gần **gấp bốn** (190 → 780 → 3160, đúng
hình O(n²)); trộn chỉ tăng hơn **gấp đôi một chút** (48 → 116 → 272, đúng
hình O(n log n)). Với `n = 80` khoảng cách đã là hơn mười một lần
(3160 so với 272) — với danh sách khách hàng thật, cỡ hàng chục nghìn
dòng, khoảng cách đó còn giãn rộng hơn nữa. Đây không phải lý thuyết
suông — là con số đếm được, trên chính đoạn mã vừa chạy.
::::

::::predict{#doan-tim-duong-tren-ban-do commitOnce}
Tình huống 2: một bản đồ giao thông giữa năm thành phố, các tuyến đường
không ghi độ dài — chỉ biết thành phố nào nối trực tiếp thành phố nào.
Cần tìm đường đi dùng **ít tuyến đường nhất**.

**Trước khi trả lời**, bạn đoán: nên dùng BFS hay DFS?

:::opt{correct}
BFS — nó lan ra từng LỚP một, nên đỉnh nào được thăm ở lớp thứ k chắc
chắn cách điểm xuất phát đúng k tuyến đường, không hơn; lớp nào chạm được
đích trước, đó chính là số tuyến ít nhất
:::

:::opt
DFS — vì DFS "đi thẳng một mạch" nên phải tới đích nhanh hơn
::why
Gần đúng ở việc DFS đúng là đi theo kiểu "thẳng một mạch" — đi sâu vào
một nhánh trước khi thử nhánh khác, không rẽ ngang liên tục như BFS.

Chỗ lệch: "đi thẳng" không có nghĩa "đi đường ngắn nhất". DFS chọn nhánh
ĐẦU TIÊN nó gặp, có thể là một nhánh rất dài vòng vèo qua nhiều thành phố
trước khi chạm đích — không có gì trong cách DFS hoạt động đảm bảo nhánh
đầu tiên nó thử là nhánh ít tuyến nhất.
::
:::

:::opt
Cả hai như nhau — vì cả hai đều thăm hết mọi thành phố nối được, nên đều
tìm ra một đường đi
::why
Gần đúng ở việc cả hai đúng là đều tìm RA một đường đi nếu đường đó tồn
tại — không thuật toán nào bỏ sót một thành phố nối được.

Chỗ lệch: câu hỏi không hỏi "có đường đi không", mà hỏi đường đi ÍT TUYẾN
NHẤT. Chỉ cách lan đều từng lớp của BFS mới đảm bảo điều đó — DFS tìm ra
MỘT đường, nhưng không đảm bảo đó là đường ngắn nhất.
::
:::

:::opt
Tham lam — vì tham lam luôn là cách nhanh nhất để chọn từng bước đi
::why
Gần đúng ở việc tham lam đúng là một cách chọn nhanh (bài "Tham lam") —
mỗi bước một lựa chọn, không quay lại.

Chỗ lệch: tham lam là cách RA QUYẾT ĐỊNH cho bài toán có nhiều LỰA CHỌN ở
mỗi bước, như trả tiền thừa (nhiều mệnh giá để chọn). Tìm đường trên một
đồ thị không trọng số không phải dạng bài toán đó — công cụ đúng ở đây là
một cách DUYỆT đồ thị (BFS), không phải một chiến lược chọn lựa từng
bước.
::
:::
::::

::::code{#kiem-de-xuat-va-do-that}
Một thực tập sinh gửi bảng đề xuất cho cả năm tình huống. Bạn viết đoạn
mã so từng đề xuất với đáp án đúng, gom lại những tình huống bị đề xuất
SAI — rồi CHỨNG MINH bằng số đo thật rằng tình huống Fibonacci đúng là
nên chọn cách có sổ, không phải đệ quy trần trụi.

```python title=starter
tinh_huong = {
    "sap_xep_khach_hang": "sắp xếp trộn",
    "tim_duong_di_ngan_nhat": "BFS",
    "fibonacci_lon": "ghi nhớ",
    "tra_tien_thua": "tham lam",
    "tra_cuu_danh_ba": "bảng băm",
}

de_xuat = {
    "sap_xep_khach_hang": "sắp xếp nổi bọt",
    "tim_duong_di_ngan_nhat": "BFS",
    "fibonacci_lon": "đệ quy không bộ nhớ",
    "tra_tien_thua": "tham lam",
    "tra_cuu_danh_ba": "bảng băm",
}

de_xuat_sai = []
for ten in tinh_huong:
    if de_xuat[ten] != ___:
        de_xuat_sai.append(___)

print(f"Số đề xuất sai: {len(de_xuat_sai)}")
print(f"Sai ở: {de_xuat_sai}")

# Chứng minh bằng số đo thật, không chỉ bằng tên gọi — tình huống Fibonacci
so_lan_tinh_that = 0

def fib_memo(n, bo_nho):
    global so_lan_tinh_that
    if n in bo_nho:
        return bo_nho[n]
    so_lan_tinh_that += 1
    if n == 0:
        ket_qua = 0
    elif n == 1:
        ket_qua = 1
    else:
        ket_qua = fib_memo(n - 1, bo_nho) + fib_memo(n - 2, bo_nho)
    bo_nho[n] = ket_qua
    return ket_qua

fib_memo(15, {})
so_lan_khong_bo_nho = 1973  # đã đo ở bài "Đếm xem tính lại bao nhiêu lần"

print(f"Có sổ tính thật: {so_lan_tinh_that} lần")
print(f"Không sổ tính thật: {so_lan_khong_bo_nho} lần")
print(f"Có sổ ít hơn ít nhất: {so_lan_khong_bo_nho // so_lan_tinh_that} lần")
```

```python title=solution
tinh_huong = {
    "sap_xep_khach_hang": "sắp xếp trộn",
    "tim_duong_di_ngan_nhat": "BFS",
    "fibonacci_lon": "ghi nhớ",
    "tra_tien_thua": "tham lam",
    "tra_cuu_danh_ba": "bảng băm",
}

de_xuat = {
    "sap_xep_khach_hang": "sắp xếp nổi bọt",
    "tim_duong_di_ngan_nhat": "BFS",
    "fibonacci_lon": "đệ quy không bộ nhớ",
    "tra_tien_thua": "tham lam",
    "tra_cuu_danh_ba": "bảng băm",
}

de_xuat_sai = []
for ten in tinh_huong:
    if de_xuat[ten] != tinh_huong[ten]:
        de_xuat_sai.append(ten)

print(f"Số đề xuất sai: {len(de_xuat_sai)}")
print(f"Sai ở: {de_xuat_sai}")

so_lan_tinh_that = 0

def fib_memo(n, bo_nho):
    global so_lan_tinh_that
    if n in bo_nho:
        return bo_nho[n]
    so_lan_tinh_that += 1
    if n == 0:
        ket_qua = 0
    elif n == 1:
        ket_qua = 1
    else:
        ket_qua = fib_memo(n - 1, bo_nho) + fib_memo(n - 2, bo_nho)
    bo_nho[n] = ket_qua
    return ket_qua

fib_memo(15, {})
so_lan_khong_bo_nho = 1973

print(f"Có sổ tính thật: {so_lan_tinh_that} lần")
print(f"Không sổ tính thật: {so_lan_khong_bo_nho} lần")
print(f"Có sổ ít hơn ít nhất: {so_lan_khong_bo_nho // so_lan_tinh_that} lần")
```

```python title=test
assert de_xuat_sai == ["sap_xep_khach_hang", "fibonacci_lon"], f"chỉ đúng hai đề xuất sai — sắp xếp khách hàng (đề xuất nhầm nổi bọt) và fibonacci_lon (đề xuất nhầm đệ quy không sổ) — đang báo sai ở {de_xuat_sai}"
assert so_lan_tinh_that == 16, f"fib(15) có sổ phải tính thật đúng 16 lần — đang ra {so_lan_tinh_that}"
assert so_lan_tinh_that < so_lan_khong_bo_nho, "có sổ phải tính thật ÍT HƠN HẲN không sổ — đây chính là con số chứng minh lựa chọn ghi nhớ là đúng"
```

:::hints
- kind: attention
  body: Hai chỗ trống ở phần ĐẦU làm hai việc khác nhau — chỗ trong if phải SO SÁNH đề xuất với đáp án đúng, chỗ trong append phải GHI LẠI tên tình huống đang xét. Phần fib_memo phía dưới đã viết sẵn trọn vẹn, không cần đụng vào.
- kind: strategy
  body: 'Đáp án đúng của tình huống đang xét nằm ở tinh_huong[ten] — so nó với de_xuat[ten] bằng !=. Khi khác nhau, cái cần ghi vào de_xuat_sai là chính TÊN tình huống đó — ten — để biết sai ở đâu, không phải giá trị đúng hay giá trị sai.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là tinh_huong[ten] và ten.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: chỗ trống trong if phải THẬT SỰ tra cứu đáp án đúng từ tinh_huong[ten] — không gõ thẳng một tên thuật toán cố định — vì đề xuất đúng hay sai còn tuỳ tình huống đang xét, không phải một hằng số
  requireAst:
  - kind: uses-name, target: tinh_huong, min: 2
  # min: 2 — đếm thật trên solution bằng kiemAst: tinh_huong xuất hiện đúng
  # 2 lần — một lần ở "for ten in tinh_huong" (đã có sẵn trong khung), một
  # lần ở "tinh_huong[ten]" trong chỗ trống 1. Điền True/1/0 vào chỗ trống 1
  # chỉ còn 1 lần (từ "for ten in tinh_huong") — dưới 2, bắt được. ĐÃ THỬ
  # THẬT: điền True/1/0 vào cả hai chỗ trống — an toàn (chỉ có for lặp trên
  # dict 5 mục cố định, không có while/đệ quy nào trong chính chỗ trống —
  # fib_memo bên dưới hoàn toàn tách biệt, không đụng chỗ trống nào), và cả
  # ba cách đều trượt tests ở assert de_xuat_sai. Cũng đã thử một lời giải
  # ĐÚNG khác — đảo vế "tinh_huong[ten] != de_xuat[ten]" — qua đủ static
  # lẫn tests, đúng tinh thần luật 2.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^Số đề xuất sai: 2\\nSai ở: \\['sap_xep_khach_hang', 'fibonacci_lon'\\]\\nCó sổ tính thật: 16 lần\\nKhông sổ tính thật: 1973 lần\\nCó sổ ít hơn ít nhất: 123 lần\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai đề xuất sai, tìm đúng cả hai — và con số 123 lần không phải một câu
khen suông, nó là bằng chứng đo được.
::::

::::reflect{#nghi-lai}
Một câu hỏi cuối, khép lại cả track này.

Ba mươi tư bài không dạy bạn một danh sách công thức để học thuộc lòng.
Chúng dạy một thói quen: trước khi tin một thuật toán "nhanh" hay "chậm",
"đúng" hay "sai" — ĐO nó. Đếm bước, gọi tên bằng Big-O, thử trên ca xấu
nhất, và nếu nghi ngờ một chiến lược nghe có vẻ hợp lý (như tham lam) thì
dựng một phản ví dụ mà thử thật, đừng tin theo cảm giác.

Track sau — **Máy chạy thế nào** — đổi hẳn câu hỏi. Không còn hỏi "thuật
toán nào nhanh hơn" nữa, mà hỏi: khi đúng những dòng code này chạy, cỗ
máy bên dưới — cái đang thực thi từng phép cộng, từng lời gọi hàm, từng
lượt lặp bạn vừa đếm bước suốt track này — THẬT SỰ đang làm gì?

Hẹn gặp lại ở đó.
::::

::::checkpoint{mastery=0.8}
::::
