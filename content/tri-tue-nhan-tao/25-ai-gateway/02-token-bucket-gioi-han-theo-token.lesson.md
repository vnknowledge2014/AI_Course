---
id: tri-tue-nhan-tao.ai-gateway.token-bucket-gioi-han-theo-token
title: "Token bucket: mỗi request tiêu thụ đúng phần token của nó"
summary: "xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau) tra ve (cho_qua, suc_chua_con_lai_moi): neu so_token_yeu_cau <= suc_chua_con_lai thi CHO QUA va tru dung so_token_yeu_cau khoi xo; neu khong du thi TU CHOI, xo GIU NGUYEN (khong tru gi ca). xu_ly_day_yeu_cau(suc_chua_ban_dau, danh_sach_so_token) chay MOT day request lien tiep tren CUNG mot xo, dem so cho_qua/tu_choi. Voi suc_chua_ban_dau=100 va day [30,40,20,50,10]: 100-30=70 (qua), 70-40=30 (qua), 30-20=10 (qua), 50>10 (TU CHOI, xo giu nguyen o 10), 10<=10 (qua, bien gioi DUNG bang nhau van cho qua, xo con 0) -- ket qua 4 cho qua, 1 tu choi. xu_ly_yeu_cau(0,0) = (True, 0): xo CAN nhung request 0 token van duoc cho qua."
locale: vi
track: tri-tue-nhan-tao
module: ai-gateway
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.token-bucket-gioi-han-theo-token]
requires: [ai.dem-token-thay-vi-dem-request]
concepts: [ai.token-bucket-gioi-han-theo-token]
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
Bài trước đo được: đếm request là SAI chỗ, đếm token mới đúng. Nhưng "đếm"
chỉ LÀ quan sát — một `AI Gateway` thật phải THỰC THI giới hạn đó theo TỪNG
request tới: cho qua, hay từ chối? Cơ chế phổ biến nhất cho việc này LÀ
**token bucket**.
::::

::::explain{#xo_token_co_han_muc}
Hình dung mỗi người dùng có một cái **xô** (bucket) chứa được tối đa
`suc_chua` token. Mỗi request tới mang theo một nhu cầu — nó cần đúng
`so_token_yeu_cau` token để được xử lý:

- Nếu xô CÒN ĐỦ (`so_token_yeu_cau <= suc_chua_con_lai`): request được
  **cho qua**, VÀ đúng `so_token_yeu_cau` bị TRỪ khỏi xô.
- Nếu xô KHÔNG ĐỦ: request bị **từ chối** — VÀ xô GIỮ NGUYÊN, không trừ gì
  cả (request bị từ chối không hề "dùng" token nào).

```
xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau):
  neu so_token_yeu_cau <= suc_chua_con_lai:
    tra ve (cho_qua=True, suc_chua_con_lai - so_token_yeu_cau)
  nguoc lai:
    tra ve (cho_qua=False, suc_chua_con_lai KHONG DOI)
```

Đây LÀ bản rút gọn của thuật toán "token bucket" thật — bản đầy đủ còn NẠP
LẠI token theo thời gian trôi qua, nhưng vì không có đồng hồ thật để mô
phỏng ở đây, bài này chỉ xét MỘT xô, xử lý MỘT dãy request LIÊN TIẾP, không
nạp lại giữa chừng.

Điểm biên quan trọng: dấu `<=` (không phải `<`). Một request cần ĐÚNG BẰNG
số token còn lại VẪN được cho qua — xô cạn về đúng `0`, chứ không bị từ
chối oan.
::::

::::example{#day_nam_request_tren_mot_xo}
```python title=readonly
def xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau):
    if so_token_yeu_cau <= suc_chua_con_lai:
        return True, suc_chua_con_lai - so_token_yeu_cau
    return False, suc_chua_con_lai


def xu_ly_day_yeu_cau(suc_chua_ban_dau, danh_sach_so_token):
    suc_chua_con_lai = suc_chua_ban_dau
    so_cho_qua = 0
    so_tu_choi = 0
    for so_token in danh_sach_so_token:
        cho_qua, suc_chua_con_lai = xu_ly_yeu_cau(suc_chua_con_lai, so_token)
        if cho_qua:
            so_cho_qua += 1
        else:
            so_tu_choi += 1
    return so_cho_qua, so_tu_choi


SUC_CHUA_BAN_DAU = 100
DAY_YEU_CAU = [30, 40, 20, 50, 10]

so_cho_qua, so_tu_choi = xu_ly_day_yeu_cau(SUC_CHUA_BAN_DAU, DAY_YEU_CAU)

print(so_cho_qua)
print(so_tu_choi)
```

```text title=readonly
4
1
```

Xô bắt đầu với `100` token. Request `30`: `30 <= 100`, cho qua, xô còn
`100-30 = 70`. Request `40`: `40 <= 70`, cho qua, xô còn `70-40 = 30`.
Request `20`: `20 <= 30`, cho qua, xô còn `30-20 = 10`. Request `50`:
`50` KHÔNG `<= 10` — TỪ CHỐI, xô GIỮ NGUYÊN ở `10`. Request `10`: `10 <= 10`
đúng ở đúng BIÊN — cho qua, xô còn `10-10 = 0`. Tổng cộng: `4` request được
cho qua, `1` request (request `50`) bị từ chối vì tại thời điểm nó tới, xô
chỉ còn `10` — không đủ.
::::

::::predict{#doan_xo_can_yeu_cau_khong_ton_token commitOnce}
Xét `xu_ly_yeu_cau(0, 0)` — xô đã CẠN HOÀN TOÀN (`suc_chua_con_lai = 0`),
và request này cần `0` token (ví dụ một lệnh kiểm tra trạng thái, không xử
lý gì tốn GPU).

**Trước khi chạy thử**, bạn đoán: `xu_ly_yeu_cau(0, 0)` trả về gì?

:::opt{correct}
`(True, 0)` — vì `0 <= 0` đúng (điều kiện dùng `<=`, và `0` bằng `0`),
request được cho qua; xô sau đó là `0 - 0 = 0`, vẫn cạn như trước, nhưng
KHÔNG bị từ chối
:::

:::opt
`(False, 0)` — xô đã cạn (`0`) thì MỌI request phải bị từ chối, kể cả một
request `0` token
::why
Gần đúng ở việc xô ĐANG cạn (`suc_chua_con_lai = 0`) — quan sát về trạng
thái xô đó đúng.

Chỗ lệch: điều kiện cho qua LÀ `so_token_yeu_cau <= suc_chua_con_lai`, tức
`0 <= 0` — biểu thức này đúng (`True`), không sai. Một xô cạn KHÔNG chặn
được một request cần ĐÚNG `0` token, vì `0` VẪN nằm trong phần "còn đủ" của
phép so sánh `<=`. Từ chối request này là hiểu nhầm "xô cạn" thành "chặn
tất cả", trong khi thuật toán chỉ chặn request cần NHIỀU HƠN những gì xô
còn lại.
::
:::

:::opt
Chương trình sẽ báo lỗi, vì không thể xử lý một request `0` token — request
`0` token không có ý nghĩa
::why
Gần đúng ở việc một request `0` token nghe có vẻ "không cần xử lý gì" —
trực giác đó dễ khiến người ta nghĩ nó là trường hợp đặc biệt cần chặn.

Chỗ lệch: `xu_ly_yeu_cau` không hề kiểm tra `so_token_yeu_cau` có bằng `0`
hay không một cách riêng biệt — nó chỉ so sánh `so_token_yeu_cau` với
`suc_chua_con_lai` bằng phép `<=` như MỌI request khác. `0 <= 0` là một
phép so sánh số học hợp lệ, không hề ném lỗi.
::
:::
::::

::::code{#viet_xu_ly_yeu_cau}
Hoàn thiện `xu_ly_yeu_cau`: điều kiện kiểm tra xô còn ĐỦ hay không (dùng
`<=`), và phần token còn lại sau khi cho qua (trừ đi đúng số đã tiêu thụ).

```python title=starter
def xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau):
    if ___:                                                  # so_token_yeu_cau <= suc_chua_con_lai
        return True, ___                                     # suc_chua_con_lai - so_token_yeu_cau
    return False, suc_chua_con_lai


def xu_ly_day_yeu_cau(suc_chua_ban_dau, danh_sach_so_token):
    suc_chua_con_lai = suc_chua_ban_dau
    so_cho_qua = 0
    so_tu_choi = 0
    for so_token in danh_sach_so_token:
        cho_qua, suc_chua_con_lai = xu_ly_yeu_cau(suc_chua_con_lai, so_token)
        if cho_qua:
            so_cho_qua += 1
        else:
            so_tu_choi += 1
    return so_cho_qua, so_tu_choi


SUC_CHUA_BAN_DAU = 100
DAY_YEU_CAU = [30, 40, 20, 50, 10]

so_cho_qua, so_tu_choi = xu_ly_day_yeu_cau(SUC_CHUA_BAN_DAU, DAY_YEU_CAU)

print(so_cho_qua)
print(so_tu_choi)
```

```python title=solution
def xu_ly_yeu_cau(suc_chua_con_lai, so_token_yeu_cau):
    if so_token_yeu_cau <= suc_chua_con_lai:
        return True, suc_chua_con_lai - so_token_yeu_cau
    return False, suc_chua_con_lai


def xu_ly_day_yeu_cau(suc_chua_ban_dau, danh_sach_so_token):
    suc_chua_con_lai = suc_chua_ban_dau
    so_cho_qua = 0
    so_tu_choi = 0
    for so_token in danh_sach_so_token:
        cho_qua, suc_chua_con_lai = xu_ly_yeu_cau(suc_chua_con_lai, so_token)
        if cho_qua:
            so_cho_qua += 1
        else:
            so_tu_choi += 1
    return so_cho_qua, so_tu_choi


SUC_CHUA_BAN_DAU = 100
DAY_YEU_CAU = [30, 40, 20, 50, 10]

so_cho_qua, so_tu_choi = xu_ly_day_yeu_cau(SUC_CHUA_BAN_DAU, DAY_YEU_CAU)

print(so_cho_qua)
print(so_tu_choi)
```

```python title=test
assert xu_ly_yeu_cau(100, 30) == (True, 70), f"100 con lai, yeu cau 30 -- phai cho qua, con lai 70 -- dang ra {xu_ly_yeu_cau(100, 30)}"
assert xu_ly_yeu_cau(10, 50) == (False, 10), f"10 con lai, yeu cau 50 -- phai TU CHOI, xo giu nguyen 10 -- dang ra {xu_ly_yeu_cau(10, 50)}"
assert xu_ly_yeu_cau(10, 10) == (True, 0), f"bien gioi: 10 con lai, yeu cau DUNG 10 -- phai CHO QUA (dung <=), xo con 0 -- dang ra {xu_ly_yeu_cau(10, 10)}"
assert xu_ly_yeu_cau(0, 0) == (True, 0), f"xo can (0), yeu cau 0 token -- van phai CHO QUA -- dang ra {xu_ly_yeu_cau(0, 0)}"
assert xu_ly_yeu_cau(0, 1) == (False, 0), f"xo can (0), yeu cau 1 token -- phai TU CHOI -- dang ra {xu_ly_yeu_cau(0, 1)}"

assert xu_ly_day_yeu_cau(100, [30, 40, 20, 50, 10]) == (4, 1), f"day 5 request tren xo 100 phai cho 4 cho qua, 1 tu choi -- dang ra {xu_ly_day_yeu_cau(100, [30, 40, 20, 50, 10])}"
assert xu_ly_day_yeu_cau(100, []) == (0, 0), f"day rong phai cho (0, 0) -- dang ra {xu_ly_day_yeu_cau(100, [])}"
assert xu_ly_day_yeu_cau(50, [50]) == (1, 0), f"xo 50, mot request dung 50 -- phai cho qua -- dang ra {xu_ly_day_yeu_cau(50, [50])}"
assert xu_ly_day_yeu_cau(49, [50]) == (0, 1), f"xo CHI 49 (it hon 1 don vi), mot request 50 -- phai TU CHOI -- xac nhan suc_chua_ban_dau THAT SU rang buoc ket qua, dang ra {xu_ly_day_yeu_cau(49, [50])}"
assert so_cho_qua == 4, f"bien so_cho_qua (demo) phai la 4 -- dang ra {so_cho_qua}"
assert so_tu_choi == 1, f"bien so_tu_choi (demo) phai la 1 -- dang ra {so_tu_choi}"
```

:::hints
- kind: attention
  body: "Hai cho trong, cung mot ham xu_ly_yeu_cau. Cho dau la DIEU KIEN cua if -- so sanh so_token_yeu_cau voi suc_chua_con_lai, dung dau <= (KHONG phai <, vi bien gioi bang nhau van phai duoc cho qua). Cho hai la GIA TRI THU HAI cua tuple tra ve khi CHO QUA -- so token CON LAI sau khi tru di phan da tieu thu."
- kind: strategy
  body: "Cho dau: so_token_yeu_cau <= suc_chua_con_lai -- neu yeu cau KHONG VUOT QUA phan con lai thi cho qua. Cho hai: suc_chua_con_lai - so_token_yeu_cau -- tru dung so token vua tieu thu khoi xo."
- kind: one-line
  body: "Cho dau la so_token_yeu_cau <= suc_chua_con_lai, cho hai la suc_chua_con_lai - so_token_yeu_cau."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: cho trong dau phai la phep so sanh so_token_yeu_cau <= suc_chua_con_lai (dung <=, khong phai <, vi bien gioi bang nhau van cho qua); cho trong hai phai la phep TRU suc_chua_con_lai - so_token_yeu_cau
  requireAst:
  - kind: uses-operator, target: "<=", min: 1
  - kind: uses-operator, target: "-", min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, trich solution tu chinh
  # file nay) -- ket qua dung du kien:
  #   "<="=1: DUY NHAT o cho trong dau (so_token_yeu_cau <=
  #   suc_chua_con_lai). Khong noi nao khac trong solution dung <=.
  #   "-"=1: DUY NHAT o cho trong hai (suc_chua_con_lai -
  #   so_token_yeu_cau). Ham xu_ly_day_yeu_cau khong dung phep tru nao.
  # Dien bua "True" vao CA HAI cho trong ("if True:" va "return True,
  # True") cho "<="=0 VA "-"=0 -- CA HAI luat CHAN DUNG (da CHAY THAT xac
  # nhan qua kiemAst).
  #
  # GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho trong --
  # xac dinh ranh gioi tu chinh khoi starter -- va CHAY THAT qua kiemAst):
  # dien "suc_chua_con_lai - so_token_yeu_cau" vao cho trong dau (dong "if
  # suc_chua_con_lai - so_token_yeu_cau:") VA dien "so_token_yeu_cau <=
  # suc_chua_con_lai" vao cho trong hai (dong "return True,
  # so_token_yeu_cau <= suc_chua_con_lai") -- tong so lan "<=" VA "-" tren
  # TOAN BO solution KHONG DOI (van la 1 va 1, chi doi VI TRI) -- da CHAY
  # THAT xac nhan qua kiemAst: static KHONG bat duoc mutant nay.
  # Mutant nay KHONG gay loi cu phap hay NameError -- ca hai bieu thuc van
  # dung duoc TEN HOP LE trong scope cua ham. Nhung no SAI VE HANH VI, va bi
  # tier 'tests' bat NGAY o assert DAU TIEN: da tu chay THAT qua python3,
  # xu_ly_yeu_cau(100, 30) voi mutant nay tra ve (True, True) (vi
  # "100 - 30 = 70" la GIA TRI SO khac 0 nen if-True, roi tra ve "True,
  # (30 <= 100)" = "True, True") -- khac han ket qua dung (True, 70). Rong
  # hon: CA BON test-case xu_ly_yeu_cau trong khoi test (100/30, 10/50,
  # 10/10, 0/0) deu cho ket qua KHAC voi mutant nay so voi ban goc (da tu
  # chay THAT xac nhan tung truong hop) -- tier 'tests' chan mutant nay o
  # nhieu diem doc lap nhau.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^4\\n1\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`4` cho qua, `1` từ chối — trên MỘT xô, một dãy request cụ thể. Nhưng "cho
qua" mới chỉ LÀ nửa đầu của cổng — TRƯỚC KHI chạm tới xô token, cổng còn
phải biết request này ĐẾN TỪ AI, và người đó CÓ ĐƯỢC PHÉP gọi API hay
không.
::::

::::reflect{#nghi-lai}
`xu_ly_yeu_cau` không mô phỏng "thời gian trôi qua" hay "nạp lại token" —
phiên bản rút gọn này chỉ xử lý MỘT dãy request liên tiếp trên một xô có
sẵn, và đó LÀ đủ để thấy rõ điều quan trọng nhất: dấu `<=` (không phải
`<`) ở đúng biên, VÀ việc TỪ CHỐI không hề trừ token (khác với CHO QUA).
Bài tiếp theo thêm một bước diễn ra TRƯỚC bước này: xác thực xem request có
mang một API key HỢP LỆ hay không — một request với key sai phải bị từ
chối NGAY, không bao giờ được chạm tới xô token của bài này.
::::

::::checkpoint{mastery=0.76}
::::
