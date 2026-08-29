---
id: nen-tang.list-dict-set-tuple.xep-nguoc-lay-ba-khoan
title: Ba khoản tốn nhất
summary: Nút `reverse=True` bảo `sorted` xếp từ lớn xuống nhỏ, nên ba khoản tốn nhất rơi đúng vào lát cắt đầu danh sách.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.sorted-reverse]
requires: [core.sort-key, core.sorted, core.list-slice, core.dict-items, core.tuple, core.for-unpack, core.dict, core.list, core.list-index, core.len, core.list-append, core.function-def, core.function-return, core.keyword-argument, ctrl.for-each, ctrl.for-range, core.fstring, core.output]
concepts: [core.danh-sach, core.cap, core.ham]
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
Danh sách xếp đúng tiêu chuẩn rồi. Chỉ là nó đang quay đầu về phía bên kia.
::::

::::explain{#dau-hang-la-khoan-re-nhat}
Bài trước bạn đã chỉ được cho `sorted` nhìn vào đâu: đưa `key=lay_tien` vào, và
năm cặp của cuốn sổ nằm theo tiền chứ không theo tên.

Nhưng `sorted` xếp từ **nhỏ lên lớn**. Cốc cà phê hai lăm nghìn lên đứng đầu,
còn khoản sếp muốn thấy trước — khoản tốn nhất — nằm tận cuối hàng.

```python title=readonly
def lay_tien(cap):
    """Đưa ra ô tiền của một cặp (tên, tiền)."""
    return cap[1]

chi = {
    "mua sách": 40000,
    "xăng xe": 60000,
    "cà phê": 25000,
    "sửa xe": 500000,
    "biếu bà": 300000,
}

cac_cap = list(chi.items())
xep = sorted(cac_cap, key=lay_tien)

for ten, tien in xep:
    print(f"{ten}: {tien} đồng")
```

```text title=readonly
cà phê: 25000 đồng
mua sách: 40000 đồng
xăng xe: 60000 đồng
biếu bà: 300000 đồng
sửa xe: 500000 đồng
```

Câu hỏi cuối bài trước là: tự tay lật ngược cả danh sách, hay `sorted` có sẵn
một cái nút?

Lật bằng tay thì làm được, vì mọi mảnh đồ nghề đều đã có. Đi từ ô cuối về ô đầu
và gắp từng cặp sang một danh sách mới:

```python title=readonly
lat = []
for i in range(len(xep)):
    lat.append(xep[len(xep) - 1 - i])
```

Ba dòng, và dòng khó nhất là `xep[len(xep) - 1 - i]`. Cuốn sổ có năm ô nên ô
cuối mang chỗ đứng số 4, mà `i` thì bắt đầu từ 0 — nên phải trừ đi một rồi mới
trừ tiếp `i`. Quên số một ấy là máy dừng ngay với `IndexError`, vì bạn vừa xin
một ô nằm ngoài sổ.

Trừ **dư** một thì còn khó chịu hơn, vì máy không kêu tiếng nào: lượt cuối xin
ô `-1`, mà `-1` là ô cuối sổ (bài 1 dạy đúng chuyện ấy), nên danh sách vẫn đủ
năm khoản — không khoản nào rơi ra. Chỉ có thứ tự bị xáo, và ba dòng đầu của
báo cáo in ra ba khoản không phải ba khoản tốn nhất. Báo cáo vẫn đẹp, vẫn đủ
dòng, vẫn sai người.

Ba dòng để làm một việc mà `sorted` vốn đã biết làm.
::::

::::explain{#nut-doi-chieu-xep}
Cái nút ấy có thật, và nó tên là `reverse`.

```python title=readonly
xep = sorted(cac_cap, key=lay_tien, reverse=True)
```

`reverse=True` viết đúng kiểu bạn đã quen từ mạch Hàm: đưa một thứ vào bằng
**tên tham số** thay vì bằng chỗ đứng. `key=` nói cho `sorted` biết *nhìn vào
đâu*; `reverse=` nói cho nó biết *xếp về chiều nào*. Hai câu hỏi khác nhau, hai
cái tên khác nhau, và chúng không giẫm lên nhau.

Chỗ dễ đọc nhầm nằm ở chữ *reverse*: nó **không** cầm cuốn sổ gốc lên rồi lật
úp. Cuốn sổ gốc vẫn nằm nguyên theo thứ tự ghi chép, y như bài 19 đã chỉ ra —
`sorted` bao giờ cũng trả về một danh sách **mới**. Thứ `reverse=True` đổi là
**chiều xếp**: thay vì nhỏ lên lớn thì lớn xuống nhỏ.

Nói cách khác, `reverse=True` không phải một thao tác làm thêm sau khi xếp xong.
Nó là một phần của chính phép xếp.
::::

::::example{#ba-o-dau-danh-sach}
Bật nút lên trên đúng cuốn sổ vừa rồi:

```python title=readonly
def lay_tien(cap):
    """Đưa ra ô tiền của một cặp (tên, tiền)."""
    return cap[1]

chi = {
    "mua sách": 40000,
    "xăng xe": 60000,
    "cà phê": 25000,
    "sửa xe": 500000,
    "biếu bà": 300000,
}

cac_cap = list(chi.items())
xep = sorted(cac_cap, key=lay_tien, reverse=True)

for ten, tien in xep:
    print(f"{ten}: {tien} đồng")

print("Sổ gốc vẫn là:", cac_cap)
```

Máy in ra:

```text title=readonly
sửa xe: 500000 đồng
biếu bà: 300000 đồng
xăng xe: 60000 đồng
mua sách: 40000 đồng
cà phê: 25000 đồng
Sổ gốc vẫn là: [('mua sách', 40000), ('xăng xe', 60000), ('cà phê', 25000), ('sửa xe', 500000), ('biếu bà', 300000)]
```

Hai chuyện đáng ghi lại:

- Khoản tốn nhất đã lên đầu hàng. Mà "ba khoản tốn nhất" thì bây giờ chính là
  **ba ô đầu danh sách** — và cắt ba ô đầu thì bạn làm được từ bài 2 rồi:
  `xep[:3]`.
- Dòng cuối in `cac_cap` ra, và nó vẫn theo thứ tự ghi chép chứ không theo tiền.
  `sorted` không đụng vào nó lần nào.

Nút `reverse` đổi câu hỏi "khoản nào tốn nhất" thành câu hỏi "ô nào nằm đầu" —
và câu hỏi thứ hai thì đã có sẵn công cụ trả lời.
::::

::::predict{#doan-nut-reverse commitOnce}
Trước khi tự tay bật nút, thử đoán trên một dãy số trần cho gọn. Ba con số dưới
đây lấy từ chính cuốn sổ trên, cố ý đặt lộn xộn.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra hai dòng nào?

```python title=readonly
tien = [60000, 500000, 25000]

print(sorted(tien, reverse=True))
print(tien)
```

:::opt{correct}
`[500000, 60000, 25000]` rồi `[60000, 500000, 25000]`
:::

:::opt
`[25000, 500000, 60000]` rồi `[60000, 500000, 25000]`
::why
Gần đúng ở chỗ bạn nhớ chính xác rằng dòng thứ hai in ra dãy gốc chưa hề bị đụng
tới — `[60000, 500000, 25000]`, đúng nguyên văn thứ tự lúc viết ra.

Chỗ lệch nằm ở chữ *reverse*. Đọc nó thành "lật ngược dãy đang có" thì dòng đầu
sẽ là dãy gốc đọc từ phải sang trái, tức `[25000, 500000, 60000]` — đúng thứ bạn
vừa chọn. Nhưng `reverse=` không lật dãy; nó nói cho phép **xếp** biết đi về
chiều nào. Máy vẫn xếp, chỉ là xếp từ lớn xuống nhỏ, nên `500000` — con số lớn
nhất — phải đứng đầu, không phải `25000`.
::
:::

:::opt
`[500000, 60000, 25000]` rồi `[500000, 60000, 25000]`
::why
Gần đúng ở chỗ dòng đầu bạn đọc không sai một chữ: `reverse=True` cho ra đúng
`[500000, 60000, 25000]`, lớn xuống nhỏ.

Chỗ lệch là dòng thứ hai. Nó hỏi cái tên `tien` đang giữ gì, và câu trả lời vẫn
là dãy y như lúc viết ra: `sorted` **trả về** một danh sách mới chứ không xếp lại
cuốn sổ được đưa cho nó — đúng điều bài 19 đã chỉ ra. Muốn giữ bản đã xếp thì
phải hứng nó bằng một cái tên; ở đây bản ấy in ra xong là không ai cầm nữa.
::
:::

:::opt
`[25000, 60000, 500000]` rồi `[60000, 500000, 25000]`
::why
Gần đúng ở chỗ bạn giữ vững một điều quan trọng: dãy gốc không đổi, nên dòng thứ
hai của bạn chính xác.

Chỗ lệch là dòng đầu — đó là kết quả của `sorted(tien)` **không kèm nút nào**,
tức chiều mặc định nhỏ lên lớn. Cách đọc dẫn tới đây rất tự nhiên: `True` nghĩa
là "đúng", nên `reverse=True` nghe như "cứ xếp đi, được phép". Nhưng cái tên
`reverse` mới là thứ mang nghĩa; `True` chỉ là công tắc bật nó lên. Bật lên thì
chiều xếp đảo lại, và `500000` đứng đầu.
::
:::
::::

::::code{#lay-ba-khoan-ton-nhat}
Giờ tới lượt bạn dựng đoạn báo cáo mà Byte cần: **ba khoản tốn nhất**, khoản tốn
nhiều nhất in trước. Vẫn đúng cuốn sổ năm khoản của hai bài vừa rồi.

Đoạn dưới hở hai chỗ, và hai chỗ ấy làm hai việc khác nhau: một chỗ đổi **chiều
xếp**, một chỗ cắt lấy **ba ô đầu**.

```python title=starter
def lay_tien(cap):
    """Đưa ra ô tiền của một cặp (tên, tiền)."""
    return cap[1]

chi = {
    "mua sách": 40000,
    "xăng xe": 60000,
    "cà phê": 25000,
    "sửa xe": 500000,
    "biếu bà": 300000,
}

cac_cap = list(chi.items())
xep = sorted(cac_cap, key=lay_tien, ___)
top_ba = xep[___]

print("BA KHOẢN TỐN NHẤT")
for ten, tien in top_ba:
    print(f"{ten}: {tien} đồng")
```

```python title=solution
def lay_tien(cap):
    """Đưa ra ô tiền của một cặp (tên, tiền)."""
    return cap[1]

chi = {
    "mua sách": 40000,
    "xăng xe": 60000,
    "cà phê": 25000,
    "sửa xe": 500000,
    "biếu bà": 300000,
}

cac_cap = list(chi.items())
xep = sorted(cac_cap, key=lay_tien, reverse=True)
top_ba = xep[:3]

print("BA KHOẢN TỐN NHẤT")
for ten, tien in top_ba:
    print(f"{ten}: {tien} đồng")
```

```python title=test
# Cuốn sổ này ghi lộn xộn có chủ ý: khoản tốn nhất (sửa xe) nằm ô thứ tư, khoản
# rẻ nhất (cà phê) nằm ô thứ ba. Nên một danh sách chỉ tình cờ trông đúng ở ô
# đầu sẽ lộ ra ngay ở ô cuối.
assert xep[0] == ("sửa xe", 500000), "xếp từ lớn xuống nhỏ thì ô đầu phải là cặp ('sửa xe', 500000) — khoản tốn nhiều nhất trong năm khoản của cuốn sổ này"
assert xep[4] == ("cà phê", 25000), "cùng cách xếp ấy đẩy khoản rẻ nhất của cuốn sổ này — cặp ('cà phê', 25000) — xuống ô cuối cùng"
assert len(top_ba) == 3, "bản báo cáo cần đúng ba dòng, nên lát cắt phải lấy ba ô đầu chứ không lấy cả năm"
assert top_ba == [("sửa xe", 500000), ("biếu bà", 300000), ("xăng xe", 60000)], "ba khoản tốn nhất của cuốn sổ này, theo chiều giảm dần, là sửa xe 500000, biếu bà 300000, rồi xăng xe 60000"
assert cac_cap == [("mua sách", 40000), ("xăng xe", 60000), ("cà phê", 25000), ("sửa xe", 500000), ("biếu bà", 300000)], "sổ gốc phải còn nguyên thứ tự ghi chép: mua sách, xăng xe, cà phê, sửa xe, biếu bà"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm trong cặp ngoặc của `sorted`, ngay sau `key=lay_tien` — nó là một thứ nữa được đưa vào bằng tên tham số, viết cùng kiểu với `key=`. Chỗ trống thứ hai nằm giữa cặp ngoặc vuông sau `xep`, tức là một lát cắt.
- kind: strategy
  body: Chỗ thứ nhất phải bật cái nút đổi chiều xếp, để danh sách chạy từ lớn xuống nhỏ thay vì nhỏ lên lớn; công tắc bật là giá trị đúng-sai mang nghĩa "có". Chỗ thứ hai lấy ba ô đầu của danh sách đã xếp, và bài 3 cho phép bỏ trống đầu bên trái của lát cắt khi muốn nói "từ ô đầu tiên".
- kind: one-line
  body: 'Chỗ thứ nhất viết `reverse=True`, chỗ thứ hai viết `:3` — thành `xep[:3]`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^BA KHOẢN TỐN NHẤT\nsửa xe: 500000 đồng\nbiếu bà: 300000 đồng\nxăng xe: 60000 đồng\s*$
- tier: output
  expect: 'xăng xe: 60000 đồng'
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không phải lật cuốn sổ. Chỉ là bảo mình xếp về chiều bên kia ngay từ đầu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Top 3 xong. Sếp cầm tờ báo cáo lên và hỏi tiếp: khoản đó chi vào **ngày** nào,
nó thuộc **nhóm** gì — ăn uống, xăng xe, hay thứ gì khác?

Bạn nhìn lại thứ mình đang cầm: `('sửa xe', 500000)`. Một cặp có đúng hai ô, một
ô tên và một ô tiền. Ngày với nhóm thì nhét vào đâu?

Kéo dài cái cặp ra cũng được — `('sửa xe', 500000, 11, 'xăng xe')`. Nhưng rồi
tới lúc đọc, bạn phải nhớ rằng ô số 2 là ngày còn ô số 3 là nhóm, và bài 17 đã
nói thẳng chuyện gì xảy ra khi nhớ nhầm một ô: máy đưa ra đúng ô bạn hỏi, không
kêu một tiếng nào.

Một khoản chi có bốn thứ đáng nhớ, và mỗi thứ xứng đáng có một **cái tên** chứ
không phải một chỗ đứng. Mà chỗ chứa tra bằng tên thì bạn có từ bài 9 rồi.

Bài sau ghép nốt hai mảnh ấy lại.
::::

::::checkpoint{mastery=0.8}
::::
