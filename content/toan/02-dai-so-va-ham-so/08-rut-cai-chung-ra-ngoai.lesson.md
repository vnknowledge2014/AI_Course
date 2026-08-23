---
id: toan.dai-so-va-ham-so.rut-cai-chung-ra-ngoai
title: Rút cái chung ra ngoài
summary: Đọc luật mở ngoặc theo chiều ngược lại — tìm phần chung của mọi cụm rồi đặt nó ra trước ngoặc, và câu tính lộ lại cấu trúc mà nó đang giấu.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.factor-common]
requires: [math.expand-brackets, math.expression, math.letter-names-a-slot, math.multiply-distributive, math.parentheses, math.order-of-operations, core.variable, core.arithmetic, core.print-variable, core.output]
concepts: [math.mo-hinh-vung, math.cau-truc, math.o-trong]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Sợi dây gỡ ra được. Nhưng hai miếng khay phải nhận ra nhau trước đã.
::::

::::explain{#so-ghi-khong-doc-ra-o}
Cuối buổi, Byte ghi tiền thu vào sổ:

```text
15000n + 30000
```

Mẹ Byte đọc sổ rồi hỏi một câu rất thường: **"Vậy sáng nay quán ra bao nhiêu ổ
bánh?"**

Nhìn vào dòng sổ ấy mà trả lời thì vướng. Cụm đầu còn nhắc tới ổ — `15000n` là
tiền của `n` ổ. Nhưng cụm sau chỉ còn là **30 000 đồng**. Nó không nói ổ nào
cả. Muốn moi ra số ổ thì phải nhớ trong đầu rằng 30 000 vốn là tiền của 2 ổ dì
Tư — mà cái nhớ ấy không nằm trên giấy, nên người khác cầm sổ lên đọc là chịu.

Còn dạng kia thì trả lời được ngay:

```text
15000 × (n + 2)
```

Ở đây `n + 2` **là** số ổ, đọc thẳng ra khỏi mặt giấy, không cần nhớ gì thêm.

Nên chuyện hôm nay không phải "tính cho nhanh". Hai dạng ấy ra cùng một số tiền
ở mọi cách điền — bài trước đã lo xong chỗ đó. Chuyện hôm nay là **đọc lại cấu
trúc**: gấp câu tính về dạng cho thấy nó được dựng từ cái gì.

Bài trước đi một chiều: có ngoặc → bỏ ngoặc. Hôm nay đi chiều còn lại.
::::

::::explain{#cai-chung-phai-hien-ra}
Luật hôm qua đọc xuôi:

```text
a × (b + c)  →  a × b + a × c
```

Đọc ngược nó, từ phải sang trái:

```text
a × b + a × c  →  a × (b + c)
```

Vế trái có hai cụm, và cả hai cụm đều mang `a`. Chính vì `a` có mặt ở **mọi**
cụm nên nó mới được đứng chung ra trước ngoặc. Người ta gọi nó là **phần chung**
của các cụm.

Nhưng câu sổ của Byte không nằm sẵn ở dạng ấy:

```text
15000 × n  +  30000
```

Cụm đầu có 15 000 nhìn thấy được. Cụm sau thì không — nó đã bị tính xong thành
một con số, và cái 15 000 bên trong biến mất khỏi mặt giấy.

Nên bước thật của bài này là **bắt phần chung hiện ra trước đã**: viết mỗi cụm
thành *phần chung × phần còn lại*.

```text
  15000 × n   +   30000
                    │
                    │  30000 chính là 15000 × 2
                    ↓
  15000 × n   +   15000 × 2
      │               │
      └──── chung ────┘
            15000
              ↓
      15000 × (n + 2)
```

Đúng cái khay bánh của bài trước, nhìn ngược chiều. Hôm qua sợi dây cắt khay
làm hai miếng. Hôm nay bạn nhặt hai miếng lên, thấy chúng cùng **cao** 15 000
đồng mỗi ổ, nên ghép sát vào nhau được thành một khay dài `n + 2` ổ:

```text
    15000 × n     15000 × 2              15000 × (n + 2)
  ┌───────────┐    ┌─────┐             ┌─────────────────┐
  │           │    │     │   ghép  →   │                 │
  │    n ổ    │    │ 2 ổ │             │     n + 2 ổ     │
  └───────────┘    └─────┘             └─────────────────┘
```

Hai miếng chỉ ghép được vì chúng cùng chiều cao. Đó là toàn bộ nội dung của
chữ "chung".
::::

::::example{#hoi-thang-cai-may}
Bắt máy làm trọng tài trên hai giá trị rất khác nhau:

```python title=readonly
n = 6
print(15000 * n + 30000)
print(15000 * (n + 2))

n = 100
print(15000 * n + 30000)
print(15000 * (n + 2))
```

Máy in ra:

```text
120000
120000
1530000
1530000
```

Máy xác nhận hai lần điền, không hơn. Lý do bắt chúng bằng nhau ở **mọi** lần
điền vẫn là cái khay — hai miếng cùng chiều cao thì ghép lại không thừa không
thiếu ổ nào.
::::

::::predict{#doan-bon-dong commitOnce}
Byte thử bốn cách viết cho cùng một buổi sáng. Dòng cuối là dòng Byte gấp vội:
đặt 15 000 ra trước ngoặc rồi bê nguyên 30 000 vào trong.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
n = 6
print(15000 * n + 30000)
print(15000 * (n + 2))
print(3000 * (5 * n + 10))
print(15000 * (n + 30000))
```

:::opt{correct}
120000, 120000, 120000 rồi 450090000
:::

:::opt
120000 cả bốn dòng
::why
Gần đúng ở chỗ bạn tin rằng gấp lại không được đổi giá trị — đó chính là điều
bài này dựng, và ba dòng đầu chứng minh bạn đúng.

Chỗ lệch nằm ở **thứ được bỏ vào trong ngoặc**. Khi 15 000 bước ra ngoài, thứ ở
lại trong ngoặc phải là *phần còn lại sau khi đã lấy phần chung ra* — cụm 30 000
phải được viết thành `15000 × 2` trước, rồi con số ở lại mới là 2. Dòng bốn bê
nguyên 30 000 vào, tức là nói "khay này dài `n + 30000` ổ" — một cái khay 30 006
ổ bánh mì. Máy tính ra 450 090 000 đồng và không thấy có gì lạ.
::
:::

:::opt
120000, 120000, 90010 rồi 450090000
::why
Gần đúng ở chỗ bạn đọc dòng ba rất kỷ luật theo luật nhân trước cộng sau:
`3000 × 5 × n` xong mới cộng 10, ra 90 010. Với một câu tính **không có ngoặc**
thì cách đọc ấy đúng từng chữ.

Ranh giới là cặp ngoặc ở dòng ba. Ngoặc được làm trước thứ tự mặc định, nên
`5 × n + 10` phải tính trọn thành 40 rồi mới nhân với 3 000. Và cặp ngoặc ấy có
lý do tồn tại: dòng ba là một cách gấp **khác** cho cùng câu tính — rút 3 000 ra
thay vì 15 000. Nó vẫn ra 120 000, chỉ là rút chưa hết nên thứ trong ngoặc chưa
đọc ra được số ổ.
::
:::

:::opt
120000, 120000, 120000 rồi máy báo lỗi vì con số quá lớn
::why
Gần đúng ở một chỗ rất đáng khen: bạn thấy dòng bốn sẽ cho ra một con số vô lý
cho một xe bánh mì, và bạn phản ứng. Linh cảm ấy đúng — nó là dấu hiệu gấp sai.

Ranh giới là **ai chịu trách nhiệm bắt cái sai ấy**. Số nguyên trong Python
không có trần: 450 090 000 hay lớn hơn nữa nó vẫn tính gọn. Máy không báo lỗi,
không cảnh báo, không hỏi lại — nó lặng lẽ đưa ra một con số đúng với câu bạn
gõ, và câu bạn gõ thì tả một cái khay 30 006 ổ.

Đó là lý do bài này dạy luật chứ không dạy mẹo bấm máy. Máy bắt được lỗi gõ
sai; nó không bắt được lỗi **nghĩ sai**. Chỗ đó chỉ có cái khay bánh bắt được.
::
:::
::::

::::explain{#luat-va-ranh-gioi}
Đặt tên cho thứ vừa thấy:

> **Rút phần chung ra ngoài**: `a × b + a × c = a × (b + c)`
>
> Phần chung là thứ có mặt ở **mọi** cụm. Đặt nó ra trước ngoặc, trong ngoặc
> giữ lại phần còn lại của từng cụm.

Ba điều đi kèm, và đều là ranh giới chứ không phải mẹo:

- **Không thấy phần chung không có nghĩa là không có.** `15000n + 30000` giấu
  mất cái 15 000 thứ hai. Cách moi nó ra là hỏi: *30 000 là 15 000 nhân với
  mấy?* — hỏi xong thì phần chung hiện ra.
- **Rút chưa hết vẫn là một câu đúng.** `15000n + 30000` cũng bằng
  `3000 × (5n + 10)`, cũng bằng `5000 × (3n + 6)`. Ba câu ấy đều đúng ở mọi
  cách điền. Chỉ khác chỗ: rút hết được 15 000 ra thì trong ngoặc còn `n + 2`,
  và `n + 2` **là số ổ bánh** — đọc được. Rút một nửa thì trong ngoặc còn `5n + 10`,
  chẳng phải số ổ, chẳng phải số tiền, chẳng là gì cả.
- **Có câu không rút được gì.** `15000n + 7`: hai cụm này không chung nhau con
  số nào ngoài 1, mà `1 × (15000n + 7)` thì chỉ dài thêm ra. Câu ấy vẫn hoàn
  toàn hợp lệ — chỉ là nó không được dựng từ một phần chung nào, nên không có gì
  để gấp lại.
::::

::::code{#gap-lai-hai-cuon-so}
Byte đưa bạn hai cuốn sổ, cuốn nào cũng đang ở dạng **đã mở ngoặc** — nhìn ra
tiền nhưng không nhìn ra số hàng. Việc của bạn là gấp mỗi cuốn lại cho số hàng
hiện ra.

- **Sổ bánh mì**: `15000 * n + 30000`. Mỗi ổ 15 000 đồng, nên 30 000 là tiền
  của mấy ổ?
- **Sổ nước**: `8000 * m + 24000`. Mỗi chai 8 000 đồng, nên 24 000 là tiền của
  mấy chai?

Bài chấm bằng **cả hai cuốn sổ**, và chúng khác nhau ở cả giá lẫn số hàng gửi
sẵn — nên không con số nào chép được từ chỗ trống này sang chỗ trống kia. Mỗi
dòng bạn viết còn được đối chiếu với chính dạng đã mở ngoặc ở ngay trên nó: gấp
lại mà đổi giá trị thì lệch ngay.

```python title=starter
# Sổ bánh mì: tiền của n ổ bán lẻ, cộng 30 000 đồng tiền bánh dì Tư lấy sẵn.
n = 6
so_banh = 15000 * n + 30000
gap_lai_banh = ___

# Sổ nước: tiền của m chai bán lẻ, cộng 24 000 đồng tiền nước gửi sẵn.
m = 7
so_nuoc = 8000 * m + 24000
gap_lai_nuoc = ___

print(gap_lai_banh)
print(gap_lai_nuoc)
```

```python title=solution
# Sổ bánh mì: tiền của n ổ bán lẻ, cộng 30 000 đồng tiền bánh dì Tư lấy sẵn.
n = 6
so_banh = 15000 * n + 30000
gap_lai_banh = 15000 * (n + 2)

# Sổ nước: tiền của m chai bán lẻ, cộng 24 000 đồng tiền nước gửi sẵn.
m = 7
so_nuoc = 8000 * m + 24000
gap_lai_nuoc = 8000 * (m + 3)

print(gap_lai_banh)
print(gap_lai_nuoc)
```

```python title=test
# Câu `!=` đứng trước: nó canh cái bẫy chép một đáp án cho cả hai sổ. Xếp nó
# sau hai câu `==` thì nó không bao giờ chạy tới, và cái bẫy không bao giờ sập.
assert gap_lai_banh != gap_lai_nuoc, "hai sổ khác giá và khác số hàng gửi sẵn — một con số dùng cho cả hai là không được"
assert gap_lai_banh == so_banh, "gấp lại không được đổi giá trị: dạng có ngoặc phải bằng đúng 15000 * n + 30000"
assert gap_lai_nuoc == so_nuoc, "gấp lại không được đổi giá trị: dạng có ngoặc phải bằng đúng 8000 * m + 24000"
assert gap_lai_banh == 120000, "6 ổ bán lẻ cộng 2 ổ của dì Tư, mỗi ổ 15 000 đồng, là 120 000 đồng"
assert gap_lai_nuoc == 80000, "7 chai bán lẻ cộng 3 chai gửi sẵn, mỗi chai 8 000 đồng, là 80 000 đồng"
```

:::hints
- kind: attention
  body: Nhìn con số đứng một mình ở cuối mỗi dòng sổ — 30 000 và 24 000. Nó là tiền của mấy món? Chia nó cho giá một món là ra, và chính con số ấy mới là thứ được bỏ vào trong ngoặc.
- kind: strategy
  body: Đặt giá một món ra trước ngoặc. Trong ngoặc là chữ cộng với số món gửi sẵn — tức tổng số món. Với sổ bánh mì, 30 000 là tiền của 2 ổ; với sổ nước, 24 000 là tiền của 3 chai.
- kind: one-line
  body: "Chỗ trống thứ nhất là `15000 * (n + 2)`, chỗ trống thứ hai là `8000 * (m + 3)`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là giá một món đặt trước một cặp ngoặc, và trong ngoặc là chữ cộng với SỐ MÓN gửi sẵn — chép lại dòng sổ ở trên hoặc gõ thẳng số tiền thì cái luật bài này vừa dựng không xuất hiện ở đâu cả
  requireAst:
  # Con số bỏ vào trong ngoặc chính là thứ cả bài đi tìm: 30000 ÷ 15000 = 2 và
  # 24000 ÷ 8000 = 3. Khung khởi đầu không chứa số 2 hay số 3 nào, nên hai luật
  # này chặn được mọi kiểu điền bừa và cả kiểu chép lại dòng sổ.
  - kind: has-literal, target: 2
  - kind: has-literal, target: 3
  # Mỗi chữ phải được ĐỌC thêm một lần nữa ở dòng gấp lại. Không có hai luật
  # này thì `gap_lai_banh = so_banh` cũng lọt, mà đó là chép chứ không phải gấp.
  - kind: uses-name, target: n, min: 2
  - kind: uses-name, target: m, min: 2
  # Khung khởi đầu có hai dấu cộng và hai dấu nhân; lời giải thêm mỗi loại hai
  # cái nữa — một cho mỗi cuốn sổ.
  - kind: uses-operator, target: +, min: 4
  - kind: uses-operator, target: *, min: 4
  forbidAst:
  # Lưới thứ hai, chặn đúng hai con số KẾT QUẢ. Lời giải thật không chứa nguyên
  # văn chúng, nên luật này không cản ai làm thật.
  - kind: has-literal, target: 120000
  - kind: has-literal, target: 80000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^120000\n80000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Giờ sổ đọc ra được số ổ rồi. Mẹ hỏi lần nữa là mình trả lời liền.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hôm nay phần chung luôn là một **con số**: 15 000 đồng một ổ, 8 000 đồng một
chai. Nó đứng ra trước ngoặc vì mọi cụm đều mang nó.

Nhưng luật không hề nói phần chung phải là số. Nó nói phần chung là *thứ có mặt
ở mọi cụm*. Vậy còn thứ kia trên mặt giấy thì sao — cái **ô trống**?

Buổi sáng, khách quen của Byte ai cũng mua đúng như nhau: mỗi người `n` ổ. Sáng
có 3 khách quen, chiều có 5 khách quen. Số bánh cả ngày viết ra là:

```text
3n + 5n
```

Hai cụm này không chung nhau con số nào — 3 với 5 chẳng chung gì. Nhưng nhìn kỹ
lại xem: có thật là chúng không chung gì không?

Và nếu rút được thứ ấy ra ngoài, thì **trong ngoặc còn lại cái gì**?

Bài sau rút chính chữ ra.
::::

::::checkpoint{mastery=0.8}
::::
