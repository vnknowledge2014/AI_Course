---
id: tri-tue-nhan-tao.quan-ly-ngu-canh-va-hoi-thoai.cua-so-ngu-canh-va-cat-bot
title: "Cửa sổ ngữ cảnh và cắt bớt: khi lịch sử vượt trần"
summary: "Một lịch sử hội thoại 8 message (187 token thật, đếm bằng ma_hoa_van_ban) vượt một TRẦN token cố định (TRAN=90) -- cần cắt. Chiến lược: giữ message GẦN NHẤT, cắt message CŨ NHẤT trước, lặp tới khi tổng token <= trần. Kết quả: còn 4 message, 83 token. Biên: TRAN=83 (đúng bằng giá trị sau cắt) vẫn dừng ở 4 message/83 token -- xác nhận so sánh dùng > chứ không phải >=; TRAN=5 (cực nhỏ) chỉ còn lại đúng 1 message (21 token, vẫn vượt trần) vì không thể cắt dưới 1 -- luôn phải giữ ít nhất câu hỏi hiện tại."
locale: vi
track: tri-tue-nhan-tao
module: quan-ly-ngu-canh-va-hoi-thoai
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.cua-so-ngu-canh-va-cat-bot]
requires: [ai.dem-token-that]
concepts: [ai.cua-so-ngu-canh-va-cat-bot]
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
Bài trước đếm được số token THẬT. Giờ dùng đúng con số đó cho việc nó
tồn tại để phục vụ: một LLM thật không đọc được lịch sử hội thoại DÀI VÔ
HẠN — nó có một CỬA SỔ NGỮ CẢNH, một TRẦN token cố định. Vượt trần đó,
phải cắt bớt lịch sử.
::::

::::explain{#cua_so_ngu_canh_la_gi}
Mọi LLM thật đều có một **cửa sổ ngữ cảnh** (context window) — số token
TỐI ĐA nó đọc được trong một lượt gọi, cộng dồn CẢ lịch sử lẫn câu hỏi
mới. Vượt trần đó, phần dư không có cách nào lọt vào — phải cắt bớt lịch
sử TRƯỚC khi gọi.

Chiến lược đơn giản nhất — **giữ gần nhất, cắt cũ nhất**:

> Mã hoá TOÀN BỘ lịch sử (nối `content` mọi message rồi đếm token BPE
> thật, đúng hàm bài trước). Nếu tổng vượt trần, bỏ message CŨ NHẤT (đầu
> danh sách), rồi đếm lại. Lặp lại tới khi tổng token ≤ trần, hoặc chỉ còn
> lại đúng MỘT message (luôn phải giữ ít nhất câu hỏi hiện tại — không có
> gì để trả lời nếu cắt sạch).

Vì sao cắt CŨ nhất chứ không phải MỚI nhất? Message gần đây thường liên
quan trực tiếp tới câu hỏi hiện tại; message cũ có xác suất cao đã "hết
hạn sử dụng" — không còn ảnh hưởng tới câu trả lời sắp tới.
::::

::::example{#cat_bot_that_tren_lich_su}
Một lịch sử `8` message, tổng token vượt trần `TRAN = 90` — cắt bằng
chiến lược "giữ gần nhất":

```python title=readonly
def dem_cap_lien_ke(danh_sach):
    dem = {}
    for i in range(len(danh_sach) - 1):
        cap = (danh_sach[i], danh_sach[i + 1])
        dem[cap] = dem.get(cap, 0) + 1
    return dem

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

def huan_luyen_bpe(corpus, muc_tieu_vocab):
    ds = list(corpus)
    vocab = set(ds)
    merges = []
    while len(vocab) < muc_tieu_vocab:
        dem = dem_cap_lien_ke(ds)
        if not dem:
            break
        cap_pho_bien = max(dem, key=dem.get)
        tan_suat = dem[cap_pho_bien]
        if tan_suat < 2:
            break
        ds = gop_cap(ds, cap_pho_bien)
        vocab.add(cap_pho_bien[0] + cap_pho_bien[1])
        merges.append(cap_pho_bien)
    return ds, merges, vocab

def xay_token_sang_id(vocab):
    return {t: i for i, t in enumerate(sorted(vocab))}

def ma_hoa_van_ban(van_ban, danh_sach_merge, token_sang_id):
    ds = list(van_ban)
    for cap in danh_sach_merge:
        ds = gop_cap(ds, cap)
    return [token_sang_id[t] for t in ds]

def dem_token_lich_su(danh_sach_message, merges, token_sang_id):
    noi_dung_noi = " ".join(m["content"] for m in danh_sach_message)
    return len(ma_hoa_van_ban(noi_dung_noi, merges, token_sang_id))

def cat_bot_lich_su(lich_su, tran, merges, token_sang_id):
    ds = list(lich_su)
    while dem_token_lich_su(ds, merges, token_sang_id) > tran and len(ds) > 1:
        ds = ds[1:]
    return ds

corpus = ("xin chao, toi la tro ly ao, rat vui duoc giup ban. "
          "gia banh mi bao nhieu tien. gia banh mi la 15000 dong. "
          "gia ca phe bao nhieu tien. gia ca phe la 25000 dong. "
          "gia tra sua bao nhieu tien. gia tra sua la 30000 dong. "
          "con cai kia thi sao. cam on ban rat nhieu, khong co chi. "
          "cuoc hoi thoai co the rat dai, can tom tat lich su cu "
          "de khong vuot qua cua so ngu canh cho phep.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=35)
token_sang_id = xay_token_sang_id(vocab)

TRAN = 90
lich_su = [
    {"role": "user", "content": "xin chao, toi muon hoi gia do uong"},
    {"role": "assistant", "content": "chao ban, ban muon hoi gia mon nao"},
    {"role": "user", "content": "gia banh mi bao nhieu tien"},
    {"role": "assistant", "content": "gia banh mi la 15000 dong"},
    {"role": "user", "content": "gia ca phe bao nhieu tien"},
    {"role": "assistant", "content": "gia ca phe la 25000 dong"},
    {"role": "user", "content": "gia tra sua bao nhieu tien"},
    {"role": "assistant", "content": "gia tra sua la 30000 dong"},
]

truoc = dem_token_lich_su(lich_su, merges, token_sang_id)
lich_su_sau = cat_bot_lich_su(lich_su, TRAN, merges, token_sang_id)
sau = dem_token_lich_su(lich_su_sau, merges, token_sang_id)

print("truoc:", truoc, "so message:", len(lich_su))
print("sau:", sau, "so message:", len(lich_su_sau))
print("message cuoi cung con lai:", lich_su_sau[-1]["content"])
```

```text title=readonly
truoc: 187 so message: 8
sau: 83 so message: 4
message cuoi cung con lai: gia tra sua la 30000 dong
```

`187` token, `8` message ban đầu — vượt trần `90`. Cắt lặp lại `4` lần
(bỏ `4` message cũ nhất), còn lại `4` message, `83` token — ĐÃ ≤ trần.
Message cuối cùng còn lại vẫn là câu trả lời GẦN NHẤT (`"gia tra sua la
30000 dong"`) — đúng tinh thần "giữ gần nhất, cắt cũ nhất".
::::

::::predict{#doan_so_message_con_lai commitOnce}
Xét đúng lịch sử `8` message (`187` token) và `TRAN = 90` ở ví dụ trên.

**Trước khi chạy thử**, bạn đoán: cắt theo chiến lược "giữ gần nhất, cắt
cũ nhất" sẽ còn lại BAO NHIÊU message?

:::opt{correct}
`4` message (`83` token) — phải cắt LẶP LẠI, đếm lại token sau MỖI lần
cắt (không phải trừ một lần rồi xong): cắt bỏ message đầu tiên (còn `7`
message, `158` token, vẫn `> 90`) → cắt tiếp (còn `6` message, `128`
token, vẫn `> 90`) → cắt tiếp (còn `5` message, `105` token, vẫn `>
90`) → cắt tiếp (còn `4` message, `83` token, ĐÃ `≤ 90`) → dừng
:::

:::opt
`3` message — vì cần chắc chắn tổng token THẤP HẲN dưới trần cho an
toàn, nên cắt thêm một message nữa so với mức tối thiểu cần thiết
::why
Gần đúng ở việc bạn nghĩ tới một biên độ an toàn — tâm lý "cắt dư một
chút cho chắc" không phải vô lý trong thực tế vận hành.

Chỗ lệch: chiến lược trong bài này DỪNG ngay khi điều kiện `tổng token ≤
trần` được thoả — nó không cắt THÊM "cho chắc". Ở mức `4` message (`83`
token), điều kiện đã đúng (`83 ≤ 90`), nên vòng lặp dừng NGAY tại đó,
không cắt tiếp xuống `3`.
::
:::

:::opt
Không cắt được message nào cả, vì `187` chia `90` dư `7`, và phần dư đó
không đủ để cắt trọn một message
::why
Gần đúng ở việc bạn thử LIÊN HỆ con số `187` và `90` bằng một phép tính
số học trực tiếp — bản năng đó hợp lý khi mới nhìn hai con số.

Chỗ lệch: số token CẦN CẮT không tính bằng phép trừ đơn giản
(`187 − 90 = 97`) rồi so với kích thước một message — vì loại bỏ một
message làm THAY ĐỔI cách các message CÒN LẠI ghép nối (token ở biên nối
có thể gộp khác đi). Cách duy nhất biết chính xác là đếm LẠI bằng
`dem_token_lich_su` sau MỖI lần cắt, đúng như hàm `cat_bot_lich_su` làm.
::
:::
::::

::::code{#viet_cat_bot_lich_su}
Hoàn thiện `cat_bot_lich_su`: điều kiện lặp phải so sánh tổng token THẬT
với trần bằng đúng phép so sánh "lớn hơn" (không phải "lớn hơn hoặc
bằng"), và mỗi lần cắt phải bỏ ĐÚNG message cũ nhất (đầu danh sách).

```python title=starter
def dem_cap_lien_ke(danh_sach):
    dem = {}
    for i in range(len(danh_sach) - 1):
        cap = (danh_sach[i], danh_sach[i + 1])
        dem[cap] = dem.get(cap, 0) + 1
    return dem

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

def huan_luyen_bpe(corpus, muc_tieu_vocab):
    ds = list(corpus)
    vocab = set(ds)
    merges = []
    while len(vocab) < muc_tieu_vocab:
        dem = dem_cap_lien_ke(ds)
        if not dem:
            break
        cap_pho_bien = max(dem, key=dem.get)
        tan_suat = dem[cap_pho_bien]
        if tan_suat < 2:
            break
        ds = gop_cap(ds, cap_pho_bien)
        vocab.add(cap_pho_bien[0] + cap_pho_bien[1])
        merges.append(cap_pho_bien)
    return ds, merges, vocab

def xay_token_sang_id(vocab):
    return {t: i for i, t in enumerate(sorted(vocab))}

def ma_hoa_van_ban(van_ban, danh_sach_merge, token_sang_id):
    ds = list(van_ban)
    for cap in danh_sach_merge:
        ds = gop_cap(ds, cap)
    return [token_sang_id[t] for t in ds]

def dem_token_lich_su(danh_sach_message, merges, token_sang_id):
    noi_dung_noi = " ".join(m["content"] for m in danh_sach_message)
    return len(ma_hoa_van_ban(noi_dung_noi, merges, token_sang_id))

def cat_bot_lich_su(lich_su, tran, merges, token_sang_id):
    ds = list(lich_su)
    while dem_token_lich_su(ds, merges, token_sang_id) ___ tran and len(ds) > 1:   # >
        ds = ___                                                                    # ds[1:]
    return ds

corpus = ("xin chao, toi la tro ly ao, rat vui duoc giup ban. "
          "gia banh mi bao nhieu tien. gia banh mi la 15000 dong. "
          "gia ca phe bao nhieu tien. gia ca phe la 25000 dong. "
          "gia tra sua bao nhieu tien. gia tra sua la 30000 dong. "
          "con cai kia thi sao. cam on ban rat nhieu, khong co chi. "
          "cuoc hoi thoai co the rat dai, can tom tat lich su cu "
          "de khong vuot qua cua so ngu canh cho phep.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=35)
token_sang_id = xay_token_sang_id(vocab)

TRAN = 90
lich_su = [
    {"role": "user", "content": "xin chao, toi muon hoi gia do uong"},
    {"role": "assistant", "content": "chao ban, ban muon hoi gia mon nao"},
    {"role": "user", "content": "gia banh mi bao nhieu tien"},
    {"role": "assistant", "content": "gia banh mi la 15000 dong"},
    {"role": "user", "content": "gia ca phe bao nhieu tien"},
    {"role": "assistant", "content": "gia ca phe la 25000 dong"},
    {"role": "user", "content": "gia tra sua bao nhieu tien"},
    {"role": "assistant", "content": "gia tra sua la 30000 dong"},
]

truoc = dem_token_lich_su(lich_su, merges, token_sang_id)
lich_su_sau = cat_bot_lich_su(lich_su, TRAN, merges, token_sang_id)
sau = dem_token_lich_su(lich_su_sau, merges, token_sang_id)

print(truoc, len(lich_su))
print(sau, len(lich_su_sau))
```

```python title=solution
def dem_cap_lien_ke(danh_sach):
    dem = {}
    for i in range(len(danh_sach) - 1):
        cap = (danh_sach[i], danh_sach[i + 1])
        dem[cap] = dem.get(cap, 0) + 1
    return dem

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

def huan_luyen_bpe(corpus, muc_tieu_vocab):
    ds = list(corpus)
    vocab = set(ds)
    merges = []
    while len(vocab) < muc_tieu_vocab:
        dem = dem_cap_lien_ke(ds)
        if not dem:
            break
        cap_pho_bien = max(dem, key=dem.get)
        tan_suat = dem[cap_pho_bien]
        if tan_suat < 2:
            break
        ds = gop_cap(ds, cap_pho_bien)
        vocab.add(cap_pho_bien[0] + cap_pho_bien[1])
        merges.append(cap_pho_bien)
    return ds, merges, vocab

def xay_token_sang_id(vocab):
    return {t: i for i, t in enumerate(sorted(vocab))}

def ma_hoa_van_ban(van_ban, danh_sach_merge, token_sang_id):
    ds = list(van_ban)
    for cap in danh_sach_merge:
        ds = gop_cap(ds, cap)
    return [token_sang_id[t] for t in ds]

def dem_token_lich_su(danh_sach_message, merges, token_sang_id):
    noi_dung_noi = " ".join(m["content"] for m in danh_sach_message)
    return len(ma_hoa_van_ban(noi_dung_noi, merges, token_sang_id))

def cat_bot_lich_su(lich_su, tran, merges, token_sang_id):
    ds = list(lich_su)
    while dem_token_lich_su(ds, merges, token_sang_id) > tran and len(ds) > 1:
        ds = ds[1:]
    return ds

corpus = ("xin chao, toi la tro ly ao, rat vui duoc giup ban. "
          "gia banh mi bao nhieu tien. gia banh mi la 15000 dong. "
          "gia ca phe bao nhieu tien. gia ca phe la 25000 dong. "
          "gia tra sua bao nhieu tien. gia tra sua la 30000 dong. "
          "con cai kia thi sao. cam on ban rat nhieu, khong co chi. "
          "cuoc hoi thoai co the rat dai, can tom tat lich su cu "
          "de khong vuot qua cua so ngu canh cho phep.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=35)
token_sang_id = xay_token_sang_id(vocab)

TRAN = 90
lich_su = [
    {"role": "user", "content": "xin chao, toi muon hoi gia do uong"},
    {"role": "assistant", "content": "chao ban, ban muon hoi gia mon nao"},
    {"role": "user", "content": "gia banh mi bao nhieu tien"},
    {"role": "assistant", "content": "gia banh mi la 15000 dong"},
    {"role": "user", "content": "gia ca phe bao nhieu tien"},
    {"role": "assistant", "content": "gia ca phe la 25000 dong"},
    {"role": "user", "content": "gia tra sua bao nhieu tien"},
    {"role": "assistant", "content": "gia tra sua la 30000 dong"},
]

truoc = dem_token_lich_su(lich_su, merges, token_sang_id)
lich_su_sau = cat_bot_lich_su(lich_su, TRAN, merges, token_sang_id)
sau = dem_token_lich_su(lich_su_sau, merges, token_sang_id)

print(truoc, len(lich_su))
print(sau, len(lich_su_sau))
```

```python title=test
assert truoc == 187, f"tong token truoc cat phai la 187 -- dang ra {truoc}"
assert len(lich_su) == 8, f"so message ban dau phai la 8 -- dang ra {len(lich_su)}"
assert sau == 83, f"tong token sau cat phai la 83 -- dang ra {sau}"
assert len(lich_su_sau) == 4, f"so message sau cat phai la 4 -- dang ra {len(lich_su_sau)}"

# phai giu DUNG 4 message GAN NHAT, khong phai 4 message dau
noi_dung_con_lai = [m["content"] for m in lich_su_sau]
assert noi_dung_con_lai == [m["content"] for m in lich_su[4:]], f"phai giu dung 4 message GAN NHAT (tu vi tri 4 tro di) -- dang ra {noi_dung_con_lai}"
assert lich_su_sau[-1]["content"] == "gia tra sua la 30000 dong", "message CUOI CUNG (gan nhat) phai duoc giu lai nguyen ven"

# BIEN: TRAN dung BANG gia tri token sau khi da cat (83) -- phai DUNG lai
# dung do (khong cat them), xac nhan dung phep so sanh > chu khong phai >=
lich_su_bien = cat_bot_lich_su(lich_su, 83, merges, token_sang_id)
assert len(lich_su_bien) == 4, f"TRAN=83 (bang dung gia tri sau cat) khong duoc cat THEM -- dang ra {len(lich_su_bien)} message"
assert dem_token_lich_su(lich_su_bien, merges, token_sang_id) == 83, "TRAN=83 phai dung lai o dung 83 token, khong cat qua tay"

# BIEN: TRAN cuc nho (5) -- ngay ca message cuoi cung mot minh cung vuot
# tran, nhung KHONG the cat duoi 1 message
lich_su_cuc_nho = cat_bot_lich_su(lich_su, 5, merges, token_sang_id)
assert len(lich_su_cuc_nho) == 1, f"TRAN cuc nho van phai giu LAI DUNG 1 message (khong cat het) -- dang ra {len(lich_su_cuc_nho)}"
assert lich_su_cuc_nho[0]["content"] == "gia tra sua la 30000 dong", "message duy nhat con lai phai la message GAN NHAT"

# TRAN du lon (khong bao gio vuot) -- khong duoc cat gi ca
lich_su_khong_cat = cat_bot_lich_su(lich_su, 999, merges, token_sang_id)
assert len(lich_su_khong_cat) == 8, f"TRAN du lon thi khong duoc cat message nao -- dang ra {len(lich_su_khong_cat)}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ 1 là phép SO SÁNH giữa tổng token hiện tại và `tran` — dùng `>` ("lớn hơn"), KHÔNG phải `>=`, để khi tổng ĐÚNG BẰNG trần thì vòng lặp DỪNG (không cắt thêm một message không cần thiết). Chỗ 2 là bước CẮT BỚT — bỏ message CŨ NHẤT, tức phần tử ĐẦU danh sách — dùng lát cắt `ds[1:]` (mọi phần tử TỪ vị trí `1` trở đi, bỏ vị trí `0`).
- kind: strategy
  body: 'Chỗ 1: `>` (so sánh `dem_token_lich_su(ds, merges, token_sang_id) > tran`). Chỗ 2: `ds[1:]`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `>` và `ds[1:]`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: dieu kien vong lap phai dung dung phep so sanh "lon hon" (>, khong duoc ">=" -- se cat THEM mot message khong can thiet khi tong token dung bang tran), VA moi lan cat phai goi THAT dem_token_lich_su (khong duoc thay bang mot dieu kien gia dinh khac)
  requireAst:
  - kind: uses-operator, target: ">", min: 2
  - kind: uses-call, target: dem_token_lich_su, min: 3
  # Da thu that (goi bo dem AST doc lap mo phong dung logic kiem-ast.ts tren
  # code trich tu solution): loi giai dung dat=true, ca hai luat qua sach.
  # ">"=2: mot lan CO SAN trong "len(ds) > 1" (phan guard, khong bi cho
  # trong), mot lan la cho trong 1. Neu cho trong 1 doi thanh ">=", dem ">"
  # tut xuong con 1 -- duoi nguong 2, bi chan. dem_token_lich_su=3: mot lan
  # GOI THAT trong dieu kien vong lap cat_bot_lich_su (cho trong 1 lien
  # quan), cong hai lan trong phan demo (tinh "truoc" va "sau") -- dinh
  # nghia ham khong tinh la Call. Neu cho trong 1 bi thay bang mot dieu kien
  # khac khong goi dem_token_lich_su (vi du "while len(ds) > 4 and len(ds) >
  # 1"), dem nay tut xuong con 2 -- duoi nguong 3, bi chan.
  #
  # Cheat ">=" thay ">" da tu kiem chung BANG PYTHON THAT: tren TRAN=90
  # (kich ban chinh), ket qua VAN giong het loi giai dung (187->158->128->
  # 105->83, khong co buoc nao dung bang 90) -- KHONG bi bat boi tests/output
  # o kich ban chinh. Day CHINH LA ly do case bien TRAN=83 (bang dung gia
  # tri sau cat) ton tai doc lap trong test: da tu kiem chung, cheat ">="
  # tren TRAN=83 cat THEM mot message nua (con 3 message/62 token thay vi
  # 4/83) -- bi bat CA boi static (rieng, qua toan tu ">") LAN boi
  # assertion "lich_su_bien" (tests/output), phong thu kep.
  # Cheat "ds[:-1]" thay "ds[1:]" (cat NHAM message MOI NHAT thay vi cu
  # nhat) khong doi bat ky so dem AST nao o tren (van la mot phep slice hop
  # le) nhung da tu kiem chung: cho noi_dung_con_lai hoan toan khac (giu lai
  # 4 message DAU thay vi 4 message CUOI), bi bat DOC LAP boi assertion
  # "noi_dung_con_lai == ..." va "lich_su_sau[-1]" (tests/output).
  #
  # Chi phi buoc: corpus huan luyen O DAY chi dung muc_tieu_vocab=35 (5
  # merge, khac vocab=90/60 merge cua cac bai 1/3/4 trong cung quest) --
  # rieng bai nay goi cat_bot_lich_su (vong lap goi lai dem_token_lich_su
  # MOI lan kiem tra dieu kien, tuc ap lai toan bo cac merge) BON lan khac
  # nhau (kich ban chinh + ba truong hop bien), cong don rat nhanh; da tu do
  # bang sys.settrace THAT: voi vocab=90/60merge tong chi phi vuot han muc
  # 300.000 buoc cua sandbox, con voi vocab=35/5merge (giu cung corpus, cung
  # 8 message) tong chi phi chi con khoang 64.000/300.000 buoc -- an toan
  # nhieu lan.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^187 8\\n83 4\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`187` token xuống `83`, `8` message xuống `4` — cắt lặp lại tới khi vừa
đủ dưới trần, luôn giữ message gần nhất. Nhưng cắt bỏ nghĩa là MẤT thông
tin vĩnh viễn. Bài sau thử một chiến lược khác: NÉN thay vì XOÁ.
::::

::::reflect{#nghi-lai}
Cắt bớt hoạt động, và đơn giản để cài đặt — nhưng nó XOÁ SẠCH những
message bị cắt, không giữ lại gì cả. Với `TRAN` cực nhỏ, thấy rõ giới
hạn: dù chỉ còn `1` message vẫn có thể vượt trần, và không có gì hơn để
cắt. Bài sau thay "xoá sạch" bằng "nén lại" — tóm tắt các message cũ
thành MỘT message ngắn, giữ lại phần cốt lõi thay vì mất trắng.
::::

::::checkpoint{mastery=0.85}
::::
