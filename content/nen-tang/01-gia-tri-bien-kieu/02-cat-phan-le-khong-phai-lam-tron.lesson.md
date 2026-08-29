---
id: nen-tang.gia-tri-bien-kieu.cat-phan-le-khong-phai-lam-tron
title: Cắt phăng chứ không làm tròn
summary: Đặt int() lên một số thực thì máy giữ đúng phần trước dấu chấm và bỏ hẳn phần sau, dù phần sau gần bằng một đồng.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.int-truncate]
requires: [core.float-contagion, core.int-cast, core.division]
concepts: [core.doi-kieu, core.kieu-gia-tri]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Mình bỏ phần lẻ đi thật. Nhưng bỏ theo kiểu của mình, không theo kiểu bạn học ở trường.
::::

::::explain{#cong-cu-cu-viec-moi}
Bài trước để lại hai câu hỏi. Câu thứ nhất trả lời được ngay, bằng một cái tên
bạn đã gõ nhiều lần ở Realm 0: `int`.

Hồi đó bạn dùng nó cho việc này:

```python
tuoi_khach = int("25")
```

Khách gõ vào bàn phím thì thứ đi ra là một câu chữ, và `int(...)` đổi câu chữ
ấy thành một con số thật, tính toán được.

Cũng cái tên ấy, đặt lên một **số thực**, thì nó cũng làm đúng việc ấy — đổi
kiểu. Lần này là từ `float` sang `int`:

```python
int(180000.0)
```

Con số đi ra là `180000`, không đuôi. Giọt mực của bài trước bị vớt ra, và
`int(...)` là cái vợt.

Nhưng vớt mực thì dễ, vì phần lẻ bằng không nên không mất gì. Câu hỏi thật là
câu thứ hai của bài trước: khi phần lẻ **khác không**, cái vợt ấy làm gì với
nó?
::::

::::example{#dat-len-so-thuc}
Byte thử hai lần. Lần đầu phần lẻ bằng không, lần sau thì không.

```python title=readonly
tien_thit = 180000.0
print(int(tien_thit))

tien_ban_b = 100000
phan_moi_nguoi = tien_ban_b / 3
print(phan_moi_nguoi)
print(int(phan_moi_nguoi))
```

Máy in ra:

```text
180000
33333.333333333336
33333
```

Dòng đầu đúng như chờ đợi: đuôi rụng, con số nguyên vẹn.

Dòng thứ hai là bàn ăn ba người, mỗi người `33333.333333333336` đồng — một con
số dài ngoằng mà không dòng sổ nào chép nổi. Dòng thứ ba, sau khi qua vợt, còn
`33333`.

Cái phần `.333333333336` không đi đâu cả. Nó biến mất.
::::

::::predict{#bay-phan-muoi commitOnce}
Hoá đơn tiền điện cả dãy trọ tháng này là `450007` đồng, chia đều cho `10`
phòng. Byte tính phần mỗi phòng, rồi đưa nó về số nguyên.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra hai con số nào?

```python
tien_dien_ca_day = 450007
so_phong = 10
moi_phong = tien_dien_ca_day / so_phong
print(moi_phong)
print(int(moi_phong))
```

:::opt{correct}
45000.7 rồi 45000
:::

:::opt
45000.7 rồi 45001
::why
Gần đúng ở chỗ bạn dùng đúng luật đã học ở trường: phần lẻ là `0.7`, lớn hơn
nửa, nên con số nhích lên `45001`. Trong một bài toán trên giấy thì bạn đã trả
lời chính xác.

Chỗ lệch: `int(...)` không hề so phần lẻ với nửa. Nó không đọc phần lẻ lần nào
cả. Nó chỉ giữ lại đoạn đứng **trước** dấu chấm và bỏ phần đứng sau — nên
`45000.1` và `45000.9` đi qua nó đều cho ra cùng một con số.

Bạn vừa đoán trúng thứ mà một công cụ **khác** sẽ làm. Công cụ đó có thật, và
nó là bài sau.
::
:::

:::opt
45000.7 rồi 45000.0
::why
Gần đúng ở chỗ bạn nhớ chắc luật lây của bài trước: `moi_phong` mang nhãn
`float`, nên bạn giữ cái đuôi cho tới cuối. Với phép cộng, phép nhân thì bạn
suy luận như vậy là đúng.

Chỗ lệch: `int(...)` không phải một phép tính, nó là một lệnh **đổi kiểu**. Nó
không nâng ai lên cả, nó dán một cái nhãn mới. Thứ đi ra khỏi nó luôn mang nhãn
`int`, và `int` thì không có chỗ nào để chứa phần lẻ — kể cả phần lẻ bằng
không.
::
:::

:::opt
Máy dừng lại và báo ValueError vì 45000.7 không phải một số nguyên
::why
Gần đúng ở chỗ bạn nhớ ra `int(...)` có lúc nổ thật: ở Realm 0, khách gõ vào
mấy chữ cái thay vì con số thì `int(...)` dừng hẳn và nói `ValueError`.

Chỗ lệch nằm ở thứ nằm trong ngoặc. Lúc ấy nó nhận một **câu chữ** mà nó không
đọc ra số nào, nên nó không có câu trả lời nào để đưa. Ở đây nó nhận một **số
thực** — và với một số thực thì nó luôn có câu trả lời sẵn, không bao giờ phải
hỏi lại.
::
:::
::::

::::explain{#cat-chu-khong-tron}
Hình dung `45000.7` viết trên một mẩu giấy dài. `int(...)` không cân nhắc, không
đắn đo, không nhìn xem phần đuôi to hay nhỏ. Nó đặt kéo ngay sau dấu chấm, cắt
một nhát, giữ mẩu bên trái. Mẩu bên phải rơi xuống đất.

> `int(...)` đặt lên một số thực thì giữ đúng phần đứng **trước** dấu chấm và bỏ
> hẳn phần đứng sau. Nó không so phần lẻ với `0.5` lần nào, nên `int(45000.1)`
> và `int(45000.9)` cho ra cùng một con số.

Bốn lần cắt, cho thấy nó lạnh lùng tới đâu:

| gõ vào | ra |
|---|---|
| `int(45000.0)` | `45000` |
| `int(45000.1)` | `45000` |
| `int(45000.7)` | `45000` |
| `int(45999.99)` | `45999` |

Dòng cuối đáng nhìn lâu: `45999.99` chỉ còn thiếu một phần trăm nữa là chạm
`46000`, mà nó vẫn rơi xuống `45999`.

Ẩn dụ đã xong, giờ tới thuật ngữ: kiểu bỏ phần lẻ này gọi là **cắt cụt** —
tiếng Anh là *truncate*. Từ ấy phân biệt nó với *round* (làm tròn), và bài sau
là bài của *round*.

Thêm một điều đáng ghi: thứ đi ra khỏi `int(...)` mang nhãn `int` **thật**,
không phải một `float` được in gọn lại. Nên đây cũng là cách duy nhất bạn đang
có để dập tắt luật lây của bài trước — vớt giọt mực ra khỏi ca nước.
::::

::::code{#dua-hai-hoa-don-ve-so-nguyen}
Tối nay quán ghi hai bàn.

- **Bàn A**: `90000` đồng, `2` người chia — chia hết, không dư đồng nào.
- **Bàn B**: `170000` đồng, `3` người chia — chia không hết, và phần lẻ lần
  này **quá nửa**.

Điền hai chỗ trống để máy in ra phần mỗi người của từng bàn, **dưới dạng số
nguyên**, không đuôi.

Bài chấm cả hai bàn. Thử đúng một bàn thì chưa phân biệt được đúng với gặp may:
bàn A chia hết nên viết kiểu nào cũng gần ra, còn bàn B mới là chỗ cái kéo thật
sự cắt vào tiền.

```python title=starter
tien_ban_a = 90000
nguoi_ban_a = 2

tien_ban_b = 170000
nguoi_ban_b = 3

phan_ban_a = ___
phan_ban_b = ___

print(phan_ban_a)
print(phan_ban_b)
```

```python title=solution
tien_ban_a = 90000
nguoi_ban_a = 2

tien_ban_b = 170000
nguoi_ban_b = 3

phan_ban_a = int(tien_ban_a / nguoi_ban_a)
phan_ban_b = int(tien_ban_b / nguoi_ban_b)

print(phan_ban_a)
print(phan_ban_b)
```

```python title=test
# Chấm bằng HAI bàn, và kiểm cả NHÃN chứ không chỉ con số.
#
# Hai bàn được chọn để mỗi câu trả lời hụt đều lộ ra ở ít nhất một bàn:
#   quên `int(...)`                       → bàn A ra 45000.0, bàn B ra
#                                           56666.666666666664 — sai cả hai;
#   chỉ bọc `int(...)` cho một bàn         → bàn kia còn đuôi;
#   chép nguyên dòng của bàn A xuống dưới  → bàn B ra 45000, sai con số;
#   điền một con số chết vào chỗ trống     → bàn kia sai;
#   dùng `round(...)` thay `int(...)`      → bàn B ra 56667, TRƯỢT.
# Câu hụt cuối là câu bài này sinh ra để bắt, nên nó quyết định luôn con số của
# bàn B. `170000 / 3` cho `56666.666…` — phần lẻ QUÁ NỬA, nên cái thước nhích
# lên 56667 còn cái kéo đứng lại 56666. Người học ôm hiểu lầm "int() làm tròn"
# sẽ lộ ra ngay tại đây.
#
# Bàn A chia hết nên nó KHÔNG đủ để chấm một mình: ở đó `int(...)` chỉ gỡ cái
# đuôi `.0`, không cắt mất gì. Và nếu bàn B lấy phần lẻ DƯỚI nửa — như
# `100000 / 3` ở khối ví dụ phía trên — thì kéo với thước lại ra cùng một số,
# và cả bài tập không phân biệt nổi hai thứ nó vừa dạy là khác nhau.
#
# Cái nhãn phải hỏi thẳng, vì trong Python `45000.0 == 45000` cho `True` —
# so sánh bằng một mình không phân biệt nổi `float` với `int`.
#
# Vì sao hỏi bằng `.__class__.__name__` chứ không bằng `type(x) is int`: bộ
# chấm chạy nhiều bài trong cùng một phiên Python, và bài 13 của chính track
# này dạy đè lên tên có sẵn (`int = 0`). Sau bài ấy, `int` không còn là cái
# kiểu nữa. Cách hỏi dưới đây không tra một cái tên có sẵn nào.
assert phan_ban_a == 45000, "bàn A chia đôi phải ra 45000"
assert phan_ban_b == 56666, "bàn B phải ra 56666 vì int() CẮT; round() sẽ nhích lên 56667 và đó là câu trả lời sai bài này đi tìm"
assert phan_ban_a.__class__.__name__ == "int", "phần bàn A phải mang nhãn int, không còn đuôi .0"
assert phan_ban_b.__class__.__name__ == "int", "phần bàn B phải mang nhãn int, không còn đuôi .666"
```

:::hints
- kind: attention
  body: Hai chỗ trống hỏi cùng một câu cho hai bàn khác nhau, và mỗi bàn có tên riêng cho tiền và cho số người. Kết quả in ra phải là số nguyên, nên trong mỗi chỗ trống có hai việc chứ không phải một.
- kind: strategy
  body: Việc thứ nhất là chia tổng tiền của bàn cho số người của chính bàn ấy. Việc thứ hai là đưa kết quả phép chia về số nguyên, gọi bằng đúng cái tên ba chữ cái bạn đã dùng ở Realm 0 để đổi chữ thành số — lần này thứ nằm trong ngoặc của nó chính là cả phép chia.
- kind: one-line
  body: "Viết `int(tien_ban_a / nguoi_ban_a)` vào chỗ trống thứ nhất và `int(tien_ban_b / nguoi_ban_b)` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^45000\n56666\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai bàn, hai số nguyên. Bàn B vừa rơi mất hai phần ba đồng — cái thước sẽ nhích lên, cái kéo thì không. Và mình không kêu tiếng nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cái kéo luôn cắt về **một phía**. Nó không bao giờ nhích lên. `int(56666.666)`
bỏ đi hai phần ba đồng; `int(45999.99)` bỏ đi gần trọn một đồng. Không hoá đơn
nào lỗ quá một đồng, và không hoá đơn nào báo lỗi.

Nhưng nó lệch về cùng một bên, mọi lần, không sót lần nào. Một nghìn hoá đơn là
một nghìn lần thiệt cùng một hướng — mấy trăm đồng bốc hơi mà sổ không ghi lấy
một dòng.

Và có chỗ phần lẻ ấy không nhỏ. Bác bán thịt đọc giá cho bạn bằng **nghìn**
cho nhanh — `25.5` nghĩa là hai mươi lăm nghìn rưỡi. Chép thẳng con số ấy ra
rồi đặt cái kéo lên, `int(25.5)` cho `25`. Năm trăm đồng, cắt xong không ai
biết. (Quyển sổ thì vẫn ghi bằng đồng như bài trước — `25.5` là lời người ta
đọc, không phải cách sổ ghi.)

Cắt thì nhanh, nhưng cắt thì thiên vị. Muốn con số nhích về phía **gần nhất** —
`45000.7` thành `45001`, còn `45000.2` vẫn là `45000` — thì gọi ai?

Cái vợt bạn đang cầm không làm được việc đó. Bài sau đưa bạn cái khác.
::::

::::checkpoint{mastery=0.8}
::::
