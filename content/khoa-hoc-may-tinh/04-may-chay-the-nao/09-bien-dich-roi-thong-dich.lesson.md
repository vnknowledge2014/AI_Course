---
id: khoa-hoc-may-tinh.may-chay-the-nao.bien-dich-roi-thong-dich
title: "Python vừa BIÊN DỊCH vừa THÔNG DỊCH — không phải chỉ một"
summary: "'Python là ngôn ngữ thông dịch' chỉ đúng một nửa: BIÊN DỊCH (nguồn → bytecode, bài 1) xảy ra đúng MỘT LẦN — ham.__code__.co_code giữ nguyên qua nhiều lần gọi, đo thật trên Pyodide — rồi CPython (bài 8) THÔNG DỊCH bytecode đó MỖI LẦN hàm được gọi."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.compile-then-interpret]
requires: [may.interpreter-is-program]
concepts: [may.compile-then-interpret]
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
"Python là ngôn ngữ thông dịch" — câu quen thuộc đó chỉ đúng một nửa.
Nửa còn lại bạn đã thấy từ bài 1, chỉ chưa ai chỉ tên nó ra.
::::

::::explain{#hai-buoc-khong-phai-mot}
Bài 1 cho bạn xem `dis.dis(ham)` in ra một chuỗi lệnh máy — đó chính
là kết quả của bước **BIÊN DỊCH** (compile): dịch mã nguồn Python
(chữ, cho người đọc) thành bytecode (lệnh, cho CPython đọc). Bài 6-8
cho bạn xem một vòng lặp — vòng lấy-hiểu-làm bên trong CPython — đọc
đúng chuỗi lệnh ấy và LÀM từng lệnh. Đó là bước **THÔNG DỊCH**
(interpret).

Hai bước này không đối lập nhau, và cũng không phải MỘT bước gộp
chung. Chúng là HAI bước, nối tiếp, và — điều quan trọng nhất — chúng
xảy ra với SỐ LẦN khác hẳn nhau:

- **BIÊN DỊCH chỉ xảy ra ĐÚNG MỘT LẦN** cho một hàm — ngay khi Python
  đọc thấy định nghĩa `def`, trước khi hàm được gọi lần nào.
- **THÔNG DỊCH xảy ra MỖI LẦN hàm được GỌI** — gọi hàm 5 lần, CPython
  chạy lại vòng lấy-hiểu-làm 5 lần, trên ĐÚNG một chuỗi bytecode đã
  dịch từ trước, không dịch lại.

Mỗi hàm Python giữ kết quả bước biên dịch trong một thuộc tính riêng:
`ham.__code__` — một OBJECT chứa đúng chuỗi bytecode dưới dạng byte
thô (`co_code`). Gọi hàm bao nhiêu lần cũng không tạo `__code__` mới.
::::

::::example{#code-object-khong-doi}
```python title=readonly
def cong(a, b):
    return a + b

print(type(cong.__code__))

ma_truoc = cong.__code__.co_code
cong(1, 2)
cong(3, 4)
cong(5, 6)
ma_sau = cong.__code__.co_code

print(ma_truoc == ma_sau)
```

```text title=readonly
<class 'code'>
True
```

`cong.__code__` là một object kiểu `code` — chính là kết quả bước
BIÊN DỊCH, đã có sẵn ngay khi `def cong(...)` chạy xong, TRƯỚC khi
`cong` được gọi lần nào. `co_code` là chuỗi byte lệnh máy thô nằm bên
trong nó — đúng thứ `dis.dis()` đọc và in ra chữ cho bạn xem ở bài 1.

Ba lần gọi `cong(1, 2)`, `cong(3, 4)`, `cong(5, 6)` — mỗi lần với đối
số khác nhau, mỗi lần THẬT SỰ tính lại phép cộng — không hề tạo ra
`co_code` mới. `ma_truoc == ma_sau` là `True`: cùng một chuỗi byte,
trước và sau ba lần gọi. THÔNG DỊCH chạy ba lần. BIÊN DỊCH chỉ chạy
một lần, từ trước đó rất lâu.
::::

::::predict{#buoc-nao-may-lan commitOnce}
Hàm `cong` ở ví dụ trên được GỌI ba lần: `cong(1, 2)`, `cong(3, 4)`,
`cong(5, 6)`. Trong hai bước BIÊN DỊCH và THÔNG DỊCH, bước nào xảy ra
BA LẦN, bước nào chỉ xảy ra ĐÚNG MỘT LẦN?

:::opt{correct}
THÔNG DỊCH xảy ra ba lần — mỗi lời gọi chạy lại vòng lấy-hiểu-làm trên
bytecode. BIÊN DỊCH chỉ xảy ra một lần — `co_code` không đổi.
:::

:::opt
BIÊN DỊCH xảy ra ba lần — mỗi lần gọi hàm là dịch lại mã nguồn thành
bytecode một lần nữa. THÔNG DỊCH chỉ xảy ra một lần, ngay từ đầu.
::why
Gần đúng ở việc bạn nhận ra ĐÚNG là có một bước xảy ra ba lần, khớp
đúng ba lần gọi hàm.

Chỗ lệch: bạn gán nhầm TÊN cho bước đó. Chính THÔNG DỊCH mới là bước
chạy lại mỗi lần gọi — đọc và LÀM từng lệnh bytecode CÓ SẴN. BIÊN DỊCH
— tạo RA bytecode đó — mới là bước chỉ chạy một lần, và ví dụ vừa đo
được đúng bằng chứng: `co_code` giữ nguyên, không hề bị tạo lại.
::
:::

:::opt
Cả hai đều xảy ra ba lần — mỗi lần gọi hàm là dịch lại từ đầu rồi mới
chạy lại từ đầu
::why
Gần đúng ở việc bạn nhận đúng THÔNG DỊCH chạy ba lần — đúng phần đó.

Chỗ lệch nằm ở BIÊN DỊCH. Nếu biên dịch chạy lại mỗi lần gọi, giá trị
của `cong.__code__.co_code` phải khác nhau qua từng lần — nhưng đo
thật cho `ma_truoc == ma_sau` là `True`. Cùng một chuỗi byte suốt ba
lần gọi, không có bản dịch mới nào được tạo ra.
::
:::

:::opt
Cả hai chỉ xảy ra một lần — gọi hàm nhiều lần chỉ IN LẠI kết quả đã
tính từ trước, không chạy lại gì cả
::why
Gần đúng ở việc bạn nắm đúng: có một phần "chỉ một lần" ở đây — nhưng
gán nhầm cho CẢ HAI bước.

Chỗ lệch: nếu đúng vậy, `cong(1, 2)`, `cong(3, 4)`, `cong(5, 6)` phải
cho CÙNG một kết quả (kết quả "tính sẵn" từ lần đầu) — nhưng ba lời
gọi này dùng đối số khác hẳn nhau và trả về ba kết quả khác hẳn nhau
(`3`, `7`, `11`). Phải có một bước THẬT SỰ chạy lại, dùng đúng đối số
mới mỗi lần — đó là thông dịch.
::
:::
::::

::::code{#do-lai-tren-ham-khac}
Làm lại phép đo của ví dụ, trên một hàm khác — `binh_phuong`. Gọi hàm
này 500 lần rồi so `co_code` trước và sau.

```python title=starter
def binh_phuong(x):
    return x * x

ma_truoc = binh_phuong.__code__.co_code

for _ in range(500):
    binh_phuong(7)

ma_sau = ___                       # lấy lại co_code SAU 500 lần gọi

print(ma_truoc == ma_sau)
```

```python title=solution
def binh_phuong(x):
    return x * x

ma_truoc = binh_phuong.__code__.co_code

for _ in range(500):
    binh_phuong(7)

ma_sau = binh_phuong.__code__.co_code

print(ma_truoc == ma_sau)
```

```python title=test
assert ma_sau == ma_truoc, f"co_code phải giữ NGUYÊN sau 500 lần gọi — biên dịch chỉ chạy một lần, đang lệch: {ma_truoc!r} != {ma_sau!r}"
assert ma_sau == binh_phuong.__code__.co_code, "ma_sau phải đọc ĐÚNG co_code hiện tại của binh_phuong, không phải một giá trị chép tay từ biến khác"
```

:::hints
- kind: attention
  body: Chỉ một chỗ trống — lấy lại đúng thứ ma_truoc đã lấy ở trên, chỉ khác thời điểm đo (SAU vòng for thay vì TRƯỚC).
- kind: strategy
  body: Dùng đúng cú pháp binh_phuong.__code__.co_code — thuộc tính co_code nằm bên trong __code__ của chính hàm binh_phuong, y hệt dòng gán ma_truoc phía trên.
- kind: one-line
  body: 'Chỗ trống là: binh_phuong.__code__.co_code'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ đọc lại binh_phuong.__code__.co_code — không được chép ma_truoc sang (điều đó không ĐO gì cả, chỉ giả vờ đo), và không để một câu không làm gì (True/1/0)
  requireAst:
  # uses-name target binh_phuong: tên "binh_phuong" xuất hiện dưới dạng
  # Name(Load) đúng 2 lần sẵn có trong khung TRƯỚC khi điền — dòng gán
  # ma_truoc (đọc binh_phuong làm gốc của thuộc tính) và dòng
  # binh_phuong(7) trong vòng for (không tính "def binh_phuong(x):",
  # đó là FunctionDef.name chứ không phải Name node). Lời giải đúng đọc
  # thêm đúng 1 lần nữa ở chỗ trống -> tổng 3.
  # ĐÃ THỬ hai cách điền "trông như đúng nhưng rỗng":
  #   - ma_sau = ma_truoc: qua cả hai assert (vì co_code không đổi thật
  #     nên ma_truoc VẪN bằng co_code hiện tại) nhưng KHÔNG đọc lại
  #     binh_phuong.__code__.co_code — chỉ đọc "ma_truoc" (tên khác) ->
  #     số lần đọc "binh_phuong" vẫn dừng ở baseline 2, dưới min:3 ->
  #     TRƯỢT static. Đây chính là "cảnh thứ hai" bắt được ca mà tests
  #     một mình không bắt được.
  #   - True/1/0: không đọc binh_phuong lần nào thêm -> trượt static
  #     NGAY, và trượt luôn tests (ma_sau thành True/1/0, so với
  #     ma_truoc là bytes -> khác nhau).
  # Không có while/for nào bị điền hụt ảnh hưởng ở đây — vòng for lặp
  # đúng 500 lần cố định, không phụ thuộc chỗ trống.
  - kind: uses-name, target: binh_phuong, min: 3
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^True\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
500 lần gọi, một bản dịch duy nhất. THÔNG DỊCH chạy 500 lần trên đúng
`co_code` ấy — không dịch lại một byte nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Biên dịch một lần, thông dịch nhiều lần — nghĩa là MỖI LẦN gọi
`binh_phuong(7)`, CPython phải lặp lại TOÀN BỘ vòng lấy-hiểu-làm (bài
6-7) trên cùng bốn, năm lệnh bytecode ấy, từ đầu. 500 lần gọi là 500
lần lặp lại y hệt.

Việc lặp lại ấy có TỐN GÌ không — có đo được không? Và nếu có một ngôn
ngữ khác bỏ hẳn tầng thông dịch này, biên dịch THẲNG ra lệnh máy CPU
thật luôn (không qua bytecode trung gian nào), nó được lợi gì, và phải
đánh đổi gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
