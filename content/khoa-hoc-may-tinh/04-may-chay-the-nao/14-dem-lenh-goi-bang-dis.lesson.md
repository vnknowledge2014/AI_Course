---
id: khoa-hoc-may-tinh.may-chay-the-nao.dem-lenh-goi-bang-dis
title: "Đếm lệnh CALL thật bằng dis, không đoán"
summary: "dis.dis() cho một con số đo trên MÃ NGUỒN — bao nhiêu dòng CALL xuất hiện trong bản in, viết đúng một lần, không đổi dù chạy với dữ liệu nào. Khác hẳn con số T3.3 đã đếm bằng biến đếm — số lần hàm THỰC SỰ được gọi lúc chạy. Hai con số đo hai chuyện khác nhau, và bài này buộc phân biệt chúng."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.count-calls-via-dis]
requires: [may.return-closes-frame, alg.count-recursive-calls]
concepts: [may.count-calls-via-dis]
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
Một con số đọc trên trang giấy. Một con số đo lúc máy chạy thật. Chúng
không phải luôn bằng nhau.
::::

::::explain{#hai-con-so-khac-nhau}
Bài "Đếm xem tính lại bao nhiêu lần" (T3.3) đếm bằng một biến toàn cục,
tăng lên đúng `1` mỗi khi hàm THỰC SỰ được gọi lúc chạy:
`fib(10)` gọi `177` lần, `fib(15)` gọi `1973` lần — hai con số đo được
SỐNG, ngay trong lúc chương trình chạy, phụ thuộc hẳn vào giá trị `n`
truyền vào.

`dis.dis()` cho một con số HOÀN TOÀN KHÁC: đếm bao nhiêu DÒNG `CALL`
xuất hiện trong CHÍNH BẢN IN. Con số này đo trên MÃ NGUỒN — viết đúng
một lần, đứng yên mãi mãi, KHÔNG phụ thuộc `n` được truyền vào lúc
chạy, vì lúc `dis.dis()` chạy, hàm còn chưa được gọi lần nào cả.

Ví dụ: `dis.dis(fib)` (hàm Fibonacci đệ quy quen thuộc) luôn cho ĐÚNG
HAI dòng `CALL` — một cho `fib(n - 1)`, một cho `fib(n - 2)` — bất kể
`fib` được gọi với `n = 10` hay `n = 10000`. Nhưng số lần nó THỰC SỰ
chạy lại tăng vọt theo `n`, vì đệ quy đi QUA LẠI đúng hai dòng `CALL`
đó rất nhiều lần — `177` lần cho `fib(10)`, con số T3.3 đã đo.

Hai con số này trả lời hai câu hỏi khác nhau:

- "Trong mã nguồn, có bao nhiêu CHỖ gọi hàm?" — đếm bằng `dis.dis()`,
  cố định, không cần chạy.
- "Lúc chạy thật, hàm được gọi bao nhiêu LẦN?" — đếm bằng biến đếm
  (T3.3), phụ thuộc dữ liệu, có thể lớn hơn số CHỖ gọi hàng trăm,
  hàng nghìn lần.

Nhầm hai con số này với nhau là nhầm rất phổ biến — và bài này tồn tại
đúng để chặn đúng chỗ nhầm ấy.
::::

::::example{#giai-thua-mot-cho-goi}
```python title=readonly
def giai_thua(n):
    if n == 0:
        return 1
    return n * giai_thua(n - 1)

dis.dis(giai_thua)
```

```text title=readonly
  4           RESUME                   0

  5           LOAD_FAST                0 (n)
              LOAD_CONST               1 (0)
              COMPARE_OP              88 (bool(==))
              POP_JUMP_IF_FALSE        1 (to L1)

  6           RETURN_CONST             2 (1)

  7   L1:     LOAD_FAST                0 (n)
              LOAD_GLOBAL              1 (giai_thua + NULL)
              LOAD_FAST                0 (n)
              LOAD_CONST               2 (1)
              BINARY_OP               10 (-)
              CALL                     1
              BINARY_OP                5 (*)
              RETURN_VALUE
```

Đúng MỘT dòng `CALL` trong toàn bộ bản in — `giai_thua` chỉ gọi lại
chính nó ở đúng MỘT chỗ trong mã nguồn.

Nhưng chạy thật: `giai_thua(6)` gọi hàm `giai_thua` đúng `7` lần (
`n` lần lượt là `6, 5, 4, 3, 2, 1, 0` — bảy giá trị, bảy lượt gọi, kể
cả lượt cuối cùng khi `n` chạm `0`); `giai_thua(10)` gọi đúng `11`
lần. Một dòng `CALL` trong mã nguồn, nhưng dòng ấy được ĐI QUA nhiều
lần khi đệ quy chạy.
::::

::::predict{#dem-tay-giai-thua commitOnce}
Cùng hàm `giai_thua` ở ví dụ trên. `giai_thua(6)` chạy — hàm
`giai_thua` được GỌI (kể cả lượt gọi đầu tiên, `giai_thua(6)`) tổng
cộng bao nhiêu lần?

:::opt{correct}
7 lần
:::

:::opt
1 lần — vì `dis.dis(giai_thua)` chỉ có đúng MỘT dòng `CALL`
::why
Gần đúng ở việc bạn đọc `dis.dis()` chính xác — quả thật chỉ có đúng
một dòng `CALL` trong bản in.

Chỗ lệch: dòng `CALL` ấy nằm TRONG THÂN của `giai_thua`, và hàm này gọi
lại CHÍNH NÓ — nên khi chương trình chạy, dòng `CALL` DUY NHẤT ấy được
ĐI QUA nhiều lần, mỗi lần đệ quy lùi thêm một bước. Một lệnh `CALL`
trong mã nguồn không giới hạn số lần nó được THỰC THI lúc chạy.
::
:::

:::opt
6 lần — đúng bằng giá trị `n` ban đầu
::why
Gần đúng ở việc bạn đang đếm đúng các bước đệ quy TỪ 6 XUỐNG 1 — sáu
bước giảm dần, đúng như `giai_thua` lùi từng bước một.

Chỗ lệch: bạn bỏ sót lượt gọi CUỐI CÙNG, khi `n` chạm `0`.
`giai_thua(0)` vẫn là một LƯỢT GỌI thật sự — nó vào thân hàm, kiểm tra
`n == 0`, rồi trả về `1` ngay, nhưng bản thân việc BƯỚC VÀO thân hàm ấy
đã được tính là một lần gọi. Đếm đủ cả `6, 5, 4, 3, 2, 1, 0` mới ra
`7`.
::
:::

:::opt
Không đếm được nếu không chạy chương trình thật
::why
Gần đúng ở sự thận trọng: `dis.dis()` một mình đúng là không đủ để nói
số lần gọi LÚC CHẠY — đó chính là điều bài này vừa nhấn mạnh.

Chỗ lệch: bạn không cần CHẠY máy mới đếm được ở đây. `giai_thua` chỉ có
đúng MỘT nhánh đệ quy (không rẽ hai như `fib`), giảm `n` đi đúng `1`
mỗi lần, dừng khi chạm `0` — cấu trúc ấy đếm được bằng tay, không cần
chạy máy: đúng `7` bước cho `n = 6`.
::
:::
::::

::::code{#hai-con-so-tren-cung-mot-ham}
Hai con số cần điền — một đọc trên bản in `dis.dis(giai_thua)` bên
dưới (đã chạy sẵn), một đếm SỐNG bằng biến đếm khi `giai_thua(6)` thực
sự chạy.

```python title=starter
# dis.dis(giai_thua) đã chạy sẵn:
#   4           RESUME                   0
#   5           LOAD_FAST                0 (n)
#               LOAD_CONST               1 (0)
#               COMPARE_OP              88 (bool(==))
#               POP_JUMP_IF_FALSE        1 (to L1)
#   6           RETURN_CONST             2 (1)
#   7   L1:     LOAD_FAST                0 (n)
#               LOAD_GLOBAL              1 (giai_thua + NULL)
#               LOAD_FAST                0 (n)
#               LOAD_CONST               2 (1)
#               BINARY_OP               10 (-)
#               CALL                     1
#               BINARY_OP                5 (*)
#               RETURN_VALUE

so_lenh_call_trong_ma_nguon = ___    # đếm số dòng "CALL" trong bản in trên

so_lan_goi = 0

def giai_thua(n):
    global so_lan_goi
    ___                                # tăng bộ đếm lên 1, mỗi lần hàm được GỌI
    if n == 0:
        return 1
    return n * giai_thua(n - 1)

giai_thua(6)
so_lan_goi_that_su = so_lan_goi

print(so_lenh_call_trong_ma_nguon)
print(so_lan_goi_that_su)
```

```python title=solution
so_lenh_call_trong_ma_nguon = 1

so_lan_goi = 0

def giai_thua(n):
    global so_lan_goi
    so_lan_goi += 1
    if n == 0:
        return 1
    return n * giai_thua(n - 1)

giai_thua(6)
so_lan_goi_that_su = so_lan_goi

print(so_lenh_call_trong_ma_nguon)
print(so_lan_goi_that_su)
```

```python title=test
assert so_lenh_call_trong_ma_nguon == 1, f"chỉ đúng MỘT dòng CALL xuất hiện trong bản in dis.dis(giai_thua) — đang ra {so_lenh_call_trong_ma_nguon}"
assert so_lan_goi_that_su == 7, f"giai_thua(6) phải GỌI giai_thua đúng 7 lần (n = 6,5,4,3,2,1,0) — đang ra {so_lan_goi_that_su}"
assert so_lan_goi_that_su != so_lenh_call_trong_ma_nguon, "hai con số này ĐO HAI CHUYỆN KHÁC NHAU — số lệnh CALL viết trong mã nguồn và số lần hàm thực sự chạy — chúng không nên bằng nhau ở bài này"
```

:::hints
- kind: attention
  body: Chỗ trống 1 chỉ cần đếm bằng MẮT trên bản in dis.dis() đã cho — không chạy gì cả. Chỗ trống 2 nằm TRONG thân hàm, phải tăng bộ đếm mỗi lần hàm THỰC SỰ được gọi — dùng đúng công cụ so_lan_goi += 1 đã học ở T3.3.
- kind: strategy
  body: 'Chỗ trống 1 — đếm dòng "CALL" trong bản in: chỉ có một dòng, ở dưới nhãn L1. Chỗ trống 2 — cộng thêm 1 vào biến toàn cục đã khai global: so_lan_goi += 1, chạy ngay khi thân hàm bắt đầu, trước cả nhánh dừng.'
- kind: one-line
  body: 'Chỗ trống 1 là 1; chỗ trống 2 là so_lan_goi += 1.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: chỗ trống 2 phải THẬT SỰ tăng so_lan_goi lên 1 mỗi lần hàm chạy — không phải một câu không làm gì như True, 1, 0
  requireAst:
  # Đếm thật trên solution: so_lan_goi được GÁN đúng 2 lần trong mã nguồn —
  # "so_lan_goi = 0" (đã có sẵn trong khung) và "so_lan_goi += 1" (chỗ
  # trống 2). Điền True/1/0 (một câu không làm gì) vào chỗ trống 2 chỉ còn
  # 1 lần gán — dưới 2, luật này chặn được. ĐÃ THỬ THẬT bằng cả ba cách
  # True/1/0: không đệ quy nào mất nhánh dừng (n == 0 và lời gọi
  # giai_thua(n - 1) nguyên vẹn trong khung), nên cả ba đều DỪNG AN TOÀN,
  # không lặp/đệ quy vô hạn — nhưng so_lan_goi_that_su khi đó luôn bằng 0,
  # thiếu hẳn 7, bị assert thứ hai bắt độc lập với luật static này.
  - kind: gan-ten, target: so_lan_goi, min: 2
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: "^1\\n7\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dòng CALL trên trang. Bảy lượt gọi lúc chạy thật. Hai con số, hai
câu chuyện — giờ bạn phân biệt được cả hai.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa đếm được hai con số khác nhau, bằng hai công cụ khác nhau. Cả
hai lần, bạn đều ĐỌC một bản in `dis.dis()` đã có sẵn — máy chạy trước,
bạn đọc kết quả sau.

Nếu đảo ngược thứ tự đó: cho một hàm nhỏ, BẠN tự viết ra dự đoán chuỗi
lệnh TRƯỚC — dùng đúng những cái tên đã học suốt mười bốn bài
(`LOAD_FAST`, `BINARY_OP`, `CALL`, `RETURN_VALUE`, các lệnh nhảy) — rồi
mới chạy `dis.dis()` thật để đối chiếu, dự đoán của bạn có khớp máy
thật không?

Bài sau trả lời — và chốt lại cụm này.
::::

::::checkpoint{mastery=0.8}
::::
