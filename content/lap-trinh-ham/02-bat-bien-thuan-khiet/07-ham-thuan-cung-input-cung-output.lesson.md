---
id: lap-trinh-ham.bat-bien-thuan-khiet.ham-thuan-cung-input-cung-output
title: "Hàm thuần: CÙNG input, CÙNG output, không side-effect"
summary: "Một hàm THUẦN luôn trả về CÙNG kết quả với CÙNG đối số, và không làm gì khác ngoài tính toán — bất biến (không sửa input) chỉ là MỘT PHẦN của điều kiện đó, không phải toàn bộ."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [fp.pure-fn-def]
requires: [fp.dataclasses-replace]
concepts: [fp.pure-fn-def]
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
Sáu bài vừa qua chỉ nói về DỮ LIỆU không đổi. Bài này nói về HÀM — một
hàm có thể không sửa gì cả, mà vẫn chưa đủ để gọi là "sạch".
::::

::::explain{#dinh-nghia-ham-thuan}
Bài trước để lại một câu hỏi: một hàm dùng đúng `frozen=True` và
`replace()`, không field nào bị sửa tại chỗ — nhưng giữa chừng nó gọi
`print()` để ghi log, hay đọc `random.random()` để tính một mã giảm giá.
Không field nào bị sửa. Vậy hàm đó có còn đáng tin không?

Câu trả lời cần một định nghĩa đầy đủ hơn "không sửa input". Một hàm gọi
là **thuần** (pure function) khi thoả CẢ HAI điều kiện sau, không thiếu
điều nào:

1. **Cùng đối số, luôn cùng kết quả.** Gọi hàm hai lần với đúng cùng giá
   trị đầu vào, kết quả trả về phải giống hệt nhau — không phụ thuộc vào
   giờ hệ thống, số ngẫu nhiên, hay bất kỳ thứ gì nằm ngoài đối số.
2. **Không làm gì khác ngoài TÍNH TOÁN.** Không in ra màn hình, không ghi
   file, không gọi mạng, không sửa dữ liệu tại chỗ (kể cả dữ liệu KHÔNG
   phải tham số của hàm — một biến toàn cục, một `list` module-level).

```python
def cong(a, b):
    return a + b
```

`cong(2, 3)` luôn ra `5`, không có lần nào khác. Hàm không đọc gì ngoài
`a` và `b`, không ghi gì ra ngoài giá trị trả về. Đây là một hàm thuần
trọn vẹn.

Bất biến — kỷ luật "không sửa dữ liệu tại chỗ" mà sáu bài vừa qua dạy —
là công cụ chính giúp giữ điều kiện 2. Nhưng điều kiện 2 rộng hơn "không
sửa list/dict của tham số": nó còn cấm sửa một biến toàn cục, và cấm mọi
tác động ra bên ngoài dù không sửa cấu trúc dữ liệu nào. Bài này chỉ đặt
định nghĩa; hai bài sau mổ xẻ riêng từng vế của điều kiện 2.
::::

::::example{#hai-ham-mot-thuan-mot-khong}
Byte viết hai hàm cạnh nhau — cùng nhận đối số, nhưng chỉ một hàm thuần.

```python title=readonly
def cong_diem(diem_cu, diem_them):
    return diem_cu + diem_them

lich_su_tru_no = []

def tru_no(no_con_lai, so_tien_tra):
    lich_su_tru_no.append(so_tien_tra)
    return no_con_lai - so_tien_tra

ket_qua_1 = cong_diem(10, 5)
ket_qua_2 = cong_diem(10, 5)
print(ket_qua_1 == ket_qua_2)

tru_no(100, 20)
tru_no(100, 20)
print(lich_su_tru_no)
```

```text title=readonly
True
[20, 20]
```

`cong_diem` thoả cả hai điều kiện: cùng `(10, 5)` luôn ra `15`, và không
đụng gì ra ngoài giá trị trả về — dòng `print` đầu xác nhận đúng vậy.

`tru_no` thì khác. Ngay cả GIÁ TRỊ TRẢ VỀ của nó vẫn đúng mỗi lần (gọi
`tru_no(100, 20)` hai lần đều ra `80`) — nhưng mỗi lần gọi, hàm còn ÂM
THẦM ghi thêm một dòng vào `lich_su_tru_no`, một danh sách nằm NGOÀI
tham số của nó. Sau hai lần gọi, danh sách đó dài ra hai phần tử. Đây
chính là điều kiện 2 bị vi phạm: hàm làm một việc khác ngoài tính toán,
dù việc đó không hề đổi kết quả trả về.
::::

::::predict{#doan-hai-dong-cuoi commitOnce}
Đoạn mã dưới đây định nghĩa lại đúng hai hàm ở trên, rồi gọi chúng theo
một trình tự khác.

**Trước khi bấm chạy**, bạn đoán hai dòng in ra gì?

```python
def cong_diem(diem_cu, diem_them):
    return diem_cu + diem_them

lich_su = []

def tru_no(no_con_lai, so_tien_tra):
    lich_su.append(so_tien_tra)
    return no_con_lai - so_tien_tra

ket_qua_1 = cong_diem(10, 5)
ket_qua_2 = cong_diem(10, 5)
print(ket_qua_1 == ket_qua_2)

tru_no(100, 20)
tru_no(100, 20)
print(lich_su)
```

:::opt{correct}
`True` rồi `[20, 20]`
:::

:::opt
`True` rồi `[20]`
::why
Gần đúng ở chỗ dòng đầu — `cong_diem(10, 5)` đúng là luôn ra `15`, nên
`True` là đúng cho dòng đó.

Chỗ lệch nằm ở `lich_su`. Mỗi lần `tru_no` chạy là một lần `.append`
CHẠY THẬT, không gộp chung với lần trước. Gọi `tru_no(100, 20)` hai lần
tạo ra hai lượt `.append(20)` riêng biệt, nên `lich_su` phải dài hai
phần tử: `[20, 20]`, không phải một.
::
:::

:::opt
`False` rồi `[20, 20]`
::why
Gần đúng ở chỗ `lich_su` — bạn đếm đúng số lần `.append` đã chạy.

Chỗ lệch nằm ở dòng đầu. `cong_diem` chỉ cộng hai số đã cho, không đọc
gì khác ngoài `diem_cu` và `diem_them` — với cùng `(10, 5)` cả hai lần
gọi, kết quả CHẮC CHẮN giống hệt nhau. Không có lý do nào để `ket_qua_1`
khác `ket_qua_2`.
::
:::

:::opt
Máy dừng lại báo lỗi, vì `tru_no` dùng `lich_su` mà không khai `global
lich_su` trong thân hàm
::why
Gần đúng ở việc bạn cảnh giác đúng chỗ — từ khoá `global` đúng là có
liên quan tới biến ở phạm vi ngoài hàm.

Chỗ lệch: `global` chỉ cần khi hàm GÁN LẠI cái tên đó (`lich_su = [...]`
bên trong hàm). Ở đây `tru_no` không gán lại `lich_su` — nó gọi
`.append(...)` trên danh sách đã có sẵn, tức SỬA TẠI CHỖ, không tạo tên
mới. Sửa tại chỗ một đối tượng đọc được từ phạm vi ngoài không cần khai
`global` gì cả — đây chính là lý do nó dễ lọt qua mắt người viết, và
cũng là lý do nó nguy hiểm.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`cong_diem` không chạm gì ra ngoài chính nó. `tru_no` thì có — dù kết
quả trả về vẫn đúng. Giờ bạn phân biệt được hai chuyện tưởng như một.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`tru_no` mất thuần vì nó GHI ra một chỗ nằm ngoài chính nó. Nhưng có một
kiểu mất thuần hoàn toàn khác — không ghi gì ra ngoài cả, không sửa
list, không sửa dict, không đụng biến toàn cục nào. Hàm chỉ ĐỌC một thứ
gì đó có thể đổi giữa hai lần gọi, dù đối số truyền vào y hệt nhau.

Một hàm như vậy có còn đoán trước được kết quả không?

Bài sau trả lời — bằng đúng một hàm bạn đã dùng quen: `random`.
::::

::::checkpoint{mastery=0.8}
::::
