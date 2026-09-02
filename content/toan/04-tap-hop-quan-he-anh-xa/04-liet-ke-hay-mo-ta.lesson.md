---
id: toan.tap-hop-quan-he-anh-xa.liet-ke-hay-mo-ta
title: Liệt kê hay mô tả
summary: "Hai cách dựng một tập hợp: liệt kê từng phần tử, hoặc MÔ TẢ một điều kiện — ký hiệu `{x | P(x)}`, viết bằng Python qua set comprehension `{... for ... if ...}` (đúng cú pháp comprehension đã học T1.4, chỉ đổi ngoặc vuông sang ngoặc nhọn)."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.set-builder]
requires: [math.set-not-element, core.list-comprehension, core.dict, ctrl.for-each]
concepts: [math.mo-ta-tap-hop, math.dieu-kien]
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
Byte có bảy luống. "Tập hợp những luống có trồng cà chua" — liệt kê tay
được, nhưng nếu vườn có bảy TRĂM luống thì sao?
::::

::::explain{#hai-cach-dung}
Một tập hợp dựng được theo HAI cách. **Liệt kê**: viết thẳng từng phần
tử ra, như bạn đã làm ba bài trước. **Mô tả**: nêu một ĐIỀU KIỆN mà phần
tử phải thoả, ký hiệu `{x | P(x)}` (đọc: "tập hợp các `x` sao cho `P(x)`
đúng"). Cùng một tập hợp, hai cách nói — liệt kê phải BIẾT TRƯỚC từng
phần tử, mô tả chỉ cần biết ĐIỀU KIỆN.

Python viết mô tả bằng cú pháp comprehension đã học ở T1.4 (`[x for x in
... if ...]` cho list) — đổi ngoặc vuông sang ngoặc nhọn là ra một tập
hợp:

```python title=readonly
vuon = {
    "luong_1": {"cà chua", "xà lách", "cà rốt"},
    "luong_2": {"xà lách", "cải bó xôi"},
    "luong_3": {"cà rốt", "khoai lang"},
}

liet_ke = {"luong_1"}
mo_ta = {ten for ten in vuon if "cà chua" in vuon[ten]}

print(liet_ke == mo_ta)
```

```text title=readonly
True
```

`mo_ta` KHÔNG hề gõ tên `"luong_1"` ra — nó nói "tập hợp mọi tên luống
`ten` trong `vuon` SAO CHO `"cà chua" ∈ vuon[ten]`", và máy tự đi TÌM
những tên thoả điều kiện đó. Kết quả trùng khít với liệt kê tay.
::::

::::example{#khong-can-biet-truoc-so-luong}
Mô tả không cần biết TRƯỚC sẽ ra mấy phần tử — máy tự đếm khi lọc:

```python title=readonly
vuon = {
    "luong_1": {"cà chua", "xà lách", "cà rốt"},
    "luong_2": {"xà lách", "cải bó xôi"},
    "luong_3": {"cà rốt", "khoai lang"},
}

mo_ta_2 = {ten for ten in vuon if "cà rốt" in vuon[ten]}
print(sorted(mo_ta_2))
print(len(mo_ta_2))
```

```text title=readonly
['luong_1', 'luong_3']
2
```

Bạn KHÔNG cần biết trước có mấy luống trồng cà rốt — điều kiện `"cà rốt"
∈ vuon[ten]` tự lọc ra đúng những tên thoả, dù vườn ba luống hay bảy
trăm luống, cú pháp không đổi một ký tự.
::::

::::predict{#doan-mo-ta-rong commitOnce}
Không luống nào trong vườn (nhỏ) này trồng bí đỏ:

```python
vuon = {
    "luong_1": {"cà chua", "xà lách"},
    "luong_2": {"xà lách", "cải bó xôi"},
}

mo_ta = {ten for ten in vuon if "bí đỏ" in vuon[ten]}
print(mo_ta == set())
print(len(mo_ta))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`True`, rồi `0`
:::

:::opt
Máy dừng lại, báo lỗi — vì cụm mô tả `{ten for ten in vuon if "bí đỏ" in
vuon[ten]}` không TÌM THẤY tên nào thoả điều kiện, nên nó không có gì để
trả về và phải báo lỗi thay vì im lặng trả một tập rỗng
::why
Gần đúng ở việc bạn nhận RA không tên luống nào thoả điều kiện — một
quan sát đúng về DỮ LIỆU (không luống nào trồng bí đỏ).

Chỗ lệch: một comprehension KHÔNG lọc trúng gì KHÔNG PHẢI một tình
huống lỗi — nó là một tình huống HOÀN TOÀN BÌNH THƯỜNG, y hệt vòng `for`
chạy đủ số lượt mà không dòng nào thoả `if` (đã quen từ T1.4). Kết quả
đơn giản là một tập hợp KHÔNG phần tử — tập rỗng — và `mo_ta == set()`
là `True`, `len(mo_ta)` là `0`. Không có dòng nào nổ ra cả.
::
:::

:::opt
`False`, rồi `2` — vì cụm mô tả LUÔN đi qua đủ hai luống trong vườn để
kiểm tra điều kiện, và khi KHÔNG có luống nào bị loại ra, nó GIỮ LẠI cả
hai tên đã duyệt qua thay vì bỏ hết
::why
Gần đúng ở việc bạn nhớ ĐÚNG rằng comprehension phải DUYỆT QUA cả hai
tên trong `vuon` để kiểm tra từng cái — một quan sát đúng về CÁCH nó
hoạt động BÊN TRONG.

Chỗ lệch: "duyệt qua để kiểm tra" và "giữ lại" là HAI việc khác nhau.
`if "bí đỏ" in vuon[ten]` là `False` cho CẢ HAI tên (không luống nào có
bí đỏ) — comprehension chỉ GIỮ những tên mà điều kiện `if` cho `True`,
và ở đây KHÔNG tên nào đạt. Duyệt qua hết mà không giữ gì cả cho ra
đúng tập rỗng, không phải "giữ hết vì không loại được cái nào".
::
:::
::::

::::code{#viet_tap_hop_mo_ta}
Viết `tap_hop_mo_ta(vuon, rau)` — trả về TẬP HỢP (không phải danh sách)
tên những luống có trồng `rau`, dựng bằng set comprehension (mô tả), KHÔNG
liệt kê tay.

```python title=starter
def tap_hop_mo_ta(vuon, rau):
    return ___


vuon = {
    "luong_1": {"cà chua", "xà lách", "cà rốt"},
    "luong_2": {"xà lách", "cải bó xôi"},
    "luong_3": {"cà rốt", "khoai lang"},
}

print(sorted(tap_hop_mo_ta(vuon, "cà chua")))
```

```python title=solution
def tap_hop_mo_ta(vuon, rau):
    return {ten for ten in vuon if rau in vuon[ten]}


vuon = {
    "luong_1": {"cà chua", "xà lách", "cà rốt"},
    "luong_2": {"xà lách", "cải bó xôi"},
    "luong_3": {"cà rốt", "khoai lang"},
}

print(sorted(tap_hop_mo_ta(vuon, "cà chua")))
```

```python title=test
assert tap_hop_mo_ta(vuon, "xà lách") == {"luong_1", "luong_2"}, "xa lach co o luong_1 va luong_2"
assert tap_hop_mo_ta(vuon, "cà rốt") == {"luong_1", "luong_3"}, "ca rot co o luong_1 va luong_3"
assert tap_hop_mo_ta(vuon, "bí đỏ") == set(), "khong luong nao trong bi do -- tap rong"
assert tap_hop_mo_ta({}, "cà chua") == set(), "vuon rong -- tap rong"
assert isinstance(tap_hop_mo_ta(vuon, "cà chua"), set), "phai tra ve mot set, khong phai list hay tuple"
```

:::hints
- kind: attention
  body: "Dung set comprehension: {ten for ten in vuon if ...}. Dieu kien if phai kiem rau co trong vuon[ten] khong."
- kind: strategy
  body: "{ten for ten in vuon if rau in vuon[ten]}"
- kind: one-line
  body: "___ = {ten for ten in vuon if rau in vuon[ten]}"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung set comprehension (dau ngoac nhon {}) de MO TA tap hop, khong phai liet ke tay hay dung list comprehension (dau ngoac vuong)
  requireAst:
  - kind: comprehension, min: 1
  - kind: uses-operator, target: in, min: 1
  - kind: uses-name, target: rau, min: 1
  - kind: uses-name, target: vuon, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\['luong_1'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mô tả không cần biết trước số lượng — máy tự lọc. Bài sau: một tập hợp
KHÔNG có phần tử nào — đó có phải lỗi không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Luống 5 chưa gieo gì — tập của nó là tập không phần tử. Luống 6 trồng
đúng "bí đỏ". Tập nào "nhỏ hơn" tập kia — và "nhỏ hơn" nghĩa là gì cho
tập hợp, khi tập hợp không phải một CON SỐ?
::::

::::checkpoint{mastery=0.8}
::::
