---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.mot-phan-ba-trong-he-muoi
title: 1/3 viết trong hệ mười cũng không xong
summary: "Trước khi trách máy tính xấp xỉ, tự tay chia 1 cho 3 — hệ mười, cái hệ con người dùng cả đời, cũng không viết hết được phân số này."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [mem.repeating-fraction]
requires: [mem.binary-counting, core.float-precision]
concepts: [mem.phan-so-tuan-hoan, mem.chia-lay-du-lap-lai]
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
Trước khi trách máy tính, mình muốn bạn tự tay chia 1 cho 3. Bằng tay thật.
::::

::::explain{#chia-tay-mot-cho-ba}
R1 đã cho bạn thấy một chuyện khó chịu: `0.1 + 0.2` không ra `0.3`. Lúc đó
lời giải thích dừng ở một câu ngắn — máy ghi số thực bằng hệ hai, và hệ hai
không viết hết được một phần mười, nên phải xấp xỉ.

Câu đó đúng, nhưng nó dễ khiến bạn nghĩ lỗi nằm ở máy, hay nằm riêng ở hệ
hai — như thể hệ mười, cái hệ bạn cầm bút viết ra mỗi ngày, thì không bao
giờ vấp phải chuyện này. Bài này gỡ đúng chỗ hiểu lầm ấy.

Vườn rau sáng nay có 1 lít nước tưới, chia đều cho 3 chậu rau húng. Mỗi
chậu được bao nhiêu lít? Cầm bút chia tay, đúng kiểu chia dài đã học ở
trường:

```text title=readonly
1,000000 ÷ 3
 10 ÷ 3 = 3, dư 1
 10 ÷ 3 = 3, dư 1
 10 ÷ 3 = 3, dư 1
 10 ÷ 3 = 3, dư 1
  ...
```

Mỗi bước, phần dư lại đúng bằng 1 — y hệt bước trước. Không có bước nào dư
về 0 để mà dừng bút lại. Chữ số 3 cứ thế viết mãi: `0,333333…`. Không phải
bạn chia sai. Phép chia này, viết trong hệ mười, thật sự không có điểm
dừng.
::::

::::explain{#luat-chia-roi-lai-chia}
So sánh với một phép chia khác: 1 lít chia đều cho 4 chậu.

```text title=readonly
1,000000 ÷ 4
 10 ÷ 4 = 2, dư 2
 20 ÷ 4 = 5, dư 0   ← dư về 0, DỪNG được
```

Hai lít, hai cách chia rau — mà một cái dừng, một cái không. Khác nhau ở
đúng một chỗ: **dư có bao giờ về 0 hay không.**

Nhìn kỹ động tác bạn vừa làm ở mỗi bước, vì nó chính là một thuật toán, ba
bước lặp đi lặp lại:

1. Lấy phần dư đang có, nhân lên 10.
2. Chia cho mẫu số: **thương** là chữ số tiếp theo sau dấu phẩy, **dư mới**
   là phần còn sót lại.
3. Dư về 0 thì con số đã viết xong — dừng. Dư khác 0 thì lặp lại bước 1,
   với dư mới thay cho dư cũ.

"Thương" và "dư" ở đây chính là `//` và `%` mà R1 đã dạy — phép **chia lấy
phần nguyên** và phép **chia lấy dư**. Bài này chỉ dùng chúng liên tiếp,
nhiều lần, thay vì một lần như mọi khi.

Vấn đề của `1/3` không phải "3 là một số xấu". Vấn đề là: nhân dư lên 10
rồi chia cho 3, phần dư chỉ có thể là 0, 1, hoặc 2 — và với `1/3` thì dư
luôn rơi đúng vào 1, không bao giờ vào 0. Không có cách chia tay nào, dù
kiên nhẫn đến đâu, đổi được kết cục đó.
::::

::::example{#may-lam-dung-viec-ban-vua-lam}
Bây giờ để máy làm đúng ba bước ấy, thay vì làm bằng tay:

```python title=readonly
def chu_so_thap_phan(tu, mau, so_buoc):
    du = tu % mau
    chu_so = []
    for _ in range(so_buoc):
        du = du * 10
        chu_so.append(du // mau)
        du = du % mau
    return chu_so, du

print(chu_so_thap_phan(1, 3, 6))
print(chu_so_thap_phan(1, 4, 6))
```

Máy in ra:

```text title=readonly
([3, 3, 3, 3, 3, 3], 1)
([2, 5, 0, 0, 0, 0], 0)
```

Hàm trả lại hai thứ: danh sách các chữ số sau dấu phẩy, và phần dư còn sót
lại sau bước cuối cùng.

Dòng đầu xác nhận đúng thứ bạn vừa chia bằng tay: sáu chữ số 3 liền nhau,
và dư còn sót lại là 1 — con số ấy sẽ còn sót lại y vậy dù bạn chia thêm
sáu bước nữa, sáu mươi bước nữa. `1/3` không viết hết trong hệ mười.

Dòng hai thì khác hẳn. Sau chữ số thứ hai, dư đã về 0. Ba chữ số 0 phía sau
không sai — chúng đúng là những chữ số tiếp theo của `0,250000…` — chỉ là
dư đã dừng đổi, nên chữ số nào tính từ đó cũng ra 0.
::::

::::predict{#doan-mot-phan-tam commitOnce}
Cũng hàm ấy, gọi với một cặp số khác — `1` chia cho `8`, sáu bước:

```python
def chu_so_thap_phan(tu, mau, so_buoc):
    du = tu % mau
    chu_so = []
    for _ in range(so_buoc):
        du = du * 10
        chu_so.append(du // mau)
        du = du % mau
    return chu_so, du

print(chu_so_thap_phan(1, 8, 6))
```

**Trước khi chạy**, bạn đoán màn hình hiện gì?

:::opt{correct}
`([1, 2, 5, 0, 0, 0], 0)`
:::

:::opt
`([1, 2, 5], 0)`
::why
Gần đúng ở chỗ ba chữ số đầu bạn tính đúng — 1, 2, 5 — và bạn cũng nhận ra
đúng thời điểm phần chia đã xong: dư về 0 ngay sau chữ số thứ ba.

Chỗ lệch nằm ở việc đoạn mã có DỪNG SỚM hay không. Nhìn lại thân hàm: vòng
lặp `for` chạy đủ `so_buoc` lần, không có lệnh nào kiểm "dư đã về 0 chưa"
để thoát ra giữa chừng. Sau khi dư về 0, bước tiếp theo vẫn chạy y hệt:
`0 * 10 = 0`, `0 // 8 = 0`, `0 % 8` vẫn là `0`. Ba chữ số 0 ấy vẫn bị thêm
vào danh sách — chỉ là chúng đều là số 0.
::
:::

:::opt
`([1, 2, 5, 0, 0, 0], 8)`
::why
Gần đúng ở chỗ bạn tính đúng cả sáu chữ số trong danh sách — không sai một
số nào.

Chỗ lệch nằm ở con số thứ hai máy trả về. Nó không phải "8" — cái mẫu số
bạn đưa vào — mà là DƯ CÒN SÓT LẠI sau bước cuối cùng. Một khi dư đã rơi
về 0, dòng `du = du % mau` giữ nó ở 0 mãi, không có lý do gì để nó nhảy
ngược lại thành mẫu số. "Chia hết" nghĩa là dư bằng 0, không phải dư bằng
mẫu.
::
:::

:::opt
`([1, 2, 5, 0, 0, 0], 6)`
::why
Gần đúng ở chỗ danh sách chữ số bạn tính đúng tuyệt đối, không sai một chữ.

Chỗ lệch nằm ở việc hiểu con số thứ hai trong kết quả trả về là gì. Nó
không đếm "đã chạy mấy bước" — con số `6` đó là `so_buoc`, thứ BẠN đưa
vào, không phải thứ hàm tính ra. Giá trị ở vị trí đó luôn là `du`, phần dư
sau bước cuối cùng — mà ở đây phần dư ấy là 0.
::
:::
::::

::::code{#viet-lai-thuat-toan-chia-tay}
Đến lượt bạn viết hai dòng còn thiếu trong chính hàm vừa xem — hai dòng
làm đúng hai bước bạn đã tự tay chia trên giấy: lấy thương của phép chia
`du` cho `mau` làm chữ số tiếp theo, lấy dư của phép chia đó làm dư mới.

Bài chấm trên BA phân số khác nhau: `1/3` (không dừng), `1/4` (dừng ngay),
và `1/6` (không dừng, nhưng dư khác hẳn `1/3`) — ba cảnh khác nhau để một
lời giải chép nhầm bước không có chỗ trốn.

```python title=starter
def chu_so_thap_phan(tu, mau, so_buoc):
    du = tu % mau
    chu_so = []
    for _ in range(so_buoc):
        du = du * 10
        chu_so.append(___)
        du = ___
    return chu_so, du

print(chu_so_thap_phan(1, 3, 6))
print(chu_so_thap_phan(1, 4, 6))
```

```python title=solution
def chu_so_thap_phan(tu, mau, so_buoc):
    du = tu % mau
    chu_so = []
    for _ in range(so_buoc):
        du = du * 10
        chu_so.append(du // mau)
        du = du % mau
    return chu_so, du

print(chu_so_thap_phan(1, 3, 6))
print(chu_so_thap_phan(1, 4, 6))
```

```python title=test
assert chu_so_thap_phan(1, 3, 6) == ([3, 3, 3, 3, 3, 3], 1), "1/3 phải lặp lại chữ số 3 mãi, và dư sau mỗi bước phải luôn là 1"
assert chu_so_thap_phan(1, 4, 6) == ([2, 5, 0, 0, 0, 0], 0), "1/4 phải DỪNG: dư phải về 0 ngay sau chữ số thứ hai"
assert chu_so_thap_phan(1, 6, 6) == ([1, 6, 6, 6, 6, 6], 4), "1/6 lặp lại chữ số 6, và dư còn sót lại phải là 4 — khác hẳn dư của 1/3"
assert len(chu_so_thap_phan(1, 3, 4)[0]) == 4, "so_buoc phải điều khiển được số chữ số tính ra, không được có sáu bước gõ cứng ở đâu đó"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm liền nhau, ngay sau dòng `du = du * 10`. Chỗ đầu đặt chữ số tiếp theo vào danh sách; chỗ sau đặt lại dư cho vòng lặp kế tiếp dùng. Đọc lại ba bước bạn vừa làm bằng tay ở phần giải thích — mỗi chỗ trống đúng là một bước trong đó.
- kind: strategy
  body: "Chữ số tiếp theo là THƯƠNG của phép chia `du` cho `mau` — dùng `//`, phép chia lấy phần nguyên mà R1 đã dạy. Dư mới là PHẦN CÒN LẠI của đúng phép chia đó — dùng `%`. Cả hai đều chia `du` cho `mau`, không phải `tu` cho `mau`: `tu` không đổi trong lúc `du` mới là thứ mang thông tin của bước hiện tại."
- kind: one-line
  body: "Chỗ trống thứ nhất là `du // mau`, chỗ trống thứ hai là `du % mau`."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^\(\[3, 3, 3, 3, 3, 3\], 1\)\n\(\[2, 5, 0, 0, 0, 0\], 0\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`1/3` không dừng trong hệ mười. Vấn đề chưa từng nằm ở máy — nó nằm ở phân
số.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`1/3` không viết hết được trong hệ mười, vì ở mỗi bước dư chỉ chạy quanh
những con số không bao giờ chạm 0. Đó không phải lỗi của con số 3 — đó là
tính chất của phép chia đó, đặt trong hệ MƯỜI đó.

Máy tính không dùng hệ mười. Realm 0 đã nói ngay từ đầu: máy chỉ có hai
trạng thái, hệ HAI. Ở hệ hai, mỗi bước không còn nhân dư lên 10 nữa — mà
nhân lên 2. Chỉ có mỗi số 2 để dùng, không có số 5 đi kèm như hệ mười.

Con số `0,1` — một phần mười, con số bạn gõ mỗi ngày không nghĩ ngợi gì —
liệu nó có viết hết được trong cái hệ chỉ-có-mỗi-số-2 đó không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
