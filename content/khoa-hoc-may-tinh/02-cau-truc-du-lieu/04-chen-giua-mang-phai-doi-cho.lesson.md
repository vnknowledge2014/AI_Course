---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.chen-giua-mang-phai-doi-cho
title: "Chèn vào giữa mảng, mọi phần tử sau phải dời chỗ"
summary: "Chèn một phần tử vào giữa mảng buộc mọi phần tử phía sau phải dời sang một bước, để nhường đúng một ô trống cho phần tử mới."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ds.array-insert-shift]
requires: [ds.dynamic-array, core.list, core.list-index, core.list-membership, core.fstring, core.variable, core.assignment]
concepts: [ds.array-insert-shift]
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
Chen một người vào giữa hàng đã xếp kín — ai đứng sau chỗ đó cũng phải
nhích ra sau một bước, dù chẳng ai đụng vào họ.
::::

::::explain{#chen-vao-giua}
Bài trước hỏi: nếu không thêm vào cuối, mà chèn một phần tử vào GIỮA một
mảng đã đầy kín hai đầu, chuyện gì phải xảy ra trước?

Nhớ lại luật của mảng (bài 1): các ô LIỀN KỀ, không hở khe nào. Muốn chèn
một giá trị mới vào đúng vị trí chỉ số `k`, ô đó phải TRỐNG trước đã. Nhưng
nếu mảng đã đầy kín, ô chỉ số `k` đang có chủ — một phần tử khác đang đứng
đó. Không có cách nào "chèn" vào một ô có chủ mà không đụng tới ai khác.

Cách duy nhất mở ra đúng một ô trống ở chỉ số `k`, mà vẫn giữ mảng liền kề
không hở khe, là: mọi phần tử ĐANG ĐỨNG từ chỉ số `k` trở đi phải dời sang
bên phải đúng MỘT bước — phần tử ở `k` dời sang `k+1`, phần tử ở `k+1` dời
sang `k+2`, cứ thế tới hết. Chỉ khi đó, chỉ số `k` mới thật sự trống, và
giá trị mới có chỗ đứng.

Phần tử mới không hề "chen" ai — chính XÁC nó là những phần tử PHÍA SAU vị
trí chèn tự dời đi để nhường chỗ. Và số phần tử phải dời không phụ thuộc
giá trị mới lớn hay nhỏ — nó phụ thuộc có BAO NHIÊU phần tử đang đứng phía
sau vị trí chèn. Chèn vào gần cuối một dãy dài chỉ dời vài phần tử; chèn
vào gần đầu một dãy dài dời gần hết cả dãy.
::::

::::example{#chen-mot-ten-vao-giua}
Bốn người đã đăng ký xếp hạng theo thứ tự, lưu trong một `list`. Byte cần
chèn thêm "Hoa" vào đúng vị trí chỉ số 1 — nghĩa là ngay sau "An".

```python title=readonly
xep_hang = ["An", "Bình", "Cúc", "Dũng"]
print(xep_hang.index("Cúc"))

xep_hang.insert(1, "Hoa")
print(xep_hang)
print(xep_hang.index("Cúc"))
```

```text title=readonly
2
['An', 'Hoa', 'Bình', 'Cúc', 'Dũng']
3
```

Trước khi chèn, "Cúc" đứng ở chỉ số 2. Sau khi `.insert(1, "Hoa")`, "Cúc"
đứng ở chỉ số 3 — TĂNG THÊM MỘT, dù không ai đụng vào tên "Cúc" cả. Điều
xảy ra thật sự: "Bình" dời từ chỉ số 1 sang 2, "Cúc" dời từ 2 sang 3,
"Dũng" dời từ 3 sang 4 — mọi phần tử từ vị trí chèn trở về sau đều dời
đúng một bước, để nhường chỉ số 1 cho "Hoa".

Chèn ở đầu một mảng dài — chỉ số 0 của một dãy một nghìn phần tử, chẳng
hạn — nghĩa là dời cả một nghìn phần tử sang phải một bước trước đã, rồi
mới đặt phần tử mới vào. Cái giá này vô hình khi bạn chỉ gõ
`list.insert(0, x)` — dòng lệnh trông ngắn gọn y hệt `.append`, nhưng bên
trong nó làm một việc khác hẳn.
::::

::::predict{#chen-vao-dau-day-dai commitOnce}
Năm người đã xếp hạng. Byte chèn "Việt" vào ngay chỉ số 0 — đầu dãy:

```python
xep_hang = ["An", "Bình", "Cúc", "Dũng", "Em"]
xep_hang.insert(0, "Việt")
print(xep_hang)
print(xep_hang.index("Em"))
```

**Trước khi chạy**, bạn đoán xem: có bao nhiêu phần tử trong NĂM người gốc
phải dời chỗ (đổi chỉ số) sau lệnh chèn này?

:::opt{correct}
Cả năm — An, Bình, Cúc, Dũng, Em đều tăng chỉ số thêm 1.
:::

:::opt
Chỉ một — đúng phần tử đứng ngay sau vị trí chèn.
::why
Gần đúng ở việc "An" — phần tử ngay sau vị trí chỉ số 0 — đúng là dời chỗ
đầu tiên trong dây chuyền đó.

Chỗ lệch là việc dời không dừng lại ở "An". Nó kéo theo TẤT CẢ phần tử
đứng sau "An" luôn, y hệt hiệu ứng domino — không riêng gì một phần tử
đứng ngay sau chỗ chèn.
::
:::

:::opt
Không phần tử nào — chỉ có "Việt" được thêm vào, còn lại giữ nguyên chỉ số.
::why
Gần đúng ở việc GIÁ TRỊ của năm người gốc không hề đổi — đúng, không ai
trong số họ bị sửa tên hay xoá đi.

Chỗ lệch là câu hỏi hỏi về CHỈ SỐ, không phải giá trị. Mảng liền kề (bài 1)
không có ô trống ở giữa để "Việt" tự nhét vào mà không đụng ai — mọi người
phía sau phải dời để nhường đúng chỉ số 0.
::
:::

:::opt
Chỉ "Em" — vì đó là phần tử đứng cuối cùng, xa nhất, nên chịu ảnh hưởng
lớn nhất.
::why
Gần đúng ở việc "Em" đúng là có dời chỗ — chỉ số của "Em" tăng từ 4 lên 5,
đúng như dòng in ra cho thấy.

Chỗ lệch là "Em" không phải người DUY NHẤT dời. An, Bình, Cúc, Dũng cũng
dời y hệt vậy — bạn chỉ đang nhìn đúng một phần tử trong cả năm.
::
:::
::::

::::code{#chen-nguoi-den-muon}
Một buổi hội thảo có bốn người đăng ký theo thứ tự: Lan, Mai, Nga, Oanh.
Một người đến muộn tên "Kim" cần được xếp vào đúng vị trí chỉ số 1 — ngay
sau Lan — vì một luật ưu tiên riêng.

Điền lệnh chèn đúng vào chỗ trống, rồi kiểm tra xem "Nga" đã dời chỗ đúng
như dự đoán chưa.

```python title=starter
danh_sach = ["Lan", "Mai", "Nga", "Oanh"]

chi_so_nga_truoc = danh_sach.index("Nga")

___                               # chèn "Kim" vào đúng chỉ số 1

chi_so_nga_sau = danh_sach.index("Nga")
so_phan_tu_da_doi_cho = chi_so_nga_sau - chi_so_nga_truoc

print(danh_sach)
print(f"Nga từ chỉ số {chi_so_nga_truoc} dời sang chỉ số {chi_so_nga_sau}")
```

```python title=solution
danh_sach = ["Lan", "Mai", "Nga", "Oanh"]

chi_so_nga_truoc = danh_sach.index("Nga")

danh_sach.insert(1, "Kim")

chi_so_nga_sau = danh_sach.index("Nga")
so_phan_tu_da_doi_cho = chi_so_nga_sau - chi_so_nga_truoc

print(danh_sach)
print(f"Nga từ chỉ số {chi_so_nga_truoc} dời sang chỉ số {chi_so_nga_sau}")
```

```python title=test
assert danh_sach == ["Lan", "Kim", "Mai", "Nga", "Oanh"], f"danh_sach phải đúng thứ tự Lan, Kim, Mai, Nga, Oanh sau khi chèn — đang ra {danh_sach}"
assert len(danh_sach) == 5, "danh_sach phải có đủ 5 người sau khi chèn thêm Kim"
assert chi_so_nga_truoc == 2, "trước khi chèn, Nga đứng ở chỉ số 2 — đừng sửa dòng tính chi_so_nga_truoc"
assert chi_so_nga_sau == 3, f"sau khi chèn Kim vào chỉ số 1, Nga phải dời sang chỉ số 3 — đang ra {chi_so_nga_sau}"
assert so_phan_tu_da_doi_cho == 1, "Nga chỉ dời đúng một bước, không hơn không kém"
```

:::hints
- kind: attention
  body: Bài yêu cầu chèn đúng "vào chỉ số 1" — không phải thêm vào cuối, không phải ghi đè lên chỗ đang có người. Chọn đúng công cụ đã dùng trong ví dụ phía trên.
- kind: strategy
  body: "list.insert(vi_tri, gia_tri) chèn gia_tri vào đúng vi_tri, dồn mọi phần tử từ đó trở đi sang phải một bước — đúng công cụ ví dụ vừa dùng để chèn 'Hoa'. Đừng dùng .append (nó chỉ thêm vào CUỐI, không phải vào giữa), và đừng gán trực tiếp danh_sach[1] = 'Kim' (đó là GHI ĐÈ, xoá mất người đang đứng ở đó, không phải chèn thêm)."
- kind: one-line
  body: 'Điền `danh_sach.insert(1, "Kim")` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ gọi danh_sach.insert(...) — bài này đang dạy đúng cơ chế của .insert() (mọi phần tử sau vị trí chèn tự dời), không phải chỉ tạo ra một danh_sach có đúng nội dung cuối cùng bằng cách gán lại nguyên khối
  requireAst:
  # Không có cổng này, `danh_sach = ["Lan", "Kim", "Mai", "Nga", "Oanh"]`
  # (gán đè cả danh sách bằng đúng kết quả cuối cùng, không hề chèn gì) chạy
  # qua đủ run/tests/output — chi_so_nga_sau tính lại bằng .index() vẫn ra
  # đúng 3, so_phan_tu_da_doi_cho vẫn ra đúng 1, vì bốn cái tên trong danh
  # sách kết quả là CỐ ĐỊNH, gõ thẳng ra cũng trùng khớp. Đếm thật bằng ast
  # trên lời giải: đúng 1 lời gọi `.insert(...)`, min:1 khớp chính xác — cách
  # viết khác dùng slice để dựng lại (`danh_sach[:1] + ["Kim"] + danh_sach[1:]`)
  # cũng trượt cổng này, đúng ý: bài này hẹp có chủ đích, chỉ dạy .insert().
  - kind: uses-call, target: insert, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\['Lan', 'Kim', 'Mai', 'Nga', 'Oanh'\\]\\nNga từ chỉ số 2 dời sang chỉ số 3\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Kim có chỗ đứng, và Nga dời đúng một bước — không hơn, không kém.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy CHÈN vào giữa buộc mọi phần tử phía sau phải dời sang bên
PHẢI, để MỞ một chỗ trống.

Vậy chuyện ngược lại thì sao? Nếu một người rút khỏi danh sách — bị XOÁ ở
đúng giữa dãy, không phải ở đầu hay cuối — cái LỖ mà họ để lại có được
phép tồn tại không? Mảng liền kề (bài 1) có cho phép một khe hở ở giữa
không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
