---
id: tri-tue-nhan-tao.boss-ha-tang-van-hanh-ai.rag-da-khach-hang-sau-hang-doi
title: "q8.6e bài 3 — RAG đa khách hàng sau hàng đợi: mỗi request mang theo khach_hang riêng"
summary: "truy_xuat_rag_cho_request(request, danh_sach_tai_lieu, danh_sach_vector, so_ung_vien, k_cuoi) rap tim_kiem_co_cach_ly (loc TRUOC khi tinh diem) + rerank (tai dung nguyen van tu q8.6c bai 3/4) tren MOT request mang theo khach_hang/cau_hoi/cau_hoi_vector. Tren kho 6 doan (3 khach hang A chi so 0-2, 3 khach hang B chi so 3-5) va CAU_HOI ve bao hanh: REQUEST_A (khach_hang='A') cho ung_vien=[0,1], ket_qua=[0]; REQUEST_B (khach_hang='B') cho ung_vien=[3,4], ket_qua=[3] -- ca hai KHONG BAO GIO giao nhau du CUNG mot cau_hoi_vector."
locale: vi
track: tri-tue-nhan-tao
module: boss-ha-tang-van-hanh-ai
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.rag-da-khach-hang-sau-hang-doi]
requires: [ai.hang-doi-gop-lo-sau-cong]
concepts: [ai.rag-da-khach-hang-sau-hang-doi]
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
Request đã qua cổng (bài `1`), đã có mặt trong lô suy luận (bài `2`). Bước
tiếp theo của MỘT hệ AI thật: nếu request đó cần RAG, truy xuất tài liệu
nào? `q8.6c` đã dạy: KHÔNG BAO GIỜ để lộ tài liệu của khách hàng khác. Bài
này gắn đúng bài học đó vào MỘT request cụ thể.
::::

::::explain{#rap_cach_ly_va_rerank_cho_mot_request}
`q8.6c` bài `3` dạy `tim_kiem_co_cach_ly` — lọc TRƯỚC khi tính điểm cosine,
để tài liệu khách hàng khác KHÔNG BAO GIỜ được đưa vào phép tính. Bài `4`
thêm `rerank` — xếp lại top ứng viên bằng một tiêu chí khác (khớp từ khoá
chính xác). Trong một hệ thống ĐẦU-CUỐI, request không tự mang "vector câu
hỏi" và "khách hàng" một cách rời rạc — nó LÀ một dict duy nhất mang CẢ BA
trường đó:

```
truy_xuat_rag_cho_request(request, danh_sach_tai_lieu, danh_sach_vector,
                           so_ung_vien, k_cuoi):
  ung_vien = tim_kiem_co_cach_ly(request["cau_hoi_vector"], danh_sach_tai_lieu,
                                  danh_sach_vector, request["khach_hang"], so_ung_vien)  # q8.6c bai 3
  ket_qua_cuoi = rerank(ung_vien, request["cau_hoi"],
                         [tai_lieu["noi_dung"] CHO MOI tai_lieu], k_cuoi)                # q8.6c bai 4
  tra ve ung_vien, ket_qua_cuoi
```

`request["khach_hang"]` quyết định TOÀN BỘ tập ứng viên được xét — hai
request có CÙNG `cau_hoi_vector` nhưng KHÁC `khach_hang` sẽ tìm trên hai
tập tài liệu HOÀN TOÀN tách biệt, đúng bất biến `q8.6c` đã đo bằng số cụ
thể.
::::

::::example{#hai_khach_hang_cung_cau_hoi}
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


def so_tu_khoa_trung_khop(cau_hoi, doan):
    tu_cau_hoi = set(cau_hoi.lower().split())
    tu_doan = set(doan.lower().split())
    return len(tu_cau_hoi & tu_doan)


def rerank(cac_chi_so_ung_vien, cau_hoi, danh_sach_tai_lieu, k):
    diem = [so_tu_khoa_trung_khop(cau_hoi, danh_sach_tai_lieu[i]) for i in cac_chi_so_ung_vien]
    thu_tu = sorted(range(len(cac_chi_so_ung_vien)), key=lambda j: diem[j], reverse=True)
    return [cac_chi_so_ung_vien[j] for j in thu_tu[:k]]


def truy_xuat_rag_cho_request(request, danh_sach_tai_lieu, danh_sach_vector, so_ung_vien, k_cuoi):
    ung_vien = tim_kiem_co_cach_ly(request["cau_hoi_vector"], danh_sach_tai_lieu, danh_sach_vector, request["khach_hang"], so_ung_vien)
    ket_qua_cuoi = rerank(ung_vien, request["cau_hoi"], [tl["noi_dung"] for tl in danh_sach_tai_lieu], k_cuoi)
    return ung_vien, ket_qua_cuoi


TAI_LIEU_RAG = [
    {"khach_hang": "A", "noi_dung": "san pham a bao hanh 12 thang chinh hang"},
    {"khach_hang": "A", "noi_dung": "gia ban san pham a la 5 trieu dong"},
    {"khach_hang": "A", "noi_dung": "ket noi mang wifi on dinh cho san pham a"},
    {"khach_hang": "B", "noi_dung": "san pham b bao hanh 24 thang toan quoc"},
    {"khach_hang": "B", "noi_dung": "mau sac san pham b co den va trang"},
    {"khach_hang": "B", "noi_dung": "pin san pham b sac nhanh trong 30 phut"},
]

VECTOR_RAG = [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.9, 0.1, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
]

CAU_HOI = "bao hanh san pham nay la bao lau"
CAU_HOI_VECTOR = [1.0, 0.0, 0.0]

REQUEST_A = {"khach_hang": "A", "cau_hoi": CAU_HOI, "cau_hoi_vector": CAU_HOI_VECTOR}
REQUEST_B = {"khach_hang": "B", "cau_hoi": CAU_HOI, "cau_hoi_vector": CAU_HOI_VECTOR}

ung_vien_a, ket_qua_a = truy_xuat_rag_cho_request(REQUEST_A, TAI_LIEU_RAG, VECTOR_RAG, 2, 1)
ung_vien_b, ket_qua_b = truy_xuat_rag_cho_request(REQUEST_B, TAI_LIEU_RAG, VECTOR_RAG, 2, 1)

print(ung_vien_a, ket_qua_a)
print(ung_vien_b, ket_qua_b)
print([TAI_LIEU_RAG[i]["khach_hang"] for i in ket_qua_a])
print([TAI_LIEU_RAG[i]["khach_hang"] for i in ket_qua_b])
```

```text title=readonly
[0, 1] [0]
[3, 4] [3]
['A']
['B']
```

`TAI_LIEU_RAG` có `6` đoạn: chỉ số `0-2` thuộc `A`, `3-5` thuộc `B`.
`REQUEST_A` VÀ `REQUEST_B` mang CÙNG `cau_hoi_vector = [1.0, 0.0, 0.0]` —
CHỈ khác `khach_hang`. Với `A`: `loc_theo_khach_hang` giữ đúng `[0, 1, 2]`;
cosine với `[1,0,0]` là `1.0, 0.0, 0.0` — top-`2` LÀ `[0, 1]` (đoạn `2` bị
loại vì đứng cuối theo điểm). Với `B`: giữ đúng `[3, 4, 5]`; cosine là
`≈0,9938, 0.0, 0.0` — top-`2` LÀ `[3, 4]`. Rerank (`k_cuoi=1`) trên từng cặp
ứng viên: đoạn `0` (nhiều từ khoá khớp `"bao hanh"` nhất trong số A) thắng,
`ket_qua_a = [0]`; đoạn `3` (tương tự, trong số B) thắng, `ket_qua_b = [3]`.
Hai dòng in cuối xác nhận TRỰC TIẾP: không có chỉ số nào của A lọt vào kết
quả B, và ngược lại.
::::

::::predict{#doan_vector_khop_tuyet_doi_khach_khac commitOnce}
Xét một request MỚI: `REQUEST_LEAK = {"khach_hang": "B", "cau_hoi": CAU_HOI,
"cau_hoi_vector": [1.0, 0.0, 0.0]}` — CHÍNH XÁC bằng `VECTOR_RAG[0]`, tức
khớp TUYỆT ĐỐI (cosine `= 1,0`) với đoạn `0`, một tài liệu của khách hàng
**A**. Nhưng `request["khach_hang"] = "B"`.

**Trước khi chạy thử**, bạn đoán: `truy_xuat_rag_cho_request(REQUEST_LEAK,
TAI_LIEU_RAG, VECTOR_RAG, 2, 1)` trả về gì?

:::opt{correct}
`([3, 4], [3])` — GIỐNG HỆT kết quả của `REQUEST_B` ở ví dụ trên; đoạn `0`
(khớp tuyệt đối, thuộc `A`) KHÔNG BAO GIỜ được xét, vì `loc_theo_khach_hang`
đã loại nó TRƯỚC khi bất kỳ phép tính cosine nào chạm tới `VECTOR_RAG[0]`
:::

:::opt
`([0, 3], [0])` — đoạn `0` có cosine CAO NHẤT tuyệt đối (`1,0`, khớp hoàn
hảo) trong TOÀN kho, nên nó vẫn lọt vào top ứng viên, bất kể `khach_hang`
yêu cầu LÀ gì
::why
Gần đúng ở việc đoạn `0` THẬT SỰ có điểm cosine cao NHẤT nếu tính trên TOÀN
kho không lọc — quan sát về độ lớn điểm số đó đúng.

Chỗ lệch: `tim_kiem_co_cach_ly` không bao giờ tính cosine trên TOÀN kho —
nó gọi `loc_theo_khach_hang` TRƯỚC, giữ lại DUY NHẤT tài liệu có
`"khach_hang" == "B"`. Đoạn `0` (thuộc `A`) bị loại NGAY TỪ ĐẦU, trước khi
`tuong_dong_cosine` từng được gọi trên `VECTOR_RAG[0]` — độ lớn điểm số của
nó, dù CAO tới đâu, không còn cơ hội nào để lọt vào.
::
:::

:::opt
Chương trình ném lỗi, vì `cau_hoi_vector` của `REQUEST_LEAK` trùng CHÍNH
XÁC với một vector đã có trong kho, tạo ra một mâu thuẫn dữ liệu
::why
Gần đúng ở việc việc trùng vector NGHE có vẻ "bất thường" — trực giác rằng
dữ liệu trùng khớp tuyệt đối là một ca đặc biệt cũng dễ hiểu.

Chỗ lệch: không có bước nào trong `tim_kiem_co_cach_ly`/`rerank` kiểm tra
hay từ chối việc hai vector trùng nhau — cosine trùng tuyệt đối (`= 1,0`)
là một giá trị HOÀN TOÀN hợp lệ, được tính VÀ so sánh bình thường như mọi
giá trị khác. Không có ngoại lệ nào được ném ra.
::
:::
::::

::::code{#viet_truy_xuat_rag_cho_request}
Hoàn thiện `truy_xuat_rag_cho_request`: gọi ĐÚNG hàm cô lập (`q8.6c` bài
`3`) để lấy ứng viên CHỈ của đúng khách hàng, rồi gọi ĐÚNG hàm rerank
(`q8.6c` bài `4`) trên các ứng viên đó.

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


def so_tu_khoa_trung_khop(cau_hoi, doan):
    tu_cau_hoi = set(cau_hoi.lower().split())
    tu_doan = set(doan.lower().split())
    return len(tu_cau_hoi & tu_doan)


def rerank(cac_chi_so_ung_vien, cau_hoi, danh_sach_tai_lieu, k):
    diem = [so_tu_khoa_trung_khop(cau_hoi, danh_sach_tai_lieu[i]) for i in cac_chi_so_ung_vien]
    thu_tu = sorted(range(len(cac_chi_so_ung_vien)), key=lambda j: diem[j], reverse=True)
    return [cac_chi_so_ung_vien[j] for j in thu_tu[:k]]


def truy_xuat_rag_cho_request(request, danh_sach_tai_lieu, danh_sach_vector, so_ung_vien, k_cuoi):
    ung_vien = ___                # tim_kiem_co_cach_ly(request["cau_hoi_vector"], danh_sach_tai_lieu, danh_sach_vector, request["khach_hang"], so_ung_vien)
    ket_qua_cuoi = ___             # rerank(ung_vien, request["cau_hoi"], [tl["noi_dung"] for tl in danh_sach_tai_lieu], k_cuoi)
    return ung_vien, ket_qua_cuoi


TAI_LIEU_RAG = [
    {"khach_hang": "A", "noi_dung": "san pham a bao hanh 12 thang chinh hang"},
    {"khach_hang": "A", "noi_dung": "gia ban san pham a la 5 trieu dong"},
    {"khach_hang": "A", "noi_dung": "ket noi mang wifi on dinh cho san pham a"},
    {"khach_hang": "B", "noi_dung": "san pham b bao hanh 24 thang toan quoc"},
    {"khach_hang": "B", "noi_dung": "mau sac san pham b co den va trang"},
    {"khach_hang": "B", "noi_dung": "pin san pham b sac nhanh trong 30 phut"},
]

VECTOR_RAG = [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.9, 0.1, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
]

CAU_HOI = "bao hanh san pham nay la bao lau"
CAU_HOI_VECTOR = [1.0, 0.0, 0.0]

REQUEST_A = {"khach_hang": "A", "cau_hoi": CAU_HOI, "cau_hoi_vector": CAU_HOI_VECTOR}
REQUEST_B = {"khach_hang": "B", "cau_hoi": CAU_HOI, "cau_hoi_vector": CAU_HOI_VECTOR}

ung_vien_a, ket_qua_a = truy_xuat_rag_cho_request(REQUEST_A, TAI_LIEU_RAG, VECTOR_RAG, 2, 1)
ung_vien_b, ket_qua_b = truy_xuat_rag_cho_request(REQUEST_B, TAI_LIEU_RAG, VECTOR_RAG, 2, 1)

print(ung_vien_a, ket_qua_a)
print(ung_vien_b, ket_qua_b)
print([TAI_LIEU_RAG[i]["khach_hang"] for i in ket_qua_a])
print([TAI_LIEU_RAG[i]["khach_hang"] for i in ket_qua_b])
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


def so_tu_khoa_trung_khop(cau_hoi, doan):
    tu_cau_hoi = set(cau_hoi.lower().split())
    tu_doan = set(doan.lower().split())
    return len(tu_cau_hoi & tu_doan)


def rerank(cac_chi_so_ung_vien, cau_hoi, danh_sach_tai_lieu, k):
    diem = [so_tu_khoa_trung_khop(cau_hoi, danh_sach_tai_lieu[i]) for i in cac_chi_so_ung_vien]
    thu_tu = sorted(range(len(cac_chi_so_ung_vien)), key=lambda j: diem[j], reverse=True)
    return [cac_chi_so_ung_vien[j] for j in thu_tu[:k]]


def truy_xuat_rag_cho_request(request, danh_sach_tai_lieu, danh_sach_vector, so_ung_vien, k_cuoi):
    ung_vien = tim_kiem_co_cach_ly(request["cau_hoi_vector"], danh_sach_tai_lieu, danh_sach_vector, request["khach_hang"], so_ung_vien)
    ket_qua_cuoi = rerank(ung_vien, request["cau_hoi"], [tl["noi_dung"] for tl in danh_sach_tai_lieu], k_cuoi)
    return ung_vien, ket_qua_cuoi


TAI_LIEU_RAG = [
    {"khach_hang": "A", "noi_dung": "san pham a bao hanh 12 thang chinh hang"},
    {"khach_hang": "A", "noi_dung": "gia ban san pham a la 5 trieu dong"},
    {"khach_hang": "A", "noi_dung": "ket noi mang wifi on dinh cho san pham a"},
    {"khach_hang": "B", "noi_dung": "san pham b bao hanh 24 thang toan quoc"},
    {"khach_hang": "B", "noi_dung": "mau sac san pham b co den va trang"},
    {"khach_hang": "B", "noi_dung": "pin san pham b sac nhanh trong 30 phut"},
]

VECTOR_RAG = [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.9, 0.1, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
]

CAU_HOI = "bao hanh san pham nay la bao lau"
CAU_HOI_VECTOR = [1.0, 0.0, 0.0]

REQUEST_A = {"khach_hang": "A", "cau_hoi": CAU_HOI, "cau_hoi_vector": CAU_HOI_VECTOR}
REQUEST_B = {"khach_hang": "B", "cau_hoi": CAU_HOI, "cau_hoi_vector": CAU_HOI_VECTOR}

ung_vien_a, ket_qua_a = truy_xuat_rag_cho_request(REQUEST_A, TAI_LIEU_RAG, VECTOR_RAG, 2, 1)
ung_vien_b, ket_qua_b = truy_xuat_rag_cho_request(REQUEST_B, TAI_LIEU_RAG, VECTOR_RAG, 2, 1)

print(ung_vien_a, ket_qua_a)
print(ung_vien_b, ket_qua_b)
print([TAI_LIEU_RAG[i]["khach_hang"] for i in ket_qua_a])
print([TAI_LIEU_RAG[i]["khach_hang"] for i in ket_qua_b])
```

```python title=test
assert ung_vien_a == [0, 1], f"ung_vien_a phai la [0, 1] -- dang ra {ung_vien_a}"
assert ket_qua_a == [0], f"ket_qua_a phai la [0] -- dang ra {ket_qua_a}"
assert ung_vien_b == [3, 4], f"ung_vien_b phai la [3, 4] -- dang ra {ung_vien_b}"
assert ket_qua_b == [3], f"ket_qua_b phai la [3] -- dang ra {ket_qua_b}"

# assert TRUC TIEP: khong bao gio ro ri qua khach hang, du CUNG cau_hoi_vector
assert all(TAI_LIEU_RAG[i]["khach_hang"] == "A" for i in ung_vien_a), "ung_vien_a KHONG DUOC chua doan cua khach hang khac A"
assert all(TAI_LIEU_RAG[i]["khach_hang"] == "B" for i in ung_vien_b), "ung_vien_b KHONG DUOC chua doan cua khach hang khac B"
assert 0 not in ung_vien_b and 0 not in ket_qua_b, "doan 0 (khach hang A) KHONG DUOC lot vao ket qua cua B"

# bien: doi khach_hang yeu cau (CUNG cau_hoi_vector khop TUYET DOI voi doan cua A) van cach ly dung
REQUEST_LEAK = {"khach_hang": "B", "cau_hoi": CAU_HOI, "cau_hoi_vector": [1.0, 0.0, 0.0]}
ung_vien_leak, ket_qua_leak = truy_xuat_rag_cho_request(REQUEST_LEAK, TAI_LIEU_RAG, VECTOR_RAG, 2, 1)
assert ung_vien_leak == [3, 4], f"REQUEST_LEAK van phai cach ly dung theo B -- dang ra {ung_vien_leak}"
assert ket_qua_leak == [3], f"REQUEST_LEAK van phai rerank dung trong so B -- dang ra {ket_qua_leak}"

# bien: so_ung_vien va k_cuoi THAT SU rang buoc ket qua
assert truy_xuat_rag_cho_request(REQUEST_A, TAI_LIEU_RAG, VECTOR_RAG, 1, 1) == ([0], [0]), "so_ung_vien=1 phai cho ung_vien_a la [0]"
assert truy_xuat_rag_cho_request(REQUEST_A, TAI_LIEU_RAG, VECTOR_RAG, 3, 1) == ([0, 1, 2], [0]), "so_ung_vien=3 phai lay het ca 3 doan cua A"
assert truy_xuat_rag_cho_request(REQUEST_A, TAI_LIEU_RAG, VECTOR_RAG, 3, 2) == ([0, 1, 2], [0, 1]), "k_cuoi=2 phai cho 2 phan tu sau rerank"
```

:::hints
- kind: attention
  body: "Hai cho trong, cung mot ham truy_xuat_rag_cho_request. Cho dau la GIA TRI GAN cho ung_vien -- goi lai HAM CO CACH LY da tai dung tu q8.6c bai 3, doc ca cau_hoi_vector LAN khach_hang TU CHINH request. Cho hai la GIA TRI GAN cho ket_qua_cuoi -- goi lai HAM RERANK da tai dung tu q8.6c bai 4, ap LEN CHINH ung_vien vua tinh (khong phai toan bo kho)."
- kind: strategy
  body: 'Cho dau: tim_kiem_co_cach_ly(request["cau_hoi_vector"], danh_sach_tai_lieu, danh_sach_vector, request["khach_hang"], so_ung_vien) -- dung NAM doi so theo dung thu tu da dinh nghia o q8.6c bai 3. Cho hai: rerank(ung_vien, request["cau_hoi"], [tl["noi_dung"] for tl in danh_sach_tai_lieu], k_cuoi) -- rerank nhan DANH SACH VAN BAN (khong phai danh sach dict), nen phai trich "noi_dung" ra truoc.'
- kind: one-line
  body: 'Cho dau la tim_kiem_co_cach_ly(request["cau_hoi_vector"], danh_sach_tai_lieu, danh_sach_vector, request["khach_hang"], so_ung_vien), cho hai la rerank(ung_vien, request["cau_hoi"], [tl["noi_dung"] for tl in danh_sach_tai_lieu], k_cuoi).'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT tim_kiem_co_cach_ly(...) voi dung nam doi so (khong duoc tu viet lai logic loc/xep hang); cho trong hai phai GOI THAT rerank(...) tren CHINH ung_vien vua tinh (khong duoc goi lai tim_kiem_co_cach_ly lan nua hay tra ve thang ung_vien)
  requireAst:
  - kind: uses-call, target: tim_kiem_co_cach_ly, min: 1
  - kind: uses-call, target: rerank, min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, dist build that tai
  # packages/exec-python/dist/kiem-ast.js, trich CHINH XAC tu solution cua
  # file nay -- gom ca toolkit cach ly/rerank tai dung tu q8.6c) -- xac nhan
  # DUNG CHINH XAC (min VA min+1): tim_kiem_co_cach_ly=1, rerank=1. CA HAI
  # ham nay CHI duoc goi DUNG 1 lan trong toan bo solution -- dung o hai cho
  # trong, khong noi nao khac (khong co demo goi lai rieng le, va ban than
  # hai ham nay khong tu goi lai chinh no).
  # Dien bua "True" vao ca hai cho trong ("ung_vien = True" va "ket_qua_cuoi
  # = True") cho CA HAI ve 0 -- CA HAI luat CHAN DUNG (da CHAY THAT xac nhan
  # qua kiemAst).
  #
  # GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho trong --
  # xac dinh ranh gioi TU CHINH khoi starter cua file nay, khong doan tay --
  # va CHAY THAT qua kiemAst() VA python3): dien 'rerank(ung_vien,
  # request["cau_hoi"], [tl["noi_dung"] for tl in danh_sach_tai_lieu],
  # k_cuoi)' vao cho trong dau (dong 'ung_vien = rerank(ung_vien,
  # request["cau_hoi"], [tl["noi_dung"] for tl in danh_sach_tai_lieu],
  # k_cuoi)') VA dien 'tim_kiem_co_cach_ly(request["cau_hoi_vector"],
  # danh_sach_tai_lieu, danh_sach_vector, request["khach_hang"], so_ung_vien)'
  # vao cho trong hai (dong 'ket_qua_cuoi =
  # tim_kiem_co_cach_ly(request["cau_hoi_vector"], danh_sach_tai_lieu,
  # danh_sach_vector, request["khach_hang"], so_ung_vien)') -- da CHAY THAT
  # qua kiemAst(): tong so lan goi tim_kiem_co_cach_ly VA rerank tren TOAN BO
  # solution DEU KHONG DOI (van dung 1 va 1, chi doi VI TRI) -- static KHONG
  # bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': dong dau tien sau hoan doi ("ung_vien =
  # rerank(ung_vien, ...)") DOC bien "ung_vien" o VE PHAI TRUOC KHI no duoc
  # GAN (ung_vien la bien LOCAL cua ham, do CHINH dong nay tao ra) -- da tu
  # chay THAT qua python3, xac nhan no nem UnboundLocalError ("cannot access
  # local variable 'ung_vien' where it is not associated with a value") NGAY
  # khi truy_xuat_rag_cho_request duoc goi lan dau -- bi chan boi tier 'run',
  # doc lap voi static.
  # Da tu ra soat GOTCHA #6: tim_kiem_co_cach_ly va rerank la HAI TEN HAM
  # rieng biet, khong trung voi ham nao khac trong toan bo toolkit da tai
  # dung tu q8.6c (tich_vo_huong, do_dai_vector, tuong_dong_cosine,
  # loc_theo_khach_hang, so_tu_khoa_trung_khop -- khong ten nao trung).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[0, 1\\] \\[0\\]\\n\\[3, 4\\] \\[3\\]\\n\\['A'\\]\\n\\['B'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`['A']` và `['B']` — dù `REQUEST_LEAK` khớp TUYỆT ĐỐI với một tài liệu của
khách hàng khác, nó KHÔNG BAO GIỜ lọt vào kết quả. Bài tiếp theo hỏi: sau
khi có kết quả RAG, hệ thống ghi nhận gì, và làm sao biết chi phí có đang
tăng bất thường không?
::::

::::reflect{#nghi-lai}
Điểm mấu chốt của bài này không phải MỘT thuật toán mới — `tim_kiem_co_cach_ly`
và `rerank` đều LÀ những hàm đã kiểm chứng ở `q8.6c`. Cái RÁP LẠI này chứng
minh được LÀ: khi request được biểu diễn ĐÚNG (một dict mang cả
`khach_hang`, `cau_hoi`, `cau_hoi_vector`), việc gọi đúng hai hàm đó theo
đúng thứ tự CHO RA một pipeline không rò rỉ — bất kể `cau_hoi_vector` GIỐNG
tới đâu giữa hai khách hàng khác nhau. `REQUEST_LEAK` (khớp cosine `1,0`
với một tài liệu của A trong khi hỏi với `khach_hang="B"`) là bằng chứng cụ
thể: cách ly xảy ra Ở TẦNG DỮ LIỆU (lọc trước khi tính điểm), không phải ở
tầng "may mắn" (hy vọng điểm số tự nhiên thấp).
::::

::::checkpoint{mastery=0.84}
::::
