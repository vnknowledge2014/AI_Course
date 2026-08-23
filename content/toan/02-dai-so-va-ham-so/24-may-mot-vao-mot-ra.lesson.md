---
id: toan.dai-so-va-ham-so.may-mot-vao-mot-ra
title: Cái máy — một đầu vào, đúng một đầu ra
summary: Ba cuốn sổ của quán, hai cái tra được và một cái không; chỗ khác nhau giữa chúng có một cái tên, và cái tên ấy là hàm số.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.function]
requires: [math.graph-of-expression, math.ordered-pair, math.coordinate-plane, math.value-table, math.substitution, math.multiplication, core.variable, core.assignment, core.arithmetic, core.number-literal, core.print-variable, core.boolean, ctrl.comparison]
concepts: [math.ham-so, math.dau-vao-dau-ra, math.xe-banh-mi]
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
Bỏ một con số vào, nhận một con số ra. Nhưng có cái nhả ra tận hai.
::::

::::explain{#ba-cuon-so}
Bài trước để lại hai cái hình. Cả hai đều là hai trục vuông góc và một nắm dấu
chấm. Một cái tra được. Cái kia tra tới 12 ổ thì trả về hai câu trả lời.

Hôm nay không vẽ thêm hình nào. Hôm nay đi **phân loại**.

Suốt track này bạn gặp đi gặp lại đúng một kiểu vật: một **quy tắc** nhận một
con số vào và trả một con số ra. Gọi tạm nó là **cái máy** cho dễ nói. Xe của
Byte đang có ba cái, và cả ba đều dựng trên chuyện có thật của một buổi bán
hàng:

- **Máy tiền** — bỏ *số ổ* vào, nhả *số tiền thu* ra. Quy tắc: nhân với 15000.
- **Sổ giờ** — bỏ *một giờ trên đồng hồ* vào, hỏi *giờ ấy bán được mấy ổ*.
- **Sổ ngược** — bỏ *một số ổ* vào, hỏi *lúc mấy giờ bán được đúng chừng ấy*.

Hai cái sau đọc từ cùng một cuốn sổ, chỉ khác chiều hỏi. Sổ chiều nay như sau:

| giờ | số ổ bán trong giờ đó |
| --- | --- |
| 6 | 5 |
| 8 | 12 |
| 10 | 9 |
| 12 | 7 |
| 14 | 3 |
| 16 | 12 |
::::

::::example{#thu-tung-cai-mot}
Đem ra thử từng cái. Mỗi lần chỉ hỏi đúng một câu:

> **Bỏ một con số vào thì nhận về mấy con số?**

**Máy tiền.** Bỏ 4 vào, ra 60000. Bỏ 4 vào lần nữa, vẫn ra 60000. Bỏ 7 vào, ra
105000. Hỏi bao nhiêu lần cũng thế: một số vào, **đúng một** số ra, và lần sau
bỏ đúng số ấy vào thì nhận về đúng số ấy.

**Sổ giờ.** Bỏ 8 vào, sổ trả 12. Bỏ 16 vào, sổ cũng trả 12. Hai đầu vào khác
nhau, cùng một đầu ra — nghe hơi kỳ, nhưng thử lại câu hỏi gốc: mỗi lần bỏ
**một** giờ vào, sổ trả lại **đúng một** con số. Hỏi "8 giờ bán mấy ổ" không
bao giờ có hai câu trả lời. Cuốn sổ này tra được.

**Sổ ngược.** Bỏ 12 vào, sổ trả 8 **và** 16. Một lần hỏi, hai câu trả lời. Muốn
ghi kết quả vào một chỗ nhớ cũng không ghi nổi — ghi cái nào?

Chấm cuốn sổ ấy lên hai trục, số ổ nằm ngang và giờ dựng đứng:

```text
  giờ
   ↑
 16│                                   ●
 14│        ●
 12│                    ●
 10│                          ●
  8│                                   ●
  6│              ●
   └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──→  số ổ bán trong một giờ
   0  1  2  3  4  5  6  7  8  9  10 11 12
```

Đi ngang tới 12 rồi ngước thẳng lên: cột dựng đứng ở đó xuyên qua **hai** dấu
chấm. Đó chính là chỗ hỏng, và nó nhìn thấy được:

> Đặt một cây thước dựng đứng ở bất kỳ chỗ nào trên trục nằm rồi trượt nó từ
> trái sang phải. Nếu có lúc nào thước chạm hai dấu chấm cùng một lúc thì cái
> hình ấy không tra được.

Đồ thị của bài trước thì trượt thước từ đầu tới cuối cũng không bao giờ chạm
hai — mỗi số ổ đúng một dấu chấm.
::::

::::explain{#dat-ten-cho-cho-khac-nhau}
Hai cái tra được, một cái không. Chỗ khác nhau vừa được nói ra ba lần, ba kiểu:
bằng cuốn sổ, bằng câu chuyện của xe, và bằng cây thước dựng đứng. Ba kiểu cùng
chỉ vào một điều, nên điều ấy đáng có một cái tên:

> Một quy tắc mà **mỗi đầu vào cho ra đúng một đầu ra** thì gọi là một
> **hàm số**.

Hai chỗ đáng cân trong câu ấy:

- **đúng một** — không phải "nhiều nhất một", cũng không phải "ít nhất một".
  Bỏ vào một lần thì nhận về một con số, và lần sau bỏ đúng số ấy vào thì nhận
  về đúng con số ấy chứ không phải con số khác.
- **đầu vào** và **đầu ra** — hai vai khác nhau, không đổi chỗ được, đúng như
  hai ngăn của cặp toạ độ ở bài 22. Đầu vào đi ngang, đầu ra đi lên.

Và đây là chỗ luật này **không** cấm, chỗ dễ nhầm nhất:

> Hai đầu vào khác nhau **được phép** cho cùng một đầu ra.

Sổ giờ làm đúng thế: 8 giờ và 16 giờ cùng ra 12 ổ. Nó vẫn là một hàm số đàng
hoàng. Luật chỉ chặn một chiều — chiều "một vào, nhiều ra". Chiều "nhiều vào,
một ra" thì để ngỏ.

Nhớ kỹ chỗ để ngỏ này. Nó sẽ quay lại, và lúc quay lại thì nó gây phiền.
::::

::::predict{#doan-hoi-hai-lan commitOnce}
Đem đúng hai điều vừa nói ra hỏi máy: một, hỏi máy tiền hai lần cùng một con
số; hai, so hai giờ khác nhau của cuốn sổ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
gia_mot_o = 15000

# Máy tiền: bỏ 4 ổ vào, hỏi hai lần.
print(gia_mot_o * 4)
print(gia_mot_o * 4)

# Sổ chiều nay: 8 giờ bán 12 ổ, 16 giờ cũng bán 12 ổ.
o_luc_8_gio = 12
o_luc_16_gio = 12
print(o_luc_8_gio == o_luc_16_gio)
print(8 == 16)
```

:::opt{correct}
60000, 60000, True, False
:::

:::opt
60000, 60000, True, True
::why
Gần đúng ở chỗ bạn nhìn ra hai giờ ấy "bằng nhau" theo một nghĩa có thật: chúng
cho cùng một kết quả. Quy tắc bạn đang dùng — *hai thứ cho cùng một kết quả thì
coi như một* — đúng ở chỗ nó thường được dùng: `1/2` và `2/4` rơi trúng cùng
một chỗ trên trục số, nên chúng đúng là một số.

Chỗ lệch là ở đó hai **cách viết** cùng chỉ vào một **con số**. Còn ở đây, 8 và
16 là hai con số khác nhau thật, chúng chỉ tình cờ cho cùng một đầu ra. Câu
`8 == 16` hỏi về chính hai con số ấy, không hỏi về thứ chúng nhả ra. Và chính
chỗ này là điều bài muốn bạn thấy: **hai đầu vào khác nhau vẫn được phép cho
cùng một đầu ra**, mà không làm hỏng cái máy.
::
:::

:::opt
60000, 60000, False, False
::why
Gần đúng ở chỗ bạn nhớ đúng một luật của R0.11: `o_luc_8_gio` và `o_luc_16_gio`
là hai cái tên khác nhau, nên chúng là hai chỗ nhớ khác nhau — sửa chỗ này
không đụng gì tới chỗ kia.

Chỗ lệch là hai chỗ nhớ khác nhau vẫn hoàn toàn được phép giữ **cùng một con
số**. Dấu `==` hỏi về con số đang nằm bên trong, không hỏi về cái nhãn dán bên
ngoài. Cả hai đang giữ số 12, nên câu trả lời là `True` — và đó là cách máy nói
lại đúng cái sự thật của cuốn sổ: hai giờ ấy bán bằng nhau.
::
:::

:::opt
60000, 120000, True, False
::why
Gần đúng ở chỗ bạn nhớ một hình ảnh rất thật từ Realm 0: hai dòng viết giống hệt
nhau nối đuôi nhau thì lần sau **không** ra như lần trước — `vi_tri = vi_tri + 1`
viết ba lần cho ba kết quả khác nhau. Quy tắc ấy đúng, và nó là một trong những
chỗ khó nhất của Realm 0.

Chỗ lệch là ở đó mỗi dòng **ghi đè** cái tên rồi dòng sau đọc lại giá trị mới.
Còn hai dòng ở đây không dòng nào đổi `gia_mot_o` cả; chúng chỉ **đọc** rồi
tính. Hỏi hai lần thì trả lời hai lần y hệt nhau — và đúng chỗ đó mới làm nó
thành một cái máy dùng được. Một cái máy mà hỏi hai lần ra hai số thì không tra
được nữa.
::
:::
::::

::::explain{#hai-chieu-khong-doi-nhau}
Máy vừa nói ra cả hai vế của cái luật, mỗi vế một dòng:

- `gia_mot_o * 4` hai lần cho cùng một số — **một đầu vào, đúng một đầu ra**.
- `8 == 16` là `False` trong khi `o_luc_8_gio == o_luc_16_gio` là `True` — **hai
  đầu vào khác nhau, một đầu ra**, và không có gì hỏng cả.

Hai dòng ấy dễ nhìn nhầm thành một. Chúng ngược chiều nhau: cái đầu nói về việc
hỏi **cùng một** đầu vào hai lần, cái sau nói về việc hỏi **hai** đầu vào khác
nhau. Chỉ chiều thứ nhất mới là điều kiện để gọi là hàm số.
::::

::::code{#kiem-hai-chieu}
Cho máy tự kiểm cả hai chiều trên đúng những con số của xe. Xe bán 15000 đồng
một ổ; buổi ấy Byte bán được `6` ổ. Sổ chiều nay ghi 8 giờ bán 12 ổ và 16 giờ
cũng bán 12 ổ.

Điền năm chỗ trống. Ba câu hỏi cuối đều là câu đúng-hay-sai, và chúng **không**
cùng một câu trả lời — nên điền một thứ giống nhau vào cả năm chỗ là hỏng ngay.

Gõ cứng `90000` vào cũng hỏng: con số ấy chính là thứ cái máy phải tự nhả ra.

```python title=starter
gia_mot_o = 15000

# MÁY TIỀN: bỏ số ổ vào, nhả số tiền ra.
so_o = 6
hoi_lan_dau = ___        # bỏ 6 ổ vào máy
hoi_lan_nua = ___        # bỏ đúng 6 ổ ấy vào lần nữa
mot_vao_mot_ra = ___     # hai lần hỏi có nhả ra cùng một số không?

# SỔ GIỜ: 8 giờ bán 12 ổ, 16 giờ cũng bán 12 ổ.
gio_sang = 8
gio_chieu = 16
o_gio_sang = 12
o_gio_chieu = 12
hai_gio_la_mot = ___     # 8 giờ và 16 giờ có phải cùng một giờ không?
cung_mot_so_o = ___      # hai giờ ấy có bán ra cùng một số ổ không?

print(hoi_lan_dau)
print(hoi_lan_nua)
print(mot_vao_mot_ra)
print(hai_gio_la_mot)
print(cung_mot_so_o)
```

```python title=solution
gia_mot_o = 15000

# MÁY TIỀN: bỏ số ổ vào, nhả số tiền ra.
so_o = 6
hoi_lan_dau = gia_mot_o * so_o        # bỏ 6 ổ vào máy
hoi_lan_nua = gia_mot_o * so_o        # bỏ đúng 6 ổ ấy vào lần nữa
mot_vao_mot_ra = hoi_lan_dau == hoi_lan_nua

# SỔ GIỜ: 8 giờ bán 12 ổ, 16 giờ cũng bán 12 ổ.
gio_sang = 8
gio_chieu = 16
o_gio_sang = 12
o_gio_chieu = 12
hai_gio_la_mot = gio_sang == gio_chieu
cung_mot_so_o = o_gio_sang == o_gio_chieu

print(hoi_lan_dau)
print(hoi_lan_nua)
print(mot_vao_mot_ra)
print(hai_gio_la_mot)
print(cung_mot_so_o)
```

```python title=test
# Hai câu `!=` đứng trước, vì chúng canh đúng cái bẫy của bài: điền một thứ
# giống nhau vào mọi chỗ trống thì ba câu đúng-sai hoá ra bằng nhau hết, và mọi
# câu `==` phía sau sẽ không bao giờ chạy tới.
assert hoi_lan_dau != so_o, "`hoi_lan_dau` phải là số TIỀN máy nhả ra, không phải số ổ bỏ vào"
assert hai_gio_la_mot != cung_mot_so_o, "8 giờ khác 16 giờ, mà hai giờ ấy lại bán ra cùng một số ổ — hai câu này không thể có chung một câu trả lời"
assert hoi_lan_dau == 90000, "6 ổ × 15000 đồng = 90000 đồng"
assert hoi_lan_nua == hoi_lan_dau, "bỏ cùng một số ổ vào thì máy phải nhả ra cùng một số tiền — chính chỗ này làm nó là một hàm số"
assert mot_vao_mot_ra == True, "hai lần hỏi ra cùng một số, nên câu trả lời phải là True"
assert hai_gio_la_mot == False, "8 và 16 là hai giờ khác nhau — đây là hai đầu vào khác nhau"
assert cung_mot_so_o == True, "cả hai giờ đều bán 12 ổ — hai đầu vào khác nhau, một đầu ra, và cuốn sổ vẫn tra được"
```

:::hints
- kind: attention
  body: Hai chỗ trống đầu hỏi cùng một câu và phải viết giống hệt nhau — đó là chủ ý, vì bài đang hỏi "bỏ cùng một số vào hai lần thì sao". Ba chỗ trống còn lại không hỏi số, chúng hỏi đúng hay sai, nên thứ điền vào phải là một phép so sánh.
- kind: strategy
  body: Số tiền tính bằng giá một ổ nhân số ổ, cả hai lần dùng đúng hai cái tên ấy. Ba câu đúng-sai thì mỗi câu so đúng hai cái tên mà lời chú thích bên cạnh đang hỏi tới: hai lần hỏi máy, hai giờ trên đồng hồ, hai số ổ của hai giờ đó. Đọc kỹ chú thích — sẽ có một câu trả lời là sai, và sai ở đó mới là điều bài muốn bạn thấy.
- kind: one-line
  body: "Lần lượt là `gia_mot_o * so_o`, `gia_mot_o * so_o`, `hoi_lan_dau == hoi_lan_nua`, `gio_sang == gio_chieu`, `o_gio_sang == o_gio_chieu`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hai chỗ đầu phải là phép nhân dựng từ hai cái tên có sẵn, ba chỗ sau phải là phép so sánh thật — gõ thẳng số hay gõ thẳng `True`/`False` thì máy không kiểm gì cả, bạn kiểm hộ nó rồi
  requireAst:
  # Hai phép nhân, đúng hai lần hỏi máy. Khung khởi đầu không có dấu nhân nào.
  - kind: uses-operator, target: *, min: 2
  # Ba câu đúng-sai phải là ba phép so sánh, không phải ba chữ gõ tay.
  - kind: uses-operator, target: ==, min: 3
  - kind: uses-name, target: gia_mot_o, min: 2
  - kind: uses-name, target: so_o, min: 2
  # `hoi_lan_dau` và `hoi_lan_nua` mỗi cái được đọc hai lần: một lần trong câu
  # so sánh, một lần ở `print`. Khung khởi đầu chỉ đọc chúng ở `print`.
  - kind: uses-name, target: hoi_lan_dau, min: 2
  - kind: uses-name, target: hoi_lan_nua, min: 2
  - kind: uses-name, target: gio_sang, min: 1
  - kind: uses-name, target: gio_chieu, min: 1
  - kind: uses-name, target: o_gio_sang, min: 1
  - kind: uses-name, target: o_gio_chieu, min: 1
  forbidAst:
  # Số tiền là KẾT QUẢ máy phải tự nhả ra. Lời giải thật không chứa nó.
  - kind: has-literal, target: 90000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^90000\n90000\nTrue\nFalse\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hỏi hai lần ra một số. Hai giờ khác nhau mà bán bằng nhau vẫn không sao. Mình
phân biệt được rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đếm lại xem xe bánh mì đang có bao nhiêu cái máy dùng được. Cả ba đều nhận đúng
một thứ vào — **số ổ bán được** — và cả ba đều là hàm số:

- bỏ số ổ vào, nhả ra **tiền thu**: mỗi ổ 15000 đồng;
- bỏ số ổ vào, nhả ra **tiền lãi**: mỗi ổ lãi ít hơn thế nhiều;
- bỏ số ổ vào, nhả ra **số ly trà đá** bán kèm.

Và đúng vì cả ba nhận cùng một thứ mà bắt đầu loạn. Thử nói câu này ra miệng:
*"bỏ 20 vào cái máy thì được bao nhiêu?"* — câu ấy chưa hỏi được gì cả, vì
chẳng ai biết bạn đang nói tới máy nào. Muốn nói rõ thì phải chép lại nguyên cả
câu tính, mỗi lần một lần.

Bài 2 của track này đã gặp đúng loại rắc rối ấy một lần rồi, và gỡ được: một
con số chưa biết thì dán cho nó một cái tên, `n`. Nhưng lần này thứ cần tên
không phải một con số — nó là **cả cái quy tắc**.

Đặt tên cho một quy tắc thì viết ra sao? Và đặt xong rồi, làm sao nói gọn câu
"bỏ 20 vào nó" mà không phải chép lại câu tính lần nữa?

Bài sau đặt tên cho cái máy.
::::

::::checkpoint{mastery=0.8}
::::
