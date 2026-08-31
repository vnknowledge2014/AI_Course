---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.khi-tham-lam-sai
title: "Khi tham lam sai — không phải lúc nào cũng đúng"
summary: "Một hệ mệnh giá giả — 4, 3, 1 — mà luật 'lớn nhất trước, lấy tối đa' của bài trước chọn sai: trả 6, tham lam chọn 4+1+1 (ba đồng), trong khi 3+3 (hai đồng) mới là ít nhất. Đo bằng chính con số đếm được, không phải bằng cảm giác 'tham lam chắc luôn đúng'."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 32
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.greedy-fails]
requires: [alg.greedy]
concepts: [alg.greedy-fails]
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
Bài trước tham lam đúng một trăm phần trăm. Hôm nay đi tìm hệ mệnh giá
làm nó vấp.
::::

::::explain{#nhanh-khong-co-nghia-dung}
Bài trước dừng lại ở một cảm giác dễ tin: "lớn nhất trước, lấy tối đa" —
nghe hợp lý tới mức như thể LUÔN đúng. Nhưng "hợp lý" và "luôn đúng" là
hai chuyện khác nhau, và cách duy nhất phân biệt được chúng là THỬ, không
phải tin theo bản năng.

Hệ mệnh giá `[50, 20, 10, 5, 2, 1]` có một tính chất đặc biệt: mỗi mệnh
giá đều **chia hết gọn** cho phần còn lại của hệ (50 chia hết cho 10, 20
chia hết cho 10, và cứ thế) — chính tính chất này khiến "lấy tối đa mệnh
giá lớn nhất" không bao giờ để lại một khoản dư khó xử. Đổi sang một hệ
mệnh giá KHÔNG có tính chất đó, luật cũ vẫn chạy — chạy trọn vẹn, không
lỗi cú pháp, không `RecursionError` — nhưng con số nó đưa ra có thể không
còn là ÍT NHẤT nữa.

Hệ mệnh giá giả: `[4, 3, 1]`. Trả tiền thừa 6:

- **Tham lam** xét 4 trước: 6 ≥ 4, lấy một đồng 4, còn thiếu 2. Mệnh giá
  3: 2 < 3, không lấy được đồng nào. Mệnh giá 1: lấy hai đồng 1. Kết quả:
  `[4, 1, 1]` — **ba đồng**.
- **Cách tốt hơn**, không tham lam: hai đồng 3 — `[3, 3]` — cộng đúng 6,
  chỉ **hai đồng**.

Tham lam thua ngay chính bài toán nó vừa thắng tuyệt đối ở bài trước. Vấn
đề không nằm ở việc viết sai dòng nào — đoạn mã bài trước áp đúng luật
của nó, không đổi một chữ. Vấn đề nằm ở chính LUẬT: "tốt nhất ngay lúc
này" (lấy đồng 4 trước, vì nó là mệnh giá lớn nhất có thể) không hề đảm
bảo "tốt nhất chung cuộc" — đồng 4 vừa lấy đã khoá chặt bước sau vào một
khoản dư 2 khó chia, trong khi bỏ qua đồng 4 để dùng hai đồng 3 mới thật
sự là ít đồng nhất.
::::

::::example{#do-that-hai-cach}
Byte đo lại đúng ví dụ trên bằng mã, không chỉ bằng lời:

```python title=readonly
menh_gia_gia = [4, 3, 1]
tien_con_thieu = 6
dong_tham_lam = []

for mg in menh_gia_gia:
    so_luong = tien_con_thieu // mg
    dong_tham_lam += [mg] * so_luong
    tien_con_thieu -= mg * so_luong

print(dong_tham_lam)
```

```text title=readonly
[4, 1, 1]
```

Đúng ba đồng như tính tay. Không có gì trong đoạn mã này khác đoạn mã bài
trước — cùng một khung, cùng một luật "lớn nhất trước, lấy tối đa". Khác
duy nhất là DANH SÁCH MỆNH GIÁ. Với `[50, 20, 10, 5, 2, 1]` luật này ra
đáp án ít đồng nhất; với `[4, 3, 1]` nó không còn như vậy. Bản thân luật
"tham lam" không hề biết — và không hề CẦN biết — nó đang chạy trên hệ
mệnh giá nào. Nó chỉ mù quáng lặp lại đúng một câu hỏi ở mỗi bước.
::::

::::predict{#doan-tra-10-nghin commitOnce}
Cũng hệ mệnh giá giả `[4, 3, 1]`, nhưng `tien_con_thieu = 10`:

```python
menh_gia_gia = [4, 3, 1]
tien_con_thieu = 10
dong_tham_lam = []

for mg in menh_gia_gia:
    so_luong = tien_con_thieu // mg
    dong_tham_lam += [mg] * so_luong
    tien_con_thieu -= mg * so_luong

print(dong_tham_lam)
```

**Trước khi chạy**, bạn đoán danh sách in ra là gì?

:::opt{correct}
`[4, 4, 1, 1]` — bốn đồng: 10 chia 4 được 2, nên LẤY TỐI ĐA hai đồng 4
liền một lúc trước khi xét mệnh giá 3, còn thiếu 2 thì mệnh giá 3 bỏ qua,
và hai đồng 1 dọn nốt phần dư
:::

:::opt
`[4, 3, 3]` — ba đồng, cộng đúng 10
::why
Gần đúng ở việc `[4, 3, 3]` đúng là MỘT cách hợp lệ để trả 10, và còn ít
đồng hơn cả đáp án đúng — đây thật ra chính là cách TỐI ƯU cho số tiền
này.

Chỗ lệch: câu hỏi không hỏi cách nào TỐT NHẤT, mà hỏi tham lam THỰC SỰ
chọn gì. Ở bước đầu, tham lam thấy 10 ≥ 4 và 10 // 4 = 2 — nó LẤY LUÔN
hai đồng 4 một lúc (đúng luật "lấy tối đa"), không hề để dành một đồng 4
lại cho bước sau. Nó không "nhìn trước" để tránh việc dư 2 khó chia.
::
:::

:::opt
`[4, 3, 1, 1, 1]` — năm đồng, thử mỗi mệnh giá đúng một lần theo thứ tự
::why
Gần đúng ở việc cũng cộng đúng 10, và cũng đi qua đủ ba mệnh giá theo
đúng thứ tự lớn tới nhỏ.

Chỗ lệch: tham lam không "thử mỗi mệnh giá đúng một lần" — dòng
`so_luong = tien_con_thieu // mg` tính TOÀN BỘ số lượng dùng được của
mệnh giá đó ngay một lượt, rồi lấy hết chỗ đó cùng lúc trước khi chuyển
sang mệnh giá kế tiếp. Với 10 và mệnh giá 4, số lượng đó là 2, không phải
1.
::
:::

:::opt
Máy báo lỗi, vì hệ mệnh giá `[4, 3, 1]` không có mệnh giá nào chia hết
cho 10
::why
Gần đúng ở việc đúng là không mệnh giá nào trong `[4, 3, 1]` chia hết gọn
cho 10.

Chỗ lệch: y hệt bài trước — `//` không đòi chia hết, nó chỉ lấy phần
nguyên và để phần dư lại cho mệnh giá nhỏ hơn xử lý tiếp, tới mệnh giá 1
thì luôn dọn hết phần còn sót. Không có tổ hợp số tiền nguyên nào khiến
đoạn mã này báo lỗi.
::
:::
::::

::::code{#chung-minh-bang-so}
Đo lại đúng ví dụ đã tính tay ở phần giải thích — trả 6 bằng hệ mệnh giá
giả `[4, 3, 1]` — và so sánh SỐ ĐỒNG tham lam chọn với số đồng của cách
tốt nhất đã biết trước (`[3, 3]`, hai đồng). Vòng lặp tham lam giữ
nguyên, đã viết sẵn — việc của bạn là điền phần SO SÁNH.

```python title=starter
menh_gia_gia = [4, 3, 1]
tien_con_thieu = 6
dong_tham_lam = []

for mg in menh_gia_gia:
    so_luong = tien_con_thieu // mg
    dong_tham_lam += [mg] * so_luong
    tien_con_thieu -= mg * so_luong

so_dong_tham_lam = len(dong_tham_lam)
so_dong_toi_uu = ___                  # đã biết trước: 3 + 3 = 6, hai đồng

print(f"Tham lam chọn: {dong_tham_lam} — {so_dong_tham_lam} đồng")
print(f"Cách tốt nhất: [3, 3] — {so_dong_toi_uu} đồng")
print(f"Tham lam có tối ưu không? {___}")
```

```python title=solution
menh_gia_gia = [4, 3, 1]
tien_con_thieu = 6
dong_tham_lam = []

for mg in menh_gia_gia:
    so_luong = tien_con_thieu // mg
    dong_tham_lam += [mg] * so_luong
    tien_con_thieu -= mg * so_luong

so_dong_tham_lam = len(dong_tham_lam)
so_dong_toi_uu = 2

print(f"Tham lam chọn: {dong_tham_lam} — {so_dong_tham_lam} đồng")
print(f"Cách tốt nhất: [3, 3] — {so_dong_toi_uu} đồng")
print(f"Tham lam có tối ưu không? {so_dong_tham_lam == so_dong_toi_uu}")
```

```python title=test
assert dong_tham_lam == [4, 1, 1], f"tham lam trên hệ [4, 3, 1] cho 6 phải chọn [4, 1, 1] — đang ra {dong_tham_lam}"
assert so_dong_toi_uu == 2, f"cách tốt nhất dùng đúng 2 đồng (3 + 3 = 6) — đang ra {so_dong_toi_uu}"
assert so_dong_tham_lam > so_dong_toi_uu, "tham lam phải dùng NHIỀU đồng hơn cách tối ưu ở hệ mệnh giá giả này — đây chính là điều bài muốn chứng minh"
```

:::hints
- kind: attention
  body: Chỗ trống 1 là một CON SỐ đã biết trước (không cần tính lại bằng vòng lặp nào). Chỗ trống 2 là một PHÉP SO SÁNH giữa hai con số đã có sẵn ở trên — không phải một câu văn.
- kind: strategy
  body: '3 + 3 = 6, đúng bằng tien_con_thieu, và dùng hai đồng — nên so_dong_toi_uu = 2. Câu hỏi "tham lam có tối ưu không" là một phép so sánh BẰNG giữa so_dong_tham_lam và so_dong_toi_uu — dùng == chứ không phải =.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là 2 và so_dong_tham_lam == so_dong_toi_uu.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống 2 phải THẬT SỰ so sánh so_dong_tham_lam với so_dong_toi_uu bằng == — không phải một câu không làm gì như True, 1, 0, và không phải chỉ nhắc tới một trong hai cái tên đó
  requireAst:
  - kind: uses-name, target: so_dong_tham_lam, min: 2
  - kind: uses-name, target: so_dong_toi_uu, min: 2
  - kind: uses-operator, target: "==", min: 1
  # min: 2 cho mỗi tên — đếm thật trên solution: mỗi tên xuất hiện đúng 1
  # lần trong dòng print có sẵn trong khung ("Tham lam chọn:... {so_dong_
  # tham_lam}..." và "Cách tốt nhất:... {so_dong_toi_uu}..."), cộng đúng 1
  # lần nữa trong chỗ trống 2 (so_dong_tham_lam == so_dong_toi_uu) = 2 mỗi
  # tên. Điền True/1/0 vào chỗ trống 2 chỉ còn 1 lần mỗi tên (từ dòng print
  # có sẵn) — dưới 2, bắt được cả hai phía của phép so sánh, không chỉ một
  # phía (bài học từ T3.1/T3.2: một luật chỉ đếm MỘT tên tham gia so sánh
  # thì đổi tên phía kia vẫn lọt). ĐÃ THỬ THẬT: điền True/1/0 vào CẢ HAI
  # chỗ trống — an toàn (không while/đệ quy nào cả), và cả ba cách đều
  # trượt tests ngay ở assert so_dong_toi_uu == 2 (True/1/0 không phải 2).
  # Cũng đã thử một lời giải ĐÚNG khác: so_dong_toi_uu == so_dong_tham_lam
  # (đảo vế) — qua đủ cả ba luật static, đúng tinh thần luật 2: không đặt
  # min theo đúng thứ tự viết của lời giải mẫu.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^Tham lam chọn: \\[4, 1, 1\\] — 3 đồng\\nCách tốt nhất: \\[3, 3\\] — 2 đồng\\nTham lam có tối ưu không\\? False\\s*$"
:::
::::

::::byte{trigger=success mood=thinking pose=lean-in}
Ba đồng, không phải hai — đo bằng số, không phải bằng niềm tin rằng "tham
lam chắc luôn đúng".
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài học hôm nay không phải "đừng dùng tham lam" — nó vẫn đúng ở hệ mệnh
giá thật bạn cầm mỗi ngày, và vẫn nhanh hơn hẳn việc thử mọi tổ hợp có
thể. Bài học là: **nhanh không có nghĩa luôn đúng** — phải tự kiểm chứng
bằng một ví dụ cụ thể, không tin theo cảm giác "hợp lý".

Có một chỗ khác trong track này cũng "chậm một cách không cần thiết" —
không phải vì nó chọn SAI như tham lam vừa rồi, mà vì nó tính ĐÚNG cùng
một câu hỏi quá nhiều lần. Còn nhớ `fib(6)` tính hết hai mươi lăm lượt gọi
chỉ để hỏi đi hỏi lại `fib(2)` và `fib(1)` không?

Nếu vấn đề lần này không phải "chọn nhầm bước", mà là "làm lại việc đã
làm", thì cách chữa có giống tham lam chữa được không? Bài sau quay lại
đúng chỗ đau đó, bằng một cách chữa khác hẳn.
::::

::::checkpoint{mastery=0.8}
::::
