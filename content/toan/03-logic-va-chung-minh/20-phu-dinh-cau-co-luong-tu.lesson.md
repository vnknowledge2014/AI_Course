---
id: toan.logic-va-chung-minh.phu-dinh-cau-co-luong-tu
title: Phủ định một câu có lượng từ
summary: Chữ "không phải" đứng ngoài một câu "với mọi" không biến mất — nó đi vào trong, và trên đường đi nó đổi "với mọi" thành "tồn tại". Đó là De Morgan của bài 9, kéo dài ra sáu vế.
locale: vi
track: toan
module: logic-va-chung-minh
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.quantifier-negation]
requires: [logic.counterexample, logic.for-all, logic.exists, logic.de-morgan, logic.open-sentence, logic.and, logic.or, logic.not, logic.parentheses, core.boolean, core.list, core.dict, core.list-append, core.function-def, core.function-call, core.function-parameter, core.function-return, core.variable, ctrl.for-each]
concepts: [logic.phu-dinh-luong-tu, logic.de-morgan-keo-dai, logic.cho-dau-phu-dinh]
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
Hai câu, cùng một buổi sáng, cùng nhờ một mình bạn Hoa mà nói được. Mình muốn
biết đó là một câu hay hai câu.
::::

::::explain{#cho-dau-bai-truoc}
Bài trước dừng lại ở hai câu mà thầy phụ trách nói ra sáng thứ Hai:

> **Không phải** mọi thành viên đều đeo thẻ.

> **Tồn tại** một thành viên **không** đeo thẻ.

Cả hai đều đúng hôm ấy, và cả hai đều nói được nhờ đúng một người: Hoa quên thẻ.

Câu hỏi để lại: hai câu ấy là **một** hay là **hai**? Nếu là một, thì chữ "không
phải" đứng ngoài cùng đã đi đâu, và vì sao chữ "mọi" hoá thành chữ "tồn tại"?

Bạn có sẵn mọi thứ để tự trả lời. Bài 17 đã bảo "với mọi" là chữ "và" kéo dài.
Bài 9 đã bảo khi chữ "không phải" đi vào trong ngoặc thì "và" hoá "hoặc". Ghép
hai câu ấy lại là ra.
::::

::::example{#day-chu-khong-phai-vao-trong}
CLB có sáu thành viên: **Nam, Lan, Minh, Hoa, Tú, Khanh**. Viết lại câu "với
mọi" theo đúng cách bài 17 đã dựng nó — sáu câu con nối bằng chữ "và":

> Nam đeo thẻ **và** Lan đeo thẻ **và** Minh đeo thẻ **và** Hoa đeo thẻ **và**
> Tú đeo thẻ **và** Khanh đeo thẻ.

Giờ đặt chữ "không phải" ra trước cả chuỗi ấy. Để nhìn cho rõ, làm với **ba**
vế trước đã — gọi ba câu con là A, B, C:

> **không phải** (A và B và C)

Bài 9 chỉ nói về **hai** vế: `không phải (P và Q)` bằng `không P hoặc không Q`.
Nhưng ba vế cũng dùng được luật ấy, chỉ cần gom hai vế đầu lại thành một cục.
Đặt `P` là cục `(A và B)`, đặt `Q` là `C`:

> không phải ((A và B) và C)
> → **không phải (A và B) hoặc không C**   ← bài 9, dùng lần thứ nhất
> → **(không A hoặc không B) hoặc không C**   ← bài 9, dùng lần thứ hai

Dấu ngoặc còn lại không đổi được gì, và bạn có cách tự kiểm chuyện đó chứ không
phải tin lời ai: dựng bảng chân lý của bài 6 cho `(X hoặc Y) hoặc Z` và cho
`X hoặc (Y hoặc Z)` — tám dòng, hai cột trùng khít. Nên gỡ ngoặc ra:

> không A **hoặc** không B **hoặc** không C

Với sáu vế thì đúng cách ấy, chỉ là dùng bài 9 thêm ba lần nữa:

> không A hoặc không B hoặc không C hoặc không D hoặc không E hoặc không F

Đọc chuỗi này ra tiếng Việt. Bài 18 đã đặt tên cho nó rồi: "hoặc" kéo dài chính
là chữ **tồn tại**. Sáu câu con bây giờ là *Nam không đeo thẻ*, *Lan không đeo
thẻ*, …, *Khanh không đeo thẻ*. Nên cả chuỗi đọc là:

> **Tồn tại** một thành viên **không** đeo thẻ.

Đó đúng là câu thứ hai. Hai câu ấy là **một**.
::::

::::explain{#luat-doc-duoc-ca-hai-chieu}
Chữ "không phải" không biến mất. Nó **đi vào trong**, và trên đường đi nó làm
đúng hai việc:

- đổi **lượng từ**: "với mọi" thành "tồn tại";
- rồi dán chính nó vào **câu mở** ở bên trong.

Còn nửa kia của cặp — phủ định một câu "tồn tại" — thì chạy y hệt, chỉ đổi vai.
Bài 18 bảo "tồn tại" là chữ "hoặc" kéo dài, và bài 9 cũng nói chiều ngược lại:
`không phải (P hoặc Q)` bằng `không P và không Q`. Kéo dài ra sáu vế:

> **không phải** (A hoặc B hoặc … hoặc F)
> → không A **và** không B **và** … **và** không F

Chuỗi "và" kéo dài là chữ "với mọi". Nên:

> Không có thành viên nào chưa nộp quỹ **⟺** Mọi thành viên đều đã nộp quỹ.

Hai câu ấy cũng là một. Viết gọn cả hai chiều thành bảng:

| câu bị phủ định | câu tương đương |
|---|---|
| Không phải **mọi** thành viên đều đeo thẻ | **Tồn tại** một thành viên **không** đeo thẻ |
| **Không** có thành viên nào chưa nộp quỹ | **Mọi** thành viên đều **đã** nộp quỹ |

Một câu để nhớ cả bảng: **phủ định đi qua lượng từ thì lượng từ đổi phe, và phủ
định rơi xuống câu mở.**

Đây không phải luật mới phải học thuộc. Đây là bài 9 kéo dài — hệt như "với
mọi" và "tồn tại" chỉ là "và" với "hoặc" kéo dài.
::::

::::explain{#cho-de-va-nhat}
Có đúng một chỗ hay va, và nó nằm ở chỗ chữ "không" **rơi xuống**.

Hai câu này trông rất giống nhau:

> (1) **Không phải** mọi thành viên đều đeo thẻ.
> (2) Mọi thành viên đều **không** đeo thẻ.

Câu (1) là câu ta vừa dựng: chữ "không" đi qua lượng từ, nên lượng từ đổi thành
"tồn tại". Câu (2) giữ nguyên lượng từ "mọi" và nhét chữ "không" vào trong —
tức là chưa từng đẩy nó qua lượng từ.

Đem cả hai về sổ sáng thứ Hai, nơi chỉ mình Hoa quên thẻ:

- Câu (1): có một người không đeo thẻ (Hoa) → **đúng**.
- Câu (2): đòi cả sáu người đều không đeo thẻ, mà Nam có đeo → **sai**.

Cùng một buổi sáng, một câu đúng một câu sai. Nên chúng không thể là một câu.

Chỗ này đáng ghi lại: **phủ định của "mọi … đều P" không phải "mọi … đều không
P"**. Nó là "tồn tại một … không P". Chữ "không" phải đi qua lượng từ trước, và
lượng từ phải đổi phe khi nó đi qua.
::::

::::predict{#doan-nua-kia commitOnce}
Byte hỏi máy nửa còn lại của cặp — nửa nói về chữ "tồn tại".

Sổ quỹ tuần này: cả sáu người **đều đã nộp**. Byte dựng hai danh sách Đ/S theo
đúng khuôn bài 16: một danh sách cho câu mở "người ấy đã nộp quỹ", một danh sách
cho câu mở "người ấy **chưa** nộp quỹ".

`all` là chữ "và" kéo dài, `any` là chữ "hoặc" kéo dài — bạn đã dùng cả hai ở
hai bài trước.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
thanh_vien = ["Nam", "Lan", "Minh", "Hoa", "Tú", "Khanh"]
da_nop_quy = {"Nam": True, "Lan": True, "Minh": True,
              "Hoa": True, "Tú": True, "Khanh": True}

da_nop = []
chua_nop = []
for ten in thanh_vien:
    da_nop.append(da_nop_quy[ten])
    chua_nop.append(not da_nop_quy[ten])

print(any(chua_nop))        # "Tồn tại một thành viên chưa nộp quỹ"
print(not any(chua_nop))    # "KHÔNG có thành viên nào chưa nộp quỹ"
print(all(da_nop))          # "Mọi thành viên đều đã nộp quỹ"
```

:::opt{correct}
`False`, rồi `True`, rồi `True`
:::

:::opt
`False`, rồi `True`, rồi `False`
::why
Hai dòng đầu bạn đọc trúng hẳn: không ai còn nợ quỹ, nên "tồn tại người chưa
nộp" là `False`, và phủ định của nó là `True`.

Chỗ lệch nằm ở dòng ba. `all(da_nop)` chạy trên danh sách `da_nop` — danh sách
của câu mở "người ấy **đã** nộp" — mà cả sáu ô trong đó đều `True`. Chuỗi "và"
sáu vế toàn Đ thì cho Đ.

Có thể bạn đang nhớ tới `da_nop_quy`, cuốn sổ gốc, và nghĩ `all` phải đọc gì đó
khác. Nhưng hai danh sách trong bài chỉ là hai cách đọc cùng cuốn sổ ấy: `da_nop`
chép nguyên, `chua_nop` lật từng ô. Dòng ba và dòng hai vì thế cùng ra `True` —
và chính chuyện chúng bằng nhau là điều bài hôm nay nói tới.
::
:::

:::opt
`False`, rồi `False`, rồi `True`
::why
Dòng đầu và dòng cuối bạn đọc trúng.

Chỗ lệch ở dòng hai, và nó là một cái bẫy thật: `not any(chua_nop)` **không**
phải "any của những cái không". Chữ `not` ở đây bọc **cả câu** `any(chua_nop)`,
tức là nó lật đúng một giá trị — cái `False` mà dòng trên vừa in ra. Lật `False`
thì được `True`.

Cách phân biệt: đọc từ trong ra ngoài. `any(chua_nop)` chạy xong trước, cho
`False`; rồi `not` mới lật nó. Đây đúng là chỗ dấu ngoặc nói ra ý, hệt bài 9.
::
:::

:::opt
`True`, rồi `False`, rồi `True`
::why
Bạn giữ đúng một quan hệ rất tốt: dòng hai phải là phủ định của dòng một, nên
hai dòng ấy phải ngược nhau. Đó là bài 3, và nó đúng.

Chỗ lệch ở dòng một. `any(chua_nop)` hỏi "trong danh sách `chua_nop` có ô nào
`True` không" — tức là "có ai **chưa** nộp không". Tuần này cả sáu người đều đã
nộp, nên cả sáu ô của `chua_nop` đều `False`, và `any` không tìm được ô nào để
gật đầu.

Có thể bạn đã đọc `any(chua_nop)` thành "có ai nộp rồi không". Câu ấy thì đúng
là `True`, nhưng nó ứng với `any(da_nop)`, một danh sách khác.
::
:::
::::

::::code{#mot-cau-hai-cach-viet}
Giờ bắt máy đối chiếu hai cách viết trên **ba cuốn sổ khác nhau**, chứ không chỉ
một buổi sáng.

Bạn viết thân của hai hàm, mỗi hàm dựng đúng một câu tiếng Việt:

1. `khong_phai_moi_nguoi_deo` — câu **"Không phải mọi thành viên đều đeo thẻ"**.
   Đây là một câu "với mọi" **bị phủ định**, nên viết nó bằng `all`, rồi lật kết
   quả bằng đúng công cụ của bài 3.
2. `ton_tai_nguoi_khong_deo` — câu **"Tồn tại một thành viên không đeo thẻ"**.
   Đây là một câu "tồn tại", nên viết nó bằng `any`, chạy trên danh sách của câu
   mở đã bị lật.

Bài cố tình đòi mỗi câu viết bằng đúng cái lượng từ của nó — một `all` và một
`any`. Cả bài hôm nay nói rằng hai cách viết ấy cho cùng một cột giá trị, mà
muốn thấy chúng trùng nhau thì phải có **hai** cách viết đứng cạnh nhau.

Hai hàm dựng danh sách Đ/S đã viết sẵn cho bạn: `bang_deo_the` chép nguyên cuốn
sổ, `bang_khong_deo_the` lật từng ô.

Bài chấm bằng ba cuốn sổ: thứ Hai (mình Hoa quên), thứ Ba (cả sáu đều đeo), thứ
Tư (cả sáu đều quên). Trả bừa `True` thì cuốn thứ Ba sai, trả bừa `False` thì
cuốn thứ Hai sai.

```python title=starter
thanh_vien = ["Nam", "Lan", "Minh", "Hoa", "Tú", "Khanh"]

so_thu_hai = {"Nam": True, "Lan": True, "Minh": True,
              "Hoa": False, "Tú": True, "Khanh": True}
so_thu_ba = {"Nam": True, "Lan": True, "Minh": True,
             "Hoa": True, "Tú": True, "Khanh": True}
so_thu_tu = {"Nam": False, "Lan": False, "Minh": False,
             "Hoa": False, "Tú": False, "Khanh": False}

def bang_deo_the(so):
    ra = []
    for ten in thanh_vien:
        ra.append(so[ten])
    return ra

def bang_khong_deo_the(so):
    ra = []
    for ten in thanh_vien:
        ra.append(not so[ten])
    return ra

# "Không phải mọi thành viên đều đeo thẻ."
def khong_phai_moi_nguoi_deo(so):
    return ___

# "Tồn tại một thành viên không đeo thẻ."
def ton_tai_nguoi_khong_deo(so):
    return ___

print(khong_phai_moi_nguoi_deo(so_thu_hai), ton_tai_nguoi_khong_deo(so_thu_hai))
print(khong_phai_moi_nguoi_deo(so_thu_ba), ton_tai_nguoi_khong_deo(so_thu_ba))
print(khong_phai_moi_nguoi_deo(so_thu_tu), ton_tai_nguoi_khong_deo(so_thu_tu))
```

```python title=solution
thanh_vien = ["Nam", "Lan", "Minh", "Hoa", "Tú", "Khanh"]

so_thu_hai = {"Nam": True, "Lan": True, "Minh": True,
              "Hoa": False, "Tú": True, "Khanh": True}
so_thu_ba = {"Nam": True, "Lan": True, "Minh": True,
             "Hoa": True, "Tú": True, "Khanh": True}
so_thu_tu = {"Nam": False, "Lan": False, "Minh": False,
             "Hoa": False, "Tú": False, "Khanh": False}

def bang_deo_the(so):
    ra = []
    for ten in thanh_vien:
        ra.append(so[ten])
    return ra

def bang_khong_deo_the(so):
    ra = []
    for ten in thanh_vien:
        ra.append(not so[ten])
    return ra

# "Không phải mọi thành viên đều đeo thẻ."
def khong_phai_moi_nguoi_deo(so):
    return not all(bang_deo_the(so))

# "Tồn tại một thành viên không đeo thẻ."
def ton_tai_nguoi_khong_deo(so):
    return any(bang_khong_deo_the(so))

print(khong_phai_moi_nguoi_deo(so_thu_hai), ton_tai_nguoi_khong_deo(so_thu_hai))
print(khong_phai_moi_nguoi_deo(so_thu_ba), ton_tai_nguoi_khong_deo(so_thu_ba))
print(khong_phai_moi_nguoi_deo(so_thu_tu), ton_tai_nguoi_khong_deo(so_thu_tu))
```

```python title=test
# Câu đứng đầu canh cái bẫy lớn nhất của bài: cuốn sổ thứ Ba là cuốn DUY NHẤT
# mà cả hai câu cùng SAI. Trả bừa `True` cho mọi cuốn sổ thì chỉ mình nó sập
# bẫy, nên nó phải chạy TRƯỚC — xếp sau một câu `==` khác mà câu ấy trượt
# trước thì bẫy không bao giờ sập.
assert khong_phai_moi_nguoi_deo(so_thu_ba) is False, "sổ thứ Ba: cả sáu người đều đeo thẻ, nên câu `mọi thành viên đều đeo thẻ` ĐÚNG, và phủ định của nó phải sai"
assert ton_tai_nguoi_khong_deo(so_thu_ba) is False, "sổ thứ Ba: không có ai không đeo thẻ, nên câu `tồn tại một thành viên không đeo thẻ` phải sai"
assert khong_phai_moi_nguoi_deo(so_thu_hai) is True, "sổ thứ Hai: Hoa quên thẻ, nên câu `mọi thành viên đều đeo thẻ` sai, và phủ định của nó phải đúng"
assert ton_tai_nguoi_khong_deo(so_thu_hai) is True, "sổ thứ Hai: Hoa là người không đeo thẻ, nên câu `tồn tại` phải đúng"
assert khong_phai_moi_nguoi_deo(so_thu_tu) is True, "sổ thứ Tư: cả sáu người đều quên thẻ, nên câu `mọi thành viên đều đeo thẻ` sai, và phủ định của nó phải đúng"
assert ton_tai_nguoi_khong_deo(so_thu_tu) is True, "sổ thứ Tư: có tới sáu người không đeo thẻ, mà một người đã đủ, nên câu `tồn tại` phải đúng"
# Điều bài hôm nay khẳng định, hỏi thẳng trên cả ba cuốn sổ: hai câu ấy là MỘT.
assert khong_phai_moi_nguoi_deo(so_thu_hai) == ton_tai_nguoi_khong_deo(so_thu_hai), "trên sổ thứ Hai, hai câu phải cho CÙNG một giá trị — chúng là hai cách viết của một câu"
assert khong_phai_moi_nguoi_deo(so_thu_ba) == ton_tai_nguoi_khong_deo(so_thu_ba), "trên sổ thứ Ba, hai câu phải cho CÙNG một giá trị — chúng là hai cách viết của một câu"
assert khong_phai_moi_nguoi_deo(so_thu_tu) == ton_tai_nguoi_khong_deo(so_thu_tu), "trên sổ thứ Tư, hai câu phải cho CÙNG một giá trị — chúng là hai cách viết của một câu"
```

:::hints
- kind: attention
  body: Mỗi hàm nhận một cuốn sổ tên là `so`, và hai hàm dựng danh sách ở ngay phía trên đều nhận đúng cuốn sổ ấy — nhìn xem hàm nào chép nguyên, hàm nào lật từng ô. Rồi đọc lại đầu bài: câu thứ nhất là câu "với mọi" bị phủ định, câu thứ hai là câu "tồn tại".
- kind: strategy
  body: "Câu \"với mọi\" viết bằng `all` chạy trên danh sách của câu mở `người ấy đeo thẻ`; muốn nói \"KHÔNG PHẢI với mọi\" thì bọc cả cái đó lại bằng công cụ lật giá trị của bài 3. Câu \"tồn tại\" viết bằng `any`, nhưng nó nói về câu mở đã bị lật — `người ấy KHÔNG đeo thẻ` — nên nó phải chạy trên danh sách kia."
- kind: one-line
  body: "Hàm thứ nhất trả `not all(bang_deo_the(so))`; hàm thứ hai trả `any(bang_khong_deo_the(so))`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi câu phải viết bằng đúng lượng từ của nó — một `all` bị phủ định và một `any` — vì cả bài dựng lên để cho thấy hai cách viết ấy trùng nhau, mà viết cùng một cách hai lần thì chẳng có gì để đối chiếu
  requireAst:
  # Khung khởi đầu không gọi `all` hay `any` lần nào. Hai luật này đòi cả hai
  # lượng từ cùng có mặt, nên đáp án viết một kiểu hai lần bị chặn ở đây.
  - kind: uses-call, target: all, min: 1
  - kind: uses-call, target: any, min: 1
  # Hai hàm dựng danh sách phải được GỌI, không phải chép tay lại sáu giá trị.
  # Khung khởi đầu định nghĩa chúng mà không gọi lần nào.
  - kind: uses-call, target: bang_deo_the, min: 1
  - kind: uses-call, target: bang_khong_deo_the, min: 1
  # Chữ "không phải" phải được viết ra. Khung khởi đầu có đúng 1 dấu `not`
  # (trong thân `bang_khong_deo_the`); lời giải có 2 — chỗ thứ hai chính là
  # chữ "không phải" đứng ngoài `all`.
  - kind: uses-operator, target: not, min: 2
  # Hai hàm phải chạy trên CUỐN SỔ được đưa vào, không phải trên một cuốn sổ
  # gõ cứng tên. Khung khởi đầu đọc `so` 2 lần (trong hai hàm dựng danh sách);
  # lời giải đọc 4.
  - kind: uses-name, target: so, min: 4
  forbidAst:
  # Chặn đáp án gõ thẳng đáp số thay vì tính. Cả lời giải không có số nguyên
  # nào, nên hai luật này không đụng vào cách viết hợp lệ nào.
  - kind: has-literal, target: 6
  - kind: has-literal, target: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^True True\nFalse False\nTrue True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba cuốn sổ, sáu con số, và hai cột trùng khít nhau. Chữ "không phải" đúng là đi
vào trong thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sáu thành viên thì máy kiểm hết trong một nháy: dựng sáu giá trị Đ/S, gọi `all`
một lần, xong.

Byte đem đúng cách ấy sang một câu khác, câu này nói về **số**:

> Với mọi số tự nhiên `n`, số `n × n + n + 41` là số nguyên tố.

Byte cho máy thử tới `n = 39` — tức là 40 số đầu tiên, kể cả `n = 0`. Không sai
lần nào. `all` trả về `True`.

Đã kết luận được chưa?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
