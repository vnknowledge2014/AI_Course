---
id: nen-tang.chuong-trinh-that.dua-ten-file-tu-ngoai-vao
title: Đưa tên file vào từ ngoài
summary: "`sys.argv` là danh sách những chữ mà dòng lệnh trao cho chương trình — `argv[0]` là chính tên chương trình, nên chữ người dùng gõ thêm nằm ở `argv[1]`."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [core.sys-argv]
requires: [core.pathlib-path, core.absolute-path, mod.import, mod.dotted-access, core.terminal, core.with-open, core.file-write, core.file-readlines, core.string-split, core.string-strip, core.int-cast, core.list, core.list-index, core.len, core.string-literal, core.variable, core.assignment, core.output, core.fstring, core.function-def, core.function-return, core.function-parameter, core.docstring, core.accumulator, ctrl.for-each]
concepts: [core.danh-sach, core.chi-so, core.dong-lenh]
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
Lan gõ thêm mấy chữ sau tên chương trình. Mấy chữ ấy đi đâu mất?
::::

::::explain{#may-chu-go-them-sau-ten-chuong-trinh}
Đường dẫn của bạn đã đúng ở mọi máy. Nhưng cái tên `"so.txt"` thì vẫn nằm cứng
trong code, và đó là một bức tường dựng ngay trước mặt người dùng: muốn mở cuốn
sổ tháng trước, Lan phải mở file code ra sửa một dòng — mà Lan không viết Python.

Từ R0·19 tới giờ bạn có đúng một cách hỏi người dùng: `input()`. Nó dừng chương
trình lại và chờ người gõ. Cách ấy dùng được, nhưng nó ép người ta phải ngồi đó
trả lời từng câu, và một chương trình phải ngồi chờ thì không đem chạy tự động
hằng đêm được.

Có một chỗ khác để nói, và bạn đã đứng ở đó từ R0·7: **dòng lệnh**. Khi Lan gõ

```text title=readonly
python so.py so-thang-3.txt
```

thì Lan vừa nói ba chữ với máy trong đúng một lần bấm Enter: chạy bằng `python`,
chạy chương trình `so.py`, và làm việc với `so-thang-3.txt`. Chữ thứ ba là thứ
Lan muốn nói cho chương trình, và nó được nói ra **trước khi** chương trình chạy
dòng đầu tiên — không có ai phải chờ ai.

Terminal gom những chữ ấy lại rồi trao cho chương trình. Chỗ nó đặt vào nằm
trong hộp `sys`, mượn về đúng cách bài 15 đã dạy:

```python title=readonly
import sys

print(sys.argv)
```

`sys.argv` là một **list các chuỗi** — vẫn là cái list bạn đã dùng suốt T1.4,
không có gì mới về hình dạng. Cái mới là nội dung của nó đến từ bên ngoài
chương trình.

Hai điều cần nhớ về nội dung ấy:

- **Chữ `python` không có trong list.** Nó là lệnh gọi trình chạy, không phải
  điều bạn nói với chương trình.
- **Phần tử đầu tiên, `argv[0]`, là chính tên chương trình** — `"so.py"`. Nên
  chữ đầu tiên mà người dùng gõ thêm nằm ở `argv[1]`, không phải `argv[0]`.

Chỗ lệch một nấc ấy là chỗ dễ vấp nhất của bài này, nên đáng nói ra vì sao nó
lệch: `argv` không phải danh sách những điều người dùng dặn, nó là **cả câu lệnh
đã gõ**, chép lại từ đầu. Câu lệnh nào cũng bắt đầu bằng tên chương trình, nên
ô số 0 lúc nào cũng đã có chủ.

Còn một chuyện nói thẳng ngay: trang học này không có terminal thật để bạn gõ
vào. Nên trong mọi đoạn mã dưới đây, Byte **chép sẵn** vào `sys.argv` đúng những
chữ mà một dòng lệnh sẽ đặt vào đó. Gán vào `sys.argv` là chuyện chỉ làm khi
đang tập; ngoài đời terminal điền chỗ ấy giúp bạn.
::::

::::example{#nhin-vao-argv}
Một dòng lệnh, và những gì chương trình nhận được từ nó.

```python title=readonly
import sys

# Dòng lệnh đã gõ:  python so.py so-thang-3.txt
sys.argv = ["so.py", "so-thang-3.txt"]

print(sys.argv)
print("Số chữ nhận được:", len(sys.argv))
print("Ô số 0:", sys.argv[0])
print("Ô số 1:", sys.argv[1])
```

Máy in ra:

```text title=readonly
['so.py', 'so-thang-3.txt']
Số chữ nhận được: 2
Ô số 0: so.py
Ô số 1: so-thang-3.txt
```

Ba chỗ đáng dừng lại nhìn:

- **Mọi ô đều là chuỗi.** `sys.argv` cho về chữ, kể cả khi người dùng gõ toàn
  chữ số — y hệt `input()` ở R0·20 và y hệt mọi dòng đọc lên từ file ở bài 10.
  Muốn có số thì vẫn phải `int()` lại.
- **Danh sách có hai ô cho một chữ người dùng gõ thêm.** Ô kia là tên chương
  trình, do terminal đặt vào, không do ai gõ thêm.
- **Chỉ số vẫn đếm từ 0.** Không có luật riêng nào cho `argv`; nó là một list
  bình thường, nên `argv[1]` nghĩa là ô thứ hai, đúng như R0·34.
::::

::::predict{#doan-argv-co-gi commitOnce}
Lan gõ đúng một dòng lệnh, và Byte chép lại nó vào `sys.argv`.

**Trước khi bấm chạy**, bạn đoán hai dòng cuối in ra gì?

```python title=readonly
import sys

# Dòng lệnh đã gõ:  python so.py so-thang-2.txt chi-tiet
sys.argv = ["so.py", "so-thang-2.txt", "chi-tiet"]

print(len(sys.argv))
print(sys.argv[1])
```

:::opt{correct}
`3` rồi `so-thang-2.txt`
:::

:::opt
`2` rồi `so-thang-2.txt`
::why
Gần đúng ở chỗ bạn đọc đúng **ý nghĩa** của dòng lệnh: Lan gõ thêm hai chữ cho
chương trình, và chữ đầu trong hai chữ ấy đúng là `so-thang-2.txt`. Phần bạn
hiểu về ý định của Lan là chính xác.

Chỗ lệch là `sys.argv` không chỉ chứa những chữ người dùng gõ thêm. Nó chép lại
cả câu lệnh kể từ tên chương trình, nên ô số 0 đã bị `"so.py"` chiếm chỗ và
danh sách dài 3, không phải 2. Nhìn thẳng vào dòng gán ở trên cũng thấy: trong
cặp ngoặc vuông có ba câu chữ.
::
:::

:::opt
`3` rồi `so.py`
::why
Gần đúng ở chỗ bạn đếm đúng độ dài: danh sách có ba phần tử, và bạn đọc `len`
không sai.

Chỗ lệch nằm ở nửa sau: bạn lấy ô số 0 trong khi dòng in hỏi ô số **1**. Ô số
0 giữ tên chương trình, `"so.py"` — và chính vì nó chiếm mất ô số 0 nên chữ
người dùng gõ thêm mới bị đẩy sang ô số 1. Đây là toàn bộ chỗ lệch một nấc của
bài này, và dòng `print` vừa hỏi thẳng vào chỗ ấy.
::
:::

:::opt
`4` rồi `so.py`
::why
Gần đúng ở chỗ bạn đếm đúng số chữ trên dòng lệnh thật: `python`, `so.py`,
`so-thang-2.txt`, `chi-tiet` — Lan gõ bốn chữ, không sai.

Chỗ lệch là chữ `python` dừng lại ở terminal. Nó nói cho hệ máy biết phải gọi
trình chạy nào; tới lúc chương trình của bạn bắt đầu chạy thì việc ấy đã xong
rồi. Cái list bắt đầu từ tên chương trình trở đi, nên nó dài 3 — và ô số 1,
thứ dòng in đang hỏi, là `so-thang-2.txt`.
::
:::
::::

::::code{#doc-cuon-so-ma-dong-lenh-chi-dinh}
Cùng một chương trình, ba lượt chạy với ba dòng lệnh khác nhau. Byte dựng sẵn
hai cuốn sổ trong thư mục dự án và viết sẵn hàm cộng tiền; việc của bạn là lấy
ra cái tên mà dòng lệnh vừa trao.

Ba chỗ trống nằm ở ba lượt, và cả ba điền giống hệt nhau: mỗi chỗ phải lấy ra
tên cuốn sổ từ danh sách mà dòng lệnh vừa trao. Lượt thứ ba tồn tại vì một lý do
riêng: ở đó Lan gõ quen tay thêm một chữ nữa ở cuối, chữ mà chương trình lần này
chưa dùng tới. Cuốn sổ cần mở vẫn là chữ Lan gõ **ngay sau** tên chương trình,
không phải chữ cuối cùng của dòng lệnh.

```python title=starter
import os
import sys
import pathlib

# Giàn giáo: dựng thư mục dự án và đặt sẵn hai cuốn sổ vào đó.
os.makedirs("/Users/lan/du-an", exist_ok=True)
thu_muc_du_an = pathlib.Path("/Users/lan/du-an")

with open(thu_muc_du_an / "so-thang-2.txt", "w") as f:
    f.write("cà phê,25000\nbún bò,40000\n")

with open(thu_muc_du_an / "so-thang-3.txt", "w") as f:
    f.write("cà phê,25000\nsửa xe,500000\nvở ghi,15000\n")


def tinh_tong(duong_dan):
    """Cộng tiền của mọi khoản trong cuốn sổ nằm ở đường dẫn này."""
    tong = 0
    with open(duong_dan, "r") as f:
        for dong in f.readlines():
            manh = dong.strip().split(",")
            tong = tong + int(manh[1])
    return tong


# Lượt 1 — Lan gõ:  python so.py so-thang-3.txt
sys.argv = ["so.py", "so-thang-3.txt"]
ten_luot_1 = ___
tong_luot_1 = tinh_tong(thu_muc_du_an / ten_luot_1)

# Lượt 2 — Lan gõ:  python so.py so-thang-2.txt
sys.argv = ["so.py", "so-thang-2.txt"]
ten_luot_2 = ___
tong_luot_2 = tinh_tong(thu_muc_du_an / ten_luot_2)

# Lượt 3 — Lan gõ:  python so.py so-thang-2.txt chi-tiet
sys.argv = ["so.py", "so-thang-2.txt", "chi-tiet"]
ten_luot_3 = ___
tong_luot_3 = tinh_tong(thu_muc_du_an / ten_luot_3)

print(f"Lượt 1 — {ten_luot_1}: {tong_luot_1} đồng")
print(f"Lượt 2 — {ten_luot_2}: {tong_luot_2} đồng")
print(f"Lượt 3 — {ten_luot_3}: {tong_luot_3} đồng")
```

```python title=solution
import os
import sys
import pathlib

# Giàn giáo: dựng thư mục dự án và đặt sẵn hai cuốn sổ vào đó.
os.makedirs("/Users/lan/du-an", exist_ok=True)
thu_muc_du_an = pathlib.Path("/Users/lan/du-an")

with open(thu_muc_du_an / "so-thang-2.txt", "w") as f:
    f.write("cà phê,25000\nbún bò,40000\n")

with open(thu_muc_du_an / "so-thang-3.txt", "w") as f:
    f.write("cà phê,25000\nsửa xe,500000\nvở ghi,15000\n")


def tinh_tong(duong_dan):
    """Cộng tiền của mọi khoản trong cuốn sổ nằm ở đường dẫn này."""
    tong = 0
    with open(duong_dan, "r") as f:
        for dong in f.readlines():
            manh = dong.strip().split(",")
            tong = tong + int(manh[1])
    return tong


# Lượt 1 — Lan gõ:  python so.py so-thang-3.txt
sys.argv = ["so.py", "so-thang-3.txt"]
ten_luot_1 = sys.argv[1]
tong_luot_1 = tinh_tong(thu_muc_du_an / ten_luot_1)

# Lượt 2 — Lan gõ:  python so.py so-thang-2.txt
sys.argv = ["so.py", "so-thang-2.txt"]
ten_luot_2 = sys.argv[1]
tong_luot_2 = tinh_tong(thu_muc_du_an / ten_luot_2)

# Lượt 3 — Lan gõ:  python so.py so-thang-2.txt chi-tiet
sys.argv = ["so.py", "so-thang-2.txt", "chi-tiet"]
ten_luot_3 = sys.argv[1]
tong_luot_3 = tinh_tong(thu_muc_du_an / ten_luot_3)

print(f"Lượt 1 — {ten_luot_1}: {tong_luot_1} đồng")
print(f"Lượt 2 — {ten_luot_2}: {tong_luot_2} đồng")
print(f"Lượt 3 — {ten_luot_3}: {tong_luot_3} đồng")
```

```python title=test
# Ba chỗ trống, và mỗi lượt loại đi một cách qua bài nhờ ăn may:
#
#   lượt 1 → so-thang-3.txt : loại ô số 0, vì ô ấy giữ "so.py" và mở nó thì
#                             không có file nào tên như vậy
#   lượt 2 → so-thang-2.txt : loại mọi cách chép cứng một cái tên, vì lượt này
#                             phải ra một cuốn sổ KHÁC lượt 1
#   lượt 3 → so-thang-2.txt : loại ô cuối cùng của danh sách. Đây là lượt duy
#                             nhất mà dòng lệnh dài hơn hai chữ, nên nó là chỗ
#                             duy nhất phân biệt "ô số 1" với "ô cuối cùng".
assert ten_luot_1 == "so-thang-3.txt", "lượt 1 gõ `python so.py so-thang-3.txt`, nên cái tên lấy ra phải là so-thang-3.txt — nếu bạn nhận được so.py thì đang lấy ô số 0, ô mà terminal dành cho tên chương trình"
assert tong_luot_1 == 540000, "sổ tháng 3 có cà phê 25000, sửa xe 500000 và vở ghi 15000, cộng lại thành 540000"
assert ten_luot_2 == "so-thang-2.txt", "lượt 2 gõ `python so.py so-thang-2.txt`, nên cái tên lấy ra phải là so-thang-2.txt — nếu nó vẫn là so-thang-3.txt thì cái tên đang được chép cứng chứ không lấy từ dòng lệnh"
assert tong_luot_2 == 65000, "sổ tháng 2 có cà phê 25000 và bún bò 40000, cộng lại thành 65000"
assert ten_luot_3 == "so-thang-2.txt", "lượt 3 gõ `python so.py so-thang-2.txt chi-tiet`, và cuốn sổ vẫn là chữ ngay sau tên chương trình — nếu bạn nhận được chi-tiet thì đang lấy ô CUỐI của danh sách, mà ô cuối chỉ tình cờ đúng khi dòng lệnh có đúng hai chữ"
assert tong_luot_3 == 65000, "lượt 3 đọc lại chính sổ tháng 2, nên tổng của nó phải bằng tổng của lượt 2, tức 65000"
assert sys.argv[0] == "so.py", "ô số 0 của lần gán cuối vẫn phải là tên chương trình so.py — chỗ trống chỉ được đọc từ sys.argv, không được sửa nó"
```

:::hints
- kind: attention
  body: Chỗ trống phải cho ra **tên cuốn sổ**, tức là một câu chữ. Ngay dòng trên nó là danh sách những chữ mà dòng lệnh vừa trao cho chương trình, và cái tên bạn cần đang nằm sẵn trong đó. Việc của bạn là chỉ đúng ô nào giữ nó.
- kind: strategy
  body: Lấy một ô ra khỏi danh sách thì viết tên danh sách rồi đặt chỉ số vào cặp ngoặc vuông, đúng như R0·34 — ở đây tên danh sách là `sys.argv`. Chỉ số nào thì nhớ rằng ô đầu tiên đã bị tên chương trình chiếm, nên chữ người dùng gõ thêm bị đẩy lùi một nấc. Đừng gõ thẳng tên cuốn sổ vào chỗ trống: làm vậy là đưa cái tên trở lại nằm cứng trong code, đúng thứ bài này đang gỡ ra.
- kind: one-line
  body: "Viết `sys.argv[1]` vào cả ba chỗ trống."
:::

:::validate
- tier: static
  requireAst:
    - kind: uses-name, target: sys, min: 6
  onFail: "cả ba chỗ trống phải đọc tên cuốn sổ ra từ `sys.argv` — gõ thẳng tên file vào là đưa nó trở lại nằm cứng trong code"
- tier: run
  timeoutMs: 8000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: ^Lượt 1 — so-thang-3\.txt: 540000 đồng\nLượt 2 — so-thang-2\.txt: 65000 đồng\nLượt 3 — so-thang-2\.txt: 65000 đồng\s*$
- tier: output
  expect: "Lượt 3 — so-thang-2.txt: 65000 đồng"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một chương trình, ba dòng lệnh, ba cuốn sổ. Và không ai phải mở code ra sửa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Chương trình nhận được tên file từ ngoài rồi. Lan gõ tên cuốn sổ nào thì nó mở
cuốn ấy, và Lan không phải chạm vào một dòng code nào.

Nhưng nó chỉ biết làm đúng một việc: in tổng. Lan còn muốn **thêm** một khoản
chi vừa tiêu, và thỉnh thoảng **xoá** một khoản ghi nhầm. Theo cách đang có,
mỗi việc là một chương trình: `xem.py` để in tổng, `them.py` để ghi thêm,
`xoa.py` để bỏ một dòng. Ba file — và đoạn mở sổ, đọc từng dòng, tách dấu phẩy
thì nằm y hệt nhau ở cả ba. Sửa một chỗ là phải nhớ sửa ba lần, mà nhớ hai lần
thì sổ bắt đầu nói dối.

Ba việc ấy đều làm trên cùng một cuốn sổ, và người dùng thì chỉ có đúng một chỗ
để nói chuyện là dòng lệnh. Có cách nào để **một** chương trình làm được cả ba,
và người dùng nói cho nó biết lần này họ muốn việc nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
