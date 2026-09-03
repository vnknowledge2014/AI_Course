---
id: toan.do-thi-modular-dai-so-truu-tuong.duong-di-va-chu-trinh
title: Đường đi và chu trình
summary: "Đường đi — dãy đỉnh LIÊN TIẾP nối bằng cạnh, KHÔNG lặp đỉnh; chu trình — đường đi mà đỉnh ĐẦU trùng đỉnh CUỐI; hai khái niệm HÌNH THỨC hoá \"đi từ luống này sang luống khác theo ống tưới\"."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.path-and-cycle]
requires: [math.handshake-lemma]
concepts: [math.duong-di, math.chu-trinh]
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
Từ luống 1, nước có "đi" được TỚI luống 3 không (qua luống 2)? "Đi
được" theo nghĩa ĐỒ THỊ là gì?
::::

::::explain{#duong-di-va-chu-trinh}
**Đường đi** — dãy đỉnh LIÊN TIẾP nối bằng cạnh, KHÔNG lặp đỉnh.
**Chu trình** — đường đi mà đỉnh ĐẦU trùng đỉnh CUỐI. Hai khái niệm
HÌNH THỨC hoá "đi TỪ luống này SANG luống khác theo ống tưới":

```python title=readonly
def la_duong_di(day, E):
    if len(day) != len(set(day)):
        return False
    return all((day[i], day[i + 1]) in E for i in range(len(day) - 1))


E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(la_duong_di(["luong_1", "luong_2", "luong_3"], E))
```

```text title=readonly
True
```

`luong_1→luong_2→luong_3` — BA đỉnh KHÁC nhau (`len(day)=len(set(day))`,
T2.4 bài 1), VÀ hai bước LIÊN TIẾP ĐỀU LÀ cạnh THẬT trong `E`
(`(luong_1,luong_2)∈E`, `(luong_2,luong_3)∈E`). Một đường đi HỢP LỆ.
::::

::::example{#khong-noi-duoc}
KHÔNG có cạnh trực tiếp — dãy KHÔNG PHẢI đường đi:

```python title=readonly
def la_duong_di(day, E):
    if len(day) != len(set(day)):
        return False
    return all((day[i], day[i + 1]) in E for i in range(len(day) - 1))


E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(la_duong_di(["luong_1", "luong_3"], E))
```

```text title=readonly
False
```

`(luong_1,luong_3)` KHÔNG có trong `E` — KHÔNG cạnh TRỰC TIẾP nối
`luong_1` VỚI `luong_3` (dù có đường ĐI QUA `luong_2`). Dãy CHỈ HAI
đỉnh nhưng THIẾU cạnh giữa CHÚNG — không phải đường đi.
::::

::::predict{#doan-duong-di-lap-dinh commitOnce}
Byte thử "đi" TỪ `luong_1` QUA `luong_2` RỒI QUAY LẠI `luong_1`:

```python
def la_duong_di(day, E):
    if len(day) != len(set(day)):
        return False
    return all((day[i], day[i + 1]) in E for i in range(len(day) - 1))

E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}
print(la_duong_di(["luong_1", "luong_2", "luong_1"], E))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì HAI bước LIÊN TIẾP (`luong_1→luong_2` VÀ `luong_2→luong_1`)
ĐỀU LÀ cạnh THẬT trong `E`, VÀ đường đi CHỈ cần từng BƯỚC hợp lệ
::why
Gần đúng ở việc bạn kiểm ĐÚNG cả hai bước liên tiếp ĐỀU LÀ cạnh thật
trong `E` — quan sát đó chính xác VỀ phần CẠNH.

Chỗ lệch: định nghĩa đường đi (đầu bài) đòi THÊM một điều: KHÔNG lặp
đỉnh. `["luong_1","luong_2","luong_1"]` có `luong_1` xuất hiện HAI
LẦN — `len(day)=3` NHƯNG `len(set(day))=2` (chỉ hai đỉnh KHÁC nhau)
— hai số KHÔNG khớp, `la_duong_di` trả VỀ `False` NGAY từ điều kiện
ĐẦU tiên, chưa cần kiểm cạnh.
::
:::

:::opt
Máy báo lỗi khi chạy — `range(len(day) - 1)` với `day` có PHẦN TỬ
LẶP LẠI khiến `range` tính RA một chỉ số ÂM
::why
Gần đúng ở việc bạn để ý `day` có phần tử LẶP — một quan sát VỀ dữ
liệu.

Chỗ lệch: `len(day)` LUÔN LÀ `3` (đếm SỐ phần tử, KỂ CẢ trùng lặp) —
`range(3-1)=range(2)` hoàn toàn BÌNH THƯỜNG, KHÔNG âm. Việc "lặp
đỉnh" bị PHÁT HIỆN bằng cách SO SÁNH với `set(day)` (bài kiểm ĐẦU
tiên trong hàm), không phải bằng LỖI của `range`.
::
:::
::::

::::code{#viet_la_duong_di}
Viết `la_duong_di(day, E)` — kiểm `day` (dãy đỉnh) có LÀ một đường
đi hợp lệ trong đồ thị `E` hay không.

```python title=starter
def la_duong_di(day, E):
    if len(day) != len(set(day)):
        return False
    return ___


E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(la_duong_di(["luong_1", "luong_2", "luong_3"], E))
```

```python title=solution
def la_duong_di(day, E):
    if len(day) != len(set(day)):
        return False
    return all((day[i], day[i + 1]) in E for i in range(len(day) - 1))


E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(la_duong_di(["luong_1", "luong_2", "luong_3"], E))
```

```python title=test
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}
assert la_duong_di([], E) is True, "day rong -- khong buoc nao can kiem, hop le"
assert la_duong_di(["luong_1"], E) is True, "mot dinh -- luon la duong di hop le"
assert la_duong_di(["luong_1", "luong_3"], E) is False, "khong co canh truc tiep"
assert la_duong_di(["luong_1", "luong_2", "luong_1"], E) is False, "lap dinh -- khong phai duong di"
assert la_duong_di(["luong_1", "luong_2", "luong_3"], E) is True, "phai khop vi du chinh"
```

:::hints
- kind: attention
  body: "Dung all() voi generator: kiem tung cap (day[i], day[i+1]) co trong E khong."
- kind: strategy
  body: "all((day[i], day[i + 1]) in E for i in range(len(day) - 1))"
- kind: one-line
  body: "___ = all((day[i], day[i + 1]) in E for i in range(len(day) - 1))"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung all() voi generator, kiem tung cap dinh lien tiep co trong E
  requireAst:
  - kind: uses-call, target: all, min: 1
  - kind: uses-operator, target: in, min: 1
  - kind: uses-call, target: range, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Từ luống 1, có đường đi TỚI mọi luống khác. Cả VƯỜN có "nối liền"
với nhau không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Từ luống 1, có đường đi TỚI luống 4 không (qua luống 2 hoặc 3)? Nếu
CÓ, và từ luống 4 CŨNG có đường quay VỀ luống 1 — cả VƯỜN có "nối
liền" với nhau không?
::::

::::checkpoint{mastery=0.8}
::::
