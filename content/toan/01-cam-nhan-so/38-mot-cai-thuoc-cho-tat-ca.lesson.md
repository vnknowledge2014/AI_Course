---
id: toan.cam-nhan-so.mot-cai-thuoc-cho-tat-ca
title: Một cái thước cho tất cả
summary: Chốt cứng mẫu ở 100 thì mọi lượng to nhỏ khác nhau đều đọc được bằng một con số — và 100 được chọn vì nó đủ mịn để tách những chênh lệch đời thường mà vẫn chỉ tốn hai chữ số.
locale: vi
track: toan
module: cam-nhan-so
order: 38
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.percent]
requires: [math.terminating-decimal, math.fraction-compare, core.arithmetic, core.division, core.float, core.variable, core.print-variable, ctrl.comparison]
concepts: [math.phan-so, math.thap-phan, math.so-sanh]
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
Mỗi lần so lại đi tìm một cái thước mới thì mệt. Chọn sẵn một cái đi.
::::

::::explain{#moi-lan-mot-thuoc-thi-met}
Câu hỏi còn treo từ bài trước: vườn Byte 40 cây mất 14, vườn An 250 cây mất 80
— vườn nào bị nặng hơn?

Bài 34 đã cho cách làm: muốn so thì phải **cùng thước**, và nó cho luôn cách
tìm thước chung — lấy mẫu bên này nhân mẫu bên kia thì luôn ra một cỡ thước mà
cả hai bên đều đếm chẵn được. Với `14/40` và `80/250` thì thước chung là
`40 × 250 = 10000`:

```text
14/40   =  3500/10000
80/250  =  3200/10000
```

`3500` cái thước so với `3200` cái thước, cùng một cỡ thước — vườn Byte nặng
hơn. Xong, và cách này không sai một chút nào.

Chỗ bất tiện lộ ra ở lần thứ hai. Chiều nay bác Tư sang khoe vườn mình: 60 cây
mất 20. So vườn bác Tư với vườn Byte thì thước chung là `60 × 40 = 2400` — một
cái thước **khác hẳn** cái vừa dùng. So bác Tư với An thì `60 × 250 = 15000`,
thêm một cái thước nữa. Ba cái vườn, ba lần so, ba cái thước.

Và không cái thước nào trong ba cái ấy dùng lại được cho lần sau. Cứ thêm một
cái vườn là lại đi tìm mẫu chung từ đầu.

Có một lối thoát, và nó đơn giản tới mức dễ bỏ qua: **thôi đừng tìm mẫu chung
nữa — chọn sẵn một mẫu, rồi bắt mọi thứ quy về đúng mẫu ấy.** Chọn xong thì
mọi cái vườn trên đời đều đọc được bằng một con số, và hai con số bất kỳ đặt
cạnh nhau là so được ngay.
::::

::::example{#vi-sao-lai-la-100}
Câu hỏi kế tiếp: chọn mẫu nào?

Mẫu ấy phải nằm trong họ mà bài 37 vừa dựng — 10, 100, 1000 — vì chỉ họ đó mới
thẳng cột sẵn, khỏi quy đồng.

**Thử mẫu 10.** Quy hai cái vườn về mẫu 10:

```text
14/40   =  3,5/10
80/250  =  3,2/10
```

Cả hai đều không ra **số nguyên** cái thước. Mà cái thước thì phải đếm được
bằng số nguyên — đó là toàn bộ ý nghĩa của bài 31: tử nói *lấy mấy cái*, và
"lấy ba cái rưỡi cái thước" thì thước đã bị bẻ thêm lần nữa rồi. Đếm chẵn bằng
thước `1/10` thì cả hai vườn đều ra "3 cái thước rồi thừa một mẩu", mà mẫu 10
không có cột nào để ghi cái mẩu ấy. Mẫu 10 quá thô: nó không viết nổi lượng của
vườn nào trong hai vườn, nên cũng chẳng tách được hai vườn ra.

**Thử mẫu 100.**

```text
14/40   =  35/100
80/250  =  32/100
```

`35` và `32`. Cả hai đều là số nguyên, và chúng khác nhau — mẫu 100 tách được
đúng cái mà mẫu 10 làm mất.

**Thử mẫu 1000.** Ra `350/1000` và `320/1000` — cũng đúng, cũng tách được. Chỉ
là với hai cái vườn này thì cột thứ ba rỗng: `350` và `320` đều thừa ra một chữ
số `0` không mang tin gì thêm, và con số nào cũng phải dài thêm một chữ số để
nuôi cái cột ấy.

Có lượng cần tới cột thứ ba thật — `1/8` là `0,125`, tức `12,5%`, đúng cái ca
bài 37 vừa dựng. Lúc đó người ta viết thêm phần lẻ sau dấu phẩy chứ không đổi
mẫu: mẫu vẫn chốt ở 100.

Vậy **100 là mức bẻ nhỏ đầu tiên đủ mịn mà vẫn viết được bằng số nguyên.** Nó
cũng chính là `10²` của bài 24 — bó-của-bó của bài 7, gom mười bó thành một bó
to đúng một lần.

Cái mẫu chốt cứng ấy có tên riêng, và bạn đã thấy nó cả nghìn lần: **phần
trăm**. Ký hiệu là `%`.

```text
1%  =  1/100   — một cái thước, cỡ một phần trăm
35% =  35/100  — ba mươi lăm cái thước ấy
```

Đọc lại hai cái vườn bằng tên mới: vườn Byte bị sâu ăn **35%**, vườn An **32%**.
Vườn Byte nặng hơn — dù nó chỉ mất 14 cây còn vườn An mất tới 80 cây.
::::

::::predict{#doan-vuon-nao-nang commitOnce}
Bác Tư vẫn còn ngồi đó, và vườn bác chưa được đo lần nào: **60 cây, sâu ăn 20
cây**. Vườn An thì vừa đo xong — **250 cây, sâu ăn 80 cây**, tức `32%`.

**Trước khi tính ra giấy**, bạn đoán vườn nào bị sâu ăn nặng hơn?

:::opt{correct}
Vườn bác Tư — hơn 33 trên 100, so với 32 trên 100 của vườn An
:::

:::opt
Vườn An, vì 80 cây chết nhiều hơn hẳn 20 cây
::why
Gần đúng ở chỗ bạn so hai con số **cùng đơn vị**, đúng luật bài 11 — cả hai đều
là "cây", và 80 cây thật sự là một tổn thất lớn hơn 20 cây. Nếu câu hỏi là
"nhà nào mất nhiều cây hơn" thì bạn trúng hoàn toàn, không bàn cãi gì.

Chỗ lệch nằm ở chữ **nặng hơn**. Nặng hơn là so phần chết với **chính cái vườn
chứa nó**: 20 trên 60 là đúng một phần ba, còn 80 trên 250 thì chưa tới một
phần ba — một phần ba của 250 là hơn 83 cây, mà vườn An mới mất 80. Hai câu hỏi
khác nhau, và chúng cho hai câu trả lời ngược nhau trên cùng một dữ liệu; đó là
lý do phải nói rõ đang hỏi câu nào.
::
:::

:::opt
Không so được, vì hai cái vườn to nhỏ khác nhau
::why
Gần đúng ở chỗ bạn nhớ đúng luật bài 34: `20/60` và `80/250` đang đo bằng hai
cái thước khác nhau, và so thẳng tử với tử, mẫu với mẫu thì cho ra kết quả bậy.
Cảnh giác ấy chặn được đúng cái lỗi phổ biến nhất về phân số.

Chỗ lệch: luật ấy cấm so **khi còn khác thước**, chứ không cấm **đổi về cùng
thước** rồi so. Bài 33 cho phép đổi thoải mái — nhân cả tử lẫn mẫu thì lượng
không suy suyển. Đo cả hai bằng cái thước `1/100` vừa chốt là được `33,33…` cái
thước so với `32` cái, cùng thước, so được ngay.
::
:::

:::opt
Bằng nhau — cả hai đều xấp xỉ một phần ba
::why
Gần đúng ở chỗ bạn ước lượng nhanh và ước lượng ấy trúng: `20/60` đúng bằng một
phần ba, còn `80/250` chỉ kém một phần ba một chút. Nghe qua điện thoại mà phải
trả lời ngay thì đây là câu trả lời tốt.

Chỗ lệch nằm ở **độ mịn của cái thước bạn dùng để ước lượng**. Chữ *xấp xỉ* đã
bỏ đi đúng cái chỗ chênh mà câu hỏi đang hỏi. Thước `1/10` cũng bỏ đi đúng chỗ
ấy: đếm chẵn bằng nó thì cả hai vườn đều ra "3 cái thước rồi thừa một mẩu", và
nhìn qua nó thì hai vườn dính làm một. Đây đúng là lý do người ta chốt mẫu ở
100 chứ không ở 10: chỗ chênh hơn một phần trăm chỉ hiện ra khi thước đủ nhỏ.
::
:::
::::

::::explain{#phan-tram-khong-xoa-duoc-cho-du}
Vườn bác Tư nặng hơn — và con số của nó lộ ra đúng cái giới hạn bài 37 vừa
dựng. `20/60` rút gọn (bài 33) là `1/3`. Mà `3` thì không kéo về 100 được: bài
37 đã chỉ ra chỗ đó bằng `999` với `1002`. Nên vườn bác Tư bị ăn `33,33…%`,
cái đuôi vẫn không dừng, và người ta viết gọn là "khoảng 33%".

Chốt mẫu ở 100 **không** phá được cái giới hạn ấy. Phần trăm không xoá được chỗ
dư — nó chỉ chọn sẵn cái thước, để khỏi phải chọn lại mỗi lần. Mà chừng ấy đã
đủ để trả lời câu vừa rồi: `33,33…` nhiều hơn `32`, khỏi cần biết cái đuôi dài
tới đâu.
::::

::::explain{#tu-so-cay-ra-phan-tram}
Cách quy về mẫu 100 mà không cần dò tìm số nhân: **nhân số phần lên 100 rồi
chia cho cái toàn thể**.

```text
14 × 100 = 1400,  1400 chia 40  = 35   →  35%
80 × 100 = 8000,  8000 chia 250 = 32   →  32%
```

Vì sao được phép làm thế? Vì đó vẫn đúng là bài 33 viết gọn lại. Muốn `14/40`
thành `?/100` thì cái `?` phải thoả: `?` đứng trên 100 mà bằng đúng `14` đứng
trên 40 — tức là `?` bằng `14` nhân với 100 rồi chia cho 40.

Một chuyện nhỏ về máy: Realm 0 đã nói phép chia `/` trong Python luôn cho ra số
**có phần thập phân**, kể cả khi chia hết — nên máy in `35.0` chứ không in `35`.
`35,0` phần trăm với `35` phần trăm là cùng một lượng.
::::

::::code{#do-hai-cai-vuon-bang-mot-thuoc}
Đo cả hai cái vườn bằng đúng một cái thước — mẫu 100 — rồi in ra hai con số.

Bài chấm bằng **cả hai** vườn, và hai vườn cho ra hai con số **khác nhau**:
điền cứng `35` vào cả hai chỗ thì vườn An sai, điền cứng `32` thì vườn Byte
sai. Chỉ một phép tính viết thật mới qua được cả hai.

```python title=starter
# Vườn Byte: 40 cây, sâu ăn 14
sau_byte = 14
ca_vuon_byte = 40
phan_tram_byte = ___

# Vườn An: 250 cây, sâu ăn 80
sau_an = 80
ca_vuon_an = 250
phan_tram_an = ___

print(phan_tram_byte)
print(phan_tram_an)
```

```python title=solution
# Vườn Byte: 40 cây, sâu ăn 14
sau_byte = 14
ca_vuon_byte = 40
phan_tram_byte = sau_byte * 100 / ca_vuon_byte

# Vườn An: 250 cây, sâu ăn 80
sau_an = 80
ca_vuon_an = 250
phan_tram_an = sau_an * 100 / ca_vuon_an

print(phan_tram_byte)
print(phan_tram_an)
```

```python title=test
# Chấm trên HAI cái vườn, vì một cái vườn thì không phân biệt nổi đúng với sai.
assert phan_tram_byte == 35, "14 trên 40 quy về mẫu 100 là 35 — tức 35%"
assert phan_tram_an == 32, "80 trên 250 quy về mẫu 100 là 32 — tức 32%"
assert phan_tram_byte > phan_tram_an, "cùng thước rồi mới so được: vườn Byte nặng hơn, dù nó mất ít cây hơn hẳn"
# Chốt luôn điều bài này bác bỏ: so thẳng số cây thì ra kết luận ngược.
assert sau_an > sau_byte, "vườn An mất nhiều cây hơn — mà vẫn là vườn nhẹ hơn"
```

:::hints
- kind: attention
  body: Mỗi vườn đã có sẵn hai cái tên ở ngay phía trên chỗ trống: số cây sâu ăn, và cả vườn có bao nhiêu cây. Chỗ trống cần dùng cả hai, cộng thêm con số 100.
- kind: strategy
  body: Quy về mẫu 100 gồm hai bước, làm theo đúng thứ tự đó là ra: nhân số phần lên 100 trước, rồi chia cho cả vườn. Dùng tên biến chứ đừng chép con số 14 hay 250 vào, vì hai dòng chỉ khác nhau ở tên.
- kind: one-line
  body: "Viết `sau_byte * 100 / ca_vuon_byte` vào chỗ trống thứ nhất, và `sau_an * 100 / ca_vuon_an` vào chỗ thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải quy về mẫu 100 bằng một phép chia thật — chép sẵn 35 và 32 vào thì bài chỉ đúng với đúng hai cái vườn này
  requireAst:
  # Khung khởi đầu không có dấu chia nào, nên `min: 2` chặn được đáp án chép
  # cứng hai con số kết quả — mỗi vườn phải có một phép chia của riêng nó.
  - kind: uses-operator, target: /, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^35\.0\n32\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
35 với 32. Một cái thước duy nhất, và mọi cái vườn đều lên cùng một hàng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bây giờ Byte có một con số dùng cho mọi cái vườn. Rất tiện — tiện tới mức dễ
quên mất nó đang đứng trên cái gì.

Năm ngoái cả hai vườn cùng bị sâu ăn đúng **30%**. Đếm lại thì thấy: vườn Byte
mất 12 cây trong 40 (`12/40 = 3/10 = 30/100`), vườn An mất 75 cây trong 250
(`75/250 = 3/10 = 30/100`). Hai con số phần trăm giống hệt nhau. Hai số cây thì
một bên 12, một bên 75 — chênh nhau hơn sáu lần.

Nếu ai đó chỉ nói với bạn "mất 30%" mà không nói gì thêm, bạn không có cách nào
biết đó là 12 cây hay 75 cây, hay 3000 cây.

Vậy con số 30 ấy còn thiếu điều gì mới đủ nghĩa? Bài sau trả lời — và câu trả
lời làm lộ ra một chuyện mà rất nhiều người lớn tính sai suốt đời.
::::

::::checkpoint{mastery=0.8}
::::
