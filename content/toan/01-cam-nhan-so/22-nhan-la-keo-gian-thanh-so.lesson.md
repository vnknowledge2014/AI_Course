---
id: toan.cam-nhan-so.nhan-la-keo-gian-thanh-so
title: Nhân là kéo giãn thanh số
summary: Gấp 3 lần thì không có hàng, không có cột. Nó là một phép kéo cả thanh số ra xa mốc 0 — và khoảng cách giữa hai vạch cũng giãn ra đúng chừng ấy.
locale: vi
track: toan
module: cam-nhan-so
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.multiply-as-scaling]
requires: [math.multiply-distributive, math.multiplication, math.thanh-so, math.number-line-add, core.variable, core.assignment, core.arithmetic, core.number-literal, core.print-variable, core.output]
concepts: [math.thanh-so, math.phep-nhan, math.don-vi-do]
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
Luống này gấp 3 luống kia. Mình đếm mãi mà không thấy hàng với cột đâu cả.
::::

::::explain{#gap-ba-lan-thi-hang-dau-cot-dau}
Bài trước để lại một câu hỏi thẳng. An nói: **"luống rau cải nhà tôi dài gấp 3
luống rau muống của cậu"** — trong câu ấy, đâu là hàng, đâu là cột?

Câu trả lời là: không có cái nào cả. Và chỗ đó đáng dừng lại một lát.

Mảng chữ nhật của bài 20 và 21 cần **hai lượng đếm được**: mấy hàng, mấy cột.
Byte xếp cây thành 7 hàng 13 cột thì cả hai con số đều là "mấy cái cây". Đổi
vai hai con số ấy, mảng xoay 90°, số cây vẫn thế.

Câu "gấp 3" thì khác hẳn. Nó có **một lượng** (luống rau muống dài 4 sải dây)
và **một số lần** (3). Con số 4 đo một đoạn đất; con số 3 không đo gì hết — nó
nói *phép biến đổi mạnh cỡ nào*. Nhét chúng vào một cái mảng chữ nhật thì phải
giả vờ rằng 3 cũng là mấy sải dây, mà nó không phải.

Nên bức tranh cho "gấp 3" nằm ở chỗ khác: **thanh số** của bài 5.

Một chuyện nhỏ về chỗ đứng của hai con số, nói ra để bạn khỏi khựng. Bài 19 đọc
`3 × 4` là "**ba lần** một lô bốn cây" — số lần đứng trước. Bài 20 đã cho phép
đổi vai hai con số mà tích không đổi, nên từ đây ta viết hệ số kéo giãn ở
**sau**: `4 × 3` đọc là "vạch 4, kéo giãn 3 lần". Cùng một con số 12, chỉ là
cách đọc hợp với sợi dây thun hơn.
::::

::::explain{#keo-ca-thanh-so-ra-xa-moc-0}
Trên thanh số, mỗi con số là một chỗ đứng, và mọi chỗ đứng đều đo từ mốc 0.

Bây giờ hình dung thanh số là một sợi dây thun có vạch chia đều, ghim chặt tại
mốc 0. Bạn cầm đầu dây kéo ra cho tới khi **mỗi khoảng dài gấp ba khoảng cũ**.

Chuyện gì xảy ra với từng cái vạch?

| vạch | trước khi kéo | sau khi kéo giãn 3 lần |
|---|---|---|
| mốc 0 | cách 0 đúng **không** khoảng | vẫn không khoảng → đứng yên tại 0 |
| vạch 1 | cách 0 đúng 1 khoảng | 1 khoảng mới = 3 khoảng cũ → tới chỗ 3 |
| vạch 4 | cách 0 đúng 4 khoảng | 4 khoảng mới = 12 khoảng cũ → tới chỗ 12 |
| vạch 10 | cách 0 đúng 10 khoảng | 10 khoảng mới = 30 khoảng cũ → tới chỗ 30 |

Đó là toàn bộ nội dung của phép nhân với 3, nhìn theo kiểu này: **`a × 3` là
chỗ mà vạch `a` rơi vào sau khi cả thanh số bị kéo giãn ba lần.**

Ba điều rơi ra ngay từ bức tranh, không phải học thuộc:

- **Mốc 0 đứng yên.** Kéo giãn là kéo ra xa 0, mà 0 thì cách 0 không khoảng
  nào; ba lần của "không khoảng nào" vẫn là không khoảng nào. Đây là lý do
  `0 × 3` bằng 0, và nó không phải một luật riêng ai đó nghĩ ra.
- **Nhân 1 không kéo gì cả.** Kéo cho mỗi khoảng dài gấp *một* lần khoảng cũ
  nghĩa là để nguyên. Thanh số không nhúc nhích, nên `a × 1` bằng chính `a`.
- **Khoảng cách giữa hai vạch cũng giãn ra.** Từ vạch 1 tới vạch 4 là 3 khoảng.
  Sau khi kéo, vạch 1 ở chỗ 3 và vạch 4 ở chỗ 12 — cách nhau 9 khoảng, đúng
  bằng 3 lần 3. Chỗ này là chỗ khác nhau lớn nhất giữa nhân và cộng, và ta sẽ
  hỏi máy ngay dưới đây.

> Bài 13 đã cho bạn một phép biến đổi thanh số rồi: **cộng là bước sang phải**.
> Cộng 3 thì mọi vạch cùng đi 3 khoảng — cả thanh trượt đi mà không đổi hình.
> Nhân 3 thì vạch càng xa 0 càng đi xa hơn — cả thanh nở ra. Hai phép, hai
> kiểu chuyển động khác hẳn nhau.

Còn bài 3 nói *đo là đếm cái thước*. Kéo thanh số giãn ra và thu nhỏ cái thước
lại là hai cách kể cùng một chuyện, và mãi về sau — lúc bạn cần rút gọn phân
số — mới tới lượt cách kể thứ hai có việc. Bài này chỉ cần một bức tranh: sợi
dây thun ghim tại 0.
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Kéo giãn thì hai cái vạch có giữ nguyên khoảng cách với nhau không? Hỏi thử.
::::

::::predict{#doan-cho-moi-cua-hai-vach commitOnce}
Byte lấy hai cái vạch trên thanh số: vạch **1** và vạch **4**. Rồi kéo giãn cả
thanh số **3 lần**.

Hai dòng đầu in ra chỗ mới của hai cái vạch ấy. Dòng thứ ba in ra **khoảng cách
giữa chúng sau khi kéo** — lấy chỗ mới của vạch 4 trừ đi chỗ mới của vạch 1.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
keo = 3
print(1 * keo)
print(4 * keo)
print(4 * keo - 1 * keo)
```

:::opt{correct}
3, rồi 12, rồi 9
:::

:::opt
3, rồi 12, rồi 3
::why
Gần đúng ở chỗ bạn giữ nguyên khoảng cách giữa hai cái vạch: trước khi kéo, từ
vạch 1 tới vạch 4 là 3 khoảng, và bạn coi con số 3 ấy như một thứ **thuộc về**
hai cái vạch, đi đâu cũng mang theo.

Suy nghĩ ấy đúng — với **phép cộng**. Bài 13: cộng là bước sang phải, cả thanh
số trượt đi cùng một quãng, nên hai vạch giữ nguyên khoảng cách với nhau. Phạm
vi của quy tắc "khoảng cách không đổi" dừng lại ở đó.

Kéo giãn thì thứ bị nhân lên chính là **mỗi cái khoảng**. Ba khoảng cũ nở thành
ba khoảng mới, mà mỗi khoảng mới dài bằng ba khoảng cũ — nên 3 thành 9.
::
:::

:::opt
3, rồi 12, rồi 11
::why
Gần đúng ở chỗ bạn tính đúng chỗ mới của vạch 4: nó rơi vào 12, không sai một
li. Và trừ đi vạch 1 để lấy khoảng cách cũng là việc đúng phải làm.

Chỗ lệch nằm ở con số bị trừ. Vạch 1 cũng đã dời chỗ rồi — nó không còn nằm ở
chỗ 1 nữa mà đã bị kéo tới chỗ 3. Dòng lệnh viết `4 * keo - 1 * keo` chứ không
viết `4 * keo - 1`, đúng vì lý do đó: cả hai đầu của cái khoảng đều đi.
::
:::

:::opt
4, rồi 7, rồi 3
::why
Gần đúng ở chỗ bạn dùng một phép biến đổi thanh số có thật, và dùng nó chuẩn
xác: **dịch cả thanh sang phải 3 bước**, tức là cộng 3. Vạch 1 sang 4, vạch 4
sang 7, khoảng cách giữ nguyên 3 — mọi bước tính đều khớp với chính nó.

Phạm vi bị vượt nằm ở dấu phép tính. Dấu `*` không dịch thanh số, nó kéo giãn
thanh số. Dịch thì mọi vạch đi cùng một quãng; kéo giãn thì vạch nào đang xa 0
hơn sẽ đi xa hơn. Cùng con số 3, hai công việc khác nhau.
::
:::
::::

::::example{#may-tra-loi}
Chạy lên, máy in ra:

```text
3
12
9
```

Vạch 1 tới chỗ 3. Vạch 4 tới chỗ 12. Và khoảng giữa chúng, vốn là 3 khoảng, giờ
là 9 khoảng — cũng bị kéo giãn đúng ba lần như mọi thứ khác trên thanh.

Ba con số ấy khớp đúng bảng ở trên, và bảng ở trên thì được suy ra từ một sợi
dây thun chứ không từ bảng cửu chương.
::::

::::code{#keo-gian-ca-vuon}
Vườn Byte có hai luống: luống rau muống dài **4** sải dây, luống mồng tơi dài
**7** sải dây. Byte kéo giãn thanh số **3 lần** rồi đọc lại cả vườn trên thanh
số mới.

Điền bốn chỗ trống: chỗ mới của từng luống, khoảng cách giữa hai luống **sau
khi kéo**, và chỗ mà **mốc 0** rơi vào sau khi kéo.

Bài chấm bằng bốn tình huống lệch nhau — hai luống dài khác nhau, khoảng cách
giữa chúng, và mốc 0. Một con số chép cứng vào cả bốn chỗ trống chỉ đúng được
nhiều nhất một dòng, nên phải viết ra phép kéo giãn thật. Cách chấm còn bắt cả
bốn dòng đi qua chính cái tên `keo`, và đối chiếu khoảng cách bạn tính với
`(mong_toi − rau_muong) × keo`.

```python title=starter
rau_muong = 4
mong_toi = 7
keo = 3

rau_muong_sau_keo = ___
mong_toi_sau_keo = ___
khoang_giua_sau_keo = ___
moc_0_sau_keo = ___

print(rau_muong_sau_keo)
print(mong_toi_sau_keo)
print(khoang_giua_sau_keo)
print(moc_0_sau_keo)
```

```python title=solution
rau_muong = 4
mong_toi = 7
keo = 3

rau_muong_sau_keo = rau_muong * keo
mong_toi_sau_keo = mong_toi * keo
khoang_giua_sau_keo = mong_toi_sau_keo - rau_muong_sau_keo
moc_0_sau_keo = 0 * keo

print(rau_muong_sau_keo)
print(mong_toi_sau_keo)
print(khoang_giua_sau_keo)
print(moc_0_sau_keo)
```

```python title=test
# Bốn câu, bốn chỗ trống — câu nào cũng đọc thẳng thứ bạn vừa viết, nên gõ sai
# là đỏ ngay. Hai luống dài khác nhau (4 và 7) nên một con số gõ cứng không qua
# nổi cả hai dòng đầu. Hai câu cuối chốt lại đúng hai điều bài vừa nói: khoảng
# cách giữa hai vạch cũng giãn ra chừng ấy lần, và mốc 0 thì đứng yên.
assert rau_muong_sau_keo == 12, "vạch 4 kéo giãn 3 lần thì rơi vào chỗ 12"
assert mong_toi_sau_keo == 21, "vạch 7 kéo giãn 3 lần thì rơi vào chỗ 21"
assert khoang_giua_sau_keo == (mong_toi - rau_muong) * keo, \
    "khoảng giữa hai luống vốn là 3 khoảng; kéo giãn 3 lần thì nó thành 9, không giữ nguyên 3"
assert moc_0_sau_keo == 0, "mốc 0 là chỗ ghim sợi dây thun — kéo kiểu gì nó cũng đứng yên"
```

:::hints
- kind: attention
  body: Ba cái tên đã có sẵn ở trên đầu bài — `rau_muong`, `mong_toi`, `keo`. Ba chỗ trống đầu lắp được từ những cái tên ấy; chỗ trống cuối cần thêm đúng một con số, mà con số ấy chính là mốc 0.
- kind: strategy
  body: Hai dòng đầu hỏi cùng một câu cho hai cái vạch khác nhau — vạch này cách 0 mấy khoảng, nhân lên chừng ấy lần. Dòng thứ ba hỏi khoảng cách giữa hai chỗ MỚI, mà chỗ mới thì hai dòng trên vừa đặt tên xong rồi. Dòng thứ tư hỏi cùng câu đầu tiên nhưng cho cái vạch nằm ngay tại chỗ ghim.
- kind: one-line
  body: "Bốn chỗ trống lần lượt là `rau_muong * keo`, `mong_toi * keo`, `mong_toi_sau_keo - rau_muong_sau_keo`, và `0 * keo`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: static
  onFail: mỗi luống phải được kéo giãn bằng chính con số `keo`, không phải chép cứng kết quả — chép cứng thì đổi `keo` sang 5 là bài sai ngay
  requireAst:
  # Ba phép nhân (một cho mỗi luống, một cho mốc 0) và ba chỗ ĐỌC tên `keo`.
  # Điền bừa vào chỗ trống thì không có dấu nhân nào, nên luật này phân biệt
  # được. `min: 3` cũng chặn luôn đáp án gõ thẳng `0` vào dòng mốc 0 — dòng ấy
  # phải là một phép kéo giãn thật thì mới nói được điều bài đang nói.
  - kind: uses-operator, target: *, min: 3
  - kind: uses-name, target: keo, min: 3
- tier: output
  match: regex
  expect: ^12\n21\n9\n0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
12 và 21, cách nhau 9. Khoảng giữa hai luống cũng bị kéo, còn mốc 0 thì không.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa có một cách đọc mới cho phép nhân: nó là một phép biến đổi cả thanh số.
Kéo giãn 3 lần thì mọi chỗ ra xa mốc 0 hơn. Kéo giãn 1 lần thì không đổi gì —
thanh số đứng im.

Nhưng thanh số của bạn không dừng ở mốc 0. Bài 16 đã kéo nó chạy tiếp sang trái,
và bài 18 cho mỗi số một **số đối** ở đúng phía bên kia.

Vậy nhân với `−1` thì làm gì với thanh số? Nó không kéo giãn — vì kéo giãn một
lần là không đổi gì, mà `−1` với `1` thì không phải cùng một số. Rễ của Byte
đang ở độ sâu `−2`; nhân nó với `−1` thì nó đi đâu?

Bài sau trả lời, và câu trả lời chỉ cần một động tác của bàn tay.
::::

::::checkpoint{mastery=0.8}
::::
