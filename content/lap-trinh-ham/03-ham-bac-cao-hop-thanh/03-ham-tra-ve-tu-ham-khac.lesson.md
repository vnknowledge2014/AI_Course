---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.ham-tra-ve-tu-ham-khac
title: "Một hàm TRẢ VỀ một hàm khác"
summary: "def nhan_voi(k): return lambda x: x * k rồi gap_ba = nhan_voi(3); gap_ba(5) == 15 — hàm ngoài TRẢ VỀ một hàm mới, đã 'nhớ' k."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [fp.function-as-return]
requires: [fp.function-as-parameter]
concepts: [fp.function-as-return]
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
Hàm truyền được vào hàm khác (bài trước). Chiều ngược lại thì sao — một
hàm TRẢ VỀ được một hàm khác không?
::::

::::explain{#ham-tra-ve-ham}
`return` không chỉ trả về số hay chuỗi — nó trả về ĐƯỢC một hàm:

```python
def nhan_voi(k):
    return lambda x: x * k

gap_ba = nhan_voi(3)
print(gap_ba(5))
print(gap_ba(10))
```

```text
15
30
```

`nhan_voi(3)` KHÔNG trả về một số — nó trả về MỘT HÀM MỚI (`lambda x: x
* 3`), gán vào tên `gap_ba`. Gọi `gap_ba(5)` sau đó là gọi CHÍNH hàm mới
này, và nó "nhớ" `k = 3` dù `nhan_voi(3)` đã chạy xong từ lâu. Gọi lại
`nhan_voi` với một số khác, được một hàm KHÁC:

```python
gap_muoi = nhan_voi(10)
print(gap_muoi(5))
```

```text
50
```

`gap_ba` và `gap_muoi` là HAI hàm riêng biệt, mỗi cái "nhớ" một `k`
khác nhau — dù cả hai đều sinh ra từ CÙNG một `nhan_voi`. (Cách một hàm
"nhớ" được biến của hàm ngoài sau khi hàm ngoài đã chạy xong gọi là
**closure** — track sẽ đào sâu ở cụm sau; bài này chỉ cần thấy "hàm trả
về hàm" là chuyện hợp lệ.)
::::

::::example{#nha-may-tao-ham}
`nhan_voi` giống một "nhà máy tạo hàm" — mỗi lần gọi, ra một hàm mới,
"cài sẵn" một con số khác:

```python title=readonly
def nhan_voi(k):
    return lambda x: x * k

cac_he_so = [2, 3, 5]
cac_ham = [nhan_voi(k) for k in cac_he_so]

for ham in cac_ham:
    print(ham(10))
```

```text title=readonly
20
30
50
```

Một `list` các HÀM, mỗi cái "cài sẵn" một hệ số khác nhau (`2`, `3`,
`5`), tạo ra từ MỘT vòng lặp gọi `nhan_voi` nhiều lần. Không có ba định
nghĩa `def` viết tay riêng lẻ nào cả — một hàm nhà máy, gọi lặp lại.
::::

::::predict{#doan-ham-nho-gia-tri commitOnce}
```python
def tao_ham_giam_gia(phan_tram):
    return lambda gia: gia * (1 - phan_tram / 100)

giam_20 = tao_ham_giam_gia(20)
giam_50 = tao_ham_giam_gia(50)

print(giam_20(100000))
print(giam_50(100000))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`80000.0` rồi `50000.0`
:::

:::opt
`80000.0` rồi `80000.0` — vì `giam_50` được tạo SAU `giam_20`, nên nó
"kế thừa" và ghi đè `phan_tram`
::why
Gần đúng ở việc bạn nhớ đúng `giam_20` ra `80000.0` — phép tính đó đúng.

Chỗ lệch: `giam_20` và `giam_50` là HAI hàm HOÀN TOÀN ĐỘC LẬP, mỗi cái
sinh ra từ MỘT lần gọi `tao_ham_giam_gia` RIÊNG, với `phan_tram` RIÊNG
(`20` và `50`). Không có "kế thừa" hay "ghi đè" nào giữa hai lần gọi —
`giam_50` chỉ nhớ `phan_tram=50` của chính nó, cho ra `50000.0`.
::
:::

:::opt
Máy báo lỗi — không định nghĩa được hai hàm khác nhau từ CÙNG một
`lambda` trong `tao_ham_giam_gia`
::why
Gần đúng ở việc bạn nghi ngờ đúng chỗ tế nhị: CÙNG một dòng `lambda`
trong mã nguồn, nhưng lại sinh ra hai hàm khác nhau — nghe có vẻ mâu
thuẫn.

Chỗ lệch: không mâu thuẫn gì cả. MỖI LẦN gọi `tao_ham_giam_gia(...)`,
Python tạo một `lambda` MỚI, với `phan_tram` của riêng lần gọi đó — dù
dòng mã định nghĩa `lambda` chỉ viết MỘT LẦN trong `tao_ham_giam_gia`.
::
:::

:::opt
`100000` rồi `100000` — vì `gia * (1 - phan_tram/100)` chỉ tính được khi
biết `gia` TRƯỚC, mà `gia` truyền vào SAU nên bị bỏ qua
::why
Gần đúng ở việc bạn để ý đúng thứ tự: `phan_tram` được truyền vào TRƯỚC
(lúc gọi `tao_ham_giam_gia`), `gia` truyền vào SAU (lúc gọi `giam_20`).

Chỗ lệch: đúng thứ tự đó KHÔNG có nghĩa `gia` bị bỏ qua — hàm được trả
về (`lambda gia: ...`) CHỜ tới khi có `gia` mới thật sự tính, và khi
`giam_20(100000)` chạy, `gia` NHẬN giá trị `100000`, tính ra
`100000 * 0.8 = 80000.0`, không phải giữ nguyên `100000`.
::
:::
::::

::::code{#tao_ham_cong}
Viết `tao_ham_cong(n)` — trả về một hàm CỘNG THÊM `n` vào bất kỳ số nào
đưa vào.

```python title=starter
def tao_ham_cong(n):
    ___

cong_5 = tao_ham_cong(5)
cong_10 = tao_ham_cong(10)

print(cong_5(3))
print(cong_10(3))
```

```python title=solution
def tao_ham_cong(n):
    return lambda x: x + n

cong_5 = tao_ham_cong(5)
cong_10 = tao_ham_cong(10)

print(cong_5(3))
print(cong_10(3))
```

```python title=test
assert cong_5(3) == 8, "cong_5(3) phải là 5 + 3 = 8"
assert cong_10(3) == 13, "cong_10(3) phải là 10 + 3 = 13"
assert cong_5(100) == 105, "phải đúng với một đối số KHÁC ví dụ trên"
cong_1000 = tao_ham_cong(1000)
assert cong_1000(1) == 1001, "gọi tao_ham_cong lần THỨ BA vẫn phải tạo đúng hàm mới"
```

:::hints
- kind: attention
  body: Chỗ trống là toàn bộ thân hàm tao_ham_cong — nó phải TRẢ VỀ một hàm khác (dùng lambda), không trả về một số.
- kind: strategy
  body: 'return lambda x: x + n — hàm được trả về nhận MỘT đối số x, cộng với n (n là tham số của tao_ham_cong, "nhớ" được bên trong lambda).'
- kind: one-line
  body: "return lambda x: x + n"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: tao_ham_cong phải TRẢ VỀ một hàm (dùng lambda), không trả về một số đã tính sẵn.
  requireAst:
  - kind: lambda, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "8"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một hàm sinh ra hàm khác, mỗi lần một bản riêng, nhớ đúng con số nó vừa
nhận. `cong_5` và `cong_10` không hề đụng nhau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cả ba bài vừa qua đều dùng `lambda` mà chưa hỏi kỹ nó LÀ GÌ — chỉ một
hàm không tên, viết gọn trên một dòng. Nó có giới hạn gì so với `def`
không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
