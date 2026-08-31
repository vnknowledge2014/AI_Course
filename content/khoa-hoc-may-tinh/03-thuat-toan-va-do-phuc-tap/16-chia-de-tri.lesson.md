---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.chia-de-tri
title: "Chia để trị: cắt nhỏ, giải từng phần, ghép lại"
summary: "Đặt tên cho khuôn chung của hai bài trước: CẮT bài toán làm đôi, GIẢI từng nửa bằng chính lời gọi đệ quy, GHÉP kết quả lại. Tìm nhị phân là ca ghép đơn giản nhất — bước Ghép chỉ trả thẳng kết quả của đúng nửa đã tìm ra."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.divide-and-conquer]
requires: [alg.binary-search-recursive]
concepts: [alg.divide-and-conquer]
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
Cắt. Giải. Ghép. Ba việc nhỏ, mà tìm nhị phân bằng đệ quy đã âm thầm làm
đủ cả ba — chỉ là chưa ai gọi tên chúng.
::::

::::explain{#ba-viec-cua-mot-khuon}
Nhìn lại hàm bài trước với con mắt khác — không hỏi "nó tìm kiếm thế
nào", mà hỏi "nó XỬ LÝ một bài toán lớn theo khuôn nào". Ba việc lộ ra:

- **Cắt** — chia đoạn `[trai, phai]` đang xét làm đôi tại ô `giua`.
- **Giải** — gọi lại chính hàm đó trên MỘT nửa (đệ quy), coi đó là một
  bài toán độc lập, nhỏ hơn bài toán ban đầu.
- **Ghép** — lấy kết quả của lời gọi đệ quy đó, biến nó thành câu trả
  lời cho bài toán lớn ban đầu.

Khuôn ba bước này có tên: **chia để trị** (divide and conquer). Nó không
phải một thuật toán cụ thể — nó là một CÁCH TIẾP CẬN mà nhiều thuật toán
khác nhau đều tuân theo, tìm nhị phân chỉ là ca đơn giản nhất trong số
đó. Ở tìm nhị phân, bước GHÉP gần như không có gì để làm: kết quả của
bài toán lớn CHÍNH LÀ kết quả của đúng nửa đã chứa giá trị cần tìm —
`return` thẳng, không cần tính toán gì thêm.

Không phải mọi bài toán chia để trị đều có bước Ghép nhàn như vậy. Có
những bài toán mà GHÉP mới là phần việc nặng — giải xong cả hai nửa rồi
còn phải LÀM VIỆC để kết hợp hai kết quả riêng lẻ ấy thành một kết quả
chung. Cụm bài sau (sắp xếp) sẽ cho gặp đúng một ca như vậy.
::::

::::example{#tim-max-chia-de-tri}
Một bài toán chia để trị khác, không liên quan gì tới tìm kiếm: tìm giá
trị LỚN NHẤT trong một mảng, bằng cách CẮT đôi, GIẢI từng nửa, rồi GHÉP
bằng cách lấy giá trị lớn hơn trong hai kết quả:

```python title=readonly
def tim_max_chia_de_tri(mang, trai, phai):
    if trai == phai:                            # chỉ còn một ô -> Cắt dừng, tự nó là kết quả
        return mang[trai]
    giua = (trai + phai) // 2
    max_trai = tim_max_chia_de_tri(mang, trai, giua)          # Giải nửa trái
    max_phai = tim_max_chia_de_tri(mang, giua + 1, phai)      # Giải nửa phải
    return max(max_trai, max_phai)                            # Ghép: lấy giá trị lớn hơn

diem = [67, 92, 45, 78, 23, 89, 56, 34]
print(tim_max_chia_de_tri(diem, 0, len(diem) - 1))
```

```text title=readonly
92
```

Ba việc hiện rõ ràng: **Cắt** đoạn `[0, 7]` tại ô giữa; **Giải** đệ quy
trên `[0, 3]` và `[4, 7]` — mỗi nửa lại tự CẮT tiếp cho tới khi chỉ còn
một ô (ca dừng, không cắt được nữa); **Ghép** bằng `max(...)`, so hai
kết quả của hai nửa, giữ cái lớn hơn. Khác tìm nhị phân, việc GHÉP ở đây
có làm việc thật — nó phải SO SÁNH, không chỉ trả thẳng một trong hai
kết quả.
::::

::::predict{#doan-buoc-ghep commitOnce}
Một người viết một hàm chia để trị khác — TÍNH TỔNG một mảng — nhưng
viết SAI bước Ghép:

```python
def tinh_tong_hong(mang, trai, phai):
    if trai == phai:
        return mang[trai]
    giua = (trai + phai) // 2
    tong_trai = tinh_tong_hong(mang, trai, giua)
    tong_phai = tinh_tong_hong(mang, giua + 1, phai)
    return tong_trai                 # <- chỉ trả nửa trái, quên mất nửa phải
```

Gọi `tinh_tong_hong([1, 2, 3, 4], 0, 3)`. **Trước khi chạy**, bạn đoán
kết quả là bao nhiêu, so với tổng thật (10)?

:::opt{correct}
Nhỏ hơn 10 — bước Ghép chỉ trả `tong_trai`, nên toàn bộ đóng góp của nửa
phải bị bỏ qua ở MỌI tầng đệ quy, không chỉ tầng ngoài cùng
:::

:::opt
Đúng bằng 10 — vì hai nửa vẫn được TÍNH đầy đủ, kết quả không thể sai
::why
Gần đúng ở việc bạn để ý đúng: `tong_phai` VẪN được tính — dòng gọi đệ
quy cho nửa phải vẫn chạy, không bị bỏ qua.

Chỗ lệch: TÍNH ra một kết quả không có nghĩa gì nếu bước Ghép không DÙNG
tới nó. `tong_phai` được tính xong rồi... bị vứt đi, vì dòng `return`
cuối cùng không hề nhắc tới nó. Với chia để trị, một nửa tính đúng không
cứu được kết quả nếu bước Ghép không thật sự kết hợp cả hai.
::
:::

:::opt
Máy báo lỗi, vì hàm có biến `tong_phai` không được dùng tới là không hợp
lệ trong Python
::why
Gần đúng ở việc bạn để ý một điều bất thường thật: `tong_phai` đúng là
được TẠO RA mà không hề dùng tới — cảm giác "có gì sai" ở đây là đúng.

Chỗ lệch: Python không cấm việc khai một biến rồi không dùng nó. Đó là
một dấu hiệu ĐÁNG NGỜ về mặt logic, nhưng hoàn toàn hợp lệ về cú pháp —
hàm chạy trọn vẹn, không có `Error` nào, chỉ là kết quả sai.
::
:::

:::opt
Lớn hơn 10 — vì đệ quy tính lặp lại một phần dữ liệu nhiều lần, giống
tình huống bài 5 gặp với Fibonacci
::why
Gần đúng ở việc bạn nhớ đúng bài 5: đệ quy CÓ THỂ tính lặp một phần việc
nhiều lần, làm kết quả trung gian phình to hơn cần thiết — điều đó có
thật trong một số bài toán đệ quy khác.

Chỗ lệch: hàm này không hề tính lặp lại ô nào — mỗi ô chỉ được cộng vào
đúng MỘT nhánh (trái hoặc phải), không nhánh nào chồng lên nhánh kia
(khác Fibonacci, nơi `fib(2)` bị hai nhánh cùng hỏi lại). Vấn đề ở đây
không phải THỪA, mà là THIẾU: cả nửa phải bị bỏ hẳn ra khỏi kết quả
cuối, nên tổng luôn NHỎ HƠN 10, không bao giờ lớn hơn.
::
:::
::::

::::code{#viet-buoc-ghep-tinh-tong}
Viết bước Ghép còn thiếu cho hàm tính tổng bằng chia để trị. Bước Cắt và
Giải đã có sẵn trong khung — chỉ còn đúng một chỗ trống, phần GHÉP hai
kết quả của hai nửa lại thành kết quả của cả đoạn.

```python title=starter
def tinh_tong_chia_de_tri(mang, trai, phai):
    if trai == phai:
        return mang[trai]
    giua = (trai + phai) // 2
    tong_trai = tinh_tong_chia_de_tri(mang, trai, giua)
    tong_phai = tinh_tong_chia_de_tri(mang, giua + 1, phai)
    return ___                       # ghép hai kết quả nửa trái và nửa phải lại

diem = [67, 92, 45, 78, 23, 89, 56, 34]
tong = tinh_tong_chia_de_tri(diem, 0, len(diem) - 1)

print(f"Tổng chia để trị: {tong}")
print(f"Tổng bằng sum(): {sum(diem)}")
```

```python title=solution
def tinh_tong_chia_de_tri(mang, trai, phai):
    if trai == phai:
        return mang[trai]
    giua = (trai + phai) // 2
    tong_trai = tinh_tong_chia_de_tri(mang, trai, giua)
    tong_phai = tinh_tong_chia_de_tri(mang, giua + 1, phai)
    return tong_trai + tong_phai

diem = [67, 92, 45, 78, 23, 89, 56, 34]
tong = tinh_tong_chia_de_tri(diem, 0, len(diem) - 1)

print(f"Tổng chia để trị: {tong}")
print(f"Tổng bằng sum(): {sum(diem)}")
```

```python title=test
assert tong == 484, f"tổng của [67, 92, 45, 78, 23, 89, 56, 34] phải là 484 — đang ra {tong}"
assert tong == sum(diem), "kết quả chia để trị phải khớp đúng sum() có sẵn của Python — nếu quên cộng một nửa, con số này sẽ lệch"
```

:::hints
- kind: attention
  body: Chỗ trống là bước GHÉP — hai biến tong_trai và tong_phai đã được TÍNH sẵn ở hai dòng ngay phía trên. Việc còn lại chỉ là kết hợp CẢ HAI, không phải chọn một trong hai.
- kind: strategy
  body: 'Bài toán là TÍNH TỔNG, nên ghép hai tổng con lại đúng nghĩa là CỘNG chúng: tong_trai + tong_phai. Trả về đúng MỘT trong hai biến (như chỉ trả tong_trai) sẽ làm mất hẳn đóng góp của nửa còn lại.'
- kind: one-line
  body: 'Chỗ trống là `tong_trai + tong_phai`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải CỘNG cả hai biến tong_trai và tong_phai lại — không phải chỉ trả về một trong hai, hay một câu không liên quan như True, 1, 0; bước Ghép phải thật sự kết hợp kết quả của CẢ HAI nửa
  requireAst:
  - kind: uses-name, target: tong_trai, min: 1
  - kind: uses-name, target: tong_phai, min: 1
  - kind: uses-operator, target: "+"
  # tong_trai/tong_phai min:1 — đếm thật (kiemAst): mỗi biến chỉ bị GÁN
  # (Store, không đếm) ở hai dòng phía trên chỗ trống, nên trong khung
  # (không tính chỗ trống) số lần ĐỌC (Load) của mỗi biến là 0. Lời giải
  # đúng đọc mỗi biến đúng 1 lần trong chỗ trống, đạt ngưỡng.
  # ĐÃ THỬ THẬT: True/1/0 (không đọc biến nào) dừng ở 0 cho cả hai — chặn
  # bởi cả hai luật cùng lúc. "return tong_trai" (chỉ trả một nửa, đúng
  # ca dự đoán sai ở bước predict phía trên) đọc tong_trai 1 lần nhưng
  # tong_phai vẫn 0 lần — chặn đúng bởi luật tong_phai, đúng bài học "đếm
  # CẢ HAI tên tham gia phép kết hợp", không chỉ một phía.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Tổng chia để trị: 484\\nTổng bằng sum\\(\\): 484\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cắt tám ô làm đôi, rồi đôi nữa, tới khi mỗi ô đứng một mình — rồi cộng
ngược lại lên, tầng nào cũng đúng một phép GHÉP.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bước Ghép trong cả hai ví dụ hôm nay đều nhàn: một phép so sánh
(`max`), hoặc một phép cộng (`+`). Cả hai nửa đã ĐƯỢC SẮP SẴN theo đúng
trật tự cần dùng — không có gì phải chỉnh lại trước khi ghép.

Nếu hai nửa đó, thay vì là hai CON SỐ, lại là hai DÃY SỐ đã tự sắp xếp
riêng — mỗi dãy tăng dần trong chính nó — thì ghép chúng thành MỘT dãy
vẫn tăng dần có còn đơn giản như một phép cộng không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
