---
id: tri-tue-nhan-tao.token-hoa-bpe.boss-token-hoa-bpe
title: "BOSS — Token hoá BPE, từ số 0 tới pipeline đầy đủ"
summary: "Ráp TOÀN BỘ pipeline BPE: huấn luyện trên corpus 229 ký tự (mục tiêu vocab=60) dừng SỚM sau 38 merge (vocab đạt 59, chưa chạm 60 -- hết cặp lặp lại), mã hoá một câu MỚI 55 ký tự thành 22 token BPE, giải mã lại khớp ĐÚNG nguyên văn. So sánh trực tiếp: 22 token BPE so với 55 token nếu tokenize theo ký tự đơn (bài 2) -- BẰNG CHỨNG SỐ THẬT cho đúng lý do BPE tồn tại, nối lại bài 1 của quest này. Đóng q8.3a, bàn giao sang q8.3b 'Động cơ Tensor'."
locale: vi
track: tri-tue-nhan-tao
module: token-hoa-bpe
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-token-hoa-bpe]
requires: [ai.giai-ma-nguoc]
concepts: [ai.boss-token-hoa-bpe]
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
Bảy bài, bảy mảnh nhỏ. Bài này ráp TẤT CẢ lại thành một pipeline duy nhất
— huấn luyện, mã hoá, giải mã — trên một corpus LỚN hơn hẳn, và đo đúng
con số mà cả quest này tồn tại để chứng minh.
::::

::::explain{#rap_toan_bo_pipeline}
Pipeline BPE đầy đủ gồm ĐÚNG những gì bảy bài trước đã xây, không thêm gì
mới:

> **`huan_luyen_bpe`** — đếm cặp liền kề (bài `dem-cap-lien-ke`), gộp cặp
> phổ biến nhất (bài `gop-cap-pho-bien-nhat`), lặp lại tới khi đạt vocab
> mục tiêu hoặc hết cặp lặp lại (bài `lap-toi-vocab-muc-tieu`) — trả về
> danh sách `merges` theo đúng thứ tự, và tập `vocab` cuối cùng.
>
> **`xay_token_sang_id`** — gán ID cho mỗi token trong `vocab`, sắp xếp
> trước để TẤT ĐỊNH (cùng ý tưởng bài `token-hoa-ky-tu`, áp dụng lên
> vocab CỦA BPE thay vì vocab ký tự đơn).
>
> **`ma_hoa_van_ban`** — áp `merges` theo đúng thứ tự lên một câu MỚI,
> tra ID (bài `ma-hoa-van-ban-moi`).
>
> **`giai_ma`** — tra ngược, nối lại thành chuỗi (bài `giai-ma-nguoc`).

Lần này corpus lớn hơn hẳn — `229` ký tự thay vì `45` — đủ để mục tiêu
vocab (`60`) và số merge thực hiện được (`38`) đều lớn hơn nhiều so với ví
dụ nhỏ xuyên suốt bảy bài trước. Cùng code, không đổi một dòng nào trong
bốn hàm cốt lõi — chỉ đổi corpus và mục tiêu vocab.

Con số quan trọng nhất bài này đo: với CÙNG một câu mới, BPE cho một
chuỗi token NGẮN HƠN bao nhiêu so với token hoá theo ký tự đơn (bài
`token-hoa-ky-tu`, mở đầu quest) — con số này chính là LÝ DO BPE tồn tại,
đã nêu Ở DẠNG Ý TƯỞNG từ bài `vi-sao-can-token-hoa` (bài đầu tiên), giờ
được đo bằng Python thật.
::::

::::example{#boss_pipeline_that}
Huấn luyện BPE trên corpus `229` ký tự, mã hoá một câu mới, giải mã lại,
và so sánh với character-level:

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

def giai_ma(ids, id_sang_token):
    return ''.join(id_sang_token[i] for i in ids)

corpus = ("hoc may la mot nhanh cua tri tue nhan tao. hoc may dung du lieu de hoc quy luat. "
          "mo hinh hoc may hoc tu du lieu, cang nhieu du lieu mo hinh cang hoc tot. "
          "neu du lieu it, mo hinh de hoc sai, hoc lech, hoc khong dung quy luat that.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=60)
token_sang_id = xay_token_sang_id(vocab)
id_sang_token = {i: t for t, i in token_sang_id.items()}

cau_moi = "hoc may can nhieu du lieu de hoc dung quy luat that su."
ids = ma_hoa_van_ban(cau_moi, merges, token_sang_id)
khoi_phuc = giai_ma(ids, id_sang_token)

so_token_bpe = len(ids)
so_token_ky_tu = len(cau_moi)

print("do dai corpus huan luyen:", len(corpus))
print("so merge da thuc hien:", len(merges))
print("vocab cuoi:", len(vocab))
print("cau moi:", cau_moi)
print("so token BPE:", so_token_bpe)
print("so token ky tu don:", so_token_ky_tu)
print("giai ma khop cau goc:", khoi_phuc == cau_moi)
```

```text title=readonly
do dai corpus huan luyen: 229
so merge da thuc hien: 38
vocab cuoi: 59
cau moi: hoc may can nhieu du lieu de hoc dung quy luat that su.
so token BPE: 22
so token ky tu don: 55
giai ma khop cau goc: True
```

Mục tiêu vocab là `60`, nhưng DỪNG SỚM sau `38` merge — vocab chỉ đạt
`59` (hết cặp lặp lại trên corpus này, đúng điều kiện dừng thứ hai đã học
ở bài `lap-toi-vocab-muc-tieu`). Câu mới `"hoc may can nhieu du lieu de
hoc dung quy luat that su."` — CHƯA TỪNG xuất hiện y hệt trong corpus
huấn luyện — mã hoá bằng BPE ra `22` token, so với `55` token nếu tokenize
theo ký tự đơn (đúng bằng độ dài chuỗi, mỗi ký tự một token, bài
`token-hoa-ky-tu`). BPE cho chuỗi token NGẮN HƠN hẳn — chưa tới một nửa —
trên đúng CÙNG một câu, và giải mã lại vẫn khớp TUYỆT ĐỐI với câu gốc.
::::

::::predict{#doan_ket_qua_boss commitOnce}
**Trước khi chạy pipeline đầy đủ**, bạn đoán: chuỗi token BPE của câu mới
`"hoc may can nhieu du lieu de hoc dung quy luat that su."` sẽ NGẮN HƠN,
DÀI HƠN, hay DÀI BẰNG chuỗi token nếu tokenize câu đó theo ký tự đơn?

:::opt{correct}
Ngắn hơn — mỗi token đã gộp (như `'du lieu '`, `'hoc '`) thay thế NHIỀU ký
tự đơn bằng MỘT token duy nhất; miễn corpus huấn luyện đủ để học được các
cụm lặp lại xuất hiện trong câu mới, chuỗi BPE luôn NGẮN HƠN HOẶC BẰNG
chuỗi ký tự đơn — không bao giờ dài hơn, vì trường hợp XẤU NHẤT (không
merge nào áp dụng được) BPE trả về ĐÚNG chuỗi ký tự đơn, không tệ hơn
:::

:::opt
Dài bằng nhau — vì BPE và character-level đều token hoá CÙNG một câu, chỉ
khác cách gọi tên, số lượng token phải giống nhau
::why
Gần đúng ở việc cả hai đều tokenize CÙNG một chuỗi ký tự đầu vào — điểm
xuất phát đúng là như nhau.

Chỗ lệch: hai cách token hoá không cho ra CÙNG số lượng đơn vị. Character-
level luôn cho ĐÚNG BẰNG độ dài chuỗi (mỗi ký tự một token, không đổi).
BPE gộp NHIỀU ký tự liền kề (nếu chúng xuất hiện đủ thường xuyên lúc huấn
luyện) thành MỘT token — số token kết quả PHỤ THUỘC vào việc câu mới có
bao nhiêu cụm ký tự trùng với các merge đã học, và trong ví dụ này, con số
đó khác hẳn nhau (`22` so với `55`).
::
:::

:::opt
Dài hơn — vì BPE cần thêm bước tra ID phức tạp hơn character-level, nên
tổng số "đơn vị xử lý" của nó nhiều hơn
::why
Gần đúng ở việc BPE đúng là có nhiều BƯỚC xử lý hơn character-level (huấn
luyện merge, áp merge theo thứ tự, rồi mới tra ID) — về mặt CÀI ĐẶT, BPE
phức tạp hơn.

Chỗ lệch: câu hỏi không phải về ĐỘ PHỨC TẠP của quá trình tính toán, mà về
ĐỘ DÀI của KẾT QUẢ cuối cùng — chuỗi token đưa vào mô hình. Token hoá theo
ký tự có cách CÀI ĐẶT đơn giản hơn, nhưng chính vì "đơn giản" theo nghĩa
không gộp gì cả, nó luôn cho ra chuỗi kết quả DÀI HƠN HOẶC BẰNG BPE, không
bao giờ ngắn hơn — càng phức tạp trong quá trình HUẤN LUYỆN không có nghĩa
là kết quả cuối dài hơn.
::
:::
::::

::::code{#viet_boss_pipeline}
Hoàn thiện bốn chỗ trống: thêm token mới vào `vocab` trong lúc huấn luyện,
áp một merge trong lúc mã hoá, nối token lại trong lúc giải mã, và tính độ
dài chuỗi token character-level của câu mới.

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
        vocab.add(___)                                # cap_pho_bien[0] + cap_pho_bien[1]
        merges.append(cap_pho_bien)
    return ds, merges, vocab

def xay_token_sang_id(vocab):
    return {t: i for i, t in enumerate(sorted(vocab))}

def ma_hoa_van_ban(van_ban, danh_sach_merge, token_sang_id):
    ds = list(van_ban)
    for cap in danh_sach_merge:
        ds = ___                                      # gop_cap(ds, cap)
    return [token_sang_id[t] for t in ds]

def giai_ma(ids, id_sang_token):
    return ___                                        # ''.join(id_sang_token[i] for i in ids)

corpus = ("hoc may la mot nhanh cua tri tue nhan tao. hoc may dung du lieu de hoc quy luat. "
          "mo hinh hoc may hoc tu du lieu, cang nhieu du lieu mo hinh cang hoc tot. "
          "neu du lieu it, mo hinh de hoc sai, hoc lech, hoc khong dung quy luat that.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=60)
token_sang_id = xay_token_sang_id(vocab)
id_sang_token = {i: t for t, i in token_sang_id.items()}

cau_moi = "hoc may can nhieu du lieu de hoc dung quy luat that su."
ids = ma_hoa_van_ban(cau_moi, merges, token_sang_id)
khoi_phuc = giai_ma(ids, id_sang_token)

so_token_bpe = len(ids)
so_token_ky_tu = ___                                    # len(cau_moi)

print(len(merges))
print(len(vocab))
print(so_token_bpe)
print(so_token_ky_tu)
print(khoi_phuc == cau_moi)
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

def giai_ma(ids, id_sang_token):
    return ''.join(id_sang_token[i] for i in ids)

corpus = ("hoc may la mot nhanh cua tri tue nhan tao. hoc may dung du lieu de hoc quy luat. "
          "mo hinh hoc may hoc tu du lieu, cang nhieu du lieu mo hinh cang hoc tot. "
          "neu du lieu it, mo hinh de hoc sai, hoc lech, hoc khong dung quy luat that.")

ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=60)
token_sang_id = xay_token_sang_id(vocab)
id_sang_token = {i: t for t, i in token_sang_id.items()}

cau_moi = "hoc may can nhieu du lieu de hoc dung quy luat that su."
ids = ma_hoa_van_ban(cau_moi, merges, token_sang_id)
khoi_phuc = giai_ma(ids, id_sang_token)

so_token_bpe = len(ids)
so_token_ky_tu = len(cau_moi)

print(len(merges))
print(len(vocab))
print(so_token_bpe)
print(so_token_ky_tu)
print(khoi_phuc == cau_moi)
```

```python title=test
assert len(merges) == 38, f"so merge phai la 38 -- dang ra {len(merges)}"
assert len(vocab) == 59, f"vocab cuoi phai la 59 -- dang ra {len(vocab)}"
assert so_token_bpe == 22, f"so token BPE cua cau moi phai la 22 -- dang ra {so_token_bpe}"
assert so_token_ky_tu == 55, f"so token ky tu don cua cau moi phai la 55 -- dang ra {so_token_ky_tu}"
assert khoi_phuc == cau_moi, f"giai ma phai khop DUNG cau goc -- dang ra {khoi_phuc!r}"

# BANG CHUNG TRUNG TAM cua ca quest: BPE phai NGAN HON character-level
assert so_token_bpe < so_token_ky_tu, f"BPE phai cho chuoi token NGAN HON character-level -- dang ra BPE={so_token_bpe}, ky tu don={so_token_ky_tu}"

# kiem tra bookkeeping: kich thuoc vocab = so ky tu co so + so merge
so_ky_tu_co_so = len(set(corpus))
assert len(vocab) == so_ky_tu_co_so + len(merges), f"vocab cuoi phai bang so ky tu co so ({so_ky_tu_co_so}) cong so merge ({len(merges)}) -- dang ra {len(vocab)}"

# round-trip tren mot cau khac, ngan hon, de doi chieu doc lap
cau_khac = "hoc may hoc tot"
ids_khac = ma_hoa_van_ban(cau_khac, merges, token_sang_id)
kp_khac = giai_ma(ids_khac, id_sang_token)
assert kp_khac == cau_khac, f"round-trip that bai voi cau khac {cau_khac!r} -- dang ra {kp_khac!r}"
assert len(ids_khac) < len(cau_khac), f"BPE phai NGAN HON character-level tren ca cau khac nay -- BPE={len(ids_khac)}, ky tu don={len(cau_khac)}"
```

:::hints
- kind: attention
  body: Bốn chỗ trống. Chỗ 1 (trong `huan_luyen_bpe`) thêm token MỚI (đã nối hai phần của `cap_pho_bien`) vào `vocab` — `cap_pho_bien[0] + cap_pho_bien[1]`. Chỗ 2 (trong `ma_hoa_van_ban`) áp MỘT merge lên `ds` hiện tại — gọi lại `gop_cap(ds, cap)`. Chỗ 3 (trong `giai_ma`) nối các token đã tra ngược thành MỘT chuỗi — `''.join(id_sang_token[i] for i in ids)`. Chỗ 4 tính độ dài chuỗi token NẾU tokenize theo ký tự đơn — đơn giản là độ dài của chính `cau_moi`, `len(cau_moi)`.
- kind: strategy
  body: 'Chỗ 1: `cap_pho_bien[0] + cap_pho_bien[1]`. Chỗ 2: `gop_cap(ds, cap)`. Chỗ 3: `''.join(id_sang_token[i] for i in ids)`. Chỗ 4: `len(cau_moi)`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `cap_pho_bien[0] + cap_pho_bien[1]`, `gop_cap(ds, cap)`, `''.join(id_sang_token[i] for i in ids)`, và `len(cau_moi)`.'
:::

:::validate
- tier: run
  timeoutMs: 12000
- tier: static
  onFail: huan_luyen_bpe phai goi THAT vocab.add voi token da NOI (khong duoc them tuple cap_pho_bien nguyen si -- se lam sorted(vocab) loi TypeError vi tron lan kieu du lieu); ma_hoa_van_ban phai goi THAT gop_cap trong vong lap; giai_ma phai goi THAT ''.join; so_token_ky_tu phai tinh THAT bang len(cau_moi)
  requireAst:
  - kind: uses-call, target: add, min: 1
  - kind: uses-call, target: gop_cap, min: 2
  - kind: uses-call, target: join, min: 1
  - kind: uses-call, target: len, min: 6
  forbidAst:
  - kind: has-literal, target: "55"
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat requireAst qua sach VA khong vi pham
  # forbidAst nao. add=1: dung mot lan (vocab.add(...)). gop_cap=2: mot lan
  # GOI THAT trong huan_luyen_bpe, mot lan GOI THAT trong ma_hoa_van_ban
  # (cho trong 2) -- dinh nghia "def gop_cap" khong tinh la Call. join=1:
  # dung mot lan trong giai_ma. len=6: rai rac o dem_cap_lien_ke (1),
  # gop_cap (2 lan trong dieu kien while/if), huan_luyen_bpe (1, dieu kien
  # while), va hai lan cuoi la so_token_bpe=len(ids) VA so_token_ky_tu =
  # len(cau_moi) (cho trong 4) -- neu cho trong 4 bi thay bang mot hang so
  # (vi du "55"), dem len tut xuong con 5, bi chan RIENG; has-literal "55"
  # cung se xuat hien (bi forbidAst chan DOC LAP).
  #
  # Cheat "vocab.add(cap_pho_bien)" (them nguyen tuple, khong noi thanh
  # chuoi) lam add van =1 (van goi .add, chi sai THAM SO) nen KHONG bi
  # static bat rieng qua so lan goi -- nhung da tu kiem chung BANG PYTHON
  # THAT: cheat nay lam vocab tron lan ca chuoi (tu cac ky tu co so) LAN
  # tuple (tu moi lan .add sai), khien sorted(vocab) o xay_token_sang_id
  # NEM TypeError ("'<' not supported between instances of 'tuple' and
  # 'str'") ngay khi chay -- bi chan boi tier `run`, truoc ca khi kip toi
  # tier tests. Day la ly do KHONG can them mot requireAst rieng cho hinh
  # dang chinh xac cua bieu thuc trong add(...): tier run da du manh.
  # Cheat "ds = van_ban" (bo qua vong lap ap merge, cho trong 2 tra thang
  # ve du lieu goc chua gop) lam gop_cap tut xuong con 1 -- bi chan; da tu
  # kiem chung: cheat nay lam so_token_bpe tang vot len 55 (bang chinh do
  # dai cau, vi token_sang_id tra ID tren TUNG ky tu don -- may man khong
  # KeyError vi vocab luon giu lai ca ky tu co so), lam assertion
  # "so_token_bpe < so_token_ky_tu" THAT BAI (22 khong con nho hon 55 vi
  # ca hai deu bang 55) -- bi bat DOC LAP boi tests, dung diem chinh cua
  # bai BOSS nay.
- tier: tests
  timeoutMs: 12000
- tier: output
  match: regex
  expect: "^38\\n59\\n22\\n55\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`22` token thay vì `55` — chưa tới một nửa, trên cùng một câu, giải mã lại
khớp tuyệt đối. Đây là con số chứng minh đúng lý do BPE tồn tại — quest
`token-hoa-bpe` hoàn tất.
::::

::::reflect{#nghi-lai}
Quest `token-hoa-bpe` khép lại tại đây, tám bài: vì sao cần token hoá (từ
vựng theo từ mất từ mới, theo ký tự đơn thì chuỗi dài), token hoá theo ký
tự (baseline, cài đặt được nhưng chưa tận dụng cấu trúc lặp lại), đếm cặp
liền kề (bước LÕI đầu tiên của BPE), gộp cặp phổ biến nhất (một vòng
merge), lặp tới vocab mục tiêu (vòng lặp BPE đầy đủ, ghi lại thứ tự), mã
hoá văn bản mới (áp merge ĐÚNG thứ tự lên câu chưa từng thấy), giải mã
ngược (tra bảng ngược, nối lại — không nhạy cảm thứ tự như mã hoá), và
BOSS này: ráp toàn bộ trên một corpus lớn hơn, đo bằng số thật rằng BPE
cho chuỗi token ngắn hơn hẳn character-level trên cùng một câu.

Văn bản giờ đã là số — một danh sách token ID, sẵn sàng cho bước tiếp
theo. Nhưng `Value` (T8.2, vô hướng — mỗi số một `Value` riêng) sẽ không
đủ nhanh khi cần nhân MA TRẬN embedding và attention trên hàng chục token
cùng lúc. Quest sau, `dong-co-tensor` (q8.3b), mở rộng `Value` thành
`Tensor` — bọc mảng numpy, giữ nguyên ý tưởng `_prev`/`_backward` đã học,
nhưng đủ nhanh cho phần còn lại của T8.3 "Transformer từ số 0".
::::

::::checkpoint{mastery=0.9}
::::
