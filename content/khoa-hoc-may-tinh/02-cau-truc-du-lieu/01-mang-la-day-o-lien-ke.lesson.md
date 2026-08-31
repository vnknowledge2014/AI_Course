---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.mang-la-day-o-lien-ke
title: "Mảng là các ô liền kề, đánh số từ 0"
summary: "Mảng không phải luật tự nhiên của bộ nhớ — nó là quyết định xếp các ô liên quan NGAY CẠNH NHAU, đánh số từ 0, và quyết định đó có cái giá của nó."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ds.array-contiguous]
requires: [mem.address, mem.byte-range, core.list, core.list-index, core.len, core.variable, core.assignment, core.fstring, core.builtin-function]
concepts: [ds.array-contiguous]
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
Một dãy tủ khoá đứng sát nhau, không hở một khe nào, đánh số từ ô đầu tiên.
Bài này gọi đúng tên cái dãy đó: một **mảng**.
::::

::::explain{#quyet-dinh-xep-canh-nhau}
T3.1 bài 19 đã dựng một hình: bộ nhớ của máy là một dãy ô đánh số, hàng tỉ
ô nối liền nhau như một con phố dài. Bài đó chỉ nói tới việc CÓ một dãy ô
như vậy. Bài này hỏi tiếp: khi bạn cần cất một LOẠT giá trị liên quan tới
nhau — tên sáu người dùng chung một dãy tủ khoá, chẳng hạn — bạn xếp chúng
vào những ô đó theo cách nào?

Có một lựa chọn tự nhiên đến mức dễ tưởng là luật bắt buộc: xếp chúng NGAY
CẠNH NHAU, ô này chạm ô kia, không hở khe trống nào ở giữa. Đó chính là
**mảng** (tiếng Anh: *array*) — một dãy ô liền kề, và theo quy ước của hầu
hết ngôn ngữ lập trình (Python trong đó), ô đầu tiên mang số **0**, không
phải số 1.

Nhưng đây KHÔNG phải luật tự nhiên của bộ nhớ. Bộ nhớ không đòi hỏi các ô
liên quan phải đứng cạnh nhau — hoàn toàn có thể rải chúng ra khắp nơi,
miễn mỗi ô biết đường tới ô kế (cụm bài sau, danh sách liên kết, làm đúng
việc đó). Xếp liền kề là một QUYẾT ĐỊNH, và như mọi quyết định thiết kế
khác, nó có cái giá của nó — cái giá ấy lộ ra dần trong các bài tới.

Một dãy tủ khoá phòng gym là hình dung rõ nhất: sáu cái tủ đứng thành một
hàng, tủ nọ chạm vách tủ kia, đánh số 0 tới 5. Muốn biết tủ số mấy đứng ở
đâu, bạn chỉ cần biết tủ số 0 đứng ở đâu — phần còn lại đếm liền một mạch.
::::

::::example{#day-tu-khoa-python}
Python cho bạn dựng một mảng như vậy chỉ bằng một dòng — đây chính là
`list` bạn đã dùng quen từ R1, chỉ là giờ nhìn nó dưới một góc mới: một dãy
ô liền kề, đánh số từ 0.

```python title=readonly
tu_khoa = ["Hà", "Minh", "Lan", "Đức", "Chi"]

print(tu_khoa[0])
print(tu_khoa[4])
print(len(tu_khoa))
```

```text title=readonly
Hà
Chi
5
```

Năm cái tên, năm ô liền kề, đánh số 0 tới 4. `tu_khoa[0]` là ô ĐẦU TIÊN
("Hà" — không phải ô thứ nhất theo cách đếm 1, 2, 3 bạn quen từ nhỏ), và
`tu_khoa[4]` là ô CUỐI CÙNG trong năm ô đó. `len(tu_khoa)` trả về 5 — tổng
số ô, không phải số thứ tự ô cuối. Hai con số ấy khác nhau đúng một đơn vị,
và chênh lệch đó sẽ còn quay lại làm khó không ít người mới.
::::

::::predict{#tu-khoa-so-5 commitOnce}
Vẫn năm cái tên ở trên, đánh số 0 tới 4. Byte gõ thêm một dòng:

```python
tu_khoa = ["Hà", "Minh", "Lan", "Đức", "Chi"]
print(tu_khoa[5])
```

**Trước khi chạy**, bạn đoán dòng này làm gì?

:::opt{correct}
Máy dừng lại, báo lỗi `IndexError` — dãy chỉ có 5 tủ, đánh số 0 tới 4; tủ
số 5 không tồn tại.
:::

:::opt
In ra `Chi` — vì đó là cái tên đứng cuối trong dãy.
::why
Gần đúng ở chỗ `"Chi"` đúng là phần tử cuối cùng trong `tu_khoa` — bạn
không nhớ nhầm nội dung.

Chỗ lệch là chỉ số của `"Chi"` là **4**, không phải 5. Năm ô đánh số 0, 1,
2, 3, 4 — số 5 nằm NGOÀI dãy, không trỏ tới bất kỳ ô nào cả.
::
:::

:::opt
In ra chuỗi rỗng `''` — vì tủ số 5 không có ai đứng, nên nó trống.
::why
Gần đúng ở cảm giác tủ số 5 "không có gì" — trực giác đó không sai.

Chỗ lệch là Python không âm thầm trả về một ô rỗng cho vị trí không tồn
tại. Nó DỪNG hẳn chương trình lại và báo lỗi ngay tại dòng đó, không đi
tiếp xuống các dòng sau.
::
:::

:::opt
In ra `None` — vì Python tự động điền `None` cho ô còn trống.
::why
Gần đúng ở việc bạn nghĩ Python "linh hoạt" tự vá những chỗ thiếu — nhiều
chỗ khác trong Python đúng là làm vậy (ví dụ `dict.get`).

Chỗ lệch là `list` không tự vá lỗ theo kiểu đó. Một mảng chỉ có ĐÚNG số ô
đã cấp — đọc ra ngoài phạm vi đó luôn gây lỗi dừng chương trình, không bao
giờ âm thầm trả `None`.
::
:::
::::

::::code{#tu-dau-toi-cuoi}
Phòng gym có một dãy tủ khoá dài hơn — sáu cái, không phải năm. Byte muốn
in ra tủ ĐẦU TIÊN và tủ CUỐI CÙNG, cùng chỉ số của từng cái.

Chỉ số của tủ đầu tiên luôn là 0. Còn chỉ số của tủ cuối cùng — đừng đếm
tay, hãy TÍNH nó từ tổng số tủ trong dãy.

```python title=starter
tu_khoa = ["Hà", "Minh", "Lan", "Đức", "Chi", "Bình"]

chi_so_dau = 0
chi_so_cuoi = ___

print(f"Tủ đầu tiên (chỉ số {chi_so_dau}): {tu_khoa[chi_so_dau]}")
print(f"Tủ cuối cùng (chỉ số {chi_so_cuoi}): {tu_khoa[chi_so_cuoi]}")
```

```python title=solution
tu_khoa = ["Hà", "Minh", "Lan", "Đức", "Chi", "Bình"]

chi_so_dau = 0
chi_so_cuoi = len(tu_khoa) - 1

print(f"Tủ đầu tiên (chỉ số {chi_so_dau}): {tu_khoa[chi_so_dau]}")
print(f"Tủ cuối cùng (chỉ số {chi_so_cuoi}): {tu_khoa[chi_so_cuoi]}")
```

```python title=test
assert tu_khoa == ["Hà", "Minh", "Lan", "Đức", "Chi", "Bình"], "đừng sửa danh sách tu_khoa"
assert chi_so_dau == 0, "chỉ số của tủ đầu tiên luôn là 0 — đừng sửa dòng này"
assert chi_so_cuoi == 5, "dãy có 6 tủ, đánh số 0 tới 5 — chỉ số tủ cuối cùng phải là 5"
assert tu_khoa[chi_so_cuoi] == "Bình", "chi_so_cuoi phải trỏ đúng tới phần tử cuối cùng trong danh sách"
```

:::hints
- kind: attention
  body: Đừng đếm tay rồi gõ thẳng con số. Bài đang hỏi bạn TÍNH chỉ số cuối cùng từ tổng số tủ — dùng công cụ đã có sẵn để đếm tổng số ô trong một dãy.
- kind: strategy
  body: "`len(tu_khoa)` cho tổng số ô — ở đây là 6. Nhưng chỉ số của ô cuối KHÔNG phải 6, vì dãy đánh số từ 0: sáu ô mang chỉ số 0, 1, 2, 3, 4, 5. Chỉ số ô cuối luôn kém tổng số ô đúng một đơn vị."
- kind: one-line
  body: "Điền `len(tu_khoa) - 1` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải TÍNH chỉ số cuối cùng bằng len(tu_khoa) - 1, không phải gõ thẳng một con số hay dùng .index() để tìm ngược lại — bài này đang dạy tính chỉ số từ TỔNG SỐ Ô, không phải dò tìm
  requireAst:
  - kind: uses-call, target: len, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Tủ đầu tiên \(chỉ số 0\): Hà\nTủ cuối cùng \(chỉ số 5\): Bình\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu ô, liền kề, đánh số từ 0 tới 5. Không đếm tay — bạn vừa TÍNH ra chỉ số
cuối cùng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa lấy `tu_khoa[0]` và `tu_khoa[5]` — hai ô nằm ở hai đầu dãy, cách xa
nhau nhất có thể. Cả hai đều ra kết quả ngay lập tức, không chờ đợi gì
khác nhau.

Nhưng máy không "nhìn thấy" cả dãy tủ khoá cùng lúc như mắt bạn. Nó chỉ
biết một con số duy nhất — chỉ số bạn đưa vào. Vậy làm sao nó tìm ra đúng
CHỖ của tủ số 5 trong bộ nhớ, mà không cần đi qua tủ số 0, 1, 2, 3, 4
trước?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
