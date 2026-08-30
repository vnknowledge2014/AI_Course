---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.ba-cach-don-dep
title: "Ba cách dọn, ba cái giá phải trả"
summary: "Tự tay dọn, đếm thẻ rồi gom rác, hay để trình dịch quyết trước khi chạy — không cách nào miễn phí, chỉ khác nhau ai trả giá và trả lúc nào."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [mem.memory-strategies]
requires: [mem.reference-cycle]
concepts: [mem.chien-luoc-don-dep, mem.vong-tro]
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
Bài trước hỏi ai trả giá gì. Giờ đặt cả ba cách cạnh nhau mà nhìn.
::::

::::explain{#khong-cach-nao-mien-phi}
Ba cách dọn bộ nhớ mà một chương trình có thể theo, xếp theo đúng thứ tự
bài này sẽ đi qua:

1. **Tự tay dọn** — người viết code tự gọi lệnh "trả lại chỗ này" đúng lúc
   giá trị hết dùng. Đây là cách ngôn ngữ C đi.
2. **Đếm thẻ, rồi gom rác khi cần** — máy tự đếm, tự dọn phần đếm-thẻ-về-
   không ngay lập tức, và có thêm một bộ quét theo đợt để vá chỗ vòng tham
   chiếu. Đây là cách Python đi, và hai bài trước vừa nhìn tận mắt.
3. **Để trình dịch quyết trước khi chạy** — không có gì diễn ra lúc chương
   trình đang chạy cả; mọi chuyện "ai dọn, dọn lúc nào" đã được tính xong
   từ trước, ngay khi mã nguồn được biên dịch. Đây là cách Rust đi, vừa hé
   ở cuối bài trước.

Ba cách này không phải "cách nào hay hơn cách nào". Mỗi cách chuyển gánh
nặng sang một chỗ khác, và bài này chỉ ra đúng chỗ đó — ai è cổ chịu, và
chịu vào lúc nào.
::::

::::explain{#cai-gia-cua-tu-tay}
Cách 1 — tự tay dọn — đặt toàn bộ trách nhiệm lên người viết code. Không có
máy nào theo dõi hộ.

Cái giá lộ ra ở đúng hai chỗ dễ vấp nhất:

- **Quên dọn một chỗ** → chỗ đó không bao giờ được trả lại, chương trình
  chạy càng lâu càng chiếm thêm bộ nhớ, dù không dùng tới. Đây gọi là
  **rò rỉ bộ nhớ** (*memory leak*).
- **Dọn một chỗ hai lần**, hoặc dọn xong còn dùng lại — chương trình sập,
  và thường sập ở một chỗ xa tít, chẳng liên quan gì tới dòng gõ sai.

Không trình dịch nào của C bắt được hai lỗi này TRƯỚC khi chạy. Chúng chỉ
lộ ra khi chương trình đang chạy thật, có khi là ở máy của người dùng, sau
hàng giờ hoạt động êm ru.
::::

::::explain{#cai-gia-cua-python}
Cách 2 — cách Python đang đi — trả giá ở hai chỗ khác hẳn, và cả hai đều đã
lộ diện trong hai bài trước.

Đếm thẻ trả một cái giá NHỎ nhưng LIÊN TỤC: mỗi giá trị mang thêm một con
số phải cập nhật ở MỌI lần một cái tên trỏ vào nó hay rời khỏi nó. Con số
đó không hề miễn phí — nó cần một chỗ chứa riêng, và cần được cộng, trừ ở
mọi thao tác gán. Bù lại, phần lớn giá trị được dọn NGAY khi hết thẻ, không
cần chờ đợt nào cả — đúng như bài `khi-khong-con-the-nao-tro-toi` đã thấy.

Gom rác theo vòng trả một cái giá THEO ĐỢT: nó phải tạm dừng lại, quét qua
một phần bộ nhớ để tìm những cụm giữ thẻ trỏ vào nhau. Việc quét đó tốn
thời gian thật — không nhiều cho một chương trình nhỏ, nhưng không phải
không đồng nào.

Không ai phải nhớ gọi `.free()` hay tự tay dọn gì cả. Cái giá chuyển từ
"người viết phải nhớ" sang "máy phải làm việc, ngay cả khi không ai để ý".
::::

::::example{#hai-kich-ban-cung-mot-vong}
Đặt cạnh nhau hai kịch bản, cùng dựng một vòng hai phần tử, chỉ khác đúng
một việc: có tự tay cắt vòng trước khi buông tên hay không.

```python title=readonly
import gc

# Kịch bản 1 — không đụng tới gì cả, để mặc bộ dọn theo vòng lo
gc.collect()
a = []
b = []
a.append(b)
b.append(a)
a = None
b = None
so1 = gc.collect()
print(f"kịch bản 1 — gc phải tự tay dọn: {so1}")

# Kịch bản 2 — TỰ TAY cắt một đầu của vòng, trước khi buông tên
gc.collect()
c = []
d = []
c.append(d)
d.append(c)
c[:] = []          # cắt đứt phần c đang giữ d — như kỷ luật của cách 1
c = None
d = None
so2 = gc.collect()
print(f"kịch bản 2 — gc phải tự tay dọn: {so2}")
```

```text title=readonly
kịch bản 1 — gc phải tự tay dọn: 2
kịch bản 2 — gc phải tự tay dọn: 0
```

Kịch bản 2 mượn đúng kỷ luật của cách 1 — tự tay cắt đứt sợi dây trước khi
buông tay — và làm ngay TRONG Python. Cắt một đầu là đủ: hết vòng khép kín
thì phần còn lại chỉ là một chuỗi một chiều, và đếm thẻ tự lo được, không
cần đợi bộ dọn theo vòng ghé qua nữa. `so2` về đúng 0.

Nhưng nhìn kỹ cái giá: kịch bản 2 phải THÊM một dòng, và người viết phải
NHỚ viết đúng dòng đó, đúng chỗ. Quên nó thì lại y hệt kịch bản 1 — không
sai, chỉ là phải chờ bộ dọn theo vòng ghé qua muộn hơn.
::::

::::predict{#doan-kich-ban-2 commitOnce}
Byte đổi tên biến rồi dựng lại đúng kịch bản 2:

```python title=readonly
import gc

gc.collect()
p = []
q = []
p.append(q)
q.append(p)

q[:] = []           # lần này cắt đầu NGƯỢC LẠI — phần q đang giữ p

p = None
q = None
so = gc.collect()
print(so)
```

**Trước khi xem đáp án**, bạn đoán `so` in ra bao nhiêu?

:::opt{correct}
0 — cắt đầu nào của vòng cũng đủ, không cần cắt cả hai.
:::

:::opt
2 — phải cắt đúng đầu mà `.append` gọi TRƯỚC (`p.append(q)`) thì mới tính.
::why
Gần đúng ở việc bạn nghĩ thứ tự có vai trò gì đó — cảm giác hợp lý, vì
nhiều chuyện trong lập trình đúng là quan tâm thứ tự.

Chỗ lệch: một vòng hai phần tử chỉ có ĐÚNG một sợi dây khép kín, đi qua cả
hai đầu. Cắt bất kỳ một đầu nào cũng làm sợi dây đó không còn khép kín nữa
— không quan trọng đầu đó được tạo ra trước hay sau. Phần còn lại chỉ còn
một chiều, và đếm thẻ dọn được ngay, không cần bộ dọn theo vòng.
::
:::

:::opt
2 — cắt một đầu chỉ làm vòng "mỏng" đi, vẫn còn dính nhau ở đầu kia.
::why
Gần đúng ở hình dung "còn dính ở đầu kia" — đúng là `q` vẫn giữ `p` sau
dòng cắt (dòng cắt chỉ đụng tới nội dung của `q`, không đụng tới `p`).

Chỗ lệch: "còn dính" ở MỘT đầu không còn là một VÒNG nữa — nó chỉ là một
chuỗi một chiều (q trỏ sang p, p không trỏ ngược lại). Đếm thẻ xử lý chuỗi
một chiều hoàn toàn bình thường, y hệt cách nó xử lý một cái tên trỏ vào
một cái nồi ở hai bài trước. Không cần vòng khép kín thì không cần bộ dọn
theo vòng.
::
:::

:::opt
1 — chỉ dọn được phần tử bị cắt, phần tử còn lại vẫn phải chờ.
::why
Gần đúng ở việc tách hai phần tử ra để đếm riêng — cách nghĩ đó không sai
ở nhiều bài toán khác.

Chỗ lệch: một khi vòng đã bị cắt, cả `p` lẫn `q` đều hết thẻ vào ĐÚNG lúc
tên bên ngoài bị gỡ, chỉ khác nhau đúng MỘT bước tính toán bên trong (Python
dọn phần tử này rồi tiện tay dọn luôn phần tử kia, vì nó vừa hết thẻ ngay
sau đó). Không phần tử nào bị bỏ lại một mình.
::
:::
::::

::::code{#tu-tay-cat-mot-dau}
Đến lượt bạn. Hai cái nồi `c` và `d` sắp bị khép thành một vòng. Trước khi
buông tên, hãy tự tay cắt đứt MỘT đầu — chỉ một đầu là đủ.

```python title=starter
import gc

gc.collect()

c = []
d = []
c.append(d)
d.append(c)

___                  # cắt đứt MỘT đầu của vòng — một đầu là đủ

c = None
d = None

so_gc_phai_don = gc.collect()
print(f"gc còn phải dọn: {so_gc_phai_don}")
```

```python title=solution
import gc

gc.collect()

c = []
d = []
c.append(d)
d.append(c)

c[:] = []

c = None
d = None

so_gc_phai_don = gc.collect()
print(f"gc còn phải dọn: {so_gc_phai_don}")
```

```python title=test
# Cắt đúng một đầu (c hoặc d, không cần cả hai) thì đếm thẻ tự lo hết —
# gc không còn gì để dọn. Không cắt gì, hoặc cắt nhầm bằng cách gán một
# danh sách MỚI (c = []) thay vì sửa CHÍNH danh sách cũ, đều để vòng
# nguyên vẹn — gc sẽ vẫn thấy 2 vật mắc kẹt.
assert so_gc_phai_don == 0, f"cắt đúng một đầu của vòng thì gc.collect() không còn gì để dọn — ở đây nó vẫn thấy {so_gc_phai_don}, nghĩa là vòng chưa thật sự bị cắt"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ngay sau hai dòng `.append` đã khép vòng, và ngay TRƯỚC hai dòng gán `None`. Nó phải sửa nội dung của `c` hoặc `d` — không phải gán cho `c` hay `d` một danh sách MỚI.
- kind: strategy
  body: 'Sửa nội dung của một danh sách đang có, mà không đổi nó thành một danh sách khác, dùng cách đã quen: gán vào lát cắt toàn phần của nó, `ten[:] = gia_tri_moi`. Cho nó rỗng là cắt đứt mọi thứ nó đang giữ.'
  autoRevealAfterFailures: 2
- kind: one-line
  body: 'Viết `c[:] = []` (hoặc tương đương, `d[:] = []`) vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "gc còn phải dọn: 0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dòng, đúng chỗ, và bộ dọn theo vòng không còn việc gì để làm. Đó chính
là kỷ luật của cách 1, mượn tạm bên trong cách 2.
::::

::::explain{#cai-gia-cua-rust}
Cách 3 — để trình dịch quyết — trả giá ở một chỗ khác hẳn cả hai cách trên:
không phải lúc chương trình CHẠY, mà lúc nó được BIÊN DỊCH.

Rust bắt mỗi giá trị chỉ có đúng MỘT chủ tại một thời điểm — không có
chuyện hai tấm thẻ cùng lúc trỏ vào một nồi như Python vẫn cho phép tự do.
Luật đó khiến vòng tham chiếu kiểu bài trước không dựng lên nổi, trừ khi
người viết code CHỦ ĐỘNG xin phép ngoại lệ bằng những công cụ riêng — những
công cụ đó nằm ngoài phạm vi khoá này.

```rust title=readonly
// Ý TƯỞNG, chưa phải cú pháp thật đã dạy:
// mỗi giá trị có một chủ. Chủ hết phạm vi hoạt động (hết hàm, hết khối
// lệnh) thì trình dịch tự chèn lệnh dọn NGAY tại đó — không đếm thẻ lúc
// chạy, không có đợt quét nào cả. Chuyện dọn dẹp là một CÂU CHUYỆN đã kể
// xong từ lúc biên dịch, không phải việc còn phải làm khi chạy.
```

Cái giá ở đây là công sức viết code sao cho ĐÚNG LUẬT MỘT CHỦ ngay từ đầu.
Có lúc phải sắp xếp lại cách một chương trình được viết ra, chỉ để trình
dịch chấp nhận. Đổi lại, một khi nó đã biên dịch xong, không có giây nào
lúc chạy bị tốn cho việc đếm thẻ hay quét vòng — mọi quyết định "ai dọn,
dọn ở đâu" đã nằm sẵn trong chính đoạn mã máy được sinh ra.
::::

::::explain{#ba-cai-gia-canh-nhau}
Xếp cả ba cạnh nhau cho dễ nhớ:

| Cách | Ai è cổ chịu | Trả giá lúc nào |
|---|---|---|
| Tự tay dọn (C) | Người viết code, từng dòng một | Ngay khi quên hoặc dọn nhầm — chương trình đang chạy mới lộ |
| Đếm thẻ + gom rác (Python) | Máy, âm thầm suốt lúc chạy | Một phần nhỏ ở MỌI thao tác gán, cộng một đợt dừng khi cần quét vòng |
| Trình dịch quyết (Rust) | Trình dịch, trước khi chạy | Công sức viết code đúng luật một chủ — trả một lần, lúc biên dịch |

Không hàng nào trong bảng này có ô "miễn phí". Ba cách chỉ khác nhau ở chỗ
cái giá rơi vào tay ai, và rơi vào lúc nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi sang bài cuối cùng của chặng này.

Suốt bốn bài vừa qua, mọi phép đo đều nhắm vào một GIÁ TRỊ TRỪU TƯỢNG — một
danh sách rỗng, một cái nồi không tên cụ thể. Nhưng một giá trị thật, sống
trong một chương trình thật, có cả một cuộc đời: nó sinh ra ở một dòng cụ
thể, có một vài cái tên trỏ vào nó tại từng thời điểm, chiếm một số byte cụ
thể, và một lúc nào đó không còn ai với tới nữa.

Nếu phải VIẾT LẠI cả cuộc đời đó thành một đoạn tường thuật — không chỉ nói
"nó được dọn bằng cách nào", mà kể đủ từ lúc sinh ra tới lúc không ai với
tới — bạn sẽ cần những con số nào?

Bài sau là lúc ghép hết lại, kể trọn một câu chuyện như vậy.
::::

::::checkpoint{mastery=0.8}
::::
