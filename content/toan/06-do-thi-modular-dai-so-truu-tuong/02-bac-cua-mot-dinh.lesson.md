---
id: toan.do-thi-modular-dai-so-truu-tuong.bac-cua-mot-dinh
title: Bậc của một đỉnh
summary: "Bậc deg(v) — số cạnh KỀ một đỉnh (số ống tưới NỐI tới một luống); đếm bằng sum(1 for e in E if v in e đầu) — quy tắc đếm (T2.5 bài 1) áp lên đồ thị."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.vertex-degree]
requires: [math.graph-as-symmetric-relation]
concepts: [math.bac-dinh]
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
Luống 2 nối với luống 1 VÀ luống 3 — hai ống tưới chụm vào NÓ. Số
NÀY có tên riêng không?
::::

::::explain{#bac-la-gi}
Có. **Bậc `deg(v)`** — số cạnh KỀ một đỉnh (số ống tưới NỐI tới một
luống); đếm bằng quy tắc đếm (T2.5 bài 1) áp LÊN đồ thị — đếm SỐ cặp
`(v, u) ∈ E` với `v` cố định:

```python title=readonly
def bac(v, E):
    return sum(1 for (a, b) in E if a == v)


E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(bac("luong_2", E))
```

```text title=readonly
2
```

`E` có HAI cặp bắt đầu BẰNG `luong_2`: `(luong_2,luong_1)` VÀ
`(luong_2,luong_3)` — `deg(luong_2)=2`, khớp ĐÚNG "luống 2 nối VỚI
hai luống khác".
::::

::::example{#dinh-co-lap}
Một đỉnh KHÔNG nối VỚI ai — bậc LÀ `0`, KHÔNG lỗi:

```python title=readonly
def bac(v, E):
    return sum(1 for (a, b) in E if a == v)


E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(bac("luong_4", E))
```

```text title=readonly
0
```

`luong_4` KHÔNG xuất hiện Ở VỊ TRÍ đầu của BẤT KỲ cặp nào trong `E`
— `sum(1 for ... if a == "luong_4")` cộng dồn TRÊN một dãy RỖNG, ra
`0`. Một đỉnh CÔ LẬP (không ống nào tới) VẪN LÀ một đỉnh hợp lệ.
::::

::::predict{#doan-bac-tu-canh-doi-xung commitOnce}
Byte tính bậc của `luong_1` — chỉ nối VỚI ĐÚNG một luống khác:

```python
def bac(v, E):
    return sum(1 for (a, b) in E if a == v)

E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}
print(bac("luong_1", E))
```

Dòng cuối in ra gì?

:::opt{correct}
`1`
:::

:::opt
`2` — vì `E` chứa CẢ `(luong_1,luong_2)` LẪN `(luong_2,luong_1)`
(đối xứng, T2.4 bài 20) — CẢ hai cặp ĐỀU "liên quan" tới `luong_1`,
nên bậc phải ĐẾM cả hai
::why
Gần đúng ở việc bạn để ý CẢ HAI cặp `(luong_1,luong_2)` VÀ
`(luong_2,luong_1)` ĐỀU nhắc TỚI `luong_1` — một quan sát đúng VỀ dữ
liệu.

Chỗ lệch: `bac(v, E)` CHỈ đếm cặp có `luong_1` Ở VỊ TRÍ ĐẦU (biến
`a`) — `(luong_2, luong_1)` có `luong_1` Ở VỊ TRÍ SAU (biến `b`),
KHÔNG được đếm. Đây LÀ đếm ĐÚNG một lần cho MỖI cạnh vô hướng thật
(dù `E` lưu CẢ hai chiều để giữ đối xứng) — nếu đếm CẢ hai vị trí,
mọi bậc sẽ bị NHÂN ĐÔI sai.
::
:::

:::opt
Máy báo lỗi khi chạy — `sum(1 for (a, b) in E if a == v)` cần `E`
được SẮP XẾP theo thứ tự trước khi đếm, một `set` không có thứ tự
(T2.4 bài 1) thì không lặp được
::why
Gần đúng ở việc bạn nhớ ĐÚNG `set` KHÔNG có thứ tự (T2.4 bài 1) —
một sự thật đã học đúng.

Chỗ lệch: KHÔNG có thứ tự KHÔNG nghĩa là KHÔNG lặp được — `for (a,b)
in E` lặp qua MỌI phần tử của `E`, chỉ LÀ thứ tự duyệt KHÔNG cố định
giữa các lần chạy (T2.4 bài 1). Tổng `sum(...)` KHÔNG phụ thuộc thứ
tự cộng (T2.1 bài 20, giao hoán) nên kết quả VẪN đúng VÀ ổn định.
::
:::
::::

::::code{#viet_bac}
Viết `bac(v, E)` — đếm số cạnh trong `E` có `v` Ở VỊ TRÍ đầu.

```python title=starter
def bac(v, E):
    return ___


E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(bac("luong_2", E))
```

```python title=solution
def bac(v, E):
    return sum(1 for (a, b) in E if a == v)


E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(bac("luong_2", E))
```

```python title=test
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}
assert bac("luong_1", E) == 1, "chi noi voi mot luong"
assert bac("luong_3", E) == 1, "chi noi voi mot luong"
assert bac("luong_4", E) == 0, "dinh co lap -- bac 0"
assert bac("luong_2", E) == 2, "phai khop vi du chinh"
assert bac("x", set()) == 0, "do thi rong -- bac cua bat ky dinh nao deu 0"
```

:::hints
- kind: attention
  body: "Dung sum() voi generator, dem cac cap (a, b) trong E co a == v."
- kind: strategy
  body: "sum(1 for (a, b) in E if a == v)"
- kind: one-line
  body: "___ = sum(1 for (a, b) in E if a == v)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung sum() voi generator, dem cac cap (a, b) trong E co a == v
  requireAst:
  - kind: uses-call, target: sum, min: 1
  - kind: uses-operator, target: '==', min: 1
  - kind: comprehension, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^2\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bậc từng luống xong. Cộng thẳng bậc CẢ vườn lại — con số đó liên
quan gì tới số ống tưới?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Luống 2 nối với luống 1 VÀ luống 3 — `deg(luong_2)=2`. Cộng THẲNG
bậc của MỌI luống lại (`luong_1` tới `luong_4`) — con số ĐÓ có liên
quan gì tới SỐ cạnh (số ống tưới) không?
::::

::::checkpoint{mastery=0.8}
::::
