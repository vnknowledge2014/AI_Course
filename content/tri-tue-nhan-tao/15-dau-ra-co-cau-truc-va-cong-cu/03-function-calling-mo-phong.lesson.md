---
id: tri-tue-nhan-tao.dau-ra-co-cau-truc-va-cong-cu.function-calling-mo-phong
title: "Function calling mô phỏng: LLM chỉ ra hàm, code thực thi hàm"
summary: "LLM mo phong llm_mo_phong_goi_ham(cau_hoi) KHONG tra loi truc tiep -- no tra ve mot CHI THI CO CAU TRUC: {'goi_ham': 'tra_cuu_gia', 'tham_so': {'ten_hang': 'banh_mi'}}. Ham xu_ly_chi_thi doc chi thi nay, tra bang CAC_CONG_CU = {'tra_cuu_gia': ham_tra_cuu_gia} de tim DUNG ham Python, roi GOI THAT ham do -- ket qua tra ve la du lieu THAT tu bang tra cuu, khong phai LLM tu bia. Do tren 3 cau hoi: goi ham dung 3/3. Doi chieu voi mot LLM mo phong 'doan bua' (khong goi cong cu, tu doan mot gia co dinh): chi dung 1/3 -- trung hop voi dung 1 mat hang. Ca hai con so deu chay that."
locale: vi
track: tri-tue-nhan-tao
module: dau-ra-co-cau-truc-va-cong-cu
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 9
teaches: [ai.function-calling-mo-phong]
requires: [ai.rang-buoc-theo-schema-va-retry]
concepts: [ai.function-calling-mo-phong]
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

::::byte{trigger=enter mood=curious pose=point-stage}
Hai bài trước: LLM mô phỏng trả lời TRỰC TIẾP (một số, một dict), và code
chỉ kiểm tra câu trả lời đó. Bài này đổi vai trò: LLM mô phỏng không trả
lời — nó CHỈ RA một hàm cần gọi, và code phía người gọi mới là bên THỰC
SỰ chạy hàm đó để lấy dữ liệu.
::::

::::explain{#function_calling_la_gi}
Một LLM (mô phỏng hay thật) không có quyền truy cập TRỰC TIẾP vào dữ liệu
sống — giá tiền hiện tại, thời tiết hôm nay, số dư tài khoản. Nó chỉ có
thể SUY LUẬN trên ngôn ngữ. **Function calling** giải quyết đúng giới hạn
đó bằng một quy ước: thay vì trả lời trực tiếp, LLM trả về một **chỉ thị
gọi hàm** — một cấu trúc có tên hàm cần gọi và tham số cần truyền, ví dụ:

```
{"goi_ham": "tra_cuu_gia", "tham_so": {"ten_hang": "banh_mi"}}
```

Code phía người gọi ĐỌC chỉ thị này, tra trong một **bảng công cụ** (một
`dict` ánh xạ tên hàm sang hàm Python thật, ví dụ
`CAC_CONG_CU = {"tra_cuu_gia": ham_tra_cuu_gia}`), tìm đúng hàm, rồi GỌI
THẬT hàm đó với đúng tham số. Kết quả trả về là dữ liệu THẬT từ bảng tra
cứu — không phải một con số LLM tự đoán hay tự bịa ra.

Điểm mấu chốt phân biệt function calling với hai bài trước: LLM không còn
là nguồn CUỐI CÙNG của câu trả lời. Nó chỉ chọn ĐÚNG công cụ và ĐÚNG tham
số; công cụ (một hàm Python tra bảng cố định trong track này, đóng vai một
"API ngoài" — không phải network thật) mới là nguồn dữ liệu thật.
::::

::::example{#do_goi_ham_that}
Ba câu hỏi về giá ba mặt hàng, xử lý qua chỉ thị gọi hàm, đối chiếu với một
LLM mô phỏng "đoán bừa" không gọi công cụ nào:

```python title=readonly
BANG_GIA = {
    "banh_mi": 15000,
    "ca_phe": 25000,
    "tra_sua": 42000,
}


def ham_tra_cuu_gia(ten_hang):
    return BANG_GIA.get(ten_hang)


CAC_CONG_CU = {
    "tra_cuu_gia": ham_tra_cuu_gia,
}


def llm_mo_phong_goi_ham(cau_hoi):
    cau_hoi_thuong = cau_hoi.lower()
    if "banh mi" in cau_hoi_thuong:
        return {"goi_ham": "tra_cuu_gia", "tham_so": {"ten_hang": "banh_mi"}}
    if "ca phe" in cau_hoi_thuong:
        return {"goi_ham": "tra_cuu_gia", "tham_so": {"ten_hang": "ca_phe"}}
    if "tra sua" in cau_hoi_thuong:
        return {"goi_ham": "tra_cuu_gia", "tham_so": {"ten_hang": "tra_sua"}}
    return {"goi_ham": None, "tham_so": {}}


def xu_ly_chi_thi(chi_thi):
    ten_ham = chi_thi["goi_ham"]
    if ten_ham is None or ten_ham not in CAC_CONG_CU:
        return None
    ham = CAC_CONG_CU[ten_ham]
    return ham(**chi_thi["tham_so"])


def llm_mo_phong_doan_bua(cau_hoi):
    # KHONG goi cong cu nao -- tu doan mot gia CO DINH (khong ngau nhien)
    return 25000


DANH_SACH_CAU_HOI = [
    "Gia banh mi bao nhieu?",
    "Gia ca phe bao nhieu?",
    "Gia tra sua bao nhieu?",
]


def dem_dung_goi_ham(danh_sach_cau_hoi):
    so_dung = 0
    for cau_hoi in danh_sach_cau_hoi:
        chi_thi = llm_mo_phong_goi_ham(cau_hoi)
        ket_qua = xu_ly_chi_thi(chi_thi)
        ten_hang = chi_thi["tham_so"]["ten_hang"]
        if ket_qua == BANG_GIA[ten_hang]:
            so_dung += 1
    return so_dung


def dem_dung_doan_bua(danh_sach_cau_hoi):
    so_dung = 0
    for cau_hoi in danh_sach_cau_hoi:
        ket_qua = llm_mo_phong_doan_bua(cau_hoi)
        for ten_hang in BANG_GIA:
            if ten_hang.replace("_", " ") in cau_hoi.lower():
                if ket_qua == BANG_GIA[ten_hang]:
                    so_dung += 1
                break
    return so_dung


so_dung_goi_ham = dem_dung_goi_ham(DANH_SACH_CAU_HOI)
so_dung_doan_bua = dem_dung_doan_bua(DANH_SACH_CAU_HOI)

print(so_dung_goi_ham, "/", len(DANH_SACH_CAU_HOI))
print(so_dung_doan_bua, "/", len(DANH_SACH_CAU_HOI))
```

```text title=readonly
3 / 3
1 / 3
```

Function calling: đúng `3/3` — mỗi câu hỏi được dịch thành đúng chỉ thị
gọi hàm, `xu_ly_chi_thi` gọi THẬT `ham_tra_cuu_gia`, kết quả luôn khớp
`BANG_GIA`. Đoán bừa (không gọi công cụ): chỉ đúng `1/3` — con số đoán cố
định `25000` tình cờ trùng giá `ca_phe`, còn `banh_mi` (`15000`) và
`tra_sua` (`42000`) đều sai, vì không có cách nào "đoán" ra dữ liệu mà
không thực sự tra cứu.
::::

::::predict{#doan_khong_tim_thay_cong_cu commitOnce}
`llm_mo_phong_goi_ham` trả về `{"goi_ham": None, "tham_so": {}}` khi câu
hỏi không khớp mặt hàng nào đã biết (ví dụ hỏi về một món không có trong
`BANG_GIA`).

**Trước khi chạy thử**, bạn đoán: `xu_ly_chi_thi({"goi_ham": None,
"tham_so": {}})` trả về gì?

:::opt{correct}
`None` — nhánh `if ten_ham is None or ten_ham not in CAC_CONG_CU:` bắt
đúng trường hợp `ten_ham` (lấy từ `chi_thi["goi_ham"]`) là `None`, nên hàm
trả về `None` ngay, không cố tra `CAC_CONG_CU` hay gọi bất kỳ hàm nào
:::

:::opt
Sẽ báo lỗi `KeyError`, vì `CAC_CONG_CU` không có khoá `None`
::why
Gần đúng ở việc bạn để ý `CAC_CONG_CU` THẬT SỰ không có khoá `None` — tra
`CAC_CONG_CU[None]` trực tiếp sẽ ném `KeyError` thật.

Chỗ lệch: `xu_ly_chi_thi` không tra `CAC_CONG_CU` một cách mù quáng — điều
kiện `if ten_ham is None or ten_ham not in CAC_CONG_CU:` kiểm TRƯỚC xem
`ten_ham` có phải `None` (hoặc không có trong bảng) hay không, và trả về
`None` ngay trong nhánh đó — dòng `ham = CAC_CONG_CU[ten_ham]` không bao
giờ chạy tới khi `ten_ham` là `None`.
::
:::

:::opt
Hàm sẽ gọi một công cụ MẶC ĐỊNH (ví dụ công cụ đầu tiên trong
`CAC_CONG_CU`) thay vì trả về `None`
::why
Gần đúng ở trực giác "hệ thống nên có phương án dự phòng" — một thiết kế
hợp lý trong một số ngữ cảnh khác.

Chỗ lệch: `xu_ly_chi_thi` không có logic "công cụ mặc định" nào — khi
`ten_ham is None`, nhánh `if` trả về `None` NGAY LẬP TỨC bằng `return None`,
không có bước nào chọn một công cụ khác để thay thế.
::
:::
::::

::::code{#viet_xu_ly_chi_thi}
Hoàn thiện `xu_ly_chi_thi`: tra đúng hàm từ `CAC_CONG_CU` bằng tên trong
chỉ thị, rồi gọi hàm đó với đúng tham số cũng lấy từ chỉ thị.

```python title=starter
BANG_GIA = {
    "banh_mi": 15000,
    "ca_phe": 25000,
    "tra_sua": 42000,
}


def ham_tra_cuu_gia(ten_hang):
    return BANG_GIA.get(ten_hang)


CAC_CONG_CU = {
    "tra_cuu_gia": ham_tra_cuu_gia,
}


def llm_mo_phong_goi_ham(cau_hoi):
    cau_hoi_thuong = cau_hoi.lower()
    if "banh mi" in cau_hoi_thuong:
        return {"goi_ham": "tra_cuu_gia", "tham_so": {"ten_hang": "banh_mi"}}
    if "ca phe" in cau_hoi_thuong:
        return {"goi_ham": "tra_cuu_gia", "tham_so": {"ten_hang": "ca_phe"}}
    if "tra sua" in cau_hoi_thuong:
        return {"goi_ham": "tra_cuu_gia", "tham_so": {"ten_hang": "tra_sua"}}
    return {"goi_ham": None, "tham_so": {}}


def xu_ly_chi_thi(chi_thi):
    ten_ham = chi_thi["goi_ham"]
    if ten_ham is None or ten_ham not in CAC_CONG_CU:
        return None
    ham = CAC_CONG_CU[___]                    # ten_ham
    return ham(**chi_thi[___])                # "tham_so"


def llm_mo_phong_doan_bua(cau_hoi):
    return 25000


DANH_SACH_CAU_HOI = [
    "Gia banh mi bao nhieu?",
    "Gia ca phe bao nhieu?",
    "Gia tra sua bao nhieu?",
]


def dem_dung_goi_ham(danh_sach_cau_hoi):
    so_dung = 0
    for cau_hoi in danh_sach_cau_hoi:
        chi_thi = llm_mo_phong_goi_ham(cau_hoi)
        ket_qua = xu_ly_chi_thi(chi_thi)
        ten_hang = chi_thi["tham_so"]["ten_hang"]
        if ket_qua == BANG_GIA[ten_hang]:
            so_dung += 1
    return so_dung


def dem_dung_doan_bua(danh_sach_cau_hoi):
    so_dung = 0
    for cau_hoi in danh_sach_cau_hoi:
        ket_qua = llm_mo_phong_doan_bua(cau_hoi)
        for ten_hang in BANG_GIA:
            if ten_hang.replace("_", " ") in cau_hoi.lower():
                if ket_qua == BANG_GIA[ten_hang]:
                    so_dung += 1
                break
    return so_dung


so_dung_goi_ham = dem_dung_goi_ham(DANH_SACH_CAU_HOI)
so_dung_doan_bua = dem_dung_doan_bua(DANH_SACH_CAU_HOI)

print(so_dung_goi_ham, "/", len(DANH_SACH_CAU_HOI))
print(so_dung_doan_bua, "/", len(DANH_SACH_CAU_HOI))
```

```python title=solution
BANG_GIA = {
    "banh_mi": 15000,
    "ca_phe": 25000,
    "tra_sua": 42000,
}


def ham_tra_cuu_gia(ten_hang):
    return BANG_GIA.get(ten_hang)


CAC_CONG_CU = {
    "tra_cuu_gia": ham_tra_cuu_gia,
}


def llm_mo_phong_goi_ham(cau_hoi):
    cau_hoi_thuong = cau_hoi.lower()
    if "banh mi" in cau_hoi_thuong:
        return {"goi_ham": "tra_cuu_gia", "tham_so": {"ten_hang": "banh_mi"}}
    if "ca phe" in cau_hoi_thuong:
        return {"goi_ham": "tra_cuu_gia", "tham_so": {"ten_hang": "ca_phe"}}
    if "tra sua" in cau_hoi_thuong:
        return {"goi_ham": "tra_cuu_gia", "tham_so": {"ten_hang": "tra_sua"}}
    return {"goi_ham": None, "tham_so": {}}


def xu_ly_chi_thi(chi_thi):
    ten_ham = chi_thi["goi_ham"]
    if ten_ham is None or ten_ham not in CAC_CONG_CU:
        return None
    ham = CAC_CONG_CU[ten_ham]
    return ham(**chi_thi["tham_so"])


def llm_mo_phong_doan_bua(cau_hoi):
    return 25000


DANH_SACH_CAU_HOI = [
    "Gia banh mi bao nhieu?",
    "Gia ca phe bao nhieu?",
    "Gia tra sua bao nhieu?",
]


def dem_dung_goi_ham(danh_sach_cau_hoi):
    so_dung = 0
    for cau_hoi in danh_sach_cau_hoi:
        chi_thi = llm_mo_phong_goi_ham(cau_hoi)
        ket_qua = xu_ly_chi_thi(chi_thi)
        ten_hang = chi_thi["tham_so"]["ten_hang"]
        if ket_qua == BANG_GIA[ten_hang]:
            so_dung += 1
    return so_dung


def dem_dung_doan_bua(danh_sach_cau_hoi):
    so_dung = 0
    for cau_hoi in danh_sach_cau_hoi:
        ket_qua = llm_mo_phong_doan_bua(cau_hoi)
        for ten_hang in BANG_GIA:
            if ten_hang.replace("_", " ") in cau_hoi.lower():
                if ket_qua == BANG_GIA[ten_hang]:
                    so_dung += 1
                break
    return so_dung


so_dung_goi_ham = dem_dung_goi_ham(DANH_SACH_CAU_HOI)
so_dung_doan_bua = dem_dung_doan_bua(DANH_SACH_CAU_HOI)

print(so_dung_goi_ham, "/", len(DANH_SACH_CAU_HOI))
print(so_dung_doan_bua, "/", len(DANH_SACH_CAU_HOI))
```

```python title=test
chi_thi_bm = llm_mo_phong_goi_ham("Gia banh mi bao nhieu?")
assert chi_thi_bm == {"goi_ham": "tra_cuu_gia", "tham_so": {"ten_hang": "banh_mi"}}, f"chi thi cho banh mi sai -- dang ra {chi_thi_bm!r}"
assert xu_ly_chi_thi(chi_thi_bm) == 15000, f"xu ly chi thi banh mi phai la 15000 -- dang ra {xu_ly_chi_thi(chi_thi_bm)!r}"

assert xu_ly_chi_thi({"goi_ham": None, "tham_so": {}}) is None, "chi thi goi_ham=None phai tra ve None"
assert xu_ly_chi_thi({"goi_ham": "khong_ton_tai", "tham_so": {}}) is None, "ten ham khong co trong CAC_CONG_CU phai tra ve None"

assert so_dung_goi_ham == 3, f"function calling phai dung ca 3/3 -- dang ra {so_dung_goi_ham}"
assert so_dung_doan_bua == 1, f"doan bua phai chi dung 1/3 -- dang ra {so_dung_doan_bua}"

# bien: goi dung ham tra_cuu_gia THAT (khong qua xu_ly_chi_thi) voi mot ten
# hang khong ton tai -- BANG_GIA.get tra ve None thay vi loi KeyError
assert ham_tra_cuu_gia("khong_ton_tai") is None, "tra cuu mot mat hang khong co phai tra ve None, khong loi"

# bien: xac nhan CAC_CONG_CU tra dung ham (khong phai mot ham khac trung
# ten bien)
assert CAC_CONG_CU["tra_cuu_gia"] is ham_tra_cuu_gia, "CAC_CONG_CU phai anh xa dung toi ham_tra_cuu_gia"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cùng nằm trong `xu_ly_chi_thi`. Chỗ đầu tra bảng `CAC_CONG_CU` bằng đúng biến đã đọc được ở dòng trên (`ten_ham`), không phải tra lại `chi_thi["goi_ham"]` từ đầu. Chỗ hai lấy từ điển tham số ra khỏi `chi_thi` — dùng đúng tên trường đã thấy trong mọi chỉ thị ở ví dụ trên (`"tham_so"`).
- kind: strategy
  body: 'Chỗ đầu: `ten_ham` (cho dòng `ham = CAC_CONG_CU[ten_ham]`). Chỗ hai: `"tham_so"` (cho dòng `return ham(**chi_thi["tham_so"])`).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `ten_ham` và `"tham_so"`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tra CAC_CONG_CU bang DUNG bien "ten_ham" (khong duoc tra lai chi_thi["goi_ham"] hay mot ten khac), VA phai doc DUNG truong "tham_so" tu chi_thi (khong duoc doi thanh mot chuoi khac)
  requireAst:
  - kind: uses-name, target: "ten_ham", min: 3
  - kind: has-literal, target: "tham_so", min: 6
  # Da thu that (goi _dem tren code trich tu solution, khong doan tay).
  # uses-name "ten_ham"=3: hai lan CO SAN (trong dieu kien "if ten_ham is
  # None or ten_ham not in CAC_CONG_CU:" -- doc ten_ham hai lan), mot lan
  # la cho trong 1 (CAC_CONG_CU[ten_ham]). Neu chi dat min=1 (ngay tho), mot
  # mutant doi cho trong 1 thanh tra lai "chi_thi["goi_ham"]" truc tiep (bo
  # qua bien ten_ham) VAN qua duoc vi con lai hai lan doc ten_ham trong
  # dieu kien -- day la GOTCHA "boilerplate-threshold-masking"; dat dung
  # min=3 (tong THAT) moi chan duoc mutant nay. Ve mat hanh vi, mutant nay
  # THAT RA van dung (chi_thi["goi_ham"] va ten_ham la cung mot gia tri) --
  # static rieng day la luat duy nhat phan biet duoc, khong output/tests
  # nao bat duoc vi ket qua cuoi cung giong het.
  # has-literal "tham_so"=6: bon lan CO SAN (khoa "tham_so" xuat hien trong
  # bon chi thi tra ve cua llm_mo_phong_goi_ham), mot lan CO SAN
  # (chi_thi["tham_so"]["ten_hang"] trong dem_dung_goi_ham), mot lan la cho
  # trong 2. Dien bua cho trong 2 thanh mot khoa khac (vi du "goi_ham" hay
  # "params") lam has-literal "tham_so" tut ve 5 -- duoi nguong min=6, bi
  # chan; dong thoi bi chan boi tests/run (chi_thi["goi_ham"] la mot chuoi,
  # khong phai dict, "**" tren no se nem TypeError).
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^3 / 3\\n1 / 3\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Function calling: `3/3` — LLM mô phỏng chỉ CHỌN công cụ, hàm THẬT mới lấy
dữ liệu. Đoán bừa: `1/3` — không có công cụ nào, không có cách nào biết
dữ liệu thật. Bài sau xử lý một trường hợp cần HAI lần gọi công cụ LIÊN
TIẾP — kết quả bước một là ĐẦU VÀO của bước hai, không thể gộp một bước.
::::

::::reflect{#nghi-lai}
Function calling tách bạch hai vai trò trước đây bị trộn lẫn: LLM (mô
phỏng hay thật) chỉ giỏi CHỌN — chọn đúng công cụ, đúng tham số, dựa trên
ngôn ngữ của câu hỏi; còn TRA DỮ LIỆU THẬT là việc của công cụ, một hàm
Python (hay một API thật ở hệ thống thật) không hề "đoán". `3/3` so với
`1/3` không phải vì LLM mô phỏng bài này "thông minh hơn" bài `đoán bừa` —
chính vì nó KHÔNG TỰ TRẢ LỜI nữa, mà giao việc tra dữ liệu cho đúng hàm.
Bài sau mở rộng đúng cơ chế này sang một tình huống LLM không thể trả lời
chỉ bằng MỘT lần gọi công cụ: cần tra một MÃ sản phẩm trước, rồi mới dùng
MÃ đó tra giá — hai bước, bước hai phụ thuộc kết quả của bước một.
::::

::::checkpoint{mastery=0.85}
::::
