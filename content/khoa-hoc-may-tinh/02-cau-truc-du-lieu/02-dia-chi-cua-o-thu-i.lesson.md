---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.dia-chi-cua-o-thu-i
title: "Địa chỉ của ô thứ i tính được, không cần dò"
summary: "Địa chỉ của ô thứ i tính bằng một phép nhân — địa_chỉ_gốc + i × cỡ_một_ô — không phải một cuộc dò tìm; đó là lý do list[0] và list[999] nhanh như nhau."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ds.array-index-address]
requires: [ds.array-contiguous, mem.address, mem.name-is-reference, core.arithmetic, core.function-def, core.function-return, core.function-call, core.function-parameter, core.variable, core.assignment, core.fstring, core.builtin-function]
concepts: [ds.array-index-address]
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
Máy không đi bộ qua từng tủ để tìm tủ số 999. Nó nhân.
::::

::::explain{#cong-thuc-mot-phep-nhan}
Bài trước để lại đúng một câu hỏi: máy tìm ra chỗ của ô thứ `i` trong bộ
nhớ bằng cách nào, mà không cần đi qua mọi ô trước nó?

Câu trả lời nằm ngay trong chính định nghĩa của mảng: các ô LIỀN KỀ, và mọi
ô CÙNG CỠ. Hai điều đó cộng lại cho ra một công thức:

> **địa_chỉ(i) = địa_chỉ_gốc + i × cỡ_một_ô**

Biết địa chỉ của ô số 0 (gọi là địa_chỉ_gốc), biết mỗi ô rộng bao nhiêu
(cỡ_một_ô), bạn tính thẳng ra địa chỉ của ô số `i` bằng một phép NHÂN rồi
một phép CỘNG — không cần bước qua ô số 0, rồi ô số 1, rồi ô số 2... cho
tới khi tới ô số `i`. Với dãy tủ khoá: biết tủ số 0 đứng ở đâu, biết mỗi
tủ rộng bao nhiêu xăng-ti-mét, bạn tính ngay được tủ số 12 đứng ở đâu, không
cần bước qua đủ 12 cái tủ trước nó.

Đây là lý do `tu_khoa[0]` và `tu_khoa[999]` (nếu dãy đủ dài) nhanh như
nhau: cả hai đều chỉ là MỘT phép tính, khác nhau đúng ở con số `i` được
đưa vào — không phải một cuộc dò tìm dài ngắn tuỳ vị trí.

Còn "cỡ_một_ô" là gì trong một `list` Python? Nhớ lại bài `mem.name-is-
reference` (T3.1): một cái tên không GIỮ giá trị, nó CHỈ TỚI giá trị. Mỗi
ô của một `list` Python cũng vậy — nó không chứa trực tiếp con số hay
chuỗi bạn nhìn thấy, nó chứa một THAM CHIẾU (một địa chỉ) trỏ tới nơi giá
trị thật đang nằm. Và vì mọi ô đều chỉ chứa MỘT tham chiếu — dù giá trị nó
trỏ tới là số `1` bé xíu hay một chuỗi dài cả nghìn ký tự — cỡ_một_ô luôn
CỐ ĐỊNH, không phụ thuộc giá trị bên trong to hay nhỏ. Đó chính là điều
khiến công thức phía trên chạy được: không có "cỡ_một_ô" cố định thì không
có phép nhân nào tính thẳng ra địa chỉ cả.
::::

::::example{#tinh-dia-chi-bang-cong-thuc}
Viết đúng công thức đó thành một hàm Python, rồi thử với vài chỉ số cách
xa nhau:

```python title=readonly
def dia_chi_o(dia_chi_goc, i, co_mot_o):
    return dia_chi_goc + i * co_mot_o

dia_chi_goc = 1000
co_mot_o = 4

print(dia_chi_o(dia_chi_goc, 0, co_mot_o))
print(dia_chi_o(dia_chi_goc, 5, co_mot_o))
print(dia_chi_o(dia_chi_goc, 999, co_mot_o))
```

```text title=readonly
1000
1020
4996
```

Ba lời gọi, ba chỉ số cách xa nhau — 0, 5, 999 — và không lời gọi nào "mất
công" hơn lời gọi kia. Mỗi lần chỉ là một phép nhân rồi một phép cộng, bất
kể `i` là 0 hay 999.

Con số `co_mot_o = 4` không phải bịa. Đo thật trên `list` Python (máy
32-bit WASM mà khoá này chạy): mỗi lần một `list` cần thêm chỗ cho nhiều ô
hơn, kích thước của nó (`sys.getsizeof`) tăng thêm đúng **4 byte** cho MỖI
ô thêm vào — dù ô đó sắp chứa một số `0` hay một chuỗi dài:

```python title=readonly
import sys

a = [0, 1, 2, 3]
print(sys.getsizeof(a))

a.append(4)
a.append(5)
a.append(6)
a.append(7)
print(sys.getsizeof(a))
```

```text title=readonly
44
60
```

Thêm đúng 4 ô (từ 4 lên 8 phần tử), kích thước tăng đúng 16 byte — 16 chia
cho 4 ô, ra đúng 4 byte một ô. Đó là cỡ của MỘT THAM CHIẾU trên máy 32-bit
này: mỗi ô của `list` chỉ đủ rộng để chứa một địa chỉ, không phải giá trị
thật.
::::

::::predict{#doan-dia-chi-tu-so-8 commitOnce}
Đúng hàm `dia_chi_o` ở trên, Byte gọi nó với một bộ tham số khác: gốc
`2000`, chỉ số `10`, mỗi ô rộng `8`.

```python
def dia_chi_o(dia_chi_goc, i, co_mot_o):
    return dia_chi_goc + i * co_mot_o

print(dia_chi_o(2000, 10, 8))
```

**Trước khi chạy**, bạn đoán kết quả là bao nhiêu?

:::opt{correct}
`2080`
:::

:::opt
`2088`
::why
Gần đúng ở việc bạn dùng đúng phép NHÂN giữa chỉ số và cỡ một ô — không
sai kỹ thuật của công thức.

Chỗ lệch là bạn đang đếm ô đầu tiên là ô số 1 thay vì ô số 0 (dùng `i + 1`
thay vì `i`). Mảng luôn đánh số từ 0 (bài trước) — chỉ số `i = 10` nghĩa
là ô thứ MƯỜI MỘT nếu đếm theo lối quen thuộc từ nhỏ, nhưng công thức này
đúng nghĩa là ô mang SỐ 10, không dịch thêm một bậc nào.
::
:::

:::opt
`2018`
::why
Gần đúng ở việc bạn cộng đúng con số `co_mot_o` vào — không quên mất nó.

Chỗ lệch là mỗi ô cách ô kế đúng `co_mot_o` byte, và có TỚI 10 ô cần vượt
qua để tới ô thứ 10 — phải NHÂN `co_mot_o` với `i`, không phải cộng suông
một lần duy nhất.
::
:::

:::opt
`80`
::why
Gần đúng ở phần nhân `10 × 8 = 80` — đó đúng là QUÃNG ĐƯỜNG phải đi thêm
kể từ ô gốc để tới ô thứ 10.

Chỗ lệch là bạn quên cộng lại điểm XUẤT PHÁT — địa chỉ gốc `2000`. Quãng
đường đi thêm phải cộng vào chỗ ĐANG ĐỨNG, không phải tự nó là kết quả
cuối cùng.
::
:::
::::

::::code{#dia-chi-hai-tu-khoa}
Một dãy tủ khoá khác bắt đầu ở vị trí 100 dọc bức tường, mỗi tủ rộng 4
xăng-ti-mét. Byte cần biết tủ số 5 và tủ số 12 đứng ở vị trí nào.

Điền vào thân hàm `dia_chi_o` đúng công thức bài này vừa học.

```python title=starter
def dia_chi_o(dia_chi_goc, i, co_mot_o):
    return ___

dia_chi_goc = 100
co_mot_o = 4

vi_tri_tu_5 = dia_chi_o(dia_chi_goc, 5, co_mot_o)
vi_tri_tu_12 = dia_chi_o(dia_chi_goc, 12, co_mot_o)

print(f"Tủ số 5 đứng ở vị trí {vi_tri_tu_5}")
print(f"Tủ số 12 đứng ở vị trí {vi_tri_tu_12}")
```

```python title=solution
def dia_chi_o(dia_chi_goc, i, co_mot_o):
    return dia_chi_goc + i * co_mot_o

dia_chi_goc = 100
co_mot_o = 4

vi_tri_tu_5 = dia_chi_o(dia_chi_goc, 5, co_mot_o)
vi_tri_tu_12 = dia_chi_o(dia_chi_goc, 12, co_mot_o)

print(f"Tủ số 5 đứng ở vị trí {vi_tri_tu_5}")
print(f"Tủ số 12 đứng ở vị trí {vi_tri_tu_12}")
```

```python title=test
# Ba cỡ ô khác nhau (4, rồi 10) và ba gốc khác nhau (0, 100, 500) — hàm
# phải đúng với MỌI bộ tham số, không chỉ đúng với bộ (100, 5, 4) mà bài
# vừa dùng để in ra kết quả.
assert dia_chi_o(0, 0, 4) == 0, "địa chỉ ô số 0 phải trùng đúng địa chỉ gốc, không cộng thêm gì"
assert dia_chi_o(100, 5, 4) == 120, "địa chỉ ô thứ 5, gốc 100, mỗi ô rộng 4: phải là 100 + 5×4 = 120"
assert dia_chi_o(100, 12, 4) == 148, "địa chỉ ô thứ 12, gốc 100, mỗi ô rộng 4: phải là 100 + 12×4 = 148"
assert dia_chi_o(500, 3, 10) == 530, "công thức phải đúng với MỌI cỡ ô, không chỉ cỡ 4 — gốc 500, mỗi ô rộng 10, ô thứ 3: 500 + 3×10 = 530"
assert vi_tri_tu_5 == 120, "vi_tri_tu_5 phải bằng dia_chi_o(100, 5, 4) = 120"
assert vi_tri_tu_12 == 148, "vi_tri_tu_12 phải bằng dia_chi_o(100, 12, 4) = 148"
```

:::hints
- kind: attention
  body: Công thức đã có sẵn trong phần giải thích phía trên — bài này không đòi bạn NGHĨ RA công thức, chỉ đòi bạn GÕ đúng nó vào thân hàm.
- kind: strategy
  body: "địa_chỉ(i) = địa_chỉ_gốc + i × cỡ_một_ô. Ba tham số của hàm đúng theo thứ tự đó — dia_chi_goc, i, co_mot_o — nên chỉ cần ghép chúng lại đúng công thức, không cần đổi tên hay đổi thứ tự tham số."
- kind: one-line
  body: "Điền `dia_chi_goc + i * co_mot_o` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Tủ số 5 đứng ở vị trí 120\nTủ số 12 đứng ở vị trí 148\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một phép nhân, một phép cộng — không có bước nào là "đi bộ qua từng tủ".
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Công thức vừa học ngầm giả định một điều: dãy ô đã có ĐỦ chỗ, mọi chỉ số
`i` bạn đưa vào đều trỏ tới một ô THẬT SỰ tồn tại.

Nhưng khi bạn gọi `.append(...)` để thêm một phần tử MỚI — một phần tử mà
lúc dãy mới dựng còn chưa có chỗ dành sẵn — chuyện gì xảy ra? Ô liền kề
tiếp theo có luôn luôn còn trống để dùng ngay không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
