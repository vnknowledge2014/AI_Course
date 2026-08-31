---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.tim-trong-cay-nhi-phan
title: "Tìm trong cây nhị phân — mỗi bước bỏ đi một nửa"
summary: "Đúng luật bài trước: so với gốc, nhỏ hơn thì đi trái BỎ HẲN cả nhánh phải, lớn hơn thì ngược lại. Mỗi bước loại được một nửa số ứng viên còn lại — không cần đi qua từng nút như mảng chưa sắp xếp."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.bst-search]
requires: [ds.bst, ctrl.while, core.function-def, ctrl.comparison, core.fstring]
concepts: [ds.bst-search]
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
Luật BST tự nó không đi tìm ai cả. Hôm nay mình biến luật đó thành một
quy trình chạy được — không cần nhìn tận mắt từng nút.
::::

::::explain{#moi-buoc-bo-mot-nua}
Bài 27 dựng luật: tại mọi nút, trái nhỏ hơn, phải lớn hơn. Luật đó cho
bạn một thứ mạnh hơn cả một cây có tổ chức — nó cho bạn một CÁCH TÌM.

So một giá trị cần tìm với nút hiện tại. Có đúng ba khả năng:

- **Bằng nhau** — tìm thấy, dừng lại.
- **Nhỏ hơn** — giá trị cần tìm, NẾU có mặt trong cây, chắc chắn nằm
  trong nhánh TRÁI (đúng luật bài 27: mọi giá trị bên trái đều nhỏ hơn
  nút). Rẽ trái, và BỎ HẲN toàn bộ nhánh phải — không cần nhìn một nút
  nào trong đó nữa.
- **Lớn hơn** — ngược lại, rẽ phải, bỏ hẳn nhánh trái.

Viết bằng vòng lặp `while`, không đệ quy: giữ một cái tên trỏ tới "nút
đang đứng", mỗi vòng lặp so sánh rồi cập nhật cái tên đó sang `trai` hay
`phai`, cho tới khi tìm thấy hoặc rơi vào `None` (hết cây, không có).

Mỗi bước loại bỏ MỘT NHÁNH NGUYÊN VẸN — không phải một nút, mà cả một
cây con, dù cây con đó có bao nhiêu nút. Đây là điều mảng chưa sắp xếp
(cụm 1) không làm được: tìm trong một mảng chưa sắp xếp buộc phải nhìn
lần lượt từng ô, không có cách nào loại bỏ nguyên một VÙNG mà chưa nhìn
tới nó.
::::

::::example{#tim-so-bao-danh}
Byte dựng cây bảy số báo danh, rồi tìm số 65 bằng vòng lặp:

```python title=readonly
goc = {"gia_tri": 50, "trai": None, "phai": None}
goc["trai"] = {"gia_tri": 30, "trai": None, "phai": None}
goc["phai"] = {"gia_tri": 70, "trai": None, "phai": None}
goc["trai"]["trai"] = {"gia_tri": 20, "trai": None, "phai": None}
goc["trai"]["phai"] = {"gia_tri": 40, "trai": None, "phai": None}
goc["phai"]["trai"] = {"gia_tri": 65, "trai": None, "phai": None}
goc["phai"]["phai"] = {"gia_tri": 90, "trai": None, "phai": None}

can_tim = 65
hien_tai = goc
duong_di = []

while hien_tai is not None:
    duong_di.append(hien_tai["gia_tri"])
    if can_tim == hien_tai["gia_tri"]:
        break
    elif can_tim < hien_tai["gia_tri"]:
        hien_tai = hien_tai["trai"]
    else:
        hien_tai = hien_tai["phai"]

print(duong_di)
```

```text title=readonly
[50, 70, 65]
```

Cây có BẢY nút. Đường đi chỉ ghé đúng BA: 50, rồi 70 (vì 65 > 50, bỏ
hẳn nhánh trái của 50 — bỏ luôn 30, 20, 40, không nhìn nút nào trong
đó), rồi 65 (vì 65 < 70, bỏ nhánh phải của 70 — bỏ luôn 90). Hai lần rẽ
nhánh, hai lần bỏ nguyên một cây con, chỉ còn lại đúng nút cần tìm.
::::

::::predict{#dem-buoc-tim commitOnce}
Byte dựng một cây bảy nút khác — cùng hình dạng, số khác — rồi tìm số
200 bằng đúng vòng lặp vừa xem:

```python
goc = {"gia_tri": 500, "trai": None, "phai": None}
goc["trai"] = {"gia_tri": 300, "trai": None, "phai": None}
goc["phai"] = {"gia_tri": 700, "trai": None, "phai": None}
goc["trai"]["trai"] = {"gia_tri": 200, "trai": None, "phai": None}
goc["trai"]["phai"] = {"gia_tri": 400, "trai": None, "phai": None}
goc["phai"]["trai"] = {"gia_tri": 650, "trai": None, "phai": None}
goc["phai"]["phai"] = {"gia_tri": 900, "trai": None, "phai": None}

can_tim = 200
hien_tai = goc
duong_di = []
while hien_tai is not None:
    duong_di.append(hien_tai["gia_tri"])
    if can_tim == hien_tai["gia_tri"]:
        break
    elif can_tim < hien_tai["gia_tri"]:
        hien_tai = hien_tai["trai"]
    else:
        hien_tai = hien_tai["phai"]

print(len(duong_di))
```

**Trước khi chạy**, bạn đoán dòng in ra là bao nhiêu?

:::opt{correct}
3
:::

:::opt
7
::why
Gần đúng ở việc 7 là con số có thật trong bài — đúng tổng số nút của cả
cây, bạn đếm cây không sai.

Chỗ lệch là câu hỏi hỏi độ dài `duong_di` — đường đi thật sự đi qua,
không phải tổng số nút toàn cây. Đúng trọng tâm bài này: nhờ luật BST,
tìm kiếm KHÔNG cần ghé qua hết mọi nút. `duong_di` chỉ ghi lại ba nút:
500, rồi 300 (vì 200<500), rồi 200 (khớp, dừng) — bốn nút còn lại
(700, 400, 650, 900) chưa từng bị chạm tới.
::
:::

:::opt
1
::why
Gần đúng ở việc bạn tin tưởng đúng vào sức mạnh của BST — cảm giác "chỉ
cần rất ít bước" không sai tinh thần.

Chỗ lệch là 200 không nằm ngay ở gốc. Bước đầu tiên luôn phải GHÉ vào
gốc để so sánh trước — `duong_di` ghi cả nút gốc (500) dù cuối cùng
không khớp, rồi mới rẽ tiếp. Ít nhất một bước "ghé rồi rẽ" luôn xảy ra
trước khi chạm tới đích, trừ khi đích chính là gốc.
::
:::

:::opt
2
::why
Gần đúng ở việc bạn đếm đúng SỐ LẦN RẼ NHÁNH: cây rẽ đúng hai lần trước
khi khớp (500→300, rồi 300→200).

Chỗ lệch là `duong_di.append(...)` chạy trước MỌI lần kiểm tra, kể cả ở
nút cuối cùng khớp — nút 200 cũng được ghi vào `duong_di`, không chỉ hai
nút đã rẽ qua. Số lần RẼ và số PHẦN TỬ trong `duong_di` lệch nhau đúng
một, vì nút đích luôn được ghi thêm sau lần rẽ cuối.
::
:::
::::

::::code{#viet-vong-lap-tim}
Cây bảy số báo danh đã dựng sẵn (readonly). Bạn viết nốt vòng lặp bên
trong hàm `tim` — đúng luật vừa học: nhỏ hơn thì rẽ trái, lớn hơn thì rẽ
phải. Hàm này sẽ được gọi BA lần với ba số khác nhau, nên cả hai nhánh
rẽ đều phải đúng, không chỉ một nhánh.

```python title=starter
goc = {"gia_tri": 50, "trai": None, "phai": None}
goc["trai"] = {"gia_tri": 30, "trai": None, "phai": None}
goc["phai"] = {"gia_tri": 70, "trai": None, "phai": None}
goc["trai"]["trai"] = {"gia_tri": 20, "trai": None, "phai": None}
goc["trai"]["phai"] = {"gia_tri": 40, "trai": None, "phai": None}
goc["phai"]["trai"] = {"gia_tri": 65, "trai": None, "phai": None}
goc["phai"]["phai"] = {"gia_tri": 90, "trai": None, "phai": None}

def tim(can_tim):
    hien_tai = goc
    duong_di = []
    tim_thay = False
    while hien_tai is not None:
        duong_di.append(hien_tai["gia_tri"])
        if can_tim == hien_tai["gia_tri"]:
            tim_thay = True
            break
        elif can_tim < hien_tai["gia_tri"]:
            hien_tai = ___              # nhỏ hơn nút hiện tại -> rẽ nhánh nào?
        else:
            hien_tai = ___              # lớn hơn nút hiện tại -> rẽ nhánh nào?
    return tim_thay, duong_di

thay_65, duong_65 = tim(65)
thay_20, duong_20 = tim(20)
thay_99, duong_99 = tim(99)

print(f"Tìm 65: {thay_65}, đường đi {duong_65}")
print(f"Tìm 20: {thay_20}, đường đi {duong_20}")
print(f"Tìm 99: {thay_99}, đường đi {duong_99}")
```

```python title=solution
goc = {"gia_tri": 50, "trai": None, "phai": None}
goc["trai"] = {"gia_tri": 30, "trai": None, "phai": None}
goc["phai"] = {"gia_tri": 70, "trai": None, "phai": None}
goc["trai"]["trai"] = {"gia_tri": 20, "trai": None, "phai": None}
goc["trai"]["phai"] = {"gia_tri": 40, "trai": None, "phai": None}
goc["phai"]["trai"] = {"gia_tri": 65, "trai": None, "phai": None}
goc["phai"]["phai"] = {"gia_tri": 90, "trai": None, "phai": None}

def tim(can_tim):
    hien_tai = goc
    duong_di = []
    tim_thay = False
    while hien_tai is not None:
        duong_di.append(hien_tai["gia_tri"])
        if can_tim == hien_tai["gia_tri"]:
            tim_thay = True
            break
        elif can_tim < hien_tai["gia_tri"]:
            hien_tai = hien_tai["trai"]
        else:
            hien_tai = hien_tai["phai"]
    return tim_thay, duong_di

thay_65, duong_65 = tim(65)
thay_20, duong_20 = tim(20)
thay_99, duong_99 = tim(99)

print(f"Tìm 65: {thay_65}, đường đi {duong_65}")
print(f"Tìm 20: {thay_20}, đường đi {duong_20}")
print(f"Tìm 99: {thay_99}, đường đi {duong_99}")
```

```python title=test
assert (thay_65, duong_65) == (True, [50, 70, 65]), f"tìm 65 phải TÌM THẤY, đi qua đúng [50, 70, 65] — đang ra {(thay_65, duong_65)}"
assert (thay_20, duong_20) == (True, [50, 30, 20]), f"tìm 20 phải TÌM THẤY, đi qua đúng [50, 30, 20] — đang ra {(thay_20, duong_20)}"
assert (thay_99, duong_99) == (False, [50, 70, 90]), f"tìm 99 phải KHÔNG THẤY (99 lớn hơn mọi nút), đi qua đúng [50, 70, 90] rồi dừng vì hết cây — đang ra {(thay_99, duong_99)}"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm trong hai nhánh khác nhau của if/elif/else — nhánh "nhỏ hơn" và nhánh "lớn hơn" phải rẽ NGƯỢC hướng nhau, không phải cùng một hướng.
- kind: strategy
  body: 'Nhánh `elif can_tim < hien_tai["gia_tri"]:` nghĩa là giá trị cần tìm NHỎ HƠN nút hiện tại — đúng luật BST, nó chỉ có thể nằm ở nhánh trái, nên gán hien_tai = hien_tai["trai"]. Nhánh `else:` là trường hợp còn lại — lớn hơn — nên hien_tai = hien_tai["phai"].'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `hien_tai["trai"]` và `hien_tai["phai"]`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải cập nhật hien_tai bằng cách đi từ CHÍNH hien_tai hiện tại (hien_tai["trai"] hoặc hien_tai["phai"]) — không được nhảy thẳng qua một biến khác như goc; bài này đang dạy cách DI CHUYỂN từng bước một, không phải cách đoán trước đích đến
  requireAst:
  - kind: uses-name, target: hien_tai, min: 6
  # min: 6 — đếm thật trên solution: hien_tai bị ĐỌC sáu lần trong khung cố
  # định cộng hai chỗ trống — while hien_tai is not None (1),
  # duong_di.append(hien_tai["gia_tri"]) (2), if can_tim ==
  # hien_tai["gia_tri"] (3), elif can_tim < hien_tai["gia_tri"] (4), rồi mỗi
  # chỗ trống đọc hien_tai đúng một lần nữa để rẽ nhánh từ đó (5, 6). Bốn
  # lần đầu đã có sẵn trong khung, không phụ thuộc lời giải — bất kỳ cách
  # điền hợp lý nào cũng phải cộng thêm đúng hai lần đọc nữa. Một lời giải
  # hụt kiểu `hien_tai = goc["phai"]` (nhảy thẳng, không đi từ hien_tai hiện
  # tại) chỉ còn 5 lần — dưới ngưỡng 6 — VÀ trên cây bảy nút này còn tạo
  # vòng lặp không bao giờ chạm None (chạy lặp vô hạn, hết thời gian chấm),
  # nên bị bắt ở cả hai tầng.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Tìm 65: True, đường đi \\[50, 70, 65\\]\\nTìm 20: True, đường đi \\[50, 30, 20\\]\\nTìm 99: False, đường đi \\[50, 70, 90\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bảy nút trong cây, chưa lần tìm nào ghé quá ba nút. Luật bài trước vừa
biến thành một quy trình chạy tay không cần nhìn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài này giả định một điều ngầm mà bạn có thể chưa để ý: mỗi lần rẽ
nhánh, số nút CÒN LẠI để xét giảm đi một nửa — 7 nút, rồi khoảng 3, rồi
khoảng 1. Điều đó chỉ đúng khi cây CÂN ĐỐI, hai nhánh nhiều ít ngang
nhau.

Nếu ai đó xây một cây tìm kiếm nhị phân đúng luật hệt bài 27, nhưng
chèn dữ liệu theo một cách khiến một bên luôn phình to hơn bên kia rất
nhiều — cây đó còn "bỏ được một nửa" mỗi bước không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
