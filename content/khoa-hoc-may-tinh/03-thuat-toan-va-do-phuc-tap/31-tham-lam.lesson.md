---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.tham-lam
title: "Tham lam: chọn tốt nhất ngay lúc này, không nhìn lại"
summary: "Trả tiền thừa bằng ít đồng nhất: mỗi bước lấy TỐI ĐA số đồng mệnh giá lớn nhất còn dùng được, không cân nhắc lựa chọn nào khác, không bao giờ quay lại sửa bước trước. Với hệ mệnh giá 1-2-5-10-20-50 quen thuộc, cách chọn vội này luôn ra đáp án ít đồng nhất."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 31
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.greedy]
requires: [alg.compare-by-big-o]
concepts: [alg.greedy]
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
Ba mươi bài trước toàn là những thuật toán đã có công thức sẵn — hôm nay
đổi câu hỏi: làm sao TỰ NGHĨ RA một cách giải, khi chưa ai chỉ sẵn công
thức nào?
::::

::::explain{#chon-voi-khong-nhin-lai}
Từ bài tìm tuyến tính tới BFS/DFS, mỗi thuật toán đã học là một THỦ TỤC cố
định — luôn làm đúng những bước ấy, theo đúng thứ tự ấy. **Tham lam**
(greedy) không phải một thủ tục cụ thể như vậy. Nó là một CÁCH RA QUYẾT
ĐỊNH áp được lên nhiều bài toán khác nhau, tóm gọn trong một câu: ở MỖI
bước, chọn lựa chọn có lợi nhất NGAY LÚC ĐÓ, không so sánh với những bước
sau, và không bao giờ quay lại sửa một bước đã chọn.

Bài toán kinh điển: một quầy thu ngân phải trả tiền thừa, dùng ÍT ĐỒNG
NHẤT có thể. Có sẵn một bộ **mệnh giá** (denomination) — những con số tiền
cố định được phép dùng, ví dụ 50, 20, 10, 5, 2, 1 (đơn vị nghìn đồng).
Cách tham lam làm: xét mệnh giá LỚN NHẤT trước — 50 — lấy càng nhiều đồng
mệnh giá đó càng tốt, miễn còn đủ tiền thừa để lấy thêm một đồng nữa.
Hết khả năng lấy thêm đồng 50 thì chuyển xuống mệnh giá kế tiếp — 20 — và
lặp lại đúng luật đó, cho tới mệnh giá 1 thì luôn lấy hết phần dư còn lại.

Không có bước nào "cân nhắc": không hỏi "nếu lấy ít đồng 50 hơn thì tổng
số đồng có ít hơn không?". Chỉ có một luật duy nhất, lặp lại: **lớn nhất
trước, lấy tối đa, rồi đi tiếp**. Chính vì không cân nhắc, mỗi bước chỉ
tốn đúng một phép tính — không thử phương án nào khác, không quay lại —
nên tham lam luôn thuộc loại thuật toán NHANH.
::::

::::example{#tra-87-nghin}
Byte viết đúng luật vừa nêu, dùng phép chia lấy phần nguyên (`//`) để
biết một mệnh giá dùng được TỐI ĐA bao nhiêu đồng, và phép chia lấy dư
(qua `-=`) để biết còn thiếu bao nhiêu sau khi lấy hết số đồng đó:

```python title=readonly
menh_gia = [50, 20, 10, 5, 2, 1]
tien_con_thieu = 87
danh_sach_dong = []

for mg in menh_gia:
    so_luong = tien_con_thieu // mg
    danh_sach_dong += [mg] * so_luong
    tien_con_thieu -= mg * so_luong

print(danh_sach_dong)
print(f"Tổng cộng {len(danh_sach_dong)} đồng")
```

```text title=readonly
[50, 20, 10, 5, 2]
Tổng cộng 5 đồng
```

Đi từng bước: 87 chia 50 được 1, lấy một đồng 50, còn thiếu 37. 37 chia
20 được 1, lấy một đồng 20, còn thiếu 17. Cứ vậy: một đồng 10 (còn 7), một
đồng 5 (còn 2), một đồng 2 (còn 0). Mệnh giá 1 không cần dùng tới, vì phần
thiếu đã về 0 từ trước. Đúng năm đồng — không có cách trả 87 nào bằng hệ
mệnh giá này dùng ÍT hơn năm đồng, dù tham lam chưa từng thử phương án
nào khác để biết điều đó.
::::

::::predict{#doan-tra-47-nghin commitOnce}
Cũng đúng luật ấy, cũng hệ mệnh giá `[50, 20, 10, 5, 2, 1]`, nhưng
`tien_con_thieu = 47`:

```python
menh_gia = [50, 20, 10, 5, 2, 1]
tien_con_thieu = 47
danh_sach_dong = []

for mg in menh_gia:
    so_luong = tien_con_thieu // mg
    danh_sach_dong += [mg] * so_luong
    tien_con_thieu -= mg * so_luong

print(danh_sach_dong)
```

**Trước khi chạy**, bạn đoán màn hình in ra danh sách nào?

:::opt{correct}
`[20, 20, 5, 2]` — mệnh giá 50 không dùng được (47 nhỏ hơn 50), nên mệnh
giá 20 là mệnh giá lớn nhất dùng được, và 47 chia 20 được 2, nên LẤY TỐI
ĐA hai đồng 20 liền một lúc, còn thiếu 7, rồi mới hạ xuống 5 và 2
:::

:::opt
`[20, 10, 10, 5, 2]` — cũng cộng đúng 47, chỉ đổi cách chia phần giữa
::why
Gần đúng ở việc tổng tiền cộng lại đúng 47 — con số đó không sai.

Chỗ lệch nằm ở đúng chỗ luật tham lam nghiêm ngặt nhất: sau khi lấy một
đồng 20, tiền còn thiếu là 27 — VẪN CÒN đủ để lấy thêm một đồng 20 nữa
(27 ≥ 20), nên luật "lấy TỐI ĐA số đồng mệnh giá này trước khi chuyển
xuống" bắt buộc phải lấy đồng 20 thứ hai ngay, không được tách nó ra
thành hai đồng 10. Tham lam không "chọn cách chia đẹp" — nó chỉ lặp lại
đúng một phép chia lấy phần nguyên.
::
:::

:::opt
`[10, 10, 10, 10, 5, 2]` — không dùng đồng 20 nào cả, chỉ dùng đồng nhỏ
::why
Gần đúng ở việc tổng cộng vẫn ra đúng 47.

Chỗ lệch: thuật toán LUÔN xét mệnh giá lớn nhất khả dụng TRƯỚC — mệnh giá
20 đứng ngay sau 50 trong danh sách, và 47 ≥ 20 nên nó chắc chắn được xét
và được lấy trước khi vòng lặp đi xuống mệnh giá 10. Bỏ qua hẳn mệnh giá
20 chỉ xảy ra nếu danh sách mệnh giá không có nó, hoặc số tiền còn thiếu
nhỏ hơn 20 — cả hai điều đó không đúng ở đây.
::
:::

:::opt
Máy báo lỗi, vì 47 không chia hết cho bất kỳ mệnh giá nào trong danh sách
::why
Gần đúng ở việc 47 đúng là không chia hết gọn cho 50, 20, 10 hay 5.

Chỗ lệch: `//` không đòi chia hết — nó chỉ lấy PHẦN NGUYÊN của phép chia,
luôn có kết quả với số nguyên, kể cả khi còn dư. Phần dư được để lại cho
mệnh giá NHỎ HƠN xử lý tiếp, và mệnh giá cuối cùng (1) luôn chia hết mọi
số nguyên — không có cách nào phần thiếu còn sót lại sau khi đi hết cả
sáu mệnh giá.
::
:::
::::

::::code{#tra-68-nghin}
Đổi số tiền cần trả thành 68. Khung vòng lặp giữ nguyên — việc của bạn là
điền đúng phép chia quyết định LẤY MẤY ĐỒNG một mệnh giá, và phép trừ cập
nhật lại số tiền còn thiếu sau khi lấy đúng chừng ấy đồng.

```python title=starter
menh_gia = [50, 20, 10, 5, 2, 1]
tien_con_thieu = 68
danh_sach_dong = []

for mg in menh_gia:
    so_luong = tien_con_thieu // ___    # tối đa mấy đồng MG này dùng được?
    danh_sach_dong += [mg] * so_luong
    tien_con_thieu -= mg * ___          # trừ đúng số tiền vừa lấy

print(danh_sach_dong)
print(f"Tổng cộng {len(danh_sach_dong)} đồng")
```

```python title=solution
menh_gia = [50, 20, 10, 5, 2, 1]
tien_con_thieu = 68
danh_sach_dong = []

for mg in menh_gia:
    so_luong = tien_con_thieu // mg
    danh_sach_dong += [mg] * so_luong
    tien_con_thieu -= mg * so_luong

print(danh_sach_dong)
print(f"Tổng cộng {len(danh_sach_dong)} đồng")
```

```python title=test
assert danh_sach_dong == [50, 10, 5, 2, 1], f"trả 68 bằng hệ mệnh giá này phải ra [50, 10, 5, 2, 1] — đang ra {danh_sach_dong}"
assert len(danh_sach_dong) == 5, f"phải dùng đúng 5 đồng — đang ra {len(danh_sach_dong)}"
assert tien_con_thieu == 0, f"phải trả HẾT, không còn dư — tien_con_thieu đang là {tien_con_thieu}"
```

:::hints
- kind: attention
  body: Hai chỗ trống là HAI VIỆC khác nhau — chỗ trong // phải tính XONG SỐ LƯỢNG đồng, chỗ trong -= phải nói lại đúng chính số lượng đó để trừ tiền cho khớp.
- kind: strategy
  body: 'so_luong = tien_con_thieu // mg tính TỐI ĐA bao nhiêu đồng mệnh giá mg dùng được (chia lấy phần nguyên). tien_con_thieu -= mg * so_luong trừ đúng số tiền vừa lấy — mg nhân với chính so_luong vừa tính, không phải một con số khác.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là mg (trong // mg) và so_luong (trong mg * so_luong).'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hai chỗ trống phải THẬT SỰ dùng mg và so_luong đúng vai trò của chúng — chỗ trống 1 quyết định SỐ LƯỢNG đồng bằng phép chia lấy phần nguyên cho mg, chỗ trống 2 phải trừ lại đúng số lượng đó — không phải một câu không làm gì như True, 1, 0
  requireAst:
  - kind: uses-name, target: mg, min: 3
  - kind: uses-name, target: so_luong, min: 2
  # min: 3 cho mg — đếm thật trên solution bằng kiemAst: mg (Load) xuất hiện
  # ở "for mg in menh_gia" là Store nên KHÔNG tính, rồi ba lần Load thật:
  # "tien_con_thieu // mg" (chỗ trống 1), "[mg] * so_luong", và
  # "mg * so_luong" (chỗ trống 2) = đúng 3. min: 2 cho so_luong — đếm thật:
  # "[mg] * so_luong" và "mg * so_luong" (chỗ trống 2) = đúng 2 (gán
  # "so_luong = ..." là Store, không tính). ĐÃ THỬ THẬT: điền True/1/0 vào
  # CẢ HAI chỗ trống cùng lúc — cả ba cách đều dừng AN TOÀN (for chạy trên
  # menh_gia cố định 6 phần tử, không có while/đệ quy nào cả nên không có
  # rủi ro treo): True/1 cho ra danh sách dài bất thường (86 đồng, toàn 50
  # và 20) và trượt cả static (mg còn 2, so_luong còn 0) lẫn tests; 0 làm nổ
  # ZeroDivisionError ngay ở phép chia đầu tiên — bị run tier bắt.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\[50, 10, 5, 2, 1\\]\\nTổng cộng 5 đồng\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm đồng, không hơn không kém — mỗi bước chỉ nhìn đúng một mệnh giá, chọn
xong là đi thẳng, không ngoái lại.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hệ mệnh giá `[50, 20, 10, 5, 2, 1]` không phải chọn ngẫu nhiên — nó là hệ
mệnh giá quen thuộc bạn vẫn cầm trên tay mỗi ngày. Với hệ này, cách chọn
vội của tham lam luôn ra đúng số đồng ÍT NHẤT có thể — dù nó chưa từng
thử phương án nào khác để biết điều đó.

Câu hỏi là: điều đó có phải MAY MẮN của riêng hệ mệnh giá 1-2-5-10-20-50
này, hay tham lam luôn đúng với BẤT KỲ bộ mệnh giá nào? Thử tưởng tượng
một bộ mệnh giá khác hẳn — không đẹp như 1, 2, 5, 10 — luật "lớn nhất
trước, lấy tối đa" có còn ra đáp án ít đồng nhất không?

Bài sau kiểm chứng đúng câu hỏi đó.
::::

::::checkpoint{mastery=0.8}
::::
