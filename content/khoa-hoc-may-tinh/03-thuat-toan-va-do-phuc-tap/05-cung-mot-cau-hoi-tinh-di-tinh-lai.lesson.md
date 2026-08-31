---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.cung-mot-cau-hoi-tinh-di-tinh-lai
title: "Cùng một câu hỏi, tính đi tính lại"
summary: "Vẽ cây cuộc gọi của fib(6) ra giấy và đếm bằng mắt: fib(2) bị hỏi lại năm lần, fib(1) bị hỏi lại tám lần — cùng câu hỏi, cùng câu trả lời, tính LẶP LẠI mỗi lần một nhánh khác đi ngang qua nó."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.recompute-waste]
requires: [alg.double-recursion]
concepts: [alg.recompute-waste]
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
`fib(2)` đâu chỉ được hỏi một lần. Vẽ cả cây ra thì thấy rõ nó bị hỏi
đi hỏi lại.
::::

::::explain{#ve-ca-cay-ra-giay}
Bài trước dừng lại ở `fib(4)`, đủ nhỏ để đọc hết nhật ký bằng mắt.
`fib(6)` lớn hơn hai bậc, và cây cuộc gọi của nó vẽ được trọn vẹn trên
giấy — hãy nhìn kỹ nó một lần, vì đây là lần cuối cùng track này còn
vẽ tay được một cây cuộc gọi Fibonacci đầy đủ:

```text
fib(6)
├─ fib(5)
│  ├─ fib(4)
│  │  ├─ fib(3)
│  │  │  ├─ fib(2)
│  │  │  │  ├─ fib(1)
│  │  │  │  └─ fib(0)
│  │  │  └─ fib(1)
│  │  └─ fib(2)
│  │     ├─ fib(1)
│  │     └─ fib(0)
│  └─ fib(3)
│     ├─ fib(2)
│     │  ├─ fib(1)
│     │  └─ fib(0)
│     └─ fib(1)
└─ fib(4)
   ├─ fib(3)
   │  ├─ fib(2)
   │  │  ├─ fib(1)
   │  │  └─ fib(0)
   │  └─ fib(1)
   └─ fib(2)
      ├─ fib(1)
      └─ fib(0)
```

Đếm bằng mắt số lần chữ `fib(2)` xuất hiện trong cây: rà từ trên
xuống, gạch dưới từng chỗ có đúng chữ `fib(2)`. Có đúng năm chỗ — hai
chỗ nằm trong nhánh con của `fib(5)` (một dưới nhánh trái của nó, một
dưới nhánh phải), một chỗ nữa cũng trong nhánh `fib(5)`, và hai chỗ
cuối nằm trong nhánh con của `fib(4)` ngoài cùng bên phải của
`fib(6)`. **Năm lần.** Cùng một câu hỏi — "Fibonacci của 2 là bao
nhiêu?" — được hỏi năm lần riêng biệt, ở năm chỗ khác nhau trên cây,
và cả năm lần đều nhận đúng một câu trả lời: `1`.

Không phải trùng hợp. `fib(2)` xuất hiện nhiều vì nó GẦN trường hợp
cơ sở — hầu như mọi nhánh, sớm muộn cũng lùi xuống gần 0 hoặc 1, và
trên đường lùi ấy nó phải đi ngang qua `fib(2)`. Đếm luôn cả `fib(1)`:
xuất hiện **tám lần** — còn nhiều hơn `fib(2)`, đúng logic đó đẩy tới
tận cùng: càng gần cơ sở, càng bị hỏi nhiều lần.

Đây chính là chỗ đau. Hàm không viết sai một dòng nào — mọi kết quả
đều đúng, `fib(6)` vẫn ra `8` như đã kiểm ở bài trước. Cái giá phải
trả không nằm ở TÍNH SAI, mà ở TÍNH THỪA: cùng một câu hỏi, cùng một
câu trả lời, bị hỏi lại nhiều lần một cách không cần thiết.
::::

::::example{#dem-that-bang-list-count}
Xác nhận đúng con số vừa đếm bằng mắt — bằng nhật ký đã quen từ hai
bài trước, cộng phương thức `.count(...)` của `list`:

```python title=readonly
nhat_ky = []

def fib(n):
    nhat_ky.append(f"ĐẨY fib({n})")
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib(n - 1) + fib(n - 2)

fib(6)

print("Số dòng nhật ký tất cả:", len(nhat_ky))
print("fib(2) bị hỏi:", nhat_ky.count("ĐẨY fib(2)"), "lần")
print("fib(1) bị hỏi:", nhat_ky.count("ĐẨY fib(1)"), "lần")
```

```text title=readonly
Số dòng nhật ký tất cả: 25
fib(2) bị hỏi: 5 lần
fib(1) bị hỏi: 8 lần
```

Đúng khớp với cây vẽ tay: năm lần cho `fib(2)`, tám lần cho `fib(1)`.
Tổng cộng hai mươi lăm lượt gọi để tính MỘT giá trị `fib(6)` — nhiều
hơn hẳn sáu bước một vòng lặp thường sẽ cần để cộng dồn từ `fib(0)`
tới `fib(6)`. `nhat_ky.count(...)` chỉ đếm lại trên một danh sách đã
ghi xong — không phải cách chính, chỉ để KIỂM chứng cây vẽ tay đúng.
::::

::::predict{#doan-cau-hoi-bi-hoi-nhieu-nhat commitOnce}
Vẫn cây cuộc gọi của `fib(6)` ở trên, với số lần xuất hiện đã đếm:
`fib(2)` — 5 lần, `fib(1)` — 8 lần.

**Trước khi đọc tiếp**, bạn đoán: trong bốn câu hỏi `fib(1)`, `fib(2)`,
`fib(3)`, `fib(4)`, câu nào bị hỏi lại NHIỀU lần nhất?

:::opt{correct}
`fib(1)` — 8 lần, nhiều hơn cả `fib(2)` (5 lần) — càng gần trường hợp
cơ sở, càng có nhiều nhánh đi ngang qua nó trên đường lùi về 0 hoặc 1
:::

:::opt
`fib(2)` — vì đó là câu hỏi cụm bài này tập trung đếm, hẳn phải là số
lớn nhất
::why
Gần đúng ở việc `fib(2)` đúng là bị hỏi lại rất nhiều — 5 lần, một
con số đáng chú ý, không phải chuyện nhỏ.

Chỗ lệch: 5 chưa phải là NHIỀU NHẤT. `fib(1)` — gần trường hợp cơ sở
hơn `fib(2)` đúng một bước — còn bị hỏi tới 8 lần, nhiều hơn hẳn. Cây
càng vẽ ra, càng thấy quy luật: đi càng gần cơ sở, càng nhiều nhánh
hội tụ về đúng chỗ đó.
::
:::

:::opt
`fib(4)` — vì nó là con số lớn nhất trong bốn lựa chọn, nên hẳn tốn
nhiều lần tính nhất
::why
Gần đúng ở trực giác "số lớn hơn thì việc nhiều hơn" — trực giác đó
đúng với TỔNG số lượt gọi cần để tính CẢ CÂY (`fib(6)` đúng là tốn
nhiều lượt gọi hơn `fib(2)` nếu tính hết cả cây con của nó).

Chỗ lệch: câu hỏi ở đây không hỏi về tổng công tính `fib(4)`, mà hỏi
riêng câu `fib(4)` — CHÍNH XÁC câu hỏi đó — bị lặp lại bao nhiêu lần.
`fib(4)` chỉ xuất hiện đúng 2 lần trong cả cây (là con trực tiếp của
`fib(5)`, và là con trực tiếp của `fib(6)`) — ÍT hơn hẳn `fib(1)` hay
`fib(2)`, vì nó ở XA trường hợp cơ sở hơn.
::
:::

:::opt
Cả bốn bị hỏi lại số lần bằng nhau, vì cây có cấu trúc đối xứng
::why
Gần đúng ở việc cây CÓ một cấu trúc lặp lại — mỗi `fib(n)` luôn sinh
đúng hai nhánh con theo cùng một luật, không phải hình dạng tuỳ tiện.

Chỗ lệch: "cùng một luật sinh cây" không có nghĩa "số lần xuất hiện
bằng nhau". Con số đếm được (`fib(1)`: 8, `fib(2)`: 5, `fib(3)`: 3,
`fib(4)`: 2) khác nhau rõ rệt, và khác theo đúng một quy luật: càng
gần cơ sở, càng xuất hiện nhiều.
::
:::
::::

::::code{#dem-fib2-va-fib4}
Xác nhận bằng mã hai con số vừa đếm bằng mắt trên cây: `fib(2)` bị
hỏi lại bao nhiêu lần, và `fib(4)` bị hỏi lại bao nhiêu lần, khi tính
`fib(6)`. Hàm ghi nhật ký đã viết sẵn và đã chạy xong — việc của bạn
chỉ là đếm lại trên `nhat_ky`.

```python title=starter
nhat_ky = []

def fib(n):
    nhat_ky.append(f"ĐẨY fib({n})")
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib(n - 1) + fib(n - 2)

fib(6)

so_lan_hoi_fib_2 = ___                 # đếm "ĐẨY fib(2)" trong nhat_ky
so_lan_hoi_fib_4 = ___                 # đếm "ĐẨY fib(4)" trong nhat_ky

print(f"fib(2) bị hỏi lại {so_lan_hoi_fib_2} lần khi tính fib(6)")
print(f"fib(4) bị hỏi lại {so_lan_hoi_fib_4} lần khi tính fib(6)")
```

```python title=solution
nhat_ky = []

def fib(n):
    nhat_ky.append(f"ĐẨY fib({n})")
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib(n - 1) + fib(n - 2)

fib(6)

so_lan_hoi_fib_2 = nhat_ky.count("ĐẨY fib(2)")
so_lan_hoi_fib_4 = nhat_ky.count("ĐẨY fib(4)")

print(f"fib(2) bị hỏi lại {so_lan_hoi_fib_2} lần khi tính fib(6)")
print(f"fib(4) bị hỏi lại {so_lan_hoi_fib_4} lần khi tính fib(6)")
```

```python title=test
assert so_lan_hoi_fib_2 == 5, f"fib(2) phải bị hỏi lại đúng 5 lần khi tính fib(6) — đang ra {so_lan_hoi_fib_2}"
assert so_lan_hoi_fib_4 == 2, f"fib(4) phải bị hỏi lại đúng 2 lần khi tính fib(6) — đang ra {so_lan_hoi_fib_4}"
assert so_lan_hoi_fib_2 > so_lan_hoi_fib_4, "fib(2) gần trường hợp cơ sở hơn fib(4), nên phải bị hỏi lại NHIỀU hơn, không ít hơn"
```

:::hints
- kind: attention
  body: Cả hai chỗ trống chỉ ĐẾM lại trên một danh sách chuỗi đã ghi sẵn — không chạy lại fib, không sửa fib. Dùng đúng phương thức .count(...) của list.
- kind: strategy
  body: 'nhat_ky.count("ĐẨY fib(2)") đếm đúng số dòng trong nhat_ky khớp CHÍNH XÁC chuỗi đó — phải viết đúng định dạng "ĐẨY fib(k)" như hàm fib tự ghi, kể cả chữ ĐẨY viết hoa và dấu ngoặc đơn quanh con số.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là nhat_ky.count("ĐẨY fib(2)") và nhat_ky.count("ĐẨY fib(4)").'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải THẬT SỰ đếm trên nhat_ky bằng .count(...) — không gõ cứng con số 5 hay 2, vì bài này đang dạy cách XÁC NHẬN một con số đếm được, không phải chép đáp án
  requireAst:
  - kind: uses-call, target: count, min: 2
  # min: 2 — đếm thật trên solution: .count(...) xuất hiện đúng 2 lần, đúng
  # bằng hai chỗ trống — không có .count nào khác có sẵn trong khung. Điền
  # True/1/0 (một con số cố định, không gọi .count) vào một hoặc cả hai chỗ
  # trống — ĐÃ THỬ THẬT: cả ba cách True/1/0 đều AN TOÀN (không có while hay
  # đệ quy nào trong chính chỗ trống — fib(6) đã chạy xong TRƯỚC khi tới chỗ
  # trống, và các chỗ trống ở đây chỉ là biểu thức gán một giá trị, không ảnh
  # hưởng gì tới việc fib(6) đã chạy) — và cả ba cho so_lan_hoi_fib_2 hoặc
  # so_lan_hoi_fib_4 sai (1 hoặc 0, không phải 5 và 2), bị test bắt độc lập.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^fib\\(2\\) bị hỏi lại 5 lần khi tính fib\\(6\\)\\nfib\\(4\\) bị hỏi lại 2 lần khi tính fib\\(6\\)\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm lần, hai lần — đúng khớp con số đếm bằng mắt trên cây. Không phải
ước lượng, là con số thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cây `fib(6)` chỉ có hai mươi lăm nút — đủ nhỏ để vẽ trọn trên giấy và
đếm bằng mắt, dù đã hơi rối. `fib(6)` cũng chỉ tính ra tổng cộng hai
mươi lăm lượt gọi.

Nếu tính không phải `fib(6)` mà là `fib(30)` — vẫn cùng một hàm,
không đổi gì — thì tổng số lượt gọi lớn cỡ nào? Còn ai vẽ nổi cây đó
ra giấy và đếm bằng mắt được nữa không?

Đếm bằng mắt tới đây là hết đường. Bài sau đổi sang một cách đếm khác
— không nhìn hình vẽ, mà để chính chương trình tự đếm lấy bằng một
con số.
::::

::::checkpoint{mastery=0.8}
::::
