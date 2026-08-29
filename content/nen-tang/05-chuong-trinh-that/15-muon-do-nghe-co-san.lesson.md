---
id: nen-tang.chuong-trinh-that.muon-do-nghe-co-san
title: Mượn đồ nghề có sẵn
summary: Python đi kèm cả một kho công cụ; `import` mang một hộp vào chương trình và bạn gọi đồ trong hộp qua tên hộp, như `os.getcwd()`.
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [mod.import, mod.dotted-access, core.import-module]
requires: [io.relative-path, core.duong-dan, core.thu-muc, core.function-call, core.builtin-function, core.string-method, core.name-lookup, err.name-error, core.fstring, core.output, core.variable, core.assignment, core.string-literal]
concepts: [core.kho-cong-cu, core.cho-dang-dung, core.duong-dan]
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
Câu này Python trả lời được. Chỉ là câu trả lời không nằm sẵn trong tầm tay bạn.
::::

::::explain{#tren-ke-co-nhung-cai-hop}
Bài trước kết bằng một chỗ bí có thật: bạn cần hỏi máy "tôi đang đứng ở thư
mục nào", và không lệnh nào bạn từng học hỏi được câu đó.

Chỗ bí ấy không phải do bạn học thiếu. `print`, `input`, `open`, `int`, `len`,
`sorted` — chúng nằm sẵn trong tầm tay vì chúng cần cho gần như mọi chương
trình. "Chỗ đang đứng" thì không: phần lớn chương trình không bao giờ cần hỏi
nó. Để một câu hỏi hiếm nằm sẵn trong tầm tay là làm cho tầm tay chật đi.

Nên Python xếp những thứ ấy ra chỗ khác. Hình dung xưởng của Byte: trên bàn là
mấy món dùng hằng ngày, còn dọc tường là một dãy **hộp đồ nghề** đã đóng sẵn,
mỗi hộp một nhóm việc. Có hộp lo chuyện ngày tháng, hộp lo chuyện số ngẫu
nhiên, hộp lo chuyện trao đổi với hệ điều hành — tức là với chính cái máy đang
chạy chương trình.

Hộp cuối cùng ấy tên là `os`. Bên trong nó có món bạn đang thiếu.

Muốn dùng, bạn phải mang hộp về bàn trước. Câu lệnh mang hộp về là `import`:

```python title=readonly
import os
```

Một dòng, đặt ở đầu chương trình. Đọc thành tiếng Việt: *"tôi sẽ dùng hộp
tên `os`"*.

Mang về rồi thì gọi đồ trong hộp bằng **tên hộp, dấu chấm, tên món**:

```python title=readonly
os.getcwd()
```

`getcwd` là tên của món đồ — nó cho về chỗ đang đứng dưới dạng một chuỗi chữ.

Cách viết `hộp.món()` này bạn đã gặp hình dạng của nó rồi: `dong.strip()`,
`dong.split(",")`, `so.append(...)`. Dấu chấm ở đó nói *"cái này thuộc về cái
kia"*, và ở đây cũng vậy — `getcwd` thuộc về hộp `os`.

Có một điểm khác đáng nhớ, và bước đoán ngay sau đây dựng riêng cho nó:
`import os` mang vào chương trình đúng **một** cái tên mới, là `os`. Mọi món
nằm trong hộp vẫn ở trong hộp.
::::

::::example{#hoi-may-dang-dung-o-dau}
Cả chương trình trả lời câu hỏi của bài trước gọn trong bốn dòng:

```python title=readonly
import os

cho_dang_dung = os.getcwd()
print(f"Chỗ đang đứng: {cho_dang_dung}")
print(f"Máy sẽ tìm sổ ở: {cho_dang_dung}/so.txt")
```

Lan mở terminal ở màn hình nền rồi chạy nó:

```text title=readonly
Chỗ đang đứng: /Users/lan/Desktop
Máy sẽ tìm sổ ở: /Users/lan/Desktop/so.txt
```

Đây là lần đầu tiên chương trình nói ra được cái mà bài trước chỉ suy luận
gián tiếp. Không phải đoán qua chuyện `open` chạy hay không nữa: chỗ đang đứng
hiện thẳng lên màn hình.

Ba chỗ đáng dừng lại nhìn:

- **Dòng `import` đứng riêng, ở lề trái, trên cùng.** Nó không nằm trong hàm
  nào, không nằm trong `if` nào. Mang hộp về bàn là việc làm một lần, trước
  khi cần dùng.
- **Kết quả là một chuỗi chữ bình thường.** `cho_dang_dung` giữ dãy ký tự
  `/Users/lan/Desktop`, đem đặt vào f-string được, đem ghép được, đem so sánh
  được — y như mọi chuỗi từ Realm 0 tới giờ.
- **Con số ấy không cố định.** Chạy chương trình này trên máy bạn, đứng ở một
  thư mục khác, dòng đầu sẽ in ra một dãy khác. Đó chính là điều bài trước
  nói: chỗ đang đứng đổi theo lúc gõ lệnh, không theo chỗ file `.py` nằm.
::::

::::predict{#doan-bo-tien-to-os commitOnce}
Byte thấy dòng `os.getcwd()` hơi dài nên thử bỏ bớt tiền tố `os.` đi, giữ
nguyên dòng `import` ở trên.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
import os

print(getcwd())
```

:::opt{correct}
Chương trình dừng; dòng cuối traceback ghi `NameError`
:::

:::opt
In ra chỗ đang đứng, y hệt lúc còn tiền tố `os.`
::why
Gần đúng ở chỗ bạn theo một lối nghĩ rất tự nhiên về chữ `import`: đã mang về
rồi thì mọi thứ trong đó phải dùng được ngay, không cần nhắc lại xuất xứ.

Chỗ lệch là `import os` mang vào chương trình đúng **một** cái tên: `os`. Các
món bên trong không được rải ra ngoài. Và có lý do để nó chặt chẽ như vậy —
`open` cũng là một cái tên bạn đang dùng, mà trong hộp `os` cũng có một món
tên `open`, làm một việc khác. Nếu `import` rải hết đồ trong hộp ra bàn thì
món ấy sẽ đè lên cái `open` bạn vẫn dùng để mở sổ, và bạn không nhìn thấy chỗ
nào trong code báo cho biết chuyện đó vừa xảy ra.
::
:::

:::opt
Chương trình dừng, và dòng cuối traceback ghi `SyntaxError`
::why
Gần đúng ở chỗ bạn nhớ đúng ranh giới của `SyntaxError`: nó là loại lỗi máy
thấy **trước khi chạy** dòng nào, khi câu lệnh không đọc nổi — thiếu ngoặc,
thiếu nháy.

Chỗ lệch là dòng `print(getcwd())` đọc rất trôi: đủ ngoặc, đủ dấu, đúng hình
dạng một lời gọi hàm. Máy đọc xong không vướng chỗ nào, nên nó bắt đầu chạy.
Rắc rối chỉ nảy ra lúc chạy tới đó và phải đi tra xem `getcwd` là cái tên của
ai — đúng cảnh mà bài "chữ và tên là hai thứ khác nhau" đã dựng: tên không
nháy thì máy phải tìm cho ra, tìm không ra thì `NameError`.
::
:::

:::opt
Chương trình dừng ngay ở dòng `import`, vì hộp `os` chưa được cài
::why
Gần đúng ở chỗ bạn đang cẩn thận đúng hướng sau hai bài vừa rồi: có những thứ
chương trình cần mà thế giới bên ngoài chưa có sẵn, và `so.txt` là một ca như
vậy thật.

Chỗ lệch là `os` không giống `so.txt`. Nó đi kèm sẵn trong Python, có mặt trên
mọi máy đã cài Python — không ai phải tạo ra nó, và dòng `import os` chạy trót
lọt. Chỗ hỏng nằm ở dòng sau: hộp đã về tới bàn, nhưng cái tên `getcwd` viết
trần thì vẫn chưa ai nhận là của mình.
::
:::
::::

::::code{#hoi-cho-dang-dung}
Viết chương trình nói ra hai điều: chỗ đang đứng lúc này, và đường dẫn đầy đủ
mà `open("so.txt")` sẽ đi mở nếu bạn gọi nó ngay bây giờ.

Hai chỗ trống: một chỗ nêu **tên hộp** cần mang về, một chỗ **gọi món đồ**
trong hộp ấy.

Dòng ghép đường dẫn đã viết sẵn cho bạn — nó chính là phép hoàn tất địa chỉ
bạn dựng bằng tay ở bài trước, giờ chạy trên chỗ đứng thật của máy này.

```python title=starter
import ___

cho_dang_dung = ___
so_may_se_tim = f"{cho_dang_dung}/so.txt"

print(f"Chỗ đang đứng: {cho_dang_dung}")
print(f"Máy sẽ tìm sổ ở: {so_may_se_tim}")
```

```python title=solution
import os

cho_dang_dung = os.getcwd()
so_may_se_tim = f"{cho_dang_dung}/so.txt"

print(f"Chỗ đang đứng: {cho_dang_dung}")
print(f"Máy sẽ tìm sổ ở: {so_may_se_tim}")
```

```python title=test
# Không chấm bằng một đường dẫn viết sẵn được: chỗ đang đứng khác nhau trên
# mỗi máy, và đó chính là điều bài này dạy. Nên khối kiểm hỏi ba chuyện mà
# một lời giải đúng luôn thoả, còn các lời giải gần đúng thì không.
#
# Chỗ trống TÊN HỘP: nêu sai thì dòng `import` dừng chương trình ngay, không
# câu nào dưới đây chạy tới.
# Chỗ trống GỌI MÓN ĐỒ: cả ba câu dưới đều soi nó.
assert isinstance(cho_dang_dung, str), "chỗ đang đứng phải là một chuỗi chữ — viết tên món đồ mà quên cặp ngoặc gọi thì cái tên còn lại là chính món đồ, không phải câu trả lời của nó"
assert cho_dang_dung == os.getcwd(), "cho_dang_dung phải là chỗ máy này đang đứng thật, hỏi từ hộp os — một đường dẫn gõ cứng sẽ khác nó trên gần như mọi máy"
assert so_may_se_tim == os.getcwd() + "/so.txt", "đường dẫn máy sắp mở là chỗ đang đứng, một dấu gạch, rồi so.txt — đúng phép hoàn tất địa chỉ của bài trước"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất đứng ngay sau chữ `import`, nên thứ điền vào là **tên một cái hộp** — viết trần, không dấu nháy, vì đây là một cái tên chứ không phải một câu chữ. Chỗ trống thứ hai phải cho ra một chuỗi, và chuỗi ấy phải do máy trả lời chứ không do bạn gõ.
- kind: strategy
  body: Hộp lo chuyện trao đổi với hệ điều hành đã được gọi tên ở đầu bài, và món đồ trả lời câu "tôi đang đứng ở đâu" cũng vậy. Viết theo đúng hình dạng `hộp.món()` — và nhớ cặp ngoặc ở cuối: thiếu nó thì bạn mới nhắc tới món đồ, chưa sai nó làm gì cả.
- kind: one-line
  body: "Chỗ trống thứ nhất viết `os`, chỗ thứ hai viết `os.getcwd()`."
:::

:::validate
- tier: static
  onFail: chỗ đang đứng phải được HỎI từ hộp `os`, không gõ cứng một đường dẫn nào
  requireAst:
  # Một lời giải chép cứng đường dẫn của máy đang chấm sẽ qua được cả ba câu
  # assert trên đúng máy ấy rồi hỏng trên mọi máy khác. Luật này đòi lời gọi
  # thật có mặt, nên nó chặn ngay trước khi chạy.
  - kind: uses-call, target: getcwd, min: 1
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
# KHÔNG có tầng `output` nào ghim đường dẫn.
#
# Bản trước ghim `^Chỗ đang đứng: /home/pyodide...` — tức chấm bằng đúng cái
# mà bài này vừa dạy là KHÔNG cố định. Trên một máy đứng ở chỗ khác, lời giải
# ĐÚNG sẽ trượt, và người học không có cách nào biết vì sao. Chú thích trong
# khối kiểm đã tự cấm điều đó rồi.
#
# Nội dung thật do ba `assert` ở tầng `tests` lo — chúng so với `os.getcwd()`
# của chính máy đang chấm, nên đúng trên mọi máy.
- tier: output
  # `contains` KHAI RÕ, không để mặc định. Ở bài này nó là lựa chọn đúng chứ
  # không phải sơ suất: đường dẫn đầy đủ khác nhau trên mỗi máy — đúng điều
  # bài dạy — nên chỉ neo được phần câu chữ do người học viết ra.
  match: contains
  expect: "Máy sẽ tìm sổ ở:"
- tier: output
  match: contains
  expect: "/so.txt"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dòng `import`, và chương trình hỏi được cái nó chưa từng hỏi nổi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ bạn hỏi được chỗ đang đứng, nên bạn thấy rõ chuyện gì đang xảy ra — và
nhìn rõ không có nghĩa là đã chữa được.

Chỗ đứng in ra là `/Users/lan/Desktop`, còn sổ nằm ở `/Users/lan/du-an/so.txt`.
Có cách viết tên file nào **không** phụ thuộc chỗ đứng không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
