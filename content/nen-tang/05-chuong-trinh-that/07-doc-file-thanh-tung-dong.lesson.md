---
id: nen-tang.chuong-trinh-that.doc-file-thanh-tung-dong
title: Đọc file thành một danh sách dòng
summary: "`.readlines()` cắt cuốn sổ tại từng chỗ ngắt dòng và đưa về một list — mỗi dòng của file thành một phần tử, để `for` đi qua từng khoản một."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.file-readlines, io.readlines]
requires: [core.file-read, core.file-write, core.newline-char, core.with-open, core.file, core.list, core.len, core.list-index, core.string-method, core.string-literal, core.variable, core.assignment, core.fstring, core.output, ctrl.for-each]
concepts: [core.file, core.dong-van-ban, core.danh-sach]
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
Sổ có ba dòng rồi. Nhưng máy vẫn đưa lại đúng một cục chữ.
::::

::::explain{#mot-cuc-chu-khong-cat-duoc}
Bài trước bạn đặt được dấu ngắt dòng vào đúng chỗ, và mở file lên bằng mắt thì
ba khoản nằm ba dòng ngay ngắn. Nhưng khi hỏi máy, câu trả lời vẫn y như cũ:

```python title=readonly
with open("so.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("bánh mì,15000\n")

with open("so.txt", "r") as f:
    ca_cuon_so = f.read()

print(len(ca_cuon_so))
```

Máy in ra `40`.

Con số ấy không sai — nó là số **ký tự** của cả cuốn sổ, kể cả ba dấu ngắt dòng
bạn vừa đặt vào. Chỉ có điều nó không phải con số bạn muốn. Bạn muốn biết sổ có
mấy **khoản**, và bạn muốn xử lý riêng từng khoản.

Thứ bạn cần đã nằm sẵn trong tay từ Realm 0: vòng `for` đi qua **từng phần tử
của một list**, mỗi lượt một phần tử. Nhưng `for` cần một list, mà `.read()`
đưa về một chuỗi. Cho một chuỗi vào `for` thì nó đi qua từng **ký tự** — và
`c`, `à`, dấu cách... không phải là những khoản chi.

Chỗ hụt nằm đúng ở đó: cuốn sổ đã có ranh giới giữa các khoản rồi, nhưng người
đọc ra ranh giới ấy là bạn, không phải máy. Cần ai đó cắt cuốn sổ tại từng chỗ
ngắt dòng **trước khi** trao nó cho bạn.

Kết nối đang mở có sẵn một cách đọc thứ hai làm đúng việc ấy:

```python title=readonly
with open("so.txt", "r") as f:
    cac_dong = f.readlines()
```

Tên nó đọc thành hai mẩu: *read* là đọc, *lines* là các dòng — **đọc thành các
dòng**. Vẫn là kết nối mở bằng `with open(..., "r")` như bài trước, vẫn là một
phương thức gọi bằng dấu chấm rồi cặp ngoặc, y hệt `.read()`. Chỉ **hình dạng
thứ nhận về** là khác:

- `.read()` đưa về **một chuỗi** — cả cuốn sổ dính liền;
- `.readlines()` đưa về **một list** — mỗi dòng của file là một phần tử.

Mà đã là list thì mọi thứ bạn học suốt T1.4 dùng lại được ngay: `len` đếm,
`for` đi qua từng phần tử, ngoặc vuông lấy phần tử theo chỗ đứng.
::::

::::example{#ba-khoan-ba-phan-tu}
Cùng một cuốn sổ ba dòng, đọc bằng cách thứ hai:

```python title=readonly
with open("so.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("bánh mì,15000\n")

with open("so.txt", "r") as f:
    cac_dong = f.readlines()

print(len(cac_dong))

for dong in cac_dong:
    print(dong)
```

Màn hình:

```text title=readonly
3
cà phê,25000

bún bò,40000

bánh mì,15000

```

Ba chỗ đáng dừng lại nhìn:

- **`len(cac_dong)` cho `3`, không cho `40`.** Cùng một hàm `len`, cùng một
  cuốn sổ, mà hai con số khác hẳn nhau — vì lần này thứ đem đếm là một list, và
  `len` của một list đếm **phần tử**. Ba khoản, ba phần tử.
- **`for dong in cac_dong` là đúng vòng `for` của R0·35.** Không có cú pháp nào
  mới ở đây. `cac_dong` là một list bình thường, nên `for` đối xử với nó y như
  đối xử với danh sách thực đơn quán phở.
- **Ngoặc vuông vẫn ăn chỗ đứng.** `cac_dong[0]` là khoản đầu sổ, `cac_dong[2]`
  là khoản cuối — đếm từ 0, đúng như R0·34.

Và có một chỗ lạ trong màn hình trên. Giữa hai khoản luôn có một dòng trống, và
sau khoản cuối cũng còn một dòng trống nữa. Bạn không viết dòng trống nào cả.
Giữ lấy chỗ lạ ấy; cuối bài ta quay lại.
::::

::::predict{#doan-may-phan-tu commitOnce}
Byte ghi ba khoản xuống sổ, mỗi khoản một dòng, và dòng cuối cùng cũng kết thúc
bằng một dấu ngắt dòng như hai dòng trên.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
with open("so.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("bánh mì,15000\n")

with open("so.txt", "r") as f:
    cac_dong = f.readlines()

print(len(cac_dong))
```

:::opt{correct}
`3`
:::

:::opt
`4`
::why
Gần đúng ở chỗ bạn theo dõi rất sát một chi tiết có thật: dòng cuối cùng **cũng**
kết thúc bằng một dấu ngắt dòng, đúng như hai dòng trên nó. Người mới học hay bỏ
qua chi tiết ấy, còn bạn thì không.

Chỗ lệch nằm ở việc dấu ngắt dòng làm gì. Nó **đóng lại** dòng đang viết dở chứ
không mở ra một dòng mới. Sau dấu ngắt cuối cùng, trong file không còn một ký tự
nào nữa — nên không có dòng thứ tư nào để mà đếm. Cuốn sổ ghi ba khoản, và
`.readlines()` đưa về đúng ba phần tử.
::
:::

:::opt
`40`
::why
Gần đúng ở chỗ bạn tính đúng con số của bài toán khác: `40` chính là số ký tự
của cả cuốn sổ ba dòng, và đó là con số `len` sẽ cho nếu ta đọc bằng `.read()`.
Bạn không đếm sai một ký tự nào.

Chỗ lệch nằm ở thứ đang được đếm. `len` không tự nó biết đếm cái gì — nó hỏi
**thứ bạn đưa vào**. Đưa vào một chuỗi thì nó đếm ký tự; đưa vào một list thì nó
đếm phần tử. `.readlines()` đưa về một list, nên lần này `len` đếm số dòng.
::
:::

:::opt
`1`
::why
Gần đúng ở chỗ bạn nhớ đúng thói quen của cách đọc cũ: từ bài 4 tới giờ, mở file
ra là nhận về **một** thứ — một chuỗi duy nhất chứa tất cả. Nghĩ rằng cách đọc
mới cũng đưa về một thứ là suy luận thẳng từ kinh nghiệm đó.

Chỗ lệch: cách đọc mới đúng là đưa về **một** thứ thật, nhưng thứ ấy là một
**list**, và `len` của một list không phải là 1 — nó là số phần tử bên trong.
Một cái rổ đựng ba quả cam vẫn là một cái rổ, mà đếm cam thì được ba.
::
:::
::::

::::code{#dem-khoan-trong-so}
Byte vừa vá lốp xe hết 100 nghìn và ghi nốt khoản thứ tư vào sổ. Giờ Byte muốn
chương trình tự nói ra sổ đang có mấy khoản, rồi đọc từng khoản lên.

Bốn dòng `f.write` ở đầu chỉ để dựng lại cuốn sổ, cho bài này chạy được một
mình — phần bạn phải điền nằm ở chỗ đọc.

```python title=starter
with open("so.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("bánh mì,15000\n")
    f.write("vá lốp,100000\n")

with open("so.txt", "r") as f:
    cac_dong = f.___()

print(f"Sổ đang có {len(cac_dong)} khoản")

for dong in cac_dong:
    print(dong)
```

```python title=solution
with open("so.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("bánh mì,15000\n")
    f.write("vá lốp,100000\n")

with open("so.txt", "r") as f:
    cac_dong = f.readlines()

print(f"Sổ đang có {len(cac_dong)} khoản")

for dong in cac_dong:
    print(dong)
```

```python title=test
# Ba câu kiểm, mỗi câu chặn một cách đọc sai khác nhau.
# `.read()` cho về một chuỗi 54 ký tự — trượt cả ba câu.
# `.readline()` cho về đúng một dòng, cũng là chuỗi — trượt cả ba câu.
assert isinstance(cac_dong, list), "cac_dong phải là một list các dòng; đọc bằng cách cho về một chuỗi thì vòng for phía dưới sẽ đi qua từng ký tự chứ không đi qua từng khoản"
assert len(cac_dong) == 4, f"cuốn sổ này ghi bốn khoản (cà phê, bún bò, bánh mì, vá lốp) nên cac_dong phải có 4 phần tử, đang có {len(cac_dong)}"
assert cac_dong[0].startswith("cà phê,25000"), "phần tử đầu của cac_dong phải là trọn dòng đầu của sổ, tức là bắt đầu bằng cà phê,25000"
assert cac_dong[3].startswith("vá lốp,100000"), "phần tử thứ tư của cac_dong phải là trọn dòng cuối của sổ, tức là bắt đầu bằng vá lốp,100000"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ngay sau `f.` — tức là chỗ gọi tên một cách đọc. Bài này có hai cách đọc, và chúng khác nhau ở hình dạng thứ đưa về. Dòng ngay dưới đòi `len(cac_dong)` phải ra số **khoản**, không phải số ký tự.
- kind: strategy
  body: Bạn cần một list, mỗi dòng của file là một phần tử. Tên của cách đọc ấy ghép từ hai mẩu tiếng Anh — *đọc* và *các dòng* — viết liền, không dấu cách, không dấu gạch. Nhớ cặp ngoặc rỗng phía sau, vì đây là một phương thức được **gọi**.
- kind: one-line
  body: "Viết `readlines` vào chỗ trống, để dòng ấy thành `cac_dong = f.readlines()`."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  expect: "Sổ đang có 4 khoản"
- tier: output
  expect: "vá lốp,100000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn khoản, bốn phần tử. Giờ vòng lặp đi qua từng khoản chứ không đi qua từng chữ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cuốn sổ đã chịu để máy cắt thành từng khoản. Nhưng cái chỗ lạ ở giữa bài vẫn
chưa được giải thích, và giờ nó lại hiện ra lần nữa: in `dong` ra thì giữa các
dòng luôn có một dòng trống thừa.

Byte thử thêm một câu nữa, và câu này mới là câu khó chịu:

```python title=readonly
dong = cac_dong[0]
print(dong == "cà phê,25000")
```

Máy trả lời `False`.

Nhìn bằng mắt thì hai bên giống hệt nhau, từng chữ một. Vậy mà máy bảo khác.
Ở T1.1 bài 25 bạn đã gặp đúng cảnh này một lần: hai chuỗi trông y hệt mà `==`
vẫn cho `False`, vì `==` so từng ký tự và nó không bỏ sót ký tự nào. Nếu máy nói
khác, thì hai bên khác thật.

Mỗi dòng lấy từ file còn dính thứ gì mà mắt bạn không nhìn ra?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
