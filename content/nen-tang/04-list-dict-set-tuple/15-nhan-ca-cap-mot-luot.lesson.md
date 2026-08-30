---
id: nen-tang.list-dict-set-tuple.nhan-ca-cap-mot-luot
title: Nhận cả cặp một lượt
summary: `chi.items()` đưa ra từng cặp khoá–giá trị dính liền nhau, mỗi lượt một món — hết cảnh cầm nhãn rồi mở ngăn.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.dict-items]
requires: [core.dict-values, core.dict-iter-keys, core.dict, ctrl.for-each, core.list, core.list-append, core.list-index, core.len, core.str-cast, core.fstring]
concepts: [ctrl.lap, core.gia-tri, core.ten]
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
Tên với tiền dính liền nhau, sang tay mình một lượt. Khỏi đi hai chuyến.
::::

::::explain{#loi-thu-ba-cua-cuon-so}
Bài trước bạn xin được riêng phần giá trị, và tổng tháng ra trong một dòng.
Nhưng bản báo cáo thì cần cả tên lẫn tiền, nên bạn buộc phải quay về lối cũ:

```python title=readonly
for khoa in chi:
    print(f"{khoa} hết {chi[khoa]} đồng")
```

Vẫn đúng hai lần việc. Máy vừa đưa cái nhãn ra thì bạn lại đem chính cái nhãn
ấy quay vào mở ngăn — mà lúc đưa nhãn, máy đang đứng ngay cạnh cái ngăn đó.

Cuốn sổ tra cứu có lối thứ ba, và đây là lối đưa **cả hai**: `chi.items()`.

Mỗi lượt nó không đưa riêng nhãn, cũng không đưa riêng tiền. Nó đưa nguyên
**một cặp** — cái nhãn và số tiền dính liền nhau thành một món, sang tay bạn
cùng lúc.

Ba lối, ba câu hỏi khác nhau hỏi trên cùng một cuốn sổ:

- `chi.keys()` — cho tôi phần nhãn (và `for khoa in chi:` là lối viết gọn).
- `chi.values()` — cho tôi phần nằm trong ngăn.
- `chi.items()` — cho tôi cả cặp, mỗi lượt một món.
::::

::::example{#in-thu-tung-cap}
Vẫn cuốn sổ ba khoản quen thuộc:

```python title=readonly
chi = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}

for cap in chi.items():
    print(cap)
```

Máy in ra:

```text
('sửa xe', 500000)
('cà phê', 25000)
('biếu bà', 300000)
```

Điểm đáng nhìn ở đây là **hình dạng** của thứ mỗi vòng trao cho bạn: một món
**có hai thứ bên trong**, nên `print(cap)` in được cả tên lẫn tiền chỉ với một
cái tên duy nhất. (Còn mỗi lần chạy bao nhiêu vòng thì khối dưới hỏi bạn.)

Nhìn kỹ hình dạng thứ vừa hiện lên màn hình:

- mở bằng dấu ngoặc **tròn** `(` và đóng bằng `)`;
- hai thứ ngăn nhau bằng một dấu phẩy;
- cái tên có dấu nháy bao quanh, con số thì không — đúng như lúc bạn viết chúng
  vào sổ.

Giữ hình dạng ấy trong đầu. Nó chưa có tên trong bài này, và nó sẽ được gọi tên
ở bài sau.
::::

::::predict{#bao-nhieu-luot commitOnce}
Cuốn sổ dưới đây chỉ có hai khoản. Đoạn code đếm số lượt vòng lặp chạy được, và
in ra thứ nó nhận được ở mỗi lượt.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python title=readonly
chi = {"cà phê": 25000, "xăng xe": 120000}

so_luot = 0
for cap in chi.items():
    so_luot = so_luot + 1
    print(cap)

print(f"{so_luot} lượt")
```

:::opt{correct}
Hai dòng cặp, rồi dòng `2 lượt`
:::

:::opt
Bốn dòng — "cà phê", 25000, "xăng xe", 120000 — rồi dòng `4 lượt`
::why
Gần đúng ở chỗ bạn đếm rất chuẩn số **thứ** mà cuốn sổ này chứa: hai cái nhãn
cộng hai con số là bốn thứ, và `.items()` đúng là đưa ra cả bốn.

Chỗ lệch nằm ở chuyện nó **gói** chúng lại trước khi đưa. Mỗi lượt máy trao một
món, và trong món ấy có sẵn hai thứ đi cùng nhau — nên số lượt bằng số khoá,
không bằng số thứ. Đó cũng chính là điểm làm `.items()` khác hai lối kia:
`.keys()` và `.values()` trao từng thứ rời, `.items()` trao từng cặp.
::
:::

:::opt
Hai dòng cặp, rồi dòng `4 lượt`
::why
Gần đúng ở nửa khó hơn: bạn đọc đúng phần in ra — hai lượt, mỗi lượt một cặp
trên một dòng. Phần suy luận về `.items()` của bạn chính xác.

Chỗ lệch nằm ở cái tên đếm. `so_luot` chỉ nhích lên một ở mỗi lượt, đúng theo
dòng `so_luot = so_luot + 1`, nên nó đếm **lượt** chứ không đếm số thứ nằm
trong mỗi món. Hai lượt thì nó dừng ở 2.
::
:::

:::opt
Máy báo lỗi, vì một cái tên không giữ được hai giá trị cùng lúc
::why
Gần đúng ở chỗ bạn nhận ra điều lạ nhất trong đoạn này: `cap` đang mang nhiều
hơn một giá trị, mà từ trước tới giờ mỗi cái tên chỉ mang đúng một thứ.

Chỗ lệch: một cái tên vẫn giữ đúng **một** thứ — chỉ là thứ ấy có thể có nhiều
phần bên trong. Bạn đã gặp chuyện đó từ Realm 0: `mon = ["tái", "chín"]` là một
cái tên giữ một danh sách gồm hai phần. Ở đây cũng vậy, và `print` in ra được
món ấy nguyên đai, đúng hình dạng bạn vừa thấy.
::
:::
::::

::::code{#nhat-tung-cap-vao-so-tay}
Byte muốn giữ lại từng cặp nguyên vẹn — không tách tên khỏi tiền — để lát nữa
còn đem xếp lại.

Chỗ trống nằm ngay sau chữ `in`, tức là chỗ nói cho vòng lặp biết nó phải đi
qua cái gì.

```python title=starter
chi = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}

cac_cap = []
for cap in ___:
    cac_cap.append(cap)
    print(cap)

print(f"Sổ có {len(cac_cap)} khoản")
```

```python title=solution
chi = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}

cac_cap = []
for cap in chi.items():
    cac_cap.append(cap)
    print(cap)

print(f"Sổ có {len(cac_cap)} khoản")
```

```python title=test
# Ba phép kiểm, ba chuyện khác nhau. Phép đầu bắt số lượt: xin nhầm sang một
# lối trao từng thứ rời thì con số này vẫn có thể đúng, nên một mình nó chưa
# đủ. Hai phép sau mới nói thật — chúng đọc lại đúng cái đã in ra màn hình, và
# chỉ một món mang CẢ HAI thứ mới in ra đúng hình dạng ấy.
assert len(cac_cap) == 3, "sổ này có ba khoá, và mỗi khoá cho đúng một món, nên phải nhặt về ba món"
assert str(cac_cap[0]) == "('sửa xe', 500000)", "món đầu tiên phải mang cả cái tên 'sửa xe' lẫn con số 500000, in ra thành ('sửa xe', 500000)"
assert str(cac_cap[2]) == "('biếu bà', 300000)", "món cuối cùng phải mang cả cái tên 'biếu bà' lẫn con số 300000, in ra thành ('biếu bà', 300000)"
```

:::hints
- kind: attention
  body: Cuốn sổ mở ra ba lối, và bạn đã đi hai. Hai lối ấy đều trao từng thứ rời — hoặc toàn nhãn, hoặc toàn tiền — nên món nhặt về sẽ thiếu mất một nửa.
- kind: strategy
  body: Thứ Byte muốn giữ là món có cả tên lẫn tiền dính liền. Vậy chỗ trống phải là lối thứ ba, viết cùng một kiểu với hai lối kia: tên cuốn sổ, dấu chấm, rồi tên của lối đi. Đừng quên cặp ngoặc rỗng ở cuối — thiếu nó thì máy đưa lại chính cái hàm chứ không đưa kết quả.
- kind: one-line
  body: 'Viết `chi.items()` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^\('sửa xe', 500000\)\n\('cà phê', 25000\)\n\('biếu bà', 300000\)\nSổ có 3 khoản\s*$
- tier: output
  expect: Sổ có 3 khoản
- tier: static
  onFail: vòng lặp phải đi qua lối trao cả cặp, không phải một lối trao từng thứ rời
  requireAst:
  # Khung chưa gọi `.items()` lần nào, và cũng chưa ĐỌC `chi` lần nào — cái tên
  # ấy mới chỉ được đặt ở dòng đầu. Cả hai luật cùng trượt trên khung, cùng đạt
  # trên lời giải.
  - kind: uses-call, target: items, min: 1
  - kind: uses-name, target: chi, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một chuyến, hai thứ. Cuốn sổ trao thẳng cả cặp vào tay mình.
::::

::::explain{#ba-loi-mot-cuon-so}
Sổ tay của cả ba bài vừa rồi, trên đúng một cuốn sổ `chi`:

| viết | mỗi lượt trao cho bạn | số lượt |
|---|---|---|
| `for khoa in chi:` hay `for khoa in chi.keys():` | một cái nhãn | số khoá |
| `for tien in chi.values():` | một số tiền | số khoá |
| `for cap in chi.items():` | một cặp nhãn–tiền | số khoá |

Ba lối cùng chạy đúng số lượt và cùng đi theo thứ tự các khoá đã ghi vào sổ.
Chúng chỉ khác nhau ở thứ được đặt vào tay bạn mỗi lượt, nên chọn lối nào là
chọn theo việc bạn định làm:

- chỉ cần tên (in danh mục nhóm chi) → `.keys()`;
- chỉ cần số (cộng tổng, tìm khoản lớn nhất) → `.values()`;
- cần cả hai trên một dòng (bản báo cáo) → `.items()`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`.items()` trao cả cặp rồi, nhưng in thử một cặp ra thì màn hình cho một hình
dạng bạn đã **thoáng gặp** ở mạch Hàm — hồi một hàm trả về hai thứ một lúc —
mà chưa bài nào trong mạch này gọi tên nó ra:

```text
('sửa xe', 500000)
```

Dấu ngoặc **tròn**. Không phải `[ ]` như danh sách, cũng không phải `{ }` như
cuốn sổ tra cứu. Một hình dạng thứ ba, và máy dùng nó để buộc hai thứ lại với
nhau.

Thứ nằm trong ngoặc tròn ấy là gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
