---
id: onboarding.may-tinh-noi-gi.doi-gia-tri-cua-ten
title: Đổi giá trị của một cái tên
summary: Cái tên không dính chết vào một giá trị. Nó chuyển được — và bài này nói giá trị cũ đi đâu.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [core.reassign]
requires: [core.variable]
concepts: [core.bien, core.gan-lai]
gradingMatrix:
  web-chrome: [static, run, tests]
  web-firefox: [static, run, tests]
  macos: [static, run, tests]
  windows: [static, run, tests]
  linux: [static, run, tests]
  android: [static, run, tests]
  ios: [static, run, tests]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Bài trước bạn dán một cái tên lên một giá trị. Giờ thử gỡ ra, dán sang chỗ khác.
::::

::::explain{#tam-the-tren-quai-noi}
Câu hỏi bỏ ngỏ cuối bài trước là: nếu bạn dán **chính cái tên đó** lên một giá
trị khác, thì giá trị cũ đi đâu?

Trước khi trả lời bằng code, hãy nhìn vào bếp.

Chị Hạnh có hai nồi nước dùng đặt cạnh nhau: một nồi bò, một nồi gà. Trên quai
nồi bò buộc một tấm thẻ gỗ, viết bằng bút dạ:

```text
NỒI ĐANG BÁN
```

Trong bếp không ai gọi "nồi bên trái" hay "nồi to hơn". Ai cần múc nước dùng thì
đi tìm tấm thẻ.

Trưa, nồi bò vơi dần, chị Hạnh chuyển sang bán gà. Chị làm đúng một việc: gỡ tấm
thẻ khỏi quai nồi bò, buộc sang quai nồi gà.

Hãy để ý chị **không** đổ nồi bò đi. Nồi bò vẫn đứng nguyên chỗ cũ, vẫn còn chừng
ấy nước dùng. Thứ di chuyển là **tấm thẻ**, không phải cái nồi.

Nhưng từ giây phút ấy, ai hô "múc nồi đang bán" cũng được đưa nước dùng gà.

Cái tên bạn đặt ở bài trước chính là tấm thẻ đó.
::::

::::example{#buoc-the-lan-thu-hai}
Đây là tấm thẻ được chuyển, viết bằng Python — và đây đúng là hai dòng bài
trước đã hẹn sẽ chạy, không phải hai dòng nào khác. Tuần trước quán tăng giá
tô đặc biệt lên 65.000đ; tuần này quán hạ xuống 60.000đ.

```python title=readonly
gia_dac_biet = 65000
gia_dac_biet = 60000
```

Đọc hai dòng này thành **hai việc xảy ra lần lượt**, chứ không phải hai lời tuyên
bố cùng đứng một chỗ:

- Dòng 1: máy làm ra giá trị `65000`, rồi buộc tấm thẻ `gia_dac_biet` vào nó.
- Dòng 2: máy làm ra giá trị `60000`, rồi **gỡ** tấm thẻ `gia_dac_biet` ra khỏi chỗ cũ và buộc vào giá trị mới.

Chỗ này cần nói thẳng ra, vì nó là chỗ vấp của gần như tất cả người mới: dấu `=`
đây **không phải** dấu "bằng" trong sách toán.

Trong sách toán, viết `gia_dac_biet = 65000` rồi viết tiếp `gia_dac_biet = 60000` là tự mâu
thuẫn — một thứ không thể vừa bằng con số này vừa bằng con số kia. Ở đây không có
mâu thuẫn nào cả, vì `=` không nói *hai bên bằng nhau*. Nó ra một mệnh lệnh:

> Buộc cái tên bên trái vào giá trị bên phải.

Mệnh lệnh thì làm xong là xong. Và ra lệnh lại lúc nào cũng được.
::::

::::predict{#the-dang-o-dau commitOnce}
Byte vừa chạy đúng hai dòng ở trên. **Trước khi xem đáp án**, bạn đoán tấm thẻ
`gia_dac_biet` bây giờ đang buộc vào con số nào?

```python
gia_dac_biet = 65000
gia_dac_biet = 60000
```

:::opt{correct}
60000 — chỉ một mình nó
:::

:::opt
65000, vì dòng đầu tiên mới là dòng đặt tên
::why
Gần đúng ở chỗ bạn nhớ chính xác dòng đầu đã làm gì: nó buộc thẻ `gia_dac_biet` vào
`65000`. Suốt cả bài trước, điều đó đúng.

Chỗ lệch nằm ở thứ tự. Máy đọc từ trên xuống, nên dòng thứ hai chạy **sau** — và
lúc nó chạy xong thì tấm thẻ đã không còn ở nồi cũ nữa. Dòng đầu không phải một
lời hứa giữ mãi mãi; nó chỉ là việc đã làm ở giây thứ nhất.
::
:::

:::opt
Cả hai: `gia_dac_biet` giữ 65000 và 60000
::why
Gần đúng ở chỗ bạn nhận ra cả hai con số đều đã thật sự có mặt trong máy. Chúng
được làm ra thật, không phải bịa.

Chỗ lệch: một tấm thẻ chỉ buộc được vào một cái nồi. Muốn giữ lại cả hai con số
thì phải có **hai** cái tên khác nhau — chẳng hạn `gia_cu` và `gia_moi`. Một cái
tên, tại một lúc, trỏ vào đúng một giá trị.
::
:::

:::opt
Máy dừng lại và báo lỗi, vì cái tên `gia_dac_biet` đã dùng ở dòng trên rồi
::why
Gần đúng ở chỗ bạn đang đọc dấu `=` như một lời tuyên bố — mà tuyên bố hai lần
với hai con số khác nhau thì nghe đúng là vô lý. Cách đọc ấy rất tự nhiên, vì
trong sách toán dấu `=` đúng là một lời tuyên bố.

Chỗ lệch: ở đây `=` là một **mệnh lệnh**. "Buộc thẻ vào giá trị này." Ra lệnh lần
thứ hai thì máy chỉ việc buộc lại lần nữa. Không có gì để mà mâu thuẫn.
::
:::
::::

::::explain{#sau-hai-dong-con-lai-gi}
Đúng như bạn vừa đoán: sau hai dòng, trong máy có đúng **một** tấm thẻ mang tên
`gia_dac_biet`, và nó đang buộc vào `60000`. Không phải hai tấm thẻ, cũng không phải
một tấm thẻ buộc vào hai chỗ.
::::

::::explain{#gia-tri-cu-di-dau}
Giờ trả lời thẳng câu hỏi cuối bài trước: `65000` đi đâu?

Trong bếp, nồi phở bò vẫn còn — chị Hạnh nhìn thấy nó, muốn múc thì múc. Trong
máy thì khác đúng một chi tiết, và chi tiết đó quyết định mọi thứ: **cách duy
nhất để bạn với tới một giá trị là gọi tên nó.** Không có mắt nào nhìn vào bộ nhớ
cả.

Tấm thẻ vừa chuyển đi, nên từ giờ không còn đường nào dẫn tới `65000` nữa. Nó
không nổ, không báo lỗi, không để lại dấu vết. Máy dọn nó đi lúc nào thì bạn
không cần bận tâm. Với chương trình của bạn, coi như nó không còn.

Nên câu trả lời hoá ra ngược với cách hỏi: giá trị cũ **không đi đâu cả**. Chính
cái tên mới là thứ đi.

Từ đây trở đi, việc này có một tên gọi: **gán lại**. Gán lần đầu là dán thẻ lên
một giá trị; gán lại là chuyển thẻ ấy sang giá trị khác.
::::

::::explain{#vi-sao-viec-nay-quan-trong}
Một cái tên đổi được giá trị nghe như chuyện vặt. Thật ra nó là thứ khiến chương
trình khác hẳn một tờ giấy dán tường.

Tờ giấy dán tường nói cùng một câu mãi mãi. Còn `so_to_da_ban` thì sáng bằng `0`,
giữa buổi bằng `12`, cuối ngày bằng `47` — cùng một cái tên, đi theo quán suốt cả
ngày. Chị Hạnh không cần nghĩ ra bốn mươi bảy cái tên khác nhau.

Gần như mọi thứ bạn học từ đây trở đi — đếm, cộng dồn, lặp lại — đều dựa lên đúng
một dòng như dòng vừa rồi.
::::

::::code{#tam-the-di-theo-buoi}
Đến lượt bạn chuyển tấm thẻ.

Quán đếm số tô bán được trong ngày. **Đầu buổi sáng** con số ấy là `0`. Tới
**giữa buổi**, quán đã bán được `12` tô.

Chỉ dùng **một** cái tên `so_to_da_ban` cho cả hai lúc — đúng như đoạn trên vừa
nói: chị Hạnh không nghĩ ra một cái tên mới cho mỗi con số.

Dòng đầu đã viết sẵn. Bạn viết dòng thứ hai.

```python title=starter
so_to_da_ban = 0
___
```

```python title=solution
so_to_da_ban = 0
so_to_da_ban = 12
```

```python title=test
# Bài này chấm bằng STATIC. Người học chưa có cách nào nhìn vào một cái tên —
# `print(so_to_da_ban)` là điều bất ngờ để dành cho bài sau, nên đoạn mã này
# cố ý không in gì cả. Khối test chỉ khẳng định nó chạy tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Dòng thứ hai trông rất giống dòng thứ nhất. Cùng một cái tên bên trái dấu `=`, chỉ khác con số bên phải.
- kind: strategy
  body: "Nhớ lại đoạn `gia_dac_biet` ở trên: chuyển thẻ chỉ là ra lệnh lại. Đừng nghĩ ra tên mới như `so_to_da_ban_2` — cả bài này nói rằng không cần."
- kind: one-line
  body: "Viết `so_to_da_ban = 12` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: static
  onFail: hãy gán lại ĐÚNG cái tên `so_to_da_ban` sang 12, đừng nghĩ ra một cái tên thứ hai — chuyện một cái tên đi theo cả ngày chính là điều bài này dạy
  requireAst:
  - kind: gan-ten, target: so_to_da_ban, min: 2
  - kind: has-literal, target: 12
:::
::::

::::reflect{#nghi-lai}
Suốt hai bài vừa rồi, bạn phải **tin lời Byte** về chuyện tấm thẻ đang buộc ở
đâu. Bạn chưa một lần nhìn thấy tận mắt.

Giờ thử ghép hai thứ bạn đã có trong tay. Bài 1 dạy `print` — nói ra một câu. Bài
này cho bạn một cái tên đang giữ một giá trị:

```python
mon_an = "Phở bò tái nạm"
print(mon_an)
```

Để ý `mon_an` nằm trong ngoặc mà **không có** dấu nháy. Bài 8 đã nói: thứ không
nháy là một cái tên, và máy phải đi tìm cái tên đó — tìm không ra thì nó dừng lại
báo `NameError`. Lần này nó tìm **thấy**.

Vậy máy sẽ in ra chính chữ `mon_an`, hay in ra thứ đang buộc ở đầu kia tấm thẻ?

Đừng trả lời vội. Bài sau bạn chạy đúng đoạn này và tự nhìn.
::::

::::checkpoint{mastery=0.8}
::::
