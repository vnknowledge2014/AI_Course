---
id: toan.do-thi-modular-dai-so-truu-tuong.tong-bac-la-hai-lan-so-canh
title: Tổng bậc là hai lần số cạnh
summary: "Bổ đề bắt tay: Σ deg(v) = 2|E| — mỗi cạnh có ĐÚNG hai đầu mút, đóng góp đúng 1 vào bậc của mỗi đầu; cộng dồn qua mọi cạnh ra 2 lần số cạnh."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.handshake-lemma]
requires: [math.vertex-degree]
concepts: [math.bo-de-bat-tay]
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
Cộng THẲNG bậc của MỌI luống lại. Con số ĐÓ có liên quan gì tới SỐ
ống tưới (số cạnh) không?
::::

::::explain{#bo-de-bat-tay}
Có — LIÊN QUAN CHẶT CHẼ. **Bổ đề bắt tay: `Σ deg(v) = 2|E|`** — MỖI
cạnh (T2.4: một CẶP đối xứng `(a,b)` VÀ `(b,a)`) có ĐÚNG hai ĐẦU MÚT,
nên nó ĐÓNG GÓP đúng `1` vào bậc của MỖI đầu — cộng dồn qua MỌI cạnh
ra ĐÚNG số phần tử của `E` (T2.4: `E` LƯU cả hai chiều):

```python title=readonly
def bac(v, E):
    return sum(1 for (a, b) in E if a == v)

def tong_bac(luong, E):
    return sum(bac(v, E) for v in luong)


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(tong_bac(luong, E))
print(len(E))
```

```text title=readonly
4
4
```

`deg(luong_1)+deg(luong_2)+deg(luong_3)+deg(luong_4) = 1+2+1+0 = 4`
— khớp ĐÚNG `len(E)=4` (E lưu bốn cặp CÓ hướng, ứng ĐÚNG hai cạnh vô
hướng thật: `luong_1-luong_2`, `luong_2-luong_3`).
::::

::::example{#chia-doi-ra-so-canh-that}
`len(E)` (bốn cặp CÓ hướng) chia đôi RA ĐÚNG số cạnh vô hướng THẬT:

```python title=readonly
def bac(v, E):
    return sum(1 for (a, b) in E if a == v)

def tong_bac(luong, E):
    return sum(bac(v, E) for v in luong)


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

so_canh_that = len(E) // 2
print(so_canh_that)
print(tong_bac(luong, E) == 2 * so_canh_that)
```

```text title=readonly
2
True
```

HAI cạnh THẬT (`luong_1-luong_2`, `luong_2-luong_3`), MỖI cạnh LƯU
`2` cặp CÓ hướng trong `E` (đối xứng, T2.4 bài 20) — `len(E)=2×2=4`.
Tổng bậc `4` khớp ĐÚNG `2 × 2`.
::::

::::predict{#doan-tong-bac-luon-chan commitOnce}
Byte thêm MỘT cạnh mới — `luong_3-luong_4` — rồi kiểm TỔNG bậc:

```python
def bac(v, E):
    return sum(1 for (a, b) in E if a == v)

def tong_bac(luong, E):
    return sum(bac(v, E) for v in luong)

luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"),
     ("luong_3", "luong_4"), ("luong_4", "luong_3")}
print(tong_bac(luong, E) % 2)
```

Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
`1` — vì THÊM một cạnh MỚI làm tổng bậc TĂNG lên một số LẺ đơn vị,
đẩy tổng TỪ chẵn SANG lẻ
::why
Gần đúng ở việc bạn nghĩ "thêm một CẠNH thì tổng thay đổi một LƯỢNG
nhỏ, lẻ" — một trực giác dễ hiểu nhầm VỀ mức độ thay đổi.

Chỗ lệch: MỖI cạnh MỚI LUÔN cộng thêm ĐÚNG `2` vào tổng bậc (`1` cho
MỖI đầu mút, bổ đề bắt tay) — KHÔNG BAO GIỜ cộng thêm một số LẺ.
Tổng bậc BẮT ĐẦU chẵn (`0`, đồ thị rỗng) VÀ CHỈ CỘNG THÊM số chẵn
(`2` mỗi cạnh) — LUÔN LUÔN chẵn, KHÔNG có cách nào LÀM nó lẻ.
::
:::

:::opt
Máy báo lỗi khi chạy — thêm cạnh MỚI khiến `luong_4` (trước đó CÔ
LẬP, bậc `0`) đổi bậc GIỮA chừng, VÀ `tong_bac` không tính LẠI kịp
::why
Gần đúng ở việc bạn để ý `luong_4` TỪNG cô lập (bậc `0`) VÀ giờ CÓ
cạnh — một quan sát đúng VỀ SỰ thay đổi trạng thái.

Chỗ lệch: `tong_bac` KHÔNG "lưu" kết quả CŨ Ở ĐÂU cả — MỖI LẦN gọi,
nó TÍNH LẠI TỪ ĐẦU trên `E` HIỆN TẠI (đưa VÀO lúc gọi). KHÔNG có gì
"chưa kịp" cả; hàm luôn phản ánh ĐÚNG dữ liệu MỚI NHẤT được truyền
vào.
::
:::
::::

::::code{#viet_tong_bac}
Viết `tong_bac(luong, E)` — cộng dồn `bac(v, E)` qua MỌI đỉnh trong
`luong`.

```python title=starter
def bac(v, E):
    return sum(1 for (a, b) in E if a == v)

def tong_bac(luong, E):
    return ___


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(tong_bac(luong, E))
```

```python title=solution
def bac(v, E):
    return sum(1 for (a, b) in E if a == v)

def tong_bac(luong, E):
    return sum(bac(v, E) for v in luong)


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(tong_bac(luong, E))
```

```python title=test
luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}
assert tong_bac(set(), set()) == 0, "khong dinh nao -- tong bac 0"
assert tong_bac(luong, set()) == 0, "khong canh nao -- tong bac 0 du co dinh"
assert tong_bac(luong, E) == len(E), "tong bac phai khop len(E)"
assert tong_bac(luong, E) % 2 == 0, "tong bac luon chan -- bo de bat tay"
```

:::hints
- kind: attention
  body: "Dung sum() voi generator, cong don bac(v, E) cho tung dinh v trong luong."
- kind: strategy
  body: "sum(bac(v, E) for v in luong)"
- kind: one-line
  body: "___ = sum(bac(v, E) for v in luong)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung sum() voi generator, cong don bac(v, E) cho tung dinh
  requireAst:
  - kind: uses-call, target: sum, min: 2
  - kind: uses-call, target: bac, min: 1
  - kind: comprehension, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^4\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tổng bậc luôn chẵn — một quy luật KHÔNG THỂ vi phạm. Có đường đi
NỐI liền cả vườn không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Tổng bậc CẢ vườn LUÔN LÀ số CHẴN (`2×` một số nguyên). Có đồ thị nào
mà TỔNG bậc LẺ không — hay đây LÀ một quy luật KHÔNG THỂ vi phạm?
::::

::::checkpoint{mastery=0.8}
::::
