---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.ham-gan-duoc-vao-ten
title: "Hàm gán được vào một tên, giữ được trong list/dict"
summary: "gap_doi = lambda x: x * 2 — một hàm không khác gì một số hay một chuỗi: gán được vào tên, đặt được vào dict, lấy ra gọi được. Đây là nền tảng của mọi HOF sắp học."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [fp.function-is-value]
requires: [core.function-def, core.variable, core.dict]
concepts: [fp.function-is-value]
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
Bạn đã viết hàm hàng trăm lần. Hôm nay Byte hỏi một câu khác: một cái
tên GIỮ một hàm được không — y hệt nó giữ một số?
::::

::::explain{#ham-la-mot-gia-tri}
`5` là một giá trị. `"xin chào"` là một giá trị. Một HÀM cũng là một giá
trị — gán được vào tên, đặt được vào `list`, đặt được vào `dict`:

```python
def tinh_tong(a, b):
    return a + b

ham = tinh_tong          # gán vào một tên KHÁC — không có dấu ()
print(ham(3, 4))         # gọi qua tên mới, kết quả y hệt tinh_tong(3, 4)
```

Chú ý: `ham = tinh_tong` KHÔNG có dấu ngoặc. Có ngoặc (`tinh_tong(3, 4)`)
là GỌI hàm, nhận về kết quả (`7`, một số). Không ngoặc là lấy CHÍNH cái
hàm, một giá trị riêng, gán được như bất kỳ giá trị nào khác — `ham` giờ
trỏ tới đúng CHỖ mà `tinh_tong` đang trỏ, đúng ý `mem.name-is-reference`
đã học ở Realm 3.

Đặt nhiều hàm vào một `dict`, tra theo tên:

```python
def tinh_tong(a, b):
    return a + b

def tinh_hieu(a, b):
    return a - b

phep_toan = {
    "cong": tinh_tong,
    "tru": tinh_hieu,
}

print(phep_toan["cong"](10, 3))   # 13
print(phep_toan["tru"](10, 3))    # 7
```

`phep_toan["cong"]` LẤY RA hàm `tinh_tong` (chưa gọi). Thêm `(10, 3)`
ngay sau mới GỌI nó. Hai bước tách rời: TRA CỨU rồi GỌI.
::::

::::example{#lay-ra-chua-goi}
Lấy một hàm ra khỏi `dict` mà CHƯA gọi nó — chuyện gì xảy ra:

```python title=readonly
def tinh_hieu(a, b):
    return a - b

phep_toan = {"tru": tinh_hieu}

x = phep_toan["tru"]
print(type(x))
print(x(10, 3))
```

```text title=readonly
<class 'function'>
7
```

`x` không phải một số — nó là một HÀM (`type(x)` in ra `<class
'function'>`). Chỉ khi thêm `(10, 3)` vào SAU, hàm mới thật sự chạy và
trả về `7`. Tách hai bước ra như vậy là chìa khoá của mọi HOF (higher-
order function) sắp học: một hàm nhận/trả về một hàm KHÁC, mà không cần
gọi nó ngay.
::::

::::predict{#doan-lay-ham-chua-goi commitOnce}
```python
def nhan_doi(x):
    return x * 2

def cong_muoi(x):
    return x + 10

cac_ham = {"a": nhan_doi, "b": cong_muoi}

y = cac_ham["b"]
print(y(5))
```

Dòng cuối in ra gì?

:::opt{correct}
`15`
:::

:::opt
`<function cong_muoi at ...>` — vì `y = cac_ham["b"]` chỉ LẤY hàm ra,
chưa GỌI nó
::why
Gần đúng ở việc bạn nhớ đúng bước ĐẦU: `y = cac_ham["b"]` đúng là chỉ lấy
hàm ra, `y` là một hàm, không phải một số — CHỖ ĐÓ bạn đúng.

Chỗ lệch: dòng ĐƯỢC HỎI là `print(y(5))`, không phải `print(y)`.
`y(5)` có dấu ngoặc — đây là bước GỌI hàm, và `y` đang là `cong_muoi`, nên
`y(5)` chạy `cong_muoi(5)` = `5 + 10` = `15`.
::
:::

:::opt
Máy báo lỗi, vì `cac_ham["b"]` không nhận được đối số `5` cùng lúc với
việc tra `dict`
::why
Gần đúng ở việc bạn nghĩ tới việc tra cứu VÀ gọi hàm cần khớp nhau về đối
số — phản xạ đúng khi đọc code lạ.

Chỗ lệch: tra `dict` (`cac_ham["b"]`) và GỌI hàm (`y(5)`) là HAI bước
HOÀN TOÀN tách rời, xảy ra ở HAI DÒNG khác nhau. `cac_ham["b"]` không cần
biết gì về đối số của `cong_muoi` cả — nó chỉ trả về cái hàm. Đối số `5`
chỉ xuất hiện ở bước GỌI, dòng sau.
::
:::

:::opt
`10` — vì `y` là `cac_ham["b"]`, và giá trị đầu tiên gán cho `b` trong
`dict` phải được cộng vào
::why
Gần đúng ở việc bạn để ý đúng `dict` có khoá `"b"` — bạn tra đúng chỗ.

Chỗ lệch: `cac_ham["b"]` không phải MỘT SỐ để cộng — nó LÀ hàm
`cong_muoi`. `10` không "tự cộng vào" ở đâu cả; `cong_muoi(5)` mới THẬT
SỰ tính `5 + 10`, và kết quả là `15`, không phải `10`.
::
:::
::::

::::code{#tra-goi-phep-tinh}
Byte có ba hàm tính giá: `gia_thuong`, `gia_giam_10`, `gia_giam_20`. Hoàn
thành `bang_gia` — một `dict` ánh xạ tên loại giá sang ĐÚNG hàm tương
ứng — rồi gọi thử qua tên tra được.

```python title=starter
def gia_thuong(gia_goc):
    return gia_goc

def gia_giam_10(gia_goc):
    return gia_goc * 0.9

def gia_giam_20(gia_goc):
    return gia_goc * 0.8

bang_gia = {
    "thuong": gia_thuong,
    "giam10": gia_giam_10,
    "giam20": ___,
}

print(bang_gia["thuong"](100000))
print(bang_gia["giam10"](100000))
print(bang_gia["giam20"](100000))
```

```python title=solution
def gia_thuong(gia_goc):
    return gia_goc

def gia_giam_10(gia_goc):
    return gia_goc * 0.9

def gia_giam_20(gia_goc):
    return gia_goc * 0.8

bang_gia = {
    "thuong": gia_thuong,
    "giam10": gia_giam_10,
    "giam20": gia_giam_20,
}

print(bang_gia["thuong"](100000))
print(bang_gia["giam10"](100000))
print(bang_gia["giam20"](100000))
```

```python title=test
assert bang_gia["giam20"](100000) == 80000.0, "giam20 phải giảm đúng 20%"
assert bang_gia["giam20"](50000) == 40000.0, "phải đúng với một giá gốc KHÁC, không chỉ giá đã in"
assert callable(bang_gia["giam20"]), "bang_gia['giam20'] phải LÀ một hàm, không phải kết quả đã tính sẵn"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ở GIÁ TRỊ của khoá "giam20" trong dict — điền TÊN của hàm đã định nghĩa phía trên, KHÔNG gọi nó (không có dấu ngoặc).
- kind: strategy
  body: 'Hai dòng trên đã làm đúng mẫu — "thuong": gia_thuong và "giam10": gia_giam_10 đều là TÊN hàm, không gọi. Chỗ trống làm y hệt, cho hàm gia_giam_20.'
- kind: one-line
  body: "gia_giam_20"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: bang_gia["giam20"] phải là TÊN hàm gia_giam_20 (không gọi nó, không viết lại phép tính) — dict phải giữ HÀM, không giữ một số đã tính sẵn.
  requireAst:
  - kind: uses-name, target: gia_giam_20, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "80000.0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba hàm, một `dict`, tra tên nào gọi hàm đó. Không `if/elif` dài dòng nào
cả — chỉ tra cứu rồi gọi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa LẤY một hàm RA khỏi `dict` rồi gọi nó. Nhưng nếu thay vì lấy ra,
bạn TRUYỀN một hàm THẲNG VÀO một hàm khác — làm tham số — thì sao? Một
hàm có nhận được một hàm khác làm đối số không, giống cách nó nhận một số
hay một chuỗi?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
