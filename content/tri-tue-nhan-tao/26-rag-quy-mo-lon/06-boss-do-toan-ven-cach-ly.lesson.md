---
id: tri-tue-nhan-tao.rag-quy-mo-lon.boss-do-toan-ven-cach-ly
title: "BOSS quý — đo tính toàn vẹn cách ly: lọc TRƯỚC không phải SAU, đóng q8.6c tại 6/6"
summary: "Kho 4 tai lieu, 2 khach hang (A: chi so 0,1; B: chi so 2,3). CAU_HOI_VECTOR=[1,0,0] duoc CHON CO Y: tai lieu 2 (khach hang B) co cosine 1,0 -- CAO HON CA HAI tai lieu that su cua A (0,7071 va 0,8847). tim_kiem_co_cach_ly (dung, loc TRUOC, bai 3) tren 'A' k=2 cho [1, 0] -- ro_ri=0. tim_kiem_KHONG_cach_ly (SAI CO Y, doc READONLY: tinh diem TOAN BO khong loc, lay top-k toan cuc) cho [2, 1] -- ro_ri=1 (tai lieu 2 CUA B THAT SU lot vao). dem_tai_lieu_ro_ri(...) dem so chi so KHONG thuoc khach_hang_yeu_cau trong mot ket qua. chenh_lech_ro_ri(dung, sai, ...) = 1 - 0 = 1 -- do CHINH XAC su khac biet giua loc TRUOC (dung) va loc SAU/khong loc (sai) bang MOT con so cu the. Dong q8.6c tai 6/6."
locale: vi
track: tri-tue-nhan-tao
module: rag-quy-mo-lon
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-do-toan-ven-cach-ly]
requires: [ai.rap-pipeline-rag-da-khach-hang]
concepts: [ai.boss-do-toan-ven-cach-ly]
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
Bài `3` đã nói: lọc TRƯỚC khi tính điểm, không phải SAU. Bài này DỰNG một
tình huống CỤ THỂ để chứng minh vì sao thứ tự đó không phải chuyện phong
cách — bằng CHẠY THẬT, đo bằng SỐ, đóng `q8.6c` tại `6/6`.
::::

::::explain{#tinh_huong_ro_ri_cu_the}
Bài `3` đã cảnh báo bằng lời: nếu tính điểm CHO MỌI tài liệu (bất kể khách
hàng) rồi mới lọc — hay tệ hơn, QUÊN lọc — một tài liệu của khách hàng KHÁC
có thể có điểm cosine CAO HƠN mọi tài liệu THẬT SỰ thuộc về khách hàng đang
hỏi. Bài này dựng ĐÚNG tình huống đó, với số liệu cụ thể:

- Khách hàng `A` có `2` tài liệu THẬT (chỉ số `0`, `1`).
- Khách hàng `B` có `2` tài liệu (chỉ số `2`, `3`) — trong đó tài liệu `2`
  được thiết kế để có vector **RẤT GẦN** câu hỏi của A (`cosine = 1,0`) —
  CAO HƠN CẢ HAI tài liệu thật của A.

Hai hàm tìm kiếm được đối chiếu TRỰC TIẾP:

```
tim_kiem_co_cach_ly(...)      # DUNG (bai 3): loc TRUOC khi tinh diem
tim_kiem_KHONG_cach_ly(...)   # SAI CO Y (chi doc, KHONG phai bai tap):
                               # tinh diem TOAN BO kho, KHONG loc khach hang,
                               # lay top-k TOAN CUC
```

`tim_kiem_KHONG_cach_ly` là một **phản ví dụ** — mô phỏng đúng lỗi thiết kế
"quên lọc" hay "định lọc sau nhưng chưa kịp viết bước đó": nó tính điểm cho
MỌI tài liệu trong kho, xếp hạng TOÀN CỤC, và trả về top-`k` mà KHÔNG hề
biết `khach_hang_yeu_cau` là ai. Đây là phần đọc-hiểu, KHÔNG phải bài tập
chính — bài tập chính là viết hàm ĐO sự khác biệt giữa hai cách:

```
dem_tai_lieu_ro_ri(ket_qua, danh_sach_tai_lieu, khach_hang_yeu_cau):
  dem SO CHI SO trong ket_qua ma tai_lieu tai chi so do KHONG thuoc khach_hang_yeu_cau

chenh_lech_ro_ri(ket_qua_dung, ket_qua_sai, danh_sach_tai_lieu, khach_hang_yeu_cau):
  tra ve dem_tai_lieu_ro_ri(ket_qua_sai, ...) - dem_tai_lieu_ro_ri(ket_qua_dung, ...)
```
::::

::::example{#do_ro_ri_bang_so_cu_the}
```python title=readonly
import math


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


def loc_theo_khach_hang(danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau):
    chi_so_thuoc_ve = [i for i, tl in enumerate(danh_sach_tai_lieu) if tl["khach_hang"] == khach_hang_yeu_cau]
    tai_lieu_loc = [danh_sach_tai_lieu[i] for i in chi_so_thuoc_ve]
    vector_loc = [danh_sach_vector[i] for i in chi_so_thuoc_ve]
    return chi_so_thuoc_ve, tai_lieu_loc, vector_loc


def tim_kiem_co_cach_ly(cau_hoi_vector, danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau, k):
    chi_so_thuoc_ve, tai_lieu_loc, vector_loc = loc_theo_khach_hang(danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau)
    diem = [tuong_dong_cosine(cau_hoi_vector, v) for v in vector_loc]
    thu_tu_trong_loc = sorted(range(len(vector_loc)), key=lambda i: diem[i], reverse=True)
    top_k_trong_loc = thu_tu_trong_loc[:k]
    return [chi_so_thuoc_ve[i] for i in top_k_trong_loc]


def tim_kiem_KHONG_cach_ly(cau_hoi_vector, danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau, k):
    # SAI CO Y: tinh diem cho MOI tai lieu (moi khach hang tron lan), sap
    # TOAN CUC, roi lay top-k -- KHONG HE loc theo khach_hang_yeu_cau, du
    # tham so nay CO duoc truyen vao. Day la phan example SAI, de doi chieu,
    # KHONG phai bai tap chinh cua bai nay.
    diem = [tuong_dong_cosine(cau_hoi_vector, v) for v in danh_sach_vector]
    thu_tu_toan_cuc = sorted(range(len(danh_sach_tai_lieu)), key=lambda i: diem[i], reverse=True)
    return thu_tu_toan_cuc[:k]


def dem_tai_lieu_ro_ri(ket_qua, danh_sach_tai_lieu, khach_hang_yeu_cau):
    return sum(1 for i in ket_qua if danh_sach_tai_lieu[i]["khach_hang"] != khach_hang_yeu_cau)


def chenh_lech_ro_ri(ket_qua_dung, ket_qua_sai, danh_sach_tai_lieu, khach_hang_yeu_cau):
    return dem_tai_lieu_ro_ri(ket_qua_sai, danh_sach_tai_lieu, khach_hang_yeu_cau) - dem_tai_lieu_ro_ri(ket_qua_dung, danh_sach_tai_lieu, khach_hang_yeu_cau)


TAI_LIEU = [
    {"id": 0, "khach_hang": "A", "noi_dung": "tai lieu A so 0"},
    {"id": 1, "khach_hang": "A", "noi_dung": "tai lieu A so 1"},
    {"id": 2, "khach_hang": "B", "noi_dung": "tai lieu B so 2 tinh co giong cau hoi cua A"},
    {"id": 3, "khach_hang": "B", "noi_dung": "tai lieu B so 3"},
]

VECTOR = [
    [0.5, 0.5, 0.0],
    [0.6, 0.3, 0.1],
    [1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
]

CAU_HOI_VECTOR = [1.0, 0.0, 0.0]
K = 2

ket_qua_dung = tim_kiem_co_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "A", K)
ket_qua_sai = tim_kiem_KHONG_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "A", K)

ro_ri_dung = dem_tai_lieu_ro_ri(ket_qua_dung, TAI_LIEU, "A")
ro_ri_sai = dem_tai_lieu_ro_ri(ket_qua_sai, TAI_LIEU, "A")
chenh_lech = chenh_lech_ro_ri(ket_qua_dung, ket_qua_sai, TAI_LIEU, "A")

print(ket_qua_dung)
print(ket_qua_sai)
print(ro_ri_dung, ro_ri_sai)
print(chenh_lech)
```

```text title=readonly
[1, 0]
[2, 1]
0 1
1
```

Cosine của `CAU_HOI_VECTOR` với từng tài liệu: tài liệu `0` (A) ≈ `0,7071`,
tài liệu `1` (A) ≈ `0,8847`, tài liệu `2` (**B**) `= 1,0` (cao NHẤT toàn
kho), tài liệu `3` (B) `= 0,0`. `tim_kiem_co_cach_ly` (đúng) loại tài liệu
`2`, `3` (của B) NGAY TỪ ĐẦU — chỉ còn `{0, 1}` để xếp hạng — kết quả
`[1, 0]` (tài liệu `1` cosine cao hơn tài liệu `0` trong SỐ hai tài liệu
thật của A). `tim_kiem_KHONG_cach_ly` (sai) xếp TOÀN kho theo cosine, KHÔNG
loại gì cả — top-`2` toàn cục LÀ `[2, 1]`: tài liệu `2` (của B, cosine cao
nhất) đứng đầu! `dem_tai_lieu_ro_ri` xác nhận bằng số: `ro_ri_dung = 0` (hàm
đúng không rò rỉ gì), `ro_ri_sai = 1` (hàm sai để lọt ĐÚNG `1` tài liệu của
B). `chenh_lech_ro_ri = 1 - 0 = 1` — khoảng cách CỤ THỂ giữa lọc TRƯỚC (đúng)
và không lọc/lọc SAU (sai), trên chính tình huống mà cosine của một tài
liệu KHÁC khách hàng vượt qua CẢ HAI tài liệu thật.
::::

::::predict{#doan_khong_cach_ly_k1 commitOnce}
Xét gọi `tim_kiem_KHONG_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "A", 1)`
— CÙNG dữ liệu như ví dụ trên, nhưng `k = 1` (chỉ lấy MỘT kết quả, thay vì
`2`).

**Trước khi chạy thử**, bạn đoán: kết quả trả về LÀ gì?

:::opt{correct}
`[2]` — tài liệu `2` (của khách hàng **B**) có cosine `1,0`, CAO NHẤT toàn
kho; `tim_kiem_KHONG_cach_ly` không hề lọc theo khách hàng, nên NGAY CẢ khi
chỉ lấy `1` kết quả duy nhất, nó VẪN là một tài liệu của B — rò rỉ xảy ra
NGAY LẬP TỨC, không cần đợi tới kết quả thứ hai
:::

:::opt
`[1]` — tài liệu `1` (của A) có cosine cao nhất TRONG SỐ tài liệu của A, và
hàm "không cách ly" vẫn ưu tiên trả về ĐÚNG khách hàng khi có thể
::why
Gần đúng ở việc tài liệu `1` THẬT SỰ có cosine cao nhất TRONG SỐ CÁC tài
liệu CỦA A (`0,8847`, hơn tài liệu `0` là `0,7071`) — quan sát đó đúng, NẾU
CHỈ xét trong phạm vi tài liệu của A.

Chỗ lệch: `tim_kiem_KHONG_cach_ly` không hề giới hạn phạm vi xét trong tài
liệu của A — nó tính cosine cho CẢ `4` tài liệu (toàn kho) rồi xếp hạng
TOÀN CỤC. Tài liệu `2` (của B) có cosine `1,0` — cao HƠN tài liệu `1` của A
(`0,8847`) — nên nó đứng đầu bảng xếp hạng toàn cục, bất kể nó thuộc khách
hàng nào.
::
:::

:::opt
Chương trình báo lỗi, vì tham số `khach_hang_yeu_cau = "A"` được truyền vào
nhưng hàm không sử dụng nó ở đâu cả
::why
Gần đúng ở việc `khach_hang_yeu_cau` THẬT SỰ không được `tim_kiem_KHONG_
cach_ly` sử dụng ở bất kỳ đâu trong thân hàm — quan sát về việc tham số
"vô dụng" đó đúng.

Chỗ lệch: Python không ném lỗi khi một tham số được truyền vào nhưng KHÔNG
được đọc ở đâu trong thân hàm — đó là một tham số hợp lệ, chỉ đơn giản là
KHÔNG ẢNH HƯỞNG gì tới kết quả. Đây CHÍNH LÀ vấn đề của lỗi thiết kế này:
hàm trông như "có tham số khách hàng" nhưng thực chất phớt lờ nó hoàn toàn
— không có ngoại lệ, không có cảnh báo nào được ném ra.
::
:::
::::

::::code{#viet_dem_va_chenh_lech_ro_ri}
Hoàn thiện `dem_tai_lieu_ro_ri` (đếm số chỉ số trong `ket_qua` KHÔNG thuộc
`khach_hang_yeu_cau`) và `chenh_lech_ro_ri` (đo khoảng cách rò rỉ CỤ THỂ
giữa hai cách tìm kiếm — sai trừ đúng).

```python title=starter
import math


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


def loc_theo_khach_hang(danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau):
    chi_so_thuoc_ve = [i for i, tl in enumerate(danh_sach_tai_lieu) if tl["khach_hang"] == khach_hang_yeu_cau]
    tai_lieu_loc = [danh_sach_tai_lieu[i] for i in chi_so_thuoc_ve]
    vector_loc = [danh_sach_vector[i] for i in chi_so_thuoc_ve]
    return chi_so_thuoc_ve, tai_lieu_loc, vector_loc


def tim_kiem_co_cach_ly(cau_hoi_vector, danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau, k):
    chi_so_thuoc_ve, tai_lieu_loc, vector_loc = loc_theo_khach_hang(danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau)
    diem = [tuong_dong_cosine(cau_hoi_vector, v) for v in vector_loc]
    thu_tu_trong_loc = sorted(range(len(vector_loc)), key=lambda i: diem[i], reverse=True)
    top_k_trong_loc = thu_tu_trong_loc[:k]
    return [chi_so_thuoc_ve[i] for i in top_k_trong_loc]


def tim_kiem_KHONG_cach_ly(cau_hoi_vector, danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau, k):
    # SAI CO Y: tinh diem cho MOI tai lieu (moi khach hang tron lan), sap
    # TOAN CUC, roi lay top-k -- KHONG HE loc theo khach_hang_yeu_cau, du
    # tham so nay CO duoc truyen vao. Day la phan example SAI, de doi chieu,
    # KHONG phai bai tap chinh cua bai nay.
    diem = [tuong_dong_cosine(cau_hoi_vector, v) for v in danh_sach_vector]
    thu_tu_toan_cuc = sorted(range(len(danh_sach_tai_lieu)), key=lambda i: diem[i], reverse=True)
    return thu_tu_toan_cuc[:k]


def dem_tai_lieu_ro_ri(ket_qua, danh_sach_tai_lieu, khach_hang_yeu_cau):
    return sum(1 for i in ket_qua if ___)                    # danh_sach_tai_lieu[i]["khach_hang"] != khach_hang_yeu_cau


def chenh_lech_ro_ri(ket_qua_dung, ket_qua_sai, danh_sach_tai_lieu, khach_hang_yeu_cau):
    return ___                                                # dem_tai_lieu_ro_ri(ket_qua_sai, danh_sach_tai_lieu, khach_hang_yeu_cau) - dem_tai_lieu_ro_ri(ket_qua_dung, danh_sach_tai_lieu, khach_hang_yeu_cau)


TAI_LIEU = [
    {"id": 0, "khach_hang": "A", "noi_dung": "tai lieu A so 0"},
    {"id": 1, "khach_hang": "A", "noi_dung": "tai lieu A so 1"},
    {"id": 2, "khach_hang": "B", "noi_dung": "tai lieu B so 2 tinh co giong cau hoi cua A"},
    {"id": 3, "khach_hang": "B", "noi_dung": "tai lieu B so 3"},
]

VECTOR = [
    [0.5, 0.5, 0.0],
    [0.6, 0.3, 0.1],
    [1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
]

CAU_HOI_VECTOR = [1.0, 0.0, 0.0]
K = 2

ket_qua_dung = tim_kiem_co_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "A", K)
ket_qua_sai = tim_kiem_KHONG_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "A", K)

ro_ri_dung = dem_tai_lieu_ro_ri(ket_qua_dung, TAI_LIEU, "A")
ro_ri_sai = dem_tai_lieu_ro_ri(ket_qua_sai, TAI_LIEU, "A")
chenh_lech = chenh_lech_ro_ri(ket_qua_dung, ket_qua_sai, TAI_LIEU, "A")

print(ket_qua_dung)
print(ket_qua_sai)
print(ro_ri_dung, ro_ri_sai)
print(chenh_lech)
```

```python title=solution
import math


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


def loc_theo_khach_hang(danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau):
    chi_so_thuoc_ve = [i for i, tl in enumerate(danh_sach_tai_lieu) if tl["khach_hang"] == khach_hang_yeu_cau]
    tai_lieu_loc = [danh_sach_tai_lieu[i] for i in chi_so_thuoc_ve]
    vector_loc = [danh_sach_vector[i] for i in chi_so_thuoc_ve]
    return chi_so_thuoc_ve, tai_lieu_loc, vector_loc


def tim_kiem_co_cach_ly(cau_hoi_vector, danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau, k):
    chi_so_thuoc_ve, tai_lieu_loc, vector_loc = loc_theo_khach_hang(danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau)
    diem = [tuong_dong_cosine(cau_hoi_vector, v) for v in vector_loc]
    thu_tu_trong_loc = sorted(range(len(vector_loc)), key=lambda i: diem[i], reverse=True)
    top_k_trong_loc = thu_tu_trong_loc[:k]
    return [chi_so_thuoc_ve[i] for i in top_k_trong_loc]


def tim_kiem_KHONG_cach_ly(cau_hoi_vector, danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau, k):
    # SAI CO Y: tinh diem cho MOI tai lieu (moi khach hang tron lan), sap
    # TOAN CUC, roi lay top-k -- KHONG HE loc theo khach_hang_yeu_cau, du
    # tham so nay CO duoc truyen vao. Day la phan example SAI, de doi chieu,
    # KHONG phai bai tap chinh cua bai nay.
    diem = [tuong_dong_cosine(cau_hoi_vector, v) for v in danh_sach_vector]
    thu_tu_toan_cuc = sorted(range(len(danh_sach_tai_lieu)), key=lambda i: diem[i], reverse=True)
    return thu_tu_toan_cuc[:k]


def dem_tai_lieu_ro_ri(ket_qua, danh_sach_tai_lieu, khach_hang_yeu_cau):
    return sum(1 for i in ket_qua if danh_sach_tai_lieu[i]["khach_hang"] != khach_hang_yeu_cau)


def chenh_lech_ro_ri(ket_qua_dung, ket_qua_sai, danh_sach_tai_lieu, khach_hang_yeu_cau):
    return dem_tai_lieu_ro_ri(ket_qua_sai, danh_sach_tai_lieu, khach_hang_yeu_cau) - dem_tai_lieu_ro_ri(ket_qua_dung, danh_sach_tai_lieu, khach_hang_yeu_cau)


TAI_LIEU = [
    {"id": 0, "khach_hang": "A", "noi_dung": "tai lieu A so 0"},
    {"id": 1, "khach_hang": "A", "noi_dung": "tai lieu A so 1"},
    {"id": 2, "khach_hang": "B", "noi_dung": "tai lieu B so 2 tinh co giong cau hoi cua A"},
    {"id": 3, "khach_hang": "B", "noi_dung": "tai lieu B so 3"},
]

VECTOR = [
    [0.5, 0.5, 0.0],
    [0.6, 0.3, 0.1],
    [1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
]

CAU_HOI_VECTOR = [1.0, 0.0, 0.0]
K = 2

ket_qua_dung = tim_kiem_co_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "A", K)
ket_qua_sai = tim_kiem_KHONG_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "A", K)

ro_ri_dung = dem_tai_lieu_ro_ri(ket_qua_dung, TAI_LIEU, "A")
ro_ri_sai = dem_tai_lieu_ro_ri(ket_qua_sai, TAI_LIEU, "A")
chenh_lech = chenh_lech_ro_ri(ket_qua_dung, ket_qua_sai, TAI_LIEU, "A")

print(ket_qua_dung)
print(ket_qua_sai)
print(ro_ri_dung, ro_ri_sai)
print(chenh_lech)
```

```python title=test
assert ket_qua_dung == [1, 0], f"ket_qua_dung phai la [1, 0] -- dang ra {ket_qua_dung}"
assert ket_qua_sai == [2, 1], f"ket_qua_sai phai la [2, 1] -- dang ra {ket_qua_sai}"
assert ro_ri_dung == 0, f"ro_ri_dung phai la 0 -- ham DUNG khong duoc ro ri gi -- dang ra {ro_ri_dung}"
assert ro_ri_sai == 1, f"ro_ri_sai phai la 1 -- ham SAI de lot DUNG 1 tai lieu cua B -- dang ra {ro_ri_sai}"
assert chenh_lech == 1, f"chenh_lech_ro_ri phai la 1 (1 - 0) -- dang ra {chenh_lech}"

# assert TRUC TIEP: ham DUNG khong bao gio de lot tai lieu 2 (cosine cao
# nhat nhung thuoc khach hang B)
assert 2 not in ket_qua_dung, "tai lieu chi so 2 (khach hang B, cosine 1,0) KHONG DUOC lot vao ket_qua_dung"
assert 2 in ket_qua_sai, "tai lieu chi so 2 PHAI thuc su lot vao ket_qua_sai (chung minh ham SAI CO ro ri that)"

# kiem tra truc tiep dem_tai_lieu_ro_ri tren vi du nho, tu tinh tay duoc
assert dem_tai_lieu_ro_ri([0, 1], TAI_LIEU, "A") == 0, "ca hai chi so 0,1 deu thuoc A -- ro ri phai la 0"
assert dem_tai_lieu_ro_ri([2, 3], TAI_LIEU, "A") == 2, "ca hai chi so 2,3 deu KHONG thuoc A -- ro ri phai la 2"
assert dem_tai_lieu_ro_ri([0, 2], TAI_LIEU, "A") == 1, "chi so 2 khong thuoc A -- ro ri phai la 1"
assert dem_tai_lieu_ro_ri([], TAI_LIEU, "A") == 0, "ket qua rong khong the ro ri gi -- phai la 0"

# kiem tra truc tiep chenh_lech_ro_ri
assert chenh_lech_ro_ri([0, 1], [0, 1], TAI_LIEU, "A") == 0, "hai ket qua giong het nhau (khong ro ri) phai cho chenh lech 0"
assert chenh_lech_ro_ri([0, 1], [2, 3], TAI_LIEU, "A") == 2, "ket qua sai ro ri ca 2, ket qua dung ro ri 0 -- chenh lech phai la 2"

# bien: k=1 van chung minh ro ri xay ra NGAY LAP TUC voi ham SAI
ket_qua_sai_k1 = tim_kiem_KHONG_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "A", 1)
assert ket_qua_sai_k1 == [2], f"voi k=1, ham SAI van tra ve [2] (tai lieu cua B) -- dang ra {ket_qua_sai_k1}"
assert dem_tai_lieu_ro_ri(ket_qua_sai_k1, TAI_LIEU, "A") == 1, "ro ri phai xay ra NGAY o k=1, khong can doi toi k=2"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `dem_tai_lieu_ro_ri`) là ĐIỀU KIỆN bên trong `sum(1 for i in ket_qua if ___)` — quyết định chỉ số `i` nào được ĐẾM (chỉ đếm khi tài liệu tại `i` KHÔNG thuộc `khach_hang_yeu_cau`). Chỗ hai (trong `chenh_lech_ro_ri`) là GIÁ TRỊ TRẢ VỀ — hiệu số giữa rò rỉ của kết quả SAI và rò rỉ của kết quả ĐÚNG.
- kind: strategy
  body: 'Chỗ đầu: `danh_sach_tai_lieu[i]["khach_hang"] != khach_hang_yeu_cau` — đếm `1` cho MỖI chỉ số mà tài liệu tương ứng THUỘC khách hàng KHÁC. Chỗ hai: `dem_tai_lieu_ro_ri(ket_qua_sai, danh_sach_tai_lieu, khach_hang_yeu_cau) - dem_tai_lieu_ro_ri(ket_qua_dung, danh_sach_tai_lieu, khach_hang_yeu_cau)` — rò rỉ của cách SAI trừ rò rỉ của cách ĐÚNG, cho ra khoảng cách CỤ THỂ.'
- kind: one-line
  body: 'Chỗ đầu là `danh_sach_tai_lieu[i]["khach_hang"] != khach_hang_yeu_cau`, chỗ hai là `dem_tai_lieu_ro_ri(ket_qua_sai, danh_sach_tai_lieu, khach_hang_yeu_cau) - dem_tai_lieu_ro_ri(ket_qua_dung, danh_sach_tai_lieu, khach_hang_yeu_cau)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai la dieu kien SO SANH danh_sach_tai_lieu[i]["khach_hang"] != khach_hang_yeu_cau (dung != de dem tai lieu KHONG thuoc khach hang yeu cau); cho trong hai phai la PHEP TRU giua hai lan GOI THAT dem_tai_lieu_ro_ri (sai truoc, dung sau) -- khong duoc chep san mot so co dinh
  requireAst:
  - kind: uses-operator, target: "!=", min: 1
  - kind: uses-call, target: dem_tai_lieu_ro_ri, min: 4
  - kind: uses-operator, target: "-", min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, dist build that, trich
  # solution TU CHINH file nay) -- xac nhan CHINH XAC (min VA min+1):
  # "!="=1, dem_tai_lieu_ro_ri=4, "-"=1.
  # "!="=1: DUY NHAT o cho trong dau -- khong noi nao khac trong solution
  # dung phep !=.
  # dem_tai_lieu_ro_ri=4 (TONG THAT, khong phai doan): 2 lan o cho trong
  # hai (chenh_lech_ro_ri goi no cho ca ket_qua_sai VA ket_qua_dung), CONG
  # 2 lan o hai dong demo cuoi file (ro_ri_dung = dem_tai_lieu_ro_ri(...),
  # ro_ri_sai = dem_tai_lieu_ro_ri(...)). Ham nay KHONG tu goi lai chinh no.
  # "-"=1: DUY NHAT o cho trong hai (phep tru giua hai lan goi
  # dem_tai_lieu_ro_ri) -- khong noi nao khac trong solution dung phep -.
  # Dien bua "True" vao ca hai cho trong ("if True" va "return True") cho
  # "!="=0, dem_tai_lieu_ro_ri=2 (duoi 4, mat hai lan goi o cho trong hai),
  # "-"=0 -- CA BA luat CHAN DUNG (da CHAY THAT xac nhan qua kiemAst).
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho
  # trong -- xac dinh ranh gioi TU CHINH khoi starter -- va CHAY THAT qua
  # kiemAst() THAT VA python3 THAT): dien "dem_tai_lieu_ro_ri(ket_qua_sai,
  # danh_sach_tai_lieu, khach_hang_yeu_cau) - dem_tai_lieu_ro_ri(ket_qua_dung,
  # danh_sach_tai_lieu, khach_hang_yeu_cau)" vao cho trong dau (than cua
  # dem_tai_lieu_ro_ri, tham so la ket_qua, danh_sach_tai_lieu,
  # khach_hang_yeu_cau -- KHONG CO ket_qua_sai/ket_qua_dung) VA dien
  # "danh_sach_tai_lieu[i][\"khach_hang\"] != khach_hang_yeu_cau" vao cho
  # trong hai (than cua chenh_lech_ro_ri, tham so la ket_qua_dung,
  # ket_qua_sai, danh_sach_tai_lieu, khach_hang_yeu_cau -- KHONG CO "i" hay
  # "ket_qua" rieng le) -- da CHAY THAT qua kiemAst(): CA BA con so
  # ("!="=1, dem_tai_lieu_ro_ri=4, "-"=1) tren TOAN BO solution DEU KHONG
  # DOI (chi doi VI TRI) -- static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run', nhung theo HAI CO CHE KHAC NHAU (da tu
  # chay THAT qua python3 xac nhan CA HAI): ben trong dem_tai_lieu_ro_ri sau
  # khi hoan doi, bieu thuc moi doc "ket_qua_sai"/"ket_qua_dung" -- hai ten
  # nay KHONG PHAI tham so cua dem_tai_lieu_ro_ri, nhung LAI la hai BIEN
  # TOAN CUC (module-level) duoc dinh nghia sau do trong file (o phan demo)
  # -- Python phan giai chung theo pham vi TOAN CUC, nen KHONG nem NameError
  # ma thay vao do dem_tai_lieu_ro_ri tu GOI LAI CHINH NO (dem_tai_lieu_ro_ri
  # goi dem_tai_lieu_ro_ri(ket_qua_sai, ...) - dem_tai_lieu_ro_ri(ket_qua_dung,
  # ...)) VO HAN LAN -- da tu chay THAT xac nhan no nem RecursionError
  # ("maximum recursion depth exceeded") ngay khi dem_tai_lieu_ro_ri duoc
  # goi lan dau. Ben trong chenh_lech_ro_ri sau khi hoan doi (tham so la
  # ket_qua_dung, ket_qua_sai, danh_sach_tai_lieu, khach_hang_yeu_cau --
  # KHONG CO "ket_qua" hay "i" nao), bieu thuc moi doc ten "ket_qua" CHUA
  # TON TAI trong scope nay -- da tu chay THAT xac nhan NameError "name
  # 'ket_qua' is not defined". Ca hai deu la loi RUNTIME, doc lap voi
  # static -- chi khac LOAI loi (RecursionError o mot phia, NameError o phia
  # kia) vi "ket_qua_sai"/"ket_qua_dung" tinh co TON TAI o pham vi module,
  # con "ket_qua"/"i" thi khong.
  # Da tu ra soat GOTCHA #6 sau khi phat hien dieu tren: day CHINH LA truong
  # hop "bien tinh co trung ten" ma gotcha #6 canh bao -- da xac nhan bang
  # CHAY THAT rang du trung ten, mutant VAN bi chan (boi RecursionError thay
  # vi NameError), nen KHONG can them rule uses-name bo sung; da ghi lai ro
  # ca chê o day thay vi bo qua.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[1, 0\\]\\n\\[2, 1\\]\\n0 1\\n1\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`0 1` — `ro_ri_dung = 0`, `ro_ri_sai = 1`. Trên chính tình huống mà một tài
liệu của khách hàng khác có cosine CAO NHẤT toàn kho, lọc TRƯỚC (đúng) không
để lọt một tài liệu nào; lọc SAU/không lọc (sai) để lọt ĐÚNG một tài liệu —
đo được, không suy đoán. `q8.6c` "RAG quy mô lớn" đóng tại `6/6`.
::::

::::reflect{#nghi-lai}
Sáu bài của `q8.6c` trả lời từng mảnh một câu hỏi mở đầu: `Chương 39.4` mô
tả RAG doanh nghiệp bằng ba khái niệm — event-driven ingestion, sharding,
tenant isolation — cộng thêm hybrid search/rerank từ `q8.5e`. Bài `1` mô
phỏng nạp theo sự kiện (hàng đợi xử lý dần, KHÔNG đổi kết quả). Bài `2` mô
phỏng sharding (round-robin, không mất không trùng). Bài `3` viết đúng cách
cô lập (lọc TRƯỚC khi tính điểm). Bài `4` mở rộng hybrid với một bước
rerank (một tiêu chí KHÁC, có thể đổi hẳn hạng `1`). Bài `5` ráp cả bốn
thành một pipeline đa khách hàng. Bài này — bài cuối — đo CHÍNH XÁC vì sao
THỨ TỰ lọc-trước-hay-sau không phải chuyện phong cách: trên một tình huống
CỤ THỂ (một tài liệu của khách hàng khác có cosine `1,0`, cao nhất toàn
kho), lọc TRƯỚC cho `ro_ri = 0`, lọc SAU/không lọc cho `ro_ri = 1` —
`chenh_lech_ro_ri = 1`, một con số ĐO ĐƯỢC, không phải một cảm giác "có vẻ
an toàn hơn".

`q8.6c` "RAG quy mô lớn" đóng tại `6/6`. `T8.6` "Hạ tầng & vận hành AI" tiếp
tục với `q8.6d` — bước sự kiện tác tử VÀ đo lường cache/chi phí — trước khi
`q8.6e` ráp TOÀN BỘ bốn quest của `T8.6` thành một luồng xử lý request
đầu-cuối, đóng CẢ `T8.6` tại `26/26` VÀ `R8` "AI · Generative AI · RAG" tại
`190/190`.
::::

::::checkpoint{mastery=0.9}
::::
