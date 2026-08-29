---
id: nen-tang.chuong-trinh-that.dung-chuong-trinh-lai-ma-hoi
title: Dừng chương trình lại mà hỏi
summary: "`breakpoint()` bắt chương trình đứng lại đúng tại dòng ấy rồi trao quyền hỏi cho bạn — gõ tên bất kỳ biến nào cũng được, không phải quyết định trước sẽ hỏi cái gì."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [dbg.breakpoint]
requires: [core.file-write, core.file-read, core.file-readlines, core.strip-newline, core.string-split, io.text-is-str, core.print-variable, core.name-lookup, err.name-error, core.variable, core.assignment, core.list, core.list-index, core.len, core.int-cast, core.string-strip, core.string-method, core.fstring, core.output, core.accumulator, ctrl.for-each]
concepts: [core.diem-dung, core.trang-thai, core.khoanh-khac]
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
Đừng đoán trước sẽ hỏi gì. Bảo chương trình đứng lại đã, rồi hỏi bao nhiêu tuỳ bạn.
::::

::::explain{#chin-dong-print-va-mot-dong-dung}
Chín dòng `print` rắc khắp hai file, màn hình ngập chữ, và sửa xong còn phải đi
xoá từng dòng — bài trước để lại đúng chỗ đau đó. Nhưng chỗ đau nặng nhất không
phải việc xoá. Nó nằm ở chỗ này:

**Một dòng `print` là một câu hỏi bạn phải quyết định TRƯỚC khi chạy.**

Bạn gõ `print(tong)` nghĩa là bạn đã đoán rằng `tong` chính là thứ đáng nghi.
Chạy xong, nhìn `tong` thấy nó bình thường, thế là câu hỏi ấy vứt đi — và muốn
xem thêm `cac_dong` thì phải mở code, gõ thêm một dòng, chạy lại từ đầu. Mỗi câu
hỏi một lượt chạy.

Python có một dòng lệnh làm việc ngược lại. Bạn không nói trước sẽ hỏi gì; bạn
chỉ nói **hỏi ở đâu**:

```python title=readonly
def bao_cao(ten_file):
    with open(ten_file, "r") as f:
        cac_dong = f.readlines()

    tong = 0
    breakpoint()          # ← đứng lại ngay đây
    for dong in cac_dong:
        manh = dong.strip().split(",")
        tong = tong + int(manh[1])
        # ... còn hơn ba mươi dòng nữa trong hàm này
```

`breakpoint()` là một **điểm dừng**. Chạy chương trình như bình thường, không
thêm cờ nào: nó chạy từ đầu, tới đúng dòng ấy thì **đứng lại giữa chừng** —
chưa chết, chưa xong, chỉ là đứng yên — và trao quyền cho bạn.

Màn hình lúc đó trông thế này:

```text title=readonly
$ python main.py so.txt
> /Users/lan/du-an/main.py(7)bao_cao()
-> for dong in cac_dong:
(Pdb) tong
0
(Pdb) len(cac_dong)
12
(Pdb) cac_dong[0]
'cà phê,25000\n'
(Pdb) ten_file
'so.txt'
(Pdb) q
```

Bốn chỗ đáng dừng lại nhìn:

- **`(Pdb)` là dấu nhắc.** Nó chờ bạn gõ, y như dấu nhắc của cửa sổ dòng lệnh mà
  bạn đã quen từ Realm 0. Gõ một cái tên rồi Enter, nó trả lời ngay bên dưới.
- **Bạn gõ được cái tên nào cũng được.** `tong`, `cac_dong`, `ten_file`, kể cả
  `len(cac_dong)` hay `cac_dong[0]` — những thứ bạn không hề nghĩ tới lúc viết
  code. Không phải sửa file, không phải chạy lại.
- **Câu trả lời là giá trị THẬT tại đúng khoảnh khắc ấy**, không phải giá trị bạn
  tưởng. `cac_dong[0]` trả về `'cà phê,25000\n'` — có cái `\n` dính đuôi, đúng
  thứ bài 7 đã cảnh báo.
- **Gõ `q` rồi Enter là thôi**, chương trình bỏ dở và bạn về lại dòng lệnh. Cái
  `q` ấy chỉ là cách đi ra; nó không phải điều bài này dạy.

Và điểm dừng để lại đúng một dòng trong code — một dòng bạn xoá là xong, chứ
không phải chín dòng rải hai file.
::::

::::predict{#doan-hoi-cai-chua-co commitOnce}
Byte thử đặt điểm dừng ở một chỗ khác: ngay dòng đầu, **trước** khi cuốn sổ được
dựng ra. Rồi tại điểm dừng, Byte gõ `cac_dong`.

**Trước khi bấm chạy**, bạn đoán màn hình trả lời gì?

```python title=readonly
ten_file = "so-ngan.txt"
breakpoint()
cac_dong = ["cà phê,25000", "bún bò,40000"]

tong = 0
for dong in cac_dong:
    tong = tong + int(dong.split(",")[1])
print(f"Tổng: {tong} đồng")
```

```text title=readonly
(Pdb) cac_dong
```

:::opt{correct}
`*** NameError: name 'cac_dong' is not defined`
:::

:::opt
`['cà phê,25000', 'bún bò,40000']`
::why
Gần đúng ở chỗ bạn đọc code rất kỹ: cái danh sách ấy nằm ngay dòng dưới, chữ nào
cũng thấy rõ, và nó đúng là thứ `cac_dong` sẽ mang.

Chỗ lệch nằm ở chữ **sẽ**. Điểm dừng không đọc mã nguồn của bạn; nó đọc **trí
nhớ của chương trình tại đúng khoảnh khắc nó đứng lại**. Khoảnh khắc ấy, dòng
gán kia còn chưa chạy, nên trong trí nhớ chưa có gì mang tên `cac_dong` cả. Dời
điểm dừng xuống dưới dòng ấy một nấc thì câu trả lời của bạn thành đúng.
::
:::

:::opt
`[]` — một danh sách rỗng
::why
Gần đúng ở chỗ bạn suy ra được rằng `cac_dong` rồi sẽ là một danh sách, và một
danh sách chưa có gì trong đó thì đúng là `[]`.

Chỗ lệch: Python không dựng sẵn cái tên nào trước cả. Một cái tên chỉ ra đời vào
đúng lúc có một dòng trao giá trị cho nó — đó là điều `NameError` ở Realm 0 nói,
và ở đây vẫn là nó. "Chưa có tên" khác hẳn "có tên, đang rỗng".
::
:::

:::opt
Chương trình chết ngay tại đó và thoát ra dòng lệnh
::why
Gần đúng ở chỗ bạn nhớ đúng hành vi của `NameError` ở Realm 0: gặp một cái tên
chưa ai đặt, chương trình dừng lại và không đi tiếp được nữa.

Chỗ lệch là ở chỗ *ai* gõ dòng ấy. Ở Realm 0, cái tên sai nằm trong **chương
trình**, nên chương trình hỏng. Ở đây bạn gõ nó tại dấu nhắc `(Pdb)` — đó là một
**câu hỏi**, không phải một dòng của chương trình. Điểm dừng trả lời rằng nó
không tìm thấy cái tên ấy, rồi vẫn đứng nguyên chờ bạn hỏi tiếp.
::
:::
::::

::::explain{#diem-dung-la-mot-khoanh-khac}
Đoán xong rồi thì phát biểu lại cho gọn, vì đây là điều dễ trượt nhất về điểm
dừng:

**Điểm dừng cho bạn một KHOẢNH KHẮC, không cho bạn cả chương trình.**

Tại khoảnh khắc ấy, mọi dòng phía trên đã chạy xong và mọi dòng phía dưới thì
chưa. Nên câu trả lời bạn nhận được luôn là "lúc này, ở đây, cái tên này đang
mang gì" — chứ không phải "cuối cùng nó sẽ mang gì".

Ba hệ quả dùng được ngay:

- Đặt điểm dừng ở đâu là **chọn xem khoảnh khắc nào**. Đặt trước vòng lặp thì
  thấy trạng thái lúc chưa cộng đồng nào; đặt sau vòng lặp thì thấy kết quả đã
  cộng xong.
- Hỏi một cái tên mà những dòng phía trên chưa hề tạo ra thì nhận về lời báo
  không tìm thấy tên — và đó cũng là một câu trả lời có ích, nó nói cho bạn biết
  dòng gán kia nằm dưới điểm dừng.
- Hỏi bao nhiêu câu cũng được trong cùng một khoảnh khắc ấy, vì chương trình
  đang đứng yên chờ. Đó chính là thứ chín dòng `print` không mua nổi.
::::

::::code{#ghi-lai-cau-tra-loi-cua-diem-dung}
Byte đưa bạn một chương trình ngắn: ghi ba khoản xuống đĩa, đọc lên, rồi cộng
tiền. Điểm dừng đặt ngay **trước** vòng lặp cộng tiền — chỗ đánh dấu bằng câu
chú thích in hoa.

Tại điểm dừng ấy, Byte gõ ba cái tên: `tong`, `len(cac_dong)`, và `ten_file`.
Việc của bạn là điền đúng ba câu trả lời mà điểm dừng sẽ in ra.

Cuốn sổ ở đây tên `so-ngan.txt` và chỉ có ba khoản — bản rút ngắn để bạn đi bộ
bằng mắt được. Cuốn sổ thật của Byte là `so.txt` với 12 khoản, và bạn sẽ
quay lại nó ở cuối bài.

```python title=starter
ten_file = "so-ngan.txt"

with open(ten_file, "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("gửi xe,10000\n")

with open(ten_file, "r") as f:
    cac_dong = f.readlines()

tong = 0
# ĐIỂM DỪNG — chương trình đứng lại đúng ở đây, ngay trước vòng lặp cộng tiền.
for dong in cac_dong:
    manh = dong.strip().split(",")
    tong = tong + int(manh[1])

dap_tong = ___
dap_so_dong = ___
dap_ten_file = ___

print(f"Ở điểm dừng, tong là {dap_tong}")
print(f"Ở điểm dừng, len(cac_dong) là {dap_so_dong}")
print(f"Ở điểm dừng, ten_file là {dap_ten_file}")
print(f"Chạy hết chương trình thì tổng là {tong} đồng")
```

```python title=solution
ten_file = "so-ngan.txt"

with open(ten_file, "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("gửi xe,10000\n")

with open(ten_file, "r") as f:
    cac_dong = f.readlines()

tong = 0
# ĐIỂM DỪNG — chương trình đứng lại đúng ở đây, ngay trước vòng lặp cộng tiền.
for dong in cac_dong:
    manh = dong.strip().split(",")
    tong = tong + int(manh[1])

dap_tong = 0
dap_so_dong = 3
dap_ten_file = "so-ngan.txt"

print(f"Ở điểm dừng, tong là {dap_tong}")
print(f"Ở điểm dừng, len(cac_dong) là {dap_so_dong}")
print(f"Ở điểm dừng, ten_file là {dap_ten_file}")
print(f"Chạy hết chương trình thì tổng là {tong} đồng")
```

```python title=test
# Ba câu trả lời được đối chiếu với trạng thái THẬT tại đúng khoảnh khắc điểm
# dừng, tính lại từ chính cuốn sổ vừa ghi ra đĩa — không đối chiếu với một con
# số gõ cứng trong đầu bài.
with open(ten_file, "r") as f:
    that_su = f.readlines()

assert dap_tong == 0, "điểm dừng đứng ngay sau dòng `tong = 0` và ngay trước vòng lặp cộng tiền, nên tại đúng khoảnh khắc ấy `tong` mang số 0 — chưa khoản nào được cộng vào"
assert dap_so_dong == len(that_su), "cuốn sổ trong bài này vừa được ghi ba dòng xuống đĩa, nên tại điểm dừng `len(cac_dong)` trả về 3"
assert dap_ten_file == ten_file, "dòng đầu chương trình đặt `ten_file` bằng tên so-ngan.txt và không dòng nào sau đó đổi nó, nên tại điểm dừng câu trả lời vẫn là chuỗi so-ngan.txt"
assert tong == 75000, "chạy hết chương trình, ba khoản 25000, 40000 và 10000 gộp lại cho `tong` bằng 75000 — con số này khác đi nghĩa là vòng lặp cộng tiền đã bị sửa"
```

:::hints
- kind: attention
  body: Đọc từ đầu chương trình xuống tới dòng chú thích in hoa, và dừng đúng ở đó. Mọi dòng phía trên đã chạy xong; vòng lặp phía dưới thì chưa chạy dòng nào. Ba câu trả lời phải nói về đúng khoảnh khắc ấy.
- kind: strategy
  body: 'Với `tong`: dòng gán nó nằm ngay trên điểm dừng, còn dòng cộng thêm tiền nằm dưới — hỏi xem lúc ấy nó vừa được trao con số nào. Với `len(cac_dong)`: đếm số dòng chương trình vừa ghi xuống file, vì đó cũng là số phần tử `readlines` đưa về. Với `ten_file`: cái tên ấy được trao giá trị ở dòng đầu tiên, và không dòng nào đổi nó.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `0`, `3`, và `"so-ngan.txt"`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^Ở điểm dừng, tong là 0\nỞ điểm dừng, len\(cac_dong\) là 3\nỞ điểm dừng, ten_file là so-ngan\.txt\nChạy hết chương trình thì tổng là 75000 đồng\s*$
- tier: output
  expect: "Chạy hết chương trình thì tổng là 75000 đồng"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dòng đặt vào, hỏi bao nhiêu câu cũng được. Không phải xoá chín dòng nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Quay lại cuốn sổ 12 khoản trên đĩa. Bạn đặt điểm dừng ngay trước vòng lặp cộng
tiền của `bao_cao()` rồi hỏi. Tại điểm dừng, `tong` đang là `0` và `cac_dong` đủ
12 phần tử — cả hai đều đúng như bạn nghĩ.

Nghĩa là chỗ hỏng nằm ở những dòng **sau** điểm dừng.

Đặt thêm điểm dừng ở từng dòng à?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
