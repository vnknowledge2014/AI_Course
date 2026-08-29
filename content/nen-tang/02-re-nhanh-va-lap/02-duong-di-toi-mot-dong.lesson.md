---
id: nen-tang.re-nhanh-va-lap.duong-di-toi-mot-dong
title: Đường đi tới một dòng
summary: Điều kiện thật của một dòng là tất cả điều kiện bao ngoài nó cùng đúng — và khối lồng không có `else` viết lại được bằng `and`.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ctrl.condition-path]
requires: [ctrl.if-nested, logic.and, ctrl.block-indent]
concepts: [ctrl.re-nhanh, logic.phep-logic]
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
Muốn biết một dòng chạy khi nào, đừng nhìn nó. Nhìn cả đường đi tới nó.
::::

::::explain{#ba-lop-cua}
Bài trước để lại một câu hỏi: bạn mở file của người khác và thấy một dòng
`print` nằm sâu ba tầng thụt lề. Dòng ấy chạy trong trường hợp nào?

Nhìn vào chính dòng đó thì không trả lời được. Nó chỉ là một câu `print` bình
thường, trên mình nó không mang điều kiện nào cả.

Câu trả lời nằm ở **phía trên** nó.

Bác Tư dắt xe về nhà. Muốn tới đúng chỗ đỗ ở khu B dưới hầm, bác phải qua ba
lớp cửa nối đuôi nhau: cổng chung cư, cửa xuống hầm, rồi lối rẽ vào khu B.
Đóng cửa nào cũng vậy thôi — bác không tới được chỗ đỗ. Cổng khoá thì bác dừng
ngay ngoài đường; cửa hầm khoá thì bác dừng ở sân.

Chỗ đỗ xe ấy không tự nó đặt ra điều kiện gì. Điều kiện thật của nó là **cả ba
cửa trên đường đi**, cửa nào cũng phải mở.

Một dòng lệnh nằm sâu trong nhiều tầng `if` đứng đúng vị trí chỗ đỗ xe đó. Cứ
mỗi tầng thụt lề là thêm một lớp cửa mà máy phải mở được mới đi tiếp.
::::

::::example{#doc-nguoc-len-theo-cot-le}
Byte ghi sổ chi tiêu ba tầng: chỉ những ngày trong tuần, mà có đi chợ, mà tiêu
quá 200 nghìn thì mới nhắc.

```python title=readonly
la_ngay_trong_tuan = True
co_di_cho = True
tien_hom_nay = 250000

if la_ngay_trong_tuan:
    if co_di_cho:
        if tien_hom_nay > 200000:
            print("Ngày đi chợ trong tuần mà tiêu quá tay.")
```

Hai dòng `if` đầu không có dấu so sánh nào, và đó không phải chỗ viết thiếu:
`la_ngay_trong_tuan` với `co_di_cho` đang giữ sẵn `True` từ hai dòng trên cùng,
nên chúng **đã là** câu trả lời có–không rồi, không cần so sánh gì thêm. Realm 0
đã chỉ chuyện này lúc dạy `and`. Chỉ dòng thứ ba mới phải hỏi một câu so sánh,
vì `tien_hom_nay` giữ một con số chứ không giữ đúng/sai.

Máy in ra:

```text
Ngày đi chợ trong tuần mà tiêu quá tay.
```

Bây giờ tập cái việc bạn sẽ làm suốt đời với code người khác viết: đứng ở dòng
`print` rồi **dò ngược lên**.

Dòng `print` nằm ở cột 12. Đi ngược lên, dòng đầu tiên có lề **nhỏ hơn** 12 là
`if tien_hom_nay > 200000:` ở cột 8. Đó là lớp cửa thứ nhất.

Đứng ở cột 8, lại đi ngược lên tìm dòng đầu tiên có lề nhỏ hơn 8: gặp
`if co_di_cho:` ở cột 4. Lớp cửa thứ hai.

Đứng ở cột 4, đi ngược lên: gặp `if la_ngay_trong_tuan:` ở cột 0. Lớp cửa thứ
ba, và cột 0 là hết đường — không còn gì bao ngoài nữa.

Ba lần dò cho ra ba câu hỏi. Ghép lại thành một câu tiếng Việt: dòng `print` ấy
chạy khi **hôm nay là ngày trong tuần, và có đi chợ, và tiêu quá 200 nghìn**.

Việc vừa làm có tên: đi ngược từ một dòng lên tới lề trái để thu đủ mọi lớp cửa
bao ngoài nó, ấy là dò **đường đi tới một dòng**. Nó là việc bạn làm mỗi lần mở
code người lạ, và cả bài hôm nay xoay quanh đúng nó.

Chữ **và** lặp lại hai lần trong câu vừa rồi không phải chuyện tình cờ.
::::

::::predict{#mot-cua-dong-giua-duong commitOnce}
Đổi hai chỗ và thêm hai dòng: hôm nay không đi chợ, nhưng tiêu tới 300 nghìn;
thêm một `print` trong thân tầng ngoài, và một `print` nữa sát lề trái.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python
la_ngay_trong_tuan = True
co_di_cho = False
tien_hom_nay = 300000

if la_ngay_trong_tuan:
    print("Đang xem một ngày trong tuần.")
    if co_di_cho:
        if tien_hom_nay > 200000:
            print("Ngày đi chợ trong tuần mà tiêu quá tay.")
print("Đã ghi vào sổ.")
```

:::opt{correct}
Hai dòng: Đang xem một ngày trong tuần, rồi Đã ghi vào sổ.
:::

:::opt
Cả ba dòng
::why
Gần đúng ở chỗ bạn kiểm hai điều kiện và kiểm chính xác cả hai: `la_ngay_trong_tuan`
đang giữ `True`, và `300000 > 200000` cũng cho `True` thật. Hai đầu của con
đường đều thông.

Chỗ lệch nằm ở khúc giữa. `co_di_cho` đang giữ `False`, mà nó là lớp cửa thứ
hai trên đường đi tới dòng `print` trong cùng. Máy tới đó, thấy `False`, và
nhảy qua nguyên cả khối bên trong — kể cả câu hỏi 200 nghìn, nó cũng không hỏi
tới. Một cửa khoá giữa đường thì hai đầu thông cũng không đi được.
::
:::

:::opt
Chỉ một dòng: Đã ghi vào sổ.
::why
Gần đúng ở chỗ bạn lần đúng đường đi tới dòng `print` trong cùng và thấy nó bị
`co_di_cho` chặn lại. Phần đó bạn suy luận chuẩn.

Chỗ lệch nằm ở dòng `print("Đang xem một ngày trong tuần.")`. Dòng ấy ở cột 4,
không phải cột 12. Dò ngược lên từ nó chỉ gặp **một** lớp cửa duy nhất là
`if la_ngay_trong_tuan:` — và cửa ấy đang mở. Nó chạy trước khi máy đụng tới
`co_di_cho`.

Mỗi dòng có đường đi riêng của nó, dài ngắn khác nhau.
::
:::

:::opt
Không in ra dòng nào cả
::why
Gần đúng ở chỗ bạn nhớ luật của bài trước: điều kiện sai thì cả khối thụt vào
bị bỏ qua. Luật đó đúng, và ở đây nó áp cho `if co_di_cho:` thật.

Chỗ lệch: `co_di_cho` chỉ khoá được những dòng nằm **trong thân** của nó, tức
là hai dòng ở cột 8 và cột 12. Dòng ở cột 4 nằm trước nó và ngoài nó. Còn
`print("Đã ghi vào sổ.")` viết sát lề trái, cột 0 — đường đi tới nó không có
lớp cửa nào, nên nó chạy trong mọi trường hợp.
::
:::
::::

::::explain{#gop-ba-cua-thanh-mot-cau}
Câu tiếng Việt lúc nãy có hai chữ "và". Realm 0 đã cho bạn từ tiếng Anh của chữ
ấy: `and`, từ nối chỉ cho `True` khi cả hai vế cùng đúng.

`and` nối được nhiều hơn hai vế. Viết `a and b and c` thì máy đọc từ trái sang,
và cả cụm chỉ cho `True` khi cả ba cùng `True` — vẫn là đúng cái `and` bạn đã
biết, dùng hai lần liền nhau.

Nên đoạn ba tầng lúc nãy viết lại được thành một dòng:

```python
la_ngay_trong_tuan = True
co_di_cho = True
tien_hom_nay = 250000

if la_ngay_trong_tuan and co_di_cho and tien_hom_nay > 200000:
    print("Ngày đi chợ trong tuần mà tiêu quá tay.")
```

Chạy lên, máy in ra đúng một dòng như cũ:

```text
Ngày đi chợ trong tuần mà tiêu quá tay.
```

Ba lớp cửa nối đuôi nhau thành một lớp cửa đòi cả ba thứ. Cách viết lại này gọi
là **làm phẳng** — tiếng Anh là *flatten*, từ để bạn tra cứu. Nó cắt ba tầng
thụt lề xuống còn một, và câu điều kiện đọc lên đúng như câu tiếng Việt bạn vừa
nói ra miệng.

Nhưng phép này chỉ làm được khi có đủ **hai** điều kiện. Điều kiện thứ nhất
chính là chỗ đoạn trong bước đoán vừa rồi khác với đoạn ở đây: **thân của tầng
ngoài không được có việc gì khác** ngoài đúng cái `if` tầng trong.

Đoạn trong bước đoán có `print("Đang xem một ngày trong tuần.")` nằm ở cột 4.
Dòng ấy chạy với mọi ngày trong tuần, đi chợ hay không. Gộp cả ba điều kiện lại
thành một `and` là dòng ấy mất chỗ đứng — nó sẽ bị kéo vào sâu và chỉ còn chạy
trong những ngày đi chợ tốn kém.

Còn điều kiện thứ hai thì hôm nay chưa có đoạn nào cho bạn thấy, vì nó chỉ lộ ra
khi khối lồng có thêm đúng một chữ. Bài sau chỉ thẳng vào chữ ấy. Từ giờ tới đó,
hãy nhớ rằng luật bạn vừa cầm còn thiếu một nửa.
::::

::::code{#viet-lai-cho-phang}
Byte nhận một file của quán phở đầu ngõ. Trong file có đúng đoạn này, và không
ai kèm theo lời giải thích nào:

```python
if troi_lanh:
    if tuoi_khach > 65:
        if la_khach_quen:
            print("Tặng bác một chén trà nóng.")
```

Chủ quán muốn gộp ba tầng `if` ấy xuống còn một. Thân của mỗi tầng ngoài không
có việc gì khác, nên đoạn này làm phẳng được — nhưng muốn gộp thì phải biết gộp
những gì, mà điều đó chỉ có một cách tìm ra: đứng ở dòng `print` rồi dò ngược
lên. Cột 12 lên cột 8, cột 8 lên cột 4, cột 4 lên cột 0. Ba lớp cửa dò được là
ba vế của câu điều kiện mới.

Một câu điều kiện chỉ được coi là viết đúng khi nó xử đúng **mọi** người khách
bước vào quán, chứ không riêng một bác. Thử với đúng một bác thì câu `True` trần
trụi cũng qua — mà `True` thì mời trà cho cả quán.

Nên Byte đặt tên cho việc mời trà, đúng cách Realm 0 đã dạy, rồi gọi nó bốn
lần, mỗi lần một cảnh khác:

- hôm lạnh, bác Tư 70 tuổi, khách quen — phải mời;
- hôm lạnh, bác Năm 70 tuổi, mới ghé lần đầu — không mời;
- hôm lạnh, anh Sáu 40 tuổi, khách quen — không mời;
- hôm nắng, bác Tư 70 tuổi, khách quen — không mời.

Bốn cảnh ấy nằm nối tiếp nhau bên dưới, mỗi cảnh gán lại ba cái tên rồi hỏi
lại đúng một câu. Viết điều kiện gộp vào **cả bốn** chỗ trống — cùng một câu,
chép xuống bốn lần.

Bài chấm bằng trọn vẹn màn hình, nên câu trả lời đúng in ra đúng một dòng —
dòng của bác Tư hôm trời lạnh.

```python title=starter
troi_lanh = True
tuoi_khach = 70
la_khach_quen = True

if ___:
    print("Tặng bác một chén trà nóng.")

troi_lanh = True
tuoi_khach = 70
la_khach_quen = False

if ___:
    print("Tặng bác một chén trà nóng.")

troi_lanh = True
tuoi_khach = 40
la_khach_quen = True

if ___:
    print("Tặng bác một chén trà nóng.")

troi_lanh = False
tuoi_khach = 70
la_khach_quen = True

if ___:
    print("Tặng bác một chén trà nóng.")
```

```python title=solution
troi_lanh = True
tuoi_khach = 70
la_khach_quen = True

if troi_lanh and tuoi_khach > 65 and la_khach_quen:
    print("Tặng bác một chén trà nóng.")

troi_lanh = True
tuoi_khach = 70
la_khach_quen = False

if troi_lanh and tuoi_khach > 65 and la_khach_quen:
    print("Tặng bác một chén trà nóng.")

troi_lanh = True
tuoi_khach = 40
la_khach_quen = True

if troi_lanh and tuoi_khach > 65 and la_khach_quen:
    print("Tặng bác một chén trà nóng.")

troi_lanh = False
tuoi_khach = 70
la_khach_quen = True

if troi_lanh and tuoi_khach > 65 and la_khach_quen:
    print("Tặng bác một chén trà nóng.")
```

```python title=test
# Chấm bằng TRỌN VẸN output của bốn lần gọi (`match: trim`), không phải một
# dòng lẻ. Bốn cảnh được chọn để mọi câu trả lời hụt một vế đều lộ ra:
#   `True`                          → in bốn dòng;
#   `False`                         → không in dòng nào;
#   dò thiếu, chỉ lấy một lớp cửa   → in ba dòng;
#   dò thiếu, chỉ lấy hai lớp cửa   → in hai dòng;
#   nối bằng `or` thay vì `and`     → in ba hoặc bốn dòng.
# Chỉ đủ ba vế nối bằng `and` mới cho ra đúng một dòng — nghĩa là chỉ người dò
# hết đường đi tới dòng `print` mới qua được. Người học chưa viết assert nên
# khối này không thêm gì; nó ở đây để nói rõ vì sao bốn cảnh là bốn.
#
# Bốn cảnh viết thẳng, không bọc vào một hàm: tới lúc này chưa bài nào dạy hàm
# có quá MỘT tham số — mọi `def` của Realm 0 và T1.1 đều một tham số hoặc
# không tham số nào. Một hàm ba tham số ở đây bắt người học tự đoán thêm hai
# luật (dấu phẩy ngăn tham số, và đối số khớp tham số theo thứ tự vị trí) ngay
# giữa bài đang dạy chuyện khác.
pass
```

:::hints
- kind: attention
  body: Bốn chỗ trống nhận cùng MỘT câu trả lời — bốn cảnh khác nhau, cùng một luật. Mỗi chỗ nằm giữa `if` và dấu hai chấm, nên chỉ chứa được **một** câu đúng/sai. Đừng vội điền: quay lên đoạn trong file của quán, đặt ngón tay vào dòng `print`, rồi đi ngược lên chép ra từng dòng có lề nhỏ hơn.
- kind: strategy
  body: Ba lần dò cho ba câu hỏi, mà cả ba phải cùng đúng thì bác mới có trà. Realm 0 đã cho bạn từ nối buộc hai câu hỏi có–không thành một, và bài này vừa nói thêm rằng dùng nó hai lần liền nhau thì nối được ba vế. Hai trong ba vế là cái tên đang giữ sẵn giá trị đúng/sai, không cần so sánh gì thêm; vế còn lại là câu hỏi về tuổi. Thử áp câu bạn định viết lên lần lượt bốn cảnh trước khi bấm chạy — ba cảnh sau phải im lặng.
- kind: one-line
  body: "Viết `troi_lanh and tuoi_khach > 65 and la_khach_quen` vào cả bốn chỗ trống, giữ nguyên dấu hai chấm."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: Tặng bác một chén trà nóng.
  match: trim
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba cửa hay một cửa đòi ba thứ — với mình là một. Với mắt bạn thì gọn hơn hẳn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Điều kiện thứ hai lúc nãy còn để ngỏ. Chữ làm nó lộ ra là `else` — và bạn tự
thấy được nó làm gì, không cần đợi bài sau nói hộ.

Thử lấy giấy bút làm một phép đổi. Đoạn dưới đây **thoả** điều kiện thứ nhất:
thân tầng ngoài không có việc gì khác ngoài đúng cái `if` tầng trong. Chỉ khác
một chỗ — tầng trong có thêm một `else`:

```python
if la_khach_quen:
    if tien >= 45000:
        print("Tính giá quen cho bác.")
    else:
        print("Bác quen mà thiếu tiền, ghi sổ nợ nhé.")
```

Gộp hai điều kiện thành `if la_khach_quen and tien >= 45000:` rồi giữ nguyên
`else` bên dưới. Bây giờ một người **lạ** bước vào quán thì máy nói câu gì với
họ?

Bài sau trả lời, và câu trả lời chỉ ra đúng chỗ mà `and` không thay được cho
lồng.
::::

::::checkpoint{mastery=0.8}
::::
