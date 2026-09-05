---
id: tri-tue-nhan-tao.boss-dung-llm-dung-cach.llm-lam-giam-khao
title: "LLM làm giám khảo: chấm theo rubric, không phải đếm từ chung"
summary: "giam_khao_mo_phong(cau_tra_loi, dap_an_chuan) mo phong 'LLM lam giam khao' bang MOT LUAT TAT DINH (khong phai mang no-ron that hieu cau tra loi): 'tu_choi' neu chua cum 'khong biet'; neu khong, 'dung' CHI khi VUA dung so tien VUA dung don vi tien te; con lai la 'sai'. Cham 6 cap (cau tra loi, dap an chuan) phu ba nhan: phan bo {'dung': 2, 'sai': 2, 'tu_choi': 2} -- can bang deu tren ca ba nhan, chay that. Uu tien tu choi: mot cau vua chua 'khong biet' vua TINH CO co dung so van bi cham 'tu_choi', khong phai 'dung'."
locale: vi
track: tri-tue-nhan-tao
module: boss-dung-llm-dung-cach
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.llm-lam-giam-khao]
requires: [ai.danh-gia-tu-dong]
concepts: [ai.llm-lam-giam-khao]
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
Bài trước đếm từ chung — hữu ích, nhưng mù mờ về Ý NGHĨA của những từ đó.
Bài này thử một cách khác: đặt MỘT LLM (mô phỏng) vào vai giám khảo, chấm
đầu ra của một LLM (mô phỏng) khác — theo một RUBRIC rõ ràng, không phải
đếm chữ.
::::

::::explain{#llm_lam_giam_khao_la_gi}
**LLM-làm-giám-khảo** (LLM-as-judge) là kỹ thuật dùng MỘT mô hình ngôn ngữ
thứ hai để CHẤM đầu ra của mô hình thứ nhất, thay vì so khớp chuỗi máy
móc. Trong thực tế, giám khảo đó thường là một LLM thật, được giao một
**rubric** — một tập luật rõ ràng nói thế nào là "đúng", thế nào là "sai".

Track này mô phỏng đúng VAI TRÒ đó bằng một hàm Python thuần, tất định —
`giam_khao_mo_phong(cau_tra_loi, dap_an_chuan)` — áp đúng MỘT rubric cố
định, theo thứ tự ưu tiên:

> **`"tu_choi"`** — nếu câu trả lời chứa cụm "không biết" (một LLM từ chối
> trả lời). Kiểm TRƯỚC TIÊN, ưu tiên cao nhất — một câu từ chối không cần
> xét gì thêm.
>
> **`"dung"`** — nếu KHÔNG từ chối, VÀ chứa ĐÚNG số tiền mong đợi VÀ ĐÚNG
> đơn vị tiền tệ mong đợi. Phải ĐỦ CẢ HAI — thiếu một trong hai không được
> tính là đúng.
>
> **`"sai"`** — mọi trường hợp còn lại (số sai, đơn vị thiếu, hoặc cả
> hai).

Điều quan trọng nhất cần nhớ: đây KHÔNG phải một LLM thật "hiểu" câu trả
lời theo nghĩa suy luận ngôn ngữ tự do — nó là một LUẬT TẤT ĐỊNH, luôn cho
CÙNG một nhãn với CÙNG một đầu vào. Track T8.4 dùng nó để mô phỏng đúng VAI
TRÒ giám khảo mà không cần mạng nơ-ron hay API thật.
::::

::::example{#do_giam_khao_that}
Sáu cặp (câu trả lời, đáp án chuẩn), chấm theo rubric ba nhãn:

```python title=readonly
import re


def trich_so(cau_tra_loi):
    cac_so = re.findall(r"\d+", cau_tra_loi)
    return int(cac_so[0]) if cac_so else None


def giam_khao_mo_phong(cau_tra_loi, dap_an_chuan):
    cau_thuong = cau_tra_loi.lower()
    if "khong biet" in cau_thuong:
        return "tu_choi"
    so = trich_so(cau_tra_loi)
    dung_so = (so == dap_an_chuan["gia"])
    dung_don_vi = (dap_an_chuan["don_vi"] in cau_thuong)
    if dung_so and dung_don_vi:
        return "dung"
    return "sai"


CAC_CAP_CHAM = [
    ("gia banh mi la 15000 dong", {"gia": 15000, "don_vi": "dong"}),
    ("gia banh mi la 15000", {"gia": 15000, "don_vi": "dong"}),
    ("gia banh mi khoang 20000 dong", {"gia": 15000, "don_vi": "dong"}),
    ("toi khong biet gia mon nay", {"gia": 15000, "don_vi": "dong"}),
    ("gia ca phe la 25000 dong", {"gia": 25000, "don_vi": "dong"}),
    ("khong biet duoc", {"gia": 30000, "don_vi": "dong"}),
]


def dem_phan_bo_nhan(cac_cap_cham):
    phan_bo = {"dung": 0, "sai": 0, "tu_choi": 0}
    for cau_tra_loi, dap_an_chuan in cac_cap_cham:
        nhan = giam_khao_mo_phong(cau_tra_loi, dap_an_chuan)
        phan_bo[nhan] += 1
    return phan_bo


phan_bo = dem_phan_bo_nhan(CAC_CAP_CHAM)
print(phan_bo)
```

```text title=readonly
{'dung': 2, 'sai': 2, 'tu_choi': 2}
```

Phân bố chia đều `2/2/2` trên `6` cặp: cặp `1` (đủ số, đủ đơn vị) và cặp
`5` (`ca_phe`, đủ cả hai) là `"dung"`. Cặp `2` (đúng số, THIẾU đơn vị
"dong") và cặp `3` (SAI số — `20000` thay vì `15000`) là `"sai"`. Cặp `4`
và cặp `6` chứa cụm "không biết" nên là `"tu_choi"` — bất kể đáp án chuẩn
là gì.
::::

::::predict{#doan_tu_choi_uu_tien commitOnce}
Rubric kiểm cụm "không biết" TRƯỚC TIÊN, trước cả việc kiểm số tiền và đơn
vị.

**Trước khi chạy thử**, bạn đoán: `giam_khao_mo_phong("khong biet, co le
15000 dong", {"gia": 15000, "don_vi": "dong"})` — một câu VỪA chứa "không
biết" VỪA TÌNH CỜ ghi đúng số tiền VÀ đúng đơn vị mong đợi — trả về nhãn
gì?

:::opt{correct}
`"tu_choi"` — nhánh `if "khong biet" in cau_thuong:` được kiểm TRƯỚC TIÊN
và `return` NGAY LẬP TỨC khi khớp, trước khi hàm kịp chạm tới bước kiểm số
tiền hay đơn vị; việc câu này TÌNH CỜ có đúng số `15000` và đúng đơn vị
"dong" không quan trọng — nhánh từ chối đã chặn đường trước đó
:::

:::opt
`"dung"` — vì câu này thực chất có đủ CẢ số tiền ĐÚNG lẫn đơn vị ĐÚNG, nên
rubric phải công nhận nó là câu trả lời đúng, bất kể có thêm cụm "không
biết" ở đầu
::why
Gần đúng ở việc bạn xác nhận đúng: câu này THẬT SỰ chứa số `15000` và đơn
vị "dong" khớp `dap_an_chuan`.

Chỗ lệch: rubric không "cộng điểm" cho việc có đủ số và đơn vị nếu câu đã
rơi vào nhánh từ chối trước đó. Thứ tự kiểm trong `giam_khao_mo_phong` là
CỐ ĐỊNH: từ chối được kiểm TRƯỚC, và một khi khớp thì hàm `return` ngay —
không có đường nào quay lại kiểm số tiền/đơn vị sau đó.
::
:::

:::opt
`"sai"` — vì câu này lẫn lộn cả một cụm từ chối lẫn một câu trả lời cụ
thể, nên rubric coi đây là một câu trả lời KHÔNG RÕ RÀNG, tính là sai
::why
Gần đúng ở việc bạn để ý câu này có phần "mâu thuẫn" (vừa từ chối vừa nêu
số) — quan sát đó hợp lý về mặt trực giác đọc hiểu.

Chỗ lệch: `giam_khao_mo_phong` không có khái niệm "mâu thuẫn" hay "không
rõ ràng" — nó chỉ có ĐÚNG BA nhánh, kiểm theo thứ tự cố định. Câu này khớp
nhánh ĐẦU TIÊN (`"khong biet" in cau_thuong`), nên trả về `"tu_choi"` chứ
không rơi vào nhánh `"sai"` (nhánh đó chỉ chạm tới khi KHÔNG khớp nhánh từ
chối trước).
::
:::
::::

::::code{#viet_giam_khao_mo_phong}
Hoàn thiện `giam_khao_mo_phong`: điều kiện "đúng" phải yêu cầu ĐỦ CẢ HAI
(số tiền đúng VÀ đơn vị đúng), và nhánh còn lại phải trả về đúng nhãn
`"sai"`.

```python title=starter
import re


def trich_so(cau_tra_loi):
    cac_so = re.findall(r"\d+", cau_tra_loi)
    return int(cac_so[0]) if cac_so else None


def giam_khao_mo_phong(cau_tra_loi, dap_an_chuan):
    cau_thuong = cau_tra_loi.lower()
    if "khong biet" in cau_thuong:
        return "tu_choi"
    so = trich_so(cau_tra_loi)
    dung_so = (so == dap_an_chuan["gia"])
    dung_don_vi = (dap_an_chuan["don_vi"] in cau_thuong)
    if dung_so ___ dung_don_vi:                       # and
        return "dung"
    return ___                                         # "sai"


CAC_CAP_CHAM = [
    ("gia banh mi la 15000 dong", {"gia": 15000, "don_vi": "dong"}),
    ("gia banh mi la 15000", {"gia": 15000, "don_vi": "dong"}),
    ("gia banh mi khoang 20000 dong", {"gia": 15000, "don_vi": "dong"}),
    ("toi khong biet gia mon nay", {"gia": 15000, "don_vi": "dong"}),
    ("gia ca phe la 25000 dong", {"gia": 25000, "don_vi": "dong"}),
    ("khong biet duoc", {"gia": 30000, "don_vi": "dong"}),
]


def dem_phan_bo_nhan(cac_cap_cham):
    phan_bo = {"dung": 0, "sai": 0, "tu_choi": 0}
    for cau_tra_loi, dap_an_chuan in cac_cap_cham:
        nhan = giam_khao_mo_phong(cau_tra_loi, dap_an_chuan)
        phan_bo[nhan] += 1
    return phan_bo


phan_bo = dem_phan_bo_nhan(CAC_CAP_CHAM)
print(phan_bo)
```

```python title=solution
import re


def trich_so(cau_tra_loi):
    cac_so = re.findall(r"\d+", cau_tra_loi)
    return int(cac_so[0]) if cac_so else None


def giam_khao_mo_phong(cau_tra_loi, dap_an_chuan):
    cau_thuong = cau_tra_loi.lower()
    if "khong biet" in cau_thuong:
        return "tu_choi"
    so = trich_so(cau_tra_loi)
    dung_so = (so == dap_an_chuan["gia"])
    dung_don_vi = (dap_an_chuan["don_vi"] in cau_thuong)
    if dung_so and dung_don_vi:
        return "dung"
    return "sai"


CAC_CAP_CHAM = [
    ("gia banh mi la 15000 dong", {"gia": 15000, "don_vi": "dong"}),
    ("gia banh mi la 15000", {"gia": 15000, "don_vi": "dong"}),
    ("gia banh mi khoang 20000 dong", {"gia": 15000, "don_vi": "dong"}),
    ("toi khong biet gia mon nay", {"gia": 15000, "don_vi": "dong"}),
    ("gia ca phe la 25000 dong", {"gia": 25000, "don_vi": "dong"}),
    ("khong biet duoc", {"gia": 30000, "don_vi": "dong"}),
]


def dem_phan_bo_nhan(cac_cap_cham):
    phan_bo = {"dung": 0, "sai": 0, "tu_choi": 0}
    for cau_tra_loi, dap_an_chuan in cac_cap_cham:
        nhan = giam_khao_mo_phong(cau_tra_loi, dap_an_chuan)
        phan_bo[nhan] += 1
    return phan_bo


phan_bo = dem_phan_bo_nhan(CAC_CAP_CHAM)
print(phan_bo)
```

```python title=test
assert phan_bo == {"dung": 2, "sai": 2, "tu_choi": 2}, f"phan bo nhan tren 6 cap phai la {{'dung': 2, 'sai': 2, 'tu_choi': 2}} -- dang ra {phan_bo}"

assert giam_khao_mo_phong("gia banh mi la 15000 dong", {"gia": 15000, "don_vi": "dong"}) == "dung", "du so va don vi phai la 'dung'"
assert giam_khao_mo_phong("gia banh mi la 15000", {"gia": 15000, "don_vi": "dong"}) == "sai", "dung so nhung THIEU don vi phai la 'sai'"
assert giam_khao_mo_phong("gia banh mi khoang 20000 dong", {"gia": 15000, "don_vi": "dong"}) == "sai", "SAI so (du co don vi) phai la 'sai'"
assert giam_khao_mo_phong("toi khong biet gia mon nay", {"gia": 15000, "don_vi": "dong"}) == "tu_choi", "cum 'khong biet' phai la 'tu_choi'"

# bang chung TRUNG TAM: uu tien tu choi TRUOC, du cau TINH CO co dung ca so
# LAN don vi mong doi
assert giam_khao_mo_phong("khong biet, co le 15000 dong", {"gia": 15000, "don_vi": "dong"}) == "tu_choi", "cau vua tu choi vua tinh co dung so/don vi VAN phai la 'tu_choi', khong phai 'dung'"

# bien: danh sach RONG phai tra ve phan bo toan 0, khong loi
assert dem_phan_bo_nhan([]) == {"dung": 0, "sai": 0, "tu_choi": 0}, f"danh sach rong phai cho phan bo toan 0 -- dang ra {dem_phan_bo_nhan([])}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu ghép hai điều kiện `dung_so` và `dung_don_vi` — nhãn `"dung"` chỉ được gán khi CẢ HAI đều đúng, dùng toán tử kết hợp CẢ HAI cùng lúc (`and`). Chỗ hai là nhãn trả về khi KHÔNG đủ điều kiện `"dung"` — dùng đúng chuỗi đã thấy trong tên `dem_phan_bo_nhan` (`"sai"`).
- kind: strategy
  body: 'Chỗ đầu: `and` (cho dòng `if dung_so and dung_don_vi:`). Chỗ hai: `"sai"` (cho dòng `return "sai"`).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `and` và `"sai"`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: dieu kien "dung" phai dung "and" de doi hoi CA HAI (dung_so VA dung_don_vi, khong duoc doi thanh "or" hay chi kiem mot dieu kien), VA nhanh con lai phai tra ve DUNG chuoi "sai" (khong duoc doi thanh mot nhan khac)
  requireAst:
  - kind: uses-operator, target: "and", min: 1
  - kind: has-literal, target: "sai", min: 2
  # Da thu that (goi kiemAst that -- trich nguyen ham _dem tu kiem-ast.ts,
  # chay qua python3 tren code trich tu solution, khong doan tay).
  # "and"=1: XUAT HIEN DUY NHAT o cho trong 1 (if dung_so and dung_don_vi:)
  # -- khong co "and" nao khac trong toan bo solution. Dien bua doi thanh
  # "or" lam so nay tut ve 0 (BoolOp doi tu And sang Or, khong con khop
  # target "and") -- duoi nguong min=1, bi chan; dong thoi bi chan boi
  # tests (voi "or", cap 2 -- dung so nhung thieu don vi -- se bi cham SAI
  # thanh "dung", lam phan_bo lech khoi {'dung':2,'sai':2,'tu_choi':2}).
  # has-literal "sai"=2: mot lan CO SAN (khoa "sai" trong dict khoi tao cua
  # dem_phan_bo_nhan, "phan_bo = {'dung': 0, 'sai': 0, 'tu_choi': 0}"), mot
  # lan la cho trong 2 (return "sai"). Neu chi dat min=1 (ngay tho), mot
  # mutant doi cho trong 2 thanh mot nhan khac (vi du "khong_ro") VAN qua
  # duoc vi con lai 1 lan "sai" trong dict khoi tao -- GOTCHA
  # "boilerplate-threshold-masking"; dat dung min=2 (tong THAT) moi chan
  # duoc mutant nay -- va no cung bi chan boi tests (phan_bo se co khoa
  # "khong_ro" thay vi "sai", so KeyError hoac gia tri lech).
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^\\{'dung': 2, 'sai': 2, 'tu_choi': 2\\}\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Phân bố chia đều `2/2/2` — một luật tất định, không mạng nơ-ron, vẫn phân
biệt rõ ba trường hợp: đúng hẳn, sai (thiếu số hoặc đơn vị), và từ chối.
Bài sau chuyển sang một dạng nguy hiểm khác: dữ liệu TRẢ VỀ từ một công cụ
có thể chứa một CHỈ THỊ GIẢ trà trộn vào — công cụ bị NHIỄM.
::::

::::reflect{#nghi-lai}
`giam_khao_mo_phong` không "đọc hiểu" gì cả — nó là một cây quyết định BA
NHÁNH, kiểm theo thứ tự CỐ ĐỊNH, luôn cho cùng nhãn với cùng đầu vào.
Nhưng chính vì tất định và có thứ tự ưu tiên rõ ràng, nó bắt được đúng một
trường hợp mà độ tương đồng từ (bài trước) không phân biệt nổi: câu
`"khong biet, co le 15000 dong"` có thể có độ tương đồng từ khá cao với
một câu trả lời đúng thật (chia sẻ cả số tiền lẫn đơn vị), nhưng RUBRIC
đúng phải xếp nó vào `"tu_choi"`, không phải `"dung"` — vì một câu từ chối
lẫn thêm một con số tình cờ đúng vẫn là MỘT CÂU TỪ CHỐI, không phải một
câu trả lời đáng tin. Đây là giá trị của một giám khảo có LUẬT rõ ràng
thay vì chỉ đếm chữ chung: nó áp được một THỨ TỰ ƯU TIÊN mà một độ đo
tương đồng đơn thuần không thể diễn đạt. Bài sau chuyển sang một mối nguy
khác hẳn: dữ liệu một CÔNG CỤ trả về (không phải câu hỏi của người dùng)
có thể bị trà trộn một CHỈ THỊ GIẢ, cố lừa hệ thống tuân theo nó.
::::

::::checkpoint{mastery=0.85}
::::
