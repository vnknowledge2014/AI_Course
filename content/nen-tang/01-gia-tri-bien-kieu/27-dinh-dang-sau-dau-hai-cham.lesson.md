---
id: nen-tang.gia-tri-bien-kieu.dinh-dang-sau-dau-hai-cham
title: Phần nằm sau dấu hai chấm
summary: Trong cặp `{}` của f-string, dấu hai chấm mở ra phần dặn cách viết — `{tien:.2f}` in đúng hai chữ số sau dấu chấm.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.format-spec]
requires: [core.fstring, core.float, core.str-cast]
concepts: [core.dinh-dang, core.chuoi]
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
Trong cặp ngoặc nhọn còn một nửa nữa. Nửa sau nói mình phải viết ra thế nào.
::::

::::explain{#o-trong-co-loi-dan}
Bài trước kết bằng một con số không chịu vừa cột:

```python
trung_binh = 1360000 / 30
print(trung_binh)
```

`45333.333333333336`. Mười tám chỗ cho một cột rộng mười.

Bạn đã có `str()` để viết con số ra thành chữ — nhưng `str()` viết ra **tất
cả**, không hỏi bạn muốn bao nhiêu. Và cắt bớt bằng lát cắt thì là cắt chữ: nó
đếm ký tự chứ không biết dấu chấm nằm đâu.

Thứ bạn thật sự muốn nói là một câu khác: *viết cho tôi hai chữ số sau dấu
chấm, thế thôi*. Đó là một **lời dặn về cách viết**, và Python có sẵn một chỗ
để nhận lời dặn ấy.

Nghĩ tới tờ phiếu ghi ở phòng khám. Mỗi ô trống có một dòng chữ nhỏ in mờ bên
dưới: *ghi theo dạng ngày/tháng/năm*, *ghi bằng chữ in hoa*. Ô trống nói **điền
cái gì vào**; dòng chữ mờ nói **viết nó ra thế nào**. Hai việc khác nhau, cùng
nằm ở một chỗ.

Trong f-string, cặp `{}` chính là ô trống ấy. Từ Realm 0 tới giờ bạn mới chỉ
dùng nửa đầu — đặt vào đó một cái tên. Nửa sau nằm ngay đằng sau một **dấu hai
chấm**, và nó là dòng chữ in mờ:

```text
f"{ điền cái gì vào : viết nó ra thế nào }"
```

Phần sau dấu hai chấm gọi là **phần định dạng**. Nó không đổi giá trị nào cả;
nó chỉ điều khiển dãy ký tự mà máy viết ra.

Lời dặn bạn cần hôm nay viết là `.2f`, gồm hai mảnh:

- **`f`** — viết ở dạng số thực có dấu chấm thập phân, tiếng Anh gọi là *fixed
  point*: phần nguyên, dấu chấm, rồi phần lẻ.
- **`.2`** — dấu chấm rồi số `2`: lấy đúng **hai** chữ số sau dấu chấm.

Ghép lại: `f"{trung_binh:.2f}"`.
::::

::::example{#hai-chu-so-la-hai}
Byte in dòng trung bình theo hai kiểu để so:

```python title=readonly
trung_binh = 1360000 / 30

print(f"Trung bình: {trung_binh}đ")
print(f"Trung bình: {trung_binh:.2f}đ")
print(trung_binh)
```

Màn hình:

```text
Trung bình: 45333.333333333336đ
Trung bình: 45333.33đ
45333.333333333336
```

Dòng một là nửa đầu của cặp `{}` — chỉ nói điền cái gì vào, nên máy viết ra tất
cả những gì nó có. Dòng hai thêm lời dặn `.2f` và được đúng hai chữ số lẻ.

Dòng ba là chỗ đáng nhớ nhất: `trung_binh` **không hề đổi**. Nó vẫn giữ nguyên
cái đuôi dài. Luật này bạn đã gặp ở `.strip()`, ở `.lower()`, ở `str()` — máy
làm ra một dãy ký tự **mới** rồi đưa cho bạn, chứ không sửa vào giá trị cũ. Sổ
vẫn giữ con số đầy đủ để còn tính tiếp; phần định dạng chỉ lo lúc **in ra**.

Còn một điều nữa phải nói rõ, vì nó khác hẳn một bài trước:

```python title=readonly
mot_tuan_tet = 2500000 / 7

print(mot_tuan_tet)
print(f"{mot_tuan_tet:.2f}")
```

```text
357142.85714285716
357142.86
```

Chữ số thứ ba sau dấu chấm là `7`, và kết quả cho ra `.86` chứ không phải `.85`.
Nghĩa là `.2f` **làm tròn**, không cắt — nó xử sự như `round` chứ không như
`int()` của bài cắt phần lẻ. Đây là điều bạn cần cho tiền: hai người cùng đọc
một dòng báo cáo thì con số không được lệch xuống một cách âm thầm.

Và `.2f` dùng được cho cả số nguyên: `f"{45000:.2f}"` cho `45000.00`. Nó viết
con số ra ở dạng số thực, dù thứ bạn đưa vào là `int`.
::::

::::predict{#doan-hai-dong commitOnce}
Byte in dòng trung bình có lời dặn, rồi ngay sau đó in thẳng cái tên ra một lần
nữa.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra hai dòng nào?

```python
tong = 1360000
so_ngay = 30
trung_binh = tong / so_ngay

print(f"{trung_binh:.2f}")
print(trung_binh)
```

:::opt{correct}
45333.33 rồi 45333.333333333336
:::

:::opt
45333.33 rồi 45333.33
::why
Gần đúng ở chỗ bạn thấy dòng đầu quả thật đã làm gọn con số lại, và bạn suy ra
việc làm gọn ấy có hiệu lực từ đó về sau — một suy luận thẳng thớm.

Chỗ lệch nằm ở thứ bị làm gọn. Cái được rút ngắn là **dãy ký tự in ra màn
hình**, không phải giá trị nằm dưới cái tên `trung_binh`. F-string dựng một
chuỗi mới rồi đưa cho `print`; nó không với tay vào sửa giá trị cũ, y như
`.strip()` không sửa chuỗi cũ. Dòng 2 hỏi thẳng cái tên nên nó nhận lại nguyên
con số đầy đủ.
::
:::

:::opt
45333.333333333336 rồi 45333.333333333336
::why
Gần đúng ở chỗ bạn nhớ bài số thực xấp xỉ: cái đuôi dài kia là thật, máy không
bịa ra, và nó không tự nhiên biến mất được. Phần đó bạn hiểu chính xác.

Chỗ lệch: phần sau dấu hai chấm không đòi máy sửa con số, nó đòi máy **viết ra
ít chữ số hơn**. Giá trị vẫn dài y nguyên — đó chính là vì sao dòng 2 in ra
đuôi dài — nhưng dòng 1 đã được dặn viết hai chữ số lẻ, nên nó viết đúng hai.
::
:::

:::opt
Máy báo lỗi vì dấu hai chấm không được nằm giữa dòng như thế
::why
Gần đúng ở chỗ bạn để ý đúng một quy luật có thật của Python: dấu hai chấm ở
`if ...:`, ở `for ...:` luôn đứng cuối dòng và mở ra một khối thụt lề bên dưới.
Gặp nó nằm lọt giữa dòng thì nghi là gõ nhầm, đó là phản xạ tốt.

Chỗ lệch: bên trong cặp `{}` của f-string là một vùng có luật riêng, không phải
luật của dòng lệnh. Ở vùng ấy, dấu hai chấm có nghĩa "hết phần nói điền gì,
sang phần nói viết thế nào". Máy đọc trọn dòng này và chạy bình thường.
::
:::
::::

::::code{#hai-dong-bao-cao}
Báo cáo cuối tháng cần hai dòng trung bình. Dòng thứ nhất: tổng cả tháng
`1360000` chia đều cho 30 ngày. Dòng thứ hai: `2500000` tiêu trong bảy ngày
Tết, chia cho 7.

Hai chỗ trống nằm trong cặp `{}` của hai f-string. Điền vào đó cái tên cần in
kèm lời dặn viết ra hai chữ số sau dấu chấm.

Hai con số này được chọn để soi ra hai lỗi khác nhau. Số thứ nhất có đuôi
`.3333…` nên nó tha thứ cho cả cắt lẫn làm tròn; số thứ hai có đuôi `.857…` nên
cắt cho ra `357142.85` còn làm tròn cho ra `357142.86` — chỉ một trong hai qua
được.

```python title=starter
tong_thang = 1360000
trung_binh = tong_thang / 30
bay_ngay_tet = 2500000 / 7

dong_thang = f"Trung bình mỗi ngày: {___}đ"
dong_tet = f"Bảy ngày Tết mỗi ngày: {___}đ"

print(dong_thang)
print(dong_tet)
```

```python title=solution
tong_thang = 1360000
trung_binh = tong_thang / 30
bay_ngay_tet = 2500000 / 7

dong_thang = f"Trung bình mỗi ngày: {trung_binh:.2f}đ"
dong_tet = f"Bảy ngày Tết mỗi ngày: {bay_ngay_tet:.2f}đ"

print(dong_thang)
print(dong_tet)
```

```python title=test
# Hai tình huống, không phải một:
#   1360000 / 30 = 45333.3333…  → cả cắt lẫn làm tròn đều ra 45333.33;
#   2500000 / 7  = 357142.857…  → cắt ra .85, làm tròn ra .86, chỉ một cái đúng.
# Số chữ số lẻ cũng bị soi: `.1f` cho 45333.3, `.3f` cho 45333.333, `.0f` cho
# 45333 — cả ba đều trượt assert đầu tiên.
assert dong_thang == "Trung bình mỗi ngày: 45333.33đ"
assert dong_tet == "Bảy ngày Tết mỗi ngày: 357142.86đ"
# Phần định dạng chỉ đổi cách VIẾT RA, không đổi giá trị đang được đặt tên.
assert trung_binh == 1360000 / 30
assert bay_ngay_tet == 2500000 / 7
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm bên trong cặp `{}`, giữa một câu chữ. Đó là ô trống của f-string, và ô ấy nhận được hai phần chứ không phải một — phần thứ hai đứng sau một dấu ngăn.
- kind: strategy
  body: Viết tên cần in trước, rồi dấu hai chấm, rồi lời dặn cách viết. Lời dặn gồm dấu chấm và số chữ số lẻ bạn muốn, rồi một chữ cái nói "viết ở dạng số thực". Cả hai dòng cần cùng một lời dặn, chỉ khác cái tên đứng trước nó.
- kind: one-line
  body: "Điền `trung_binh:.2f` vào chỗ trống thứ nhất và `bay_ngay_tet:.2f` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Trung bình mỗi ngày: 45333\.33đ\nBảy ngày Tết mỗi ngày: 357142\.86đ\s*$
- tier: static
  onFail: mỗi ô trống phải in ra chính cái tên đang giữ con số, kèm lời dặn định dạng — không chép cứng kết quả và không làm tròn sẵn giá trị
  requireAst:
  - kind: uses-name, target: trung_binh, min: 1
  - kind: uses-name, target: bay_ngay_tet, min: 1
  forbidAst:
  - kind: uses-call, target: round
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai chữ số lẻ, không hơn. Con số dưới cái tên thì mình vẫn giữ nguyên vẹn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Dòng trung bình đọc một cái là ra: `45333.33đ`. Nhưng ngay phía trên nó, báo
cáo còn in tổng cả tháng:

```text
Tổng tháng này: 1360000đ
```

Bảy chữ số dính liền một dải. Muốn biết đó là một triệu ba trăm sáu mươi nghìn
hay mười ba triệu sáu trăm nghìn, mắt bạn phải tự tách từng nhóm ba chữ số từ
phải sang trái — và tách nhầm một nhịp là lệch mười lần.

Trên giấy thì không ai viết thế. Hoá đơn quán phở viết `1.360.000`, sổ ngân
hàng viết `1,360,000`. Nhóm ba chữ số một, có dấu ngăn.

Bạn vừa dặn được máy một chuyện về cách viết — bao nhiêu chữ số sau dấu chấm.
Vậy chỗ ấy có nhận thêm một lời dặn nữa không: **ngăn nhóm ba chữ số giúp
tôi**? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
