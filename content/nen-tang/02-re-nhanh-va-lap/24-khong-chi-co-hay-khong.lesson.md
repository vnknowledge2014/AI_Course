---
id: nen-tang.re-nhanh-va-lap.khong-chi-co-hay-khong
title: Không chỉ có hay không
summary: Thay vì ghi True vào lá cờ, ghi thẳng con số vừa tìm được — cái tên trả lời được cả câu hỏi "ngày nào".
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [core.result-variable]
requires: [core.flag-variable, ctrl.for-each, ctrl.break, core.reassign, core.augmented-assign]
concepts: [core.bien, ctrl.tim-kiem, ctrl.lap]
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
Lá cờ gật đầu được thôi. Còn ngày nào, thì mình phải ghi lại đúng con số ấy.
::::

::::explain{#tu-gat-dau-sang-chi-tay}
Bài trước để lại đúng một câu hỏi, và đó là câu bà chủ hỏi ngay khi đọc dòng
chữ máy in ra: **ngày nào?**

Quay lại bãi biển Sầm Sơn hôm có sóng dữ. Lá cờ đỏ vẫn treo trên cột — cả bãi
biết là hôm nay có chuyện. Nhưng trong chòi cứu hộ còn một cuốn sổ trực, và
trên đó người gác đã ghi:

> 6 giờ 10 — cột số 3.

Lá cờ và dòng sổ được viết ra cùng một lúc, bởi cùng một người, vì cùng một
chuyện. Khác nhau ở chỗ **thứ được giữ lại**. Lá cờ giữ một câu trả lời có–không.
Dòng sổ giữ chính con số mà người gác đang cầm trong tay lúc đó.

Trong code cũng vậy, và nhịp ba của bài trước không đổi một chữ nào:

- **Trước vòng**: sinh ra một cái tên.
- **Trong vòng**: gặp chuyện thì ghi vào cái tên ấy.
- **Sau vòng**: đọc nó ra.

Chỗ duy nhất đổi là **thứ ghi vào**. Thay vì ghi `True`, bạn ghi con số. Cái
tên đó có tên gọi riêng: **biến kết quả** — nó giữ kết quả của việc tìm kiếm,
chứ không chỉ giữ tin là đã tìm thấy.

Còn một chi tiết nhỏ phải chuẩn bị trước. Khi bạn duyệt sổ bằng
`for tien in chi_tieu:`, cái tên `tien` giữ **số tiền** của lượt này, và chỉ thế
thôi. Không ai nói cho bạn biết đó là ngày thứ mấy.

Muốn có con số ngày trong tay thì tự đếm lấy, bằng đúng phép cộng dồn viết gọn
của bài *Viết gọn phép cộng dồn*: một cái tên đặt bằng 0 trước vòng, rồi `+= 1`
ở đầu mỗi lượt.

```python title=readonly
chi_tieu = [80, 120, 95]
ngay = 0

for tien in chi_tieu:
    ngay += 1
    print(f"Ngày {ngay} tiêu {tien} nghìn")
```

```text title=readonly
Ngày 1 tiêu 80 nghìn
Ngày 2 tiêu 120 nghìn
Ngày 3 tiêu 95 nghìn
```

Vòng lặp đưa `tien` vào tay bạn; `ngay` là thứ bạn tự dựng lấy. Hai dòng ấy
không phải phần mới của hôm nay — chúng chỉ là `+= 1` đặt vào chỗ nhỏ nhất mà nó
dùng được. Có chúng rồi thì lúc `if` chạy đúng, trong tay bạn có sẵn **cả** số
tiền lẫn số ngày, và bạn được chọn ghi cái nào.
::::

::::example{#ghi-so-thay-vi-ghi-true}
Sổ chi tiêu bảy ngày, tính bằng nghìn đồng. Ngày 4 tiêu 240 — ngày đầu tiên
vượt 200.

Bài trước bạn viết thế này, và nó trả lời được câu hỏi "có hay không":

```python title=readonly
chi_tieu = [80, 120, 95, 240, 60, 110, 210]
da_vuot = False

for tien in chi_tieu:
    if tien > 200:
        da_vuot = True
        break

print(da_vuot)
```

```text title=readonly
True
```

Bây giờ đổi để nó trả lời "ngày nào":

```python title=readonly
chi_tieu = [80, 120, 95, 240, 60, 110, 210]
ngay = 0
ngay_tim_duoc = 0

for tien in chi_tieu:
    ngay += 1
    if tien > 200:
        ngay_tim_duoc = ngay
        break

print(f"Ngày đầu tiên vượt ngưỡng: ngày {ngay_tim_duoc}")
```

```text title=readonly
Ngày đầu tiên vượt ngưỡng: ngày 4
```

Đặt hai đoạn cạnh nhau, chỗ khác nằm ở đúng một dòng trong thân vòng:

- `da_vuot = True` → `ngay_tim_duoc = ngay`.

Hai dòng còn lại là bộ đếm bạn vừa dựng ở trên: `ngay = 0` trước vòng và
`ngay += 1` ở đầu thân. Chúng có mặt để lúc `if` chạy đúng, trong tay bạn đang
có sẵn con số ngày để ghi.

Và `break` vẫn ở nguyên chỗ cũ, vẫn làm đúng việc cũ: câu hỏi là **ngày đầu
tiên**, nên gặp một ngày là đủ biết, không cần dò nốt.

Nếu câu hỏi đổi thành "ngày đầu tiên vượt ngưỡng ấy tiêu bao nhiêu", thì cùng
một hình dạng, chỉ đổi thứ ghi vào:

```python title=readonly
chi_tieu = [80, 120, 95, 240, 60, 110, 210]
tien_tim_duoc = 0

for tien in chi_tieu:
    if tien > 200:
        tien_tim_duoc = tien
        break

print(f"Tiêu {tien_tim_duoc} nghìn")
```

```text title=readonly
Tiêu 240 nghìn
```

Để ý là bộ đếm biến mất khỏi đoạn này. Lần này không cần tới nó — thứ phải ghi
là `tien`, mà `tien` thì vòng lặp đã đưa sẵn vào tay bạn mỗi lượt.

Ghi `ngay` thì bạn giữ được **vị trí**; ghi `tien` thì bạn giữ được **giá trị**.
Cái tên trung thành với đúng thứ bạn ghi vào nó, không tự đoán thêm.
::::

::::predict{#doan-con-so-in-ra commitOnce}
Byte gõ lại đoạn tìm ngày, nhưng lần này quên mất một dòng. Sổ tuần này có ba
ngày vượt 200: ngày 2 tiêu 240, ngày 4 tiêu 260, ngày 7 tiêu 210.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
chi_tieu = [80, 240, 95, 260, 60, 110, 210]
ngay = 0
ngay_tim_duoc = 0

for tien in chi_tieu:
    ngay += 1
    if tien > 200:
        ngay_tim_duoc = ngay

print(ngay_tim_duoc)
```

:::opt{correct}
7
:::

:::opt
2
::why
Gần đúng ở chỗ bạn đọc trúng ý định của người viết — câu hỏi là *ngày đầu
tiên*, và ngày 2 đúng là ngày đầu tiên vượt ngưỡng. Ở lượt thứ hai, `ngay_tim_duoc`
thật sự đang giữ số 2.

Chỗ lệch nằm ở dòng bị quên: không có `break`, vòng lặp không dừng lại ở đó. Nó
chạy tiếp tới ngày 4, thấy 260 vượt ngưỡng, và ghi đè 4 lên số 2. Rồi tới ngày
7, ghi đè 7 lên số 4. Cái tên chỉ giữ được lần ghi **cuối cùng**, và ở đây lần
cuối là ngày 7.
::
:::

:::opt
240
::why
Bạn đọc dòng ghi theo đúng nghĩa của nó — *giữ lại thứ vừa làm nên chuyện* — và
ở ngày 2 thì thứ làm nên chuyện quả thật là số tiền 240. Suy luận ấy chính là
luận điểm của bài, chỉ đang gắn nhầm vào một dòng khác.

Chỗ lệch nằm ở vế phải của dòng ghi. Dòng đó viết `ngay_tim_duoc = ngay`, nên
thứ được ghi vào là số **thứ tự ngày**, không phải số tiền. Muốn giữ số tiền thì
vế phải phải là `tien` — như đoạn `tien_tim_duoc` ở trên.
::
:::

:::opt
0
::why
Gần đúng ở chỗ bạn nhìn ra `ngay_tim_duoc` được đặt bằng 0 trước vòng, rồi hỏi
tiếp một câu rất đáng hỏi: vòng lặp chạy xong thì cái tên ấy còn giữ được gì.

Chỗ lệch: thân vòng lặp không phải một căn phòng riêng có cửa đóng. Dòng
`ngay_tim_duoc = ngay` dán lại đúng cái tên đã sinh ra ở trên, chứ không tạo ra
một cái tên mới chỉ sống trong vòng. Vòng chạy xong, cái tên vẫn ở đó, mang giá
trị lần ghi gần nhất. Nó chỉ còn là 0 khi thân `if` không lần nào chạy.
::
:::
::::

::::explain{#break-quyet-dinh-dau-tien-hay-cuoi-cung}
Cái bẫy vừa rồi cho bạn một mẹo đáng nhớ hơn là một lỗi cần tránh:

- **Có `break` thì bạn giữ được lần gặp đầu tiên.** Ghi xong là rời vòng ngay,
  không lượt nào sau đó ghi đè lên được nữa.
- **Không có `break` thì bạn giữ được lần gặp cuối cùng.** Mỗi lần gặp lại ghi
  đè, nên thứ còn lại sau vòng là lần gặp muộn nhất.

Cả hai đều có lúc dùng. "Ngày đầu tiên tiêu quá 200" cần `break`; "ngày gần nhất
tiêu quá 200" thì bỏ `break` đi là xong. Một từ khoá quyết định câu trả lời, nên
trước khi viết, hãy đọc lại câu hỏi xem nó hỏi đầu hay cuối.

Hai điều nữa gói lại cho gọn:

- **Biến kết quả nói được nhiều hơn lá cờ.** Nó vừa cho biết chuyện đã xảy ra,
  vừa cho biết xảy ra ở đâu. Lá cờ vẫn có chỗ của nó — khi bạn thật sự chỉ cần
  có–không thì `True/False` đọc lên rõ nghĩa hơn một con số.
- **Đặt tên theo thứ nó giữ.** `ngay_tim_duoc`, `tien_cao_nhat`, `ban_dat_truoc`
  — đọc lên là biết trong đó có gì. Tên kiểu `ket_qua` hay `x` thì mười dòng sau
  bạn phải dò ngược lên mới nhớ ra nó đang giữ ngày hay giữ tiền.
::::

::::code{#ghi-lai-ngay-tim-duoc}
Bà chủ đưa ra **hai** cuốn sổ — tuần này và tuần trước — rồi hỏi cùng một câu
cho cả hai. Không phải *có hay không* nữa, mà là: **ngày nào?**

Bộ đếm đã dựng sẵn ở cả hai đoạn, `break` đã nằm đúng chỗ. Còn thiếu đúng dòng
ghi lại ngày vừa tìm được — một dòng cho mỗi cuốn sổ.

```python title=starter
tuan_nay = [80, 120, 260, 60, 110, 130, 70]
tuan_truoc = [95, 70, 110, 130, 240, 90, 60]

ngay = 0
ngay_tuan_nay = 0
for tien in tuan_nay:
    ngay += 1
    if tien > 200:
        ___
        break

ngay = 0
ngay_tuan_truoc = 0
for tien in tuan_truoc:
    ngay += 1
    if tien > 200:
        ___
        break

print(f"Tuần này: ngày {ngay_tuan_nay}")
print(f"Tuần trước: ngày {ngay_tuan_truoc}")
```

```python title=solution
tuan_nay = [80, 120, 260, 60, 110, 130, 70]
tuan_truoc = [95, 70, 110, 130, 240, 90, 60]

ngay = 0
ngay_tuan_nay = 0
for tien in tuan_nay:
    ngay += 1
    if tien > 200:
        ngay_tuan_nay = ngay
        break

ngay = 0
ngay_tuan_truoc = 0
for tien in tuan_truoc:
    ngay += 1
    if tien > 200:
        ngay_tuan_truoc = ngay
        break

print(f"Tuần này: ngày {ngay_tuan_nay}")
print(f"Tuần trước: ngày {ngay_tuan_truoc}")
```

```python title=test
# Tuần này vượt ngưỡng ở ngày 3, tuần trước ở ngày 5 — hai con số khác nhau.
# Chép thẳng một con số vào chỗ trống thì đúng được một cuốn sổ và sai cuốn kia.
# Ghi nhầm `tien` thì nó giữ 260 và 240; quên hẳn dòng ghi thì cả hai vẫn là 0.
assert ngay_tuan_nay == 3, "tuần này tiêu dè hai ngày đầu, ngày 3 mới là ngày đầu tiên quá 200 nghìn"
assert ngay_tuan_truoc == 5, "tuần trước mãi tới ngày 5 mới có ngày quá 200 nghìn — cùng một câu hỏi, hai cuốn sổ ra hai ngày khác nhau"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm trong nhánh đúng của `if`, ngay trên `break`. Tới được dòng đó nghĩa là ngày đang xét vượt ngưỡng — và ngay lúc ấy, cái tên nào đang giữ số thứ tự của ngày này?
- kind: strategy
  body: Mỗi chỗ trống dán một cái tên kết quả lên giá trị mà bộ đếm đang giữ. Vế trái là cái tên sinh ra ngay phía trên vòng ấy; vế phải là cái tên vừa được cộng thêm 1 ở đầu lượt. Hai vòng dùng hai tên kết quả khác nhau, nhưng chung một bộ đếm.
- kind: one-line
  body: Chỗ trống thứ nhất viết `ngay_tuan_nay = ngay`, chỗ thứ hai viết `ngay_tuan_truoc = ngay` — cả hai thụt vào đúng bằng dòng `break` ngay dưới nó.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Tuần này: ngày 3
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ngày 3 và ngày 5. Hai chỗ trống bạn điền vào có cùng một hình dạng, mà hai cuốn
sổ ra hai câu trả lời khác nhau — vì bạn ghi cái tên đang cầm con số, chứ không
chép con số.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại hai dòng bạn viết **trước** mỗi vòng lặp: `ngay_tuan_nay = 0` và
`ngay_tuan_truoc = 0`. Byte đặt số 0 vào đó mà chưa nói vì sao, và bạn cũng chưa
cần dùng tới nó — vì mọi cuốn sổ hôm nay đều có ít nhất một ngày vượt ngưỡng,
nên dòng ghi luôn được chạy.

Bây giờ đưa máy cuốn sổ thứ ba, một tuần tiêu dè:
`[80, 120, 95, 60, 110, 130, 70]`. Không ngày nào quá 200. Thân `if` không lần
nào chạy, không ai ghi gì vào biến kết quả cả, và nó vẫn giữ nguyên thứ bạn đặt
lúc đầu.

Máy in ra:

```text title=readonly
Tuần trước nữa: ngày 0
```

Bà chủ đọc dòng đó và đi tìm ngày 0 trong cuốn lịch. Không có ngày 0 trong tháng
nào cả.

Vậy trước vòng, biến kết quả nên được đặt bằng gì — và sau vòng, làm sao bạn
biết được rằng con số trong đó là một ngày thật, chứ không phải cái bạn đặt sẵn
từ đầu?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
