---
id: toan.to-hop-xac-suat-thong-ke.quy-tac-nhan
title: Quy tắc nhân
summary: "Khi một lựa chọn có m cách, lựa chọn tiếp theo (độc lập) có n cách, tổng số cách là m × n — chính là |A×B|=|A|·|B| (T2.4 bài 15) áp dụng lên nhiều bước liên tiếp."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.rule-of-product]
requires: [math.inclusion-exclusion-three]
concepts: [math.quy-tac-nhan]
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
Bốn luống, MỖI luống chọn 1 trong 3 loại hạt giống, ĐỘC LẬP nhau.
Đếm hết mọi cách chọn — liệt kê tay còn nổi không?
::::

::::explain{#quy-tac-nhan-la-gi}
Không nổi tay, nhưng KHÔNG cần liệt kê. **Quy tắc nhân** — một lựa
chọn có `m` cách, lựa chọn TIẾP THEO (không phụ thuộc lựa chọn
trước) có `n` cách, tổng số cách LÀ `m × n`; đúng phép nhân của tích
Descartes (T2.4 bài 15, `|A×B|=|A|·|B|`) áp lên NHIỀU bước liên tiếp:

```python title=readonly
def dem_cach_chon_lien_tiep(cac_so):
    ket_qua = 1
    for so in cac_so:
        ket_qua = ket_qua * so
    return ket_qua


print(dem_cach_chon_lien_tiep([3, 3, 3, 3]))
```

```text title=readonly
81
```

Bốn luống, MỖI luống 3 lựa chọn ĐỘC LẬP — `3×3×3×3 = 81`. Vòng lặp
NHÂN DỒN (đúng cấu trúc cộng dồn R1 bài 12, chỉ đổi `+` thành `*`) —
bắt đầu từ `1` (giống `0` là điểm bắt đầu của cộng dồn), vì nhân với
`1` không đổi gì (T2.1 bài 20 đã dạy `1` là phần tử trung hoà của
nhân).
::::

::::example{#hai-buoc-khac-nhau}
Số lựa chọn KHÔNG cần GIỐNG nhau mỗi bước — quy tắc nhân vẫn áp:

```python title=readonly
def dem_cach_chon_lien_tiep(cac_so):
    ket_qua = 1
    for so in cac_so:
        ket_qua = ket_qua * so
    return ket_qua


# luống 1: 3 loại hạt; luống 2 (đất hẹp hơn): chỉ 2 loại phù hợp
print(dem_cach_chon_lien_tiep([3, 2]))
```

```text title=readonly
6
```

Luống 1 có 3 lựa chọn, luống 2 CHỈ có 2 (đất hẹp không hợp loại thứ
ba) — `3×2=6` cách, KHÔNG phải `3×3`. Quy tắc nhân đếm ĐÚNG số lựa
chọn THỰC của TỪNG bước, không giả định chúng bằng nhau.
::::

::::predict{#doan-mot-buoc-khong-co-lua-chon commitOnce}
Byte thêm luống thứ ba — NHƯNG đất luống này chỉ hợp ĐÚNG MỘT loại
hạt (không có lựa chọn khác):

```python
def dem_cach_chon_lien_tiep(cac_so):
    ket_qua = 1
    for so in cac_so:
        ket_qua = ket_qua * so
    return ket_qua

print(dem_cach_chon_lien_tiep([3, 2, 1]))
```

Dòng cuối in ra gì?

:::opt{correct}
`6`
:::

:::opt
`0` — nhân với `1` (một lựa chọn DUY NHẤT) coi như "không có gì để
chọn", nên toàn bộ kết quả sụp về `0`
::why
Gần đúng ở việc bạn cảm nhận "chỉ một lựa chọn" nghe GẦN như "không
có lựa chọn" — một trực giác dễ hiểu nhầm.

Chỗ lệch: "chỉ một lựa chọn" và "không có lựa chọn nào" là HAI tình
huống KHÁC hẳn nhau. Một lựa chọn nghĩa là bước đó KHÔNG làm thay
đổi tổng số cách — nhân với `1` giữ NGUYÊN kết quả (T2.1 bài 20, `1`
là phần tử trung hoà). `0` lựa chọn (không loại hạt nào hợp đất) mới
là trường hợp khiến TOÀN BỘ sụp về `0` — vì không có cách nào hoàn
thành bước đó, nên không có cách nào hoàn thành CẢ DÃY.
::
:::

:::opt
Máy báo lỗi khi chạy — danh sách `[3, 2, 1]` có phần tử `1`, và nhân
với `1` là một PHÉP TOÁN thừa, Python cảnh báo rồi dừng
::why
Gần đúng ở việc bạn để ý con số `1` xuất hiện trong danh sách — một
quan sát đúng VỀ dữ liệu.

Chỗ lệch: Python KHÔNG hề cấm hay cảnh báo nhân với `1` — đó là một
phép nhân HOÀN TOÀN bình thường, kết quả giữ nguyên số đang có. Biên
dịch sạch, chạy sạch, không có gì "thừa" về mặt kỹ thuật.
::
:::
::::

::::code{#viet_dem_cach_chon_lien_tiep}
Viết `dem_cach_chon_lien_tiep(cac_so)` — nhân dồn TOÀN BỘ các số
trong `cac_so`, áp dụng quy tắc nhân.

```python title=starter
def dem_cach_chon_lien_tiep(cac_so):
    ket_qua = 1
    for so in cac_so:
        ket_qua = ___
    return ket_qua


print(dem_cach_chon_lien_tiep([3, 3, 3, 3]))
```

```python title=solution
def dem_cach_chon_lien_tiep(cac_so):
    ket_qua = 1
    for so in cac_so:
        ket_qua = ket_qua * so
    return ket_qua


print(dem_cach_chon_lien_tiep([3, 3, 3, 3]))
```

```python title=test
assert dem_cach_chon_lien_tiep([]) == 1, "day rong -- khong buoc nao, ket qua giu nguyen 1"
assert dem_cach_chon_lien_tiep([5]) == 5, "mot buoc duy nhat -- ket qua bang chinh so do"
assert dem_cach_chon_lien_tiep([3, 2]) == 6, "3 x 2 = 6"
assert dem_cach_chon_lien_tiep([3, 2, 1]) == 6, "nhan voi 1 khong doi ket qua"
assert dem_cach_chon_lien_tiep([2, 0, 5]) == 0, "mot buoc khong co lua chon nao -- toan bo sup ve 0"
```

:::hints
- kind: attention
  body: "Nhan don vao ket_qua moi luot -- doi phep cong dong (R1) thanh phep nhan."
- kind: strategy
  body: "ket_qua * so"
- kind: one-line
  body: "ket_qua = ket_qua * so"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai nhan don ket_qua voi so moi luot, dung vong lap da co
  requireAst:
  - kind: uses-operator, target: '*', min: 1
  - kind: uses-name, target: ket_qua, min: 1
  - kind: uses-name, target: so, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^81\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhân dồn xong. Nhưng nếu túi CHỈ CÓ đúng bốn hạt (dùng rồi thì hết),
mỗi luống một hạt khác nhau — quy tắc nhân còn tính đúng không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Túi CHỈ có đúng bốn hạt giống, KHÔNG lặp — Byte muốn gieo CẢ BỐN hạt
vào bốn luống, mỗi luống một hạt, MỌI thứ tự gieo đều tính là một
cách khác nhau. Nhân `4×3×2×1` được bao nhiêu — và con số CUỐI (nhân
tới `1`) này có phải LÚC NÀO cũng vậy không?
::::

::::checkpoint{mastery=0.8}
::::
