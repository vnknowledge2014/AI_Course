---
id: toan.cam-nhan-so.ben-trai-so-khong
title: Bên trái số 0
summary: Thanh số không dừng lại ở 0. Nó chạy tiếp sang trái, và mỗi chỗ bên đó cũng có tên riêng.
locale: vi
track: toan
module: cam-nhan-so
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.negative-number]
requires: [math.number-line-subtract, math.number-line-add, math.thanh-so, core.arithmetic, core.number-literal, core.variable, core.print-variable, core.output]
concepts: [math.thanh-so, math.so-am, math.moc-khong]
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
Thanh số của mình còn chạy tiếp về bên kia số 0. Đi thử một đoạn nhé.
::::

::::explain{#luat-lui-co-noi-phai-dung-khong}
Bài trước để lại một câu hỏi. Từ 5 tới 12 là 7 bước sang phải; từ 12 tới 5 cũng
7 bước, chỉ ngược chiều. Nhưng viết `5 − 12` thì bạn phải đứng ở vạch 5 mà lùi
12 bước — lùi qua khỏi số 0. **Bên trái số 0 có gì?**

Trước khi đi tìm kết quả, hãy nhìn lại chính cái luật bạn đang dùng. Bài 14
nói: *trừ b nghĩa là lùi b bước*. Đọc kỹ câu đó — nó nói về **số bước**, và nó
không hề nói một chữ nào về chỗ bạn đứng lúc bắt đầu.

Thử ba lần lùi, cùng một luật:

- Đứng ở vạch 12, lùi 5 bước. Tới vạch 7. Không ai thắc mắc.
- Đứng ở vạch 5, lùi 5 bước. Tới vạch 0. Vẫn còn trong bảng.
- Đứng ở vạch 5, lùi 6 bước. Bước thứ sáu đặt xuống đâu?

Tới đây bạn có đúng hai đường đi tiếp, và toán học đã chọn một trong hai:

1. **Thêm một luật ngoại lệ:** "lùi tới 0 thì dừng, đừng lùi nữa". Luật cũ vẫn
   giữ nguyên, nhưng từ nay nó có một cái bẫy: mỗi lần trừ, bạn phải kiểm trước
   xem có đủ chỗ để lùi không.
2. **Kéo dài thanh số về bên trái.** Không thêm luật nào cả. Bước thứ sáu đặt
   xuống một chỗ mới, và chỗ đó cần một cái tên.

Đường số 2 được chọn, và lý do rất cụ thể: một luật có ngoại lệ là một luật bạn
sẽ quên đúng lúc cần nó nhất. Kéo dài thanh số thì câu *"trừ b là lùi b bước"*
đúng ở mọi chỗ, mọi lần, không cần kiểm gì trước.
::::

::::explain{#hai-manh-lam-nen-mot-cai-ten}
Chỗ mới ấy tên gì?

Bên phải 0, một cái tên chỉ cần một mảnh: *cách 0 bao xa*. Vạch 7 là chỗ cách 0
bảy bước, và chỉ có đúng một chỗ như thế.

Bên trái 0 thì một mảnh không đủ nữa. Có **hai** chỗ cách 0 bảy bước — một bên
này, một bên kia. Muốn chỉ đúng một chỗ, cái tên phải mang hai mảnh:

- **cách 0 bao xa** — con số;
- **ở phía nào** — dấu.

Người ta viết mảnh thứ hai bằng một dấu gạch ngang đặt trước: `-7`, đọc là **âm
bảy**. Không có dấu thì hiểu là phía bên phải. Những chỗ bên trái 0 gọi chung
là **số âm**.

Một chỗ dễ vấp: dấu gạch ngang ấy đúng là phím bạn vẫn bấm để trừ, nhưng ở đây
nó làm việc khác hẳn.

- Trong `12 - 5`, dấu `-` là một **việc phải làm**: lùi lại.
- Trong `-7`, dấu `-` là một **phần của cái tên**: chỗ này nằm bên trái 0.

Cùng một ký hiệu, hai vai — giống hệt dấu `+` ở Realm 0, khi thì cộng hai số,
khi thì nối hai câu chữ. Máy phân biệt được bằng chỗ đứng của dấu, và bạn cũng
vậy.
::::

::::byte{trigger=enter mood=thinking pose=point-stage}
Mình cắm cái cọc đo ở luống rau. Mặt đất là vạch 0 — chỗ dễ nhìn nhất thôi.
::::

::::example{#cai-coc-trong-vuon}
Byte cắm một cái cọc xuống luống rau, lấy **mặt đất** làm vạch 0. Đó chỉ là chỗ
Byte thấy dễ nhìn nhất, không phải chỗ thế giới kết thúc — dưới mặt đất vẫn còn
đất, và rễ vẫn đi tiếp xuống đó.

Vạch trên cọc, đọc từ dưới lên:

```text
    3 ┤ ngọn mầm cải
    2 ┤
    1 ┤
    0 ┼──── mặt đất
   -1 ┤
   -2 ┤
   -3 ┤
   -4 ┤
   -5 ┤ đầu rễ ớt
```

Đặt cái cọc ấy nằm xuống, xoay một góc vuông, thì nó chính là thanh số bài 5 —
chỉ khác là bây giờ nó dài ra về bên trái:

```text
   ───┬────┬────┬────┬────┬────┬────┬────┬────┬───
      -5   -4   -3   -2   -1    0    1    2    3
      rễ ớt                  mặt đất      ngọn mầm
```

Hai bức tranh, một thứ. Trên cọc dựng đứng thì "sang phải" thành "cao hơn";
đầu rễ ớt ở vạch `-5`, ngọn mầm cải ở vạch `3`.

Bây giờ hỏi thẳng cái máy, đúng ba phép mà bài trước còn nợ:

```python title=readonly
print(12 - 5)
print(5 - 12)
print(0 - 3)
```

Máy in ra:

```text
7
-7
-3
```

Dòng đầu là chuyện cũ. Dòng thứ hai là câu trả lời cho bài trước: `5 − 12` ra
`-7` — đứng ở vạch 5, lùi 12 bước, dừng ở chỗ cách 0 bảy bước về bên trái. Dòng
thứ ba là phiên bản gọn nhất: xuất phát ngay từ mặt đất mà lùi 3 bước.
::::

::::predict{#doan-truoc-khi-chay commitOnce}
Byte đang đứng ở vạch 2 trên thanh số và lùi 6 bước.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(2 - 6)
```

:::opt{correct}
-4
:::

:::opt
4
::why
Gần đúng ở chỗ bạn đếm không sai một bước nào: từ vạch 2 lùi 6 bước thì dừng ở
một chỗ cách vạch 0 đúng **bốn** bước. Con số 4 bạn tìm ra là thật, và nó là
mảnh thứ nhất của cái tên.

Chỗ lệch nằm ở mảnh thứ hai. Quy tắc "số đo là một con số trần, không dấu" đúng
trong phạm vi đo **độ dài** — bài 3 đo luống đất, mà độ dài thì không có phía.
Còn ở đây bạn đang gọi tên một **chỗ**, và bên trái 0 thì có tới hai chỗ cùng
cách 0 bốn bước. Bỏ dấu đi là hai chỗ khác nhau mang chung một tên.
::
:::

:::opt
0
::why
Gần đúng ở chỗ bạn nhận ra vạch 2 chỉ đủ cho hai bước lùi: hết bước thứ hai là
chạm 0, không còn gì để bớt nữa. Ý nghĩ ấy đúng hẳn hoi — với **hạt**. Byte có
2 hạt thì không ai lấy đi 6 hạt được, và cái nắm hạt ấy nhiều nhất chỉ vơi tới
rỗng.

Phạm vi của quy tắc ấy là những thứ đếm được mà không bẻ nhỏ được — đúng cái
loại đơn vị bài 4 gọi tên. Chỗ lệch: `2 - 6` ở đây không hỏi về nắm hạt, nó hỏi
về một **chỗ** trên thanh số. Dưới mặt đất vẫn là đất, và đi xuống thêm bốn
phân nữa là việc cái rễ làm hằng ngày. Thanh số không có bức tường ở vạch 0.
::
:::

:::opt
Máy dừng lại và báo lỗi, vì kết quả không phải một con số
::why
Gần đúng ở chỗ bạn nhớ rất chuẩn tính cách của máy từ Realm 0: gặp việc không
có nghĩa thì nó dừng hẳn và nói ra, chứ không đoán bừa giúp bạn — đúng như lúc
nó ném ra `TypeError` khi bạn bảo nó cộng một câu chữ với một con số.

Chỗ lệch: việc này **có** nghĩa. Máy giữ số theo đúng cái thanh số vừa kéo dài
ban nãy, nên với nó `2 - 6` là một câu hỏi bình thường và nó trả lời được ngay,
không phải nghĩ lâu hơn `12 - 5` một chút nào.
::
:::
::::

::::explain{#hai-so-khong-phai-mot}
Chốt lại một câu để mang theo: **thanh số không kết thúc ở 0.** Số 0 là một vạch
giữa đường, không phải cái mép.

Và một chỗ đáng ghi vào sổ: `4` với `-4` là **hai số khác nhau**, dù mảnh thứ
nhất của tên chúng giống hệt nhau. Chúng cách 0 bằng nhau, nhưng nằm hai phía.
Trên cái cọc trong vườn, một cái là ngọn mầm cao bốn phân, một cái là đầu rễ sâu
bốn phân — không ai nhầm hai thứ đó ngoài vườn cả.
::::

::::code{#hai-lan-lui}
Byte lùi hai lần trên thanh số, rồi ghi lại tên một chỗ nằm sẵn bên trái vạch 0.

Hai lần lùi được chọn để ra **hai loại kết quả khác nhau**: lần một dừng lại khi
còn bên phải vạch 0, lần hai đi qua hẳn sang bên trái. Chép cứng một con số vào
cả hai chỗ trống thì nhiều nhất cũng chỉ qua được một dòng — mỗi chỗ trống phải
là một phép lùi viết ra thật.

Chỗ trống thứ ba thì ngược lại: không có phép nào phải làm, chỉ có một cái tên
phải viết ra cho đủ hai mảnh.

```python title=starter
# Thanh số của Byte: vạch 0 là mặt đất. Sang phải là phía mầm mọc lên,
# sang trái là phía rễ ăn xuống. Lùi lại thì viết dấu trừ.

# Lần một: đang ở vạch 9, lùi 4 bước.
sau_lan_mot = ___

# Lần hai: đang ở vạch 3, lùi 7 bước — nhiều hơn số bước từ 3 về tới 0.
sau_lan_hai = ___

# Đầu rễ ớt nằm ở chỗ cách mặt đất 5 phân về phía dưới.
# Ở đây không có lần lùi nào cả — chỉ viết TÊN của chỗ ấy ra,
# cái tên gồm hai mảnh: cách 0 bao xa, và ở phía nào.
cho_re_ot = ___

print(sau_lan_mot)
print(sau_lan_hai)
print(cho_re_ot)
```

```python title=solution
# Thanh số của Byte: vạch 0 là mặt đất. Sang phải là phía mầm mọc lên,
# sang trái là phía rễ ăn xuống. Lùi lại thì viết dấu trừ.

# Lần một: đang ở vạch 9, lùi 4 bước.
sau_lan_mot = 9 - 4

# Lần hai: đang ở vạch 3, lùi 7 bước — nhiều hơn số bước từ 3 về tới 0.
sau_lan_hai = 3 - 7

# Đầu rễ ớt nằm ở chỗ cách mặt đất 5 phân về phía dưới.
# Ở đây không có lần lùi nào cả — chỉ viết TÊN của chỗ ấy ra,
# cái tên gồm hai mảnh: cách 0 bao xa, và ở phía nào.
cho_re_ot = -5

print(sau_lan_mot)
print(sau_lan_hai)
print(cho_re_ot)
```

```python title=test
# Bốn câu khẳng định này chốt lại đúng điều bài vừa dạy, chứ không chỉ chấm bài:
# nếu một ngày nào đó máy chạy bài học đổi cách hiểu dấu trừ, cổng kiểm phải đỏ
# lên chứ không được dạy sai lặng lẽ.
assert sau_lan_mot == 5, "từ vạch 9 lùi 4 bước thì dừng ở vạch 5"
assert sau_lan_hai == -4, "từ vạch 3 lùi 7 bước thì đi qua 0 và dừng ở vạch -4"
assert sau_lan_hai < 0, "lùi quá vạch 0 thì chỗ đến nằm bên trái, nên tên nó phải mang dấu"
assert cho_re_ot == -5, "chỗ cách vạch 0 năm bước về bên trái mang tên -5"
```

:::hints
- kind: attention
  body: Hai chỗ trống đầu, đề bài cho bạn hai con số cho mỗi lần: chỗ đang đứng, và số bước lùi. Cả hai con số đó đều phải xuất hiện trong chỗ trống, không được thiếu cái nào. Chỗ trống thứ ba không cho bạn hai con số, vì ở đó không có lần lùi nào.
- kind: strategy
  body: Bài 14 nói lùi lại thì viết dấu trừ. Vậy hai chỗ trống đầu có dạng "chỗ đang đứng, dấu trừ, số bước lùi" — đừng tự tính nhẩm rồi chép con số kết quả vào, lần hai là chỗ dễ nhẩm sai nhất. Chỗ trống thứ ba thì đọc lại hai mảnh của một cái tên bên trái vạch 0: khoảng cách viết thành con số, phía viết thành dấu đặt trước nó.
- kind: one-line
  body: "Ba chỗ trống lần lượt là `9 - 4`, `3 - 7` và `-5`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hai chỗ trống đầu phải là một phép lùi viết bằng dấu trừ, đúng trên hai con số đề bài cho; chỗ trống thứ ba phải là tên của chỗ cách vạch 0 năm bước về bên trái
  requireAst:
  - kind: uses-operator, target: -, min: 2
  - kind: has-literal, target: 9
  - kind: has-literal, target: 4
  - kind: has-literal, target: 3
  - kind: has-literal, target: 7
  - kind: has-literal, target: 5
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^5\n-4\n-5\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Vạch -4. Bên trái số 0 vẫn còn đất, và chỗ nào cũng có tên.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại cái cọc trong vườn. Vạch `-5` và vạch `5` cách mặt đất bằng nhau đúng
năm phân, chỉ khác phía: một cái là đầu rễ, một cái là ngọn mầm.

Bây giờ so hai cái rễ với nhau. Rễ ớt ở vạch `-5`, rễ húng ở vạch `-2`. Rễ ớt ăn
sâu hơn — chuyện đó ai đào lên cũng thấy. Nhưng nếu hỏi **số nào lớn hơn**, `-5`
hay `-2`?

Bạn biết chắc `5` lớn hơn `2`. Câu đó có kéo theo `-5` lớn hơn `-2` không, hay
qua bên trái vạch 0 thì phải hỏi lại từ đầu?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
