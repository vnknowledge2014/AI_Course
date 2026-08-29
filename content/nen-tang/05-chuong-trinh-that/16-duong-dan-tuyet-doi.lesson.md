---
id: nen-tang.chuong-trinh-that.duong-dan-tuyet-doi
title: Đường dẫn kể từ gốc
summary: "Một đường dẫn bắt đầu bằng dấu gạch chéo là đường dẫn kể từ gốc — nó trỏ đúng một chỗ trên máy, dù người gõ lệnh đang đứng ở đâu."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.absolute-path]
requires: [mod.import, mod.dotted-access, io.relative-path, err.file-not-found, core.with-open, core.file-write, core.file-read, core.duong-dan, core.thu-muc, core.file, core.string-sequence, core.string-literal, core.string-concat, core.variable, core.assignment, core.output, core.fstring]
concepts: [core.duong-dan, core.thu-muc, core.goc-cua-may]
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
Có kiểu chỉ đường đi từ chỗ bạn đứng. Và có kiểu chỉ đường đi từ đầu thành phố.
::::

::::explain{#hai-kieu-chi-duong}
Bài trước bạn mượn được cái hộp `os` và hỏi máy một câu mà trước đó không có
cách nào hỏi: *tôi đang đứng ở đâu*. Máy trả lời `/Users/lan/Desktop`.

Rồi câu trả lời ấy lại đẻ ra một chỗ đau. Cuốn sổ của Lan nằm trong thư mục
`du-an`, không nằm trên Desktop. Chương trình viết `open("so.txt")`, máy đi tìm
`so.txt` **ngay tại chỗ đang đứng** — tức là trên Desktop — và không thấy gì cả.
Chuyển sang ngồi gõ ở một thư mục khác thì cùng dòng code ấy lại đi tìm ở một
chỗ khác nữa. Cái tên `"so.txt"` không tự nó nói được cuốn sổ nằm đâu.

Hãy nghĩ tới chuyện chỉ đường ngoài đời. Có hai kiểu:

- **Kiểu thứ nhất — chỉ từ chỗ người hỏi đang đứng:** "đi thẳng, tới ngã tư thứ
  hai rẽ trái". Lời chỉ đường này đúng với người đang đứng cạnh bạn, và sai với
  mọi người khác. Đứng chỗ khác thì cũng lời ấy dẫn tới một quán khác.
- **Kiểu thứ hai — chỉ từ một mốc chung của cả thành phố:** "số 12 đường Lê Lợi,
  phường Bến Nghé, Quận 1". Ai đang ở đâu cũng tìm ra đúng một chỗ. Lời chỉ
  đường này không nhắc gì tới người nghe.

Máy tính có đúng hai kiểu ấy, và cái tên `"so.txt"` bạn dùng nãy giờ là kiểu
thứ nhất. Kiểu thứ hai gọi là **đường dẫn tuyệt đối**.

Đường dẫn tuyệt đối viết ra từ **gốc** của máy — chỗ mà mọi thư mục khác đều
nằm bên trong. Trên macOS và Linux, gốc là một dấu gạch chéo:

```text title=readonly
/Users/lan/du-an/so.txt
```

Đọc từ trái sang: bắt đầu ở gốc `/`, vào `Users`, vào `lan`, vào `du-an`, và ở
đó có một file tên `so.txt`. Không có chỗ nào trong câu đó phụ thuộc vào việc
bạn đang ngồi gõ ở đâu — nên nó trỏ đúng một chỗ, và trỏ đúng chỗ ấy mãi.

Nhìn bằng mắt cũng phân biệt được hai kiểu, và dấu hiệu nằm ở **ký tự đầu tiên**:

- `"so.txt"` và `"du-an/so.txt"` — không bắt đầu bằng `/`, nên máy hiểu là "tính
  từ chỗ đang đứng". Đây là đường dẫn **tương đối**, thứ bài trước vừa nói tới.
- `"/Users/lan/du-an/so.txt"` — bắt đầu bằng `/`, nên máy hiểu là "tính từ gốc".

Máy Windows cũng có gốc, chỉ khác hình dạng: nó bắt đầu bằng tên ổ đĩa, ví dụ
`C:\Users\lan\du-an\so.txt`. Ý thì y hệt — kể từ một điểm mà cả máy đều biết,
chứ không kể từ chỗ bạn đứng.
::::

::::example{#mo-so-bang-duong-dan-tuyet-doi}
Đây là cuốn sổ của Lan, ghi và đọc lại bằng đường dẫn tuyệt đối.

```python title=readonly
import os

# Giàn giáo cho ví dụ chạy được: dựng sẵn thư mục du-an trên máy của Lan.
# Nó không phải thứ bài này dạy, cứ coi như thư mục đã có sẵn ở đó.
os.makedirs("/Users/lan/du-an", exist_ok=True)

with open("/Users/lan/du-an/so.txt", "w") as f:
    f.write("cà phê,25000\n")

with open("/Users/lan/du-an/so.txt", "r") as f:
    print("Đọc được:", f.read().strip())

duong_dan = "/Users/lan/du-an/so.txt"
print("Ký tự đầu tiên:", duong_dan[0])
```

Máy in ra:

```text title=readonly
Đọc được: cà phê,25000
Ký tự đầu tiên: /
```

Ba chỗ đáng dừng lại nhìn:

- **Không có dòng nào hỏi "tôi đang đứng ở đâu".** Chương trình này không cần
  biết điều đó nữa. Bạn chạy nó từ Desktop, từ thư mục `du-an`, hay từ chỗ nào
  khác trong máy, nó vẫn ghi vào đúng một file.
- **Đường dẫn vẫn là một câu chữ bình thường.** Nó nằm trong dấu nháy, đặt được
  vào một cái tên, và lấy được ký tự đầu bằng `duong_dan[0]` y như mọi chuỗi từ
  T1.1 tới giờ. Cái mới nằm ở **nội dung** câu chữ ấy, không ở kiểu của nó.
- **Ký tự đầu là `/`.** Đây là toàn bộ khác biệt giữa hai kiểu chỉ đường, gói
  vào một ký tự: có nó thì máy đếm từ gốc, không có nó thì máy đếm từ chỗ đứng.
::::

::::predict{#doan-hai-duong-dan commitOnce}
Lan đang ngồi gõ ở `/Users/lan/Desktop`. Trên Desktop **không có** thư mục nào
tên `du-an`; cuốn sổ nằm ở `/Users/lan/du-an/so.txt`.

Byte viết hai đường dẫn trỏ tới cùng cuốn sổ ấy — theo ý Byte — rồi mở lần lượt.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
duong_dan_a = "du-an/so.txt"
duong_dan_b = "/Users/lan/du-an/so.txt"

with open(duong_dan_a, "r") as f:
    print("A đọc được:", f.read().strip())

with open(duong_dan_b, "r") as f:
    print("B đọc được:", f.read().strip())
```

:::opt{correct}
Không dòng nào được in ra — máy dừng ngay ở chỗ mở `duong_dan_a` với
`FileNotFoundError`
:::

:::opt
Cả hai dòng đều in ra nội dung cuốn sổ, vì hai đường dẫn cùng trỏ tới một file
::why
Gần đúng ở chỗ bạn đọc đúng **ý định** của Byte: Byte viết hai đường dẫn ấy để
chúng cùng trỏ tới một cuốn sổ, và trong đầu Byte thì đúng là như vậy.

Chỗ lệch nằm ở chỗ Lan đang đứng. `"du-an/so.txt"` không bắt đầu bằng `/`, nên
máy tính nó từ chỗ đang đứng — tức là đi tìm `/Users/lan/Desktop/du-an/so.txt`.
Trên Desktop không có thư mục `du-an` nào, nên chỗ đó rỗng. Chỉ `duong_dan_b`
mới trỏ tới cuốn sổ thật, nhưng chương trình đã dừng trước khi tới nó.
::
:::

:::opt
`A đọc được: cà phê,25000` rồi mới báo lỗi ở `duong_dan_b`
::why
Gần đúng ở chỗ bạn theo dõi đúng thứ tự chạy: máy đi từ trên xuống, nên nếu có
một dòng in ra được thì đó phải là dòng A.

Chỗ lệch là ở việc đường dẫn nào tìm ra file. `duong_dan_b` viết đủ cả đường từ
gốc `/` tới cuốn sổ, nên nó là đường dẫn **tìm ra**. Còn `duong_dan_a` mới là
đường dẫn hụt, vì nó ngầm thêm chỗ đứng `/Users/lan/Desktop` vào phía trước.
Hai vai đang bị đổi cho nhau.
::
:::

:::opt
`A đọc được:` với phần nội dung để trống, rồi `B đọc được: cà phê,25000`
::why
Gần đúng ở chỗ bạn nhớ một hành vi có thật của `open`: ở chế độ `"w"` và chế độ
`"a"`, mở một file chưa có thì máy **tạo** file ấy ra, và lúc đó nội dung đúng
là rỗng. Bạn nhớ không sai.

Chỗ lệch là chế độ ở đây là `"r"` — chỉ đọc. Đọc thì không có gì để tạo ra cả:
máy không thể bịa nội dung cho một file chưa từng tồn tại, nên nó dừng và nói
`FileNotFoundError`, đúng như bài 13 đã bàn.
::
:::
::::

::::code{#viet-hai-duong-dan-tu-goc}
Máy của Lan có hai chỗ, và mỗi chỗ giữ một cuốn sổ **khác nhau** dù hai cuốn
trùng tên `so.txt`:

- trong thư mục dự án `/Users/lan/du-an` là cuốn sổ đang dùng;
- trên Desktop `/Users/lan/Desktop` là một bản cũ Lan để quên từ tháng trước.

Byte đưa sẵn ba câu chữ: tên hai thư mục và tên file. Việc của bạn là viết ra
hai đường dẫn tuyệt đối, mỗi cái trỏ tới đúng một trong hai cuốn.

Chú ý một chuyện: hai thư mục viết không có dấu gạch ở cuối, còn tên file viết
không có dấu gạch ở đầu — nên chỗ nối giữa chúng đang thiếu đúng một dấu gạch,
và bạn phải tự đặt nó vào.

```python title=starter
import os

# Giàn giáo: dựng sẵn hai thư mục trên máy của Lan.
os.makedirs("/Users/lan/du-an", exist_ok=True)
os.makedirs("/Users/lan/Desktop", exist_ok=True)

thu_muc_du_an = "/Users/lan/du-an"
thu_muc_desktop = "/Users/lan/Desktop"
ten_so = "so.txt"

duong_dan_so = ___
duong_dan_ban_cu = ___

with open(duong_dan_so, "w") as f:
    f.write("cà phê,25000\n")

with open(duong_dan_ban_cu, "w") as f:
    f.write("bún bò,40000\n")

# Byte kiểm lại bằng cách tự mình mở đúng hai chỗ mà hai cuốn sổ phải nằm.
with open("/Users/lan/du-an/so.txt", "r") as f:
    print("Trong du-an:", f.read().strip())

with open("/Users/lan/Desktop/so.txt", "r") as f:
    print("Trên Desktop:", f.read().strip())

print(f"Ký tự đầu của hai đường dẫn: {duong_dan_so[0]} và {duong_dan_ban_cu[0]}")
```

```python title=solution
import os

# Giàn giáo: dựng sẵn hai thư mục trên máy của Lan.
os.makedirs("/Users/lan/du-an", exist_ok=True)
os.makedirs("/Users/lan/Desktop", exist_ok=True)

thu_muc_du_an = "/Users/lan/du-an"
thu_muc_desktop = "/Users/lan/Desktop"
ten_so = "so.txt"

duong_dan_so = thu_muc_du_an + "/" + ten_so
duong_dan_ban_cu = thu_muc_desktop + "/" + ten_so

with open(duong_dan_so, "w") as f:
    f.write("cà phê,25000\n")

with open(duong_dan_ban_cu, "w") as f:
    f.write("bún bò,40000\n")

# Byte kiểm lại bằng cách tự mình mở đúng hai chỗ mà hai cuốn sổ phải nằm.
with open("/Users/lan/du-an/so.txt", "r") as f:
    print("Trong du-an:", f.read().strip())

with open("/Users/lan/Desktop/so.txt", "r") as f:
    print("Trên Desktop:", f.read().strip())

print(f"Ký tự đầu của hai đường dẫn: {duong_dan_so[0]} và {duong_dan_ban_cu[0]}")
```

```python title=test
# Mỗi chỗ trống có ít nhất hai câu chấm chạm tới nó, và hai cuốn sổ cố tình
# giữ nội dung KHÁC nhau: điền cùng một đường dẫn vào cả hai chỗ thì lần ghi
# thứ hai đè lên lần thứ nhất, và câu kiểm nội dung của du-an sẽ vỡ.
assert duong_dan_so[0] == "/", "đường dẫn tuyệt đối kể từ gốc, nên duong_dan_so phải bắt đầu bằng dấu gạch chéo, không phải bằng chữ s của so.txt"
assert duong_dan_so == "/Users/lan/du-an/so.txt", "cuốn sổ đang dùng nằm trong thư mục du-an, nên duong_dan_so phải đúng bằng /Users/lan/du-an/so.txt — thiếu dấu gạch giữa thư mục và tên file thì ra /Users/lan/du-anso.txt, thừa một dấu thì ra /Users/lan/du-an//so.txt"
assert duong_dan_ban_cu[0] == "/", "bản cũ cũng phải được gọi bằng đường dẫn kể từ gốc, nên duong_dan_ban_cu cũng phải bắt đầu bằng dấu gạch chéo"
assert duong_dan_ban_cu == "/Users/lan/Desktop/so.txt", "bản cũ nằm trên Desktop, nên duong_dan_ban_cu phải đúng bằng /Users/lan/Desktop/so.txt — nếu nó trùng với duong_dan_so thì hai cuốn sổ đã bị ghi đè lên nhau"

with open("/Users/lan/du-an/so.txt", "r") as f:
    assert f.read() == "cà phê,25000\n", "file /Users/lan/du-an/so.txt phải giữ dòng cà phê,25000 — nếu nó đang giữ dòng bún bò thì duong_dan_ban_cu đã trỏ nhầm vào thư mục du-an"

with open("/Users/lan/Desktop/so.txt", "r") as f:
    assert f.read() == "bún bò,40000\n", "file /Users/lan/Desktop/so.txt phải giữ dòng bún bò,40000 — nếu nó đang giữ dòng cà phê thì duong_dan_so đã trỏ nhầm lên Desktop"
```

:::hints
- kind: attention
  body: Nhìn ba câu chữ Byte đưa sẵn và ghép thử bằng mắt trước đã. `thu_muc_du_an` dừng lại ngay sau chữ `du-an`, còn `ten_so` bắt đầu ngay ở chữ `s`. Dán thẳng hai câu ấy vào nhau thì chỗ nối trông thế nào?
- kind: strategy
  body: Bạn cần một câu chữ dài hơn, ghép từ ba mảnh: tên thư mục, một dấu gạch chéo, rồi tên file. Phép nối chuỗi bằng dấu cộng đã có từ Realm 0 và dùng được ở đây. Hai chỗ trống khác nhau đúng một mảnh — mảnh thư mục — nên đừng dùng cùng một tên thư mục cho cả hai, làm vậy thì hai cuốn sổ sẽ nằm chồng lên nhau ở một chỗ.
- kind: one-line
  body: 'Chỗ trống thứ nhất viết `thu_muc_du_an + "/" + ten_so`, chỗ thứ hai viết `thu_muc_desktop + "/" + ten_so`. Viết thẳng cả câu `"/Users/lan/du-an/so.txt"` cũng là một đáp án đúng — nó cũng là đường dẫn kể từ gốc.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^Trong du-an: cà phê,25000\nTrên Desktop: bún bò,40000\nKý tự đầu của hai đường dẫn: / và /\s*$
- tier: output
  expect: "Trên Desktop: bún bò,40000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cuốn sổ trùng tên, hai chỗ khác nhau, và giờ máy phân biệt được chúng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đường dẫn kể từ gốc vừa gỡ được chỗ đau lớn nhất: chương trình không còn phụ
thuộc vào chỗ bạn ngồi gõ nữa.

Nhưng để ý dòng bạn vừa viết. Bạn viết cứng `/Users/lan/du-an/so.txt` — và câu
chữ ấy chỉ đúng trên máy của Lan. Máy Windows của bạn cùng lớp dùng dấu `\` và
ổ `C:`, nên cùng cuốn sổ ấy ở bên đó có tên là `C:\Users\lan\du-an\so.txt`.
Còn phép ghép bằng dấu cộng thì mong manh theo một kiểu khác: bạn phải tự nhớ
đặt đúng **một** dấu gạch vào chỗ nối — quên thì thiếu, mà thư mục nào đã sẵn
một dấu gạch ở cuối thì lại thừa.

Có thứ nào biết ghép đường dẫn đúng cho mọi máy không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
