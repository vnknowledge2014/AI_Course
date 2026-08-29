---
id: nen-tang.re-nhanh-va-lap.lap-khi-chua-biet-may-luot
title: Lặp khi chưa biết mấy lượt
summary: Có loại việc không đếm trước được số lượt — chỉ nói được điều kiện để còn lặp tiếp.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ctrl.while]
requires: [ctrl.if, ctrl.for-range, ctrl.for-each, ctrl.comparison, core.reassign, core.input, core.input-returns-str, core.fstring, err.name-error]
concepts: [ctrl.lap, ctrl.re-nhanh]
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
Số lượt thì mình không biết trước. Nhưng lúc nào thì thôi, bạn nói được.
::::

::::explain{#khong-ai-dem-truoc}
Bài trước để lại câu hỏi: muốn hỏi lại cho tới khi khách gõ đúng thì
`range(n)` cần một con số `n` — mà `n` bằng bao nhiêu?

Thử đi tìm nó xem có tìm được không. Khách này gõ đúng ngay lần đầu, `n` là 1.
Khách kia bấm Enter suông ba lần rồi mới gõ số, `n` là 4. Con số ấy chỉ lộ ra
**sau khi** chuyện đã xảy ra, còn `range(n)` thì đòi bạn viết nó ra từ lúc còn
đang gõ code, trước khi có khách nào bước vào quán.

Chuyện này không có gì lạ ngoài đời. Bà chủ quán phở sáng nào cũng nấu một nồi
nước dùng rồi bán. Không ai dặn *"sáng nay múc đúng 47 tô"*. Lời dặn thật là:

> Còn nước dùng thì còn bán.

Việc lặp lại vẫn y hệt — múc một tô, đưa khách, múc tô nữa. Chỉ có chỗ dừng là
đổi: không phải một con số đếm sẵn, mà một **điều kiện** được xem lại.

Python có đúng một câu lệnh cho lối dặn đó: `while`. Tiếng Anh nghĩa là *trong
khi*, *chừng nào còn*. Đọc `while` thành *"chừng nào ... còn đúng thì làm"* là
đọc đúng nó.

Nhìn hình dạng của một dòng `while`, bạn thấy quen ngay:

```python
while to_con_lai > 0:
    print("Múc một tô cho khách")
```

- `while` — từ khoá, đứng đầu dòng.
- `to_con_lai > 0` — một câu đúng-sai, đúng loại câu bạn vẫn viết sau `if`.
- Dấu hai chấm cuối dòng, bắt buộc.
- Thân thụt vào bốn dấu cách.

Bốn chỗ ấy giống `if` từng nét. Khác đúng một điều, và đó là toàn bộ bài hôm
nay: `if` chạy xong thân thì đi tiếp xuống dưới, còn `while` chạy xong thân thì
**quay lại** dòng điều kiện.
::::

::::example{#noi-nuoc-dung}
Nồi sáng nay còn ba tô:

```python title=readonly
to_con_lai = 3

while to_con_lai > 0:
    print("Múc một tô cho khách")
    to_con_lai = to_con_lai - 1

print("Hết nước dùng, treo biển nghỉ")
```

Màn hình hiện ra:

```text
Múc một tô cho khách
Múc một tô cho khách
Múc một tô cho khách
Hết nước dùng, treo biển nghỉ
```

Đi lại đúng đường máy đi:

- `to_con_lai` là 3. Câu hỏi `3 > 0` cho `True`, máy vào thân: in một dòng, rồi
  hạ `to_con_lai` xuống 2. Hết thân, máy quay lại dòng `while`.
- `2 > 0` cho `True`. In dòng nữa, `to_con_lai` còn 1. Quay lại.
- `1 > 0` cho `True`. In dòng thứ ba, `to_con_lai` còn 0. Quay lại.
- `0 > 0` cho `False`. Máy không vào thân nữa, nó nhảy xuống dòng đầu tiên nằm
  sau vòng lặp và in câu treo biển.

Thử đổi đúng một chữ: viết `if` thay cho `while`, giữ nguyên mọi dòng còn lại.
Màn hình chỉ còn hai dòng — một lần múc, rồi câu treo biển — và trong nồi vẫn
còn 2 tô. `if` hỏi một lần; `while` hỏi lại ở mỗi lượt.

Và đây là câu trả lời cho bài trước: con số lượt **không nằm ở đâu** trong đoạn
code cả. Nó là hệ quả của việc nồi có bao nhiêu tô. Với `while`, bạn thôi không
đếm lượt nữa — bạn nói ra điều kiện để còn được lặp tiếp.

Còn một chuyện nữa: chuyện hỏi lại khách hôm trước. Trước khi viết nó, phải biết
máy nhận về **cái gì** khi khách bấm Enter mà chưa gõ chữ nào.

Realm 0 ví `input` như người phụ quán đứng nghe khách rồi ghi lên tờ giấy: khách
nói *"hai lăm"*, tờ giấy có hai nét mực. Khách bấm Enter suông là khách chẳng
nói gì — người phụ quán vẫn đưa lại tờ giấy, chỉ là tờ giấy **không có nét mực
nào**. Python viết tờ giấy trắng ấy là `""`: hai dấu nháy dính nhau, ở giữa
trống không.

Không cần tin, hỏi thẳng máy:

```python title=readonly
tra_loi = input("Số tiền hôm nay: ")
print(tra_loi == "")
```

Người ngồi trước máy bấm Enter luôn, không gõ gì:

```text
Số tiền hôm nay: 
True
```

Vậy là bạn có sẵn một câu đúng-sai nói được ý *"khách chưa gõ gì"* — đúng loại
câu mà `while` cần:

```python title=readonly
tra_loi = input("Số tiền hôm nay: ")

while tra_loi == "":
    tra_loi = input("Số tiền hôm nay: ")

print(f"Đã ghi: {tra_loi}")
```

Đi lại đúng đường máy đi, với một khách bấm Enter suông hai lần rồi mới gõ
`50000`:

- Dòng `input` **đầu tiên** chạy trước khi vào vòng. Khách bấm Enter suông, nên
  `tra_loi` giữ `""`.
- **Đầu lượt 1** — `"" == ""` cho `True`. Vào thân: máy hỏi lại, khách lại bấm
  Enter suông, `tra_loi` vẫn `""`. Hết thân, quay lại dòng `while`.
- **Đầu lượt 2** — vẫn `True`. Máy hỏi lại, lần này khách gõ `50000`, `tra_loi`
  giữ `"50000"`. Hết thân, quay lại.
- **Đầu lượt 3** — `"50000" == ""` cho `False`. Máy không vào thân nữa, nó nhảy
  xuống và in `Đã ghi: 50000`.

Hai chỗ dễ vấp trong đoạn này:

- **Sao phải hỏi một lần trước khi vào vòng?** Vì dòng `while` cần có sẵn một câu
  trả lời để mà xem. Chưa hỏi lần nào thì `tra_loi` chưa tồn tại, mà cái chưa
  tồn tại thì máy không xem được — nó dừng ngay tại dòng `while` và báo
  `NameError`.
- **Sao dòng `input` xuất hiện hai lần?** Vì hai dòng ấy làm hai việc khác nhau:
  dòng ngoài hỏi lần đầu, dòng trong hỏi **lại**. Ở nồi nước dùng, con số mới của
  mỗi lượt đến từ một phép tính (`to_con_lai - 1`); ở đây nó đến từ người đang
  ngồi gõ.

Khách bấm Enter suông mấy lần thì máy hỏi lại đúng mấy lần. Không dòng nào
trong đoạn code có con số ấy, và cũng không cần.
::::

::::predict{#doan-may-luot commitOnce}
Byte có 100 nghìn trong ví, mỗi tô phở 45 nghìn. Byte ăn chừng nào còn đủ tiền
cho một tô nữa.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python title=readonly
tien_con = 100000

while tien_con >= 45000:
    print(f"Ăn một tô, còn {tien_con - 45000} đồng")
    tien_con = tien_con - 45000

print("Không đủ tiền tô nữa")
```

:::opt{correct}
Ăn một tô, còn 55000 đồng — Ăn một tô, còn 10000 đồng — Không đủ tiền tô nữa
:::

:::opt
Ba dòng ăn phở, rồi mới Không đủ tiền tô nữa
::why
Gần đúng ở chỗ bạn nhẩm 100 nghìn chia cho 45 nghìn và thấy nó hơn hai — cảm
giác "còn ăn được lần nữa" là có căn cứ, vì trong ví vẫn còn tiền thật.

Chỗ lệch nằm ở lần đọc điều kiện thứ ba. Lúc ấy `tien_con` là 10000, và câu hỏi
là `10000 >= 45000` — sai. Máy không bán chịu phần thiếu: điều kiện đòi đủ tiền
cho **trọn một tô** thì 10 nghìn không qua được cửa.
::
:::

:::opt
Một dòng ăn phở thôi, rồi Không đủ tiền tô nữa
::why
Gần đúng ở chỗ bạn đọc dòng `while` y như đọc một dòng `if` — và trông chúng
giống nhau tới mức đó là cách đọc tự nhiên nhất khi mới gặp `while`.

Chỗ lệch nằm ở chính chữ `while`: hết thân, máy **quay lại** dòng điều kiện chứ
không đi tiếp xuống dưới. `if` hỏi một lần rồi thôi; `while` hỏi lại mỗi khi
vừa làm xong một lượt.
::
:::

:::opt
In mãi không dừng, vì điều kiện đã đọc là đúng ngay từ đầu thì nó đúng mãi
::why
Gần đúng ở một chỗ quan trọng: điều kiện đúng là **được đọc lại**, chứ máy không
nhớ mãi câu trả lời của lần đọc đầu tiên. Bạn đang hỏi đúng câu cần hỏi.

Chỗ lệch: giữa hai lần đọc ấy có dòng `tien_con = tien_con - 45000` chạy, nên
lần đọc sau máy nhìn vào một con số khác. Cứ mỗi lượt, con số lại nhỏ đi một
tô, và tới lúc nó tụt xuống dưới 45 nghìn thì cửa đóng.

Còn *đọc lại vào lúc nào* thì đúng là chuyện đáng hỏi — bài sau nói riêng về nó.
::
:::
::::

::::explain{#chon-loi-lap-nao}
Từ giờ bạn có ba lối lặp, và mỗi lối hợp với một loại việc:

- **`for ... in range(n)`** khi số lượt đã biết trước lúc gõ code: in 30 dòng
  lịch tháng, hỏi đủ 7 ngày trong tuần.
- **`for mon in danh_sach`** khi có sẵn một danh sách và bạn muốn đi hết nó.
- **`while <điều kiện>`** khi số lượt chỉ hiện ra lúc chương trình đang chạy:
  chừng nào còn tiền, chừng nào khách còn bấm Enter suông, chừng nào nồi còn
  nước dùng.

Không lối nào cao cấp hơn lối nào. Chọn lối nào là trả lời một câu: *cái quyết
định lúc dừng là một con số bạn đếm được từ trước, hay một điều kiện phải xem
lại từng lượt?*

> Cẩn thận: `while` là câu lệnh đầu tiên có thể khiến chương trình chạy mãi
> không dừng. Để ý trong cả ba vòng lặp hôm nay, thân vòng luôn có một dòng
> làm thứ nằm trong điều kiện đổi đi — nồi vơi một tô, ví bớt một tô phở, câu
> trả lời được hỏi lại. Dòng đó quan trọng tới mức có hẳn một bài riêng cho nó.
::::

::::code{#mua-cho-toi-khi-het-tien}
Byte cầm 240 nghìn tiền chợ. Mỗi món ở chợ hết 60 nghìn, và Byte mua chừng nào
còn đủ tiền cho một món nữa.

Số món mua được thì bạn đừng đếm hộ máy — hãy viết điều kiện để nó tự dừng đúng
lúc. Điền vào chỗ trống.

```python title=starter
tien_cho = 240000

while ___:
    print("Mua thêm một món")
    tien_cho = tien_cho - 60000

print(f"Còn lại {tien_cho} đồng, không mua nữa")
```

```python title=solution
tien_cho = 240000

while tien_cho >= 60000:
    print("Mua thêm một món")
    tien_cho = tien_cho - 60000

print(f"Còn lại {tien_cho} đồng, không mua nữa")
```

```python title=test
# Ranh giới nằm ở đúng lượt thứ tư: lúc ấy trong túi còn vừa vặn 60 nghìn, và
# vừa vặn 60 nghìn thì vẫn mua được — nên tiêu hết sạch, còn lại 0 đồng.
# Điều kiện bỏ sót điểm bằng sẽ dừng sớm một lượt và để lại 60 nghìn.
assert tien_cho == 0, "240 nghìn tiền chợ chia vừa hết cho món 60 nghìn, nên mua tới lúc dừng thì trong túi không còn đồng nào; còn lại 60 nghìn là đã bỏ lỡ lượt mua cuối, lúc trong túi vừa vặn đủ đúng một món"
```

:::hints
- kind: attention
  body: Chỗ trống nằm giữa `while` và dấu hai chấm — chỗ ấy cần một câu đúng-sai, đúng loại câu bạn vẫn viết sau `if`. Câu đó phải nói về cái tên mà thân vòng làm thay đổi.
- kind: strategy
  body: Câu hỏi ở đầu mỗi lượt là "còn mua thêm được một món nữa không". Mua được nghĩa là số tiền chợ còn lại phải từ 60 nghìn trở lên — vừa đúng 60 nghìn thì vẫn mua được.
- kind: one-line
  body: "Viết `tien_cho >= 60000` vào chỗ trống, giữ nguyên dấu hai chấm cuối dòng."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Mua thêm một món\nMua thêm một món\nMua thêm một món\nMua thêm một món\nCòn lại 0 đồng, không mua nữa\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bạn không đếm lượt nữa. Bạn nói điều kiện, còn lượt thì mình đếm.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`while` viết y hệt `if` — cùng một từ khoá đứng đầu, cùng một câu đúng-sai, cùng
dấu hai chấm, cùng thân thụt vào. Khác đúng chỗ nó quay lại.

Vậy nó xem lại điều kiện vào lúc nào?

Nhìn thân vòng lúc nãy: nó có hai dòng, và dòng dưới trừ đi 60 nghìn. Ở lượt
cuối, ngay sau dòng trừ ấy, `tien_cho` tụt xuống 0 — điều kiện
`tien_cho >= 60000` lúc đó đã sai rồi. Thân vòng ấy vừa vặn hết ngay tại đó, nên
bạn không nhìn thấy được chuyện gì xảy ra tiếp theo.

Giờ thêm một dòng nữa vào **sau** dòng trừ, chẳng hạn một câu báo lại số tiền
còn trong túi. Ở lượt cuối, dòng ấy có được chạy không?

Hai cách hiểu đều nghe lọt tai:

- Máy canh chừng điều kiện liên tục, nên vừa thấy sai là dừng ngay giữa thân,
  bỏ luôn những dòng còn lại của lượt ấy.
- Máy đợi hết lượt rồi mới hỏi lại, nên mọi dòng trong thân vẫn chạy trọn.

Hai cách hiểu cho hai kết quả khác nhau trên màn hình. Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
