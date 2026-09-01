---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.closure-ham-nho-moi-truong
title: "Closure — hàm bên trong VẪN THẤY biến của hàm ngoài, sau khi hàm ngoài đã return"
summary: "def lam_bang(k): return lambda x: x * k rồi nhan3 = lam_bang(3) — sau khi lam_bang(3) ĐÃ RETURN XONG, hàm được trả về VẪN nhớ k=3. Gọi CLOSURE."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [fp.closure-basics]
requires: [fp.review-hof]
concepts: [fp.closure-basics]
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
Cụm trước bạn đã viết hàm trả về hàm. Hôm nay Byte hỏi sâu hơn: hàm được
trả về đó "nhớ" bằng cách nào, khi hàm ngoài đã CHẠY XONG từ lâu?
::::

::::explain{#closure-la-gi}
```python
def lam_bang(k):
    return lambda x: x * k

nhan3 = lam_bang(3)
print(nhan3(5))
```

```text
15
```

`lam_bang(3)` CHẠY XONG, `return` ngay lập tức — theo trực giác thông
thường, `k = 3` (một biến CỤC BỘ của `lam_bang`) đáng lẽ phải "biến
mất" khi hàm kết thúc, giống mọi biến cục bộ khác. Nhưng `nhan3(5)` vẫn
tính ra `15` — nghĩa là `k` KHÔNG biến mất.

Hàm được trả về (`lambda x: x * k`) mang theo một liên kết tới `k`,
sống SUỐT thời gian hàm đó còn tồn tại — không chỉ trong lúc `lam_bang`
đang chạy. Hiện tượng này gọi là **closure**: hàm bên trong "đóng gói"
(close over) biến của hàm ngoài, giữ nó sống lâu hơn cả hàm ngoài.

```python
nhan5 = lam_bang(5)
print(nhan5(10))
print(nhan3(10))
```

```text
50
30
```

`nhan3` và `nhan5` là HAI closure ĐỘC LẬP — mỗi cái mang theo `k` của
riêng LẦN GỌI `lam_bang` sinh ra nó. Tạo `nhan5` không hề đụng tới `k`
mà `nhan3` đang giữ — `nhan3(10)` vẫn ra `30` (dùng `k=3`) như trước.
::::

::::example{#nhieu-closure-doc-lap}
Một vòng lặp tạo NHIỀU closure, mỗi cái nhớ một giá trị riêng:

```python title=readonly
def lam_bang(k):
    return lambda x: x * k

cac_ham = [lam_bang(k) for k in [2, 3, 5]]

for ham in cac_ham:
    print(ham(100))
```

```text title=readonly
200
300
500
```

Ba closure, sinh ra từ BA lần gọi `lam_bang` (với `k` lần lượt là `2`,
`3`, `5`) trong vòng lặp. Mỗi lần gọi tạo một closure MỚI, mang theo
`k` CỦA RIÊNG lần gọi đó — không có closure nào "ghi đè" lên closure
khác, dù cả ba đều sinh ra từ CÙNG một dòng `lambda x: x * k`.
::::

::::predict{#doan-hai-closure-doc-lap commitOnce}
```python
def lam_tru(n):
    return lambda x: x - n

tru_2 = lam_tru(2)
print(tru_2(10))

tru_7 = lam_tru(7)
print(tru_7(10))
print(tru_2(10))
```

Ba dòng in ra gì, theo đúng thứ tự?

:::opt{correct}
`8` rồi `3` rồi `8`
:::

:::opt
`8` rồi `3` rồi `3` — vì tạo `tru_7` sau đó đã "cập nhật" giá trị `n`
mà `tru_2` đang giữ
::why
Gần đúng ở việc bạn tính đúng hai kết quả đầu (`10-2=8`, `10-7=3`) — hai
phép trừ đó đúng.

Chỗ lệch: `tru_2` và `tru_7` là HAI closure hoàn toàn tách biệt, mỗi cái
"nhớ" `n` của riêng LẦN GỌI `lam_tru` sinh ra nó — `tru_2` mãi mãi nhớ
`n=2`, không hề bị `tru_7` (nhớ `n=7`) ảnh hưởng. `tru_2(10)` ở dòng
cuối vẫn ra `8`, y hệt lần gọi đầu.
::
:::

:::opt
Máy báo lỗi ở dòng `tru_7 = lam_tru(7)` — không tạo được closure THỨ HAI
từ CÙNG một hàm `lam_tru`
::why
Gần đúng ở việc bạn cảnh giác về việc tạo NHIỀU closure từ cùng một
"nhà máy" — một mối lo hợp lý nếu chưa chắc cơ chế này hoạt động ra sao.

Chỗ lệch: gọi `lam_tru` NHIỀU LẦN là chuyện HOÀN TOÀN BÌNH THƯỜNG — mỗi
lần gọi tạo một closure MỚI, độc lập, không giới hạn số lần. Ví dụ ở
trên (`cac_ham`) đã tạo tới BA closure từ cùng một hàm, không lỗi gì.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một hàm "nhớ" được biến của hàm sinh ra nó, dù hàm đó đã chạy xong từ
lâu — đó là closure. Mỗi lần gọi, một bản nhớ riêng, không đụng nhau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Closure "nhớ" được một giá trị — nhưng nó có SỬA được giá trị đó không,
hay chỉ ĐỌC? Track này đã dạy suốt rằng đọc và sửa là hai chuyện rất
khác nhau (T4.1).

Bài sau trả lời — bắt đầu bằng một closure CHỈ ĐỌC.
::::

::::checkpoint{mastery=0.8}
::::
