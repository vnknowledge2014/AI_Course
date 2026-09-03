---
id: toan.to-hop-xac-suat-thong-ke.phuong-sai-mau
title: Phương sai mẫu và độ lệch chuẩn mẫu
summary: "Phương sai mẫu s² = (Σ(xᵢ−x̄)²)/n — CÙNG công thức Var(X) (bài 26) áp lên dữ liệu THẬT thay vì phân phối lý thuyết, dùng x̄ (bài 27) thay E[X]. Độ lệch chuẩn mẫu s = √s² — đo dữ liệu \"TẢN RA\" xa x̄ bao nhiêu."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 31
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.sample-variance]
requires: [math.mode]
concepts: [math.phuong-sai-mau]
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
Byte (có mùa mất trắng) và Lan (đều đặn quanh 40) có thể CÙNG `x̄`
gần bằng nhau. Có cách nào đo NGAY sự khác biệt "đồng đều hay
bấp bênh" đó không?
::::

::::explain{#phuong-sai-mau-la-gi}
Có. **Phương sai mẫu `s² = (Σ(xᵢ−x̄)²)/n`** — CÙNG công thức
`Var(X)` (bài 26) áp lên dữ liệu THẬT thay vì phân phối LÝ THUYẾT,
dùng `x̄` (bài 27) thay `E[X]`:

```python title=readonly
def phuong_sai_mau(du_lieu, x_tb):
    return sum((x - x_tb) ** 2 for x in du_lieu) / len(du_lieu)


mua_byte = [40, 42, 41, 39, 43, 5]

print(round(phuong_sai_mau(mua_byte, 35), 2))
```

```text title=readonly
181.67
```

Mùa `5` kg CÁCH `x̄=35` những `30` — bình phương khoảng cách ĐÓ
(`900`) áp đảo NĂM số hạng còn lại, kéo `s²` LÊN rất cao (`≈181.67`,
làm TRÒN hai chữ số thập phân cho DỄ đọc — con số THẬT có đuôi thập
phân dài, T2.1 đã dạy số thực là liên tục).
::::

::::example{#do-lech-chuan-mau}
`s = √s²` — ĐƯA đơn vị TRỞ LẠI kilôgam (giống `σ`, bài 26):

```python title=readonly
def phuong_sai_mau(du_lieu, x_tb):
    return sum((x - x_tb) ** 2 for x in du_lieu) / len(du_lieu)


mua_byte = [40, 42, 41, 39, 43, 5]
s2 = phuong_sai_mau(mua_byte, 35)

print(round(s2 ** 0.5, 2))
```

```text title=readonly
13.48
```

`s ≈ 13.48` kg — "trung bình" MỘT mùa CÁCH `x̄` khoảng `13.48` kg.
Con số NÀY LỚN (so với `x̄=35`) VÌ mùa `5` kg KÉO mạnh — dữ liệu của
Lan (đồng đều hơn) sẽ cho `s` NHỎ hơn NHIỀU.
::::

::::predict{#doan-du-lieu-deu-tam commitOnce}
Byte thử dữ liệu bốn mùa GIỐNG hệt nhau (`10` kg cả bốn — hoàn toàn
KHÔNG "tản ra"):

```python
def phuong_sai_mau(du_lieu, x_tb):
    return sum((x - x_tb) ** 2 for x in du_lieu) / len(du_lieu)

mua_deu = [10, 10, 10, 10]
print(phuong_sai_mau(mua_deu, 10))
```

Dòng cuối in ra gì?

:::opt{correct}
`0.0`
:::

:::opt
Máy báo lỗi khi chạy — `phuong_sai_mau` cần dữ liệu có ÍT NHẤT một
giá trị KHÁC `x_tb`, nếu MỌI giá trị TRÙNG `x_tb` thì phép tính
"trở nên vô nghĩa"
::why
Gần đúng ở việc bạn nghĩ "phương sai cần sự KHÁC BIỆT mới có Ý
nghĩa" — một trực giác hợp lý VỀ mặt Ý NGHĨA của "độ tản ra".

Chỗ lệch: KHÔNG có gì "vô nghĩa" khi mọi giá trị TRÙNG `x_tb` — mỗi
khoảng cách `(x−x_tb)` đơn giản LÀ `0`, bình phương VẪN LÀ `0`, cộng
bốn số `0` chia `4` RA `0.0`. Đây chính LÀ kết quả ĐÚNG — dữ liệu
KHÔNG tản ra chút nào thì `s²` PHẢI LÀ `0`, không phải lỗi.
::
:::

:::opt
`10.0` — vì `phuong_sai_mau` "kế thừa" giá trị `x_tb` khi mọi phần
tử ĐỀU bằng nó, tương tự cách kỳ vọng "kế thừa" giá trị chắc chắn
::why
Gần đúng ở việc bạn liên hệ TỚI bài 26 (`Var` của một giá trị chắc
chắn) — một sự liên hệ đúng HƯỚNG.

Chỗ lệch: bài 26 ĐÃ chỉ ra chính XÁC ngược lại — `Var` của một giá
trị chắc chắn LÀ `0`, KHÔNG "kế thừa" giá trị đó. Ở ĐÂY cũng vậy:
`s²` đo KHOẢNG CÁCH bình phương tới `x_tb`, và khoảng cách LÀ `0`
cho MỌI phần tử — `s²=0.0`, không phải `10.0`.
::
:::
::::

::::code{#viet_phuong_sai_mau}
Viết `phuong_sai_mau(du_lieu, x_tb)` — tính `s² = Σ(xᵢ−x_tb)²/n`.

```python title=starter
def phuong_sai_mau(du_lieu, x_tb):
    return ___


du_lieu = [1, 2, 3, 4, 5]
x_tb = 3

print(phuong_sai_mau(du_lieu, x_tb))
```

```python title=solution
def phuong_sai_mau(du_lieu, x_tb):
    return sum((x - x_tb) ** 2 for x in du_lieu) / len(du_lieu)


du_lieu = [1, 2, 3, 4, 5]
x_tb = 3

print(phuong_sai_mau(du_lieu, x_tb))
```

```python title=test
assert phuong_sai_mau([10, 10, 10, 10], 10) == 0.0, "khong tan ra chut nao"
assert phuong_sai_mau([0, 10], 5) == 25.0, "hai gia tri cach deu 5"
assert phuong_sai_mau([1, 2, 3, 4, 5], 3) == 2.0, "phai khop bai chinh"
```

:::hints
- kind: attention
  body: "Dung sum() voi generator: binh phuong khoang cach (x - x_tb), chia cho len(du_lieu)."
- kind: strategy
  body: "sum((x - x_tb) ** 2 for x in du_lieu) / len(du_lieu)"
- kind: one-line
  body: "___ = sum((x - x_tb) ** 2 for x in du_lieu) / len(du_lieu)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai binh phuong khoang cach (x - x_tb) roi chia tong cho len(du_lieu)
  requireAst:
  - kind: uses-call, target: sum, min: 1
  - kind: uses-call, target: len, min: 1
  - kind: uses-operator, target: '**', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^2\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`s` đo được sự "bấp bênh" của MỘT khu vườn. Có cách nào mô tả HÌNH
DÁNG chi tiết hơn một con số trung tâm không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sáu mùa của Byte (có mùa mất trắng) VÀ sáu mùa của Lan (đều đặn
quanh 40) có THỂ cùng `x̄` gần bằng nhau, NHƯNG CHẮC CHẮN khác `s`.
`s` LỚN hơn nói lên điều gì về ĐỘ ĐÁNG TIN của việc dùng `x̄` để dự
đoán mùa TỚI?
::::

::::checkpoint{mastery=0.8}
::::
