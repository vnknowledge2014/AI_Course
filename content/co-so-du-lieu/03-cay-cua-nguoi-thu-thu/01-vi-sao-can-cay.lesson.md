---
id: co-so-du-lieu.cay-cua-nguoi-thu-thu.vi-sao-can-cay
title: Vì sao cần cây
summary: "Quét tuyến tính (q00-q02) cần TỚI N phép so sánh cho N bản ghi khi khoá cần tìm nằm CUỐI (hoặc không tồn tại) — chi phí LỚN DẦN theo số lượng. Một cấu trúc CÂY, nơi mỗi node chứa NHIỀU khoá đã sắp xếp, mở ra khả năng bỏ qua CẢ MỘT NHÁNH cùng lúc thay vì so sánh từng khoá một."
locale: vi
track: co-so-du-lieu
module: cay-cua-nguoi-thu-thu
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [db.why-a-tree]
requires: [db.recovery-continue-operating]
concepts: [db.why-a-tree]
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
q00-q02 quét TUYẾN tính — TỪ sector `0`, so sánh TỪNG bản ghi một.
NHẬT ký CÀNG dài, quét CÀNG lâu. LÀM sao đo được CÁI giá đó, bằng
CON số?
::::

::::explain{#dem-so-sanh}
`dem_so_sanh_tuyen_tinh` đếm ĐÚNG số LẦN so sánh cần LÀM để tìm MỘT
khoá TRONG một danh sách đã sắp XẾP — quét TỪ đầu, dừng khi khớp:

```python title=readonly
def dem_so_sanh_tuyen_tinh(danh_sach_khoa, khoa_can_tim):
    dem = 0
    for k in danh_sach_khoa:
        dem += 1
        if k == khoa_can_tim:
            return dem
    return dem


danh_sach = [10, 20, 30, 40, 50, 60]
print(dem_so_sanh_tuyen_tinh(danh_sach, 10))
print(dem_so_sanh_tuyen_tinh(danh_sach, 60))
```

```text title=readonly
1
6
```

Tìm `10` (khoá ĐẦU) chỉ tốn `1` LẦN so sánh. Tìm `60` (khoá CUỐI)
tốn ĐỦ `6` lần — PHẢI đi QUA hết mọi khoá TRƯỚC nó. Chi phí PHỤ
thuộc VÀO vị trí khoá NẰM Ở đâu, VÀ có bao NHIÊU khoá TRƯỚC nó.
::::

::::example{#chi-phi-lon-dan}
VỚI `10000` khoá, tìm khoá CUỐI cùng tốn ĐÚNG `10000` lần so SÁNH
— chi phí lớn DẦN đúng theo số LƯỢNG bản ghi:

```python title=readonly
danh_sach_lon = list(range(1, 10001))
print(dem_so_sanh_tuyen_tinh(danh_sach_lon, 10000))
```

```text title=readonly
10000
```

`10000` khoá, `10000` lần SO sánh Ở trường hợp XẤU nhất — GẤP đôi
số khoá LÀ gấp đôi chi PHÍ. Đây chính LÀ điều `tim_diem_dung_that`
(q02) VÀ `tim_bang_quet` (q01) đều CHỊU — quét TUYẾN tính KHÔNG có
cách nào "nhảy CÓC" qua nhiều bản ghi CÙNG lúc.
::::

::::predict{#doan-chi-phi-danh-sach-lon commitOnce}
Byte tìm khoá CUỐI cùng TRONG một danh sách `10000` khoá LIÊN tục
TỪ `1` đến `10000`:

```python
danh_sach = list(range(1, 10001))
print(dem_so_sanh_tuyen_tinh(danh_sach, 10000))
```

Dòng cuối in ra gì?

:::opt{correct}
`10000`
:::

:::opt
`100` — vì máy tính HIỆN đại đủ NHANH để "nhảy CÓC" qua nhiều phần
tử CÙNG lúc, KHÔNG cần so sánh TỪNG cái
::why
Gần đúng ở việc bạn nghĩ TỚI tốc độ xử LÝ nhanh của máy TÍNH hiện
đại — MỘT trực giác hợp lý VỀ phần CỨNG.

Chỗ lệch: `dem_so_sanh_tuyen_tinh` LÀ một vòng LẶP tự viết, duyệt
ĐÚNG TỪNG phần tử một (`for k in danh_sach_khoa`) — máy CHẠY nhanh
KHÔNG có nghĩa LÀ "bỏ qua" bước NÀO, nó vẫn PHẢI thực thi đủ `10000`
vòng lặp, MỖI vòng tăng `dem` đúng MỘT lần.
::
:::

:::opt
`1` — vì Python TỰ tối ưu, tìm THẲNG tới vị trí ĐÚNG mà KHÔNG cần
duyệt QUA các phần tử KHÁC
::why
Gần đúng ở việc bạn tin Python đủ "THÔNG minh" tối ưu code TỰ động
— một niềm TIN hợp lý VỚI một số THAO tác built-in (như
`list.index`).

Chỗ lệch: `dem_so_sanh_tuyen_tinh` LÀ hàm do NGƯỜI viết tự ĐỊNH
nghĩa, KHÔNG phải MỘT built-in được tối ưu NGẦM — Python chạy ĐÚNG
những gì vòng lặp VIẾT ra, từng bước MỘT, không "đoán" được VỊ trí.
::
:::
::::

::::code{#viet_dem_so_sanh_tuyen_tinh}
Hoàn thiện `dem_so_sanh_tuyen_tinh(danh_sach_khoa, khoa_can_tim)` —
dừng NGAY VÀ trả về số lần so SÁNH khi khớp khoá.

```python title=starter
def dem_so_sanh_tuyen_tinh(danh_sach_khoa, khoa_can_tim):
    dem = 0
    for k in danh_sach_khoa:
        dem += 1
        ___
    return dem


danh_sach = [10, 20, 30, 40, 50, 60]
print(dem_so_sanh_tuyen_tinh(danh_sach, 60))
```

```python title=solution
def dem_so_sanh_tuyen_tinh(danh_sach_khoa, khoa_can_tim):
    dem = 0
    for k in danh_sach_khoa:
        dem += 1
        if k == khoa_can_tim: return dem
    return dem


danh_sach = [10, 20, 30, 40, 50, 60]
print(dem_so_sanh_tuyen_tinh(danh_sach, 60))
```

```python title=test
danh_sach = [10, 20, 30, 40, 50, 60]
assert dem_so_sanh_tuyen_tinh(danh_sach, 10) == 1, "khoa dau -- 1 lan so sanh"
assert dem_so_sanh_tuyen_tinh(danh_sach, 60) == 6, "khoa cuoi -- 6 lan so sanh"
assert dem_so_sanh_tuyen_tinh(danh_sach, 30) == 3, "khoa giua"
assert dem_so_sanh_tuyen_tinh(danh_sach, 999) == 6, "khoa khong ton tai -- quet het"
assert dem_so_sanh_tuyen_tinh([], 5) == 0, "danh sach rong"
```

:::hints
- kind: attention
  body: "Neu k == khoa_can_tim thi return dem ngay -- mot dong."
- kind: strategy
  body: "if k == khoa_can_tim: return dem"
- kind: one-line
  body: "if k == khoa_can_tim: return dem"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai return dem ngay khi k == khoa_can_tim
  requireAst:
  - kind: uses-name, target: k, min: 1
  - kind: uses-name, target: dem, min: 2
  - kind: uses-name, target: khoa_can_tim, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^6\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chi phí quét tuyến TÍNH lớn dần THEO số lượng — không có CÁCH nào
"nhảy cóc". Một CÂY, nơi MỖI node chứa NHIỀU khoá đã sắp xếp, CÓ
thể bỏ qua CẢ một nhánh CÙNG lúc — bắt đầu TỪ đâu?
::::

::::reflect{#nghi-lai}
Quét tuyến tính TỐN đúng số lần so SÁNH bằng vị trí khoá — CÀNG
nhiều bản ghi, CÀNG tốn. Một cấu trúc CÂY thay đổi luật CHƠI: mỗi
NODE (một trang, một SECTOR) chứa NHIỀU khoá đã sắp XẾP, và mỗi
bước đi XUỐNG cây có thể LOẠI bỏ cả một NHÁNH lớn cùng lúc, thay VÌ
so sánh TỪNG khoá một. Bước đầu tiên: MỘT node như vậy TRÔNG như
thế nào?
::::

::::checkpoint{mastery=0.8}
::::
