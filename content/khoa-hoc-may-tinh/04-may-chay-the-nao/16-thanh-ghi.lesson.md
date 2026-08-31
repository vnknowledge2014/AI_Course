---
id: khoa-hoc-may-tinh.may-chay-the-nao.thanh-ghi
title: "Thanh ghi: kho nhỏ nhất, nhanh nhất, ngay trong CPU"
summary: "T3.1 dạy bộ nhớ là dãy ô có địa chỉ — nhưng CPU còn giữ một nhúm ô KHÔNG có địa chỉ kiểu đó, nằm ngay trong chip: thanh ghi. Đây là nơi BINARY_OP (bài 3) thật sự diễn ra ở tầng phần cứng, dưới cả ngăn xếp tính toán."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [may.registers]
requires: [mem.address]
concepts: [may.registers]
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
`LOAD_FAST` lấy một giá trị ra — nhưng lấy ra TỪ ĐÂU? Không phải mọi "ở
đâu" trong máy đều nhanh như nhau.
::::

::::explain{#kho-khong-co-dia-chi}
Bài "Bộ nhớ là một dãy ô có đánh số" (T3.1) dạy một sự thật đã dùng suốt
từ đó tới giờ: mọi ô nhớ có một **địa chỉ** — một con số thứ tự, như số
nhà trên một con phố cực dài. Muốn đọc một ô, máy phải CHO địa chỉ đó,
rồi ĐỢI câu trả lời quay về. Bước gửi-rồi-đợi ấy tốn thời gian thật, dù
rất nhỏ.

CPU còn giữ một nhúm chỗ chứa khác hẳn — không nằm trên con phố đó, không
có địa chỉ kiểu đánh số như RAM. Chúng nằm NGAY TRONG con chip CPU, ngay
cạnh bộ phận làm phép tính. Tên gọi của chúng: **thanh ghi** (register).

Ba điều làm thanh ghi khác hẳn một ô nhớ thường:

- **Cực ít.** Một CPU hiện đại có vài chục thanh ghi, không hơn — đếm
  được trên hai bàn tay, không phải hàng tỉ ô như RAM.
- **Cực nhỏ.** Mỗi thanh ghi chỉ giữ được đúng một giá trị tại một thời
  điểm — một số, hoặc một địa chỉ trỏ đi đâu đó.
- **Không cần bước gửi-địa-chỉ-rồi-đợi.** Vì không đánh số kiểu đường
  phố, CPU không phải "hỏi rồi đợi" — giá trị đã nằm sẵn ngay bên cạnh bộ
  phận sẽ dùng nó.

Nhớ lại bài "Máy tính bằng một ngăn xếp riêng" (bài 3): `BINARY_OP` lấy
hai giá trị ở đỉnh ngăn xếp tính toán ra rồi CỘNG chúng. Cái ngăn xếp tính
toán đó vẫn là một cấu trúc nằm trong vùng nhớ CÓ địa chỉ — CPython tự
quản nó bằng C, nhưng nó vẫn là bộ nhớ bình thường. Còn phép CỘNG tự nó —
hành động hai con số thật sự chập vào nhau thành một — chỉ có thể xảy ra
ở một chỗ: bên trong bộ phận tính toán của CPU, dùng đúng vài thanh ghi
làm chỗ giữ tạm hai số hạng và kết quả. Đây là tầng THẤP HƠN cả bài 3 nói
tới — Python không cho bạn thấy trực tiếp (không có `id()` nào chỉ vào
một thanh ghi), nhưng nó luôn ở đó, dưới đáy mọi phép tính.
::::

::::example{#dem-lan-muon-thanh-ghi}
Chạy lại đúng công cụ quen thuộc từ bài 1 — `dis.dis()` — trên một hàm
cộng hai số:

```python title=readonly
import dis

def cong(a, b):
    return a + b

dis.dis(cong)
```

```text title=readonly
  4           RESUME                   0

  5           LOAD_FAST_LOAD_FAST      1 (a, b)
              BINARY_OP                0 (+)
              RETURN_VALUE
```

`dis.dis()` chỉ cho thấy lệnh Ở TẦNG BYTECODE — `BINARY_OP` là MỘT lệnh,
không chia nhỏ hơn được nữa trong danh sách này. Nhưng khi CPython thật
sự THI HÀNH lệnh `BINARY_OP` đó (bài 8: trình thông dịch cũng là một
chương trình chạy trên CPU thật), chính đoạn mã C xử lý phép cộng phải
đưa `a` và `b` vào thanh ghi trước, cộng trong thanh ghi, rồi mới đẩy kết
quả trở lại ngăn xếp tính toán. Một dòng `dis.dis()` — nhiều bước hơn thế
ở tầng phần cứng bên dưới nó.
::::

::::predict{#phep-cong-o-dau commitOnce}
```python
def cong(a, b):
    return a + b
```

`dis.dis(cong)` cho thấy `BINARY_OP 0 (+)` là lệnh thực hiện phép cộng.
Khi CPU THẬT SỰ làm phép cộng đó — không phải bước gọi tên lệnh, mà bước
hai con số chập vào nhau thành một — việc đó xảy ra Ở ĐÂU, vật lý?

:::opt{correct}
Trong thanh ghi — CPU phải đưa cả `a` lẫn `b` vào thanh ghi trước, vì bộ
phận tính toán chỉ làm việc được với giá trị đã nằm sẵn ngay cạnh nó
:::

:::opt
Ngay trong RAM, tại đúng hai địa chỉ mà `a` và `b` đang lưu — không cần
di chuyển đi đâu cả
::why
Gần đúng ở chỗ `a` và `b` đúng là CÓ một chỗ lưu, và trước khi phép cộng
xảy ra, hai giá trị đó có thể đang nằm trong vùng nhớ có địa chỉ.

Chỗ lệch: bộ phận làm phép tính của CPU không "vươn tay" ra RAM để tính
tại chỗ. Nó chỉ tính được với giá trị đã có mặt ngay bên cạnh nó — nghĩa
là phải ĐƯA giá trị vào thanh ghi trước, một bước di chuyển thật, không
phải "tính thẳng tại nơi giá trị đang đứng".
::
:::

:::opt
Ngay trong ngăn xếp tính toán mà bài 3 đã dạy, vì `BINARY_OP` thao tác
trực tiếp trên đó
::why
Gần đúng ở tầng BYTECODE — đúng là `BINARY_OP` LẤY hai giá trị từ đỉnh
ngăn xếp tính toán ra, khớp hệt bài 3.

Chỗ lệch: ngăn xếp tính toán là một cấu trúc CPython tự quản trong vùng
nhớ có địa chỉ — cùng loại chỗ chứa với RAM, chỉ do interpreter tự quản.
Bản thân phép TÍNH — bước hai số thật sự chập lại — vẫn phải xảy ra ở
một tầng THẤP HƠN nó: trong thanh ghi của CPU, không phải ngay trên ngăn
xếp đó.
::
:::

:::opt
Không ở đâu cụ thể — phép cộng diễn ra tức thời, không cần chỗ chứa nào
cả
::why
Gần đúng ở cảm giác "nhanh tới mức như không tốn thời gian" — thanh ghi
đúng là cực nhanh, gần như tức thời so với RAM.

Chỗ lệch: "tức thời" không có nghĩa là "không cần chỗ". Mọi con số trong
máy, dù tồn tại trong bao lâu, đều phải NẰM ở một chỗ vật lý cụ thể —
một bit thật, ở một vị trí thật. Thanh ghi chính là chỗ đó cho phép
cộng, không phải "hư không".
::
:::
::::

::::code{#dem-lan-can-thanh-ghi}
Hàm `tinh` dưới đây có BA phép tính nối tiếp — mỗi phép tính là một lần
CPU phải mượn thanh ghi để làm việc. Việc của bạn: đếm đúng số lần đó,
bằng phương thức `.count()` của chuỗi, trên chính văn bản `dis.dis()`
in ra.

```python title=starter
import dis, io

def tinh(a, b, c):
    return a * b + c - 1

dis.dis(tinh)

bo_nho_tam = io.StringIO()
dis.dis(tinh, file=bo_nho_tam)
van_ban = bo_nho_tam.getvalue()

so_lan_binary_op = ___          # đếm "BINARY_OP" trong van_ban bằng .count()
print(f"Số lần CPU phải mượn thanh ghi để tính: {so_lan_binary_op}")
```

```python title=solution
import dis, io

def tinh(a, b, c):
    return a * b + c - 1

dis.dis(tinh)

bo_nho_tam = io.StringIO()
dis.dis(tinh, file=bo_nho_tam)
van_ban = bo_nho_tam.getvalue()

so_lan_binary_op = van_ban.count("BINARY_OP")
print(f"Số lần CPU phải mượn thanh ghi để tính: {so_lan_binary_op}")
```

```python title=test
assert so_lan_binary_op == 3, f"tinh có ba phép tính nối tiếp (nhân, cộng, trừ) — mỗi phép là một BINARY_OP, phải đếm được 3 — đang ra {so_lan_binary_op}"
assert van_ban.count("BINARY_OP") == so_lan_binary_op, "so_lan_binary_op phải đúng bằng số lần chữ 'BINARY_OP' thật sự xuất hiện trong van_ban — không được gõ một con số khác vào chỗ trống"
```

:::hints
- kind: attention
  body: Chỉ một chỗ trống. Nó phải là một phép ĐẾM thật trên chuỗi van_ban, không phải một con số bạn tự đoán rồi gõ vào.
- kind: strategy
  body: 'Chuỗi có sẵn phương thức .count(chuoi_con) — trả về số lần chuoi_con xuất hiện. Đếm đúng số lần "BINARY_OP" xuất hiện trong van_ban: van_ban.count("BINARY_OP").'
- kind: one-line
  body: 'Chỗ trống là: van_ban.count("BINARY_OP")'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: bài này đòi đếm THẬT bằng phương thức .count() của chuỗi trên van_ban — không được gõ thẳng con số 3 vào chỗ trống, vì đó không còn là một phép đếm nữa
  requireAst:
  # Đề bài nêu đích danh công cụ (".count() của chuỗi"), nên cổng hẹp đúng
  # công cụ đó là hợp lệ theo Luật 3. min:1 khớp đúng lời giải — chỉ có
  # một .count() trong toàn khối, và không cách viết đúng nào khác của
  # bài này tránh được việc gọi .count() ít nhất một lần, vì đó chính là
  # công cụ đề bài chỉ định.
  - kind: uses-call, target: count, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: "BINARY_OP"
- tier: output
  expect: "Số lần CPU phải mượn thanh ghi để tính: 3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba phép tính, ba lần mượn thanh ghi — nhân, rồi cộng, rồi trừ. Ngăn xếp
tính toán giữ thứ tự; thanh ghi làm việc tính thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Thanh ghi cực nhanh — nhưng cũng cực NHỎ, chỉ vài chục chỗ chứa, không
thể nào giữ nổi cả một chương trình đang chạy, với đủ loại biến, danh
sách, chuỗi ký tự.

Vậy giữa thanh ghi (vài chục chỗ, ngay trong chip) và RAM (hàng tỉ ô, có
địa chỉ, phải gửi-rồi-đợi) — có chỗ nào ở giữa không? Lớn hơn thanh ghi,
nhưng vẫn nhanh hơn hẳn RAM?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
