---
id: khoa-hoc-may-tinh.may-chay-the-nao.dich-tay-doi-chieu-dis
title: "Dịch tay một hàm nhỏ, đối chiếu với dis thật"
summary: "Bài chốt cụm: tự viết dự đoán chuỗi lệnh cho một hàm ba dòng — dùng đúng vốn từ đã học (LOAD_FAST, BINARY_OP, CALL, RETURN_VALUE, các lệnh nhảy) — TRƯỚC KHI chạy dis.dis(), rồi đối chiếu với máy thật bằng dis.get_instructions(). Không đoán suông — tự kiểm bằng máy thật, ngay trong chính đoạn mã."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.hand-translate]
requires: [may.count-calls-via-dis]
concepts: [may.hand-translate]
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
Mười bốn bài để ĐỌC máy nói. Bài này, bạn nói TRƯỚC — rồi để máy chấm
lại.
::::

::::explain{#von-tu-va-phuong-phap}
Suốt cụm bài này, bạn đã học đúng một bộ từ vựng, đủ để đọc gần hết một
hàm nhỏ trên `dis.dis()`:

| Lệnh | Việc nó làm |
|---|---|
| `LOAD_FAST` / `LOAD_FAST_LOAD_FAST` | Đẩy giá trị của một (hoặc hai) biến cục bộ lên ngăn xếp tính toán |
| `LOAD_CONST` | Đẩy một hằng số (viết thẳng trong mã nguồn) lên ngăn xếp |
| `BINARY_OP` | Lấy hai đỉnh ngăn xếp ra, tính, đẩy kết quả lại |
| `COMPARE_OP` | Lấy hai đỉnh ra, so sánh, đẩy `True`/`False` lại |
| `STORE_FAST` | Lấy đỉnh ngăn xếp ra, cất vào một biến cục bộ |
| `POP_JUMP_IF_FALSE` | Lấy đỉnh ra; nếu `False` thì nhảy tới nhãn khác, nếu `True` thì đi tiếp |
| `LOAD_GLOBAL` (kèm `+ NULL`) | Đẩy một hàm toàn cục lên ngăn xếp, chuẩn bị gọi nó |
| `CALL` | Gọi hàm vừa nạp, với đúng N đối số cũng vừa nạp |
| `RETURN_VALUE` / `RETURN_CONST` | Lấy đỉnh ngăn xếp (hoặc một hằng số), đóng khung, trả về khung đã gọi |

Bài này không thêm một lệnh mới nào. Nó đổi CHIỀU làm việc: mọi bài
trước, máy chạy trước, bạn đọc kết quả sau. Bài này, bạn viết dự đoán
TRƯỚC — dùng đúng bảng từ vựng trên — rồi mới chạy `dis.dis()` thật để
đối chiếu.

Phương pháp: đọc mã nguồn từng dòng, hỏi "dòng này ĐỌC biến nào, TÍNH
gì, GỌI ai, RẼ NHÁNH ở đâu, TRẢ VỀ gì" — rồi viết ra đúng thứ tự lệnh,
theo đúng thứ tự các câu hỏi ấy được trả lời.
::::

::::example{#dich-tay-mau}
Hàm ba dòng, không có lời gọi hàm nào, để thấy rõ phương pháp trước
khi thêm `CALL` vào:

```python title=readonly
def tinh(a, b):
    c = a + b
    return c * 2
```

Dịch tay, từng dòng:

- `c = a + b` — ĐỌC `a`, ĐỌC `b` (gộp thành `LOAD_FAST_LOAD_FAST`),
  TÍNH cộng (`BINARY_OP`), CẤT vào `c` (`STORE_FAST`).
- `return c * 2` — ĐỌC `c` (`LOAD_FAST`), ĐỌC hằng số `2`
  (`LOAD_CONST`), TÍNH nhân (`BINARY_OP`), TRẢ VỀ (`RETURN_VALUE`).

Dự đoán viết ra:

```text
LOAD_FAST_LOAD_FAST, BINARY_OP, STORE_FAST,
LOAD_FAST, LOAD_CONST, BINARY_OP, RETURN_VALUE
```

Đối chiếu với `dis.dis(tinh)` thật:

```text title=readonly
  4           RESUME                   0

  5           LOAD_FAST_LOAD_FAST      1 (a, b)
              BINARY_OP                0 (+)
              STORE_FAST               2 (c)

  6           LOAD_FAST                2 (c)
              LOAD_CONST               1 (2)
              BINARY_OP                5 (*)
              RETURN_VALUE
```

Khớp đúng từng lệnh (bỏ qua `RESUME` — lệnh mở đầu MỌI hàm, không cần
dịch, không tính vào bản dự đoán).
::::

::::predict{#doan-chuoi-lenh-co-goi commitOnce}
```python
def nhan_ba(x):
    return x * 3

def tru_roi_nhan_ba(a, b):
    hieu = a - b
    return nhan_ba(hieu)
```

Bốn chuỗi dưới đây, chỉ MỘT chuỗi khớp đúng `dis.dis(tru_roi_nhan_ba)`
THẬT (bỏ qua `RESUME`). Đâu là chuỗi đúng?

:::opt{correct}
`LOAD_FAST_LOAD_FAST, BINARY_OP, STORE_FAST, LOAD_GLOBAL, LOAD_FAST,
CALL, RETURN_VALUE`
:::

:::opt
`LOAD_FAST_LOAD_FAST, BINARY_OP, STORE_FAST, LOAD_FAST, LOAD_GLOBAL,
CALL, RETURN_VALUE`
::why
Gần đúng ở việc bạn liệt kê đủ bảy lệnh, đúng cả nhóm lệnh xuất hiện —
không thiếu, không thừa cái tên nào.

Chỗ lệch: thứ tự `LOAD_GLOBAL` và `LOAD_FAST` bị đảo. Muốn `CALL` biết
nó đang gọi HÀM NÀO, cái TÊN hàm (`nhan_ba`) phải được đẩy lên ngăn xếp
TRƯỚC đối số của nó (bài "Gọi hàm cũng chỉ là một lệnh") — `LOAD_GLOBAL`
luôn đứng trước `LOAD_FAST` của tham số truyền vào, không phải ngược
lại.
::
:::

:::opt
`LOAD_FAST_LOAD_FAST, BINARY_OP, LOAD_GLOBAL, LOAD_FAST, CALL,
RETURN_VALUE`
::why
Gần đúng ở việc bạn theo dõi đúng luồng giá trị — hiệu số vẫn tính
đúng, vẫn được dùng làm đối số gọi `nhan_ba`.

Chỗ lệch: bạn bỏ mất `STORE_FAST`. Mã nguồn có VIẾT RA một cái tên —
`hieu = a - b` — và mỗi lần một cái tên được GÁN, luôn có một
`STORE_FAST` cất giá trị vào biến cục bộ đó, TRƯỚC KHI dòng sau kịp đọc
lại nó bằng `LOAD_FAST`. Không có "đường tắt" giữ giá trị lơ lửng trên
ngăn xếp qua hai dòng mã nguồn khác nhau.
::
:::

:::opt
`LOAD_FAST_LOAD_FAST, BINARY_OP, STORE_FAST, LOAD_GLOBAL, LOAD_FAST,
RETURN_VALUE, CALL`
::why
Gần đúng ở việc bạn liệt kê đủ bảy lệnh, không thiếu không thừa.

Chỗ lệch: thứ tự `CALL` và `RETURN_VALUE` bị đảo. `nhan_ba(hieu)` phải
CHẠY XONG — đóng khung của chính nó, trả giá trị về (bài "RETURN_VALUE
đóng khung") — TRƯỚC KHI `tru_roi_nhan_ba` có giá trị nào để
`RETURN_VALUE`. Không hàm nào trả về được một thứ chưa hề tồn tại.
::
:::
::::

::::code{#du-doan-doi-chieu-may-that}
Hai hàm dưới đây đã viết sẵn — đừng sửa chúng. Trước khi xem máy thật
nói gì, TỰ VIẾT một danh sách tên lệnh bạn đoán `dis.dis(kiem_tra)` sẽ
cho ra, đúng thứ tự, dùng đúng những cái tên đã học suốt cụm này:
`LOAD_FAST`, `LOAD_CONST`, `COMPARE_OP`, `POP_JUMP_IF_FALSE`,
`LOAD_GLOBAL`, `CALL`, `RETURN_VALUE`, `RETURN_CONST`. Bỏ qua `RESUME`.

```python title=starter
import dis

def tang_gap_doi(n):
    return n * 2

def kiem_tra(n):
    if n > 0:
        return tang_gap_doi(n)
    return 0

du_doan = ___

lenh_that = [
    ins.opname for ins in dis.get_instructions(kiem_tra)
    if ins.opname != "RESUME"
]

print(du_doan == lenh_that)
print(lenh_that)
```

```python title=solution
import dis

def tang_gap_doi(n):
    return n * 2

def kiem_tra(n):
    if n > 0:
        return tang_gap_doi(n)
    return 0

du_doan = [
    "LOAD_FAST", "LOAD_CONST", "COMPARE_OP", "POP_JUMP_IF_FALSE",
    "LOAD_GLOBAL", "LOAD_FAST", "CALL", "RETURN_VALUE", "RETURN_CONST",
]

lenh_that = [
    ins.opname for ins in dis.get_instructions(kiem_tra)
    if ins.opname != "RESUME"
]

print(du_doan == lenh_that)
print(lenh_that)
```

```python title=test
assert du_doan == lenh_that, f"bản dịch tay chưa khớp máy thật — bạn đoán {du_doan}, máy thật cho {lenh_that}"
assert lenh_that == ["LOAD_FAST", "LOAD_CONST", "COMPARE_OP", "POP_JUMP_IF_FALSE", "LOAD_GLOBAL", "LOAD_FAST", "CALL", "RETURN_VALUE", "RETURN_CONST"], f"lenh_that phải đúng bytecode thật của kiem_tra, không được sửa hai hàm đã cho — đang ra {lenh_that}"
```

:::hints
- kind: attention
  body: 'du_doan phải là một DANH SÁCH CHUỖI, đúng thứ tự — không phải một số, không phải True/False. kiem_tra có một nhánh if (rẽ nhánh, bài "Rẽ nhánh if"), một lời gọi hàm (bài "Gọi hàm"), và hai đường trả về khác nhau.'
- kind: strategy
  body: 'Đi từng dòng của kiem_tra — `if n > 0:` cần ĐỌC n, ĐỌC hằng 0, SO SÁNH, rồi NHẢY CÓ ĐIỀU KIỆN nếu sai. Nhánh đúng: ĐỌC hàm tang_gap_doi, ĐỌC n, GỌI, TRẢ VỀ. Nhánh sai (rơi tới nhãn nhảy): TRẢ VỀ hằng số 0 — dùng RETURN_CONST, không phải LOAD_CONST + RETURN_VALUE, vì đây là một hằng số trả thẳng.'
- kind: one-line
  body: 'du_doan = ["LOAD_FAST", "LOAD_CONST", "COMPARE_OP", "POP_JUMP_IF_FALSE", "LOAD_GLOBAL", "LOAD_FAST", "CALL", "RETURN_VALUE", "RETURN_CONST"]'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: "^True\\n\\['LOAD_FAST', 'LOAD_CONST', 'COMPARE_OP', 'POP_JUMP_IF_FALSE', 'LOAD_GLOBAL', 'LOAD_FAST', 'CALL', 'RETURN_VALUE', 'RETURN_CONST'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`True` — bản dịch tay của bạn khớp máy thật, từng lệnh một. Không phải
đoán suông: bạn vừa tự kiểm bằng đúng máy đang chạy mã của bạn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mọi lệnh bạn vừa dịch tay — `LOAD_FAST`, `BINARY_OP`, `CALL` — đều thao
tác trên MỘT ngăn xếp tính toán, nói tới từ rất sớm trong track này.
Nhưng ngăn xếp ấy VẬT LÝ nằm ở đâu trong một cỗ máy thật? Bộ nhớ có địa
chỉ, dãy ô đánh số, đã học từ T3.1 — hay một chỗ khác hẳn, còn nhanh
hơn cả bước đọc một địa chỉ?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
