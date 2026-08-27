---
id: nen-tang.list-dict-set-tuple.xoa-mot-khoan
title: Bỏ một khoản ra khỏi sổ
summary: '`.remove(gia_tri)` bỏ khoản theo nội dung chứ không theo chỗ đứng, và chỉ bỏ lần xuất hiện đầu tiên — gọi trên thứ không có thì máy dừng bằng `ValueError`.'
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.list-remove]
requires: [core.slice-copy, core.list-aliasing, core.list-slice, core.list, core.list-append, core.list-index, core.string-method, core.value-error, ctrl.for-each, ctrl.if, ctrl.comparison, core.fstring]
concepts: [core.danh-sach, core.sua-duoc, core.loi]
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
Ghi thêm thì mình gọi tên chỗ cuối. Lấy ra thì mình phải gọi tên đúng khoản.
::::

::::explain{#lay-ra-thi-chua-co-gi}
Bài trước để lại một chuyện có thật: hôm qua bạn ghi nhầm một khoản 500 nghìn
vào sổ. Nó đang nằm đó, và nó không được nằm ở đó.

Nhìn lại bộ đồ nghề. Ghi **thêm** vào sổ thì bạn có `.append` từ Realm 0. Lấy
một khoản **ra** thì... chưa có gì cả.

Cách xoay xở bằng thứ đang có thì được, nhưng phải dựng hẳn một cuốn sổ mới:

```python title=readonly
so = [85000, 500000, 240000]

so_moi = []
for khoan in so:
    if khoan != 500000:
        so_moi.append(khoan)

so = so_moi
print(so)
```

Năm dòng lệnh và một cuốn sổ nháp. Dòng cuối còn phải dán lại tấm nhãn `so`
sang cuốn mới — đúng thứ bài 4 vừa dạy bạn để ý. Tất cả chỉ để bỏ đúng một dòng.

Danh sách có sẵn một lệnh làm thẳng việc ấy:

```python title=readonly
so.remove(500000)
```

Đọc thành lời: *tìm trong sổ khoản 500000, rồi bỏ nó ra.*
::::

::::explain{#ba-dieu-phai-biet-ve-remove}
`.remove` gắn sau tên danh sách y như `.append` — cùng lối viết dấu chấm mà
mạch Giá trị đã gọi tên là **phương thức**. Nhưng có ba điều về nó phải biết
trước khi dùng, vì cả ba đều là chỗ người ta hay đoán sai.

**Một: nó xoá theo NỘI DUNG, không theo chỗ đứng.**

Con số trong ngoặc là **khoản tiền cần bỏ**, không phải ô thứ mấy. `so.remove(0)`
không có nghĩa "bỏ ô đầu tiên" — nó có nghĩa "tìm khoản có giá trị 0 rồi bỏ".
Đây là chỗ dễ vấp thật sự, vì từ bài 1 tới giờ mọi ngoặc vuông bạn gõ đều nói
về **chỗ đứng**: `so[0]` là ô đầu, `so[-1]` là ô cuối. `.remove` đổi hẳn cách
hỏi: bạn không nói ô nào, bạn nói **cái gì**.

**Hai: nó chỉ bỏ LẦN XUẤT HIỆN ĐẦU TIÊN.**

Sổ có hai dòng 500000 thì một lời gọi `.remove(500000)` bỏ đúng dòng nằm gần
đầu sổ hơn. Dòng kia ở lại. Muốn bỏ cả hai thì gọi hai lần.

Chỗ này khác hẳn đoạn vòng lặp ở trên: vòng lặp ấy xét từng khoản một và chép
sang mọi khoản không khớp, nên nó bỏ **mọi** dòng 500000 trong sổ. Hai cách
làm hai việc khác nhau, và bạn chọn theo việc mình cần — bỏ một dòng ghi nhầm,
hay dọn sạch mọi dòng loại ấy.

**Ba: nó sửa cuốn sổ tại chỗ.**

Giống `.append`, khác lát cắt. `.remove` không đưa ra cuốn sổ mới nào; nó mở
đúng cuốn mà cái tên đang dán vào rồi bỏ một dòng khỏi đó. Nên hai bài vừa rồi
vẫn còn nguyên giá trị: nếu hai cái tên cùng dán lên một cuốn, cả hai đều thấy
dòng ấy biến mất; muốn cuốn gốc yên thì cắt bản sao ra trước rồi hãy xoá.
::::

::::example{#hai-dong-nham-bo-tung-dong-mot}
Cuốn sổ dưới có **hai** dòng 500000. Đoạn code gọi `.remove` hai lần, in ra sau
mỗi lần.

```python title=readonly
so = [85000, 500000, 120000, 500000]

so.remove(500000)
print(so)

so.remove(500000)
print(so)
```

Máy in ra:

```text
[85000, 120000, 500000]
[85000, 120000]
```

Dòng in thứ nhất là dòng nói nhiều nhất. Sổ vẫn còn một khoản 500000 sau lời
gọi thứ nhất — chứng cứ tận mắt cho điều thứ hai ở trên: một lời gọi bỏ đúng
một dòng.

Và dòng bị bỏ là dòng **đứng trước**. Ban đầu 500000 nằm ở ô thứ hai và ô thứ
tư; sau lời gọi thứ nhất, ô thứ hai biến mất còn ô thứ tư ở lại, trượt lên
đứng cuối sổ. Lời gọi thứ hai mới dọn nốt nó.
::::

::::explain{#goi-tren-thu-khong-co}
Còn một trường hợp nữa, và nó là trường hợp bạn sẽ gặp trong đời thật: gọi
`.remove` trên một khoản **không có trong sổ**.

```python title=readonly
so = [85000, 120000]

so.remove(500000)

print("dòng này không bao giờ chạy")
```

Máy không lặng lẽ bỏ qua. Nó dừng ngay tại dòng ấy:

```text
Traceback (most recent call last):
  File "<stdin>", line 3, in <module>
ValueError: list.remove(x): x not in list
```

Loại lỗi này bạn đã gặp ở Realm 0: `ValueError` — **đúng loại việc, sai nội
dung**. Lúc đó là `int("hai lăm nghìn")`: đổi chữ thành số là việc `int` biết
làm, chỉ có nội dung đưa vào là không dùng được. Lần này cũng vậy: bỏ một
khoản khỏi danh sách là việc `.remove` biết làm, chỉ có khoản bạn nêu tên thì
danh sách không có.

Câu tiếng Anh cuối cùng nói đúng chuyện đó: giá trị bạn đưa vào không nằm
trong danh sách.

Điều đáng lo không phải là lỗi này khó hiểu. Điều đáng lo là **chỗ nó dừng**:
đúng tại dòng `.remove`, và mọi dòng phía sau không chạy nữa. Một chương trình
dọn sổ có 50 khoản cần bỏ, chạy tới khoản thứ 30 thì gặp một khoản sổ không
có: hai mươi khoản còn lại chưa ai dọn, và bản báo cáo cuối chương trình không
bao giờ được in.
::::

::::predict{#doan-sau-mot-lan-remove commitOnce}
Byte cắt một bản sao trước, rồi mới xoá trên cuốn gốc. Cuốn gốc có **hai** dòng
120000.

**Trước khi bấm chạy**, bạn đoán hai dòng cuối in ra gì?

```python title=readonly
so = [120000, 85000, 120000]
ban_sao = so[:]

so.remove(120000)

print(f"so      = {so}")
print(f"ban_sao = {ban_sao}")
```

:::opt{correct}
`so = [85000, 120000]` và `ban_sao = [120000, 85000, 120000]`
:::

:::opt
`so = [85000]` và `ban_sao = [120000, 85000, 120000]`
::why
Gần đúng ở chỗ bạn giữ chắc bài trước: `ban_sao` được cắt ra thành một cuốn
riêng nên nó không đổi, và bạn đọc nó chính xác.

Chỗ lệch nằm ở số dòng bị bỏ. Cách hiểu của bạn khớp với đoạn vòng lặp lọc ở
đầu bài — chép sang mọi khoản không khớp, nên mọi dòng 120000 đều rụng.
`.remove` thì không xét hết cả sổ: nó tìm tới dòng khớp **đầu tiên**, bỏ dòng
ấy, rồi thôi. Muốn bỏ cả hai thì phải gọi hai lần.
::
:::

:::opt
`so = [120000, 85000]` và `ban_sao = [120000, 85000, 120000]`
::why
Gần đúng ở chỗ bạn nhớ rằng một lời gọi chỉ bỏ đúng một dòng. Phần đếm của bạn
không sai.

Chỗ lệch là dòng nào bị bỏ. Đoán "dòng cuối" là suy luận rất tự nhiên, vì
`.append` làm việc ở cuối sổ nên người ta chờ `.remove` cũng vậy. Nhưng
`.remove` không làm việc ở một đầu cố định — nó dò từ đầu sổ đi xuống và dừng
lại ở dòng khớp đầu tiên gặp được. Dòng 120000 đầu tiên nằm ở ô thứ nhất, nên
chính nó rụng.
::
:::

:::opt
Cả hai đều là `[85000, 120000]`
::why
Gần đúng ở chỗ khó nhất của bài này: bạn tính ra `so` sau lệnh xoá hoàn toàn
chính xác — dòng 120000 đầu tiên rụng, còn lại 85000 và 120000.

Chỗ lệch là `ban_sao`. Dòng `ban_sao = so[:]` là một lát cắt, mà lát cắt thì
dựng ra một cuốn mới rồi chép sang. `.remove` sửa cuốn mà tên `so` đang dán
vào, nên cuốn kia không hề bị đụng. Nếu dòng ấy viết trần là `ban_sao = so`
thì đáp án của bạn mới đúng — và đó chính là hai bài vừa rồi đặt cạnh nhau.
::
:::
::::

::::code{#bo-dong-ghi-nham}
Sổ tháng Chín đang thừa một dòng. Hôm qua Byte chi 500000 đúng **một** lần,
nhưng lúc ghi sổ thì ghi thành hai dòng.

Hãy điền dòng còn thiếu để bỏ đúng **một** dòng 500000 ra khỏi sổ — dòng kia
là khoản chi thật, phải ở lại.

```python title=starter
so_thang_chin = [85000, 500000, 240000, 500000, 120000]

___

print(so_thang_chin)
```

```python title=solution
so_thang_chin = [85000, 500000, 240000, 500000, 120000]

so_thang_chin.remove(500000)

print(so_thang_chin)
```

```python title=test
# Cuốn sổ này có HAI dòng 500000, và đó là chỗ chấm thật. Gọi `.remove` hai
# lần thì khoản chi thật cũng mất, còn dựng vòng lặp lọc sạch mọi dòng 500000
# thì sổ chỉ còn ba dòng. Đúng MỘT lời gọi mới ra bốn dòng dưới đây.
assert so_thang_chin == [85000, 240000, 500000, 120000], "sau một lời gọi, sổ phải còn bốn dòng theo đúng thứ tự 85000, 240000, 500000, 120000: dòng 500000 đầu tiên bị bỏ, dòng 500000 thứ hai ở lại vì đó là khoản chi thật"
```

:::hints
- kind: attention
  body: Dòng cần điền đứng một mình, không có dấu bằng nào ở đầu. Nó gắn sau tên cuốn sổ bằng dấu chấm, đúng lối viết mà `.append` đã dùng — và thứ đặt trong ngoặc là số tiền của khoản cần bỏ, không phải ô thứ mấy.
- kind: strategy
  body: Sổ có hai dòng 500000 và bạn chỉ được bỏ một. May là một lời gọi vốn chỉ bỏ đúng lần khớp đầu tiên, nên bạn không phải làm gì thêm để giữ dòng kia — gọi đúng một lần là vừa đủ. Gọi hai lần thì khoản chi thật cũng mất, và câu kiểm sẽ thấy sổ chỉ còn ba dòng.
- kind: one-line
  body: 'Viết `so_thang_chin.remove(500000)` vào chỗ trống, sát lề trái như hai dòng còn lại.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: '[85000, 240000, 500000, 120000]'
- tier: static
  onFail: dòng bạn điền phải bỏ một khoản khỏi chính cuốn sổ `so_thang_chin`, không phải chép tay lại một danh sách ngắn hơn
  requireAst:
  # Khung đã đọc `so_thang_chin` đúng MỘT lần (trong dòng `print`). Lời giải
  # đúng đọc nó thêm một lần nữa ở dòng xoá, nên phải có ít nhất hai. Một đáp
  # án chép tay `[85000, 240000, 500000, 120000]` vẫn chỉ có một — và đáp án
  # ấy qua được câu `assert`, nên hai luật dưới đây là chỗ duy nhất bắt nó.
  - kind: uses-name, target: so_thang_chin, min: 2
  - kind: uses-call, target: remove, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bỏ đúng một dòng. Dòng kia là tiền thật, mình không đụng vào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hôm nay bạn có lệnh xoá, và bạn cũng thấy mặt trái của nó: gọi `.remove` trên
một thứ không có trong sổ thì chương trình dừng giữa chừng, ngay tại dòng ấy.
Mọi việc phía sau không chạy nữa.

Nên trước khi xoá, phải hỏi một câu: *sổ có khoản 500000 nào không?*

Với những gì đang có trong tay, bạn hỏi câu đó bằng cách duyệt cả sổ, so từng
khoản, và dựng một biến cờ để nhớ đã gặp hay chưa — `for`, `if`, thêm một cái
tên nữa, rồi mới tới lệnh xoá. Bốn dòng chuẩn bị cho một dòng làm việc.

Một câu hỏi trả lời có hay không, mà phải viết dài như vậy sao? Hay có cách hỏi
ngắn hơn?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
