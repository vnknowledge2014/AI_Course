---
id: tri-tue-nhan-tao.rag-quy-mo-lon.cach-ly-theo-khach-hang
title: "Cô lập theo khách hàng: lọc TRƯỚC khi tính điểm, không phải SAU"
summary: "loc_theo_khach_hang(danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau) loai HOAN TOAN tai lieu cua khach hang khac TRUOC khi tinh diem -- tra ve chi so goc, tai lieu, vector CHI cua dung khach hang do. tim_kiem_co_cach_ly(...) tinh tuong_dong_cosine (tai dung tu q8.5a/b) CHI tren tap da loc, roi anh xa nguoc ve chi so GOC. Tren kho 6 tai lieu (3 khach hang A: chi so 0,2,4; 3 khach hang B: chi so 1,3,5) va CAU_HOI_VECTOR=[1,0,0]: tim_kiem_co_cach_ly(..., 'A', 2) = [0, 2] (khong bao gio co 1,3,5); doi khach_hang_yeu_cau sang 'B' cho [1, 5] (doc B chi so 1 co cosine 0,9939 -- CAO hon ca hai doc A con lai -- nhung KHONG THE lot vao ket qua cua A vi da bi loai TRUOC khi tinh diem). k=3 tren 'A' cho ca 3: [0, 2, 4]."
locale: vi
track: tri-tue-nhan-tao
module: rag-quy-mo-lon
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.cach-ly-theo-khach-hang]
requires: [ai.phan-manh-va-tim-manh]
concepts: [ai.cach-ly-theo-khach-hang]
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
Sharding (bài trước) giải quyết "kho quá lớn cho một máy". Bài này giải
quyết một câu hỏi khác hẳn: kho phục vụ NHIỀU khách hàng — làm sao chắc chắn
câu hỏi của khách hàng A KHÔNG BAO GIỜ trả về dữ liệu của khách hàng B?
::::

::::explain{#loc_truoc_khong_phai_loc_sau}
`Chương 39.4` mục `2` nêu thẳng: **"Dữ liệu của khách hàng A không bao giờ
được search trộn lẫn vào khách hàng B"** — gọi LÀ **Tenant Isolation**, thực
hiện bằng Filters Metadata của VectorDB thật (Qdrant/Milvus/Pinecone).

Có HAI cách viết một hàm tìm kiếm "có cô lập":

**Cách SAI (lọc SAU)**: tính điểm cosine cho **MỌI** tài liệu trong kho —
kể cả của khách hàng khác — sắp xếp TOÀN BỘ, rồi mới LỌC BỎ những tài liệu
không thuộc khách hàng yêu cầu. Đây là một lỗi thiết kế DỄ MẮC: nó "trông"
đúng (kết quả CÓ THỂ vẫn đúng nếu lọc cẩn thận), nhưng một hệ thống thật có
thể vô tình để lộ dữ liệu qua log, qua thời gian tính toán, hay qua một bước
lọc bị viết sai — VÌ tài liệu của khách hàng khác đã được ĐƯA VÀO tính điểm
trước khi bị loại.

**Cách ĐÚNG (lọc TRƯỚC)**: loại BỎ HOÀN TOÀN tài liệu không thuộc khách hàng
yêu cầu — TRƯỚC KHI bất kỳ phép tính điểm nào chạm vào chúng. Tài liệu của
khách hàng khác không bao giờ xuất hiện trong bất kỳ phép tính, log, hay
biến trung gian nào của quá trình tìm kiếm.

```
loc_theo_khach_hang(danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau):
  chi_so_thuoc_ve = [i CHO MOI i, tl DUYET danh_sach_tai_lieu NEU tl["khach_hang"] == khach_hang_yeu_cau]
  tra ve chi_so_thuoc_ve, tai_lieu DA LOC, vector DA LOC

tim_kiem_co_cach_ly(cau_hoi_vector, danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau, k):
  (chi_so_thuoc_ve, tai_lieu_loc, vector_loc) = loc_theo_khach_hang(...)
  diem = [tuong_dong_cosine(cau_hoi_vector, v) CHO MOI v TRONG vector_loc]   # CHI tren tap DA LOC
  top_k = sap xep giam dan theo diem, lay k dau
  tra ve [chi_so_thuoc_ve[i] CHO MOI i TRONG top_k]        # anh xa NGUOC ve chi so GOC
```

`tuong_dong_cosine` (tái dùng nguyên văn từ q8.5a/b) chỉ BAO GIỜ nhìn thấy
vector của ĐÚNG khách hàng yêu cầu — không có cơ hội nào để một tài liệu
khách khác "lọt" vào, vì nó chưa từng được đưa vào phép tính.
::::

::::example{#loc_truoc_tren_kho_hai_khach_hang}
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


TAI_LIEU = [
    {"id": 0, "khach_hang": "A", "noi_dung": "tai lieu A ve gia ca san pham"},
    {"id": 1, "khach_hang": "B", "noi_dung": "tai lieu B ve chinh sach bao mat"},
    {"id": 2, "khach_hang": "A", "noi_dung": "tai lieu A ve huong dan cai dat"},
    {"id": 3, "khach_hang": "B", "noi_dung": "tai lieu B ve dieu khoan su dung"},
    {"id": 4, "khach_hang": "A", "noi_dung": "tai lieu A ve chinh sach doi tra"},
    {"id": 5, "khach_hang": "B", "noi_dung": "tai lieu B ve gia ca san pham"},
]

VECTOR = [
    [1.0, 0.0, 0.0],
    [0.9, 0.1, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.9, 0.1],
    [0.0, 0.0, 1.0],
    [0.1, 0.0, 0.9],
]

CAU_HOI_VECTOR = [1.0, 0.0, 0.0]

ket_qua_a = tim_kiem_co_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "A", 2)
ket_qua_b = tim_kiem_co_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "B", 2)
ket_qua_a3 = tim_kiem_co_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "A", 3)

print(ket_qua_a)
print(ket_qua_b)
print(ket_qua_a3)
print(all(TAI_LIEU[i]["khach_hang"] == "A" for i in ket_qua_a))
print(all(TAI_LIEU[i]["khach_hang"] == "B" for i in ket_qua_b))
```

```text title=readonly
[0, 2]
[1, 5]
[0, 2, 4]
True
True
```

`CAU_HOI_VECTOR = [1.0, 0.0, 0.0]` khớp gần như hoàn hảo với tài liệu chỉ số
`1` (`VECTOR[1] = [0.9, 0.1, 0.0]`, cosine ≈ `0,9939` — CAO HƠN cả `2` tài
liệu còn lại của khách hàng `A`, chỉ số `2` và `4`, đều có cosine `0,0`).
Nhưng tài liệu `1` thuộc khách hàng **B**. Khi hỏi với `khach_hang_yeu_cau =
"A"`, `loc_theo_khach_hang` loại BỎ tài liệu `1`, `3`, `5` (của `B`) NGAY TỪ
ĐẦU — `tuong_dong_cosine` không bao giờ thấy `VECTOR[1]` trong lần gọi này.
`ket_qua_a = [0, 2]` — CHỈ chứa chỉ số của khách hàng `A`, dù tài liệu `1`
(của B) có điểm cosine cao hơn NHIỀU so với cả hai. Đảo ngược: `ket_qua_b =
[1, 5]` khi hỏi với `"B"` — tài liệu `1` (cosine cao nhất) đứng hạng `1`,
đúng như mong đợi CHO khách hàng B. `ket_qua_a3` (lấy `k=3`, đủ CẢ `3` tài
liệu của A) xác nhận: `[0, 2, 4]` — không có gì khác ngoài tài liệu của `A`,
bất kể `k` lớn tới đâu.
::::

::::predict{#doan_doi_khach_hang_yeu_cau commitOnce}
Xét việc gọi lại `tim_kiem_co_cach_ly` với CÙNG `CAU_HOI_VECTOR = [1.0, 0.0,
0.0]` và CÙNG `k = 2` như ví dụ trên, nhưng đổi `khach_hang_yeu_cau` từ
`"A"` THÀNH `"B"`.

**Trước khi chạy thử**, bạn đoán: kết quả trả về LÀ gì?

:::opt{correct}
`[1, 5]` — tài liệu `1` (cosine ≈ `0,9939`, cao nhất trong SỐ tài liệu của
`B`) đứng đầu, tài liệu `5` (cosine ≈ `0,1104`) đứng nhì; tài liệu `3`
(cosine `0,0`) bị bỏ lại; đổi tham số `khach_hang_yeu_cau` đổi HẲN tập tài
liệu được xét, nên đổi luôn kết quả
:::

:::opt
`[0, 2]`, giống hệt kết quả khi hỏi với `"A"` ở ví dụ trên — vì cùng một
`CAU_HOI_VECTOR` và cùng `k` thì phải cho cùng kết quả
::why
Gần đúng ở việc `CAU_HOI_VECTOR` và `k` THẬT SỰ không đổi giữa hai lần gọi —
quan sát đó đúng.

Chỗ lệch: `khach_hang_yeu_cau` là tham số THỨ TƯ, quyết định TẬP tài liệu
nào được `loc_theo_khach_hang` giữ lại TRƯỚC KHI tính điểm — đổi tham số
này đổi HẲN tập ứng viên, nên dù `CAU_HOI_VECTOR`/`k` giữ nguyên, kết quả
vẫn đổi theo. `[0, 2]` là kết quả CHO khách hàng `A`, không áp dụng khi
tham số đã đổi thành `"B"`.
::
:::

:::opt
`[1, 3]` — hai tài liệu ĐẦU TIÊN của khách hàng B theo thứ tự chỉ số gốc
(`1` và `3`), vì `tim_kiem_co_cach_ly` chỉ lọc rồi lấy `k` tài liệu ĐẦU
TIÊN, không sắp xếp theo điểm
::why
Gần đúng ở việc TẬP ứng viên đúng LÀ `{1, 3, 5}` — ba tài liệu của khách
hàng `B` — quan sát về việc lọc đúng tập đó chính xác.

Chỗ lệch: sau khi lọc, hàm VẪN sắp xếp tập đó theo điểm cosine giảm dần
(`sorted(..., reverse=True)`) trước khi lấy `k` phần tử đầu — không lấy
`k` phần tử đầu theo thứ tự CHỈ SỐ GỐC. Tài liệu `5` (cosine ≈ `0,1104`) có
điểm CAO HƠN tài liệu `3` (cosine `0,0`), nên `5` được ưu tiên trước `3`
trong kết quả, dù chỉ số gốc của `3` nhỏ hơn.
::
:::
::::

::::code{#viet_loc_va_tim_kiem_co_cach_ly}
Hoàn thiện `loc_theo_khach_hang` (điều kiện GIỮ LẠI đúng tài liệu thuộc
khách hàng yêu cầu) và `tim_kiem_co_cach_ly` (ánh xạ chỉ số trong tập ĐÃ LỌC
trở lại chỉ số GỐC của kho đầy đủ).

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
    chi_so_thuoc_ve = [i for i, tl in enumerate(danh_sach_tai_lieu) if ___]   # tl["khach_hang"] == khach_hang_yeu_cau
    tai_lieu_loc = [danh_sach_tai_lieu[i] for i in chi_so_thuoc_ve]
    vector_loc = [danh_sach_vector[i] for i in chi_so_thuoc_ve]
    return chi_so_thuoc_ve, tai_lieu_loc, vector_loc


def tim_kiem_co_cach_ly(cau_hoi_vector, danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau, k):
    chi_so_thuoc_ve, tai_lieu_loc, vector_loc = loc_theo_khach_hang(danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau)
    diem = [tuong_dong_cosine(cau_hoi_vector, v) for v in vector_loc]
    thu_tu_trong_loc = sorted(range(len(vector_loc)), key=lambda i: diem[i], reverse=True)
    top_k_trong_loc = thu_tu_trong_loc[:k]
    return ___                                              # [chi_so_thuoc_ve[i] for i in top_k_trong_loc]


TAI_LIEU = [
    {"id": 0, "khach_hang": "A", "noi_dung": "tai lieu A ve gia ca san pham"},
    {"id": 1, "khach_hang": "B", "noi_dung": "tai lieu B ve chinh sach bao mat"},
    {"id": 2, "khach_hang": "A", "noi_dung": "tai lieu A ve huong dan cai dat"},
    {"id": 3, "khach_hang": "B", "noi_dung": "tai lieu B ve dieu khoan su dung"},
    {"id": 4, "khach_hang": "A", "noi_dung": "tai lieu A ve chinh sach doi tra"},
    {"id": 5, "khach_hang": "B", "noi_dung": "tai lieu B ve gia ca san pham"},
]

VECTOR = [
    [1.0, 0.0, 0.0],
    [0.9, 0.1, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.9, 0.1],
    [0.0, 0.0, 1.0],
    [0.1, 0.0, 0.9],
]

CAU_HOI_VECTOR = [1.0, 0.0, 0.0]

ket_qua_a = tim_kiem_co_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "A", 2)
ket_qua_b = tim_kiem_co_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "B", 2)
ket_qua_a3 = tim_kiem_co_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "A", 3)

print(ket_qua_a)
print(ket_qua_b)
print(ket_qua_a3)
print(all(TAI_LIEU[i]["khach_hang"] == "A" for i in ket_qua_a))
print(all(TAI_LIEU[i]["khach_hang"] == "B" for i in ket_qua_b))
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


TAI_LIEU = [
    {"id": 0, "khach_hang": "A", "noi_dung": "tai lieu A ve gia ca san pham"},
    {"id": 1, "khach_hang": "B", "noi_dung": "tai lieu B ve chinh sach bao mat"},
    {"id": 2, "khach_hang": "A", "noi_dung": "tai lieu A ve huong dan cai dat"},
    {"id": 3, "khach_hang": "B", "noi_dung": "tai lieu B ve dieu khoan su dung"},
    {"id": 4, "khach_hang": "A", "noi_dung": "tai lieu A ve chinh sach doi tra"},
    {"id": 5, "khach_hang": "B", "noi_dung": "tai lieu B ve gia ca san pham"},
]

VECTOR = [
    [1.0, 0.0, 0.0],
    [0.9, 0.1, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.9, 0.1],
    [0.0, 0.0, 1.0],
    [0.1, 0.0, 0.9],
]

CAU_HOI_VECTOR = [1.0, 0.0, 0.0]

ket_qua_a = tim_kiem_co_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "A", 2)
ket_qua_b = tim_kiem_co_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "B", 2)
ket_qua_a3 = tim_kiem_co_cach_ly(CAU_HOI_VECTOR, TAI_LIEU, VECTOR, "A", 3)

print(ket_qua_a)
print(ket_qua_b)
print(ket_qua_a3)
print(all(TAI_LIEU[i]["khach_hang"] == "A" for i in ket_qua_a))
print(all(TAI_LIEU[i]["khach_hang"] == "B" for i in ket_qua_b))
```

```python title=test
assert ket_qua_a == [0, 2], f"ket_qua_a phai la [0, 2] -- dang ra {ket_qua_a}"
assert ket_qua_b == [1, 5], f"ket_qua_b phai la [1, 5] -- dang ra {ket_qua_b}"
assert ket_qua_a3 == [0, 2, 4], f"ket_qua_a3 (k=3) phai la [0, 2, 4] -- dang ra {ket_qua_a3}"

# assert TRUC TIEP: khong bao gio ro ri chi so cua khach hang khac
assert all(TAI_LIEU[i]["khach_hang"] == "A" for i in ket_qua_a), "ket_qua_a KHONG DUOC chua bat ky tai lieu nao khong thuoc khach hang A"
assert all(TAI_LIEU[i]["khach_hang"] == "B" for i in ket_qua_b), "ket_qua_b KHONG DUOC chua bat ky tai lieu nao khong thuoc khach hang B"
assert all(TAI_LIEU[i]["khach_hang"] == "A" for i in ket_qua_a3), "ket_qua_a3 KHONG DUOC chua bat ky tai lieu nao khong thuoc khach hang A"
assert 1 not in ket_qua_a, "tai lieu chi so 1 (khach hang B, cosine cao nhat) KHONG DUOC lot vao ket qua cua khach hang A"

# bien: doi khach_hang_yeu_cau THAT SU rang buoc ket qua
assert ket_qua_a != ket_qua_b, "doi khach_hang_yeu_cau tu A sang B phai cho ket qua KHAC nhau"

# kiem tra truc tiep loc_theo_khach_hang tren vi du nho
chi_so, tl, vc = loc_theo_khach_hang(TAI_LIEU, VECTOR, "A")
assert chi_so == [0, 2, 4], f"loc_theo_khach_hang(..., 'A') phai giu dung chi so [0, 2, 4] -- dang ra {chi_so}"
assert len(tl) == 3 and len(vc) == 3, "tai_lieu_loc va vector_loc phai co DUNG 3 phan tu (dung so tai lieu cua A)"
assert all(t["khach_hang"] == "A" for t in tl), "tai_lieu_loc chi duoc chua tai lieu cua khach hang A"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `loc_theo_khach_hang`) là ĐIỀU KIỆN `if` bên trong một `list comprehension` — quyết định chỉ số `i` nào được GIỮ LẠI. Chỗ hai (trong `tim_kiem_co_cach_ly`) là GIÁ TRỊ TRẢ VỀ CUỐI CÙNG — ánh xạ chỉ số trong tập ĐÃ LỌC (`top_k_trong_loc`) trở lại chỉ số GỐC (dùng `chi_so_thuoc_ve`).
- kind: strategy
  body: 'Chỗ đầu: `tl["khach_hang"] == khach_hang_yeu_cau` — giữ lại chỉ số `i` khi VÀ CHỈ KHI tài liệu `tl` tại chỉ số đó thuộc đúng khách hàng yêu cầu. Chỗ hai: `[chi_so_thuoc_ve[i] for i in top_k_trong_loc]` — mỗi `i` trong `top_k_trong_loc` là một chỉ số TRONG tập đã lọc; `chi_so_thuoc_ve[i]` tra NGƯỢC ra chỉ số GỐC tương ứng.'
- kind: one-line
  body: 'Chỗ đầu là `tl["khach_hang"] == khach_hang_yeu_cau`, chỗ hai là `[chi_so_thuoc_ve[i] for i in top_k_trong_loc]`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai la dieu kien SO SANH tl["khach_hang"] == khach_hang_yeu_cau (khong duoc dung == voi gia tri khac hay chep san True); cho trong hai phai la MOT list comprehension ANH XA top_k_trong_loc qua chi_so_thuoc_ve (khong duoc tra ve thang top_k_trong_loc, se lam lo chi so SAI trong tap da loc thay vi chi so GOC)
  requireAst:
  - kind: uses-operator, target: "==", min: 5
  - kind: comprehension, min: 9
  - kind: uses-name, target: chi_so_thuoc_ve, min: 4
  # Da CHAY THAT qua kiemAst() (node + pyodide, dist build that, trich
  # solution TU CHINH file nay) -- xac nhan CHINH XAC (min VA min+1):
  # "=="=5, comprehension=9, uses-name(chi_so_thuoc_ve)=4.
  # "=="=5: 2 lan trong tuong_dong_cosine (boilerplate, "d1 == 0 or d2 ==
  # 0"), 1 lan o cho trong dau, 2 lan trong hai dong print cuoi file
  # (all(...== "A"...), all(...== "B"...)). TONG THAT la 5, khong phai
  # doan.
  # comprehension=9: 5 list comprehension trong hai ham loc_theo_khach_hang/
  # tim_kiem_co_cach_ly (chi_so_thuoc_ve, tai_lieu_loc, vector_loc, diem,
  # VA cho trong hai), CONG 2 generator expression trong tich_vo_huong/
  # do_dai_vector (sum(...for...)), CONG 2 generator expression trong hai
  # dong print cuoi (all(... for i in ket_qua_a), all(... for i in
  # ket_qua_b)). TONG THAT la 9.
  # uses-name(chi_so_thuoc_ve)=4: doc trong tai_lieu_loc (1), doc trong
  # vector_loc (1), doc trong cho trong hai (1), CONG 1 lan o dong "return
  # chi_so_thuoc_ve, tai_lieu_loc, vector_loc" (tra ve chinh no, mot Load).
  # Dien bua "True" vao ca hai cho trong ("if True" va "return True") cho
  # "=="=4 (duoi 5), comprehension=8 (duoi 9, mat mot list comp o cho trong
  # hai), uses-name(chi_so_thuoc_ve)=3 (duoi 4, mat lan doc o cho trong
  # hai) -- CA BA luat CHAN DUNG (da CHAY THAT xac nhan qua kiemAst).
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho
  # trong -- xac dinh ranh gioi TU CHINH khoi starter -- va CHAY THAT qua
  # kiemAst() THAT VA python3 THAT): dien
  # "[chi_so_thuoc_ve[i] for i in top_k_trong_loc]" vao cho trong dau ("if
  # [chi_so_thuoc_ve[i] for i in top_k_trong_loc]" trong
  # loc_theo_khach_hang) VA dien "tl[\"khach_hang\"] == khach_hang_yeu_cau"
  # vao cho trong hai ("return tl[\"khach_hang\"] == khach_hang_yeu_cau"
  # trong tim_kiem_co_cach_ly) -- da CHAY THAT qua kiemAst(): CA BA con so
  # (5, 9, 4) tren TOAN BO solution DEU KHONG DOI (chi doi VI TRI) -- static
  # KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong loc_theo_khach_hang (tham so
  # la danh_sach_tai_lieu, danh_sach_vector, khach_hang_yeu_cau -- KHONG CO
  # "top_k_trong_loc" nao trong scope nay), bieu thuc moi doc ten
  # "top_k_trong_loc" CHUA TON TAI -- da tu chay THAT qua python3, xac nhan
  # NameError "name 'top_k_trong_loc' is not defined" ngay khi
  # loc_theo_khach_hang duoc goi lan dau. Ben trong tim_kiem_co_cach_ly
  # (KHONG CO bien don le "tl" nao trong scope nay, chi co "tl" LA BIEN VONG
  # LAP CUC BO ben trong list comprehension cua loc_theo_khach_hang, khac
  # ham), bieu thuc moi "return tl[\"khach_hang\"] == khach_hang_yeu_cau"
  # doc ten "tl" CHUA TON TAI -- da tu chay THAT xac nhan NameError "name
  # 'tl' is not defined". Ca hai bi chan boi tier 'run', doc lap voi static.
  # Da tu ra soat GOTCHA #6: "chi_so_thuoc_ve" la ten BIEN CUC BO duy nhat
  # trong pham vi hai ham nay, khong trung voi bien nao khac; "tl" cung la
  # ten bien vong lap RIENG CUA loc_theo_khach_hang, khong ton tai o dau
  # khac -- khong co rui ro nham lan tinh co ve HINH DANG.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[0, 2\\]\\n\\[1, 5\\]\\n\\[0, 2, 4\\]\\nTrue\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`[0, 2]` cho A, `[1, 5]` cho B — dù tài liệu `1` (của B) có điểm cosine CAO
HƠN mọi tài liệu của A, nó KHÔNG BAO GIỜ lọt vào kết quả của A. Bài tiếp
theo mở rộng: sau khi tìm được ứng viên, làm sao xếp lại chúng CHÍNH XÁC
hơn bằng một bước rerank?
::::

::::reflect{#nghi-lai}
Điểm mấu chốt của bài này không nằm ở công thức cosine — nó nằm ở THỨ TỰ:
`loc_theo_khach_hang` chạy TRƯỚC, `tuong_dong_cosine` chạy SAU, và không có
cách nào đảo ngược thứ tự đó trong `tim_kiem_co_cach_ly`. Tài liệu `1` (của
khách hàng B) có cosine ≈ `0,9939` với câu hỏi — cao hơn CẢ HAI tài liệu
thật sự của A — nhưng nó KHÔNG BAO GIỜ được tính điểm khi hỏi với
`khach_hang_yeu_cau = "A"`, vì `loc_theo_khach_hang` đã loại nó TRƯỚC. Nếu
đảo ngược thứ tự (tính điểm mọi tài liệu, RỒI mới lọc), tài liệu `1` VẪN sẽ
bị loại khỏi kết quả CUỐI CÙNG nếu người viết lọc cẩn thận — nhưng nó đã
XUẤT HIỆN trong biến điểm số trung gian, trong log, trong thời gian tính
toán — một BỀ MẶT rò rỉ mà lọc TRƯỚC không hề có. Bài sau mở rộng thêm một
bước: sau khi có ứng viên (đã cô lập đúng), một bộ chấm điểm KHÁC — reranker
— xếp lại chúng bằng một tiêu chí tinh hơn.
::::

::::checkpoint{mastery=0.8}
::::
