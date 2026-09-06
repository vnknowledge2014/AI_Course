---
id: tri-tue-nhan-tao.boss-rag-tu-so-0.chunking-embedding-tai-lieu-moi
title: "Chunking + vector trên tài liệu MỚI: nguyên liệu cho pipeline RAG"
summary: "Tai lieu MOI 14 cau ve san pham zt8821 (chua tung dung o quest nao truoc), chia bang chia_theo_cau (q8.5a) thanh DUNG 14 doan -- do dai trung binh 13,0714 tu/doan (183 tu tren 14 doan). Vector dem tu tinh_vector_dem_tu (q8.5a, tren TU_VUNG 16 muc cu, khong doi) cho doan thu 12 (index 11, 'che do bao hanh...zt8821...18 thang') la vector TOAN SO 0 -- khong mot trong 16 muc tu vung khop dung voi cau nay. Day la nen mong cho ca quest: mot chi tiet cu the (thoi han bao hanh) ma vector KHONG THAY duoc."
locale: vi
track: tri-tue-nhan-tao
module: boss-rag-tu-so-0
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.chunking-embedding-tai-lieu-moi]
requires: [ai.boss-hybrid-sua-ca-hai-diem-mu]
concepts: [ai.chunking-embedding-tai-lieu-moi]
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
Năm quest đã xây từng mảnh: chunking + vector tự chế, vét cạn cosine, HNSW tự
cài, BM25, hybrid RRF. Quest cuối cùng của `T8.5` ráp TẤT CẢ chúng lại thành
MỘT pipeline RAG hoàn chỉnh, chạy trên một tài liệu hoàn toàn MỚI. Bài này bắt
đầu bằng bước đầu tiên: chia tài liệu, tính vector.
::::

::::explain{#tai_lieu_moi_va_muc_tieu_ca_quest}
Năm bài học tiếp theo dùng LẠI — không viết lại — đúng những hàm đã xây suốt
`T8.5`: `chia_theo_cau` (q8.5a), `tinh_vector_dem_tu`/`TU_VUNG`/`CUM_TU_GOC`
(q8.5a), `chen_hnsw`/`tim_kiem_hnsw` (q8.5c), `xep_hang_bm25` (q8.5d),
`dong_gop_rrf` (q8.5e). Mục tiêu không phải học thêm một thuật toán mới — là
RÁP những thuật toán đã học thành một hệ thống, rồi đo bằng số việc có pipeline
đó cải thiện ra sao so với không có gì cả.

Để đo cải thiện một cách THUYẾT PHỤC, pipeline cần chạy trên một tài liệu mà
một "LLM mô phỏng" (bài BOSS cuối) không thể "biết" sẵn — một tài liệu bịa,
với CHI TIẾT CỤ THỂ (tên sản phẩm, con số) không nằm trong bất kỳ quy tắc cố
định nào. Tài liệu của quest này: một bản tin ngắn về sản phẩm hư cấu
`zt8821`, `14` câu, chưa từng xuất hiện ở bất kỳ bài nào trước trong track.

`TU_VUNG` và `CUM_TU_GOC` (q8.5a) giữ NGUYÊN — `16` mục cố định, nửa kỹ thuật
nửa ẩm thực, không đổi theo tài liệu. Điều này có một hệ quả quan trọng: một
câu nói về `zt8821` có thể dùng vài từ TRÙNG với `8` mục kỹ thuật (`"vi xu
ly"`, `"may tinh"`, `"phan mem"`...), nhưng CHI TIẾT riêng của sản phẩm này —
mã `"zt8821"`, thời hạn bảo hành, giá bán — không nằm trong `16` mục đó. Vector
đếm từ (embedding tự chế) sẽ HOÀN TOÀN MÙ trước những chi tiết ấy — một hạt
giống quan trọng cho các bài sau.
::::

::::example{#chia_va_tinh_vector_tai_lieu_moi}
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


def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


TAI_LIEU_MOI = (
    "cong ty ra mat san pham may tinh bang zt8821 vao thang nay. "
    "may tinh bang zt8821 dung vi xu ly moi manh hon the he truoc. "
    "phan mem tren zt8821 duoc lap trinh toi uu cho ung dung van phong. "
    "gia ban chinh thuc cua zt8821 la 8990000 dong. "
    "ket noi mang cua zt8821 ho tro ca wifi va 4g. "
    "du lieu nguoi dung tren zt8821 duoc luu tru bang thuat toan ma hoa rieng. "
    "vi xu ly cua zt8821 giup ung dung khoi dong nhanh hon. "
    "lap trinh vien co the viet phan mem moi cho zt8821 qua bo cong cu rieng. "
    "thuat toan nen du lieu tren zt8821 tiet kiem bo nho luu tru. "
    "ung dung camera cua zt8821 xu ly anh bang thuat toan tri tue nhan tao. "
    "ket noi mang khong day cua zt8821 on dinh trong pham vi rong. "
    "che do bao hanh chinh thuc cua zt8821 la 18 thang. "
    "du lieu ban dau cho thay zt8821 ban duoc 50000 chiec trong tuan dau. "
    "cong ty cam ket cap nhat phan mem cho zt8821 trong 3 nam."
)

DOAN = chia_theo_cau(TAI_LIEU_MOI)
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DOAN]
so_tu_moi_doan = [len(d.split()) for d in DOAN]

so_doan = len(DOAN)
do_dai_trung_binh = sum(so_tu_moi_doan) / so_doan

print(so_doan)
print(round(do_dai_trung_binh, 4))
print(sum(1 for x in VEC[11] if x != 0))
```

```text title=readonly
14
13.0714
0
```

`TAI_LIEU_MOI` có đúng `14` câu, `chia_theo_cau` cho ra đúng `14` đoạn — một
tài liệu mới, chưa từng dùng ở quest nào trước, đúng yêu cầu của bài BOSS cuối
`T8.5`. Độ dài trung bình `13,0714` từ/đoạn (`183` từ tổng cộng trên `14`
đoạn) — không đoạn nào quá dài. Đoạn thứ `12` (chỉ số `11`, nói về thời hạn
bảo hành) cho vector **toàn số `0`** — không một trong `16` mục `TU_VUNG` xuất
hiện đúng nguyên văn trong câu này. Đây KHÔNG phải một lỗi: đó là hệ quả trực
tiếp của việc `TU_VUNG` là một danh sách CỐ ĐỊNH, và mã sản phẩm `"zt8821"`
cùng cụm `"bao hanh"`/`"18 thang"` không nằm trong danh sách đó.
::::

::::predict{#doan_vector_doan_bao_hanh commitOnce}
Xét đúng đoạn thứ `12` của `DOAN` (chỉ số `11`) — `"che do bao hanh chinh
thuc cua zt8821 la 18 thang"` — và `TU_VUNG` cố định `16` mục (`8` kỹ thuật,
`8` ẩm thực) không đổi từ q8.5a.

**Trước khi chạy thử**, bạn đoán: `tinh_vector_dem_tu` của đoạn này có bao
nhiêu chỉ số KHÁC `0`?

:::opt{correct}
`0` — không một trong `16` cụm từ chính xác của `TU_VUNG` (`"may tinh"`,
`"phan mem"`, ..., `"mon trang mieng"`) xuất hiện nguyên văn trong câu này;
mã `"zt8821"` và cụm `"bao hanh"`/`"18 thang"` đều không nằm trong danh sách
cố định đó, nên `chuan_hoa_cum_tu` không thay thế gì, và không mục từ vựng
nào đếm được lần xuất hiện nào
:::

:::opt
`8` — vì đoạn này nói về một sản phẩm công nghệ, nửa từ vựng "kỹ thuật" (`8`
mục) của `TU_VUNG` chắc chắn khớp được với chủ đề đó
::why
Gần đúng ở việc đoạn này THẬT SỰ thuộc chủ đề công nghệ (sản phẩm `zt8821`)
— quan sát về CHỦ ĐỀ đó đúng.

Chỗ lệch: `tinh_vector_dem_tu` không đếm theo "chủ đề" — nó chỉ đếm CHÍNH XÁC
`8` cụm từ kỹ thuật cố định (`"may tinh"`, `"phan mem"`, `"lap trinh"`,
`"du lieu"`, `"vi xu ly"`, `"ket noi mang"`, `"thuat toan"`, `"ung dung"`).
Câu `"che do bao hanh chinh thuc cua zt8821 la 18 thang"` không chứa NGUYÊN
VĂN bất kỳ cụm nào trong số đó — nói về công nghệ không đồng nghĩa với việc
chứa đúng những cụm từ mà từ vựng liệt kê.
::
:::

:::opt
`1` — vì `"zt8821"` là một sản phẩm công nghệ nên được tính vào mục
`ung_dung` của từ vựng
::why
Gần đúng ở việc `"zt8821"` THẬT SỰ là một mặt hàng công nghệ trong tài liệu
này — quan sát về BẢN CHẤT sản phẩm đó đúng.

Chỗ lệch: không có cơ chế nào trong `tinh_vector_dem_tu` "phân loại theo cảm
nhận" một từ lạ vào một mục từ vựng có sẵn. Nó chỉ đếm CHUỖI CHÍNH XÁC —
`"zt8821"` không phải là chuỗi `"ung_dung"` (sau chuẩn hoá), nên không mục
nào của vector tăng lên vì sự có mặt của mã sản phẩm này.
::
:::
::::

::::code{#viet_dem_doan_va_trung_binh_tai_lieu_moi}
Hoàn thiện phần đếm trên tài liệu MỚI: số đoạn sinh ra (`so_doan`), và độ
dài trung bình mỗi đoạn tính bằng số từ (`do_dai_trung_binh`).

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
        kq = kq.replace(cum, token)
    return kq


def tinh_vector_dem_tu(van_ban, tu_vung, cum_tu_goc):
    vb = chuan_hoa_cum_tu(van_ban.lower(), cum_tu_goc)
    cac_tu = vb.split()
    return [cac_tu.count(tu) for tu in tu_vung]


def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


TAI_LIEU_MOI = (
    "cong ty ra mat san pham may tinh bang zt8821 vao thang nay. "
    "may tinh bang zt8821 dung vi xu ly moi manh hon the he truoc. "
    "phan mem tren zt8821 duoc lap trinh toi uu cho ung dung van phong. "
    "gia ban chinh thuc cua zt8821 la 8990000 dong. "
    "ket noi mang cua zt8821 ho tro ca wifi va 4g. "
    "du lieu nguoi dung tren zt8821 duoc luu tru bang thuat toan ma hoa rieng. "
    "vi xu ly cua zt8821 giup ung dung khoi dong nhanh hon. "
    "lap trinh vien co the viet phan mem moi cho zt8821 qua bo cong cu rieng. "
    "thuat toan nen du lieu tren zt8821 tiet kiem bo nho luu tru. "
    "ung dung camera cua zt8821 xu ly anh bang thuat toan tri tue nhan tao. "
    "ket noi mang khong day cua zt8821 on dinh trong pham vi rong. "
    "che do bao hanh chinh thuc cua zt8821 la 18 thang. "
    "du lieu ban dau cho thay zt8821 ban duoc 50000 chiec trong tuan dau. "
    "cong ty cam ket cap nhat phan mem cho zt8821 trong 3 nam."
)

DOAN = chia_theo_cau(TAI_LIEU_MOI)
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DOAN]
so_tu_moi_doan = [len(d.split()) for d in DOAN]

so_doan = ___                                              # len(DOAN)
do_dai_trung_binh = ___                                     # sum(so_tu_moi_doan) / so_doan

print(so_doan)
print(round(do_dai_trung_binh, 4))
print(sum(1 for x in VEC[11] if x != 0))
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


def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


TAI_LIEU_MOI = (
    "cong ty ra mat san pham may tinh bang zt8821 vao thang nay. "
    "may tinh bang zt8821 dung vi xu ly moi manh hon the he truoc. "
    "phan mem tren zt8821 duoc lap trinh toi uu cho ung dung van phong. "
    "gia ban chinh thuc cua zt8821 la 8990000 dong. "
    "ket noi mang cua zt8821 ho tro ca wifi va 4g. "
    "du lieu nguoi dung tren zt8821 duoc luu tru bang thuat toan ma hoa rieng. "
    "vi xu ly cua zt8821 giup ung dung khoi dong nhanh hon. "
    "lap trinh vien co the viet phan mem moi cho zt8821 qua bo cong cu rieng. "
    "thuat toan nen du lieu tren zt8821 tiet kiem bo nho luu tru. "
    "ung dung camera cua zt8821 xu ly anh bang thuat toan tri tue nhan tao. "
    "ket noi mang khong day cua zt8821 on dinh trong pham vi rong. "
    "che do bao hanh chinh thuc cua zt8821 la 18 thang. "
    "du lieu ban dau cho thay zt8821 ban duoc 50000 chiec trong tuan dau. "
    "cong ty cam ket cap nhat phan mem cho zt8821 trong 3 nam."
)

DOAN = chia_theo_cau(TAI_LIEU_MOI)
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DOAN]
so_tu_moi_doan = [len(d.split()) for d in DOAN]

so_doan = len(DOAN)
do_dai_trung_binh = sum(so_tu_moi_doan) / so_doan

print(so_doan)
print(round(do_dai_trung_binh, 4))
print(sum(1 for x in VEC[11] if x != 0))
```

```python title=test
assert so_doan == 14, f"so_doan phai la 14 -- dang ra {so_doan}"
assert round(do_dai_trung_binh, 4) == 13.0714, f"do_dai_trung_binh phai xap xi 13,0714 -- dang ra {do_dai_trung_binh}"
assert so_tu_moi_doan == [13, 14, 14, 9, 11, 15, 12, 16, 13, 15, 13, 11, 14, 13], f"so tu moi doan sai -- dang ra {so_tu_moi_doan}"
assert sum(so_tu_moi_doan) == 183, f"tong so tu phai la 183 -- dang ra {sum(so_tu_moi_doan)}"
assert len(VEC) == so_doan, "so vector phai bang dung so doan"

# bien: doan 11 (bao hanh) phai la vector TOAN SO 0 -- dung nhu du doan o tren
assert VEC[11] == [0] * 16, f"vector doan 11 (bao hanh) phai toan so 0 -- dang ra {VEC[11]}"
assert DOAN[11] == "che do bao hanh chinh thuc cua zt8821 la 18 thang", f"noi dung doan 11 sai -- dang ra {DOAN[11]!r}"

# bien: doan 1 (chi so 1, noi ve vi xu ly) phai co it nhat mot chi so khac 0
assert sum(1 for x in VEC[1] if x != 0) > 0, "doan 1 (noi ve vi xu ly/may tinh) phai co it nhat mot chi so vector khac 0"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cả hai đều là một phép ĐẾM trên dữ liệu đã có sẵn, đúng khuôn đã dùng ở q8.5a. Chỗ đầu đếm SỐ ĐOẠN — `DOAN` đã là danh sách các đoạn, đếm ĐỘ DÀI danh sách đó. Chỗ hai tính TRUNG BÌNH — tổng số từ (`so_tu_moi_doan`, đã có sẵn) chia cho số đoạn (biến vừa tính ở chỗ trống đầu).
- kind: strategy
  body: 'Chỗ đầu: `len(DOAN)` — đếm số phần tử trong danh sách đoạn. Chỗ hai: `sum(so_tu_moi_doan) / so_doan` — tổng số từ chia cho số đoạn, dùng biến `so_doan` vừa gán ở chỗ trống đầu.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `len(DOAN)` và `sum(so_tu_moi_doan) / so_doan`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai dem THAT do dai danh sach 'DOAN' bang len(...) (khong duoc chep san so 14); VA cho trong hai phai chia TONG THAT cua 'so_tu_moi_doan' cho 'so_doan' (khong duoc chep san so 13,0714)
  requireAst:
  - kind: uses-call, target: len, min: 2
  - kind: uses-call, target: sum, min: 2
  - kind: uses-name, target: so_doan, min: 2
  - kind: uses-name, target: so_tu_moi_doan, min: 1
  # Da thu that (dung ban sao dem_ast.mjs goi THANG kiemAst() that qua dist
  # build + pyodide that, chay tren CHINH van ban solution da trich tu file
  # nay) -- ket qua [2, 2, 2, 1] cho bon luat theo dung thu tu khai bao o tren.
  # len=2 (TONG THAT): mot lan CO SAN trong "so_tu_moi_doan = [len(d.split())
  # for d in DOAN]", mot lan CHINH la cho trong dau ("so_doan = len(DOAN)").
  # Neu chi dat min=1 (ngay tho), mot mutant chep san "so_doan = 14" (bo qua
  # goi ham THAT) van qua duoc vi con lai 1 lan len(...) trong comprehension
  # -- GOTCHA "boilerplate-threshold-masking"; da tu kiem chung bang cong cu
  # THAT: mutant nay cho dem=[1,2,2,1] (len tut xuong 1), bi chan boi min=2,
  # trong khi tests/output KHONG bat duoc (14 la gia tri THAT nen ra dung ket
  # qua) -- xac nhan min=2 (tong THAT) la muc CAN THIET, khong phai doan tay.
  # sum=2 (TONG THAT): mot lan CHINH la cho trong hai ("sum(so_tu_moi_doan)"),
  # mot lan CO SAN o dong cuoi ("sum(1 for x in VEC[11] if x != 0)"). Da tu
  # kiem chung mutant chep san do_dai_trung_binh: dem sum tut tu 2 xuong 1,
  # bi chan boi min=2.
  # so_doan=2 (TONG THAT): doc (Load) o "sum(so_tu_moi_doan) / so_doan" (cho
  # trong hai) VA o "print(so_doan)" -- dong gan "so_doan = ___" la Store,
  # khong dem.
  # so_tu_moi_doan=1 (TONG THAT): doc dung MOT lan, trong "sum(so_tu_moi_doan)"
  # o cho trong hai; dong gan "so_tu_moi_doan = [...]" la Store, khong dem.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua
  # python3 VA qua dem_ast.mjs de xac nhan, khong doan tay): dien
  # "sum(so_tu_moi_doan) / so_doan" vao cho trong dau ("so_doan =
  # sum(so_tu_moi_doan) / so_doan") VA dien "len(DOAN)" vao cho trong hai
  # ("do_dai_trung_binh = len(DOAN)") -- da chay THAT qua dem_ast.mjs: ket
  # qua dem la [2, 2, 2, 1] -- Y HET ban dung (moi ten/ham van duoc goi/doc
  # dung so lan, chi DOI VI TRI giua hai dong). Static KHONG bat duoc mutant
  # nay. Mutant nay BI BAT DOC LAP boi tier 'run': dong dau tien
  # "so_doan = sum(so_tu_moi_doan) / so_doan" DOC bien 'so_doan' o VE PHAI
  # TRUOC KHI no duoc GAN bat cu dau (day la lan dau tien ten 'so_doan' xuat
  # hien trong toan bo chuong trinh) -- da tu chay THAT mutant nay qua
  # python3, xac nhan no nem NameError ("name 'so_doan' is not defined")
  # ngay khi dong nay chay, truoc ca khi tram dong thu hai kip thuc thi.
  # Da tu ra soat GOTCHA #6 cho ca bon luat: khong co bien nao khac trong
  # pham vi bai nay cung do dai/gia tri voi 'so_doan'/'so_tu_moi_doan' de co
  # the tinh co thay the ma van qua duoc tests/output (vi du thay
  # 'so_tu_moi_doan' bang 'DOAN' hay 'VEC' se lam sum(...) nem TypeError ngay
  # lap tuc, vi khong cong duoc chuoi/danh sach voi so nguyen 0) -- rui ro nay
  # da duoc ra soat va KHONG ap dung o day.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^14\\n13\\.0714\\n0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`14` đoạn, độ dài trung bình `13,0714` từ, đoạn `11` (bảo hành) vector toàn số
`0` — nguyên liệu cho pipeline đã sẵn sàng. Bài sau xây chỉ mục HNSW trên
đúng `14` vector này.
::::

::::reflect{#nghi-lai}
Bài này không dạy một kỹ thuật mới — `chia_theo_cau` và `tinh_vector_dem_tu`
đã học nguyên vẹn từ q8.5a, áp dụng CHÍNH XÁC như cũ lên một tài liệu khác.
Điều quan trọng nhất bài này thiết lập không phải cơ chế, mà là DỮ LIỆU: một
tài liệu `14` đoạn với một chi tiết cụ thể (thời hạn bảo hành `zt8821`, đoạn
`11`) mà vector đếm từ, dù chạy đúng thuật toán, cho ra vector TOÀN SỐ `0` —
không "thấy" được chi tiết đó chút nào, vì từ vựng cố định của nó không liệt
kê những từ này. Đây không phải giới hạn có thể sửa bằng cách chạy lại — nó là
bản chất của một embedding tự chế trên từ vựng nhỏ, cố định. Ba bài tiếp theo
(HNSW, hybrid, prompt) xây trên đúng dữ liệu này, và bài BOSS cuối cùng đo
xem một hệ thống RAG đầy đủ có vượt qua được giới hạn này hay không, bằng số
cụ thể — không suy đoán.
::::

::::checkpoint{mastery=0.85}
::::
