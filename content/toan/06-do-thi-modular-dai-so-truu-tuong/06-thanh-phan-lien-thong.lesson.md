---
id: toan.do-thi-modular-dai-so-truu-tuong.thanh-phan-lien-thong
title: Thành phần liên thông
summary: "Thành phần liên thông — một NHÓM đỉnh TỐI ĐA mà MỌI cặp trong nhóm CÓ đường đi nối nhau; các thành phần RỜI NHAU (T2.4 bài 10) và gộp lại vừa khít TOÀN đồ thị — đúng cấu trúc phân hoạch (T2.4 bài 23)."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.connected-component]
requires: [math.connected-graph]
concepts: [math.thanh-phan-lien-thong]
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
Vườn Byte KHÔNG liên thông — luống 4 bị cô lập. Vậy vườn chia thành
MẤY "cụm" tách biệt?
::::

::::explain{#thanh-phan-lien-thong-la-gi}
**Thành phần liên thông** — một NHÓM đỉnh TỐI ĐA mà MỌI cặp trong
nhóm CÓ đường đi nối nhau; MỘT đồ thị KHÔNG liên thông chia thành
NHIỀU thành phần, các thành phần RỜI NHAU (T2.4 bài 10) VÀ gộp lại
vừa khít TOÀN đồ thị — đúng cấu trúc PHÂN HOẠCH (T2.4 bài 23):

```python title=readonly
def dinh_toi_duoc(v, E):
    da_tham = {v}
    while True:
        moi = da_tham | {b for (a, b) in E if a in da_tham}
        if moi == da_tham:
            return da_tham
        da_tham = moi

def thanh_phan_lien_thong(luong, E):
    con_lai = set(luong)
    tp = []
    while con_lai:
        v = next(iter(con_lai))
        nhom = dinh_toi_duoc(v, E)
        tp.append(nhom)
        con_lai = con_lai - nhom
    return tp


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(len(thanh_phan_lien_thong(luong, E)))
```

```text title=readonly
2
```

Hai thành phần: `{luong_1,luong_2,luong_3}` (liên thông VỚI nhau) VÀ
`{luong_4}` (đứng RIÊNG). Vòng lặp LẤY một đỉnh CÒN LẠI, tính tập
tới được (bài 5), GHI làm MỘT thành phần, rồi BỚT nhóm đó khỏi `con_lai`
— LẶP tới khi `con_lai` RỖNG.
::::

::::example{#khong-canh-nao}
KHÔNG cạnh nào — MỖI đỉnh LÀ thành phần RIÊNG của CHÍNH nó:

```python title=readonly
def dinh_toi_duoc(v, E):
    da_tham = {v}
    while True:
        moi = da_tham | {b for (a, b) in E if a in da_tham}
        if moi == da_tham:
            return da_tham
        da_tham = moi

def thanh_phan_lien_thong(luong, E):
    con_lai = set(luong)
    tp = []
    while con_lai:
        v = next(iter(con_lai))
        nhom = dinh_toi_duoc(v, E)
        tp.append(nhom)
        con_lai = con_lai - nhom
    return tp


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}

print(len(thanh_phan_lien_thong(luong, set())))
```

```text title=readonly
4
```

KHÔNG ống tưới nào — BỐN luống, BỐN thành phần TÁCH biệt (MỖI luống
CHỈ tới được CHÍNH nó, `dinh_toi_duoc(v, set()) = {v}`).
::::

::::predict{#doan-noi-them-mot-ong commitOnce}
Byte NỐI thêm ĐÚNG một ống GIỮA `luong_3` VÀ `luong_4` (nối hai thành
phần LẠI):

```python
def dinh_toi_duoc(v, E):
    da_tham = {v}
    while True:
        moi = da_tham | {b for (a, b) in E if a in da_tham}
        if moi == da_tham:
            return da_tham
        da_tham = moi

def thanh_phan_lien_thong(luong, E):
    con_lai = set(luong)
    tp = []
    while con_lai:
        v = next(iter(con_lai))
        nhom = dinh_toi_duoc(v, E)
        tp.append(nhom)
        con_lai = con_lai - nhom
    return tp

luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}
E2 = E | {("luong_3", "luong_4"), ("luong_4", "luong_3")}
print(len(thanh_phan_lien_thong(luong, E2)))
```

Dòng cuối in ra gì?

:::opt{correct}
`1`
:::

:::opt
`3` — vì trước đó CÓ hai thành phần (`{luong_1,luong_2,luong_3}` VÀ
`{luong_4}`), THÊM một cạnh MỚI khiến `luong_4` TÁCH thành một thành
phần THỨ BA riêng biệt, cộng dồn LÊN ba
::why
Gần đúng ở việc bạn nhớ ĐÚNG TRƯỚC đó CÓ hai thành phần — quan sát
NỀN tảng đó đúng.

Chỗ lệch: NỐI thêm một cạnh KHÔNG hề "tách thêm" thành phần — NÓ
GỘP hai thành phần CŨ (`{luong_1,luong_2,luong_3}` VÀ `{luong_4}`)
LẠI thành MỘT (vì giờ CÓ đường đi TỪ `luong_1` TỚI `luong_4`, QUA
`luong_3`). Số thành phần GIẢM (từ `2` xuống `1`), không TĂNG.
::
:::

:::opt
Máy báo lỗi khi chạy — `E2` giờ CÓ một cạnh nối `luong_3` VỚI
`luong_4`, NHƯNG `luong` (tập đỉnh) VẪN giữ nguyên, gây MÂU THUẪN
giữa số đỉnh VÀ số cạnh
::why
Gần đúng ở việc bạn để ý `luong` KHÔNG đổi trong khi `E2` đổi — một
quan sát VỀ SỰ khác biệt giữa hai biến.

Chỗ lệch: KHÔNG có "mâu thuẫn" nào cả — `luong` (tập ĐỈNH) VÀ `E`
(tập CẠNH) LÀ hai thứ TÁCH biệt, `E` HOÀN TOÀN có thể thay đổi (thêm
cạnh MỚI) mà KHÔNG cần đổi `luong`, MIỄN hai đỉnh của cạnh MỚI đã CÓ
sẵn trong `luong` (ĐÚNG Ở đây: `luong_3`, `luong_4` đều Ở TRONG
`luong`). Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_thanh_phan_lien_thong}
Viết `thanh_phan_lien_thong(luong, E)` — trả về danh sách CÁC thành
phần liên thông của đồ thị `(luong, E)`.

```python title=starter
def dinh_toi_duoc(v, E):
    da_tham = {v}
    while True:
        moi = da_tham | {b for (a, b) in E if a in da_tham}
        if moi == da_tham:
            return da_tham
        da_tham = moi

def thanh_phan_lien_thong(luong, E):
    con_lai = set(luong)
    tp = []
    while con_lai:
        v = next(iter(con_lai))
        nhom = ___
        tp.append(nhom)
        con_lai = con_lai - nhom
    return tp


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(len(thanh_phan_lien_thong(luong, E)))
```

```python title=solution
def dinh_toi_duoc(v, E):
    da_tham = {v}
    while True:
        moi = da_tham | {b for (a, b) in E if a in da_tham}
        if moi == da_tham:
            return da_tham
        da_tham = moi

def thanh_phan_lien_thong(luong, E):
    con_lai = set(luong)
    tp = []
    while con_lai:
        v = next(iter(con_lai))
        nhom = dinh_toi_duoc(v, E)
        tp.append(nhom)
        con_lai = con_lai - nhom
    return tp


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(len(thanh_phan_lien_thong(luong, E)))
```

```python title=test
assert thanh_phan_lien_thong(set(), set()) == [], "khong dinh nao -- khong thanh phan nao"
assert len(thanh_phan_lien_thong({"x"}, set())) == 1, "mot dinh -- dung mot thanh phan"
luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
assert len(thanh_phan_lien_thong(luong, set())) == 4, "khong canh nao -- moi dinh mot thanh phan rieng"
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}
assert len(thanh_phan_lien_thong(luong, E)) == 2, "phai khop vi du chinh"
```

:::hints
- kind: attention
  body: "Goi dinh_toi_duoc(v, E) de tinh tap dinh toi duoc tu v -- do la mot thanh phan."
- kind: strategy
  body: "dinh_toi_duoc(v, E)"
- kind: one-line
  body: "nhom = dinh_toi_duoc(v, E)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi dinh_toi_duoc(v, E) de tinh thanh phan chua v
  requireAst:
  - kind: uses-call, target: dinh_toi_duoc, min: 1
  - kind: uses-name, target: v, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^2\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đồ thị chia thành các thành phần. Có thành phần nào KHÔNG hề có chu
trình không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hệ ống tưới của Byte CÓ chu trình — luống 1→2→3→1, nước CÓ thể chảy
VÒNG. Bớt đúng MỘT ống trong vòng đó — đồ thị còn liên thông không,
và còn chu trình nào không?
::::

::::checkpoint{mastery=0.8}
::::
