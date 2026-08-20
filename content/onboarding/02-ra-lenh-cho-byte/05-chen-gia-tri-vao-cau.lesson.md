---
id: onboarding.ra-lenh-cho-byte.chen-gia-tri-vao-cau
title: Chèn giá trị vào giữa câu
summary: Viết trọn câu chữ một lần, chừa sẵn chỗ trống, rồi để máy tự điền giá trị vào những chỗ đó.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [core.fstring]
requires: [core.variable, core.string-literal]
concepts: [core.chuoi]
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
Viết cả câu ra một lần, chừa chỗ trống. Máy điền giá trị vào giúp bạn.
::::

::::explain{#to-phieu-in-san}
Ở bãi gửi xe trước cổng chợ, người trông xe không viết tay từng tờ vé. Người ta
in sẵn hàng nghìn tờ giống hệt nhau:

> **VÉ XE MÁY** — Biển số: .......... — Giờ vào: ..........

Phần chữ in sẵn một lần cho tất cả. Mỗi lần có xe vào, người trông xe chỉ điền
vào **hai chỗ chấm chấm**.

Cách nối chuỗi bằng dấu `+` mà bạn có từ trước giống việc làm ngược lại: cắt câu
thành từng mẩu chữ rời, rồi dán xen kẽ với giá trị. Bạn phải tự nhớ chừa dấu
cách ở đầu mẩu, tự đếm dấu nháy, và tới chỗ nào là số thì chương trình gãy vì
`TypeError` — như bài trước bạn vừa thấy.

Python có sẵn kiểu "tờ phiếu in sẵn". Nó tên là **f-string** — câu chữ có chỗ
chừa, và máy tự điền vào lúc chạy.
::::

::::example{#f-dau-tien}
Đây là câu chào ở cuối bài trước, viết theo lối tờ phiếu:

```python title=readonly
ten_khach = "Lan"
tuoi_khach = 25
print(f"Chào {ten_khach}, bạn {tuoi_khach} tuổi")
```

Máy in ra:

```text title=readonly
Chào Lan, bạn 25 tuổi
```

Đọc dòng thứ ba từ trái sang phải:

- Chữ **`f`** dán **sát** trước dấu nháy mở, không có dấu cách ở giữa. `f` là
  chữ đầu của *format*, tiếng Anh nghĩa là "điền theo mẫu". Chính chữ này báo
  cho máy: *câu chữ sau đây có chỗ chừa, nhớ nhìn kỹ*.
- Mọi thứ **ngoài** cặp ngoặc nhọn vẫn là chữ đọc nguyên văn: `Chào `, rồi
  `, bạn `, rồi ` tuổi`. Luật cũ không đổi chút nào.
- **Trong** cặp ngoặc nhọn `{ }` là một **cái tên**. Gặp nó, máy làm đúng việc
  nó vẫn làm với một cái tên đứng ngoài nháy: đi tìm giá trị mà tên đó đang giữ,
  rồi đặt giá trị ấy vào chỗ chừa.

Và đây là chỗ f-string đỡ cho bạn nhiều nhất: `{tuoi_khach}` đang giữ số `25`,
mà câu vẫn in ra bình thường, **không** `TypeError`. Ngay tại chỗ chừa, máy tự
viết con số thành chữ giúp bạn.

Để thấy khác biệt, đây là cùng câu đó viết bằng dấu `+`:

```python title=readonly
print("Chào " + ten_khach + ", bạn " + tuoi_khach + " tuổi")
```

```text title=readonly
TypeError: can only concatenate str (not "int") to str
```

Bốn mẩu chữ, ba dấu cộng, và vẫn gãy ở chỗ con số. Tờ phiếu in sẵn không có
chuyện đó.

Chỗ chừa muốn có bao nhiêu cũng được — một chỗ, hai chỗ, hay năm chỗ trong cùng
một câu:

```python title=readonly
quan = "Phở Thìn"
mon = "tái nạm"
gia = 60000
print(f"{quan}: một tô {mon} giá {gia} đồng")
```

```text title=readonly
Phở Thìn: một tô tái nạm giá 60000 đồng
```
::::

::::predict{#quen-chu-f commitOnce}
Đoạn dưới **thiếu** chữ `f`. **Trước khi bấm chạy**, bạn đoán màn hình hiện ra
gì?

```python title=readonly
mon = "phở tái"
print("Hôm nay ăn {mon}")
```

:::opt{correct}
Hôm nay ăn {mon}
:::

:::opt
Hôm nay ăn phở tái
::why
Gần đúng ở chỗ bạn đã nắm chắc ý nghĩa của `{mon}`: đó là một chỗ chừa, và giá
trị cần điền vào là `"phở tái"`. Phần hiểu đó chính xác.

Chỗ lệch: cặp ngoặc nhọn chỉ trở thành chỗ chừa **khi có chữ `f`** đứng trước
dấu nháy mở. Thiếu `f` thì câu này quay về luật cũ nhất của cả khoá — trong
nháy là chữ, đọc nguyên văn — và hai dấu ngoặc nhọn cũng chỉ là hai ký tự chữ
như mọi ký tự khác.

Đây là lỗi mà người học Python nào cũng mắc vài lần trong tuần đầu. Nó không
báo lỗi, nó chỉ in ra sai — nên phải tự nhớ nhìn chữ `f`.
::
:::

:::opt
Hôm nay ăn
::why
Gần đúng ở chỗ bạn hình dung máy có thử nhìn vào cặp ngoặc nhọn, thấy không điền
được, nên bỏ trống chỗ đó. Suy luận ấy hợp lý nếu máy có một bước "cố điền xem
sao".

Nhưng thiếu chữ `f` thì máy không hề có bước đó. Nó không nhìn vào trong ngoặc
nhọn, không đi tìm cái tên nào cả. Với nó, cả câu chỉ là một dãy ký tự để đọc
lại y nguyên — nên không có gì bị bỏ đi cả.
::
:::

:::opt
Máy báo lỗi, vì `mon` bị kẹt trong dấu nháy
::why
Gần đúng ở chỗ bạn đang dùng đúng cái trục quan trọng nhất của cả module trước:
trong nháy là chữ, ngoài nháy là tên máy phải đi tìm. Bạn nhớ luật ấy là rất
tốt.

Chỗ lệch nằm ở hậu quả. Máy chỉ báo lỗi khi nó **không hiểu** bạn viết gì. Ở đây
nó hiểu hoàn toàn: bạn đưa cho `print` một câu chữ, và nó đọc câu chữ đó ra. Kết
quả không như bạn muốn, nhưng với máy thì không có gì trục trặc cả.
::
:::
::::

::::explain{#hai-luat-van-con-nguyen}
Hai điều đáng ghi lại trước khi bạn tự viết:

**Chữ `f` phải dính sát dấu nháy.** `f"Chào {ten}"` chạy được. `f "Chào {ten}"`
— có dấu cách ở giữa — thì máy không đọc nổi và báo `SyntaxError`, loại lỗi nó
phát hiện trước cả khi chạy dòng nào.

**Trong ngoặc nhọn phải là cái tên máy tìm được.** Gõ nhầm thành `{ten_khac}`
trong khi bạn đặt tên là `ten_khach`, máy sẽ đi tìm và không thấy — rồi báo
`NameError`, đúng loại lỗi bạn đã gặp khi viết một cái tên mà chưa dán nó lên
giá trị nào.

Nói cách khác: bên trong `{ }`, mọi luật về **tên** vẫn y nguyên. Chỉ có một
thứ mới, là chỗ chừa.
::::

::::code{#hoa-don-mot-dong}
Quán phở in cho khách một dòng hoá đơn. Tên món và giá đã nằm sẵn trong hai cái
tên.

Hãy điền vào chỗ trống để máy in ra đúng dòng: `Một tô phở tái giá 45000 đồng`

```python title=starter
ten_mon = "phở tái"
gia = 45000
print(___)
```

```python title=solution
ten_mon = "phở tái"
gia = 45000
print(f"Một tô {ten_mon} giá {gia} đồng")
```

```python title=test
# Chấm bằng OUTPUT: dòng hoá đơn phải khớp từng chữ, kể cả dấu cách.
pass
```

:::hints
- kind: attention
  body: Chỗ trống cần đúng **một** câu chữ — một cặp dấu nháy duy nhất, không có dấu `+` nào. Trong câu đó có hai chỗ sẽ được điền: tên món và giá.
- kind: strategy
  body: Cứ viết trọn câu như bạn muốn đọc thấy trên hoá đơn. Sau đó thay chỗ tên món bằng `{ten_mon}`, thay chỗ giá bằng `{gia}`. Cuối cùng nhìn lại dấu nháy mở — nó còn thiếu một chữ cái đứng sát trước.
- kind: one-line
  body: "Viết `f\"Một tô {ten_mon} giá {gia} đồng\"` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: Một tô phở tái giá 45000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=point-editor}
Một câu viết một lần, dùng cho mọi món. Đổi giá thì hoá đơn tự đúng theo.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Câu chào của bạn giờ tự điền tên, tự điền tuổi. Nhưng để ý kỹ: dù ai đứng trước
máy, **khuôn câu vẫn y một khuôn**. Chỗ chừa đổi giá trị, còn lời thì không đổi.
Cụ bảy mươi tuổi và cháu bé tám tuổi đều nghe đúng một câu như nhau.

Quán phở thật thì không nói vậy. Quán có tấm biển: *khách từ 18 tuổi trở lên
được nhận phiếu tích điểm*. Người bán nhìn khách rồi mới quyết định nói câu nào.

Muốn máy nói **khác đi** với khách trên 18 tuổi, trước hết nó phải trả lời được
một câu mà câu trả lời chỉ có đúng hai khả năng: *khách này có từ 18 tuổi trở
lên không?* Không có "hơi hơi", không có "gần được".

Máy trả lời loại câu hỏi hai khả năng ấy bằng cách nào, và câu trả lời của nó
trông ra sao khi in lên màn hình?

Đừng trả lời vội. Bài sau là về hai từ mà máy dùng để nói "đúng" và "sai".
::::

::::checkpoint{mastery=0.8}
::::
