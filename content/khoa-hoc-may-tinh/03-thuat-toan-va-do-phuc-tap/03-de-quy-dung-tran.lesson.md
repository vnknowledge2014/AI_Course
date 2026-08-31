---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.de-quy-dung-tran
title: "Đệ quy đụng trần — đo thật, đừng đoán"
summary: "sys.getrecursionlimit() đọc thẳng mức trần của chồng lời gọi trên CHÍNH máy đang chạy — đo được 1000 trên Pyodide của khoá này — thay cho câu 'khoảng một nghìn' bỏ ngỏ ở bài Hàm gọi chính nó."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [alg.recursion-limit]
requires: [alg.recursion-vs-loop, err.recursion-error]
concepts: [alg.recursion-limit]
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
"Khoảng một nghìn" là câu bỏ ngỏ. Máy biết chính xác con số — chỉ cần
hỏi nó.
::::

::::explain{#khong-doan-nua-hoi-thang}
Bài "Hàm gọi chính nó" từng nói: *"Mức trần ấy vào khoảng một nghìn
lời gọi — con số cụ thể tuỳ máy, và nó không phải thứ đáng nhớ."* Đúng
lúc đó, con số chưa đáng nhớ vì nó chưa **đo được**. Giờ thì có cách.

Python giữ mức trần ấy trong một hàm đọc được: `sys.getrecursionlimit()`.
Nó không phải một quy luật vật lý của ngôn ngữ — nó là một **van an
toàn** trình thông dịch tự đặt, để chồng lời gọi không phình tới mức
làm hỏng cả tiến trình. Vượt quá, Python dừng chương trình bằng
`RecursionError` — một lỗi CÓ báo, đúng như bài trước đã học — thay vì
để máy chết lặng không rõ lý do.

Con số này KHÔNG cố định qua mọi bản Python. Nó tuỳ vào máy, vào bản
dựng. Sách có thể ghi một số; máy bạn đang chạy — Pyodide, biên dịch
cho WASM 32-bit, không phải máy tính thường — có thể cho ra một số
khác. Đo trên chính runtime của khoá này:

```python title=readonly
import sys
print(sys.getrecursionlimit())
```

```text title=readonly
1000
```

Một nghìn, đúng như bài trước phỏng đoán — nhưng giờ là một con số đo
được, không còn là "khoảng".
::::

::::example{#cham-tran-that}
Một hàm đếm LÊN, không có nhánh dừng — cố ý viết để chạm mức trần,
không phải một lỗi vô tình:

```python title=readonly
def dem_len(n):
    return 1 + dem_len(n + 1)

dem_len(0)
```

```text title=readonly
Traceback (most recent call last):
  File "de_quy.py", line 4, in <module>
    dem_len(0)
  File "de_quy.py", line 2, in dem_len
    return 1 + dem_len(n + 1)
  File "de_quy.py", line 2, in dem_len
    return 1 + dem_len(n + 1)
  File "de_quy.py", line 2, in dem_len
    return 1 + dem_len(n + 1)
  [Previous line repeated 994 more times]
RecursionError: maximum recursion depth exceeded
```

Đếm đúng những tờ phiếu traceback vừa liệt kê: một tờ cho dòng gọi
ngoài cùng (chưa phải `dem_len`, đó là dòng module gọi `dem_len(0)`),
ba tờ `dem_len` in tường minh, rồi "994 more times" — tức thêm 994 tờ
`dem_len` nữa. Cộng đúng: 3 + 994 = 997 tờ `dem_len`, cộng thêm một tờ
ngoài cùng, tức 998 tờ tất cả trên chồng lời gọi. Sát ngay dưới mức
trần 1000 mà `sys.getrecursionlimit()` vừa báo — phần chênh lệch nhỏ
là vì tờ phiếu của chính chương trình (dòng gọi `dem_len(0)` ở module)
cũng tính vào, không dành trọn 1000 tờ cho một mình `dem_len`.

Mức trần không phải một ước lượng mơ hồ nữa. Nó là con số
`sys.getrecursionlimit()` vừa đọc, và traceback phía trên là bằng
chứng nó đúng — chạm gần sát con số ấy thì vỡ, không sớm hơn nhiều,
không muộn hơn nhiều.
::::

::::predict{#doan-vuot-tran commitOnce}
```python
import sys

def dem_len(n, dich):
    if n == dich:
        return n
    return dem_len(n + 1, dich)

gioi_han = sys.getrecursionlimit()
ket_qua = dem_len(0, gioi_han + 500)
print(ket_qua)
```

`dem_len` viết đúng — có nhánh dừng, mỗi lượt tiến một bước gần `dich`
hơn. Đích đưa vào là `gioi_han + 500`, tức là hơn mức trần 500 bước.

**Trước khi chạy**, bạn đoán chuyện gì xảy ra?

:::opt{correct}
Chương trình dừng bằng `RecursionError` — đích quá xa mức trần, chồng
lời gọi vỡ trước khi kịp đi hết đường
:::

:::opt
In ra đúng con số `gioi_han + 500`, chỉ là chạy lâu hơn bình thường
một chút
::why
Gần đúng ở việc bạn tin đúng vào LOGIC của `dem_len` — hàm này viết
đúng, không có lỗi, và NẾU được chạy trọn vẹn thì nó sẽ ra đúng con số
đó.

Chỗ lệch: "được chạy trọn vẹn" là điều không xảy ra ở đây. Mỗi lượt
gọi thêm một tờ phiếu, và cái chồng ấy có mức trần cứng. Đích còn cách
rất xa (500 tầng nữa) thì chồng đã chạm trần và máy dừng lại bằng lỗi
— không phải logic hàm sai, mà là tài nguyên chồng lời gọi không đủ
cho đích này.
::
:::

:::opt
Máy tự động nâng `sys.getrecursionlimit()` lên để chạy tiếp, vì đây
là một lời gọi hợp lệ
::why
Gần đúng ở một sự thật có thật: mức trần KHÔNG phải bất biến — có một
hàm tên `sys.setrecursionlimit(...)` cho phép chính bạn đổi nó.

Chỗ lệch: máy không tự làm việc đó. Mức trần là một con số cố định
cho tới khi CHÍNH BẠN gọi hàm để đổi nó — và ngay cả khi đổi, con số
mới cũng chỉ là một trần khác, không phải "không còn trần". Ở đây
không dòng nào gọi `sys.setrecursionlimit`, nên trần vẫn nguyên như
`sys.getrecursionlimit()` đã đọc.
::
:::

:::opt
Chương trình chạy MÃI MÃI, không bao giờ dừng, vì mỗi lượt gọi vẫn
đúng đắn tiến một bước gần đích hơn
::why
Gần đúng ở phần bạn quan sát đúng: xét riêng LOGIC, mỗi lượt gọi thật
sự tiến một bước gần `dich` hơn — nếu không có mức trần, nó sẽ tới
đích.

Chỗ lệch: có một mức trần, và nó được kiểm TRƯỚC KHI đích được chạm
tới. Chồng lời gọi không cao vô hạn — nó dừng ngay khi số tờ phiếu
vượt `sys.getrecursionlimit()`, bất kể đích còn xa bao nhiêu. Đây
chính là lý do van an toàn ấy tồn tại: để chương trình dừng SỚM bằng
một lỗi rõ ràng, thay vì chạy mãi hoặc làm hỏng cả tiến trình.
::
:::
::::

::::code{#do-muc-tran}
Đọc mức trần của CHÍNH máy đang chạy, rồi chứng minh nó có thật bằng
cách cố ý vượt qua nó.

```python title=starter
import sys

gioi_han = ___                        # đọc mức trần đệ quy của CHÍNH máy này

def dem_len(n, dich):
    if n == dich:
        return n
    return dem_len(n + 1, dich)

# Đích rất nhỏ so với mức trần — phải chạy xong bình thường.
ket_qua_nho = dem_len(0, 100)

# Đích gấp mười lần mức trần — phải vỡ bằng RecursionError.
da_vo = False
try:
    dem_len(0, gioi_han * 10)
except RecursionError:
    ___                                # ghi nhận: nó ĐÃ vỡ đúng như dự đoán

print(f"Mức trần đo được: {gioi_han}")
print(f"Kết quả nhỏ: {ket_qua_nho}")
print(f"Đã vỡ đúng như dự đoán: {da_vo}")
```

```python title=solution
import sys

gioi_han = sys.getrecursionlimit()

def dem_len(n, dich):
    if n == dich:
        return n
    return dem_len(n + 1, dich)

ket_qua_nho = dem_len(0, 100)

da_vo = False
try:
    dem_len(0, gioi_han * 10)
except RecursionError:
    da_vo = True

print(f"Mức trần đo được: {gioi_han}")
print(f"Kết quả nhỏ: {ket_qua_nho}")
print(f"Đã vỡ đúng như dự đoán: {da_vo}")
```

```python title=test
import sys
assert gioi_han == sys.getrecursionlimit(), "gioi_han phải là kết quả THẬT của sys.getrecursionlimit() — không phải một con số gõ cứng, vì con số này tuỳ máy đang chạy"
assert ket_qua_nho == 100, f"đích nhỏ (100) phải chạy xong bình thường và trả về đúng 100 — đang ra {ket_qua_nho}"
assert da_vo == True, "gọi dem_len với đích gấp mười lần mức trần phải vỡ bằng RecursionError — da_vo phải thành True trong khối except"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất chỉ đọc một con số có sẵn của module sys — không tính toán gì. Chỗ trống thứ hai chỉ ghi nhận một sự kiện đã xảy ra — nó nằm NGAY TRONG khối except, tức là chỉ chạy khi RecursionError đã thật sự nổ ra.
- kind: strategy
  body: 'Chỗ trống 1: gọi đúng hàm bài vừa học để đọc mức trần — sys.getrecursionlimit(). Chỗ trống 2: bên trong except, gán da_vo = True để đánh dấu chồng lời gọi đã vỡ đúng như dự đoán — không cần return, không cần in gì thêm ở đây.'
- kind: one-line
  body: 'Chỗ trống 1 là sys.getrecursionlimit(); chỗ trống 2 là da_vo = True.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: chỗ trống thứ nhất phải THẬT SỰ gọi sys.getrecursionlimit() để đo mức trần của máy đang chạy — không gõ cứng một con số; chỗ trống thứ hai phải THẬT SỰ gán da_vo = True bên trong except, không phải một câu không làm gì
  requireAst:
  - kind: uses-call, target: getrecursionlimit, min: 1
  - kind: gan-ten, target: da_vo, min: 2
  # gan-ten min: 2 — đếm thật trên solution: da_vo được GÁN đúng 2 lần trong mã
  # nguồn — "da_vo = False" (đã có sẵn trong khung) và "da_vo = True" (chỗ
  # trống). Điền True/1/0 (một câu không làm gì) vào chỗ trống 2 chỉ còn 1 lần
  # gán — dưới 2, luật này chặn được. ĐÃ THỬ THẬT bằng cả ba cách True/1/0 cho
  # cả hai chỗ trống: cả ba đều dừng an toàn và nhanh (dem_len(0, 100) luôn
  # chạy xong vì nhánh dừng và bước tiến không phụ thuộc chỗ trống; nhánh
  # except chỉ chạy một câu không làm gì, không đệ quy thêm) — không cách nào
  # lặp vô hạn — và cả ba đều cho gioi_han sai (True/1/0, không phải 1000) lẫn
  # da_vo vẫn là False, nên assert đầu tiên và assert cuối đều bắt được độc
  # lập với luật static này.
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: "^Mức trần đo được: 1000\\nKết quả nhỏ: 100\\nĐã vỡ đúng như dự đoán: True\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
1000. Không phải "khoảng một nghìn" nữa — đo được, và chứng minh được
bằng chính cái lỗi nó gây ra khi vượt qua.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mọi hàm đệ quy học tới giờ — đếm ngược, tính tổng, duyệt cây — đều gọi
lại chính mình đúng **một lần** mỗi lượt. Chồng lời gọi vì vậy luôn là
một cột thẳng: tờ này chồng lên tờ kia, một hàng duy nhất.

Nhưng có những bài toán mà một lượt gọi cần gọi lại chính mình **hai
lần**, không phải một — chẳng hạn tính một số trong dãy mà mỗi số là
tổng của hai số ngay trước nó. Lúc đó cái chồng còn là một cột thẳng
không, hay nó bắt đầu phân nhánh?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
