---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.tim-nhi-phan-bang-de-quy
title: "Tìm nhị phân viết bằng đệ quy"
summary: "Đúng bài học của bài 2, áp lên chính thuật toán vừa học ở bài 14: mỗi lời gọi đệ quy nhận đúng NỬA mảng còn lại, thay vì tự tay dịch trai/phai bằng vòng lặp. Ca dừng là khi nửa ấy rỗng, hoặc khi tìm thấy."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.binary-search-recursive]
requires: [alg.binary-search, alg.recursion-vs-loop]
concepts: [alg.binary-search-recursive]
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
Bài trước tự tay dịch `trai` và `phai` mỗi vòng lặp. Lần này, để mỗi
lời gọi hàm tự giữ đúng một cặp `trai`/`phai` của riêng nó.
::::

::::explain{#chia-doi-bang-loi-goi}
Bài 2 của track này đã làm đúng việc này một lần rồi: lấy một thuật toán
viết bằng `while` cộng một ngăn xếp tự quản, rồi viết lại CHÍNH VIỆC ĐÓ
bằng đệ quy — để lộ ra rằng đệ quy chỉ là cách để trình thông dịch tự
làm hộ việc "nhớ đường về", thay vì bạn tự tay đẩy/lấy trên một `list`.

Tìm nhị phân ở bài 14 cũng tự tay quản một trạng thái: cặp `trai`/`phai`
đánh dấu đoạn còn lại, dịch chuyển sau mỗi vòng lặp. Đúng bài học bài 2,
giờ đổi nó thành đệ quy: thay vì DỊCH `trai`/`phai` rồi lặp lại, mỗi lời
gọi hàm nhận một cặp `trai`/`phai` MỚI làm THAM SỐ, rồi gọi lại chính nó
với cặp thu hẹp hơn cho nửa còn lại.

Ca dừng lại (R1.T1.3 bài 18 đã dạy: một hàm đệ quy luôn cần một trường
hợp không gọi tiếp) ở đây rất tự nhiên: khi `trai` vượt qua `phai` —
nghĩa là đoạn đang xét đã RỖNG, không còn ô nào để so sánh — hàm trả về
ngay, không gọi thêm. Trường hợp còn lại (tìm thấy ở ô giữa) cũng là một
ca dừng, y hệt bài 14.

Khác với tìm nhị phân bằng vòng lặp, phiên bản đệ quy này không có biến
`trai`/`phai` nào "sống" bên ngoài và bị dịch chuyển qua từng vòng. Mỗi
lời gọi có bản sao THAM SỐ của riêng nó — đúng như mỗi tờ phiếu trên
chồng lời gọi (bài 1) mang một bộ giá trị cục bộ riêng, không ai ghi đè
lên ai.
::::

::::example{#hai-ban-song-song}
Cùng mười số báo danh bài trước, giờ viết bằng đệ quy. So sánh trực
tiếp với bản `while` ở bài 14:

```python title=readonly
def tim_nhi_phan_de_quy(danh_sach, can_tim, trai, phai, so_buoc):
    so_buoc += 1
    if trai > phai:                 # đoạn đang xét đã rỗng -> ca dừng, không tìm thấy
        return -1, so_buoc
    giua = (trai + phai) // 2
    if danh_sach[giua] == can_tim:
        return giua, so_buoc
    elif danh_sach[giua] < can_tim:
        return tim_nhi_phan_de_quy(danh_sach, can_tim, giua + 1, phai, so_buoc)
    else:
        return tim_nhi_phan_de_quy(danh_sach, can_tim, trai, giua - 1, so_buoc)

so_bao_danh = [12, 27, 34, 45, 58, 66, 71, 89, 93, 100]

print(tim_nhi_phan_de_quy(so_bao_danh, 66, 0, len(so_bao_danh) - 1, 0))
print(tim_nhi_phan_de_quy(so_bao_danh, 50, 0, len(so_bao_danh) - 1, 0))
```

```text title=readonly
(5, 3)
(-1, 5)
```

`66` ra đúng ba bước — CÙNG con số bản vòng lặp của bài 14 tìm ra. Cùng
một thuật toán, viết bằng hai lối khác nhau, cùng một số bước để CHẠM
tới ô giữa cần so sánh.

Ca `50` (không tìm thấy) lại ra **5**, không phải 4 như bản vòng lặp bài
14. Không phải một thuật toán chậm hơn thuật toán kia — khác biệt nằm ở
việc ĐẾM CÁI GÌ. Bản vòng lặp đếm số LẦN LẶP (mỗi lần chạm được một ô
giữa mới). Bản đệ quy đếm số LẦN GỌI HÀM — và có một lời gọi cuối cùng,
khi `trai > phai`, chạm ngay ca dừng mà không hề đọc thêm ô nào. Lời gọi
đó vẫn được tính, đúng cách bài 6 đã đếm: MỌI lần hàm được gọi, kể cả
lần chạm ngay trường hợp cơ sở.
::::

::::predict{#doan-neu-thieu-ca-dung commitOnce}
Một người viết lại hàm trên nhưng LÀM MẤT dòng `if trai > phai:` — bỏ
hẳn ca dừng cho đoạn rỗng, chỉ giữ lại nhánh "tìm thấy" và hai nhánh gọi
đệ quy:

```python
def tim_nhi_phan_hong(danh_sach, can_tim, trai, phai):
    giua = (trai + phai) // 2
    if danh_sach[giua] == can_tim:
        return giua
    elif danh_sach[giua] < can_tim:
        return tim_nhi_phan_hong(danh_sach, can_tim, giua + 1, phai)
    else:
        return tim_nhi_phan_hong(danh_sach, can_tim, trai, giua - 1)
```

Gọi `tim_nhi_phan_hong(so_bao_danh, 50, 0, 9)` — giá trị 50 không hề có
mặt trong dãy. **Trước khi chạy**, bạn đoán chuyện gì xảy ra?

:::opt{correct}
Chương trình dừng lại bằng lỗi `RecursionError` — thiếu ca dừng cho đoạn
rỗng, `trai` và `phai` cứ vượt qua nhau mãi mà không hàm nào chịu ngừng
gọi tiếp, tới khi chồng lời gọi chạm trần
:::

:::opt
In ra `-1`, giống hệt bản đầy đủ, chỉ là chậm hơn một chút vì thiếu một
điều kiện kiểm tra
::why
Gần đúng ở việc bạn tin hàm vẫn "biết" báo không tìm thấy — cảm giác đó
hợp lý nếu nhìn hàm như một hộp đen.

Chỗ lệch: không có dòng nào trong hàm hỏng này TỰ IN RA `-1` khi giá trị
không có mặt — `-1` chỉ xuất hiện ở phiên bản ĐẦY ĐỦ, đúng tại dòng
`if trai > phai:` vừa bị xoá. Thiếu dòng đó, hàm không có cách nào diễn
đạt "tôi đã tìm khắp rồi và không thấy" — nó chỉ biết gọi tiếp.
::
:::

:::opt
Chạy mãi mãi, không bao giờ dừng, vì đoạn code không có gì sai cú pháp
::why
Gần đúng ở phần khó nhất: không có `return` nào bắt được trường hợp
"không tìm thấy", và xét thuần theo logic của các dòng còn lại thì đúng
là không có điểm dừng nào được VIẾT RA.

Chỗ lệch nằm ở điều nằm NGOÀI code, đúng như bài 3 đã đo: chồng lời gọi
có một mức trần thật (trên Pyodide 32-bit của khoá này là 1000 tầng).
Chương trình không "chạy mãi mãi" theo nghĩa tuyệt đối — nó cứ gọi tiếp
cho tới khi chồng cao quá mức trần đó, rồi Python dừng nó lại bằng một
lỗi đàng hoàng, đúng cách bài 3 đã đo được.
::
:::

:::opt
Máy báo lỗi ngay lập tức, kiểu `SyntaxError`, vì thiếu một nhánh `if` là
không hợp lệ
::why
Gần đúng ở việc bạn cảm thấy thiếu một điều kiện là "không đủ" — cảm
giác cẩn trọng đó không sai tinh thần.

Chỗ lệch: Python không đòi hỏi một hàm phải xử lý MỌI trường hợp mới
được coi là hợp lệ về cú pháp. Hàm này hoàn toàn phân tích được, chạy
được — nó chỉ THIẾU một khả năng LOGIC (báo không tìm thấy), không thiếu
gì về mặt cú pháp. Lỗi chỉ xuất hiện lúc CHẠY, không phải lúc đọc code.
::
:::
::::

::::code{#viet-de-quy-nhi-phan}
Mười số báo danh đã sắp sẵn (readonly). Ca dừng cho đoạn rỗng đã có sẵn
trong khung — bạn viết nốt hai điều kiện so sánh, đúng luật bài 14.

```python title=starter
def tim_nhi_phan_de_quy(danh_sach, can_tim, trai, phai, so_buoc):
    so_buoc += 1
    if trai > phai:
        return -1, so_buoc
    giua = (trai + phai) // 2
    if ___:                        # ô giữa có đúng bằng giá trị cần tìm không?
        return giua, so_buoc
    elif ___:                      # ô giữa NHỎ HƠN giá trị cần tìm -> nửa nào còn lại?
        return tim_nhi_phan_de_quy(danh_sach, can_tim, giua + 1, phai, so_buoc)
    else:
        return tim_nhi_phan_de_quy(danh_sach, can_tim, trai, giua - 1, so_buoc)

so_bao_danh = [12, 27, 34, 45, 58, 66, 71, 89, 93, 100]

vi_tri_66, buoc_66 = tim_nhi_phan_de_quy(so_bao_danh, 66, 0, len(so_bao_danh) - 1, 0)
vi_tri_100, buoc_100 = tim_nhi_phan_de_quy(so_bao_danh, 100, 0, len(so_bao_danh) - 1, 0)
vi_tri_50, buoc_50 = tim_nhi_phan_de_quy(so_bao_danh, 50, 0, len(so_bao_danh) - 1, 0)

print(f"Tìm 66: vị trí {vi_tri_66}, mất {buoc_66} bước")
print(f"Tìm 100: vị trí {vi_tri_100}, mất {buoc_100} bước")
print(f"Tìm 50: vị trí {vi_tri_50}, mất {buoc_50} bước")
```

```python title=solution
def tim_nhi_phan_de_quy(danh_sach, can_tim, trai, phai, so_buoc):
    so_buoc += 1
    if trai > phai:
        return -1, so_buoc
    giua = (trai + phai) // 2
    if danh_sach[giua] == can_tim:
        return giua, so_buoc
    elif danh_sach[giua] < can_tim:
        return tim_nhi_phan_de_quy(danh_sach, can_tim, giua + 1, phai, so_buoc)
    else:
        return tim_nhi_phan_de_quy(danh_sach, can_tim, trai, giua - 1, so_buoc)

so_bao_danh = [12, 27, 34, 45, 58, 66, 71, 89, 93, 100]

vi_tri_66, buoc_66 = tim_nhi_phan_de_quy(so_bao_danh, 66, 0, len(so_bao_danh) - 1, 0)
vi_tri_100, buoc_100 = tim_nhi_phan_de_quy(so_bao_danh, 100, 0, len(so_bao_danh) - 1, 0)
vi_tri_50, buoc_50 = tim_nhi_phan_de_quy(so_bao_danh, 50, 0, len(so_bao_danh) - 1, 0)

print(f"Tìm 66: vị trí {vi_tri_66}, mất {buoc_66} bước")
print(f"Tìm 100: vị trí {vi_tri_100}, mất {buoc_100} bước")
print(f"Tìm 50: vị trí {vi_tri_50}, mất {buoc_50} bước")
```

```python title=test
assert (vi_tri_66, buoc_66) == (5, 3), f"tìm 66 phải ra vị trí 5, mất 3 lần gọi — đang ra {(vi_tri_66, buoc_66)}"
assert (vi_tri_100, buoc_100) == (9, 4), f"100 là phần tử cuối cùng — phải ra vị trí 9, mất 4 lần gọi, đang ra {(vi_tri_100, buoc_100)}"
assert (vi_tri_50, buoc_50) == (-1, 5), f"50 không có mặt — phải trả về -1 sau 5 lần gọi (kể cả lần chạm ca dừng), đang ra {(vi_tri_50, buoc_50)}"
assert buoc_66 == 3, "đúng bằng số lần gọi của bản vòng lặp ở bài 14 cho cùng giá trị 66 — hai lối viết, cùng số bước chạm ô giữa"
```

:::hints
- kind: attention
  body: Đừng chạm vào dòng if trai > phai — đó là ca dừng, đã đúng sẵn. Hai chỗ trống chỉ là điều kiện so sánh ô giữa với giá trị cần tìm, y hệt bài 14, chỉ khác cách bài này gọi lại chính nó thay vì lặp.
- kind: strategy
  body: 'Chỗ trống 1: danh_sach[giua] == can_tim — khớp thì dừng ngay. Chỗ trống 2: danh_sach[giua] < can_tim — đúng nhánh "nhỏ hơn ô giữa", khớp với lời gọi đệ quy dùng giua + 1 làm trai mới (bỏ nửa trái, kể cả ô giữa).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `danh_sach[giua] == can_tim` và `danh_sach[giua] < can_tim`.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: cả hai chỗ trống phải THẬT SỰ so sánh danh_sach[giua] với can_tim — không phải một câu luôn đúng/luôn sai như True, 1, 0; đúng luật bài 14, chỉ đổi từ vòng lặp sang lời gọi đệ quy
  requireAst:
  - kind: uses-name, target: can_tim, min: 2
  - kind: uses-name, target: giua, min: 5
  - kind: uses-operator, target: "=="
  # can_tim min:2, giua min:5 — đếm thật (kiemAst) trên lời giải đúng, y
  # hệt cách đo ở bài 14 (cùng hình dạng hai chỗ trống, chỉ thân hàm đổi
  # từ while sang lời gọi đệ quy — số Name-Load không đổi vì hai lời gọi
  # đệ quy tham chiếu giua đúng một lần mỗi lời gọi, giống hai dòng gán
  # trai/phai ở bản vòng lặp).
  #
  # ĐÃ THỬ THẬT (script ngoài, chín tổ hợp True/1/0 cho hai chỗ trống, đo
  # thời gian): mọi tổ hợp chạy XONG trong vài mili-giây, KHÔNG một tổ hợp
  # nào rơi vào đệ quy vô hạn hay chạm RecursionError — vì dòng ca dừng
  # (if trai > phai) và hai lời gọi đệ quy (với giua + 1 / giua - 1) đều
  # nằm NGOÀI chỗ trống, cố định trong khung; đoạn [trai, phai] luôn hẹp
  # dần đúng một nửa ở MỌI tổ hợp, bất kể nhánh nào bị chọn sai. Kết quả ra
  # sai khác hẳn (5,3)/(9,4)/(-1,5) — bị tests và output bắt độc lập.
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: "^Tìm 66: vị trí 5, mất 3 bước\\nTìm 100: vị trí 9, mất 4 bước\\nTìm 50: vị trí -1, mất 5 bước\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba lần gọi để chạm 66 — đúng con số bản vòng lặp cũng ra. Cùng một thuật
toán, chỉ khác ai giữ "đoạn còn lại": bạn tự tay dịch trai/phai, hay để
mỗi lời gọi hàm giữ một bản sao của riêng nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cả tìm nhị phân bằng vòng lặp (bài 14) và bằng đệ quy (bài này) đều làm
đúng MỘT việc theo cùng MỘT khuôn: cắt đoạn đang xét làm đôi, giải quyết
trên đúng một nửa, bỏ hẳn nửa kia. Khuôn đó không chỉ dùng được cho việc
TÌM — nó là một cách tiếp cận chung cho rất nhiều bài toán khác.

Khuôn chung đó có tên riêng của nó chưa?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
