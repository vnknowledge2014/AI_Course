---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.ghep-doi-hoi-tung-mat-xich-thuan
title: "Ghép được AN TOÀN đòi hỏi MỖI mắt xích phải thuần"
summary: "Nếu MỘT hàm trong chuỗi pipe không thuần (ví dụ gọi random.randint() giữa chừng), cả PIPELINE mất khả năng dự đoán — pipe(x, f, g, h) không còn cho cùng kết quả với cùng x, dù f, h đều thuần."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [fp.composability-needs-purity]
requires: [fp.real-pipeline, fp.pure-fn-def]
concepts: [fp.composability-needs-purity]
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
Bài trước ghép bốn hàm THUẦN thành một pipeline dự đoán được. Hôm nay:
điều gì xảy ra nếu MỘT mắt xích không thuần?
::::

::::explain{#mot-mat-xich-hong-pha-ca-chuoi}
Nhớ lại T4.1: một hàm THUẦN, gọi lại CÙNG đối số, LUÔN ra CÙNG kết quả.
`pipe(x, f, g, h)` — nếu CẢ BA hàm `f`, `g`, `h` đều thuần, cả chuỗi
cũng THUẦN (gọi lại `pipe(x, f, g, h)` với CÙNG `x` luôn ra CÙNG kết
quả). Nhưng chỉ cần MỘT trong ba không thuần:

```python
import random

def pipe(value, *ham):
    ket_qua = value
    for f in ham:
        ket_qua = f(ket_qua)
    return ket_qua

cong_1 = lambda x: x + 1
random_hoa = lambda x: x + random.randint(0, 1000000)
nhan_2 = lambda x: x * 2

a = pipe(5, cong_1, random_hoa, nhan_2)
b = pipe(5, cong_1, random_hoa, nhan_2)
print(a == b)
```

```text
False
```

`cong_1` và `nhan_2` — CẢ HAI đều thuần. Chỉ `random_hoa` (mắt xích
GIỮA CHỪNG) không thuần. Nhưng kết quả CUỐI CÙNG của `pipe(5, cong_1,
random_hoa, nhan_2)` vẫn KHÔNG dự đoán được — gọi lại với CÙNG `5`,
KHÔNG ra cùng kết quả. MỘT mắt xích không thuần ĐỦ để phá tính dự đoán
được của CẢ chuỗi, dù hai đầu (đầu vào cố định, hai hàm còn lại đều
thuần) trông có vẻ "an toàn".

Đây là lý do "ghép được AN TOÀN" (composability) ĐÒI HỎI mọi mắt xích
phải thuần — không phải sở thích phong cách, mà là ĐIỀU KIỆN để cả
pipeline giữ được tính chất dự đoán được mà từng hàm thuần lẻ có.
::::

::::example{#mot-mat-xich-du-de-pha}
Không cần TOÀN BỘ chuỗi không thuần — chỉ MỘT mắt xích ở BẤT KỲ vị trí
nào cũng đủ:

```python title=readonly
import random

def pipe(value, *ham):
    ket_qua = value
    for f in ham:
        ket_qua = f(ket_qua)
    return ket_qua

cong_1 = lambda x: x + 1
nhan_2 = lambda x: x * 2
tru_3 = lambda x: x - 3
random_hoa = lambda x: x + random.randint(0, 1000000)

# random_hoa ở ĐẦU chuỗi
c1a = pipe(5, random_hoa, cong_1, nhan_2)
c1b = pipe(5, random_hoa, cong_1, nhan_2)

# random_hoa ở CUỐI chuỗi
c2a = pipe(5, cong_1, nhan_2, random_hoa)
c2b = pipe(5, cong_1, nhan_2, random_hoa)

print(c1a == c1b)
print(c2a == c2b)
```

```text title=readonly
False
False
```

VỊ TRÍ của mắt xích không thuần (đầu, giữa, hay cuối chuỗi) không quan
trọng — CÓ MẶT một mắt xích không thuần LÀ ĐỦ để phá tính dự đoán được
của TOÀN BỘ pipeline, bất kể nó đứng ở đâu.
::::

::::predict{#doan-mat-xich-hong-giua-chung commitOnce}
```python
import random

def pipe(value, *ham):
    ket_qua = value
    for f in ham:
        ket_qua = f(ket_qua)
    return ket_qua

cong_thue = lambda gia: gia * 1.1
them_phi_ngau_nhien = lambda gia: gia + random.randint(1, 1000000)
lam_tron = lambda gia: round(gia)

a = pipe(100, cong_thue, them_phi_ngau_nhien, lam_tron)
b = pipe(100, cong_thue, them_phi_ngau_nhien, lam_tron)
print(a == b)
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì `cong_thue` và `lam_tron` (hai đầu chuỗi) đều thuần, và
"đa số thuần" đủ để cả chuỗi dự đoán được
::why
Gần đúng ở việc bạn xác định ĐÚNG `cong_thue` và `lam_tron` đều thuần —
cả hai hàm đó không có dấu hiệu không thuần nào.

Chỗ lệch: không có khái niệm "đa số thuần thì đủ" — CHỈ CẦN MỘT mắt
xích không thuần (`them_phi_ngau_nhien`, gọi `random.randint`) là ĐỦ để
phá tính dự đoán được của CẢ chuỗi, bất kể bao nhiêu mắt xích khác vẫn
thuần. `a == b` vẫn là `False`.
::
:::

:::opt
Máy báo lỗi — `pipe` không cho phép TRỘN hàm thuần và hàm không thuần
trong CÙNG một chuỗi
::why
Gần đúng ở việc bạn cảnh giác về việc TRỘN hai loại hàm khác nhau
trong cùng một chuỗi — một mối lo hợp lý khi mới gặp khái niệm này.

Chỗ lệch: `pipe` KHÔNG hề kiểm tra hay CẤM việc trộn hàm thuần với
không thuần — nó chạy TẤT CẢ, không phân biệt loại. Vấn đề không phải
"chạy được hay không" (chạy được, không lỗi), mà là "kết quả có dự
đoán được không" (không, một khi có mắt xích không thuần).
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ghép hàm AN TOÀN không phải chuyện may rủi — nó đòi hỏi MỖI mắt xích
phải thuần. Một hàm không thuần, dù nhỏ, đủ để phá cả chuỗi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Biết một pipeline CÓ THỂ hỏng vì một mắt xích không thuần là một
chuyện — nhưng khi nó THẬT SỰ ra kết quả sai, làm sao TÌM ĐÚNG mắt
xích gây ra, mà không phải tách cả chuỗi ra chạy từng phần?

Bài sau đưa ra một công cụ cho đúng việc đó.
::::

::::checkpoint{mastery=0.8}
::::
