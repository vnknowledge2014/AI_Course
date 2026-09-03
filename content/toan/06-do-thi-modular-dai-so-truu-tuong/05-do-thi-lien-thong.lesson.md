---
id: toan.do-thi-modular-dai-so-truu-tuong.do-thi-lien-thong
title: Đồ thị liên thông
summary: "Liên thông — CÓ đường đi giữa MỌI cặp đỉnh; một sơ đồ tưới liên thông nghĩa LÀ nước CÓ thể chảy (qua nhiều ống nối tiếp) TỚI bất kỳ luống nào TỪ bất kỳ luống nào khác."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.connected-graph]
requires: [math.path-and-cycle]
concepts: [math.lien-thong]
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
Từ luống 1, có đường đi TỚI luống 4 không (qua luống 2 hoặc 3)? Cả
VƯỜN có "nối liền" với nhau không?
::::

::::explain{#lien-thong-la-gi}
**Liên thông** — CÓ đường đi giữa MỌI cặp đỉnh. Một sơ đồ tưới liên
thông nghĩa LÀ nước CÓ thể chảy (qua nhiều ống NỐI TIẾP) TỚI bất kỳ
luống nào TỪ bất kỳ luống nào khác. Kiểm bằng cách tính TẬP đỉnh
"TỚI ĐƯỢC" từ một đỉnh (LẶP tới khi KHÔNG thêm được đỉnh nào nữa —
đúng lối "lặp tới khi ổn định" đã quen), rồi so với TOÀN bộ `luong`:

```python title=readonly
def dinh_toi_duoc(v, E):
    da_tham = {v}
    while True:
        moi = da_tham | {b for (a, b) in E if a in da_tham}
        if moi == da_tham:
            return da_tham
        da_tham = moi


E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(sorted(dinh_toi_duoc("luong_1", E)))
```

```text title=readonly
['luong_1', 'luong_2', 'luong_3']
```

TỪ `luong_1`: bước đầu THÊM `luong_2` (cạnh trực tiếp), bước SAU
THÊM `luong_3` (qua `luong_2`) — LẶP tới khi thêm KHÔNG được nữa
(`moi == da_tham`). Tập tới được LÀ `{luong_1,luong_2,luong_3}`.
::::

::::example{#khong-lien-thong}
`luong_4` KHÔNG có trong tập tới được TỪ `luong_1` — đồ thị KHÔNG
liên thông:

```python title=readonly
def dinh_toi_duoc(v, E):
    da_tham = {v}
    while True:
        moi = da_tham | {b for (a, b) in E if a in da_tham}
        if moi == da_tham:
            return da_tham
        da_tham = moi

def la_lien_thong(luong, E):
    if not luong:
        return True
    v = next(iter(luong))
    return dinh_toi_duoc(v, E) == luong


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(la_lien_thong(luong, E))
```

```text title=readonly
False
```

Tập tới được TỪ `luong_1` (`{luong_1,luong_2,luong_3}`) KHÁC `luong`
(có CẢ `luong_4`) — `la_lien_thong` trả VỀ `False`. `luong_4` bị CÔ
LẬP, không ống nào tới nó.
::::

::::predict{#doan-lien-thong-do-thi-rong commitOnce}
Byte kiểm liên thông TRÊN một đồ thị KHÔNG có đỉnh nào cả (`luong =
set()`):

```python
def dinh_toi_duoc(v, E):
    da_tham = {v}
    while True:
        moi = da_tham | {b for (a, b) in E if a in da_tham}
        if moi == da_tham:
            return da_tham
        da_tham = moi

def la_lien_thong(luong, E):
    if not luong:
        return True
    v = next(iter(luong))
    return dinh_toi_duoc(v, E) == luong

print(la_lien_thong(set(), set()))
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — vì "liên thông" đòi CÓ đường đi GIỮA mọi cặp đỉnh, mà một
đồ thị KHÔNG đỉnh nào thì KHÔNG hề có "cặp" nào để KIỂM, nên điều
kiện đó KHÔNG THỂ được thoả
::why
Gần đúng ở việc bạn nghĩ "không có gì để kiểm" nghe như "chưa xác
định được" — một trực giác dễ hiểu nhầm với chân lý rỗng.

Chỗ lệch: ĐÚNG như T2.3 bài 11 đã dạy (chân lý rỗng) — một điều kiện
phát biểu TRÊN "MỌI cặp" mà KHÔNG có cặp nào để VI PHẠM thì ĐƯƠNG
NHIÊN đúng, không phải "không xác định". Không đỉnh nào để mà THIẾU
đường đi — liên thông LUÔN đúng cho đồ thị RỖNG, giống hệt `all([])
is True` (T2.4 bài 19 đã dùng lại Ý này).
::
:::

:::opt
Máy báo lỗi khi chạy — `next(iter(luong))` cố lấy MỘT phần tử TỪ
`luong`, mà `luong` LÀ tập RỖNG thì KHÔNG có phần tử nào để lấy
::why
Gần đúng ở việc bạn để ý ĐÚNG `next(iter(set()))` (nếu GỌI trực
tiếp) SẼ báo lỗi (`StopIteration`) — một quan sát KỸ THUẬT chính
xác.

Chỗ lệch: dòng `if not luong: return True` chạy TRƯỚC — nó KIỂM
`luong` có RỖNG hay không VÀ trả VỀ NGAY khi rỗng, KHÔNG BAO GIỜ chạy
tới dòng `next(iter(luong))`. Đây LÀ MỘT ca ĐÃ được XỬ LÝ riêng,
không phải bị bỏ SÓT.
::
:::
::::

::::code{#viet_la_lien_thong}
Viết `la_lien_thong(luong, E)` — kiểm ĐỒ THỊ `(luong, E)` có liên
thông hay không.

```python title=starter
def dinh_toi_duoc(v, E):
    da_tham = {v}
    while True:
        moi = da_tham | {b for (a, b) in E if a in da_tham}
        if moi == da_tham:
            return da_tham
        da_tham = moi

def la_lien_thong(luong, E):
    if not luong:
        return True
    v = next(iter(luong))
    return ___


luong = {"luong_1", "luong_2", "luong_3"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(la_lien_thong(luong, E))
```

```python title=solution
def dinh_toi_duoc(v, E):
    da_tham = {v}
    while True:
        moi = da_tham | {b for (a, b) in E if a in da_tham}
        if moi == da_tham:
            return da_tham
        da_tham = moi

def la_lien_thong(luong, E):
    if not luong:
        return True
    v = next(iter(luong))
    return dinh_toi_duoc(v, E) == luong


luong = {"luong_1", "luong_2", "luong_3"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(la_lien_thong(luong, E))
```

```python title=test
assert la_lien_thong(set(), set()) is True, "do thi rong -- chan ly rong"
assert la_lien_thong({"x"}, set()) is True, "mot dinh, khong canh -- van lien thong"
luong4 = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}
assert la_lien_thong(luong4, E) is False, "luong_4 co lap -- khong lien thong"
E2 = E | {("luong_3", "luong_4"), ("luong_4", "luong_3")}
assert la_lien_thong(luong4, E2) is True, "noi them mot canh -- lien thong het"
```

:::hints
- kind: attention
  body: "So sanh dinh_toi_duoc(v, E) voi chinh luong -- lien thong khi va chi khi bang nhau."
- kind: strategy
  body: "dinh_toi_duoc(v, E) == luong"
- kind: one-line
  body: "___ = dinh_toi_duoc(v, E) == luong"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai so sanh dinh_toi_duoc(v, E) voi luong bang ==
  requireAst:
  - kind: uses-call, target: dinh_toi_duoc, min: 1
  - kind: uses-operator, target: '==', min: 1
  - kind: uses-name, target: luong, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khi KHÔNG liên thông, vườn chia thành mấy "cụm" tách biệt?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte có TÁM luống, NHƯNG hệ ống tưới chia thành HAI cụm TÁCH biệt
(không ống nào nối hai cụm). Đồ thị NÀY có liên thông không — và
nếu KHÔNG, "phần" nào của nó LÀ liên thông?
::::

::::checkpoint{mastery=0.8}
::::
