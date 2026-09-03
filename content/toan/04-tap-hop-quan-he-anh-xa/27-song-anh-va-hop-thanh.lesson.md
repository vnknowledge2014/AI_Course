---
id: toan.tap-hop-quan-he-anh-xa.song-anh-va-hop-thanh
title: Song ánh và hợp thành
summary: "Song ánh = đơn ánh + toàn ánh — CHỈ khi đó ánh xạ mới đảo ngược được. Hợp thành nối hai ánh xạ (luống→người, người→làng) thành một ánh xạ luống→làng — và đổi thứ tự nối thì đổi luôn kết quả."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.bijective, math.relation-composition]
requires: [math.surjective, math.function-composition]
concepts: [math.song-anh, math.hop-thanh, math.thu-tu-hop-thanh]
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
`phan_cong_toan` (bài 26) vừa đơn ánh vừa toàn ánh — CẢ HAI đúng
cùng lúc. Điều đó có tên riêng không, và nó làm được gì mà ánh xạ
thường thì không?
::::

::::explain{#song-anh-la-gi}
Có. **Song ánh** — vừa đơn ánh (bài 25) vừa toàn ánh (bài 26), kiểm
bằng `and` của hai hàm đã có:

```python title=readonly
def la_don_anh(phan_cong):
    nguoi = {y for (x, y) in phan_cong}
    return len(nguoi) == len(phan_cong)

def la_toan_anh(phan_cong, b):
    nguoi_co_viec = {y for (x, y) in phan_cong}
    return nguoi_co_viec == b

def la_song_anh(phan_cong, b):
    return la_don_anh(phan_cong) and la_toan_anh(phan_cong, b)


nguoi_lam_vuon = {"Lan", "Minh", "Tu"}
phan_cong_song_anh = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu")}

print(la_song_anh(phan_cong_song_anh, nguoi_lam_vuon))
```

```text title=readonly
True
```

MỖI luống một người KHÁC nhau (đơn ánh) VÀ mọi người ĐỀU có việc
(toàn ánh) — khi CẢ HAI đúng, đảo NGƯỢC từng cặp (`luống, người` →
`người, luống`) vẫn LÀ một ánh xạ hợp lệ: MỖI người giờ trỏ tới ĐÚNG
MỘT luống. Song ánh LÀ điều kiện để "đảo ngược" hoạt động — thiếu
MỘT trong hai, phép đảo sẽ PHÁ vỡ (một người trỏ tới HAI luống, hoặc
một luống KHÔNG ai trỏ tới).
::::

::::example{#khong-song-anh}
Bốn luống, ba người — Lan phụ trách CẢ `luong_1` lẫn `luong_4` (bài
26). MỌI người đều có việc (toàn ánh) NHƯNG Lan bị gán TRÙNG — KHÔNG
song ánh:

```python title=readonly
def la_don_anh(phan_cong):
    nguoi = {y for (x, y) in phan_cong}
    return len(nguoi) == len(phan_cong)

def la_toan_anh(phan_cong, b):
    nguoi_co_viec = {y for (x, y) in phan_cong}
    return nguoi_co_viec == b

def la_song_anh(phan_cong, b):
    return la_don_anh(phan_cong) and la_toan_anh(phan_cong, b)

nguoi_lam_vuon = {"Lan", "Minh", "Tu"}
phan_cong_4luong = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu"), ("luong_4", "Lan")}

print(la_toan_anh(phan_cong_4luong, nguoi_lam_vuon))
print(la_song_anh(phan_cong_4luong, nguoi_lam_vuon))
```

```text title=readonly
True
False
```

`la_toan_anh` LÀ `True` (Lan, Minh, Tu ĐỀU có việc) NHƯNG `la_don_anh`
LÀ `False` (Lan xuất hiện Ở HAI cặp) — `and` khiến `la_song_anh` trả
VỀ `False`. Toàn ánh THÔI CHƯA đủ; PHẢI CẢ HAI cùng lúc.
::::

::::explain{#hop-thanh-la-gi}
Byte ĐÃ nối hai hàm số liên tiếp trước đây (T2.2 bài 34) — `f(g(x))`.
**Hợp thành** LÀM Y HỆT điều đó, NHƯNG trên ánh xạ VIẾT bằng tập cặp:
nối `luống → người` VỚI `người → làng` thành `luống → làng`, GHÉP hai
cặp khi VẾ SAU của cặp thứ NHẤT trùng VẾ TRƯỚC của cặp thứ HAI:

```python title=readonly
def hop_thanh(quan_he1, quan_he2):
    return {(x, z) for (x, y1) in quan_he1 for (y2, z) in quan_he2 if y1 == y2}


phan_cong = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu")}
nguoi_lang = {("Lan", "Lang_A"), ("Minh", "Lang_B"), ("Tu", "Lang_A")}

print(sorted(hop_thanh(phan_cong, nguoi_lang)))
```

```text title=readonly
[('luong_1', 'Lang_A'), ('luong_2', 'Lang_B'), ('luong_3', 'Lang_A')]
```

(In bằng `sorted(...)` VÌ `hop_thanh` trả VỀ một `set` — THỨ TỰ của
`set` KHÔNG cố định, bài 1.) `luong_1` nối QUA `Lan` tới `Lang_A`;
`luong_3` nối QUA `Tu` CŨNG tới `Lang_A` — hai luống KHÁC nhau CÙNG
ra một làng LÀ chuyện BÌNH THƯỜNG của hợp thành, dù `phan_cong` (vế
đầu) đơn ánh.
::::

::::predict{#doan-doi-thu-tu-hop-thanh commitOnce}
Byte gọi `hop_thanh` theo thứ tự NGƯỢC — `hop_thanh(nguoi_lang,
phan_cong)` thay vì `hop_thanh(phan_cong, nguoi_lang)`:

```python
def hop_thanh(quan_he1, quan_he2):
    return {(x, z) for (x, y1) in quan_he1 for (y2, z) in quan_he2 if y1 == y2}

phan_cong = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu")}
nguoi_lang = {("Lan", "Lang_A"), ("Minh", "Lang_B"), ("Tu", "Lang_A")}

print(hop_thanh(nguoi_lang, phan_cong))
```

Dòng cuối in ra gì?

:::opt{correct}
`set()` — tập rỗng
:::

:::opt
Y HỆT kết quả trước, CHỈ đảo chiều từng cặp — `{('Lang_A', 'luong_1'),
('Lang_B', 'luong_2'), ('Lang_A', 'luong_3')}` — vì hợp thành, GIỐNG
phép cộng, KHÔNG quan tâm thứ tự hai đối số
::why
Gần đúng ở việc bạn nhớ ĐÚNG kết quả GỐC (`luống → làng` qua `Lan`,
`Minh`, `Tu`) — một quan sát chính xác VỀ VÍ DỤ trước.

Chỗ lệch: T2.2 bài 35 ĐÃ chỉ ra đổi thứ tự nối hàm THÌ đổi luôn kết
quả — hợp thành đây CŨNG vậy, và LỆCH còn RÕ hơn: `hop_thanh(quan_he1,
quan_he2)` ghép cặp KHI vế SAU của `quan_he1` khớp vế TRƯỚC của
`quan_he2`. Gọi `hop_thanh(nguoi_lang, phan_cong)` nghĩa LÀ tìm vế
SAU của `nguoi_lang` (TÊN LÀNG, như `Lang_A`) khớp vế TRƯỚC của
`phan_cong` (TÊN LUỐNG, như `luong_1`) — HAI loại dữ liệu này KHÔNG
BAO GIỜ trùng nhau. KHÔNG cặp nào ghép được.
::
:::

:::opt
Máy báo lỗi biên dịch — `hop_thanh` được ĐỊNH NGHĨA để nhận `quan_he1`
LÀ `luống → người` trước, Python sẽ TỪ CHỐI chạy KHI đối số đầu LÀ
`nguoi_lang` thay vì `phan_cong`
::why
Gần đúng ở việc bạn nghĩ TỚI vai trò CỐ ĐỊNH của từng đối số TRONG Ý
ĐỊNH thiết kế (đối số đầu "NÊN" LÀ `luống → người`) — một trực giác
hợp lý VỀ mặt Ý NGHĨA.

Chỗ lệch: Python KHÔNG hề gắn KIỂU dữ liệu cho THAM SỐ của
`hop_thanh` — CẢ `quan_he1` LẪN `quan_he2` chỉ ĐỢI nhận MỘT tập cặp
BẤT KỲ, thứ tự đối số nào cũng biên dịch sạch, chạy sạch. CHỈ có KẾT
QUẢ (rỗng HAY không) mới lộ ra thứ tự SAI VỀ mặt Ý NGHĨA toán học.
::
:::
::::

::::code{#viet_hop_thanh}
Viết `hop_thanh(quan_he1, quan_he2)` — nối `quan_he1` VỚI `quan_he2`
thành một quan hệ MỚI: ghép `(x, z)` KHI tồn tại `y` sao cho `(x, y)`
thuộc `quan_he1` VÀ `(y, z)` thuộc `quan_he2`.

```python title=starter
def hop_thanh(quan_he1, quan_he2):
    return ___


phan_cong = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu")}
nguoi_lang = {("Lan", "Lang_A"), ("Minh", "Lang_B"), ("Tu", "Lang_A")}

print(sorted(hop_thanh(phan_cong, nguoi_lang)))
```

```python title=solution
def hop_thanh(quan_he1, quan_he2):
    return {(x, z) for (x, y1) in quan_he1 for (y2, z) in quan_he2 if y1 == y2}


phan_cong = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu")}
nguoi_lang = {("Lan", "Lang_A"), ("Minh", "Lang_B"), ("Tu", "Lang_A")}

print(sorted(hop_thanh(phan_cong, nguoi_lang)))
```

```python title=test
assert hop_thanh(set(), set()) == set(), "tap rong -- hop thanh hien nhien"
assert hop_thanh(nguoi_lang, phan_cong) == set(), "vai tro doi -- khong cap nao ghep duoc"
assert len(hop_thanh(phan_cong, nguoi_lang)) == 3, "ba luong deu noi duoc toi mot lang"
assert ("luong_1", "Lang_A") in hop_thanh(phan_cong, nguoi_lang), "luong_1 qua Lan toi Lang_A"
```

:::hints
- kind: attention
  body: "Dung set comprehension long hai vong for: mot qua quan_he1 (x, y1), mot qua quan_he2 (y2, z), chi giu khi y1 == y2."
- kind: strategy
  body: "{(x, z) for (x, y1) in quan_he1 for (y2, z) in quan_he2 if y1 == y2}"
- kind: one-line
  body: "return {(x, z) for (x, y1) in quan_he1 for (y2, z) in quan_he2 if y1 == y2}"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung set comprehension long hai vong for tren quan_he1 va quan_he2, ghep (x, z) khi y1 == y2
  requireAst:
  - kind: comprehension, min: 1
  - kind: uses-operator, target: '==', min: 1
  - kind: uses-name, target: quan_he2, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[\('luong_1', 'Lang_A'\), \('luong_2', 'Lang_B'\), \('luong_3', 'Lang_A'\)\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Song ánh = đảo ngược được. Hợp thành = nối hai ánh xạ, thứ tự tính.
Mọi luống giờ có: tập rau, quan hệ tưới, người phụ trách. Ghép cả ba
lại trên MỘT khu vườn trông như thế nào?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte có bốn luống, mỗi luống một tập rau (bài 1-13), một chỗ trong
quan hệ "cùng lịch tưới" (bài 14-23), và một người phụ trách QUA một
ánh xạ (bài 24-27). Nếu ánh xạ phụ trách ĐÓ song ánh, Byte có suy
ngược từ MỘT người ra ĐÚNG MỘT luống không — và cách nào kiểm điều
đó bằng đúng những hàm đã viết?
::::

::::checkpoint{mastery=0.8}
::::
