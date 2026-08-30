---
id: onboarding.ra-lenh-cho-byte.mot-cho-chua-nhieu-gia-tri
title: Một chỗ chứa nhiều giá trị
summary: Một cái tên giữ cả một dãy giá trị có thứ tự, viết giữa hai dấu ngoặc vuông.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [core.list]
requires: [core.variable, ctrl.loop-variable]
concepts: [core.danh-sach]
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
Từ đầu tới giờ một cái tên giữ một giá trị. Hôm nay nó giữ cả tấm bảng thực đơn.
::::

::::explain{#ba-cai-ten-roi-rac}
Câu hỏi bỏ ngỏ của bài trước: ba món "Phở tái", "Phở chín", "Phở nạm" nằm ở đâu
thì vòng lặp đi qua được?

Cách bạn đã biết là đặt cho mỗi món một cái tên:

```python title=readonly
mon_1 = "Phở tái"
mon_2 = "Phở chín"
mon_3 = "Phở nạm"
```

Ba cái tên rời rạc. Ổn khi có ba món. Quán thêm món thứ tư thì thêm một dòng, và
mọi chỗ nào trong chương trình có nhắc tới thực đơn đều phải sửa theo.

Chỗ kẹt lớn hơn nằm ở vòng lặp. `range` phát ra 0, 1, 2 — chứ không phát ra
`mon_1`, `mon_2`, `mon_3`. Ba cái tên này không có một chỗ đứng chung nào để máy
đi qua lần lượt. Chúng chỉ tình cờ trông giống nhau, với máy thì chúng chẳng
liên quan gì tới nhau cả.

Nhìn lên tường quán thì thấy cách làm khác. Thực đơn ở đó không phải ba tấm
bảng. Nó là **một** tấm bảng, ba dòng, xếp theo thứ tự từ trên xuống. Ai muốn
xem thực đơn thì nhìn đúng một chỗ.

Python có đúng thứ đó.
::::

::::example{#tam-bang-thuc-don}
```python title=readonly
thuc_don = ["Phở tái", "Phở chín", "Phở nạm"]
print(thuc_don)
```

Máy in ra:

```text title=readonly
['Phở tái', 'Phở chín', 'Phở nạm']
```

Đọc dòng đầu từ trái sang phải:

- `thuc_don` — một cái tên, dán bằng dấu `=`, y hệt mọi cái tên từ trước tới giờ.
- Dấu ngoặc vuông `[` mở chỗ chứa, dấu `]` đóng lại.
- Ba giá trị nằm bên trong, ngăn nhau bằng dấu phẩy. Mỗi giá trị vẫn cần đủ hai dấu nháy của nó, vì mỗi món vẫn là một câu chữ.

Một chi tiết nhỏ ở dòng máy in ra: máy dùng dấu nháy **đơn** `'` chứ không dùng
nháy kép `"` như bạn đã gõ. Với máy hai loại nháy này cùng nghĩa; nó chỉ chọn
sẵn một loại để in cho gọn.

Và thứ tự bạn viết là thứ tự máy giữ. "Phở tái" viết trước thì nó đứng trước,
lần chạy nào cũng vậy, cho tới khi chính bạn đổi.
::::

::::predict{#print-mot-hay-hai-dong commitOnce}
Quán ghi lại những bàn đã đặt trước. **Trước khi bấm chạy**, bạn đoán máy in ra
gì?

```python title=readonly
ban_da_dat = ["Bàn 2", "Bàn 5"]
print(ban_da_dat)
```

:::opt{correct}
Một dòng, gồm cả hai bàn nằm trong dấu ngoặc vuông
:::

:::opt
Hai dòng: Bàn 2, rồi Bàn 5
::why
Gần đúng ở chỗ bạn đang nghĩ tới đúng cái việc mà người ta hay làm với một chỗ
chứa nhiều thứ: đọc lần lượt từng cái. Đó thật sự là bước kế tiếp của mạch này.

Chỗ lệch là ở đây chưa có gì bảo máy đọc lần lượt — không có `for` nào cả.
`print` nhận đúng **một** thứ, và một thứ đó là cả cái chỗ chứa. Nên nó in ra
một dòng, kèm luôn hai dấu ngoặc vuông để bạn nhìn là biết đây là một chỗ chứa
chứ không phải một câu chữ.
::
:::

:::opt
Chữ ban_da_dat
::why
Gần đúng ở chỗ bạn nhớ rất chắc luật của dấu nháy: cái gì trong nháy thì máy đọc
nguyên văn.

Chỗ lệch là `ban_da_dat` ở đây **không** có nháy. Không nháy thì nó là một cái
tên, máy đi tìm và in ra thứ cái tên ấy đang giữ — đúng như ở bài in ra thứ mà
cái tên đang giữ. Lần này thứ nó đang giữ chỉ tình cờ to hơn mọi lần trước.
::
:::

:::opt
Máy báo lỗi, vì một cái tên không giữ được hai giá trị cùng lúc
::why
Gần đúng ở một luật vẫn còn nguyên giá trị, và bạn giữ nó rất đúng: một cái tên
giữ đúng một giá trị.

Luật ấy không hề bị phá ở đây. Chỗ lệch nằm chỗ khác: thứ mà `ban_da_dat` đang
giữ **là một danh sách**, và bản thân danh sách là *một* giá trị — dù bên trong
nó có hai món, hai chục món hay hai trăm món. Cái tên vẫn chỉ trỏ vào một thứ:
tấm bảng.
::
:::
::::

::::explain{#danh-sach}
Chỗ chứa này có tên: **danh sách**. Tiếng Anh là `list`, và đó là từ bạn cần khi
đi tra cứu.

Vài điều gọn về nó:

- **Chứa được số, không riêng gì chữ.** `gia_pho = [45000, 50000, 60000]` là một danh sách hợp lệ.
- **Danh sách rỗng viết là `[]`** — một tấm bảng chưa ghi gì lên.
- **Nó là một kiểu riêng.** Đem `type(thuc_don)` ra hỏi như ở bài mỗi giá trị có một kiểu, máy đáp `<class 'list'>`. Danh sách đứng cạnh chữ và số như một loại giá trị nữa.

> Dễ nhầm: ngoặc vuông `[ ]` không phải ngoặc tròn `( )`. Ngoặc tròn là chỗ
> đưa thứ gì đó vào cho một việc — `print(...)`, `range(...)`. Ngoặc vuông là
> chỗ dựng nên một danh sách.
>
> Và gõ nhầm hai loại ngoặc ấy hỏng theo hai kiểu khác hẳn nhau, **không kiểu
> nào báo lỗi trước khi chạy**:
>
> - Gõ ngoặc tròn thay ngoặc vuông thì máy **không kêu một tiếng nào**. Nó
>   dựng ra một thứ khác, chạy trơn tru, và bạn chỉ biết khi in ra thấy `( )`
>   thay vì `[ ]`.
> - Gõ ngoặc vuông thay ngoặc tròn — `print["Phở tái"]` — thì máy chạy tới
>   đúng dòng ấy mới dừng, và nó nói `TypeError`: bạn đang xin một ô trong
>   `print`, mà `print` không phải chỗ có ô để xin.
::::

::::code{#viet-thuc-don}
Quán mở thêm ca sáng và cần thực đơn nằm gọn trong một cái tên.

Hãy cho `thuc_don` giữ một danh sách đúng ba món, đúng thứ tự này: Phở tái, Phở
chín, Phở nạm.

```python title=starter
thuc_don = ___
print(thuc_don)
```

```python title=solution
thuc_don = ["Phở tái", "Phở chín", "Phở nạm"]
print(thuc_don)
```

```python title=test
# Chấm bằng OUTPUT: cả ba món hiện ra trên một dòng, đúng thứ tự đã cho.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm bên phải dấu `=`. Chỗ đó cần **một** giá trị, và giá trị ấy là cả tấm bảng ba món.
- kind: strategy
  body: Mở bằng một dấu ngoặc vuông, đóng bằng một dấu ngoặc vuông. Bên trong là ba câu chữ, mỗi câu đủ hai dấu nháy của nó, ngăn nhau bằng dấu phẩy.
- kind: one-line
  body: 'Viết `["Phở tái", "Phở chín", "Phở nạm"]` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: ['Phở tái', 'Phở chín', 'Phở nạm']
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cả thực đơn nằm trong một cái tên. Thêm món thì sửa đúng một chỗ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Tấm bảng đã nằm gọn trong một cái tên. Nhưng khách vào quán không gọi cả tấm
bảng — khách gọi **một** món.

`print(thuc_don)` in ra cả ba, kèm dấu ngoặc vuông, trông không giống thứ mang
ra cho khách xem. Muốn lấy riêng "Phở tái" — món đứng đầu bảng — thì nói với máy
thế nào?

Và nếu bạn bảo máy đưa "món số 1", bạn nghĩ nó đưa ra món nào?

Bài sau trả lời, và câu trả lời sẽ giải thích luôn vì sao `range(3)` lại đếm từ
0 chứ không đếm từ 1.
::::

::::checkpoint{mastery=0.8}
::::
