---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.giai-quyet-dung-do-bang-xau
title: "Giải quyết đụng độ bằng cách xâu chuỗi"
summary: "Mỗi ô của bảng băm không giữ một giá trị nữa — nó giữ đầu của một danh sách liên kết các giá trị cùng đụng độ, ghép cụm 3 và cụm 4 lại để vá đúng lỗ hổng bài trước."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.hash-chaining]
requires: [ds.hash-collision, ds.singly-linked-list, ds.linked-node, ctrl.while, core.dict, core.function-def]
concepts: [ds.hash-chaining]
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
Bài trước để lộ một lỗi im lặng — tra sai người, không báo hiệu gì cả. Giờ
vá nó, bằng đúng công cụ cụm 3 đã dựng cho bạn.
::::

::::explain{#moi-o-la-mot-day}
Lỗi ở bài 21 nằm ở đúng một dòng: `bang[chi_so] = gia_tri` GHI ĐÈ, không
GIỮ LẠI cái đã có. Cách vá không phải đổi công thức băm — là đổi những gì
MỘT Ô của bảng được phép giữ.

Thay vì một ô giữ TRỰC TIẾP một giá trị, cho nó giữ **đầu của một danh
sách liên kết** (cụm 3, bài 13) — một chuỗi các nút, mỗi nút một giá trị
từng đụng độ vào ô đó. Đúng quy ước bài 13 đã lập: mỗi nút là
`{"gia_tri": ..., "tiep": None}`.

**Chèn** giờ không ghi đè nữa — nó THÊM một nút MỚI vào ĐẦU chuỗi hiện có
ở ô đó (đúng lối bài 15: thêm vào đầu, rẻ, không phải dời ai):

```python title=readonly
chi_so = bam(ten, co_bang)
nut_moi = {"gia_tri": {"ten": ten, ...}, "tiep": bang[chi_so]}
bang[chi_so] = nut_moi
```

Nút mới trỏ `tiep` về đúng cái đang đứng ở ô đó TRƯỚC KHI chèn — dù đó là
`None` (ô đang rỗng) hay một nút khác (đã có người đụng độ trước). Không
gì bị mất: nút cũ vẫn còn nguyên, chỉ là không còn đứng ở đầu chuỗi nữa.

**Tra cứu** giờ không chỉ đọc một ô — nó phải ĐI BỘ dọc theo chuỗi ở đúng
ô đó, so tên từng nút một, cho tới khi khớp hoặc hết chuỗi:

```python title=readonly
chi_so = bam(ten, co_bang)
nut = bang[chi_so]
while nut is not None:
    if nut["gia_tri"]["ten"] == ten:
        return nut["gia_tri"]["so_dien_thoai"]
    nut = nut["tiep"]
return None
```

Vẫn phải "băm rồi nhảy thẳng" tới đúng Ô — chỉ khác là bây giờ, MỘT KHI đã
tới ô, có thể còn phải đi bộ TIẾP, nhưng chỉ trong phạm vi những giá trị
CÙNG đụng độ ở ô đó, không phải đi bộ qua cả bảng.
::::

::::example{#vha-loi-an-quan}
Cùng kịch bản gây lỗi ở bài 21 — "An" rồi "Quân", cùng băm ra ô 5 — giờ
chèn bằng cách xâu chuỗi:

```python title=readonly
def bam(ten, co_bang):
    return sum(ten.encode("utf-8")) % co_bang

def chen(bang, ten, so_dien_thoai, co_bang):
    chi_so = bam(ten, co_bang)
    nut_moi = {"gia_tri": {"ten": ten, "so_dien_thoai": so_dien_thoai}, "tiep": bang[chi_so]}
    bang[chi_so] = nut_moi

def tra_cuu(bang, ten, co_bang):
    chi_so = bam(ten, co_bang)
    nut = bang[chi_so]
    while nut is not None:
        if nut["gia_tri"]["ten"] == ten:
            return nut["gia_tri"]["so_dien_thoai"]
        nut = nut["tiep"]
    return None

co_bang = 10
danh_ba = [None] * co_bang

chen(danh_ba, "An", "0901111111", co_bang)
chen(danh_ba, "Quân", "0907777777", co_bang)

print(danh_ba[5])
print(tra_cuu(danh_ba, "An", co_bang))
print(tra_cuu(danh_ba, "Quân", co_bang))
```

```text title=readonly
{'gia_tri': {'ten': 'Quân', 'so_dien_thoai': '0907777777'}, 'tiep': {'gia_tri': {'ten': 'An', 'so_dien_thoai': '0901111111'}, 'tiep': None}}
0901111111
0907777777
```

`danh_ba[5]` giờ không phải một dict giá trị nữa — nó là ĐẦU của một chuỗi
hai nút: Quân trước (chèn sau cùng, đứng đầu), rồi `tiep` trỏ tới nút An
(chèn trước, giờ đứng thứ hai), rồi `tiep` cuối cùng là `None`. Tra "An"
đi bộ qua nút Quân (không khớp tên), rồi tới nút An (khớp) — đúng số của
An, không còn bị Quân ghi đè nữa.
::::

::::predict{#doan-so-nut-di-qua commitOnce}
Bảng đã chèn "An" trước, rồi "Quân" sau — đúng ví dụ trên, cả hai cùng ô
5, xâu chuỗi bằng cách thêm vào ĐẦU mỗi lần chèn.

**Trước khi đếm tay**, bạn đoán: `tra_cuu(danh_ba, "An", 10)` phải XEM QUA
bao nhiêu nút (kể cả nút cuối cùng khớp tên) trước khi tìm thấy đúng nút
của An?

:::opt{correct}
2 nút — nút Quân (không khớp) rồi tới nút An (khớp).
:::

:::opt
1 nút — vì An được chèn TRƯỚC, nên nó đứng đầu chuỗi
::why
Gần đúng ở chỗ bạn nhớ đúng thứ tự chèn: An đúng là được chèn trước Quân.

Chỗ lệch là "chèn vào ĐẦU" (bài 15) có nghĩa là người chèn SAU luôn đứng
GẦN ĐẦU HƠN người chèn trước — ngược với trực giác "trước thì đứng
trước". Quân chèn sau, nên Quân mới là đầu chuỗi; An bị đẩy xuống vị trí
thứ hai.
::
:::

:::opt
0 nút — `bang[5]` biết ngay ai đang ở trong chuỗi của nó, không cần xem
::why
Gần đúng ở chỗ bạn tin bảng "biết" nhiều hơn một con số — cảm giác đó
không sai với một CẤU TRÚC DỮ LIỆU tốt.

Chỗ lệch là `bang[5]` chỉ giữ được ĐỊA CHỈ của nút ĐẦU chuỗi (mem.name-is-
reference, T3.1 bài 20) — nó không "biết" tên của bất kỳ ai trong chuỗi
đó. Muốn biết một nút tên gì, buộc phải ĐI VÀO nút đó và ĐỌC trường
`"gia_tri"]["ten"]` của nó — không có cách nào bỏ qua bước đọc này.
::
:::

:::opt
Không tìm thấy An nữa, vì Quân đã đè mất chỗ của An
::why
Gần đúng ở chỗ đây ĐÚNG LÀ chuyện đã xảy ra ở BÀI TRƯỚC (bài 21), trước
khi có xâu chuỗi — bạn nhớ đúng lỗi cũ.

Chỗ lệch là xâu chuỗi đã vá đúng lỗi đó: mỗi nút MỚI chỉ được THÊM vào
đầu chuỗi qua trường `"tiep"`, không hề GHI ĐÈ nút cũ. Nút của An vẫn còn
nguyên trong bảng, chỉ là không còn đứng ở đầu — `tra_cuu` vẫn tìm ra nó
được, chỉ phải đi qua nút Quân trước.
::
:::
::::

::::code{#chen-bang-xau-chuoi}
`tra_cuu` đã viết xong — đi bộ dọc chuỗi, so tên, trả về đúng số hoặc
`None`. Bạn hoàn thành `chen`: chỗ trống phải giữ lại chuỗi CŨ đang có ở ô
đó, không được làm nó biến mất.

```python title=starter
def bam(ten, co_bang):
    return sum(ten.encode("utf-8")) % co_bang

def chen(bang, ten, so_dien_thoai, co_bang):
    chi_so = bam(ten, co_bang)
    nut_moi = {"gia_tri": {"ten": ten, "so_dien_thoai": so_dien_thoai}, "tiep": ___}
    bang[chi_so] = nut_moi

def tra_cuu(bang, ten, co_bang):
    chi_so = bam(ten, co_bang)
    nut = bang[chi_so]
    while nut is not None:
        if nut["gia_tri"]["ten"] == ten:
            return nut["gia_tri"]["so_dien_thoai"]
        nut = nut["tiep"]
    return None

co_bang = 10
danh_ba = [None] * co_bang
chen(danh_ba, "An", "0901111111", co_bang)
chen(danh_ba, "Quân", "0907777777", co_bang)

print(tra_cuu(danh_ba, "An", co_bang))
print(tra_cuu(danh_ba, "Quân", co_bang))
print(tra_cuu(danh_ba, "Hoa", co_bang))
```

```python title=solution
def bam(ten, co_bang):
    return sum(ten.encode("utf-8")) % co_bang

def chen(bang, ten, so_dien_thoai, co_bang):
    chi_so = bam(ten, co_bang)
    nut_moi = {"gia_tri": {"ten": ten, "so_dien_thoai": so_dien_thoai}, "tiep": bang[chi_so]}
    bang[chi_so] = nut_moi

def tra_cuu(bang, ten, co_bang):
    chi_so = bam(ten, co_bang)
    nut = bang[chi_so]
    while nut is not None:
        if nut["gia_tri"]["ten"] == ten:
            return nut["gia_tri"]["so_dien_thoai"]
        nut = nut["tiep"]
    return None

co_bang = 10
danh_ba = [None] * co_bang
chen(danh_ba, "An", "0901111111", co_bang)
chen(danh_ba, "Quân", "0907777777", co_bang)

print(tra_cuu(danh_ba, "An", co_bang))
print(tra_cuu(danh_ba, "Quân", co_bang))
print(tra_cuu(danh_ba, "Hoa", co_bang))
```

```python title=test
assert danh_ba[5]["gia_tri"]["ten"] == "Quân", "nút ĐẦU chuỗi ở ô 5 phải là Quân — chèn sau cùng, đứng đầu"
assert danh_ba[5]["tiep"]["gia_tri"]["ten"] == "An", "nút THỨ HAI ở ô 5 phải là An — chèn trước, bị đẩy xuống, nhưng KHÔNG bị mất"
assert danh_ba[5]["tiep"]["tiep"] is None, "chuỗi ở ô 5 chỉ có đúng hai nút — nút thứ hai phải trỏ 'tiep' về None, kết thúc chuỗi"
assert tra_cuu(danh_ba, "An", co_bang) == "0901111111", "tra_cuu('An') phải tìm đúng số của An, dù An không còn đứng đầu chuỗi"
assert tra_cuu(danh_ba, "Quân", co_bang) == "0907777777", "tra_cuu('Quân') phải tìm đúng số của Quân"
assert tra_cuu(danh_ba, "Hoa", co_bang) is None, "'Hoa' chưa từng được chèn — ô 0 vẫn None, tra_cuu phải trả None"
```

:::hints
- kind: attention
  body: Ngay TRƯỚC khi dòng `bang[chi_so] = nut_moi` chạy, `bang[chi_so]` vẫn đang giữ CHUỖI CŨ (có thể là `None`, có thể là một nút khác). Nút mới cần GIỮ LẠI đúng thứ đó trong trường `"tiep"` của nó, trước khi bị ghi đè.
- kind: strategy
  body: "Đọc lại ví dụ phía trên: nút mới trỏ `tiep` về đúng những gì `bang[chi_so]` đang có, TRƯỚC lúc gán lại. Dùng chính biểu thức `bang[chi_so]` — đọc giá trị hiện tại của ô, không phải một giá trị cố định nào."
- kind: one-line
  body: 'Điền `bang[chi_so]` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải giữ lại giá trị HIỆN TẠI của bang[chi_so] (chuỗi cũ, trước khi ghi đè) để không làm mất nút nào đã có — không được gõ thẳng None hay một giá trị cố định khác
  requireAst:
  - kind: uses-name, target: bang, min: 3
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^0901111111\\n0907777777\\nNone\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
An và Quân cùng ô, cùng còn nguyên cả hai — không ai bị đè mất nữa. Cụm 3
và cụm 4 vừa ghép tay để vá một lỗ hổng của chính cụm 4.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Xâu chuỗi vá đúng lỗi mất dữ liệu. Nhưng hãy tưởng tượng một ô, thay vì
chỉ có 2 nút như An và Quân, có tới 50 nút cùng đụng độ vào đó — vì bảng
chỉ có vài ô mà đã nhét vào hàng trăm cái tên.

Tra một tên rơi đúng vào ô đó lúc này sẽ phải đi bộ qua BAO NHIÊU nút? Và
"băm rồi nhảy thẳng" — lời hứa ban đầu của bảng băm — còn giữ được không,
nếu mỗi ô đều dài như vậy?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
