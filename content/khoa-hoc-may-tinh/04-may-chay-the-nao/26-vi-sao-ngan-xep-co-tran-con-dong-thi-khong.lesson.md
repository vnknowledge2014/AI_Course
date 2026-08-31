---
id: khoa-hoc-may-tinh.may-chay-the-nao.vi-sao-ngan-xep-co-tran-con-dong-thi-khong
title: "Vì sao ngăn xếp có TRẦN mà đống thì không"
summary: "Ngăn xếp gọi hàm được cấp một vùng CỐ ĐỊNH, cỡ nhỏ, khi hết chỗ là RecursionError. Đống lớn hơn nhiều và LỚN DẦN theo nhu cầu — chỉ dừng khi hết RAM thật, không phải một con số cấu hình sẵn."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.stack-limit-vs-heap]
requires: [may.value-location, alg.recursion-limit]
concepts: [may.stack-limit-vs-heap]
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
Một vùng có trần cứng, một vùng gần như không có. Lý do nằm ngay ở cách
mỗi vùng được cấp chỗ.
::::

::::explain{#vi-sao-mot-vung-co-tran}
T3.3 bài 3 đo được: `sys.getrecursionlimit()` cho `1000` trên máy của
khoá này. Vượt quá, `RecursionError`. Giờ nối câu hỏi đó với bài 22-25:
TẠI SAO ngăn xếp gọi hàm có trần, mà đống cấp phát (bài 24) thì hầu như
không?

Câu trả lời nằm ở CÁCH mỗi vùng được cấp chỗ, không phải một luật bí ẩn
nào của ngôn ngữ:

- **Ngăn xếp gọi hàm** được hệ điều hành cấp một khoảng RAM CỐ ĐỊNH,
  kích thước nhỏ, ngay lúc tiến trình khởi động (bài 22). Mỗi khung
  (bài 12) mở ra chiếm một phần của khoảng đó. Khi số khung đủ nhiều để
  lấp đầy hết khoảng cố định ấy — hết chỗ — chương trình PHẢI dừng, và
  Python báo bằng `RecursionError` thay vì để tiến trình đổ vỡ không rõ
  lý do. Trần tồn tại CHÍNH VÌ khoảng cấp cho nó không đổi.

- **Đống cấp phát** không được cấp một khoảng cố định như vậy. Khi
  chương trình cần thêm chỗ cho một `list` mới, một `dict` mới, và
  vùng đang có không đủ, Python (qua bộ quản lý bộ nhớ của nó) XIN
  THÊM bộ nhớ từ hệ điều hành — vùng đống LỚN DẦN theo nhu cầu thật.
  Nó chỉ dừng lại khi cả MÁY hết RAM thật sự — một giới hạn của phần
  cứng, không phải một con số cấu hình sẵn như
  `sys.getrecursionlimit()`.

Nói cách khác: trần của ngăn xếp là một **van an toàn CHỦ ĐỘNG**, đặt
sẵn cỡ nhỏ để chương trình dừng SỚM bằng một lỗi rõ ràng — vì một vùng
RAM cố định, cỡ nhỏ, là lựa chọn có chủ đích cho vùng cần TRUY CẬP CỰC
NHANH mỗi khi gọi/trả hàm. Đống không cần tốc độ đó tới mức ấy, nên nó
được phép lớn dần, đổi lấy khả năng chứa được nhiều hơn rất nhiều.
::::

::::example{#do-that-hai-tran}
Đo cả hai, trên cùng một đoạn mã:

```python title=readonly
import sys

def dem_sau(n, dich):
    if n == dich:
        return n
    return dem_sau(n + 1, dich)

gioi_han = sys.getrecursionlimit()
print(gioi_han)

try:
    dem_sau(0, gioi_han * 2)
except RecursionError:
    print("Ngăn xếp gọi hàm đã vỡ trần.")

dong_thu = []
for i in range(500000):
    dong_thu.append(i)
print(len(dong_thu))
```

```text title=readonly
1000
Ngăn xếp gọi hàm đã vỡ trần.
500000
```

`dem_sau` gọi lại chính nó, mở thêm một khung MỖI lần — chạm mức trần
`1000` từ rất sớm so với đích (`gioi_han * 2 = 2000`), và vỡ bằng
`RecursionError`, y hệt T3.3 bài 3 đã đo. Nhưng `dong_thu.append(i)`
chạy đúng NĂM TRĂM NGÀN lần, không một lỗi nào — đống cấp phát không hề
than phiền, vì nó không có một con số trần cấu hình sẵn như ngăn xếp.
::::

::::predict{#doan-vi-sao-khac-tran commitOnce}
Byte viết hai đoạn mã: một đệ quy không nhánh dừng, một vòng `for` thêm
liên tục vào một `list`.

**Trước khi đọc đáp án**, phát biểu nào dưới đây đúng về LÝ DO hai đoạn
mã này cư xử khác nhau?

:::opt{correct}
Ngăn xếp gọi hàm được cấp một vùng RAM CỐ ĐỊNH ngay từ đầu; đống cấp
phát có thể XIN THÊM RAM khi cần, nên nó chỉ dừng khi máy thật sự hết
RAM
:::

:::opt
Cả hai đều lớn dần bằng nhau — chỉ khác `RecursionError` là một loại
lỗi ngôn ngữ, không liên quan gì tới bộ nhớ thật
::why
Gần đúng ở việc bạn nhận ra hai loại lỗi khác nhau — `RecursionError`
đúng là một loại lỗi CÓ TÊN riêng, khác `MemoryError`.

Chỗ lệch: `RecursionError` không phải một luật "ngôn ngữ tuỳ hứng" tách
rời khỏi bộ nhớ — nó là hệ quả TRỰC TIẾP của việc ngăn xếp gọi hàm được
cấp một khoảng RAM cố định. Hai vùng KHÔNG lớn dần bằng nhau: đống lớn
dần theo nhu cầu thật; ngăn xếp thì không, nó dừng lại đúng ở mức trần
đã định trước.
::
:::

:::opt
Đệ quy có trần vì Python cố ý "phạt" đệ quy để khuyến khích dùng vòng
lặp thay thế
::why
Gần đúng ở việc bạn nhớ đúng: T3.3 từng so sánh đệ quy với vòng lặp —
có bài học thật ở đó.

Chỗ lệch: `sys.getrecursionlimit()` không phải một hình phạt nhắm riêng
vào đệ quy. Nó là trần của CẢ VÙNG ngăn xếp gọi hàm — áp dụng cho MỌI
lời gọi hàm đang dở, dù là đệ quy hay một chuỗi hàm A gọi B gọi C không
đệ quy chút nào. Vấn đề là VÙNG BỘ NHỚ, không phải "đệ quy bị ghét".
::
:::

:::opt
Đống cấp phát cũng có trần cố định giống ngăn xếp, chỉ là một con số
lớn hơn nhiều, nên khó chạm tới hơn
::why
Gần đúng ở việc bạn nhận ra có sự KHÁC BIỆT về QUY MÔ — đống đúng là
chứa được nhiều hơn hẳn ngăn xếp.

Chỗ lệch: đó không chỉ là "một con số lớn hơn". Đống không có một con
số trần CẤU HÌNH SẴN nào để đọc bằng một hàm như
`sys.getrecursionlimit()` — nó lớn dần bằng cách XIN THÊM bộ nhớ từ hệ
điều hành mỗi khi cần, và giới hạn thật của nó là RAM VẬT LÝ của cả
máy, một thứ khác hẳn về BẢN CHẤT so với một khoảng cấp sẵn cố định.
::
:::
::::

::::code{#do-hai-tran-that}
Đo cả hai mức trần trên chính máy đang chạy: cố ý vượt trần ngăn xếp
gọi hàm (bắt lỗi lại), rồi thêm rất nhiều phần tử vào đống — không lỗi
nào cả.

```python title=starter
import sys

def dem_them(n, dich):
    if n == dich:
        return n
    return dem_them(n + 1, dich)

gioi_han = sys.getrecursionlimit()

da_vo_ngan_xep = False
try:
    dem_them(0, gioi_han * 5)
except RecursionError:
    da_vo_ngan_xep = ___          # ghi nhận: ngăn xếp ĐÃ vỡ trần

dong_lon = []
for i in range(100000):
    ___                            # dùng .append(i) để thêm i vào
                                    # dong_lon — đống không có trần
                                    # cứng như vậy

print(f"Ngăn xếp vỡ ở {gioi_han * 5} lời gọi: {da_vo_ngan_xep}")
print(f"Đống chứa được: {len(dong_lon)} phần tử, không lỗi")
```

```python title=solution
import sys

def dem_them(n, dich):
    if n == dich:
        return n
    return dem_them(n + 1, dich)

gioi_han = sys.getrecursionlimit()

da_vo_ngan_xep = False
try:
    dem_them(0, gioi_han * 5)
except RecursionError:
    da_vo_ngan_xep = True

dong_lon = []
for i in range(100000):
    dong_lon.append(i)

print(f"Ngăn xếp vỡ ở {gioi_han * 5} lời gọi: {da_vo_ngan_xep}")
print(f"Đống chứa được: {len(dong_lon)} phần tử, không lỗi")
```

```python title=test
assert da_vo_ngan_xep is True, "gọi dem_them với đích gấp năm lần mức trần phải vỡ bằng RecursionError — da_vo_ngan_xep phải thành True"
assert len(dong_lon) == 100000, f"dong_lon phải chứa đủ 100000 phần tử, không lỗi nào — đang ra {len(dong_lon)}"
assert dong_lon[0] == 0 and dong_lon[-1] == 99999, "dong_lon phải chứa đúng các số từ 0 tới 99999, theo đúng thứ tự thêm vào"
```

:::hints
- kind: attention
  body: Hai chỗ trống làm hai việc khác hẳn nhau. Chỗ trống 1 nằm TRONG except — chỉ ghi nhận một sự kiện đã xảy ra. Chỗ trống 2 nằm TRONG thân vòng for — phải dùng đúng .append(i) để thêm i vào dong_lon ở MỌI lượt lặp, bài này đang đo bằng công cụ đó.
- kind: strategy
  body: 'Chỗ trống 1: gán da_vo_ngan_xep = True, đúng lối T3.3 bài "Đệ quy đụng trần" đã dạy. Chỗ trống 2: gọi dong_lon.append(i) — đúng công cụ ngăn xếp/list quen thuộc, chỉ khác lần này minh hoạ đống không có trần cứng.'
- kind: one-line
  body: 'Chỗ trống 1 là: True. Chỗ trống 2 là: dong_lon.append(i)'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: chỗ trống 1 phải THẬT SỰ gán da_vo_ngan_xep = True bên trong except; chỗ trống 2 phải THẬT SỰ gọi dong_lon.append(i) ở mỗi lượt lặp — không phải một câu không làm gì như True, 1, 0
  requireAst:
  # gan-ten target da_vo_ngan_xep min: 2 — đếm thật trên solution: gán đúng
  # 2 lần (da_vo_ngan_xep = False đã có sẵn trong khung + chỗ trống 1).
  # uses-call target append min: 1 — đếm thật trên solution: .append() xuất
  # hiện đúng 1 lần (đúng ở chỗ trống 2, không có append nào khác trong
  # bài). ĐÃ THỬ THẬT: điền True/1/0 vào CẢ HAI chỗ trống CÙNG LÚC (đúng
  # cách kiem_ma_bai_hoc.mjs thử) — không vòng lặp nào chạy vô hạn (điều
  # khiển for i in range(100000) và việc dem_them() vỡ hay không đều KHÔNG
  # phụ thuộc vào nội dung hai chỗ trống, chỉ phụ thuộc gioi_han và range đã
  # cố định trong khung) — cả ba cách (True/1/0) đều dừng an toàn, nhanh, và
  # cho len(dong_lon) == 0 (khác 100000) — assert bắt được độc lập với luật
  # static này.
  #
  # CỔNG NÀY CỐ Ý HẸP: `dong_lon += [i]` cũng cho đúng kết quả (100000 phần
  # tử, thứ tự đúng, qua run/tests/output) nhưng KHÔNG gọi .append() — đếm
  # thật bằng kiemAst() xác nhận nó bị luật này chặn (thiếu uses-call target
  # append). Vì lời giải khác đó tồn tại thật, đề bài đã được SỬA để nêu
  # đích danh công cụ — chỗ trống trong khung ghi rõ "dùng .append(i)" thay
  # vì chỉ nói chung chung "thêm i vào dong_lon" — khớp luật 3 (cổng hẹp thì
  # đề phải nêu đích danh).
  - kind: gan-ten, target: da_vo_ngan_xep, min: 2
  - kind: uses-call, target: append, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^Ngăn xếp vỡ ở 5000 lời gọi: True\\nĐống chứa được: 100000 phần tử, không lỗi\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ngăn xếp vỡ ở 5000, đúng như dự đoán. Đống chứa trọn một trăm ngàn phần
tử, không một tiếng kêu ca. Hai vùng, hai cách cấp chỗ, hai số phận
khác hẳn nhau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Suốt sáu bài vừa qua, mỗi bài đo MỘT khía cạnh riêng: bao nhiêu LỆNH
(`dis.dis()`, cụm 1), bao nhiêu KHUNG mở ra (cụm 3), giá trị nằm ở
ngăn xếp hay đống, vùng nào có trần.

Nếu ghép TẤT CẢ những công cụ đo đó lại — số lệnh, số bước tự đếm, và
cả thời gian THẬT bằng đồng hồ — trên CÙNG một đoạn mã, ba con số đó
có luôn kể cùng một câu chuyện không, hay mỗi con số nói một điều khác?

Bài sau đo thật, ghép cả ba lại.
::::

::::checkpoint{mastery=0.8}
::::
