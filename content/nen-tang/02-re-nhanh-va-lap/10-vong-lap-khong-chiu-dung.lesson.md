---
id: nen-tang.re-nhanh-va-lap.vong-lap-khong-chiu-dung
title: Vòng lặp không chịu dừng
summary: Điều kiện không tự đổi ý — thân vòng phải có một dòng đẩy nó dần tới sai, thiếu dòng đó là lặp mãi mãi.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ctrl.loop-progress]
requires: [ctrl.while, ctrl.while-check-timing, ctrl.if, core.reassign, core.fstring]
concepts: [ctrl.lap, core.bien]
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
Mình đọc lại điều kiện rất chăm chỉ. Nhưng mình không tự làm nó đổi.
::::

::::explain{#doc-lai-thi-doc-duoc-gi}
Bài trước để lại một vòng lặp mà suốt thân không có dòng nào động tới cái tên
nằm trong điều kiện:

```python
to_con_lai = 3

while to_con_lai > 0:
    print("Múc một tô cho khách")
```

Câu hỏi là: đầu lượt sau, máy đọc lại điều kiện thì đọc được gì?

Đọc được **đúng con số cũ**. `to_con_lai` vẫn là 3, vì không dòng nào trong
thân đổi nó. Nên `3 > 0` vẫn ra `True`, máy lại vào thân, lại in một dòng, lại
quay lên đọc `3 > 0`, lại `True`.

Không có lượt nào khác lượt nào. Vòng này chạy tới khi bạn bảo nó dừng.

Chỗ dễ hiểu nhầm: nghĩ rằng chữ `while` tự nó biết đếm, hoặc máy đủ tinh ý để
nhận ra "mình đang lặp lại y hệt, chắc là hỏng rồi". Không. Máy đọc điều kiện,
thấy `True`, và làm đúng thứ bạn viết. Nó không so lượt này với lượt trước, và
nó không có ý niệm nào về chuyện đủ hay chưa đủ.

Giới lập trình gọi cái vòng này là **vòng lặp vô hạn** (tiếng Anh: *infinite
loop*).

Điều kiện của một `while` chỉ đổi được khi có ai đó đổi nó. Người duy nhất làm
được việc đó là chính thân vòng. Vậy trong thân phải có một dòng đẩy cái tên
trong điều kiện **tiến dần về phía làm điều kiện sai** — bài này gọi nó là
**bước tiến**.

Bà chủ quán múc phở thì nồi vơi đi. Chính chuyện nồi vơi là bước tiến: sau đủ
nhiều tô, nồi cạn, và câu "còn nước dùng thì múc" trở thành sai. Nếu vừa múc ra
lại vừa chan ngược vào nồi đúng chừng ấy, bà sẽ múc tới sáng mai.

Một bước tiến hợp lệ phải đủ ba điều, và thiếu bất kỳ điều nào cũng ra cùng một
kết quả:

- **Nằm trong thân vòng.** Đặt nó sau vòng thì nó chỉ chạy khi vòng đã xong —
  mà vòng thì không bao giờ xong.
- **Chạm đúng cái tên có trong điều kiện.** Sửa một cái tên khác thì điều kiện
  không hay biết gì.
- **Đi đúng hướng.** `to_con_lai` phải nhỏ dần thì `to_con_lai > 0` mới có ngày
  sai. Cộng thêm mỗi lượt là chạy càng lúc càng xa cái đích.
::::

::::example{#hai-doan-chi-khac-mot-dong}
Hai đoạn dưới đây khác nhau đúng một dòng.

Đoạn thiếu bước tiến:

```python title=readonly
to_con_lai = 3

while to_con_lai > 0:
    print("Múc một tô cho khách")

print("Treo biển nghỉ")
```

Màn hình:

```text
Múc một tô cho khách
Múc một tô cho khách
Múc một tô cho khách
Múc một tô cho khách
Múc một tô cho khách
```

…và cứ thế, không có dòng cuối. Dòng `Treo biển nghỉ` không bao giờ được in,
vì muốn xuống tới nó thì vòng phải kết thúc trước đã.

Đoạn có bước tiến — thêm đúng một dòng:

```python title=readonly
to_con_lai = 3

while to_con_lai > 0:
    print("Múc một tô cho khách")
    to_con_lai = to_con_lai - 1

print("Treo biển nghỉ")
```

Màn hình:

```text
Múc một tô cho khách
Múc một tô cho khách
Múc một tô cho khách
Treo biển nghỉ
```

Ba lượt rồi dừng. Đầu lượt bốn, máy đọc `0 > 0` và được `False`.

**Khi lỡ chạy phải một vòng vô hạn.** Bạn không phải chờ nó. Màn hình chạy có
một nút **Dừng** — bấm vào đó là chương trình ngắt ngay tại chỗ. Ở terminal thì
tổ hợp phím tương đương là `Ctrl` + `C`.

Và có một chuyện đáng nhớ hơn cả cách dừng: máy **không hề báo lỗi**. Không
`SyntaxError`, không `NameError`, không dòng chữ đỏ nào. Với máy, đoạn code kia
hoàn toàn hợp lệ — bạn bảo lặp chừng nào điều kiện còn đúng, và điều kiện thì
còn đúng thật. Đây là loại sai không ai nhắc bạn, nên nó nằm trong danh sách
những thứ bạn phải tự soi mỗi khi viết `while`: *thân vòng có đang đẩy điều
kiện đi đâu không?*
::::

::::predict{#de-danh-mua-xe commitOnce}
Byte để dành mua xe đạp 500 nghìn. Trong heo đất đã có 100 nghìn, mỗi tháng để
dành thêm 100 nghìn nữa.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
da_de_danh = 100000

while da_de_danh < 500000:
    print("Chưa đủ, để dành tiếp")
    thang_nay = 100000

print("Mua xe đạp")
```

:::opt{correct}
Dòng "Chưa đủ, để dành tiếp" in ra mãi không dứt — phải tự bấm Dừng
:::

:::opt
In "Chưa đủ, để dành tiếp" bốn lần rồi in "Mua xe đạp"
::why
Gần đúng ở chỗ bạn nhẩm đúng bài toán ngoài đời: 100 nghìn đã có, mỗi tháng
thêm 100 nghìn, thì tháng thứ tư là chạm 500 nghìn. Đó chính là con số người
viết đoạn này muốn thấy.

Chỗ lệch nằm ở dòng `thang_nay = 100000`. Nó có đổi một cái tên thật, nhưng cái
tên ấy là `thang_nay` — trong khi điều kiện lại hỏi về `da_de_danh`. Hai cái
tên khác nhau thì không liên quan gì tới nhau: `da_de_danh` vẫn nằm nguyên ở
100000 từ đầu tới cuối. Bước tiến phải chạm đúng cái tên có mặt trong điều
kiện.
::
:::

:::opt
In "Chưa đủ, để dành tiếp" đúng một lần rồi in "Mua xe đạp"
::why
Gần đúng ở chỗ bạn thấy dòng `thang_nay = 100000` chạy xong là thân đã hết
việc, nên nghĩ vòng cũng hết luôn. Suy nghĩ ấy đúng cho `if`: vào thân, chạy
xong, đi tiếp.

Chỗ lệch là điểm khác biệt duy nhất giữa `if` và `while`. Hết thân, `if` đi
xuống dòng dưới; `while` thì **quay lên đọc lại điều kiện**. Và lần đọc lại ấy
gặp đúng con số 100000 của lần trước, nên nó lại cho `True` một lần nữa.
::
:::

:::opt
Máy báo lỗi vì nhận ra vòng lặp này không bao giờ dừng được
::why
Gần đúng ở chỗ bạn nhớ đúng một thói quen có thật của Python: gặp thứ nó không
hiểu thì nó dừng lại và nói ra, thay vì đoán bừa.

Chỗ lệch: ở đây không có gì để máy không hiểu. Từng dòng đều đúng cú pháp,
từng cái tên đều tồn tại, và câu lệnh bạn ra — *lặp chừng nào `da_de_danh` còn
dưới 500000* — máy làm được. Nó không cách nào biết bạn **định** cộng vào
`da_de_danh` mà lại gõ ra `thang_nay`. Muốn biết một vòng có dừng hay không thì
phải đoán trước tương lai của chương trình, và đó là việc máy không nhận.
::
:::
::::

::::code{#them-buoc-tien}
Vòng dưới đây đang thiếu đúng bước tiến. `thang` đã đếm sẵn số tháng trôi qua,
nhưng số tiền trong heo đất thì không ai đụng tới, nên vòng chạy mãi.

Byte để dành thêm **100 nghìn mỗi tháng**. Hãy điền dòng còn thiếu để heo đất
dày lên sau mỗi lượt.

```python title=starter
da_de_danh = 100000
thang = 0

while da_de_danh < 500000:
    thang = thang + 1
    ___
    print(f"Hết tháng {thang}: đã có {da_de_danh} đồng")

print(f"Đủ tiền mua xe sau {thang} tháng")
```

```python title=solution
da_de_danh = 100000
thang = 0

while da_de_danh < 500000:
    thang = thang + 1
    da_de_danh = da_de_danh + 100000
    print(f"Hết tháng {thang}: đã có {da_de_danh} đồng")

print(f"Đủ tiền mua xe sau {thang} tháng")
```

```python title=test
# Byte nhẩm tay: 100k sẵn có, mỗi tháng thêm 100k, tới hết tháng thứ tư là
# chạm 500k. Kiểm cả hai cái tên — số tháng phải đúng, và số tiền cuối cùng
# cũng phải đúng.
assert thang == 4, "heo đất mở màn với 100 nghìn, mỗi tháng dày thêm 100 nghìn, nên phải hết tháng thứ tư mới đủ tiền xe đạp"
assert da_de_danh == 500000, "chiếc xe đạp giá 500 nghìn, và tháng cuối cùng đưa heo đất chạm đúng con số ấy — thiếu một đồng thì chưa mua được xe"
```

:::hints
- kind: attention
  body: Dòng cần điền nằm trong thân vòng. Điều kiện đang hỏi về cái tên `da_de_danh` — hãy xem trong thân có dòng nào chạm tới đúng cái tên đó chưa.
- kind: strategy
  body: Bước tiến phải đẩy `da_de_danh` **lớn dần**, vì điều kiện là `da_de_danh < 500000` và nó chỉ sai khi số tiền đã lên tới 500 nghìn. Viết theo đúng lối mà dòng ngay trên đang dùng cho `thang`, chỉ khác cái tên và khác con số cộng thêm.
- kind: one-line
  body: Viết `da_de_danh = da_de_danh + 100000` vào chỗ trống, thụt vào bốn dấu cách cho thẳng hàng với hai dòng còn lại trong thân.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Hết tháng 1: đã có 200000 đồng
- tier: output
  expect: Đủ tiền mua xe sau 4 tháng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn lượt rồi hết. Có ai đẩy thì điều kiện mới có ngày sai được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bước tiến bạn vừa viết là dòng này:

```python
da_de_danh = da_de_danh + 100000
```

Và ngay trên nó, đếm số tháng, là dòng cùng một khuôn:

```python
thang = thang + 1
```

Bây giờ hãy đọc dòng thứ hai như một câu toán ở trường: *thang bằng thang cộng
một*. Trừ đi hai vế thì còn `0 = 1`. Không con số nào trên đời làm câu ấy đúng
được — trong toán, đây là một phương trình vô nghiệm.

Vậy mà máy nhận nó, chạy nó, và cho ra kết quả bạn muốn.

Nghĩa là dấu `=` trong Python **không** đọc như dấu bằng trong toán. Nó không
tuyên bố hai vế bằng nhau. Nó làm một việc gì đó, theo một thứ tự nào đó — và
chính cái thứ tự ấy giải thích vì sao một cái tên đứng được ở cả hai bên.

Máy đọc dòng đó ra sao? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
