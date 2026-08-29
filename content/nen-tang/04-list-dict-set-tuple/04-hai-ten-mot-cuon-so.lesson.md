---
id: nen-tang.list-dict-set-tuple.hai-ten-mot-cuon-so
title: Hai cái tên, một cuốn sổ
summary: Dấu bằng không chép cuốn sổ ra làm hai — `so_moi = so` chỉ dán thêm một tấm nhãn lên đúng cuốn cũ, nên ai ghi vào cũng thấy.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.list-aliasing]
requires: [core.list, core.list-append, core.variable, core.assignment, core.argument-not-copy, core.function-def, core.function-call, core.fstring]
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
Mình chưa hề chép cuốn sổ ra làm hai. Vậy mà hai cái tên đều dài thêm một dòng.
::::

::::explain{#cau-tra-loi-la-co}
Bài trước kết bằng một câu hỏi thẳng: bạn đưa **cả cuốn sổ** cho hàm
`don_so(so)`, trong hàm nó `.append` thêm một dòng. Ra ngoài, cuốn sổ của
**bạn** có dòng đó không?

Câu trả lời là **có**. Và bạn đã gặp chuyện này rồi — mạch Hàm đã cho xem tận
mắt: lúc gọi hàm, máy không chép cuốn sổ ra một bản mới cho hàm dùng riêng. Nó
đưa vào hàm đúng cuốn sổ đang nằm ngoài kia.

Chỗ đáng dừng lại ở bài này là một câu khác, và nó lớn hơn:

**Chuyện ấy không phải là một luật của hàm.**

Hàm chỉ là nơi nó lộ ra sớm nhất, vì hàm có hẳn hai cái tên cho bạn nhìn — tên
tham số bên trong, tên biến bên ngoài. Nhưng luật thật nằm ở một chỗ nhỏ hơn
nhiều, chỗ bạn gõ mỗi ngày mà chưa từng nghi ngờ: **dấu bằng**.

Viết `so_moi = so` thì máy không dựng ra một cuốn sổ thứ hai. Nó chỉ dán thêm
một tấm nhãn nữa lên đúng cuốn đang có. Hai nhãn, một cuốn. Từ lúc đó, ghi vào
cuốn qua tên nào cũng như nhau — vì chỉ có một cuốn để mà ghi.
::::

::::example{#nhin-tan-mat}
Không có hàm nào trong đoạn dưới. Chỉ có hai cái tên và một dấu bằng.

```python title=readonly
so_thang_tam = [85000, 240000, 120000]
so = so_thang_tam

so.append(95000)

print(f"so           = {so}")
print(f"so_thang_tam = {so_thang_tam}")
```

Máy in ra:

```text
so           = [85000, 240000, 120000, 95000]
so_thang_tam = [85000, 240000, 120000, 95000]
```

Dòng thứ hai là dòng đáng nhìn. Cả đoạn code chỉ có đúng một lệnh `.append`, và
nó viết qua cái tên `so`. Cái tên `so_thang_tam` không được nhắc tới lần nào
giữa hai dòng ấy. Vậy mà nó dài thêm một dòng.

Đi lại đúng đường máy đi:

- Dòng 1 dựng một cuốn sổ ba dòng, rồi dán tấm nhãn `so_thang_tam` lên đó.
- Dòng 2, `so = so_thang_tam`, đọc xem tấm nhãn `so_thang_tam` đang dán vào
  cuốn nào, rồi dán tấm nhãn `so` lên **đúng cuốn ấy**. Không có cuốn thứ hai
  nào được dựng ra.
- `so.append(95000)` cầm cuốn mà tấm nhãn `so` đang dán vào và viết thêm một
  dòng. Cuốn ấy cũng chính là cuốn mà `so_thang_tam` đang dán vào.
- Hai lệnh `print` mở cùng một cuốn ra đọc, nên chúng phải in ra cùng một thứ.
::::

::::explain{#vi-sao-mai-toi-gio-moi-lo-ra}
Câu hỏi tự nhiên tiếp theo: dấu bằng vẫn thế từ Realm 0 tới giờ, sao mãi tới
bây giờ chuyện này mới lộ ra?

Vì tới giờ bạn mới có trong tay một thứ **sửa được**.

Nhìn hai cái tên cùng trỏ vào một con số:

```python title=readonly
tuoi = 5
tuoi_khac = tuoi
tuoi_khac = tuoi_khac + 1

print(tuoi)
```

Máy in ra `5`. Nhưng lý do không phải là "gán đã chép con số ra làm hai bản".
Lý do là dòng thứ ba **cũng là một phép gán**: nó tính ra một con số mới rồi
dán lại tấm nhãn `tuoi_khac` sang con số mới ấy. Tấm nhãn `tuoi` không ai đụng
tới, nên nó vẫn dán vào số 5.

Và với số thì bạn **không có cách nào khác**: không có lệnh nào sửa được số 5
thành số 6 tại chỗ. Chuỗi cũng vậy — mạch Giá trị đã cho bạn thử sửa một ký tự
trong chuỗi và bị máy từ chối thẳng. Cả hai loại ấy chỉ cho bạn dán lại nhãn,
nên chuyện "hai tấm nhãn, một giá trị" chưa bao giờ gây ra hậu quả nào để mà
nhìn thấy.

Danh sách thì khác. Realm 0 đã nói rõ: list **sửa được**, và `.append` là lệnh
sửa nó tại chỗ. Từ lúc trong tay có một thứ sửa được, việc hai cái tên cùng
trỏ vào một cuốn mới thành chuyện đáng biết.

Một câu để nhớ, và nó sẽ còn dùng lại nhiều lần: **dấu bằng dán nhãn,
`.append` viết vào sổ.**

> Chỗ dễ vấp: `so_moi = so` trông rất giống một lệnh sao lưu, vì trong tiếng
> Việt "cho tôi một cái tên khác của cuốn sổ" nghe gần như "cho tôi một cuốn
> sổ khác giống hệt". Máy không đọc ra ý ấy. Nó làm đúng một việc: đọc tên bên
> phải xem đang dán vào đâu, rồi dán tên bên trái vào cùng chỗ.
::::

::::predict{#doan-hai-dong-in-ra commitOnce}
Byte định lưu lại một bản sổ trước khi ghi tiếp, rồi ghi thêm một dòng qua mỗi
cái tên.

**Trước khi bấm chạy**, bạn đoán hai dòng cuối in ra gì?

```python title=readonly
so = [85000, 240000]
sao_luu = so

so.append(310000)
sao_luu.append(60000)

print(f"so      = {so}")
print(f"sao_luu = {sao_luu}")
```

:::opt{correct}
Hai dòng in ra giống hệt nhau, cùng là `[85000, 240000, 310000, 60000]`
:::

:::opt
`so = [85000, 240000, 310000]` và `sao_luu = [85000, 240000, 60000]`
::why
Gần đúng ở chỗ bạn đọc từng lệnh `.append` rất chuẩn: lệnh thứ nhất viết qua
tên `so`, lệnh thứ hai viết qua tên `sao_luu`, và mỗi lệnh chỉ ghi đúng một
dòng. Phần ấy bạn không sai chữ nào.

Chỗ lệch nằm ở dòng `sao_luu = so`. Nó không dựng ra cuốn sổ thứ hai, nên
không có hai cuốn để mà mỗi cuốn giữ dòng của mình. Hai lệnh `.append` viết
vào **cùng một cuốn**, lần lượt: 310000 trước, 60000 sau. Xong cả hai, cuốn ấy
có bốn dòng, và cả hai tấm nhãn đều đang dán lên nó.
::
:::

:::opt
`so = [85000, 240000, 310000, 60000]` và `sao_luu = [85000, 240000, 60000]`
::why
Gần đúng ở chỗ khó: bạn nhận ra `so` nhìn thấy cả hai dòng mới, kể cả dòng
được ghi qua cái tên kia. Đó chính là điều bài này đang chỉ ra.

Chỗ lệch là bạn để chuyện ấy chạy một chiều. Nếu `sao_luu` đã là một cuốn
riêng thì dòng 60000 ghi vào cuốn riêng ấy, và `so` không có cách nào nhìn
thấy nó. Hai cái tên hoặc cùng một cuốn, hoặc là hai cuốn — không có kiểu nửa
chừng. Ở đây chúng cùng một cuốn, nên chúng thấy đủ cả hai dòng.
::
:::

:::opt
Máy báo lỗi, vì hai cái tên cùng ghi vào một chỗ
::why
Gần đúng ở chỗ bạn thấy đúng cái nguy: hai cái tên cùng viết vào một cuốn sổ
là chuyện dễ sinh chuyện, và với người đọc code thì nó gần như vô hình.

Chỗ lệch là máy không coi đó là lỗi. Với máy, cả hai lệnh đều hợp lệ: một tấm
nhãn hợp lệ, một cuốn sổ sửa được, một lệnh ghi thêm. Không có gì để chặn.
Đúng chỗ đó mới là điều đáng nhớ — không một tiếng báo nào, mà cuốn "sao lưu"
của bạn đã đổi.
::
:::
::::

::::explain{#khi-nao-thi-day-la-thu-minh-muon}
Hai cái tên trỏ chung một cuốn không phải lúc nào cũng là tai nạn. Nhiều lúc nó
đúng là thứ bạn cần.

Cả nhà Byte ghi chung một cuốn sổ chi tiêu. Đoạn code lo phần của Byte gọi nó
là `so_cua_byte`, đoạn lo phần cả nhà gọi nó là `so_chung`. Hai cái tên, và
đúng ra thì chỉ được có **một** cuốn — ghi qua tên nào thì cuốn sổ chung cũng
phải thấy.

Đó là lúc `so_cua_byte = so_chung` là lệnh đúng, chứ không phải lệnh nhầm.

Điều đáng nhớ không phải là "đừng bao giờ viết dấu bằng giữa hai danh sách".
Điều đáng nhớ là **biết mình đang chọn gì**: dấu bằng cho bạn thêm một cái tên
của cùng một cuốn, và đó là chuyện tốt khi bạn muốn đúng như vậy.
::::

::::code{#mot-cuon-hai-cai-ten}
Cuốn sổ chi tiêu của cả nhà đã có ba khoản. Byte cần một cái tên riêng để ghi
phần của mình vào, nhưng cả nhà chỉ có **một** cuốn sổ — ghi qua tên nào thì
`so_chung` cũng phải dài thêm.

Hãy điền vế phải của dòng còn thiếu.

```python title=starter
so_chung = [85000, 240000, 120000]

# Byte ghi phần mình qua cái tên `so_cua_byte`. Cả nhà chỉ có MỘT cuốn sổ.
so_cua_byte = ___

so_cua_byte.append(95000)

print(f"Byte nhìn thấy:   {so_cua_byte}")
print(f"Cả nhà nhìn thấy: {so_chung}")
```

```python title=solution
so_chung = [85000, 240000, 120000]

# Byte ghi phần mình qua cái tên `so_cua_byte`. Cả nhà chỉ có MỘT cuốn sổ.
so_cua_byte = so_chung

so_cua_byte.append(95000)

print(f"Byte nhìn thấy:   {so_cua_byte}")
print(f"Cả nhà nhìn thấy: {so_chung}")
```

```python title=test
# Một lời giải chép tay lại ba con số cũ vẫn in ra đúng dòng đầu tiên, nhưng
# cuốn sổ của cả nhà thì nó không đụng tới. Câu kiểm đầu tiên lôi chuyện ấy ra.
assert so_chung == [85000, 240000, 120000, 95000], "ghi 95000 qua cái tên `so_cua_byte` thì `so_chung` phải dài thêm đúng dòng ấy — hai cái tên phải đang dán lên MỘT cuốn sổ"
# Ghi thêm một dòng NỮA, sau khi đoạn trên đã chạy xong: hai cái tên phải
# dính vào nhau ở mọi lần ghi, không phải chỉ ở lần ghi đầu tiên.
so_cua_byte.append(60000)
assert so_chung == [85000, 240000, 120000, 95000, 60000], "ghi thêm 60000 qua `so_cua_byte` thì `so_chung` cũng phải có dòng 60000: một cuốn sổ, hai cái tên"
assert so_cua_byte == so_chung, "hai cái tên phải luôn cho ra cùng một nội dung, vì chúng cùng mở một cuốn sổ ra đọc"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ở vế phải một dấu bằng. Vế phải là chỗ bạn nói cho máy biết tấm nhãn `so_cua_byte` sẽ dán vào đâu — và cuốn sổ cần dán vào đã có tên sẵn ở dòng đầu tiên rồi.
- kind: strategy
  body: Bạn không cần dựng thêm cuốn sổ nào, cũng không cần chép lại ba con số. Việc cần làm ngược lại: chỉ thẳng vào cuốn đang có, để hai tấm nhãn cùng dán lên nó. Chép tay ba con số vào chỗ trống sẽ cho ra hai cuốn rời nhau, và dòng in cuối cùng sẽ thiếu mất khoản 95000.
- kind: one-line
  body: 'Viết `so_chung` vào chỗ trống, thành `so_cua_byte = so_chung`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Byte nhìn thấy:   \[85000, 240000, 120000, 95000\]\nCả nhà nhìn thấy: \[85000, 240000, 120000, 95000\]\s*$
- tier: static
  onFail: chỗ trống phải chỉ thẳng vào cuốn sổ đã có tên `so_chung`, không phải chép tay một danh sách mới
  requireAst:
  # Khung đã đọc `so_chung` đúng MỘT lần (trong dòng `print` cuối). Lời giải
  # đúng đọc nó thêm một lần nữa ở vế phải dấu bằng, nên phải có ít nhất hai.
  # Một đáp án chép tay `[85000, 240000, 120000]` vẫn chỉ có một.
  - kind: uses-name, target: so_chung, min: 2
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một cuốn sổ, hai cái tên — và lần này mình cố ý muốn thế.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hôm nay bạn thấy chiều thuận: dấu bằng cho thêm một cái tên của cùng một cuốn,
và có lúc đó đúng là thứ cần. Nhưng có lúc thì không.

Bạn muốn đưa cuốn sổ cho một hàm nghịch thử — thêm dòng, bớt dòng, xáo tung —
mà cuốn sổ thật vẫn phải nguyên vẹn. Muốn vậy thì hàm phải nghịch trên một
**bản riêng**, và câu hỏi thành ra: lấy bản riêng ấy ở đâu?

Bài 3 đã đưa bạn một thứ trông rất giống: `so[:]` — bỏ trống cả hai đầu lát
cắt, nghĩa là "từ đầu tới hết", tức là cả cuốn.

Thứ nó đưa ra là **chính cuốn cũ**, hay một cuốn khác?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
