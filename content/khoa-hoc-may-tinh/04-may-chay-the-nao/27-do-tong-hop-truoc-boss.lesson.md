---
id: khoa-hoc-may-tinh.may-chay-the-nao.do-tong-hop-truoc-boss
title: "Đo tổng hợp: đếm lệnh, xem bytecode, ước lượng thời gian"
summary: "Ghép ba công cụ đo đã học rời rạc — dis.dis() (bài 1, đếm lệnh), đếm bước tự viết (T3.3 cụm 2), time.perf_counter() (đo thời gian thật) — trên CÙNG một đoạn mã, đối chiếu ba con số với nhau."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.combined-measurement]
requires: [may.stack-limit-vs-heap]
concepts: [may.combined-measurement]
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
Ba con số, một đoạn mã. Chúng có luôn đồng ý với nhau không?
::::

::::explain{#ba-cong-cu-mot-cho}
Suốt track này, ba công cụ đo từng xuất hiện RIÊNG LẺ:

1. **`dis.dis()`** (bài 1) — đếm số LỆNH MÁY một hàm dịch thành. Con số
   này CỐ ĐỊNH — bài 22 vừa chứng minh nó không đổi dù gọi hàm bao
   nhiêu lần, với dữ liệu gì.
2. **Đếm bước tự viết** (T3.3 cụm 2) — một biến đếm tăng lên 1 mỗi lần
   chạm việc chính của vòng lặp. Con số này ĐỔI THEO DỮ LIỆU — vòng
   lặp chạy trên 10 phần tử cho một con số, trên 10.000 phần tử cho
   con số khác hẳn.
3. **`time.perf_counter()`** — đọc đồng hồ THẬT, độ phân giải cỡ nano
   giây (đã đo ở T3.1). Hiệu hai lần đọc, trước và sau một đoạn mã, là
   THỜI GIAN THẬT nó chạy — không phải ước lượng, không phải đếm bước,
   mà đo bằng giây thật trên chính máy này.

Ba công cụ đo BA THỨ khác nhau: một đếm HÌNH DẠNG của mã (cố định), một
đếm SỐ LẦN chạm việc chính (đổi theo dữ liệu), một đo THỜI GIAN thật
(đổi theo dữ liệu VÀ theo tốc độ máy). Ghép cả ba trên CÙNG một đoạn mã
cho một bức tranh đầy đủ hơn hẳn một con số lẻ.
::::

::::example{#ba-con-so-tren-mot-ham}
Đếm số chẵn trong một mảng — đo cả ba công cụ:

```python title=readonly
import dis, time

def dem_so_chan(mang):
    so_buoc = 0
    dem = 0
    for x in mang:
        so_buoc += 1
        if x % 2 == 0:
            dem += 1
    return dem, so_buoc

mang_kiem_tra = list(range(10000))

bat_dau = time.perf_counter()
dem, so_buoc = dem_so_chan(mang_kiem_tra)
thoi_gian = time.perf_counter() - bat_dau

print(f"số chẵn: {dem}, số bước: {so_buoc}")
print(f"thời gian: {thoi_gian:.6f} giây")
dis.dis(dem_so_chan)
```

```text title=readonly
số chẵn: 5000, số bước: 10000
thời gian: 0.002400 giây
  4           RESUME                   0

  5           LOAD_CONST               1 (0)
              STORE_FAST               1 (so_buoc)

  6           LOAD_CONST               1 (0)
              STORE_FAST               2 (dem)

  7           LOAD_FAST                0 (mang)
              GET_ITER
      L1:     FOR_ITER                24 (to L3)
              STORE_FAST               3 (x)

  8           LOAD_FAST                1 (so_buoc)
              LOAD_CONST               2 (1)
              BINARY_OP               13 (+=)
              STORE_FAST               1 (so_buoc)

  9           LOAD_FAST                3 (x)
              LOAD_CONST               3 (2)
              BINARY_OP                6 (%)
              LOAD_CONST               1 (0)
              COMPARE_OP              88 (bool(==))
              POP_JUMP_IF_TRUE         2 (to L2)
              JUMP_BACKWARD           19 (to L1)

 10   L2:     LOAD_FAST                2 (dem)
              LOAD_CONST               2 (1)
              BINARY_OP               13 (+=)
              STORE_FAST               2 (dem)
              JUMP_BACKWARD           26 (to L1)

  7   L3:     END_FOR
              POP_TOP

 11           LOAD_FAST_LOAD_FAST     33 (dem, so_buoc)
              BUILD_TUPLE              2
              RETURN_VALUE
```

Ba con số kể ba câu chuyện KHÁC NHAU cho CÙNG một lần chạy: `so_buoc =
10000` — đúng bằng số phần tử của `mang_kiem_tra`, đổi theo dữ liệu.
`thoi_gian` — một con số giây thật, phụ thuộc cả `so_buoc` LẪN tốc độ
máy lúc đó. `dis.dis()` — chuỗi lệnh CỐ ĐỊNH, không đổi dù chạy trên
mảng 10 phần tử hay 10.000. Gọi lại `dem_so_chan` với mảng khác cỡ, hai
con số đầu đổi; con số thứ ba (chuỗi lệnh) đứng yên.
::::

::::predict{#doan-lenh-co-dinh commitOnce}
Byte gọi lại đúng hàm `dem_so_chan` ở ví dụ trên, nhưng lần này với
`mang_kiem_tra = list(range(100))` — một mảng NHỎ HƠN một trăm lần.

**Trước khi đọc đáp án**, phát biểu nào dưới đây đúng?

:::opt{correct}
`so_buoc` và `thoi_gian` đều nhỏ đi rất nhiều; `dis.dis(dem_so_chan)`
vẫn in ra ĐÚNG chuỗi lệnh y hệt như trước
:::

:::opt
Cả ba con số đều nhỏ đi theo đúng tỉ lệ 100 lần — vì cùng một hàm, cùng
một logic
::why
Gần đúng ở việc bạn nghĩ tới TỈ LỆ — `so_buoc` đúng là nhỏ đi theo tỉ
lệ gần sát 100 lần, vì nó đếm đúng số phần tử.

Chỗ lệch: `dis.dis()` không "nhỏ đi" — nó không hề đổi. Chuỗi lệnh máy
mà `dem_so_chan` dịch thành là một thuộc tính của MÃ NGUỒN, không phải
của dữ liệu truyền vào. Gọi hàm với mảng cỡ nào, `dis.dis(dem_so_chan)`
luôn in ra đúng số dòng, đúng tên lệnh như cũ.
::
:::

:::opt
`dis.dis()` sẽ in ra ÍT lệnh hơn, vì vòng lặp chạy ít lần hơn nên cần
ít lệnh để mô tả nó
::why
Gần đúng ở trực giác "làm ít việc hơn thì cần ít lệnh hơn" — trực giác
đó đúng cho `so_buoc`, con số ĐẾM SỐ LẦN chạy.

Chỗ lệch: `dis.dis()` không đếm số lần một lệnh CHẠY — nó liệt kê danh
sách lệnh mà hàm được DỊCH THÀNH, một lần, trước khi chạy (bài 1, 9).
Một vòng `for` luôn dịch thành ĐÚNG một cụm lệnh cố định
(`FOR_ITER`/`JUMP_BACKWARD`/...) bất kể nó sẽ CHẠY bao nhiêu vòng lúc
thực thi.
::
:::

:::opt
`thoi_gian` sẽ không đổi, vì `time.perf_counter()` đo tốc độ máy, không
đo lượng việc phải làm
::why
Gần đúng ở việc bạn nhớ đúng: `time.perf_counter()` đọc đồng hồ THẬT
của máy — không phải một phép đếm trừu tượng.

Chỗ lệch: đồng hồ đọc được THỜI GIAN đã TRÔI QUA giữa hai lần gọi — mà
thời gian đó phụ thuộc TRỰC TIẾP vào lượng việc CPU phải làm trong
khoảng ấy. Mảng nhỏ hơn 100 lần → vòng lặp chạy ít hơn 100 lần → CPU
làm ít việc hơn hẳn trong khoảng đó → `thoi_gian` đo được cũng nhỏ đi
theo, không đứng yên.
::
:::
::::

::::code{#ghep-ba-cong-cu}
Tính tổng `1 + 2 + ... + n` bằng vòng lặp, đếm bước SONG SONG. Gọi hàm
với hai cỡ `n` khác hẳn nhau — 2.000 rồi 40.000 — và đối chiếu cả ba
công cụ đo trên cùng một hàm.

```python title=starter
import dis, time

def tong_1_den(n):
    tong = 0
    so_buoc = 0
    for i in range(1, n + 1):
        so_buoc += 1
        tong += i
    return tong, so_buoc

# Công cụ 1: mã lệnh — cố định, không phụ thuộc n (bài 1, 22)
dis.dis(tong_1_den)

# Công cụ 2 + 3: đếm BƯỚC tự viết, và đo THỜI GIAN thật
bat_dau_nho = time.perf_counter()
ket_qua_nho, buoc_nho = tong_1_den(2000)
thoi_gian_nho = time.perf_counter() - bat_dau_nho

bat_dau_lon = time.perf_counter()
ket_qua_lon, buoc_lon = ___          # gọi tong_1_den với n = 40000
thoi_gian_lon = time.perf_counter() - bat_dau_lon

buoc_dung = (buoc_nho, buoc_lon) == (2000, 40000)
thoi_gian_lon_hon = thoi_gian_lon > thoi_gian_nho

print(f"bước: {buoc_nho} rồi {buoc_lon} — đúng tỉ lệ n: {buoc_dung}")
print(f"thời gian n lớn có lâu hơn n nhỏ: {thoi_gian_lon_hon}")
```

```python title=solution
import dis, time

def tong_1_den(n):
    tong = 0
    so_buoc = 0
    for i in range(1, n + 1):
        so_buoc += 1
        tong += i
    return tong, so_buoc

dis.dis(tong_1_den)

bat_dau_nho = time.perf_counter()
ket_qua_nho, buoc_nho = tong_1_den(2000)
thoi_gian_nho = time.perf_counter() - bat_dau_nho

bat_dau_lon = time.perf_counter()
ket_qua_lon, buoc_lon = tong_1_den(40000)
thoi_gian_lon = time.perf_counter() - bat_dau_lon

buoc_dung = (buoc_nho, buoc_lon) == (2000, 40000)
thoi_gian_lon_hon = thoi_gian_lon > thoi_gian_nho

print(f"bước: {buoc_nho} rồi {buoc_lon} — đúng tỉ lệ n: {buoc_dung}")
print(f"thời gian n lớn có lâu hơn n nhỏ: {thoi_gian_lon_hon}")
```

```python title=test
assert ket_qua_lon == 800020000, f"tong_1_den(40000) phải trả tổng 1+2+...+40000 = 800020000 — đang ra {ket_qua_lon}"
assert buoc_dung is True, f"buoc_nho phải là 2000 và buoc_lon phải là 40000 — đang ra ({buoc_nho}, {buoc_lon})"
assert thoi_gian_lon_hon is True, "n lớn hơn một trăm lần phải đo được thời gian LÂU HƠN — nếu không, chỗ trống chưa thật sự gọi tong_1_den(40000)"
```

:::hints
- kind: attention
  body: Chỉ một chỗ trống — một lời GỌI tong_1_den với n=40000, gán kết quả (một cặp giá trị) vào đúng hai tên ket_qua_lon, buoc_lon.
- kind: strategy
  body: 'Gọi lại đúng hàm tong_1_den, lần này với n=40000 — y hệt cách bat_dau_nho/ket_qua_nho/buoc_nho đã làm ở trên với n=2000.'
- kind: one-line
  body: 'Chỗ trống là: tong_1_den(40000)'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: chỗ trống phải THẬT SỰ gọi tong_1_den(40000) — không gõ cứng một cặp số, vì bài này đang đo THỜI GIAN THẬT của một lần chạy thật
  requireAst:
  # min: 2 — đếm thật trên solution: tong_1_den bị GỌI đúng 2 lần trong mã
  # nguồn (ket_qua_nho, buoc_nho = tong_1_den(2000) đã có sẵn trong khung +
  # chỗ trống). ĐÃ THỬ THẬT bằng cả ba cách True/1/0: mỗi cách làm dòng
  # "ket_qua_lon, buoc_lon = True" (hay 1, hay 0) NÉM NGAY một TypeError khi
  # chạy — Python không unpack được một bool/int thành hai tên — nên tầng
  # `run` đã chặn được cả ba, độc lập với luật static này; không vòng lặp
  # nào bị ảnh hưởng bởi chỗ trống (vòng for nằm TRONG hàm, không đụng chỗ
  # trống), nên không cách nào chạy vô hạn.
  - kind: uses-call, target: tong_1_den, min: 2
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "đúng tỉ lệ n: True"
- tier: output
  match: contains
  expect: "lâu hơn n nhỏ: True"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mã lệnh đứng yên, bước đếm nhân đúng trăm lần, đồng hồ thật cũng chạy
lâu hơn. Ba công cụ, một câu chuyện thống nhất.
::::

::::reflect{#nghi-lai}
Một câu hỏi cuối trước khi tổng kết cả track.

Hai mươi bảy bài — từ một dòng mã nguồn biến thành lệnh máy, tới vòng
lấy-hiểu-làm, tới khung mở trên ngăn xếp gọi hàm, tới giá trị thật nằm
trên đống, tới tầng bộ nhớ nào bị chạm quyết định tốc độ.

Nếu lấy đúng MỘT dòng mã nguồn — một dòng có gọi hàm, có một chút tính
toán — và kể lại TRỌN VẸN hành trình của nó, từ lúc bạn gõ phím tới lúc
CPU thực thi xong, bạn kể được đủ những đoạn nào?

Bài cuối cùng của track này để bạn tự kể.
::::

::::checkpoint{mastery=0.8}
::::
