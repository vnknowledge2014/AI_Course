---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.xoa-giua-mang-cung-phai-doi-cho
title: "Xoá ở giữa cũng vậy — dồn chỗ trống lại"
summary: "Xoá một phần tử ở giữa để lại một lỗ; mảng liền kề không được phép có lỗ, nên mọi phần tử phía sau phải dồn lên lấp đúng chỗ trống đó."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ds.array-delete-shift]
requires: [ds.array-insert-shift, core.list, core.list-index, core.list-membership, core.fstring, core.variable, core.assignment]
concepts: [ds.array-delete-shift]
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
Một người rời khỏi hàng đang xếp giữa chừng — không ai được phép để lại
một khoảng trống giữa hàng cả.
::::

::::explain{#xoa-de-lai-lo}
Bài trước dựng một chiều: chèn vào giữa buộc mọi phần tử phía sau dời sang
PHẢI, để MỞ một ô trống. Bài này là gương đối xứng của nó.

Xoá một phần tử ở giữa mảng để lại đúng một LỖ — ô đó vừa mất chủ. Nhưng
luật của mảng (bài 1) không đổi: các ô phải LIỀN KỀ, không hở khe nào ở
giữa. Một mảng có lỗ ở giữa không còn là mảng liền kề nữa — nó là hai mảng
liền kề bị tách rời bởi một khoảng trống, và không công thức nào ở bài 2
còn tính đúng địa chỉ cho các ô phía sau lỗ đó nữa.

Vì vậy, ngay khi một ô bị xoá, mọi phần tử ĐỨNG SAU ô đó phải dồn lên bên
TRÁI đúng một bước — phần tử ở chỉ số `k+1` dồn về `k`, phần tử ở `k+2` dồn
về `k+1`, cứ thế tới hết dãy — để lấp đúng cái lỗ vừa mở ra. Chỉ khi đó
mảng mới liền kề trở lại.

Số phần tử phải dồn phụ thuộc đúng một điều: có bao nhiêu phần tử đang
đứng SAU vị trí bị xoá — không phụ thuộc phần tử bị xoá là gì. Xoá gần
cuối một dãy dài chỉ dồn vài phần tử; xoá gần đầu dồn gần hết cả dãy.
::::

::::example{#xoa-mot-ten-o-giua}
Năm người đã có mặt. Byte xoá "Kim" — người đứng ở chỉ số 1.

```python title=readonly
danh_sach = ["Lan", "Kim", "Mai", "Nga", "Oanh"]
print(danh_sach.index("Oanh"))

del danh_sach[1]
print(danh_sach)
print(danh_sach.index("Oanh"))
```

```text title=readonly
4
['Lan', 'Mai', 'Nga', 'Oanh']
3
```

Trước khi xoá, "Oanh" đứng ở chỉ số 4. Sau khi `del danh_sach[1]`, "Oanh"
đứng ở chỉ số 3 — GIẢM ĐI MỘT, dù không ai đụng vào tên "Oanh" cả. Điều
thật sự xảy ra: "Mai" dồn từ chỉ số 2 về 1, "Nga" dồn từ 3 về 2, "Oanh" dồn
từ 4 về 3 — mọi phần tử đứng sau chỗ vừa xoá đều dồn lên đúng một bước,
lấp đúng cái lỗ mà "Kim" để lại. Không có khoảng trống nào còn sót lại
giữa "Lan" và "Mai" — mảng vẫn liền kề, chỉ ngắn đi một ô.
::::

::::predict{#xoa-o-dau-day commitOnce}
Năm người đã có mặt. Byte xoá người đứng ĐẦU dãy — chỉ số 0:

```python
danh_sach = ["Lan", "Mai", "Nga", "Oanh", "Kim"]
del danh_sach[0]
print(danh_sach)
print(danh_sach.index("Kim"))
```

**Trước khi chạy**, bạn đoán xem: có bao nhiêu phần tử trong BỐN người còn
lại phải dồn chỗ (đổi chỉ số) sau lệnh xoá này?

:::opt{correct}
Cả bốn — Mai, Nga, Oanh, Kim đều giảm chỉ số đi 1.
:::

:::opt
Không phần tử nào — máy chỉ để lại một ô trống ở chỉ số 0, không đụng gì
tới phần còn lại.
::why
Gần đúng ở việc hình dung có một "lỗ" xuất hiện ngay chỗ "Lan" vừa đứng —
cảm giác đó không sai.

Chỗ lệch là mảng liền kề (bài 1) không được phép giữ lỗ trống ở giữa. Máy
PHẢI dồn mọi phần tử phía sau lên để lấp ngay lập tức, không để hở — đúng
như phần giải thích phía trên vừa nói.
::
:::

:::opt
Chỉ một — đúng phần tử đứng ngay sau chỗ vừa xoá.
::why
Gần đúng ở việc "Mai" — phần tử ngay sau "Lan" — đúng là phải dồn lên đầu
tiên trong dây chuyền đó.

Chỗ lệch là việc dồn không dừng lại ở "Mai". Nó kéo theo TẤT CẢ phần tử
đứng sau "Mai" luôn, y hệt hiệu ứng domino — không riêng gì một phần tử
đứng ngay sau chỗ xoá.
::
:::

:::opt
Chỉ "Kim" — vì đó là phần tử đứng cuối bảng.
::why
Gần đúng ở việc "Kim" đúng có đổi chỉ số — từ 4 xuống 3, đúng như dòng in
ra cho thấy.

Chỗ lệch là "Kim" không phải phần tử DUY NHẤT đổi. Mai, Nga, Oanh cũng đổi
y hệt vậy, chỉ là bạn đang nhìn đúng một phần tử trong bốn.
::
:::
::::

::::code{#nguoi-rut-lui}
Buổi hội thảo từ bài trước giờ có năm người: Lan, Kim, Mai, Nga, Oanh. Kim
báo rút lui vào phút chót — cần xoá đúng người ở chỉ số 1.

Điền lệnh xoá vào chỗ trống, rồi kiểm tra "Oanh" đã dồn chỗ đúng như dự
đoán chưa.

```python title=starter
danh_sach = ["Lan", "Kim", "Mai", "Nga", "Oanh"]

chi_so_oanh_truoc = danh_sach.index("Oanh")

___                                # "Kim" rút lui — xoá phần tử ở chỉ số 1

chi_so_oanh_sau = danh_sach.index("Oanh")
so_phan_tu_da_doi_cho = chi_so_oanh_truoc - chi_so_oanh_sau

print(danh_sach)
print(f"Oanh từ chỉ số {chi_so_oanh_truoc} dồn về chỉ số {chi_so_oanh_sau}")
```

```python title=solution
danh_sach = ["Lan", "Kim", "Mai", "Nga", "Oanh"]

chi_so_oanh_truoc = danh_sach.index("Oanh")

del danh_sach[1]

chi_so_oanh_sau = danh_sach.index("Oanh")
so_phan_tu_da_doi_cho = chi_so_oanh_truoc - chi_so_oanh_sau

print(danh_sach)
print(f"Oanh từ chỉ số {chi_so_oanh_truoc} dồn về chỉ số {chi_so_oanh_sau}")
```

```python title=test
assert danh_sach == ["Lan", "Mai", "Nga", "Oanh"], f"danh_sach phải đúng thứ tự Lan, Mai, Nga, Oanh sau khi xoá Kim — đang ra {danh_sach}"
assert len(danh_sach) == 4, "danh_sach phải còn đúng 4 người sau khi Kim rút lui"
assert "Kim" not in danh_sach, "Kim phải bị xoá hẳn khỏi danh_sach, không chỉ ghi đè bằng giá trị khác"
assert chi_so_oanh_truoc == 4, "trước khi xoá, Oanh đứng ở chỉ số 4 — đừng sửa dòng tính chi_so_oanh_truoc"
assert chi_so_oanh_sau == 3, f"sau khi xoá phần tử ở chỉ số 1, Oanh phải dồn về chỉ số 3 — đang ra {chi_so_oanh_sau}"
assert so_phan_tu_da_doi_cho == 1, "Oanh chỉ dồn đúng một bước, không hơn không kém"
```

:::hints
- kind: attention
  body: Bài yêu cầu xoá đúng "phần tử ở chỉ số 1" — không phải phần tử cuối cùng, không phải ghi đè bằng một giá trị rỗng. Chọn đúng công cụ đã dùng trong ví dụ phía trên.
- kind: strategy
  body: "del danh_sach[vi_tri] xoá hẳn phần tử ở vi_tri, dồn mọi phần tử từ đó trở đi sang trái một bước — đúng công cụ ví dụ vừa dùng để xoá 'Kim' khỏi chỉ số 1. Đừng dùng .pop() không tham số (nó xoá phần tử CUỐI, không phải chỉ số 1), và đừng gán danh_sach[1] = None (đó là GHI ĐÈ, để lại một ô mang giá trị None thay vì thật sự xoá và dồn chỗ)."
- kind: one-line
  body: "Điền `del danh_sach[1]` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ dùng danh_sach để xoá phần tử ở chỉ số 1 (ví dụ del danh_sach[1]) — bài này đang dạy đúng cơ chế xoá giữa mảng khiến phần tử sau dồn lên, không phải chỉ tạo ra một danh_sach có đúng nội dung cuối cùng bằng cách gán lại nguyên khối
  requireAst:
  # Không có cổng này, `danh_sach = ["Lan", "Mai", "Nga", "Oanh"]` (gán đè cả
  # danh sách bằng đúng kết quả cuối cùng, không hề xoá gì qua danh_sach) chạy
  # qua đủ run/tests/output — mọi assert đều tính lại từ danh_sach nên khớp
  # đúng như xoá thật. `del danh_sach[1]` không phải một lời gọi hàm nên
  # `uses-call` không bắt được nó; nhưng bên trong cú pháp `del x[i]`, cái tên
  # `x` vẫn ở ngữ cảnh ĐỌC (Load) — `ast.dump` xác nhận điều này — nên đếm
  # bằng `uses-name` bắt được. Đếm thật trên lời giải: `danh_sach` được đọc 4
  # lần (hai lần trong hai lời gọi .index(), một lần trong del, một lần trong
  # print) — cách viết khác dùng .pop(1) (không phải .pop() không tham số)
  # cũng đọc đúng 4 lần, nên min không ép theo riêng một cách viết. Bản hardcode
  # ở trên chỉ đọc 3 lần (thiếu đúng lần đọc để XOÁ), nên min:4 chặn đúng nó.
  - kind: uses-name, target: danh_sach, min: 4
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\['Lan', 'Mai', 'Nga', 'Oanh'\\]\\nOanh từ chỉ số 4 dồn về chỉ số 3\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Kim rời đi, và không một khe hở nào còn sót lại — Oanh dồn đúng một bước.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Năm bài vừa qua, bạn luôn làm việc với `list` — một dãy Ô CÁC BẠN TỰ CHỌN
đưa vào: số, tên người, bất kỳ thứ gì. Nhưng có một loại mảng bạn dùng mỗi
ngày mà chưa ai gọi đúng tên nó: mỗi khi gõ một câu tiếng Việt, bạn cũng
đang cầm một mảng — mảng của các KÝ TỰ.

Loại mảng đó có cho bạn `.insert()` hay `del` như `list` vừa rồi không? Hay
nó có một luật khoá cứng hơn hẳn?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
