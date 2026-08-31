---
id: khoa-hoc-may-tinh.may-chay-the-nao.may-co-vong-lap-rieng
title: "Máy có MỘT vòng lặp riêng: lấy lệnh, hiểu lệnh, làm lệnh"
summary: "Danh sách lệnh dis.dis() in ra ở bài 1-5 không tự chạy — cần một vòng lặp KHÁC, ở tầng thấp hơn, cứ LẤY một lệnh, HIỂU nó là lệnh gì, LÀM nó, rồi lấy lệnh kế tiếp. Tự tay dựng một vòng lặp lấy-hiểu-làm (fetch-decode-execute) trên một chương trình nhỏ, tự đặt tên, để thấy đúng cơ chế đó."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.fetch-decode-execute]
requires: [may.branch-is-jump]
concepts: [may.fetch-decode-execute]
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
Danh sách lệnh không tự đứng dậy mà chạy. Có một vòng lặp khác đọc nó,
hiểu nó, rồi làm đúng việc nó ghi.
::::

::::explain{#danh-sach-khong-tu-chay}
Năm bài trước cho bạn xem đúng chuỗi lệnh máy thật — `LOAD_FAST`,
`BINARY_OP`, `JUMP_BACKWARD`, `POP_JUMP_IF_FALSE`,... — mà `dis.dis()`
in ra. Nhưng có một câu chưa ai trả lời: bản thân danh sách ấy CHỈ LÀ
MỘT DANH SÁCH. Nó nằm yên, như một tờ công thức nấu ăn dán trên tường.
Không có gì trong bản thân tờ giấy đó tự đứng dậy đi làm bếp.

Cần một CÁI GÌ ĐÓ đọc từng dòng công thức, hiểu dòng ấy bảo làm gì, làm
đúng việc đó, rồi chuyển sang dòng kế tiếp — lặp lại, cho tới khi hết
công thức. Máy tính có đúng một cơ chế như vậy, và nó cứ lặp không
ngừng, ba bước một:

1. **Lấy** (fetch) — lấy đúng MỘT lệnh kế tiếp trong danh sách.
2. **Hiểu** (decode) — xác định lệnh ấy là loại lệnh gì (`LOAD_FAST`?
   `BINARY_OP`? lệnh khác?).
3. **Làm** (execute) — thực hiện đúng việc lệnh ấy quy định.

Rồi quay lại bước 1, lấy lệnh KẾ TIẾP. Ba bước ấy, lặp không ngừng, có
tên riêng: **chu trình lấy-hiểu-làm** (tiếng Anh: fetch-decode-execute
cycle). Đây không phải một chi tiết vặt của Python — mọi máy tính, từ
điện thoại tới siêu máy tính, đều chạy đúng ba bước này ở tầng thấp
nhất của nó. Trình thông dịch Python thật (bài sau sẽ nói rõ nó là gì)
viết vòng lặp này bằng ngôn ngữ C, phức tạp hơn nhiều — nhưng Ý CỐT LÕI
giống hệt: một vòng lặp Ở TẦNG THẤP HƠN, không phải chính danh sách
lệnh, làm cho danh sách ấy "sống dậy".
::::

::::example{#may-tinh-nho-bang-tay}
Dưới đây là một "chương trình" nhỏ, tự đặt tên riêng cho dễ đọc — MỘT
DANH SÁCH các lệnh, mỗi lệnh là một `tuple`. Đứng riêng, danh sách này
không tính gì cả. Đoạn mã sau nó là một chu trình lấy-hiểu-làm TỰ TAY
viết, đọc và thi hành đúng danh sách đó.

```python title=readonly
chuong_trinh = [
    ("DAY", 3),
    ("DAY", 4),
    ("CONG",),
    ("IN",),
]

ngan_xep = []
for lenh in chuong_trinh:
    ten = lenh[0]                      # HIỂU: lệnh này tên gì
    if ten == "DAY":
        ngan_xep.append(lenh[1])       # LÀM: đẩy giá trị vào ngăn xếp
    elif ten == "CONG":
        b = ngan_xep.pop()
        a = ngan_xep.pop()
        ngan_xep.append(a + b)         # LÀM: lấy 2, cộng, đẩy 1 lại
    elif ten == "IN":
        print(ngan_xep[-1])            # LÀM: in giá trị đỉnh
```

```text title=readonly
7
```

`for lenh in chuong_trinh:` chính là bước **LẤY** — lấy đúng một phần
tử kế tiếp mỗi vòng. Dòng `ten = lenh[0]` là bước **HIỂU** — đọc xem
lệnh này thuộc loại nào. Khối `if/elif` là bước **LÀM** — mỗi nhánh chỉ
làm đúng MỘT việc nhỏ (bài 2 đã dạy điều này), y hệt cách `LOAD_FAST`/
`BINARY_OP` thao tác lên ngăn xếp tính toán (bài 3). `DAY 3` rồi `DAY
4` đẩy hai giá trị lên; `CONG` lấy hai ra, cộng, đẩy `7` lại; `IN` in
giá trị đang ở đỉnh.

Xoá hẳn dòng `for lenh in chuong_trinh:` và mọi dòng sau nó, chỉ để lại
`chuong_trinh = [...]` — chạy file đó, không có gì in ra cả. Danh sách
vẫn nằm y nguyên, không phép cộng nào xảy ra. Chính vòng lặp mới là thứ
làm danh sách ấy "chạy".
::::

::::predict{#danh-sach-rieng-mot-minh commitOnce}
Byte chỉ viết đúng dòng này rồi kết thúc chương trình — không viết
`for` nào cả:

```python
chuong_trinh = [
    ("DAY", 4),
    ("DAY", 5),
    ("CONG",),
    ("DAY", 2),
    ("NHAN",),
    ("IN",),
]
```

Chạy file này, điều gì xảy ra?

:::opt{correct}
Không có gì được in ra, và không phép tính nào xảy ra — `chuong_trinh`
chỉ là một `list` chứa các `tuple`, dữ liệu đứng yên, không hơn.
:::

:::opt
Python tự nhận ra đây là một chuỗi lệnh cần thi hành, vì các tuple bên
trong trông giống lệnh máy — nên nó tự chạy tuần tự và in kết quả.
::why
Gần đúng ở việc bạn để ý đúng: thứ tự các tuple bên trong
`chuong_trinh` CÓ Ý NGHĨA — đúng là thứ tự đó quan trọng, NẾU có ai đó
THI HÀNH nó.

Chỗ lệch: Python không có cơ chế nào "tự nhận ra" một `list` bất kỳ là
chương trình cần chạy, dù các phần tử bên trong trông giống lệnh cỡ
nào. `list` luôn chỉ là dữ liệu, dù bạn đặt tên biến gợi nhớ ra sao.
Phải có một vòng lặp lấy-hiểu-làm THẬT SỰ đọc nó, như ví dụ vừa xem,
thì mới có chuyện gì xảy ra.
::
:::

:::opt
Bị lỗi cú pháp, vì `list` không được phép chứa `tuple` mô tả lệnh máy
::why
Gần đúng ở việc bạn thận trọng với cú pháp — thái độ đúng khi gặp mã
lạ.

Chỗ lệch: đây là cú pháp Python hoàn toàn hợp lệ. Một `list` chứa các
`tuple` là dữ liệu bình thường, y hệt `[(1, 2), (3, 4)]` — không có
luật nào cấm đặt chữ `"DAY"`, `"CONG"` bên trong một tuple.
::
:::

:::opt
In ra `18`, vì Python tính trước biểu thức `(4 + 5) * 2` ngay khi tạo
danh sách
::why
Gần đúng ở việc bạn nhớ đúng: nếu có ai đó THI HÀNH đúng chuỗi lệnh này
(như ví dụ phần trước làm), kết quả cuối đúng là `18`.

Chỗ lệch: `chuong_trinh` không hề chứa biểu thức `(4 + 5) * 2` nào cả —
nó chỉ chứa các chuỗi và số nằm trong tuple, y hệt việc gõ `["DAY", 4]`
không có nghĩa là "gán biến DAY bằng 4". Không có phép cộng, phép nhân
nào NẰM TRONG cú pháp tạo `list` này để mà "tính trước".
::
:::
::::

::::code{#lenh-nhan}
Chương trình dưới tính `(4 + 5) * 2` — nhánh `"DAY"`, `"CONG"`, `"IN"`
đã có sẵn, đúng lối bạn vừa đọc. Còn thiếu đúng một lệnh: `"NHAN"`.
Viết nhánh ấy sao cho nó làm đúng MỘT việc, giống hệt cách nhánh
`"CONG"` đã làm — chỉ khác phép tính.

```python title=starter
chuong_trinh = [
    ("DAY", 4),
    ("DAY", 5),
    ("CONG",),
    ("DAY", 2),
    ("NHAN",),
    ("IN",),
]

ngan_xep = []
for lenh in chuong_trinh:
    ten = lenh[0]
    if ten == "DAY":
        ngan_xep.append(lenh[1])
    elif ten == "CONG":
        b = ngan_xep.pop()
        a = ngan_xep.pop()
        ngan_xep.append(a + b)
    elif ten == "NHAN":
        b = ngan_xep.pop()
        a = ngan_xep.pop()
        ngan_xep.append(___)           # đẩy kết quả NHÂN a với b
    elif ten == "IN":
        print(ngan_xep[-1])
```

```python title=solution
chuong_trinh = [
    ("DAY", 4),
    ("DAY", 5),
    ("CONG",),
    ("DAY", 2),
    ("NHAN",),
    ("IN",),
]

ngan_xep = []
for lenh in chuong_trinh:
    ten = lenh[0]
    if ten == "DAY":
        ngan_xep.append(lenh[1])
    elif ten == "CONG":
        b = ngan_xep.pop()
        a = ngan_xep.pop()
        ngan_xep.append(a + b)
    elif ten == "NHAN":
        b = ngan_xep.pop()
        a = ngan_xep.pop()
        ngan_xep.append(a * b)
    elif ten == "IN":
        print(ngan_xep[-1])
```

```python title=test
assert ngan_xep == [18], f"(4 + 5) * 2 phải ra 18, và IN không lấy gì ra khỏi ngăn xếp — ngăn xếp cuối phải là [18], đang là {ngan_xep}"
```

:::hints
- kind: attention
  body: Nhánh NHAN chỉ khác nhánh CONG đúng một chỗ — phép tính cuối cùng. Vẫn lấy ra đúng hai dòng .pop(), chỉ đổi phép cộng thành phép nhân.
- kind: strategy
  body: Nhìn đúng nhánh CONG ngay phía trên — nó lấy b rồi a ra, đẩy a + b vào. Nhánh NHAN làm hệt vậy, chỉ đổi a + b thành a * b.
- kind: one-line
  body: 'Chỗ trống là: a * b'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: nhánh NHAN phải THẬT SỰ nhân hai giá trị a và b lấy ra từ ngăn xếp — không gõ cứng kết quả, không để một câu không làm gì (True/1/0) thay vào chỗ đó
  requireAst:
  # Đếm thật trên toàn bộ starter + blank: tên "a" được ĐỌC đúng 1 lần
  # sẵn có trong nhánh CONG (a + b) trước khi điền gì; lời giải đúng đọc
  # thêm đúng 1 lần trong nhánh NHAN -> tổng 2. Tên "b" tương tự: 1 lần
  # sẵn có (CONG), lời giải đúng thêm 1 lần -> tổng 2. Một lời giải khác
  # đúng ngữ nghĩa `b * a` (đổi thứ tự, phép nhân giao hoán) vẫn đọc a
  # 1 lần và b 1 lần trong nhánh NHAN -> cùng đạt min:2 cho cả hai tên —
  # luật không ép thứ tự viết, chỉ ép có đọc đủ cả hai tên.
  # Toán tử "*" không xuất hiện ở đâu khác trong khung (CONG dùng "+"),
  # nên min:1 đã đủ chặn.
  # ĐÃ THỬ BA CÁCH ĐIỀN HỤT (True/1/0): không cách nào dùng "*", không
  # cách nào đọc thêm a hay b -> cả ba trượt static NGAY. Cả ba cũng
  # trượt luôn ở tests (ngan_xep cuối thành [True]/[1]/[0], không phải
  # [18]). Vòng lặp là `for` trên một list 6 phần tử cố định — không
  # cách điền nào (kể cả True/1/0) ảnh hưởng số vòng lặp, nên không có
  # nguy cơ chạy vô thời hạn.
  - kind: uses-name, target: a, min: 2
  - kind: uses-name, target: b, min: 2
  - kind: uses-operator, target: "*", min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^18\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhánh NHAN vừa chạy đúng ba bước lấy-hiểu-làm, y hệt nhánh CONG. Không
lệnh nào tự biết làm việc của mình — vòng `for` là thứ đọc và thi hành
từng lệnh.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vòng lặp bạn vừa viết là một `for lenh in chuong_trinh:` — nó luôn đi
qua từng lệnh THEO ĐÚNG THỨ TỰ ghi trong danh sách, từ đầu tới cuối,
không lệch một bước.

Nhưng bài 4 cho thấy `JUMP_BACKWARD` đưa vị trí đang chạy QUAY NGƯỢC
lại một lệnh ĐÃ đi qua rồi — không phải "đi tiếp theo thứ tự". Một vòng
`for` bình thường, cứ đi lệnh kế tiếp trong danh sách, có làm được việc
NHẢY NGƯỢC đó không? Nếu không, vòng lặp lấy-hiểu-làm THẬT cần thêm THỨ
GÌ để biết chính xác "đang đứng ở lệnh nào" — và có thể nhảy tới bất cứ
đâu, không chỉ đi tiếp theo thứ tự?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
