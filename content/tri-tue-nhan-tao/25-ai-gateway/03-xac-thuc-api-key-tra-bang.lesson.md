---
id: tri-tue-nhan-tao.ai-gateway.xac-thuc-api-key-tra-bang
title: "Xác thực API key: tra bảng cố định, từ chối TRƯỚC khi chạm token bucket"
summary: "xac_thuc(api_key) = BANG_NGUOI_DUNG.get(api_key) -- tra mot dict CO DINH, tra ve thong tin nguoi dung neu key hop le, None neu KHONG (khong nem loi cho key la, dung khuon .get cua q8.6a bai 1). xu_ly_yeu_cau_co_xac_thuc(api_key, suc_chua_con_lai, so_token_yeu_cau) ghep xac_thuc VOI xu_ly_yeu_cau (bai truoc): neu nguoi_dung is None tra ve ('tu_choi_sai_key', suc_chua_con_lai) NGAY, TRUOC KHI cham toi xu_ly_yeu_cau. xu_ly_yeu_cau_co_xac_thuc('key-la', 100, 30) tra ve ('tu_choi_sai_key', 100) MAC DU suc_chua_con_lai=100 THUA du cho yeu cau 30 token -- xac nhan kiem tra key xay ra TRUOC, doc lap voi token con lai bao nhieu."
locale: vi
track: tri-tue-nhan-tao
module: ai-gateway
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.xac-thuc-api-key-tra-bang]
requires: [ai.token-bucket-gioi-han-theo-token]
concepts: [ai.xac-thuc-api-key-tra-bang]
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
Bài trước kết thúc bằng một câu hỏi bỏ ngỏ: TRƯỚC KHI một request chạm tới
xô token, cổng phải biết nó đến từ AI. `Chương 39.3` gọi bước này LÀ
**xác thực API Key** — và nó phải xảy ra ĐẦU TIÊN, trước MỌI bước khác.
::::

::::explain{#tra_bang_khong_nem_loi}
Xác thực bằng API key, trong dạng đơn giản nhất, LÀ một phép **tra bảng**:
một `dict` cố định ánh xạ MỖI key hợp lệ tới thông tin người dùng sở hữu nó
(tên, hạn mức...). Đây LÀ đúng khuôn đã dùng để phân loại tầng của `AI
Stack` (`q8.6a` bài `1`): `BANG_TANG.get(...)`.

```
xac_thuc(api_key) = BANG_NGUOI_DUNG.get(api_key)
```

Điểm mấu chốt, LẶP LẠI từ `q8.6a` bài `1`: dùng `.get(api_key)`, KHÔNG dùng
`BANG_NGUOI_DUNG[api_key]`. Một API key LẠ (không có trong bảng) không làm
chương trình NỔ với `KeyError` — nó trả về `None`, một tín hiệu TƯỜNG MINH
rằng "key này không hợp lệ", để bước sau XỬ LÝ tín hiệu đó một cách có chủ
đích, thay vì crash.

Ghép với bài trước: một request có API key KHÔNG hợp lệ phải bị từ chối
**NGAY LẬP TỨC** — TRƯỚC KHI chạm tới bước kiểm tra token bucket. Thứ tự
này quan trọng: không có lý do gì để tốn công kiểm tra xô token của một
người dùng mà cổng thậm chí còn không biết là AI.
::::

::::example{#xac_thuc_roi_moi_kiem_token}
```python title=readonly
def xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau):
    if so_token_yeu_cau <= suc_chua_con_lai:
        return True, suc_chua_con_lai - so_token_yeu_cau
    return False, suc_chua_con_lai


BANG_NGUOI_DUNG = {
    "key-abc123": {"ten": "an", "han_muc": 1000},
    "key-def456": {"ten": "binh", "han_muc": 5000},
    "key-ghi789": {"ten": "chi", "han_muc": 200},
}


def xac_thuc(api_key):
    return BANG_NGUOI_DUNG.get(api_key)


def xu_ly_yeu_cau_co_xac_thuc(api_key, suc_chua_con_lai, so_token_yeu_cau):
    nguoi_dung = xac_thuc(api_key)
    if nguoi_dung is None:
        return "tu_choi_sai_key", suc_chua_con_lai
    cho_qua, suc_chua_con_lai_moi = xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau)
    if not cho_qua:
        return "tu_choi_het_token", suc_chua_con_lai_moi
    return "cho_qua", suc_chua_con_lai_moi


print(xac_thuc("key-abc123"))
print(xac_thuc("key-la"))
print(xu_ly_yeu_cau_co_xac_thuc("key-abc123", 100, 30))
print(xu_ly_yeu_cau_co_xac_thuc("key-la", 100, 30))
print(xu_ly_yeu_cau_co_xac_thuc("key-abc123", 10, 50))
print(xu_ly_yeu_cau_co_xac_thuc("key-la", 5, 1000000))
```

```text title=readonly
{'ten': 'an', 'han_muc': 1000}
None
('cho_qua', 70)
('tu_choi_sai_key', 100)
('tu_choi_het_token', 10)
('tu_choi_sai_key', 5)
```

`xac_thuc("key-abc123")` trả về đúng thông tin người dùng `an`;
`xac_thuc("key-la")` (không có trong `BANG_NGUOI_DUNG`) trả về `None` —
KHÔNG ném lỗi. `xu_ly_yeu_cau_co_xac_thuc("key-abc123", 100, 30)`: key hợp
lệ, xô còn `100` đủ cho `30` — `("cho_qua", 70)`.
`xu_ly_yeu_cau_co_xac_thuc("key-la", 100, 30)`: key SAI — trả về ngay
`("tu_choi_sai_key", 100)`, xô GIỮ NGUYÊN `100` (không hề chạm tới bước
kiểm tra token, dù `100` THỪA đủ cho `30`).
`xu_ly_yeu_cau_co_xac_thuc("key-abc123", 10, 50)`: key hợp lệ nhưng xô chỉ
còn `10`, không đủ cho `50` — `("tu_choi_het_token", 10)`. Dòng cuối cùng
LÀ minh chứng rõ nhất cho THỨ TỰ kiểm tra: `xu_ly_yeu_cau_co_xac_thuc(
"key-la", 5, 1000000)` — key sai, VÀ số token yêu cầu (`1000000`) lớn hơn
RẤT NHIỀU so với xô còn lại (`5`) — nhưng kết quả vẫn LÀ
`("tu_choi_sai_key", 5)`, không phải `"tu_choi_het_token"`. Việc kiểm tra
key sai xảy ra TRƯỚC, nên hàm không bao giờ "nhìn thấy" con số `1000000` bất
thường đó.
::::

::::predict{#doan_key_sai_va_token_khong_lo commitOnce}
Xét gọi `xu_ly_yeu_cau_co_xac_thuc("key-la", 5, 1000000)` — CHÍNH XÁC như
dòng cuối của ví dụ trên: `"key-la"` KHÔNG có trong `BANG_NGUOI_DUNG`, VÀ
`so_token_yeu_cau` (`1000000`) vượt xa xô còn lại (`5`, chênh nhau
`199998` lần).

**Trước khi chạy thử**, bạn đoán: kết quả trả về LÀ gì?

:::opt{correct}
`("tu_choi_sai_key", 5)` — kiểm tra API key xảy ra TRƯỚC TIÊN; hàm trả về
ngay khi thấy `nguoi_dung is None`, không bao giờ chạy tới dòng gọi
`xu_ly_yeu_cau`, bất kể `so_token_yeu_cau` có lớn bất thường tới đâu
:::

:::opt
`("tu_choi_het_token", 5)` — vì `1000000` token VƯỢT XA số còn lại, request
này chắc chắn phải bị từ chối VÌ HẾT TOKEN
::why
Gần đúng ở việc `1000000` token THẬT SỰ vượt xa xô còn lại (`5`) — nếu
`xu_ly_yeu_cau` được gọi, nó chắc chắn sẽ trả về `cho_qua=False`. Quan sát
về ĐỘ LỆCH token đó đúng.

Chỗ lệch: `xu_ly_yeu_cau_co_xac_thuc` kiểm tra `nguoi_dung is None` TRƯỚC
khi gọi `xu_ly_yeu_cau` — với `api_key="key-la"`, điều kiện đó đúng ngay từ
đầu, và hàm `return "tu_choi_sai_key", ...` LẬP TỨC, không bao giờ chạy tới
dòng `xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau)`. Con số
`1000000` không hề được nhìn tới trong lần gọi này.
::
:::

:::opt
Chương trình sẽ NỔ với một lỗi số học, vì `so_token_yeu_cau` lớn hơn
`suc_chua_con_lai` một cách bất thường
::why
Gần đúng ở việc `1000000` VÀ `5` chênh lệch rất lớn — trực giác "một con số
bất thường có thể gây lỗi" không phải lúc nào cũng sai trong lập trình nói
chung.

Chỗ lệch: `xu_ly_yeu_cau` chỉ so sánh hai số bằng `<=` — không có phép toán
nào (chia, căn, chỉ số mảng...) có thể "nổ" vì một số quá lớn ở đây. Hơn
nữa, với `api_key` sai, hàm còn KHÔNG BAO GIỜ chạm tới `xu_ly_yeu_cau` — nó
trả về sớm ở bước xác thực, một nhánh hoàn toàn không liên quan tới phép so
sánh số học nào.
::
:::
::::

::::code{#viet_xac_thuc_va_ghep_voi_token_bucket}
Hoàn thiện `xac_thuc` (tra `BANG_NGUOI_DUNG` qua `.get`, không ném lỗi cho
key lạ) và điều kiện từ chối sớm trong `xu_ly_yeu_cau_co_xac_thuc` (kiểm
tra key sai TRƯỚC KHI chạm tới token bucket).

```python title=starter
def xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau):
    if so_token_yeu_cau <= suc_chua_con_lai:
        return True, suc_chua_con_lai - so_token_yeu_cau
    return False, suc_chua_con_lai


BANG_NGUOI_DUNG = {
    "key-abc123": {"ten": "an", "han_muc": 1000},
    "key-def456": {"ten": "binh", "han_muc": 5000},
    "key-ghi789": {"ten": "chi", "han_muc": 200},
}


def xac_thuc(api_key):
    return ___                                              # BANG_NGUOI_DUNG.get(api_key)


def xu_ly_yeu_cau_co_xac_thuc(api_key, suc_chua_con_lai, so_token_yeu_cau):
    nguoi_dung = xac_thuc(api_key)
    if ___:                                                 # nguoi_dung is None
        return "tu_choi_sai_key", suc_chua_con_lai
    cho_qua, suc_chua_con_lai_moi = xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau)
    if not cho_qua:
        return "tu_choi_het_token", suc_chua_con_lai_moi
    return "cho_qua", suc_chua_con_lai_moi


print(xac_thuc("key-abc123"))
print(xac_thuc("key-la"))
print(xu_ly_yeu_cau_co_xac_thuc("key-abc123", 100, 30))
print(xu_ly_yeu_cau_co_xac_thuc("key-la", 100, 30))
print(xu_ly_yeu_cau_co_xac_thuc("key-abc123", 10, 50))
print(xu_ly_yeu_cau_co_xac_thuc("key-la", 5, 1000000))
```

```python title=solution
def xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau):
    if so_token_yeu_cau <= suc_chua_con_lai:
        return True, suc_chua_con_lai - so_token_yeu_cau
    return False, suc_chua_con_lai


BANG_NGUOI_DUNG = {
    "key-abc123": {"ten": "an", "han_muc": 1000},
    "key-def456": {"ten": "binh", "han_muc": 5000},
    "key-ghi789": {"ten": "chi", "han_muc": 200},
}


def xac_thuc(api_key):
    return BANG_NGUOI_DUNG.get(api_key)


def xu_ly_yeu_cau_co_xac_thuc(api_key, suc_chua_con_lai, so_token_yeu_cau):
    nguoi_dung = xac_thuc(api_key)
    if nguoi_dung is None:
        return "tu_choi_sai_key", suc_chua_con_lai
    cho_qua, suc_chua_con_lai_moi = xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau)
    if not cho_qua:
        return "tu_choi_het_token", suc_chua_con_lai_moi
    return "cho_qua", suc_chua_con_lai_moi


print(xac_thuc("key-abc123"))
print(xac_thuc("key-la"))
print(xu_ly_yeu_cau_co_xac_thuc("key-abc123", 100, 30))
print(xu_ly_yeu_cau_co_xac_thuc("key-la", 100, 30))
print(xu_ly_yeu_cau_co_xac_thuc("key-abc123", 10, 50))
print(xu_ly_yeu_cau_co_xac_thuc("key-la", 5, 1000000))
```

```python title=test
assert xac_thuc("key-abc123") == {"ten": "an", "han_muc": 1000}, f"key-abc123 phai tra ve dung thong tin nguoi dung an -- dang ra {xac_thuc('key-abc123')}"
assert xac_thuc("key-def456") == {"ten": "binh", "han_muc": 5000}, f"key-def456 phai tra ve dung thong tin nguoi dung binh -- dang ra {xac_thuc('key-def456')}"
assert xac_thuc("key-khong-ton-tai") is None, f"key khong co trong bang phai tra ve None, KHONG duoc nem loi -- dang ra {xac_thuc('key-khong-ton-tai')}"
assert xac_thuc("") is None, f"chuoi rong cung phai tra ve None -- dang ra {xac_thuc('')}"

assert xu_ly_yeu_cau_co_xac_thuc("key-abc123", 100, 30) == ("cho_qua", 70), f"key hop le, xo du -- phai cho_qua, con lai 70 -- dang ra {xu_ly_yeu_cau_co_xac_thuc('key-abc123', 100, 30)}"
assert xu_ly_yeu_cau_co_xac_thuc("key-la", 100, 30) == ("tu_choi_sai_key", 100), f"key sai -- phai tu_choi_sai_key NGAY, xo giu nguyen 100 -- dang ra {xu_ly_yeu_cau_co_xac_thuc('key-la', 100, 30)}"
assert xu_ly_yeu_cau_co_xac_thuc("key-abc123", 10, 50) == ("tu_choi_het_token", 10), f"key hop le nhung xo khong du -- phai tu_choi_het_token -- dang ra {xu_ly_yeu_cau_co_xac_thuc('key-abc123', 10, 50)}"
assert xu_ly_yeu_cau_co_xac_thuc("key-la", 5, 1000000) == ("tu_choi_sai_key", 5), f"key sai PHAI duoc kiem TRUOC, du so_token_yeu_cau lon bat thuong -- dang ra {xu_ly_yeu_cau_co_xac_thuc('key-la', 5, 1000000)}"
```

:::hints
- kind: attention
  body: "Hai cho trong, o hai ham khac nhau. Cho dau (trong xac_thuc) la GIA TRI TRA VE -- tra BANG_NGUOI_DUNG bang .get, KHONG duoc dung [] truc tiep (se nem KeyError cho key la). Cho hai (trong xu_ly_yeu_cau_co_xac_thuc) la DIEU KIEN cua if -- kiem tra bien nguoi_dung (vua gan o dong truoc) co phai None hay khong, dung toan tu is."
- kind: strategy
  body: "Cho dau: BANG_NGUOI_DUNG.get(api_key) -- .get voi MOT doi so, mac dinh tra None khi khong tim thay (khong can ghi ro None lam doi so thu hai). Cho hai: nguoi_dung is None -- so sanh danh tinh voi None, dung is (khong dung ==)."
- kind: one-line
  body: "Cho dau la BANG_NGUOI_DUNG.get(api_key), cho hai la nguoi_dung is None."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: cho trong dau phai dung BANG_NGUOI_DUNG.get(api_key) (tra bang qua .get, KHONG dung [] truc tiep); cho trong hai phai kiem tra nguoi_dung is None (dung toan tu is)
  requireAst:
  - kind: uses-call, target: "get", min: 1
  - kind: uses-operator, target: "is", min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, trich solution tu chinh
  # file nay) -- ket qua dung du kien:
  #   "get"=1: DUY NHAT o cho trong dau (BANG_NGUOI_DUNG.get(api_key)).
  #   Khong noi nao khac trong solution goi .get.
  #   "is"=1: DUY NHAT o cho trong hai (nguoi_dung is None). Khong co "is"
  #   nao khac trong file (kiem tra "not cho_qua" dung tu khoa not, khong
  #   phai toan tu is).
  # Dien bua "True" vao CA HAI cho trong ("return True" va "if True:") cho
  # "get"=0 VA "is"=0 -- CA HAI luat CHAN DUNG (da CHAY THAT xac nhan qua
  # kiemAst).
  #
  # GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho trong --
  # xac dinh ranh gioi tu chinh khoi starter -- va CHAY THAT qua kiemAst va
  # python3): dien "nguoi_dung is None" vao cho trong dau (dong "return
  # nguoi_dung is None" trong xac_thuc) VA dien
  # "BANG_NGUOI_DUNG.get(api_key)" vao cho trong hai (dong "if
  # BANG_NGUOI_DUNG.get(api_key):" trong xu_ly_yeu_cau_co_xac_thuc) -- tong
  # so lan "get" VA "is" tren TOAN BO solution KHONG DOI (van la 1 va 1,
  # chi doi VI TRI) -- da CHAY THAT xac nhan qua kiemAst: static KHONG bat
  # duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong xac_thuc (tham so la
  # api_key, KHONG co bien "nguoi_dung" nao trong scope nay), bieu thuc moi
  # "nguoi_dung is None" dung mot ten CHUA HE TON TAI -- NameError NGAY LAP
  # TUC khi xac_thuc("key-abc123") duoc goi lan dau (dong dau tien cua phan
  # demo). Da tu chay THAT qua python3, xac nhan thong bao "name
  # 'nguoi_dung' is not defined" -- bi chan boi tier 'run', doc lap voi
  # static.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^\\{'ten': 'an', 'han_muc': 1000\\}\\nNone\\n\\('cho_qua', 70\\)\\n\\('tu_choi_sai_key', 100\\)\\n\\('tu_choi_het_token', 10\\)\\n\\('tu_choi_sai_key', 5\\)\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Key sai bị chặn ngay, KHÔNG cần biết token còn bao nhiêu — thứ tự kiểm tra
đã đúng. Ba mảnh đã có: đếm đúng theo token, thực thi giới hạn bằng xô, xác
thực trước khi chạm xô. Còn thiếu một mảnh: cổng đứng trước NHIỀU node GPU
— request đã qua cả hai vòng kiểm tra thì nên ĐI TỚI ĐÂU?
::::

::::reflect{#nghi-lai}
`xu_ly_yeu_cau_co_xac_thuc` không thêm khái niệm mới nào cho việc xác thực
hay token bucket — nó RÁP LẠI hai hàm đã có (`xac_thuc`, `xu_ly_yeu_cau`)
theo đúng MỘT thứ tự: xác thực TRƯỚC, token bucket SAU. Thứ tự đó không
phải chi tiết vụn vặt — dòng cuối của ví dụ (`"key-la"` VỚI
`so_token_yeu_cau=1000000`) cho thấy: nếu đảo ngược thứ tự, một request với
key sai có thể vô tình bị gắn nhãn "hết token" thay vì "sai key" — hai lý do
từ chối khác nhau, cần được phân biệt RÕ RÀNG cho người vận hành cổng. Bài
tiếp theo rời khỏi câu hỏi "có nên phục vụ request này không" để hỏi câu
hỏi tiếp theo: nếu phục vụ, PHỤC VỤ Ở ĐÂU — chọn giữa nhiều node GPU như thế
nào?
::::

::::checkpoint{mastery=0.77}
::::
