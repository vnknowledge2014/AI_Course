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
  web-chrome: []
  web-firefox: []
  macos: []
  windows: []
  linux: []
  android: []
  ios: []
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
Đây là tấm thẻ được chuyển, viết bằng Python:

```python title=readonly
gia_pho = 45000
gia_pho = 50000
```

Đọc hai dòng này thành **hai việc xảy ra lần lượt**, chứ không phải hai lời tuyên
bố cùng đứng một chỗ:

- Dòng 1: máy làm ra giá trị `45000`, rồi buộc tấm thẻ `gia_pho` vào nó.
- Dòng 2: máy làm ra giá trị `50000`, rồi **gỡ** tấm thẻ `gia_pho` ra khỏi chỗ cũ và buộc vào giá trị mới.

Sau hai dòng, trong máy có đúng một tấm thẻ mang tên `gia_pho`, và nó đang buộc
vào `50000`.

Chỗ này cần nói thẳng ra, vì nó là chỗ vấp của gần như tất cả người mới: dấu `=`
đây **không phải** dấu "bằng" trong sách toán.

Trong sách toán, viết `gia_pho = 45000` rồi viết tiếp `gia_pho = 50000` là tự mâu
thuẫn — một thứ không thể vừa bằng con số này vừa bằng con số kia. Ở đây không có
mâu thuẫn nào cả, vì `=` không nói *hai bên bằng nhau*. Nó ra một mệnh lệnh:

> Buộc cái tên bên trái vào giá trị bên phải.

Mệnh lệnh thì làm xong là xong. Và ra lệnh lại lúc nào cũng được.
::::

::::predict{#the-dang-o-dau commitOnce}
Byte vừa chạy đúng hai dòng ở trên. **Trước khi xem đáp án**, bạn đoán tấm thẻ
`gia_pho` bây giờ đang buộc vào con số nào?

```python
gia_pho = 45000
gia_pho = 50000
```

:::opt{correct}
50000 — chỉ một mình nó
:::

:::opt
45000, vì dòng đầu tiên mới là dòng đặt tên
::why
Gần đúng ở chỗ bạn nhớ chính xác dòng đầu đã làm gì: nó buộc thẻ `gia_pho` vào
`45000`. Suốt cả bài trước, điều đó đúng.

Chỗ lệch nằm ở thứ tự. Máy đọc từ trên xuống, nên dòng thứ hai chạy **sau** — và
lúc nó chạy xong thì tấm thẻ đã không còn ở nồi cũ nữa. Dòng đầu không phải một
lời hứa giữ mãi mãi; nó chỉ là việc đã làm ở giây thứ nhất.
::
:::

:::opt
Cả hai: `gia_pho` giữ 45000 và 50000
::why
Gần đúng ở chỗ bạn nhận ra cả hai con số đều đã thật sự có mặt trong máy. Chúng
được làm ra thật, không phải bịa.

Chỗ lệch: một tấm thẻ chỉ buộc được vào một cái nồi. Muốn giữ lại cả hai con số
thì phải có **hai** cái tên khác nhau — chẳng hạn `gia_cu` và `gia_moi`. Một cái
tên, tại một lúc, trỏ vào đúng một giá trị.
::
:::

:::opt
Máy dừng lại và báo lỗi, vì cái tên `gia_pho` đã dùng ở dòng trên rồi
::why
Gần đúng ở chỗ bạn đang đọc dấu `=` như một lời tuyên bố — mà tuyên bố hai lần
với hai con số khác nhau thì nghe đúng là vô lý. Cách đọc ấy rất tự nhiên, vì
trong sách toán dấu `=` đúng là một lời tuyên bố.

Chỗ lệch: ở đây `=` là một **mệnh lệnh**. "Buộc thẻ vào giá trị này." Ra lệnh lần
thứ hai thì máy chỉ việc buộc lại lần nữa. Không có gì để mà mâu thuẫn.
::
:::
::::

::::explain{#gia-tri-cu-di-dau}
Giờ trả lời thẳng câu hỏi cuối bài trước: `45000` đi đâu?

Trong bếp, nồi phở bò vẫn còn — chị Hạnh nhìn thấy nó, muốn múc thì múc. Trong
máy thì khác đúng một chi tiết, và chi tiết đó quyết định mọi thứ: **cách duy
nhất để bạn với tới một giá trị là gọi tên nó.** Không có mắt nào nhìn vào bộ nhớ
cả.

Tấm thẻ vừa chuyển đi, nên từ giờ không còn đường nào dẫn tới `45000` nữa. Nó
không nổ, không báo lỗi, không để lại dấu vết. Máy dọn nó đi lúc nào thì bạn
không cần bận tâm. Với chương trình của bạn, coi như nó không còn.

Nên câu trả lời hoá ra ngược với cách hỏi: giá trị cũ **không đi đâu cả**. Chính
cái tên mới là thứ đi.

Từ đây trở đi, việc này có một tên gọi: **gán lại**. Gán lần đầu là dán thẻ lên
một giá trị; gán lại là chuyển thẻ ấy sang giá trị khác.
::::

::::explain{#ve-phai-lam-truoc}
Còn một luật nhỏ nữa, và nó quyết định phần còn lại của bài:

> Máy làm cho xong **vế phải** dấu `=` trước. Xong rồi mới đụng tới cái tên bên
> trái. Đây là điều quan trọng cần nhớ.

Đọc theo tấm thẻ: chị Hạnh phải bắc xong nồi gà rồi mới buộc thẻ sang. Không ai
buộc thẻ vào chỗ trống rồi mới đi nấu.

Luật này chưa cần đến khi vế phải là một con số viết sẵn như `50000` — chẳng có
gì để làm cho xong cả. Nó thành ra quan trọng ngay khi vế phải có một phép tính,
và quan trọng nhất khi phép tính ấy nhắc tới **chính cái tên đang đứng bên trái**.
::::

::::predict{#tu-cong-them-mot commitOnce}
Quán đếm số tô bán được trong ngày. Đến giữa buổi, cái tên `so_to_da_ban` đang
giữ `12`. Có thêm một khách gọi phở, và chị Hạnh chạy đúng dòng này:

```python
so_to_da_ban = 12
so_to_da_ban = so_to_da_ban + 1
```

**Trước khi xem đáp án**, bạn đoán sau dòng thứ hai, `so_to_da_ban` giữ con số nào?

:::opt{correct}
13
:::

:::opt
Vẫn là 12
::why
Gần đúng ở chỗ bạn nhìn ra đúng chỗ khó nhất của dòng này: cái tên `so_to_da_ban`
xuất hiện ở **cả hai** bên dấu `=`. Đó là chi tiết đáng dừng lại thật.

Chỗ lệch là thứ tự. Máy làm vế phải trước, và lúc nó làm vế phải thì tấm thẻ vẫn
còn nguyên ở chỗ cũ: nó đi tìm `so_to_da_ban`, thấy `12`, tính `12 + 1`, ra `13`.
Đến lúc đó nó mới chuyển thẻ sang `13`. Hai bên dấu `=` không xảy ra cùng lúc.
::
:::

:::opt
Máy báo lỗi, vì không con số nào bằng chính nó cộng một
::why
Gần đúng — và xét theo sách toán thì bạn hoàn toàn có lý: `x = x + 1` là một
phương trình vô nghiệm, không con số nào thoả được nó.

Chỗ lệch nằm ở chỗ dấu `=` trong Python không hỏi "hai bên có bằng nhau không".
Nó ra lệnh: *tính vế phải cho xong, rồi buộc cái tên bên trái vào kết quả*. Đọc
như mệnh lệnh thì dòng này bình thường tới mức cả bếp làm suốt ngày — **lấy số
cũ, cộng thêm một, ghi đè lên chỗ ghi số tô.**
::
:::

:::opt
1, vì gán lại làm mất con số cũ trước khi kịp cộng
::why
Gần đúng ở chỗ bạn nhớ đúng điều bài này vừa nói: gán lại thì giá trị cũ mất
đường về. Điều đó có thật.

Chỗ lệch là **lúc nào** nó mất. Giá trị cũ chỉ mất đường về sau khi tấm thẻ đã
chuyển đi — mà thẻ chỉ chuyển sau khi vế phải đã tính xong. Vế phải kịp đọc `12`
trước khi có gì kịp mất.
::
:::
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
