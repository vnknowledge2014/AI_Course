---
id: toan.cam-nhan-so.thuoc-be-nho-duoc-hay-khong
title: Thước bẻ được và thước không bẻ được
summary: Nửa sải dây vẫn là dây, còn nửa hạt thì không còn là hạt — chính chỗ đó tách số đếm ra khỏi số đo.
locale: vi
track: toan
module: cam-nhan-so
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.don-vi-roi-va-lien]
requires: [math.don-vi, core.output, core.variable, core.print-variable, core.arithmetic, core.division, core.float, core.boolean, ctrl.comparison]
concepts: [math.don-vi, math.so-dem, math.so-do]
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
Nửa sải dây thì mình vẫn đo được. Nửa hạt thì mình chịu.
::::

::::explain{#nua-hat-la-gi}
Bài trước kết bằng một thách thức: nếu đếm hạt cũng là đo — với cái thước là
"một hạt" — thì hai chuyện là một. Muốn biết chúng có thật là một không, cứ
**bẻ đôi cái thước** ra rồi nhìn.

Bẻ đôi một sải dây:

```text
một sải:   [===============================]
bẻ đôi:    [==============][===============]
```

Hai nửa ấy vẫn là dây. Vẫn căng ra được, vẫn đặt dọc luống được, vẫn đếm được
"nửa sải này lọt vào luống mấy lần". Cái thước không mất nghĩa khi bị bẻ — nó
chỉ thành một cái thước **nhỏ hơn**.

Bẻ đôi một hạt:

```text
một hạt:   (●)
bẻ đôi:    (◗  ◖)
```

Hai mảnh ấy không còn là hạt. Gieo xuống không mọc, đếm vào đống cũng không ai
tính. "Nửa hạt" là một câu chữ đọc lên nghe được, nhưng ngoài vườn thì không
chỉ tay vào được vật nào.

Chợ búa đầy hai loại này đứng cạnh nhau. Trên cùng một cái sạp:

- **Quả trứng, cái bát, con cá** — bổ đôi thì hết là quả trứng, cái bát, con
  cá. Đơn vị **rời**.
- **Lon gạo, mét vải, lít nước mắm** — đong nửa lon vẫn là gạo, cắt nửa mét
  vẫn là vải. Đơn vị **bẻ nhỏ được**.

Vậy chỗ khác nhau thật giữa đếm và đo không nằm ở con số, cũng không nằm ở
động tác. Nó nằm ở **cái đơn vị**: bẻ nó ra thì còn nghĩa, hay mất nghĩa.

Và chỗ khác nhau ấy kéo theo một hệ quả bạn dùng được suốt cả track. Nếu cái
thước bẻ được, thì giữa hai số đo **luôn còn chỗ** — bởi vì bao giờ cũng bẻ
thêm được một lần nữa.
::::

::::example{#be-doi-mai}
Byte đo luống rau: dài hơn 3 sải, chưa tới 4 sải. Hỏi máy xem giữa 3 và 4 có
số đo nào không, rồi hỏi tiếp giữa 3 với số vừa tìm được:

```python title=readonly
giua = (3 + 4) / 2
print(giua)

giua_nua = (3 + giua) / 2
print(giua_nua)

print(3 < giua_nua)
print(giua_nua < giua)
```

Máy in ra:

```text
3.5
3.25
True
True
```

Một chuyện chấm câu trước khi đi tiếp: trên giấy người Việt viết `3,5`, còn
Python viết `3.5` bằng dấu chấm. Cùng một con số, hai cách chấm câu — bài này
viết bằng dấu phẩy ở phần văn xuôi và bằng dấu chấm trong code, và hai chỗ đó
nói cùng một điều.

Lần bẻ thứ nhất lòi ra `3.5`. Lần bẻ thứ hai lòi ra `3.25`, và hai dòng `True`
xác nhận nó nằm hẳn giữa `3` với `3.5` chứ không trùng đầu nào.

Bẻ tiếp thì được `3.125`, rồi `3.0625`. Không có lần nào hết chỗ, vì mỗi lần
bẻ lại sinh ra một cái thước nhỏ hơn để bẻ tiếp.

Bây giờ làm y hệt thế với hạt. Giữa **3 hạt** và **4 hạt**, tìm một đống hạt
nằm ở giữa. Tìm mãi cũng không ra — vì `3,5 hạt` không phải một đống hạt nào
cả. Với đơn vị rời, giữa hai số kề nhau là **hết**, không còn gì.
::::

::::predict{#doan-lan-be-thu-hai commitOnce}
Byte chạy lại đúng hai lần bẻ ấy, nhưng lần này chỉ in ra kết quả cuối.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
giua = (3 + 4) / 2
giua_nua = (3 + giua) / 2
print(giua_nua)
```

:::opt{correct}
3.25
:::

:::opt
3.5
::why
Gần đúng ở chỗ bạn tìm ra đúng cái điểm giữa đầu tiên: 3 và 4 gặp nhau ở 3,5,
và dòng thứ nhất đúng là cất con số ấy vào tên `giua`.

Chỗ lệch nằm ở dòng thứ hai. Nó không lấy điểm giữa của 3 và 4 một lần nữa —
nó lấy điểm giữa của 3 và **`giua`**, tức là của 3 và 3,5. Cái đoạn đang bị bẻ
đã ngắn đi một nửa so với lần trước.
::
:::

:::opt
3.75
::why
Gần đúng ở chỗ bạn hiểu chính xác việc đang làm: bẻ đôi cái đoạn vừa có, rồi
lấy điểm giữa của một nửa. Đó đúng là ý của cả bài này.

Chỗ lệch là bẻ nhầm nửa. `3.75` là điểm giữa của **3,5 và 4** — nửa bên phải.
Còn dòng code viết `(3 + giua)`, tức là nó đang cầm nửa bên **trái**: từ 3 tới
3,5.
::
:::

:::opt
3.0
::why
Gần đúng ở chỗ bạn đang giữ một quy tắc thật sự đúng: khi đếm bằng đơn vị
**rời**, bẻ đôi mãi rồi cũng phải rơi về một trong hai đầu, vì giữa 3 hạt và 4
hạt không có đống hạt nào đứng được. Với hạt, với quả trứng, với con cá, bạn
suy luận thế là trúng.

Chỗ lệch là phạm vi. Dòng này đang đo bằng **sải dây** — cái thước bẻ được.
Bẻ đôi sải dây thì vẫn còn dây, nên `3.5` rồi `3.25` đều là số đo có thật, và
máy không kéo chúng về đầu nào cả.
::
:::
::::

::::explain{#hai-loai-thuoc}
Gọi tên hai loại thước ấy để lần sau nói cho nhanh:

- **Đơn vị rời** — bẻ ra là mất nghĩa: hạt, quả trứng, cái ghế, người. Đếm
  bằng nó thì con số **nhảy từng nấc**, và giữa hai nấc kề nhau không có gì.
- **Đơn vị bẻ nhỏ được** — bẻ ra vẫn còn nghĩa: sải dây, mét vải, lon gạo,
  phút. Đo bằng nó thì giữa hai số **bao giờ cũng còn chỗ**.

Một câu để mang theo: **cái quyết định không phải con số, mà là đơn vị đứng
sau con số.** `3` với `4` viết ra thì giống hệt nhau ở cả hai loại; chỉ khi
bạn hỏi "3 **cái gì**" — câu hỏi của bài 1 — thì mới biết giữa chúng còn chỗ
hay đã hết.

Và đây cũng là chỗ trả lời cho thách thức của bài trước: đếm **không** phải là
đo, dù hai việc trông giống nhau tới mức lẫn được. Chúng khác nhau đúng ở chỗ
cái thước có bẻ được hay không.
::::

::::code{#tim-cho-o-giua}
Luống rau nhà Byte đo được 3 sải. Luống nhà An đo được 4 sải. Thước là **sải
dây** — thước bẻ được.

Điền ba chỗ trống. Hai chỗ đầu đi với **sải dây**: một số đo nằm hẳn giữa hai
luống ấy, rồi bẻ tiếp một lần nữa để lấy một số đo nằm hẳn giữa `3` và cái số
bạn vừa viết.

Hai chỗ ấy đòi **hai con số khác nhau**: một con số gõ cứng vào cả hai thì
không thể vừa nằm giữa 3 với 4, vừa nằm giữa 3 với chính nó.

Chỗ thứ ba đổi thước: vẫn hai con số `3` và `4` ấy, vẫn câu hỏi "giữa chúng còn
chỗ không", nhưng lần này đơn vị là **hạt**.

```python title=starter
# 1) Một số đo nằm HẲN giữa 3 và 4 — không bằng đầu nào.
giua = ___

# 2) Bẻ tiếp: một số đo nằm HẲN giữa 3 và cái số bạn vừa viết ở trên.
giua_nua = ___

# 3) Giữa 3 HẠT và 4 HẠT có đống hạt nào nằm ở giữa không? Viết True hay False.
co_dong_hat_o_giua = ___

print(giua)
print(giua_nua)
print(co_dong_hat_o_giua)
```

```python title=solution
# 1) Một số đo nằm HẲN giữa 3 và 4 — không bằng đầu nào.
giua = 3.5

# 2) Bẻ tiếp: một số đo nằm HẲN giữa 3 và cái số bạn vừa viết ở trên.
giua_nua = 3.25

# 3) Giữa 3 HẠT và 4 HẠT có đống hạt nào nằm ở giữa không? Viết True hay False.
co_dong_hat_o_giua = False

print(giua)
print(giua_nua)
print(co_dong_hat_o_giua)
```

```python title=test
# Ba câu đầu chấm phía sải dây, và chúng chấm được cả những đáp án khác lời
# giải mẫu: ai viết 3.4 rồi 3.1 cũng qua, miễn là hai con số thật sự nằm lồng
# trong nhau.
assert 3 < giua < 4, "số đo thứ nhất phải nằm HẲN giữa hai luống — không bằng 3, cũng không bằng 4"
assert 3 < giua_nua < giua, "số đo thứ hai phải nằm HẲN giữa 3 và con số bạn vừa viết ở trên"
assert 3 < (3 + giua_nua) / 2 < giua_nua, "bẻ thêm lần nữa thì VẪN còn chỗ — thước bẻ được thì không bao giờ hết chỗ để bẻ"
# Hai câu cuối chấm phía hạt — nửa còn lại của cặp khái niệm mà bài này dạy.
assert co_dong_hat_o_giua == False, "hạt là đơn vị RỜI — bẻ đôi hạt thì hết là hạt, nên giữa 3 hạt và 4 hạt là hết, không còn gì"
assert co_dong_hat_o_giua != (3 < giua_nua < giua), "cùng hai con số 3 và 4, cùng câu hỏi 'giữa chúng còn chỗ không' — sải dây trả lời có, hạt trả lời không. Cái quyết định là đơn vị, không phải con số."
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm giữa hai con số 3 và 4; chỗ trống thứ hai nằm giữa 3 và cái số bạn vừa viết. Hai đoạn ấy dài ngắn khác nhau, nên hai câu trả lời không chép được cho nhau. Còn chỗ trống thứ ba thì không hỏi con số nào cả — nó hỏi có hay không, và đơn vị lúc này là hạt chứ không phải sải dây.
- kind: strategy
  body: Chỗ dễ tìm nhất trong một đoạn là điểm chính giữa. Ví dụ trên đầu bài đã in ra sẵn hai con số ấy khi máy tự bẻ đôi hai lần — nhìn lại hai dòng đầu của phần máy in. Còn chỗ thứ ba thì nhớ lại hình bẻ đôi một hạt ở đầu bài: hai mảnh ấy không còn là hạt nữa.
- kind: one-line
  body: "Viết `3.5` vào chỗ trống thứ nhất, `3.25` vào chỗ trống thứ hai, và `False` vào chỗ trống thứ ba."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bẻ hai lần vẫn còn chỗ. Bẻ mười lần cũng thế — mình thử rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

"Giữa 3 và 4 vẫn còn chỗ" — nghe thì gật đầu, và máy vừa in ra hai con số nằm
trong cái chỗ ấy. Nhưng thử làm việc này xem: **chỉ tay vào** chỗ ấy.

Khó, đúng không. Vì trong đầu bạn, `3` tới giờ vẫn đang là một **đống ba hạt**,
mà giữa đống ba hạt với đống bốn hạt thì không có khoảng nào để chỉ vào cả —
hai cái đống là hai cái đống, chúng không nằm cạnh nhau ở đâu hết.

Vậy phải vẽ ra một bức tranh khác, bức tranh mà trong đó `3` không còn là một
đống. Bức tranh ấy trông thế nào, và cần bao nhiêu thứ để dựng được nó?

Bài sau trả lời, và nó chỉ cần đúng hai thứ.
::::

::::checkpoint{mastery=0.8}
::::
