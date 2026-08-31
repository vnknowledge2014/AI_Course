---
id: khoa-hoc-may-tinh.may-chay-the-nao.re-nhanh-la-nhay-co-dieu-kien
title: "Rẽ nhánh if là một lệnh nhảy CÓ ĐIỀU KIỆN"
summary: "dis.dis() một if/else cho thấy COMPARE_OP rồi POP_JUMP_IF_FALSE — chạy thật trên CPython 3.13. Đối xứng với bài trước: if dịch thành một lệnh nhảy CÓ ĐIỀU KIỆN, nhảy tới nhánh else nếu sai, đi tiếp nếu đúng — cùng công cụ với vòng lặp, chỉ khác điều kiện và hướng."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.branch-is-jump]
requires: [may.loop-is-jump]
concepts: [may.branch-is-jump]
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
Có — nhưng nhảy CÓ ĐIỀU KIỆN, và không bao giờ nhảy lùi.
::::

::::explain{#nhay-co-dieu-kien}
Bài trước: `JUMP_BACKWARD` luôn được thực hiện, không hỏi han gì, và
luôn đưa vị trí đang chạy về một chỗ ĐÃ đi qua. `if` cần một công cụ
khác hẳn — nó không lặp lại, nó CHỌN, và chỉ chọn một trong hai đường
đi TỚI, chưa đi qua bao giờ.

Hai lệnh mới làm việc đó. **`COMPARE_OP`** — lệnh so sánh — lấy đúng hai
giá trị ở đỉnh ngăn xếp tính toán (bài 3!) ra, so chúng theo đúng phép
so sánh ghi trong ngoặc (`>=`, `==`,...), rồi đẩy kết quả `True` hoặc
`False` trở lại đỉnh.

**`POP_JUMP_IF_FALSE`** — lấy đúng MỘT giá trị ở đỉnh ngăn xếp tính
toán ra (đúng kết quả `COMPARE_OP` vừa đẩy vào). Nếu giá trị đó là
`False`, nó NHẢY tới một nhãn khác — thường là nơi bắt đầu nhánh
`else`. Nếu là `True`, nó không nhảy đi đâu cả — dòng chạy cứ rơi thẳng
xuống lệnh KẾ TIẾP, đúng là nhánh `if`.

Đối chiếu với bài trước: `JUMP_BACKWARD` không điều kiện, luôn đi
NGƯỢC. `POP_JUMP_IF_FALSE` có điều kiện — chỉ nhảy khi sai — và luôn đi
TỚI, tới một đoạn mã CHƯA chạy lần nào. "Rẽ nhánh" và "lặp lại" hoá ra
dùng cùng một loại công cụ (lệnh nhảy), chỉ khác điều kiện và hướng.
::::

::::example{#phan-loai-tuoi}
```python title=readonly
import dis

def phan_loai(tuoi):
    if tuoi >= 18:
        return "lon"
    else:
        return "nho"

dis.dis(phan_loai)
```

```text title=readonly
  3           RESUME                   0

  4           LOAD_FAST                0 (tuoi)
              LOAD_CONST               1 (18)
              COMPARE_OP             188 (bool(>=))
              POP_JUMP_IF_FALSE        1 (to L1)

  5           RETURN_CONST             2 ('lon')

  7   L1:     RETURN_CONST             3 ('nho')
```

`COMPARE_OP 188 (bool(>=))` tính `tuoi >= 18`, đẩy `True`/`False`.
`POP_JUMP_IF_FALSE 1 (to L1)` lấy giá trị đó ra: nếu `False`, nhảy tới
`L1` — đúng chỗ `return "nho"` đứng. Nếu `True`, không nhảy — dòng chạy
rơi thẳng xuống `RETURN_CONST 'lon'`, dòng NGAY SAU nó.

Đúng một lệnh so sánh, đúng một lệnh nhảy có điều kiện — không cần hơn,
kể cả khi có hai nhánh.
::::

::::predict{#du-doan-xep-loai commitOnce}
```python
def xep_loai(diem):
    if diem >= 8:
        xep = "gioi"
    else:
        xep = "chua gioi"
    return xep
```

Bốn tên lệnh dưới đây, chỉ MỘT trong số đó là lệnh THẬT mà CPython 3.13
dùng để hiện thực `if diem >= 8:`. Đâu là lệnh đúng?

:::opt{correct}
POP_JUMP_IF_FALSE
:::

:::opt
JUMP_IF_FALSE
::why
Gần đúng ở việc bạn đoán đúng phần "nhảy nếu sai" — đúng tinh thần của
lệnh thật.

Chỗ lệch: tên thật LUÔN có tiền tố `POP_`. Đó không phải chi tiết trang
trí — nó nói rõ lệnh này còn làm thêm một việc trước khi nhảy: LẤY
(pop) giá trị so sánh ra khỏi ngăn xếp tính toán. Thiếu chữ `POP_` là bỏ
sót đúng việc bài 3 vừa dạy: giá trị so sánh phải được LẤY ra khỏi ngăn
xếp, không phải cứ nằm đó "nhìn" rồi nhảy.
::
:::

:::opt
JUMP_BACKWARD
::why
Gần đúng ở việc bạn nhớ đúng: đây LÀ một lệnh nhảy, có thật, học ở bài
trước.

Chỗ lệch: `JUMP_BACKWARD` là công cụ RIÊNG của vòng lặp — luôn đi NGƯỢC
về một chỗ đã chạy qua. `if` không lặp lại gì cả, nó chỉ chọn một nhánh
rồi đi TỚI — không có gì để nhảy lùi về.
::
:::

:::opt
GOTO_ELSE
::why
Gần đúng ở việc bạn hiểu đúng Ý của lệnh — có một bước "đi tới nhánh
else nếu sai".

Chỗ lệch: không có lệnh nào tên như vậy trong bytecode Python. Máy
không có một lệnh nhảy RIÊNG cho từng trường hợp dùng (không có lệnh
riêng cho `else`, riêng cho việc thoát vòng lặp,...). Nó chỉ có một
công cụ nhảy có điều kiện DÙNG CHUNG — `POP_JUMP_IF_FALSE` — cho mọi
chỗ cần rẽ theo một điều kiện đúng/sai.
::
:::
::::

::::code{#xep-hang-diem}
Xếp hạng theo điểm: từ 5 trở lên là đạt. Nhánh `else` đã viết sẵn —
điền đúng chỗ trống ở nhánh `if`.

```python title=starter
import dis

def xep_hang(diem):
    if diem >= 5:
        ket_qua = ___                  # "đạt"
    else:
        ket_qua = "chưa đạt"
    return ket_qua

print(xep_hang(7))
print(xep_hang(3))
dis.dis(xep_hang)
```

```python title=solution
import dis

def xep_hang(diem):
    if diem >= 5:
        ket_qua = "đạt"
    else:
        ket_qua = "chưa đạt"
    return ket_qua

print(xep_hang(7))
print(xep_hang(3))
dis.dis(xep_hang)
```

```python title=test
assert xep_hang(7) == "đạt", f"7 điểm phải xếp 'đạt' — đang ra {xep_hang(7)!r}"
assert xep_hang(5) == "đạt", f"đúng 5 điểm (biên >=) phải xếp 'đạt' — đang ra {xep_hang(5)!r}"
assert xep_hang(3) == "chưa đạt", f"3 điểm phải xếp 'chưa đạt' — đang ra {xep_hang(3)!r}"
```

:::hints
- kind: attention
  body: Chỉ một chỗ trống — giá trị gán cho ket_qua khi diem đạt từ 5 trở lên. Nhánh else đã có sẵn "chưa đạt", không cần đụng vào.
- kind: strategy
  body: Gán đúng chữ "đạt" (có dấu, viết thường) vào ket_qua ở nhánh if — cùng kiểu chữ với nhánh else đã cho.
- kind: one-line
  body: 'Chỗ trống là: "đạt"'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^đạt\\nchưa đạt\\n"
- tier: output
  expect: "POP_JUMP_IF_FALSE"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một lệnh so sánh, một lệnh nhảy có điều kiện — POP_JUMP_IF_FALSE lấy
kết quả so sánh ra, nhảy nếu sai, đứng yên nếu đúng. Không nhảy lùi lần
nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Năm bài vừa qua cho bạn xem đúng DANH SÁCH lệnh máy — `LOAD_FAST`,
`BINARY_OP`, `JUMP_BACKWARD`, `POP_JUMP_IF_FALSE`, và vài lệnh khác.
Nhưng một danh sách nằm yên trên giấy không tự làm gì cả. Cái gì THẬT
SỰ đọc từng dòng trong danh sách đó, hiểu nó là lệnh gì, rồi thực hiện
đúng việc — trước khi chuyển sang dòng kế tiếp?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
