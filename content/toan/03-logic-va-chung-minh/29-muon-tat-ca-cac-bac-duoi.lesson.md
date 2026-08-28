---
id: toan.logic-va-chung-minh.muon-tat-ca-cac-bac-duoi
title: Mượn tất cả các bậc dưới
summary: Khi bậc liền trước chẳng giúp được gì, bước quy nạp được phép giả sử MỌI bậc từ cơ sở tới k — vẫn không vòng tròn, vì mỗi bậc chỉ mượn những bậc đã đổ.
locale: vi
track: toan
module: logic-va-chung-minh
order: 29
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [logic.strong-induction]
requires: [logic.induction-base-case, logic.induction-hypothesis, logic.induction, logic.implication, logic.direct-proof, logic.for-all, logic.counterexample, logic.finite-check-not-proof, logic.use-definition, math.multiplication, math.remainder, math.factor-common, core.boolean, core.variable, core.reassign, core.number-literal, core.arithmetic, core.modulo, core.floor-division, core.function-def, core.function-call, core.function-parameter, core.function-return, core.list, core.list-index, core.len, core.dict, core.dict-assign, core.print-variable, ctrl.while, ctrl.if, ctrl.else, ctrl.comparison]
concepts: [logic.quy-nap-manh, logic.bac-lien-truoc-vo-dung, logic.moi-bac-da-do]
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
Bậc mình được mượn thì chẳng giúp gì. Bậc giúp được thì mình không được mượn.
::::

::::explain{#bac-lien-truoc-vo-dung}
Bài trước tắc ở đúng một chỗ, và chỗ ấy đáng nhìn cho kỹ.

Câu cần chứng minh: **mọi số nguyên từ 2 trở lên đều viết được thành một tích
các số nguyên tố.** Gọi câu ở bậc `n` là `P(n)`.

Cơ sở đứng vững: 2 là số nguyên tố, nên nó đã là một tích gồm đúng một thừa số.
Việc thứ nhất của bài 28 xong.

Bước thì tắc. Lấy bậc 12 làm ví dụ. Tách 12 ra thì được `3 × 4 = 12` — hai
thừa số, mỗi cái nhỏ hơn 12. Muốn ghép thành tích các số nguyên tố, ta cần biết
**3** tách được và **4** tách được.

Còn bước quy nạp thường cho mượn cái gì? Đúng một thứ: `P(11)`. Bậc liền trước.

Mà 11 là số nguyên tố. Nó không phải thừa số nào của 12, không chia hết 12, và
nó chẳng nói được câu nào về 3 hay 4.

Đây không phải chuyện chọn cách tách vụng. Tách kiểu nào thì hai thừa số cũng
tụt xuống thấp hơn 11 — `2 × 6 = 12` cũng vậy, `6 × 2 = 12` cũng vậy. **Bậc cần
mượn nằm sâu bên dưới bậc được phép mượn.**

Vấn đề không nằm ở câu cần chứng minh. Nó nằm ở chỗ ta đang tự trói tay mình.
::::

::::explain{#noi-rong-quyen-muon}
Quay lại hàng domino của bài 26 và hỏi một câu rất thật: khi quân thứ 12 chuẩn
bị đổ, những quân nào đã nằm rạp rồi?

Không phải mỗi quân thứ 11. **Tất cả** — từ quân đầu tiên tới quân thứ 11, quân
nào cũng đã đổ xong.

Vậy sao lại chỉ cho phép tựa vào một quân? Chuyện ấy không phải luật của quy
nạp, nó chỉ là thói quen từ mấy câu đầu tiên ta gặp, những câu mà bậc liền trước
vừa đủ dùng.

Nới quyền mượn ra cho đúng với thực tế:

> **Quy nạp mạnh.** Muốn chứng minh `P(n)` đúng với mọi `n` từ cơ sở trở lên,
> làm hai việc:
>
> 1. **Cơ sở** — chứng minh bậc đầu tiên, kiểm thẳng, y như bài 28.
> 2. **Bước** — với một `k` bất kỳ, **giả sử `P` đúng ở tất cả các bậc từ cơ sở
>    tới `k`**, rồi từ đó chứng minh `P(k + 1)`.
>
> Khác quy nạp thường đúng một chữ: chỗ trước kia chỉ được mượn `P(k)`, giờ
> được mượn cả `P(2)`, `P(3)`, …, `P(k)` — bậc nào cũng được, bao nhiêu bậc
> cũng được.

Và phải trả lời ngay câu nghi ngờ mà bài 27 đã dựng sẵn khuôn để trả lời: mượn
nhiều thế thì có vòng tròn không?

Không. Lý do y hệt lần trước, không thêm một chữ nào:

- Cơ sở chẳng mượn ai. Nó được chứng minh thẳng.
- Ở bậc `k + 1`, mọi bậc được mượn đều **nhỏ hơn** `k + 1`, nghĩa là chúng đã
  đổ rồi tại thời điểm ấy.
- Nên nếu lần theo, mỗi lần mượn lại tụt xuống một bậc thấp hơn, và tụt mãi thì
  chạm cơ sở — chỗ không mượn ai nữa.

Quy nạp thường cũng chạy đúng lập luận ấy, chỉ khác là nó tự nguyện mượn ít
hơn quyền nó có. Nên quy nạp mạnh không phải một nguyên lý thứ hai phải học
thuộc; nó là cùng một nguyên lý, thôi tự trói tay.
::::

::::example{#chung-minh-tach-thanh-tich}
Chứng minh đầy đủ, bằng đúng hai việc vừa nêu.

**Cơ sở — bậc 2.** 2 là số nguyên tố. Một tích gồm đúng một thừa số vẫn là một
tích, nên `P(2)` đúng.

**Bước — bậc `k + 1`.** Cho `k + 1` là một số nguyên lớn hơn 2. Giả sử `P` đúng
ở mọi bậc từ 2 tới `k`. Xét hai trường hợp, và hai trường hợp này phủ kín, đúng
tính lưỡng trị của bài 2:

- *`k + 1` là số nguyên tố.* Thế thì nó đã là một tích gồm đúng một thừa số.
  Xong, chẳng cần mượn ai.
- *`k + 1` không phải số nguyên tố.* Theo định nghĩa của T2.1, nó có một ước
  `a` nằm giữa 2 và `k`, và khi ấy `k + 1 = a × b` với `b` cũng nằm giữa 2 và
  `k`. Cả `a` lẫn `b` đều là bậc **đã đổ**, nên giả thiết quy nạp mạnh cho mượn
  cả hai: `a` viết được thành một tích các số nguyên tố, `b` cũng vậy. Nối hai
  tích ấy lại là được một tích các số nguyên tố cho `k + 1`.

Hai trường hợp đều xong, nên `P(k + 1)` đúng. Hết chứng minh.

Đây là chỗ đáng dừng lại: **cả `a` lẫn `b` đều không phải bậc liền trước.** Quy
nạp thường không đưa nổi cho ta một trong hai. Quy nạp mạnh đưa cả hai mà không
phải xin thêm quyền nào ngoài cái quyền vốn đã có.

Theo dấu chứng minh ấy trên số 12, chọn ước nhỏ nhất làm `a`:

```text
12 = 2 × 6      (a = 2 đã là số nguyên tố; còn phải tách 6)
 6 = 2 × 3      (cả hai đã là số nguyên tố)
```

Nối lại: `2 × 2 × 3 = 12`. Và nếu chọn cách tách khác thì vẫn về cùng một chỗ —
tách theo `3 × 4 = 12` rồi tách tiếp 4 thành `2 × 2 = 4`, ta được
`3 × 2 × 2 = 12`. Cùng ba thừa số ấy, chỉ khác thứ tự viết.

Chứng minh không hề bảo phải tách kiểu nào. Nó chỉ đòi hai thừa số đều nằm giữa
2 và `k` — và cả hai cách tách đều đạt.
::::

::::predict{#doan-uoc-nho-nhat commitOnce}
Trước khi bắt máy dựng cả bảng, Byte thử riêng cái việc "tìm một ước để tách".

Hàm `uoc_nho_nhat` dò từ 2 đi lên, trả về ước đầu tiên nó gặp. Nếu dò hết mà
không gặp ước nào thì số đó không tách được — nó là số nguyên tố — và hàm trả
về chính nó.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def uoc_nho_nhat(n):
    d = 2
    while d * d <= n:
        if n % d == 0:
            return d
        d = d + 1
    return n

print(uoc_nho_nhat(12))
print(12 // uoc_nho_nhat(12))
print(uoc_nho_nhat(11))
print(uoc_nho_nhat(4))
```

:::opt{correct}
`2`, `6`, `11`, `2`
:::

:::opt
`3`, `4`, `11`, `2`
::why
Gần đúng ở hai dòng cuối, và ở chỗ bạn nhớ đúng ví dụ của bài: phần giải thích
tách 12 thành hai thừa số 3 và 4, nên bạn chờ máy đưa ra đúng cặp ấy.

Chỗ lệch là hàm này không đi tìm cặp "cân đối" nào cả. Nó dò từ 2 đi lên và trả
về **ước đầu tiên** gặp được, mà 12 chia hết cho 2 ngay ở vòng dò thứ nhất. Nên
nó dừng ở 2, và phần còn lại là 6. Chứng minh chấp nhận cả hai cách tách — nó
chỉ đòi hai thừa số đều nhỏ hơn số đang xét và không nhỏ hơn 2.
::
:::

:::opt
`1`, `12`, `1`, `1`
::why
Gần đúng ở chỗ bạn nhớ một sự thật thật: 1 là ước của mọi số, không sót số nào.

Chỗ lệch nằm ở dòng `d = 2` mở đầu hàm. Cuộc dò bắt đầu từ 2, nên số 1 không
bao giờ được xét tới. Và điều đó là cố ý: tách một số thành `1 × chính nó` thì
thừa số thứ hai không hề nhỏ đi, nên bậc phải mượn vẫn là chính bậc đang xét —
đúng cái vòng tròn mà bài 27 đã dựng cả một bài để tránh.
::
:::

:::opt
`2`, `6`, `1`, `2`
::why
Gần đúng ở ba dòng, và cách đọc của bạn cho hai dòng đầu chính xác hoàn toàn:
máy dò từ 2 lên, gặp 2 chia hết 12, nên trả 2 rồi phần còn lại là 6.

Chỗ lệch ở dòng thứ ba. Với 11, vòng dò thử `d` bằng 2 rồi 3; tới `d` bằng 4 thì
`4 × 4 = 16` đã vượt 11 nên vòng thoát mà chưa gặp ước nào. Lúc ấy hàm chạy tới
dòng cuối và trả về **chính `n`**, tức 11 — chứ không trả 1. Giá trị trả về ấy
là cách hàm nói "số này không tách được nữa", và cả bảng lát nữa dựa vào đúng
tín hiệu đó để biết khi nào dừng.
::
:::
::::

::::code{#dung-bang-tu-duoi-len}
Giờ dựng chứng minh vừa rồi thành một cái bảng, và dựng **từ dưới lên** — đó
đúng là hình dạng của quy nạp mạnh khi cho máy chạy.

Bảng `bang` ghi cho mỗi số từ 2 tới 30 một danh sách các thừa số nguyên tố của
nó. Vòng lặp đi từ 2 lên 30, và ở mỗi số nó làm đúng hai trường hợp của chứng
minh:

1. **Số nguyên tố** — nhận ra bằng dấu hiệu `a` bằng chính `n`. Danh sách của
   nó gồm đúng một thừa số.
2. **Không phải số nguyên tố** — tách thành `a` nhân phần còn lại, rồi **mượn
   dòng đã có sẵn trong bảng** của phần còn lại ấy.

Chỗ trống thứ hai là chỗ quy nạp mạnh hiện ra thành mã: dòng bảng được mượn có
thể nằm ở bất kỳ đâu bên dưới, không nhất thiết là dòng ngay trên. Với 26 thì
dòng được mượn là dòng số 13, cách dòng liền trước rất xa — và nó có sẵn ở đó
vì vòng lặp đã đi qua 13 từ lâu.

Một luật của bài: **phần còn lại phải tra trong `bang`**, đừng viết thẳng hai
thừa số ra thành một danh sách hai phần tử. Tách một lần là chưa hết việc: 8
tách thành 2 nhân 4, mà 4 chưa phải số nguyên tố.

Bài chấm bằng nhiều dòng của bảng, chọn để cư xử khác hẳn nhau: một số nguyên
tố, một số có thừa số lặp lại, một số phải mượn dòng nằm rất sâu bên dưới.

```python title=starter
# Ước nhỏ nhất từ 2 trở lên của n. Bằng đúng n thì n là số nguyên tố (T2.1).
def uoc_nho_nhat(n):
    d = 2
    while d * d <= n:
        if n % d == 0:
            return d
        d = d + 1
    return n


bang = {}
n = 2
while n <= 30:
    a = uoc_nho_nhat(n)
    if a == n:
        bang[n] = ___
    else:
        bang[n] = ___
    n = n + 1

print(bang[12])
print(bang[13])
print(bang[30])
print(len(bang))
```

```python title=solution
# Ước nhỏ nhất từ 2 trở lên của n. Bằng đúng n thì n là số nguyên tố (T2.1).
def uoc_nho_nhat(n):
    d = 2
    while d * d <= n:
        if n % d == 0:
            return d
        d = d + 1
    return n


bang = {}
n = 2
while n <= 30:
    a = uoc_nho_nhat(n)
    if a == n:
        bang[n] = [n]
    else:
        bang[n] = [a] + bang[n // a]
    n = n + 1

print(bang[12])
print(bang[13])
print(bang[30])
print(len(bang))
```

```python title=test
# Ba câu đầu canh đúng ba cái bẫy của bài, nên chúng đứng trước: dừng lại sau
# một lần tách, mượn nhầm dòng liền trước, và chép nguyên số vào chỗ số nguyên
# tố. Xếp chúng sau một câu dễ thì có bẫy không bao giờ sập.
assert bang[8] == [2, 2, 2], "8 tách thành 2 nhân 4, mà 4 chưa phải số nguyên tố nên còn phải mượn tiếp dòng của nó — dừng ở hai thừa số là chưa xong"
assert bang[26] == [2, 13], "26 tách thành 2 nhân 13, và dòng phải mượn là dòng 13, không phải dòng 25 nằm ngay trên"
assert bang[13] == [13], "13 là số nguyên tố: danh sách của nó gồm đúng một thừa số là chính nó"
assert bang[2] == [2], "2 là cơ sở của cả bảng — nó nguyên tố, nên danh sách của nó chỉ có chính nó"
assert bang[12] == [2, 2, 3], "12 tách thành 2 nhân 6, rồi mượn dòng 6 vốn đã là hai thừa số 2 và 3"
assert bang[27] == [3, 3, 3], "27 tách thành 3 nhân 9, và 9 còn tách được nữa thành 3 nhân 3"
assert bang[30] == [2, 3, 5], "30 tách thành 2 nhân 15, rồi mượn dòng 15 vốn đã là hai thừa số 3 và 5"
assert bang[16] == [2, 2, 2, 2], "16 phải mượn ba lần liên tiếp mới hết: 16 xuống 8, 8 xuống 4, 4 xuống 2 — thành bốn thừa số 2"
assert len(bang) == 29, "bảng phải có đủ mọi số từ 2 tới 30, tức 29 dòng — thiếu dòng nào thì có bậc chưa đổ"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều nhận một DANH SÁCH các thừa số. Đọc lại hai gạch đầu dòng ngay trên khung mã: gạch thứ nhất tả trường hợp số nguyên tố, gạch thứ hai tả trường hợp tách được. Và để ý ba cái tên đang bày sẵn ngay tại đó — `n` là số đang xét, `a` là ước vừa tìm được, còn `bang` là chỗ chứa mọi dòng đã dựng xong từ trước.
- kind: strategy
  body: "Trường hợp thứ nhất ngắn: số nguyên tố thì danh sách chỉ gồm chính nó, tức một danh sách một phần tử. Trường hợp thứ hai gồm hai mẩu nối lại: mẩu đầu là ước `a` vừa tách ra, đứng một mình trong một danh sách một phần tử; mẩu sau là dòng đã có sẵn của phần còn lại, mà phần còn lại tính bằng phép chia lấy phần nguyên. Nối hai danh sách bằng dấu cộng, đúng cách T1.3 đã dùng."
- kind: one-line
  body: "Chỗ thứ nhất là `[n]`; chỗ thứ hai là `[a] + bang[n // a]`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: chỗ trống thứ hai phải TRA LẠI bảng cho phần còn lại — viết thẳng hai thừa số ra là dừng sau một lần tách, và bài đang chấm một thứ nó không hề kiểm
  requireAst:
  # Khung khởi đầu đọc tên `bang` 6 lần (hai dòng gán vào ô, ba dòng `print`,
  # một dòng `len`). Lời giải đọc 7 — chỗ thứ bảy chính là lần TRA LẠI bảng.
  # Luật này một mình chặn đáp án `[a, n // a]`, thứ mà cột số nguyên tố không
  # phân biệt nổi với lời giải thật.
  - kind: uses-name, target: bang, min: 7
  # Ước vừa tách ra phải có mặt trong danh sách, và phải được dùng để tính
  # phần còn lại. Khung khởi đầu đọc `a` 1 lần (trong `if a == n`).
  - kind: uses-name, target: a, min: 3
  # Phần còn lại tính bằng phép chia lấy phần nguyên. Khung khởi đầu có 0 dấu.
  - kind: uses-operator, target: '//', min: 1
  # Hai danh sách nối lại bằng dấu cộng. Khung khởi đầu có 2 dấu cộng
  # (`d = d + 1` và `n = n + 1`); lời giải có 3.
  - kind: uses-operator, target: '+', min: 3
  forbidAst:
  # Lưới thứ hai, chặn ba con số là KẾT QUẢ của việc tra bảng chứ không phải
  # nguyên liệu: 6, 15 và 5 lần lượt là phần còn lại của 12, của 30 và một
  # thừa số của 30. Mọi cách viết hợp lệ đều dựng từ `n`, `a` và `bang`, nên
  # không cách nào chứa nguyên văn ba số ấy.
  - kind: has-literal, target: 6
  - kind: has-literal, target: 15
  - kind: has-literal, target: 5
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[2, 2, 3\]\n\[13\]\n\[2, 3, 5\]\n29\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Dòng nào cũng dựng được, vì mọi dòng dưới nó đã nằm sẵn ở đó rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Để ý cách cái bảng vừa rồi được dựng: **bậc thứ nhất, bậc thứ hai, bậc thứ
ba…** — quy nạp chạy theo bậc, và bậc chạy theo một thứ tự không bao giờ nhảy
cóc.

Vòng `while` cũng chạy theo một thứ tự y hệt: lượt 1, lượt 2, lượt 3… Và bạn đã
gõ vòng `while` từ T1.2, gõ rất nhiều, mà chưa bao giờ chứng minh điều gì về
nó.

Lấy đúng cuốn sổ quỹ của CLB cờ vua lớp 6A — sáu tháng, mỗi tháng một khoản —
và cái vòng cộng dồn quen thuộc của T1.2.12:

```text
tong = 0
i = 0
mỗi lượt: cộng khoản thứ i vào tong, rồi đẩy i lên một
```

Sau **mỗi** lượt, có câu nào cứ đúng đi đúng lại không?

Bài sau đi tìm câu ấy.
::::

::::checkpoint{mastery=0.8}
::::
