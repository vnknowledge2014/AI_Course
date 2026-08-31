---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.tim-nhi-phan
title: "Tìm nhị phân: cần dữ liệu đã sắp xếp trước"
summary: "Đúng việc T3.2 làm trên cây nhị phân, giờ làm trên một MẢNG đã sắp: so với ô giữa, nhỏ hơn thì bỏ nửa phải, lớn hơn thì bỏ nửa trái. Đếm bước lộ ra một hình dạng không tuyến tính, không bậc hai — mỏng hơn hẳn cả hai."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [alg.binary-search]
requires: [alg.linear-search, alg.big-o-notation]
concepts: [alg.binary-search]
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
Trên một cây, luật "trái nhỏ hơn, phải lớn hơn" cho phép bỏ hẳn một nhánh
mỗi bước. Một mảng không có nhánh — nhưng nó có ô GIỮA.
::::

::::explain{#so-voi-o-giua}
Bài trước dừng lại ở một câu hỏi: nếu dãy đã SẮP XẾP TĂNG DẦN từ trước,
thứ trật tự đó có tận dụng được để bớt bước không?

Có. Và cách làm gần như là chép lại đúng luật T3.2 đã dùng trên cây tìm
kiếm nhị phân, chỉ đổi "nút" thành "ô ở giữa":

- So giá trị cần tìm với ô Ở CHÍNH GIỮA đoạn đang xét.
- **Bằng nhau** — tìm thấy, dừng lại.
- **Nhỏ hơn ô giữa** — vì dãy đã TĂNG DẦN, nếu giá trị cần tìm có mặt, nó
  chắc chắn nằm ở NỬA TRÁI (mọi ô bên phải ô giữa đều lớn hơn ô giữa,
  nên càng lớn hơn giá trị đang tìm). Bỏ hẳn nửa phải, không cần đọc một
  ô nào trong đó.
- **Lớn hơn ô giữa** — ngược lại, bỏ hẳn nửa trái.

Thu hẹp đoạn đang xét lại còn một nửa, rồi lặp lại đúng việc đó trên nửa
còn lại. Cách làm này có tên: **tìm nhị phân** (binary search) — "nhị
phân" vì mỗi bước chia đoạn đang xét làm ĐÔI.

Khác với cây (nơi "nửa trái" và "nửa phải" là hai nhánh có thật, hai
cụm ô nằm rải rác), một mảng chỉ là một dãy ô liền kề đánh số. "Nửa" ở
đây không phải một cấu trúc riêng — nó chỉ là một ĐOẠN chỉ số, giữ bằng
hai cái tên `trai` và `phai`. Thu hẹp một nửa nghĩa là dịch một trong
hai cái tên đó lại gần cái kia, không đụng gì tới bản thân mảng.

Đây là điều tìm tuyến tính không có: nó bắt buộc đọc TỪNG ô một, không
có cách nào loại bỏ nguyên một VÙNG mà chưa từng đọc ô nào trong đó. Tìm
nhị phân làm được đúng điều mảng chưa sắp xếp không cho phép — và cái
giá của nó là đúng điều kiện đã nêu: dữ liệu phải SẮP XẾP SẴN, không có
cách nào bỏ qua bước đó.
::::

::::example{#tim-so-bao-danh-sap}
Byte có mười số báo danh, đã sắp xếp tăng dần từ trước. Tìm ba giá trị
khác nhau, đếm số bước:

```python title=readonly
def tim_nhi_phan(danh_sach, can_tim):
    so_buoc = 0
    trai = 0
    phai = len(danh_sach) - 1
    while trai <= phai:
        so_buoc += 1
        giua = (trai + phai) // 2
        if danh_sach[giua] == can_tim:
            return giua, so_buoc
        elif danh_sach[giua] < can_tim:
            trai = giua + 1
        else:
            phai = giua - 1
    return -1, so_buoc

so_bao_danh = [12, 27, 34, 45, 58, 66, 71, 89, 93, 100]

print(tim_nhi_phan(so_bao_danh, 66))
print(tim_nhi_phan(so_bao_danh, 100))
print(tim_nhi_phan(so_bao_danh, 50))
```

```text title=readonly
(5, 3)
(9, 4)
(-1, 4)
```

Mười ô trong dãy. `66` — đúng ba bước. `100` là phần tử CUỐI CÙNG — thứ
mà tìm tuyến tính (bài trước) phải mất đủ MƯỜI bước để chạm tới — tìm
nhị phân chỉ mất bốn. `50` không hề có mặt — cũng chỉ bốn bước để biết
chắc, không phải mười.

So với tìm tuyến tính trên đúng độ dài này (ca xấu nhất: 10 bước), tìm
nhị phân không hề tuyến tính: gấp đôi số ô không làm số bước tăng gấp
đôi theo. Nó cũng không bậc hai — càng không thể, vì số bước còn ÍT HƠN
cả tuyến tính rất nhiều. Đây là một hình dạng tăng mỏng hơn hẳn hai hình
dạng bài 9 đã đặt tên — track này chưa có ký hiệu Big-O riêng cho nó,
nhưng con số đo được đã đủ để thấy nó khác hẳn, không cần một cái tên
mới để tin vào phép đo.
::::

::::predict{#doan-buoc-nhi-phan commitOnce}
Vẫn mười số báo danh đã sắp `[12, 27, 34, 45, 58, 66, 71, 89, 93, 100]`.
Byte tìm `93` bằng đúng hàm `tim_nhi_phan` ở trên.

**Trước khi chạy**, bạn đoán số bước là bao nhiêu?

:::opt{correct}
3 — ô giữa đầu tiên là 58 (93 lớn hơn, bỏ nửa trái), ô giữa tiếp theo
trong nửa còn lại là 89 (93 vẫn lớn hơn, bỏ tiếp nửa trái của phần đó),
ô giữa cuối cùng chính là 93 — khớp
:::

:::opt
10 — vì đây vẫn là một MẢNG, mà tìm trong mảng luôn phải xét hết mọi ô
::why
Gần đúng ở việc bạn nhớ đúng bài `alg.linear-search`: TÌM TUYẾN TÍNH trên
đúng mảng này ở ca xấu nhất thật sự mất tới 10 bước.

Chỗ lệch: hàm đang chạy ở đây không phải tìm tuyến tính. Nó là tìm nhị
phân — mỗi bước BỎ HẲN một nửa đoạn đang xét, không đọc ô nào trong đó.
Trên mười ô, tìm nhị phân không bao giờ cần tới 10 bước.
::
:::

:::opt
1 — vì mảng đã sắp xếp sẵn, máy tính thẳng ra vị trí của 93 mà không cần
so sánh gì cả
::why
Gần đúng ở việc bạn tin đúng vào sức mạnh của dữ liệu đã sắp xếp — niềm
tin đó không sai tinh thần.

Chỗ lệch: "đã sắp xếp" chỉ cho phép BỎ MỘT NỬA mỗi lần so sánh, không
cho phép NHẢY THẲNG tới đúng ô mà không so sánh gì. Máy vẫn phải ghé ô
giữa, so sánh, rồi mới quyết định bỏ nửa nào — và với đoạn còn lại sau
mỗi lần bỏ, nó lại phải ghé ô giữa MỚI, so sánh tiếp. Ba lần ghé-so-sánh
là ít nhất cần có để tới đúng 93 trong dãy này.
::
:::

:::opt
5 — vì mười ô chia đôi được năm lần liên tiếp trước khi chỉ còn một ô
::why
Gần đúng ở việc bạn nghĩ đúng hướng chia đôi — mỗi bước tìm nhị phân
đúng là thu hẹp đoạn đang xét xuống còn một nửa.

Chỗ lệch: bài toán không cần chia tới khi CHỈ CÒN MỘT Ô mới dừng. Nó
dừng ngay khi ô giữa KHỚP với giá trị cần tìm — mà với `93`, điều đó xảy
ra ngay ở bước thứ ba, không cần chia hết cả năm lần.
::
:::
::::

::::code{#viet-vong-lap-nhi-phan}
Mười số báo danh đã sắp sẵn (readonly). Bạn viết nốt hai điều kiện so
sánh trong vòng lặp — đúng luật vừa học: bằng nhau thì dừng, nhỏ hơn ô
giữa thì bỏ nửa phải, lớn hơn thì bỏ nửa trái. Hàm này sẽ được gọi với
BA giá trị khác nhau, nên cả hai chỗ trống đều phải đúng.

```python title=starter
def tim_nhi_phan(danh_sach, can_tim):
    so_buoc = 0
    trai = 0
    phai = len(danh_sach) - 1
    while trai <= phai:
        so_buoc += 1
        giua = (trai + phai) // 2
        if ___:                        # ô giữa có đúng bằng giá trị cần tìm không?
            return giua, so_buoc
        elif ___:                      # ô giữa NHỎ HƠN giá trị cần tìm -> bỏ nửa nào?
            trai = giua + 1
        else:
            phai = giua - 1
    return -1, so_buoc

so_bao_danh = [12, 27, 34, 45, 58, 66, 71, 89, 93, 100]

vi_tri_66, buoc_66 = tim_nhi_phan(so_bao_danh, 66)
vi_tri_100, buoc_100 = tim_nhi_phan(so_bao_danh, 100)
vi_tri_50, buoc_50 = tim_nhi_phan(so_bao_danh, 50)

print(f"Tìm 66: vị trí {vi_tri_66}, mất {buoc_66} bước")
print(f"Tìm 100: vị trí {vi_tri_100}, mất {buoc_100} bước")
print(f"Tìm 50: vị trí {vi_tri_50}, mất {buoc_50} bước")
```

```python title=solution
def tim_nhi_phan(danh_sach, can_tim):
    so_buoc = 0
    trai = 0
    phai = len(danh_sach) - 1
    while trai <= phai:
        so_buoc += 1
        giua = (trai + phai) // 2
        if danh_sach[giua] == can_tim:
            return giua, so_buoc
        elif danh_sach[giua] < can_tim:
            trai = giua + 1
        else:
            phai = giua - 1
    return -1, so_buoc

so_bao_danh = [12, 27, 34, 45, 58, 66, 71, 89, 93, 100]

vi_tri_66, buoc_66 = tim_nhi_phan(so_bao_danh, 66)
vi_tri_100, buoc_100 = tim_nhi_phan(so_bao_danh, 100)
vi_tri_50, buoc_50 = tim_nhi_phan(so_bao_danh, 50)

print(f"Tìm 66: vị trí {vi_tri_66}, mất {buoc_66} bước")
print(f"Tìm 100: vị trí {vi_tri_100}, mất {buoc_100} bước")
print(f"Tìm 50: vị trí {vi_tri_50}, mất {buoc_50} bước")
```

```python title=test
assert (vi_tri_66, buoc_66) == (5, 3), f"tìm 66 phải ra vị trí 5, mất 3 bước — đang ra {(vi_tri_66, buoc_66)}"
assert (vi_tri_100, buoc_100) == (9, 4), f"100 là phần tử CUỐI CÙNG — tìm tuyến tính sẽ mất 10 bước cho ca này, tìm nhị phân chỉ mất 4, đang ra {(vi_tri_100, buoc_100)}"
assert (vi_tri_50, buoc_50) == (-1, 4), f"50 không có mặt trong dãy — phải trả về -1 sau đúng 4 bước, đang ra {(vi_tri_50, buoc_50)}"
assert buoc_100 < len(so_bao_danh), "đúng trọng tâm bài này: tìm nhị phân trên ca xấu của tìm tuyến tính vẫn phải mất ÍT bước hơn hẳn độ dài dãy"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều là ĐIỀU KIỆN so sánh, không phải lệnh gán. Đừng chạm vào giua = (trai + phai) // 2 hay hai dòng trai = giua + 1 / phai = giua - 1 — chúng đã đúng sẵn trong khung, chỉ hai câu hỏi so sánh còn thiếu.
- kind: strategy
  body: 'Chỗ trống 1 hỏi "ô giữa có KHỚP không" — so sánh danh_sach[giua] với can_tim bằng ==. Chỗ trống 2 nằm trong nhánh elif đã được đặt tên "ô giữa NHỎ HƠN giá trị cần tìm" — so sánh đúng bằng <, để nhánh này rẽ đúng sang phía trai = giua + 1 (bỏ nửa trái, kể cả ô giữa).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `danh_sach[giua] == can_tim` và `danh_sach[giua] < can_tim`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải THẬT SỰ so sánh danh_sach[giua] với can_tim — không phải một câu luôn đúng/luôn sai như True, 1, 0; bài này đòi bạn ĐỌC ô giữa và SO nó với giá trị cần tìm, không phải đoán mù
  requireAst:
  - kind: uses-name, target: can_tim, min: 2
  - kind: uses-name, target: giua, min: 5
  - kind: uses-operator, target: "=="
  # can_tim min:2 — trong khung (không tính hai chỗ trống), can_tim chỉ
  # xuất hiện làm THAM SỐ hàm (ast.arg, không phải Name-Load, không bị
  # đếm) — đếm thật là 0. Lời giải đúng chạm can_tim đúng MỘT lần ở mỗi
  # chỗ trống, ra tổng 2. ĐÃ THỬ chín tổ hợp True/1/0 cho cả hai chỗ trống:
  # can_tim đếm được 0 trong mọi tổ hợp không dùng nó — dưới ngưỡng 2.
  #
  # giua min:5 — đếm thật (kiemAst) trên lời giải đúng: giua bị ĐỌC đúng
  # năm lần — trong chỗ trống 1 (danh_sach[giua]), chỗ trống 2
  # (danh_sach[giua]), và ba lần nữa đã có sẵn trong khung (return giua,
  # so_buoc; trai = giua + 1; phai = giua - 1). Bốn tổ hợp điền hụt không
  # chạm giua ở CẢ HAI chỗ trống chỉ còn 3 — dưới ngưỡng 5; các tổ hợp
  # điền hụt MỘT chỗ trống, đúng chỗ còn lại, dừng ở 4 — vẫn dưới 5, vẫn bị
  # chặn (đúng bài học: đếm để chặn TỪNG chỗ trống, không chỉ tổng thể).
  #
  # ĐÃ THỬ THẬT (script ngoài, chín tổ hợp True/1/0 cho hai chỗ trống, đo
  # thời gian chạy): mọi tổ hợp chạy XONG trong vài mili-giây, KHÔNG một
  # tổ hợp nào lặp vô hạn — vì giua, trai, phai đều được TÍNH bằng đúng
  # công thức trong khung (không nằm trong chỗ trống); chỉ nhánh REDIRECT
  # (bỏ nửa nào) bị chọn sai, còn khoảng cách trai..phai vẫn hẹp dần chắc
  # chắn ở mọi tổ hợp. Kết quả sai khác hẳn (5,3)/(9,4)/(-1,4) — bị tests
  # và output bắt độc lập.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Tìm 66: vị trí 5, mất 3 bước\\nTìm 100: vị trí 9, mất 4 bước\\nTìm 50: vị trí -1, mất 4 bước\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mười ô, chưa lần tìm nào mất quá bốn bước. Cùng luật "trái nhỏ hơn, phải
lớn hơn" T3.2 dạy trên cây — chỉ khác lần này không có nhánh, chỉ có
một dãy ô và hai cái tên đánh dấu đoạn còn lại.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vòng lặp `while` vừa viết tự tay quản lý hai cái tên `trai` và `phai`,
thu hẹp chúng lại từng bước một — y hệt việc T3.2 bài 30 tự tay quản một
ngăn xếp bằng `list`, thay vì để một hàm gọi lại chính nó lo việc đó.

Bạn đã học một cách khác để làm đúng việc "nhớ đường về" — không tự quản
bằng tay, mà để CHÍNH LỜI GỌI HÀM giữ chỗ hộ mình. Tìm nhị phân viết theo
lối đó sẽ trông ra sao?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
