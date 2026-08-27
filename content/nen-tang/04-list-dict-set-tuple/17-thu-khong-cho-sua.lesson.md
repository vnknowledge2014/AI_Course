---
id: nen-tang.list-dict-set-tuple.thu-khong-cho-sua
title: Thứ không cho sửa
summary: Gán vào một ô của tuple là `TypeError` — tuple bất biến, và chính chỗ đó giữ cho tên khoản không rời khỏi số tiền của nó.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.tuple-immutable]
practices: [core.list-index, core.arithmetic, core.fstring]
requires: [core.tuple, core.string-immutable, core.list, core.list-append, core.list-index, core.dict, core.dict-items, err.type-error, err.traceback, core.arithmetic, core.fstring]
concepts: [core.gia-tri, core.bien, core.loi-khi-chay]
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
Đọc ô nào mình cũng đưa. Dán đè lên một ô thì mình chịu.
::::

::::explain{#tam-the-da-in-roi}
Bài trước kết bằng một câu gõ dở: `cap[1] = 450000`. Chạy lên thì máy từ chối,
và lời từ chối ấy bạn đã nghe một lần rồi.

Quay lại tấm thẻ giá treo trên kệ. Hai ô đã in xong, mực đã khô. Muốn đổi con
số dưới ô tiền thì không ai lấy bút xoá đi rồi ghi đè lên tấm cũ — người ta in
**một tấm mới**, gỡ tấm cũ xuống, treo tấm mới lên.

Tính chất ấy có tên, và bạn đã gặp đúng tên ấy ở mạch Giá trị — biến — kiểu, lúc
thử kéo chữ đầu của một chuỗi thành chữ hoa: giá trị **bất biến**, tiếng Anh là
*immutable*, nghĩa đen là *không đổi được*. Chuỗi bất biến. Và điều bài này thêm
vào: **tuple cũng bất biến** — gán vào một ô bên trong nó là việc máy không nhận.

Chỗ này đáng tách cho rõ, vì hai chuyện hay bị gộp làm một:

- **Cái tên** thì gỡ ra dán sang giá trị khác được. Viết `cap = ("sửa xe",
  450000)` là hoàn toàn hợp lệ — bạn dựng một cặp mới rồi cho cái tên `cap` trỏ
  sang nó, y như tháo tấm thẻ cũ xuống và treo tấm mới lên cùng cái móc.
- **Cái giá trị** — tấm thẻ — thì không sửa được một ô nào. `cap[1] = 450000` là
  việc đó, và đó là việc bị từ chối.

Danh sách không như vậy: `.append` ở Realm 0 làm chính cái danh sách đang có dài
thêm một món, không phải dựng danh sách mới. Cùng là "một dãy có thứ tự", nhưng
một bên sửa tại chỗ được, một bên thì không.
::::

::::example{#may-tu-choi-o-dau}
Đây là câu hỏi bỏ ngỏ của bài trước, viết thành code chạy được:

```python title=readonly
cap = ("sửa xe", 500000)

print(cap[1])
cap[1] = 450000
print("dòng này có chạy không?")
```

Chạy lên, dòng đầu êm ru, dòng thứ ba nổ:

```text
500000
Traceback (most recent call last):
  File "so_chi_tieu.py", line 4, in <module>
    cap[1] = 450000
    ~~~^^^
TypeError: 'tuple' object does not support item assignment
```

Ba chuyện đáng nhìn kỹ:

- **`500000` in ra được.** Nên chuyện `cap[1]` chỉ đúng vào ô tiền vẫn là thật.
  Máy không hề quên cách đọc; nó chỉ từ chối đúng một việc là **dán đè**.
- **Dòng cuối nói thẳng việc bị từ chối**, dịch sát từng mảnh: *`'tuple'` — thứ
  nằm bên trái là một cái gói; `does not support` — không nhận; `item
  assignment` — phép gán vào một ô bên trong.* Vẫn là `TypeError`, đúng cái tên
  bạn gặp từ Realm 0: sai không nằm ở cú pháp mà nằm ở **kiểu** của thứ đem ra
  dùng. Đổi chữ `'tuple'` thành `'str'` thì đây là nguyên văn thông báo bạn đã
  gặp khi thử sửa một ký tự của chuỗi.
- **Câu hỏi in ở dòng cuối cùng không bao giờ hiện ra.** Máy dừng hẳn tại chỗ
  nổ. Còn mười dòng phía dưới thì mười dòng đó không được chạy.

Đây là loại lỗi **kêu thành tiếng**. Nó khó chịu lúc đang gõ, nhưng nó là loại
lỗi tử tế nhất: bạn biết ngay mình vừa làm gì sai và sai ở dòng số mấy.
::::

::::predict{#doan-in-duoc-may-dong commitOnce}
Byte thử lại lần nữa, lần này kẹp câu lệnh sửa vào giữa hai câu thông báo.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
cap = ("sửa xe", 500000)

print("trước khi sửa")
cap[1] = 450000
print("sau khi sửa")
```

:::opt{correct}
Hiện `trước khi sửa`, rồi một thông báo TypeError, và không có dòng `sau khi sửa`
:::

:::opt
Hiện cả hai dòng, và cặp đã đổi thành `('sửa xe', 450000)`
::why
Gần đúng ở chỗ bạn suy ra từ một điều đã học chắc: `cap[1]` đọc được, mà đọc
được thì thường cũng ghi được — với danh sách thì đúng là như thế thật, và
danh sách là chỗ chứa bạn quen nhất tới lúc này.

Chỗ lệch nằm ở loại chỗ chứa. Tuple là giá trị bất biến: đọc thì mở, ghi thì
đóng. Máy không lặng lẽ sửa, cũng không lặng lẽ bỏ qua — nó dừng hẳn ở đúng
dòng đó, nên dòng `print` nằm dưới không tới lượt chạy.
::
:::

:::opt
Không hiện dòng nào cả, vì máy phát hiện lỗi trước khi bắt đầu chạy
::why
Gần đúng ở chỗ có một loại lỗi hành xử đúng như bạn tả: lỗi cú pháp. Gõ thiếu
dấu hai chấm thì máy không chạy dòng nào hết, kể cả dòng đầu tiên — bạn đã gặp
kiểu đó ở Realm 0.

Chỗ lệch: `cap[1] = 450000` viết **đúng cú pháp**. Máy chỉ vỡ lẽ khi chạy tới
nơi và nhìn thấy thứ bên trái là một tuple. Nên nó chạy bình thường từ trên
xuống, in xong câu đầu, rồi mới dừng — đó là lý do dòng `trước khi sửa` kịp
hiện ra.
::
:::

:::opt
Hiện cả hai dòng, nhưng cặp vẫn là `('sửa xe', 500000)` vì máy bỏ qua dòng sửa
::why
Gần đúng ở chỗ bạn giữ đúng nửa quan trọng: cái cặp **không** đổi, và điều đó
chính xác.

Chỗ lệch là ở cách máy từ chối. Máy không bỏ qua trong im lặng rồi đi tiếp —
im lặng đi tiếp là điều tệ nhất một cái máy có thể làm với bạn, vì chương trình
sẽ chạy tới hết với dữ liệu bạn tưởng đã sửa mà thật ra chưa. Ở đây nó kêu lên
và dừng lại, nên `sau khi sửa` không bao giờ in ra.
::
:::
::::

::::explain{#vi-sao-do-la-diem-manh}
Nghe qua thì "không cho sửa" giống một chỗ thiếu. Với cặp tên–tiền thì nó là chỗ
mạnh nhất, và lý do nằm ở vết thương của bài hai dãy song song.

Ở đó bạn có hai danh sách đi cạnh nhau, `ten` và `tien`, nối với nhau bằng chỗ
đứng — khoản thứ ba của dãy này đi với khoản thứ ba của dãy kia. Sợi dây ấy chỉ
nằm trong đầu bạn, máy không hề biết nó có. Xoá một tên mà quên xoá
tiền thì hai dãy lệch nhau, và máy in tên một đằng tiền một nẻo **mà không kêu
một tiếng nào** — bạn chỉ biết khi đọc bản báo cáo và thấy con số vô lý, có khi
là một tháng sau.

Một cặp thì không thế. `("sửa xe", 500000)` là **một** giá trị, không phải hai
thứ đứng gần nhau. Không ai gỡ được một nửa của nó ra thay bằng thứ khác — máy
chặn đúng ở đó. Muốn đổi thì phải dựng một tấm thẻ mới, và lúc dựng thì cả hai
ô đều nằm trước mắt bạn trên cùng một dòng, khó mà sửa nửa này rồi quên nửa kia.

Nói cho gọn: hai dãy song song để ngỏ một cách sai âm thầm. Cặp bất biến đổi cái
cách sai ấy lấy một tiếng kêu.
::::

::::code{#dung-mot-tam-the-moi}
Byte muốn thử một phép tính: **nếu** hai khoản dưới đây đắt thêm 5000 đồng mỗi
khoản thì tấm thẻ sẽ ghi con số nào? Chỉ là thử — hai tấm thẻ gốc phải còn
nguyên, vì sổ chi tiêu ghi đúng số tiền đã tiêu.

Mà ô tiền của cặp cũ thì không sửa được. Nên việc phải làm là **dựng một cặp
mới** cho mỗi khoản: giữ nguyên tên khoản lấy ra từ cặp cũ, và số tiền thì
**tính ra từ số tiền cũ** chứ không gõ thẳng con số mới vào.

Hai chỗ trống, hai khoản khác nhau — nên một câu trả lời chỉ tình cờ đúng cho
khoản trên sẽ lộ ra ngay ở khoản dưới.

```python title=starter
cu_1 = ("cà phê", 25000)
cu_2 = ("bún bò", 40000)

moi_1 = ___
moi_2 = ___

print(moi_1)
print(moi_2)
```

```python title=solution
cu_1 = ("cà phê", 25000)
cu_2 = ("bún bò", 40000)

moi_1 = (cu_1[0], cu_1[1] + 5000)
moi_2 = (cu_2[0], cu_2[1] + 5000)

print(moi_1)
print(moi_2)
```

```python title=test
# Mỗi chỗ trống có câu canh của riêng nó, và hai câu đòi hai kết quả khác
# nhau — nên không có một hằng số nào điền được vào cả hai chỗ mà qua được.
assert moi_1 == ("cà phê", 30000), "khoản cà phê đang ghi 25000, thêm 5000 thì tấm thẻ thử ghi 30000, và ô tên vẫn phải là chuỗi 'cà phê'"
assert moi_2 == ("bún bò", 45000), "khoản bún bò đang ghi 40000, thêm 5000 thì tấm thẻ thử ghi 45000, và ô tên vẫn phải là chuỗi 'bún bò'"
assert cu_1 == ("cà phê", 25000), "tấm thẻ gốc của khoản cà phê phải còn nguyên ('cà phê', 25000) — dựng thẻ thử là tạo thêm một giá trị, không đụng gì tới thẻ cũ"
```

:::hints
- kind: attention
  body: Một tấm thẻ mới cũng viết y như tấm cũ — ngoặc tròn, hai thứ, một dấu phẩy ở giữa. Chỉ có điều lần này bạn không gõ sẵn hai thứ đó ra, mà lấy chúng từ tấm thẻ đang cầm trên tay.
- kind: strategy
  body: Ô tên của cặp mới chính là ô tên của cặp cũ, đọc ra bằng chỉ số. Ô tiền là ô tiền của cặp cũ đem cộng thêm 5000, cũng đọc ra bằng chỉ số rồi mới cộng. Hai chỗ trống viết cùng một hình dạng, chỉ khác cái tên của cặp cũ.
- kind: one-line
  body: "Viết `(cu_1[0], cu_1[1] + 5000)` vào chỗ trống thứ nhất, và `(cu_2[0], cu_2[1] + 5000)` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: ('cà phê', 30000)
- tier: static
  onFail: cặp mới phải dựng TỪ cặp cũ — lấy tên ra bằng chỉ số và cộng thêm vào số tiền cũ, chứ không gõ thẳng con số mới
  requireAst:
  # Mỗi lời giải đọc `cu_1` hai lần (ô tên, ô tiền) và `cu_2` hai lần. Dòng
  # dựng cặp cũ không được tính vào, vì ở đó cái tên mới chỉ được đặt ra.
  - kind: uses-name, target: cu_1, min: 2
  - kind: uses-name, target: cu_2, min: 2
  forbidAst:
  # Con số mới phải được TÍNH ra, không được chép vào.
  - kind: has-literal, target: 30000
  - kind: has-literal, target: 45000
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tấm cũ còn nguyên, tấm mới đã treo. Không ai sửa lén nửa nào cả.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cặp giữ tên dính chặt lấy tiền, và máy chặn tay bạn nếu định gỡ một nửa. Đổi
lại, mọi lần muốn đụng tới nội dung, bạn phải viết `cap[0]` và `cap[1]`.

Thử đọc to đoạn này lên:

```python
print(f"{cap[0]} tiêu hết {cap[1]} đồng")
```

Câu ấy chạy đúng, nhưng nó không tự nói ra mình đang nói về cái gì. Ô 0 là tên
hay là tiền? Bạn phải nhớ. Và nếu một hôm bạn gõ nhầm thành `cap[1]` ở chỗ đáng
lẽ là `cap[0]`, thì máy vẫn in ra một dòng đầy đủ, đúng chính tả, chỉ có điều
tên và tiền đứng ngược chỗ — **không một tiếng báo nào**, đúng cái kiểu sai âm
thầm mà bài này vừa nói là đáng sợ nhất.

Vậy thay vì cầm cả tấm thẻ rồi mỗi lần lại đếm ô, có cách nào đặt tên cho từng
ô **ngay lúc nhận** cặp không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
