---
id: tri-tue-nhan-tao.prompt-engineering-nen-tang.zero-shot-va-few-shot
title: "Zero-shot và few-shot: ví dụ mẫu đổi hành vi ra sao"
summary: "LLM mo phong tinh tong so luong nhac trong mot cau hoi tieng Viet ngan. ZERO-SHOT (khong vi du mau, chi cau hoi) dung mot LUAT DI TAT: chi cong HAI so dau tien tim duoc -- tren cau hoi 3 so (3, 5, 2) cho ket qua 8, SAI so voi tong dung 10. FEW-SHOT voi dung 2 cap vi du user/assistant dat TRUOC cau hoi doi hanh vi sang cong TOAN BO cac so, cho dung ket qua 10. Voi CHI 1 vi du, hanh vi VAN chua doi (van ra 8) -- nguong la >=2, khong phai >=1. Ba con so (8, 10, 8) deu chay that bang Python, khong suy doan."
locale: vi
track: tri-tue-nhan-tao
module: prompt-engineering-nen-tang
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.zero-shot-va-few-shot]
requires: [ai.vai-tro-message]
concepts: [ai.zero-shot-va-few-shot]
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
Bài trước: `role` sai chỗ làm mất hiệu lực một chỉ thị. Bài này giữ
nguyên cấu trúc message, nhưng thêm các cặp `user`/`assistant` mẫu TRƯỚC
câu hỏi thật — và đo xem bao nhiêu ví dụ mẫu mới đủ đổi cách LLM mô phỏng
tính toán.
::::

::::explain{#zero_shot_few_shot_la_gi}
**Zero-shot** — gọi mô hình với ĐÚNG câu hỏi, không kèm ví dụ mẫu nào.
**Few-shot** — TRƯỚC câu hỏi thật, chèn thêm một vài CẶP message
`user`/`assistant` đóng vai trò ví dụ mẫu: mỗi cặp cho thấy "câu hỏi dạng
này thì trả lời dạng kia". Với một LLM thật, những ví dụ mẫu này không
làm mô hình "học" theo nghĩa cập nhật trọng số — chúng chỉ nằm trong
ngữ cảnh của lượt gọi, và mô hình suy luận dựa theo MẪU đã thấy.

LLM mô phỏng của bài này tái hiện đúng hiệu ứng đó bằng một luật cố định,
đơn giản đến mức đo được từng bước: hàm `llm_dem_tong` nhận một câu hỏi
kiểu "X có A quả táo, B quả cam, ... Hỏi X có tất cả bao nhiêu quả?", tách
ra các con số trong câu, rồi cộng chúng. Nhưng CÁCH cộng phụ thuộc vào số
lượng ví dụ mẫu đứng trước:

> **Không có ví dụ mẫu (zero-shot)** — dùng một luật ĐI TẮT: chỉ cộng HAI
> số ĐẦU TIÊN tìm thấy trong câu, bỏ qua phần còn lại. Với câu hỏi có
> đúng hai đại lượng, luật này vẫn cho kết quả đúng — nhưng với câu hỏi có
> BA đại lượng trở lên, nó cho kết quả THIẾU.
>
> **Có ít nhất `2` ví dụ mẫu (few-shot)** — đổi sang cộng TOÀN BỘ các số
> tìm thấy, không bỏ sót đại lượng nào.

Đây là một mô phỏng CÓ CHỦ ĐÍCH của một hiện tượng có thật ở LLM: khi câu
hỏi phức tạp hơn (nhiều bước/nhiều đại lượng hơn), một mô hình không có
gợi ý về ĐỊNH DẠNG suy luận mong muốn dễ "đi tắt", còn vài ví dụ mẫu đúng
định dạng thường sửa được việc đó. LLM mô phỏng ở đây dùng một CON SỐ
NGƯỠNG cụ thể (`>= 2` ví dụ) để hiện tượng này đo được chính xác, không mơ
hồ.
::::

::::example{#do_zero_shot_vs_few_shot}
Cùng MỘT câu hỏi có `3` đại lượng (`3` quả táo, `5` quả cam, `2` quả lê —
tổng đúng là `10`), gọi qua ba cách khác nhau:

```python title=readonly
import re


def trich_so(text):
    return [int(x) for x in re.findall(r"\d+", text)]


def dem_vi_du_fewshot(messages):
    return sum(1 for tin in messages if tin["role"] == "assistant")


def lay_cau_hoi_cuoi(messages):
    cau_hoi = None
    for tin in messages:
        if tin["role"] == "user":
            cau_hoi = tin["content"]
    return cau_hoi


def llm_dem_tong(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    so_list = trich_so(cau_hoi)
    so_vi_du = dem_vi_du_fewshot(messages)
    if so_vi_du >= 2:
        tong = sum(so_list)
    else:
        tong = sum(so_list[:2])
    return str(tong)


cau_hoi_test = "Lan co 3 qua tao, 5 qua cam va 2 qua le. Hoi Lan co tat ca bao nhieu qua?"

messages_zero_shot = [
    {"role": "user", "content": cau_hoi_test},
]

messages_mot_vi_du = [
    {"role": "user", "content": "Nam co 1 qua tao va 2 qua cam. Hoi Nam co tat ca bao nhieu qua?"},
    {"role": "assistant", "content": "3"},
    {"role": "user", "content": cau_hoi_test},
]

messages_few_shot = [
    {"role": "user", "content": "Nam co 1 qua tao va 2 qua cam. Hoi Nam co tat ca bao nhieu qua?"},
    {"role": "assistant", "content": "3"},
    {"role": "user", "content": "Hoa co 4 qua tao, 1 qua cam va 3 qua le. Hoi Hoa co tat ca bao nhieu qua?"},
    {"role": "assistant", "content": "8"},
    {"role": "user", "content": cau_hoi_test},
]

tong_dung = sum(trich_so(cau_hoi_test))

print(tong_dung)
print(llm_dem_tong(messages_zero_shot))
print(llm_dem_tong(messages_mot_vi_du))
print(llm_dem_tong(messages_few_shot))
```

```text title=readonly
10
8
8
10
```

Tổng ĐÚNG của câu hỏi là `10` (`3 + 5 + 2`). Zero-shot (không ví dụ mẫu)
trả `8` — SAI, vì luật đi tắt chỉ cộng `3 + 5`, bỏ sót số `2`. Với ĐÚNG
`1` ví dụ mẫu, kết quả VẪN là `8` — chưa đổi hành vi, vì ngưỡng là `>= 2`,
không phải `>= 1`. Chỉ khi có ĐỦ `2` ví dụ mẫu trở lên, hàm mới chuyển
sang cộng toàn bộ và cho ra `10` — kết quả ĐÚNG.
::::

::::predict{#doan_nguong_fewshot commitOnce}
Giữ nguyên `messages_few_shot` (đã có `2` ví dụ mẫu, cho kết quả đúng
`10`). Giả sử chèn thêm một cặp `user`/`assistant` THỨ BA (một ví dụ mẫu
nữa) vào TRƯỚC câu hỏi cuối, làm `so_vi_du` tăng từ `2` lên `3`.

**Trước khi chạy thử**, bạn đoán: kết quả của `llm_dem_tong` trên danh
sách message MỚI (`3` ví dụ mẫu) có khác `10` không?

:::opt{correct}
Không đổi — vẫn là `10`. Điều kiện trong code là `so_vi_du >= 2`, một
NGƯỠNG chứ không phải một phép đếm tỉ lệ; hễ đạt hoặc vượt `2`, nhánh cộng
toàn bộ đều được chọn — `2` ví dụ hay `3` ví dụ đều rơi vào CÙNG một nhánh
:::

:::opt
Có, kết quả sẽ càng đúng hơn (chính xác hơn `10`) vì có thêm một ví dụ mẫu
nữa
::why
Gần đúng ở trực giác chung "thêm ví dụ mẫu thường cải thiện chất lượng" —
trực giác đó có cơ sở với LLM thật trong nhiều tình huống.

Chỗ lệch: hàm mô phỏng ở đây không có khái niệm "đúng hơn `10`" — một khi
đã rơi vào nhánh `sum(so_list)`, kết quả LUÔN là tổng CHÍNH XÁC của mọi số
tìm thấy trong câu hỏi, không có mức độ "đúng hơn nữa". Thêm ví dụ mẫu thứ
`3` không đổi con số nào cả, vì điều kiện `>= 2` đã thoả từ trước.
::
:::

:::opt
Không xác định được — hành vi few-shot của LLM phụ thuộc nội dung CỤ THỂ
của từng ví dụ mẫu, không chỉ số LƯỢNG
::why
Gần đúng với LLM THẬT — nội dung của từng ví dụ mẫu (không chỉ số lượng)
thường ảnh hưởng tới câu trả lời.

Chỗ lệch: đây là LLM MÔ PHỎNG, luật của nó đọc thẳng trong code —
`dem_vi_du_fewshot` chỉ ĐẾM số message có `role == "assistant"`, không đọc
nội dung của chúng. `llm_dem_tong` chỉ so `so_vi_du >= 2` — một phép đếm
đơn thuần, hoàn toàn xác định trước khi chạy.
::
:::
::::

::::code{#viet_llm_dem_tong}
Hoàn thiện `llm_dem_tong`: ngưỡng chuyển từ luật đi tắt sang cộng toàn bộ
là ĐÚNG `2` ví dụ mẫu trở lên; nhánh đi tắt chỉ giữ lại HAI số đầu tiên.

```python title=starter
import re


def trich_so(text):
    return [int(x) for x in re.findall(r"\d+", text)]


def dem_vi_du_fewshot(messages):
    return sum(1 for tin in messages if tin["role"] == "assistant")


def lay_cau_hoi_cuoi(messages):
    cau_hoi = None
    for tin in messages:
        if tin["role"] == "user":
            cau_hoi = tin["content"]
    return cau_hoi


def llm_dem_tong(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    so_list = trich_so(cau_hoi)
    so_vi_du = dem_vi_du_fewshot(messages)
    if so_vi_du ___ 2:                      # >=
        tong = sum(so_list)
    else:
        tong = sum(so_list[___])            # :2
    return str(tong)


cau_hoi_test = "Lan co 3 qua tao, 5 qua cam va 2 qua le. Hoi Lan co tat ca bao nhieu qua?"

messages_zero_shot = [
    {"role": "user", "content": cau_hoi_test},
]

messages_few_shot = [
    {"role": "user", "content": "Nam co 1 qua tao va 2 qua cam. Hoi Nam co tat ca bao nhieu qua?"},
    {"role": "assistant", "content": "3"},
    {"role": "user", "content": "Hoa co 4 qua tao, 1 qua cam va 3 qua le. Hoi Hoa co tat ca bao nhieu qua?"},
    {"role": "assistant", "content": "8"},
    {"role": "user", "content": cau_hoi_test},
]

messages_mot_vi_du = [
    {"role": "user", "content": "Nam co 1 qua tao va 2 qua cam. Hoi Nam co tat ca bao nhieu qua?"},
    {"role": "assistant", "content": "3"},
    {"role": "user", "content": cau_hoi_test},
]

dap_zero_shot = llm_dem_tong(messages_zero_shot)
dap_few_shot = llm_dem_tong(messages_few_shot)
dap_mot_vi_du = llm_dem_tong(messages_mot_vi_du)
tong_dung = sum(trich_so(cau_hoi_test))

print(dap_zero_shot)
print(dap_few_shot)
print(dap_mot_vi_du)
print(tong_dung)
```

```python title=solution
import re


def trich_so(text):
    return [int(x) for x in re.findall(r"\d+", text)]


def dem_vi_du_fewshot(messages):
    return sum(1 for tin in messages if tin["role"] == "assistant")


def lay_cau_hoi_cuoi(messages):
    cau_hoi = None
    for tin in messages:
        if tin["role"] == "user":
            cau_hoi = tin["content"]
    return cau_hoi


def llm_dem_tong(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    so_list = trich_so(cau_hoi)
    so_vi_du = dem_vi_du_fewshot(messages)
    if so_vi_du >= 2:
        tong = sum(so_list)
    else:
        tong = sum(so_list[:2])
    return str(tong)


cau_hoi_test = "Lan co 3 qua tao, 5 qua cam va 2 qua le. Hoi Lan co tat ca bao nhieu qua?"

messages_zero_shot = [
    {"role": "user", "content": cau_hoi_test},
]

messages_few_shot = [
    {"role": "user", "content": "Nam co 1 qua tao va 2 qua cam. Hoi Nam co tat ca bao nhieu qua?"},
    {"role": "assistant", "content": "3"},
    {"role": "user", "content": "Hoa co 4 qua tao, 1 qua cam va 3 qua le. Hoi Hoa co tat ca bao nhieu qua?"},
    {"role": "assistant", "content": "8"},
    {"role": "user", "content": cau_hoi_test},
]

messages_mot_vi_du = [
    {"role": "user", "content": "Nam co 1 qua tao va 2 qua cam. Hoi Nam co tat ca bao nhieu qua?"},
    {"role": "assistant", "content": "3"},
    {"role": "user", "content": cau_hoi_test},
]

dap_zero_shot = llm_dem_tong(messages_zero_shot)
dap_few_shot = llm_dem_tong(messages_few_shot)
dap_mot_vi_du = llm_dem_tong(messages_mot_vi_du)
tong_dung = sum(trich_so(cau_hoi_test))

print(dap_zero_shot)
print(dap_few_shot)
print(dap_mot_vi_du)
print(tong_dung)
```

```python title=test
assert tong_dung == 10, f"tong dung phai la 10 -- dang ra {tong_dung}"
assert dap_zero_shot == "8", f"zero-shot phai la '8' (luat di tat, thieu so thu 3) -- dang ra {dap_zero_shot!r}"
assert dap_few_shot == "10", f"few-shot (2 vi du) phai la '10' (cong toan bo) -- dang ra {dap_few_shot!r}"
assert dap_mot_vi_du == "8", f"chi 1 vi du VAN phai la '8' (chua dat nguong >=2) -- dang ra {dap_mot_vi_du!r}"

# bien: cau hoi chi co DUNG HAI dai luong -- luat di tat va luat cong toan
# bo phai cho CUNG mot ket qua, vi khong co so thu ba nao bi bo sot
cau_hai_so = "Mai co 2 qua tao va 3 qua cam. Hoi Mai co tat ca bao nhieu qua?"
messages_hai_so = [{"role": "user", "content": cau_hai_so}]
assert llm_dem_tong(messages_hai_so) == "5", f"cau hoi 2 dai luong phai dung du zero-shot -- dang ra {llm_dem_tong(messages_hai_so)!r}"

# bien: dung 2 vi du mau nhung cau hoi chi co 1 dai luong -- van phai cong
# dung (nhanh full-sum khong lam sai cau don gian)
messages_few_shot_don = [
    {"role": "user", "content": "Nam co 1 qua tao va 2 qua cam. Hoi Nam co tat ca bao nhieu qua?"},
    {"role": "assistant", "content": "3"},
    {"role": "user", "content": "Hoa co 4 qua tao, 1 qua cam va 3 qua le. Hoi Hoa co tat ca bao nhieu qua?"},
    {"role": "assistant", "content": "8"},
    {"role": "user", "content": "An co 7 qua xoai. Hoi An co tat ca bao nhieu qua?"},
]
assert llm_dem_tong(messages_few_shot_don) == "7", f"few-shot tren cau 1 dai luong phai la '7' -- dang ra {llm_dem_tong(messages_few_shot_don)!r}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu là toán tử so sánh trong `if so_vi_du ___ 2:` — ngưỡng đúng là "lớn hơn hoặc bằng `2`". Chỗ hai là chỉ số cắt trong `so_list[___]` ở nhánh đi tắt — chỉ giữ lại HAI phần tử đầu tiên của danh sách.
- kind: strategy
  body: 'Chỗ đầu: `>=` (cho dòng `if so_vi_du >= 2:`). Chỗ hai: `:2` (cho dòng `sum(so_list[:2])` — lát cắt từ đầu tới trước chỉ số `2`).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `>=` và `:2`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: dieu kien nguong phai dung toan tu ">=" so voi 2 (khong duoc doi thanh ">" hay mot hang so khac), VA nhanh di tat phai cat DUNG hai phan tu dau (khong duoc doi thanh mot lat cat khac)
  requireAst:
  - kind: uses-operator, target: ">=", min: 1
  - kind: has-literal, target: "2", min: 2
  # Da thu that (goi _dem tren code trich tu solution). ">="=1: DUY NHAT
  # mot lan, chinh la cho trong 1 -- dien bua thanh ">" hay "==" lam so nay
  # tut ve 0, duoi nguong min=1, bi chan; dong thoi bi chan boi tests vi
  # messages_mot_vi_du (so_vi_du=1) va messages_few_shot (so_vi_du=2) se
  # cho ket qua sai neu doi toan tu.
  # has-literal "2"=2: mot lan trong "so_vi_du >= 2" (cho trong 1), mot lan
  # trong "so_list[:2]" (cho trong 2) -- day la TONG THAT tren toan bo
  # solution (khong co "2" nao khac lam Constant so, vi cac chu so "2" xuat
  # hien trong noi dung cau hoi vi du deu nam BEN TRONG mot chuoi văn ban
  # duy nhat, khong phai Constant so rieng le). Dien bua cho trong 2 thanh
  # "so_list[:3]" lam has-literal "2" tut ve 1 -- duoi nguong min=2, bi
  # chan; dong thoi bi chan boi tests vi dap_zero_shot se doi tu '8' sang
  # '10' (cong ca ba so, trung voi few-shot, mat kha nang phan biet).
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^8\\n10\\n8\\n10\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`8` (zero-shot, sai), `8` (một ví dụ, chưa đủ), `10` (hai ví dụ, đúng) —
đúng ngưỡng đo được. Nhưng few-shot chỉ đổi được ĐỘ CHÍNH XÁC của con số;
nó không đổi được ĐỊNH DẠNG câu trả lời. Bài sau thêm trục thứ hai: ràng
buộc rõ ràng để thu hẹp cách trả lời về ĐÚNG một định dạng.
::::

::::reflect{#nghi-lai}
Ba con số — `8`, `8`, `10` — tới từ đúng MỘT ngưỡng trong code:
`so_vi_du >= 2`. Không có gì "thông minh dần" khi thêm ví dụ mẫu thứ nhất;
hiệu ứng chỉ xuất hiện khi NGƯỠNG bị vượt qua. Với một LLM thật, ngưỡng đó
không tường minh như thế này — nhưng nguyên lý few-shot vẫn là: các ví dụ
mẫu đặt TRƯỚC câu hỏi có thể đổi ĐỊNH DẠNG/CÁCH suy luận của câu trả lời,
không chỉ là "ngữ cảnh thêm cho vui". Bài sau chuyển sang một trục khác:
dùng CHỈ THỊ tường minh (không cần ví dụ mẫu) để thu hẹp không gian câu
trả lời.
::::

::::checkpoint{mastery=0.85}
::::
