---
id: nen-tang.gia-tri-bien-kieu.chua-co-gi-khac-bang-khong
title: '"Chưa có gì" không phải số 0'
summary: Máy có sẵn một giá trị dành riêng cho ô chưa ai chạm bút vào — nó không phải số 0, cũng không phải chuỗi rỗng.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.none]
requires: [core.variable, core.assignment, core.string-literal, core.number-literal, core.type-fn, core.fstring]
concepts: [core.gia-tri, core.kieu-gia-tri, core.o-trong]
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
  reviewed: true
---

::::byte{trigger=enter mood=curious pose=lean-in}
Ô đã hỏi mà để trống, với ô chưa ai chạm bút vào — mình có hai chỗ khác nhau.
::::

::::explain{#hai-o-trong-tren-mot-trang-so}
Bài trước để lại một chỗ vướng. Khoản gửi xe **0 đồng** — quán cho gửi miễn phí
— rơi vào nhánh `False`, y hệt ô bạn **chưa nhập gì cả**. Cùng một nhánh, hai
câu chuyện khác hẳn nhau.

Mở cuốn sổ chi tiêu ra nhìn cho kỹ. Dòng hôm nay có ba ô: tên khoản, tiền, ghi
chú. Hai ô dưới đây trông đều "không có gì", nhưng người ghi sổ phân biệt được
ngay:

- Ô tiền của dòng **gửi xe** có nét mực: một con số `0`. Bạn đã hỏi bà chủ, bà
  bảo cho gửi miễn phí. Số `0` ở đây là một **câu trả lời**.
- Ô tiền của dòng **ăn trưa** thì trắng giấy. Bạn chưa hỏi ai. Không có câu trả
  lời nào cả, kể cả câu trả lời "không mất đồng nào".

Mắt bạn phân biệt hai ô ấy bằng nét mực. Máy thì không nhìn thấy tờ giấy. Với
máy, một cái tên bao giờ cũng đang giữ **một giá trị nào đó** — không có chuyện
giữ khoảng trắng. Vậy nên nếu muốn nói "ô này chưa có gì", bạn phải đưa cho máy
một giá trị mang đúng nghĩa ấy.

Python có sẵn một giá trị như thế, và nó chỉ có mỗi việc đó: **`None`**.

Viết hoa chữ `N`, ba chữ còn lại thường, và **không có dấu nháy**. Bọc nó vào
nháy thành `"None"` thì bạn được một chuỗi bốn ký tự — một câu chữ như mọi câu
chữ khác, không còn nghĩa "chưa có gì" nữa.

`None` cũng có nhãn kiểu của riêng nó, như mọi giá trị khác: `NoneType`. Đó là
một kiểu đặc biệt ở chỗ cả kiểu chỉ có đúng **một** giá trị duy nhất — chính là
`None`. Không có `None` thứ hai để mà lẫn.
::::

::::example{#ba-kieu-trong-khac-nhau}
Ba ô, ba nghĩa khác nhau, viết ra thành code:

```python title=readonly
tien_gui_xe = 0        # đã hỏi rồi: quán cho gửi miễn phí
ghi_chu = ""           # đã hỏi rồi: khách bảo khỏi ghi gì
tien_an_trua = None    # chưa hỏi được ai

print(tien_gui_xe)
print(ghi_chu)
print(tien_an_trua)
print(type(tien_an_trua))
```

```text title=readonly
0

None
<class 'NoneType'>
```

Bốn dòng ra, đọc từng dòng:

- `0` in ra một con số. Nhãn của nó là `int`, và nó tham gia được vào phép cộng
  như mọi con số khác.
- Dòng thứ hai **trắng**. `print("")` vẫn in một dòng, chỉ là dòng ấy không có
  ký tự nào. Đây là ô "đã nhập, nhập rỗng".
- `None` in ra đúng bốn chữ `None`. Để ý: máy **có thứ để in**. `None` không
  phải chỗ trống trong bộ nhớ — nó là một giá trị có mặt hẳn hoi, chỉ là giá trị
  ấy mang nghĩa "chưa có gì".
- `type(tien_an_trua)` cho `<class 'NoneType'>` — đúng cái nhãn vừa nói.

Gom lại thành ba câu ngắn, đáng nhớ hơn ba đoạn văn:

| Giá trị | Nghĩa của nó trên sổ |
|---|---|
| `0` | đã biết, và bằng không |
| `""` | đã nhập, và nhập rỗng |
| `None` | chưa có gì, chưa ai chạm vào |

Và đây là dòng còn thiếu mà bài 9 đã hẹn: `bool(None)` cũng cho `False`. Nghe
ngược với đoạn ngay trên, chỗ vừa nói `None` là một giá trị **có mặt** hẳn hoi.
Python vẫn xếp nó cùng phía với `0` và `""`, vì cả ba đều không mang **nội
dung** nào để làm việc. Có mặt là một chuyện, có nội dung là chuyện khác.
::::

::::predict{#doan-none-bang-cai-gi commitOnce}
Byte sắp chạy đoạn dưới để xem `None` đứng ở đâu so với hai ô trống kia. **Trước
khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
ghi_chu = None
print("Ghi chú:", ghi_chu)
print(ghi_chu == "")
print(ghi_chu == 0)
```

:::opt{correct}
Ba dòng: `Ghi chú: None`, rồi `False`, rồi `False`
:::

:::opt
Ba dòng: `Ghi chú:` (không có chữ nào phía sau), rồi `True`, rồi `False`
::why
Gần đúng ở chỗ bạn nghĩ "chưa có gì" thì in ra sẽ chẳng thấy gì — và trên tờ
giấy đúng là như vậy thật: ô để trống thì mắt không đọc được chữ nào.

Chỗ lệch nằm ở việc `None` là một **giá trị**, không phải một chỗ trống. `print`
luôn có thứ để in ra, và thứ nó in là bốn chữ `None`. Cái in ra một dòng trắng
là `""` — và vì hai thứ ấy hiện lên khác nhau, `ghi_chu == ""` trả lời `False`.
::
:::

:::opt
Ba dòng: `Ghi chú: None`, rồi `False`, rồi `True`
::why
Gần đúng ở chỗ bạn nối "chưa có gì" với "không có đồng nào". Trong tiếng Việt
hằng ngày hai câu ấy nghe gần như một, và trên máy tính bỏ túi thì màn hình để
trống với con số 0 cũng là một.

Chỗ lệch: với Python, `0` là một con số **đã được chốt**, còn `None` là chỗ chưa
có con số nào. Nên `None == 0` cho `False`. Và may là như vậy — nếu hai cái ấy
bằng nhau thì khoản gửi xe miễn phí lại lẫn vào khoản bạn chưa kịp hỏi giá, đúng
cái nhập nhằng mà bài này đang gỡ.
::
:::

:::opt
Máy in được dòng `Ghi chú: None` rồi dừng lại với `TypeError`, vì đem một chuỗi
so với `None`
::why
Gần đúng ở chỗ bạn nhớ rằng hai kiểu không đi cùng nhau thì máy dừng lại và nói
ra — với dấu `+` thì đúng như thế thật, `"45000" + 5000` nổ `TypeError`.

Chỗ lệch nằm ở câu hỏi mà dấu `==` đặt ra. Nó không bắt hai bên phải hợp kiểu
với nhau; nó chỉ hỏi *"hai thứ này có phải cùng một giá trị không"*. Câu ấy luôn
có câu trả lời, kể cả khi hai bên mang hai nhãn khác nhau — và lúc đó câu trả
lời là `False`.
::
:::
::::

::::explain{#none-la-thu-ban-tu-dat-vao}
`None` không tự mọc ra ở đâu cả. Nó là một giá trị bạn **gán** vào một cái tên,
y hệt cách bạn gán `0` hay gán `"cà phê"`:

```text
ghi_chu = None
```

Dòng ấy là một dòng gán đầy đủ. Nó nói: *cái tên `ghi_chu` từ giờ có mặt, và thứ
nó đang giữ là "chưa có gì".*

Nghe thì lạ — sao phải mất công gán một thứ rỗng? Vì đây chính là cách bạn **giữ
chỗ**. Khi ghi sổ, bạn kẻ đủ ba ô trước rồi mới đi hỏi từng ô; ô nào chưa hỏi
được thì để đó, nhưng cái ô ấy **đã có trên trang giấy**. Trong code cũng vậy:
khai `ghi_chu = None` ở đầu, rồi lát nữa mới điền nội dung thật vào, thì từ đầu
tới cuối chương trình cái tên `ghi_chu` lúc nào cũng chạm được.

Và lợi ích lớn nhất của việc đó là bạn phân biệt được **ba** trạng thái thay vì
hai: đã biết và bằng không (`0`), đã nhập và nhập rỗng (`""`), chưa có gì
(`None`). Ba câu chuyện, ba giá trị, không cái nào giả làm cái nào.
::::

::::code{#ke-du-ba-o}
Trưa nay Byte ghi một dòng sổ, và mỗi ô có một tình cảnh riêng:

- **Tiền gửi xe**: đã hỏi bà chủ, quán cho gửi miễn phí — biết chắc là không mất
  đồng nào.
- **Ghi chú**: đã hỏi khách, khách bảo khỏi ghi gì — nên ô này đã nhập, nhập
  rỗng.
- **Số người chia**: cả bàn còn đang gọi món, chưa ai chốt mấy người — chưa hỏi
  được.

Điền ba chỗ trống sao cho mỗi ô mang đúng một trong ba nghĩa trên.

```python title=starter
tien_gui_xe = ___
ghi_chu = ___
so_nguoi_chia = ___

print(f"Gửi xe: {tien_gui_xe}đ")
print(f"Ghi chú: {ghi_chu}")
print(f"Số người chia: {so_nguoi_chia}")
```

```python title=solution
tien_gui_xe = 0
ghi_chu = ""
so_nguoi_chia = None

print(f"Gửi xe: {tien_gui_xe}đ")
print(f"Ghi chú: {ghi_chu}")
print(f"Số người chia: {so_nguoi_chia}")
```

```python title=test
# Ba ô, ba nghĩa. Điền cùng một thứ vào cả ba là hỏng ít nhất một dòng.
# `type(tien_gui_xe) == type(0)` là để chặn `False`: `False == 0` cho True,
# nhưng nhãn của nó là bool chứ không phải int, và ô tiền thì phải là con số.
assert tien_gui_xe == 0, "đã hỏi bà chủ và quán cho gửi miễn phí, nên ô này là một câu trả lời: không mất đồng nào"
assert type(tien_gui_xe) == type(0), "ô tiền giữ một con số; câu đúng-sai cũng bằng 0 nhưng nó không phải một khoản chi"
assert ghi_chu == "", "đã hỏi khách và khách bảo khỏi ghi gì, nên ô ghi chú là ô đã nhập mà nhập rỗng"
assert type(so_nguoi_chia) == type(None), "cả bàn còn đang gọi món, chưa ai chốt mấy người — ô này mang nghĩa chưa có gì, không phải số 0 cũng không phải ô rỗng"
```

:::hints
- kind: attention
  body: Đọc lại bảng ba dòng ở phần ví dụ. Mỗi ô trong bài này khớp với đúng một dòng của bảng — và không có hai ô nào khớp cùng một dòng.
- kind: strategy
  body: Ô gửi xe đã có câu trả lời, câu trả lời ấy là một con số. Ô ghi chú đã được hỏi, chỉ là nội dung rỗng — thứ rỗng của chữ viết thế nào? Ô số người chia thì chưa có câu trả lời nào, và đó là chỗ duy nhất cần tới giá trị vừa học.
- kind: one-line
  body: "Lần lượt điền `0`, rồi `\"\"` (hai dấu nháy liền nhau), rồi `None`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Gửi xe: 0đ\nGhi chú: \nSố người chia: None\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba ô, ba nghĩa. Giờ mình không lẫn khoản miễn phí với khoản chưa hỏi nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ba ô đã khác nhau thật — `0`, `""` và `None` là ba giá trị riêng biệt, `==` xác
nhận điều đó. Nhưng thử mang cả ba trở lại chỗ chúng sinh ra rắc rối, tức là đặt
chúng vào `if`:

```text
bool(0)      →  False
bool("")     →  False
bool(None)   →  False
```

Cả ba vẫn nằm chung một phía. Nghĩa là `if ghi_chu:` — cách viết gọn mà bài
trước vừa cho bạn — vẫn gộp "chưa hỏi được" chung với "đã hỏi, khách không ghi
gì". Bạn có ba giá trị khác nhau trong tay mà câu hỏi thì chỉ chia được làm hai
phía.

Vậy muốn hỏi thẳng đúng một câu — *ô này đã có ai chạm bút vào chưa* — thì hỏi
bằng gì? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
