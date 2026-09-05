---
id: tri-tue-nhan-tao.quan-ly-ngu-canh-va-hoi-thoai.tom-tat-lich-su-cu
title: "Tóm tắt lịch sử cũ: nén thay vì xoá sạch"
summary: "Thay vì CẮT BỎ 4 message cũ nhất (mất trắng, bài trước), NÉN chúng thành MỘT message tóm tắt qua tom_tat_mo_phong -- một hàm Python thuần tra bảng từ khoá cố định (KHÔNG phải LLM thật tóm tắt), biến 4 message thành câu 'tom tat lich su cu, da hoi gia banh mi.'. Đo bằng token thật: TRƯỚC (8 message) = 83 token; SAU (1 tóm tắt + 4 gần đây) = 50 token -- giảm 33 token, nhưng vẫn LỚN HƠN nếu xoá sạch hoàn toàn (28 token) -- tóm tắt giữ lại một phần thông tin, khác cắt bỏ ở bài trước."
locale: vi
track: tri-tue-nhan-tao
module: quan-ly-ngu-canh-va-hoi-thoai
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.tom-tat-lich-su-cu]
requires: [ai.cua-so-ngu-canh-va-cat-bot]
concepts: [ai.tom-tat-lich-su-cu]
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
Bài trước cắt bớt — hiệu quả, nhưng XOÁ SẠCH những message bị cắt. Nếu
message cũ đó có thông tin còn cần dùng sau này (như bài `bộ nhớ hội
thoại` sắp tới), xoá sạch là mất trắng. Bài này thử một cách khác: NÉN
thay vì XOÁ.
::::

::::explain{#nen_thay_vi_xoa}
**Tóm tắt lịch sử cũ** thay N message cũ bằng ĐÚNG MỘT message tóm tắt,
thay vì xoá hẳn chúng. Ý tưởng: một message tóm tắt ngắn tốn ÍT token hơn
nhiều so với giữ nguyên N message gốc, nhưng vẫn còn NHIỀU HƠN không
token nào cả — nó giữ lại một phần cốt lõi.

Track này KHÔNG gọi một LLM thật để tóm tắt (không mạng, không API thật —
nguyên tắc xuyên suốt T8.4). Thay vào đó, `tom_tat_mo_phong` là một **hàm
Python thuần, tra bảng từ khoá CỐ ĐỊNH biết trước**:

> Quét nội dung của N message cũ, tìm xem những TỪ KHOÁ đã biết trước
> (tên sản phẩm, trong ví dụ này) có xuất hiện hay không. Ghép các từ
> khoá tìm được thành MỘT câu tóm tắt cố định — không sáng tạo câu chữ
> mới, không suy luận ngữ nghĩa, chỉ TRA và GHÉP theo luật đã lập trình
> sẵn.

Đây KHÔNG phải một phép tóm tắt "thông minh" — nó là một QUY TẮC đơn
giản, tất định, luôn cho CÙNG một kết quả với cùng đầu vào. Nhưng nó đủ
để minh hoạ đúng CƠ CHẾ: một message tóm tắt có thể thay thế nhiều
message cũ, giảm token đáng kể mà không mất trắng hoàn toàn.
::::

::::example{#tom_tat_that_tren_lich_su}
Tóm tắt `4` message đầu tiên (chào hỏi + hỏi giá bánh mì) thành MỘT
message, giữ nguyên `4` message gần đây (hỏi giá cà phê, trà sữa):

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

TU_KHOA_SAN_PHAM = ["banh mi", "ca phe", "tra sua"]

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

corpus = ("xin chao, toi la tro ly ao, rat vui duoc giup ban. "
          "gia banh mi bao nhieu tien. gia banh mi la 15000 dong. "
          "gia ca phe bao nhieu tien. gia ca phe la 25000 dong. "
          "gia tra sua bao nhieu tien. gia tra sua la 30000 dong. "
          "con cai kia thi sao. cam on ban rat nhieu, khong co chi. "
          "cuoc hoi thoai co the rat dai, can tom tat lich su cu "
          "de khong vuot qua cua so ngu canh cho phep.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=90)
token_sang_id = xay_token_sang_id(vocab)

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

cu = lich_su[:4]
gan_day = lich_su[4:]
tin_tom_tat = tom_tat_mo_phong(cu)
lich_su_sau = [tin_tom_tat] + gan_day

truoc = dem_token_lich_su(lich_su, merges, token_sang_id)
sau = dem_token_lich_su(lich_su_sau, merges, token_sang_id)
xoa_sach = dem_token_lich_su(gan_day, merges, token_sang_id)

print("tin tom tat:", tin_tom_tat["content"])
print("truoc:", truoc, "so message:", len(lich_su))
print("sau tom tat:", sau, "so message:", len(lich_su_sau))
print("neu xoa sach (khong tom tat):", xoa_sach, "so message:", len(gan_day))
```

```text title=readonly
tin tom tat: tom tat lich su cu, da hoi gia banh mi.
truoc: 83 so message: 8
sau tom tat: 50 so message: 5
neu xoa sach (khong tom tat): 28 so message: 4
```

`83` token trước, `50` token sau khi tóm tắt — giảm `33` token, còn `5`
message thay vì `8`. Nhưng nếu XOÁ SẠCH `4` message cũ (không tóm tắt gì
cả, đúng chiến lược bài trước), chỉ còn `28` token — ÍT HƠN `50`. Tóm tắt
tốn NHIỀU HƠN xoá sạch (`50 > 28`, vì message tóm tắt vẫn có nội dung),
nhưng vẫn ÍT HƠN NHIỀU so với giữ nguyên toàn bộ (`50 < 83`) — nó giữ lại
đúng MỘT PHẦN, không mất trắng, không giữ hết.
::::

::::predict{#doan_sau_so_voi_xoa_sach commitOnce}
Xét đúng lịch sử `8` message ở ví dụ trên.

**Trước khi chạy thử**, bạn đoán: tổng token SAU KHI TÓM TẮT (`1` message
tóm tắt + `4` message gần đây) so với tổng token nếu XOÁ SẠCH hoàn toàn
`4` message cũ đó (chỉ giữ `4` message gần đây, không tóm tắt) — cái nào
LỚN HƠN?

:::opt{correct}
Tóm tắt LỚN HƠN xoá sạch — vì xoá sạch giữ `0` token cho phần cũ, còn tóm
tắt vẫn giữ lại MỘT message chứa nội dung (`"tom tat lich su cu, da hoi
gia banh mi."`), tốn một số token khác `0`; đổi lại, tóm tắt KHÔNG MẤT
TRẮNG thông tin đã hỏi trước đó như xoá sạch làm
:::

:::opt
Tóm tắt NHỎ HƠN xoá sạch — vì mục đích của tóm tắt là GIẢM token, nên nó
phải luôn cho kết quả nhỏ hơn MỌI chiến lược khác, kể cả xoá sạch hoàn
toàn
::why
Gần đúng ở việc tóm tắt ĐÚNG LÀ để giảm token so với GIỮ NGUYÊN toàn bộ
— mục tiêu đó không sai.

Chỗ lệch: "giảm token" không có nghĩa là giảm NHIỀU HƠN MỌI chiến lược
khác, kể cả xoá sạch. Xoá sạch là trường hợp CỰC ĐOAN nhất — nó bỏ tất
cả, giữ đúng `0` token cho phần cũ. Không có chiến lược nào GIỮ LẠI THÔNG
TIN (dù chỉ một câu tóm tắt ngắn) có thể tốn ÍT HƠN xoá sạch hoàn toàn —
tóm tắt luôn phải trả một cái giá bằng token để đổi lấy việc không mất
trắng.
::
:::

:::opt
Bằng nhau — vì cả hai đều xử lý `4` message cũ theo một cách nào đó, kết
quả token phải như nhau bất kể có tóm tắt hay không
::why
Gần đúng ở việc cả hai chiến lược đều XỬ LÝ đúng `4` message cũ như
nhau — điểm khởi đầu giống nhau.

Chỗ lệch: "xử lý" theo hai cách RẤT khác nhau: xoá sạch bỏ nội dung
hoàn toàn (thay `4` message bằng KHÔNG GÌ CẢ); tóm tắt thay `4` message
bằng MỘT message MỚI có nội dung riêng (`"tom tat lich su cu, da hoi gia
banh mi."`, dài `31` ký tự). Hai kết quả có nội dung khác nhau thì số
token đếm được cũng khác nhau — không có lý do gì để bằng nhau.
::
:::
::::

::::code{#viet_tom_tat_mo_phong}
Hoàn thiện `tom_tat_mo_phong`: thêm từ khoá tìm được vào danh sách chủ
đề, rồi ghép các chủ đề đó thành một câu bằng dấu nối `" va "`.

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

TU_KHOA_SAN_PHAM = ["banh mi", "ca phe", "tra sua"]

def tom_tat_mo_phong(cac_message_cu):
    chu_de = []
    for tin in cac_message_cu:
        noi_dung = tin["content"].lower()
        for tu in TU_KHOA_SAN_PHAM:
            if tu in noi_dung and tu not in chu_de:
                chu_de.___(tu)                                     # append
    if not chu_de:
        noi_dung_tom_tat = "tom tat lich su cu, khong co san pham nao duoc hoi."
    else:
        noi_dung_tom_tat = "tom tat lich su cu, da hoi gia " + ___.join(chu_de) + "."   # " va "
    return {"role": "system", "content": noi_dung_tom_tat}

corpus = ("xin chao, toi la tro ly ao, rat vui duoc giup ban. "
          "gia banh mi bao nhieu tien. gia banh mi la 15000 dong. "
          "gia ca phe bao nhieu tien. gia ca phe la 25000 dong. "
          "gia tra sua bao nhieu tien. gia tra sua la 30000 dong. "
          "con cai kia thi sao. cam on ban rat nhieu, khong co chi. "
          "cuoc hoi thoai co the rat dai, can tom tat lich su cu "
          "de khong vuot qua cua so ngu canh cho phep.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=90)
token_sang_id = xay_token_sang_id(vocab)

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

cu = lich_su[:4]
gan_day = lich_su[4:]
tin_tom_tat = tom_tat_mo_phong(cu)
lich_su_sau = [tin_tom_tat] + gan_day

truoc = dem_token_lich_su(lich_su, merges, token_sang_id)
sau = dem_token_lich_su(lich_su_sau, merges, token_sang_id)

print(truoc, len(lich_su))
print(sau, len(lich_su_sau))
print(tin_tom_tat["content"])
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

TU_KHOA_SAN_PHAM = ["banh mi", "ca phe", "tra sua"]

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

corpus = ("xin chao, toi la tro ly ao, rat vui duoc giup ban. "
          "gia banh mi bao nhieu tien. gia banh mi la 15000 dong. "
          "gia ca phe bao nhieu tien. gia ca phe la 25000 dong. "
          "gia tra sua bao nhieu tien. gia tra sua la 30000 dong. "
          "con cai kia thi sao. cam on ban rat nhieu, khong co chi. "
          "cuoc hoi thoai co the rat dai, can tom tat lich su cu "
          "de khong vuot qua cua so ngu canh cho phep.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=90)
token_sang_id = xay_token_sang_id(vocab)

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

cu = lich_su[:4]
gan_day = lich_su[4:]
tin_tom_tat = tom_tat_mo_phong(cu)
lich_su_sau = [tin_tom_tat] + gan_day

truoc = dem_token_lich_su(lich_su, merges, token_sang_id)
sau = dem_token_lich_su(lich_su_sau, merges, token_sang_id)

print(truoc, len(lich_su))
print(sau, len(lich_su_sau))
print(tin_tom_tat["content"])
```

```python title=test
assert truoc == 83, f"tong token truoc phai la 83 -- dang ra {truoc}"
assert sau == 50, f"tong token sau tom tat phai la 50 -- dang ra {sau}"
assert len(lich_su_sau) == 5, f"so message sau tom tat phai la 5 (1 tom tat + 4 gan day) -- dang ra {len(lich_su_sau)}"
assert tin_tom_tat == {"role": "system", "content": "tom tat lich su cu, da hoi gia banh mi."}, f"tin tom tat sai -- dang ra {tin_tom_tat}"

# so sanh voi xoa sach hoan toan (chien luoc bai truoc) -- tom tat phai
# GIAM DANG KE so voi truoc, nhung KHONG duoc bang xoa sach (phai > 0,
# giu lai mot phan thong tin)
xoa_sach = dem_token_lich_su(gan_day, merges, token_sang_id)
assert xoa_sach == 28, f"neu xoa sach hoan toan (bai truoc) phai la 28 token -- dang ra {xoa_sach}"
assert sau < truoc, f"tom tat phai giam so voi truoc -- sau={sau}, truoc={truoc}"
assert sau > xoa_sach, f"tom tat phai giu lai NHIEU HON xoa sach hoan toan -- sau={sau}, xoa_sach={xoa_sach}"

# bien: khong co san pham nao duoc nhac toi trong cac message cu
tin_rong = tom_tat_mo_phong([{"role": "user", "content": "xin chao ban"}])
assert tin_rong == {"role": "system", "content": "tom tat lich su cu, khong co san pham nao duoc hoi."}, f"tin tom tat khi khong co san pham sai -- dang ra {tin_rong}"

# bien: danh sach RONG cung phai cho ket qua nhu tren, khong loi
tin_danh_sach_rong = tom_tat_mo_phong([])
assert tin_danh_sach_rong == tin_rong, "danh sach rong phai cho cung ket qua voi khong co san pham nao"

# bien: ca BA san pham, CO LAP LAI -- khong duoc trung lap trong cau tom tat
tin_ba_sp = tom_tat_mo_phong([
    {"role": "user", "content": "gia banh mi bao nhieu tien"},
    {"role": "user", "content": "gia banh mi bao nhieu tien"},
    {"role": "user", "content": "gia ca phe bao nhieu tien"},
    {"role": "user", "content": "gia tra sua bao nhieu tien"},
])
assert tin_ba_sp["content"] == "tom tat lich su cu, da hoi gia banh mi va ca phe va tra sua.", f"tom tat ca ba san pham (khong trung lap) sai -- dang ra {tin_ba_sp['content']!r}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ 1 thêm từ khoá vừa tìm thấy vào danh sách `chu_de` — dùng phương thức `.append(...)` của `list`. Chỗ 2 nối các phần tử của `chu_de` thành MỘT chuỗi, xen giữa chúng bằng chữ `" va "` — dùng `" va ".join(chu_de)` (một chuỗi gọi `.join(...)` trên MỘT danh sách khác, không phải ngược lại).
- kind: strategy
  body: 'Chỗ 1: `chu_de.append(tu)`. Chỗ 2: `" va ".join(chu_de)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `chu_de.append(tu)` và `" va ".join(chu_de)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: tom_tat_mo_phong phai goi THAT chu_de.append(tu) de tich luy tung tu khoa tim duoc (khong duoc gan lai bang mot cach khac), VA phai goi THAT mot phep join de ghep danh sach chu_de thanh chuoi
  requireAst:
  - kind: uses-call, target: append, min: 4
  - kind: uses-call, target: join, min: 2
  # Da thu that (goi bo dem AST doc lap mo phong dung logic kiem-ast.ts tren
  # code trich tu solution): loi giai dung dat=true, ca hai luat qua sach.
  # append=4: hai lan CO SAN trong gop_cap (ra.append(token_moi),
  # ra.append(danh_sach[i])), mot lan CO SAN trong huan_luyen_bpe
  # (merges.append(cap_pho_bien)), cong mot lan la cho trong 1
  # (chu_de.append(tu)) -- tong dung 4. Neu cho trong 1 doi thanh
  # "chu_de += [tu]" (khong goi .append), dem nay tut xuong con 3 -- duoi
  # nguong 4, bi chan.
  # join=2: mot lan CO SAN trong dem_token_lich_su (" ".join(...)), cong mot
  # lan la cho trong 2 (" va ".join(chu_de)) -- tong dung 2. Neu cho trong 2
  # bi thay bang mot cach ghep chuoi khac khong goi join (vi du vong lap
  # cong chuoi thu cong), dem nay tut xuong con 1 -- duoi nguong 2, bi chan.
  #
  # Luu y: chuoi phan cach chinh xac (" va ", co dau cach hai dau) KHONG co
  # luat has-literal rieng -- muc tieu AST voi target ket thuc bang dau
  # cach bi bo qua boi bo phan tich schema (dau cach cuoi chuoi YAML bi
  # cat), giong okay da ghi nhan o q8.4a. Dien bua cho trong 2 thanh
  # ", ".join(chu_de) (dau phan cach SAI) khong doi so dem join (van la 2)
  # nhung da tu kiem chung bang Python that: cheat nay lam tin_ba_sp["content"]
  # tra ve "tom tat lich su cu, da hoi gia banh mi, ca phe, tra sua." (sai
  # dau noi), bi bat DOC LAP boi assertion tin_ba_sp/tests.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^83 8\\n50 5\\ntom tat lich su cu, da hoi gia banh mi\\.\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`83` xuống `50` — vẫn giảm đáng kể, nhưng KHÔNG mất trắng như xoá bỏ
(`28`). Bài sau chuyển hướng: một hàm/qui trình giữ TRẠNG THÁI hội thoại
xuyên NHIỀU lượt hỏi-đáp, để lượt sau tham chiếu được ngữ cảnh của lượt
trước.
::::

::::reflect{#nghi-lai}
Tóm tắt đánh đổi: tốn nhiều token hơn xoá sạch (`50` so với `28`), đổi
lại giữ được một phần cốt lõi thay vì mất trắng. Với một corpus huấn
luyện đủ tốt, một hàm tóm tắt mô phỏng — dù chỉ là tra bảng từ khoá cố
định, không phải một LLM thật suy luận — vẫn đo được cải thiện CỤ THỂ
bằng số. Bài sau dùng đúng ý tưởng "giữ lại thông tin cũ" này cho một
mục đích khác: một hàm nhớ được CÂU HỎI và CÂU TRẢ LỜI của những lượt
trước, để lượt sau có thể tham chiếu lại.
::::

::::checkpoint{mastery=0.85}
::::
