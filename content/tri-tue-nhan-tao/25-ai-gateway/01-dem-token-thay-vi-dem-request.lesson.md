---
id: tri-tue-nhan-tao.ai-gateway.dem-token-thay-vi-dem-request
title: "Đếm token, không phải đếm request: vì sao rate limit phải tính theo TOKEN"
summary: "dem_so_request(danh_sach_request) = len(danh_sach_request) -- chi dem SO LUONG, khong quan tam token dung nhieu hay it. dem_token_da_dung(danh_sach_request) cong don truong so_token cua TUNG request trong danh sach. NGUOI_DUNG_NHE (4 request, 18/20/22/20 token) cho dem_so_request=4, dem_token_da_dung=18+20+22+20=80. NGUOI_DUNG_NANG (4 request, 15000/14000/16000/15000 token) cho dem_so_request=4 -- BANG voi NGUOI_DUNG_NHE -- nhung dem_token_da_dung=15000+14000+16000+15000=60000, gap 60000/80=750 lan. Mot request 'tom tat 3 cau' (20 token) va mot request 'phan tich 200 trang' (15000 token) chenh dung 15000/20=750 lan tren MOT request duy nhat -- 'cong bang theo request' la SAI cho ca hai truong hop, 'cong bang theo token' moi phan anh dung chi phi that."
locale: vi
track: tri-tue-nhan-tao
module: ai-gateway
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.dem-token-thay-vi-dem-request]
requires: [ai.boss-hang-doi-suy-luan-gop-lo]
concepts: [ai.dem-token-thay-vi-dem-request]
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
`q8.6a` đóng tại `5/5`: ba tầng kiến trúc, chi phí khởi động, gộp lô, cấp
phát theo khối. `T8.6` giờ chuyển sang tầng đứng NGAY TRƯỚC `Inference` —
`AI Gateway`. Câu hỏi đầu tiên `Chương 39.3` đặt ra: khi giới hạn tốc độ gọi
API (rate limit) cho một người dùng, đếm CÁI GÌ mới công bằng?
::::

::::explain{#dem_gi_moi_cong_bang}
Một cổng AI (`AI Gateway`) đứng giữa người dùng VÀ tầng `Inference` phải
ngăn một người dùng gọi API liên tục làm kiệt quệ GPU — đó LÀ **rate
limiting**. Câu hỏi LÀ: giới hạn "bao nhiêu" — bao nhiêu CÁI GÌ?

Cách nghĩ đầu tiên, có vẻ tự nhiên: đếm **số request**. Mỗi người dùng được
gọi tối đa `N` request mỗi khoảng thời gian, bất kể request đó LÀ gì.

```
dem_so_request(danh_sach_request) = len(danh_sach_request)
```

Vấn đề: một request "tóm tắt `3` câu" VÀ một request "phân tích `200` trang
tài liệu" ĐỀU được tính LÀ **một** request như nhau — dù chi phí tính toán
thật của chúng chênh nhau tới hàng nghìn lần. Một request tóm tắt có thể chỉ
cần `20` token; một request phân tích `200` trang có thể cần tới `15000`
token — `15000 / 20 = 750` lần chênh lệch, TRÊN CÙNG MỘT request.

Cách nghĩ đúng: đếm **tổng số token đã dùng**, cộng dồn qua TỪNG request:

```
dem_token_da_dung(danh_sach_request) = SUM(so_token cua TUNG request trong danh sach)
```

Rate limit theo `dem_so_request` vừa QUÁ CHẶT với người dùng nhẹ (toàn
request nhỏ), vừa QUÁ LỎNG với người dùng nặng (vài request đã ngốn hết GPU)
— đúng nhận định `Chương 39.3, Bài tập 1` đưa ra. Rate limit phải tính theo
`dem_token_da_dung`.
::::

::::example{#hai_nguoi_dung_cung_so_request_khac_han_token}
```python title=readonly
def dem_so_request(danh_sach_request):
    return len(danh_sach_request)


def dem_token_da_dung(danh_sach_request):
    tong = 0
    for yeu_cau in danh_sach_request:
        tong += yeu_cau["so_token"]
    return tong


NGUOI_DUNG_NHE = [
    {"nguoi_dung": "nhe", "so_token": 18},
    {"nguoi_dung": "nhe", "so_token": 20},
    {"nguoi_dung": "nhe", "so_token": 22},
    {"nguoi_dung": "nhe", "so_token": 20},
]
NGUOI_DUNG_NANG = [
    {"nguoi_dung": "nang", "so_token": 15000},
    {"nguoi_dung": "nang", "so_token": 14000},
    {"nguoi_dung": "nang", "so_token": 16000},
    {"nguoi_dung": "nang", "so_token": 15000},
]

so_request_nhe = dem_so_request(NGUOI_DUNG_NHE)
so_request_nang = dem_so_request(NGUOI_DUNG_NANG)
token_nhe = dem_token_da_dung(NGUOI_DUNG_NHE)
token_nang = dem_token_da_dung(NGUOI_DUNG_NANG)

cung_so_request = so_request_nhe == so_request_nang
cong_bang_theo_request_sai = cung_so_request and token_nhe != token_nang

print(so_request_nhe, so_request_nang)
print(token_nhe, token_nang)
print(cung_so_request)
print(cong_bang_theo_request_sai)
```

```text title=readonly
4 4
80 60000
True
True
```

`NGUOI_DUNG_NHE` VÀ `NGUOI_DUNG_NANG` đều gửi ĐÚNG `4` request —
`dem_so_request` trả về `4` cho CẢ HAI, `cung_so_request` LÀ `True`. Nếu
rate limit chỉ đếm request, hai người dùng này bị đối xử Y HỆT nhau. Nhưng
`dem_token_da_dung`: `NGUOI_DUNG_NHE` dùng `18+20+22+20 = 80` token;
`NGUOI_DUNG_NANG` dùng `15000+14000+16000+15000 = 60000` token —
`60000 / 80 = 750` lần nhiều hơn, TRÊN CÙNG một số lượng request. Biến cuối
`cong_bang_theo_request_sai` xác nhận đúng nghịch lý này: SỐ REQUEST bằng
nhau tuyệt đối, nhưng TỔNG TOKEN chênh nhau `750` lần — "công bằng theo
request" là một phép đo SAI chỗ, che giấu mất sự khác biệt thật sự quan
trọng.
::::

::::predict{#doan_mot_request_khong_lo commitOnce}
Xét một danh sách CHỈ có đúng `1` request duy nhất:
`MOT_REQUEST_KHONG_LO = [{"nguoi_dung": "sieu_nang", "so_token": 60000}]`
— dùng ĐÚNG bằng tổng số token của cả `NGUOI_DUNG_NANG` ở trên (`4` request
cộng lại), nhưng gói gọn trong DUY NHẤT một lần gọi.

**Trước khi chạy thử**, bạn đoán: `dem_so_request(NGUOI_DUNG_NHE)` (định
nghĩa ở phần ví dụ, `4` request nhưng tổng chỉ `80` token) so với
`dem_so_request(MOT_REQUEST_KHONG_LO)` (`1` request, `60000` token) — cái
nào LỚN HƠN?

:::opt{correct}
`dem_so_request(NGUOI_DUNG_NHE)` LỚN HƠN (`4 > 1`) — dù `NGUOI_DUNG_NHE`
dùng ÍT HƠN RẤT NHIỀU token (`80` so với `60000`); `dem_so_request` chỉ
đếm ĐỘ DÀI danh sách (`len`), không hề nhìn vào trường `so_token` của bất kỳ
request nào
:::

:::opt
`dem_so_request(MOT_REQUEST_KHONG_LO)` lớn hơn, vì request đó dùng nhiều
token hơn hẳn nên phải được tính "nặng" hơn
::why
Gần đúng ở việc `MOT_REQUEST_KHONG_LO` THẬT SỰ dùng nhiều token hơn — quan
sát đó đúng, và chính LÀ điều `dem_token_da_dung` (không phải
`dem_so_request`) sẽ phản ánh.

Chỗ lệch: `dem_so_request` không hề đọc trường `so_token` — nó chỉ trả về
`len(danh_sach_request)`, tức ĐỘ DÀI của danh sách. Một request "nặng" hàng
chục nghìn token vẫn chỉ LÀ MỘT phần tử trong danh sách, y hệt một request
"nhẹ". Nhầm `dem_so_request` thành một phép đo "trọng số theo token" là
nhầm đúng hai hàm mà cả bài học này cố ý TÁCH RIÊNG để chỉ ra chúng KHÁC
nhau.
::
:::

:::opt
Bằng nhau, vì `dem_so_request` đối xử mọi request như nhau bất kể token
dùng nhiều hay ít
::why
Gần đúng ở việc `dem_so_request` ĐÚNG LÀ đối xử mọi request như nhau về mặt
TRỌNG SỐ — mỗi request, dù nặng hay nhẹ, đều được cộng thêm ĐÚNG `1` vào
tổng đếm. Nguyên tắc đó đúng.

Chỗ lệch: "đối xử như nhau về trọng số" không có nghĩa LÀ "cho ra kết quả
bằng nhau" — `dem_so_request` vẫn trả về ĐỘ DÀI thật của MỖI danh sách, và
hai danh sách này có độ dài KHÁC nhau (`4` phần tử so với `1` phần tử).
`4 ≠ 1` dù mỗi phần tử được đếm cùng trọng số như nhau trong cả hai danh
sách.
::
:::
::::

::::code{#viet_dem_so_request_va_dem_token}
Hoàn thiện `dem_so_request` (trả về độ dài danh sách) và `dem_token_da_dung`
(cộng dồn trường `so_token` của từng request trong danh sách).

```python title=starter
def dem_so_request(danh_sach_request):
    return ___                                              # len(danh_sach_request)


def dem_token_da_dung(danh_sach_request):
    tong = 0
    for yeu_cau in danh_sach_request:
        tong += ___                                          # yeu_cau["so_token"]
    return tong


NGUOI_DUNG_NHE = [
    {"nguoi_dung": "nhe", "so_token": 18},
    {"nguoi_dung": "nhe", "so_token": 20},
    {"nguoi_dung": "nhe", "so_token": 22},
    {"nguoi_dung": "nhe", "so_token": 20},
]
NGUOI_DUNG_NANG = [
    {"nguoi_dung": "nang", "so_token": 15000},
    {"nguoi_dung": "nang", "so_token": 14000},
    {"nguoi_dung": "nang", "so_token": 16000},
    {"nguoi_dung": "nang", "so_token": 15000},
]

so_request_nhe = dem_so_request(NGUOI_DUNG_NHE)
so_request_nang = dem_so_request(NGUOI_DUNG_NANG)
token_nhe = dem_token_da_dung(NGUOI_DUNG_NHE)
token_nang = dem_token_da_dung(NGUOI_DUNG_NANG)

cung_so_request = so_request_nhe == so_request_nang
cong_bang_theo_request_sai = cung_so_request and token_nhe != token_nang

print(so_request_nhe, so_request_nang)
print(token_nhe, token_nang)
print(cung_so_request)
print(cong_bang_theo_request_sai)
```

```python title=solution
def dem_so_request(danh_sach_request):
    return len(danh_sach_request)


def dem_token_da_dung(danh_sach_request):
    tong = 0
    for yeu_cau in danh_sach_request:
        tong += yeu_cau["so_token"]
    return tong


NGUOI_DUNG_NHE = [
    {"nguoi_dung": "nhe", "so_token": 18},
    {"nguoi_dung": "nhe", "so_token": 20},
    {"nguoi_dung": "nhe", "so_token": 22},
    {"nguoi_dung": "nhe", "so_token": 20},
]
NGUOI_DUNG_NANG = [
    {"nguoi_dung": "nang", "so_token": 15000},
    {"nguoi_dung": "nang", "so_token": 14000},
    {"nguoi_dung": "nang", "so_token": 16000},
    {"nguoi_dung": "nang", "so_token": 15000},
]

so_request_nhe = dem_so_request(NGUOI_DUNG_NHE)
so_request_nang = dem_so_request(NGUOI_DUNG_NANG)
token_nhe = dem_token_da_dung(NGUOI_DUNG_NHE)
token_nang = dem_token_da_dung(NGUOI_DUNG_NANG)

cung_so_request = so_request_nhe == so_request_nang
cong_bang_theo_request_sai = cung_so_request and token_nhe != token_nang

print(so_request_nhe, so_request_nang)
print(token_nhe, token_nang)
print(cung_so_request)
print(cong_bang_theo_request_sai)
```

```python title=test
assert dem_so_request(NGUOI_DUNG_NHE) == 4, f"NGUOI_DUNG_NHE co 4 request -- dang ra {dem_so_request(NGUOI_DUNG_NHE)}"
assert dem_so_request(NGUOI_DUNG_NANG) == 4, f"NGUOI_DUNG_NANG cung co 4 request -- dang ra {dem_so_request(NGUOI_DUNG_NANG)}"
assert dem_so_request([]) == 0, f"danh sach rong phai co 0 request -- dang ra {dem_so_request([])}"

assert dem_token_da_dung(NGUOI_DUNG_NHE) == 80, f"NGUOI_DUNG_NHE phai dung DUNG 80 token (18+20+22+20) -- dang ra {dem_token_da_dung(NGUOI_DUNG_NHE)}"
assert dem_token_da_dung(NGUOI_DUNG_NANG) == 60000, f"NGUOI_DUNG_NANG phai dung DUNG 60000 token (15000+14000+16000+15000) -- dang ra {dem_token_da_dung(NGUOI_DUNG_NANG)}"
assert dem_token_da_dung([]) == 0, f"danh sach rong phai dung 0 token -- dang ra {dem_token_da_dung([])}"

assert cung_so_request is True, f"hai nguoi dung phai CUNG so request (4=4) -- dang ra {cung_so_request}"
assert cong_bang_theo_request_sai is True, f"cung so request nhung khac token PHAI xac nhan cong bang theo request la SAI -- dang ra {cong_bang_theo_request_sai}"

MOT_REQUEST_KHONG_LO = [{"nguoi_dung": "sieu_nang", "so_token": 60000}]
assert dem_so_request(MOT_REQUEST_KHONG_LO) == 1, f"MOT_REQUEST_KHONG_LO chi co 1 request -- dang ra {dem_so_request(MOT_REQUEST_KHONG_LO)}"
assert dem_token_da_dung(MOT_REQUEST_KHONG_LO) == 60000, f"MOT_REQUEST_KHONG_LO phai dung DUNG 60000 token -- dang ra {dem_token_da_dung(MOT_REQUEST_KHONG_LO)}"
assert dem_so_request(NGUOI_DUNG_NHE) > dem_so_request(MOT_REQUEST_KHONG_LO), f"NGUOI_DUNG_NHE (4 request) phai co SO REQUEST lon hon MOT_REQUEST_KHONG_LO (1 request), du dung it token hon han -- dang ra {dem_so_request(NGUOI_DUNG_NHE)} vs {dem_so_request(MOT_REQUEST_KHONG_LO)}"
```

:::hints
- kind: attention
  body: "Hai cho trong, o hai ham khac nhau. Cho dau (trong dem_so_request) la GIA TRI TRA VE cua ham do -- do DAI cua chinh danh sach duoc truyen vao, khong lien quan gi toi truong so_token ben trong tung request. Cho hai (trong dem_token_da_dung) nam BEN TRONG vong lap -- gia tri CONG THEM vao tong o MOI vong, doc truong so_token cua request hien tai (bien yeu_cau)."
- kind: strategy
  body: "Cho dau: len(danh_sach_request) -- ham dung san cua Python, tra ve so phan tu cua danh sach. Cho hai: yeu_cau[\"so_token\"] -- doc gia tri ung voi khoa \"so_token\" trong dict yeu_cau (bien vong lap), CONG vao tong qua toan tu +=."
- kind: one-line
  body: "Cho dau la len(danh_sach_request), cho hai la yeu_cau[\"so_token\"]."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: cho trong dau phai la len(danh_sach_request) (do dai danh sach, KHONG doc truong so_token); cho trong hai phai doc yeu_cau["so_token"] (truong cua REQUEST HIEN TAI trong vong lap, khong phai danh_sach_request)
  requireAst:
  - kind: uses-call, target: "len", min: 1
  - kind: uses-name, target: "yeu_cau", min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, trich solution tu chinh
  # file nay, dung tool tu dung o packages/exec-python/dist/kiem-ast.js) --
  # ket qua dung du kien:
  #   "len"=1: DUY NHAT o cho trong dau (return len(danh_sach_request)).
  #   Khong noi nao khac trong toan bo solution goi len().
  #   uses-name("yeu_cau")=1: DUY NHAT o cho trong hai (yeu_cau["so_token"],
  #   mot lan doc/Load). Ten vong lap "yeu_cau" trong "for yeu_cau in ..."
  #   la mot Store, KHONG duoc uses-name dem (uses-name chi dem Load) -- nen
  #   tong THAT dung la 1, khong phai 2.
  # Dien bua "True" vao CA HAI cho trong cho "len"=0 VA uses-name("yeu_cau")=0
  # -- CA HAI luat CHAN DUNG (da CHAY THAT xac nhan qua kiemAst).
  #
  # GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho trong --
  # xac dinh ranh gioi tu chinh khoi starter -- va CHAY THAT qua kiemAst va
  # python3): dien "yeu_cau[\"so_token\"]" vao cho trong dau (dong "return
  # yeu_cau[\"so_token\"]" trong dem_so_request) VA dien
  # "len(danh_sach_request)" vao cho trong hai (dong "tong +=
  # len(danh_sach_request)" trong dem_token_da_dung) -- tong so lan "len" VA
  # uses-name("yeu_cau") tren TOAN BO solution KHONG DOI (van la 1 va 1, chi
  # doi VI TRI) -- da CHAY THAT xac nhan qua kiemAst: static KHONG bat duoc
  # mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong dem_so_request (tham so la
  # danh_sach_request, KHONG co bien "yeu_cau" nao trong scope nay -- ham
  # nay khong co vong lap), bieu thuc moi "yeu_cau[\"so_token\"]" dung mot
  # ten CHUA HE TON TAI trong scope -- NameError NGAY LAP TUC khi
  # dem_so_request(NGUOI_DUNG_NHE) duoc goi lan dau (dong dau tien cua phan
  # demo). Da tu chay THAT qua python3, xac nhan thong bao "name 'yeu_cau'
  # is not defined" -- bi chan boi tier 'run', doc lap voi static.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^4 4\\n80 60000\\nTrue\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`4 = 4` (số request BẰNG nhau), nhưng `80` so với `60000` — chênh `750`
lần. Rate limit theo request đã bị lật tẩy: SAI chỗ, che mất đúng thứ quan
trọng nhất. Câu hỏi tiếp theo: nếu giới hạn ĐÚNG theo token, cơ chế nào
thực thi giới hạn đó — cho một request qua, hay từ chối nó — theo TỪNG lần
gọi?
::::

::::reflect{#nghi-lai}
`dem_so_request` không sai về mặt code — nó làm ĐÚNG những gì tên nó nói:
đếm số lượng. Cái sai LÀ dùng nó LÀM CĂN CỨ cho rate limit, một quyết định
vận hành cần phản ánh đúng CHI PHÍ THẬT (token), không phải SỐ LẦN gọi.
`NGUOI_DUNG_NHE` VÀ `NGUOI_DUNG_NANG` gửi cùng `4` request — một con số
giống hệt nhau — nhưng tiêu tốn tài nguyên GPU khác nhau tới `750` lần.
Bài tiếp theo xây cơ chế THỰC THI giới hạn theo token đó: token bucket —
mỗi người dùng có một "xô" token, mỗi request tiêu thụ đúng phần của nó,
hết xô thì bị từ chối.
::::

::::checkpoint{mastery=0.75}
::::
