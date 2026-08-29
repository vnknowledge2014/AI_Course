---
id: nen-tang.chuong-trinh-that.import-chay-ca-file
title: import chạy cả file được mượn
summary: "`import` không chép mỗi cái tên sang — nó đọc và chạy toàn bộ file được mượn, từ dòng đầu tới dòng cuối, đúng một lần."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.import-runs-file]
requires: [core.import-from, core.own-module, core.import-module, mod.import, mod.dotted-access, core.with-open, core.file-write, core.file-readlines, io.readlines, core.newline-char, core.strip-newline, core.string-split, core.int-cast, core.tuple, core.list, core.list-append, core.len, core.function-def, core.function-call, core.function-return, core.function-parameter, core.docstring, core.variable, core.assignment, core.string-literal, core.fstring, core.output, core.print-variable]
concepts: [core.mo-dun, core.le-trai, core.dan-ten]
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
Bạn thêm đúng một dòng vào sổ sách. Máy đọc luôn cả cuốn.
::::

::::explain{#cau-do-hien-len-truoc-khi-goi-ham}
Bài trước bạn lấy riêng hai cái tên ra khỏi `so_sach.py`, rồi gõ thêm một dòng
`print("đang mở sổ...")` ở lề trái file ấy để thử xem nó có được đọc tới không.
Chạy `bao_cao.py` — chưa gọi hàm nào cả — mà câu đó đã hiện lên màn hình.

Đó không phải trục trặc. Đó là `import` đang làm đúng việc của nó, và việc ấy
lớn hơn cái tên "mượn một cái tên" nghe có vẻ gợi ra.

Khi Python gặp dòng `import so_sach`, nó làm ba chuyện, theo thứ tự:

1. đi tìm file `so_sach.py`;
2. **đọc và chạy toàn bộ file ấy**, từ dòng đầu xuống dòng cuối, y như bạn gõ
   `python so_sach.py`;
3. gom mọi cái tên vừa sinh ra trong lúc chạy, gói lại thành một hộp, rồi đặt
   hộp đó vào chương trình của bạn dưới cái tên `so_sach`.

Bước 2 là bước bị bỏ quên. Cái tên `doc_so` mà bạn muốn mượn không có sẵn ở đâu
cả — nó chỉ tồn tại **sau khi** dòng `def doc_so(ten_file):` được chạy qua. Muốn
có cái tên, phải chạy cái file. Không có đường tắt nào khác.

Vậy còn dòng `def` thì sao — nó cũng chạy à? Có, và đây là chỗ đáng phân biệt
cho rõ:

- Một dòng ở lề trái **chạy thật**, ngay lúc đó. `print(...)` thì in ra màn
  hình. `NGUONG_LON = 100000` thì cất con số vào một cái tên.
- Dòng `def doc_so(ten_file):` cũng là một dòng ở lề trái, nên nó cũng chạy
  thật. Chỉ có điều việc nó làm là **dán cái tên `doc_so` lên thân hàm** rồi
  thôi. Thân hàm nằm thụt vào trong, và phần thụt vào ấy chỉ chạy khi có người
  gọi — đúng như T1.3 đã nói ngay từ bài đầu.

Nên nhìn từ ngoài vào, `import` cho cảm giác lặng lẽ khi file được mượn chỉ toàn
`def`: mỗi dòng `def` chạy qua, dán một cái tên, không kêu tiếng nào. Thêm một
dòng `print` ở lề trái là bạn vừa cho file ấy một cái miệng.

Còn một chuyện nữa, và nó là nửa sau của luật: **đúng một lần**. Lần đầu gặp
`import so_sach`, Python chạy file rồi cất cái hộp lại. Những lần gặp sau — dù ở
file khác, dù cách đó hai chục dòng — nó thấy hộp đã có sẵn nên đưa lại hộp cũ,
không đọc lại file lần nào nữa.
::::

::::example{#hai-file-mot-lan-doc}
Vẫn `so_sach.py` của bài trước, chỉ khác hai dòng ở lề trái mà Byte vừa thêm
vào: dòng `print` bạn gõ để thử, và một hằng số `NGUONG_LON` — mức tiền mà cả
cuốn sổ coi là khoản lớn, đúng cái mốc 100 nghìn quen thuộc từ T1.4.

```python title=readonly
# ── so_sach.py ──────────────────────────────────────────────────────
print("so_sach.py: đang mở sổ...")

NGUONG_LON = 100000


def doc_so(ten_file):
    """Đọc file sổ, đưa ra danh sách các khoản dạng (tên, tiền)."""
    khoan = []
    with open(ten_file, "r") as f:
        for dong in f.readlines():
            manh = dong.strip().split(",")
            khoan.append((manh[0], int(manh[1])))
    return khoan


def tong_tien(khoan):
    """Cộng tiền của mọi khoản trong một danh sách khoản."""
    return sum([tien for ten, tien in khoan])
```

```python title=readonly
# ── bao_cao.py ──────────────────────────────────────────────────────
import so_sach

print("bao_cao.py: bắt đầu")
print("bao_cao.py: mức khoản lớn là", so_sach.NGUONG_LON, "đồng")
```

Gõ `python bao_cao.py`, máy in ra:

```text title=readonly
so_sach.py: đang mở sổ...
bao_cao.py: bắt đầu
bao_cao.py: mức khoản lớn là 100000 đồng
```

Ba chỗ đáng dừng lại nhìn:

- **Dòng của `so_sach.py` đứng TRƯỚC dòng đầu tiên của `bao_cao.py`.** Câu
  `print("bao_cao.py: bắt đầu")` là câu lệnh thứ hai của `bao_cao.py`, nhưng nó
  vẫn hiện sau. Vì câu lệnh thứ nhất — `import so_sach` — chưa xong việc: nó
  còn đang chạy cả một file khác.
- **`NGUONG_LON` đã có giá trị mà không ai gọi gì.** Trong `bao_cao.py` không có
  một lời gọi hàm nào của hộp cả. Con số ấy được cất vào cái tên lúc dòng
  `NGUONG_LON = 100000` chạy qua, tức là lúc import.
- **Không có chữ nào của `doc_so` hiện ra.** Cái tên `doc_so` đã nằm trong hộp
  rồi, nhưng thân hàm thì chưa chạy lần nào — không ai mở file, không ai tách
  dòng. Dán tên là một chuyện, gọi là chuyện khác.

Viết `from so_sach import doc_so` như bài trước cũng vậy thôi: dạng viết ấy chỉ
đổi chỗ cái tên đi đâu trong chương trình của bạn, còn bước "chạy cả file được
mượn" thì vẫn xảy ra nguyên vẹn.
::::

::::predict{#doan-hai-lan-import commitOnce}
Byte sửa `bao_cao.py`: viết hẳn hai dòng `import so_sach` cho chắc, vì hai chỗ
trong file đều cần tới sổ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
# ── so_sach.py ──────────────────────────────────────────────────────
print("so_sach.py: đang mở sổ...")

NGUONG_LON = 100000

# (hai hàm doc_so và tong_tien vẫn nằm dưới đây, y như bài trước)
```

```python title=readonly
# ── bao_cao.py ──────────────────────────────────────────────────────
import so_sach
import so_sach

print("bao_cao.py: bắt đầu")
```

:::opt{correct}
`so_sach.py: đang mở sổ...` một lần, rồi `bao_cao.py: bắt đầu`
:::

:::opt
`so_sach.py: đang mở sổ...` hai lần, rồi `bao_cao.py: bắt đầu`
::why
Gần đúng ở chỗ bạn theo dõi rất sát cái luật vừa học: `import` chạy cả file
được mượn, mà đây có hai dòng `import`, nên hai lần chạy. Suy luận ấy đi đúng
đường; nó chỉ thiếu nửa sau của luật.

Chỗ lệch: Python cất cái hộp lại sau lần chạy đầu tiên. Tới dòng `import` thứ
hai, nó nhìn vào chỗ cất, thấy hộp `so_sach` đã nằm sẵn ở đó, nên đưa lại đúng
hộp ấy và không mở file lần nào nữa. Dòng `print` ở lề trái chỉ có một cơ hội
để chạy, và cơ hội đó đã dùng xong.
::
:::

:::opt
Chỉ có `bao_cao.py: bắt đầu`, vì chưa ai dùng tới cái tên nào trong hộp
::why
Gần đúng ở chỗ bạn đang nghĩ theo một lối rất hợp lý: chưa cần thì chưa làm.
Có những công cụ trong lập trình đúng là làm việc theo lối ấy.

Chỗ lệch là `import` không thuộc số đó. Cái tên `so_sach.NGUONG_LON` hay
`so_sach.doc_so` chỉ tồn tại **sau khi** file kia đã chạy xong — trước đó không
có gì để mà lấy. Nên Python phải chạy trước, chạy ngay tại dòng `import`, rồi
mới có hộp để đưa cho bạn.
::
:::

:::opt
`bao_cao.py: bắt đầu` trước, rồi `so_sach.py: đang mở sổ...`
::why
Gần đúng ở chỗ bạn đọc đúng thứ tự các dòng theo một nghĩa nào đó: dòng `print`
của `bao_cao.py` là dòng có chữ "bắt đầu", nghe như nó phải hiện đầu tiên.

Chỗ lệch là máy đọc từ trên xuống, không đọc theo nghĩa của chữ. Dòng `import`
nằm phía trên, nên nó chạy trước, và nó chỉ trả quyền lại cho `bao_cao.py` sau
khi đã chạy hết file `so_sach.py`. Chữ "bắt đầu" là bạn đặt cho mình đọc, máy
không nhìn tới.
::
:::
::::

::::code{#mot-cai-da-co-mot-cai-phai-goi}
Đến lượt bạn nhìn tận mắt.

Khung tập chỉ có một ô nhập, mà `import` thì cần hai file nằm cạnh nhau. Nên
đoạn đầu cho chương trình **tự ghi ra** `so_sach.py` và một cuốn sổ ba dòng,
bằng đúng động tác `with open(...)` bạn đã dùng suốt từ bài 3. Đoạn ấy không có
chỗ trống nào; bạn chỉ cần đọc để biết file kia chứa gì.

Hai chỗ trống nằm ở hai câu hỏi khác nhau:

- một câu hỏi thứ **đã có sẵn ngay sau `import`**, do một dòng ở lề trái của
  file kia chạy qua;
- một câu hỏi thứ **chỉ có khi bạn gọi hàm** — và hàm này cần biết đọc cuốn sổ
  nào, nên nó nhận vào một tên file.

```python title=starter
import sys

# ── Phần Byte làm sẵn: ghi ra so_sach.py và một cuốn sổ ba dòng ──────
NGUON_SO_SACH = '''print("so_sach.py: đang mở sổ...")

NGUONG_LON = 100000


def doc_so(ten_file):
    """Đọc file sổ, đưa ra danh sách các khoản dạng (tên, tiền)."""
    khoan = []
    with open(ten_file, "r") as f:
        for dong in f.readlines():
            manh = dong.strip().split(",")
            khoan.append((manh[0], int(manh[1])))
    return khoan
'''

with open("so_sach.py", "w") as f:
    f.write(NGUON_SO_SACH)

with open("so-thang-8.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("trà sữa,45000\n")

# Dòng này là việc riêng của trang học: nó dọn chỗ để lần mượn ngay dưới
# đọc đúng file vừa ghi. Trên máy của bạn không cần nó.
sys.modules.pop("so_sach", None)

# ── Từ đây là bao_cao.py của bạn ─────────────────────────────────────
import so_sach

# Chưa gọi hàm nào cả. Nhưng dòng `NGUONG_LON = 100000` nằm ở lề trái của
# file kia, nên nó đã chạy rồi. Lấy con số ấy ra khỏi hộp.
nguong_ngay_sau_import = ___

# Bây giờ mới tới lượt gọi hàm.
khoan = ___

print(f"Ngay sau import, NGUONG_LON đã bằng {nguong_ngay_sau_import}")
print(f"Gọi hàm xong mới cầm được {len(khoan)} khoản")
```

```python title=solution
import sys

# ── Phần Byte làm sẵn: ghi ra so_sach.py và một cuốn sổ ba dòng ──────
NGUON_SO_SACH = '''print("so_sach.py: đang mở sổ...")

NGUONG_LON = 100000


def doc_so(ten_file):
    """Đọc file sổ, đưa ra danh sách các khoản dạng (tên, tiền)."""
    khoan = []
    with open(ten_file, "r") as f:
        for dong in f.readlines():
            manh = dong.strip().split(",")
            khoan.append((manh[0], int(manh[1])))
    return khoan
'''

with open("so_sach.py", "w") as f:
    f.write(NGUON_SO_SACH)

with open("so-thang-8.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("trà sữa,45000\n")

# Dòng này là việc riêng của trang học: nó dọn chỗ để lần mượn ngay dưới
# đọc đúng file vừa ghi. Trên máy của bạn không cần nó.
sys.modules.pop("so_sach", None)

# ── Từ đây là bao_cao.py của bạn ─────────────────────────────────────
import so_sach

# Chưa gọi hàm nào cả. Nhưng dòng `NGUONG_LON = 100000` nằm ở lề trái của
# file kia, nên nó đã chạy rồi. Lấy con số ấy ra khỏi hộp.
nguong_ngay_sau_import = so_sach.NGUONG_LON

# Bây giờ mới tới lượt gọi hàm.
khoan = so_sach.doc_so("so-thang-8.txt")

print(f"Ngay sau import, NGUONG_LON đã bằng {nguong_ngay_sau_import}")
print(f"Gọi hàm xong mới cầm được {len(khoan)} khoản")
```

```python title=test
# Chỗ trống 1 bị soi bởi câu ngay dưới đây. Chương trình của bạn không tự
# cất con số nào vào đâu cả, nên 100000 chỉ có thể tới từ một dòng đã chạy
# sẵn bên trong file được mượn.
assert nguong_ngay_sau_import == 100000, "so_sach.py đặt NGUONG_LON bằng 100000 ở lề trái, và dòng ấy đã chạy ngay lúc import, nên ngay sau import con số này phải là 100000"
# Chỗ trống 2 bị soi bởi câu này: nó đòi đúng ba khoản của cuốn sổ vừa ghi,
# mỗi khoản là một cặp (tên, tiền) — thứ chỉ có được khi thân hàm doc_so
# thật sự chạy trên đúng tên file ấy.
assert khoan == [("cà phê", 25000), ("bún bò", 40000), ("trà sữa", 45000)], "doc_so đọc so-thang-8.txt và đưa ra ba cặp: cà phê 25000, bún bò 40000, trà sữa 45000"
# Đọc một cái hộp không làm hộp ấy đổi.
assert so_sach.NGUONG_LON == 100000, "sau khi bạn gọi hàm, hộp so_sach vẫn phải giữ nguyên NGUONG_LON bằng 100000"
```

:::hints
- kind: attention
  body: "Sau dòng `import so_sach`, mọi thứ mà file kia sinh ra đều nằm trong một cái hộp mang đúng cái tên ấy. Chỗ trống thứ nhất hỏi một con số đã nằm sẵn trong hộp; chỗ trống thứ hai hỏi một việc phải nhờ hộp làm giúp, mà việc ấy cần biết làm trên cuốn sổ nào."
- kind: strategy
  body: "Cách lấy đồ trong hộp là viết tên hộp, một dấu chấm, rồi tên món đồ — đúng như bài 20. Chỗ trống thứ hai lấy ra một cái hàm, mà một cái hàm chỉ chạy khi có cặp ngoặc theo sau; trong cặp ngoặc ấy đặt tên cuốn sổ vừa được ghi ra ở phần trên, viết trong dấu nháy."
- kind: one-line
  body: "Chỗ trống thứ nhất viết `so_sach.NGUONG_LON`, chỗ thứ hai viết `so_sach.doc_so(\"so-thang-8.txt\")`."
:::

:::validate
- tier: static
  onFail: cả hai chỗ trống đều phải đi qua hộp `so_sach`, và chỗ thứ hai phải GỌI hàm chứ không lấy thẳng một cái tên khác
  requireAst:
  # `min: 2` vì có hai chỗ trống, và cái tên `so_sach` chưa được ĐỌC lần nào
  # trong khung — dòng `import` không tính là đọc, chuỗi trong `sys.modules.pop`
  # cũng không. Nên luật này chặn cả con số 100000 chép tay lẫn danh sách gõ lại
  # bằng tay.
  - kind: uses-name, target: so_sach, min: 2
  # Và chỗ trống thứ hai phải là một lời GỌI, không phải một phép lấy đồ.
  - kind: uses-call, target: doc_so, min: 1
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^so_sach\.py: đang mở sổ\.\.\.\nNgay sau import, NGUONG_LON đã bằng 100000\nGọi hàm xong mới cầm được 3 khoản\s*$
- tier: output
  expect: "Ngay sau import, NGUONG_LON đã bằng 100000"
- tier: output
  expect: "Gọi hàm xong mới cầm được 3 khoản"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mượn một file là chạy cả file đó. Giờ thì câu chào của nó không còn bí ẩn nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vậy là rõ: mọi dòng ở lề trái của `so_sach.py` đều chạy, mỗi khi có ai đó import
nó. Câu chào, hằng số, và cả những dòng bạn viết ra chỉ để thử cho nhanh.

Mà bạn thì vẫn muốn giữ mấy dòng thử ấy. Gõ `python so_sach.py` một cái là thấy
ngay sổ đang có mấy khoản — tiện hơn nhiều so với việc mở một file khác ra chỉ
để kiểm tra. Bỏ chúng đi thì mất chỗ thử; để nguyên thì `bao_cao.py` phải gánh
chúng mỗi lần chạy.

Làm sao để phần thử **chỉ** chạy khi bạn gọi thẳng file ấy, và nằm im khi nó bị
mượn?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
