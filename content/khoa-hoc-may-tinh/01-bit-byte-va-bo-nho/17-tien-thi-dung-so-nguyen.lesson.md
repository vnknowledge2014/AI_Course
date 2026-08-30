---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.tien-thi-dung-so-nguyen
title: "Tính tiền thì đếm bằng đồng, đừng đếm bằng nghìn"
summary: "Tiền không có 'nửa đồng' — cộng dồn bằng số thập phân (nghìn) có thể đẻ ra một khoản tiền lẻ vô lý; đếm bằng số nguyên (đồng) thì không bao giờ sai kiểu đó."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [mem.integer-money]
requires: [mem.float-compare, core.float-precision, core.variable, core.assignment, core.augmented-assign, core.number-literal, core.fstring, ctrl.for-range, ctrl.comparison, core.boolean]
concepts: [mem.integer-money]
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
Bài trước bạn học cách so sánh số lẻ cho khéo. Bài này chỉ một cách né hẳn:
đừng để tiền thành số lẻ ngay từ đầu.
::::

::::explain{#dong-la-don-vi-nho-nhat}
Bài trước dạy: đừng hỏi `0.1 + 0.2 == 0.3`, vì máy lưu số lẻ trong hệ nhị
phân, và câu hỏi ấy gần như luôn trả lời sai. Cách chữa là đổi câu hỏi —
hỏi "chênh nhau ít hơn một ngưỡng" thay vì hỏi "bằng nhau y hệt".

Cách đó chữa được **triệu chứng**. Bài này chỉ một cách né hẳn **nguyên
nhân**, áp dụng riêng cho một thứ bạn tính mỗi ngày: tiền.

Tiền Việt Nam không có "nửa đồng". Đơn vị nhỏ nhất là **đồng**, và một
khoản tiền hợp lệ luôn là một số **nguyên** đồng — không lẻ, không chia nhỏ
hơn được nữa. 40 000 đồng là 40 000 đồng, không bao giờ là
40 000,37 đồng.

Nhưng có một thói quen dễ vấp: viết giá theo đơn vị **nghìn** cho gọn — 40
thay vì 40000, 15,5 thay vì 15500. Con số 15,5 ấy không còn là số nguyên
nữa. Nó là một số thập phân, và mọi số thập phân trong máy đều mang đúng
cái nguy cơ mà bài trước vừa chỉ ra.
::::

::::example{#quy-boa-le}
Buổi sáng, ba lượt khách để lại tiền boa lẻ cho cô Bảy — mỗi lượt đúng 100
đồng. Byte thử cộng dồn ba lượt ấy, ghi sổ theo đơn vị **nghìn** đồng cho
gọn, tức mỗi lượt là 0,1 nghìn.

```python title=readonly
quy_boa_nghin = 0.0
for lan in range(3):
    quy_boa_nghin += 0.1

print(f"Quỹ boa: {quy_boa_nghin} nghìn đồng")
print(f"Đổi ra đồng: {quy_boa_nghin * 1000} đồng")
```

Máy in ra:

```text title=readonly
Quỹ boa: 0.30000000000000004 nghìn đồng
Đổi ra đồng: 300.00000000000006 đồng
```

Dòng thứ hai là chỗ đáng dừng lại nhìn. "300,00000000000006 đồng" không
phải một con số làm tròn hơi xấu — nó là **một phần rất nhỏ của một
đồng**, một thứ không tồn tại trong đời thật. Ba lượt 100 đồng cộng lại
phải ra đúng 300 đồng, không hơn không kém, và máy vừa cho ra một khoản
tiền không ai in nổi trên hoá đơn.

Byte thử lại, lần này đếm bằng **đồng** — số nguyên — ngay từ đầu:

```python title=readonly
quy_boa = 0
for lan in range(3):
    quy_boa += 100

print(f"Quỹ boa: {quy_boa} đồng")
```

```text title=readonly
Quỹ boa: 300 đồng
```

Không có phần lẻ nào để mà lo. Số nguyên cộng số nguyên luôn ra số nguyên,
đúng tuyệt đối — không có sai số nào tích luỹ qua từng lượt cộng, dù cộng
ba lần hay ba nghìn lần.
::::

::::explain{#chon-don-vi-nho-nhat}
Đây là lý do quán phở suốt các bài trước luôn viết giá bằng số nguyên
đồng — tô nhỏ 40.000, tô vừa 45.000, tô to 55.000, đĩa quẩy 10.000, trà đá
5.000 — chứ không viết 40, 45, 55, 10, 5 rồi ngầm hiểu là nghìn. Không
phải tình cờ, và giờ bạn biết vì sao.

Luật chung: chọn đơn vị **nhỏ nhất** mà thứ bạn tính có thật — với tiền
Việt Nam, đó là đồng — và giữ nó là số **nguyên** suốt từ đầu tới cuối
phép tính. Đừng để dấu phẩy thập phân bén mảng vào chỗ nào cộng dồn, nhân,
chia tiền. Muốn hiển thị gọn theo nghìn thì **chia ra để in**, chứ đừng
**tính bằng số đã chia**.
::::

::::predict{#doan-cong-don-lai commitOnce}
Byte thử một buổi khác. Lần này ba lượt tính lãi bán trà đá, mỗi lượt lãi
đúng 0,1 nghìn đồng — cộng dồn qua một vòng lặp, y hệt cách quỹ boa vừa
cộng hỏng ở trên.

**Trước khi xem đáp án**, bạn đoán hai dòng in ra gì?

```python
lai = 0.0
for i in range(3):
    lai += 0.1

print(lai)
print(lai == 0.3)
```

:::opt{correct}
`0.30000000000000004`, rồi `False`
:::

:::opt
`0.3`, rồi `True`
::why
Gần đúng ở chỗ đây đúng là kết quả mà phép toán thật sự muốn nói: 0,1 cộng
ba lần đúng bằng 0,3. Về mặt số học, bạn không sai một ly nào.

Chỗ lệch nằm ở chỗ máy không lưu 0,1 chính xác. Hệ nhị phân không viết hết
được phân số một phần mười — đúng như bài trước đã chỉ ra với
`0.1 + 0.2`. Cộng ba lần một con số đã xấp xỉ thì phần xấp xỉ ấy cộng dồn
theo, và kết quả không còn tròn trịa 0,3 nữa.
::
:::

:::opt
`0.30000000000000004`, rồi `True`
::why
Gần đúng ở chỗ bạn đọc trúng dòng đầu — con số dài ngoằng đó đúng là thứ
máy in ra, không phải bạn tưởng tượng.

Chỗ lệch nằm ở dòng sau. Dấu `==` không tự làm tròn hay "châm chước" một
chút sai số nào cả; nó so từng bit một. `0.30000000000000004` và `0.3` là
hai dãy bit khác nhau, nên `==` giữa chúng trả lời đúng những gì nó thấy:
`False`.
::
:::

:::opt
Máy dừng lại báo lỗi, vì không cộng được số thập phân trong vòng lặp
::why
Gần đúng ở chỗ bạn cảnh giác đúng chỗ: vòng lặp này đang làm một việc dễ
vỡ, và cảnh giác trước một đoạn mã như vậy là phản xạ tốt.

Chỗ lệch là Python cộng số thập phân trong vòng lặp bình thường như mọi
phép cộng khác — không có gì bị cấm. Cái vỡ không nằm ở chỗ "có cộng được
hay không"; cộng được, chạy trơn tru, không một tiếng báo lỗi nào. Cái vỡ
nằm ở chỗ con số cộng ra không còn đúng bằng con số bạn kỳ vọng.
::
:::
::::

::::code{#quy-boa-bang-dong}
Tới lượt bạn ghi sổ quỹ boa — lần này đúng theo cách vừa học: đếm bằng
**đồng**, số nguyên, ngay từ đầu, không quy ra nghìn.

Ba lượt khách để lại tiền boa buổi sáng, mỗi lượt đúng 100 đồng. Cộng dồn
qua vòng lặp, y như đoạn Byte vừa làm hỏng ở trên — chỉ khác đơn vị.

```python title=starter
quy_boa = 0
for lan in range(3):
    quy_boa += ___

print(f"Quỹ boa buổi sáng: {quy_boa} đồng")
print(quy_boa == 300)
```

```python title=solution
quy_boa = 0
for lan in range(3):
    quy_boa += 100

print(f"Quỹ boa buổi sáng: {quy_boa} đồng")
print(quy_boa == 300)
```

```python title=test
# Sai kiểu này không cho ra một CON SỐ khác — nó cho ra một KIỂU khác. Cộng
# bằng 0.1 hay 100.0 vẫn có thể trông đúng thoáng qua, nên phải hỏi thẳng
# kiểu dữ liệu, không chỉ hỏi giá trị.
assert isinstance(quy_boa, int), "quỹ boa phải là SỐ NGUYÊN đồng — cộng bằng 0.1 hay 100.0 sẽ cho ra một số thập phân, đúng thứ bài này dạy phải tránh"
assert quy_boa == 300, "ba lượt, mỗi lượt 100 đồng — cộng dồn qua ba vòng lặp phải ra đúng 300, không hơn không kém"
```

:::hints
- kind: attention
  body: Chỗ trống là số tiền MỘT LƯỢT boa, tính bằng đồng — không phải tổng cả buổi, và không phải đơn vị nghìn. Vòng lặp chạy đúng ba lần; mỗi lần nó cộng thêm đúng chỗ trống ấy vào `quy_boa`.
- kind: strategy
  body: Đọc lại đề — "mỗi lượt đúng 100 đồng". Điền thẳng con số đó, viết nguyên, không có dấu phẩy thập phân, và không nhân với biến đếm vòng lặp `lan` — biến ấy chỉ để đếm số lần lặp, không phải một phần của số tiền boa.
- kind: one-line
  body: "Điền `100` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi lượt boa phải là con số NGUYÊN 100, cộng thẳng qua vòng lặp — không nhân với biến đếm vòng lặp, và không dùng số thập phân
  requireAst:
  - kind: has-literal, target: 100
  forbidAst:
  - kind: uses-operator, target: "*"
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Quỹ boa buổi sáng: 300 đồng\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba lượt, mỗi lượt một trăm, cộng đúng ba trăm — không thừa một phần nghìn
đồng nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Tiền giữ đúng khi bạn đổi ý niệm "nghìn" lẻ thành số nguyên "đồng". Còn cái
tên món trên bảng thực đơn của cô Bảy — chữ **Phở** — thì sao?

Chữ không phải số bạn gõ tay như `40000`. Nhưng hồi mới học "vì sao máy
chỉ hiểu số", bạn đã biết mỗi ký tự có một con số riêng trong một bảng, và
bảng ấy lớn tới mức một con số như 7903 — số của chữ `ở` — không nhét vừa
một byte, vốn chỉ đếm được tới 255.

Không nhét vừa một byte thì chữ ấy nằm ở đâu? Nó có tràn sang ô bên cạnh
không, và nếu có thì tràn đúng bao nhiêu ô?

Đừng trả lời vội. Bài sau bạn tự đếm.
::::

::::checkpoint{mastery=0.8}
::::
