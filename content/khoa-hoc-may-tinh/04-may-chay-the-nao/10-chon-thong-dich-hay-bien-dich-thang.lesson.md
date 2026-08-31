---
id: khoa-hoc-may-tinh.may-chay-the-nao.chon-thong-dich-hay-bien-dich-thang
title: "Chọn thông dịch hay biên dịch thẳng — đánh đổi gì"
summary: "Bài chốt cụm: đo THẬT bằng time.perf_counter() cho thấy nhiều lần gọi hàm hơn tốn nhiều thời gian thật hơn — cái giá của việc CPython phải thông dịch lại bytecode mỗi lần gọi. Ngôn ngữ biên dịch thẳng ra lệnh máy CPU bỏ được tầng đó, nhanh hơn, nhưng phải biên dịch lại riêng cho từng loại máy; Python đổi tốc độ lấy sự tiện — một bản bytecode chạy được ở bất cứ đâu có CPython."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.interpret-vs-compile-tradeoff]
requires: [may.compile-then-interpret]
concepts: [may.interpret-vs-compile-tradeoff]
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
Không có lựa chọn nào miễn phí cả. Bài này đo đúng cái giá phải trả —
và cái được đổi lại.
::::

::::explain{#cai-gia-cua-tang-trung-gian}
Bài 9 chốt: BIÊN DỊCH một lần, THÔNG DỊCH mỗi lần gọi. Vòng lấy-hiểu-
làm (bài 6-7), nằm bên trong CPython (bài 8), phải chạy lại từ đầu MỖI
LẦN một hàm được gọi — dù bytecode không đổi một byte nào.

Việc lặp lại đó không miễn phí. Mỗi vòng của chu trình lấy-hiểu-làm là
công việc THẬT: đọc một lệnh, xác định nó là loại gì, rồi làm — ba
bước tốn thời gian CPU thật, đo được bằng đồng hồ thật.

Có một cách khác để tránh hẳn cái giá này: một ngôn ngữ **biên dịch
thẳng** (ví dụ C, Rust) không dừng lại ở bytecode trung gian như
Python. Nó biên dịch một lần, ra thẳng LỆNH MÁY CPU THẬT — thứ CPU
hiểu được trực tiếp, không cần một chương trình trung gian (như
CPython) đứng giữa đọc-hiểu-làm nữa. Không còn tầng thông dịch, không
còn chi phí lặp lại vòng lấy-hiểu-làm ở PHẦN MỀM mỗi lần gọi hàm.

Đổi lại, ngôn ngữ biên dịch thẳng phải trả một cái giá khác: lệnh máy
CPU không phải một chuẩn chung cho mọi máy. Mỗi họ CPU (dùng trong máy
tính, điện thoại,...) có tập lệnh máy RIÊNG. Biên dịch xong một bản
cho loại CPU này, đem chạy trên loại CPU khác — không chạy được, phải
biên dịch LẠI, riêng cho từng loại máy.

Python đi đường khác: giữ một tầng trung gian (bytecode) không gắn với
bất kỳ loại CPU cụ thể nào. Cái giá là tầng thông dịch chạy mỗi lần
gọi hàm (đo được, dưới đây). Cái được là một bản bytecode DUY NHẤT —
hay đơn giản hơn, một file `.py` DUY NHẤT — chạy được trên BẤT KỲ máy
nào có cài CPython, không cần biên dịch lại: máy Windows, macOS,
Linux, kể cả trên chính trình duyệt bạn đang học khoá này (Pyodide,
chạy bên trong WebAssembly) — không có bản CPython riêng nào được
biên dịch tay cho từng loại CPU ở đây cả.
::::

::::example{#do-thoi-gian-that}
```python title=readonly
import time

def lap_phuong(x):
    return x ** 3

bat_dau = time.perf_counter()
for _ in range(500_000):
    lap_phuong(2)
ket_thuc = time.perf_counter()

thoi_gian_da_qua = ket_thuc - bat_dau
print(thoi_gian_da_qua > 0)
```

```text title=readonly
True
```

500 nghìn lần gọi `lap_phuong` là 500 nghìn lần CPython chạy lại TOÀN
BỘ vòng lấy-hiểu-làm trên đúng vài lệnh bytecode của nó (`RESUME`,
`LOAD_FAST`, `BINARY_OP`, `RETURN_VALUE` — đúng dạng bài 1 đã dịch cho
một phép tính hai ngôi). `thoi_gian_da_qua` không phải `0` — nó là một
con số dương đo được thật, dù rất nhỏ với một máy hiện đại (đo thật
trên máy của khoá này: khoảng 0.09 giây cho nửa triệu lần gọi). Con số
CHÍNH XÁC sẽ khác nhau giữa các lần chạy, giữa các máy — vì vậy bài
này chỉ kiểm tra `> 0`, không ghim một con số tuyệt đối, đúng cách
T3.1 đã dạy với `sys.getsizeof`.

Đo tiếp với ít lần gọi hơn — chỉ 20 nghìn lần thay vì 500 nghìn — cho
`thoi_gian_da_qua` nhỏ hơn hẳn (đo thật: khoảng 0.003 giây). Gọi hàm
NHIỀU lần hơn tốn nhiều thời gian CPU thật hơn — vì mỗi lần gọi là một
lượt thông dịch riêng, không dùng lại kết quả lượt trước. Một ngôn ngữ
biên dịch thẳng, chạy đúng phép tính này bằng lệnh CPU thật, không qua
tầng thông dịch nào, thường đo được thời gian NHỎ HƠN cho CÙNG số lần
gọi — chính là cái giá bài này đang nói tới.
::::

::::predict{#doi-lay-gi commitOnce}
Ngôn ngữ A biên dịch THẲNG ra lệnh máy CPU, không qua bước thông dịch
bytecode nào. Ngôn ngữ B (như Python) biên dịch ra bytecode trung
gian rồi THÔNG DỊCH nó mỗi lần chạy (bài 9).

Viết CÙNG một chương trình bằng cả hai ngôn ngữ, chạy trên máy bạn
đang dùng. Điều nào đúng?

:::opt{correct}
Chương trình A thường chạy nhanh hơn (không tốn thời gian cho tầng
thông dịch phụ) — nhưng đem chạy nó trên một loại CPU KHÁC thì phải
biên dịch LẠI, riêng cho loại CPU đó. Chương trình B chạy được ngay
trên máy khác, chỉ cần có sẵn trình thông dịch phù hợp (như CPython),
không cần biên dịch lại.
:::

:::opt
Chương trình B luôn nhanh hơn A, vì bytecode của nó đã được biên dịch
sẵn từ trước (bài 9), không phải dịch lại mỗi lần chạy
::why
Gần đúng ở việc bạn nhớ đúng bài 9: bytecode CỦA B đúng là đã biên
dịch sẵn, không dịch lại mỗi lần gọi hàm — phần đó chính xác.

Chỗ lệch: "biên dịch sẵn" ở đây chỉ tạo ra BYTECODE — một dạng lệnh
TRUNG GIAN, chưa phải lệnh CPU thật. B vẫn còn NGUYÊN một tầng thông
dịch (bài 8) phải chạy MỖI LẦN gọi hàm để biến bytecode đó thành hành
động thật — CPU không đọc được bytecode trực tiếp. A tạo ra lệnh CPU
thật NGAY từ bước biên dịch, không còn tầng nào phải chạy thêm mỗi
lần gọi — đó là lý do A thường nhanh hơn, không phải ngược lại.
::
:::

:::opt
Không có khác biệt nào cả — cuối cùng máy nào cũng chạy lệnh máy CPU
giống nhau, ngôn ngữ nguồn không quan trọng
::why
Gần đúng ở việc bạn nhớ đúng: đúng là CUỐI CÙNG, mọi phép tính đều
phải chạy thành lệnh máy mà CPU hiểu được — không có ngoại lệ.

Chỗ lệch nằm ở chữ "giống nhau". A tạo ra lệnh máy CPU NGAY từ bước
biên dịch — một bước duy nhất. B tạo ra bytecode TRUNG GIAN trước, rồi
cần thêm một chương trình (CPython) đọc bytecode đó và TỰ NÓ gọi lệnh
CPU thật, MỖI LẦN chạy — nhiều tầng hơn hẳn. Khác biệt nằm ở SỐ TẦNG
phải đi qua, không phải đích đến cuối cùng.
::
:::

:::opt
Chương trình A chạy được trên MỌI loại máy mà không cần biên dịch
lại, vì lệnh máy CPU là một chuẩn chung cho mọi CPU
::why
Gần đúng ở việc bạn tin có một "chuẩn chung" nào đó giúp mọi thứ chạy
được khắp nơi — đúng tinh thần, chỉ sai VỊ TRÍ của cái chuẩn đó.

Chỗ lệch: lệnh máy CPU KHÔNG chung cho mọi CPU — mỗi họ CPU (dùng
trong máy tính để bàn, điện thoại,...) có tập lệnh RIÊNG, khác hẳn
nhau. Đây chính xác là lý do A phải biên dịch LẠI cho từng loại máy.
Cái "chuẩn chung, chạy khắp nơi" thuộc về B — bytecode của B không gắn
với loại CPU nào, miễn có CPython cài sẵn là chạy được.
::
:::
::::

::::code{#do-thoi-gian-goi-ham}
Đo lại phép đo của ví dụ, nhưng lần này đo HAI vòng lặp khác kích cỡ —
2 nghìn lần rồi 20 nghìn lần — trên một hàm khác, `lap_phuong` (luỹ
thừa ba). Điền đúng hai chỗ trống, cả hai đều ghi lại thời điểm BẮT
ĐẦU đo, ngay trước vòng lặp tương ứng.

```python title=starter
import time

def lap_phuong(x):
    return x ** 3

bat_dau_nho = ___                  # BẮT ĐẦU đo, ngay trước vòng lặp 2 nghìn lần
for _ in range(2_000):
    lap_phuong(2)
ket_thuc_nho = time.perf_counter()
thoi_gian_nho = ket_thuc_nho - bat_dau_nho

bat_dau_lon = ___                  # BẮT ĐẦU đo, ngay trước vòng lặp 20 nghìn lần
for _ in range(20_000):
    lap_phuong(2)
ket_thuc_lon = time.perf_counter()
thoi_gian_lon = ket_thuc_lon - bat_dau_lon

print(lap_phuong(2))
```

```python title=solution
import time

def lap_phuong(x):
    return x ** 3

bat_dau_nho = time.perf_counter()
for _ in range(2_000):
    lap_phuong(2)
ket_thuc_nho = time.perf_counter()
thoi_gian_nho = ket_thuc_nho - bat_dau_nho

bat_dau_lon = time.perf_counter()
for _ in range(20_000):
    lap_phuong(2)
ket_thuc_lon = time.perf_counter()
thoi_gian_lon = ket_thuc_lon - bat_dau_lon

print(lap_phuong(2))
```

```python title=test
assert lap_phuong(2) == 8, f"lap_phuong(2) phải là 2 ** 3 = 8 — đang ra {lap_phuong(2)}"
assert isinstance(thoi_gian_nho, float) and thoi_gian_nho >= 0, f"thoi_gian_nho phải là một số thực không âm — đang là {thoi_gian_nho!r}"
assert isinstance(thoi_gian_lon, float) and thoi_gian_lon >= 0, f"thoi_gian_lon phải là một số thực không âm — đang là {thoi_gian_lon!r}"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều làm ĐÚNG MỘT việc như nhau — ghi lại thời điểm hiện tại, ngay TRƯỚC vòng lặp mà nó đo. Nhìn đúng dòng ket_thuc_nho/ket_thuc_lon ngay bên dưới mỗi chỗ trống để biết công cụ cần dùng.
- kind: strategy
  body: Dùng đúng time.perf_counter() — công cụ đo thời gian độ phân giải cao, cùng công cụ mà ket_thuc_nho và ket_thuc_lon đã dùng. Gọi nó Ở THỜI ĐIỂM BẮT ĐẦU, ngay trước for, không phải sau.
- kind: one-line
  body: 'Cả hai chỗ trống đều là: time.perf_counter()'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: cả hai chỗ trống phải THẬT SỰ gọi time.perf_counter() ngay trước vòng lặp tương ứng — không gõ một số cố định (như True/1/0) vào đó, vì phép đo sẽ không còn đo THỜI ĐIỂM BẮT ĐẦU thật nữa, mà đo một hằng số vô nghĩa
  requireAst:
  # uses-call target perf_counter: khung đã có sẵn đúng 2 lời gọi
  # time.perf_counter() (hai dòng ket_thuc_nho, ket_thuc_lon) TRƯỚC khi
  # điền gì cả. Lời giải đúng thêm đúng 2 lời gọi nữa (hai chỗ trống)
  # -> tổng 4.
  # uses-name target time: tên "time" là Name(Load) ở gốc mỗi lời gọi
  # time.perf_counter() — cùng đếm y hệt: baseline 2 (hai dòng
  # ket_thuc), lời giải đúng thêm 2 -> tổng 4. "import time" không tạo
  # ra Name(Load) nên không tính vào baseline.
  # ĐÃ THỬ BA CÁCH ĐIỀN HỤT (True/1/0 vào CẢ HAI chỗ trống): không cách
  # nào gọi perf_counter hay đọc tên time thêm lần nào -> cả ba TRƯỢT
  # static ngay lập tức (dừng ở baseline 2, dưới min:4).
  # Về mặt runtime (không dùng để chấm, chỉ ghi lại vì đã đo thật): cả
  # ba dud đều cho lap_phuong(2)==8 đúng (không phụ thuộc chỗ trống) và
  # cả hai thoi_gian_* vẫn là float không âm (bool/int trừ float vẫn ra
  # float, và trên máy đo được luôn dương vì đồng hồ hệ thống không lùi
  # về âm) — nghĩa là TẦNG TESTS một mình KHÔNG phân biệt được dud với
  # lời giải đúng ở bài này (giá trị tuyệt đối của time.perf_counter()
  # phụ thuộc mốc tham chiếu riêng của môi trường, không ổn định để làm
  # cổng). Đây là lý do bắt buộc tầng static trở thành cổng CHÍNH cho
  # khối này, không phải phụ trợ — đã kiểm tra riêng, static một mình
  # chặn đủ cả ba dud.
  # Không có while nào ở đây; hai vòng for lặp số lần CỐ ĐỊNH
  # (range(2_000), range(20_000)), hoàn toàn không phụ thuộc giá trị
  # điền vào hai chỗ trống — không có nguy cơ chạy vô thời hạn dù điền
  # gì. Kích cỡ hai vòng lặp NHỎ HƠN ví dụ phần trên (vốn dùng 20 nghìn/
  # 500 nghìn) là CÓ CHỦ Ý: bộ kiểm THẬT (kiem_ma_bai_hoc.mjs) đếm bước
  # thực thi qua sys.settrace với trần 300.000 bước để bắt vòng lặp
  # không dừng — đã ĐO THẬT bằng đúng cơ chế đó (packages/exec-python)
  # rằng range(20_000)+range(500_000) trong CÙNG một lần chạy tốn
  # khoảng 2.600.000 bước (mỗi vòng gọi hàm tốn ~5 bước: line, call,
  # line trong hàm, return, cộng chi phí ngoài vòng), VƯỢT trần — khiến
  # CHÍNH LỜI GIẢI ĐÚNG bị cổng báo "vượt hạn mức bước" dù không hề có
  # vòng lặp vô hạn nào. range(2_000)+range(20_000) chỉ tốn khoảng
  # 110.000 bước, dưới 40% trần, an toàn nhiều lần cả khi cộng thêm chi
  # phí của ba lần dud thử lại. Phần "example" phía trên vẫn giữ nguyên
  # 20 nghìn/500 nghìn vì đó là khối title=readonly, không được cổng
  # chấm chạy qua sys.settrace (cổng chỉ chạy solution của khối `code`).
  - kind: uses-call, target: perf_counter, min: 4
  - kind: uses-name, target: time, min: 4
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^8\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không có bữa trưa miễn phí — thông dịch đổi tốc độ lấy sự tiện, biên
dịch thẳng đổi sự tiện lấy tốc độ. Python chọn vế sau của Python:
chạy chậm hơn một chút, chạy được ở khắp nơi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mọi lệnh máy bạn đã thấy tới giờ — `LOAD_FAST`, `BINARY_OP`,
`JUMP_BACKWARD`, `POP_JUMP_IF_FALSE` — đều là những bước NHỎ xảy ra
ngay TRONG một lần gọi hàm, không rời khỏi nó. Nhưng chương trình thật
luôn có nhiều hàm GỌI LẪN NHAU — dòng như `tra_gia(co_to)` gọi sang
một hàm khác hẳn.

Việc GỌI một hàm khác có phải một LỆNH giống các lệnh bạn đã học
không, hay nó là một cơ chế đặc biệt, tách hẳn khỏi danh sách lệnh?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
