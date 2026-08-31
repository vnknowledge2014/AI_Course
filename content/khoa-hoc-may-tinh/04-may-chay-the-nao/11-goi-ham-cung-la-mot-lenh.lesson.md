---
id: khoa-hoc-may-tinh.may-chay-the-nao.goi-ham-cung-la-mot-lenh
title: "Gọi hàm cũng chỉ là một lệnh: CALL"
summary: "CALL là một lệnh máy thật, nằm ngay trong dis.dis() — đúng chỗ mã nguồn gọi một hàm khác. Con số đứng sau nó (CALL 1, CALL 2...) là SỐ ĐỐI SỐ truyền vào, đọc được ngay trên bản in — không phải cơ chế bí ẩn riêng, cùng họ với LOAD_FAST, BINARY_OP đã học."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.call-instruction]
requires: [may.interpret-vs-compile-tradeoff, alg.recursion-is-stack]
concepts: [may.call-instruction]
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
Tờ phiếu ấy — T3.3 gọi nó ngăn xếp cuộc gọi. Hôm nay bạn thấy tận mắt
đúng lệnh đặt nó lên.
::::

::::explain{#goi-ham-la-lenh}
Bài "Đệ quy chính là một ngăn xếp" đã nói: mỗi lần một hàm được gọi, máy
đặt thêm một "tờ phiếu" lên chồng lời gọi. Câu đó đúng, nhưng còn bỏ
lửng một chỗ: đặt tờ phiếu ấy lên KHÔNG tự nhiên mà có. Có một LỆNH MÁY
làm đúng việc đó.

Từ bài 1 tới bài 10, mọi lệnh bạn gặp — `LOAD_FAST`, `BINARY_OP`,
`JUMP_BACKWARD`, `POP_JUMP_IF_FALSE` — đều nằm gọn trong một hàm, chưa
hàm nào gọi hàm khác. Giờ thêm đúng một lệnh mới, cùng loại với những
lệnh ấy: gọi một hàm KHÁC là lệnh **`CALL`**.

Trước khi `CALL` chạy, hai việc đã xảy ra trên ngăn xếp tính toán (bài
3):

1. `LOAD_GLOBAL` tải chính cái hàm sắp gọi lên ngăn xếp — bản in ghi
   kèm `+ NULL`, một chỗ trống kỹ thuật CPython luôn dành sẵn cho mọi
   lời gọi hàm; không cần đào sâu bên trong nó ở bài này.
2. Mỗi `LOAD_FAST` (hoặc `LOAD_FAST_LOAD_FAST` nếu gộp được, bài 2) tải
   một đối số lên tiếp.

Rồi `CALL` chạy — và con số đứng NGAY SAU nó, như `CALL 1` hay
`CALL 2`, chính là **SỐ ĐỐI SỐ vừa được tải lên**. Đếm được ngay trên
bản in, không phải đoán.

`CALL` không phải một cơ chế bí ẩn riêng cho việc gọi hàm. Nó là một
lệnh, đứng cùng hàng với `LOAD_FAST`, `BINARY_OP` — chỉ khác việc nó
làm.
::::

::::example{#hai-loi-goi-doi-chieu}
Hàm không đối số, gọi một hàm khác không đối số:

```python title=readonly
def chao():
    return "xin chào"

def goi_chao():
    return chao()

dis.dis(goi_chao)
```

```text title=readonly
  7           RESUME                   0

  8           LOAD_GLOBAL              1 (chao + NULL)
              CALL                     0
              RETURN_VALUE
```

`chao()` không nhận đối số nào — đúng `CALL 0`.

Đổi sang một hàm nhận HAI đối số:

```python title=readonly
def cong(a, b):
    return a + b

def tinh_tong(x, y):
    ket_qua = cong(x, y)
    return ket_qua

dis.dis(tinh_tong)
```

```text title=readonly
  7           RESUME                   0

  8           LOAD_GLOBAL              1 (cong + NULL)
              LOAD_FAST_LOAD_FAST      1 (x, y)
              CALL                     2
              STORE_FAST               2 (ket_qua)

  9           LOAD_FAST                2 (ket_qua)
              RETURN_VALUE
```

`cong(x, y)` nhận đúng hai đối số — `CALL 2`. Con số sau `CALL` đi
thẳng theo số đối số, không đi theo bất cứ thứ gì khác.
::::

::::predict{#y-nghia-con-so-sau-call commitOnce}
Một hàm cộng BA số, gọi với ba giá trị truyền thẳng:

```python
def tron(a, b, c):
    return a + b + c

def goi_tron(x, y, z):
    return tron(x, y, z)
```

`dis.dis(goi_tron)` (đã chạy thật) cho ra:

```text title=readonly
  7           RESUME                   0

  8           LOAD_GLOBAL              1 (tron + NULL)
              LOAD_FAST_LOAD_FAST      1 (x, y)
              LOAD_FAST                2 (z)
              CALL                     3
              RETURN_VALUE
```

Con số `3` đứng ngay sau `CALL` ở đây nghĩa là gì?

:::opt{correct}
Số đối số đã được nạp lên ngăn xếp tính toán ngay trước lệnh `CALL`
này — ở đây là ba: `x`, `y`, `z`
:::

:::opt
Số dòng mã nguồn của hàm `tron`
::why
Gần đúng ở việc bạn đang tìm một con số cố định, gắn liền với hàm
`tron` — đúng tinh thần "có một con số đặc trưng ở đây".

Chỗ lệch: thân hàm `tron` chỉ có ĐÚNG MỘT dòng
(`return a + b + c`), nhưng `CALL` lại ghi số 3 — hai con số này không
khớp nhau, và không có lý do gì để chúng phải khớp. `CALL` không đếm
gì về BÊN TRONG hàm được gọi cả, nó chỉ đếm những gì vừa được NẠP LÊN
NGAY TRƯỚC nó, ở phía hàm đang gọi.
::
:::

:::opt
Số lần hàm `tron` đã từng được gọi trước đó trong chương trình
::why
Gần đúng ở việc bạn cảm thấy con số này phải liên quan tới "chạy bao
nhiêu lần" — cảm giác đó có lý, và một bài sau trong cụm này sẽ đào
đúng vào chuyện đếm số lần chạy.

Chỗ lệch: con số sau `CALL` được ghi cố định ngay trong bản in
`dis.dis()`, TRƯỚC KHI chương trình chạy dù chỉ một lần — nó không thể
đếm việc "đã từng gọi" vì chưa có lần gọi nào xảy ra tại thời điểm dịch
mã nguồn thành bytecode. Nó đếm đúng những giá trị được xếp lên ngăn
xếp CHO lời gọi này, viết sẵn trong mã nguồn.
::
:::

:::opt
Thứ tự lệnh `CALL` này xuất hiện trong toàn bộ danh sách lệnh (lệnh
`CALL` đầu tiên trong chương trình mang số 0, lệnh thứ hai mang số 1,
...)
::why
Gần đúng ở việc bạn nhận ra đây LÀ một con số đếm được — đúng, nó là
một con số đếm thật.

Chỗ lệch: nó không đếm VỊ TRÍ của lệnh `CALL` trong danh sách. Một hàm
chỉ có đúng MỘT lệnh `CALL` duy nhất vẫn có thể mang số khác 0 — ở ví
dụ trên, `CALL 3` là lệnh `CALL` DUY NHẤT trong toàn bộ hàm
`goi_tron`, không phải lệnh thứ tư. Con số ấy đếm đối số, không đếm
thứ tự.
::
:::
::::

::::code{#gap-doi-roi-cong}
Hàm `gap_doi` đã viết sẵn. Hoàn thiện `tinh_tong_gap_doi(a, b)` để nó
trả về GẤP ĐÔI `a`, cộng thêm `b` — bằng cách GỌI `gap_doi`, không tính
`a * 2` trực tiếp.

```python title=starter
import dis

def gap_doi(x):
    return x * 2

def tinh_tong_gap_doi(a, b):
    ket_qua = ___ + b        # gấp đôi a bằng gap_doi, rồi cộng b
    return ket_qua

print(tinh_tong_gap_doi(3, 5))
print(tinh_tong_gap_doi(0, 0))
print(tinh_tong_gap_doi(10, 1))
dis.dis(tinh_tong_gap_doi)
```

```python title=solution
import dis

def gap_doi(x):
    return x * 2

def tinh_tong_gap_doi(a, b):
    ket_qua = gap_doi(a) + b
    return ket_qua

print(tinh_tong_gap_doi(3, 5))
print(tinh_tong_gap_doi(0, 0))
print(tinh_tong_gap_doi(10, 1))
dis.dis(tinh_tong_gap_doi)
```

```python title=test
assert tinh_tong_gap_doi(3, 5) == 11, f"gấp đôi 3 (=6) cộng 5 phải là 11 — đang ra {tinh_tong_gap_doi(3, 5)}"
assert tinh_tong_gap_doi(0, 0) == 0, f"gấp đôi 0 cộng 0 phải là 0 — đang ra {tinh_tong_gap_doi(0, 0)}"
assert tinh_tong_gap_doi(10, 1) == 21, f"gấp đôi 10 (=20) cộng 1 phải là 21 — đang ra {tinh_tong_gap_doi(10, 1)}"
```

:::hints
- kind: attention
  body: Chỉ một chỗ trống — GẤP ĐÔI a bằng cách GỌI hàm gap_doi đã có sẵn, không tính a * 2 trực tiếp. Bài này đang dạy đúng việc gọi hàm là một LỆNH CALL thật, phải hiện ra trên dis.dis().
- kind: strategy
  body: 'Gọi gap_doi với đúng một đối số a — gap_doi(a) — rồi cộng thêm b, khớp đúng công thức "gấp đôi a, cộng b".'
- kind: one-line
  body: 'Chỗ trống là: gap_doi(a)'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ gọi hàm gap_doi — a * 2 cho đúng cùng con số nhưng không dùng lệnh CALL nào, mà bài này đang dạy đúng việc gọi hàm là một lệnh CALL thật
  requireAst:
  # a * 2 + b và gap_doi(a) + b cho ra CÙNG kết quả số học ở MỌI đối số —
  # tương đương thật, không phải do thiếu dữ liệu test. Không assert/output
  # nào phân biệt nổi hai cách viết. Luật này đếm THẬT trên solution:
  # gap_doi(...) xuất hiện đúng 1 lần — min:1 chặn được lối viết a * 2.
  - kind: uses-call, target: gap_doi, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^11\\n0\\n21\\n"
- tier: output
  expect: "CALL"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`CALL 1` — đúng một đối số, đúng như `gap_doi(a)` chỉ cần đúng một giá
trị để chạy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`CALL` vừa chạy xong thì cái gì THẬT SỰ được dựng lên để giữ đối số
`a`, biến `ket_qua`, và biết phải quay lại đúng dòng nào khi xong? Bài
"Máy nhớ đường về" đã gọi nó bằng một cái tên dân dã.

Bài sau gọi tên chính thức của nó.
::::

::::checkpoint{mastery=0.8}
::::
