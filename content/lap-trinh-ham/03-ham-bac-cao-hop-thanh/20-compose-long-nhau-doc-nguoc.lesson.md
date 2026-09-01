---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.compose-long-nhau-doc-nguoc
title: "Ghép NHIỀU `compose` lồng nhau — thứ tự đọc ngược trực giác"
summary: "compose(str, compose(nhan_doi, cong_mot)) — chạy cong_mot → nhan_doi → str, nhưng ĐỌC TỪ TRÁI SANG PHẢI trong mã lại thấy str trước. Bài predict: đoán THỨ TỰ THỰC THI (không phải thứ tự viết)."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [fp.compose-nested]
requires: [fp.compose-basics]
concepts: [fp.compose-nested]
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
`compose(f, g)` ghép được HAI hàm. Ghép BA hàm thì sao — lồng
`compose` vào chính `compose`?
::::

::::explain{#long-compose}
```python
def compose(f, g):
    return lambda x: f(g(x))

cong_mot = lambda x: x + 1
nhan_doi = lambda x: x * 2

ham = compose(str, compose(nhan_doi, cong_mot))
print(ham(3))
print(type(ham(3)))
```

```text
8
<class 'str'>
```

`compose(nhan_doi, cong_mot)` (bên TRONG) tạo MỘT hàm mới trước — gọi
nó là `buoc_trong`, chạy `cong_mot` rồi `nhan_doi`. Rồi
`compose(str, buoc_trong)` (bên NGOÀI) ghép `str` với `buoc_trong` đó.

Thứ tự THỰC THI khi gọi `ham(3)`:

1. `cong_mot(3) = 4` — chạy TRƯỚC TIÊN (nằm SÂU NHẤT, sát `x` nhất)
2. `nhan_doi(4) = 8` — chạy THỨ HAI
3. `str(8) = "8"` — chạy CUỐI CÙNG (nằm NGOÀI CÙNG bên trái khi viết)

ĐỌC mã từ TRÁI sang PHẢI (`str`, rồi `compose(nhan_doi, cong_mot)` bên
trong đó lại là `nhan_doi`, rồi `cong_mot`), bạn thấy `str` TRƯỚC TIÊN —
nhưng nó chạy CUỐI CÙNG. Thứ tự ĐỌC và thứ tự CHẠY hoàn toàn NGƯỢC
nhau. Đây không phải lỗi — nó là hệ quả TẤT YẾU của cách `f(g(x))` lồng
nhau: hàm SÂU NHẤT (gần `x` nhất) LUÔN phải chạy trước để có kết quả
cho hàm bên ngoài dùng.
::::

::::example{#hinh-dung-tung-lop}
Hình dung `compose` lồng nhau như BÓC một củ hành từ NGOÀI vào TRONG
khi ĐỌC, nhưng CHẠY lại từ TRONG ra NGOÀI:

```python title=readonly
def compose(f, g):
    return lambda x: f(g(x))

def ghi_lai(nhan):
    return lambda x: (print(f"{nhan}: nhận {x}"), x)[1]

buoc_a = ghi_lai("A")
buoc_b = ghi_lai("B")
buoc_c = ghi_lai("C")

ham = compose(buoc_a, compose(buoc_b, buoc_c))
ham(1)
```

```text title=readonly
C: nhận 1
B: nhận 1
A: nhận 1
```

`buoc_c` (nằm TRONG CÙNG, viết SAU CÙNG khi đọc) IN RA TRƯỚC TIÊN —
đúng thứ tự CHẠY, không phải thứ tự VIẾT. `A`, `B`, `C` đều nhận `1`
làm đối số ở đây (`ghi_lai` không đổi giá trị, chỉ IN RA rồi trả về y
hệt) — chỉ để lộ rõ THỨ TỰ chạy, không phải giá trị bị biến đổi.
::::

::::predict{#doan-thu-tu-thuc-thi commitOnce}
```python
def compose(f, g):
    return lambda x: f(g(x))

gap_doi = lambda x: x * 2
tru_2 = lambda x: x - 2
binh_phuong = lambda x: x * x

ham = compose(binh_phuong, compose(tru_2, gap_doi))
print(ham(5))
```

Dòng cuối in ra gì?

:::opt{correct}
`64`
:::

:::opt
`36` — vì `binh_phuong` viết TRƯỚC TIÊN (ngoài cùng bên trái), nên nó
chạy TRƯỚC TIÊN: `binh_phuong(5) = 25`, rồi `tru_2(25) = 23`, rồi
`gap_doi(23) = 46`... (tính sai theo hướng này ra một số khác `36`)
::why
Gần đúng ở việc bạn để ý ĐÚNG vị trí VIẾT của `binh_phuong` (ngoài
cùng, trái nhất) — vị trí đó đúng như bạn thấy.

Chỗ lệch: `compose` chạy NGƯỢC thứ tự viết — hàm SÂU NHẤT trong lồng
ghép (`gap_doi`, nằm TRONG CÙNG) chạy TRƯỚC, không phải hàm viết trước.
Thứ tự THẬT: `gap_doi(5) = 10` trước, rồi `tru_2(10) = 8`, rồi
`binh_phuong(8) = 64` — cuối cùng mới tới hàm viết ngoài cùng.
::
:::

:::opt
`9` — vì `compose(tru_2, gap_doi)` chạy trước CẢ hàm ngoài, tính
`gap_doi(5) - 2` sai công thức thành `10 - 2 = 8` rồi bình phương SỐ MŨ
2 nhầm thành cộng thêm 1
::why
Gần đúng ở việc bạn xác định ĐÚNG `compose(tru_2, gap_doi)` chạy TRƯỚC
`binh_phuong` — thứ tự lồng đó đúng, và `gap_doi(5) - 2` cũng tính đúng
hướng.

Chỗ lệch: phép tính cuối — `binh_phuong(8)` nghĩa là `8 * 8 = 64`
(bình phương ĐÚNG NGHĨA, nhân chính nó), không phải `8 + 1 = 9`. Kết
quả cuối cùng đúng phải là `64`, không phải `9`.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`compose` lồng nhau chạy từ TRONG ra NGOÀI — ngược thứ tự viết. Đọc mã
không đủ để biết thứ tự chạy; phải hiểu cấu trúc lồng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Thứ tự đọc-ngược-thứ-tự-chạy này dễ gây nhầm — nhất là khi ghép NHIỀU
hơn ba hàm. Có cách viết nào để thứ tự ĐỌC và thứ tự CHẠY trùng nhau,
không phải lật ngược trong đầu mỗi lần đọc?

Bài sau giải quyết đúng vấn đề này.
::::

::::checkpoint{mastery=0.8}
::::
