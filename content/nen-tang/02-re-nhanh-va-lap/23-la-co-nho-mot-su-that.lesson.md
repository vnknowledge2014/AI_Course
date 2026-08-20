---
id: nen-tang.re-nhanh-va-lap.la-co-nho-mot-su-that
title: Lá cờ nhớ một sự thật
summary: Một biến True/False đặt trước vòng, bật lên trong vòng, đọc lại sau vòng — cách báo cho phần còn lại của chương trình biết chuyện đã xảy ra.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.flag-variable]
requires: [core.boolean, ctrl.break-scope, ctrl.for-each, core.reassign]
concepts: [core.bien, core.dung-sai, ctrl.lap]
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
Chuyện xảy ra một lần lúc nãy. Mình cần chỗ nào đó ghi lại là nó đã xảy ra.
::::

::::explain{#keo-co-len-cot}
Câu hỏi bỏ ngỏ của bài trước: vòng trong đã biết là tìm thấy, vòng ngoài thì
không. Làm sao báo cho nó?

Ra bãi biển Sầm Sơn một sáng có sóng dữ. Người cứu hộ thấy sóng lúc sáu giờ,
thấy đúng một lần, ở đúng một chỗ. Nhưng cả ngày hôm đó, ai đi ngang cột cờ cũng
biết — vì lá cờ đỏ vẫn đang treo trên đó.

Lá cờ làm được việc mà tiếng hô không làm được. Tiếng hô chỉ tới tai người đang
đứng gần. Lá cờ thì **ở lại**: nó biến một chuyện xảy ra trong chốc lát thành
một sự thật mà bất cứ ai, ở bất cứ lúc nào sau đó, cũng đọc được.

Trong code, lá cờ ấy là một **biến**. Chỗ đặc biệt duy nhất của nó là thứ nó
giữ: không phải số tiền, không phải câu chữ, mà đúng một trong hai giá trị
`True` / `False` bạn đã gặp từ Realm 0. Người ta gọi nó là **biến cờ** — tiếng
Anh là *flag*, nghĩa đen cũng là lá cờ.

Và nó có một nhịp ba, luôn luôn đủ ba:

- **Trước vòng**: sinh ra lá cờ, hạ xuống — `False`, nghĩa là "chưa thấy gì".
- **Trong vòng**: gặp chuyện thì kéo lên — `True`.
- **Sau vòng**: đọc lá cờ để biết chuyện có xảy ra hay không.

Nhịp ba này bạn đã gặp rồi, ở bài cộng dồn: `tong = 0` trước vòng, cộng trong
vòng, in ra sau vòng. Lá cờ là cùng một hình dạng ấy, chỉ đổi thứ được nhớ từ
một con số thành một câu trả lời có–không.
::::

::::example{#co-mot-ngay-nao-vuot-khong}
Sổ chi tiêu một tuần, tính bằng nghìn đồng. Câu hỏi: **cả tuần có ngày nào tiêu
quá 200 không?**

```python title=readonly
chi_tieu = [80, 120, 95, 240, 60, 110, 130]
da_vuot = False

for tien in chi_tieu:
    if tien > 200:
        da_vuot = True
        break

if da_vuot:
    print("Tuần này có ngày tiêu quá 200 nghìn")
else:
    print("Cả tuần không ngày nào quá 200 nghìn")
```

```text title=readonly
Tuần này có ngày tiêu quá 200 nghìn
```

Ba chỗ, đúng nhịp ba:

- `da_vuot = False` nằm **trước** `for`, sát lề trái. Đây là lúc chưa dò ngày
  nào, nên câu trả lời trung thực là "chưa thấy".
- `da_vuot = True` nằm trong nhánh đúng của `if`, tức là chỉ chạy ở đúng cái
  ngày làm nên chuyện. `break` ngay sau đó vì đã đủ biết — một ngày vượt ngưỡng
  là đã trả lời xong câu hỏi.
- `if da_vuot:` nằm **sau** vòng, sát lề trái. Nó không hỏi về tiền nữa; nó hỏi
  về lá cờ.

Để ý dòng `if da_vuot:` viết trần, không có dấu so sánh nào. Được, vì `da_vuot`
đang giữ sẵn `True` hoặc `False` — đúng thứ mà `if` cần. Viết `if da_vuot == True:`
cũng chạy, nhưng đó là hỏi *"cái đúng này có đúng không"*, thừa một vòng.

Bây giờ mang lá cờ vào chỗ nó sinh ra để giải quyết: lịch tháng 4 tuần × 7 ngày,
dừng hẳn ở ngày đầu tiên vượt ngưỡng.

```python title=readonly
chi_tieu = [80, 120, 95, 240, 60, 110, 210,
            70, 90, 100, 85, 140, 75, 180,
            60, 95, 260, 120, 100, 70, 130,
            90, 110, 100, 230, 150, 95, 220]
da_vuot = False

for tuan in range(1, 5):
    print(f"Đang dò tuần {tuan}")
    for ngay_trong_tuan in range(1, 8):
        ngay = (tuan - 1) * 7 + ngay_trong_tuan
        if chi_tieu[ngay - 1] > 200:
            da_vuot = True
            break
    if da_vuot:
        break

if da_vuot:
    print("Tháng này có ngày tiêu quá 200 nghìn")
else:
    print("Cả tháng không ngày nào quá 200 nghìn")
```

```text title=readonly
Đang dò tuần 1
Tháng này có ngày tiêu quá 200 nghìn
```

Chỉ tuần 1 được dò. Bài trước, đoạn tương tự in ra đủ bốn dòng `Tuần`.

Nhìn khối `if da_vuot: break` mới thêm vào: nó **thụt vào bốn dấu cách**, tức là
nằm trong thân vòng tuần, ngay dưới vòng ngày. Vị trí ấy có nghĩa: *vòng ngày
vừa chạy xong — dù xong tự nhiên hay bị cắt — giờ xem lá cờ đã.* Và vì nó nằm
trong thân vòng tuần, `break` của nó nói về vòng tuần. Đúng luật bài trước, chưa
có ngoại lệ nào.

Lá cờ chính là thứ nối hai vòng lại: vòng trong kéo cờ lên, vòng ngoài nhìn cờ.
::::

::::predict{#doan-la-co-con-treo-khong commitOnce}
Byte viết đoạn dưới để trả lời "cả tuần có ngày nào tiêu quá 200 không". Sổ có
ngày thứ hai tiêu 240 — quá ngưỡng. **Trước khi bấm chạy**, bạn đoán máy in ra
gì?

```python title=readonly
chi_tieu = [80, 240, 95, 110]
da_vuot = False

for tien in chi_tieu:
    if tien > 200:
        da_vuot = True
    else:
        da_vuot = False

print(da_vuot)
```

:::opt{correct}
False
:::

:::opt
True
::why
Gần đúng ở chỗ bạn đọc đúng ý định của người viết, và đúng cả chuyện xảy ra ở
lượt thứ hai: `240 > 200` là đúng, nên `da_vuot` được kéo lên `True` thật. Nếu
in ra ngay lúc đó, bạn đã thắng.

Chỗ lệch nằm ở hai lượt tiếp theo. Vòng lặp không dừng lại sau lượt hai — nó
chạy tiếp với 95 rồi 110. Cả hai đều không quá 200, nên máy vào nhánh `else` và
chạy `da_vuot = False`, dán đè lên lá cờ vừa kéo lên. Lúc `print` chạy, thứ còn
lại trong `da_vuot` là câu trả lời của **ngày cuối cùng**, không phải của cả
tuần.

Đây là lý do một biến cờ chỉ được kéo **lên**, không bao giờ có dòng hạ xuống
trong thân vòng.
::
:::

:::opt
Máy in bốn dòng: False, True, False, False — mỗi lượt một dòng
::why
Gần đúng ở chỗ bạn theo dõi rất sát giá trị của `da_vuot` qua từng lượt, và bốn
giá trị bạn liệt kê đúng từng cái một. Đó là cách đọc code mà người viết chương
trình giỏi vẫn làm trong đầu.

Chỗ lệch nằm ở chỗ đứng của `print`. Nó viết sát lề trái, ngoài thân vòng, nên
nó không thuộc về lượt nào cả — nó chạy đúng một lần, sau khi vòng lặp đã xong
hẳn. Muốn thấy bốn dòng thì `print` phải lùi vào trong thân vòng.
::
:::

:::opt
240
::why
Gần đúng ở chỗ bạn nhớ rằng đâu đó trong đoạn này có con số làm nên chuyện, và
240 đúng là con số ấy.

Chỗ lệch nằm ở thứ `da_vuot` được giao giữ. Không có dòng nào viết
`da_vuot = tien` cả; hai dòng gán vào nó chỉ đưa `True` hoặc `False`. Một biến
cờ cố tình chỉ giữ có–không, không giữ con số — và đó vừa là sức mạnh vừa là
giới hạn của nó.
::
:::
::::

::::explain{#la-co-chi-keo-len}
Rút ra ba điều từ cái bẫy vừa rồi:

- **Cờ chỉ đi một chiều.** Đặt `False` một lần trước vòng, rồi trong vòng chỉ có
  dòng gán `True`. Thân vòng không bao giờ chứa dòng hạ cờ xuống. Nhánh `else`
  ở bài trên trông rất hợp lý — mỗi lượt trả lời cho ngày của lượt đó — nhưng
  câu hỏi bạn đang hỏi là câu hỏi về **cả tuần**, không phải về một ngày.
- **Cờ phải sinh ra trước vòng.** Viết `da_vuot = False` trong thân vòng thì mỗi
  lượt lại hạ cờ một lần, y hệt cái bẫy trên. Còn không viết nó ở đâu cả thì
  dòng `if da_vuot:` sau vòng gặp `NameError` — cái tên chưa từng tồn tại.
- **Đặt tên cho lá cờ nghe ra một câu có–không.** `da_vuot`, `co_khach_moi`,
  `da_ghi_so` — đọc lên là thành một câu trả lời được bằng có hoặc không. Tên
  kiểu `kiem_tra` hay `trang_thai` thì lúc đọc `if kiem_tra:` bạn không biết
  đang hỏi gì.

Một chuyện nữa đáng nhớ, vì nó là lý do lá cờ tồn tại: `da_vuot` được sinh ra
sát lề trái, nên nó không thuộc về vòng lặp nào. Vòng lặp kết thúc thì các cái
tên sát lề trái vẫn còn nguyên. Vòng trong chạm được vào nó, vòng ngoài chạm
được vào nó, và mọi dòng nằm sau cả hai vòng cũng vậy.
::::

::::code{#keo-co-len-khi-gap}
Byte dò sổ chi tiêu tuần này để trả lời một câu duy nhất: **có ngày nào tiêu quá
200 nghìn không?**

Lá cờ đã được sinh ra trước vòng, và khối `if/else` sau vòng đã sẵn sàng đọc nó.
Còn thiếu đúng dòng kéo cờ lên khi gặp ngày vượt ngưỡng.

```python title=starter
chi_tieu = [80, 120, 260, 60, 110, 130, 70]
da_vuot = False

for tien in chi_tieu:
    if tien > 200:
        ___
        break

if da_vuot:
    print("Tuần này có ngày tiêu quá 200 nghìn")
else:
    print("Cả tuần không ngày nào quá 200 nghìn")
```

```python title=solution
chi_tieu = [80, 120, 260, 60, 110, 130, 70]
da_vuot = False

for tien in chi_tieu:
    if tien > 200:
        da_vuot = True
        break

if da_vuot:
    print("Tuần này có ngày tiêu quá 200 nghìn")
else:
    print("Cả tuần không ngày nào quá 200 nghìn")
```

```python title=test
# Ngày thứ ba tiêu 260 nghìn, nên sau vòng lá cờ phải đang được kéo lên.
# Thiếu dòng kéo cờ thì `da_vuot` vẫn là False và máy nói ngược lại sự thật.
assert da_vuot
```

:::hints
- kind: attention
  body: Chỗ trống nằm trong nhánh đúng của `if`, ngay trên dòng `break`. Tới được dòng đó nghĩa là máy vừa gặp một ngày quá 200 — và bạn cần ghi lại sự thật ấy vào một chỗ mà dòng `if da_vuot:` bên dưới đọc được.
- kind: strategy
  body: Cái tên cần ghi vào đã có sẵn ở dòng thứ hai của chương trình, đang giữ `False`. Việc của bạn là dán lại cái tên ấy lên giá trị ngược lại — giá trị nghĩa là "đúng, đã thấy".
- kind: one-line
  body: "Viết `da_vuot = True` vào chỗ trống, thụt vào đúng bằng dòng `break` ngay dưới nó."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Tuần này có ngày tiêu quá 200 nghìn
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cờ đã kéo lên từ ngày thứ ba. Vòng lặp xong lâu rồi mà nó vẫn treo đó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bà chủ đọc dòng chữ máy in ra — *"Tuần này có ngày tiêu quá 200 nghìn"* — rồi
hỏi lại ngay câu mà ai cũng sẽ hỏi:

> Ngày nào?

Lá cờ chịu. Nó chỉ có hai giá trị, và cả hai đều không phải một con số ngày.
Lúc máy chạy tới ngày thứ ba, nó **đã cầm** con số 3 trong tay — rồi nó vứt đi,
chỉ giữ lại một chữ `True`.

Chỗ để ghi thì vẫn là chỗ ấy: một cái tên sinh ra trước vòng, được chạm trong
vòng, đọc lại sau vòng. Chỉ có thứ ghi vào là phải đổi.

Vậy thay vì ghi `True`, ghi gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
