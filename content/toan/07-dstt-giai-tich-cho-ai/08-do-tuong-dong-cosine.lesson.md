---
id: toan.dstt-giai-tich-cho-ai.do-tuong-dong-cosine
title: Độ tương đồng cosine
summary: "Độ tương đồng cosine — CHÍNH LÀ cos θ (bài 7), đặt TÊN ứng dụng: so sánh 'hồ SƠ' hai luống MÀ không quan tâm ĐỘ LỚN, chỉ quan tâm TỈ LỆ các thành phần — công cụ CHUẨN so sánh embedding trong AI hiện đại."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.cosine-similarity]
requires: [math.vector-angle]
concepts: [math.do-tuong-dong-cosine]
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
Có cách nào dùng `cos θ` LÀM thước đo "giống nhau" CHUẨN hoá, không
phụ thuộc ĐỘ LỚN?
::::

::::explain{#do-tuong-dong-cosine}
CÓ — VÀ đây LÀ công cụ ĐÓ. **Độ tương đồng cosine** — CHÍNH LÀ
`cos θ` (bài 7), CHỈ đặt TÊN theo ỨNG dụng: so sánh "hồ SƠ" hai
luống MÀ KHÔNG quan tâm ĐỘ LỚN, CHỈ quan tâm TỈ LỆ các thành phần —
công cụ CHUẨN so sánh embedding trong AI hiện đại:

```python title=readonly
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def do_tuong_dong_cosine(u, v):
    return tich_vo_huong(u, v) / (do_dai(u) * do_dai(v))


ho_so_a = (1.0, 2.0, 3.0)
ho_so_b = (10.0, 20.0, 30.0)

print(do_tuong_dong_cosine(ho_so_a, ho_so_b))
```

```text title=readonly
1.0
```

`ho_so_b` LÀ `ho_so_a` nhân GẤP `10` (bài 3) — TỈ LỆ giữa các thành
phần GIỐNG hệt nhau, CHỈ độ LỚN khác. Độ tương đồng cosine RA `1.0`
— HOÀN TOÀN giống nhau VỀ "hình DẠNG hồ sơ", dù MỘT hồ sơ lớn gấp
`10` lần hồ sơ KIA.
::::

::::example{#khoang-cach-lon-nhung-tuong-dong-cao}
So SÁNH với khoảng cách (bài 5) — hai thước đo cho câu TRẢ lời KHÁC
hẳn:

```python title=readonly
def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def khoang_cach(u, v):
    hieu = tuple(a - b for a, b in zip(u, v))
    return do_dai(hieu)


ho_so_a = (1.0, 2.0, 3.0)
ho_so_b = (10.0, 20.0, 30.0)

print(khoang_cach(ho_so_a, ho_so_b))
```

```text title=readonly
33.67491648096547
```

Khoảng CÁCH (bài 5) giữa `ho_so_a` VÀ `ho_so_b` RẤT lớn (`~33.67`)
— hai điểm Ở XA nhau TRONG không gian. NHƯNG độ tương ĐỒNG cosine
(bài NÀY) LẠI nói CHÚNG "giống hệt" (`1.0`) — VÌ cosine CHỈ quan
TÂM hướng, KHÔNG quan tâm độ LỚN, còn khoảng cách quan TÂM CẢ hai.
Hai thước đo ĐÚNG cho hai CÂU hỏi khác nhau.
::::

::::predict{#doan-dao-thu-tu commitOnce}
Byte so sánh `ho_so_a = (1.0, 2.0, 3.0)` VÀ `ho_so_c = (3.0, 2.0,
1.0)` — CÙNG BA con số, NHƯNG đảo THỨ tự:

```python
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def do_tuong_dong_cosine(u, v):
    return tich_vo_huong(u, v) / (do_dai(u) * do_dai(v))

ho_so_a = (1.0, 2.0, 3.0)
ho_so_c = (3.0, 2.0, 1.0)
print(round(do_tuong_dong_cosine(ho_so_a, ho_so_c), 4))
```

Dòng cuối in ra gì?

:::opt{correct}
`0.7143`
:::

:::opt
`1.0` — vì `ho_so_a` VÀ `ho_so_c` chứa CHÍNH XÁC cùng BA con số
(`1,2,3`), CHỈ khác thứ tự SẮP xếp, nên "hồ sơ" của chúng PHẢI
giống hệt nhau
::why
Gần đúng ở việc bạn để ý ĐÚNG cả hai hồ sơ chứa CÙNG tập số — một
quan sát VỀ NỘI dung đúng.

Chỗ lệch: độ tương đồng cosine so sánh TỪNG VỊ TRÍ tương ứng (bài
1: vị trí quyết định Ý nghĩa), KHÔNG so sánh "tập số CHỨA gì". Đảo
thứ tự thành `(3,2,1)` LÀM thành phần THỨ NHẤT đổi từ "dài" sang một
giá trị KHÁC hẳn Ở VỊ trí đó — hồ sơ THAY đổi thật SỰ, dù chứa cùng
con số. Kết quả THẤP hơn `1.0` (`≈0.714`) phản ánh SỰ khác biệt ĐÓ.
::
:::

:::opt
Máy báo lỗi khi chạy — `ho_so_a` VÀ `ho_so_c` LÀ hai biến RIÊNG
BIỆT nhưng chứa các con số TRÙNG nhau, Python từ chối SO sánh hai
vector có "nội DUNG trùng lặp"
::why
Gần đúng ở việc bạn để ý ĐÚNG hai vector chứa các con số TRÙNG nhau
— một quan sát VỀ dữ liệu.

Chỗ lệch: KHÔNG có RÀNG buộc nào NHƯ vậy — hai vector HOÀN TOÀN có
thể chứa các con số TRÙNG (dù khác thứ tự) mà KHÔNG gây lỗi GÌ.
Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_do_tuong_dong_cosine}
Viết `do_tuong_dong_cosine(u, v)` — tính độ tương đồng cosine giữa
hai hồ sơ.

```python title=starter
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def do_tuong_dong_cosine(u, v):
    return ___


ho_so_a = (1.0, 2.0, 3.0)
ho_so_b = (10.0, 20.0, 30.0)
print(do_tuong_dong_cosine(ho_so_a, ho_so_b))
```

```python title=solution
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def do_tuong_dong_cosine(u, v):
    return tich_vo_huong(u, v) / (do_dai(u) * do_dai(v))


ho_so_a = (1.0, 2.0, 3.0)
ho_so_b = (10.0, 20.0, 30.0)
print(do_tuong_dong_cosine(ho_so_a, ho_so_b))
```

```python title=test
ho_so_a = (1.0, 2.0, 3.0)
ho_so_b = (10.0, 20.0, 30.0)
assert do_tuong_dong_cosine(ho_so_a, ho_so_b) == 1.0, "cung ti le -- tuong dong tuyet doi"
ho_so_c = (3.0, 2.0, 1.0)
assert round(do_tuong_dong_cosine(ho_so_a, ho_so_c), 4) == 0.7143, "dao thu tu -- khac hoan"
assert do_tuong_dong_cosine((1.0, 0.0), (-1.0, 0.0)) == -1.0, "nguoc huong hoan toan"
assert do_tuong_dong_cosine((1.0, 0.0), (0.0, 1.0)) == 0.0, "vuong goc -- khong lien quan"
```

:::hints
- kind: attention
  body: "Do tuong dong cosine CHINH LA cos_goc bai truoc -- chia tich vo huong cho tich hai do dai."
- kind: strategy
  body: "tich_vo_huong(u, v) / (do_dai(u) * do_dai(v))"
- kind: one-line
  body: "___ = tich_vo_huong(u, v) / (do_dai(u) * do_dai(v))"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai chia tich_vo_huong(u,v) cho do_dai(u)*do_dai(v)
  requireAst:
  - kind: uses-call, target: tich_vo_huong, min: 1
  - kind: uses-call, target: do_dai, min: 2
  - kind: uses-operator, target: '/', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^1\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Độ tương đồng cosine — đóng TRỌN cụm vector. Một VƯỜN CÓ nhiều luống
— LÀM sao viết GỌN toàn bộ dữ liệu thành MỘT cấu trúc DUY nhất?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Một VƯỜN CÓ nhiều luống — LÀM sao VIẾT gọn TOÀN bộ dữ liệu (nhiều
vector) thành MỘT cấu trúc DUY nhất?
::::

::::checkpoint{mastery=0.8}
::::
