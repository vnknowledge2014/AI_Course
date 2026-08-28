---
id: toan.logic-va-chung-minh.khi-va-chi-khi
title: Khi và chỉ khi
summary: Ba chữ "và chỉ khi" khẳng định cả hai chiều kéo theo cùng một lúc — và đó là hình dạng của mọi định nghĩa, nên định nghĩa dùng được theo cả hai hướng.
locale: vi
track: toan
module: logic-va-chung-minh
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.biconditional]
requires: [logic.contrapositive, logic.converse, logic.inverse, logic.implication, logic.vacuous-truth, logic.equivalence, logic.tautology, logic.truth-table, logic.negation, logic.conjunction, logic.not, logic.and, core.boolean, core.variable, core.list, core.list-append, core.tuple, core.print-variable, core.function-def, core.function-call, core.function-parameter, core.function-return, ctrl.for-each, ctrl.if]
concepts: [logic.tuong-duong-hai-chieu, logic.hai-nua-cua-mot-cau, logic.dang-cua-dinh-nghia]
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
Dòng của thư viện dài hơn dòng của CLB đúng ba chữ. Ba chữ ấy làm gì?
::::

::::explain{#ba-chu-them-vao}
Bài trước để lại một câu hỏi. Nội quy CLB nói một chiều:

> "Nếu Nam là thành viên thì Nam đeo thẻ."

Còn thư viện trường viết dòng của họ khác hẳn:

> **"Nam được mượn sách khi và chỉ khi Nam có thẻ thư viện."**

Vẫn hai vế, nhưng chữ nối không còn là "nếu... thì". Cách chắc chắn nhất để đọc
ra một chữ nối mới là hỏi đúng câu mà bài 10 đã hỏi: **khi nào thì dòng ấy bị
coi là sai?**

Cô thủ thư kể ra hai kiểu buổi sáng làm cô phải gỡ tờ nội quy xuống:

- **Kiểu thứ nhất.** Có bạn cầm thẻ thư viện hẳn hoi mà bị từ chối, không mượn
  được sách. Dòng nội quy hứa "có thẻ thì mượn được" — lời hứa ấy vỡ.
- **Kiểu thứ hai.** Có bạn không hề có thẻ mà vẫn ôm sách ra khỏi thư viện. Dòng
  nội quy hứa "muốn mượn thì phải có thẻ" — lời hứa này cũng vỡ.

Hai kiểu vỡ, hai lời hứa. Nên dòng của thư viện là **hai** câu "nếu... thì" nói
cùng một lúc, mỗi câu một chiều:

| nửa | câu đầy đủ | dạng |
|---|---|---|
| nửa "khi" | Nếu Nam có thẻ thì Nam được mượn sách. | nếu **P** thì **Q** |
| nửa "chỉ khi" | Nếu Nam được mượn sách thì Nam có thẻ. | nếu **Q** thì **P** |

với **P** — "Nam có thẻ thư viện", **Q** — "Nam được mượn sách".

Nửa thứ hai chính là **câu đảo** của nửa thứ nhất — thứ mà bài 12 đã tách ra
thành một câu riêng vì nó không đi kèm sẵn. Ba chữ "và chỉ khi" là cách tiếng
Việt đòi nó về, và đòi ra mặt.

> **Tương đương hai chiều** (viết "P khi và chỉ khi Q") là câu ghép khẳng định
> **cả** "nếu P thì Q" **lẫn** "nếu Q thì P" cùng một lúc. Nó đúng khi cả hai
> nửa cùng đúng, và sai ngay khi một nửa bị phá.
::::

::::explain{#cot-cua-cau-hai-chieu}
Câu ghép mới thì dựng bảng mới. Hai vế tự do với nhau nên vẫn bốn dòng, và giá
trị của câu ghép tính từ hai nửa: nửa "khi" và nửa "chỉ khi" phải **cùng** đúng
— chữ đòi-cả-hai ấy là chữ "và" của bài 4.

| dòng | P: có thẻ | Q: được mượn | nửa "khi": nếu P thì Q | nửa "chỉ khi": nếu Q thì P | P khi và chỉ khi Q |
|---|---|---|---|---|---|
| 1 | Đ | Đ | Đ | Đ | **Đ** |
| 2 | Đ | S | S | Đ | **S** |
| 3 | S | Đ | Đ | S | **S** |
| 4 | S | S | Đ | Đ | **Đ** |

Cột cuối có một hình dạng dễ nhớ: câu hai chiều **đúng ở đúng những dòng mà hai
vế mang cùng một giá trị**, và sai ở hai dòng mà chúng lệch nhau.

Để ý dòng 4: không có thẻ, và cũng không mượn được sách. Không nửa nào bị phá —
mỗi nửa đều có vế trước sai, mà vế trước sai thì không ai phá được câu, đúng như
bài 11. Nên dòng ấy ghi Đ. Nội quy thư viện không hứa rằng ai cũng có thẻ; nó
chỉ hứa rằng hai chuyện ấy luôn đi cùng nhau.

Đây cũng là chỗ **định nghĩa** sống. Mọi định nghĩa đều viết ở dạng hai chiều,
kể cả khi người viết không gõ ra ba chữ ấy:

> "Một số là **số chẵn** khi và chỉ khi nó chia hết cho 2."

Vì là câu hai chiều nên nó dùng được theo **cả hai hướng**: gặp một số được gọi
là chẵn, bạn rút ra được nó chia hết cho 2; và gặp một số chia hết cho 2, bạn
rút ra được nó là số chẵn. Một câu "nếu... thì" một chiều không cho bạn quyền
thứ hai — bài 12 đã dựng hẳn một bài để nói điều đó.
::::

::::example{#do-lai-hai-bai-truoc}
Công cụ mới đo lại được hai kết quả cũ, và đo bằng một con mắt duy nhất.

Bài 14 nói: câu gốc và câu phản đảo trùng cột. Bài 12 nói: câu gốc và câu đảo
thì không. Giờ đem chính chữ "khi và chỉ khi" nối hai câu ấy lại, rồi nhìn cột
kết quả.

Bốn dòng dưới đây là bốn dòng của bảng bài 14 — bảng về Nam và tấm thẻ CLB, chứ
không phải thẻ thư viện: dòng 1 là Nam vừa là thành viên vừa đeo thẻ, dòng 2 là
thành viên mà không đeo thẻ, dòng 3 là đeo thẻ mà không phải thành viên, dòng 4
là không cái nào cả.

| dòng | câu gốc | câu phản đảo | câu đảo | gốc **khi và chỉ khi** phản đảo | gốc **khi và chỉ khi** đảo |
|---|---|---|---|---|---|
| 1 | Đ | Đ | Đ | Đ | Đ |
| 2 | S | S | Đ | Đ | **S** |
| 3 | Đ | Đ | S | Đ | **S** |
| 4 | Đ | Đ | Đ | Đ | Đ |

Cột áp chót **toàn Đ** ở mọi dòng — đó đúng là chữ **hằng đúng** của bài 7. Cột
cuối thì không: nó ghi S ở hai dòng.

Ghép lại thành một câu gọn, và câu ấy cũng là một câu hai chiều:

> Hai câu trùng cột với nhau **khi và chỉ khi** cột của "câu này khi và chỉ khi
> câu kia" toàn Đ.

Nên "trùng bảng" của bài 8 và "khi và chỉ khi" của hôm nay không phải hai thứ rời
nhau: một cái là chuyện xảy ra giữa hai cột, cái kia là một chữ nối dựng ra một
cột mới — và cột mới ấy toàn Đ đúng lúc chuyện kia xảy ra.
::::

::::predict{#doan-nua-cau commitOnce}
Sáng thứ Sáu, cô thủ thư ghi lại bốn bạn ghé thư viện: ai có thẻ, ai mượn được
sách.

| tên | có thẻ thư viện? | được mượn sách? |
|---|---|---|
| Nam | có | có |
| Lan | có | **không** |
| Minh | **không** | có |
| Hoa | không | không |

Byte gõ mấy dòng để hỏi máy xem ai phá nội quy — nhưng đoạn mã dưới đây mới kiểm
**một nửa** dòng nội quy, nửa "khi": có thẻ thì được mượn.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

# (tên, có thẻ thư viện?, được mượn sách?)
so_thu_vien = [("Nam", True, True), ("Lan", True, False),
               ("Minh", False, True), ("Hoa", False, False)]

bi_pha = []
for ten, co_the, duoc_muon in so_thu_vien:
    if not keo_theo(co_the, duoc_muon):
        bi_pha.append(ten)

print(bi_pha)
```

:::opt{correct}
`['Lan']`
:::

:::opt
`['Lan', 'Minh']`
::why
Gần đúng ở chỗ bạn trả lời trúng câu hỏi **của cô thủ thư**: sáng ấy đúng là có
hai bạn làm dòng nội quy đầy đủ bị phá, Lan ở nửa này và Minh ở nửa kia. Giữ
chặt câu trả lời đó — bước sau của bài chính là bắt máy nói ra nó.

Chỗ lệch: đoạn mã trên chưa hỏi câu hỏi ấy. Nó chỉ gọi `keo_theo(co_the,
duoc_muon)` — đúng một nửa "khi", với "có thẻ" ở vế trước. Với Minh thì vế trước
sai (Minh không có thẻ), mà vế trước sai thì câu "nếu... thì" đúng, không ai phá
được. Nên nửa này để Minh đi qua, và tên Minh không rơi vào danh sách.
::
:::

:::opt
`['Minh']`
::why
Gần đúng ở chỗ bạn khoanh trúng người mà nội quy thư viện thật sự cần chặn: một
bạn không thẻ mà vẫn ôm sách ra là chuyện cô thủ thư lo nhất.

Chỗ lệch nằm ở chỗ nửa nào đang được kiểm. Bắt được Minh là việc của nửa "**chỉ
khi**" — nửa đặt "được mượn sách" ở vế trước và "có thẻ" ở vế sau. Đoạn mã trên
đặt hai vế theo thứ tự ngược lại, nên nó bắt kiểu vi phạm kia: người có thẻ mà
không mượn được, tức là Lan.
::
:::

:::opt
`['Lan', 'Hoa']`
::why
Gần đúng ở chỗ bạn đọc trúng Lan, và ở chỗ bạn để ý tới Hoa — bạn duy nhất không
có gì cả, nên nhìn thì thấy đáng ngờ.

Chỗ lệch: Hoa không có thẻ, nên ở nửa "có thẻ thì được mượn", vế trước của Hoa
sai. Bài 11 đã chốt: vế trước sai thì không phá được câu — người không có thẻ
không làm hỏng một lời hứa dành cho người có thẻ. `keo_theo` trả `True` cho Hoa,
nên tên Hoa không được ghi vào.
::
:::
::::

::::code{#kiem-ca-hai-nua}
Giờ viết dòng nội quy **đầy đủ** của thư viện, đủ cả hai nửa, rồi thả nó lên
cuốn sổ sáng thứ Sáu.

Chỗ trống là thân của `khi_va_chi_khi`. Nó nhận hai giá trị Đ/S của một bạn — có
thẻ hay không, mượn được hay không — và trả về **một** giá trị Đ/S: dòng nội quy
có đứng vững với bạn ấy không.

Viết nó đúng cái định nghĩa vừa dựng: **hai** câu "nếu... thì" viết bằng hàm
`keo_theo`, nối lại bằng chữ đòi-cả-hai của bài 4. Nửa "khi" đặt "có thẻ" ở vế
trước; nửa "chỉ khi" đặt "được mượn" ở vế trước.

Bài chấm bằng **cả bốn dòng sổ**, và bốn dòng ấy được chọn để không nửa nào trốn
được: Lan chỉ bị bắt bởi nửa "khi", Minh chỉ bị bắt bởi nửa "chỉ khi", còn Nam
và Hoa phải đi qua được cả hai nửa. Viết một nửa thôi thì một trong hai người
lọt lưới, và bài đỏ ngay.

```python title=starter
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

# (tên, có thẻ thư viện?, được mượn sách?)
so_thu_vien = [("Nam", True, True), ("Lan", True, False),
               ("Minh", False, True), ("Hoa", False, False)]

def khi_va_chi_khi(co_the, duoc_muon):
    return ___

cot = []
bi_pha = []
for ten, co_the, duoc_muon in so_thu_vien:
    dung = khi_va_chi_khi(co_the, duoc_muon)
    cot.append(dung)
    if not dung:
        bi_pha.append(ten)

print(cot)
print(bi_pha)
```

```python title=solution
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

# (tên, có thẻ thư viện?, được mượn sách?)
so_thu_vien = [("Nam", True, True), ("Lan", True, False),
               ("Minh", False, True), ("Hoa", False, False)]

def khi_va_chi_khi(co_the, duoc_muon):
    return keo_theo(co_the, duoc_muon) and keo_theo(duoc_muon, co_the)

cot = []
bi_pha = []
for ten, co_the, duoc_muon in so_thu_vien:
    dung = khi_va_chi_khi(co_the, duoc_muon)
    cot.append(dung)
    if not dung:
        bi_pha.append(ten)

print(cot)
print(bi_pha)
```

```python title=test
# Bốn câu đầu gọi thẳng hàm với bốn kiểu buổi sáng, không đi qua cuốn sổ. Hai
# câu đầu là hai nửa của dòng nội quy: viết thiếu nửa nào thì đúng câu ấy vỡ,
# và thông điệp nói ra thiếu nửa nào. Xếp chúng sau các câu so danh sách thì
# người học chỉ biết "sai ở đâu đó", không biết nửa nào.
assert khi_va_chi_khi(True, False) == False, "Lan có thẻ mà không được mượn sách: nửa 'có thẻ thì được mượn' bị phá, nên với cặp giá trị này câu hai chiều phải cho Sai"
assert khi_va_chi_khi(False, True) == False, "Minh không có thẻ mà vẫn mượn được sách: nửa 'được mượn thì phải có thẻ' bị phá, nên với cặp giá trị này câu hai chiều phải cho Sai"
assert khi_va_chi_khi(True, True) == True, "Nam có thẻ và mượn được sách: không nửa nào bị phá, nên với cặp giá trị này câu hai chiều phải cho Đúng"
assert khi_va_chi_khi(False, False) == True, "Hoa không có thẻ và cũng không mượn sách: vế trước của cả hai nửa đều sai, nên không nửa nào bị phá và câu hai chiều cho Đúng"
assert cot == [True, False, False, True], "bốn dòng sổ theo đúng thứ tự Nam, Lan, Minh, Hoa cho Đúng, Sai, Sai, Đúng"
assert bi_pha == ["Lan", "Minh"], "sáng thứ Sáu có đúng hai bạn làm dòng nội quy thư viện bị phá: Lan (có thẻ mà không được mượn) và Minh (không thẻ mà vẫn mượn được); Nam và Hoa thì không"
```

:::hints
- kind: attention
  body: Chỗ trống là thân một hàm, nên nó phải trả về **một** giá trị Đ/S cho cả câu hai chiều. Đọc lại bảng hai nửa ở trên: mỗi nửa là một câu "nếu... thì" trọn vẹn, và bạn đã có sẵn `keo_theo` để viết một câu như thế. Hai thứ đang cầm trong tay tên là `co_the` và `duoc_muon`.
- kind: strategy
  body: "Viết riêng từng nửa trước. Nửa \"khi\" đặt `co_the` ở chỗ vế trước và `duoc_muon` ở chỗ vế sau; nửa \"chỉ khi\" đưa vào đúng hai thứ ấy theo thứ tự ngược lại. Câu hai chiều đòi cả hai nửa cùng đúng, mà chữ đòi-cả-hai của bài 4 trong Python là `and`."
- kind: one-line
  body: "`return keo_theo(co_the, duoc_muon) and keo_theo(duoc_muon, co_the)`"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: thân hàm phải là hai câu "nếu... thì" dựng bằng `keo_theo` và nối bằng `and` — viết một nửa thôi, hay gõ cứng một giá trị Đ/S, thì bài đang chấm một thứ nó không hề kiểm
  requireAst:
  # Khung khởi đầu KHÔNG gọi `keo_theo` lần nào. Câu hai chiều là hai nửa, nên
  # phải có hai lời gọi. Luật này một mình chặn cả đáp án gõ cứng `True` lẫn
  # đáp án chỉ viết một nửa.
  - kind: uses-call, target: keo_theo, min: 2
  # Khung khởi đầu có sẵn MỘT `and` (trong thân `keo_theo`). Chữ nối hai nửa là
  # cái thứ hai.
  - kind: uses-operator, target: and, min: 2
  # Mỗi nửa nhắc tới cả hai vế, nên mỗi tên phải được đọc hai lần trong thân
  # hàm. Khung khởi đầu đọc mỗi tên đúng 1 lần (ở lời gọi trong vòng lặp), nên
  # con số 3 đo đúng phần người học viết ra.
  - kind: uses-name, target: co_the, min: 3
  - kind: uses-name, target: duoc_muon, min: 3
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[True, False, False, True\]\n\['Lan', 'Minh'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai nửa, hai người bị bắt. Bỏ nửa nào thì có một người đi lọt.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại toàn bộ chặng đường từ đầu track tới đây: mọi câu bạn đem ra phân xử
đều nói về **một** người có tên. "Nam đã nộp quỹ." "Nam là thành viên." "Nam
được mượn sách khi và chỉ khi Nam có thẻ." Có tên ở đó thì mở sổ ra là biết câu
ấy đúng hay sai.

Nhưng dòng trên tấm bảng nội quy CLB không viết như thế. Nó viết:

> **"Mọi thành viên đều đeo thẻ."**

Dòng ấy không nêu tên ai cả. Từ đầu bài này ta vẫn cứ nói nó là cách viết gọn
sáu câu, mỗi thành viên một câu — nhưng chưa lần nào dừng lại xem chỗ **để trống**
tên người ấy thật ra là cái gì.

Và đây là câu hỏi: riêng mấy chữ

> **"bạn ấy đeo thẻ"**

thì đúng hay sai?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
