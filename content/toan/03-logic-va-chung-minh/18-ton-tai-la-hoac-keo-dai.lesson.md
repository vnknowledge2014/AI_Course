---
id: toan.logic-va-chung-minh.ton-tai-la-hoac-keo-dai
title: "Tồn tại" là chữ "hoặc" kéo dài
summary: Chuỗi "hoặc" nối sáu vế gói lại thành ba chữ "tồn tại một người" — và vì "hoặc" của logic bao gồm cả hai, câu ấy nói ít nhất một, không nói đúng một.
locale: vi
track: toan
module: logic-va-chung-minh
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.exists]
requires: [logic.for-all, logic.open-sentence, logic.disjunction, logic.or, logic.negation, logic.not, logic.proposition, core.boolean, core.dict, core.list, core.list-append, core.function-def, core.function-parameter, core.function-return, core.function-call, core.builtin-function, core.variable, core.print-variable, ctrl.for-each]
concepts: [logic.ton-tai, logic.hoac-keo-dai, logic.it-nhat-mot]
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
Chuỗi "và" thì gói lại thành "mọi thành viên đều...". Còn chuỗi "hoặc" gói lại
thành gì?
::::

::::explain{#chuoi-hoac-doc-ra-sao}
Bài trước để lại đúng câu hỏi đó, và để lại luôn cái chuỗi cần gói:

> Nam chưa nộp quỹ **hoặc** Lan chưa nộp quỹ **hoặc** Minh chưa nộp quỹ **hoặc**
> Hoa chưa nộp quỹ **hoặc** Tú chưa nộp quỹ **hoặc** Khanh chưa nộp quỹ.

Sáu vế, năm chữ "hoặc". Đọc cả câu ra rồi tự hỏi: người nói câu này đang khẳng
định chuyện gì?

Người ấy không chỉ vào Minh, cũng không chỉ vào Khanh. Người ấy không nói được
tên nào cả — nếu nói được thì đã nói một câu ngắn hơn nhiều. Thứ người ấy khẳng
định là: **trong sáu người ấy, có người chưa nộp quỹ.**

Tiếng Việt có sẵn mấy cách nói câu đó, và cách nào cũng gói đúng chuỗi trên:

- "Có một thành viên chưa nộp quỹ."
- "Có thành viên chưa nộp quỹ."
- và cách mà nghề toán chọn: "**Tồn tại** một thành viên chưa nộp quỹ."

Ba chữ "tồn tại một" thay được cho cả một chuỗi năm chữ "hoặc", đúng như ba chữ
"mọi thành viên" ở bài trước thay được cho cả chuỗi "và".
::::

::::explain{#luat-cua-ton-tai}
Chữ nối đã biết, giờ lấy luật của nó.

Bài 5 đã chốt luật của chữ "hoặc" cho hai vế: câu ghép đúng khi **ít nhất một**
vế đúng. Nối sáu vế thì luật ấy không đổi gì, nó chỉ được dùng nhiều lần hơn:

> **Tồn tại một thành viên chưa nộp quỹ** đúng khi **ít nhất một** trong sáu mệnh
> đề con đúng. Cả sáu mệnh đề con cùng sai thì câu ấy sai.

Câu bắt đầu bằng chữ "tồn tại" như thế gọi là một câu có **lượng từ tồn tại**.
Nghề toán viết tắt lượng từ ấy bằng ký hiệu **∃**, đọc là "tồn tại". Cũng như ký
hiệu ∀ của bài trước: biết mặt là đủ, bài này vẫn dùng chữ tiếng Việt.

Và đây là chỗ bài 5 đã dặn trước mà giờ mới thu tiền. Bài 5 nói: chữ "hoặc" của
logic **bao gồm cả hai**, khác với chữ "hoặc" ngoài quán vốn ngầm hiểu là chọn
một. Kéo dài ra sáu vế thì chỗ khác nhau ấy cũng dài theo:

> "Tồn tại một thành viên chưa nộp quỹ" nói **ít nhất một**. Nó **không** nói
> đúng một, và nó không hứa gì về số người.

Nên nếu cả Minh lẫn Khanh cùng chưa nộp, câu ấy vẫn đúng — đúng y như khi chỉ
mình Minh chưa nộp. Chữ "một" trong "tồn tại một" là chữ "một" của bài 5, không
phải chữ "một" của lúc gọi đồ uống.

Còn một chỗ nữa giữ nguyên từ bài trước: câu "tồn tại" cũng phải nói rõ nó chạy
trên danh sách nào. "Tồn tại một thành viên CLB chưa nộp quỹ" và "tồn tại một
người trong tổ cờ nhanh chưa nộp quỹ" là hai câu khác nhau.
::::

::::example{#hai-cau-ton-tai}
Đem luật ấy chấm hai câu, vẫn trên cuốn sổ ba cột đã quen:

| thành viên | đeo thẻ (sáng thứ Hai) | đã nộp quỹ tháng này | có mặt buổi họp |
|---|---|---|---|
| Nam | có | rồi | có |
| Lan | có | rồi | có |
| Minh | có | **chưa** | có |
| Hoa | **không** | rồi | có |
| Tú | có | rồi | có |
| Khanh | có | **chưa** | có |

**Câu thứ nhất — "Tồn tại một thành viên chưa nộp quỹ."**

Câu mở ở đây là `___ chưa nộp quỹ`, và nó là câu ngược của câu mở `___ đã nộp
quỹ` — đúng phép phủ định của bài 3, chỉ khác là giờ nó áp lên một câu còn chỗ
trống. Điền sáu cái tên vào rồi lật giá trị:

| điền vào chỗ trống | đã nộp quỹ | chưa nộp quỹ |
|---|---|---|
| Nam | Đ | S |
| Lan | Đ | S |
| Minh | S | **Đ** |
| Hoa | Đ | S |
| Tú | Đ | S |
| Khanh | S | **Đ** |

Chuỗi "hoặc" sáu vế: S, S, **Đ**, S, S, **Đ**. Có vế đúng, nên cả chuỗi đúng. Câu
này mang giá trị **Đ**.

Để ý: có **hai** vế đúng chứ không phải một, và câu vẫn cứ đúng, không đúng hơn
và cũng không hỏng đi. Đó là chữ "hoặc" bao gồm cả hai, nhìn tận mắt.

**Câu thứ hai — "Tồn tại một thành viên vắng buổi họp."**

Câu mở `___ vắng buổi họp` cũng là câu ngược, lần này của `___ có mặt buổi họp`.
Cột "có mặt" ghi "có" ở cả sáu dòng, nên cột "vắng" là S ở cả sáu dòng.

Chuỗi "hoặc" sáu vế: S, S, S, S, S, S. Không vế nào đúng, nên cả chuỗi sai. Câu
này mang giá trị **S**.

Đặt hai câu cạnh nhau thì thấy chữ "hoặc" kéo dài xử việc rất gọn: nó đi tìm
**một** vế đúng. Tìm thấy thì trả Đ và chuyện kết thúc ở đó; đi hết mà không thấy
thì mới trả S.
::::

::::predict{#doan-any commitOnce}
Python có chữ để nối "hoặc" bao nhiêu lần cũng được, và nó đứng ngay cạnh `all`
của bài trước: `any`. Bỏ vào cho nó một danh sách giá trị Đ/S, nó trả về `True`
khi **ít nhất một** giá trị là `True`.

Byte gõ bốn dòng: một chuỗi `or` viết tay, rồi đúng chuỗi ấy viết bằng `any`, rồi
hai danh sách khác.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(False or False or True or False or False or True)
print(any([False, False, True, False, False, True]))
print(any([False, False, False]))
print(any([True, True]))
```

:::opt{correct}
`True`, rồi `True`, rồi `False`, rồi `True`
:::

:::opt
`False`, rồi `False`, rồi `False`, rồi `True`
::why
Gần đúng ở chỗ bạn đọc trúng hai dòng cuối, và ở chỗ bạn dùng một lối nghĩ rất
tự nhiên với chữ "hoặc" ngoài đời: ở quán, "gọi trà hoặc cà phê" mà bê về cả hai
cốc thì người ta bảo bạn nghe nhầm. Hai dòng đầu có **hai** giá trị `True`, nên
theo lối ấy chúng phải hỏng.

Chỗ lệch nằm đúng ở chỗ bài 5 đã tách ra: chữ "hoặc" của logic **bao gồm cả hai**.
Nó hỏi "có ít nhất một vế đúng không", và câu hỏi ấy không cấm vế thứ hai cũng
đúng. Hai vế đúng thì câu trả lời vẫn là "có" — nên hai dòng đầu in ra `True`.

Dòng cuối là chỗ nhìn thấy điều đó rõ nhất: cả hai giá trị đều `True`, và `any`
vẫn trả `True` chứ không đổi ý.
::
:::

:::opt
`True`, rồi `True`, rồi `False`, rồi `False`
::why
Gần đúng ở chỗ bạn đọc trúng ba dòng đầu — trong đó có dòng ba, chỗ nhiều người
lưỡng lự nhất — và ở chỗ bạn giữ chặt một sự thật thật của tiếng Việt: nói "tồn
tại **một** người" trong khi có tới hai người thì nghe kỳ.

Chỗ lệch: chữ "một" trong "tồn tại một" đếm **sàn**, không đếm **trần**. Nó nói
"có ít nhất chừng này", không nói "có đúng chừng này". Trong sổ quỹ cũng vậy: cả
Minh lẫn Khanh cùng chưa nộp, mà câu "tồn tại một thành viên chưa nộp quỹ" vẫn
đúng — vì để câu ấy sai thì phải cả sáu người đều đã nộp.

Nên `any([True, True])` trả `True`. Muốn nói "đúng một người" thì cần một câu
khác hẳn, và câu ấy không phải chuỗi "hoặc" kéo dài.
::
:::

:::opt
`True`, rồi `True`, rồi `True`, rồi `True`
::why
Gần đúng ở chỗ bạn nắm chắc nửa quan trọng của `any`: hễ thấy một `True` là nó
trả `True`, không cần xét tiếp. Ba dòng trong bốn dòng đúng theo lối ấy thật.

Chỗ lệch nằm ở dòng ba, và nó là nửa còn lại của cùng một luật. `any` đi tìm một
giá trị `True`; danh sách ở dòng ba có ba ô và cả ba đều `False`, nên nó đi hết
danh sách mà không tìm thấy gì. Không tìm thấy thì nó trả `False`.

Đây đúng là câu "tồn tại một thành viên vắng buổi họp" ở trên: cả sáu người đều
có mặt, nên chuỗi "hoặc" không có vế nào đúng, và câu ấy sai. Một câu "tồn tại"
sai được — nó sai đúng khi cả danh sách không có ai làm nó đúng.
::
:::
::::

::::code{#tim-mot-ve-dung}
Giờ bắt máy dựng hai cột giá trị của hai câu mở **ngược**, rồi thu mỗi cột về
một giá trị bằng chữ "hoặc" kéo dài.

Bốn chỗ trống:

1. Trong `bang_chua_nop` — câu mở `___ chưa nộp quỹ`. Cuốn sổ `quy_thang_nay` ghi ai
   **đã** nộp, nên chỗ này phải lật giá trị tra được, bằng đúng công cụ của bài 3.
2. Trong `bang_vang_mat` — câu mở `___ vắng buổi họp`, lật cuốn sổ `co_mat` y như
   thế.
3. `co_nguoi_chua_nop` — giá trị của câu "tồn tại một thành viên chưa nộp quỹ".
4. `co_nguoi_vang` — giá trị của câu "tồn tại một thành viên vắng buổi họp".

Bài chấm bằng **hai cuốn sổ cư xử ngược nhau**: sổ quỹ có hai người chưa nộp nên
câu thứ nhất đúng, còn sổ họp không thiếu ai nên câu thứ hai sai. Gõ cứng một giá
trị Đ/S vào hai chỗ cuối thì hai dòng ấy giống hệt nhau và hỏng ngay. Khối test
còn hỏi riêng hai người **cùng** chưa nộp, để chắc rằng bạn viết ra chữ "hoặc"
của bài 5 chứ không phải một phép chọn-đúng-một.

```python title=starter
thanh_vien = ["Nam", "Lan", "Minh", "Hoa", "Tú", "Khanh"]
quy_thang_nay = {"Nam": True, "Lan": True, "Minh": False,
                 "Hoa": True, "Tú": True, "Khanh": False}
co_mat = {"Nam": True, "Lan": True, "Minh": True,
          "Hoa": True, "Tú": True, "Khanh": True}

# Câu mở "___ chưa nộp quỹ" — câu ngược của "___ đã nộp quỹ".
def bang_chua_nop(danh_sach):
    ra = []
    for ten in danh_sach:
        ra.append(___)
    return ra

# Câu mở "___ vắng buổi họp" — câu ngược của "___ có mặt buổi họp".
def bang_vang_mat(danh_sach):
    ra = []
    for ten in danh_sach:
        ra.append(___)
    return ra

# "Tồn tại một thành viên chưa nộp quỹ" — chữ "hoặc" nối sáu vế.
co_nguoi_chua_nop = ___

# "Tồn tại một thành viên vắng buổi họp" — cũng chữ "hoặc" nối sáu vế.
co_nguoi_vang = ___

print(bang_chua_nop(thanh_vien))
print(co_nguoi_chua_nop)
print(co_nguoi_vang)
```

```python title=solution
thanh_vien = ["Nam", "Lan", "Minh", "Hoa", "Tú", "Khanh"]
quy_thang_nay = {"Nam": True, "Lan": True, "Minh": False,
                 "Hoa": True, "Tú": True, "Khanh": False}
co_mat = {"Nam": True, "Lan": True, "Minh": True,
          "Hoa": True, "Tú": True, "Khanh": True}

# Câu mở "___ chưa nộp quỹ" — câu ngược của "___ đã nộp quỹ".
def bang_chua_nop(danh_sach):
    ra = []
    for ten in danh_sach:
        ra.append(not quy_thang_nay[ten])
    return ra

# Câu mở "___ vắng buổi họp" — câu ngược của "___ có mặt buổi họp".
def bang_vang_mat(danh_sach):
    ra = []
    for ten in danh_sach:
        ra.append(not co_mat[ten])
    return ra

# "Tồn tại một thành viên chưa nộp quỹ" — chữ "hoặc" nối sáu vế.
co_nguoi_chua_nop = any(bang_chua_nop(thanh_vien))

# "Tồn tại một thành viên vắng buổi họp" — cũng chữ "hoặc" nối sáu vế.
co_nguoi_vang = any(bang_vang_mat(thanh_vien))

print(bang_chua_nop(thanh_vien))
print(co_nguoi_chua_nop)
print(co_nguoi_vang)
```

```python title=test
# Câu `!=` đứng ĐẦU. Nó canh cái bẫy lớn nhất của bài: chép cùng một dòng vào cả
# hai chỗ trống cuối, hoặc gõ cứng một giá trị Đ/S vào cả hai. Xếp nó xuống dưới
# các câu `==` thì một câu `==` trượt trước và bẫy không bao giờ sập.
assert co_nguoi_chua_nop != co_nguoi_vang, "sổ quỹ có người chưa nộp còn sổ họp không thiếu ai, nên hai câu 'tồn tại' này không thể cùng một giá trị"
assert bang_chua_nop(thanh_vien) == [False, False, True, False, False, True], "cột của câu mở `___ chưa nộp quỹ` trên sáu thành viên: chỉ hai ô của Minh và Khanh là Đ, bốn ô còn lại là S"
assert bang_vang_mat(thanh_vien) == [False, False, False, False, False, False], "sổ họp ghi cả sáu người đều có mặt, nên cột của câu mở `___ vắng buổi họp` là S ở cả sáu ô"
assert co_nguoi_chua_nop is True, "chuỗi 'hoặc' sáu vế có vế của Minh và vế của Khanh cùng đúng, nên câu 'tồn tại một thành viên chưa nộp quỹ' mang giá trị Đ"
assert co_nguoi_vang is False, "cả sáu vế đều sai, nên chuỗi 'hoặc' không tìm được vế đúng nào và câu 'tồn tại một thành viên vắng buổi họp' mang giá trị S"
# HAI người cùng chưa nộp, và câu vẫn phải là Đ. Câu này tồn tại để phân biệt
# chữ "hoặc" của bài 5 với chữ "hoặc" ngoài quán: ai hiểu "tồn tại một" thành
# "đúng một người" sẽ cho danh sách dưới đây giá trị S.
assert any(bang_chua_nop(["Minh", "Khanh"])) is True, "Minh và Khanh cùng chưa nộp quỹ, mà chữ 'hoặc' của logic bao gồm cả hai, nên câu vẫn mang giá trị Đ"
assert any(bang_chua_nop(["Nam", "Minh"])) is True, "trong hai người này riêng Minh chưa nộp quỹ, mà một vế đúng là đủ cho cả chuỗi 'hoặc'"
assert any(bang_chua_nop(["Nam", "Lan", "Hoa", "Tú"])) is False, "bốn người này đều đã nộp quỹ tháng này, nên chuỗi 'hoặc' bốn vế không có vế nào đúng"
assert any(bang_vang_mat(["Minh", "Khanh"])) is False, "Minh và Khanh đều có mặt buổi họp, nên câu mở `___ vắng buổi họp` cho S ở cả hai ô"
```

:::hints
- kind: attention
  body: Hai chỗ trống đầu nằm trong ngoặc của `ra.append`, và mỗi chỗ nói về đúng người đang cầm trong tay ở lượt này — tên người ấy là `ten`. Đọc kỹ chú thích trên mỗi `def`: cuốn sổ ghi ai ĐÃ nộp và ai CÓ MẶT, còn câu mở cần dựng lại hỏi ngược lại chuyện đó. Hai chỗ trống cuối thì nhìn chú thích ngay trên mỗi dòng: nó ghi rõ đây là chữ nối nào và nối mấy vế.
- kind: strategy
  body: "Hai chỗ đầu làm hai việc nối tiếp: tra sổ theo cái tên đang cầm — tên cuốn sổ rồi cặp ngoặc vuông như Realm 1 đã dựng — rồi lật giá trị vừa tra được bằng đúng công cụ phủ định của bài 3. Hai chỗ cuối cũng làm hai việc nối tiếp: dựng cột giá trị bằng chính cái hàm vừa viết ở trên, rồi thu cột ấy về một giá trị bằng chữ \"hoặc\" kéo dài mà phần đoán trước vừa dùng. Hai dòng cuối chỉ khác nhau ở chỗ gọi hàm nào."
- kind: one-line
  body: "Bốn chỗ lần lượt là `not quy_thang_nay[ten]`, `not co_mat[ten]`, `any(bang_chua_nop(thanh_vien))` và `any(bang_vang_mat(thanh_vien))`."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: hai câu mở phải TRA SỔ rồi LẬT giá trị, và hai câu "tồn tại" phải được thu từ chính cột giá trị ấy — gõ cứng một giá trị Đ/S thì chuyện "sáu giá trị thu về một" không được làm ở đâu cả
  requireAst:
  # Hai cuốn sổ, mỗi cuốn phải được ĐỌC. Hai dòng `... = {...}` ở đầu khung là
  # gán, không phải đọc, nên khung khởi đầu đọc cả hai cuốn 0 lần. Hai luật này
  # chặn mọi đáp án gõ cứng giá trị Đ/S vào hai chỗ trống đầu.
  - kind: uses-name, target: quy_thang_nay, min: 1
  - kind: uses-name, target: co_mat, min: 1
  # Hai câu mở đều là câu NGƯỢC, nên mỗi câu cần một phép phủ định. Khung khởi
  # đầu không có `not` nào. Thiếu luật này thì đáp án quên lật giá trị vẫn còn
  # cửa qua ở những danh sách mà hai cột tình cờ trùng nhau.
  - kind: uses-operator, target: not, min: 2
  # Chỗ trống phải nói về người đang cầm trong tay. Khung khởi đầu đọc `ten` 0
  # lần (`for ten in ...` là gán, không phải đọc).
  - kind: uses-name, target: ten, min: 2
  # Hai chỗ cuối phải thu cột về bằng chữ "hoặc" kéo dài. Khung khởi đầu không
  # gọi `any` lần nào.
  - kind: uses-call, target: any, min: 2
  # ... và phải thu từ HAI cột thật, mỗi cột một hàm. Khung khởi đầu gọi
  # `bang_chua_nop` 1 lần (dòng `print`) và `bang_vang_mat` 0 lần; lời giải gọi
  # 2 và 1. Không có luật thứ hai thì `co_nguoi_vang = any(bang_chua_nop(...))`
  # — một dòng nói về sổ quỹ mà mang tên sổ họp — vẫn còn cửa.
  - kind: uses-call, target: bang_chua_nop, min: 2
  - kind: uses-call, target: bang_vang_mat, min: 1
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^\[False, False, True, False, False, True\]\nTrue\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một vế đúng là đủ. Mà đi hết sáu vế không thấy vế nào thì mới dám nói không có.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hai bài vừa rồi đặt cạnh nhau hai chữ nối kéo dài, và chúng tốn công không giống
nhau chút nào.

Muốn **khẳng định** "tồn tại một thành viên chưa nộp quỹ", bạn chỉ ra Minh là
xong. Năm người còn lại không ai hỏi tới nữa, vì một vế đúng đã đủ cho cả chuỗi
"hoặc".

Muốn **khẳng định** "với mọi thành viên: người ấy đeo thẻ", bạn phải đi hết cả
sáu. Xét năm người rồi dừng thì chưa nói được gì — người thứ sáu vẫn có thể làm
chuỗi "và" hụt một mắt.

Đó là giá của hai việc **khẳng định**: một người, và sáu người.

Nhưng mỗi câu còn một việc thứ hai đi kèm, mà chưa bài nào đụng tới: **bác bỏ**
nó. Nói một dòng nội quy đúng là một việc; nói nó bị phá là một việc khác hẳn, và
hai việc ấy chưa chắc tốn công như nhau.

Vậy còn hai việc bác bỏ thì bao nhiêu? Bác bỏ "với mọi" tốn mấy người, và bác bỏ
"tồn tại" tốn mấy?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
