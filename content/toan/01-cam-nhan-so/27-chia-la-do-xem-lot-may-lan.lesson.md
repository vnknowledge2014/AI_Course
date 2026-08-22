---
id: toan.cam-nhan-so.chia-la-do-xem-lot-may-lan
title: Chia là đo xem lọt mấy lần
summary: Khi cỡ mỗi phần đã cho sẵn, phép chia đi tìm số phần — và đó đúng là câu hỏi đo của bài 3: cái thước này đặt lặp lại được mấy lần.
locale: vi
track: toan
module: cam-nhan-so
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.division-quotative]
requires: [math.division-partitive, math.multiplication, math.thanh-so, core.arithmetic, core.output, core.variable, core.reassign, core.accumulator, ctrl.for-range, ctrl.if, ctrl.comparison]
concepts: [math.chia-do, math.dat-thuoc-lap-lai, math.thua-so-con-thieu]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Lần này mình biết cỡ mỗi đoạn. Cái mình chưa biết là cắt ra được mấy đoạn.
::::

::::explain{#khong-phat-vong-quanh-duoc}
Bài trước dừng lại ở chỗ khó chịu: cuộn dây **12 mét** của An, cắt thành từng
đoạn **3 mét**. Trong sổ vẫn viết `12 : 3`, kết quả vẫn là 4 — mà cách làm thì
không dùng lại được.

Với 12 hạt và 3 luống, Byte phát vòng quanh: luống A một hạt, luống B một hạt,
luống C một hạt, quay lại. Ba cái luống có sẵn ngoài vườn để mà phát vào.

Với cuộn dây thì không có cái gì để phát vào. Bốn đoạn dây **chưa tồn tại** —
chúng chỉ ra đời sau khi Byte cắt. Không thể chia đều một cuộn dây cho những
cái đoạn còn chưa có.

Nên Byte làm động tác khác hẳn: **đặt cái thước lên và đếm**.

```text
cuộn dây 12 mét
├───────────────────────────────────────────────┤
├───3───┤                                              đặt lần 1, còn 9
├───3───┼───3───┤                                      đặt lần 2, còn 6
├───3───┼───3───┼───3───┤                              đặt lần 3, còn 3
├───3───┼───3───┼───3───┼───3───┤                      đặt lần 4, còn 0
```

Hết dây. Đặt được **4 lần**, nên cắt ra **4 đoạn**.

Đó là **chia như đo**. Lần này:

- Byte **biết trước** cỡ mỗi phần: mỗi đoạn 3 mét. Con số 3 do An ra đề.
- Byte **đi tìm** số phần: cắt ra mấy đoạn. Con số 4 là thứ cuộc đo vừa sinh ra.

So với bài trước thì hai vai vừa đổi chỗ cho nhau, mà dòng chữ trong sổ thì y
hệt: `12 : 3 = 4`.
::::

::::explain{#day-la-bai-ba-quay-lai}
Câu "3 mét lọt vào 12 mét mấy lần" không phải câu hỏi mới. Bạn đã hỏi nó từ bài
3, chỉ là hồi đó nó chưa có dấu chia.

Bài 3 nói: **số đo là đếm xem cái thước đã chọn đặt lặp lại mấy lần**. Đo luống
đất bằng sải dây thì thước là "một sải"; đo bằng gang tay thì thước là "một
gang" — đổi thước thì con số đổi, còn luống đất thì không đổi tí nào.

Ở đây thước là **đoạn 3 mét**. Con số 4 là số đo của cuộn dây khi lấy 3 mét làm
thước. `12 : 3` chính là câu hỏi đo ấy, viết bằng ký hiệu.

Còn **vì sao** 4 là con số đúng? Vì cuộc đo dừng đúng lúc không còn dây. Bốn
đoạn, mỗi đoạn 3 mét, ghép lại thì đúng bằng cuộn ban đầu:

`4 × 3 = 12`

Đặt cạnh bài trước thì thấy hai phép chia là hai câu hỏi khác nhau về cùng một
phép nhân `? × ? = 12`:

| Câu hỏi | Đã biết | Đi tìm | Phép nhân |
|---|---|---|---|
| 12 hạt cho 3 luống, mỗi luống mấy hạt | số phần = 3 | cỡ mỗi phần | `3 × ? = 12` |
| 12 mét cắt đoạn 3 mét, được mấy đoạn | cỡ mỗi phần = 3 | số phần | `? × 3 = 12` |

Bài 20 đã nói xoay mảng chữ nhật 90° thì số cây không đổi, nên hai phép nhân
trên ra cùng một con số. Đó là lý do hai câu hỏi rất khác nhau lại viết chung
được một dòng `12 : 3` — và cũng là lý do người ta hay lẫn chúng.

Bức tranh thứ ba, trên **thanh số** của bài 5: đứng ở 0 và nhảy từng bước dài 3
về phía 12. Bước 1 tới 3, bước 2 tới 6, bước 3 tới 9, bước 4 tới 12 — dừng. Số
bước là 4. Chia đều là biết trước phải nhảy mấy bước rồi hỏi bước dài bao
nhiêu; chia đo là biết trước bước dài bao nhiêu rồi hỏi phải nhảy mấy bước.
::::

::::example{#may-khong-phan-biet-duoc}
Hỏi máy cả hai lần đo bằng dấu chia của Realm 0:

```python title=readonly
print(12 / 3)
print(21 / 7)
```

Máy in ra:

```text
4.0
3.0
```

Hai con số, không kèm chữ nào. Máy không biết `4.0` là "bốn hạt mỗi luống" hay
"bốn đoạn dây" — nó chỉ cho ra chỗ đứng trên thanh số, còn cái đơn vị đi kèm
thì bài 1 đã nói: nó nằm trong đầu người hỏi.

Đó chính là lý do bài này đáng đi chậm. Chỗ khác nhau giữa hai phép chia
**không** hiện ra ở kết quả, nên nếu bạn không bắt được nó trước khi bấm máy
thì sau đó không còn chỗ nào bắt được nữa.
::::

::::predict{#doan-cuoc-do commitOnce}
Byte tả cuộc đo cho máy nghe. `con_lai` là chỗ dây còn chưa cắt; `so_doan` đếm
số lần đặt thước.

Đọc bốn dòng giữa như một câu tiếng Việt: *"làm lại nhiều lần: nếu chỗ dây còn
lại vẫn đủ 3 mét thì cắt đi 3 mét và đếm thêm một đoạn."*

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
con_lai = 12
so_doan = 0
for lan in range(12):
    if con_lai >= 3:
        con_lai = con_lai - 3
        so_doan = so_doan + 1
print(so_doan)
print(con_lai)
```

:::opt{correct}
4 rồi 0
:::

:::opt
3 rồi 3
::why
Gần đúng ở chỗ bạn giữ chặt con số 3 — và ở bài trước thì bám vào con số cho
sẵn là cách đúng: hồi đó số 3 là số luống, và số luống đúng là thứ đã biết.

Chỗ lệch là ở đây con số 3 đổi vai. Nó không còn là số phần nữa; nó là **cỡ mỗi
phần**, tức cái thước. `so_doan` đếm xem cái thước ấy đặt được mấy lần, nên nó
là thứ chưa biết, và nó không có lý do gì phải bằng 3.

Cũng vì thế mà `con_lai` không dừng ở 3: mỗi lần cắt xong máy còn hỏi lại một
lần nữa, và 3 mét cuối vẫn đủ để cắt thêm một đoạn.
::
:::

:::opt
12 rồi 0
::why
Gần đúng ở chỗ bạn đọc `range(12)` là mười hai lượt, và đúng là vòng lặp ghé
qua đủ mười hai lượt — không lượt nào bị bỏ.

Chỗ lệch: `so_doan` chỉ tăng ở những lượt mà **cái thước còn lọt**. Sau bốn lần
cắt thì `con_lai` chỉ còn 0, câu hỏi `0 >= 3` cho ra sai, nên tám lượt còn lại
vòng lặp vẫn ghé qua mà không đếm thêm gì. Số lượt ghé và số đoạn cắt được là
hai chuyện khác nhau.
::
:::

:::opt
4 rồi 12
::why
Gần đúng ở chỗ bạn đếm đúng bốn đoạn — phần khó của bài này bạn đã làm xong.

Chỗ lệch nằm ở dòng `con_lai = con_lai - 3`. Cái tên `con_lai` bị **viết đè**
sau mỗi lần cắt, đúng như Realm 0 đã dạy: vế phải tính xong trước, rồi kết quả
mới được gán lại vào cái tên. Nên tới cuối, `con_lai` không còn giữ cuộn dây
ban đầu nữa; nó giữ chỗ dây chưa cắt, và chỗ đó đã hết sạch.
::
:::
::::

::::explain{#doi-thuoc-thi-doi-so-doan}
Vì con số tìm được là một **số đo**, nó đổi theo cái thước — đúng luật bài 3.

Cùng cuộn dây 12 mét: cắt đoạn 2 mét thì được 6 đoạn; cắt đoạn 6 mét thì được 2
đoạn. Cuộn dây không dài thêm hay ngắn đi mét nào. Thước càng nhỏ thì càng đặt
được nhiều lần.

Để ý chiều của chuyện này ngược với bài trước. Ở chia đều, **càng nhiều phần thì
mỗi phần càng nhỏ**. Ở chia đo, **thước càng nhỏ thì càng nhiều phần**. Cùng
một dấu chia, hai chiều đọc — nên khi gặp một bài toán chia, câu hỏi đầu tiên
đáng hỏi không phải "tính thế nào", mà là **con số sau dấu chia đang đứng vai
nào**.
::::

::::code{#cat-hai-cuon}
Hai lần đo, hai cái thước khác nhau:

- **Cuộn dây của An**: dài **12 mét**, cắt thành từng đoạn **3 mét** để buộc giàn.
- **Cây vải của mẹ Byte**: dài **21 mét**, cắt thành từng khúc **7 mét** để may rèm.

Bốn chỗ trống đều là **cái thước** của lần đo đang chạy — con số quyết định khi
nào thì còn lọt, và mỗi lần lọt thì bớt đi bao nhiêu.

Bài chấm bằng cả hai lần đo, và hai lần được chọn để cho ra **hai số phần khác
nhau** (4 và 3): một con số gõ cứng vào cả bốn chỗ trống thì nhiều nhất chỉ
đúng được một lần đo.

```python title=starter
# Cuộn dây 12 mét, cắt thành từng đoạn 3 mét.
con_lai_day = 12
so_doan_day = 0
for lan in range(12):
    if con_lai_day >= ___:
        con_lai_day = con_lai_day - ___
        so_doan_day = so_doan_day + 1

# Cây vải 21 mét, cắt thành từng khúc 7 mét.
con_lai_vai = 21
so_khuc_vai = 0
for lan in range(21):
    if con_lai_vai >= ___:
        con_lai_vai = con_lai_vai - ___
        so_khuc_vai = so_khuc_vai + 1

print(so_doan_day)
print(so_khuc_vai)
```

```python title=solution
# Cuộn dây 12 mét, cắt thành từng đoạn 3 mét.
con_lai_day = 12
so_doan_day = 0
for lan in range(12):
    if con_lai_day >= 3:
        con_lai_day = con_lai_day - 3
        so_doan_day = so_doan_day + 1

# Cây vải 21 mét, cắt thành từng khúc 7 mét.
con_lai_vai = 21
so_khuc_vai = 0
for lan in range(21):
    if con_lai_vai >= 7:
        con_lai_vai = con_lai_vai - 7
        so_khuc_vai = so_khuc_vai + 1

print(so_doan_day)
print(so_khuc_vai)
```

```python title=test
# Hai lần đo cho hai số phần khác nhau (4 và 3), nên một con số điền bừa vào
# cả bốn chỗ trống không qua nổi cả hai dòng đầu.
#
# Hai assert giữa là chỗ bài này khác bài trước: ghép các đoạn lại phải về
# đúng cuộn ban đầu, và con số nhân với THƯỚC chứ không nhân với số phần.
assert so_doan_day == 4, "thước 3 mét đặt được đúng 4 lần trên cuộn dây 12 mét"
assert so_khuc_vai == 3, "thước 7 mét đặt được đúng 3 lần trên cây vải 21 mét"
assert so_doan_day * 3 == 12, "ghép 4 đoạn, mỗi đoạn 3 mét, phải về đúng cuộn 12 mét"
assert so_khuc_vai * 7 == 21, "ghép 3 khúc, mỗi khúc 7 mét, phải về đúng cây vải 21 mét"
assert con_lai_day == 0, "đo xong mà vẫn còn dây thừa thì cuộc đo chưa chạy hết"
assert con_lai_vai == 0, "cây vải cũng vừa khít, không dư mẩu nào"
```

:::hints
- kind: attention
  body: Cả bốn chỗ trống nằm ở hai chỗ giống nhau trong mỗi khối: một chỗ hỏi "còn lọt không", một chỗ "cắt đi bấy nhiêu". Hai chỗ ấy trong cùng một khối phải mang cùng một con số.
- kind: strategy
  body: Con số ấy là cái thước — cỡ mỗi đoạn, do đề bài cho sẵn chứ không phải thứ đi tìm. Mỗi khối có cái thước riêng, và cả hai đã ghi trong dòng chú thích ngay phía trên.
- kind: one-line
  body: "Hai chỗ trống của khối trên là `3`, hai chỗ trống của khối dưới là `7`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^4\n3\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn đoạn dây, ba khúc vải. Mình không phát cho ai cả — mình chỉ đo.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bây giờ bạn có một cách đọc dấu chia mà bài trước chưa có: **`12 : 3` hỏi "3
lọt vào 12 mấy lần"**. Câu hỏi ấy trả lời được bằng tay, bằng thanh số, bằng
sợi dây — và Byte vừa bắt máy đếm hộ.

Cái thước đổi thì con số đổi, nhưng câu hỏi vẫn trả lời được. Thước 3 mét: bốn
lần. Thước 2 mét: sáu lần. Thước 12 mét: một lần. Thước 24 mét: không lần nào
lọt, và "không lần nào" cũng là một câu trả lời.

Byte tinh nghịch thử nốt cái thước cuối cùng còn lại trong hộp: một đoạn dây
dài **0 mét**.

Đặt nó lên cuộn dây 12 mét. Nó lọt. Bớt đi 0 mét — cuộn dây vẫn còn nguyên 12
mét. Đặt lần nữa: vẫn lọt. Vẫn còn 12 mét.

Vậy **0 lọt vào 12 mấy lần**?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
