---
id: nen-tang.chuong-trinh-that.ghep-duong-dan-cho-dung
title: Ghép đường dẫn cho đúng
summary: "`pathlib.Path` là một kiểu riêng dành cho đường dẫn: ghép bằng dấu `/`, và nó tự đặt đúng một dấu ngăn, đúng dấu mà máy đang chạy dùng."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.pathlib-path]
requires: [core.absolute-path, mod.import, mod.dotted-access, core.with-open, core.file-write, core.file-read, core.duong-dan, core.thu-muc, core.file, core.string-concat, core.string-literal, core.variable, core.assignment, core.output, core.fstring, core.type-of-value]
concepts: [core.duong-dan, core.kieu-rieng, core.toan-tu-theo-kieu]
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
Dán hai mẩu chữ vào nhau thì máy không biết đó là đường dẫn. Nó chỉ thấy chữ.
::::

::::explain{#day-chu-khong-biet-no-la-duong-dan}
Đường dẫn kể từ gốc đã gỡ được chuyện phụ thuộc chỗ đứng. Nhưng cách bạn **dựng**
ra nó ở bài trước thì vẫn còn mong manh, và mong manh ở hai chỗ khác nhau.

**Chỗ thứ nhất: dấu gạch ở chỗ nối.** Với máy, `thu_muc + "/" + ten_so` không
phải phép ghép đường dẫn — nó là phép nối chuỗi của Realm 0, y hệt phép nối
`"Phở" + " bò"`. Máy dán ba mẩu chữ vào nhau, đúng từng ký tự một, và không hề
biết ba mẩu ấy đang tả một chỗ trên đĩa. Nên nếu số dấu gạch bạn đặt vào không
đúng một cái, máy vẫn dán, và dán ra một câu chữ hỏng:

```python title=readonly
thu_muc_a = "/Users/lan/du-an"
thu_muc_b = "/Users/lan/du-an/"

print(thu_muc_a + "so.txt")
print(thu_muc_b + "/so.txt")
```

Máy in ra:

```text title=readonly
/Users/lan/du-anso.txt
/Users/lan/du-an//so.txt
```

Dòng đầu thiếu dấu gạch, và nó trỏ tới một cái tên không có thật. Dòng sau thừa
một dấu, và nó vẫn mở được file — nhưng câu chữ ấy đem in ra cho người dùng đọc
thì trông sai, đem so bằng `==` với đường dẫn viết đúng thì cho `False`. Chuyện
đáng ngại là **cả hai đều không kêu tiếng nào lúc ghép**: chuỗi cộng chuỗi thì
lúc nào cũng thành công.

**Chỗ thứ hai: mỗi hệ máy ngăn thư mục bằng một dấu khác nhau.** Bạn viết `/`
vì máy của Lan là macOS. Bạn cùng lớp mở chính file code ấy trên Windows, nơi
đường dẫn viết là `C:\Users\lan\du-an\so.txt`. Không có cách nào để một dấu gạch
gõ cứng trong code đúng được ở cả hai bên.

Cả hai chỗ mong manh có chung một gốc: **máy không biết thứ nó đang cầm là một
đường dẫn.** Nên cách chữa cũng là một: nói cho máy biết.

Python có sẵn một hộp đồ nghề tên `pathlib`, mượn về đúng cách bài 15 đã dạy:

```python title=readonly
import pathlib

duong_dan = pathlib.Path("/Users/lan/du-an")
```

`pathlib.Path(...)` nhận vào một câu chữ và đưa ra một giá trị thuộc **kiểu
riêng dành cho đường dẫn** — không còn là `str` nữa. Ý niệm "mỗi giá trị có một
kiểu" thì bạn có từ R0·14; ở đây chỉ có thêm một kiểu mới trong danh sách.

Và vì nó là một kiểu riêng, nó cư xử theo luật riêng. Luật đáng nhớ nhất:

```python title=readonly
import pathlib

thu_muc = pathlib.Path("/Users/lan/du-an")
duong_dan_so = thu_muc / "so.txt"

print(duong_dan_so)
```

```text title=readonly
/Users/lan/du-an/so.txt
```

Dấu `/` đặt giữa một `Path` và một mẩu tên **là phép ghép đường dẫn**. Nó tự
đặt đúng một dấu ngăn vào chỗ nối: thư mục đã có sẵn dấu gạch ở cuối thì nó
không thêm nữa, chưa có thì nó thêm vào. Và trên Windows nó đặt dấu `\`, vì lúc
đó nó biết mình đang chạy ở đâu — còn bạn thì vẫn gõ đúng một dấu `/` trong code.

Bạn đã gặp chuyện "cùng một dấu, nghĩa đổi theo kiểu" một lần rồi: dấu `+` giữa
hai con số là phép cộng, giữa hai câu chữ là phép nối (R0·10). `/` cũng vậy —
giữa hai con số là phép chia, còn bên trái là một `Path` thì nó là phép ghép.
::::

::::example{#ghep-nhieu-tang}
Ghép được một tầng thì ghép được nhiều tầng, và mỗi tầng vẫn là một dấu `/`.

```python title=readonly
import os
import pathlib

# Giàn giáo cho ví dụ chạy được: dựng sẵn thư mục và thư mục con.
os.makedirs("/Users/lan/du-an/sao-luu", exist_ok=True)

thu_muc = pathlib.Path("/Users/lan/du-an")
duong_dan_so = thu_muc / "so.txt"
duong_dan_sao_luu = thu_muc / "sao-luu" / "so.txt"

print(duong_dan_so)
print(duong_dan_sao_luu)

with open(duong_dan_so, "w") as f:
    f.write("cà phê,25000\n")

with open(duong_dan_so, "r") as f:
    print("Đọc được:", f.read().strip())
```

Máy in ra:

```text title=readonly
/Users/lan/du-an/so.txt
/Users/lan/du-an/sao-luu/so.txt
Đọc được: cà phê,25000
```

Ba chỗ đáng dừng lại nhìn:

- **`thu_muc` không hề đổi.** Dòng thứ hai và dòng thứ ba đều bắt đầu từ nó, và
  mỗi dòng cho ra một đường dẫn mới. Ghép không sửa cái cũ — giống hệt chuyện
  `.strip()` đưa về một chuỗi mới chứ không cắt tại chỗ.
- **Ghép hai tầng viết liền một mạch:** `thu_muc / "sao-luu" / "so.txt"`. Máy
  làm từ trái sang, y như cách nó đọc `1 + 2 + 3`: ghép xong tầng đầu thì lại
  có một `Path`, nên ghép tiếp tầng sau được.
- **`open` nhận thẳng một `Path`.** Bạn không phải đổi nó về chuỗi trước; `open`
  hiểu cả hai kiểu.
::::

::::predict{#doan-hai-cach-ghep commitOnce}
Byte lấy một tên thư mục viết theo thói quen có dấu gạch ở cuối, rồi ghép tên
file vào theo hai cách: một bằng phép cộng chuỗi, một bằng `pathlib`.

**Trước khi bấm chạy**, bạn đoán hai dòng in ra gì?

```python title=readonly
import pathlib

thu_muc = "/Users/lan/du-an/"

print(thu_muc + "/so.txt")
print(pathlib.Path(thu_muc) / "so.txt")
```

:::opt{correct}
`/Users/lan/du-an//so.txt` rồi `/Users/lan/du-an/so.txt`
:::

:::opt
Hai dòng in ra giống hệt nhau: `/Users/lan/du-an/so.txt`
::why
Gần đúng ở chỗ bạn đọc đúng **ý định** của cả hai dòng: cả hai đều đang muốn nói
tới cuốn sổ trong thư mục `du-an`, và trên đĩa thì `/Users/lan/du-an//so.txt`
đúng là mở ra cuốn sổ ấy thật.

Chỗ lệch nằm ở việc dòng đầu chỉ là phép nối chuỗi. Nó dán `"/Users/lan/du-an/"`
với `"/so.txt"` đúng từng ký tự, nên chỗ nối có hai dấu gạch đứng cạnh nhau và
**câu chữ in ra mang cả hai**. Chỉ dòng thứ hai mới biết mình đang tả một đường
dẫn, nên chỉ nó rút hai dấu ấy về một.
::
:::

:::opt
Cả hai dòng đều in ra `/Users/lan/du-an//so.txt`
::why
Gần đúng ở chỗ bạn nhìn ra ngay chỗ hỏng của dòng đầu — hai dấu gạch chồng nhau
— và bạn nhìn không sai một ký tự.

Chỗ lệch là bạn cho rằng `pathlib.Path` chỉ bọc lại câu chữ rồi chuyển tiếp
nguyên vẹn. Nó làm nhiều hơn thế: nó tách câu chữ ấy ra thành từng tầng thư mục,
và khi in lại thì nó ráp các tầng ấy bằng đúng một dấu ngăn cho mỗi chỗ nối.
Dấu gạch thừa không sống sót qua chỗ đó.
::
:::

:::opt
Dòng đầu in ra `/Users/lan/du-an//so.txt`, dòng sau báo lỗi vì `/` là phép chia,
mà không chia được hai câu chữ
::why
Gần đúng ở chỗ bạn nhớ đúng một luật đã học: `/` giữa hai giá trị vốn là phép
chia, và đem chia hai câu chữ thì đúng là máy báo `TypeError` (R0·16).

Chỗ lệch là vế trái ở đây không phải một câu chữ. `pathlib.Path(thu_muc)` cho về
một giá trị thuộc kiểu đường dẫn, và nghĩa của một dấu toán được quyết định bởi
kiểu của giá trị đứng đó. Giống hệt chuyện `+` đổi nghĩa giữa số và chuỗi: `/`
đổi nghĩa khi bên trái là một `Path`, và ở đó nó là phép ghép.
::
:::
::::

::::code{#ghep-hai-duong-dan-bang-pathlib}
Lan giữ hai cuốn sổ: cuốn đang dùng nằm thẳng trong thư mục dự án, và một bản
sao lưu nằm trong thư mục con `sao-luu` bên trong nó.

Byte đưa tên thư mục dự án **hai lần, viết theo hai thói quen khác nhau** —
`thu_muc_co_gach` có dấu gạch ở cuối, `thu_muc_khong_gach` thì không. Đó là
chuyện thường gặp khi tên thư mục đến từ hai nguồn khác nhau, và nó chính là
chỗ phép cộng chuỗi trượt chân. Cách ghép bạn viết phải cho ra đường dẫn đúng
từ **cả hai** kiểu viết ấy.

```python title=starter
import os
import pathlib

# Giàn giáo: dựng sẵn thư mục dự án và thư mục con sao-luu.
os.makedirs("/Users/lan/du-an/sao-luu", exist_ok=True)

thu_muc_co_gach = "/Users/lan/du-an/"
thu_muc_khong_gach = "/Users/lan/du-an"

duong_dan_so = ___
duong_dan_sao_luu = ___

with open(duong_dan_so, "w") as f:
    f.write("cà phê,25000\n")

with open(duong_dan_sao_luu, "w") as f:
    f.write("bún bò,40000\n")

print(f"Sổ đang dùng: {duong_dan_so}")
print(f"Bản sao lưu: {duong_dan_sao_luu}")

with open("/Users/lan/du-an/so.txt", "r") as f:
    print("Trong du-an:", f.read().strip())

with open("/Users/lan/du-an/sao-luu/so.txt", "r") as f:
    print("Trong sao-luu:", f.read().strip())
```

```python title=solution
import os
import pathlib

# Giàn giáo: dựng sẵn thư mục dự án và thư mục con sao-luu.
os.makedirs("/Users/lan/du-an/sao-luu", exist_ok=True)

thu_muc_co_gach = "/Users/lan/du-an/"
thu_muc_khong_gach = "/Users/lan/du-an"

duong_dan_so = pathlib.Path(thu_muc_co_gach) / "so.txt"
duong_dan_sao_luu = pathlib.Path(thu_muc_khong_gach) / "sao-luu" / "so.txt"

with open(duong_dan_so, "w") as f:
    f.write("cà phê,25000\n")

with open(duong_dan_sao_luu, "w") as f:
    f.write("bún bò,40000\n")

print(f"Sổ đang dùng: {duong_dan_so}")
print(f"Bản sao lưu: {duong_dan_sao_luu}")

with open("/Users/lan/du-an/so.txt", "r") as f:
    print("Trong du-an:", f.read().strip())

with open("/Users/lan/du-an/sao-luu/so.txt", "r") as f:
    print("Trong sao-luu:", f.read().strip())
```

```python title=test
# Chỗ trống thứ nhất bị soi bởi câu so chuỗi ngay dưới đây: nó dựng từ
# `thu_muc_co_gach`, thứ ĐÃ có sẵn một dấu gạch ở cuối — nên lời giải cộng
# thêm `"/so.txt"` sẽ ra hai dấu gạch và vỡ ngay tại đây.
assert str(duong_dan_so) == "/Users/lan/du-an/so.txt", "cuốn sổ đang dùng nằm thẳng trong thư mục dự án, nên đường dẫn của nó phải đọc ra đúng /Users/lan/du-an/so.txt — hai dấu gạch liền nhau ở chỗ nối là dấu hiệu thư mục đã có gạch cuối mà vẫn bị cộng thêm một dấu nữa"
# Chỗ trống thứ hai bị soi bởi câu này: nó cần HAI tầng ghép, và
# `thu_muc_khong_gach` thì KHÔNG có dấu gạch cuối — nên lời giải dán thẳng
# hai mẩu vào nhau sẽ ra /Users/lan/du-ansao-luu/so.txt.
assert str(duong_dan_sao_luu) == "/Users/lan/du-an/sao-luu/so.txt", "bản sao lưu nằm trong thư mục con sao-luu, nên đường dẫn của nó phải đọc ra đúng /Users/lan/du-an/sao-luu/so.txt — thiếu một dấu gạch ở chỗ nối thì tên thư mục dính liền vào nhau"

with open("/Users/lan/du-an/so.txt", "r") as f:
    assert f.read() == "cà phê,25000\n", "file /Users/lan/du-an/so.txt phải giữ dòng cà phê,25000 — nếu nó đang giữ dòng bún bò thì duong_dan_sao_luu đã quên tầng sao-luu và ghi đè lên cuốn sổ đang dùng"

with open("/Users/lan/du-an/sao-luu/so.txt", "r") as f:
    assert f.read() == "bún bò,40000\n", "file /Users/lan/du-an/sao-luu/so.txt phải giữ dòng bún bò,40000 — đây là bản sao lưu, không phải cuốn sổ đang dùng"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều phải cho ra một đường dẫn, và ở cả hai chỗ thứ Byte đưa bạn mới chỉ là một câu chữ. Việc đầu tiên là nói cho máy biết câu chữ ấy tả một đường dẫn, rồi mới nối các tầng còn lại vào. Chỗ thứ hai cần nhiều hơn chỗ thứ nhất đúng một tầng, vì bản sao lưu nằm sâu hơn một thư mục.
- kind: strategy
  body: Bọc câu chữ tên thư mục lại bằng hộp đồ nghề vừa mượn, rồi nối từng tầng bằng dấu ghép của kiểu đường dẫn — mỗi tầng một dấu, và đừng tự gõ thêm dấu ngăn nào vào trong dấu nháy, vì chỗ nối đã có người lo. Nhớ rằng một thư mục có dấu gạch ở cuối và một thư mục không có phải cho ra cùng một kết quả.
- kind: one-line
  body: 'Chỗ trống thứ nhất viết `pathlib.Path(thu_muc_co_gach) / "so.txt"`, chỗ thứ hai viết `pathlib.Path(thu_muc_khong_gach) / "sao-luu" / "so.txt"`.'
:::

:::validate
- tier: static
  requireAst:
    - kind: uses-name, target: pathlib, min: 2
    # `min: 3`, không phải 1. `uses-name` chỉ đếm chỗ ĐỌC tên nên
    # `import pathlib` không tính, và hai chỗ trống mỗi chỗ nhắc `pathlib` một
    # lần là đã đủ 2 — tức chỗ trống thứ hai chép cứng cả đường dẫn vẫn lọt,
    # đúng thứ bài này dạy cách bỏ đi. Lời giải mẫu có ba dấu `/`.
    - kind: uses-operator, target: /, min: 3
    - kind: uses-name, target: thu_muc_co_gach, min: 1
  onFail: "hai chỗ trống phải ghép đường dẫn bằng `pathlib` VÀ nối từng tầng bằng dấu `/` như bài vừa dạy — không cộng chuỗi, và cũng đừng dồn hết các tầng vào trong một cặp ngoặc `Path(...)`: cách ấy chạy ra đúng kết quả, nhưng nó bỏ qua chính dấu `/` mà bài đang dạy"
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^Sổ đang dùng: /Users/lan/du-an/so\.txt\nBản sao lưu: /Users/lan/du-an/sao-luu/so\.txt\nTrong du-an: cà phê,25000\nTrong sao-luu: bún bò,40000\s*$
- tier: output
  expect: "Bản sao lưu: /Users/lan/du-an/sao-luu/so.txt"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dấu gạch bạn gõ, và máy tự biết phải đặt dấu nào ở chỗ nối.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đường dẫn của bạn giờ không phụ thuộc **chỗ đứng** — vì nó kể từ gốc — và
không phụ thuộc **dấu ngăn** của hệ máy — vì `pathlib` tự chọn dấu.

Một chỗ vẫn còn nợ, và nói ra cho sòng phẳng: `pathlib` không cấp cho bạn cái
ổ `C:` mà Windows cần. Đưa `/Users/lan/du-an` cho nó trên Windows thì nó ghép
ra `\Users\lan\du-an\so.txt` — đúng dấu ngăn, mà vẫn thiếu ổ. Cái gốc thật
sự của mỗi máy là chuyện máy ấy tự biết, không phải chuyện một dòng code đoán
được; bài sau đưa bạn cách hỏi máy chỗ ấy thay vì tự viết ra.

Nhưng nhìn lại dòng bạn vừa viết mà xem. Cái tên `"so.txt"` vẫn nằm cứng trong
code. Lan muốn xem sổ tháng trước — cuốn `so-thang-2.txt` nằm ngay cạnh đó — thì
cách duy nhất là mở file code ra, tìm đúng dòng ấy, sửa một câu chữ, lưu lại,
rồi chạy. Mà Lan không biết Python, và bạn thì không ngồi cạnh Lan cả ngày.

Người dùng phải nói được cho chương trình biết họ muốn mở cuốn nào, mà không
phải chạm vào code. Nói bằng đường nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
