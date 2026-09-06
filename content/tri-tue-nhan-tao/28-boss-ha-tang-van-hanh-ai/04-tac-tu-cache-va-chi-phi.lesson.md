---
id: tri-tue-nhan-tao.boss-ha-tang-van-hanh-ai.tac-tu-cache-va-chi-phi
title: "q8.6e bài 4 — bước tác tử + đo cache/chi phí: ghi sự kiện, đo hit rate, tra cứu KHI cần"
summary: "buoc_tac_tu_va_do_chi_phi(lich_su_su_kien, id_request, danh_sach_cau_hoi_vector, cache, nguong_cosine, nguong_ty_le_hit_thap) ghi MOT su kien 'HoanThanhRequestEvent' vao lich su (event sourcing toi gian, q8.6d bai 1), goi do_ty_le_cache_hit (tai dung nguyen van tu q8.6d bai 2) de do ty le cache hit, roi NEU ty le do THAP HON nguong_ty_le_hit_thap thi tra_cuu_su_co('chi_phi_tang_bat_thuong') (tai dung nguyen van tu q8.6d bai 4) -- khac thi huong_xu_ly la None. Tren CACHE/CAC_VECTOR_MOI cua q8.6d (ty_le_hit=0,6) va nguong=0,7: goi 3 lan lien tiep cho 3 id_request khac nhau -- lich_su_su_kien co DUNG 3 su kien theo dung thu tu [101,102,103], huong_xu_ly LA dict cua 'khong co cache...'. Doi nguong xuong DUNG 0,6 (bang tuyet doi voi ty_le_hit): huong_xu_ly THANH None (0,6 khong NHO HON 0,6)."
locale: vi
track: tri-tue-nhan-tao
module: boss-ha-tang-van-hanh-ai
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.tac-tu-cache-va-chi-phi]
requires: [ai.rag-da-khach-hang-sau-hang-doi]
concepts: [ai.tac-tu-cache-va-chi-phi]
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
Request đã qua cổng, đã vào lô, đã truy xuất RAG (nếu cần). Việc CUỐI CÙNG
một hệ thống lớn làm SAU khi phục vụ một request: ghi lại rằng nó ĐÃ XONG,
VÀ tự hỏi — hệ thống có đang chạy bình thường không?
::::

::::explain{#su_kien_toi_gian_va_bao_dong_chi_phi}
`q8.6d` bài `1` dạy Event Sourcing ĐẦY ĐỦ: nhiều tác tử phối hợp qua một
hàng đợi sự kiện, mỗi sự kiện có thể sinh sự kiện MỚI. Ở QUY MÔ một hệ
thống lớn hơn (bài này), điều CẦN GHI LẠI đơn giản hơn nhiều: một request
đã hoàn thành. Đó LÀ mức TỐI GIẢN của event sourcing — một sự kiện
`"HoanThanhRequestEvent"` được `append` vào lịch sử, không sinh ra sự kiện
tiếp theo nào.

Song song đó, `q8.6d` bài `2` đã dạy `do_ty_le_cache_hit` — đo tỉ lệ câu
hỏi được trả lời từ cache thay vì gọi LLM thật. Và bài `4` dạy
`tra_cuu_su_co` — tra bảng troubleshooting khi có triệu chứng. Bài này nối
CẢ BA:

```
buoc_tac_tu_va_do_chi_phi(lich_su_su_kien, id_request, danh_sach_cau_hoi_vector,
                           cache, nguong_cosine, nguong_ty_le_hit_thap):
  lich_su_su_kien.append({"loai_su_kien": "HoanThanhRequestEvent", "request_id": id_request})  # event sourcing toi gian
  ty_le_hit = do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine)                # q8.6d bai 2
  neu ty_le_hit < nguong_ty_le_hit_thap:
    huong_xu_ly = tra_cuu_su_co("chi_phi_tang_bat_thuong")                                      # q8.6d bai 4
  khac:
    huong_xu_ly = None
  tra ve lich_su_su_kien, ty_le_hit, huong_xu_ly
```

Đây CHÍNH LÀ chuỗi suy luận `q8.6d` BOSS đã dạy (`ty_le_hit` thấp bất
thường → tra cứu `"chi_phi_tang_bat_thuong"`), đặt LẠI trong ngữ cảnh MỘT
request cụ thể, thay vì MỘT "ngày vận hành" trừu tượng.
::::

::::example{#ba_request_lien_tiep}
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
    "gpu_oom_tai_cao": {
        "nguyen_nhan": "batch size hoac do dai ngu canh qua lon",
        "cach_xu_ly": "giam max_model_len, bat paged attention cua vllm",
    },
    "do_tre_cao_gpu_ranh": {
        "nguyen_nhan": "request xu ly tuan tu, khong gop lo",
        "cach_xu_ly": "bat continuous batching",
    },
    "chi_phi_tang_bat_thuong": {
        "nguyen_nhan": "khong co cache, prompt lap lai nhieu lan",
        "cach_xu_ly": "them semantic cache va prompt caching",
    },
    "rag_lech_chu_de": {
        "nguyen_nhan": "chunking kem hoac thieu buoc rerank",
        "cach_xu_ly": "giam kich thuoc chunk, them buoc rerank",
    },
    "timeout_gateway": {
        "nguyen_nhan": "request llm dai hon timeout mac dinh",
        "cach_xu_ly": "tang timeout va chuyen sang streaming",
    },
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


CACHE = [
    {"cau_hoi": "ao thun mau xanh gia bao nhieu", "vector": [1.0, 0.0, 0.0], "tra_loi": "Ao thun xanh gia 150000 dong"},
    {"cau_hoi": "chinh sach doi tra trong bao lau", "vector": [0.0, 1.0, 0.0], "tra_loi": "Doi tra trong vong 7 ngay"},
    {"cau_hoi": "lam sao lien he cham soc khach hang", "vector": [0.0, 0.0, 1.0], "tra_loi": "Goi hotline 1900 xxxx"},
]

CAC_VECTOR_MOI = [
    [0.99, 0.05, 0.0],
    [0.02, 0.98, 0.0],
    [0.3, 0.3, 0.3],
    [0.0, 0.02, 0.99],
    [0.5, 0.5, 0.0],
]

LICH_SU_SU_KIEN = []

lich_1, ty_le_1, huong_1 = buoc_tac_tu_va_do_chi_phi(LICH_SU_SU_KIEN, 101, CAC_VECTOR_MOI, CACHE, 0.95, 0.7)
lich_2, ty_le_2, huong_2 = buoc_tac_tu_va_do_chi_phi(LICH_SU_SU_KIEN, 102, CAC_VECTOR_MOI, CACHE, 0.95, 0.7)
lich_3, ty_le_3, huong_3 = buoc_tac_tu_va_do_chi_phi(LICH_SU_SU_KIEN, 103, CAC_VECTOR_MOI, CACHE, 0.95, 0.5)

print(len(LICH_SU_SU_KIEN))
print([e["request_id"] for e in LICH_SU_SU_KIEN])
print(ty_le_1, huong_1)
print(ty_le_3, huong_3)
```

```text title=readonly
3
[101, 102, 103]
0.6 {'nguyen_nhan': 'khong co cache, prompt lap lai nhieu lan', 'cach_xu_ly': 'them semantic cache va prompt caching'}
0.6 None
```

Ba lần gọi liên tiếp, mỗi lần MỘT `id_request` khác nhau (`101, 102, 103`)
— `lich_su_su_kien` (CHÍNH LÀ `LICH_SU_SU_KIEN`, được TRUYỀN VÀO VÀ trả về,
`.append` sửa TẠI CHỖ) tích luỹ đúng `3` sự kiện, ĐÚNG thứ tự gọi. `CACHE`
VÀ `CAC_VECTOR_MOI` giống hệt `q8.6d` bài `2` — `ty_le_hit = 0,6` (`3` HIT
trên `5` câu hỏi), KHÔNG ĐỔI giữa các lần gọi vì dữ liệu đầu vào giống hệt
nhau. Lần gọi `1` VÀ `2` dùng `nguong_ty_le_hit_thap = 0,7` — `0,6 < 0,7`
đúng, BẤT THƯỜNG, `huong_1 = tra_cuu_su_co("chi_phi_tang_bat_thuong")`. Lần
`3` đổi ngưỡng xuống `0,5` — `0,6 < 0,5` SAI, KHÔNG bất thường,
`huong_3 = None`.
::::

::::predict{#doan_nguong_bang_tuyet_doi commitOnce}
Xét đúng `CACHE`/`CAC_VECTOR_MOI` ở ví dụ trên (`ty_le_hit` LUÔN LÀ `0,6`
trên tập này). Gọi `buoc_tac_tu_va_do_chi_phi` với `nguong_ty_le_hit_thap`
đặt ĐÚNG BẰNG `0,6` — TUYỆT ĐỐI bằng `ty_le_hit`, không hơn không kém.

**Trước khi chạy thử**, bạn đoán: `huong_xu_ly` trả về LÀ gì?

:::opt{correct}
`None` — điều kiện là `ty_le_hit < nguong_ty_le_hit_thap`, dùng `<` NGHIÊM
NGẶT; `0,6 < 0,6` LÀ `False`, nên nhánh bất thường KHÔNG chạy, dù hai giá
trị bằng nhau TUYỆT ĐỐI
:::

:::opt
Dict của `"chi_phi_tang_bat_thuong"` — vì `ty_le_hit` "chạm" đúng ngưỡng
cũng đủ để coi LÀ bất thường, an toàn hơn là bỏ sót
::why
Gần đúng ở trực giác "phòng ngừa": trong một hệ thống GIÁM SÁT thật, coi
việc CHẠM ngưỡng LÀ đáng báo động cũng LÀ một lựa chọn thiết kế hợp lý.

Chỗ lệch: `buoc_tac_tu_va_do_chi_phi`, đúng như đã viết, dùng phép so sánh
`<` NGHIÊM NGẶT — không phải `<=`. Ở ranh giới CHÍNH XÁC (`ty_le_hit` BẰNG
`nguong_ty_le_hit_thap`), điều kiện đó SAI, nên `huong_xu_ly = None`. Đây
không phải một lỗi thiết kế bài dạy — nó LÀ ranh giới THẬT của phép so
sánh đã viết, giống hệt cách `xu_ly_yeu_cau` (`q8.6b`) dùng `<=` chứ không
`<` ở MỘT ranh giới KHÁC.
::
:::

:::opt
Chương trình ném lỗi, vì so sánh hai số THỰC bằng nhau tuyệt đối
(`0,6 == 0,6`) không đáng tin cậy trong Python (sai số dấu phẩy động)
::why
Gần đúng ở việc so sánh SỐ THỰC bằng `==` đôi khi THẬT SỰ gặp sai số dấu
phẩy động trong các phép TÍNH TRUNG GIAN phức tạp — đây LÀ một mối lo hợp
lý trong lập trình số học nói chung.

Chỗ lệch: ở đây không hề có phép so sánh `==` nào cả — điều kiện dùng
`<`, và cả hai giá trị (`ty_le_hit` tính từ `3/5`, `nguong_ty_le_hit_thap`
được TRUYỀN thẳng LÀ `0,6`) đều LÀ kết quả của phép tính ĐƠN GIẢN, không hề
tích luỹ sai số nào tới mức gây ra hành vi bất định. `3/5` trong Python cho
ĐÚNG `0.6`, không xấp xỉ.
::
:::
::::

::::code{#viet_buoc_tac_tu_va_do_chi_phi}
Hoàn thiện `buoc_tac_tu_va_do_chi_phi`: gọi ĐÚNG hàm đo tỉ lệ cache hit
(`q8.6d` bài `2`), rồi tra cứu ĐÚNG bảng sự cố (`q8.6d` bài `4`) khi tỉ lệ
đó THẤP BẤT THƯỜNG.

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
    "gpu_oom_tai_cao": {
        "nguyen_nhan": "batch size hoac do dai ngu canh qua lon",
        "cach_xu_ly": "giam max_model_len, bat paged attention cua vllm",
    },
    "do_tre_cao_gpu_ranh": {
        "nguyen_nhan": "request xu ly tuan tu, khong gop lo",
        "cach_xu_ly": "bat continuous batching",
    },
    "chi_phi_tang_bat_thuong": {
        "nguyen_nhan": "khong co cache, prompt lap lai nhieu lan",
        "cach_xu_ly": "them semantic cache va prompt caching",
    },
    "rag_lech_chu_de": {
        "nguyen_nhan": "chunking kem hoac thieu buoc rerank",
        "cach_xu_ly": "giam kich thuoc chunk, them buoc rerank",
    },
    "timeout_gateway": {
        "nguyen_nhan": "request llm dai hon timeout mac dinh",
        "cach_xu_ly": "tang timeout va chuyen sang streaming",
    },
}


def tra_cuu_su_co(trieu_chung):
    return BANG_SU_CO.get(trieu_chung, {"nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them"})


def buoc_tac_tu_va_do_chi_phi(lich_su_su_kien, id_request, danh_sach_cau_hoi_vector, cache, nguong_cosine, nguong_ty_le_hit_thap):
    lich_su_su_kien.append({"loai_su_kien": "HoanThanhRequestEvent", "request_id": id_request})
    ty_le_hit = ___                      # do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine)
    if ty_le_hit < nguong_ty_le_hit_thap:
        huong_xu_ly = ___                # tra_cuu_su_co("chi_phi_tang_bat_thuong")
    else:
        huong_xu_ly = None
    return lich_su_su_kien, ty_le_hit, huong_xu_ly


CACHE = [
    {"cau_hoi": "ao thun mau xanh gia bao nhieu", "vector": [1.0, 0.0, 0.0], "tra_loi": "Ao thun xanh gia 150000 dong"},
    {"cau_hoi": "chinh sach doi tra trong bao lau", "vector": [0.0, 1.0, 0.0], "tra_loi": "Doi tra trong vong 7 ngay"},
    {"cau_hoi": "lam sao lien he cham soc khach hang", "vector": [0.0, 0.0, 1.0], "tra_loi": "Goi hotline 1900 xxxx"},
]

CAC_VECTOR_MOI = [
    [0.99, 0.05, 0.0],
    [0.02, 0.98, 0.0],
    [0.3, 0.3, 0.3],
    [0.0, 0.02, 0.99],
    [0.5, 0.5, 0.0],
]

LICH_SU_SU_KIEN = []

lich_1, ty_le_1, huong_1 = buoc_tac_tu_va_do_chi_phi(LICH_SU_SU_KIEN, 101, CAC_VECTOR_MOI, CACHE, 0.95, 0.7)
lich_2, ty_le_2, huong_2 = buoc_tac_tu_va_do_chi_phi(LICH_SU_SU_KIEN, 102, CAC_VECTOR_MOI, CACHE, 0.95, 0.7)
lich_3, ty_le_3, huong_3 = buoc_tac_tu_va_do_chi_phi(LICH_SU_SU_KIEN, 103, CAC_VECTOR_MOI, CACHE, 0.95, 0.5)

print(len(LICH_SU_SU_KIEN))
print([e["request_id"] for e in LICH_SU_SU_KIEN])
print(ty_le_1, huong_1)
print(ty_le_3, huong_3)
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
    "gpu_oom_tai_cao": {
        "nguyen_nhan": "batch size hoac do dai ngu canh qua lon",
        "cach_xu_ly": "giam max_model_len, bat paged attention cua vllm",
    },
    "do_tre_cao_gpu_ranh": {
        "nguyen_nhan": "request xu ly tuan tu, khong gop lo",
        "cach_xu_ly": "bat continuous batching",
    },
    "chi_phi_tang_bat_thuong": {
        "nguyen_nhan": "khong co cache, prompt lap lai nhieu lan",
        "cach_xu_ly": "them semantic cache va prompt caching",
    },
    "rag_lech_chu_de": {
        "nguyen_nhan": "chunking kem hoac thieu buoc rerank",
        "cach_xu_ly": "giam kich thuoc chunk, them buoc rerank",
    },
    "timeout_gateway": {
        "nguyen_nhan": "request llm dai hon timeout mac dinh",
        "cach_xu_ly": "tang timeout va chuyen sang streaming",
    },
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


CACHE = [
    {"cau_hoi": "ao thun mau xanh gia bao nhieu", "vector": [1.0, 0.0, 0.0], "tra_loi": "Ao thun xanh gia 150000 dong"},
    {"cau_hoi": "chinh sach doi tra trong bao lau", "vector": [0.0, 1.0, 0.0], "tra_loi": "Doi tra trong vong 7 ngay"},
    {"cau_hoi": "lam sao lien he cham soc khach hang", "vector": [0.0, 0.0, 1.0], "tra_loi": "Goi hotline 1900 xxxx"},
]

CAC_VECTOR_MOI = [
    [0.99, 0.05, 0.0],
    [0.02, 0.98, 0.0],
    [0.3, 0.3, 0.3],
    [0.0, 0.02, 0.99],
    [0.5, 0.5, 0.0],
]

LICH_SU_SU_KIEN = []

lich_1, ty_le_1, huong_1 = buoc_tac_tu_va_do_chi_phi(LICH_SU_SU_KIEN, 101, CAC_VECTOR_MOI, CACHE, 0.95, 0.7)
lich_2, ty_le_2, huong_2 = buoc_tac_tu_va_do_chi_phi(LICH_SU_SU_KIEN, 102, CAC_VECTOR_MOI, CACHE, 0.95, 0.7)
lich_3, ty_le_3, huong_3 = buoc_tac_tu_va_do_chi_phi(LICH_SU_SU_KIEN, 103, CAC_VECTOR_MOI, CACHE, 0.95, 0.5)

print(len(LICH_SU_SU_KIEN))
print([e["request_id"] for e in LICH_SU_SU_KIEN])
print(ty_le_1, huong_1)
print(ty_le_3, huong_3)
```

```python title=test
assert lich_1 is LICH_SU_SU_KIEN, "lich_su_su_kien tra ve phai la CHINH danh sach da truyen vao (sua tai cho)"
assert len(LICH_SU_SU_KIEN) == 3, f"phai co DUNG 3 su kien sau 3 lan goi -- dang ra {len(LICH_SU_SU_KIEN)}"
assert [e["request_id"] for e in LICH_SU_SU_KIEN] == [101, 102, 103], f"thu tu request_id sai -- dang ra {[e['request_id'] for e in LICH_SU_SU_KIEN]}"
assert all(e["loai_su_kien"] == "HoanThanhRequestEvent" for e in LICH_SU_SU_KIEN), "moi su kien phai co loai_su_kien la HoanThanhRequestEvent"
assert ty_le_1 == 0.6, f"ty_le_1 phai la 0.6 -- dang ra {ty_le_1}"
assert huong_1 == {
    "nguyen_nhan": "khong co cache, prompt lap lai nhieu lan",
    "cach_xu_ly": "them semantic cache va prompt caching",
}, f"huong_1 sai -- dang ra {huong_1}"
assert huong_2 == huong_1, "cung du lieu, cung nguong -- huong_2 phai giong huong_1"
assert ty_le_3 == 0.6, f"ty_le_3 phai van la 0.6 (doi nguong khong doi ty le) -- dang ra {ty_le_3}"
assert huong_3 is None, f"nguong 0.5 (0.6 khong nho hon 0.5) phai cho huong_3 la None -- dang ra {huong_3}"

# bien: nguong_ty_le_hit_thap BANG TUYET DOI voi ty_le_hit (0.6) phai cho None, khong phai dict
lich_4, ty_le_4, huong_4 = buoc_tac_tu_va_do_chi_phi(LICH_SU_SU_KIEN, 104, CAC_VECTOR_MOI, CACHE, 0.95, 0.6)
assert ty_le_4 == 0.6, f"ty_le_4 phai van la 0.6 -- dang ra {ty_le_4}"
assert huong_4 is None, f"nguong BANG TUYET DOI ty_le_hit (0.6 < 0.6 la False) phai cho None -- dang ra {huong_4}"
assert len(LICH_SU_SU_KIEN) == 4, f"lich su phai co DUNG 4 su kien sau lan goi thu 4 -- dang ra {len(LICH_SU_SU_KIEN)}"

# kiem tra truc tiep tach khoi chuoi demo: cache rong-tuong-doi (toan MISS) phai cho ty_le 0.0 va huong_xu_ly la dict
lich_rieng = []
_, ty_le_mot_minh, huong_mot_minh = buoc_tac_tu_va_do_chi_phi(lich_rieng, 999, [[0.0, 0.0, 1.0]], [{"cau_hoi": "x", "vector": [1.0, 0.0, 0.0], "tra_loi": "y"}], 0.95, 0.1)
assert ty_le_mot_minh == 0.0, f"cau hoi khong khop cache nao phai cho ty le 0.0 -- dang ra {ty_le_mot_minh}"
assert huong_mot_minh is not None, "ty le 0.0 duoi nguong 0.1 phai kich hoat tra cuu su co"
```

:::hints
- kind: attention
  body: "Hai cho trong, cung mot ham buoc_tac_tu_va_do_chi_phi. Cho dau la GIA TRI GAN cho ty_le_hit -- goi lai HAM DO TI LE CACHE HIT da tai dung tu q8.6d bai 2. Cho hai (BEN TRONG nhanh if) la GIA TRI GAN cho huong_xu_ly -- goi lai HAM TRA CUU SU CO da tai dung tu q8.6d bai 4, voi DUNG trieu chung co dinh 'chi_phi_tang_bat_thuong'."
- kind: strategy
  body: "Cho dau: do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine) -- dung khuon da viet o q8.6d bai 2. Cho hai: tra_cuu_su_co(\"chi_phi_tang_bat_thuong\") -- trieu chung CO DINH, dung logic Chuong 39 rang cache hit thap gay tang chi phi."
- kind: one-line
  body: 'Cho dau la do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine), cho hai la tra_cuu_su_co("chi_phi_tang_bat_thuong").'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT do_ty_le_cache_hit(...) voi dung ba doi so (khong duoc tu viet lai vong lap dem hit); cho trong hai phai GOI THAT tra_cuu_su_co("chi_phi_tang_bat_thuong") (khong duoc chep san mot dict co dinh hay goi voi trieu chung khac)
  requireAst:
  - kind: uses-call, target: do_ty_le_cache_hit, min: 1
  - kind: uses-call, target: tra_cuu_su_co, min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, dist build that tai
  # packages/exec-python/dist/kiem-ast.js, trich CHINH XAC tu solution cua
  # file nay -- gom ca toolkit cache/su-co tai dung tu q8.6d) -- xac nhan
  # DUNG CHINH XAC (min VA min+1): do_ty_le_cache_hit=1, tra_cuu_su_co=1.
  # CA HAI ham nay CHI duoc goi DUNG 1 lan trong toan bo solution -- dung o
  # hai cho trong, khong noi nao khac (khong co demo goi lai rieng le, va
  # ban than hai ham nay khong tu goi lai chinh no).
  # Dien bua "True" vao ca hai cho trong ("ty_le_hit = True" -- se nem loi so
  # sanh dict/float o tier run, nhung rieng o tier static thi
  # do_ty_le_cache_hit=0; "huong_xu_ly = True" cho tra_cuu_su_co=0) -- CA HAI
  # luat CHAN DUNG (da CHAY THAT xac nhan qua kiemAst).
  #
  # GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho trong --
  # xac dinh ranh gioi TU CHINH khoi starter cua file nay, khong doan tay --
  # va CHAY THAT qua kiemAst() VA python3): dien
  # 'tra_cuu_su_co("chi_phi_tang_bat_thuong")' vao cho trong dau (dong
  # 'ty_le_hit = tra_cuu_su_co("chi_phi_tang_bat_thuong")') VA dien
  # 'do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine)' vao
  # cho trong hai (dong 'huong_xu_ly =
  # do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine)') --
  # da CHAY THAT qua kiemAst(): CA HAI con so (do_ty_le_cache_hit=1,
  # tra_cuu_su_co=1) tren TOAN BO solution DEU KHONG DOI (chi doi VI TRI) --
  # static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': dong "ty_le_hit =
  # tra_cuu_su_co(\"chi_phi_tang_bat_thuong\")" CHAY duoc (khong loi), gan
  # ty_le_hit LA MOT DICT; nhung dong ke tiep "if ty_le_hit <
  # nguong_ty_le_hit_thap:" so sanh MOT DICT voi MOT SO THUC bang toan tu
  # '<' -- da tu chay THAT qua python3, xac nhan no nem TypeError ("'<' not
  # supported between instances of 'dict' and 'float'") NGAY tai dong if,
  # truoc ca khi kip chay toi dong huong_xu_ly. Bi chan boi tier 'run', doc
  # lap voi static.
  # Da tu ra soat GOTCHA #6: "do_ty_le_cache_hit" va "tra_cuu_su_co" la hai
  # TEN HAM rieng biet, khong trung voi ham nao khac trong toan bo toolkit
  # da tai dung (tich_vo_huong, do_dai_vector, tuong_dong_cosine,
  # tim_trong_cache -- khong ten nao trung).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^3\\n\\[101, 102, 103\\]\\n0\\.6 \\{'nguyen_nhan': 'khong co cache, prompt lap lai nhieu lan', 'cach_xu_ly': 'them semantic cache va prompt caching'\\}\\n0\\.6 None\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`3` sự kiện ghi đúng thứ tự, `ty_le_hit = 0,6` không đổi, `huong_xu_ly` đổi
HẲN theo ngưỡng — MỘT hàm nối được cả sổ sự kiện, cả phép đo cache, cả bảng
tra cứu. Bài CUỐI CÙNG của `q8.6e` ráp CẢ BỐN bài này thành MỘT luồng xử lý
request đầu-cuối, đo trên một tải giả hỗn hợp.
::::

::::reflect{#nghi-lai}
`buoc_tac_tu_va_do_chi_phi` không phát minh phép đo MỚI nào —
`do_ty_le_cache_hit` VÀ `tra_cuu_su_co` đều LÀ những hàm đã kiểm chứng ở
`q8.6d`. Event sourcing ở đây cũng KHÔNG cần bộ máy đầy đủ của `q8.6d` bài
`1` (hàng đợi sự kiện sinh sự kiện mới, giới hạn vòng lặp) — MỘT
`.append()` là đủ, vì mọi request CHỈ sinh ra ĐÚNG MỘT sự kiện
`"HoanThanhRequestEvent"`, không có gì để phản ứng tiếp. Điều bài này
chứng minh được LÀ: một chỉ số đo được (`ty_le_hit`) có thể trực tiếp QUYẾT
ĐỊNH có cần tra cứu sự cố hay không — VÀ ranh giới đó (`<` nghiêm ngặt) LÀ
tường minh, không mập mờ ngay cả khi hai giá trị bằng nhau tuyệt đối.
::::

::::checkpoint{mastery=0.85}
::::
