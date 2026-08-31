---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.mang-vs-lien-ket-danh-doi-gi
title: "Mảng và danh sách liên kết: đổi cái gì lấy cái gì"
summary: "Bài chốt cụm, đối chiếu trực diện: mảng tới thẳng ô bất kỳ nhưng chèn/xoá giữa phải dời; danh sách liên kết chèn/xoá đầu rẻ nhưng tới ô bất kỳ phải đi bộ. Không cấu trúc nào thắng tuyệt đối."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ds.array-vs-linked-tradeoff]
requires: [ds.linked-insert-middle, ds.array-index-address]
concepts: [ds.array-vs-linked-tradeoff]
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
Không có cấu trúc nào thắng tuyệt đối. Bài này xếp cả hai cạnh nhau để
bạn thấy rõ cái giá của từng bên.
::::

::::explain{#hai-cai-gia-doi-ngươc}
Sáu bài vừa qua dựng ra hai cấu trúc trả lời cùng một câu hỏi — "giữ một
dãy giá trị có thứ tự" — theo hai cách trái ngược nhau:

| việc cần làm | mảng | danh sách liên kết |
|---|---|---|
| tới phần tử thứ k bất kỳ | tính thẳng địa chỉ (bài 2) | phải đi bộ từ đầu, qua từng nút (bài 16) |
| chèn/xoá ở ĐẦU | dời cả dãy (bài 4, 5) | chỉ đổi một trường `tiep` (bài 15) |
| chèn/xoá ở GIỮA | dời mọi phần tử phía sau | phải đi bộ tới đó, rồi đổi hai trường |
| các ô nằm ở đâu | liền kề bắt buộc | không cần liền kề |

Không có dòng nào trong bảng này mà cả hai cột cùng thắng. Mảng trả giá
đắt đúng ở chỗ danh sách liên kết trả giá rẻ, và ngược lại. Đây không
phải một cấu trúc "tốt hơn" cấu trúc kia — đây là hai lựa chọn đánh đổi
khác nhau cho cùng một bài toán.

Vì vậy câu hỏi đúng không phải "cấu trúc nào nhanh hơn" — mà là "công việc
BẠN đang làm đòi hỏi thứ gì nhiều hơn: tới thẳng một vị trí bất kỳ, hay
chèn/xoá ở hai đầu?"
::::

::::example{#dem-so-buoc-phai-di}
Byte viết một hàm lấy phần tử thứ k của danh sách liên kết, và đếm luôn
SỐ BƯỚC phải đi để tới đó — con số này chính là cái giá bài 16 đã nói
"phải đi bộ", nhìn thấy được bằng mắt.

```python title=readonly
def lay_gia_tri_thu_k(dau, k):
    hien = dau
    so_buoc = 0
    while so_buoc < k:
        hien = hien["tiep"]
        so_buoc += 1
    return hien["gia_tri"], so_buoc

nut4 = {"gia_tri": "Mùa thu cho em", "tiep": None}
nut3 = {"gia_tri": "Diễm xưa", "tiep": nut4}
nut2 = {"gia_tri": "Hạ trắng", "tiep": nut3}
nut1 = {"gia_tri": "Cát bụi", "tiep": nut2}

for k in [0, 1, 2, 3]:
    print(k, "->", lay_gia_tri_thu_k(nut1, k))
```

```text title=readonly
0 -> ('Cát bụi', 0)
1 -> ('Hạ trắng', 1)
2 -> ('Diễm xưa', 2)
3 -> ('Mùa thu cho em', 3)
```

So bước luôn bằng đúng k — không có cách nào tới `k=3` mà đi ít hơn 3
bước, vì không có địa chỉ nào tính thẳng được như bài 2 làm với mảng. Một
danh sách phát 500 bài, lấy bài thứ 499, là 499 bước đi bộ, không hơn
không kém.
::::

::::predict{#chon-cau-truc-cho-viec commitOnce}
Byte đang xây một trình phát nhạc thật. Người dùng liên tục bấm "phát bài
này NGAY BÂY GIỜ" — thao tác này LUÔN đưa một bài lên vị trí ĐẦU danh
sách đang phát, hàng trăm lần một buổi nghe nhạc. Ứng dụng hầu như không
bao giờ cần lấy "bài thứ k bất kỳ" — chỉ cần biết bài đang phát (đầu
danh sách) và bài tiếp theo.

Byte nên xây danh sách đang phát bằng cấu trúc nào?

:::opt{correct}
Danh sách liên kết — vì thao tác chính (đưa lên đầu) rẻ trên cấu trúc này
:::

:::opt
Mảng — vì mảng nhanh hơn danh sách liên kết trong MỌI việc
::why
Gần đúng ở chỗ mảng thật sự nhanh hơn cho một việc CỤ THỂ — tới thẳng
phần tử thứ k bằng chỉ số (bài 2).

Chỗ lệch là "nhanh hơn trong mọi việc" không đúng — bảng đối chiếu ở trên
cho thấy mảng trả giá RẤT đắt đúng ở việc Byte cần làm nhiều nhất: đưa
một phần tử lên đầu. Mỗi lần bấm "phát ngay", mảng phải dời TOÀN BỘ các
phần tử còn lại (bài 4) — một cấu trúc không "nhanh hơn" chung chung, nó
nhanh hơn tuỳ việc.
::
:::

:::opt
Mảng — vì các ô liền kề nên máy luôn đọc nhanh hơn dù làm việc gì
::why
Gần đúng ở chỗ tính liền kề của mảng đúng là có lợi thế thật (bài 1),
đặc biệt khi ĐỌC TUẦN TỰ nhiều phần tử liên tiếp.

Chỗ lệch là lợi thế "liền kề" đó không giúp gì cho việc CHÈN vào đầu — để
giữ tính liền kề, mảng buộc phải dời chỗ mọi phần tử mỗi lần chèn ở đầu,
bất kể các ô có nằm sát nhau hay không. Liền kề là lợi thế cho việc TỚI
một ô, không phải lợi thế cho việc CHÈN.
::
:::

:::opt
Không cấu trúc nào quan trọng, vì máy tính bây giờ đủ nhanh cho cả hai
::why
Gần đúng ở chỗ với một danh sách vài chục bài hát, sự khác biệt có thể
không ai nhận ra được bằng mắt thường — điều đó có thật.

Chỗ lệch là bài học này không nói về việc "nhanh cỡ nào", mà về việc PHẢI
LÀM BAO NHIÊU VIỆC cho mỗi thao tác — dời một phần tử hay dời cả nghìn
phần tử là hai việc khác hẳn nhau về LƯỢNG CÔNG VIỆC, dù máy có nhanh tới
đâu cũng phải làm đúng từng ấy việc.
::
:::
::::

::::code{#dem-buoc-lay-phan-tu}
Viết hàm `lay_gia_tri_thu_k(dau, k)`: đi từ `dau`, bước đúng `k` lần theo
`"tiep"`, rồi trả về một tuple `(gia_tri, so_buoc)` — giá trị tại vị trí
`k`, và số bước THẬT SỰ đã đi để tới đó.

```python title=starter
def lay_gia_tri_thu_k(dau, k):
    hien = dau
    so_buoc = 0
    while so_buoc < k:
        ___
        so_buoc += 1
    return hien["gia_tri"], so_buoc

nut4 = {"gia_tri": "Mùa thu cho em", "tiep": None}
nut3 = {"gia_tri": "Diễm xưa", "tiep": nut4}
nut2 = {"gia_tri": "Hạ trắng", "tiep": nut3}
nut1 = {"gia_tri": "Cát bụi", "tiep": nut2}

print(lay_gia_tri_thu_k(nut1, 0))
print(lay_gia_tri_thu_k(nut1, 2))
print(lay_gia_tri_thu_k(nut1, 3))
```

```python title=solution
def lay_gia_tri_thu_k(dau, k):
    hien = dau
    so_buoc = 0
    while so_buoc < k:
        hien = hien["tiep"]
        so_buoc += 1
    return hien["gia_tri"], so_buoc

nut4 = {"gia_tri": "Mùa thu cho em", "tiep": None}
nut3 = {"gia_tri": "Diễm xưa", "tiep": nut4}
nut2 = {"gia_tri": "Hạ trắng", "tiep": nut3}
nut1 = {"gia_tri": "Cát bụi", "tiep": nut2}

print(lay_gia_tri_thu_k(nut1, 0))
print(lay_gia_tri_thu_k(nut1, 2))
print(lay_gia_tri_thu_k(nut1, 3))
```

```python title=test
assert lay_gia_tri_thu_k(nut1, 0) == ("Cát bụi", 0), "k=0 nghĩa là không đi bước nào cả"
assert lay_gia_tri_thu_k(nut1, 1) == ("Hạ trắng", 1), "k=1 phải đi đúng 1 bước"
assert lay_gia_tri_thu_k(nut1, 2) == ("Diễm xưa", 2), "k=2 phải đi đúng 2 bước"
assert lay_gia_tri_thu_k(nut1, 3) == ("Mùa thu cho em", 3), "k=3 phải đi đúng 3 bước — không được báo sai số bước"
```

:::hints
- kind: attention
  body: Dòng cập nhật so_buoc đã có sẵn trong khung. Chỗ trống chỉ còn thiếu một việc — bước hien sang nút kế — để giá trị trả về cũng đúng, không riêng số bước.
- kind: strategy
  body: 'Bước sang nút kế nghĩa là đi theo trường "tiep" của nút hiện tại: `hien = hien["tiep"]`. Thiếu dòng này thì so_buoc vẫn đếm đúng (dòng dưới đã lo phần đó), nhưng hien đứng yên tại chỗ suốt vòng lặp — giá trị trả về sẽ luôn là nút dau, dù k là bao nhiêu.'
- kind: one-line
  body: 'Điền `hien = hien["tiep"]` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^\('Cát bụi', 0\)\n\('Diễm xưa', 2\)\n\('Mùa thu cho em', 3\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Số bước bằng đúng k — không có cách nào rút ngắn, vì không có địa chỉ nào
tính thẳng được ở đây.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Danh sách liên kết một chiều chỉ đi được MỘT hướng — từ đầu tới cuối, qua
trường `"tiep"`. Muốn biết bài hát ĐỨNG NGAY TRƯỚC bài đang phát, không
có cách nào khác ngoài đi lại từ đầu và đếm.

Nếu mỗi nút giữ thêm MỘT trường nữa — trỏ ngược lại nút đứng trước nó —
việc đó có còn khó như vậy không? Cái giá phải trả để có thêm khả năng đó
là gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
