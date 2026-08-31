---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.ngan-xep-vao-sau-ra-truoc
title: "Ngăn xếp: vào sau, ra trước"
summary: "Ngăn xếp (LIFO) dựng bằng đúng list.append/list.pop không tham số — cả hai đều thao tác ở ĐUÔI mảng, chỗ duy nhất thêm/bớt không kéo ai phải dời chỗ."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ds.stack]
requires: [ds.dynamic-array, core.list, core.list-append, core.variable, core.assignment, core.fstring]
concepts: [ds.stack]
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
Có một chỗ như vậy — và bạn dùng nó mỗi ngày mà chưa để ý.
::::

::::explain{#duoi-day-la-cho-mien-phi}
Bài trước để lại một câu hỏi: trong tất cả các chỗ có thể chèn vào hay xoá
đi trên một mảng — đầu, giữa, cuối — có chỗ nào KHÔNG kéo theo dây chuyền
dời chỗ nào cả không?

Có. Chính là **đuôi** dãy. Thêm một phần tử ngay sau ô cuối cùng chỉ cần
ghi vào một ô đang trống — không ai đứng sau nó để phải nhường chỗ, vì
không có gì ở đó cả. Bớt phần tử cuối cùng cũng vậy: bỏ đúng nó đi, không
ai phía trước hay phía sau phải dồn lại. Đây đúng là việc `list.append(x)`
và `list.pop()` (không tham số) làm — cả hai đều CHỈ động vào đuôi. (Thỉnh
thoảng `append` vẫn phải xin vùng nhớ mới và chép hết sang, đúng bài 3 đã
nói — nhưng đó là chuyện xin CHỖ, không phải chuyện DỜI phần tử để nhường
chỗ. Hai chuyện khác nhau.)

Giới hạn bản thân vào đúng hai thao tác đó — thêm ở đuôi, bớt ở đuôi —
dựng nên một cấu trúc có tên riêng: **ngăn xếp** (stack). Luật của nó gói
trong bốn chữ: **vào sau, ra trước** — tiếng Anh gọi là LIFO (Last In,
First Out). Phần tử bạn vừa thêm gần đây nhất luôn là phần tử bạn lấy ra
đầu tiên, vì nó đang đứng ở đúng chỗ duy nhất bạn được phép lấy — đuôi
dãy.

Hình dung một chồng đĩa ở quầy rửa bát: đĩa rửa xong đặt LÊN TRÊN chồng.
Muốn lấy một đĩa sạch để dùng, bạn chỉ lấy được đĩa TRÊN CÙNG — không ai
rút một đĩa ở giữa chồng ra mà không làm đổ những đĩa phía trên nó. Đĩa
đặt lên sau cùng luôn là đĩa được lấy ra trước tiên.
::::

::::example{#chong-dia-len-xuong}
Byte rửa xong bốn cái đĩa, xếp lần lượt lên chồng — rồi lấy hai cái ra
dùng.

```python title=readonly
chong_dia = []
chong_dia.append("đĩa 1")
chong_dia.append("đĩa 2")
chong_dia.append("đĩa 3")
print(chong_dia)

dia_dung = chong_dia.pop()
print(dia_dung)
print(chong_dia)
```

```text title=readonly
['đĩa 1', 'đĩa 2', 'đĩa 3']
đĩa 3
['đĩa 1', 'đĩa 2']
```

`chong_dia.pop()` không tham số lấy đúng phần tử CUỐI CÙNG của danh sách —
"đĩa 3", cái vừa đặt lên sau cùng — chứ không phải "đĩa 1", cái đặt lên
đầu tiên. Sau khi lấy, chồng chỉ còn lại `['đĩa 1', 'đĩa 2']`: không có ô
trống nào ở giữa, không ai phải dồn chỗ, vì "đĩa 3" vốn đã đứng ở đúng
đuôi dãy.
::::

::::predict{#viec-can-lam commitOnce}
Byte quản một ngăn xếp việc cần làm — việc mới ghi luôn được xử lý trước
việc cũ, đúng luật LIFO:

```python
viec = []
viec.append("rửa bát")
viec.append("quét nhà")
viec.append("giặt đồ")

viec.pop()
viec.append("nấu cơm")

print(viec)
print(viec.pop())
```

**Trước khi chạy**, bạn đoán hai dòng in ra là gì?

:::opt{correct}
`['rửa bát', 'quét nhà', 'nấu cơm']`, rồi `nấu cơm`
:::

:::opt
`['rửa bát', 'quét nhà', 'nấu cơm']`, rồi `giặt đồ`
::why
Gần đúng ở dòng đầu — danh sách cuối cùng đúng là `['rửa bát', 'quét nhà',
'nấu cơm']`, bạn theo dõi đúng các lượt `append`.

Chỗ lệch là dòng thứ hai. "giặt đồ" đã bị `viec.pop()` lấy ra và bỏ đi từ
TRƯỚC khi "nấu cơm" được thêm vào — nó không còn nằm trong `viec` nữa để
mà lấy ra lần thứ hai. Lần `pop()` cuối cùng lấy đúng phần tử đang đứng ở
đuôi LÚC ĐÓ, và lúc đó "nấu cơm" mới là phần tử mới nhất.
::
:::

:::opt
`['rửa bát', 'quét nhà', 'giặt đồ', 'nấu cơm']`, rồi `nấu cơm`
::why
Gần đúng ở việc bạn nhớ đúng "nấu cơm" là phần tử được thêm sau cùng, và
đúng là phần tử `pop()` cuối cùng lấy ra.

Chỗ lệch là dòng đầu. Trước khi "nấu cơm" được thêm, đã có một lệnh
`viec.pop()` chạy — nó lấy "giặt đồ" ra khỏi ngăn xếp thật sự, không phải
chỉ "đọc thử". Danh sách lúc `append("nấu cơm")` chạy chỉ còn hai phần tử
(`rửa bát`, `quét nhà`), không phải ba.
::
:::

:::opt
`['rửa bát', 'quét nhà', 'nấu cơm']`, rồi `rửa bát`
::why
Gần đúng ở dòng đầu — bạn tính đúng trạng thái cuối cùng của `viec`.

Chỗ lệch là bạn đang nghĩ `pop()` lấy phần tử ĐẦU TIÊN từng được thêm vào
("rửa bát") thay vì phần tử GẦN ĐÂY NHẤT. Đó là luật của một cấu trúc
khác — bài sau sẽ gọi đúng tên nó. Ngăn xếp luôn lấy ở đuôi, nơi phần tử
mới nhất đang đứng.
::
:::
::::

::::code{#chong-dia-quan-an}
Quán ăn có một chồng đĩa sạch. Byte xếp thêm một đĩa lên trên, rồi lấy hai
đĩa liên tiếp ra dùng — luôn luôn từ TRÊN CÙNG chồng.

```python title=starter
ngan_xep = []

ngan_xep.append("đĩa A")
ngan_xep.append("đĩa B")
ngan_xep.append("đĩa C")
___                                # xếp thêm "đĩa D" lên TRÊN CÙNG chồng

dia_lay_ra_1 = ___                 # lấy đĩa đang ở TRÊN CÙNG ra dùng
dia_lay_ra_2 = ngan_xep.pop()

print(ngan_xep)
print(f"Dùng lần lượt: {dia_lay_ra_1}, rồi {dia_lay_ra_2}")
```

```python title=solution
ngan_xep = []

ngan_xep.append("đĩa A")
ngan_xep.append("đĩa B")
ngan_xep.append("đĩa C")
ngan_xep.append("đĩa D")

dia_lay_ra_1 = ngan_xep.pop()
dia_lay_ra_2 = ngan_xep.pop()

print(ngan_xep)
print(f"Dùng lần lượt: {dia_lay_ra_1}, rồi {dia_lay_ra_2}")
```

```python title=test
assert ngan_xep == ["đĩa A", "đĩa B"], f"sau hai lượt lấy, chồng chỉ còn đúng 'đĩa A' và 'đĩa B' — đang ra {ngan_xep}"
assert dia_lay_ra_1 == "đĩa D", f"lượt lấy đầu tiên phải lấy đúng đĩa TRÊN CÙNG lúc đó — 'đĩa D' — đang ra {dia_lay_ra_1!r}"
assert dia_lay_ra_2 == "đĩa C", f"lượt lấy thứ hai phải lấy đúng đĩa TRÊN CÙNG mới, sau khi 'đĩa D' đã rời đi — 'đĩa C' — đang ra {dia_lay_ra_2!r}"
```

:::hints
- kind: attention
  body: Ngăn xếp chỉ có đúng hai việc được phép làm ở đuôi dãy — thêm vào và lấy ra. Không dùng .insert(), không dùng chỉ số, không dùng pop(0).
- kind: strategy
  body: 'Chỗ trống thứ nhất: xếp "đĩa D" lên trên bằng đúng công cụ ví dụ vừa dùng — .append(...). Chỗ trống thứ hai: lấy đĩa trên cùng hiện tại bằng .pop() không tham số, đúng lúc "đĩa D" vừa mới lên trên nên nó sẽ là đĩa được lấy ra trước.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `ngan_xep.append("đĩa D")` và `ngan_xep.pop()`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải thật sự dùng .append (thêm vào đuôi) và .pop() không tham số (lấy ra ở đuôi) — bài này đang dạy đúng kỷ luật ngăn xếp chỉ động vào một đầu duy nhất, không phải .insert(), không phải pop(0), không phải đọc trực tiếp qua chỉ số
  requireAst:
  # Khung đã có sẵn 3 lượt append cố định (đĩa A, B, C) và không lượt pop nào.
  # Lời giải đúng cộng thêm đúng 1 append (đĩa D) và 2 pop — đếm thật bằng ast
  # trên khối solution xác nhận: append() xuất hiện 4 lần, pop() xuất hiện 2
  # lần. Thiếu cổng này, ngan_xep.insert(0, "đĩa D") ở chỗ trống 1 (đẩy đĩa D
  # xuống ĐÁY thay vì lên TRÊN) vẫn còn đó 3 append cũ nên không lộ ra nếu chỉ
  # đếm append một cách lỏng lẻo — min:4 mới chặn đúng nó, vì mất một lượt
  # append thì chỉ còn 3.
  - kind: uses-call, target: append, min: 4
  - kind: uses-call, target: pop, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\['đĩa A', 'đĩa B'\\]\\nDùng lần lượt: đĩa D, rồi đĩa C\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đĩa D vừa lên trên là đĩa D ra trước — không đĩa nào khác trong chồng phải
xê dịch một milimét.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ngăn xếp rất giỏi một việc: lấy lại đúng thứ GẦN ĐÂY NHẤT. Nhưng bản thân
cơ chế "vào sau, ra trước" chưa tự nó có ích — nó cần một bài toán thật để
áp dụng vào.

Nghĩ về một chuỗi ký tự như `"a(b[c]d)e"`. Làm sao bạn kiểm tra được mọi
dấu ngoặc mở đều có đúng một dấu ngoặc đóng CÙNG LOẠI, và chúng lồng vào
nhau đúng thứ tự — không phải chỉ đếm xem số dấu mở có bằng số dấu đóng
hay không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
