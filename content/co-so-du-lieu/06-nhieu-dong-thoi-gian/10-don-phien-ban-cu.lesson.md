---
id: co-so-du-lieu.nhieu-dong-thoi-gian.don-phien-ban-cu
title: Dọn phiên bản cũ
summary: "don_phien_ban_cu(danh_sach_phien_ban, watermark) giữ lại phiên bản MỚI NHẤT có ts <= watermark (có thể vẫn cần, cho giao dịch cũ nhất còn hoạt động) và MỌI phiên bản có ts > watermark (chưa từng bị 'vượt qua' bởi bất kỳ ai) — loại bỏ mọi phiên bản CŨ hơn watermark mà không phải bản được giữ lại đó."
locale: vi
track: co-so-du-lieu
module: nhieu-dong-thoi-gian
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [db.version-gc]
requires: [db.read-write-conflict-detection]
concepts: [db.version-gc]
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
Mỗi lần GHI thêm một phiên bản, KHÔNG hề xoá bản cũ — chuỗi phiên
bản chỉ dài THÊM mãi. Phiên bản nào MỚI đủ an toàn để xoá?
::::

::::explain{#watermark-la-gi}
`watermark` LÀ mốc snapshot CŨ nhất còn ĐANG được một giao dịch nào
đó SỬ dụng. `don_phien_ban_cu` giữ lại phiên bản MỚI nhất có `ts <=
watermark` (CÓ thể vẫn cần) VÀ mọi phiên bản có `ts > watermark`
(chưa bị "vượt qua" bởi ai) — loại BỎ phần còn lại:

```python title=readonly
def don_phien_ban_cu(danh_sach_phien_ban, watermark):
    ung_vien_giu = None
    for (ts, gia_tri) in danh_sach_phien_ban:
        if ts <= watermark:
            if ung_vien_giu is None or ts > ung_vien_giu[0]:
                ung_vien_giu = (ts, gia_tri)
    ket_qua = []
    for (ts, gia_tri) in danh_sach_phien_ban:
        if ts > watermark or (ts, gia_tri) == ung_vien_giu:
            ket_qua.append((ts, gia_tri))
    return ket_qua


ds = [(1, "a"), (2, "b"), (3, "c"), (5, "d"), (7, "e")]
print(don_phien_ban_cu(ds, 4))
```

```text title=readonly
[(3, 'c'), (5, 'd'), (7, 'e')]
```

`watermark=4` — phiên bản CÓ `ts <= 4` LÀ `(1,a)`, `(2,b)`, `(3,c)`;
MỚI nhất trong số ĐÓ LÀ `(3,c)`, giữ LẠI nó, loại `(1,a)` VÀ `(2,b)`
(chúng đã bị `(3,c)` "vượt qua", KHÔNG giao dịch nào có snapshot ÍT
nhất bằng `4` còn cần TỚI chúng). `(5,d)` VÀ `(7,e)` có `ts > 4` —
giữ NGUYÊN, vì có THỂ còn giao dịch mới hơn `watermark` cần đọc TỚI
chúng.
::::

::::example{#watermark-bang-khong}
Nếu `watermark=0` (KHÔNG có snapshot nào cũ TỚI mức đó), KHÔNG
phiên bản nào bị LOẠI:

```python title=readonly
print(don_phien_ban_cu(ds, 0))
```

```text title=readonly
[(1, 'a'), (2, 'b'), (3, 'c'), (5, 'd'), (7, 'e')]
```

KHÔNG có phiên bản NÀO thoả `ts <= 0` (mọi `ts` trong `ds` đều ÍT
nhất LÀ `1`) — `ung_vien_giu` giữ nguyên `None`. Điều kiện GIỮ chỉ
CÒN `ts > 0`, ĐÚNG cho MỌI phiên bản — không phiên bản NÀO bị loại.
::::

::::predict{#doan-watermark-rat-lon commitOnce}
`watermark=100` — LỚN hơn timestamp của MỌI phiên bản đang có:

```python
print(don_phien_ban_cu(ds, 100))
```

Dòng cuối in ra gì?

:::opt{correct}
`[(7, 'e')]`
:::

:::opt
`[]` — vì MỌI phiên bản đều CÓ `ts <= 100`, KHÔNG phiên bản nào
thoả `ts > watermark`, nên TẤT cả đều bị loại
::why
Gần đúng ở việc bạn tính ĐÚNG: đúng LÀ mọi `ts` trong `ds` đều nhỏ
hơn `100` — một quan sát chính xác về SO sánh số.

Chỗ lệch: điều kiện GIỮ lại KHÔNG chỉ LÀ `ts > watermark` — nó LÀ
`ts > watermark OR (ts, gia_tri) == ung_vien_giu`. Khi TẤT cả phiên
bản thoả `ts <= 100`, `ung_vien_giu` LÀ phiên bản có `ts` LỚN nhất
trong SỐ đó — chính LÀ `(7, 'e')` — VÀ nó thoả vế THỨ hai của điều
kiện, nên VẪN được giữ lại.
::
:::

:::opt
Máy báo lỗi — vì `watermark=100` LỚN hơn mọi timestamp thật SỰ đang
tồn tại trong `ds`, LÀ một giá trị không hợp LỆ
::why
Gần đúng ở việc bạn để Ý `100` LỚN hơn hẳn mọi `ts` thật — MỘT quan
sát đúng VỀ dữ liệu.

Chỗ lệch: `don_phien_ban_cu` KHÔNG hề kiểm tra `watermark` có "khớp"
với dữ liệu HAY không — nó chỉ đơn thuần LÀ một mốc SO sánh, hoàn
toàn hợp LỆ dù lớn hơn mọi `ts` thật. KHÔNG có `raise` nào cả.
::
:::
::::

::::code{#viet_don_phien_ban_cu}
Hoàn thiện `don_phien_ban_cu` — trong vòng lặp THỨ hai, giữ lại
phiên bản thoả `ts > watermark` HOẶC chính LÀ phiên bản cần giữ đã
tìm được Ở vòng lặp đầu.

```python title=starter
def don_phien_ban_cu(danh_sach_phien_ban, watermark):
    ung_vien_giu = None
    for (ts, gia_tri) in danh_sach_phien_ban:
        if ts <= watermark:
            if ung_vien_giu is None or ts > ung_vien_giu[0]:
                ung_vien_giu = (ts, gia_tri)
    ket_qua = []
    for (ts, gia_tri) in danh_sach_phien_ban:
        ___
    return ket_qua


ds = [(1, "a"), (2, "b"), (3, "c"), (5, "d"), (7, "e")]
print(don_phien_ban_cu(ds, 4))
```

```python title=solution
def don_phien_ban_cu(danh_sach_phien_ban, watermark):
    ung_vien_giu = None
    for (ts, gia_tri) in danh_sach_phien_ban:
        if ts <= watermark:
            if ung_vien_giu is None or ts > ung_vien_giu[0]:
                ung_vien_giu = (ts, gia_tri)
    ket_qua = []
    for (ts, gia_tri) in danh_sach_phien_ban:
        if ts > watermark or (ts, gia_tri) == ung_vien_giu:
            ket_qua.append((ts, gia_tri))
    return ket_qua


ds = [(1, "a"), (2, "b"), (3, "c"), (5, "d"), (7, "e")]
print(don_phien_ban_cu(ds, 4))
```

```python title=test
ds1 = [(1, "a"), (2, "b"), (3, "c"), (5, "d"), (7, "e")]
assert don_phien_ban_cu(ds1, 4) == [(3, "c"), (5, "d"), (7, "e")], "giu ban moi nhat truoc watermark, va moi ban sau watermark"
assert don_phien_ban_cu(ds1, 0) == ds1, "watermark truoc moi phien ban -- khong loai gi ca"
assert don_phien_ban_cu(ds1, 100) == [(7, "e")], "watermark sau moi phien ban -- chi giu ban moi nhat"
assert don_phien_ban_cu([], 5) == [], "danh sach rong -- ket qua rong"
assert don_phien_ban_cu([(1, "a")], 1) == [(1, "a")], "mot phien ban duy nhat, dung tai watermark -- giu lai"
```

:::hints
- kind: attention
  body: "Trong than vong for THU HAI, neu ts > watermark HOAC (ts, gia_tri) == ung_vien_giu, them (ts, gia_tri) vao ket_qua -- mot dong."
- kind: strategy
  body: "if ts > watermark or (ts, gia_tri) == ung_vien_giu: ket_qua.append((ts, gia_tri))"
- kind: one-line
  body: "if ts > watermark or (ts, gia_tri) == ung_vien_giu: ket_qua.append((ts, gia_tri))"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai giu lai phien ban co ts > watermark hoac chinh la ung_vien_giu
  requireAst:
  - kind: uses-name, target: watermark, min: 2
  - kind: uses-name, target: ung_vien_giu, min: 3
  - kind: uses-name, target: ts, min: 6
  - kind: uses-name, target: gia_tri, min: 3
  - kind: uses-name, target: ket_qua, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[\(3, 'c'\), \(5, 'd'\), \(7, 'e'\)\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Dọn dẹp đúng — giữ những gì CÒN cần, bỏ những gì đã bị "vượt qua"
KHÔNG ai còn dùng tới. Ghép TẤT cả q06 lại thành một hệ THỐNG MVCC
hoàn chỉnh trông ra sao?
::::

::::reflect{#nghi-lai}
`don_phien_ban_cu` chỉ giữ lại ĐÚNG những gì CÒN có thể CẦN — một
phiên bản CŨ hơn watermark nhưng đã bị MỘT phiên bản mới hơn (vẫn
cũ hơn watermark) "vượt qua" LÀ an toàn để xoá, vì KHÔNG giao dịch
nào (snapshot cũ nhất LÀ watermark) còn có thể NHÌN thấy nó nữa.
Version chain, đọc theo snapshot, hai mức cô lập, xung đột ghi-ghi,
write skew, SSI, VÀ dọn phiên bản cũ — bảy mảnh RIÊNG lẻ. Ghép TẤT
cả thành MỘT hệ thống MVCC hoàn chỉnh trông RA sao?
::::

::::checkpoint{mastery=0.8}
::::
