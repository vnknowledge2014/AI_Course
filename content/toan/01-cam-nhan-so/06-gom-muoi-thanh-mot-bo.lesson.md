---
id: toan.cam-nhan-so.gom-muoi-thanh-mot-bo
title: Gom mười thành một bó
summary: Buộc mười hạt lại thành một bó rồi đi đếm bó — và cái luật ấy dùng lại được cho chính mấy cái bó.
locale: vi
track: toan
module: cam-nhan-so
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.dong-goi]
requires: [math.thanh-so, math.don-vi-roi-va-lien, math.don-vi, core.output, core.variable, core.print-variable, core.arithmetic, core.boolean, ctrl.comparison]
concepts: [math.dong-goi, math.don-vi, math.don-vi-moi]
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
Đếm một trăm ba mươi bảy lần thì mình nhầm. Đếm mười một lần thì mình không nhầm.
::::

::::explain{#buoc-lai-thanh-bo}
Câu hỏi bài trước để lại: chỉ vào chỗ của `137` trên thanh số thì phải đếm qua
137 cái vạch, mà đếm 137 lần thì tay mỏi mắt lạc. Ngoài chợ người ta bán trăm
quả trứng suốt ngày mà chẳng ai đếm tới một trăm — họ làm thế nào?

Họ **không đếm trứng**. Họ đếm **chục**.

Một chục là mười quả buộc sẵn thành một khay. Người bán không nhìn ba mươi quả
trứng nữa, họ nhìn ba cái khay. Ba lần đếm thay cho ba mươi lần.

Byte làm y hệt thế với hạt. Luật của Byte gọn một dòng:

> Đủ **mười** hạt lẻ thì buộc lại thành **một bó**, rồi đem đếm bó.

Đổ 137 hạt ra bàn và buộc:

```text
trước:   ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● …   (đếm 137 lần)

sau:     [●●●●●●●●●●] × 13  và  ● ● ● ● ● ● ●
         13 bó                  7 hạt lẻ            (đếm 13 + 7 = 20 lần)
```

Số hạt trên bàn không đổi một hạt nào — vẫn đúng chừng ấy. Chỉ có **số lần
phải đếm** tụt từ 137 xuống 20.

Cái bó ấy không phải một mẹo xếp đồ. Nó là một **đơn vị mới**, to gấp mười đơn
vị cũ, đúng nghĩa cái thước mới của bài 3: đổi thước thì con số đổi, còn lượng
đất — ở đây là đống hạt — không đổi tí nào. Đo đống hạt bằng thước "một hạt"
thì ra 137; đo cũng đống ấy bằng thước "một bó" thì ra 13 bó và thừa 7.

**Luật ấy có dùng lại được cho chính mấy cái bó không?**

Có. Mười ba cái bó vẫn còn nhiều, mà bó cũng là vật đếm được như hạt. Nên áp
đúng cái luật cũ lần thứ hai:

> Đủ **mười bó** thì buộc lại thành **một bó-của-bó**.

```text
1 bó-của-bó       3 bó            7 hạt lẻ
⟦ bó ×10 ⟧        [bó][bó][bó]    ● ● ● ● ● ● ●     (đếm 1 + 3 + 7 = 11 lần)
```

Từ 137 lần đếm xuống 11 lần, và không lần nào phải đếm quá mười. Đây không
phải luật mới — nó là **cùng một luật, chạy lần thứ hai**. Và nó chạy được lần
thứ ba, thứ tư, không có chỗ dừng.
::::

::::example{#kiem-lai-so-hat}
Bó thì gọn, nhưng gọn mà làm mất hạt thì hỏng. Bắt máy kiểm xem sau hai lần
buộc, đống hạt còn nguyên không:

```python title=readonly
bo_cua_bo = 10 + 10 + 10 + 10 + 10 + 10 + 10 + 10 + 10 + 10
print(bo_cua_bo)

print(bo_cua_bo + 10 + 10 + 10 + 7)
print(bo_cua_bo + 10 + 10 + 10 + 7 == 137)
```

Máy in ra:

```text
100
137
True
```

Dòng đầu đếm mười cái bó, mỗi bó mười hạt: một bó-của-bó là **100 hạt**.

Dòng thứ hai gộp lại đúng những gì còn trên bàn — 1 bó-của-bó, 3 bó, 7 hạt lẻ
— và ra `137`. Dòng thứ ba hỏi thẳng máy câu đó, máy trả `True`.

Để ý một chuyện: bên trái dấu `==` có **năm** con số, bên phải có một. Cả hai
bên là cùng một đống hạt. Buộc bó chỉ đổi cách nói về đống, không đổi đống.
::::

::::predict{#doan-so-vat-tren-ban commitOnce}
Byte đổ 137 hạt ra bàn rồi buộc mười thành một bó. Buộc xong, trên bàn có 13
cái bó và 7 hạt lẻ.

Bây giờ Byte đếm **số vật trên bàn** — mỗi cái bó tính là **một** vật, dù trong
nó có mười hạt.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
so_vat_tren_ban = 13 + 7
print(so_vat_tren_ban == 137)
```

:::opt{correct}
False
:::

:::opt
True
::why
Gần đúng ở chỗ bạn đang giữ chắc điều quan trọng nhất của bài: buộc bó **không
làm mất hạt nào**, trên bàn vẫn đúng 137 hạt như lúc mới đổ ra. Ý nghĩ ấy đúng,
và nó là lý do cả cái luật đóng gói này dùng được.

Chỗ lệch là phạm vi: dòng trên không đếm **hạt**, nó đếm **vật**. Một cái bó là
một vật để cầm lên, dù bên trong là mười hạt. 13 bó với 7 hạt lẻ là 20 vật —
và đúng 20 vật ấy vẫn đang chứa đủ 137 hạt. Hai con số cùng đúng, chỉ đang trả
lời hai câu hỏi khác nhau.
::
:::

:::opt
20
::why
Gần đúng ở chỗ bạn cộng đúng: `13 + 7` cho ra `20`, và đó chính là con số mà
dòng thứ nhất vừa cất vào tên `so_vat_tren_ban`.

Chỗ lệch nằm ở dòng thứ hai. `print` ở đây không in con số ấy ra; nó in **câu
trả lời cho một câu hỏi có–không**, thứ mà Realm 0 đã dạy là luôn ra `True`
hoặc `False`. Muốn nhìn thấy `20` thì bỏ hẳn phần `== 137` đi.
::
:::

:::opt
Máy báo lỗi, vì 20 vật và 137 hạt là hai đơn vị khác nhau nên không so được
::why
Gần đúng ở chỗ bạn đang nghĩ đúng theo bài 1: một con số phải là "mấy **cái
gì**", và `20 vật` với `137 hạt` đúng là hai thứ không đem so thẳng được. Sự
cảnh giác ấy còn trả lãi to ở bài 11, chỗ nói vì sao chỉ gộp được thứ cùng đơn
vị.

Chỗ lệch: cái đơn vị nằm trong đầu bạn, không nằm trong máy. Máy chỉ thấy hai
con số trần `20` và `137`, so xong rồi trả `False` — nó không biết bên nào là
vật, bên nào là hạt. Người canh đơn vị là bạn, không phải nó.
::
:::
::::

::::explain{#bo-la-don-vi-moi}
Rút ra ba điều để mang theo:

- **Đóng gói** là gom một số cố định vật rời thành một vật mới rồi đi đếm vật
  mới. Byte gom theo **mười**, nên một bó là mười hạt.
- Cái bó là một **đơn vị mới**, to gấp mười đơn vị cũ. Nó là cái thước của bài
  3 phóng to lên mười lần — nên đo cùng một đống, số đo nhỏ đi mười lần.
- Cùng một luật **lặp lại được**: đủ mười bó thì thành một bó-của-bó, đủ mười
  bó-của-bó lại thành một bó to hơn nữa. Mỗi lần lặp, cái thước lại to gấp
  mười.

Nhờ lặp được như thế, đống nào to tới đâu cũng đếm được mà không lần nào phải
đếm quá chín. Đó là toàn bộ chỗ lợi.
::::

::::code{#dem-bo-thay-vi-dem-hat}
Byte và An mỗi người một đống, cùng buộc theo luật mười.

Điền hai chỗ trống:

1. `bo_cua_bo` — một bó-của-bó là mấy **hạt**? (mười bó, mà mỗi bó mười hạt)
2. `dong_cua_an` — An buộc xong được **4 bó và 5 hạt lẻ**. Đống của An là mấy
   **hạt**?

Hai chỗ ấy đòi hai con số khác hẳn nhau, và bài chấm bằng cả hai — nên một con
số gõ cứng vào cả hai chỗ thì trượt ít nhất một câu.

```python title=starter
# 1) Một bó-của-bó là mấy HẠT?
bo_cua_bo = ___

# 2) An có 4 bó và 5 hạt lẻ. Đống của An là mấy HẠT?
dong_cua_an = ___

print(bo_cua_bo)
print(dong_cua_an)
```

```python title=solution
# 1) Một bó-của-bó là mấy HẠT?
bo_cua_bo = 100

# 2) An có 4 bó và 5 hạt lẻ. Đống của An là mấy HẠT?
dong_cua_an = 45

print(bo_cua_bo)
print(dong_cua_an)
```

```python title=test
# Vế phải của hai câu đầu cố ý viết bằng phép cộng chứ không bằng một con số
# gõ sẵn: cộng từng cái mười CHÍNH LÀ việc "đếm bó" mà bài vừa dạy. Nó cũng
# không mượn phép nhân — thứ mà track này chưa đi tới.
assert bo_cua_bo == 10 + 10 + 10 + 10 + 10 + 10 + 10 + 10 + 10 + 10, "một bó-của-bó là MƯỜI BÓ, mà mỗi bó là mười hạt"
assert dong_cua_an == 10 + 10 + 10 + 10 + 5, "4 bó là bốn lần mười hạt, rồi cộng thêm 5 hạt lẻ"
# Câu cuối buộc con số vừa điền phải khớp với chính đống 137 hạt trong bài:
# 1 bó-của-bó, 3 bó và 7 hạt lẻ phải quay về đúng chỗ cũ.
assert bo_cua_bo + 10 + 10 + 10 + 7 == 137, "buộc bó không được làm mất hạt nào — gộp lại phải ra đúng đống ban đầu"
```

:::hints
- kind: attention
  body: Cả hai chỗ trống đều hỏi "mấy HẠT", không hỏi mấy bó. Nên trước khi viết, đổi mọi cái bó về hạt đã.
- kind: strategy
  body: Một bó là mười hạt. Muốn biết mười bó là mấy hạt thì cộng mười cái mười lại — phần máy in ở trên đã làm sẵn phép cộng ấy một lần. Với đống của An thì cộng bốn cái mười, rồi nhớ cộng nốt mấy hạt lẻ còn rời.
- kind: one-line
  body: "Viết `100` vào chỗ trống thứ nhất và `45` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mười một lần đếm cho một trăm ba mươi bảy hạt. Tay mình đỡ mỏi hẳn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Buộc bó xong thì đếm nhẹ hẳn. Nhưng Byte còn phải **ghi lại** ra giấy để mai
đọc lại, và chỗ này thì bí.

Byte có **13 bó và 7 hạt lẻ**. Vẽ 13 cái bó ra giấy thì lại đúng cái công đếm
vừa tránh được. Viết chữ "13 bó, 7 hạt" thì dài, mà cả vườn có mấy chục đống
như thế.

Thử viết gọn hết cỡ: `13` với `7` đặt cạnh nhau, thành `137`. Nhưng lúc đọc
lại, lấy gì để biết con `13` là **bó** còn con `7` là **hạt** — chứ không phải
ngược lại, hay không phải cả `137` là hạt?

Bài sau trả lời, và câu trả lời không nằm ở việc viết thêm chữ.
::::

::::checkpoint{mastery=0.8}
::::
