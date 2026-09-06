---
id: tri-tue-nhan-tao.chunking-va-embedding-tu-che.vector-tu-che-dem-tu
title: "Vector tự chế: đếm từ trên một từ vựng cố định"
summary: "Mot TU_VUNG co dinh 16 tu (8 ve cong nghe, 8 ve am thuc). Hai doan A1/A2 (cung chu de cong nghe) cho vector dem tu voi DUNG 4 chi so cung khac 0 (may_tinh, phan_mem, lap_trinh, du_lieu); doan B1 (am thuc) so voi A1 cho DUNG 0 chi so cung khac 0. 4 > 0 -- do bang so THAT, khong suy doan: hai doan cung chu de co vector 'giong nhau' hon han hai doan khac chu de. Vector nay la EMBEDDING TU CHE (dem tu tren tu vung nho, KHONG phai word2vec/BERT/mo hinh embedding that da huan luyen truoc) -- day CO CHE, khong day chat luong ngu nghia san xuat."
locale: vi
track: tri-tue-nhan-tao
module: chunking-va-embedding-tu-che
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.vector-tu-che-dem-tu]
requires: [ai.cua-so-truot-co-chong-lap]
concepts: [ai.vector-tu-che-dem-tu]
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
Hai bài trước đã chia tài liệu thành đoạn. Nhưng máy tính không so sánh
được hai CHUỖI KÝ TỰ theo nghĩa "gần nhau về nội dung" — nó chỉ so được số.
Bài này biến mỗi đoạn văn bản thành một dãy số.
::::

::::explain{#embedding_tu_che_la_gi}
Một "embedding" — trong công nghệ RAG THẬT ngoài đời — là một vector số đại
diện cho ý NGHĨA của một đoạn văn bản, sinh ra bởi một mô hình đã được huấn
luyện trên hàng tỷ câu (word2vec, BERT, hay các mô hình embedding của những
nhà cung cấp lớn). Sandbox của track này chạy trong trình duyệt qua Pyodide
(WASM) — KHÔNG có mạng, không thể tải bất kỳ mô hình embedding THẬT nào đã
huấn luyện sẵn.

**Vì vậy toàn bộ track `T8.5` dùng một "embedding" TỰ CHẾ**: một vector đếm
từ có trọng số (term-frequency vector) trên một TỪ VỰNG CỐ ĐỊNH, nhỏ, biết
trước. Đây KHÔNG phải kỹ thuật embedding hiện đại — nó không "hiểu" nghĩa
của từ, không biết `"máy tính"` và `"thiết bị điện toán"` là gần nghĩa nhau
nếu cả hai không cùng xuất hiện NGUYÊN VĂN trong từ vựng. Mục tiêu sư phạm
của bài này không phải dạy CHẤT LƯỢNG ngữ nghĩa của một mô hình embedding
sản xuất thật — mà dạy đúng CƠ CHẾ: một đoạn văn bản → một vector số có độ
dài CỐ ĐỊNH, và hai đoạn "gần chủ đề" cho ra hai vector "gần nhau" theo một
cách đo được (bài sau).

Cách xây: chọn trước một **từ vựng cố định** (một danh sách từ/cụm từ liên
quan tới chủ đề tài liệu, ví dụ `16` mục). Với mỗi đoạn văn bản, ĐẾM số lần
MỖI mục trong từ vựng xuất hiện trong đoạn đó — kết quả là một vector số
nguyên có độ dài ĐÚNG BẰNG kích thước từ vựng (`16` số cho từ vựng `16` mục,
bất kể đoạn văn dài hay ngắn). Hai đoạn có nhiều mục từ vựng CHUNG cùng xuất
hiện sẽ cho vector với nhiều CHỈ SỐ khác `0` GIỐNG NHAU — một dấu hiệu (thô,
không hoàn hảo, nhưng đo được) rằng chúng cùng chủ đề.
::::

::::example{#tinh_vector_hai_chu_de}
Một từ vựng cố định `16` mục, nửa đầu về công nghệ, nửa sau về ẩm thực. Ba
đoạn văn bản: `DOAN_A1`, `DOAN_A2` (cùng chủ đề công nghệ), `DOAN_B1` (chủ đề
ẩm thực):

```python title=readonly
TU_VUNG = [
    "may_tinh", "phan_mem", "lap_trinh", "du_lieu", "vi_xu_ly", "ket_noi_mang", "thuat_toan", "ung_dung",
    "mon_an", "cong_thuc", "gia_vi", "nha_bep", "dau_bep", "thuc_pham", "nau_an", "mon_trang_mieng",
]

CUM_TU_GOC = {
    "may_tinh": "may tinh", "phan_mem": "phan mem", "lap_trinh": "lap trinh", "du_lieu": "du lieu",
    "vi_xu_ly": "vi xu ly", "ket_noi_mang": "ket noi mang", "thuat_toan": "thuat toan", "ung_dung": "ung dung",
    "mon_an": "mon an", "cong_thuc": "cong thuc", "gia_vi": "gia vi", "nha_bep": "nha bep",
    "dau_bep": "dau bep", "thuc_pham": "thuc pham", "nau_an": "nau an", "mon_trang_mieng": "mon trang mieng",
}


def chuan_hoa_cum_tu(van_ban, cum_tu_goc):
    kq = van_ban
    for token, cum in cum_tu_goc.items():
        kq = kq.replace(cum, token)
    return kq


def tinh_vector_dem_tu(van_ban, tu_vung, cum_tu_goc):
    vb = chuan_hoa_cum_tu(van_ban.lower(), cum_tu_goc)
    cac_tu = vb.split()
    return [cac_tu.count(tu) for tu in tu_vung]


def dem_vi_tri_cung_khac_0(v1, v2):
    return sum(1 for a, b in zip(v1, v2) if a > 0 and b > 0)


DOAN_A1 = "may tinh hien dai chay phan mem duoc viet bang ngon ngu lap trinh xu ly du lieu qua vi xu ly manh me"
DOAN_A2 = "lap trinh vien viet phan mem de may tinh xu ly du lieu nhanh hon ung dung thuat toan toi uu"
DOAN_B1 = "mon an ngon phu thuoc vao gia vi va cong thuc nau dau bep gioi trong nha bep sach se tao ra mon trang mieng hap dan"

vA1 = tinh_vector_dem_tu(DOAN_A1, TU_VUNG, CUM_TU_GOC)
vA2 = tinh_vector_dem_tu(DOAN_A2, TU_VUNG, CUM_TU_GOC)
vB1 = tinh_vector_dem_tu(DOAN_B1, TU_VUNG, CUM_TU_GOC)

print(vA1)
print(vA2)
print(vB1)
print(dem_vi_tri_cung_khac_0(vA1, vA2))
print(dem_vi_tri_cung_khac_0(vA1, vB1))
```

```text title=readonly
[1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
[1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]
[0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1]
4
0
```

`vA1` và `vA2` (cùng chủ đề công nghệ) cùng khác `0` ở đúng `4` chỉ số:
`may_tinh`, `phan_mem`, `lap_trinh`, `du_lieu` — cả hai đoạn đều dùng những
từ này. `vA1` và `vB1` (chủ đề khác nhau) cùng khác `0` ở `0` chỉ số — không
một mục từ vựng công nghệ nào xuất hiện trong đoạn ẩm thực, và ngược lại.
`4 > 0` — hai đoạn cùng chủ đề cho vector "giống nhau" hơn hẳn hai đoạn khác
chủ đề, đo được bằng số nguyên, không suy đoán. `chuan_hoa_cum_tu` làm việc
đơn giản: thay mỗi cụm từ gốc (`"may tinh"`, `"gia vi"`, …) bằng một TOKEN
liền (`"may_tinh"`, `"gia_vi"`, …) trước khi tách từ và đếm — nếu không có
bước này, `.split()` sẽ tách `"may tinh"` thành hai từ riêng (`"may"`,
`"tinh"`), không khớp được với mục từ vựng `"may_tinh"`.
::::

::::predict{#doan_so_chi_so_cung_khac_0 commitOnce}
Xét đúng ba vector `vA1`, `vA2`, `vB1` ở ví dụ trên — `vA1`/`vA2` cùng chủ
đề công nghệ, `vB1` chủ đề ẩm thực.

**Trước khi chạy thử**, bạn đoán: `dem_vi_tri_cung_khac_0(vA1, vB1)` (giữa
hai đoạn KHÁC chủ đề) là bao nhiêu?

:::opt{correct}
`0` — không có mục từ vựng nào trong `8` mục "công nghệ" xuất hiện ở đoạn ẩm
thực (`DOAN_B1`), và không mục nào trong `8` mục "ẩm thực" xuất hiện ở đoạn
công nghệ (`DOAN_A1`); hai nửa từ vựng hoàn toàn tách biệt theo chủ đề của
hai đoạn này
:::

:::opt
`4` — bằng đúng số chỉ số cùng khác `0` giữa `vA1` và `vA2`, vì mọi cặp
đoạn văn bản đều có một mức độ trùng từ vựng cơ bản giống nhau
::why
Gần đúng ở việc `4` đúng là con số cho MỘT cặp cụ thể (`vA1`/`vA2`, cùng chủ
đề) — quan sát đó không sai cho cặp đó.

Chỗ lệch: không có gì đảm bảo MỌI cặp đoạn đều trùng từ vựng ở cùng MỘT mức
độ. Từ vựng ở đây được thiết kế chia rõ `8` mục công nghệ và `8` mục ẩm
thực — không mục nào xuất hiện ở CẢ hai đoạn thuộc hai chủ đề khác nhau,
nên số chỉ số cùng khác `0` giữa một cặp KHÁC chủ đề rơi xuống đúng `0`,
không phải một con số cố định lặp lại cho mọi cặp.
::
:::

:::opt
`8` — vì `vA1` và `vB1` cùng có tổng cộng `16` chỉ số, và một nửa trong số
đó (do từ vựng chia đôi `8`/`8`) sẽ trùng khớp lẫn nhau
::why
Gần đúng ở việc từ vựng THẬT SỰ chia đôi `8` mục mỗi chủ đề — quan sát cấu
trúc từ vựng đó đúng.

Chỗ lệch: "chia đôi từ vựng" không có nghĩa là "một nửa số chỉ số sẽ trùng
khớp". `dem_vi_tri_cung_khac_0` chỉ đếm chỉ số nào CẢ HAI vector đều khác
`0` — mà `vA1` chỉ khác `0` ở `5` chỉ số công nghệ, `vB1` chỉ khác `0` ở
`6` chỉ số ẩm thực, và HAI tập chỉ số đó (công nghệ và ẩm thực) không giao
nhau — nên số chỉ số CHUNG khác `0` là `0`, không phải một nửa của `16`.
::
:::
::::

::::code{#viet_tinh_vector_dem_tu}
Hoàn thiện `chuan_hoa_cum_tu` (thay cụm từ gốc bằng token trước khi đếm) và
`tinh_vector_dem_tu` (đếm mỗi mục từ vựng trong đoạn đã chuẩn hoá).

```python title=starter
TU_VUNG = [
    "may_tinh", "phan_mem", "lap_trinh", "du_lieu", "vi_xu_ly", "ket_noi_mang", "thuat_toan", "ung_dung",
    "mon_an", "cong_thuc", "gia_vi", "nha_bep", "dau_bep", "thuc_pham", "nau_an", "mon_trang_mieng",
]

CUM_TU_GOC = {
    "may_tinh": "may tinh", "phan_mem": "phan mem", "lap_trinh": "lap trinh", "du_lieu": "du lieu",
    "vi_xu_ly": "vi xu ly", "ket_noi_mang": "ket noi mang", "thuat_toan": "thuat toan", "ung_dung": "ung dung",
    "mon_an": "mon an", "cong_thuc": "cong thuc", "gia_vi": "gia vi", "nha_bep": "nha bep",
    "dau_bep": "dau bep", "thuc_pham": "thuc pham", "nau_an": "nau an", "mon_trang_mieng": "mon trang mieng",
}


def chuan_hoa_cum_tu(van_ban, cum_tu_goc):
    kq = van_ban
    for token, cum in cum_tu_goc.items():
        kq = kq.replace(___, ___)                          # cum, token
    return kq


def tinh_vector_dem_tu(van_ban, tu_vung, cum_tu_goc):
    vb = chuan_hoa_cum_tu(van_ban.lower(), cum_tu_goc)
    cac_tu = vb.split()
    return [cac_tu.count(___) for tu in tu_vung]            # tu


def dem_vi_tri_cung_khac_0(v1, v2):
    return sum(1 for a, b in zip(v1, v2) if a > 0 and b > 0)


DOAN_A1 = "may tinh hien dai chay phan mem duoc viet bang ngon ngu lap trinh xu ly du lieu qua vi xu ly manh me"
DOAN_A2 = "lap trinh vien viet phan mem de may tinh xu ly du lieu nhanh hon ung dung thuat toan toi uu"
DOAN_B1 = "mon an ngon phu thuoc vao gia vi va cong thuc nau dau bep gioi trong nha bep sach se tao ra mon trang mieng hap dan"

vA1 = tinh_vector_dem_tu(DOAN_A1, TU_VUNG, CUM_TU_GOC)
vA2 = tinh_vector_dem_tu(DOAN_A2, TU_VUNG, CUM_TU_GOC)
vB1 = tinh_vector_dem_tu(DOAN_B1, TU_VUNG, CUM_TU_GOC)

print(vA1)
print(vA2)
print(vB1)
print(dem_vi_tri_cung_khac_0(vA1, vA2))
print(dem_vi_tri_cung_khac_0(vA1, vB1))
```

```python title=solution
TU_VUNG = [
    "may_tinh", "phan_mem", "lap_trinh", "du_lieu", "vi_xu_ly", "ket_noi_mang", "thuat_toan", "ung_dung",
    "mon_an", "cong_thuc", "gia_vi", "nha_bep", "dau_bep", "thuc_pham", "nau_an", "mon_trang_mieng",
]

CUM_TU_GOC = {
    "may_tinh": "may tinh", "phan_mem": "phan mem", "lap_trinh": "lap trinh", "du_lieu": "du lieu",
    "vi_xu_ly": "vi xu ly", "ket_noi_mang": "ket noi mang", "thuat_toan": "thuat toan", "ung_dung": "ung dung",
    "mon_an": "mon an", "cong_thuc": "cong thuc", "gia_vi": "gia vi", "nha_bep": "nha bep",
    "dau_bep": "dau bep", "thuc_pham": "thuc pham", "nau_an": "nau an", "mon_trang_mieng": "mon trang mieng",
}


def chuan_hoa_cum_tu(van_ban, cum_tu_goc):
    kq = van_ban
    for token, cum in cum_tu_goc.items():
        kq = kq.replace(cum, token)
    return kq


def tinh_vector_dem_tu(van_ban, tu_vung, cum_tu_goc):
    vb = chuan_hoa_cum_tu(van_ban.lower(), cum_tu_goc)
    cac_tu = vb.split()
    return [cac_tu.count(tu) for tu in tu_vung]


def dem_vi_tri_cung_khac_0(v1, v2):
    return sum(1 for a, b in zip(v1, v2) if a > 0 and b > 0)


DOAN_A1 = "may tinh hien dai chay phan mem duoc viet bang ngon ngu lap trinh xu ly du lieu qua vi xu ly manh me"
DOAN_A2 = "lap trinh vien viet phan mem de may tinh xu ly du lieu nhanh hon ung dung thuat toan toi uu"
DOAN_B1 = "mon an ngon phu thuoc vao gia vi va cong thuc nau dau bep gioi trong nha bep sach se tao ra mon trang mieng hap dan"

vA1 = tinh_vector_dem_tu(DOAN_A1, TU_VUNG, CUM_TU_GOC)
vA2 = tinh_vector_dem_tu(DOAN_A2, TU_VUNG, CUM_TU_GOC)
vB1 = tinh_vector_dem_tu(DOAN_B1, TU_VUNG, CUM_TU_GOC)

print(vA1)
print(vA2)
print(vB1)
print(dem_vi_tri_cung_khac_0(vA1, vA2))
print(dem_vi_tri_cung_khac_0(vA1, vB1))
```

```python title=test
assert vA1 == [1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], f"vA1 sai -- dang ra {vA1}"
assert vA2 == [1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0], f"vA2 sai -- dang ra {vA2}"
assert vB1 == [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1], f"vB1 sai -- dang ra {vB1}"
assert len(vA1) == 16 and len(vB1) == 16, "moi vector phai co do dai DUNG BANG kich thuoc tu vung (16), bat ke doan van dai hay ngan"
assert dem_vi_tri_cung_khac_0(vA1, vA2) == 4, f"cung chu de phai co 4 chi so cung khac 0 -- dang ra {dem_vi_tri_cung_khac_0(vA1, vA2)}"
assert dem_vi_tri_cung_khac_0(vA1, vB1) == 0, f"khac chu de phai co 0 chi so cung khac 0 -- dang ra {dem_vi_tri_cung_khac_0(vA1, vB1)}"

# bien: doan van RONG -- vector phai la toan so 0, khong loi
v_rong = tinh_vector_dem_tu("", TU_VUNG, CUM_TU_GOC)
assert v_rong == [0] * 16, f"doan rong phai cho vector toan so 0 -- dang ra {v_rong}"

# bien: chuan_hoa_cum_tu phai THAY THE cum tu goc bang token, khong phai
# nguoc lai -- kiem tra truc tiep tren mot chuoi don gian
assert chuan_hoa_cum_tu("toi thich gia vi ngon", CUM_TU_GOC) == "toi thich gia_vi ngon", f"chuan_hoa_cum_tu phai bien 'gia vi' thanh 'gia_vi' -- dang ra {chuan_hoa_cum_tu('toi thich gia vi ngon', CUM_TU_GOC)!r}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `chuan_hoa_cum_tu`) là hai đối số của `kq.replace(cu, moi)` — thay CỤM TỪ GỐC (biến `cum`, ví dụ `"may tinh"`) bằng TOKEN liền (biến `token`, ví dụ `"may_tinh"`) — thứ tự đối số của `.replace()` quan trọng: đối số ĐẦU là thứ CẦN TÌM, đối số SAU là thứ THAY VÀO. Chỗ hai (trong `tinh_vector_dem_tu`) đếm số lần một TỪ VỰNG cụ thể (biến vòng lặp `tu`) xuất hiện trong danh sách từ đã tách (`cac_tu`).
- kind: strategy
  body: 'Chỗ đầu: `kq.replace(cum, token)` — tìm `cum` (cụm từ gốc, có dấu cách), thay bằng `token` (không dấu cách). Chỗ hai: `cac_tu.count(tu)` — đếm từ đang xét trong vòng lặp `for tu in tu_vung`.'
- kind: one-line
  body: 'Chỗ đầu là `kq.replace(cum, token)`, chỗ hai là `cac_tu.count(tu)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai goi .replace(cum, token) DUNG THU TU (tim cum tu GOC co dau cach, thay bang token KHONG dau cach) -- khong duoc dao nguoc; VA cho trong hai phai dem DUNG bien vong lap 'tu' (khong duoc dem mot ten khac)
  requireAst:
  - kind: uses-call, target: replace, min: 1
  - kind: uses-name, target: token, min: 1
  - kind: uses-name, target: cum, min: 1
  - kind: uses-name, target: tu, min: 1
  # Da thu that (trich nguyen ham _dem cua kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [1, 1, 1, 1] cho
  # bon luat theo dung thu tu khai bao o tren.
  # replace=1: dung mot lan GOI THAT, o cho trong dau.
  # token=1, cum=1: ca hai bien nay la MUC TIEU cua vong lap
  # "for token, cum in cum_tu_goc.items():" -- do la GAN (Store), khong duoc
  # uses-name dem. Moi bien CHI duoc DOC (Load) dung MOT lan, chinh la trong
  # loi goi .replace(...) o cho trong dau.
  # tu=1: bien nay la muc tieu vong lap "for tu in tu_vung" trong list-
  # comprehension (Store, khong dem) -- duoc DOC dung mot lan, o cho trong
  # hai ("cac_tu.count(tu)").
  #
  # 🔴🔴🔴 GOTCHA QUAN TRONG NHAT o CHO TRONG DAU (da tu dung mutant "hoan
  # doi ca cum" va CHAY THAT qua ham _dem de xac nhan, khong doan tay): hai
  # doi so cua .replace() la HAI TEN BIEN rieng biet (token, cum) CO THE
  # hoan doi vi tri cho nhau ve mat cu phap -- "kq.replace(token, cum)" van
  # la Python hop le. Da thu chay THAT mutant nay (doi cho token/cum trong
  # loi goi replace) qua ham _dem: ket qua AST la [1, 1, 1, 1] -- Y HET ban
  # dung (moi ten van duoc doc dung 1 lan, chi doi VI TRI trong lenh goi,
  # tong so lan doc khong doi). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT DOC LAP boi tests/output: ".replace(token, cum)" tim
  # kiem chuoi token (vi du "may_tinh", co dau gach duoi) trong van ban GOC
  # (dung dau cach, "may tinh") -- KHONG BAO GIO khop, nen chuan_hoa_cum_tu
  # tro thanh mot phep KHONG LAM GI CA (van ban giu nguyen, khong co token
  # nao duoc tao ra). Ket qua: MOI vector dem tu deu tro thanh toan so 0 (vi
  # .split() tren van ban CHUA chuan hoa khong bao gio khop voi cac muc tu
  # vung dang "xxx_yyy" co gach duoi). Da tu chay THAT mutant nay qua
  # python3 de xac nhan vA1 tro thanh [0]*16 thay vi gia tri mong doi, va
  # assert rieng "vA1 == [1, 1, 1, 1, 1, 0, ...]" (o tren) bat duoc NGAY LAP
  # TUC, doc lap voi static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0\\]\\n\\[1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0\\]\\n\\[0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1\\]\\n4\\n0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`4` chỉ số cùng khác `0` cho cặp cùng chủ đề, `0` cho cặp khác chủ đề —
một vector đếm từ tự chế, không mô hình nào đứng sau, vẫn tách được hai chủ
đề rõ ràng. Bài sau đo "gần nhau" bằng MỘT con số duy nhất: tích vô hướng.
::::

::::reflect{#nghi-lai}
"Embedding tự chế" ở đây không thông minh — nó không biết `"vi xu ly"` và
`"chip"` là cùng một khái niệm nếu từ vựng không liệt kê cả hai. Nhưng nó
làm đúng MỘT việc cần thiết cho RAG: biến một đoạn văn bản có độ dài BẤT KỲ
thành một vector số có độ dài CỐ ĐỊNH, mà hai đoạn cùng chủ đề cho vector
"gần" nhau hơn — đo được bằng số, không suy đoán. Bài này đếm CHỈ SỐ CÙNG
KHÁC `0` — một phép đo thô, chỉ nói "có bao nhiêu từ chung", không nói MỨC
ĐỘ chung nhiều hay ít. Bài sau thay phép đếm thô đó bằng MỘT con số duy
nhất, tính đến CẢ số lần lặp lại của từng từ: tích vô hướng (dot product).
::::

::::checkpoint{mastery=0.85}
::::
