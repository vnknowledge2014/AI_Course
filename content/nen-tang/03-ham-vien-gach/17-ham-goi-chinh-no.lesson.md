---
id: nen-tang.ham-vien-gach.ham-goi-chinh-no
title: Hàm gọi chính nó
summary: Viết lại chính tên hàm trong thân hàm đó thì máy chấp nhận, và chồng lời gọi cứ cao mãi cho tới khi máy dừng lại bằng RecursionError.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [func.recursion, err.recursion-error]
requires: [func.call-stack, core.function-def, core.function-call, core.function-parameter, core.function-argument, core.arithmetic, core.fstring, core.output, err.traceback]
concepts: [func.ham, func.de-quy]
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
Mình nhờ được người khác. Vậy mình nhờ chính mình thì sao nhỉ.
::::

::::explain{#may-co-cam-viet-nhu-vay-khong}
Bài trước để lại một câu hỏi bỏ ngỏ, và trước khi trả lời nó thì phải hỏi một
câu nhỏ hơn trước: máy có **cho phép** viết như vậy không?

Nghe thì lạ tai. Hàm còn đang được định nghĩa dở mà đã gọi chính nó — như thể
bà chủ quán, giữa lúc làm tô phở cho khách A, lại đặt xuống bàn một phiếu order
mới, và trên phiếu ấy ghi đúng một việc: *làm một tô phở*.

Nhưng hãy nhớ máy nhìn thấy gì. Khi đọc dòng `def`, máy **không chạy** một dòng
nào trong thân hàm. Nó chỉ ghi lại: từ nay cái tên này ứng với đoạn việc kia.
Mọi cái tên viết trong thân chỉ được đem đi tìm vào lúc thân ấy thật sự chạy —
tức là lúc hàm được gọi. Mà lúc đó thì dòng `def` đã đọc xong từ đời nào, cái
tên đã có mặt đàng hoàng.

Nên câu trả lời là: máy cho phép, và cho phép một cách bình thản. Nó không có
luật nào cấm một cái tên xuất hiện trong chính đoạn việc mang tên đó.

Cách viết ấy có tên riêng: **đệ quy** — hàm gọi lại chính nó.

Đây là hàm Byte viết để đếm ngược:

```python
def dem_nguoc(n):
    print(f"Byte đếm: {n}")
    dem_nguoc(n - 1)

dem_nguoc(3)
```

Đúng ba dòng, và không dòng nào là cú pháp mới. Dòng thứ ba là một lời gọi hàm
y hệt mọi lời gọi bạn đã viết, chỉ khác đúng một điểm: cái tên trong đó là tên
của hàm đang chứa nó.
::::

::::predict{#doan-chuyen-gi-xay-ra commitOnce}
**Trước khi bấm chạy**, bạn đoán màn hình hiện ra chuyện gì?

```python
def dem_nguoc(n):
    print(f"Byte đếm: {n}")
    dem_nguoc(n - 1)

dem_nguoc(3)
```

:::opt{correct}
In ra một dãy số rất dài, càng lúc càng âm, rồi chương trình dừng bằng một lỗi
:::

:::opt
Máy báo lỗi ngay lúc đọc dòng `def`, vì cái tên `dem_nguoc` chưa viết xong mà đã bị gọi
::why
Gần đúng ở một chỗ rất đáng khen: bạn đang áp luật của Realm 0 — một cái tên
chưa được đặt thì máy tìm không ra và báo `NameError`. Luật ấy có thật và bạn
nhớ đúng.

Chỗ lệch nằm ở **lúc nào** máy đi tìm cái tên. Đọc dòng `def`, máy chỉ cất
thân hàm đi chứ không chạy dòng nào trong đó, nên nó chưa đi tìm `dem_nguoc`
lần nào cả. Tới khi dòng cuối gọi `dem_nguoc(3)` thì dòng `def` đã đọc xong,
cái tên đã nằm sẵn đó. Lúc thân hàm chạy tới lời gọi bên trong, máy tìm là
thấy ngay.
::
:::

:::opt
In ra 3, 2, 1, 0 rồi dừng, vì đếm ngược tới 0 là hết
::why
Gần đúng ở chỗ bạn đọc ra ý định của người viết — cái tên `dem_nguoc` nói rằng
nó đếm ngược, và đếm ngược thì đời thường dừng ở 0.

Chỗ lệch: cái tên hàm là chữ viết cho người đọc, máy không đọc nghĩa của nó.
Máy chỉ làm đúng ba dòng trong thân: in ra `n`, rồi gọi lại chính mình với
`n - 1`, hết. Không dòng nào nói "tới 0 thì thôi", nên tới 0 máy vẫn gọi tiếp
với -1, rồi -2. Muốn có chỗ dừng thì chỗ dừng phải được viết ra.
::
:::

:::opt
Chương trình chạy mãi mãi không bao giờ dừng, phải tự tay tắt nó đi
::why
Gần đúng ở chỗ khó nhất, và phần suy luận của bạn chính xác: không có dòng nào
bảo dừng, nên xét theo đúng chữ trong code thì nó đúng là không có điểm kết.

Chỗ lệch nằm ở một thứ nằm ngoài code — cái chồng lời gọi ở bài trước. Mỗi lần
gọi lại là thêm một tờ phiếu, mà mỗi tờ phiếu chiếm chỗ nhớ thật. Chồng cao mãi
thì máy hết chỗ. Python không đợi tới lúc ấy: nó đặt sẵn một mức trần cho độ
cao của chồng, và vượt trần thì nó dừng chương trình bằng một thông báo lỗi
đàng hoàng thay vì chết lặng.
::
:::
::::

::::example{#chong-cao-toi-tran}
Chạy thật đoạn ấy. Byte thu gọn phần giữa lại cho vừa trang, phần đầu và phần
cuối để nguyên.

```python title=readonly
def dem_nguoc(n):
    print(f"Byte đếm: {n}")
    dem_nguoc(n - 1)

dem_nguoc(3)
```

Máy in ra:

```text
Byte đếm: 3
Byte đếm: 2
Byte đếm: 1
Byte đếm: 0
Byte đếm: -1
Byte đếm: -2
...
Byte đếm: -990
Traceback (most recent call last):
  File "dem.py", line 5, in <module>
    dem_nguoc(3)
  File "dem.py", line 3, in dem_nguoc
    dem_nguoc(n - 1)
  File "dem.py", line 3, in dem_nguoc
    dem_nguoc(n - 1)
  [Previous line repeated 993 more times]
  File "dem.py", line 2, in dem_nguoc
    print(f"Byte đếm: {n}")
RecursionError: maximum recursion depth exceeded
```

Ba điều đáng dừng lại nhìn kỹ.

**Nó chạy được thật.** Bốn dòng đầu in ra đúng như một hàm đếm ngược tử tế.
Không có `SyntaxError`, không có `NameError`. Máy chấp nhận đệ quy, y như đã
nói ở trên.

**Nó đi qua 0 mà không hề ngập ngừng.** `0 - 1` là -1, và -1 vẫn là một con số
hợp lệ để in ra và để trừ tiếp. Con số 0 chẳng có gì đặc biệt với máy — nó chỉ
đặc biệt với người đọc cái tên `dem_nguoc`.

**Traceback in ra toàn một cái tên.** Bài trước dạy rằng mỗi cặp dòng trong
traceback là một tờ phiếu đang nằm trên chồng. Ở đây gần một nghìn tờ phiếu ấy
đều mang cùng một tên hàm, đều đang đợi ở cùng một dòng. Python thấy chúng
giống hệt nhau nên gộp lại thành một câu: *dòng vừa rồi lặp thêm chừng ấy lần
nữa*.

Dòng cuối cùng — dòng đáng đọc nhất, theo đúng luật đọc lỗi của Realm 0 — là
`RecursionError: maximum recursion depth exceeded`. Dịch ra tiếng Việt cho
đúng nghĩa của bài trước: **chồng lời gọi đã cao quá mức máy cho phép**.
::::

::::explain{#vi-sao-chong-khong-bao-gio-go}
Giờ trả lời thẳng câu hỏi bài trước để lại.

Cái chồng gỡ ra được là nhờ một lượt gọi **chạy hết thân mình** — chạy tới
`return`, hoặc đơn giản là hết dòng để chạy. Bài 11 đã chốt: hàm không viết
`return` vẫn xong lượt, và vẫn đưa ra `None`. Mỗi lần một hàm chạy hết việc,
tờ phiếu trên cùng bị bỏ đi và tầng dưới được chạy tiếp. Ở `tra_gia` bài trước, chuỗi
gọi chạm tới một hàm **không gọi ai nữa**, và đó là chỗ chồng bắt đầu thấp
xuống.

Trong `dem_nguoc` thì không có cái chỗ ấy. Mỗi lần vào thân, máy đi được hai
dòng: in một câu, rồi gọi lại chính mình. Dòng gọi ấy chưa xong thì lượt hiện
tại chưa xong, nên tờ phiếu hiện tại chưa gỡ được — mà lượt mới lại rơi vào
đúng tình cảnh y hệt. Chồng chỉ có một chiều: lên.

Đây là chỗ đáng ghi lại, vì nó là cùng một sự thật nhìn từ hai phía:

- Nhìn từ **code**: không dòng nào bảo dừng, nên không có lượt gọi nào kết
  thúc.
- Nhìn từ **cái chồng**: không tờ phiếu nào được gỡ, nên chồng cao tới trần.

`RecursionError` vì vậy không phải một lỗi bí ẩn. Nó là câu máy nói ra khi cái
chồng bạn đã hình dung được ở bài trước cao quá sức chứa. Mức trần ấy vào
khoảng một nghìn lời gọi — con số cụ thể tuỳ máy, và nó không phải thứ đáng
nhớ. Thứ đáng nhớ là ý nghĩa của nó.

> Chỗ dễ vấp: thấy `RecursionError` rồi kết luận "đệ quy là thứ hỏng, đừng
> dùng". Đoạn code trên hỏng không phải vì nó gọi lại chính nó. Nó hỏng vì
> **mọi** lượt gọi đều gọi tiếp, không chừa lượt nào.
::::

::::sandbox{#nghich-voi-cai-chong}
Đây là sân chơi — không có đáp án đúng, không ai chấm. Chương trình dưới đây sẽ
dừng lại bằng lỗi, và đó là chuyện đáng xảy ra chứ không phải bạn làm sai.

Vài thứ đáng thử, mỗi lần một thứ:

- Đổi `dem_nguoc(3)` thành `dem_nguoc(1000000)`. Chồng có cao hơn không, hay
  vẫn đúng chừng ấy tầng?
- Đổi `n - 1` thành `n + 1`. Có gì khác không?
- Bỏ hẳn dòng `print` đi, để lại mỗi lời gọi. Màn hình im lặng, nhưng câu cuối
  cùng máy nói có đổi không?
- Đổi `n - 1` thành `n`. Con số đứng yên một chỗ — máy có vì thế mà dừng lại
  không?

```python title=starter
def dem_nguoc(n):
    print(f"Byte đếm: {n}")
    dem_nguoc(n - 1)

dem_nguoc(3)
```
::::

::::byte{trigger=enter mood=thinking pose=lean-in}
Mình gọi được chính mình. Mình chỉ chưa biết lúc nào thì thôi không gọi nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại cho công bằng: đoạn code vừa rồi không sai cú pháp, không sai tên,
không sai kiểu. Máy hiểu trọn vẹn từng dòng và làm đúng từng chữ bạn viết. Bốn
dòng đầu nó in ra chính xác cái bạn muốn.

Nó chạy được, chỉ là không chịu dừng. Thiếu đúng một thứ. Thứ đó là gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
