---
id: toan.do-thi-modular-dai-so-truu-tuong.dinh-va-canh-la-mot-tap-hop
title: Đỉnh và cạnh là một tập hợp
summary: "G = (V, E) — đồ thị vô hướng LÀ một tập đỉnh V (T2.4 bài 1) cùng một quan hệ ĐỐI XỨNG E trên V (T2.4 bài 16 quan hệ, bài 20 đối xứng). Mỗi cạnh (a,b) đi kèm (b,a) — ống tưới nối HAI CHIỀU."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.graph-as-symmetric-relation]
requires: [math.relation-symmetric]
concepts: [math.do-thi, math.dinh, math.canh]
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
Byte có bốn luống, nối nhau bằng ống tưới. Sơ đồ NÀY có phải một
CẤU TRÚC toán học đã học rồi không?
::::

::::explain{#do-thi-la-gi}
Có. **`G = (V, E)`** — đồ thị VÔ HƯỚNG LÀ một tập đỉnh `V` (T2.4 bài
1, tập hợp) CÙNG một quan hệ ĐỐI XỨNG `E` trên `V` (T2.4 bài 16,
quan hệ LÀ tập con của `V×V`; bài 20, đối xứng: `(a,b)∈E` thì
`(b,a)∈E`). KHÔNG khái niệm MÁY mới — CHỈ đặt TÊN "đồ thị" lên một
quan hệ đối xứng ĐÃ biết dựng:

```python title=readonly
def la_doi_xung(quan_he):
    return all((b, a) in quan_he for (a, b) in quan_he)


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(la_doi_xung(E))
```

```text title=readonly
True
```

`E` GHI cả `(luong_1,luong_2)` LẪN `(luong_2,luong_1)` — ống tưới nối
HAI CHIỀU. `la_doi_xung(E)` (T2.4 bài 20) trả VỀ `True` — `E` LÀ một
quan hệ đối xứng, ĐÚNG định nghĩa đồ thị vô hướng.
::::

::::example{#quen-ghi-chieu-nguoc}
Nếu QUÊN ghi chiều NGƯỢC — `la_doi_xung` phát hiện NGAY:

```python title=readonly
def la_doi_xung(quan_he):
    return all((b, a) in quan_he for (a, b) in quan_he)


ong_tuoi_mot_chieu = {("luong_1", "luong_2"), ("luong_2", "luong_3")}

print(la_doi_xung(ong_tuoi_mot_chieu))
```

```text title=readonly
False
```

`(luong_1,luong_2)` CÓ trong tập, NHƯNG `(luong_2,luong_1)` THÌ
KHÔNG — `la_doi_xung` trả VỀ `False`. Đây CHƯA phải một đồ thị vô
hướng HỢP LỆ; nó THIẾU chiều ngược.
::::

::::predict{#doan-dong-doi-xung commitOnce}
Byte viết một hàm TỰ ĐỘNG thêm chiều NGƯỢC còn thiếu — `dong_doi_xung`
— rồi áp lên `ong_tuoi_mot_chieu`:

```python
def dong_doi_xung(canh):
    return canh | {(b, a) for (a, b) in canh}

ong_tuoi_mot_chieu = {("luong_1", "luong_2"), ("luong_2", "luong_3")}
E = dong_doi_xung(ong_tuoi_mot_chieu)
print(len(E))
```

Dòng cuối in ra gì?

:::opt{correct}
`4`
:::

:::opt
`2` — vì `dong_doi_xung` chỉ THÊM những cặp CÒN thiếu, và túi hai
cặp GỐC ĐÃ đủ mô tả đồ thị, nên "đóng" xong VẪN giữ nguyên hai cặp
::why
Gần đúng ở việc bạn nghĩ "đóng đối xứng" nghĩa LÀ "sửa lại cho ĐÚNG"
mà KHÔNG đổi kích thước — một trực giác hợp lý VỀ Ý nghĩa của
"đóng".

Chỗ lệch: `dong_doi_xung` THÊM cặp ngược (`(b,a)`) cho MỖI cặp gốc
`(a,b)` — hai cặp gốc (`luong_1→luong_2`, `luong_2→luong_3`) SINH ra
THÊM hai cặp ngược (`luong_2→luong_1`, `luong_3→luong_2`), TỔNG bốn
cặp Ở TẬP kết quả (`|`, phép hợp, T2.4 bài 8). KHÔNG PHẢI "sửa tại
chỗ" — nó CỘNG THÊM.
::
:::

:::opt
Máy báo lỗi khi chạy — `{(b, a) for (a, b) in canh}` cố GÁN LẠI biến
`a`, `b` vốn ĐÃ dùng LÀM tên tham số của hàm, Python từ chối GÁN
CHỒNG
::why
Gần đúng ở việc bạn để ý `a`, `b` xuất hiện Ở CẢ tham số hàm (KHÔNG
— thật ra tham số LÀ `canh`) LẪN trong comprehension — một quan sát
VỀ tên biến, dù không hoàn toàn khớp.

Chỗ lệch: `a`, `b` Ở TRONG `{(b, a) for (a, b) in canh}` LÀ biến CỤC
BỘ của CHÍNH comprehension đó (T2.4 nhiều bài đã dùng mẫu NÀY) —
KHÔNG hề trùng VỚI tham số `canh` của hàm. Biên dịch sạch, chạy
sạch.
::
:::
::::

::::code{#viet_dong_doi_xung}
Viết `dong_doi_xung(canh)` — trả về `canh` HỢP với tập các cặp NGƯỢC
của TỪNG cặp trong `canh` (bảo đảm kết quả LUÔN đối xứng).

```python title=starter
def dong_doi_xung(canh):
    return ___


ong_tuoi_mot_chieu = {("luong_1", "luong_2"), ("luong_2", "luong_3")}

print(len(dong_doi_xung(ong_tuoi_mot_chieu)))
```

```python title=solution
def dong_doi_xung(canh):
    return canh | {(b, a) for (a, b) in canh}


ong_tuoi_mot_chieu = {("luong_1", "luong_2"), ("luong_2", "luong_3")}

print(len(dong_doi_xung(ong_tuoi_mot_chieu)))
```

```python title=test
def la_doi_xung(quan_he):
    return all((b, a) in quan_he for (a, b) in quan_he)

assert dong_doi_xung(set()) == set(), "tap rong -- dong doi xung van rong"
da_doi_xung = {("luong_1", "luong_2"), ("luong_2", "luong_1")}
assert dong_doi_xung(da_doi_xung) == da_doi_xung, "da doi xung roi thi khong doi gi them"
assert la_doi_xung(dong_doi_xung({("a", "b")})) is True, "sau khi dong, luon doi xung"
```

:::hints
- kind: attention
  body: "Hop canh voi mot set comprehension: dao cap (a, b) thanh (b, a)."
- kind: strategy
  body: "canh | {(b, a) for (a, b) in canh}"
- kind: one-line
  body: "___ = canh | {(b, a) for (a, b) in canh}"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai hop canh voi tap cac cap dao nguoc, dung | va comprehension
  requireAst:
  - kind: comprehension, min: 1
  - kind: uses-name, target: canh, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^4\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đồ thị = quan hệ đối xứng. Nhưng "nối" thôi chưa nói được LUỐNG nào
BẬN rộn nhất — bao nhiêu ống chụm vào MỘT luống?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`(luong_1, luong_2) ∈ E`. Theo ĐỊNH nghĩa đối xứng (T2.4 bài 20),
`(luong_2, luong_1)` CŨNG phải Ở TRONG `E`. Đếm số cạnh NỐI TỚI một
luống cụ thể — luống 2 nối VỚI mấy luống KHÁC — có tên riêng cho con
số ĐÓ không?
::::

::::checkpoint{mastery=0.8}
::::
