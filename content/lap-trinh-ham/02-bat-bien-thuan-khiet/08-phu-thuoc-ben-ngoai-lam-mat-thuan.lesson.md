---
id: lap-trinh-ham.bat-bien-thuan-khiet.phu-thuoc-ben-ngoai-lam-mat-thuan
title: "Phụ thuộc vào cái gì đó BÊN NGOÀI làm hàm mất thuần"
summary: "random.randint(1,100), datetime.now() — gọi hai lần với cùng đối số, có thể ra hai kết quả khác nhau. Một hàm phụ thuộc vào cái gì đó ngoài đối số của nó thì không còn đoán trước được, và rất khó test."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [fp.nondeterministic-impure]
requires: [fp.pure-fn-def]
concepts: [fp.nondeterministic-impure]
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
Bài trước để lại câu hỏi: một hàm không sửa gì cả, chỉ ĐỌC một thứ có
thể đổi — còn đoán trước được không? Byte gọi `random` ra làm chứng.
::::

::::explain{#doc-thu-khong-tat-dinh}
`tru_no` (bài trước) mất thuần vì nó GHI ra ngoài — thêm một dòng vào
`lich_su`. Có một kiểu mất thuần khác, ngược hướng: hàm không ghi gì cả,
nhưng nó ĐỌC một nguồn dữ liệu không nằm trong đối số, và nguồn đó có
thể đổi giữa hai lần gọi.

Hai nguồn quen thuộc nhất kiểu này:

- `random.randint(a, b)` — mỗi lần gọi rút một số NGẪU NHIÊN trong đoạn
  `[a, b]`, không đọc gì thêm ngoài `a` và `b`, nhưng kết quả không do
  `a`, `b` quyết định hoàn toàn.
- `datetime.now()` — không nhận đối số nào cả, kết quả phụ thuộc HOÀN
  TOÀN vào giờ hệ thống tại đúng khoảnh khắc gọi.

Một hàm gọi một trong hai thứ này bên trong nó thì KHÔNG THUẦN, dù thân
hàm không hề sửa list, dict, hay biến toàn cục nào — vì điều kiện 1 của
bài trước ("cùng đối số, luôn cùng kết quả") đã bị phá, không cần điều
kiện 2 (side-effect) tham gia.

Hệ quả không chỉ là lý thuyết: một hàm không tất định RẤT KHÓ TEST. Viết
`assert f() == 42` cho một hàm gọi `random.randint(...)` bên trong là vô
nghĩa — không có giá trị cố định nào để so, vì chính hàm không hứa hẹn
một giá trị cố định.
::::

::::example{#hai-lan-chay-hai-ket-qua}
Byte viết một hàm rút số may mắn, rồi chạy đúng cùng một đoạn mã hai lần
riêng biệt, không sửa gì giữa hai lần chạy.

```python title=readonly
import random

def rut_so_may_man():
    return random.randint(1, 100)

print(rut_so_may_man())
print(rut_so_may_man())
```

Lần chạy thứ nhất, hai dòng in ra là `14` rồi `3`. Chạy lại ĐÚNG đoạn mã
đó lần thứ hai, không đổi một chữ nào, hai dòng in ra lại là `32` rồi
`75`. Bốn con số, không con nào lặp lại con nào.

`rut_so_may_man` không nhận đối số nào — theo điều kiện 1, một hàm không
đối số mà thuần thì phải luôn ra ĐÚNG MỘT giá trị, mọi lần gọi. Ở đây nó
ra bốn giá trị khác nhau qua bốn lần gọi. Không có gì "sai" trong đoạn
mã — `random.randint` hoạt động đúng như nó phải hoạt động. Chỉ là hàm
BỌC quanh nó thừa hưởng đúng sự không đoán trước ấy.
::::

::::predict{#doan-hai-so-co-trung commitOnce}
`rut_so()` dưới đây rút một số nguyên từ `1` đến `10` — khoảng hẹp hơn
ví dụ trên, để việc trùng số dễ xảy ra hơn.

```python
import random

def rut_so():
    return random.randint(1, 10)

a = rut_so()
b = rut_so()
print(a == b)
```

**Trước khi bấm chạy**, điều gì bạn CHẮC CHẮN được về dòng in ra?

:::opt{correct}
Không thể chắc chắn dòng nào cả — có lần chạy in `True` (hai số trùng),
có lần chạy in `False` (hai số khác nhau), và không cách nào đoán trước
lần này sẽ ra sao
:::

:::opt
Luôn in `False`, vì hai lần gọi `random.randint` chắc chắn cho hai số
khác nhau
::why
Gần đúng ở trực giác "ngẫu nhiên thì chắc không trùng" — cảm giác đó dễ
hiểu.

Chỗ lệch: `random.randint(1, 10)` có thể rút TRÙNG một số ở hai lần gọi
liên tiếp — không có luật nào cấm điều đó, xác suất trùng ở đây còn khá
cao (khoảng một phần mười). "Ngẫu nhiên" không có nghĩa là "không bao
giờ lặp lại", nó có nghĩa là "không đoán trước được lần sau ra gì".
::
:::

:::opt
Luôn in `True`, vì `a` và `b` cùng gọi một hàm `rut_so()` với cùng kiểu
đối số (không đối số nào), nên phải ra cùng kết quả
::why
Gần đúng ở việc bạn đang áp dụng đúng định nghĩa hàm thuần — "cùng đối
số, cùng kết quả" là một luật có thật.

Chỗ lệch: luật đó chỉ áp dụng cho hàm THUẦN, và `rut_so` không thuần —
nó gọi `random.randint`, một nguồn không tất định. "Không có đối số
nào" không phải là "cùng một đối số" theo nghĩa làm hàm đoán trước được;
nó chỉ có nghĩa hàm không nhận input từ bên ngoài qua tham số, còn kết
quả vẫn phụ thuộc vào bộ sinh số ngẫu nhiên bên trong.
::
:::

:::opt
Máy dừng lại báo lỗi, vì gọi `random.randint` hai lần trong cùng một
chương trình
::why
Gần đúng ở việc bạn cẩn trọng với những gì gọi lặp lại — phản xạ đó
đúng trong nhiều tình huống khác (ví dụ mở cùng một file hai lần).

Chỗ lệch: không có giới hạn nào như vậy trong Python. Gọi
`random.randint` bao nhiêu lần tuỳ thích, mỗi lần đều hợp lệ và độc lập
với lần trước — không hàm nào bị "khoá" sau lần gọi đầu.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không đoán trước được — và đó chính xác là điều làm một hàm mất thuần.
Không phải vì nó sai, mà vì nó không hứa hẹn cùng một kết quả nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`rut_so_may_man` mất thuần vì nó ĐỌC một nguồn không tất định. Nhưng
thử hình dung một hàm khác: nó không đọc `random`, không đọc
`datetime.now()`, đối số nào cũng cho đúng một kết quả cố định — nhưng
giữa chừng, nó gọi `print(...)` để báo cho người dùng biết nó vừa chạy.

Kết quả TRẢ VỀ vẫn y hệt mỗi lần. Vậy hàm đó có thuần không?

Bài sau trả lời — và câu trả lời không phải điều bạn nghĩ.
::::

::::checkpoint{mastery=0.8}
::::
