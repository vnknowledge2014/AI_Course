---
id: nen-tang.list-dict-set-tuple.phan-rieng-cua-ben-nay
title: Phần riêng của một bên
summary: Dấu `-` giữa hai rổ giữ lại những gì bên trái có mà bên phải không — và khác `&`, đổi chỗ hai rổ là đổi hẳn câu trả lời.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 29
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.set-difference]
requires: [core.set-intersection, core.set, core.set-unordered, core.list-membership, core.list, core.list-append, core.len, ctrl.for-each, ctrl.if, core.fstring]
concepts: [core.tap-hop, core.cho-chua]
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
Lần này mình không hỏi "cả hai cùng có". Mình hỏi "chỉ bên này có".
::::

::::explain{#cau-hoi-nghe-giong-ma-khong-phai}
Vẫn hai cái rổ của bài trước, không đổi một nhóm nào:

- Tháng này: `ăn uống`, `xăng xe`, `học phí` — ba nhóm.
- Tháng trước: `ăn uống`, `xăng xe`, `biếu tặng`, `thuê nhà` — bốn nhóm.

Phần chung, bài trước đã tính: `ăn uống` và `xăng xe` — hai nhóm.

Byte hỏi câu mới: **nhóm nào tháng này mới phát sinh, tháng trước chưa từng
có?** Nhìn bằng mắt thì thấy ngay: `học phí`.

Câu này nghe giống câu cũ, nhưng nó là câu ngược lại. `&` giữ lại đúng hai nhóm
mà câu này muốn **vứt đi**. Đem `&` ra dùng thì bạn nhận về một cái rổ trông
chỉnh tề và trả lời sai.

Đồ nghề cũ vẫn làm được. Duyệt rổ tháng này, nhóm nào **không** nằm trong rổ
tháng trước thì nhặt sang một chỗ chứa mới:

```python title=readonly
moi_phat_sinh = []
for nhom in thang_nay:
    if nhom not in thang_truoc:
        moi_phat_sinh.append(nhom)
```

Vẫn đúng bốn dòng của bài trước, đổi mỗi câu `if`: thêm một chữ `not`. Chữ ấy
nhỏ tới mức đọc lướt là trượt — mà nó lật ngược toàn bộ ý nghĩa của đoạn.

Ý "có ở bên này mà không có ở bên kia" cũng có tên sẵn: **phần riêng**. Nhà toán
học gọi là phép **hiệu**. Python viết nó bằng dấu trừ, đúng dấu bạn đã dùng cho
số từ Realm 0:

```python title=readonly
moi_phat_sinh = thang_nay - thang_truoc
```

Đọc thành lời: *lấy cả rổ bên trái, rồi bỏ đi những gì bên phải cũng có.*
::::

::::example{#hai-chieu-hai-cau-tra-loi}
Chạy phép hiệu theo cả hai chiều, trên đúng hai cái rổ ấy.

```python title=readonly
thang_nay = {"ăn uống", "xăng xe", "học phí"}
thang_truoc = {"ăn uống", "xăng xe", "biếu tặng", "thuê nhà"}

moi_phat_sinh = thang_nay - thang_truoc
khong_thay_lai = thang_truoc - thang_nay

print(len(moi_phat_sinh))
print("học phí" in moi_phat_sinh)
print("ăn uống" in moi_phat_sinh)

print(len(khong_thay_lai))
print("thuê nhà" in khong_thay_lai)
```

Máy in ra:

```text title=readonly
1
True
False
2
True
```

Ba dòng đầu nói về chiều thứ nhất: `moi_phat_sinh` giữ đúng 1 nhóm, `học phí`
nằm trong đó, còn `ăn uống` thì không — tháng trước cũng chi nhóm ấy, nên nó bị
gạch đi.

Hai dòng cuối là chỗ đáng dừng lại. Cùng một dấu trừ, cùng hai cái rổ, mà đặt
ngược lại thì ra một con số khác: `thang_truoc - thang_nay` giữ **2** nhóm, và
hai nhóm ấy là `biếu tặng` với `thuê nhà`.

Cách đếm ra hai con số ấy là cùng một cách: **lấy số nhóm của rổ bên trái, trừ đi
phần chung**. Phần chung là 2 nhóm, bài trước đã tính. Chiều thứ nhất, rổ bên
trái có 3 nhóm, còn lại 3 − 2 = 1. Chiều ngược lại, rổ bên trái có 4 nhóm, còn
lại 4 − 2 = 2.
::::

::::explain{#tru-khong-giong-tru-so}
Đây là chỗ `-` giữa hai rổ khác hẳn `&`, và cũng khác dấu trừ giữa hai con số.

- **Đổi chỗ hai rổ là đổi câu trả lời.** `&` hỏi "có mặt ở cả hai không" — một
  câu không nêu tên bên nào đứng trước, nên đổi chỗ vô hại. `-` hỏi "có mặt ở bên
  **này** mà không có ở bên **kia**" — câu ấy tự nó phát cho hai cái rổ hai vai
  khác nhau. Rổ nào đứng bên trái thì phần riêng lấy được là phần riêng của rổ ấy.
- **Rổ bên phải chỉ đóng vai người gạch tên.** Không một nhóm nào của rổ bên
  phải đi vào kết quả. `học phí` nằm trong `thang_nay`, nhưng
  `thang_truoc - thang_nay` không hề chứa `học phí` — kết quả chỉ lấy từ rổ bên
  trái.
- **Nhóm chỉ có ở bên phải thì bị lờ đi, không gây lỗi.** Dấu trừ giữa hai con số
  có thể cho ra số âm; giữa hai cái rổ thì không có "âm" nào cả. Bên phải có
  `học phí` mà bên trái không có, máy chỉ đơn giản không tìm thấy gì để gạch.

Kết quả của `-` vẫn là một **rổ**: không chứa hai lần cùng một nhóm, không xếp
hàng, hỏi `in` thì trả lời ngay — đủ tính nết của bài 26 và bài 27. Và cũng như
`&`, nó dựng ra một rổ mới: sau hai dòng trên, `thang_nay` vẫn đủ ba nhóm,
`thang_truoc` vẫn đủ bốn.

> Chỗ dễ vấp: khi mọi nhóm của rổ bên trái đều có mặt ở rổ bên phải, `a - b` cho
> ra một rổ **rỗng** — `len` bằng `0`. Rỗng ở đây là một câu trả lời thật, nghĩa
> là "bên này chẳng có gì mà bên kia không có", chứ không phải máy quên làm việc.
::::

::::predict{#doi-cho-thi-sao commitOnce}
Byte viết hai dòng, cùng một phép hiệu nhưng hai cái rổ đặt theo hai thứ tự
ngược nhau.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
thang_nay = {"ăn uống", "xăng xe", "học phí"}
thang_truoc = {"ăn uống", "xăng xe", "biếu tặng", "thuê nhà"}

print(len(thang_nay - thang_truoc))
print(len(thang_truoc - thang_nay))
```

:::opt{correct}
`1` rồi `2`
:::

:::opt
`1` rồi `1` — đổi chỗ vẫn ra chừng ấy, y như `&` ở bài trước
::why
Gần đúng ở chỗ bạn mang một phát hiện thật của bài trước sang bài này, và con số
`1` cho dòng đầu thì chính xác: `học phí` là nhóm duy nhất tháng này có mà tháng
trước không có.

Chỗ lệch nằm ở chỗ hai phép hỏi hai câu khác nhau. `&` hỏi "có mặt ở cả hai
không" — câu đó không nêu tên bên nào đứng trước. `-` hỏi "có ở bên **này** mà
không có ở bên **kia**", tức là nó phát cho hai cái rổ hai vai khác hẳn nhau: một
bên được giữ lại, một bên đi gạch tên. Đổi vai thì đổi kết quả.
::
:::

:::opt
`2` rồi `1`
::why
Gần đúng tới mức chỉ còn một bước: hai con số `1` và `2` đúng là hai con số bài
này cho ra, bạn đã tính ra cả hai.

Chỗ lệch là con số nào ứng với dòng nào. Trong `a - b`, phần được giữ lại lấy từ
**`a`**, tức là rổ đứng trước dấu trừ. Dòng đầu để `thang_nay` đứng trước, nên nó
giữ phần riêng của tháng này — chỉ `học phí`, một nhóm. Dòng sau để `thang_truoc`
đứng trước, giữ phần riêng của tháng trước — `biếu tặng` và `thuê nhà`, hai nhóm.
::
:::

:::opt
`3` rồi `4` — trừ nghĩa là bỏ hẳn rổ bên phải đi, còn nguyên rổ bên trái
::why
Gần đúng ở chỗ bạn đọc dấu `-` theo đúng nghĩa quen thuộc của nó: bớt cái này ra
khỏi cái kia. Nghĩa ấy không sai, chỉ chưa đủ chi tiết.

Chỗ lệch: máy không bỏ cả rổ bên phải, nó chỉ gạch **từng nhóm** của bên phải ra
khỏi bên trái, và chỉ gạch được những nhóm bên trái thật sự có. Hai nhóm bị gạch
chính là phần chung của bài trước. Rổ bên trái ở dòng đầu có 3 nhóm nên còn
3 − 2 = 1; ở dòng sau có 4 nhóm nên còn 4 − 2 = 2.
::
:::
::::

::::code{#hai-chieu-cua-mot-dau-tru}
Byte muốn hai con số cho bản tổng kết: **tháng này phát sinh thêm mấy nhóm**, và
**mấy nhóm của tháng trước không thấy lại ở tháng này**.

Hai chỗ trống, cùng một dấu, khác nhau ở thứ tự hai cái rổ. Đặt ngược là ra
ngược.

```python title=starter
thang_nay = {"ăn uống", "xăng xe", "học phí"}
thang_truoc = {"ăn uống", "xăng xe", "biếu tặng", "thuê nhà"}

moi_phat_sinh = ___
khong_thay_lai = ___

print(f"Tháng này phát sinh thêm: {len(moi_phat_sinh)} nhóm")
print(f"Tháng trước có mà tháng này không: {len(khong_thay_lai)} nhóm")
```

```python title=solution
thang_nay = {"ăn uống", "xăng xe", "học phí"}
thang_truoc = {"ăn uống", "xăng xe", "biếu tặng", "thuê nhà"}

moi_phat_sinh = thang_nay - thang_truoc
khong_thay_lai = thang_truoc - thang_nay

print(f"Tháng này phát sinh thêm: {len(moi_phat_sinh)} nhóm")
print(f"Tháng trước có mà tháng này không: {len(khong_thay_lai)} nhóm")
```

```python title=test
# Kiểm nội dung từng rổ chứ không kiểm mỗi `len`: đặt ngược hai cái rổ vẫn có
# thể ra một con số trông hợp lý, mà nội dung thì khác hẳn.
assert moi_phat_sinh == {"học phí"}, "trong ba nhóm của tháng này, chỉ học phí là nhóm tháng trước không có"
assert khong_thay_lai == {"biếu tặng", "thuê nhà"}, "hai nhóm của tháng trước không thấy lại ở tháng này là biếu tặng và thuê nhà"
assert "ăn uống" not in moi_phat_sinh, "ăn uống có mặt ở cả hai rổ, nên nó bị gạch khỏi phần riêng của tháng này"
```

:::hints
- kind: attention
  body: Hai chỗ trống dùng đúng một dấu, và cả hai đều cần đủ hai cái tên rổ ở phía trên. Điều duy nhất khác nhau giữa hai dòng là cái tên nào được đặt **trước** dấu ấy — hãy đọc lại tên biến ở vế trái mỗi dòng để biết phần riêng đang hỏi về tháng nào.
- kind: strategy
  body: Phần được giữ lại luôn lấy từ rổ đứng **trước** dấu trừ; rổ đứng sau chỉ đi gạch tên. Dòng thứ nhất hỏi về nhóm mới của tháng này, nên rổ tháng này phải đứng trước. Dòng thứ hai hỏi ngược lại, nên hai cái tên cũng đổi chỗ cho nhau.
- kind: one-line
  body: "Dòng thứ nhất viết `thang_nay - thang_truoc`, dòng thứ hai viết `thang_truoc - thang_nay`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Tháng này phát sinh thêm: 1 nhóm
- tier: output
  expect: Tháng trước có mà tháng này không: 2 nhóm
- tier: static
  onFail: mỗi dòng phải đem CẢ HAI cái rổ ra, chứ không gõ sẵn tên nhóm vào
  requireAst:
  - kind: uses-name, target: thang_nay, min: 2
  - kind: uses-name, target: thang_truoc, min: 2
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một dấu trừ. Đổi chỗ hai cái rổ là đổi hẳn câu hỏi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ngoảnh lại ba bài vừa rồi, cả ba đều mở đầu bằng đúng một khuôn tay:

- Bài 25, gom các nhóm chi đã gặp: một chỗ chứa rỗng, một vòng `for`, một câu
  `if`, một `.append`.
- Bài 28, dựng phần chung bằng tay: đúng bốn dòng ấy.
- Bài 29, dựng phần riêng bằng tay: vẫn bốn dòng ấy, thêm một chữ `not`.

Với hai cái rổ thì bạn đã có `&` và `-` để nói gọn lại. Nhưng khuôn tay ấy còn
xuất hiện ở những chỗ chẳng liên quan gì tới rổ. Quay lại cuốn sổ của bài 23 —
một danh sách các khoản, mỗi khoản là một dict:

```python title=readonly
ten_cac_khoan = []
for khoan in so:
    ten = khoan["ten"]
    ten_cac_khoan.append(ten)
```

Bốn dòng chỉ để nói đúng một câu: *lấy tên của mọi khoản trong sổ*. Không có điều
kiện nào, không có phép tính nào, không có gì để đắn đo.

Nói nguyên câu ấy trong **một dòng** được không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
