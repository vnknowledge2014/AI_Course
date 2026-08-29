---
id: nen-tang.gia-tri-bien-kieu.boss-mot-dong-so-chi-tieu
title: BOSS — Một dòng sổ hoàn chỉnh
summary: Ghép cả track thành một chương trình: câu người gõ vào ra tới một dòng sổ thẳng cột.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 30
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
practices: [core.string-strip, core.int-cast, core.floor-division, core.none, core.thousands-sep, core.field-width, core.fstring, core.don-vi-nho-nhat]
requires: [core.fstring, core.thousands-sep, core.field-width, core.string-strip, core.int-cast, core.floor-division, core.none]
concepts: [core.dinh-dang, core.doi-kieu, core.bien]
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
Hôm nay không có gì mới. Chỉ có một dòng sổ, và tất cả những gì bạn đã học để viết nó.
::::

::::explain{#mot-dong-can-sau-viec}
Byte đi ăn bún chả với ba người bạn. Về nhà, ghi một dòng vào sổ chi tiêu. Chỉ
một dòng — nhưng từ câu người ta gõ vào tới dòng chữ nằm trên sổ là sáu việc,
và bạn đã học riêng từng việc một:

1. **Làm sạch tên khoản.** Người gõ `"  Bún Chả  "` — thừa khoảng trắng hai
   đầu, hoa thường lẫn lộn. Sổ cần `"bún chả"`, để hôm sau gõ `"BÚN CHẢ"` thì
   máy vẫn thấy là cùng một khoản.
2. **Đổi chữ thành số.** Người gõ `"245.5"` — nghĩa là 245,5 nghìn. Đó là chữ,
   và có phần lẻ.
3. **Chọn đơn vị và kiểu.** Tiền thì đếm, nên giữ bằng **đồng** dưới dạng `int`.
   `245.5` nghìn thành `245500` đồng.
4. **Tính thêm phí.** Quán tính 5% phí phục vụ. Con số 5 ấy không được nằm trần
   giữa công thức — nó là một **hằng**, có tên viết HOA ở đầu chương trình.
5. **Chia đều cho nhóm.** Bốn người ăn chung: mỗi người bao nhiêu, và còn thừa
   mấy đồng lẻ không chia được.
6. **In ra một dòng thẳng cột.** Ô tên 12 chỗ, ô tiền 10 chỗ, có dấu ngăn nhóm.

Còn một ô nữa trên dòng sổ: **ghi chú**. Hôm nay chưa ai viết gì vào đó — mà
"chưa viết gì" không phải số 0, cũng không phải chuỗi rỗng. Bạn có sẵn một giá
trị dành riêng cho nó.

Bài này không dạy thứ gì mới. Nó là chỗ bạn tự ghép.
::::

::::example{#nua-dau-cua-mot-dong}
Đây là hai việc đầu, viết ra thành code. Ba câu người gõ được ghi sẵn trong một
danh sách, vì ô chấm bài chạy lúc không có ai ngồi trước bàn phím — cả ba đều
là chữ, đúng thứ `input()` đưa về.

```python title=readonly
NGUOI_GO = ["  Bún Chả  ", "245.5", "4"]

ten_sach = NGUOI_GO[0].strip()
ten = ten_sach.lower()

tien_nghin = float(NGUOI_GO[1])
tong = round(tien_nghin * 1000)

print(f"[{ten}]")
print(tong, type(tong))
```

```text
[bún chả]
245500 <class 'int'>
```

Hai dấu ngoặc vuông là của Byte, để bạn thấy hai đầu chuỗi đã sạch khoảng trắng.

Dòng `tong = round(tien_nghin * 1000)` là dòng đáng nhìn kỹ nhất. `tien_nghin`
đang giữ một số thực, mà một số thực lẫn vào thì cả phép nhân ra số thực —
`245.5 * 1000` cho `245500.0`. Cái đuôi `.0` ấy sẽ theo con số đi hết chương
trình nếu không có ai chặn lại ngay tại đây.

Chặn bằng `round`, không phải `int` — đúng như bài 7 đã chốt. Ở riêng con số
`245.5` thì hai cách cho cùng kết quả, nên nhìn vào đây không thấy khác biệt.
Nhưng hôm nào khách gõ `32.3` thì `32.3 * 1000` máy giữ là
`32299.999999999996`, và `int` cắt xuống `32299` — mất một đồng, im lặng,
không báo lỗi nào. Chọn công cụ theo cả những con số chưa gõ tới, chứ không
theo con số đang có trước mắt.
::::

::::predict{#doan-cai-duoi commitOnce}
Giả sử ai đó quên mất `round()` ở dòng vừa rồi, rồi in con số ra kèm dấu ngăn
nhóm. **Trước khi bấm chạy**, bạn đoán màn hình hiện ra dòng nào?

```python
tien_nghin = float("245.5")
tong = tien_nghin * 1000
print(f"{tong:,}đ")
```

:::opt{correct}
245,500.0đ
:::

:::opt
245,500đ
::why
Gần đúng ở chỗ bạn đọc dấu phẩy đúng như bài trước dạy: nó gom `245500` thành
`245,500`. Phần ngăn nhóm bạn làm đúng từng chữ số.

Chỗ lệch nằm ở kiểu của `tong`. `float("245.5")` cho một số thực, và một số
thực nhân với `1000` vẫn ra số thực — nên `tong` đang giữ `245500.0`, kèm cái
đuôi. Dấu phẩy chỉ ngăn nhóm; nó không đổi kiểu giúp bạn. Muốn mất cái đuôi thì
phải gọi `int()`, và gọi trước lúc in.
::
:::

:::opt
245,500.00đ
::why
Gần đúng ở chỗ bạn nhớ phần định dạng quyết định được số chữ số sau dấu chấm —
đúng, nhưng đó là việc của `.2f`, và ở đây không ai viết `.2f`.

Chỗ lệch: dấu phẩy đứng một mình chỉ làm mỗi việc ngăn nhóm, phần lẻ để nguyên
như khi in trần. `245500.0` in trần ra một chữ số lẻ, nên có ngăn nhóm thì vẫn
đúng một chữ số lẻ.
::
:::

:::opt
Máy nổ ValueError, vì dấu ngăn nhóm chỉ đặt được lên số nguyên
::why
Gần đúng ở chỗ bạn nhớ rằng bài trước giới thiệu dấu phẩy trên một số nguyên —
`tong_thang` là `int` thật — và bạn đang suy ra ranh giới của dấu phẩy từ đúng
ví dụ mình đã gặp.

Chỗ lệch: dấu phẩy làm việc với cả hai kiểu số. Với số thực nó ngăn nhóm phần
đứng **trước** dấu chấm và để nguyên phần lẻ phía sau — đó cũng chính là lý do
`,` ghép được với `.2f` thành `,.2f`.
::
:::
::::

::::code{#chia-deu-cho-nhom}
Hoá đơn của cả nhóm đã tính xong: `257775` đồng, bốn người ăn chung. Byte cần
hai con số: mỗi người đưa bao nhiêu, và còn thừa mấy đồng lẻ.

Hai chỗ trống nằm ở hai dòng liền nhau, và chúng cần **hai phép khác nhau** —
một phép lấy phần chia được cho mỗi người, một phép lấy phần không chia nổi.
Điền cùng một thứ vào cả hai chỗ thì có một dòng sai.

Phép chia thường không dùng được ở đây: `257775 / 4` cho `64443.75`, mà không
ai đưa được ba phần tư đồng.

```python title=starter
phai_tra = 257775
so_nguoi = 4

moi_nguoi = phai_tra ___ so_nguoi
con_thua = phai_tra ___ so_nguoi

print(f"Mỗi người: {moi_nguoi:,}đ")
print(f"Còn thừa: {con_thua}đ")
```

```python title=solution
phai_tra = 257775
so_nguoi = 4

moi_nguoi = phai_tra // so_nguoi
con_thua = phai_tra % so_nguoi

print(f"Mỗi người: {moi_nguoi:,}đ")
print(f"Còn thừa: {con_thua}đ")
```

```python title=test
assert moi_nguoi == 64443, "hoá đơn 257775 đồng chia đều cho bốn người thì mỗi người đưa 64443 đồng"
assert con_thua == 3, "chia xong còn 3 đồng lẻ, không đủ thêm một suất cho ai"
# Tiền không tự sinh ra cũng không tự mất đi: bốn phần bằng nhau cộng phần thừa
# phải đúng bằng hoá đơn. Một chỗ trống điền sai là đẳng thức này gãy.
assert moi_nguoi * so_nguoi + con_thua == phai_tra, "bốn phần bằng nhau cộng phần lẻ phải đúng bằng hoá đơn — tiền không tự sinh ra cũng không tự mất đi"
# Và mỗi người đưa một số đồng chẵn — phép chia thường sẽ cho 64443.75.
assert type(moi_nguoi) is int, "mỗi người đưa một số đồng chẵn, không ai đưa được ba phần tư đồng"
```

:::hints
- kind: attention
  body: Hai dòng có cùng hình dạng — cùng số bị chia, cùng số chia — nên thứ phân biệt chúng chỉ có thể là dấu phép nằm ở chỗ trống. Nhìn lại tên hai cái tên đang chờ kết quả: một cái hỏi "mỗi người", một cái hỏi "còn thừa".
- kind: strategy
  body: Bạn cần hai phép chia đã học ở đầu track: một phép trả về phần nguyên của thương và không bao giờ có đuôi lẻ, một phép trả về đúng phần còn lại sau khi đã chia hết mức có thể. Cả hai đều viết bằng dấu, không phải bằng tên hàm.
- kind: one-line
  body: "Chỗ trống thứ nhất là `//`, chỗ trống thứ hai là `%`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Mỗi người: 64,443đ\nCòn thừa: 3đ\s*$
- tier: static
  onFail: hai chỗ trống cần hai phép chia khác nhau — một phép lấy phần nguyên, một phép lấy phần dư
  requireAst:
  - kind: uses-operator, target: //, min: 1
  - kind: uses-operator, target: %, min: 1
:::
::::

::::explain{#o-ghi-chu-va-dong-cuoi}
Còn hai mảnh nữa trước khi ghép.

**Ô ghi chú.** Hôm nay Byte chưa viết gì vào đó. "Chưa viết gì" khác "viết vào
số 0" và khác "viết vào một chuỗi rỗng" — nên cái tên ấy giữ `None`, và câu hỏi
để nhận ra nó là `is None`:

```python title=readonly
ghi_chu = None

if ghi_chu is None:
    print("  (chưa có ghi chú)")
```

**Dòng sổ.** Ô tên 12 chỗ, ô tiền 10 chỗ, tiền có dấu ngăn nhóm:

```python title=readonly
print(f"{'bún chả':12}{257775:10,}đ")
```

```text
bún chả        257,775đ
```

Hai ô ấy đứng liền nhau trong cùng một câu, không có khoảng trắng nào chen giữa
— khoảng trắng bạn nhìn thấy là phần ô tên còn thừa và phần ô tiền còn thừa, do
máy chèn vào.
::::

::::assemble{#ghep-mot-dong-so}
Đây là cả chương trình, còn hở năm chỗ nằm ở năm phần khác nhau. Điền vào để nó
chạy trọn vẹn từ câu người gõ tới dòng sổ.

Byte đóng vai người gõ: ba câu ghi sẵn trong `NGUOI_GO`, cả ba đều là chữ.

```python title=starter
NGUOI_GO = ["  Bún Chả  ", "245.5", "4"]

# Phí phục vụ của quán, tính theo phần trăm. Hằng để bằng số nguyên `5` chứ
# không phải `0.05`, để phép tính phía dưới không kéo tiền sang số thực.
PHAN_TRAM_PHUC_VU = 5

ten_sach = NGUOI_GO[0].___()
ten = ten_sach.lower()

tien_nghin = float(NGUOI_GO[1])
tong = ___(tien_nghin * 1000)

phi = tong * PHAN_TRAM_PHUC_VU // 100
phai_tra = tong + phi

so_nguoi = int(NGUOI_GO[2])
moi_nguoi = phai_tra // so_nguoi
con_thua = phai_tra % so_nguoi

ghi_chu = None

print(f"{ten:___}{phai_tra:___,}đ")
print(f"  {so_nguoi} người chia · mỗi người {moi_nguoi:,}đ · còn thừa {con_thua}đ")
if ghi_chu ___ None:
    print("  (chưa có ghi chú)")
```

```python title=solution
NGUOI_GO = ["  Bún Chả  ", "245.5", "4"]

# Phí phục vụ của quán, tính theo phần trăm. Hằng để bằng số nguyên `5` chứ
# không phải `0.05`, để phép tính phía dưới không kéo tiền sang số thực.
PHAN_TRAM_PHUC_VU = 5

ten_sach = NGUOI_GO[0].strip()
ten = ten_sach.lower()

tien_nghin = float(NGUOI_GO[1])
tong = round(tien_nghin * 1000)

phi = tong * PHAN_TRAM_PHUC_VU // 100
phai_tra = tong + phi

so_nguoi = int(NGUOI_GO[2])
moi_nguoi = phai_tra // so_nguoi
con_thua = phai_tra % so_nguoi

ghi_chu = None

print(f"{ten:12}{phai_tra:10,}đ")
print(f"  {so_nguoi} người chia · mỗi người {moi_nguoi:,}đ · còn thừa {con_thua}đ")
if ghi_chu is None:
    print("  (chưa có ghi chú)")
```

```python title=test
# Năm chỗ trống, năm phép kiểm riêng — sai chỗ nào lộ chỗ ấy.
assert ten == "bún chả", "người gõ '  Bún Chả  ' thì sổ phải ghi bún chả: sạch khoảng trắng hai đầu và về hết chữ thường"
assert tong == 245500, "245,5 nghìn là 245500 đồng — sổ đếm tiền bằng đơn vị nhỏ nhất"
assert type(tong) is int, "đã đếm bằng đồng thì không còn nửa đồng nào, nên con số không được mang đuôi .0 của số thực"
assert phi == 12275, "quán thu 5% phí phục vụ trên 245500 đồng, và phí thu về cũng là một số đồng chẵn"
assert phai_tra == 257775, "hoá đơn cả nhóm phải trả là tiền món cộng thêm phí phục vụ"
assert moi_nguoi == 64443 and con_thua == 3, "bốn người chia hoá đơn 257775 đồng thì mỗi người 64443 đồng, còn lẻ 3 đồng"
assert ghi_chu is None, "hôm nay chưa ai viết gì vào ô ghi chú, mà chưa viết gì thì là None chứ không phải số 0 hay chuỗi rỗng"
# Bề rộng hai ô thì phần chấm theo màn hình lo: chữ đ phải rơi vào chỗ thứ 23.
```

:::hints
- kind: attention
  body: Năm chỗ trống nằm ở năm phần khác nhau. Hai chỗ đầu thuộc phần làm sạch và đổi kiểu; hai chỗ trong dòng `print` đầu tiên là bề rộng hai ô; chỗ cuối nằm trong câu hỏi về ô ghi chú.
- kind: strategy
  body: Chỗ một cần phương thức cắt khoảng trắng hai đầu chuỗi. Chỗ hai cần lệnh đưa một số thực về số nguyên đồng — bài 7 đã chỉ đích danh nên dùng cái nào trong hai cái, và vì sao cái kia làm mất tiền. Hai chỗ trong f-string là số chỗ của ô tên và ô tiền, đúng hai con số cuốn sổ đã kẻ. Chỗ cuối là câu hỏi dành riêng cho giá trị "chưa có gì" — không phải phép so sánh bằng thông thường.
- kind: one-line
  body: "Lần lượt năm chỗ trống là `strip`, `round`, `12`, `10`, và `is`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^bún chả {8}257,775đ\n  4 người chia · mỗi người 64,443đ · còn thừa 3đ\n  \(chưa có ghi chú\)\s*$
- tier: static
  onFail: chương trình còn thiếu bước làm sạch khoảng trắng hai đầu tên khoản
  requireAst:
  # CHỈ còn luật `strip`. Bản trước đòi thêm `uses-call target: int min: 2` và
  # nó đánh trượt một lời giải ĐÚNG HƠN lời giải mẫu: điền `round` cho đúng
  # luật bài 7 thì chương trình chỉ còn một `int` (ở `int(NGUOI_GO[2])`), nên
  # luật gãy — và người học nhận về câu "còn thiếu bước đưa tiền về số nguyên
  # đồng", đúng cái bước họ vừa làm cẩn thận hơn.
  #
  # Bước đổi kiểu không cần luật tĩnh nào canh: hai `assert` dưới kia đã hỏi
  # thẳng KẾT QUẢ — `tong == 245500` và `type(tong) is int` — mà không buộc
  # người học phải đi bằng đúng một cái tên hàm.
  - kind: uses-call, target: strip, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dòng sổ. Từ câu người ta gõ vội tới chữ đ nằm đúng cột. Hết track.
::::

::::reflect{#nghi-lai}
Một câu hỏi mang sang chặng sau.

Một dòng thì đã đẹp. Nhưng cuốn sổ đâu chỉ để nhìn — Byte muốn nó **xếp loại**
giúp: dưới 50 nghìn là khoản lặt vặt, từ 50 nghìn tới 500 nghìn là khoản
thường, trên 500 nghìn là khoản lớn.

Ba mức thì bạn viết được ngay bằng `if` / `elif` / `else` của Realm 0.

Nhưng Byte còn dặn thêm một câu: *riêng khoản trên 500 nghìn mà lại là tiền ăn
thì nhắc mình một tiếng.* Câu ấy không xếp ngang hàng với ba mức trên được.
"Có phải tiền ăn không" chỉ đáng hỏi với những khoản **đã** vượt 500 nghìn —
hỏi nó với khoản gửi xe 5 nghìn thì hỏi cũng bằng thừa.

Một câu hỏi chỉ được hỏi khi một câu hỏi khác đã đúng. Điều kiện nằm trong điều
kiện. Viết ra thế nào?

Đó là việc đầu tiên của chặng sau.
::::

::::checkpoint{mastery=0.85}
::::
