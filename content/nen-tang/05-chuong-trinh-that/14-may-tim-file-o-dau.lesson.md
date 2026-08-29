---
id: nen-tang.chuong-trinh-that.may-tim-file-o-dau
title: Máy tìm file ở đâu
summary: Một cái tên file viết trần là địa chỉ chưa xong; máy hoàn tất nó bằng thư mục bạn đang đứng lúc gõ lệnh, không phải bằng chỗ file .py nằm.
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [io.relative-path]
requires: [err.file-not-found, core.file, core.thu-muc, core.duong-dan, core.terminal, core.function-def, core.function-return, core.function-parameter, core.docstring, core.fstring, core.output, core.string-literal, core.variable, core.assignment]
concepts: [core.duong-dan, core.cho-dang-dung, core.file]
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
Cái tên bạn viết trong nháy chưa phải một chỗ. Nó mới là nửa cái địa chỉ.
::::

::::explain{#nua-cai-dia-chi}
Bài trước để lại một chuyện khó chịu: `so.txt` nằm ngay cạnh `main.py`, cùng
một thư mục, bạn nhìn thấy nó bằng mắt — mà chương trình vẫn báo chưa có sổ.

Nhánh `except FileNotFoundError:` không hỏng. Nó đỡ đúng thứ nó phải đỡ. Vấn
đề nằm ở chỗ khác: **máy thật sự không tìm thấy gì**, chỉ là nó không tìm ở
chỗ bạn tưởng.

Nhớ lại cái hộp có tên ở Realm 0: một file gồm nội dung, một cái tên, và một
**chỗ đứng** — thư mục chứa nó. Ba phần, không phải hai. Khi bạn viết:

```python title=readonly
open("so.txt", "r")
```

bạn mới đưa cho máy đúng phần **tên**. Phần chỗ đứng bạn chưa nói. Và một cái
tên trần không đủ để trỏ tới một file: trong máy Lan có thể có bốn cái tên
`so.txt` ở bốn thư mục khác nhau, và chúng là bốn file khác nhau.

Cái tên viết kiểu này gọi là **đường dẫn tương đối** — tương đối với một chỗ
nào đó. Máy phải tự điền nốt phần còn thiếu trước khi đi tìm. Câu hỏi của cả
bài gói vào đúng một chỗ: **nó điền bằng thư mục nào?**

Có hai ứng viên, và cả hai đều nghe hợp lý:

- thư mục chứa file `main.py` — vì dòng `open` được viết trong file ấy;
- thư mục bạn đang đứng trong terminal lúc gõ lệnh chạy.

Chỉ một trong hai là thật.
::::

::::predict{#doan-may-hoan-tat-bang-thu-muc-nao commitOnce}
Trên máy Lan, hai file nằm cạnh nhau trong cùng một thư mục:

```text title=readonly
/Users/lan/du-an/main.py     ← chương trình
/Users/lan/du-an/so.txt      ← cuốn sổ, nằm ngay cạnh
```

Nội dung `main.py`:

```python title=readonly
with open("so.txt", "r") as f:
    noi_dung = f.read()

print(f"Sổ dài {len(noi_dung)} ký tự.")
```

Lan mở terminal ở màn hình nền rồi gõ:

```text title=readonly
lan@may:/Users/lan$ python du-an/main.py
```

**Trước khi bấm Enter**, bạn đoán máy đi mở file nào?

:::opt{correct}
`/Users/lan/so.txt` — không có file nào ở đó, nên `FileNotFoundError`
:::

:::opt
`/Users/lan/du-an/so.txt` — chương trình chạy trơn tru
::why
Gần đúng ở chỗ bạn đang theo một lối nghĩ rất chắc chắn, và là lối nghĩ mà
phần lớn người viết chương trình lần đầu đều theo: dòng `open("so.txt")` được
gõ trong `main.py`, nên "ở đây" nghe như phải là chỗ `main.py` đang nằm.

Chỗ lệch là máy không bao giờ hỏi câu "file `.py` này nằm ở đâu" để hoàn tất
đường dẫn. Nó hoàn tất bằng **thư mục bạn đang đứng lúc gõ lệnh** — ở lượt
chạy này là `/Users/lan`. Đúng file `main.py` ấy, chép sang bất kỳ đâu
cũng vậy: đứng chỗ nào thì `so.txt` được tìm ở chỗ đó. Điều đó nghe kỳ, nhưng
nó chính là thứ cho phép một chương trình đọc cuốn sổ của **thư mục bạn đang
mở**, thay vì mãi mãi đọc đúng cuốn nằm cạnh nó.
::
:::

:::opt
Máy tìm ở `/Users/lan/du-an` trước, không thấy thì tìm sang `/Users/lan`
::why
Gần đúng ở chỗ bạn đang thiết kế một hành vi tử tế, và nhiều công cụ khác
ngoài đời đúng là làm vậy thật.

Chỗ lệch là máy không đoán hộ, và Realm 0 đã chốt điều này từ bài thứ ba: máy
chờ, nó không tự bổ sung ý bạn quên nói. Tìm ở nhiều chỗ nghĩa là khi hai thư
mục cùng có một `so.txt`, cuốn sổ được mở sẽ đổi tuỳ hôm — và bạn không có
cách nào biết mình vừa cộng tiền của cuốn nào. Một chỗ tìm duy nhất thì kết
quả buồn cười cũng còn đoán được; hai chỗ thì không.
::
:::

:::opt
Máy quét cả ổ đĩa tìm file tên `so.txt`, thấy đúng một cái nên mở được
::why
Gần đúng ở chỗ bạn nhìn ra một chuyện thật: cái tên `so.txt` trần trụi thì
đúng là chưa nói được file nào, nên phải có thêm thông tin từ đâu đó.

Chỗ lệch là thông tin thêm ấy đến từ chỗ bạn đang đứng, không đến từ một cuộc
tìm kiếm. Quét cả ổ đĩa cho mỗi lần `open` sẽ chậm tới mức không dùng nổi, và
tệ hơn: hôm nay ổ đĩa có một `so.txt` nên chương trình chạy, mai bạn tải về
một `so.txt` khác là nó mở nhầm cuốn. Ghép tên với chỗ đứng thì luôn ra đúng
một chỗ, và chỗ ấy nói ra được trước khi chạy.
::
:::
::::

::::explain{#may-hoan-tat-bang-cho-dang-dung}
Đáp án là ứng viên thứ hai: **thư mục bạn đang đứng trong terminal lúc gõ
lệnh**. Người ta gọi nó là *chỗ đang đứng* của chương trình.

Máy hoàn tất địa chỉ theo đúng một phép rất thẳng: lấy chỗ đang đứng, đặt cái
tên bạn viết vào sau nó, ngăn giữa bằng một dấu gạch.

Cùng một `main.py`, hai lần chạy từ hai chỗ khác nhau:

```text title=readonly
lan@may:/Users/lan/du-an$ python main.py
    → máy mở  /Users/lan/du-an/so.txt      ✓ có, chạy được

lan@may:/Users/lan$ python du-an/main.py
    → máy mở  /Users/lan/so.txt            ✗ không có, FileNotFoundError
```

File `main.py` không đổi một chữ. `so.txt` không hề đi đâu. Thứ đổi là **chỗ
bạn đứng lúc gõ lệnh**, và chỉ riêng nó đã đủ làm chương trình sống hay chết.

Nhìn kỹ dòng thứ hai một lần nữa: `du-an/main.py` cũng là một đường dẫn tương
đối, và nó cũng được hoàn tất từ `/Users/lan` — nên máy tìm thấy `main.py`
không chút vướng mắc. Cùng một luật, áp cho cả hai. Máy tìm ra
chương trình, rồi tìm hụt cuốn sổ, vì hai cái tên được viết theo hai kiểu khác
nhau: một cái có nói đường đi, một cái không.

Ở bước sau bạn viết lại phép hoàn tất ấy bằng tay, để nhìn cho rõ. Máy làm
chuyện này bên trong `open`, không ai phải gõ nó ra; ta gõ ra một lần, đúng
một lần, để thấy nó là phép ghép chứ không phải phép màu.
::::

::::code{#hoan-tat-cai-dia-chi}
Viết hàm `may_tim_o_dau`: nhận vào **chỗ đang đứng** và **cái tên bạn viết
trong code**, cho ra đường dẫn đầy đủ mà máy sẽ đi mở.

Ba lượt gọi bên dưới lần lượt là: đứng trong thư mục dự án, đứng ở màn hình
nền, và một cái tên có kèm thư mục con.

```python title=starter
def may_tim_o_dau(cho_dang_dung, ten_da_viet):
    """Hoàn tất một đường dẫn tương đối bằng chỗ đang đứng."""
    return ___


print(may_tim_o_dau("/Users/lan/du-an", "so.txt"))
print(may_tim_o_dau("/Users/lan/Desktop", "so.txt"))
print(may_tim_o_dau("/Users/lan/du-an", "thang-truoc/so.txt"))
```

```python title=solution
def may_tim_o_dau(cho_dang_dung, ten_da_viet):
    """Hoàn tất một đường dẫn tương đối bằng chỗ đang đứng."""
    return f"{cho_dang_dung}/{ten_da_viet}"


print(may_tim_o_dau("/Users/lan/du-an", "so.txt"))
print(may_tim_o_dau("/Users/lan/Desktop", "so.txt"))
print(may_tim_o_dau("/Users/lan/du-an", "thang-truoc/so.txt"))
```

```python title=test
# Ba lượt gọi, và mỗi lượt bịt một cách qua bài nhờ ăn may:
#
#   lượt 1 : chốt thứ tự — ghép ngược thành "so.txt/..." là vỡ ngay ở đây
#   lượt 2 : đổi chỗ đứng mà giữ nguyên tên — một câu trả lời chép cứng
#            "/Users/lan/du-an/so.txt" qua được lượt 1 nhưng chết ở lượt này
#   lượt 3 : đổi tên mà giữ nguyên chỗ đứng, và cái tên lần này có sẵn một
#            dấu gạch bên trong — chép cứng chỗ nào cũng vỡ, và một lời giải
#            quên hẳn dấu gạch nối cũng vỡ ở cả ba lượt
assert may_tim_o_dau("/Users/lan/du-an", "so.txt") == "/Users/lan/du-an/so.txt", "đứng trong /Users/lan/du-an mà viết so.txt thì máy đi mở /Users/lan/du-an/so.txt — chỗ đứng đứng trước, cái tên đứng sau, giữa hai phần có đúng một dấu gạch"
assert may_tim_o_dau("/Users/lan/Desktop", "so.txt") == "/Users/lan/Desktop/so.txt", "vẫn cái tên so.txt ấy, nhưng đứng ở /Users/lan/Desktop thì máy đi mở /Users/lan/Desktop/so.txt — đổi chỗ đứng là đổi file"
assert may_tim_o_dau("/Users/lan/du-an", "thang-truoc/so.txt") == "/Users/lan/du-an/thang-truoc/so.txt", "cái tên viết trong code có thể mang sẵn một thư mục con; nó vẫn được ghép nguyên vẹn vào sau chỗ đang đứng"
```

:::hints
- kind: attention
  body: Hàm nhận về hai mảnh và phải cho ra một mảnh. Nhìn lại ba dòng kết quả mong đợi trong khối kiểm: mỗi dòng ấy đọc từ trái sang là chỗ đang đứng, rồi một dấu gạch, rồi cái tên đã viết. Không mảnh nào bị bỏ đi, không mảnh nào đảo chỗ.
- kind: strategy
  body: Bạn đang dựng một câu chữ từ hai giá trị đang nằm trong hai cái tên, và f-string sinh ra đúng để làm việc đó — đặt mỗi cái tên vào một cặp ngoặc nhọn, và gõ dấu gạch vào giữa hai cặp ấy như gõ một chữ bình thường. Đừng gõ thẳng `/Users/lan/du-an` vào lời giải: lượt gọi thứ hai đứng ở một chỗ khác, và nó sẽ nói ngay.
- kind: one-line
  body: "Viết `f\"{cho_dang_dung}/{ten_da_viet}\"` vào chỗ trống."
:::

:::validate
- tier: static
  onFail: lời giải phải ĐỌC cả hai mảnh nhận vào, không chép cứng chỗ đứng hay tên file nào
  requireAst:
  # Hai cái tên đều phải được đọc. Một câu trả lời chép cứng chỉ nhắc tới
  # nhiều nhất một trong hai, và luật này chặn nó ngay trước khi chạy — sớm
  # hơn ba câu assert, nên người học nhận được đúng lời nhắc cần nghe.
  - kind: uses-name, target: cho_dang_dung, min: 1
  - kind: uses-name, target: ten_da_viet, min: 1
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^/Users/lan/du-an/so\.txt\n/Users/lan/Desktop/so\.txt\n/Users/lan/du-an/thang-truoc/so\.txt\s*$
- tier: output
  expect: "/Users/lan/Desktop/so.txt"
- tier: output
  expect: "/Users/lan/du-an/thang-truoc/so.txt"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một cái tên, hai chỗ đứng, hai cuốn sổ. Giờ thì nói ra được vì sao.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Luật đã rõ: máy hoàn tất cái tên bạn viết bằng chỗ bạn đang đứng. Muốn biết
chương trình sắp mở file nào, bạn cần biết chỗ đứng ấy là chỗ nào.

Và đây là lần đầu tiên kể từ bài `print` đầu tiên của Realm 0 mà bạn bí thật.
Vốn liếng Python của bạn tới giờ gồm `print`, `input`, `open`, `int`, `len`,
`sorted`, `def`, `try` — hãy thử điểm lại từng cái một: không cái nào hỏi được
câu "tôi đang đứng ở đâu".

Vậy "chỗ đang đứng" ngay lúc này là chỗ nào? Trong tay bạn chưa có một lệnh
nào hỏi được câu đó. Lấy công cụ ấy ở đâu ra?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
