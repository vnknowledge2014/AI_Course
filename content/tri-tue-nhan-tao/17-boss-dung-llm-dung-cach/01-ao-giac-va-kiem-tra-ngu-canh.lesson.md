---
id: tri-tue-nhan-tao.boss-dung-llm-dung-cach.ao-giac-va-kiem-tra-ngu-canh
title: "Ảo giác và kiểm tra ngữ cảnh: câu trả lời có nằm trong phạm vi được cấp không"
summary: "Mot LLM mo phong tra loi gia mat hang: neu mat hang CO trong BANG_GIA, tra loi dung; neu KHONG co (vi du sinh_to, nuoc_ep, banh_ngot), no AO GIAC -- bia mot con so co dinh (20000) thay vi noi khong biet. Ham kiem_tra_ao_giac(cau_tra_loi, ngu_canh_cho_phep) trich so tien trong cau tra loi, kiem so do co nam trong tap gia tri duoc phep (ngu_canh_cho_phep = {15000, 25000, 30000}) hay khong. Chay 6 cau hoi (3 co trong bang, 3 khong co): 3/3 cau dung duoc xac nhan an toan, 3/3 cau ao giac duoc phat hien dung -- ca hai con so deu chay that."
locale: vi
track: tri-tue-nhan-tao
module: boss-dung-llm-dung-cach
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.ao-giac-va-kiem-tra-ngu-canh]
requires: [ai.boss-hoi-thoai-tu-cat-tom-tat]
concepts: [ai.ao-giac-va-kiem-tra-ngu-canh]
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
Track T8.4 sắp khép lại — quest cuối: dùng LLM ĐÚNG CÁCH. Bài đầu tiên hỏi
câu quan trọng nhất: khi một LLM (mô phỏng hay thật) không biết câu trả
lời, nó có NÓI "không biết" không — hay nó BỊA?
::::

::::explain{#ao_giac_la_gi}
**Ảo giác** (hallucination) là khi một LLM trả lời một điều KHÔNG có căn cứ
trong dữ liệu nó được cấp — nó không nói "tôi không biết", mà bịa ra một
câu trả lời nghe có vẻ hợp lý. Với một trợ lý tra giá mặt hàng, ảo giác cụ
thể nhất là: hỏi giá một món KHÔNG có trong bảng giá, và thay vì từ chối,
LLM mô phỏng trả lời bằng một CON SỐ — một giá bịa ra.

Cách phát hiện đơn giản nhất không cần "hiểu" câu trả lời — chỉ cần đối
chiếu: số tiền được nhắc trong câu trả lời có nằm trong **ngữ cảnh cho
phép** hay không, tức tập hợp mọi giá trị THẬT SỰ có trong dữ liệu nguồn
đã được cấp. Nếu số đó không khớp BẤT KỲ giá trị nào trong tập đó, câu trả
lời chắc chắn không tới từ dữ liệu nguồn — nó bị bịa ra.

Đây là một phép kiểm NGẶT nhưng ĐƠN GIẢN: `kiem_tra_ao_giac(cau_tra_loi,
ngu_canh_cho_phep)` trích con số được nhắc trong `cau_tra_loi`, rồi kiểm nó
có thuộc tập `ngu_canh_cho_phep` không. Trả về `True` nếu số đó CÓ trong
phạm vi (an toàn, không ảo giác), `False` nếu KHÔNG (ảo giác, hoặc không
tìm thấy số nào để xác minh).
::::

::::example{#do_ao_giac_that}
Sáu mặt hàng: ba mặt hàng có thật trong bảng giá, ba mặt hàng không tồn
tại — LLM mô phỏng "bịa" đúng `20000` đồng cho mọi mặt hàng không có thật:

```python title=readonly
import re

BANG_GIA = {
    "banh_mi": 15000,
    "ca_phe": 25000,
    "tra_sua": 30000,
}

NGU_CANH_CHO_PHEP = set(BANG_GIA.values())

DANH_SACH_TEN_HANG = ["banh_mi", "ca_phe", "tra_sua", "sinh_to", "nuoc_ep", "banh_ngot"]


def llm_mo_phong_tra_loi(ten_hang):
    if ten_hang in BANG_GIA:
        gia = BANG_GIA[ten_hang]
        return f"gia {ten_hang} la {gia} dong"
    # KHONG co trong BANG_GIA -- AO GIAC: bia mot con so CO DINH (khong
    # ngau nhien) thay vi noi "khong biet"
    return f"gia {ten_hang} la 20000 dong"


def trich_so(cau_tra_loi):
    cac_so = re.findall(r"\d+", cau_tra_loi)
    return int(cac_so[0]) if cac_so else None


def kiem_tra_ao_giac(cau_tra_loi, ngu_canh_cho_phep):
    so = trich_so(cau_tra_loi)
    if so is None:
        return False
    return so in ngu_canh_cho_phep


def dem_phat_hien_ao_giac(danh_sach_ten_hang, ngu_canh_cho_phep):
    so_an_toan = 0
    so_phat_hien_dung = 0
    for ten_hang in danh_sach_ten_hang:
        cau_tra_loi = llm_mo_phong_tra_loi(ten_hang)
        an_toan = kiem_tra_ao_giac(cau_tra_loi, ngu_canh_cho_phep)
        la_ao_giac_that = ten_hang not in BANG_GIA
        if an_toan and not la_ao_giac_that:
            so_an_toan += 1
        if la_ao_giac_that and not an_toan:
            so_phat_hien_dung += 1
    return so_an_toan, so_phat_hien_dung


so_an_toan, so_phat_hien_dung = dem_phat_hien_ao_giac(DANH_SACH_TEN_HANG, NGU_CANH_CHO_PHEP)
print(so_an_toan, "/", 3)
print(so_phat_hien_dung, "/", 3)
```

```text title=readonly
3 / 3
3 / 3
```

Ba mặt hàng có thật (`banh_mi`, `ca_phe`, `tra_sua`): câu trả lời đúng, số
tiền nằm trong `NGU_CANH_CHO_PHEP` — `kiem_tra_ao_giac` xác nhận AN TOÀN cả
`3/3`. Ba mặt hàng không tồn tại (`sinh_to`, `nuoc_ep`, `banh_ngot`): LLM mô
phỏng bịa `20000` đồng cho cả ba — con số này KHÔNG nằm trong
`{15000, 25000, 30000}`, nên `kiem_tra_ao_giac` phát hiện đúng cả `3/3` là
ảo giác.
::::

::::predict{#doan_ao_giac_trung_gia_that commitOnce}
`kiem_tra_ao_giac` chỉ kiểm con số có nằm trong `ngu_canh_cho_phep` hay
không — nó KHÔNG kiểm mặt hàng được nhắc có thật hay không.

**Trước khi chạy thử**, bạn đoán: xét một câu trả lời BỊA ra cho một mặt
hàng không tồn tại, nhưng con số bịa TÌNH CỜ trùng đúng giá của MỘT mặt
hàng có thật khác (ví dụ `"gia mon_la la 25000 dong"` — `mon_la` không tồn
tại, nhưng `25000` là giá thật của `ca_phe`) — `kiem_tra_ao_giac(...)` trả
về gì?

:::opt{correct}
`True` (bị coi là AN TOÀN, dù đây thực chất VẪN là ảo giác) — `kiem_tra_ao_giac`
chỉ kiểm SỐ TIỀN có thuộc tập `ngu_canh_cho_phep` không, nó không đối
chiếu số đó có khớp ĐÚNG MẶT HÀNG đang được hỏi hay không; `25000` có mặt
trong `NGU_CANH_CHO_PHEP` nên phép kiểm này cho qua, dù `mon_la` chưa từng
tồn tại trong `BANG_GIA`
:::

:::opt
`False` — vì `mon_la` không tồn tại trong `BANG_GIA`, nên bất kỳ câu trả
lời nào về nó cũng phải bị đánh dấu ảo giác, bất kể con số là gì
::why
Gần đúng ở việc bạn nhớ đúng: câu trả lời này VỀ BẢN CHẤT vẫn là một ảo
giác — `mon_la` thật sự không tồn tại.

Chỗ lệch: `kiem_tra_ao_giac` không hề biết `mon_la` là mặt hàng gì — nó
CHỈ đọc con số trong câu trả lời rồi tra tập `ngu_canh_cho_phep`. Đây là
một giới hạn CÓ THẬT của phép kiểm này: nó bắt được ảo giác khi con số bịa
ra NẰM NGOÀI mọi giá trị đã biết, nhưng bỏ lọt khi con số bịa ra TÌNH CỜ
trùng một giá trị THẬT của một mặt hàng KHÁC.
::
:::

:::opt
Sẽ báo lỗi, vì `trich_so` không thể phân biệt số tiền với tên mặt hàng
trong cùng một câu
::why
Gần đúng ở việc câu này THẬT SỰ có cả chữ (`mon_la`) lẫn số (`25000`) trộn
lẫn — quan sát về hình dạng câu đó đúng.

Chỗ lệch: `trich_so` dùng `re.findall(r"\d+", ...)` — nó chỉ tìm CHỮ SỐ,
hoàn toàn bỏ qua phần chữ. Câu này có đúng một dãy chữ số (`25000`), nên
`trich_so` trích được nó bình thường, không có gì để báo lỗi.
::
:::
::::

::::code{#viet_kiem_tra_ao_giac}
Hoàn thiện `kiem_tra_ao_giac` (kiểm số trích được có thuộc `ngu_canh_cho_phep`
không) và `dem_phat_hien_ao_giac` (xác định một mặt hàng có THẬT SỰ không
tồn tại trong `BANG_GIA` hay không).

```python title=starter
import re

BANG_GIA = {
    "banh_mi": 15000,
    "ca_phe": 25000,
    "tra_sua": 30000,
}

NGU_CANH_CHO_PHEP = set(BANG_GIA.values())

DANH_SACH_TEN_HANG = ["banh_mi", "ca_phe", "tra_sua", "sinh_to", "nuoc_ep", "banh_ngot"]


def llm_mo_phong_tra_loi(ten_hang):
    if ten_hang in BANG_GIA:
        gia = BANG_GIA[ten_hang]
        return f"gia {ten_hang} la {gia} dong"
    return f"gia {ten_hang} la 20000 dong"


def trich_so(cau_tra_loi):
    cac_so = re.findall(r"\d+", cau_tra_loi)
    return int(cac_so[0]) if cac_so else None


def kiem_tra_ao_giac(cau_tra_loi, ngu_canh_cho_phep):
    so = trich_so(cau_tra_loi)
    if so is None:
        return False
    return so ___ ngu_canh_cho_phep                # in


def dem_phat_hien_ao_giac(danh_sach_ten_hang, ngu_canh_cho_phep):
    so_an_toan = 0
    so_phat_hien_dung = 0
    for ten_hang in danh_sach_ten_hang:
        cau_tra_loi = llm_mo_phong_tra_loi(ten_hang)
        an_toan = kiem_tra_ao_giac(cau_tra_loi, ngu_canh_cho_phep)
        la_ao_giac_that = ten_hang ___ BANG_GIA      # not in
        if an_toan and not la_ao_giac_that:
            so_an_toan += 1
        if la_ao_giac_that and not an_toan:
            so_phat_hien_dung += 1
    return so_an_toan, so_phat_hien_dung


so_an_toan, so_phat_hien_dung = dem_phat_hien_ao_giac(DANH_SACH_TEN_HANG, NGU_CANH_CHO_PHEP)
print(so_an_toan, "/", 3)
print(so_phat_hien_dung, "/", 3)
```

```python title=solution
import re

BANG_GIA = {
    "banh_mi": 15000,
    "ca_phe": 25000,
    "tra_sua": 30000,
}

NGU_CANH_CHO_PHEP = set(BANG_GIA.values())

DANH_SACH_TEN_HANG = ["banh_mi", "ca_phe", "tra_sua", "sinh_to", "nuoc_ep", "banh_ngot"]


def llm_mo_phong_tra_loi(ten_hang):
    if ten_hang in BANG_GIA:
        gia = BANG_GIA[ten_hang]
        return f"gia {ten_hang} la {gia} dong"
    return f"gia {ten_hang} la 20000 dong"


def trich_so(cau_tra_loi):
    cac_so = re.findall(r"\d+", cau_tra_loi)
    return int(cac_so[0]) if cac_so else None


def kiem_tra_ao_giac(cau_tra_loi, ngu_canh_cho_phep):
    so = trich_so(cau_tra_loi)
    if so is None:
        return False
    return so in ngu_canh_cho_phep


def dem_phat_hien_ao_giac(danh_sach_ten_hang, ngu_canh_cho_phep):
    so_an_toan = 0
    so_phat_hien_dung = 0
    for ten_hang in danh_sach_ten_hang:
        cau_tra_loi = llm_mo_phong_tra_loi(ten_hang)
        an_toan = kiem_tra_ao_giac(cau_tra_loi, ngu_canh_cho_phep)
        la_ao_giac_that = ten_hang not in BANG_GIA
        if an_toan and not la_ao_giac_that:
            so_an_toan += 1
        if la_ao_giac_that and not an_toan:
            so_phat_hien_dung += 1
    return so_an_toan, so_phat_hien_dung


so_an_toan, so_phat_hien_dung = dem_phat_hien_ao_giac(DANH_SACH_TEN_HANG, NGU_CANH_CHO_PHEP)
print(so_an_toan, "/", 3)
print(so_phat_hien_dung, "/", 3)
```

```python title=test
assert (so_an_toan, so_phat_hien_dung) == (3, 3), f"ca hai con so tren bo 6 mat hang phai la (3, 3) -- dang ra {(so_an_toan, so_phat_hien_dung)}"

# xac nhan truc tiep kiem_tra_ao_giac tren tung cau tra loi cu the -- KHONG
# chi dua vao ket qua tong hop cua dem_phat_hien_ao_giac
assert kiem_tra_ao_giac("gia banh_mi la 15000 dong", NGU_CANH_CHO_PHEP) == True, "cau tra loi DUNG (banh_mi) phai duoc xac nhan an toan"
assert kiem_tra_ao_giac("gia sinh_to la 20000 dong", NGU_CANH_CHO_PHEP) == False, "cau tra loi AO GIAC (sinh_to, bia 20000) phai bi phat hien"
assert kiem_tra_ao_giac("", NGU_CANH_CHO_PHEP) == False, "chuoi rong (khong tim thay so nao) phai bi coi la khong an toan"

# bien: danh sach RONG phai tra ve (0, 0), khong loi
assert dem_phat_hien_ao_giac([], NGU_CANH_CHO_PHEP) == (0, 0), f"danh sach rong phai la (0, 0) -- dang ra {dem_phat_hien_ao_giac([], NGU_CANH_CHO_PHEP)}"

# bien QUAN TRONG: mot mat hang CO THAT don le -- xac nhan dung (1, 0),
# phan biet duoc mot cheat "dao nguoc" giua hai cho trong (kiem_tra_ao_giac
# dung "not in" thay vi "in", VA la_ao_giac_that dung "in" thay vi "not in"
# cung luc) -- cheat do KHONG doi tong so dem AST cua "in"/"not in" (chi di
# chuyen occurrence), nhung lam ket qua tren danh sach mot phan tu nay SAI
# hoan toan (xem ghi chu static ben duoi)
assert dem_phat_hien_ao_giac(["banh_mi"], NGU_CANH_CHO_PHEP) == (1, 0), f"chi banh_mi (co that) phai la (1, 0) -- dang ra {dem_phat_hien_ao_giac(['banh_mi'], NGU_CANH_CHO_PHEP)}"
assert dem_phat_hien_ao_giac(["sinh_to"], NGU_CANH_CHO_PHEP) == (0, 1), f"chi sinh_to (ao giac) phai la (0, 1) -- dang ra {dem_phat_hien_ao_giac(['sinh_to'], NGU_CANH_CHO_PHEP)}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu (trong `kiem_tra_ao_giac`) kiểm số `so` có thuộc tập `ngu_canh_cho_phep` không — dùng phép kiểm PHẦN TỬ CỦA TẬP HỢP (`in`). Chỗ hai (trong `dem_phat_hien_ao_giac`) xác định mặt hàng KHÔNG tồn tại trong `BANG_GIA` — dùng phép phủ định của phép kiểm đó (`not in`).
- kind: strategy
  body: 'Chỗ đầu: `in` (cho dòng `return so in ngu_canh_cho_phep`). Chỗ hai: `not in` (cho dòng `la_ao_giac_that = ten_hang not in BANG_GIA`).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `in` và `not in`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: kiem_tra_ao_giac phai dung "in" de kiem so co thuoc ngu_canh_cho_phep (khong duoc doi thanh "not in" hay xoa phep kiem), VA la_ao_giac_that phai dung "not in" de kiem mat hang KHONG co trong BANG_GIA (khong duoc doi thanh "in")
  requireAst:
  - kind: uses-operator, target: "in", min: 2
  - kind: uses-operator, target: "not in", min: 1
  # Da thu that (goi kiemAst that -- trich nguyen ham _dem tu kiem-ast.ts,
  # chay qua python3 tren code trich tu solution, khong doan tay).
  # "in"=2: mot lan CO SAN trong llm_mo_phong_tra_loi ("if ten_hang in
  # BANG_GIA:"), mot lan la cho trong 1 (return so in ngu_canh_cho_phep).
  # Neu chi dat min=1 (ngay tho), mot mutant xoa cho trong 1 (vi du doi
  # thanh "return True") VAN qua duoc vi con lai 1 lan "in" trong
  # boilerplate -- GOTCHA "boilerplate-threshold-masking"; dat dung min=2
  # (tong THAT) moi chan duoc mutant nay -- va no cung bi chan boi tests
  # (kiem_tra_ao_giac tren cau ao giac se sai thanh True).
  # "not in"=1: XUAT HIEN DUY NHAT o cho trong 2 -- khong co "not in" nao
  # khac trong toan bo solution. Dien bua thanh "in" (dao nguoc logic) lam
  # so nay tut ve 0 -- duoi nguong min=1, bi chan; dong thoi bi chan boi
  # tests (dem_phat_hien_ao_giac tren bo 6 mat hang se sai).
  #
  # 🔴 GOTCHA da tu xac minh doc lap (khong suy luan suong): mot mutant HOAN
  # DOI CA HAI cho trong cung luc (cho trong 1 dung "not in" thay vi "in",
  # VA cho trong 2 dung "in" thay vi "not in") KHONG doi tong so dem AST
  # (van la 2 va 1 -- hoan doi chi DI CHUYEN mot occurrence giua hai cho,
  # tong khong doi) -- da chay that qua kiemAst() de xac nhan dieu nay,
  # KHONG suy doan. Tren bo 6-mat-hang CHINH, mutant nay TINH CO cho ra
  # DUNG cung ket qua tong hop (3, 3) (vi vai tro cua hai gia tri boolean
  # bi doi cheo cho nhau theo cach trung khop o muc TONG), nen KHONG bi bat
  # boi assertion tren ca bo. Day la ly do hai ca test RIENG voi danh sach
  # MOT PHAN TU (["banh_mi"] va ["sinh_to"]) ton tai doc lap trong khoi test
  # o tren -- da tu kiem chung: voi mutant hoan doi ca hai, ca hai ket qua
  # nay LECH so voi (1, 0) va (0, 1) mong doi, bi bat DOC LAP boi hai
  # assertion nay, khong phu thuoc static.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^3 / 3\\n3 / 3\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`3/3` câu trả lời đúng được xác nhận an toàn, `3/3` câu ảo giác bị phát
hiện đúng — nhưng phép kiểm này có một điểm mù: nó chỉ nhìn CON SỐ, không
nhìn MẶT HÀNG. Bài sau xây một cách đo khác: so hai câu trả lời với nhau,
không chỉ so một con số với một tập hợp.
::::

::::reflect{#nghi-lai}
`kiem_tra_ao_giac` không "hiểu" gì cả — nó chỉ đối chiếu một con số với một
tập giá trị đã biết trước. Điều đó đủ để bắt đúng dạng ảo giác phổ biến
nhất (bịa một con số hoàn toàn không có căn cứ), nhưng có MỘT điểm mù đã
thấy ở predict: nếu con số bịa ra tình cờ trùng giá trị THẬT của một mặt
hàng khác, phép kiểm này bị qua mặt — nó không đối chiếu được MẶT HÀNG,
chỉ đối chiếu được CON SỐ. Đây là một bài học quan trọng cho toàn track:
một phép kiểm đơn giản, tất định, vẫn có GIÁ TRỊ THẬT (bắt được `3/3` ca ảo
giác rõ ràng), dù nó không hoàn hảo. Bài sau chuyển hướng: thay vì so một
con số với một tập hợp, so SÁNH HAI CÂU TRẢ LỜI với nhau — một câu của mô
hình, một câu đáp án chuẩn — bằng hai độ đo tự cài, không dùng thư viện
ngoài.
::::

::::checkpoint{mastery=0.8}
::::
