---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.dau-cham-troi
title: Dấu chấm biết trôi
summary: "Mổ xẻ `0x1.999999999999ap-4` ra ba phần — dấu, mũ, phần định trị — và thấy vì sao số thực trong máy gọi là 'chấm động', không phải 'chấm cố định'."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [mem.float-layout]
requires: [mem.binary-fraction]
concepts: [mem.dau-mu-dinh-tri, mem.chuoi-hex-float]
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
Ba phần, ngăn bởi `x`, dấu chấm, và chữ `p`. Mổ từng phần một nhé.
::::

::::explain{#cach-viet-quen-thuoc}
Trước khi mổ chuỗi của máy, nhớ lại một cách viết bạn đã học ở trường:
viết một số rất lớn hay rất nhỏ bằng luỹ thừa của 10. Một triệu ba trăm
nghìn viết gọn thành `1,3 × 10^6`. Cách viết ấy có ba mảnh: **dấu** (ở đây
là dương), một con số gọn gọi là **phần định trị**, và một **số mũ** nói
con số gọn đó phải "phóng to" bao nhiêu lần.

Điều hay của cách viết này: dấu chấm thập phân trong phần định trị KHÔNG
đứng yên một chỗ. Số mũ càng lớn, dấu chấm "trôi" càng xa về bên phải nếu
viết ra đầy đủ. Viết `1,3 × 10^6` hay viết đầy đủ `1300000` là cùng một
con số — chỉ khác chỗ dấu chấm đứng.

Máy tính ghi số thực đúng theo tinh thần đó, chỉ đổi cơ số 10 thành 2:

> **giá trị = dấu × phần định trị × 2^(số mũ)**

Cách ghi này gọi là **dấu chấm động** (tiếng Anh: floating point — chữ
`float` bạn gặp từ những bài đầu tiên chính là viết tắt của cụm này). Gọi
là "động" vì dấu chấm không neo cứng ở một vị trí: số mũ quyết định nó
trôi tới đâu. Nhờ vậy, cùng một số bit ghi được cả số cực lớn lẫn số cực
nhỏ — đổi số mũ là đủ, không cần thêm chỗ chứa.

(Trái ngược là **chấm cố định**: nếu bạn LUÔN dành đúng ba chữ số cuối làm
phần lẻ — như một cách đọc khác của 15000 là 15,000 — thì dấu chấm đứng
yên ở đúng một chỗ, không bao giờ trôi. Bài 17 quay lại đúng ý này.)
::::

::::example{#doc-chuoi-hex-cua-python}
`.hex()` của Python in nguyên văn ba phần ấy ra, viết theo hệ mười-sáu:

```python title=readonly
print((1.0).hex())
print((2.0).hex())
print((4.0).hex())
print((0.5).hex())
print((-2.5).hex())
```

```text title=readonly
0x1.0000000000000p+0
0x1.0000000000000p+1
0x1.0000000000000p+2
0x1.0000000000000p-1
-0x1.4000000000000p+1
```

Đọc từng dòng theo khuôn `[dấu]0x[phần định trị]p[số mũ]`:

- **Dấu**: dấu `-` đứng trước cùng nếu số âm; không có gì thì là số dương.
  Chỉ dòng cuối có dấu `-`, vì `-2,5` là số âm duy nhất trong năm dòng.
- **Phần định trị**: luôn bắt đầu bằng `1.` — trong hệ hai, chữ số khác
  không duy nhất là 1, nên sau khi "trôi" dấu chấm về ngay sau bit khác
  không đầu tiên, chữ số ấy CHẮC CHẮN là 1. Phần sau dấu chấm là mười ba
  chữ số hex, ghi năm mươi hai bit theo sau cái bit 1 ấy.
- **Số mũ**: theo sau chữ `p` (viết tắt của *power*, luỹ thừa), LUÔN là số
  mũ của 2, dù phần định trị viết bằng hệ mười-sáu. `4,0 = 1 × 2^2`, nên
  số mũ là `+2`. `0,5 = 1 × 2^(-1)`, nên số mũ là `-1`.

Dòng `-2,5` gói cả ba ý lại: `2,5 = 1,25 × 2`, viết `1,25` trong hệ hai là
`1,01`, gộp theo hệ mười-sáu thành `1.4` — khớp đúng
`-0x1.4000000000000p+1`.
::::

::::predict{#doan-bon-phay-khong commitOnce}
**Trước khi chạy**, bạn đoán `(4.0).hex()` in ra chuỗi nào?

```python
print((4.0).hex())
```

:::opt{correct}
`'0x1.0000000000000p+2'`
:::

:::opt
`'0x1.0000000000000p+4'`
::why
Gần đúng ở chỗ con số `4` đúng là có mặt trong câu trả lời của bạn — bạn
không bịa ra một số mũ ngẫu nhiên.

Chỗ lệch là con số `4` đó đứng NHẦM VAI TRÒ. Số mũ không phải là chính giá
trị đang xét — nó là LUỸ THỪA của 2 cần dùng để dựng lại giá trị đó từ
phần định trị `1.0`. `4 = 1 × 2^2`, nên số mũ đúng phải là `2`, không phải
`4`.
::
:::

:::opt
`'0x4.0000000000000p+0'`
::why
Gần đúng ở chỗ bạn giữ nguyên con số `4` — trực giác "cứ giữ số gốc" là
phản xạ tự nhiên khi chưa quen cách ghi này.

Chỗ lệch: phần định trị trong hệ hai LUÔN chuẩn hoá về dạng bắt đầu bằng
`1.`, không bao giờ là `4.`. Máy luôn "trôi" dấu chấm tới ngay sau bit 1
đầu tiên rồi mới ghi số mũ — đó chính là lý do gọi nó là "trôi".
::
:::

:::opt
`'0x1.0000000000000p-2'`
::why
Gần đúng ở chỗ phần định trị `1.0` bạn viết đúng, không sai một chữ số.

Chỗ lệch là dấu của số mũ. Số mũ ÂM dành cho những số NHỎ HƠN 1 — như
`0,5 = 1 × 2^(-1)`, số mũ `-1`. `4,0` lớn hơn 1 nhiều lần, nên số mũ phải
DƯƠNG: `4 = 1 × 2^(+2)`.
::
:::
::::

::::code{#tach-ba-phan}
Viết một hàm tách chuỗi `.hex()` ra đúng ba phần: dấu, phần định trị, số
mũ — bằng chính những phương thức chuỗi R1 đã dạy.

```python title=starter
def tach_ba_phan(x):
    chuoi = x.hex()
    dau = "-" if chuoi.startswith("-") else "+"
    chuoi = chuoi.lstrip("-")[2:]
    than_dinh_tri, mu = ___
    return dau, than_dinh_tri, ___

print(tach_ba_phan(0.1))
print(tach_ba_phan(4.0))
print(tach_ba_phan(-2.5))
```

```python title=solution
def tach_ba_phan(x):
    chuoi = x.hex()
    dau = "-" if chuoi.startswith("-") else "+"
    chuoi = chuoi.lstrip("-")[2:]
    than_dinh_tri, mu = chuoi.split("p")
    return dau, than_dinh_tri, int(mu)

print(tach_ba_phan(0.1))
print(tach_ba_phan(4.0))
print(tach_ba_phan(-2.5))
```

```python title=test
assert tach_ba_phan(0.1) == ('+', '1.999999999999a', -4), "0.1: dấu phải là '+', phần định trị '1.999999999999a', số mũ -4 (số nguyên, không phải chuỗi '-4')"
assert tach_ba_phan(4.0) == ('+', '1.0000000000000', 2), "4.0 = 1 × 2^2, số mũ phải là số nguyên 2"
assert tach_ba_phan(-2.5) == ('-', '1.4000000000000', 1), "-2.5: dấu phải là '-', số mũ là 1"
assert tach_ba_phan(1024.0) == ('+', '1.0000000000000', 10), "1024 = 2^10: số mũ phải đọc theo hệ MƯỜI là 10 — nếu bạn chuyển mu bằng int(mu, 16), phép thử này lộ ra vì 16 khác 10"
```

:::hints
- kind: attention
  body: Sau khi hai dòng cho sẵn đã lột dấu `-` và tiền tố `0x`, chuỗi còn lại có hình dạng `1.999999999999ap-4` — một phần định trị, chữ `p`, rồi số mũ. Chỗ trống thứ nhất phải tách chuỗi ấy làm đôi tại đúng chữ `p`.
- kind: strategy
  body: "Chuỗi có một phương thức tách theo một ký tự cho trước, trả về đúng hai mảnh nếu ký tự đó chỉ xuất hiện một lần — R1 đã dùng nó để tách một dòng thành từng cột theo dấu phẩy. Mảnh sau khi tách vẫn là CHUỖI; số mũ cần đưa qua một hàm chuyển chuỗi số thành số nguyên trước khi trả về, nếu không nó vẫn mang theo dấu `+` hay `-` dư thừa và không so sánh được với một số nguyên thật."
- kind: one-line
  body: "Chỗ trống thứ nhất là `chuoi.split(\"p\")`, chỗ trống thứ hai là `int(mu)`."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^\('\+', '1\.999999999999a', -4\)\n\('\+', '1\.0000000000000', 2\)\n\('-', '1\.4000000000000', 1\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Dấu, mũ, phần định trị — ba mảnh, và bạn vừa tự tay tách được cả ba.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy VÌ SAO `0,1` không được cất đúng: phần định trị của nó phải
bị cắt ở đâu đó, giữa một dãy đáng lẽ vô hạn. Biết lý do là một chuyện;
SỐNG CHUNG với nó lại là chuyện khác.

`0,1 + 0,1 + 0,1` có bằng `0,3` không? Dấu `==` sẽ trả lời — nhưng câu trả
lời của nó có đáng tin không, sau tất cả những gì bạn vừa thấy về việc
dãy số bị cắt ngang?

Bài sau trả lời, bằng đúng một câu hỏi khác để thay cho `==`.
::::

::::checkpoint{mastery=0.8}
::::
