---
id: co-so-du-lieu.nhieu-dong-thoi-gian.vi-sao-can-nhieu-phien-ban
title: Vì sao cần nhiều phiên bản
summary: "Với khoá (q05), MỌI giao dịch chỉ ĐỌC vẫn phải chờ nếu MỘT giao dịch khác đang GHI — dù đọc không hề sửa gì. Nếu mỗi giao dịch nhìn thấy một PHIÊN bản dữ liệu riêng của chính nó, KHÔNG ai cần chờ ai để đọc — đây là ý tưởng cốt lõi của MVCC."
locale: vi
track: co-so-du-lieu
module: nhieu-dong-thoi-gian
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [db.why-multiple-versions]
requires: [db.deadlock-detection]
concepts: [db.why-multiple-versions]
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
q05 khoá độc quyền CHẶN giao dịch ghi cho tới khi rảnh — nhưng khoá
đọc/ghi (bài `khoa-doc-va-khoa-ghi`) VẪN chặn người ĐỌC nếu có ai
đang GHI. Có cách nào ĐỌC mà KHÔNG cần chờ ai cả?
::::

::::explain{#dem-so-lan-cho}
`dem_cho_bang_khoa` đếm SỐ giao dịch đọc phải CHỜ khi có MỘT giao
dịch ghi đang giữ khoá — MỌI giao dịch đọc đều bị CHẶN, dù chúng
chỉ muốn NHÌN, không sửa gì:

```python title=readonly
def dem_cho_bang_khoa(so_giao_dich_doc, dang_co_giao_dich_ghi):
    if dang_co_giao_dich_ghi:
        return so_giao_dich_doc
    return 0


def dem_cho_bang_mvcc(so_giao_dich_doc, dang_co_giao_dich_ghi):
    return 0


print(dem_cho_bang_khoa(5, True))
print(dem_cho_bang_mvcc(5, True))
```

```text title=readonly
5
0
```

CÓ một giao dịch ghi đang giữ khoá (`dang_co_giao_dich_ghi=True`) —
`dem_cho_bang_khoa` báo CẢ `5` giao dịch đọc đều phải CHỜ.
`dem_cho_bang_mvcc` LUÔN trả về `0` — dù CÓ giao dịch ghi đang chạy,
KHÔNG giao dịch đọc nào phải chờ CẢ, vì mỗi giao dịch nhìn MỘT
phiên bản riêng.
::::

::::example{#khong-ai-ghi-thi-khong-ai-cho}
Nếu KHÔNG có giao dịch ghi nào đang chạy, khoá KHÔNG bắt ai chờ CẢ
— giống hệt MVCC:

```python title=readonly
print(dem_cho_bang_khoa(3, False))
```

```text title=readonly
0
```

Không CÓ ai giữ khoá ghi — `dang_co_giao_dich_ghi=False`, hàm rơi
THẲNG tới `return 0`. Sự khác BIỆT giữa khoá và MVCC CHỈ lộ ra khi
CÓ một giao dịch ghi đang chạy — đó chính LÀ lúc khoá bắt người đọc
phải CHỜ, còn MVCC thì không.
::::

::::predict{#doan-nhieu-nguoi-doc-hon commitOnce}
Có `10` giao dịch đọc, VÀ một giao dịch ghi ĐANG giữ khoá:

```python
print(dem_cho_bang_khoa(10, True))
```

Dòng cuối in ra gì?

:::opt{correct}
`10`
:::

:::opt
`1` — vì CHỈ giao dịch ghi ĐANG hoạt động (chính nó) mới tính LÀ
"đang chờ", MƯỜI giao dịch đọc kia không hề bị đếm
::why
Gần đúng ở việc bạn nghĩ TỚI vai trò của "giao dịch ghi" — MỘT
điểm THẬT sự quan trọng trong tình huống này.

Chỗ lệch: `dem_cho_bang_khoa` đếm SỐ giao dịch ĐỌC (tham số ĐẦU,
`so_giao_dich_doc=10`) phải CHỜ, không phải đếm giao dịch GHI —
khi `dang_co_giao_dich_ghi=True`, hàm trả VỀ ĐÚNG `10`, tức LÀ TẤT
cả mười giao dịch đọc.
::
:::

:::opt
Máy báo lỗi — vì `10` giao dịch cùng CHỜ MỘT khoá LÀ một số lượng
không hợp LỆ
::why
Gần đúng ở việc bạn nghĩ TỚI một GIỚI hạn hợp lý cho số lượng giao
dịch đồng thời — một MỐI lo thực tế cho hệ thống THẬT.

Chỗ lệch: `dem_cho_bang_khoa` chỉ LÀ một phép SO sánh VÀ trả về số
nguyên — KHÔNG có `raise` nào, VÀ không CÓ giới hạn nào được kiểm
tra trong hàm NÀY.
::
:::
::::

::::code{#viet_dem_cho_bang_khoa}
Hoàn thiện `dem_cho_bang_khoa` — nếu ĐANG có giao dịch ghi, TRẢ về
đúng số giao dịch đọc đang bị CHẶN.

```python title=starter
def dem_cho_bang_khoa(so_giao_dich_doc, dang_co_giao_dich_ghi):
    if dang_co_giao_dich_ghi:
        ___
    return 0


def dem_cho_bang_mvcc(so_giao_dich_doc, dang_co_giao_dich_ghi):
    return 0


print(dem_cho_bang_khoa(5, True), dem_cho_bang_mvcc(5, True))
```

```python title=solution
def dem_cho_bang_khoa(so_giao_dich_doc, dang_co_giao_dich_ghi):
    if dang_co_giao_dich_ghi:
        return so_giao_dich_doc
    return 0


def dem_cho_bang_mvcc(so_giao_dich_doc, dang_co_giao_dich_ghi):
    return 0


print(dem_cho_bang_khoa(5, True), dem_cho_bang_mvcc(5, True))
```

```python title=test
assert dem_cho_bang_khoa(5, True) == 5, "5 giao dich doc phai cho khi co giao dich ghi"
assert dem_cho_bang_khoa(3, False) == 0, "khong ai ghi thi khong ai cho"
assert dem_cho_bang_khoa(10, True) == 10, "10 giao dich doc deu phai cho"
assert dem_cho_bang_khoa(0, True) == 0, "khong co giao dich doc nao thi khong ai cho ca"
assert dem_cho_bang_mvcc(10, True) == 0, "MVCC khong bao gio bat nguoi doc cho"
```

:::hints
- kind: attention
  body: "Neu dang_co_giao_dich_ghi la True, tra ve dung so_giao_dich_doc -- mot dong."
- kind: strategy
  body: "return so_giao_dich_doc"
- kind: one-line
  body: "return so_giao_dich_doc"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai return so_giao_dich_doc khi dang_co_giao_dich_ghi la True
  requireAst:
  - kind: uses-name, target: so_giao_dich_doc, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^5 0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
MVCC không bắt người đọc chờ AI cả — mỗi giao dịch nhìn một phiên
bản RIÊNG. Nhưng "phiên bản riêng" đó trông NHƯ thế nào — dữ liệu
lưu ra sao?
::::

::::reflect{#nghi-lai}
Khoá đọc/ghi (q05) buộc người ĐỌC phải chờ nếu CÓ ai đang ghi — một
sự đánh ĐỔI hợp lý khi CẦN đảm bảo tuyệt đối "không đọc dữ liệu dở
dang". MVCC (Multi-Version Concurrency Control) chọn một hướng
KHÁC: giữ LẠI nhiều phiên bản của CÙNG một dữ liệu, mỗi giao dịch
đọc đúng phiên bản PHÙ hợp với thời điểm nó bắt đầu — không cần
chờ, không cần khoá đọc. Nhưng giữ NHIỀU phiên bản nghĩa LÀ gì cụ
thể — mỗi bản ghi lưu trữ RA sao?
::::

::::checkpoint{mastery=0.8}
::::
