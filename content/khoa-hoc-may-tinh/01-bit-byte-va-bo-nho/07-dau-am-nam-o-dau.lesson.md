---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.dau-am-nam-o-dau
title: Dấu âm cất ở đâu trong tám cái ô
summary: Cách nghĩ đầu tiên ai cũng thử — dành riêng một ô làm cờ dấu — có vẻ hợp lý, cho tới khi bạn tự tay tính ra HAI mẫu bit cùng mang nghĩa "số không".
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [mem.sign-bit]
requires: [mem.byte-range, mem.binary-to-int, core.floor-division, core.modulo, core.arithmetic, core.function-def, core.function-parameter, core.function-return, ctrl.if, ctrl.else, core.variable, core.print-variable]
concepts: [mem.sign-bit, mem.hai-so-khong]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Ba cách viết cùng một số dương — bài trước xong rồi. Số âm thì mình
chưa biết cất ở đâu trong tám cái ô cả.
::::

::::explain{#danh-rieng-mot-o-lam-co}
Bài 4 chốt rằng một byte, đọc thẳng như mọi bit đều là số, chứa được
đúng 0 tới 255 — toàn số không âm. Không có ô nào để dành riêng cho dấu
trừ, vì mỗi ô chỉ biết `0` hoặc `1`.

Cách nghĩ đầu tiên gần như ai cũng thử: mượn Ý TƯỞNG của dấu `+`/`−`
viết tay, rồi nhét nó vào một BIT. Dành riêng ô ĐẦU TIÊN (bên trái nhất)
làm **cờ dấu** — `0` nghĩa là số dương, `1` nghĩa là số âm. Bảy ô còn
lại đọc bình thường như bài 2 đã dạy, ra một con số gọi là ĐỘ LỚN.

```text title=readonly
0 0000101   →  dấu dương, độ lớn 5     →  +5
1 0000101   →  dấu âm,    độ lớn 5     →  -5
```

Bảy ô độ lớn chứa được từ `0000000` (0) tới `1111111` (127) — đúng
2⁷ = 128 mức. Cờ dấu nhân đôi số ấy lên, cho phạm vi trông có vẻ hợp lý:
từ -127 tới +127.
::::

::::example{#doi-dau-chi-doi-mot-o}
Đổi dấu một số theo cách này chỉ đụng vào ĐÚNG một ô — cờ dấu. Bảy ô độ
lớn đứng yên.

Viết cả tám ô dưới dạng một mẫu bit — một số nguyên 0-255, đúng cách
bài 4 đã đọc. Cờ dấu là ô có TRỌNG SỐ 128 (2⁷, ô ngoài cùng bên trái).
Tách nó ra khỏi bảy ô còn lại bằng đúng cặp phép chia bài 3 đã dạy:

```python title=readonly
def doc_kieu_ngay_tho(mau):
    dau = mau // 128     # ô cờ dấu — 0 hoặc 1
    do_lon = mau % 128   # bảy ô còn lại, đọc như bài 4
    if dau == 0:
        return do_lon
    else:
        return -do_lon

print(doc_kieu_ngay_tho(5))     # 00000101 — cờ 0, độ lớn 5
print(doc_kieu_ngay_tho(133))   # 10000101 — cờ 1, độ lớn 5
```

```text title=readonly
5
-5
```

Hai mẫu `5` và `133` chỉ khác nhau đúng ở ô cờ dấu — bảy ô độ lớn
`0000101` đứng yên ở cả hai. Nhìn thoáng qua, cách này gọn và hợp lý —
cho tới khi bạn thử một mẫu ít ai nghĩ tới.
::::

::::predict{#mau-la-nao commitOnce}
Byte thử mẫu `1 0000000` — cờ dấu là `1`, bảy ô độ lớn toàn số `0`.

**Trước khi xem đáp án**, áp đúng luật của cách ngây thơ (cờ dấu + độ
lớn) — bạn đoán mẫu này mang giá trị nào?

:::opt{correct}
Số không — giống hệt mẫu `0 0000000`, dù hai mẫu bit khác hẳn nhau.
:::

:::opt
`-128` — cờ `1` cộng bảy số `0` còn lại nghe như số âm lớn nhất có thể.
::why
Gần đúng ở chỗ bạn nhớ đúng luật: cờ `1` nghĩa là ÂM, và đi tìm "số âm
lớn nhất trong tám ô" là một phản xạ hợp lý.

Chỗ lệch: cách ngây thơ này không phải bù hai (bài sau mới nói tới bù
hai, và `-128` sẽ xuất hiện ở ĐÓ, không phải ở đây). Bảy ô sau cờ dấu
CHỈ LÀ độ lớn, đọc riêng, không dính gì tới cờ: `0000000` đọc ra 0, hết.
Số đối của 0 vẫn là 0 — không có "độ lớn âm cực đại" nào giấu trong bảy
ô toàn số không.
::
:::

:::opt
`128` — đọc nguyên tám ô như một số không dấu, giống hệt bài 4.
::why
Gần đúng ở chỗ TÍNH TOÁN: `10000000` đúng là bằng 128 nếu đọc theo cách
bài 4 — mọi ô đều là một cột giá trị, không ô nào đặc biệt.

Chỗ lệch: bài này không dùng cách đọc của bài 4 nữa. Cách ngây thơ RÚT
RIÊNG ô đầu ra khỏi hàng cột giá trị, biến nó thành một cờ báo dấu chứ
không còn mang trọng số 128 nào cả. Hai cách đọc khác nhau cho cùng một
dãy tám ô, và bài này đang dùng cách thứ hai.
::
:::

:::opt
Không hợp lệ — một số không thể vừa mang dấu âm vừa là số không.
::why
Gần đúng ở chỗ bạn ngửi thấy có gì đó kỳ quặc — trực giác ấy đúng, và
phần còn lại của bài này gọi tên chính xác điều kỳ quặc đó.

Chỗ lệch: cách ngây thơ không hề từ chối mẫu này hay báo lỗi gì cả. Nó
tính ra một giá trị đàng hoàng — `0` — y như mọi mẫu khác. Cái kỳ quặc
không nằm ở chỗ "không hợp lệ", mà ở chỗ mẫu NÀY và mẫu `00000000` —
hai dãy bit khác hẳn nhau — lại cùng tính ra đúng một giá trị.
::
:::
::::

::::explain{#hai-so-khong}
Bạn vừa tự tay tìm ra chỗ hỏng.

Một byte có 2⁸ = 256 mẫu bit khác nhau — bài 4 đã tính con số đó. Nhưng
cách ngây thơ chỉ tạo ra được 255 giá trị PHÂN BIỆT, không phải 256. Hai
mẫu `00000000` và `10000000` — khác nhau hoàn toàn về bit — lại cùng
mang một ý nghĩa: số không.

Nói cách khác, cách này có "số không dương" và "số không âm", hai tấm
áo cho cùng một con số. Một mẫu bit bị LÃNG PHÍ — nó có thể dùng để
biểu diễn một giá trị khác, nhưng lại bị buộc phải trùng nghĩa với một
mẫu đã có sẵn.

Đây không phải lỗi vặt bỏ qua được. Máy tính thật không dùng cách ngây
thơ này để lưu số âm. Bài sau chỉ cách khác — cách không hề có chuyện
hai số không, và tận dụng được cả 256 mẫu bit, không phí mẫu nào.
::::

::::code{#kiem-hai-so-khong}
Đến lượt bạn viết lại đúng hai phép chia vừa rồi — lần này để hàm tự nói
ra được cả bốn mẫu, kể cả mẫu `128` mang đúng chỗ hỏng của bài.

Ô cờ dấu là ô có TRỌNG SỐ 128 (2⁷) — ô ngoài cùng bên trái trong tám ô.
Tách nó ra khỏi bảy ô còn lại bằng đúng cặp phép chia bài 3 đã dạy: chia
lấy phần nguyên cho 128 tách ra cờ dấu, chia lấy phần dư tách ra độ
lớn.

```python title=starter
def y_nghia_ngay_tho(mau):
    dau = ___
    do_lon = ___
    if dau == 0:
        return do_lon
    else:
        return -do_lon

print(y_nghia_ngay_tho(5))
print(y_nghia_ngay_tho(133))
print(y_nghia_ngay_tho(0))
print(y_nghia_ngay_tho(128))
```

```python title=solution
def y_nghia_ngay_tho(mau):
    dau = mau // 128
    do_lon = mau % 128
    if dau == 0:
        return do_lon
    else:
        return -do_lon

print(y_nghia_ngay_tho(5))
print(y_nghia_ngay_tho(133))
print(y_nghia_ngay_tho(0))
print(y_nghia_ngay_tho(128))
```

```python title=test
# Bốn mẫu, mỗi mẫu chặn một kiểu nhầm khác nhau:
#   mau=5    (00000101) — mẫu dương thường, dò lỗi thô nhất trước.
#   mau=133  (10000101) — cờ dấu là 1, độ lớn 5: bắt lỗi đảo ngược phép
#                          chia (dùng % thay // và ngược lại).
#   mau=0    (00000000) — số không "dương".
#   mau=128  (10000000) — số không "âm": đúng chỗ hai số không của bài.
#                          Thiếu cảnh này thì một lời giải sai vẫn có
#                          thể qua lọt nếu chỉ tình cờ đúng ở ba mẫu
#                          trên.
assert y_nghia_ngay_tho(5) == 5, "mẫu 00000101 (5) phải mang giá trị 5 — cờ dấu là 0 nên đọc thẳng độ lớn"
assert y_nghia_ngay_tho(133) == -5, "mẫu 10000101 (133) phải mang giá trị -5 — cờ dấu là 1, độ lớn vẫn là 5"
assert y_nghia_ngay_tho(0) == 0, "mẫu 00000000 (0) phải mang giá trị 0"
assert y_nghia_ngay_tho(128) == 0, "mẫu 10000000 (128) cũng phải mang giá trị 0 — đúng chỗ hai mẫu bit cùng nghĩa 'số không' mà bài này chỉ ra"
```

:::hints
- kind: attention
  body: Ô cờ dấu mang trọng số 128, nên tách nó ra khỏi bảy ô còn lại bằng một phép chia cho 128 — không phải chia cho 2, và không phải chia cho 127.
- kind: strategy
  body: "`dau` là phần THƯƠNG khi chia `mau` cho 128 — với mau từ 0 tới 255, thương ấy chỉ ra được 0 hoặc 1, đúng bằng giá trị của một ô. `do_lon` là phần DƯ của phép chia đó — bảy ô còn lại, đọc như một số bình thường."
- kind: one-line
  body: "Chỗ trống thứ nhất là `mau // 128`, chỗ trống thứ hai là `mau % 128`."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^5\n-5\n0\n0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`0` và `0` — hai mẫu bit khác hẳn nhau, cùng ra một con số. Đúng chỗ
hỏng bạn vừa tự tay tìm thấy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cách ngây thơ hỏng ở chỗ RÚT RIÊNG một ô ra làm cờ, tách nó khỏi hàng
cột giá trị. Nhưng nếu không rút ô nào ra cả — nếu số âm vẫn được tính
bằng đúng tám ô, đúng cách đọc của bài 4, không có ô nào "đặc biệt" —
thì số âm phải nằm ở đâu trong 256 mẫu bit đó?

Bài sau chỉ cách máy tính thật làm — và cách ấy không hề có khái niệm
"cờ dấu" nào cả.
::::

::::checkpoint{mastery=0.8}
::::
