---
id: nen-tang.chuong-trinh-that.mot-chuong-trinh-nhieu-lenh
title: Một chương trình, nhiều lệnh
summary: "Ô `argv[1]` không bắt buộc phải đựng tên file. Cho nó đựng tên VIỆC, thì một chương trình làm được ba việc — chỗ đọc sổ chỉ viết một lần."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [core.subcommand]
requires: [core.sys-argv, core.with-open, core.file-append, io.readlines, core.strip-newline, core.string-split, core.string-strip, io.text-is-str, core.int-cast, core.function-def, core.function-return, core.function-parameter, core.docstring, core.tuple, core.for-unpack, core.list-append, core.list-comprehension, core.comprehension-filter, core.len, core.fstring, ctrl.if, ctrl.elif, ctrl.else, ctrl.for-each]
concepts: [core.lenh-con, core.doi-so, core.chi-so]
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
Cái ô ngay sau tên chương trình là của bạn. Bạn đã cho nó đựng tên file. Nó
đựng được thứ khác đấy.
::::

::::explain{#ba-file-cho-ba-viec}
Bài trước gỡ được tên cuốn sổ ra khỏi code: giờ bạn gõ

```text title=readonly
python so.py so-thang-8.txt
```

và chương trình đi đọc đúng cuốn sổ ấy rồi in tổng. Người dùng đổi sổ mà không
phải mở file code ra sửa dòng nào.

Nhưng chương trình mới biết làm đúng **một** việc: in tổng. Lan còn muốn thêm
một khoản vừa tiêu, và thỉnh thoảng xoá một khoản ghi nhầm. Cách gần nhất trong
tầm tay là viết thêm hai file nữa, đúng như bài trước vừa hình dung:

```text title=readonly
xem.py    — mở sổ, cộng tiền, in tổng
them.py   — mở sổ, ghi thêm một dòng
xoa.py    — mở sổ, bỏ một dòng, ghi lại
```

Ba file, và cả ba đều bắt đầu bằng cùng một đoạn: mở file, `.readlines()`,
`.strip()`, `.split(",")`, `int()`. Đoạn ấy được chép ba lần. Ngày nào bạn đổi
dấu phân cách từ dấu phẩy sang dấu chấm phẩy, bạn phải nhớ sửa cả ba chỗ — và
chỗ bạn quên sẽ không kêu lên tiếng nào, nó chỉ đọc sai.

Trước khi đi tìm công cụ mới, hãy nhìn lại thứ bạn đang gõ. Dòng lệnh trao cho
chương trình một danh sách chữ, và bài trước đã mở nó ra:

```text title=readonly
python so.py so-thang-8.txt

  argv[0] = "so.py"            ← máy tự điền, là tên chương trình
  argv[1] = "so-thang-8.txt"   ← chữ đầu tiên bạn gõ sau tên chương trình
```

`argv[0]` là tên chính chương trình, máy tự điền. Còn `argv[1]` là **ô của
bạn** — bạn muốn đặt gì vào đó cũng được. Bài trước bạn đặt tên file vào đó, vì
lúc ấy tên file là thứ duy nhất cần đưa từ ngoài vào.

Bây giờ có thứ cần đưa vào trước cả tên file: **việc phải làm**.

```text title=readonly
python so.py xem  so-thang-8.txt
python so.py them so-thang-8.txt "trà sữa" 45000
python so.py xoa  so-thang-8.txt "gửi xe"
```

Chữ đứng ở `argv[1]` bây giờ là tên một việc — người ta gọi nó là **lệnh con**.
Phần `argv` còn lại là dữ liệu cho việc đó, và mỗi việc cần một lượng dữ liệu
khác nhau: `xem` cần một tên file, `them` cần thêm tên khoản và số tiền, `xoa`
cần thêm tên khoản phải bỏ.

Mở dòng lệnh dài nhất trong ba dòng ấy ra xem từng ô đựng gì:

```text title=readonly
python so.py them so-thang-8.txt "trà sữa" 45000

  argv[0] = "so.py"            ← máy tự điền
  argv[1] = "them"             ← tên việc
  argv[2] = "so-thang-8.txt"   ← cuốn sổ phải mở
  argv[3] = "trà sữa"          ← tên khoản mới
  argv[4] = "45000"            ← số tiền, và nó vẫn là chữ
```

Bên trong chương trình, chọn việc là một câu `if` / `elif` / `else` mà bạn đã
viết từ Realm 0 — chỉ khác ở chỗ thứ đem ra so là chữ vừa lấy từ dòng lệnh.

Một chương trình, ba việc, và đoạn đọc sổ nằm đúng một chỗ.
::::

::::example{#chon-viec-bang-argv}
Trên máy thật, dòng lệnh là thứ điền vào `sys.argv`. Trang học này không có
dòng lệnh, nên chương trình tự đặt vào `sys.argv` đúng những chữ mà dòng lệnh
sẽ trao — đúng cách bài trước đã làm. Coi như Lan vừa gõ
`python so.py xem so-thang-8.txt`.

```python title=readonly
import sys

sys.argv = ["so.py", "xem", "so-thang-8.txt"]

viec = sys.argv[1]
ten_file = sys.argv[2]

print(f"việc: {viec}")
print(f"file: {ten_file}")

if viec == "xem":
    print("→ mở sổ ra đọc")
elif viec == "them":
    print("→ ghi thêm một khoản")
elif viec == "xoa":
    print("→ bỏ một khoản đi")
else:
    print(f"→ không biết việc {viec}")
```

Máy in ra:

```text title=readonly
việc: xem
file: so-thang-8.txt
→ mở sổ ra đọc
```

Ba chỗ đáng dừng lại nhìn:

- **Tên file đã dịch sang ô bên cạnh.** Ở bài trước nó nằm ở `argv[1]`; giờ
  `argv[1]` dành cho tên việc, nên tên file lùi xuống `argv[2]`. Vị trí trong
  `argv` không có ý nghĩa tự thân — nó có nghĩa vì bạn với người dùng thoả
  thuận với nhau là ô nào đựng gì.
- **Nhánh `else` không phải phần thừa.** Lan gõ `python so.py xme` là
  chuyện sẽ xảy ra. Không có `else` thì chương trình chạy hết mà không làm gì,
  không nói gì, và người gõ tưởng nó đã làm xong.
- **`viec` là một chuỗi.** Nó đến từ dòng lệnh, mà dòng lệnh chỉ trao chữ — y
  hệt `input()` ở Realm 0 và y hệt mọi thứ đọc lên từ file. Nên đem nó so bằng
  `==` với những chuỗi trong dấu nháy.
::::

::::predict{#doan-dat-nham-o commitOnce}
Byte quen tay gõ tên file trước, tên việc sau:

```text title=readonly
python so.py so-thang-8.txt xem
```

Chương trình vẫn là chương trình ở trên, không sửa một chữ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
import sys

sys.argv = ["so.py", "so-thang-8.txt", "xem"]

viec = sys.argv[1]

if viec == "xem":
    print("→ mở sổ ra đọc")
elif viec == "them":
    print("→ ghi thêm một khoản")
else:
    print(f"→ không biết việc {viec}")
```

:::opt{correct}
`→ không biết việc so-thang-8.txt`
:::

:::opt
`→ mở sổ ra đọc`
::why
Gần đúng ở chỗ bạn nhìn đúng thứ có mặt: chữ `xem` nằm trong `sys.argv` thật,
Byte đã gõ nó ra và dòng lệnh đã trao nó cho chương trình. Nó không biến đi
đâu cả.

Chỗ lệch là ở chuyện chương trình **nhìn vào đâu**. Dòng `viec = sys.argv[1]`
chỉ tay vào đúng một ô, ô thứ hai của danh sách, và nó lấy thứ nằm ở đó chứ
không đi tìm chữ `xem` trong cả danh sách. Lần này ô thứ hai đựng
`"so-thang-8.txt"`, còn `"xem"` nằm ở ô kế bên, `sys.argv[2]` — không ai đọc
tới. Đó cũng chính là lý do thứ tự các chữ trên dòng lệnh là một thoả thuận
phải giữ.
::
:::

:::opt
`→ không biết việc so.py`
::why
Gần đúng ở chỗ bạn đang đếm rất cẩn thận từ đầu danh sách, và đếm từ 0 là thói
quen đúng — Realm 0 dạy đúng như vậy.

Chỗ lệch nằm ở ô số 0. Nó không phải chữ đầu tiên bạn gõ; nó là **tên chính
chương trình**, do máy tự điền vào trước khi chương trình chạy dòng nào. Nên
chữ đầu tiên bạn gõ sau tên chương trình rơi vào ô số 1, và ở lần chạy này ô
số 1 đựng `"so-thang-8.txt"`.
::
:::

:::opt
`→ mở sổ ra đọc` rồi `→ không biết việc so-thang-8.txt`
::why
Gần đúng ở chỗ bạn để ý rằng có hai điều đáng nói ở đây — chữ `xem` có mặt, mà
`argv[1]` lại đựng tên file. Cả hai quan sát ấy đều đúng.

Chỗ lệch là một chuỗi `if` / `elif` / `else` chỉ chạy **một** nhánh. Máy thử
từng điều kiện từ trên xuống, gặp điều kiện đầu tiên đúng thì chạy nhánh đó rồi
bỏ qua hết phần còn lại; không có điều kiện nào đúng thì nó chạy `else`, cũng
đúng một nhánh. Ở đây `viec` là `"so-thang-8.txt"`, không khớp `"xem"`, không
khớp `"them"`, nên chỉ `else` chạy.
::
:::
::::

::::explain{#mot-cho-doc-so}
Đoán xong rồi thì lắp phần còn lại: đoạn đọc sổ.

Vì cả ba việc đều phải mở cuốn sổ ra, đoạn ấy được gói vào **một** hàm và cả ba
nhánh cùng gọi nó:

```python title=readonly
def doc_so(ten_file):
    """Đọc file sổ, đưa ra danh sách các khoản dạng (tên, tiền)."""
    khoan = []
    with open(ten_file, "r") as f:
        for dong in f.readlines():
            manh = dong.strip().split(",")
            khoan.append((manh[0], int(manh[1])))
    return khoan
```

Không có gì mới trong hàm này — nó là đúng những dòng bạn đã viết ở các bài
trước, gói lại dưới một cái tên. Chỗ mới của bài hôm nay chỉ là câu `if` chọn
việc phía trên nó.

Đó cũng là điều đáng nhớ nhất: `them.py` và `xoa.py` không cần tồn tại. Cái
khiến chúng tưởng như cần thiết là mỗi chương trình chỉ làm được một việc, và
`argv[1]` vừa gỡ bỏ đúng điều đó.
::::

::::code{#ba-lenh-con-mot-file}
Chương trình dưới đây làm ba việc trên cùng cuốn sổ tháng 8 của Lan, và nó được
gọi bốn lần liên tiếp — mỗi lần đặt vào `sys.argv` một dòng lệnh khác:

```text title=readonly
python so.py xem  so-thang-8.txt
python so.py them so-thang-8.txt "trà sữa" 45000
python so.py xoa  so-thang-8.txt "gửi xe"
python so.py xem  so-thang-8.txt
```

Bốn chỗ trống, tất cả nằm trong hàm `chay`, và tất cả cùng hỏi một câu: **ô nào
của `sys.argv` đựng thứ này**. Nhánh `xoa` đã điền sẵn để bạn có một chỗ đối
chiếu.

```python title=starter
import sys

with open("so-thang-8.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("gửi xe,10000\n")


def doc_so(ten_file):
    """Đọc file sổ, đưa ra danh sách các khoản dạng (tên, tiền)."""
    khoan = []
    with open(ten_file, "r") as f:
        for dong in f.readlines():
            manh = dong.strip().split(",")
            khoan.append((manh[0], int(manh[1])))
    return khoan


def chay():
    """Nhìn vào sys.argv để biết phải làm việc gì, rồi làm đúng việc đó."""
    viec = ___
    ten_file = ___

    if viec == "xem":
        khoan = doc_so(ten_file)
        tong = sum([tien for ten, tien in khoan])
        return f"xem: {len(khoan)} khoản, tổng {tong} đồng"

    elif viec == "them":
        ten_moi = ___
        tien_moi = ___
        with open(ten_file, "a") as f:
            f.write(f"{ten_moi},{tien_moi}\n")
        return f"them: đã ghi {ten_moi} {tien_moi} đồng vào {ten_file}"

    elif viec == "xoa":
        ten_bo = sys.argv[3]
        con_lai = [k for k in doc_so(ten_file) if k[0] != ten_bo]
        with open(ten_file, "w") as f:
            for ten, tien in con_lai:
                f.write(f"{ten},{tien}\n")
        return f"xoa: đã bỏ {ten_bo} khỏi {ten_file}"

    else:
        return f"không biết việc {viec}"


nhat_ky = []

sys.argv = ["so.py", "xem", "so-thang-8.txt"]
nhat_ky.append(chay())

sys.argv = ["so.py", "them", "so-thang-8.txt", "trà sữa", "45000"]
nhat_ky.append(chay())

sys.argv = ["so.py", "xoa", "so-thang-8.txt", "gửi xe"]
nhat_ky.append(chay())

sys.argv = ["so.py", "xem", "so-thang-8.txt"]
nhat_ky.append(chay())

for dong in nhat_ky:
    print(dong)
```

```python title=solution
import sys

with open("so-thang-8.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("gửi xe,10000\n")


def doc_so(ten_file):
    """Đọc file sổ, đưa ra danh sách các khoản dạng (tên, tiền)."""
    khoan = []
    with open(ten_file, "r") as f:
        for dong in f.readlines():
            manh = dong.strip().split(",")
            khoan.append((manh[0], int(manh[1])))
    return khoan


def chay():
    """Nhìn vào sys.argv để biết phải làm việc gì, rồi làm đúng việc đó."""
    viec = sys.argv[1]
    ten_file = sys.argv[2]

    if viec == "xem":
        khoan = doc_so(ten_file)
        tong = sum([tien for ten, tien in khoan])
        return f"xem: {len(khoan)} khoản, tổng {tong} đồng"

    elif viec == "them":
        ten_moi = sys.argv[3]
        tien_moi = int(sys.argv[4])
        with open(ten_file, "a") as f:
            f.write(f"{ten_moi},{tien_moi}\n")
        return f"them: đã ghi {ten_moi} {tien_moi} đồng vào {ten_file}"

    elif viec == "xoa":
        ten_bo = sys.argv[3]
        con_lai = [k for k in doc_so(ten_file) if k[0] != ten_bo]
        with open(ten_file, "w") as f:
            for ten, tien in con_lai:
                f.write(f"{ten},{tien}\n")
        return f"xoa: đã bỏ {ten_bo} khỏi {ten_file}"

    else:
        return f"không biết việc {viec}"


nhat_ky = []

sys.argv = ["so.py", "xem", "so-thang-8.txt"]
nhat_ky.append(chay())

sys.argv = ["so.py", "them", "so-thang-8.txt", "trà sữa", "45000"]
nhat_ky.append(chay())

sys.argv = ["so.py", "xoa", "so-thang-8.txt", "gửi xe"]
nhat_ky.append(chay())

sys.argv = ["so.py", "xem", "so-thang-8.txt"]
nhat_ky.append(chay())

for dong in nhat_ky:
    print(dong)
```

```python title=test
# Bốn chỗ trống, và mỗi chỗ có ít nhất một câu dưới đây vỡ nếu điền sai ô.
#
# Chỗ 1 (`viec`): trỏ nhầm sang ô đựng tên file thì không nhánh việc nào khớp,
# cả bốn dòng nhật ký thành "không biết việc so-thang-8.txt".
assert nhat_ky[0] == "xem: 3 khoản, tổng 75000 đồng", "lượt xem đầu tiên chạy trên cuốn sổ ba khoản cà phê 25000, bún bò 40000, gửi xe 10000 — ba khoản ấy cộng lại là 75000"
# Chỗ 2 (`ten_file`): trỏ sang ô số 3 thì lượt `xem` đầu tiên chỉ có ba ô,
# chương trình dừng vì đòi một ô không tồn tại.
assert nhat_ky[3] == "xem: 3 khoản, tổng 110000 đồng", "lượt xem cuối chạy sau khi đã thêm trà sữa 45000 và bỏ gửi xe 10000, nên sổ còn cà phê, bún bò, trà sữa — ba khoản, cộng lại 110000"
# Chỗ 3 và 4 (`ten_moi`, `tien_moi`): lấy nhầm ô thì tên khoản mới hoặc số
# tiền mới sai, và câu dưới đây nói ra ngay ô nào đã lấy nhầm.
assert nhat_ky[1] == "them: đã ghi trà sữa 45000 đồng vào so-thang-8.txt", "dòng lệnh của lượt thêm là `so.py them so-thang-8.txt \"trà sữa\" 45000`, nên tên khoản mới phải là trà sữa và số tiền phải là 45000"
assert nhat_ky[2] == "xoa: đã bỏ gửi xe khỏi so-thang-8.txt", "lượt xoá nhận tên khoản phải bỏ ở ô số 3, và ở dòng lệnh ấy ô số 3 đựng gửi xe"
# Cuốn sổ trên đĩa cũng phải đúng, không riêng câu chữ in ra màn hình.
assert doc_so("so-thang-8.txt") == [("cà phê", 25000), ("bún bò", 40000), ("trà sữa", 45000)], "sau ba lệnh con, file so-thang-8.txt phải còn đúng ba dòng: cà phê 25000, bún bò 40000 và trà sữa 45000 — gửi xe đã bị lệnh xoa bỏ đi"
```

:::hints
- kind: attention
  body: Cả bốn chỗ trống đều nằm bên phải một dấu bằng, và thứ điền vào đều lấy từ cùng một danh sách. Đọc lại bốn dòng lệnh ở đầu bước này rồi đếm các chữ trên từng dòng, bắt đầu từ 0 ở tên chương trình.
- kind: strategy
  body: 'Với `so.py them so-thang-8.txt "trà sữa" 45000`: ô 0 là tên chương trình, ô 1 là tên việc, ô 2 là tên file, ô 3 là tên khoản mới, ô 4 là số tiền. Nhánh `xoa` ngay bên dưới đã viết sẵn một dòng lấy ô số 3 — chép đúng hình dạng ấy. Riêng số tiền đọc lên vẫn là chữ, mà chữ thì không cộng được với tiền trong sổ, nên nó cần đi qua một bước đổi kiểu.'
- kind: one-line
  body: 'Lần lượt bốn chỗ trống là `sys.argv[1]`, `sys.argv[2]`, `sys.argv[3]` và `int(sys.argv[4])`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: tests
  timeoutMs: 8000
- tier: output
  expect: "xem: 3 khoản, tổng 75000 đồng"
- tier: output
  expect: "them: đã ghi trà sữa 45000 đồng vào so-thang-8.txt"
- tier: output
  expect: "xem: 3 khoản, tổng 110000 đồng"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba việc, một file. Đoạn đọc sổ nằm đúng một chỗ, nên sửa nó cũng đúng một chỗ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`so.py` giờ dài ra rất nhanh: ba lệnh con, mỗi lệnh vài chục dòng, cộng thêm
hàm `doc_so` và phần đọc `sys.argv` ở đầu. Nó vẫn là một file, và nó vẫn gọn
hơn ba file chép chồng lên nhau.

Nhưng Lan đặt hàng thêm một thứ nữa: một chương trình làm **bản báo cáo** cuối
tháng, in ra từng khoản rồi in tổng. Chương trình ấy không thêm, không xoá,
không nhận lệnh con nào — nó dùng đúng một thứ trong `so.py`, là hàm `doc_so`.

Chép hàm đó sang file mới à? Thế là quay lại đúng chỗ đầu bài: một đoạn code
nằm ở hai nơi, và ngày bạn sửa một nơi thì phải nhớ sửa cả nơi kia.

Bạn đã từng mượn công cụ của người khác bằng `import`. Có cách nào mượn hàm
của **chính bạn** theo đúng lối đó không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
