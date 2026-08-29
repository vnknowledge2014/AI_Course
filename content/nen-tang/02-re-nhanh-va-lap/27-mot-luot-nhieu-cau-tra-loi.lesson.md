---
id: nen-tang.re-nhanh-va-lap.mot-luot-nhieu-cau-tra-loi
title: Một lượt, nhiều câu trả lời
summary: Nhiều cái tên cùng ghi chép trên một lượt duyệt — và tổng đi cùng số đếm thì có luôn trung bình.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.multi-accumulator]
requires: [core.accumulator, core.augmented-assign, ctrl.for-each, ctrl.if, core.reassign, core.fstring]
concepts: [ctrl.lap, core.bien]
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
Cuốn sổ chỉ có một cuốn. Sao mình lại phải lật nó năm lần?
::::

::::explain{#mot-la-phieu-may-dong-gach}
Bài trước để lại câu hỏi: bản báo cáo cuối tháng cần năm con số. Chẳng lẽ viết
năm vòng lặp, mỗi vòng lo một câu hỏi?

Viết năm vòng thì chạy được thật, không sai chỗ nào. Nhưng trước khi viết, hãy
nhìn cảnh kiểm phiếu ở nhà văn hoá tổ dân phố.

Tối bầu tổ trưởng, thùng phiếu được mở ra. Một người bốc từng lá, đọc to. Trên
tấm bảng phía sau có mấy dòng kẻ sẵn: dòng gạch phiếu cho bác Tư, dòng gạch
phiếu cho cô Bảy, dòng gạch tổng số phiếu đã đọc, dòng gạch phiếu không hợp lệ.
Mỗi lá đọc lên, mấy người cầm phấn cùng nghe — ai thấy phần của mình thì gạch
thêm một vạch vào dòng của mình.

Không ai đề nghị *"đọc hết thùng cho bác Tư trước, rồi đổ phiếu về thùng, đọc
lại từ đầu cho cô Bảy"*. Vì lá phiếu đã nằm trước mắt rồi: mọi câu hỏi về nó
trả lời được ngay trong lần đọc ấy.

Vòng lặp làm việc y như vậy. Mỗi lượt, cái tên `tien` đang giữ số tiền của đúng
một ngày. Mọi câu hỏi về ngày đó đều trả lời được **ngay trong lượt đó** — miễn
là mỗi câu hỏi có một cái tên riêng đứng ngoài vòng, làm dòng gạch cho nó.
::::

::::example{#hai-vong-gop-thanh-mot}
Sổ chi tiêu bảy ngày của Byte, tính bằng nghìn đồng. Hai câu hỏi: **cả tuần
tiêu hết bao nhiêu**, và **sổ ghi được mấy ngày**.

Viết theo lối mỗi câu một vòng thì ra thế này:

```python title=readonly
chi_tieu = [120, 250, 90, 310, 150, 100, 240]

tong = 0
for tien in chi_tieu:
    tong += tien

ngay = 0
for tien in chi_tieu:
    ngay += 1

print(f"Cả tuần tiêu {tong} nghìn trong {ngay} ngày")
```

```text title=readonly
Cả tuần tiêu 1260 nghìn trong 7 ngày
```

Đúng số. Nhưng đọc kỹ vòng thứ hai: nó lật lại cả bảy ô của cuốn sổ, và trong
thân nó thậm chí **không đụng tới** cái tên `tien`. Nó chỉ cần biết "có thêm một
lượt nữa" — thứ mà vòng thứ nhất cũng đã biết, ở đúng những lượt ấy, vài dòng
phía trên.

Gộp hai dòng gạch lên cùng một tấm bảng:

```python title=readonly
chi_tieu = [120, 250, 90, 310, 150, 100, 240]

tong = 0
ngay = 0

for tien in chi_tieu:
    tong += tien
    ngay += 1

print(f"Cả tuần tiêu {tong} nghìn trong {ngay} ngày")
print(f"Trung bình mỗi ngày {tong / ngay} nghìn")
```

```text title=readonly
Cả tuần tiêu 1260 nghìn trong 7 ngày
Trung bình mỗi ngày 180.0 nghìn
```

Hai cái tên, hai dòng trong thân vòng, một lượt duyệt. Mỗi dòng chỉ chạm vào
cái tên của riêng nó: `tong += tien` không biết `ngay` tồn tại, và ngược lại.
Chúng chỉ tình cờ đi chung một chuyến.

Và đây là chỗ được lời. Dòng cuối in ra một con số mà **không cái tên nào một
mình trả lời nổi**: trung bình mỗi ngày là tổng chia cho số ngày. Nếu hai câu
hỏi ấy được giao cho hai vòng lặp rời nhau thì bạn vẫn ghép được — nhưng chính
vì chúng đi cùng một chuyến, tới cuối vòng cả hai con số đã sẵn sàng cùng lúc,
và phép chia chỉ còn là một dòng.

Một chi tiết nhỏ: máy in `180.0` chứ không phải `180`. Dấu `/` một gạch luôn
cho ra số có phần lẻ, đúng như Realm 0 đã nói — kể cả khi chia hết. Ở đây
`1260 / 7` chia trọn nên phần lẻ là `.0`, và máy vẫn ghi nó ra.

Dấu `//` hai gạch của T1.1 thì khác hẳn: đặt giữa hai số nguyên, nó trả về một
`int` thật, `1260 // 7` ra `180` không đuôi.
::::

::::predict{#doan-ba-dong-in-ra commitOnce}
Byte chép lại đoạn trên, nhưng dòng `ngay = 0` bị tụt vào trong thân vòng —
thụt lề bằng đúng hai dòng dưới nó.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra ba con số nào?

```python title=readonly
chi_tieu = [120, 250, 90, 310, 150, 100, 240]
tong = 0

for tien in chi_tieu:
    ngay = 0
    tong += tien
    ngay += 1

print(f"Tổng: {tong}")
print(f"Số ngày: {ngay}")
print(f"Trung bình: {tong / ngay}")
```

:::opt{correct}
Tổng: 1260 · Số ngày: 1 · Trung bình: 1260.0
:::

:::opt
Tổng: 1260 · Số ngày: 7 · Trung bình: 180.0
::why
Gần đúng ở chỗ bạn đọc `tong` chính xác từng lượt một: nó sinh ra **trước** vòng
nên mỗi lượt nó cộng thêm mà không mất phần cũ, và 1260 đúng là tổng của cả bảy
ngày.

Chỗ lệch nằm ở mức thụt lề của dòng `ngay = 0`. Nó nằm trong thân vòng, nên nó
là một dòng của **mỗi lượt**, không phải một dòng chuẩn bị chạy một lần. Đầu mỗi
lượt nó dựng `ngay` về 0, xoá sạch vạch của lượt trước; rồi `ngay += 1` vạch lại
đúng một vạch. Bảy lượt như nhau, nên cuối vòng `ngay` bằng 1.

Hai cái tên nằm trong cùng một thân vòng nhưng ở hai mức thụt lề khác nhau thì
sống hai đời khác nhau — đó là chỗ đáng nhìn kỹ nhất khi một lượt gánh nhiều
việc.
::
:::

:::opt
Tổng: 240 · Số ngày: 1 · Trung bình: 240.0
::why
Gần đúng ở chỗ bạn nhận ra dòng nằm trong thân vòng thì mỗi lượt nó dựng lại cái
tên từ đầu, nên cuối vòng cái tên ấy chỉ còn dấu vết của lượt cuối cùng. Cách
đọc đó chính xác, và nó cho ra `Số ngày: 1` đúng như đáp án.

Chỗ lệch là ở chỗ luật ấy áp cho **cái tên nào**. Chỉ `ngay = 0` bị tụt vào thân
vòng thôi; `tong = 0` vẫn nằm nguyên ngoài vòng, sát lề trái. Cái tên không bị
dựng lại thì vẫn cộng dồn qua đủ bảy ngày, nên `tong` vẫn là 1260 — chỉ có
`ngay` là mất trí nhớ.
::
:::

:::opt
Máy dừng lại và báo `ZeroDivisionError`, vì `ngay` bằng 0
::why
Gần đúng ở chỗ bạn nhìn thấy dòng `ngay = 0` chạy đi chạy lại mỗi lượt rồi nghĩ
ngay tới hậu quả ở dòng chia cuối bài. Nối hai chỗ cách xa nhau như vậy là đúng
cách đọc code.

Chỗ lệch: `ngay = 0` không phải dòng **cuối** của thân vòng. Ngay dưới nó còn
`ngay += 1`, và dòng đó chạy sau. Nên kết thúc mỗi lượt `ngay` bằng 1, không
phải 0. Nếu dòng `ngay += 1` cũng biến mất thì đúng là bạn được `ZeroDivisionError`
thật.
::
:::
::::

::::explain{#nhip-ba-cua-tung-cai-ten}
Mỗi cái tên làm nhiệm vụ ghi chép đều đi theo đúng một nhịp ba, và ba chặng ấy
nằm ở ba mức thụt lề khác nhau:

- **Sinh ra trước vòng** — sát lề trái, chạy đúng một lần. Đây là lúc kẻ dòng
  trên bảng.
- **Được chạm trong vòng** — thụt vào trong thân, chạy mỗi lượt một lần. Đây là
  lúc gạch thêm vạch.
- **Đọc lại sau vòng** — sát lề trái trở lại, khi cuốn sổ đã lật xong.

Thêm một câu hỏi vào bản báo cáo nghĩa là thêm một cái tên đi trọn ba chặng đó.
Tấm bảng không giới hạn số dòng gạch:

```python title=readonly
chi_tieu = [120, 250, 90, 310, 150, 100, 240]
nguong = 200

tong = 0
ngay = 0
so_ngay_vuot = 0

for tien in chi_tieu:
    tong += tien
    ngay += 1
    if tien > nguong:
        so_ngay_vuot += 1

print(f"{ngay} ngày, tổng {tong} nghìn, trung bình {tong / ngay} nghìn")
print(f"Số ngày tiêu quá {nguong} nghìn: {so_ngay_vuot}")
```

```text title=readonly
7 ngày, tổng 1260 nghìn, trung bình 180.0 nghìn
Số ngày tiêu quá 200 nghìn: 3
```

Cái tên thứ ba khác hai cái kia ở đúng một điểm: dòng gạch của nó nằm trong thân
một `if`, nên nó chỉ vạch ở những lượt đáng vạch. Nhưng nhịp ba thì vẫn nguyên
vẹn, và vòng lặp vẫn chỉ có một.

Còn một chỗ **không** gộp vào lượt duyệt được, và biết trước thì đỡ mất buổi
tối: câu hỏi nào cần biết kết quả của **cả cuốn sổ** rồi mới trả lời được cho
từng ngày. Viết `print(tong / ngay)` vào giữa thân vòng thì máy in ra bảy dòng,
mỗi dòng là trung bình của phần sổ đã đi qua tới lúc đó — `120.0`, `185.0`,
`153.33…` — chứ không phải trung bình của cả tuần. Một lượt duyệt trả lời được
mọi câu hỏi **về từng ngày**; câu hỏi đem từng ngày ra so với con số của cả
cuốn sổ thì phải đợi lượt sau.
::::

::::code{#hai-cuon-so-mot-luot}
Byte đưa **hai** cuốn sổ, và hỏi cùng ba câu cho cả hai: mấy ngày, tổng bao
nhiêu, trung bình mỗi ngày bao nhiêu. Nên đoạn dưới có hai khối giống nhau từng
chữ một, chỉ khác đúng dãy số trong `chi_tieu`.

Hai cuốn cố ý khác nhau cả về **độ dài** lẫn **tổng tiền**: tuần này bảy ngày,
tuần trước năm ngày. Mỗi khối hở hai chỗ — một chỗ trong thân vòng, một chỗ
trong dòng in. Cùng hai chỗ ấy, điền hai lần y hệt nhau.

```python title=starter
chi_tieu = [120, 250, 90, 310, 150, 100, 240]
tong = 0
ngay = 0

for tien in chi_tieu:
    tong += tien
    ___

print(f"Tuần này: {ngay} ngày, tổng {tong} nghìn, trung bình {___} nghìn")

chi_tieu = [200, 160, 90, 110, 190]
tong = 0
ngay = 0

for tien in chi_tieu:
    tong += tien
    ___

print(f"Tuần trước: {ngay} ngày, tổng {tong} nghìn, trung bình {___} nghìn")
```

```python title=solution
chi_tieu = [120, 250, 90, 310, 150, 100, 240]
tong = 0
ngay = 0

for tien in chi_tieu:
    tong += tien
    ngay += 1

print(f"Tuần này: {ngay} ngày, tổng {tong} nghìn, trung bình {tong / ngay} nghìn")

chi_tieu = [200, 160, 90, 110, 190]
tong = 0
ngay = 0

for tien in chi_tieu:
    tong += tien
    ngay += 1

print(f"Tuần trước: {ngay} ngày, tổng {tong} nghìn, trung bình {tong / ngay} nghìn")
```

```python title=test
# Chấm bằng TRỌN VẸN hai dòng output, không phải một dòng.
#
# Một cuốn sổ thì chưa phân biệt được gì: gõ thẳng `7` vào chỗ đếm và `180.0`
# vào chỗ trung bình cũng ra đúng dòng mà khối ấy mong đợi. Cuốn thứ hai dài
# năm ngày và trung bình 150.0, nên mọi câu gõ cứng theo cuốn thứ nhất đều lộ
# ngay ở dòng in thứ hai.
#
# Khối này khẳng định ba cái tên sau khi chạy hết đoạn đang giữ kết quả của
# cuốn sổ thứ hai — tức là vòng thứ hai đã đi trọn năm lượt, không dừng sớm và
# không mang theo con số của tuần trước đó.
assert ngay == 5, "sổ tuần trước ghi được năm ngày, nên lượt duyệt cuốn ấy phải gạch đúng năm vạch — không mang theo bảy vạch của tuần này"
assert tong == 750, "năm ngày của tuần trước cộng lại hết 750 nghìn"
assert tong / ngay == 150.0, "750 nghìn chia cho năm ngày là mỗi ngày 150 nghìn — trung bình chỉ ra đúng khi cả tổng lẫn số ngày cùng là của một cuốn sổ"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm ngay dưới `tong += tien`, cùng mức thụt lề với nó — nó là dòng gạch của cái tên còn lại, cái tên đang được in ra ở đầu dòng kết quả. Chỗ trống thứ hai nằm trong dòng `print`, sau chữ "trung bình".
- kind: strategy
  body: Cái tên `ngay` sinh ra bằng 0 trước vòng, nên trong thân vòng nó cần một dòng nhích nó lên một sau mỗi ngày đọc được — không phụ thuộc vào số tiền của ngày đó. Còn trung bình mỗi ngày thì lấy tổng chia cho số ngày, và tới sau vòng thì hai con số ấy đều đã sẵn sàng.
- kind: one-line
  body: "Trong thân vòng viết `ngay += 1`; trong dòng `print` viết `tong / ngay`. Điền y hệt vào **cả hai** khối."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Tuần này: 7 ngày, tổng 1260 nghìn, trung bình 180\.0 nghìn\nTuần trước: 5 ngày, tổng 750 nghìn, trung bình 150\.0 nghìn\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cuốn sổ, mỗi cuốn lật đúng một lần, ba câu trả lời mỗi cuốn. Bảng gạch làm
được việc thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ba con số ra từ một lượt, và thêm cái tên thứ tư, thứ năm cũng chỉ là thêm dòng
gạch trên cùng tấm bảng ấy. Bản báo cáo cuối tháng của Byte — tổng, trung bình,
số ngày vượt ngưỡng, ngày kỷ lục, ngày đầu tiên vượt — nằm gọn trong một vòng
lặp.

Bây giờ Byte đưa thêm sổ tháng Hai. Cùng những câu hỏi ấy, chỉ khác dãy số.

Cách duy nhất bạn đang có là bôi đen cả đoạn vòng lặp vừa viết, chép xuống dưới,
rồi sửa dãy số. Tháng Ba thì chép thêm lần nữa. Trên màn hình sẽ có ba đoạn gần
giống hệt nhau — và cả ba đều gọi cùng những cái tên `tong`, `ngay`.

Chép lần thứ hai thì còn chịu được. Nhưng sửa một chỗ trong đoạn đầu, hai đoạn
kia có tự đổi theo không?
::::

::::checkpoint{mastery=0.8}
::::
