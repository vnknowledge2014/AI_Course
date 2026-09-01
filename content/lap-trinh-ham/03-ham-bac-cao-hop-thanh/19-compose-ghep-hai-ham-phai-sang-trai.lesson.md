---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.compose-ghep-hai-ham-phai-sang-trai
title: "`compose(f, g)` — ghép hai hàm, chạy PHẢI-SANG-TRÁI"
summary: "def compose(f, g): return lambda x: f(g(x)) rồi compose(str, lambda x: x*2)(4) == '8' — chạy x*2 TRƯỚC, str SAU. Đọc từ PHẢI sang TRÁI, ngược thứ tự viết."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [fp.compose-basics]
requires: [fp.review-closures]
concepts: [fp.compose-basics]
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
Cụm trước: "chốt sẵn" một phần dữ liệu. Cụm này mở đầu một Ý khác: nối
NHIỀU hàm nhỏ thành MỘT hàm lớn, chạy TUẦN TỰ.
::::

::::explain{#compose-la-gi}
```python
def compose(f, g):
    return lambda x: f(g(x))

nhan_doi = lambda x: x * 2
ham = compose(str, nhan_doi)
print(ham(4))
print(type(ham(4)))
```

```text
8
<class 'str'>
```

`compose(f, g)` trả về một closure (bài 12!) nhận `x`, rồi tính
`f(g(x))` — chạy `g` TRƯỚC, lấy kết quả đưa vào `f`. `ham = compose(str,
nhan_doi)` rồi `ham(4)`: `g = nhan_doi` chạy trước (`4 * 2 = 8`), rồi
`f = str` chạy sau (`str(8) = "8"`, một CHUỖI, không phải số nguyên).

Điểm DỄ NHẦM nhất: đọc `compose(str, nhan_doi)` từ TRÁI sang PHẢI (thứ
tự VIẾT), bạn thấy `str` TRƯỚC — nhưng nó chạy SAU. `compose` chạy
PHẢI-SANG-TRÁI: hàm viết CUỐI CÙNG (gần đối số nhất khi gọi lồng nhau,
`f(g(x))`) chạy TRƯỚC. Đây không phải lỗi hay điều gì lạ — nó phản ánh
đúng cách viết `f(g(x))` bằng tay: `g(x)` LUÔN phải tính XONG trước khi
`f(...)` có thứ để nhận.
::::

::::example{#khong-phai-khai-niem-moi}
`compose` không giới thiệu hạ tầng mới — nó GHÉP đúng những mảnh đã có
(hàm là giá trị, hàm trả về hàm) theo một cách dùng mới:

```python title=readonly
def compose(f, g):
    return lambda x: f(g(x))

cong_1 = lambda x: x + 1
nhan_doi = lambda x: x * 2

tang_roi_nhan = compose(nhan_doi, cong_1)
nhan_roi_tang = compose(cong_1, nhan_doi)

print(tang_roi_nhan(3))
print(nhan_roi_tang(3))
```

```text title=readonly
8
7
```

`tang_roi_nhan(3)`: `cong_1` chạy trước (`3 + 1 = 4`), rồi `nhan_doi`
chạy sau (`4 * 2 = 8`). `nhan_roi_tang(3)`: `nhan_doi` chạy trước
(`3 * 2 = 6`), rồi `cong_1` chạy sau (`6 + 1 = 7`). CÙNG hai hàm, THỨ
TỰ ghép khác nhau — kết quả khác nhau. `compose(a, b)` KHÁC
`compose(b, a)`, giống hệt bài 11's bài học về thứ tự `map`/`filter`.
::::

::::predict{#doan-thu-tu-compose commitOnce}
```python
def compose(f, g):
    return lambda x: f(g(x))

binh_phuong = lambda x: x * x
tru_1 = lambda x: x - 1

ham = compose(tru_1, binh_phuong)
print(ham(4))
```

Dòng cuối in ra gì?

:::opt{correct}
`15`
:::

:::opt
`9` — vì `tru_1` được viết TRƯỚC trong `compose(tru_1, binh_phuong)`,
nên nó chạy TRƯỚC
::why
Gần đúng ở việc bạn để ý ĐÚNG thứ tự VIẾT trong lời gọi
`compose(tru_1, binh_phuong)` — `tru_1` quả thật đứng trước
`binh_phuong` khi ĐỌC từ trái sang phải.

Chỗ lệch: `compose` chạy PHẢI-SANG-TRÁI, không phải trái-sang-phải —
tham số THỨ HAI (`binh_phuong`, gần `x` nhất trong `f(g(x))`) chạy
TRƯỚC. `binh_phuong(4) = 16`, rồi `tru_1(16) = 15` — không phải
`tru_1(4) = 3` rồi `binh_phuong(3) = 9`.
::
:::

:::opt
Máy báo lỗi — `compose` chỉ ghép được các hàm CÙNG kiểu dữ liệu vào/ra,
`binh_phuong` (số → số) và `tru_1` (số → số) không tương thích theo
đúng thứ tự này
::why
Gần đúng ở việc bạn nghĩ tới việc kiểu dữ liệu vào/ra của hai hàm phải
KHỚP nhau để ghép được — quan sát đó có lý (bài 19-20 sẽ còn gặp lại ý
này khi ghép nhiều hàm).

Chỗ lệch: `binh_phuong` và `tru_1` CÙNG nhận và trả về số — hoàn toàn
tương thích theo BẤT KỲ thứ tự ghép nào ở đây, không có xung đột kiểu
dữ liệu. `compose(tru_1, binh_phuong)(4)` chạy được bình thường, ra
`15`.
::
:::
::::

::::code{#compose}
Tự viết `compose(f, g)` — ghép hai hàm, chạy `g` TRƯỚC rồi `f` SAU.

```python title=starter
def compose(f, g):
    ___

nhan_doi = lambda x: x * 2
ham = compose(str, nhan_doi)
print(ham(4))
```

```python title=solution
def compose(f, g):
    return lambda x: f(g(x))

nhan_doi = lambda x: x * 2
ham = compose(str, nhan_doi)
print(ham(4))
```

```python title=test
assert ham(4) == "8", "phải chạy g TRƯỚC (nhân đôi 4=8), rồi f SAU (str(8)='8')"
assert isinstance(ham(4), str), "kết quả phải là str, không phải int"
cong_1 = lambda x: x + 1
ham2 = compose(nhan_doi, cong_1)
assert ham2(3) == 8, "chạy cong_1 trước (3+1=4), rồi nhan_doi sau (4*2=8)"
assert ham2(3) != 7, "không được chạy nhan_doi trước rồi cong_1 sau — thứ tự sai sẽ ra 3*2+1=7"
```

:::hints
- kind: attention
  body: "compose(f, g) phải trả về một hàm nhận x, gọi g(x) TRƯỚC, rồi đưa kết quả đó vào f — không phải gọi f(x) trước."
- kind: strategy
  body: 'return lambda x: f(g(x)) — g nằm SÁT x nhất, nên g chạy trước; f bọc bên ngoài kết quả của g, nên f chạy sau.'
- kind: one-line
  body: "return lambda x: f(g(x))"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: compose phải dùng CẢ HAI tham số f và g — trả về một closure gọi lồng f(g(x)), không được bỏ qua một trong hai hàm.
  requireAst:
  - kind: uses-name, target: f, min: 1
  - kind: uses-name, target: g, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "8"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`compose(f, g)` ghép hai hàm thành một — chạy `g` trước, `f` sau. Đọc
ngược thứ tự viết, đúng bản chất `f(g(x))`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nếu ghép BA hàm bằng cách lồng `compose` vào nhau —
`compose(a, compose(b, c))` — thứ tự chạy THẬT SỰ là gì? Đọc từ trái
sang phải có còn đúng không?

Bài sau đo chính xác điều đó.
::::

::::checkpoint{mastery=0.8}
::::
