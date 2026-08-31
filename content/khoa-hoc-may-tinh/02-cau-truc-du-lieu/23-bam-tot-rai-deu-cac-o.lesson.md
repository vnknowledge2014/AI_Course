---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.bam-tot-rai-deu-cac-o
title: "Băm tốt là băm rải đều các ô"
summary: "Khi tỉ lệ giá trị trên ô vượt một ngưỡng, mỗi ô biến thành một danh sách liên kết dài — bảng băm phải tự lớn lên và chèn lại toàn bộ, đúng như mảng động ở bài 3, nhưng phải tính lại từng chỉ số chứ không chỉ chép."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.hash-load-factor]
requires: [ds.hash-chaining, ds.dynamic-array, ctrl.while, ctrl.for-each, core.function-def]
concepts: [ds.hash-load-factor]
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
Bài trước vá được lỗi mất dữ liệu. Nhưng nếu MỌI ô đều dài dằng dặc thì
lời hứa "nhảy thẳng" của bảng băm còn nghĩa lý gì?
::::

::::explain{#he-so-tai-va-lon-bang}
Gọi **hệ số tải** (load factor) của một bảng băm là:

> hệ số tải = số mục đang có / số ô của bảng

Hệ số tải càng cao, càng nhiều mục phải chen chung một ô — mỗi ô, theo
bài 22, là một danh sách liên kết, và chuỗi càng dài thì `tra_cuu` càng
phải đi bộ qua nhiều nút hơn để tìm đúng thứ cần tìm. "Băm rồi nhảy
thẳng" (bài 20) chỉ còn đúng phần NHẢY THẲNG TỚI Ô; một khi đã tới ô, nếu
ô đó dài, phần còn lại vẫn phải ĐI BỘ — y hệt cái giá danh sách liên kết
từng phải trả ở bài 16.

Cách giữ hệ số tải thấp: khi nó vượt một NGƯỠNG (một con số chọn trước,
ví dụ 0.7), bảng phải **tự lớn lên** — đúng việc mảng động đã làm ở bài 3.
Nhưng có một khác biệt quan trọng với mảng động: mảng động khi lớn lên chỉ
CHÉP nguyên xi các phần tử cũ sang vùng mới, thứ tự không đổi. Bảng băm
thì không chép được — công thức bài 19 là `tong % co_bang`, và `co_bang`
vừa đổi. Chỉ số cũ của mọi mục giờ đã SAI. Bảng phải:

1. Dựng một mảng MỚI, lớn hơn (thường gấp đôi).
2. Đi qua TỪNG mục đang có trong bảng CŨ.
3. TÍNH LẠI chỉ số của từng mục bằng `co_bang` MỚI, rồi chèn lại vào bảng
   mới.

Bước này gọi là **chèn lại toàn bộ** (rehash). Nó không rẻ — phải chạm
vào MỌI mục đang có, không chỉ mục mới — nhưng đổi lại, hệ số tải sau khi
lớn giảm hẳn, mỗi ô lại ngắn, tra cứu lại gần với "nhảy thẳng" như ban
đầu.
::::

::::example{#bang_chat_roi_lon}
Byte dựng một bảng CHẬT — chỉ 4 ô — rồi chèn 4 người, dùng đúng `chen` xâu
chuỗi của bài 22:

```python title=readonly
def bam(ten, co_bang):
    return sum(ten.encode("utf-8")) % co_bang

def chen(bang, ten, so_dien_thoai, co_bang):
    chi_so = bam(ten, co_bang)
    nut_moi = {"gia_tri": {"ten": ten, "so_dien_thoai": so_dien_thoai}, "tiep": bang[chi_so]}
    bang[chi_so] = nut_moi

co_bang = 4
danh_ba = [None] * co_bang
chen(danh_ba, "An", "0901111111", co_bang)
chen(danh_ba, "Hoa", "0902222222", co_bang)
chen(danh_ba, "Lan", "0903333333", co_bang)
chen(danh_ba, "Nam", "0904444444", co_bang)

print(danh_ba)
```

```text title=readonly
[{'gia_tri': {'ten': 'Nam', 'so_dien_thoai': '0904444444'}, 'tiep': {'gia_tri': {'ten': 'Hoa', 'so_dien_thoai': '0902222222'}, 'tiep': None}}, None, None, {'gia_tri': {'ten': 'Lan', 'so_dien_thoai': '0903333333'}, 'tiep': {'gia_tri': {'ten': 'An', 'so_dien_thoai': '0901111111'}, 'tiep': None}}]
```

4 mục, 4 ô — hệ số tải đúng bằng `1.0`. Nhìn kỹ: ô 0 đã có 2 nút (Nam,
Hoa), ô 3 cũng có 2 nút (Lan, An) — hai chuỗi dài gấp đôi mức cần thiết,
trong khi ô 1 và ô 2 trống trơn. Hàm băm không "cố ý" xếp lệch — đây chỉ
là hệ quả của việc nhét 4 mục vào một bảng quá chật.

Giờ lớn bảng lên gấp đôi, `co_bang_moi = 8`, và chèn lại từng mục:

```python title=readonly
def lon_bang(bang_cu, co_bang_moi):
    bang_moi = [None] * co_bang_moi
    for o_cu in bang_cu:
        nut = o_cu
        while nut is not None:
            muc = nut["gia_tri"]
            chen(bang_moi, muc["ten"], muc["so_dien_thoai"], co_bang_moi)
            nut = nut["tiep"]
    return bang_moi

danh_ba_lon = lon_bang(danh_ba, 8)
for o in danh_ba_lon:
    print(o)
```

```text title=readonly
{'gia_tri': {'ten': 'Hoa', 'so_dien_thoai': '0902222222'}, 'tiep': None}
None
None
{'gia_tri': {'ten': 'Lan', 'so_dien_thoai': '0903333333'}, 'tiep': None}
{'gia_tri': {'ten': 'Nam', 'so_dien_thoai': '0904444444'}, 'tiep': None}
None
None
{'gia_tri': {'ten': 'An', 'so_dien_thoai': '0901111111'}, 'tiep': None}
```

Sau khi lớn: 4 mục, 8 ô, hệ số tải giảm còn `0.5` — và mỗi mục giờ đứng
MỘT MÌNH trong ô của nó, không còn chuỗi nào dài quá một nút. Chú ý:
`lon_bang` không hề CHÉP các nút cũ sang chỗ mới — nó đọc lại `ten` và
`so_dien_thoai` từ mỗi nút cũ, rồi gọi `chen` để TÍNH CHỈ SỐ MỚI và dựng
NÚT MỚI hoàn toàn, đúng như bài học vừa nói.
::::

::::predict{#doan-he-so-tai commitOnce}
Ngay TRƯỚC khi gọi `lon_bang` ở ví dụ trên: `danh_ba` có `co_bang = 4` và
đã chứa đúng 4 mục (An, Hoa, Lan, Nam).

**Trước khi tính**, bạn đoán hệ số tải của `danh_ba` LÚC ĐÓ — trước khi
lớn — bằng bao nhiêu?

:::opt{correct}
`1.0` — 4 mục chia cho 4 ô.
:::

:::opt
`0.25` — 1 chia cho 4
::why
Gần đúng ở chỗ cả hai con số `1` và `4` đều có THẬT trong bài toán —
không bịa ra con số nào.

Chỗ lệch là bạn tính NGƯỢC: hệ số tải là SỐ MỤC chia cho SỐ Ô
(`4 / 4`), không phải một mục chia cho số ô. Đảo tử số và mẫu số ra một
con số hoàn toàn khác nghĩa — `0.25` sẽ nói bảng đang RẤT RỘNG RÃI, ngược
hẳn với tình trạng thật.
::
:::

:::opt
`4` — đúng bằng số mục đang có
::why
Gần đúng ở chỗ `4` là một con số có thật — đúng số mục `danh_ba` đang
giữ.

Chỗ lệch là hệ số tải luôn là một TỈ LỆ (số mục CHIA CHO số ô), không phải
một con số đếm suông. Quên chia cho số ô nghĩa là bỏ mất một nửa công
thức — con số `4` này sẽ không đổi dù bảng có 4 ô hay 400 ô, trong khi hệ
số tải PHẢI đổi theo cỡ bảng.
::
:::

:::opt
`8` — cỡ bảng SAU KHI đã lớn lên
::why
Gần đúng ở chỗ `8` cũng là một con số có thật trong ví dụ — chỉ là nó
thuộc về bảng SAU, không phải bảng câu hỏi đang hỏi tới.

Chỗ lệch là câu hỏi hỏi hệ số tải TRƯỚC khi gọi `lon_bang` — lúc đó
`danh_ba` vẫn còn `co_bang = 4`, chưa hề lớn lên. `8` là cỡ bảng MỚI, chỉ
tồn tại SAU khi `lon_bang` chạy xong.
::
:::
::::

::::code{#lon_bang}
`chen` và `tra_cuu` đã có sẵn, đừng sửa. Bạn hoàn thành `lon_bang`: chỗ
trống phải chèn từng mục cũ vào bảng MỚI bằng đúng cỡ bảng MỚI, không
phải cỡ bảng cũ.

```python title=starter
def bam(ten, co_bang):
    return sum(ten.encode("utf-8")) % co_bang

def chen(bang, ten, so_dien_thoai, co_bang):
    chi_so = bam(ten, co_bang)
    nut_moi = {"gia_tri": {"ten": ten, "so_dien_thoai": so_dien_thoai}, "tiep": bang[chi_so]}
    bang[chi_so] = nut_moi

def tra_cuu(bang, ten, co_bang):
    chi_so = bam(ten, co_bang)
    nut = bang[chi_so]
    while nut is not None:
        if nut["gia_tri"]["ten"] == ten:
            return nut["gia_tri"]["so_dien_thoai"]
        nut = nut["tiep"]
    return None

def lon_bang(bang_cu, co_bang_moi):
    bang_moi = [None] * co_bang_moi
    for o_cu in bang_cu:
        nut = o_cu
        while nut is not None:
            muc = nut["gia_tri"]
            chen(bang_moi, muc["ten"], muc["so_dien_thoai"], ___)
            nut = nut["tiep"]
    return bang_moi

co_bang = 4
danh_ba = [None] * co_bang
chen(danh_ba, "An", "0901111111", co_bang)
chen(danh_ba, "Hoa", "0902222222", co_bang)
chen(danh_ba, "Lan", "0903333333", co_bang)
chen(danh_ba, "Nam", "0904444444", co_bang)

danh_ba_lon = lon_bang(danh_ba, 8)

print(len(danh_ba_lon))
print(tra_cuu(danh_ba_lon, "An", 8))
print(tra_cuu(danh_ba_lon, "Hoa", 8))
print(tra_cuu(danh_ba_lon, "Lan", 8))
print(tra_cuu(danh_ba_lon, "Nam", 8))
```

```python title=solution
def bam(ten, co_bang):
    return sum(ten.encode("utf-8")) % co_bang

def chen(bang, ten, so_dien_thoai, co_bang):
    chi_so = bam(ten, co_bang)
    nut_moi = {"gia_tri": {"ten": ten, "so_dien_thoai": so_dien_thoai}, "tiep": bang[chi_so]}
    bang[chi_so] = nut_moi

def tra_cuu(bang, ten, co_bang):
    chi_so = bam(ten, co_bang)
    nut = bang[chi_so]
    while nut is not None:
        if nut["gia_tri"]["ten"] == ten:
            return nut["gia_tri"]["so_dien_thoai"]
        nut = nut["tiep"]
    return None

def lon_bang(bang_cu, co_bang_moi):
    bang_moi = [None] * co_bang_moi
    for o_cu in bang_cu:
        nut = o_cu
        while nut is not None:
            muc = nut["gia_tri"]
            chen(bang_moi, muc["ten"], muc["so_dien_thoai"], co_bang_moi)
            nut = nut["tiep"]
    return bang_moi

co_bang = 4
danh_ba = [None] * co_bang
chen(danh_ba, "An", "0901111111", co_bang)
chen(danh_ba, "Hoa", "0902222222", co_bang)
chen(danh_ba, "Lan", "0903333333", co_bang)
chen(danh_ba, "Nam", "0904444444", co_bang)

danh_ba_lon = lon_bang(danh_ba, 8)

print(len(danh_ba_lon))
print(tra_cuu(danh_ba_lon, "An", 8))
print(tra_cuu(danh_ba_lon, "Hoa", 8))
print(tra_cuu(danh_ba_lon, "Lan", 8))
print(tra_cuu(danh_ba_lon, "Nam", 8))
```

```python title=test
assert len(danh_ba_lon) == 8, "bảng mới phải có đúng 8 ô — bằng co_bang_moi truyền vào lon_bang"
assert tra_cuu(danh_ba_lon, "An", 8) == "0901111111", "sau khi lớn bảng, tra 'An' với co_bang=8 (đúng cỡ bảng mới) phải vẫn ra đúng số của An"
assert tra_cuu(danh_ba_lon, "Hoa", 8) == "0902222222", "tra 'Hoa' sau khi lớn bảng phải ra đúng số"
assert tra_cuu(danh_ba_lon, "Lan", 8) == "0903333333", "tra 'Lan' sau khi lớn bảng phải ra đúng số"
assert tra_cuu(danh_ba_lon, "Nam", 8) == "0904444444", "tra 'Nam' sau khi lớn bảng phải ra đúng số"
assert all(o is None or o["tiep"] is None for o in danh_ba_lon), "với 4 mục trong 8 ô, mỗi ô đang dùng chỉ nên có đúng MỘT nút — nếu có ô hai nút trở lên, chỉ số đang tính sai cỡ bảng"

# Lớn bảng LẦN NỮA, sang một cỡ KHÁC HẲN (16, không phải 8) — nếu chỗ trống
# hardcode con số 8 (đúng cho lần gọi lon_bang(danh_ba, 8) ở trên) thay vì
# dùng đúng tham số co_bang_moi, lần gọi lon_bang(danh_ba, 16) này sẽ chèn
# lại vào SAI cỡ, tra cứu ra None thay vì đúng số điện thoại.
danh_ba_lon2 = lon_bang(danh_ba, 16)
assert len(danh_ba_lon2) == 16, "lon_bang(danh_ba, 16) phải dựng bảng mới đúng CỠ 16 được truyền vào"
assert tra_cuu(danh_ba_lon2, "An", 16) == "0901111111", "chèn lại vào bảng cỡ 16 phải tính chỉ số bằng ĐÚNG co_bang_moi (16) cho từng mục — không phải hardcode con số 8 đã dùng ở lần lớn bảng trước"
assert tra_cuu(danh_ba_lon2, "Hoa", 16) == "0902222222", "tra 'Hoa' trong bảng cỡ 16 phải đúng"
assert tra_cuu(danh_ba_lon2, "Lan", 16) == "0903333333", "tra 'Lan' trong bảng cỡ 16 phải đúng"
assert tra_cuu(danh_ba_lon2, "Nam", 16) == "0904444444", "tra 'Nam' trong bảng cỡ 16 phải đúng"
```

:::hints
- kind: attention
  body: Vòng lặp đang chèn từng mục CŨ vào `bang_moi` — nhưng `bang_moi` có cỡ `co_bang_moi`, không phải cỡ bảng cũ. `chen` cần biết ĐÚNG cỡ bảng nó đang chèn vào để tính chỉ số cho khớp.
- kind: strategy
  body: "Tham số cuối của `chen` phải là cỡ của bảng đang được chèn vào — ở đây là `bang_moi`, tức đúng bằng `co_bang_moi`. Đừng dùng lại biến `co_bang` toàn cục hay bất kỳ con số cố định nào — dùng đúng tham số `co_bang_moi` mà `lon_bang` đã nhận."
- kind: one-line
  body: 'Điền `co_bang_moi` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^8\\n0901111111\\n0902222222\\n0903333333\\n0904444444\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
4 mục, chật cứng trong 4 ô, giờ rải đều trong 8 ô — mỗi ô đúng một nút.
Bảng vừa tự lớn lên, đúng lúc.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa TỰ TAY dựng một bảng băm hoàn chỉnh: hàm băm, mảng, xâu chuỗi khi
đụng độ, tự lớn khi chật. Đúng ba việc mà `dict` của Python — thứ bạn đã
dùng hàng trăm lần từ R1 — làm ẩn phía sau `d[khoa]`.

Nếu `dict` thật sự làm đúng ba việc bạn vừa dựng bằng tay, thì có việc nào
`dict` làm mà mấy bài vừa qua CHƯA nói tới — ví dụ, tại sao có kiểu dữ
liệu bạn KHÔNG được phép dùng làm khoá của nó?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
