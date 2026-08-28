---
id: toan.logic-va-chung-minh.khi-ve-truoc-khong-xay-ra
title: Khi vế trước không xảy ra
summary: Vế trước sai thì cả câu "nếu... thì" đúng, vì không có nghĩa vụ nào để mà bỏ — và một câu đúng như thế chẳng nói gì về vế sau của nó.
locale: vi
track: toan
module: logic-va-chung-minh
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.vacuous-truth]
requires: [logic.implication, logic.de-morgan, logic.equivalence, logic.tautology, logic.contradiction, logic.truth-table, logic.disjunction, logic.conjunction, logic.negation, logic.truth-value, logic.proposition, logic.and, logic.or, logic.not, core.boolean, core.variable, core.list, core.list-append, core.tuple, core.for-unpack, core.print-variable, core.function-def, core.function-call, core.function-parameter, core.function-return, ctrl.for-each, ctrl.if]
concepts: [logic.chan-ly-rong, logic.sai-keo-theo-bat-ky, logic.cau-dung-khong-noi-gi-ve-ve-sau]
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
Nội quy của CLB nói về thành viên. Bình không phải thành viên — vậy nó nói gì về Bình?
::::

::::explain{#hai-dong-con-lai}
Bài trước chốt được luật của chữ "nếu... thì": câu sai ở đúng một dòng, dòng vế
trước đúng mà vế sau sai. Nhưng bài ấy chỉ đi kỹ hai dòng đầu, hai dòng mà người
được nhắc tên **là** thành viên. Hai dòng dưới mới được ghi Đ vào mà chưa ai bàn.

Câu hỏi để lại là câu về Bình. Bình học cùng lớp 6A nhưng không vào CLB cờ vua,
và sáng nay Bình không đeo thẻ nào cả. Câu nội quy về Bình:

> **"Nếu Bình là thành viên thì Bình đeo thẻ."**

Vế trước sai, vế sau cũng sai — đúng dòng 4 của bảng. Nội quy có bị phá không?

Đi tìm câu trả lời bằng đúng cách bài 10 đã đi: hỏi xem có bắt quả tang được
không. Muốn nói "dòng chữ này sai", thầy phụ trách phải chỉ ra được một người
**là thành viên** mà **không đeo thẻ**. Bình không phải thành viên, nên Bình
không đứng vào chỗ ấy được. Nội quy chưa hề đặt lên vai Bình nghĩa vụ nào, mà
không có nghĩa vụ thì không có nghĩa vụ nào bị bỏ.

Nói cách khác: Bình **không phá được** câu ấy, dù Bình làm gì đi nữa.

Không phá được thì câu không sai. Và bài 2 đã đóng cửa thứ ba: một mệnh đề mang
đúng một trong hai giá trị, không có ô "chưa biết" để đứng. Không sai thì chỉ
còn một chỗ.

> Câu "nếu P thì Q" có **vế trước sai** thì cả câu **đúng**, bất kể vế sau đúng
> hay sai. Nghề toán gọi kiểu đúng ấy là **chân lý rỗng** — đúng vì rỗng, vì
> không có ca nào để mà hỏng.

Chữ "rỗng" ở đây không phải lời chê. Nó tả đúng chuyện đang xảy ra: cái rổ những
người có thể bắt quả tang câu này là một cái rổ không có ai trong đó.
::::

::::explain{#cau-dung-khong-hua-gi-ve-ve-sau}
Chân lý rỗng kéo theo một hệ quả mà nhiều người đọc lướt qua rồi hiểu ngược.

Nội quy CLB đúng. Bình không đeo thẻ. **Cả hai chuyện ấy cùng đúng một lúc**, và
chúng không cãi nhau chút nào — vì câu nội quy về Bình có vế trước sai, nó đúng
theo kiểu rỗng, và cái đúng ấy không hứa hẹn gì về việc Bình có đeo thẻ hay
không.

> "P → Q đúng" **không** có nghĩa "Q đúng".

Câu kéo theo là một lời hứa **có điều kiện**. Nó chỉ mở miệng khi điều kiện được
thoả. Điều kiện chưa thoả thì nó im lặng, và một câu im lặng thì không nói cho
bạn biết vế sau ra sao.

Thử một câu nghe còn lạ tai hơn. Thầy phụ trách viết thêm lên bảng:

> **"Nếu Bình là thành viên thì Bình phải đứng một chân suốt buổi tập."**

Câu này **đúng**. Không phải vì CLB thật sự bắt ai đứng một chân, mà vì Bình
không phải thành viên: không có buổi tập nào mà Bình đến với tư cách thành viên
để mà bị bắt lỗi. Vế trước sai kéo theo bất kỳ vế sau nào, và cả câu vẫn đứng
vững.

Chỗ này đáng dừng lại một nhịp, vì nó dễ bị đọc thành "logic cho phép nói bậy".
Không phải. Câu ấy đúng, nhưng nó **rỗng ruột** — nó không cho bạn thêm một mẩu
tin nào về thế giới. Muốn một câu kéo theo nói được điều gì có ích, bạn phải cầm
thêm một thứ trong tay: bằng chứng rằng vế trước của nó **đúng**.
::::

::::example{#buoi-sang-o-cua-phong}
Một buổi sáng thật ở cửa phòng CLB, bốn người đi qua:

| tên | là thành viên CLB? | đang đeo thẻ? | câu nội quy về người này |
|---|---|---|---|
| Nam | Đ | Đ | Đ — lời hứa được giữ |
| Lan | Đ | S | **S** — thành viên mà không đeo thẻ |
| Bình | S | S | Đ — rỗng, Bình không có nghĩa vụ nào |
| Kiên | S | Đ | Đ — rỗng; Kiên đeo thẻ khách vào ngồi xem |

Đọc cột cuối theo hàng dọc thì thấy hai chuyện.

**Chuyện thứ nhất.** Ba trong bốn câu đúng, và trong ba câu đúng ấy có hai câu
đúng theo kiểu rỗng. Nếu chỉ nhìn con số "ba câu đúng" mà kết luận "nội quy đang
được chấp hành tốt" thì kết luận ấy nhầm — hai trong ba câu ấy đúng mà không cần
ai chấp hành gì cả.

**Chuyện thứ hai.** Bình và Kiên nằm cùng một chỗ trên bảng nội quy: cả hai đều
có câu đúng theo kiểu rỗng. Vậy mà một người không đeo thẻ, một người đang đeo.
Hai buổi sáng khác hẳn nhau ở vế sau, mà câu nội quy về họ vẫn cùng một giá
trị — đây chính là "câu đúng không nói gì về vế sau", nhìn thấy tận mắt.
::::

::::predict{#doan-cau-ve-binh commitOnce}
Byte hỏi máy đúng ba câu về Bình, dùng lại cái máy `keo_theo` của bài trước.

Dòng thứ ba hỏi một câu so sánh: giá trị của **cả câu nội quy về Bình** có bằng
giá trị của **vế sau** (Bình có đeo thẻ không) hay không.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

# Bình học cùng lớp 6A nhưng không vào CLB, và sáng nay Bình không đeo thẻ.
binh_la_thanh_vien = False
binh_deo_the = False

noi_quy_ve_binh = keo_theo(binh_la_thanh_vien, binh_deo_the)

print(noi_quy_ve_binh)
print(binh_deo_the)
print(noi_quy_ve_binh == binh_deo_the)
```

:::opt{correct}
`True` rồi `False` rồi `False`
:::

:::opt
`False` rồi `False` rồi `True`
::why
Gần đúng ở chỗ bạn để ý rằng Bình không đeo thẻ, và trong tiếng Việt hằng ngày
một dòng nội quy về đeo thẻ mà gặp người không đeo thẻ thì nghe như đang bị vi
phạm.

Chỗ lệch: muốn câu "nếu Bình là thành viên thì Bình đeo thẻ" sai, phải có **cả
hai** chuyện cùng lúc — Bình là thành viên, và Bình không đeo thẻ. Ở đây chuyện
thứ nhất không xảy ra, nên `truoc and not sau` cho `False`, nhánh `if` không
chạy, và hàm trả về `True`. Dòng thứ ba khi đó là `True == False`, tức `False`.
::
:::

:::opt
`True` rồi `True` rồi `True`
::why
Gần đúng ở chỗ bạn nắm được nửa đầu của bài: câu nội quy về Bình đúng, và dòng
đầu đúng là `True`.

Chỗ lệch ở dòng thứ hai. `print(binh_deo_the)` in ra giá trị mà cái tên ấy đang
giữ, và ngay bên trên nó được gán `False` — chuyện Bình không đeo thẻ là một sự
việc, không phải thứ mà một dòng nội quy đúng có thể sửa lại. Đây chính là điều
bài vừa nói: câu kéo theo đúng **không** làm cho vế sau của nó đúng theo.
::
:::

:::opt
`True` rồi `False` rồi `True`
::why
Gần đúng ở chỗ hai dòng đầu bạn đọc chính xác: câu nội quy về Bình đúng, và Bình
thì không đeo thẻ.

Chỗ lệch ở dòng thứ ba. Nó không hỏi "hai câu này có cùng nói về Bình không" mà
hỏi "hai **giá trị** này có bằng nhau không" — dấu `==` của bài R0.24 so hai giá
trị Đ/S. Bên trái là `True`, bên phải là `False`, hai giá trị khác nhau, nên máy
trả lời `False`.
::
:::
::::

::::code{#chia-bon-nguoi-lam-hai-ro}
Giờ đến lượt bạn cho máy đi qua cả bốn người ở cửa phòng và chia họ vào hai rổ
khác nhau: rổ những người mà **nội quy im lặng** (không đòi hỏi gì ở họ), và rổ
những người **bắt quả tang được** nội quy bị bội.

Ba chỗ trống:

1. Sau `dung_voi_nguoi_nay =` — giá trị của câu nội quy về người đang xét, viết
   bằng `keo_theo`. Trong tay mỗi lượt có `la_thanh_vien` và `deo_the`.
2. Sau chữ `if` thứ nhất — điều kiện để **nội quy im lặng** với người này. Nội
   quy chỉ mở miệng với thành viên.
3. Sau chữ `if` thứ hai — điều kiện để người này **phá được** nội quy, đúng cái
   dòng duy nhất mà bài 10 đã chốt.

Ba chỗ này được chấm bằng ba đường khác nhau: chỗ một bị đo qua cả cột bốn ô,
chỗ hai và chỗ ba bị đo qua hai danh sách tên **không có ai trùng nhau**. Chép
cùng một câu vào hai chỗ thì một trong hai danh sách sai ngay.

```python title=starter
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

# Sáng nay ở cửa phòng CLB: (tên, là thành viên CLB?, đang đeo thẻ?)
cua_phong = [
    ("Nam", True, True),
    ("Lan", True, False),
    ("Bình", False, False),
    ("Kiên", False, True),
]

cot_noi_quy = []       # câu nội quy VỀ NGƯỜI NÀY đúng hay sai
noi_quy_im_lang = []   # tên người mà nội quy không đòi hỏi gì
pha_duoc_noi_quy = []  # tên người bắt quả tang được nội quy bị bội

for ten, la_thanh_vien, deo_the in cua_phong:
    dung_voi_nguoi_nay = ___
    cot_noi_quy.append(dung_voi_nguoi_nay)
    if ___:
        noi_quy_im_lang.append(ten)
    if ___:
        pha_duoc_noi_quy.append(ten)

print(cot_noi_quy)
print(noi_quy_im_lang)
print(pha_duoc_noi_quy)
```

```python title=solution
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

# Sáng nay ở cửa phòng CLB: (tên, là thành viên CLB?, đang đeo thẻ?)
cua_phong = [
    ("Nam", True, True),
    ("Lan", True, False),
    ("Bình", False, False),
    ("Kiên", False, True),
]

cot_noi_quy = []       # câu nội quy VỀ NGƯỜI NÀY đúng hay sai
noi_quy_im_lang = []   # tên người mà nội quy không đòi hỏi gì
pha_duoc_noi_quy = []  # tên người bắt quả tang được nội quy bị bội

for ten, la_thanh_vien, deo_the in cua_phong:
    dung_voi_nguoi_nay = keo_theo(la_thanh_vien, deo_the)
    cot_noi_quy.append(dung_voi_nguoi_nay)
    if not la_thanh_vien:
        noi_quy_im_lang.append(ten)
    if la_thanh_vien and not deo_the:
        pha_duoc_noi_quy.append(ten)

print(cot_noi_quy)
print(noi_quy_im_lang)
print(pha_duoc_noi_quy)
```

```python title=test
assert cot_noi_quy == [True, False, True, True], "trong bốn người sáng nay, chỉ câu nội quy về Lan là sai — Lan là thành viên mà không đeo thẻ; ba câu còn lại đúng"
assert noi_quy_im_lang == ["Bình", "Kiên"], "nội quy chỉ đòi hỏi ở thành viên, nên nó im lặng với đúng hai người ngoài CLB sáng nay là Bình và Kiên — và im lặng với cả hai, không phân biệt ai đang đeo thẻ"
assert pha_duoc_noi_quy == ["Lan"], "sáng nay chỉ một mình Lan vừa là thành viên vừa không đeo thẻ, nên chỉ Lan bắt quả tang được nội quy bị bội"
assert cot_noi_quy[2] is True, "câu nội quy về Bình ĐÚNG dù Bình không đeo thẻ: Bình không phải thành viên nên không có nghĩa vụ nào để bỏ — đây là chân lý rỗng"
assert cot_noi_quy[3] is True, "câu nội quy về Kiên cũng ĐÚNG, và Kiên thì đang đeo thẻ: hai người cùng được câu ĐÚNG mà vế sau của họ khác nhau"
assert "Bình" not in pha_duoc_noi_quy, "Bình không đeo thẻ, nhưng Bình không phải thành viên nên Bình không phá được nội quy CLB"
assert "Kiên" in noi_quy_im_lang and "Kiên" not in pha_duoc_noi_quy, "Kiên đang đeo thẻ khách, nhưng Kiên không phải thành viên nên nội quy vẫn im lặng với Kiên và Kiên cũng không phá được nó"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất là một lời gọi hàm, giống hệt cách bài 10 gọi `keo_theo` — chỉ khác tên hai thứ đem vào, và thứ tự thì đọc lại chú thích của `cot_noi_quy`: vế trước là gì, vế sau là gì. Hai chỗ trống còn lại đều nằm sau chữ `if`, nên mỗi chỗ là một câu Đ/S về **người đang xét trong lượt này**.
- kind: strategy
  body: "Nội quy im lặng với ai? Với người mà vế trước của câu SAI — tức là không phải thành viên, và chữ \"không\" của bài 3 viết là `not`. Còn người phá được nội quy thì phải thoả HAI chuyện cùng lúc, đúng cái dòng duy nhất làm câu kéo theo sai; hai chuyện cùng lúc thì nối bằng `and`."
- kind: one-line
  body: "Chỗ một là `keo_theo(la_thanh_vien, deo_the)`; chỗ hai là `not la_thanh_vien`; chỗ ba là `la_thanh_vien and not deo_the`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: chỗ một phải gọi `keo_theo` với hai vế đúng thứ tự, chỗ hai phải phủ định vế trước, chỗ ba phải nói đủ cả hai chuyện (vế trước đúng VÀ vế sau sai) — gõ cứng một giá trị Đ/S vào các chỗ ấy thì hai cái rổ nhận hết hoặc không nhận ai, và bài không đo được gì
  requireAst:
  # Khung khởi đầu không gọi `keo_theo` lần nào — thân hàm là chỗ ĐỊNH NGHĨA
  # nó, không phải chỗ gọi. Chỗ trống thứ nhất là lời gọi duy nhất.
  - kind: uses-call, target: keo_theo, min: 1
  # Khung khởi đầu có sẵn MỘT `not` (trong thân `keo_theo`). Chỗ hai cần thêm
  # một, chỗ ba cần thêm một nữa. Luật này một mình chặn mọi đáp án gõ cứng
  # `True`/`False` vào hai chỗ trống sau chữ `if`.
  - kind: uses-operator, target: not, min: 3
  # Khung khởi đầu có sẵn MỘT `and` (trong thân `keo_theo`). Chỗ ba cần thêm
  # một nữa — đó là chỗ nói "vế trước đúng VÀ vế sau sai".
  - kind: uses-operator, target: and, min: 2
  # Khung khởi đầu KHÔNG đọc `la_thanh_vien` lần nào (dòng `for` là gán, không
  # phải đọc). Lời giải đọc nó ở cả ba chỗ trống.
  - kind: uses-name, target: la_thanh_vien, min: 3
  # `deo_the` được đọc ở chỗ một và chỗ ba.
  - kind: uses-name, target: deo_the, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[True, False, True, True\]\n\['Bình', 'Kiên'\]\n\['Lan'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cái rổ, không ai đứng cả hai chỗ. Nội quy im lặng với người ngoài CLB, và im lặng thì không ai phá được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hôm nay bạn học được rằng câu kéo theo đúng mà chẳng nói gì về vế sau. Giờ thử
đi theo chiều ngược lại, chiều mà người ta hay đi nhất khi đứng ở cửa phòng.

Nội quy CLB **đúng** — cả trường công nhận. Và bạn nhìn thấy một người **đang
đeo thẻ**. Chuyện ấy có đủ để kết luận "người này là thành viên CLB" không?

Nhớ lại Kiên ở bảng ban nãy: Kiên đeo thẻ khách vào phòng ngồi xem, và Kiên
không phải thành viên. Vậy mà câu nội quy về Kiên vẫn đúng.

Có vẻ như câu "nếu đeo thẻ thì là thành viên" là một câu **khác** với câu nội
quy, chứ không phải cùng một câu đọc ngược. Bảng của nó trông ra sao?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
