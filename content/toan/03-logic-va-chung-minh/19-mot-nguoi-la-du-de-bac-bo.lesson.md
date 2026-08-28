---
id: toan.logic-va-chung-minh.mot-nguoi-la-du-de-bac-bo
title: Một người là đủ để bác bỏ
summary: Khẳng định "mọi thành viên" phải xét cả sáu, nhưng bác bỏ nó thì một người là đủ — câu "với mọi" sai khi và chỉ khi có ít nhất một phản ví dụ.
locale: vi
track: toan
module: logic-va-chung-minh
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.counterexample]
requires: [logic.for-all, logic.exists, logic.open-sentence, logic.vacuous-truth, logic.not, core.boolean, core.list, core.dict, core.list-append, core.function-def, core.function-call, core.function-parameter, core.function-return, core.return-multiple, core.none, core.variable, core.accumulator, ctrl.for-each, ctrl.if]
concepts: [logic.phan-vi-du, logic.gia-cua-bac-bo, logic.mien-rong]
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
Khẳng định "cả sáu người" thì tốn sáu lượt xét. Còn bác bỏ nó thì tốn mấy?
::::

::::explain{#gia-cua-bon-viec}
Bài trước để lại một câu hỏi về **giá**.

Khẳng định "**tồn tại** một thành viên chưa nộp quỹ" — chỉ ra một người là xong,
người còn lại không ai hỏi tới nữa. Khẳng định "**với mọi** thành viên: người ấy
đeo thẻ" — phải đi hết cả sáu, thiếu một người là chưa nói được gì.

Đó là giá của hai việc **khẳng định**. Còn hai việc **bác bỏ** thì bao nhiêu?

Câu hỏi này không phải câu hỏi mẹo. Bảng nội quy của CLB cờ vua lớp 6A có dòng
"mọi thành viên đều đeo thẻ", và mỗi sáng thầy phụ trách phải nói được một trong
hai câu: hôm nay dòng ấy đúng, hay hôm nay dòng ấy bị phá. Việc thứ hai tốn bao
nhiêu công là chuyện có thật.
::::

::::example{#so-diem-danh-sang-thu-hai}
CLB có sáu thành viên: **Nam, Lan, Minh, Hoa, Tú, Khanh**. Sáng thứ Hai thầy
phụ trách đi một vòng và ghi lại:

| thành viên | đeo thẻ? |
|---|---|
| Nam | có |
| Lan | có |
| Minh | có |
| Hoa | **không** |
| Tú | có |
| Khanh | có |

Câu đang xét là câu "với mọi" của bài 17:

> Với mọi thành viên: người ấy đeo thẻ.

Bài 17 dựng nó thành chữ "và" kéo dài — sáu câu con nối bằng "và". Sáu câu con
ấy là: *Nam đeo thẻ* (Đ), *Lan đeo thẻ* (Đ), *Minh đeo thẻ* (Đ), *Hoa đeo thẻ*
(**S**), *Tú đeo thẻ* (Đ), *Khanh đeo thẻ* (Đ).

Bài 4 đã chốt luật của chữ "và": một vế sai là cả câu ghép sai. Nối sáu lần thì
luật ấy vẫn thế — một câu con sai là cả chuỗi sai.

Nên riêng dòng "Hoa" đã kết thúc câu chuyện. Không phải "gần như sai", không
phải "sai một phần sáu". Câu ấy **sai**, hết.

Hoa có một cái tên riêng trong nghề:

> **Phản ví dụ** của câu "với mọi thành viên: người ấy đeo thẻ" là một thành
> viên cụ thể làm câu mở "người ấy đeo thẻ" thành **sai**.

Và đây là điều đáng dừng lại lâu hơn một chút:

> Câu "với mọi" **sai khi và chỉ khi** có ít nhất một phản ví dụ.

Đọc cả hai chiều, vì bài 15 cho phép đọc một câu "khi và chỉ khi" theo cả hai
chiều:

- **Có phản ví dụ thì câu sai.** Chuyện vừa xảy ra với Hoa.
- **Câu sai thì có phản ví dụ.** Chuỗi "và" sáu vế chỉ sai được bằng một cách —
  một vế nào đó sai. Không có vế nào sai thì cả sáu vế cùng đúng, mà cả sáu vế
  cùng đúng thì câu "với mọi" đúng.

Hai chiều ấy khép lại thành cái bảng giá mà bài trước để hở:

| việc | phải làm gì | tốn mấy người |
|---|---|---|
| khẳng định "tồn tại người chưa nộp quỹ" | chỉ ra một **nhân chứng** | 1 |
| khẳng định "với mọi người: người ấy đeo thẻ" | xét hết | 6 |
| **bác bỏ** "với mọi người: người ấy đeo thẻ" | chỉ ra một **phản ví dụ** | 1 |
| **bác bỏ** "tồn tại người chưa nộp quỹ" | xét hết | 6 |

Bảng này lệch, và nó lệch chéo. "Với mọi" thì đắt khi khẳng định, rẻ khi bác bỏ.
"Tồn tại" thì ngược lại. Chỗ lệch chéo ấy là thứ cả bài hôm nay nói tới, và bài
sau sẽ giải thích vì sao nó phải lệch như thế chứ không lệch kiểu khác.
::::

::::explain{#nhieu-hon-mot-khong-lam-no-sai-hon}
Có hai chỗ dễ va, cùng nằm ở chữ "một".

**Chỗ thứ nhất: tìm thêm phản ví dụ không làm câu sai hơn.** Giả sử sáng thứ Ba
cả Hoa lẫn Tú đều quên thẻ. Câu "với mọi thành viên: người ấy đeo thẻ" hôm ấy có
sai *nặng hơn* thứ Hai không? Không. Bài 2 đã chốt: một mệnh đề mang **đúng một**
trong hai giá trị, và "sai" không có mức độ. Hai phản ví dụ và một phản ví dụ
dẫn tới cùng một chỗ.

Chuyện này có ích thật sự khi bạn đi kiểm: gặp phản ví dụ đầu tiên là **dừng
được**. Không phải vì lười, mà vì xét tiếp không đổi được câu trả lời nữa.

**Chỗ thứ hai: không có phản ví dụ nào thì câu đúng.** Nghe thì thuận tai, cho
tới lúc gặp một tổ chưa có ai.

Tuần này CLB dán thêm một tờ giấy: *"Mọi người trong **tổ trực nhật** đều đeo
thẻ."* Nhưng tổ trực nhật tuần này chưa xếp ai — danh sách rỗng.

Câu ấy đúng hay sai? Đi tìm phản ví dụ: một người **trong tổ** mà **không** đeo
thẻ. Trong tổ không có ai, nên không có ai để mà không đeo thẻ. Không có phản ví
dụ nào cả.

Không có phản ví dụ thì câu không sai. Mà mỗi mệnh đề mang đúng một trong hai
giá trị, không có cửa thứ ba. Nên câu ấy **đúng**.

Bạn đã gặp đúng cảm giác này một lần rồi, ở bài 11: nội quy "nếu là thành viên
thì phải đeo thẻ" không bị phá bởi một người ngoài CLB, vì người ngoài không
làm gì để mà phá. Ở đây cũng vậy — một câu "với mọi" trên một danh sách rỗng
không có ai để làm nó sai. Nó đúng vì **không ai phá nổi**, chứ không phải vì
tổ trực nhật giỏi giang.
::::

::::predict{#doan-to-rong commitOnce}
Byte gõ mấy dòng để hỏi thẳng máy. Hàm `bang_dung_sai` biến một danh sách tên
thành danh sách sáu (hoặc ít hơn) giá trị Đ/S — đúng cái bảng mà bài 16 gọi là
"điền từng cái tên vào chỗ trống".

`all` và `any` bạn đã dùng ở hai bài trước: `all` là chữ "và" kéo dài, `any` là
chữ "hoặc" kéo dài.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
thanh_vien = ["Nam", "Lan", "Minh", "Hoa", "Tú", "Khanh"]
deo_the = {"Nam": True, "Lan": True, "Minh": True,
           "Hoa": False, "Tú": True, "Khanh": True}
to_truc_nhat = []

def bang_dung_sai(danh_sach):
    ra = []
    for ten in danh_sach:
        ra.append(deo_the[ten])
    return ra

print(all(bang_dung_sai(thanh_vien)))
print(all(bang_dung_sai(to_truc_nhat)))
print(any(bang_dung_sai(to_truc_nhat)))
```

:::opt{correct}
`False`, rồi `True`, rồi `False`
:::

:::opt
`False`, rồi `False`, rồi `False`
::why
Gần đúng ở chỗ bạn đọc trúng dòng đầu — Hoa quên thẻ nên chuỗi "và" sáu vế cho
`False`, không bàn cãi. Và cách nghĩ ở hai dòng sau cũng có gốc thật: tổ trực
nhật chưa có ai đeo thẻ cả, nên nói "mọi người trong tổ đều đeo thẻ" nghe rất
kỳ.

Chỗ lệch nằm ở câu hỏi mà `all` thật sự hỏi. Nó không hỏi "có ai đeo thẻ
không" — đó là việc của `any`. Nó hỏi "**có ai làm câu sai không**", và trả
`False` đúng khi tìm thấy một người như thế. Danh sách rỗng không có ai để làm
câu sai, nên `all` không có lý do nào trả `False`.

Dòng thứ ba thì bạn đoán trúng: `any` đòi ít nhất một người **có** đeo thẻ, mà
trong tổ không có ai, nên nó là `False`. Hai dòng cuối lệch nhau chính vì hai
câu hỏi ấy khác nhau.
::
:::

:::opt
`False`, rồi `True`, rồi `True`
::why
Gần đúng ở chỗ bạn giữ đúng một nguyên tắc rất tốt: hai dòng cuối chạy trên cùng
một danh sách rỗng, nên chúng "phải" giống nhau. Với nhiều cặp câu thì lối nghĩ
ấy đúng.

Chỗ lệch: `all` và `any` là hai câu hỏi ngược chiều nhau, và danh sách rỗng là
chỗ duy nhất chúng tách hẳn ra. `all` hỏi "có ai làm câu sai không" — không có,
nên `True`. `any` hỏi "có ai làm câu đúng không" — cũng không có, nên `False`.
Cùng một lý do "trong tổ chẳng có ai", mà hai câu trả lời ngược nhau, vì hai câu
hỏi vốn ngược nhau.
::
:::

:::opt
Máy báo lỗi ở dòng `all(bang_dung_sai(to_truc_nhat))`, vì danh sách rỗng
::why
Gần đúng ở chỗ bạn cẩn thận với danh sách rỗng — đó là thói quen tốt, và ở nhiều
việc khác nó cứu bạn thật: lấy phần tử đầu của một danh sách rỗng thì máy nổ
ngay.

Chỗ lệch: `all` không đi lấy phần tử nào cả, nó **duyệt** danh sách. Duyệt một
danh sách rỗng là duyệt qua không lượt nào — vòng `for` của T1.2 gặp danh sách
rỗng cũng chạy đúng không lượt và không hề báo lỗi. Không có lượt nào thì cũng
không có lượt nào hỏng, nên máy in ra một giá trị bình thường.
::
:::
::::

::::code{#san-phan-vi-du}
Giờ bắt máy làm đúng cái việc mà bảng giá vừa nói: **săn phản ví dụ, và dừng
ngay khi gặp**.

Bạn viết thân của `san_phan_vi_du`. Nó nhận một danh sách tên, đi từ đầu, và trả
về hai thứ:

- **tên người đầu tiên** làm câu mở "người ấy đeo thẻ" thành sai — hoặc `None`
  nếu đi hết mà không gặp ai;
- **số người đã phải xét** cho tới lúc trả lời. Con số này chính là cái giá, nên
  nó phải là số đếm thật, không phải một con số bạn tự biết trước.

Hai chỗ trống:

1. Sau `da_xet =` — cộng thêm một lượt vào bộ đếm, y như biến cộng dồn của
   T1.2.12.
2. Sau `if` — câu hỏi "người này có phải phản ví dụ không".

Bài chấm bằng **năm danh sách khác nhau**, trong đó có một danh sách rỗng và một
danh sách chỉ có mỗi Hoa. Gõ cứng một cái tên hay một con số vào thì danh sách
bên cạnh sai ngay, nên phải viết ra phép đếm thật và câu hỏi thật.

```python title=starter
thanh_vien = ["Nam", "Lan", "Minh", "Hoa", "Tú", "Khanh"]
deo_the = {"Nam": True, "Lan": True, "Minh": True,
           "Hoa": False, "Tú": True, "Khanh": True}
to_truc_nhat = []

def san_phan_vi_du(danh_sach):
    da_xet = 0
    for ten in danh_sach:
        da_xet = ___
        if ___:
            return ten, da_xet
    return None, da_xet

ai, ton = san_phan_vi_du(thanh_vien)
ai_to, ton_to = san_phan_vi_du(to_truc_nhat)

print(ai, ton)
print(ai_to, ton_to)
```

```python title=solution
thanh_vien = ["Nam", "Lan", "Minh", "Hoa", "Tú", "Khanh"]
deo_the = {"Nam": True, "Lan": True, "Minh": True,
           "Hoa": False, "Tú": True, "Khanh": True}
to_truc_nhat = []

def san_phan_vi_du(danh_sach):
    da_xet = 0
    for ten in danh_sach:
        da_xet = da_xet + 1
        if not deo_the[ten]:
            return ten, da_xet
    return None, da_xet

ai, ton = san_phan_vi_du(thanh_vien)
ai_to, ton_to = san_phan_vi_du(to_truc_nhat)

print(ai, ton)
print(ai_to, ton_to)
```

```python title=test
# Hai câu đứng đầu canh hai cái bẫy lớn nhất của bài, nên chúng phải chạy
# TRƯỚC: nếu một câu `==` ở dưới trượt trước, hai bẫy này không bao giờ sập.
#
#   · bẫy 1 — trả bừa một cái tên: danh sách rỗng phải trả `None`.
#   · bẫy 2 — đếm bừa một con số: danh sách rỗng phải đếm 0 lượt.
assert san_phan_vi_du([])[0] is None, "danh sách rỗng thì không có ai để làm câu mở sai, nên không có phản ví dụ nào để trả về"
assert san_phan_vi_du([])[1] == 0, "danh sách rỗng thì vòng `for` chạy đúng không lượt, nên bộ đếm phải dừng ở 0"
assert ai == "Hoa", "trong sổ sáng thứ Hai, Hoa là người duy nhất không đeo thẻ"
assert ton == 4, "Hoa đứng thứ tư trong danh sách `thanh_vien`, nên phải xét đúng 4 người mới gặp — xét ít hơn là bỏ sót, xét nhiều hơn là chưa dừng lúc gặp"
assert ai_to is None and ton_to == 0, "tổ trực nhật tuần này chưa xếp ai, nên không có phản ví dụ và cũng không có lượt xét nào"
assert san_phan_vi_du(["Hoa"]) == ("Hoa", 1), "danh sách chỉ có mỗi Hoa: gặp phản ví dụ ngay lượt đầu, nên đếm dừng ở 1"
assert san_phan_vi_du(["Nam", "Lan"]) == (None, 2), "Nam và Lan đều đeo thẻ trong sổ sáng thứ Hai, nên xét hết 2 người mà vẫn không có phản ví dụ"
assert san_phan_vi_du(["Lan", "Hoa", "Minh"]) == ("Hoa", 2), "trong danh sách này Hoa đứng thứ hai, nên xét 2 người là gặp và dừng — Minh không được xét tới"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm ngay sau `da_xet =`, và nó phải làm con số ấy nhích lên MỘT sau mỗi vòng — đọc lại dòng `da_xet = 0` đứng trên để thấy nó bắt đầu từ đâu. Chỗ trống thứ hai là một câu hỏi Đ/S về đúng người đang đứng trong tay, tên người ấy là `ten`, và cuốn sổ tra thẻ tên là `deo_the`.
- kind: strategy
  body: "Bộ đếm cộng dồn viết theo khuôn của T1.2.12: lấy giá trị cũ rồi cộng thêm một, gán ngược lại vào chính cái tên đó. Còn câu hỏi trong `if`: `deo_the[ten]` cho biết người ấy CÓ đeo thẻ hay không, mà phản ví dụ lại là người KHÔNG đeo — nên bạn cần lật giá trị ấy bằng đúng công cụ của bài 3."
- kind: one-line
  body: "Chỗ thứ nhất là `da_xet + 1`; chỗ thứ hai là `not deo_the[ten]`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: bộ đếm phải đếm thật và câu hỏi phải tra vào cuốn sổ `deo_the` — gõ cứng một con số hay một giá trị Đ/S thì bài đang chấm một thứ nó không hề tính
  requireAst:
  # Khung khởi đầu KHÔNG đọc tên `deo_the` lần nào (dòng `deo_the = {...}` là
  # gán, không phải đọc). Luật này một mình chặn mọi đáp án không tra sổ:
  # `if True`, `if ten == "Hoa"`, `if da_xet == 4`.
  - kind: uses-name, target: deo_the, min: 1
  # Bộ đếm phải ĐỌC giá trị cũ của chính nó. Khung khởi đầu đọc `da_xet` 2 lần
  # (ở hai câu `return`); lời giải đọc 3 — chỗ thứ ba nằm đúng trong phép cộng
  # dồn. Gõ `da_xet = 4` thì con số 3 ấy không bao giờ đạt.
  - kind: uses-name, target: da_xet, min: 3
  # Cộng dồn là một phép CỘNG. Khung khởi đầu không có dấu cộng nào.
  - kind: uses-operator, target: +, min: 1
  forbidAst:
  # Lưới thứ hai, chặn đúng hai con số là ĐÁP ÁN chứ không phải dữ liệu: 4 là
  # số lượt phải xét, 6 là số thành viên. Mọi cách viết hợp lệ đều dựng từ
  # `da_xet`, số 1 và cuốn sổ, nên không cách nào chứa nguyên văn hai số này.
  - kind: has-literal, target: 4
  - kind: has-literal, target: 6
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Hoa 4\nNone 0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn lượt là xong với cả CLB. Còn tổ chưa có ai thì không lượt nào — mà vẫn đúng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sáng thứ Hai, thầy phụ trách có thể nói ra hai câu:

> **Không phải** mọi thành viên đều đeo thẻ.

> **Tồn tại** một thành viên **không** đeo thẻ.

Cả hai đều đúng hôm ấy, và cả hai đều được nói ra nhờ đúng một người là Hoa.

Vậy hai câu ấy là **một** hay là **hai**? Nếu là một, thì cái chữ "không phải"
đứng ngoài cùng đã đi đâu, và vì sao chữ "mọi" hoá thành chữ "tồn tại" khi nó đi
vào trong?

Và còn nửa kia của cặp: phủ định của một câu "**tồn tại**" thì đọc ra sao?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
