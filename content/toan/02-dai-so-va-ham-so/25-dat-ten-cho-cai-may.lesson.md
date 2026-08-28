---
id: toan.dai-so-va-ham-so.dat-ten-cho-cai-may
title: Đặt tên cho cái máy
summary: Quán có ba cái máy nên phải có ba cái tên — và `t(20)` là thứ máy `t` nhả ra khi bỏ 20 vào, không phải `t` nhân với 20.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.function-notation]
requires: [math.function, math.multiplication, core.function-def, core.function-parameter, core.function-return, core.function-call, core.variable, core.print-variable]
concepts: [math.ten-cua-may, math.dau-vao, math.dau-ra]
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
Ba cái máy đứng cạnh nhau. Gọi cái nào cũng là "cái máy" thì mình gọi ai đây?
::::

::::explain{#ba-cai-may-mot-cai-ten}
Bài trước chốt được một luật: **cái máy** là quy tắc mà mỗi đầu vào cho ra đúng
một đầu ra. Luật ấy đúng, và nó vừa để lại một rắc rối rất cụ thể: quán không có
*một* cái máy. Quán có ba, và cả ba đều nhận cùng một thứ — **số ổ bánh mì bán
được**.

- Bỏ số ổ vào, nhả ra **tiền thu**: mỗi ổ 15 000 đồng.
- Bỏ số ổ vào, nhả ra **tiền lãi**: mỗi ổ lãi 6 000 đồng sau khi trừ vốn, rồi cả
  buổi còn phải bớt 30 000 đồng tiền thuê chỗ.
- Bỏ số ổ vào, nhả ra **số chai nước cần lấy** cho buổi sau: Byte lấy gấp ba số
  ổ, vì trưa nắng khách uống nhiều.

Bây giờ thử nói câu này ra miệng: *"bỏ 20 vào cái máy thì được bao nhiêu?"*

Câu ấy chưa hỏi được gì cả. 20 ổ cho ra 300 000, hay 90 000, hay 60? Ba câu trả
lời đều đúng, và đúng cho ba máy khác nhau. Chỗ tắc không nằm ở toán — mọi phép
tính trong này bạn đã làm được từ bài 17. Nó nằm ở chỗ **chưa có tên**.

Cách gỡ thì bạn đã thấy một lần rồi, ở bài 2. Hồi đó, một câu tính có hai ô
trống mà không phân biệt được ô nào là ô nào; cách gỡ là dán lên mỗi ô một chữ
cái. Lần này cũng dán chữ cái — chỉ khác chỗ dán:

> Chữ cái ở bài 2 dán lên một **ô trống**. Chữ cái ở đây dán lên **cả cái máy**.

Byte dán ba cái tên: `t` cho máy tiền thu, `l` cho máy tiền lãi, `c` cho máy
chai nước. Và đây là cách viết trọn một cái máy ra giấy:

> **`t(n) = 15000 × n`**
>
> Đọc: *"máy `t`: bỏ `n` vào thì nhả ra `15000 × n`."*

Trong ngoặc là **chỗ trống của máy** — chính là chữ cái bạn đã quen từ bài 2,
giờ đứng ở một vị trí mới để nói rõ "đây là chỗ đút đồ vào".

Còn khi đã có số thật để đút vào thì viết luôn số ấy vào ngoặc:

> **`t(20) = 300000`**
>
> Đọc: *"tê của hai mươi bằng ba trăm nghìn."*

Ba máy, ba tên, ba câu trả lời không lẫn vào nhau được nữa:

> `t(n) = 15000 × n` — `t(20) = 300000`
>
> `l(n) = 6000 × n − 30000` — `l(20) = 90000`
>
> `c(n) = 3 × n` — `c(20) = 60`

**Một chỗ phải cẩn thận, vì nó trông y hệt một thứ khác.** `t(20)` **không**
phải `t` nhân với 20. Từ bài 7 bạn quen viết `15000n` để nói `15000 × n` — một
**con số** đứng sát cái gì đó thì đúng là nhân. Còn
`t` một mình không phải con số nào cả; nó là một cái máy. Nhân một cái máy với
20 thì không có nghĩa gì. Dấu ngoặc đứng ngay sau **tên một cái máy** mang nghĩa
khác hẳn: *bỏ cái này vào.*
::::

::::example{#ba-may-ba-ten}
Cùng một con số 20 đi vào ba cái máy có tên, ra ba chỗ khác nhau:

```text
                ┌───────┐
      20  ───→  │   t   │  ───→  300000   đồng thu
                └───────┘
                ┌───────┐
      20  ───→  │   l   │  ───→   90000   đồng lãi
                └───────┘
                ┌───────┐
      20  ───→  │   c   │  ───→      60   chai nước
                └───────┘
```

Bây giờ tới chỗ đáng ngạc nhiên. Cái ký hiệu vừa học không phải thứ mới với
bạn — bạn đã gõ nó từ Realm 0, chỉ là chưa ai gọi nó bằng tên toán học. Trong
Python, tên máy viết dài ra cho dễ đọc, nhưng từng mảnh khớp một–một:

```python title=readonly
def thu(n):
    return 15000 * n

def lai(n):
    return 6000 * n - 30000

def nuoc(n):
    return 3 * n

print(thu(20))
print(lai(20))
print(nuoc(20))
```

Máy in ra:

```text
300000
90000
60
```

Đặt hai cách viết cạnh nhau thì thấy chúng là **một**:

| Trong vở toán | Trong Python | Nghĩa |
|---|---|---|
| `t` | `thu` | tên của cái máy |
| `n` trong `t(n)` | `n` trong `def thu(n)` | chỗ trống để đút đồ vào |
| `= 15000 × n` | `return 15000 * n` | luật: nhả ra cái gì |
| `t(20)` | `thu(20)` | thứ máy nhả ra khi bỏ 20 vào |

Ký hiệu `t(n)` ra đời ở châu Âu thế kỷ 18, `def` ra đời năm 1991. Chúng giống
nhau đến mức này vì cả hai đang tả cùng một vật: một quy tắc có chỗ trống.
::::

::::predict{#doan-ba-may commitOnce}
Byte bỏ **cùng một con số 8** vào cả ba máy.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def thu(n):
    return 15000 * n

def lai(n):
    return 6000 * n - 30000

def nuoc(n):
    return 3 * n

print(thu(8))
print(lai(8))
print(nuoc(8))
```

:::opt{correct}
120000 rồi 18000 rồi 24
:::

:::opt
120000 ba lần — vì cả ba dòng đều bỏ đúng con số 8 vào
::why
Gần đúng ở chỗ bạn đọc trúng phần khó nhất của bài: cả ba lần gọi đều đưa vào
**cùng một** đầu vào, không lần nào đưa số khác. Điều đó thật, và nó chính là lý
do bài này tồn tại.

Chỗ lệch nằm ở chỗ cái gì quyết định đầu ra. Quy tắc bạn đang dùng — *cùng đầu
vào thì cùng đầu ra* — là quy tắc đúng, nhưng nó chỉ đúng **trong lòng một cái
máy**: bài trước nói `thu(8)` hôm nay và `thu(8)` ngày mai phải bằng nhau. Ranh
giới của nó là ba máy khác nhau. Ở đây một đầu ra cần **hai** thứ: con số trong
ngoặc, và cái tên đứng trước ngoặc. Đổi tên là đổi luật.
::
:::

:::opt
Máy báo lỗi ở dòng `print(thu(8))`, vì `thu` chưa giữ con số nào để nhân với 8
::why
Gần đúng ở chỗ bạn nhớ đúng một quy tắc có thật: một cái tên đứng sát trước dấu
ngoặc thì thường là phép nhân. `15000(8)` đúng là 120 000, và trong vở toán suốt
T2.1 bạn đọc như thế không sai lần nào.

Ranh giới của quy tắc ấy là **cái tên đó đang giữ gì**. Nó là phép nhân khi hai
bên đều là số. Nhưng `thu` không giữ một con số — dòng `def thu(n):` dán cái tên
ấy lên một **quy tắc**, và nhân một quy tắc với 8 thì không có nghĩa. Vì `thu`
không phải số, dấu ngoặc buộc phải mang nghĩa còn lại: *bỏ 8 vào máy này.*
::
:::

:::opt
Máy báo lỗi, vì `n` chưa được đặt tên ở đâu trước khi `def thu(n)` dùng tới nó
::why
Gần đúng ở chỗ bạn đang áp đúng một luật mà Realm 0 đã dạy và nó nghiêm túc: tên
phải có giá trị *trước* khi bạn đọc nó, không thì máy dừng và báo `NameError`.
Bạn đọc mã theo đúng thứ tự trên xuống — đó là thói quen tốt.

Ranh giới nằm ở chỗ `n` trong `def thu(n)` không phải một cái tên bạn đặt ở
ngoài. Nó là **chỗ trống của riêng cái máy**, đúng nghĩa "ô trống" mà cả track
này nói từ bài 1: nó chưa giữ số nào, và nó không cần giữ số nào cho tới lúc có
người gọi. Con số điền vào nó đến từ trong ngoặc lúc gọi — `thu(8)` là lúc số 8
nhảy vào ô trống ấy.
::
:::
::::

::::explain{#may-va-thu-may-nha-ra}
Có hai vật rất khác nhau mà cách viết chỉ chênh nhau một cặp ngoặc. Tách chúng
ra một lần cho dứt điểm:

- **`t`** một mình là **cái máy**. Nó không phải con số. Không cộng được, không
  so được với 5. Nó là *một luật*.
- **`t(n)`** là **cách tính đang chờ** — đúng cái vật của bài 3, chưa ra số vì
  `n` còn trống.
- **`t(20)`** là **một con số**, và là con số 300 000. Chỗ trống đã được điền,
  máy đã chạy xong, kết quả nằm trên bàn.

Vì `t(20)` là một con số thật, nó dùng được ở mọi chỗ mà một con số dùng được:

> `t(20) − l(20) = 300000 − 90000 = 210000`

Đó là tiền vốn của 20 ổ cộng tiền thuê chỗ — thứ Byte đã bỏ ra trong buổi ấy.
Không có tên máy thì câu này viết ra là một mớ `15000 × 20 − (6000 × 20 −
30000)`; có tên máy thì nó đọc gần như tiếng Việt: *thu của hai mươi trừ lãi của
hai mươi*.

Và đây là chỗ cái tên trả công lần nữa. Từ giờ, muốn nói về **cả** cái máy — mọi
đầu vào, mọi đầu ra, cả cái bảng của nó — bạn chỉ cần một chữ. Đó đúng là thứ
bài sau cần tới.
::::

::::code{#ba-may-cua-quan}
Dựng lại đúng ba cái máy của quán, rồi hỏi cả ba cùng một buổi: sáng nay Byte
bán **20 ổ**.

- `thu` — mỗi ổ 15 000 đồng.
- `lai` — mỗi ổ lãi 6 000 đồng, rồi cả buổi bớt 30 000 đồng tiền thuê chỗ.
- `nuoc` — số chai nước cần lấy cho buổi sau, gấp ba số ổ.

Sáu chỗ trống: ba chỗ tả **luật của máy**, ba chỗ **bỏ 20 vào** từng máy.

Bài chấm bằng cả ba máy chứ không phải một, và bằng nhiều đầu vào chứ không phải
mỗi số 20 — vì một cái máy chỉ chạy đúng ở một chỗ thì chưa phải cái máy. Gõ
thẳng `300000` vào chỗ trống sẽ không qua được: máy `thu` còn bị hỏi ở số 7 nữa.

```python title=starter
def thu(n):
    return ___

def lai(n):
    return ___

def nuoc(n):
    return ___

# Sáng nay Byte bán 20 ổ. Hỏi cả ba máy cùng một câu.
thu_sang = ___
lai_sang = ___
nuoc_sang = ___

print(thu_sang)
print(lai_sang)
print(nuoc_sang)
```

```python title=solution
def thu(n):
    return 15000 * n

def lai(n):
    return 6000 * n - 30000

def nuoc(n):
    return 3 * n

# Sáng nay Byte bán 20 ổ. Hỏi cả ba máy cùng một câu.
thu_sang = thu(20)
lai_sang = lai(20)
nuoc_sang = nuoc(20)

print(thu_sang)
print(lai_sang)
print(nuoc_sang)
```

```python title=test
# Ba câu `!=` đứng trước: chúng canh đúng cái bẫy "ba máy hoá một". Xếp sau các
# câu `==` thì chúng không bao giờ chạy tới, và cái bẫy không bao giờ sập.
assert thu_sang != lai_sang, "cùng bỏ 20 vào, nhưng máy thu và máy lai là hai luật khác nhau nên không được ra cùng một số"
assert lai_sang != nuoc_sang, "máy lai nhả ra tiền, máy nuoc nhả ra số chai — hai kết quả này không thể bằng nhau"
assert thu(7) != thu(8), "máy thu phải tính THEO chỗ trống n, nên bỏ 7 và bỏ 8 vào phải ra hai số khác nhau — gõ cứng một con số thì máy trả về đúng nó ở mọi đầu vào"
assert thu_sang == 300000, "20 ổ, mỗi ổ 15 000 đồng: máy thu phải nhả ra 300 000"
assert lai_sang == 90000, "20 ổ lãi 120 000, bớt 30 000 thuê chỗ: máy lai phải nhả ra 90 000"
assert nuoc_sang == 60, "20 ổ, lấy gấp ba: máy nuoc phải nhả ra 60 chai"
assert thu(7) == 105000, "cùng cái máy thu ấy, bỏ 7 vào phải ra 105 000 — luật của máy không đổi theo đầu vào"
assert nuoc(3) == 9, "bỏ 3 vào máy nuoc phải ra 9 chai"
assert lai(5) == 0, "bán 5 ổ lãi vừa đúng 30 000, đủ bù tiền thuê chỗ — máy lai phải nhả ra 0"
assert lai(0) == -30000, "chưa bán ổ nào mà đã trả 30 000 thuê chỗ, nên máy lai nhả ra -30000"
```

:::hints
- kind: attention
  body: Ba dòng gạch đầu dòng ngay trên khung mã nói ra luật của từng máy. Máy `lai` là máy duy nhất có hai phần — một phần đi theo số ổ, một phần thì không. Còn ba chỗ trống dưới không hỏi luật nữa; chúng hỏi "bỏ 20 vào máy này thì ra gì", nên trong đó phải có tên một cái máy.
- kind: strategy
  body: 'Một cái máy gồm hai phần: dòng `def` nói tên và chỗ trống, dòng `return` nói luật — và luật ấy phải viết bằng chính chỗ trống `n`, đừng viết bằng số 20, vì máy còn bị hỏi ở số khác. Ba chỗ trống cuối thì ngược lại: viết tên máy rồi mở ngoặc bỏ số 20 vào.'
- kind: one-line
  body: "Ba luật là `15000 * n`, `6000 * n - 30000`, `3 * n`; ba chỗ gọi là `thu(20)`, `lai(20)`, `nuoc(20)`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi `return` phải tính bằng chính chỗ trống `n` của máy, và mỗi chỗ trống dưới phải GỌI một cái máy bằng cách viết tên nó rồi mở ngoặc — gõ thẳng con số thì không có cái máy nào chạy cả
  requireAst:
  # Khung khởi đầu không đọc `n` lần nào (dòng `def thu(n)` là chỗ ĐẶT tên, không
  # phải chỗ đọc). Ba luật máy phải đọc `n` mỗi cái một lần, nên `min: 3` chặn
  # đúng đáp án viết luật bằng con số 20 thay vì bằng ô trống.
  - kind: uses-name, target: n, min: 3
  - kind: uses-operator, target: *, min: 3
  # Máy `lai` là máy duy nhất còn phải bớt tiền thuê chỗ. Thiếu dấu trừ nghĩa là
  # khoản ấy bị bỏ quên.
  - kind: uses-operator, target: -, min: 1
  # Ba luật dưới đây chặn đáp án gõ cứng ba con số kết quả: mỗi cái buộc phải có
  # ít nhất một lần GỌI đúng cái máy ấy. Khung khởi đầu không gọi máy nào.
  - kind: uses-call, target: thu, min: 1
  - kind: uses-call, target: lai, min: 1
  - kind: uses-call, target: nuoc, min: 1
  forbidAst:
  # Lưới thứ hai, chặn đúng ba con số là KẾT QUẢ. Lời giải thật chỉ chứa 15000,
  # 6000, 30000, 3 và 20 — không chỗ nào có nguyên văn 300000, 90000 hay 60.
  - kind: has-literal, target: 300000
  - kind: has-literal, target: 90000
  - kind: has-literal, target: 60
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^300000\n90000\n60\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba cái tên, ba câu trả lời, không lẫn nhau nữa. Giờ mình gọi đúng máy mình cần.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cái máy tiền thu giờ đã có tên gọi đàng hoàng: `t(n) = 15000 × n`.

Ở bài 23, bạn đã chấm bảng của **đúng cái máy này** lên mặt phẳng. Chín cái chấm
hôm ấy nằm thẳng hàng — thẳng đến mức đặt được cây thước lên mà không chấm nào
lệch ra.

Nhưng chưa ai hỏi **vì sao** chúng thẳng. Không có luật nào bắt phải thế. Hoàn
toàn tưởng tượng được một cái máy mà các chấm của nó cong xuống, hoặc gãy một
khúc ở giữa, hoặc nhảy lên nhảy xuống không theo hình gì. Vậy `t` có gì mà các
chấm của nó không làm thế?

Đừng nhìn vào hình để tìm câu trả lời — hình chỉ cho thấy *kết quả*. Nhìn vào
cái bảng: đi từ dòng này xuống dòng ngay dưới nó, cột kết quả đổi bao nhiêu? Rồi
làm lại phép ấy ở một chỗ khác trên bảng, xa hơn về cuối. Con số vừa tìm được có
đổi theo chỗ bạn đứng không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
