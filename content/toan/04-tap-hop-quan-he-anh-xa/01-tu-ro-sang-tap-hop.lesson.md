---
id: toan.tap-hop-quan-he-anh-xa.tu-ro-sang-tap-hop
title: Từ cái rổ sang tập hợp
summary: "Cái rổ Realm 1 đã dạy (không thứ tự, không trùng, tra bằng `in`) giờ có một cái tên trong toán: tập hợp — vẫn đúng máy đã học, chỉ đổi ngôn ngữ gọi tên."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.set]
requires: [core.set, core.set-unordered, core.set-membership, core.print-variable, core.variable, core.boolean, core.function-def, core.function-call, core.function-parameter, core.function-return]
concepts: [math.tap-hop, math.ro-va-tap-hop]
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
Vườn giờ có bảy luống, không phải một. Mỗi luống trồng mấy loại rau khác
nhau — mà cái rổ Realm 1 đã LÀM ĐƯỢC việc "chứa mấy loại, không phân biệt
thứ tự" từ lâu rồi, phải không?
::::

::::explain{#ro-da-biet}
Đúng vậy. Track này không dạy lại cái rổ — nó dạy một cái TÊN mới cho cái
bạn đã có.

Nhắc lại đúng những gì Realm 1 đã chứng minh trên máy (T1.4 "Cái rổ không
chứa hai lần" và "Rổ không xếp hàng"): rổ không phân biệt thứ tự bỏ vào,
không giữ phần tử trùng, và `in` tra được ngay một thứ có nằm trong rổ hay
không.

```python title=readonly
luong_1 = {"cà chua", "xà lách", "cà rốt"}

print(len(luong_1))
print("cà chua" in luong_1)
print("bí đỏ" in luong_1)
```

```text title=readonly
3
True
False
```

Ba loại rau, `len` đếm ra 3. `"cà chua" in luong_1` tra thấy có, `"bí đỏ"`
thì không. Không gì trong đoạn này mới — nó là chính xác thứ bạn đã gõ ở
Realm 1.
::::

::::explain{#ten-toan-cua-cai-ro}
Cái mới là TÊN. Trong toán, một cái rổ như `luong_1` được gọi là một **tập
hợp**: một nhóm mà thứ duy nhất quan trọng là CÓ những gì trong đó — không
phải kể theo thứ tự nào, không phải kể lặp mấy lần.

Toán không viết `{"cà chua", "xà lách", "cà rốt"}` bằng dấu ngoặc nhọn
Python. Nó có ký hiệu riêng, và ký hiệu đầu tiên — cách viết ngắn của
`in` — xuất hiện ngay ở bài sau.
::::

::::example{#hai-nguoi-ke-mot-luong}
Lan và Minh cùng ra xem luống 1, rồi kể lại cho Byte — không ai nhìn bài
của người kia:

```python title=readonly
lan_ke = {"cà chua", "xà lách", "cà rốt"}
minh_ke = {"cà rốt", "cà chua", "xà lách"}

print(lan_ke == minh_ke)
```

```text title=readonly
True
```

Lan kể theo một thứ tự, Minh kể theo thứ tự khác hẳn — và `==` xác nhận
đây là CÙNG một tập hợp. Đây chính là điều "tập hợp" muốn nói: hai lời kể
trông khác nhau vẫn có thể là MỘT câu trả lời, miễn là chúng nói tới đúng
những phần tử ấy.
::::

::::predict{#doan-tu-ke-lap-lai commitOnce}
Tú cũng ra xem luống 1, nhưng kể lộn xộn hơn — nhắc "cà chua" tới hai lần:

```python
lan_ke = {"cà chua", "xà lách", "cà rốt"}
tu_ke = {"cà chua", "cà chua", "xà lách", "cà rốt"}

print(tu_ke == lan_ke)
print(len(tu_ke))
```

Tú nói ra BỐN tiếng rau (đếm cả lần lặp), Lan nói ra BA. Hai dòng cuối in
ra gì?

:::opt{correct}
`True`, rồi `3`
:::

:::opt
`False`, rồi `4` — vì Tú kể tới BỐN tiếng rau còn Lan chỉ BA, nên hai lời
kể phải là hai tập hợp KHÁC nhau, và `len` phải đếm đúng số tiếng Tú đã
nói ra
::why
Gần đúng ở việc bạn đếm ĐÚNG số TIẾNG Tú nói ra — bốn tiếng, không sai
một tiếng nào.

Chỗ lệch: "tập hợp" không đếm số TIẾNG NÓI RA, nó chỉ đếm số PHẦN TỬ KHÁC
NHAU thật sự có mặt. `tu_ke = {"cà chua", "cà chua", "xà lách", "cà rốt"}`
— dấu ngoặc nhọn xây một cái RỔ, và rổ (T1.4) tự động bỏ phần trùng NGAY
LÚC xây, trước khi có cơ hội đếm. Rổ ấy chỉ giữ lại BA loại: cà chua, xà
lách, cà rốt — giống hệt Lan. `tu_ke == lan_ke` là `True`, `len(tu_ke)` là
`3`, không phải `4`. Nhắc hai lần không làm một loại rau "nhiều hơn" —
luống 1 chỉ CÓ hoặc KHÔNG CÓ cà chua, không có khái niệm "có cà chua gấp
đôi" ở tầng tập hợp.
::
:::

:::opt
Máy báo lỗi biên dịch — `tu_ke` khai hai lần `"cà chua"` trong cùng một
tập hợp, Python cấm viết trùng một giá trị hai lần trong dấu `{}`
::why
Gần đúng ở việc bạn để ý `"cà chua"` xuất hiện hai lần trong dòng khai
`tu_ke` — một quan sát đúng về CÁCH VIẾT.

Chỗ lệch: Python hoàn toàn không cấm điều đó — `{"cà chua", "cà chua",
...}` là cú pháp hợp lệ, y hệt cách gọi `.add("cà chua")` hai lần đã học
ở T1.4 (lần gọi thứ hai không đổi gì, không lỗi gì). Máy chạy sạch, chỉ
đơn giản GIỮ LẠI mỗi giá trị đúng một lần.
::
:::
::::

::::code{#viet_cung_mot_tap_hop}
Viết `cung_mot_tap_hop` — nhận vào HAI lời kể (có thể khác thứ tự, có thể
lặp), trả lời chúng có phải CÙNG một tập hợp hay không.

Lời kể tới dưới dạng `list` (Realm 1 đã quen: kể theo thứ tự nói ra, có
thể lặp) — việc của hàm là CHUYỂN nó thành tập hợp rồi mới so sánh, đúng
đúng ý bài này: tập hợp không quan tâm thứ tự hay số lần lặp của LỜI KỂ,
chỉ quan tâm nó chứa những gì.

```python title=starter
def cung_mot_tap_hop(ke_lai_1, ke_lai_2):
    return ___


print(cung_mot_tap_hop(["cà chua", "xà lách", "cà rốt"], ["cà rốt", "cà chua", "xà lách"]))
```

```python title=solution
def cung_mot_tap_hop(ke_lai_1, ke_lai_2):
    return set(ke_lai_1) == set(ke_lai_2)


print(cung_mot_tap_hop(["cà chua", "xà lách", "cà rốt"], ["cà rốt", "cà chua", "xà lách"]))
```

```python title=test
assert cung_mot_tap_hop(["cà chua", "cà chua", "xà lách"], ["xà lách", "cà chua"]) is True, "lời kể lặp 'cà chua' vẫn là cùng tập hợp với lời kể không lặp — rổ tự bỏ trùng"
assert cung_mot_tap_hop(["cà chua", "xà lách"], ["cà chua", "cà rốt"]) is False, "xà lách và cà rốt khác nhau — hai lời kể phải khác tập hợp"
assert cung_mot_tap_hop([], []) is True, "hai lời kể rỗng đều tả cùng một tập hợp rỗng"
assert cung_mot_tap_hop(["bí đỏ"], []) is False, "một lời kể có rau, một lời kể rỗng — không thể cùng tập hợp"
assert cung_mot_tap_hop(["a", "a", "a"], ["a"]) is True, "lặp lại bao nhiêu lần cũng chỉ còn một phần tử duy nhất"
```

:::hints
- kind: attention
  body: Chuyển MỖI lời kể thành một tập hợp (dùng set(...)) rồi mới so sánh bằng ==. So trực tiếp hai list thì thứ tự và số lần lặp sẽ làm sai kết quả.
- kind: strategy
  body: "set(ke_lai_1) == set(ke_lai_2)"
- kind: one-line
  body: "___ = set(ke_lai_1) == set(ke_lai_2)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: chỗ trống phải CHUYỂN cả hai lời kể thành tập hợp (set(...)) rồi mới so sánh — so trực tiếp hai list (ke_lai_1 == ke_lai_2) thì thứ tự và số lần lặp làm sai kết quả, đúng cái bài này đang kiểm
  requireAst:
  # set() phải gọi trên CẢ HAI lời kể — khung khởi đầu gọi 0 lần.
  - kind: uses-call, target: set, min: 2
  - kind: uses-operator, target: '==', min: 1
  - kind: uses-name, target: ke_lai_1, min: 1
  - kind: uses-name, target: ke_lai_2, min: 1
  forbidAst:
  # Chặn `list(...)`/`sorted(...)` thay cho `set(...)`: sorted hai list
  # trùng phần tử VẪN phân biệt được số lần lặp, nên nó sẽ đọc sai
  # ca "cà chua lặp lại" trong bộ test.
  - kind: uses-call, target: sorted
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai lời kể trông khác hẳn, mà là cùng một tập hợp. Bài sau: viết đúng cái
câu "có mặt" ấy bằng ký hiệu, không phải bằng chữ `in`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte viết `cà chua ∈ luống_1` — bằng ký hiệu, không phải bằng chữ Python.
Máy có đọc được câu ấy thẳng như vậy không, hay phải dịch nó sang một dòng
Python trước?
::::

::::checkpoint{mastery=0.8}
::::
