---
id: tri-tue-nhan-tao.token-hoa-bpe.giai-ma-nguoc
title: "Giải mã ngược: từ token ID về lại văn bản gốc"
summary: "Cài giai_ma: tra bảng ID-sang-token NGƯỢC rồi nối lại thành chuỗi. Mã hoá 'con ga con an ca' -> [9, 11, 6, 0, 9, 6, 15, 7, 6] -> giải mã lại khớp ĐÚNG nguyên văn. Điểm bất đối xứng thú vị: encode PHỤ THUỘC thứ tự merge (bài trước), nhưng decode KHÔNG -- nối các mảnh theo đúng thứ tự chúng xuất hiện luôn cho lại đúng chuỗi gốc, bất kể thứ tự merge nào đã tạo ra cách phân mảnh đó."
locale: vi
track: tri-tue-nhan-tao
module: token-hoa-bpe
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.giai-ma-nguoc]
requires: [ai.ma-hoa-van-ban-moi]
concepts: [ai.giai-ma-nguoc]
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
Mã hoá xong — câu văn giờ là một danh sách số nguyên. Nhưng một mô hình
ngôn ngữ không chỉ ĐỌC số, nó cũng SINH RA số — và người dùng cần đọc được
CHỮ, không phải số. Bài này đi chiều ngược lại.
::::

::::explain{#giai_ma_nguoc_the_nao}
Giải mã (decode) là phép NGƯỢC của mã hoá: từ một danh sách token ID (số
nguyên), khôi phục lại đúng văn bản gốc. Chỉ cần hai việc:

> **1. Tra ngược** — với mỗi ID, tìm token (chuỗi) tương ứng, qua bảng
> `id_sang_token` (đảo chiều của `token_sang_id` đã xây lúc huấn luyện —
> `{i: t for t, i in token_sang_id.items()}`).
>
> **2. Nối lại** — ghép TẤT CẢ các token đã tra được, theo ĐÚNG thứ tự
> chúng xuất hiện trong danh sách ID, thành MỘT chuỗi duy nhất
> (`''.join(...)`).

Không cần thêm dấu cách hay ký tự phân tách nào giữa các token khi nối —
khoảng trắng (nếu có) đã là MỘT PHẦN của chính token đó (ví dụ token
`'con '` đã bao gồm khoảng trắng ở cuối, từ lúc nó được gộp lúc huấn
luyện). Nối trực tiếp, không chèn gì thêm, là đủ để khôi phục nguyên văn.

Một điểm bất đối xứng đáng chú ý: **mã hoá phụ thuộc thứ tự merge** (bài
trước — áp sai thứ tự cho một cách PHÂN MẢNH khác, dẫn tới danh sách token
khác); nhưng **giải mã thì không** — bất kể danh sách token ĐƯỢC PHÂN MẢNH
như thế nào (miễn các mảnh đó, nối lại theo đúng thứ tự, ĐÚNG BẰNG chuỗi
gốc), phép nối luôn cho lại kết quả đúng. `join` không quan tâm các mảnh
đến từ đâu, nó chỉ ghép chúng lại theo thứ tự đưa vào.
::::

::::example{#giai_ma_that}
Mã hoá `"con ga con an ca"` thành ID (đúng thứ tự merge, bài trước), rồi
giải mã ngược lại:

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

def ma_hoa_van_ban(van_ban, danh_sach_merge, token_sang_id):
    ds = list(van_ban)
    for cap in danh_sach_merge:
        ds = gop_cap(ds, cap)
    return [token_sang_id[t] for t in ds]

def giai_ma(ids, id_sang_token):
    return ''.join(id_sang_token[i] for i in ids)

danh_sach_merge = [('n', ' '), ('c', 'o'), ('co', 'n '), (' ', 'a'),
                    (' a', 'n '), ('o', ' an '), (',', ' '), (', ', 'con ')]
token_sang_id = {' ': 0, ' a': 1, ' an ': 2, ',': 3, ', ': 4, ', con ': 5,
                  'a': 6, 'c': 7, 'co': 8, 'con ': 9, 'e': 10, 'g': 11,
                  'h': 12, 'm': 13, 'n': 14, 'n ': 15, 'o': 16,
                  'o an ': 17, 't': 18}
id_sang_token = {i: t for t, i in token_sang_id.items()}

cau_moi = "con ga con an ca"
ids = ma_hoa_van_ban(cau_moi, danh_sach_merge, token_sang_id)
khoi_phuc = giai_ma(ids, id_sang_token)

print("ids:", ids)
print("khoi phuc:", khoi_phuc)
print("khop voi cau goc:", khoi_phuc == cau_moi)
```

```text title=readonly
ids: [9, 11, 6, 0, 9, 6, 15, 7, 6]
khoi phuc: con ga con an ca
khop voi cau goc: True
```

`9` token ID, tra ngược và nối lại, khớp ĐÚNG TUYỆT ĐỐI với câu gốc —
round-trip encode → decode bảo toàn nguyên văn.
::::

::::predict{#doan_giai_ma_sai_thu_tu commitOnce}
Bài trước đã đo: mã hoá `"con ga con an ca"` theo thứ tự merge SAI (đảo
ngược) cho `11` token ID — KHÁC với `9` token ID của thứ tự đúng.

**Trước khi chạy thử**, bạn đoán: giải mã danh sách `11` ID đó (từ thứ tự
merge SAI) bằng đúng hàm `giai_ma` ở trên — có cho lại đúng câu gốc `"con
ga con an ca"` không?

:::opt{correct}
Có — dù cách PHÂN MẢNH thành token khác nhau (`9` token so với `11`
token), MỖI token trong cả hai cách đều là một mảnh CON của câu gốc, và
nối chúng lại theo đúng thứ tự luôn tái tạo lại chuỗi ban đầu; `giai_ma`
chỉ nối chuỗi, nó không quan tâm các mảnh đó được TẠO RA bằng thứ tự merge
nào
:::

:::opt
Không — vì danh sách `11` ID này khác hẳn danh sách `9` ID đúng, `giai_ma`
sẽ tra nhầm token và cho ra một chuỗi khác hoặc gây lỗi
::why
Gần đúng ở việc `11` ID và `9` ID đúng là hai danh sách HOÀN TOÀN khác
nhau về số lượng lẫn giá trị — quan sát đó không sai.

Chỗ lệch: `giai_ma` không so sánh danh sách ID với "danh sách đúng" nào cả
— nó chỉ tra TỪNG ID trong CHÍNH danh sách được đưa vào, rồi nối lại. Vì
mỗi ID trong danh sách `11` phần tử VẪN tra ra một token hợp lệ (một mảnh
con thật sự của câu gốc, dù chia nhỏ hơn), nối chúng lại theo đúng thứ tự
vẫn cho ra đúng câu gốc — không có gì để "tra nhầm" cả.
::
:::

:::opt
Có, nhưng chỉ vì đây là một sự trùng hợp NGẪU NHIÊN của riêng câu
`"con ga con an ca"` này — với một câu khác, giải mã một chuỗi ID từ thứ
tự merge sai có thể không khớp
::why
Gần đúng ở việc nghi ngờ một kết quả "có vẻ đúng" liệu có phải trùng hợp
ngẫu nhiên — thái độ hoài nghi đó nói chung là tốt.

Chỗ lệch: đây KHÔNG phải trùng hợp của riêng câu này — nó là một tính chất
CẤU TRÚC của phép `join`. Bất kể thứ tự merge nào tạo ra cách phân mảnh
nào, mỗi token luôn là một chuỗi con LIỀN MẠCH của văn bản gốc, và ghép
mọi mảnh liền mạch theo đúng thứ tự luôn tái tạo lại đúng chuỗi ban đầu —
đây là tính chất TOÁN HỌC của phép nối chuỗi, đúng với MỌI câu, không phải
đặc thù của câu này.
::
:::
::::

::::code{#viet_giai_ma}
Hoàn thiện `giai_ma`: xây bảng tra ngược từ `token_sang_id`, rồi tra và
nối các token theo đúng thứ tự ID được đưa vào.

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
        ds = gop_cap(ds, cap)
    return [token_sang_id[t] for t in ds]

def xay_bang_nguoc(token_sang_id):
    return {i: t for t, i in ___.items()}          # token_sang_id

def giai_ma(ids, id_sang_token):
    return ___(id_sang_token[i] for i in ids)        # ''.join

danh_sach_merge = [('n', ' '), ('c', 'o'), ('co', 'n '), (' ', 'a'),
                    (' a', 'n '), ('o', ' an '), (',', ' '), (', ', 'con ')]
token_sang_id = {' ': 0, ' a': 1, ' an ': 2, ',': 3, ', ': 4, ', con ': 5,
                  'a': 6, 'c': 7, 'co': 8, 'con ': 9, 'e': 10, 'g': 11,
                  'h': 12, 'm': 13, 'n': 14, 'n ': 15, 'o': 16,
                  'o an ': 17, 't': 18}
id_sang_token = xay_bang_nguoc(token_sang_id)

cau_moi = "con ga con an ca"
ids = ma_hoa_van_ban(cau_moi, danh_sach_merge, token_sang_id)
khoi_phuc = giai_ma(ids, id_sang_token)

print(ids)
print(khoi_phuc)
print(khoi_phuc == cau_moi)
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

def xay_bang_nguoc(token_sang_id):
    return {i: t for t, i in token_sang_id.items()}

def giai_ma(ids, id_sang_token):
    return ''.join(id_sang_token[i] for i in ids)

danh_sach_merge = [('n', ' '), ('c', 'o'), ('co', 'n '), (' ', 'a'),
                    (' a', 'n '), ('o', ' an '), (',', ' '), (', ', 'con ')]
token_sang_id = {' ': 0, ' a': 1, ' an ': 2, ',': 3, ', ': 4, ', con ': 5,
                  'a': 6, 'c': 7, 'co': 8, 'con ': 9, 'e': 10, 'g': 11,
                  'h': 12, 'm': 13, 'n': 14, 'n ': 15, 'o': 16,
                  'o an ': 17, 't': 18}
id_sang_token = xay_bang_nguoc(token_sang_id)

cau_moi = "con ga con an ca"
ids = ma_hoa_van_ban(cau_moi, danh_sach_merge, token_sang_id)
khoi_phuc = giai_ma(ids, id_sang_token)

print(ids)
print(khoi_phuc)
print(khoi_phuc == cau_moi)
```

```python title=test
assert khoi_phuc == "con ga con an ca", f"giai ma phai khop DUNG cau goc -- dang ra {khoi_phuc!r}"
assert khoi_phuc == cau_moi, "giai ma phai bang chinh xac cau_moi"

# giai ma tren danh sach ID tu ma hoa SAI thu tu (bai truoc) VAN phai khop
ids_sai_thu_tu = ma_hoa_van_ban(cau_moi, list(reversed(danh_sach_merge)), token_sang_id)
khoi_phuc_tu_sai = giai_ma(ids_sai_thu_tu, id_sang_token)
assert khoi_phuc_tu_sai == cau_moi, f"giai ma tu ids (ma hoa sai thu tu) van phai khop cau goc -- dang ra {khoi_phuc_tu_sai!r}"
assert len(ids_sai_thu_tu) == 11, f"kiem tra phu: ids sai thu tu phai co 11 phan tu -- dang ra {len(ids_sai_thu_tu)}"

# round-trip tren nhieu cau khac
for c in ["ga com", "con cho", "thoc"]:
    idsx = ma_hoa_van_ban(c, danh_sach_merge, token_sang_id)
    kpx = giai_ma(idsx, id_sang_token)
    assert kpx == c, f"round-trip that bai voi cau {c!r} -- giai ma lai ra {kpx!r}"

# danh sach ID rong phai giai ma ra chuoi rong
assert giai_ma([], id_sang_token) == "", "giai ma danh sach ID rong phai tra ve chuoi rong"

# bang nguoc phai la NGHICH DAO chinh xac: moi cap (token, id) phai anh xa
# hai chieu dung nhau
for t, i in token_sang_id.items():
    assert id_sang_token[i] == t, f"bang nguoc sai o token {t!r} (id={i})"
```

:::hints
- kind: attention
  body: Hai chỗ trống. `xay_bang_nguoc` cần đảo NGƯỢC bảng `token_sang_id` — với mỗi cặp `(t, i)` trong `token_sang_id.items()`, tạo cặp NGƯỢC `(i, t)`. `giai_ma` cần nối một chuỗi các mảnh token thành MỘT — dùng `str.join` trên một chuỗi rỗng.
- kind: strategy
  body: 'Chỗ 1: `token_sang_id.items()` (lặp qua để đảo chiều). Chỗ 2: `''.join`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `token_sang_id` (cho `token_sang_id.items()`) và `''.join`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: xay_bang_nguoc phai goi THAT token_sang_id.items() de dao chieu bang anh xa; giai_ma phai goi THAT ''.join de noi cac token lai thanh mot chuoi
  requireAst:
  - kind: uses-call, target: items, min: 1
  - kind: uses-call, target: join, min: 1
  - kind: uses-name, target: token_sang_id, min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca ba luat qua sach. items=1: dung mot lan trong
  # xay_bang_nguoc. join=1: dung mot lan trong giai_ma. token_sang_id=2:
  # mot lan lam THAM SO ham (khong tinh, tham so khong phai doc gia tri),
  # sua lai -- doc THAT: mot lan trong ".items()" (doc thuoc tinh cua bien
  # token_sang_id) cong mot lan luc goi xay_bang_nguoc(token_sang_id) o
  # script muc top-level -- tong la 2 lan doc.
  #
  # Cheat "id_sang_token = {} " (tra ve bang RONG, bo qua dao chieu) lam
  # items tut ve 0 -- bi chan; da tu kiem chung: cheat nay lam giai_ma nem
  # KeyError ngay lap tuc (tra ID khong ton tai trong dict rong). Cheat
  # "giai_ma tra ve list thay vi join" (vi du "return [id_sang_token[i] for
  # i in ids]", quen join) lam join tut ve 0 -- bi chan; da tu kiem chung:
  # cheat nay lam khoi_phuc la MOT DANH SACH cac chuoi con, so sanh
  # "== cau_moi" (mot chuoi) luon False, bi bat DOC LAP boi tests.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^\\[9, 11, 6, 0, 9, 6, 15, 7, 6\\]\\ncon ga con an ca\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Round-trip hoàn chỉnh — mã hoá rồi giải mã, khớp đúng nguyên văn, kể cả
khi thử với thứ tự merge sai. Bài BOSS cuối quest ráp TOÀN BỘ pipeline lại
làm một.
::::

::::reflect{#nghi-lai}
Bảy bài, bảy mảnh: vì sao cần token hoá, token hoá theo ký tự, đếm cặp,
gộp cặp, lặp tới vocab mục tiêu, mã hoá câu mới, giải mã ngược. Mỗi mảnh
đã được đo bằng Python thật, không suy luận tay.

Bài BOSS ráp TẤT CẢ lại: huấn luyện BPE trên một corpus LỚN hơn (vài trăm
ký tự), mã hoá một câu mới, giải mã lại, và đo con số mà cả quest này tồn
tại để chứng minh — chuỗi token của BPE so với chuỗi token nếu chỉ dùng
character-level (bài `token-hoa-ky-tu`), trên CÙNG một câu.
::::

::::checkpoint{mastery=0.85}
::::
