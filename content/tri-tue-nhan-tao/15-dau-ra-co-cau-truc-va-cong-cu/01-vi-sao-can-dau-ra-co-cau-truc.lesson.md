---
id: tri-tue-nhan-tao.dau-ra-co-cau-truc-va-cong-cu.vi-sao-can-dau-ra-co-cau-truc
title: "Vì sao cần đầu ra có cấu trúc: văn xuôi tự do so với JSON"
summary: "Mot LLM mo phong tra loi cau hoi gia hang bang VAN XUOI TU DO -- ba cach dien dat khac nhau tuy mat hang, mot cach dat so tien day du, mot cach dung don vi 'nghin', mot cach chen mot so KHONG lien quan truoc gia that. Trich so bang regex ngay tho (lay SO DAU TIEN tim thay) chi dung 1/3 mat hang. Doi chieu: LLM mo phong tra ve TRUC TIEP mot dict co truong co dinh {'gia':..., 'don_vi':...} -- dap_an['gia'] dung 3/3, khong doan mo ho. Ca hai con so (1/3 va 3/3) deu chay that bang Python."
locale: vi
track: tri-tue-nhan-tao
module: dau-ra-co-cau-truc-va-cong-cu
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 9
teaches: [ai.vi-sao-can-dau-ra-co-cau-truc]
requires: [ai.boss-so-sanh-bon-chien-luoc]
concepts: [ai.vi-sao-can-dau-ra-co-cau-truc]
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
Quest trước dùng THẬT `Tensor`/`khối_transformer` đã huấn luyện để so bốn
chiến lược giải mã. Quest này quay lại **LLM mô phỏng** (hàm Python thuần,
tra bảng cố định, không mạng nơ-ron) — nhưng đổi câu hỏi: khi một LLM cần
TRẢ LỜI cho một chương trình khác đọc tiếp, đầu ra nên có HÌNH DẠNG gì?
::::

::::explain{#vi_sao_can_cau_truc}
Track T8.4 (`prompt-engineering-nen-tang`, q8.4a) đã đo: cùng một chỉ thị,
LLM mô phỏng có thể trả lời bằng một trong NHIỀU cách diễn đạt khác nhau —
mơ hồ với NGƯỜI đọc, nhưng còn mơ hồ hơn với MỘT CHƯƠNG TRÌNH cần đọc câu
trả lời đó bằng code.

Xét một trợ lý mô phỏng trả lời câu hỏi "giá một món hàng bao nhiêu". Nếu
câu trả lời là **văn xuôi tự do**, một pipeline cần con số phải PARSE nó —
thường bằng regex, tìm chữ số trong câu. Vấn đề: LLM (thật hay mô phỏng)
không cam kết một ĐỊNH DẠNG cố định cho văn xuôi. Ba mặt hàng có thể sinh
ba câu trả lời với chữ số nằm ở BA VỊ TRÍ Ý NGHĨA khác nhau:

> Một câu ghi thẳng số tiền đầy đủ — regex trích ĐÚNG.
>
> Một câu chèn một số KHÔNG liên quan (ví dụ thứ tự món trong danh sách)
> ĐỨNG TRƯỚC giá — regex "lấy số đầu tiên" trích SAI, lấy nhầm số thứ tự.
>
> Một câu rút gọn số tiền theo đơn vị "nghìn" (ví dụ viết "`42`" thay vì
> "`42000`") — regex trích ra một con số CÓ THẬT trong câu, nhưng SAI vì
> lệch quy mô — không phải giá trị đồng thật.

Ba lỗi khác nhau, cùng một nguyên nhân: định dạng văn xuôi không cố định,
nên MỘT luật trích xuất không thể đúng cho MỌI cách diễn đạt.

**Đầu ra có cấu trúc** giải quyết đúng vấn đề đó: thay vì trả về một câu
văn, LLM trả về TRỰC TIẾP một giá trị có trường cố định — trong Python, một
`dict` đại diện cho một đối tượng JSON đã được phân tích sẵn, ví dụ
`{"gia": 42000, "don_vi": "dong"}`. Không cần đoán số nằm ở đâu trong câu:
`dap_an["gia"]` luôn là con số, luôn ở đúng một chỗ, bất kể mặt hàng nào.
::::

::::example{#do_tu_do_vs_cau_truc}
Ba mặt hàng, ba cách diễn đạt văn xuôi khác nhau, đối chiếu với đầu ra có
cấu trúc:

```python title=readonly
import re

BANG_GIA = {
    "banh_mi": 15000,
    "ca_phe": 25000,
    "tra_sua": 42000,
}

DANH_SACH_HANG = ["banh_mi", "ca_phe", "tra_sua"]


def llm_mo_phong_van_xuoi(ten_hang):
    gia = BANG_GIA[ten_hang]
    if ten_hang == "banh_mi":
        return f"Gia {ten_hang} la {gia} dong, co the re hon mot chut."
    if ten_hang == "ca_phe":
        return f"Mon nay dat hang thu 2, gia khoang {gia} dong."
    gia_nghin = gia // 1000
    return f"Gia la khoang {gia_nghin} nghin dong, co the re hon."


def llm_mo_phong_co_cau_truc(ten_hang):
    gia = BANG_GIA[ten_hang]
    return {"gia": gia, "don_vi": "dong", "mat_hang": ten_hang}


def trich_gia_tu_do(text):
    cac_so = re.findall(r"\d+", text)
    return int(cac_so[0]) if cac_so else None


def dem_dung(danh_sach_hang):
    so_dung_tu_do = 0
    so_dung_co_cau_truc = 0
    for ten_hang in danh_sach_hang:
        gia_dung = BANG_GIA[ten_hang]

        van_xuoi = llm_mo_phong_van_xuoi(ten_hang)
        gia_tu_do = trich_gia_tu_do(van_xuoi)
        if gia_tu_do == gia_dung:
            so_dung_tu_do += 1

        dap_an = llm_mo_phong_co_cau_truc(ten_hang)
        gia_co_cau_truc = dap_an["gia"]
        if gia_co_cau_truc == gia_dung:
            so_dung_co_cau_truc += 1

    return so_dung_tu_do, so_dung_co_cau_truc


so_dung_tu_do, so_dung_co_cau_truc = dem_dung(DANH_SACH_HANG)
print(so_dung_tu_do, "/", len(DANH_SACH_HANG))
print(so_dung_co_cau_truc, "/", len(DANH_SACH_HANG))
```

```text title=readonly
1 / 3
3 / 3
```

Văn xuôi tự do: đúng `1/3` — chỉ `banh_mi` (`"Gia banh_mi la 15000 dong..."`)
ghi số tiền đầy đủ ngay từ đầu câu nên regex trích đúng. `ca_phe` sai vì
regex lấy nhầm số `2` (thứ tự món) đứng trước giá `25000`; `tra_sua` sai vì
regex lấy `42` (đơn vị nghìn) thay vì `42000`. Đầu ra có cấu trúc: đúng
`3/3` — `dap_an["gia"]` luôn đọc đúng trường, không phụ thuộc câu chữ xung
quanh viết thế nào.
::::

::::predict{#doan_trich_ca_phe commitOnce}
Câu trả lời văn xuôi cho `ca_phe` là
`"Mon nay dat hang thu 2, gia khoang 25000 dong."` — giá THẬT là `25000`.

**Trước khi chạy thử**, bạn đoán: `trich_gia_tu_do` gọi trên câu này trả về
bao nhiêu?

:::opt{correct}
`2` — `trich_gia_tu_do` dùng `re.findall(r"\d+", text)` rồi lấy PHẦN TỬ ĐẦU
TIÊN của danh sách kết quả; trong câu này, số `2` (từ "hàng thứ `2`") xuất
hiện TRƯỚC số `25000`, nên nó được lấy, không phải giá thật
:::

:::opt
`25000` — vì đó là con số ĐÚNG có mặt trong câu, và hàm trích xuất chắc
chắn tìm ra giá trị có ý nghĩa nhất
::why
Gần đúng ở việc `25000` THẬT SỰ có mặt trong chuỗi văn bản — bạn không đọc
sai dữ liệu.

Chỗ lệch: `trich_gia_tu_do` không có khái niệm "con số có ý nghĩa nhất" —
nó chỉ lấy phần tử `[0]` của danh sách `re.findall` trả về, tức là số XUẤT
HIỆN TRƯỚC TIÊN trong chuỗi theo thứ tự đọc trái sang phải, bất kể số đó
đại diện cho điều gì.
::
:::

:::opt
`None` — vì câu này không có số tiền viết đầy đủ ngay từ đầu như câu của
`banh_mi`, nên hàm không tìm thấy số nào để trích
::why
Gần đúng ở việc bạn để ý câu này có CẤU TRÚC khác câu của `banh_mi` (số
tiền không đứng ngay đầu câu).

Chỗ lệch: `trich_gia_tu_do` trả về `None` chỉ khi `re.findall` không tìm
thấy CHỮ SỐ nào trong toàn bộ chuỗi. Câu này có tới HAI chữ số (`2` và
`25000`), nên danh sách không rỗng — hàm trả về phần tử đầu, không phải
`None`.
::
:::
::::

::::code{#viet_dem_dung}
Hoàn thiện `dem_dung`: so sánh giá trích được bằng regex với giá đúng để
đếm văn xuôi tự do, và đọc đúng trường `"gia"` từ dict để đếm đầu ra có
cấu trúc.

```python title=starter
import re

BANG_GIA = {
    "banh_mi": 15000,
    "ca_phe": 25000,
    "tra_sua": 42000,
}

DANH_SACH_HANG = ["banh_mi", "ca_phe", "tra_sua"]


def llm_mo_phong_van_xuoi(ten_hang):
    gia = BANG_GIA[ten_hang]
    if ten_hang == "banh_mi":
        return f"Gia {ten_hang} la {gia} dong, co the re hon mot chut."
    if ten_hang == "ca_phe":
        return f"Mon nay dat hang thu 2, gia khoang {gia} dong."
    gia_nghin = gia // 1000
    return f"Gia la khoang {gia_nghin} nghin dong, co the re hon."


def llm_mo_phong_co_cau_truc(ten_hang):
    gia = BANG_GIA[ten_hang]
    return {"gia": gia, "don_vi": "dong", "mat_hang": ten_hang}


def trich_gia_tu_do(text):
    cac_so = re.findall(r"\d+", text)
    return int(cac_so[0]) if cac_so else None


def dem_dung(danh_sach_hang):
    so_dung_tu_do = 0
    so_dung_co_cau_truc = 0
    for ten_hang in danh_sach_hang:
        gia_dung = BANG_GIA[ten_hang]

        van_xuoi = llm_mo_phong_van_xuoi(ten_hang)
        gia_tu_do = trich_gia_tu_do(van_xuoi)
        if gia_tu_do ___ gia_dung:                  # ==
            so_dung_tu_do += 1

        dap_an = llm_mo_phong_co_cau_truc(ten_hang)
        gia_co_cau_truc = dap_an[___]                # "gia"
        if gia_co_cau_truc == gia_dung:
            so_dung_co_cau_truc += 1

    return so_dung_tu_do, so_dung_co_cau_truc


so_dung_tu_do, so_dung_co_cau_truc = dem_dung(DANH_SACH_HANG)
print(so_dung_tu_do, "/", len(DANH_SACH_HANG))
print(so_dung_co_cau_truc, "/", len(DANH_SACH_HANG))
```

```python title=solution
import re

BANG_GIA = {
    "banh_mi": 15000,
    "ca_phe": 25000,
    "tra_sua": 42000,
}

DANH_SACH_HANG = ["banh_mi", "ca_phe", "tra_sua"]


def llm_mo_phong_van_xuoi(ten_hang):
    gia = BANG_GIA[ten_hang]
    if ten_hang == "banh_mi":
        return f"Gia {ten_hang} la {gia} dong, co the re hon mot chut."
    if ten_hang == "ca_phe":
        return f"Mon nay dat hang thu 2, gia khoang {gia} dong."
    gia_nghin = gia // 1000
    return f"Gia la khoang {gia_nghin} nghin dong, co the re hon."


def llm_mo_phong_co_cau_truc(ten_hang):
    gia = BANG_GIA[ten_hang]
    return {"gia": gia, "don_vi": "dong", "mat_hang": ten_hang}


def trich_gia_tu_do(text):
    cac_so = re.findall(r"\d+", text)
    return int(cac_so[0]) if cac_so else None


def dem_dung(danh_sach_hang):
    so_dung_tu_do = 0
    so_dung_co_cau_truc = 0
    for ten_hang in danh_sach_hang:
        gia_dung = BANG_GIA[ten_hang]

        van_xuoi = llm_mo_phong_van_xuoi(ten_hang)
        gia_tu_do = trich_gia_tu_do(van_xuoi)
        if gia_tu_do == gia_dung:
            so_dung_tu_do += 1

        dap_an = llm_mo_phong_co_cau_truc(ten_hang)
        gia_co_cau_truc = dap_an["gia"]
        if gia_co_cau_truc == gia_dung:
            so_dung_co_cau_truc += 1

    return so_dung_tu_do, so_dung_co_cau_truc


so_dung_tu_do, so_dung_co_cau_truc = dem_dung(DANH_SACH_HANG)
print(so_dung_tu_do, "/", len(DANH_SACH_HANG))
print(so_dung_co_cau_truc, "/", len(DANH_SACH_HANG))
```

```python title=test
assert dem_dung(DANH_SACH_HANG) == (1, 3), f"ket qua tren ca bo phai la (1, 3) -- dang ra {dem_dung(DANH_SACH_HANG)}"

# bien: danh sach rong -- khong duoc loi, phai tra ve (0, 0)
assert dem_dung([]) == (0, 0), f"bo rong phai tra ve (0, 0) -- dang ra {dem_dung([])}"

# bien: tung mat hang rieng le, xac nhan dung TUNG truong hop khong phai
# trung hop tren ca bo
assert dem_dung(["banh_mi"]) == (1, 1), f"chi banh_mi phai la (1, 1) -- dang ra {dem_dung(['banh_mi'])}"
assert dem_dung(["ca_phe"]) == (0, 1), f"chi ca_phe phai la (0, 1) -- dang ra {dem_dung(['ca_phe'])}"
assert dem_dung(["tra_sua"]) == (0, 1), f"chi tra_sua phai la (0, 1) -- dang ra {dem_dung(['tra_sua'])}"

# bien: dap_an co cau truc luon la mot dict co truong "gia" doc duoc truc
# tiep, bat ke thu tu cac truong khac trong dict
assert llm_mo_phong_co_cau_truc("tra_sua")["gia"] == 42000, "dap_an co cau truc phai doc duoc truong 'gia' truc tiep"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu so sánh `gia_tu_do` (kết quả trích bằng regex) với `gia_dung` (giá thật từ `BANG_GIA`) — dùng toán tử so sánh BẰNG NHAU. Chỗ hai đọc trường giá từ `dap_an` (một `dict`) — dùng đúng TÊN trường đã thấy ở `llm_mo_phong_co_cau_truc` (`"gia"`, `"don_vi"`, `"mat_hang"`).
- kind: strategy
  body: 'Chỗ đầu: `==` (cho dòng `if gia_tu_do == gia_dung:`). Chỗ hai: `"gia"` (cho dòng `gia_co_cau_truc = dap_an["gia"]` — đúng trường chứa số tiền, không phải `"don_vi"` hay `"mat_hang"`).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `==` và `"gia"`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: so sanh gia_tu_do voi gia_dung phai dung toan tu "==" (khong duoc doi thanh mot toan tu khac hay bo so sanh), VA phai doc DUNG truong "gia" tu dap_an (khong duoc doi thanh "don_vi" hay mot chuoi khac)
  requireAst:
  - kind: uses-operator, target: "==", min: 4
  - kind: has-literal, target: "gia", min: 2
  # Da thu that (goi _dem tren code trich tu solution, khong doan tay).
  # "=="=4: hai lan CO SAN (ten_hang == "banh_mi", ten_hang == "ca_phe"
  # trong llm_mo_phong_van_xuoi), mot lan CO SAN (gia_co_cau_truc ==
  # gia_dung), mot lan la cho trong 1 (gia_tu_do == gia_dung). Neu chi dat
  # min=1 (ngay tho), mot mutant xoa so sanh o cho trong 1 (vi du doi thanh
  # "if True:") VAN qua duoc vi con lai ba lan "==" khac trong boilerplate
  # -- day la GOTCHA "boilerplate-threshold-masking"; dat dung min=4 (tong
  # THAT) moi chan duoc mutant nay -- va no cung bi chan boi tests (ca bo
  # se dem sai vi so_dung_tu_do luon tang bat ke gia co dung khong).
  # has-literal "gia"=2: mot lan CO SAN (khoa "gia" trong dict tra ve cua
  # llm_mo_phong_co_cau_truc), mot lan la cho trong 2. Dien bua cho trong 2
  # thanh "don_vi" hay "mat_hang" lam has-literal "gia" tut ve 1 -- duoi
  # nguong min=2, bi chan; dong thoi bi chan boi tests vi gia_co_cau_truc se
  # la mot chuoi ("dong" hoac ten mat hang) thay vi so tien, lam so sanh
  # voi gia_dung luon False.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^1 / 3\\n3 / 3\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Văn xuôi tự do: `1/3` — trích đúng chỉ khi định dạng câu "may mắn" thẳng
hàng với luật trích xuất. Đầu ra có cấu trúc: `3/3` — luôn đúng, vì
`dap_an["gia"]` đọc một TRƯỜNG cố định, không đoán câu chữ. Bài sau xử lý
một vấn đề khác của đầu ra có cấu trúc: LLM đôi khi trả về JSON THIẾU
trường — cần một cách kiểm tra và YÊU CẦU LẠI khi phát hiện thiếu.
::::

::::reflect{#nghi-lai}
Ba mặt hàng, ba câu văn xuôi khác nhau — không phải vì LLM mô phỏng "tuỳ
hứng", mà vì không có RÀNG BUỘC nào ép số tiền phải luôn nằm ở CÙNG MỘT VỊ
TRÍ trong câu. Một pipeline PARSE văn xuôi bằng regex phải đoán đúng vị trí
đó cho MỌI cách diễn đạt có thể xảy ra — mà chỉ cần MỘT cách diễn đạt lệch
khỏi giả định của regex là trích sai, dù con số đúng vẫn có mặt trong câu
chữ. Đầu ra có cấu trúc loại bỏ hoàn toàn việc đoán đó: một `dict` với
trường cố định `"gia"` không có "vị trí trong câu" để đoán — nó là một
TRƯỜNG, đọc trực tiếp. Bài sau đi sâu hơn: cấu trúc không tự nhiên mà có —
LLM (mô phỏng hay thật) đôi khi trả về JSON THIẾU một trường bắt buộc, và
cần một cơ chế kiểm tra ĐÚNG SCHEMA rồi YÊU CẦU LẠI khi phát hiện lỗi đó.
::::

::::checkpoint{mastery=0.8}
::::
