---
id: lap-trinh-ham.bat-bien-thuan-khiet.in-ra-cung-la-side-effect
title: "print() cũng là side-effect — không chỉ đổi biến ngoài"
summary: "Side-effect không chỉ là sửa dữ liệu tại chỗ — bất kỳ tác động nào ra thế giới bên ngoài lời gọi hàm (in ra màn hình, ghi file, gọi mạng) đều là side-effect, kể cả khi kết quả trả về vẫn y hệt mỗi lần gọi."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [fp.io-is-side-effect]
requires: [fp.nondeterministic-impure]
concepts: [fp.io-is-side-effect]
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
Kết quả trả về đúng y hệt mỗi lần. Không list nào bị sửa. Không biến
toàn cục nào bị đụng. Byte vẫn nói: hàm này KHÔNG thuần.
::::

::::explain{#khong-sua-du-lieu-khac-khong-thuan}
Bài 7 đặt hai điều kiện cho một hàm thuần: (1) cùng đối số luôn cùng kết
quả, (2) không làm gì khác ngoài TÍNH TOÁN. Sáu bài đầu track này —
`tuple`, `frozen=True`, `MappingProxyType` sắp tới — đều xoay quanh MỘT
CÁCH để giữ điều kiện 2: đừng sửa dữ liệu tại chỗ. Nhưng điều kiện 2 nói
rộng hơn thế nhiều: "không làm gì khác ngoài tính toán", không riêng gì
"không sửa dữ liệu".

**In ra màn hình, ghi file, gọi mạng** — không cái nào trong ba việc đó
SỬA một `list` hay `dict` nào cả. Không có `.append`, không có gán vào
ô, không `global`. Nhưng cả ba đều có một điểm chung: chúng TÁC ĐỘNG ra
thế giới bên ngoài phạm vi của lời gọi hàm — màn hình đổi nội dung, file
trên đĩa đổi nội dung, một máy chủ ở xa nhận được một gói tin. Tác động
đó gọi là **side-effect** (tác dụng phụ): bất cứ điều gì một hàm làm,
ngoài việc tính ra giá trị trả về của nó.

Một hàm chỉ `print(...)` một dòng thông báo, không hề sửa list hay dict
nào, vẫn KHÔNG THUẦN — vì gọi nó hai lần nghĩa là màn hình bị in HAI
LẦN, một tác động lặp lại có thể không ai mong muốn nếu chương trình gọi
nhầm hàm ấy hai lần liên tiếp. "Không sửa dữ liệu" và "không thuần" là
hai điều KHÁC NHAU — hàm dưới đây chứng minh đúng điều đó.
::::

::::example{#chao-khong-sua-gi-van-khong-thuan}
Byte viết một hàm chào, gọi nó hai lần với đúng cùng một cái tên.

```python title=readonly
def chao(ten):
    print(f"Chào {ten}!")
    return f"Đã chào {ten}"

ket_qua_1 = chao("Lan")
ket_qua_2 = chao("Lan")

print(ket_qua_1 == ket_qua_2)
```

```text title=readonly
Chào Lan!
Chào Lan!
True
```

Dòng cuối là `True` — đúng điều kiện 1, cùng đối số `"Lan"` cả hai lần,
kết quả trả về giống hệt nhau. Không `list` nào bị sửa, không biến toàn
cục nào bị đụng tới — `chao` không cầm giữ bất kỳ dữ liệu nào giữa hai
lần gọi để mà sửa.

Nhưng nhìn lên hai dòng đầu: "Chào Lan!" xuất hiện HAI LẦN trên màn
hình, dù chỉ có một người tên Lan cần được chào. Đó là điều kiện 2 bị
phá — mỗi lần gọi `chao` là một lần TÁC ĐỘNG THẬT ra màn hình, một tác
động không nằm trong giá trị trả về và không thể "hoàn tác" chỉ bằng
cách nhìn vào kết quả hàm trả về.
::::

::::predict{#doan-so-lan-in commitOnce}
Đúng hàm `chao` ở trên, gọi lại theo một trình tự khác — ba lần thay vì
hai, và không phải lần nào cũng cùng một tên.

**Trước khi bấm chạy**, bạn đoán có bao nhiêu dòng "Chào ...!" xuất hiện
trên màn hình?

```python
def chao(ten):
    print(f"Chào {ten}!")
    return f"Đã chào {ten}"

chao("Lan")
chao("Lan")
chao("Mai")
```

:::opt{correct}
Ba dòng "Chào ...!" — mỗi lần GỌI hàm là một lần `print` chạy, bất kể
đối số truyền vào có trùng lần trước hay không
:::

:::opt
Hai dòng "Chào ...!" — hai lần gọi `chao("Lan")` trùng đối số nên chỉ
tính là MỘT tác động, cộng thêm dòng của `chao("Mai")`
::why
Gần đúng ở việc bạn để ý đúng chỗ hai lời gọi đầu dùng CHUNG đối số
`"Lan"` — quan sát đó không sai.

Chỗ lệch: Python không "gộp" hai lần gọi hàm có cùng đối số thành một.
`print(...)` là một CÂU LỆNH chạy mỗi khi dòng chứa nó được thực thi —
gọi `chao("Lan")` hai lần là chạy thân hàm `chao` hai lần riêng biệt,
tức hai lần `print` riêng biệt, không quan tâm đối số có trùng hay
không.
::
:::

:::opt
Một dòng "Chào ...!" — chỉ có `chao("Mai")` mới thật sự in, vì hai lời
gọi `chao("Lan")` không lưu kết quả vào biến nên bị Python bỏ qua
::why
Gần đúng ở việc bạn nhận ra hai lời gọi đầu không được GÁN vào tên nào
— `chao("Lan")` đứng một mình, không có `ket_qua = ...` phía trước.

Chỗ lệch: không lưu kết quả trả về không có nghĩa là hàm không CHẠY.
Python vẫn thực thi trọn vẹn thân hàm — bao gồm cả dòng `print` — dù
kết quả trả về sau đó bị bỏ đi vì không ai giữ lại. "Không dùng kết
quả" và "không gọi hàm" là hai chuyện khác nhau.
::
:::

:::opt
Không có dòng "Chào ...!" nào, vì `chao` không được gán vào biến nào cả
nên Python coi đây chỉ là khai báo, không phải lời gọi thật
::why
Gần đúng ở việc bạn cảnh giác với cú pháp "gọi mà không gán" — nhìn
thoáng qua nó có vẻ thiếu một phần.

Chỗ lệch: `chao("Lan")` — có tên hàm, có dấu ngoặc, có đối số bên trong
— ĐÃ là một lời gọi hàm trọn vẹn, hoàn chỉnh về cú pháp. Việc gán kết
quả vào một biến (`x = chao("Lan")`) là một bước RIÊNG, xảy ra sau khi
hàm đã chạy xong, không phải điều kiện để hàm được gọi.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba lần gọi, ba lần in — không phụ thuộc đối số có trùng nhau hay không.
Side-effect không hỏi "dữ liệu có đổi không", nó chỉ hỏi "có tác động ra
ngoài không".
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`chao` mất thuần vì nó GỌI `print(...)` ngay bên trong thân hàm — không
có cách nào tách rời "tính năng chào" khỏi "tác động in ra màn hình" một
khi cả hai đã trộn chung vào một hàm. Nhưng nếu một hàm THẬT SỰ cần biết
giờ hiện tại để hoạt động đúng — ví dụ chào theo buổi sáng/trưa/tối — nó
có buộc phải tự gọi `datetime.now()` bên trong, và vì thế buộc phải mất
thuần, hay có cách nào khác?

Bài sau chỉ ra lối thoát.
::::

::::checkpoint{mastery=0.8}
::::
