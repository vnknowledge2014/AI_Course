---
id: khoa-hoc-may-tinh.may-chay-the-nao.dong-cap-phat-khac-dong-cau-truc
title: "'Đống cấp phát' khác 'đống cấu trúc dữ liệu' ở chỗ nào"
summary: "T3.2 dạy đống là một cây nhị phân đặc biệt cài trong mảng. Vùng bộ nhớ chứa MỌI giá trị Python đang sống (list, dict, object bất kỳ) CŨNG gọi là đống — không có gì giống cây nhị phân, tên gọi mượn ở tinh thần 'cấp phát không theo thứ tự cố định', không phải hình dạng."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [may.heap-memory-vs-ds-heap]
requires: [may.call-stack-vs-ds-stack, ds.heap]
concepts: [may.heap-memory-vs-ds-heap]
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
Khác hẳn kiểu khác — lần này không cùng luật, chỉ mượn đúng một chữ.
::::

::::explain{#khong-giong-cay-nhi-phan-chut-nao}
"Ngăn xếp" ở bài trước còn giữ được LUẬT (vào-sau-ra-trước), chỉ khác ai
cầm quyền. "Đống" thì khác hẳn — không có gì chung về HÌNH DẠNG cả.

T3.2 bài 32 dạy đống (heap) là một CÂY NHỊ PHÂN đặc biệt: mỗi nút lớn
hơn hoặc bằng cả hai con, cài gọn trong một mảng. Nó là một cấu trúc dữ
liệu bạn tự dựng, có luật rõ ràng, có hình dạng cụ thể.

Vùng bộ nhớ gọi là **đống cấp phát** (heap memory) trong track này
KHÔNG phải cây, không phải mảng có luật cha-con nào cả. Nó đơn giản là
vùng bộ nhớ chứa MỌI giá trị Python đang tồn tại lúc chương trình chạy
— mọi `list`, mọi `dict`, mọi chuỗi, kể cả những số bạn tự tạo ra khi
chạy (bài sau sẽ nói chính xác điều gì SỐNG trên đó). Interpreter cấp
chỗ cho từng giá trị ở BẤT KỲ ô trống nào tìm được — không theo thứ tự
tạo trước tạo sau, không theo luật cha lớn hơn con nào cả.

Cái tên "đống" mượn đúng Ở TINH THẦN: một đống đồ ai đó chất lên bàn,
không xếp hàng, không theo thứ tự — trái ngược hẳn với ngăn xếp
(bài 23), nơi CHỈ được đụng vào đúng một đầu, theo đúng một luật. Chữ
"đống cấp phát" muốn nói "cấp phát không theo thứ tự cố định" — không
liên quan gì tới việc T3.2's đống là một CÂY. Hai chữ giống nhau, hai
vật khác hẳn nhau — khác hơn cả "ngăn xếp" ở bài trước.
::::

::::example{#khong-lifo-tren-dong}
Ba giá trị tạo lần lượt. Nếu đây là một ngăn xếp, chỉ có giá trị tạo
SAU CÙNG mới đụng được vào. Nhưng đây là đống — thử xem:

```python title=readonly
a = [1, 2, 3]
b = {"mon": "phở", "gia": 45000}
c = "đơn hàng #7"

b = None          # bỏ tên "b" — không .pop(), không quan tâm thứ tự

print(a)
print(c)
```

```text title=readonly
[1, 2, 3]
đơn hàng #7
```

`a` tạo TRƯỚC `b` và `c`. `b` tạo sau `a`, trước `c`. Nếu đây là ngăn
xếp, muốn đụng vào `a` phải "lấy ra" `c` rồi `b` trước — đúng thứ tự
ngược. Nhưng không có luật nào như vậy ở đây: bỏ tên `b` đi (gán `None`,
không dùng `.pop()` gì cả), `a` và `c` vẫn dùng được y nguyên, dù chúng
tạo TRƯỚC `b`. Đống cấp phát không đòi bạn phải "dọn" theo thứ tự nào —
mỗi giá trị sống độc lập, đụng được bất cứ lúc nào, miễn còn tên trỏ
tới nó.
::::

::::predict{#doan-dung-sai-luat-dong commitOnce}
Byte tạo bốn giá trị, rồi bỏ tên của giá trị được tạo ĐẦU TIÊN:

```python
mon_1 = "phở"
mon_2 = "bún chả"
mon_3 = "cơm tấm"
mon_4 = "gỏi cuốn"

mon_1 = None

print(mon_2, mon_3, mon_4)
```

**Trước khi chạy**, bạn đoán dòng in ra có vấn đề gì không?

:::opt{correct}
Không có vấn đề gì — in ra bình thường `bún chả cơm tấm gỏi cuốn`, vì
đống cấp phát không đòi hỏi thứ tự vào-ra như ngăn xếp
:::

:::opt
Lỗi — không thể bỏ `mon_1` trước khi bỏ `mon_2`, `mon_3`, `mon_4`, vì
chúng được tạo SAU `mon_1`
::why
Gần đúng ở việc bạn áp đúng một LUẬT có thật — luật vào-sau-ra-trước,
đúng luật của NGĂN XẾP (bài 23).

Chỗ lệch: luật đó không áp dụng cho đống. Bốn giá trị này không nằm
trên ngăn xếp gọi hàm, chúng nằm trên đống cấp phát — nơi mỗi giá trị
độc lập với nhau, không quan tâm ai tạo trước ai tạo sau. Bỏ `mon_1` đi
không đụng chạm gì tới ba giá trị còn lại.
::
:::

:::opt
In ra `None cơm tấm gỏi cuốn`, vì `mon_1` vẫn còn giữ giá trị cũ ở đâu
đó
::why
Gần đúng ở việc bạn nghĩ tới chuyện giá trị cũ có thể còn "vương lại" —
sự cẩn trọng đó không sai khi nghĩ về bộ nhớ.

Chỗ lệch: dòng `print` không hề nhắc tới `mon_1` — nó chỉ in `mon_2`,
`mon_3`, `mon_4`, ba cái tên chưa từng bị đổi. `mon_1` đổi thành `None`
không ảnh hưởng gì tới nội dung của ba tên khác — chúng vẫn trỏ đúng ba
chuỗi ban đầu của mình.
::
:::

:::opt
Lỗi — đống cấp phát cũng có một luật hình dạng ngầm, giống cây nhị phân
T3.2 dạy, nên xáo trộn thứ tự sẽ làm hỏng cấu trúc
::why
Gần đúng ở việc bạn tin đống có MỘT luật nào đó — đúng, T3.2's đống
(cấu trúc dữ liệu) có luật cha-con thật.

Chỗ lệch: đó là luật của một CẤU TRÚC DỮ LIỆU bạn tự dựng bằng tay,
không phải luật của VÙNG BỘ NHỚ interpreter quản lý. Đống cấp phát
không hề bắt buộc các giá trị Python phải xếp thành cây, thành thứ tự,
hay bất cứ hình dạng nào — mỗi giá trị chỉ đơn giản chiếm một ô trống
tìm được.
::
:::
::::

::::code{#dung-toi-truoc-du-tao-sau}
Một quán ăn ghi điểm khách quen. `vip_dau_tien` được thêm vào TRƯỚC, rồi
năm khách khác thêm vào SAU nó. Dù vậy, `vip_dau_tien` vẫn phải dùng
được bình thường — đúng luật đống cấp phát, không phải luật ngăn xếp.

```python title=starter
kho = []

vip_dau_tien = {"ten": "Byte", "diem": 100}
kho.append(vip_dau_tien)

for i in range(5):
    kho.append({"ten": f"khach_{i}", "diem": i})

# vip_dau_tien vẫn dùng được — dù năm object khác đã tạo SAU nó, và
# đống cấp phát không đòi bạn phải "dọn" chúng trước
diem_vip = ___                 # lấy điểm của vip_dau_tien
print(diem_vip)
print(len(kho))
```

```python title=solution
kho = []

vip_dau_tien = {"ten": "Byte", "diem": 100}
kho.append(vip_dau_tien)

for i in range(5):
    kho.append({"ten": f"khach_{i}", "diem": i})

diem_vip = vip_dau_tien["diem"]
print(diem_vip)
print(len(kho))
```

```python title=test
assert diem_vip == 100, f"diem_vip phải là điểm của vip_dau_tien — 100 — đang ra {diem_vip}"
assert len(kho) == 6, f"kho phải có đúng 6 phần tử (vip_dau_tien + 5 khách sau) — đang ra {len(kho)}"
```

:::hints
- kind: attention
  body: Chỉ một chỗ trống — lấy đúng giá trị ở khoá "diem" của vip_dau_tien, dùng cách tra cứu dict quen thuộc.
- kind: strategy
  body: 'Tra cứu vip_dau_tien theo đúng khoá "diem" — vip_dau_tien["diem"] — dù năm khách khác đã được thêm vào kho sau nó, vip_dau_tien vẫn còn nguyên, dùng được trực tiếp.'
- kind: one-line
  body: 'Chỗ trống là: vip_dau_tien["diem"]'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ tra cứu qua tên vip_dau_tien — không gõ cứng con số 100, vì bài này đang chứng minh vip_dau_tien vẫn DÙNG ĐƯỢC dù tạo trước năm object khác
  requireAst:
  # min: 2 — đếm thật trên solution: vip_dau_tien được ĐỌC đúng 2 lần trong
  # mã nguồn (kho.append(vip_dau_tien) đã có sẵn trong khung + chỗ trống).
  # Điền True/1/0 (một con số không đọc tên nào) chỉ còn 1 lần đọc — dưới 2,
  # luật này chặn được. ĐÃ THỬ THẬT bằng cả ba cách True/1/0: không vòng
  # lặp nào phụ thuộc chỗ trống (vòng for đã cố định range(5), không blank),
  # nên không cách nào chạy vô hạn — cả ba dừng ngay, và cả ba cho diem_vip
  # sai (True/1/0, không phải 100) — assert bắt được độc lập với luật static
  # này.
  - kind: uses-name, target: vip_dau_tien, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^100\\n6\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm object khác chen vào sau, và `vip_dau_tien` chẳng hề nhúc nhích.
Đống không bắt ai xếp hàng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đống cấp phát giữ giá trị THẬT — `list`, `dict`, mọi object đang sống.
Nhưng cái TÊN bạn gõ (`vip_dau_tien`, `kho`) — bản thân cái tên đó nằm ở
đâu? Nó nằm CÙNG một chỗ với giá trị nó trỏ tới, hay ở một vùng hoàn
toàn khác?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
