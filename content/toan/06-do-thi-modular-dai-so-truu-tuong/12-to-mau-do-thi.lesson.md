---
id: toan.do-thi-modular-dai-so-truu-tuong.to-mau-do-thi
title: Tô màu đồ thị
summary: "Tô màu đồ thị — gán MỘT màu cho MỖI đỉnh sao cho HAI đỉnh KỀ nhau (có cạnh nối) KHÔNG cùng màu; số màu tô χ(G) — số màu ÍT NHẤT cần dùng; tưới hai luống kề nhau cùng lúc thì áp lực nước yếu, cần xếp lịch khác giờ = tô màu khác."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.graph-coloring]
requires: [math.vertex-degree, math.combination-select-k]
concepts: [math.to-mau-do-thi, math.chi-so-mau]
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
Bốn luống, MỌI cặp ĐỀU kề nhau. Cần ÍT NHẤT bao nhiêu "giờ tưới"
khác nhau để KHÔNG hai luống kề nào tưới CÙNG giờ?
::::

::::explain{#to-mau-la-gi}
**Tô màu đồ thị** — gán MỘT màu (Ở đây LÀ một "giờ tưới") cho MỖI
đỉnh SAO cho HAI đỉnh KỀ nhau (có cạnh nối) KHÔNG cùng màu; **số màu
tô `χ(G)`** — số màu ÍT NHẤT cần dùng để LÀM được điều đó:

```python title=readonly
def la_to_mau_hop_le(mau, E):
    return all(mau[a] != mau[b] for (a, b) in E)


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E_day_du = {(a, b) for a in luong for b in luong if a != b}
mau_bon = {"luong_1": "do", "luong_2": "vang", "luong_3": "xanh_la", "luong_4": "xanh_duong"}

print(la_to_mau_hop_le(mau_bon, E_day_du))
```

```text title=readonly
True
```

Bốn luống, MỌI cặp ĐỀU kề nhau (đồ thị ĐẦY ĐỦ, `E_day_du` nối HẾT
mọi cặp) — bốn màu KHÁC nhau, MỖI luống một GIỜ tưới riêng, KHÔNG
cặp kề nào trùng.
::::

::::example{#ba-mau-khong-du}
Chỉ DÙNG ba màu (MỘT cặp buộc phải TRÙNG) — không hợp lệ:

```python title=readonly
def la_to_mau_hop_le(mau, E):
    return all(mau[a] != mau[b] for (a, b) in E)


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E_day_du = {(a, b) for a in luong for b in luong if a != b}
mau_ba = {"luong_1": "do", "luong_2": "vang", "luong_3": "xanh_la", "luong_4": "do"}

print(la_to_mau_hop_le(mau_ba, E_day_du))
```

```text title=readonly
False
```

`luong_1` VÀ `luong_4` cùng "đỏ" — VÀ vì đồ thị ĐẦY ĐỦ, HAI luống ĐÓ
CHẮC CHẮN kề nhau (MỌI cặp Ở đây ĐỀU kề) — VI PHẠM NGAY. Với BỐN
luống kề NHAU HẾT, χ(G) = 4: KHÔNG cách nào TÔ hợp lệ VỚI chỉ BA
màu, DÙ chọn cặp NÀO để trùng.
::::

::::predict{#doan-doi-cap-trung-mau commitOnce}
Byte thử LẠI với ba màu, NHƯNG lần NÀY cho `luong_2` VÀ `luong_3`
(một cặp KHÁC) cùng màu, thay VÌ `luong_1` VÀ `luong_4`:

```python
def la_to_mau_hop_le(mau, E):
    return all(mau[a] != mau[b] for (a, b) in E)

luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E_day_du = {(a, b) for a in luong for b in luong if a != b}
mau_khac = {"luong_1": "do", "luong_2": "vang", "luong_3": "vang", "luong_4": "xanh_la"}
print(la_to_mau_hop_le(mau_khac, E_day_du))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì lần NÀY LÀ một CẶP trùng màu KHÁC (`luong_2`, `luong_3`)
so VỚI lần TRƯỚC (`luong_1`, `luong_4`), NÊN kết quả CŨNG phải khác
::why
Gần đúng ở việc bạn để ý ĐÚNG đây LÀ một cặp trùng màu KHÁC — một
quan sát chính xác VỀ dữ liệu.

Chỗ lệch: trên đồ thị ĐẦY ĐỦ, MỌI cặp đỉnh ĐỀU kề nhau — KHÔNG quan
TRỌNG bạn chọn CẶP nào để trùng màu, hễ CÓ hai đỉnh CÙNG màu LÀ VI
PHẠM NGAY (vì cặp ĐÓ chắc chắn kề). Đổi CẶP trùng KHÔNG cứu được gì
— với BỐN luống kề nhau hết, BA màu LUÔN KHÔNG đủ, bất kể xếp kiểu
nào.
::
:::

:::opt
Máy báo lỗi khi chạy — `mau_khac` dùng `"vang"` cho CẢ `luong_2` LẪN
`luong_3`, mà một `dict` Python KHÔNG cho phép HAI khoá khác nhau
trỏ TỚI CÙNG một giá trị chuỗi
::why
Gần đúng ở việc bạn để ý ĐÚNG `luong_2` VÀ `luong_3` cùng trỏ tới
`"vang"` — một quan sát VỀ dữ liệu.

Chỗ lệch: `dict` HOÀN TOÀN cho phép nhiều khoá CÙNG một giá trị (chỉ
KHOÁ mới cần DUY NHẤT — CHÍNH đây LÀ lý do bài toán tô màu THÚ VỊ:
"giá trị" ĐƯỢC PHÉP trùng, chỉ KHÔNG được trùng GIỮA hai đỉnh KỀ
nhau). Biên dịch sạch, chạy sạch — LỖI nằm Ở LOGIC tô màu, không
phải cú pháp.
::
:::
::::

::::code{#viet_la_to_mau_hop_le}
Viết `la_to_mau_hop_le(mau, E)` — kiểm CÁCH tô `mau` (dict đỉnh→màu)
có hợp lệ trên đồ thị `(V, E)` hay không.

```python title=starter
def la_to_mau_hop_le(mau, E):
    return ___


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E_day_du = {(a, b) for a in luong for b in luong if a != b}
mau_bon = {"luong_1": "do", "luong_2": "vang", "luong_3": "xanh_la", "luong_4": "xanh_duong"}

print(la_to_mau_hop_le(mau_bon, E_day_du))
```

```python title=solution
def la_to_mau_hop_le(mau, E):
    return all(mau[a] != mau[b] for (a, b) in E)


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E_day_du = {(a, b) for a in luong for b in luong if a != b}
mau_bon = {"luong_1": "do", "luong_2": "vang", "luong_3": "xanh_la", "luong_4": "xanh_duong"}

print(la_to_mau_hop_le(mau_bon, E_day_du))
```

```python title=test
assert la_to_mau_hop_le({}, set()) is True, "khong dinh, khong canh -- hop le rong"
luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E_day_du = {(a, b) for a in luong for b in luong if a != b}
mau_bon = {"luong_1": "do", "luong_2": "vang", "luong_3": "xanh_la", "luong_4": "xanh_duong"}
assert la_to_mau_hop_le(mau_bon, E_day_du) is True, "bon mau khac nhau tren K4 -- hop le"
mau_ba = {"luong_1": "do", "luong_2": "vang", "luong_3": "xanh_la", "luong_4": "do"}
assert la_to_mau_hop_le(mau_ba, E_day_du) is False, "chi ba mau tren K4 -- luon co cap trung, vi pham"
mau_khac = {"luong_1": "do", "luong_2": "vang", "luong_3": "vang", "luong_4": "xanh_la"}
assert la_to_mau_hop_le(mau_khac, E_day_du) is False, "doi cap trung khac -- van vi pham tren K4"
```

:::hints
- kind: attention
  body: "Voi moi canh (a, b) trong E, mau[a] phai KHAC mau[b]."
- kind: strategy
  body: "all(mau[a] != mau[b] for (a, b) in E)"
- kind: one-line
  body: "___ = all(mau[a] != mau[b] for (a, b) in E)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai kiem MOI canh (a,b) co mau[a] khac mau[b] khong, dung all va !=
  requireAst:
  - kind: uses-call, target: all, min: 1
  - kind: uses-operator, target: '!=', min: 1
  - kind: uses-name, target: mau, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Xong χ(G) — số màu ÍT NHẤT cho một đồ thị. Đổi hẳn chủ đề: lịch tưới
của Byte LẶP LẠI mỗi BA ngày — Hai, Tư, Bảy, RỒI quay VỀ Hai.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Lịch tưới LẶP mỗi BA ngày: Hai, Tư, Bảy, Hai, Tư, Bảy,... Ngày thứ
`10` (đếm từ `1`) LÀ ngày NÀO trong chu kỳ? Có cách NÀO tính KHÔNG
cần đếm TAY từng ngày?
::::

::::checkpoint{mastery=0.8}
::::
