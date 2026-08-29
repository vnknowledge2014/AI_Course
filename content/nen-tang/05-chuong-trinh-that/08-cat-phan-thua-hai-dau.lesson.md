---
id: nen-tang.chuong-trinh-that.cat-phan-thua-hai-dau
title: Cắt phần thừa ở hai đầu
summary: "Với `.strip()`, dấu ngắt dòng cũng là khoảng trắng — nên cái kéo bạn đã có từ T1.1 xén được luôn cái đuôi mà mỗi dòng file mang theo, mà không đụng vào phần giữa."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.strip-newline]
requires: [io.readlines, core.newline-char, core.with-open, core.file-write, core.string-strip, core.string-method, core.string-literal, core.len, core.list, core.list-append, core.list-index, core.variable, core.assignment, core.output, ctrl.for-each, ctrl.comparison]
concepts: [core.chuoi, core.khoang-trang, core.dong-van-ban]
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
Cái kéo xén hai đầu ấy, bạn có từ lâu rồi. Chỉ là chưa biết nó ăn cả thứ này.
::::

::::explain{#thu-dinh-o-duoi-moi-dong}
Bài trước để lại hai chỗ khó chịu, và cả hai đều chỉ về cùng một hướng: in một
dòng ra thì có một dòng trống thừa, còn `dong == "cà phê,25000"` thì cho `False`
dù nhìn giống hệt.

Hỏi thẳng máy xem dòng ấy dài bao nhiêu:

```python title=readonly
with open("so.txt", "w") as f:
    f.write("cà phê,25000\n")

with open("so.txt", "r") as f:
    cac_dong = f.readlines()

print(len(cac_dong[0]))
print(len("cà phê,25000"))
```

Máy in ra `13` rồi `12`. Dòng lấy từ file dài hơn đúng một ký tự.

Một ký tự — con số ấy đủ để gọi tên thủ phạm, vì bài 6 vừa dặn: chỗ xuống dòng
là **một** ký tự, viết là `\n`. Bạn tự tay ghi nó vào cuối mỗi khoản. Khi
`.readlines()` cắt cuốn sổ ra, nó cắt **sau** dấu ấy, nên dấu ấy đi theo dòng
chứ không bị bỏ lại.

Thế là mọi chuyện khớp: `print` in ra dòng chữ, gặp `\n` nên xuống dòng, rồi
bản thân `print` xuống dòng thêm lần nữa — thành một dòng trống. Và `==` thì so
từng ký tự, mà một bên có thêm một ký tự ở đuôi, nên nó nói `False` hoàn toàn
thành thật.

Giờ tới phần dễ chịu. Bạn **không** cần công cụ mới. Ở T1.1 bài 25, khi ô nhập
trả về `"  cà phê  "` thừa dấu cách hai đầu, bạn đã có một cái kéo xén hai đầu
mà không đụng phần giữa: `.strip()`.

Điều bài này thêm vào chỉ là một sự thật về cái kéo ấy:

> Với `.strip()`, dấu ngắt dòng `\n` cũng được tính là **khoảng trắng**.

Nó không chỉ ăn dấu cách. Nó ăn cả dấu ngắt dòng, và ăn ở **cả hai đầu**. Còn
mọi luật cũ của `.strip()` thì giữ nguyên từng chữ: nó ăn từ ngoài vào và dừng
lại ngay khi chạm ký tự đầu tiên không phải khoảng trắng, nên phần giữa không hề
suy suyển; và nó **trả về một chuỗi mới**, chuỗi cũ vẫn còn nguyên.
::::

::::example{#xen-hai-dau-tren-dong-that}
Byte gõ sổ hơi vội, nên dòng thứ hai lỡ dính thêm một dấu cách ở **đầu**. Cứ để
nguyên như thế mà xén:

```python title=readonly
with open("so.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write(" bún bò,40000\n")

with open("so.txt", "r") as f:
    cac_dong = f.readlines()

print(len(cac_dong[0]), len(cac_dong[0].strip()))
print(len(cac_dong[1]), len(cac_dong[1].strip()))
print(cac_dong[1].strip() == "bún bò,40000")
print(cac_dong[0])
print(cac_dong[0].strip())
```

Màn hình:

```text title=readonly
13 12
14 12
True
cà phê,25000

cà phê,25000
```

Bốn chỗ đáng dừng lại nhìn:

- **Dòng đầu rụng một ký tự, dòng thứ hai rụng hai.** Dòng đầu chỉ thừa `\n` ở
  đuôi. Dòng thứ hai thừa `\n` ở đuôi **và** một dấu cách ở đầu, nên cái kéo ăn
  ở cả hai phía. Đây đúng là chỗ hai chữ "hai đầu" có việc để làm.
- **`.strip()` xoá cái đuôi, không xoá cái tên.** Sau khi xén, dòng thứ hai so
  bằng `==` với `"bún bò,40000"` cho `True` — thứ máy giữ lại đúng bằng thứ bạn
  gõ vào.
- **Phần giữa còn nguyên.** Cả hai dòng sau khi xén vẫn dài 12 ký tự, và dấu
  cách giữa `cà` với `phê`, giữa `bún` với `bò` vẫn ở nguyên chỗ. Nếu cái kéo ăn
  luôn dấu cách bên trong thì hai dòng ấy đã ngắn đi và tên khoản đã dính chữ.
- **Hai dòng in cuối khác nhau đúng một dòng trống.** In `cac_dong[0]` thì có
  dòng trống theo sau; in bản đã xén thì không.
::::

::::predict{#doan-ba-con-so commitOnce}
Byte lấy dòng cuối của sổ ra, xén nó, rồi hỏi máy ba câu.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
dong = "bánh mì,15000\n"
sach = dong.strip()

print(len(dong), len(sach))
print(sach == "bánh mì,15000")
```

:::opt{correct}
`14 13` rồi `True`
:::

:::opt
`14 14` rồi `False`
::why
Gần đúng ở chỗ bạn nhớ chính xác điều T1.1 bài 25 đã nói: `.strip()` là cái kéo
xén **khoảng trắng**. Bài ấy minh hoạ bằng dấu cách và chỉ bằng dấu cách, nên
hiểu "khoảng trắng nghĩa là dấu cách" là hiểu đúng theo đúng những gì đã được
cho xem.

Chỗ lệch là ở chỗ chữ "khoảng trắng" rộng hơn một chút so với những ví dụ hồi
đó. Dấu ngắt dòng cũng là một ký tự trắng — nó chiếm chỗ mà không vẽ ra nét mực
nào — nên `.strip()` xén luôn cả nó. Đó chính là điều mới của bài này, và cũng
là lý do dòng lấy từ file rụng đi đúng một ký tự.
::
:::

:::opt
`14 12` rồi `False`
::why
Gần đúng ở chỗ bạn đang cẩn thận đúng hướng: nếu cái kéo ăn khoảng trắng, thì
dấu cách giữa `bánh` và `mì` cũng là khoảng trắng, và lo nó bị ăn mất là một nỗi
lo hợp lý.

Chỗ lệch nằm ở phạm vi cái kéo với tới. `.strip()` ăn từ ngoài vào và **dừng lại
ngay khi chạm ký tự đầu tiên không phải khoảng trắng** — ở đây là chữ `b` ngoài
cùng bên trái. Dấu cách giữa `bánh` và `mì` nằm sau chốt chặn ấy nên nó không
với tới. Chuỗi rụng đúng một ký tự `\n` ở đuôi, còn `==` thì cho `True`.
::
:::

:::opt
`15 13` rồi `True`
::why
Gần đúng ở chỗ bạn đếm rất kỹ và đếm đúng những gì nhìn thấy trên màn hình: trong
mã nguồn, chỗ ngắt dòng được gõ bằng hai phím, một dấu gạch chéo ngược và một
chữ `n`.

Chỗ lệch là ở chỗ hai phím ấy chỉ là **cách viết ra**, không phải thứ nằm trong
chuỗi. Bài 6 vừa chốt đúng chuyện này: `\n` là **một** ký tự, không phải hai.
Nên chuỗi trước khi xén dài 14, và sau khi xén còn 13 — vế thứ hai bạn đoán
trúng, vế thứ nhất lệch đi một.
::
:::
::::

::::code{#lam-sach-ca-cuon-so}
Cuốn sổ bốn khoản của Byte, và dòng thứ hai vẫn dính dấu cách gõ vội ở đầu. Việc
của bạn: đi qua từng dòng, xén sạch phần thừa ở hai đầu, cất vào một list mới.

Bốn dòng `f.write` ở đầu chỉ để dựng lại cuốn sổ cho bài chạy được một mình.

```python title=starter
with open("so.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write(" bún bò,40000\n")
    f.write("bánh mì,15000\n")
    f.write("vá lốp,100000\n")

with open("so.txt", "r") as f:
    cac_dong = f.readlines()

dong_sach = []
for dong in cac_dong:
    dong_sach.append(dong.___())

for dong in dong_sach:
    print(dong)
```

```python title=solution
with open("so.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write(" bún bò,40000\n")
    f.write("bánh mì,15000\n")
    f.write("vá lốp,100000\n")

with open("so.txt", "r") as f:
    cac_dong = f.readlines()

dong_sach = []
for dong in cac_dong:
    dong_sach.append(dong.strip())

for dong in dong_sach:
    print(dong)
```

```python title=test
# Dòng thứ hai của sổ cố tình thừa dấu cách ở ĐẦU và dấu ngắt dòng ở ĐUÔI.
# Nó là lý do bài này chấm được hai chữ "hai đầu": một cái kéo chỉ xén đuôi vẫn
# để lại " bún bò,40000" và trượt ngay câu kiểm đầu tiên.
assert dong_sach == [
    "cà phê,25000",
    "bún bò,40000",
    "bánh mì,15000",
    "vá lốp,100000",
], f"bốn dòng sau khi xén phải sạch cả hai đầu; dòng thứ hai ghi là ' bún bò,40000' nên phải rụng cả dấu cách đầu lẫn dấu ngắt cuối, đang có {dong_sach}"
assert len(dong_sach[0]) == 12, f"dòng 'cà phê,25000' sau khi xén còn đúng 12 ký tự — dấu cách giữa 'cà' và 'phê' là nội dung thật, không được xén mất; đang là {len(dong_sach[0])}"
assert cac_dong[0] != dong_sach[0], "cac_dong phải còn nguyên phần thừa của nó: cái kéo đưa ra một chuỗi mới chứ không sửa chuỗi cũ, nên phần tử đầu của hai list này không thể bằng nhau"
```

:::hints
- kind: attention
  body: Chỗ trống nằm sau `dong.`, tức là chỗ gọi tên một phương thức của chuỗi. Bạn đã dùng đúng phương thức này một lần rồi, ở T1.1 bài 25, khi ô nhập trả về một cái tên dính dấu cách hai đầu.
- kind: strategy
  body: Việc cần làm giống hệt lần ấy — xén phần thừa ở hai đầu, giữ nguyên phần giữa. Điều duy nhất đổi là loại rác cần xén: lần này ngoài dấu cách còn có dấu ngắt dòng, mà cái kéo ấy tính cả hai là khoảng trắng. Nhớ cặp ngoặc rỗng phía sau vì đây là một phương thức được gọi.
- kind: one-line
  body: "Viết `strip` vào chỗ trống, để dòng ấy thành `dong_sach.append(dong.strip())`."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  expect: "bún bò,40000"
- tier: output
  expect: "vá lốp,100000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn dòng sạch trơn. Giờ mỗi dòng đúng bằng thứ bạn đã gõ vào sổ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mỗi dòng giờ là một chuỗi sạch: `"cà phê,25000"`. Không thừa, không thiếu, so
bằng `==` là khớp.

Nhưng cuối tháng Byte không hỏi "sổ có mấy dòng". Byte hỏi tiêu hết bao nhiêu
tiền. Muốn cộng, bạn cần riêng phần **sau dấu phẩy** — `25000` — tách khỏi tên
khoản.

Ở T1.1 bạn có cắt được một khúc chuỗi theo chỗ đứng. Nhưng nhìn hai dòng này thì
biết ngay là không xong: trong `"cà phê,25000"` dấu phẩy đứng ở một chỗ, còn
trong `"bánh mì,15000"` nó đứng ở chỗ khác, vì tên khoản dài ngắn không đều.

Cắt ở đúng chỗ dấu phẩy — dù nó đứng đâu — bằng gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
