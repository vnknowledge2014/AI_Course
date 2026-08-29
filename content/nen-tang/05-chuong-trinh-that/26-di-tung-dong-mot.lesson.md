---
id: nen-tang.chuong-trinh-that.di-tung-dong-mot
title: Đi từng dòng một
summary: "Tại điểm dừng, lệnh `n` cho máy đi đúng một dòng rồi dừng lại tiếp, `c` cho chạy nốt — nên bạn xem được giá trị đổi thế nào sau từng dòng, không phải đặt lại điểm dừng."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [dbg.step-line]
requires: [dbg.breakpoint, core.string-split, core.strip-newline, core.variable, core.assignment, core.reassign, core.accumulator, core.list, core.list-index, core.int-cast, core.string-method, core.fstring, core.output, ctrl.for-each, ctrl.loop-variable, ctrl.block-indent]
concepts: [core.diem-dung, core.tung-buoc, core.vet-chay]
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
Một điểm dừng cho bạn một khoảnh khắc. Còn khoảnh khắc kế tiếp thì xin thêm được.
::::

::::explain{#xin-them-mot-khoanh-khac}
Bài trước để lại một chỗ bí có thật: tại điểm dừng, `tong` bằng `0` và `cac_dong`
đủ 12 phần tử, cả hai đều đúng. Chỗ hỏng nằm ở những dòng **sau** điểm dừng — mà
điểm dừng thì chỉ cho bạn đúng một khoảnh khắc.

Đặt thêm điểm dừng ở từng dòng là làm được, nhưng nó kéo bạn về đúng chỗ đau của
`print`: mỗi câu hỏi một lần sửa code, một lần chạy lại.

Ở dấu nhắc `(Pdb)`, ngoài tên biến bạn còn gõ được **lệnh**. Và có một lệnh dài
đúng một chữ cái:

```text title=readonly
(Pdb) n
```

`n` bảo máy: *đi đúng MỘT dòng nữa rồi dừng lại.* Chương trình nhích lên một
bước, in ra dòng sắp chạy tiếp theo, rồi lại trao dấu nhắc cho bạn. Muốn xem
khoảnh khắc kế nữa thì gõ `n` lần nữa. Giữa mỗi lần, bạn vẫn hỏi tên biến được
như cũ.

Đi kèm nó là một lệnh nữa cũng một chữ cái:

```text title=readonly
(Pdb) c
```

`c` bảo máy: *thôi đừng nhích từng bước nữa, chạy tiếp cho tới hết* (hoặc tới
điểm dừng kế tiếp, nếu bạn có đặt thêm). Dùng khi bạn đã xem đủ.

Nói cho gọn: điểm dừng chọn **chỗ bắt đầu nhìn**, còn `n` chọn **nhìn thêm bao
xa**. Một dòng `breakpoint()` cộng với `n` gõ mấy lần tuỳ ý thay được cho một
nắm điểm dừng rải khắp hàm.
::::

::::predict{#doan-vong-lap-in-ra-gi commitOnce}
Đây là phần cộng tiền của `bao_cao()`, chép ra riêng cho ngắn, chạy trên một cuốn
sổ ba khoản.

**Trước khi bấm chạy**, bạn đoán dòng cuối in ra gì?

```python title=readonly
cac_dong = ["cà phê,25000", "bún bò,40000", "gửi xe,10000"]

tong = 0
for dong in cac_dong:
    manh = dong.split(",")
    tong = tong + int(manh[1])
    tong = 0

print(f"Tổng: {tong} đồng")
```

:::opt{correct}
`Tổng: 0 đồng`
:::

:::opt
`Tổng: 75000 đồng`
::why
Gần đúng ở chỗ bạn cộng không sai một đồng: ba khoản 25000, 40000 và 10000 gộp
lại đúng bằng 75000, và đó là con số cuốn sổ đáng ra phải cho.

Chỗ lệch nằm ở dòng cuối cùng của **thân vòng lặp**. Nó thụt vào bằng dòng cộng,
nên nó nằm trong vòng lặp và chạy sau mỗi lượt — mỗi lượt vừa cộng xong thì nó
lại trao số 0 cho `tong`. Lượt cuối cũng vậy, nên thứ còn lại lúc thoát vòng lặp
là con số 0 chứ không phải tổng.
::
:::

:::opt
`Tổng: 10000 đồng`
::why
Gần đúng ở chỗ bạn nhìn ra được cái bẫy chính: có một dòng trao lại số 0 cho
`tong` bên trong vòng lặp, nên tổng không cộng dồn nổi.

Chỗ lệch là **thứ tự hai dòng trong thân**. Nếu dòng trao số 0 đứng ở ĐẦU thân
vòng lặp thì mỗi lượt sẽ xoá trước rồi cộng sau, và thứ sót lại đúng là khoản
cuối cùng, 10000. Ở đây nó đứng ở CUỐI thân: cộng trước, xoá sau. Lượt cuối cùng
kết thúc bằng cú xoá, nên không khoản nào sống sót.
::
:::

:::opt
Chương trình dừng vì lỗi, do `tong` bị gán hai lần
::why
Gần đúng ở chỗ bạn thấy có gì đó không ổn với việc `tong` bị trao giá trị nhiều
lần trong cùng một chương trình — cảm giác ấy đúng, đây là một con bọ thật.

Chỗ lệch: gán lại một cái tên là chuyện Python cho phép và bạn đã làm suốt từ
Realm 0, `tong = tong + tien` cũng chính là gán lại. Máy không có cách nào biết
lần gán nào là ý bạn muốn. Nên nó không kêu một tiếng nào — đó đúng là lý do con
bọ này sống lâu được đến vậy.
::
:::
::::

::::explain{#di-bo-qua-vong-lap}
Giờ đem `n` ra soi đúng đoạn ấy. Đặt `breakpoint()` ngay trước vòng lặp, chạy,
rồi gõ `n` từng nhát một và hỏi `tong` sau mỗi nhát:

```text title=readonly
> /Users/lan/du-an/thu.py(5)<module>()
-> for dong in cac_dong:
(Pdb) tong
0
(Pdb) n
-> manh = dong.split(",")
(Pdb) n
-> tong = tong + int(manh[1])
(Pdb) n
-> tong = 0
(Pdb) tong
25000
(Pdb) n
-> for dong in cac_dong:
(Pdb) tong
0
(Pdb) n
-> manh = dong.split(",")
(Pdb) n
-> tong = tong + int(manh[1])
(Pdb) n
-> tong = 0
(Pdb) tong
40000
(Pdb) c
Tổng: 0 đồng
```

Đọc cái vệt ấy từ trên xuống, con bọ tự lộ ra:

- Sau dòng cộng ở lượt một, `tong` là `25000` — đúng y như mong đợi.
- Gõ `n` một nhát nữa, dòng `tong = 0` chạy, và lần hỏi kế tiếp `tong` trả về
  `0`. Số tiền vừa cộng biến mất ngay trong cùng một lượt.
- Lượt hai lặp lại nguyên vẹn: cộng lên `40000`, rồi lại về `0`.

Ba lần gõ `n` đủ để thấy điều mà chín dòng `print` không cho thấy: không phải
`tong` sai ở cuối, mà nó **bị xoá lại sau mỗi lượt**.

Cần nói thẳng một chuyện. Ở đây thân vòng lặp có ba dòng, nên dòng trao số 0
nằm ngay trước mắt bạn. Trong `bao_cao.py` thật, nó nằm ở dòng thứ ba mươi mấy
của một hàm bốn mươi dòng, kẹp giữa phần mở file và phần in báo cáo — và ở đó
thì mắt không bắt được, còn `n` thì vẫn bắt được y như trên.
::::

::::code{#ghi-lai-vet-chay}
Cùng vòng lặp hỏng ấy, nhưng cuốn sổ đã khác: ba khoản khác, ba số tiền khác. Vệt
chạy ở phần trên **không** dùng lại được — bạn phải xăng xe từng dòng trên cuốn
sổ mới này.

Bốn khoảnh khắc cần ghi lại, tất cả đều là giá trị của `tong`:

1. ngay sau khi dòng cộng chạy xong ở lượt **một**;
2. ngay sau khi dòng trao số 0 chạy xong ở lượt **một**;
3. ngay sau khi dòng cộng chạy xong ở lượt **hai**;
4. sau khi vòng lặp đã chạy hết cả ba lượt.

```python title=starter
cac_dong = ["vở ghi,15000", "trà sữa,45000", "bánh mì,20000"]

tong = 0
for dong in cac_dong:
    manh = dong.split(",")
    tong = tong + int(manh[1])
    tong = 0

sau_cong_luot_1 = ___
sau_trao_khong_luot_1 = ___
sau_cong_luot_2 = ___
tong_cuoi_cung = ___

print(f"Sau dòng cộng ở lượt 1: {sau_cong_luot_1}")
print(f"Sau dòng trao số 0 ở lượt 1: {sau_trao_khong_luot_1}")
print(f"Sau dòng cộng ở lượt 2: {sau_cong_luot_2}")
print(f"Sau khi vòng lặp xong: {tong_cuoi_cung}")
```

```python title=solution
cac_dong = ["vở ghi,15000", "trà sữa,45000", "bánh mì,20000"]

tong = 0
for dong in cac_dong:
    manh = dong.split(",")
    tong = tong + int(manh[1])
    tong = 0

sau_cong_luot_1 = 15000
sau_trao_khong_luot_1 = 0
sau_cong_luot_2 = 45000
tong_cuoi_cung = 0

print(f"Sau dòng cộng ở lượt 1: {sau_cong_luot_1}")
print(f"Sau dòng trao số 0 ở lượt 1: {sau_trao_khong_luot_1}")
print(f"Sau dòng cộng ở lượt 2: {sau_cong_luot_2}")
print(f"Sau khi vòng lặp xong: {tong_cuoi_cung}")
```

```python title=test
# Đi lại đúng vòng lặp ấy một lần nữa, lần này ghi xuống MỌI khoảnh khắc của
# `tong` — y như gõ `n` từng nhát — rồi đối chiếu với bốn con số bạn điền.
moc = []
kt = 0
for d in cac_dong:
    m = d.split(",")
    kt = kt + int(m[1])
    moc.append(kt)
    kt = 0
    moc.append(kt)

assert sau_cong_luot_1 == moc[0], "lượt một xử lý khoản vở ghi 15000, và trước lượt ấy `tong` đang là 0, nên ngay sau dòng cộng `tong` mang 15000"
assert sau_trao_khong_luot_1 == moc[1], "dòng cuối thân vòng lặp trao số 0 cho `tong`, nên ngay sau nó, ở lượt một, `tong` mang 0 chứ không còn 15000"
assert sau_cong_luot_2 == moc[2], "lượt hai bắt đầu với `tong` bằng 0 và xử lý khoản trà sữa 45000, nên ngay sau dòng cộng `tong` mang 45000 — không phải 15000 cộng 45000, vì lượt một đã bị xoá"
assert tong_cuoi_cung == tong, "lượt ba cũng kết thúc bằng dòng trao số 0, nên khi vòng lặp chạy hết cả ba lượt, `tong` mang 0"
assert moc[4] == 20000, "lượt ba xử lý khoản bánh mì 20000 trên một `tong` vừa bị xoá, nên ngay sau dòng cộng của lượt ấy `tong` mang 20000"
```

:::hints
- kind: attention
  body: Thân vòng lặp có ba dòng và chúng chạy theo đúng thứ tự viết ra, hết dòng cuối thì quay lại dòng đầu cho lượt sau. Bốn câu hỏi trỏ vào bốn chỗ khác nhau trong cái vòng ấy, nên bốn câu trả lời không nhất thiết giống nhau.
- kind: strategy
  body: Đi bộ qua lượt một trước: `tong` vào lượt mang giá trị nào, dòng cộng thêm số tiền của khoản đầu tiên vào, rồi dòng cuối thân làm gì với nó. Xong lượt một thì lượt hai bắt đầu từ đúng cái giá trị mà lượt một để lại. Câu thứ tư hỏi thứ còn sót sau lượt ba, và lượt ba cũng kết thúc bằng đúng dòng cuối ấy.
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `15000`, `0`, `45000`, `0`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^Sau dòng cộng ở lượt 1: 15000\nSau dòng trao số 0 ở lượt 1: 0\nSau dòng cộng ở lượt 2: 45000\nSau khi vòng lặp xong: 0\s*$
- tier: output
  expect: "Sau dòng cộng ở lượt 2: 45000"
- tier: output
  expect: "Sau khi vòng lặp xong: 0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhích một dòng, hỏi một câu. Con bọ không trốn được vào giữa hai dòng nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Thủ phạm đã lộ: `tong = 0` bị đặt nhầm vào trong thân vòng lặp, nên mỗi vòng nó
xoá lại. Sửa thì một nhát là xong — kéo dòng ấy ra khỏi thân vòng lặp.

Nhưng vì sao mắt bạn không thấy?

Vì `bao_cao()` dài 40 dòng, vừa mở file, vừa tách dòng, vừa cộng, vừa in.

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
