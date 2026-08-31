---
id: khoa-hoc-may-tinh.may-chay-the-nao.nguon-thanh-lenh-may
title: "Mã nguồn Python biến thành lệnh máy trước khi chạy"
summary: "`dis.dis(ham)` in ra đúng chuỗi LỆNH MÁY thật mà một hàm Python được dịch thành, chạy thật trên CPython 3.13 của khoá — máy không đọc Python, nó dịch trước, và bước dịch ấy xem tận mắt được."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.bytecode]
requires: [core.function-def]
concepts: [may.bytecode]
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
Câu hỏi cuối track trước còn treo đó. Hôm nay mở nắp: máy không đọc
Python — nó dịch trước.
::::

::::explain{#may-co-doc-duoc-python-khong}
Bạn gõ `gia_mot_ve * 3`, nhấn chạy, con số hiện ra. Máy tính có **đọc**
được dòng chữ đó không — theo đúng nghĩa đọc một câu, như bạn đọc một
câu tiếng Việt?

Không. Bộ xử lý (CPU) bên trong máy chỉ hiểu được những bước cực nhỏ gọi
là **lệnh máy** — những hành động đơn giản tới mức không chia nhỏ hơn
được nữa (bài sau sẽ đào sâu đúng một việc mỗi lệnh làm). `gia_mot_ve *
3` không phải một lệnh máy. Nó là chữ viết cho NGƯỜI đọc. Trước khi máy
chạy được gì cả, phải có một bước DỊCH: đổi dòng chữ ấy thành một chuỗi
lệnh máy thật.

Bước dịch đó không phải phép màu giấu kín. Có một công cụ cho bạn xem
tận mắt: mô-đun `dis` (viết tắt disassemble — tháo rời). Gọi
`dis.dis(ten_ham)`, Python in ra đúng chuỗi lệnh máy mà hàm ấy đã được
dịch thành, trên chính máy bạn đang chạy.

Một chi tiết sẽ xuất hiện ở MỌI hàm bạn đưa cho `dis.dis()`: dòng đầu
tiên luôn là `RESUME` — một lệnh mở đầu bắt buộc, đứng trước mọi hàm để
chuẩn bị cho nó bắt đầu chạy. Từ giờ về sau bạn sẽ luôn thấy nó đứng đầu
— cứ xem nó như một nghi thức mở màn, không cần đào sâu.
::::

::::example{#gia-ba-ve}
Một hàm nhân giá một vé lên gấp ba:

```python title=readonly
import dis

def gia_ba_ve(gia_mot_ve):
    return gia_mot_ve * 3

dis.dis(gia_ba_ve)
```

```text title=readonly
  3           RESUME                   0

  4           LOAD_FAST                0 (gia_mot_ve)
              LOAD_CONST               1 (3)
              BINARY_OP                5 (*)
              RETURN_VALUE
```

Bốn lệnh thật, chạy thật trên máy của khoá này. `LOAD_FAST` lấy giá trị
đang giữ trong `gia_mot_ve` ra. `LOAD_CONST` lấy con số `3` ra — nó đã
có sẵn trong mã, không cần "lấy từ biến" nào. `BINARY_OP` (viết tắt của
"phép toán hai ngôi") thực hiện đúng phép nhân, dấu `*` được ghi ngay
trong ngoặc để bạn biết là phép tính nào. `RETURN_VALUE` trả kết quả về.

Không có dòng nào trong bốn dòng này nói "nhân gia_mot_ve với 3" như mã
nguồn viết. Mã nguồn là MỘT câu. Lệnh máy là BỐN bước.
::::

::::predict{#du-doan-cong-thue commitOnce}
Một hàm khác, tính tổng hai khoản tiền rồi trả về:

```python
def cong_thue(gia, thue):
    tong = gia + thue
    return tong
```

**Trước khi chạy** `dis.dis(cong_thue)`, bạn đoán dãy TÊN LỆNH nào hiện
ra (bỏ qua chi tiết trong ngoặc, chỉ tên lệnh, đúng thứ tự)?

:::opt{correct}
RESUME, LOAD_FAST_LOAD_FAST, BINARY_OP, STORE_FAST, LOAD_FAST,
RETURN_VALUE
:::

:::opt
RESUME, LOAD_FAST, LOAD_FAST, BINARY_ADD, STORE_FAST, LOAD_FAST,
RETURN_VALUE
::why
Gần đúng ở chỗ bạn liệt kê đúng THỨ TỰ việc phải làm: lấy `gia`, lấy
`thue`, cộng, cất vào `tong`, lấy `tong` ra lại, trả về — đúng mạch
logic từng chữ.

Chỗ lệch là tên lệnh. Bạn đang nhớ theo sách nói về các bản Python cũ
hơn, nơi hai lệnh lấy giá trị đứng tách rời và phép cộng có tên riêng
`BINARY_ADD`. Trên CPython 3.13 chạy trong khoá này, hai lệnh LẤY liền
nhau, cùng đọc từ biến cục bộ, được GỘP in chung thành một dòng
`LOAD_FAST_LOAD_FAST`, và mọi phép tính hai ngôi đều mang tên chung
`BINARY_OP` kèm dấu phép tính trong ngoặc. Đừng đoán theo trí nhớ sách —
`dis.dis()` chạy thật trên máy này mới là sự thật.
::
:::

:::opt
RESUME, LOAD_FAST_LOAD_FAST, BINARY_OP, RETURN_VALUE
::why
Gần đúng ở việc bạn nhận ra máy phải lấy hai giá trị rồi cộng — đúng hai
bước đầu.

Chỗ lệch: hàm có một dòng RIÊNG `tong = gia + thue`, tách khỏi dòng
`return tong`. Dòng gán ấy là một việc THẬT máy phải làm — CẤT kết quả
phép cộng vào cái tên `tong` (`STORE_FAST`). Rồi dòng `return tong` phải
LẤY LẠI đúng giá trị vừa cất (`LOAD_FAST`) trước khi trả về. Gộp hai
dòng nguồn thành một bước tính là bỏ sót hai việc máy thật sự làm.
::
:::

:::opt
RESUME, LOAD_FAST, LOAD_FAST, BINARY_OP, STORE_FAST, LOAD_FAST,
RETURN_VALUE
::why
Gần đúng ở gần như toàn bộ: đúng số lệnh, đúng thứ tự, đúng tên
`BINARY_OP` thay vì tên cũ.

Chỗ lệch chỉ nằm ở hai lệnh lấy giá trị đầu tiên. `gia` và `thue` đứng
NGAY CẠNH NHAU trong dòng lệnh và cả hai đều đọc từ biến cục bộ — đúng
điều kiện để CPython 3.13 GỘP chúng thành một dòng in duy nhất
`LOAD_FAST_LOAD_FAST`, không phải hai dòng `LOAD_FAST` tách rời.
::
:::
::::

::::code{#tinh-tien-ve}
Quầy vé cần một hàm tính tổng tiền: số vé nhân với giá một vé. Điền vào
chỗ trống, rồi xem chính máy dịch nó thành lệnh gì.

```python title=starter
import dis

def tinh_tien_ve(so_ve, gia_mot_ve):
    return ___                         # nhân so_ve với gia_mot_ve

print(tinh_tien_ve(3, 45000))
dis.dis(tinh_tien_ve)
```

```python title=solution
import dis

def tinh_tien_ve(so_ve, gia_mot_ve):
    return so_ve * gia_mot_ve

print(tinh_tien_ve(3, 45000))
dis.dis(tinh_tien_ve)
```

```python title=test
assert tinh_tien_ve(3, 45000) == 135000, f"3 vé giá 45000 phải ra 135000 — đang ra {tinh_tien_ve(3, 45000)}"
assert tinh_tien_ve(5, 10000) == 50000, f"5 vé giá 10000 phải ra 50000 — đang ra {tinh_tien_ve(5, 10000)}"
```

:::hints
- kind: attention
  body: Chỉ một việc cần làm — tính tổng tiền bằng cách NHÂN hai đại lượng đã có sẵn tên, so_ve và gia_mot_ve. Không cần gọi thêm hàm nào.
- kind: strategy
  body: Viết đúng phép nhân giữa hai cái tên đã có — so_ve và gia_mot_ve. Thứ tự nào cũng được, phép nhân không quan tâm ai đứng trước.
- kind: one-line
  body: 'Chỗ trống là: so_ve * gia_mot_ve'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ nhân so_ve với gia_mot_ve — dùng đúng hai cái tên đã có, đừng gõ cứng một con số
  requireAst:
  # Cả hai tên phải được ĐỌC ít nhất một lần — chặn kiểu gõ cứng
  # `return 135000`. Đã thử: một lời giải khác `gia_mot_ve * so_ve` (đổi
  # thứ tự, phép nhân giao hoán) vẫn qua cả static lẫn tests — đúng luật 2.
  - kind: uses-name, target: so_ve, min: 1
  - kind: uses-name, target: gia_mot_ve, min: 1
  - kind: uses-operator, target: "*", min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^135000\\n"
- tier: output
  expect: "LOAD_FAST_LOAD_FAST"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`so_ve` và `gia_mot_ve` đứng cạnh nhau, cùng đọc từ biến cục bộ — máy
gộp chúng thành một dòng `LOAD_FAST_LOAD_FAST`. Bạn vừa thấy đúng bản
dịch thật, không phải hình dung suông.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`LOAD_FAST_LOAD_FAST`, `BINARY_OP`, `RETURN_VALUE` — mỗi lệnh trong
danh sách ấy làm được bao nhiêu việc? Một lệnh có thể vừa lấy giá trị,
vừa tính toán, vừa trả kết quả trong một bước không — hay mỗi lệnh chỉ
được phép làm đúng một việc nhỏ, không hơn?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
