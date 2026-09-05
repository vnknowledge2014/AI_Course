---
id: tri-tue-nhan-tao.quan-ly-ngu-canh-va-hoi-thoai.dem-token-that
title: "Đếm token thật: BPE, không phải ký tự hay từ"
summary: "Huấn luyện lại đúng huan_luyen_bpe (nguyên hàm từ q8.3a) trên một corpus hội thoại 368 ký tự (muc_tieu_vocab=90, dừng ở đúng 60 merge). Đếm SỐ TOKEN THẬT bằng len(ma_hoa_van_ban(...)), đối chiếu len(text) và len(text.split()) trên ba câu: 'gia banh mi bao nhieu tien' (26 ký tự, 6 từ, 8 token), 'gia ca phe la 25000 dong' (24 ký tự, 6 từ, 8 token), và 'tuyet voi' (9 ký tự, 2 từ -- CHƯA từng xuất hiện lúc huấn luyện, 8 token). Ba câu hoàn toàn khác độ dài và chủ đề đều cho ĐÚNG 8 token -- bằng chứng số thật rằng token BPE không suy ra được từ ký tự hay từ."
locale: vi
track: tri-tue-nhan-tao
module: quan-ly-ngu-canh-va-hoi-thoai
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.dem-token-that]
requires: [ai.boss-tro-ly-goi-cong-cu]
concepts: [ai.dem-token-that]
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
T8.3 xây BPE thật từ số `0`. T8.4 dùng LLM mô phỏng để học cơ chế prompt,
schema, công cụ. Bài này ráp HAI thứ đó lại: mọi phép "đếm token" từ giờ
trong track này phải chạy qua đúng pipeline BPE thật đã xây — không được
đếm ký tự, không được đếm từ giả làm token.
::::

::::explain{#vi_sao_phai_dem_token_that}
Một cửa sổ ngữ cảnh (context window — bài sau) có TRẦN đo bằng **token**,
không phải ký tự, không phải từ. Đếm sai đơn vị thì mọi phép quản lý ngữ
cảnh dựa trên con số đó đều sai theo.

Ba cách đếm nghe có vẻ "gần giống nhau" nhưng KHÔNG PHẢI CÙNG MỘT SỐ:

> **Đếm ký tự** — `len(van_ban)`. Đơn giản, nhưng không phản ánh cách một
> LLM thật "nhìn thấy" văn bản — một LLM không xử lý từng ký tự riêng lẻ.
>
> **Đếm từ** — `len(van_ban.split())`. Gần với trực giác con người hơn,
> nhưng vẫn SAI: BPE không tách theo khoảng trắng, nó gộp theo tần suất
> học được lúc huấn luyện — một token có thể CHỨA khoảng trắng bên trong
> (như bài `token-hoa-bpe`, q8.3a, đã đo: token `'con '` bao gồm cả dấu
> cách), và một "từ" có thể vỡ thành NHIỀU token nếu nó chưa từng xuất
> hiện đủ nhiều lúc huấn luyện để được gộp.
>
> **Đếm token BPE thật** — `len(ma_hoa_van_ban(van_ban, merges,
> token_sang_id))`, dùng ĐÚNG pipeline đã huấn luyện. Đây là con số MỘT
> LLM thật dùng để tính chi phí và để biết còn bao nhiêu chỗ trống trong
> cửa sổ ngữ cảnh của nó.

Ba con số này, trên cùng một câu, THƯỜNG khác nhau cả ba — không có công
thức nào suy ra số token từ số ký tự hay số từ mà không cần chạy qua BPE
thật.
::::

::::example{#dem_ba_cach_tren_cau_that}
Huấn luyện lại đúng `huan_luyen_bpe` (từ q8.3a) trên một corpus hội thoại
nhỏ, rồi đếm ba cách trên ba câu khác hẳn nhau:

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

def so_sanh_ba_cach_dem(van_ban, merges, token_sang_id):
    so_ky_tu = len(van_ban)
    so_tu = len(van_ban.split())
    so_token = len(ma_hoa_van_ban(van_ban, merges, token_sang_id))
    return {"ky_tu": so_ky_tu, "tu": so_tu, "token": so_token}

corpus = ("xin chao, toi la tro ly ao, rat vui duoc giup ban. "
          "gia banh mi bao nhieu tien. gia banh mi la 15000 dong. "
          "gia ca phe bao nhieu tien. gia ca phe la 25000 dong. "
          "gia tra sua bao nhieu tien. gia tra sua la 30000 dong. "
          "con cai kia thi sao. cam on ban rat nhieu, khong co chi. "
          "cuoc hoi thoai co the rat dai, can tom tat lich su cu "
          "de khong vuot qua cua so ngu canh cho phep.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=90)
token_sang_id = xay_token_sang_id(vocab)

s1 = "gia banh mi bao nhieu tien"
s2 = "gia ca phe la 25000 dong"
s3 = "tuyet voi"

print(so_sanh_ba_cach_dem(s1, merges, token_sang_id))
print(so_sanh_ba_cach_dem(s2, merges, token_sang_id))
print(so_sanh_ba_cach_dem(s3, merges, token_sang_id))
```

```text title=readonly
{'ky_tu': 26, 'tu': 6, 'token': 8}
{'ky_tu': 24, 'tu': 6, 'token': 8}
{'ky_tu': 9, 'tu': 2, 'token': 8}
```

Huấn luyện dừng ở đúng `60` merge, vocab đạt đúng mục tiêu `90`. Ba câu:
`s1` (`26` ký tự, `6` từ), `s2` (`24` ký tự, `6` từ — khác `s1` cả ký tự
lẫn nội dung), và `s3` (`"tuyet voi"`, chỉ `9` ký tự, `2` từ, CHƯA từng
xuất hiện trong corpus huấn luyện) — ba câu khác nhau hoàn toàn về độ dài
và chủ đề, vậy mà cả BA đều cho ĐÚNG `8` token. Không phải trùng hợp ngẫu
nhiên vô căn cứ: `s1`/`s2` được nén một phần nhờ các cụm quen thuộc
(`"gia"`, `"bao nhieu"`, `"dong"`) đã học lúc huấn luyện; `s3` gần như
KHÔNG được nén (chỉ giảm từ `9` ký tự xuống `8` token, đúng một merge duy
nhất áp dụng được) vì `"tuyet"` và `"voi"` chưa từng xuất hiện lúc huấn
luyện. Hai cơ chế hoàn toàn khác nhau tình cờ cho CÙNG một con số token —
đúng thứ mà đếm ký tự hay đếm từ không bao giờ tự suy ra được.
::::

::::predict{#doan_token_s2_vs_s3 commitOnce}
Xét `s2 = "gia ca phe la 25000 dong"` (`24` ký tự, `6` từ) và
`s3 = "tuyet voi"` (`9` ký tự, `2` từ) ở ví dụ trên.

**Trước khi chạy thử**, bạn đoán: số token BPE của `s2` so với `s3` —
lớn hơn, nhỏ hơn, hay bằng nhau?

:::opt{correct}
Bằng nhau — cả hai đều cho đúng `8` token, dù `s2` dài hơn `s3` gần gấp
ba lần về ký tự (`24` so với `9`) và nhiều gấp ba lần về từ (`6` so với
`2`); số token phụ thuộc vào việc các CỤM trong câu có khớp với những gì
đã học lúc huấn luyện hay không, không phụ thuộc độ dài câu
:::

:::opt
`s2` lớn hơn — vì `s2` dài hơn hẳn về cả ký tự lẫn số từ, và số token
luôn tỉ lệ thuận với độ dài câu
::why
Gần đúng ở việc `s2` THẬT SỰ dài hơn `s3` về cả hai thước đo thông thường
(ký tự, từ) — quan sát đó không sai.

Chỗ lệch: số token BPE không tỉ lệ thuận với độ dài câu theo ký tự hay
từ. `s2` chứa nhiều cụm QUEN THUỘC (`"gia"`, `"bao nhieu"`/`"la"`,
`"dong"` — đã xuất hiện lặp lại lúc huấn luyện) nên được NÉN mạnh; `s3`
gần như không được nén vì `"tuyet"`, `"voi"` là từ MỚI. Nén mạnh trên câu
dài và không nén trên câu ngắn có thể tình cờ CHẠM cùng một con số token
— đúng điều đã xảy ra ở đây.
::
:::

:::opt
`s3` lớn hơn — vì `s3` là câu MỚI, chưa từng xuất hiện lúc huấn luyện,
nên BPE luôn xử lý câu lạ bằng NHIỀU token hơn câu quen thuộc
::why
Gần đúng ở việc `s3` đúng là câu "lạ" với bộ huấn luyện — quan sát đó
đúng hướng.

Chỗ lệch: "câu lạ có nhiều token hơn CÂU BẤT KỲ khác" không phải một quy
luật cứng — nó chỉ đúng khi so với chính CHIỀU DÀI ký tự của câu đó (câu
lạ khó nén, nên token gần bằng ký tự). Khi so với một câu KHÁC dài hơn
nhiều nhưng được nén tốt (như `s1`, `s2`), số token tuyệt đối của câu lạ
ngắn hoàn toàn có thể BẰNG hoặc ít hơn — không có gì đảm bảo nó luôn lớn
hơn MỌI câu khác.
::
:::
::::

::::code{#viet_so_sanh_ba_cach_dem}
Hoàn thiện `so_sanh_ba_cach_dem`: đếm số từ bằng `split()`, và đếm số
token bằng đúng pipeline BPE thật (`ma_hoa_van_ban`) — không phải một
con số suy diễn từ ký tự hay từ.

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

def so_sanh_ba_cach_dem(van_ban, merges, token_sang_id):
    so_ky_tu = len(van_ban)
    so_tu = ___                                          # len(van_ban.split())
    so_token = ___                                        # len(ma_hoa_van_ban(van_ban, merges, token_sang_id))
    return {"ky_tu": so_ky_tu, "tu": so_tu, "token": so_token}

corpus = ("xin chao, toi la tro ly ao, rat vui duoc giup ban. "
          "gia banh mi bao nhieu tien. gia banh mi la 15000 dong. "
          "gia ca phe bao nhieu tien. gia ca phe la 25000 dong. "
          "gia tra sua bao nhieu tien. gia tra sua la 30000 dong. "
          "con cai kia thi sao. cam on ban rat nhieu, khong co chi. "
          "cuoc hoi thoai co the rat dai, can tom tat lich su cu "
          "de khong vuot qua cua so ngu canh cho phep.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=90)
token_sang_id = xay_token_sang_id(vocab)

s1 = "gia banh mi bao nhieu tien"
s2 = "gia ca phe la 25000 dong"
s3 = "tuyet voi"

print(so_sanh_ba_cach_dem(s1, merges, token_sang_id))
print(so_sanh_ba_cach_dem(s2, merges, token_sang_id))
print(so_sanh_ba_cach_dem(s3, merges, token_sang_id))
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

def so_sanh_ba_cach_dem(van_ban, merges, token_sang_id):
    so_ky_tu = len(van_ban)
    so_tu = len(van_ban.split())
    so_token = len(ma_hoa_van_ban(van_ban, merges, token_sang_id))
    return {"ky_tu": so_ky_tu, "tu": so_tu, "token": so_token}

corpus = ("xin chao, toi la tro ly ao, rat vui duoc giup ban. "
          "gia banh mi bao nhieu tien. gia banh mi la 15000 dong. "
          "gia ca phe bao nhieu tien. gia ca phe la 25000 dong. "
          "gia tra sua bao nhieu tien. gia tra sua la 30000 dong. "
          "con cai kia thi sao. cam on ban rat nhieu, khong co chi. "
          "cuoc hoi thoai co the rat dai, can tom tat lich su cu "
          "de khong vuot qua cua so ngu canh cho phep.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=90)
token_sang_id = xay_token_sang_id(vocab)

s1 = "gia banh mi bao nhieu tien"
s2 = "gia ca phe la 25000 dong"
s3 = "tuyet voi"

print(so_sanh_ba_cach_dem(s1, merges, token_sang_id))
print(so_sanh_ba_cach_dem(s2, merges, token_sang_id))
print(so_sanh_ba_cach_dem(s3, merges, token_sang_id))
```

```python title=test
assert len(merges) == 60, f"so merge phai la 60 -- dang ra {len(merges)}"
assert len(vocab) == 90, f"vocab cuoi phai la 90 -- dang ra {len(vocab)}"

kq1 = so_sanh_ba_cach_dem(s1, merges, token_sang_id)
kq2 = so_sanh_ba_cach_dem(s2, merges, token_sang_id)
kq3 = so_sanh_ba_cach_dem(s3, merges, token_sang_id)

assert kq1 == {"ky_tu": 26, "tu": 6, "token": 8}, f"ket qua s1 sai -- dang ra {kq1}"
assert kq2 == {"ky_tu": 24, "tu": 6, "token": 8}, f"ket qua s2 sai -- dang ra {kq2}"
assert kq3 == {"ky_tu": 9, "tu": 2, "token": 8}, f"ket qua s3 sai -- dang ra {kq3}"

# BANG CHUNG TRUNG TAM: hai cau khac han nhau (do dai, chu de) van cho DUNG
# cung mot so token -- token khong suy ra duoc tu ky tu hay tu
assert kq1["token"] == kq3["token"], f"s1 va s3 phai cho CUNG so token (8) -- dang ra {kq1['token']} va {kq3['token']}"
assert kq1["ky_tu"] != kq3["ky_tu"], "s1 va s3 phai KHAC nhau ve so ky tu (de chung minh token khong ti le voi ky tu)"
assert kq1["tu"] != kq3["tu"], "s1 va s3 phai KHAC nhau ve so tu (de chung minh token khong ti le voi tu)"

# so token khong duoc bang so ky tu (tru truong hop khong nen duoc gi ca)
assert kq1["token"] != kq1["ky_tu"], f"token cua s1 khong duoc trung ky tu -- ca hai deu la {kq1['token']}"
assert kq2["token"] != kq2["ky_tu"], f"token cua s2 khong duoc trung ky tu -- ca hai deu la {kq2['token']}"

# bien: chuoi RONG phai cho ca ba so deu la 0, khong loi
kq_rong = so_sanh_ba_cach_dem("", merges, token_sang_id)
assert kq_rong == {"ky_tu": 0, "tu": 0, "token": 0}, f"chuoi rong phai cho ca ba so la 0 -- dang ra {kq_rong}"

# mot tu don, chua tung gap du -- token PHAI bang dung ky tu (khong duoc nen gi)
kq_xoai = so_sanh_ba_cach_dem("xoai", merges, token_sang_id)
assert kq_xoai == {"ky_tu": 4, "tu": 1, "token": 4}, f"'xoai' phai cho token bang dung ky tu (chua tung gop) -- dang ra {kq_xoai}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ 1 đếm số TỪ bằng cách tách chuỗi theo khoảng trắng — `van_ban.split()` trả về một danh sách các từ, đếm độ dài danh sách đó bằng `len(...)`. Chỗ 2 đếm số TOKEN THẬT — không phải một phép tính suy diễn, mà phải GỌI `ma_hoa_van_ban(van_ban, merges, token_sang_id)` (đúng pipeline BPE đã huấn luyện) rồi lấy độ dài kết quả bằng `len(...)`.
- kind: strategy
  body: 'Chỗ 1: `len(van_ban.split())`. Chỗ 2: `len(ma_hoa_van_ban(van_ban, merges, token_sang_id))`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `len(van_ban.split())` và `len(ma_hoa_van_ban(van_ban, merges, token_sang_id))`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: so_tu phai goi THAT van_ban.split() (khong duoc doan hay chep san mot con so), VA so_token phai goi THAT ma_hoa_van_ban(...) (khong duoc dung len(van_ban) hay len(van_ban.split()) lam gia -- do la chinh xac dieu bai nay day tranh)
  requireAst:
  - kind: uses-call, target: split, min: 1
  - kind: uses-call, target: ma_hoa_van_ban, min: 1
  - kind: uses-call, target: len, min: 7
  # Da thu that (goi kiemAst that tren code trich tu solution, dung mot bo
  # dem AST doc lap mo phong dung logic cua kiem-ast.ts): loi giai dung dat
  # true, ca ba luat qua sach. split=1: dung mot lan trong "van_ban.split()"
  # (cho trong 1). ma_hoa_van_ban=1: dung mot lan GOI THAT ben trong
  # so_sanh_ba_cach_dem (cho trong 2) -- dinh nghia "def ma_hoa_van_ban"
  # khong tinh la mot Call. len=7: rai rac o dem_cap_lien_ke (1), gop_cap (2
  # lan trong dieu kien while/if), huan_luyen_bpe (1, dieu kien while), va BA
  # lan trong so_sanh_ba_cach_dem (so_ky_tu=len(van_ban), cho trong 1, cho
  # trong 2) -- tong dung 7.
  #
  # Cheat "so_tu = 6" (chep san so tu, dung voi ca s1 va s2 nhung SAI voi s3
  # va chuoi rong) lam split tut ve 0 VA len tut ve 6 -- bi chan boi CA HAI
  # luat rieng; da tu kiem chung bang Python that: cheat nay lam kq3["tu"]
  # tra ve 6 thay vi 2 (sai), bi bat CA boi static LAN boi assertion
  # kq3 == {...}.
  # Cheat "so_token = so_ky_tu" (gia lam token bang dung so ky tu, dung tinh
  # co ca hai deu la 8 tren s1... that ra SAI ngay ca o do vi kq1["ky_tu"]=26
  # khac kq1["token"]=8) lam ma_hoa_van_ban tut ve 0 VA len tut ve 6 -- bi
  # chan; da tu kiem chung: cheat nay cho kq1={"ky_tu":26,"tu":6,"token":26}
  # (token bang thang ky tu, sai hoan toan so voi 8 mong doi), bi bat DOC LAP
  # boi assertion kq1 == {...} VA boi assertion "kq1['token'] != kq1['ky_tu']".
  # Cheat "so_token = len(van_ban.split())" (dung so tu gia lam token) KHONG
  # lam giam so len (van con 7, vi van la mot loi goi len(...) hop le) nhung
  # lam ma_hoa_van_ban tut ve 0 -- bi chan RIENG boi luat nay; da tu kiem
  # chung: cheat nay cho kq1["token"]=6 (bang so tu) thay vi 8, bi bat boi
  # assertion kq1 == {...}.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\{'ky_tu': 26, 'tu': 6, 'token': 8\\}\\n\\{'ky_tu': 24, 'tu': 6, 'token': 8\\}\\n\\{'ky_tu': 9, 'tu': 2, 'token': 8\\}\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`8` token cho cả ba câu — dù độ dài ký tự và số từ khác nhau hoàn toàn.
Từ đây, mọi phép "đếm ngữ cảnh" trong quest này đều dùng đúng con số này,
không phải ký tự hay từ giả làm token.
::::

::::reflect{#nghi-lai}
Ba cách đếm nghe tương tự nhau nhưng đo ba thứ khác nhau: ký tự đo ĐỘ
DÀI CHUỖI, từ đo NGẮT THEO KHOẢNG TRẮNG, còn token đo CÁCH MỘT BPE ĐÃ
HUẤN LUYỆN thực sự chia nhỏ văn bản — thứ duy nhất một LLM thật quan tâm
khi tính chi phí và khi kiểm tra còn bao nhiêu chỗ trống trong cửa sổ
ngữ cảnh của nó. Bài sau dùng đúng con số này để mô phỏng một cửa sổ
ngữ cảnh có TRẦN — và cắt bớt lịch sử khi vượt trần.
::::

::::checkpoint{mastery=0.85}
::::
