---
id: nen-tang.list-dict-set-tuple.chi-xin-cac-gia-tri
title: Chỉ xin phần giá trị
summary: `chi.values()` đưa ra riêng đống số tiền, không kèm khoá — và một cái tên có sẵn tên là `sum` cộng cả đống ấy trong một dòng.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.dict-values]
requires: [core.dict-iter-keys, core.dict, ctrl.for-each, core.accumulator, core.builtin-function, core.fstring, core.variable]
concepts: [ctrl.lap, core.gia-tri, core.ham]
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
Có lúc mình chẳng cần cái nhãn nào. Mình chỉ cần thứ nằm trong ngăn.
::::

::::explain{#cai-nhan-thanh-duong-vong}
Bài trước kết bằng một chỗ vướng, và nó nằm gọn trong ba dòng này:

```python title=readonly
tong = 0
for khoa in chi:
    tong = tong + chi[khoa]
```

Đọc thật chậm dòng giữa và dòng cuối. Máy đưa cho bạn một cái nhãn. Việc đầu
tiên — và cũng là việc duy nhất — bạn làm với cái nhãn ấy là đem nó đi mở ngăn,
rồi vứt nó đi. Cái nhãn không sai, nó chỉ là một đường vòng: máy vốn đang đứng
ngay cạnh cái ngăn mà nó lại đưa bạn tờ giấy dán ngoài.

Cuốn sổ tra cứu mở ra được nhiều hơn một lối. Lối bạn đã đi là `chi.keys()` —
phần nhãn. Lối thứ hai là `chi.values()` — **chỉ phần nằm trong ngăn**.

```python title=readonly
for tien in chi.values():
    tong = tong + tien
```

Mỗi lượt, cái tên lặp mang thẳng một số tiền, và không có cái nhãn nào đi kèm.

Một điểm khác đáng nhớ: `for khoa in chi:` có lối viết gọn vì đi qua khoá là
việc hay làm nhất, còn phần giá trị thì **không** có lối gọn nào. Muốn nó thì
phải gọi tên nó ra: `chi.values()`.
::::

::::example{#ba-con-so}
Vẫn cuốn sổ ba khoản của bài trước:

```python title=readonly
chi = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}

print(chi.values())

for tien in chi.values():
    print(tien)
```

Máy in ra:

```text
dict_values([500000, 25000, 300000])
500000
25000
300000
```

Dòng đầu là dòng đáng dừng lại. Máy không in ra một danh sách trơn `[500000,
25000, 300000]` mà bọc nó trong chữ `dict_values` — cách máy nói "đây là phần
giá trị của một cuốn sổ tra cứu, không phải một danh sách rời". Bạn không cần
làm gì với chữ ấy; chỉ cần biết rằng gặp nó trên màn hình nghĩa là bạn đang
nhìn đúng thứ mình xin.

Ba dòng sau là ba số tiền, đúng thứ tự các khoá trong sổ. Không cái nhãn nào
xuất hiện, và cũng không cần cặp ngoặc vuông nào để mở ngăn.

Cộng chúng lại thì đúng cái khuôn cộng dồn quen thuộc:

```python title=readonly
chi = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}

tong = 0
for tien in chi.values():
    tong = tong + tien

print(tong)
```

Máy in ra `825000`, vì `500000 + 25000 + 300000 = 825000`.
::::

::::explain{#mot-cai-ten-co-san-an-ca-dong}
Bốn dòng để cộng một cuốn sổ vẫn là bốn dòng, và bạn sẽ chép lại chúng vào mỗi
tháng sau.

Nhưng ở mạch Hàm bạn đã biết một chuyện: máy dọn sẵn cả trăm cái tên trước khi
bạn viết dòng đầu tiên, và mỗi cái tên ấy là một hàm người khác đã viết xong.
`len` là một trong số đó — đưa cho nó cả một danh sách, nó đưa lại một con số.

`sum` là một cái tên có sẵn cùng loại: đưa cho nó một đống **số**, nó cộng hết
rồi đưa lại tổng.

```python title=readonly
chi = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}

print(sum(chi.values()))
```

Máy in ra `825000`. Bốn dòng thu về một.

Chỗ phải cẩn thận nằm ở chữ **số**. `sum` cộng đúng thứ bạn đưa cho nó, không
tự đoán bạn muốn phần nào của cuốn sổ. Đưa `chi.values()` thì nó nhận được ba
con số. Còn đưa thẳng `chi`, thứ nó nhận được là ba cái nhãn — và bạn vừa thấy
ở bài trước chuyện gì xảy ra khi một con số bị bảo cộng với một cái tên.
::::

::::predict{#dua-nham-cho-sum commitOnce}
Byte gõ hai dòng, dòng trên quên mất phần `.values()`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
chi = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}

print(sum(chi))
print(sum(chi.values()))
```

:::opt{correct}
Không con số nào in ra — máy dừng ngay ở dòng đầu với `TypeError`
:::

:::opt
825000, rồi 825000 lần nữa
::why
Gần đúng ở chỗ bạn đọc `sum(chi)` là "cộng cả cuốn sổ", và đó là cách đọc tự
nhiên nhất — trong tiếng Việt lẫn trong đầu người viết, "cộng cuốn sổ chi tiêu"
đương nhiên nghĩa là cộng tiền.

Chỗ lệch: máy không đọc được ý định ấy. Đưa thẳng cuốn sổ cho một thứ đi qua
từng phần tử thì thứ nó nhận được là các **khoá** — đúng luật bài trước, và
`sum` cũng đi qua từng phần tử như một vòng `for`. Nó khởi đầu tổng bằng số `0`
rồi cộng `"sửa xe"` vào, nên nó dừng ở đó.
::
:::

:::opt
Dòng đầu in `0`, dòng sau in 825000
::why
Gần đúng ở một chỗ thật: `sum` của một đống rỗng đúng là `0` — không có gì để
cộng thì tổng bằng không, và đó là câu trả lời hợp lý chứ không phải hỏng hóc.

Chỗ lệch là `sum(chi)` không rơi vào trường hợp ấy. Cuốn sổ này có ba khoá, nên
`sum` có đúng ba thứ để cộng; vấn đề là ba thứ ấy là chữ chứ không phải số. Nó
dừng vì gặp thứ không cộng được, không phải vì không gặp gì.
::
:::

:::opt
Dòng đầu báo `TypeError`, rồi dòng sau vẫn in 825000
::why
Gần đúng ở chỗ bạn đọc đúng cả hai dòng: dòng trên hỏng, dòng dưới thì viết
hoàn toàn đúng và tự nó sẽ cho ra 825000.

Chỗ lệch nằm ở chuyện xảy ra sau khi máy nổ lỗi: chương trình dừng hẳn ngay tại
dòng ấy, mọi dòng phía dưới không được chạy lượt nào. Thông báo lỗi trông như
một dòng in ra bình thường, nhưng nó là dấu chấm hết chứ không phải một dòng
trong bản báo cáo — đúng như `ValueError` bạn gặp ở Realm 0.
::
:::
::::

::::code{#so-sanh-hai-thang}
Byte đưa hai cuốn sổ và hỏi một câu: tháng này tiêu hơn tháng trước bao nhiêu?

Hai chỗ trống là cùng một kiểu, chỉ khác tên cuốn sổ. Hai cuốn cố ý khác nhau —
sổ tháng trước có hai khoản, sổ tháng này có ba — nên một đáp án chỉ tình cờ
đúng ở cuốn trên sẽ lộ ra ở cuốn dưới.

```python title=starter
thang_nay = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}
thang_truoc = {"cà phê": 30000, "xăng xe": 120000}

tong_nay = sum(___)
tong_truoc = sum(___)
chenh = tong_nay - tong_truoc

print(f"Tháng này: {tong_nay} đồng")
print(f"Tháng trước: {tong_truoc} đồng")
print(f"Tháng này tiêu hơn {chenh} đồng")
```

```python title=solution
thang_nay = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}
thang_truoc = {"cà phê": 30000, "xăng xe": 120000}

tong_nay = sum(thang_nay.values())
tong_truoc = sum(thang_truoc.values())
chenh = tong_nay - tong_truoc

print(f"Tháng này: {tong_nay} đồng")
print(f"Tháng trước: {tong_truoc} đồng")
print(f"Tháng này tiêu hơn {chenh} đồng")
```

```python title=test
# Chấm trên CẢ HAI cuốn sổ. Một cuốn thì chưa nói lên điều gì — con số của nó
# có thể được gõ cứng vào. Hai cuốn khác số khoản, khác số tiền, thì chỉ một
# lời xin đúng phần giá trị mới ra được cả ba con số dưới đây.
assert tong_nay == 825000, "sổ tháng này có ba khoản — 500000, 25000 và 300000 — cộng lại là 825000"
assert tong_truoc == 150000, "sổ tháng trước chỉ có hai khoản — 30000 và 120000 — cộng lại là 150000"
assert chenh == 675000, "lấy 825000 của tháng này trừ 150000 của tháng trước thì còn 675000"
```

:::hints
- kind: attention
  body: '`sum` phải được đưa cho một đống số. Nhưng thứ đang nằm trong `thang_nay` là cả một cuốn sổ tra cứu, mà cuốn sổ thì có hai phần: phần nhãn dán ngoài ngăn và phần nằm trong ngăn.'
- kind: strategy
  body: Bài này vừa cho thấy đưa thẳng cuốn sổ cho `sum` thì nó nhận được các khoá và dừng lại. Nên chỗ trống phải là lời xin đúng một phần của cuốn sổ — phần các con số — viết bằng dấu chấm đặt sau tên sổ, và có cặp ngoặc rỗng ở cuối vì đây là một lời gọi. Hai chỗ trống hỏi cùng một câu, chỉ đổi tên cuốn sổ.
- kind: one-line
  body: 'Chỗ trên viết `thang_nay.values()`, chỗ dưới viết `thang_truoc.values()`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Tháng này tiêu hơn 675000 đồng
- tier: output
  expect: Tháng trước: 150000 đồng
- tier: static
  onFail: mỗi chỗ trống phải xin phần giá trị của một cuốn sổ, không gõ cứng con số nào
  requireAst:
  # Hai lời gọi `.values()` — mỗi cuốn sổ một lời. Khung chưa có lời nào.
  - kind: uses-call, target: values, min: 2
  # `thang_truoc` chỉ được ĐẶT tên trong khung, chưa được đọc lần nào; lời giải
  # phải đọc nó ở chỗ trống thứ hai.
  - kind: uses-name, target: thang_truoc, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Xin thẳng phần mình cần. Không phải mở từng ngăn một nữa.
::::

::::explain{#ba-cau-hoi-mot-cuon-so}
Gói lại: **một cuốn sổ tra cứu trả lời được nhiều hơn một câu hỏi**, và bạn
phải nói rõ mình hỏi câu nào.

- `chi.keys()` — cho tôi phần nhãn. Đây là câu hỏi mặc định, nên `for khoa in
  chi:` viết gọn được.
- `chi.values()` — cho tôi phần nằm trong ngăn. Không có lối viết gọn; phải gọi
  ra.

Số lượt của cả hai lối đều bằng **số khoá** trong sổ. Sổ ba khoá thì `.keys()`
đưa ra ba cái nhãn và `.values()` đưa ra ba con số — hai đống dài bằng nhau,
xếp cùng một thứ tự.

> Chỗ dễ vấp: viết `chi.values` mà quên cặp ngoặc. Máy không báo lỗi — nó chỉ
> đưa cho bạn chính cái hàm ấy chứ không đưa kết quả, và `sum` của một cái hàm
> thì mới là chỗ nổ. Cặp ngoặc rỗng là thứ nói "làm đi", giống hệt `len()` và
> `input()` bạn đã gọi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Tổng thì ra trong một dòng rồi. Nhưng bản báo cáo cuối tháng lại cần **cả tên
lẫn tiền** trên cùng một dòng — *sửa xe hết 500000 đồng* — chứ không phải một
con số trơ trọi.

Quay về `for khoa in chi:` rồi tra ngược `chi[khoa]` thì vẫn đúng hai lần việc,
đúng chỗ vướng bài trước để lại.

Cuốn sổ đã cho xin riêng phần nhãn, cho xin riêng phần tiền. Xin luôn cả **cặp**
được không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
