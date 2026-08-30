---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.python-khong-tran
title: Vì sao Python không bao giờ tràn
summary: "Số nguyên của Python không khoá ở một số ô cố định — khi một số cần nhiều chỗ hơn, nó tự xin thêm ô, nên phép cộng thường của Python không bao giờ tràn như khung tám-ô bài trước."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [mem.arbitrary-precision]
requires: [mem.overflow]
concepts: [mem.so-tuy-y-lon, mem.cap-them-o]
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
Bạn vừa hỏi một câu hay. Hôm nay mình cho bạn nhìn thẳng vào chỗ chứa.
::::

::::explain{#khong-khoa-o-tam}
Bài trước, khung tám-ô của bạn hết chỗ ở `256` vì nó bị KHOÁ CỨNG ở đúng
tám ô — không hơn, không kém, dù kết quả có cần thêm bao nhiêu. Số nguyên
thường của Python không đi theo luật đó.

Python không khoá số nguyên ở tám ô, cũng không khoá ở 32 hay 64 ô như
nhiều ngôn ngữ khác vẫn làm. Khi một số cần nhiều chỗ hơn để đứng, Python
tự CẤP THÊM chỗ — không đợi bạn xin, không báo lỗi, không cắt bớt gì cả.
Cách này gọi là **số nguyên độ chính xác tuỳ ý** (arbitrary precision): độ
lớn không có trần, chỉ có chừng nào bộ nhớ của máy còn đưa được.

Bạn đã thấy hệ quả của điều này từ track T2.2, khi cộng và nhân những con
số có hàng chục chữ số mà không hề gặp lỗi tràn nào. Giờ là lúc nhìn vào
chỗ chứa thật của nó.
::::

::::example{#do-cho-chua-bang-getsizeof}
Python có một hàm hỏi thẳng "cái tên này đang chiếm mấy byte trong bộ
nhớ": `sys.getsizeof(...)`.

```python title=readonly
import sys

print(sys.getsizeof(0))
print(sys.getsizeof(2**30))
print(sys.getsizeof(2**300))
```

Trên máy chạy khoá học này — đo đúng lúc viết bài — ba dòng trên in ra
`16`, `20`, `56`. Con số CHÍNH XÁC có thể xê dịch đôi chút nếu Python đổi
cách cài đặt bên trong; đừng nhớ ba con số này. Thứ đáng nhớ là chuyện
KHÔNG đổi: dòng dưới luôn lớn hơn dòng trên nó.

`2**30` là một số hơn một tỷ — nhiều chữ số hơn hẳn `0` — nên nó cần nhiều
ô hơn để đứng, và Python cấp cho nó nhiều ô hơn thật, không cắt bớt như
khung tám-ô bài trước. `2**300` còn lớn hơn nữa — một con số 91 chữ số —
nên chỗ chứa của nó lại lớn hơn thêm một lần nữa.

Không có khung nào bị vượt qua cả, vì không có khung cố định nào để vượt.
::::

::::predict{#so-khong-lo commitOnce}
Byte tính một con số rất lớn — `2**1000`, tức 2 nhân với chính nó một
nghìn lần — rồi in nó ra.

**Trước khi bấm chạy**, bạn đoán chuyện gì xảy ra?

```python
print(2**1000)
```

:::opt{correct}
Máy in ra trọn vẹn con số đó — hơn ba trăm chữ số, không thiếu chữ số nào,
không có lỗi gì cả.
:::

:::opt
Máy báo lỗi tràn số, vì `2**1000` quá lớn để một biến chứa nổi.
::why
Gần đúng ở chỗ nhiều ngôn ngữ THẬT SỰ báo lỗi này — C, Java hay Rust đều
khoá số nguyên ở một số ô cố định (thường 32 hay 64 ô), và một con số cần
hơn ba trăm chữ số thập phân chắc chắn làm chúng tràn hoặc báo lỗi ngay.

Chỗ lệch là bạn đang áp luật của những ngôn ngữ đó vào Python. Số nguyên
Python không khoá ở một số ô nào — nó cấp thêm ô khi cần, đúng như bài này
vừa dựng lại bằng `sys.getsizeof`. Không có trần để mà tràn qua.
::
:::

:::opt
Máy in ra một con số SAI — bị cắt bớt giống hàm bài trước — vì mọi con số
trong máy tính đều phải đóng khung trong một số ô nhất định.
::why
Gần đúng ở chỗ nửa đầu đúng thật: mọi con số trong máy đều nằm trong những
cái ô có thật, không có ngoại lệ. Bài `mem.byte-range` và bài trước đều xây
trên sự thật đó.

Chỗ lệch là "một số ô NHẤT ĐỊNH". Khung tám-ô bài trước khoá số ô LẠI, nên
nó cắt. Số nguyên Python không khoá số ô — nó xin thêm ô mỗi khi thiếu, nên
không có gì để cắt cả. Cùng nằm trong ô, khác ở việc số ô có bị khoá hay
không.
::
:::

:::opt
Máy treo hoặc chạy rất lâu, vì con số quá lớn để tính trong thời gian
ngắn.
::why
Gần đúng ở chỗ trực giác của bạn hợp lý: con số hơn ba trăm chữ số nghe có
vẻ nặng.

Chỗ lệch là phép nhân đôi liên tiếp một nghìn lần không hề chậm — máy làm
xong gần như tức thời. "Số cần nhiều ô hơn" không có nghĩa là "máy phải
tính lâu hơn nhiều"; cấp thêm chỗ chứa là một việc rẻ, không phải một việc
chậm.
::
:::
::::

::::code{#so-sanh-cho-chua}
Viết một hàm hỏi đúng câu mà `sys.getsizeof` trả lời được: số này có cần
NHIỀU Ô HƠN số kia để chứa không?

```python title=starter
import sys


def can_them_o_khong(so_nho, so_to):
    """True nếu SO_TO cần nhiều ô nhớ hơn SO_NHO để chứa."""
    return sys.getsizeof(so_to) ___ sys.getsizeof(so_nho)


print(can_them_o_khong(0, 2**300))
print(can_them_o_khong(2**30, 2**30))
```

```python title=solution
import sys


def can_them_o_khong(so_nho, so_to):
    """True nếu SO_TO cần nhiều ô nhớ hơn SO_NHO để chứa."""
    return sys.getsizeof(so_to) > sys.getsizeof(so_nho)


print(can_them_o_khong(0, 2**300))
print(can_them_o_khong(2**30, 2**30))
```

```python title=test
# Không assert nào ghim một con số getsizeof tuyệt đối — đúng luật của
# track này. Năm cảnh so nhiều chiều: xuôi, ngược, và một cảnh HAI SỐ CÙNG
# HẠNG, để dấu >= (thay vì >) cũng phải lộ mặt.
assert can_them_o_khong(0, 2**300) is True, "2**300 cần nhiều chữ số hơn hẳn 0, nên nó phải cần nhiều ô hơn"
assert can_them_o_khong(2**30, 2**300) is True, "2**300 vẫn lớn hơn 2**30 rất nhiều, vẫn cần nhiều ô hơn"
assert can_them_o_khong(2**300, 2**30) is False, "đổi chiều: SO_TO giờ là 2**30, nhỏ hơn SO_NHO là 2**300 — nó không cần thêm ô nào cả"
assert can_them_o_khong(5, 2**60) is True, "2**60 cần nhiều ô hơn hẳn con số 5 bé tí"
assert can_them_o_khong(2**30, 2**30) is False, "hai số bằng nhau thì chỗ chứa bằng nhau — không số nào 'cần thêm', nên kết quả phải là False, không phải True"
```

:::hints
- kind: attention
  body: Chỗ trống là một dấu so sánh, đứng giữa hai lời gọi `sys.getsizeof`. Đọc lại câu hỏi hàm này phải trả lời — "SO_TO có cần NHIỀU ô HƠN không" — rồi tìm đúng dấu cho câu hỏi đó.
- kind: strategy
  body: "'Nhiều hơn' là so sánh LỆCH, không phải so sánh BẰNG HAY HƠN. Cảnh cuối của bài test cho hai số bằng nhau — nếu dấu bạn chọn coi bằng nhau cũng là 'cần thêm ô' thì cảnh đó sẽ báo sai."
- kind: one-line
  body: 'Điền dấu `>`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: hàm phải trả lời bằng một phép so sánh LỆCH giữa hai getsizeof (con số này thật sự lớn hơn con số kia) — không phải so sánh không-lệch (`>=`, vốn coi hai số bằng nhau cũng là "cần thêm ô")
  requireAst:
  - kind: uses-operator, target: '>', min: 1
  - kind: uses-call, target: getsizeof, min: 2
  forbidAst:
  - kind: uses-operator, target: '>='
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Số to hơn thì chỗ chứa to theo. Không phép màu — chỉ là cấp thêm ô.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy Python cấp thêm ô khi một số cần. Nhưng "ô" không chỉ để đếm
to nhỏ — quay lại khung tám-ô của hai bài trước, MỖI ô trong đó còn mang
một câu trả lời riêng: bật hay tắt, `1` hay `0`.

Bài `logic.and`/`logic.or` của Realm 1 dạy bạn gộp HAI câu trả lời có–không
thành một câu trả lời mới, bằng `and` và `or`. Nếu một byte là TÁM câu trả
lời có–không đứng liền nhau, gộp cùng lúc cả tám câu ấy bằng `and`/`or`
được không?

Bài sau trả lời — và câu trả lời không hẳn là bạn nghĩ.
::::

::::checkpoint{mastery=0.8}
::::
