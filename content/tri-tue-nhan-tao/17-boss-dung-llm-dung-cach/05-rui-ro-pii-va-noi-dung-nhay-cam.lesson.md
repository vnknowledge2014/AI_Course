---
id: tri-tue-nhan-tao.boss-dung-llm-dung-cach.rui-ro-pii-va-noi-dung-nhay-cam
title: "Rủi ro PII và nội dung nhạy cảm: che trước khi gửi đi"
summary: "Ham phat_hien_pii(van_ban) quet MAU co dinh cho so dien thoai (dung re, dang 10 chu so lien bat dau bang 0) VA mot danh sach tu khoa nhay cam co dinh (cmnd, cccd, so nha, dia chi). Truoc khi 'gui' cau hoi cho cong cu/LLM, xu_ly_truoc_khi_gui che PII bang placeholder co dinh ('[SO_DIEN_THOAI]', '[THONG_TIN_NHAY_CAM]'). Tren 6 cau hoi (4 co PII duoi nhieu dang khac nhau, 2 khong co): 4/4 ca duoc che dung, 2/2 cau KHONG co PII di qua nguyen ven khong doi mot ky tu. Kiem bien: so 9 chu so va 11 chu so lien deu KHONG bi coi la so dien thoai (dung ranh gioi tu \\b). Tat ca con so deu chay that."
locale: vi
track: tri-tue-nhan-tao
module: boss-dung-llm-dung-cach
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.rui-ro-pii-va-noi-dung-nhay-cam]
requires: [ai.phat-hien-prompt-injection]
concepts: [ai.rui-ro-pii-va-noi-dung-nhay-cam]
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
Bốn bài trước lo về câu TRẢ LỜI — ảo giác, đánh giá, giám khảo, dữ liệu
công cụ bị nhiễm. Bài này lo về đầu VÀO: câu hỏi của người dùng có thể
chứa thông tin cá nhân, và nó cần được CHE trước khi đi xa hơn.
::::

::::explain{#pii_la_gi}
**PII** (Personally Identifiable Information — thông tin định danh cá
nhân) là bất kỳ dữ liệu nào có thể dùng để nhận diện một người cụ thể: số
điện thoại, số CMND/CCCD, địa chỉ nhà. Trước khi một câu hỏi của người
dùng được gửi tiếp cho một "công cụ" hay một "LLM" (mô phỏng hay thật),
PII trong câu hỏi đó cần được QUÉT và CHE lại — không phải vì bản thân câu
hỏi sai, mà vì hệ thống phía sau (log, một LLM thứ ba, một dịch vụ khác)
không cần và không nên thấy dữ liệu đó.

Bài này tự cài `phat_hien_pii(van_ban)` bằng hai cơ chế quét ĐƠN GIẢN,
CỐ ĐỊNH — không cần một mô hình "hiểu" văn bản:

> **Mẫu số điện thoại** — dùng `re` (module chuẩn của Python) tìm một dãy
> ĐÚNG `10` chữ số liền, bắt đầu bằng `0` (`r"0\d{9}\b"` — dấu `\b` đảm bảo
> dãy số này KẾT THÚC ở đó, không phải một phần của một dãy số dài hơn).
>
> **Từ khoá nhạy cảm cố định** — một danh sách biết trước
> (`"cmnd"`, `"cccd"`, `"so nha"`, `"dia chi"`), quét không phân biệt hoa
> thường.

Nếu phát hiện PII, `che_pii(van_ban)` thay thế: số điện thoại bằng
placeholder cố định `"[SO_DIEN_THOAI]"`, và bất kỳ từ khoá nhạy cảm nào
bằng `"[THONG_TIN_NHAY_CAM]"`. Câu hỏi KHÔNG chứa PII phải đi qua NGUYÊN
VẸN, không đổi một ký tự nào.
::::

::::example{#do_che_pii_that}
Sáu câu hỏi: bốn câu chứa PII dưới nhiều dạng khác nhau (số điện thoại,
từ khoá nhạy cảm), hai câu hoàn toàn sạch:

```python title=readonly
import re

MAU_SO_DIEN_THOAI = re.compile(r"0\d{9}\b")

TU_KHOA_NHAY_CAM = ["cmnd", "cccd", "so nha", "dia chi"]


def phat_hien_pii(van_ban):
    van_ban_thuong = van_ban.lower()
    co_so_dien_thoai = bool(MAU_SO_DIEN_THOAI.search(van_ban))
    co_tu_khoa_nhay_cam = any(tu in van_ban_thuong for tu in TU_KHOA_NHAY_CAM)
    return co_so_dien_thoai or co_tu_khoa_nhay_cam


def che_pii(van_ban):
    van_ban_da_che = MAU_SO_DIEN_THOAI.sub("[SO_DIEN_THOAI]", van_ban)
    for tu in TU_KHOA_NHAY_CAM:
        van_ban_da_che = re.sub(re.escape(tu), "[THONG_TIN_NHAY_CAM]", van_ban_da_che, flags=re.IGNORECASE)
    return van_ban_da_che


def xu_ly_truoc_khi_gui(cau_hoi):
    if phat_hien_pii(cau_hoi):
        return che_pii(cau_hoi)
    return cau_hoi


DANH_SACH_CAU_HOI = [
    ("gia banh mi bao nhieu tien", False),
    ("goi cho toi qua so 0912345678 khi co gia", True),
    ("so cmnd cua toi la 123456789012, gia ca phe bao nhieu", True),
    ("gia tra sua bao nhieu, dia chi giao hang la 12 nguyen trai", True),
    ("lien he 0987654321 de biet gia sinh to", True),
    ("cam on ban rat nhieu", False),
]


def dem_xu_ly_dung(danh_sach_cau_hoi):
    so_che_dung = 0
    so_giu_nguyen_dung = 0
    for cau_hoi, co_pii in danh_sach_cau_hoi:
        ket_qua = xu_ly_truoc_khi_gui(cau_hoi)
        if co_pii:
            if ket_qua != cau_hoi:
                so_che_dung += 1
        else:
            if ket_qua == cau_hoi:
                so_giu_nguyen_dung += 1
    return so_che_dung, so_giu_nguyen_dung


so_che_dung, so_giu_nguyen_dung = dem_xu_ly_dung(DANH_SACH_CAU_HOI)
print(so_che_dung, "/", 4)
print(so_giu_nguyen_dung, "/", 2)
```

```text title=readonly
4 / 4
2 / 2
```

Bốn câu có PII (số điện thoại dưới hai dạng viết khác nhau, từ khoá
`"cmnd"`, từ khoá `"dia chi"`) đều được CHE đúng, `4/4`. Hai câu sạch
(không PII) đi qua NGUYÊN VẸN, `2/2` — không một ký tự nào bị đổi, dù
`xu_ly_truoc_khi_gui` đã CHẠY qua chúng.
::::

::::predict{#doan_cau_khong_co_pii commitOnce}
Xét câu `"cam on ban rat nhieu"` — không chứa số điện thoại, không chứa
bất kỳ từ khoá nhạy cảm nào trong `TU_KHOA_NHAY_CAM`.

**Trước khi chạy thử**, bạn đoán: `xu_ly_truoc_khi_gui("cam on ban rat
nhieu")` trả về gì?

:::opt{correct}
`"cam on ban rat nhieu"` — Y HỆT chuỗi gốc, không đổi một ký tự nào —
`phat_hien_pii` trả về `False` (không khớp mẫu số điện thoại, không khớp
từ khoá nào), nên `xu_ly_truoc_khi_gui` đi vào nhánh `return cau_hoi`,
trả về chính đối số nhận vào, không gọi `che_pii` chút nào
:::

:::opt
Một chuỗi đã được CHE MỘT PHẦN, ví dụ thêm placeholder `"[AN_TOAN]"` ở đầu
câu — để đánh dấu rằng hệ thống ĐÃ QUÉT qua câu này, dù không tìm thấy PII
::why
Gần đúng ở trực giác "nên có dấu hiệu đã kiểm tra" — một thiết kế ghi log
hợp lý trong một số hệ thống khác.

Chỗ lệch: `xu_ly_truoc_khi_gui` không có bước "đánh dấu đã quét" nào cả —
nó chỉ có ĐÚNG hai nhánh: gọi `che_pii` (khi `phat_hien_pii` là `True`)
hoặc trả về nguyên câu hỏi không đổi gì (khi `False`). Không có nhánh thứ
ba nào chèn thêm placeholder cho câu sạch.
::
:::

:::opt
Chuỗi rỗng `""` — vì hàm coi một câu không có PII là "không có gì đáng
gửi đi", nên trả về rỗng cho an toàn
::why
Gần đúng ở tinh thần "thà an toàn" — một triết lý phòng vệ hợp lý trong
một số ngữ cảnh khác.

Chỗ lệch: `xu_ly_truoc_khi_gui` không có logic nào làm RỖNG một câu hỏi
sạch — nhánh `return cau_hoi` trả về ĐÚNG đối số nhận vào, nguyên vẹn. Làm
rỗng câu hỏi hợp lệ sẽ phá vỡ hoàn toàn khả năng trả lời câu hỏi đó, một
hành vi không có trong thiết kế của hàm này.
::
:::
::::

::::code{#viet_phat_hien_va_che_pii}
Hoàn thiện `phat_hien_pii` (kết hợp CẢ HAI tín hiệu — số điện thoại HOẶC
từ khoá nhạy cảm) và `xu_ly_truoc_khi_gui` (gọi đúng hàm che khi phát hiện
PII).

```python title=starter
import re

MAU_SO_DIEN_THOAI = re.compile(r"0\d{9}\b")

TU_KHOA_NHAY_CAM = ["cmnd", "cccd", "so nha", "dia chi"]


def phat_hien_pii(van_ban):
    van_ban_thuong = van_ban.lower()
    co_so_dien_thoai = bool(MAU_SO_DIEN_THOAI.search(van_ban))
    co_tu_khoa_nhay_cam = any(tu in van_ban_thuong for tu in TU_KHOA_NHAY_CAM)
    return co_so_dien_thoai ___ co_tu_khoa_nhay_cam     # or


def che_pii(van_ban):
    van_ban_da_che = MAU_SO_DIEN_THOAI.sub("[SO_DIEN_THOAI]", van_ban)
    for tu in TU_KHOA_NHAY_CAM:
        van_ban_da_che = re.sub(re.escape(tu), "[THONG_TIN_NHAY_CAM]", van_ban_da_che, flags=re.IGNORECASE)
    return van_ban_da_che


def xu_ly_truoc_khi_gui(cau_hoi):
    if phat_hien_pii(cau_hoi):
        return ___(cau_hoi)                              # che_pii
    return cau_hoi


DANH_SACH_CAU_HOI = [
    ("gia banh mi bao nhieu tien", False),
    ("goi cho toi qua so 0912345678 khi co gia", True),
    ("so cmnd cua toi la 123456789012, gia ca phe bao nhieu", True),
    ("gia tra sua bao nhieu, dia chi giao hang la 12 nguyen trai", True),
    ("lien he 0987654321 de biet gia sinh to", True),
    ("cam on ban rat nhieu", False),
]


def dem_xu_ly_dung(danh_sach_cau_hoi):
    so_che_dung = 0
    so_giu_nguyen_dung = 0
    for cau_hoi, co_pii in danh_sach_cau_hoi:
        ket_qua = xu_ly_truoc_khi_gui(cau_hoi)
        if co_pii:
            if ket_qua != cau_hoi:
                so_che_dung += 1
        else:
            if ket_qua == cau_hoi:
                so_giu_nguyen_dung += 1
    return so_che_dung, so_giu_nguyen_dung


so_che_dung, so_giu_nguyen_dung = dem_xu_ly_dung(DANH_SACH_CAU_HOI)
print(so_che_dung, "/", 4)
print(so_giu_nguyen_dung, "/", 2)
```

```python title=solution
import re

MAU_SO_DIEN_THOAI = re.compile(r"0\d{9}\b")

TU_KHOA_NHAY_CAM = ["cmnd", "cccd", "so nha", "dia chi"]


def phat_hien_pii(van_ban):
    van_ban_thuong = van_ban.lower()
    co_so_dien_thoai = bool(MAU_SO_DIEN_THOAI.search(van_ban))
    co_tu_khoa_nhay_cam = any(tu in van_ban_thuong for tu in TU_KHOA_NHAY_CAM)
    return co_so_dien_thoai or co_tu_khoa_nhay_cam


def che_pii(van_ban):
    van_ban_da_che = MAU_SO_DIEN_THOAI.sub("[SO_DIEN_THOAI]", van_ban)
    for tu in TU_KHOA_NHAY_CAM:
        van_ban_da_che = re.sub(re.escape(tu), "[THONG_TIN_NHAY_CAM]", van_ban_da_che, flags=re.IGNORECASE)
    return van_ban_da_che


def xu_ly_truoc_khi_gui(cau_hoi):
    if phat_hien_pii(cau_hoi):
        return che_pii(cau_hoi)
    return cau_hoi


DANH_SACH_CAU_HOI = [
    ("gia banh mi bao nhieu tien", False),
    ("goi cho toi qua so 0912345678 khi co gia", True),
    ("so cmnd cua toi la 123456789012, gia ca phe bao nhieu", True),
    ("gia tra sua bao nhieu, dia chi giao hang la 12 nguyen trai", True),
    ("lien he 0987654321 de biet gia sinh to", True),
    ("cam on ban rat nhieu", False),
]


def dem_xu_ly_dung(danh_sach_cau_hoi):
    so_che_dung = 0
    so_giu_nguyen_dung = 0
    for cau_hoi, co_pii in danh_sach_cau_hoi:
        ket_qua = xu_ly_truoc_khi_gui(cau_hoi)
        if co_pii:
            if ket_qua != cau_hoi:
                so_che_dung += 1
        else:
            if ket_qua == cau_hoi:
                so_giu_nguyen_dung += 1
    return so_che_dung, so_giu_nguyen_dung


so_che_dung, so_giu_nguyen_dung = dem_xu_ly_dung(DANH_SACH_CAU_HOI)
print(so_che_dung, "/", 4)
print(so_giu_nguyen_dung, "/", 2)
```

```python title=test
assert (so_che_dung, so_giu_nguyen_dung) == (4, 2), f"ket qua tren 6 cau hoi phai la (4, 2) -- dang ra {(so_che_dung, so_giu_nguyen_dung)}"

assert phat_hien_pii("gia banh mi bao nhieu tien") == False, "cau khong co PII phai la False"
assert phat_hien_pii("goi cho toi qua so 0912345678 khi co gia") == True, "so dien thoai 10 chu so phai duoc phat hien"
assert phat_hien_pii("so cmnd cua toi la 123456789012, gia ca phe bao nhieu") == True, "tu khoa 'cmnd' phai duoc phat hien"

# bang chung TRUNG TAM: cau KHONG co PII phai di qua NGUYEN VEN
assert xu_ly_truoc_khi_gui("cam on ban rat nhieu") == "cam on ban rat nhieu", "cau khong PII phai giu nguyen, khong doi mot ky tu"
assert xu_ly_truoc_khi_gui("goi cho toi qua so 0912345678 khi co gia") == "goi cho toi qua so [SO_DIEN_THOAI] khi co gia", "so dien thoai phai duoc thay bang placeholder co dinh"

# bien BIEN GIOI: so 9 chu so va 11 chu so lien KHONG duoc coi la so dien
# thoai (mau chi khop DUNG 10 chu so, co ranh gioi tu \\b)
assert phat_hien_pii("so dien thoai 091234567 la sai") == False, f"9 chu so khong duoc coi la so dien thoai -- dang ra {phat_hien_pii('so dien thoai 091234567 la sai')}"
assert phat_hien_pii("so dien thoai 09123456789 la sai") == False, f"11 chu so lien nhau khong duoc coi la so dien thoai -- dang ra {phat_hien_pii('so dien thoai 09123456789 la sai')}"

# bien: danh sach RONG phai tra ve (0, 0)
assert dem_xu_ly_dung([]) == (0, 0), f"danh sach rong phai la (0, 0) -- dang ra {dem_xu_ly_dung([])}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu kết hợp hai tín hiệu độc lập (`co_so_dien_thoai`, `co_tu_khoa_nhay_cam`) — PII được coi là có mặt nếu CHỈ CẦN MỘT trong hai tín hiệu đúng, dùng toán tử kết hợp "một trong hai" (`or`). Chỗ hai gọi đúng hàm CHE dữ liệu khi phát hiện PII — dùng tên hàm đã định nghĩa phía trên (`che_pii`).
- kind: strategy
  body: 'Chỗ đầu: `or` (cho dòng `return co_so_dien_thoai or co_tu_khoa_nhay_cam`). Chỗ hai: `che_pii` (cho dòng `return che_pii(cau_hoi)`).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `or` và `che_pii`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phat_hien_pii phai dung "or" de ket hop hai tin hieu (khong duoc doi thanh "and", se bo lot truong hop chi co MOT tin hieu dung), VA xu_ly_truoc_khi_gui phai GOI THAT che_pii (khong duoc tra ve cau_hoi nguyen van hay mot ham khac)
  requireAst:
  - kind: uses-operator, target: "or", min: 1
  - kind: uses-call, target: che_pii, min: 1
  # Da thu that (goi kiemAst that -- trich nguyen ham _dem tu kiem-ast.ts,
  # chay qua python3 tren code trich tu solution, khong doan tay).
  # "or"=1: XUAT HIEN DUY NHAT o cho trong 1 -- khong co "or" nao khac
  # trong toan bo solution. Dien bua doi thanh "and" lam so nay tut ve 0
  # (BoolOp doi tu Or sang And) -- duoi nguong min=1, bi chan; dong thoi bi
  # chan boi tests (voi "and", cac cau CHI co MOT tin hieu -- vi du chi co
  # so dien thoai, khong co tu khoa nhay cam -- se KHONG duoc phat hien,
  # lam so_che_dung tut xuong duoi 4).
  # uses-call che_pii=1: XUAT HIEN DUY NHAT o cho trong 2 (dinh nghia "def
  # che_pii(...)" khong tinh la Call). Dien bua tra ve cau_hoi khong doi
  # (bo qua viec che) lam so nay tut ve 0 -- duoi nguong min=1, bi chan;
  # dong thoi bi chan boi tests (xu_ly_truoc_khi_gui tren cau co PII se tra
  # ve nguyen van, lo PII thay vi che).
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^4 / 4\\n2 / 2\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`4/4` ca có PII được che đúng, `2/2` câu sạch đi qua nguyên vẹn — và cả
trường hợp biên `9`/`11` chữ số cũng không bị báo nhầm. Bài BOSS cuối cùng
ráp CẢ NĂM kỹ thuật của track — che PII, gọi công cụ, phát hiện injection,
kiểm ảo giác, chấm điểm — thành MỘT trợ lý, đóng T8.4 tại `28/28`.
::::

::::reflect{#nghi-lai}
Che PII không cần một mô hình "hiểu" văn bản là gì — hai tín hiệu ĐƠN GIẢN
(một mẫu số điện thoại cố định, một danh sách từ khoá cố định) đã đủ để
bắt đúng bốn dạng PII khác nhau trong bộ kiểm thử này, VÀ quan trọng không
kém — không báo nhầm trên câu sạch, không báo nhầm trên số có độ dài GẦN
GIỐNG số điện thoại nhưng không đúng hệt (`9` hay `11` chữ số). Nguyên tắc
xuyên suốt: che TRƯỚC KHI gửi đi, không phải sau — một khi dữ liệu đã rời
khỏi hệ thống (được ghi log, được gửi cho một dịch vụ khác), không có cách
nào "che lại" được nữa. Đây là mảnh ghép CUỐI CÙNG trong số năm kỹ thuật
của track T8.4: kiểm ảo giác (bài `1`), đánh giá tự động (bài `2`), giám
khảo mô phỏng (bài `3`), phát hiện injection (bài `4`), và che PII (bài
này). Bài BOSS ráp cả năm lại thành MỘT trợ lý duy nhất — và đóng track
`T8.4` tại `28/28`.
::::

::::checkpoint{mastery=0.85}
::::
