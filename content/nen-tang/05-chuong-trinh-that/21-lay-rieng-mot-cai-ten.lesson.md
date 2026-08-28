---
id: nen-tang.chuong-trinh-that.lay-rieng-mot-cai-ten
title: Lấy riêng một cái tên ra
summary: "`from so_sach import doc_so` mang riêng một cái tên từ trong hộp vào chương trình, và từ đó gọi nó trần — không tiền tố, không dấu chấm."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.import-from]
requires: [core.own-module, core.import-module, mod.import, mod.dotted-access, core.with-open, io.readlines, core.strip-newline, core.string-split, core.string-strip, io.text-is-str, core.int-cast, core.function-def, core.function-return, core.function-parameter, core.docstring, core.tuple, core.for-unpack, core.list-append, core.list-comprehension, core.comprehension-filter, core.fstring, ctrl.for-each]
concepts: [core.mo-dun, core.tien-to, core.ten]
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
Bạn mượn cả cái hộp về, rồi mở ra lấy đúng hai món. Lấy thẳng hai món ấy có
được không?
::::

::::explain{#tien-to-lap-hai-chuc-lan}
Cái hộp `so_sach` đã cứu bạn khỏi việc chép hàm sang hai file. Nhưng nó để lại
một chuyện nhỏ mà đọc mãi thì mỏi: mỗi lần dùng, cái tên hộp lại đứng chắn phía
trước.

```python title=readonly
khoan = so_sach.doc_so(ten_file)
tong = so_sach.tong_tien(khoan)
con_lai = so_sach.doc_so(ten_khac)
tong_khac = so_sach.tong_tien(con_lai)
```

Trong `so.py` với ba lệnh con, hình dạng ấy lặp lại hai chục lần. Và điều đáng
để ý là: suốt hai chục lần đó, cả file chỉ với tới đúng **hai** cái tên bên
trong hộp — `doc_so` và `tong_tien`. Cái hộp không mang lại thông tin gì mới ở
lần thứ hai mươi; nó chỉ dài thêm ra.

Python có một dạng mượn thứ hai, dành đúng cho chuyện này:

```python title=readonly
from so_sach import doc_so
```

Đọc từ trái sang phải, nó là một câu tiếng Việt gần như nguyên vẹn: *từ hộp
`so_sach`, mang cái tên `doc_so` vào đây.* Sau dòng ấy, `doc_so` là một cái tên
của chương trình bạn, đứng ngang hàng với mọi hàm bạn tự viết trong file:

```python title=readonly
khoan = doc_so(ten_file)
```

Cần hai cái tên thì kể tên cả hai, cách nhau bởi dấu phẩy:

```python title=readonly
from so_sach import doc_so, tong_tien
```

Có một chỗ dễ vấp, và nó cũng chính là chỗ phân biệt hai dạng mượn. `from ...
import ...` mang về đúng những cái tên bạn kể ra, **và không mang về cái tên
`so_sach`**. Sau dòng trên, `so_sach.khoan_tren(...)` sẽ hỏng — không phải vì
hàm ấy biến mất khỏi hộp, mà vì trong chương trình của bạn không có cái tên
`so_sach` nào để đặt trước dấu chấm.

Nói gọn lại, hai dạng mượn khác nhau ở chỗ **cái gì được đặt tên trong chương
trình của bạn**:

- `import so_sach` — một cái tên mới: `so_sach`, trỏ tới cả hộp. Mọi thứ bên
  trong với tới qua dấu chấm.
- `from so_sach import doc_so, tong_tien` — hai cái tên mới: `doc_so` và
  `tong_tien`, gọi trần được ngay.

Cả hai đều dùng được, và cái giá phải trả cũng khác nhau. Với dạng thứ hai, đọc
tới dòng `khoan = doc_so(...)` ở giữa file, bạn không còn nhìn thấy hàm ấy đến
từ đâu — muốn biết thì phải lên đầu file đọc dòng `from`. Nên nó hợp với những
cái tên bạn dùng đi dùng lại và đã thuộc nằm lòng.
::::

::::example{#lay-hai-ten-ra}
Vẫn cái hộp của bài trước, không sửa một chữ. Chỉ dòng mượn là đổi.

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

# Dòng này là việc riêng của trang học: nó dọn chỗ để lần mượn ngay dưới đọc
# đúng file vừa ghi. Trên máy của bạn không cần nó.
sys.modules.pop("so_sach", None)

from so_sach import doc_so, tong_tien

khoan = doc_so("so-thang-8.txt")
print(khoan)
print(tong_tien(khoan))
```

Máy in ra:

```text title=readonly
[('cà phê', 25000), ('bún bò', 40000), ('trà sữa', 45000)]
110000
```

Cùng cuốn sổ ba khoản của bài trước, cùng hai con số ấy. Thứ đổi là hình dạng
lời gọi, không phải kết quả.

Ba chỗ đáng dừng lại nhìn:

- **`doc_so` giờ đứng một mình.** Nhìn dòng `khoan = doc_so("so-thang-8.txt")`
  thì không có gì phân biệt nó với một hàm được `def` ngay trong file này. Đó
  đúng là điều dòng `from` vừa làm: nó đặt cái tên ấy vào chương trình của bạn.
- **Dấu phẩy kể thêm tên, không kể thêm hộp.** Cả hai cái tên trong dòng
  `from` đều đến từ một hộp duy nhất, tên hộp viết đúng một lần ở giữa.
- **`khoan_tren` không có mặt.** Nó vẫn nằm nguyên trong `so_sach.py`, nhưng
  dòng `from` không kể tên nó, nên chương trình này không có cái tên ấy. Gọi
  `khoan_tren(...)` ở đây là một `NameError` — và cách chữa là kể thêm tên nó
  vào cuối dòng `from`.
::::

::::predict{#doan-ten-khong-duoc-ke commitOnce}
Byte mượn riêng `doc_so`, rồi quen tay gọi luôn `tong_tien` ở dòng dưới.

Giả sử `so_sach.py` đã nằm sẵn cạnh chương trình, đúng nội dung ở trên, và
`so-thang-8.txt` có ba khoản.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
from so_sach import doc_so

khoan = doc_so("so-thang-8.txt")
print(len(khoan))
print(tong_tien(khoan))
```

:::opt{correct}
In `3`, rồi chương trình dừng vì `NameError: name 'tong_tien' is not defined`
:::

:::opt
In `3`, rồi in `110000`
::why
Gần đúng ở chỗ bạn đọc đúng ý định của chương trình, và cũng đúng về cái hộp:
`tong_tien` nằm sẵn trong `so_sach.py`, cộng ba khoản ấy đúng ra 110000 thật.

Chỗ lệch là dòng `from` mang về **những cái tên nó kể ra**, chứ không mở cả
hộp. Ở đây nó kể đúng một tên là `doc_so`. `tong_tien` vẫn ở trong hộp và
không bước ra, nên chương trình của bạn không có cái tên ấy. Kể thêm nó — viết
`from so_sach import doc_so, tong_tien` — là chạy được ngay.
::
:::

:::opt
Chương trình dừng ngay tại dòng `from`, vì phải `import so_sach` trước đã
::why
Gần đúng ở chỗ bạn đang cẩn thận đúng chỗ: hai dạng mượn quả thật có quan hệ
với nhau, và cả hai cùng đi tìm một file `so_sach.py`.

Chỗ lệch là chúng không xếp chồng lên nhau. `from so_sach import doc_so` tự nó
đã làm trọn việc đi tìm file, đọc nó lên, rồi lấy ra cái tên được kể. Nó không
đòi một dòng `import so_sach` đứng trước. Hai dạng chỉ khác nhau ở chỗ cuối
cùng cái tên nào được đặt vào chương trình của bạn.
::
:::

:::opt
In `3`, rồi chương trình dừng vì `AttributeError`
::why
Gần đúng ở chỗ bạn nhớ rằng `tong_tien` là thứ nằm *bên trong* một cái hộp, và
lỗi khi với vào bên trong một cái hộp thường mang tên `AttributeError` thật.

Chỗ lệch là ở hình dạng của dòng code này. `tong_tien(khoan)` viết trần, không
có dấu chấm nào, không có hộp nào đứng trước. Nên máy không đi tìm bên trong
thứ gì cả — nó làm đúng việc Realm 0 đã tả: gặp một cái tên không có nháy thì
đi tìm cái tên ấy trong chương trình. Tìm không ra thì `NameError`.
::
:::
::::

::::code{#bao-cao-goi-tran}
Vẫn là `bao_cao.py` của bài trước, làm đúng việc ấy, in ra đúng bốn dòng ấy.
Lần này nó mượn riêng hai cái tên và gọi chúng trần.

Ba chỗ trống: một chỗ kể tên những thứ cần mượn, hai chỗ gọi chúng.

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

from so_sach import ___

khoan = ___("so-thang-8.txt")
tong = ___(khoan)

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

from so_sach import doc_so, tong_tien

khoan = doc_so("so-thang-8.txt")
tong = tong_tien(khoan)

dong_bao_cao = []
for ten, tien in khoan:
    dong_bao_cao.append(f"{ten}: {tien} đồng")
dong_bao_cao.append(f"Tổng: {tong} đồng")

for dong in dong_bao_cao:
    print(dong)
```

```python title=test
# Chỗ trống thứ hai (`khoan = ___(...)`): chỉ hàm đọc sổ mới dựng ra được ba
# cặp tên–tiền này từ file.
assert khoan == [("cà phê", 25000), ("bún bò", 40000), ("trà sữa", 45000)], "doc_so phải đọc file so-thang-8.txt thành ba khoản: cà phê 25000, bún bò 40000, trà sữa 45000"
# Chỗ trống thứ ba (`tong = ___(khoan)`): gọi nhầm hàm thì con số này lệch —
# ví dụ gọi lại doc_so ở đây sẽ dừng ngay vì nó đòi một tên file.
assert tong == 110000, "ba khoản 25000, 40000 và 45000 cộng lại là 110000"
assert dong_bao_cao == [
    "cà phê: 25000 đồng",
    "bún bò: 40000 đồng",
    "trà sữa: 45000 đồng",
    "Tổng: 110000 đồng",
], "bản báo cáo phải có bốn dòng: ba khoản theo đúng thứ tự ghi trong sổ, rồi dòng tổng 110000"
# Chỗ trống thứ nhất (`from so_sach import ___`): kể thiếu một trong hai tên
# thì lời gọi tương ứng ở trên đã dừng vì NameError, và không câu nào dưới đây
# chạy tới. Còn chuyện gọi trần chứ không gọi qua tiền tố thì luật `static` của
# bước này canh.
assert doc_so("so-thang-8.txt") == khoan, "cái tên doc_so trong chương trình này phải là hàm đọc sổ mượn từ hộp so_sach, gọi lại nó trên cùng file phải cho lại đúng ba khoản ấy"
assert tong_tien([("a", 1000), ("b", 2000)]) == 3000, "cái tên tong_tien trong chương trình này phải là hàm cộng tiền mượn từ hộp so_sach: hai khoản 1000 và 2000 phải cho 3000"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm ngay sau chữ `import` trong một dòng đã có sẵn chữ `from so_sach`. Hai chỗ trống còn lại nằm ngay trước cặp ngoặc gọi hàm, tức là chỗ đáng ra phải có một cái tên hàm. Đọc hai dòng dưới cùng để biết chương trình cần đúng những việc gì.
- kind: strategy
  body: Chương trình này cần hai việc từ trong hộp — một việc đọc file sổ ra thành danh sách khoản, một việc cộng tiền của danh sách ấy. Kể tên cả hai vào dòng `from`, cách nhau bởi dấu phẩy. Rồi ở hai lời gọi bên dưới, viết thẳng tên hàm, đừng đặt tên hộp và dấu chấm phía trước — sau dòng `from` thì cái tên hộp không có mặt trong chương trình này.
- kind: one-line
  body: Dòng mượn là `from so_sach import doc_so, tong_tien`; hai chỗ trống còn lại lần lượt là `doc_so` và `tong_tien`.
:::

:::validate
- tier: static
  requireAst:
  - kind: uses-call, target: doc_so, min: 1
  - kind: uses-call, target: tong_tien, min: 1
  forbidAst:
  - kind: uses-name, target: so_sach
  onFail: bài này gọi trần hai cái tên đã mượn riêng — không đặt `so_sach.` phía trước
- tier: run
  timeoutMs: 8000
- tier: tests
  timeoutMs: 8000
- tier: output
  expect: "bún bò: 40000 đồng"
- tier: output
  expect: "Tổng: 110000 đồng"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cái tên, gọi trần. Bản báo cáo đọc y như thể bạn tự viết cả hai hàm ngay
tại đây.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cái hộp giờ cho mượn cả hộp lẫn từng món, và bạn chọn được kiểu mượn nào hợp
với từng file.

Nhưng lúc dựng cuốn sổ này, có một chuyện xảy ra mà chưa ai giải thích cho bạn.

Bạn thêm dòng `print("đang mở sổ...")` ở lề trái `so_sach.py` để thử — lề
trái, tức là không nằm trong hàm nào cả. Rồi bạn chạy `main.py`, mà `main.py`
mới có đúng dòng mượn ở đầu, chưa gọi hàm nào của hộp.

Câu `đang mở sổ...` đã hiện lên màn hình.

Bạn không gọi gì cả. Vì sao nó chạy?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
