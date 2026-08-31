---
id: khoa-hoc-may-tinh.may-chay-the-nao.boss-ke-mot-dong-lenh
title: "BOSS — Kể chuyện một dòng lệnh, từ gõ phím tới CPU thực thi"
summary: "Ghép trọn hành trình một dòng mã nguồn: dịch thành bytecode, chạy qua vòng lấy-hiểu-làm, mở khung trên ngăn xếp gọi hàm nếu có lời gọi, giá trị thật nằm trên đống còn tên chỉ giữ địa chỉ, và tốc độ phụ thuộc tầng bộ nhớ nào bị chạm."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.instruction-story]
requires: [may.combined-measurement, may.stack-limit-vs-heap, may.cache-locality, may.compile-then-interpret]
concepts: [may.instruction-story]
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
Hai mươi bảy bài để tới đây. Giờ ghép hết lại: kể trọn hành trình một
dòng lệnh, bằng chính chương trình của bạn.
::::

::::explain{#mot-dong-lenh-co-may-doan}
Suốt track này, mỗi cụm bài mở một mảnh khác nhau của cùng một câu
chuyện — **hành trình một dòng mã nguồn**, từ lúc bạn gõ nó tới lúc CPU
thật sự thực thi xong. Ghép lại, câu chuyện đó có năm đoạn:

1. **Dịch trước, chạy sau** — trước khi chạy được gì, mã nguồn phải
   qua một bước BIÊN DỊCH, đổi thành bytecode (bài 1-5, 9). `dis.dis()`
   cho xem đúng bản dịch đó.
2. **Vòng lấy-hiểu-làm** — một vòng lặp RIÊNG, ở tầng thấp hơn code của
   bạn, cứ lấy một lệnh, hiểu nó, làm nó, rồi lấy lệnh tiếp theo — cho
   tới hết (bài 6-10). Vòng lặp đó CHÍNH LÀ CPython, một chương trình
   khác đang chạy chương trình của bạn.
3. **Nếu có lời gọi hàm** — lệnh `CALL` mở một khung MỚI trên ngăn xếp
   gọi hàm (bài 11-15, 23); `RETURN_VALUE` đóng khung đó lại. Ngăn xếp
   này trùng tên nhưng khác vai trò với ngăn xếp `list` T3.2 dạy.
4. **Giá trị thật nằm trên đống** — mọi `list`, `dict`, mọi object bạn
   tạo ra lúc chạy sống trên đống cấp phát (bài 24); tên biến chỉ là
   một ô nhỏ trên khung, giữ ĐỊA CHỈ trỏ sang đó (bài 25).
5. **Tốc độ phụ thuộc tầng bộ nhớ nào bị chạm** — thanh ghi, bộ nhớ
   đệm, RAM, đĩa (bài 16-21), mỗi tầng chậm hơn tầng trước một bậc.
   Duyệt một mảng liền kề tận dụng được bộ nhớ đệm tốt hơn nhiều so
   với một cấu trúc rời rạc kiểu con trỏ (bài 19) — cùng một phép
   toán, khác hẳn tốc độ thật.

Năm đoạn đó là toàn bộ nguyên liệu để kể trọn một dòng lệnh. Bài này
ghép chúng vào một chương trình duy nhất.
::::

::::example{#doi-mot-loi-goi-ham}
Một hàm nhận một `list` điểm thưởng, thêm một điểm mới, rồi trả về tổng
— gọi một lần, kể lại từng đoạn:

```python title=readonly
import dis

def them_diem_thuong(diem_khach, diem_moi):
    diem_khach.append(diem_moi)
    return sum(diem_khach)

diem_cua_lan = [10, 20]
tong_diem = them_diem_thuong(diem_cua_lan, 15)

print(diem_cua_lan)
print(tong_diem)
dis.dis(them_diem_thuong)
```

```text title=readonly
[10, 20, 15]
45
  4           RESUME                   0

  5           LOAD_FAST                0 (diem_khach)
              LOAD_ATTR                1 (append + NULL|self)
              LOAD_FAST                1 (diem_moi)
              CALL                     1
              POP_TOP

  6           LOAD_GLOBAL              3 (sum + NULL)
              LOAD_FAST                0 (diem_khach)
              CALL                     1
              RETURN_VALUE
```

Kể trọn năm đoạn cho đúng lời gọi `them_diem_thuong(diem_cua_lan, 15)`
này: (1) `them_diem_thuong` được dịch một lần thành đúng chuỗi lệnh
`dis.dis()` vừa in — có tới HAI lệnh `CALL` bên trong, một cho
`.append(...)`, một cho `sum(...)`. (2) Vòng lấy-hiểu-làm chạy lần lượt
từng dòng đó. (3) Lệnh `CALL` ngoài cùng (không hiện trong bản dịch
này, nó nằm ở nơi GỌI `them_diem_thuong`) mở một khung mới — bên trong
khung đó, `diem_khach` là một Ô giữ địa chỉ. (4) Địa chỉ đó trỏ đúng
sang `list` `[10, 20]` đã tồn tại trên đống TRƯỚC khi hàm được gọi —
không phải bản sao, nên `.append(15)` sửa xong, `diem_cua_lan` bên
ngoài hàm thấy ngay `[10, 20, 15]`. (5) `list` này chỉ có ba phần tử —
quá nhỏ để tầng bộ nhớ nào tạo khác biệt thấy được, nhưng với một `list`
hàng trăm ngàn phần tử, việc `sum()` duyệt qua nó LIỀN KỀ trong bộ nhớ
(bài 19) là lý do nó nhanh hơn hẳn một cấu trúc rời rạc cùng kích cỡ.
::::

::::predict{#doan-doan-nao-giai-thich commitOnce}
Byte đo hai đoạn mã: đoạn A gọi một hàm `tinh_thue(gia)` một triệu lần
liên tiếp trong vòng `for`; đoạn B duyệt qua một `list` một triệu số
rồi cộng dồn. Cả hai đều chạy XONG, không lỗi nào — nhưng đoạn A CHẬM
HƠN đoạn B RẤT NHIỀU, dù "một triệu bước" ở cả hai đoạn nghe có vẻ
giống nhau.

**Trước khi đọc đáp án**, đoạn nào trong năm đoạn của bài này giải
thích ĐÚNG NHẤT lý do chênh lệch tốc độ ấy?

:::opt{correct}
Đoạn 3 — mỗi lần gọi `tinh_thue(gia)` phải MỞ một khung mới trên ngăn
xếp gọi hàm rồi ĐÓNG nó lại; đoạn B không mở khung nào thêm, chỉ lặp
:::

:::opt
Đoạn 1 — hàm `tinh_thue` phải được BIÊN DỊCH LẠI mỗi lần gọi, còn vòng
`for` cộng dồn chỉ biên dịch một lần
::why
Gần đúng ở việc bạn nhớ đúng: có một bước BIÊN DỊCH xảy ra (đoạn 1) —
điều đó có thật.

Chỗ lệch: biên dịch chỉ xảy ra ĐÚNG MỘT LẦN cho `tinh_thue`, ngay khi
hàm được định nghĩa (bài 9) — không phải mỗi lần GỌI. Một triệu lần gọi
dùng lại đúng MỘT bản dịch đã có sẵn (bài 22 vừa chứng minh
`.__code__` không đổi). Biên dịch lại không phải lý do đoạn A chậm hơn.
::
:::

:::opt
Đoạn 5 — `list` trong đoạn B nằm trên một tầng bộ nhớ NHANH HƠN hẳn so
với nơi hàm `tinh_thue` được lưu
::why
Gần đúng ở việc bạn nghĩ tới TẦNG BỘ NHỚ — đúng là một khái niệm quan
trọng của track này.

Chỗ lệch: cả `list` lẫn mã lệnh của `tinh_thue` đều nằm trong CÙNG dải
RAM của tiến trình (bài 22) — không có chuyện một cái ở tầng nhanh, một
cái ở tầng chậm hẳn theo kiểu đó. Chênh lệch tốc độ ở đây tới từ việc
GỌI HÀM một triệu lần tốn công MỞ/ĐÓNG khung (đoạn 3), không phải từ
việc mã lệnh và dữ liệu nằm khác tầng nhau.
::
:::

:::opt
Đoạn 4 — `list` trong đoạn B sống trên đống, còn kết quả của
`tinh_thue` thì không, nên đoạn A phải tính lại từ đầu
::why
Gần đúng ở việc bạn nhớ đúng: `list` sống trên đống — điều đó có thật
(bài 24-25).

Chỗ lệch: kết quả `tinh_thue(gia)` trả về CŨNG sống trên đống, y hệt
mọi giá trị Python khác — không có "ngoại lệ" nào ở đây khiến nó phải
tính lại từ đầu. Sự chênh lệch tốc độ không tới từ chỗ giá trị NẰM Ở
ĐÂU, mà từ việc GỌI HÀM tốn thêm công mở/đóng khung mà một vòng lặp
thuần tuý không cần.
::
:::
::::

::::code{#ke-mot-dong-lenh-that}
Giỏ hàng cần thêm một món quà tặng, rồi tính tổng — đúng một dòng lệnh
gọi hàm, kể trọn cả năm đoạn. Điền chỗ trống để hoàn thành đơn hàng nhỏ,
rồi xem hàm chạy lại trên một giỏ hàng lớn hơn nhiều.

```python title=starter
import dis, time

def tong_gio_hang(gio_hang, qua_tang):
    gio_hang.append(qua_tang)
    tong = 0
    for gia in gio_hang:
        tong += gia
    return tong

# Đoạn 1-2: mã lệnh cố định, vòng lấy-hiểu-làm chạy nó
dis.dis(tong_gio_hang)

# Đoạn 3-4: gọi hàm mở khung; gio_hang chỉ là một Ô giữ địa chỉ, trỏ
# sang list ĐÃ TỒN TẠI trên đống — không phải bản sao
gio_that = [45000, 32000, 18000]
tong_nho = ___                    # gọi tong_gio_hang(gio_that, 5000)

# Đoạn 5: tầng bộ nhớ nào bị chạm quyết định tốc độ — một giỏ hàng lớn
# hơn rất nhiều, vẫn duyệt liền kề (bài 19)
gio_lon = list(range(1, 50001))
bat_dau = time.perf_counter()
tong_lon = tong_gio_hang(gio_lon, 0)
thoi_gian = time.perf_counter() - bat_dau

print(f"giỏ nhỏ sau khi thêm quà: {gio_that}")
print(f"tổng nhỏ: {tong_nho}")
print(f"tổng lớn: {tong_lon}, đo trong {thoi_gian:.4f} giây")
```

```python title=solution
import dis, time

def tong_gio_hang(gio_hang, qua_tang):
    gio_hang.append(qua_tang)
    tong = 0
    for gia in gio_hang:
        tong += gia
    return tong

dis.dis(tong_gio_hang)

gio_that = [45000, 32000, 18000]
tong_nho = tong_gio_hang(gio_that, 5000)

gio_lon = list(range(1, 50001))
bat_dau = time.perf_counter()
tong_lon = tong_gio_hang(gio_lon, 0)
thoi_gian = time.perf_counter() - bat_dau

print(f"giỏ nhỏ sau khi thêm quà: {gio_that}")
print(f"tổng nhỏ: {tong_nho}")
print(f"tổng lớn: {tong_lon}, đo trong {thoi_gian:.4f} giây")
```

```python title=test
assert gio_that == [45000, 32000, 18000, 5000], f"gio_that phải thấy quà tặng vừa thêm QUA lời gọi hàm — đang ra {gio_that}"
assert tong_nho == 100000, f"tong_nho phải là 45000+32000+18000+5000 = 100000 — đang ra {tong_nho}"
assert tong_lon == 1250025000, f"tong_lon phải là tổng 1..50000 cộng 0 = 1250025000 — đang ra {tong_lon}"
assert thoi_gian >= 0, "thoi_gian phải là một khoảng thời gian đo được, không âm"
```

:::hints
- kind: attention
  body: Chỉ một chỗ trống — một lời GỌI hàm tong_gio_hang, với gio_that và số 5000, gán kết quả vào tong_nho.
- kind: strategy
  body: 'Gọi tong_gio_hang(gio_that, 5000) — đúng hàm vừa định nghĩa, truyền gio_that (list có sẵn) và 5000 (quà tặng thêm vào). Kết quả trả về chính là tổng sau khi đã cộng thêm quà.'
- kind: one-line
  body: 'Chỗ trống là: tong_gio_hang(gio_that, 5000)'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: chỗ trống phải THẬT SỰ gọi tong_gio_hang(gio_that, 5000) — không gõ cứng con số 100000, vì bài này đang chứng minh gio_that thấy được sự thay đổi QUA lời gọi hàm thật
  requireAst:
  # min: 2 — đếm thật trên solution: tong_gio_hang bị GỌI đúng 2 lần trong
  # mã nguồn (chỗ trống + tong_lon = tong_gio_hang(gio_lon, 0) đã có sẵn
  # trong khung). Lưu ý: dis.dis(tong_gio_hang) KHÔNG tính vào — ở đó
  # tong_gio_hang chỉ là một cái TÊN đưa vào dis.dis, không có dấu ngoặc
  # gọi theo sau nó, nên AST của nó là Call với func là "dis.dis", không
  # phải Call với func là "tong_gio_hang". ĐÃ THỬ THẬT bằng cả ba cách
  # True/1/0: không vòng lặp nào phụ thuộc chỗ trống (vòng for TRONG hàm
  # không đụng blank; gio_lon đã list(range(...)) cố định trong khung), nên
  # không cách nào chạy vô hạn — cả ba dừng ngay, và cả ba khiến gio_that
  # KHÔNG hề đổi (thiếu lượt gọi), cho tong_nho sai (True/1/0, không phải
  # 100000) — assert bắt được độc lập với luật static này.
  - kind: uses-call, target: tong_gio_hang, min: 2
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "tổng nhỏ: 100000"
- tier: output
  match: contains
  expect: "tổng lớn: 1250025000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một lời gọi, năm đoạn: dịch một lần, chạy qua vòng lấy-hiểu-làm, mở một
khung, sửa đúng giá trị trên đống, và tốc độ tuỳ tầng bộ nhớ bị chạm.
Bạn vừa kể trọn.
::::

::::reflect{#nghi-lai}
Một câu hỏi cuối, khép lại cả track này.

Hai mươi tám bài, đi từ một dòng mã nguồn không được máy "đọc" mà phải
DỊCH trước, tới một vòng lặp CPython tự chạy chương trình của bạn, tới
những vùng bộ nhớ mang tên quen mà vai trò khác hẳn — và hôm nay, một
chương trình tự kể được hành trình của chính dòng lệnh nó vừa chạy.

Nhưng suốt cả track, bạn luôn QUAN SÁT một trình thông dịch ĐÃ CÓ SẴN —
CPython, viết bằng C, ai đó khác đã xây xong. Bạn xem `dis.dis()`, bạn
đo tốc độ, bạn đo mức trần — nhưng chưa từng tự tay dựng lấy MỘT bản
thông dịch nào, dù chỉ cho một ngôn ngữ tí hon.

Nếu tự bạn phải xây một trình thông dịch — nhận vào một chuỗi lệnh của
riêng bạn, và THẬT SỰ chạy nó — bạn cần những gì? Một vòng lấy-hiểu-làm
(bài 6) là chắc chắn cần. Còn gì nữa?

Đó là câu hỏi mở đầu track sau.
::::

::::checkpoint{mastery=0.85}
::::
