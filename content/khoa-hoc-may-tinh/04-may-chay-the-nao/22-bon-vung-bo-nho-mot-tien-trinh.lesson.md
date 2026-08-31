---
id: khoa-hoc-may-tinh.may-chay-the-nao.bon-vung-bo-nho-mot-tien-trinh
title: "Bộ nhớ một tiến trình chia thành nhiều vùng"
summary: "RAM không phải một khối trơn cho một chương trình đang chạy dùng tuỳ ý — nó chia thành bốn vùng có việc riêng: mã lệnh (bài 1), dữ liệu tĩnh, ngăn xếp gọi hàm (bài 11-15), và đống cấp phát khi chạy."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [may.process-memory-layout]
requires: [may.memory-hierarchy]
concepts: [may.process-memory-layout]
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
Năm tầng xếp xong theo tốc độ. Nhưng bên trong tầng RAM ấy — chỗ chương
trình của bạn thật sự sống — có ngăn nắp gì không?
::::

::::explain{#bon-vung-co-viec-rieng}
Có. Một **tiến trình** (process — một chương trình đang chạy, đã được
nạp vào RAM) không được phép vung tay dùng RAM một cách lộn xộn. Vùng
RAM cấp cho nó chia thành bốn phần, mỗi phần chỉ làm đúng một việc:

1. **Mã lệnh** — nơi giữ chính chuỗi lệnh máy mà `dis.dis()` cho bạn xem
   từ bài 1. Nó được nạp vào MỘT lần, lúc chương trình bắt đầu chạy, và
   không đổi trong suốt vòng đời chương trình — dù hàm đó được gọi một
   lần hay một triệu lần.
2. **Dữ liệu tĩnh** — những hằng số, chuỗi chữ đã có sẵn ngay trong mã
   nguồn (như con số `3` trong `LOAD_CONST` ở bài 1), nạp sẵn cùng lúc
   với mã lệnh, cũng không đổi lúc chạy.
3. **Ngăn xếp gọi hàm** — nơi mỗi lời gọi hàm (bài 11-15) mở một khung
   mới (bài 12). Vùng này đổi liên tục: phình ra khi có lời gọi, co lại
   khi hàm trả về.
4. **Đống cấp phát** — nơi những giá trị tạo ra LÚC CHƯƠNG TRÌNH ĐANG
   CHẠY được cấp chỗ — một `list` mới, một `dict` mới. Cũng đổi liên
   tục, nhưng không theo luật vào-sau-ra-trước như ngăn xếp.

Hai vùng đầu ĐỨNG YÊN — nạp một lần, giữ nguyên. Hai vùng sau SỐNG ĐỘNG —
lớn lên, co lại theo đúng nhịp chương trình chạy. Cụm bài này sẽ đào sâu
đúng hai vùng sống động đó, vì tên của chúng — "ngăn xếp" và "đống" —
nghe rất quen.
::::

::::example{#gia-giam-khong-doi-ma-lenh}
Một hàm tính giá sau khi giảm, gọi hai lần với hai cặp số khác hẳn nhau:

```python title=readonly
import dis

def gia_sau_giam(gia, phan_tram_giam):
    return gia - gia * phan_tram_giam / 100

gia_ao_1 = gia_sau_giam(200000, 10)
gia_ao_2 = gia_sau_giam(500000, 20)

ma_lenh_khong_doi = gia_sau_giam.__code__ is gia_sau_giam.__code__
print(gia_ao_1, gia_ao_2)
print(ma_lenh_khong_doi)
```

```text title=readonly
180000.0 400000.0
True
```

`gia_ao_1` và `gia_ao_2` là hai KẾT QUẢ khác hẳn nhau — dữ liệu lúc chạy,
đổi theo từng lời gọi. Nhưng `gia_sau_giam.__code__` — chính đối tượng
lệnh máy mà `dis.dis()` sẽ tháo ra được — là CÙNG một vật, dù gọi bao
nhiêu lần, với số nào. `ma_lenh_khong_doi` luôn `True`. Vùng mã lệnh
đứng yên; vùng chứa kết quả tính toán thì không.
::::

::::predict{#vung-nao-khong-doi commitOnce}
Một chương trình gọi `gia_sau_giam(300000, 15)` ba lần liên tiếp, với ba
cặp số khác nhau mỗi lần.

**Trước khi đọc đáp án**, bạn đoán: vùng nào trong bốn vùng bài này vừa
nêu KHÔNG hề đổi, bất kể gọi bao nhiêu lần, với số nào?

:::opt{correct}
Mã lệnh — `dis.dis(gia_sau_giam)` luôn in ra đúng cùng một chuỗi lệnh,
không quan tâm đối số truyền vào hay số lần gọi.
:::

:::opt
Đống cấp phát — vì đống cũng chứa những giá trị dùng lại được nhiều lần,
không tạo mới liên tục
::why
Gần đúng ở việc bạn nhớ đúng: đống KHÔNG theo luật vào-sau-ra-trước cứng
nhắc như ngăn xếp — phần đó đúng.

Chỗ lệch: mỗi lần `gia_sau_giam` chạy, phép trừ `gia - gia * ... / 100`
tính ra một SỐ MỚI, và số mới đó cần chỗ trên đống (bài sau sẽ đào sâu
đúng điều này). Ba lần gọi với ba cặp số khác nhau tạo ra BA kết quả
khác nhau nằm trên đống — đống vẫn đổi liên tục, chỉ là không theo luật
vào-sau-ra-trước.
::
:::

:::opt
Ngăn xếp gọi hàm — vì cùng một hàm được gọi thì luôn mở đúng MỘT khung
giống hệt nhau
::why
Gần đúng ở việc bạn nhớ đúng: mỗi lần gọi `gia_sau_giam` đều mở một khung
(bài 12) có HÌNH DẠNG giống nhau — cùng số biến cục bộ, cùng cấu trúc.

Chỗ lệch: khung MỚI vẫn được MỞ MỚI mỗi lần gọi, và NỘI DUNG bên trong nó
(giá trị của `gia`, `phan_tram_giam` lúc đó) khác nhau mỗi lần. Vùng ngăn
xếp gọi hàm liên tục phình ra rồi co lại — nó không "đứng yên" như mã
lệnh.
::
:::

:::opt
Dữ liệu tĩnh — vì con số `100` trong `phan_tram_giam / 100` không đổi
suốt chương trình
::why
Gần đúng ở đúng MỘT chi tiết: hằng số `100` nằm trong mã nguồn — nó thuộc
dữ liệu tĩnh, và đúng là không đổi.

Chỗ lệch: câu hỏi hỏi về CẢ VÙNG, không phải một hằng số lẻ. Việc `100`
không đổi không có nghĩa TOÀN BỘ chuỗi lệnh máy không đổi — điều đó phải
nói về vùng mã lệnh, nơi giữ cả `LOAD_FAST`, `BINARY_OP`, `RETURN_VALUE`
chứ không chỉ một con số.
::
:::
::::

::::code{#hoa-hong-khong-doi-ma-lenh}
Quầy bán hàng tính hoa hồng cho hai đơn khác nhau. Gọi thêm một lần nữa
với số khác hẳn, rồi xác nhận vùng mã lệnh vẫn đứng yên.

```python title=starter
import dis

def tien_hoa_hong(doanh_so, ty_le_phan_tram):
    return doanh_so * ty_le_phan_tram / 100

hoa_hong_nho = tien_hoa_hong(3000000, 5)
hoa_hong_lon = ___                 # gọi tien_hoa_hong với doanh_so=20000000, ty_le=8

ma_lenh_khong_doi = tien_hoa_hong.__code__ is tien_hoa_hong.__code__
print(hoa_hong_nho, hoa_hong_lon)
print(ma_lenh_khong_doi)
```

```python title=solution
import dis

def tien_hoa_hong(doanh_so, ty_le_phan_tram):
    return doanh_so * ty_le_phan_tram / 100

hoa_hong_nho = tien_hoa_hong(3000000, 5)
hoa_hong_lon = tien_hoa_hong(20000000, 8)

ma_lenh_khong_doi = tien_hoa_hong.__code__ is tien_hoa_hong.__code__
print(hoa_hong_nho, hoa_hong_lon)
print(ma_lenh_khong_doi)
```

```python title=test
assert hoa_hong_nho == 150000.0, f"hoa_hong_nho phải là 150000.0 — đang ra {hoa_hong_nho}"
assert hoa_hong_lon == 1600000.0, f"hoa_hong_lon phải gọi tien_hoa_hong(20000000, 8) và ra 1600000.0 — đang ra {hoa_hong_lon}"
assert ma_lenh_khong_doi is True, "vùng mã lệnh của tien_hoa_hong phải là CÙNG một đối tượng ở mọi lần truy cập .__code__"
```

:::hints
- kind: attention
  body: Chỉ một chỗ trống — một lời GỌI hàm tien_hoa_hong với hai đối số mới, đúng thứ tự (doanh_so trước, ty_le_phan_tram sau).
- kind: strategy
  body: 'Gọi lại đúng hàm tien_hoa_hong, lần này với doanh_so=20000000 và ty_le_phan_tram=8 — cùng một hàm, cùng vùng mã lệnh, chỉ khác dữ liệu truyền vào.'
- kind: one-line
  body: 'Chỗ trống là: tien_hoa_hong(20000000, 8)'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ gọi tien_hoa_hong(...) lần thứ hai — không gõ cứng một con số, vì bài này đang chứng minh vùng mã lệnh đứng yên qua NHIỀU LẦN GỌI thật
  requireAst:
  # min: 2 — đếm thật trên solution: tien_hoa_hong bị GỌI đúng 2 lần trong mã
  # nguồn (hoa_hong_nho đã có sẵn trong khung + chỗ trống). Điền True/1/0 chỉ
  # còn 1 lần gọi — dưới 2, luật này chặn được. Đã thử cả ba cách hollow: cả
  # ba đều dừng an toàn, không lặp gì cả (không có vòng lặp trong bài này),
  # và cả ba đều cho hoa_hong_lon sai (True/1/0, không phải 1600000.0) —
  # assert bắt được độc lập với luật static này.
  - kind: uses-call, target: tien_hoa_hong, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^150000\\.0 1600000\\.0\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
150000.0 rồi 1600000.0 — hai kết quả khác hẳn. Nhưng `.__code__` vẫn là
đúng một vật, cả hai lần. Vùng mã lệnh không nhúc nhích.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Trong bốn vùng vừa nêu, một vùng gọi là **ngăn xếp gọi hàm**. Cái tên ấy
nghe rất quen — T3.2 đã dạy một cấu trúc dữ liệu cũng mang tên **ngăn
xếp**, dựng bằng `list` và `.append`/`.pop`.

Hai thứ cùng tên này có phải là MỘT không? Hay chúng chỉ tình cờ trùng
chữ?

Bài sau trả lời thẳng.
::::

::::checkpoint{mastery=0.8}
::::
