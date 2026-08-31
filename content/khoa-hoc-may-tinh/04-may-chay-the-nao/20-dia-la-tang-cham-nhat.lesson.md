---
id: khoa-hoc-may-tinh.may-chay-the-nao.dia-la-tang-cham-nhat
title: "Đĩa là tầng chậm nhất — vì sao mở file luôn chậm hơn đọc biến"
summary: "R1.T1.5 đã cho mở file, đọc/ghi — nhưng chưa nói vì sao nó CHẬM HƠN HẲN so với đọc một biến đã có trong bộ nhớ. Đĩa là tầng ngoài cùng, xa CPU nhất, và mọi phép đọc/ghi file đều phải đi qua toàn bộ chặng đường từ đĩa lên RAM rồi mới tới CPU — đo được thật, hàng trăm lần."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [may.disk-slowest]
requires: [may.cache-locality]
concepts: [may.disk-slowest]
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
RAM tắt là quên sạch. Có một chỗ khác, không quên — nhưng cái giá phải
trả là tốc độ.
::::

::::explain{#tang-ngoai-cung}
Bài trước so hai cách giữ dữ liệu — mảng và danh sách liên kết — nhưng
cả hai đều sống trong RAM. Và RAM có một đặc điểm chưa track này nói tới:
nó chỉ giữ được dữ liệu trong lúc máy còn CÓ ĐIỆN. Tắt máy, RAM trắng
trơn, không còn gì.

**Đĩa** (ổ SSD hoặc HDD) là một thiết bị VẬT LÝ khác hẳn, tách biệt khỏi
CPU và RAM — nó giữ dữ liệu dù mất điện, đúng lý do R1.T1.5 dạy bạn dùng
`open()` để lưu một file: bạn MUỐN dữ liệu đó còn sống sau khi chương
trình tắt.

Cái giá của việc "giữ được dù mất điện" là khoảng cách. Đĩa đứng ở tầng
NGOÀI CÙNG so với CPU — xa hơn cả RAM (bài 18). Đọc một biến đã có trong
RAM chỉ cần đi qua tối đa vài tầng bài này đã dạy: thanh ghi, bộ nhớ đệm,
RAM. Đọc một file thì khác hẳn: yêu cầu phải rời hẳn khỏi những tầng đó,
đi qua một hệ thống quản lý tệp riêng của hệ điều hành, chạm tới thiết bị
lưu trữ thật, rồi mang dữ liệu ngược lại — CẢ MỘT chặng đường dài hơn hẳn,
mỗi lần `open()`/`.read()`/`.write()` được gọi.

Đây chính là lý do `open()`, `.read()`, `.write()` (R1.T1.5) luôn được
dạy như những thao tác "nặng" — không phải vì cú pháp khó, mà vì mỗi lần
gọi chúng, chương trình phải rời khỏi những tầng nhanh nhất mà bài 16-19
vừa dạy, đi ra tận thiết bị lưu trữ.
::::

::::example{#do-that-bien-vs-file}
So sánh thời gian đọc một giá trị đã có sẵn trong biến, với thời gian mở
lại một file để đọc CHÍNH giá trị đó — lặp lại nhiều lần để thấy rõ:

```python title=readonly
import time

gia_tri = 12345678

def doc_bien(n):
    tong = 0
    for _ in range(n):
        tong += gia_tri
    return tong

with open("so_lieu.txt", "w") as f:
    f.write(str(gia_tri))

def doc_file(n):
    tong = 0
    for _ in range(n):
        with open("so_lieu.txt") as f:
            tong += int(f.read())
    return tong

LUOT = 2000
t0 = time.perf_counter()
doc_bien(LUOT)
t1 = time.perf_counter()

t2 = time.perf_counter()
doc_file(LUOT)
t3 = time.perf_counter()

print(f"đọc biến {LUOT} lần: {t1 - t0:.4f} giây")
print(f"đọc file {LUOT} lần: {t3 - t2:.4f} giây")
print(f"file chậm hơn biến khoảng {(t3 - t2) / (t1 - t0):.0f} lần")
```

```text title=readonly
đọc biến 2000 lần: 0.0002 giây
đọc file 2000 lần: 0.0837 giây
file chậm hơn biến khoảng 372 lần
```

Ngay cả trong một môi trường sandbox như Pyodide — nơi "đĩa" thật ra chỉ
là một vùng nhớ ảo hoá, KHÔNG phải một ổ SSD/HDD quay thật — việc mở file
vẫn phải đi qua một lớp hệ thống tệp riêng, tách biệt hẳn khỏi việc đọc
một biến. Con số chính xác dao động theo lần đo, nhưng luôn ở mức HÀNG
TRĂM LẦN, không phải một vài lần. Trên máy thật, với một ổ đĩa vật lý
thật, khoảng cách đó còn xa hơn nữa — cần thêm bước tìm đúng vị trí dữ
liệu nằm trên thiết bị (với HDD quay cơ học, bước đó càng tốn thời gian).
::::

::::predict{#vi-sao-file-cham commitOnce}
Hai đoạn mã dưới đây tính ra CÙNG một giá trị:

```python
# Đoạn A
gia_tri = 12345678
ket_qua = gia_tri + 1

# Đoạn B
with open("so.txt", "w") as f:
    f.write("12345678")
with open("so.txt") as f:
    ket_qua = int(f.read()) + 1
```

Đoạn B luôn chậm hơn Đoạn A rất nhiều, dù kết quả giống hệt nhau. Vì
sao?

:::opt{correct}
Đoạn B phải rời khỏi RAM, đi qua hệ thống quản lý tệp, chạm tới thiết bị
lưu trữ rồi quay lại — một chặng đường CPU/RAM không cần đi trong Đoạn A
:::

:::opt
Vì Đoạn B có nhiều dòng mã hơn, nên máy phải làm nhiều việc hơn về mặt
số lượng câu lệnh
::why
Gần đúng ở việc bạn đếm đúng: Đoạn B có nhiều DÒNG mã hơn thật.

Chỗ lệch: số DÒNG mã không phải lý do khoảng cách tốc độ lớn tới hàng
trăm lần. Một vài dòng Python biên dịch thành một số lượng lệnh bytecode
khiêm tốn (bài 1-2) — không đủ giải thích một khoảng cách lớn cỡ đó. Lý
do thật nằm ở LOẠI thao tác `open`/`.write`/`.read` phải làm, không phải
SỐ LƯỢNG dòng gọi chúng.
::
:::

:::opt
Vì chuỗi `"12345678"` phải được Python chuyển đổi qua lại giữa số và
chữ, còn Đoạn A giữ nguyên là số suốt
::why
Gần đúng ở việc bạn để ý đúng một chi tiết CÓ thật — Đoạn B đúng là có
bước `int(...)` chuyển chữ thành số, còn Đoạn A thì không.

Chỗ lệch: bước chuyển đổi số/chữ đó cực nhanh, chỉ tốn vài lệnh bytecode
— không thể giải thích khoảng cách hàng trăm lần. Phần tốn thời gian
thật nằm ở việc RỜI KHỎI RAM để tới đĩa, không phải việc đổi kiểu dữ
liệu.
::
:::

:::opt
Không có lý do rõ ràng — máy nào cũng khác nhau, không đoán trước được
::why
Gần đúng ở việc con số CHÍNH XÁC quả có khác nhau tuỳ máy — điều đó
đúng, ví dụ trên cũng nói rõ vậy.

Chỗ lệch: dù con số dao động, LÝ DO thì luôn giống nhau trên mọi máy —
Đoạn B buộc phải rời khỏi RAM, đi qua một hệ thống tệp riêng, chạm tới
thiết bị lưu trữ. Đây không phải sự ngẫu nhiên, mà là một chặng đường
vật lý CỐ ĐỊNH mà Đoạn A không hề cần đi.
::
:::
::::

::::code{#ghi-roi-doc-lai}
Byte muốn LƯU một câu vào đĩa, rồi ĐỌC LẠI để chắc chắn đúng câu đó vẫn
còn nguyên.

```python title=starter
cau = "RAM mất dữ liệu khi tắt máy, đĩa thì không"

with open("ghi_chu.txt", "w") as f:
    ___                              # ghi cau vào file

with open("ghi_chu.txt") as f:
    doc_lai = f.read()

print(doc_lai)
print(doc_lai == cau)
```

```python title=solution
cau = "RAM mất dữ liệu khi tắt máy, đĩa thì không"

with open("ghi_chu.txt", "w") as f:
    f.write(cau)

with open("ghi_chu.txt") as f:
    doc_lai = f.read()

print(doc_lai)
print(doc_lai == cau)
```

```python title=test
assert doc_lai == cau, f"câu đọc lại từ file phải trùng khớp câu đã ghi — đang ra {doc_lai!r}"
```

:::hints
- kind: attention
  body: Chỗ trống nằm bên trong khối "with open(..., 'w') as f:" — nó phải GHI, không phải ĐỌC.
- kind: strategy
  body: 'File đang mở ở chế độ ghi ("w"), giữ trong tên f. Ghi đúng biến cau vào đó bằng phương thức .write(): f.write(cau).'
- kind: one-line
  body: 'Chỗ trống là: f.write(cau)'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải thật sự gọi f.write(cau) — ghi đúng biến cau, không phải gõ lại nguyên văn câu chữ thành một chuỗi mới, vì bài này đang dạy việc GHI một giá trị đã có sẵn, không phải chép tay
  requireAst:
  # Đề bài nêu đích danh công cụ (".write()"), nên cổng hẹp đúng công cụ
  # đó hợp lệ theo Luật 3. Đếm thật trên lời giải: cau đọc 2 lần — trong
  # chỗ trống (f.write(cau)) và trong print(doc_lai == cau) có sẵn ở
  # khung. Gõ lại nguyên văn chuỗi thay vì dùng cau chỉ còn đọc được 1 lần
  # (chỉ dòng print có sẵn) — dưới min:2, chặn được.
  - kind: uses-call, target: write, min: 1
  - kind: uses-name, target: cau, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^RAM mất dữ liệu khi tắt máy, đĩa thì không\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ghi xong, đọc lại, đúng nguyên văn — dữ liệu đó giờ sống sót cả khi máy
tắt nguồn, cái giá là chặng đường xa hơn để lấy nó về.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Từ bài 16 tới giờ, track này đi qua bốn chặng: thanh ghi (bài 16), bộ
nhớ đệm — mà tự nó còn CHIA làm nhiều mức nữa (bài 17), RAM (bài 18), và
giờ là đĩa. Mỗi chặng lớn hơn chặng trước, và chậm hơn chặng trước.

Nếu xếp TẤT CẢ những mức đó cạnh nhau theo đúng thứ tự tốc độ — kể cả
việc tách bộ nhớ đệm ra thành các mức riêng — sẽ ra bao nhiêu tầng, và
quy luật gì lặp lại đều đặn ở MỖI bước chuyển từ tầng này sang tầng kế?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
