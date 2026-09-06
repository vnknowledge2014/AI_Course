---
id: tri-tue-nhan-tao.tim-kiem-vector-va-do-tuong-dong.do-toc-do-va-lam-chuan-doi-chieu
title: "Đo tốc độ: vét cạn luôn đúng, luôn tuyến tính theo N"
summary: "Dem so lan GOI THAT ham tinh do tuong dong (khong dung time.perf_counter -- dem SO PHEP TOAN, tat dinh) khi tim k lan can tren kho N=8 doan. Ket qua THAT: k=1, k=3, k=8 deu cho DUNG 8 phep so sanh -- so phep so sanh CHI phu thuoc N (kich thuoc kho), khong phu thuoc k. Vet can LUON dung (khong xap xi) nhung tang TUYEN TINH theo N -- day la 'dap an chuan' (ground truth) de doi chieu do chinh xac (recall) cua chi muc HNSW tu cai o quest sau (hnsw-tu-cai, q8.5c)."
locale: vi
track: tri-tue-nhan-tao
module: tim-kiem-vector-va-do-tuong-dong
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.do-toc-do-va-lam-chuan-doi-chieu]
requires: [ai.tim-k-lan-can-gan-nhat-vet-can]
concepts: [ai.do-toc-do-va-lam-chuan-doi-chieu]
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
Bài trước: vét cạn tìm đúng `3` đoạn công nghệ trong kho `8` đoạn. Nhưng nó
đã "tốn" bao nhiêu phép tính để làm việc đó? Bài này đo — bằng số phép
toán, không phải đồng hồ.
::::

::::explain{#do_toc_do_bang_so_phep_toan}
`tim_k_lan_can_vet_can` (bài trước) tính cosine similarity giữa câu hỏi và
TỪNG một vector trong kho — không có cách nào bỏ qua một vector mà vẫn đảm
bảo không bỏ sót lân cận gần nhất. Vì vậy, số phép so sánh (tính cosine
similarity một lần) mà nó thực hiện PHẢI đúng bằng kích thước kho: `N` phép
so sánh cho một kho `N` phần tử — BẤT KỂ `k` (số lân cận cần tìm) là bao
nhiêu. Tìm `1` lân cận hay tìm `N` lân cận trong cùng một kho đều cần TÍNH
similarity với CẢ `N` phần tử trước — sự khác biệt duy nhất là bước SẮP XẾP
VÀ CẮT diễn ra sau đó, không ảnh hưởng tới số lần tính similarity.

Đo tốc độ ở đây KHÔNG dùng đồng hồ (`time.perf_counter()`), vì thời gian
đồng hồ phụ thuộc máy chạy, tải hệ thống, và có thể khác nhau giữa hai lần
chạy CÙNG một đoạn mã. Thay vào đó, đo bằng **số phép toán** — cụ thể là số
lần hàm tính similarity được GỌI THẬT — một con số TẤT ĐỊNH, giống hệt nhau
ở mọi lần chạy, trên mọi máy.

Vai trò của bài này trong track: vét cạn LUÔN cho kết quả ĐÚNG (không xấp
xỉ, không "gần đúng") — nhưng số phép so sánh của nó tăng TUYẾN TÍNH theo
kích thước kho (`N` phép so sánh cho `N` phần tử). Với một kho vài chục
triệu đoạn, tính `N` lần similarity cho MỖI câu hỏi là quá chậm để dùng
thực tế. Quest sau (`hnsw-tu-cai`, q8.5c) xây một chỉ mục HNSW tự cài, mục
tiêu là GIẢM số phép so sánh xuống dưới `N` mà VẪN giữ độ chính xác cao —
và "đáp án chuẩn" để đối chiếu độ chính xác đó (recall) chính là kết quả
vét cạn của bài trước: vét cạn luôn đúng, nên bất cứ khi nào kết quả của
HNSW KHÁC với vét cạn, đó là một dấu hiệu độ chính xác bị giảm.
::::

::::example{#dem_so_phep_so_sanh_bang_N}
Dùng lại đúng kho `8` đoạn của bài trước — nhưng thêm một BỘ ĐẾM, tăng lên
mỗi lần hàm tính similarity được gọi thật:

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


def tuong_dong_cosine_dem(v1, v2, bo_dem):
    bo_dem[0] += 1
    return tuong_dong_cosine(v1, v2)


def tim_k_lan_can_dem_so_sanh(vector_cau_hoi, danh_sach_vector_kho, k):
    bo_dem = [0]
    do_tuong_dong = [tuong_dong_cosine_dem(vector_cau_hoi, v, bo_dem) for v in danh_sach_vector_kho]
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k], bo_dem[0]


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

vector_cau_hoi = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)
vector_kho = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in KHO_DOAN]

ket_qua_k1, dem_k1 = tim_k_lan_can_dem_so_sanh(vector_cau_hoi, vector_kho, 1)
ket_qua_k3, dem_k3 = tim_k_lan_can_dem_so_sanh(vector_cau_hoi, vector_kho, 3)
ket_qua_k8, dem_k8 = tim_k_lan_can_dem_so_sanh(vector_cau_hoi, vector_kho, 8)

print(dem_k1)
print(dem_k3)
print(dem_k8)
print(dem_k1 == dem_k3 == dem_k8 == len(vector_kho))
```

```text title=readonly
8
8
8
True
```

Kho có `N = 8` đoạn. Gọi `tim_k_lan_can_dem_so_sanh` với `k=1`, `k=3`, rồi
`k=8` — MỖI lần gọi đều cho số phép so sánh ĐÚNG BẰNG `8`, không phải `1`,
`3`, hay một con số nào khác liên quan tới `k`. Lý do: hàm phải tính cosine
similarity cho CẢ `8` phần tử trong kho trước khi có thể sắp xếp và biết
đâu là `1`, `3`, hay `8` phần tử gần nhất — bước "biết `k` phần tử nào gần
nhất" chỉ diễn ra SAU KHI đã tính đủ similarity cho MỌI phần tử, nên `k`
không thể làm giảm số phép so sánh cần thiết chút nào. `bo_dem` là một danh
sách một phần tử (`[0]`), tăng lên `1` mỗi lần `tuong_dong_cosine_dem` được
gọi thật — một con số đếm được, tất định, không phụ thuộc máy chạy nhanh
hay chậm.
::::

::::predict{#doan_so_phep_so_sanh_khong_doi commitOnce}
Xét đúng kho `N = 8` đoạn ở ví dụ trên.

**Trước khi chạy thử**, bạn đoán: gọi `tim_k_lan_can_dem_so_sanh` với
`k=1` so với `k=8`, số phép so sánh (giá trị `bo_dem[0]` cuối cùng) có
KHÁC NHAU không?

:::opt{correct}
Không — cả hai lần gọi đều cho đúng `8` phép so sánh, vì vét cạn PHẢI tính
cosine similarity cho MỌI phần tử trong kho trước khi biết phần tử nào là
`k` phần tử gần nhất, bất kể `k` bằng `1` hay bằng `8`
:::

:::opt
Có — `k=1` chỉ cần tìm lân cận gần nhất DUY NHẤT, nên có thể dừng tính
similarity ngay khi tìm thấy một ứng viên đủ tốt, trong khi `k=8` phải
tính hết mới đủ `8` kết quả
::why
Gần đúng ở trực giác chung "tìm ít hơn thì có thể dừng sớm hơn" — một ý
tưởng hợp lý cho MỘT SỐ thuật toán tìm kiếm khác (ví dụ tìm kiếm có chỉ
mục, sẽ học ở quest sau).

Chỗ lệch: `tim_k_lan_can_vet_can`/`tim_k_lan_can_dem_so_sanh` không có cơ
chế "dừng sớm" nào — nó tính similarity cho TOÀN BỘ danh sách trước
(`[tuong_dong_cosine_dem(...) for v in danh_sach_vector_kho]`), RỒI MỚI sắp
xếp và cắt lấy `k` phần tử. Không có cách nào biết phần tử nào thuộc `top-
k` mà KHÔNG cần tính similarity cho MỌI phần tử trước — đó chính là bản
chất "vét cạn" của thuật toán này.
::
:::

:::opt
Có — số phép so sánh phải bằng CHÍNH XÁC `k`, vì đó là số kết quả cần trả
về, và tính nhiều hơn số cần trả về là lãng phí không cần thiết
::why
Gần đúng ở trực giác "chỉ tính đúng số cần dùng để tránh lãng phí" — một
mục tiêu HỢP LÝ cho một thuật toán được TỐI ƯU (chính là động lực của
HNSW ở quest sau).

Chỗ lệch: `tim_k_lan_can_dem_so_sanh` KHÔNG được tối ưu theo hướng đó — nó
luôn tính similarity cho CẢ `N` phần tử của kho trước khi sắp xếp, bất kể
`k` nhỏ hay lớn. Số phép so sánh phụ thuộc vào `N` (kích thước kho), không
phụ thuộc vào `k` (số kết quả cần trả về) — với kho `8` phần tử, con số đó
luôn là `8`, kể cả khi `k=1`.
::
:::
::::

::::code{#viet_dem_so_phep_so_sanh}
Hoàn thiện `tim_k_lan_can_dem_so_sanh`: gọi hàm đếm cho TỪNG vector trong
kho, rồi trả về cả kết quả `top-k` LẪN tổng số phép so sánh đã thực hiện.

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


def tuong_dong_cosine_dem(v1, v2, bo_dem):
    bo_dem[0] += 1
    return tuong_dong_cosine(v1, v2)


def tim_k_lan_can_dem_so_sanh(vector_cau_hoi, danh_sach_vector_kho, k):
    bo_dem = [0]
    do_tuong_dong = [___ for v in danh_sach_vector_kho]       # tuong_dong_cosine_dem(vector_cau_hoi, v, bo_dem)
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k], ___                              # bo_dem[0]


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

vector_cau_hoi = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)
vector_kho = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in KHO_DOAN]

ket_qua_k1, dem_k1 = tim_k_lan_can_dem_so_sanh(vector_cau_hoi, vector_kho, 1)
ket_qua_k3, dem_k3 = tim_k_lan_can_dem_so_sanh(vector_cau_hoi, vector_kho, 3)
ket_qua_k8, dem_k8 = tim_k_lan_can_dem_so_sanh(vector_cau_hoi, vector_kho, 8)

print(dem_k1)
print(dem_k3)
print(dem_k8)
print(dem_k1 == dem_k3 == dem_k8 == len(vector_kho))
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


def tuong_dong_cosine_dem(v1, v2, bo_dem):
    bo_dem[0] += 1
    return tuong_dong_cosine(v1, v2)


def tim_k_lan_can_dem_so_sanh(vector_cau_hoi, danh_sach_vector_kho, k):
    bo_dem = [0]
    do_tuong_dong = [tuong_dong_cosine_dem(vector_cau_hoi, v, bo_dem) for v in danh_sach_vector_kho]
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k], bo_dem[0]


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

vector_cau_hoi = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)
vector_kho = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in KHO_DOAN]

ket_qua_k1, dem_k1 = tim_k_lan_can_dem_so_sanh(vector_cau_hoi, vector_kho, 1)
ket_qua_k3, dem_k3 = tim_k_lan_can_dem_so_sanh(vector_cau_hoi, vector_kho, 3)
ket_qua_k8, dem_k8 = tim_k_lan_can_dem_so_sanh(vector_cau_hoi, vector_kho, 8)

print(dem_k1)
print(dem_k3)
print(dem_k8)
print(dem_k1 == dem_k3 == dem_k8 == len(vector_kho))
```

```python title=test
assert dem_k1 == 8, f"k=1 van phai tinh du 8 phep so sanh (vet can) -- dang ra {dem_k1}"
assert dem_k3 == 8, f"k=3 van phai tinh du 8 phep so sanh -- dang ra {dem_k3}"
assert dem_k8 == 8, f"k=8 van phai tinh du 8 phep so sanh -- dang ra {dem_k8}"
assert dem_k1 == dem_k3 == dem_k8 == len(vector_kho), "so phep so sanh phai luon bang N (kich thuoc kho), khong phu thuoc k"
assert ket_qua_k1 == [ket_qua_k3[0]], f"k=1 phai cho dung chi so dau tien cua k=3 -- dang ra {ket_qua_k1}"
assert set(ket_qua_k8) == set(range(8)), "k=8 (bang N) phai tra ve DU 8 chi so, moi chi so dung 1 lan"

# kiem tra truc tiep bo dem tren mot vi du nho
bo_dem_nho = [0]
_ = tuong_dong_cosine_dem([1, 0], [0, 1], bo_dem_nho)
assert bo_dem_nho[0] == 1, f"goi 1 lan phai tang bo dem len 1 -- dang ra {bo_dem_nho[0]}"
_ = tuong_dong_cosine_dem([1, 0], [1, 1], bo_dem_nho)
assert bo_dem_nho[0] == 2, f"goi lan thu hai phai tang bo dem len 2 -- dang ra {bo_dem_nho[0]}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu là biểu thức bên trong list comprehension `[... for v in danh_sach_vector_kho]` — gọi hàm ĐẾM (`tuong_dong_cosine_dem`, không phải `tuong_dong_cosine` thường) với đủ `3` đối số (`vector_cau_hoi`, `v`, và `bo_dem` để nó tăng đúng bộ đếm dùng chung). Chỗ hai là phần tử THỨ HAI của tuple trả về — giá trị bộ đếm cuối cùng (`bo_dem[0]`), đi kèm với `k` chỉ số đã tìm được.
- kind: strategy
  body: 'Chỗ đầu: `tuong_dong_cosine_dem(vector_cau_hoi, v, bo_dem)` — PHẢI truyền `bo_dem` vào, nếu không bộ đếm sẽ không bao giờ tăng. Chỗ hai: `bo_dem[0]` — đọc giá trị hiện tại của bộ đếm, đặt sau `chi_so_sap_xep[:k]` trong cùng một tuple trả về.'
- kind: one-line
  body: 'Chỗ đầu là `tuong_dong_cosine_dem(vector_cau_hoi, v, bo_dem)`, chỗ hai là `bo_dem[0]`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT tuong_dong_cosine_dem VOI DU ca ba doi so (vector_cau_hoi, v, bo_dem) cho TUNG vector trong kho; VA cho trong hai phai DOC dung bo_dem[0] lam gia tri thu hai cua tuple tra ve
  requireAst:
  - kind: uses-call, target: tuong_dong_cosine_dem, min: 1
  - kind: uses-name, target: bo_dem, min: 3
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [1, 3] cho hai
  # luat theo dung thu tu khai bao o tren.
  # tuong_dong_cosine_dem=1: CHI mot lan GOI THAT trong toan bo solution,
  # dung o cho trong dau. Ham tuong_dong_cosine_dem KHONG tu goi lai chinh
  # no trong dinh nghia cua no.
  # bo_dem=3 (TONG THAT, da xac nhan bang cong cu, khong doan tay): bien
  # nay duoc DOC (Load) o dung 3 cho -- (1) ben trong tuong_dong_cosine_dem,
  # dong "bo_dem[0] += 1" (day la Load vi Python can DOC gia tri hien tai
  # cua bo_dem truoc khi ghi lai vao o [0]), (2) cho trong dau, khi truyen
  # bo_dem lam doi so thu ba, (3) cho trong hai, khi doc "bo_dem[0]" de tra
  # ve. Dong khoi tao "bo_dem = [0]" la Store, KHONG duoc dem. Neu cho trong
  # dau BO SOT doi so bo_dem (vi du goi tuong_dong_cosine_dem(vector_cau_hoi,
  # v) thieu doi so thu ba -- se loi TypeError ngay lap tuc, hoac goi thang
  # tuong_dong_cosine binh thuong thay vi ban _dem), so lan doc bo_dem se
  # tut xuong 2 hoac thap hon -- bi chan boi min=3.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua ham
  # _dem de xac nhan, khong doan tay): dien "bo_dem[0]" vao cho trong dau
  # ("do_tuong_dong = [bo_dem[0] for v in danh_sach_vector_kho]") VA dien
  # "tuong_dong_cosine_dem(vector_cau_hoi, v, bo_dem)" vao cho trong hai
  # ("return chi_so_sap_xep[:k], tuong_dong_cosine_dem(vector_cau_hoi, v,
  # bo_dem)") -- ket qua AST van la [1, 3], Y HET ban dung (tong so lan doc
  # 'bo_dem' va so lan goi 'tuong_dong_cosine_dem' khong doi, chi doi VI
  # TRI). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': cho trong hai dung bien 'v' -- nhung
  # 'v' la bien vong lap CUA LIST COMPREHENSION o cho trong dau, va list
  # comprehension trong Python 3 co SCOPE RIENG, khong ro ri 'v' ra ngoai
  # ham -- da tu chay THAT mutant nay qua python3, xac nhan no nem
  # NameError: name 'v' is not defined ngay khi ham thuc thi den dong
  # return -- bi chan boi tier 'run', doc lap voi static.
  # Da tu ra soat them GOTCHA #6 cho luat uses-call (tuong_dong_cosine_dem):
  # trong pham vi list comprehension o cho trong dau, cac ten trong scope la
  # 'v' (bien vong lap) va 'bo_dem' (bien dong -- MOT danh sach 1 phan tu,
  # KHAC ve kieu/do dai so voi 'danh_sach_vector_kho' co 8 phan tu) -- khong
  # co danh sach nao khac CUNG DO DAI voi 'danh_sach_vector_kho' de mot
  # mutant "dung sai bien nhung tinh co cung ket qua" co the ket hop; rui ro
  # nay da duoc ra soat va KHONG ap dung o day. Rieng rui ro "goi dung ham
  # nhung thieu doi so bo_dem" da duoc chan boi luat uses-name bo_dem min=3
  # o tren.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^8\\n8\\n8\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`8`, `8`, `8` — số phép so sánh không đổi dù `k` thay đổi, đo được bằng số
tất định. Vét cạn luôn đúng, luôn tuyến tính theo `N` — đây là chuẩn để đối
chiếu ở quest sau. Bài BOSS ráp lại toàn bộ quest trên một kho tài liệu tự
nghĩ.
::::

::::reflect{#nghi-lai}
Con số `8 = 8 = 8` không phải một điều bất ngờ — nó là hệ quả TẤT YẾU của
định nghĩa "vét cạn": muốn chắc chắn không bỏ sót lân cận gần nhất nào,
phải tính similarity với MỌI phần tử trước, không có đường tắt nào khác.
Đây chính là lý do vét cạn không mở rộng tốt: `N` phép so sánh cho `N`
phần tử nghĩa là kho tăng gấp đôi thì số phép so sánh MỖI câu hỏi cũng tăng
gấp đôi — tuyến tính, không tệ, nhưng cũng không "rẻ" khi `N` lên tới hàng
triệu. Cái mà bài này để lại cho quest sau không phải là một vấn đề cần
sửa ngay — mà là một CHUẨN ĐỐI CHIẾU: vét cạn luôn đúng (không xấp xỉ),
nên bất cứ khi nào một thuật toán nhanh hơn (như HNSW ở `hnsw-tu-cai`, quest
sau) cho ra một kết quả KHÁC với vét cạn, đó là bằng chứng độ chính xác
(recall) của nó đã giảm — đo được, so được, không phải đoán.
::::

::::checkpoint{mastery=0.85}
::::
