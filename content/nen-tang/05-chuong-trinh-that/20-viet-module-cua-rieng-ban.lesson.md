---
id: nen-tang.chuong-trinh-that.viet-module-cua-rieng-ban
title: Viết module của riêng bạn
summary: "`import` không dành riêng cho kho công cụ của Python. File `.py` của chính bạn cũng là một cái hộp mượn được — mượn về rồi gọi qua `so_sach.doc_so(...)`."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.own-module]
requires: [core.import-module, mod.import, mod.dotted-access, core.subcommand, core.with-open, io.readlines, core.file-append, core.strip-newline, core.string-split, core.string-strip, io.text-is-str, core.int-cast, core.function-def, core.function-return, core.function-parameter, core.docstring, core.tuple, core.for-unpack, core.list-append, core.list-comprehension, core.comprehension-filter, core.fstring, ctrl.for-each]
concepts: [core.mo-dun, core.tien-to, core.ham]
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
Bạn mượn được hộp của Python rồi. Hộp của bạn thì sao — có cho mượn được không?
::::

::::explain{#chep-ham-la-hen-mot-lan-quen}
Bài trước gom ba việc vào một file, và đoạn đọc sổ chỉ còn nằm ở một chỗ: hàm
`doc_so` trong `so.py`.

Rồi Lan đặt hàng thêm một chương trình nữa — một bản báo cáo in ra từng khoản
kèm tổng cuối tháng. Chương trình ấy không thêm, không xoá, không nhận lệnh
con nào; nó dùng đúng một thứ của `so.py`, là `doc_so`.

Chép hàm ấy sang `bao_cao.py` thì chạy được ngay hôm nay. Nhưng nó đặt sẵn một
cái hẹn: ngày bạn đổi dấu phân cách trong sổ, hoặc thêm một cột ngày tháng vào
mỗi dòng, bạn phải nhớ sửa hai chỗ. Chỗ bạn quên vẫn chạy, vẫn không báo lỗi —
nó chỉ đọc ra những con số khác.

Bạn đã từng mượn đồ mà không chép: `import os` mang cả một hộp công cụ của
Python vào chương trình, và bạn gọi `os.getcwd()`. Cái hộp ấy cũng là một file
`.py` nằm đâu đó trong máy, do người khác viết.

Đây là chỗ đáng dừng lại: **Python không phân biệt file của nó với file của
bạn.** Bất kỳ file `.py` nào nằm cạnh chương trình đang chạy cũng mượn được
theo đúng lối đó. File `.py` của chính bạn là một **module**.

Nên tách đoạn đọc sổ ra thành một file riêng, đặt tên `so_sach.py`:

```python title=readonly
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


def khoan_tren(khoan, nguong):
    """Lọc ra những khoản tốn hơn một mức tiền cho trước."""
    return [k for k in khoan if k[1] > nguong]
```

Không một dòng nào trong đó là mới. Đó là ba hàm bạn đã biết viết, chỉ khác chỗ
chúng nằm: một file của riêng chúng, không kèm chương trình nào.

Giờ cả `so.py` lẫn `bao_cao.py` đều mở đầu bằng một dòng:

```python title=readonly
import so_sach
```

Hai chi tiết trong dòng ấy:

- **Không có `.py`.** Bạn viết tên module, không viết tên file. Tên module là
  tên file bỏ đuôi đi — `so_sach.py` cho ra module `so_sach`.
- **Không có đường dẫn.** Máy đi tìm `so_sach.py` bắt đầu từ thư mục chứa
  chương trình đang chạy, nên hai file nằm cạnh nhau là đủ.

Chỗ tìm ấy khác chỗ tìm của `open`, và hai câu không đá nhau vì chúng nói về
hai việc khác nhau: `open("so.txt")` tính từ thư mục bạn đang đứng lúc gõ lệnh
— đúng như bài 14 đã chỉ ra — còn `import so_sach` bắt đầu từ thư mục chứa file
`.py` đang chạy. Nhờ vế sau, để `so_sach.py` nằm cạnh `bao_cao.py` là đủ, dù
bạn đứng ở đâu lúc gõ lệnh.

Mượn xong thì gọi y như gọi công cụ của `os`: tên hộp, dấu chấm, tên thứ bên
trong hộp.

```python title=readonly
khoan = so_sach.doc_so("so-thang-8.txt")
tong = so_sach.tong_tien(khoan)
```

Bây giờ đoạn đọc sổ nằm ở đúng một chỗ trong cả dự án, và hai chương trình cùng
nhìn vào chỗ ấy. Sửa `so_sach.py` một lần là cả hai cùng đổi theo.
::::

::::example{#muon-hop-cua-chinh-minh}
Trên máy thật bạn tạo `so_sach.py` bằng trình soạn thảo rồi để nó nằm cạnh
chương trình. Trang học này chỉ chạy được một file, nên chương trình phải tự
ghi `so_sach.py` ra đĩa trước — bằng đúng `with open(..., "w")` của các bài
đầu track. Ba dấu nháy đơn mở một chuỗi trải nhiều dòng, để cả nội dung file
nằm gọn trong một cái tên.

```python title=readonly
import sys

NGUON_SO_SACH = '''
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


def khoan_tren(khoan, nguong):
    """Lọc ra những khoản tốn hơn một mức tiền cho trước."""
    return [k for k in khoan if k[1] > nguong]
'''

with open("so_sach.py", "w") as f:
    f.write(NGUON_SO_SACH)

with open("so-thang-8.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("trà sữa,45000\n")

# Dòng này là việc riêng của trang học: nó dọn chỗ để lần `import` ngay dưới
# đọc đúng file vừa ghi. Trên máy của bạn không cần nó.
sys.modules.pop("so_sach", None)

import so_sach

khoan = so_sach.doc_so("so-thang-8.txt")
print(khoan)
print(so_sach.tong_tien(khoan))
print(so_sach.khoan_tren(khoan, 30000))
```

Máy in ra:

```text title=readonly
[('cà phê', 25000), ('bún bò', 40000), ('trà sữa', 45000)]
110000
[('bún bò', 40000), ('trà sữa', 45000)]
```

Cuốn sổ ở đây là cuốn bạn để lại cuối bài trước: ba khoản, sau khi đã thêm trà
sữa và bỏ gửi xe.

Ba chỗ đáng dừng lại nhìn:

- **`import so_sach` mang về đúng một cái tên: `so_sach`.** Cái tên ấy trỏ tới
  cả cái hộp. Ba hàm bên trong không tự bước ra ngoài; chúng vẫn ở trong hộp và
  được với tới qua dấu chấm.
- **Dấu chấm ở đây là cùng một dấu chấm của `os.getcwd()`.** Bên trái là hộp,
  bên phải là thứ nằm trong hộp. Bạn không học một cú pháp mới, bạn dùng lại cú
  pháp cũ trên một cái hộp do bạn viết.
- **`so_sach.py` không có dòng nào gọi hàm.** Nó chỉ định nghĩa. Việc gọi là
  của chương trình đi mượn — đó là điều khiến cùng một hộp phục vụ được `so.py`,
  `bao_cao.py` và mọi chương trình sau này.
::::

::::predict{#doan-goi-tran commitOnce}
Byte mượn hộp xong thì gọi thẳng `doc_so`, không viết tiền tố.

Giả sử `so_sach.py` đã nằm sẵn cạnh chương trình, đúng nội dung ở trên.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
import so_sach

print("đã mượn xong hộp")

khoan = doc_so("so-thang-8.txt")
print(len(khoan))
```

:::opt{correct}
In `đã mượn xong hộp`, rồi chương trình dừng vì `NameError: name 'doc_so' is not defined`
:::

:::opt
In `đã mượn xong hộp`, rồi in `3`
::why
Gần đúng ở chỗ bạn hiểu đúng mục đích của dòng `import`: sau nó, hàm `doc_so`
thật sự đã được nạp vào chương trình và sẵn sàng chạy. Nó có ở đó.

Chỗ lệch là ở chuyện `import` mang về **cái tên nào**. Nó mang về đúng một cái
tên, `so_sach`, và cái tên ấy trỏ tới cả hộp. Những cái tên bên trong hộp không
được rải ra ngoài — muốn tới chúng thì phải đi qua hộp, tức là viết
`so_sach.doc_so(...)`. Viết `doc_so` trần là bảo máy đi tìm một cái tên đứng
ngoài, và ngoài đó không có cái tên ấy.
::
:::

:::opt
Chương trình dừng ngay tại dòng `import`, không in gì cả
::why
Gần đúng ở chỗ bạn cẩn thận với dòng `import`: nó đúng là dòng dễ hỏng, và
`so_sach` không phải một cái tên có sẵn của Python như `os`.

Chỗ lệch là máy tìm module ở đâu. Nó không chỉ tìm trong kho của Python; nó
tìm cả ở thư mục chứa chương trình đang chạy — và `so_sach.py` nằm đúng đó. Nên
dòng `import` chạy trót lọt, câu `đã mượn xong hộp` kịp in ra, và chuyện chỉ
hỏng ở dòng sau đó.
::
:::

:::opt
Chương trình dừng vì `NameError`, và câu `đã mượn xong hộp` cũng không kịp in ra
::why
Gần đúng ở chỗ bạn nhớ đúng một chuyện có thật: có loại lỗi máy phát hiện
**trước khi** chạy dòng nào — Realm 0 gọi nó là `SyntaxError`, và lúc ấy quả
thật không dòng nào được chạy, kể cả dòng `print` đứng trước chỗ hỏng.

Chỗ lệch là `NameError` không thuộc loại đó. Nó là lỗi **lúc chạy**: máy chỉ
biết `doc_so` không tồn tại vào đúng khoảnh khắc nó chạy tới dòng gọi hàm ấy.
Mọi dòng phía trên đã chạy xong xuôi từ trước, nên `đã mượn xong hộp` đã nằm
trên màn hình rồi.
::
:::
::::

::::code{#bao-cao-muon-tu-hop}
Đây là `bao_cao.py`: nó không thêm, không xoá, không nhận lệnh con. Nó mở cuốn
sổ ra, in từng khoản, rồi in tổng — và nó không tự đọc file, nó mượn `so_sach`.

Ba chỗ trống, và cả ba cùng hỏi một câu: **cái hộp tên là gì**.

```python title=starter
import sys

NGUON_SO_SACH = '''
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


def khoan_tren(khoan, nguong):
    """Lọc ra những khoản tốn hơn một mức tiền cho trước."""
    return [k for k in khoan if k[1] > nguong]
'''

with open("so_sach.py", "w") as f:
    f.write(NGUON_SO_SACH)

with open("so-thang-8.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("trà sữa,45000\n")

sys.modules.pop("so_sach", None)

import ___

khoan = ___.doc_so("so-thang-8.txt")
tong = ___.tong_tien(khoan)

dong_bao_cao = []
for ten, tien in khoan:
    dong_bao_cao.append(f"{ten}: {tien} đồng")
dong_bao_cao.append(f"Tổng: {tong} đồng")

for dong in dong_bao_cao:
    print(dong)
```

```python title=solution
import sys

NGUON_SO_SACH = '''
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


def khoan_tren(khoan, nguong):
    """Lọc ra những khoản tốn hơn một mức tiền cho trước."""
    return [k for k in khoan if k[1] > nguong]
'''

with open("so_sach.py", "w") as f:
    f.write(NGUON_SO_SACH)

with open("so-thang-8.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("trà sữa,45000\n")

sys.modules.pop("so_sach", None)

import so_sach

khoan = so_sach.doc_so("so-thang-8.txt")
tong = so_sach.tong_tien(khoan)

dong_bao_cao = []
for ten, tien in khoan:
    dong_bao_cao.append(f"{ten}: {tien} đồng")
dong_bao_cao.append(f"Tổng: {tong} đồng")

for dong in dong_bao_cao:
    print(dong)
```

```python title=test
# Chỗ trống thứ nhất (`import ___`): mượn nhầm tên thì không có hộp nào để
# gọi, và hai dòng dưới nó dừng ngay.
#
# Chỗ trống thứ hai (`___.doc_so`): nếu nó không dẫn tới cái hộp vừa mượn thì
# `khoan` không bao giờ thành ba khoản này.
assert khoan == [("cà phê", 25000), ("bún bò", 40000), ("trà sữa", 45000)], "so_sach.doc_so phải đọc file so-thang-8.txt thành ba khoản: cà phê 25000, bún bò 40000, trà sữa 45000"
# Chỗ trống thứ ba (`___.tong_tien`): con số này chỉ ra đúng khi tổng được
# tính bằng hàm trong hộp, trên đúng ba khoản ở trên.
assert tong == 110000, "ba khoản 25000, 40000 và 45000 cộng lại là 110000"
assert dong_bao_cao == [
    "cà phê: 25000 đồng",
    "bún bò: 40000 đồng",
    "trà sữa: 45000 đồng",
    "Tổng: 110000 đồng",
], "bản báo cáo phải có bốn dòng: ba khoản theo đúng thứ tự ghi trong sổ, rồi dòng tổng 110000"
# Hộp phải thật sự được mượn, chứ không phải ba hàm ấy được chép thẳng vào đây.
assert so_sach.khoan_tren(khoan, 30000) == [("bún bò", 40000), ("trà sữa", 45000)], "hộp so_sach còn giữ hàm khoan_tren; lọc ba khoản trên với ngưỡng 30000 phải còn lại bún bò 40000 và trà sữa 45000"
```

:::hints
- kind: attention
  body: Ba chỗ trống điền cùng một chữ. Chữ ấy đã xuất hiện hai lần ngay phía trên, trong dòng ghi file — nhưng ở đó nó đi kèm đuôi `.py`, còn ở đây thì không.
- kind: strategy
  body: Tên module là tên file bỏ đuôi `.py` đi. Sau khi mượn, cái tên ấy trỏ tới cả cái hộp, nên muốn tới một hàm bên trong thì viết tên hộp, dấu chấm, rồi tên hàm — đúng hình dạng `os.getcwd()` mà bạn đã dùng ở bài mượn đồ nghề.
- kind: one-line
  body: Cả ba chỗ trống đều là `so_sach`.
:::

:::validate
- tier: static
  requireAst:
  - kind: uses-call, target: doc_so, min: 1
  - kind: uses-call, target: tong_tien, min: 1
  onFail: bản báo cáo phải gọi `doc_so` và `tong_tien` của hộp `so_sach`, không tự đọc file lấy
- tier: run
  timeoutMs: 8000
- tier: tests
  timeoutMs: 8000
- tier: output
  expect: "cà phê: 25000 đồng"
- tier: output
  expect: "Tổng: 110000 đồng"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một cái hộp, hai chương trình cùng mượn. Sửa cuốn sổ một chỗ là cả hai cùng
biết.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bản báo cáo vừa rồi ngắn, nên tiền tố `so_sach.` mới hiện ra hai lần. `so.py`
thì khác: ba lệnh con, mỗi lệnh mở sổ rồi cộng tiền rồi ghi lại, nên cái tiền
tố ấy rải kín cả file — và mỗi lệnh con bạn thêm vào sau này lại kéo theo một
nắm nữa.

Nhìn kỹ hơn thì cả file dùng đúng **hai** cái tên trong hộp — `doc_so` và
`tong_tien` — mà lần nào cũng phải đi vòng qua tên hộp mới với tới được.

Lấy riêng hai cái tên ấy ra được không? Mượn về thẳng chúng, rồi gọi trần như
gọi một hàm bạn tự viết ngay trong file?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
