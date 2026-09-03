---
id: toan.to-hop-xac-suat-thong-ke.trung-vi
title: Trung vị
summary: "Trung vị — sắp dữ liệu THEO THỨ TỰ, lấy giá trị Ở GIỮA (hoặc trung bình hai giá trị giữa nếu n chẵn); KHÔNG cộng dồn như x̄, chỉ quan tâm VỊ TRÍ sau khi sắp — một mùa bất thường không kéo nó đi xa."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.median]
requires: [math.sample-mean]
concepts: [math.trung-vi]
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
`x̄ = 35`, bị mùa `5` kg kéo LỆCH. Có con số nào KHÔNG bị một mùa bất
thường kéo đi xa không?
::::

::::explain{#trung-vi-la-gi}
Có. **Trung vị** — SẮP dữ liệu THEO THỨ TỰ, lấy giá trị Ở GIỮA (hoặc
trung bình hai giá trị GIỮA nếu `n` chẵn); KHÔNG cộng dồn như `x̄`,
CHỈ quan tâm VỊ TRÍ sau khi sắp — một mùa bất thường KHÔNG kéo nó đi
xa:

```python title=readonly
def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]


mua_byte = [40, 42, 41, 39, 43, 5]

print(trung_vi(mua_byte))
```

```text title=readonly
40.5
```

Sắp lại: `5, 39, 40, 41, 42, 43` — SÁU giá trị (chẵn), lấy trung
bình HAI giá trị GIỮA (`40`, `41`) LÀ `40.5`. GẦN năm mùa bình
thường HƠN HẲN `x̄=35` (bài 27).
::::

::::example{#so-le-vs-chan}
Số phần tử LẺ — trung vị LÀ ĐÚNG MỘT giá trị Ở giữa, không cần trung
bình hai giá trị:

```python title=readonly
def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]


print(trung_vi([1, 2, 3]))
```

```text title=readonly
2
```

BA phần tử (lẻ), `giua = 3//2 = 1` — LẤY THẲNG `s[1] = 2`, phần tử
CHÍNH GIỮA sau khi sắp. KHÔNG cần chia đôi cộng lại vì CHỈ có MỘT
phần tử Ở giữa.
::::

::::predict{#doan-trung-vi-ben-ngoai-le commitOnce}
Byte đổi mùa mất mùa TỪ `5` LÊN `11` (như bài 27) — trung vị có đổi
theo không?

```python
def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]

mua_moi = [40, 42, 41, 39, 43, 11]
print(trung_vi(mua_moi))
```

Dòng cuối in ra gì?

:::opt{correct}
`40.5`
:::

:::opt
`43.0` — vì đổi giá trị NHỎ nhất (`5`) thành `11` khiến nó KHÔNG còn
là NHỎ nhất theo một cách nào đó, đẩy TRUNG VỊ dịch lên gần đỉnh dữ
liệu
::why
Gần đúng ở việc bạn nghĩ TỚI việc thay đổi giá trị NHỎ nhất PHẢI ảnh
hưởng tới cấu trúc SẮP XẾP — một trực giác hợp lý VỀ mặt trực quan.

Chỗ lệch: `11` VẪN LÀ giá trị NHỎ nhất trong sáu số (`11 < 39`) —
SẮP lại vẫn LÀ `11, 39, 40, 41, 42, 43`, và HAI giá trị GIỮA VẪN LÀ
`40`, `41` (CÙNG vị trí như trước, chỉ có SỐ NHỎ nhất đổi giá trị,
không đổi VỊ TRÍ). Trung vị KHÔNG hề dịch — vẫn `(40+41)/2=40.5`.
::
:::

:::opt
Máy báo lỗi khi chạy — dữ liệu MỚI có giá trị `11` LỚN hơn `5` NHƯNG
VẪN nhỏ hơn `39`, tạo một "khoảng trống" lớn (`11` tới `39`) mà hàm
không xử lý được
::why
Gần đúng ở việc bạn để ý CÓ một khoảng CÁCH lớn giữa `11` VÀ `39`
trong dữ liệu SAU khi sắp — một quan sát đúng VỀ mặt DỮ LIỆU.

Chỗ lệch: `trung_vi` KHÔNG hề quan tâm KHOẢNG CÁCH giữa các giá trị
liền kề — nó CHỈ cần `sorted()` sắp được (LUÔN sắp được VỚI số), rồi
lấy đúng VỊ TRÍ giữa. "Khoảng trống" lớn hay nhỏ không ảnh hưởng gì.
::
:::
::::

::::code{#viet_trung_vi}
Viết `trung_vi(du_lieu)` — sắp `du_lieu`, trả về giá trị Ở GIỮA (hay
trung bình hai giá trị giữa nếu `n` chẵn).

```python title=starter
def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return ___
    return s[giua]


mua_byte = [40, 42, 41, 39, 43, 5]

print(trung_vi(mua_byte))
```

```python title=solution
def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]


mua_byte = [40, 42, 41, 39, 43, 5]

print(trung_vi(mua_byte))
```

```python title=test
assert trung_vi([1, 2, 3]) == 2, "so le -- lay dung phan tu giua"
assert trung_vi([10]) == 10, "mot phan tu -- chinh no la trung vi"
assert trung_vi([1, 2, 3, 4]) == 2.5, "so chan -- trung binh hai phan tu giua"
assert trung_vi([40, 42, 41, 39, 43, 11]) == 40.5, "doi gia tri nho nhat khong doi trung vi (van nho nhat)"
assert trung_vi([5, 39, 40, 41, 42, 43]) == trung_vi([40, 42, 41, 39, 43, 5]), "thu tu ghi chep khong doi trung vi"
```

:::hints
- kind: attention
  body: "Khi n chan, lay trung binh hai phan tu s[giua-1] va s[giua]."
- kind: strategy
  body: "(s[giua - 1] + s[giua]) / 2"
- kind: one-line
  body: "return (s[giua - 1] + s[giua]) / 2"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai lay trung binh s[giua-1] va s[giua] khi n chan
  requireAst:
  - kind: uses-operator, target: '/', min: 1
  - kind: uses-operator, target: '+', min: 1
  - kind: uses-name, target: giua, min: 3
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^40\.5\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Trung vị bền, trung bình nhạy. Khi nào dùng CÁI nào?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sáu mùa sắp lại: `5, 39, 40, 41, 42, 43`. Trung vị (`40.5`) GẦN năm
mùa bình thường hơn hẳn `x̄=35` (bài 27). Vậy trung vị LUÔN "đáng tin
cậy hơn" trung bình, hay MỖI công cụ có chỗ RIÊNG để dùng?
::::

::::checkpoint{mastery=0.8}
::::
