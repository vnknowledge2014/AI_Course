---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.hai-noi-tro-vao-nhau
title: Hai cái nồi trỏ vào nhau thì ai dọn
summary: "Đếm thẻ thua ngay chỗ hai giá trị giữ tấm thẻ trỏ VỀ NHAU — và đó là lý do Python cần thêm một bộ dọn khác."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [mem.reference-cycle]
requires: [mem.refcount]
concepts: [mem.vong-tro, mem.dem-the]
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
Bài trước để lại một cái bẫy: hai cái nồi trỏ thẳng vào nhau. Giờ mình nhảy
thẳng vào bẫy đó xem sao.
::::

::::explain{#vong-tro-la-gi}
Câu hỏi cuối bài trước: nồi A giữ một tấm thẻ chỉ sang nồi B, nồi B giữ một
tấm thẻ chỉ ngược lại sang nồi A. Gỡ hết tên bên ngoài khỏi cả hai. Đếm thẻ
của A và B có bao giờ về không được không?

Không. Và lý do nằm ngay trong cách đếm thẻ hoạt động: mỗi lần gỡ MỘT thẻ
thì con số giảm MỘT — nhưng ở đây, thẻ mà nồi B giữ để trỏ sang nồi A không
hề đi đâu cả khi bạn gỡ cái tên bên ngoài. Cái tên và cái thẻ-bên-trong-B là
hai thứ khác nhau. Bạn chỉ gỡ được cái đầu tiên.

Kết quả: đếm thẻ của A dừng lại ở một — vẫn còn đúng một thẻ trỏ vào nó, nằm
bên trong B. Đếm thẻ của B cũng dừng ở một, vì lý do y hệt, chỉ đổi vai. Cả
hai mắc kẹt ở mức một, mãi mãi không xuống được số không, dù không còn cái
tên nào bên ngoài với tới được chúng nữa.

Cấu trúc như vậy — một chuỗi tấm thẻ khép kín, quay trở lại đúng chỗ nó xuất
phát — gọi là một **vòng tham chiếu**. Và đây chính là chỗ cách đếm thẻ của
bài trước THUA.
::::

::::example{#hai-noi-quay-vao-nhau}
Dựng một vòng tham chiếu không đòi công cụ mới nào — vẫn `.append`, thứ đã
dùng từ lâu.

```python title=readonly
import gc, sys

gc.collect()          # dọn sạch trước, cho một nền sạch để đo

noi_a = []
noi_b = []
noi_a.append(noi_b)    # noi_a giữ một thẻ trỏ sang noi_b
noi_b.append(noi_a)    # noi_b giữ một thẻ trỏ NGƯỢC LẠI sang noi_a

print(f"đếm thẻ của noi_a: {sys.getrefcount(noi_a)}")
print(f"đếm thẻ của noi_b: {sys.getrefcount(noi_b)}")

noi_a = None
noi_b = None

so_con_ket = gc.collect()
print(f"số vật gc phải tự tay dọn: {so_con_ket}")
```

```text title=readonly
đếm thẻ của noi_a: 3
đếm thẻ của noi_b: 3
số vật gc phải tự tay dọn: 2
```

Nhớ lời dặn bài trước: con số 3 tự nó không nói lên gì — nó gồm cả phần dôi
do lượt gọi `sys.getrefcount` gây ra. Điều đáng nhìn là: **cả hai đều ở mức
CAO HƠN mức chỉ-có-một-thẻ-bên-ngoài**, đúng vì mỗi nồi còn giữ thêm một thẻ
từ phía bên kia.

Rồi tới dòng đáng chú ý nhất: sau khi gán cả `noi_a` lẫn `noi_b` về `None` —
không còn cái tên nào bên ngoài trỏ tới hai cái nồi đó nữa — `gc.collect()`
vẫn tìm thấy đúng **hai** vật phải tự tay dọn. Nếu cách đếm thẻ ở bài trước
đã lo được việc này, con số đó phải là 0. Nó không phải 0. Đếm thẻ đã bó
tay, và một bộ phận khác của Python vừa phải ra tay dọn thay.

Bộ phận đó gọi là **bộ dọn theo vòng** (tiếng Anh: *cyclic garbage
collector*, nằm trong mô-đun `gc`). Nó không chạy liên tục như đếm thẻ. Nó
chạy theo đợt: thỉnh thoảng quét qua bộ nhớ, tìm những cụm giá trị vẫn giữ
thẻ trỏ vào nhau nhưng không còn cái tên nào bên ngoài với tới, rồi dọn cả
cụm một lượt. Gọi `gc.collect()` bằng tay như trên chỉ để BẠN nhìn thấy nó
làm việc ngay lập tức — chương trình bình thường không cần gọi dòng đó, bộ
dọn tự chạy theo đợt của riêng nó.
::::

::::predict{#vong-co-duoc-don-ngay-khong commitOnce}
Byte dựng một vòng khác, y hệt cấu trúc trên nhưng đổi tên:

```python title=readonly
hop_a = []
hop_b = []
hop_a.append(hop_b)
hop_b.append(hop_a)

hop_a = None
hop_b = None

# đúng NGAY sau hai dòng trên, chưa gọi gc.collect() lần nào
```

**Trước khi xem đáp án**, ngay sau hai dòng gán `None` đó — chưa ai gọi
`gc.collect()` — hai cái danh sách cũ đã bị dọn khỏi bộ nhớ chưa?

:::opt{correct}
Chưa. Chúng vẫn còn nằm trong bộ nhớ, chờ tới lượt bộ dọn theo vòng ghé qua.
:::

:::opt
Rồi — đếm thẻ vẫn chạy liên tục, không nghỉ, nên dọn được ngay như bài trước.
::why
Gần đúng ở việc bạn nhớ đúng: đếm thẻ đúng là chạy liên tục, không hề nghỉ,
đúng như bài trước đã học.

Chỗ lệch: đếm thẻ chạy liên tục KHÔNG có nghĩa nó luôn thành công. Bài này
vừa chỉ ra chính xác chỗ nó thua — hai cái hộp giữ thẻ trỏ vào nhau, nên gỡ
tên bên ngoài không bao giờ đưa đếm thẻ của chúng về không. Nó vẫn chạy,
chỉ là chạy mà không đủ để dọn.
::
:::

:::opt
Chưa, và sẽ không bao giờ được dọn — chương trình vừa rò rỉ bộ nhớ vĩnh viễn.
::why
Gần đúng ở nửa đầu: đúng là chưa dọn ngay, đếm thẻ thật sự bó tay ở đây.

Chỗ lệch nằm ở "vĩnh viễn". Nếu Python chỉ có mỗi cách đếm thẻ thì đúng là
rò rỉ mãi mãi thật — và đó chính xác là điều một số ngôn ngữ khác gặp phải.
Nhưng Python có thêm một bộ phận thứ hai lo đúng trường hợp này: bộ dọn
theo vòng. Nó không dọn NGAY, nhưng tới lượt của nó thì hai cái hộp này sẽ
bị dọn.
::
:::

:::opt
Máy dừng lại và báo lỗi, vì phát hiện một vòng không hợp lệ.
::why
Gần đúng ở việc bạn nghĩ máy "phát hiện" ra điều gì đó bất thường — cảm
giác rất tự nhiên, vì vòng lặp vô hạn từng làm máy treo.

Chỗ lệch: dựng một vòng tham chiếu là chuyện hoàn toàn hợp lệ trong Python,
không có lỗi nào được báo ra cả. `hop_a.append(hop_b)` chỉ là một lệnh
`.append` bình thường; máy không hề biết trước rằng dòng kế tiếp sẽ khép
vòng lại. Nó cứ chạy êm, và cái giá phải trả chỉ lộ ra sau đó, ở chỗ dọn.
::
:::
::::

::::code{#dung-vong-hai-noi}
Đến lượt bạn dựng một vòng. Hai cái nồi `nong_a` và `nong_b` đã có sẵn,
rỗng. Dùng `.append` để mỗi nồi giữ một tấm thẻ trỏ sang nồi kia.

```python title=starter
import gc

gc.collect()

nong_a = []
nong_b = []
___                 # nồi A giữ một tấm thẻ trỏ sang nồi B
___                 # nồi B giữ một tấm thẻ trỏ NGƯỢC LẠI sang nồi A

# đúng NỒI KIA, không phải chính nó — ghi lại trước khi buông tên
dung_vong = nong_a[0] is nong_b and nong_b[0] is nong_a

nong_a = None
nong_b = None

so_con_ket = gc.collect()
print(f"số vật gc phải tự tay dọn: {so_con_ket}")
```

```python title=solution
import gc

gc.collect()

nong_a = []
nong_b = []
nong_a.append(nong_b)
nong_b.append(nong_a)

# đúng NỒI KIA, không phải chính nó — ghi lại trước khi buông tên
dung_vong = nong_a[0] is nong_b and nong_b[0] is nong_a

nong_a = None
nong_b = None

so_con_ket = gc.collect()
print(f"số vật gc phải tự tay dọn: {so_con_ket}")
```

```python title=test
# Một vòng ĐÚNG hai chiều thì gc phải tìm thấy ít nhất hai vật mắc kẹt.
# Nếu chỉ nối MỘT chiều (quên chiều ngược lại), đếm thẻ vẫn tự lo xong
# xuôi — con số này sẽ tụt về 0 hoặc 1, không đạt hai.
assert so_con_ket >= 2, f"dựng đúng một vòng hai chiều thì gc.collect() phải tìm thấy ít nhất 2 vật mắc kẹt, ở đây chỉ thấy {so_con_ket} — kiểm lại cả hai dòng .append đã nối ĐÚNG NGƯỢC lại chưa"
# Nối MỖI nồi vào CHÍNH NÓ (thay vì vào nồi kia) cũng tạo ra hai vòng —
# gc vẫn đếm được 2 — nhưng đó KHÔNG phải bài yêu cầu: đề bài đòi nồi A
# trỏ sang nồi B và nồi B trỏ NGƯỢC LẠI sang nồi A, không phải mỗi nồi tự
# ôm lấy mình. `dung_vong` phân biệt đúng hai trường hợp này.
assert dung_vong is True, "nồi A phải trỏ ĐÚNG sang nồi B, và nồi B phải trỏ NGƯỢC LẠI đúng sang nồi A — không phải mỗi nồi tự trỏ vào chính mình"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất đứng ngay dưới dòng `nong_b = []`, nói việc nồi A phải làm. Chỗ trống thứ hai nói việc nồi B phải làm, và nó phải trỏ NGƯỢC LẠI — không phải trỏ vào chính mình.
- kind: strategy
  body: Cách nối một danh sách vào một danh sách khác đã quen từ lâu — `.append(cai_kia)`. Nồi A gọi `.append` với nồi B làm đối số; nồi B gọi `.append` với nồi A làm đối số. Hai dòng gần như soi gương nhau, chỉ đổi vai.
- kind: one-line
  body: 'Chỗ trống thứ nhất viết `nong_a.append(nong_b)`, chỗ trống thứ hai viết `nong_b.append(nong_a)`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: static
  onFail: mỗi chỗ trống phải gọi `.append` để nối một nồi vào nồi kia — đây là cách đã quen dùng để một danh sách chứa một danh sách khác
  requireAst:
  - kind: uses-call, target: append, min: 2
- tier: output
  match: contains
  expect: "số vật gc phải tự tay dọn:"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cái nồi ôm lấy nhau, và đếm thẻ nhìn chúng bó tay. Bộ dọn theo vòng vừa
ra tay thay.
::::

::::explain{#rust-chon-duong-khac}
Một chỗ đáng dừng lại, dù chưa học Rust — chỉ nhìn ý tưởng thôi.

Rust không có một bộ dọn nào chạy phía sau chương trình cả — không đếm thẻ
liên tục, cũng không có đợt quét theo vòng. Lý do là Rust chặn đứng vấn đề
từ trước khi nó xảy ra: theo mặc định, **mỗi giá trị chỉ có đúng MỘT chủ**,
không phải nhiều tấm thẻ cùng lúc như Python.

```rust title=readonly
// Ý TƯỞNG, không phải cú pháp thật đã dạy — chỉ để nhìn hình dạng:
// nồi A giữ nồi B, nồi B lại giữ nồi A?
// Trình dịch Rust từ chối biên dịch dòng thứ hai — nồi B đã có
// một chủ khác giữ rồi, không được có chủ thứ hai.
```

Không có tấm thẻ thứ hai thì không có cách nào hai cái nồi trỏ vào nhau
theo kiểu bài này vừa dựng. Rust không cần dọn vòng, vì luật của nó không
cho vòng hình thành ngay từ đầu. Đó là "con đường khác" mà bài sau sẽ đặt
cạnh hai cách của Python để so cho rõ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ trong tay có hai cách Python tự dọn: đếm thẻ, chạy liên tục, gần như
miễn phí — và bộ dọn theo vòng, chạy theo đợt, để vá đúng chỗ đếm thẻ thua.

Nhưng "theo đợt" nghĩa là nó phải tạm dừng chương trình, đi quét qua một
lượt bộ nhớ, tốn thời gian thật. Không có gì trong máy tính là miễn phí cả
— kể cả một chương trình có vẻ "tự lo" mọi thứ.

Vậy so với việc TỰ TAY dọn từng chút một (cách C vẫn làm), hay để trình dịch
tính hết trước khi chạy (cách Rust vừa hé), ba lối đi này ai trả giá gì?

Bài sau đặt cả ba cạnh nhau.
::::

::::checkpoint{mastery=0.8}
::::
