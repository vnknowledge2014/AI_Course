---
id: nen-tang.re-nhanh-va-lap.dem-nhung-luot-dang-ke
title: Đếm những lượt đáng kể
summary: Một `if` đặt trong thân vòng lặp, và cái vạch đếm đặt trong nhánh đúng — chỉ những lượt lọt qua câu hỏi mới được tính.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.counter-if]
requires: [core.augmented-assign, ctrl.for-each, ctrl.if, ctrl.block-indent, core.reassign, core.fstring]
concepts: [core.bien, ctrl.lap, ctrl.re-nhanh, core.khoi-lenh]
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
Mình vẫn đi qua đủ sáu ngày. Chỉ là không phải ngày nào cũng được ghi vạch.
::::

::::explain{#khong-phai-luot-nao-cung-tinh}
Sổ chi tiêu sáu ngày của Byte:

```python
chi_tieu = [45000, 120000, 30000, 260000, 85000, 210000]
```

Câu hỏi *"cả tuần tiêu hết bao nhiêu"* thì bạn cộng dồn: một cái tên sinh ra
trước vòng, mỗi lượt cộng thêm `tien` vào nó, xong vòng thì đọc ra `750000`.
Mọi lượt đều góp phần, không lượt nào bị bỏ.

Hôm nay bà chủ quán hỏi câu khác:

> **Bao nhiêu ngày** tiêu quá 200 nghìn?

Vẫn phải đi qua đủ sáu ngày — không xem thì không biết ngày nào vượt. Nhưng lần
này hai chuyện đổi hẳn.

**Chuyện thứ nhất: không phải lượt nào cũng được tính.** Ngày 45 nghìn thì không
góp gì cả. Ngày 260 nghìn thì đáng một vạch. Bạn đã có sẵn công cụ để hỏi một
câu và chỉ chạy tiếp khi câu ấy đúng — đó là `if`. Chỗ mới là bạn đặt nó **bên
trong thân vòng lặp**, để câu hỏi ấy được hỏi lại một lần cho mỗi ngày.

**Chuyện thứ hai: thứ cộng vào không còn là số tiền.** Bà chủ hỏi *bao nhiêu
ngày*, chứ không hỏi *bao nhiêu đồng*. Ngày 260 nghìn và ngày 210 nghìn, dù
lệch nhau 50 nghìn, vẫn đáng đúng một vạch như nhau. Nên dòng cộng dồn viết là:

```python
dem += 1
```

Số `1` ấy chính là cái vạch. Cộng `1` thì `dem` đếm **lượt**; cộng `tien` thì
`dem` lại quay về đếm tiền — hai câu trả lời cho hai câu hỏi khác nhau.

Người bán hàng ngoài chợ làm đúng như vậy khi kiểm hàng: nhìn từng quả, quả nào
hỏng thì gạch một vạch lên giấy. Vạch nào cũng dài bằng nhau, vì mỗi vạch chỉ có
nghĩa là *một quả*.

Ghép lại thành ba tầng thụt lề:

```python
dem = 0                        # tầng 0 — chạy một lần, trước khi đi
for tien in chi_tieu:          # tầng 1 — thân vòng, chạy mỗi ngày
    if tien > 200000:          # tầng 2 — nhánh đúng, chỉ chạy ngày vượt ngưỡng
        dem += 1
```

Tên riêng của cái tên `dem` là **biến đếm** (tiếng Anh: *counter*).
::::

::::example{#hai-tang-lam-hai-viec}
Byte cho in ra từng bước để bạn nhìn thấy hai tầng đang làm hai việc khác nhau:

```python title=readonly
chi_tieu = [45000, 120000, 30000, 260000, 85000, 210000]
dem = 0

for tien in chi_tieu:
    print(f"Xem ngày tiêu {tien} đồng")
    if tien > 200000:
        dem += 1
        print("   → vượt ngưỡng, ghi một vạch")

print(f"Có {dem} ngày tiêu quá 200 nghìn")
```

Màn hình hiện ra:

```text title=readonly
Xem ngày tiêu 45000 đồng
Xem ngày tiêu 120000 đồng
Xem ngày tiêu 30000 đồng
Xem ngày tiêu 260000 đồng
   → vượt ngưỡng, ghi một vạch
Xem ngày tiêu 85000 đồng
Xem ngày tiêu 210000 đồng
   → vượt ngưỡng, ghi một vạch
Có 2 ngày tiêu quá 200 nghìn
```

Đếm số dòng là thấy ngay câu chuyện: dòng *Xem ngày* hiện **sáu** lần, dòng *ghi
một vạch* hiện **hai** lần. Cùng nằm trong một vòng lặp, mà chạy số lần khác
nhau — khác nhau đúng ở mức thụt lề.

- `print("Xem ngày ...")` thụt vào **bốn** dấu cách. Nó thuộc thân vòng, nên nó
  chạy mọi lượt.
- `dem += 1` thụt vào **tám** dấu cách. Nó thuộc nhánh đúng của `if`, nên nó chỉ
  chạy ở những lượt câu hỏi trả lời đúng.

Còn `dem = 0` và `print` cuối cùng viết sát lề trái — cả hai nằm ngoài vòng, mỗi
cái chạy đúng một lần. Một cái mở sổ, một cái đọc kết quả.
::::

::::predict{#dem-lech-mot-tang commitOnce}
Byte gõ vội và để dòng đếm thụt vào ít hơn một tầng. **Trước khi bấm chạy**, bạn
đoán dòng cuối cùng in ra con số nào?

```python title=readonly
chi_tieu = [45000, 120000, 30000, 260000, 85000, 210000]
dem = 0

for tien in chi_tieu:
    if tien > 200000:
        print("vượt ngưỡng")
    dem += 1

print(dem)
```

:::opt{correct}
6
:::

:::opt
2
::why
Gần đúng ở chỗ bạn đọc ra chính xác ý người viết đoạn này: họ muốn đếm những
ngày vượt 200 nghìn, và trong sổ đúng là có hai ngày như thế. Con số 2 là câu
trả lời cho **câu hỏi** — nó chỉ không phải câu trả lời cho **đoạn code này**.

Chỗ lệch nằm ở cột thụt lề của dòng `dem += 1`. Nó thụt vào bốn dấu cách, ngang
hàng với chữ `if`, chứ không phải tám dấu cách như dòng `print` phía trên. Ngang
hàng với `if` nghĩa là nó thuộc **thân vòng lặp**, không thuộc nhánh đúng của
`if`. Thân vòng chạy đủ sáu lượt, nên cái vạch được ghi sáu lần.
::
:::

:::opt
4
::why
Gần đúng ở chỗ bạn nhìn ra dòng `dem += 1` **không** nằm trong nhánh đúng của
`if` — nó thụt ít hơn dòng `print` một tầng. Nhận xét đó chính xác, và nó là
nửa quan trọng của bài này.

Chỗ lệch là nửa còn lại: một dòng lùi ra khỏi khối `if` không tự trở thành phần
`else`. `else` phải được viết ra bằng đúng chữ `else`, kèm dấu hai chấm. Ở đây
không có chữ ấy, nên `dem += 1` chỉ đơn giản là dòng tiếp theo của thân vòng —
chạy ở **mọi** lượt, cả bốn ngày dưới ngưỡng lẫn hai ngày trên ngưỡng.
::
:::

:::opt
0
::why
Gần đúng ở chỗ bạn để ý thấy `dem = 0` được viết ở ngoài vòng còn `dem += 1`
được viết ở trong vòng, và bạn đặt câu hỏi hai chỗ ấy có phải cùng một cái tên
không. Đó là câu hỏi đáng hỏi.

Chỗ lệch: trong Python, thân vòng lặp không phải một căn phòng riêng. Cái tên
`dem` bên trong vòng chính là cái tên `dem` bên ngoài — mỗi lượt sửa vào đúng nó,
và giá trị sửa được vẫn còn nguyên sau khi vòng kết thúc. Đó cũng chính là lý do
biến đếm phải sinh ra **trước** vòng: nếu nó là căn phòng riêng, không cách nào
mang con số ra ngoài để in.
::
:::
::::

::::explain{#mot-cai-vach-dem-duoc-nhieu-thu}
Cái khuôn ba tầng ở trên không dính gì tới tiền. Đổi câu hỏi trong `if` là nó
đếm sang chuyện khác ngay, mà không phải sửa gì thêm:

- `if tien == 0:` — bao nhiêu ngày không tiêu đồng nào
- `if tien < 50000:` — bao nhiêu ngày tiêu dưới 50 nghìn
- `if tien > 200000:` — bao nhiêu ngày vượt ngưỡng, như bài này

Và có một con số bạn nên biết trước khi gặp nó: nếu **không lượt nào** lọt qua
câu hỏi, `dem` vẫn có giá trị — nó giữ nguyên con số `0` bạn đã đặt trước vòng.
Không có lỗi nào cả. Câu `Có 0 ngày tiêu quá 200 nghìn` là một câu trả lời đúng
và đầy đủ, chứ không phải dấu hiệu chương trình hỏng.
::::

::::code{#dem-ngay-vuot-nguong}
Sổ chi tiêu sáu ngày đã nằm sẵn, và biến đếm đã sinh ra trước vòng. Trong thân
vòng còn hai chỗ trống: chỗ đặt **câu hỏi**, và chỗ đặt **cái vạch**.

Bà chủ quán muốn biết có bao nhiêu ngày tiêu **quá 200 nghìn**.

```python title=starter
chi_tieu = [45000, 120000, 30000, 260000, 85000, 210000]
dem = 0

for tien in chi_tieu:
    if ___:
        ___

print(f"Có {dem} ngày tiêu quá 200 nghìn")
```

```python title=solution
chi_tieu = [45000, 120000, 30000, 260000, 85000, 210000]
dem = 0

for tien in chi_tieu:
    if tien > 200000:
        dem += 1

print(f"Có {dem} ngày tiêu quá 200 nghìn")
```

```python title=test
# Đếm LƯỢT chứ không đếm TIỀN: cộng `tien` thay vì cộng 1 sẽ cho ra 470000.
assert dem < 10, f"dem đang là {dem} — nó đang giữ số tiền chứ không giữ số ngày"
# Byte dò tay: 260000 và 210000 là hai ngày vượt 200 nghìn, vậy `dem` phải là 2.
# Đếm mọi lượt thì ra 6; không lượt nào được tính thì ra 0.
assert dem == 2, f"dem đang là {dem}, phải là 2"
# Sổ chi tiêu chỉ để đọc — vòng lặp không được sửa nó.
assert chi_tieu == [45000, 120000, 30000, 260000, 85000, 210000]
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm giữa chữ `if` và dấu hai chấm — đó là câu hỏi hỏi về `tien` của ngày đang xem. Chỗ trống thứ hai thụt vào sâu hơn một tầng, nên nó chỉ chạy ở những ngày câu hỏi trả lời đúng.
- kind: strategy
  body: Câu hỏi so số tiền của ngày này với ngưỡng 200 nghìn — "quá" nghĩa là lớn hơn hẳn. Còn dòng bên dưới cộng thêm đúng một cái vạch vào `dem`, không cộng số tiền.
- kind: one-line
  body: "`if tien > 200000:` ở dòng trên, `dem += 1` ở dòng dưới."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Có 2 ngày tiêu quá 200 nghìn
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu ngày mình xem hết, mà chỉ ghi hai vạch. Đếm là như vậy đó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa có hai cách gom một sổ chi tiêu thành một con số: cộng **số tiền** ở mọi
lượt, và cộng **một cái vạch** ở những lượt đáng kể. Cả hai đều bắt đầu từ 0 và
lớn dần lên.

Bà chủ quán hỏi câu thứ ba:

> Cả tuần, ngày tiêu nhiều nhất tiêu hết bao nhiêu?

Thử cộng dồn xem: cộng hết thì ra 750 nghìn — đó là cả tuần, không phải một
ngày. Cộng những ngày vượt ngưỡng thì ra 470 nghìn — vẫn là hai ngày gộp lại.
Đếm vạch thì ra 2 — đó là số ngày, không phải số tiền.

Không phép cộng nào trả lời nổi, và lý do nằm ở chính chữ *cộng*. Con số bạn cần
là `260000` — nó **đã nằm sẵn** trong sổ, ở đúng một ngày. Việc phải làm không
phải là cộng thêm vào cái tên, mà là **giữ lại** một con số và bỏ đi những con
số kém hơn.

Một cái tên chỉ nhớ được một giá trị. Đi tới ngày sau, làm sao biết có nên thay
con số đang giữ hay không? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
