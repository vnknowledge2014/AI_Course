---
id: nen-tang.chuong-trinh-that.ghi-chu-xuong-file
title: Ghi một dòng xuống đĩa
summary: "`open(ten, \"w\")` mở một file ra để ghi và `.write(...)` đặt câu chữ vào — lần đầu tiên chương trình của bạn tự tạo ra một thứ nằm ngoài chính nó."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.file-write]
requires: [core.file, core.list-of-dicts, core.list-comprehension, core.nested-index, core.list-index, core.fstring, core.string-method, core.builtin-function, core.variable, core.assignment, core.output, core.string-literal]
concepts: [core.file, core.chuoi, core.bien]
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
Cuốn sổ bạn vừa dựng đang nằm trong điện. Mình biết một cái ngăn kéo mà điện
tắt nó vẫn còn nguyên.
::::

::::explain{#so-tinh-xong-roi-tat-may}
Bạn vừa đóng lại một cuốn sổ chi tiêu tự tổng kết: mười khoản, tổng cả tháng,
tiền từng nhóm, ba khoản tốn nhất. Nó chạy đúng. Rồi bạn tắt máy, và sáng mai
mở lên thì không còn gì — trừ đúng mười khoản gõ cứng trong file code.

Chỗ đó bạn đã biết tên từ Realm 0. Bài `file-va-thu-muc` nói về hai chỗ ghi
chữ trong quán cô Bảy: cái bảng con cạnh bếp, hết ca là lau sạch; và cuốn sổ
trong ngăn kéo, khoá cửa mai mở ra vẫn nguyên từng dòng. Máy tính cũng có đúng
hai chỗ như vậy, và thứ nằm ở chỗ thứ hai gọi là một **file**.

Hồi đó bạn mới chỉ **nhìn** file — nhìn tên nó, nhìn chỗ nó đứng. Bây giờ
chương trình sẽ tự tay tạo ra một cái.

Có hai động tác, viết trên hai dòng:

```python title=readonly
f = open("so-thang-tam.txt", "w")
f.write("cà phê,25000")
```

Dòng thứ nhất, `open`, nhận vào hai thứ:

- **tên file** — đúng cái tên mà Realm 0 gọi là tên của file;
- **một chữ nói bạn định làm gì với nó** — ở đây là `"w"`, viết tắt của
  *write*, nghĩa là *tôi định ghi*.

Dòng thứ hai đặt câu chữ vào. Hình dạng của nó bạn đã quen từ T1.1: một cái
tên, dấu chấm, rồi việc muốn làm — y như `"  cà phê  ".strip()`. Ở đây cái tên
đứng trước dấu chấm là `f`, và việc muốn làm là `write`.

Còn `f` ở đâu ra? Đó là thứ `open` **đưa lại**, và ta đặt cho nó một cái tên
để còn gọi lại ở dòng dưới. Thứ ấy không phải câu chữ trong file. Nó nhớ hai
điều bạn vừa nói với `open`, và bạn hỏi lại được:

```python title=readonly
print(f.name)   # so-thang-tam.txt
print(f.mode)   # w
```

`.write` cũng đưa lại một thứ: **một tờ biên nhận**, là số ký tự nó vừa đặt
vào. Ghi `"cà phê,25000"` thì biên nhận ghi 12, vì câu ấy dài đúng 12 ký tự.
::::

::::example{#mot-dong-xuong-file}
Byte rút cuốn sổ xuống còn **ba khoản đầu tháng** cho dễ nhìn bằng mắt — nên
con số tổng ở đây là tổng của ba khoản ấy, không phải tổng cả tháng mười khoản
ở bài BOSS vừa rồi. Cộng xong thì ghi câu tổng kết xuống file.

```python title=readonly
so = [
    {"ten": "cà phê", "nhom": "ăn uống", "tien": 25000},
    {"ten": "bún bò", "nhom": "ăn uống", "tien": 40000},
    {"ten": "gửi xe", "nhom": "đi lại", "tien": 10000},
]

tong = sum([khoan["tien"] for khoan in so])
cau_tong = f"tổng,{tong}"

f = open("so-thang-tam.txt", "w")
da_ghi = f.write(cau_tong)

print(f"Câu đem ghi: {cau_tong}")
print(f"Số ký tự đã đặt vào file: {da_ghi}")
```

Máy in ra:

```text title=readonly
Câu đem ghi: tổng,75000
Số ký tự đã đặt vào file: 10
```

Bốn chỗ đáng dừng lại nhìn:

- **Phần trên là cuốn sổ cũ, không có gì mới.** Ba dict trong một list, một
  comprehension nhặt ô `"tien"`, `sum` cộng lại: 25 000 + 40 000 + 10 000 =
  75 000. Toàn thứ bạn viết suốt T1.4.
- **`open` là chỗ mới, và nó không in ra gì cả.** Nó lặng lẽ tạo ra một file
  mang tên `so-thang-tam.txt`, rồi đưa lại thứ mà ta gọi là `f`.
- **Biên nhận là 10.** Câu đem ghi là `tổng,75000`. Đếm từng ký tự: bốn chữ
  `t ổ n g`, một dấu phẩy, rồi năm chữ số `7 5 0 0 0` — 4 + 1 + 5 = 10.
- **Màn hình không hề hiện nội dung file.** `print` in ra thứ đang nằm trong
  chương trình, còn file thì nằm ngoài chương trình. Hai chỗ khác nhau, và
  cả bài này là chuyện bắc một nhịp giữa chúng.
::::

::::predict{#doan-open-dua-lai-gi commitOnce}
Byte tò mò không biết `open` đưa lại cái gì, nên in thẳng nó ra xem.

**Trước khi bấm chạy**, bạn đoán màn hình hiện gì?

```python title=readonly
f = open("nhap.txt", "w")
f.write("cà phê,25000")
print(f)
```

:::opt{correct}
Một dòng mô tả thứ vừa mở, trong đó có `name='nhap.txt'` và `mode='w'`.
:::

:::opt
Đúng câu vừa ghi: `cà phê,25000`.
::why
Gần đúng ở chỗ bạn theo dõi rất sát dòng ngay phía trên: câu ấy vừa được đưa
vào thật, không sai một chữ.

Chỗ lệch là `f` không phải nội dung file. `f` được đặt tên từ dòng `open`, tức
là từ **trước khi** có chữ nào được ghi, nên nó không thể là câu chữ ấy được.
Nó là thứ mà `.write` phải đi qua để đặt câu ấy vào.
::
:::

:::opt
Tên file: `nhap.txt`, không có gì thêm.
::why
Gần đúng ở chỗ bạn nhớ đúng một điều có thật: thứ này có nhớ tên file, và bạn
hỏi được bằng `f.name` — đúng ra `nhap.txt`.

Chỗ lệch là `f` và `f.name` không phải một. `f.name` là **một trong những điều
f nhớ**; còn `f` là cả cái thứ đang nhớ chúng. In cả cái thứ ấy ra thì Python
mô tả nó cho bạn, và trong lời mô tả có cả tên lẫn chữ `w` bạn vừa trao.
::
:::

:::opt
Một dòng trống, vì file vừa tạo ra thì chưa có gì trong đó.
::why
Gần đúng ở chỗ bạn suy luận đúng chiều: một file vừa được tạo thì đúng là chưa
có sẵn nội dung gì, và bạn nghĩ tới điều đó là một phản xạ tốt.

Chỗ lệch là `print(f)` không đi mở file ra đọc. `print` chỉ nhìn thứ đang nằm
trong chương trình và mô tả nó. Muốn xem chữ trong file thì phải bảo máy đi
lấy về, mà bạn chưa có lệnh nào làm việc đó.
::
:::
::::

::::code{#ghi-dong-dau-cua-so}
Byte muốn ghi **dòng đầu** của cuốn sổ xuống file — khoản đầu tiên, viết thành
`tên,tiền`.

Hai chỗ trống nằm ở hai động tác của bài này: một chỗ nói với `open` rằng bạn
định ghi, một chỗ đưa cho `.write` câu chữ đem ghi.

```python title=starter
so = [
    {"ten": "cà phê", "nhom": "ăn uống", "tien": 25000},
    {"ten": "bún bò", "nhom": "ăn uống", "tien": 40000},
    {"ten": "gửi xe", "nhom": "đi lại", "tien": 10000},
]

khoan_dau = so[0]
dong_dau = f"{khoan_dau['ten']},{khoan_dau['tien']}"

f = open("so-thang-tam.txt", ___)
da_ghi = f.write(___)

print(f"Dòng đem ghi: {dong_dau}")
print(f"Số ký tự đã đặt vào file: {da_ghi}")
```

```python title=solution
so = [
    {"ten": "cà phê", "nhom": "ăn uống", "tien": 25000},
    {"ten": "bún bò", "nhom": "ăn uống", "tien": 40000},
    {"ten": "gửi xe", "nhom": "đi lại", "tien": 10000},
]

khoan_dau = so[0]
dong_dau = f"{khoan_dau['ten']},{khoan_dau['tien']}"

f = open("so-thang-tam.txt", "w")
da_ghi = f.write(dong_dau)

print(f"Dòng đem ghi: {dong_dau}")
print(f"Số ký tự đã đặt vào file: {da_ghi}")
```

```python title=test
# Chỗ trống thứ nhất: chữ trao cho `open`. Câu dưới đây vỡ nếu điền chữ khác
# — thứ `open` đưa lại luôn nhớ đúng chữ bạn đã trao.
assert f.mode == "w", "chỗ trống thứ nhất là chữ nói với open rằng bạn định GHI, tức là 'w' — thứ open đưa lại đang nhớ một chữ khác"
# Chỗ trống thứ hai: câu chữ đem ghi. Dòng đầu của sổ là 'cà phê,25000', dài
# 12 ký tự; câu tổng kết 'tổng,75000' chỉ dài 10, nên đưa nhầm là vỡ ngay.
assert da_ghi == 12, "dòng đem ghi là 'cà phê,25000' — đếm cả dấu phẩy thì được 12 ký tự, nên biên nhận của .write phải là 12"
assert da_ghi == len(dong_dau), "con số .write báo lại phải bằng đúng độ dài của dong_dau; lệch nghĩa là thứ đem ghi không phải dong_dau"
# Cuốn sổ là thứ chỉ đọc trong bài này, không được sửa.
assert so[0]["tien"] == 25000, "ghi sổ xuống file là việc chỉ đọc: khoản cà phê trong sổ phải vẫn còn nguyên 25000"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm ở đối số thứ hai của `open`, ngay sau tên file — chỗ bạn nói với máy rằng lần này bạn định làm gì với file. Chỗ trống thứ hai nằm trong ngoặc của `.write`, nên thứ điền vào phải là câu chữ đem đặt vào file, và câu ấy đã được dựng sẵn ở dòng ngay trên.
- kind: strategy
  body: Chữ trao cho `open` là chữ cái đầu của *write*, viết thường và đặt trong dấu nháy như mọi dãy chữ. Còn thứ đưa cho `.write` là một cái tên đã có sẵn giá trị, đừng gõ lại câu ấy bằng tay — gõ tay thì khi cuốn sổ đổi khoản đầu, file vẫn ghi câu cũ.
- kind: one-line
  body: 'Chỗ trống thứ nhất viết `"w"`, chỗ thứ hai viết `dong_dau`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^Dòng đem ghi: cà phê,25000\nSố ký tự đã đặt vào file: 12\s*$
- tier: output
  expect: "Số ký tự đã đặt vào file: 12"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chương trình của bạn vừa tự tay tạo ra một file. Đây là lần đầu tiên đấy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại hai dòng đã làm nên cả bài này. `open` không đưa lại câu chữ nào cả.
Nó đưa lại một thứ mà `.write` phải đi qua — thứ bạn đặt tên là `f`, thứ nhớ
tên file và nhớ chữ `w`.

Thứ đó là gì, và nó tồn tại tới bao giờ?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
