---
id: tri-tue-nhan-tao.dau-ra-co-cau-truc-va-cong-cu.vong-lap-react
title: "Vòng lặp ReAct: suy luận, gọi công cụ, quan sát, lặp lại"
summary: "Mo phong mot cau hoi CAN HAI buoc goi cong cu lien tiep: buoc 1 tra cuu MA san pham tu TEN (ham_tra_cuu_ma), buoc 2 dung MA do tra GIA (ham_tra_cuu_gia_theo_ma) -- khong the gop lam mot buoc vi buoc 2 can dung INPUT la ket qua cua buoc 1. LLM mo phong llm_mo_phong_react(ten_hang, vong, lich_su_quan_sat) tra ve mot chi thi goi ham o MOI vong cho toi khi tra ve chi thi 'tra_loi_cuoi'. Vong lap chay_react dem so vong THAT: dung 3 vong (goi cong cu 1, goi cong cu 2, tra loi cuoi) cho ca ba mat hang, gia tri cuoi dung khop bang tra cuu goc. Gioi han vong qua som (1 hoac 2 vong) cho ket qua None -- xac nhan vong lap dung CAN du buoc, khong tu 'nhay tat'."
locale: vi
track: tri-tue-nhan-tao
module: dau-ra-co-cau-truc-va-cong-cu
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.vong-lap-react]
requires: [ai.function-calling-mo-phong]
concepts: [ai.vong-lap-react]
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
Bài trước: MỘT chỉ thị gọi hàm, MỘT lần gọi công cụ, xong. Nhưng có những
câu hỏi cần HAI lần gọi LIÊN TIẾP — kết quả bước một là ĐẦU VÀO của bước
hai. Bài này lặp cho tới khi đủ thông tin: đây là vòng lặp **ReAct**.
::::

::::explain{#react_la_gi}
**ReAct** (Reasoning + Acting) là một khuôn mẫu cho vòng lặp dùng công cụ
nhiều bước: ở MỖI vòng, mô hình SUY LUẬN xem cần làm gì tiếp (Reasoning),
đưa ra một HÀNH ĐỘNG — thường là một chỉ thị gọi công cụ (Action) — rồi
nhận QUAN SÁT là kết quả công cụ trả về (Observation). Vòng lặp này LẶP
LẠI, mỗi lần dùng thêm quan sát MỚI, cho tới khi mô hình có đủ thông tin để
đưa ra một CÂU TRẢ LỜI CUỐI — khác hẳn một chỉ thị gọi công cụ.

Bài trước, một chỉ thị gọi hàm là đủ vì công cụ chỉ cần MỘT đầu vào có sẵn
ngay từ câu hỏi (tên mặt hàng). Có những câu hỏi không như vậy: để tra
GIÁ theo MÃ sản phẩm, trước tiên phải biết MÃ — mà MÃ đó lại phải tra từ
TÊN mặt hàng. Hai bước này không gộp được thành một, vì bước hai cần MỘT
GIÁ TRỊ chỉ có SAU KHI bước một đã chạy xong.

LLM mô phỏng của bài này nhận thêm hai tham số: `vong` (vòng lặp thứ mấy,
đếm từ `0`) và `lich_su_quan_sat` (một `dict` tích luỹ kết quả các công cụ
ĐÃ gọi ở những vòng trước). Ở mỗi vòng, nó trả về MỘT trong hai dạng chỉ
thị:

> **Chỉ thị gọi hàm** — `{"goi_ham": ..., "tham_so": {...}}`, giống hệt
> bài trước.
>
> **Chỉ thị trả lời cuối** — `{"tra_loi_cuoi": ...}` — báo hiệu ĐÃ đủ
> thông tin, vòng lặp phải DỪNG và trả về giá trị này.

Vòng lặp chạy cho tới khi gặp chỉ thị thứ hai, hoặc hết số vòng tối đa cho
phép (giống giới hạn `so_lan_toi_da` của retry ở bài `rang-buoc-theo-
schema-va-retry`, nhưng lần này giới hạn SỐ VÒNG hành động, không phải số
lần gọi lại vì lỗi schema).
::::

::::example{#do_react_hai_buoc}
Tra giá qua MÃ sản phẩm — cần đúng hai lần gọi công cụ liên tiếp rồi mới
trả lời được:

```python title=readonly
BANG_MA = {
    "banh_mi": "SP001",
    "ca_phe": "SP002",
    "tra_sua": "SP003",
}

BANG_GIA_THEO_MA = {
    "SP001": 15000,
    "SP002": 25000,
    "SP003": 42000,
}


def ham_tra_cuu_ma(ten_hang):
    return BANG_MA.get(ten_hang)


def ham_tra_cuu_gia_theo_ma(ma_sp):
    return BANG_GIA_THEO_MA.get(ma_sp)


CAC_CONG_CU = {
    "tra_cuu_ma": ham_tra_cuu_ma,
    "tra_cuu_gia_theo_ma": ham_tra_cuu_gia_theo_ma,
}


def llm_mo_phong_react(ten_hang, vong, lich_su_quan_sat):
    if vong == 0:
        return {"goi_ham": "tra_cuu_ma", "tham_so": {"ten_hang": ten_hang}}
    if vong == 1:
        ma_sp = lich_su_quan_sat["tra_cuu_ma"]
        return {"goi_ham": "tra_cuu_gia_theo_ma", "tham_so": {"ma_sp": ma_sp}}
    gia = lich_su_quan_sat["tra_cuu_gia_theo_ma"]
    return {"tra_loi_cuoi": gia}


def chay_react(ten_hang, so_vong_toi_da=5):
    lich_su_quan_sat = {}
    for vong in range(so_vong_toi_da):
        chi_thi = llm_mo_phong_react(ten_hang, vong, lich_su_quan_sat)
        if "tra_loi_cuoi" in chi_thi:
            return chi_thi["tra_loi_cuoi"], vong + 1
        ten_ham = chi_thi["goi_ham"]
        ham = CAC_CONG_CU[ten_ham]
        ket_qua = ham(**chi_thi["tham_so"])
        lich_su_quan_sat[ten_ham] = ket_qua
    return None, so_vong_toi_da


dap_tra_sua, so_vong_tra_sua = chay_react("tra_sua")
print(dap_tra_sua, so_vong_tra_sua)
```

```text title=readonly
42000 3
```

`chay_react("tra_sua")` chạy đúng `3` vòng: vòng `0` gọi `tra_cuu_ma` (kết
quả `"SP003"`, lưu vào `lich_su_quan_sat`), vòng `1` gọi
`tra_cuu_gia_theo_ma` với ĐÚNG mã vừa tra được (kết quả `42000`), vòng `2`
nhận chỉ thị `"tra_loi_cuoi"` và dừng. Kết quả `42000` khớp đúng giá thật
của `tra_sua` trong `BANG_GIA_THEO_MA["SP003"]`.
::::

::::predict{#doan_gioi_han_vong commitOnce}
`chay_react("tra_sua")` (không truyền `so_vong_toi_da`, dùng mặc định `5`)
chạy đúng `3` vòng rồi dừng ở chỉ thị `"tra_loi_cuoi"`.

**Trước khi chạy thử**, bạn đoán: nếu gọi `chay_react("tra_sua",
so_vong_toi_da=1)` (giới hạn CHỈ `1` vòng), kết quả sẽ là gì?

:::opt{correct}
`(None, 1)` — vòng lặp `for vong in range(1)` chỉ chạy ĐÚNG một lần
(`vong=0`): gọi `tra_cuu_ma`, lưu kết quả vào `lich_su_quan_sat`, rồi hết
vòng lặp vì `range(1)` không còn giá trị nào tiếp theo. Hàm chưa hề tới
được vòng `1` (nơi mới gọi `tra_cuu_gia_theo_ma`) hay vòng `2` (nơi mới có
`"tra_loi_cuoi"`), nên rơi vào dòng cuối `return None, so_vong_toi_da`
:::

:::opt
Sẽ báo lỗi `KeyError` vì `lich_su_quan_sat` chưa có khoá
`"tra_cuu_gia_theo_ma"` khi hàm cố đọc nó
::why
Gần đúng ở việc bạn để ý ĐÚNG một rủi ro có thật trong code: dòng
`gia = lich_su_quan_sat["tra_cuu_gia_theo_ma"]` (ở nhánh `vong` không phải
`0` hay `1`) THẬT SỰ sẽ ném `KeyError` nếu chạy tới đó mà chưa có khoá này.

Chỗ lệch: với `so_vong_toi_da=1`, vòng lặp `for vong in range(1)` CHỈ lặp
với `vong=0` — không bao giờ chạy tới nhánh đọc
`lich_su_quan_sat["tra_cuu_gia_theo_ma"]` (nhánh đó chỉ chạm tới khi
`vong` là `2` trở lên). Vòng lặp kết thúc BÌNH THƯỜNG (hết `range`, không
phải do lỗi) trước khi tới được dòng có nguy cơ đó.
::
:::

:::opt
Sẽ trả về `"SP003"` (mã sản phẩm) — vì đó là kết quả THẬT đã tính được ở
vòng duy nhất đã chạy
::why
Gần đúng ở việc bạn nhớ đúng: `"SP003"` THẬT SỰ được tính ra và lưu vào
`lich_su_quan_sat["tra_cuu_ma"]` trong vòng `0`.

Chỗ lệch: `chay_react` không có nhánh nào trả về một GIÁ TRỊ TRUNG GIAN từ
`lich_su_quan_sat` khi hết vòng lặp — dòng cuối cùng LUÔN là
`return None, so_vong_toi_da`, bất kể `lich_su_quan_sat` đã tích luỹ được
gì. Chỉ có ĐÚNG một cách để hàm trả về một giá trị khác `None`: gặp chỉ
thị `"tra_loi_cuoi"` bên trong vòng lặp.
::
:::
::::

::::code{#viet_chay_react}
Hoàn thiện `chay_react`: nhận diện đúng chỉ thị trả lời cuối để dừng vòng
lặp, và lưu đúng kết quả công cụ vào `lich_su_quan_sat` theo TÊN hàm vừa
gọi.

```python title=starter
BANG_MA = {
    "banh_mi": "SP001",
    "ca_phe": "SP002",
    "tra_sua": "SP003",
}

BANG_GIA_THEO_MA = {
    "SP001": 15000,
    "SP002": 25000,
    "SP003": 42000,
}


def ham_tra_cuu_ma(ten_hang):
    return BANG_MA.get(ten_hang)


def ham_tra_cuu_gia_theo_ma(ma_sp):
    return BANG_GIA_THEO_MA.get(ma_sp)


CAC_CONG_CU = {
    "tra_cuu_ma": ham_tra_cuu_ma,
    "tra_cuu_gia_theo_ma": ham_tra_cuu_gia_theo_ma,
}


def llm_mo_phong_react(ten_hang, vong, lich_su_quan_sat):
    if vong == 0:
        return {"goi_ham": "tra_cuu_ma", "tham_so": {"ten_hang": ten_hang}}
    if vong == 1:
        ma_sp = lich_su_quan_sat["tra_cuu_ma"]
        return {"goi_ham": "tra_cuu_gia_theo_ma", "tham_so": {"ma_sp": ma_sp}}
    gia = lich_su_quan_sat["tra_cuu_gia_theo_ma"]
    return {"tra_loi_cuoi": gia}


def chay_react(ten_hang, so_vong_toi_da=5):
    lich_su_quan_sat = {}
    for vong in range(so_vong_toi_da):
        chi_thi = llm_mo_phong_react(ten_hang, vong, lich_su_quan_sat)
        if ___ in chi_thi:                        # "tra_loi_cuoi"
            return chi_thi["tra_loi_cuoi"], vong + 1
        ten_ham = chi_thi["goi_ham"]
        ham = CAC_CONG_CU[ten_ham]
        ket_qua = ham(**chi_thi["tham_so"])
        lich_su_quan_sat[___] = ket_qua            # ten_ham
    return None, so_vong_toi_da


dap_tra_sua, so_vong_tra_sua = chay_react("tra_sua")
print(dap_tra_sua, so_vong_tra_sua)
```

```python title=solution
BANG_MA = {
    "banh_mi": "SP001",
    "ca_phe": "SP002",
    "tra_sua": "SP003",
}

BANG_GIA_THEO_MA = {
    "SP001": 15000,
    "SP002": 25000,
    "SP003": 42000,
}


def ham_tra_cuu_ma(ten_hang):
    return BANG_MA.get(ten_hang)


def ham_tra_cuu_gia_theo_ma(ma_sp):
    return BANG_GIA_THEO_MA.get(ma_sp)


CAC_CONG_CU = {
    "tra_cuu_ma": ham_tra_cuu_ma,
    "tra_cuu_gia_theo_ma": ham_tra_cuu_gia_theo_ma,
}


def llm_mo_phong_react(ten_hang, vong, lich_su_quan_sat):
    if vong == 0:
        return {"goi_ham": "tra_cuu_ma", "tham_so": {"ten_hang": ten_hang}}
    if vong == 1:
        ma_sp = lich_su_quan_sat["tra_cuu_ma"]
        return {"goi_ham": "tra_cuu_gia_theo_ma", "tham_so": {"ma_sp": ma_sp}}
    gia = lich_su_quan_sat["tra_cuu_gia_theo_ma"]
    return {"tra_loi_cuoi": gia}


def chay_react(ten_hang, so_vong_toi_da=5):
    lich_su_quan_sat = {}
    for vong in range(so_vong_toi_da):
        chi_thi = llm_mo_phong_react(ten_hang, vong, lich_su_quan_sat)
        if "tra_loi_cuoi" in chi_thi:
            return chi_thi["tra_loi_cuoi"], vong + 1
        ten_ham = chi_thi["goi_ham"]
        ham = CAC_CONG_CU[ten_ham]
        ket_qua = ham(**chi_thi["tham_so"])
        lich_su_quan_sat[ten_ham] = ket_qua
    return None, so_vong_toi_da


dap_tra_sua, so_vong_tra_sua = chay_react("tra_sua")
print(dap_tra_sua, so_vong_tra_sua)
```

```python title=test
assert (dap_tra_sua, so_vong_tra_sua) == (42000, 3), f"tra_sua phai la (42000, 3) -- dang ra {(dap_tra_sua, so_vong_tra_sua)}"

BANG_GIA_DUNG = {"banh_mi": 15000, "ca_phe": 25000, "tra_sua": 42000}
for ten_hang, gia_dung in BANG_GIA_DUNG.items():
    dap, so_vong = chay_react(ten_hang)
    assert dap == gia_dung, f"{ten_hang}: gia tra ve phai la {gia_dung} -- dang ra {dap!r}"
    assert so_vong == 3, f"{ten_hang}: phai dung DUNG 3 vong -- dang ra {so_vong}"

# bien: gioi han vong qua som (1 hoac 2 vong) -- chua kip tra loi, phai
# tra ve None, KHONG duoc bao loi
dap_1_vong, so_vong_1 = chay_react("tra_sua", so_vong_toi_da=1)
assert dap_1_vong is None, f"gioi han 1 vong phai tra ve None -- dang ra {dap_1_vong!r}"
assert so_vong_1 == 1, f"gioi han 1 vong phai dung o so_vong_toi_da=1 -- dang ra {so_vong_1}"

dap_2_vong, so_vong_2 = chay_react("tra_sua", so_vong_toi_da=2)
assert dap_2_vong is None, f"gioi han 2 vong van chua du, phai tra ve None -- dang ra {dap_2_vong!r}"
assert so_vong_2 == 2, f"gioi han 2 vong phai dung o so_vong_toi_da=2 -- dang ra {so_vong_2}"

# bien: gioi han vong VUA DU (3) -- phai thanh cong giong mac dinh
dap_3_vong, so_vong_3 = chay_react("tra_sua", so_vong_toi_da=3)
assert dap_3_vong == 42000, f"gioi han dung 3 vong phai du de tra loi -- dang ra {dap_3_vong!r}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cùng nằm trong `chay_react`. Chỗ đầu kiểm tra chỉ thị hiện tại có phải chỉ thị TRẢ LỜI CUỐI không — dùng đúng tên khoá đã thấy trong nhánh cuối của `llm_mo_phong_react` (`{"tra_loi_cuoi": gia}`). Chỗ hai lưu `ket_qua` vào `lich_su_quan_sat` — dùng làm khoá đúng TÊN HÀM vừa gọi (biến đã có sẵn ngay phía trên, không phải một chuỗi mới).
- kind: strategy
  body: 'Chỗ đầu: `"tra_loi_cuoi"` (cho dòng `if "tra_loi_cuoi" in chi_thi:`). Chỗ hai: `ten_ham` (cho dòng `lich_su_quan_sat[ten_ham] = ket_qua`).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `"tra_loi_cuoi"` và `ten_ham`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai kiem tra DUNG khoa "tra_loi_cuoi" de nhan dien chi thi tra loi cuoi (khong duoc doi thanh mot chuoi khac), VA phai luu ket_qua vao lich_su_quan_sat bang KHOA la bien "ten_ham" (khong duoc dung mot chuoi co dinh hay bien khac)
  requireAst:
  - kind: has-literal, target: "tra_loi_cuoi", min: 3
  - kind: uses-name, target: "ten_ham", min: 2
  # Da thu that (goi _dem tren code trich tu solution, khong doan tay).
  # has-literal "tra_loi_cuoi"=3: mot lan CO SAN (khoa trong dict tra ve
  # cuoi cung cua llm_mo_phong_react, "{"tra_loi_cuoi": gia}"), mot lan CO
  # SAN (chi_thi["tra_loi_cuoi"] trong nhanh return cua chay_react), mot
  # lan la cho trong 1. Neu chi dat min=1 (ngay tho), mot mutant xoa cho
  # trong 1 (vi du doi dieu kien thanh "if chi_thi.get("goi_ham") is
  # None:") VAN qua duoc vi con lai hai lan "tra_loi_cuoi" khac trong
  # boilerplate -- day la GOTCHA "boilerplate-threshold-masking"; dat dung
  # min=3 (tong THAT) moi chan duoc mutant nay -- va no cung bi chan boi
  # tests (dieu kien do khong con dung voi cau truc chi thi hien tai).
  # uses-name "ten_ham"=2: mot lan CO SAN (ham = CAC_CONG_CU[ten_ham]), mot
  # lan la cho trong 2. Dien bua cho trong 2 thanh mot chuoi co dinh (vi du
  # lich_su_quan_sat["tra_cuu_ma"] = ket_qua, luon dung CHUOI CO DINH thay
  # vi bien) lam uses-name "ten_ham" tut ve 1 -- duoi nguong min=2, bi
  # chan; dong thoi bi chan boi tests (vong thu hai can khoa
  # "tra_cuu_gia_theo_ma" trong lich_su_quan_sat, se bi KeyError neu
  # cho trong luon ghi de "tra_cuu_ma").
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^42000 3\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba vòng: gọi công cụ `1`, gọi công cụ `2`, trả lời cuối — đúng `3`, không
hơn không kém, và giới hạn vòng quá sớm (`1` hay `2`) cho `None`, xác nhận
vòng lặp thật sự CẦN đủ bước, không "nhảy tắt". Bài sau (BOSS quý) ráp lại
TẤT CẢ: một "trợ lý" nhận nhiều câu hỏi khác nhau — có câu cần một công cụ,
có câu cần ReAct hai bước — chạy vòng lặp đầy đủ, ép kiểm schema và retry
trên mỗi phản hồi dọc đường, rồi đo cải thiện so với một phiên bản ngây thơ.
::::

::::reflect{#nghi-lai}
ReAct không phải "gọi công cụ nhiều lần cho chắc" — nó là một vòng lặp có
ĐIỀU KIỆN DỪNG rõ ràng (chỉ thị `"tra_loi_cuoi"`) và một GIỚI HẠN an toàn
(`so_vong_toi_da`) để không lặp vô hạn khi mô hình không bao giờ tới được
điều kiện dừng. Ba vòng đo được ở `tra_sua` không phải một con số tuỳ ý —
nó phản ánh đúng CẤU TRÚC phụ thuộc của bài toán: bước hai (tra giá theo
mã) không thể chạy trước khi có kết quả của bước một (tra mã theo tên).
Bài sau (BOSS quý của quest) ráp mọi kỹ thuật đã học — đầu ra có cấu trúc
(bài `1`), schema và retry (bài `2`), function calling (bài `3`), và vòng
lặp ReAct (bài này) — thành một "trợ lý" xử lý được CẢ câu hỏi một bước
lẫn câu hỏi hai bước, đo cải thiện bằng số so với một phiên bản không có
ReAct và không có retry.
::::

::::checkpoint{mastery=0.85}
::::
