---
id: nen-tang.re-nhanh-va-lap.mot-ten-o-ca-hai-ben
title: Một cái tên ở cả hai bên dấu `=`
summary: Máy tính xong vế phải bằng giá trị cũ rồi mới dán cái tên lên kết quả — đọc, sửa, ghi.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.read-modify-write]
requires: [core.reassign, core.variable, ctrl.while, ctrl.loop-progress, core.fstring, err.name-error]
concepts: [core.bien, core.gan-lai]
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
Vế phải xong xuôi mình mới đụng tới cái tên. Không bao giờ ngược lại.
::::

::::explain{#dau-bang-khong-phai-dau-bang}
Bài trước để lại một dòng trông vô lý:

```python
thang = thang + 1
```

Đọc như toán thì nó vô nghiệm. Nhưng trong Python, dấu `=` không tuyên bố hai
vế bằng nhau. Nó là một **mệnh lệnh**, và mệnh lệnh ấy có thứ tự:

1. **Tính xong vế phải trước.** Vế trái lúc này chưa được đụng tới.
2. **Rồi mới dán cái tên bên trái lên kết quả vừa tính.**

Đó là quy tắc bạn đã dùng từ Realm 0 mỗi lần viết `tong = a + b`. Chuyện mới ở
đây chỉ là: cái tên bên trái được phép **cũng có mặt** ở vế phải.

Và một khi đã theo đúng thứ tự trên thì chuyện đó chẳng có gì rắc rối. Lúc máy
tính vế phải, `thang` vẫn đang mang giá trị cũ — cái tên chưa bị dán đi đâu cả.
Máy lấy giá trị cũ ấy cộng 1, được một con số, rồi mới dán `thang` lên con số
mới. Con số cũ mất chỗ dựa và biến khỏi cuộc chơi.

Ba việc, theo thứ tự, và giới lập trình gọi tên chúng là **đọc – sửa – ghi**
(tiếng Anh: *read–modify–write*):

- **đọc** — lấy giá trị cái tên đang mang;
- **sửa** — làm phép tính trên giá trị vừa lấy;
- **ghi** — dán cái tên lên kết quả.

Nghĩ tới tấm bảng giá treo trước quán phở. Bà chủ muốn tăng giá 5 nghìn: bà
**nhìn** con số đang viết trên bảng (45.000), **cộng** nhẩm trong đầu ra
50.000, rồi **xoá** đi viết lại. Suốt lúc bà nhẩm, con số cũ vẫn còn nguyên
trên bảng — nếu nó biến mất trước khi bà cộng xong thì bà chẳng biết cộng vào
đâu. Và tấm bảng không bao giờ mang hai con số cùng lúc: hết 45.000 thì tới
50.000, không có khoảnh khắc nào ở giữa.

Một hệ quả rơi thẳng ra từ chữ **đọc**: cái tên phải **đã mang một giá trị**
trước khi bạn viết dòng ấy. Chưa có gì để đọc thì cả ba việc không bắt đầu
được, và máy nói ra bằng `NameError` — đúng cái tên lỗi bạn đã gặp ở Realm 0.
Đó là lý do bài trước phải viết `thang = 0` ở đâu đó trước vòng lặp.
::::

::::example{#xem-tung-viec-mot}
Byte cầm 100 nghìn, ăn một tô phở 45 nghìn:

```python title=readonly
tien_trong_vi = 100000
tien_trong_vi = tien_trong_vi - 45000
print(tien_trong_vi)
```

Máy in ra:

```text
55000
```

Mổ dòng thứ hai ra làm ba việc, theo đúng thứ tự máy làm:

- **đọc** — máy đi tìm cái tên `tien_trong_vi` ở vế phải và lấy về `100000`.
  Ngay lúc này, cái tên vẫn đang dán trên `100000`.
- **sửa** — `100000 - 45000` cho ra `55000`. Con số này chưa có tên, nó mới chỉ
  nằm trong tay máy.
- **ghi** — máy gỡ cái tên `tien_trong_vi` khỏi `100000` và dán lên `55000`.

Sau dòng ấy, `100000` không còn cách nào gọi tới được nữa. Một cái tên chỉ dán
lên đúng một giá trị tại một thời điểm.

Và đây là chỗ dòng ấy khác hẳn một phương trình: **nó không phải một lời tuyên
bố về sự thật, nó là một việc phải làm**. Chạy hai lần thì làm hai lần, mỗi lần
đọc lại con số mới nhất:

```python title=readonly
tien_trong_vi = 100000
tien_trong_vi = tien_trong_vi - 45000
tien_trong_vi = tien_trong_vi - 45000
print(tien_trong_vi)
```

```text
10000
```

Hai dòng giữa gõ giống hệt nhau nhưng cho ra hai kết quả khác nhau — 55000 rồi
10000 — vì mỗi lần "đọc" gặp một con số khác. Đó chính là lý do một dòng duy
nhất nằm trong thân vòng lặp làm được việc của cả một dãy phép trừ.
::::

::::predict{#me-thuong-gap-doi commitOnce}
Mẹ hứa với Byte: cuối mỗi tháng, con để dành được bao nhiêu mẹ cho thêm đúng
bấy nhiêu. Trong heo đất đang có 50 nghìn, và Byte đã qua hai tháng.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
heo_dat = 50000
heo_dat = heo_dat + heo_dat
heo_dat = heo_dat + heo_dat
print(heo_dat)
```

:::opt{correct}
200000
:::

:::opt
100000
::why
Gần đúng ở chỗ bạn tính đúng dòng thứ hai: `50000 + 50000` ra `100000`, không
sai một đồng.

Chỗ lệch nằm ở dòng thứ ba. Bạn đọc nó bằng con số gốc `50000` một lần nữa —
như thể `heo_dat` mãi mãi là cái tên của 50 nghìn. Nhưng việc **ghi** ở cuối
dòng hai đã dán cái tên ấy sang `100000` rồi. Nên việc **đọc** ở dòng ba nhặt
được `100000`, và `100000 + 100000` cho ra `200000`.
::
:::

:::opt
50000
::why
Gần đúng ở chỗ bạn đọc `heo_dat = heo_dat + heo_dat` như một dòng nói lại chính
nó: vế trái và vế phải cùng một cái tên, mà một câu nói lại chính nó thì không
mang tin gì mới. Theo cách đọc ấy, con số đầu tiên phải còn nguyên tới cuối.

Chỗ lệch: `heo_dat + heo_dat` không phải "chính nó", nó là một **phép cộng** và
mọi phép cộng đều cho ra một con số mới. Việc ghi ở cuối dòng dán cái tên lên
con số mới đó. Dòng ấy làm việc thật, và làm hai lần.
::
:::

:::opt
Máy báo lỗi, vì một cái tên không được đứng cả hai bên dấu `=`
::why
Gần đúng ở chỗ bạn nhận ra dòng này khác hẳn một dòng gán thông thường, và bạn
nhớ đúng rằng Python hay dừng lại khi gặp thứ nó không hiểu.

Chỗ lệch nằm ở thứ tự. Máy tính **xong** vế phải rồi mới đụng tới vế trái, nên
tại thời điểm nó đọc `heo_dat` ở vế phải, cái tên ấy vẫn đang mang giá trị cũ
đàng hoàng. Không có mâu thuẫn nào để báo. Trường hợp duy nhất dòng này hỏng là
khi cái tên **chưa từng** mang giá trị nào — lúc đó việc đọc thất bại và máy
báo `NameError`.
::
:::
::::

::::code{#dem-mon-di-cho}
Sáng nay Byte cầm 100 nghìn ra chợ. Mỗi món hàng đúng 25 nghìn, và Byte cứ mua
chừng nào còn đủ tiền cho một món nữa.

Vòng lặp đã có bước tiến rồi — `vi` vơi đi sau mỗi món. Còn thiếu người **đếm**
số món. Hãy điền dòng làm cho `mon` lớn thêm một sau mỗi lượt.

```python title=starter
vi = 100000
mon = 0

while vi >= 25000:
    vi = vi - 25000
    ___
    print(f"Mua món thứ {mon}, ví còn {vi} đồng")

print(f"Cả buổi chợ mua được {mon} món")
```

```python title=solution
vi = 100000
mon = 0

while vi >= 25000:
    vi = vi - 25000
    mon = mon + 1
    print(f"Mua món thứ {mon}, ví còn {vi} đồng")

print(f"Cả buổi chợ mua được {mon} món")
```

```python title=test
# 100 nghìn chia cho 25 nghìn một món là bốn món, và ví phải sạch trơn.
# Kiểm cả hai: đếm đúng số món, và không ai lỡ tay sửa số tiền.
assert mon == 4
assert vi == 0
```

:::hints
- kind: attention
  body: Nhìn dòng ngay trên chỗ trống. Nó cũng là một cái tên đứng ở cả hai bên dấu `=`, và nó làm cho `vi` nhỏ đi 25 nghìn mỗi lượt. Chỗ trống cần làm điều tương tự với `mon`, chỉ khác hướng.
- kind: strategy
  body: Máy tính vế phải trước, nên `mon` ở vế phải là số món đếm được **tính tới lượt trước**. Cộng thêm đúng một món của lượt này, rồi dán lại cái tên `mon` lên kết quả. Dòng in ngay bên dưới phải nói "Mua món thứ 1" ở lượt đầu, nên phép cộng phải xảy ra **trước** khi in.
- kind: one-line
  body: Viết `mon = mon + 1` vào chỗ trống, thụt vào bốn dấu cách cho thẳng hàng với dòng ngay trên nó.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Mua món thứ 1, ví còn 75000 đồng
- tier: output
  expect: Cả buổi chợ mua được 4 món
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn món. Mỗi lượt mình đọc con số cũ, cộng một, rồi dán tên lên con số mới.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`mon = mon + 1` cộng thêm **1** ở mỗi lượt, nên cuối cùng `mon` mang số lượt đã
chạy. Bốn lượt thì `mon` là 4. Cái tên ấy đang làm một cái máy đếm.

Nhưng con số `1` trong dòng đó không có gì thiêng liêng. Nó chỉ là vế phải của
một phép cộng, và vế phải thì thay được bằng bất cứ thứ gì cho ra một con số.

Sổ chi tiêu của Byte có sáu ngày, mỗi ngày một số tiền khác nhau: 45 nghìn, rồi
120 nghìn, rồi 30 nghìn… Bây giờ hình dung một vòng lặp đi qua sáu ngày ấy, và
trong thân, thay vì cộng `1`, bạn cộng **số tiền tiêu của chính ngày đó**:

```python
tong = tong + tien
```

Sau sáu lượt, cái tên `tong` đang giữ cái gì? Và trước lượt đầu tiên, nó phải
mang sẵn con số nào — nhớ rằng việc **đọc** cần có thứ để đọc?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
