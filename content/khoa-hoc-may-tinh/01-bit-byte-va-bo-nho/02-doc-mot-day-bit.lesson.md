---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.doc-mot-day-bit
title: Đọc một dãy bit ra số
summary: "Mỗi cột trong dãy bit đáng giá gấp đôi cột bên phải nó — đúng bảng vị trí toán học đã dựng, chỉ đổi cơ số từ mười xuống hai. Cộng các cột đang bật lại là đọc ra được con số."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [mem.binary-to-int]
requires: [mem.binary-counting]
concepts: [mem.binary-to-int]
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
Cô Bảy vừa lắp một máy đếm khách trước cửa. Đèn của nó đang hiện `1011`.
::::

::::explain{#dong-bo-danh-gia-bao-nhieu}
Bài trước để lại một câu hỏi thẳng: dòng `101`, đứng ở lần đếm thứ sáu trong
bảng, là số mấy?

Đếm bằng bảng vị trí. Bạn đã quen bảng ấy từ hệ mười — cột hàng chục đáng giá
gấp **mười** cột hàng đơn vị, cột hàng trăm gấp mười cột hàng chục, cứ thế.
Chuyện đó không phải luật riêng của số mười; nó là luật của **cơ số**: mỗi
cột đáng giá gấp cơ số so với cột ngay bên phải nó.

Ở đây cơ số là hai — đúng cái ngưỡng lên bó bài trước vừa dùng. Nên mỗi cột
chỉ đáng giá gấp **hai** cột bên phải nó, không phải gấp mười:

```text title=readonly
cột phải cùng đáng giá 1
cột kế tiếp     đáng giá 2   (gấp đôi 1)
cột kế nữa      đáng giá 4   (gấp đôi 2)
cột kế nữa      đáng giá 8   (gấp đôi 4)
```

Đọc một dãy bit ra số: nhìn từng cột đang **bật**, cộng đáng giá của những
cột đó lại. Cột tắt thì góp đúng 0, khỏi cộng.

Đọc thử `101`: cột phải cùng là `1` (đáng giá 1), cột giữa là `0` (đáng giá 2,
nhưng tắt nên góp 0), cột trái cùng là `1` (đáng giá 4). Cộng lại: 4 + 0 + 1 =
**5**.

Vậy dòng `101` ở lần đếm thứ sáu trong bảng trước là số **5** — đúng bằng lần
đếm trừ đi một, vì bảng bắt đầu đếm từ `000` là số 0, không phải từ 1. Không
phải tình cờ: bạn đã đếm đúng thứ tự các số nguyên từ 0 trở lên, chỉ chưa biết
gọi tên chúng mà thôi.
::::

::::example{#doc-den-may-dem-khach}
Đèn máy đếm khách của cô Bảy đang hiện bốn đèn: `1011`. Đọc ra bằng vòng lặp,
đi từ cột phải cùng sang trái, mỗi bước cột đáng giá gấp đôi:

```python title=readonly
day = "1011"

gia_tri = 0
cot = 1                       # cột phải cùng đáng giá 1
for ky_tu in reversed(day):   # đi từ phải sang trái
    if ky_tu == "1":
        gia_tri += cot
    cot = cot * 2              # mỗi bước sang trái, cột nặng gấp đôi

print(gia_tri)
```

Máy in ra:

```text title=readonly
11
```

Đi qua từng bước: `reversed(day)` cho ký tự cuối cùng ra trước — đúng cột
phải cùng. Cột đó là `1`, cộng vào 1. Cột kế là `1` (đáng giá đã gấp đôi
thành 2), cộng vào — được 3. Cột kế nữa là `0` (đáng giá 4), tắt nên bỏ qua.
Cột trái cùng là `1` (đáng giá 8), cộng vào — được 11.

Máy đếm khách của cô Bảy đang báo **11 khách**.
::::

::::predict{#doan-day-den-1010 commitOnce}
Đèn máy đếm khách đổi sang `1010`.

**Trước khi tính tay**, bạn đoán nó đang báo bao nhiêu khách?

:::opt{correct}
10 khách.
:::

:::opt
5 khách — đọc từ trái qua phải, cột đầu tiên đáng giá 1.
::why
Gần đúng ở chỗ bạn cộng đúng những cột đang bật — không bỏ sót cột nào, không
cộng thừa cột nào tắt. Việc chọn CỘT NÀO để cộng, bạn làm đúng.

Chỗ lệch là hướng gán đáng giá. Cột đáng giá **1** luôn là cột phải cùng,
đúng như hàng đơn vị luôn đứng bên phải hàng chục — không phải cột đầu tiên
mắt bạn đọc tới. Gán ngược hướng thì `1010` bị đọc thành `4 + 0 + 1 + 0 = 5`,
trong khi đọc đúng hướng nó là `8 + 0 + 2 + 0 = 10`.
::
:::

:::opt
1010 khách — dãy bit hiện sao thì đọc vậy.
::why
Gần đúng ở chỗ `1010` đúng là thứ đang thật sự hiện trên đèn — bạn không đọc
sai một ký tự nào của dãy bit.

Chỗ lệch là dãy bit **không phải** chính con số nó đại diện. `1010` là bốn
ký tự `1`, `0`, `1`, `0` xếp cạnh nhau; con số nó đại diện phải đi qua bước
cộng đáng giá từng cột mới ra được. Y hệt cách `101` ở bài trước không phải
số một trăm lẻ một — nó là số 5, sau khi đọc qua bảng vị trí.
::
:::

:::opt
20 khách — cột phải cùng đáng giá 2, cột kế đáng giá 4, cứ thế.
::why
Gần đúng ở chỗ bạn dùng đúng luật "mỗi cột gấp đôi cột bên phải" — không sai
quy tắc nhân đôi chút nào.

Chỗ lệch là điểm xuất phát. Cột phải cùng luôn đáng giá **1**, không phải 2 —
đó là cột nhỏ nhất có thể có, và mọi cột khác gấp đôi dần lên TỪ đó. Đẩy điểm
xuất phát lên 2 thì cả dãy đáng giá bị lệch hẳn một cột, và `1010` bị đọc
thừa ra gấp đôi giá trị thật.
::
:::
::::

::::code{#doc-day-den-chieu}
Chiều nay đèn máy đếm khách đổi tiếp sang `0110`. Byte đã gõ sẵn khung vòng
lặp giống hệt ví dụ trên, chỉ bỏ trống hai chỗ: chỗ CỘNG vào tổng, và chỗ
GẤP ĐÔI đáng giá cột cho lượt kế tiếp.

```python title=starter
day = "0110"

gia_tri = 0
cot = 1
for ky_tu in reversed(day):
    if ky_tu == "1":
        gia_tri += ___
    cot = ___

print(gia_tri)
```

```python title=solution
day = "0110"

gia_tri = 0
cot = 1
for ky_tu in reversed(day):
    if ky_tu == "1":
        gia_tri += cot
    cot = cot * 2

print(gia_tri)
```

```python title=test
# 0110: cột phải cùng tắt, hai cột giữa bật (đáng giá 2 và 4), cột trái cùng
# tắt. 2 + 4 = 6.
assert day == "0110", "đèn chiều nay là 0110 — chỗ trống không được sửa dãy bit này"
assert gia_tri == 6, "hai cột giữa của 0110 đang bật, đáng giá 2 và 4 — cộng lại phải ra 6"
assert cot == 16, "sau đúng bốn lượt gấp đôi kể từ 1, cot phải đứng ở 16 — nếu không, chỗ trống thứ hai chưa thật sự gấp đôi mỗi lượt"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm ngay dưới dòng kiểm tra "ký tự đang bật" — nó phải cộng đúng đáng giá của cột hiện tại vào tổng. Chỗ trống thứ hai nằm ngoài khối `if`, nên nó chạy ở MỌI lượt, kể cả lượt cột đang tắt.
- kind: strategy
  body: Đáng giá của cột hiện tại đã có sẵn một cái tên — `cot` — đừng gõ lại con số bằng tay. Và đáng giá của lượt kế tiếp luôn gấp đôi đáng giá lượt này, nên chỗ trống thứ hai là `cot` nhân với 2.
- kind: one-line
  body: "Chỗ trống thứ nhất là `cot`, chỗ trống thứ hai là `cot * 2`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^6\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
6 khách vào chiều nay. Đọc một dãy bit giờ chỉ còn là một phép cộng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa học đọc một dãy bit **có sẵn** ra thành số. Nhưng máy đếm khách đâu tự
nó biết đèn nào bật đèn nào tắt — có ai đó, hay một đoạn mã nào đó, phải bật
tắt đúng đèn để con số hiện lên.

Tối nay quán đóng cửa với đúng 22 khách đã ghé. Muốn đèn hiện đúng con số ấy,
thì phải bật tắt đèn nào? Đây là chiều ngược lại của việc bạn vừa làm — có
con số sẵn, cần dãy bit.

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
