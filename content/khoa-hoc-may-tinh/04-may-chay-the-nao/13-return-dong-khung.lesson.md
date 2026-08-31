---
id: khoa-hoc-may-tinh.may-chay-the-nao.return-dong-khung
title: "RETURN_VALUE đóng khung, trả kết quả về"
summary: "RETURN_VALUE lấy giá trị đỉnh ngăn xếp tính toán CỦA KHUNG ĐANG ĐÓNG, đẩy đúng giá trị ấy vào ngăn xếp CỦA KHUNG VỪA GỌI nó — đúng chỗ CALL đang đứng. Đối xứng với CALL (bài 11) mở khung: RETURN_VALUE đóng khung, và giá trị nó mang theo là món quà để lại cho khung gọi nó."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.return-closes-frame]
requires: [may.call-frame]
concepts: [may.return-closes-frame]
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
Mở khung là một lệnh. Đóng khung cũng vậy — và bạn đã biết tên nó từ
rất lâu rồi.
::::

::::explain{#dong-khung-va-chuyen-gia-tri}
Bài 1-3 đã nói `RETURN_VALUE` lấy đúng MỘT giá trị ở đỉnh ngăn xếp tính
toán ra để trả về — đúng, nhưng chưa nói hết. Ngăn xếp tính toán đó
không phải một cái DÙNG CHUNG cho cả chương trình — mỗi khung (bài
trước) có ngăn xếp tính toán CỦA RIÊNG NÓ. Câu đầy đủ:

`RETURN_VALUE` lấy giá trị từ đỉnh ngăn xếp tính toán của khung ĐANG
CHẠY (khung sắp đóng), rồi đẩy ĐÚNG giá trị ấy vào đỉnh ngăn xếp tính
toán của khung VỪA GỌI nó — đúng chỗ lệnh `CALL` (bài 11) đang đứng
đợi.

Sau khi giá trị đã chuyển chỗ, khung vừa chạy xong không còn ĐANG THỰC
THI nữa — quyền điều khiển (khung nào đang chạy, con trỏ lệnh nào đang
được đọc) trả hẳn về khung đã gọi nó. Đó là nghĩa của "đóng khung" ở
bài này: không phải khung biến mất khỏi bộ nhớ ngay lập tức (bài trước
cho thấy một khung vẫn giữ nguyên `f_locals` nếu có biến nào đó còn giữ
tham chiếu tới nó) — mà là khung đó không còn đứng đầu ngăn xếp gọi
hàm, không còn là nơi trình thông dịch (bài 8) đang trỏ tới.

Đối xứng gọn: `CALL` MỞ một khung. `RETURN_VALUE` ĐÓNG nó — và giá trị
nó mang theo chính là thứ khung ấy để lại cho khung đã gọi nó.
::::

::::example{#gia-tri-doi-cho}
Cùng cặp hàm bài trước đã dùng:

```python title=readonly
def cong(a, b):
    return a + b

dis.dis(cong)
```

```text title=readonly
  4           RESUME                   0

  5           LOAD_FAST_LOAD_FAST      1 (a, b)
              BINARY_OP                0 (+)
              RETURN_VALUE
```

```python title=readonly
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

Đi theo đúng một giá trị, qua cả hai khung:

- `RETURN_VALUE` cuối cùng của `cong` lấy `a + b` từ đỉnh ngăn xếp CỦA
  `cong`, đóng khung `cong` lại, và đẩy đúng con số ấy vào đỉnh ngăn
  xếp CỦA `tinh_tong` — đúng chỗ `CALL` vừa đứng.
- `STORE_FAST 2 (ket_qua)` lấy giá trị đó ra khỏi ngăn xếp của
  `tinh_tong`, cất vào biến `ket_qua`.
- `RETURN_VALUE` cuối cùng của `tinh_tong` lại lấy đúng giá trị ấy (giờ
  đọc lại bằng `LOAD_FAST`), đóng khung `tinh_tong`, đẩy nó tiếp lên
  ngăn xếp của khung đã gọi `tinh_tong`.

Một giá trị, đi qua hai lần đóng khung, không đổi dạng, không rơi rớt ở
đâu cả.
::::

::::predict{#ngan-xep-cua-khung-giua commitOnce}
```python
def a_ham(n):
    return n + 100

def b_ham(n):
    ket_qua = a_ham(n)
    return ket_qua * 2

print(b_ham(5))
```

Ngay SAU KHI `RETURN_VALUE` của `a_ham` chạy xong (nhưng TRƯỚC KHI phép
nhân `* 2` trong `b_ham` kịp chạy), đỉnh ngăn xếp tính toán CỦA KHUNG
`b_ham` đang có gì?

:::opt{correct}
`105` — kết quả `a_ham(5)` vừa trả về, đúng chỗ `CALL` vừa đứng
:::

:::opt
Rỗng — `a_ham` đóng khung trước khi kịp để lại gì cho `b_ham`
::why
Gần đúng ở việc bạn nhớ đúng thứ tự: `a_ham` đóng khung trước khi
`b_ham` chạy tiếp — đúng, đó chính là lúc đang được hỏi tới.

Chỗ lệch: "đóng khung" không có nghĩa là ĐI TAY KHÔNG. `RETURN_VALUE`
của `a_ham` đẩy đúng giá trị `105` vào ngăn xếp của `b_ham` TRƯỚC KHI
đóng khung của chính nó — hai việc xảy ra cùng một lệnh, không phải
đóng khung rồi mới nghĩ tới chuyện để lại gì.
::
:::

:::opt
Toàn bộ khung của `a_ham` — cả biến `n` lẫn con trỏ lệnh của nó, không
chỉ một giá trị
::why
Gần đúng ở việc bạn nhớ đúng rằng khung `a_ham` có nhiều thứ bên trong
— tên hàm, biến cục bộ, con trỏ lệnh (bài trước).

Chỗ lệch: `RETURN_VALUE` chỉ chuyển đúng MỘT GIÁ TRỊ — giá trị đang nằm
ở đỉnh ngăn xếp tính toán. Nó không sao chép cả khung, không mang theo
biến `n` hay bất cứ thứ gì khác của `a_ham` sang cho `b_ham`. Đúng một
con số, không hơn.
::
:::

:::opt
`10` — kết quả đã nhân đôi
::why
Gần đúng ở việc bạn tính đúng con số CUỐI CÙNG của cả chương trình
— sai đơn vị nhưng đúng phép tính `* 2` sẽ diễn ra.

Chỗ lệch: câu hỏi chốt ở thời điểm TRƯỚC KHI phép nhân `* 2` kịp chạy.
Phép nhân đó nằm trong THÂN của `b_ham`, chạy SAU khi `CALL` gọi
`a_ham` đã hoàn tất và giá trị `105` đã nằm sẵn trên ngăn xếp — chưa
tới lượt `BINARY_OP` nào chạy cả ở thời điểm được hỏi.
::
:::
::::

::::code{#doc-ngan-xep-qua-return}
Đọc bản in `dis.dis()` THẬT dưới đây — không chạy hàm nào. Đếm trên
chính ngăn xếp tính toán CỦA `tinh_tong` (không phải của `cong`).

```python title=readonly
# dis.dis(cong):
#   4           RESUME                   0
#   5           LOAD_FAST_LOAD_FAST      1 (a, b)
#               BINARY_OP                0 (+)
#               RETURN_VALUE
#
# dis.dis(tinh_tong):
#   7           RESUME                   0
#   8           LOAD_GLOBAL              1 (cong + NULL)
#               LOAD_FAST_LOAD_FAST      1 (x, y)
#               CALL                     2
#               STORE_FAST               2 (ket_qua)
#   9           LOAD_FAST                2 (ket_qua)
#               RETURN_VALUE
```

```python title=starter
so_gia_tri_sau_call = ___                    # trên ngăn xếp CỦA tinh_tong, ngay SAU KHI CALL chạy xong
so_gia_tri_sau_return_cuoi = ___              # ngay SAU KHI RETURN_VALUE cuối cùng của tinh_tong chạy

print(so_gia_tri_sau_call)
print(so_gia_tri_sau_return_cuoi)
```

```python title=solution
so_gia_tri_sau_call = 1
so_gia_tri_sau_return_cuoi = 0

print(so_gia_tri_sau_call)
print(so_gia_tri_sau_return_cuoi)
```

```python title=test
assert so_gia_tri_sau_call == 1, f"CALL đóng khung của cong lại và đẩy đúng MỘT giá trị (kết quả cong) vào ngăn xếp của tinh_tong — phải là 1, đang ra {so_gia_tri_sau_call}"
assert so_gia_tri_sau_return_cuoi == 0, f"RETURN_VALUE cuối cùng lấy nốt giá trị đó ra khỏi ngăn xếp của tinh_tong để đóng khung tinh_tong — ngăn xếp phải rỗng, đang ra {so_gia_tri_sau_return_cuoi}"
```

:::hints
- kind: attention
  body: Không chạy hàm nào cả — chỉ đếm bằng mắt trên bản in đã có sẵn, và luôn đếm trên ngăn xếp CỦA tinh_tong, không phải của cong (khung của cong đã đóng, ngăn xếp của nó không còn liên quan nữa).
- kind: strategy
  body: 'CALL đóng khung của cong (nó đã tự đóng bằng RETURN_VALUE riêng của nó) và để lại đúng một giá trị — kết quả a + b — trên ngăn xếp của tinh_tong. STORE_FAST lấy giá trị đó cất vào ket_qua, ngăn xếp còn 0. LOAD_FAST đọc lại ket_qua, đẩy lên 1. RETURN_VALUE cuối cùng lấy nốt giá trị đó ra, ngăn xếp còn 0.'
- kind: one-line
  body: Hai chỗ trống lần lượt là 1 và 0.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^1\\n0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một giá trị, đóng một khung, mở lại đúng một chỗ trên ngăn xếp của
khung đã gọi nó. Không rơi rớt ở đâu cả.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa đếm bằng mắt trên một bản in đã có sẵn. Nhưng T3.3 từng đếm số
lần một hàm được gọi bằng một cách hoàn toàn khác — một biến đếm, tăng
lên MỖI LẦN hàm THỰC SỰ chạy. Hai cách đếm đó có luôn cho cùng một con
số không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
