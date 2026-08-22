---
id: nen-tang.gia-tri-bien-kieu.cai-ten-may-tu-choi
title: Cái tên máy từ chối thẳng
summary: Có ba luật cứng về hình dạng một cái tên. Phạm luật thì máy không im lặng, cũng không đợi chạy tới — nó từ chối trước khi chạy dòng nào.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.name-rules]
requires: [core.variable, core.assignment, core.builtin-shadowing, core.fstring, err.syntax-error, err.name-error]
concepts: [core.ten, core.loi-truoc-khi-chay]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Có mấy chục từ mình không nhường được. Không phải vì tiếc — vì mình cần chúng để đọc câu của bạn.
::::

::::explain{#khau-lenh-trong-bep}
Bài trước kết bằng một câu hỏi: máy để bạn đè lên `int`, lên `print`, lên
`round` — vậy nó **có** từ chối cái tên nào không?

Có. Và cách nó từ chối khác hẳn.

Trong bếp quán phở, người ta gọi nhau bằng biệt danh gì cũng được: chị Ba, anh
Tí, cô Hoa. Nhưng có mấy từ không ai dám lấy làm biệt danh, vì chúng là **khẩu
lệnh**: "Tái!", "Chín!", "Nạm!", "Thêm hành!". Đặt biệt danh một người là
"Chín" thì mỗi lần bếp hô một tô chín, cả quán đứng hình không biết đang gọi
món hay gọi người.

Python cũng có một nhúm từ như thế. Chúng không phải tên dán lên công cụ như
`int` — chúng là **khẩu lệnh của chính ngữ pháp**: `if` mở một lối rẽ, `for`
mở một vòng lặp, `return` trả kết quả về. Máy dựa vào chúng để hiểu câu bạn
viết có hình dạng gì. Nhường một từ như vậy cho bạn làm tên biến thì máy mất
luôn khả năng đọc.

Người ta gọi chúng là **từ khoá** — tiếng Anh là *keyword*.

Bạn đã dùng qua kha khá rồi: `if`, `elif`, `else`, `for`, `in`, `and`, `or`,
`not`, `is`, `def`, `return`, `None`, `True`, `False`. Cả bảng có đúng 35 từ;
số còn lại bạn sẽ gặp dần ở các track sau.

Từ khoá là **luật thứ ba** trong ba luật cứng về hình dạng một cái tên:

1. **Chỉ chữ cái, chữ số và dấu gạch dưới `_`.** Không dấu cách, không dấu
   `-`, không dấu chấm. (Chữ cái ở đây tính cả chữ tiếng Việt có dấu: `số_người`
   là một cái tên hợp lệ thật.)
2. **Không mở đầu bằng chữ số.** `so_2` được, `2_nguoi` thì không.
3. **Không được là một trong 35 từ khoá.**

Phạm bất kỳ luật nào trong ba luật này, máy không im lặng như bài trước. Nó
cũng không đợi chạy tới dòng đó. Nó từ chối cả file, trước khi chạy dòng nào.
::::

::::example{#tu-choi-truoc-khi-chay}
Ba dòng gán, mỗi dòng phạm một luật. Chạy từng dòng một để thấy máy nói gì.

**Luật 3 — lấy từ khoá làm tên.** Chú ý dòng `print` ở trên cùng:

```python title=readonly
print("Sổ tháng 8")
class = "ăn ngoài"
print(class)
```

```text
  File "so_chi_tieu.py", line 2
    class = "ăn ngoài"
          ^
SyntaxError: invalid syntax
```

Dòng `print("Sổ tháng 8")` viết đúng từng chữ. Nó **không in ra gì cả**. Đây là
chỗ khác biệt lớn nhất so với bài trước: `SyntaxError` là lỗi máy bắt được lúc
**soát**, trước khi chạy — đúng thứ bạn đã gặp ở Realm 0 hôm quên một dấu
nháy. Chưa soát xong cả file thì chưa dòng nào được phép chạy.

**Luật 2 — mở đầu bằng chữ số:**

```python title=readonly
2_nguoi = 2
```

```text
  File "so_chi_tieu.py", line 1
    2_nguoi = 2
     ^
SyntaxError: invalid decimal literal
```

*Decimal literal* nghĩa là "con số viết thẳng ra". Thấy chữ số đứng đầu, máy
tưởng bạn đang viết một con số, và nó đọc tiếp `_nguoi` như phần đuôi của con
số ấy. Không có con số nào đuôi là `nguoi`, nên nó dừng. Máy không đoán rằng ý
bạn là một cái tên.

**Luật 1 — có ký tự lạ:**

```python title=readonly
tien-moi-nguoi = 60000
```

```text
  File "so_chi_tieu.py", line 1
    tien-moi-nguoi = 60000
    ^^^^^^^^^^^^^^
SyntaxError: cannot assign to expression here. Maybe you meant '==' instead of '='?
```

Dấu `-` với máy là **phép trừ**, không phải dấu nối chữ. Nên nó không đọc ra
một cái tên, nó đọc ra một phép tính: `tien` trừ `moi` trừ `nguoi`. Rồi bạn bảo
nó gán vào phép tính ấy. *Cannot assign to expression* nghĩa là "không gán vào
một phép tính được" — mảnh giấy dán phải dán lên một cái tên, không dán lên
một biểu thức.

Ba thông báo, ba câu chữ khác nhau, nhưng cùng một họ: `SyntaxError`. Và cùng
một hậu quả: không dòng nào chạy.
::::

::::predict{#doan-may-in-gi commitOnce}
Byte ghi một khoản vào sổ và đặt tên biến là `class` cho "loại khoản chi".
**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print("Sổ tháng 8")
tien = 25000
class = "cà phê"
print(f"{class}: {tien}đ")
```

:::opt{correct}
Máy không in dòng nào cả, chỉ báo SyntaxError ở dòng 3
:::

:::opt
In ra `Sổ tháng 8`, rồi mới báo lỗi ở dòng 3
::why
Gần đúng ở chỗ bạn áp đúng cách máy làm việc mà bài trước vừa cho thấy: chạy
lần lượt từ trên xuống, dòng nào tới lượt thì chạy, tới dòng hỏng mới dừng.
Với lỗi lúc chạy — `TypeError` của bài trước chẳng hạn — đúng là như vậy, và
hai dòng đầu đã kịp in ra thật.

Chỗ lệch nằm ở loại lỗi. Lỗi ngữ pháp bị bắt ở bước **soát**, trước lượt chạy
đầu tiên. Máy đọc hết cả file để hiểu từng câu có hình dạng gì đã; gặp một câu
không đọc ra nghĩa, nó dừng ngay tại bước đó. Lượt chạy chưa bao giờ bắt đầu,
nên `print` ở dòng 1 chưa hề tới lượt.
::
:::

:::opt
Chạy êm hết, in ra hai dòng — `class` là tên có sẵn nên đè được như `int`
::why
Gần đúng ở chỗ bạn dùng đúng luật vừa học ở bài trước, và dùng đúng chỗ đáng
dùng: đè lên một cái tên máy dọn sẵn thì máy nhận, không kêu một tiếng.

Chỗ lệch nằm ở chỗ `class` không cùng loại với `int`. `int` là một **cái tên**,
đang dán lên một công cụ — nên gỡ ra dán chỗ khác được. `class` không dán lên
giá trị nào cả; nó là một mảnh ngữ pháp, đứng cùng hạng với dấu `=` hay dấu hai
chấm. Đặt nó bên trái dấu `=` thì câu ấy không còn hình dạng nào máy đọc ra
được — y như viết `= = 5`.
::
:::

:::opt
Máy báo NameError, vì `class` chưa từng được gán
::why
Gần đúng ở chỗ bạn nhớ đúng luật: một cái tên chỉ dùng được từ dòng gán trở
xuống, dùng trước thì `NameError`.

Chỗ lệch có hai lớp. Thứ nhất, dòng 3 chính là dòng **đang gán** cho `class`,
nên nếu `class` là một cái tên bình thường thì nó đã có mặt từ đó. Thứ hai,
`NameError` là lỗi lúc chạy — mà chương trình này chưa chạy dòng nào để kịp
tra bảng tên.
::
:::
::::

::::explain{#ba-muc-nghiem-khac}
Xếp ba bài liền nhau cạnh nhau, bạn thấy một cái thang:

| bạn viết | máy làm gì | báo lúc nào |
|---|---|---|
| dùng tên chưa từng gán | báo `NameError` | đúng dòng đó, lúc chạy |
| đè lên tên có sẵn (`int = 0`) | im lặng nhận | nổ ở chỗ khác, có khi rất xa |
| lấy từ khoá làm tên (`class = 0`) | từ chối cả file | trước khi chạy dòng nào |

Nhìn theo chiều "máy giúp bạn được bao nhiêu" thì dòng cuối là dòng tử tế
nhất: lỗi lộ ra sớm nhất, gần chỗ bạn gõ nhất, và không có dòng nào kịp chạy
để làm hỏng gì.

Hai mẹo kiểm tra, mỗi mẹo cho một loại:

- **Đây có phải từ khoá không?** Viết thử một dòng `ten_do = 0` rồi chạy.
  `SyntaxError` nghĩa là từ khoá — đổi tên khác.
- **Đây có phải tên có sẵn không?** Mẹo của bài trước: `print(ten_do)`. In ra
  mô tả một công cụ nghĩa là chỗ đó có người ở; `NameError` nghĩa là chỗ trống.

Một chỗ dễ vấp cuối bài: `None`, `True`, `False` cũng nằm trong 35 từ khoá.
Bạn đã dùng `ghi_chu = None` suốt mấy bài trước và nó chạy ngon lành — vì ở đó
`None` đứng **bên phải** dấu `=`, chỗ dành cho giá trị. Kéo nó sang **bên
trái**, viết `None = 5`, thì máy trả lời rất thẳng:

```text
  File "so_chi_tieu.py", line 1
    None = 5
    ^^^^
SyntaxError: cannot assign to None
```
::::

::::code{#dat-lai-ba-cai-ten}
Byte gõ vội một dòng sổ: khoản **"ăn ngoài"**, **2 người**, mỗi người
**60000đ**. Ba cái tên Byte chọn thì phạm đủ ba luật, mỗi cái một luật.

Đặt lại tên cho cả ba, và sửa cả chỗ gọi chúng trong f-string ở dòng cuối. Ba
tên mới phải là: `so_nguoi`, `loai_khoan`, `tien_moi_nguoi`.

Một chuyện nên biết trước khi chạy: máy chỉ báo **lỗi đầu tiên** nó gặp. Sửa
xong cái thứ nhất, chạy lại, bạn sẽ thấy cái thứ hai hiện ra. Đó là chuyện
bình thường, không phải bạn sửa hỏng thêm.

```python title=starter
2_nguoi = 2
class = "ăn ngoài"
tien-moi-nguoi = 60000

print(f"{class}: {2_nguoi} người, mỗi người {tien-moi-nguoi}đ")
```

```python title=solution
so_nguoi = 2
loai_khoan = "ăn ngoài"
tien_moi_nguoi = 60000

print(f"{loai_khoan}: {so_nguoi} người, mỗi người {tien_moi_nguoi}đ")
```

```python title=test
# Ba câu hỏi cho ba cái tên, vì ba cái tên phạm ba luật khác nhau — sửa được
# một cái thì máy hết báo lỗi đầu tiên, nhưng bài chưa xong.
# Giá trị phải đi theo đúng tên cũ của nó: ai đổi tên xong mà gán nhầm 2 cho
# `tien_moi_nguoi` thì trượt ở đây chứ không lọt qua bằng một dòng in trông
# giông giống.
assert so_nguoi == 2
assert loai_khoan == "ăn ngoài"
assert tien_moi_nguoi == 60000
```

:::hints
- kind: attention
  body: Đọc lại ba luật cứng rồi soi từng dòng: dòng 1 mở đầu bằng cái gì, dòng 2 dùng từ nào, dòng 3 có ký tự nào không phải chữ, số hay `_`. Mỗi dòng phạm đúng một luật.
- kind: strategy
  body: Đổi tên ở dòng gán thôi thì chưa đủ — f-string ở dòng cuối vẫn đang gọi ba cái tên cũ, và tên cũ thì máy vẫn không đọc ra được. Sửa từng cặp một, chạy lại sau mỗi lần để thấy máy chuyển sang báo lỗi kế tiếp.
- kind: one-line
  body: Ba dòng gán lần lượt thành `so_nguoi = 2`, `loai_khoan = "ăn ngoài"`, `tien_moi_nguoi = 60000`; trong f-string ở dòng cuối, thay ba cặp ngoặc nhọn bằng đúng ba cái tên mới ấy, giữ nguyên thứ tự cũ.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: ăn ngoài: 2 người, mỗi người 60000đ
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đọc được rồi. Ba cái tên này mình nhận, và mình nhận từ lúc soát chứ không phải đợi tới lúc chạy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Máy cấm đúng 35 từ. Ngoài 35 từ ấy ra, nó nhận tuốt — và nhận **như nhau**.

Thử nhìn bốn cái tên này, cả bốn đều hợp lệ, cả bốn đều chạy y hệt nhau:

```python
t = 60000
x1 = 60000
dulieu = 60000
tien_moi_nguoi = 60000
```

Máy không có ý kiến gì về việc chọn cái nào. Với nó, `t` và `tien_moi_nguoi`
là hai cái tên ngang hàng, không cái nào tốt hơn cái nào.

Nhưng bạn thì có ý kiến, nhất là ba tháng sau lúc mở lại file này.

Vậy khi máy đã hết quan tâm, ai là người còn lại quyết định tên nào tốt — và
quyết định dựa vào cái gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
