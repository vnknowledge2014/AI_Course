---
id: nen-tang.chuong-trinh-that.tach-phan-tinh-khoi-phan-cham-file
title: Tách phần tính khỏi phần chạm file
summary: "Một hàm hoặc chạm thế giới bên ngoài — đọc ghi file, in ra màn hình — hoặc chỉ nhận vào và trả ra; để một hàm làm cả hai thì con bọ có chỗ trốn và không ai kiểm nổi nó."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [core.io-boundary]
requires: [dbg.step-line, core.file-write, core.file-read, core.file-readlines, core.strip-newline, core.string-split, io.text-is-str, core.function-def, core.function-call, core.function-parameter, core.function-argument, core.function-return, core.docstring, core.pure-function, core.side-effect, core.implicit-return-none, core.none, core.one-job-name, core.max-tracker, core.list, core.list-comprehension, core.string-strip, core.string-method, core.int-cast, core.fstring, core.output, core.accumulator, ctrl.for-each, ctrl.if]
concepts: [core.ranh-gioi, core.ham-thuan, core.cham-ben-ngoai]
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
Bốn việc trong một hàm thì con bọ có bốn chỗ để trốn. Chia ra, nó hết chỗ.
::::

::::explain{#bon-viec-trong-mot-cai-ham}
Con bọ đã bắt được rồi, nhưng câu hỏi cuối bài trước còn để ngỏ: **vì sao mắt
không thấy nó?**

Nhìn lại `bao_cao()`. Một hàm, bốn việc khác hẳn nhau:

```python title=readonly
def bao_cao(ten_file):
    with open(ten_file, "r") as f:      # việc 1 — chạm đĩa
        cac_dong = f.readlines()

    tong = 0
    for dong in cac_dong:
        manh = dong.strip().split(",")  # việc 2 — cắt gọt chữ
        tong = tong + int(manh[1])      # việc 3 — cộng tiền
        tong = 0

    print(f"Tổng: {tong} đồng")         # việc 4 — chạm màn hình
```

Bốn việc trộn vào nhau kéo theo hai chuyện, và cả hai đều làm con bọ sống lâu:

- **Mắt không có chỗ nghỉ.** Đọc từ trên xuống, bạn đang nghĩ về file thì phải
  đổi sang nghĩ về chuỗi, rồi về số. Dòng `tong = 0` lạc vào giữa lúc bạn đang
  bận đổi ý nghĩ, và nó trông y như mọi dòng gán khác quanh đó.
- **Không có cách nào thử riêng phần cộng tiền.** Muốn biết phép cộng có đúng
  không, bạn phải có một file trên đĩa, đúng tên, đúng chỗ, đúng nội dung. Thử
  một lần là dựng cả cái sân khấu ấy một lần.

Cách chữa là một luật xếp việc:

> **Một hàm HOẶC chạm thế giới bên ngoài, HOẶC chỉ nhận vào — trả ra.**

"Chạm thế giới bên ngoài" nghĩa là đọc hoặc ghi file, in ra màn hình, đọc thứ mà
dòng lệnh trao cho chương trình. Đó là những việc để lại dấu vết ngoài chương
trình, hoặc lấy vào một thứ chương trình không tự sinh ra được.

"Chỉ nhận vào — trả ra" thì đúng là cái hàm thuần khiết bạn viết suốt T1.3: đưa
cho nó giá trị, nó đưa lại giá trị, không đụng gì khác.

Tách `bao_cao()` theo luật ấy thì được ba hàm, mỗi hàm một loại:

- `doc_dong(ten_file)` — **chạm đĩa**, và chỉ làm mỗi việc đó: mở file, đưa ra
  danh sách dòng đã cắt sạch.
- `tinh_tong(cac_dong)` — **không chạm gì cả**: nhận danh sách dòng, trả ra một
  con số.
- `bao_cao(ten_file)` — hàm **ghép**. Nó vẫn chạm đĩa và chạm màn hình, nhưng nó
  không tự tính lấy đồng nào; nó gọi hai hàm kia. Nên nó ngắn, và ngắn thì nhìn
  hết được trong một lần.
::::

::::example{#ba-ham-thay-cho-mot}
Đây là cả ba hàm, cùng con bọ đã sửa: dòng `tong = 0` về đúng chỗ của nó, phía
trên vòng lặp.

```python title=readonly
def doc_dong(ten_file):
    """Chạm đĩa: mở file, đưa ra danh sách dòng đã cắt sạch hai đầu."""
    with open(ten_file, "r") as f:
        cac_dong = f.readlines()
    return [dong.strip() for dong in cac_dong]


def tinh_tong(cac_dong):
    """Không chạm gì bên ngoài: nhận danh sách dòng, đưa ra tổng tiền."""
    tong = 0
    for dong in cac_dong:
        manh = dong.split(",")
        tong = tong + int(manh[1])
    return tong


def bao_cao(ten_file):
    """Chạm đĩa rồi chạm màn hình: ghép hai hàm trên lại và in ra."""
    print(f"Tổng: {tinh_tong(doc_dong(ten_file))} đồng")


with open("so-ngan.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("gửi xe,10000\n")

bao_cao("so-ngan.txt")
print(tinh_tong(["vở ghi,15000", "trà sữa,45000"]))
```

Máy in ra:

```text title=readonly
Tổng: 75000 đồng
60000
```

Ba chỗ đáng dừng lại nhìn:

- **Dòng cuối cùng gọi `tinh_tong` mà không có file nào.** Đó là chỗ luật vừa
  đổi lấy được: phần cộng tiền giờ nhận một danh sách dòng, và danh sách ấy gõ
  thẳng vào cũng được, đọc từ đĩa lên cũng được. Nó không quan tâm chữ ấy từ
  đâu tới.
- **`tinh_tong` không in một chữ nào.** Nó `return`. Muốn thấy con số thì chỗ
  gọi tự in lấy, như dòng cuối đang làm.
- **`bao_cao` còn đúng một dòng thân.** Con bọ `tong = 0` mà bài trước phải gõ
  `n` ba nhát mới bắt được, ở hình dạng này thì không có chỗ nào để nấp: hàm nào
  giữ `tong` cũng chỉ dài năm dòng.

Cuốn sổ dùng ở đây tên `so-ngan.txt` và chỉ có ba khoản — vẫn là bản rút ngắn
của hai bài trước, để bạn dò bằng mắt được. Cuốn `thang-chin.txt` 12 khoản của
Byte cho ra một con số khác hẳn, nên đừng mang tổng của cuốn này sang cuốn kia.
::::

::::predict{#doan-in-ra-khong-phai-tra-ve commitOnce}
Byte tách hàm, nhưng để sót một dòng `print` bên trong `tinh_tong` — và bỏ luôn
dòng `return`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
def tinh_tong(cac_dong):
    tong = 0
    for dong in cac_dong:
        tong = tong + int(dong.split(",")[1])
    print(f"Tổng: {tong} đồng")


kq = tinh_tong(["cà phê,25000", "bún bò,40000"])
print(kq)
```

:::opt{correct}
`Tổng: 65000 đồng` rồi `None`
:::

:::opt
`Tổng: 65000 đồng` rồi `65000`
::why
Gần đúng ở chỗ bạn cộng đúng và theo dõi đúng: hàm ấy thật sự tính ra 65000, và
con số ấy có tồn tại thật bên trong hàm.

Chỗ lệch là giữa **cho xem** và **trao lại**. `print` đặt chữ lên màn hình rồi
thôi; `return` mới là câu trao giá trị về cho chỗ đã gọi. Hàm này không có dòng
`return` nào, nên con số 65000 chết theo hàm lúc hàm kết thúc, và `kq` nhận về
thứ mà T1.3 đã gọi tên: `None` — dấu hiệu của một hàm không trả gì.
::
:::

:::opt
Chỉ một dòng `Tổng: 65000 đồng`
::why
Gần đúng ở chỗ bạn suy ra được rằng `kq` chẳng cầm gì có giá trị, và một thứ
rỗng thì in ra cũng như không.

Chỗ lệch: `None` không phải chỗ trống, nó là một giá trị hẳn hoi mang nghĩa
"không có gì" — bạn gặp nó ở T1.3 khi một hàm thiếu `return`. Đưa một giá trị
cho `print` thì `print` viết nó ra, nên dòng thứ hai vẫn hiện, với đúng chữ
`None`.
::
:::

:::opt
`Tổng: 65000 đồng` hiện ra hai lần
::why
Gần đúng ở chỗ bạn đếm đúng số lần có lệnh in: một lần bên trong hàm, một lần ở
dòng cuối. Hai lệnh in thì đúng là hai dòng chữ.

Chỗ lệch là ở **thứ** mà lệnh in thứ hai đang cầm. `kq` không giữ lại câu chữ mà
hàm đã hiện lên màn hình — chữ đã hiện thì hiện rồi, không ai cất nó lại. `kq`
giữ thứ hàm **trao về**, mà hàm này không trao gì, nên dòng thứ hai in ra `None`.
::
:::
::::

::::code{#tach-lam-ba}
Byte có một hàm nữa cùng bệnh: nó vừa mở file, vừa cắt chữ, vừa dò xem khoản nào
tốn nhất, vừa in kết quả.

```python title=readonly
def khoan_ton_nhat_cu(ten_file):
    with open(ten_file, "r") as f:
        cac_dong = f.readlines()
    lon_nhat = 0
    ten_lon_nhat = ""
    for dong in cac_dong:
        manh = dong.strip().split(",")
        if int(manh[1]) > lon_nhat:
            lon_nhat = int(manh[1])
            ten_lon_nhat = manh[0]
    print(f"Khoản tốn nhất: {ten_lon_nhat}")
```

Bên dưới là nó đã được tách làm ba theo đúng luật. Hai chỗ trống nằm ở hai bên
của cùng một đường ranh: một chỗ ở cuối hàm **chạm đĩa**, một chỗ ở cuối hàm
**không chạm gì**.

```python title=starter
def doc_dong(ten_file):
    """Chạm đĩa: mở file, đưa ra danh sách dòng đã cắt sạch hai đầu."""
    with open(ten_file, "r") as f:
        cac_dong = f.readlines()
    return [___ for dong in cac_dong]


def khoan_ton_nhat(cac_dong):
    """Không chạm gì bên ngoài: nhận danh sách dòng, đưa ra tên khoản tốn nhất."""
    lon_nhat = 0
    ten_lon_nhat = ""
    for dong in cac_dong:
        manh = dong.split(",")
        if int(manh[1]) > lon_nhat:
            lon_nhat = int(manh[1])
            ten_lon_nhat = manh[0]
    ___


def bao_cao(ten_file):
    """Chạm đĩa rồi chạm màn hình: ghép hai hàm trên lại và in ra."""
    print(f"Khoản tốn nhất: {khoan_ton_nhat(doc_dong(ten_file))}")


with open("so-ngan.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("gửi xe,10000\n")

bao_cao("so-ngan.txt")
```

```python title=solution
def doc_dong(ten_file):
    """Chạm đĩa: mở file, đưa ra danh sách dòng đã cắt sạch hai đầu."""
    with open(ten_file, "r") as f:
        cac_dong = f.readlines()
    return [dong.strip() for dong in cac_dong]


def khoan_ton_nhat(cac_dong):
    """Không chạm gì bên ngoài: nhận danh sách dòng, đưa ra tên khoản tốn nhất."""
    lon_nhat = 0
    ten_lon_nhat = ""
    for dong in cac_dong:
        manh = dong.split(",")
        if int(manh[1]) > lon_nhat:
            lon_nhat = int(manh[1])
            ten_lon_nhat = manh[0]
    return ten_lon_nhat


def bao_cao(ten_file):
    """Chạm đĩa rồi chạm màn hình: ghép hai hàm trên lại và in ra."""
    print(f"Khoản tốn nhất: {khoan_ton_nhat(doc_dong(ten_file))}")


with open("so-ngan.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("gửi xe,10000\n")

bao_cao("so-ngan.txt")
```

```python title=test
# Chỗ trống thứ nhất — cuối hàm CHẠM ĐĨA. Ba dòng đọc lên từ file đều còn dính
# ký tự xuống dòng ở đuôi; hàm này phải cắt sạch trước khi trao ra.
assert doc_dong("so-ngan.txt") == ["cà phê,25000", "bún bò,40000", "gửi xe,10000"], "doc_dong phải trả ra ba dòng đã cắt sạch hai đầu: cà phê,25000 rồi bún bò,40000 rồi gửi xe,10000 — còn dính ký tự xuống dòng ở đuôi là chưa cắt"
# Chỗ trống thứ hai — cuối hàm KHÔNG CHẠM GÌ. Ba lời gọi dưới đây không mở file
# nào cả, và đó chính là thứ việc tách vừa đổi lấy được. Cả ba đều so với một
# CHUỖI, nên một hàm chỉ in ra rồi thôi sẽ trả về None và trượt ngay câu đầu.
assert khoan_ton_nhat(["cà phê,25000", "bún bò,40000", "gửi xe,10000"]) == "bún bò", "trong ba khoản 25000, 40000 và 10000 thì bún bò tốn nhất, nên hàm phải trao lại chuỗi bún bò — không phải khoản đứng đầu danh sách"
assert khoan_ton_nhat(["gửi xe,10000"]) == "gửi xe", "danh sách chỉ có một khoản thì chính khoản ấy tốn nhất, nên hàm phải trao lại chuỗi gửi xe"
assert khoan_ton_nhat(["vở ghi,15000", "trà sữa,45000"]) == "trà sữa", "trong hai khoản 15000 và 45000 thì trà sữa tốn nhất, nên hàm phải trao lại chuỗi trà sữa — trả về khoản đứng đầu danh sách là dò nhầm chiều"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm bên trong ngoặc vuông, ở chỗ nói *lấy cái gì ra từ mỗi dòng* — mà mỗi dòng đọc từ file lên thì còn dính một thứ ở đuôi, đúng thứ bài 8 đã dạy cách cắt. Chỗ trống thứ hai là dòng cuối cùng của một hàm không được phép in ra gì; hàm ấy đã dò xong và đang giữ kết quả trong một cái tên.
- kind: strategy
  body: 'Chỗ một: gọi phương thức cắt hai đầu trên biến chạy của vòng comprehension, và giữ nguyên phần `for dong in cac_dong` phía sau. Chỗ hai: hàm này không chạm màn hình, nên câu cuối của nó phải là câu trao giá trị về cho chỗ gọi — và giá trị cần trao là cái tên đang giữ tên khoản, không phải cái tên đang giữ số tiền.'
- kind: one-line
  body: 'Chỗ một viết `dong.strip()`, chỗ hai viết `return ten_lon_nhat`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  expect: "Khoản tốn nhất: bún bò"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hàm chạm đĩa lo việc chạm đĩa. Hàm tính lo việc tính. Ai vào chỗ nấy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`tinh_tong(cac_dong)` giờ chạy được mà không cần một file nào — nghĩa là kiểm nó
bằng một dòng `assert` cũng được, đúng loại câu mà khối kiểm ở cuối mỗi bài vẫn
dùng để chấm bài của bạn.

Bạn đã có đủ mọi mảnh: ghi xuống đĩa và đọc lên, cắt dòng và tách chữ, bắt lấy
lỗi để đi tiếp, ghép đường dẫn cho đúng mọi máy, nhận tên file từ dòng lệnh,
chia một chương trình thành nhiều lệnh con, gói việc vào module riêng, và giữ
phần thử tách khỏi phần cho mượn.

Ghép thành một cuốn sổ chi tiêu thật được chưa?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.85}
::::
