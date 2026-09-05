---
id: tri-tue-nhan-tao.token-hoa-bpe.lap-toi-vocab-muc-tieu
title: "Lặp tới kích thước từ vựng mục tiêu: vòng lặp BPE đầy đủ"
summary: "Cài huan_luyen_bpe: lặp lại (đếm cặp -> gộp cặp phổ biến nhất) cho tới khi vocab đạt mục tiêu HOẶC không còn cặp nào lặp lại. Với mục tiêu vocab=25 trên corpus 45 ký tự, vòng lặp DỪNG SỚM sau đúng 8 merge (vocab chỉ đạt 19, chưa chạm 25) vì hết cặp lặp lại -- so token cuối cùng còn 20. Danh sách 8 merge được GHI LẠI theo đúng thứ tự thực hiện -- quan trọng cho bài sau."
locale: vi
track: tri-tue-nhan-tao
module: token-hoa-bpe
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.lap-toi-vocab-muc-tieu]
requires: [ai.gop-cap-pho-bien-nhat]
concepts: [ai.lap-toi-vocab-muc-tieu]
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
Đếm cặp, gộp cặp — hai bài trước làm ĐÚNG MỘT vòng. BPE thật lặp lại việc
đó nhiều lần. Bài này ráp vòng lặp đầy đủ, và huấn luyện xong sẽ để lại
đúng một thứ quan trọng cho phần còn lại của quest: DANH SÁCH các merge đã
thực hiện, theo ĐÚNG thứ tự.
::::

::::explain{#vong_lap_bpe_day_du}
Huấn luyện BPE là lặp lại đúng hai bước của hai bài trước, nhiều lần liên
tiếp:

> **1. Đếm** mọi cặp liền kề trên danh sách token HIỆN TẠI (danh sách này
> THAY ĐỔI sau mỗi vòng — có thể chứa cả token đã gộp từ vòng trước, như
> `'n '`).
>
> **2. Gộp** cặp phổ biến nhất thành một token mới, cập nhật danh sách.

Lặp lại cho tới khi xảy ra MỘT trong hai điều kiện dừng:

> **Đạt kích thước từ vựng MỤC TIÊU** — số token DUY NHẤT (`vocab`, một
> tập hợp CHỈ TĂNG: mỗi lần gộp thêm ĐÚNG một token mới vào tập, các token
> cũ không hề bị xoá khỏi vocab dù chúng có còn xuất hiện trong corpus hay
> không) đã đạt số lượng mong muốn.
>
> **Dừng SỚM** — không còn cặp nào xuất hiện từ `2` lần trở lên (một cặp
> chỉ xuất hiện ĐÚNG `1` lần thì gộp nó không giúp ích gì — không có
> "lặp lại" nào để tận dụng). Với một corpus nhỏ, điều này có thể xảy ra
> TRƯỚC khi chạm kích thước từ vựng mục tiêu — và đó chính xác là điều sẽ
> xảy ra ở ví dụ dưới đây.

Danh sách các cặp đã gộp — theo ĐÚNG thứ tự thực hiện — phải được GHI LẠI
lại (`merges.append(cap_pho_bien)` sau mỗi vòng). Bài sau sẽ dùng đúng
danh sách này để mã hoá một câu HOÀN TOÀN MỚI: áp lại từng merge, theo
đúng thứ tự đã học, không phải thứ tự tuỳ ý.
::::

::::example{#huan_luyen_bpe_that}
Huấn luyện BPE trên corpus `45` ký tự, với kích thước từ vựng MỤC TIÊU là
`25` (cao hơn corpus nhỏ này có thể đạt tới):

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

corpus = "con meo an ca, con cho an com, con ga an thoc"
ds_cuoi, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=25)

print("so merge da thuc hien:", len(merges))
print("kich thuoc vocab cuoi:", len(vocab))
print("so token cuoi trong corpus:", len(ds_cuoi))
print("danh sach merge theo THU TU:", merges)
```

```text title=readonly
so merge da thuc hien: 8
kich thuoc vocab cuoi: 19
so token cuoi trong corpus: 20
danh sach merge theo THU TU: [('n', ' '), ('c', 'o'), ('co', 'n '), (' ', 'a'), (' a', 'n '), ('o', ' an '), (',', ' '), (', ', 'con ')]
```

Mục tiêu là `25`, nhưng vòng lặp DỪNG SỚM sau đúng `8` merge, vocab chỉ
đạt `19` — chưa chạm mục tiêu. Lý do: sau `8` lần gộp, không còn cặp nào
lặp lại từ `2` lần trở lên nữa trên corpus nhỏ này — không phải lỗi, mà
đúng điều kiện dừng thứ hai đã nêu ở trên. Với một corpus LỚN hơn (bài
BOSS cuối quest sẽ dùng một corpus vài trăm ký tự), mục tiêu vocab thường
đạt được TRƯỚC khi hết cặp lặp lại.
::::

::::predict{#doan_muc_tieu_thap_hon commitOnce}
Xét CÙNG corpus `45` ký tự, nhưng đặt mục tiêu vocab THẤP hơn nhiều —
`muc_tieu_vocab=14` thay vì `25`.

**Trước khi chạy thử**, bạn đoán: vòng lặp sẽ dừng vì lý do gì, và bao
nhiêu merge sẽ được thực hiện?

:::opt{correct}
Dừng vì ĐẠT MỤC TIÊU (không phải dừng sớm) — đúng `3` merge: vocab bắt đầu
ở `11` (số ký tự duy nhất), mỗi merge thêm đúng `1` token mới vào vocab,
nên sau `3` merge vocab đạt `11 + 3 = 14`, KHỚP đúng mục tiêu, vòng lặp
dừng trước khi kịp cạn cặp lặp lại (cạn cặp chỉ xảy ra sau `8` merge, như
ví dụ ở trên)
:::

:::opt
Dừng sớm giống hệt trường hợp mục tiêu `25` — vì corpus này CHỈ CÓ khả
năng thực hiện tối đa `8` merge, bất kể mục tiêu đặt ra là bao nhiêu
::why
Gần đúng ở việc `8` ĐÚNG LÀ số merge TỐI ĐA mà corpus này có thể thực hiện
trước khi cạn cặp lặp lại — quan sát đó không sai.

Chỗ lệch: với mục tiêu THẤP hơn `8` merge cần để cạn cặp, vòng lặp có thể
dừng SỚM HƠN vì đã ĐẠT MỤC TIÊU trước — hai điều kiện dừng là một cuộc
đua, điều kiện nào chạm TRƯỚC thì thắng. Mục tiêu `14` chỉ cần `3` merge
để đạt (`11 + 3 = 14`), nên vòng lặp dừng ở đó — SỚM HƠN nhiều so với việc
đợi cạn hết cặp lặp lại ở merge thứ `8`.
::
:::

:::opt
Không đủ thông tin để biết trước — số merge cần để đạt một mục tiêu vocab
cụ thể không thể tính được nếu chưa chạy thử
::why
Gần đúng ở việc thận trọng trước khi khẳng định một con số cụ thể mà chưa
chạy — tinh thần đó đúng nói chung.

Chỗ lệch: quan hệ giữa số merge và kích thước vocab ở đây có một QUY LUẬT
đơn giản có thể suy luận trước — mỗi merge làm `vocab` (một `set`) tăng
ĐÚNG `1` phần tử (token mới vừa tạo), không hơn không kém. Vậy số merge
cần để đạt mục tiêu `M`, xuất phát từ vocab cơ sở `11`, đơn giản là
`M − 11` — với `M = 14`, đó là `3`, HOÀN TOÀN suy ra được trước khi chạy,
miễn là mục tiêu đó đạt được TRƯỚC khi cạn cặp lặp lại.
::
:::
::::

::::code{#viet_huan_luyen_bpe}
Hoàn thiện `huan_luyen_bpe`: thêm token mới vào tập `vocab`, và ghi lại
cặp vừa gộp vào danh sách `merges`.

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
        vocab.___(cap_pho_bien[0] + cap_pho_bien[1])    # add
        merges.___(cap_pho_bien)                          # append
    return ds, merges, vocab

corpus = "con meo an ca, con cho an com, con ga an thoc"
ds_cuoi, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=25)

print(len(merges))
print(len(vocab))
print(len(ds_cuoi))
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

corpus = "con meo an ca, con cho an com, con ga an thoc"
ds_cuoi, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab=25)

print(len(merges))
print(len(vocab))
print(len(ds_cuoi))
```

```python title=test
assert len(merges) == 8, f"so merge phai la 8 -- dang ra {len(merges)}"
assert len(vocab) == 19, f"kich thuoc vocab cuoi phai la 19 -- dang ra {len(vocab)}"
assert len(ds_cuoi) == 20, f"so token cuoi trong corpus phai la 20 -- dang ra {len(ds_cuoi)}"
assert merges[0] == ('n', ' '), f"merge DAU TIEN phai la ('n', ' ') -- dang ra {merges[0]}"
assert merges[-1] == (', ', 'con '), f"merge CUOI CUNG phai la (', ', 'con ') -- dang ra {merges[-1]}"
assert merges == [('n', ' '), ('c', 'o'), ('co', 'n '), (' ', 'a'), (' a', 'n '), ('o', ' an '), (',', ' '), (', ', 'con ')], f"danh sach merge sai thu tu hoac noi dung -- dang ra {merges}"

# muc tieu THAP hon: phai dung vi DAT MUC TIEU, khong phai vi can cap
_, merges_14, vocab_14 = huan_luyen_bpe(corpus, muc_tieu_vocab=14)
assert len(merges_14) == 3, f"voi muc tieu 14, phai dung dung 3 merge -- dang ra {len(merges_14)}"
assert len(vocab_14) == 14, f"voi muc tieu 14, vocab cuoi phai dung 14 -- dang ra {len(vocab_14)}"

# bien: muc tieu BANG DUNG kich thuoc vocab co so (11) -- vong lap khong
# duoc chay merge nao ca (dieu kien "len(vocab) < muc_tieu_vocab" phai
# FALSE ngay tu dau)
_, merges_11, vocab_11 = huan_luyen_bpe(corpus, muc_tieu_vocab=11)
assert merges_11 == [], f"muc tieu bang dung vocab co so thi KHONG duoc merge nao ca -- dang ra {merges_11}"
assert len(vocab_11) == 11, f"vocab phai giu nguyen 11 khi muc tieu da dat tu dau -- dang ra {len(vocab_11)}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cả hai đều là PHƯƠNG THỨC gọi trên một cấu trúc dữ liệu. `vocab` là một `set` — thêm một phần tử MỚI vào `set` dùng phương thức `.add(...)`. `merges` là một `list` — thêm một phần tử vào CUỐI `list` dùng phương thức `.append(...)`.
- kind: strategy
  body: 'Chỗ 1: `vocab.add(cap_pho_bien[0] + cap_pho_bien[1])`. Chỗ 2: `merges.append(cap_pho_bien)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `add` và `append`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: huan_luyen_bpe phai goi THAT vocab.add(...) de them token moi vao tap vocab, VA merges.append(...) de ghi lai cap vua gop -- thieu mot trong hai se lam vong lap sai (vocab khong tang, hoac danh sach merge rong)
  requireAst:
  - kind: uses-call, target: add, min: 1
  - kind: uses-call, target: append, min: 3
  - kind: uses-call, target: get, min: 1
  # Da thu that (goi kiemAst that tren toan bo code trich tu solution, tinh
  # CA gop_cap/dem_cap_lien_ke vi requireAst quet toan bo nguon): loi giai
  # dung dat=true, ca ba luat qua sach. add=1: dung dung mot lan
  # (vocab.add(...) trong huan_luyen_bpe). append=3: HAI lan trong gop_cap
  # (ra.append(token_moi) va ra.append(danh_sach[i])) CONG mot lan trong
  # huan_luyen_bpe (merges.append(cap_pho_bien)) -- neu cho trong 2
  # (merges.append) bi thay bang mot cach khac (vi du merges = merges +
  # [cap_pho_bien], khong goi .append), dem append tut xuong con 2, bi
  # chan. get=1: dict.get trong dem_cap_lien_ke, dam bao ham do khong bi
  # sua thanh mot dang khac.
  #
  # Cheat "thieu vocab.add(...)" (bo cho trong 1, vi du gan '= None') lam
  # add tut ve 0 -- bi chan; da tu kiem chung: cheat nay lam vong lap KHONG
  # BAO GIO thoat theo dieu kien dat muc tieu (len(vocab) khong tang, nen
  # "< muc_tieu_vocab" luon dung), CHAY MAI cho toi khi dung vi cap cheat
  # (het cap lap lai) -- merges van dung 8 phan tu tren du lieu nay (vi
  # dieu kien dung con lai van hoat dong), NHUNG vocab cuoi CHI con 11 (khong
  # tang), bi bat boi assertion len(vocab) == 19. Cheat "thieu
  # merges.append(...)" lam append tut ve 2 -- bi chan; da tu kiem chung:
  # merges cuoi RONG, bi bat boi assertion merges[0].
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^8\\n19\\n20\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`8` merge, vocab từ `11` lên `19`, ghi lại theo đúng thứ tự. Bài sau dùng
CHÍNH danh sách này để mã hoá một câu HOÀN TOÀN MỚI — chưa từng xuất hiện
lúc huấn luyện.
::::

::::reflect{#nghi-lai}
Vòng lặp BPE giờ đã đầy đủ: đếm, gộp, lặp lại tới khi đạt mục tiêu hoặc
cạn cặp lặp lại. Danh sách `merges` — theo ĐÚNG thứ tự — là "kết quả huấn
luyện" thật sự của BPE, tương đương với trọng số đã học của một mạng
nơ-ron. Bài sau dùng danh sách này để mã hoá một câu MỚI, minh hoạ TẠI SAO
thứ tự của các merge lại quan trọng đến vậy.
::::

::::checkpoint{mastery=0.85}
::::
