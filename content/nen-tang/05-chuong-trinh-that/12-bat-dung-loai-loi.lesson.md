---
id: nen-tang.chuong-trinh-that.bat-dung-loai-loi
title: Chỉ bắt đúng loại lỗi đã gọi tên
summary: "Viết tên một loại lỗi ngay sau `except` là thu hẹp cái lưới lại vừa đúng loại ấy — mọi loại khác đi thẳng qua và vẫn nổ ra như cũ."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [err.except-named]
requires: [err.try-except, io.text-is-str, core.string-split, core.strip-newline, io.readlines, core.with-open, core.file-read, core.file-write, core.int-cast, core.value-error, err.name-error, err.traceback, core.accumulator, core.counter-if, core.list, core.list-index, core.len, core.fstring, core.variable, core.assignment, core.output, ctrl.for-each, ctrl.block-indent]
concepts: [core.loi-khi-chay, core.thong-bao-loi, core.file]
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
Lưới mắt to bắt được cả con bạn muốn thả. Đan lại cho vừa một loại thôi.
::::

::::explain{#cai-luoi-khong-hoi-gi}
Bài trước để lại một chỗ đau rất khó thấy, nên hãy nhìn nó bằng số cụ thể.

Vẫn cuốn sổ bốn dòng ấy, vẫn cặp `try`/`except` ấy, chỉ khác một chuyện: khối
`try` được nới ra hai dòng, và dòng sau gõ nhầm `tein` thay cho `tien`.

```python title=readonly
for dong in cac_dong:
    manh = dong.strip().split(",")
    try:
        tien = int(manh[1])
        tong = tong + tein
    except:
        so_dong_hong = so_dong_hong + 1

print(f"Cộng được: {tong} đồng")
print(f"Bỏ qua {so_dong_hong} dòng hỏng")
```

Máy in ra:

```text title=readonly
Cộng được: 0 đồng
Bỏ qua 4 dòng hỏng
```

Không một dòng traceback nào. Chương trình chạy tới hết, in ra hai câu trông
đàng hoàng, và hai câu ấy nói dối: nó bảo cả bốn dòng sổ đều hỏng, trong khi
cuốn sổ chỉ hỏng đúng một dòng. Thứ hỏng ba dòng còn lại là cái tên `tein` mà
bạn vừa gõ nhầm.

`except:` trơn là một cái lưới không hỏi gì cả. Nó bắt `ValueError` — thứ bạn
muốn bắt — và bắt luôn `NameError`, thứ đáng ra phải nổ tung lên để bạn nhìn
thấy. Cái lưới ấy làm đúng một việc bạn không hề nhờ: **giấu lỗi của chính
bạn**.
::::

::::explain{#goi-ten-cai-can-bat}
Cách chữa nhỏ tới mức dễ coi thường: viết **tên loại lỗi** ngay sau chữ
`except`.

```python title=readonly
try:
    tien = int(manh[1])
    tong = tong + tein
except ValueError:
    so_dong_hong = so_dong_hong + 1
```

Đọc thành tiếng Việt: *thử chạy khối trên; nếu nó nổ ra một `ValueError` thì
nhảy xuống khối dưới — còn nổ ra loại khác thì đây không phải chỗ của nó.*

Chạy lại cuốn sổ ấy với cái lưới đã gọi tên, máy in ra:

```text title=readonly
Traceback (most recent call last):
  File "so.py", line 17, in <module>
    tong = tong + tein
                  ^^^^
NameError: name 'tein' is not defined
```

Đúng cái traceback mà Realm 0 dạy bạn đọc từ dòng cuối lên: dòng cuối nói loại
lỗi, dòng trên nói chỗ nó xảy ra. Con số 17 kia là số dòng trong chương trình
đầy đủ của bạn, nên nó sẽ khác khi chương trình dài ngắn khác đi.

Chương trình dừng, và lần này nó dừng vì một lý do hoàn toàn xứng đáng: code
đang hỏng thì chạy tiếp chỉ tạo thêm số liệu sai.

Ba điều đáng ghi lại:

- **Gọi tên là thu hẹp, không phải mở rộng.** `except ValueError:` bắt được ít
  hơn `except:` trơn, và đó chính là điều bạn muốn.
- **Loại không được gọi tên thì vẫn nổ ra như cũ.** Cặp `try`/`except` không
  làm gì nó cả; nó đi thẳng qua như thể chưa từng có cái lưới nào.
- **Nên gọi tên đúng loại bạn đã lường trước.** Ở đây bạn lường trước rằng có
  dòng ghi tiền bằng chữ, và loại lỗi của chuyện đó là `ValueError` — đúng
  loại Realm 0 đã đặt tên: đúng việc, sai nội dung.

Một cái `try` cũng có thể có **nhiều** nhánh `except`, mỗi nhánh gọi tên một
loại. Mỗi nhánh chỉ đón đúng loại nó gọi, còn loại không nhánh nào gọi tới thì
vẫn thoát ra ngoài. Bước tiếp theo dùng đúng hình dạng ấy.
::::

::::example{#hai-kieu-dong-hong}
Cuốn sổ có hai kiểu dòng hỏng, và chúng hỏng vì hai lý do khác hẳn nhau:

- `bún bò,hai lăm nghìn` — cắt ra đủ hai mảnh, mảnh tiền có mặt, chỉ có nội
  dung không viết ra con số nào. `int` nổ `ValueError`.
- `nước suối` — không có dấu phẩy, nên cắt ra chỉ được **một** mảnh. Xin
  `manh[1]` là xin ô thứ hai của một danh sách một phần tử, và cái đó nổ
  `IndexError` — đúng cái bờ mà T1.4 đã dặn.

Hai nhánh `except`, hai cái tên, hai chỗ đếm riêng:

```python title=readonly
cac_dong = ["cà phê,25000", "bún bò,hai lăm nghìn", "nước suối", "sách,120000"]

tong = 0
so_dong_tien_bang_chu = 0
so_dong_thieu_cot = 0
for dong in cac_dong:
    manh = dong.strip().split(",")
    try:
        tong = tong + int(manh[1])
    except ValueError:
        so_dong_tien_bang_chu = so_dong_tien_bang_chu + 1
    except IndexError:
        so_dong_thieu_cot = so_dong_thieu_cot + 1

print(f"Cộng được: {tong} đồng")
print(f"Dòng ghi tiền bằng chữ: {so_dong_tien_bang_chu}")
print(f"Dòng thiếu hẳn cột tiền: {so_dong_thieu_cot}")
```

Máy in ra:

```text title=readonly
Cộng được: 145000 đồng
Dòng ghi tiền bằng chữ: 1
Dòng thiếu hẳn cột tiền: 1
```

Hai chỗ đáng dừng lại nhìn:

- **Mỗi dòng hỏng rơi đúng vào nhánh của nó.** Không nhánh nào nhận nhầm phần
  của nhánh kia, vì mỗi nhánh chỉ đón đúng cái tên nó gọi. Nhờ vậy bản báo cáo
  nói được cuốn sổ bẩn theo *kiểu* nào, chứ không chỉ bẩn bao nhiêu dòng.
- **Đổi nhánh đầu thành `except:` trơn là hỏng cả hai bản báo cáo.** Lưới
  bắt-tất-cả sẽ đón luôn `IndexError`, dòng `nước suối` bị đếm nhầm sang cột
  bên trái, và cột bên phải mãi mãi đứng ở 0. Python còn không cho bạn viết như
  thế ở đây: nhánh bắt-tất-cả bắt buộc phải là nhánh **cuối cùng**, mà ở đây
  dưới nó còn một nhánh nữa.
::::

::::predict{#doan-loi-khong-goi-ten commitOnce}
Byte giăng lưới `except ValueError:`, rồi vô tình gõ nhầm tên ô tiền: `oo`
thay cho `o`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
cac_o = ["25000", "hai lăm nghìn"]

tong = 0
for o in cac_o:
    try:
        tong = tong + int(oo)
    except ValueError:
        print("bỏ qua ô:", o)

print("tổng:", tong)
```

:::opt{correct}
Chương trình dừng ngay ở lượt đầu và in traceback `NameError`. Không có dòng
nào khác được in ra.
:::

:::opt
`bỏ qua ô: 25000` rồi `bỏ qua ô: hai lăm nghìn` rồi `tổng: 0`
::why
Gần đúng ở chỗ bạn đếm đúng số lượt và đoán đúng rằng lượt nào cũng gặp trục
trặc: cái tên `oo` không tồn tại, nên cả hai lượt đều nổ.

Chỗ lệch là ai đón cái nổ ấy. Đây là thói quen còn lại từ bài trước, nơi
`except:` trơn đón tất cả — và đó là một thói quen rất dễ mang theo. Nhưng
nhánh ở đây đã gọi tên `ValueError`, mà thứ nổ ra là `NameError`. Tên không
khớp thì nhánh này không phải chỗ của nó, nên nó đi thẳng ra ngoài.
::
:::

:::opt
`bỏ qua ô: hai lăm nghìn` rồi `tổng: 25000`
::why
Gần đúng ở chỗ bạn đọc đúng **ý định** của đoạn code: Byte muốn cộng ô đọc
được và bỏ qua ô ghi bằng chữ, và nếu không có lỗi gõ nhầm thì đây chính là
màn hình bạn sẽ thấy.

Chỗ lệch nằm ở đúng hai chữ cái. Dòng trong `try` viết `int(oo)`, không phải
`int(o)`. Cái tên `oo` chưa bao giờ được gán, nên máy nổ `NameError` ngay lượt
đầu, trước cả khi có gì để cộng.
::
:::

:::opt
`tổng: 0`
::why
Gần đúng ở chỗ bạn suy ra đúng một nửa: không lượt nào cộng được gì, nên `tong`
đứng nguyên ở 0.

Chỗ lệch là dòng `print` cuối cùng có chạy hay không. `NameError` ở lượt đầu
không có nhánh nào đón, nên nó dừng cả chương trình ngay tại đó — y như mọi lỗi
không được bắt từ Realm 0 tới giờ. Những dòng nằm sau vòng lặp không bao giờ
tới lượt chạy, kể cả dòng in tổng.
::
:::
::::

::::code{#hai-nhanh-hai-loai-loi}
Cuốn sổ bốn dòng, hỏng theo hai kiểu khác nhau, và bản báo cáo phải tách được
hai kiểu ấy ra.

Nhánh thứ hai đã gọi tên sẵn. Chỗ trống là tên của nhánh thứ nhất.

```python title=starter
with open("so_bai_muoi_hai.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,hai lăm nghìn\n")
    f.write("nước suối\n")
    f.write("sách,120000\n")

with open("so_bai_muoi_hai.txt", "r") as f:
    cac_dong = f.readlines()

tong = 0
so_dong_tien_bang_chu = 0
so_dong_thieu_cot = 0
for dong in cac_dong:
    manh = dong.strip().split(",")
    try:
        tong = tong + int(manh[1])
    except ___:
        so_dong_tien_bang_chu = so_dong_tien_bang_chu + 1
    except IndexError:
        so_dong_thieu_cot = so_dong_thieu_cot + 1

print(f"Cộng được: {tong} đồng")
print(f"Dòng ghi tiền bằng chữ: {so_dong_tien_bang_chu}")
print(f"Dòng thiếu hẳn cột tiền: {so_dong_thieu_cot}")
```

```python title=solution
with open("so_bai_muoi_hai.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,hai lăm nghìn\n")
    f.write("nước suối\n")
    f.write("sách,120000\n")

with open("so_bai_muoi_hai.txt", "r") as f:
    cac_dong = f.readlines()

tong = 0
so_dong_tien_bang_chu = 0
so_dong_thieu_cot = 0
for dong in cac_dong:
    manh = dong.strip().split(",")
    try:
        tong = tong + int(manh[1])
    except ValueError:
        so_dong_tien_bang_chu = so_dong_tien_bang_chu + 1
    except IndexError:
        so_dong_thieu_cot = so_dong_thieu_cot + 1

print(f"Cộng được: {tong} đồng")
print(f"Dòng ghi tiền bằng chữ: {so_dong_tien_bang_chu}")
print(f"Dòng thiếu hẳn cột tiền: {so_dong_thieu_cot}")
```

```python title=test
# Một chỗ trống, nhưng nó bị soi bằng ba câu, và mỗi cách điền sai làm vỡ một
# câu khác nhau:
#
#   `except:` trơn      → Python từ chối ngay lúc đọc chương trình: nhánh
#                         bắt-tất-cả phải đứng cuối, mà dưới nó còn một nhánh.
#                         Không câu nào dưới đây chạy tới.
#   `except Exception:` → đón luôn cả IndexError, nên dòng "nước suối" bị đếm
#                         sang cột bên trái: câu thứ hai ra 2 và câu thứ ba
#                         ra 0, cả hai cùng vỡ.
#   `except TypeError:` → ValueError không nhánh nào đón, chương trình dừng
#                         giữa chừng và không câu nào dưới đây chạy tới.
assert tong == 145000, "hai dòng đọc được số là cà phê 25000 và sách 120000; cộng lại phải ra 145000"
assert so_dong_tien_bang_chu == 1, "trong bốn dòng của sổ này chỉ có đúng một dòng ghi tiền bằng chữ, là 'bún bò,hai lăm nghìn'"
assert so_dong_thieu_cot == 1, "dòng 'nước suối' không có dấu phẩy nên cắt ra chỉ được một mảnh, và đó là dòng duy nhất của sổ này làm manh[1] nổ IndexError"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ngay sau chữ `except` của nhánh THỨ NHẤT. Nhánh ngay dưới nó đã gọi tên `IndexError` — nhìn nhánh ấy để thấy chỗ trống này cần loại chữ gì.
- kind: strategy
  body: Hai nhánh phải chia nhau hai kiểu dòng hỏng. Dòng `nước suối` thiếu hẳn cột tiền, và nhánh dưới đã nhận phần ấy rồi. Phần còn lại là dòng cắt ra đủ hai mảnh nhưng mảnh tiền viết bằng chữ — Realm 0 đã đặt tên cho đúng loại lỗi đó khi `int` gặp một chuỗi không viết ra con số nào.
- kind: one-line
  body: "Viết `ValueError` vào chỗ trống."
:::

:::validate
- tier: static
  onFail: nhánh thứ nhất phải GỌI TÊN loại lỗi mà nó nhận, không để trống và không gọi một cái tên bao trùm cả nhánh dưới
  requireAst:
  # Cái tên `ValueError` phải có mặt thật trong mã. `except:` trơn hay
  # `except Exception:` đều không có nó.
  - kind: uses-name, target: ValueError, min: 1
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^Cộng được: 145000 đồng\nDòng ghi tiền bằng chữ: 1\nDòng thiếu hẳn cột tiền: 1\s*$
- tier: output
  expect: "Dòng thiếu hẳn cột tiền: 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mỗi cái lưới một tên. Thứ không ai gọi tên thì cứ để nó nổ — nổ là nó đang nói.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ba bài vừa rồi, chương trình nào cũng **tự ghi** cuốn sổ ra trước rồi mới đọc
lên. Nhờ vậy file lúc nào cũng có sẵn, và bạn chưa bao giờ phải nghĩ tới chuyện
nó có tồn tại hay không.

Ngày mai bạn bỏ đoạn ghi đi — hợp lý thôi, vì sổ đã nằm trên đĩa từ hôm qua rồi,
ghi lại là xoá mất. Chương trình còn mỗi phần đọc, và nó chạy ngon lành trên máy
bạn. Rồi bạn chép nó sang máy của người bạn cùng lớp, máy chưa từng có cuốn sổ
nào.

Máy ấy dừng ngay dòng `open`. Lỗi lần này không phải `ValueError`: mảnh tiền
còn chưa được đọc lên thì lấy đâu ra nội dung sai. Cũng không phải `IndexError`.

Nó là loại gì — và có phải do bạn viết sai code không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
