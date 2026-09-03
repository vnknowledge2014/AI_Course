---
id: toan.to-hop-xac-suat-thong-ke.trung-binh-du-lieu-that
title: Trung bình của dữ liệu thật
summary: "Trung bình mẫu x̄ = (Σxᵢ)/n — TÍNH giống hệt tong/dem (R1 bài 27); nhưng khác E[X] (bài 24) ở chỗ x̄ tính từ dữ liệu ĐÃ XẢY RA, E[X] tính từ xác suất DỰ ĐOÁN trước khi xảy ra."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.sample-mean]
requires: [math.variance]
concepts: [math.trung-binh-mau]
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
Byte GIEO THẬT sáu mùa liền, GHI LẠI sáu con số kilôgam thu hoạch.
`tong/dem` (R1) đã biết tính trung bình sáu con số ấy — nó có phải
`E[X]` (bài 24) không?
::::

::::explain{#trung-binh-mau-la-gi}
Không hẳn — cùng CÔNG THỨC, khác NGUỒN. **Trung bình mẫu `x̄ =
(Σxᵢ)/n`** — TÍNH giống hệt `tong/dem` (R1 bài 27, đã biết CÁCH GÕ).
NHƯNG khác `E[X]` (bài 24) Ở CHỖ: `x̄` tính từ dữ liệu ĐÃ XẢY RA
(sáu mùa CÓ THẬT), `E[X]` tính từ xác suất DỰ ĐOÁN TRƯỚC khi xảy ra:

```python title=readonly
def trung_binh(du_lieu):
    return sum(du_lieu) / len(du_lieu)


mua_byte = [40, 42, 41, 39, 43, 5]

print(trung_binh(mua_byte))
```

```text title=readonly
35.0
```

Sáu mùa: `40, 42, 41, 39, 43, 5` (kg). Mùa cuối MẤT MÙA nặng (sâu
bệnh). `x̄ = 210/6 = 35`. CÙNG công thức trọng số đều như `E[X]`
(mỗi mùa "nặng" như nhau, `1/n`), NHƯNG đây LÀ sáu con số THẬT đã
xảy ra, không phải xác suất DỰ ĐOÁN.
::::

::::example{#thu-tu-khong-quan-trong}
Đổi THỨ TỰ ghi chép — `x̄` KHÔNG đổi (phép cộng giao hoán, T2.1 bài
20):

```python title=readonly
def trung_binh(du_lieu):
    return sum(du_lieu) / len(du_lieu)


mua_byte = [40, 42, 41, 39, 43, 5]
mua_byte_sap_xep = [5, 39, 40, 41, 42, 43]

print(trung_binh(mua_byte))
print(trung_binh(mua_byte_sap_xep))
```

```text title=readonly
35.0
35.0
```

CÙNG sáu con số, THỨ TỰ khác — `x̄` KHÔNG đổi. `sum()` cộng dồn
KHÔNG quan tâm thứ tự các số hạng.
::::

::::predict{#doan-trung-binh-them-mua commitOnce}
Byte đổi mùa mất mùa TỪ `5` LÊN `11` (đỡ tệ hơn MỘT chút, năm mùa
kia giữ nguyên):

```python
def trung_binh(du_lieu):
    return sum(du_lieu) / len(du_lieu)

mua_moi = [40, 42, 41, 39, 43, 11]
print(trung_binh(mua_moi))
```

Dòng cuối in ra gì?

:::opt{correct}
`36.0`
:::

:::opt
`35.0` — vì thay đổi CHỈ MỘT trong sáu con số (từ `5` lên `11`) là
một thay đổi NHỎ, không đủ để dịch chuyển trung bình của CẢ sáu mùa
::why
Gần đúng ở việc bạn nghĩ "một thay đổi NHỎ thì ảnh hưởng NHỎ" — một
trực giác hợp lý VỚI nhiều phép đo khác.

Chỗ lệch: `x̄` CỘNG DỒN mọi giá trị RỒI chia — MỖI con số, dù nhỏ
tới đâu, đều góp phần TRỰC TIẾP vào tổng. Đổi `5` thành `11` LÀ cộng
thêm `6` vào tổng (`210 → 216`), chia cho `6` mùa RA `36.0`, không
còn `35.0`. `x̄` NHẠY với TỪNG con số, không có "thay đổi nhỏ không
đáng kể".
::
:::

:::opt
Máy báo lỗi khi chạy — `mua_moi` có giá trị `11` LỚN hơn giá trị cũ
`5` Ở CÙNG vị trí, và `trung_binh` từ chối tính khi dữ liệu KHÔNG
theo thứ tự tăng dần
::why
Gần đúng ở việc bạn để ý giá trị Ở vị trí cuối THAY ĐỔI — một quan
sát đúng VỀ DỮ LIỆU.

Chỗ lệch: `trung_binh` (bài này) KHÔNG hề đòi dữ liệu phải sắp xếp
theo bất kỳ thứ tự nào (đúng như VÍ DỤ Ở TRÊN đã chỉ ra) — nó chỉ
cộng dồn VÀ chia. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_trung_binh}
Viết `trung_binh(du_lieu)` — tính `x̄ = (Σxᵢ)/n`.

```python title=starter
def trung_binh(du_lieu):
    return ___


mua_byte = [40, 42, 41, 39, 43, 5]

print(trung_binh(mua_byte))
```

```python title=solution
def trung_binh(du_lieu):
    return sum(du_lieu) / len(du_lieu)


mua_byte = [40, 42, 41, 39, 43, 5]

print(trung_binh(mua_byte))
```

```python title=test
assert trung_binh([10]) == 10.0, "mot mua duy nhat -- trung binh bang chinh no"
assert trung_binh([10, 10, 10]) == 10.0, "moi mua deu nhau -- trung binh khong doi"
assert trung_binh([5, 39, 40, 41, 42, 43]) == trung_binh([40, 42, 41, 39, 43, 5]), "doi thu tu khong doi trung binh"
assert trung_binh([40, 42, 41, 39, 43, 11]) == 36.0, "doi mot gia tri -- trung binh doi theo"
```

:::hints
- kind: attention
  body: "Chia sum(du_lieu) cho len(du_lieu)."
- kind: strategy
  body: "sum(du_lieu) / len(du_lieu)"
- kind: one-line
  body: "___ = sum(du_lieu) / len(du_lieu)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai chia sum(du_lieu) cho len(du_lieu)
  requireAst:
  - kind: uses-call, target: sum, min: 1
  - kind: uses-call, target: len, min: 1
  - kind: uses-operator, target: '/', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^35\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`x̄ = 35`. Nhưng năm mùa kia đều quanh `40`. `35` có ĐẠI DIỆN đúng
cho "một mùa BÌNH THƯỜNG" không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sáu mùa: `40, 42, 41, 39, 43, 5`. Mùa cuối mất mùa nặng (sâu bệnh).
`x̄ = 35`. Con số `35` có ĐẠI DIỆN đúng cho "một mùa BÌNH THƯỜNG"
của Byte không, hay đang bị MỘT con số bất thường kéo LỆCH?
::::

::::checkpoint{mastery=0.8}
::::
