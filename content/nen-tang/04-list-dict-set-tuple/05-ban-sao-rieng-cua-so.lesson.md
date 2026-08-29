---
id: nen-tang.list-dict-set-tuple.ban-sao-rieng-cua-so
title: Một bản của riêng mình
summary: Lát cắt không bao giờ đưa ra cuốn sổ cũ — nó dựng một cuốn mới rồi chép sang, nên `so[:]` cho ra một cuốn riêng, sửa được mà không đụng cuốn gốc.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.slice-copy]
requires: [core.list-aliasing, core.slice-open-end, core.list-slice, core.list, core.list-append, core.variable, core.assignment, core.fstring]
concepts: [core.danh-sach, core.bien, core.sua-duoc]
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
Mình cắt lấy cả cuốn. Thứ rơi ra tay mình là một cuốn khác.
::::

::::explain{#cat-ca-cuon-thi-ra-cai-gi}
Bài trước để lại một câu hỏi: `so[:]` — bỏ trống cả hai đầu lát cắt, tức là
"từ đầu tới hết" — đưa ra **chính cuốn cũ** hay một cuốn khác?

Đưa ra **một cuốn khác**. Và lý do thì bạn đã nhìn thấy từ bài 2, chỉ là chưa
gọi tên nó ra.

Nhớ lại lát cắt `so[0:3]` ở bài 2. Thứ máy đưa ra tay bạn chỉ có ba khoản,
trong khi cuốn sổ gốc dài bao nhiêu cũng được và nó không hề ngắn đi sau lát
cắt. Ba khoản ấy vì vậy không thể là cuốn cũ. Máy phải làm gì đó: nó **dựng
một danh sách mới**, rồi chép sang những khoản nằm trong khúc bạn chỉ.

Đó là việc mà lát cắt luôn làm, mọi lần, không có ngoại lệ. `so[0:3]` dựng một
cuốn mới rồi chép sang ba khoản. `so[-3:]` dựng một cuốn mới rồi chép sang ba
khoản cuối. Và `so[:]` cũng làm đúng như thế — chỉ khác ở chỗ khúc bạn chỉ là
cả cuốn, nên cuốn mới nhận đủ mọi khoản.

Cuốn mới ấy là một cuốn **riêng**: ghi thêm một dòng vào cuốn này thì cuốn kia
không dài ra, và bỏ một dòng khỏi cuốn kia thì cuốn này không ngắn đi.

Bạn vừa có thứ bài trước còn thiếu: một bản của riêng mình.
::::

::::example{#ba-ky-tu-doi-tat-ca}
Đặt hai dòng này cạnh nhau: `ban_sao = so` và `ban_sao = so[:]`. Chúng khác
nhau đúng **ba ký tự** — `[`, `:` và `]` gắn vào sau cái tên bên phải dấu bằng
— mà làm hai việc khác hẳn. Đoạn dưới dùng dòng thứ hai.

```python title=readonly
so = [85000, 240000, 120000]
ban_sao = so[:]

ban_sao.append(95000)

print(f"so      = {so}")
print(f"ban_sao = {ban_sao}")
```

Máy in ra:

```text
so      = [85000, 240000, 120000]
ban_sao = [85000, 240000, 120000, 95000]
```

Đặt cạnh bài trước thì thấy rõ hai lệnh khác nhau ở chỗ nào:

- `ban_sao = so` — máy đọc xem tên `so` đang dán vào cuốn nào rồi dán thêm tấm
  nhãn `ban_sao` lên **đúng cuốn ấy**. Một cuốn, hai nhãn.
- `ban_sao = so[:]` — máy dựng một cuốn **mới**, chép sang mọi khoản của cuốn
  cũ, rồi dán tấm nhãn `ban_sao` lên cuốn mới. Hai cuốn, mỗi cuốn một nhãn.

Nên ở đoạn trên, `.append` viết vào cuốn mới. Cuốn cũ chưa ai đụng tới, và nó
in ra đúng ba khoản như lúc đầu.

Một câu để nhớ, đi liền với câu của bài trước: **dấu bằng dán nhãn, lát cắt
dựng cuốn mới.**

> Sổ tay: `list(so)` cũng cho ra một cuốn mới với đúng những khoản của `so` —
> đó là cách viết thứ hai của cùng một việc, họ hàng với `int()` và `str()` mà
> Realm 0 đã dùng để đổi một giá trị sang loại khác. Bài này dùng `so[:]` vì
> nó nối thẳng vào lát cắt bạn vừa học, nhưng gặp `list(so)` trong code người
> khác viết thì bạn đã biết nó làm gì.
::::

::::predict{#doan-ba-cai-ten commitOnce}
Byte dựng ba cái tên cho một cuốn sổ: một bằng lát cắt, một bằng dấu bằng trần.
Rồi Byte ghi thêm một dòng qua cái tên gốc.

**Trước khi bấm chạy**, bạn đoán hai dòng cuối in ra gì?

```python title=readonly
so = [85000, 240000]
ban_sao = so[:]
ten_khac = so

so.append(120000)

print(f"ban_sao  = {ban_sao}")
print(f"ten_khac = {ten_khac}")
```

:::opt{correct}
`ban_sao = [85000, 240000]` và `ten_khac = [85000, 240000, 120000]`
:::

:::opt
Cả hai đều là `[85000, 240000, 120000]`
::why
Gần đúng ở chỗ bạn giữ chắc bài học hôm qua: một cái tên mới của danh sách cũ
thì nhìn thấy mọi dòng ghi thêm. Với `ten_khac` bạn đọc chính xác.

Chỗ lệch nằm ở `ban_sao`. `so[:]` không phải là một cách viết khác của `so` —
nó là một **lát cắt**, và lát cắt thì dựng ra một danh sách mới rồi chép sang.
Nó chỉ giống `so` ở chỗ nội dung trùng nhau lúc vừa cắt xong. Từ lúc ấy trở
đi, hai cuốn đi hai đường.
::
:::

:::opt
Cả hai đều là `[85000, 240000]`
::why
Gần đúng ở chỗ bạn nhận ra `ban_sao` là một cuốn riêng và dòng 120000 không
lọt sang được. Nửa ấy đúng.

Chỗ lệch là `ten_khac`. Dòng `ten_khac = so` là một dấu bằng trần, không có
lát cắt nào cả — nên nó chỉ dán thêm một tấm nhãn lên đúng cuốn mà `so` đang
dán vào. Hai cách viết trông gần giống nhau nhưng làm hai việc khác hẳn: có
`[:]` thì máy dựng cuốn mới, không có thì máy dán nhãn.
::
:::

:::opt
`ban_sao = [85000, 240000, 120000]` và `ten_khac = [85000, 240000]`
::why
Gần đúng ở chỗ khó nhất: bạn nhận ra hai cái tên này cư xử **khác nhau**, một
cái theo cuốn gốc và một cái đứng riêng. Đó chính là điều bài này chỉ ra, và
nhiều người đọc xong vẫn tưởng cả ba tên đi cùng một đường.

Chỗ lệch là bạn gán ngược vai. Cái đứng riêng là cái có `[:]`, vì lát cắt dựng
ra cuốn mới. Cái đi theo cuốn gốc là cái viết bằng dấu bằng trần, vì dấu bằng
chỉ dán nhãn. Đọc lại hai dòng ấy và tìm dấu ngoặc vuông: nó nằm ở dòng
`ban_sao`.
::
:::
::::

::::explain{#ban-nhap-de-nghich-thoai-mai}
Bản sao là thứ cho phép bạn **thử** mà không sợ hỏng.

Đây là hình dạng bạn sẽ gặp lại nhiều lần: cuốn sổ thật nằm yên một chỗ, còn
mọi phép nghịch — thêm khoản giả định, bớt khoản, xáo tung để xem thử — diễn
ra trên một bản nháp cắt ra từ nó. Nghịch xong mà thấy không ưng thì vứt bản
nháp đi, sổ thật chưa mất một dòng nào.

Nó cũng chính là câu trả lời cho chỗ đau bài trước. Muốn đưa cuốn sổ cho một
hàm nghịch thoải mái mà sổ thật vẫn nguyên, bạn không đưa `so` — bạn đưa
`so[:]`. Hàm nhận đúng cuốn mà lời gọi đưa vào, nên đưa bản nháp thì nó nghịch
trên bản nháp.

Một chỗ dễ vấp, và nó đáng nhớ vì cái sai không kêu lên tiếng nào: cắt ra bản
sao rồi mà vẫn ghi nhầm vào cuốn cũ thì mọi thứ vẫn chạy trơn tru, chỉ có sổ
thật bị sửa mất. Sau khi cắt, hãy để ý mỗi lệnh `.append` đang gắn sau cái tên
nào.
::::

::::code{#ban-nhap-cua-thang-tam}
Sổ tháng Tám đã chốt, ba khoản. Byte muốn thử xem **nếu** tháng này ghi thêm
một khoản 500000 thì sổ trông thế nào — thử thôi, sổ thật không được đụng tới.

Hãy điền vế phải của dòng còn thiếu, sao cho `nhap` là một cuốn riêng mang
đúng những khoản của sổ tháng Tám.

```python title=starter
so_thang_tam = [85000, 240000, 120000]

# Bản nháp để thử. Sổ thật phải còn nguyên ba khoản.
nhap = ___

nhap.append(500000)

print(f"Bản nháp: {nhap}")
print(f"Sổ thật:  {so_thang_tam}")
```

```python title=solution
so_thang_tam = [85000, 240000, 120000]

# Bản nháp để thử. Sổ thật phải còn nguyên ba khoản.
nhap = so_thang_tam[:]

nhap.append(500000)

print(f"Bản nháp: {nhap}")
print(f"Sổ thật:  {so_thang_tam}")
```

```python title=test
assert nhap == [85000, 240000, 120000, 500000], "bản nháp phải mang đúng ba khoản của sổ tháng Tám, rồi cộng thêm khoản 500000 vừa thử"
assert so_thang_tam == [85000, 240000, 120000], "sổ thật phải còn đúng ba khoản như lúc đầu — khoản 500000 chỉ được ghi vào bản nháp"
# Ghi thêm một dòng NỮA vào bản nháp, sau khi đoạn trên đã chạy xong: hai
# cuốn phải rời nhau ở mọi lần ghi, không phải chỉ rời nhau ở lần ghi đầu.
nhap.append(60000)
assert so_thang_tam == [85000, 240000, 120000], "ghi thêm 60000 vào bản nháp thì sổ thật vẫn phải là ba khoản cũ: hai cuốn rời nhau, không phải hai cái tên của một cuốn"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ở vế phải một dấu bằng, và cuốn sổ cần chép ra đã có tên sẵn ở dòng đầu. Bài trước đã chỉ ra rằng viết trần cái tên ấy vào đây thì bạn được thêm một cái tên, không được thêm một cuốn.
- kind: strategy
  body: Thứ dựng ra cuốn mới là lát cắt. Bạn cần cả cuốn chứ không phải một khúc, mà "cả cuốn" thì viết bằng cách bỏ trống cả hai đầu — bài 3 đã dạy nghĩa của mỗi đầu bỏ trống. Gắn cặp ngoặc vuông ấy vào ngay sau tên cuốn sổ.
- kind: one-line
  body: 'Viết `so_thang_tam[:]` vào chỗ trống, thành `nhap = so_thang_tam[:]`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Bản nháp: \[85000, 240000, 120000, 500000\]\nSổ thật:  \[85000, 240000, 120000\]\s*$
- tier: output
  expect: 'Sổ thật:  [85000, 240000, 120000]'
- tier: static
  onFail: bản nháp phải được cắt ra từ chính `so_thang_tam`, không phải chép tay lại ba con số
  requireAst:
  # Khung đã đọc `so_thang_tam` đúng MỘT lần (trong dòng `print` cuối). Lời
  # giải đúng đọc nó thêm một lần nữa ở vế phải dấu bằng, nên phải có ít nhất
  # hai. Một đáp án chép tay `[85000, 240000, 120000]` vẫn chỉ có một — và
  # đáp án ấy qua được cả ba câu `assert`, nên chỗ này là chỗ duy nhất bắt nó.
  - kind: uses-name, target: so_thang_tam, min: 2
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nghịch nát bản nháp cũng được. Sổ thật mình để yên một chỗ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bây giờ bạn có bản sao rồi: cắt ra một cuốn riêng, nghịch thoải mái, sổ thật
không suy suyển.

Nhưng hôm qua có chuyện khác. Bạn ghi nhầm một khoản 500 nghìn vào sổ — nhầm
thật, không phải thử. Khoản ấy đang nằm trong sổ và nó không được nằm ở đó.

Ghi **thêm** vào sổ thì bạn có `.append` từ Realm 0. Còn lấy một khoản **ra**
khỏi danh sách thì bằng gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
