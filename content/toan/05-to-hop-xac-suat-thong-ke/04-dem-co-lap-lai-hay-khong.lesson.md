---
id: toan.to-hop-xac-suat-thong-ke.dem-co-lap-lai-hay-khong
title: "Đếm: lặp lại được phép hay không"
summary: "Hai kiểu đếm khác nhau tuỳ điều kiện bài toán: có lặp (mỗi bước vẫn đủ n lựa chọn) cho n×n×...×n; không lặp (mỗi bước bớt một lựa chọn đã dùng) cho n×(n−1)×(n−2)×... — cùng quy tắc nhân, khác số lựa chọn mỗi bước."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.permutation-vs-repetition]
requires: [math.rule-of-product]
concepts: [math.co-lap-khong-lap]
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
Túi CHỈ có đúng bốn hạt giống KHÁC nhau — dùng hạt nào rồi thì hạt
đó HẾT. Gieo cả bốn vào bốn luống — quy tắc nhân bài 3 còn tính
thẳng `4×4×4×4` được không?
::::

::::explain{#co-lap-khong-lap}
Không. Hai kiểu đếm KHÁC nhau tuỳ bài toán CÒN đủ lựa chọn mỗi bước
hay không: **có lặp** — mỗi bước VẪN đủ `n` lựa chọn (dùng rồi vẫn
còn, như bài 3: hạt giống mua thêm được) — dãy lựa chọn LÀ `[n, n,
n, ...]`. **Không lặp** — mỗi bước BỚT một lựa chọn đã dùng (túi
chỉ có đúng bấy nhiêu hạt) — dãy lựa chọn LÀ `[n, n−1, n−2, ...]`:

```python title=readonly
def day_co_lap(n, k):
    return [n] * k

def dem_cach_chon_lien_tiep(cac_so):
    ket_qua = 1
    for so in cac_so:
        ket_qua = ket_qua * so
    return ket_qua


print(day_co_lap(4, 4))
print(dem_cach_chon_lien_tiep(day_co_lap(4, 4)))
```

```text title=readonly
[4, 4, 4, 4]
256
```

`day_co_lap(4, 4)` DỰNG dãy `[4,4,4,4]` — bốn bước, MỖI bước VẪN đủ
4 lựa chọn (túi hạt vô hạn, dùng bao nhiêu cũng còn). Quy tắc nhân
(bài 3) cho `4⁴=256`. Đây LÀ tình huống bài 3 (hạt giống mua thêm
được, không bao giờ hết).
::::

::::example{#khong-lap-giam-dan}
Túi CHỈ có đúng bốn hạt — dùng một hạt thì bước SAU chỉ còn ba lựa
chọn, KHÔNG phải bốn:

```python title=readonly
def day_khong_lap(n, k):
    return [n - i for i in range(k)]

def dem_cach_chon_lien_tiep(cac_so):
    ket_qua = 1
    for so in cac_so:
        ket_qua = ket_qua * so
    return ket_qua


print(day_khong_lap(4, 4))
print(dem_cach_chon_lien_tiep(day_khong_lap(4, 4)))
```

```text title=readonly
[4, 3, 2, 1]
24
```

`day_khong_lap(4, 4)` DỰNG dãy `[4,3,2,1]` — luống 1 có 4 hạt để
chọn, luống 2 CHỈ còn 3 (một hạt đã dùng), luống 3 còn 2, luống 4
còn ĐÚNG 1. Cùng quy tắc nhân, `4×3×2×1=24` — KHÁC HẲN `256`.
::::

::::predict{#doan-chon-it-hon-tui commitOnce}
Byte CHỈ gieo hai luống (không phải cả bốn), vẫn từ túi bốn hạt
không lặp:

```python
def day_khong_lap(n, k):
    return [n - i for i in range(k)]

def dem_cach_chon_lien_tiep(cac_so):
    ket_qua = 1
    for so in cac_so:
        ket_qua = ket_qua * so
    return ket_qua

print(day_khong_lap(4, 2))
print(dem_cach_chon_lien_tiep(day_khong_lap(4, 2)))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`[4, 3]`, rồi `12`
:::

:::opt
`[4, 3]`, rồi `24` — vì túi VẪN có bốn hạt như cũ, tổng số cách phải
GIỮ NGUYÊN dù chỉ gieo hai luống thay vì bốn
::why
Gần đúng ở việc bạn đọc ĐÚNG `day_khong_lap(4, 2)` LÀ `[4, 3]` (chỉ
HAI bước, vì `k=2`) — dòng đầu đúng.

Chỗ lệch: SỐ HẠT trong túi (`n=4`) không quyết định KẾT QUẢ đếm một
mình — SỐ BƯỚC (`k`, mấy luống được gieo) mới quyết định dãy DÀI bao
nhiêu. `day_khong_lap(4, 2)` chỉ có HAI phần tử (`[4, 3]`), nên quy
tắc nhân CHỈ nhân hai số đó: `4×3=12`, không phải `4×3×2×1=24`.
Gieo ÍT luống hơn thì tự nhiên ÍT bước nhân hơn, kết quả NHỎ hơn.
::
:::

:::opt
Máy báo lỗi khi chạy — `day_khong_lap(4, 2)` chỉ trả về HAI phần tử
trong khi túi có BỐN hạt, Python phát hiện thiếu VÀ từ chối tính
::why
Gần đúng ở việc bạn để ý túi có bốn hạt mà dãy CHỈ có hai phần tử —
một quan sát đúng VỀ SỐ LƯỢNG.

Chỗ lệch: Python KHÔNG hề đòi dãy phải "dùng hết" số hạt trong túi
— `day_khong_lap(n, k)` được ĐỊNH NGHĨA để dừng SAU đúng `k` bước,
dù túi còn thừa bao nhiêu hạt chưa dùng tới. Biên dịch sạch, chạy
sạch — đây chính LÀ tình huống "chọn `k` trong `n`", không phải lỗi.
::
:::
::::

::::code{#viet_day_khong_lap}
Viết `day_khong_lap(n, k)` — trả về danh sách `k` số GIẢM DẦN bắt
đầu từ `n`, biểu diễn số lựa chọn CÒN LẠI mỗi bước khi KHÔNG lặp.

```python title=starter
def day_khong_lap(n, k):
    return ___


print(day_khong_lap(4, 4))
```

```python title=solution
def day_khong_lap(n, k):
    return [n - i for i in range(k)]


print(day_khong_lap(4, 4))
```

```python title=test
assert day_khong_lap(4, 0) == [], "khong buoc nao -- day rong"
assert day_khong_lap(5, 1) == [5], "mot buoc -- day chi co n"
assert day_khong_lap(4, 2) == [4, 3], "hai buoc dau, giam dan tu n"
assert day_khong_lap(4, 4) == [4, 3, 2, 1], "du n buoc -- giam het ve 1"
def dem_cach_chon_lien_tiep(cac_so):
    ket_qua = 1
    for so in cac_so:
        ket_qua = ket_qua * so
    return ket_qua
assert dem_cach_chon_lien_tiep(day_khong_lap(4, 4)) == 24, "4x3x2x1 = 24"
assert dem_cach_chon_lien_tiep(day_khong_lap(4, 2)) == 12, "4x3 = 12, khac han 4x4=16 (co lap)"
```

:::hints
- kind: attention
  body: "Dung list comprehension: moi buoc thu i (bat dau tu 0) thi con lai n - i lua chon."
- kind: strategy
  body: "[n - i for i in range(k)]"
- kind: one-line
  body: "___ = [n - i for i in range(k)]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung list comprehension voi range(k), giam dan tu n bang phep tru
  requireAst:
  - kind: comprehension, min: 1
  - kind: uses-call, target: range, min: 1
  - kind: uses-operator, target: '-', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[4, 3, 2, 1\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không lặp, gieo HẾT túi — dãy giảm dần tới 1. Có tên riêng cho phép
nhân MỘT dãy như vậy không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`day_khong_lap(4, 4)` cho `[4,3,2,1]`, nhân dồn ra `24` — cách xếp
thứ tự thu hoạch TOÀN BỘ bốn luống. Nhân một dãy giảm dần TỪ `n`
XUỐNG `1` như vậy có tên riêng không, và tên đó viết gọn thế nào?
::::

::::checkpoint{mastery=0.8}
::::
