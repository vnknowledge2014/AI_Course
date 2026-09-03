---
id: co-so-du-lieu.bon-chu-cai-acid.phat-hien-deadlock
title: Phát hiện deadlock
summary: "co_deadlock_hai_ben kiểm tra ĐÚNG một cặp giao dịch — a CÓ đang chờ b VÀ b CÓ đang chờ a hay không, cả hai điều kiện phải ĐÚNG cùng lúc mới là deadlock thật. Hàm này CHỈ phát hiện vòng tròn HAI node trực tiếp — một vòng tròn DÀI hơn (ba giao dịch trở lên) sẽ bị bỏ SÓT nếu chỉ kiểm từng cặp."
locale: vi
track: co-so-du-lieu
module: bon-chu-cai-acid
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [db.deadlock-detection]
requires: [db.deadlock-idea]
concepts: [db.deadlock-detection]
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
Con người vẽ đồ thị chờ (bài TRƯỚC) bằng mắt dễ thấy vòng TRÒN.
Máy tính kiểm tra điều đó bằng CODE thế nào — với đúng MỘT cặp giao
dịch?
::::

::::explain{#kiem-tra-cap}
`co_deadlock_hai_ben` kiểm tra ĐÚNG một cặp — `a` CÓ đang chờ `b`
VÀ `b` CÓ đang chờ `a` hay KHÔNG, cả hai điều kiện phải ĐÚNG cùng
lúc mới LÀ deadlock thật:

```python title=readonly
def co_deadlock_hai_ben(do_thi_cho, a, b):
    a_cho_b = b in do_thi_cho.get(a, set())
    b_cho_a = a in do_thi_cho.get(b, set())
    return a_cho_b and b_cho_a


print(co_deadlock_hai_ben({"T1": {"T2"}, "T2": {"T1"}}, "T1", "T2"))
```

```text title=readonly
True
```

`do_thi_cho["T1"] = {"T2"}` — `a_cho_b` (`"T2" in {"T2"}`) LÀ
`True`. `do_thi_cho["T2"] = {"T1"}` — `b_cho_a` (`"T1" in
{"T1"}`) CŨNG `True`. Cả HAI đúng cùng lúc — mũi tên đi VÀ mũi tên
về, MỘT vòng tròn hai chiều thật SỰ — deadlock được xác nhận.
::::

::::example{#chi-mot-chieu-khong-phai-deadlock}
Một chuỗi chờ MỘT chiều (`T1` chờ `T2`, `T2` chờ `T3` — KHÔNG chờ
`T1`) — kiểm tra cặp `(T1, T2)` KHÔNG báo deadlock:

```python title=readonly
print(co_deadlock_hai_ben({"T1": {"T2"}, "T2": {"T3"}}, "T1", "T2"))
```

```text title=readonly
False
```

`a_cho_b` (`"T2" in {"T2"}`) LÀ `True` — `T1` ĐÚNG là đang chờ
`T2`. NHƯNG `b_cho_a` (`"T1" in {"T3"}`) LÀ `False` — `T2` đang chờ
`T3`, KHÔNG hề chờ `T1`. Chỉ MỘT chiều mũi tên — `T2` vẫn CÓ thể
tiến lên (một khi `T3` xong việc), không hề bị kẹt CỨNG với `T1`.
::::

::::predict{#doan-vong-tron-ba-node commitOnce}
Một vòng tròn DÀI hơn — `T1` chờ `T2`, `T2` chờ `T3`, `T3` chờ LẠI
`T1` (một deadlock THẬT, chỉ dài BA bước thay vì hai):

```python
do_thi = {"T1": {"T2"}, "T2": {"T3"}, "T3": {"T1"}}
print(co_deadlock_hai_ben(do_thi, "T1", "T2"))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì `T1` VÀ `T2` đều NẰM trong một vòng tròn deadlock THẬT
sự (dù dài ba bước), NÊN kiểm tra cặp `(T1, T2)` phải phát HIỆN ra
::why
Gần đúng ở việc bạn nhận RA đúng: `{"T1":{"T2"}, "T2":{"T3"},
"T3":{"T1"}}` LÀ một vòng tròn deadlock THẬT — một quan sát chính
xác VỀ toàn cảnh đồ thị.

Chỗ lệch: `co_deadlock_hai_ben` CHỈ kiểm tra ĐÚNG hai điều kiện
trực tiếp giữa `a` VÀ `b` — `b_cho_a` hỏi `"T1" in
do_thi_cho.get("T2", set())`, TỨC `"T1" in {"T3"}`, LÀ `False`.
`T2` chờ `T3`, KHÔNG chờ TRỰC tiếp `T1` — hàm nÀY hoàn toàn KHÔNG
"nhìn" xa hơn một bước, nên bỏ SÓT vòng tròn dài hơn hai node.
::
:::

:::opt
Máy báo lỗi — vì `do_thi_cho.get("T3", set())` không hề được TRUY
cập trong lệnh gọi NÀY, khiến hàm thiếu DỮ liệu cần
::why
Gần đúng ở việc bạn để Ý `"T3"` không được TRUY cập trực tiếp
trong lệnh GỌI `co_deadlock_hai_ben(do_thi, "T1", "T2")` — một quan
sát ĐÚNG về luồng thực thi.

Chỗ lệch: hàm không CẦN truy cập `"T3"` để CHẠY xong — nó chỉ đọc
`do_thi_cho.get("T1", ...)` VÀ `do_thi_cho.get("T2", ...)`, cả hai
đều CÓ mặt trong `do_thi`, không hề THIẾU dữ liệu, không lỗi gì.
::
:::
::::

::::code{#viet_co_deadlock_hai_ben}
Hoàn thiện `co_deadlock_hai_ben` — deadlock CHỈ xảy ra khi CẢ hai
chiều chờ đều đúng CÙNG lúc.

```python title=starter
def co_deadlock_hai_ben(do_thi_cho, a, b):
    a_cho_b = b in do_thi_cho.get(a, set())
    b_cho_a = a in do_thi_cho.get(b, set())
    ___


print(co_deadlock_hai_ben({"T1": {"T2"}, "T2": {"T1"}}, "T1", "T2"))
```

```python title=solution
def co_deadlock_hai_ben(do_thi_cho, a, b):
    a_cho_b = b in do_thi_cho.get(a, set())
    b_cho_a = a in do_thi_cho.get(b, set())
    return a_cho_b and b_cho_a


print(co_deadlock_hai_ben({"T1": {"T2"}, "T2": {"T1"}}, "T1", "T2"))
```

```python title=test
assert co_deadlock_hai_ben({"T1": {"T2"}, "T2": {"T1"}}, "T1", "T2") == True, "vong tron hai chieu that su -- phai la deadlock"
assert co_deadlock_hai_ben({"T1": {"T2"}}, "T1", "T2") == False, "chi mot chieu -- khong phai deadlock"
assert co_deadlock_hai_ben({"T1": {"T2"}, "T2": {"T3"}}, "T1", "T2") == False, "T2 cho T3 chu khong cho T1 -- khong deadlock giua T1 va T2"
assert co_deadlock_hai_ben({}, "T1", "T2") == False, "do thi rong -- khong ai cho ai ca"
assert co_deadlock_hai_ben({"T1": {"T2"}, "T2": {"T3"}, "T3": {"T1"}}, "T1", "T2") == False, "vong tron ba node -- kiem tra cap truc tiep khong thay"
```

:::hints
- kind: attention
  body: "Dong con thieu tra ve ket qua cuoi cung: ca hai chieu cho phai dung CUNG luc, dung AND."
- kind: strategy
  body: "return a_cho_b and b_cho_a"
- kind: one-line
  body: "return a_cho_b and b_cho_a"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai return a_cho_b and b_cho_a
  requireAst:
  - kind: uses-name, target: a_cho_b, min: 1
  - kind: uses-name, target: b_cho_a, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Phát hiện được deadlock giữa MỘT cặp — dù nó có thể bỏ SÓT vòng
tròn dài hơn. Ghép TẤT cả lại: khoá, 2PL, phát hiện deadlock, VÀ
giải quyết nó khi xảy ra?
::::

::::reflect{#nghi-lai}
`co_deadlock_hai_ben` phát hiện ĐÚNG vòng tròn hai chiều TRỰC tiếp
— cả `a` chờ `b` LẪN `b` chờ `a`. Nó KHÔNG "nhìn" xa hơn một bước,
nên bỏ SÓT vòng tròn dài hơn (ba giao dịch trở LÊN) — một hệ thống
THẬT cần dò xa hơn (đi theo TỪNG mũi tên, không chỉ kiểm MỘT cặp).
Nhưng phát hiện thôi CHƯA đủ — phát hiện XONG rồi làm gì để hai
giao dịch KẸT nhau có thể tiếp tục?
::::

::::checkpoint{mastery=0.8}
::::
