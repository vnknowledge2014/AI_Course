---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.chon-tim-tuyen-tinh-hay-nhi-phan
title: "Chọn tìm tuyến tính hay nhị phân"
summary: "Bài chốt cụm, không dạy khái niệm mới: cho vài tình huống thật, chọn đúng cách tìm và giải thích tại sao cách kia tệ hơn ở đây — hoặc không dùng được, vì thiếu đúng một điều kiện tiên quyết."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.choose-search]
requires: [alg.divide-and-conquer, alg.merge-step]
concepts: [alg.choose-search]
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
Sáu bài, không khái niệm nào mới hôm nay. Chỉ một câu hỏi: gặp dữ liệu
thật, bạn chọn đúng cách tìm không — và biết vì sao cách kia thua?
::::

::::explain{#dieu-kien-tien-quyet}
Không có khái niệm mới trong bài này. Việc của bạn là nhớ lại đúng MỘT
điều kiện, và áp nó vào tình huống thật:

> **Tìm nhị phân đòi dữ liệu ĐÃ SẮP XẾP SẴN. Tìm tuyến tính không đòi
> gì cả.**

Đó không phải một chi tiết nhỏ — nó là điều kiện TIÊN QUYẾT. Không có
nó, tìm nhị phân không chỉ chậm hơn — nó SAI. Luật "nhỏ hơn ô giữa thì
bỏ nửa phải" (bài 14) chỉ đúng khi dãy đã tăng dần; trên dữ liệu lộn
xộn, ô giữa chẳng nói lên điều gì về việc giá trị cần tìm nằm bên nào,
và việc "bỏ nửa" có thể bỏ nhầm đúng nửa chứa giá trị đang tìm.

Vậy câu hỏi khi chọn cách tìm không phải "cách nào nhanh hơn" — mà là:

1. **Dữ liệu đã sắp xếp sẵn chưa?** Chưa — chỉ còn một lựa chọn: tìm
   tuyến tính (bài 13). Không có đường tắt nào khác.
2. **Nếu đã sắp xếp — bạn tra cứu trên nó BAO NHIÊU LẦN?** Tra một lần
   duy nhất thì việc sắp xếp trước (chưa học ở track này, cụm bài sau sẽ
   dạy) tốn công hơn hẳn số bước tiết kiệm được. Tra đi tra lại NHIỀU
   lần trên cùng một dữ liệu ổn định — sắp xếp một lần, tìm nhị phân mãi
   mãi sau đó — mới thật sự đáng.

Bài 10-11 đã cho bạn công cụ để nói phần "nhanh hơn bao nhiêu" bằng
Big-O: tìm tuyến tính là **O(n)** — càng nhiều dữ liệu, ca xấu nhất càng
lâu, tỉ lệ thẳng. Track này chưa đặt tên Big-O riêng cho tìm nhị phân
(bài 14 đã nói: một hình dạng mỏng hơn hẳn, chưa có ký hiệu) — nhưng số
bước ĐO ĐƯỢC ở bài 14 (bốn bước cho mười ô, thay vì mười) đã đủ để
khẳng định: khi dữ liệu lớn dần, khoảng cách giữa hai cách càng ngày
càng xa.
::::

::::example{#hai-tinh-huong-that}
Hai tình huống, cùng hỏi "tìm một khách hàng" nhưng khác hẳn nhau:

**Nhật ký giao dịch ngân hàng hôm nay** — mỗi giao dịch được ghi thêm
vào CUỐI danh sách ngay khi nó xảy ra, theo đúng thứ tự thời gian. Danh
sách này không hề sắp xếp theo tên khách hay số tiền — nó chỉ sắp theo
"cái gì xảy ra trước". Muốn tìm giao dịch của một khách cụ thể: **tìm
tuyến tính** — không có lựa chọn nào khác, vì dữ liệu không hề có trật
tự nào để tận dụng.

**Danh bạ điện thoại đã in sẵn** — hàng triệu số, sắp xếp theo TÊN từ A
tới Z, và không đổi trong suốt một năm phát hành. Tra cứu diễn ra hàng
triệu lần, dữ liệu vẫn y nguyên: **tìm nhị phân** — dữ liệu đã đúng điều
kiện tiên quyết, và số lần tra cứu đủ lớn để công tận dụng trật tự đó
đáng giá gấp nhiều lần công tìm tuần tự.

Cùng là "tìm một cái tên", nhưng KHÔNG có câu trả lời chung cho mọi
tình huống — chỉ có câu trả lời đúng cho từng tình huống cụ thể.
::::

::::predict{#doan-tinh-huong-kho commitOnce}
Một mảng có đúng NĂM phần tử, hoàn toàn chưa sắp xếp:
`[42, 7, 19, 3, 88]`. Bạn cần tìm một giá trị trong đó, đúng MỘT lần.

**Trước khi đọc tiếp**, bạn đoán: cách tìm nào hợp lý hơn ở đây?

:::opt{correct}
Tìm tuyến tính — dữ liệu chưa sắp xếp, nên tìm nhị phân không dùng được
(sẽ cho kết quả SAI, không chỉ chậm); mảng lại rất ngắn nên chênh lệch
tốc độ giữa hai cách gần như không đáng kể
:::

:::opt
Tìm nhị phân — mảng chỉ có năm phần tử, quá ngắn để tuyến tính hay nhị
phân tạo ra khác biệt đáng kể, nên dùng cách "nhanh hơn về lý thuyết"
::why
Gần đúng ở nhận xét mảng ngắn thì chênh lệch tốc độ không đáng kể — quan
sát đó có lý, và đúng là với năm phần tử, hiệu năng không phải vấn đề.

Chỗ lệch nằm ở chỗ nặng hơn nhiều: tìm nhị phân đòi dữ liệu ĐÃ SẮP XẾP
làm điều kiện TIÊN QUYẾT, không phải một gợi ý về tốc độ. Trên mảng
`[42, 7, 19, 3, 88]` chưa sắp, tìm nhị phân không chỉ "chậm hơn dự
kiến" — nó có thể BÁO SAI, kết luận "không tìm thấy" một giá trị THẬT
SỰ có mặt, vì logic "bỏ nửa" của nó dựa hoàn toàn vào giả định dữ liệu
đã tăng dần.
::
:::

:::opt
Không cách nào dùng được — mảng quá ngắn để bất kỳ thuật toán tìm kiếm
nào hoạt động đúng
::why
Gần đúng ở việc bạn cẩn trọng với các trường hợp "nhỏ, đặc biệt" — sự
cẩn trọng đó có giá trị trong nhiều tình huống lập trình khác.

Chỗ lệch: không có ràng buộc nào về ĐỘ DÀI tối thiểu cho tìm tuyến tính
— nó chạy đúng trên một mảng chỉ có MỘT phần tử, hay thậm chí RỖNG (vòng
lặp không chạy lần nào, trả về "không tìm thấy" ngay). Độ dài ngắn không
phải lý do khiến một thuật toán tìm kiếm ngừng hoạt động.
::
:::

:::opt
Sắp xếp mảng trước, rồi dùng tìm nhị phân — luôn là lựa chọn AN TOÀN
NHẤT, vì nó tránh được rủi ro tìm sai của tìm nhị phân trên dữ liệu chưa
sắp
::why
Gần đúng ở việc bạn xác định đúng: dữ liệu PHẢI được sắp xếp trước khi
tìm nhị phân là an toàn. Về mặt kỹ thuật, cách này thật sự cho kết quả
ĐÚNG.

Chỗ lệch là ở chữ "luôn" — với một mảng năm phần tử, chỉ tìm ĐÚNG MỘT
lần, công sức sắp xếp (một việc chưa hề rẻ — cụm bài sau mới dạy) hoàn
toàn không đáng, so với việc cứ đọc tuần tự năm ô một cách trực tiếp.
Sắp xếp trước chỉ đáng làm khi việc tra cứu lặp lại đủ NHIỀU LẦN trên
CÙNG một dữ liệu, như tình huống danh bạ điện thoại vừa xem.
::
:::
::::

::::code{#kiem-de-xuat}
Một thực tập sinh đề xuất cách tìm cho năm tình huống. Bạn viết đoạn mã
so từng đề xuất với đáp án đúng, gom lại những tình huống bị đề xuất
SAI.

```python title=starter
tinh_huong = {
    "giao_dich_ngan_hang": "tuyến tính",
    "danh_ba_dien_thoai": "nhị phân",
    "diem_thi_moi_nhap": "tuyến tính",
    "tu_dien_tieng_viet": "nhị phân",
    "khach_vip_5_nguoi": "tuyến tính",
}

de_xuat = {
    "giao_dich_ngan_hang": "tuyến tính",
    "danh_ba_dien_thoai": "tuyến tính",
    "diem_thi_moi_nhap": "tuyến tính",
    "tu_dien_tieng_viet": "nhị phân",
    "khach_vip_5_nguoi": "nhị phân",
}

de_xuat_sai = []
for ten in tinh_huong:
    if de_xuat[ten] != ___:
        de_xuat_sai.append(___)

print(f"Số đề xuất sai: {len(de_xuat_sai)}")
print(f"Sai ở: {de_xuat_sai}")
```

```python title=solution
tinh_huong = {
    "giao_dich_ngan_hang": "tuyến tính",
    "danh_ba_dien_thoai": "nhị phân",
    "diem_thi_moi_nhap": "tuyến tính",
    "tu_dien_tieng_viet": "nhị phân",
    "khach_vip_5_nguoi": "tuyến tính",
}

de_xuat = {
    "giao_dich_ngan_hang": "tuyến tính",
    "danh_ba_dien_thoai": "tuyến tính",
    "diem_thi_moi_nhap": "tuyến tính",
    "tu_dien_tieng_viet": "nhị phân",
    "khach_vip_5_nguoi": "nhị phân",
}

de_xuat_sai = []
for ten in tinh_huong:
    if de_xuat[ten] != tinh_huong[ten]:
        de_xuat_sai.append(ten)

print(f"Số đề xuất sai: {len(de_xuat_sai)}")
print(f"Sai ở: {de_xuat_sai}")
```

```python title=test
assert de_xuat_sai == ["danh_ba_dien_thoai", "khach_vip_5_nguoi"], f"chỉ đúng hai đề xuất sai — danh bạ điện thoại (đã sắp sẵn, đề xuất nhầm tuyến tính) và khách VIP 5 người (chưa sắp, đề xuất nhầm nhị phân) — đang báo sai ở {de_xuat_sai}"
```

:::hints
- kind: attention
  body: Hai chỗ trống làm hai việc khác nhau — chỗ trong if phải SO SÁNH đề xuất với đáp án đúng của ĐÚNG tình huống đang xét, chỗ trong append phải GHI LẠI tên tình huống đó, không phải một giá trị khác.
- kind: strategy
  body: 'Đáp án đúng của tình huống đang xét nằm ở tinh_huong[ten] — so nó với đề xuất bằng !=. Khi khác nhau, cái cần ghi vào de_xuat_sai là chính TÊN tình huống — ten — để biết sai ở đâu, không phải giá trị đúng hay giá trị đề xuất.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `tinh_huong[ten]` và `ten`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống trong điều kiện if phải THẬT SỰ tra cứu đáp án đúng từ tinh_huong[ten] — không gõ thẳng một tên cố định; chỗ trống trong append phải ghi TÊN tình huống (ten), không phải một giá trị cố định như True, 1, 0
  requireAst:
  - kind: uses-name, target: tinh_huong, min: 2
  - kind: uses-name, target: ten, min: 3
  # tinh_huong min:2 — đếm thật: trong khung, tinh_huong chỉ xuất hiện MỘT
  # lần, ở "for ten in tinh_huong" (Load, vì đây là duyệt qua dict). Lời
  # giải đúng thêm đúng 1 lần nữa ở chỗ trống 1 ("tinh_huong[ten]"), ra
  # tổng 2.
  # ten min:3 — trong khung, "ten" xuất hiện Load một lần sẵn có, ở
  # "de_xuat[ten]" (điều kiện if, đã cho sẵn). Lời giải đúng thêm 1 lần ở
  # chỗ trống 1 ("tinh_huong[ten]") và 1 lần ở chỗ trống 2 ("ten" trong
  # append), ra tổng 3. (Bản thân "for ten in ..." là Store, không đếm.)
  #
  # ĐÃ THỬ THẬT: điền True/True (hoặc 1/0, hoặc 0/1) cho hai chỗ trống —
  # tinh_huong dừng ở 1 (dưới 2), ten dừng ở 1 (dưới 3) — chặn ở CẢ HAI
  # luật. Kết quả chạy: mọi tổ hợp cho "Số đề xuất sai: 5" (toàn bộ năm
  # tình huống bị coi là sai, vì so sánh với hằng số cố định luôn lệch),
  # khác hẳn "Số đề xuất sai: 2" của lời giải đúng — bị tests và output
  # bắt độc lập. Đây là vòng for với range cố định (duyệt đúng 5 khoá của
  # dict) — không có while/đệ quy, không có rủi ro treo.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Số đề xuất sai: 2\\nSai ở: \\['danh_ba_dien_thoai', 'khach_vip_5_nguoi'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai đề xuất sai, tìm đúng cả hai — không phải bằng cách nhớ luật, mà
bằng cách hỏi đúng câu hỏi: dữ liệu này đã sắp xếp chưa, và tra cứu bao
nhiêu lần.
::::

::::reflect{#nghi-lai}
Một câu hỏi khép lại cả cụm bài này.

Tìm nhị phân chỉ dùng được khi dữ liệu ĐÃ SẮP XẾP SẴN. Nhưng "giao dịch
ngân hàng hôm nay" ở ví dụ trên — dữ liệu CHƯA sắp xếp — không phải mãi
mãi chưa sắp xếp. Rất có thể, tới cuối ngày, có ai đó cần một BẢN ĐÃ SẮP
XẾP của đúng dữ liệu ấy, để ngày mai tra cứu bằng tìm nhị phân.

Ai — hay cái gì — biến một dãy lộn xộn thành một dãy đã sắp xếp?

Bài sau bắt đầu trả lời.
::::

::::checkpoint{mastery=0.8}
::::
