---
id: nen-tang.list-dict-set-tuple.mo-cap-ra-hai-ten
title: Mở cặp ra hai cái tên
summary: Đặt hai cái tên ngay trên dòng `for` thì mỗi lượt cặp tự tháo ra — `for ten, tien in chi.items()` và trong thân vòng không còn con số ngoặc vuông nào.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.for-unpack]
practices: [core.tuple-unpack, core.accumulator, core.fstring, ctrl.for-each]
requires: [core.tuple, core.tuple-immutable, core.tuple-unpack, core.multi-assign, core.dict, core.dict-items, core.value-error, core.accumulator, core.arithmetic, ctrl.for-each, core.fstring]
concepts: [core.gia-tri, core.bien, ctrl.lap]
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
Đừng cầm cả tấm thẻ rồi đếm ô. Nhận xong là mở ra luôn.
::::

::::explain{#mo-ngay-luc-nhan}
Bài trước để lại một chỗ khó chịu: cặp thì an toàn, nhưng đọc nó phải đếm ô.
`cap[0]` là tên hay là tiền — câu đó không có trên màn hình, nó nằm trong trí
nhớ của bạn, và trí nhớ thì không báo lỗi khi nhầm.

Động tác chữa chỗ này bạn đã làm rồi, ở mạch Hàm. Lúc ấy hàm đưa ra một cái gói
hai ô, và thay vì giữ nguyên cái gói rồi mỗi lần lại thò tay đếm, bạn đặt **hai
cái tên** bên trái dấu bằng:

```python
tien_hang, tien_thue = tinh_hoa_don(45000, 2)
```

Vế phải là một cái gói, vế trái là hai cái tên, và máy chia theo thứ tự đứng.
Cách viết ấy tên là **mở gói**.

Máy không quan tâm cái gói từ đâu tới. Nó tới từ một lời gọi hàm cũng được, mà
nó đang nằm sẵn trong tay bạn cũng được:

```python
cap = ("sửa xe", 500000)
ten, tien = cap
```

Hai dòng ấy chạy y hệt: `ten` nhận ô 0, `tien` nhận ô 1. Từ đó trở đi trong
chương trình không còn con số nào trong ngoặc vuông nữa, chỉ còn hai cái tên tự
nói ra mình là gì.

Và đây là chỗ mới của bài này — chỗ **đặt** động tác ấy. Bạn không phải mở gói ở
một dòng riêng bên trong thân vòng. Hai cái tên đặt được thẳng lên **dòng `for`**,
ngay chỗ mà cho tới giờ bạn chỉ viết đúng một cái tên:

```python
for ten, tien in chi.items():
    print(f"{ten} tiêu hết {tien} đồng")
```

Đọc dòng ấy như đọc một câu tiếng Việt: *với mỗi tên và tiền trong sổ chi*. Mỗi
lượt, `.items()` thả ra một cặp, và cặp ấy được tháo ngay tại chỗ thành hai cái
tên trước khi thân vòng bắt đầu.

Số tên bên trái phải khớp số ô trong cặp — đúng luật bạn đã gặp lúc mở gói ở
mạch Hàm. Cặp có hai ô thì viết hai tên; viết ba tên thì máy dừng lại và kêu
`ValueError`, vì nó không biết lấy đâu ra ô thứ ba.
::::

::::example{#hai-cach-viet-cung-mot-vong}
Cùng một cuốn sổ, cùng một bản báo cáo, viết hai lần — lần đầu đếm ô, lần sau mở
cặp ngay trên dòng `for`:

```python title=readonly
chi = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}

print("— cách đếm ô —")
for cap in chi.items():
    print(f"{cap[0]}: {cap[1]} đồng")

print("— cách mở cặp —")
for ten, tien in chi.items():
    print(f"{ten}: {tien} đồng")
```

Máy in ra:

```text
— cách đếm ô —
sửa xe: 500000 đồng
cà phê: 25000 đồng
biếu bà: 300000 đồng
— cách mở cặp —
sửa xe: 500000 đồng
cà phê: 25000 đồng
biếu bà: 300000 đồng
```

Hai vòng cho ra sáu dòng giống hệt nhau từng chữ. Máy chấm cả hai đều đúng.
Khác biệt nằm ở chỗ khác:

- **Vòng trên** có hai con số trong ngoặc vuông, và cả hai đều câm. Đọc lại sau
  một tuần, bạn phải quay về bài `.items()` để nhớ ô nào đứng trước.
- **Vòng dưới** không còn con số nào. Hai chữ `ten` và `tien` tự nói ra chúng
  đang giữ gì, ngay trên dòng đầu tiên của vòng lặp.

Chỗ đáng để ý nhất: hai cái tên ấy là **tên của bạn**. Máy không hề đọc chữ
`ten` để đoán rằng nó phải nhận một cái tên khoản. Nó chia theo **vị trí** và
chỉ theo vị trí — tên thứ nhất nhận ô 0, tên thứ hai nhận ô 1. Viết `for a, b in
chi.items():` thì `a` vẫn nhận tên khoản như thường.
::::

::::predict{#doan-khi-viet-nguoc commitOnce}
Byte gõ vội và đặt hai cái tên ngược thứ tự trên dòng `for`, còn dòng `print`
bên trong thì giữ nguyên.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
chi = {"sửa xe": 500000, "cà phê": 25000}

for tien, ten in chi.items():
    print(f"{ten}: {tien} đồng")
```

:::opt{correct}
Hai dòng `500000: sửa xe đồng` và `25000: cà phê đồng` — máy không báo lỗi nào
:::

:::opt
Hai dòng `sửa xe: 500000 đồng` và `cà phê: 25000 đồng`, vì máy biết `ten` phải nhận tên khoản
::why
Gần đúng ở chỗ bạn đọc code như đọc tiếng Việt, và đó là một thói quen tốt —
người viết đặt tên `ten` chính là để người đọc hiểu ngay ô đó chứa gì.

Chỗ lệch: cái tên chỉ nói với **người đọc**, không nói với máy. Với máy, `ten`
và `tien` chỉ là hai cái móc trống, và nó treo ô 0 lên móc thứ nhất, ô 1 lên móc
thứ hai, bất kể hai móc ấy mang chữ gì. Ở đây móc thứ nhất tên là `tien`, nên
`tien` nhận chuỗi `"sửa xe"`.
::
:::

:::opt
Máy dừng và báo `ValueError`, vì hai cái tên đặt sai thứ tự
::why
Gần đúng ở chỗ bạn nhớ đúng một luật có thật của phép mở gói: số tên bên trái
phải khớp số ô bên phải, lệch số là `ValueError` — bạn đã gặp nó ở mạch Hàm.

Chỗ lệch: luật ấy chỉ **đếm**, nó không xét nghĩa. Ở đây hai tên và hai ô, số
khớp nhau hoàn hảo, nên máy chia xong và đi tiếp không vướng gì. Thứ tự có hợp
lý hay không là chuyện của người viết, và máy không có cách nào biết.
::
:::

:::opt
Máy dừng và báo `TypeError`, vì `tien` đang giữ một chuỗi chứ không phải số
::why
Gần đúng ở chỗ bạn nhớ đúng lúc nào `TypeError` xuất hiện: khi một giá trị bị
đem đi làm việc mà kiểu của nó không nhận, chẳng hạn đem một chuỗi đi trừ hay
đi chia.

Chỗ lệch: trong đoạn này không có phép tính nào cả. Việc duy nhất làm với `tien`
là chèn nó vào một câu chữ, mà chèn vào câu chữ thì kiểu nào cũng chèn được —
số ra số, chuỗi ra chuỗi. Nên máy in ra êm ru, và đó mới đúng là chỗ đáng ngại:
sai kiểu này không kêu tiếng nào.
::
:::
::::

::::code{#bao-cao-gon-va-mot-dong-tong}
Cuốn sổ ba khoản. Byte muốn mỗi khoản một dòng, rồi một dòng tổng ở cuối.

Chỗ trống nằm ngay trên dòng `for`, đúng chỗ cho tới giờ bạn chỉ viết một cái
tên. Cả thân vòng bên dưới đã viết sẵn và nó gọi thẳng hai cái tên — nên chỗ
trống phải cho ra đúng hai cái tên ấy, đúng thứ tự.

```python title=starter
chi = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}
tong = 0

for ___ in chi.items():
    print(f"{ten}: {tien} đồng")
    tong = tong + tien

print(f"Tổng: {tong} đồng")
```

```python title=solution
chi = {"sửa xe": 500000, "cà phê": 25000, "biếu bà": 300000}
tong = 0

for ten, tien in chi.items():
    print(f"{ten}: {tien} đồng")
    tong = tong + tien

print(f"Tổng: {tong} đồng")
```

```python title=test
# Hai câu canh hai vai khác nhau của cùng một chỗ trống.
# Đặt hai tên ngược thứ tự thì `tong = tong + tien` đem một chuỗi đi cộng vào
# một con số và chương trình nổ ngay ở lượt đầu — không có đường nào lách qua.
assert tong == 825000, "ba khoản trong sổ này là 500000, 25000 và 300000, cộng lại đúng 825000 — cái tên đứng SAU dấu phẩy phải đang giữ số tiền của từng cặp"
assert ten == "biếu bà", "cặp cuối cùng của sổ này là ('biếu bà', 300000), nên sau khi vòng chạy xong, cái tên đứng TRƯỚC dấu phẩy phải còn giữ chuỗi 'biếu bà'"
```

:::hints
- kind: attention
  body: Đọc hai dòng trong thân vòng trước khi điền. Chúng gọi tới những cái tên nào, và những cái tên ấy đã được đặt ra ở dòng nào trong chương trình?
- kind: strategy
  body: '`.items()` thả ra một cặp mỗi lượt, và cặp có hai ô. Bên trái chữ `in` bạn đặt đúng số tên bằng số ô, ngăn nhau bằng một dấu phẩy — hình dạng y như lúc mở gói ở mạch Hàm, chỉ khác chỗ đứng. Thứ tự thì theo thứ tự ô của cặp, và trong sổ tra cứu thì khoá đứng trước.'
- kind: one-line
  body: "Viết `ten, tien` vào chỗ trống, giữ nguyên chữ `in` và dấu hai chấm cuối dòng."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: sửa xe: 500000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không còn ngoặc vuông nào trong thân vòng. Đọc một lượt là hiểu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bản báo cáo in gọn rồi: mỗi dòng một tên, một số tiền, và một dòng tổng ở cuối.
Đưa cho ai đọc cũng hiểu.

Nhưng thử nhìn lại **thứ tự** các dòng ấy. Chúng hiện ra đúng thứ tự bạn ghi vào
sổ: sửa xe trước vì hôm đó bạn ghi trước, cà phê sau vì ghi sau. Sổ chi tiêu
thật của một tháng thì không phải ba dòng — nó bốn mươi dòng.

Sếp hỏi một câu rất bình thường: **khoản tốn nhất tháng này là khoản nào?**

Với ba dòng thì bạn liếc một cái là ra. Với bốn mươi dòng, cách duy nhất bạn
đang có là dò bằng mắt từ trên xuống dưới, nhớ con số lớn nhất trong đầu, và hy
vọng không bỏ sót dòng nào.

Cuốn sổ vẫn nằm nguyên đó, đúng thứ tự ghi vào. Có cách nào bảo máy **xếp lại**
giúp bạn — để những con số ấy hiện ra theo thứ tự lớn nhỏ chứ không theo thứ tự
bạn đã ghi?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
