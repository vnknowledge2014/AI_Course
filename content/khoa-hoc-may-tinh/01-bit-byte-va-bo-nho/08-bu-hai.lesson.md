---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.bu-hai
title: "Bù hai: cách máy thật sự nhớ số âm"
summary: Số đối của n trong một byte là số cộng vào n để tràn đúng hết tám ô, ra số không — không cờ dấu nào, chỉ có một phép cộng tràn.
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [mem.twos-complement]
requires: [mem.sign-bit, mem.byte-range, core.arithmetic, core.function-def, core.function-parameter, core.function-return, ctrl.comparison, core.boolean, core.variable, core.print-variable]
concepts: [mem.twos-complement, mem.so-doi]
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
Cách ngây thơ có hai số không. Máy tính thật không rút ô nào ra làm cờ
cả — nó giải quyết chuyện dấu âm bằng một phép cộng.
::::

::::explain{#cau-hoi-khac-han}
Cách ngây thơ hỏi: "ô nào là cờ dấu?" Câu hỏi ấy dẫn thẳng tới chỗ hỏng
của bài trước.

Máy tính thật hỏi một câu khác hẳn, không nhắc gì tới "dấu" cả: **cộng
với số dương n bao nhiêu thì tràn hết một byte, còn lại đúng số
không?**

Một byte có 256 mẫu bit (bài 4). Cộng hai số mà tổng vừa đúng 256, rồi
chỉ GIỮ LẠI tám ô cuối — bỏ đi phần tràn ra ngoài về bên trái, giống kim
đồng hồ quay hết một vòng mười hai giờ rồi quay lại số mười hai — thì
tám ô còn lại đọc ra đúng số 0.

Con số cộng vào `n` để ra 256 chính là `256 - n`. Gọi nó là **số đối**
của `n` trong một byte — và số đối đó chính là mẫu bit dùng để biểu
diễn `-n`.

```text title=readonly
n = 5:  số đối = 256 - 5 = 251
        5 + 251 = 256  →  cắt còn tám ô  →  0
```

Cách gán nghĩa này gọi là **bù hai** (tiếng Anh: *two's complement*).
Không có ô nào bị rút riêng ra làm cờ. Cả tám ô vẫn đọc y hệt cách bài 4
đã dạy — số đối chỉ là MỘT SỐ như mọi số khác, tình cờ mang nghĩa "âm"
vì cách nó cộng ra 0.
::::

::::example{#kiem-bang-python}
Python có sẵn một mẹo để tính số đối trong một byte, dùng dấu `&` — dấu
này bài 11 mới giải thích kỹ, ở đây chỉ mượn nó làm công cụ ĐỐI CHIẾU.
`& 0xff` cắt một số nguyên xuống còn đúng tám ô cuối cùng, bỏ hết phần
bên trái.

```python title=readonly
print(-5 & 0xff)
print(bin(-5 & 0xff))
print((5 + (-5 & 0xff)) & 0xff)
```

```text title=readonly
251
0b11111011
0
```

`-5 & 0xff` ra `251` — đúng bằng `256 - 5` bạn vừa tính tay. Dòng cuối
kiểm lại đúng định nghĩa: cộng `5` với số đối của nó, cắt còn tám ô, ra
`0`.

Còn một chỗ đáng dừng lại: mẫu `10000000` (128) — mẫu từng là "số không
âm" gây rắc rối ở bài trước — giờ mang nghĩa gì?

```python title=readonly
print(-128 & 0xff)
```

```text title=readonly
128
```

`128` chính là số đối của `128` trong bù hai — nghĩa là mẫu `10000000`
giờ biểu diễn `-128`, một giá trị RIÊNG, không trùng với số không nữa.
Chỗ từng lãng phí ở bài trước, bù hai tận dụng trọn vẹn.
::::

::::predict{#tim-so-doi-cua-12 commitOnce}
Áp đúng định nghĩa: số đối của `n` là số cộng vào `n` để tràn hết một
byte, ra đúng 0.

**Trước khi xem đáp án**, bạn đoán số đối của `12` trong một byte là số
nguyên KHÔNG DẤU nào (0 tới 255)?

:::opt{correct}
`244` — vì `256 - 12 = 244`, và `12 + 244 = 256`.
:::

:::opt
`243`
::why
Gần đúng ở chỗ `255 - 12 = 243` chính là kết quả của việc ĐẢO NGƯỢC
từng ô (0 thành 1, 1 thành 0) — đúng bước đầu của công thức "đảo bit
rồi cộng 1" mà bù hai còn dùng theo cách khác.

Chỗ lệch: bạn dừng lại trước bước cộng 1 cuối cùng. Cộng `243` với `12`
chỉ ra `255` — toàn số 1, CHƯA tràn hết ra 0, còn thiếu đúng 1. Thêm 1
nữa (`243 + 1 = 244`) thì `12 + 244 = 256`, và cắt còn tám ô mới thật
sự ra 0.
::
:::

:::opt
`-12`
::why
Gần đúng ở chỗ đó đúng là giá trị bạn MUỐN biểu diễn — không sai ý
nghĩa cuối cùng.

Chỗ lệch: câu hỏi hỏi MẪU BIT bên trong byte, mà mọi mẫu trong tám ô
đều đọc ra một số nguyên KHÔNG ÂM, từ 0 tới 255 — không ô nào mang dấu
trừ riêng. `-12` không phải một mẫu tám-bit hợp lệ để đọc trực tiếp; nó
là ý nghĩa mà mẫu `244` được GÁN cho, không phải chính cái mẫu đó.
::
:::

:::opt
`116`
::why
Gần đúng ở chỗ `128 - 12 = 116`, và `128` đúng là con số trung tâm của
bài TRƯỚC — cách ngây thơ dùng nó làm ranh giới dương/âm.

Chỗ lệch: bù hai không dùng cách ngây thơ đó nữa. Số đối ở đây được
định nghĩa qua chuyện TRÀN HẾT CẢ TÁM Ô, tức modulus là 256 (2⁸ — toàn
bộ số mẫu bit của một byte), không phải 128 (2⁷ — chỉ bảy ô độ lớn của
cách ngây thơ). Hai bài dùng hai con số trung tâm khác nhau, vì hai
cách đọc khác nhau.
::
:::
::::

::::code{#viet-ham-so-doi}
Viết hàm tính số đối trong một byte — đúng công thức bạn vừa dùng: cộng
với `n` bao nhiêu để tràn hết một byte, ra 0.

Ba dòng `kiem_...` bên dưới đối chiếu kết quả của bạn với mẹo `& 0xff`
của Python — nếu cả hai cách tính trùng nhau, chúng phải in ra `True`.

```python title=starter
def so_doi_trong_byte(n):
    return ___

kiem_5 = so_doi_trong_byte(5) == (-5 & 0xff)
kiem_12 = so_doi_trong_byte(12) == (-12 & 0xff)
kiem_128 = so_doi_trong_byte(128) == (-128 & 0xff)

print(so_doi_trong_byte(5))
print(kiem_5)
print(kiem_12)
print(kiem_128)
```

```python title=solution
def so_doi_trong_byte(n):
    return 256 - n

kiem_5 = so_doi_trong_byte(5) == (-5 & 0xff)
kiem_12 = so_doi_trong_byte(12) == (-12 & 0xff)
kiem_128 = so_doi_trong_byte(128) == (-128 & 0xff)

print(so_doi_trong_byte(5))
print(kiem_5)
print(kiem_12)
print(kiem_128)
```

```python title=test
# Ba mẫu, mỗi mẫu chặn một kiểu nhầm khác nhau:
#   n=5   — dò lỗi thô nhất; cũng bắt lỗi "bù MỘT" (255-n, quên +1).
#   n=12  — cùng dạng lỗi bù-một, giá trị khác để không trùng ngẫu
#           nhiên với mẫu 5.
#   n=128 — mẫu biên: bắt lỗi dùng nhầm modulus 128 (của bài TRƯỚC)
#           thay vì 256. Với n=128 hai công thức 256-n và 128-n cho
#           hai kết quả xa nhau nhất (128 so với 0), nên đây là cảnh
#           phân biệt rõ nhất giữa hai modulus.
assert so_doi_trong_byte(5) == 251, "số đối của 5 trong một byte phải là 251 (256 - 5)"
assert so_doi_trong_byte(12) == 244, "số đối của 12 trong một byte phải là 244 (256 - 12)"
assert so_doi_trong_byte(128) == 128, "số đối của 128 trong một byte phải là 128 (256 - 128) — đúng mẫu biên nối sang bài trước"
assert kiem_5 is True, "so_doi_trong_byte(5) phải khớp với mẹo -5 & 0xff của Python"
assert kiem_12 is True, "so_doi_trong_byte(12) phải khớp với mẹo -12 & 0xff của Python"
assert kiem_128 is True, "so_doi_trong_byte(128) phải khớp với mẹo -128 & 0xff của Python"
```

:::hints
- kind: attention
  body: Chỗ trống là một phép TRỪ — con số cộng vào `n` để tràn hết một byte ra 0. Đừng dùng dấu `&` ở đây; dòng `& 0xff` bên dưới chỉ để ĐỐI CHIẾU, không phải cách bạn cần viết.
- kind: strategy
  body: Một byte tràn hết ở 256 (2⁸ mẫu bit, đúng bài 4). Số đối của `n` là khoảng cách còn thiếu từ `n` tới 256 — chính là `256 - n`.
- kind: one-line
  body: "Chỗ trống là `256 - n`."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^251\nTrue\nTrue\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`251`, và cả ba phép đối chiếu đều khớp với Python. Không cờ dấu nào,
chỉ một phép cộng tràn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy `5 + 251`, cắt còn tám ô, ra `0` — đúng như định nghĩa số
đối. Nhưng `251` là một số ĐÃ ĐƯỢC GÁN nghĩa "âm" từ trước. Nếu hai số
CÙNG DƯƠNG cộng lại mà vượt quá 255 — chẳng hạn `200 + 100 = 300` — thì
cắt còn tám ô, phần thừa đi đâu? Và con số cắt ra được đó còn ĐÚNG
NGHĨA hay không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
