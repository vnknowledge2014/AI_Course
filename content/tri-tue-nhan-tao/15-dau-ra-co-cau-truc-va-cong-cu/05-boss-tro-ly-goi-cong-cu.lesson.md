---
id: tri-tue-nhan-tao.dau-ra-co-cau-truc-va-cong-cu.boss-tro-ly-goi-cong-cu
title: "BOSS — Trợ lý gọi công cụ: ReAct + schema + retry ráp lại"
summary: "Rap ca bon ky thuat cua quest: dau ra co cau truc (bai 1), schema+retry (bai 2), function calling (bai 3), vong lap ReAct (bai 4) thanh MOT 'tro ly' xu ly 4 cau hoi co dinh -- 2 cau chi can 1 buoc goi cong cu (banh_mi, ca_phe), 2 cau can ReAct 2 buoc (tra_sua, sinh_to qua ma san pham); 2 trong 4 cau co phan hoi LLM mo phong THIEU truong o buoc cuoi, can retry moi dat schema. Tro ly day du (chay_tro_ly, co ReAct+retry): dung 4/4. Phien ban ngay tho (chi 2 vong goi LLM co dinh, KHONG kiem schema): dung 1/4. Hai doi chung rieng le (chi ReAct khong retry: 2/4: chi retry khong du vong: 2/4) xac nhan hai truc DOC LAP, can CA HAI moi dat toi da. Tat ca con so deu chay that."
locale: vi
track: tri-tue-nhan-tao
module: dau-ra-co-cau-truc-va-cong-cu
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-tro-ly-goi-cong-cu]
requires: [ai.vong-lap-react]
concepts: [ai.boss-tro-ly-goi-cong-cu]
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

::::byte{trigger=enter mood=happy pose=jump}
Bốn bài: đầu ra có cấu trúc, schema và retry, function calling, vòng lặp
ReAct. Bài này ráp TẤT CẢ lại thành một "trợ lý" — nhận nhiều câu hỏi khác
nhau, gọi ĐÚNG công cụ cần thiết (một bước hay hai bước), ép kiểm schema
trên mỗi phản hồi dọc đường, rồi đo cải thiện bằng số so với một phiên bản
ngây thơ.
::::

::::explain{#rap_toan_bo_tro_ly}
Một "trợ lý" hoàn chỉnh gồm ĐÚNG những gì bốn bài trước đã xây, ráp vào
MỘT vòng lặp duy nhất:

> **Đầu ra có cấu trúc** (`vi-sao-can-dau-ra-co-cau-truc`, bài `1`) — câu
> trả lời cuối luôn là một `dict` với trường cố định `"gia"`/`"don_vi"`,
> không phải văn xuôi.
>
> **Schema + retry** (`rang-buoc-theo-schema-va-retry`, bài `2`) — MỖI
> phản hồi LLM mô phỏng dọc đường (kể cả chỉ thị gọi hàm, kể cả câu trả lời
> cuối) đều bị kiểm schema; sai thì gọi lại, tối đa vài lần.
>
> **Function calling** (`function-calling-mo-phong`, bài `3`) — LLM mô
> phỏng không tự bịa dữ liệu, nó CHỈ RA công cụ cần gọi, code thực thi
> công cụ đó để lấy dữ liệu thật.
>
> **Vòng lặp ReAct** (`vong-lap-react`, bài `4`) — với câu hỏi cần NHIỀU
> bước (tra mã rồi mới tra giá), vòng lặp tiếp tục cho tới khi gặp chỉ thị
> trả lời cuối.

Bộ `4` câu hỏi kiểm thử cố định, mỗi câu một cấu hình khác nhau:

> `banh_mi` — giá tra trực tiếp (`1` bước), phản hồi cuối ĐỦ trường ngay
> lần đầu.
>
> `ca_phe` — giá tra trực tiếp (`1` bước), nhưng phản hồi cuối THIẾU
> trường `"don_vi"` ở lần gọi đầu — cần retry.
>
> `tra_sua` — cần tra MÃ trước rồi mới tra giá (ReAct `2` bước), phản hồi
> cuối ĐỦ trường ngay lần đầu.
>
> `sinh_to` — cần tra MÃ trước (ReAct `2` bước) VÀ phản hồi cuối THIẾU
> trường ở lần gọi đầu — cần CẢ ReAct lẫn retry.

Phép đo trung tâm: chạy bộ `4` câu hỏi qua **trợ lý đầy đủ** (ReAct đa
bước + schema/retry ở mỗi bước) và qua một **phiên bản ngây thơ** (chỉ
cho phép ĐÚNG `2` lần gọi LLM cố định, không kiểm schema, chấp nhận bất cứ
gì nhận được), đếm bao nhiêu câu trả lời ĐÚNG (đúng công cụ, đúng giá trị,
đúng schema) ở mỗi phiên bản.
::::

::::example{#boss_do_cai_thien}
Chạy cả trợ lý đầy đủ lẫn phiên bản ngây thơ trên bộ `4` câu hỏi:

```python title=readonly
BANG_GIA_TRUC_TIEP = {"banh_mi": 15000, "ca_phe": 25000}
BANG_MA = {"tra_sua": "SP003", "sinh_to": "SP004"}
BANG_GIA_THEO_MA = {"SP003": 42000, "SP004": 30000}
BANG_GIA_DUNG = {"banh_mi": 15000, "ca_phe": 25000, "tra_sua": 42000, "sinh_to": 30000}

DANH_SACH_CAU_HOI = ["banh_mi", "ca_phe", "tra_sua", "sinh_to"]

CAN_TRA_MA = {"tra_sua", "sinh_to"}
CAN_RETRY = {"ca_phe", "sinh_to"}


def ham_tra_cuu_gia(ten_hang):
    return BANG_GIA_TRUC_TIEP.get(ten_hang)


def ham_tra_cuu_ma(ten_hang):
    return BANG_MA.get(ten_hang)


def ham_tra_cuu_gia_theo_ma(ma_sp):
    return BANG_GIA_THEO_MA.get(ma_sp)


CAC_CONG_CU = {
    "tra_cuu_gia": ham_tra_cuu_gia,
    "tra_cuu_ma": ham_tra_cuu_ma,
    "tra_cuu_gia_theo_ma": ham_tra_cuu_gia_theo_ma,
}


def llm_mo_phong_react(ten_hang, vong, lan_goi_trong_vong, lich_su_quan_sat):
    if ten_hang in CAN_TRA_MA:
        if vong == 0:
            return {"goi_ham": "tra_cuu_ma", "tham_so": {"ten_hang": ten_hang}}
        if vong == 1:
            ma_sp = lich_su_quan_sat["tra_cuu_ma"]
            return {"goi_ham": "tra_cuu_gia_theo_ma", "tham_so": {"ma_sp": ma_sp}}
        gia = lich_su_quan_sat["tra_cuu_gia_theo_ma"]
    else:
        if vong == 0:
            return {"goi_ham": "tra_cuu_gia", "tham_so": {"ten_hang": ten_hang}}
        gia = lich_su_quan_sat["tra_cuu_gia"]

    if ten_hang in CAN_RETRY and lan_goi_trong_vong == 0:
        return {"tra_loi_cuoi": {"gia": gia}}
    return {"tra_loi_cuoi": {"gia": gia, "don_vi": "dong"}}


def kiem_tra_schema(dap_an, cac_field_bat_buoc):
    if not isinstance(dap_an, dict):
        return False
    for field in cac_field_bat_buoc:
        if field not in dap_an:
            return False
    return True


def kiem_tra_schema_day_du(chi_thi_tho):
    if not isinstance(chi_thi_tho, dict):
        return False
    if "goi_ham" in chi_thi_tho:
        return kiem_tra_schema(chi_thi_tho, ["goi_ham", "tham_so"])
    if "tra_loi_cuoi" in chi_thi_tho:
        return (kiem_tra_schema(chi_thi_tho, ["tra_loi_cuoi"])
                and kiem_tra_schema(chi_thi_tho["tra_loi_cuoi"], ["gia", "don_vi"]))
    return False


def goi_llm_voi_retry(ten_hang, vong, lich_su_quan_sat, so_lan_toi_da=3):
    for lan_goi in range(so_lan_toi_da):
        chi_thi = llm_mo_phong_react(ten_hang, vong, lan_goi, lich_su_quan_sat)
        if kiem_tra_schema_day_du(chi_thi):
            return chi_thi, lan_goi + 1
    return None, so_lan_toi_da


def chay_tro_ly(ten_hang, so_vong_toi_da=4):
    lich_su_quan_sat = {}
    tong_so_lan_goi_llm = 0
    for vong in range(so_vong_toi_da):
        chi_thi, so_lan_goi = goi_llm_voi_retry(ten_hang, vong, lich_su_quan_sat)
        tong_so_lan_goi_llm += so_lan_goi
        if chi_thi is None:
            return None, tong_so_lan_goi_llm
        if "tra_loi_cuoi" in chi_thi:
            return chi_thi["tra_loi_cuoi"], tong_so_lan_goi_llm
        ten_ham = chi_thi["goi_ham"]
        ham = CAC_CONG_CU[ten_ham]
        ket_qua = ham(**chi_thi["tham_so"])
        lich_su_quan_sat[ten_ham] = ket_qua
    return None, tong_so_lan_goi_llm


def tro_ly_day_du(ten_hang):
    dap_an, _ = chay_tro_ly(ten_hang)
    return dap_an


def naive_tro_ly(ten_hang):
    lich_su_quan_sat = {}
    chi_thi_1 = llm_mo_phong_react(ten_hang, 0, 0, lich_su_quan_sat)
    if "tra_loi_cuoi" in chi_thi_1:
        return chi_thi_1["tra_loi_cuoi"]
    ten_ham = chi_thi_1["goi_ham"]
    ham = CAC_CONG_CU[ten_ham]
    ket_qua = ham(**chi_thi_1["tham_so"])
    lich_su_quan_sat[ten_ham] = ket_qua

    chi_thi_2 = llm_mo_phong_react(ten_hang, 1, 0, lich_su_quan_sat)
    if "tra_loi_cuoi" in chi_thi_2:
        return chi_thi_2["tra_loi_cuoi"]
    return None


def dung_hay_khong(dap_an, ten_hang):
    if dap_an is None:
        return False
    if not kiem_tra_schema(dap_an, ["gia", "don_vi"]):
        return False
    return dap_an["gia"] == BANG_GIA_DUNG[ten_hang] and dap_an["don_vi"] == "dong"


def chay_bo_kiem_thu(ham_tro_ly):
    so_dung = 0
    for ten_hang in DANH_SACH_CAU_HOI:
        dap_an = ham_tro_ly(ten_hang)
        if dung_hay_khong(dap_an, ten_hang):
            so_dung += 1
    return so_dung


so_dung_ngay_tho = chay_bo_kiem_thu(naive_tro_ly)
so_dung_day_du = chay_bo_kiem_thu(tro_ly_day_du)

print(so_dung_ngay_tho, "/", len(DANH_SACH_CAU_HOI))
print(so_dung_day_du, "/", len(DANH_SACH_CAU_HOI))
print(so_dung_day_du - so_dung_ngay_tho)
```

```text title=readonly
1 / 4
4 / 4
3
```

Phiên bản ngây thơ: `1/4` — chỉ `banh_mi` đúng (không cần retry lẫn không
cần bước hai). Trợ lý đầy đủ: `4/4` — ReAct xử lý đúng cả hai câu hỏi
`2` bước (`tra_sua`, `sinh_to`), retry sửa đúng cả hai câu thiếu trường
(`ca_phe`, `sinh_to`). Cải thiện đo được: `+3` câu đúng trên `4` câu, từ
`25%` lên `100%`.
::::

::::example{#boss_doi_chung_hai_truc}
Để xác nhận ReAct đa bước và retry là HAI trục ĐỘC LẬP (không phải một kỹ
thuật ngẫu nhiên "làm được tất cả"), chạy thêm hai đối chứng: CHỈ có ReAct
đa bước (không retry), và CHỈ có retry nhưng giới hạn số vòng NHƯ bản ngây
thơ (không đủ bước cho câu hỏi `2` bước):

```python title=readonly
BANG_GIA_TRUC_TIEP = {"banh_mi": 15000, "ca_phe": 25000}
BANG_MA = {"tra_sua": "SP003", "sinh_to": "SP004"}
BANG_GIA_THEO_MA = {"SP003": 42000, "SP004": 30000}

DANH_SACH_CAU_HOI = ["banh_mi", "ca_phe", "tra_sua", "sinh_to"]

CAN_TRA_MA = {"tra_sua", "sinh_to"}
CAN_RETRY = {"ca_phe", "sinh_to"}


def ham_tra_cuu_gia(ten_hang):
    return BANG_GIA_TRUC_TIEP.get(ten_hang)


def ham_tra_cuu_ma(ten_hang):
    return BANG_MA.get(ten_hang)


def ham_tra_cuu_gia_theo_ma(ma_sp):
    return BANG_GIA_THEO_MA.get(ma_sp)


CAC_CONG_CU = {
    "tra_cuu_gia": ham_tra_cuu_gia,
    "tra_cuu_ma": ham_tra_cuu_ma,
    "tra_cuu_gia_theo_ma": ham_tra_cuu_gia_theo_ma,
}


def llm_mo_phong_react(ten_hang, vong, lan_goi_trong_vong, lich_su_quan_sat):
    if ten_hang in CAN_TRA_MA:
        if vong == 0:
            return {"goi_ham": "tra_cuu_ma", "tham_so": {"ten_hang": ten_hang}}
        if vong == 1:
            ma_sp = lich_su_quan_sat["tra_cuu_ma"]
            return {"goi_ham": "tra_cuu_gia_theo_ma", "tham_so": {"ma_sp": ma_sp}}
        gia = lich_su_quan_sat["tra_cuu_gia_theo_ma"]
    else:
        if vong == 0:
            return {"goi_ham": "tra_cuu_gia", "tham_so": {"ten_hang": ten_hang}}
        gia = lich_su_quan_sat["tra_cuu_gia"]

    if ten_hang in CAN_RETRY and lan_goi_trong_vong == 0:
        return {"tra_loi_cuoi": {"gia": gia}}
    return {"tra_loi_cuoi": {"gia": gia, "don_vi": "dong"}}


def kiem_tra_schema(dap_an, cac_field_bat_buoc):
    if not isinstance(dap_an, dict):
        return False
    for field in cac_field_bat_buoc:
        if field not in dap_an:
            return False
    return True


def kiem_tra_schema_day_du(chi_thi_tho):
    if not isinstance(chi_thi_tho, dict):
        return False
    if "goi_ham" in chi_thi_tho:
        return kiem_tra_schema(chi_thi_tho, ["goi_ham", "tham_so"])
    if "tra_loi_cuoi" in chi_thi_tho:
        return (kiem_tra_schema(chi_thi_tho, ["tra_loi_cuoi"])
                and kiem_tra_schema(chi_thi_tho["tra_loi_cuoi"], ["gia", "don_vi"]))
    return False


def goi_llm_voi_retry(ten_hang, vong, lich_su_quan_sat, so_lan_toi_da=3):
    for lan_goi in range(so_lan_toi_da):
        chi_thi = llm_mo_phong_react(ten_hang, vong, lan_goi, lich_su_quan_sat)
        if kiem_tra_schema_day_du(chi_thi):
            return chi_thi, lan_goi + 1
    return None, so_lan_toi_da


BANG_GIA_DUNG = {"banh_mi": 15000, "ca_phe": 25000, "tra_sua": 42000, "sinh_to": 30000}


def dung_hay_khong(dap_an, ten_hang):
    if dap_an is None:
        return False
    if not kiem_tra_schema(dap_an, ["gia", "don_vi"]):
        return False
    return dap_an["gia"] == BANG_GIA_DUNG[ten_hang] and dap_an["don_vi"] == "dong"


def chay_bo_kiem_thu(ham_tro_ly):
    so_dung = 0
    for ten_hang in DANH_SACH_CAU_HOI:
        dap_an = ham_tro_ly(ten_hang)
        if dung_hay_khong(dap_an, ten_hang):
            so_dung += 1
    return so_dung


def chay_tro_ly_khong_retry(ten_hang, so_vong_toi_da=4):
    lich_su_quan_sat = {}
    for vong in range(so_vong_toi_da):
        chi_thi = llm_mo_phong_react(ten_hang, vong, 0, lich_su_quan_sat)
        if not kiem_tra_schema_day_du(chi_thi):
            return None
        if "tra_loi_cuoi" in chi_thi:
            return chi_thi["tra_loi_cuoi"]
        ten_ham = chi_thi["goi_ham"]
        ham = CAC_CONG_CU[ten_ham]
        ket_qua = ham(**chi_thi["tham_so"])
        lich_su_quan_sat[ten_ham] = ket_qua
    return None


def chay_tro_ly_chi_retry_gioi_han_vong(ten_hang):
    lich_su_quan_sat = {}
    for vong in range(2):
        chi_thi, _ = goi_llm_voi_retry(ten_hang, vong, lich_su_quan_sat)
        if chi_thi is None:
            return None
        if "tra_loi_cuoi" in chi_thi:
            return chi_thi["tra_loi_cuoi"]
        ten_ham = chi_thi["goi_ham"]
        ham = CAC_CONG_CU[ten_ham]
        ket_qua = ham(**chi_thi["tham_so"])
        lich_su_quan_sat[ten_ham] = ket_qua
    return None


so_dung_chi_react = chay_bo_kiem_thu(chay_tro_ly_khong_retry)
so_dung_chi_retry = chay_bo_kiem_thu(chay_tro_ly_chi_retry_gioi_han_vong)

print(so_dung_chi_react, "/", len(DANH_SACH_CAU_HOI))
print(so_dung_chi_retry, "/", len(DANH_SACH_CAU_HOI))
```

```text title=readonly
2 / 4
2 / 4
```

**Chỉ ReAct** (không retry): `2/4` — đúng `banh_mi` và `tra_sua` (không
câu nào trong hai câu này cần retry), SAI `ca_phe` và `sinh_to` (cả hai
đều gặp phản hồi thiếu trường ở bước cuối và KHÔNG được gọi lại). **Chỉ
retry, giới hạn `2` vòng như bản ngây thơ**: `2/4` — đúng `banh_mi` và
`ca_phe` (retry sửa được lỗi thiếu trường trong đúng `2` vòng cho phép),
SAI `tra_sua` và `sinh_to` (cả hai cần MỘT vòng thứ `3` mà phiên bản này
không có, bất kể retry có sửa lỗi thiếu trường tốt tới đâu). Hai đối chứng
này xác nhận: ReAct đa bước và schema/retry là HAI trục tách biệt, và CHỈ
trợ lý đầy đủ (cả hai) mới đạt tối đa trên CẢ `4` câu.
::::

::::predict{#doan_naive_ca_phe commitOnce}
`naive_tro_ly` không kiểm schema — nó CHẤP NHẬN nguyên trạng bất cứ chỉ
thị `"tra_loi_cuoi"` nào nhận được ở lần gọi thứ nhất hoặc thứ hai, dù
thiếu trường.

**Trước khi chạy thử**, bạn đoán: `naive_tro_ly("ca_phe")` trả về gì? (nhớ
lại: `ca_phe` chỉ cần `1` bước công cụ, nhưng phản hồi trả lời cuối lần
gọi ĐẦU TIÊN của nó thiếu trường `"don_vi"`.)

:::opt{correct}
`{"gia": 25000}` — thiếu trường `"don_vi"`. `naive_tro_ly` gọi
`llm_mo_phong_react(..., lan_goi_trong_vong=0, ...)` — CỐ ĐỊNH tham số
`lan_goi_trong_vong=0`, không bao giờ thử lại với `1` — và thấy chỉ thị có
khoá `"tra_loi_cuoi"` thì trả về NGUYÊN VẸN giá trị đó, không hề gọi
`kiem_tra_schema_day_du` để phát hiện thiếu trường
:::

:::opt
`{"gia": 25000, "don_vi": "dong"}` — vì hàm vẫn chạy đủ `2` lần gọi LLM,
nên tới lần thứ hai thì trường đã được điền đủ
::why
Gần đúng ở việc bạn nhớ đúng giá trị SỐ (`25000`) — đó thật sự là giá
đúng của `ca_phe`.

Chỗ lệch: `2` lần gọi của `naive_tro_ly` là để xử lý TỐI ĐA `2` VÒNG hành
động (gọi công cụ rồi trả lời cuối) — không phải `2` LẦN THỬ LẠI cho CÙNG
một chỉ thị. Với `ca_phe`, vòng đầu ĐÃ là bước gọi công cụ
(`tra_cuu_gia`); vòng thứ hai (lần gọi LLM thứ hai) đã là câu trả lời cuối
— và vì `lan_goi_trong_vong` luôn cố định là `0`, nó nhận đúng phiên bản
THIẾU trường, không có cơ hội retry để sửa.
::
:::

:::opt
Sẽ báo lỗi, vì chỉ thị thiếu trường `"don_vi"` không đúng định dạng mà
hàm mong đợi
::why
Gần đúng ở việc bạn để ý chỉ thị này THẬT SỰ thiếu một trường so với
schema đầy đủ.

Chỗ lệch: `naive_tro_ly` không gọi `kiem_tra_schema` hay
`kiem_tra_schema_day_du` ở bất kỳ đâu — nó chỉ kiểm tra CÓ khoá
`"tra_loi_cuoi"` hay không (`if "tra_loi_cuoi" in chi_thi_2:`), rồi
`return chi_thi_2["tra_loi_cuoi"]` ngay, bất kể dict đó thiếu trường gì.
Không có bước nào trong hàm này có thể ném lỗi vì thiếu trường.
::
:::
::::

::::predict{#doan_ablation_sinh_to commitOnce}
`chay_tro_ly_khong_retry` có vòng lặp ReAct ĐẦY ĐỦ (không giới hạn `2`
vòng như bản ngây thơ) nhưng KHÔNG retry: nếu một chỉ thị không đạt
schema, hàm `return None` NGAY LẬP TỨC.

**Trước khi chạy thử**, bạn đoán: `chay_tro_ly_khong_retry("sinh_to")`
trả về gì? (nhớ lại: `sinh_to` cần ReAct `2` bước — tra mã rồi tra giá —
VÀ phản hồi trả lời cuối lần gọi đầu tiên của nó thiếu trường `"don_vi"`.)

:::opt{correct}
`None` — vòng `0` (tra mã) và vòng `1` (tra giá theo mã) đều là chỉ thị
gọi hàm ĐẦY ĐỦ trường (`kiem_tra_schema_day_du` đúng ngay lần đầu, không
cần retry ở hai vòng này); nhưng ở vòng `2` (trả lời cuối), chỉ thị nhận
được (với `lan_goi_trong_vong=0` cố định) là `{"tra_loi_cuoi": {"gia":
30000}}` — thiếu `"don_vi"`, nên `kiem_tra_schema_day_du` trả về `False`,
và hàm `return None` ngay, dù đã đi đúng cả hai bước ReAct trước đó
:::

:::opt
`{"gia": 30000}` — vì hàm vẫn "cố gắng" trả về những gì nhận được, dù
thiếu trường, giống cách `naive_tro_ly` chấp nhận nguyên trạng
::why
Gần đúng ở việc bạn nhớ đúng: ĐÂY chính xác là hành vi của `naive_tro_ly`
(bài toán trước) — chấp nhận nguyên trạng.

Chỗ lệch: `chay_tro_ly_khong_retry` là một hàm KHÁC, được viết để LUÔN
kiểm schema (`if not kiem_tra_schema_day_du(chi_thi): return None`) trước
khi làm bất cứ gì khác với chỉ thị — nó thà bỏ cuộc còn hơn trả về một
kết quả chưa qua kiểm. Đây chính là sự khác biệt giữa "có ReAct nhưng
không retry" và "không có gì cả".
::
:::

:::opt
`{"gia": 30000, "don_vi": "dong"}` — vì vòng lặp ReAct đầy đủ (không giới
hạn `2` vòng) cho đủ thời gian để LLM mô phỏng tự sửa trường thiếu
::why
Gần đúng ở việc bạn nhớ đúng: TRỢ LÝ ĐẦY ĐỦ (`chay_tro_ly`, có CẢ ReAct
LẪN retry) THẬT SỰ trả về đúng dict này cho `sinh_to`.

Chỗ lệch: "tự sửa trường thiếu" không xảy ra chỉ vì có NHIỀU vòng ReAct —
nó xảy ra vì có RETRY (gọi lại LLM với `lan_goi_trong_vong` tăng lên) ở
MỖI vòng. `chay_tro_ly_khong_retry` cố ý bỏ cơ chế retry đó
(`llm_mo_phong_react(..., 0, ...)` — luôn cố định `lan_goi_trong_vong=0`),
nên dù có đủ vòng ReAct, nó vẫn không có cơ hội nhận phiên bản đã sửa.
::
:::
::::

::::code{#viet_boss_ket_luan}
Hoàn thiện phần cuối: gọi `chay_bo_kiem_thu` với đúng hàm trợ lý cho mỗi
phiên bản, rồi tổng hợp CẢ HAI bằng chứng (có cải thiện VÀ đạt tối đa)
thành một kết luận duy nhất.

```python title=starter
BANG_GIA_TRUC_TIEP = {"banh_mi": 15000, "ca_phe": 25000}
BANG_MA = {"tra_sua": "SP003", "sinh_to": "SP004"}
BANG_GIA_THEO_MA = {"SP003": 42000, "SP004": 30000}
BANG_GIA_DUNG = {"banh_mi": 15000, "ca_phe": 25000, "tra_sua": 42000, "sinh_to": 30000}

DANH_SACH_CAU_HOI = ["banh_mi", "ca_phe", "tra_sua", "sinh_to"]

CAN_TRA_MA = {"tra_sua", "sinh_to"}
CAN_RETRY = {"ca_phe", "sinh_to"}


def ham_tra_cuu_gia(ten_hang):
    return BANG_GIA_TRUC_TIEP.get(ten_hang)


def ham_tra_cuu_ma(ten_hang):
    return BANG_MA.get(ten_hang)


def ham_tra_cuu_gia_theo_ma(ma_sp):
    return BANG_GIA_THEO_MA.get(ma_sp)


CAC_CONG_CU = {
    "tra_cuu_gia": ham_tra_cuu_gia,
    "tra_cuu_ma": ham_tra_cuu_ma,
    "tra_cuu_gia_theo_ma": ham_tra_cuu_gia_theo_ma,
}


def llm_mo_phong_react(ten_hang, vong, lan_goi_trong_vong, lich_su_quan_sat):
    if ten_hang in CAN_TRA_MA:
        if vong == 0:
            return {"goi_ham": "tra_cuu_ma", "tham_so": {"ten_hang": ten_hang}}
        if vong == 1:
            ma_sp = lich_su_quan_sat["tra_cuu_ma"]
            return {"goi_ham": "tra_cuu_gia_theo_ma", "tham_so": {"ma_sp": ma_sp}}
        gia = lich_su_quan_sat["tra_cuu_gia_theo_ma"]
    else:
        if vong == 0:
            return {"goi_ham": "tra_cuu_gia", "tham_so": {"ten_hang": ten_hang}}
        gia = lich_su_quan_sat["tra_cuu_gia"]

    if ten_hang in CAN_RETRY and lan_goi_trong_vong == 0:
        return {"tra_loi_cuoi": {"gia": gia}}
    return {"tra_loi_cuoi": {"gia": gia, "don_vi": "dong"}}


def kiem_tra_schema(dap_an, cac_field_bat_buoc):
    if not isinstance(dap_an, dict):
        return False
    for field in cac_field_bat_buoc:
        if field not in dap_an:
            return False
    return True


def kiem_tra_schema_day_du(chi_thi_tho):
    if not isinstance(chi_thi_tho, dict):
        return False
    if "goi_ham" in chi_thi_tho:
        return kiem_tra_schema(chi_thi_tho, ["goi_ham", "tham_so"])
    if "tra_loi_cuoi" in chi_thi_tho:
        return (kiem_tra_schema(chi_thi_tho, ["tra_loi_cuoi"])
                and kiem_tra_schema(chi_thi_tho["tra_loi_cuoi"], ["gia", "don_vi"]))
    return False


def goi_llm_voi_retry(ten_hang, vong, lich_su_quan_sat, so_lan_toi_da=3):
    for lan_goi in range(so_lan_toi_da):
        chi_thi = llm_mo_phong_react(ten_hang, vong, lan_goi, lich_su_quan_sat)
        if kiem_tra_schema_day_du(chi_thi):
            return chi_thi, lan_goi + 1
    return None, so_lan_toi_da


def chay_tro_ly(ten_hang, so_vong_toi_da=4):
    lich_su_quan_sat = {}
    tong_so_lan_goi_llm = 0
    for vong in range(so_vong_toi_da):
        chi_thi, so_lan_goi = goi_llm_voi_retry(ten_hang, vong, lich_su_quan_sat)
        tong_so_lan_goi_llm += so_lan_goi
        if chi_thi is None:
            return None, tong_so_lan_goi_llm
        if "tra_loi_cuoi" in chi_thi:
            return chi_thi["tra_loi_cuoi"], tong_so_lan_goi_llm
        ten_ham = chi_thi["goi_ham"]
        ham = CAC_CONG_CU[ten_ham]
        ket_qua = ham(**chi_thi["tham_so"])
        lich_su_quan_sat[ten_ham] = ket_qua
    return None, tong_so_lan_goi_llm


def tro_ly_day_du(ten_hang):
    dap_an, _ = chay_tro_ly(ten_hang)
    return dap_an


def naive_tro_ly(ten_hang):
    lich_su_quan_sat = {}
    chi_thi_1 = llm_mo_phong_react(ten_hang, 0, 0, lich_su_quan_sat)
    if "tra_loi_cuoi" in chi_thi_1:
        return chi_thi_1["tra_loi_cuoi"]
    ten_ham = chi_thi_1["goi_ham"]
    ham = CAC_CONG_CU[ten_ham]
    ket_qua = ham(**chi_thi_1["tham_so"])
    lich_su_quan_sat[ten_ham] = ket_qua

    chi_thi_2 = llm_mo_phong_react(ten_hang, 1, 0, lich_su_quan_sat)
    if "tra_loi_cuoi" in chi_thi_2:
        return chi_thi_2["tra_loi_cuoi"]
    return None


def dung_hay_khong(dap_an, ten_hang):
    if dap_an is None:
        return False
    if not kiem_tra_schema(dap_an, ["gia", "don_vi"]):
        return False
    return dap_an["gia"] == BANG_GIA_DUNG[ten_hang] and dap_an["don_vi"] == "dong"


def chay_bo_kiem_thu(ham_tro_ly):
    so_dung = 0
    for ten_hang in DANH_SACH_CAU_HOI:
        dap_an = ham_tro_ly(ten_hang)
        if dung_hay_khong(dap_an, ten_hang):
            so_dung += 1
    return so_dung


so_dung_ngay_tho = chay_bo_kiem_thu(___)                 # naive_tro_ly
so_dung_day_du = chay_bo_kiem_thu(___)                    # tro_ly_day_du

co_cai_thien = so_dung_day_du > so_dung_ngay_tho
dat_toi_da = so_dung_day_du == len(DANH_SACH_CAU_HOI)
ket_luan = ___                                            # co_cai_thien and dat_toi_da

print(so_dung_ngay_tho, "/", len(DANH_SACH_CAU_HOI))
print(so_dung_day_du, "/", len(DANH_SACH_CAU_HOI))
print(co_cai_thien, dat_toi_da)
print(ket_luan)
```

```python title=solution
BANG_GIA_TRUC_TIEP = {"banh_mi": 15000, "ca_phe": 25000}
BANG_MA = {"tra_sua": "SP003", "sinh_to": "SP004"}
BANG_GIA_THEO_MA = {"SP003": 42000, "SP004": 30000}
BANG_GIA_DUNG = {"banh_mi": 15000, "ca_phe": 25000, "tra_sua": 42000, "sinh_to": 30000}

DANH_SACH_CAU_HOI = ["banh_mi", "ca_phe", "tra_sua", "sinh_to"]

CAN_TRA_MA = {"tra_sua", "sinh_to"}
CAN_RETRY = {"ca_phe", "sinh_to"}


def ham_tra_cuu_gia(ten_hang):
    return BANG_GIA_TRUC_TIEP.get(ten_hang)


def ham_tra_cuu_ma(ten_hang):
    return BANG_MA.get(ten_hang)


def ham_tra_cuu_gia_theo_ma(ma_sp):
    return BANG_GIA_THEO_MA.get(ma_sp)


CAC_CONG_CU = {
    "tra_cuu_gia": ham_tra_cuu_gia,
    "tra_cuu_ma": ham_tra_cuu_ma,
    "tra_cuu_gia_theo_ma": ham_tra_cuu_gia_theo_ma,
}


def llm_mo_phong_react(ten_hang, vong, lan_goi_trong_vong, lich_su_quan_sat):
    if ten_hang in CAN_TRA_MA:
        if vong == 0:
            return {"goi_ham": "tra_cuu_ma", "tham_so": {"ten_hang": ten_hang}}
        if vong == 1:
            ma_sp = lich_su_quan_sat["tra_cuu_ma"]
            return {"goi_ham": "tra_cuu_gia_theo_ma", "tham_so": {"ma_sp": ma_sp}}
        gia = lich_su_quan_sat["tra_cuu_gia_theo_ma"]
    else:
        if vong == 0:
            return {"goi_ham": "tra_cuu_gia", "tham_so": {"ten_hang": ten_hang}}
        gia = lich_su_quan_sat["tra_cuu_gia"]

    if ten_hang in CAN_RETRY and lan_goi_trong_vong == 0:
        return {"tra_loi_cuoi": {"gia": gia}}
    return {"tra_loi_cuoi": {"gia": gia, "don_vi": "dong"}}


def kiem_tra_schema(dap_an, cac_field_bat_buoc):
    if not isinstance(dap_an, dict):
        return False
    for field in cac_field_bat_buoc:
        if field not in dap_an:
            return False
    return True


def kiem_tra_schema_day_du(chi_thi_tho):
    if not isinstance(chi_thi_tho, dict):
        return False
    if "goi_ham" in chi_thi_tho:
        return kiem_tra_schema(chi_thi_tho, ["goi_ham", "tham_so"])
    if "tra_loi_cuoi" in chi_thi_tho:
        return (kiem_tra_schema(chi_thi_tho, ["tra_loi_cuoi"])
                and kiem_tra_schema(chi_thi_tho["tra_loi_cuoi"], ["gia", "don_vi"]))
    return False


def goi_llm_voi_retry(ten_hang, vong, lich_su_quan_sat, so_lan_toi_da=3):
    for lan_goi in range(so_lan_toi_da):
        chi_thi = llm_mo_phong_react(ten_hang, vong, lan_goi, lich_su_quan_sat)
        if kiem_tra_schema_day_du(chi_thi):
            return chi_thi, lan_goi + 1
    return None, so_lan_toi_da


def chay_tro_ly(ten_hang, so_vong_toi_da=4):
    lich_su_quan_sat = {}
    tong_so_lan_goi_llm = 0
    for vong in range(so_vong_toi_da):
        chi_thi, so_lan_goi = goi_llm_voi_retry(ten_hang, vong, lich_su_quan_sat)
        tong_so_lan_goi_llm += so_lan_goi
        if chi_thi is None:
            return None, tong_so_lan_goi_llm
        if "tra_loi_cuoi" in chi_thi:
            return chi_thi["tra_loi_cuoi"], tong_so_lan_goi_llm
        ten_ham = chi_thi["goi_ham"]
        ham = CAC_CONG_CU[ten_ham]
        ket_qua = ham(**chi_thi["tham_so"])
        lich_su_quan_sat[ten_ham] = ket_qua
    return None, tong_so_lan_goi_llm


def tro_ly_day_du(ten_hang):
    dap_an, _ = chay_tro_ly(ten_hang)
    return dap_an


def naive_tro_ly(ten_hang):
    lich_su_quan_sat = {}
    chi_thi_1 = llm_mo_phong_react(ten_hang, 0, 0, lich_su_quan_sat)
    if "tra_loi_cuoi" in chi_thi_1:
        return chi_thi_1["tra_loi_cuoi"]
    ten_ham = chi_thi_1["goi_ham"]
    ham = CAC_CONG_CU[ten_ham]
    ket_qua = ham(**chi_thi_1["tham_so"])
    lich_su_quan_sat[ten_ham] = ket_qua

    chi_thi_2 = llm_mo_phong_react(ten_hang, 1, 0, lich_su_quan_sat)
    if "tra_loi_cuoi" in chi_thi_2:
        return chi_thi_2["tra_loi_cuoi"]
    return None


def dung_hay_khong(dap_an, ten_hang):
    if dap_an is None:
        return False
    if not kiem_tra_schema(dap_an, ["gia", "don_vi"]):
        return False
    return dap_an["gia"] == BANG_GIA_DUNG[ten_hang] and dap_an["don_vi"] == "dong"


def chay_bo_kiem_thu(ham_tro_ly):
    so_dung = 0
    for ten_hang in DANH_SACH_CAU_HOI:
        dap_an = ham_tro_ly(ten_hang)
        if dung_hay_khong(dap_an, ten_hang):
            so_dung += 1
    return so_dung


so_dung_ngay_tho = chay_bo_kiem_thu(naive_tro_ly)
so_dung_day_du = chay_bo_kiem_thu(tro_ly_day_du)

co_cai_thien = so_dung_day_du > so_dung_ngay_tho
dat_toi_da = so_dung_day_du == len(DANH_SACH_CAU_HOI)
ket_luan = co_cai_thien and dat_toi_da

print(so_dung_ngay_tho, "/", len(DANH_SACH_CAU_HOI))
print(so_dung_day_du, "/", len(DANH_SACH_CAU_HOI))
print(co_cai_thien, dat_toi_da)
print(ket_luan)
```

```python title=test
assert so_dung_ngay_tho == 1, f"phien ban ngay tho phai la 1/4 -- dang ra {so_dung_ngay_tho}"
assert so_dung_day_du == 4, f"tro ly day du phai la 4/4 -- dang ra {so_dung_day_du}"
assert co_cai_thien == True, "co_cai_thien phai la True"
assert dat_toi_da == True, "dat_toi_da phai la True"
assert ket_luan == True, "ket_luan phai la True -- CA HAI bang chung deu phai dung"

# bien: doi chung rieng le, xac nhan ReAct da buoc va schema/retry la HAI
# truc doc lap (khong phai mot con so ngau nhien)
def chay_tro_ly_khong_retry(ten_hang, so_vong_toi_da=4):
    lich_su_quan_sat = {}
    for vong in range(so_vong_toi_da):
        chi_thi = llm_mo_phong_react(ten_hang, vong, 0, lich_su_quan_sat)
        if not kiem_tra_schema_day_du(chi_thi):
            return None
        if "tra_loi_cuoi" in chi_thi:
            return chi_thi["tra_loi_cuoi"]
        ten_ham = chi_thi["goi_ham"]
        ham = CAC_CONG_CU[ten_ham]
        ket_qua = ham(**chi_thi["tham_so"])
        lich_su_quan_sat[ten_ham] = ket_qua
    return None


def chay_tro_ly_chi_retry_gioi_han_vong(ten_hang):
    lich_su_quan_sat = {}
    for vong in range(2):
        chi_thi, _ = goi_llm_voi_retry(ten_hang, vong, lich_su_quan_sat)
        if chi_thi is None:
            return None
        if "tra_loi_cuoi" in chi_thi:
            return chi_thi["tra_loi_cuoi"]
        ten_ham = chi_thi["goi_ham"]
        ham = CAC_CONG_CU[ten_ham]
        ket_qua = ham(**chi_thi["tham_so"])
        lich_su_quan_sat[ten_ham] = ket_qua
    return None


assert chay_bo_kiem_thu(chay_tro_ly_khong_retry) == 2, f"chi ReAct (khong retry) phai la 2/4 -- dang ra {chay_bo_kiem_thu(chay_tro_ly_khong_retry)}"
assert chay_bo_kiem_thu(chay_tro_ly_chi_retry_gioi_han_vong) == 2, f"chi retry (gioi han vong) phai la 2/4 -- dang ra {chay_bo_kiem_thu(chay_tro_ly_chi_retry_gioi_han_vong)}"

# GOTCHA "and" vs "or": tren du lieu THAT cua bai nay, co_cai_thien (True)
# va dat_toi_da (True) deu True nen "and"/"or" cho CUNG ket qua -- chi
# static rieng moi phan biet duoc bien the toan tu nay, khong output/tests
# nao bat duoc vi ca hai deu cho ket_luan=True.
assert naive_tro_ly("banh_mi") == {"gia": 15000, "don_vi": "dong"}, "naive banh_mi phai dung du"
assert naive_tro_ly("ca_phe") == {"gia": 25000}, f"naive ca_phe phai thieu don_vi -- dang ra {naive_tro_ly('ca_phe')!r}"
assert naive_tro_ly("tra_sua") is None, "naive tra_sua phai la None (can 3 vong, naive chi cho 2)"
assert naive_tro_ly("sinh_to") is None, "naive sinh_to phai la None (can ca ReAct lan retry)"

assert chay_tro_ly("banh_mi") == ({"gia": 15000, "don_vi": "dong"}, 2), f"chay_tro_ly banh_mi sai -- dang ra {chay_tro_ly('banh_mi')}"
assert chay_tro_ly("sinh_to") == ({"gia": 30000, "don_vi": "dong"}, 4), f"chay_tro_ly sinh_to sai -- dang ra {chay_tro_ly('sinh_to')}"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ đầu và chỗ hai truyền ĐÚNG hàm trợ lý (không gọi nó, chỉ truyền TÊN hàm) cho `chay_bo_kiem_thu` — `naive_tro_ly` cho phiên bản ngây thơ, `tro_ly_day_du` cho phiên bản đầy đủ. Chỗ ba tổng hợp HAI điều kiện (`co_cai_thien`, `dat_toi_da`) bằng toán tử kết hợp CẢ HAI phải đúng.
- kind: strategy
  body: 'Chỗ đầu: `naive_tro_ly`. Chỗ hai: `tro_ly_day_du`. Chỗ ba: `co_cai_thien and dat_toi_da`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `naive_tro_ly`, `tro_ly_day_du`, và `co_cai_thien and dat_toi_da`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: hai loi goi chay_bo_kiem_thu phai dung DUNG hai ham tro ly khac nhau (naive_tro_ly va tro_ly_day_du, khong duoc dao nguoc hay goi trung mot ham), VA ket_luan phai la phep 'and' THAT giua ca hai dieu kien (khong duoc chep san True, khong doi thanh 'or')
  requireAst:
  - kind: uses-call, target: chay_bo_kiem_thu, min: 2
  - kind: uses-name, target: naive_tro_ly, min: 1
  - kind: uses-name, target: tro_ly_day_du, min: 1
  - kind: uses-name, target: co_cai_thien, min: 2
  - kind: uses-operator, target: and, min: 4
  # Da thu that (goi _dem tren code trich tu solution, khong doan tay).
  # chay_bo_kiem_thu=2: dung hai lan goi, chinh la cho trong 1 va 2 --
  # dinh nghia ham khong tinh la Call. Dien bua chep san
  # "so_dung_ngay_tho = 0" (bo qua goi ham THAT) lam so nay tut xuong duoi
  # 2 -- bi chan; dong thoi bi chan boi tests.
  # naive_tro_ly=1, tro_ly_day_du=1: moi ten CHI xuat hien dung 1 lan, o
  # dung cho trong tuong ung. Dao nguoc hai cho trong cho nhau khong doi so
  # dem AST nao (van moi ten 1 lan) NHUNG lam so_dung_ngay_tho va
  # so_dung_day_du hoan doi gia tri cho nhau -- bi chan boi tests
  # (so_dung_ngay_tho phai la 1, se thanh 4 neu dao nguoc).
  # uses-name "co_cai_thien"=2: mot lan CO SAN trong "print(co_cai_thien,
  # dat_toi_da)" (luon chay, khong bi cho trong), mot lan la CHINH cho
  # trong 3. Neu chi dat min=1 (ngay tho), mot mutant thay ten
  # "co_cai_thien" bang mot ten KHAC luon-dung-True (vi du
  # "chay_bo_kiem_thu", mot ham, luon truthy) van qua duoc vi con lai 1
  # lan doc "co_cai_thien" trong print -- day la GOTCHA
  # "boilerplate-threshold-masking"; dat dung min=2 (tong THAT) moi chan
  # duoc mutant nay. Mutant nay KHONG bi bat boi output/tests vi ca hai gia
  # tri lam "X and dat_toi_da" tra ve dat_toi_da=True nhu nhau -- CHI static
  # rieng moi bat duoc.
  # and=4: ba lan CO SAN (llm_mo_phong_react: "ten_hang in CAN_RETRY and
  # lan_goi_trong_vong == 0"; kiem_tra_schema_day_du: hai lenh
  # kiem_tra_schema noi boi "and"; dung_hay_khong: "== BANG_GIA_DUNG[...]
  # and dap_an['don_vi'] == 'dong'"), mot lan la cho trong 3. Neu chi dat
  # min=1 (ngay tho), mot mutant chep san "ket_luan = True" (bo hoan toan
  # BoolOp) VAN qua duoc vi con lai ba lan "and" trong boilerplate -- day
  # la GOTCHA "boilerplate-threshold-masking"; dat dung min=4 (tong THAT)
  # moi chan duoc mutant nay -- va no cung bi chan boi tests CHI o tang
  # static rieng (ket_luan van la True nen khong bi bat boi output/tests).
  # Doi "and" thanh "or" o cho trong 3 cung bi chan boi static (BoolOp doi
  # tu And sang Or, "and" tut ve 3 duoi nguong 4) MAC DU tren du lieu THAT
  # ca hai dieu kien deu True nen "or" cho CUNG ket_luan=True -- khong
  # output/tests nao bat duoc bien the nay, chi static rieng moi bat.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^1 / 4\\n4 / 4\\nTrue True\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Phiên bản ngây thơ: `1/4`. Trợ lý đầy đủ (đầu ra có cấu trúc + schema/retry
+ function calling + ReAct): `4/4`. Cải thiện `+3`, từ `25%` lên `100%`,
đo bằng số thật trên cùng một bộ câu hỏi. Quest
`dau-ra-co-cau-truc-va-cong-cu` (q8.4c) khép lại tại đây.
::::

::::reflect{#nghi-lai}
Năm bài, một domain xuyên suốt (tra cứu giá mặt hàng qua một "trợ lý mô
phỏng"), bốn kỹ thuật ráp dần vào một hệ thống:

> **`vi-sao-can-dau-ra-co-cau-truc`** (bài `1`) — văn xuôi tự do không có
> vị trí cố định cho dữ liệu; một `dict` với trường cố định thì có.
>
> **`rang-buoc-theo-schema-va-retry`** (bài `2`) — một schema (danh sách
> trường bắt buộc) phát hiện được câu trả lời thiếu trường, và retry cho
> một cơ hội sửa — nhưng có giới hạn, không lặp vô hạn.
>
> **`function-calling-mo-phong`** (bài `3`) — LLM (mô phỏng hay thật)
> không tự bịa dữ liệu; nó CHỌN công cụ, công cụ mới lấy dữ liệu thật.
>
> **`vong-lap-react`** (bài `4`) — một số câu hỏi cần NHIỀU bước công cụ
> liên tiếp, mỗi bước dùng kết quả của bước trước; vòng lặp dừng đúng lúc
> gặp câu trả lời cuối.
>
> **`boss-tro-ly-goi-cong-cu`** (bài `5`, quest này) — ráp cả bốn kỹ
> thuật, đo cải thiện `1/4 → 4/4` trên cùng một bộ kiểm thử, và tách bạch
> hai trục ReAct/retry bằng hai đối chứng riêng.

Toàn bộ quest dùng đúng MỘT nguyên tắc xuyên suốt track T8.4: một LLM MÔ
PHỎNG — hàm Python thuần, tất định, không mạng nơ-ron, không gọi API
thật, không network — vẫn đủ để đo ĐƯỢC BẰNG SỐ những cơ chế cốt lõi của
việc DÙNG một mô hình ngôn ngữ: vì sao cần đầu ra có cấu trúc, vì sao cần
kiểm schema và retry, vì sao công cụ (không phải chính mô hình) mới là
nguồn dữ liệu thật, và vì sao một số câu hỏi cần một VÒNG LẶP thay vì một
lần gọi.
::::

::::checkpoint{mastery=0.9}
::::
