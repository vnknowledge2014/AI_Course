---
id: toan.dstt-giai-tich-cho-ai.nhan-vo-huong-vector
title: Nhân vô hướng
summary: "k·v — nhân MỖI thành phần VỚI cùng một số k; tăng khẩu phần GẤP k lần LÀ nhân vô hướng, KHÔNG phải cộng."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.vector-scalar-multiplication]
requires: [math.vector-addition]
concepts: [math.nhan-vo-huong-vector]
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
Nhân MỖI thành phần của vector lên GẤP ĐÔI (tăng khẩu phần tưới gấp
đôi) — có phải phép TOÁN khác cộng vector không?
::::

::::explain{#nhan-vo-huong}
CÓ, một phép TOÁN khác hẳn. **`k·v`** — nhân MỖI thành phần VỚI
CÙNG một số `k` (GỌI LÀ **số vô hướng** — MỘT số ĐƠN, khác vector);
tăng khẩu phần GẤP `k` lần LÀ nhân vô hướng, KHÔNG phải cộng (cộng
CẦN hai VECTOR, nhân vô hướng chỉ CẦN một SỐ VÀ một vector):

```python title=readonly
def nhan_vo_huong(k, v):
    return tuple(k * x for x in v)


luong_1 = (2.0, 5.0, 6.0)

print(nhan_vo_huong(2, luong_1))
```

```text title=readonly
(4.0, 10.0, 12.0)
```

MỖI thành phần NHÂN VỚI `2` — chiều dài GẤP đôi (`4.0`), nước GẤP
đôi (`10.0`), nắng GẤP đôi (`12.0`). "Tăng khẩu phần GẤP `k` lần"
nghĩa LÀ nhân TOÀN bộ vector VỚI `k`.
::::

::::example{#nhan-voi-0}
Nhân VỚI `0` — RA vector KHÔNG (mọi thành phần bằng `0`):

```python title=readonly
def nhan_vo_huong(k, v):
    return tuple(k * x for x in v)


luong_1 = (2.0, 5.0, 6.0)

print(nhan_vo_huong(0, luong_1))
```

```text title=readonly
(0.0, 0.0, 0.0)
```

NHÂN mọi thành phần VỚI `0` — TẤT CẢ VỀ `0`. Vector KHÔNG (`(0,0,0)`)
đóng vai TRÒ đơn vị của phép CỘNG vector (bài 2), giống HỆT số `0`
với phép cộng SỐ thường (T2.6 bài 23).
::::

::::predict{#doan-nhan-voi-am commitOnce}
Byte nhân vector `(2.0, 5.0, 6.0)` VỚI `k = -1`:

```python
def nhan_vo_huong(k, v):
    return tuple(k * x for x in v)

luong_1 = (2.0, 5.0, 6.0)
print(nhan_vo_huong(-1, luong_1))
```

Dòng cuối in ra gì?

:::opt{correct}
`(-2.0, -5.0, -6.0)`
:::

:::opt
`(2.0, 5.0, 6.0)` — vì nhân VỚI `-1` HAI lần MỚI đổi dấu, nhân MỘT
lần chỉ "ĐÁNH dấu" chứ chưa THỰC sự đổi giá TRỊ
::why
Gần đúng ở việc bạn nghĩ TỚI quy tắc "âm nhân âm RA dương" — MỘT
quy tắc THẬT của phép nhân (T2.1), áp dụng khi có HAI số âm.

Chỗ lệch: Ở ĐÂY chỉ CÓ MỘT số âm (`k=-1`), nhân VỚI một số DƯƠNG
(`2.0`, `5.0`, `6.0`) CHO ra kết quả ÂM NGAY LẦN đầu — `(-1)×2.0=
-2.0`, KHÔNG cần nhân HAI lần. "Đánh dấu rồi chưa đổi" KHÔNG phải
CÁCH phép nhân hoạt ĐỘNG — MỖI phép nhân TÍNH ra kết quả THẬT ngay
lập tức.
::
:::

:::opt
Máy báo lỗi khi chạy — `k=-1` LÀ một số ÂM, VÀ hàm `nhan_vo_huong`
chỉ được THIẾT kế cho `k` KHÔNG âm (giống MỌI ví dụ TRƯỚC, `k=2`,
`k=0`)
::why
Gần đúng ở việc bạn để ý ĐÚNG mọi ví dụ TRƯỚC dùng `k` KHÔNG âm —
một quan sát VỀ MẪU hình dữ liệu.

Chỗ lệch: KHÔNG có RÀNG buộc nào TRONG hàm giới hạn `k`, phép nhân
(`*`) HOẠT động BÌNH thường VỚI số ÂM. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_nhan_vo_huong}
Viết `nhan_vo_huong(k, v)` — nhân MỖI thành phần của `v` VỚI `k`.

```python title=starter
def nhan_vo_huong(k, v):
    return ___


luong_1 = (2.0, 5.0, 6.0)
print(nhan_vo_huong(2, luong_1))
```

```python title=solution
def nhan_vo_huong(k, v):
    return tuple(k * x for x in v)


luong_1 = (2.0, 5.0, 6.0)
print(nhan_vo_huong(2, luong_1))
```

```python title=test
assert nhan_vo_huong(2, ()) == (), "vector rong -- ket qua rong"
luong_1 = (2.0, 5.0, 6.0)
assert nhan_vo_huong(2, luong_1) == (4.0, 10.0, 12.0), "nhan gap doi"
assert nhan_vo_huong(0, luong_1) == (0.0, 0.0, 0.0), "nhan voi 0 -- vector khong"
assert nhan_vo_huong(-1, luong_1) == (-2.0, -5.0, -6.0), "nhan voi -1 -- doi dau"
assert nhan_vo_huong(0.5, (4.0, 6.0)) == (2.0, 3.0), "nhan voi phan so -- thu nho"
```

:::hints
- kind: attention
  body: "Nhan k voi TUNG thanh phan x cua v."
- kind: strategy
  body: "tuple(k * x for x in v)"
- kind: one-line
  body: "___ = tuple(k * x for x in v)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai nhan k voi tung thanh phan cua v
  requireAst:
  - kind: uses-call, target: tuple, min: 1
  - kind: uses-operator, target: '*', min: 1
  - kind: comprehension, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(4\.0, 10\.0, 12\.0\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhân vô hướng — phóng TO hoặc thu NHỎ một vector. So sánh `(4.0,
6.0)` VÀ `(2.0, 3.0)` — có CÙNG hướng không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

So sánh vector `(4.0, 6.0)` VÀ vector `(2.0, 3.0)` — CÓ "cùng
HƯỚNG" không (một LÀ bản phóng to của cái kia)? Đo "độ LỚN" một
vector — dùng con số NÀO?
::::

::::checkpoint{mastery=0.8}
::::
