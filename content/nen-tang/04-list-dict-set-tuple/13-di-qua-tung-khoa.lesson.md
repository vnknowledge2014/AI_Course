---
id: nen-tang.list-dict-set-tuple.di-qua-tung-khoa
title: Đi qua từng khoá
summary: Duyệt một cuốn sổ tra cứu bằng `for` thì mỗi lượt cái tên lặp mang một KHOÁ — muốn số tiền thì phải tra ngược `chi[khoa]`.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.dict-iter-keys]
requires: [core.dict, ctrl.for-each, core.list, core.list-append, core.list-index, core.len, core.fstring, core.variable]
concepts: [ctrl.lap, core.ten, core.gia-tri]
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
Mình đi dọc cuốn sổ. Thứ rơi vào tay mình mỗi lượt là cái nhãn.
::::

::::explain{#may-dua-ra-cai-gi}
Bài trước cuốn sổ tra cứu của bạn dày dần lên: có khoá thì sửa, chưa có thì
thêm mới, tất cả bằng một dòng gán. Giờ sổ đã hai chục khoá, và muốn in cả sổ
ra thì viết `for x in chi:` là chuyện tự nhiên nhất trên đời — bạn đã duyệt
danh sách kiểu đó từ Realm 0.

Câu hỏi bài trước để lại: mỗi lượt, `x` mang cái gì? Ba cách hiểu đều nghe lọt
tai — khoá, giá trị, hay cả cặp.

Hình dung cuốn sổ tra cứu là một dãy ngăn kéo. Mỗi ngăn dán một cái nhãn viết
tay ở mặt trước — *sửa xe*, *cà phê* — còn số tiền thì nằm bên trong ngăn.
Bây giờ bạn đi dọc dãy ngăn ấy. Thứ đọc được mà chưa phải mở ngăn nào ra, là
các **nhãn**.

Máy chọn đúng như vậy. `for khoa in chi:` — mỗi lượt cái tên lặp mang một
**khoá**, và chỉ khoá thôi.

Lối đi này có tên đầy đủ: `chi.keys()`, đọc là "cho tôi phần khoá của sổ".
Viết `for khoa in chi:` chỉ là lối viết gọn của `for khoa in chi.keys():` —
hai dòng ấy chạy y hệt nhau. Python cho phép viết gọn vì đi qua khoá là việc
người ta làm nhiều nhất với một cuốn sổ tra cứu.

Và vì trong tay bạn là cái nhãn, muốn biết trong ngăn có bao nhiêu thì phải mở
ngăn ra: `chi[khoa]` — đúng phép tra cứu bạn đã dùng từ lúc cuốn sổ ra đời.
::::

::::example{#ba-cai-nhan}
Cuốn sổ ngắn ba khoản, đủ để dò bằng mắt:

```python title=readonly
chi = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}

for khoa in chi:
    print(khoa)
```

Máy in ra:

```text
sửa xe
cà phê
biếu bà
```

Ba dòng, ba cái nhãn, đúng thứ tự bạn đã ghi vào sổ. Không con số nào hiện ra —
tiền vẫn nằm yên trong ngăn, vì chưa ai mở ngăn nào cả.

Muốn cả tên lẫn tiền thì mở ngăn ngay trong thân vòng, bằng chính cái nhãn vừa
cầm:

```python title=readonly
chi = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}

for khoa in chi:
    print(f"{khoa} hết {chi[khoa]} đồng")
```

Máy in ra:

```text
sửa xe hết 500000 đồng
cà phê hết 25000 đồng
biếu bà hết 300000 đồng
```

Nhìn kỹ chỗ `{chi[khoa]}`. Bên trong cặp ngoặc nhọn của f-string, `khoa` là một
cái tên chứ không phải một chữ cố định — nên **không** có dấu nháy nào quanh
nó. Lượt đầu `khoa` đang mang `"sửa xe"`, nên `chi[khoa]` mở đúng cái ngăn dán
nhãn *sửa xe*.

Mỗi dòng báo cáo vì vậy tốn hai bước: máy đưa nhãn ra, rồi bạn đem nhãn ấy đi
mở ngăn.
::::

::::predict{#cong-cai-nhan commitOnce}
Byte cần tổng tiền cả sổ, và Byte làm theo đúng cái khuôn cộng dồn đã quen: một
cái tên sinh ra từ `0`, mỗi lượt cộng thêm thứ vòng lặp vừa đưa cho.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
chi = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}

tong = 0
for khoa in chi:
    tong = tong + khoa

print(tong)
```

:::opt{correct}
Không con số nào in ra — máy dừng lại ở dòng cộng với `TypeError`
:::

:::opt
825000
::why
Gần đúng ở chỗ con số ấy có thật: `500000 + 25000 + 300000 = 825000`, đúng tổng
cả sổ. Bạn đọc đúng ý định của đoạn code.

Chỗ lệch nằm ở thứ vòng lặp đưa ra. Mỗi lượt `khoa` mang một cái **nhãn**, chứ
không mang số tiền — nhãn thì không cộng vào một con số được. Muốn ra 825000
thì dòng cộng phải mở ngăn trước: `tong = tong + chi[khoa]`.
::
:::

:::opt
3
::why
Gần đúng ở chỗ bạn đếm số lượt rất chuẩn: sổ ba khoá thì vòng chạy ba lượt.

Chỗ lệch là thứ được cộng vào mỗi lượt. Dòng trong thân không phải
`tong = tong + 1` — nó cộng vào chính thứ mà vòng lặp vừa đưa ra. Một cái tên
đếm lượt và một cái tên cộng dồn giá trị trông giống nhau vì cùng bắt đầu từ
`0`, nhưng dòng bên trong thân quyết định chúng là hai loại khác nhau.
::
:::

:::opt
0sửa xecà phêbiếu bà
::why
Gần đúng ở chỗ bạn nhớ rằng dấu `+` đặt giữa hai chuỗi thì nối chúng lại —
Realm 0 đã dạy đúng như vậy, và nếu mọi thứ trong phép cộng đều là chữ thì màn
hình sẽ ra một dòng dính liền y như bạn viết.

Chỗ lệch: `tong` sinh ra bằng số `0`, không phải chuỗi `"0"`. Cộng một con số
với một cái tên thì Python từ chối chứ không tự đổi bên nào sang bên nào — đúng
`TypeError` bạn đã gặp ở Realm 0 khi trộn hai kiểu không hợp nhau.
::
:::
::::

::::code{#ban-bao-cao-tung-dong}
Byte cần một bản báo cáo: mỗi khoản một dòng, có tên có tiền.

Đoạn dưới dựng từng dòng chữ vào một danh sách trước, rồi mới in cả xấp ra. Chỗ
trống nằm giữa câu, đúng chỗ phải cho ra **số tiền** của lượt này.

```python title=starter
chi = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}

cac_dong = []
for khoa in chi:
    cac_dong.append(f"{khoa} hết {___} đồng")

for dong in cac_dong:
    print(dong)
```

```python title=solution
chi = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}

cac_dong = []
for khoa in chi:
    cac_dong.append(f"{khoa} hết {chi[khoa]} đồng")

for dong in cac_dong:
    print(dong)
```

```python title=test
# Ba dòng phải mang ba con số KHÁC nhau, mỗi con số lấy đúng từ cái ngăn dán
# nhãn của lượt ấy. Chép cứng một con số vào chỗ trống thì dòng thứ hai lộ ra
# ngay, vì nó đòi 25000 chứ không đòi 500000.
assert len(cac_dong) == 3, "sổ này có ba khoá, nên vòng lặp phải dựng đúng ba dòng báo cáo"
assert cac_dong[0] == "sửa xe hết 500000 đồng", "dòng đầu nói về khoá 'sửa xe', và cái ngăn dán nhãn ấy đang giữ 500000"
assert cac_dong[1] == "cà phê hết 25000 đồng", "dòng thứ hai nói về khoá 'cà phê', và cái ngăn dán nhãn ấy đang giữ 25000"
assert cac_dong[2] == "biếu bà hết 300000 đồng", "dòng thứ ba nói về khoá 'biếu bà', và cái ngăn dán nhãn ấy đang giữ 300000"
```

:::hints
- kind: attention
  body: Chỗ trống nằm giữa hai chữ trong câu báo cáo, và nó phải cho ra số tiền của lượt này. Nhưng thứ vòng lặp vừa đặt vào tay bạn chỉ có `khoa` — cái nhãn dán ngoài ngăn, chưa phải thứ nằm trong ngăn.
- kind: strategy
  body: Mở một cái ngăn ra thì cần hai thứ viết cạnh nhau: tên cuốn sổ, rồi cái nhãn đặt trong cặp ngoặc vuông. Cái nhãn ở đây không phải một chữ cố định — nó đổi theo từng lượt, nên chỗ ấy phải là chính cái tên lặp. Ba dòng in ra phải mang ba con số khác nhau, nên một con số gõ cứng sẽ không qua được.
- kind: one-line
  body: 'Viết `chi[khoa]` vào chỗ trống, và đừng đặt dấu nháy nào quanh `khoa`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: sửa xe hết 500000 đồng
- tier: output
  expect: biếu bà hết 300000 đồng
- tier: static
  onFail: chỗ trống phải mở cuốn sổ ra bằng chính cái nhãn của lượt này, không phải gõ cứng một con số
  requireAst:
  # `chi` phải được ĐỌC hai lần: một lần ở dòng `for` (khung đã có sẵn), một
  # lần nữa ở chỗ trống. Đáp án gõ cứng chỉ đọc `chi` đúng một lần.
  # `khoa` cũng vậy: khung mới đọc nó một lần trong f-string.
  - kind: uses-name, target: chi, min: 2
  - kind: uses-name, target: khoa, min: 2
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cái nhãn trong tay, cái ngăn mở ra. Hai bước cho mỗi dòng báo cáo.
::::

::::explain{#hai-buoc-cho-mot-dong}
Gói lại một câu: **duyệt một cuốn sổ tra cứu thì thứ máy đưa ra là khoá.**

Từ câu đó rơi ra hai chuyện dùng được ngay:

- `for khoa in chi:` và `for khoa in chi.keys():` là **một** — lối thứ nhất chỉ
  là cách viết gọn. Gặp lối viết dài trong code người khác thì đừng tưởng nó
  làm chuyện gì khác.
- Số lượt của vòng bằng **số khoá** trong sổ, không phải số thứ có trong sổ. Sổ
  ba khoá thì ba lượt, dù mỗi khoản gồm cả một cái tên lẫn một con số.

> Chỗ dễ vấp: đặt tên cái tên lặp là `tien` rồi viết `for tien in chi:`. Máy
> chạy êm, không báo gì cả — nhưng thứ nằm trong `tien` vẫn là cái nhãn, và
> mọi dòng phía sau đọc lên sẽ nói dối bạn. Đây là loại sai máy không nhắc,
> nên cái tên lặp ở đây phải nói thật: `khoa`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cuối tháng Byte chỉ cần **tổng tiền**, chẳng cần tên khoản nào cả. Nhưng cách
duy nhất bạn đang có là duyệt khoá rồi lại tra ngược `chi[khoa]` — hai lần việc
cho một con số, mà cái nhãn lấy ra rồi thì vứt đi ngay.

Xin thẳng đống giá trị được không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
