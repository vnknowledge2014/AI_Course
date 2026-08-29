---
id: nen-tang.gia-tri-bien-kieu.dau-ngan-hang-nghin
title: Dấu ngăn hàng nghìn
summary: Một dấu phẩy đặt trong phần định dạng bảo máy tách con số thành từng nhóm ba chữ số khi in ra.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [core.thousands-sep]
requires: [core.fstring, core.division]
concepts: [core.dinh-dang, core.chuoi]
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
Một triệu ba trăm sáu mươi nghìn. Bạn vừa phải đếm chữ số mới đọc ra, phải không?
::::

::::explain{#so-dai-thi-mat-phai-dem}
Bài trước đã dạy chỗ nằm sau dấu hai chấm trong cặp `{}`, và bạn dùng nó để dặn
máy in đúng hai chữ số sau dấu chấm: `f"{trung_binh:.2f}"` cho ra `45333.33`.

Dòng trung bình xong rồi. Còn dòng tổng thì vẫn thế này:

```text
Cả tháng: 1360000đ
```

Một triệu ba trăm sáu mươi nghìn. Mắt bạn không đọc ra ngay con số ấy — bạn
phải rà từ phải sang trái, gom ba chữ số một, rồi mới gọi được tên nó. Bảy chữ
số dính liền là bảy chữ số không có mốc nào để bám.

Trên hoá đơn giấy không ai in như vậy. Người ta cắt con số thành từng cụm ba
chữ số, đếm từ hàng đơn vị trở lên, rồi đặt một dấu ngăn giữa các cụm. Nhìn vào
là biết ngay có mấy triệu, mấy nghìn.

Phần sau dấu hai chấm là chỗ bạn ra lệnh cho máy về **cách viết ra**. Bài trước
bạn đã đặt vào đó `.2f`. Lần này chỉ cần một dấu phẩy:

```python
f"{tong_thang:,}"
```

Dấu phẩy ấy có tên: **dấu ngăn nhóm hàng nghìn** — tiếng Anh gọi là *thousands
separator*. Đó là từ để bạn tra cứu khi cần.
::::

::::example{#mot-dau-phay-doi-ca-dong}
Cùng một con số, in hai lần, khác nhau đúng một dấu phẩy:

```python title=readonly
tong_thang = 1360000

print(f"Cả tháng: {tong_thang}đ")
print(f"Cả tháng: {tong_thang:,}đ")
print(tong_thang + 40000)
```

Máy in ra:

```text
Cả tháng: 1360000đ
Cả tháng: 1,360,000đ
1400000
```

Hai chỗ đáng nhìn kỹ.

Thứ nhất: máy gom nhóm **từ phải sang trái**, mỗi nhóm ba chữ số. `1360000` có
bảy chữ số, nên nhóm ngoài cùng bên trái chỉ còn một chữ số — `1,360,000`, chứ
không phải `136,0000`. Cách gom này khớp với cách người ta đọc tiền: nghìn,
triệu, tỷ.

Thứ hai: dòng cuối. `tong_thang + 40000` vẫn cộng được và ra `1400000` — không
có dấu phẩy nào lọt vào. Cái tên `tong_thang` vẫn đang giữ một `int` nguyên
vẹn. Dấu phẩy trong phần định dạng chỉ tác động lên **bản chép ra bằng chữ** mà
f-string dựng lên để in, đúng luật bạn đã gặp ở bài chuỗi bất biến: định dạng
không sửa con số, nó viết ra một dòng chữ mới.
::::

::::predict{#doan-mot-dau-phay commitOnce}
Byte ghi một khoản lẻ: cà phê 25 nghìn. **Trước khi bấm chạy**, bạn đoán màn
hình hiện ra đúng dòng nào?

```python
tien = 25000
print(f"{tien:,}đ")
```

:::opt{correct}
25,000đ
:::

:::opt
25.000đ
::why
Gần đúng ở chỗ bạn viết ra đúng cách một cuốn sổ tiếng Việt ghi con số này: cụm
ba chữ số cuối tách khỏi số 25 bằng một dấu ngăn. Bạn nhóm đúng, đếm đúng, và
đặt dấu đúng chỗ.

Chỗ lệch là **dấu nào**. Dấu phẩy bạn gõ trong phần định dạng luôn cho ra dấu
phẩy khi in, vì Python theo quy ước tiếng Anh: dấu phẩy ngăn nghìn, dấu chấm để
dành cho phần lẻ. Phần giải thích ngay sau đây nói rõ khoảng cách này, và chỉ
chỗ vá nó.
::
:::

:::opt
2,5000đ
::why
Gần đúng ở chỗ bạn nhận ra máy chèn dấu vào giữa con số theo từng nhóm ba chữ
số — đó chính là việc nó làm.

Chỗ lệch là **chiều đếm**. Nếu gom ba chữ số từ trái sang thì `25000` ra
`250,00`, và một con số như vậy không đọc được theo nghìn hay triệu. Máy đếm từ
hàng đơn vị ngược lên, nên nhóm ba chữ số cuối luôn đủ ba: `25,000`.
::
:::

:::opt
25,000.00đ
::why
Gần đúng ở chỗ bạn nhớ rằng phần định dạng quyết định được số chữ số sau dấu
chấm — bài trước vừa dạy đúng chuyện đó.

Chỗ lệch: hai chữ số lẻ là việc của `.2f`, và ở đây bạn không viết `.2f`. Dấu
phẩy đứng một mình chỉ làm đúng một việc là ngăn nhóm. `tien` đang giữ `25000`,
một số nguyên, nên nó in ra không có phần lẻ nào cả.
::
:::
::::

::::explain{#dau-phay-hay-dau-cham}
Nói cho hết một chuyện mà bài này không giấu: `f"{tong:,}"` cho ra `1,360,000`,
trong khi cuốn sổ trên bàn bạn viết `1.360.000đ`.

Python dùng quy ước tiếng Anh — dấu phẩy ngăn nghìn, dấu chấm mở phần lẻ — và
trong phần định dạng không có ký hiệu nào đảo hai dấu ấy cho bạn.

Chỗ vá nằm ngoài phần định dạng: in xong ra chữ rồi mới đổi ký tự.

```python title=readonly
tong_thang = 1360000
dong = f"{tong_thang:,}"
print(dong.replace(",", ".") + "đ")
```

```text
1.360.000đ
```

`replace` không phải khái niệm mới của bài này — nó là một **phương thức chuỗi**,
cùng họ với `.lower()` và `.strip()` bạn đã dùng: gọi bằng dấu chấm sau một
chuỗi, và trả về một **chuỗi mới**, không sửa chuỗi cũ. Bạn đưa cho nó hai
mảnh: tìm cái gì, thay bằng cái gì.

Từ đây tới hết mạch, các bài vẫn để nguyên dấu phẩy. Lý do: một dòng đã dán
thêm `.replace` thì khó nhìn ra thứ đang học nằm ở đâu. Bạn biết chỗ vá là đủ,
và ngày làm sổ thật cho người Việt đọc thì lấy nó ra dùng.
::::

::::explain{#ghep-hai-menh-lenh}
Còn một chuyện nữa: dòng trung bình mỗi ngày cũng dài không kém.

`1360000 / 30` cho `45333.333333333336`. Bài trước đã cắt nó còn hai chữ số lẻ
bằng `.2f`, ra `45333.33`. Nhưng phần trước dấu chấm vẫn là năm chữ số dính
liền.

Hai mệnh lệnh ấy viết được cùng lúc, trong cùng một phần định dạng, và dấu phẩy
đứng trước:

```python
f"{trung_binh:,.2f}"
```

Đọc từ trái sang: *ngăn nhóm hàng nghìn cho tôi, và in đúng hai chữ số sau dấu
chấm.* Kết quả là `45,333.33`.

Thứ tự này không đảo được: `.2f,` là một câu máy không hiểu. Cách nhớ: dấu phẩy
nói về phần **trước** dấu chấm, nên nó cũng đứng trước.
::::

::::code{#hai-dong-bao-cao}
Báo cáo cuối tháng của Byte có hai dòng: cả tháng tiêu hết bao nhiêu, và trung
bình mỗi ngày bao nhiêu. Tổng là `1360000` đồng — một số nguyên, vì tiền thì
đếm bằng đồng. Trung bình là `1360000 / 30`, một số thực, vì nó là một phép đo
chứ không phải khoản tiền ai đó thật sự trả.

Hai chỗ trống đều nằm sau dấu hai chấm. Hai dòng cần hai câu lệnh **khác nhau**:
dòng tổng chỉ cần ngăn nhóm, dòng trung bình cần ngăn nhóm và hai chữ số lẻ.
Điền cùng một thứ vào cả hai chỗ thì có một dòng sai — đó là lý do bài chấm cả
hai dòng một lượt.

```python title=starter
tong_thang = 1360000
trung_binh_ngay = tong_thang / 30

print(f"Cả tháng: {tong_thang:___}đ")
print(f"Mỗi ngày: {trung_binh_ngay:___}đ")
```

```python title=solution
tong_thang = 1360000
trung_binh_ngay = tong_thang / 30

print(f"Cả tháng: {tong_thang:,}đ")
print(f"Mỗi ngày: {trung_binh_ngay:,.2f}đ")
```

```python title=test
# Chấm bằng TRỌN VẸN màn hình (`match: regex`), trên HAI dòng chứ không một.
# Hai dòng ấy chọn cố ý để mọi câu trả lời hụt đều lộ ra:
#   `,` điền cho cả hai chỗ  → dòng hai ra 45,333.333333333336, thừa đuôi;
#   `.2f` điền cho cả hai chỗ→ dòng một ra 1360000.00, không có dấu ngăn;
#   `,.2f` cho cả hai chỗ    → dòng một ra 1,360,000.00, thừa hai số 0;
#   `.2f,` (đảo thứ tự)      → máy nổ ValueError ngay khi định dạng.
# Người học chưa cần viết assert ở bài này; khối này ghi rõ vì sao hai dòng là
# hai, không phải một.
pass
```

:::hints
- kind: attention
  body: Cả hai chỗ trống nằm sau dấu hai chấm, tức là trong phần định dạng bài trước đã mở ra. Nhìn kỹ hai giá trị được in: một cái là số nguyên, một cái là số thực có đuôi dài.
- kind: strategy
  body: Dòng đầu cần đúng một việc — ngăn nhóm ba chữ số. Dòng sau cần hai việc cùng lúc — ngăn nhóm, và giữ đúng hai chữ số sau dấu chấm; hai mệnh lệnh ấy viết liền nhau trong cùng một phần định dạng, mệnh lệnh ngăn nhóm đứng trước.
- kind: one-line
  body: "Chỗ trống thứ nhất là `,`, chỗ trống thứ hai là `,.2f`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Cả tháng: 1,360,000đ\nMỗi ngày: 45,333\.33đ\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một phẩy ba trăm sáu mươi nghìn. Đọc một phát ra ngay, không phải đếm.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Con số đã dễ đọc. Nhưng sổ chi tiêu không có một dòng — nó có nhiều dòng nằm
liền nhau. Thử in ba khoản của hôm nay:

```python
print(f"cà phê {25000:,}đ")
print(f"bún chả {1250000:,}đ")
print(f"gửi xe {5000:,}đ")
```

```text
cà phê 25,000đ
bún chả 1,250,000đ
gửi xe 5,000đ
```

Ba con số đều dễ đọc, nhưng ba **cột tiền** thì bắt đầu ở ba chỗ khác nhau, vì
tên khoản dài ngắn không đều. Muốn cộng nhẩm cả cột, mắt bạn phải nhảy qua nhảy
lại.

Trên cuốn sổ giấy, cột được kẻ **trước**: tên khoản rộng 12 chỗ, tiền rộng 10
chỗ — đúng hai con số mà bạn đã đo bằng `len` mấy bài trước. Ô nào chữ ngắn thì
để trống phần còn lại, chứ cột không co lại theo chữ.

Vậy bảo máy giữ mỗi ô đúng một bề rộng cố định thì viết thế nào? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
