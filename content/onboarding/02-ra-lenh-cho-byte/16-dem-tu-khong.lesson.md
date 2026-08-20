---
id: onboarding.ra-lenh-cho-byte.dem-tu-khong
title: Đếm từ 0
summary: Gọi riêng một món trong danh sách bằng chỗ đứng của nó, và chỗ đầu tiên mang số 0.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [core.list-index]
requires: [core.list, core.variable]
concepts: [core.danh-sach, core.chi-so]
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
Cả tấm bảng nằm trong một cái tên rồi. Giờ mình chỉ bạn cách gọi riêng một món.
::::

::::explain{#day-ghe-trong-quan}
Bài trước, ba món xếp chung vào một cái tên: `thuc_don` giữ *Phở tái*,
*Phở chín*, *Phở nạm* cùng một lúc. Nhưng khách vào quán không gọi cả tấm bảng.
Khách chỉ tay vào **một** món.

Hình dung ba chiếc ghế nhựa kê dọc tường, mỗi ghế có một món ngồi sẵn, xếp đúng
thứ tự bạn đã viết:

> ghế đầu: Phở tái · ghế kế: Phở chín · ghế cuối: Phở nạm

Bạn đứng ngay đầu dãy. Muốn lấy món nào, bạn nói với Byte đúng một câu: *đi bao
nhiêu bước từ chỗ tôi đang đứng.*

Byte không nhớ món nào tên gì nằm ở đâu. Nó chỉ đếm bước — và đếm rất chính xác.
::::

::::example{#hai-dau-ngoac-vuong}
Câu "đi mấy bước" viết bằng hai dấu ngoặc vuông, đặt dính ngay sau tên danh sách:

```python title=readonly
thuc_don = ["Phở tái", "Phở chín", "Phở nạm"]
print(thuc_don[0])
print(thuc_don[2])
```

Màn hình hiện ra:

```text title=readonly
Phở tái
Phở nạm
```

Đọc `thuc_don[0]` từ trái sang phải:

- `thuc_don` — dãy ghế cần nhìn vào.
- `[0]` — đi **0 bước** từ đầu dãy, tức là đứng nguyên tại ghế đầu tiên.

Con số nằm trong ngoặc vuông có tên riêng: **chỉ số** (tiếng Anh: `index`). Nó
không phải nội dung của món, cũng không phải tên món — nó là *chỗ đứng* của món
trong dãy.

Để ý luôn một điều: `print(thuc_don)` in ra cả dãy kèm ngoặc vuông và dấu nháy,
còn `print(thuc_don[0])` chỉ in ra nội dung một món, không nháy, không ngoặc.
::::

::::explain{#vi-sao-bat-dau-tu-khong}
Ngoài chợ, trong lớp, trên tờ thực đơn treo tường, món đầu tiên luôn là món
**số 1**. Máy đếm khác, và có một lý do rất cụ thể.

Chỉ số không trả lời câu hỏi *"món thứ mấy"*. Nó trả lời câu hỏi *"đi mấy bước
từ đầu dãy"*. Món đầu tiên nằm ngay dưới chân bạn, nên câu trả lời là 0. Món kế
tiếp cách một bước, nên là 1.

Từ đó ra một điều dùng được suốt đời:

> Dãy có ba món thì chỉ số chạy 0, 1, 2. Chỉ số cuối cùng luôn nhỏ hơn số món
> đúng một đơn vị.

Đây cũng là câu trả lời cho chuyện bạn đã gặp mà chưa được giải thích: vì sao
`range(3)` phát ra 0, 1, 2 chứ không phát ra 1, 2, 3. Cả hai chỗ đều đang đếm
cùng một thứ — số bước tính từ đầu — nên chúng khớp nhau: `range(3)` sinh ra
đúng những chỗ đứng có thật của một dãy ba món.

Nếu bạn xin một chỗ không có ghế — viết `thuc_don[3]` khi dãy chỉ có ba món —
Byte dừng lại và nói `list index out of range`, nghĩa là *chỉ số vượt ra ngoài
dãy*. Gặp dòng ấy thì đếm lại số món trên bảng: gần như lần nào cũng là chuyện
quên rằng ghế cuối mang số 2 chứ không phải số 3.
::::

::::predict{#doan-mon-so-mot commitOnce}
Vẫn tấm bảng ấy. **Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
thuc_don = ["Phở tái", "Phở chín", "Phở nạm"]
print(thuc_don[1])
```

:::opt{correct}
Phở chín
:::

:::opt
Phở tái
::why
Gần đúng ở chỗ bạn đang đếm theo đúng cách cả nước vẫn đếm: món số 1 là món đầu
tiên. Trên mọi tấm bảng treo tường, cách đếm ấy không sai chỗ nào.

Chỗ lệch nằm ở câu hỏi mà máy đang hỏi. `[1]` không hỏi "món thứ mấy" mà hỏi
"đi mấy bước từ đầu dãy". Đi 1 bước là bạn đã rời khỏi *Phở tái* và dừng lại ở
*Phở chín*.

Muốn lấy *Phở tái* thì đứng yên, đi 0 bước: `thuc_don[0]`.
::
:::

:::opt
'Phở chín' — có kèm hai dấu nháy
::why
Gần đúng, và gần đúng ở phần khó nhất: bạn chọn trúng món rồi. Chỉ còn phần hình
thức.

Dấu nháy xuất hiện khi bạn in **cả dãy**, vì lúc đó máy phải vẽ ranh giới giữa
các ô cho bạn nhìn. Còn ở đây bạn lấy ra đúng một món và in nội dung của nó, nên
màn hình chỉ hiện `Phở chín`.
::
:::

:::opt
Máy in ra con số 1
::why
Gần đúng ở chỗ bạn nhìn thấy một con số và biết máy làm việc được với số — điều
này bạn nắm chắc từ mấy bài trước.

Chỗ lệch là con số ấy không đứng một mình. Nó nằm trong ngoặc vuông, dính ngay
sau `thuc_don`. Dính như vậy nghĩa là "lấy món ở chỗ số 1 của dãy này", chứ
không phải "đây là số 1".

Muốn in ra chính con số thì viết `print(1)` — không tên danh sách, không ngoặc
vuông.
::
:::
::::

::::code{#lay-ban-cuoi-so}
Quán ghi sổ những bàn khách đã đặt trước, theo đúng thứ tự gọi điện tới.

Chủ quán hỏi: *bàn nào đặt sau cùng?* Hãy điền chỉ số để Byte in ra ô cuối sổ.

```python title=starter
ban_da_dat = ["Bàn 2", "Bàn 5", "Bàn 9"]
print(ban_da_dat[___])
```

```python title=solution
ban_da_dat = ["Bàn 2", "Bàn 5", "Bàn 9"]
print(ban_da_dat[2])
```

```python title=test
# Chấm bằng OUTPUT: người học chưa biết hàm nên chưa assert được gì.
# Khối này chỉ khẳng định chương trình chạy được tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm giữa hai dấu ngoặc vuông. Chỗ đó không nhận tên bàn — nó nhận một con số.
- kind: strategy
  body: Sổ có ba ô. Đứng ở đầu sổ rồi đếm bước - ô đầu là 0 bước, ô kế là 1 bước. Ô cuối cùng là mấy bước?
- kind: one-line
  body: Viết `2` vào chỗ trống, thành `print(ban_da_dat[2])`.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: Bàn 9
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bạn vừa chỉ đúng một ghế trong dãy. Byte không đoán món — nó đếm bước.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bảng hôm nay có ba món, nên gõ ba dòng `print` cũng còn chịu được. Nhưng quán
bên cạnh treo bảng hai mươi món. Hai mươi dòng `print`, mỗi dòng đổi đúng một
con số trong ngoặc vuông — rồi hôm sau quán bỏ bớt một món, bạn ngồi sửa lại số
của mọi dòng phía sau.

Trước đó bạn đã học cách bảo máy **làm lại một việc nhiều lần**, và học rằng cái
tên trong vòng lặp mang một giá trị khác nhau ở mỗi lượt. Bây giờ bạn có thêm
một dãy món xếp sẵn theo thứ tự, và vừa biết rằng chỗ đứng của chúng đánh số
0, 1, 2 — đúng những con số `range` phát ra.

Hai thứ ấy nối vào nhau được không? Nối thì nối ở chỗ nào?

Bài sau trả lời, và câu trả lời làm con số trong ngoặc vuông gần như biến mất
khỏi code của bạn.
::::

::::checkpoint{mastery=0.8}
::::
