---
id: tri-tue-nhan-tao.boss-ha-tang-van-hanh-ai.hang-doi-gop-lo-sau-cong
title: "q8.6e bài 2 — hàng đợi suy luận có gộp lô: chỉ request ĐÃ qua cổng mới được đo thông lượng"
summary: "do_thong_luong_hang_doi_da_qua_cong(danh_sach_request, danh_sach_tai_node, chi_phi_khoi_dong, kich_thuoc_lo) chay MOI request qua xu_ly_mot_request_qua_cong (bai 1, tai dung nguyen van), LOC chi giu don_vi_tinh cua request duoc_chap_nhan, roi do thong luong tuan tu/gop lo bang tong_thoi_gian_tuan_tu/tong_thoi_gian_gop_lo/thong_luong (tai dung nguyen van tu q8.6a bai 5). Tren 5 request (2 bi tu choi KHONG co truong don_vi_tinh, chi_phi_khoi_dong=200, kich_thuoc_lo=3): don_vi_tinh_duoc_nhan=[10, 20, 15], tong_tuan_tu=645, tong_gop_lo=220, thong_luong_tuan_tu~0,004651, thong_luong_gop_lo~0,013636, ti le tang toc 645/220~2,93 lan -- gop_lo_thang=True."
locale: vi
track: tri-tue-nhan-tao
module: boss-ha-tang-van-hanh-ai
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.hang-doi-gop-lo-sau-cong]
requires: [ai.cong-ai-mot-request]
concepts: [ai.hang-doi-gop-lo-sau-cong]
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
Bài `1` cho MỖI request MỘT trong ba kết cục. Nhưng cổng AI không phục vụ
từng request đơn lẻ mãi — nó CHỒNG nhiều request `duoc_chap_nhan` lại thành
MỘT hàng đợi suy luận, đúng `Chương 39.2` đã dạy ở `q8.6a`. Câu hỏi bài này:
gộp lô có còn tăng thông lượng khi hàng đợi đó đã ĐI QUA cổng trước?
::::

::::explain{#chi_do_thong_luong_tren_request_da_qua_cong}
`q8.6a` bài `5` đã đo thông lượng của MỘT hàng đợi có sẵn. Bài này thêm một
bước TRƯỚC đó: hàng đợi không đến từ hư không — nó LÀ tập hợp CÁC request
ĐÃ ĐƯỢC cổng (bài `1`) chấp nhận. Request bị từ chối (sai key hay vượt hạn
mức) không bao giờ tốn một đơn vị tính toán GPU nào — chúng phải bị LOẠI
KHỎI phép đo thông lượng, không phải vì "công bằng" mà vì chúng chưa từng
chạm tới tầng suy luận.

```
do_thong_luong_hang_doi_da_qua_cong(danh_sach_request, danh_sach_tai_node,
                                     chi_phi_khoi_dong, kich_thuoc_lo):
  ket_qua_cong = [xu_ly_mot_request_qua_cong(r, danh_sach_tai_node) CHO MOI r]   # bai 1
  don_vi_tinh_duoc_nhan = [r["don_vi_tinh"] CHO MOI r CO ket_qua == "duoc_chap_nhan"]
  tong_tuan_tu = tong_thoi_gian_tuan_tu(don_vi_tinh_duoc_nhan, chi_phi_khoi_dong)   # q8.6a
  tong_gop_lo = tong_thoi_gian_gop_lo(don_vi_tinh_duoc_nhan, chi_phi_khoi_dong, kich_thuoc_lo)  # q8.6a
  thong_luong_tuan_tu = thong_luong(len(don_vi_tinh_duoc_nhan), tong_tuan_tu)      # q8.6a
  thong_luong_gop_lo = thong_luong(len(don_vi_tinh_duoc_nhan), tong_gop_lo)        # q8.6a
```

Điểm mấu chốt nằm ở phép LỌC: `[r["don_vi_tinh"] for r, kq in zip(...) if
kq["ket_qua"] == "duoc_chap_nhan"]` — điều kiện `if` được Python đánh giá
TRƯỚC khi đọc `r["don_vi_tinh"]`. Một request bị từ chối KHÔNG CẦN mang
trường `don_vi_tinh"` nào cả — nó không bao giờ bị đọc tới.
::::

::::example{#do_thong_luong_tren_nam_request}
```python title=readonly
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


CHUOI_REQUEST_2 = [
    {"api_key": "key-abc123", "so_token": 100, "don_vi_tinh": 10},
    {"api_key": "key-xyz-invalid", "so_token": 50},
    {"api_key": "key-ghi789", "so_token": 250},
    {"api_key": "key-def456", "so_token": 200, "don_vi_tinh": 20},
    {"api_key": "key-ghi789", "so_token": 100, "don_vi_tinh": 15},
]

DANH_SACH_TAI_NODE_2 = [0, 0, 0, 0]
CHI_PHI_KHOI_DONG = 200
KICH_THUOC_LO = 3

(ket_qua_cong, don_vi_tinh_duoc_nhan, tong_tuan_tu, tong_gop_lo,
 thong_luong_tuan_tu, thong_luong_gop_lo) = do_thong_luong_hang_doi_da_qua_cong(
    CHUOI_REQUEST_2, DANH_SACH_TAI_NODE_2, CHI_PHI_KHOI_DONG, KICH_THUOC_LO
)

gop_lo_thang = thong_luong_gop_lo > thong_luong_tuan_tu

print(don_vi_tinh_duoc_nhan)
print(tong_tuan_tu)
print(tong_gop_lo)
print(round(thong_luong_tuan_tu, 6))
print(round(thong_luong_gop_lo, 6))
print(round(tong_tuan_tu / tong_gop_lo, 2))
print(gop_lo_thang)
```

```text title=readonly
[10, 20, 15]
645
220
0.004651
0.013636
2.93
True
```

Hai request bị từ chối (`key-xyz-invalid` — sai key; `key-ghi789` cần `250`
nhưng xô chỉ có `200` — vượt hạn mức) hoàn toàn KHÔNG có trường
`"don_vi_tinh"` trong dict của chúng — vẫn KHÔNG lỗi, vì điều kiện lọc chặn
chúng TRƯỚC khi `r["don_vi_tinh"]` được đọc. Ba request còn lại được chấp
nhận — `don_vi_tinh_duoc_nhan = [10, 20, 15]`. Tuần tự: `(200+10) +
(200+20) + (200+15) = 210+220+215 = 645`. Gộp lô với `kich_thuoc_lo=3` (cả
ba vào MỘT lô): `200 + max(10,20,15) = 200+20 = 220`. Thông lượng tuần tự
`3/645 ≈ 0,004651`; gộp lô `3/220 ≈ 0,013636` — CAO hơn, tỉ lệ tăng tốc
`645/220 ≈ 2,93` lần.
::::

::::predict{#doan_khong_co_don_vi_tinh_co_loi_khong commitOnce}
Xét đúng `CHUOI_REQUEST_2` ở ví dụ trên. Hai request bị từ chối (vị trí thứ
`2` và thứ `3`) hoàn toàn KHÔNG có khoá `"don_vi_tinh"` nào trong dict của
chúng.

**Trước khi chạy thử**, bạn đoán: `do_thong_luong_hang_doi_da_qua_cong` có
ném lỗi `KeyError` khi chạy trên `CHUOI_REQUEST_2` không?

:::opt{correct}
KHÔNG lỗi gì cả — trong `[r["don_vi_tinh"] for r, kq in zip(...) if
kq["ket_qua"] == "duoc_chap_nhan"]`, điều kiện `if` được đánh giá TRƯỚC cho
MỖI phần tử; hai request bị từ chối có `kq["ket_qua"] != "duoc_chap_nhan"`
nên KHÔNG BAO GIỜ đi tới bước đọc `r["don_vi_tinh"]`
:::

:::opt
Có, ném `KeyError` ngay tại request thứ `2` (`key-xyz-invalid`) — vì một
`list comprehension` luôn tính TRƯỚC biểu thức bên trái (`r["don_vi_tinh"]`)
cho MỌI phần tử, rồi mới áp điều kiện `if` để lọc kết quả
::why
Gần đúng ở việc CÓ những cách viết Python (như dùng `map` rồi lọc SAU) mà
thứ tự đó ĐÚNG là "tính trước, lọc sau" — quan sát đó không sai cho MỌI
trường hợp.

Chỗ lệch: cú pháp `[bieu_thuc for x in day if dieu_kien]` của Python đánh
giá `dieu_kien` TRƯỚC, và CHỈ khi `dieu_kien` đúng mới tính `bieu_thuc` cho
phần tử đó — đây là hành vi CHUẨN của mọi `list comprehension` có mệnh đề
`if`, không phải một trường hợp đặc biệt của bài này. Vì vậy `r["don_vi_tinh"]`
không bao giờ được gọi trên hai request bị từ chối.
::
:::

:::opt
Không lỗi, vì Python tự động coi `"don_vi_tinh"` bị thiếu LÀ `0` cho những
dict không khai báo khoá đó
::why
Gần đúng ở việc KẾT QUẢ cuối cùng THẬT SỰ không hề bị ảnh hưởng bởi việc
thiếu khoá này — quan sát về hệ quả cuối cùng không sai.

Chỗ lệch: Python KHÔNG có cơ chế "mặc định ngầm" nào cho `dict["khoa"]` khi
`"khoa"` không tồn tại — truy cập kiểu đó (không qua `.get`) LUÔN ném
`KeyError` nếu khoá thật sự bị đọc. Lý do không lỗi ở đây không phải vì có
giá trị mặc định `0`, mà vì `r["don_vi_tinh"]` CHƯA BAO GIỜ được đọc trên
hai request đó — do điều kiện `if` đã chặn TRƯỚC.
::
:::
::::

::::code{#viet_do_thong_luong_hang_doi_da_qua_cong}
Hoàn thiện `do_thong_luong_hang_doi_da_qua_cong`: gọi ĐÚNG hàm gộp lô (`q8.6a`)
để tính tổng thời gian, rồi gọi ĐÚNG hàm thông lượng (`q8.6a`) để tính thông
lượng gộp lô.

```python title=starter
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
    tong_gop_lo = ___                                   # tong_thoi_gian_gop_lo(don_vi_tinh_duoc_nhan, chi_phi_khoi_dong, kich_thuoc_lo)
    thong_luong_tuan_tu = thong_luong(len(don_vi_tinh_duoc_nhan), tong_tuan_tu)
    thong_luong_gop_lo = ___                            # thong_luong(len(don_vi_tinh_duoc_nhan), tong_gop_lo)
    return ket_qua_cong, don_vi_tinh_duoc_nhan, tong_tuan_tu, tong_gop_lo, thong_luong_tuan_tu, thong_luong_gop_lo


CHUOI_REQUEST_2 = [
    {"api_key": "key-abc123", "so_token": 100, "don_vi_tinh": 10},
    {"api_key": "key-xyz-invalid", "so_token": 50},
    {"api_key": "key-ghi789", "so_token": 250},
    {"api_key": "key-def456", "so_token": 200, "don_vi_tinh": 20},
    {"api_key": "key-ghi789", "so_token": 100, "don_vi_tinh": 15},
]

DANH_SACH_TAI_NODE_2 = [0, 0, 0, 0]
CHI_PHI_KHOI_DONG = 200
KICH_THUOC_LO = 3

(ket_qua_cong, don_vi_tinh_duoc_nhan, tong_tuan_tu, tong_gop_lo,
 thong_luong_tuan_tu, thong_luong_gop_lo) = do_thong_luong_hang_doi_da_qua_cong(
    CHUOI_REQUEST_2, DANH_SACH_TAI_NODE_2, CHI_PHI_KHOI_DONG, KICH_THUOC_LO
)

gop_lo_thang = thong_luong_gop_lo > thong_luong_tuan_tu

print(don_vi_tinh_duoc_nhan)
print(tong_tuan_tu)
print(tong_gop_lo)
print(round(thong_luong_tuan_tu, 6))
print(round(thong_luong_gop_lo, 6))
print(round(tong_tuan_tu / tong_gop_lo, 2))
print(gop_lo_thang)
```

```python title=solution
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


CHUOI_REQUEST_2 = [
    {"api_key": "key-abc123", "so_token": 100, "don_vi_tinh": 10},
    {"api_key": "key-xyz-invalid", "so_token": 50},
    {"api_key": "key-ghi789", "so_token": 250},
    {"api_key": "key-def456", "so_token": 200, "don_vi_tinh": 20},
    {"api_key": "key-ghi789", "so_token": 100, "don_vi_tinh": 15},
]

DANH_SACH_TAI_NODE_2 = [0, 0, 0, 0]
CHI_PHI_KHOI_DONG = 200
KICH_THUOC_LO = 3

(ket_qua_cong, don_vi_tinh_duoc_nhan, tong_tuan_tu, tong_gop_lo,
 thong_luong_tuan_tu, thong_luong_gop_lo) = do_thong_luong_hang_doi_da_qua_cong(
    CHUOI_REQUEST_2, DANH_SACH_TAI_NODE_2, CHI_PHI_KHOI_DONG, KICH_THUOC_LO
)

gop_lo_thang = thong_luong_gop_lo > thong_luong_tuan_tu

print(don_vi_tinh_duoc_nhan)
print(tong_tuan_tu)
print(tong_gop_lo)
print(round(thong_luong_tuan_tu, 6))
print(round(thong_luong_gop_lo, 6))
print(round(tong_tuan_tu / tong_gop_lo, 2))
print(gop_lo_thang)
```

```python title=test
assert don_vi_tinh_duoc_nhan == [10, 20, 15], f"don_vi_tinh_duoc_nhan phai la [10, 20, 15] -- dang ra {don_vi_tinh_duoc_nhan}"
assert tong_tuan_tu == 645, f"tong_tuan_tu phai la 645 -- dang ra {tong_tuan_tu}"
assert tong_gop_lo == 220, f"tong_gop_lo phai la 220 -- dang ra {tong_gop_lo}"
assert round(thong_luong_tuan_tu, 6) == round(3 / 645, 6), f"thong_luong_tuan_tu sai -- dang ra {thong_luong_tuan_tu}"
assert round(thong_luong_gop_lo, 6) == round(3 / 220, 6), f"thong_luong_gop_lo sai -- dang ra {thong_luong_gop_lo}"
assert gop_lo_thang is True, f"gop lo PHAI dat thong luong cao hon tuan tu -- dang ra {gop_lo_thang}"
assert round(tong_tuan_tu / tong_gop_lo, 2) == 2.93, f"ti le tang toc phai la 2,93 -- dang ra {round(tong_tuan_tu / tong_gop_lo, 2)}"
assert len(ket_qua_cong) == 5, f"ket_qua_cong phai giu DUNG 5 phan tu (dung so request dau vao) -- dang ra {len(ket_qua_cong)}"
assert sum(1 for k in ket_qua_cong if k["ket_qua"] == "duoc_chap_nhan") == 3, "phai co DUNG 3 request duoc chap nhan trong CHUOI_REQUEST_2"

# bien: kich_thuoc_lo THAT SU rang buoc ket qua (khong phai tham so thua)
# -- dat lai xo token TRUOC moi lan goi bien the: CHUOI_REQUEST_2 dung LAI
# CUNG ba api_key, va SUC_CHUA_CON_LAI la trang thai CHIA SE cap module --
# khong dat lai thi lan goi THU HAI se thay xo cua key-ghi789 da can bot
# tu lan goi THU NHAT, lam tap duoc_chap_nhan tu nhien DOI, khong phai vi
# kich_thuoc_lo.
SUC_CHUA_CON_LAI["key-abc123"] = 1000
SUC_CHUA_CON_LAI["key-def456"] = 5000
SUC_CHUA_CON_LAI["key-ghi789"] = 200
_, _, tt1, tg1, _, _ = do_thong_luong_hang_doi_da_qua_cong(CHUOI_REQUEST_2, [0, 0, 0, 0], 200, 1)
assert tg1 == tt1, f"kich_thuoc_lo=1 phai cho KET QUA GIONG HET tuan tu (khong loi ich gop lo) -- gop_lo={tg1}, tuan_tu={tt1}"
SUC_CHUA_CON_LAI["key-abc123"] = 1000
SUC_CHUA_CON_LAI["key-def456"] = 5000
SUC_CHUA_CON_LAI["key-ghi789"] = 200
_, _, tt2, tg2, _, _ = do_thong_luong_hang_doi_da_qua_cong(CHUOI_REQUEST_2, [0, 0, 0, 0], 200, 2)
assert tg2 == 435, f"kich_thuoc_lo=2 phai cho tong_gop_lo la 435 -- dang ra {tg2}"
assert tg2 != tong_gop_lo, "doi kich_thuoc_lo tu 3 sang 2 phai doi tong_gop_lo"
```

:::hints
- kind: attention
  body: "Hai cho trong, cung mot ham do_thong_luong_hang_doi_da_qua_cong. Cho dau la GIA TRI GAN cho tong_gop_lo -- goi lai HAM GOP LO da tai dung tu q8.6a, dung DUNG ba doi so (don_vi_tinh_duoc_nhan, chi_phi_khoi_dong, kich_thuoc_lo). Cho hai la GIA TRI GAN cho thong_luong_gop_lo -- goi lai HAM THONG LUONG da tai dung tu q8.6a, tren so luong request VA tong_gop_lo (KHONG PHAI tong_tuan_tu)."
- kind: strategy
  body: "Cho dau: tong_thoi_gian_gop_lo(don_vi_tinh_duoc_nhan, chi_phi_khoi_dong, kich_thuoc_lo) -- dung khuon da viet o q8.6a bai 3/5. Cho hai: thong_luong(len(don_vi_tinh_duoc_nhan), tong_gop_lo) -- so request CHIA tong thoi gian GOP LO vua tinh o dong TRUOC do."
- kind: one-line
  body: "Cho dau la tong_thoi_gian_gop_lo(don_vi_tinh_duoc_nhan, chi_phi_khoi_dong, kich_thuoc_lo), cho hai la thong_luong(len(don_vi_tinh_duoc_nhan), tong_gop_lo)."
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT tong_thoi_gian_gop_lo(don_vi_tinh_duoc_nhan, chi_phi_khoi_dong, kich_thuoc_lo) (khong duoc tu viet lai vong lap gop lo); cho trong hai phai GOI THAT thong_luong(len(don_vi_tinh_duoc_nhan), tong_gop_lo) (khong duoc tra ve thang mot phep chia tu viet, va PHAI dung tong_gop_lo chu khong phai tong_tuan_tu)
  requireAst:
  - kind: uses-call, target: "tong_thoi_gian_gop_lo", min: 1
  - kind: uses-call, target: "thong_luong", min: 2
  # Da CHAY THAT qua kiemAst() (node + pyodide, dist build that tai
  # packages/exec-python/dist/kiem-ast.js, trich CHINH XAC tu solution cua
  # file nay -- gom ca toolkit cong AI tai dung tu bai 1) -- xac nhan DUNG
  # CHINH XAC (min VA min+1): "tong_thoi_gian_gop_lo"=1, "thong_luong"=2.
  #   "tong_thoi_gian_gop_lo"=1: DUY NHAT o cho trong dau (dinh nghia ham
  #   khong tu goi lai chinh no).
  #   "thong_luong"=2: 1 lan o dong "thong_luong_tuan_tu = thong_luong(...)"
  #   (DA CHO SAN, khong blank), CONG 1 lan o cho trong hai. Ten
  #   "thong_luong_tuan_tu"/"thong_luong_gop_lo" la BIEN, khac han ten HAM
  #   "thong_luong" -- uses-call so khop CHINH XAC ten ham qua func.id, khong
  #   nham voi ten bien dai hon.
  # Dien bua "True" vao CA HAI cho trong ("tong_gop_lo = True" va
  # "thong_luong_gop_lo = True") cho "tong_thoi_gian_gop_lo"=0 (duoi nguong
  # 1) VA "thong_luong"=1 (duoi nguong 2, chi con lan goi cho thong_luong_tuan_tu)
  # -- CA HAI luat CHAN DUNG (da CHAY THAT xac nhan qua kiemAst).
  #
  # GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho trong --
  # xac dinh ranh gioi TU CHINH khoi starter, khong doan tay -- va CHAY THAT
  # qua kiemAst() VA python3): dien "thong_luong(len(don_vi_tinh_duoc_nhan),
  # tong_gop_lo)" vao cho trong dau (dong "tong_gop_lo =
  # thong_luong(len(don_vi_tinh_duoc_nhan), tong_gop_lo)") VA dien
  # "tong_thoi_gian_gop_lo(don_vi_tinh_duoc_nhan, chi_phi_khoi_dong,
  # kich_thuoc_lo)" vao cho trong hai (dong "thong_luong_gop_lo =
  # tong_thoi_gian_gop_lo(don_vi_tinh_duoc_nhan, chi_phi_khoi_dong,
  # kich_thuoc_lo)") -- da CHAY THAT qua kiemAst(): CA HAI con so
  # ("tong_thoi_gian_gop_lo"=1, "thong_luong"=2) tren TOAN BO solution DEU
  # KHONG DOI (chi doi VI TRI) -- static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': dong dau tien sau hoan doi ("tong_gop_lo
  # = thong_luong(len(don_vi_tinh_duoc_nhan), tong_gop_lo)") DOC bien
  # "tong_gop_lo" o VE PHAI TRUOC KHI no duoc GAN (tong_gop_lo la bien LOCAL
  # cua ham, do CHINH dong nay tao ra) -- da tu chay THAT qua python3, xac
  # nhan no nem UnboundLocalError ("cannot access local variable
  # 'tong_gop_lo' where it is not associated with a value") NGAY khi
  # do_thong_luong_hang_doi_da_qua_cong duoc goi -- bi chan boi tier 'run',
  # doc lap voi static.
  # Da tu ra soat GOTCHA #6: "tong_thoi_gian_gop_lo" (ten HAM) khong trung
  # voi "tong_gop_lo" (ten BIEN) -- hai dinh danh KHAC nhau ve chu, uses-call
  # chi so khop CHINH XAC "tong_thoi_gian_gop_lo". "thong_luong" (ten HAM)
  # khong trung voi "thong_luong_tuan_tu"/"thong_luong_gop_lo" (ten BIEN) --
  # ba dinh danh nay la ba chuoi KHAC NHAU, khong co rui ro nham lan.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[10, 20, 15\\]\\n645\\n220\\n0\\.004651\\n0\\.013636\\n2\\.93\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`2,93` lần nhanh hơn — trên CÙNG một cổng, CÙNG một chuỗi request, chỉ đổi
chiến lược xử lý PHẦN đã qua được cổng. Bài tiếp theo chuyển sang câu hỏi
khác hẳn: khi request `duoc_chap_nhan` cần TRUY XUẤT tài liệu, làm sao đảm
bảo khách hàng A không bao giờ thấy dữ liệu của khách hàng B?
::::

::::reflect{#nghi-lai}
`do_thong_luong_hang_doi_da_qua_cong` không phát minh công thức đo lường
MỚI nào — `tong_thoi_gian_tuan_tu`, `tong_thoi_gian_gop_lo`, `thong_luong`
đều LÀ những hàm đã kiểm chứng ở `q8.6a`. Cái RÁP LẠI này chứng minh được
LÀ: một bước LỌC đơn giản (chỉ giữ request đã được CỔNG chấp nhận) đủ để
nối hai tầng hạ tầng hoàn toàn độc lập — `q8.6b` (gateway) và `q8.6a`
(inference queue) — mà không cần viết lại MỘT dòng logic nào của một
trong hai bên. `2,93` lần tăng tốc đo được ở đây nhỏ hơn `10` lần đã thấy ở
`q8.6a` bài `5`, đúng vì workload ở đây KHÔNG đều nhau (`10, 20, 15`, không
phải toàn `10`) — chi phí khởi động không còn chiếm áp đảo thời gian tuần tự
như trước.
::::

::::checkpoint{mastery=0.82}
::::
