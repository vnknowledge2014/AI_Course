---
id: khoa-hoc-may-tinh.may-chay-the-nao.may-tinh-bang-ngan-xep
title: "Máy tính bằng một ngăn xếp riêng, không phải bằng biến"
summary: "LOAD_FAST ĐẨY giá trị vào một ngăn xếp tính toán tạm — không phải biến a, b bạn đặt tên; BINARY_OP LẤY hai đỉnh ra, tính, đẩy kết quả lại — đúng kỷ luật vào-sau-ra-trước của ds.stack, chỉ khác đây là ngăn xếp của trình thông dịch."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.eval-stack]
requires: [may.instruction-atomic, ds.stack]
concepts: [may.eval-stack]
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
Không trôi nổi đâu cả. Có một chỗ để tạm — và bạn đã học đúng kỷ luật
của nó rồi, ở một bài khác.
::::

::::explain{#ngan-xep-cua-may}
Nhớ lại bài "Ngăn xếp: vào sau, ra trước": một cấu trúc dựng từ `list`,
chỉ cho hai việc — `.append(x)` thêm vào đuôi, `.pop()` lấy đúng phần tử
vừa thêm gần đây nhất ra. Luật của nó: vào sau, ra trước.

Trình thông dịch Python có đúng một cấu trúc như vậy, của riêng nó,
không phải `list` bạn tự tay dựng — gọi là **ngăn xếp tính toán**
(evaluation stack). Bạn không viết `.append` hay `.pop` nào cho nó cả;
mỗi lệnh máy bài 1-2 đã học tự động thao tác lên nó, ẩn phía sau:

- `LOAD_FAST` không "thả" giá trị ra giữa không khí. Nó ĐẨY giá trị vào
  đỉnh ngăn xếp tính toán — đúng việc `.append` làm.
- `BINARY_OP` LẤY đúng hai giá trị ở đỉnh ra — đúng việc `.pop()` làm,
  hai lần liền — tính, rồi ĐẨY kết quả trở lại đỉnh.
- `RETURN_VALUE` LẤY đúng một giá trị ở đỉnh ra để trả về.

Một điều cần nói thẳng: giá trị của `a`, `b` KHÔNG hề biến mất khỏi chỗ
giữ biến cục bộ của hàm khi `LOAD_FAST` chạy. Lệnh ấy chỉ SAO một bản
giá trị, đẩy bản sao đó lên ngăn xếp tính toán — chỗ giữ biến gốc vẫn
nguyên, dùng lại được nếu mã nguồn cần đọc `a` một lần nữa.
::::

::::example{#doi-chieu-cong-hai-so}
Cùng hàm `cong` đã dùng ở bài trước:

```text title=readonly
  4           LOAD_FAST_LOAD_FAST      1 (a, b)
              BINARY_OP                0 (+)
              RETURN_VALUE
```

Đi từng lệnh, và theo dõi đúng ngăn xếp tính toán:

| Lệnh chạy | Ngăn xếp tính toán SAU khi lệnh đó chạy |
|---|---|
| `LOAD_FAST_LOAD_FAST (a, b)` | `[gia_tri_a, gia_tri_b]` — hai giá trị, `b` ở đỉnh |
| `BINARY_OP (+)` | `[a_cong_b]` — lấy `b` rồi `a` ra, tính, đẩy một kết quả lại |
| `RETURN_VALUE` | `[]` — lấy nốt kết quả cuối, ngăn xếp rỗng |

Đúng lúc hàm kết thúc, ngăn xếp tính toán rỗng — không sớm, không muộn.
Mỗi lượt đẩy đều có đúng một lượt lấy tương ứng.
::::

::::predict{#du-doan-tinh-abc commitOnce}
```python
def tinh(a, b, c):
    return a + b * c
```

Ngay TRƯỚC KHI lệnh `BINARY_OP` ĐẦU TIÊN chạy (phép nhân — độ ưu tiên
toán tử tính nhân trước cộng), ngăn xếp tính toán đang chứa bao nhiêu
giá trị, và đó là (những) giá trị nào?

:::opt{correct}
Ba giá trị, theo đúng thứ tự đẩy vào: giá trị của a, rồi b, rồi c
:::

:::opt
Hai giá trị: giá trị của a, và kết quả b * c đã tính sẵn
::why
Gần đúng ở việc bạn biết phép nhân `b * c` phải xảy ra TRƯỚC phép cộng
— đúng, độ ưu tiên toán tử đúng.

Chỗ lệch: máy CHƯA tính `b * c` ở thời điểm được hỏi. `BINARY_OP` đầu
tiên (phép nhân) chính là lệnh SẮP chạy, chưa chạy. Trước khi nó chạy,
ngăn xếp chỉ toàn giá trị THÔ vừa lấy ra — chưa có giá trị nào được
TÍNH cả.
::
:::

:::opt
Hai giá trị: giá trị của b, và giá trị của c
::why
Gần đúng ở việc bạn nhận ra `b`, `c` phải có mặt trước khi nhân được —
đúng.

Chỗ lệch: bạn bỏ sót `a`. Vì `a` đứng ở vị trí ĐẦU TIÊN trong biểu thức
`a + b * c`, lệnh lấy giá trị của nó chạy trước cả `b`, `c` — nên khi
tới lượt `BINARY_OP` đầu tiên, `a` đã nằm sẵn trên ngăn xếp từ trước
rồi, chỉ là chưa TỚI LƯỢT dùng tới.
::
:::

:::opt
Một giá trị: giá trị của a
::why
Gần đúng ở việc bạn nhớ đúng `a` được lấy ra đầu tiên.

Chỗ lệch: máy không dừng lại sau khi lấy `a` để "đi tính riêng" `b * c`
rồi mới quay về. Nó tiếp tục LẤY `b`, rồi LẤY `c`, chồng cả ba giá trị
lên ngăn xếp TRƯỚC, rồi mới bắt đầu chạy lệnh `BINARY_OP` nào cả. Lấy
giá trị và tính toán là hai việc tách rời, không xen kẽ như bạn nghĩ.
::
:::
::::

::::code{#doc-tren-ban-in-that}
Dưới đây là bản in `dis.dis()` THẬT của một hàm khác — đã chạy sẵn.
Đừng chạy lại hàm; chỉ ĐỌC đúng bản in này để trả lời.

```python title=readonly
import dis

def tinh_tong_gia(a, b, c):
    return a * b + c

dis.dis(tinh_tong_gia)
```

```text title=readonly
  3           RESUME                   0

  4           LOAD_FAST_LOAD_FAST      1 (a, b)
              BINARY_OP                5 (*)
              LOAD_FAST                2 (c)
              BINARY_OP                0 (+)
              RETURN_VALUE
```

Đếm trên chính bản in ấy, từng lệnh một, từ trên xuống — rồi điền hai
con số.

```python title=starter
so_gia_tri_sau_load_fast_c = ___    # ngay SAU KHI "LOAD_FAST (c)" chạy
so_gia_tri_sau_return = ___         # ngay SAU KHI "RETURN_VALUE" chạy

print(so_gia_tri_sau_load_fast_c)
print(so_gia_tri_sau_return)
```

```python title=solution
so_gia_tri_sau_load_fast_c = 2
so_gia_tri_sau_return = 0

print(so_gia_tri_sau_load_fast_c)
print(so_gia_tri_sau_return)
```

```python title=test
assert so_gia_tri_sau_load_fast_c == 2, f"sau LOAD_FAST_LOAD_FAST (đẩy a,b -> 2), BINARY_OP * (lấy 2 đẩy 1 -> 1), rồi LOAD_FAST c (đẩy thêm 1) -> phải còn đúng 2 giá trị, đang là {so_gia_tri_sau_load_fast_c}"
assert so_gia_tri_sau_return == 0, f"RETURN_VALUE lấy nốt giá trị cuối cùng ra để trả về -> ngăn xếp phải rỗng, đang là {so_gia_tri_sau_return}"
```

:::hints
- kind: attention
  body: Không chạy hàm nào cả — chỉ đếm bằng mắt trên bản in dis.dis() đã có sẵn ở trên, từng lệnh một, từ trên xuống.
- kind: strategy
  body: 'Đi từng lệnh: LOAD_FAST_LOAD_FAST đẩy 2 giá trị (a, b) -> ngăn xếp có 2. BINARY_OP (*) lấy 2 ra, đẩy 1 kết quả lại -> còn 1. LOAD_FAST (c) đẩy thêm 1 -> có 2 — đúng lúc câu hỏi đầu hỏi tới. Đi tiếp: BINARY_OP (+) lấy 2 ra đẩy 1 lại -> còn 1. RETURN_VALUE lấy nốt giá trị cuối đó ra -> còn 0.'
- kind: one-line
  body: Hai chỗ trống lần lượt là 2 và 0.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^2\\n0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba giá trị chồng lên, hai lệnh BINARY_OP lần lượt gỡ bớt, RETURN_VALUE
gỡ nốt giá trị cuối. Ngăn xếp rỗng đúng lúc hàm kết thúc — không sớm,
không muộn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ngăn xếp tính toán này chỉ lo MỘT lượt tính, từ đầu tới cuối, không
quay lại đâu cả. Nhưng khi một đoạn mã cần LẶP LẠI — chạy đi chạy lại
đúng một khối lệnh, như `for` — con trỏ đang đứng trong dãy lệnh phải
làm gì để quay về đúng chỗ cũ và bắt đầu lại?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
