---
id: tri-tue-nhan-tao.prompt-engineering-nen-tang.do-thuc-nghiem-anh-huong-prompt
title: "Đo thực nghiệm A/B: ảnh hưởng của prompt, đo bằng số"
summary: "Thuc nghiem A/B that: mot bo 5 cau hoi CO DINH (so luong dai luong 2,3,2,4,3), chay qua LLM mo phong VOI mot phien ban prompt THEM cum 'Hay suy nghi tung buoc.' vao cuoi cau hoi (kich hoat cong TOAN BO cac so, giong hieu ung few-shot o bai 2 nhung kich hoat bang MOT CUM TU, khong can vi du mau). Phien ban KHONG them cum tu: dung 2/5 cau (chi dung khi cau hoi co <=2 dai luong). Phien ban CO them cum tu: dung 5/5 -- cai thien +3 cau dung tren cung bo kiem thu, do that bang Python, khong suy doan."
locale: vi
track: tri-tue-nhan-tao
module: prompt-engineering-nen-tang
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.do-thuc-nghiem-anh-huong-prompt]
requires: [ai.prompt-template]
concepts: [ai.do-thuc-nghiem-anh-huong-prompt]
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

::::byte{trigger=enter mood=curious pose=point-editor}
Từng bài trước đo MỘT câu hỏi, MỘT thay đổi. Bài này chạy một THỰC NGHIỆM
thật: một bộ câu hỏi cố định, hai phiên bản prompt, đếm bao nhiêu câu
đúng ở mỗi phiên bản — và đo chênh lệch bằng một con số duy nhất.
::::

::::explain{#do_ab_va_cot}
Một thực nghiệm A/B về prompt không so sánh hai câu trả lời ĐƠN LẺ — nó
so sánh hai PHIÊN BẢN của cùng một prompt trên CÙNG một BỘ câu hỏi, rồi
đếm xem phiên bản nào cho nhiều câu trả lời ĐÚNG hơn. Một, hai câu hỏi
riêng lẻ có thể đúng do trùng hợp; đo trên cả một bộ mới cho một con số
đáng tin.

Bài này giới thiệu một kỹ thuật MỚI để cải thiện độ chính xác, không cần
few-shot: **prompt suy luận từng bước** (chain-of-thought — thường viết
tắt CoT). Ý tưởng: thêm một cụm chỉ thị như `"Hãy suy nghĩ từng bước."`
vào cuối câu hỏi, yêu cầu mô hình xử lý TỪNG PHẦN của bài toán thay vì
"đi tắt" ra ngay một câu trả lời.

LLM mô phỏng ở bài này tái hiện đúng hiệu ứng đó: hàm `llm_dem_tong_v2`
giữ nguyên luật đi tắt (chỉ cộng hai số đầu) của bài `zero-shot-va-
few-shot`, nhưng giờ có HAI cách kích hoạt luật cộng-toàn-bộ, độc lập với
nhau:

> **Đủ `2` ví dụ mẫu trở lên** (`so_vi_du >= 2`, giống bài `2`).
>
> **Câu hỏi cuối cùng chứa cụm `"hay suy nghi tung buoc"`** (không phân
> biệt hoa/thường) — MỘT chỉ thị, không cần ví dụ mẫu nào.

Chỉ cần MỘT trong hai điều kiện đúng là đủ để chuyển sang cộng toàn bộ.
Bài này đo hiệu ứng của NHÁNH THỨ HAI một mình: một bộ `5` câu hỏi có số
lượng đại lượng khác nhau (`2`, `3`, `2`, `4`, `3`), chạy KHÔNG kèm cụm từ
CoT rồi CÓ kèm, đếm bao nhiêu câu cho kết quả đúng ở mỗi phiên bản.
::::

::::example{#thuc_nghiem_that}
Bộ `5` câu hỏi cố định, mỗi câu chạy hai lần — không và có cụm từ CoT:

```python title=readonly
import re


def trich_so(text):
    return [int(x) for x in re.findall(r"\d+", text)]


def lay_cau_hoi_cuoi(messages):
    cau_hoi = None
    for tin in messages:
        if tin["role"] == "user":
            cau_hoi = tin["content"]
    return cau_hoi


def dem_vi_du_fewshot(messages):
    return sum(1 for tin in messages if tin["role"] == "assistant")


def co_yeu_cau_cot(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    return "hay suy nghi tung buoc" in cau_hoi.lower()


def llm_dem_tong_v2(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    so_list = trich_so(cau_hoi)
    so_vi_du = dem_vi_du_fewshot(messages)
    dung_day_du = co_yeu_cau_cot(messages) or so_vi_du >= 2
    if dung_day_du:
        tong = sum(so_list)
    else:
        tong = sum(so_list[:2])
    return tong


BO_CAU_HOI_KIEM_THU = [
    "An co 2 qua tao va 3 qua cam. Hoi An co tat ca bao nhieu qua?",
    "Binh co 1 qua tao, 2 qua le va 4 qua cam. Hoi Binh co tat ca bao nhieu qua?",
    "Chi co 5 qua cam va 1 qua tao. Hoi Chi co tat ca bao nhieu qua?",
    "Dung co 2 qua tao, 3 qua cam, 1 qua le va 4 qua buoi. Hoi Dung co tat ca bao nhieu qua?",
    "Em co 3 qua tao, 2 qua cam va 5 qua le. Hoi Em co tat ca bao nhieu qua?",
]

HAU_TO_COT = " Hay suy nghi tung buoc."


def chay_thuc_nghiem(bo_cau_hoi, dung_cot):
    so_dung = 0
    for cau_hoi in bo_cau_hoi:
        noi_dung = cau_hoi + HAU_TO_COT if dung_cot else cau_hoi
        messages = [{"role": "user", "content": noi_dung}]
        dap_an = llm_dem_tong_v2(messages)
        tong_dung = sum(trich_so(cau_hoi))
        if dap_an == tong_dung:
            so_dung += 1
    return so_dung


so_dung_khong_cot = chay_thuc_nghiem(BO_CAU_HOI_KIEM_THU, dung_cot=False)
so_dung_co_cot = chay_thuc_nghiem(BO_CAU_HOI_KIEM_THU, dung_cot=True)

print(so_dung_khong_cot, "/", len(BO_CAU_HOI_KIEM_THU))
print(so_dung_co_cot, "/", len(BO_CAU_HOI_KIEM_THU))
print(so_dung_co_cot - so_dung_khong_cot)
```

```text title=readonly
2 / 5
5 / 5
3
```

Không có cụm từ CoT: đúng `2/5` — chỉ hai câu hỏi có ĐÚNG `2` đại lượng
(`"An..."`, `"Chi..."`) tình cờ đúng, vì luật đi tắt (cộng hai số đầu)
trùng với tổng thật khi chỉ có hai đại lượng. Ba câu hỏi còn lại (`3` hoặc
`4` đại lượng) đều SAI vì bị bỏ sót số. Thêm cụm từ CoT vào MỌI câu hỏi:
`5/5` — ĐÚNG hoàn toàn, vì `co_yeu_cau_cot` giờ luôn `True`, buộc nhánh
cộng-toàn-bộ chạy bất kể số lượng đại lượng. Cải thiện đo được: `+3` câu
đúng trên cùng bộ `5` câu, không suy luận, chạy Python thật.
::::

::::predict{#doan_ket_hop_cot_fewshot commitOnce}
`dung_day_du = co_yeu_cau_cot(messages) or so_vi_du >= 2` dùng toán tử
`or` — chỉ cần MỘT trong hai vế đúng là đủ.

**Trước khi chạy thử**, bạn đoán: nếu một message vừa có ĐỦ `2` ví dụ mẫu
(few-shot) VÀ câu hỏi cuối vừa chứa cụm từ CoT, kết quả `dung_day_du` có
khác so với việc chỉ có MỘT trong hai điều kiện đó không?

:::opt{correct}
Không khác — `dung_day_du` vẫn là `True` trong cả ba trường hợp (chỉ
few-shot, chỉ CoT, hoặc cả hai); toán tử `or` chỉ cần MỘT vế đúng, có cả
hai vế đúng không làm `True` "đúng hơn" — Python không có khái niệm đó
:::

:::opt
Có — khi cả hai điều kiện cùng đúng, `dung_day_du` sẽ mạnh hơn, khiến kết
quả chính xác hơn nữa
::why
Gần đúng ở trực giác "càng nhiều bằng chứng ủng hộ thì càng chắc chắn" —
một trực giác hợp lý trong nhiều ngữ cảnh thống kê.

Chỗ lệch: `dung_day_du` là một giá trị `bool` — chỉ có `True` hoặc `False`,
không có mức độ "mạnh hơn". Biểu thức `or` trong Python trả về `True` ngay
khi gặp vế đầu tiên đúng (đoản mạch — short-circuit), và dù cả hai vế đều
đúng, giá trị cuối cùng vẫn chỉ là `True` — không khác gì trường hợp chỉ
một vế đúng.
::
:::

:::opt
Không xác định được — phụ thuộc vế nào được Python đánh giá trước
::why
Gần đúng ở việc bạn để ý Python CÓ đánh giá các vế của `or` theo thứ tự
(trái trước, rồi mới tới phải nếu cần) — quan sát đó đúng với cách `or`
hoạt động.

Chỗ lệch: thứ tự đánh giá chỉ ảnh hưởng hàm nào được GỌI trước (ở đây
không hàm nào có tác dụng phụ), không ảnh hưởng KẾT QUẢ cuối cùng của
biểu thức `or`. Với hai vế đều `True`, biểu thức luôn cho `True`, bất kể
thứ tự đánh giá.
::
:::
::::

::::code{#viet_chay_thuc_nghiem}
Hoàn thiện `chay_thuc_nghiem`: so sánh câu trả lời với tổng đúng, và cộng
dồn số câu đúng.

```python title=starter
import re


def trich_so(text):
    return [int(x) for x in re.findall(r"\d+", text)]


def lay_cau_hoi_cuoi(messages):
    cau_hoi = None
    for tin in messages:
        if tin["role"] == "user":
            cau_hoi = tin["content"]
    return cau_hoi


def dem_vi_du_fewshot(messages):
    return sum(1 for tin in messages if tin["role"] == "assistant")


def co_yeu_cau_cot(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    return "hay suy nghi tung buoc" in cau_hoi.lower()


def llm_dem_tong_v2(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    so_list = trich_so(cau_hoi)
    so_vi_du = dem_vi_du_fewshot(messages)
    dung_day_du = co_yeu_cau_cot(messages) or so_vi_du >= 2
    if dung_day_du:
        tong = sum(so_list)
    else:
        tong = sum(so_list[:2])
    return tong


BO_CAU_HOI_KIEM_THU = [
    "An co 2 qua tao va 3 qua cam. Hoi An co tat ca bao nhieu qua?",
    "Binh co 1 qua tao, 2 qua le va 4 qua cam. Hoi Binh co tat ca bao nhieu qua?",
    "Chi co 5 qua cam va 1 qua tao. Hoi Chi co tat ca bao nhieu qua?",
    "Dung co 2 qua tao, 3 qua cam, 1 qua le va 4 qua buoi. Hoi Dung co tat ca bao nhieu qua?",
    "Em co 3 qua tao, 2 qua cam va 5 qua le. Hoi Em co tat ca bao nhieu qua?",
]

HAU_TO_COT = " Hay suy nghi tung buoc."


def chay_thuc_nghiem(bo_cau_hoi, dung_cot):
    so_dung = 0
    for cau_hoi in bo_cau_hoi:
        noi_dung = cau_hoi + HAU_TO_COT if dung_cot else cau_hoi
        messages = [{"role": "user", "content": noi_dung}]
        dap_an = llm_dem_tong_v2(messages)
        tong_dung = sum(trich_so(cau_hoi))
        if dap_an ___ tong_dung:              # ==
            so_dung ___ 1                       # += 
    return so_dung


so_dung_khong_cot = chay_thuc_nghiem(BO_CAU_HOI_KIEM_THU, dung_cot=False)
so_dung_co_cot = chay_thuc_nghiem(BO_CAU_HOI_KIEM_THU, dung_cot=True)

print(so_dung_khong_cot, "/", len(BO_CAU_HOI_KIEM_THU))
print(so_dung_co_cot, "/", len(BO_CAU_HOI_KIEM_THU))
print(so_dung_co_cot - so_dung_khong_cot)
```

```python title=solution
import re


def trich_so(text):
    return [int(x) for x in re.findall(r"\d+", text)]


def lay_cau_hoi_cuoi(messages):
    cau_hoi = None
    for tin in messages:
        if tin["role"] == "user":
            cau_hoi = tin["content"]
    return cau_hoi


def dem_vi_du_fewshot(messages):
    return sum(1 for tin in messages if tin["role"] == "assistant")


def co_yeu_cau_cot(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    return "hay suy nghi tung buoc" in cau_hoi.lower()


def llm_dem_tong_v2(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    so_list = trich_so(cau_hoi)
    so_vi_du = dem_vi_du_fewshot(messages)
    dung_day_du = co_yeu_cau_cot(messages) or so_vi_du >= 2
    if dung_day_du:
        tong = sum(so_list)
    else:
        tong = sum(so_list[:2])
    return tong


BO_CAU_HOI_KIEM_THU = [
    "An co 2 qua tao va 3 qua cam. Hoi An co tat ca bao nhieu qua?",
    "Binh co 1 qua tao, 2 qua le va 4 qua cam. Hoi Binh co tat ca bao nhieu qua?",
    "Chi co 5 qua cam va 1 qua tao. Hoi Chi co tat ca bao nhieu qua?",
    "Dung co 2 qua tao, 3 qua cam, 1 qua le va 4 qua buoi. Hoi Dung co tat ca bao nhieu qua?",
    "Em co 3 qua tao, 2 qua cam va 5 qua le. Hoi Em co tat ca bao nhieu qua?",
]

HAU_TO_COT = " Hay suy nghi tung buoc."


def chay_thuc_nghiem(bo_cau_hoi, dung_cot):
    so_dung = 0
    for cau_hoi in bo_cau_hoi:
        noi_dung = cau_hoi + HAU_TO_COT if dung_cot else cau_hoi
        messages = [{"role": "user", "content": noi_dung}]
        dap_an = llm_dem_tong_v2(messages)
        tong_dung = sum(trich_so(cau_hoi))
        if dap_an == tong_dung:
            so_dung += 1
    return so_dung


so_dung_khong_cot = chay_thuc_nghiem(BO_CAU_HOI_KIEM_THU, dung_cot=False)
so_dung_co_cot = chay_thuc_nghiem(BO_CAU_HOI_KIEM_THU, dung_cot=True)

print(so_dung_khong_cot, "/", len(BO_CAU_HOI_KIEM_THU))
print(so_dung_co_cot, "/", len(BO_CAU_HOI_KIEM_THU))
print(so_dung_co_cot - so_dung_khong_cot)
```

```python title=test
assert so_dung_khong_cot == 2, f"khong CoT phai dung dung 2/5 -- dang ra {so_dung_khong_cot}"
assert so_dung_co_cot == 5, f"co CoT phai dung ca 5/5 -- dang ra {so_dung_co_cot}"
assert so_dung_co_cot - so_dung_khong_cot == 3, f"cai thien phai la +3 -- dang ra {so_dung_co_cot - so_dung_khong_cot}"

# bien: bo kiem thu RONG -- vong lap khong chay lan nao, phai tra ve 0
# (khong duoc loi vi chia cho do dai hay truy cap phan tu khong ton tai)
assert chay_thuc_nghiem([], dung_cot=True) == 0, "bo kiem thu rong phai tra ve 0"

# bien: mot cau hoi CHI co 1 dai luong -- khong CoT van phai dung (luat di
# tat khong lam sai cau don gian), co CoT cung phai dung (khong lam hong
# cau da dung san)
mot_cau = ["Kim co 6 qua tao. Hoi Kim co tat ca bao nhieu qua?"]
assert chay_thuc_nghiem(mot_cau, dung_cot=False) == 1, "cau 1 dai luong khong CoT phai dung"
assert chay_thuc_nghiem(mot_cau, dung_cot=True) == 1, "cau 1 dai luong co CoT van phai dung"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cả hai nằm trong thân vòng lặp `for cau_hoi in bo_cau_hoi:`. Chỗ đầu so sánh `dap_an` với `tong_dung` — dùng toán tử so sánh BẰNG NHAU. Chỗ hai cộng dồn bộ đếm `so_dung` thêm `1` khi so sánh đó đúng — dùng phép gán cộng dồn (`+=`), không phải gán lại từ đầu.
- kind: strategy
  body: 'Chỗ đầu: `==` (cho dòng `if dap_an == tong_dung:`). Chỗ hai: `+=` (cho dòng `so_dung += 1`).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `==` và `+=`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: so sanh dap_an voi tong_dung phai dung toan tu "==" (khong duoc doi thanh mot toan tu khac), VA bo dem so_dung phai duoc CONG DON bang "+=" (khong duoc gan lai mot hang so co dinh)
  requireAst:
  - kind: uses-operator, target: "==", min: 3
  - kind: uses-operator, target: "+", min: 2
  # Da thu that (goi _dem tren code trich tu solution, khong doan tay).
  # "=="=3: hai lan CO SAN (tin["role"]=="user" trong lay_cau_hoi_cuoi,
  # tin["role"]=="assistant" trong dem_vi_du_fewshot), mot lan la cho
  # trong 1 (if dap_an == tong_dung). Dien bua thanh "is" hay bo so sanh
  # lam "=="=2 -- duoi nguong min=3, bi chan; dong thoi bi chan boi
  # run/tests (bieu thuc sai kieu hoac logic sai).
  # "+"=2: mot lan CO SAN (bieu thuc "cau_hoi + HAU_TO_COT" -- BinOp Add
  # noi chuoi), mot lan la cho trong 2 ("so_dung += 1" la AugAssign voi
  # toan tu Add, CUNG khop target "+"). Neu chi dat min=1 (ngay tho) thi
  # mot mutant xoa bo cho trong 2 (vi du thay bang "so_dung = 1", gan lai
  # hang so co dinh, mat tinh cong don) VAN qua duoc vi con lai 1 lan "+"
  # o dong noi chuoi -- day la GOTCHA "boilerplate-threshold-masking";
  # dat dung min=2 (tong THAT) moi chan duoc mutant nay -- va no cung bi
  # chan boi tests vi so_dung_khong_cot/so_dung_co_cot se sai (luon ket
  # thuc la 1 thay vi dem dung so cau).
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^2 / 5\\n5 / 5\\n3\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`2/5` không CoT, `5/5` có CoT — cải thiện `+3` câu đúng, đo trên cùng một
bộ câu hỏi, không suy luận. Bài sau (BOSS quý) ráp TẤT CẢ: vai trò message
+ ràng buộc rõ ràng + few-shot + CoT, thành MỘT prompt hoàn chỉnh, rồi đo
cải thiện so với một prompt trần trụi — không có kỹ thuật nào.
::::

::::reflect{#nghi-lai}
Đo một thực nghiệm A/B trên MỘT câu hỏi duy nhất dễ nhầm trùng hợp với
hiệu quả thật; đo trên một BỘ câu hỏi cố định (ở đây `5` câu, số lượng
đại lượng khác nhau) mới tách được điều đó — `2/5` không CoT rồi `5/5` có
CoT là một con số không thể "may mắn" theo cùng một hướng ở cả năm câu
hỏi khác nhau. Bài sau ráp cụm chỉ thị CoT NÀY cùng với vai trò message
(bài `1`), ràng buộc rõ ràng (bài `3`), few-shot (bài `2`) và khuôn mẫu
(bài `4`) thành một prompt DUY NHẤT — rồi lặp lại đúng phép đo A/B này để
xem tổ hợp cả bốn kỹ thuật cải thiện được bao nhiêu so với một prompt
trần trụi.
::::

::::checkpoint{mastery=0.85}
::::
