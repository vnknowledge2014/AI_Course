---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.dict-python-chinh-la-bang-bam
title: "dict của Python chính là bảng băm"
summary: "R1 dạy cách dùng dict; đây là lý do d[khoa] nhanh gần như tức thời dù d có một triệu mục — và lý do list không được phép làm khoá dict hay phần tử set: nó sửa được, mà số băm phải cố định suốt đời."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.dict-is-hash-table]
requires: [ds.hash-load-factor, mem.mutability, core.dict, core.set, err.type-error, err.try-except, core.function-def]
concepts: [ds.dict-is-hash-table]
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
Bảng bạn vừa tự tay dựng suốt năm bài qua — giờ thú nhận: nó chính là
`dict` của Python, đúng nghĩa đen.
::::

::::explain{#dict-chinh-la-bang-bam}
R1.T1.4 dạy bạn CÁCH DÙNG `dict`: `d[khoa] = gia_tri`, đọc `d[khoa]`,
kiểm `khoa in d`. Bạn dùng nó hàng trăm lần mà chưa ai nói VÌ SAO
`d[khoa]` tra ra kết quả gần như tức thời, dù `d` có một triệu mục.

Giờ bạn đã biết câu trả lời — vì bạn vừa TỰ DỰNG nó: CPython cài `dict`
đúng theo ba việc track này vừa làm bằng tay:

- Một mảng ẩn bên trong (bài 20), cùng một hàm băm THẬT (`hash()`, phức
  tạp hơn công thức đơn giản `bam()` bạn tự viết ở bài 19, nhưng cùng một
  ý tưởng: một chuỗi luôn băm ra đúng một số).
- Một cách xử lý đụng độ khi có (bài 21–22) — CPython dùng một kỹ thuật
  khác tên gọi riêng (không phải xâu chuỗi bằng danh sách liên kết y hệt
  bài 22), nhưng giải đúng bài toán giống hệt: đụng độ không được phép
  làm mất dữ liệu.
- Tự lớn lên khi hệ số tải vượt ngưỡng (bài 23) — đúng lý do vì sao chèn
  liên tục vào một `dict` đang đầy đôi khi chậm hơn hẳn bình thường một
  nhịp, giống hệt mảng động ở bài 3.

`set` của Python cũng vậy — nó là một `dict` mà mỗi mục chỉ có KHOÁ, không
có GIÁ TRỊ đi kèm. Cùng một bảng băm, khác mỗi việc có cất giá trị đi cùng
khoá hay không.

Còn một bí ẩn track này nợ bạn từ R1: **vì sao `list` không dùng được làm
khoá `dict` hay phần tử `set`?** Câu trả lời nằm ở đúng chỗ track này vừa
dựng. Muốn tra một khoá, bảng phải băm nó RỒI NHẢY THẲNG tới đúng ô — số
băm đó phải giữ NGUYÊN suốt đời khoá đó còn nằm trong bảng, nếu không, lần
tra sau sẽ nhảy nhầm ô. Nhưng `list` là kiểu SỬA ĐƯỢC tại chỗ
(`mem.mutability`, T3.1 bài 23) — `l.append(...)` đổi nội dung của nó mà
không dựng vật mới. Nếu `list` được phép làm khoá, và ai đó sửa nó SAU khi
đã cất vào bảng, số băm của nó phải đổi theo nội dung mới — nhưng bảng thì
không hề hay biết để dời nó sang ô khác. Khoá đó coi như MẤT, kẹt vĩnh
viễn ở ô cũ, không bao giờ tra lại được nữa dù nó vẫn còn trong bảng.

Python chặn nguy cơ đó ngay từ đầu, không đợi tới lúc hỏng: `list` (và mọi
kiểu SỬA ĐƯỢC tại chỗ) đơn giản KHÔNG ĐƯỢC PHÉP băm — gọi `hash()` trên nó
là lỗi ngay lập tức. `str`, `int`, `tuple` (chỉ chứa các phần tử không sửa
được) đều KHÔNG sửa được tại chỗ, nên số băm của chúng an toàn để giữ cố
định suốt đời — chúng làm khoá được.
::::

::::example{#hash-that-va-list-bi-chan}
```python title=readonly
print(hash("An"))
print(hash("An") == hash("An"))
```

```text title=readonly
-1395048929
True
```

`hash()` là hàm băm THẬT của Python — số nó trả ra khác hẳn công thức đơn
giản `bam()` bạn tự viết (nó còn cho ra số ÂM, điều `bam()` không bao giờ
làm vì có `% co_bang`), nhưng luật cốt lõi giống hệt bài 19: gọi lại trên
đúng chuỗi đó, luôn ra đúng một số.

Giờ thử băm một `list`:

```python title=readonly
try:
    hash([1, 2, 3])
except TypeError as loi:
    print("Lỗi:", loi)

toa_do = {}
try:
    toa_do[[10, 20]] = "Ngã tư"
except TypeError as loi:
    print("Lỗi khi làm khoá dict:", loi)
```

```text title=readonly
Lỗi: unhashable type: 'list'
Lỗi khi làm khoá dict: unhashable type: 'list'
```

Cả hai đều chặn NGAY LẬP TỨC, không đợi tới lúc list bị sửa mới hỏng.
`tuple` thì khác — nó không sửa được tại chỗ, nên hash được, làm khoá
được:

```python title=readonly
diem_den = {}
diem_den[(10, 20)] = "Ngã tư"
print(diem_den[(10, 20)])
```

```text title=readonly
Ngã tư
```
::::

::::predict{#doan-loi-list-lam-khoa commitOnce}
Byte định dùng một DANH SÁCH toạ độ `[10, 20]` làm khoá cho một dict lưu
tên địa điểm:

```python
dia_diem = {}
dia_diem[[10, 20]] = "Ngã tư"
```

**Trước khi chạy**, bạn đoán chuyện gì xảy ra?

:::opt{correct}
Máy dừng lại NGAY LẬP TỨC ở dòng gán, báo `TypeError: unhashable type:
'list'` — không có gì được lưu vào `dia_diem` cả.
:::

:::opt
Lưu thành công bình thường, vì `list` cũng là một kiểu dữ liệu hợp lệ
trong Python
::why
Gần đúng ở chỗ `list` ĐÚNG là một kiểu hợp lệ — bạn dùng nó làm GIÁ TRỊ,
làm phần tử của một list khác, làm biến, hoàn toàn bình thường ở khắp nơi.

Chỗ lệch là làm KHOÁ dict thì khác. Muốn làm khoá, một giá trị phải băm
được — mà `list` SỬA ĐƯỢC tại chỗ (`mem.mutability`), nên Python cấm nó
băm, và cấm nó làm khoá, dù nó hợp lệ ở mọi vai trò khác.
::
:::

:::opt
Lưu thành công, nhưng tra lại `dia_diem[[10, 20]]` sau đó luôn ra
`KeyError` vì hai `list` khác nhau không so sánh bằng nhau được
::why
Gần đúng ở chỗ bạn cảm nhận đúng: CÓ một vấn đề thật liên quan tới việc
so sánh và tra cứu lại sau này với `list` làm khoá.

Chỗ lệch là Python không để tình huống đó có cơ hội xảy ra — nó không đợi
tới lúc TRA LẠI mới phát hiện vấn đề. Dòng GÁN đầu tiên
(`dia_diem[[10, 20]] = ...`) đã bị chặn ngay, chưa có gì được lưu để mà
tra lại cả.
::
:::

:::opt
Lưu thành công, nhưng nếu sau đó ai sửa `[10, 20]` thành `[10, 21]` thì
khoá trong `dia_diem` cũng tự đổi theo, gây lẫn lộn
::why
Gần đúng ở chỗ bạn vừa gọi tên ĐÚNG nỗi sợ thật sự khiến Python cấm
`list` làm khoá — nếu nó được phép, sửa khoá SAU khi cất sẽ làm số băm
lệch khỏi ô đã cất, khoá coi như mất.

Chỗ lệch là Python không đợi tới lúc kịch bản nguy hiểm đó xảy ra rồi mới
xử lý. Nó chặn NGAY TỪ LÚC GÁN — trước khi có bất kỳ cơ hội nào để ai đó
sửa `list` sau này. Nỗi sợ bạn nêu ra là đúng, chỉ là Python đã ngăn nó từ
gốc, không để nó có đường xảy ra.
::
:::
::::

::::code{#kiem-tra-hash-duoc}
Trước khi lỡ tay dùng một giá trị không hash được làm khoá, bạn viết một
hàm kiểm tra trước: `co_the_lam_khoa` nhận một giá trị, trả về `True` nếu
nó băm được (dùng làm khoá dict / phần tử set an toàn), `False` nếu
không.

```python title=starter
def co_the_lam_khoa(gia_tri):
    try:
        ___
        return True
    except TypeError:
        return False

print(co_the_lam_khoa("An"))
print(co_the_lam_khoa((10, 20)))
print(co_the_lam_khoa([10, 20]))
print(co_the_lam_khoa(1000))
print(co_the_lam_khoa({"a": 1}))
```

```python title=solution
def co_the_lam_khoa(gia_tri):
    try:
        hash(gia_tri)
        return True
    except TypeError:
        return False

print(co_the_lam_khoa("An"))
print(co_the_lam_khoa((10, 20)))
print(co_the_lam_khoa([10, 20]))
print(co_the_lam_khoa(1000))
print(co_the_lam_khoa({"a": 1}))
```

```python title=test
assert co_the_lam_khoa("An") is True, "chuỗi luôn hash được"
assert co_the_lam_khoa((10, 20)) is True, "tuple (chỉ chứa số) hash được"
assert co_the_lam_khoa([10, 20]) is False, "list SỬA ĐƯỢC tại chỗ — không hash được"
assert co_the_lam_khoa(1000) is True, "số nguyên luôn hash được"
assert co_the_lam_khoa({"a": 1}) is False, "dict cũng SỬA ĐƯỢC tại chỗ — không hash được, giống list"
```

:::hints
- kind: attention
  body: Bài dùng đúng công cụ track này vừa dựng để "thử xem có băm được không" — không phải kiểm tra KIỂU DỮ LIỆU (str, list, ...) bằng `type` hay `isinstance`, vì cách đó phải liệt kê hết mọi kiểu, còn thiếu là sai (như `dict` trong bộ kiểm tra này).
- kind: strategy
  body: "Gọi thẳng hàm băm THẬT của Python lên `gia_tri`, bên trong khối `try`. Nếu `gia_tri` không băm được, Python tự ném ra đúng lỗi mà khối `except TypeError` đang chờ bắt — không cần bạn tự kiểm tra kiểu dữ liệu là gì."
- kind: one-line
  body: 'Điền `hash(gia_tri)` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ gọi hash(gia_tri) để Python tự phát hiện giá trị có băm được không — không được đoán bằng type(), isinstance(), hay liệt kê thủ công các kiểu hash được
  requireAst:
  - kind: uses-call, target: hash, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^True\\nTrue\\nFalse\\nTrue\\nFalse\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`str`, `tuple`, `int` — băm được. `list`, `dict` — không. Đúng một luật:
sửa được tại chỗ thì không được làm khoá.
::::

::::reflect{#nghi-lai}
Một câu hỏi khép lại cả sáu bài vừa qua.

Suốt từ mảng (cụm 1) tới ngăn xếp, hàng đợi (cụm 2), danh sách liên kết
(cụm 3), rồi tới bảng băm (cụm này) — mọi cấu trúc bạn dựng đều là một
DÃY: phần tử này nối tiếp phần tử kia, một đường thẳng có đầu có cuối.
Ngay cả bảng băm, bên trong mỗi ô, cũng chỉ là một dãy — một danh sách
liên kết.

Nhưng một thư mục máy tính không phải một dãy. Thư mục `Ảnh` có thể chứa
thư mục con `2024`, thư mục con đó lại chứa thêm thư mục con nữa — không
còn là "cái này nối cái kia" theo một đường thẳng nữa.

Nếu một NÚT (bài 13 đã định nghĩa: `{"gia_tri": ..., "tiep": None}`) được
phép có NHIỀU HƠN MỘT `tiep` cùng lúc — không phải đúng một, mà nhiều
nhánh — bạn nghĩ cấu trúc đó trông sẽ ra sao?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
