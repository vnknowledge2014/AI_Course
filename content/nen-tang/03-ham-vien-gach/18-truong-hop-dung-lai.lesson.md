---
id: nen-tang.ham-vien-gach.truong-hop-dung-lai
title: Trường hợp dừng lại
summary: Một nhánh không gọi lại chính mình là chỗ chồng lời gọi bắt đầu gỡ ra — và mọi lượt gọi đều phải đi về phía nhánh ấy.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [func.base-case]
requires: [func.recursion, func.call-stack, core.function-def, core.function-call, core.function-parameter, core.function-return, core.string-concat, core.fstring, ctrl.if, ctrl.else, ctrl.comparison]
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
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Phải có một lượt nào đó tự trả lời được. Không thì nhờ tới bao giờ.
::::

::::explain{#phai-co-mot-luot-khong-nho-ai}
Bài trước để lại một đoạn code không sai chỗ nào mà vẫn không dùng được. Nó
chạy, nó in đúng, nó chỉ không chịu dừng. Thiếu đúng một thứ — và giờ ta gọi
tên thứ ấy ra.

Byte ngồi cuối một dãy bàn dài và cần biết trong dãy có bao nhiêu người. Byte
quay sang người bên cạnh: *"Đằng trước cậu có mấy người? Nói tớ biết rồi tớ
cộng thêm mình vào."* Người ấy quay sang người kế tiếp hỏi đúng câu đó. Cứ thế
câu hỏi chạy dần lên đầu dãy.

Dãy bàn có kết thúc, nhưng chuyện chỉ xong khi tới **người ngồi đầu dãy** —
người không còn ai để quay sang hỏi nữa. Người ấy không hỏi ai cả; người ấy
đáp thẳng: *"Trước tớ không có ai. Không người nào."* Chỉ từ giây đó câu trả
lời mới bắt đầu chạy ngược về, mỗi người cộng thêm một, cho tới Byte.

Nếu ai cũng quay sang hỏi người bên cạnh, kể cả người đầu dãy, thì không câu
trả lời nào bao giờ khởi hành. Đó chính xác là chuyện đã xảy ra ở bài trước.

Cái nhánh mà người đầu dãy đi vào — nhánh **trả lời thẳng, không nhờ ai** — có
tên riêng: **trường hợp cơ sở**.

Nói bằng ngôn ngữ của cái chồng phiếu ở bài 16: trường hợp cơ sở là lượt gọi
duy nhất **không đặt thêm phiếu nào lên chồng**. Nó chạy xong, nó `return`, và
tờ phiếu của nó được gỡ đi. Tờ nằm dưới lập tức được chạy tiếp, rồi cũng gỡ.
Cả cái chồng đổ xuống theo dây chuyền, từ trên xuống dưới, nhanh y như lúc nó
cao lên.
::::

::::example{#dem-nguoc-biet-dung}
Cũng hàm đếm ngược của bài trước, thêm đúng một nhánh.

```python title=readonly
def dem_nguoc(n):
    if n == 0:
        print("Xong!")
    else:
        print(f"Byte đếm: {n}")
        dem_nguoc(n - 1)

dem_nguoc(3)
```

Máy in ra:

```text
Byte đếm: 3
Byte đếm: 2
Byte đếm: 1
Xong!
```

Bốn dòng, rồi chương trình kết thúc bình thường. Không traceback, không lỗi.

Nhánh `if n == 0:` chính là trường hợp cơ sở. Nhìn kỹ nó khác nhánh kia ở đúng
một điểm: trong thân nó **không có tên `dem_nguoc`**. Đó là toàn bộ bí mật.

Đi lại đường máy đi, và đếm phiếu như bài 16:

- `dem_nguoc(3)` — `n` là 3, không phải 0, nên đi nhánh `else`: in câu, rồi gọi
  `dem_nguoc(2)`. Phiếu thứ nhất đặt xuống, đang đợi.
- `dem_nguoc(2)` — cũng nhánh `else`. Phiếu thứ hai. Chồng cao 2.
- `dem_nguoc(1)` — cũng nhánh `else`. Phiếu thứ ba. Chồng cao 3.
- `dem_nguoc(0)` — lần này `n` đúng bằng 0, nên đi nhánh `if`: in "Xong!" và
  hết việc. Phiếu thứ tư **không gọi ai**. Chồng cao 4, và đây là đỉnh.
- Phiếu thứ tư chạy hết thân thì được gỡ. Tầng dưới nó cũng vừa hết việc —
  dòng gọi là dòng cuối trong thân — nên cũng gỡ. Rồi tầng dưới nữa, rồi tầng
  dưới nữa.

Chồng cao lên bốn tầng rồi xẹp xuống bốn tầng. Bài trước nó cũng cao lên y như
vậy, chỉ khác là nó không có tầng nào chịu dừng, nên nó cao mãi tới trần.
::::

::::explain{#hai-dieu-phai-du-ca-hai}
Có một nhánh không gọi lại chính mình là điều kiện thứ nhất. Nó cần, nhưng một
mình nó chưa đủ.

Điều kiện thứ hai: **mọi lượt gọi đều phải đi về phía nhánh ấy.**

Nghĩa là mỗi lần gọi lại chính mình, thứ đưa vào phải nhích lại gần trường hợp
cơ sở hơn lần trước. Trong `dem_nguoc`, cơ sở là `n == 0`, và mỗi lượt đưa vào
`n - 1` — con số bé dần, nên sớm muộn cũng chạm đúng 0.

Đổi đúng một dấu, từ `n - 1` thành `n + 1`, thì nhánh cơ sở vẫn nằm nguyên đó,
vẫn viết đúng, và vẫn chẳng bao giờ có ai đi vào. Con số chạy 3, 4, 5, 6 và
càng lúc càng xa 0. Chương trình quay về đúng cảnh bài trước.

Nói cho gọn, một hàm đệ quy chạy được cần đủ hai vế:

- **một nhánh dừng** — trong thân nhánh ấy không có tên của chính hàm;
- **một đường đi tới nhánh đó** — mỗi lượt gọi làm đầu vào tiến lại gần nó.

Thiếu vế đầu thì không tờ phiếu nào được gỡ. Thiếu vế sau thì có tờ phiếu chờ
sẵn để gỡ, nhưng không lượt nào tới được chỗ đó. Hai kiểu thiếu khác nhau, một
kết cục giống nhau, và cùng một câu báo lỗi.

> Chỗ dễ vấp: viết cơ sở là `n == 0` rồi gọi với `n - 2`. Xuất phát từ số chẵn
> thì chạy ngon; xuất phát từ số lẻ thì đi qua 0 mà không hề chạm vào nó —
> 5, 3, 1, -1, -3 — và chương trình dừng bằng lỗi. Cùng một hàm, có đầu vào
> chạy được có đầu vào không, nên đây là loại sai lúc thử thì không thấy.
::::

::::predict{#doan-khi-di-sai-huong commitOnce}
Byte giữ nguyên nhánh cơ sở, chỉ đổi dấu trừ thành dấu cộng.

**Trước khi bấm chạy**, bạn đoán chuyện gì xảy ra?

```python
def dem_nguoc(n):
    if n == 0:
        print("Xong!")
    else:
        print(f"Byte đếm: {n}")
        dem_nguoc(n + 1)

dem_nguoc(3)
```

:::opt{correct}
In ra 3, 4, 5 và tăng mãi, rồi chương trình dừng bằng `RecursionError`
:::

:::opt
Vẫn in 3, 2, 1 rồi Xong — vì nhánh `if n == 0` còn nguyên nên hàm vẫn biết dừng
::why
Gần đúng ở chỗ bạn nhìn vào đúng thứ đáng nhìn: nhánh cơ sở là thứ quyết định
hàm có dừng được hay không, và nhánh ấy ở đây quả thật còn nguyên vẹn.

Chỗ lệch là ở chỗ nhánh cơ sở không tự kéo ai về phía nó. Nó chỉ là một cái
cửa đứng yên; ai đi tới thì mới qua được. Con số đưa vào mỗi lượt là `n + 1`
nên nó đi 3, 4, 5 — mỗi bước một xa cái cửa ấy thêm. Cửa mở sẵn suốt mà không
lượt nào bước tới.
::
:::

:::opt
In ra một dãy số tăng dần, rồi tới lúc nào đó gặp 0 và in Xong
::why
Gần đúng ở chỗ bạn theo dõi đúng dãy số — nó tăng thật, 3 rồi 4 rồi 5. Phần
quan sát của bạn chính xác.

Chỗ lệch nằm ở kỳ vọng rằng một dãy tăng rồi cũng "vòng lại" 0. Số trong máy
không đi vòng tròn: nó cứ lớn thêm mãi. Muốn chạm 0 thì phải đi xuống, mà mỗi
lượt ở đây lại đi lên. Điều kiện `n == 0` hỏi một câu chỉ đúng tại một điểm
duy nhất, và dãy này bỏ điểm đó lại phía sau ngay từ bước đầu.
::
:::

:::opt
Máy báo lỗi ngay lúc đọc dòng `dem_nguoc(n + 1)`, vì đệ quy bắt buộc phải dùng phép trừ
::why
Gần đúng ở chỗ bạn cảm nhận được là dấu cộng ở đây có gì đó không ổn — cảm
nhận ấy đúng, và nó chính là nội dung bài này.

Chỗ lệch: không có luật nào của Python nói về dấu trừ trong hàm đệ quy. Máy
không biết `n + 1` đi xa hay đi gần cái gì; nó cộng, và chạy tiếp. Với một hàm
mà trường hợp cơ sở là `n == 10` thì `n + 1` lại là hướng đúng. Thứ phải đúng
không phải cái dấu, mà là **đi về phía nhánh cơ sở**.
::
:::
::::

::::code{#go-tung-to-phieu}
Byte có một chồng tô bẩn cao ba tô và muốn viết một hàm đệ quy gỡ chúng xuống,
vừa gỡ vừa kể lại.

Nhánh gọi lại chính mình đã viết sẵn cho bạn: nó kể tô đang gỡ, rồi nhờ chính
nó lo phần chồng còn lại — thấp hơn đúng một tô.

Còn thiếu nhánh dừng. Hãy điền điều kiện vào chỗ trống, sao cho lượt gọi nào
rơi vào nhánh ấy thì trả về thẳng chữ `hết chồng` mà không nhờ ai nữa.

Với chồng ba tô, câu Byte muốn nghe là:

`gỡ tô 3, gỡ tô 2, gỡ tô 1, hết chồng`

```python title=starter
def go_chong(so_to):
    if ___:
        return "hết chồng"
    return f"gỡ tô {so_to}, " + go_chong(so_to - 1)

cau = go_chong(3)
print(cau)
```

```python title=solution
def go_chong(so_to):
    if so_to == 0:
        return "hết chồng"
    return f"gỡ tô {so_to}, " + go_chong(so_to - 1)

cau = go_chong(3)
print(cau)
```

```python title=test
# Ba phép kiểm cho ba tình huống khác nhau, và phải đủ cả ba.
#
# Chồng ba tô kiểm phần "gỡ đủ, gỡ đúng thứ tự". Chồng rỗng kiểm riêng nhánh
# dừng — nếu điều kiện luôn đúng thì phép kiểm đầu đã trượt, còn nếu điều kiện
# không bao giờ đúng thì hàm chạy tới `RecursionError` chứ không tới đây.
# Chồng một tô là ca hẹp nhất còn có việc để làm.
assert go_chong(3) == "gỡ tô 3, gỡ tô 2, gỡ tô 1, hết chồng", "chồng ba tô phải gỡ đủ ba tô theo thứ tự 3, 2, 1 rồi mới tới câu hết chồng"
assert go_chong(0) == "hết chồng", "chồng rỗng thì không có tô nào để kể — hàm phải rơi thẳng vào nhánh dừng và trả về đúng hai chữ hết chồng"
assert go_chong(1) == "gỡ tô 1, hết chồng", "chồng một tô phải kể đúng một tô rồi dừng — không thừa tô nào, không thiếu tô nào"
assert cau == "gỡ tô 3, gỡ tô 2, gỡ tô 1, hết chồng", "cái tên cau phải đang giữ nguyên câu mà go_chong(3) trả về"
```

:::hints
- kind: attention
  body: Nhìn dòng cuối trong thân hàm. Mỗi lượt gọi lại truyền vào `so_to - 1`, nên con số đi 3, rồi 2, rồi 1, rồi tới một chỗ mà không còn tô nào để kể nữa. Điều kiện bạn cần viết là câu hỏi nhận ra đúng chỗ ấy.
- kind: strategy
  body: Trường hợp cơ sở là lượt gọi không nhờ ai. Ở đây nó là lượt mà chồng đã rỗng, tức là `so_to` đã đi hết đường xuống. Viết một phép so sánh giữa cái tên đang giữ số tô và con số ứng với chồng rỗng. Đừng gõ số 3 vào đó — số 3 là chồng lúc bắt đầu, không phải chỗ dừng, và hàm còn phải chạy đúng với những chồng cao thấp khác nữa.
- kind: one-line
  body: Viết `so_to == 0` vào chỗ trống, giữ nguyên dấu hai chấm ở cuối dòng.
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  expect: gỡ tô 3, gỡ tô 2, gỡ tô 1, hết chồng
- tier: static
  onFail: điều kiện dừng phải hỏi về chính cái tên đang giữ số tô, chứ không phải một câu đúng sẵn
  requireAst:
  # Khung đã ĐỌC `so_to` hai lần (trong câu chữ và trong lời gọi con). Điều
  # kiện bạn viết phải là lần đọc thứ ba — một điều kiện không nhắc tới nó thì
  # không thể nào nhận ra lúc nào chồng đã rỗng.
  - kind: uses-name, target: so_to, min: 3
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chồng xẹp xuống rồi! Có một tầng chịu tự trả lời là cả chồng đổ theo.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Chạy đúng rồi. Nhưng lúc viết thân hàm bạn vẫn phải nhẩm xem lời gọi con chạy
ra sao — `go_chong(3)` gọi `go_chong(2)`, mà `go_chong(2)` lại gọi
`go_chong(1)`, mà `go_chong(1)` thì gọi `go_chong(0)` — nhẩm tới tầng thứ tư
thì loạn.

Bài 1 nói bạn dùng `len()` mà không cần mở ra xem. Làm y như vậy với chính hàm
mình đang viết được không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.85}
::::
