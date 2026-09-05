---
id: tri-tue-nhan-tao.quan-ly-ngu-canh-va-hoi-thoai.bo-nho-hoi-thoai-nhieu-luot
title: "Bộ nhớ hội thoại nhiều lượt: lượt sau nhớ được lượt trước"
summary: "Một hàm hoi_va_cap_nhat tích luỹ CẢ câu hỏi lẫn câu trả lời (qua LLM mô phỏng llm_mo_phong_hoi_thoai) vào lich_su xuyên nhiều lượt. Lượt 1 hỏi giá banh mi (15000 dong). Lượt 2 hỏi mơ hồ 'con cai kia thi sao' -- CẦN đọc lại lịch sử lượt 1 mới suy ra 'cai kia' la ca phe (25000 dong), qua một bảng CAP_DOI cố định. Lượt 3 hỏi 'gia mon dau tien la bao nhieu' -- cần nhớ LẠI đúng banh mi từ lượt 1 (không phải lượt 2 gần nhất), câu trả lời TRÙNG KHỚP lượt 1. Sau 3 lượt: 6 message tích luỹ, 57 token thật (đo bằng ma_hoa_van_ban) -- xác nhận thông tin lượt đầu vẫn truy cập được ở lượt cuối."
locale: vi
track: tri-tue-nhan-tao
module: quan-ly-ngu-canh-va-hoi-thoai
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.bo-nho-hoi-thoai-nhieu-luot]
requires: [ai.tom-tat-lich-su-cu]
concepts: [ai.bo-nho-hoi-thoai-nhieu-luot]
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
Hai bài trước quản lý lịch sử KHI NÓ ĐÃ DÀI (cắt bớt, tóm tắt). Bài này
lùi lại một bước: lịch sử đó TỪ ĐÂU MÀ RA? Từ việc mỗi lượt hỏi-đáp mới
đều được GHI LẠI — cả câu hỏi lẫn câu trả lời — để lượt sau còn tham
chiếu được.
::::

::::explain{#nho_xuyen_nhieu_luot}
Một cuộc hội thoại nhiều lượt không phải NHIỀU lần gọi độc lập — mỗi
lượt mới phải THẤY được toàn bộ những gì đã nói TRƯỚC ĐÓ. Cơ chế: sau
mỗi lượt, thêm CẢ HAI message (câu hỏi của người dùng VÀ câu trả lời của
LLM mô phỏng) vào cùng một danh sách `lich_su` tích luỹ — không chỉ giữ
câu hỏi, cũng không chỉ giữ câu trả lời.

Vì sao cần giữ CẢ HAI? Vì câu hỏi ở một lượt SAU có thể MƠ HỒ nếu tách
rời khỏi ngữ cảnh — ví dụ `"con cai kia thi sao"` (còn cái kia thì sao)
không nêu tên sản phẩm nào cả. Trả lời đúng câu này đòi hỏi ĐỌC LẠI lịch
sử để biết "cái kia" đang nói tới cái gì — cụ thể, sản phẩm nào đã được
hỏi ở lượt TRƯỚC.

Bài này mô phỏng việc "hiểu ngữ cảnh" đó bằng một LUẬT CỐ ĐỊNH (không
phải một LLM thật suy luận): quét các message `role="user"` TRƯỚC ĐÓ
trong lịch sử, tìm sản phẩm gần nhất đã được nêu tên, rồi tra một bảng
"cặp đôi" cố định (`CAP_DOI`) để biết "cái kia" nghĩa là gì.
::::

::::example{#bo_nho_ba_luot_that}
Ba lượt hỏi-đáp liên tiếp: lượt `1` hỏi thẳng, lượt `2` hỏi mơ hồ (cần
lượt `1`), lượt `3` hỏi lại thông tin của ĐÚNG lượt `1` (không phải lượt
`2` gần nhất):

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

GIA = {"banh mi": 15000, "ca phe": 25000}
CAP_DOI = {"banh mi": "ca phe", "ca phe": "banh mi"}

def tim_san_pham(cau):
    for sp in ("banh mi", "ca phe"):
        if sp in cau:
            return sp
    return None

def llm_mo_phong_hoi_thoai(lich_su):
    cau_hien_tai = lich_su[-1]["content"].lower()
    sp = tim_san_pham(cau_hien_tai)
    if sp is not None:
        return f"gia {sp} la {GIA[sp]} dong"
    san_pham_da_hoi = []
    for tin in lich_su[:-1]:
        if tin["role"] == "user":
            spx = tim_san_pham(tin["content"].lower())
            if spx is not None:
                san_pham_da_hoi.append(spx)
    if "cai kia" in cau_hien_tai and san_pham_da_hoi:
        sp_kia = CAP_DOI[san_pham_da_hoi[-1]]
        return f"gia {sp_kia} la {GIA[sp_kia]} dong"
    if "mon dau tien" in cau_hien_tai and san_pham_da_hoi:
        sp_dau = san_pham_da_hoi[0]
        return f"gia {sp_dau} la {GIA[sp_dau]} dong"
    return "toi khong hieu ban dang hoi ve san pham nao"

def hoi_va_cap_nhat(lich_su, cau_hoi):
    lich_su_moi = lich_su + [{"role": "user", "content": cau_hoi}]
    tra_loi = llm_mo_phong_hoi_thoai(lich_su_moi)
    lich_su_moi = lich_su_moi + [{"role": "assistant", "content": tra_loi}]
    return lich_su_moi, tra_loi

corpus = ("xin chao, toi la tro ly ao, rat vui duoc giup ban. "
          "gia banh mi bao nhieu tien. gia banh mi la 15000 dong. "
          "gia ca phe bao nhieu tien. gia ca phe la 25000 dong. "
          "gia tra sua bao nhieu tien. gia tra sua la 30000 dong. "
          "con cai kia thi sao. cam on ban rat nhieu, khong co chi. "
          "cuoc hoi thoai co the rat dai, can tom tat lich su cu "
          "de khong vuot qua cua so ngu canh cho phep.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=90)
token_sang_id = xay_token_sang_id(vocab)

lich_su = []
lich_su, tl1 = hoi_va_cap_nhat(lich_su, "gia banh mi bao nhieu tien")
lich_su, tl2 = hoi_va_cap_nhat(lich_su, "con cai kia thi sao")
lich_su, tl3 = hoi_va_cap_nhat(lich_su, "vay gia mon dau tien la bao nhieu")

tong_token = dem_token_lich_su(lich_su, merges, token_sang_id)

print("luot 1:", tl1)
print("luot 2:", tl2)
print("luot 3:", tl3)
print("so message tich luy:", len(lich_su), " tong token:", tong_token)
print("luot 3 khop luot 1:", tl1 == tl3)
```

```text title=readonly
luot 1: gia banh mi la 15000 dong
luot 2: gia ca phe la 25000 dong
luot 3: gia banh mi la 15000 dong
so message tich luy: 6  tong token: 57
luot 3 khop luot 1: True
```

Lượt `1` hỏi thẳng — trả lời trực tiếp, không cần lịch sử. Lượt `2` hỏi
`"con cai kia thi sao"` — KHÔNG nêu tên sản phẩm nào; hàm phải ĐỌC LẠI
lượt `1` (tìm ra `"banh mi"` là sản phẩm vừa hỏi), tra `CAP_DOI["banh
mi"]` ra `"ca phe"`, trả lời đúng giá cà phê. Lượt `3` hỏi về "món ĐẦU
TIÊN" — phải nhớ LẠI đúng lượt `1` (`"banh mi"`), KHÔNG PHẢI lượt `2` gần
hơn (`"ca phe"`) — và quả thật, câu trả lời lượt `3` khớp Y HỆT lượt `1`.
Sau `3` lượt, `6` message đã tích luỹ (`3` câu hỏi + `3` câu trả lời),
tổng `57` token — và thông tin của lượt ĐẦU TIÊN vẫn còn nguyên trong
`lich_su`, truy cập được ở tận lượt cuối.
::::

::::predict{#doan_luot_3 commitOnce}
Xét đúng ba lượt ở ví dụ trên: lượt `1` hỏi giá `"banh mi"`, lượt `2`
hỏi mơ hồ `"con cai kia thi sao"` (trả lời về `"ca phe"`).

**Trước khi chạy thử**, bạn đoán: lượt `3` hỏi `"vay gia mon dau tien la
bao nhieu"` (giá món ĐẦU TIÊN là bao nhiêu) sẽ trả lời về sản phẩm nào?

:::opt{correct}
`"banh mi"` — vì "món đầu tiên" phải tham chiếu tới sản phẩm được hỏi ở
LƯỢT ĐẦU TIÊN (lượt `1`, `"banh mi"`), không phải sản phẩm được nhắc gần
đây nhất (lượt `2` trả lời về `"ca phe"`); hàm phải quét TOÀN BỘ lịch sử
`user` và lấy đúng phần tử ĐẦU (`san_pham_da_hoi[0]`), không phải phần
tử cuối
:::

:::opt
`"ca phe"` — vì đó là sản phẩm được nhắc tới GẦN NHẤT (ở câu trả lời của
lượt `2`), và một hàm đọc lịch sử luôn ưu tiên thông tin MỚI NHẤT
::why
Gần đúng ở việc `"ca phe"` đúng là thông tin GẦN NHẤT trong lịch sử —
quan sát đó không sai về mặt VỊ TRÍ.

Chỗ lệch: câu hỏi ở lượt `3` không hỏi "cái GẦN NHẤT" — nó hỏi rõ "món
ĐẦU TIÊN" (`"mon dau tien"`), một cụm từ KHÁC với `"cai kia"` của lượt
`2`. Hàm mô phỏng phân biệt hai cụm này bằng hai NHÁNH luật khác nhau:
`"cai kia"` tra sản phẩm GẦN NHẤT (`san_pham_da_hoi[-1]`); `"mon dau
tien"` tra sản phẩm ĐẦU TIÊN (`san_pham_da_hoi[0]`) — hai chỉ số khác
hẳn nhau trên CÙNG một danh sách lịch sử.
::
:::

:::opt
Không trả lời được, vì lượt `3` không nêu tên sản phẩm nào và cũng không
dùng đúng cụm `"cai kia"` như lượt `2` — hàm sẽ rơi vào nhánh mặc định
::why
Gần đúng ở việc bạn để ý lượt `3` THẬT SỰ không dùng cụm `"cai kia"` —
quan sát về CÂU CHỮ chính xác đó đúng.

Chỗ lệch: hàm mô phỏng có HAI nhánh đọc lịch sử riêng biệt, không chỉ
một — nhánh `"cai kia"` VÀ nhánh `"mon dau tien"` (`"mon dau tien" in
cau_hien_tai`). Lượt `3` khớp đúng nhánh THỨ HAI này, không rơi vào mặc
định.
::
:::
::::

::::code{#viet_llm_mo_phong_hoi_thoai}
Hoàn thiện `llm_mo_phong_hoi_thoai`: nhánh `"cai kia"` phải tra sản phẩm
GẦN NHẤT trong lịch sử, còn nhánh `"mon dau tien"` phải tra sản phẩm ĐẦU
TIÊN — hai vị trí khác nhau trên CÙNG một danh sách.

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

GIA = {"banh mi": 15000, "ca phe": 25000}
CAP_DOI = {"banh mi": "ca phe", "ca phe": "banh mi"}

def tim_san_pham(cau):
    for sp in ("banh mi", "ca phe"):
        if sp in cau:
            return sp
    return None

def llm_mo_phong_hoi_thoai(lich_su):
    cau_hien_tai = lich_su[-1]["content"].lower()
    sp = tim_san_pham(cau_hien_tai)
    if sp is not None:
        return f"gia {sp} la {GIA[sp]} dong"
    san_pham_da_hoi = []
    for tin in lich_su[:-1]:
        if tin["role"] == "user":
            spx = tim_san_pham(tin["content"].lower())
            if spx is not None:
                san_pham_da_hoi.append(spx)
    if "cai kia" in cau_hien_tai and san_pham_da_hoi:
        sp_kia = CAP_DOI[___]                                   # san_pham_da_hoi[-1]
        return f"gia {sp_kia} la {GIA[sp_kia]} dong"
    if "mon dau tien" in cau_hien_tai and san_pham_da_hoi:
        sp_dau = ___                                              # san_pham_da_hoi[0]
        return f"gia {sp_dau} la {GIA[sp_dau]} dong"
    return "toi khong hieu ban dang hoi ve san pham nao"

def hoi_va_cap_nhat(lich_su, cau_hoi):
    lich_su_moi = lich_su + [{"role": "user", "content": cau_hoi}]
    tra_loi = llm_mo_phong_hoi_thoai(lich_su_moi)
    lich_su_moi = lich_su_moi + [{"role": "assistant", "content": tra_loi}]
    return lich_su_moi, tra_loi

corpus = ("xin chao, toi la tro ly ao, rat vui duoc giup ban. "
          "gia banh mi bao nhieu tien. gia banh mi la 15000 dong. "
          "gia ca phe bao nhieu tien. gia ca phe la 25000 dong. "
          "gia tra sua bao nhieu tien. gia tra sua la 30000 dong. "
          "con cai kia thi sao. cam on ban rat nhieu, khong co chi. "
          "cuoc hoi thoai co the rat dai, can tom tat lich su cu "
          "de khong vuot qua cua so ngu canh cho phep.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=90)
token_sang_id = xay_token_sang_id(vocab)

lich_su = []
lich_su, tl1 = hoi_va_cap_nhat(lich_su, "gia banh mi bao nhieu tien")
lich_su, tl2 = hoi_va_cap_nhat(lich_su, "con cai kia thi sao")
lich_su, tl3 = hoi_va_cap_nhat(lich_su, "vay gia mon dau tien la bao nhieu")

tong_token = dem_token_lich_su(lich_su, merges, token_sang_id)

print(tl1)
print(tl2)
print(tl3)
print(len(lich_su), tong_token)
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

GIA = {"banh mi": 15000, "ca phe": 25000}
CAP_DOI = {"banh mi": "ca phe", "ca phe": "banh mi"}

def tim_san_pham(cau):
    for sp in ("banh mi", "ca phe"):
        if sp in cau:
            return sp
    return None

def llm_mo_phong_hoi_thoai(lich_su):
    cau_hien_tai = lich_su[-1]["content"].lower()
    sp = tim_san_pham(cau_hien_tai)
    if sp is not None:
        return f"gia {sp} la {GIA[sp]} dong"
    san_pham_da_hoi = []
    for tin in lich_su[:-1]:
        if tin["role"] == "user":
            spx = tim_san_pham(tin["content"].lower())
            if spx is not None:
                san_pham_da_hoi.append(spx)
    if "cai kia" in cau_hien_tai and san_pham_da_hoi:
        sp_kia = CAP_DOI[san_pham_da_hoi[-1]]
        return f"gia {sp_kia} la {GIA[sp_kia]} dong"
    if "mon dau tien" in cau_hien_tai and san_pham_da_hoi:
        sp_dau = san_pham_da_hoi[0]
        return f"gia {sp_dau} la {GIA[sp_dau]} dong"
    return "toi khong hieu ban dang hoi ve san pham nao"

def hoi_va_cap_nhat(lich_su, cau_hoi):
    lich_su_moi = lich_su + [{"role": "user", "content": cau_hoi}]
    tra_loi = llm_mo_phong_hoi_thoai(lich_su_moi)
    lich_su_moi = lich_su_moi + [{"role": "assistant", "content": tra_loi}]
    return lich_su_moi, tra_loi

corpus = ("xin chao, toi la tro ly ao, rat vui duoc giup ban. "
          "gia banh mi bao nhieu tien. gia banh mi la 15000 dong. "
          "gia ca phe bao nhieu tien. gia ca phe la 25000 dong. "
          "gia tra sua bao nhieu tien. gia tra sua la 30000 dong. "
          "con cai kia thi sao. cam on ban rat nhieu, khong co chi. "
          "cuoc hoi thoai co the rat dai, can tom tat lich su cu "
          "de khong vuot qua cua so ngu canh cho phep.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=90)
token_sang_id = xay_token_sang_id(vocab)

lich_su = []
lich_su, tl1 = hoi_va_cap_nhat(lich_su, "gia banh mi bao nhieu tien")
lich_su, tl2 = hoi_va_cap_nhat(lich_su, "con cai kia thi sao")
lich_su, tl3 = hoi_va_cap_nhat(lich_su, "vay gia mon dau tien la bao nhieu")

tong_token = dem_token_lich_su(lich_su, merges, token_sang_id)

print(tl1)
print(tl2)
print(tl3)
print(len(lich_su), tong_token)
```

```python title=test
assert tl1 == "gia banh mi la 15000 dong", f"cau tra loi luot 1 sai -- dang ra {tl1!r}"
assert tl2 == "gia ca phe la 25000 dong", f"cau tra loi luot 2 sai -- dang ra {tl2!r}"
assert tl3 == "gia banh mi la 15000 dong", f"cau tra loi luot 3 sai -- dang ra {tl3!r}"

# BANG CHUNG TRUNG TAM: thong tin luot DAU (banh mi) van con truy cap
# duoc o luot CUOI, du luot GIUA da chuyen sang san pham khac
assert tl3 == tl1, f"luot 3 phai KHOP DUNG luot 1 (nho lai san pham dau tien) -- tl1={tl1!r}, tl3={tl3!r}"
assert tl3 != tl2, f"luot 3 khong duoc lay nham san pham cua luot 2 (gan nhat) -- tl3={tl3!r}"

assert len(lich_su) == 6, f"so message tich luy sau 3 luot phai la 6 -- dang ra {len(lich_su)}"
assert tong_token == 57, f"tong token sau 3 luot phai la 57 -- dang ra {tong_token}"

# noi dung cau hoi goc phai con nguyen trong lich su (khong bi sua doi)
noi_dung_lich_su = [m["content"] for m in lich_su]
assert noi_dung_lich_su[0] == "gia banh mi bao nhieu tien", "cau hoi luot 1 phai con nguyen trong lich su"
assert noi_dung_lich_su[1] == tl1, "cau tra loi luot 1 phai duoc luu dung trong lich su"

# bien: hoi "con cai kia thi sao" NGAY LUOT DAU TIEN (khong co lich su
# nao ca) -- phai roi vao nhanh mac dinh, khong duoc bao loi
tl_bien_1 = llm_mo_phong_hoi_thoai([{"role": "user", "content": "con cai kia thi sao"}])
assert tl_bien_1 == "toi khong hieu ban dang hoi ve san pham nao", f"hoi mo ho ngay tu dau (khong co lich su) phai roi vao mac dinh -- dang ra {tl_bien_1!r}"

# bien: hoi "mon dau tien" ngay luot dau tien -- cung phai roi vao mac dinh
tl_bien_2 = llm_mo_phong_hoi_thoai([{"role": "user", "content": "vay gia mon dau tien la bao nhieu"}])
assert tl_bien_2 == "toi khong hieu ban dang hoi ve san pham nao", f"hoi 'mon dau tien' ngay tu dau (khong co lich su) phai roi vao mac dinh -- dang ra {tl_bien_2!r}"

# kiem tra doc lap: sau CHINH XAC 1 luot, chi co 2 message
lich_su_1_luot, _ = hoi_va_cap_nhat([], "gia ca phe bao nhieu tien")
assert len(lich_su_1_luot) == 2, f"sau 1 luot phai co dung 2 message -- dang ra {len(lich_su_1_luot)}"

# bien QUAN TRONG: HAI san pham KHAC nhau da duoc hoi truoc do (banh mi
# roi ca phe) -- luc nay san_pham_da_hoi co 2 phan tu, nen chi so [0]
# (DAU TIEN) va [-1] (GAN NHAT) tro toi HAI san pham KHAC nhau. Neu hai
# cho trong bi DAO NGUOC cho nhau (nhanh "cai kia" dung [0] thay vi [-1],
# nhanh "mon dau tien" dung [-1] thay vi [0]), ca hai cau tra loi deu doi
# sang "ca phe" thay vi "banh mi" -- day la bang chung DOC LAP phan biet
# duoc mot cheat dao nguoc CA HAI cho trong cung luc (da tu kiem chung
# rieng: cheat dao nguoc khong doi tong so dem AST cua "neg"/has-literal
# "0" vi tong ca hai khong doi khi hoan doi vi tri cho nhau -- CHI test
# nay moi bat duoc, khong phai static rieng le).
lich_su_hai_san_pham = [
    {"role": "user", "content": "gia banh mi bao nhieu tien"},
    {"role": "assistant", "content": "gia banh mi la 15000 dong"},
    {"role": "user", "content": "gia ca phe bao nhieu tien"},
    {"role": "assistant", "content": "gia ca phe la 25000 dong"},
    {"role": "user", "content": "con cai kia thi sao"},
]
tl_hai_sp_kia = llm_mo_phong_hoi_thoai(lich_su_hai_san_pham)
assert tl_hai_sp_kia == "gia banh mi la 15000 dong", f"'cai kia' voi 2 san pham (banh mi roi ca phe) phai tra ve san pham GAN NHAT bi doi cheo (banh mi) -- dang ra {tl_hai_sp_kia!r}"

lich_su_hai_san_pham_dau = [
    {"role": "user", "content": "gia banh mi bao nhieu tien"},
    {"role": "assistant", "content": "gia banh mi la 15000 dong"},
    {"role": "user", "content": "gia ca phe bao nhieu tien"},
    {"role": "assistant", "content": "gia ca phe la 25000 dong"},
    {"role": "user", "content": "vay gia mon dau tien la bao nhieu"},
]
tl_hai_sp_dau = llm_mo_phong_hoi_thoai(lich_su_hai_san_pham_dau)
assert tl_hai_sp_dau == "gia banh mi la 15000 dong", f"'mon dau tien' voi 2 san pham (banh mi roi ca phe) phai tra ve san pham DAU TIEN (banh mi) -- dang ra {tl_hai_sp_dau!r}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cùng tra vào danh sách `san_pham_da_hoi` (các sản phẩm ĐÃ hỏi trước lượt hiện tại, theo đúng thứ tự xuất hiện). Chỗ 1 (nhánh `"cai kia"`) cần sản phẩm GẦN NHẤT — phần tử CUỐI danh sách, chỉ số `-1`. Chỗ 2 (nhánh `"mon dau tien"`) cần sản phẩm ĐẦU TIÊN — phần tử ĐẦU danh sách, chỉ số `0`.
- kind: strategy
  body: 'Chỗ 1: `san_pham_da_hoi[-1]`. Chỗ 2: `san_pham_da_hoi[0]`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `san_pham_da_hoi[-1]` và `san_pham_da_hoi[0]`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: nhanh "cai kia" phai tra CHI SO -1 (san pham GAN NHAT) tren san_pham_da_hoi, VA nhanh "mon dau tien" phai tra CHI SO 0 (san pham DAU TIEN) -- dao nguoc hai chi so nay se lam luot 3 tra loi SAI san pham
  requireAst:
  - kind: uses-operator, target: "neg", min: 3, max: 3
  - kind: has-literal, target: "0", min: 4
  # Da thu that (goi bo dem AST doc lap mo phong dung logic kiem-ast.ts tren
  # code trich tu solution): loi giai dung dat=true, ca hai luat qua sach.
  # "neg" (UnaryOp USub, KHONG khop qua has-literal vi so am la mot phep
  # toan mot ngoi, khong phai Constant thuan)=3: "lich_su[-1]" (dong dau
  # llm_mo_phong_hoi_thoai), "lich_su[:-1]" (vong lap quet san_pham_da_hoi),
  # va cho trong 1 (san_pham_da_hoi[-1]) -- tong dung 3, dat ca min VA max.
  # has-literal "0"=4: "dem.get(cap, 0)" (dem_cap_lien_ke), "i = 0" (gop_cap),
  # mot noi khac tuong tu trong huan_luyen_bpe/gop_cap, va cho trong 2
  # (san_pham_da_hoi[0]) -- tong dung 4.
  #
  # Cheat "chi doi cho trong 1" (thanh [0], giu cho trong 2 nguyen) lam
  # "neg" tut xuong 2 (duoi min=3) -- bi chan rieng boi static; da tu kiem
  # chung: cheat nay lam tl2 (luot 2, hoi "cai kia") tra ve "gia banh mi la
  # 15000 dong" (SAI -- phai la ca phe) tren kich ban 3-luot chinh.
  #
  # 🔴 GOTCHA THAT SU (tu xac minh doc lap, KHONG phai suy luan): cheat "dao
  # nguoc CA HAI cho trong cung luc" (cho trong 1 dung [0], cho trong 2 dung
  # [-1]) KHONG doi tong so dem AST cua "neg" hay has-literal "0" -- vi hoan
  # doi vi tri chi DI CHUYEN mot occurrence tu cho nay sang cho kia, tong
  # khong doi (van la 3 va 4). Static KHONG bat duoc cheat nay. Tren kich
  # ban 3-luot CHINH cua vi du/test o tren, san_pham_da_hoi LUON chi co
  # DUNG 1 phan tu ("banh mi") tai moi diem goi toi hai nhanh nay -- nen
  # [0] va [-1] TINH CO tro toi CUNG mot gia tri, va cheat dao-nguoc-ca-hai
  # cung KHONG bi bat boi tests/output cua kich ban chinh. Day la mot LO
  # DOT BIEN THAT tu xac minh doc lap qua kiemAst() that + chay Python that
  # (khong phai tu bao cao cua agent thiet ke) -- da SUA bang cach them hai
  # ca test RIENG voi mot lich su co HAI san pham KHAC nhau da hoi truoc do
  # (banh mi roi ca phe, xem "bien QUAN TRONG" o duoi trong khoi test) --
  # luc do san_pham_da_hoi co 2 phan tu, [0] va [-1] tro toi hai gia tri
  # KHAC nhau, va cheat dao-nguoc-ca-hai bi bat DOC LAP boi hai assertion
  # moi do (tl_hai_sp_kia/tl_hai_sp_dau), khong phu thuoc static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^gia banh mi la 15000 dong\\ngia ca phe la 25000 dong\\ngia banh mi la 15000 dong\\n6 57\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`6` message, `57` token — và thông tin của lượt đầu tiên (`"banh mi"`,
`15000 dong`) vẫn nguyên vẹn, truy cập được ở tận lượt thứ ba. Bài BOSS
cuối quest ráp TẤT CẢ: một vòng lặp hội thoại nhiều lượt, tự cắt hoặc tự
tóm tắt NGAY khi gần đầy cửa sổ ngữ cảnh.
::::

::::reflect{#nghi-lai}
Bộ nhớ hội thoại không phải một cơ chế "thông minh" bí ẩn — nó chỉ là
việc GIỮ LẠI đủ dữ liệu (cả câu hỏi lẫn câu trả lời, mọi lượt) và ĐỌC LẠI
đúng phần cần thiết khi lượt sau cần tới. Nhưng giữ lại MÃI MÃI, không
giới hạn, chính là thứ ba bài trước đã cảnh báo: tổng token sẽ tăng
không ngừng qua nhiều lượt, sớm muộn vượt trần cửa sổ ngữ cảnh. Bài BOSS
ráp cả bốn kỹ thuật — đếm token thật, cắt bớt, tóm tắt, bộ nhớ nhiều
lượt — thành MỘT vòng lặp hội thoại tự quản lý.
::::

::::checkpoint{mastery=0.85}
::::
