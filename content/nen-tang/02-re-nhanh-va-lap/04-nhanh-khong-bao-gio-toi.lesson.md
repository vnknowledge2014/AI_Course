---
id: nen-tang.re-nhanh-va-lap.nhanh-khong-bao-gio-toi
title: Nhánh không bao giờ tới lượt
summary: Trong một chuỗi rẽ nhánh, nhánh rộng đứng trước nuốt mất nhánh hẹp đứng sau — và máy không kêu một tiếng nào.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ctrl.branch-order]
requires: [ctrl.elif, ctrl.if]
concepts: [ctrl.chuoi-dieu-kien, ctrl.nhanh-chet]
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
Nhánh trên đã đúng sẵn thì nhánh dưới có bao giờ được hỏi không?
::::

::::explain{#hang-ro-o-cho}
Bài trước để lại đúng câu hỏi đó: chuỗi `elif` dừng ở nhánh đúng đầu tiên, vậy
nếu nhánh đứng trước đã đúng với cả trường hợp mà nhánh sau định bắt thì sao?

Ở chợ đầu mối có một hàng ba cái rổ, mỗi rổ dán một cái nhãn. Người phân loại
cầm từng bó rau đi dọc hàng rổ, thấy rổ nào **vừa nhãn trước tiên** thì bỏ vào
rổ đó rồi quay lại lấy bó khác. Không ai đi tiếp xem rổ phía sau có vừa hơn
không — bỏ được rồi là xong bó ấy.

Bây giờ tưởng tượng hai cái nhãn được dán như thế này:

- Rổ thứ nhất: *trên 100 nghìn*
- Rổ thứ hai: *trên 200 nghìn*

Một ngày tiêu 250 nghìn đi dọc hàng rổ. Nó vừa nhãn rổ thứ nhất — trên 100
nghìn thì đúng là trên 100 nghìn thật — nên nó nằm lại đó.

Chuyện đáng nói không phải là ngày 250 nghìn ấy. Chuyện đáng nói là: **không có
ngày nào** rơi vào rổ thứ hai được. Muốn vừa nhãn *trên 200 nghìn* thì trước hết
phải trên 100 nghìn đã, mà hễ trên 100 nghìn là rổ thứ nhất giữ lại mất rồi.

Một nhánh nằm trong chương trình mà không giá trị nào tới được — người ta gọi nó
là **nhánh chết**.
::::

::::example{#xep-loai-mot-ngay}
Hàng rổ đó viết bằng Python, dùng sổ chi tiêu của Byte:

```python title=readonly
tien = 250000

if tien > 100000:
    print("Ngày tiêu nhiều")
elif tien > 200000:
    print("Ngày tiêu rất nhiều")
else:
    print("Ngày tiêu ít")
```

Máy in ra:

```text
Ngày tiêu nhiều
```

Đọc theo đúng thứ tự máy đọc: `250000 > 100000` cho ra `True` ngay ở nhánh đầu,
nên máy chạy dòng in bên dưới rồi bỏ qua toàn bộ phần còn lại của chuỗi. Dòng
`elif` không được hỏi lần nào.

Giờ thử đổi con số ở dòng đầu. Đổi thành `999999`, rồi `500000`, rồi `100001`:

```python
tien = 999999
```

Lần nào màn hình cũng hiện `Ngày tiêu nhiều`. Cả chương trình này chỉ in ra được
hai câu — `Ngày tiêu nhiều` và `Ngày tiêu ít`. Câu `Ngày tiêu rất nhiều` nằm
trong file, đọc được bằng mắt, mà không con số nào dẫn tới nó.

Đó là nhánh chết. Nó không nằm chết vì con số hôm nay xui, mà vì **thứ tự** hai
nhánh: nhánh rộng (dễ thoả) đứng trên nhánh hẹp (khó thoả).
::::

::::predict{#ngay-tieu-350-nghin commitOnce}
Vẫn chuỗi ba nhánh đó, đổi đúng một con số: hôm nay Byte tiêu 350 nghìn — vượt
cả hai ngưỡng.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python
tien = 350000

if tien > 100000:
    print("Ngày tiêu nhiều")
elif tien > 200000:
    print("Ngày tiêu rất nhiều")
else:
    print("Ngày tiêu ít")
```

:::opt{correct}
Chỉ một dòng: Ngày tiêu nhiều
:::

:::opt
Ngày tiêu rất nhiều
::why
Gần đúng ở chỗ bạn chọn nhánh **mô tả đúng nhất** ngày hôm nay. 350 nghìn thì
gọi là "rất nhiều" nghe hợp lý hơn hẳn, và `350000 > 200000` cũng là `True`
thật — phần tính toán đó bạn làm chính xác.

Chỗ lệch: máy không so xem nhánh nào mô tả khéo hơn. Nó đi từ trên xuống và dừng
ở nhánh đúng **đầu tiên**. Nhánh `> 100000` nằm trên, và với 350 nghìn thì nhánh
ấy cũng đúng, nên chuỗi kết thúc ngay tại đó.

Muốn máy nói "rất nhiều", bạn không sửa con số nào cả — bạn đổi chỗ hai nhánh.
::
:::

:::opt
Cả hai dòng: Ngày tiêu nhiều, rồi Ngày tiêu rất nhiều
::why
Gần đúng ở chỗ bạn kiểm cả hai điều kiện và thấy cả hai cùng đúng. Với hai câu
lệnh `if` viết rời nhau thì bạn đoán đúng hoàn toàn — hai `if` độc lập thì cái
nào đúng cái đó chạy, in ra hai dòng.

Chỗ lệch nằm ở chữ `elif`. Nó buộc các nhánh thành **một** chuỗi, và trong một
chuỗi thì đúng một nhánh được chạy. `elif` mang nghĩa "còn không thì mới hỏi
tiếp" — mà ở đây không còn chữ "còn không" nào, vì nhánh đầu đã đúng.
::
:::

:::opt
Máy báo lỗi vì nhánh thứ hai không bao giờ chạy được
::why
Gần đúng ở chỗ bạn nhìn ra nhánh thứ hai là nhánh chết. Nhìn ra được điều đó chỉ
bằng mắt là kỹ năng mà cả bài học này nhắm tới.

Chỗ lệch: Python không đi kiểm chuyện ấy giúp bạn. Mỗi dòng vẫn đúng ngữ pháp,
mỗi điều kiện vẫn cho ra `True` hoặc `False` đàng hoàng, nên chương trình chạy
trơn từ đầu tới cuối.

Nhánh chết không phải lỗi ngữ pháp — nó là lỗi **ý nghĩa**. Máy không có cách
nào biết bạn định xếp loại ba mức, nó chỉ thấy ba dòng điều kiện hợp lệ. Loại
sai này chỉ có mắt người bắt được, và đó là lý do bạn cần biết nó tồn tại.
::
:::
::::

::::explain{#hep-truoc-rong-sau}
Luật để không bao giờ tự tay tạo ra một nhánh chết:

- **Nhánh hẹp lên trước, nhánh rộng xuống sau.** Hẹp là điều kiện khó thoả —
  ít ngày lọt qua. `> 200000` hẹp hơn `> 100000`, nên nó phải đứng trên.
- **Cách kiểm nhanh bằng mắt:** đọc từng nhánh từ trên xuống và tự hỏi *"có giá
  trị nào tới được nhánh này không?"*. Nếu mọi giá trị thoả nhánh dưới đều đã
  thoả một nhánh nào đó ở trên, nhánh dưới là nhánh chết.
- **Thứ tự là một phần ý nghĩa của chương trình.** Trong một chuỗi `if / elif`,
  đổi chỗ hai nhánh là đổi kết quả — không phải chuyện sắp cho gọn mắt.

> Dễ nhầm: thứ tự chỉ quyết định trong **một chuỗi**. Nếu bạn viết hai câu `if`
> rời nhau, không dính `elif`, thì cả hai đều được hỏi và ngày 350 nghìn sẽ in
> ra hai dòng. Một chuỗi chọn một nhánh; hai `if` rời là hai quyết định độc lập.
::::

::::code{#xep-lai-hang-ro}
Sổ chi tiêu của Byte: hôm nay tiêu 250 nghìn. Byte muốn xếp loại ngày này thành
một trong ba mức, và mức đúng phải là `Ngày tiêu rất nhiều`.

Chuỗi ba nhánh đã viết sẵn, ba câu chữ cũng đã đúng chỗ. Chỉ còn hai con số
`100000` và `200000` chưa biết đặt vào đâu. Điền vào hai chỗ trống.

```python title=starter
tien = 250000

if tien > ___:
    print("Ngày tiêu rất nhiều")
elif tien > ___:
    print("Ngày tiêu nhiều")
else:
    print("Ngày tiêu ít")
```

```python title=solution
tien = 250000

if tien > 200000:
    print("Ngày tiêu rất nhiều")
elif tien > 100000:
    print("Ngày tiêu nhiều")
else:
    print("Ngày tiêu ít")
```

```python title=test
# Chấm bằng OUTPUT: với 250000, chuỗi phải chọn đúng nhánh "rất nhiều".
# Đặt hai ngưỡng ngược lại thì chương trình vẫn chạy, chỉ là in ra câu khác —
# đúng cái bẫy mà bài này nói tới.
pass
```

:::hints
- kind: attention
  body: Hai chỗ trống nhận hai con số `100000` và `200000`. Không phải chọn con số nào, mà chọn con số nào **lên trên**.
- kind: strategy
  body: Nhánh đứng trên phải là nhánh khó thoả hơn. Trong hai ngưỡng này, ngưỡng nào có ít ngày vượt qua hơn thì ngưỡng đó lên trước.
- kind: one-line
  body: "Viết `200000` vào chỗ trống thứ nhất và `100000` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: Ngày tiêu rất nhiều
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Vẫn ba dòng ấy. Đổi chỗ hai con số là chương trình nói khác hẳn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Xếp đúng thứ tự rồi. Nhưng sổ chi tiêu còn một cột nữa: ngày đó đã ghi vào sổ
hay chưa. Cột này chỉ có hai trạng thái, nên nó là một cái tên giữ `True` hoặc
`False` — thứ bạn đã gặp ở bài *Đúng hay sai*.

Byte muốn nhắc những ngày **không** có trong sổ, nên viết thế này:

```python
if co_trong_so == False:
    print("Ngày này chưa ghi sổ")
```

Nó chạy đúng. Nhưng đọc lên thành: *"nếu có-trong-sổ bằng sai"* — vòng qua hai
lần phủ định, trong khi tiếng Việt chỉ cần hai chữ: *không có*.

Có cách nói thẳng hơn không? Bài sau là đúng một từ ngắn để nói ngược lại một
câu.
::::

::::checkpoint{mastery=0.8}
::::
