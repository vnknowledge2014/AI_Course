---
id: nen-tang.re-nhanh-va-lap.cong-don-qua-tung-luot
title: Cộng dồn qua từng lượt
summary: Cái tên giữ tổng phải sinh ra trước vòng lặp — khai nó trong thân thì mỗi lượt xoá sạch công của lượt trước.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.accumulator]
requires: [core.read-modify-write, ctrl.for-each, core.list, core.variable, core.fstring]
concepts: [core.bien, core.gan-lai, ctrl.lap]
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
Muốn cộng dồn thì phải có chỗ để dồn vào. Chỗ ấy sinh ra lúc nào?
::::

::::explain{#thay-mot-bang-so-tien}
Bài trước, dòng đếm số món của bạn là:

```python
mon = mon + 1
```

Đọc – sửa – ghi: lấy con số cũ, cộng thêm `1`, dán lại cái tên. Con số `1` nằm
ở vế phải của một phép cộng, mà vế phải thì thay được. Thay `1` bằng số tiền
tiêu của chính ngày đang xét:

```python
tong = tong + tien
```

Cơ chế y hệt, chỉ khác thứ được cộng vào. Sau sáu lượt, `tong` không còn giữ
*số lượt* nữa — nó giữ **tổng số tiền của sáu ngày**. Một cái tên dùng theo
kiểu này, lớn dần qua từng lượt và chỉ đọc ra ở cuối, có tên riêng: **biến tích
luỹ** (tiếng Anh: *accumulator*).

Bây giờ tới chỗ khó thật sự, và nó không nằm ở dòng cộng.

Chữ **đọc** trong đọc – sửa – ghi đòi cái tên `tong` phải đã mang một giá trị
từ trước. Ở lượt đầu tiên thì chưa ai cho nó gì cả, nên phải có một dòng khai
nó ra:

```python
tong = 0
```

Con số 0 là điểm xuất phát đúng của một phép cộng: cộng thêm gì vào 0 cũng ra
đúng thứ đó, nên nó không làm lệch kết quả.

Câu hỏi duy nhất của bài này: dòng ấy đặt ở đâu?

Có hai chỗ nghe đều lọt tai. Đặt **trước** vòng, sát lề trái. Hoặc đặt **trong**
thân vòng, ngay trên dòng cộng — nghe cũng hợp lý, vì "mỗi lượt đều cần `tong`
thì mỗi lượt đều khai nó ra".

Hai chỗ ấy cho hai kết quả khác nhau. Và cái sai thì không kèm theo dòng chữ đỏ
nào cả.
::::

::::example{#ba-buoi-sang-cua-ba-chu}
Bà chủ quán phở ghi tiền bán được ba buổi sáng, rồi nhờ Byte cộng lại:

```python title=readonly
tien_ban = [540000, 675000, 480000]
tong = 0
for tien in tien_ban:
    tong = tong + tien
print(f"Ba buổi bán được {tong} đồng")
```

Máy in ra:

```text
Ba buổi bán được 1695000 đồng
```

Đi theo từng lượt, ghi lại con số mà `tong` đang mang:

- **Trước vòng** — `tong = 0` chạy đúng một lần. `tong` là `0`.
- **Lượt 1** — `tien` là `540000`. Đọc `0`, cộng, ghi. `tong` là `540000`.
- **Lượt 2** — `tien` là `675000`. Đọc `540000`, cộng, ghi. `tong` là
  `1215000`.
- **Lượt 3** — `tien` là `480000`. Đọc `1215000`, cộng, ghi. `tong` là
  `1695000`.
- **Sau vòng** — `print` đọc `tong` lần cuối.

Cái giữ cho ba lượt nối được vào nhau chính là chuyện `tong` **sống sót qua
mỗi lượt**. Lượt 2 đọc được `540000` là nhờ lượt 1 đã ghi con số ấy lên cái tên
và không ai xoá đi.
::::

::::predict{#khai-o-trong-than commitOnce}
Byte thấy dòng `tong = 0` đứng chơ vơ một mình ngoài vòng nên dọn nó vào trong
thân cho gọn. Ba con số vẫn thế.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
tien_ban = [540000, 675000, 480000]
for tien in tien_ban:
    tong = 0
    tong = tong + tien
print(f"Ba buổi bán được {tong} đồng")
```

:::opt{correct}
Ba buổi bán được 480000 đồng
:::

:::opt
Ba buổi bán được 1695000 đồng
::why
Gần đúng ở chỗ bạn thấy cả bốn dòng vẫn còn nguyên, không dòng nào bị xoá, và
dòng cộng vẫn nằm đúng chỗ của nó trong thân. Nếu chuyện duy nhất đáng kể là
*có mặt hay không*, thì con số phải y như cũ.

Chỗ lệch: `tong = 0` bây giờ nằm trong thân, nên nó chạy **ba lần** chứ không
phải một lần — và mỗi lần nó dán lại cái tên `tong` lên số `0`. Đầu lượt 2,
con số `540000` mà lượt 1 vừa ghi bị thay bằng `0` trước khi phép cộng kịp đọc
nó. Mỗi lượt bắt đầu lại từ đầu.
::
:::

:::opt
Ba buổi bán được 0 đồng
::why
Gần đúng ở chỗ bạn bắt đúng thủ phạm: dòng `tong = 0` chạy lại ở mỗi lượt và
xoá sạch công của lượt trước. Nửa đầu suy luận của bạn chính xác.

Chỗ lệch nằm ở thứ tự hai dòng trong thân. `tong = 0` đứng **trên** dòng cộng,
nên ở mỗi lượt, xoá xong rồi mới cộng — và lượt cuối cùng cộng vào `480000`.
Máy ra khỏi vòng ngay sau đó, không còn lần xoá nào nữa. Muốn ra `0` thì dòng
`tong = 0` phải là dòng **cuối** của thân.
::
:::

:::opt
Máy báo `NameError`, vì `tong` sinh ra bên trong thân thì ra ngoài không còn
::why
Gần đúng ở chỗ bạn nối được hai việc lại với nhau: `print` nằm ngoài vòng, còn
cái tên `tong` thì được khai bên trong — nên bạn hỏi liệu ra ngoài có còn gọi
được nó không. Đó là một câu hỏi đúng chỗ, và trong nhiều ngôn ngữ lập trình
khác thì câu trả lời đúng là "không còn".

Chỗ lệch nằm ở Python. Thân vòng lặp **không** dựng lên một khu vực riêng cho
tên: một cái tên khai trong thân vẫn dùng được sau khi vòng kết thúc, và nó giữ
đúng giá trị của lượt cuối cùng. Đó cũng chính là lý do cả bài này chạy được —
`tong` sống sót ra tới dòng `print`.
::
:::
::::

::::explain{#khai-truoc-vong}
Một câu là đủ cho cả bài: **biến tích luỹ phải sinh ra trước vòng lặp.**

Lý do nằm ở chỗ hai dòng ấy có nhịp chạy khác nhau, và bạn chọn nhịp bằng cách
chọn mức thụt lề:

- `tong = 0` — chạy **một lần**, dựng điểm xuất phát. Sát lề trái, ngoài vòng.
- `tong = tong + tien` — chạy **mỗi lượt**, nối lượt này vào công của lượt
  trước. Thụt vào, trong thân.

Kéo dòng thứ nhất vào trong thân là biến việc-làm-một-lần thành
việc-làm-mỗi-lượt. Nó vẫn dựng điểm xuất phát, chỉ có điều nó dựng lại điểm
xuất phát ngay trước mỗi phép cộng, nên chẳng lượt nào biết tới lượt nào.

> Chỗ dễ vấp: đoạn sai vẫn **chạy trót lọt**. Không `SyntaxError`, không
> `NameError`, màn hình hiện ra một con số trông rất đàng hoàng — chỉ có điều
> nó là tiền của buổi cuối chứ không phải tổng ba buổi. Đây là loại sai máy
> không nhắc bạn, và cách bắt nó là nhẩm tay một trường hợp nhỏ rồi đối chiếu:
> ba con số ấy cộng lại phải hơn triệu rưỡi, mà màn hình chỉ hiện 480 nghìn.
::::

::::code{#so-chi-tieu-ca-tuan}
Sổ chi tiêu sáu ngày của Byte đây. Đoạn dưới **chạy được** và in ra một con số,
nhưng con số cuối cùng là tiền của riêng ngày cuối chứ không phải cả tuần — vì
dòng khai đang nằm sai chỗ.

Hãy chuyển nó về đúng chỗ, để dòng cuối in ra tổng của cả sáu ngày.

```python title=starter
chi_tieu = [45000, 120000, 30000, 260000, 85000, 210000]

for tien in chi_tieu:
    tong = 0
    tong = tong + tien
    print(f"Cộng xong một ngày, tổng đang là {tong} đồng")

print(f"Cả tuần tiêu {tong} đồng")
```

```python title=solution
chi_tieu = [45000, 120000, 30000, 260000, 85000, 210000]
tong = 0

for tien in chi_tieu:
    tong = tong + tien
    print(f"Cộng xong một ngày, tổng đang là {tong} đồng")

print(f"Cả tuần tiêu {tong} đồng")
```

```python title=test
# Byte cộng tay sáu con số trong sổ và được 750000. Dòng báo cáo giữa vòng
# cũng phải lớn dần lên, nên đây không phải chuyện gán bừa một con số ở cuối.
assert tong == 750000
```

:::hints
- kind: attention
  body: So mức thụt lề của hai dòng bắt đầu bằng chữ `tong`. Cả hai đang thụt vào bốn dấu cách, nghĩa là cả hai đều chạy ở mỗi lượt — trong khi chỉ một trong hai cần chạy mỗi lượt.
- kind: strategy
  body: Dòng dựng điểm xuất phát chỉ được chạy đúng một lần, trước khi lượt đầu tiên bắt đầu. Muốn vậy thì nó phải nằm **ngoài** thân vòng, tức là viết sát lề trái và đặt trên dòng `for`. Dòng cộng thì ở nguyên trong thân.
- kind: one-line
  body: Cắt dòng `tong = 0` khỏi thân vòng, dán nó lên trên dòng `for` và bỏ hết phần thụt lề để nó sát lề trái.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Cộng xong một ngày, tổng đang là 165000 đồng
- tier: output
  expect: Cả tuần tiêu 750000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu ngày dồn vào một chỗ, vì cái chỗ ấy chỉ được dựng lên một lần.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn kỹ dòng làm hết việc trong bài này:

```python
tong = tong + tien
```

Cái tên `tong` phải gõ **hai lần trên cùng một dòng**, và hai lần ấy buộc phải
giống nhau tới từng chữ cái. Một cái tên dài hơn — `tong_chi_thang_nay` chẳng
hạn — là hai lần gõ một chuỗi hai mươi ký tự, và chúng nằm cách nhau đúng ba ký
tự trên màn hình.

Chuyện phiền không phải là mỏi tay. Chuyện phiền là gõ lệch một chữ:

```python
tong = tng + tien
```

Dòng này viết đúng cú pháp, máy không kêu lúc bạn đang gõ. Nó chỉ nói ra lúc
chạy, và điều nó nói lại là về một cái tên bạn chưa từng có ý định tạo ra.

Đọc – sửa – ghi thì cần cả ba việc, không bỏ được việc nào. Nhưng có nhất thiết
phải **gõ tên hai lần** mới ra lệnh được cho cả ba việc ấy không? Bài sau trả
lời.
::::

::::checkpoint{mastery=0.8}
::::
