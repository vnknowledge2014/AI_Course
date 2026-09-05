---
id: tri-tue-nhan-tao.token-hoa-bpe.gop-cap-pho-bien-nhat
title: "Gộp cặp phổ biến nhất: một vòng merge của BPE"
summary: "Cài gop_cap: thay MỌI lần xuất hiện của cặp ('n', ' ') (phổ biến nhất, tần suất 6, đo ở bài trước) bằng một token mới 'n '. Corpus 45 token giảm còn 39 token sau ĐÚNG một vòng merge -- mỗi lần gộp một cặp thành một token thì số token giảm đúng bằng tần suất cặp đó (6 lần gộp, mỗi lần bớt 1 token thừa). Có boundary-case: cặp nằm ở CUỐI danh sách, không có phần tử kế tiếp -- hàm phải an toàn, không IndexError."
locale: vi
track: tri-tue-nhan-tao
module: token-hoa-bpe
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.gop-cap-pho-bien-nhat]
requires: [ai.dem-cap-lien-ke]
concepts: [ai.gop-cap-pho-bien-nhat]
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

::::byte{trigger=enter mood=curious pose=point-editor}
Bài trước tìm ra cặp phổ biến nhất — `('n', ' ')`, tần suất `6`. Giờ tới
phần THỰC HIỆN: ghép cặp đó thành một token mới, ở MỌI nơi nó xuất hiện.
Đây là ĐÚNG một vòng "merge" của BPE.
::::

::::explain{#mot_vong_merge}
Gộp một cặp `(a, b)` thành một token mới nghĩa là: quét qua danh sách
token hiện tại, và bất cứ chỗ nào thấy `a` NGAY SAU ĐÓ là `b` (hai vị trí
liền kề), thay CẢ HAI bằng một token DUY NHẤT — nối `a` và `b` lại
(`a + b`).

Có một điểm CẦN CẨN THẬN: sau khi gộp một cặp, phải NHẢY QUA cả hai vị trí
đã gộp (`i += 2`), không chỉ nhảy một bước — nếu không, vị trí thứ hai
(`b`, đã bị "nuốt" vào token mới) sẽ bị xét lại một lần nữa, sai hoàn
toàn. Và trước khi so sánh `danh_sach[i]` với `danh_sach[i + 1]`, phải
kiểm tra `i` KHÔNG PHẢI vị trí CUỐI CÙNG của danh sách — vị trí cuối không
có ai đứng sau nó, so sánh `danh_sach[i + 1]` ở đó sẽ vượt ra ngoài danh
sách.

Kết quả trực tiếp quan sát được: **số lượng token trong corpus GIẢM ĐI**
sau mỗi vòng merge — hai token cũ nhập lại thành một, nên tổng số token
giảm đúng bằng số lần cặp đó được gộp.
::::

::::example{#gop_cap_tren_corpus}
Gộp cặp `('n', ' ')` (tần suất `6`, tìm được ở bài trước) trên đúng corpus
`45` ký tự:

```python title=readonly
def gop_cap(danh_sach, cap):
    a, b = cap
    token_moi = a + b
    ra = []
    i = 0
    while i < len(danh_sach):
        if i < len(danh_sach) - 1 and danh_sach[i] == a and danh_sach[i + 1] == b:
            ra.append(token_moi)
            i += 2
        else:
            ra.append(danh_sach[i])
            i += 1
    return ra

corpus = "con meo an ca, con cho an com, con ga an thoc"
danh_sach = list(corpus)
print("so token TRUOC khi gop:", len(danh_sach))

danh_sach_sau = gop_cap(danh_sach, ('n', ' '))
print("so token SAU khi gop:", len(danh_sach_sau))
print(danh_sach_sau)
```

```text title=readonly
so token TRUOC khi gop: 45
so token SAU khi gop: 39
['c', 'o', 'n ', 'm', 'e', 'o', ' ', 'a', 'n ', 'c', 'a', ',', ' ', 'c', 'o', 'n ', 'c', 'h', 'o', ' ', 'a', 'n ', 'c', 'o', 'm', ',', ' ', 'c', 'o', 'n ', 'g', 'a', ' ', 'a', 'n ', 't', 'h', 'o', 'c']
```

`45` token trước, `39` token sau — giảm đúng `6`, KHỚP với tần suất
`6` lần xuất hiện của cặp `('n', ' ')` đã đếm được ở bài trước. Mỗi lần
gộp biến HAI token cũ (`'n'` và `' '`) thành MỘT token mới (`'n '`, có
khoảng trắng bên trong) — nhìn vào danh sách kết quả, `'n '` giờ xuất hiện
như một khối duy nhất, thay vì hai ký tự tách rời.
::::

::::predict{#doan_so_token_giam commitOnce}
Giả sử thay vì gộp cặp phổ biến nhất `('n', ' ')` (tần suất `6`), ta gộp
cặp `('c', 'o')` (tần suất `4`, đo được ở bài trước) trên CÙNG corpus `45`
ký tự này.

**Trước khi chạy thử**, bạn đoán: số token SAU khi gộp `('c', 'o')` sẽ là
bao nhiêu?

:::opt{correct}
`41` — vì cặp `('c', 'o')` có tần suất `4`, mỗi lần gộp giảm đúng `1`
token (hai token cũ nhập một), nên tổng số token giảm đúng `4`, từ `45`
xuống còn `41`
:::

:::opt
`39` — giống hệt kết quả gộp `('n', ' ')`, vì bất kể gộp cặp nào, số token
giảm luôn là MỘT con số cố định
::why
Gần đúng ở việc cả hai phép gộp ĐỀU làm giảm số token — quan sát chung đó
đúng.

Chỗ lệch: lượng GIẢM phụ thuộc trực tiếp vào TẦN SUẤT của cặp được chọn,
không phải một con số cố định chung cho mọi cặp. `('n', ' ')` có tần suất
`6` nên giảm `6`; `('c', 'o')` có tần suất CHỈ `4` (ít hơn) nên giảm ÍT
hơn — đúng `4`, không phải `6`.
::
:::

:::opt
`45` — số token không đổi, vì gộp một cặp chỉ ĐỔI TÊN hai token thành một
tên khác, không thực sự XOÁ token nào khỏi danh sách
::why
Gần đúng ở việc "gộp" nghe như một phép biến đổi TẠI CHỖ, không nhất thiết
làm ngắn danh sách — trực giác đó hợp lý ở cái nhìn đầu tiên.

Chỗ lệch: gộp không phải "đổi tên", nó THAY THẾ HAI phần tử bằng MỘT phần
tử — danh sách kết quả có ÍT HƠN so với danh sách gốc đúng bằng số lần
thay thế đã xảy ra. Ví dụ tối giản: danh sách `['a', 'b', 'c']` gộp cặp
`('a', 'b')` thành `['ab', 'c']` — từ `3` phần tử còn `2`, giảm đúng `1`,
không giữ nguyên `3`.
::
:::
::::

::::code{#viet_gop_cap}
Hoàn thiện `gop_cap`: kiểm tra vị trí `i` KHÔNG PHẢI vị trí cuối cùng
trước khi so sánh với vị trí kế tiếp, và nhảy đúng `2` bước sau khi gộp
thành công.

```python title=starter
def gop_cap(danh_sach, cap):
    a, b = cap
    token_moi = a + b
    ra = []
    i = 0
    while i < len(danh_sach):
        if i ___ len(danh_sach) - 1 and danh_sach[i] == a and danh_sach[i + 1] == b:   # <
            ra.append(token_moi)
            i += ___                                                                   # 2
        else:
            ra.append(danh_sach[i])
            i += 1
    return ra

corpus = "con meo an ca, con cho an com, con ga an thoc"
danh_sach = list(corpus)
danh_sach_sau = gop_cap(danh_sach, ('n', ' '))

print(len(danh_sach))
print(len(danh_sach_sau))
```

```python title=solution
def gop_cap(danh_sach, cap):
    a, b = cap
    token_moi = a + b
    ra = []
    i = 0
    while i < len(danh_sach):
        if i < len(danh_sach) - 1 and danh_sach[i] == a and danh_sach[i + 1] == b:
            ra.append(token_moi)
            i += 2
        else:
            ra.append(danh_sach[i])
            i += 1
    return ra

corpus = "con meo an ca, con cho an com, con ga an thoc"
danh_sach = list(corpus)
danh_sach_sau = gop_cap(danh_sach, ('n', ' '))

print(len(danh_sach))
print(len(danh_sach_sau))
```

```python title=test
assert len(danh_sach) == 45, f"so token truoc gop phai la 45 -- dang ra {len(danh_sach)}"
assert len(danh_sach_sau) == 39, f"so token sau gop phai la 39 -- dang ra {len(danh_sach_sau)}"
assert danh_sach_sau[2] == 'n ', f"vi tri thu 3 (index 2) phai la token da gop 'n ' -- dang ra {danh_sach_sau[2]!r}"

# doi chieu tu tay: "aabb" gop cap ('a','a') phai ra ['aa', 'b', 'b']
assert gop_cap(list("aabb"), ('a', 'a')) == ['aa', 'b', 'b'], f"gop cap tren 'aabb' sai -- dang ra {gop_cap(list('aabb'), ('a', 'a'))}"

# gop mot cap KHONG xuat hien trong danh sach -- phai giu nguyen HET
assert gop_cap(['x', 'y', 'z'], ('a', 'b')) == ['x', 'y', 'z'], "gop mot cap khong xuat hien phai giu nguyen danh sach"

# BIEN: phan tu CUOI CUNG cua danh sach dung bang gia tri DAU cua cap can
# gop, nhung KHONG CO phan tu ke tiep (het danh sach) -- ham dung phai AN
# TOAN, tra ve danh sach KHONG DOI, khong duoc IndexError. Da tu kiem
# chung: neu bo dieu kien "i < len(danh_sach) - 1" (doi thanh "<="), truong
# hop nay lam chuong trinh NEM IndexError that su.
ket_qua_bien = gop_cap(['x', 'y', 'n'], ('n', ' '))
assert ket_qua_bien == ['x', 'y', 'n'], f"truong hop bien (cap o CUOI danh sach, khong co ky tu ke tiep) phai giu nguyen -- dang ra {ket_qua_bien}"

# danh sach rong / 1 phan tu
assert gop_cap([], ('a', 'b')) == [], "danh sach rong phai tra ve danh sach rong"
assert gop_cap(['a'], ('a', 'b')) == ['a'], "danh sach 1 phan tu phai giu nguyen (khong co cap nao de gop)"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ 1 là phép SO SÁNH bảo đảm `i` còn CHỖ để nhìn sang vị trí kế tiếp (`i + 1`) mà không vượt ra ngoài danh sách — dùng `<`, KHÔNG phải `<=` (nếu `i` là vị trí cuối cùng, `i + 1` đã vượt ra ngoài). Chỗ 2 là bước NHẢY sau khi gộp thành công — đã "nuốt" cả hai vị trí `i` và `i + 1` thành một token, nên phải nhảy `2` bước, không phải `1`.
- kind: strategy
  body: 'Chỗ 1: `<`, cho điều kiện `i < len(danh_sach) - 1`. Chỗ 2: `2`, cho `i += 2`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `<` và `2`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: gop_cap phai dung dung toan tu "<" (khong duoc "<=" -- se doc qua vi tri cuoi danh sach, gay IndexError khi cap can gop nam dung o CUOI), VA phai nhay dung 2 buoc sau khi gop thanh cong (khong duoc 1 -- se xet lai vi tri da bi nuot)
  requireAst:
  - kind: uses-operator, target: "<", min: 2
  - kind: has-literal, target: "2", min: 1
  - kind: uses-name, target: token_moi, min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca ba luat qua sach. "<"=2: mot lan trong dieu kien vong
  # while ("i < len(danh_sach)"), mot lan trong dieu kien if (cho trong 1)
  # -- neu cho trong 1 doi thanh "<=", dem "<" tut xuong con 1, bi chan.
  # has-literal "2"=1: gia tri "i += 2" (cho trong 2) -- neu doi thanh "i +=
  # 1" (cheat), has-literal "2" tut ve 0, bi chan RIENG (day la ly do rieng
  # no ton tai). uses-name token_moi=1: doc trong "ra.append(token_moi)" --
  # dam bao token da NOI (a+b) duoc dung, khong phai mot gia tri khac.
  #
  # Cheat "<=" thay "<": TREN CORPUS CHINH cua bai nay KHONG crash va cho
  # CUNG mot ket qua (da tu kiem chung: ky tu cuoi cua corpus la 'c', khac
  # 'n', nen dieu kien so sanh false o vi tri cuoi du dung "<" hay "<=") --
  # day CHINH LA ly do assertion "ket_qua_bien" (danh sach tong hop ep dung
  # bien, ['x','y','n']) ton tai doc lap: da tu kiem chung THAT, cheat "<="
  # NEM IndexError that su tren dung input bien do, dung luc corpus chinh
  # khong lo ra loi gi. Cheat "i += 1" (thay vi 2) lam vi tri thu hai bi xet
  # lai sau khi da gop -- da tu kiem chung: ket qua danh_sach_sau tren
  # corpus nay đổi hoàn toàn khac 39 token, bi bat boi output/tests VA
  # has-literal "2" rieng.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^45\\n39\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`45` xuống `39` — đúng một vòng merge, giảm đúng bằng tần suất cặp được
gộp. Bài sau lặp lại việc này NHIỀU LẦN — vòng lặp BPE đầy đủ.
::::

::::reflect{#nghi-lai}
Một vòng merge làm được đúng một việc: gộp cặp phổ biến nhất thành một
token mới, số token giảm đi tương ứng. Nhưng BPE thật không dừng lại sau
MỘT vòng — nó LẶP LẠI: đếm cặp lại (trên danh sách token MỚI, đã có
`'n '`), tìm cặp phổ biến nhất TIẾP THEO, gộp tiếp — cho tới khi đạt một
kích thước từ vựng mục tiêu. Bài sau cài đặt đúng vòng lặp đó.
::::

::::checkpoint{mastery=0.85}
::::
