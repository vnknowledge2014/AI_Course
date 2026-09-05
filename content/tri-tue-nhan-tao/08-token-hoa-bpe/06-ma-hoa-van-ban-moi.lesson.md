---
id: tri-tue-nhan-tao.token-hoa-bpe.ma-hoa-van-ban-moi
title: "Mã hoá văn bản mới: áp danh sách merge đã học, ĐÚNG thứ tự"
summary: "Cài ma_hoa_van_ban: áp 8 merge đã học ở bài trước (theo ĐÚNG thứ tự) lên một câu MỚI, 'con ga con an ca', chưa từng xuất hiện lúc huấn luyện -- ra đúng 9 token, ID [9, 11, 6, 0, 9, 6, 15, 7, 6]. Áp SAI thứ tự (đảo ngược) cho 11 token, KHÁC hẳn -- bằng chứng cụ thể thứ tự merge quyết định kết quả mã hoá, không phải một chi tiết phụ."
locale: vi
track: tri-tue-nhan-tao
module: token-hoa-bpe
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.ma-hoa-van-ban-moi]
requires: [ai.lap-toi-vocab-muc-tieu]
concepts: [ai.ma-hoa-van-ban-moi]
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

::::byte{trigger=enter mood=curious pose=point-stage}
Huấn luyện xong — `8` merge, đúng thứ tự, đã ghi lại. Nhưng huấn luyện chỉ
là một nửa câu chuyện. Nửa còn lại: dùng đúng danh sách đó để mã hoá một
câu HOÀN TOÀN MỚI, một câu BPE chưa từng thấy lúc học.
::::

::::explain{#ma_hoa_dung_thu_tu}
Mã hoá (encode) một câu mới nghĩa là: bắt đầu từ danh sách ký tự đơn của
câu đó, rồi áp LẦN LƯỢT từng cặp trong danh sách `merges` đã học — theo
ĐÚNG thứ tự chúng được học lúc huấn luyện, không phải thứ tự tuỳ ý:

> `for cap in danh_sach_merge: ds = gop_cap(ds, cap)`

Vì sao thứ tự quan trọng? Merge thứ hai (`('c', 'o')` trong ví dụ bài
trước) được HỌC dựa trên giả định rằng merge thứ NHẤT (`('n', ' ')`) ĐÃ
được áp dụng — một số cặp `('c', 'o')` chỉ lộ ra RÕ vị trí sau khi khoảng
trắng theo sau `n` đã bị "nuốt" vào token `'n '`. Áp các merge theo thứ tự
KHÁC — ví dụ đảo ngược, `('c', 'o')` TRƯỚC `('n', ' ')` — sẽ khớp những
cặp khác đi, sản sinh một chuỗi token HOÀN TOÀN khác, dù dùng đúng CÙNG
một tập hợp các cặp.

Sau khi có chuỗi token cuối cùng, bước cuối là tra ID cho từng token qua
bảng `token_sang_id` (xây một lần, lúc huấn luyện — mỗi token trong vocab
có đúng một ID cố định) — kết quả là một danh sách số nguyên, sẵn sàng đưa
vào `Value`/`Tensor`.
::::

::::example{#ma_hoa_cau_moi_that}
Áp `8` merge đã học (bài trước) lên câu `"con ga con an ca"` — MỘT câu
CHƯA TỪNG xuất hiện trong corpus huấn luyện — theo ĐÚNG thứ tự, rồi so với
áp NGƯỢC thứ tự:

```python title=readonly
def gop_cap(danh_sach, cap):
    a, b = cap
    token_moi = a + b
    ra = []
    i = 0
    while i < len(danh_sach):
        if i < len(danh_sach) - 1 and danh_sach[i] == a and danh_sach[i + 1] == b:
            ra.append(token_moi)
            i += 2
        else:
            ra.append(danh_sach[i])
            i += 1
    return ra

def ma_hoa_token(van_ban, danh_sach_merge):
    ds = list(van_ban)
    for cap in danh_sach_merge:
        ds = gop_cap(ds, cap)
    return ds

danh_sach_merge = [('n', ' '), ('c', 'o'), ('co', 'n '), (' ', 'a'),
                    (' a', 'n '), ('o', ' an '), (',', ' '), (', ', 'con ')]

cau_moi = "con ga con an ca"
tokens_dung_thu_tu = ma_hoa_token(cau_moi, danh_sach_merge)
tokens_sai_thu_tu = ma_hoa_token(cau_moi, list(reversed(danh_sach_merge)))

print("dung thu tu:", tokens_dung_thu_tu)
print("so token (dung thu tu):", len(tokens_dung_thu_tu))
print("sai thu tu (dao nguoc):", tokens_sai_thu_tu)
print("so token (sai thu tu):", len(tokens_sai_thu_tu))
```

```text title=readonly
dung thu tu: ['con ', 'g', 'a', ' ', 'con ', 'a', 'n ', 'c', 'a']
so token (dung thu tu): 9
sai thu tu (dao nguoc): ['co', 'n ', 'g', 'a', ' ', 'co', 'n', ' a', 'n ', 'c', 'a']
so token (sai thu tu): 11
```

Câu `"con ga con an ca"` (`16` ký tự) mã hoá ĐÚNG thứ tự ra `9` token —
`'con '` (cả hai lần từ `"con"` xuất hiện đều gộp trọn vẹn) là một token
DUY NHẤT dù dài `4` ký tự. Áp SAI thứ tự (đảo ngược danh sách merge) cho
`11` token — nhiều hơn, và tách sai: `"con"` không còn được gộp trọn vẹn ở
lần xuất hiện thứ hai (`'co', 'n'` tách rời thay vì `'con '` liền một
khối) — CÙNG một tập hợp `8` cặp, chỉ khác THỨ TỰ áp dụng, cho ra hai kết
quả mã hoá khác nhau hẳn.
::::

::::predict{#doan_thu_tu_merge commitOnce}
Xét CÙNG `8` merge, CÙNG câu `"con ga con an ca"` như trên.

**Trước khi chạy thử**, bạn đoán: nếu áp merge theo một thứ tự KHÁC nữa —
lần này chỉ hoán đổi HAI merge ĐẦU TIÊN cho nhau (`('c', 'o')` trước,
`('n', ' ')` sau, còn lại giữ nguyên thứ tự `6` merge cuối) — kết quả mã
hoá có chắc chắn GIỐNG với kết quả đúng thứ tự (`9` token) không?

:::opt{correct}
Không chắc chắn giống — đổi thứ tự của NGAY CẢ hai merge đầu tiên cũng có
thể làm những merge SAU đó (vốn giả định merge trước đã chạy xong) khớp
sai vị trí, cho một chuỗi token khác; cách chắc chắn duy nhất để biết là
CHẠY THẬT với đúng thứ tự đã hoán đổi, không suy luận suông
:::

:::opt
Chắc chắn giống — vì tập hợp `8` cặp merge không đổi, chỉ đổi chỗ hai
phần tử ĐẦU trong một danh sách, và phép gộp cặp không quan tâm thứ tự
::why
Gần đúng ở việc tập hợp CÁC CẶP đúng là không đổi — vẫn `8` cặp y hệt,
không thêm không bớt.

Chỗ lệch: `gop_cap` là một phép biến đổi PHỤ THUỘC THỨ TỰ — mỗi lần gộp
thay đổi chính danh sách token mà lần gộp TIẾP THEO sẽ nhìn thấy. Đảo vị
trí của `('c', 'o')` và `('n', ' ')` nghĩa là `('c', 'o')` giờ chạy TRÊN
danh sách ký tự đơn gốc (chưa có `'n '` nào), khớp những vị trí khác hẳn
so với khi nó chạy SAU `('n', ' ')` (trên danh sách đã có `'n '`) — đây
chính là bài học cốt lõi của bài này, không phải một trường hợp ngoại lệ.
::
:::

:::opt
Chắc chắn KHÁC — bất kỳ thay đổi thứ tự nào, dù chỉ hoán đổi hai vị trí,
LUÔN cho một kết quả khác
::why
Gần đúng ở việc nhấn mạnh thứ tự CÓ THỂ tạo khác biệt — tinh thần cảnh
giác đó đúng hướng.

Chỗ lệch: "có thể khác" không phải "chắc chắn luôn khác trong MỌI trường
hợp". Có những câu, hoặc những cách hoán đổi, mà kết quả cuối cùng TÌNH CỜ
trùng nhau (ví dụ nếu hai merge bị hoán đổi không bao giờ áp dụng được lên
cùng một vị trí trong câu cụ thể đó). Khẳng định "LUÔN LUÔN khác" là một
khẳng định QUÁ MẠNH mà không có phép thử nào — điều DUY NHẤT chắc chắn
được là "không đảm bảo giống", không phải "đảm bảo khác".
::
:::
::::

::::code{#viet_ma_hoa_van_ban}
Hoàn thiện `ma_hoa_van_ban`: áp lần lượt từng merge lên danh sách token,
rồi tra ID cho từng token trong kết quả CUỐI CÙNG.

```python title=starter
def gop_cap(danh_sach, cap):
    a, b = cap
    token_moi = a + b
    ra = []
    i = 0
    while i < len(danh_sach):
        if i < len(danh_sach) - 1 and danh_sach[i] == a and danh_sach[i + 1] == b:
            ra.append(token_moi)
            i += 2
        else:
            ra.append(danh_sach[i])
            i += 1
    return ra

def ma_hoa_van_ban(van_ban, danh_sach_merge, token_sang_id):
    ds = list(van_ban)
    for cap in danh_sach_merge:
        ds = ___                                    # gop_cap(ds, cap)
    return [token_sang_id[t] for t in ___]           # ds

danh_sach_merge = [('n', ' '), ('c', 'o'), ('co', 'n '), (' ', 'a'),
                    (' a', 'n '), ('o', ' an '), (',', ' '), (', ', 'con ')]
token_sang_id = {' ': 0, ' a': 1, ' an ': 2, ',': 3, ', ': 4, ', con ': 5,
                  'a': 6, 'c': 7, 'co': 8, 'con ': 9, 'e': 10, 'g': 11,
                  'h': 12, 'm': 13, 'n': 14, 'n ': 15, 'o': 16,
                  'o an ': 17, 't': 18}

cau_moi = "con ga con an ca"
ids = ma_hoa_van_ban(cau_moi, danh_sach_merge, token_sang_id)

print(len(ids))
print(ids)
```

```python title=solution
def gop_cap(danh_sach, cap):
    a, b = cap
    token_moi = a + b
    ra = []
    i = 0
    while i < len(danh_sach):
        if i < len(danh_sach) - 1 and danh_sach[i] == a and danh_sach[i + 1] == b:
            ra.append(token_moi)
            i += 2
        else:
            ra.append(danh_sach[i])
            i += 1
    return ra

def ma_hoa_van_ban(van_ban, danh_sach_merge, token_sang_id):
    ds = list(van_ban)
    for cap in danh_sach_merge:
        ds = gop_cap(ds, cap)
    return [token_sang_id[t] for t in ds]

danh_sach_merge = [('n', ' '), ('c', 'o'), ('co', 'n '), (' ', 'a'),
                    (' a', 'n '), ('o', ' an '), (',', ' '), (', ', 'con ')]
token_sang_id = {' ': 0, ' a': 1, ' an ': 2, ',': 3, ', ': 4, ', con ': 5,
                  'a': 6, 'c': 7, 'co': 8, 'con ': 9, 'e': 10, 'g': 11,
                  'h': 12, 'm': 13, 'n': 14, 'n ': 15, 'o': 16,
                  'o an ': 17, 't': 18}

cau_moi = "con ga con an ca"
ids = ma_hoa_van_ban(cau_moi, danh_sach_merge, token_sang_id)

print(len(ids))
print(ids)
```

```python title=test
assert len(ids) == 9, f"so token cua cau moi phai la 9 -- dang ra {len(ids)}"
assert ids == [9, 11, 6, 0, 9, 6, 15, 7, 6], f"ma hoa 'con ga con an ca' sai -- dang ra {ids}"

# ap SAI thu tu (dao nguoc danh sach merge) phai cho ra KET QUA KHAC
ids_sai_thu_tu = ma_hoa_van_ban(cau_moi, list(reversed(danh_sach_merge)), token_sang_id)
assert ids_sai_thu_tu != ids, "ap merge theo thu tu SAI (dao nguoc) phai cho ket qua KHAC voi ap dung thu tu -- neu giong het, ham dang khong nhay cam voi thu tu merge"
assert len(ids_sai_thu_tu) == 11, f"ap sai thu tu phai cho 11 token -- dang ra {len(ids_sai_thu_tu)}"

# mot cau khac, ngan hon, de doi chieu doc lap
cau_ngan = "ca"
ids_ngan = ma_hoa_van_ban(cau_ngan, danh_sach_merge, token_sang_id)
assert ids_ngan == [7, 6], f"ma hoa 'ca' phai la [7, 6] (khong co merge nao ap dung duoc len 'ca') -- dang ra {ids_ngan}"

# cau RONG phai tra ve danh sach RONG
assert ma_hoa_van_ban("", danh_sach_merge, token_sang_id) == [], "ma hoa chuoi rong phai tra ve danh sach id rong"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ 1 áp MỘT merge lên danh sách token hiện tại — gọi lại `gop_cap` với `ds` và `cap` của vòng lặp. Chỗ 2 là NGUỒN của list comprehension tra ID — phải là `ds` SAU KHI đã áp xong TOÀN BỘ vòng lặp `for`, không phải `van_ban` gốc (còn là chuỗi ký tự đơn, chưa gộp gì).
- kind: strategy
  body: 'Chỗ 1: `gop_cap(ds, cap)`. Chỗ 2: `ds`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `gop_cap(ds, cap)` và `ds`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: ma_hoa_van_ban phai goi THAT gop_cap(ds, cap) BEN TRONG vong lap for (ap tung merge mot, dung thu tu), VA phai tra ID tren ds SAU KHI da ap xong toan bo vong lap (khong duoc tra ID truc tiep tren van_ban goc)
  requireAst:
  - kind: uses-call, target: gop_cap, min: 1
  - kind: uses-name, target: ds, min: 2
  - kind: uses-name, target: van_ban, min: 1
  - kind: has-literal, target: "con ga con an ca", min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution, dung
  # ast.walk dem chinh xac): loi giai dung dat=true, ca bon luat qua sach.
  # gop_cap=1: dung mot LOI GOI gop_cap(ds, cap) ben trong vong lap (dinh
  # nghia "def gop_cap" khong tinh la mot Call). ds=2: doc mot lan lam tham
  # so cho gop_cap ("gop_cap(ds, cap)"), doc mot lan lam nguon comprehension
  # ("for t in ds") -- ve trai cua phep gan "ds = ..." khong tinh la doc.
  # van_ban=1: doc dung mot lan trong "ds = list(van_ban)".
  #
  # Cheat "cho trong 1 = van_ban" (bo qua vong lap, tra thang van_ban chua
  # gop gi) lam ds tut xuong con 1 (chi con o comprehension, KHONG con o
  # gop_cap) VA van_ban tang len 2 -- bi chan boi ca hai luat; da tu kiem
  # chung: cheat nay cho ket qua id tra ID tren TUNG KY TU DON cua cau goc
  # (16 ky tu -- ma phan lon ky tu don khong co trong token_sang_id vi
  # token_sang_id chi chua CAC TOKEN SAU KHI GOP, vi du " " co nhung "c" don
  # le van co vi trung voi ky tu don gian) -- tren du lieu that se KHONG
  # crash nhung ids dai 16 thay vi 9, bi bat boi tests/output. Cheat "cho
  # trong 2 = van_ban" (tra ID tren chuoi goc thay vi ds da gop) cung bi
  # chan tuong tu boi has-literal cau moi van con nguyen (khong doi) nhung
  # gia tri ids sai hoan toan, bi bat boi assertion ids == [...].
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^9\\n\\[9, 11, 6, 0, 9, 6, 15, 7, 6\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`9` token, ID khớp chính xác — mã hoá một câu MỚI bằng danh sách merge đã
học, đúng thứ tự. Bài sau đi NGƯỢC lại: từ danh sách ID này, khôi phục lại
đúng câu văn gốc.
::::

::::reflect{#nghi-lai}
Mã hoá xong — câu văn giờ là một danh sách số nguyên, sẵn sàng cho
`Value`/`Tensor`. Nhưng một pipeline hoàn chỉnh cần đi được CẢ HAI CHIỀU:
từ số nguyên một mô hình sinh ra, phải khôi phục lại được văn bản con
người đọc được. Bài sau cài đặt chiều ngược lại đó — giải mã.
::::

::::checkpoint{mastery=0.85}
::::
