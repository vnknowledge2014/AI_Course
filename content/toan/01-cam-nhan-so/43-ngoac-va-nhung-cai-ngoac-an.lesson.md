---
id: toan.cam-nhan-so.ngoac-va-nhung-cai-ngoac-an
title: Dấu ngoặc và những cái ngoặc ẩn
summary: Dấu ngoặc là chỗ bạn tự vẽ ra cái khối, khi khối bạn muốn không trùng khối mà luật đọc mặc định giả sử — và gạch phân số với số mũ đã làm việc ấy từ lâu.
locale: vi
track: toan
module: cam-nhan-so
order: 43
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.parentheses]
requires: [math.order-of-operations, math.exponent, math.fraction, math.division-partitive, core.arithmetic, core.division, core.float, core.variable, core.print-variable, core.number-literal]
concepts: [math.ky-hieu, math.thu-tu-phep-toan, math.cau-truc]
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
Vườn của An có thật, mình ra đếm rồi. Chỉ là mình chưa có chỗ nào để viết nó ra.
::::

::::explain{#cai-vuon-chua-viet-duoc}
Bài trước khép lại với một món nợ. Dòng `3 + 4 × 5` kể chuyện vườn của Byte: ba
hạt lẻ trong túi, và một khối bốn luống mỗi luống năm hạt — 23 hạt.

Nhưng ý của An cũng tả một buổi làm vườn có thật: Byte đang có **3 luống**, đào
thêm **4 luống** nữa, rồi gieo **5 hạt** vào mỗi luống trong cả bảy luống. Ra
đếm tay được 35 hạt, không sai một hạt nào.

Hai cái vườn. Một dòng chữ. Luật đọc mặc định đã trao dòng ấy cho vườn thứ nhất,
và vườn thứ hai thì không còn dòng nào để viết.

Chỗ khác nhau giữa hai vườn nằm ở **cái gì là một khối**.

```text
Vườn của Byte — 3 + 4 × 5

  hạt lẻ   [•••]
  khối     [•••••][•••••][•••••][•••••]      ← 4 luống, mỗi luống 5 hạt
           3  +              20              =  23


Vườn của An — cái khối bao trọn cả bảy luống

  luống cũ  [•••••][•••••][•••••]
  luống mới [•••••][•••••][•••••][•••••]
  khối      (3 + 4) luống, mỗi luống 5 hạt   =  35
```

Ở vườn của Byte, con số 4 nằm trong khối và con số 3 đứng ngoài. Ở vườn của An,
cả 3 lẫn 4 đều nằm trong khối — chúng gộp lại thành *số luống*, rồi mới đem
nhân.

Nên thứ còn thiếu không phải một phép toán mới. Thứ còn thiếu là một cách **rào
lấy một khúc của dòng chữ** rồi bảo: khúc này là một khối, gói nó xong trước đã.

Cái rào ấy là `( )`.

```text
(3 + 4) × 5
```

Đọc là: gộp 3 với 4 thành một khối, rồi lấy khối ấy nhân 5. Ra 35 — đúng cái
vườn của An.
::::

::::explain{#ngoac-khong-phai-de-tinh-truoc}
Có một cách hiểu sai rất phổ biến về dấu ngoặc, và bỏ nó đi sớm thì đỡ vướng cả
đời: *"ngoặc là chỗ mình bảo máy tính trước cho chắc"*.

Không phải. Ngoặc không nói về **thứ tự bấm máy**, nó nói về **hình dạng của câu
chuyện** — y hệt điều bài trước vừa dựng. Một cặp ngoặc là bạn cầm bút khoanh
lấy một khối, thay vì để luật đọc mặc định khoanh hộ.

Từ đó ra hai hệ quả, và cả hai đều dùng được ngay:

- **Ngoặc trùng với luật mặc định thì thừa.** `3 + (4 × 5)` cho ra đúng cái mà
  `3 + 4 × 5` cho ra: 23. Cặp ngoặc ấy không đổi gì hết, vì nó khoanh đúng cái
  khối mà luật đã khoanh. Thừa nhưng không sai — và nhiều người vẫn viết ra để
  người đọc khỏi phải nhớ luật.
- **Ngoặc trái với luật mặc định thì ngoặc thắng.** `(3 + 4) × 5` ra 35. Đây mới
  là lúc dấu ngoặc kiếm sống: khi hình dạng trong đầu bạn không trùng hình dạng
  mà luật giả sử.

Ngoặc lồng trong ngoặc thì gói từ **trong ra ngoài** — cái khối nhỏ phải thành
hình trước thì cái khối lớn mới có gì mà ôm. Ví dụ `18 : (3 : 4)` khoanh riêng
`3 : 4` thành một lượng, rồi mới đem 18 đo bằng lượng ấy.
::::

::::explain{#nhung-cai-ngoac-an}
Bây giờ tới chỗ bất ngờ của bài: **bạn đã dùng dấu ngoặc từ lâu rồi, chỉ là nó
không mang hình dấu ngoặc.**

**Gạch phân số là một cái ngoặc.** Bài 31 viết phân số bằng một cái gạch, tử ở
trên, mẫu ở dưới. Cái gạch ấy làm đúng việc của một cặp ngoặc: nó rào lấy cả
cụm bên trên và cả cụm bên dưới.

```text
Trên giấy                Ép xuống một dòng
  12 + 8                   (12 + 8) : 4
  ──────
     4
```

Trên giấy không cần ngoặc, vì **chỗ đứng đã nói cấu trúc rồi** — cái gì nằm trên
gạch thì thuộc về tử, khỏi bàn. Ép tất cả xuống một dòng thì cái gạch biến mất,
chỗ đứng biến mất theo, và `12 + 8 : 4` quay về luật mặc định: chia làm trước,
nên chỉ mình số 8 bị chia. Cặp ngoặc là thứ dựng lại cái gạch đã mất.

**Số mũ cũng là một cái ngoặc.** Bài 24 viết số mũ nhỏ ở trên: `10⁵`. Cái "ở
trên" ấy là một cái rào — thứ gì viết trên đó thì thuộc về số mũ.

```text
Trên giấy                Ép xuống một dòng
  10²⁺³                    10 ** (2 + 3)
```

Bỏ ngoặc đi thì `10 ** 2 + 3` đọc thành "mười mũ hai, rồi cộng 3" — ra 103, chứ
không phải 100000.

Hai chuyện trên là **một** chuyện. Trên giấy, vị trí mang thông tin — đúng cái
ý mà bài 7 đã dựng khi nói chỗ đứng của chữ số quyết định giá trị của nó. Khi
mọi thứ bị dồn xuống một hàng ngang, vị trí không còn chỗ để nói, và `( )` là
thứ đứng ra nói thay.
::::

::::predict{#doan-bon-dong commitOnce}
Bốn dòng, từng cặp một khác nhau đúng một cặp ngoặc.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(3 + 4 * 5)
print((3 + 4) * 5)
print(10 ** 2 + 3)
print(10 ** (2 + 3))
```

:::opt{correct}
23, 35, 103 rồi 100000
:::

:::opt
23, 35, 100000 rồi 100000
::why
Gần đúng ở chỗ bạn mang sang một thói quen đọc rất chuẩn của giấy bút: trên
giấy, `10²⁺³` viết cả cụm `2 + 3` nhỏ ở trên, nên cả cụm ấy thuộc về số mũ.
Trong vở toán bạn đọc như thế là không sai một lần nào.

Ranh giới nằm ở chỗ *ép xuống một dòng*. Cái "nhỏ ở trên" là một cái rào có thật
trên giấy; viết thành `10 ** 2 + 3` thì cái rào ấy không đi theo, vì trên một
hàng ngang không có chỗ nào là "ở trên". Không rào thì luật mặc định lên tiếng:
`**` chỉ ôm lấy con số ngay bên phải nó, tức là 2 — rồi 3 mới được gộp vào sau.

Dòng 4 dựng lại cái rào bằng `( )`, nên nó mới là dòng ra 100000.
::
:::

:::opt
23, 23, 103 rồi 100000
::why
Gần đúng ở chỗ bạn nắm được một điều mà nhiều người không để ý: **rất nhiều cặp
ngoặc là thừa**. `3 + (4 × 5)` đúng bằng `3 + 4 × 5`, và trong đời bạn sẽ gặp
kiểu ngoặc thừa ấy nhiều hơn kiểu kia.

Ranh giới là: ngoặc thừa khi nó khoanh **đúng cái khối mà luật đã khoanh**. Ở
dòng 2 thì ngược lại — luật muốn khoanh `4 * 5`, còn cặp ngoặc khoanh `3 + 4`.
Hai cái khoanh ấy chỏi nhau, và lúc chỏi nhau thì ngoặc thắng, vì ngoặc là bạn
nói thẳng còn luật chỉ là chỗ dựa khi bạn không nói gì.

Dòng 2 ra 35: bảy luống, mỗi luống 5 hạt — đúng cái vườn của An.
::
:::

:::opt
23, 35, 23 rồi 50
::why
Gần đúng ở chỗ bạn nối được số mũ về đúng gốc của nó. Bài 24 dựng luỹ thừa lên
từ phép nhân lặp, nên nghĩ tới phép nhân khi thấy `**` là nghĩ đúng hướng.

Ranh giới nằm ở việc số mũ **đếm cái gì**. Bài 24 nói rõ: số mũ đếm số **thừa
số**, không phải một thừa số. `10 ** 2` không phải "10 nhân 2" mà là "hai con 10
nhân nhau" — tức 10 × 10, ra 100. Cũng vậy, `10 ** 5` là năm con 10 nhân nhau,
chứ không phải 50.

Đây đúng là chỗ mà cách viết gọn dễ gạt người: `10 ** 2` trông ngắn hơn hẳn
`10 × 10`, nên mắt hay đọc nhầm con số 2 thành một thừa số.
::
:::
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Hai dòng. Một dòng mình khoanh bằng tay, một dòng cái gạch phân số đã khoanh sẵn.
::::

::::code{#hai-dong-hai-cai-rao}
Hai việc trong vườn, mỗi việc một dòng. Cả hai đều cần một cặp ngoặc, nhưng vì
hai lý do khác nhau.

- **Vườn của An.** Byte đang có **3 luống**, đào thêm **4 luống** nữa, rồi gieo
  **5 hạt** vào mỗi luống trong cả bảy luống. Đếm tay: **35 hạt**.
- **Đống phân trộn.** Byte trộn **12 kg** phân chuồng với **8 kg** tro, rồi chia
  đều đống trộn ấy cho **4 luống**. Trên giấy Byte viết nó thành một phân số: cả
  đống trộn nằm trên gạch, số luống nằm dưới. Ép xuống một dòng thì cái gạch
  biến mất. Đếm tay: **5 kg mỗi luống**.

Hai chỗ trống là hai dòng ấy. Chỉ dùng dấu cộng, dấu nhân, dấu chia và dấu ngoặc
— đừng gõ thẳng con số đã đếm được.

Hai việc được chọn để hỏng theo hai kiểu khác nhau nếu thiếu ngoặc: bỏ ngoặc ở
dòng đầu thì nó tụt về đúng cái vườn của bài trước, còn bỏ ngoặc ở dòng sau thì
12 kg phân chuồng không đến được luống nào. Bài chấm bắt cả hai kiểu hỏng ấy.

```python title=starter
# Vườn của An: 3 luống cũ, đào thêm 4 luống, gieo 5 hạt vào MỖI luống.
so_hat_cua_an = ___

# Đống phân trộn: 12 kg phân chuồng với 8 kg tro, chia đều cho 4 luống.
phan_moi_luong = ___

print(so_hat_cua_an)
print(phan_moi_luong)
```

```python title=solution
# Vườn của An: 3 luống cũ, đào thêm 4 luống, gieo 5 hạt vào MỖI luống.
so_hat_cua_an = (3 + 4) * 5

# Đống phân trộn: 12 kg phân chuồng với 8 kg tro, chia đều cho 4 luống.
phan_moi_luong = (12 + 8) / 4

print(so_hat_cua_an)
print(phan_moi_luong)
```

```python title=test
# Hai việc cho hai con số khác nhau, nên không có đáp án nào điền chung được.
assert so_hat_cua_an == 35, "3 luống cũ gộp 4 luống mới là 7 luống, mỗi luống 5 hạt"
assert phan_moi_luong == 5.0, "đống trộn nặng 20 kg, chia đều cho 4 luống thì mỗi luống 5 kg"
# Hai câu `!=` chốt đúng hai kiểu hỏng khi cái rào bị bỏ quên.
assert so_hat_cua_an != 23, "thiếu ngoặc thì dòng ấy tụt về vườn của bài 42 — 3 hạt lẻ và một khối 4 × 5"
assert phan_moi_luong != 14.0, "thiếu ngoặc thì chỉ mình 8 kg tro bị chia, còn 12 kg phân chuồng không đến luống nào"
```

:::hints
- kind: attention
  body: Ở dòng đầu, con số 5 nói mỗi luống mấy hạt — nên nó phải nhân với TỔNG SỐ LUỐNG, chứ không phải với riêng số luống mới đào. Ở dòng sau, con số 4 chia CẢ ĐỐNG TRỘN, chứ không phải riêng đống tro.
- kind: strategy
  body: Cả hai dòng đều có một phép gộp phải xong trước, mà luật đọc mặc định lại cho nhân và chia đi trước. Khoanh cái phép gộp ấy lại bằng một cặp ngoặc, rồi mới viết phép nhân hay phép chia ra ngoài cặp ngoặc đó.
- kind: one-line
  body: "Viết `(3 + 4) * 5` cho dòng đầu và `(12 + 8) / 4` cho dòng sau."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi dòng phải viết ra đủ phép gộp và phép nhân (hoặc phép chia) đi kèm — gõ thẳng con số đã đếm được thì không có cấu trúc nào để đọc
  requireAst:
  # Khung khởi đầu chưa có dấu nào trong ba dấu này, nên bộ luật chặn được đáp
  # án chép cứng 35 với 5.0. Riêng cặp ngoặc thì AST không hỏi thẳng được —
  # đó là việc của hai câu `!=` trong khối test.
  - kind: uses-operator, target: +, min: 2
  - kind: uses-operator, target: *, min: 1
  - kind: uses-operator, target: /, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^35\n5\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
35 hạt và 5 kg. Giờ mình viết được cả cái vườn mà hôm qua chưa viết nổi.
::::

::::explain{#chot-lai}
Một câu mang theo:

> **Dấu ngoặc là chỗ bạn tự khoanh lấy cái khối.** Không khoanh thì luật đọc mặc
> định khoanh hộ, và nó khoanh theo hình dạng hay gặp nhất — không theo hình
> dạng của bạn.

Và một câu nữa, để nhìn lại cả quãng đường ký hiệu vừa đi:

> Trên giấy, **chỗ đứng** nói cấu trúc — chữ số đứng cột nào (bài 7), cụm nào
> nằm trên gạch phân số, cụm nào viết nhỏ ở trên làm số mũ. Ép xuống một hàng
> ngang thì chỗ đứng hết chỗ, và `( )` là thứ gánh lại phần việc ấy.

Từ đây Byte đọc được mọi biểu thức của track: đơn vị, cột, số âm, mảng, phân số,
thập phân, phần trăm, tỉ lệ, thứ tự phép toán, và cái rào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cả track này đi từ một câu hỏi rất nhỏ — *mấy cái gì* — rồi mỗi bài thêm đúng
một sự bất tiện và đúng một cách gỡ. Bây giờ thử cầm hết chỗ đồ nghề ấy lên một
lần.

Vườn của Byte có **6 luống**, mỗi luống gieo **15 hạt**. Đợt rét làm **20%** số
hạt gieo không nảy. Số cây sống được đánh ra một bờ dài **18 mét**, chia thành
các ô rộng **3/4 mét**.

**Mỗi ô được mấy cây?**

Một câu hỏi, một con số đi ra. Nhưng để đi tới con số ấy phải đi qua gần như mọi
bài đã học: đơn vị và cái thước, cột và số 0 giữ chỗ, mảng chữ nhật, phần trăm
*của cái gì*, phân số tương đương, thập phân, chia đo, quy về một đơn vị, thứ tự
phép toán — và cái rào bạn vừa cầm được hôm nay.

Viết ra được không? Bài sau đi hết một lượt, chặng nào cũng chốt bằng một dòng
cho máy làm trọng tài.
::::

::::checkpoint{mastery=0.8}
::::
