---
id: toan.cam-nhan-so.vi-sao-nhan-truoc-cong
title: Vì sao nhân trước cộng
summary: Phép nhân gói một lượng thành một khối, mà khối thì phải gói xong mới đem gộp được — nên luật "nhân trước cộng" ghi lại đúng cấu trúc ấy.
locale: vi
track: toan
module: cam-nhan-so
order: 42
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.order-of-operations]
requires: [math.unit-rate, core.arithmetic, core.variable, core.print-variable, core.number-literal]
concepts: [math.thu-tu-phep-toan, math.ky-hieu]
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
Cùng một dòng, hai kết quả. Mà ngoài khoảnh chỉ có đúng một số hạt.
::::

::::explain{#di-dem-truoc-da}
Bài trước dừng ở một chỗ không hoà được: Byte viết `3 + 4 × 5`, An đọc ra 35,
Byte ra 23.

Trước khi cãi nhau về cách đọc, ra khoảnh đất mà đếm đã. Toán ở track này luôn
kiểm được bằng tay, và đây là một trong những chỗ kiểm dễ nhất.

Ngoài khoảnh có 4 luống nhỏ, mỗi luống 5 hạt. Đếm từng luống: 5, 10, 15, 20. Cả
bốn luống là 20 hạt. Trong túi Byte còn 3 hạt lẻ chưa gieo. Đổ chỗ ấy ra đếm
tiếp: 21, 22, 23.

**23 hạt.** Không có cách đếm nào ra 35, vì trong khoảnh không có 7 luống.

Nhưng đừng vội nói An sai. Con số 35 của An cũng không phải bịa: gộp 3 với 4
được 7, rồi 7 luống mỗi luống 5 hạt thì đúng 35 hạt. Đó là một cái vườn hoàn
toàn có thật — chỉ không phải cái vườn mà dòng chữ kia nói tới.

Nên câu hỏi thật không phải "ai tính sai". Cả hai đều tính đúng phép tính mình
nghĩ. Câu hỏi là: **dòng `3 + 4 × 5` mô tả cái vườn nào**, và vì sao lại là cái
đó chứ không phải cái kia.
::::

::::explain{#nhan-la-mot-cai-goi}
Nhìn lại hai con số đứng hai bên dấu nhân.

Bài 19 đã nói rõ một điều mà lúc ấy chưa thấy hết tầm quan trọng: trong phép
nhân, hai con số **đứng hai vai khác nhau**. Ở đây `4` nói *lấy mấy lô*, còn `5`
nói *mỗi lô to bao nhiêu*. Chúng không phải hai lượng ngang hàng nằm cạnh nhau
— chúng dính vào nhau thành **một mô tả duy nhất**: "4 luống, mỗi luống 5 hạt".

Đó là một cái **khối**. Trước khi biết khối ấy chứa bao nhiêu hạt, bạn chưa có
gì trong tay để đem đi đâu cả.

Còn dấu cộng thì làm việc khác hẳn. Bài 10 nói: cộng là **gộp hai đống** thành
một đống. Muốn gộp thì phải có sẵn hai đống.

Ghép hai câu ấy lại là ra câu trả lời: cái khối phải **gói xong** thì mới thành
một đống để đem gộp. Gói trước, gộp sau. Đó không phải thứ tự bấm máy, đó là
thứ tự của chính công việc ngoài vườn.

Ba bức tranh cho cùng một chuyện.

Sơ đồ dải — 3 hạt lẻ đứng riêng, bốn khối luống đứng thành một mảng:

```text
hạt lẻ   [•••]
luống    [•••••][•••••][•••••][•••••]
tổng      3  +          20           =  23
```

Mô hình vùng — bài 20 và 21 dựng sẵn hình này rồi. Cái khối là một mảng chữ
nhật khít khao, còn 3 hạt lẻ nằm ngoài mảng:

```text
      5 hạt
    • • • • •
    • • • • •   ← 4 luống, mảng 4 × 5 = 20 hạt
    • • • • •
    • • • • •

    • • •       ← 3 hạt lẻ, không thuộc mảng nào
```

Thanh số — bài 13 nói cộng là bước sang phải, bài 22 nói nhân là kéo giãn. Đứng
ở vạch 3, bước sang phải bốn nhịp, mỗi nhịp dài 5:

```text
 |--|----|----|----|----|
 0  3    8    13   18   23
  lẻ  +5   +5   +5   +5
```

Cả ba đều dừng ở 23, và cả ba đều cho thấy cùng một điều: cái mảng 4 × 5 là một
vật nguyên khối. Nếu tính từ trái sang phải, gộp 3 với 4 trước, thì con số 4
bị lôi khỏi vai "mấy luống" của nó — và cái khối tan mất trước khi kịp thành
hình.
::::

::::predict{#doan-mot-dong commitOnce}
Byte hỏi thẳng cái máy đúng dòng đang tranh cãi.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(3 + 4 * 5)
```

:::opt{correct}
23
:::

:::opt
35
::why
Gần đúng ở chỗ bạn đọc từ trái sang phải, tới đâu tính tới đó. Cách đọc ấy đúng
với chữ viết, và đúng cả trong toán khi cả dòng chỉ toàn dấu cộng: bài 10 đã
cho thấy `2 + 3 + 4` gộp đống nào trước cũng ra một kết quả, nên ở đó thứ tự
không đáng bàn.

Chỗ lệch là ở dòng này có một dấu nhân xen vào, và hai con số hai bên dấu nhân
đã dính vào nhau thành một khối "4 luống, mỗi luống 5 hạt". Kéo số 4 ra khỏi
khối ấy để gộp với 3 là làm tan cái khối. Con số 35 bạn ra vẫn là số hạt của
một cái vườn có thật — 7 luống, mỗi luống 5 hạt — chỉ là dòng chữ này không nói
về cái vườn đó.
::
:::

:::opt
20
::why
Gần đúng ở chỗ bạn làm xong đúng phần cốt lõi của bài: gói `4 * 5` thành một
khối 20 trước, không để nó bị xé ra. Người nắm được chuyện đó là người đã hiểu
vì sao nhân đi trước.

Chỗ lệch nằm ở bước sau cùng. Gói xong thì vẫn còn một việc nữa: cái khối 20
hạt mới là một đống, mà ngoài nó còn đống thứ hai — 3 hạt lẻ trong túi. Dấu
cộng trong dòng đang chờ đúng việc gộp hai đống ấy lại. Đếm tay ngoài khoảnh
cũng có bước ấy: đếm hết luống rồi mới đổ túi ra đếm tiếp.
::
:::

:::opt
Máy báo lỗi vì một dòng đọc được theo hai cách
::why
Gần đúng ở chỗ bạn nhớ một luật đắt giá của Realm 0 bài 3: máy không tự bổ sung
ý bạn quên nói, và gặp chỗ không có câu trả lời đúng duy nhất thì nó dừng lại
hỏi — đó chính là cách `TypeError` ra đời.

Chỗ lệch là dòng này không mập mờ với máy. Người đọc thấy hai cách vì người đọc
chưa có luật; máy thì có sẵn một luật đọc và áp nó không do dự. Điều đáng hỏi
không phải là "sao máy không thắc mắc", mà là "luật ấy có lý do gì" — và bài
này đang trả lời đúng câu đó.
::
:::
::::

::::explain{#quy-uoc-nay-duoc-chon}
Đến đây phải nói một câu cho sòng phẳng, vì track này bán "vì sao đúng" chứ
không bán "học thuộc đi".

Luật "nhân làm trước cộng" **là một quy ước** — người ta chọn nó. Không có định
lý nào bắt buộc nó phải như thế, và về nguyên tắc loài người đã có thể chọn
ngược lại.

Nhưng chọn thế không phải rút thăm. Cấu trúc "có mấy khối, cộng thêm mấy cái
lẻ" gặp nhiều hơn hẳn mọi cấu trúc khác — trong sổ chợ, trong vườn, trong tiền
lương, ở đâu cũng vậy. Chọn cho phép nhân đi trước nghĩa là cấu trúc hay gặp
nhất viết ra được bằng dòng ngắn nhất, không phải đeo thêm dấu gì. Nếu chọn
ngược lại thì mỗi câu "mấy luống mỗi luống mấy hạt, cộng thêm ít lẻ" đều phải
kèm một dấu báo hiệu — mà loại dấu ấy là chuyện của bài sau.

Và luật này còn trả một món nợ cũ. Bài 21 cắt mảng 7 × 13 thành hai miếng rồi
viết:

```text
7 × 13  =  7 × 10  +  7 × 3
```

Vế phải của dòng ấy chỉ đúng nếu **cả hai phép nhân xong trước** rồi mới cộng.
Lúc ấy bạn đã dùng luật này rồi, chỉ chưa ai nói tên nó ra. Giờ thì nói rồi, và
lý do cũng đúng lý do cũ: `7 × 10` với `7 × 3` là hai miếng đã cắt của cùng một
mảng — hai cái khối — nên phải gói xong mới gộp lại được.

Hai chú thích ngắn, cả hai đều là đồ bạn đã có:

- **Phép chia đứng cùng hạng với phép nhân.** Bài 27 cho thấy chia cũng là một
  cách gói: "mấy đoạn 3 mét lọt vào 12 mét" cũng là một khối, không phải hai
  lượng ngang hàng.
- **Phép trừ đứng cùng hạng với phép cộng.** Bài 18 đã cho thấy trừ chính là
  cộng với số đối, nên nó không thể có hạng riêng.
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Hai buổi, hai dòng. Mỗi dòng đúng một khối và một nhúm hạt lẻ.
::::

::::code{#viet-lai-hai-buoi}
Byte ghi sổ hai buổi, và mỗi buổi đã đi đếm tay để biết trước con số thật.

- **Sáng**: trong túi còn **3 hạt lẻ**, ngoài khoảnh có **4 luống**, mỗi luống
  **5 hạt**. Đếm tay: **23 hạt**.
- **Chiều**: ngoài khoảnh có **6 luống**, mỗi luống **7 hạt**, và **2 hạt** rơi
  ngoài lối đi. Đếm tay: **44 hạt**.

Viết mỗi buổi thành **một dòng duy nhất**, theo đúng thứ tự người ta kể chuyện.
Không dùng thêm dấu nào ngoài dấu cộng và dấu nhân.

Hai buổi được chọn để **cái khối nằm ở hai phía khác nhau**: sáng thì hạt lẻ kể
trước, chiều thì khối kể trước. Một dòng viết đúng vì ăn may vị trí sẽ hỏng ở
buổi còn lại, và một con số gõ cứng thì hỏng ngay ở khối bị đòi kiểm bên dưới.

```python title=starter
sang = ___

chieu = ___

print(sang)
print(chieu)
```

```python title=solution
sang = 3 + 4 * 5

chieu = 6 * 7 + 2

print(sang)
print(chieu)
```

```python title=test
# Hai buổi cho hai con số khác nhau, nên không có đáp án nào điền chung được.
assert sang == 23, "3 hạt lẻ, cộng 4 luống mỗi luống 5 hạt — tay đếm ra 23"
assert chieu == 44, "6 luống mỗi luống 7 hạt, cộng 2 hạt rơi — tay đếm ra 44"
# Bỏ nhúm hạt lẻ ra thì phần còn lại phải đúng bằng cái KHỐI. Đây mới là chỗ
# phân biệt được người gói khối trước với người tính từ trái sang phải: đọc
# kiểu trái-sang-phải cho buổi sáng ra 35, và 35 trừ 3 không phải 20.
assert sang - 3 == 20, "gỡ 3 hạt lẻ ra, phần còn lại phải là khối 4 luống × 5 hạt = 20"
assert chieu - 2 == 42, "gỡ 2 hạt rơi ra, phần còn lại phải là khối 6 luống × 7 hạt = 42"
```

:::hints
- kind: attention
  body: Mỗi buổi kể đúng hai mảnh: một nhúm hạt lẻ, và một khối "mấy luống, mỗi luống mấy hạt". Chỗ trống cần cả hai mảnh nằm trên cùng một dòng.
- kind: strategy
  body: Cái khối luôn là hai con số nhân với nhau — số luống nhân số hạt mỗi luống. Nhúm lẻ đứng riêng, nối vào bằng dấu cộng. Cứ viết đúng thứ tự người ta kể; dấu nhân vẫn được tính trước dù nó đứng đầu dòng hay cuối dòng.
- kind: one-line
  body: "Viết `3 + 4 * 5` cho buổi sáng và `6 * 7 + 2` cho buổi chiều."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi dòng phải có đủ một phép nhân cho cái khối và một phép cộng cho nhúm hạt lẻ — gõ thẳng con số đã đếm được thì không viết ra cấu trúc nào cả
  requireAst:
  - kind: uses-operator, target: *, min: 2
  - kind: uses-operator, target: +, min: 2
- tier: output
  match: regex
  expect: ^23\n44\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Gói khối xong rồi mới gộp. Hai buổi đều khớp với tay đếm của mình.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy `3 + 4 × 5` không ghi lại một dãy việc phải làm, mà ghi lại một
**hình dạng**: có một khối, và có mấy hạt lẻ. Bài 41 để lại một quan sát cùng
hướng — đúng một dòng `90 : 6` mang được ba câu hỏi khác nhau, mỗi câu cho câu
trả lời một đơn vị khác. Ký hiệu toán không chép lại các bước bấm máy; nó chép
lại hình dạng của câu chuyện.

Nhưng quay lại ý của An mà xem, lần này nghiêm túc. "Gộp 3 với 4 trước, rồi mới
nhân 5" — ý ấy không hề vô nghĩa. Nó tả một buổi làm vườn có thật: Byte đang có
3 luống, đào thêm 4 luống nữa, rồi gieo 5 hạt vào mỗi luống trong cả bảy luống.
Ra vườn đếm tay được 35 hạt, không sai một hạt nào.

Hai câu chuyện đều thật, đều đếm được. Nhưng chỉ một trong hai viết ra được
bằng `3 + 4 × 5`. Câu chuyện còn lại chưa có dòng nào để viết — luật đọc mặc
định đã chiếm mất dòng đó rồi.

Vậy khi hình dạng trong đầu bạn không trùng với hình dạng mà luật đọc mặc định
giả sử, bạn nói nó ra bằng gì? Bài sau trả lời — và món đồ ấy bạn đã cầm trong
tay từ lâu mà chưa biết tên nó.
::::

::::checkpoint{mastery=0.8}
::::
