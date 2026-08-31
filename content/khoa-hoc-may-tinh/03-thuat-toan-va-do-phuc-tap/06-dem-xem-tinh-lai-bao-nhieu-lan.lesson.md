---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.dem-xem-tinh-lai-bao-nhieu-lan
title: "Đếm xem tính lại bao nhiêu lần — đừng chỉ nhìn bằng mắt"
summary: "Một biến đếm toàn cục, tăng lên 1 mỗi lần fib được GỌI — không phải đếm trên một nhật ký đã ghi xong, mà đếm SỐNG ngay trong lúc hàm đang chạy. fib(10) gọi 177 lần, fib(15) gọi 1973 lần — công cụ này dùng được cho mọi thứ, không riêng đệ quy."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.count-recursive-calls]
requires: [alg.recompute-waste]
concepts: [alg.count-recursive-calls]
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
Không vẽ nổi cây `fib(30)` bằng tay đâu. Nhưng máy đếm được — nếu bạn
bảo nó đếm.
::::

::::explain{#mot-bien-tang-moi-lan-goi}
Bài trước đếm bằng cách nhìn vào một danh sách nhật ký ĐÃ ghi xong —
`nhat_ky.count("ĐẨY fib(2)")` — sau khi `fib(6)` chạy trọn vẹn. Cách
đó cần lưu lại MỌI dòng nhật ký trước, rồi mới lọc. Với `fib(6)` — hai
mươi lăm dòng — không vấn đề gì. Với `fib(30)`, số lượt gọi lớn tới
mức lưu hết nhật ký cũng tốn kém, chưa nói tới việc đọc lại bằng mắt.

Có một cách gọn hơn nhiều: không cần ghi lại MỌI CHI TIẾT của từng
lượt gọi, chỉ cần biết TỔNG số lượt. Một biến toàn cục, khởi tạo bằng
`0`, và mỗi khi hàm được gọi thì tăng nó lên đúng `1`:

```python
so_lan_goi = 0

def fib(n):
    global so_lan_goi
    so_lan_goi += 1
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib(n - 1) + fib(n - 2)
```

Dòng `global so_lan_goi` là điều bắt buộc: không có nó, dòng
`so_lan_goi += 1` bên trong hàm sẽ tạo ra một biến CỤC BỘ mới tên
`so_lan_goi`, chỉ sống trong lần gọi đó, thay vì tăng biến ở ngoài.
`+= 1` chạy đúng một lần mỗi khi thân hàm bắt đầu chạy — không quan
trọng lượt đó rơi vào trường hợp cơ sở hay gọi tiếp hai nhánh con —
nên nó đếm đúng SỐ LƯỢT `fib` được gọi, không thiếu một lượt nào,
không đếm trùng lượt nào.

Đây không phải kỹ thuật riêng cho đệ quy. Đây là kỹ thuật đếm bước
dùng được cho **bất cứ đoạn mã nào** — bài sau sẽ áp đúng nó lên một
vòng lặp bình thường, không đệ quy. Chỗ khác biệt duy nhất ở đây là
BIẾN được tăng nằm bên trong một hàm gọi lại chính nó.
::::

::::example{#so-fib10-va-fib15}
Chạy trọn `fib(10)` và `fib(15)` với bộ đếm, đặt lại đếm về 0 giữa
hai lần đo để không cộng dồn:

```python title=readonly
so_lan_goi = 0

def fib(n):
    global so_lan_goi
    so_lan_goi += 1
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib(n - 1) + fib(n - 2)

fib(10)
print("fib(10) gọi:", so_lan_goi, "lần")

so_lan_goi = 0
fib(15)
print("fib(15) gọi:", so_lan_goi, "lần")
```

```text title=readonly
fib(10) gọi: 177 lần
fib(15) gọi: 1973 lần
```

`n` chỉ tăng từ 10 lên 15 — thêm đúng năm bước — mà số lượt gọi nhảy
từ 177 lên 1973, hơn mười một lần. Đây là con số THẬT, đếm SỐNG ngay
lúc chương trình chạy, không phải ước lượng và không cần vẽ cây nào
cả — cây của `fib(15)` có gần hai nghìn nút, không ai vẽ nổi nó bằng
tay.
::::

::::predict{#doan-neu-quen-reset commitOnce}
Cũng đoạn mã trên, nhưng bỏ mất dòng `so_lan_goi = 0` giữa hai lần đo:

```python
so_lan_goi = 0

def fib(n):
    global so_lan_goi
    so_lan_goi += 1
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib(n - 1) + fib(n - 2)

fib(10)
print("fib(10) gọi:", so_lan_goi, "lần")

fib(15)
print("fib(15) gọi:", so_lan_goi, "lần")
```

**Trước khi chạy**, bạn đoán dòng in thứ hai — cho `fib(15)` — hiện ra
con số nào?

:::opt{correct}
2150 — vì `so_lan_goi` không được đặt lại về 0, số lần gọi của
`fib(15)` (1973) bị CỘNG THÊM vào số đã có sẵn từ `fib(10)` (177)
:::

:::opt
1973 — vì mỗi lần đo lượt gọi độc lập với nhau, không liên quan gì
tới việc đo trước đó
::why
Gần đúng ở việc bạn tin đúng CÁCH TÍNH của bản thân `fib(15)` — nó
vẫn gọi đúng 1973 lượt, không đổi.

Chỗ lệch: `so_lan_goi` là MỘT biến toàn cục duy nhất, không phải một
biến riêng cho mỗi lần đo. Thiếu dòng đặt lại về 0, giá trị 177 từ
lần đo `fib(10)` vẫn còn nguyên trong biến khi `fib(15)` bắt đầu cộng
tiếp vào — con số cuối cùng là tổng của cả hai, không phải chỉ riêng
`fib(15)`.
::
:::

:::opt
177 — vì biến toàn cục chỉ giữ giá trị của lần GẦN NHẤT được gán
::why
Gần đúng ở việc bạn nhớ đúng con số 177 xuất hiện trước đó trong
chương trình — nó có thật, đúng là kết quả đo `fib(10)`.

Chỗ lệch: `so_lan_goi += 1` không GÁN LẠI biến bằng một giá trị mới
độc lập — nó CỘNG THÊM vào giá trị đang có. `fib(15)` chạy 1973 lượt,
mỗi lượt cộng thêm 1 vào con số đang sẵn có (177), nên kết quả cuối
là 177 + 1973, không dừng lại ở 177.
::
:::

:::opt
Máy báo lỗi, vì `so_lan_goi` bị dùng hai lần mà không đặt lại là
không hợp lệ trong Python
::why
Gần đúng ở việc bạn cảm thấy có gì đó "không sạch" khi tái sử dụng
một biến mà không dọn nó trước — cảm giác đó có lý trong nhiều ngữ
cảnh viết mã cẩn thận.

Chỗ lệch: không có luật nào của Python cấm việc này. Biến toàn cục cứ
tồn tại và cứ được cộng dồn cho tới khi CHÍNH BẠN gán lại nó — không
có ngoại lệ, không có lỗi nào nổ ra. Chương trình chạy trọn vẹn, chỉ
là con số cuối cùng không phải con số bạn tưởng.
::
:::
::::

::::code{#dem-song-hai-co}
Thêm bộ đếm sống vào `fib`, rồi đo đúng hai cỡ: `fib(10)` và `fib(15)`
— nhớ đặt lại `so_lan_goi` về `0` giữa hai lần đo.

```python title=starter
so_lan_goi = 0

def fib_dem(n):
    global so_lan_goi
    ___                              # tăng bộ đếm lên 1, mỗi lần hàm được GỌI
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib_dem(n - 1) + fib_dem(n - 2)

fib_dem(10)
so_lan_goi_10 = so_lan_goi

___                                  # đặt lại bộ đếm về 0 trước khi đo tiếp
fib_dem(15)
so_lan_goi_15 = so_lan_goi

print(f"Số lần gọi khi tính fib(10): {so_lan_goi_10}")
print(f"Số lần gọi khi tính fib(15): {so_lan_goi_15}")
```

```python title=solution
so_lan_goi = 0

def fib_dem(n):
    global so_lan_goi
    so_lan_goi += 1
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib_dem(n - 1) + fib_dem(n - 2)

fib_dem(10)
so_lan_goi_10 = so_lan_goi

so_lan_goi = 0
fib_dem(15)
so_lan_goi_15 = so_lan_goi

print(f"Số lần gọi khi tính fib(10): {so_lan_goi_10}")
print(f"Số lần gọi khi tính fib(15): {so_lan_goi_15}")
```

```python title=test
assert so_lan_goi_10 == 177, f"fib(10) phải gọi fib_dem đúng 177 lần — đang ra {so_lan_goi_10}"
assert so_lan_goi_15 == 1973, f"fib(15) phải gọi fib_dem đúng 1973 lần — đang ra {so_lan_goi_15}; nếu ra một con số lớn hơn nhiều (như 2150), có thể bộ đếm chưa được đặt lại về 0 trước lần đo thứ hai"
assert so_lan_goi_15 > so_lan_goi_10, "chỉ tăng n từ 10 lên 15 mà số lần gọi phải tăng lên nhiều, không phải giảm hay đứng yên"
```

:::hints
- kind: attention
  body: Chỗ trống 1 nằm TRONG thân hàm — mỗi lần hàm bắt đầu chạy phải tăng bộ đếm, bất kể lượt đó là trường hợp cơ sở hay gọi tiếp. Chỗ trống 2 nằm NGOÀI hàm, giữa hai lần đo — phải đưa bộ đếm về đúng điểm xuất phát trước khi đo lần thứ hai.
- kind: strategy
  body: 'Chỗ trống 1: so_lan_goi += 1 — cộng thêm 1 vào biến toàn cục đã khai global. Chỗ trống 2: so_lan_goi = 0 — gán lại về 0, không phải cộng thêm gì, để lần đo fib(15) không bị cộng chồng lên kết quả của fib(10).'
- kind: one-line
  body: 'Chỗ trống 1 là so_lan_goi += 1; chỗ trống 2 là so_lan_goi = 0.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: chỗ trống 1 phải THẬT SỰ tăng so_lan_goi lên 1 mỗi lần hàm chạy; chỗ trống 2 phải THẬT SỰ đặt lại so_lan_goi về 0 trước lần đo thứ hai — không phải một câu không làm gì như True, 1, 0
  requireAst:
  - kind: gan-ten, target: so_lan_goi, min: 3
  # min: 3 — đếm thật trên solution: so_lan_goi được GÁN đúng 3 lần trong mã
  # nguồn — "so_lan_goi = 0" ở đầu (đã có sẵn trong khung), "so_lan_goi += 1"
  # bên trong hàm (chỗ trống 1), và "so_lan_goi = 0" giữa hai lần đo (chỗ
  # trống 2). Điền True/1/0 (một câu không làm gì) vào MỘT trong hai chỗ
  # trống chỉ còn 2 lần gán — dưới 3, luật này chặn được; điền cả hai chỉ còn
  # 1. ĐÃ THỬ THẬT bằng cả ba cách True/1/0 cho cả hai chỗ trống cùng lúc: cả
  # ba đều dừng AN TOÀN và NHANH — không lặp vô hạn, vì cả hai chỗ trống chỉ
  # ảnh hưởng tới việc ĐẾM, không ảnh hưởng gì tới hai trường hợp cơ sở
  # (n == 0, n == 1) hay lời gọi đệ quy fib_dem(n-1) + fib_dem(n-2), vốn giữ
  # nguyên trong khung và luôn tiến về 0/1 bình thường — và cả ba cho
  # so_lan_goi_10 == 0 và so_lan_goi_15 == 0, sai hẳn so với 177 và 1973, bị
  # tests bắt độc lập với luật static này.
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: "^Số lần gọi khi tính fib\\(10\\): 177\\nSố lần gọi khi tính fib\\(15\\): 1973\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
177, rồi 1973 — con số THẬT, đếm SỐNG, không cần vẽ một nét nào lên
giấy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bộ đếm vừa dùng không có gì đặc biệt dành riêng cho đệ quy — chỉ là
một biến, tăng thêm 1 mỗi lần chạm đúng chỗ cần đếm. Chỗ đó tình cờ
nằm trong một hàm gọi lại chính nó, nhưng bản thân kỹ thuật đếm không
hề biết, cũng không cần biết, điều đó.

Nếu chỗ cần đếm không phải một lượt gọi hàm đệ quy, mà là một lượt
lặp bình thường — một vòng `for` chạy qua từng phần tử của danh sách,
hay một vòng `while` so sánh từng cặp số — bộ đếm y hệt thế này có
còn dùng được không?

Bài sau trả lời — và mở ra một cách nhìn dùng được cho MỌI đoạn mã,
không riêng gì đệ quy.
::::

::::checkpoint{mastery=0.8}
::::
