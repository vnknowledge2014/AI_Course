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
estimatedMinutes: 12
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

Chữ **và** lặp lại hai lần trong câu vừa rồi không phải chuyện tình cờ.
::::

::::predict{#mot-cua-dong-giua-duong commitOnce}
Đổi hai chỗ: hôm nay không đi chợ, nhưng tiêu tới 300 nghìn. Và tầng ngoài cùng
có thêm một dòng `print` của riêng nó.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python
la_ngay_trong_tuan = True
co_di_cho = False
tien_hom_nay = 300000

if la_ngay_trong_tuan:
    print("Đang xem một ngày trong tuần.")
    if co_di_cho:
        if tien_hom_nay > 200000:
            print("Đi chợ tốn quá tay.")
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

Nhưng có một điều kiện để làm được phép này, và nó chính là chỗ đoạn trong bước
đoán vừa rồi khác với đoạn ở đây: **thân của tầng ngoài không được có việc gì
khác** ngoài đúng cái `if` tầng trong.

Đoạn trong bước đoán có `print("Đang xem một ngày trong tuần.")` nằm ở cột 4.
Dòng ấy chạy với mọi ngày trong tuần, đi chợ hay không. Gộp cả ba điều kiện lại
thành một `and` là dòng ấy mất chỗ đứng — nó sẽ bị kéo vào sâu và chỉ còn chạy
trong những ngày đi chợ tốn kém.
::::

::::code{#viet-lai-cho-phang}
Quán phở tặng trà cho khách quen trên 65 tuổi. Byte viết đoạn này bằng hai tầng
lồng:

```python
if tuoi_khach > 65:
    if la_khach_quen:
        print("Tặng bác một chén trà.")
```

Thân của tầng ngoài không có việc gì khác, nên đoạn này làm phẳng được. Hãy
viết điều kiện gộp vào chỗ trống, để dòng `print` chỉ chạy khi **cả hai** điều
cùng đúng.

```python title=starter
tuoi_khach = 70
la_khach_quen = True

if ___:
    print("Tặng bác một chén trà.")
```

```python title=solution
tuoi_khach = 70
la_khach_quen = True

if tuoi_khach > 65 and la_khach_quen:
    print("Tặng bác một chén trà.")
```

```python title=test
# Chấm bằng OUTPUT: khách 70 tuổi và là khách quen, nên câu tặng trà phải hiện
# ra. Người học chưa biết viết assert nên khối này chỉ khẳng định chương trình
# chạy được tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm giữa `if` và dấu hai chấm — chỗ đó chỉ chứa được **một** câu trả lời đúng/sai. Mà bạn đang có hai câu hỏi cần hỏi cùng lúc.
- kind: strategy
  body: Realm 0 đã cho bạn một từ nối buộc hai câu hỏi có–không thành một, và cụm ấy chỉ cho `True` khi cả hai vế cùng đúng. Vế trái là câu hỏi về tuổi; vế phải là cái tên đang giữ sẵn một giá trị đúng/sai, không cần so sánh gì thêm.
- kind: one-line
  body: "Viết `tuoi_khach > 65 and la_khach_quen` vào chỗ trống, giữ nguyên dấu hai chấm."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: Tặng bác một chén trà.
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba cửa hay một cửa đòi ba thứ — với mình là một. Với mắt bạn thì gọn hơn hẳn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nếu khối lồng nào cũng viết lại được bằng `and`, và viết bằng `and` thì bớt
được mấy tầng thụt lề, thì còn cần lồng làm gì nữa?

Thử lấy giấy bút làm một phép đổi. Đoạn dưới đây có hai tầng, và tầng trong có
thêm một `else`:

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
