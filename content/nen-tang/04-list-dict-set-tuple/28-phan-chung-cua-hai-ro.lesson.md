---
id: nen-tang.list-dict-set-tuple.phan-chung-cua-hai-ro
title: Phần chung của hai rổ
summary: Dấu `&` giữa hai rổ cho ra một rổ mới chỉ chứa những gì cả hai cùng có — một lượt, thay cho một vòng lặp và một chỗ chứa rỗng.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.set-intersection]
requires: [core.set, core.set-unordered, core.list-membership, core.list, core.list-append, core.len, ctrl.for-each, ctrl.if, core.fstring]
concepts: [core.tap-hop, core.cho-chua]
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
Hai cái rổ đặt cạnh nhau. Mình muốn hỏi cả hai cùng một lúc.
::::

::::explain{#hai-ro-mot-cau-hoi}
Bài trước bạn đổi một thứ lấy một thứ: cái rổ không xếp hàng, không có `ro[0]`
— bù lại, `nhom in ro` trả lời ngay mà không phải dò từng ô.

Byte đem tới hai cái rổ. Rổ thứ nhất là các nhóm chi của **tháng này**, đúng cái
rổ bạn đã dựng ở bài 26 từ cuốn sổ sáu khoản:

- Tháng này: `ăn uống`, `xăng xe`, `học phí` — ba nhóm.

Rổ thứ hai là các nhóm chi của **tháng trước**, Byte lấy từ cuốn sổ tháng trước
theo đúng cách ấy:

- Tháng trước: `ăn uống`, `xăng xe`, `biếu tặng`, `thuê nhà` — bốn nhóm.

Byte hỏi: **nhóm nào tháng nào cũng chi?**

Đồ nghề trong tay bạn đã đủ để trả lời. Duyệt rổ tháng này, mỗi nhóm hỏi rổ
tháng trước một câu `in`, câu nào ra `True` thì nhặt sang một chỗ chứa mới. Bốn
dòng, và nhờ bài trước thì mỗi câu `in` ấy rẻ.

Bốn động tác ấy — *tạo một chỗ chứa rỗng · đi qua từng nhóm · hỏi một câu · nhặt
vào nếu đúng* — chính là chỗ bài trước dừng lại. Chúng chạy đúng, nhưng chúng
không **nói** ra cái ý mà bạn định nói; người đọc phải ghép ngược cả bốn động tác
lại mới ra ý ban đầu.

Ý ấy trong tiếng Việt gọn đúng ba chữ, như bài trước đã hẹn: **phần chung nhau**.
Nói tắt thì là *phần chung*, còn nhà toán học gọi nó là phép **giao**. Python
viết nó bằng đúng một dấu:

```python title=readonly
deu_co = thang_nay & thang_truoc
```

Đọc thành lời: *lấy những gì có mặt trong rổ này **và** có mặt trong rổ kia.*

Thứ `&` trả ra cũng là một **rổ** — nên nó mang đủ tính nết của rổ mà hai bài
trước đã dựng: không chứa hai lần cùng một nhóm, không xếp hàng, và hỏi `in` thì
trả lời ngay.
::::

::::example{#bon-dong-va-mot-dau}
Làm bằng tay trước, để có cái đối chiếu.

```python title=readonly
thang_nay = {"ăn uống", "xăng xe", "học phí"}
thang_truoc = {"ăn uống", "xăng xe", "biếu tặng", "thuê nhà"}

deu_co = []
for nhom in thang_nay:
    if nhom in thang_truoc:
        deu_co.append(nhom)

print(len(deu_co))
```

Máy in ra:

```text title=readonly
2
```

Hai nhóm ấy là `ăn uống` và `xăng xe`.

Ở đây bài chỉ in ra **số lượng** chứ không in cả danh sách, và đó là chuyện có
lý do: `for nhom in thang_nay` đang duyệt một cái **rổ**, mà rổ thì không xếp
hàng — thứ tự hai cái tên rơi vào `deu_co` có thể khác nhau giữa hai máy. Số
lượng thì không đổi.

Giờ viết lại bằng một dấu:

```python title=readonly
thang_nay = {"ăn uống", "xăng xe", "học phí"}
thang_truoc = {"ăn uống", "xăng xe", "biếu tặng", "thuê nhà"}

deu_co = thang_nay & thang_truoc

print(len(deu_co))
print("ăn uống" in deu_co)
print("học phí" in deu_co)
```

Máy in ra:

```text title=readonly
2
True
False
```

Cùng hai nhóm ấy, một dòng thay cho bốn.

Hai dòng cuối nói thêm một chuyện. `học phí` **có** trong rổ tháng này — bài 25
kể rồi, đó là khoản mua vở. Nhưng tháng trước Byte không chi nhóm ấy. Một bên
có, một bên không, thì nó không phải phần chung. `&` chỉ giữ lại thứ nào **cả
hai** rổ cùng có, chứ không giữ thứ nào ít nhất một rổ có.
::::

::::explain{#doc-dau-and-cho-dung}
Ba chuyện đáng ghim lại về dấu `&`:

- **Nó đứng giữa hai cái rổ và cho ra một cái rổ mới.** Hai rổ cũ không bị đụng
  tới. Sau dòng `deu_co = thang_nay & thang_truoc`, `thang_nay` vẫn đủ ba nhóm,
  `thang_truoc` vẫn đủ bốn. Nếp này bạn đã gặp: `sorted` ở bài 19 cũng trả về
  một danh sách mới và để sổ gốc nguyên vẹn, `set(danh_sach)` ở bài 26 cũng
  không đụng gì tới danh sách được đưa vào.
- **Phần chung không bao giờ nhiều hơn cái rổ ít nhóm hơn.** Một nhóm muốn lọt
  vào phần chung thì phải có mặt ở cả hai bên, nên cái rổ ít nhóm hơn chính là
  cái trần. Ở đây rổ tháng này có 3 nhóm, rổ tháng trước có 4, và phần chung là
  2 — không cách nào ra quá 3 được.
- **Phần chung có thể rỗng.** Hai rổ chẳng có nhóm nào giống nhau thì `&` cho ra
  một rổ không chứa gì, `len` của nó là `0`. Đó là một câu trả lời đàng hoàng —
  "không nhóm nào tháng nào cũng chi" — chứ không phải hỏng hóc.

> Chỗ dễ vấp: `&` làm việc giữa **hai cái rổ**. Đem một danh sách ra đặt bên
> cạnh, ví dụ `thang_nay & ["ăn uống"]`, thì máy dừng lại báo `TypeError` — đúng
> họ hàng với lỗi bạn gặp ở bài 27 khi gõ `ro[0]`. Muốn hỏi phần chung với một
> danh sách thì bọc nó thành rổ trước bằng `set(...)` của bài 26.
::::

::::predict{#doi-cho-hai-ro commitOnce}
Byte viết hai dòng, cùng một phép giao nhưng hai cái rổ đặt theo hai thứ tự
ngược nhau.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
thang_nay = {"ăn uống", "xăng xe", "học phí"}
thang_truoc = {"ăn uống", "xăng xe", "biếu tặng", "thuê nhà"}

print(len(thang_nay & thang_truoc))
print(len(thang_truoc & thang_nay))
```

:::opt{correct}
Hai dòng giống hệt nhau: `2` rồi `2`
:::

:::opt
`3` rồi `4` — mỗi lần lấy theo rổ đứng trước
::why
Gần đúng ở chỗ bạn để ý tới rổ đứng trước, và trong nhiều phép viết kiểu
`a <dấu> b` thì vế trái đúng là vế được ưu tiên — cách nghĩ ấy không hề vô cớ.

Chỗ lệch: `3` và `4` chính là số nhóm của **nguyên** hai cái rổ ban đầu, tức là
`&` chẳng bỏ đi gì cả. Mà phép giao thì luôn bỏ: nó vứt mọi nhóm chỉ có ở một
bên. `học phí` chỉ tháng này có; `biếu tặng` và `thuê nhà` chỉ tháng trước có —
ba nhóm ấy rơi ra, còn lại 2.
::
:::

:::opt
`2` rồi `1` — đổi chỗ thì kết quả đổi theo
::why
Gần đúng ở chỗ bạn đọc ra con số `2` cho dòng đầu, và đó là con số đúng: hai
nhóm `ăn uống` và `xăng xe` có mặt ở cả hai rổ.

Chỗ lệch nằm ở dòng thứ hai. Hãy đọc lại câu hỏi mà `&` đặt ra: *nhóm này có mặt
ở cả hai rổ không?* Câu ấy không nêu tên bên nào đứng trước — nó chỉ hỏi "cả
hai". Một nhóm hoặc có mặt ở cả hai, hoặc không; đổi chỗ hai cái rổ không đổi
được câu trả lời cho từng nhóm, nên rổ kết quả cũng y nguyên.
::
:::

:::opt
Máy báo lỗi, vì `&` cần rổ lớn đứng trước rổ nhỏ
::why
Gần đúng ở chỗ bạn cẩn thận với thứ tự — và cẩn thận với thứ tự là thói quen
tốt, vì có những phép thật sự kén thứ tự.

Chỗ lệch: `&` không đòi hỏi gì về kích thước hai bên. Thứ nó đòi là **cả hai bên
đều phải là rổ**; hai bên ở đây đều là rổ, nên nó chạy bình thường. Loại lỗi bạn
đang nghĩ tới chỉ nổ ra khi đặt sai **kiểu**, ví dụ một danh sách bên cạnh một
rổ.
::
:::
::::

::::code{#giao-hai-ro}
Byte muốn một bản tóm tắt hai tháng. Hai cái rổ đã có sẵn; ba dòng `print` cuối
bài cũng viết sẵn rồi.

Việc của bạn là dựng ra `deu_co` — cái rổ chứa những nhóm mà **tháng nào cũng
chi** — bằng đúng một dòng.

```python title=starter
thang_nay = {"ăn uống", "xăng xe", "học phí"}
thang_truoc = {"ăn uống", "xăng xe", "biếu tặng", "thuê nhà"}

deu_co = ___

co_an_uong = "ăn uống" in deu_co
co_hoc_phi = "học phí" in deu_co

print(f"Số nhóm tháng nào cũng chi: {len(deu_co)}")
print(f"Ăn uống tháng nào cũng chi: {co_an_uong}")
print(f"Học phí tháng nào cũng chi: {co_hoc_phi}")
```

```python title=solution
thang_nay = {"ăn uống", "xăng xe", "học phí"}
thang_truoc = {"ăn uống", "xăng xe", "biếu tặng", "thuê nhà"}

deu_co = thang_nay & thang_truoc

co_an_uong = "ăn uống" in deu_co
co_hoc_phi = "học phí" in deu_co

print(f"Số nhóm tháng nào cũng chi: {len(deu_co)}")
print(f"Ăn uống tháng nào cũng chi: {co_an_uong}")
print(f"Học phí tháng nào cũng chi: {co_hoc_phi}")
```

```python title=test
# Kiểm thẳng vào nội dung cái rổ chứ không kiểm mỗi con số 2: một rổ sai nội
# dung vẫn có thể tình cờ đủ hai phần tử. Hai câu sau chỉ đích danh hai nhóm
# phân biệt được "phần chung" với "gộp cả hai rổ lại".
assert deu_co == {"ăn uống", "xăng xe"}, "hai nhóm có mặt trong CẢ HAI rổ là ăn uống và xăng xe — không hơn, không kém"
assert co_an_uong is True, "ăn uống nằm trong rổ tháng này và cũng nằm trong rổ tháng trước, nên nó phải thuộc phần chung"
assert co_hoc_phi is False, "học phí chỉ nằm trong rổ tháng này; tháng trước không chi nhóm ấy, nên nó không thuộc phần chung của HAI rổ này"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ở dòng dựng ra `deu_co`. Câu hỏi hỏi về **cả hai** tháng, nên cả hai cái tên rổ ở ngay phía trên đều phải có mặt trong dòng bạn viết — gõ sẵn hai cái tên nhóm vào đó thì tháng sau sổ đổi, dòng ấy nói dối.
- kind: strategy
  body: Bạn cần đúng phép mà phần giải thích gọi là **phần chung**: một dấu đặt giữa hai cái rổ, cho ra một cái rổ mới. Thứ nó trả ra vẫn là rổ, nên `len(deu_co)` và hai câu `in` ở dưới dùng được ngay mà không phải sửa gì.
- kind: one-line
  body: "Viết `thang_nay & thang_truoc` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Số nhóm tháng nào cũng chi: 2\nĂn uống tháng nào cũng chi: True\nHọc phí tháng nào cũng chi: False\s*$
- tier: output
  expect: Học phí tháng nào cũng chi: False
- tier: static
  onFail: dòng bạn viết phải đem HAI cái rổ ra so với nhau, chứ không gõ sẵn kết quả
  requireAst:
  - kind: uses-name, target: thang_nay, min: 1
  - kind: uses-name, target: thang_truoc, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dấu, hai cái rổ, một câu trả lời. Mình thích lối nói ngắn này.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte đọc bản tóm tắt xong thì hỏi tiếp một câu nghe rất giống câu vừa rồi, mà
hoá ra không phải: **nhóm nào tháng này mới phát sinh, tháng trước chưa từng
có?**

Đó là `học phí` — khoản mua vở của bài 25, cái nhóm mà lúc viết chương trình
chưa ai nhắc tới.

Thử đem `&` ra dùng xem. `thang_nay & thang_truoc` cho ra `ăn uống` và `xăng xe`
— đúng những nhóm **cũ**, tức là đúng những nhóm mà câu hỏi này muốn loại ra. `&`
trả lời sai câu đó, và nó sai theo kiểu khó chịu nhất: nó vẫn đưa cho bạn một
cái rổ trông rất chỉnh tề.

Câu hỏi mới không hỏi "có mặt ở cả hai", nó hỏi "có mặt ở bên này mà **không** có
mặt ở bên kia". Phép nào đúng?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
