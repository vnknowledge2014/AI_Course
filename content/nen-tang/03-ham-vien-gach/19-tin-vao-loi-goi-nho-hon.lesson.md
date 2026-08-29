---
id: nen-tang.ham-vien-gach.tin-vao-loi-goi-nho-hon
title: Tin vào lời gọi nhỏ hơn
summary: Viết đệ quy bằng cách coi lời gọi với đầu vào nhỏ hơn là một hộp đen ĐÃ đúng — việc của bạn gọn lại còn đúng một bước ghép thêm.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [func.recursive-trust]
requires: [func.recursion, core.function-def, core.function-call, core.function-parameter, core.function-return, ctrl.if, core.arithmetic, core.output]
concepts: [func.ham, func.de-quy]
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
Lời gọi con thì mình không mở ra xem. Mình tin nó, y như tin `len()`.
::::

::::explain{#khong-nham-toi-tang-thu-tu}
Hàm đệ quy của bài trước chạy đúng. Nhưng lúc viết nó, bạn làm một việc rất
mệt: nhẩm theo máy. `dem_nguoc(4)` gọi `dem_nguoc(3)`, cái đó lại gọi
`dem_nguoc(2)`… tới tầng thứ tư thì trong đầu có bốn lượt gọi dở dang cùng
một lúc, và bạn không còn chắc mình đang đứng ở tầng nào.

Cái mệt ấy không phải do đệ quy khó. Nó do bạn đang tự bắt mình mở hộp ra xem.

Nhớ lại bài 1. Bạn gõ `len("phở")` mà chưa một lần đọc bên trong `len`. Bạn
không nhẩm xem nó đếm ký tự ra sao. Bạn biết đúng ba điều — tên nó là gì, phải
đưa vào cái gì, nhận ra cái gì — rồi lấy con số nhận được đem đi làm việc tiếp.

Bài này áp đúng cách nhìn ấy lên chính hàm bạn đang viết:

> **Lời gọi `dem_nguoc(n - 1)` nằm trong thân `dem_nguoc` cũng là một hàm do
> người khác viết xong xuôi rồi.** Người khác đó tình cờ là bạn, nhưng trong
> lúc viết thân hàm thì cứ coi như không phải.

Được phép giả sử như vậy thì việc của bạn co lại rất nhiều. Bạn không còn phải
lo cả bốn tầng nữa. Bạn chỉ lo đúng **một** tầng: từ thứ mà lời gọi nhỏ hơn
đưa lại, ghép thêm một bước để thành kết quả cho `n`.

Ba câu hỏi, hỏi theo đúng thứ tự này, là đủ để viết xong một hàm đệ quy:

1. Trường hợp nhỏ nhất — cái nhánh không gọi lại chính mình — trả ra gì?
   (Bài trước lo câu này.)
2. Giả sử lời gọi với đầu vào nhỏ hơn ĐÃ đúng: nó đưa cho mình cái gì?
3. Từ cái đó, thêm **đúng một bước** thì ra kết quả của `n`. Bước đó là gì?

Không câu nào trong ba câu bắt bạn nhẩm tới tầng thứ tư.
::::

::::example{#doc-lai-bang-con-mat-tin}
Hàm đếm ngược của bài trước, chép lại nguyên văn:

```python title=readonly
def dem_nguoc(n):
    """In n, rồi n-1, xuống dần tới 1, cuối cùng in Xong."""
    if n == 0:
        print("Xong")
        return
    print(n)
    dem_nguoc(n - 1)

dem_nguoc(4)
```

Máy in ra:

```text
4
3
2
1
Xong
```

Bây giờ đọc lại thân hàm bằng con mắt tin, không nhẩm một tầng nào:

- Dòng `dem_nguoc(n - 1)` là một hộp đen. Nó hứa: *in từ `n - 1` xuống 1 rồi
  in Xong*. Bạn nhận lời hứa đó y như nhận lời hứa của `len`.
- Vậy để `dem_nguoc(n)` in từ `n` xuống 1 rồi in Xong, thân hàm còn thiếu đúng
  một việc: in `n` ra **trước** khi giao phần còn lại cho hộp đen.
- Đó chính là dòng `print(n)` nằm ngay trên. Thân hàm hết việc.

Ba câu hỏi lúc nãy, trả lời cho hàm này: trường hợp nhỏ nhất (`n` bằng 0) in
Xong; lời gọi nhỏ hơn đưa lại phần đếm từ `n - 1`; bước ghép thêm là in `n`.

Niềm tin này chỉ thành thật khi hai điều sau còn đúng, và cả hai đều là chuyện
bạn đã lo xong ở hai bài trước:

- **Lời gọi con phải nhận đầu vào nhỏ hơn**, nhỏ theo hướng đi về trường hợp
  cơ sở. Viết `dem_nguoc(n)` gọi thẳng `dem_nguoc(n)` thì chẳng có gì nhỏ đi,
  niềm tin thành vay mượn vòng quanh, và chồng lời gọi cao mãi tới
  `RecursionError`.
- **Trường hợp cơ sở phải trả về đúng.** Nó là tầng duy nhất không được tin ai
  cả, nên nó là tầng duy nhất bạn phải tự tay kiểm.
::::

::::predict{#quen-buoc-ghep commitOnce}
Byte viết một hàm cộng các số từ 1 tới `n`. Byte nhớ trường hợp cơ sở, nhớ cho
đầu vào nhỏ đi mỗi tầng — nhưng dòng cuối thì Byte viết hơi vội.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra con số nào?

```python
def tong_den(n):
    """Cộng các số từ 1 tới n."""
    if n == 1:
        return 1
    return tong_den(n - 1)

print(tong_den(4))
```

:::opt{correct}
1
:::

:::opt
10
::why
Gần đúng ở chỗ bạn tin lời gọi nhỏ hơn — đúng tinh thần của bài này — và bạn
cộng nhẩm đúng: 4 + 3 + 2 + 1 = 10 là con số mà hàm *lẽ ra* phải đưa ra.

Chỗ lệch nằm ở chỗ niềm tin dừng lại. Tin lời gọi nhỏ hơn nghĩa là tin nó đưa
đúng thứ nó đưa, chứ không phải tin nó đoán ra ý bạn. Dòng cuối viết
`return tong_den(n - 1)`: máy nhận thứ tầng dưới đưa lên rồi chuyền thẳng ra
ngoài, không cộng gì thêm. Cả bốn tầng cùng chuyền tay như vậy, nên con số của
tầng đáy — số 1 — đi thẳng ra tới ngoài cùng.

Câu chữ trong docstring nói hàm cộng, nhưng máy không đọc docstring (bài 5).
Bước ghép phải nằm trong mã.
::
:::

:::opt
3
::why
Gần đúng ở chỗ bạn nhìn thẳng vào đối số và thấy `n - 1`. Ở tầng ngoài cùng
`n` đang là 4, nên `n - 1` đúng là 3 thật — phần tính toán đó bạn không sai
một chữ nào.

Chỗ lệch là ý nghĩa của `return`. Nó không trả về **đối số** đưa vào lời gọi,
nó trả về **thứ lời gọi đưa ra**. `tong_den(3)` chạy trọn một lượt của chính
thân hàm ấy rồi mới đưa ra một con số, và con số đó lại đến từ `tong_den(2)`,
cứ thế xuống tới đáy.
::
:::

:::opt
Máy báo `RecursionError` vì hàm gọi lại chính nó
::why
Gần đúng ở chỗ bạn cảnh giác đúng chỗ đáng cảnh giác: một hàm gọi lại chính nó
mà không có đường xuống thì chồng lời gọi cao mãi, và bài 17 đã cho bạn thấy
tận mắt.

Chỗ lệch: ở đây đường xuống vẫn còn nguyên. Nhánh `if n == 1` chưa mất, và mỗi
tầng vẫn gọi với `n - 1` nên đầu vào vẫn nhỏ dần về phía nhánh ấy. Chuỗi lời
gọi vẫn chạm đáy và vẫn gỡ ra được.

Thứ thiếu trong đoạn này không phải chỗ dừng — nó là bước ghép thêm ở dòng
cuối. Hai chuyện khác nhau, và hàm này hỏng đúng chuyện thứ hai.
::
:::
::::

::::explain{#mot-buoc-la-bao-nhieu}
Bước ghép thêm luôn là bước tính từ **một** tầng, không bao giờ là hai.

Ở `dem_nguoc`, bước ấy là một dòng `print`. Ở `tong_den`, bước ấy là cộng thêm
chính `n` vào con số mà tầng dưới đưa lên. Ở một hàm khác nữa, bước ấy có thể
là nhân, là nối chuỗi, là kiểm một điều kiện. Nhưng luôn luôn: **nhận thứ tầng
dưới đưa lên, làm thêm một việc, đưa ra ngoài.**

Cách kiểm nhanh xem mình đã viết đúng chưa, không cần chạy thử: đọc dòng cuối
và tự hỏi *"nếu lời gọi trong dòng này đưa lại đúng thứ nó hứa, thì cả dòng
này có đưa ra đúng thứ hàm mình hứa không?"* Nếu có, hàm xong. Nếu không, chỗ
thiếu chính là bước ghép.

> Chỗ dễ vấp: ghép thêm **hai** bước cũng không báo lỗi gì cả. Máy chạy trơn
> tru và cho ra một con số sai — loại lỗi tệ nhất, loại phải tự đọc lại mới
> thấy. Nên khi viết dòng cuối, đếm xem mình vừa làm mấy việc.
::::

::::code{#ghep-them-mot-to}
Byte đi ăn phở với bạn bè, mỗi tô 45 nghìn, và muốn một hàm nói ra tổng tiền
của mấy tô.

Con số này còn tính được bằng một phép nhân, ngắn hơn hẳn. Nhưng bài này
không đi tìm cách ngắn nhất — nó tập cái nhìn tin, nên hàm dưới đây viết bằng
đệ quy và chỗ trống nằm đúng ở bước ghép.

Trường hợp cơ sở đã có sẵn: không tô nào thì hết 0 đồng. Hãy điền dòng cuối
theo đúng ba câu hỏi ở đầu bài.

```python title=starter
def tong_tien(so_to):
    """Tổng tiền của so_to tô phở, mỗi tô 45 nghìn."""
    if so_to == 0:
        return 0
    return ___

print(tong_tien(3))
```

```python title=solution
def tong_tien(so_to):
    """Tổng tiền của so_to tô phở, mỗi tô 45 nghìn."""
    if so_to == 0:
        return 0
    return 45000 + tong_tien(so_to - 1)

print(tong_tien(3))
```

```python title=test
# Ba con số, ba tầng khác nhau: tầng đáy, tầng ngay trên đáy, và một tầng đủ
# xa đáy để một bước ghép sai lộ ra. Chỉ kiểm tong_tien(3) thôi thì một lời
# giải cộng nhầm ở tầng đáy vẫn có thể lọt.
assert tong_tien(0) == 0, "không ăn tô nào thì hết 0 đồng — đó là trường hợp cơ sở, nhánh duy nhất không gọi lại chính mình"
assert tong_tien(1) == 45000, "một tô phở hết 45 nghìn: lời gọi nhỏ hơn đưa lại 0 đồng, bước ghép cộng thêm đúng một tô"
assert tong_tien(3) == 135000, "ba tô phở 45 nghìn hết 135 nghìn — mỗi tầng chỉ được cộng thêm đúng MỘT tô vào con số tầng dưới đưa lên"
```

:::hints
- kind: attention
  body: Chỗ trống là dòng cuối thân hàm, nằm dưới nhánh dừng. Xuống được tới đó nghĩa là `so_to` không phải 0, nên còn ít nhất một tô chưa ai trả tiền.
- kind: strategy
  body: "Coi `tong_tien(so_to - 1)` là hàm của người khác đã chạy đúng: nó đưa lại tiền của tất cả các tô, trừ một tô. Bạn còn thiếu đúng tiền của một tô ấy, và giá một tô thì đề bài cho sẵn."
- kind: one-line
  body: 'Viết `45000 + tong_tien(so_to - 1)` vào chỗ trống, giữ nguyên chữ `return` ở đầu dòng.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: 135000
- tier: static
  onFail: dòng bạn điền phải NHẬN lại thứ mà lời gọi nhỏ hơn đưa ra rồi mới ghép thêm một tô
  requireAst:
  # `min: 2` vì khung đã có sẵn MỘT lời gọi `tong_tien(3)` ở dòng cuối file.
  # Lời gọi thứ hai chính là lời gọi nhỏ hơn nằm trong thân hàm — thiếu nó thì
  # đoạn mã vẫn ra đúng 135000 bằng phép nhân, mà bài này thì không tập nhân.
  - kind: uses-call, target: tong_tien, min: 2
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một bước thôi. Phần còn lại mình giao cho lời gọi nhỏ hơn, và nó lo xong.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Trong `dem_nguoc(4)`, tầng ngoài đang giữ `n = 4` thì tầng trong đã `n = 3`.
Cùng một cái tên `n`, sao chúng không giẫm lên nhau?

Ở Realm 0 bạn đã học: dán một cái tên lên giá trị mới thì giá trị cũ không còn
ai gọi được nữa. Nếu `n` trong hàm cũng là một cái tên như thế, thì bốn tầng
của `dem_nguoc(4)` đang thi nhau dán đè lên cùng một chỗ.

Chuyện này không phải chỉ để tò mò. Hàm cộng ở phần trên ghép thêm một bước
bằng cách cộng chính `n` vào con số mà tầng dưới đưa lên — nghĩa là tầng ngoài
cùng phải còn giữ nguyên số 4 của riêng nó, đúng vào lúc tầng trong đang làm
việc với số 3.

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
