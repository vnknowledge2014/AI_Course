---
id: nen-tang.gia-tri-bien-kieu.lam-tron-cho-cong-bang
title: Làm tròn cho công bằng
summary: round() không cắt mà đo — nó đưa con số về cái cọc số nguyên gần nó nhất, và thứ đi ra là int.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.round]
requires: [core.int-truncate, core.float-contagion, core.division]
concepts: [core.doi-kieu, core.so-thap-phan]
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
Có người thứ hai cũng bỏ phần lẻ đi. Nhưng người này nhìn phần lẻ trước khi bỏ.
::::

::::explain{#hai-cai-coc}
Bài trước bạn có một cái kéo. Nó nhanh, nó gọn, và nó luôn cắt về cùng một phía
— nên `45000.7` rơi xuống `45000`, thiệt gần trọn một đồng mà không kêu.

Việc bạn cần bây giờ là việc khác: nhìn phần lẻ rồi mới quyết.

Hình dung `45000.7` là một cái cột đứng giữa hai cái cọc đóng sẵn dưới đất:
cọc `45000` bên trái và cọc `45001` bên phải. Cột đứng gần cọc bên phải hơn
nhiều — chỉ cách `0.3`, trong khi cách cọc trái tới `0.7`.

Cái kéo của bài trước không nhìn hai cái cọc. Nó luôn ngả về cọc bên trái, bất
kể cột đứng đâu.

Python có sẵn một cái tên làm đúng việc còn lại: **`round`**. Nó đo, rồi ngả về
cọc gần hơn.
::::

::::example{#hai-cach-bo-phan-le}
Byte đặt hai công cụ cạnh nhau, trên hai con số.

```python title=readonly
tien = 45000.7
print(int(tien))
print(round(tien))

tien_it = 45000.2
print(int(tien_it))
print(round(tien_it))
```

Máy in ra:

```text
45000
45001
45000
45000
```

Hai dòng đầu là `45000.7`. Cái kéo cắt xuống `45000`; `round` đo thấy cọc phải
gần hơn nên đưa lên `45001`.

Hai dòng sau là `45000.2`, và lần này **hai công cụ cho ra cùng một con số**.
Không phải vì chúng giống nhau, mà vì cột lần này đứng nghiêng về bên trái —
cọc gần nhất tình cờ cũng chính là cọc mà cái kéo luôn ngả về.

Đó là lý do thử một con số thôi thì không đủ để biết mình đang dùng công cụ
nào.
::::

::::predict{#chia-cho-sau-nguoi commitOnce}
Bữa tối `100000` đồng, sáu người chia đều. Byte đưa phần mỗi người về số nguyên
bằng cả hai công cụ, để so.

Phép chia cho ra `16666.666666666668`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra hai con số nào?

```python
tien_bua_toi = 100000
moi_nguoi = tien_bua_toi / 6
print(int(moi_nguoi))
print(round(moi_nguoi))
```

:::opt{correct}
16666 rồi 16667
:::

:::opt
16666 rồi 16666
::why
Gần đúng ở chỗ bạn nhớ chắc bài trước, và nhớ đúng từng chữ: cái kéo giữ phần
đứng trước dấu chấm, nên `16666.666…` cắt xong còn `16666`. Nửa đầu bạn không
sai chỗ nào.

Chỗ lệch nằm ở dòng thứ hai. `round` không cắt, nó đo. Phần lẻ ở đây là `.666`
— đã đi quá nửa đường sang cọc bên phải — nên nó ngả về `16667`.
::
:::

:::opt
16667 rồi 16667
::why
Gần đúng ở chỗ bạn tính đúng phép làm tròn: `16666.666…` gần `16667` hơn thật,
và dòng thứ hai bạn đoán trúng.

Chỗ lệch nằm ở dòng thứ nhất. `int(...)` không làm tròn lần nào cả — nó không
đo, không so, không nhìn hai cái cọc. Hai công cụ này trùng kết quả mỗi khi phần lẻ
chưa đủ để nhích lên cọc phải; ở đây phần lẻ là `.666` nên chúng tách hẳn ra,
mỗi người một cọc.
::
:::

:::opt
16666.0 rồi 16667.0
::why
Gần đúng ở chỗ bạn theo đúng luật lây của bài 1: `moi_nguoi` mang nhãn `float`,
nên bạn giữ cái đuôi cho tới cuối. Với `+`, `-`, `*` thì suy luận ấy đúng.

Chỗ lệch: `int(...)` và `round(...)` không phải phép tính, chúng là hai lệnh
**đổi kiểu**. Cả hai đều trả về một giá trị mang nhãn `int`, và `int` không có
chỗ nào chứa phần lẻ. Cái đuôi đi hẳn, ở cả hai dòng.
::
:::
::::

::::explain{#luat-va-mot-cho-la}
> `round(x)` đưa `x` về **số nguyên gần nó nhất**, và thứ đi ra mang nhãn `int`.

Bốn lần đo, đặt cạnh cái kéo của bài trước:

| gõ vào | `int(...)` | `round(...)` |
|---|---|---|
| `45000.2` | `45000` | `45000` |
| `45000.7` | `45000` | `45001` |
| `45999.99` | `45999` | `46000` |
| `16666.666666666668` | `16666` | `16667` |

Cột giữa lúc nào cũng lùi. Cột phải thì lúc lùi lúc tiến, tuỳ cột đứng gần cọc
nào.

Ẩn dụ đã xong, giờ tới thuật ngữ: việc này gọi là **làm tròn** — tiếng Anh là
*round*, đúng bằng cái tên bạn vừa gõ.

**Một chỗ lạ, ghi ra để bạn khỏi hoang mang.** Có một trường hợp không có "cọc
gần nhất": khi cột đứng **đúng chính giữa**, như `2.5`. Hai cọc cách đều nhau,
nên phải có một quy ước để phá hoà. Python chọn cọc **chẵn**: `round(2.5)` ra
`2`, còn `round(3.5)` ra `4`. Quy ước ấy để một chuỗi dài phép làm tròn không
lệch dần về một phía. Bạn chưa cần dùng tới nó; chỉ cần biết là hôm nào gõ thử
`round(2.5)` mà thấy `2` thì không phải bạn gõ sai.
::::

::::code{#chia-deu-hai-bua}
Cùng một bữa `100000` đồng, hai lần chia khác nhau.

- Lần thứ nhất: **ba** người.
- Lần thứ hai: **sáu** người.

Điền hai chỗ trống để máy in ra phần mỗi người của từng lần, đã làm tròn về số
nguyên gần nhất.

Bài chấm cả hai lần chia, vì đúng một trong hai lần mới phân biệt được hai công
cụ: chia ba thì phần lẻ là `.333` — dưới nửa, nên cái kéo và cái thước cho cùng
một kết quả. Chia sáu thì phần lẻ là `.666`, và hai công cụ tách ra.

```python title=starter
tien_bua_toi = 100000

chia_ba_nguoi = ___
chia_sau_nguoi = ___

print(chia_ba_nguoi)
print(chia_sau_nguoi)
```

```python title=solution
tien_bua_toi = 100000

chia_ba_nguoi = round(tien_bua_toi / 3)
chia_sau_nguoi = round(tien_bua_toi / 6)

print(chia_ba_nguoi)
print(chia_sau_nguoi)
```

```python title=test
# Chấm bằng HAI lần chia, và kiểm cả NHÃN chứ không chỉ con số.
#
# Vì sao hai lần chứ không một: chia ba có phần lẻ `.333`, nằm dưới nửa, nên
# `int(...)` và `round(...)` cho ra cùng `33333` — một mình nó không phân biệt
# được bài trước với bài này. Chia sáu có phần lẻ `.666` nên nó tách hai công
# cụ ra: cắt cho `16666`, làm tròn cho `16667`.
#
# Mỗi câu trả lời hụt lộ ra ở ít nhất một dòng:
#   dùng `int(...)` cho cả hai        → dòng chia sáu ra 16666, hụt một đồng;
#   quên bọc, chỉ để phép chia        → còn nguyên đuôi float, sai nhãn;
#   chép nguyên dòng chia ba xuống    → dòng chia sáu ra 33333;
#   điền một con số chết              → dòng kia sai.
#
# Cái nhãn phải hỏi thẳng, vì `33333.0 == 33333` cho `True` trong Python —
# so sánh bằng một mình không phân biệt nổi `float` với `int`.
#
# Vì sao hỏi bằng `.__class__.__name__` chứ không bằng `type(x) is int`: bộ
# chấm chạy nhiều bài trong cùng một phiên Python, và bài 13 của chính track
# này dạy đè lên tên có sẵn (`int = 0`). Sau bài ấy, `int` không còn là cái
# kiểu nữa. Cách hỏi dưới đây không tra một cái tên có sẵn nào.
assert chia_ba_nguoi == 33333, "chia ba người phải ra 33333"
assert chia_sau_nguoi == 16667, "chia sáu người phải ra 16667 vì 16666.666… gần cọc 16667 hơn"
assert chia_ba_nguoi.__class__.__name__ == "int", "kết quả phải mang nhãn int, không còn đuôi"
assert chia_sau_nguoi.__class__.__name__ == "int", "kết quả phải mang nhãn int, không còn đuôi"
```

:::hints
- kind: attention
  body: Hai chỗ trống chia cùng một số tiền cho hai số người khác nhau, nên hai dòng không viết giống hệt nhau được. Kết quả phải là số nguyên, nên mỗi chỗ trống chứa hai việc chứ không phải một.
- kind: strategy
  body: Việc thứ nhất là chia tổng tiền cho số người. Việc thứ hai là đưa kết quả về cọc gần nhất, gọi bằng cái tên năm chữ cái vừa học ở bài này, và cả phép chia nằm gọn trong ngoặc của nó. Nếu dùng cái kéo của bài trước thì dòng chia sáu sẽ hụt đúng một đồng.
- kind: one-line
  body: "Viết `round(tien_bua_toi / 3)` vào chỗ trống thứ nhất và `round(tien_bua_toi / 6)` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^33333\n16667\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cọc nào gần thì về cọc ấy. Lần này mình có nhìn trước khi bỏ phần lẻ đi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cầm hai con số vừa in ra, rồi cộng ngược lại xem quán thu về bao nhiêu.

Bàn ba người: `33333 * 3` bằng `99999`. Hoá đơn `100000`. Thiếu một đồng —
quán chịu, và quán chịu thì không ai kiện.

Bàn sáu người: `16667 * 6` bằng `100002`. Hoá đơn vẫn `100000`. **Thừa hai
đồng** — hai đồng ấy là tiền của khách, và quán vừa thu quá tay mà không ai bấm
nút nào.

Đó là chỗ "gần nhất" tách khỏi "công bằng". Gần nhất thì có lúc nhích lên; mà
chia tiền thì không được phép nhích lên, vì phần thừa ấy chẳng của ai cả. Chia
tiền chỉ có một chiều an toàn: luôn luôn **xuống**.

Chiều xuống thì bài trước đã có — `int(100000 / 6)` cho `16666`, đúng ý bạn.
Nhưng nhìn lại dòng ấy mà xem: hai công cụ chồng lên nhau cho một việc bạn làm
hằng ngày, và ở giữa vẫn phải đẻ ra một `16666.666666666668` chỉ để cắt bỏ nó
ngay dòng sau.

Vậy có dấu phép tính nào làm thẳng việc "chia rồi lấy phần nguyên" trong đúng
một bước — và cho ra `int` ngay từ đầu, không đi vòng qua số thực — không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
