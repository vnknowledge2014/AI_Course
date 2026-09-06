---
id: tri-tue-nhan-tao.boss-ha-tang-van-hanh-ai.boss-ha-tang-van-hanh-ai
title: "BOSS quý — hạ tầng & vận hành AI đầu-cuối: cổng + hàng đợi + RAG + tác tử, đóng T8.6 tại 26/26 VÀ R8 tại 190/190"
summary: "xu_ly_toan_bo_he_thong(...) rap CA BON bai cua q8.6e: (a) do_thong_luong_hang_doi_da_qua_cong chay MOI request qua cong (bai 1) roi do thong luong gop lo/tuan tu tren PHAN duoc_chap_nhan (bai 2); (b) dem so_duoc_phuc_vu/so_tu_choi_sai_key/so_tu_choi_vuot_han_muc tu ket_qua_cong; (c) VOI TUNG request duoc_chap_nhan, goi truy_xuat_rag_cho_request (bai 3) roi buoc_tac_tu_va_do_chi_phi (bai 4) tren tap cau_hoi_vector CUA RIENG cac request duoc phuc vu. Tren MOT tai gia 10 request hon hop (6 hop le -- 1 cau hoi TRUNG voi cau truoc, 2 sai key, 2 vuot han muc): so_duoc_phuc_vu=6, so_tu_choi_sai_key=2, so_tu_choi_vuot_han_muc=2, ty_le_cache_hit=4/6~0,6667, thong_luong_gop_lo~0,013699, thong_luong_tuan_tu~0,004651, huong_xu_ly LA dict 'chi_phi_tang_bat_thuong' (0,6667 < nguong 0,7). Dong T8.6 tai 26/26 VA R8 tai 190/190."
locale: vi
track: tri-tue-nhan-tao
module: boss-ha-tang-van-hanh-ai
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-ha-tang-van-hanh-ai]
requires: [ai.tac-tu-cache-va-chi-phi]
concepts: [ai.boss-ha-tang-van-hanh-ai]
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
Bốn bài: cổng xử lý MỘT request (bài `1`), hàng đợi gộp lô đo thông lượng
(bài `2`), RAG đa khách hàng không rò rỉ (bài `3`), tác tử ghi sự kiện + đo
cache/chi phí (bài `4`). BOSS quý này KHÔNG thêm khái niệm mới — nó RÁP CẢ
BỐN thành MỘT luồng xử lý request đầu-cuối, đúng như `q8.6e` đã hứa từ đầu.
::::

::::explain{#rap_bon_buoc_thanh_mot_he_thong}
Một hệ thống AI vận hành thật, khi MỘT lô request hỗn hợp tới, làm đúng BỐN
việc theo MỘT thứ tự cố định:

1. **Cổng + hàng đợi** (bài `1` + `2`) — mỗi request đi qua cổng (xác thực,
   token bucket, định tuyến); TRONG SỐ những request được chấp nhận, đo
   thông lượng gộp lô SO VỚI tuần tự.
2. **Đếm ba kết cục** — từ CHÍNH kết quả cổng, đếm bao nhiêu được phục vụ,
   bao nhiêu sai key, bao nhiêu vượt hạn mức.
3. **RAG đa khách hàng** (bài `3`) — CHỈ những request đã qua cổng mới được
   truy xuất tài liệu, cách ly ĐÚNG theo khách hàng của CHÍNH request đó.
4. **Tác tử + cache/chi phí** (bài `4`) — mỗi request phục vụ xong ghi MỘT
   sự kiện hoàn thành; tỉ lệ cache hit được đo trên TẬP câu hỏi của ĐÚNG
   những request đã được phục vụ; nếu tỉ lệ đó thấp bất thường, tra cứu
   ngay nguyên nhân.

```
xu_ly_toan_bo_he_thong(danh_sach_request, danh_sach_tai_node, chi_phi_khoi_dong,
                        kich_thuoc_lo, danh_sach_tai_lieu, danh_sach_vector,
                        so_ung_vien, k_cuoi, cache, nguong_cosine, nguong_ty_le_hit_thap):
  (ket_qua_cong, ..., thong_luong_tuan_tu, thong_luong_gop_lo) =
      do_thong_luong_hang_doi_da_qua_cong(danh_sach_request, danh_sach_tai_node,
                                           chi_phi_khoi_dong, kich_thuoc_lo)          # (1)

  so_duoc_phuc_vu = dem ket_qua_cong == "duoc_chap_nhan"                             # (2)
  so_tu_choi_sai_key = dem ket_qua_cong == "tu_choi_sai_key"
  so_tu_choi_vuot_han_muc = dem ket_qua_cong == "tu_choi_vuot_han_muc"

  cho MOI (request, ket_qua) DUOC CHAP NHAN:
    truy_xuat_rag_cho_request(request, danh_sach_tai_lieu, danh_sach_vector,
                               so_ung_vien, k_cuoi)                                    # (3)
    (lich_su_su_kien, ty_le_cache_hit, huong_xu_ly) =
        buoc_tac_tu_va_do_chi_phi(lich_su_su_kien, request["id"],
                                   cau_hoi_vector CUA CAC request duoc phuc vu,
                                   cache, nguong_cosine, nguong_ty_le_hit_thap)        # (4)

  tra ve {so_duoc_phuc_vu, so_tu_choi_sai_key, so_tu_choi_vuot_han_muc,
          ty_le_cache_hit, thong_luong_gop_lo, thong_luong_tuan_tu, huong_xu_ly}
```

Bốn hàm được GỌI LẠI NGUYÊN VĂN — không viết lại logic nào ở đây. Điểm dạy
quan trọng: `danh_sach_cau_hoi_vector` đưa vào `buoc_tac_tu_va_do_chi_phi`
CHỈ chứa câu hỏi của những request ĐÃ ĐƯỢC PHỤC VỤ — một request bị từ chối
không có cơ hội "kéo" tỉ lệ cache hit đi đâu cả, vì nó chưa từng hỏi bất kỳ
điều gì.
::::

::::example{#tai_gia_muoi_request_hon_hop}
```python title=readonly
import math

# ---- q8.6b: cong AI (bai 1) ----
BANG_NGUOI_DUNG = {
    "key-abc123": {"ten": "an", "han_muc": 1000},
    "key-def456": {"ten": "binh", "han_muc": 5000},
    "key-ghi789": {"ten": "chi", "han_muc": 200},
}

SUC_CHUA_CON_LAI = {
    "key-abc123": 1000,
    "key-def456": 5000,
    "key-ghi789": 200,
}

DANH_SACH_NODE = ["gpu-0", "gpu-1", "gpu-2", "gpu-3"]


def xac_thuc(api_key):
    return BANG_NGUOI_DUNG.get(api_key)


def xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau):
    if so_token_yeu_cau <= suc_chua_con_lai:
        return True, suc_chua_con_lai - so_token_yeu_cau
    return False, suc_chua_con_lai


def least_loaded(danh_sach_tai):
    chi_so_nhe_nhat = 0
    for i in range(1, len(danh_sach_tai)):
        if danh_sach_tai[i] < danh_sach_tai[chi_so_nhe_nhat]:
            chi_so_nhe_nhat = i
    return chi_so_nhe_nhat


def xu_ly_mot_request_qua_cong(request, danh_sach_tai_node):
    nguoi_dung = xac_thuc(request["api_key"])
    if nguoi_dung is None:
        return {"ket_qua": "tu_choi_sai_key"}

    suc_chua_con_lai = SUC_CHUA_CON_LAI[request["api_key"]]
    cho_qua, suc_chua_con_lai_moi = xu_ly_yeu_cau(suc_chua_con_lai, request["so_token"])
    if not cho_qua:
        return {"ket_qua": "tu_choi_vuot_han_muc"}
    SUC_CHUA_CON_LAI[request["api_key"]] = suc_chua_con_lai_moi

    chi_so_node = least_loaded(danh_sach_tai_node)
    danh_sach_tai_node[chi_so_node] += 1

    return {"ket_qua": "duoc_chap_nhan", "node": DANH_SACH_NODE[chi_so_node]}


# ---- q8.6a: hang doi gop lo (bai 2) ----
def tong_thoi_gian_tuan_tu(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong):
    tong = 0
    for thoi_gian_tinh in danh_sach_thoi_gian_tinh:
        tong += chi_phi_khoi_dong + thoi_gian_tinh
    return tong


def tong_thoi_gian_gop_lo(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong, kich_thuoc_lo):
    tong = 0
    for i in range(0, len(danh_sach_thoi_gian_tinh), kich_thuoc_lo):
        lo = danh_sach_thoi_gian_tinh[i:i + kich_thuoc_lo]
        tong += chi_phi_khoi_dong + max(lo)
    return tong


def thong_luong(so_request, tong_thoi_gian):
    return so_request / tong_thoi_gian


def do_thong_luong_hang_doi_da_qua_cong(danh_sach_request, danh_sach_tai_node, chi_phi_khoi_dong, kich_thuoc_lo):
    ket_qua_cong = [xu_ly_mot_request_qua_cong(r, danh_sach_tai_node) for r in danh_sach_request]
    don_vi_tinh_duoc_nhan = [
        r["don_vi_tinh"] for r, kq in zip(danh_sach_request, ket_qua_cong) if kq["ket_qua"] == "duoc_chap_nhan"
    ]
    tong_tuan_tu = tong_thoi_gian_tuan_tu(don_vi_tinh_duoc_nhan, chi_phi_khoi_dong)
    tong_gop_lo = tong_thoi_gian_gop_lo(don_vi_tinh_duoc_nhan, chi_phi_khoi_dong, kich_thuoc_lo)
    thong_luong_tuan_tu = thong_luong(len(don_vi_tinh_duoc_nhan), tong_tuan_tu)
    thong_luong_gop_lo = thong_luong(len(don_vi_tinh_duoc_nhan), tong_gop_lo)
    return ket_qua_cong, don_vi_tinh_duoc_nhan, tong_tuan_tu, tong_gop_lo, thong_luong_tuan_tu, thong_luong_gop_lo


# ---- q8.6c: RAG da khach hang (bai 3) ----
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


# ---- q8.6d: tac tu + cache + su co (bai 4) ----
def tim_trong_cache(cau_hoi_vector, cache, nguong_cosine):
    for muc in cache:
        do_tuong_dong = tuong_dong_cosine(cau_hoi_vector, muc["vector"])
        if do_tuong_dong > nguong_cosine:
            return muc["tra_loi"]
    return None


def do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine):
    so_hit = 0
    for v in danh_sach_cau_hoi_vector:
        if tim_trong_cache(v, cache, nguong_cosine) is not None:
            so_hit += 1
    return so_hit / len(danh_sach_cau_hoi_vector)


BANG_SU_CO = {
    "gpu_oom_tai_cao": {"nguyen_nhan": "batch size hoac do dai ngu canh qua lon", "cach_xu_ly": "giam max_model_len, bat paged attention cua vllm"},
    "do_tre_cao_gpu_ranh": {"nguyen_nhan": "request xu ly tuan tu, khong gop lo", "cach_xu_ly": "bat continuous batching"},
    "chi_phi_tang_bat_thuong": {"nguyen_nhan": "khong co cache, prompt lap lai nhieu lan", "cach_xu_ly": "them semantic cache va prompt caching"},
    "rag_lech_chu_de": {"nguyen_nhan": "chunking kem hoac thieu buoc rerank", "cach_xu_ly": "giam kich thuoc chunk, them buoc rerank"},
    "timeout_gateway": {"nguyen_nhan": "request llm dai hon timeout mac dinh", "cach_xu_ly": "tang timeout va chuyen sang streaming"},
}


def tra_cuu_su_co(trieu_chung):
    return BANG_SU_CO.get(trieu_chung, {"nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them"})


def buoc_tac_tu_va_do_chi_phi(lich_su_su_kien, id_request, danh_sach_cau_hoi_vector, cache, nguong_cosine, nguong_ty_le_hit_thap):
    lich_su_su_kien.append({"loai_su_kien": "HoanThanhRequestEvent", "request_id": id_request})
    ty_le_hit = do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine)
    if ty_le_hit < nguong_ty_le_hit_thap:
        huong_xu_ly = tra_cuu_su_co("chi_phi_tang_bat_thuong")
    else:
        huong_xu_ly = None
    return lich_su_su_kien, ty_le_hit, huong_xu_ly


# ---- BOSS: rap ca bon buoc ----
def xu_ly_toan_bo_he_thong(danh_sach_request, danh_sach_tai_node, chi_phi_khoi_dong, kich_thuoc_lo,
                           danh_sach_tai_lieu, danh_sach_vector, so_ung_vien, k_cuoi,
                           cache, nguong_cosine, nguong_ty_le_hit_thap):
    (ket_qua_cong, don_vi_tinh_duoc_nhan, tong_tuan_tu, tong_gop_lo,
     thong_luong_tuan_tu, thong_luong_gop_lo) = do_thong_luong_hang_doi_da_qua_cong(
        danh_sach_request, danh_sach_tai_node, chi_phi_khoi_dong, kich_thuoc_lo
    )

    so_duoc_phuc_vu = sum(1 for kq in ket_qua_cong if kq["ket_qua"] == "duoc_chap_nhan")
    so_tu_choi_sai_key = sum(1 for kq in ket_qua_cong if kq["ket_qua"] == "tu_choi_sai_key")
    so_tu_choi_vuot_han_muc = sum(1 for kq in ket_qua_cong if kq["ket_qua"] == "tu_choi_vuot_han_muc")

    danh_sach_cau_hoi_vector_duoc_phuc_vu = [
        r["cau_hoi_vector"] for r, kq in zip(danh_sach_request, ket_qua_cong) if kq["ket_qua"] == "duoc_chap_nhan"
    ]

    lich_su_su_kien = []
    ty_le_cache_hit = None
    huong_xu_ly = None
    for request, ket_qua in zip(danh_sach_request, ket_qua_cong):
        if ket_qua["ket_qua"] != "duoc_chap_nhan":
            continue
        truy_xuat_rag_cho_request(request, danh_sach_tai_lieu, danh_sach_vector, so_ung_vien, k_cuoi)
        lich_su_su_kien, ty_le_cache_hit, huong_xu_ly = buoc_tac_tu_va_do_chi_phi(
            lich_su_su_kien, request["id"], danh_sach_cau_hoi_vector_duoc_phuc_vu, cache, nguong_cosine, nguong_ty_le_hit_thap
        )

    return {
        "so_duoc_phuc_vu": so_duoc_phuc_vu,
        "so_tu_choi_sai_key": so_tu_choi_sai_key,
        "so_tu_choi_vuot_han_muc": so_tu_choi_vuot_han_muc,
        "ty_le_cache_hit": ty_le_cache_hit,
        "thong_luong_gop_lo": thong_luong_gop_lo,
        "thong_luong_tuan_tu": thong_luong_tuan_tu,
        "huong_xu_ly": huong_xu_ly,
    }


# ---- du lieu tai gia hon hop ----
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

CACHE_BOSS = [
    {"cau_hoi": "bao hanh san pham a la bao lau", "vector": [1.0, 0.0, 0.0], "tra_loi": "Bao hanh 12 thang"},
    {"cau_hoi": "gia san pham b bao nhieu", "vector": [0.0, 1.0, 0.0], "tra_loi": "Gia lien he dai ly"},
]

DAY_REQUEST = [
    {"id": 301, "api_key": "key-unknown-1", "so_token": 10},
    {"id": 201, "api_key": "key-abc123", "so_token": 100, "don_vi_tinh": 10,
     "khach_hang": "A", "cau_hoi": "bao hanh san pham nay la bao lau", "cau_hoi_vector": [0.99, 0.05, 0.0]},
    {"id": 303, "api_key": "key-ghi789", "so_token": 250},
    {"id": 202, "api_key": "key-def456", "so_token": 150, "don_vi_tinh": 12,
     "khach_hang": "B", "cau_hoi": "gia san pham nay bao nhieu tien", "cau_hoi_vector": [0.02, 0.98, 0.0]},
    {"id": 203, "api_key": "key-abc123", "so_token": 120, "don_vi_tinh": 18,
     "khach_hang": "A", "cau_hoi": "ket noi mang co on dinh khong", "cau_hoi_vector": [0.3, 0.3, 0.3]},
    {"id": 302, "api_key": "", "so_token": 5},
    {"id": 204, "api_key": "key-def456", "so_token": 180, "don_vi_tinh": 20,
     "khach_hang": "B", "cau_hoi": "mau sac san pham co gi", "cau_hoi_vector": [0.0, 0.02, 0.99]},
    {"id": 304, "api_key": "key-ghi789", "so_token": 300},
    {"id": 205, "api_key": "key-abc123", "so_token": 90, "don_vi_tinh": 14,
     "khach_hang": "A", "cau_hoi": "bao hanh san pham nay la bao lau", "cau_hoi_vector": [0.99, 0.05, 0.0]},
    {"id": 206, "api_key": "key-def456", "so_token": 130, "don_vi_tinh": 16,
     "khach_hang": "B", "cau_hoi": "bao hanh san pham nay la bao lau", "cau_hoi_vector": [1.0, 0.0, 0.0]},
]

DANH_SACH_TAI_NODE_BOSS = [0, 0, 0, 0]
CHI_PHI_KHOI_DONG_BOSS = 200
KICH_THUOC_LO_BOSS = 3
SO_UNG_VIEN_BOSS = 2
K_CUOI_BOSS = 1
NGUONG_COSINE_BOSS = 0.95
NGUONG_TY_LE_HIT_THAP_BOSS = 0.7

BAO_CAO = xu_ly_toan_bo_he_thong(
    DAY_REQUEST, DANH_SACH_TAI_NODE_BOSS, CHI_PHI_KHOI_DONG_BOSS, KICH_THUOC_LO_BOSS,
    TAI_LIEU_RAG, VECTOR_RAG, SO_UNG_VIEN_BOSS, K_CUOI_BOSS,
    CACHE_BOSS, NGUONG_COSINE_BOSS, NGUONG_TY_LE_HIT_THAP_BOSS,
)

print(BAO_CAO)
```

```text title=readonly
{'so_duoc_phuc_vu': 6, 'so_tu_choi_sai_key': 2, 'so_tu_choi_vuot_han_muc': 2, 'ty_le_cache_hit': 0.6666666666666666, 'thong_luong_gop_lo': 0.0136986301369863, 'thong_luong_tuan_tu': 0.004651162790697674, 'huong_xu_ly': {'nguyen_nhan': 'khong co cache, prompt lap lai nhieu lan', 'cach_xu_ly': 'them semantic cache va prompt caching'}}
```

`10` request hỗn hợp: `2` sai key (`301`, `302`), `2` vượt hạn mức (`303`
cần `250` nhưng `key-ghi789` chỉ có `200`; `304` cần `300`), `6` được chấp
nhận (`201-206`, dùng `key-abc123`/`key-def456`, đều đủ hạn mức). Bước `(1)`
đo thông lượng trên `6` request đó, `don_vi_tinh = [10, 12, 18, 20, 14,
16]`, `kich_thuoc_lo=3` — `2` lô `[10,12,18]` VÀ `[20,14,16]` — tuần tự
`1290`, gộp lô `438` — `thong_luong_gop_lo ≈ 0,013699` CAO HƠN
`thong_luong_tuan_tu ≈ 0,004651`. Bước `(3)`+`(4)`: MỖI trong `6` request đó
được truy xuất RAG (cách ly ĐÚNG theo `khach_hang`), rồi ghi sự kiện VÀ đo
cache hit trên CHÍNH `6` câu hỏi đó — request `205` HỎI LẠI đúng câu của
`201` (`"bao hanh san pham nay la bao lau"`, CÙNG `cau_hoi_vector`) — MỘT
trong `4` lần hit (`201, 202, 205, 206` khớp `CACHE_BOSS`; `203, 204`
không) — `ty_le_cache_hit = 4/6 ≈ 0,6667`, THẤP HƠN ngưỡng `0,7` —
`huong_xu_ly` LÀ kết quả THẬT của `tra_cuu_su_co("chi_phi_tang_bat_thuong")`.
::::

::::predict{#doan_doi_mot_cau_hoi_thanh_hit commitOnce}
Xét đúng `DAY_REQUEST` ở ví dụ trên. Request `203` (`khach_hang="A"`, hỏi về
kết nối mạng) có `cau_hoi_vector = [0.3, 0.3, 0.3]` — KHÔNG khớp mục nào của
`CACHE_BOSS` (cosine `≈ 0,577` với cả hai mục, dưới ngưỡng `0,95`) — MỘT
trong hai câu MISS của `ty_le_cache_hit = 4/6`.

**Trước khi chạy thử**, bạn đoán: nếu đổi `cau_hoi_vector` của request
`203` từ `[0.3, 0.3, 0.3]` THÀNH `[1.0, 0.0, 0.0]` (khớp tuyệt đối mục ĐẦU
của `CACHE_BOSS`) — giữ nguyên MỌI thứ khác — `huong_xu_ly` trong `BAO_CAO`
MỚI LÀ gì?

:::opt{correct}
`None` — `5` trên `6` câu hỏi giờ trùng cache (`ty_le_cache_hit = 5/6 ≈
0,8333`), VƯỢT ngưỡng `nguong_ty_le_hit_thap = 0,7`, nên nhánh "bất thường"
không kích hoạt nữa
:::

:::opt
Vẫn LÀ dict cũ (`"chi_phi_tang_bat_thuong"`) — vì `huong_xu_ly` chỉ được
tính MỘT LẦN, dựa trên cấu hình HỆ THỐNG, không phụ thuộc câu hỏi CỤ THỂ
nào của request
::why
Gần đúng ở việc `nguong_ty_le_hit_thap` THẬT SỰ LÀ một tham số CẤU HÌNH cố
định (`0,7`), không đổi theo từng request — quan sát đó đúng.

Chỗ lệch: `huong_xu_ly` không chỉ phụ thuộc NGƯỠNG — nó phụ thuộc
`ty_le_cache_hit`, và `ty_le_cache_hit` được TÍNH LẠI từ TOÀN BỘ tập
`cau_hoi_vector` của các request đã phục vụ. Đổi MỘT `cau_hoi_vector` (dù
chỉ MỘT trong `6`) đổi TRỰC TIẾP giá trị đó, và ngưỡng so sánh nó với vẫn
giữ nguyên — kết luận cuối cùng ("bất thường" hay không) có thể đổi theo.
::
:::

:::opt
Một dict KHÁC — `tra_cuu_su_co("rag_lech_chu_de")` — vì đổi `cau_hoi_vector`
ảnh hưởng tới bước RAG, nên triệu chứng tra cứu cũng phải đổi theo
::why
Gần đúng ở việc đổi `cau_hoi_vector` THẬT SỰ ảnh hưởng bước RAG của CHÍNH
request `203` (nó sẽ tìm ứng viên khác trong số tài liệu của `A`) — quan
sát về ẢNH HƯỞNG cục bộ đó đúng.

Chỗ lệch: `xu_ly_toan_bo_he_thong` LUÔN gọi `tra_cuu_su_co` với ĐÚNG MỘT
triệu chứng cố định — `"chi_phi_tang_bat_thuong"` — không hề đổi triệu
chứng theo NGUYÊN NHÂN thật của bất kỳ request nào. Với dữ liệu mới,
`ty_le_cache_hit` KHÔNG còn thấp hơn ngưỡng nữa, nên nhánh gọi
`tra_cuu_su_co` không chạy — `huong_xu_ly` LÀ `None`, không phải MỘT dict
khác.
::
:::
::::

::::code{#viet_xu_ly_toan_bo_he_thong}
Hoàn thiện `xu_ly_toan_bo_he_thong`: với MỖI request `duoc_chap_nhan`, gọi
truy xuất RAG (bài `3`) rồi gọi bước tác tử + đo chi phí (bài `4`).

```python title=starter
import math

# ---- q8.6b: cong AI (bai 1) ----
BANG_NGUOI_DUNG = {
    "key-abc123": {"ten": "an", "han_muc": 1000},
    "key-def456": {"ten": "binh", "han_muc": 5000},
    "key-ghi789": {"ten": "chi", "han_muc": 200},
}

SUC_CHUA_CON_LAI = {
    "key-abc123": 1000,
    "key-def456": 5000,
    "key-ghi789": 200,
}

DANH_SACH_NODE = ["gpu-0", "gpu-1", "gpu-2", "gpu-3"]


def xac_thuc(api_key):
    return BANG_NGUOI_DUNG.get(api_key)


def xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau):
    if so_token_yeu_cau <= suc_chua_con_lai:
        return True, suc_chua_con_lai - so_token_yeu_cau
    return False, suc_chua_con_lai


def least_loaded(danh_sach_tai):
    chi_so_nhe_nhat = 0
    for i in range(1, len(danh_sach_tai)):
        if danh_sach_tai[i] < danh_sach_tai[chi_so_nhe_nhat]:
            chi_so_nhe_nhat = i
    return chi_so_nhe_nhat


def xu_ly_mot_request_qua_cong(request, danh_sach_tai_node):
    nguoi_dung = xac_thuc(request["api_key"])
    if nguoi_dung is None:
        return {"ket_qua": "tu_choi_sai_key"}

    suc_chua_con_lai = SUC_CHUA_CON_LAI[request["api_key"]]
    cho_qua, suc_chua_con_lai_moi = xu_ly_yeu_cau(suc_chua_con_lai, request["so_token"])
    if not cho_qua:
        return {"ket_qua": "tu_choi_vuot_han_muc"}
    SUC_CHUA_CON_LAI[request["api_key"]] = suc_chua_con_lai_moi

    chi_so_node = least_loaded(danh_sach_tai_node)
    danh_sach_tai_node[chi_so_node] += 1

    return {"ket_qua": "duoc_chap_nhan", "node": DANH_SACH_NODE[chi_so_node]}


# ---- q8.6a: hang doi gop lo (bai 2) ----
def tong_thoi_gian_tuan_tu(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong):
    tong = 0
    for thoi_gian_tinh in danh_sach_thoi_gian_tinh:
        tong += chi_phi_khoi_dong + thoi_gian_tinh
    return tong


def tong_thoi_gian_gop_lo(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong, kich_thuoc_lo):
    tong = 0
    for i in range(0, len(danh_sach_thoi_gian_tinh), kich_thuoc_lo):
        lo = danh_sach_thoi_gian_tinh[i:i + kich_thuoc_lo]
        tong += chi_phi_khoi_dong + max(lo)
    return tong


def thong_luong(so_request, tong_thoi_gian):
    return so_request / tong_thoi_gian


def do_thong_luong_hang_doi_da_qua_cong(danh_sach_request, danh_sach_tai_node, chi_phi_khoi_dong, kich_thuoc_lo):
    ket_qua_cong = [xu_ly_mot_request_qua_cong(r, danh_sach_tai_node) for r in danh_sach_request]
    don_vi_tinh_duoc_nhan = [
        r["don_vi_tinh"] for r, kq in zip(danh_sach_request, ket_qua_cong) if kq["ket_qua"] == "duoc_chap_nhan"
    ]
    tong_tuan_tu = tong_thoi_gian_tuan_tu(don_vi_tinh_duoc_nhan, chi_phi_khoi_dong)
    tong_gop_lo = tong_thoi_gian_gop_lo(don_vi_tinh_duoc_nhan, chi_phi_khoi_dong, kich_thuoc_lo)
    thong_luong_tuan_tu = thong_luong(len(don_vi_tinh_duoc_nhan), tong_tuan_tu)
    thong_luong_gop_lo = thong_luong(len(don_vi_tinh_duoc_nhan), tong_gop_lo)
    return ket_qua_cong, don_vi_tinh_duoc_nhan, tong_tuan_tu, tong_gop_lo, thong_luong_tuan_tu, thong_luong_gop_lo


# ---- q8.6c: RAG da khach hang (bai 3) ----
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


# ---- q8.6d: tac tu + cache + su co (bai 4) ----
def tim_trong_cache(cau_hoi_vector, cache, nguong_cosine):
    for muc in cache:
        do_tuong_dong = tuong_dong_cosine(cau_hoi_vector, muc["vector"])
        if do_tuong_dong > nguong_cosine:
            return muc["tra_loi"]
    return None


def do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine):
    so_hit = 0
    for v in danh_sach_cau_hoi_vector:
        if tim_trong_cache(v, cache, nguong_cosine) is not None:
            so_hit += 1
    return so_hit / len(danh_sach_cau_hoi_vector)


BANG_SU_CO = {
    "gpu_oom_tai_cao": {"nguyen_nhan": "batch size hoac do dai ngu canh qua lon", "cach_xu_ly": "giam max_model_len, bat paged attention cua vllm"},
    "do_tre_cao_gpu_ranh": {"nguyen_nhan": "request xu ly tuan tu, khong gop lo", "cach_xu_ly": "bat continuous batching"},
    "chi_phi_tang_bat_thuong": {"nguyen_nhan": "khong co cache, prompt lap lai nhieu lan", "cach_xu_ly": "them semantic cache va prompt caching"},
    "rag_lech_chu_de": {"nguyen_nhan": "chunking kem hoac thieu buoc rerank", "cach_xu_ly": "giam kich thuoc chunk, them buoc rerank"},
    "timeout_gateway": {"nguyen_nhan": "request llm dai hon timeout mac dinh", "cach_xu_ly": "tang timeout va chuyen sang streaming"},
}


def tra_cuu_su_co(trieu_chung):
    return BANG_SU_CO.get(trieu_chung, {"nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them"})


def buoc_tac_tu_va_do_chi_phi(lich_su_su_kien, id_request, danh_sach_cau_hoi_vector, cache, nguong_cosine, nguong_ty_le_hit_thap):
    lich_su_su_kien.append({"loai_su_kien": "HoanThanhRequestEvent", "request_id": id_request})
    ty_le_hit = do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine)
    if ty_le_hit < nguong_ty_le_hit_thap:
        huong_xu_ly = tra_cuu_su_co("chi_phi_tang_bat_thuong")
    else:
        huong_xu_ly = None
    return lich_su_su_kien, ty_le_hit, huong_xu_ly


# ---- BOSS: rap ca bon buoc ----
def xu_ly_toan_bo_he_thong(danh_sach_request, danh_sach_tai_node, chi_phi_khoi_dong, kich_thuoc_lo,
                           danh_sach_tai_lieu, danh_sach_vector, so_ung_vien, k_cuoi,
                           cache, nguong_cosine, nguong_ty_le_hit_thap):
    (ket_qua_cong, don_vi_tinh_duoc_nhan, tong_tuan_tu, tong_gop_lo,
     thong_luong_tuan_tu, thong_luong_gop_lo) = do_thong_luong_hang_doi_da_qua_cong(
        danh_sach_request, danh_sach_tai_node, chi_phi_khoi_dong, kich_thuoc_lo
    )

    so_duoc_phuc_vu = sum(1 for kq in ket_qua_cong if kq["ket_qua"] == "duoc_chap_nhan")
    so_tu_choi_sai_key = sum(1 for kq in ket_qua_cong if kq["ket_qua"] == "tu_choi_sai_key")
    so_tu_choi_vuot_han_muc = sum(1 for kq in ket_qua_cong if kq["ket_qua"] == "tu_choi_vuot_han_muc")

    danh_sach_cau_hoi_vector_duoc_phuc_vu = [
        r["cau_hoi_vector"] for r, kq in zip(danh_sach_request, ket_qua_cong) if kq["ket_qua"] == "duoc_chap_nhan"
    ]

    lich_su_su_kien = []
    ty_le_cache_hit = None
    huong_xu_ly = None
    for request, ket_qua in zip(danh_sach_request, ket_qua_cong):
        if ket_qua["ket_qua"] != "duoc_chap_nhan":
            continue
        ___                                                    # truy_xuat_rag_cho_request(request, danh_sach_tai_lieu, danh_sach_vector, so_ung_vien, k_cuoi)
        lich_su_su_kien, ty_le_cache_hit, huong_xu_ly = ___     # buoc_tac_tu_va_do_chi_phi(lich_su_su_kien, request["id"], danh_sach_cau_hoi_vector_duoc_phuc_vu, cache, nguong_cosine, nguong_ty_le_hit_thap)

    return {
        "so_duoc_phuc_vu": so_duoc_phuc_vu,
        "so_tu_choi_sai_key": so_tu_choi_sai_key,
        "so_tu_choi_vuot_han_muc": so_tu_choi_vuot_han_muc,
        "ty_le_cache_hit": ty_le_cache_hit,
        "thong_luong_gop_lo": thong_luong_gop_lo,
        "thong_luong_tuan_tu": thong_luong_tuan_tu,
        "huong_xu_ly": huong_xu_ly,
    }


# ---- du lieu tai gia hon hop ----
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

CACHE_BOSS = [
    {"cau_hoi": "bao hanh san pham a la bao lau", "vector": [1.0, 0.0, 0.0], "tra_loi": "Bao hanh 12 thang"},
    {"cau_hoi": "gia san pham b bao nhieu", "vector": [0.0, 1.0, 0.0], "tra_loi": "Gia lien he dai ly"},
]

DAY_REQUEST = [
    {"id": 301, "api_key": "key-unknown-1", "so_token": 10},
    {"id": 201, "api_key": "key-abc123", "so_token": 100, "don_vi_tinh": 10,
     "khach_hang": "A", "cau_hoi": "bao hanh san pham nay la bao lau", "cau_hoi_vector": [0.99, 0.05, 0.0]},
    {"id": 303, "api_key": "key-ghi789", "so_token": 250},
    {"id": 202, "api_key": "key-def456", "so_token": 150, "don_vi_tinh": 12,
     "khach_hang": "B", "cau_hoi": "gia san pham nay bao nhieu tien", "cau_hoi_vector": [0.02, 0.98, 0.0]},
    {"id": 203, "api_key": "key-abc123", "so_token": 120, "don_vi_tinh": 18,
     "khach_hang": "A", "cau_hoi": "ket noi mang co on dinh khong", "cau_hoi_vector": [0.3, 0.3, 0.3]},
    {"id": 302, "api_key": "", "so_token": 5},
    {"id": 204, "api_key": "key-def456", "so_token": 180, "don_vi_tinh": 20,
     "khach_hang": "B", "cau_hoi": "mau sac san pham co gi", "cau_hoi_vector": [0.0, 0.02, 0.99]},
    {"id": 304, "api_key": "key-ghi789", "so_token": 300},
    {"id": 205, "api_key": "key-abc123", "so_token": 90, "don_vi_tinh": 14,
     "khach_hang": "A", "cau_hoi": "bao hanh san pham nay la bao lau", "cau_hoi_vector": [0.99, 0.05, 0.0]},
    {"id": 206, "api_key": "key-def456", "so_token": 130, "don_vi_tinh": 16,
     "khach_hang": "B", "cau_hoi": "bao hanh san pham nay la bao lau", "cau_hoi_vector": [1.0, 0.0, 0.0]},
]

DANH_SACH_TAI_NODE_BOSS = [0, 0, 0, 0]
CHI_PHI_KHOI_DONG_BOSS = 200
KICH_THUOC_LO_BOSS = 3
SO_UNG_VIEN_BOSS = 2
K_CUOI_BOSS = 1
NGUONG_COSINE_BOSS = 0.95
NGUONG_TY_LE_HIT_THAP_BOSS = 0.7

BAO_CAO = xu_ly_toan_bo_he_thong(
    DAY_REQUEST, DANH_SACH_TAI_NODE_BOSS, CHI_PHI_KHOI_DONG_BOSS, KICH_THUOC_LO_BOSS,
    TAI_LIEU_RAG, VECTOR_RAG, SO_UNG_VIEN_BOSS, K_CUOI_BOSS,
    CACHE_BOSS, NGUONG_COSINE_BOSS, NGUONG_TY_LE_HIT_THAP_BOSS,
)

print(BAO_CAO)
```

```python title=solution
import math

# ---- q8.6b: cong AI (bai 1) ----
BANG_NGUOI_DUNG = {
    "key-abc123": {"ten": "an", "han_muc": 1000},
    "key-def456": {"ten": "binh", "han_muc": 5000},
    "key-ghi789": {"ten": "chi", "han_muc": 200},
}

SUC_CHUA_CON_LAI = {
    "key-abc123": 1000,
    "key-def456": 5000,
    "key-ghi789": 200,
}

DANH_SACH_NODE = ["gpu-0", "gpu-1", "gpu-2", "gpu-3"]


def xac_thuc(api_key):
    return BANG_NGUOI_DUNG.get(api_key)


def xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau):
    if so_token_yeu_cau <= suc_chua_con_lai:
        return True, suc_chua_con_lai - so_token_yeu_cau
    return False, suc_chua_con_lai


def least_loaded(danh_sach_tai):
    chi_so_nhe_nhat = 0
    for i in range(1, len(danh_sach_tai)):
        if danh_sach_tai[i] < danh_sach_tai[chi_so_nhe_nhat]:
            chi_so_nhe_nhat = i
    return chi_so_nhe_nhat


def xu_ly_mot_request_qua_cong(request, danh_sach_tai_node):
    nguoi_dung = xac_thuc(request["api_key"])
    if nguoi_dung is None:
        return {"ket_qua": "tu_choi_sai_key"}

    suc_chua_con_lai = SUC_CHUA_CON_LAI[request["api_key"]]
    cho_qua, suc_chua_con_lai_moi = xu_ly_yeu_cau(suc_chua_con_lai, request["so_token"])
    if not cho_qua:
        return {"ket_qua": "tu_choi_vuot_han_muc"}
    SUC_CHUA_CON_LAI[request["api_key"]] = suc_chua_con_lai_moi

    chi_so_node = least_loaded(danh_sach_tai_node)
    danh_sach_tai_node[chi_so_node] += 1

    return {"ket_qua": "duoc_chap_nhan", "node": DANH_SACH_NODE[chi_so_node]}


# ---- q8.6a: hang doi gop lo (bai 2) ----
def tong_thoi_gian_tuan_tu(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong):
    tong = 0
    for thoi_gian_tinh in danh_sach_thoi_gian_tinh:
        tong += chi_phi_khoi_dong + thoi_gian_tinh
    return tong


def tong_thoi_gian_gop_lo(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong, kich_thuoc_lo):
    tong = 0
    for i in range(0, len(danh_sach_thoi_gian_tinh), kich_thuoc_lo):
        lo = danh_sach_thoi_gian_tinh[i:i + kich_thuoc_lo]
        tong += chi_phi_khoi_dong + max(lo)
    return tong


def thong_luong(so_request, tong_thoi_gian):
    return so_request / tong_thoi_gian


def do_thong_luong_hang_doi_da_qua_cong(danh_sach_request, danh_sach_tai_node, chi_phi_khoi_dong, kich_thuoc_lo):
    ket_qua_cong = [xu_ly_mot_request_qua_cong(r, danh_sach_tai_node) for r in danh_sach_request]
    don_vi_tinh_duoc_nhan = [
        r["don_vi_tinh"] for r, kq in zip(danh_sach_request, ket_qua_cong) if kq["ket_qua"] == "duoc_chap_nhan"
    ]
    tong_tuan_tu = tong_thoi_gian_tuan_tu(don_vi_tinh_duoc_nhan, chi_phi_khoi_dong)
    tong_gop_lo = tong_thoi_gian_gop_lo(don_vi_tinh_duoc_nhan, chi_phi_khoi_dong, kich_thuoc_lo)
    thong_luong_tuan_tu = thong_luong(len(don_vi_tinh_duoc_nhan), tong_tuan_tu)
    thong_luong_gop_lo = thong_luong(len(don_vi_tinh_duoc_nhan), tong_gop_lo)
    return ket_qua_cong, don_vi_tinh_duoc_nhan, tong_tuan_tu, tong_gop_lo, thong_luong_tuan_tu, thong_luong_gop_lo


# ---- q8.6c: RAG da khach hang (bai 3) ----
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


# ---- q8.6d: tac tu + cache + su co (bai 4) ----
def tim_trong_cache(cau_hoi_vector, cache, nguong_cosine):
    for muc in cache:
        do_tuong_dong = tuong_dong_cosine(cau_hoi_vector, muc["vector"])
        if do_tuong_dong > nguong_cosine:
            return muc["tra_loi"]
    return None


def do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine):
    so_hit = 0
    for v in danh_sach_cau_hoi_vector:
        if tim_trong_cache(v, cache, nguong_cosine) is not None:
            so_hit += 1
    return so_hit / len(danh_sach_cau_hoi_vector)


BANG_SU_CO = {
    "gpu_oom_tai_cao": {"nguyen_nhan": "batch size hoac do dai ngu canh qua lon", "cach_xu_ly": "giam max_model_len, bat paged attention cua vllm"},
    "do_tre_cao_gpu_ranh": {"nguyen_nhan": "request xu ly tuan tu, khong gop lo", "cach_xu_ly": "bat continuous batching"},
    "chi_phi_tang_bat_thuong": {"nguyen_nhan": "khong co cache, prompt lap lai nhieu lan", "cach_xu_ly": "them semantic cache va prompt caching"},
    "rag_lech_chu_de": {"nguyen_nhan": "chunking kem hoac thieu buoc rerank", "cach_xu_ly": "giam kich thuoc chunk, them buoc rerank"},
    "timeout_gateway": {"nguyen_nhan": "request llm dai hon timeout mac dinh", "cach_xu_ly": "tang timeout va chuyen sang streaming"},
}


def tra_cuu_su_co(trieu_chung):
    return BANG_SU_CO.get(trieu_chung, {"nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them"})


def buoc_tac_tu_va_do_chi_phi(lich_su_su_kien, id_request, danh_sach_cau_hoi_vector, cache, nguong_cosine, nguong_ty_le_hit_thap):
    lich_su_su_kien.append({"loai_su_kien": "HoanThanhRequestEvent", "request_id": id_request})
    ty_le_hit = do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine)
    if ty_le_hit < nguong_ty_le_hit_thap:
        huong_xu_ly = tra_cuu_su_co("chi_phi_tang_bat_thuong")
    else:
        huong_xu_ly = None
    return lich_su_su_kien, ty_le_hit, huong_xu_ly


# ---- BOSS: rap ca bon buoc ----
def xu_ly_toan_bo_he_thong(danh_sach_request, danh_sach_tai_node, chi_phi_khoi_dong, kich_thuoc_lo,
                           danh_sach_tai_lieu, danh_sach_vector, so_ung_vien, k_cuoi,
                           cache, nguong_cosine, nguong_ty_le_hit_thap):
    (ket_qua_cong, don_vi_tinh_duoc_nhan, tong_tuan_tu, tong_gop_lo,
     thong_luong_tuan_tu, thong_luong_gop_lo) = do_thong_luong_hang_doi_da_qua_cong(
        danh_sach_request, danh_sach_tai_node, chi_phi_khoi_dong, kich_thuoc_lo
    )

    so_duoc_phuc_vu = sum(1 for kq in ket_qua_cong if kq["ket_qua"] == "duoc_chap_nhan")
    so_tu_choi_sai_key = sum(1 for kq in ket_qua_cong if kq["ket_qua"] == "tu_choi_sai_key")
    so_tu_choi_vuot_han_muc = sum(1 for kq in ket_qua_cong if kq["ket_qua"] == "tu_choi_vuot_han_muc")

    danh_sach_cau_hoi_vector_duoc_phuc_vu = [
        r["cau_hoi_vector"] for r, kq in zip(danh_sach_request, ket_qua_cong) if kq["ket_qua"] == "duoc_chap_nhan"
    ]

    lich_su_su_kien = []
    ty_le_cache_hit = None
    huong_xu_ly = None
    for request, ket_qua in zip(danh_sach_request, ket_qua_cong):
        if ket_qua["ket_qua"] != "duoc_chap_nhan":
            continue
        truy_xuat_rag_cho_request(request, danh_sach_tai_lieu, danh_sach_vector, so_ung_vien, k_cuoi)
        lich_su_su_kien, ty_le_cache_hit, huong_xu_ly = buoc_tac_tu_va_do_chi_phi(
            lich_su_su_kien, request["id"], danh_sach_cau_hoi_vector_duoc_phuc_vu, cache, nguong_cosine, nguong_ty_le_hit_thap
        )

    return {
        "so_duoc_phuc_vu": so_duoc_phuc_vu,
        "so_tu_choi_sai_key": so_tu_choi_sai_key,
        "so_tu_choi_vuot_han_muc": so_tu_choi_vuot_han_muc,
        "ty_le_cache_hit": ty_le_cache_hit,
        "thong_luong_gop_lo": thong_luong_gop_lo,
        "thong_luong_tuan_tu": thong_luong_tuan_tu,
        "huong_xu_ly": huong_xu_ly,
    }


# ---- du lieu tai gia hon hop ----
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

CACHE_BOSS = [
    {"cau_hoi": "bao hanh san pham a la bao lau", "vector": [1.0, 0.0, 0.0], "tra_loi": "Bao hanh 12 thang"},
    {"cau_hoi": "gia san pham b bao nhieu", "vector": [0.0, 1.0, 0.0], "tra_loi": "Gia lien he dai ly"},
]

DAY_REQUEST = [
    {"id": 301, "api_key": "key-unknown-1", "so_token": 10},
    {"id": 201, "api_key": "key-abc123", "so_token": 100, "don_vi_tinh": 10,
     "khach_hang": "A", "cau_hoi": "bao hanh san pham nay la bao lau", "cau_hoi_vector": [0.99, 0.05, 0.0]},
    {"id": 303, "api_key": "key-ghi789", "so_token": 250},
    {"id": 202, "api_key": "key-def456", "so_token": 150, "don_vi_tinh": 12,
     "khach_hang": "B", "cau_hoi": "gia san pham nay bao nhieu tien", "cau_hoi_vector": [0.02, 0.98, 0.0]},
    {"id": 203, "api_key": "key-abc123", "so_token": 120, "don_vi_tinh": 18,
     "khach_hang": "A", "cau_hoi": "ket noi mang co on dinh khong", "cau_hoi_vector": [0.3, 0.3, 0.3]},
    {"id": 302, "api_key": "", "so_token": 5},
    {"id": 204, "api_key": "key-def456", "so_token": 180, "don_vi_tinh": 20,
     "khach_hang": "B", "cau_hoi": "mau sac san pham co gi", "cau_hoi_vector": [0.0, 0.02, 0.99]},
    {"id": 304, "api_key": "key-ghi789", "so_token": 300},
    {"id": 205, "api_key": "key-abc123", "so_token": 90, "don_vi_tinh": 14,
     "khach_hang": "A", "cau_hoi": "bao hanh san pham nay la bao lau", "cau_hoi_vector": [0.99, 0.05, 0.0]},
    {"id": 206, "api_key": "key-def456", "so_token": 130, "don_vi_tinh": 16,
     "khach_hang": "B", "cau_hoi": "bao hanh san pham nay la bao lau", "cau_hoi_vector": [1.0, 0.0, 0.0]},
]

DANH_SACH_TAI_NODE_BOSS = [0, 0, 0, 0]
CHI_PHI_KHOI_DONG_BOSS = 200
KICH_THUOC_LO_BOSS = 3
SO_UNG_VIEN_BOSS = 2
K_CUOI_BOSS = 1
NGUONG_COSINE_BOSS = 0.95
NGUONG_TY_LE_HIT_THAP_BOSS = 0.7

BAO_CAO = xu_ly_toan_bo_he_thong(
    DAY_REQUEST, DANH_SACH_TAI_NODE_BOSS, CHI_PHI_KHOI_DONG_BOSS, KICH_THUOC_LO_BOSS,
    TAI_LIEU_RAG, VECTOR_RAG, SO_UNG_VIEN_BOSS, K_CUOI_BOSS,
    CACHE_BOSS, NGUONG_COSINE_BOSS, NGUONG_TY_LE_HIT_THAP_BOSS,
)

print(BAO_CAO)
```

```python title=test
assert BAO_CAO == {
    "so_duoc_phuc_vu": 6,
    "so_tu_choi_sai_key": 2,
    "so_tu_choi_vuot_han_muc": 2,
    "ty_le_cache_hit": 4 / 6,
    "thong_luong_gop_lo": 6 / 438,
    "thong_luong_tuan_tu": 6 / 1290,
    "huong_xu_ly": {
        "nguyen_nhan": "khong co cache, prompt lap lai nhieu lan",
        "cach_xu_ly": "them semantic cache va prompt caching",
    },
}, f"BAO_CAO sai -- dang ra {BAO_CAO}"
assert BAO_CAO["so_duoc_phuc_vu"] + BAO_CAO["so_tu_choi_sai_key"] + BAO_CAO["so_tu_choi_vuot_han_muc"] == len(DAY_REQUEST), "tong ba bo dem phai bang DUNG tong so request dau vao"
assert isinstance(BAO_CAO["so_duoc_phuc_vu"], int), "so_duoc_phuc_vu phai la int"
assert BAO_CAO["huong_xu_ly"] is None or isinstance(BAO_CAO["huong_xu_ly"], dict), "huong_xu_ly phai la None hoac dict"

# bien: nguong_ty_le_hit_thap THAT SU quyet dinh co tra cuu hay khong (cung du lieu)
BAO_CAO_NGUONG_THAP = xu_ly_toan_bo_he_thong(
    DAY_REQUEST, [0, 0, 0, 0], CHI_PHI_KHOI_DONG_BOSS, KICH_THUOC_LO_BOSS,
    TAI_LIEU_RAG, VECTOR_RAG, SO_UNG_VIEN_BOSS, K_CUOI_BOSS,
    CACHE_BOSS, NGUONG_COSINE_BOSS, 0.5,
)
assert BAO_CAO_NGUONG_THAP["ty_le_cache_hit"] == 4 / 6, "doi nguong khong duoc doi ty_le_cache_hit"
assert BAO_CAO_NGUONG_THAP["huong_xu_ly"] is None, f"nguong 0.5 (0,6667 khong nho hon 0.5) phai cho huong_xu_ly la None -- dang ra {BAO_CAO_NGUONG_THAP['huong_xu_ly']}"

# bien: kich_thuoc_lo THAT SU rang buoc thong_luong_gop_lo (khong phai tham so thua)
BAO_CAO_LO_6 = xu_ly_toan_bo_he_thong(
    DAY_REQUEST, [0, 0, 0, 0], CHI_PHI_KHOI_DONG_BOSS, 6,
    TAI_LIEU_RAG, VECTOR_RAG, SO_UNG_VIEN_BOSS, K_CUOI_BOSS,
    CACHE_BOSS, NGUONG_COSINE_BOSS, NGUONG_TY_LE_HIT_THAP_BOSS,
)
assert BAO_CAO_LO_6["thong_luong_gop_lo"] == 6 / 220, f"kich_thuoc_lo=6 (mot lo duy nhat) phai cho thong_luong_gop_lo la 6/220 -- dang ra {BAO_CAO_LO_6['thong_luong_gop_lo']}"
assert BAO_CAO_LO_6["thong_luong_gop_lo"] != BAO_CAO["thong_luong_gop_lo"], "doi kich_thuoc_lo phai doi thong_luong_gop_lo"

# assert TRUC TIEP: RAG van cach ly dung tren CHINH du lieu tai gia nay (khong ro ri qua bo dem cua BAO_CAO)
req_a = [r for r in DAY_REQUEST if r["id"] == 201][0]
req_b = [r for r in DAY_REQUEST if r["id"] == 202][0]
ung_a, ket_a = truy_xuat_rag_cho_request(req_a, TAI_LIEU_RAG, VECTOR_RAG, SO_UNG_VIEN_BOSS, K_CUOI_BOSS)
ung_b, ket_b = truy_xuat_rag_cho_request(req_b, TAI_LIEU_RAG, VECTOR_RAG, SO_UNG_VIEN_BOSS, K_CUOI_BOSS)
assert all(TAI_LIEU_RAG[i]["khach_hang"] == "A" for i in ung_a), "request 201 (khach hang A) KHONG DUOC ung vien nao ngoai A"
assert all(TAI_LIEU_RAG[i]["khach_hang"] == "B" for i in ung_b), "request 202 (khach hang B) KHONG DUOC ung vien nao ngoai B"
```

:::hints
- kind: attention
  body: "Hai cho trong, cung mot vong for trong xu_ly_toan_bo_he_thong, o NHANH CHI CHAY khi request duoc_chap_nhan. Cho dau la MOT CAU LENH RIENG (khong gan bien) -- goi lai HAM TRUY XUAT RAG da tai dung tu bai 3. Cho hai la GIA TRI GAN cho bo ba (lich_su_su_kien, ty_le_cache_hit, huong_xu_ly) -- goi lai HAM TAC TU + DO CHI PHI da tai dung tu bai 4, tren danh_sach_cau_hoi_vector_duoc_phuc_vu (KHONG PHAI toan bo danh_sach_request)."
- kind: strategy
  body: "Cho dau: truy_xuat_rag_cho_request(request, danh_sach_tai_lieu, danh_sach_vector, so_ung_vien, k_cuoi) -- goi de MO PHONG buoc truy xuat, khong can dung ket qua. Cho hai: buoc_tac_tu_va_do_chi_phi(lich_su_su_kien, request[\"id\"], danh_sach_cau_hoi_vector_duoc_phuc_vu, cache, nguong_cosine, nguong_ty_le_hit_thap) -- dung DUNG bien danh_sach_cau_hoi_vector_duoc_phuc_vu da tinh TRUOC vong lap, khong phai mot danh sach khac."
- kind: one-line
  body: 'Cho dau la truy_xuat_rag_cho_request(request, danh_sach_tai_lieu, danh_sach_vector, so_ung_vien, k_cuoi), cho hai la buoc_tac_tu_va_do_chi_phi(lich_su_su_kien, request["id"], danh_sach_cau_hoi_vector_duoc_phuc_vu, cache, nguong_cosine, nguong_ty_le_hit_thap).'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT truy_xuat_rag_cho_request(...) (khong duoc bo qua buoc RAG); cho trong hai phai GOI THAT buoc_tac_tu_va_do_chi_phi(...) va GAN vao DUNG bo ba (lich_su_su_kien, ty_le_cache_hit, huong_xu_ly) (khong duoc goi lai truy_xuat_rag_cho_request lan nua hay gan gia tri co dinh)
  requireAst:
  - kind: uses-call, target: truy_xuat_rag_cho_request, min: 1
  - kind: uses-call, target: buoc_tac_tu_va_do_chi_phi, min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, dist build that tai
  # packages/exec-python/dist/kiem-ast.js, trich CHINH XAC tu solution cua
  # file nay -- gom CA BON toolkit tai dung tu bai 1-4) -- xac nhan DUNG
  # CHINH XAC (min VA min+1): truy_xuat_rag_cho_request=1,
  # buoc_tac_tu_va_do_chi_phi=1. CA HAI ham nay CHI duoc goi DUNG 1 lan
  # trong toan bo solution -- dung o hai cho trong, khong noi nao khac
  # (khong co dong demo nao goi rieng le chung ben ngoai vong lap, va ban
  # than hai ham nay khong tu goi lai chinh no).
  # Dien bua "True" vao ca hai cho trong ("True" cho cau lenh rieng, va
  # "lich_su_su_kien, ty_le_cache_hit, huong_xu_ly = True" -- se nem loi
  # unpack o tier run, nhung rieng o tier static thi
  # truy_xuat_rag_cho_request=0 VA buoc_tac_tu_va_do_chi_phi=0) -- CA HAI
  # luat CHAN DUNG (da CHAY THAT xac nhan qua kiemAst).
  #
  # GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho trong --
  # xac dinh ranh gioi TU CHINH khoi starter cua file nay, khong doan tay --
  # va CHAY THAT qua kiemAst() VA python3): dien "buoc_tac_tu_va_do_chi_phi(
  # lich_su_su_kien, request[\"id\"], danh_sach_cau_hoi_vector_duoc_phuc_vu,
  # cache, nguong_cosine, nguong_ty_le_hit_thap)" vao cho trong dau (CAU
  # LENH RIENG, khong gan bien) VA dien
  # "truy_xuat_rag_cho_request(request, danh_sach_tai_lieu, danh_sach_vector,
  # so_ung_vien, k_cuoi)" vao cho trong hai (gan cho bo ba "lich_su_su_kien,
  # ty_le_cache_hit, huong_xu_ly") -- da CHAY THAT qua kiemAst(): CA HAI con
  # so (truy_xuat_rag_cho_request=1, buoc_tac_tu_va_do_chi_phi=1) tren TOAN
  # BO solution DEU KHONG DOI (chi doi VI TRI) -- static KHONG bat duoc
  # mutant nay.
  # Mutant nay BI BAT boi tier 'run': dong hai sau hoan doi
  # ("lich_su_su_kien, ty_le_cache_hit, huong_xu_ly =
  # truy_xuat_rag_cho_request(request, danh_sach_tai_lieu, danh_sach_vector,
  # so_ung_vien, k_cuoi)") co gang GIAI NEN ba bien tu ket qua cua
  # truy_xuat_rag_cho_request -- nhung ham do CHI tra ve MOT CAP hai gia tri
  # (ung_vien, ket_qua_cuoi), khong phai ba -- da tu chay THAT qua python3,
  # xac nhan no nem ValueError ("not enough values to unpack (expected 3,
  # got 2)") NGAY khi xu_ly_toan_bo_he_thong duoc goi tren request
  # duoc_chap_nhan DAU TIEN. Bi chan boi tier 'run', doc lap voi static.
  # Da tu ra soat GOTCHA #6: "truy_xuat_rag_cho_request" va
  # "buoc_tac_tu_va_do_chi_phi" la hai TEN HAM rieng biet, khong trung voi
  # bat ky ham nao khac trong toan bo BON toolkit da tai dung tu bai 1-4
  # (xu_ly_mot_request_qua_cong, do_thong_luong_hang_doi_da_qua_cong,
  # tim_kiem_co_cach_ly, rerank, do_ty_le_cache_hit, tra_cuu_su_co -- khong
  # ten nao trung).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\{'so_duoc_phuc_vu': 6, 'so_tu_choi_sai_key': 2, 'so_tu_choi_vuot_han_muc': 2, 'ty_le_cache_hit': 0\\.6666666666666666, 'thong_luong_gop_lo': 0\\.0136986301369863, 'thong_luong_tuan_tu': 0\\.004651162790697674, 'huong_xu_ly': \\{'nguyen_nhan': 'khong co cache, prompt lap lai nhieu lan', 'cach_xu_ly': 'them semantic cache va prompt caching'\\}\\}\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`6` được phục vụ, `2` sai key, `2` vượt hạn mức, `4` trên `6` câu hỏi trùng
cache, `2,95` lần tăng tốc gộp lô, VÀ một hướng xử lý CỤ THỂ vì chi phí có
dấu hiệu bất thường — MỘT lệnh gọi, BỐN quest, HAI mươi bốn bài trước đó
đều lên tiếng. `T8.6` "Hạ tầng & vận hành AI" đóng tại `26/26`. `R8` "AI ·
Generative AI · RAG" đóng tại `190/190`.
::::

::::reflect{#nghi-lai}
`xu_ly_toan_bo_he_thong` không viết thêm MỘT thuật toán nào — mọi con số nó
trả về đều đến từ bốn hàm đã kiểm chứng riêng lẻ ở bốn bài của `q8.6e`
(chính chúng, đến lượt mình, đều RÁP LẠI những hàm đã kiểm chứng ở `q8.6a`,
`q8.6b`, `q8.6c`, `q8.6d`). Điều nó chứng minh được LÀ: một luồng xử lý
request đầu-cuối — cổng, hàng đợi, RAG, tác tử — không cần MỘT dòng logic
mới nào, CHỈ cần ráp ĐÚNG thứ tự VÀ truyền ĐÚNG dữ liệu giữa các tầng. Đo cụ
thể trên MỘT tải giả `10` request cố ý không tròn trịa: `so_duoc_phuc_vu=6`,
`so_tu_choi_sai_key=2`, `so_tu_choi_vuot_han_muc=2` (đúng ba kết cục của
bài `1`, không hơn không kém); `ty_le_cache_hit = 4/6 ≈ 0,6667` (không phải
`100%` hay `0%` — request `205` hỏi lại đúng câu của `201` là MỘT trong
bốn lần hit đó); `thong_luong_gop_lo/thong_luong_tuan_tu ≈ 2,95` lần (không
phải `10` lần tròn trịa của `q8.6a` bài `5`, vì workload ở đây không đều);
VÀ `huong_xu_ly` LÀ một dict CỤ THỂ, không phải `None` hay một chuỗi đoán
mò — vì `0,6667` THẬT SỰ thấp hơn ngưỡng `0,7` đã đặt ra.

`q8.6e` "Hạ tầng & vận hành AI" đóng tại `5/5`. `T8.6` "Hạ tầng & vận hành
AI" đóng tại `26/26`: bốn quest, mỗi quest một mảnh của `Chương 39` —
`ha-tang-suy-luan`, `ai-gateway`, `rag-quy-mo-lon`,
`da-tac-tu-va-quan-sat-ai` — giờ ráp lại thành MỘT hệ thống chạy được, đo
được bằng số cụ thể, không còn LÀ văn xuôi mô tả một kiến trúc production
xa lạ. VÀ với `q8.6e` khép lại, CẢ REALM `R8` "AI · Generative AI · RAG"
đóng tại `190/190` — từ vector embedding đầu tiên (`T8.1`) tới hệ thống vận
hành đầu-cuối này. `MASTERPLAN` bàn giao sang `R9` "Kỹ nghệ ứng dụng AI"
(`180` bài, TypeScript, `7` track) — lãnh thổ CUỐI CÙNG.
::::

::::checkpoint{mastery=0.9}
::::
