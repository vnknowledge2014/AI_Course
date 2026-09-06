---
id: tri-tue-nhan-tao.da-tac-tu-va-quan-sat-ai.boss-quan-sat-ai-day-du
title: "BOSS quý — vận hành một ngày: sự kiện tác tử + cache + ưu tiên + tra cứu sự cố"
summary: "van_hanh_mot_ngay(...) rap CA BON bai: (a) xu_ly_toan_bo_hang_doi xu ly chuoi su kien tac tu cho MOT task, dem so_su_kien_da_xu_ly; (b) do_ty_le_cache_hit do ty le cache hit tren mot loat cau hoi toi; (c) xu_ly_toan_bo_hai_hang_doi xu ly request qua hang doi uu tien, dem so_mien_phi_da_phuc_vu; (d) NEU ty_le_cache_hit < nguong_ty_le_hit_thap (bat thuong THAP), tra_cuu_su_co('chi_phi_tang_bat_thuong') tra ve huong xu ly, KHAC thi huong_xu_ly la None. Kich ban ngay 1 (cache hit 0,6, nguong 0,7): so_su_kien_da_xu_ly=4, ty_le_cache_hit=0,6, so_mien_phi_da_phuc_vu=2, huong_xu_ly LA dict {nguyen_nhan, cach_xu_ly} cua 'khong co cache...' (0,6 < 0,7 -- bat thuong, TRA CUU). Kich ban ngay 2 (cung du lieu tac tu/uu tien, nhung cau hoi toan trung khop cache -- ty_le=1,0): huong_xu_ly la None (1,0 khong nho hon 0,7 -- KHONG bat thuong). Doi nguong_ty_le_hit_thap tu 0,7 xuong 0,5 tren kich ban ngay 1 (ty_le van la 0,6): huong_xu_ly doi tu dict THANH None (0,6 khong con nho hon 0,5) -- tham so nguong THAT SU quyet dinh co tra cuu hay khong. Dong q8.6d tai 5/5."
locale: vi
track: tri-tue-nhan-tao
module: da-tac-tu-va-quan-sat-ai
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-quan-sat-ai-day-du]
requires: [ai.bang-tra-cuu-su-co]
concepts: [ai.boss-quan-sat-ai-day-du]
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
Bốn bài, bốn mảnh: sự kiện tác tử (bài `1`), semantic cache (bài `2`), hàng
đợi ưu tiên (bài `3`), tra cứu sự cố (bài `4`). BOSS quý không dạy khái
niệm mới — nó RÁP cả bốn thành một hàm duy nhất, mô phỏng "vận hành MỘT
NGÀY" của hệ thống AI. Đóng `q8.6d` tại `5/5`.
::::

::::explain{#rap_bon_manh_thanh_mot_ngay_van_hanh}
Một ngày vận hành thật của hệ thống trong `Chương 39` làm BỐN việc, không
theo thứ tự thời gian cố định nào — nhưng BOSS này mô phỏng chúng theo một
trình tự CỐ ĐỊNH để đo được bằng số:

1. **Sự kiện tác tử** (bài `1`) — một tác vụ đi qua vòng
   Coder→Tester→Coder, đếm `so_su_kien_da_xu_ly`.
2. **Semantic cache** (bài `2`) — một loạt câu hỏi tới, đo
   `ty_le_cache_hit`.
3. **Hàng đợi ưu tiên** (bài `3`) — một loạt request qua hai hàng đợi, đếm
   `so_mien_phi_da_phuc_vu`.
4. **Tra cứu sự cố** (bài `4`) — NẾU `ty_le_cache_hit` THẤP BẤT THƯỜNG
   (dưới một ngưỡng `nguong_ty_le_hit_thap`), tra bảng troubleshooting với
   triệu chứng `"chi_phi_tang_bat_thuong"` (đúng logic §39: không cache,
   chi phí LLM tăng) để biết NGAY hướng xử lý; nếu KHÔNG bất thường,
   `huong_xu_ly` là `None` — không có gì cần tra.

```
van_hanh_mot_ngay(hang_doi_su_kien_tac_tu, gioi_han_vong_lap_tac_tu,
                   danh_sach_cau_hoi_vector, cache, nguong_cosine,
                   hang_doi_tra_phi, hang_doi_mien_phi, gioi_han_uu_tien,
                   nguong_ty_le_hit_thap):
  ket_qua_su_kien, _ = xu_ly_toan_bo_hang_doi(hang_doi_su_kien_tac_tu, gioi_han_vong_lap_tac_tu)   # (a)
  ty_le_hit = do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine)                    # (b)
  thu_tu_phuc_vu, so_mien_phi = xu_ly_toan_bo_hai_hang_doi(hang_doi_tra_phi, hang_doi_mien_phi, gioi_han_uu_tien)  # (c)
  neu ty_le_hit < nguong_ty_le_hit_thap:                                                            # (d)
    huong_xu_ly = tra_cuu_su_co("chi_phi_tang_bat_thuong")
  khac:
    huong_xu_ly = None
  tra ve {so_su_kien_da_xu_ly, ty_le_cache_hit, so_mien_phi_da_phuc_vu, huong_xu_ly}
```

Bốn hàm được GỌI LẠI NGUYÊN VĂN — không viết lại logic nào ở đây. Đây
CHÍNH LÀ điểm dạy: một hệ thống quan sát AI thật không phát minh lại từng
phần — nó RÁP các cơ chế đã kiểm chứng riêng lẻ thành một luồng vận hành,
và bước (d) là "cầu nối" — dùng một CON SỐ đo được từ bước (b) để quyết
định có cần gọi bước (d) hay không.
::::

::::example{#mot_ngay_binh_thuong_va_mot_ngay_bat_thuong}
```python title=readonly
import math


def chay_test_tat_dinh(su_kien):
    return su_kien["ket_qua_tinh"] == su_kien["ket_qua_mong_doi"]


def xu_ly_mot_su_kien(su_kien, hang_doi_ket_qua):
    loai = su_kien["loai_su_kien"]
    if loai == "CodeWrittenEvent":
        if chay_test_tat_dinh(su_kien):
            hang_doi_ket_qua.append({"loai_su_kien": "TestPassedEvent", "task_id": su_kien["task_id"]})
        else:
            hang_doi_ket_qua.append({
                "loai_su_kien": "TestFailedEvent", "task_id": su_kien["task_id"],
                "ket_qua_tinh": su_kien["ket_qua_tinh"], "ket_qua_mong_doi": su_kien["ket_qua_mong_doi"],
            })
    elif loai == "TestFailedEvent":
        hang_doi_ket_qua.append({
            "loai_su_kien": "CodeWrittenEvent", "task_id": su_kien["task_id"],
            "ket_qua_tinh": su_kien["ket_qua_tinh"] + 1, "ket_qua_mong_doi": su_kien["ket_qua_mong_doi"],
        })


def xu_ly_toan_bo_hang_doi(hang_doi_ban_dau, gioi_han_vong_lap):
    hang_doi = list(hang_doi_ban_dau)
    da_xu_ly = []
    so_vong = 0
    while hang_doi and so_vong < gioi_han_vong_lap:
        su_kien = hang_doi.pop(0)
        da_xu_ly.append(su_kien)
        ket_qua_moi = []
        xu_ly_mot_su_kien(su_kien, ket_qua_moi)
        hang_doi.extend(ket_qua_moi)
        so_vong += 1
    return da_xu_ly, hang_doi


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


def lay_request_tiep_theo(hang_doi_tra_phi, hang_doi_mien_phi, so_lan_da_uu_tien_lien_tiep, gioi_han_uu_tien):
    if hang_doi_tra_phi and so_lan_da_uu_tien_lien_tiep < gioi_han_uu_tien:
        request = hang_doi_tra_phi.pop(0)
        return request, True, so_lan_da_uu_tien_lien_tiep + 1
    if hang_doi_mien_phi:
        request = hang_doi_mien_phi.pop(0)
        return request, False, 0
    if hang_doi_tra_phi:
        request = hang_doi_tra_phi.pop(0)
        return request, True, so_lan_da_uu_tien_lien_tiep + 1
    return None, None, so_lan_da_uu_tien_lien_tiep


def xu_ly_toan_bo_hai_hang_doi(hang_doi_tra_phi_ban_dau, hang_doi_mien_phi_ban_dau, gioi_han_uu_tien):
    hang_doi_tra_phi = list(hang_doi_tra_phi_ban_dau)
    hang_doi_mien_phi = list(hang_doi_mien_phi_ban_dau)
    thu_tu_phuc_vu = []
    so_lan_da_uu_tien_lien_tiep = 0
    while hang_doi_tra_phi or hang_doi_mien_phi:
        request, la_tra_phi, so_lan_da_uu_tien_lien_tiep = lay_request_tiep_theo(
            hang_doi_tra_phi, hang_doi_mien_phi, so_lan_da_uu_tien_lien_tiep, gioi_han_uu_tien
        )
        thu_tu_phuc_vu.append((request, la_tra_phi))
    so_mien_phi_da_phuc_vu = sum(1 for _, la_tra_phi in thu_tu_phuc_vu if la_tra_phi is False)
    return thu_tu_phuc_vu, so_mien_phi_da_phuc_vu


BANG_SU_CO = {
    "gpu_oom_tai_cao": {"nguyen_nhan": "batch size hoac do dai ngu canh qua lon", "cach_xu_ly": "giam max_model_len, bat paged attention cua vllm"},
    "do_tre_cao_gpu_ranh": {"nguyen_nhan": "request xu ly tuan tu, khong gop lo", "cach_xu_ly": "bat continuous batching"},
    "chi_phi_tang_bat_thuong": {"nguyen_nhan": "khong co cache, prompt lap lai nhieu lan", "cach_xu_ly": "them semantic cache va prompt caching"},
    "rag_lech_chu_de": {"nguyen_nhan": "chunking kem hoac thieu buoc rerank", "cach_xu_ly": "giam kich thuoc chunk, them buoc rerank"},
    "timeout_gateway": {"nguyen_nhan": "request llm dai hon timeout mac dinh", "cach_xu_ly": "tang timeout va chuyen sang streaming"},
}


def tra_cuu_su_co(trieu_chung):
    return BANG_SU_CO.get(trieu_chung, {"nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them"})


def van_hanh_mot_ngay(hang_doi_su_kien_tac_tu, gioi_han_vong_lap_tac_tu,
                       danh_sach_cau_hoi_vector, cache, nguong_cosine,
                       hang_doi_tra_phi, hang_doi_mien_phi, gioi_han_uu_tien,
                       nguong_ty_le_hit_thap):
    ket_qua_su_kien, con_lai_su_kien = xu_ly_toan_bo_hang_doi(hang_doi_su_kien_tac_tu, gioi_han_vong_lap_tac_tu)
    ty_le_hit = do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine)
    thu_tu_phuc_vu, so_mien_phi_da_phuc_vu = xu_ly_toan_bo_hai_hang_doi(hang_doi_tra_phi, hang_doi_mien_phi, gioi_han_uu_tien)
    if ty_le_hit < nguong_ty_le_hit_thap:
        huong_xu_ly = tra_cuu_su_co("chi_phi_tang_bat_thuong")
    else:
        huong_xu_ly = None
    return {
        "so_su_kien_da_xu_ly": len(ket_qua_su_kien),
        "ty_le_cache_hit": ty_le_hit,
        "so_mien_phi_da_phuc_vu": so_mien_phi_da_phuc_vu,
        "huong_xu_ly": huong_xu_ly,
    }


HANG_DOI_SU_KIEN = [
    {"loai_su_kien": "CodeWrittenEvent", "task_id": 1, "ket_qua_tinh": 4, "ket_qua_mong_doi": 5},
]
CACHE = [
    {"cau_hoi": "ao thun mau xanh gia bao nhieu", "vector": [1.0, 0.0, 0.0], "tra_loi": "Ao thun xanh gia 150000 dong"},
    {"cau_hoi": "chinh sach doi tra trong bao lau", "vector": [0.0, 1.0, 0.0], "tra_loi": "Doi tra trong vong 7 ngay"},
    {"cau_hoi": "lam sao lien he cham soc khach hang", "vector": [0.0, 0.0, 1.0], "tra_loi": "Goi hotline 1900 xxxx"},
]
CAC_VECTOR_MOI = [
    [0.99, 0.05, 0.0], [0.02, 0.98, 0.0], [0.3, 0.3, 0.3], [0.0, 0.02, 0.99], [0.5, 0.5, 0.0],
]
HANG_DOI_TRA_PHI = ["P1", "P2", "P3", "P4", "P5", "P6", "P7"]
HANG_DOI_MIEN_PHI = ["F1", "F2"]

KET_QUA_NGAY_1 = van_hanh_mot_ngay(
    HANG_DOI_SU_KIEN, 10, CAC_VECTOR_MOI, CACHE, 0.95,
    HANG_DOI_TRA_PHI, HANG_DOI_MIEN_PHI, 3, 0.7,
)

print(KET_QUA_NGAY_1)
```

```text title=readonly
{'so_su_kien_da_xu_ly': 4, 'ty_le_cache_hit': 0.6, 'so_mien_phi_da_phuc_vu': 2, 'huong_xu_ly': {'nguyen_nhan': 'khong co cache, prompt lap lai nhieu lan', 'cach_xu_ly': 'them semantic cache va prompt caching'}}
```

`(a)` task `1` (bug `4` sai với mong đợi `5`, sửa xong sau đúng `1` lần) →
`so_su_kien_da_xu_ly = 4`, đúng như bài `1`. `(b)` `5` câu hỏi qua `CACHE`
`3` mục, ngưỡng `0,95` → `ty_le_cache_hit = 0,6` (`3/5`), đúng như bài `2`.
`(c)` `7` request trả phí + `2` miễn phí, `gioi_han_uu_tien = 3` →
`so_mien_phi_da_phuc_vu = 2`, đúng như bài `3`. `(d)` `ty_le_hit = 0,6` NHỎ
HƠN `nguong_ty_le_hit_thap = 0,7` — BẤT THƯỜNG (cache hit thấp hơn kỳ vọng)
→ `huong_xu_ly` LÀ kết quả THẬT của `tra_cuu_su_co("chi_phi_tang_bat_thuong")`,
đúng như bài `4`. Bốn con số, bốn cơ chế, MỘT lệnh gọi.
::::

::::predict{#doan_ngay_khong_bat_thuong commitOnce}
Xét gọi LẠI `van_hanh_mot_ngay` với TẤT CẢ tham số GIỮ NGUYÊN như ví dụ
trên, CHỈ đổi `CAC_VECTOR_MOI` thành MỘT danh sách MỚI mà cả `3` câu hỏi
đều TRÙNG KHỚP TUYỆT ĐỐI với `3` mục của `CACHE`
(`[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]` — cosine `= 1,0` với
chính mục tương ứng của nó, nên `ty_le_cache_hit` sẽ LÀ `1,0`).

**Trước khi chạy thử**, bạn đoán: `huong_xu_ly` trong kết quả MỚI LÀ gì?

:::opt{correct}
`None` — `ty_le_hit = 1,0` KHÔNG nhỏ hơn `nguong_ty_le_hit_thap = 0,7`
(`1,0 < 0,7` là `False`), nên nhánh `if` không chạy — không có gì BẤT
THƯỜNG để tra cứu
:::

:::opt
Vẫn LÀ kết quả của `tra_cuu_su_co("chi_phi_tang_bat_thuong")` — vì bảng
troubleshooting LUÔN được tra mỗi khi hàm chạy, bất kể tỉ lệ cache hit là
bao nhiêu
::why
Gần đúng ở việc `tra_cuu_su_co` THẬT SỰ được ĐỊNH NGHĨA sẵn, sẵn sàng gọi
bất cứ lúc nào — quan sát về sự TỒN TẠI của hàm đó đúng.

Chỗ lệch: `van_hanh_mot_ngay` chỉ gọi `tra_cuu_su_co` BÊN TRONG nhánh
`if ty_le_hit < nguong_ty_le_hit_thap` — một điều kiện có thể ĐÚNG hoặc
SAI tùy dữ liệu. Với `ty_le_hit = 1,0` (mọi câu hỏi đều hit), điều kiện SAI,
nên nhánh `else` chạy, gán `huong_xu_ly = None` — hàm KHÔNG hề gọi
`tra_cuu_su_co` trong trường hợp này.
::
:::

:::opt
`{"nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them"}` —
vì không có bất thường nào xảy ra, nên `tra_cuu_su_co` được gọi với một
triệu chứng RỖNG/không xác định
::why
Gần đúng ở việc `"khong_xac_dinh"` LÀ một giá trị THẬT mà `tra_cuu_su_co`
có thể trả về (bài `4`, khi triệu chứng lạ) — quan sát về sự tồn tại của
giá trị đó đúng.

Chỗ lệch: `van_hanh_mot_ngay` KHÔNG gọi `tra_cuu_su_co` với bất kỳ triệu
chứng nào khi `ty_le_hit >= nguong_ty_le_hit_thap` — nhánh `else` gán
THẲNG `huong_xu_ly = None`, không thông qua `tra_cuu_su_co` chút nào.
`{"nguyen_nhan": "khong_xac_dinh", ...}` chỉ xuất hiện nếu hàm ĐÃ gọi
`tra_cuu_su_co` với một triệu chứng LẠ — điều không xảy ra ở đây.
::
:::
::::

::::code{#viet_van_hanh_mot_ngay}
Hoàn thiện `van_hanh_mot_ngay`: gọi ĐÚNG hàm xử lý hai hàng đợi ưu tiên
(bài `3`) để lấy `so_mien_phi_da_phuc_vu`, rồi gọi ĐÚNG hàm tra cứu sự cố
(bài `4`) khi tỉ lệ cache hit THẤP BẤT THƯỜNG.

```python title=starter
import math


def chay_test_tat_dinh(su_kien):
    return su_kien["ket_qua_tinh"] == su_kien["ket_qua_mong_doi"]


def xu_ly_mot_su_kien(su_kien, hang_doi_ket_qua):
    loai = su_kien["loai_su_kien"]
    if loai == "CodeWrittenEvent":
        if chay_test_tat_dinh(su_kien):
            hang_doi_ket_qua.append({"loai_su_kien": "TestPassedEvent", "task_id": su_kien["task_id"]})
        else:
            hang_doi_ket_qua.append({
                "loai_su_kien": "TestFailedEvent", "task_id": su_kien["task_id"],
                "ket_qua_tinh": su_kien["ket_qua_tinh"], "ket_qua_mong_doi": su_kien["ket_qua_mong_doi"],
            })
    elif loai == "TestFailedEvent":
        hang_doi_ket_qua.append({
            "loai_su_kien": "CodeWrittenEvent", "task_id": su_kien["task_id"],
            "ket_qua_tinh": su_kien["ket_qua_tinh"] + 1, "ket_qua_mong_doi": su_kien["ket_qua_mong_doi"],
        })


def xu_ly_toan_bo_hang_doi(hang_doi_ban_dau, gioi_han_vong_lap):
    hang_doi = list(hang_doi_ban_dau)
    da_xu_ly = []
    so_vong = 0
    while hang_doi and so_vong < gioi_han_vong_lap:
        su_kien = hang_doi.pop(0)
        da_xu_ly.append(su_kien)
        ket_qua_moi = []
        xu_ly_mot_su_kien(su_kien, ket_qua_moi)
        hang_doi.extend(ket_qua_moi)
        so_vong += 1
    return da_xu_ly, hang_doi


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


def lay_request_tiep_theo(hang_doi_tra_phi, hang_doi_mien_phi, so_lan_da_uu_tien_lien_tiep, gioi_han_uu_tien):
    if hang_doi_tra_phi and so_lan_da_uu_tien_lien_tiep < gioi_han_uu_tien:
        request = hang_doi_tra_phi.pop(0)
        return request, True, so_lan_da_uu_tien_lien_tiep + 1
    if hang_doi_mien_phi:
        request = hang_doi_mien_phi.pop(0)
        return request, False, 0
    if hang_doi_tra_phi:
        request = hang_doi_tra_phi.pop(0)
        return request, True, so_lan_da_uu_tien_lien_tiep + 1
    return None, None, so_lan_da_uu_tien_lien_tiep


def xu_ly_toan_bo_hai_hang_doi(hang_doi_tra_phi_ban_dau, hang_doi_mien_phi_ban_dau, gioi_han_uu_tien):
    hang_doi_tra_phi = list(hang_doi_tra_phi_ban_dau)
    hang_doi_mien_phi = list(hang_doi_mien_phi_ban_dau)
    thu_tu_phuc_vu = []
    so_lan_da_uu_tien_lien_tiep = 0
    while hang_doi_tra_phi or hang_doi_mien_phi:
        request, la_tra_phi, so_lan_da_uu_tien_lien_tiep = lay_request_tiep_theo(
            hang_doi_tra_phi, hang_doi_mien_phi, so_lan_da_uu_tien_lien_tiep, gioi_han_uu_tien
        )
        thu_tu_phuc_vu.append((request, la_tra_phi))
    so_mien_phi_da_phuc_vu = sum(1 for _, la_tra_phi in thu_tu_phuc_vu if la_tra_phi is False)
    return thu_tu_phuc_vu, so_mien_phi_da_phuc_vu


BANG_SU_CO = {
    "gpu_oom_tai_cao": {"nguyen_nhan": "batch size hoac do dai ngu canh qua lon", "cach_xu_ly": "giam max_model_len, bat paged attention cua vllm"},
    "do_tre_cao_gpu_ranh": {"nguyen_nhan": "request xu ly tuan tu, khong gop lo", "cach_xu_ly": "bat continuous batching"},
    "chi_phi_tang_bat_thuong": {"nguyen_nhan": "khong co cache, prompt lap lai nhieu lan", "cach_xu_ly": "them semantic cache va prompt caching"},
    "rag_lech_chu_de": {"nguyen_nhan": "chunking kem hoac thieu buoc rerank", "cach_xu_ly": "giam kich thuoc chunk, them buoc rerank"},
    "timeout_gateway": {"nguyen_nhan": "request llm dai hon timeout mac dinh", "cach_xu_ly": "tang timeout va chuyen sang streaming"},
}


def tra_cuu_su_co(trieu_chung):
    return BANG_SU_CO.get(trieu_chung, {"nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them"})


def van_hanh_mot_ngay(hang_doi_su_kien_tac_tu, gioi_han_vong_lap_tac_tu,
                       danh_sach_cau_hoi_vector, cache, nguong_cosine,
                       hang_doi_tra_phi, hang_doi_mien_phi, gioi_han_uu_tien,
                       nguong_ty_le_hit_thap):
    ket_qua_su_kien, con_lai_su_kien = xu_ly_toan_bo_hang_doi(hang_doi_su_kien_tac_tu, gioi_han_vong_lap_tac_tu)
    ty_le_hit = do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine)
    thu_tu_phuc_vu, so_mien_phi_da_phuc_vu = ___                # xu_ly_toan_bo_hai_hang_doi(hang_doi_tra_phi, hang_doi_mien_phi, gioi_han_uu_tien)
    if ty_le_hit < nguong_ty_le_hit_thap:
        huong_xu_ly = ___                                        # tra_cuu_su_co("chi_phi_tang_bat_thuong")
    else:
        huong_xu_ly = None
    return {
        "so_su_kien_da_xu_ly": len(ket_qua_su_kien),
        "ty_le_cache_hit": ty_le_hit,
        "so_mien_phi_da_phuc_vu": so_mien_phi_da_phuc_vu,
        "huong_xu_ly": huong_xu_ly,
    }


HANG_DOI_SU_KIEN = [
    {"loai_su_kien": "CodeWrittenEvent", "task_id": 1, "ket_qua_tinh": 4, "ket_qua_mong_doi": 5},
]
CACHE = [
    {"cau_hoi": "ao thun mau xanh gia bao nhieu", "vector": [1.0, 0.0, 0.0], "tra_loi": "Ao thun xanh gia 150000 dong"},
    {"cau_hoi": "chinh sach doi tra trong bao lau", "vector": [0.0, 1.0, 0.0], "tra_loi": "Doi tra trong vong 7 ngay"},
    {"cau_hoi": "lam sao lien he cham soc khach hang", "vector": [0.0, 0.0, 1.0], "tra_loi": "Goi hotline 1900 xxxx"},
]
CAC_VECTOR_MOI = [
    [0.99, 0.05, 0.0], [0.02, 0.98, 0.0], [0.3, 0.3, 0.3], [0.0, 0.02, 0.99], [0.5, 0.5, 0.0],
]
HANG_DOI_TRA_PHI = ["P1", "P2", "P3", "P4", "P5", "P6", "P7"]
HANG_DOI_MIEN_PHI = ["F1", "F2"]

KET_QUA_NGAY_1 = van_hanh_mot_ngay(
    HANG_DOI_SU_KIEN, 10, CAC_VECTOR_MOI, CACHE, 0.95,
    HANG_DOI_TRA_PHI, HANG_DOI_MIEN_PHI, 3, 0.7,
)

print(KET_QUA_NGAY_1)
```

```python title=solution
import math


def chay_test_tat_dinh(su_kien):
    return su_kien["ket_qua_tinh"] == su_kien["ket_qua_mong_doi"]


def xu_ly_mot_su_kien(su_kien, hang_doi_ket_qua):
    loai = su_kien["loai_su_kien"]
    if loai == "CodeWrittenEvent":
        if chay_test_tat_dinh(su_kien):
            hang_doi_ket_qua.append({"loai_su_kien": "TestPassedEvent", "task_id": su_kien["task_id"]})
        else:
            hang_doi_ket_qua.append({
                "loai_su_kien": "TestFailedEvent", "task_id": su_kien["task_id"],
                "ket_qua_tinh": su_kien["ket_qua_tinh"], "ket_qua_mong_doi": su_kien["ket_qua_mong_doi"],
            })
    elif loai == "TestFailedEvent":
        hang_doi_ket_qua.append({
            "loai_su_kien": "CodeWrittenEvent", "task_id": su_kien["task_id"],
            "ket_qua_tinh": su_kien["ket_qua_tinh"] + 1, "ket_qua_mong_doi": su_kien["ket_qua_mong_doi"],
        })


def xu_ly_toan_bo_hang_doi(hang_doi_ban_dau, gioi_han_vong_lap):
    hang_doi = list(hang_doi_ban_dau)
    da_xu_ly = []
    so_vong = 0
    while hang_doi and so_vong < gioi_han_vong_lap:
        su_kien = hang_doi.pop(0)
        da_xu_ly.append(su_kien)
        ket_qua_moi = []
        xu_ly_mot_su_kien(su_kien, ket_qua_moi)
        hang_doi.extend(ket_qua_moi)
        so_vong += 1
    return da_xu_ly, hang_doi


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


def lay_request_tiep_theo(hang_doi_tra_phi, hang_doi_mien_phi, so_lan_da_uu_tien_lien_tiep, gioi_han_uu_tien):
    if hang_doi_tra_phi and so_lan_da_uu_tien_lien_tiep < gioi_han_uu_tien:
        request = hang_doi_tra_phi.pop(0)
        return request, True, so_lan_da_uu_tien_lien_tiep + 1
    if hang_doi_mien_phi:
        request = hang_doi_mien_phi.pop(0)
        return request, False, 0
    if hang_doi_tra_phi:
        request = hang_doi_tra_phi.pop(0)
        return request, True, so_lan_da_uu_tien_lien_tiep + 1
    return None, None, so_lan_da_uu_tien_lien_tiep


def xu_ly_toan_bo_hai_hang_doi(hang_doi_tra_phi_ban_dau, hang_doi_mien_phi_ban_dau, gioi_han_uu_tien):
    hang_doi_tra_phi = list(hang_doi_tra_phi_ban_dau)
    hang_doi_mien_phi = list(hang_doi_mien_phi_ban_dau)
    thu_tu_phuc_vu = []
    so_lan_da_uu_tien_lien_tiep = 0
    while hang_doi_tra_phi or hang_doi_mien_phi:
        request, la_tra_phi, so_lan_da_uu_tien_lien_tiep = lay_request_tiep_theo(
            hang_doi_tra_phi, hang_doi_mien_phi, so_lan_da_uu_tien_lien_tiep, gioi_han_uu_tien
        )
        thu_tu_phuc_vu.append((request, la_tra_phi))
    so_mien_phi_da_phuc_vu = sum(1 for _, la_tra_phi in thu_tu_phuc_vu if la_tra_phi is False)
    return thu_tu_phuc_vu, so_mien_phi_da_phuc_vu


BANG_SU_CO = {
    "gpu_oom_tai_cao": {"nguyen_nhan": "batch size hoac do dai ngu canh qua lon", "cach_xu_ly": "giam max_model_len, bat paged attention cua vllm"},
    "do_tre_cao_gpu_ranh": {"nguyen_nhan": "request xu ly tuan tu, khong gop lo", "cach_xu_ly": "bat continuous batching"},
    "chi_phi_tang_bat_thuong": {"nguyen_nhan": "khong co cache, prompt lap lai nhieu lan", "cach_xu_ly": "them semantic cache va prompt caching"},
    "rag_lech_chu_de": {"nguyen_nhan": "chunking kem hoac thieu buoc rerank", "cach_xu_ly": "giam kich thuoc chunk, them buoc rerank"},
    "timeout_gateway": {"nguyen_nhan": "request llm dai hon timeout mac dinh", "cach_xu_ly": "tang timeout va chuyen sang streaming"},
}


def tra_cuu_su_co(trieu_chung):
    return BANG_SU_CO.get(trieu_chung, {"nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them"})


def van_hanh_mot_ngay(hang_doi_su_kien_tac_tu, gioi_han_vong_lap_tac_tu,
                       danh_sach_cau_hoi_vector, cache, nguong_cosine,
                       hang_doi_tra_phi, hang_doi_mien_phi, gioi_han_uu_tien,
                       nguong_ty_le_hit_thap):
    ket_qua_su_kien, con_lai_su_kien = xu_ly_toan_bo_hang_doi(hang_doi_su_kien_tac_tu, gioi_han_vong_lap_tac_tu)
    ty_le_hit = do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine)
    thu_tu_phuc_vu, so_mien_phi_da_phuc_vu = xu_ly_toan_bo_hai_hang_doi(hang_doi_tra_phi, hang_doi_mien_phi, gioi_han_uu_tien)
    if ty_le_hit < nguong_ty_le_hit_thap:
        huong_xu_ly = tra_cuu_su_co("chi_phi_tang_bat_thuong")
    else:
        huong_xu_ly = None
    return {
        "so_su_kien_da_xu_ly": len(ket_qua_su_kien),
        "ty_le_cache_hit": ty_le_hit,
        "so_mien_phi_da_phuc_vu": so_mien_phi_da_phuc_vu,
        "huong_xu_ly": huong_xu_ly,
    }


HANG_DOI_SU_KIEN = [
    {"loai_su_kien": "CodeWrittenEvent", "task_id": 1, "ket_qua_tinh": 4, "ket_qua_mong_doi": 5},
]
CACHE = [
    {"cau_hoi": "ao thun mau xanh gia bao nhieu", "vector": [1.0, 0.0, 0.0], "tra_loi": "Ao thun xanh gia 150000 dong"},
    {"cau_hoi": "chinh sach doi tra trong bao lau", "vector": [0.0, 1.0, 0.0], "tra_loi": "Doi tra trong vong 7 ngay"},
    {"cau_hoi": "lam sao lien he cham soc khach hang", "vector": [0.0, 0.0, 1.0], "tra_loi": "Goi hotline 1900 xxxx"},
]
CAC_VECTOR_MOI = [
    [0.99, 0.05, 0.0], [0.02, 0.98, 0.0], [0.3, 0.3, 0.3], [0.0, 0.02, 0.99], [0.5, 0.5, 0.0],
]
HANG_DOI_TRA_PHI = ["P1", "P2", "P3", "P4", "P5", "P6", "P7"]
HANG_DOI_MIEN_PHI = ["F1", "F2"]

KET_QUA_NGAY_1 = van_hanh_mot_ngay(
    HANG_DOI_SU_KIEN, 10, CAC_VECTOR_MOI, CACHE, 0.95,
    HANG_DOI_TRA_PHI, HANG_DOI_MIEN_PHI, 3, 0.7,
)

print(KET_QUA_NGAY_1)
```

```python title=test
assert KET_QUA_NGAY_1 == {
    "so_su_kien_da_xu_ly": 4,
    "ty_le_cache_hit": 0.6,
    "so_mien_phi_da_phuc_vu": 2,
    "huong_xu_ly": {
        "nguyen_nhan": "khong co cache, prompt lap lai nhieu lan",
        "cach_xu_ly": "them semantic cache va prompt caching",
    },
}, f"KET_QUA_NGAY_1 sai -- dang ra {KET_QUA_NGAY_1}"

# bien: mot ngay KHONG bat thuong (cache hit toan trung khop) -> huong_xu_ly la None
CAC_VECTOR_TOT = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
KET_QUA_NGAY_2 = van_hanh_mot_ngay(
    HANG_DOI_SU_KIEN, 10, CAC_VECTOR_TOT, CACHE, 0.95,
    HANG_DOI_TRA_PHI, HANG_DOI_MIEN_PHI, 3, 0.7,
)
assert KET_QUA_NGAY_2["ty_le_cache_hit"] == 1.0, f"ty_le_cache_hit ngay 2 phai la 1.0 -- dang ra {KET_QUA_NGAY_2['ty_le_cache_hit']}"
assert KET_QUA_NGAY_2["huong_xu_ly"] is None, f"khong bat thuong thi huong_xu_ly phai la None -- dang ra {KET_QUA_NGAY_2['huong_xu_ly']}"
assert KET_QUA_NGAY_2["so_su_kien_da_xu_ly"] == 4, "so_su_kien_da_xu_ly khong phu thuoc du lieu cache, phai van la 4"
assert KET_QUA_NGAY_2["so_mien_phi_da_phuc_vu"] == 2, "so_mien_phi_da_phuc_vu khong phu thuoc du lieu cache, phai van la 2"

# bien: nguong_ty_le_hit_thap THAT SU rang buoc co tra cuu hay khong (tren CUNG du lieu ngay 1)
KET_QUA_NGUONG_THAP = van_hanh_mot_ngay(
    HANG_DOI_SU_KIEN, 10, CAC_VECTOR_MOI, CACHE, 0.95,
    HANG_DOI_TRA_PHI, HANG_DOI_MIEN_PHI, 3, 0.5,
)
assert KET_QUA_NGUONG_THAP["ty_le_cache_hit"] == 0.6, "doi nguong_ty_le_hit_thap khong doi ty_le_cache_hit (van la 0.6)"
assert KET_QUA_NGUONG_THAP["huong_xu_ly"] is None, f"nguong 0.5 (0.6 khong nho hon 0.5) phai cho huong_xu_ly la None -- dang ra {KET_QUA_NGUONG_THAP['huong_xu_ly']}"
assert KET_QUA_NGUONG_THAP["huong_xu_ly"] != KET_QUA_NGAY_1["huong_xu_ly"], "doi nguong_ty_le_hit_thap tu 0.7 xuong 0.5 phai doi huong_xu_ly (dict -> None)"

# kiem tra kieu du lieu: so_mien_phi_da_phuc_vu phai la SO NGUYEN, khong phai chuoi hay tuple
assert isinstance(KET_QUA_NGAY_1["so_mien_phi_da_phuc_vu"], int), "so_mien_phi_da_phuc_vu phai la int"
assert KET_QUA_NGAY_1["huong_xu_ly"] is None or isinstance(KET_QUA_NGAY_1["huong_xu_ly"], dict), "huong_xu_ly phai la None hoac dict, khong phai kieu khac"
```

:::hints
- kind: attention
  body: Hai chỗ trống, CÙNG một hàm `van_hanh_mot_ngay`. Chỗ đầu là GIÁ TRỊ GÁN cho cặp `thu_tu_phuc_vu, so_mien_phi_da_phuc_vu` — gọi lại hàm xử lý hai hàng đợi ưu tiên đã viết ở bài `3`. Chỗ hai (BÊN TRONG nhánh `if`) là GIÁ TRỊ GÁN cho `huong_xu_ly` — gọi lại hàm tra cứu sự cố đã viết ở bài `4`, với ĐÚNG triệu chứng `"chi_phi_tang_bat_thuong"`.
- kind: strategy
  body: 'Chỗ đầu: `xu_ly_toan_bo_hai_hang_doi(hang_doi_tra_phi, hang_doi_mien_phi, gioi_han_uu_tien)` — đúng ba đối số theo thứ tự đã định nghĩa ở bài `3`, trả về MỘT cặp `(thu_tu_phuc_vu, so_mien_phi_da_phuc_vu)`. Chỗ hai: `tra_cuu_su_co("chi_phi_tang_bat_thuong")` — triệu chứng CỐ ĐỊNH, đúng logic §39 rằng cache hit thấp gây tăng chi phí.'
- kind: one-line
  body: 'Chỗ đầu là `xu_ly_toan_bo_hai_hang_doi(hang_doi_tra_phi, hang_doi_mien_phi, gioi_han_uu_tien)`, chỗ hai là `tra_cuu_su_co("chi_phi_tang_bat_thuong")`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT xu_ly_toan_bo_hai_hang_doi(...) voi dung ba doi so (khong duoc tu viet lai logic hang doi uu tien); cho trong hai phai GOI THAT tra_cuu_su_co("chi_phi_tang_bat_thuong") (khong duoc chep san mot dict co dinh hay goi voi trieu chung khac)
  requireAst:
  - kind: uses-call, target: xu_ly_toan_bo_hai_hang_doi, min: 1
  - kind: uses-call, target: tra_cuu_su_co, min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, dist build that, trich
  # CHINH XAC khoi solution cua file nay -- gom toan bo NAM toolkit da tai
  # dung tu bai 1-4) -- xac nhan DUNG CHINH XAC (min VA min+1):
  # xu_ly_toan_bo_hai_hang_doi=1, tra_cuu_su_co=1.
  # xu_ly_toan_bo_hai_hang_doi=1: DUY NHAT o cho trong dau -- ham nay KHONG
  # tu goi lai chinh no, va khong co dong demo nao khac trong solution goi
  # rieng le no.
  # tra_cuu_su_co=1: DUY NHAT o cho trong hai (ben trong nhanh if) -- ham
  # nay KHONG tu goi lai chinh no, va khong co dong demo nao khac goi no.
  # Dien bua "True" vao ca hai cho trong ("thu_tu_phuc_vu,
  # so_mien_phi_da_phuc_vu = True" -- se NEM LOI unpack o tier run, nhung
  # rieng o tier static thi xu_ly_toan_bo_hai_hang_doi=0; "huong_xu_ly =
  # True" cho tra_cuu_su_co=0) -- CA HAI luat CHAN DUNG (da CHAY THAT xac
  # nhan qua kiemAst).
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho
  # trong -- xac dinh ranh gioi TU CHINH khoi starter, khong doan tay -- va
  # CHAY THAT qua kiemAst() THAT VA python3 THAT): dien
  # "tra_cuu_su_co(\"chi_phi_tang_bat_thuong\")" vao cho trong dau
  # ("thu_tu_phuc_vu, so_mien_phi_da_phuc_vu =
  # tra_cuu_su_co(\"chi_phi_tang_bat_thuong\")") VA dien
  # "xu_ly_toan_bo_hai_hang_doi(hang_doi_tra_phi, hang_doi_mien_phi,
  # gioi_han_uu_tien)" vao cho trong hai ("huong_xu_ly =
  # xu_ly_toan_bo_hai_hang_doi(hang_doi_tra_phi, hang_doi_mien_phi,
  # gioi_han_uu_tien)") -- da CHAY THAT qua kiemAst(): tong so lan goi
  # xu_ly_toan_bo_hai_hang_doi VA tong so lan goi tra_cuu_su_co tren TOAN
  # BO solution DEU KHONG DOI (van dung 1 va 1, chi doi VI TRI) -- static
  # KHONG bat duoc mutant nay.
  # Mutant nay KHONG nem loi runtime (khac cac bai truoc trong quest nay):
  # `tra_cuu_su_co(...)` tra ve mot dict CO DUNG hai khoa ("nguyen_nhan",
  # "cach_xu_ly"), va gan tuple hai bien tu MOT dict trong Python duyet qua
  # CAC KHOA cua no theo thu tu chen -- nen "thu_tu_phuc_vu,
  # so_mien_phi_da_phuc_vu = tra_cuu_su_co(...)" THAT SU chay duoc, gan
  # thu_tu_phuc_vu = "nguyen_nhan", so_mien_phi_da_phuc_vu = "cach_xu_ly"
  # (hai CHUOI, khong phai gia tri that). Tuong tu, "huong_xu_ly =
  # xu_ly_toan_bo_hai_hang_doi(...)" cung chay duoc (ham nay nhan dung ba
  # doi so ton tai trong scope), nhung tra ve MOT TUPLE (thu_tu_phuc_vu,
  # so_mien_phi_da_phuc_vu), khong phai mot dict/None nhu huong_xu_ly phai
  # la. Da tu CHAY THAT xac nhan: ket qua cuoi cung LA
  # {"so_su_kien_da_xu_ly": 4, "ty_le_cache_hit": 0.6,
  # "so_mien_phi_da_phuc_vu": "cach_xu_ly", "huong_xu_ly": (<list>, 2)} --
  # HOAN TOAN khac KET_QUA_NGAY_1 mong doi. Mutant nay BI BAT boi tier
  # 'tests': assert KET_QUA_NGAY_1 == {...} that bai vi
  # so_mien_phi_da_phuc_vu la chuoi "cach_xu_ly" thay vi so nguyen 2, VA
  # assert isinstance(..., int) that bai them lan nua -- doc lap voi
  # static.
  # Da tu ra soat GOTCHA #6: "xu_ly_toan_bo_hai_hang_doi" va "tra_cuu_su_co"
  # la hai TEN HAM rieng biet, khong trung ten voi bat ky ham nao khac
  # trong toan bo NAM toolkit da tai dung (chay_test_tat_dinh,
  # xu_ly_mot_su_kien, xu_ly_toan_bo_hang_doi, tich_vo_huong, do_dai_vector,
  # tuong_dong_cosine, tim_trong_cache, do_ty_le_cache_hit,
  # lay_request_tiep_theo -- khong ten nao trung).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\{'so_su_kien_da_xu_ly': 4, 'ty_le_cache_hit': 0\\.6, 'so_mien_phi_da_phuc_vu': 2, 'huong_xu_ly': \\{'nguyen_nhan': 'khong co cache, prompt lap lai nhieu lan', 'cach_xu_ly': 'them semantic cache va prompt caching'\\}\\}\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn con số, một lệnh gọi: `4` sự kiện tác tử, `0,6` tỉ lệ cache hit, `2`
request miễn phí không bị đói, VÀ một hướng xử lý CỤ THỂ vì cache hit thấp
bất thường. `q8.6d` "Đa tác tử và quan sát AI" đóng tại `5/5`.
::::

::::reflect{#nghi-lai}
`van_hanh_mot_ngay` không viết thêm MỘT thuật toán nào — mọi con số nó trả
về đều đến từ bốn hàm đã kiểm chứng riêng lẻ ở bốn bài trước
(`xu_ly_toan_bo_hang_doi`, `do_ty_le_cache_hit`, `xu_ly_toan_bo_hai_hang_doi`,
`tra_cuu_su_co`). Điều nó chứng minh được là RÁP đúng thứ tự cho ra một
luồng vận hành có Ý NGHĨA: một con số đo được ở bước `(b)`
(`ty_le_cache_hit`) trực tiếp QUYẾT ĐỊNH có cần chạy bước `(d)` hay không —
đúng cách một hệ thống quan sát AI thật hoạt động: các chỉ số (metrics)
KHÔNG tồn tại riêng lẻ, chúng KÍCH HOẠT hành động tiếp theo. Đo cụ thể:
cùng một ngày vận hành (`ty_le_hit = 0,6`, `nguong = 0,7`) → CÓ tra cứu;
đổi `nguong_ty_le_hit_thap` xuống `0,5` (cùng `ty_le_hit`) → KHÔNG tra cứu
— `huong_xu_ly` đổi từ một `dict` cụ thể thành `None`, chỉ vì MỘT tham số
ngưỡng đổi.

`q8.6d` "Đa tác tử và quan sát AI" đóng tại `5/5`. `T8.6` "Hạ tầng & vận
hành AI" tiếp tục với `q8.6e` — BOSS quý CUỐI CÙNG của `T8.6`, ráp TOÀN BỘ
bốn quest (`q8.6a` hạ tầng suy luận, `q8.6b` AI Gateway, `q8.6c` RAG quy mô
lớn, `q8.6d` — quest này) thành MỘT luồng xử lý request đầu-cuối, đóng
`T8.6` tại `26/26` VÀ `R8` "AI · Generative AI · RAG" tại `190/190`.
::::

::::checkpoint{mastery=0.88}
::::
