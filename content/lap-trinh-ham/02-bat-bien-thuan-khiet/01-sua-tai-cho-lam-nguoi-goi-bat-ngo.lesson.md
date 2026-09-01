---
id: lap-trinh-ham.bat-bien-thuan-khiet.sua-tai-cho-lam-nguoi-goi-bat-ngo
title: "Sửa dữ liệu tại chỗ làm người gọi hàm bất ngờ"
summary: "Một hàm nhận list làm tham số rồi sửa nó TẠI CHỖ — người gọi hàm không ngờ list gốc của họ đã đổi, vì tham số và đối số CÙNG TRỎ một list, y hệt bài hai tấm thẻ một cái nồi."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.mutation-surprise]
requires: [mem.aliasing-explained, mem.mutability, core.function-def]
concepts: [fp.mutation-surprise]
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
Cổng Rust vừa để lại một câu hỏi: giá trị này, ai đang GIỮ, ai chỉ đang
MƯỢN? Python không có trình biên dịch nào trả lời hộ câu đó — và hôm
nay bạn thấy đúng điều xảy ra khi không ai trả lời nó.
::::

::::explain{#khi-khong-ai-tra-loi-cau-do}
Nhớ lại hai bài đã học: tấm thẻ không giữ cái nồi, nó giữ SỐ NHÀ của cái
nồi (`mem.aliasing-explained`) — và một số kiểu dữ liệu, như `list`, cho
phép SỬA TẠI CHỖ, nghĩa là sửa đúng cái nồi mà không cần dựng nồi mới
(`mem.mutability`). Gọi một hàm cũng chỉ là buộc thêm một tấm thẻ — tham
số của hàm trùng địa chỉ với đối số truyền vào.

Ghép ba điều đó lại, một nguy cơ hiện ra: nếu THÂN HÀM sửa tại chỗ trên
tham số của nó, thì cái nồi bị sửa chính là cái nồi mà NGƯỜI GỌI HÀM
đang giữ — dù thân hàm không hề biết (và không cần biết) người gọi đặt
tên gì cho nó ở ngoài.

Đây không phải một lỗi hiếm. Đây là hệ quả THẲNG của hai luật bạn đã
học, xảy ra bất cứ khi nào một hàm nhận `list` (hay bất kỳ thứ sửa được
tại chỗ nào) rồi gọi `.append`, gán vào một ô, hay bất kỳ phép sửa tại
chỗ nào khác lên đúng tham số đó. Người gọi hàm — người không đọc thân
hàm, chỉ đọc tên hàm và tài liệu của nó — hoàn toàn không có cách nào
đoán trước điều này sẽ xảy ra.
::::

::::example{#bang-gia-doi-mot-minh}
Một quán trà đá có bảng giá cố định. Byte viết một hàm tính giá sau khi
cộng thêm phí giao hàng — chỉ để XEM THỬ giá mới, không có ý định đổi
bảng giá đang treo trên tường.

```python title=readonly
def tang_them_phi_giao_hang(gia, phi):
    for i in range(len(gia)):
        gia[i] += phi
    return gia

bang_gia = [40000, 55000, 60000]
gia_giao_hang = tang_them_phi_giao_hang(bang_gia, 5000)

print(f"Giá giao hàng: {gia_giao_hang}")
print(f"Bảng giá gốc:  {bang_gia}")
```

Máy in ra:

```text title=readonly
Giá giao hàng: [45000, 60000, 65000]
Bảng giá gốc:  [45000, 60000, 65000]
```

Dòng thứ hai chính là cú bất ngờ: `bang_gia` — cái tên đứng NGOÀI hàm,
đại diện cho bảng giá thật đang treo trên tường — cũng đã đổi, dù không
dòng lệnh nào trong chương trình viết `bang_gia = ...`. Lý do nằm gọn
trong thân hàm: `gia[i] += phi` là SỬA TẠI CHỖ trên tham số `gia`, mà
`gia` lại trùng địa chỉ với `bang_gia` — đúng luật `mem.aliasing-explained`.
Byte chỉ định "xem thử", nhưng hàm này đã âm thầm SỬA THẬT.
::::

::::predict{#doan-diem-khuyen-khich commitOnce}
Một hàm cộng thêm điểm khuyến khích vào bảng điểm của lớp — cũng sửa
tại chỗ, y hệt ví dụ trên.

**Trước khi chạy**, bạn đoán dòng `print(diem_lop)` in ra gì?

```python
def cong_diem_khuyen_khich(diem):
    for i in range(len(diem)):
        diem[i] += 2
    return diem

diem_lop = [7, 8, 6]
diem_moi = cong_diem_khuyen_khich(diem_lop)

print(diem_lop)
```

:::opt{correct}
`[9, 10, 8]`
:::

:::opt
`[7, 8, 6]` — không đổi, vì `diem_moi` mới là bản chứa điểm đã cộng
::why
Gần đúng ở việc bạn tin hàm chỉ nên đổi thứ nó TRẢ VỀ (`diem_moi`), để
`diem_lop` yên — đó đúng là cách một hàm NÊN cư xử, nhưng chưa phải cách
hàm NÀY thật sự cư xử.

Chỗ lệch: `diem` (tham số) trùng địa chỉ với `diem_lop` ngay từ lúc gọi
hàm — đúng luật đã học. Dòng `diem[i] += 2` sửa tại chỗ trên đúng bảng
điểm đó, nên `diem_lop` ở ngoài thấy thay đổi ngay, không cần đợi gán
lại qua `diem_moi`.
::
:::

:::opt
`[9, 10, 6]` — chỉ hai phần tử đầu đổi, phần tử cuối giữ nguyên
::why
Gần đúng ở việc bạn đoán đúng ĐÂY LÀ sửa tại chỗ (không phải bản sao) —
đúng hướng suy luận chính bài này đang dạy.

Chỗ lệch nằm ở vòng lặp. `range(len(diem))` với `diem` có 3 phần tử sinh
ra đúng ba chỉ số: `0`, `1`, `2` — không thiếu chỉ số cuối. Phần tử ở vị
trí `2` (giá trị `6`) cũng được duyệt tới và cộng thêm `2`, ra `8`, y hệt
hai phần tử kia.
::
:::

:::opt
Máy dừng lại báo lỗi, vì sửa `diem` ngay trong lúc vòng lặp đang chạy
trên chính nó
::why
Gần đúng ở việc bạn nhớ đúng một cái bẫy có thật của Python: sửa một
`list` NGAY TRONG LÚC lặp trực tiếp qua nó (`for x in diem:`) có thể làm
vòng lặp bỏ sót hoặc lặp lại phần tử — phản xạ cảnh giác đó không sai.

Chỗ lệch: vòng lặp ở đây không lặp trực tiếp qua `diem`, mà lặp qua
`range(len(diem))` — một dãy CHỈ SỐ được tính một lần, ngay từ đầu, độc
lập với việc `diem` có bị sửa hay không trong lúc chạy. Sửa GIÁ TRỊ tại
một chỉ số (`diem[i] += 2`) không đụng tới SỐ LƯỢNG phần tử, nên
`range(len(diem))` không hề bị ảnh hưởng — không có lỗi nào cả.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đúng, và đáng lo: một dòng lệnh không hề nhắc tên `diem_lop` vẫn làm nó
đổi. Đây chính là cái giá của việc không ai trả lời câu hỏi "ai đang
giữ, ai chỉ đang mượn".
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hàm `tang_them_phi_giao_hang` và `cong_diem_khuyen_khich` đều làm đúng
một việc: SỬA tham số của chúng tại chỗ. Nhưng mục đích thật của cả hai
hàm không phải "sửa list gốc" — mục đích là TÍNH RA một list mới, để
người gọi tự quyết định dùng nó thế nào.

Có cách nào viết lại hai hàm này để chúng vẫn tính đúng — nhưng không
đụng một chữ nào vào `bang_gia` hay `diem_lop` ở ngoài không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
