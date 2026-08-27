---
id: nen-tang.list-dict-set-tuple.dem-nguoc-tu-cuoi
title: Khoản vừa ghi nằm ở đâu
summary: Cuốn sổ dài thêm mỗi ngày, nên chỗ đứng của khoản cuối đổi mỗi ngày — `so[-1]` nói đúng "khoản cuối" mà không phải hỏi sổ dài bao nhiêu.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.list-negative-index]
requires: [core.negative-index, core.list, core.list-index, core.list-append, core.len, core.function-def, core.function-parameter, core.function-return, core.argument-not-copy, core.fstring]
concepts: [core.danh-sach, core.chi-so]
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
Sổ dài thêm mỗi ngày. Chỗ đứng của khoản cuối vì thế cũng đổi mỗi ngày.
::::

::::explain{#cho-dung-cua-khoan-cuoi}
Cái máy tính hoá đơn đã chạy. Byte đem nó về nhà và làm thêm một việc nhỏ mỗi
tối: ghi lại khoản mình vừa tiêu trong ngày vào một cuốn sổ.

Cuốn sổ ấy là một danh sách các con số, và việc ghi thêm thì gói trong một hàm
ba dòng — đúng hình dạng bạn đã dựng ở mạch hàm:

```python
def ghi_chi(so, tien):
    """Ghi thêm một khoản vào cuối cuốn sổ được đưa vào. Không trả về gì."""
    so.append(tien)
```

Hàm này nhận **chính** cuốn sổ ngoài kia chứ không nhận một bản photo — bài
*Đối số không phải bản sao* đã cho bạn thấy tận mắt. Nên gọi `ghi_chi` một lần
là cuốn sổ thật dài thêm một dòng.

Và đây là chỗ sinh chuyện. Ghi xong, Byte muốn liếc lại **khoản vừa ghi** để
chắc mình không gõ thừa một số 0. Xin một chỗ trong danh sách thì bạn biết
rồi: mở ngoặc vuông, đặt số chỗ vào trong, chỗ đầu tiên mang số `0`.

Nhưng số chỗ của khoản cuối là bao nhiêu?

Tối nay sổ có 6 khoản, chỗ cuối mang số 5, nên viết `so[5]`. Tối mai Byte ghi
thêm một khoản nữa: sổ thành 7 khoản, `so[5]` lập tức trỏ vào khoản của **hôm
nay** chứ không phải khoản vừa ghi. Con số ấy không đứng yên được một ngày nào.

Cách viết đúng cho mọi tối thì có:

```python
so[len(so) - 1]
```

Chạy đúng thật. Nhưng nhìn xem nó bắt bạn làm gì: gõ tên cuốn sổ hai lần trên
một dòng, hỏi sổ dài bao nhiêu, rồi nhớ trừ đi một — ba việc, để nói một câu
rất ngắn: *khoản cuối*.

Bạn đã gặp lối tắt cho đúng câu ngắn ấy rồi, chỉ là gặp trên một thứ khác. Ở
mạch chuỗi, bài *Tra từng ký tự một* nói: bên trong ngoặc vuông, con số luôn là
một **chỗ đứng**, và dấu âm không phải phép trừ — nó đổi **chiều đếm**. Số
dương đếm xuôi từ đầu dãy, số âm đếm ngược từ cuối dãy, và chiều ngược bắt đầu
ngay ở `-1`.

Chuỗi là một dãy có thứ tự. Danh sách cũng là một dãy có thứ tự — cùng cặp
ngoặc vuông, cùng cách đếm từ 0. Nên chiều đếm ngược dùng được nguyên vẹn trên
cuốn sổ:

```python
so[-1]   # khoản cuối sổ
so[-2]   # khoản kế cuối
```

`so[-1]` là khoản cuối của **mọi** cuốn sổ — sổ ba khoản hay sổ ba trăm khoản
cũng vậy, hôm nay hay tối mai cũng vậy. Bạn không cần biết sổ dài bao nhiêu mới
sờ được vào cái đuôi của nó.
::::

::::example{#hai-chieu-tren-cuon-so}
Sổ tháng Ba của Byte, năm khoản đầu tháng:

```python title=readonly
so_thang_ba = [25000, 40000, 15000, 60000, 30000]

print(so_thang_ba[0])
print(so_thang_ba[-1])
print(so_thang_ba[-2])
```

Màn hình hiện ra:

```text title=readonly
25000
30000
60000
```

Cùng một cuốn sổ, đọc theo hai chiều. Hàng trên là số chỗ đếm xuôi, hàng dưới
là số chỗ đếm ngược:

```text title=readonly
  25000  40000  15000  60000  30000
      0      1      2      3      4
     -5     -4     -3     -2     -1
```

Mỗi khoản có hai cách gọi, và cả hai đều chỉ đúng nó. `so_thang_ba[3]` với
`so_thang_ba[-2]` là cùng một khoản 60 nghìn.

Bờ của cái thang này thì y như luật bạn đã biết với danh sách: xin một chỗ mà
sổ không có thì máy dừng lại và nói ra.

```python title=readonly
so_thang_ba = [25000, 40000, 15000, 60000, 30000]
print(so_thang_ba[-6])
```

```text title=readonly
IndexError: list index out of range
```

Sổ năm khoản thì chỗ xa nhất về phía sau là `-5`. Đếm chiều nào cũng không đẻ
thêm ra khoản thứ sáu.

Còn đây là chỗ chiều ngược ăn đứt một con số cố định. Byte ghi thêm khoản của
tối nay vào đúng cuốn sổ ấy:

```python title=readonly
so_thang_ba = [25000, 40000, 15000, 60000, 30000]


def ghi_chi(so, tien):
    """Ghi thêm một khoản vào cuối cuốn sổ được đưa vào. Không trả về gì."""
    so.append(tien)


ghi_chi(so_thang_ba, 12000)
print(so_thang_ba)
print(so_thang_ba[-1])
```

```text title=readonly
[25000, 40000, 15000, 60000, 30000, 12000]
12000
```

Sổ dài thêm một khoản, và `[-1]` đi theo cái đuôi ấy mà không ai phải sửa gì.
Trong khi đó `[4]` vẫn đứng nguyên chỗ cũ, giờ trỏ vào khoản 30 nghìn của hôm
qua.
::::

::::predict{#doan-hai-cach-goi commitOnce}
Byte ghi khoản 12 nghìn của tối nay vào sổ tháng Ba, rồi in ra hai thứ: một
dòng dùng số chỗ đếm xuôi, một dòng dùng chiều đếm ngược.

Trước lúc ghi thêm, sổ có năm khoản.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra hai dòng nào?

```python title=readonly
so_thang_ba = [25000, 40000, 15000, 60000, 30000]


def ghi_chi(so, tien):
    """Ghi thêm một khoản vào cuối cuốn sổ được đưa vào. Không trả về gì."""
    so.append(tien)


ghi_chi(so_thang_ba, 12000)

print(so_thang_ba[4])
print(so_thang_ba[-1])
```

:::opt{correct}
30000 rồi 12000
:::

:::opt
12000 rồi 12000 — hai dòng in ra y hệt nhau
::why
Gần đúng ở chỗ bạn nhớ một chuyện có thật: lúc sổ mới có năm khoản, chỗ số 4
**đúng là** chỗ cuối, nên `[4]` và `[-1]` khi ấy trỏ vào cùng một khoản. Bạn
đọc bảng hai chiều không sai.

Chỗ lệch là dòng `ghi_chi` nằm giữa. Nó viết thêm khoản thứ sáu vào cuối sổ,
nên chỗ cuối chuyển sang số 5. Con số `4` thì không tự đi theo — nó vẫn chỉ
đúng chỗ nó luôn chỉ, và chỗ ấy giờ là khoản áp chót, 30 nghìn.

Đó chính là lý do bài này tồn tại: một con số chỗ đứng viết cứng chỉ đúng cho
một độ dài sổ, còn `[-1]` đúng cho mọi độ dài.
::
:::

:::opt
30000 rồi 30000 — hai dòng in ra y hệt nhau
::why
Gần đúng ở chỗ bạn đọc dòng đầu chính xác: sau khi ghi thêm, `[4]` trỏ vào
khoản 30 nghìn, không sai một đồng.

Chỗ lệch nằm ở cách đọc dấu trừ. Bạn đang hiểu `[-1]` là "bớt một khoản ở
cuối", tức khoản áp chót — cách hiểu rất tự nhiên, vì ngoài ngoặc vuông thì dấu
trừ đúng là phép trừ. Bên trong ngoặc vuông thì không: con số ở đó là một **chỗ
đứng**, và dấu âm chỉ đổi chiều đếm. Chiều ngược khởi hành ngay tại ô cuối
cùng, và ô ấy mang số `-1`.
::
:::

:::opt
Máy báo `IndexError` ở dòng `print(so_thang_ba[4])`
::why
Gần đúng ở chỗ bạn nhớ có một cái bờ, và xin quá bờ thì máy dừng lại kêu lên —
luật ấy có thật với danh sách.

Chỗ lệch là vị trí của cái bờ. Sau khi `ghi_chi` chạy, sổ có sáu khoản, nên
những chỗ hợp lệ chạy từ `0` tới `5` theo chiều xuôi, và từ `-6` tới `-1` theo
chiều ngược. Số `4` nằm gọn bên trong. Ghi thêm vào sổ chỉ **nới rộng** khoảng
hợp lệ ra, không bao giờ làm hẹp lại.
::
:::
::::

::::explain{#viet-ra-dieu-minh-muon}
Hai cách viết cùng chỉ vào một khoản, vậy chọn cách nào cũng như nhau?

Như nhau, cho tới lúc cuốn sổ đổi độ dài — mà một cuốn sổ chi tiêu thì tối nào
cũng đổi. Với `[-1]` bạn viết **một** câu và nó đúng cho sổ hôm nay, sổ tối
mai, và cả cuốn sổ tháng sau chưa ai mở ra. Với `len(so) - 1` thì bạn phải kéo
đúng cái tên ấy vào từng dòng, và chép nhầm tên thì câu lệnh đọc sai cuốn sổ mà
không báo lỗi nào cả.

Cái được ở đây không phải gõ ít phím hơn. Nó là chuyện câu lệnh nói thẳng ra
điều bạn muốn: *khoản cuối* — chứ không phải *khoản ở chỗ bằng độ dài trừ một*.

> Chỗ dễ vấp: `[-1]` là chỗ cuối, `[0]` là chỗ đầu. Không có `[-0]`, vì số `0`
> đã có chủ rồi — nó là chỗ đầu tiên. Viết `so[-0]` thì máy đọc ra `so[0]` và
> đưa bạn khoản **đầu** sổ, im lặng, không một lời cảnh báo.
::::

::::code{#doc-lai-khoan-vua-ghi}
Byte muốn một hàm tên `khoan_vua_ghi`: đưa vào cuốn sổ, nhận về khoản nằm cuối
sổ. Hàm ấy phải đúng cho **mọi** cuốn sổ, dài ngắn thế nào cũng vậy — nên nó
không được phép hỏi sổ dài bao nhiêu, và cũng không được lấy đi khoản nào của
sổ.

Bài chấm trên hai cuốn sổ có độ dài khác nhau. Một con số chỗ đứng viết cứng
không thể vừa trúng cuối cuốn này vừa trúng cuối cuốn kia.

Điền vào chỗ trống câu ngắn nhất nói ra đúng ý *khoản cuối*.

```python title=starter
def ghi_chi(so, tien):
    """Ghi thêm một khoản vào cuối cuốn sổ được đưa vào. Không trả về gì."""
    so.append(tien)


def khoan_vua_ghi(so):
    """Nhận cuốn sổ, trả về khoản nằm CUỐI sổ. Sổ không bị đụng tới."""
    return ___


so_hom_qua = [25000, 40000, 15000]
so_hom_nay = [25000, 40000, 15000, 60000, 30000]

ghi_chi(so_hom_nay, 12000)

print(f"Sổ hôm qua — khoản cuối: {khoan_vua_ghi(so_hom_qua)} đồng")
print(f"Sổ hôm nay — khoản cuối: {khoan_vua_ghi(so_hom_nay)} đồng")
```

```python title=solution
def ghi_chi(so, tien):
    """Ghi thêm một khoản vào cuối cuốn sổ được đưa vào. Không trả về gì."""
    so.append(tien)


def khoan_vua_ghi(so):
    """Nhận cuốn sổ, trả về khoản nằm CUỐI sổ. Sổ không bị đụng tới."""
    return so[-1]


so_hom_qua = [25000, 40000, 15000]
so_hom_nay = [25000, 40000, 15000, 60000, 30000]

ghi_chi(so_hom_nay, 12000)

print(f"Sổ hôm qua — khoản cuối: {khoan_vua_ghi(so_hom_qua)} đồng")
print(f"Sổ hôm nay — khoản cuối: {khoan_vua_ghi(so_hom_nay)} đồng")
```

```python title=test
# Bốn phép kiểm, mỗi phép chặn một kiểu điền sai khác nhau:
#   một hằng số bất kỳ  → trượt ngay phép kiểm đầu
#   `so[2]`             → đúng cho sổ ba khoản, trượt ở sổ sáu khoản
#   `so[-2]`            → trượt cả hai phép kiểm đầu
#   `so.pop()`          → trả đúng khoản cuối, nhưng LẤY nó ra khỏi sổ,
#                         và phép kiểm cuối bắt được chỗ đó
assert khoan_vua_ghi([25000, 40000, 15000]) == 15000, "cuốn sổ ba khoản này kết ở khoản 15 nghìn, nên khoản cuối của nó là 15000"
assert khoan_vua_ghi([25000, 40000, 15000, 60000, 30000, 12000]) == 12000, "cuốn sổ sáu khoản này kết ở khoản 12 nghìn — một con số chỗ đứng cố định không thể vừa trúng cuối sổ ba khoản vừa trúng cuối sổ sáu khoản"
assert khoan_vua_ghi([70000]) == 70000, "cuốn sổ mới có đúng một khoản thì khoản ấy vừa là khoản đầu vừa là khoản cuối"
assert so_hom_nay == [25000, 40000, 15000, 60000, 30000, 12000], "sổ hôm nay có sáu khoản sau khi `ghi_chi` viết thêm 12 nghìn vào cuối — ĐỌC khoản cuối không được lấy nó ra khỏi sổ"
```

:::hints
- kind: attention
  body: Chỗ trống nằm sau `return`, nên thứ bạn điền phải là MỘT khoản tiền, không phải cả cuốn sổ. Cái tên đang giữ cuốn sổ ở đây là tham số `so`. Hai cuốn sổ đem chấm dài ba khoản và sáu khoản, nên hãy hỏi mình câu này trước khi gõ — chỗ đứng của khoản cuối có giống nhau ở hai cuốn không.
- kind: strategy
  body: Xin một chỗ trong dãy thì viết tên rồi mở ngoặc vuông. Ở đây bạn cần chỗ cuối, mà không có con số dương nào trúng chỗ cuối của cả hai cuốn sổ. Chiều đếm ngược thì có, và nó khởi hành ngay tại ô cuối cùng chứ không phải ở số 0.
- kind: one-line
  body: Viết `so[-1]` vào chỗ trống.
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  expect: Sổ hôm nay — khoản cuối: 12000 đồng
- tier: static
  onFail: câu này phải nói ra "khoản cuối" mà không phải hỏi sổ dài bao nhiêu — bỏ `len` đi và dùng chiều đếm ngược
  requireAst:
  # `min: 2` vì thân `ghi_chi` đã đọc tên `so` một lần rồi. Dòng bạn điền phải
  # là lần đọc thứ hai — hỏi `min: 1` thì điền một hằng số cũng thoả.
  - kind: uses-name, target: so, min: 2
  forbidAst:
  - kind: uses-call, target: len
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sổ ba khoản hay sổ sáu khoản, mình vẫn sờ đúng cái đuôi bằng một câu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ bạn có hai đầu cuốn sổ trong tay: `[-1]` cho khoản cuối, `[0]` cho khoản
đầu.

Byte làm báo cáo và cần **ba khoản đầu tháng** — không phải một khoản, mà một
nhóm ba khoản liền nhau, để đem cộng lại và đưa vào một dòng báo cáo.

Cách duy nhất bạn đang có là gõ từng chỗ một rồi tự ghép tay lại thành một danh
sách mới:

```python
ba_khoan = [so[0], so[1], so[2]]
```

Chạy được. Nhưng tháng sau sếp của Byte đổi ý, muốn **mười** khoản đầu tháng —
và dòng trên phải viết lại từ đầu, mười chỗ, không sót một dấu phẩy.

Gõ `so[0]`, `so[1]`, `so[2]` rồi tự ghép tay lại thành một danh sách mới sao?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
