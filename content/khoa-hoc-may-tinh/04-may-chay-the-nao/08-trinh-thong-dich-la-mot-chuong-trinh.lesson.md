---
id: khoa-hoc-may-tinh.may-chay-the-nao.trinh-thong-dich-la-mot-chuong-trinh
title: "Trình thông dịch chính nó cũng là một chương trình"
summary: "Vòng lấy-hiểu-làm bài 6-7 không tự nhiên có sẵn trong máy — nó LÀ CPython, một chương trình có thật (platform.python_implementation() trả về 'CPython' thật trên runtime này). 'Máy chạy Python' là 'một chương trình chạy một chương trình khác' — và chính chu trình lấy-hiểu-làm tự tay viết cũng chỉ là một hàm, gọi được như mọi hàm khác."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.interpreter-is-program]
requires: [may.instruction-pointer]
concepts: [may.interpreter-is-program]
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
Cái vòng lặp lấy-hiểu-làm ấy có tên thật, và tên đó không phải phép
màu — nó là một chương trình, giống hệt chương trình bạn viết.
::::

::::explain{#vong-lap-ay-la-ai}
Bài 6 dựng một vòng lặp lấy-hiểu-làm cho MỘT "chương trình" tự đặt tên
(`DAY`, `CONG`, `NHAN`, `IN`). Bài 7 cho nó thêm con trỏ, để nhảy được.
Nhưng vòng lặp ấy — bản thân nó — nằm Ở ĐÂU?

Câu trả lời: nó KHÔNG tự nhiên có sẵn trong máy tính, chờ sẵn từ
trước. Với bytecode Python thật (bài 1-5), vòng lấy-hiểu-làm đọc và
thi hành nó có tên riêng: **CPython** — chữ "C" vì nó được viết bằng
ngôn ngữ lập trình C, và "Python" vì việc nó làm là thi hành bytecode
Python. CPython là bản cài đặt (implementation) phổ biến nhất của
Python — và nó là một CHƯƠNG TRÌNH CÓ THẬT, với mã nguồn thật, được
biên dịch thành lệnh máy CPU thật, và CHẠY — giống hệt bất kỳ chương
trình nào khác bạn từng viết.

Nói cách khác: câu "máy chạy chương trình Python của bạn" thật ra có
nghĩa là "MỘT chương trình (CPython) đang chạy MỘT chương trình khác
(mã nguồn `.py` của bạn, đã dịch thành bytecode)". Không phải máy tính
tự nhiên "biết" Python. Có một chương trình trung gian đứng giữa, và
nó làm đúng việc bài 6-7 vừa mô phỏng: lấy lệnh, hiểu lệnh, làm lệnh,
lặp lại.

Python tự nói cho bạn biết điều này, nếu bạn hỏi đúng chỗ.
::::

::::example{#hoi-python-no-la-ai}
```python title=readonly
import sys
import platform

print(platform.python_implementation())
print(sys.implementation.name)
```

```text title=readonly
CPython
cpython
```

Hai dòng in ra ĐỀU gọi đúng tên: CPython. Đây không phải một chuỗi chữ
trang trí — nó là tên THẬT của chương trình đang thi hành đúng đoạn mã
này, ngay lúc này. Nếu Python được cài đặt bằng một chương trình KHÁC
(có — PyPy chẳng hạn, viết bằng chính Python để chạy nhanh hơn ở một
số việc), hai dòng trên sẽ in ra tên khác.
::::

::::predict{#vong-lap-chay-o-dau commitOnce}
Vòng lặp lấy-hiểu-làm mà bài 6-7 mô phỏng — thứ THẬT SỰ đọc và thi
hành bytecode `LOAD_FAST`, `BINARY_OP`,... của một hàm Python bạn viết
— chạy Ở ĐÂU?

:::opt{correct}
Bên trong CPython — một chương trình có thật, bản thân nó cũng được
biên dịch ra lệnh máy CPU và chạy, y hệt mọi chương trình khác
:::

:::opt
Ngay bên trong CPU, trực tiếp — CPU tự hiểu được bytecode Python,
không cần chương trình trung gian nào
::why
Gần đúng ở việc bạn nhớ đúng: CUỐI CÙNG, mọi thứ đều phải chạy thành
lệnh máy mà CPU hiểu được — đúng hướng.

Chỗ lệch: CPU chỉ hiểu ĐÚNG tập lệnh máy của riêng NÓ (khác hẳn với
`LOAD_FAST`, `BINARY_OP` — những cái tên đó là lệnh của CPython, không
phải lệnh CPU). CPU không đọc được bytecode Python trực tiếp. Cần một
chương trình trung gian — CPython — đọc bytecode đó rồi TỰ NÓ thực
hiện bằng những lệnh CPU thật mà nó gọi tới.
::
:::

:::opt
Ngay trong chính file mã nguồn `.py` — file đó tự biết cách chạy chính
mình
::why
Gần đúng ở việc bạn xác định đúng NƠI bắt đầu — mọi chuyện đúng là bắt
đầu từ file `.py` bạn viết.

Chỗ lệch: một file `.py` chỉ là VĂN BẢN — một chuỗi ký tự nằm trên
đĩa, giống hệt một file `.txt`. Văn bản không có khả năng "tự" làm gì
cả. Nó cần một chương trình KHÁC (CPython) đọc, dịch (bài 1), rồi thi
hành nó — đúng như bài 6 cho thấy `chuong_trinh = [...]` chỉ nằm yên
nếu không có vòng lặp nào đọc nó.
::
:::

:::opt
Không cố định — tuỳ máy nào chạy, máy đó tự quyết định cách hiểu
Python, không cần một chương trình trung gian cụ thể
::why
Gần đúng ở việc bạn để ý có SỰ KHÁC BIỆT giữa các máy — đúng, có nhiều
bản cài đặt Python khác nhau (CPython, PyPy,...) và chọn cài bản nào
là tuỳ máy.

Chỗ lệch: dù chọn bản nào, LUÔN CÓ một chương trình trung gian cụ thể,
đặt tên được (như dòng lệnh vừa in ra) — không có chuyện "tự hiểu" mà
không qua chương trình nào. `platform.python_implementation()` luôn
trả về một cái tên thật, không bao giờ để trống.
::
:::
::::

::::code{#trinh-thong-dich-la-mot-ham}
Bài 6 viết một vòng lấy-hiểu-làm rời rạc, nằm thẳng trong file. Bọc
đúng cái vòng lặp ấy vào bên trong một HÀM — để thấy tận tay: "trình
thông dịch" chẳng qua CŨNG chỉ là một hàm Python bình thường, gọi
được, dùng lại được, y hệt mọi hàm bạn từng viết.

`thi_hanh` bên dưới nhận một `chuong_trinh` bất kỳ (đúng định dạng
`DAY`/`CONG`/`IN` bài 6 dùng) và trả về danh sách mọi giá trị đã in.
Nó đã viết xong. Việc của bạn: gọi nó trên `chuong_trinh_1`.

```python title=starter
def thi_hanh(chuong_trinh):
    ngan_xep = []
    ket_qua_in = []
    for lenh in chuong_trinh:
        ten = lenh[0]
        if ten == "DAY":
            ngan_xep.append(lenh[1])
        elif ten == "CONG":
            b = ngan_xep.pop()
            a = ngan_xep.pop()
            ngan_xep.append(a + b)
        elif ten == "IN":
            ket_qua_in.append(ngan_xep[-1])
    return ket_qua_in

chuong_trinh_1 = [("DAY", 10), ("DAY", 20), ("CONG",), ("IN",)]
chuong_trinh_2 = [("DAY", 1), ("DAY", 2), ("CONG",), ("DAY", 100), ("CONG",), ("IN",)]

ket_qua_1 = ___                        # gọi thi_hanh trên chuong_trinh_1
ket_qua_2 = thi_hanh(chuong_trinh_2)

print(ket_qua_1)
print(ket_qua_2)
print(type(thi_hanh))
```

```python title=solution
def thi_hanh(chuong_trinh):
    ngan_xep = []
    ket_qua_in = []
    for lenh in chuong_trinh:
        ten = lenh[0]
        if ten == "DAY":
            ngan_xep.append(lenh[1])
        elif ten == "CONG":
            b = ngan_xep.pop()
            a = ngan_xep.pop()
            ngan_xep.append(a + b)
        elif ten == "IN":
            ket_qua_in.append(ngan_xep[-1])
    return ket_qua_in

chuong_trinh_1 = [("DAY", 10), ("DAY", 20), ("CONG",), ("IN",)]
chuong_trinh_2 = [("DAY", 1), ("DAY", 2), ("CONG",), ("DAY", 100), ("CONG",), ("IN",)]

ket_qua_1 = thi_hanh(chuong_trinh_1)
ket_qua_2 = thi_hanh(chuong_trinh_2)

print(ket_qua_1)
print(ket_qua_2)
print(type(thi_hanh))
```

```python title=test
assert ket_qua_1 == [30], f"chuong_trinh_1 cộng 10 + 20 rồi in — kết quả phải là [30], đang ra {ket_qua_1}"
assert ket_qua_2 == [103], f"chuong_trinh_2 cộng 1 + 2 rồi cộng tiếp 100 — kết quả phải là [103], đang ra {ket_qua_2}"
```

:::hints
- kind: attention
  body: thi_hanh đã viết xong hoàn chỉnh — không cần sửa bên trong nó. Chỗ trống chỉ cần GỌI nó, đúng như dòng ngay dưới đã gọi trên chuong_trinh_2.
- kind: strategy
  body: Gọi thi_hanh(chuong_trinh_1) — một hàm được gọi y hệt cách bạn gọi bất kỳ hàm nào khác, kể cả khi bên trong nó là cả một vòng lặp lấy-hiểu-làm.
- kind: one-line
  body: 'Chỗ trống là: thi_hanh(chuong_trinh_1)'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ gọi thi_hanh(chuong_trinh_1) — không gõ cứng kết quả [30], vì cả bài đang chứng minh trình thông dịch là một hàm GỌI ĐƯỢC, không phải một câu trả lời có sẵn
  requireAst:
  # uses-call target thi_hanh: khung đã gọi thi_hanh đúng 1 lần sẵn
  # (dòng ket_qua_2), lời giải đúng gọi thêm 1 lần ở chỗ trống -> tổng
  # 2. uses-name target chuong_trinh_1: đây là biến chỉ được GÁN
  # (chuong_trinh_1 = [...]), chưa được ĐỌC lần nào trong khung trước
  # khi điền -> baseline 0; lời giải đúng đọc nó đúng 1 lần khi truyền
  # làm đối số -> tổng 1.
  # ĐÃ THỬ BA CÁCH ĐIỀN HỤT (True/1/0): không cách nào gọi thi_hanh,
  # không cách nào đọc chuong_trinh_1 -> cả ba trượt static ngay, và
  # cũng trượt tests (ket_qua_1 thành True/1/0, không phải [30]).
  # Không có while/for nào bị điền hụt ở đây — hai lời gọi thi_hanh()
  # đều lặp trên list cố định độ dài 4 và 6, không phụ thuộc chỗ trống.
  - kind: uses-call, target: thi_hanh, min: 2
  - kind: uses-name, target: chuong_trinh_1, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\[30\\]\\n\\[103\\]\\n<class 'function'>\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`thi_hanh` chỉ là một hàm — gọi được, dùng lại được, y hệt mọi hàm
khác. CPython cũng vậy: một chương trình có tên thật, không phải phép
màu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy CPython là MỘT CHƯƠNG TRÌNH — nó ĐỌC bytecode và THI HÀNH
từng lệnh, mỗi lần một hàm Python được GỌI (bài 6-8). Nhưng bài 1 đã
cho thấy chuỗi bytecode ấy được DỊCH RA từ trước — trước khi hàm chạy
lần nào, `dis.dis()` đã in được nó ra rồi.

Vậy "dịch mã nguồn ra bytecode" và "đọc bytecode rồi thi hành nó" có
phải CÙNG MỘT bước, xảy ra mỗi lần gọi hàm — hay là HAI bước tách rời
hẳn nhau, mỗi bước xảy ra vào một LÚC khác nhau, với số LẦN khác nhau?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
