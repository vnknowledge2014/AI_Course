---
id: nen-tang.re-nhanh-va-lap.boss-so-chi-tieu-cua-byte
title: BOSS — Sổ chi tiêu của Byte
summary: Một lượt duyệt cuốn sổ, năm câu trả lời — kèm nhánh cho tháng chẳng có ngày nào vượt ngưỡng.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
practices: [core.accumulator, core.compound-assign, core.counter-if, core.max-tracker, core.sentinel, ctrl.nested-if, ctrl.for-each, core.fstring]
requires: [core.accumulator, core.augmented-assign, core.max-tracker, ctrl.sentinel-value, ctrl.if-nested, ctrl.for-each]
concepts: [ctrl.lap, ctrl.re-nhanh, core.bien]
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
  reviewed: true
---

::::byte{trigger=enter mood=curious pose=lean-in}
Hôm nay không có gì mới. Chỉ có cuốn sổ, một lượt duyệt, và năm câu hỏi.
::::

::::explain{#nam-cau-hoi-mot-luot}
Bài trước kết bằng một lời cảnh báo hơn là một câu hỏi: năm con số đã ra được từ
một vòng duyệt, nhưng đoạn vòng ấy rồi sẽ phải chép lại cho tháng sau. Chuyện
chép lại để dành tới cuối bài. Trước hết, viết cho xong một lần đã.

Byte đưa bạn cuốn sổ chi tiêu của mình. Mỗi ngày một con số, ghi theo thứ tự
ngày, không ghi kèm ngày tháng — ngày thứ mấy thì đếm theo chỗ đứng trong sổ.
Cuối kỳ Byte muốn năm câu trả lời:

1. Cả kỳ tiêu hết bao nhiêu?
2. Trung bình mỗi ngày bao nhiêu?
3. Bao nhiêu ngày tiêu quá 200 nghìn?
4. Ngày nào tiêu nhiều nhất, và nhiều bao nhiêu?
5. Ngày **đầu tiên** vượt ngưỡng là ngày nào?

Năm câu hỏi, một cuốn sổ. Bạn đã có đủ đồ nghề cho từng câu, và cả năm câu đều
xong trong **một** lượt duyệt — vì mỗi câu chỉ cần một cái tên riêng đứng ngoài
vòng, làm nhiệm vụ nhớ giúp:

- `tong` — cộng dồn mọi lượt, sinh ra từ `0`.
- `ngay` — cũng cộng dồn mọi lượt, nhưng mỗi lượt `+= 1`, nên nó chính là số thứ
  tự ngày. Chia `tong` cho nó là ra trung bình.
- `so_ngay_vuot` — chỉ cộng ở những lượt vượt ngưỡng.
- `tien_ky_luc` và `ngay_ky_luc` — so sánh rồi mới thay thế, đi thành một cặp.
- `ngay_dau_vuot` — giá trị canh `-1` nghĩa là "chưa gặp ngày nào".

Cả sáu cái tên đều theo đúng một nhịp ba: **sinh ra trước vòng**, **được chạm
trong vòng**, **đọc lại sau vòng**.
::::

::::example{#ca-cuon-so}
Bắt đầu bằng một cuốn sổ ngắn: năm ngày đầu tháng. Ngắn để bạn dò được bằng mắt.

```python title=readonly
so_chi_tieu = [85000, 240000, 120000, 95000, 310000]
nguong = 200000

ngay = 0
tong = 0
so_ngay_vuot = 0
tien_ky_luc = 0
ngay_ky_luc = 0
ngay_dau_vuot = -1

for tien in so_chi_tieu:
    ngay += 1
    tong += tien
    if tien > nguong:
        so_ngay_vuot += 1
        if ngay_dau_vuot == -1:
            ngay_dau_vuot = ngay
    if tien > tien_ky_luc:
        tien_ky_luc = tien
        ngay_ky_luc = ngay

print("SỔ CHI TIÊU — năm ngày đầu tháng")
print(f"Tổng chi: {tong} đồng")
print(f"Trung bình mỗi ngày: {tong / ngay} đồng")
print(f"Số ngày tiêu quá {nguong} đồng: {so_ngay_vuot}")
print(f"Ngày tiêu nhiều nhất: ngày {ngay_ky_luc} — {tien_ky_luc} đồng")
if ngay_dau_vuot == -1:
    print("Cả năm ngày không có ngày nào vượt ngưỡng")
else:
    print(f"Ngày đầu tiên vượt ngưỡng: ngày {ngay_dau_vuot}")
```

Máy in ra:

```text title=readonly
SỔ CHI TIÊU — năm ngày đầu tháng
Tổng chi: 850000 đồng
Trung bình mỗi ngày: 170000.0 đồng
Số ngày tiêu quá 200000 đồng: 2
Ngày tiêu nhiều nhất: ngày 5 — 310000 đồng
Ngày đầu tiên vượt ngưỡng: ngày 2
```

Cả thân vòng chỉ có bảy dòng, và không dòng nào là thứ bạn chưa gặp.

Ba chỗ đáng dừng lại nhìn kỹ:

- **`ngay += 1` đứng ngay dòng đầu thân vòng.** Cuốn sổ chỉ chứa tiền, không
  chứa ngày. Cái tên `ngay` là thứ duy nhất biết lượt này là lượt thứ mấy, nên
  nó phải được đẩy lên **trước** mọi dòng có nhắc tới ngày.
- **`170000.0` chứ không phải `170000`.** Phép chia luôn cho ra số có phần lẻ,
  đúng như Realm 0 đã nói. Ở đây `850000 / 5` chia hết, nên phần lẻ là `.0` —
  máy vẫn ghi nó ra vì kiểu của kết quả là số thập phân.
- **Hai `if` trong thân vòng đứng ngang hàng nhau**, cùng thụt vào bốn dấu cách.
  Chúng là hai câu hỏi độc lập, hỏi trên cùng một ngày: *ngày này có vượt ngưỡng
  không* và *ngày này có phá kỷ lục không*. Một ngày trả lời "có" cho cả hai
  (ngày 5) thì cả hai thân đều chạy.
::::

::::explain{#vi-sao-khong-dung-else-cua-vong}
Câu hỏi số 5 — *ngày đầu tiên vượt ngưỡng* — nghe giống hệt bài tìm kiếm, và bài
tìm kiếm thì có sẵn hai công cụ gọn hơn: `break` để dừng ngay khi đủ biết, rồi
`else` của vòng lặp để lo nhánh "không lần nào gặp".

Ở đây cả hai đều không dùng được, và lý do nằm ở chữ **một lượt**.

`break` thoát khỏi vòng ngay tại chỗ. Nhưng vòng này còn đang gánh bốn câu hỏi
khác: dừng ở ngày 2 thì `tong` mới cộng được hai ngày, kỷ lục ngày 5 chưa ai
nhìn thấy. Bốn câu kia cần đi hết cuốn sổ, nên vòng không được phép dừng sớm.

Không có `break` thì `else` của vòng lặp cũng mất tác dụng: nó chạy khi vòng kết
thúc mà chưa lần nào gặp `break` — mà vòng này thì **lần nào cũng** kết thúc như
vậy. Viết `else` vào đây thì nhánh đó chạy cả khi có ngày vượt lẫn khi không, tức
là nó chẳng phân biệt được gì.

Nên việc "nhớ đã tìm thấy hay chưa" quay về cho giá trị canh: `ngay_dau_vuot`
sinh ra bằng `-1`, và câu `if ngay_dau_vuot == -1:` bên trong vòng chính là cái
cổng — chỉ lượt nào thấy nó vẫn còn `-1` mới được ghi vào. Đúng cái cổng ấy, viết
lại một lần nữa **sau** vòng, trả lời nốt câu "cả kỳ có ngày nào vượt không".
::::

::::predict{#doan-bo-mat-cong commitOnce}
Byte đưa nốt năm ngày còn lại của kỳ, và trong lúc chép lại đoạn code, Byte làm
rơi mất dòng canh cổng `if ngay_dau_vuot == -1:` — dòng gán thì vẫn còn nguyên.

**Trước khi bấm chạy**, bạn đoán dòng cuối in ra gì? (Ba ngày vượt ngưỡng 200000
là ngày 2, ngày 5 và ngày 8.)

```python title=readonly
so_chi_tieu = [85000, 240000, 120000, 95000, 310000,
               60000, 180000, 250000, 75000, 115000]
nguong = 200000

ngay = 0
ngay_dau_vuot = -1

for tien in so_chi_tieu:
    ngay += 1
    if tien > nguong:
        ngay_dau_vuot = ngay

print(f"Ngày đầu tiên vượt ngưỡng: ngày {ngay_dau_vuot}")
```

:::opt{correct}
Ngày đầu tiên vượt ngưỡng: ngày 8
:::

:::opt
Ngày đầu tiên vượt ngưỡng: ngày 2
::why
Gần đúng ở chỗ bạn theo dõi vòng lặp rất chuẩn tới ngày 2: đó là ngày đầu tiên
lọt qua `tien > nguong`, và `ngay_dau_vuot` được dán lên số 2 thật. Tới đó bạn
đọc không sai một chữ nào.

Chỗ lệch nằm ở sáu lượt sau. Ngày 5 và ngày 8 cũng lọt qua đúng điều kiện ấy, và
dấu `=` thì luôn dán lại — nó không hỏi cái tên đang giữ gì. Số 2 bị số 5 đè,
rồi số 5 bị số 8 đè. Cuối vòng, cái tên giữ ngày **cuối cùng** vượt ngưỡng.

Dòng canh cổng bị rơi mất chính là thứ duy nhất phân biệt "lần đầu" với "mọi
lần".
::
:::

:::opt
Ngày đầu tiên vượt ngưỡng: ngày -1
::why
Gần đúng ở chỗ bạn nhớ rằng `-1` là giá trị canh, và một biến canh còn nguyên
`-1` sau vòng nghĩa là chưa lần nào tìm thấy. Cách đọc đó chính xác.

Chỗ lệch là ở dòng nào đã bị rơi. Dòng rơi mất là dòng **kiểm** `if
ngay_dau_vuot == -1:`, còn dòng **gán** `ngay_dau_vuot = ngay` vẫn nằm nguyên
trong thân `if tien > nguong:`. Mất cổng thì lối vào rộng thêm chứ không hẹp
lại: giờ lượt nào vượt ngưỡng cũng ghi được vào.
::
:::

:::opt
Ngày đầu tiên vượt ngưỡng: ngày 3
::why
Gần đúng ở chỗ con số 3 có thật trong bài này: đúng ba ngày vượt ngưỡng, và bạn
đếm không sai.

Chỗ lệch là ý nghĩa của cái tên. `ngay_dau_vuot` không đếm — nó chỉ **giữ lại**
thứ mà `ngay` đang mang ở lượt gán gần nhất. Đếm là việc của một cái tên khác,
`so_ngay_vuot`, và cái tên đó không có mặt trong đoạn này. Hai loại biến trông
giống nhau vì cùng là số, nhưng một bên cộng thêm, một bên thay thế.
::
:::
::::

::::code{#giu-lai-ngay-ky-luc}
Ghép từng mảnh trước, ghép cả bản báo cáo sau. Mảnh đầu tiên là cặp kỷ lục.

Byte đưa **hai** cuốn sổ, mỗi cuốn năm ngày, và hỏi cùng một câu cho cả hai:
**ngày nào tiêu nhiều nhất**. Nên đoạn dưới có hai khối giống hệt nhau — cùng ba
cái tên sinh ra từ `0`, cùng một vòng duyệt, cùng hai dòng trong thân `if`. Chỗ
trống cũng chỉ có một kiểu, lặp lại hai lần: điền đúng cùng một điều kiện vào cả
hai chỗ.

Hai cuốn sổ cố ý khác nhau một điểm. Sổ A lập kỷ lục vào **ngày cuối**; sổ B lập
kỷ lục ngay **ngày 2** rồi bốn ngày sau không ngày nào phá nổi. Vì vậy một điều
kiện chỉ tình cờ đúng ở sổ A sẽ lộ ra ngay ở dòng in thứ hai.

```python title=starter
so_chi_tieu = [85000, 240000, 120000, 95000, 310000]

ngay = 0
tien_ky_luc = 0
ngay_ky_luc = 0

for tien in so_chi_tieu:
    ngay += 1
    if ___:
        tien_ky_luc = tien
        ngay_ky_luc = ngay

print(f"Sổ A — ngày {ngay_ky_luc} tiêu nhiều nhất: {tien_ky_luc} đồng")

so_chi_tieu = [70000, 330000, 150000, 90000, 120000]

ngay = 0
tien_ky_luc = 0
ngay_ky_luc = 0

for tien in so_chi_tieu:
    ngay += 1
    if ___:
        tien_ky_luc = tien
        ngay_ky_luc = ngay

print(f"Sổ B — ngày {ngay_ky_luc} tiêu nhiều nhất: {tien_ky_luc} đồng")
```

```python title=solution
so_chi_tieu = [85000, 240000, 120000, 95000, 310000]

ngay = 0
tien_ky_luc = 0
ngay_ky_luc = 0

for tien in so_chi_tieu:
    ngay += 1
    if tien > tien_ky_luc:
        tien_ky_luc = tien
        ngay_ky_luc = ngay

print(f"Sổ A — ngày {ngay_ky_luc} tiêu nhiều nhất: {tien_ky_luc} đồng")

so_chi_tieu = [70000, 330000, 150000, 90000, 120000]

ngay = 0
tien_ky_luc = 0
ngay_ky_luc = 0

for tien in so_chi_tieu:
    ngay += 1
    if tien > tien_ky_luc:
        tien_ky_luc = tien
        ngay_ky_luc = ngay

print(f"Sổ B — ngày {ngay_ky_luc} tiêu nhiều nhất: {tien_ky_luc} đồng")
```

```python title=test
# Chấm trên cả hai cuốn sổ, vì một cuốn thì chưa phân biệt được gì: kỷ lục
# của sổ A đứng cuối sổ, nên "cứ lượt nào cũng ghi" cũng ra đúng con số ấy.
# Sổ B là cuốn nói thật — kỷ lục của nó rơi vào ngày 2, và ba cái tên sau khi
# chạy hết đoạn đang giữ kết quả của chính cuốn này.
assert ngay == 5, "sổ B có năm ngày, và phải lật trọn cả năm mới chắc là không ngày nào phá nổi kỷ lục"
assert tien_ky_luc == 330000, "ngày tiêu mạnh nhất của sổ B hết 330 nghìn — con số ấy có sẵn trong sổ, không phải cộng ra"
assert ngay_ky_luc == 2, "sổ B lập kỷ lục ngay từ ngày 2, bốn ngày sau không ngày nào tiêu hơn"
```

:::hints
- kind: attention
  body: Hai khối dưới là một khối được chép lại: cùng ba cái tên sinh ra từ `0`, cùng hai dòng trong thân `if`. Hai dòng ấy đều nói về lượt này — tiền của lượt này, ngày của lượt này — nên chúng chỉ được chạy ở những lượt đáng thay kỷ lục.
- kind: strategy
  body: Câu hỏi mỗi lượt là "tiền hôm nay có nhiều hơn con số đang giữ kỷ lục không". Bên trái dấu so sánh là cái tên giữ tiền của lượt này, bên phải là cái tên giữ kỷ lục. Hai chỗ trống hỏi đúng một câu ấy, nên điền giống hệt nhau.
- kind: one-line
  body: "Viết `tien > tien_ky_luc` vào **cả hai** chỗ trống, giữ nguyên dấu hai chấm cuối dòng."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Sổ A — ngày 5 tiêu nhiều nhất: 310000 đồng\nSổ B — ngày 2 tiêu nhiều nhất: 330000 đồng\s*$
- tier: output
  expect: Sổ B — ngày 2 tiêu nhiều nhất: 330000 đồng
:::
::::

::::explain{#khi-ca-thang-khong-co-ngay-nao}
Còn một chuyện phải nói rõ trước khi bạn ghép cả bản báo cáo: cuốn sổ có thể
chẳng có ngày nào vượt ngưỡng.

Tháng nào cũng vậy thì tốt cho ví tiền, nhưng nó là một trường hợp mà chương
trình phải xử lý tử tế. Nếu bỏ qua, dòng cuối sẽ in ra `Ngày đầu tiên vượt
ngưỡng: ngày -1` — một câu vô nghĩa với người đọc báo cáo, và tệ hơn: nó **trông
như** một câu trả lời thật.

Vì sao lại chọn `-1` chứ không phải `0`? Vì ngày trong sổ đếm từ 1, nên `-1` là
con số không bao giờ là một ngày thật. Còn `0` thì gần với ngày thật quá — một
lúc nào đó ai đó đổi cách đếm ngày, `0` trở thành ngày hợp lệ, và cái cổng
`== 0` bắt đầu nói dối mà không báo trước.

Nhánh cần viết sau vòng, vì vậy, là:

```python title=readonly
if ngay_dau_vuot == -1:
    print("Cả mười ngày không có ngày nào vượt ngưỡng")
else:
    print(f"Ngày đầu tiên vượt ngưỡng: ngày {ngay_dau_vuot}")
```
::::

::::assemble{#ghep-ca-ban-bao-cao}
Cuốn sổ mười ngày, năm câu hỏi, một lượt duyệt. Đoạn dưới còn hở ba chỗ, và ba
chỗ ấy nằm ở ba chặng khác nhau của cùng một nhịp ba: một chỗ **trước** vòng,
một chỗ **trong** vòng, một chỗ **sau** vòng.

```python title=starter
so_chi_tieu = [85000, 240000, 120000, 95000, 310000,
               60000, 180000, 250000, 75000, 115000]
nguong = 200000

ngay = 0
tong = 0
so_ngay_vuot = 0
tien_ky_luc = 0
ngay_ky_luc = 0
ngay_dau_vuot = ___

for tien in so_chi_tieu:
    ngay += 1
    tong += tien
    if tien > nguong:
        ___
        if ngay_dau_vuot == -1:
            ngay_dau_vuot = ngay
    if tien > tien_ky_luc:
        tien_ky_luc = tien
        ngay_ky_luc = ngay

print("SỔ CHI TIÊU — mười ngày đầu tháng")
print(f"Tổng chi: {tong} đồng")
print(f"Trung bình mỗi ngày: {tong / ngay} đồng")
print(f"Số ngày tiêu quá {nguong} đồng: {so_ngay_vuot}")
print(f"Ngày tiêu nhiều nhất: ngày {ngay_ky_luc} — {tien_ky_luc} đồng")
if ___:
    print("Cả mười ngày không có ngày nào vượt ngưỡng")
else:
    print(f"Ngày đầu tiên vượt ngưỡng: ngày {ngay_dau_vuot}")

# Sổ tháng sau. Mười ngày, không ngày nào vượt ngưỡng — nên nhánh còn lại của
# câu hỏi cuối mới tới lượt chạy.
so_chi_tieu_b = [85000, 120000, 95000, 60000, 180000,
                 75000, 115000, 90000, 130000, 70000]

ngay_b = 0
ngay_dau_vuot_b = ___

for tien in so_chi_tieu_b:
    ngay_b += 1
    if tien > nguong:
        if ngay_dau_vuot_b == -1:
            ngay_dau_vuot_b = ngay_b

print("SỔ CHI TIÊU — tháng sau")
if ___:
    print("Cả mười ngày không có ngày nào vượt ngưỡng")
else:
    print(f"Ngày đầu tiên vượt ngưỡng: ngày {ngay_dau_vuot_b}")
```

```python title=solution
so_chi_tieu = [85000, 240000, 120000, 95000, 310000,
               60000, 180000, 250000, 75000, 115000]
nguong = 200000

ngay = 0
tong = 0
so_ngay_vuot = 0
tien_ky_luc = 0
ngay_ky_luc = 0
ngay_dau_vuot = -1

for tien in so_chi_tieu:
    ngay += 1
    tong += tien
    if tien > nguong:
        so_ngay_vuot += 1
        if ngay_dau_vuot == -1:
            ngay_dau_vuot = ngay
    if tien > tien_ky_luc:
        tien_ky_luc = tien
        ngay_ky_luc = ngay

print("SỔ CHI TIÊU — mười ngày đầu tháng")
print(f"Tổng chi: {tong} đồng")
print(f"Trung bình mỗi ngày: {tong / ngay} đồng")
print(f"Số ngày tiêu quá {nguong} đồng: {so_ngay_vuot}")
print(f"Ngày tiêu nhiều nhất: ngày {ngay_ky_luc} — {tien_ky_luc} đồng")
if ngay_dau_vuot == -1:
    print("Cả mười ngày không có ngày nào vượt ngưỡng")
else:
    print(f"Ngày đầu tiên vượt ngưỡng: ngày {ngay_dau_vuot}")

# Sổ tháng sau. Mười ngày, không ngày nào vượt ngưỡng — nên nhánh còn lại của
# câu hỏi cuối mới tới lượt chạy.
so_chi_tieu_b = [85000, 120000, 95000, 60000, 180000,
                 75000, 115000, 90000, 130000, 70000]

ngay_b = 0
ngay_dau_vuot_b = -1

for tien in so_chi_tieu_b:
    ngay_b += 1
    if tien > nguong:
        if ngay_dau_vuot_b == -1:
            ngay_dau_vuot_b = ngay_b

print("SỔ CHI TIÊU — tháng sau")
if ngay_dau_vuot_b == -1:
    print("Cả mười ngày không có ngày nào vượt ngưỡng")
else:
    print(f"Ngày đầu tiên vượt ngưỡng: ngày {ngay_dau_vuot_b}")
```

```python title=test
# Năm câu trả lời, kiểm từng câu một. Bốn dòng đầu chỉ đúng nếu vòng lặp
# đi trọn mười ngày; dòng cuối chỉ đúng nếu cái cổng giá trị canh còn nguyên.
#
# Sổ tháng sau có mặt ở đây vì một lẽ. Với riêng sổ tháng này, `ngay_dau_vuot`
# luôn bằng 2, nên nhánh "không ngày nào vượt" KHÔNG BAO GIỜ chạy — và chỗ
# trống thứ ba nhận được MỌI biểu thức cho ra `False`, kể cả
# `ngay_dau_vuot == 0`, đúng cái lỗi mà bài 25 dựng riêng phần `reflect` để
# cảnh báo. Bản trước của chính bài BOSS này không bắt được nó.
#
# Sổ tháng sau không có ngày nào quá ngưỡng, nên nhánh ấy mới tới lượt chạy,
# và một giá trị canh sai sẽ in ra "ngày -1" thay vì câu kết luận.
assert tong == 1530000, "mười ngày trong sổ cộng lại hết 1 triệu 530 nghìn"
assert ngay == 10, "sổ ghi mười ngày, nên lượt duyệt phải đi trọn mười ngày mới hết sổ"
assert so_ngay_vuot == 3, "ba ngày trong kỳ tiêu quá 200 nghìn: ngày 2, ngày 5 và ngày 8"
assert ngay_ky_luc == 5, "ngày tiêu nhiều nhất cả kỳ là ngày 5"
assert tien_ky_luc == 310000, "ngày 5 tiêu hết 310 nghìn, nhiều nhất trong mười ngày"
assert ngay_dau_vuot == 2, "ngày 2 là ngày ĐẦU TIÊN vượt ngưỡng — ngày 5 và ngày 8 cũng vượt, nhưng chúng tới sau và không được ghi đè lên"
assert ngay_dau_vuot_b == -1, "sổ tháng sau không ngày nào quá ngưỡng, nên giá trị canh phải còn nguyên chứ không nhận ngày nào"
```

:::hints
- kind: attention
  body: Năm chỗ trống, nhưng chỉ ba câu trả lời khác nhau — hai chỗ cuối lặp lại chỗ thứ nhất và thứ ba cho cuốn sổ thứ hai, với tên có thêm `_b`. Chỗ thứ nhất nằm trong nhóm dòng sinh ra các cái tên, trước khi vòng bắt đầu. Chỗ thứ hai nằm trong thân của `if tien > nguong:`, ngay trên cái cổng. Chỗ thứ ba là điều kiện của nhánh cuối.
- kind: strategy
  body: Chỗ thứ nhất cần giá trị canh mang nghĩa "chưa gặp ngày nào" — cùng con số mà cái cổng bên dưới đem ra so. Chỗ thứ hai là phép đếm có điều kiện: mỗi ngày vượt ngưỡng thì cái tên đếm nhích lên một. Chỗ thứ ba hỏi lại đúng câu mà cái cổng trong vòng đã hỏi.
- kind: one-line
  body: "Lần lượt năm chỗ trống là `-1`, `so_ngay_vuot += 1`, `ngay_dau_vuot == -1`, rồi `-1` và `ngay_dau_vuot_b == -1` cho cuốn sổ thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^SỔ CHI TIÊU — mười ngày đầu tháng\nTổng chi: 1530000 đồng\nTrung bình mỗi ngày: 153000\.0 đồng\nSố ngày tiêu quá 200000 đồng: 3\nNgày tiêu nhiều nhất: ngày 5 — 310000 đồng\nNgày đầu tiên vượt ngưỡng: ngày 2\nSỔ CHI TIÊU — tháng sau\nCả mười ngày không có ngày nào vượt ngưỡng\s*$
- tier: output
  expect: Ngày tiêu nhiều nhất: ngày 5 — 310000 đồng
- tier: output
  expect: Ngày đầu tiên vượt ngưỡng: ngày 2
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một lượt duyệt, năm câu trả lời. Cuốn sổ chỉ bị lật đúng một lần.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi sang mạch sau.

Byte vừa đưa thêm sổ tháng Hai và sổ tháng Ba. Tháng Hai vẫn xét ở ngưỡng 200
nghìn, tháng Ba Byte muốn xét ở 150 nghìn. Cách duy nhất bạn đang có là chép
nguyên đoạn vòng lặp thêm hai lần, mỗi lần sửa đúng một con số.

Chép xong thì trên màn hình có ba đoạn gần giống hệt nhau — và cả ba đều dùng
chung những cái tên `tong`, `ngay`, `so_ngay_vuot`. Chạy hết đoạn thứ hai,
`tong` đang giữ tiền của tháng nào? Sửa một chỗ trong đoạn thứ nhất, hai đoạn kia
có đổi theo không?

Bạn đã đặt tên cho một **giá trị** ở Realm 0, và đặt tên cho **cả một cuốn sổ**
bằng danh sách. Còn đặt tên cho **cả đoạn việc** này — "đưa vào một cuốn sổ và
một ngưỡng, nhận về năm con số" — thì sao?

Ở Realm 0 bạn đã gặp thoáng qua `def`, đủ để gói một việc nhỏ dưới một cái tên.
Mạch sau (T1.3 — Hàm) nhận đúng câu hỏi này và trả lời cho hết: một đoạn việc có
tên thì cái tên `tong` bên trong nó thuộc về ai, và vì sao ba tháng dùng chung
một đoạn mà không giẫm lên nhau.
::::

::::checkpoint{mastery=0.85}
::::
