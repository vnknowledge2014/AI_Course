---
id: nen-tang.chuong-trinh-that.loi-den-tu-ben-ngoai
title: Có loại lỗi không phải lỗi của bạn
summary: "`FileNotFoundError` sinh ra từ thế giới bên ngoài — file chưa có ở chỗ máy tìm — nên code viết đúng vẫn phải chuẩn bị đón nó."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [err.file-not-found]
requires: [err.except-named, err.try-except, io.readlines, core.with-open, core.file-read, core.file-write, core.file-close, core.value-error, err.traceback, err.name-error, core.function-def, core.function-return, core.function-parameter, core.docstring, core.len, core.list, core.fstring, core.output, core.string-literal, core.variable, core.assignment]
concepts: [core.loi-ngoai-canh, core.file, core.so-chi-tieu]
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
Lần này máy không chê chữ bạn viết. Nó chê cái thế giới quanh chương trình.
::::

::::explain{#chay-lan-dau-tren-may-moi}
Bài trước bạn đã nêu đích danh loại lỗi cần bắt. `except ValueError:` đỡ lấy
đúng một dòng sổ gõ nhầm, còn cái tên biến bạn gõ sai thì vẫn nổ ra như cũ —
và đó là chỗ `except:` trơn từng nuốt mất.

Rồi bài trước để bạn lại ở đúng một chỗ khó. Bạn bỏ đoạn tự ghi sổ đi — sổ đã
nằm trên đĩa từ hôm qua rồi, ghi lại là xoá mất — nên chương trình còn mỗi phần
đọc. Trên máy bạn nó chạy ngon lành. Bạn chép `main.py` sang máy của Lan, người
bạn cùng lớp, máy chưa từng có cuốn sổ nào.

Trên máy Lan chưa có `so.txt`, và chưa ai ghi khoản nào vào đó cả. Máy ấy dừng
ngay dòng `open`.

Câu hỏi của bài này nằm gọn trong khoảnh khắc ấy: **`open("so.txt", "r")` gặp
một cái tên không ứng với file nào thì làm gì?**

Đây là câu hỏi mà mười hai bài vừa rồi chưa bao giờ phải hỏi, vì lần nào cuốn
sổ cũng do chính bạn ghi ra ở bài trước đó. Lần này thì không.
::::

::::predict{#doan-open-r-gap-file-chua-co commitOnce}
Trên máy vừa cài, chưa có file nào tên `so.txt`.

**Trước khi bấm chạy**, bạn đoán máy làm gì?

```python title=readonly
with open("so.txt", "r") as f:
    noi_dung = f.read()

print(f"Đọc được {len(noi_dung)} ký tự.")
```

:::opt{correct}
Chương trình dừng ngay tại dòng `open`; dòng cuối traceback ghi
`FileNotFoundError`, và câu `print` không chạy
:::

:::opt
In ra `Đọc được 0 ký tự.` — chưa có file thì coi như file rỗng
::why
Gần đúng ở chỗ bạn nhớ một hành vi có thật và nhớ đúng: chế độ `"w"` ở bài ghi
dòng đầu tiên **tự tạo ra** file khi chưa có cái nào mang tên ấy. Bạn đã tận
mắt thấy một file mọc ra từ chương trình của mình.

Chỗ lệch là `"r"` và `"w"` không cùng một việc. `"w"` nói *"đặt chữ vào đây"*,
nên máy có quyền dựng chỗ mới để đặt. `"r"` nói *"đưa tôi thứ đang nằm ở đây"* —
mà không có gì nằm ở đó cả, và máy không có quyền bịa ra một cuốn sổ trống rồi
bảo với bạn rằng đó là sổ của Lan. Nên nó dừng lại và nói ra.
::
:::

:::opt
Dòng `open` chạy qua bình thường; chương trình dừng ở dòng `print`, khi
`noi_dung` mới thật sự được dùng tới
::why
Gần đúng ở chỗ bạn để ý một điều hoàn toàn có thật: `open` chưa đọc chữ nào cả.
Nó mở kết nối, còn `.read()` mới là lúc lấy nội dung ra. Suy ra rằng chuyện
hỏng phải lộ muộn hơn là một suy luận có căn cứ.

Chỗ lệch là `open` không chỉ ghi nhớ cái tên rồi để đó chờ. Ngay tại dòng ấy nó
đã đi hỏi hệ điều hành để lấy về một **kết nối đang mở** — đúng cái kết nối mà
bài "file đang mở thì phải đóng" nói tới. Không có file thì không có kết nối
nào để đưa lại, nên nó dừng ngay tại chỗ. Và chi tiết ấy đáng giá: traceback
chỉ thẳng vào dòng `open`, chứ không bắt bạn dò ngược cả chương trình để đoán
xem chuyện hỏng bắt đầu từ đâu.
::
:::

:::opt
Chương trình dừng, và dòng cuối traceback ghi `NameError`
::why
Gần đúng ở chỗ bạn dịch `NameError` ra tiếng Việt rất sát: "không tìm thấy".
Mà ở đây máy đúng là không tìm thấy thật.

Chỗ lệch là **cái gì** không tìm thấy. `NameError` nói về một cái tên trong
code mà máy phải đi tra, và bài "chữ và tên là hai thứ khác nhau" đã vạch ranh
giới ấy bằng đúng cặp dấu nháy: `so.txt` không nháy mới là một cái tên phải
tra; `"so.txt"` có nháy là một câu chữ, máy đọc nguyên văn và không tra ai cả.
Thứ vắng mặt không nằm trong chương trình — nó nằm ngoài kia, trên đĩa.
::
:::
::::

::::example{#trong-thay-cai-traceback}
Đây là chương trình Lan chạy, đã có `except ValueError:` của bài trước bọc lấy
chỗ đổi chữ thành số:

```python title=readonly
tong = 0
with open("so.txt", "r") as f:
    for dong in f.readlines():
        manh = dong.strip().split(",")
        try:
            tong = tong + int(manh[1])
        except ValueError:
            print(f"bỏ qua dòng hỏng: {dong.strip()}")
print(f"Tổng: {tong} đồng")
```

Trên máy Lan, màn hình hiện ra:

```text title=readonly
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    with open("so.txt", "r") as f:
FileNotFoundError: [Errno 2] No such file or directory: 'so.txt'
```

Đọc **từ dòng cuối lên**, đúng như bài đọc thông báo lỗi đã dạy:

- Dòng cuối cho biết loại lỗi: `FileNotFoundError`. Một cái tên mới, chưa gặp
  bao giờ. Phần sau dấu hai chấm nói luôn thứ nó đi tìm mà không có: `'so.txt'`.
- Dòng trên nói chỗ nổ: `line 2`, tức là chính dòng `open`.

Hai chi tiết đáng dừng lại thật lâu.

**Thứ nhất: `line 2`.** Vòng lặp chưa chạy lượt nào, `int()` chưa được gọi lần
nào, và cái `except ValueError:` bạn cất công viết ở bài trước chưa có dịp
làm gì cả. Chương trình chết trước khi có một dòng sổ nào để đọc.

**Thứ hai, và đây mới là điều bài này muốn nói: không có chữ nào trong đoạn
code kia viết sai.** Không tên biến gõ nhầm, không kiểu lệch, không dấu ngoặc
thiếu. Đem đúng file `main.py` ấy về máy bạn — nơi `so.txt` đã có sẵn từ tuần
trước — nó chạy đúng như mọi lần.

Mọi lỗi từ Realm 0 tới giờ đều sửa được bằng cách sửa một chỗ trong code: đặt
lại tên, đổi kiểu, thêm dấu ngoặc, bọc `int()` vào `try`. Lỗi này thì không.
Thứ vắng mặt nằm **ngoài** chương trình, và không có cách viết code nào làm
cho một file chưa tồn tại hiện ra.
::::

::::explain{#don-truoc-cai-con-thieu}
Nếu không sửa được nguyên nhân thì làm gì?

Hỏi lại cho đúng: **"chưa có sổ" có phải một tai nạn không?** Không. Nó là
trạng thái bình thường của lần chạy đầu tiên, trên mọi máy, với mọi người
dùng. Ai cũng phải đi qua nó đúng một lần.

Một trạng thái bình thường thì đáng được chương trình *chuẩn bị đón*, chứ
không đáng làm chương trình chết. Và bạn đã có sẵn công cụ để đón: đúng cái
`try` / `except` của hai bài trước, chỉ đổi tên loại lỗi.

```python title=readonly
try:
    with open("so.txt", "r") as f:
        cac_dong = f.readlines()
except FileNotFoundError:
    cac_dong = []
    print("Chưa có sổ nào — hôm nay là ngày mở sổ.")

print(f"Sổ đang có {len(cac_dong)} khoản.")
```

Trên máy Lan, lần chạy đầu tiên:

```text title=readonly
Chưa có sổ nào — hôm nay là ngày mở sổ.
Sổ đang có 0 khoản.
```

Ba chỗ đáng nhìn kỹ:

- **`try` bọc cả `with`, không bọc bên trong.** Chỗ nổ là chính dòng `open`,
  nên `open` phải nằm trong `try`. Đặt `try` vào bên trong khối `with` thì nó
  không bao giờ được chạy tới.
- **Nhánh `except` phải để lại một giá trị dùng được.** `cac_dong = []` nói
  rằng sổ trống — một danh sách không có phần tử nào, thứ mà `len` và `for`
  đối xử y hệt mọi danh sách khác. Không có dòng ấy thì cái tên `cac_dong`
  chưa từng được dán lên gì, và dòng `print` phía dưới sẽ nổ `NameError`.
- **Tên loại lỗi vẫn được nêu đích danh.** `except FileNotFoundError:` đỡ đúng
  chuyện thiếu file. Một dòng sổ gõ nhầm vẫn nổ `ValueError` ra ngoài như cũ,
  vì đó là chuyện khác và đáng được xử lý ở chỗ khác.

Nói gọn lại điều mới của bài: có những lỗi **không đến từ code**. Chúng đến từ
thế giới bên ngoài — file chưa có, đĩa đầy, mạng đứt. Code viết đúng tới đâu
cũng không ngăn được chúng, nên thứ duy nhất làm được là **chuẩn bị đón**. Đó
chính là lý do cả cơ chế ngoại lệ tồn tại.
::::

::::code{#dem-khoan-khi-so-chua-co}
Byte đưa cho bạn một hàm đếm số khoản trong sổ, và hai cuốn sổ để thử nó.

Ba dòng đầu chương trình ghi ra cuốn sổ của tháng này, nên cuốn đó chắc chắn
có. Cuốn `so-cua-lan-thang-truoc.txt` thì chưa ai ghi lần nào — và sẽ không ai
ghi cả, đó là ý đồ của bài.

Hai chỗ trống nằm trong nhánh đón lỗi: **tên loại lỗi** cần đỡ, và **con số**
để lại cho một cuốn sổ chưa có.

```python title=starter
with open("so-cua-lan.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")


def dem_khoan(ten_file):
    """Đếm số khoản trong một cuốn sổ. Sổ chưa có thì coi như chưa khoản nào."""
    try:
        with open(ten_file, "r") as f:
            return len(f.readlines())
    except ___:
        return ___


so_khoan_thang_nay = dem_khoan("so-cua-lan.txt")
so_khoan_thang_truoc = dem_khoan("so-cua-lan-thang-truoc.txt")

print(f"Sổ tháng này: {so_khoan_thang_nay} khoản")
print(f"Sổ tháng trước: {so_khoan_thang_truoc} khoản")
```

```python title=solution
with open("so-cua-lan.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")


def dem_khoan(ten_file):
    """Đếm số khoản trong một cuốn sổ. Sổ chưa có thì coi như chưa khoản nào."""
    try:
        with open(ten_file, "r") as f:
            return len(f.readlines())
    except FileNotFoundError:
        return 0


so_khoan_thang_nay = dem_khoan("so-cua-lan.txt")
so_khoan_thang_truoc = dem_khoan("so-cua-lan-thang-truoc.txt")

print(f"Sổ tháng này: {so_khoan_thang_nay} khoản")
print(f"Sổ tháng trước: {so_khoan_thang_truoc} khoản")
```

```python title=test
# Hai chỗ trống, và mỗi chỗ có ít nhất một câu vỡ nếu điền sai.
#
# Chỗ trống TÊN LOẠI LỖI: cuốn `so-cua-lan-thang-truoc.txt` chưa hề tồn tại,
# nên `open` ở đó ném ra một `FileNotFoundError` thật. Nêu sai tên loại lỗi
# thì không nhánh nào đỡ, chương trình dừng, và cả ba câu dưới đây đều không
# chạy tới.
assert so_khoan_thang_nay == 2, "ba dòng đầu chương trình vừa ghi đúng hai khoản vào so-cua-lan.txt, nên đếm cuốn ấy phải ra 2"
# Chỗ trống CON SỐ: chỉ câu này soi nó. Để lại 1 thay vì 0 thì cuốn sổ chưa
# tồn tại lại được báo là đang có một khoản.
assert so_khoan_thang_truoc == 0, "so-cua-lan-thang-truoc.txt chưa ai ghi lần nào, nên nhánh đón lỗi phải để lại con số 0 khoản"
# Một tên file khác, cũng chưa từng có, để chắc rằng nhánh đón lỗi làm việc
# cho MỌI cuốn sổ vắng mặt chứ không riêng cuốn đã gọi ở trên.
assert dem_khoan("so-cua-lan-nam-ngoai.txt") == 0, "so-cua-lan-nam-ngoai.txt cũng chưa từng tồn tại, nên gọi dem_khoan trên nó phải cho về 0 chứ không làm chương trình dừng"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm ngay sau chữ `except`, tức là chỗ nêu tên loại lỗi mà nhánh này nhận đỡ. Loại lỗi ấy vừa hiện lên ở dòng cuối traceback trong phần ví dụ. Chỗ trống thứ hai nằm sau `return`, trong nhánh chạy khi cuốn sổ không có — nên nó là câu trả lời cho "một cuốn sổ chưa tồn tại thì đang có bao nhiêu khoản".
- kind: strategy
  body: Bài trước đã dặn rằng nêu tên loại lỗi thì chỉ loại đó bị bắt, nên tên phải viết đúng từng chữ, không nháy, và viết đúng cái tên máy đã in ra chứ không phải một tên gần giống. Còn con số ở chỗ trống thứ hai thì hỏi thẳng: một cuốn sổ chưa ai ghi vào đang giữ mấy khoản?
- kind: one-line
  body: "Chỗ trống thứ nhất viết `FileNotFoundError`, chỗ thứ hai viết `0`."
:::

:::validate
- tier: static
  onFail: nhánh đón lỗi phải nêu đích danh loại lỗi thiếu file, không bắt gộp mọi thứ
  requireAst:
  # Bài trước dạy rằng nêu tên loại lỗi mới là bắt đúng chỗ. `except Exception:`
  # chạy qua được cả ba câu assert, nhưng nó bắt luôn mọi lỗi khác — đúng cái
  # bẫy `except:` trơn mà bài 11 đã dựng lên rồi bài 12 đã gỡ. Luật này đòi
  # chính cái tên ấy có mặt trong mã.
  - kind: uses-name, target: FileNotFoundError, min: 1
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^Sổ tháng này: 2 khoản\nSổ tháng trước: 0 khoản\s*$
- tier: output
  expect: "Sổ tháng trước: 0 khoản"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chương trình của bạn vừa sống sót qua một chuyện nó không gây ra.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn cầm cái `except FileNotFoundError:` vừa viết về máy mình để thử lại cho
chắc. Ở đây thì `so.txt` có thật: nó nằm ngay cạnh `main.py`, cùng một thư
mục, bạn nhìn thấy nó bằng mắt trong cửa sổ quản lý file.

Bạn mở terminal ở màn hình nền, gõ lệnh chạy `main.py`. Máy vẫn in ra
`Chưa có sổ nào`.

Nhưng `so.txt` nằm ngay cạnh `main.py`, bạn nhìn thấy nó bằng mắt. Máy vẫn bảo
không có. Máy đang tìm ở đâu?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
