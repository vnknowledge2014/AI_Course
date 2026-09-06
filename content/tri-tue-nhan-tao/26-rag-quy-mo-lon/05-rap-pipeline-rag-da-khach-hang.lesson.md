---
id: tri-tue-nhan-tao.rag-quy-mo-lon.rap-pipeline-rag-da-khach-hang
title: "Ráp pipeline RAG đa khách hàng: nạp + phân mảnh + cách ly + rerank"
summary: "pipeline_rag_da_khach_hang(hang_doi_nap, so_manh, tu_vung, cum_tu_goc, cau_hoi, khach_hang_yeu_cau, so_ung_vien, k_cuoi) rap (a) xu_ly_hang_doi_nap voi xu_ly_mot_tai_lieu (tach cau + gan khach_hang, bai 1), (b) phan_manh de ghi so bo (bai 2), (c) tim_kiem_co_cach_ly (bai 3) tren toan bo doan, (d) rerank (bai 4) tren top so_ung_vien. Tren HANG_DOI_NAP gom 2 tai lieu tho MOI khach hang A/B (4 doan/khach hang sau khi tach cau, tong 8 doan), CAU_HOI ve bao hanh: khach hang A duoc [0] (dung doan A dau tien noi ve bao hanh), khach hang B duoc [4] (doan B tuong ung) -- KHONG BAO GIO doan cua nguoi kia. Doi so_manh tu 2 sang 3 KHONG doi ung_vien/ket_qua (sharding la bookkeeping vat ly, doc lap voi loc logic theo khach hang) nhung DOI cac_manh (bookkeeping)."
locale: vi
track: tri-tue-nhan-tao
module: rag-quy-mo-lon
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.rap-pipeline-rag-da-khach-hang]
requires: [ai.mo-rong-hybrid-voi-rerank]
concepts: [ai.rap-pipeline-rag-da-khach-hang]
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
Bốn bài: nạp theo sự kiện, phân mảnh, cô lập theo khách hàng, rerank. Bài
này KHÔNG thêm khái niệm mới — nó RÁP LẠI cả bốn thành MỘT pipeline duy
nhất, đúng thứ tự `Chương 39.4` mô tả một hệ RAG doanh nghiệp thật vận
hành.
::::

::::explain{#rap_bon_manh_thanh_mot_pipeline}
Một hệ RAG đa khách hàng, khi tài liệu MỚI đổ về VÀ một câu hỏi tới, làm
đúng BỐN việc theo MỘT thứ tự cố định:

1. **Nạp theo sự kiện** (bài `1`) — mỗi tài liệu thô đi qua hàng đợi, được
   tách thành các đoạn nhỏ hơn, MỖI đoạn giữ lại đúng `khach_hang` của tài
   liệu gốc.
2. **Phân mảnh** (bài `2`) — mọi đoạn (bất kể khách hàng nào) được rải đều
   vào `so_manh` mảnh lưu trữ, chỉ để ghi sổ VẬT LÝ dữ liệu nằm ở đâu.
3. **Cô lập theo khách hàng** (bài `3`) — khi khách hàng gửi câu hỏi, CHỈ
   tìm trong số đoạn THUỘC ĐÚNG khách hàng đó (lọc TRƯỚC khi tính điểm).
4. **Rerank** (bài `4`) — trong số các ứng viên đã cô lập đúng, xếp lại
   theo độ khớp từ khoá chính xác.

```
pipeline_rag_da_khach_hang(hang_doi_nap, so_manh, tu_vung, cum_tu_goc,
                            cau_hoi, khach_hang_yeu_cau, so_ung_vien, k_cuoi):
  toan_bo_doan = xu_ly_hang_doi_nap(hang_doi_nap, xu_ly_mot_tai_lieu)   # (a)
  cac_manh = phan_manh(toan_bo_doan, so_manh)                            # (b)
  vector_toan_bo = [vector hoa moi doan CHO MOI doan TRONG toan_bo_doan]
  cau_hoi_vector = vector hoa cau_hoi
  ung_vien = tim_kiem_co_cach_ly(cau_hoi_vector, toan_bo_doan,
                                  vector_toan_bo, khach_hang_yeu_cau, so_ung_vien)  # (c)
  ket_qua_cuoi = rerank(ung_vien, cau_hoi,
                         [noi_dung CHO MOI doan TRONG toan_bo_doan], k_cuoi)        # (d)
  tra ve toan_bo_doan, cac_manh, ung_vien, ket_qua_cuoi
```

Điểm quan trọng: bước `(b)` (phân mảnh) chỉ ảnh hưởng tới `cac_manh` (bookkeeping
lưu trữ) — nó KHÔNG hề tham gia vào bước `(c)` (tìm kiếm cô lập), vì tìm kiếm
chạy trên `toan_bo_doan` đầy đủ, lọc theo `khach_hang_yeu_cau`, không phụ
thuộc tài liệu nằm ở MẢNH nào. Sharding (phân bố VẬT LÝ) và tenant isolation
(lọc LOGIC) là hai mối quan tâm HOÀN TOÀN tách biệt.
::::

::::example{#pipeline_day_du_hai_khach_hang}
```python title=readonly
import math


def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


def xu_ly_hang_doi_nap(hang_doi_tai_lieu, ham_xu_ly):
    hang_doi = list(hang_doi_tai_lieu)
    ket_qua = []
    while hang_doi:
        tai_lieu = hang_doi.pop(0)
        ket_qua.extend(ham_xu_ly(tai_lieu))
    return ket_qua


def tim_manh_chua(chi_so_tai_lieu, so_manh):
    return chi_so_tai_lieu % so_manh


def phan_manh(danh_sach_tai_lieu, so_manh):
    cac_manh = [[] for _ in range(so_manh)]
    for chi_so, tai_lieu in enumerate(danh_sach_tai_lieu):
        chi_so_manh = tim_manh_chua(chi_so, so_manh)
        cac_manh[chi_so_manh].append(tai_lieu)
    return cac_manh


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


TU_VUNG_BOSS = ["bao_hanh", "gia_ca", "ket_noi_mang", "pin", "mau_sac", "kich_thuoc"]
CUM_TU_GOC_BOSS = {
    "bao_hanh": "bao hanh", "gia_ca": "gia ca", "ket_noi_mang": "ket noi mang",
    "pin": "pin", "mau_sac": "mau sac", "kich_thuoc": "kich thuoc",
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


def xu_ly_mot_tai_lieu(tai_lieu_tho):
    cac_cau = chia_theo_cau(tai_lieu_tho["noi_dung"])
    return [{"khach_hang": tai_lieu_tho["khach_hang"], "noi_dung": cau} for cau in cac_cau]


def pipeline_rag_da_khach_hang(hang_doi_nap, so_manh, tu_vung, cum_tu_goc, cau_hoi, khach_hang_yeu_cau, so_ung_vien, k_cuoi):
    toan_bo_doan = xu_ly_hang_doi_nap(hang_doi_nap, xu_ly_mot_tai_lieu)
    cac_manh = phan_manh(toan_bo_doan, so_manh)
    vector_toan_bo = [tinh_vector_dem_tu(d["noi_dung"], tu_vung, cum_tu_goc) for d in toan_bo_doan]
    cau_hoi_vector = tinh_vector_dem_tu(cau_hoi, tu_vung, cum_tu_goc)
    ung_vien = tim_kiem_co_cach_ly(cau_hoi_vector, toan_bo_doan, vector_toan_bo, khach_hang_yeu_cau, so_ung_vien)
    ket_qua_cuoi = rerank(ung_vien, cau_hoi, [d["noi_dung"] for d in toan_bo_doan], k_cuoi)
    return toan_bo_doan, cac_manh, ung_vien, ket_qua_cuoi


HANG_DOI_NAP = [
    {"khach_hang": "A", "noi_dung": "san pham cua khach hang a co bao hanh 12 thang. gia ca san pham la 5 trieu dong."},
    {"khach_hang": "A", "noi_dung": "ket noi mang cua san pham on dinh. pin su dung duoc 2 ngay."},
    {"khach_hang": "B", "noi_dung": "san pham cua khach hang b co bao hanh 24 thang. mau sac co ca den va trang."},
    {"khach_hang": "B", "noi_dung": "kich thuoc man hinh la 6 inch. pin sac nhanh trong 30 phut."},
]

CAU_HOI = "bao hanh san pham nay dung duoc bao lau"

toan_bo_doan, cac_manh, ung_vien_a, ket_qua_a = pipeline_rag_da_khach_hang(
    HANG_DOI_NAP, 2, TU_VUNG_BOSS, CUM_TU_GOC_BOSS, CAU_HOI, "A", 3, 1
)
_, _, ung_vien_b, ket_qua_b = pipeline_rag_da_khach_hang(
    HANG_DOI_NAP, 2, TU_VUNG_BOSS, CUM_TU_GOC_BOSS, CAU_HOI, "B", 3, 1
)

print(len(toan_bo_doan), sum(len(m) for m in cac_manh))
print(ung_vien_a, ket_qua_a)
print(ung_vien_b, ket_qua_b)
print([toan_bo_doan[i]["khach_hang"] for i in ket_qua_a])
print([toan_bo_doan[i]["khach_hang"] for i in ket_qua_b])
```

```text title=readonly
8 8
[0, 1, 2] [0]
[4, 5, 6] [4]
['A']
['B']
```

`HANG_DOI_NAP` có `4` tài liệu thô (`2` của A, `2` của B), mỗi tài liệu `2`
câu — sau bước `(a)`, `toan_bo_doan` có đúng `8` đoạn, THEO ĐÚNG thứ tự hàng
đợi: đoạn `0-3` thuộc `A`, đoạn `4-7` thuộc `B`. Bước `(b)` phân `8` đoạn đó
vào `2` mảnh — `sum(len(m) for m in cac_manh) = 8`, khớp đúng bất biến bài
`2`. Hỏi với `khach_hang_yeu_cau = "A"`: `ung_vien_a = [0, 1, 2]` — CHỈ chỉ
số của A (bước `(c)` cô lập đúng); `ket_qua_a = [0]` — sau rerank (bước
`(d)`), đoạn `0` (`"san pham cua khach hang a co bao hanh 12 thang"`, khớp
NHIỀU từ khoá với câu hỏi) đứng đầu. Hỏi với `"B"`: `ung_vien_b = [4, 5, 6]`
— CHỈ chỉ số của B; `ket_qua_b = [4]` — đoạn tương ứng của B. Hai dòng in
cuối xác nhận TRỰC TIẾP: `ket_qua_a` toàn bộ thuộc `'A'`, `ket_qua_b` toàn
bộ thuộc `'B'` — không có chỉ số nào của khách hàng kia lọt vào.
::::

::::predict{#doan_doi_so_manh_khong_doi_ket_qua commitOnce}
Xét việc gọi lại `pipeline_rag_da_khach_hang` với TẤT CẢ tham số GIỮ NGUYÊN
như ví dụ trên (CÙNG `HANG_DOI_NAP`, CÙNG `CAU_HOI`, CÙNG
`khach_hang_yeu_cau = "A"`, CÙNG `so_ung_vien = 3`, CÙNG `k_cuoi = 1`), CHỈ
đổi `so_manh` từ `2` THÀNH `3`.

**Trước khi chạy thử**, bạn đoán: `ung_vien_a` (kết quả bước cô lập) VÀ
`ket_qua_a` (kết quả sau rerank) có ĐỔI theo không?

:::opt{correct}
KHÔNG đổi — `ung_vien_a` vẫn là `[0, 1, 2]`, `ket_qua_a` vẫn là `[0]`; chỉ
`cac_manh` (bookkeeping phân mảnh) đổi. Tìm kiếm cô lập (bước `c`) chạy
trên `toan_bo_doan` VÀ `khach_hang_yeu_cau`, không hề đọc `cac_manh` hay
`so_manh` ở bất kỳ đâu trong logic của nó
:::

:::opt
Có đổi — đổi `so_manh` làm các đoạn rơi vào MẢNH khác, nên bước tìm kiếm
cũng phải tìm trong MẢNH khác, cho ra ứng viên khác
::why
Gần đúng ở việc đổi `so_manh` THẬT SỰ làm `cac_manh` đổi — mỗi đoạn có thể
rơi vào một mảnh khác so với trước (quan sát về BOOKKEEPING đó đúng).

Chỗ lệch: `tim_kiem_co_cach_ly` (bước `c`) nhận đối số `toan_bo_doan` (danh
sách ĐẦY ĐỦ, không chia mảnh) và `khach_hang_yeu_cau` — nó KHÔNG hề nhận
`cac_manh` hay biết gì về mảnh nào chứa đoạn nào. Sharding chỉ là một sổ ghi
chép VẬT LÝ song song, không phải một bước LỌC trong luồng tìm kiếm.
::
:::

:::opt
Không đổi `ung_vien_a`, nhưng `ket_qua_a` đổi vì bước rerank ĐỌC `cac_manh`
để ưu tiên tài liệu trong cùng một mảnh
::why
Gần đúng ở việc `ung_vien_a` THẬT SỰ không đổi — quan sát đó đúng, vì bước
`(c)` không phụ thuộc `so_manh`.

Chỗ lệch: `rerank` (bước `d`) chỉ nhận `cac_chi_so_ung_vien`, `cau_hoi`, và
nội dung văn bản của các đoạn — nó KHÔNG nhận `cac_manh` làm đối số nào cả,
nên không có cách nào để nó "đọc" thông tin mảnh. Điểm số của `rerank` chỉ
dựa vào số từ khoá khớp chính xác giữa `cau_hoi` và văn bản đoạn.
::
:::
::::

::::code{#viet_pipeline_rag_da_khach_hang}
Hoàn thiện `pipeline_rag_da_khach_hang`: gọi ĐÚNG hàm cô lập (bài `3`) để
lấy ứng viên, rồi gọi ĐÚNG hàm rerank (bài `4`) trên các ứng viên đó để có
kết quả cuối cùng.

```python title=starter
import math


def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


def xu_ly_hang_doi_nap(hang_doi_tai_lieu, ham_xu_ly):
    hang_doi = list(hang_doi_tai_lieu)
    ket_qua = []
    while hang_doi:
        tai_lieu = hang_doi.pop(0)
        ket_qua.extend(ham_xu_ly(tai_lieu))
    return ket_qua


def tim_manh_chua(chi_so_tai_lieu, so_manh):
    return chi_so_tai_lieu % so_manh


def phan_manh(danh_sach_tai_lieu, so_manh):
    cac_manh = [[] for _ in range(so_manh)]
    for chi_so, tai_lieu in enumerate(danh_sach_tai_lieu):
        chi_so_manh = tim_manh_chua(chi_so, so_manh)
        cac_manh[chi_so_manh].append(tai_lieu)
    return cac_manh


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


TU_VUNG_BOSS = ["bao_hanh", "gia_ca", "ket_noi_mang", "pin", "mau_sac", "kich_thuoc"]
CUM_TU_GOC_BOSS = {
    "bao_hanh": "bao hanh", "gia_ca": "gia ca", "ket_noi_mang": "ket noi mang",
    "pin": "pin", "mau_sac": "mau sac", "kich_thuoc": "kich thuoc",
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


def xu_ly_mot_tai_lieu(tai_lieu_tho):
    cac_cau = chia_theo_cau(tai_lieu_tho["noi_dung"])
    return [{"khach_hang": tai_lieu_tho["khach_hang"], "noi_dung": cau} for cau in cac_cau]


def pipeline_rag_da_khach_hang(hang_doi_nap, so_manh, tu_vung, cum_tu_goc, cau_hoi, khach_hang_yeu_cau, so_ung_vien, k_cuoi):
    toan_bo_doan = xu_ly_hang_doi_nap(hang_doi_nap, xu_ly_mot_tai_lieu)
    cac_manh = phan_manh(toan_bo_doan, so_manh)
    vector_toan_bo = [tinh_vector_dem_tu(d["noi_dung"], tu_vung, cum_tu_goc) for d in toan_bo_doan]
    cau_hoi_vector = tinh_vector_dem_tu(cau_hoi, tu_vung, cum_tu_goc)
    ung_vien = ___                              # tim_kiem_co_cach_ly(cau_hoi_vector, toan_bo_doan, vector_toan_bo, khach_hang_yeu_cau, so_ung_vien)
    ket_qua_cuoi = ___                           # rerank(ung_vien, cau_hoi, [d["noi_dung"] for d in toan_bo_doan], k_cuoi)
    return toan_bo_doan, cac_manh, ung_vien, ket_qua_cuoi


HANG_DOI_NAP = [
    {"khach_hang": "A", "noi_dung": "san pham cua khach hang a co bao hanh 12 thang. gia ca san pham la 5 trieu dong."},
    {"khach_hang": "A", "noi_dung": "ket noi mang cua san pham on dinh. pin su dung duoc 2 ngay."},
    {"khach_hang": "B", "noi_dung": "san pham cua khach hang b co bao hanh 24 thang. mau sac co ca den va trang."},
    {"khach_hang": "B", "noi_dung": "kich thuoc man hinh la 6 inch. pin sac nhanh trong 30 phut."},
]

CAU_HOI = "bao hanh san pham nay dung duoc bao lau"

toan_bo_doan, cac_manh, ung_vien_a, ket_qua_a = pipeline_rag_da_khach_hang(
    HANG_DOI_NAP, 2, TU_VUNG_BOSS, CUM_TU_GOC_BOSS, CAU_HOI, "A", 3, 1
)
_, _, ung_vien_b, ket_qua_b = pipeline_rag_da_khach_hang(
    HANG_DOI_NAP, 2, TU_VUNG_BOSS, CUM_TU_GOC_BOSS, CAU_HOI, "B", 3, 1
)

print(len(toan_bo_doan), sum(len(m) for m in cac_manh))
print(ung_vien_a, ket_qua_a)
print(ung_vien_b, ket_qua_b)
print([toan_bo_doan[i]["khach_hang"] for i in ket_qua_a])
print([toan_bo_doan[i]["khach_hang"] for i in ket_qua_b])
```

```python title=solution
import math


def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


def xu_ly_hang_doi_nap(hang_doi_tai_lieu, ham_xu_ly):
    hang_doi = list(hang_doi_tai_lieu)
    ket_qua = []
    while hang_doi:
        tai_lieu = hang_doi.pop(0)
        ket_qua.extend(ham_xu_ly(tai_lieu))
    return ket_qua


def tim_manh_chua(chi_so_tai_lieu, so_manh):
    return chi_so_tai_lieu % so_manh


def phan_manh(danh_sach_tai_lieu, so_manh):
    cac_manh = [[] for _ in range(so_manh)]
    for chi_so, tai_lieu in enumerate(danh_sach_tai_lieu):
        chi_so_manh = tim_manh_chua(chi_so, so_manh)
        cac_manh[chi_so_manh].append(tai_lieu)
    return cac_manh


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


TU_VUNG_BOSS = ["bao_hanh", "gia_ca", "ket_noi_mang", "pin", "mau_sac", "kich_thuoc"]
CUM_TU_GOC_BOSS = {
    "bao_hanh": "bao hanh", "gia_ca": "gia ca", "ket_noi_mang": "ket noi mang",
    "pin": "pin", "mau_sac": "mau sac", "kich_thuoc": "kich thuoc",
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


def xu_ly_mot_tai_lieu(tai_lieu_tho):
    cac_cau = chia_theo_cau(tai_lieu_tho["noi_dung"])
    return [{"khach_hang": tai_lieu_tho["khach_hang"], "noi_dung": cau} for cau in cac_cau]


def pipeline_rag_da_khach_hang(hang_doi_nap, so_manh, tu_vung, cum_tu_goc, cau_hoi, khach_hang_yeu_cau, so_ung_vien, k_cuoi):
    toan_bo_doan = xu_ly_hang_doi_nap(hang_doi_nap, xu_ly_mot_tai_lieu)
    cac_manh = phan_manh(toan_bo_doan, so_manh)
    vector_toan_bo = [tinh_vector_dem_tu(d["noi_dung"], tu_vung, cum_tu_goc) for d in toan_bo_doan]
    cau_hoi_vector = tinh_vector_dem_tu(cau_hoi, tu_vung, cum_tu_goc)
    ung_vien = tim_kiem_co_cach_ly(cau_hoi_vector, toan_bo_doan, vector_toan_bo, khach_hang_yeu_cau, so_ung_vien)
    ket_qua_cuoi = rerank(ung_vien, cau_hoi, [d["noi_dung"] for d in toan_bo_doan], k_cuoi)
    return toan_bo_doan, cac_manh, ung_vien, ket_qua_cuoi


HANG_DOI_NAP = [
    {"khach_hang": "A", "noi_dung": "san pham cua khach hang a co bao hanh 12 thang. gia ca san pham la 5 trieu dong."},
    {"khach_hang": "A", "noi_dung": "ket noi mang cua san pham on dinh. pin su dung duoc 2 ngay."},
    {"khach_hang": "B", "noi_dung": "san pham cua khach hang b co bao hanh 24 thang. mau sac co ca den va trang."},
    {"khach_hang": "B", "noi_dung": "kich thuoc man hinh la 6 inch. pin sac nhanh trong 30 phut."},
]

CAU_HOI = "bao hanh san pham nay dung duoc bao lau"

toan_bo_doan, cac_manh, ung_vien_a, ket_qua_a = pipeline_rag_da_khach_hang(
    HANG_DOI_NAP, 2, TU_VUNG_BOSS, CUM_TU_GOC_BOSS, CAU_HOI, "A", 3, 1
)
_, _, ung_vien_b, ket_qua_b = pipeline_rag_da_khach_hang(
    HANG_DOI_NAP, 2, TU_VUNG_BOSS, CUM_TU_GOC_BOSS, CAU_HOI, "B", 3, 1
)

print(len(toan_bo_doan), sum(len(m) for m in cac_manh))
print(ung_vien_a, ket_qua_a)
print(ung_vien_b, ket_qua_b)
print([toan_bo_doan[i]["khach_hang"] for i in ket_qua_a])
print([toan_bo_doan[i]["khach_hang"] for i in ket_qua_b])
```

```python title=test
assert len(toan_bo_doan) == 8, f"toan_bo_doan phai co 8 doan (4 tai lieu tho x 2 cau) -- dang ra {len(toan_bo_doan)}"
assert sum(len(m) for m in cac_manh) == 8, f"tong tren cac_manh phai bang 8 -- dang ra {sum(len(m) for m in cac_manh)}"
assert ung_vien_a == [0, 1, 2], f"ung_vien_a phai la [0, 1, 2] -- dang ra {ung_vien_a}"
assert ket_qua_a == [0], f"ket_qua_a phai la [0] -- dang ra {ket_qua_a}"
assert ung_vien_b == [4, 5, 6], f"ung_vien_b phai la [4, 5, 6] -- dang ra {ung_vien_b}"
assert ket_qua_b == [4], f"ket_qua_b phai la [4] -- dang ra {ket_qua_b}"

# assert TRUC TIEP: khong bao gio ro ri qua khach hang
assert all(toan_bo_doan[i]["khach_hang"] == "A" for i in ket_qua_a), "ket_qua_a KHONG DUOC chua doan cua khach hang khac A"
assert all(toan_bo_doan[i]["khach_hang"] == "B" for i in ket_qua_b), "ket_qua_b KHONG DUOC chua doan cua khach hang khac B"
assert all(toan_bo_doan[i]["khach_hang"] == "A" for i in ung_vien_a), "ung_vien_a KHONG DUOC chua doan cua khach hang khac A"
assert all(toan_bo_doan[i]["khach_hang"] == "B" for i in ung_vien_b), "ung_vien_b KHONG DUOC chua doan cua khach hang khac B"

# bien: so_manh KHONG anh huong ket qua logic (sharding doc lap voi cach ly)
_, cac_manh_3, ung_vien_a_manh3, ket_qua_a_manh3 = pipeline_rag_da_khach_hang(
    HANG_DOI_NAP, 3, TU_VUNG_BOSS, CUM_TU_GOC_BOSS, CAU_HOI, "A", 3, 1
)
assert ung_vien_a_manh3 == ung_vien_a, f"doi so_manh khong duoc doi ung_vien_a -- dang ra {ung_vien_a_manh3}"
assert ket_qua_a_manh3 == ket_qua_a, f"doi so_manh khong duoc doi ket_qua_a -- dang ra {ket_qua_a_manh3}"
assert len(cac_manh_3) == 3, f"cac_manh_3 phai co DUNG 3 manh -- dang ra {len(cac_manh_3)}"
assert cac_manh_3 != cac_manh, "cac_manh (bookkeeping) PHAI doi khi so_manh doi tu 2 sang 3"

# bien: so_ung_vien va k_cuoi THAT SU rang buoc pipeline
_, _, ung_vien_a_2, ket_qua_a_2 = pipeline_rag_da_khach_hang(
    HANG_DOI_NAP, 2, TU_VUNG_BOSS, CUM_TU_GOC_BOSS, CAU_HOI, "A", 2, 2
)
assert ung_vien_a_2 == [0, 1], f"so_ung_vien=2 phai cho ung_vien_a_2 la [0, 1] -- dang ra {ung_vien_a_2}"
assert ket_qua_a_2 == [0, 1], f"k_cuoi=2 phai cho ket_qua_a_2 la [0, 1] -- dang ra {ket_qua_a_2}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cùng một hàm `pipeline_rag_da_khach_hang`, ở BƯỚC SAU khi đã có `vector_toan_bo` VÀ `cau_hoi_vector`. Chỗ đầu là GIÁ TRỊ GÁN cho `ung_vien` — gọi lại hàm cô lập đã viết ở bài `3`. Chỗ hai là GIÁ TRỊ GÁN cho `ket_qua_cuoi` — gọi lại hàm rerank đã viết ở bài `4`, áp lên CHÍNH `ung_vien` vừa tính.
- kind: strategy
  body: 'Chỗ đầu: `tim_kiem_co_cach_ly(cau_hoi_vector, toan_bo_doan, vector_toan_bo, khach_hang_yeu_cau, so_ung_vien)` — đúng năm đối số theo thứ tự đã định nghĩa ở bài `3`. Chỗ hai: `rerank(ung_vien, cau_hoi, [d["noi_dung"] for d in toan_bo_doan], k_cuoi)` — rerank nhận DANH SÁCH VĂN BẢN (không phải danh sách dict), nên phải trích `"noi_dung"` ra trước.'
- kind: one-line
  body: 'Chỗ đầu là `tim_kiem_co_cach_ly(cau_hoi_vector, toan_bo_doan, vector_toan_bo, khach_hang_yeu_cau, so_ung_vien)`, chỗ hai là `rerank(ung_vien, cau_hoi, [d["noi_dung"] for d in toan_bo_doan], k_cuoi)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT tim_kiem_co_cach_ly(...) voi dung nam doi so (khong duoc tu viet lai logic loc/xep hang); cho trong hai phai GOI THAT rerank(...) tren CHINH ung_vien vua tinh (khong duoc goi lai tim_kiem_co_cach_ly lan nua hay tra ve thang ung_vien)
  requireAst:
  - kind: uses-call, target: tim_kiem_co_cach_ly, min: 1
  - kind: uses-call, target: rerank, min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, dist build that, trich
  # solution TU CHINH file nay -- gom toan bo bon toolkit da tai dung tu
  # bai 1-4) -- xac nhan CHINH XAC (min VA min+1): tim_kiem_co_cach_ly=1,
  # rerank=1. CA HAI ham nay CHI duoc goi DUNG 1 lan trong toan bo solution
  # -- dung o hai cho trong, khong noi nao khac (khong co demo goi lai rieng
  # le, va ban than hai ham nay khong tu goi lai chinh no).
  # Dien bua "True" vao ca hai cho trong ("ung_vien = True" va "ket_qua_cuoi
  # = True") cho CA HAI ve 0 -- CA HAI luat CHAN DUNG (da CHAY THAT xac nhan
  # qua kiemAst).
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho
  # trong -- xac dinh ranh gioi TU CHINH khoi starter -- va CHAY THAT qua
  # kiemAst() THAT VA python3 THAT): dien "rerank(ung_vien, cau_hoi,
  # [d[\"noi_dung\"] for d in toan_bo_doan], k_cuoi)" vao cho trong dau
  # ("ung_vien = rerank(ung_vien, cau_hoi, [d[\"noi_dung\"] for d in
  # toan_bo_doan], k_cuoi)") VA dien "tim_kiem_co_cach_ly(cau_hoi_vector,
  # toan_bo_doan, vector_toan_bo, khach_hang_yeu_cau, so_ung_vien)" vao cho
  # trong hai ("ket_qua_cuoi = tim_kiem_co_cach_ly(cau_hoi_vector,
  # toan_bo_doan, vector_toan_bo, khach_hang_yeu_cau, so_ung_vien)") -- da
  # CHAY THAT qua kiemAst(): tong so lan goi tim_kiem_co_cach_ly VA rerank
  # tren TOAN BO solution DEU KHONG DOI (van dung 1 va 1, chi doi VI TRI)
  # -- static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': dong dau tien "ung_vien =
  # rerank(ung_vien, ...)" DOC bien "ung_vien" o VE PHAI TRUOC KHI no duoc
  # GAN (day la bien LOCAL do chinh dong nay tao ra trong
  # pipeline_rag_da_khach_hang) -- da tu chay THAT qua python3, xac nhan no
  # nem UnboundLocalError ("cannot access local variable 'ung_vien' where
  # it is not associated with a value") NGAY khi pipeline_rag_da_khach_hang
  # duoc goi lan dau, TRUOC CA khi kip chay toi dong ket_qua_cuoi. Bi chan
  # boi tier 'run', doc lap voi static.
  # Da tu ra soat GOTCHA #6: tim_kiem_co_cach_ly va rerank la HAI TEN HAM
  # rieng biet, khong trung voi ham nao khac trong toan bo bon toolkit da
  # tai dung (chia_theo_cau, xu_ly_hang_doi_nap, tim_manh_chua, phan_manh,
  # loc_theo_khach_hang, so_tu_khoa_trung_khop, v.v. -- khong ten nao trung).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^8 8\\n\\[0, 1, 2\\] \\[0\\]\\n\\[4, 5, 6\\] \\[4\\]\\n\\['A'\\]\\n\\['B'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`['A']` và `['B']` — mỗi khách hàng chỉ nhận ĐÚNG dữ liệu của chính mình,
qua cả bốn bước: nạp, phân mảnh, cô lập, rerank. Bài cuối cùng đo một tình
huống CỤ THỂ hơn: nếu lọc SAI thứ tự (SAU thay vì TRƯỚC), rò rỉ sẽ trông
như thế nào bằng số?
::::

::::reflect{#nghi-lai}
Bài này không xây thuật toán mới — mọi mảnh (`xu_ly_hang_doi_nap`,
`phan_manh`, `tim_kiem_co_cach_ly`, `rerank`) đã được viết VÀ kiểm chứng
riêng ở bốn bài trước. Điều pipeline này chứng minh được LÀ: ráp đúng thứ
tự (nạp → phân mảnh → cô lập → rerank) cho ra một hệ thống mà **mỗi khách
hàng chỉ thấy dữ liệu của chính mình**, đo bằng số cụ thể (`8` đoạn nạp vào,
`8` đoạn phân mảnh, ứng viên VÀ kết quả cuối của A/B không hề giao nhau).
Một phát hiện phụ quan trọng: đổi `so_manh` (một quyết định THUẦN VẬN HÀNH,
về việc lưu trữ vật lý) không hề ảnh hưởng `ung_vien`/`ket_qua_cuoi` (một
quyết định LOGIC, về việc ai được thấy gì) — hai mối quan tâm này ĐỘC LẬP
với nhau trong thiết kế đúng.

Bài cuối cùng đo sâu hơn: dựng MỘT tình huống cụ thể nơi nếu lọc SAI thứ tự
(tính điểm rồi mới lọc, thay vì lọc rồi mới tính điểm), một tài liệu của
khách hàng KHÁC sẽ THẬT SỰ lọt vào kết quả — chứng minh bằng chạy thật, đóng
`q8.6c` tại `6/6`.
::::

::::checkpoint{mastery=0.85}
::::
