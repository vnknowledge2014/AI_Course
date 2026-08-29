---
id: nen-tang.chuong-trinh-that.boss-so-chi-tieu-cua-byte
title: BOSS — Sổ chi tiêu của Byte
summary: "Cuốn sổ rời khỏi file code và nằm trên đĩa: một chương trình nhận lệnh con từ dòng lệnh, ghi nối vào sổ, đọc lại sổ, và không chết vì một dòng gõ nhầm."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
practices: [core.pure-function, core.one-job-name, core.function-def, core.function-return, core.return-multiple, core.docstring, core.tuple, core.tuple-unpack, core.accumulator, core.counter-if, core.string-strip, core.string-method, core.int-cast, core.value-error, core.list-index, core.len, core.fstring, ctrl.for-each, ctrl.if, ctrl.elif, ctrl.else]
requires: [core.function-def, core.function-return, core.function-parameter, core.docstring, core.pure-function, core.one-job-name, core.accumulator, core.counter-if, core.return-multiple, core.tuple, core.tuple-unpack, core.list, core.list-index, core.len, core.int-cast, core.value-error, core.string-strip, core.string-method, core.fstring, core.variable, core.assignment, core.reassign, core.string-literal, core.output, ctrl.for-each, ctrl.if, ctrl.elif, ctrl.else, ctrl.comparison, err.traceback]
concepts: [core.file, core.duong-dan, core.ham]
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
Hôm nay không có công cụ nào mới. Chỉ có một cuốn sổ phải sống được qua đêm.
::::

::::explain{#du-manh-roi}
Bài trước kết bằng một câu hỏi thẳng: `tinh_tong(cac_dong)` giờ chạy được mà
không cần một file nào, bạn đã có đủ mọi mảnh — ghép thành một cuốn sổ chi tiêu
thật được chưa?

Được. Và bài này không mang theo thứ gì mới để ghép: mọi thứ dùng ở đây đều đã
có tên và đã có một bài của riêng nó.

Đây là những mảnh sắp được ghép lại, mỗi mảnh một dòng:

- **Ghi xuống đĩa** — `with open(...) as f` rồi `f.write(...)`, và hai chế độ
  không được lẫn: `"w"` xoá sạch trước khi ghi, `"a"` ghi nối vào cuối.
- **Ký tự `\n`** ở cuối mỗi lần ghi, để các khoản không dính liền một dòng.
- **Đọc lại** — `"r"` và `.readlines()` cho về một danh sách dòng, mỗi dòng còn
  dính `\n` ở đuôi.
- **Làm sạch rồi tách** — `.strip()` cắt hai đầu, `.split(",")` cắt tại dấu phẩy.
- **`int()`** — vì trong file thì tất cả đều là chữ.
- **`try` / `except ValueError`** — một dòng gõ nhầm không được giết cả cuốn sổ.
- **`except FileNotFoundError`** — chạy lần đầu trên máy chưa có sổ nào cũng phải
  ra một câu trả lời tử tế.
- **`pathlib.Path`** — đường dẫn ghép đúng trên mọi hệ máy.
- **Dòng lệnh** — ô số 0 là tên chương trình, ô số 1 mới là **việc** cần làm.
- **`if __name__ == "__main__":`** — phần tự chạy tách khỏi phần cho mượn.
- **Và luật của bài 27** — một hàm hoặc chạm thế giới bên ngoài, hoặc chỉ nhận
  vào – trả ra.

Cặp `encoding="utf-8"` đi kèm mỗi lần `open` trong bài là để chữ có dấu ra vào
đúng trên mọi máy.
::::

::::example{#hai-nua-cua-mot-chuong-trinh}
Trước khi ghép cả cuốn sổ, hãy nhìn luật của bài 27 ở cỡ nhỏ nhất: hai hàm, một
cái chạm đĩa, một cái không.

```python title=readonly
def tinh_tong(cac_dong):
    """Nhận vào các dòng chữ, đưa ra tổng tiền. Không mở file nào."""
    tong = 0
    for dong in cac_dong:
        manh = dong.strip().split(",")
        tong = tong + int(manh[1])
    return tong


def doc_so(duong_dan):
    """Chạm vào đĩa: mở sổ ra và đưa lại mọi dòng."""
    with open(duong_dan, "r", encoding="utf-8") as f:
        return f.readlines()


with open("so-ngan.txt", "w", encoding="utf-8") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("gửi xe,10000\n")

print(doc_so("so-ngan.txt"))
print(tinh_tong(doc_so("so-ngan.txt")))
print(tinh_tong(["gửi xe,10000\n", "đổ xăng,100000\n"]))
```

Máy in ra:

```text title=readonly
['cà phê,25000\n', 'bún bò,40000\n', 'gửi xe,10000\n']
75000
110000
```

Ba chỗ đáng dừng lại nhìn:

- **Dòng đọc lên còn dính `\n`.** `.readlines()` giữ nguyên ký tự xuống dòng ở
  đuôi **mọi** phần tử, kể cả phần tử cuối — vì dòng cuối của sổ cũng được ghi
  kèm `\n`, bạn nhìn thấy nó trong danh sách vừa in ra. Riêng `int()` thì rộng
  lượng: `int("40000\n")` vẫn cho 40000. Nên `.strip()` ở đây không cứu phép
  cộng; nó cứu mọi việc khác — so sánh tên khoản, in ra màn hình, ghép chuỗi —
  những chỗ mà một ký tự vô hình dính ở đuôi làm hỏng lặng lẽ.
- **`tinh_tong` không biết file là gì.** Nó nhận một danh sách chuỗi, và nguồn
  gốc của danh sách ấy không phải việc của nó.
- **Dòng cuối chứng minh điều đó.** `tinh_tong(["gửi xe,10000\n", "đổ xăng,100000\n"])`
  chạy được mà trên đĩa không có file nào tên như vậy — 10000 + 100000 = 110000.
  Đây chính là thứ khiến bài này chấm được: nửa không chạm file thì kiểm bằng
  một dòng `assert` cũng xong.
::::

::::predict{#doan-ghi-bang-w commitOnce}
Byte viết lệnh **thêm một khoản** vào sổ. Cuốn sổ đã có ba dòng; Byte mở nó ra
lần nữa để ghi khoản thứ tư, và dùng cùng một chế độ đã dùng lúc tạo sổ.

**Trước khi bấm chạy**, bạn đoán dòng cuối in ra gì?

```python title=readonly
with open("so-thu.txt", "w", encoding="utf-8") as f:
    f.write("cà phê,25000\n")
    f.write("sửa xe,500000\n")
    f.write("bún bò,40000\n")

with open("so-thu.txt", "w", encoding="utf-8") as f:
    f.write("trà sữa,45000\n")

with open("so-thu.txt", "r", encoding="utf-8") as f:
    print(f.readlines())
```

:::opt{correct}
`['trà sữa,45000\n']`
:::

:::opt
`['cà phê,25000\n', 'sửa xe,500000\n', 'bún bò,40000\n', 'trà sữa,45000\n']`
::why
Gần đúng ở chỗ bạn đọc đúng **ý định** của đoạn code: lần mở thứ hai mang
nghĩa "thêm khoản trà sữa vào cuốn sổ đang có", và bốn dòng là đúng thứ Byte
muốn thấy.

Chỗ lệch nằm ở chữ `"w"`. Nó không có nghĩa "ghi vào", nó có nghĩa **ghi đè**:
ngay lúc `open` mở file ở chế độ ấy, mọi thứ đang nằm trong file bị xoá sạch,
trước cả khi `f.write` nói được câu nào. Chế độ ghi nối vào cuối là `"a"`, và
đó cũng là chỗ trống thứ nhất bạn sắp phải điền ở cuối bài.
::
:::

:::opt
`['cà phê,25000\n', 'sửa xe,500000\n', 'bún bò,40000\n']`
::why
Gần đúng ở chỗ bạn đang cẩn thận đúng hướng: bạn nghi rằng mở lại một file đã
có bằng `"w"` là chuyện máy sẽ không cho làm, nên lần ghi thứ hai không có tác
dụng gì và sổ giữ nguyên ba dòng cũ.

Chỗ lệch là máy không hỏi lại câu nào cả. `"w"` mở được cả file chưa tồn tại
lẫn file đã có: chưa có thì nó tạo, đã có thì nó dọn trắng. Không có cảnh báo,
không có lỗi — và cũng vì thế mà chọn nhầm chế độ là loại lỗi im lặng, đúng
kiểu bạn đã đi tìm bằng `breakpoint()` ở bài 25.
::
:::

:::opt
`['cà phê,25000\nsửa xe,500000\nbún bò,40000\n', 'trà sữa,45000\n']`
::why
Gần đúng ở chỗ bạn nhớ rằng file này được ghi bằng **hai lần mở** khác nhau, và
bạn đếm đúng: hai lần.

Chỗ lệch là `.readlines()` không cắt theo số lần ghi. Nó cắt theo ký tự `\n`,
và ký tự ấy là thứ duy nhất trong file đánh dấu chỗ hết dòng — file không giữ
lại bất cứ dấu vết nào về việc ai đã ghi nó, ghi mấy lần, ghi lúc nào. Nếu cả
ba khoản đầu được ghi bằng một lần `f.write` duy nhất mà vẫn có đủ ba dấu `\n`,
kết quả cắt ra vẫn y hệt.
::
:::
::::

::::explain{#nua-khong-cham-file}
Đoán xong thì chốt lại chỗ vừa suýt trượt: `"w"` **dọn trắng**, `"a"` **ghi
nối**. Lệnh thêm một khoản vào sổ chỉ có một chế độ đúng.

Giờ tới nửa còn lại — nửa không chạm file. Cuốn sổ của Byte có sáu dòng, và một
dòng trong đó bị gõ bằng chữ:

```text title=readonly
cà phê,25000
sửa xe,500000
bún bò,40000
đổ xăng,100000
bánh mì,mười lăm nghìn
gửi xe,10000
```

Đọc lên, đây là sáu chuỗi. Năm chuỗi có `int()` đọc được, một chuỗi thì không —
`int("mười lăm nghìn")` nổ `ValueError`, đúng như bài 10 đã cho bạn thấy.

Nên hàm tính toán phải trả về **hai** con số, không phải một: tổng tiền, và số
dòng đã phải bỏ qua. Con số thứ hai mới là thứ nói thật với người dùng — một
cuốn sổ báo "675 nghìn" mà im lặng về dòng nó không đọc được thì đang nói dối
nhẹ nhàng.

Hai chỗ trống ở bước sau nằm trong cùng một vòng lặp, ở hai nhánh khác nhau của
cùng một câu `try`: nhánh chạy êm và nhánh vấp `ValueError`.
::::

::::code{#dem-ca-tien-lan-dong-hong}
Sáu dòng, hai con số. Hàm này không mở file nào — nó nhận danh sách chuỗi mà
`.readlines()` sẽ đưa cho nó.

```python title=starter
dong_da_doc = [
    "cà phê,25000\n",
    "sửa xe,500000\n",
    "bún bò,40000\n",
    "đổ xăng,100000\n",
    "bánh mì,mười lăm nghìn\n",
    "gửi xe,10000\n",
]


def tong_va_bo_qua(cac_dong):
    """Cộng tiền các dòng đọc được, và đếm số dòng phải bỏ qua."""
    tong = 0
    bo_qua = 0
    for dong in cac_dong:
        manh = dong.strip().split(",")
        try:
            tong = tong + ___
        except ValueError:
            bo_qua = ___
    return tong, bo_qua


tong, bo_qua = tong_va_bo_qua(dong_da_doc)
print(f"Tổng đọc được: {tong} đồng")
print(f"Bỏ qua {bo_qua} dòng hỏng")
```

```python title=solution
dong_da_doc = [
    "cà phê,25000\n",
    "sửa xe,500000\n",
    "bún bò,40000\n",
    "đổ xăng,100000\n",
    "bánh mì,mười lăm nghìn\n",
    "gửi xe,10000\n",
]


def tong_va_bo_qua(cac_dong):
    """Cộng tiền các dòng đọc được, và đếm số dòng phải bỏ qua."""
    tong = 0
    bo_qua = 0
    for dong in cac_dong:
        manh = dong.strip().split(",")
        try:
            tong = tong + int(manh[1])
        except ValueError:
            bo_qua = bo_qua + 1
    return tong, bo_qua


tong, bo_qua = tong_va_bo_qua(dong_da_doc)
print(f"Tổng đọc được: {tong} đồng")
print(f"Bỏ qua {bo_qua} dòng hỏng")
```

```python title=test
# Chỗ trống thứ nhất (nhánh chạy êm) bị soi bởi con số tổng: năm dòng đọc được
# cộng lại là 25000 + 500000 + 40000 + 100000 + 10000 = 675000.
assert tong == 675000, "năm dòng đọc được của cuốn sổ này là 25000, 500000, 40000, 100000 và 10000, cộng lại thành 675000 — dòng bánh mì ghi tiền bằng chữ nên không vào tổng"
# Chỗ trống thứ hai (nhánh vấp ValueError) bị soi bởi con số đếm: cuốn sổ này
# có đúng một dòng ghi tiền bằng chữ.
assert bo_qua == 1, "cuốn sổ này có đúng một dòng ghi tiền bằng chữ là 'bánh mì,mười lăm nghìn', nên bo_qua phải là 1"
# Hàm không chạm file nên gọi lại được với dữ liệu khác, ngay tại đây.
# Ba lời gọi dưới đây tách riêng ba tình huống: toàn dòng đọc được, toàn dòng
# hỏng, và không có dòng nào.
assert tong_va_bo_qua(["bún bò,40000\n"]) == (40000, 0), "một dòng duy nhất là 'bún bò,40000' thì tổng phải là 40000 và không có dòng nào bị bỏ qua"
assert tong_va_bo_qua(["bánh mì,mười lăm nghìn\n"]) == (0, 1), "một dòng duy nhất là 'bánh mì,mười lăm nghìn' thì không cộng được đồng nào nên tổng phải là 0, và đúng một dòng bị bỏ qua"
assert tong_va_bo_qua([]) == (0, 0), "không có dòng nào để đọc thì tổng phải là 0 và số dòng bỏ qua cũng phải là 0"
# Ca HAI dòng hỏng. Không có nó thì `bo_qua = 1` — gán thay vì tăng, đúng cái
# lỗi mà một bộ đếm sinh ra để chống — đậu sạch mọi câu kiểm phía trên, vì
# không cuốn sổ thử nào có quá một dòng hỏng.
assert tong_va_bo_qua(["bánh mì,mười lăm nghìn\n", "trà đá,năm nghìn\n", "bún bò,40000\n"]) == (40000, 2), "cuốn sổ thử này có HAI dòng ghi tiền bằng chữ, nên tổng chỉ còn 40000 và số dòng bỏ qua phải là 2 — một bộ đếm gán thẳng số 1 sẽ dừng ở 1"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm ở hai nhánh của cùng một câu `try`. Nhánh trên chạy khi dòng đọc được, và bên trái dấu bằng ở đó là `tong` — thứ đang được cộng dồn. Nhánh dưới chỉ chạy khi `int()` vừa nổ `ValueError`, và bên trái dấu bằng ở đó là `bo_qua`. Ở cả hai nhánh, `manh` đã được tách sẵn ở dòng trên.
- kind: strategy
  body: 'Nhánh trên: `manh` là danh sách hai mảnh, tên khoản đứng ô 0 và số tiền đứng ô 1 — mà ô ấy vẫn là chữ, nên phải đổi thành số trước khi cộng. Nhánh dưới: đây là một bộ đếm, và một bộ đếm tăng lên bằng cách lấy chính nó cộng thêm một.'
- kind: one-line
  body: 'Chỗ trống thứ nhất viết `int(manh[1])`, chỗ thứ hai viết `bo_qua + 1`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^Tổng đọc được: 675000 đồng\nBỏ qua 1 dòng hỏng\s*$
- tier: output
  expect: "Bỏ qua 1 dòng hỏng"
:::
::::

::::explain{#truoc-khi-ghep}
Một chuyện phải nói thẳng ra trước khi ghép: **số liệu không đổi**. Cuốn sổ ở
bước ghép vẫn đúng sáu dòng bạn vừa cộng — cà phê, sửa xe, bún bò, đổ xăng,
bánh mì ghi bằng chữ, gửi xe. Cái đổi là chỗ nó nằm: sáu dòng ấy rời khỏi file
code và xuống đĩa, còn chương trình thì nhận việc từ dòng lệnh.

Chương trình có bốn hàm, chia làm hai loại đúng theo luật bài 27:

- `tong_va_bo_qua(cac_dong)` — hàm bạn vừa viết xong, không chạm gì ngoài.
- `doc_so(duong_dan)` — chạm đĩa: mở sổ, đưa lại mọi dòng. Sổ chưa có thì đưa
  lại danh sách rỗng.
- `them_khoan(duong_dan, ten, tien)` — chạm đĩa: ghi nối một dòng vào cuối.
- `chay(argv)` — chạm dòng lệnh và màn hình: đọc việc cần làm, gọi đúng hàm.

Ba chỗ trống nằm ở ba hàm khác nhau:

- Trong `them_khoan`: **chế độ mở file**. Đây là chỗ bước đoán vừa dựng bẫy.
- Trong `chay`: **việc cần làm nằm ở ô nào của dòng lệnh**. Ô số 0 là tên
  chương trình, nên tham số đầu tiên không nằm ở đó.
- Trong `doc_so`: **tên loại lỗi cần bắt**. Sổ chưa tồn tại không phải lỗi của
  code bạn viết, và cũng không phải `ValueError` — nó là loại lỗi đến từ thế
  giới bên ngoài.

Còn một chuyện về `sys.argv`. Trên máy thật, bạn gõ `python so.py xem so.txt`
và hệ điều hành đặt sẵn ba chữ ấy vào `sys.argv` cho bạn. Màn hình này không có
dòng lệnh nào để gõ, nên chương trình tự đặt `sys.argv` bằng tay rồi gọi
`chay(sys.argv)` — đúng cái danh sách mà dòng lệnh sẽ trao, chỉ khác chỗ ai đặt
nó vào.
::::

::::assemble{#ghep-ca-cuon-so}
Bốn hàm, bốn dòng lệnh, một cuốn sổ nằm trên đĩa. Ba chỗ trống, ba câu hỏi khác
nhau.

```python title=starter
import sys
from pathlib import Path


def tong_va_bo_qua(cac_dong):
    """Cộng tiền các dòng đọc được, và đếm số dòng phải bỏ qua."""
    tong = 0
    bo_qua = 0
    for dong in cac_dong:
        manh = dong.strip().split(",")
        try:
            tong = tong + int(manh[1])
        except ValueError:
            bo_qua = bo_qua + 1
    return tong, bo_qua


def doc_so(duong_dan):
    """Chạm đĩa: đưa lại mọi dòng của sổ. Chưa có sổ thì coi như sổ rỗng."""
    try:
        with open(duong_dan, "r", encoding="utf-8") as f:
            return f.readlines()
    except ___:
        return []


def them_khoan(duong_dan, ten, tien):
    """Chạm đĩa: ghi NỐI một khoản vào cuối sổ, không đụng phần đã có."""
    with open(duong_dan, ___, encoding="utf-8") as f:
        f.write(f"{ten},{tien}\n")


def chay(argv):
    """Chạm dòng lệnh và màn hình: đọc việc cần làm rồi gọi đúng hàm."""
    viec = ___
    duong_dan = Path(argv[2])
    if viec == "xem":
        tong, bo_qua = tong_va_bo_qua(doc_so(duong_dan))
        print(f"Sổ {duong_dan}: tổng {tong} đồng, bỏ qua {bo_qua} dòng hỏng")
    elif viec == "them":
        them_khoan(duong_dan, argv[3], argv[4])
        print(f"Đã ghi {argv[3]} vào cuối sổ {duong_dan}")
    else:
        print(f"Không có lệnh con nào tên {viec}")


if __name__ == "__main__":
    # Dựng lại cuốn sổ trên đĩa để lần chạy nào cũng bắt đầu như nhau.
    with open("so.txt", "w", encoding="utf-8") as f:
        f.write("cà phê,25000\n")
        f.write("sửa xe,500000\n")
        f.write("bún bò,40000\n")
        f.write("đổ xăng,100000\n")
        f.write("bánh mì,mười lăm nghìn\n")
        f.write("gửi xe,10000\n")

    sys.argv = ["so.py", "xem", "so.txt"]
    chay(sys.argv)
    sys.argv = ["so.py", "them", "so.txt", "trà sữa", "45000"]
    chay(sys.argv)
    sys.argv = ["so.py", "xem", "so.txt"]
    chay(sys.argv)
    sys.argv = ["so.py", "xem", "so-thang-truoc.txt"]
    chay(sys.argv)
```

```python title=solution
import sys
from pathlib import Path


def tong_va_bo_qua(cac_dong):
    """Cộng tiền các dòng đọc được, và đếm số dòng phải bỏ qua."""
    tong = 0
    bo_qua = 0
    for dong in cac_dong:
        manh = dong.strip().split(",")
        try:
            tong = tong + int(manh[1])
        except ValueError:
            bo_qua = bo_qua + 1
    return tong, bo_qua


def doc_so(duong_dan):
    """Chạm đĩa: đưa lại mọi dòng của sổ. Chưa có sổ thì coi như sổ rỗng."""
    try:
        with open(duong_dan, "r", encoding="utf-8") as f:
            return f.readlines()
    except FileNotFoundError:
        return []


def them_khoan(duong_dan, ten, tien):
    """Chạm đĩa: ghi NỐI một khoản vào cuối sổ, không đụng phần đã có."""
    with open(duong_dan, "a", encoding="utf-8") as f:
        f.write(f"{ten},{tien}\n")


def chay(argv):
    """Chạm dòng lệnh và màn hình: đọc việc cần làm rồi gọi đúng hàm."""
    viec = argv[1]
    duong_dan = Path(argv[2])
    if viec == "xem":
        tong, bo_qua = tong_va_bo_qua(doc_so(duong_dan))
        print(f"Sổ {duong_dan}: tổng {tong} đồng, bỏ qua {bo_qua} dòng hỏng")
    elif viec == "them":
        them_khoan(duong_dan, argv[3], argv[4])
        print(f"Đã ghi {argv[3]} vào cuối sổ {duong_dan}")
    else:
        print(f"Không có lệnh con nào tên {viec}")


if __name__ == "__main__":
    # Dựng lại cuốn sổ trên đĩa để lần chạy nào cũng bắt đầu như nhau.
    with open("so.txt", "w", encoding="utf-8") as f:
        f.write("cà phê,25000\n")
        f.write("sửa xe,500000\n")
        f.write("bún bò,40000\n")
        f.write("đổ xăng,100000\n")
        f.write("bánh mì,mười lăm nghìn\n")
        f.write("gửi xe,10000\n")

    sys.argv = ["so.py", "xem", "so.txt"]
    chay(sys.argv)
    sys.argv = ["so.py", "them", "so.txt", "trà sữa", "45000"]
    chay(sys.argv)
    sys.argv = ["so.py", "xem", "so.txt"]
    chay(sys.argv)
    sys.argv = ["so.py", "xem", "so-thang-truoc.txt"]
    chay(sys.argv)
```

```python title=test
# Ba chỗ trống, ba nhóm câu kiểm. Mỗi nhóm vỡ nếu chỗ trống của nó điền sai.
#
# Chỗ 1 — chế độ mở file trong `them_khoan`. Chương trình vừa dựng sổ sáu dòng
# rồi chạy lệnh `them` đúng một lần. Ghi bằng "w" thì sổ chỉ còn một dòng, và
# cả ba câu dưới đây vỡ.
so_sau_khi_chay = doc_so("so.txt")
assert len(so_sau_khi_chay) == 7, "sổ so.txt được dựng ra với sáu dòng, rồi lệnh `them` ghi thêm khoản trà sữa, nên lúc này nó phải có đúng 7 dòng"
assert so_sau_khi_chay[0].strip() == "cà phê,25000", "dòng đầu của sổ so.txt vẫn phải là 'cà phê,25000' — lệnh `them` chỉ được ghi nối vào cuối, phần đã ghi không được đụng tới"
assert so_sau_khi_chay[6].strip() == "trà sữa,45000", "dòng thứ bảy của sổ so.txt phải là 'trà sữa,45000', khoản mà lệnh `them` vừa ghi vào"
#
# Chỗ 2 — việc cần làm lấy từ ô nào của dòng lệnh. Ô 0 là tên chương trình, ô 1
# mới là việc. Lấy nhầm ô 2 thì `chay` đọc ra "so.txt", không khớp nhánh nào,
# và sổ không dài thêm dòng nào.
chay(["so.py", "them", "so.txt", "kem", "20000"])
assert len(doc_so("so.txt")) == 8, "gọi chay(['so.py', 'them', 'so.txt', 'kem', '20000']) phải nhận ra việc cần làm là `them` — nó nằm ở ô số 1 — rồi ghi nối một dòng nữa, nên sổ so.txt lúc này phải có 8 dòng"
assert tong_va_bo_qua(doc_so("so.txt")) == (740000, 1), "sổ so.txt lúc này gồm 675000 đồng của sáu dòng dựng ban đầu, cộng 45000 của trà sữa và 20000 của kem, thành 740000 — và vẫn đúng một dòng bánh mì ghi tiền bằng chữ"
#
# Chỗ 3 — tên loại lỗi bắt trong `doc_so`. Bắt nhầm loại thì lỗi thật bay ra
# ngoài và câu dưới đây nổ ngay tại chỗ.
assert doc_so("so-thang-truoc.txt") == [], "trên đĩa chưa có file nào tên so-thang-truoc.txt, nên doc_so phải đưa lại danh sách rỗng thay vì để lỗi bay ra ngoài"
```

:::hints
- kind: attention
  body: Chỗ trống trong `doc_so` đứng ngay sau chữ `except`, nên thứ điền vào là một tên loại lỗi. Chỗ trống trong `them_khoan` nằm ở ô thứ hai của `open`, tức chỗ khai chế độ — bước đoán vừa cho bạn thấy chế độ nào dọn trắng cuốn sổ. Chỗ trống trong `chay` đứng bên phải dấu bằng của `viec`, và thứ duy nhất hàm ấy có trong tay là `argv`.
- kind: strategy
  body: 'Với `except`: đây là loại lỗi mà bài 13 gọi là lỗi đến từ thế giới bên ngoài — sổ chưa có trên đĩa — và tên nó ghép từ ba chữ tiếng Anh nghĩa là file, không tìm thấy, lỗi. Với chế độ mở file: chế độ ghi nối vào cuối là một chữ cái duy nhất viết trong dấu nháy, và nó không phải chữ w. Với `viec`: dòng ngay dưới đã lấy `argv[2]` làm đường dẫn, còn tên chương trình chiếm ô 0, nên việc cần làm chỉ còn đúng một ô để nằm.'
- kind: one-line
  body: 'Lần lượt ba chỗ trống là `FileNotFoundError`, `"a"`, và `argv[1]`.'
:::

:::validate
- tier: static
  onFail: chỗ trống trong `doc_so` phải nêu ĐÍCH DANH loại lỗi cần bắt — một câu `except` không tên sẽ nuốt luôn cả lỗi gõ nhầm của chính bạn, đúng cái bẫy bài 11 dựng ra
  requireAst:
  - kind: uses-name, target: FileNotFoundError, min: 1
- tier: run
  timeoutMs: 8000
- tier: tests
  timeoutMs: 8000
- tier: output
  expect: "Sổ so.txt: tổng 675000 đồng, bỏ qua 1 dòng hỏng"
- tier: output
  expect: "Sổ so.txt: tổng 720000 đồng, bỏ qua 1 dòng hỏng"
- tier: output
  expect: "Sổ so-thang-truoc.txt: tổng 0 đồng, bỏ qua 0 dòng hỏng"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tắt máy đi. Mai bật lên, cuốn sổ vẫn còn đó — và nó tự kể được nó đã bỏ qua
mấy dòng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi sang chặng sau.

Cuốn sổ giờ sống được qua đêm, nhận việc từ dòng lệnh, và hỏng thì soi được
bằng `breakpoint()`. Cả Realm 1 khép lại ở đây.

Nhưng sổ của bạn giờ 10.000 dòng. Hỏi "tháng ba tiêu bao nhiêu", chương trình
đọc từ dòng 1 tới dòng 10.000 — rồi lần hỏi sau lại quét lại từ đầu.

Có cách cất số liệu nào để **tìm** mà không phải xem hết không?
::::

::::checkpoint{mastery=0.85}
::::
