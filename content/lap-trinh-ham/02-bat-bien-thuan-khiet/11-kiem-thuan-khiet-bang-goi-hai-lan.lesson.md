---
id: lap-trinh-ham.bat-bien-thuan-khiet.kiem-thuan-khiet-bang-goi-hai-lan
title: "Kiểm một hàm có thuần không: gọi HAI LẦN, so kết quả"
summary: "assert f(2, 3) == f(2, 3) không chứng minh được tuyệt đối, nhưng là phép kiểm nhanh nhất trong thực tế. Kết hợp với tầng static (pure-fn) dò trong THÂN hàm — hai lớp bổ sung nhau, không lớp nào một mình đủ."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.test-purity]
requires: [fp.dependency-injection]
concepts: [fp.test-purity]
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
Không cần đọc từng dòng thân hàm. Gọi nó hai lần, so hai kết quả — Byte
chỉ bạn phép kiểm nhanh nhất.
::::

::::explain{#goi-hai-lan-so-ket-qua}
Định nghĩa hàm thuần (bài 7) nói: cùng đối số, luôn cùng kết quả. Từ
đúng câu đó suy ra một phép kiểm cực đơn giản — không cần hiểu hàm làm
gì bên trong, chỉ cần GỌI nó hai lần với đúng cùng đối số, rồi so hai
kết quả:

```python
assert f(2, 3) == f(2, 3), "gọi hai lần cùng đối số phải ra cùng kết quả"
```

Nếu `assert` này ném lỗi, `f` CHẮC CHẮN không thuần — không có hàm thuần
nào có thể làm `assert` này trượt. Nhưng chiều ngược lại không chắc: nếu
`assert` này ĐẬU, `f` chưa chắc đã thuần. Một hàm đọc `random.randint`
vẫn có thể TÌNH CỜ rút trúng cùng một số hai lần liên tiếp — phép kiểm
này không phân biệt được "thuần thật" với "tình cờ trùng kết quả một
lần". Đây là phép kiểm THỰC TẾ, nhanh và rẻ, không phải một PHÉP CHỨNG
MINH.

Track này có thêm một công cụ khác, mạnh hơn kiểu kiểm bằng cách gọi
thử: tầng `static` với luật `pure-fn`, dò thẳng vào MÃ NGUỒN của thân
hàm để tìm bốn dấu hiệu không thuần — gọi `global`/`nonlocal`, gọi hàm
xuất/nhập (`print`, `input`, `open`...), gọi nguồn không tất định
(`random.*`, `datetime.now`...), và sửa dữ liệu tại chỗ. Luật này cũng
có giới hạn riêng: nó KHÔNG chứng minh được purity qua một hàm PHỤ được
gọi gián tiếp bên trong — nếu thân hàm gọi `ham_khac(...)` và
`ham_khac` không thuần, `pure-fn` không lần theo được vào bên trong
`ham_khac` để phát hiện.

Hai lớp kiểm — gọi hai lần, và dò mã nguồn — bắt được những lỗi KHÁC
NHAU. Gọi hai lần bắt được nondeterminism thật sự lộ ra qua kết quả.
Dò mã nguồn bắt được side-effect (như `print`) dù kết quả trả về vẫn
đúng mỗi lần — thứ mà chỉ gọi hai lần và so kết quả KHÔNG BAO GIỜ phát
hiện ra, vì side-effect đó không đổi giá trị trả về. Dùng cả hai, không
lớp nào một mình đủ.
::::

::::example{#mot-ham-that-sao-hai-lop}
Byte viết một hàm tính giá sau chiết khấu, rồi kiểm nó bằng cả hai
cách.

```python title=readonly
def tinh_gia_cuoi(gia, phan_tram_giam):
    return gia * (1 - phan_tram_giam / 100)

# Lớp 1: gọi hai lần, so kết quả
assert tinh_gia_cuoi(100000, 10) == tinh_gia_cuoi(100000, 10)
print("Lớp 1 (gọi hai lần): đậu")
```

```text title=readonly
Lớp 1 (gọi hai lần): đậu
```

`tinh_gia_cuoi` đậu lớp 1 — hai lần gọi cùng `(100000, 10)` ra cùng
`90000.0`. Nếu hàm này còn có thêm một dòng `print(f"tính giá cho
{gia}")` ở đầu thân hàm, lớp 1 VẪN đậu y hệt — kết quả trả về không hề
đổi, chỉ có màn hình bị in thêm một dòng mỗi lần gọi. Đây đúng là chỗ
lớp 2 (dò mã nguồn qua `pure-fn`) cần vào cuộc: nó nhìn thẳng vào thân
hàm, thấy lời gọi `print`, và báo hỏng — bất kể kết quả trả về có đúng
hay không.
::::

::::predict{#doan-lop-nao-bat-duoc commitOnce}
Ba hàm dưới đây đều tính đúng giá sau chiết khấu — gọi với cùng đối số
đều ra cùng con số. Nhưng chỉ một trong ba hàm THẬT SỰ thuần.

```python
def phien_ban_a(gia, giam):
    return gia * (1 - giam / 100)

def phien_ban_b(gia, giam):
    print(f"đang tính cho {gia}")
    return gia * (1 - giam / 100)

so_lan_goi = 0
def phien_ban_c(gia, giam):
    global so_lan_goi
    so_lan_goi += 1
    return gia * (1 - giam / 100)
```

**Trước khi đọc tiếp**, phép kiểm "gọi hai lần, so kết quả" — MỘT MÌNH,
không có tầng `static` — có phân biệt được `phien_ban_a` với hai phiên
bản còn lại không?

:::opt{correct}
Không — cả ba hàm đều đậu phép kiểm "gọi hai lần" như nhau, vì cả ba
đều trả về CÙNG kết quả với CÙNG đối số; chỉ có việc dò mã nguồn (dấu
`print`, dấu `global`) mới phân biệt được `phien_ban_a` với hai hàm kia
:::

:::opt
Có — `phien_ban_b` và `phien_ban_c` sẽ làm `assert f(x) == f(x)` ném
lỗi, vì thân hai hàm đó dài hơn `phien_ban_a`
::why
Gần đúng ở việc bạn để ý đúng hai hàm kia có thêm dòng lệnh — quan sát
đó không sai.

Chỗ lệch: độ dài thân hàm không quyết định GIÁ TRỊ TRẢ VỀ. Cả ba hàm
đều kết thúc bằng đúng một dòng `return gia * (1 - giam / 100)`, với
đúng công thức đó — gọi hai lần cùng đối số, cả ba đều ra đúng cùng một
số. Dòng `print` hay dòng `global ...; so_lan_goi += 1` chạy TRƯỚC dòng
`return`, không đổi con số mà `return` đưa ra.
::
:::

:::opt
Có — `phien_ban_c` sẽ ném lỗi `NameError`, vì `so_lan_goi` được khai
`global` bên trong hàm nhưng gán giá trị ở NGOÀI hàm
::why
Gần đúng ở việc bạn nhận ra `so_lan_goi` xuất hiện ở CẢ hai chỗ — bên
ngoài (`so_lan_goi = 0`) và bên trong (`global so_lan_goi`).

Chỗ lệch: đó chính xác là cách dùng `global` ĐÚNG — khai `so_lan_goi =
0` ở phạm vi module, rồi khai `global so_lan_goi` bên trong hàm để hàm
được phép GÁN LẠI (qua `+=`) cái tên đó. Không có `NameError` nào cả;
`phien_ban_c` chạy trót lọt, chỉ là nó có side-effect (tăng bộ đếm) mà
`assert f(x) == f(x)` không thấy được.
::
:::

:::opt
Có — cả `phien_ban_b` lẫn `phien_ban_c` đều báo lỗi `SyntaxError`, vì
Python không cho phép `print` hoặc `global` đứng trước `return` trong
cùng một hàm
::why
Gần đúng ở việc bạn nghĩ tới khả năng lỗi cú pháp — một phản xạ tốt khi
gặp mã lạ.

Chỗ lệch: không có luật nào như vậy trong Python. Một hàm được phép
chứa nhiều câu lệnh theo bất kỳ trình tự hợp lệ nào trước `return` —
`print(...)`, khai `global ...`, rồi `return ...` đều là những câu lệnh
bình thường, không câu nào xung đột cú pháp với câu nào.
::
:::
::::

::::code{#kiem-hai-lop-that}
Viết `tinh_gia_cuoi(gia, phan_tram_giam)` — tính giá sau khi giảm đúng
`phan_tram_giam` phần trăm. Hàm phải THUẦN: không `print`, không
`global`, không sửa dữ liệu ngoài, chỉ tính rồi trả về.

```python title=starter
def tinh_gia_cuoi(gia, phan_tram_giam):
    ___

print(tinh_gia_cuoi(100000, 10))
```

```python title=solution
def tinh_gia_cuoi(gia, phan_tram_giam):
    return gia * (1 - phan_tram_giam / 100)

print(tinh_gia_cuoi(100000, 10))
```

```python title=test
assert tinh_gia_cuoi(100000, 10) == 90000.0, "100000 giảm 10% phải ra 90000.0"
assert tinh_gia_cuoi(200000, 50) == 100000.0, "200000 giảm 50% phải ra 100000.0"
assert tinh_gia_cuoi(100000, 10) == tinh_gia_cuoi(100000, 10), "gọi hai lần cùng đối số phải ra cùng kết quả — đây là phép kiểm thuần khiết lớp 1"
```

:::hints
- kind: attention
  body: Chỗ trống thay cho TOÀN BỘ thân hàm — chỉ cần một dòng return, không print, không global, không biến phụ nào ở ngoài hàm.
- kind: strategy
  body: 'Giảm phan_tram_giam phần trăm nghĩa là còn lại (100 - phan_tram_giam) phần trăm. Viết dưới dạng phân số: gia * (1 - phan_tram_giam / 100).'
- kind: one-line
  body: "Điền `return gia * (1 - phan_tram_giam / 100)` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hàm tinh_gia_cuoi phải THUẦN — tầng static phát hiện một dấu hiệu không thuần trong thân hàm (print, global/nonlocal, gọi nguồn không tất định, hoặc sửa dữ liệu tại chỗ). Chỉ tính công thức rồi return, đừng thêm gì khác.
  requireAst:
  - kind: pure-fn, target: tinh_gia_cuoi
  - kind: uses-operator, target: "*", min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "90000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Gọi hai lần đậu, dò mã nguồn cũng đậu — hai lớp, cùng đồng ý. Giờ bạn có
đủ cả hai cách để KIỂM một hàm, không chỉ VIẾT nó cho đúng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Năm bài vừa qua (7-11) mổ xẻ riêng từng mảnh của hàm thuần: định nghĩa,
sự không tất định, side-effect qua `print`, tiêm phụ thuộc, và cách
kiểm. Nhưng track này còn một nửa nữa chưa nối lại — sáu bài ĐẦU (1-6)
dạy về DỮ LIỆU bất biến: `frozen=True`, `replace()`.

Một hàm nhận một đối tượng `frozen=True` làm tham số, trả về một đối
tượng MỚI bằng `replace()`, và không có side-effect nào — hàm đó vừa
bất biến VỪA thuần. Trông nó như thế nào, viết trọn vẹn?

Bài sau ghép hai nửa track lại thành một.
::::

::::checkpoint{mastery=0.8}
::::
