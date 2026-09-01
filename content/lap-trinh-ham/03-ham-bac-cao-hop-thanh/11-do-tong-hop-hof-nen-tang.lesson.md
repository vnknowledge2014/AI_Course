---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.do-tong-hop-hof-nen-tang
title: "Đo tổng hợp: ba HOF nền tảng"
summary: "Một chuỗi xử lý ba bước dùng CẢ BA — filter() lọc, map() biến đổi, reduce() gộp — trên cùng một tập dữ liệu. Không khái niệm mới."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [fp.review-hof]
requires: [fp.reduce-from-scratch]
concepts: [fp.review-hof]
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
`filter`, `map`, `reduce` — ba công cụ riêng lẻ. Hôm nay ghép cả ba vào
MỘT chuỗi xử lý.
::::

::::explain{#ghep-ba-cong-cu}
Ba bước, ba công cụ, một luồng dữ liệu chảy qua:

```python
from functools import reduce

xs = [1, 2, 3, 4, 5, 6]

so_chan = filter(lambda x: x % 2 == 0, xs)      # LỌC: giữ số chẵn
binh_phuong = map(lambda x: x * x, so_chan)      # BIẾN ĐỔI: bình phương
tong = reduce(lambda acc, x: acc + x, binh_phuong, 0)  # GỘP: cộng dồn

print(tong)
```

```text
56
```

`2² + 4² + 6² = 4 + 16 + 36 = 56`. Ba bước, không dòng nào sửa `xs`
gốc, không dòng nào cần biết TRƯỚC dữ liệu trông ra sao — mỗi bước chỉ
biết "tôi nhận một dãy, tôi trả ra một dãy hoặc một giá trị". `filter`
không biết `map` sẽ làm gì với kết quả của nó; `map` không biết `reduce`
sẽ gộp thế nào. Ba mảnh độc lập, ghép lại vẫn đúng.
::::

::::example{#thu-tu-anh-huong-ket-qua}
THỨ TỰ lọc/biến đổi có ảnh hưởng — không phải lúc nào cũng hoán đổi
được tự do:

```python title=readonly
xs = [3, 6, 9]

loc_truoc = list(map(lambda x: x * 2, filter(lambda x: x > 5, xs)))
doi_truoc = list(filter(lambda x: x > 5, map(lambda x: x * 2, xs)))

print(loc_truoc)
print(doi_truoc)
```

```text title=readonly
[12, 18]
[6, 12, 18]
```

`loc_truoc`: LỌC `x > 5` trên GIÁ TRỊ GỐC trước (`6`, `9` qua được),
rồi NHÂN ĐÔI (`12`, `18`). `doi_truoc`: NHÂN ĐÔI HẾT trước (`6`, `12`,
`18`), rồi LỌC `> 5` trên GIÁ TRỊ ĐÃ NHÂN ĐÔI — lúc này cả `6` (từ `3*2`)
cũng qua được điều kiện, vì `6 > 5` đúng NGAY TRÊN GIÁ TRỊ MỚI. Hai kết
quả KHÁC NHAU thật sự — thứ tự ghép không phải chi tiết tuỳ ý.
::::

::::predict{#doan-doi-thu-tu commitOnce}
```python
diem = [4, 7, 9, 3]

cach_1 = list(filter(lambda d: d >= 5, map(lambda d: d + 2, diem)))
cach_2 = list(map(lambda d: d + 2, filter(lambda d: d >= 5, diem)))

print(cach_1)
print(cach_2)
```

Hai dòng cuối in ra gì?

:::opt{correct}
`[6, 9, 11, 5]` rồi `[9, 11]`
:::

:::opt
`[6, 9, 11, 5]` rồi `[6, 9, 11, 5]` — vì `map` và `filter` HOÁN ĐỔI được
tự do, thứ tự nào cũng ra cùng kết quả
::why
Gần đúng ở việc bạn tính đúng `cach_1` (`+2` hết rồi lọc `>=5` trên giá
trị MỚI: `6,9,11,5` — cả bốn đều `>=5` sau khi cộng).

Chỗ lệch: `cach_2` LỌC TRƯỚC trên giá trị GỐC (`4,7,9,3` — chỉ `7` và
`9` thoả `>=5`), rồi mới `+2` (`9`, `11`) — chỉ HAI phần tử, không phải
bốn. Ví dụ ở trên (`loc_truoc`/`doi_truoc`) đã cho thấy đúng hiện tượng
này — thứ tự đổi kết quả thật sự, không phải trùng hợp.
::
:::

:::opt
Cả hai đều báo lỗi — không hoán đổi thứ tự `filter`/`map` được, phải giữ
đúng một trật tự cố định
::why
Gần đúng ở việc bạn nghi ngờ đúng: thứ tự CÓ ảnh hưởng thật (bạn đúng ở
điểm đó).

Chỗ lệch: cả hai thứ tự đều CHẠY ĐƯỢC bình thường, không lỗi gì — chỉ là
chúng cho ra HAI KẾT QUẢ KHÁC NHAU, không phải một trong hai bị cấm.
Python không ép bạn theo một trật tự cố định nào; bạn phải TỰ CHỌN đúng
trật tự cho đúng Ý mình muốn.
::
:::
::::

::::code{#tong_binh_phuong_so_chan}
Viết `tong_binh_phuong_so_chan(xs)` — LỌC số chẵn, BÌNH PHƯƠNG từng số,
rồi CỘNG DỒN — dùng cả `filter()`, `map()`, `reduce()`.

```python title=starter
def tong_binh_phuong_so_chan(xs):
    ___

print(tong_binh_phuong_so_chan([1, 2, 3, 4, 5, 6]))
```

```python title=solution
from functools import reduce

def tong_binh_phuong_so_chan(xs):
    so_chan = filter(lambda x: x % 2 == 0, xs)
    binh_phuong = map(lambda x: x * x, so_chan)
    return reduce(lambda acc, x: acc + x, binh_phuong, 0)

print(tong_binh_phuong_so_chan([1, 2, 3, 4, 5, 6]))
```

```python title=test
assert tong_binh_phuong_so_chan([1, 2, 3, 4, 5, 6]) == 56, "2²+4²+6² = 4+16+36 = 56"
assert tong_binh_phuong_so_chan([1, 3, 5]) == 0, "không số chẵn nào thì tổng phải là 0"
assert tong_binh_phuong_so_chan([]) == 0, "danh sách rỗng phải ra 0"
assert tong_binh_phuong_so_chan([2]) == 4, "một số chẵn duy nhất: bình phương của chính nó"
```

:::hints
- kind: attention
  body: "Ba bước theo đúng thứ tự — filter() giữ số chẵn, map() bình phương từng số còn lại, reduce() cộng dồn tất cả. Nhớ import reduce từ functools."
- kind: strategy
  body: 'so_chan = filter(lambda x: x % 2 == 0, xs); binh_phuong = map(lambda x: x * x, so_chan); return reduce(lambda acc, x: acc + x, binh_phuong, 0) — ba dòng, mỗi dòng một công cụ.'
- kind: one-line
  body: "from functools import reduce\nso_chan = filter(lambda x: x % 2 == 0, xs)\nbinh_phuong = map(lambda x: x * x, so_chan)\nreturn reduce(lambda acc, x: acc + x, binh_phuong, 0)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: tong_binh_phuong_so_chan phải dùng ĐỦ cả ba công cụ filter()/map()/reduce() — đây là bài ghép lại cả cụm, không phải viết lại bằng vòng lặp hay comprehension.
  requireAst:
  - kind: uses-call, target: filter, min: 1
  - kind: uses-call, target: map, min: 1
  - kind: uses-call, target: reduce, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "56"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba công cụ nhỏ, ghép thành một chuỗi rõ ràng: lọc, biến đổi, gộp. Mỗi
bước chỉ làm ĐÚNG một việc.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã ghép được các hàm THÀNH một chuỗi (`filter` → `map` → `reduce`).
Nhưng mỗi lần gọi lại phải viết lại đúng ba dòng đó. Có cách nào một hàm
"nhớ" luôn một cấu hình — ví dụ luôn nhân với MỘT số cố định — mà không
cần viết lại `lambda` mỗi lần?

Bài sau mở một cụm mới: closures — hàm nhớ được môi trường nó sinh ra.
::::

::::checkpoint{mastery=0.8}
::::
