---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.hang-doi-vong
title: "Hàng đợi vòng: đi hết thì quay lại đầu"
summary: "Né cái bẫy bài 10 bằng cách không dồn ai cả — giữ hai con trỏ đầu/cuối trên một mảng cỡ cố định, và dùng % để quay vòng chỉ số khi chạm hết mảng."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.circular-queue]
requires: [ds.queue-array-trap, core.modulo, core.dict, core.function-def, core.function-parameter, core.function-call, ctrl.for-each, core.list, core.list-index, core.augmented-assign, core.none, core.fstring]
concepts: [ds.circular-queue]
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
Có. Không dồn ai — chỉ cần nhớ đầu hàng đang ở đâu, và biết quay lại khi
hết chỗ.
::::

::::explain{#hai-con-tro-va-mot-phep-chia-du}
Cái bẫy bài 10 chỉ xảy ra vì `.pop(0)` luôn ép đầu hàng phải nằm ở đúng
chỉ số 0 — muốn vậy thì mọi người còn lại buộc phải dồn lên. Nhưng đầu
hàng không CẦN nằm ở chỉ số 0. Nó chỉ cần một chỗ để bạn NHỚ nó đang ở
đâu.

Cách né bẫy: dùng một mảng cỡ CỐ ĐỊNH (biết trước sức chứa tối đa), cùng
hai con trỏ — `dau` (chỉ số của người đứng đầu hàng) và `cuoi` (chỉ số ô
trống kế tiếp để thêm người mới). Phục vụ một người: đọc ô ở `dau`, rồi
CHỈ dịch con trỏ `dau` sang một, không đụng tới ai khác trong mảng. Thêm
một người: ghi vào ô ở `cuoi`, rồi dịch `cuoi` sang một. Không lệnh nào
trong hai việc này DỜI một phần tử đã có sẵn — chúng chỉ dịch một con số.

Vấn đề: con trỏ dịch mãi về bên phải, sớm muộn cũng CHẠM hết mảng. Đây là
lúc phép `%` (chia lấy dư) — cùng phép T3.1 bài 3 dùng để tách bit — vào
việc: thay vì để chỉ số vượt quá cỡ mảng, lấy `(chi_so + 1) % suc_chua`.
Khi `chi_so` chạm đúng `suc_chua - 1`, phép `%` kéo nó quay về `0` —
"quay vòng" lại đầu mảng, tận dụng những ô đã bị bỏ trống bởi người đã
được phục vụ từ lâu. Cấu trúc này gọi là **hàng đợi vòng** (circular
queue).

Đánh đổi: hàng đợi vòng phải biết trước SỨC CHỨA tối đa — khác với `list`
tự lớn ở bài 3. Đầy đúng sức chứa mà vẫn cố thêm người mới sẽ ghi đè lên
người CHƯA được phục vụ — một lỗi thật, không phải chuyện lý thuyết. Track
này không đi sâu vào cách chặn lỗi đó; điều quan trọng là hiểu được CƠ CHẾ
quay vòng trước đã.
::::

::::example{#phong-cho-nam-ghe}
Byte dựng một hàng đợi vòng sức chứa 5, biểu diễn bằng một `dict` giữ
mảng và hai con trỏ. Năm người vào lấp đầy, hai người rời đi, rồi hai
người mới vào — ngay vào đúng hai ô vừa trống.

```python title=readonly
suc_chua = 5
hang_doi = {"day": [None] * suc_chua, "dau": 0, "cuoi": 0}

def them_vao(hd, gia_tri):
    hd["day"][hd["cuoi"]] = gia_tri
    hd["cuoi"] = (hd["cuoi"] + 1) % suc_chua

def lay_ra(hd):
    gia_tri = hd["day"][hd["dau"]]
    hd["dau"] = (hd["dau"] + 1) % suc_chua
    return gia_tri

for ten in ["A", "B", "C", "D", "E"]:
    them_vao(hang_doi, ten)
print(hang_doi["day"], "dau =", hang_doi["dau"], "cuoi =", hang_doi["cuoi"])

lay_ra(hang_doi)
lay_ra(hang_doi)
print("sau khi A, B rời đi — dau =", hang_doi["dau"])

them_vao(hang_doi, "F")
them_vao(hang_doi, "G")
print(hang_doi["day"], "dau =", hang_doi["dau"], "cuoi =", hang_doi["cuoi"])
```

```text title=readonly
['A', 'B', 'C', 'D', 'E'] dau = 0 cuoi = 0
sau khi A, B rời đi — dau = 2
['F', 'G', 'C', 'D', 'E'] dau = 2 cuoi = 2
```

Nhìn dòng đầu tiên: `cuoi` đã quay về `0` ngay sau khi "E" được thêm —
`(4 + 1) % 5` đúng bằng `0`, mảng vừa đầy vừa "khép vòng". Nhìn dòng
cuối: "F" và "G" ghi ĐÈ lên đúng hai ô mà "A" và "B" từng chiếm — không ô
nào trong "C", "D", "E" bị đụng tới, không ai phải dồn chỗ. Mảng vẫn y
nguyên năm ô từ đầu tới cuối; chỉ có ý nghĩa của từng ô đổi theo con trỏ.
::::

::::predict{#vong-suc-chua-ba commitOnce}
Byte dựng một hàng đợi vòng sức chứa CHỈ 3, rồi: thêm "X", "Y", "Z" (đầy),
phục vụ một người, rồi thêm "W".

```python
suc_chua = 3
hang_doi = {"day": [None, None, None], "dau": 0, "cuoi": 0}

def them_vao(hd, gia_tri):
    hd["day"][hd["cuoi"]] = gia_tri
    hd["cuoi"] = (hd["cuoi"] + 1) % suc_chua

def lay_ra(hd):
    gia_tri = hd["day"][hd["dau"]]
    hd["dau"] = (hd["dau"] + 1) % suc_chua
    return gia_tri

them_vao(hang_doi, "X")
them_vao(hang_doi, "Y")
them_vao(hang_doi, "Z")
lay_ra(hang_doi)
them_vao(hang_doi, "W")

print(hang_doi["day"])
print(hang_doi["dau"])
```

**Trước khi chạy**, bạn đoán `hang_doi["day"]` và `hang_doi["dau"]` cuối
cùng là gì?

:::opt{correct}
`['W', 'Y', 'Z']`, và `dau` bằng `1`
:::

:::opt
`['X', 'Y', 'Z', 'W']`, và `dau` bằng `1`
::why
Gần đúng ở việc bạn nhớ đúng `dau` bằng `1` sau lượt `lay_ra`.

Chỗ lệch là bạn nghĩ `them_vao("W")` làm mảng DÀI RA thêm một ô. Nhưng
`day` có cỡ CỐ ĐỊNH là 3 ô ngay từ dòng khai báo — `them_vao` không bao
giờ thêm ô mới, nó chỉ GHI ĐÈ vào một ô đã có sẵn, đúng tại vị trí `cuoi`
đang trỏ tới.
::
:::

:::opt
`['W', 'Y', 'Z']`, và `dau` bằng `0`
::why
Gần đúng ở phần `day` — `['W', 'Y', 'Z']` đúng là mảng cuối cùng.

Chỗ lệch là `dau`. Lệnh `lay_ra(hang_doi)` không chỉ ĐỌC ô ở `dau`, nó
còn tự dịch `dau` sang một bước ngay sau đó — `(0 + 1) % 3 = 1`. Con trỏ
`dau` không đứng yên chỉ vì bạn chưa gọi `them_vao`.
::
:::

:::opt
`[None, 'Y', 'Z']`, và `dau` bằng `1` — "W" bị chặn lại vì ô của "X" đã
từng bị chiếm
::why
Gần đúng ở việc `dau` bằng `1` — đúng, lượt `lay_ra` dịch nó tới đó.

Chỗ lệch là hàng đợi vòng không hề "khoá" một ô chỉ vì nó từng chứa dữ
liệu. Ô ở chỉ số 0 (chỗ cũ của "X") đã được `lay_ra` giải phóng về mặt
LOGIC — con trỏ `cuoi` vẫn đang trỏ đúng ô đó, và `them_vao("W")` ghi đè
lên nó bình thường, không có gì chặn cả.
::
:::
::::

::::code{#dien-cong-thuc-quay-vong}
Byte viết công thức cho `them_vao` và `lay_ra` của hàng đợi vòng — cả hai
đều phải TỰ QUAY VỀ đầu mảng khi con trỏ chạm hết `suc_chua`. Bên dưới là
một buổi sáng bận rộn: nhiều lượt vào — ra xen kẽ, đủ để cả hai con trỏ
đều phải quay vòng ít nhất một lần.

```python title=starter
suc_chua = 5
hang_doi = {"day": [None] * suc_chua, "dau": 0, "cuoi": 0, "so_luong": 0}

def them_vao(hd, gia_tri):
    hd["day"][hd["cuoi"]] = gia_tri
    hd["cuoi"] = ___                        # dịch cuoi sang 1, quay vòng nếu chạm hết suc_chua
    hd["so_luong"] += 1

def lay_ra(hd):
    gia_tri = hd["day"][hd["dau"]]
    hd["dau"] = ___                         # dịch dau sang 1, quay vòng nếu chạm hết suc_chua
    hd["so_luong"] -= 1
    return gia_tri

for ten in ["A", "B", "C", "D", "E"]:
    them_vao(hang_doi, ten)

lay_ra(hang_doi)
lay_ra(hang_doi)
them_vao(hang_doi, "F")
them_vao(hang_doi, "G")
lay_ra(hang_doi)
lay_ra(hang_doi)
them_vao(hang_doi, "H")
lay_ra(hang_doi)
them_vao(hang_doi, "I")

print(hang_doi["day"])
print(f"dau={hang_doi['dau']}, cuoi={hang_doi['cuoi']}, so_luong={hang_doi['so_luong']}")
```

```python title=solution
suc_chua = 5
hang_doi = {"day": [None] * suc_chua, "dau": 0, "cuoi": 0, "so_luong": 0}

def them_vao(hd, gia_tri):
    hd["day"][hd["cuoi"]] = gia_tri
    hd["cuoi"] = (hd["cuoi"] + 1) % suc_chua
    hd["so_luong"] += 1

def lay_ra(hd):
    gia_tri = hd["day"][hd["dau"]]
    hd["dau"] = (hd["dau"] + 1) % suc_chua
    hd["so_luong"] -= 1
    return gia_tri

for ten in ["A", "B", "C", "D", "E"]:
    them_vao(hang_doi, ten)

lay_ra(hang_doi)
lay_ra(hang_doi)
them_vao(hang_doi, "F")
them_vao(hang_doi, "G")
lay_ra(hang_doi)
lay_ra(hang_doi)
them_vao(hang_doi, "H")
lay_ra(hang_doi)
them_vao(hang_doi, "I")

print(hang_doi["day"])
print(f"dau={hang_doi['dau']}, cuoi={hang_doi['cuoi']}, so_luong={hang_doi['so_luong']}")
```

```python title=test
assert hang_doi["day"] == ["F", "G", "H", "I", "E"], f"mảng cuối cùng phải đúng ['F', 'G', 'H', 'I', 'E'] — đang ra {hang_doi['day']}"
assert hang_doi["dau"] == 0, f"dau phải quay vòng về đúng 0 sau chuỗi thao tác này — đang ra {hang_doi['dau']}"
assert hang_doi["cuoi"] == 4, f"cuoi phải dừng ở đúng 4 — đang ra {hang_doi['cuoi']}"
assert hang_doi["so_luong"] == 4, f"so_luong phải đúng bằng số người còn trong hàng — 4 — đang ra {hang_doi['so_luong']}"
```

:::hints
- kind: attention
  body: Cả hai chỗ trống cùng một hình dạng công thức, chỉ khác con trỏ nào đang dịch — cuoi ở chỗ trống 1, dau ở chỗ trống 2. Đừng chỉ viết "+ 1" suông — thiếu phần quay vòng, chỉ số sẽ tràn ra ngoài mảng và chương trình dừng bằng lỗi.
- kind: strategy
  body: 'Công thức "quay vòng" là (chi_so + 1) % suc_chua — cộng thêm 1 như bình thường, rồi lấy dư cho suc_chua để chỉ số không bao giờ vượt quá suc_chua - 1. Chỗ trống 1 áp dụng công thức này cho hd["cuoi"], chỗ trống 2 áp dụng đúng công thức đó cho hd["dau"].'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `(hd["cuoi"] + 1) % suc_chua` và `(hd["dau"] + 1) % suc_chua`.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: cả hai chỗ trống phải thật sự dùng phép % với suc_chua để quay vòng chỉ số — cộng 1 suông (không quay vòng) sẽ làm chỉ số tràn khỏi mảng và chương trình dừng bằng lỗi ở đúng buổi sáng bận rộn này; bài đang dạy cơ chế quay vòng, không phải phép cộng đơn thuần
  requireAst:
  # Đếm thật trên solution: suc_chua được ĐỌC đúng 2 lần (một lần trong mỗi
  # công thức) — dòng "suc_chua = 5" là GÁN, không tính. Thiếu cổng này, một
  # lời giải chỉ cộng 1 cho MỘT trong hai con trỏ (ví dụ quên quay vòng dau)
  # vẫn ra đúng kết quả TRONG một chuỗi thao tác ngắn không đủ để dau chạm
  # suc_chua — đây là lý do khung phải có chuỗi vào/ra dài, xen kẽ đủ để CẢ
  # hai con trỏ đều buộc phải quay vòng ít nhất một lần; nếu thiếu độ dài đó,
  # thử bỏ % ở dau vẫn cho ra CÙNG kết quả với lời giải đúng.
  - kind: uses-operator, target: "%", min: 2
  - kind: uses-name, target: suc_chua, min: 2
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: "^\\['F', 'G', 'H', 'I', 'E'\\]\\ndau=0, cuoi=4, so_luong=4\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai con trỏ, một phép chia lấy dư, và không một ai trong hàng phải dồn
chỗ suốt cả buổi sáng bận rộn đó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại cả cụm bài vừa qua: ngăn xếp và hàng đợi — kể cả bản vòng vừa
né được cái bẫy của bài 10 — đều dựng trên một MẢNG. Mà mảng luôn kéo
theo một ràng buộc: hàng đợi vòng phải biết trước SỨC CHỨA tối đa (bài
này), còn ngăn xếp thỉnh thoảng phải xin hẳn một vùng nhớ mới và chép hết
sang khi mảng cũ đầy (bài 3).

Bạn đã có hai cấu trúc kỷ luật ra-vào khác hẳn nhau. Nhưng làm sao biết
nên dùng cái nào cho một việc cụ thể — undo trong một trình soạn thảo,
hay hàng người đang chờ máy in xử lý?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
