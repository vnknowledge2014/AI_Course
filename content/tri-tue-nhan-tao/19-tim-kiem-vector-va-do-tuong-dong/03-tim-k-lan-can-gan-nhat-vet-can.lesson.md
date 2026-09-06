---
id: tri-tue-nhan-tao.tim-kiem-vector-va-do-tuong-dong.tim-k-lan-can-gan-nhat-vet-can
title: "Tìm k lân cận gần nhất, vét cạn: so hết, không bỏ sót"
summary: "tim_k_lan_can_vet_can(vector_cau_hoi, danh_sach_vector_kho, k) tinh cosine similarity giua cau hoi va TUNG vector trong kho (mot vong lap duyet HET), sap xep giam dan, tra ve k CHI SO (khong phai gia tri similarity) cao nhat. Tren mot kho 8 doan (3 cong nghe xen giua 5 am thuc, o chi so 1, 3, 5), voi cau hoi cong nghe: k=3 lan can gan nhat la DUNG chi so [1, 3, 5] -- ca 3 deu la doan cong nghe, khong lan doan am thuc nao, xac nhan bang chi so cu the."
locale: vi
track: tri-tue-nhan-tao
module: tim-kiem-vector-va-do-tuong-dong
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.tim-k-lan-can-gan-nhat-vet-can]
requires: [ai.chuan-hoa-va-cosine-similarity]
concepts: [ai.tim-k-lan-can-gan-nhat-vet-can]
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
Cosine similarity đo được "gần nhau" giữa MỘT cặp vector, công bằng. Nhưng
một kho tài liệu có nhiều đoạn — làm sao tìm ra `k` đoạn gần câu hỏi nhất
trong TẤT CẢ chúng?
::::

::::explain{#tim_k_lan_can_vet_can}
Bài toán "truy xuất" trong RAG (bài `vi-sao-can-rag`, q8.5a) giờ có đủ công
cụ để làm đúng cách: cho một câu hỏi và một kho gồm nhiều đoạn văn, tìm ra
`k` đoạn có cosine similarity CAO NHẤT với câu hỏi đó.

Cách đơn giản nhất — và LUÔN cho kết quả đúng — là **vét cạn** (brute-force
/ exhaustive search): tính cosine similarity giữa vector câu hỏi và TỪNG
MỘT vector trong kho, không bỏ sót phần tử nào, rồi sắp xếp giảm dần theo
similarity và lấy `k` phần tử đầu.

Một điểm quan trọng: kết quả trả về là `k` **chỉ số** (index — vị trí của
đoạn đó trong kho), KHÔNG PHẢI `k` giá trị similarity. Lý do: chỉ số mới
cho phép LẤY LẠI đúng đoạn văn bản gốc sau này (`kho_doan[chi_so]`) — bản
thân con số similarity không nói được đoạn nào đã sinh ra nó.
::::

::::example{#tim_3_lan_can_trong_kho_8_doan}
Một kho `8` đoạn (`3` đoạn công nghệ xen giữa `5` đoạn ẩm thực), một câu
hỏi thuộc chủ đề công nghệ, chạm tới cả `8` mục từ vựng công nghệ:

```python title=readonly
import math

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


def tich_vo_huong(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))


def do_dai_vector(v):
    return math.sqrt(sum(x ** 2 for x in v))


def tuong_dong_cosine(v1, v2):
    d1 = do_dai_vector(v1)
    d2 = do_dai_vector(v2)
    if d1 == 0 or d2 == 0:
        return 0.0
    return tich_vo_huong(v1, v2) / (d1 * d2)


def tim_k_lan_can_vet_can(vector_cau_hoi, danh_sach_vector_kho, k):
    do_tuong_dong = [tuong_dong_cosine(vector_cau_hoi, v) for v in danh_sach_vector_kho]
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k]


CAU_HOI = "may tinh chay phan mem dung lap trinh xu ly du lieu qua vi xu ly voi ket noi mang va thuat toan cho ung dung"

KHO_DOAN = [
    "mon an ngon phu thuoc vao gia vi va cong thuc nau",
    "may tinh hien dai chay phan mem manh xu ly du lieu nhanh chong",
    "dau bep gioi lam viec can than trong nha bep sach se",
    "lap trinh vien dung thuat toan toi uu de viet phan mem tot hon",
    "thuc pham tuoi giup mon an them dam da huong vi",
    "vi xu ly toc do cao ket noi mang on dinh giup ung dung chay muot ma",
    "mon trang mieng ngot ngao thuong duoc dung sau bua an",
    "nha bep sach se giup dau bep nau an thoai mai hon",
]
CHU_DE_KHO = ["am_thuc", "cong_nghe", "am_thuc", "cong_nghe", "am_thuc", "cong_nghe", "am_thuc", "am_thuc"]

vector_cau_hoi = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)
vector_kho = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in KHO_DOAN]

chi_so_ket_qua = tim_k_lan_can_vet_can(vector_cau_hoi, vector_kho, 3)
chu_de_ket_qua = [CHU_DE_KHO[i] for i in chi_so_ket_qua]

print(chi_so_ket_qua)
print(chu_de_ket_qua)
print(chu_de_ket_qua.count("cong_nghe"))
```

```text title=readonly
[1, 3, 5]
['cong_nghe', 'cong_nghe', 'cong_nghe']
3
```

`tim_k_lan_can_vet_can` tính cosine similarity giữa `vector_cau_hoi` và
CẢ `8` vector trong kho — mỗi đoạn công nghệ (chỉ số `1`, `3`, `5`) cho
cosine similarity `≈ 0,6124` (dùng chung `3` mục từ vựng khác nhau với câu
hỏi), mỗi đoạn ẩm thực cho cosine similarity `= 0,0` (không chung mục từ
vựng nào — công nghệ và ẩm thực dùng hai nửa từ vựng tách biệt hoàn toàn).
Với `k=3`: kết quả là ĐÚNG `[1, 3, 5]` — CẢ `3` chỉ số đều là đoạn công
nghệ, không đoạn ẩm thực nào lọt vào, xác nhận bằng chỉ số cụ thể chứ không
chỉ bằng "cảm giác đúng chủ đề".
::::

::::predict{#doan_k3_toan_cong_nghe commitOnce}
Xét đúng kho `8` đoạn ở ví dụ trên: chỉ số `1`, `3`, `5` là công nghệ (mỗi
đoạn cho cosine similarity `≈ 0,6124` với câu hỏi), năm chỉ số còn lại là
ẩm thực (mỗi đoạn cho cosine similarity `= 0,0`).

**Trước khi chạy thử**, bạn đoán: trong `3` chỉ số mà
`tim_k_lan_can_vet_can(vector_cau_hoi, vector_kho, 3)` trả về, có BAO NHIÊU
chỉ số là đoạn CÔNG NGHỆ?

:::opt{correct}
Cả `3` chỉ số (`3/3`) — mọi đoạn công nghệ đều có cosine similarity
`≈ 0,6124` (dương), mọi đoạn ẩm thực đều có `0,0`; sắp xếp giảm dần thì
`3` giá trị dương luôn đứng TRƯỚC `5` giá trị `0`, nên `k=3` chỉ lấy được
đúng `3` đoạn công nghệ
:::

:::opt
`2/3` — vì với `8` đoạn mà chỉ có `3` đoạn công nghệ, xác suất một đoạn ẩm
thực lọt vào `top-3` vẫn tồn tại, đặc biệt khi các giá trị similarity gần
nhau
::why
Gần đúng ở việc lo lắng "một đoạn không cùng chủ đề có thể lọt vào top-k"
— một rủi ro CÓ THẬT trong nhiều tình huống khác của cosine similarity
(bài BOSS sẽ cho thấy một dạng rủi ro tương tự với dot product thô).

Chỗ lệch: ở ĐÚNG dữ liệu này, mọi đoạn ẩm thực có cosine similarity BẰNG
TUYỆT ĐỐI `0,0` — không phải một giá trị dương nhỏ, gần với `0,6124` của
đoạn công nghệ. Khoảng cách giữa `0,6124` và `0,0` không hề mong manh, nên
không có rủi ro "lẫn lộn" nào xảy ra trên tập dữ liệu cụ thể này.
::
:::

:::opt
`0/3` — vì vòng lặp vét cạn duyệt kho theo đúng THỨ TỰ xuất hiện, nên `k=3`
đầu tiên sẽ là `3` đoạn XUẤT HIỆN ĐẦU TIÊN trong danh sách (`am_thuc`,
`cong_nghe`, `am_thuc`), không phải `3` đoạn có similarity cao nhất
::why
Gần đúng ở việc vòng lặp vét cạn THẬT SỰ duyệt kho theo đúng thứ tự xuất
hiện để TÍNH similarity — quan sát về bước tính đó đúng.

Chỗ lệch: sau khi tính xong similarity cho MỌI đoạn, `tim_k_lan_can_vet_can`
còn có một bước nữa — SẮP XẾP GIẢM DẦN theo similarity trước khi lấy `k`
đoạn đầu. Bước sắp xếp này bỏ qua hoàn toàn thứ tự xuất hiện ban đầu, chỉ
quan tâm tới GIÁ TRỊ similarity — nên `k=3` lấy đúng `3` đoạn similarity
CAO NHẤT, bất kể chúng nằm ở đâu trong danh sách gốc.
::
:::
::::

::::code{#viet_tim_k_lan_can_vet_can}
Hoàn thiện `tim_k_lan_can_vet_can`: tính cosine similarity với TỪNG vector
trong kho (một vòng lặp duyệt hết), rồi trả về `k` chỉ số cao nhất.

```python title=starter
import math

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


def tich_vo_huong(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))


def do_dai_vector(v):
    return math.sqrt(sum(x ** 2 for x in v))


def tuong_dong_cosine(v1, v2):
    d1 = do_dai_vector(v1)
    d2 = do_dai_vector(v2)
    if d1 == 0 or d2 == 0:
        return 0.0
    return tich_vo_huong(v1, v2) / (d1 * d2)


def tim_k_lan_can_vet_can(vector_cau_hoi, danh_sach_vector_kho, k):
    do_tuong_dong = [___ for v in danh_sach_vector_kho]      # tuong_dong_cosine(vector_cau_hoi, v)
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return ___                                                # chi_so_sap_xep[:k]


CAU_HOI = "may tinh chay phan mem dung lap trinh xu ly du lieu qua vi xu ly voi ket noi mang va thuat toan cho ung dung"

KHO_DOAN = [
    "mon an ngon phu thuoc vao gia vi va cong thuc nau",
    "may tinh hien dai chay phan mem manh xu ly du lieu nhanh chong",
    "dau bep gioi lam viec can than trong nha bep sach se",
    "lap trinh vien dung thuat toan toi uu de viet phan mem tot hon",
    "thuc pham tuoi giup mon an them dam da huong vi",
    "vi xu ly toc do cao ket noi mang on dinh giup ung dung chay muot ma",
    "mon trang mieng ngot ngao thuong duoc dung sau bua an",
    "nha bep sach se giup dau bep nau an thoai mai hon",
]
CHU_DE_KHO = ["am_thuc", "cong_nghe", "am_thuc", "cong_nghe", "am_thuc", "cong_nghe", "am_thuc", "am_thuc"]

vector_cau_hoi = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)
vector_kho = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in KHO_DOAN]

chi_so_ket_qua = tim_k_lan_can_vet_can(vector_cau_hoi, vector_kho, 3)
chu_de_ket_qua = [CHU_DE_KHO[i] for i in chi_so_ket_qua]

print(chi_so_ket_qua)
print(chu_de_ket_qua)
print(chu_de_ket_qua.count("cong_nghe"))
```

```python title=solution
import math

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


def tich_vo_huong(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))


def do_dai_vector(v):
    return math.sqrt(sum(x ** 2 for x in v))


def tuong_dong_cosine(v1, v2):
    d1 = do_dai_vector(v1)
    d2 = do_dai_vector(v2)
    if d1 == 0 or d2 == 0:
        return 0.0
    return tich_vo_huong(v1, v2) / (d1 * d2)


def tim_k_lan_can_vet_can(vector_cau_hoi, danh_sach_vector_kho, k):
    do_tuong_dong = [tuong_dong_cosine(vector_cau_hoi, v) for v in danh_sach_vector_kho]
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k]


CAU_HOI = "may tinh chay phan mem dung lap trinh xu ly du lieu qua vi xu ly voi ket noi mang va thuat toan cho ung dung"

KHO_DOAN = [
    "mon an ngon phu thuoc vao gia vi va cong thuc nau",
    "may tinh hien dai chay phan mem manh xu ly du lieu nhanh chong",
    "dau bep gioi lam viec can than trong nha bep sach se",
    "lap trinh vien dung thuat toan toi uu de viet phan mem tot hon",
    "thuc pham tuoi giup mon an them dam da huong vi",
    "vi xu ly toc do cao ket noi mang on dinh giup ung dung chay muot ma",
    "mon trang mieng ngot ngao thuong duoc dung sau bua an",
    "nha bep sach se giup dau bep nau an thoai mai hon",
]
CHU_DE_KHO = ["am_thuc", "cong_nghe", "am_thuc", "cong_nghe", "am_thuc", "cong_nghe", "am_thuc", "am_thuc"]

vector_cau_hoi = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)
vector_kho = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in KHO_DOAN]

chi_so_ket_qua = tim_k_lan_can_vet_can(vector_cau_hoi, vector_kho, 3)
chu_de_ket_qua = [CHU_DE_KHO[i] for i in chi_so_ket_qua]

print(chi_so_ket_qua)
print(chu_de_ket_qua)
print(chu_de_ket_qua.count("cong_nghe"))
```

```python title=test
assert chi_so_ket_qua == [1, 3, 5], f"k=3 lan can gan nhat phai la chi so [1, 3, 5] -- dang ra {chi_so_ket_qua}"
assert chu_de_ket_qua == ["cong_nghe", "cong_nghe", "cong_nghe"], f"ca 3 ket qua phai la DUNG 3 doan cong nghe -- dang ra {chu_de_ket_qua}"
assert chu_de_ket_qua.count("cong_nghe") == 3, "khong duoc lan doan am thuc nao vao ket qua"

# kiem tra truc tiep tren mot kho nho, tu tinh tay duoc
kho_nho = [[1, 0], [0, 1], [1, 1]]
assert tim_k_lan_can_vet_can([1, 0], kho_nho, 1) == [0], "cau hoi [1,0] gan [1,0] nhat (cosine=1.0)"
assert tim_k_lan_can_vet_can([1, 0], kho_nho, 2) == [0, 2], "thu 2 gan nhat phai la [1,1] (cosine ~ 0,7071), khong phai [0,1] (cosine=0.0)"

# bien: k bang N (kich thuoc kho) phai tra ve DU moi chi so, dung 1 lan
assert set(tim_k_lan_can_vet_can(vector_cau_hoi, vector_kho, 8)) == set(range(8)), "k=8 (bang N) phai tra ve DU 8 chi so, moi chi so dung 1 lan"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu là biểu thức bên trong list comprehension `[... for v in danh_sach_vector_kho]` — với MỖI vector `v` trong kho, tính cosine similarity của nó với `vector_cau_hoi` (dùng lại `tuong_dong_cosine` đã xây ở bài trước — đây chính là vòng lặp "duyệt hết", không bỏ sót phần tử nào của kho). Chỗ hai là giá trị TRẢ VỀ cuối cùng — lấy `k` chỉ số ĐẦU TIÊN của danh sách đã sắp xếp giảm dần (biến `chi_so_sap_xep`), không phải toàn bộ danh sách.
- kind: strategy
  body: 'Chỗ đầu: `tuong_dong_cosine(vector_cau_hoi, v)` — gọi đúng hàm đo tương đồng đã xây ở bài trước, với `v` là biến vòng lặp. Chỗ hai: `chi_so_sap_xep[:k]` — cắt lấy `k` phần tử đầu của danh sách chỉ số đã sắp xếp.'
- kind: one-line
  body: 'Chỗ đầu là `tuong_dong_cosine(vector_cau_hoi, v)`, chỗ hai là `chi_so_sap_xep[:k]`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT tuong_dong_cosine(vector_cau_hoi, v) cho TUNG vector trong kho (khong duoc bo qua buoc tinh similarity); VA cho trong hai phai DOC dung bien 'chi_so_sap_xep' va cat lay k phan tu dau (khong duoc tra ve toan bo danh sach hay mot gia tri chep san)
  requireAst:
  - kind: uses-call, target: tuong_dong_cosine, min: 1
  - kind: uses-name, target: chi_so_sap_xep, min: 1
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [1, 1] cho hai
  # luat theo dung thu tu khai bao o tren.
  # tuong_dong_cosine=1: CHI mot lan GOI THAT trong toan bo solution, dung o
  # cho trong dau (ben trong list comprehension). Ham tuong_dong_cosine
  # KHONG tu goi lai chinh no trong dinh nghia cua no.
  # chi_so_sap_xep=1: bien nay CHI duoc DOC (Load) dung mot lan, o cho trong
  # hai ("chi_so_sap_xep[:k]") -- dong gan "chi_so_sap_xep = sorted(...)" la
  # Store, khong duoc uses-name dem.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua ham
  # _dem de xac nhan, khong doan tay): dien "chi_so_sap_xep[:k]" vao cho
  # trong dau ("do_tuong_dong = [chi_so_sap_xep[:k] for v in
  # danh_sach_vector_kho]") VA dien "tuong_dong_cosine(vector_cau_hoi, v)"
  # vao cho trong hai ("return tuong_dong_cosine(vector_cau_hoi, v)") -- ket
  # qua AST van la [1, 1], Y HET ban dung (moi ten van xuat hien dung 1 lan,
  # chi doi VI TRI). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': cho trong hai dung bien 'v' -- nhung
  # 'v' la bien vong lap CUA LIST COMPREHENSION o cho trong dau, va list
  # comprehension trong Python 3 co SCOPE RIENG, khong ro ri 'v' ra ngoai
  # ham -- da tu chay THAT mutant nay qua python3, xac nhan no nem
  # NameError: name 'v' is not defined ngay khi tim_k_lan_can_vet_can(...)
  # thuc thi den dong return -- bi chan boi tier 'run', doc lap voi static.
  # Da tu ra soat them GOTCHA #6 (uses-call ma khong kiem tham so): trong
  # pham vi list comprehension o cho trong dau, bien 'v' la bien vong lap
  # DUY NHAT trong scope -- khong co danh sach nao khac cung do dai de mot
  # mutant "dung sai bien" co the tinh co khop; rui ro nay KHONG ap dung o
  # day.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[1, 3, 5\\]\\n\\['cong_nghe', 'cong_nghe', 'cong_nghe'\\]\\n3\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`[1, 3, 5]` — đúng `3` chỉ số công nghệ, không lẫn đoạn ẩm thực nào, đo
được trên một kho `8` đoạn. Bài sau đo TỐC ĐỘ của cách làm này — và giải
thích vì sao nó là "đáp án chuẩn" để đối chiếu sau này.
::::

::::reflect{#nghi-lai}
`tim_k_lan_can_vet_can` không "thông minh" — nó không bỏ qua bất kỳ đoạn
nào để đoán nhanh, nó tính cosine similarity với TỪNG một đoạn trong kho,
không sót đoạn nào, rồi mới sắp xếp và chọn. Chính vì KHÔNG bỏ sót gì, nó
LUÔN cho kết quả ĐÚNG — không có khái niệm "gần đúng" hay "có thể sai" ở
đây. Cái giá phải trả là gì? Bài sau đo chính xác cái giá đó bằng số: số
phép so sánh cần thực hiện, và tại sao nó tăng theo kích thước kho.
::::

::::checkpoint{mastery=0.85}
::::
