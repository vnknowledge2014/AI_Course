---
id: tri-tue-nhan-tao.quan-ly-ngu-canh-va-hoi-thoai.boss-hoi-thoai-tu-cat-tom-tat
title: "BOSS — Hội thoại tự cắt/tóm tắt: quản lý ngữ cảnh không phải chuyện phụ"
summary: "Ráp bốn kỹ thuật của quest thành MỘT vòng lặp hội thoại 6 lượt: mỗi lượt thêm message mới, đếm token THẬT (ma_hoa_van_ban), nếu vượt TRAN=110 thì tóm tắt phần lịch sử cũ (giữ 2 message gần nhất). Qua 6 lượt: 3 lần kích hoạt tóm tắt, đúng các cặp (truoc,sau) = (149,92), (126,82), (119,85) -- token SAU luôn <= 110 ngay sau kích hoạt. Đối chứng với phiên bản ngây thơ (không bao giờ cắt/tóm tắt): token tăng không ngừng qua 6 lượt (36, 71, 105, 149, 183, 220) -- vượt hẳn TRAN=110 và tiếp tục tăng, trong khi phiên bản có quản lý kết thúc ở 85 token, chưa tới một nửa."
locale: vi
track: tri-tue-nhan-tao
module: quan-ly-ngu-canh-va-hoi-thoai
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-hoi-thoai-tu-cat-tom-tat]
requires: [ai.bo-nho-hoi-thoai-nhieu-luot]
concepts: [ai.boss-hoi-thoai-tu-cat-tom-tat]
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

::::byte{trigger=enter mood=happy pose=jump}
Bốn bài: đếm token thật, cắt bớt khi vượt trần, tóm tắt thay vì xoá sạch,
bộ nhớ xuyên nhiều lượt. Bài này ráp TẤT CẢ vào một vòng lặp hội thoại
chạy `6` lượt liên tiếp, TỰ quyết định khi nào cần tóm tắt — và đo bằng
số tại sao việc đó không phải chuyện phụ.
::::

::::explain{#rap_toan_bo_vong_lap}
Một vòng lặp hội thoại "tự quản lý" gồm ĐÚNG những gì bốn bài trước đã
xây, chạy MỖI lượt:

> **1. Thêm** message mới (câu hỏi + câu trả lời qua LLM mô phỏng) vào
> `lich_su` (bài `bo-nho-hoi-thoai-nhieu-luot`).
>
> **2. Đếm** tổng token THẬT của `lich_su` bằng `ma_hoa_van_ban` (bài
> `dem-token-that`) — không phải ký tự, không phải từ.
>
> **3. Nếu vượt trần** — tóm tắt phần lịch sử CŨ (mọi message TRỪ vài
> message gần nhất) thành MỘT message, bằng đúng kỹ thuật
> `tom_tat_mo_phong` (bài `tom-tat-lich-su-cu`), rồi tiếp tục.

Khác bài `cua-so-ngu-canh-va-cat-bot` (cắt bỏ hẳn), vòng lặp này NÉN thay
vì xoá — đúng kỹ thuật bài `tom-tat-lich-su-cu`, áp dụng LẶP LẠI qua
nhiều lượt thay vì chỉ một lần.

Một điểm quan trọng về hiệu năng: BPE chỉ được HUẤN LUYỆN đúng MỘT LẦN,
trước khi vòng lặp `6` lượt bắt đầu — bên trong vòng lặp, `merges` đã học
chỉ được ÁP DỤNG lại (qua `ma_hoa_van_ban`), không huấn luyện lại bao giờ.

Để thấy rõ vì sao việc này KHÔNG PHẢI chuyện phụ, bài này chạy SONG SONG
một phiên bản **ngây thơ** — chỉ thêm message, KHÔNG BAO GIỜ cắt hay tóm
tắt gì cả — trên ĐÚNG cùng `6` câu hỏi.
::::

::::example{#boss_vong_lap_that}
Chạy `6` lượt hỏi liên tiếp qua cả phiên bản CÓ QUẢN LÝ (`TRAN = 110`,
giữ `2` message gần nhất mỗi lần tóm tắt) và phiên bản NGÂY THƠ:

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

GIA_3 = {"banh mi": 15000, "ca phe": 25000, "tra sua": 30000}
TU_KHOA_SAN_PHAM = ["banh mi", "ca phe", "tra sua"]

def tim_san_pham_3(cau):
    for sp in TU_KHOA_SAN_PHAM:
        if sp in cau:
            return sp
    return None

def llm_mo_phong(cau_hoi):
    sp = tim_san_pham_3(cau_hoi.lower())
    if sp is None:
        return "toi khong hieu ban dang hoi ve san pham nao"
    return f"gia {sp} la {GIA_3[sp]} dong"

def tom_tat_mo_phong(cac_message_cu):
    chu_de = []
    for tin in cac_message_cu:
        noi_dung = tin["content"].lower()
        for tu in TU_KHOA_SAN_PHAM:
            if tu in noi_dung and tu not in chu_de:
                chu_de.append(tu)
    if not chu_de:
        noi_dung_tom_tat = "tom tat lich su cu, khong co san pham nao duoc hoi."
    else:
        noi_dung_tom_tat = "tom tat lich su cu, da hoi gia " + " va ".join(chu_de) + "."
    return {"role": "system", "content": noi_dung_tom_tat}

def chay_hoi_thoai_ngay_tho(cau_hoi_ds, merges, token_sang_id):
    lich_su = []
    danh_sach_tong = []
    for cau_hoi in cau_hoi_ds:
        tra_loi = llm_mo_phong(cau_hoi)
        lich_su.append({"role": "user", "content": cau_hoi})
        lich_su.append({"role": "assistant", "content": tra_loi})
        danh_sach_tong.append(dem_token_lich_su(lich_su, merges, token_sang_id))
    return lich_su, danh_sach_tong

def chay_hoi_thoai_quan_ly(cau_hoi_ds, tran, giu_lai_gan_day, merges, token_sang_id):
    lich_su = []
    cac_cap_truoc_sau = []
    for cau_hoi in cau_hoi_ds:
        tra_loi = llm_mo_phong(cau_hoi)
        lich_su.append({"role": "user", "content": cau_hoi})
        lich_su.append({"role": "assistant", "content": tra_loi})
        tong_truoc = dem_token_lich_su(lich_su, merges, token_sang_id)
        if tong_truoc > tran:
            cu = lich_su[:-giu_lai_gan_day]
            gan_day = lich_su[-giu_lai_gan_day:]
            tin_tom_tat = tom_tat_mo_phong(cu)
            lich_su = [tin_tom_tat] + gan_day
            tong_sau = dem_token_lich_su(lich_su, merges, token_sang_id)
            cac_cap_truoc_sau.append((tong_truoc, tong_sau))
    return lich_su, cac_cap_truoc_sau

corpus = ("xin chao, toi la tro ly ao, rat vui duoc giup ban. "
          "gia banh mi bao nhieu tien. gia banh mi la 15000 dong. "
          "gia ca phe bao nhieu tien. gia ca phe la 25000 dong. "
          "gia tra sua bao nhieu tien. gia tra sua la 30000 dong. "
          "con cai kia thi sao. cam on ban rat nhieu, khong co chi. "
          "cuoc hoi thoai co the rat dai, can tom tat lich su cu "
          "de khong vuot qua cua so ngu canh cho phep.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=40)
token_sang_id = xay_token_sang_id(vocab)

CAU_HOI_6_LUOT = [
    "gia banh mi bao nhieu tien",
    "gia ca phe bao nhieu tien",
    "gia tra sua bao nhieu tien",
    "vay gia banh mi la bao nhieu tien nhi",
    "con gia ca phe thi sao",
    "gia tra sua bao nhieu tien nhi",
]

TRAN = 110
GIU_LAI_GAN_DAY = 2

lich_su_ngay_tho, danh_sach_tong_ngay_tho = chay_hoi_thoai_ngay_tho(CAU_HOI_6_LUOT, merges, token_sang_id)
lich_su_quan_ly, cac_cap_truoc_sau = chay_hoi_thoai_quan_ly(CAU_HOI_6_LUOT, TRAN, GIU_LAI_GAN_DAY, merges, token_sang_id)

print("ngay tho, tong token qua tung luot:", danh_sach_tong_ngay_tho)
print("quan ly, cac cap (truoc, sau) khi kich hoat:", cac_cap_truoc_sau)
print("so lan kich hoat:", len(cac_cap_truoc_sau))
print("tat ca sau <= TRAN:", all(sau <= TRAN for (_, sau) in cac_cap_truoc_sau))
print("token cuoi ngay tho:", danh_sach_tong_ngay_tho[-1], " token cuoi quan ly:", dem_token_lich_su(lich_su_quan_ly, merges, token_sang_id))
```

```text title=readonly
ngay tho, tong token qua tung luot: [36, 71, 105, 149, 183, 220]
quan ly, cac cap (truoc, sau) khi kich hoat: [(149, 92), (126, 82), (119, 85)]
so lan kich hoat: 3
tat ca sau <= TRAN: True
token cuoi ngay tho: 220  token cuoi quan ly: 85
```

Phiên bản NGÂY THƠ: token tăng ĐỀU qua từng lượt (`36, 71, 105, 149, 183,
220`) — không có gì chặn lại, vượt hẳn `TRAN = 110` từ lượt `4` trở đi và
CỨ TĂNG TIẾP không giới hạn cho tới hết `6` lượt. Phiên bản CÓ QUẢN LÝ:
ba lượt ĐẦU (`36`, `71`, `105`) đều CHƯA vượt trần, không kích hoạt gì
cả; kích hoạt tóm tắt đúng `3` lần ở lượt `4`, `5`, `6`, mỗi lần đưa
token về LẠI dưới trần NGAY LẬP TỨC (`149→92`, `126→82`, `119→85` — cả
ba đều `≤ 110`), kết thúc ở `85` token — chưa tới MỘT NỬA so với `220`
của bản ngây thơ trên CÙNG `6` câu hỏi.
::::

::::predict{#doan_so_lan_kich_hoat commitOnce}
Xét đúng `6` câu hỏi và `TRAN = 110` ở ví dụ trên.

**Trước khi chạy thử**, bạn đoán: qua `6` lượt, tóm tắt được kích hoạt
BAO NHIÊU lần?

:::opt{correct}
`3` lần — lượt `1`, `2`, `3` có tổng token `36`, `71`, `105`, đều `≤
110`, KHÔNG kích hoạt; lượt `4` đạt `149 > 110` → kích hoạt (tóm tắt về
`92`); lượt `5` cộng thêm câu hỏi mới vào lịch sử ĐÃ được nén, đạt `126 >
110` → kích hoạt tiếp (về `82`); lượt `6` đạt `119 > 110` → kích hoạt
lần thứ ba (về `85`) — tổng cộng đúng `3` lần trên `6` lượt
:::

:::opt
`6` lần — vì mỗi lượt đều THÊM message mới, nên token luôn tăng, và một
khi đã bắt đầu tóm tắt thì MỌI lượt sau đó cũng phải tóm tắt tiếp
::why
Gần đúng ở việc mỗi lượt ĐỀU thêm message mới — quan sát đó đúng, và
việc "tăng liên tục" từ đó cũng đúng MỘT PHẦN.

Chỗ lệch: sau khi tóm tắt, tổng token bị NÉN XUỐNG THẤP HẲN (ví dụ về
`92`, `82`, `85` — đều còn cách trần `110` một khoảng), nên KHÔNG PHẢI
lượt nào cũng vượt lại trần NGAY LẬP TỨC. Ba lượt ĐẦU (`1`, `2`, `3`)
chưa bao giờ vượt trần — tổng của chúng (`36`, `71`, `105`) đều còn dưới
`110` — nên hoàn toàn KHÔNG kích hoạt tóm tắt nào cả, không phải `6`
lần.
::
:::

:::opt
`0` lần — vì `TRAN = 110` đủ lớn để chứa toàn bộ `6` câu hỏi + câu trả
lời, không cần tóm tắt gì cả
::why
Gần đúng ở việc `TRAN = 110` KHÔNG PHẢI một con số cực nhỏ — với vài
lượt ĐẦU, nó đúng là đủ chứa (`36`, `71`, `105` đều dưới `110`).

Chỗ lệch: "đủ lớn cho vài lượt đầu" không có nghĩa là "đủ lớn cho TOÀN
BỘ `6` lượt". Mỗi lượt mới CỘNG DỒN thêm token vào lịch sử — tới lượt
`4`, tổng đã đạt `149`, VƯỢT `110` — buộc phải kích hoạt. Với một số lượt
đủ nhiều, MỌI trần cố định (dù không quá nhỏ) sớm muộn cũng bị vượt nếu
không có cơ chế cắt/tóm tắt.
::
:::
::::

::::predict{#doan_ngay_tho_vs_quan_ly commitOnce}
**Trước khi chạy thử**, bạn đoán: token CUỐI CÙNG (sau đúng `6` lượt) của
phiên bản NGÂY THƠ so với phiên bản CÓ QUẢN LÝ — cái nào lớn hơn, và
phiên bản có quản lý có LUÔN giữ được dưới `TRAN = 110` sau mỗi lần kích
hoạt không?

:::opt{correct}
Ngây thơ LỚN HƠN NHIỀU (`220` so với `85`, hơn gấp đôi) — và phiên bản
có quản lý LUÔN đưa token về `≤ 110` NGAY SAU mỗi lần kích hoạt (`92`,
`82`, `85` — cả ba đều thoả); ngây thơ thì VƯỢT `110` từ lượt `4` và
tiếp tục tăng không có gì chặn lại
:::

:::opt
Ngây thơ lớn hơn, nhưng phiên bản có quản lý ĐÔI KHI vẫn vượt `TRAN`
ngay sau khi kích hoạt, vì tóm tắt không đảm bảo giảm đủ nhiều
::why
Gần đúng ở việc bạn nghi ngờ liệu tóm tắt có LUÔN đủ mạnh để về dưới
trần — sự thận trọng đó không sai về NGUYÊN TẮC (một bản tóm tắt QUÁ
dài về lý thuyết có thể không đủ giảm).

Chỗ lệch: trên đúng bộ dữ liệu này, đã đo bằng số thật — CẢ BA lần kích
hoạt (`149→92`, `126→82`, `119→85`) đều đưa token về ĐÚNG dưới `110`,
không có ngoại lệ nào. Khẳng định "đôi khi vẫn vượt" không khớp với con
số đã đo được trên kịch bản cụ thể này.
::
:::

:::opt
Cả hai bằng nhau ở lượt cuối, vì cả hai đều xử lý ĐÚNG CÙNG `6` câu hỏi
và câu trả lời như nhau
::why
Gần đúng ở việc cả hai phiên bản THẬT SỰ nhận đúng CÙNG `6` câu hỏi —
đầu vào giống hệt nhau.

Chỗ lệch: đầu vào giống nhau không có nghĩa là XỬ LÝ giống nhau. Ngây
thơ GIỮ NGUYÊN mọi message (không xoá, không nén gì), nên lịch sử của
nó dài dần theo `12` message hoàn chỉnh. Phiên bản có quản lý NÉN lại
mỗi khi vượt trần — kết thúc với chỉ `3` message (`1` tóm tắt + `2` gần
nhất), token thấp hơn HẲN.
::
:::
::::

::::code{#viet_chay_hoi_thoai_quan_ly}
Hoàn thiện ba chỗ trống: điều kiện kích hoạt tóm tắt phải so sánh ĐÚNG
"vượt trần" (không phải "từ trần trở lên"), lịch sử sau khi tóm tắt phải
gồm CẢ message tóm tắt LẪN các message gần đây giữ nguyên, và kết luận
cuối phải xác nhận CẢ HAI bằng chứng (luôn dưới trần VÀ tốt hơn ngây
thơ).

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

GIA_3 = {"banh mi": 15000, "ca phe": 25000, "tra sua": 30000}
TU_KHOA_SAN_PHAM = ["banh mi", "ca phe", "tra sua"]

def tim_san_pham_3(cau):
    for sp in TU_KHOA_SAN_PHAM:
        if sp in cau:
            return sp
    return None

def llm_mo_phong(cau_hoi):
    sp = tim_san_pham_3(cau_hoi.lower())
    if sp is None:
        return "toi khong hieu ban dang hoi ve san pham nao"
    return f"gia {sp} la {GIA_3[sp]} dong"

def tom_tat_mo_phong(cac_message_cu):
    chu_de = []
    for tin in cac_message_cu:
        noi_dung = tin["content"].lower()
        for tu in TU_KHOA_SAN_PHAM:
            if tu in noi_dung and tu not in chu_de:
                chu_de.append(tu)
    if not chu_de:
        noi_dung_tom_tat = "tom tat lich su cu, khong co san pham nao duoc hoi."
    else:
        noi_dung_tom_tat = "tom tat lich su cu, da hoi gia " + " va ".join(chu_de) + "."
    return {"role": "system", "content": noi_dung_tom_tat}

def chay_hoi_thoai_ngay_tho(cau_hoi_ds, merges, token_sang_id):
    lich_su = []
    danh_sach_tong = []
    for cau_hoi in cau_hoi_ds:
        tra_loi = llm_mo_phong(cau_hoi)
        lich_su.append({"role": "user", "content": cau_hoi})
        lich_su.append({"role": "assistant", "content": tra_loi})
        danh_sach_tong.append(dem_token_lich_su(lich_su, merges, token_sang_id))
    return lich_su, danh_sach_tong

def chay_hoi_thoai_quan_ly(cau_hoi_ds, tran, giu_lai_gan_day, merges, token_sang_id):
    lich_su = []
    cac_cap_truoc_sau = []
    for cau_hoi in cau_hoi_ds:
        tra_loi = llm_mo_phong(cau_hoi)
        lich_su.append({"role": "user", "content": cau_hoi})
        lich_su.append({"role": "assistant", "content": tra_loi})
        tong_truoc = dem_token_lich_su(lich_su, merges, token_sang_id)
        if tong_truoc ___ tran:                                          # >
            cu = lich_su[:-giu_lai_gan_day]
            gan_day = lich_su[-giu_lai_gan_day:]
            tin_tom_tat = tom_tat_mo_phong(cu)
            lich_su = ___                                                 # [tin_tom_tat] + gan_day
            tong_sau = dem_token_lich_su(lich_su, merges, token_sang_id)
            cac_cap_truoc_sau.append((tong_truoc, tong_sau))
    return lich_su, cac_cap_truoc_sau

corpus = ("xin chao, toi la tro ly ao, rat vui duoc giup ban. "
          "gia banh mi bao nhieu tien. gia banh mi la 15000 dong. "
          "gia ca phe bao nhieu tien. gia ca phe la 25000 dong. "
          "gia tra sua bao nhieu tien. gia tra sua la 30000 dong. "
          "con cai kia thi sao. cam on ban rat nhieu, khong co chi. "
          "cuoc hoi thoai co the rat dai, can tom tat lich su cu "
          "de khong vuot qua cua so ngu canh cho phep.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=40)
token_sang_id = xay_token_sang_id(vocab)

CAU_HOI_6_LUOT = [
    "gia banh mi bao nhieu tien",
    "gia ca phe bao nhieu tien",
    "gia tra sua bao nhieu tien",
    "vay gia banh mi la bao nhieu tien nhi",
    "con gia ca phe thi sao",
    "gia tra sua bao nhieu tien nhi",
]

TRAN = 110
GIU_LAI_GAN_DAY = 2

lich_su_ngay_tho, danh_sach_tong_ngay_tho = chay_hoi_thoai_ngay_tho(CAU_HOI_6_LUOT, merges, token_sang_id)
lich_su_quan_ly, cac_cap_truoc_sau = chay_hoi_thoai_quan_ly(CAU_HOI_6_LUOT, TRAN, GIU_LAI_GAN_DAY, merges, token_sang_id)

so_lan_kich_hoat = len(cac_cap_truoc_sau)
tat_ca_duoi_tran = all(sau <= TRAN for (_, sau) in cac_cap_truoc_sau)
tong_cuoi_quan_ly = dem_token_lich_su(lich_su_quan_ly, merges, token_sang_id)
tong_cuoi_ngay_tho = danh_sach_tong_ngay_tho[-1]
quan_ly_tot_hon = tat_ca_duoi_tran ___ (tong_cuoi_quan_ly < tong_cuoi_ngay_tho)   # and

print(danh_sach_tong_ngay_tho)
print(cac_cap_truoc_sau)
print(so_lan_kich_hoat, tat_ca_duoi_tran)
print(tong_cuoi_quan_ly, tong_cuoi_ngay_tho)
print(quan_ly_tot_hon)
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

GIA_3 = {"banh mi": 15000, "ca phe": 25000, "tra sua": 30000}
TU_KHOA_SAN_PHAM = ["banh mi", "ca phe", "tra sua"]

def tim_san_pham_3(cau):
    for sp in TU_KHOA_SAN_PHAM:
        if sp in cau:
            return sp
    return None

def llm_mo_phong(cau_hoi):
    sp = tim_san_pham_3(cau_hoi.lower())
    if sp is None:
        return "toi khong hieu ban dang hoi ve san pham nao"
    return f"gia {sp} la {GIA_3[sp]} dong"

def tom_tat_mo_phong(cac_message_cu):
    chu_de = []
    for tin in cac_message_cu:
        noi_dung = tin["content"].lower()
        for tu in TU_KHOA_SAN_PHAM:
            if tu in noi_dung and tu not in chu_de:
                chu_de.append(tu)
    if not chu_de:
        noi_dung_tom_tat = "tom tat lich su cu, khong co san pham nao duoc hoi."
    else:
        noi_dung_tom_tat = "tom tat lich su cu, da hoi gia " + " va ".join(chu_de) + "."
    return {"role": "system", "content": noi_dung_tom_tat}

def chay_hoi_thoai_ngay_tho(cau_hoi_ds, merges, token_sang_id):
    lich_su = []
    danh_sach_tong = []
    for cau_hoi in cau_hoi_ds:
        tra_loi = llm_mo_phong(cau_hoi)
        lich_su.append({"role": "user", "content": cau_hoi})
        lich_su.append({"role": "assistant", "content": tra_loi})
        danh_sach_tong.append(dem_token_lich_su(lich_su, merges, token_sang_id))
    return lich_su, danh_sach_tong

def chay_hoi_thoai_quan_ly(cau_hoi_ds, tran, giu_lai_gan_day, merges, token_sang_id):
    lich_su = []
    cac_cap_truoc_sau = []
    for cau_hoi in cau_hoi_ds:
        tra_loi = llm_mo_phong(cau_hoi)
        lich_su.append({"role": "user", "content": cau_hoi})
        lich_su.append({"role": "assistant", "content": tra_loi})
        tong_truoc = dem_token_lich_su(lich_su, merges, token_sang_id)
        if tong_truoc > tran:
            cu = lich_su[:-giu_lai_gan_day]
            gan_day = lich_su[-giu_lai_gan_day:]
            tin_tom_tat = tom_tat_mo_phong(cu)
            lich_su = [tin_tom_tat] + gan_day
            tong_sau = dem_token_lich_su(lich_su, merges, token_sang_id)
            cac_cap_truoc_sau.append((tong_truoc, tong_sau))
    return lich_su, cac_cap_truoc_sau

corpus = ("xin chao, toi la tro ly ao, rat vui duoc giup ban. "
          "gia banh mi bao nhieu tien. gia banh mi la 15000 dong. "
          "gia ca phe bao nhieu tien. gia ca phe la 25000 dong. "
          "gia tra sua bao nhieu tien. gia tra sua la 30000 dong. "
          "con cai kia thi sao. cam on ban rat nhieu, khong co chi. "
          "cuoc hoi thoai co the rat dai, can tom tat lich su cu "
          "de khong vuot qua cua so ngu canh cho phep.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=40)
token_sang_id = xay_token_sang_id(vocab)

CAU_HOI_6_LUOT = [
    "gia banh mi bao nhieu tien",
    "gia ca phe bao nhieu tien",
    "gia tra sua bao nhieu tien",
    "vay gia banh mi la bao nhieu tien nhi",
    "con gia ca phe thi sao",
    "gia tra sua bao nhieu tien nhi",
]

TRAN = 110
GIU_LAI_GAN_DAY = 2

lich_su_ngay_tho, danh_sach_tong_ngay_tho = chay_hoi_thoai_ngay_tho(CAU_HOI_6_LUOT, merges, token_sang_id)
lich_su_quan_ly, cac_cap_truoc_sau = chay_hoi_thoai_quan_ly(CAU_HOI_6_LUOT, TRAN, GIU_LAI_GAN_DAY, merges, token_sang_id)

so_lan_kich_hoat = len(cac_cap_truoc_sau)
tat_ca_duoi_tran = all(sau <= TRAN for (_, sau) in cac_cap_truoc_sau)
tong_cuoi_quan_ly = dem_token_lich_su(lich_su_quan_ly, merges, token_sang_id)
tong_cuoi_ngay_tho = danh_sach_tong_ngay_tho[-1]
quan_ly_tot_hon = tat_ca_duoi_tran and (tong_cuoi_quan_ly < tong_cuoi_ngay_tho)

print(danh_sach_tong_ngay_tho)
print(cac_cap_truoc_sau)
print(so_lan_kich_hoat, tat_ca_duoi_tran)
print(tong_cuoi_quan_ly, tong_cuoi_ngay_tho)
print(quan_ly_tot_hon)
```

```python title=test
assert danh_sach_tong_ngay_tho == [36, 71, 105, 149, 183, 220], f"tong token ngay tho qua 6 luot sai -- dang ra {danh_sach_tong_ngay_tho}"
assert cac_cap_truoc_sau == [(149, 92), (126, 82), (119, 85)], f"cac cap (truoc,sau) khi kich hoat sai -- dang ra {cac_cap_truoc_sau}"
assert so_lan_kich_hoat == 3, f"so lan kich hoat tom tat qua 6 luot phai la 3 -- dang ra {so_lan_kich_hoat}"
assert tat_ca_duoi_tran == True, "moi lan kich hoat, token SAU phai <= TRAN -- khong duoc co ngoai le"
assert tong_cuoi_quan_ly == 85, f"token cuoi cung (co quan ly) phai la 85 -- dang ra {tong_cuoi_quan_ly}"
assert tong_cuoi_ngay_tho == 220, f"token cuoi cung (ngay tho) phai la 220 -- dang ra {tong_cuoi_ngay_tho}"
assert quan_ly_tot_hon == True, "quan_ly_tot_hon phai la True -- CA HAI bang chung (duoi tran VA tot hon ngay tho) deu phai dung"

# BANG CHUNG TRUNG TAM: ngay tho VUOT tran va TIEP TUC tang khong gioi han
assert tong_cuoi_ngay_tho > TRAN, f"ngay tho phai VUOT tran ({TRAN}) -- dang ra {tong_cuoi_ngay_tho}"
for i in range(1, len(danh_sach_tong_ngay_tho)):
    assert danh_sach_tong_ngay_tho[i] > danh_sach_tong_ngay_tho[i - 1], f"token ngay tho phai tang o MOI luot -- luot {i} khong tang so voi luot truoc"

# moi cap (truoc, sau) phai xac nhan: truoc > TRAN (ly do kich hoat), sau <= TRAN (ket qua)
for (truoc, sau) in cac_cap_truoc_sau:
    assert truoc > TRAN, f"kich hoat chi xay ra khi truoc > TRAN -- dang ra truoc={truoc}"
    assert sau <= TRAN, f"ngay sau kich hoat, sau phai <= TRAN -- dang ra sau={sau}"

# BIEN, RE (chi 3 luot dau, giu chi phi tinh toan thap): TRAN dung bang
# 105 (dung gia tri tong_truoc cua luot 3) -- luot 3 KHONG duoc kich hoat
# (105 khong > 105), xac nhan phep so sanh dung ">" chu khong phai ">="
_, cac_cap_bien = chay_hoi_thoai_quan_ly(CAU_HOI_6_LUOT[:3], 105, GIU_LAI_GAN_DAY, merges, token_sang_id)
assert cac_cap_bien == [], f"TRAN=105 (bang dung tong luot 3) khong duoc kich hoat -- dang ra {cac_cap_bien}"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ 1 là điều kiện KÍCH HOẠT tóm tắt — so sánh tổng token HIỆN TẠI (`tong_truoc`) với `tran` bằng `>` ("vượt qua", KHÔNG phải "từ trần trở lên"). Chỗ 2 xây LỊCH SỬ MỚI sau khi tóm tắt — phải gồm CẢ message tóm tắt (`tin_tom_tat`, đặt trong một danh sách một phần tử) LẪN các message gần đây giữ nguyên (`gan_day`), nối bằng `+`. Chỗ 3 kết luận CUỐI CÙNG — phải đúng CẢ HAI điều kiện (luôn dưới trần VÀ tốt hơn ngây thơ) cùng lúc, dùng `and`.
- kind: strategy
  body: 'Chỗ 1: `>` (`if tong_truoc > tran:`). Chỗ 2: `[tin_tom_tat] + gan_day`. Chỗ 3: `and` (`tat_ca_duoi_tran and (tong_cuoi_quan_ly < tong_cuoi_ngay_tho)`).'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `>`, `[tin_tom_tat] + gan_day`, và `and`.'
:::

:::validate
- tier: run
  timeoutMs: 15000
- tier: static
  onFail: dieu kien kich hoat phai dung dung ">" (khong duoc ">=" -- se kich hoat som hon can thiet o cac truong hop bien); lich su sau tom tat phai gom CA tin_tom_tat LAN gan_day (khong duoc thieu mot trong hai); ket luan cuoi phai la phep "and" THAT giua hai dieu kien (khong duoc "or" hay chep san True)
  requireAst:
  - kind: uses-operator, target: ">", min: 1
  - kind: uses-name, target: gan_day, min: 1
  - kind: uses-name, target: tin_tom_tat, min: 1
  - kind: uses-operator, target: and, min: 3
  - kind: uses-name, target: tat_ca_duoi_tran, min: 2
  - kind: uses-name, target: tong_cuoi_quan_ly, min: 2
  - kind: uses-operator, target: "<", min: 5
  # Da thu that (goi bo dem AST doc lap mo phong dung logic kiem-ast.ts tren
  # code trich tu solution): loi giai dung dat=true, tat ca bay luat qua
  # sach. ">"=1: XUAT HIEN DUY NHAT o cho trong 1 (khong co ">" nao khac
  # trong toan bo boilerplate BPE -- dem_cap_lien_ke/gop_cap/huan_luyen_bpe
  # chi dung "<", khong dung ">"). uses-name gan_day=1 va tin_tom_tat=1: ca
  # hai CHI xuat hien o cho trong 2 (dinh nghia "gan_day = ..." va
  # "tin_tom_tat = ..." la GAN, khong tinh la doc). and=3: hai lan CO SAN
  # (gop_cap: "i < len(danh_sach) - 1 and danh_sach[i] == a and
  # danh_sach[i + 1] == b" la MOT BoolOp voi 3 toan hang -- tinh la 1 nut,
  # khong phai 2; va tom_tat_mo_phong: "tu in noi_dung and tu not in
  # chu_de"), cong mot lan cho trong 3. tat_ca_duoi_tran=2: mot lan DOC o
  # cho trong 3 (ve trai cua "and"), mot lan DOC trong
  # "print(so_lan_kich_hoat, tat_ca_duoi_tran)" -- dinh nghia
  # "tat_ca_duoi_tran = all(...)" la GAN, khong tinh. tong_cuoi_quan_ly=2:
  # mot lan DOC o cho trong 3 (trong "tong_cuoi_quan_ly < ..."), mot lan DOC
  # trong "print(tong_cuoi_quan_ly, tong_cuoi_ngay_tho)". "<"=5: hai lan CO
  # SAN trong gop_cap ("i < len(danh_sach)" o dieu kien while, "i <
  # len(danh_sach) - 1" o dieu kien if), hai lan CO SAN trong huan_luyen_bpe
  # ("len(vocab) < muc_tieu_vocab", "tan_suat < 2"), cong mot lan cho trong
  # 3 ("tong_cuoi_quan_ly < tong_cuoi_ngay_tho") -- tong dung 5.
  #
  # Ba luat cuoi (tat_ca_duoi_tran, tong_cuoi_quan_ly, "<") sinh ra TU MOT
  # vong dot bien that (tools/kiem_dot_bien.mjs) da chay tren ban nhap dau
  # cua bai nay va bat duoc ba lo: doi TEN o cho trong 3 tu
  # "tat_ca_duoi_tran" thanh "tong_cuoi_quan_ly" (hoac nguoc lai) khong doi
  # gia tri quan_ly_tot_hon (vi tong_cuoi_quan_ly=85, mot so khac 0, la
  # truthy trong "and", va True<220 cung la True trong Python -- ca hai
  # bien the deu VO TINH cho cung ket qua True tren du lieu nay), va doi "<"
  # thanh "<=" cung khong doi gi (85<220 va 85<=220 deu True) -- ca ba deu
  # qua sach moi tang tests/output truoc khi them ba luat static nay. Sau
  # khi them, ca ba bi chan: doi ten tat_ca_duoi_tran lam dem no tut ve 1
  # (duoi nguong 2, vi chi con lai o print); doi ten tong_cuoi_quan_ly
  # tuong tu; doi "<" thanh "<=" lam dem "<" tut ve 4 (duoi nguong 5).
  #
  # Cheat ">=" thay ">" (cho trong 1) da tu kiem chung BANG PYTHON THAT: tren
  # kich ban CHINH (TRAN=110), KHONG co gia tri tong_truoc nao trong sau
  # (36, 71, 105, 149, 183, 220) BANG DUNG 110, nen ket qua GIONG HET loi
  # giai dung -- KHONG bi bat boi tests/output o kich ban chinh, CHI static
  # rieng moi bat (">"=0, duoi nguong 1). Day CHINH LA ly do truong hop bien
  # (chi 3 luot dau, TRAN=105, dung bang tong_truoc cua luot 3) ton tai doc
  # lap trong test: da tu kiem chung, cheat ">=" tren TRAN=105 (voi 3 luot)
  # kich hoat SOM o luot 3 (105>=105 dung), cho cac_cap_bien = [(105, 73)]
  # thay vi [] -- bi bat CA boi static (rieng) LAN boi assertion
  # "cac_cap_bien" (tests/output), phong thu kep. Truong hop bien nay CHI
  # dung 3 luot dau (khong phai ca 6) de giu chi phi tinh toan trong han
  # muc buoc cua sandbox -- da tu do bang sys.settrace THAT: huan luyen
  # (muc_tieu_vocab=40, 10 merge) + hai lan chay toan bo 6 luot (ngay tho
  # VA co quan ly) da het khoang 101.000/300.000 buoc; mot lan chay bien
  # CHI 3 luot cong them chi vai nghin buoc nua, con dung xa han muc.
  # Cheat "lich_su = gan_day" (bo tin_tom_tat, mat het thong tin cu, giong
  # het chien luoc CAT BO cua bai 2) lam uses-name tin_tom_tat tut ve 0 --
  # bi chan rieng; da tu kiem chung: cheat nay van cho sau <= tran (vi gan_day
  # con it token hon ca lich su co tom tat), nhung SAI VE Y NGHIA (day la
  # cat bo, khong phai tom tat) -- duoc static bat truoc khi can den kiem tra
  # ngu nghia sau hon. Cheat "lich_su = [tin_tom_tat]" (bo gan_day, mat luon
  # ca cau hoi vua hoi) lam uses-name gan_day tut ve 0 -- bi chan rieng; da
  # tu kiem chung: cheat nay lam cac_cap_truoc_sau co gia tri "sau" khac han
  # (chi con tin tom tat, khong con 2 message gan nhat), bi bat DOC LAP boi
  # assertion cac_cap_truoc_sau == [...].
  # Cheat "or" thay "and" (cho trong 3) da tu kiem chung: tren du lieu THAT,
  # tat_ca_duoi_tran=True VA (tong_cuoi_quan_ly < tong_cuoi_ngay_tho)=True,
  # nen "and"/"or" cho CUNG ket qua quan_ly_tot_hon=True -- KHONG bi bat boi
  # tests/output, CHI static rieng moi bat (and=2, duoi nguong 3).
- tier: tests
  timeoutMs: 15000
- tier: output
  match: regex
  expect: "^\\[36, 71, 105, 149, 183, 220\\]\\n\\[\\(149, 92\\), \\(126, 82\\), \\(119, 85\\)\\]\\n3 True\\n85 220\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`3` lần kích hoạt, token luôn về dưới `110` ngay lập tức, kết thúc ở `85`
— so với `220` của phiên bản ngây thơ, VƯỢT trần và cứ tăng mãi. Quest
`quan-ly-ngu-canh-va-hoi-thoai` (q8.4d) khép lại tại đây.
::::

::::reflect{#nghi-lai}
Năm bài, một câu chuyện xuyên suốt: đếm token bằng BPE THẬT (không phải
ký tự hay từ giả làm token), một cửa sổ ngữ cảnh luôn có TRẦN, hai cách
xử lý khi vượt trần (cắt bỏ — nhanh nhưng mất trắng; tóm tắt — tốn hơn
nhưng giữ lại cốt lõi), một bộ nhớ tích luỹ xuyên nhiều lượt để lượt sau
tham chiếu được lượt trước, và BOSS này: ráp cả bốn thành một vòng lặp
TỰ quản lý, đo bằng số cụ thể (`3` lần kích hoạt, luôn dưới trần, `85`
so với `220`) rằng quản lý ngữ cảnh không phải một chi tiết kỹ thuật phụ
— nó là thứ QUYẾT ĐỊNH một trợ lý hội thoại có còn dùng được sau vài
chục lượt hay không.
::::

::::checkpoint{mastery=0.9}
::::
