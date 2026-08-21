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
estimatedMinutes: 14
teaches: [core.flag-variable]
requires: [core.boolean, ctrl.break-scope, ctrl.nested-loop, ctrl.for-each, ctrl.for-range, core.reassign, ctrl.block-indent, core.list-index, core.fstring]
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

Đoạn này có **hai** dòng `break`, nên câu hỏi bật ra ngay: cái bên trong còn để
làm gì? Thử bỏ nó đi mà xem — đoạn code vẫn in ra đúng hai dòng ấy. Khác nhau ở
phần không nhìn thấy: không có nó, vòng ngày dò nốt ngày 5, 6, 7 của tuần 1 dù
câu trả lời đã có từ ngày 4. `break` bên trong là để **thôi ngay khi đã đủ
biết** — đúng việc bài "Dừng ngay khi đã đủ biết" đã dạy. `break` bên ngoài là
để **báo tin ra tầng trên**. Hai dòng cùng một từ khoá, làm hai việc khác nhau.

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
Bạn đọc trúng ý định của người viết, và trúng cả chuyện xảy ra ở lượt thứ hai:
`240 > 200` là đúng, nên `da_vuot` được kéo lên `True` thật. Nếu `print` chạy
ngay lúc đó, bạn đã thắng.

Chỗ lệch nằm ở hai lượt tiếp theo. Vòng lặp không dừng lại sau lượt hai — nó
chạy tiếp với 95 rồi 110. Cả hai đều không quá 200, nên máy vào nhánh `else` và
chạy `da_vuot = False`, dán đè lên lá cờ vừa kéo lên. Lúc `print` chạy, thứ còn
lại trong `da_vuot` là câu trả lời của **ngày cuối cùng**, không phải của cả
tuần.

Với câu hỏi *có ngày nào… không*, đây chính là lý do lá cờ chỉ được kéo **lên**:
thân vòng không có chỗ cho một dòng hạ nó xuống.
::
:::

:::opt
Máy in bốn dòng: False, True, False, False — mỗi lượt một dòng
::why
Bốn giá trị bạn liệt kê đúng từng cái một: `da_vuot` thật sự lần lượt mang
`False`, `True`, `False`, `False` qua bốn lượt. Dò được như vậy là đang đọc code
đúng cái cách người viết chương trình vẫn đọc trong đầu.

Chỗ lệch nằm ở chỗ đứng của `print`. Nó viết sát lề trái, ngoài thân vòng, nên
nó không thuộc về lượt nào cả — nó chạy đúng một lần, sau khi vòng lặp đã xong
hẳn. Muốn thấy bốn dòng thì `print` phải lùi vào trong thân vòng.
::
:::

:::opt
Máy báo lỗi, vì `da_vuot` bị gán tới bốn lần trong một vòng lặp
::why
Bạn đếm đúng một chuyện có thật: cùng một cái tên bị dán lại bốn lần, mỗi lượt
một lần. Đoạn code này quả thật ghi đè liên tục lên `da_vuot`.

Chỗ lệch nằm ở việc Python coi chuyện ghi đè ấy là bình thường. Một cái tên
không phải cái ô chỉ điền được một lần — nó là mảnh giấy dán, gỡ ra dán sang giá
trị khác bao nhiêu lần cũng được, đúng như `tong` ở bài cộng dồn đã bị dán lại
sau mỗi lượt. Máy không kêu một tiếng nào; nó lặng lẽ nhận lần dán cuối cùng —
và đó mới là chỗ cái bẫy này nằm.
::
:::
::::

::::explain{#la-co-chi-keo-len}
Rút ra ba điều từ cái bẫy vừa rồi:

- **Với câu hỏi *có ngày nào… không*, cờ chỉ đi một chiều.** Đặt `False` một lần
  trước vòng, rồi trong vòng chỉ có dòng kéo lên `True`. Hạ cờ trong thân vòng
  là dán đè lên chuyện đã xảy ra. Nhánh `else` ở bài trên trông rất hợp lý — mỗi
  lượt trả lời cho ngày của lượt đó — nhưng câu hỏi bạn đang hỏi là câu hỏi về
  **cả tuần**, không phải về một ngày.
- **Cờ phải sinh ra trước vòng.** Viết `da_vuot = False` trong thân vòng thì mỗi
  lượt lại hạ cờ một lần, y hệt cái bẫy trên. Còn bỏ hẳn dòng ấy đi thì bài vẫn
  chạy — nhưng chỉ khi may mắn có ít nhất một ngày vượt ngưỡng, vì lúc đó dòng
  `da_vuot = True` kịp tạo ra cái tên. Đưa vào một tuần tiêu dè, thân `if` không
  chạy lần nào, cái tên chưa từng tồn tại, và `if da_vuot:` sau vòng gặp
  `NameError`.
- **Đặt tên cho lá cờ nghe ra một câu có–không.** `da_vuot`, `co_khach_moi`,
  `da_ghi_so` — đọc lên là thành một câu trả lời được bằng có hoặc không. Tên
  kiểu `kiem_tra` hay `trang_thai` thì lúc đọc `if kiem_tra:` bạn không biết
  đang hỏi gì.

Nói cho hết một chuyện mà người mới hay hiểu ngược. Ở Python, một cái tên đã
được tạo ra thì còn đó tới hết chương trình — vòng lặp không nhốt cái tên nào
lại, tên sinh trong thân vòng sống y như tên sinh ngoài vòng. Vậy nên lý do
`da_vuot = False` phải nằm trước vòng không phải là chuyện thụt lề, mà đúng là
chuyện vừa nói ở gạch đầu dòng thứ hai: để dù thân `if` không lần nào chạy, dòng
đọc cờ sau vòng vẫn có thứ để đọc.

Đã sinh ra rồi thì `da_vuot` chạm được từ mọi phía: vòng trong ghi vào nó, vòng
ngoài đọc nó, và mọi dòng nằm sau cả hai vòng cũng đọc được.
::::

::::code{#bao-tin-ra-vong-tuan}
Sổ chi tiêu tháng trước của Byte, 28 ngày xếp thành 4 tuần. Tháng này tiêu dè
hơn: mãi tới **ngày 17** mới có ngày đầu tiên quá 200 nghìn. Byte muốn dò tới đó
là dừng hẳn, không mở nốt tuần 4.

Trong vòng ngày, lá cờ đã được kéo lên và `break` đã cắt vòng ngày. Còn thiếu
đúng chỗ **báo tin ra vòng tuần** — không có nó, vòng tuần cứ mở tiếp tuần sau
như chưa có chuyện gì.

Chỗ trống là **hai dòng**: một dòng đọc lá cờ, và dưới nó một dòng cắt vòng
tuần.

```python title=starter
chi_tieu = [80, 120, 95, 140, 60, 110, 180,
            70, 90, 100, 85, 140, 75, 190,
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
    ___

if da_vuot:
    print("Tháng này có ngày tiêu quá 200 nghìn")
else:
    print("Cả tháng không ngày nào quá 200 nghìn")
```

```python title=solution
chi_tieu = [80, 120, 95, 140, 60, 110, 180,
            70, 90, 100, 85, 140, 75, 190,
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

```python title=test
# Tuần 1 và tuần 2 không ngày nào quá 200; ngày 17 (tuần 3) là ngày đầu tiên.
# Đặt đúng tầng thì vòng tuần dừng ngay sau tuần 3, nên cái tên `tuan` — vẫn
# còn đó sau khi vòng lặp xong — đang giữ số 3.
# Quên hẳn hai dòng ấy, hoặc thụt chúng vào thân vòng ngày, thì vòng tuần chạy
# trọn bốn lượt và `tuan` là 4. Viết `break` trần không kèm dòng đọc cờ thì nó
# cắt ngay từ tuần 1, lúc `da_vuot` còn chưa được kéo lên.
assert da_vuot
assert tuan == 3
```

:::hints
- kind: attention
  body: Chỗ trống nằm sát dưới vòng ngày, thụt vào bốn dấu cách — tức là trong thân vòng tuần. Máy chạy tới đó khi vòng ngày vừa xong một tuần, dù xong tự nhiên hay bị `break` cắt. Lúc ấy thứ duy nhất cho bạn biết tuần vừa rồi có chuyện hay không là lá cờ.
- kind: strategy
  body: Dòng trên hỏi lá cờ, dòng dưới làm việc khi cờ đang treo. Nhớ luật bài trước — lệnh cắt ngang luôn nói về vòng gần nhất bao quanh nó — nên muốn nó cắt vòng tuần thì cả hai dòng phải nằm trong thân vòng tuần, chứ không phải trong thân vòng ngày.
- kind: one-line
  body: "Viết `if da_vuot:` vào chỗ trống, rồi xuống dòng viết `break` thụt vào tám dấu cách."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Tháng này có ngày tiêu quá 200 nghìn
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cờ kéo lên ở tuần 3, vòng tuần nhìn thấy và dừng luôn. Tuần 4 mình chưa mở tới.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bà chủ đọc dòng chữ máy in ra — *"Tháng này có ngày tiêu quá 200 nghìn"* — rồi
hỏi lại ngay câu mà ai cũng sẽ hỏi:

> Ngày nào?

Lá cờ chịu. Nó chỉ có hai giá trị, và cả hai đều không phải một con số ngày.
Lúc máy dừng lại, `ngay` trong tay nó đang là 17 — rồi nó vứt con số ấy đi, chỉ
giữ lại một chữ `True`.

Chỗ để ghi thì vẫn là chỗ ấy: một cái tên sinh ra trước vòng, được chạm trong
vòng, đọc lại sau vòng. Chỉ có thứ ghi vào là phải đổi.

Vậy thay vì ghi `True`, ghi gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
