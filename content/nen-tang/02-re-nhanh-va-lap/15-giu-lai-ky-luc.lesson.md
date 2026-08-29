---
id: nen-tang.re-nhanh-va-lap.giu-lai-ky-luc
title: Giữ lại kỷ lục
summary: Mỗi lượt so con số mới với con số đang giữ, và chỉ thay thế khi nó lớn hơn — cách một cái tên tìm ra giá trị cao nhất cả sổ.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.max-tracker]
requires: [core.counter-if, core.augmented-assign, ctrl.for-each, ctrl.if, ctrl.comparison, core.reassign, core.list-index, core.fstring]
concepts: [core.bien, core.gan-lai, core.so-sanh, ctrl.lap]
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
Con số bạn cần đã nằm sẵn trong sổ rồi. Mình chỉ phải không đánh rơi nó thôi.
::::

::::explain{#giu-thay-vi-cong}
Vẫn sổ chi tiêu sáu ngày ấy:

```python
chi_tieu = [45000, 120000, 30000, 260000, 85000, 210000]
```

Câu hỏi hôm nay: **ngày tiêu nhiều nhất tiêu hết bao nhiêu?**

Câu trả lời là `260000`. Để ý một điều: con số đó **đã có sẵn** trong sổ, ở đúng
một ngày. Bạn không phải tạo ra nó bằng phép tính nào cả — bạn chỉ phải đi qua
sáu ngày mà không đánh rơi nó.

Đó là chỗ khác hẳn mọi thứ bạn làm với một vòng lặp tới giờ. Cộng dồn thì cái tên
**lớn dần lên** và cuối cùng giữ một con số chưa từng có trong sổ (750000). Đếm
vạch thì cái tên **lớn dần lên** từng nấc một. Lần này cái tên không lớn dần: nó
**đổi hẳn** sang một con số khác, hoặc đứng yên.

Bà chủ quán lật sổ đúng theo kiểu đó. Bà lấy ngón tay đè lên một con số và bảo:
*"cao nhất tới giờ là chỗ này."* Lật sang trang sau:

- Con số mới **lớn hơn** con số đang đè? Nhấc tay, đè sang con số mới.
- Không lớn hơn? Để yên ngón tay, lật tiếp.

Hết sổ, con số dưới ngón tay chính là kỷ lục. Viết ra Python thì ngón tay ấy là
một cái tên:

```python
cao_nhat = 0

for tien in chi_tieu:
    if tien > cao_nhat:
        cao_nhat = tien
```

Ba tầng thụt lề vẫn y như khi đếm: một cái tên sinh ra trước vòng, một câu hỏi
trong thân vòng, một dòng làm việc trong nhánh đúng. Nhưng hai chỗ đã đổi ruột:

**Câu hỏi so con số mới với chính cái tên đang giữ.** `tien > cao_nhat` — không
phải so với một ngưỡng cố định như `200000`, mà so với thứ `cao_nhat` đang giữ
*ở lượt này*. Nghĩa là câu hỏi tự đổi theo từng lượt: ở ngày thứ tư nó hỏi
"260000 có lớn hơn 120000 không", ở ngày thứ sáu nó hỏi "210000 có lớn hơn
260000 không".

**Dòng trong nhánh đúng là thay thế, không phải cộng thêm.** `cao_nhat = tien`
dán lại cái tên `cao_nhat` lên con số của ngày này, và buông hẳn con số cũ. Viết
`cao_nhat += tien` ở đây là hỏng: nó cộng hai kỷ lục vào nhau thành một con số
chẳng của ngày nào.

Tên riêng của cái tên này là **biến kỷ lục**.
::::

::::example{#ngon-tay-doi-cho-may-lan}
Byte cho in ra quyết định của từng ngày:

```python title=readonly
chi_tieu = [45000, 120000, 30000, 260000, 85000, 210000]
cao_nhat = 0

for tien in chi_tieu:
    if tien > cao_nhat:
        cao_nhat = tien
        print(f"{tien} — kỷ lục mới")
    else:
        print(f"{tien} — không bằng kỷ lục {cao_nhat}")

print(f"Ngày tiêu nhiều nhất hết {cao_nhat} đồng")
```

Màn hình hiện ra:

```text title=readonly
45000 — kỷ lục mới
120000 — kỷ lục mới
30000 — không bằng kỷ lục 120000
260000 — kỷ lục mới
85000 — không bằng kỷ lục 260000
210000 — không bằng kỷ lục 260000
Ngày tiêu nhiều nhất hết 260000 đồng
```

Sáu ngày, mà ngón tay chỉ nhấc lên **ba** lần: ngày 1, ngày 2, ngày 4. Ba lượt
còn lại vòng lặp vẫn chạy đủ, vẫn hỏi đủ, chỉ là câu trả lời sai nên không có gì
xảy ra.

Hai lượt cuối đáng nhìn kỹ. `85000` và `210000` đều là những con số to — `210000`
còn lớn hơn bốn trong sáu ngày của sổ. Nhưng câu hỏi ở đây không phải *"có to
không"*, mà là *"có lớn hơn thứ đang giữ không"*. `210000` thua `260000`, nên nó
bị bỏ lại y như `30000`.

Và ngày đầu tiên: `45000 > 0` đúng, nên lượt một luôn nhấc ngón tay. Con số `0`
bạn đặt trước vòng chỉ sống được đúng một lượt.
::::

::::predict{#moc-khoi-hanh-dat-sai commitOnce}
Byte nghĩ: *"đặt mốc thật cao cho chắc"* — và viết `cao_nhat = 1000000`. **Trước
khi bấm chạy**, bạn đoán màn hình in ra con số nào?

```python title=readonly
chi_tieu = [45000, 120000, 30000, 260000, 85000, 210000]
cao_nhat = 1000000

for tien in chi_tieu:
    if tien > cao_nhat:
        cao_nhat = tien

print(cao_nhat)
```

:::opt{correct}
1000000
:::

:::opt
260000
::why
Gần đúng ở chỗ bạn nhận ra `260000` là con số lớn nhất thật sự trong sổ, và đó
đúng là thứ đoạn code này đang đi tìm. Cả khuôn ba tầng bạn đọc cũng chính xác.

Chỗ lệch nằm ở lượt đầu tiên. Câu hỏi là `tien > cao_nhat`, mà `cao_nhat` đang
giữ `1000000`. Ngày cao nhất trong sổ là `260000` — vẫn thua xa. Không ngày nào
trả lời đúng, nên dòng `cao_nhat = tien` **không chạy lần nào**, và cái tên vẫn
giữ nguyên con số bạn đặt lúc đầu.
::
:::

:::opt
45000
::why
Gần đúng ở chỗ bạn nghĩ lượt đầu tiên là lượt đặc biệt — rằng ngày đầu tiên bao
giờ cũng chiếm chỗ, rồi những ngày sau mới tranh với nó. Đó chính là điều xảy ra
khi mốc khởi hành là `0`, và là cách khuôn này thường được viết.

Chỗ lệch: máy không có luật riêng nào cho lượt đầu. Nó hỏi cùng một câu ở cả sáu
lượt, kể cả lượt một. `45000 > 1000000` là sai, nên ngày đầu bị bỏ qua y hệt
những ngày sau.
::
:::

:::opt
Máy dừng lại và báo lỗi, vì `1000000` không phải một ngày có trong sổ
::why
Gần đúng ở chỗ bạn đối chiếu con số `1000000` với sổ và thấy nó không khớp: một
kỷ lục chi tiêu phải là số tiền của **một ngày có thật**, mà trong sổ không ngày
nào tiêu chừng đó. Phép đối chiếu ấy chính xác, và nó chính là cách bắt lỗi này.

Chỗ lệch: máy không biết `cao_nhat` *nên* mang ý nghĩa gì. Với nó, dòng
`cao_nhat = 1000000` là một phép gán hoàn toàn hợp lệ, và sáu câu so sánh sau đó
đều chạy trơn tru. Đây là loại sai nguy hiểm nhất: không lỗi, không dừng, chỉ có
một con số sai lặng lẽ đi ra tới màn hình.
::
:::
::::

::::explain{#chon-moc-khoi-hanh}
Bài này để lộ ra rằng mốc khởi hành của biến kỷ lục không phải chuyện gõ cho có.
Nó phải là một con số mà **mọi giá trị thật đều thắng được**.

Với tiền tiêu trong ngày, `0` là mốc đúng: không ngày nào tiêu ít hơn 0 đồng, nên
ngày đầu tiên chắc chắn nhấc được ngón tay. Còn `1000000` thì quá cao, và bạn vừa
thấy hậu quả.

Có một mốc luôn an toàn, không cần đoán: lấy chính **con số đầu tiên trong sổ**.

```python
cao_nhat = chi_tieu[0]
```

Ngón tay đặt sẵn lên trang đầu, rồi vòng lặp đi so những trang còn lại. Cách này
không bao giờ trả về một con số lạ, vì mốc khởi hành cũng là một ngày có thật.

**Phần thưởng.** Đổi đúng một ký tự thì cùng khuôn ấy đi tìm ngày tiêu **ít
nhất**:

```python
thap_nhat = chi_tieu[0]

for tien in chi_tieu:
    if tien < thap_nhat:
        thap_nhat = tien
```

Dấu `>` thành `<`, thế thôi. Nhưng để ý mốc khởi hành: ở đây `0` là mốc **sai**
hẳn — không ngày nào tiêu ít hơn 0 đồng, nên ngón tay sẽ không bao giờ nhấc lên
và câu trả lời sẽ luôn là `0`. Mốc phải chọn theo hướng bạn đang đi tìm, và
`chi_tieu[0]` đúng cho cả hai hướng.
::::

::::code{#tim-ngay-cao-nhat}
Sổ chi tiêu sáu ngày đã nằm sẵn, và biến kỷ lục đã sinh ra trước vòng với mốc
`0`. Trong thân vòng còn hai chỗ trống: chỗ đặt **câu hỏi**, và chỗ **thay ngón
tay sang con số mới**.

```python title=starter
chi_tieu = [45000, 120000, 30000, 260000, 85000, 210000]
cao_nhat = 0

for tien in chi_tieu:
    if ___:
        ___

print(f"Ngày tiêu nhiều nhất hết {cao_nhat} đồng")
```

```python title=solution
chi_tieu = [45000, 120000, 30000, 260000, 85000, 210000]
cao_nhat = 0

for tien in chi_tieu:
    if tien > cao_nhat:
        cao_nhat = tien

print(f"Ngày tiêu nhiều nhất hết {cao_nhat} đồng")
```

```python title=test
# Kỷ lục phải là một con số CÓ THẬT trong sổ. Cộng dồn ra 750000 (không có
# trong sổ); quên thay thế thì `cao_nhat` vẫn là mốc 0 (cũng không có trong sổ).
assert cao_nhat in chi_tieu, f"cao_nhat đang là {cao_nhat} — không ngày nào tiêu đúng chừng đó"
# Giữ con số cuối cùng thì ra 210000, giữ con số đầu tiên thì ra 45000.
assert cao_nhat == 260000, f"cao_nhat đang là {cao_nhat}, phải là 260000"
# Sổ chi tiêu chỉ để đọc — vòng lặp không được sửa nó.
assert chi_tieu == [45000, 120000, 30000, 260000, 85000, 210000], "sáu ngày trong sổ phải còn nguyên như lúc chép vào — đi tìm kỷ lục là đọc sổ, không phải sửa sổ"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất so số tiền của ngày đang xem với con số mà `cao_nhat` đang giữ — cả hai cái tên đều xuất hiện trong câu hỏi đó. Chỗ trống thứ hai chỉ chạy khi câu hỏi trả lời đúng, và việc của nó là nhấc ngón tay sang chỗ mới.
- kind: strategy
  body: Ngày này chỉ đáng thay kỷ lục khi nó lớn hơn kỷ lục đang giữ. Dòng bên dưới không cộng gì cả — nó dán lại cái tên `cao_nhat` lên đúng con số của ngày này, và buông con số cũ.
- kind: one-line
  body: "`if tien > cao_nhat:` ở dòng trên, `cao_nhat = tien` ở dòng dưới."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Ngày tiêu nhiều nhất hết 260000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ngón tay nhấc ba lần, dừng lại đúng chỗ. 260 nghìn, ngày thứ tư.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sổ cả tháng có 30 ngày. Để trả lời *"ngày tiêu nhiều nhất hết bao nhiêu"*, bạn
buộc phải xem đủ 30 ngày — bỏ sót một ngày là có thể bỏ sót đúng ngày kỷ lục.
Không cách nào biết trước.

Bây giờ đổi câu hỏi:

> Cả tháng có ngày nào tiêu quá 500 nghìn không?

Câu này không cần một kỷ lục, cũng không cần một con số. Nó chỉ cần *có* hoặc
*không*. Và nó khác câu trên ở một điểm rất lớn: nếu ngày thứ tư tiêu 620 nghìn,
thì 26 ngày còn lại có tiêu bao nhiêu cũng **không đổi được câu trả lời nữa**.

Bà chủ quán tới đó là gấp sổ. Còn vòng lặp của bạn thì không: nó vẫn lật đủ 26
trang còn lại, vẫn hỏi đủ 26 lần một câu đã có đáp án.

Vậy làm sao bảo nó dừng lại giữa chừng? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
