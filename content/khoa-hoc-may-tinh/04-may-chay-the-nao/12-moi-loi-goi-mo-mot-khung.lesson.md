---
id: khoa-hoc-may-tinh.may-chay-the-nao.moi-loi-goi-mo-mot-khung
title: "Mỗi lời gọi mở một khung mới"
summary: "sys._getframe() cho xem tận mắt một khung (frame) thật: f_code.co_name (tên hàm), f_locals (biến cục bộ RIÊNG của khung), f_back (khung đã gọi nó), f_lasti (con trỏ lệnh riêng). Đúng thứ 'tờ phiếu' R1.T1.3 đã dạy — CALL (bài 11) mở nó ra, mỗi lời gọi một khung độc lập."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.call-frame]
requires: [may.call-instruction, func.call-stack]
concepts: [may.call-frame]
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
Tờ phiếu ấy có một tên chính thức. Và bạn xem tận mắt được nó giữ gì
bên trong.
::::

::::explain{#khung-la-gi}
Bài trước: `CALL` là lệnh gọi hàm. "Máy nhớ đường về" đã nói: mỗi lần
gọi, máy đặt một "tờ phiếu" lên chồng lời gọi — ghi tên hàm, những giá
trị nó đang cầm, và dòng nào đang đợi kết quả.

Tên chính thức của tờ phiếu ấy, trong tiếng lập trình: **khung**
(frame). `CALL` không chỉ nhảy tới hàm khác — nó DỰNG một khung mới, và
khung ấy giữ đúng ba thứ "Máy nhớ đường về" đã liệt kê, chỉ khác giờ có
tên trường dữ liệu thật, xem được bằng module `sys`:

- `f_code.co_name` — tên hàm đang chạy trong khung này.
- `f_locals` — biến cục bộ CỦA RIÊNG khung này, một cuốn sổ không hàm
  nào khác đọc hay ghi được.
- `f_back` — khung đã GỌI khung này, chính là "biết quay về đâu" trong
  câu chuyện tờ phiếu.
- `f_lasti` — con trỏ lệnh (bài 7) CỦA RIÊNG khung này: mỗi khung tự
  nhớ nó đang đứng ở lệnh nào, không dùng chung một con trỏ với khung
  khác.

`sys._getframe()` là một hàm trong module `sys`, trả về đúng khung
ĐANG CHẠY tại chỗ nó được gọi — không phải mô phỏng, là khung thật của
trình thông dịch.
::::

::::example{#doc-mot-khung-that}
```python title=readonly
import sys

def trong(a, b):
    khung = sys._getframe()
    print("tên hàm khung này đang chạy:", khung.f_code.co_name)
    print("biến cục bộ a của khung này:", khung.f_locals["a"])
    print("khung đã GỌI khung này:", khung.f_back.f_code.co_name)
    print("con trỏ lệnh của khung này:", khung.f_lasti)
    print("con trỏ lệnh của khung NGOÀI (đang đợi):", khung.f_back.f_lasti)
    return a + b

def ngoai(x, y):
    return trong(x, y)

print(ngoai(3, 4))
```

```text title=readonly
tên hàm khung này đang chạy: trong
biến cục bộ a của khung này: 3
khung đã GỌI khung này: ngoai
con trỏ lệnh của khung này: 256
con trỏ lệnh của khung NGOÀI (đang đợi): 14
7
```

Hai con trỏ lệnh khác hẳn nhau — 256 với 14 — vì đó là hai khung KHÁC
NHAU, mỗi khung đứng ở một chỗ riêng trong danh sách lệnh của chính nó.
`trong` đang chạy giữa thân hàm; `ngoai` đang đứng yên đợi ngay sau
`CALL`, tại đúng dòng đã gọi `trong`. Con số cụ thể không quan trọng để
nhớ — điều đáng nhớ là MỖI khung có con trỏ CỦA RIÊNG NÓ, không chia sẻ
với khung nào khác.
::::

::::predict{#khung-song-sau-khi-tra-ve commitOnce}
```python
luu = {}

def dem_xuong(n):
    luu[n] = sys._getframe()
    if n == 0:
        return
    dem_xuong(n - 1)

dem_xuong(4)
print(luu[2].f_locals["n"])
print(luu[0].f_locals["n"])
```

Mọi lời gọi `dem_xuong` đã CHẠY XONG HẲN — `dem_xuong(4)` đã trả về từ
lâu — trước khi hai dòng `print` cuối cùng chạy. Hai dòng ấy in ra gì?

:::opt{correct}
`2`, rồi `0`
:::

:::opt
`0`, rồi `0`
::why
Gần đúng ở việc bạn tin `luu` là một từ điển dùng chung — đúng, `luu`
CHỈ là một từ điển bình thường, không có gì đặc biệt.

Chỗ lệch: bạn đang nghĩ mọi khung được LƯU trong đó chia sẻ CHUNG một
`f_locals`, bị ghi đè bởi lượt gọi cuối cùng (`n == 0`). Nhưng mỗi khoá
`luu[2]` và `luu[0]` đang giữ hai KHUNG KHÁC NHAU — hai đối tượng riêng
biệt, mỗi khung có `f_locals` của chính nó, không hàm nào ghi đè lên
`f_locals` của một khung khác.
::
:::

:::opt
`4`, rồi `4`
::why
Gần đúng ở việc bạn nhận ra mỗi khung giữ ĐÚNG một giá trị riêng, không
bị lẫn với khung khác — đúng tinh thần bài này.

Chỗ lệch: bạn đang nghĩ mỗi khung nhớ giá trị `n` LÚC `dem_xuong` bắt
đầu được gọi LẦN ĐẦU TIÊN trong toàn chuỗi (`4`), không phải giá trị
`n` mà CHÍNH khung đó đang cầm. `luu[2]` là khung của LƯỢT GỌI
`dem_xuong(2)` — biến cục bộ `n` của lượt gọi đó vốn dĩ là `2` ngay từ
khi lượt gọi ấy bắt đầu, không phải `4`.
::
:::

:::opt
Máy báo lỗi — khung của một lời gọi đã trả về không còn tồn tại để đọc
nữa
::why
Gần đúng ở sự thận trọng: một khung ĐÃ ĐÓNG không còn ĐANG CHẠY, đúng
như bài này dạy.

Chỗ lệch: "không còn đang chạy" không có nghĩa là đối tượng khung biến
mất khỏi bộ nhớ. `luu` vẫn giữ một tham chiếu tới từng khung, nên đối
tượng khung ấy còn sống, và `f_locals` của nó vẫn đọc được bình thường
— chỉ là khung đó không còn là nơi trình thông dịch đang đứng để chạy
tiếp.
::
:::
::::

::::code{#khung-nao-goi-toi}
Hoàn thiện `sau(a, b)` để nó trả về một cặp: tên hàm đang chạy (chính
`sau`), và tên hàm ĐÃ GỌI `sau` — dùng `f_back`.

```python title=starter
import sys

def sau(a, b):
    khung = sys._getframe()
    ten_ham_hien_tai = khung.f_code.co_name
    ten_ham_da_goi = ___                    # tên hàm đã GỌI hàm sau này
    return ten_ham_hien_tai, ten_ham_da_goi

def truoc(x, y):
    return sau(x, y)

def khac(x, y):
    return sau(x, y)

print(truoc(3, 4))
print(khac(5, 6))
```

```python title=solution
import sys

def sau(a, b):
    khung = sys._getframe()
    ten_ham_hien_tai = khung.f_code.co_name
    ten_ham_da_goi = khung.f_back.f_code.co_name
    return ten_ham_hien_tai, ten_ham_da_goi

def truoc(x, y):
    return sau(x, y)

def khac(x, y):
    return sau(x, y)

print(truoc(3, 4))
print(khac(5, 6))
```

```python title=test
assert truoc(3, 4) == ("sau", "truoc"), f"gọi từ truoc, khung của sau phải báo đúng ('sau', 'truoc') — đang ra {truoc(3, 4)}"
assert khac(5, 6) == ("sau", "khac"), f"gọi từ khac, khung của sau phải báo đúng ('sau', 'khac') — đang ra {khac(5, 6)}; nếu vẫn ra 'truoc' thì bạn đang dùng một tên hàm chép cứng, không đọc thật f_back"
```

:::hints
- kind: attention
  body: Chỗ trống chỉ cần lấy tên hàm của khung ĐÃ GỌI khung hiện tại — dòng ngay trên nó đã lấy tên của khung hiện tại bằng khung.f_code.co_name, chỗ trống làm y hệt nhưng trên một khung khác.
- kind: strategy
  body: 'Khung ĐÃ GỌI khung hiện tại là khung.f_back. Thêm .f_code.co_name vào sau để lấy tên hàm của khung ấy — đúng công thức khung hiện tại đã dùng, chỉ đổi khung.'
- kind: one-line
  body: 'Chỗ trống là: khung.f_back.f_code.co_name'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\('sau', 'truoc'\\)\\n\\('sau', 'khac'\\)\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một hàm sau, hai khung khác nhau, hai câu trả lời khác nhau — mỗi
khung thật sự nhớ đúng ai đã gọi nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Khung mở ra đúng lúc `CALL` chạy (bài trước). Nó ĐÓNG lại đúng lúc nào
— và giá trị nó đang cầm trong tay đi đâu, khi hàm kết thúc?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
