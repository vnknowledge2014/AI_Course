---
id: nen-tang.list-dict-set-tuple.cong-don-theo-nhom
title: Cộng dồn theo nhóm
summary: Một sổ tra cứu rỗng làm chỗ cộng dồn — `tong[nhom] = tong.get(nhom, 0) + tien` cho khoá tự mọc ra khi gặp nhóm chưa từng thấy.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.dict-accumulator]
requires: [core.dict-nested-index, core.dict-get-default, core.dict-items, core.for-unpack, core.dict, core.list-of-dicts, core.len, ctrl.for-each, core.fstring]
concepts: [core.so-tra-cuu, core.cong-don, ctrl.lap]
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
Tên nhóm nằm trong sổ, không nằm trong chương trình mình. Nên mình không gõ nó
ra trước được.
::::

::::explain{#hai-loai-ten-bi-nhet-lam-mot}
Bài trước để lại đúng chỗ này: mỗi nhóm một cái tên riêng — `tong_an_uong`,
`tong_xang_xe` — rồi một chuỗi `if` mấy nhánh cộng vào đúng cái tên ấy. Chạy
được, ra đúng số. Nhưng Byte mua vở đi học, sổ mọc ra nhóm `"học phí"`, và
chương trình không có nhánh nào cho nó.

Câu hỏi thật không phải "thêm một nhánh nữa ở đâu". Câu hỏi là vì sao hễ dữ
liệu đổi thì chương trình cũng phải đổi theo.

Nhìn kỹ thì thấy hai loại tên khác hẳn nhau đang bị nhét làm một:

- `"ăn uống"` là **dữ liệu**. Nó nằm trong sổ, sổ mỗi tháng một khác, và bạn
  không cầm nó trong tay lúc ngồi viết chương trình.
- `tong_an_uong` là **một cái tên trong chương trình**. Bạn phải gõ nó ra bằng
  tay, trước khi chạy. Gõ được nó nghĩa là bạn đã biết trước sổ sẽ có nhóm ấy.

Đòi biết trước mọi nhóm sẽ phát sinh là đòi một chuyện không ai làm được. Nên
câu hỏi rút gọn lại thành: **cất một con số dưới một cái tên do dữ liệu quyết
định** thì cất ở đâu?

Bạn đã có đúng một chỗ như thế rồi. Khoá của một `dict` là một giá trị bình
thường — nó lấy thẳng từ trong sổ ra được, và bạn không phải gõ nó lúc viết.
::::

::::explain{#hai-cong-cu-cu-ghep-lai}
Cuốn sổ tổng mà ta cần cũng là một `dict`, nhưng lúc bắt đầu nó chưa có khoá
nào — một sổ tra cứu trắng, viết bằng một cặp ngoặc nhọn rỗng:

```python title=readonly
tong = {}
```

Rồi mỗi khoản trong sổ chi tiêu đóng góp đúng một dòng:

```python title=readonly
tong[nhom] = tong.get(nhom, 0) + khoan["tien"]
```

Dòng ấy không có công cụ nào mới. Nó là hai thứ bạn đã dùng, đặt cạnh nhau.
Máy chạy vế phải trước, y như mọi phép gán từ Realm 0 tới giờ:

1. `tong.get(nhom, 0)` — hỏi sổ tổng: *nhóm này đã cộng được bao nhiêu rồi?*
   Nhóm đã có mặt thì nhận về con số đang giữ. Nhóm chưa từng gặp thì nhận về
   `0`, và chương trình đi tiếp chứ không dừng lại — đúng việc mà lối hỏi có
   mang sẵn câu trả lời dự phòng được dạy để làm.
2. `+ khoan["tien"]` — cộng thêm tiền của khoản đang xét.
3. `tong[nhom] = ...` — ghi con số mới vào đúng khoá ấy. Khoá đã có thì bị sửa
   đè; khoá chưa có thì được **thêm mới** ngay tại đây.

Ghép hai công cụ cũ lại thì được một thứ mà không công cụ nào một mình làm
nổi: một chỗ cộng dồn có **các ngăn tự mọc ra theo dữ liệu**. Gặp nhóm lạ,
`.get` đưa ra `0` để phép cộng có chỗ bắt đầu, rồi phép gán mở luôn ngăn mới.
Bạn không phải viết trước một dòng nào cho nhóm "học phí" cả.

Cả đoạn vẫn đi đúng nhịp ba quen thuộc: **sinh ra trước vòng** (`tong = {}`),
**được chạm trong vòng** (một dòng ở trên), **đọc lại sau vòng** (in báo cáo).
Khác mỗi chuyện lần này thứ được sinh ra không phải một con số, mà một chỗ
chứa có thể mọc thêm ngăn.
::::

::::example{#ca-cuon-so-sau-khoan}
Cuốn sổ bài trước, cộng thêm khoản mua vở ở ngày 17 — chính cái nhóm mà chương
trình cũ bỏ rơi.

```python title=readonly
so = [
    {"ten": "cà phê", "tien": 25000, "ngay": 2, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 60000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 30000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 14, "nhom": "ăn uống"},
    {"ten": "vở", "tien": 20000, "ngay": 17, "nhom": "học phí"},
]

tong = {}

for khoan in so:
    nhom = khoan["nhom"]
    tong[nhom] = tong.get(nhom, 0) + khoan["tien"]

for nhom, tien in tong.items():
    print(f"{nhom}: {tien} đồng")
```

Máy in ra:

```text title=readonly
ăn uống: 80000 đồng
xăng xe: 90000 đồng
học phí: 20000 đồng
```

Dò lại bằng tay cho chắc, vì một bản tổng kết sai thì trông y hệt một bản
đúng:

- **ăn uống** gom cà phê, bún bò và bánh mì: 25000 + 40000 + 15000 = 80000
- **xăng xe** gom xăng với vá lốp: 60000 + 30000 = 90000
- **học phí** chỉ có khoản mua vở: 20000

Cộng cả ba nhóm: 80000 + 90000 + 20000 = 190000 — đúng bằng tổng sáu khoản
trong sổ. Không đồng nào rơi ra ngoài, kể cả tiền mua vở của một nhóm mà lúc
viết chương trình chưa ai nhắc tới.

Hai chỗ đáng dừng lại nhìn:

- **Không dòng nào trong chương trình gõ chữ `"học phí"`.** Khoá ấy đi từ sổ
  chi tiêu vào sổ tổng, qua cái tên `nhom`. Thêm nhóm mới vào sổ thì bản tổng
  kết tự dài thêm một dòng, không ai phải mở chương trình ra sửa.
- **Ba nhóm hiện ra theo thứ tự chúng xuất hiện lần đầu trong sổ** — ăn uống ở
  khoản 1, xăng xe ở khoản 2, học phí mãi khoản 6. Duyệt một `dict` là đi theo
  thứ tự khoá được thêm vào, đúng như lúc bạn duyệt khoá của một sổ tra cứu.
  Đây là tính chất của `dict`, và bạn sẽ thấy nó đáng nhớ ở bài sau.
::::

::::predict{#bo-mat-cho-du-phong commitOnce}
Byte chép lại đoạn cộng dồn nhưng bỏ mất `.get`: viết thẳng `tong[nhom]` ở cả
hai vế.

**Trước khi bấm chạy**, bạn đoán máy làm gì? Nhớ rằng `tong` bắt đầu bằng một
sổ tra cứu rỗng, và khoản đầu tiên thuộc nhóm `"ăn uống"`.

```python title=readonly
so = [
    {"ten": "cà phê", "tien": 25000, "ngay": 2, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 60000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"},
]

tong = {}

for khoan in so:
    nhom = khoan["nhom"]
    tong[nhom] = tong[nhom] + khoan["tien"]

print(tong)
```

:::opt{correct}
Máy dừng ngay ở khoản đầu tiên, báo `KeyError: 'ăn uống'`
:::

:::opt
In ra sổ tổng đúng như bản có `.get`
::why
Gần đúng ở chỗ bạn đọc dòng gán theo đúng nghĩa nó đã được dạy: `tong[nhom] =
…` thì khoá chưa có sẽ được thêm mới. Câu đó vẫn đúng nguyên — nó nói về **vế
trái**.

Chỗ lệch nằm ở thứ tự máy làm việc. Máy tính xong **vế phải** rồi mới đụng tới
vế trái, và vế phải ở đây có `tong[nhom]` — một lệnh **đọc** khoá `"ăn uống"`
trong một sổ tra cứu còn rỗng. Lệnh đọc ấy vấp trước khi phép gán kịp mở ngăn
mới.
::
:::

:::opt
Máy chạy hết, mỗi nhóm giữ đúng tiền của khoản cuối cùng thuộc nhóm ấy
::why
Gần đúng ở chỗ bạn nhớ một chuyện thật: dấu `=` ghi đè chứ không cộng thêm, nên
một dòng viết nhầm thành `tong[nhom] = khoan["tien"]` sẽ cho ra đúng kết quả
bạn mô tả.

Chỗ lệch là dòng trong bài này không phải dòng ấy. Vế phải vẫn còn phép cộng,
và số hạng đầu của phép cộng là `tong[nhom]` — máy phải đọc được nó thì mới
cộng được. Với khoản đầu tiên thì đọc không ra, nên chương trình dừng ngay ở
đó, chưa lượt nào kịp ghi đè lượt nào.
::
:::

:::opt
Máy coi khoá chưa có là `0` và vẫn cộng bình thường
::why
Gần đúng ở chỗ con số `0` đúng là thứ ta cần cho nhóm chưa từng gặp — cả bài
này dựng lên quanh chuyện đó.

Chỗ lệch: `0` ấy không tự có. Nó là **cái bạn đưa cho `.get`** ở chỗ dự phòng,
và `.get` đưa nó ra thay cho việc dừng chương trình. Bỏ `.get` đi thì không
còn ai đưa `0` nữa. Máy tra một khoá không có trong sổ và làm đúng thứ nó vẫn
làm trong trường hợp ấy: kêu lên bằng `KeyError`.
::
:::
::::

::::explain{#vi-sao-la-get}
Có một cách khác cho ra cùng kết quả: hỏi trước rồi mới cộng.

```python title=readonly
if nhom in tong:
    tong[nhom] = tong[nhom] + khoan["tien"]
else:
    tong[nhom] = khoan["tien"]
```

Bốn dòng này chạy đúng, và nếu bạn tự nghĩ ra chúng thì bạn đã hiểu đúng vấn
đề. Nhưng để ý hai nhánh nói cùng một câu bằng hai giọng, và cái tên `nhom`
phải gõ tới bốn lần — bốn chỗ để gõ nhầm.

`tong.get(nhom, 0)` gộp cả hai nhánh thành một câu: *lấy con số đang có, không
có thì lấy `0`*. Con số `0` ở đây có một cái tên đáng nhớ — nó là **giá trị
khởi đầu** của phép cộng dồn, đúng cái `0` mà mọi biến `tong` từ trước tới giờ
đều sinh ra từ đó. Khác biệt duy nhất: trước kia bạn viết nó một lần trước
vòng lặp, giờ mỗi nhóm nhận nó vào đúng lúc nó xuất hiện lần đầu.

> Chỗ dễ vấp: `tong.get(nhom, 0)` **không** tạo ra khoá nào cả. Nó chỉ đọc và
> đưa lại một con số. Sổ tổng chỉ mọc thêm ngăn ở dấu `=` bên vế trái. Viết
> mỗi `tong.get(nhom, 0) + khoan["tien"]` thành một dòng đứng riêng thì máy
> tính ra một con số rồi vứt đi, và hết vòng `tong` vẫn rỗng trơn.
::::

::::code{#tong-theo-nhom}
Cuốn sổ sáu khoản ở trên, và bản tổng kết theo nhóm.

Khung dưới đã có sẵn cuốn sổ, sổ tổng rỗng, vòng duyệt và đoạn in báo cáo. Còn
thiếu đúng **dòng cộng dồn** trong thân vòng.

Nhớ rằng chương trình không được gõ tên nhóm nào ra bằng tay: bản tổng kết
phải tự dài thêm khi sổ có nhóm mới.

```python title=starter
so = [
    {"ten": "cà phê", "tien": 25000, "ngay": 2, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 60000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 30000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 14, "nhom": "ăn uống"},
    {"ten": "vở", "tien": 20000, "ngay": 17, "nhom": "học phí"},
]

tong = {}

for khoan in so:
    nhom = khoan["nhom"]
    ___

for nhom, tien in tong.items():
    print(f"{nhom}: {tien} đồng")
```

```python title=solution
so = [
    {"ten": "cà phê", "tien": 25000, "ngay": 2, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 60000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 30000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 14, "nhom": "ăn uống"},
    {"ten": "vở", "tien": 20000, "ngay": 17, "nhom": "học phí"},
]

tong = {}

for khoan in so:
    nhom = khoan["nhom"]
    tong[nhom] = tong.get(nhom, 0) + khoan["tien"]

for nhom, tien in tong.items():
    print(f"{nhom}: {tien} đồng")
```

```python title=test
# Bốn phép kiểm, mỗi phép chặn một cách làm hỏng khác nhau.
#
# Dòng 1 vỡ khi dòng bạn điền không mở được ngăn nào (sổ tổng còn rỗng) hoặc
# mở nhầm ngăn: lấy `ten` làm khoá thì ra sáu ngăn chứ không phải ba.
# Dòng 2 và 3 vỡ khi dòng bạn điền GHI ĐÈ thay vì cộng dồn — hai nhóm nhiều
# khoản ấy sẽ chỉ còn giữ tiền của khoản cuối cùng thuộc nhóm.
# Dòng 4 là nhóm chỉ có một khoản: nó vẫn đúng cả khi ba dòng trên vỡ, nên
# một mình nó không đủ để chấm. Nó ở đây để bắt trường hợp vòng lặp bỏ sót
# dòng cuối sổ.
assert len(tong) == 3, "sáu khoản trong cuốn sổ này thuộc đúng ba nhóm khác nhau, nên sổ tổng phải có đúng ba khoá"
assert tong["ăn uống"] == 80000, "nhóm ăn uống trong cuốn sổ này có ba khoản, 25000 với 40000 và 15000 — cộng lại phải ra 80000"
assert tong["xăng xe"] == 90000, "nhóm xăng xe trong cuốn sổ này có hai khoản, 60000 và 30000 — cộng lại phải ra 90000"
assert tong["học phí"] == 20000, "nhóm học phí trong cuốn sổ này chỉ có khoản mua vở 20000 đồng, và nó nằm ở dòng cuối sổ"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ngay dưới dòng `nhom = khoan["nhom"]`, nên lúc đó bạn đang cầm hai thứ của lượt này: tên nhóm trong `nhom`, và số tiền trong `khoan["tien"]`. Dòng cần điền cũng là dòng duy nhất chạm được vào `tong`.
- kind: strategy
  body: Viết vế phải trước cho dễ nghĩ: lấy con số nhóm này đang giữ, rồi cộng thêm tiền của khoản đang xét. Chỗ khó là "đang giữ" khi nhóm mới gặp lần đầu — dùng đúng lối hỏi có mang sẵn câu trả lời dự phòng, để nhận về giá trị khởi đầu của phép cộng thay vì dừng chương trình. Xong vế phải rồi thì vế trái chỉ còn là ghi kết quả vào đúng khoá `nhom`.
- kind: one-line
  body: 'Viết `tong[nhom] = tong.get(nhom, 0) + khoan["tien"]` vào chỗ trống, thụt vào bốn dấu cách cho thẳng hàng với dòng ngay trên nó.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  expect: 'ăn uống: 80000 đồng'
- tier: static
  onFail: dòng bạn điền phải lấy tên nhóm và số tiền TỪ lượt đang chạy, không được gõ sẵn tên nhóm hay con số tổng
  requireAst:
  # KHÔNG hỏi `uses-name target: nhom`. Bản trước đòi `min: 2` và luật ấy đánh
  # trượt một lời giải ĐÚNG: người học bỏ qua biến bắc cầu, viết thẳng
  # `tong[khoan["nhom"]] = tong.get(khoan["nhom"], 0) + khoan["tien"]` — chạy
  # đúng, qua cả bốn assert lẫn tier output — mà chỉ đọc `nhom` một lần.
  #
  # Hỏi thẳng thứ thật sự cần: dòng ấy phải GÁN VÀO MỘT Ô của `tong`, và phải
  # lấy tiền từ lượt đang chạy. Cả hai lối viết đều thoả.
  - kind: subscript-assign, target: tong
  # Khung đọc `khoan` một lần, nên chỗ trống vẫn buộc phải chạm vào lượt này.
  - kind: uses-name, target: khoan, min: 2
  forbidAst:
  # Hai con số này KHÔNG có trong sổ — chúng chỉ ra đời sau phép cộng. Gõ
  # thẳng chúng vào là chép đáp án, không phải cộng dồn.
  - kind: has-literal, target: 80000
  - kind: has-literal, target: 90000
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ngăn "học phí" tự mọc ra. Mình không hề gõ chữ đó ở đâu cả.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Trước khi cộng, sếp hỏi tháng này chi vào những **nhóm nào**. Chỉ cần danh
sách tên nhóm thôi, không cần tiền.

Cuốn sổ thật thì dài hơn sáu dòng nhiều — cứ cho là bốn mươi dòng. Duyệt hết
bốn mươi dòng ấy mà `.append` mọi tên nhóm gặp được thì "ăn uống" hiện ra ba
mươi lần trong danh sách. Lọc trùng thì viết thế này:

```python title=readonly
ds = []
for khoan in so:
    nhom = khoan["nhom"]
    if nhom not in ds:
        ds.append(nhom)
```

Chạy được. Nhưng nhìn lại câu `nhom not in ds`: để trả lời được nó, máy phải
**dò từng ô** của `ds` từ đầu — đúng sự thật bạn đã cất đi từ bài hỏi `in`
trên một danh sách. Bốn mươi dòng sổ là bốn mươi lượt dò, mỗi lượt lại dò trên
một danh sách đang dài dần ra.

Có chỗ chứa nào **tự nó** không nhận hai lần không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
