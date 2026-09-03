---
id: toan.do-thi-modular-dai-so-truu-tuong.cay-la-do-thi-khong-chu-trinh
title: Cây là đồ thị liên thông không chu trình
summary: "Cây (lý thuyết đồ thị) — đồ thị liên thông (bài 5) VÀ CÓ ĐÚNG n−1 cạnh (n = số đỉnh) — hệ quả trực tiếp bổ đề bắt tay (bài 3); một khái niệm HÌNH THỨC mới, không liên quan cấu trúc dữ liệu 'cây'."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.tree-graph]
requires: [math.connected-component, math.handshake-lemma]
concepts: [math.cay-do-thi]
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
Bớt đúng MỘT ống khỏi vòng luống 1→2→3→1 — đồ thị VẪN nối liền,
nhưng còn "thừa" đường nào để đi VÒNG không?
::::

::::explain{#cay-la-gi}
**Cây (lý thuyết đồ thị)** — đồ thị liên thông (bài 5) VÀ CÓ ĐÚNG
`n−1` cạnh, VỚI `n` LÀ số đỉnh — hệ quả TRỰC TIẾP của bổ đề bắt tay
(bài 3): một cây "VỪA ĐỦ" liên thông, KHÔNG thừa cạnh nào để tạo
CHU TRÌNH VÀ KHÔNG thiếu cạnh nào để rời rạc. Đây LÀ một khái niệm
HÌNH THỨC hoàn toàn MỚI, KHÔNG liên quan cấu trúc dữ liệu "cây" (nhánh
cha-con, phân cấp) mà một track LẬP TRÌNH khác có thể dùng:

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

def la_cay(luong, E):
    if not la_lien_thong(luong, E):
        return False
    return len(E) // 2 == len(luong) - 1


luong = {"luong_1", "luong_2", "luong_3"}
E_vong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"), ("luong_3", "luong_1"), ("luong_1", "luong_3")}

print(la_cay(luong, E_vong))
```

```text title=readonly
False
```

Luống 1→2→3→1 LÀ một VÒNG (tam giác) — liên thông ĐÚNG, NHƯNG CÓ `3`
cạnh trong khi `n−1 = 2`; THỪA đúng MỘT cạnh (chính LÀ cạnh khép vòng)
— KHÔNG phải cây.
::::

::::example{#bot-canh-khep-vong}
Bớt đúng cạnh khép vòng (`luong_3-luong_1`) — còn lại MỘT đường THẲNG:

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

def la_cay(luong, E):
    if not la_lien_thong(luong, E):
        return False
    return len(E) // 2 == len(luong) - 1


luong = {"luong_1", "luong_2", "luong_3"}
E_duong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(la_cay(luong, E_duong))
```

```text title=readonly
True
```

Còn lại ĐÚNG `2` cạnh cho `3` đỉnh, VẪN liên thông (đi luong_1→
luong_2→luong_3) — ĐÚNG định nghĩa cây: vừa đủ, không thừa không
thiếu.
::::

::::predict{#doan-dem-canh-chua-du commitOnce}
Byte THÊM một luống thứ tư (`luong_4`), KHÔNG nối ống nào tới nó, RỒI
đếm cạnh trên đồ thị VÒNG (tam giác) CŨ:

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

def la_cay(luong, E):
    if not la_lien_thong(luong, E):
        return False
    return len(E) // 2 == len(luong) - 1

luong4 = {"luong_1", "luong_2", "luong_3", "luong_4"}
E_vong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"), ("luong_3", "luong_1"), ("luong_1", "luong_3")}
print(len(E_vong) // 2 == len(luong4) - 1)
print(la_cay(luong4, E_vong))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì dòng TRƯỚC đã in `True` (số cạnh `3` KHỚP ĐÚNG `n−1=3`),
nên `la_cay` CŨNG phải trả `True`
::why
Gần đúng ở việc bạn đọc ĐÚNG dòng trước — số cạnh THẬT khớp `n−1`,
một quan sát chính xác VỀ phép đếm.

Chỗ lệch: `la_cay` kiểm liên thông TRƯỚC (dòng `if not la_lien_thong`)
— `luong_4` CÔ LẬP (không cạnh nào chạm nó trong `E_vong`), nên
`la_lien_thong` trả `False`, VÀ `la_cay` trả VỀ `False` NGAY, CHƯA
BAO GIỜ chạy tới phép đếm cạnh. Số cạnh khớp `n−1` LÀ điều kiện CẦN
NHƯNG chưa ĐỦ — còn cần liên thông (đây chính LÀ ví dụ chứng minh
điều đó).
::
:::

:::opt
Máy báo lỗi khi chạy — `luong4` CÓ `4` đỉnh NHƯNG `E_vong` chỉ nối
`3` trong số đó (`luong_4` không xuất hiện trong bất kỳ cặp nào),
Python từ chối tính liên thông trên một đỉnh "mồ côi"
::why
Gần đúng ở việc bạn để ý ĐÚNG `luong_4` không xuất hiện trong `E_vong`
— một quan sát chính xác VỀ dữ liệu.

Chỗ lệch: KHÔNG có gì "từ chối" cả — `dinh_toi_duoc(v, E)` CHỈ lần
theo cạnh CÓ trong `E`, một đỉnh không cạnh nào ĐƠN GIẢN không được
thêm vào tập tới được (`{luong_1,luong_2,luong_3}` ≠ `luong4`). So
sánh lệch nhau → `la_lien_thong` trả `False`, HOÀN TOÀN hợp lệ,
không lỗi gì.
::
:::
::::

::::code{#viet_la_cay}
Viết `la_cay(luong, E)` — kiểm ĐỒ THỊ `(luong, E)` có PHẢI một cây
hay không.

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
    return dinh_toi_duoc(v, E) == luong

def la_cay(luong, E):
    if not la_lien_thong(luong, E):
        return False
    return ___


luong = {"luong_1", "luong_2", "luong_3"}
E_duong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(la_cay(luong, E_duong))
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

def la_cay(luong, E):
    if not la_lien_thong(luong, E):
        return False
    return len(E) // 2 == len(luong) - 1


luong = {"luong_1", "luong_2", "luong_3"}
E_duong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(la_cay(luong, E_duong))
```

```python title=test
assert la_cay(set(), set()) is False, "khong dinh nao -- khong phai cay theo dinh nghia nay"
assert la_cay({"x"}, set()) is True, "mot dinh, khong canh -- van la cay"
luong = {"luong_1", "luong_2", "luong_3"}
E_duong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}
assert la_cay(luong, E_duong) is True, "duong thang ba dinh -- la cay"
E_vong = E_duong | {("luong_3", "luong_1"), ("luong_1", "luong_3")}
assert la_cay(luong, E_vong) is False, "them canh khep vong -- thua canh, khong con la cay"
luong4 = {"luong_1", "luong_2", "luong_3", "luong_4"}
assert la_cay(luong4, E_vong) is False, "so canh khop n-1 nhung khong lien thong -- van khong phai cay"
```

:::hints
- kind: attention
  body: "So sanh so canh THAT (len(E) chia 2, vi E luu ca hai chieu) voi n-1 (n = len(luong))."
- kind: strategy
  body: "len(E) // 2 == len(luong) - 1"
- kind: one-line
  body: "___ = len(E) // 2 == len(luong) - 1"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dem so canh that (len(E)//2) va so sanh voi len(luong)-1
  requireAst:
  - kind: uses-call, target: len, min: 2
  - kind: uses-operator, target: '//', min: 1
  - kind: uses-operator, target: '-', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cây LÀ đồ thị "vừa đủ" liên thông. Nhưng mọi cạnh Ở ĐÂY đều HAI
CHIỀU — nếu trạm bơm CHỈ đẩy nước MỘT chiều thì sao?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Trạm bơm CHỈ đẩy nước MỘT chiều tới từng luống — KHÔNG có ống nào
ngược lại TỪ luống VỀ trạm bơm. Cạnh MỘT chiều như VẬY — `E` VẪN
phải LÀ một quan hệ đối xứng (bài 1) để được TÍNH LÀ đồ thị "của
track này" không, hay đó LÀ một LOẠI đồ thị khác?
::::

::::checkpoint{mastery=0.8}
::::
