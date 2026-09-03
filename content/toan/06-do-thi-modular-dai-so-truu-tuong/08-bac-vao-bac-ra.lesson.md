---
id: toan.do-thi-modular-dai-so-truu-tuong.bac-vao-bac-ra
title: Bậc vào, bậc ra
summary: "Đồ thị CÓ HƯỚNG — cạnh LÀ cặp CÓ thứ tự (a,b) (T2.4 bài 14, ống MỘT chiều: nước chảy TỪ a TỚI b); bậc VÀO (số cạnh TRỎ vào) VÀ bậc RA (số cạnh TRỎ ra) — hai con số TÁCH biệt, khác bậc thường (bài 2, đồ thị VÔ hướng)."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.in-out-degree]
requires: [math.vertex-degree, math.ordered-pair]
concepts: [math.bac-vao, math.bac-ra, math.do-thi-co-huong]
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
Trạm bơm ĐẨY nước MỘT chiều tới bốn luống — KHÔNG luống nào bơm
ngược lại. "Bậc" (bài 2) CÒN đủ để mô tả điều NÀY không?
::::

::::explain{#do-thi-co-huong}
CHƯA đủ. **Đồ thị CÓ HƯỚNG** — cạnh LÀ một cặp CÓ thứ tự `(a,b)`
(T2.4 bài 14, KHÁC cặp KHÔNG thứ tự), ống MỘT chiều: nước chảy TỪ
`a` TỚI `b`, KHÔNG cần `(b,a)` cũng có mặt (khác định nghĩa đồ thị
VÔ hướng của track, bài 1). Vì hướng đi QUAN TRỌNG, MỘT con số "bậc"
không còn đủ — cần TÁCH thành **bậc VÀO** (số cạnh TRỎ vào `v`) VÀ
**bậc RA** (số cạnh TRỎ ra TỪ `v`):

```python title=readonly
def bac_ra(v, E):
    return sum(1 for (a, b) in E if a == v)

def bac_vao(v, E):
    return sum(1 for (a, b) in E if b == v)


tram_bom = "tram_bom"
E = {(tram_bom, "luong_1"), (tram_bom, "luong_2"), (tram_bom, "luong_3"), (tram_bom, "luong_4")}

print(bac_ra(tram_bom, E), bac_vao(tram_bom, E))
```

```text title=readonly
4 0
```

Trạm bơm ĐẨY nước TỚI bốn luống — bậc RA LÀ `4` (bốn cạnh XUẤT phát
TỪ nó), bậc VÀO LÀ `0` (KHÔNG cạnh nào trỏ VÀO nó). Hai con số TÁCH
biệt hẳn — khác đồ thị vô hướng (bài 2), nơi CHỈ có MỘT bậc chung.
::::

::::example{#bac-cua-mot-luong}
MỘT luống nhận nước — bậc VÀO khác HẲN bậc RA:

```python title=readonly
def bac_ra(v, E):
    return sum(1 for (a, b) in E if a == v)

def bac_vao(v, E):
    return sum(1 for (a, b) in E if b == v)


tram_bom = "tram_bom"
E = {(tram_bom, "luong_1"), (tram_bom, "luong_2"), (tram_bom, "luong_3"), (tram_bom, "luong_4")}

print(bac_vao("luong_1", E), bac_ra("luong_1", E))
```

```text title=readonly
1 0
```

`luong_1` NHẬN đúng MỘT cạnh (TỪ trạm bơm) — bậc VÀO LÀ `1`; nó
KHÔNG đẩy nước tới đâu — bậc RA LÀ `0`.
::::

::::predict{#doan-tong-bac-vao-ra commitOnce}
Byte cộng TỔNG bậc VÀO của MỌI đỉnh, RỒI cộng TỔNG bậc RA của MỌI
đỉnh, RỒI so hai tổng ĐÓ VỚI số cạnh:

```python
def bac_ra(v, E):
    return sum(1 for (a, b) in E if a == v)

def bac_vao(v, E):
    return sum(1 for (a, b) in E if b == v)

tram_bom = "tram_bom"
E = {(tram_bom, "luong_1"), (tram_bom, "luong_2"), (tram_bom, "luong_3"), (tram_bom, "luong_4")}
dinh = {tram_bom, "luong_1", "luong_2", "luong_3", "luong_4"}

tong_vao = sum(bac_vao(v, E) for v in dinh)
tong_ra = sum(bac_ra(v, E) for v in dinh)
print(tong_vao == tong_ra == len(E))
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — số liệu TRÊN sơ đồ NÀY tình cờ khớp, NHƯNG "tổng bậc vào =
tổng bậc ra = số cạnh" KHÔNG PHẢI quy luật CHUNG, chỉ đúng khi đồ thị
CÓ hình dạng đặc biệt (như trạm bơm toả TIA)
::why
Gần đúng ở việc bạn NGHI NGỜ một khẳng định "LUÔN đúng" — một thói
quen tốt (T2.3: kiểm hữu hạn KHÔNG phải chứng minh).

Chỗ lệch: đây LÀ quy luật CHUNG, không phải trùng hợp — MỖI cạnh có
CÓ ĐÚNG một đầu RA (đóng góp `1` vào bậc RA của MỘT đỉnh) VÀ ĐÚNG một
đầu VÀO (đóng góp `1` vào bậc VÀO của MỘT đỉnh KHÁC hoặc CHÍNH nó).
Cộng dồn qua MỌI cạnh: tổng bậc RA = tổng bậc VÀO = số cạnh, LUÔN
đúng VỚI BẤT KỲ đồ thị có hướng nào — đúng LỐI bổ đề bắt tay (bài 3)
áp cho đồ thị có hướng.
::
:::

:::opt
Máy báo lỗi khi chạy — `tong_vao == tong_ra == len(E)` xâu chuỗi BA
giá trị VÀO một phép so sánh, Python CHỈ cho phép so sánh HAI giá trị
MỘT lúc
::why
Gần đúng ở việc bạn để ý biểu thức CÓ tới BA giá trị nối bằng `==`
— một quan sát VỀ hình thức đúng.

Chỗ lệch: Python CHO PHÉP xâu chuỗi so sánh (`a == b == c` nghĩa LÀ
`a == b AND b == c`, ĐÃ dùng dạng NÀY từ T2.3) — KHÔNG phải lỗi cú
pháp. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_bac_vao}
Viết `bac_vao(v, E)` — đếm số cạnh CÓ HƯỚNG trỏ VÀO đỉnh `v`.

```python title=starter
def bac_ra(v, E):
    return sum(1 for (a, b) in E if a == v)

def bac_vao(v, E):
    return ___


tram_bom = "tram_bom"
E = {(tram_bom, "luong_1"), (tram_bom, "luong_2"), (tram_bom, "luong_3"), (tram_bom, "luong_4")}

print(bac_vao("luong_1", E))
```

```python title=solution
def bac_ra(v, E):
    return sum(1 for (a, b) in E if a == v)

def bac_vao(v, E):
    return sum(1 for (a, b) in E if b == v)


tram_bom = "tram_bom"
E = {(tram_bom, "luong_1"), (tram_bom, "luong_2"), (tram_bom, "luong_3"), (tram_bom, "luong_4")}

print(bac_vao("luong_1", E))
```

```python title=test
tram_bom = "tram_bom"
E = {(tram_bom, "luong_1"), (tram_bom, "luong_2"), (tram_bom, "luong_3"), (tram_bom, "luong_4")}
assert bac_vao(tram_bom, E) == 0, "tram bom khong nhan canh nao"
assert bac_ra(tram_bom, E) == 4, "tram bom day nuoc toi bon luong"
assert bac_vao("luong_1", E) == 1, "luong_1 nhan dung mot canh"
assert bac_vao("luong_khong_ton_tai", E) == 0, "dinh khong co canh nao tro vao -- bac vao 0"
dinh = {tram_bom, "luong_1", "luong_2", "luong_3", "luong_4"}
assert sum(bac_vao(v, E) for v in dinh) == len(E), "tong bac vao phai bang so canh"
```

:::hints
- kind: attention
  body: "Dem cap (a, b) trong E ma DAU b (khong phai a) bang v -- day la chieu 'tro vao'."
- kind: strategy
  body: "sum(1 for (a, b) in E if b == v)"
- kind: one-line
  body: "___ = sum(1 for (a, b) in E if b == v)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dem cap (a,b) trong E ma DAU b (dau vao) bang v, dung sum va comprehension
  requireAst:
  - kind: uses-name, target: b, min: 1
  - kind: uses-operator, target: '==', min: 2
  - kind: comprehension, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^1\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bậc vào, bậc ra tách bạch trên đồ thị có hướng. Bảy cây cầu nối bốn
vùng đất — có cách nào đi qua HẾT, MỖI cầu ĐÚNG một lần, không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bảy cây cầu nối bốn vùng đất Ở Königsberg — MỖI cầu LÀ một cạnh, MỖI
vùng đất LÀ một đỉnh. Người dân MUỐN đi qua ĐỦ cả bảy cầu, MỖI cầu
ĐÚNG một lần, RỒI quay VỀ điểm xuất phát. CÓ làm được không?
::::

::::checkpoint{mastery=0.8}
::::
