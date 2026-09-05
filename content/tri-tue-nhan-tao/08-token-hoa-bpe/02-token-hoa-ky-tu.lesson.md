---
id: tri-tue-nhan-tao.token-hoa-bpe.token-hoa-ky-tu
title: "Token hoá theo ký tự: baseline đơn giản nhất"
summary: "Cài xay_vocab_ky_tu/ma_hoa_ky_tu/giai_ma_ky_tu -- token hoá theo TỪNG KÝ TỰ ĐƠN, baseline trước khi có BPE. Corpus 45 ký tự cho vocab CHỈ 11 ký tự duy nhất; mã hoá 'con ga' -> [3, 9, 8, 0, 5, 2], giải mã ngược khớp ĐÚNG nguyên văn. Gotcha: character-level KHÔNG loại bỏ hoàn toàn OOV -- một ký tự THẬT SỰ chưa từng thấy (ví dụ 'è' khi vocab xây từ corpus không dấu) vẫn gây KeyError, chỉ là bề mặt OOV thu hẹp xuống mức bảng chữ cái thay vì mức từ."
locale: vi
track: tri-tue-nhan-tao
module: token-hoa-bpe
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.token-hoa-ky-tu]
requires: [ai.vi-sao-can-token-hoa]
concepts: [ai.token-hoa-ky-tu]
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

::::byte{trigger=enter mood=idle pose=idle}
Bài trước chỉ SO SÁNH hai lựa chọn. Bài này CÀI ĐẶT lựa chọn đơn giản
nhất — token hoá theo ký tự đơn — ba hàm nhỏ: xây từ vựng, mã hoá, giải
mã.
::::

::::explain{#ba_ham_token_hoa_ky_tu}
Token hoá theo ký tự đơn cần đúng ba việc:

> **1. Xây từ vựng** — quét CẢ corpus, gom mọi ký tự DUY NHẤT xuất hiện,
> gán cho mỗi ký tự một số nguyên (token ID). Sắp xếp trước khi gán ID
> (`sorted(set(...))`) để kết quả TẤT ĐỊNH — cùng corpus luôn cho cùng
> bảng ánh xạ, không phụ thuộc thứ tự ngẫu nhiên của `set`.
>
> **2. Mã hoá (encode)** — với một câu bất kỳ, tra cứu ID của TỪNG ký tự
> theo đúng thứ tự xuất hiện, trả về một danh sách số nguyên.
>
> **3. Giải mã (decode)** — làm NGƯỢC LẠI: từ danh sách số nguyên, tra
> cứu lại ký tự tương ứng với mỗi ID (bảng ánh xạ NGƯỢC, ID → ký tự), rồi
> nối tất cả lại thành một chuỗi.

Bảng ánh xạ ngược (`id_sang_tu`) chỉ là bảng xuôi (`tu_sang_id`) đảo
chiều: `id_sang_tu = {i: k for k, i in tu_sang_id.items()}`. Vì mỗi ký tự
có ĐÚNG MỘT ID và mỗi ID có ĐÚNG MỘT ký tự (ánh xạ một-một), phép đảo
chiều này luôn thực hiện được mà không mất thông tin.
::::

::::example{#ma_hoa_giai_ma_ky_tu}
Xây từ vựng từ một corpus `45` ký tự, rồi mã hoá và giải mã một câu ngắn:

```python title=readonly
def xay_vocab_ky_tu(van_ban):
    ky_tu_duy_nhat = sorted(set(van_ban))
    return {k: i for i, k in enumerate(ky_tu_duy_nhat)}

def ma_hoa_ky_tu(van_ban, tu_sang_id):
    return [tu_sang_id[k] for k in van_ban]

def giai_ma_ky_tu(ids, id_sang_tu):
    return ''.join(id_sang_tu[i] for i in ids)

corpus = "con meo an ca, con cho an com, con ga an thoc"
tu_sang_id = xay_vocab_ky_tu(corpus)
id_sang_tu = {i: k for k, i in tu_sang_id.items()}

print("do dai corpus:", len(corpus))
print("so ky tu duy nhat:", len(tu_sang_id))

cau = "con ga"
ids = ma_hoa_ky_tu(cau, tu_sang_id)
print("ma hoa:", ids)

khoi_phuc = giai_ma_ky_tu(ids, id_sang_tu)
print("giai ma:", khoi_phuc)
print("khop voi cau goc:", khoi_phuc == cau)
```

```text title=readonly
do dai corpus: 45
so ky tu duy nhat: 11
ma hoa: [3, 9, 8, 0, 5, 2]
giai ma: con ga
khop voi cau goc: True
```

Corpus dài `45` ký tự nhưng chỉ có `11` ký tự DUY NHẤT — khoảng trắng,
dấu phẩy, và chín chữ cái (`a, c, e, g, h, m, n, o, t`). Câu `"con ga"`
(`6` ký tự) mã hoá thành đúng `6` số nguyên; giải mã cho lại ĐÚNG chuỗi
gốc — round-trip khớp tuyệt đối.
::::

::::predict{#doan_ky_tu_moi commitOnce}
`tu_sang_id` ở trên được xây từ một corpus KHÔNG dấu — `11` ký tự của nó
không hề có ký tự có dấu tiếng Việt nào (`è`, `ề`, `ẽ`, ...).

**Trước khi chạy thử**, bạn đoán: gọi `ma_hoa_ky_tu("con mèo", tu_sang_id)`
— một câu chứa ký tự `'è'`, CHƯA TỪNG xuất hiện trong corpus xây từ vựng —
điều gì xảy ra?

:::opt{correct}
Chương trình dừng với `KeyError` — tra `tu_sang_id['è']` không tìm thấy
khoá đó trong dict, vì `'è'` chưa từng được đưa vào từ vựng lúc xây; token
hoá theo ký tự KHÔNG loại bỏ HOÀN TOÀN vấn đề OOV, nó chỉ thu hẹp bề mặt
OOV xuống mức BẢNG CHỮ CÁI (thay vì mức TỪ) — một ký tự thật sự MỚI vẫn
gây lỗi y hệt một từ mới ở token hoá theo từ
:::

:::opt
Không có lỗi gì — `ma_hoa_ky_tu` tự động thêm `'è'` vào từ vựng và gán cho
nó một ID mới ngay lúc mã hoá
::why
Gần đúng ở việc đây LÀ một hướng xử lý hợp lý mà một hệ thống thật CÓ THỂ
làm (tự mở rộng từ vựng khi gặp ký tự lạ) — nhưng đó là một tính năng phải
được LẬP TRÌNH THÊM, không phải hành vi mặc định.

Chỗ lệch: `ma_hoa_ky_tu` ở đây chỉ làm ĐÚNG MỘT việc — tra cứu
(`tu_sang_id[k]`) — và một phép tra cứu dict với khoá không tồn tại trong
Python luôn ném `KeyError`, không bao giờ tự lặng lẽ thêm khoá mới. Nếu
muốn hành vi "tự mở rộng", phải viết thêm code xử lý riêng cho trường hợp
đó.
::
:::

:::opt
Không có lỗi gì — `ma_hoa_ky_tu` bỏ qua ký tự lạ và chỉ mã hoá những ký tự
đã biết trong câu, trả về danh sách ngắn hơn bình thường
::why
Gần đúng ở việc "bỏ qua ký tự lạ, mã hoá phần còn lại" LÀ một chiến lược
có thật trong nhiều hệ thống token hoá thực tế (thường qua một token đặc
biệt kiểu `<UNK>`).

Chỗ lệch: đó không phải điều đoạn code NÀY làm. `[tu_sang_id[k] for k in
van_ban]` tra cứu TỪNG ký tự không điều kiện — không có nhánh `if k in
tu_sang_id` nào để bỏ qua. Ký tự đầu tiên không tìm thấy sẽ làm dừng NGAY
LẬP TỨC bằng `KeyError`, trước khi kịp xử lý các ký tự còn lại.
::
:::
::::

::::code{#viet_token_hoa_ky_tu}
Hoàn thiện ba chỗ trống: `xay_vocab_ky_tu` (sắp xếp tập ký tự duy nhất
trước khi gán ID), `ma_hoa_ky_tu` (lặp qua đúng `van_ban`), và
`giai_ma_ky_tu` (nối các ký tự đã giải mã thành MỘT chuỗi).

```python title=starter
def xay_vocab_ky_tu(van_ban):
    ky_tu_duy_nhat = sorted(___)                    # set(van_ban)
    return {k: i for i, k in enumerate(ky_tu_duy_nhat)}

def ma_hoa_ky_tu(van_ban, tu_sang_id):
    return [tu_sang_id[k] for k in ___]              # van_ban

def giai_ma_ky_tu(ids, id_sang_tu):
    return ___(id_sang_tu[i] for i in ids)           # ''.join

corpus = "con meo an ca, con cho an com, con ga an thoc"
tu_sang_id = xay_vocab_ky_tu(corpus)
id_sang_tu = {i: k for k, i in tu_sang_id.items()}

cau = "con ga"
ids = ma_hoa_ky_tu(cau, tu_sang_id)
khoi_phuc = giai_ma_ky_tu(ids, id_sang_tu)

print(len(tu_sang_id))
print(ids)
print(khoi_phuc)
```

```python title=solution
def xay_vocab_ky_tu(van_ban):
    ky_tu_duy_nhat = sorted(set(van_ban))
    return {k: i for i, k in enumerate(ky_tu_duy_nhat)}

def ma_hoa_ky_tu(van_ban, tu_sang_id):
    return [tu_sang_id[k] for k in van_ban]

def giai_ma_ky_tu(ids, id_sang_tu):
    return ''.join(id_sang_tu[i] for i in ids)

corpus = "con meo an ca, con cho an com, con ga an thoc"
tu_sang_id = xay_vocab_ky_tu(corpus)
id_sang_tu = {i: k for k, i in tu_sang_id.items()}

cau = "con ga"
ids = ma_hoa_ky_tu(cau, tu_sang_id)
khoi_phuc = giai_ma_ky_tu(ids, id_sang_tu)

print(len(tu_sang_id))
print(ids)
print(khoi_phuc)
```

```python title=test
assert len(tu_sang_id) == 11, f"so ky tu duy nhat phai la 11 -- dang ra {len(tu_sang_id)}"
assert ids == [3, 9, 8, 0, 5, 2], f"ma hoa 'con ga' sai -- dang ra {ids}"
assert khoi_phuc == "con ga", f"giai ma phai khop dung cau goc -- dang ra {khoi_phuc!r}"

# round-trip tren nhieu cau khac, khong chi cau da in san o tren
for c in ["con meo", "an thoc", "ga com", "cho an ca"]:
    ma = ma_hoa_ky_tu(c, tu_sang_id)
    gm = giai_ma_ky_tu(ma, id_sang_tu)
    assert gm == c, f"round-trip that bai voi cau {c!r} -- giai ma lai ra {gm!r}"

# tu_sang_id phai TAT DINH (sap xep) -- xay lai tu cung corpus phai cho
# HET SUC cung mot ket qua
tu_sang_id_2 = xay_vocab_ky_tu(corpus)
assert tu_sang_id == tu_sang_id_2, "xay_vocab_ky_tu phai TAT DINH -- goi lai voi cung corpus phai ra cung mot bang anh xa"
assert list(tu_sang_id.keys()) == sorted(tu_sang_id.keys()), "cac ky tu trong tu_sang_id phai duoc gan ID theo dung THU TU SAP XEP, khong duoc theo thu tu ngau nhien cua set"

# ky tu CHUA TUNG thay phai gay KeyError, khong duoc am tham bo qua/tu them
try:
    ma_hoa_ky_tu("con mèo", tu_sang_id)
    assert False, "ma_hoa_ky_tu voi ky tu 'è' (chua tung thay) phai nem KeyError, khong duoc chay qua trot lot"
except KeyError:
    pass
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ 1 (`xay_vocab_ky_tu`) cần tập hợp các ký tự DUY NHẤT của `van_ban` trước khi sắp xếp — dùng `set(...)`. Chỗ 2 (`ma_hoa_ky_tu`) lặp qua chính chuỗi đầu vào — biến đang có sẵn tên `van_ban`. Chỗ 3 (`giai_ma_ky_tu`) nối một chuỗi các ký tự lại làm MỘT — phương thức `str.join` gọi trên một chuỗi rỗng.
- kind: strategy
  body: 'Chỗ 1: `set(van_ban)`. Chỗ 2: `van_ban`. Chỗ 3: `''.join`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `set(van_ban)`, `van_ban`, và `''.join`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: xay_vocab_ky_tu phai goi THAT set() truoc sorted (khong duoc sap xep truc tiep chuoi co ky tu lap); ma_hoa_ky_tu phai lap dung tren van_ban; giai_ma_ky_tu phai goi THAT ''.join
  requireAst:
  - kind: uses-call, target: set, min: 1
  - kind: uses-call, target: sorted, min: 1
  - kind: uses-name, target: van_ban, min: 2
  - kind: uses-call, target: join, min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat qua sach. set=1: dung dung mot lan trong
  # xay_vocab_ky_tu. sorted=1: bao quanh set(). van_ban=2: mot lan lam
  # tham so cho set(), mot lan trong "for k in van_ban" cua ma_hoa_ky_tu
  # (tham so ham KHONG tinh la "dung ten" -- chi tinh cho doc GIA TRI).
  # join=1: goi ''.join(...) trong giai_ma_ky_tu.
  #
  # Cheat "xay_vocab_ky_tu dung sorted(van_ban) truc tiep, bo set()" van
  # cho ra 11 ky tu DUY NHAT dung boi vi dict comprehension {k: i for i, k
  # in enumerate(...)} tu dong GHI DE cac ky tu lap (chi ID cuoi cung con
  # lai) -- output CO THE giong het, nhung so lan xuat hien "set" tut ve 0,
  # bi chan RIENG boi static (day la ly do static ton tai o day, khong chi
  # dua vao output). Da tu kiem chung: voi corpus nay, ca hai cach cho
  # CUNG mot ket qua tu_sang_id (vi thu tu ky tu lap khong doi ID cuoi), nen
  # chi static moi phan biet duoc.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^11\\n\\[3, 9, 8, 0, 5, 2\\]\\ncon ga\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba hàm nhỏ, round-trip khớp tuyệt đối trên một corpus không dấu. Nhưng
character-level vẫn để lại đúng điểm yếu bài trước đã nêu: chuỗi token dài
— mỗi ký tự đơn lẻ gần như không mang nghĩa gì. Bài sau bắt đầu sửa điều
đó.
::::

::::reflect{#nghi-lai}
Token hoá theo ký tự là baseline ĐƠN GIẢN NHẤT có thể cài — không OOV ở
mức từ, tất định, dễ hiểu. Nhưng nó chưa TẬN DỤNG được thông tin rằng một
số cụm ký tự (như `"con "`, xuất hiện lặp lại nhiều lần trong corpus) luôn
đi cùng nhau — mỗi lần lặp lại vẫn tốn `4` token riêng biệt.

BPE giải quyết đúng chỗ này: ghép những cặp ký tự XUẤT HIỆN THƯỜNG XUYÊN
NHẤT thành các token lớn hơn. Bài sau cài đặt bước LÕI đầu tiên — đếm tần
suất của MỌI cặp ký tự liền kề trong corpus.
::::

::::checkpoint{mastery=0.85}
::::
