---
id: khoa-hoc-may-tinh.may-chay-the-nao.xep-nam-tang-theo-toc-do
title: "Xếp năm tầng bộ nhớ theo tốc độ"
summary: "Bài chốt cụm: thanh ghi → bộ nhớ đệm gần CPU → bộ nhớ đệm xa hơn → RAM → đĩa, mỗi tầng lớn hơn và chậm hơn tầng trước một bậc. Người học tự xếp thứ tự và giải thích TẠI SAO — không học thuộc bảng."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [may.memory-hierarchy]
requires: [may.disk-slowest]
concepts: [may.memory-hierarchy]
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
Năm tầng, một quy luật lặp lại đúng năm lần. Xếp chúng cạnh nhau, và quy
luật đó tự hiện ra.
::::

::::explain{#nam-tang-mot-quy-luat}
Bài 17 để hé lộ một điều: bộ nhớ đệm không phải MỘT tầng đơn — CPU hiện
đại thường có ít nhất hai mức bộ nhớ đệm xếp chồng, mức gần CPU nhất
(nhỏ nhất, nhanh nhất) và mức xa hơn (lớn hơn một chút, chậm hơn một
chút, nhưng vẫn nhanh hơn RAM rất nhiều). Tách rõ hai mức đó ra, cả track
này đã đi qua đúng NĂM tầng:

| tầng | học ở bài | kích cỡ | tốc độ |
|---|---|---|---|
| thanh ghi | 16 | vài chục chỗ chứa | nhanh nhất |
| bộ nhớ đệm gần CPU | 17 | hàng nghìn tới hàng chục nghìn byte | rất nhanh |
| bộ nhớ đệm xa CPU hơn | 17 | lớn hơn mức trên | nhanh, kém mức trên |
| RAM | 18 | hàng tỉ byte | chậm hơn hẳn (bài 18) |
| đĩa | 20 | vượt xa RAM | chậm nhất (bài 20) |

Một quy luật lặp lại ở MỖI bước chuyển từ tầng này sang tầng kế: **tầng
sau luôn LỚN HƠN, nhưng luôn CHẬM HƠN**. Không có tầng nào vừa lớn vừa
nhanh nhất — đó không phải sự trùng hợp, mà là một sự ĐÁNH ĐỔI vật lý:
một chỗ chứa càng lớn thì càng khó đặt gần CPU (không đủ chỗ trên con
chip nhỏ bé), càng khó đặt gần CPU thì càng phải đi xa hơn để chạm tới
— đúng nguyên do thanh ghi (bài 16) buộc phải nhỏ xíu, còn đĩa (bài 20)
mới có thể chứa được hàng terabyte.

Nhớ đúng bảng trên không phải mục tiêu của bài này. Mục tiêu là hiểu
được QUY LUẬT — lớn hơn thì chậm hơn — để khi gặp một tầng bộ nhớ MỚI,
chưa từng học, vẫn đoán được nó nằm ở đâu trong thứ tự đó.
::::

::::example{#xep-theo-khoang-cach}
Gán cho mỗi tầng một con số ước lượng "khoảng cách" tới CPU — càng lớn
càng xa, càng chậm — rồi để Python tự xếp thứ tự thay vì học thuộc lòng:

```python title=readonly
tang_bo_nho = {
    "RAM": 100,
    "thanh ghi": 1,
    "đĩa": 100000,
    "bộ nhớ đệm xa CPU (L2, L3)": 10,
    "bộ nhớ đệm gần CPU (L1)": 4,
}

thu_tu = sorted(tang_bo_nho, key=tang_bo_nho.get)
for ten in thu_tu:
    print(f"{ten}: {tang_bo_nho[ten]}")
```

```text title=readonly
thanh ghi: 1
bộ nhớ đệm gần CPU (L1): 4
bộ nhớ đệm xa CPU (L2, L3): 10
RAM: 100
đĩa: 100000
```

Không cần nhớ tên tầng nào đứng trước tầng nào — chỉ cần nhớ QUY LUẬT
(khoảng cách tới CPU càng lớn thì càng chậm), và dữ liệu tự xếp đúng thứ
tự. Chú ý độ lớn của khoảng cách: từ thanh ghi tới bộ nhớ đệm gần CPU
chỉ gấp vài lần, nhưng từ RAM tới đĩa gấp tới hàng nghìn lần — bước nhảy
cuối cùng, ra khỏi chip hẳn, luôn là bước nhảy LỚN nhất.
::::

::::predict{#quy-luat-la-gi commitOnce}
Giả sử có một tầng bộ nhớ MỚI, chưa từng nhắc trong track này — gọi nó
là "tầng X" — và bạn được biết đúng một điều: tầng X có thể chứa được
NHIỀU dữ liệu hơn RAM rất nhiều lần.

Dựa theo đúng quy luật vừa học, bạn có thể đoán trước điều gì về tốc độ
của tầng X, dù chưa từng đo?

:::opt{correct}
Tầng X gần như chắc chắn CHẬM HƠN RAM — kích cỡ lớn hơn luôn đi kèm tốc
độ chậm hơn, đúng quy luật lặp lại ở mọi tầng đã học
:::

:::opt
Không thể đoán được gì cả — kích cỡ và tốc độ là hai chuyện hoàn toàn
độc lập với nhau
::why
Gần đúng ở sự thận trọng — kích cỡ và tốc độ đúng là hai đại lượng KHÁC
NHAU, đo bằng hai đơn vị khác nhau.

Chỗ lệch: dù là hai đại lượng khác nhau, bài này vừa cho thấy chúng
KHÔNG độc lập — mọi tầng đã đo (thanh ghi, hai mức bộ nhớ đệm, RAM, đĩa)
đều tuân đúng một quy luật: lớn hơn luôn đi kèm chậm hơn. Quy luật lặp
lại đều đặn năm lần không còn là ngẫu nhiên nữa.
::
:::

:::opt
Tầng X nhanh hơn RAM — vì công nghệ mới luôn cải tiến theo hướng vừa
nhanh vừa lớn hơn
::why
Gần đúng ở niềm tin "công nghệ mới thì tốt hơn" — công nghệ THẬT SỰ có
tiến bộ theo thời gian.

Chỗ lệch: tiến bộ công nghệ làm MỌI tầng nhanh hơn phiên bản CŨ của
CHÍNH nó theo thời gian — không xoá được sự đánh đổi GIỮA CÁC tầng tại
cùng một thời điểm. Một tầng lớn hơn RAM rất nhiều, ở BẤT KỲ thời điểm
nào, vẫn phải đặt xa CPU hơn RAM — quy luật vật lý đó không đổi dù công
nghệ có tiến bộ tới đâu.
::
:::

:::opt
Tầng X có tốc độ ngang bộ nhớ đệm — vì kích cỡ lớn không liên quan gì
tới việc nó gần hay xa CPU
::why
Gần đúng ở việc bạn nhắc đúng khái niệm "gần hay xa CPU" — đúng chính
điều quyết định tốc độ (bài 16-20).

Chỗ lệch: kích cỡ và khoảng cách tới CPU KHÔNG độc lập với nhau, như bài
này vừa chỉ ra — một chỗ chứa càng lớn càng khó đặt gần CPU (không đủ
chỗ trên con chip nhỏ), nên càng lớn thì càng có khả năng phải đặt XA
hơn. Không thể có một tầng vừa lớn hơn RAM nhiều lần vừa nhanh ngang bộ
nhớ đệm.
::
:::
::::

::::code{#tu-xep-nam-tang}
Cho sẵn năm tầng cùng "khoảng cách" ước lượng của chúng, CHƯA sắp xếp.
Viết ra danh sách năm tên tầng, xếp từ NHANH NHẤT tới CHẬM NHẤT.

```python title=starter
tang_bo_nho = {
    "RAM": 100,
    "thanh ghi": 1,
    "đĩa": 100000,
    "bộ nhớ đệm xa CPU (L2, L3)": 10,
    "bộ nhớ đệm gần CPU (L1)": 4,
}

thu_tu_dung = ___          # danh sách 5 tên, xếp từ tầng NHANH NHẤT tới CHẬM NHẤT

for ten in thu_tu_dung:
    print(f"{ten}: {tang_bo_nho[ten]}")
```

```python title=solution
tang_bo_nho = {
    "RAM": 100,
    "thanh ghi": 1,
    "đĩa": 100000,
    "bộ nhớ đệm xa CPU (L2, L3)": 10,
    "bộ nhớ đệm gần CPU (L1)": 4,
}

thu_tu_dung = ["thanh ghi", "bộ nhớ đệm gần CPU (L1)", "bộ nhớ đệm xa CPU (L2, L3)", "RAM", "đĩa"]

for ten in thu_tu_dung:
    print(f"{ten}: {tang_bo_nho[ten]}")
```

```python title=test
assert len(thu_tu_dung) == 5, f"phải có đúng năm tầng, không thiếu không thừa — đang có {len(thu_tu_dung)}"
assert set(thu_tu_dung) == set(tang_bo_nho.keys()), "danh sách phải chứa đúng năm cái tên có trong tang_bo_nho, không đổi tên, không thêm bớt"
for i in range(len(thu_tu_dung) - 1):
    a, b = thu_tu_dung[i], thu_tu_dung[i + 1]
    assert tang_bo_nho[a] < tang_bo_nho[b], f"'{a}' phải đứng TRƯỚC '{b}' — khoảng cách của '{a}' ({tang_bo_nho[a]}) phải nhỏ hơn khoảng cách của '{b}' ({tang_bo_nho[b]})"
```

:::hints
- kind: attention
  body: Không cần dùng hàm sắp xếp có sẵn — viết thẳng ra một list năm phần tử, đúng thứ tự bạn suy luận được từ bài học.
- kind: strategy
  body: 'Khoảng cách càng NHỎ thì càng nhanh, xếp trước. Nhìn giá trị trong tang_bo_nho: thanh ghi (1) nhỏ nhất, rồi bộ nhớ đệm gần CPU (4), bộ nhớ đệm xa CPU (10), RAM (100), đĩa (100000) lớn nhất — xếp đúng thứ tự tăng dần đó.'
- kind: one-line
  body: 'Chỗ trống là: ["thanh ghi", "bộ nhớ đệm gần CPU (L1)", "bộ nhớ đệm xa CPU (L2, L3)", "RAM", "đĩa"]'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: "thanh ghi: 1"
- tier: output
  expect: "đĩa: 100000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm tầng, đúng thứ tự — và cùng một quy luật đứng sau cả năm: lớn hơn
thì chậm hơn, không có ngoại lệ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cả cụm bài này xếp RAM như MỘT tầng, một khối duy nhất. Nhưng một chương
trình đang chạy dùng RAM đó để giữ nhiều thứ khác nhau cùng lúc: chính
đoạn mã lệnh của nó (bài 1), các khung của những lời gọi hàm đang dở
(bài 12), và dữ liệu vừa được cấp phát lúc chạy.

RAM có phải một khối TRƠN, dùng tuỳ ý ở đâu cũng được — hay nó cũng được
CHIA VÙNG, mỗi vùng một việc riêng, giống hệt cách track này vừa chia
năm tầng theo tốc độ?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
