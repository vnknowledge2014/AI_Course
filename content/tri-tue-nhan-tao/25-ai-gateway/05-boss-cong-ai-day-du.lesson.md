---
id: tri-tue-nhan-tao.ai-gateway.boss-cong-ai-day-du
title: "BOSS quý — cổng AI đầy đủ: xác thực + token bucket + định tuyến + ghi phí, đóng q8.6b tại 5/5"
summary: "xu_ly_cong_ai(api_key, so_token_yeu_cau, danh_sach_tai_node) rap CA BON bai: (a) xac_thuc(api_key) (bai 3), tra ve 'tu_choi_sai_key' NGAY neu None; (b) xu_ly_yeu_cau tren xo rieng cua nguoi dung do (bai 2, doc/ghi SUC_CHUA_CON_LAI[api_key]), tra ve 'tu_choi_het_token' neu khong du; (c) neu qua ca hai, least_loaded(danh_sach_tai_node) (bai 4) chon node nhe nhat, TANG tai node do len 1; (d) cong don so token vao SO_GHI_PHI[api_key] (billing). Chay day 8 request qua chay_day_yeu_cau: 4 PHUC VU (dinh tuyen toi gpu-0/gpu-1/gpu-2/gpu-3, moi node DUNG 1 request nho least-loaded), 2 tu_choi_sai_key, 2 tu_choi_het_token -- SO_GHI_PHI cuoi cung la {key-abc123: 950, key-def456: 200, key-ghi789: 100}. Dong q8.6b tai 5/5."
locale: vi
track: tri-tue-nhan-tao
module: ai-gateway
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-cong-ai-day-du]
requires: [ai.can-bang-tai-round-robin-va-least-loaded]
concepts: [ai.boss-cong-ai-day-du]
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
Bốn bài: đếm đúng theo token (không phải request), xô token thực thi giới
hạn, xác thực trước tiên, định tuyến biết chọn node nhẹ. BOSS quý này KHÔNG
thêm khái niệm mới — nó RÁP LẠI cả bốn thành MỘT hàm duy nhất: một cổng AI
đầy đủ, đúng như `Chương 39.3` mô tả.
::::

::::explain{#rap_bon_manh_thanh_mot_cong}
Một `AI Gateway` thật, khi một request tới, làm đúng BỐN việc theo MỘT thứ
tự cố định:

1. **Xác thực** (bài `3`) — key sai bị từ chối NGAY, trước mọi bước khác.
2. **Giới hạn theo token** (bài `2`) — nếu key hợp lệ, kiểm tra xô token
   RIÊNG của người dùng đó; không đủ thì từ chối, GIỮ NGUYÊN xô.
3. **Định tuyến** (bài `4`) — nếu qua cả hai bước trên, chọn node GPU ÍT
   TẢI NHẤT (least-loaded) VÀ tăng tải của node đó thêm `1` (request mới
   vừa được giao cho nó).
4. **Ghi phí** (billing) — cộng dồn số token VỪA DÙNG vào một "sổ ghi phí",
   theo TỪNG người dùng.

```
xu_ly_cong_ai(api_key, so_token_yeu_cau, danh_sach_tai_node):
  nguoi_dung = xac_thuc(api_key)
  neu nguoi_dung is None: tra ve "tu_choi_sai_key"

  (cho_qua, suc_chua_moi) = xu_ly_yeu_cau(xo cua api_key, so_token_yeu_cau)
  neu khong cho_qua: tra ve "tu_choi_het_token"
  cap nhat xo cua api_key = suc_chua_moi

  chi_so_node = least_loaded(danh_sach_tai_node)
  tang danh_sach_tai_node[chi_so_node] them 1
  cong don so_token_yeu_cau vao so ghi phi cua api_key

  tra ve TEN cua node duoc chon
```

Kết quả trả về LÀ MỘT trong BA khả năng, phân biệt RÕ hai lý do từ chối:
`"tu_choi_sai_key"`, `"tu_choi_het_token"`, hoặc TÊN node được định tuyến
tới (khi request được phục vụ). Vì mỗi người dùng cần MỘT xô token VÀ một
sổ ghi phí RIÊNG, tồn tại qua NHIỀU lần gọi liên tiếp (không phải chỉ MỘT
lần), cả hai được lưu trong hai `dict` cấp module — `SUC_CHUA_CON_LAI` VÀ
`SO_GHI_PHI` — được ĐỌC và GHI trực tiếp bởi `xu_ly_cong_ai` qua khoá
`api_key`.
::::

::::example{#day_tam_request_qua_cong_ai}
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

SO_GHI_PHI = {}

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


def xu_ly_cong_ai(api_key, so_token_yeu_cau, danh_sach_tai_node):
    nguoi_dung = xac_thuc(api_key)
    if nguoi_dung is None:
        return "tu_choi_sai_key"

    suc_chua_con_lai = SUC_CHUA_CON_LAI[api_key]
    cho_qua, suc_chua_con_lai_moi = xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau)
    if not cho_qua:
        return "tu_choi_het_token"
    SUC_CHUA_CON_LAI[api_key] = suc_chua_con_lai_moi

    chi_so_node = least_loaded(danh_sach_tai_node)
    danh_sach_tai_node[chi_so_node] += 1

    SO_GHI_PHI[api_key] = SO_GHI_PHI.get(api_key, 0) + so_token_yeu_cau

    return DANH_SACH_NODE[chi_so_node]


def chay_day_yeu_cau(danh_sach_yeu_cau, danh_sach_tai_node):
    so_phuc_vu = 0
    so_tu_choi_sai_key = 0
    so_tu_choi_het_token = 0
    for yeu_cau in danh_sach_yeu_cau:
        ket_qua = xu_ly_cong_ai(yeu_cau["api_key"], yeu_cau["so_token"], danh_sach_tai_node)
        if ket_qua == "tu_choi_sai_key":
            so_tu_choi_sai_key += 1
        elif ket_qua == "tu_choi_het_token":
            so_tu_choi_het_token += 1
        else:
            so_phuc_vu += 1
    return so_phuc_vu, so_tu_choi_sai_key, so_tu_choi_het_token


DAY_YEU_CAU = [
    {"api_key": "key-abc123", "so_token": 100},
    {"api_key": "key-xyz-invalid", "so_token": 50},
    {"api_key": "key-def456", "so_token": 200},
    {"api_key": "key-ghi789", "so_token": 250},
    {"api_key": "key-abc123", "so_token": 850},
    {"api_key": "key-ghi789", "so_token": 100},
    {"api_key": "key-def456", "so_token": 4900},
    {"api_key": "", "so_token": 10},
]

DANH_SACH_TAI_NODE = [0, 0, 0, 0]

ket_qua = chay_day_yeu_cau(DAY_YEU_CAU, DANH_SACH_TAI_NODE)

print(ket_qua)
print(DANH_SACH_TAI_NODE)
print(SO_GHI_PHI)
```

```text title=readonly
(4, 2, 2)
[1, 1, 1, 1]
{'key-abc123': 950, 'key-def456': 200, 'key-ghi789': 100}
```

Tám request, từng cái một: (1) `key-abc123` cần `100` token — xô còn
`1000`, đủ — least-loaded chọn `gpu-0` (mọi tải đang bằng `0`, giữ chỉ số
ĐẦU TIÊN), xô còn lại `1000-100 = 900`. (2) `key-xyz-invalid` — không có
trong `BANG_NGUOI_DUNG` — `"tu_choi_sai_key"` NGAY. (3) `key-def456` cần
`200` — xô còn `5000`, đủ — tải hiện tại `[1,0,0,0]`, least-loaded chọn
`gpu-1` (tải `0`, nhẹ nhất). (4) `key-ghi789` cần `250` nhưng xô CHỈ còn
`200` — `250 > 200` — `"tu_choi_het_token"`, xô GIỮ NGUYÊN `200`. (5)
`key-abc123` cần `850` — xô còn `900` (từ bước 1), đủ (`850 <= 900`) — xô
còn lại `900-850 = 50` — tải `[1,1,0,0]`, chọn `gpu-2`. (6) `key-ghi789`
cần `100` — xô VẪN còn `200` (bước `4` bị từ chối không hề trừ token) —
đủ — tải `[1,1,1,0]`, chọn `gpu-3`. (7) `key-def456` cần `4900` nhưng xô
chỉ còn `5000-200 = 4800` — `4900 > 4800` — `"tu_choi_het_token"`. (8)
chuỗi rỗng `""` — không có trong bảng — `"tu_choi_sai_key"`. Tổng kết:
`4` request PHỤC VỤ (bước `1,3,5,6`), `2` từ chối vì sai key (bước `2,8`),
`2` từ chối vì hết token (bước `4,7`) — `ket_qua = (4, 2, 2)`. Vì mỗi node
được least-loaded chọn ĐÚNG MỘT lần (do tải luôn tăng dần đều sau mỗi lần
phục vụ), tải cuối cùng LÀ `[1, 1, 1, 1]` — cân bằng tuyệt đối. Sổ ghi phí
chỉ cộng dồn cho request THÀNH CÔNG: `key-abc123` LÀ `100+850 = 950`;
`key-def456` LÀ `200` (bước `7` bị từ chối, KHÔNG được ghi); `key-ghi789`
LÀ `100` (bước `4` bị từ chối, KHÔNG được ghi, chỉ bước `6` được tính).
::::

::::predict{#doan_bien_gioi_va_hoa_tai commitOnce}
Xét một người dùng MỚI, `"key-bien"`, có xô ĐANG còn ĐÚNG `5` token
(`suc_chua_con_lai = 5`). Request này cần ĐÚNG `5` token — CHÍNH XÁC bằng
phần còn lại. Đồng thời, CẢ BỐN node GPU đang có CÙNG một tải:
`danh_sach_tai_node = [3, 3, 3, 3]` (hoà tuyệt đối).

**Trước khi chạy thử**, bạn đoán: `xu_ly_cong_ai("key-bien", 5, [3, 3, 3,
3])` trả về gì?

:::opt{correct}
`"gpu-0"` — `5 <= 5` đúng (bài `2` dùng `<=`, không phải `<`), nên request
được phục vụ; VÀ khi mọi node có tải BẰNG NHAU, `least_loaded` (bài `4`)
CHỈ cập nhật chỉ số khi gặp một tải NHỎ HƠN THỰC SỰ (dùng `<`, không phải
`<=`) — không node nào có tải nhỏ hơn node đầu tiên, nên chỉ số GIỮ NGUYÊN
ở `0`, tức `DANH_SACH_NODE[0] = "gpu-0"`
:::

:::opt
`"tu_choi_het_token"` — xô chỉ còn ĐÚNG bằng số token cần dùng, không CÒN
DƯ, nên phải bị từ chối để an toàn
::why
Gần đúng ở việc xô ĐANG ở đúng ranh giới (`suc_chua_con_lai` bằng CHÍNH
`so_token_yeu_cau`) — quan sát về vị trí biên đó đúng.

Chỗ lệch: điều kiện cho qua của `xu_ly_yeu_cau` (bài `2`) LÀ
`so_token_yeu_cau <= suc_chua_con_lai`, và `5 <= 5` đúng — nó không đòi hỏi
phải CÒN DƯ (`<`), chỉ đòi hỏi ĐỦ (`<=`). Từ chối một request đúng bằng
phần còn lại là áp dụng một điều kiện chặt hơn (`<`) so với điều kiện thật
sự đã viết trong `xu_ly_yeu_cau`.
::
:::

:::opt
Request được phục vụ, nhưng KHÔNG THỂ xác định được node nào, vì tải của cả
bốn node đang bằng nhau tuyệt đối
::why
Gần đúng ở việc CẢ BỐN node THẬT SỰ có tải bằng nhau — không có node nào
"rõ ràng" nhẹ hơn các node còn lại về mặt số học.

Chỗ lệch: `least_loaded` không hề "bó tay" khi có nhiều node cùng tải nhỏ
nhất — nó LUÔN trả về một kết quả xác định: chỉ số của node ĐẦU TIÊN đạt
tải nhỏ nhất (vì vòng lặp chỉ cập nhật `chi_so_nhe_nhat` khi tìm thấy một
tải NHỎ HƠN THỰC SỰ tải đã lưu, dùng `<`; hoà thì KHÔNG cập nhật, chỉ số
đầu tiên vẫn giữ nguyên). Với `[3, 3, 3, 3]`, không phần tử nào sau chỉ số
`0` có tải nhỏ hơn `3`, nên chỉ số kết quả LÀ `0` — một quy tắc tất định,
không phải một trường hợp "không xác định".
::
:::
::::

::::code{#viet_xu_ly_cong_ai}
Hoàn thiện `xu_ly_cong_ai`: định tuyến bằng `least_loaded` (bài `4`) tới
node ít tải nhất, và ghi phí bằng cách cộng dồn số token vừa dùng vào
`SO_GHI_PHI` (dùng `.get` với mặc định `0`, đúng khuôn `dem_token_da_dung`
của bài `1`).

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

SO_GHI_PHI = {}

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


def xu_ly_cong_ai(api_key, so_token_yeu_cau, danh_sach_tai_node):
    nguoi_dung = xac_thuc(api_key)
    if nguoi_dung is None:
        return "tu_choi_sai_key"

    suc_chua_con_lai = SUC_CHUA_CON_LAI[api_key]
    cho_qua, suc_chua_con_lai_moi = xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau)
    if not cho_qua:
        return "tu_choi_het_token"
    SUC_CHUA_CON_LAI[api_key] = suc_chua_con_lai_moi

    chi_so_node = ___                                        # least_loaded(danh_sach_tai_node)
    danh_sach_tai_node[chi_so_node] += 1

    SO_GHI_PHI[api_key] = ___                                 # SO_GHI_PHI.get(api_key, 0) + so_token_yeu_cau

    return DANH_SACH_NODE[chi_so_node]


def chay_day_yeu_cau(danh_sach_yeu_cau, danh_sach_tai_node):
    so_phuc_vu = 0
    so_tu_choi_sai_key = 0
    so_tu_choi_het_token = 0
    for yeu_cau in danh_sach_yeu_cau:
        ket_qua = xu_ly_cong_ai(yeu_cau["api_key"], yeu_cau["so_token"], danh_sach_tai_node)
        if ket_qua == "tu_choi_sai_key":
            so_tu_choi_sai_key += 1
        elif ket_qua == "tu_choi_het_token":
            so_tu_choi_het_token += 1
        else:
            so_phuc_vu += 1
    return so_phuc_vu, so_tu_choi_sai_key, so_tu_choi_het_token


DAY_YEU_CAU = [
    {"api_key": "key-abc123", "so_token": 100},
    {"api_key": "key-xyz-invalid", "so_token": 50},
    {"api_key": "key-def456", "so_token": 200},
    {"api_key": "key-ghi789", "so_token": 250},
    {"api_key": "key-abc123", "so_token": 850},
    {"api_key": "key-ghi789", "so_token": 100},
    {"api_key": "key-def456", "so_token": 4900},
    {"api_key": "", "so_token": 10},
]

DANH_SACH_TAI_NODE = [0, 0, 0, 0]

ket_qua = chay_day_yeu_cau(DAY_YEU_CAU, DANH_SACH_TAI_NODE)

print(ket_qua)
print(DANH_SACH_TAI_NODE)
print(SO_GHI_PHI)
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

SO_GHI_PHI = {}

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


def xu_ly_cong_ai(api_key, so_token_yeu_cau, danh_sach_tai_node):
    nguoi_dung = xac_thuc(api_key)
    if nguoi_dung is None:
        return "tu_choi_sai_key"

    suc_chua_con_lai = SUC_CHUA_CON_LAI[api_key]
    cho_qua, suc_chua_con_lai_moi = xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau)
    if not cho_qua:
        return "tu_choi_het_token"
    SUC_CHUA_CON_LAI[api_key] = suc_chua_con_lai_moi

    chi_so_node = least_loaded(danh_sach_tai_node)
    danh_sach_tai_node[chi_so_node] += 1

    SO_GHI_PHI[api_key] = SO_GHI_PHI.get(api_key, 0) + so_token_yeu_cau

    return DANH_SACH_NODE[chi_so_node]


def chay_day_yeu_cau(danh_sach_yeu_cau, danh_sach_tai_node):
    so_phuc_vu = 0
    so_tu_choi_sai_key = 0
    so_tu_choi_het_token = 0
    for yeu_cau in danh_sach_yeu_cau:
        ket_qua = xu_ly_cong_ai(yeu_cau["api_key"], yeu_cau["so_token"], danh_sach_tai_node)
        if ket_qua == "tu_choi_sai_key":
            so_tu_choi_sai_key += 1
        elif ket_qua == "tu_choi_het_token":
            so_tu_choi_het_token += 1
        else:
            so_phuc_vu += 1
    return so_phuc_vu, so_tu_choi_sai_key, so_tu_choi_het_token


DAY_YEU_CAU = [
    {"api_key": "key-abc123", "so_token": 100},
    {"api_key": "key-xyz-invalid", "so_token": 50},
    {"api_key": "key-def456", "so_token": 200},
    {"api_key": "key-ghi789", "so_token": 250},
    {"api_key": "key-abc123", "so_token": 850},
    {"api_key": "key-ghi789", "so_token": 100},
    {"api_key": "key-def456", "so_token": 4900},
    {"api_key": "", "so_token": 10},
]

DANH_SACH_TAI_NODE = [0, 0, 0, 0]

ket_qua = chay_day_yeu_cau(DAY_YEU_CAU, DANH_SACH_TAI_NODE)

print(ket_qua)
print(DANH_SACH_TAI_NODE)
print(SO_GHI_PHI)
```

```python title=test
assert xac_thuc("key-abc123") == {"ten": "an", "han_muc": 1000}, f"key-abc123 phai xac thuc dung -- dang ra {xac_thuc('key-abc123')}"
assert xac_thuc("khong-ton-tai") is None, f"key la phai tra ve None -- dang ra {xac_thuc('khong-ton-tai')}"

assert ket_qua == (4, 2, 2), f"day 8 request phai cho 4 phuc vu, 2 sai key, 2 het token -- dang ra {ket_qua}"
assert DANH_SACH_TAI_NODE == [1, 1, 1, 1], f"moi node phai duoc dinh tuyen DUNG 1 lan (can bang tuyet doi) -- dang ra {DANH_SACH_TAI_NODE}"
assert SO_GHI_PHI == {"key-abc123": 950, "key-def456": 200, "key-ghi789": 100}, f"so ghi phi cuoi cung phai dung nhu tinh toan -- dang ra {SO_GHI_PHI}"
assert SUC_CHUA_CON_LAI == {"key-abc123": 50, "key-def456": 4800, "key-ghi789": 100}, f"xo con lai cuoi cung cua tung nguoi dung phai dung -- dang ra {SUC_CHUA_CON_LAI}"

assert xu_ly_cong_ai("key-khong-ton-tai", 10, [0, 0]) == "tu_choi_sai_key", f"key la phai bi tu choi NGAY -- dang ra {xu_ly_cong_ai('key-khong-ton-tai', 10, [0, 0])}"
assert xu_ly_cong_ai("key-ghi789", 101, [0, 0, 0, 0]) == "tu_choi_het_token", f"key-ghi789 chi con dung 100 token, yeu cau 101 phai bi tu choi -- dang ra {xu_ly_cong_ai('key-ghi789', 101, [0, 0, 0, 0])}"
assert xu_ly_cong_ai("key-ghi789", 50, [0, 0, 0, 0]) == "gpu-0", f"key-ghi789 con 100, yeu cau 50 hop le, node tai deu 0 phai chon gpu-0 -- dang ra {xu_ly_cong_ai('key-ghi789', 50, [0, 0, 0, 0])}"
assert SUC_CHUA_CON_LAI["key-ghi789"] == 50, f"sau khi tru them 50, key-ghi789 phai con 50 -- dang ra {SUC_CHUA_CON_LAI['key-ghi789']}"
assert SO_GHI_PHI["key-ghi789"] == 150, f"sau khi cong them 50, so ghi phi key-ghi789 phai la 150 (100 truoc do + 50) -- dang ra {SO_GHI_PHI['key-ghi789']}"
```

:::hints
- kind: attention
  body: "Hai cho trong, cung mot ham xu_ly_cong_ai, o BUOC SAU khi da qua ca xac thuc lan token bucket. Cho dau la GIA TRI GAN cho chi_so_node -- goi lai HAM DINH TUYEN da viet o bai 4, truyen danh_sach_tai_node. Cho hai la GIA TRI GAN cho SO_GHI_PHI[api_key] -- CONG DON so token vua dung vao gia tri DA CO trong so ghi phi (dung .get voi mac dinh 0, vi nguoi dung nay co the CHUA tung xuat hien trong SO_GHI_PHI)."
- kind: strategy
  body: "Cho dau: least_loaded(danh_sach_tai_node) -- goi ham da viet o bai 4, tim chi so node it tai nhat. Cho hai: SO_GHI_PHI.get(api_key, 0) + so_token_yeu_cau -- doc gia tri da ghi phi tu truoc (0 neu chua co), cong them so token cua request nay."
- kind: one-line
  body: "Cho dau la least_loaded(danh_sach_tai_node), cho hai la SO_GHI_PHI.get(api_key, 0) + so_token_yeu_cau."
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI HAM least_loaded(danh_sach_tai_node) (dinh tuyen bang bai 4, khong duoc tu viet lai logic tim min); cho trong hai phai dung SO_GHI_PHI.get(api_key, 0) + so_token_yeu_cau (cong don qua .get voi mac dinh 0, dung khuon bai 1)
  requireAst:
  - kind: uses-call, target: "least_loaded", min: 1
  - kind: uses-call, target: "get", min: 2
  # Da CHAY THAT qua kiemAst() (node + pyodide, trich solution tu chinh
  # file nay, TOAN BO ca sau ham) -- ket qua dung du kien:
  #   "least_loaded"=1: DUY NHAT o cho trong dau (dinh nghia ham
  #   least_loaded khong tu goi lai chinh no -- khong de quy).
  #   "get"=2: 1 lan trong xac_thuc (BANG_NGUOI_DUNG.get(api_key), DA CHO
  #   SAN trong starter tu bai 3) CONG 1 lan o cho trong hai
  #   (SO_GHI_PHI.get(api_key, 0)) -- tong THAT la 2.
  # Dien bua "True" vao CA HAI cho trong ("chi_so_node = True" va
  # "SO_GHI_PHI[api_key] = True") cho "least_loaded"=0 (duoi nguong 1) VA
  # "get" TUT xuong 1 (chi con lan trong xac_thuc, duoi nguong 2) -- CA HAI
  # luat CHAN DUNG (da CHAY THAT xac nhan qua kiemAst).
  #
  # GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho trong --
  # xac dinh ranh gioi tu chinh khoi starter -- va CHAY THAT qua kiemAst va
  # python3): dien "SO_GHI_PHI.get(api_key, 0) + so_token_yeu_cau" vao cho
  # trong dau (dong "chi_so_node = SO_GHI_PHI.get(api_key, 0) +
  # so_token_yeu_cau") VA dien "least_loaded(danh_sach_tai_node)" vao cho
  # trong hai (dong "SO_GHI_PHI[api_key] =
  # least_loaded(danh_sach_tai_node)") -- tong so lan "least_loaded" VA
  # "get" tren TOAN BO solution KHONG DOI (van la 1 va 2, chi doi VI TRI)
  # -- da CHAY THAT xac nhan qua kiemAst: static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ca hai bieu thuc moi deu la TEN HOP
  # LE trong scope cua xu_ly_cong_ai (SO_GHI_PHI, api_key, so_token_yeu_cau,
  # danh_sach_tai_node, least_loaded deu ton tai) nen KHONG nem NameError --
  # nhung dong ke tiep "danh_sach_tai_node[chi_so_node] += 1" dung
  # chi_so_node = SO_GHI_PHI.get(api_key, 0) + so_token_yeu_cau = 0 + 100 =
  # 100 (voi request dau tien, key-abc123, so_token=100) lam CHI SO TRUY
  # CAP mot danh sach chi co 4 phan tu -- IndexError "list index out of
  # range" NGAY o request DAU TIEN cua chay_day_yeu_cau. Da tu chay THAT
  # qua python3, xac nhan dung thong bao loi nay -- bi chan boi tier 'run',
  # doc lap voi static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\(4, 2, 2\\)\\n\\[1, 1, 1, 1\\]\\n\\{'key-abc123': 950, 'key-def456': 200, 'key-ghi789': 100\\}\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`(4, 2, 2)` — `4` phục vụ, `2` sai key, `2` hết token, tải cuối cùng
`[1, 1, 1, 1]` — cân bằng TUYỆT ĐỐI. `q8.6b` đóng tại `5/5`: giới hạn theo
token (không phải request), xô token thực thi giới hạn đó, xác thực trước
tiên, định tuyến biết chọn node nhẹ, VÀ ghi phí cho từng người dùng — bốn
khái niệm của `Chương 39.3` giờ ĐỀU LÀ những hàm Python chạy được VÀ đo
được bằng số cụ thể.
::::

::::reflect{#nghi-lai}
`xu_ly_cong_ai` không đưa ra thuật toán MỚI nào — mỗi bước bên trong nó ĐÃ
được viết VÀ kiểm chứng riêng ở bốn bài trước. Cái BOSS quý này làm được LÀ
đặt chúng vào ĐÚNG một thứ tự — xác thực trước token, token trước định
tuyến, định tuyến trước ghi phí — và chỉ ra rằng thứ tự đó KHÔNG tuỳ ý: đảo
ngược nó (ghi phí trước khi biết request có hợp lệ hay không, hay định
tuyến trước khi biết xô có đủ token) sẽ cho ra một cổng AI ghi sai số liệu
hoặc phí tài nguyên cho những request đáng lẽ phải bị từ chối. `SO_GHI_PHI`
cuối cùng — `950` cho `key-abc123`, `200` cho `key-def456`, `100` cho
`key-ghi789` — chỉ tính đúng những gì THẬT SỰ được phục vụ, không tính một
token nào của `2` request bị từ chối vì hết hạn mức. `q8.6b` đóng tại
`5/5`. Track tiếp theo của `T8.6` hỏi một câu hỏi lớn hơn: khi kho tài
liệu RAG (`q8.5f`) phải phục vụ HÀNG TRIỆU trang, phân mảnh VÀ cô lập dữ
liệu giữa nhiều khách hàng như thế nào?
::::

::::checkpoint{mastery=0.83}
::::
