---
id: toan.cam-nhan-so.cat-mang-thanh-mieng
title: Cắt mảng thành miếng
summary: Cắt một cạnh của mảng chữ nhật thì phép nhân tách theo — `7 × 13` thành `(7 × 10) + (7 × 3)`, và cắt ở đâu cũng ra đúng chừng ấy cây.
locale: vi
track: toan
module: cam-nhan-so
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.multiply-distributive]
requires: [math.multiply-commutative, math.multiplication, core.arithmetic, core.variable, core.print-variable, core.output]
concepts: [math.mo-hinh-vung, math.tinh-chat-phan-phoi]
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
Sợi dây căng ngang mảng đất. Không cây nào rơi ra ngoài đâu.
::::

::::explain{#cang-day-qua-mang}
Bài trước để lại một mảng 7 hàng, mỗi hàng 13 cây, và một câu hỏi: cắt nó thành
hai miếng thì có mất cây nào không?

Đây là cái mảng, và sợi dây căng dọc giữa cột thứ 10 và cột thứ 11:

```text
      ← 10 cột →   ← 3 cột →
      ● ● ● ● ● ● ● ● ● ●│● ● ●
      ● ● ● ● ● ● ● ● ● ●│● ● ●
      ● ● ● ● ● ● ● ● ● ●│● ● ●
7     ● ● ● ● ● ● ● ● ● ●│● ● ●
hàng  ● ● ● ● ● ● ● ● ● ●│● ● ●
      ● ● ● ● ● ● ● ● ● ●│● ● ●
      ● ● ● ● ● ● ● ● ● ●│● ● ●
```

Sợi dây không nhổ cây nào lên, cũng không trồng thêm cây nào. Mỗi cây trong
vườn rơi vào **đúng một** miếng — không cây nào nằm cả hai bên, không cây nào
nằm ngoài cả hai. Bài 2 gọi chuyện đó là ghép một–một, và chính nó bảo đảm:
đếm miếng trái rồi đếm miếng phải rồi cộng lại thì ra đúng số cây ban đầu.

Bây giờ đếm từng miếng. Cả hai miếng đều vẫn còn đủ **7 hàng** — sợi dây cắt
dọc thì không đụng gì tới hàng:

- Miếng trái: 7 hàng × 10 cột.
- Miếng phải: 7 hàng × 3 cột.

Để ý thứ tự viết. Bài 19 viết cỡ lô trước — `8 × 5` là lô 8 cây, lấy 5 lượt. Từ
bài 20, hai chiều đọc cho ra đúng một con số, nên từ đây Byte viết theo thứ tự
nhìn vào mảng cho tiện: **số hàng trước, số cột sau**. `7 × 13` là 7 hàng, mỗi
hàng 13 cây. Đổi được là nhờ cái mảng xoay một góc ở bài trước — không phải vì
hai vai đã hết khác nhau.

Viết ra:

```text
7 × 13  =  (7 × 10) + (7 × 3)
        =     70    +   21
        =           91
```

Hai dấu ngoặc ở đó không phải để trang trí. Chúng nói thẳng ra cấu trúc: *có hai
miếng, mỗi miếng là một mảng, rồi mới gộp hai miếng lại*. Nhiều người viết dòng
ấy không cần ngoặc và vẫn ra 91 — vì sao bỏ ngoặc được mà không hỏng, bài 42 sẽ
trả lời. Trong bài này Byte để nguyên ngoặc, để bạn nhìn thấy hai cái miếng.

Và để ý miếng trái: `7 × 10` là **7 bó mười**. Cái bó ở bài 6 quay lại đúng chỗ
này — nó là miếng dễ đếm nhất trong mọi cách cắt.
::::

::::explain{#cat-cho-nao-cung-duoc}
Điều đáng giá của bài này không phải riêng phép cắt 13 = 10 + 3. Nó là: **cắt ở
đâu cũng được**, miễn cắt trọn một đường.

Cắt 13 cột thành 6 và 7:

```text
7 × 13  =  (7 × 6) + (7 × 7)  =  42 + 49  =  91
```

Hay quay sang cắt theo chiều kia — cắt 7 hàng thành 5 hàng và 2 hàng. Lúc này
hai miếng đều còn đủ 13 cột:

```text
7 × 13  =  (5 × 13) + (2 × 13)  =  65 + 26  =  91
```

Ba cách cắt, ba dòng chữ khác nhau, cùng 91 cây. Vẫn cùng một lý do: sợi dây
không thêm bớt cây nào.

Còn một chỗ phải nhớ, và đây là chỗ hay sập nhất: cắt **một** cạnh thì cạnh kia
**đi theo cả hai miếng**. Cắt 13 cột thành 10 và 3 thì cả hai miếng vẫn 7 hàng.
Cắt 7 hàng thành 5 và 2 thì cả hai miếng vẫn 13 cột. Con số không bị cắt là con
số phải viết lại đủ hai lần.

Tính chất này có tên trong sách: **tính chất phân phối** của phép nhân đối với
phép cộng. Cái tên đến sau; thứ bạn cần mang theo là sợi dây và hai cái miếng.
::::

::::example{#hoi-thang-cai-may}
Bắt máy chấm cả ba cách cắt cùng lúc:

```python title=readonly
print(7 * 13)
print((7 * 10) + (7 * 3))
print((7 * 6) + (7 * 7))
print((5 * 13) + (2 * 13))
```

Máy in ra:

```text
91
91
91
91
```

Bốn dòng, một con số. Bạn vừa nhân 7 với 13 mà không phải cộng 13 bảy lần —
`7 × 10` thì nhẩm ra ngay vì nó là 7 bó mười, `7 × 3` thì nhẩm ra ngay vì nó
nhỏ, và cộng hai cái đó thì bài 12 đã lo.
::::

::::predict{#doan-bon-dong commitOnce}
Byte đổi sang một mảng khác: **12 hàng, mỗi hàng 13 cây**. Byte thử cắt theo
mấy kiểu rồi in kết quả ra.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(12 * 13)
print((10 * 13) + (2 * 13))
print((10 * 10) + (2 * 3))
print((10 * 10) + (10 * 3) + (2 * 10) + (2 * 3))
```

:::opt{correct}
156 rồi 156 rồi 106 rồi 156
:::

:::opt
156 rồi 156 rồi 156 rồi 156
::why
Gần đúng ở chỗ bạn tin rằng cắt kiểu nào cũng ra đúng chừng ấy cây — và đó đúng
là điều bài này dạy.

Chỗ lệch: dòng ba **không phải một phép cắt**. Nó cắt 12 hàng thành 10 và 2,
rồi cắt luôn 13 cột thành 10 và 3. Cắt cả hai cạnh thì mảng vỡ thành **bốn**
miếng, không phải hai: `10 × 10`, `10 × 3`, `2 × 10` và `2 × 3`. Dòng ba chỉ
nhặt hai miếng ở hai góc chéo nhau rồi bỏ quên hai miếng còn lại — 30 cây và
20 cây. Nên nó ra 106, thiếu đúng 50 cây. Dòng bốn nhặt đủ bốn miếng và về lại
156.
::
:::

:::opt
156 rồi 156 rồi 106 rồi 106
::why
Gần đúng ở chỗ bạn để ý hai dòng cuối cùng bắt đầu bằng `10 * 10`, nên chờ
chúng cùng thiếu như nhau. Quan sát ấy sắc: hai dòng đó đúng là xuất phát từ
cùng một **kiểu** cắt — cắt cả hai cạnh.

Mà cắt cả hai cạnh thì mảng vỡ thành **bốn** miếng, nên chỉ khi nhặt đủ bốn
miếng thì mới thành một phép cắt trọn vẹn. Chỗ lệch nằm ở phần đuôi: dòng ba
dừng lại sau hai miếng góc nên còn thiếu; dòng bốn viết tiếp
`+ (10 * 3) + (2 * 10)` — hai miếng bị bỏ quên, 30 cây và 20 cây — nên
`100 + 30 + 20 + 6 = 156`, bằng đúng mảng ban đầu.
::
:::

:::opt
156 rồi 106 rồi 156 rồi 156
::why
Gần đúng ở chỗ bạn nhớ chính xác rằng trong bốn dòng có một dòng ra thiếu, và
nhớ đúng cả con số 106.

Chỗ lệch là **dòng nào**. Dòng hai cắt đúng một cạnh: 12 hàng thành 10 hàng và
2 hàng. Cạnh còn lại — 13 cột — nó chép lại đủ cho cả hai miếng, nên không cây
nào rơi ra: `130 + 26 = 156`. Dòng ba mới là dòng cắt cả hai cạnh rồi chỉ nhặt
hai góc.
::
:::
::::

::::explain{#cat-de-nham-duoc}
Cách cắt hữu ích nhất là cắt theo cái bó ở bài 6: tách một cạnh thành **phần
chục và phần lẻ**.

- `8 × 14` → cắt 14 thành 10 + 4 → `(8 × 10) + (8 × 4)` = 80 + 32 = 112.
- `6 × 19` → cắt 19 thành 10 + 9 → `(6 × 10) + (6 × 9)` = 60 + 54 = 114.

Miếng đầu luôn dễ: nhân với 10 là đếm bằng bó, và bảng giá trị vị trí ở bài 7
đã lo hết. Miếng sau nhỏ, nhẩm được.

Đây chính là cái bạn thấy người bán hàng ngoài chợ làm trong đầu: "19 nghìn một
cân, 6 cân — 6 lần 20 là 120, hụt 6, còn 114". Họ cũng đang cắt mảng, chỉ khác
chỗ cắt.
::::

::::code{#dem-hai-manh-vuon}
Byte đo hai mảnh vườn, và cả hai đều đã được cắt sẵn theo phần chục. Miếng đầu
của mỗi mảnh đã viết xong; việc của bạn là viết **miếng còn lại**.

- **Mảnh hạt**: 8 hàng × 14 cột. Cắt 14 cột thành 10 cột và 4 cột.
- **Mảnh rau**: 6 hàng × 19 cột. Cắt 19 cột thành 10 cột và 9 cột.

Nhớ chỗ hay sập ở trên: sợi dây cắt dọc theo cột, nên miếng thứ hai vẫn còn đủ
số hàng như cũ. Hai mảnh có số hàng khác nhau và số cột lẻ khác nhau, nên không
chỗ trống nào chép được sang chỗ trống kia.

```python title=starter
manh_hat = (8 * 10) + ___
manh_rau = (6 * 10) + ___

print(manh_hat)
print(manh_rau)
```

```python title=solution
manh_hat = (8 * 10) + (8 * 4)
manh_rau = (6 * 10) + (6 * 9)

print(manh_hat)
print(manh_rau)
```

```python title=test
# Chấm bằng hai mảnh, và mỗi mảnh được đối chiếu với phép nhân chưa cắt của
# chính nó. Cắt đúng thì hai vế bằng nhau; quên chép lại số hàng cho miếng thứ
# hai thì lệch ngay, và lệch khác nhau ở hai mảnh.
assert manh_hat == 8 * 14, "cắt 14 cột thành 10 và 4 phải ra đúng bằng mảng 8 × 14 chưa cắt"
assert manh_rau == 6 * 19, "cắt 19 cột thành 10 và 9 phải ra đúng bằng mảng 6 × 19 chưa cắt"
assert manh_hat == 112 and manh_rau == 114, "112 cây và 114 cây"
assert 8 * 14 != 6 * 19, "hai mảnh không cùng số cây — một đáp án chép cho cả hai là không được"
```

:::hints
- kind: attention
  body: Miếng còn lại cũng là một mảng chữ nhật. Nó rộng mấy cột thì đề đã nói rồi — còn thiếu con số kia: nó có mấy **hàng**?
- kind: strategy
  body: Sợi dây cắt dọc theo cột thì không đụng vào hàng nào, nên cả hai miếng giữ nguyên số hàng của mảnh đó. Mảnh hạt 8 hàng, miếng còn lại 4 cột; mảnh rau 6 hàng, miếng còn lại 9 cột.
- kind: one-line
  body: "Viết `(8 * 4)` vào chỗ trống thứ nhất và `(6 * 9)` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một phép nhân — miếng còn lại vẫn là một mảng hàng × cột, không phải một con số trần
  requireAst:
  - kind: uses-operator, target: *, min: 4
  - kind: uses-operator, target: +, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^112\n114\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
112 với 114. Cắt xong đếm hai miếng, vẫn đủ từng ấy cây.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa nhân 7 với 13 mà không phải cộng 13 bảy lần, và làm được thế là nhờ cắt
mảng ra chỗ có cái bó mười của bài 6. Cái cột "10" trong phép cắt không phải một
con số Byte chọn bừa — nó đúng là cái bó bạn đã đóng từ bài 6, và cái cột bạn đã
viết từ bài 7.

Nhưng đọc câu này xem: **"luống rau nhà An dài gấp 3 luống nhà Byte."**

Ở đây cũng có chữ "nhân", cũng có con số 3. Chỉ có điều không có cây nào để
đếm, không có hàng, không có cột. Chỉ có một sợi dây dài, và một sợi dài gấp ba.

Cái mảng chữ nhật đã đưa bạn đi qua hai bài, nhưng ở câu này thì **đâu là hàng,
đâu là cột**? Nếu không chỉ ra được, thì phép nhân còn một bức tranh thứ hai mà
bạn chưa có — và bức tranh ấy vẽ trên cái gì?

Bài sau lấy lại thanh số ở bài 5.
::::

::::checkpoint{mastery=0.8}
::::
