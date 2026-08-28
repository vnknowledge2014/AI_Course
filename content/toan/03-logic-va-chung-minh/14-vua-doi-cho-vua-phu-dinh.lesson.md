---
id: toan.logic-va-chung-minh.vua-doi-cho-vua-phu-dinh
title: Vừa đổi chỗ vừa phủ định
summary: Làm cả hai chuyện cùng lúc thì hai cái hỏng bù nhau — câu phản đảo có cột trùng khít cột câu gốc, nên hai câu ấy thay được cho nhau ở mọi chỗ.
locale: vi
track: toan
module: logic-va-chung-minh
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.contrapositive]
requires: [logic.inverse, logic.converse, logic.implication, logic.vacuous-truth, logic.equivalence, logic.truth-table, logic.negation, logic.not, logic.and, core.boolean, core.variable, core.list, core.list-append, core.tuple, core.print-variable, core.function-def, core.function-call, core.function-parameter, core.function-return, ctrl.for-each, ctrl.nested-loop, ctrl.if]
concepts: [logic.phan-dao, logic.hai-cai-hong-bu-nhau, logic.thay-duoc-cho-nhau]
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
Đổi chỗ ra câu khác. Phủ định ra câu khác. Làm cả hai một lượt xem sao.
::::

::::explain{#hai-cai-hong}
Hai bài vừa rồi để lại một câu hỏi: hai cách sửa đều làm hỏng, nhưng làm **cả
hai** cùng lúc thì hai cái hỏng có bù nhau không?

Vẫn câu về Nam, vẫn hai vế cũ:

- **P** — "Nam là thành viên CLB."
- **Q** — "Nam đeo thẻ."

Vừa đổi chỗ hai vế, vừa phủ định chúng, thì được câu này:

> **"Nếu Nam không đeo thẻ thì Nam không phải thành viên."** — nếu **không Q**
> thì **không P**.

Nó cũng có tên riêng:

> **Mệnh đề phản đảo** của một câu "nếu... thì" là câu dựng bằng cách **đổi chỗ
> hai vế và phủ định cả hai**.

Cái tên ghép từ hai chữ đã có, và ghép theo thứ tự nào cũng ra đúng câu ấy: lấy
câu **đảo** ("nếu Q thì P") rồi phủ định hai vế của nó, ta được "nếu không Q thì
không P"; lấy câu **phản** ("nếu không P thì không Q") rồi đổi chỗ hai vế, ta
cũng được "nếu không Q thì không P". Hai đường, một câu.
::::

::::explain{#dung-bang-lan-nua}
Bảng vẫn bốn dòng, vẫn hai vế tự do với nhau, và luật chấm vẫn là luật bài 10:
câu "nếu... thì" sai ở đúng một dòng — vế trước đúng mà vế sau sai.

| dòng | P: Nam là thành viên | Q: Nam đeo thẻ | không Q | không P | câu gốc | câu đảo | câu phản đảo |
|---|---|---|---|---|---|---|---|
| 1 | Đ | Đ | S | S | Đ | Đ | Đ |
| 2 | Đ | S | Đ | S | **S** | Đ | **S** |
| 3 | S | Đ | S | Đ | Đ | **S** | Đ |
| 4 | S | S | Đ | Đ | Đ | Đ | Đ |

Dòng 2 là dòng đáng đọc chậm. Nam **là** thành viên mà **không** đeo thẻ:

- **Câu gốc** ở dòng 2: vế trước đúng, vế sau sai. Câu gốc **sai** — đây là dòng
  duy nhất phá được nội quy, đúng như bài 10 đã chốt.
- **Câu phản đảo** ở dòng 2: vế trước của nó là "Nam không đeo thẻ", dòng 2 ghi
  Đ cho câu ấy. Vế sau của nó là "Nam không phải thành viên", dòng 2 ghi S. Vế
  trước đúng, vế sau sai. Câu phản đảo cũng **sai**.

Cùng một dòng làm hỏng cả hai. Và ba dòng còn lại thì cả hai cùng đúng. Đọc cả
cột:

| | dòng 1 | dòng 2 | dòng 3 | dòng 4 |
|---|---|---|---|---|
| câu gốc | Đ | S | Đ | Đ |
| câu phản đảo | Đ | S | Đ | Đ |

Trùng khít. Bài 8 đã cấp cho chuyện này một cái quyền: hai câu trùng bảng thì
**thay được cho nhau** ở bất kỳ chỗ nào mà không đổi giá trị của câu bao ngoài.

Vậy hai cái hỏng đúng là bù nhau. Câu đảo lệch khỏi câu gốc; câu phản cũng lệch
khỏi câu gốc; mà làm cả hai một lượt thì về lại đúng chỗ cũ.
::::

::::example{#thay-mot-nguoi-khong-deo-the}
Cái quyền vừa nhận được dùng để làm gì? Đứng ở cửa phòng CLB một buổi sáng thì
thấy.

Nội quy về Nam **đúng** — thầy phụ trách dán nó lên bảng và cả CLB không ai cãi.
Câu ấy đúng nghĩa là sáng nay ta đang đứng ở một dòng mà cột "câu gốc" ghi Đ:
dòng 1, dòng 3 hoặc dòng 4. Dòng 2 bị loại.

**Tình huống một.** Thầy nhìn thấy Nam **không** đeo thẻ. Trong ba dòng còn lại,
dòng nào có cột Q ghi S? Chỉ dòng 4. Mà ở dòng 4, cột P cũng ghi S. Kết luận:
Nam không phải thành viên CLB. Một dòng còn lại, một câu trả lời chắc chắn.

**Tình huống hai.** Thầy nhìn thấy Nam **đang** đeo thẻ. Trong ba dòng còn lại,
dòng nào có cột Q ghi Đ? Dòng 1 và dòng 3. Ở dòng 1 thì Nam là thành viên, ở
dòng 3 thì không. Hai dòng cùng đứng được, nên không kết luận được gì — đúng
điều bài 11 đã cảnh báo, và đúng lý do bài 12 phải tách câu đảo ra thành một câu
riêng.

Hai tình huống, hai kết cục khác hẳn nhau. Và chỗ khác nhau ấy có tên: tình
huống một là **câu phản đảo** đang làm việc. Nội quy đúng thì câu phản đảo của
nó cũng đúng — cùng một cột mà — nên thấy "không đeo thẻ" là suy ra được "không
phải thành viên", không phải kiểm gì thêm.
::::

::::predict{#doan-cot-phan-dao commitOnce}
Byte để máy đi tìm **dòng lệch**: dòng nào mà câu gốc và câu phản đảo cho hai
giá trị khác nhau. Có dòng nào như thế thì hai câu ấy không trùng bảng.

`bon_dong` là bốn dòng của bảng, viết ra thành danh sách: mỗi dòng là một cặp
(Nam là thành viên?, Nam đeo thẻ?).

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

bon_dong = [(True, True), (True, False), (False, True), (False, False)]

lech = []
for p, q in bon_dong:
    goc = keo_theo(p, q)
    phan_dao = keo_theo(not q, not p)
    if goc != phan_dao:
        lech.append((p, q))

print(lech)
print(len(lech))
```

:::opt{correct}
`[]` rồi `0`
:::

:::opt
`[(True, False), (False, True)]` rồi `2`
::why
Gần đúng ở chỗ bạn nhớ chính xác hai dòng mà câu gốc bị lệch — dòng 2 và dòng 3
đúng là chỗ **câu đảo** và **câu phản** tách khỏi câu gốc, và đó là kết quả thật
của hai bài trước.

Chỗ lệch: câu trong đoạn mã này không phải câu đảo, cũng không phải câu phản.
`keo_theo(not q, not p)` đặt "không Q" ở vế trước và "không P" ở vế sau — vừa
đổi chỗ, vừa phủ định. Ở dòng 2 (Nam là thành viên, không đeo thẻ) câu ấy có vế
trước đúng và vế sau sai, nên nó **sai** — mà câu gốc ở dòng ấy cũng sai. Hai
câu cùng sai thì không tính là lệch.
::
:::

:::opt
`[(True, False)]` rồi `1`
::why
Gần đúng ở chỗ bạn khoanh trúng dòng quan trọng nhất của bảng: dòng 2 là dòng
duy nhất làm câu gốc sai, nên nếu có chỗ nào hai câu tách nhau thì đó là chỗ
đáng ngờ nhất.

Chỗ lệch: ở dòng 2, câu phản đảo cũng sai. Vế trước của nó là "Nam không đeo
thẻ" — dòng 2 ghi Đ cho câu ấy; vế sau là "Nam không phải thành viên" — dòng 2
ghi S. Vế trước đúng, vế sau sai, nên câu phản đảo sai theo đúng luật bài 10.
Hai câu cùng sai ở dòng ấy, và `!=` chỉ ghi vào danh sách khi hai bên **khác**
nhau.
::
:::

:::opt
`[]` rồi `[]`
::why
Gần đúng ở chỗ bạn đọc trúng dòng đầu: không dòng nào lệch, nên danh sách in ra
rỗng thật.

Chỗ lệch nằm ở dòng thứ hai. `len` không trả về một danh sách — nó **đếm** xem
danh sách có bao nhiêu phần tử và trả về một con số. Danh sách rỗng có không
phần tử, nên `len` của nó là `0`, và máy in ra đúng chữ số ấy.
::
:::
::::

::::code{#dung-cot-phan-dao}
Giờ đến lượt bạn dựng cuộc so sánh đầy đủ trên cả bốn dòng: một cột cho câu phản
đảo để đặt cạnh cột câu gốc, và một danh sách gom những dòng mà câu **đảo** lệch
khỏi câu gốc.

Hai chỗ trống:

1. Sau `dao =` — câu đảo của bài 12: giữ nguyên hai vế, đổi chỗ chúng.
2. Sau `phan_dao =` — câu phản đảo: đổi chỗ hai vế **và** phủ định cả hai. Viết
   nó bằng `keo_theo` và bằng `not` cho mỗi vế bị phủ định.

Hai chỗ ấy được chấm bằng hai đường khác nhau, nên không chỗ nào trốn được: câu
đảo bị đo qua danh sách dòng lệch, câu phản đảo bị đo qua cả một cột bốn ô. Chép
cùng một câu vào cả hai chỗ thì một trong hai đường vỡ ngay.

```python title=starter
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

cot_goc = []       # "nếu Nam là thành viên thì Nam đeo thẻ"
cot_phan_dao = []  # "nếu Nam không đeo thẻ thì Nam không phải thành viên"
lech_voi_dao = []  # dòng nào câu gốc và câu ĐẢO cho hai giá trị khác nhau

for p in [True, False]:
    for q in [True, False]:
        goc = keo_theo(p, q)
        dao = ___
        phan_dao = ___
        cot_goc.append(goc)
        cot_phan_dao.append(phan_dao)
        if goc != dao:
            lech_voi_dao.append((p, q))

print(cot_goc)
print(cot_phan_dao)
print(lech_voi_dao)
```

```python title=solution
def keo_theo(truoc, sau):
    if truoc and not sau:
        return False
    return True

cot_goc = []       # "nếu Nam là thành viên thì Nam đeo thẻ"
cot_phan_dao = []  # "nếu Nam không đeo thẻ thì Nam không phải thành viên"
lech_voi_dao = []  # dòng nào câu gốc và câu ĐẢO cho hai giá trị khác nhau

for p in [True, False]:
    for q in [True, False]:
        goc = keo_theo(p, q)
        dao = keo_theo(q, p)
        phan_dao = keo_theo(not q, not p)
        cot_goc.append(goc)
        cot_phan_dao.append(phan_dao)
        if goc != dao:
            lech_voi_dao.append((p, q))

print(cot_goc)
print(cot_phan_dao)
print(lech_voi_dao)
```

```python title=test
# Hai câu đầu đo hai chỗ trống bằng hai đường khác nhau, và chúng đứng TRƯỚC vì
# chúng canh cái bẫy chép-một-câu-vào-cả-hai-chỗ. Xếp chúng sau thì một câu `==`
# ở dưới trượt trước, người học sửa theo thông điệp của câu ấy, và cái bẫy
# không bao giờ sập.
assert lech_voi_dao == [(True, False), (False, True)], "câu gốc và câu đảo lệch nhau ở đúng hai dòng: dòng Nam là thành viên mà không đeo thẻ, và dòng Nam đeo thẻ mà không phải thành viên"
assert cot_phan_dao == [True, False, True, True], "câu phản đảo chỉ sai ở dòng thứ hai — ở đó vế trước của nó (Nam không đeo thẻ) đúng, mà vế sau của nó (Nam không phải thành viên) sai; ba dòng còn lại nó đúng"
assert cot_goc == cot_phan_dao, "đây là chỗ bất ngờ của bài: trên cả bốn dòng, câu gốc và câu phản đảo cho cùng một giá trị — lệch nhau nghĩa là chỗ trống thứ hai chưa viết đúng câu phản đảo"
assert cot_goc == [True, False, True, True], "câu gốc chỉ sai ở dòng thứ hai — Nam là thành viên (vế trước đúng) mà không đeo thẻ (vế sau sai)"
assert len(lech_voi_dao) == 2, "hai dòng lệch ấy phải được ghi lại đủ cả hai, mỗi dòng một lần"
```

:::hints
- kind: attention
  body: Cả hai chỗ trống đều là **một câu "nếu... thì"** viết bằng `keo_theo`, giống hệt dòng `goc = keo_theo(p, q)` ngay trên chúng — khác ở chỗ cái gì được đặt vào vế trước. Đọc lại chú thích bên phải hai cột để biết mỗi câu bắt đầu bằng vế nào. Trong tay mỗi lượt có `p` (câu "Nam là thành viên") và `q` (câu "Nam đeo thẻ").
- kind: strategy
  body: "Câu đảo đưa hai thứ vào `keo_theo` theo thứ tự ngược với câu gốc, và không thêm chữ nào. Câu phản đảo cũng đi theo thứ tự ngược ấy, nhưng mỗi thứ đưa vào phải được phủ định trước — chữ \"không\" của bài 3 viết là `not` đặt trước tên."
- kind: one-line
  body: "Chỗ thứ nhất là `keo_theo(q, p)`; chỗ thứ hai là `keo_theo(not q, not p)`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: cả hai chỗ trống phải là câu "nếu... thì" dựng bằng `keo_theo`, và chỗ thứ hai phải phủ định cả hai vế — gõ cứng một giá trị Đ/S thì cột phản đảo ra một cột đứng yên, và chuyện "trùng khít cột câu gốc" không được chứng minh ở đâu cả
  requireAst:
  # Khung khởi đầu gọi `keo_theo` một lần (câu gốc). Câu đảo và câu phản đảo là
  # lời gọi thứ hai và thứ ba.
  - kind: uses-call, target: keo_theo, min: 3
  # Khung khởi đầu có sẵn MỘT `not` (trong thân `keo_theo`). Câu phản đảo cần
  # thêm hai `not` nữa, mỗi vế một cái. Thiếu chúng thì chỗ trống thứ hai đang
  # chứa một câu không hề phủ định gì.
  - kind: uses-operator, target: not, min: 3
  # Khung khởi đầu đọc `p` hai lần (lời gọi câu gốc và lần ghi cặp vào danh
  # sách); hai câu người học viết thêm hai lần nữa. Dòng `for p in ...` là gán,
  # không phải đọc.
  - kind: uses-name, target: p, min: 4
  - kind: uses-name, target: q, min: 4
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[True, False, True, True\]\n\[True, False, True, True\]\n\[\(True, False\), \(False, True\)\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cột trùng khít, mà cột kia lệch hai dòng. Hai cái hỏng bù nhau thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Câu gốc và câu phản đảo luôn đi cùng nhau: cùng cột, nên buổi nào cũng cùng đúng
hoặc cùng sai. Câu gốc và câu đảo thì không — chúng tách nhau ở hai dòng.

Nhưng "không luôn đi cùng nhau" chưa phải là "không bao giờ cùng đúng". Có những
buổi mà **cả câu gốc lẫn câu đảo** cùng đúng: nhìn lại bảng, dòng 1 và dòng 4 là
hai dòng như thế. Nếu ai đó muốn khẳng định rằng chuyện ấy luôn xảy ra — rằng cả
hai chiều đều đúng, không chỉ một chiều — thì họ nói câu gì?

Thư viện trường có một dòng nội quy viết khác hẳn dòng của CLB:

> **"Được mượn sách khi và chỉ khi có thẻ."**

Ba chữ "và chỉ khi" ấy thêm vào điều gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
