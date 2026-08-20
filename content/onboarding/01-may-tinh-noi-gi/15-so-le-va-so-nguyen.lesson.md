---
id: onboarding.may-tinh-noi-gi.so-le-va-so-nguyen
title: Khi con số có phần lẻ
summary: Có một kiểu số thứ hai, mang phần lẻ. Và phép chia luôn cho ra nó — kể cả khi chia hết.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [core.float, core.division]
requires: [core.type-of-value, core.type-fn]
concepts: [core.kieu-gia-tri, core.so-thap-phan]
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
Bài trước bạn hỏi: chia hết thì máy dán nhãn gì? Câu trả lời hơi bất ngờ.
::::

::::explain{#cau-hoi-con-treo}
Bài trước bạn học được rằng mọi giá trị đều mang sẵn một cái nhãn, và bạn hỏi
được cái nhãn đó bằng `type`. Hai nhãn đã gặp:

- `str` — câu chữ, viết giữa hai dấu nháy.
- `int` — số nguyên, viết trần không nháy.

Rồi câu hỏi cuối bài để lại một chỗ hở. Quán bán được `45000` đồng, hai người
chia đôi. `45000` chia `2` được `22500` — chẵn chằn chặn, không dư một đồng nào.

Kết quả nguyên vẹn như thế thì máy dán nhãn gì cho nó?

Câu trả lời đáng để bạn đoán trước, vì nó là chỗ mà người mới học hay đọc lướt
qua rồi vài tuần sau ngồi tìm mãi không ra.
::::

::::predict{#chia-het-thi-ra-gi commitOnce}
Byte sắp chạy đúng phép chia đó. **Trước khi bấm chạy**, bạn đoán màn hình hiện
ra con số nào?

```python
tien_ca_ngay = 45000
print(tien_ca_ngay / 2)
```

:::opt{correct}
22500.0
:::

:::opt
22500
::why
Gần đúng — và về mặt số học thì bạn đúng hoàn toàn: chia đôi bốn mươi lăm nghìn
được hai mươi hai nghìn năm trăm, không dư đồng nào. Byte không cãi được chỗ đó.

Chỗ lệch nằm ở cái đuôi `.0` mà máy viết thêm. Máy không nhìn kết quả rồi mới
chọn cách viết. Nó biết trước: hễ bạn dùng dấu `/` thì thứ đi ra là **một con số
có phần lẻ**, và phần lẻ ấy tình cờ bằng không thì nó vẫn viết ra cho bạn thấy.

Vì sao lại thế thì phần dưới nói kỹ. Bạn vừa đoán trúng con số nhưng trượt cái
đuôi — mà cái đuôi ấy lại chính là cả bài học hôm nay.
::
:::

:::opt
22500,0
::why
Gần đúng ở chỗ bạn nhớ ra phần lẻ có mặt trong kết quả. Và cách viết `22500,0`
đúng theo lối Việt Nam: ở chợ, ở hoá đơn, ở sách vở, ta ngăn phần lẻ bằng dấu
**phẩy**.

Chỗ lệch chỉ là quy ước. Python dùng dấu **chấm** để ngăn phần lẻ, theo lối Anh —
Mỹ. Dấu phẩy trong Python đã được dành cho việc khác, nên nó không nhận dấu phẩy
làm phần lẻ.

Đây là chỗ đáng ghi lại ngay bây giờ: bạn gõ `1,5` với ý là một cân rưỡi thì máy
hiểu ra một thứ khác hẳn. Muốn một cân rưỡi thì gõ `1.5`.
::
:::
::::

::::example{#hoi-thang-cai-nhan}
Byte chạy, và hỏi luôn cái nhãn:

```python title=readonly
tien_ca_ngay = 45000
print(tien_ca_ngay / 2)
print(type(tien_ca_ngay / 2))
```

Máy trả lời:

```text
22500.0
<class 'float'>
```

Cái nhãn thứ ba của bạn đây: **`float`** (đọc là "phờ-lốt"). Nó là nhãn của **số
có phần lẻ** — `22500.0`, `1.5`, `0.25`, `-3.75` đều mang nhãn này.

Tên `float` tiếng Anh nghĩa gốc là "trôi", đặt theo chỗ đứng của dấu chấm: nó
trôi được, khi thì đứng sát cuối như `22500.0`, khi thì lên gần đầu như `0.25`.
Bạn chưa cần dùng tới ý đó, chỉ cần một chỗ để móc cái tên vào cho dễ nhớ.

Số có phần lẻ đến tay bạn bằng hai đường:

- Bạn tự viết ra, với dấu chấm: `can_thit = 1.5`.
- Hoặc nó sinh ra từ một phép chia — như dòng vừa rồi.
::::

::::explain{#cai-can-o-cho}
Vì sao chia hết rồi mà máy vẫn viết `.0`?

Hãy nghĩ tới cái cân điện tử ở sạp thịt ngoài chợ. Bà bán hàng đặt lên đúng hai
cân thịt bò, không thừa không thiếu một gam. Màn hình cân hiện:

```text
2.00 kg
```

Nó không hiện `2`. Cái cân này là cân **có phần lẻ** — nó sinh ra để cân được một
cân rưỡi, một cân hai lạng, nên nó luôn có chỗ cho phần lẻ, kể cả khi phần lẻ
bằng không. Cái đuôi `.00` không nói "còn dư một tí"; nó nói **"đây là loại cân
đo được cả phần lẻ"**.

Dấu `/` trong Python là cái cân đó. Đây là luật, và nó đúng cho mọi phép chia
bạn viết từ giờ:

> Dấu `/` luôn cho ra một giá trị mang nhãn `float`, kể cả khi phép chia không dư
> gì cả. Đây là điều quan trọng cần nhớ.

Đặt cạnh những phép tính bạn đã biết thì thấy `/` đứng riêng một mình:

| Phép tính | Kết quả | Nhãn |
|---|---|---|
| `45000 + 5000` | `50000` | `int` |
| `45000 - 5000` | `40000` | `int` |
| `45000 * 2` | `90000` | `int` |
| `45000 / 2` | `22500.0` | `float` |

Ba phép trên: vào là số nguyên, ra là số nguyên. Riêng phép chia thì đổi nhãn,
vì chia là phép duy nhất trong bốn phép có thể làm ra phần lẻ — `45000 / 7` thì
lẻ thật, lẻ dài dằng dặc. Máy chọn nhãn theo **phép tính**, không phải theo kết
quả tình cờ ra chẵn hay lẻ.

Chuyện này có hậu quả thật, không phải chi tiết vụn: một cái máy in hoá đơn mà
tính tiền bằng `/` sẽ in ra "45000.0 đồng". Người đọc thấy lạ mắt ngay. Bạn sẽ
học cách dọn cái đuôi đó đi, nhưng trước hết phải biết nó từ đâu ra.
::::

::::code{#chia-tien-bon-nguoi}
Bốn người bạn đi ăn phở, hết tất cả 180000 đồng, chia đều mỗi người một phần.

Hai cái tên đã có sẵn. Hãy điền chỗ trống để máy nói ra **số tiền mỗi người phải
trả**.

```python title=starter
tong_tien = 180000
so_nguoi = 4
print(___)
```

```python title=solution
tong_tien = 180000
so_nguoi = 4
print(tong_tien / so_nguoi)
```

```python title=test
# Bài này chấm bằng OUTPUT: người học chưa biết hàm nên chưa assert được gì.
# Khối này chỉ khẳng định chương trình chạy được tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm giữa hai dấu ngoặc của `print`. Lần này ở đó cần một **phép tính**, không phải một cái tên đứng một mình — câu hỏi là "mỗi người bao nhiêu", chứ không phải "cả bàn bao nhiêu".
- kind: strategy
  body: Chia đều nghĩa là lấy tổng tiền chia cho số người. Hai cái tên giữ hai con số đó đã nằm sẵn ở hai dòng trên, và dấu chia trong Python là dấu gạch chéo `/`.
- kind: one-line
  body: "Viết `tong_tien / so_nguoi` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: 45000.0
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đúng 45000 một người. Cái đuôi chấm-không kia là vì bạn đã bảo mình chia.
::::

::::reflect{#nghi-lai}
Bạn đang có ba cái nhãn trong tay: `str` cho câu chữ, `int` cho số nguyên,
`float` cho số có phần lẻ. Và bạn biết cách hỏi máy nhãn nào là nhãn nào.

Giờ nhìn một dòng trông vô hại:

```python
gia_cu = "45000"
print(gia_cu + 5000)
```

Hãy nhìn kỹ dòng đầu: `"45000"` có **dấu nháy**. Theo bài 8, thứ nằm giữa hai dấu
nháy là chữ — máy đọc nguyên văn, không tính toán. Còn `5000` ở dòng dưới thì
viết trần, là một con số thật.

Bạn đã gặp dấu `+` hai lần, và hai lần nó làm hai việc khác nhau:

- Bài 9: `+` giữa hai con số là **cộng** — `2 + 3` ra `5`.
- Bài 10: `+` giữa hai câu chữ là **ghép nối** — `"Phở" + " bò"` ra `"Phở bò"`.

Vậy `+` đứng giữa **một câu chữ và một con số** thì máy chọn đường nào? Nó ghép
thành `"450005000"`, hay nó cộng ra `50000`?

Đừng trả lời vội. Bài sau bạn chạy đúng dòng này. Byte chỉ mách trước một điều:
máy còn một lựa chọn thứ ba mà bạn chưa nghĩ tới.
::::

::::checkpoint{mastery=0.8}
::::
