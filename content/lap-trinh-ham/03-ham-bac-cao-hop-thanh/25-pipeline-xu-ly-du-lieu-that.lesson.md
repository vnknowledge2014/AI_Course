---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.pipeline-xu-ly-du-lieu-that
title: "Một pipeline xử lý dữ liệu thật: chuẩn hoá → lọc → biến đổi → tổng hợp"
summary: "Xử lý một chuỗi văn bản qua pipe() — chuẩn hoá (.strip().lower()), tách từ (.split()), lọc từ ngắn, đếm số từ còn lại — mỗi bước một hàm THUẦN riêng, ghép bằng pipe. Ví dụ thực tế, mô phỏng đúng cách pipe/compose dùng trong mã thật."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.real-pipeline]
requires: [fp.review-composition]
concepts: [fp.real-pipeline]
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
Cụm 4 chốt lại mọi công cụ ghép hàm. Cụm này đưa chúng vào một tình
huống THẬT: dữ liệu lộn xộn, cần nhiều bước xử lý nối tiếp.
::::

::::explain{#pipeline-that}
```python
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def dem_tu_dai(vb):
    chuan_hoa = lambda s: s.strip().lower()
    tach_tu = lambda s: s.split()
    loc_tu_dai = lambda ds: [t for t in ds if len(t) > 2]
    dem = lambda ds: len(ds)
    return pipe(vb, chuan_hoa, tach_tu, loc_tu_dai, dem)

print(dem_tu_dai("  Toi La Mot Lap Trinh Vien FP  "))
```

```text
5
```

Bốn bước, một ĐƯỜNG DỮ LIỆU rõ ràng: `chuan_hoa` bỏ khoảng trắng thừa
và HOA/thường (`"toi la mot lap trinh vien fp"`), `tach_tu` cắt thành
danh sách từ, `loc_tu_dai` GIỮ những từ dài hơn 2 ký tự (bỏ `"la"`,
`"fp"`), `dem` đếm số từ còn lại (`5`: `toi, mot, lap, trinh, vien`).

Đây KHÔNG phải bài tập trừu tượng — mã THẬT xử lý văn bản (tìm kiếm,
phân tích log, lọc dữ liệu người dùng nhập) thường có ĐÚNG hình dạng
này: một chuỗi bước nhỏ, mỗi bước THUẦN (không side effect), ghép bằng
`pipe`. Không có bước nào "phức tạp" — cái khó nằm ở việc CHIA đúng
từng bước, không phải viết một khối logic khổng lồ.
::::

::::example{#tung-buoc-that}
Theo dõi dữ liệu BIẾN DẠNG qua từng bước:

```python title=readonly
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

vb = "  Ham La CONG DAN Hang Nhat  "

chuan_hoa = lambda s: s.strip().lower()
tach_tu = lambda s: s.split()
loc_tu_dai = lambda ds: [t for t in ds if len(t) > 2]

b1 = pipe(vb, chuan_hoa)
b2 = pipe(vb, chuan_hoa, tach_tu)
b3 = pipe(vb, chuan_hoa, tach_tu, loc_tu_dai)

print(repr(b1))
print(b2)
print(b3)
```

```text title=readonly
'ham la cong dan hang nhat'
['ham', 'la', 'cong', 'dan', 'hang', 'nhat']
['ham', 'cong', 'dan', 'hang', 'nhat']
```

`b1` vẫn là MỘT chuỗi (đã sạch). `b2` là một DANH SÁCH các từ. `b3` là
danh sách đó đã LỌC BỚT (`"la"` — 2 ký tự — bị loại). Kiểu dữ liệu ĐỔI
qua từng bước (chuỗi → danh sách → danh sách ngắn hơn) — `pipe` không
quan tâm kiểu dữ liệu là gì, chỉ đưa kết quả bước trước làm đầu vào
bước sau.
::::

::::predict{#doan-pipeline-du-lieu-that commitOnce}
```python
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

chuan_hoa = lambda s: s.strip().lower()
tach_tu = lambda s: s.split()
loc_tu_dai = lambda ds: [t for t in ds if len(t) > 2]
dem = lambda ds: len(ds)

print(pipe("  A Vai Con Cho  ", chuan_hoa, tach_tu, loc_tu_dai, dem))
```

Dòng cuối in ra gì?

:::opt{correct}
`2`
:::

:::opt
`4` — vì `loc_tu_dai` chỉ lọc CHỮ HOA, không lọc theo ĐỘ DÀI từ
::why
Gần đúng ở việc bạn tính đúng SỐ TỪ SAU khi tách (`a, vai, con, cho` —
bốn từ) — bước `tach_tu` đó đúng.

Chỗ lệch: `loc_tu_dai` lọc theo ĐỘ DÀI (`len(t) > 2`), không phải chữ
hoa — mà `chuan_hoa` đã hạ hết về chữ thường TRƯỚC ĐÓ rồi (không còn
chữ hoa nào để phân biệt). `"a"` (1 ký tự) và `"vai"`... khoan, `"vai"`
dài 3 ký tự nên GIỮ; chỉ `"a"` (1 ký tự) bị loại. Còn lại `vai, con,
cho` — ba từ, không phải bốn.
::
:::

:::opt
Máy báo lỗi — chuỗi `"  A Vai Con Cho  "` có khoảng trắng THỪA ở đầu
VÀ cuối, `pipe` không xử lý được đầu vào "bẩn" như vậy
::why
Gần đúng ở việc bạn để ý chuỗi đầu vào có khoảng trắng thừa — quan sát
đó đúng, có khoảng trắng thật ở cả hai đầu.

Chỗ lệch: ĐÓ CHÍNH LÀ lý do bước `chuan_hoa` (`.strip().lower()`) tồn
tại — nó XỬ LÝ ĐÚNG khoảng trắng thừa, không phải một trường hợp gây
lỗi. `pipeline` được thiết kế NGAY TỪ ĐẦU để nhận dữ liệu "bẩn" như
vậy, không cần đầu vào đã sạch sẵn.
::
:::
::::

::::code{#dem_tu_dai}
Viết `dem_tu_dai(vb)` — dùng `pipe()` ghép BỐN bước: chuẩn hoá (bỏ
khoảng trắng thừa, hạ chữ thường), tách từ, lọc GIỮ từ dài hơn 2 ký
tự, rồi đếm số từ còn lại.

```python title=starter
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def dem_tu_dai(vb):
    ___

print(dem_tu_dai("  Toi La Mot Lap Trinh Vien FP  "))
```

```python title=solution
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def dem_tu_dai(vb):
    chuan_hoa = lambda s: s.strip().lower()
    tach_tu = lambda s: s.split()
    loc_tu_dai = lambda ds: [t for t in ds if len(t) > 2]
    dem = lambda ds: len(ds)
    return pipe(vb, chuan_hoa, tach_tu, loc_tu_dai, dem)

print(dem_tu_dai("  Toi La Mot Lap Trinh Vien FP  "))
```

```python title=test
assert dem_tu_dai("  Toi La Mot Lap Trinh Vien FP  ") == 5, "phải chuẩn hoá, tách từ, lọc từ dài (>2 ký tự), đếm đúng"
assert dem_tu_dai("") == 0, "chuỗi rỗng phải ra 0"
assert dem_tu_dai("  a  ") == 0, "một từ quá ngắn (1 ký tự) phải bị lọc bỏ, còn lại 0"
assert dem_tu_dai("HOC LAP TRINH HAM RAT VUI") == 6, "phải không phân biệt HOA/thường sau khi chuẩn hoá, và đếm đúng số từ dài"
```

:::hints
- kind: attention
  body: "Bốn lambda riêng: chuẩn hoá (.strip().lower()), tách từ (.split()), lọc list comprehension (len(t) > 2), đếm (len()). Ghép cả bốn bằng pipe(vb, ...)."
- kind: strategy
  body: 'chuan_hoa/tach_tu/loc_tu_dai/dem — bốn hàm nhỏ, mỗi hàm MỘT việc, rồi return pipe(vb, chuan_hoa, tach_tu, loc_tu_dai, dem).'
- kind: one-line
  body: "chuan_hoa = lambda s: s.strip().lower()\ntach_tu = lambda s: s.split()\nloc_tu_dai = lambda ds: [t for t in ds if len(t) > 2]\ndem = lambda ds: len(ds)\nreturn pipe(vb, chuan_hoa, tach_tu, loc_tu_dai, dem)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: dem_tu_dai phải dùng pipe() để ghép các bước, và phải dùng strip()/split() (bài đang dạy đúng cách xử lý văn bản qua pipeline) — không viết gọn thành một biểu thức duy nhất bỏ qua pipe.
  requireAst:
  - kind: uses-call, target: pipe, min: 1
  - kind: uses-call, target: strip, min: 1
  - kind: uses-call, target: split, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "5"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Dữ liệu thật lộn xộn — nhưng chia thành từng bước nhỏ, THUẦN, ghép
bằng `pipe`, thì xử lý nó không khác gì ví dụ số nhỏ đã học.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Một pipeline bốn bước như thế này, nếu chạy SAI kết quả, làm sao biết
BƯỚC NÀO gây ra lỗi mà không phải tách nó ra viết lại từng phần?

Bài sau: một công cụ debug đúng cho tình huống này.
::::

::::checkpoint{mastery=0.8}
::::
