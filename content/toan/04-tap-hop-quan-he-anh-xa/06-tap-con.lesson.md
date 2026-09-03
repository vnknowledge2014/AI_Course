---
id: toan.tap-hop-quan-he-anh-xa.tap-con
title: Tập con
summary: "`⊆` — A là tập con của B khi MỌI phần tử của A đều có mặt trong B. `∅ ⊆` MỌI tập hợp — không phần tử nào để phản chứng (chân lý rỗng, T2.3), và `all()` trên một chuỗi rỗng tự động xác nhận điều đó."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.subset]
requires: [math.set-empty, logic.for-all, ctrl.for-each]
concepts: [math.tap-con, math.chan-ly-rong]
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
Luống 5 là `∅`. Luống 6 chỉ có "bí đỏ". Tập nào "nhỏ hơn" tập kia — và
"nhỏ hơn" nghĩa là gì khi tập hợp không phải một con số?
::::

::::explain{#tap-con-la-gi}
"Nhỏ hơn" cho tập hợp KHÔNG so số lượng phần tử — nó hỏi một câu khác
hẳn: **`⊆`** (tập con) — A là tập con của B khi **MỌI** phần tử của A
đều có mặt trong B. Nói bằng `all()` (T2.3): `all(x ∈ B với mọi x ∈ A)`.

```python title=readonly
rau_la_xanh = {"xà lách", "cải bó xôi"}
luong_2 = {"xà lách", "cải bó xôi", "cà rốt"}

print(all(x in luong_2 for x in rau_la_xanh))
```

```text title=readonly
True
```

`rau_la_xanh ⊆ luống_2`: cả hai phần tử của `rau_la_xanh` (`xà lách`,
`cải bó xôi`) đều có mặt trong `luống_2`. `luống_2` có THÊM `cà rốt` —
không sao, `⊆` chỉ đòi hỏi A không CHỨA GÌ ngoài B, không đòi hai tập
phải giống hệt nhau.
::::

::::example{#dao-lai-khong-con-dung}
Đảo chiều câu hỏi — `⊆` KHÔNG đối xứng, đúng vai trò của nó y hệt `⊆`
đọc một chiều trong lời văn:

```python title=readonly
rau_la_xanh = {"xà lách", "cải bó xôi"}
luong_2 = {"xà lách", "cải bó xôi", "cà rốt"}

print(all(x in rau_la_xanh for x in luong_2))
```

```text title=readonly
False
```

`luống_2 ⊆ rau_la_xanh`? SAI — `luống_2` có `cà rốt`, mà `rau_la_xanh`
không có. `rau_la_xanh ⊆ luống_2` đúng (bài trên), NHƯNG `luống_2 ⊆
rau_la_xanh` sai — hai câu KHÁC nhau, đảo A và B không giữ nguyên kết
quả.
::::

::::predict{#doan-tap-rong-la-tap-con-cua-moi-thu commitOnce}
Byte hỏi tập rỗng có phải tập con của luống 2 không:

```python
luong_2 = {"xà lách", "cải bó xôi", "cà rốt"}

print(all(x in luong_2 for x in set()))
print(all(x in set() for x in set()))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`True`, rồi `True`
:::

:::opt
`False`, rồi `False` — vì `set()` KHÔNG CÓ phần tử nào để `all()` kiểm
tra, và một phép kiểm tra không có gì để kiểm thì PHẢI thất bại (không
có bằng chứng nào để nói "đúng")
::why
Gần đúng ở việc bạn nhận RA `set()` không có phần tử nào để `all()`
LẶP qua — một quan sát đúng về DỮ LIỆU đầu vào rỗng.

Chỗ lệch: đây chính xác là "chân lý rỗng" đã học Ở T2.3 (bài 11, `khi-
ve-truoc-khong-xay-ra`) — `all()` trên MỘT chuỗi RỖNG luôn trả `True`,
vì "MỌI phần tử của tập rỗng thoả điều kiện X" không có phần tử NÀO để
LÀM nó SAI (không có phản ví dụ). `∅ ⊆ B` đúng cho MỌI `B`, kể cả `B`
CŨNG là `∅` — bạn không thể chỉ ra được MỘT phần tử của `∅` mà KHÔNG
nằm trong `B`, vì `∅` không hề CÓ phần tử nào.
::
:::

:::opt
Máy báo lỗi biên dịch — `all(x in luong_2 for x in set())` gọi
generator expression TRÊN một `set()` RỖNG, và Python cấm lặp qua một
chỗ chứa không có phần tử nào để duyệt
::why
Gần đúng ở việc bạn để ý `set()` KHÔNG có phần tử — một quan sát đúng
về NỘI DUNG.

Chỗ lệch: lặp qua một chỗ chứa RỖNG hoàn toàn hợp lệ trong Python (đã
quen từ T1.4: vòng `for` chạy 0 lượt, không lỗi gì) — `all()` gọi trên
một generator KHÔNG sinh ra phần tử nào chỉ đơn giản trả về `True` NGAY
(giá trị mặc định của `all` khi rỗng). Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_la_tap_con}
Viết `la_tap_con(a, b)` — trả về `True` nếu `a ⊆ b`.

```python title=starter
def la_tap_con(a, b):
    return ___


print(la_tap_con({"xà lách", "cải bó xôi"}, {"xà lách", "cải bó xôi", "cà rốt"}))
```

```python title=solution
def la_tap_con(a, b):
    return all(x in b for x in a)


print(la_tap_con({"xà lách", "cải bó xôi"}, {"xà lách", "cải bó xôi", "cà rốt"}))
```

```python title=test
assert la_tap_con({"xà lách", "cải bó xôi", "cà rốt"}, {"xà lách", "cải bó xôi"}) is False, "luong_2 khong phai tap con cua rau_la_xanh"
assert la_tap_con(set(), {"a", "b"}) is True, "tap rong la tap con cua moi tap hop"
assert la_tap_con(set(), set()) is True, "tap rong la tap con cua chinh no"
assert la_tap_con({"a", "b"}, {"a", "b"}) is True, "mot tap hop la tap con cua chinh no"
assert la_tap_con({"a", "c"}, {"a", "b"}) is False, "c khong co trong {a, b}"
```

:::hints
- kind: attention
  body: "Dung all() voi generator expression: kiem MOI phan tu x cua a co nam trong b khong."
- kind: strategy
  body: "all(x in b for x in a)"
- kind: one-line
  body: "___ = all(x in b for x in a)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung all(...) voi generator expression kiem TUNG phan tu cua a co trong b khong -- dung == de so sanh hai tap hop se khong dung y nghia "tap con"
  requireAst:
  - kind: uses-call, target: all, min: 1
  - kind: uses-operator, target: in, min: 1
  - kind: uses-name, target: a, min: 1
  - kind: uses-name, target: b, min: 1
  forbidAst:
  - kind: uses-operator, target: <=
  - kind: uses-operator, target: '=='
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tập rỗng là tập con của mọi tập hợp — chân lý rỗng, không phải trường
hợp đặc biệt. Bài sau: khi nào hai tập hợp thật sự BẰNG nhau?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

"Luống rau lá xanh" ⊆ "luống 2". Đảo lại — luống 2 có phải tập con của
"rau lá xanh" không? Nếu KHÔNG cả hai chiều, hai tập ấy quan hệ gì?
::::

::::checkpoint{mastery=0.8}
::::
