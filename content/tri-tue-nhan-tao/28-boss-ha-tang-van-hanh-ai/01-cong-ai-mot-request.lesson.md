---
id: tri-tue-nhan-tao.boss-ha-tang-van-hanh-ai.cong-ai-mot-request
title: "q8.6e bài 1 — cổng AI xử lý MỘT request: xác thực + token bucket + định tuyến, ba kết cục"
summary: "xu_ly_mot_request_qua_cong(request, danh_sach_tai_node) rap lai xac_thuc + xu_ly_yeu_cau + least_loaded (ca ba tai dung nguyen van tu q8.6b bai 5, KHONG con ghi phi) thanh MOT ham tra ve DUNG MOT trong ba dict: {'ket_qua': 'tu_choi_sai_key'}, {'ket_qua': 'tu_choi_vuot_han_muc'}, hoac {'ket_qua': 'duoc_chap_nhan', 'node': ten_node}. Chay day 5 request qua CHUOI_REQUEST (BANG_NGUOI_DUNG/SUC_CHUA_CON_LAI/DANH_SACH_NODE tai dung tu q8.6b): 3 duoc_chap_nhan (gpu-0, gpu-1, gpu-2), 1 tu_choi_sai_key, 1 tu_choi_vuot_han_muc -- DANH_SACH_TAI_NODE cuoi [1,1,1,0], SUC_CHUA_CON_LAI cuoi {key-abc123:900, key-def456:4800, key-ghi789:100}."
locale: vi
track: tri-tue-nhan-tao
module: boss-ha-tang-van-hanh-ai
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.cong-ai-mot-request]
requires: [ai.boss-quan-sat-ai-day-du]
concepts: [ai.cong-ai-mot-request]
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
`q8.6d` "Đa tác tử và quan sát AI" đóng tại `5/5`. `T8.6` mở quest CUỐI CÙNG:
`q8.6e` "Hạ tầng & vận hành AI" — năm bài ráp TOÀN BỘ bốn quest trước thành
MỘT luồng xử lý request đầu-cuối. Bài `1` bắt đầu từ nơi mọi request bắt
đầu: cổng vào.
::::

::::explain{#rap_ba_manh_thanh_mot_cong}
`q8.6b` đã dạy BỐN việc một cổng AI làm: xác thực, giới hạn theo token, định
tuyến, ghi phí. Bài này lấy lại BA việc ĐẦU — xác thực (bài `3`), token
bucket (bài `2`), định tuyến least-loaded (bài `4`) — và bỏ việc ghi phí
(billing không cần cho luồng bài `q8.6e` này). Kết quả: một hàm xử lý ĐÚNG
MỘT request, luôn trả về MỘT trong BA kết cục:

```
xu_ly_mot_request_qua_cong(request, danh_sach_tai_node):
  nguoi_dung = xac_thuc(request["api_key"])
  neu nguoi_dung is None: tra ve {"ket_qua": "tu_choi_sai_key"}

  suc_chua_con_lai = SUC_CHUA_CON_LAI[request["api_key"]]
  (cho_qua, suc_chua_con_lai_moi) = xu_ly_yeu_cau(suc_chua_con_lai, request["so_token"])
  neu khong cho_qua: tra ve {"ket_qua": "tu_choi_vuot_han_muc"}
  cap nhat SUC_CHUA_CON_LAI cua api_key = suc_chua_con_lai_moi

  chi_so_node = least_loaded(danh_sach_tai_node)
  tang danh_sach_tai_node[chi_so_node] them 1

  tra ve {"ket_qua": "duoc_chap_nhan", "node": TEN cua node do}
```

Thứ tự VẪN cố định như `q8.6b`: xác thực trước token, token trước định
tuyến — đảo ngược sẽ định tuyến (tốn tài nguyên) cho một request đáng lẽ bị
từ chối. `xac_thuc`, `xu_ly_yeu_cau`, `least_loaded` được GỌI LẠI NGUYÊN VĂN
— bài này không viết lại logic nào của chúng, chỉ RÁP.
::::

::::example{#nam_request_qua_cong}
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


CHUOI_REQUEST = [
    {"api_key": "key-abc123", "so_token": 100},
    {"api_key": "key-xyz-invalid", "so_token": 50},
    {"api_key": "key-ghi789", "so_token": 250},
    {"api_key": "key-def456", "so_token": 200},
    {"api_key": "key-ghi789", "so_token": 100},
]

DANH_SACH_TAI_NODE = [0, 0, 0, 0]

KET_QUA = [xu_ly_mot_request_qua_cong(r, DANH_SACH_TAI_NODE) for r in CHUOI_REQUEST]

so_duoc_chap_nhan = sum(1 for k in KET_QUA if k["ket_qua"] == "duoc_chap_nhan")
so_tu_choi_sai_key = sum(1 for k in KET_QUA if k["ket_qua"] == "tu_choi_sai_key")
so_tu_choi_vuot_han_muc = sum(1 for k in KET_QUA if k["ket_qua"] == "tu_choi_vuot_han_muc")

print(KET_QUA)
print(DANH_SACH_TAI_NODE)
print(SUC_CHUA_CON_LAI)
print(so_duoc_chap_nhan, so_tu_choi_sai_key, so_tu_choi_vuot_han_muc)
```

```text title=readonly
[{'ket_qua': 'duoc_chap_nhan', 'node': 'gpu-0'}, {'ket_qua': 'tu_choi_sai_key'}, {'ket_qua': 'tu_choi_vuot_han_muc'}, {'ket_qua': 'duoc_chap_nhan', 'node': 'gpu-1'}, {'ket_qua': 'duoc_chap_nhan', 'node': 'gpu-2'}]
[1, 1, 1, 0]
{'key-abc123': 900, 'key-def456': 4800, 'key-ghi789': 100}
3 1 1
```

Năm request, TỪNG cái một: (1) `key-abc123` cần `100` — xô còn `1000`, đủ —
tải `[0,0,0,0]` đều nhau, `least_loaded` giữ chỉ số ĐẦU TIÊN, chọn `gpu-0`;
xô còn `900`. (2) `key-xyz-invalid` — không có trong `BANG_NGUOI_DUNG` —
`tu_choi_sai_key` NGAY, không chạm token hay định tuyến. (3) `key-ghi789`
cần `250` nhưng xô CHỈ còn `200` — `250 > 200` — `tu_choi_vuot_han_muc`, xô
GIỮ NGUYÊN `200`. (4) `key-def456` cần `200` — xô còn `5000`, đủ — tải hiện
tại `[1,0,0,0]`, `least_loaded` chọn `gpu-1` (tải `0`, nhẹ nhất); xô còn
`4800`. (5) `key-ghi789` cần `100` — xô VẪN còn `200` (bước `3` bị từ chối
không hề trừ) — đủ — tải `[1,1,0,0]`, chọn `gpu-2`; xô còn `100`. Tổng kết:
`3` được chấp nhận (bước `1,4,5`), `1` sai key (bước `2`), `1` vượt hạn mức
(bước `3`) — đúng `so_duoc_chap_nhan, so_tu_choi_sai_key,
so_tu_choi_vuot_han_muc = 3, 1, 1`.
::::

::::predict{#doan_request_bien_sau_chuoi commitOnce}
Ngay SAU khi chạy hết `CHUOI_REQUEST` ở ví dụ trên, `DANH_SACH_TAI_NODE` LÀ
`[1, 1, 1, 0]` và `SUC_CHUA_CON_LAI["key-ghi789"] = 100`. Xét việc gọi
THÊM MỘT request nữa, NGAY SAU đó (tiếp tục dùng CHÍNH `DANH_SACH_TAI_NODE`
đó, chưa reset): `xu_ly_mot_request_qua_cong({"api_key": "key-ghi789",
"so_token": 100}, DANH_SACH_TAI_NODE)`.

**Trước khi chạy thử**, bạn đoán: kết quả trả về LÀ gì?

:::opt{correct}
`{"ket_qua": "duoc_chap_nhan", "node": "gpu-3"}` — xô còn ĐÚNG `100`, request
cần ĐÚNG `100`, và `100 <= 100` đúng (`xu_ly_yeu_cau` dùng `<=`) nên được
chấp nhận; tải hiện tại `[1,1,1,0]` — CHỈ chỉ số `3` có tải NHỎ HƠN THỰC SỰ
tải ở chỉ số `0`, nên `least_loaded` chọn `3`, tức `"gpu-3"`
:::

:::opt
`{"ket_qua": "tu_choi_vuot_han_muc"}` — xô chỉ CÒN ĐÚNG bằng số token cần
dùng, không CÒN DƯ, nên phải bị từ chối để an toàn
::why
Gần đúng ở việc xô ĐANG ở đúng ranh giới (`suc_chua_con_lai` bằng CHÍNH số
token yêu cầu) — quan sát về vị trí biên đó đúng.

Chỗ lệch: điều kiện cho qua của `xu_ly_yeu_cau` LÀ `so_token_yeu_cau <=
suc_chua_con_lai`, và `100 <= 100` đúng — nó không đòi hỏi phải CÒN DƯ (`<`),
chỉ đòi hỏi ĐỦ (`<=`). Từ chối một request đúng bằng phần còn lại là áp
dụng một điều kiện chặt hơn so với điều kiện thật đã viết trong
`xu_ly_yeu_cau`.
::
:::

:::opt
`{"ket_qua": "duoc_chap_nhan", "node": "gpu-0"}` — vì `gpu-0` luôn là node
ĐẦU TIÊN được `least_loaded` chọn khi có nhiều node cùng "thấp"
::why
Gần đúng ở việc `least_loaded` THẬT SỰ ưu tiên chỉ số ĐẦU TIÊN — nhưng CHỈ
khi các tải đó BẰNG NHAU tuyệt đối (`q8.6b` bài `4`/`5` đã dạy rõ điều này).

Chỗ lệch: tải hiện tại LÀ `[1, 1, 1, 0]` — KHÔNG bằng nhau. Chỉ số `3` có
tải `0`, THẤP HƠN THỰC SỰ so với chỉ số `0` (tải `1`), nên vòng lặp của
`least_loaded` CẬP NHẬT `chi_so_nhe_nhat` từ `0` sang `3` ngay khi gặp nó
— kết quả LÀ `3`, không phải `0`.
::
:::
::::

::::code{#viet_xu_ly_mot_request_qua_cong}
Hoàn thiện `xu_ly_mot_request_qua_cong`: định tuyến bằng `least_loaded` tới
node ít tải nhất, rồi trả về kết quả CHẤP NHẬN kèm TÊN node đó.

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

    chi_so_node = ___                                        # least_loaded(danh_sach_tai_node)
    danh_sach_tai_node[chi_so_node] += 1

    return ___                                                # {"ket_qua": "duoc_chap_nhan", "node": DANH_SACH_NODE[chi_so_node]}


CHUOI_REQUEST = [
    {"api_key": "key-abc123", "so_token": 100},
    {"api_key": "key-xyz-invalid", "so_token": 50},
    {"api_key": "key-ghi789", "so_token": 250},
    {"api_key": "key-def456", "so_token": 200},
    {"api_key": "key-ghi789", "so_token": 100},
]

DANH_SACH_TAI_NODE = [0, 0, 0, 0]

KET_QUA = [xu_ly_mot_request_qua_cong(r, DANH_SACH_TAI_NODE) for r in CHUOI_REQUEST]

so_duoc_chap_nhan = sum(1 for k in KET_QUA if k["ket_qua"] == "duoc_chap_nhan")
so_tu_choi_sai_key = sum(1 for k in KET_QUA if k["ket_qua"] == "tu_choi_sai_key")
so_tu_choi_vuot_han_muc = sum(1 for k in KET_QUA if k["ket_qua"] == "tu_choi_vuot_han_muc")

print(KET_QUA)
print(DANH_SACH_TAI_NODE)
print(SUC_CHUA_CON_LAI)
print(so_duoc_chap_nhan, so_tu_choi_sai_key, so_tu_choi_vuot_han_muc)
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


CHUOI_REQUEST = [
    {"api_key": "key-abc123", "so_token": 100},
    {"api_key": "key-xyz-invalid", "so_token": 50},
    {"api_key": "key-ghi789", "so_token": 250},
    {"api_key": "key-def456", "so_token": 200},
    {"api_key": "key-ghi789", "so_token": 100},
]

DANH_SACH_TAI_NODE = [0, 0, 0, 0]

KET_QUA = [xu_ly_mot_request_qua_cong(r, DANH_SACH_TAI_NODE) for r in CHUOI_REQUEST]

so_duoc_chap_nhan = sum(1 for k in KET_QUA if k["ket_qua"] == "duoc_chap_nhan")
so_tu_choi_sai_key = sum(1 for k in KET_QUA if k["ket_qua"] == "tu_choi_sai_key")
so_tu_choi_vuot_han_muc = sum(1 for k in KET_QUA if k["ket_qua"] == "tu_choi_vuot_han_muc")

print(KET_QUA)
print(DANH_SACH_TAI_NODE)
print(SUC_CHUA_CON_LAI)
print(so_duoc_chap_nhan, so_tu_choi_sai_key, so_tu_choi_vuot_han_muc)
```

```python title=test
assert KET_QUA == [
    {"ket_qua": "duoc_chap_nhan", "node": "gpu-0"},
    {"ket_qua": "tu_choi_sai_key"},
    {"ket_qua": "tu_choi_vuot_han_muc"},
    {"ket_qua": "duoc_chap_nhan", "node": "gpu-1"},
    {"ket_qua": "duoc_chap_nhan", "node": "gpu-2"},
], f"KET_QUA sai -- dang ra {KET_QUA}"
assert DANH_SACH_TAI_NODE == [1, 1, 1, 0], f"DANH_SACH_TAI_NODE cuoi phai la [1, 1, 1, 0] -- dang ra {DANH_SACH_TAI_NODE}"
assert SUC_CHUA_CON_LAI == {"key-abc123": 900, "key-def456": 4800, "key-ghi789": 100}, f"SUC_CHUA_CON_LAI cuoi sai -- dang ra {SUC_CHUA_CON_LAI}"
assert (so_duoc_chap_nhan, so_tu_choi_sai_key, so_tu_choi_vuot_han_muc) == (3, 1, 1), f"bo dem sai -- dang ra {(so_duoc_chap_nhan, so_tu_choi_sai_key, so_tu_choi_vuot_han_muc)}"

# kiem tra truc tiep tung ket cuc, tach khoi chuoi demo
assert xu_ly_mot_request_qua_cong({"api_key": "khong-ton-tai", "so_token": 1}, [0, 0]) == {"ket_qua": "tu_choi_sai_key"}, "key la phai bi tu choi sai key NGAY"
assert xu_ly_mot_request_qua_cong({"api_key": "key-ghi789", "so_token": 101}, [0, 0, 0, 0]) == {"ket_qua": "tu_choi_vuot_han_muc"}, "key-ghi789 chi con 100 token (sau chuoi demo), yeu cau 101 phai bi tu choi"
assert SUC_CHUA_CON_LAI["key-ghi789"] == 100, f"tu choi vuot han muc KHONG DUOC tru xo -- dang ra {SUC_CHUA_CON_LAI['key-ghi789']}"

# bien: doi tai node THAT SU doi node duoc chon (khong phai tham so thua)
assert xu_ly_mot_request_qua_cong({"api_key": "key-ghi789", "so_token": 50}, [0, 0, 0, 0]) == {"ket_qua": "duoc_chap_nhan", "node": "gpu-0"}, "tai deu 0 phai chon gpu-0"
assert SUC_CHUA_CON_LAI["key-ghi789"] == 50, f"sau khi tru them 50, key-ghi789 phai con 50 -- dang ra {SUC_CHUA_CON_LAI['key-ghi789']}"
assert xu_ly_mot_request_qua_cong({"api_key": "key-ghi789", "so_token": 50}, [3, 3, 3, 3]) == {"ket_qua": "duoc_chap_nhan", "node": "gpu-0"}, "tai HOA tuyet doi van phai chon node DAU TIEN, khong phai 'khong xac dinh'"
```

:::hints
- kind: attention
  body: "Hai cho trong, cung mot ham xu_ly_mot_request_qua_cong, o BUOC SAU khi da qua ca xac thuc lan token bucket. Cho dau la GIA TRI GAN cho chi_so_node -- goi lai HAM DINH TUYEN da tai dung tu q8.6b, truyen danh_sach_tai_node. Cho hai la GIA TRI TRA VE CUOI CUNG -- mot dict co hai khoa, 'node' phai tra ve TEN node (dung DANH_SACH_NODE de tra tu chi_so_node vua tinh)."
- kind: strategy
  body: "Cho dau: least_loaded(danh_sach_tai_node) -- tim chi so node it tai nhat, dung nguyen ham da viet o q8.6b. Cho hai: {\"ket_qua\": \"duoc_chap_nhan\", \"node\": DANH_SACH_NODE[chi_so_node]} -- tra ve dict co dung hai khoa, 'node' la TEN (chuoi), khong phai CHI SO (so nguyen)."
- kind: one-line
  body: "Cho dau la least_loaded(danh_sach_tai_node), cho hai la {\"ket_qua\": \"duoc_chap_nhan\", \"node\": DANH_SACH_NODE[chi_so_node]}."
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI HAM least_loaded(danh_sach_tai_node) (khong duoc tu viet lai logic tim min); cho trong hai phai la mot DICT tra ve dung "ket_qua":"duoc_chap_nhan" VA "node": DANH_SACH_NODE[chi_so_node] (doc lai chi_so_node vua tinh, khong duoc tra ve thang chi_so_node hay mot gia tri co dinh)
  requireAst:
  - kind: uses-call, target: "least_loaded", min: 1
  - kind: uses-name, target: chi_so_node, min: 2
  # Da CHAY THAT qua kiemAst() (node + pyodide, dist build that tai
  # packages/exec-python/dist/kiem-ast.js, trich CHINH XAC tu solution cua
  # file nay) -- xac nhan DUNG CHINH XAC (min VA min+1):
  #   "least_loaded"=1: DUY NHAT o cho trong dau (dinh nghia ham least_loaded
  #   khong tu goi lai chinh no).
  #   uses-name(chi_so_node)=2: 1 lan o dong "danh_sach_tai_node[chi_so_node]
  #   += 1" (da cho san, mot Load), CONG 1 lan o cho trong hai (dong
  #   "DANH_SACH_NODE[chi_so_node]", cung mot Load). Gan "chi_so_node = ..."
  #   la Store, khong duoc uses-name dem.
  # Dien bua "True" vao CA HAI cho trong ("chi_so_node = True" va "return
  # True") cho "least_loaded"=0 (duoi nguong 1, mat lan goi duy nhat) VA
  # uses-name(chi_so_node)=1 (duoi nguong 2, mat lan doc o cho trong hai) --
  # CA HAI luat CHAN DUNG (da CHAY THAT xac nhan qua kiemAst).
  #
  # GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho trong --
  # xac dinh ranh gioi TU CHINH khoi starter cua file nay, khong doan tay --
  # va CHAY THAT qua kiemAst() VA python3): dien
  # '{"ket_qua": "duoc_chap_nhan", "node": DANH_SACH_NODE[chi_so_node]}' vao
  # cho trong dau (dong "chi_so_node = {\"ket_qua\": \"duoc_chap_nhan\",
  # \"node\": DANH_SACH_NODE[chi_so_node]}") VA dien
  # "least_loaded(danh_sach_tai_node)" vao cho trong hai (dong "return
  # least_loaded(danh_sach_tai_node)") -- da CHAY THAT qua kiemAst(): CA HAI
  # con so ("least_loaded"=1, uses-name(chi_so_node)=2) tren TOAN BO solution
  # DEU KHONG DOI (chi doi VI TRI) -- static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': dong dau tien sau hoan doi ("chi_so_node
  # = {...\"node\": DANH_SACH_NODE[chi_so_node]}") DOC bien "chi_so_node" o VE
  # PHAI TRUOC KHI no duoc GAN (chi_so_node la bien LOCAL cua ham, do CHINH
  # dong nay tao ra) -- da tu chay THAT qua python3, xac nhan no nem
  # UnboundLocalError ("cannot access local variable 'chi_so_node' where it
  # is not associated with a value") NGAY o request DAU TIEN duoc chap nhan
  # cua CHUOI_REQUEST (key-abc123, request dau tien) -- bi chan boi tier
  # 'run', doc lap voi static.
  # Da tu ra soat GOTCHA #6: "chi_so_node" la ten bien CUC BO DUY NHAT trong
  # pham vi xu_ly_mot_request_qua_cong, khong trung voi bat ky bien nao khac
  # (nguoi_dung, suc_chua_con_lai, cho_qua, suc_chua_con_lai_moi deu la ten
  # KHAC han) -- khong co rui ro nham lan tinh co ve HINH DANG.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[\\{'ket_qua': 'duoc_chap_nhan', 'node': 'gpu-0'\\}, \\{'ket_qua': 'tu_choi_sai_key'\\}, \\{'ket_qua': 'tu_choi_vuot_han_muc'\\}, \\{'ket_qua': 'duoc_chap_nhan', 'node': 'gpu-1'\\}, \\{'ket_qua': 'duoc_chap_nhan', 'node': 'gpu-2'\\}\\]\\n\\[1, 1, 1, 0\\]\\n\\{'key-abc123': 900, 'key-def456': 4800, 'key-ghi789': 100\\}\\n3 1 1\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`3` được chấp nhận, `1` sai key, `1` vượt hạn mức — MỘT hàm, BA kết cục,
đúng như một cổng AI thật xử lý MỖI request. Bài tiếp theo hỏi câu tiếp
theo: khi NHIỀU request đã qua được cổng này CÙNG lúc, gộp lô có tăng
thông lượng không?
::::

::::reflect{#nghi-lai}
`xu_ly_mot_request_qua_cong` không dạy thuật toán MỚI nào — `xac_thuc`,
`xu_ly_yeu_cau`, `least_loaded` đều LÀ những hàm đã kiểm chứng riêng ở
`q8.6b`. Điều bài này làm được LÀ đặt chúng vào ĐÚNG một thứ tự VÀ thu gọn
kết quả về đúng BA khả năng — một hợp đồng (contract) rõ ràng mà mọi bài
sau của `q8.6e` sẽ dựa vào: bất kỳ request nào đi qua cổng CHỈ có thể LÀ
một trong ba kết cục đó, không hơn không kém. Dữ liệu module-level
(`BANG_NGUOI_DUNG`, `SUC_CHUA_CON_LAI`, `DANH_SACH_NODE`) VẪN LÀ trạng thái
CHIA SẺ, tồn tại qua nhiều lần gọi — đúng tinh thần một cổng AI THẬT phục vụ
liên tục, không phải khởi động lại cho mỗi request.
::::

::::checkpoint{mastery=0.8}
::::
