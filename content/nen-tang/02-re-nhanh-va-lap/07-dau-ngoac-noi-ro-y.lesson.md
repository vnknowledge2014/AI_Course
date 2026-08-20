---
id: nen-tang.re-nhanh-va-lap.dau-ngoac-noi-ro-y
title: Dấu ngoặc nói rõ ý bạn
summary: Cặp ngoặc gom mấy chữ thành một cụm và bắt máy tính cụm ấy trước — đè lên thứ tự mặc định.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [logic.parentheses]
requires: [logic.precedence, logic.not, logic.and]
concepts: [logic.phep-logic, core.dung-sai]
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
Mình không tự đoán chỗ nào thuộc về chỗ nào. Bạn khoanh lại thì mình theo.
::::

::::explain{#cau-noi-bi-cat-doi}
Bài trước cho bạn bảng thứ tự máy luôn theo: số học → so sánh → `not` → `and`
→ `or`. Biết bảng ấy rồi thì đọc được mọi điều kiện người khác viết.

Nhưng nó để lại một chuyện chưa xong. Sáng nay Byte tính đi ăn phở. Đi được khi
**vừa đủ tiền vừa kịp giờ** — thiếu một trong hai là thôi. Byte muốn máy nhắc
một câu vào đúng những sáng phải nhịn:

> Không phải là vừa đủ tiền vừa kịp giờ.

Trong câu tiếng Việt đó, hai chữ *không phải* trùm lên **cả cụm** phía sau. Bây
giờ viết thẳng nó ra Python:

```python
not du_tien and kip_gio
```

Máy không đọc như bạn nói. Theo bảng thứ tự của bài trước, `not` được tính
**trước** `and`, nghĩa là `not` chỉ ôm đúng cái tên đứng sát ngay sau nó —
`du_tien` — rồi mới tới lượt `and`. Câu máy đọc được là:

> Không đủ tiền, **mà lại** kịp giờ.

Hai câu khác hẳn nhau. Câu của bạn nói *thiếu ít nhất một trong hai*. Câu máy
đọc nói *chắc chắn thiếu tiền, và chắc chắn đủ giờ*.

Chỗ hụt không nằm ở thứ tự — thứ tự thì máy làm đúng như đã hứa. Chỗ hụt là bạn
chưa có cách nói *"mấy chữ này gom lại thành một cụm"*.

Cách nói ấy bạn đã dùng từ hồi học toán ở trường: `2 + 3 * 4` ra 14, còn
`(2 + 3) * 4` ra 20. Dấu ngoặc gom `2 + 3` thành một món, và món trong ngoặc
được tính trước.

Python dùng lại đúng dấu ngoặc đó, và dùng cho mọi loại giá trị chứ không riêng
số.
::::

::::example{#hai-cach-doc-mot-dong}
Sáng nay Byte đủ tiền, nhưng dậy muộn nên không kịp giờ:

```python title=readonly
du_tien = True
kip_gio = False

print(not du_tien and kip_gio)
print(not (du_tien and kip_gio))
```

Máy in ra:

```text
False
True
```

Hai dòng chỉ khác nhau một cặp ngoặc, và chúng cho hai câu trả lời ngược nhau.
Đi lại từng dòng:

- Dòng thứ tư: `not` ôm `du_tien` trước, `not True` cho `False`. Còn lại
  `False and kip_gio`, tức `False and False`, cho `False`.
- Dòng thứ năm: cụm trong ngoặc được tính trước, `True and False` cho `False`.
  Rồi `not` ôm cả kết quả ấy, `not False` cho `True`.

`True` ở dòng cuối chính là câu bạn muốn nói: sáng nay **không phải** vừa đủ
tiền vừa kịp giờ, nên đúng là phải nhịn phở thật.

Câu ngắn để nhớ: dấu ngoặc **đè lên** bảng thứ tự. Máy vẫn giữ nguyên bảng ấy ở
mọi chỗ khác; riêng cụm nằm trong ngoặc thì được tính trước tất cả.
::::

::::predict{#doan-sang-nay commitOnce}
Sáng nay thêm một chi tiết: có bạn rủ đi cùng. Byte viết điều kiện có ngoặc như
dưới đây. **Trước khi bấm chạy**, bạn đoán màn hình hiện ra dòng nào?

```python title=readonly
du_tien = False
kip_gio = True
ban_ru = True

if du_tien and (kip_gio or ban_ru):
    print("Đi ăn phở")
else:
    print("Sáng nay nhịn phở")
```

:::opt{correct}
Sáng nay nhịn phở
:::

:::opt
Đi ăn phở — vì `ban_ru` đúng, mà `or` chỉ cần một vế đúng là đủ
::why
Gần đúng ở chỗ bạn nhớ chính xác luật của `or`: một vế đúng là cả phép `or` cho
`True`. Ở đây `kip_gio or ban_ru` đúng là ra `True` thật.

Chỗ lệch nằm ở chuyện phép nào là phép **cuối cùng**. Cặp ngoặc đã nhốt `or` vào
bên trong, nên nó chỉ làm ra một vế thôi. Phép quyết định câu trả lời là `and` ở
ngoài, mà `and` thì đòi **cả hai** vế cùng đúng. Vế `du_tien` đang là `False`,
nên cả điều kiện là `False`.
::
:::

:::opt
Đi ăn phở — vì cụm trong ngoặc cho `True`, nên cả điều kiện thành `True`
::why
Gần đúng ở chỗ bạn tính cụm trong ngoặc trước và tính ra `True` — đúng thứ tự
bài này vừa nói, và đó là bước khó nhất.

Chỗ lệch: `True` ấy chưa phải kết quả cuối, nó mới là **một vế** của phép `and`
còn đang dở. Dừng lại ở đó cũng giống như tính `(2 + 3) * 4` rồi trả lời 5.
::
:::

:::opt
Máy báo lỗi: dấu ngoặc là của `print` và `input`, không đặt trong điều kiện được
::why
Gần đúng ở chỗ mọi dấu ngoặc bạn gặp cho tới hôm nay đều dính liền sau một cái
tên: `print(...)`, `input(...)`, `int(...)`. Thấy chúng đi cùng nhau suốt mấy
chục bài thì nghĩ chúng thuộc về nhau là chuyện tự nhiên.

Chỗ lệch: dấu ngoặc làm hai việc, và máy phân biệt bằng **chỗ nó đứng**. Đứng
sát ngay sau một cái tên thì nó nghĩa là *"gọi việc này"*. Đứng một mình, như
trong dòng `if` ở đây, nó nghĩa là *"gom cụm này lại"*. Cả hai đều hợp lệ.
::
:::
::::

::::explain{#ngoac-de-len-thu-tu}
Ba điều dùng được ngay từ hôm nay:

- **Trong ngoặc tính trước.** Máy làm xong cụm bên trong, thu về đúng một giá
  trị, rồi mới đem giá trị ấy đi tính tiếp với phần còn lại.
- **Ngoặc lồng trong ngoặc thì tính từ trong ra.** Với
  `not ((du_tien or ban_ru) and kip_gio)`, máy làm `du_tien or ban_ru` trước,
  rồi `and kip_gio`, rồi mới `not`.
- **Ngoặc thừa không sai.** `(tien > 200000) and (da_ghi_so)` cho kết quả y hệt
  khi bỏ hai cặp ngoặc đi, vì so sánh vốn đã được tính trước `and`. Chúng không
  đổi kết quả, chỉ đỡ cho người đọc khỏi phải nhớ bảng thứ tự. Viết ngoặc cho rõ
  ý là chuyện nên làm.

> Dễ nhầm: `not a and b` và `not (a and b)` là hai câu khác nhau, nhưng chúng
> cho cùng kết quả trong một nửa số trường hợp — đủ để bạn chạy thử đúng một lần
> thấy khớp rồi tin nhầm. Hễ trong đầu bạn có chữ *"không phải là **cả** hai
> điều này"* thì cặp ngoặc là bắt buộc.
::::

::::code{#noi-dung-y-ban}
Sáng nay Byte đủ tiền, nhưng dậy muộn nên không kịp giờ.

Hãy viết điều kiện sao cho câu nhắc chỉ hiện ra vào những sáng **không phải là
vừa đủ tiền vừa kịp giờ**.

```python title=starter
du_tien = True
kip_gio = False

if ___:
    print("Sáng nay nhịn phở")
```

```python title=solution
du_tien = True
kip_gio = False

if not (du_tien and kip_gio):
    print("Sáng nay nhịn phở")
```

```python title=test
# Chấm bằng OUTPUT: sáng nay thiếu giờ, nên câu nhắc phải hiện ra.
# Viết `not du_tien and kip_gio` thì màn hình trống trơn — đúng cái bẫy
# mà bài này nói tới.
pass
```

:::hints
- kind: attention
  body: Ý bạn có hai phần: một cụm "vừa… vừa…" gồm hai điều kiện, và chữ "không phải" trùm lên cả cụm ấy. Trong dòng code, cái gì đánh dấu ranh giới của cả cụm?
- kind: strategy
  body: Viết cụm "vừa đủ tiền vừa kịp giờ" ra trước bằng `and`, khoanh nó lại, rồi đặt `not` ở ngoài cùng. Đặt `not` sát vào một cái tên là nó chỉ lật đúng cái tên đó.
- kind: one-line
  body: "Viết `not (du_tien and kip_gio)` vào chỗ trống, giữ nguyên dấu hai chấm cuối dòng."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: Sáng nay nhịn phở
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cặp ngoặc đó không phải để cho đẹp. Nó là chỗ bạn chỉ ranh giới cho mình.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Điều kiện của bạn giờ nói được đúng ý, dù ý có nhiều tầng tới đâu. Nhưng hãy
nhìn lại chỗ con số **đi vào** chương trình.

Từ Realm 0, cách hỏi khách là `input`. Máy hỏi **đúng một lần**: khách gõ nhầm
một chữ, hay lỡ bấm Enter suông, thì cái tên nhận về một câu rỗng và chương
trình vẫn đi tiếp với nó.

Muốn hỏi lại cho tới khi khách gõ đúng, công cụ lặp bạn đang có là
`for ... in range(n)` — mà `range(n)` bắt bạn viết `n` ra ngay lúc gõ code.

Vậy `n` bằng bao nhiêu? Viết 3, gặp người gõ sai lần thứ tư thì sao? Viết 100
thì 99 lượt còn lại để làm gì?

Câu hỏi thật ra là: có lối lặp nào **không cần biết trước số lượt** không? Bài
sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
