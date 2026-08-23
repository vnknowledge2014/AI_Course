---
id: toan.dai-so-va-ham-so.khi-khong-chay-nguoc-duoc
title: Khi không chạy ngược được
summary: Máy chỉ quay đầu được khi hai đầu vào khác nhau không bao giờ cho cùng một đầu ra — và cái gương của parabol chính là chỗ điều kiện ấy gãy.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 33
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.one-to-one]
requires: [math.inverse-function, math.parabola-symmetry, math.quadratic-function, math.function-notation, math.slope, math.multiply-by-negative, math.negative-number, math.multiplication, math.compare-on-number-line, core.function-def, core.function-parameter, core.function-return, core.function-call, core.boolean, ctrl.comparison, core.variable, core.arithmetic, core.print-variable]
concepts: [math.mot-mot, math.hai-dau-vao-mot-dau-ra, math.may-khac-hien-vat]
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

::::byte{trigger=enter mood=dizzy pose=lean-in}
Chín mét vuông thì cạnh là ba. Hay là âm ba? Mình đứng đây nãy giờ.
::::

::::explain{#cau-hoi-con-treo}
Bài trước để Byte đứng trước một cái máy không chịu quay đầu.

Máy xuôi là `dien_tich(n) = n × n`. Bỏ `9` vào máy ngược, đòi lại cạnh — và có
**hai** câu trả lời cùng đúng:

```text
    3  ──►  [ n × n ]  ──►  9
   −3  ──►  [ n × n ]  ──►  9
```

Cả hai mũi tên đều thật. `3 × 3 = 9`, và `(−3) × (−3) = 9` vì hai dấu trừ triệt
tiêu nhau — T2.1 đã chốt chuyện đó, và bài 28 còn để lại hẳn một dòng kiểm cho
nó.

Giờ quay đầu mũi tên lại:

```text
    9  ──►  [ máy ngược ]  ──►  3 ?  hay  −3 ?
```

Byte có ba lối thoát, và cả ba đều bịt:

- **Nhả ra cả hai.** Không được. Bài 24 đã dựng luật của một cái máy: mỗi đầu
  vào cho **đúng một** đầu ra. Nhả ra hai số thì nó không còn là máy nữa.
- **Chọn bừa một cái, chẳng hạn `3`.** Cũng hỏng, nhưng hỏng ở chỗ tinh hơn: bỏ
  `−3` vào máy xuôi ra `9`, rồi bỏ `9` vào máy ngược thì ra `3`. Vòng xuôi–ngược
  trả về `3` chứ không trả về `−3` — mà bài trước đã nói rõ máy ngược phải trả
  lại **đúng** cái đã bỏ vào.
- **Bảo đề sai.** Không sai chỗ nào. `9` là một diện tích hoàn toàn bình thường.

Vậy kết luận đành phải là: **máy `n × n` không có máy ngược.** Không phải vì ta
chưa nghĩ ra cách dựng, mà vì không có cái máy nào dựng được.
::::

::::example{#hai-cai-bang}
Chỗ hỏng nằm ở đâu? Đặt hai cái máy cạnh nhau rồi đọc **cột kết quả**.

Máy vuông, cho chạy qua cả mốc 0:

| bỏ vào | −3 | −2 | −1 | 0 | 1 | 2 | 3 |
|---|---|---|---|---|---|---|---|
| `n × n` | 9 | 4 | 1 | 0 | 1 | 4 | 9 |

Đọc hàng dưới từ trái sang, rồi từ phải sang: giống hệt nhau. Đó chính là cái
gương mà bài 29 đã chỉ ra trên parabol. Và cái gương ấy làm mỗi con số ở hàng
dưới — trừ số `0` đứng ngay trên trục gương — **xuất hiện hai lần**.

Máy két của bài trước:

| bỏ vào | 0 | 1 | 2 | 3 | 4 |
|---|---|---|---|---|---|
| `15000n + 50000` | 50000 | 65000 | 80000 | 95000 | 110000 |

Hàng dưới không có con số nào lặp lại. Không bao giờ có, dù kéo bảng dài tới
đâu — vì mỗi bước sang phải cộng thêm đúng `15000`, và cộng thêm một lượng khác
`0` thì đi tiếp là đi xa, không có đường quay lại. Đó là bài 26 nói lại bằng chữ
khác.

Hai bảng, một khác biệt duy nhất: bảng trên có số lặp ở hàng kết quả, bảng dưới
thì không. Và đúng cái máy có số lặp là cái máy không quay đầu được.
::::

::::predict{#doan-ba-va-am-ba commitOnce}
Byte gõ mấy dòng để nhìn tận mắt chỗ hai mũi tên chụm vào nhau.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def dien_tich(n):
    return n * n

canh = 3
canh_kia = -3

print(dien_tich(canh))
print(dien_tich(canh_kia))
print(canh == canh_kia)
```

:::opt{correct}
`9`, rồi `9`, rồi `False`
:::

:::opt
`9`, rồi `-9`, rồi `False`
::why
Gần đúng ở chỗ bạn nhớ một luật thật của T2.1: có thừa số âm thì tích mang dấu
trừ. Với `3 × (−3)` thì luật ấy đúng y nguyên — ra `−9`, không bàn cãi.

Ranh giới nằm ở chỗ luật ấy **đếm số dấu trừ**, chứ không chỉ hỏi "có dấu trừ
hay không". Một dấu trừ thì tích âm. Ở đây có **hai** — cả hai thừa số đều âm —
và hai dấu trừ triệt tiêu nhau, đúng như bài về nhân với số âm đã dựng trên
thanh số: nhân với số âm là quay đầu, quay đầu hai lần thì lại nhìn về hướng cũ.
Nên `(−3) × (−3)` bằng `+9`. Và chính vì nó bằng `+9` mà cả bài học này mới có
chuyện để kể.
::
:::

:::opt
`9`, rồi `9`, rồi `True`
::why
Gần đúng ở chỗ bạn dùng đúng lối nghĩ mà cả bài trước đã dựng: hai thứ chạy qua
máy cho ra cùng một kết quả thì coi như một. Với **máy quay đầu được** thì lối
nghĩ ấy đúng hoàn toàn — hai buổi mà két cùng có `350000` đồng thì chắc chắn
cùng bán 20 ổ, không khả năng nào khác.

Ranh giới: đó là tính chất của **một số máy**, không phải của mọi máy — và
`n × n` là cái máy đầu tiên bạn gặp không có nó. Thêm nữa, dấu `==` ở dòng cuối
không so hai đầu ra; nó so thẳng `canh` với `canh_kia`, tức so `3` với `−3`. Trên
thanh số, hai vạch ấy nằm hai bên mốc 0 và cách nhau đúng 6 đơn vị. Chúng khác
nhau, nên máy in `False` — và chính cái `False` ấy là bằng chứng cho thấy hai đầu
vào khác nhau đã cho cùng một đầu ra.
::
:::

:::opt
Máy báo lỗi ở dòng `canh_kia = -3`, vì cạnh của một mảnh sân không thể âm
::why
Gần đúng ở chỗ bạn nói ra một sự thật hoàn toàn đúng, và là sự thật quan trọng
nhất trong cả bài này: **chưa có mảnh sân nào cạnh −3 mét.** Giữ chặt câu đó.

Ranh giới: câu ấy nói về **mảnh sân**, không nói về **cái máy**. Máy `n × n` chỉ
biết nhận một số rồi nhân nó với chính nó; nó chưa từng nghe tới sân, tới mét,
tới hàng rào. Bỏ `−3` vào là hợp lệ hoàn toàn — bài 28 đã cố ý chốt đúng chỗ đó
bằng một dòng kiểm. Nên Python không báo lỗi; nó ngoan ngoãn in `9`. Chỗ cấm số
âm nằm ở mảnh sân chứ không nằm ở cái máy, và lát nữa chính chỗ tách bạch này sẽ
là thứ cứu được máy ngược.
::
:::
::::

::::explain{#luat-mot-mot}
Đặt tên cho điều kiện vừa lộ ra:

> Một cái máy **quay đầu được** khi và chỉ khi **hai đầu vào khác nhau không bao
> giờ cho cùng một đầu ra**. Máy như thế gọi là máy **một–một**: mỗi đầu ra ứng
> với đúng một đầu vào, không nhiều hơn.

Vì sao điều kiện ấy vừa đủ — không thừa, không thiếu:

- **Thiếu nó thì hỏng.** Nếu hai đầu vào khác nhau `p` và `q` cùng cho `k`, thì
  máy ngược nhận `k` phải trả về một trong hai. Trả cái nào cũng bỏ rơi cái kia,
  và vòng xuôi–ngược của cái bị bỏ rơi sẽ trả về nhầm người.
- **Có nó thì chạy.** Nếu mỗi đầu ra chỉ do đúng một đầu vào sinh ra, thì "trả
  lại đầu vào đã sinh ra nó" là một mệnh lệnh không mập mờ. Máy ngược có đúng
  một việc để làm, và nó làm được.

Soi lại cả tủ máy bằng cái điều kiện ấy:

- **Mọi đường thẳng dốc khác 0** đều một–một. Mỗi bước cộng thêm cùng một lượng
  khác 0, nên cột kết quả chỉ đi một chiều, không quay lại. Chúng quay đầu được —
  bài trước đã dựng một cái.
- **Đường thẳng dốc bằng 0** hỏng nặng nhất: nó nhả ra cùng một số cho mọi đầu
  vào. Máy ngược nhận số ấy thì phải trả về… tất cả.
- **Parabol** hỏng vì có gương. Và đây không phải chuyện riêng của mảnh sân: bài
  29 đã cho bạn thấy đúng chuyện ấy trên **đường lãi** của quán — tăng giá `2`
  nghìn và tăng giá `8` nghìn cho **cùng một** khoản lãi `364` nghìn, vì hai mức
  ấy cách trục gương (`t = 5`) bằng nhau. Nên câu hỏi *"muốn lãi 364 nghìn thì
  tăng giá bao nhiêu"* có **hai** câu trả lời. Đó không phải lỗi tính toán; đó là
  câu trả lời đúng của một câu hỏi có hai đáp án.

Còn mảnh sân thì sao — chẳng lẽ Byte chịu, không đo ngược được cạnh từ diện
tích? Chỗ gỡ nằm đúng ở chỗ tách bạch của bài 28, và nó **không** phải một luật
toán mới:

> Máy `n × n` nhận mọi số. **Mảnh sân** thì chưa bao giờ nhận số âm — không phải
> vì toán cấm, mà vì sân là sân.

Khi cái máy ấy đang làm việc cho một mảnh sân, đầu vào chỉ có thể là `0` trở lên.
Đọc lại bảng nhưng chỉ đọc nửa từ `0` sang phải:

| bỏ vào | 0 | 1 | 2 | 3 | 4 |
|---|---|---|---|---|---|
| `n × n` | 0 | 1 | 4 | 9 | 16 |

Không còn số nào lặp. Cái gương vẫn ở đó, nhưng nửa bên kia của nó không còn đầu
vào nào đứng. Máy một–một trở lại, và máy ngược sống lại: bỏ `9` vào, nó trả `3`.

Cái chữa không phải "ta cắt cho hết hỏng". Cái chữa là **nhớ ra cái máy đang làm
việc cho ai.**
::::

::::code{#do-hai-may}
Bắt máy tự chỉ ra chỗ khác nhau giữa hai cái máy: một cái có số lặp ở hàng kết
quả, một cái không.

Bạn dựng lại hai cái máy đã quen, rồi chạy mỗi cái trên **hai đầu vào khác
nhau**:

- **Máy vuông** chạy trên `3` và `−3` — hai cạnh đối nhau qua mốc 0.
- **Máy két** chạy trên `3` ổ và `8` ổ.

Đọc ba dòng máy in ra: hai số đầu **trùng nhau** — đó là chỗ máy vuông gãy; số
thứ ba **khác 0** — đó là chỗ máy két lành lặn.

Bài chấm bằng **cả hai cái máy**, và chúng cho những con số khác hẳn nhau. Gõ
cứng `9` vào mọi chỗ thì máy két sai; gõ cứng một khoản tiền thì máy vuông sai.
Chỉ hai cái máy viết thật mới qua được cả hai.

```python title=starter
# Máy vuông của bài 28: bỏ một số vào, nhả ra số ấy nhân chính nó.
def dien_tich(n):
    return ___

# Máy két của bài trước: 15000 một ổ, cộng 50000 tiền lẻ bỏ sẵn trong két.
def ket(n):
    return ___

# Máy vuông, hai đầu vào khác nhau ở hai bên mốc 0.
vuong_duong = dien_tich(3)
vuong_am = ___

# Máy két, hai đầu vào khác nhau.
ket_it = ket(3)
ket_nhieu = ___

print(vuong_duong)
print(vuong_am)
print(ket_nhieu - ket_it)
```

```python title=solution
# Máy vuông của bài 28: bỏ một số vào, nhả ra số ấy nhân chính nó.
def dien_tich(n):
    return n * n

# Máy két của bài trước: 15000 một ổ, cộng 50000 tiền lẻ bỏ sẵn trong két.
def ket(n):
    return 15000 * n + 50000

# Máy vuông, hai đầu vào khác nhau ở hai bên mốc 0.
vuong_duong = dien_tich(3)
vuong_am = dien_tich(-3)

# Máy két, hai đầu vào khác nhau.
ket_it = ket(3)
ket_nhieu = ket(8)

print(vuong_duong)
print(vuong_am)
print(ket_nhieu - ket_it)
```

```python title=test
# Hai câu `!=` đứng TRƯỚC. Chúng canh đúng cái bẫy của bài: chép một con số vào
# cả bốn chỗ thì mọi thứ bằng nhau hết, và cái khác biệt mà bài này dựng lên cả
# trang để nói tới sẽ biến mất. Xếp chúng sau các câu `==` thì không bao giờ
# chạy tới, và bẫy không bao giờ sập.
assert ket_it != ket_nhieu, "3 ổ và 8 ổ phải cho hai khoản tiền khác nhau — chính vì thế máy két mới quay đầu được"
assert vuong_duong != ket_it, "hai cái máy khác nhau, đo hai thứ khác nhau, nên chúng không thể cùng nhả ra một con số ở đây"
assert ket(1) != ket(2) and ket(2) != ket(3), "cột kết quả của máy két không lặp lại ở đâu cả, đó là điều kiện một–một"
assert vuong_duong == vuong_am, "hai đầu vào khác nhau mà máy vuông nhả ra CÙNG một số — đó đúng là chỗ máy ngược gãy"
assert vuong_duong == 9, "3 nhân 3 bằng 9"
assert vuong_am == 9, "hai dấu trừ triệt tiêu nhau, nên −3 nhân −3 cũng bằng 9"
assert ket_nhieu - ket_it == 75000, "chênh 5 ổ, mỗi ổ 15000 đồng; 50000 tiền lẻ có mặt ở cả hai bên nên nó tự triệt tiêu"
assert dien_tich(-5) == dien_tich(5), "cái gương không phải chuyện riêng của 3 và −3: MỌI cặp số đối nhau đều rơi vào cùng một chỗ"
assert ket(0) == 50000, "bỏ 0 ổ vào máy két thì còn đúng chỗ tiền lẻ bỏ sẵn — đó là con số `b` của bài 27"
```

:::hints
- kind: attention
  body: Hai chỗ trống đầu là thân của hai cái máy, nên chúng phải viết theo chữ `n` — chú thích ngay trên mỗi `def` đã kể đủ máy ấy làm gì. Hai chỗ trống sau thì nhìn dòng ngay phía trên nó: dòng ấy đã viết sẵn khuôn, chỉ khác con số bỏ vào.
- kind: strategy
  body: "Máy vuông là \"số ấy nhân chính nó\", nên cùng một chữ `n` phải đứng ở cả hai bên dấu nhân. Máy két là giá một ổ nhân số ổ rồi cộng tiền lẻ, đúng theo thứ tự ấy. Hai chỗ cuối thì đừng gõ thẳng kết quả — cứ gọi tên máy và bỏ đầu vào vào, vì chính hai kết quả trùng nhau của máy vuông là thứ bài đang nhờ máy chỉ ra."
- kind: one-line
  body: "Bốn chỗ lần lượt là `n * n`, `15000 * n + 50000`, `dien_tich(-3)` và `ket(8)`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: hai cái máy phải viết theo chữ `n`, và hai chỗ cuối phải CHẠY máy chứ đừng gõ kết quả — gõ cứng con số thì hai đầu ra trùng nhau của máy vuông không được chứng minh ở đâu cả
  requireAst:
  # Khung khởi đầu không có dấu `*` hay `+` nào ngoài chú thích, mà chú thích thì
  # không nằm trong cây cú pháp. Hai luật này một mình đã chặn đáp án chép cứng
  # thân hai cái máy.
  - kind: uses-operator, target: *, min: 2
  - kind: uses-operator, target: +, min: 1
  # Hai cái máy phải viết theo `n`, không theo một con số. Khung khởi đầu không
  # đọc `n` lần nào; lời giải đọc 3 lần (hai lần trong máy vuông, một trong máy
  # két) — đúng chỗ `n * n` khác hẳn `n * 3`.
  - kind: uses-name, target: n, min: 3
  # Hai chỗ trống cuối phải CHẠY máy, không được gõ kết quả. Khung khởi đầu gọi
  # mỗi máy đúng 1 lần; lời giải gọi 2 lần.
  - kind: uses-call, target: dien_tich, min: 2
  - kind: uses-call, target: ket, min: 2
  forbidAst:
  # `uses-operator` đếm trên cả file nên không chặn nổi đáp án chép cứng ĐÚNG
  # MỘT chỗ. Bốn luật dưới chặn từng kết quả một. Chúng không cản cách viết hợp
  # lệ nào: `n * n`, `15000 * n + 50000`, `dien_tich(-3)`, `ket(8)` đều không
  # chứa nguyên văn con số nào trong danh sách.
  - kind: has-literal, target: 9
  - kind: has-literal, target: 95000
  - kind: has-literal, target: 170000
  - kind: has-literal, target: 75000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^9\n9\n75000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chín rồi lại chín — máy vuông đúng là đang nói hai chuyện bằng một câu. Còn máy
két thì rành mạch từng đồng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Tủ của Byte giờ khá đầy: máy thẳng, máy cong, máy nhân đôi, và với những máy
một–một thì có thêm cả máy ngược đi kèm.

Nhưng chiều nay Byte cần một con số mà **chưa máy nào trong tủ nhả ra được**:
tiền **lãi**.

Tính nó thì phải làm hai việc, theo đúng thứ tự này:

1. Từ **số ổ**, tính ra **tiền thu** — cái máy nhân `15000` mà bạn đã quen.
2. Từ **tiền thu**, trừ đi tiền thuê chỗ cả ngày — việc này cũng là một cái máy.

Byte làm được: chạy máy thứ nhất, cầm con số nó nhả ra, đem nhét vào máy thứ
hai. Hai lần bỏ vào, hai lần lấy ra, và một mẩu giấy nháp ở giữa để ghi con số
trung gian.

Chỉ có điều tối nào cũng làm hai lượt thì phiền. Byte muốn một cái máy **duy
nhất**: bỏ số ổ vào một đầu, tiền lãi rơi ra đầu kia, không mẩu giấy nháp nào ở
giữa.

Nối đuôi hai cái máy có thành một cái máy mới được không — và nếu được thì cái
máy mới ấy tên là gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
