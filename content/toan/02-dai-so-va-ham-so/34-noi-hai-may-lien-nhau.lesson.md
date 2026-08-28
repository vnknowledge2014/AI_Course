---
id: toan.dai-so-va-ham-so.noi-hai-may-lien-nhau
title: Nối hai máy nối đuôi nhau
summary: Đầu ra của máy này cắm thẳng vào miệng máy kia, và cái ống nối ấy sinh ra một cái máy MỚI — dùng được mà không phải mở máy nào ra xem bên trong.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 34
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.function-composition]
requires: [math.function, math.function-notation, math.slope, math.substitution, math.expression, math.value-table, math.multiplication, math.additive-inverse, math.parentheses, math.order-of-operations, math.don-vi, core.function-def, core.function-call, core.function-parameter, core.function-argument, core.function-return, core.variable, core.assignment, core.print-variable, core.arithmetic, core.rhs-first]
concepts: [math.hop-ham, math.may-noi-tiep, math.dau-ra-khop-dau-vao]
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
Hai cái máy đứng cạnh nhau. Bắc một cái ống từ miệng ra của cái này sang miệng vào của cái kia thì sao?
::::

::::explain{#hai-viec-hai-may}
Bài trước để lại một câu hỏi rất thực tế: muốn biết **tiền lãi** thì phải tính
**tiền thu** trước rồi mới trừ tiền vốn. Hai việc, làm theo đúng thứ tự ấy.

Trong tủ máy của quán, hai việc ấy đã là hai cái máy riêng từ lâu:

- **Máy `thu`** — bỏ vào **số ổ**, nhả ra **tiền thu**. `thu(n) = 15000 × n`.
- **Máy `lai`** — bỏ vào **tiền thu**, nhả ra **tiền lãi**. `lai(t) = t − 30000`
  (ba mươi nghìn là tiền thuê chỗ mỗi ngày, đúng con số bài 27 đã chốt).

Byte đang phải chạy tay hai lượt: bấm máy thứ nhất, cầm con số nó nhả ra, đi
mấy bước sang máy thứ hai, bỏ con số ấy vào. Ngày nào cũng thế, mỗi ngày mấy
chục lần.

Nên Byte làm cái việc mà ai cũng sẽ nghĩ tới: **bắc một cái ống** từ miệng ra
của máy `thu` sang thẳng miệng vào của máy `lai`.

Chỗ đáng dừng lại một chút là: **vì sao bắc được?**

Không phải máy nào cũng nối được với máy nào. Máy `thu` nhả ra một số **tiền**.
Máy `lai` đòi được đưa vào một số **tiền**. Hai đầu ấy trả lời cùng một câu hỏi
*"mấy cái gì"* — cùng là đồng bạc — nên chúng khớp.

Thử một cặp không khớp cho thấy rõ: máy đổi tiền thu ra **số ly nước đá** cũng
là một cái máy đàng hoàng, nhưng đem cắm đầu ra của nó (số ly) vào miệng vào của
máy `lai` (đòi số tiền) thì con số chạy tiếp vẫn ra, mà nó chẳng còn nghĩa gì.
Cái ống nối được là vì **đầu ra của máy trước và đầu vào của máy sau đo cùng một
thứ**.
::::

::::example{#cai-ong-noi}
Byte bán 1 ổ. Con số đi qua cái ống:

```text
   n  ──►  [ thu ]  ──►  15 000  ──►  [ lai ]  ──►  −15 000
   │                     │                          │
   số ổ                  tiền thu                   tiền lãi
```

Con số `15 000` ở giữa **có thật**. Nó nằm trong ống, tồn tại đúng một khoảnh
khắc. Nhưng người hỏi "bán 1 ổ thì lãi bao nhiêu" không cần thấy nó — họ chỉ
cần con số bên phải cùng.

Đó chính là điều mới của bài này: khi bắc ống xong, **hai cái máy hoá thành
một**. Bỏ số ổ vào một đầu, tiền lãi rơi ra đầu kia. Cái máy mới ấy có bảng
riêng của nó:

```text
  số ổ n │     thu(n) │  lai(thu(n))
─────────┼────────────┼─────────────
       0 │          0 │       -30000
       1 │      15000 │       -15000
       2 │      30000 │            0
      20 │     300000 │       270000
      30 │     450000 │       420000
```

Cột giữa là thứ nằm trong ống. Che nó đi thì còn đúng hai cột — **một cột vào,
một cột ra** — tức là đúng hình dạng của một cái máy như bài 24 đã mô tả: mỗi
đầu vào cho đúng một đầu ra.

Viết cái máy mới ấy bằng ký hiệu của bài 25:

> **`lai(thu(n))`**

Và đọc nó theo đúng luật đã có từ bài 43 của mạch trước: **cặp ngoặc trong cùng
chạy trước**. `thu(n)` xong đã, ra một con số, rồi con số ấy mới bước vào `lai`.
Viết trên giấy thì `thu` nằm bên trong, mà chạy thì `thu` chạy trước — hình dạng
của cái ống nằm ngược với thứ tự đọc chữ.

Hỏi thẳng cái máy cả hai cách:

```python title=readonly
def thu(n):
    return 15000 * n

def lai(t):
    return t - 30000

# chạy tay hai lượt, cầm con số ở giữa
tien_thu = thu(1)
print(tien_thu)
print(lai(tien_thu))

# bắc ống: một lượt duy nhất, không ai cầm con số ở giữa
print(lai(thu(1)))
```

Máy in ra:

```text
90000
-10000
-10000
```

Hai dòng cuối trùng khít. Cái ống không làm đổi con số nào — nó chỉ **cất giùm**
con số ở giữa.
::::

::::predict{#doan-thu-tu commitOnce}
Byte đổi số ổ sang `4` và bỏ luôn cái tên ở giữa.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def thu(n):
    return 15000 * n

def lai(t):
    return t - 30000

print(lai(thu(4)))
```

:::opt{correct}
-40000
:::

:::opt
60000
::why
Gần đúng ở chỗ bạn chạy đúng cái máy phải chạy trước: `thu(4)` ra 60 000, và
con số ấy có thật, nó đúng là số tiền thu được khi bán bốn ổ. Quy tắc bạn đang
dùng — *trong ngoặc làm trước* — là quy tắc đúng, và nó chính là thứ bài này
vừa dựng.

Chỗ lệch nằm ở chỗ dừng. Sau khi `thu(4)` ra 60 000, dòng ấy chưa xong: cặp
ngoặc **ngoài** còn một cái máy nữa đang há miệng chờ. 60 000 là con số nằm
**trong ống**, chưa ra tới đầu bên kia. Muốn thấy nó thật thì phải cắt ống ra
làm hai lượt như ví dụ ở trên.
::
:::

:::opt
-1499940000
::why
Gần đúng ở chỗ bạn đọc từ trái sang phải, gặp máy nào chạy máy ấy — và với chữ
viết thì đó đúng là cách đọc. Nhìn dòng `lai(thu(4))`, chữ `lai` đứng trước
thật.

Chỗ lệch là **thứ tự đọc chữ không phải thứ tự chạy máy**. Dấu ngoặc mới quyết
định, và luật ấy đã có từ bài 43 mạch trước: trong cùng chạy trước. Chạy `lai`
trước nghĩa là bỏ số **4** — bốn cái ổ bánh mì — vào một cái máy chỉ nhận
**tiền**, ra −99 996, rồi nhân nó với 15 000. Con số khổng lồ ấy là dấu hiệu
cái ống đã bị bắc ngược đầu.
::
:::

:::opt
Máy báo lỗi, vì `lai` đang được đưa một cái máy chứ không phải một con số
::why
Gần đúng ở chỗ bạn để ý đúng thứ bài này lấy làm trọng tâm: máy `lai` chỉ nhận
được một **con số**, và nó phải là số tiền. Ai nhìn ra chỗ ấy là đang nhìn đúng
cái ống.

Chỗ lệch: `thu(4)` không phải một cái máy đang được trao tay. Nó là **lệnh gọi
máy** — máy chạy xong ngay tại đó và để lại một con số. Đây đúng là điều Realm 0
đã dựng khi nói vế phải làm xong trước: tới lúc `lai` được gọi thì trong tay nó
đã là 60 000, không còn cái máy nào cả.
::
:::
::::

::::explain{#mot-cai-may-that}
Đặt tên cho thứ vừa làm, để mang đi được:

> Bắc đầu ra của máy `f` vào đầu vào của máy `g` thì được một cái máy **mới**.
> Máy ấy viết là **`g(f(n))`**, và người ta gọi nó là **hợp** của hai máy.

Ba điều đi kèm cái tên, và cả ba đều đáng giữ:

1. **Nó là một cái máy thật, không phải một mẹo viết tắt.** Nó có bảng, có đồ
   thị, có độ dốc. Nhìn cột trái và cột phải của bảng trên: cứ thêm 10 ổ thì
   tiền lãi thêm đúng 150 000 — một lượng cố định. Theo bài 26, thế nghĩa là
   đồ thị của nó là một **đường thẳng**, dốc đúng bằng 15 000, y như máy `thu`.
   Máy `lai` chỉ hạ cả đường xuống 100 000 chứ không bẻ nó cong.
2. **Nối được là nhờ hai đầu khớp nhau.** Đầu ra của `f` phải là thứ mà `g`
   nhận được. Đây là điều kiện duy nhất, và nó nói về **đơn vị**, không nói về
   con số.
3. **Dùng được mà không phải mở ra.** Để viết `lai(thu(n))`, bạn không cần biết
   bên trong `thu` là phép nhân hay bên trong `lai` là phép trừ. Bạn chỉ cần
   biết mỗi máy **nhận gì, nhả ra gì**. Ngày nào Byte đổi giá một ổ từ 15 000
   lên 17 000, chỉ mình ruột máy `thu` đổi — cái ống, và cái máy mới, không
   phải viết lại chữ nào.

Điều thứ ba nghe nhỏ mà là chỗ cả bài này hướng tới. Nó là cách người ta dựng
những thứ lớn từ những mảnh nhỏ: đóng kín từng mảnh lại, rồi chỉ nối miệng
chúng với nhau.
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Ba buổi chợ, ba con số vào. Cùng một cái ống.
::::

::::code{#may-lai-theo-o}
Hai cái máy đã có sẵn trong khung. Việc của bạn là **bắc ống**, ba lần, cho ba
buổi chợ khác nhau.

Ba buổi được chọn để cho ra ba con số khác hẳn nhau — một buổi **lỗ**, một buổi
lãi vừa, một buổi lãi to. Gõ cứng một con số thì hai buổi kia sai ngay.

Và một luật của riêng bài này: **không mở máy nào ra**. Trong ba chỗ trống,
đừng viết lại `15000` hay `30000` — cứ gọi tên hai cái máy là đủ.

```python title=starter
def thu(n):
    return 15000 * n

def lai(t):
    return t - 30000

# Ba buổi chợ: 1 ổ, 20 ổ, 30 ổ.
# Mỗi lần cho SỐ Ổ đi qua cả hai máy, không cầm con số ở giữa.
lai_ban_1 = ___
lai_ban_20 = ___
lai_ban_30 = ___

print(lai_ban_1)
print(lai_ban_20)
print(lai_ban_30)
```

```python title=solution
def thu(n):
    return 15000 * n

def lai(t):
    return t - 30000

# Ba buổi chợ: 1 ổ, 20 ổ, 30 ổ.
# Mỗi lần cho SỐ Ổ đi qua cả hai máy, không cầm con số ở giữa.
lai_ban_1 = lai(thu(1))
lai_ban_20 = lai(thu(20))
lai_ban_30 = lai(thu(30))

print(lai_ban_1)
print(lai_ban_20)
print(lai_ban_30)
```

```python title=test
# Hai câu `!=` chốt đúng cái bẫy của bài — dừng ở giữa ống — và chúng đứng
# TRƯỚC vì chương trình dừng ngay tại câu vỡ đầu tiên; xếp sau câu `==` bao
# trùm chúng thì chúng không bao giờ chạy tới.
assert lai_ban_1 != 15000, "15 000 là tiền THU khi bán 1 ổ — con số ấy mới ra khỏi máy đầu, chưa đi qua máy trừ tiền thuê chỗ"
assert lai_ban_20 != 300000, "300 000 là tiền THU khi bán 20 ổ; tiền lãi còn phải bớt 30 000 tiền thuê chỗ"
# Ba buổi chợ khác nhau, nên một con số gõ cứng chỉ qua được nhiều nhất một câu.
assert lai_ban_1 == -15000, "bán 1 ổ thu 15 000, trả 30 000 tiền thuê chỗ, còn thiếu 15 000"
assert lai_ban_20 == 270000, "bán 20 ổ thu 300 000, trừ 30 000 tiền thuê chỗ"
assert lai_ban_30 == 420000, "bán 30 ổ thu 450 000, trừ 30 000 tiền thuê chỗ"
# Máy nối vẫn là một đường thẳng: mười ổ nữa thì lãi thêm đúng mười lần giá một ổ.
assert lai_ban_30 - lai_ban_20 == 150000, "thêm 10 ổ thì tiền lãi thêm đúng 150 000 — cái ống không bẻ cong đường thẳng"
```

:::hints
- kind: attention
  body: Đọc lại hai dòng `def` ở đầu khung. Máy nào nhận SỐ Ổ, máy nào nhận SỐ TIỀN? Chỗ trống nhận vào một số ổ và phải nhả ra một số tiền lãi, nên con số phải đi qua cả hai — theo đúng một thứ tự duy nhất.
- kind: strategy
  body: Máy chạy trước là máy nhận được số ổ. Kết quả của nó đem bỏ thẳng vào máy còn lại, nên tên máy chạy trước nằm BÊN TRONG cặp ngoặc của máy chạy sau. Đừng đặt thêm cái tên nào cho con số ở giữa, và đừng viết lại con số nào có sẵn trong ruột hai máy.
- kind: one-line
  body: "Ba chỗ trống là `lai(thu(1))`, `lai(thu(20))` và `lai(thu(30))`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: ba chỗ trống phải NỐI hai máy có sẵn — gõ thẳng con số kết quả, hay chép lại 15000 với 30000, đều là mở máy ra thay vì dùng nó
  requireAst:
  # Khung khởi đầu không GỌI máy nào (hai dòng `def` là định nghĩa, không phải
  # lệnh gọi), nên hai luật này chặn được cả đáp án điền bừa lẫn đáp án chỉ
  # dùng một máy rồi tự trừ tay 30000.
  - kind: uses-call, target: thu, min: 3
  - kind: uses-call, target: lai, min: 3
  forbidAst:
  # Lưới thứ hai, chặn hai con số KẾT QUẢ. Lời giải thật chỉ chứa 1, 20 và 30,
  # nên luật này không cản ai làm thật.
  #
  # KHÔNG chặn được 15000 (kết quả buổi thứ nhất là −15000), vì chính ruột máy
  # `thu` chứa số ấy — luật sẽ nổ ngay trên khung khởi đầu. Chỗ chặn thật cho
  # buổi ấy là hai luật `uses-call` phía trên: gõ cứng thì không gọi máy nào.
  - kind: has-literal, target: 270000
  - kind: has-literal, target: 420000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^-15000\n270000\n420000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một buổi lỗ, hai buổi lãi — mà mình chỉ bấm một nút. Cái ống làm nốt phần còn lại.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cái ống vừa bắc chỉ chạy được một chiều: `thu` trước, `lai` sau. Chiều ngược lại
không có nghĩa gì — bỏ **số ổ** vào máy `lai` là trừ một trăm nghìn khỏi mấy cái
ổ bánh mì.

Nhưng chiều nay quán treo cùng lúc **hai tấm biển**:

- *Giảm 20% cả hoá đơn.*
- *Có phiếu thì bớt thêm 10 nghìn.*

Cả hai đều là máy: bỏ **số tiền** vào, nhả **số tiền** ra. Lần này hai đầu khớp
nhau ở **cả hai chiều** — nối kiểu nào cũng bắc được ống.

Một bác khách đưa hoá đơn 200 nghìn cùng tấm phiếu, và đứng chờ.

Bớt phiếu trước rồi mới giảm 20%, hay giảm 20% trước rồi mới bớt phiếu?

Hai cách nối, hai cái máy — hay vẫn là một?
::::

::::checkpoint{mastery=0.8}
::::
